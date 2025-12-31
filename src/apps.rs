use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: String,
    pub path: String,
}

fn scan_dir_for_apps(root: &Path, out: &mut Vec<AppInfo>) {
    if !root.exists() {
        return;
    }

    for entry in WalkDir::new(root)
        .max_depth(4)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) == Some("app") {
            let name = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();

            out.push(AppInfo {
                name,
                path: p.to_string_lossy().to_string(),
            });
        }
    }
}

pub fn list_macos_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();

    let roots: Vec<PathBuf> = vec![
        "/Applications".into(),
        "/System/Applications".into(),
        "/System/Applications/Utilities".into(),
    ];

    for r in roots {
        scan_dir_for_apps(&r, &mut apps);
    }

    if let Some(home) = dirs::home_dir() {
        scan_dir_for_apps(&home.join("Applications"), &mut apps);
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps.dedup_by(|a, b| a.path == b.path);

    apps
}
