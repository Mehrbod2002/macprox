slint::include_modules!();

mod apps;
use apps::list_macos_apps;

use slint::{ModelRc, VecModel};

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::process::{Child, Command};

#[derive(Clone)]
struct TunnelState {
    child: Arc<Mutex<Option<Child>>>,
    askpass_path: Arc<Mutex<Option<PathBuf>>>,
}

impl TunnelState {
    fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            askpass_path: Arc::new(Mutex::new(None)),
        }
    }

    async fn disconnect(&self) {
        let child_opt: Option<Child> = { self.child.lock().unwrap().take() };

        if let Some(mut child) = child_opt {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }

        let askpass_opt: Option<PathBuf> = { self.askpass_path.lock().unwrap().take() };

        if let Some(p) = askpass_opt {
            let _ = fs::remove_file(p);
        }
    }
}

fn build_askpass_script() -> Option<PathBuf> {
    // Script prints password stored in env var MACPROX_SSH_PASSWORD
    let script = r#"#!/bin/sh
printf "%s" "$MACPROX_SSH_PASSWORD"
"#;

    let mut path = std::env::temp_dir();
    // Unique-ish name to avoid collisions
    path.push(format!("macprox_askpass_{}.sh", std::process::id()));

    fs::write(&path, script).ok()?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).ok()?;
    Some(path)
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = AppListWindow::new()?;

    // Load apps
    let rows: Vec<AppRow> = list_macos_apps()
        .into_iter()
        .map(|a| AppRow {
            name: a.name.into(),
            path: a.path.into(),
        })
        .collect();
    let model: ModelRc<AppRow> = std::rc::Rc::new(VecModel::from(rows)).into();
    ui.set_apps(model);

    let state = TunnelState::new();

    // Connect
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();

        ui.on_connect_proxy(move |remark, server, port_str, user, password| {
            let ui = match ui_weak.upgrade() {
                Some(u) => u,
                None => return,
            };

            let remark = remark.to_string().trim().to_string();
            let server = server.to_string().trim().to_string();
            let port_str = port_str.to_string().trim().to_string();
            let user = user.to_string().trim().to_string();
            let password = password.to_string();

            if server.is_empty() || user.is_empty() {
                ui.set_proxy_status("Server and Username required".into());
                return;
            }

            let port: u16 = port_str.parse().unwrap_or(22);

            let remote = format!("{}@{}", user, server);

            let display_name = if !remark.is_empty() {
                remark
            } else {
                format!("{}@{}:{}", user, server, port)
            };

            let ui_weak2 = ui.as_weak();
            let state2 = state.clone();

            slint::spawn_local(async move {
                let ui = match ui_weak2.upgrade() {
                    Some(u) => u,
                    None => return,
                };

                // Prevent double-connect
                if ui.get_proxy_connected() {
                    ui.set_proxy_status("Already connected. Disconnect first.".into());
                    return;
                }

                ui.set_proxy_status(format!("Connecting to {}...", display_name).into());

                // clean old remnants just in case
                state2.disconnect().await;

                let mut cmd = Command::new("sshuttle");
                cmd.arg("--dns")
                    .arg("-r")
                    .arg(&remote)
                    .arg("0.0.0.0/0")
                    .arg("--method")
                    .arg("auto")
                    .arg("-x")
                    .arg(&server);

                // No terminal IO (GUI-safe)
                cmd.stdin(Stdio::null());
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::piped());

                // Build ssh command that sshuttle will use
                // -p PORT is the correct way to set port (fixes your current bug)
                let mut ssh_e = format!(
                    "ssh -p {} \
                     -o ExitOnForwardFailure=yes \
                     -o ServerAliveInterval=15 \
                     -o ServerAliveCountMax=3",
                    port
                );

                if !password.is_empty() {
                    // Force password auth & disable key auth if password provided
                    ssh_e.push_str(" -o PreferredAuthentications=password -o PubkeyAuthentication=no -o NumberOfPasswordPrompts=1");

                    // Create askpass helper
                    if let Some(askpass_path) = build_askpass_script() {
                        *state2.askpass_path.lock().unwrap() = Some(askpass_path.clone());

                        // Supply password through env var, and force ssh to use askpass
                        cmd.env("MACPROX_SSH_PASSWORD", password);
                        cmd.env("SSH_ASKPASS", askpass_path);
                        cmd.env("SSH_ASKPASS_REQUIRE", "force");

                        // ssh checks DISPLAY; dummy value is fine
                        cmd.env("DISPLAY", ":0");
                    } else {
                        ui.set_proxy_status("Failed to create askpass helper".into());
                        return;
                    }
                }

                cmd.arg("-e").arg(ssh_e);

                let mut child = match cmd.spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        ui.set_proxy_status(format!("Failed to start sshuttle: {}", e).into());
                        state2.disconnect().await;
                        return;
                    }
                };

                // Give it a moment to fail fast if auth/command is wrong
                tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

                if let Ok(Some(status)) = child.try_wait() {
                    // It exited quickly => failure. Capture stderr if possible
                    let  err_msg = format!("Connection failed: {}", status);

                    if let Ok(Some(status)) = child.try_wait() {
                        let err_msg = format!("Connection failed: {}", status);
                        ui.set_proxy_status(err_msg.into());
                        state2.disconnect().await;
                        return;
                    }


                    ui.set_proxy_status(err_msg.into());
                    state2.disconnect().await;
                    return;
                }

                *state2.child.lock().unwrap() = Some(child);

                ui.set_proxy_connected(true);
                ui.set_proxy_status(format!("Connected via sshuttle: {}", display_name).into());
            })
            .unwrap();
        });
    }

    // Disconnect
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();

        ui.on_disconnect_proxy(move || {
            let ui = match ui_weak.upgrade() {
                Some(u) => u,
                None => return,
            };

            let ui_weak2 = ui.as_weak();
            let state2 = state.clone();

            slint::spawn_local(async move {
                let ui = match ui_weak2.upgrade() {
                    Some(u) => u,
                    None => return,
                };

                state2.disconnect().await;
                ui.set_proxy_connected(false);
                ui.set_proxy_status("Disconnected".into());
            })
            .unwrap();
        });
    }

    // Optional: row click handler (no-op unless you want behavior)
    ui.on_row_clicked(|_index| {});

    {
        let state = state.clone();
        ui.on_window_close_requested(move || {
            let state = state.clone();
            slint::spawn_local(async move {
                state.disconnect().await;
            })
            .unwrap();
        });
    }

    ui.run()
}
