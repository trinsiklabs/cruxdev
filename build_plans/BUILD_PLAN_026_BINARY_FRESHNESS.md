# BUILD_PLAN_026: Build Artifact Freshness Gate in Convergence Pipeline

**Status:** NOT STARTED
**Priority:** Critical
**Depends on:** BUILD_PLAN_016 (git automation)

## Context

When a build plan converges and modifies source files, compiled/built artifacts remain stale until manually rebuilt. Other sessions using those artifacts continue running old versions with missing features, outdated logic, and stale behavior. This is a silent regression — nothing fails, the artifacts just don't reflect current code.

This applies to ANY build artifact: Rust binaries, JavaScript bundles, static site output, Go binaries, Python wheels, Docker images, compiled CSS, WASM modules, etc.

The convergence pipeline must enforce: if source files changed and the project has a build step, artifacts must be rebuilt before the plan is considered converged.

## Phase 1: Binary Freshness Detection

- [ ] 1.1 New module: `rust/src/engine/binary_freshness.rs`
- [ ] 1.2 `check_binary_freshness(project_dir, binary_path, source_dirs) -> FreshnessResult`
  - Compare binary mtime against newest source file mtime
  - If any source file is newer than binary → stale
  - Return: stale (bool), binary_age, newest_source, files_changed_since_build
- [ ] 1.3 `detect_build_targets(project_dir) -> Vec<BuildTarget>`
  - Scan for Cargo.toml → `target/release/{name}` (Rust binary)
  - Scan for package.json with `build` script → `dist/` or `build/` (JS/TS bundle, static site)
  - Scan for go.mod → binary output (Go)
  - Scan for Makefile/CMakeLists.txt → compiled output (C/C++)
  - Scan for Dockerfile → image tag (Docker)
  - Scan for pyproject.toml with `[build-system]` → `dist/*.whl` (Python wheel)
  - Scan for astro.config.* → `dist/` (Astro static site)
  - Each target: artifact_path, build_command, source_dirs, artifact_type
- [ ] 1.4 Tests: stale detection, fresh detection, missing binary

## Phase 2: Build Gate in Convergence Pipeline

- [ ] 2.1 After CONVERGED state, before commit: check binary freshness
- [ ] 2.2 If stale → run build command → verify success → then proceed to commit
- [ ] 2.3 If build fails → escalate (do not commit stale binary)
- [ ] 2.4 Build command from project config (`.cruxdev/growth.toml` or new `build` section)
- [ ] 2.5 For Rust: `cargo build --release` (auto-detected from Cargo.toml)

## Phase 3: Configuration

- [ ] 3.1 Add `[build]` section to `.cruxdev/growth.toml`:
```toml
[[build.targets]]
artifact = "rust/target/release/cruxdev"
command = "cargo build --release"
working_dir = "rust/"
source_dirs = ["rust/src/"]
type = "binary"

[[build.targets]]
artifact = "dist/"
command = "npm run build"
working_dir = ""
source_dirs = ["src/", "public/"]
type = "static_site"

[build]
auto_rebuild = true
fail_on_stale = true
```
- [ ] 3.2 If no config, auto-detect from Cargo.toml/package.json/go.mod/Makefile/Dockerfile
- [ ] 3.3 `auto_rebuild = true` means convergence rebuilds automatically; `false` means it flags but doesn't build
- [ ] 3.4 `fail_on_stale = true` means convergence cannot complete with stale artifacts

## Phase 4: MCP Tools

- [ ] 4.1 `check_build_freshness(project_dir)` — MCP tool to check ALL build artifacts
- [ ] 4.2 Returns: per-target status (artifact path, stale/fresh, age, build command to fix)
- [ ] 4.3 Wire into `cruxdev_status` — include build freshness in health check
- [ ] 4.4 `rebuild_stale(project_dir, dry_run)` — rebuild all stale artifacts

## Phase 5: Integration Points

- [ ] 5.1 Post-convergence: rebuild before commit
- [ ] 5.2 Post-push: verify binary matches pushed code
- [ ] 5.3 `run_growth_cycle`: check freshness at start, refuse to run with stale binary
- [ ] 5.4 `cruxdev_status`: report binary freshness as health indicator

## Phase 6: Tests

- [ ] 6.1 Unit: freshness detection with tempdir + fake binary
- [ ] 6.2 Unit: auto-detect Cargo.toml → binary target
- [ ] 6.3 E2E: convergence pipeline rebuilds stale binary

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
