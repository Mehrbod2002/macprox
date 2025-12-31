# Security Policy

## Supported Versions

MacProx is currently under active development.

Only the **latest version on the `main` branch** and the **latest tagged release** are supported with security updates.

Older versions may not receive fixes.

---

## Reporting a Vulnerability

If you discover a security vulnerability in MacProx, **please do not open a public GitHub issue**.

Instead, report it by opening a **private security advisory** via GitHub (recommended).

- Emailing: **security@m9.akhlaghpoor@gmail.com**
- Or opening a **private security advisory** via GitHub (recommended)

When reporting, please include:

- A clear description of the issue
- Steps to reproduce (if applicable)
- Affected version or commit
- Any relevant logs or screenshots (redact secrets)

We will acknowledge receipt of your report as soon as possible and work with you to resolve the issue.

---

## Security Considerations

- MacProx uses `sshuttle` to create a system-wide tunnel.
- SSH key authentication is strongly recommended.
- Password authentication is supported via `SSH_ASKPASS` for compatibility, but should be avoided on shared or untrusted machines.
- MacProx does **not** store SSH credentials on disk.

---

## Responsible Disclosure

Please allow a reasonable amount of time for a fix before publicly disclosing any security issues.

We appreciate responsible disclosure and contributions that help keep MacProx secure.
