# BUILD_PLAN_029: Multi-Platform Binary Distribution via GitHub Actions

**Status:** NOT STARTED
**Priority:** Critical (blocks adoption on non-ARM-Mac platforms)
**Depends on:** BUILD_PLAN_016 (git automation)

## Context

CruxDev only builds for ARM macOS (aarch64-apple-darwin). Any project on Intel Mac, Linux, or CI/CD cannot use it. OpenClaw on Intel is blocked. GitHub Actions CI with a build matrix compiling for all targets and attaching binaries to GitHub Releases is the standard approach.

## Target Matrix

| Platform | Rust Target | Runner | Priority |
|----------|-------------|--------|----------|
| macOS ARM (Apple Silicon) | `aarch64-apple-darwin` | `macos-latest` | Current |
| macOS Intel | `x86_64-apple-darwin` | `macos-13` | Critical |
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `ubuntu-latest` | Critical |
| Linux ARM | `aarch64-unknown-linux-gnu` | `ubuntu-latest` + cross | Future |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `windows-latest` | Future |

## Phase 1: GitHub Actions CI Workflow

- [ ] 1.1 Create `.github/workflows/release.yml`
- [ ] 1.2 Trigger: on push of version tag (`v*`)
- [ ] 1.3 Build matrix: macOS ARM, macOS Intel, Linux x86_64
- [ ] 1.4 Each job: checkout, install Rust, cargo build --release, cargo test
- [ ] 1.5 Upload binary as artifact per job
- [ ] 1.6 Create GitHub Release with all binaries attached
- [ ] 1.7 Binary naming: `cruxdev-{version}-{target}` (e.g. `cruxdev-v0.2.0-x86_64-apple-darwin`)

## Phase 2: CI Test Workflow

- [ ] 2.1 Create `.github/workflows/ci.yml`
- [ ] 2.2 Trigger: on push to master, on PR
- [ ] 2.3 Jobs: cargo test, cargo clippy -- -D warnings, cargo fmt -- --check
- [ ] 2.4 Fail PR if any check fails

## Phase 3: Install Script

- [ ] 3.1 Create `install.sh` — detects OS/arch, downloads correct binary from GitHub Releases
- [ ] 3.2 Verify checksum (SHA256)
- [ ] 3.3 Place binary in user's PATH
- [ ] 3.4 One-liner: `curl -fsSL https://cruxdev.dev/install.sh | sh`
- [ ] 3.5 Add install script to website

## Phase 4: Cross-Compilation Setup

- [ ] 4.1 For Linux ARM: use `cross` tool or Docker-based cross-compilation
- [ ] 4.2 Verify SQLite bundled compilation works on all targets (rusqlite bundled feature)
- [ ] 4.3 Verify reqwest rustls-tls works on all targets (no OpenSSL dependency)

## Phase 5: Update Docs

- [ ] 5.1 Update install page with platform-specific tabs
- [ ] 5.2 Update README with install one-liner
- [ ] 5.3 Update llms.txt with platform support

## Verification

```bash
# Verify binary runs on each platform
./cruxdev-v0.2.0-x86_64-apple-darwin status
./cruxdev-v0.2.0-x86_64-unknown-linux-gnu status
./cruxdev-v0.2.0-aarch64-apple-darwin status
```
