# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in CruxDev, please report it responsibly:

1. **Do NOT open a public GitHub issue** for security vulnerabilities
2. Email security concerns to the maintainers (see repo contact info)
3. Include: description of the vulnerability, steps to reproduce, potential impact

## Security Design

CruxDev follows these security principles:

- **No secrets in config files** — only env var names, never values
- **Pre-commit safety gate** — rejects secrets (.env, .pem, .key, credentials.json), binaries >1MB, gitignored files
- **Never force push** — git safety gates prevent destructive operations
- **5-layer prompt injection defense** — for GitHub issue monitoring (sanitization, architectural separation, schema validation, dry-run default, audit trail)
- **Input validation** — JSON Schema constraints on all MCP tool parameters
- **Atomic writes** — write-then-rename for all state files to prevent corruption

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.2.x   | Yes       |
| < 0.2   | No        |
