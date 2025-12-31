# MacProx

![CI](https://github.com/mehrbod2002/macprox/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/github/license/mehrbod2002/macprox)
![Rust](https://img.shields.io/badge/rust-2024-edition?logo=rust)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![Security](https://img.shields.io/badge/security-policy-blue)

A small macOS GUI application for easily managing a **system-wide tunnel** using [`sshuttle`](https://github.com/sshuttle/sshuttle).

Creates a transparent VPN-like tunnel through an SSH server — no browser extension or SOCKS proxy configuration needed.

## Features

- Simple native macOS interface
- Start/stop system-wide sshuttle tunnel with one click
- Shows current connection status
- (Future) Lists installed applications (potential for per-app routing)

> **Note**: This is **not** a SOCKS proxy toggler.
> It routes **all system traffic** through the SSH tunnel (like a lightweight VPN).

## Project Status

MacProx is under active development.  
The core tunnel functionality is stable; UI and per-app features are evolving.

## Requirements

- macOS (tested on 12+)
- Rust toolchain (`cargo`, `rustc`)
- One of:
  - Homebrew (recommended)
  - Python 3 + pip3

## Installation / Setup (recommended)

```bash
# Install sshuttle (the only real dependency)
make deps
```

````

This command will:

- Use `brew install sshuttle` if Homebrew is available
- Fall back to `pip3 install sshuttle` otherwise

## Development

```bash
# Run in development mode
cargo run

# Build release version
cargo build --release
```

The release binary will appear in `target/release/macprox`

## Security Recommendations

- **Strongly prefer SSH key authentication**
- Password authentication works (via `SSH_ASKPASS`), but **avoid** it on shared or untrusted machines
- Never run MacProx with `sudo` — it asks for admin rights only when starting/stopping the tunnel

## Troubleshooting

- `sshuttle` may require administrator privileges (you'll see a macOS prompt)
- If connection fails:
  - First verify you can connect manually:
    ```bash
    ssh user@your.server.com
    ```
  - Check that the server allows TCP forwarding (`AllowTcpForwarding yes` in `sshd_config`)
  - Make sure no conflicting VPN / other tunnel is active
  - Try with `-v` (verbose mode) to see detailed errors:
    ```bash
    sshuttle --dns -v -r user@server.com 0/0
    ```

## License

[MIT](LICENSE)

```

This version is:

- clean & modern
- properly formatted for GitHub rendering
- includes realistic sections most users expect
- contains practical security & troubleshooting info
- ready to be used as your project's `README.md`
```
````
