# BUILD_PLAN_110: Metrics-Driven Content — Zero Hardcoded Numbers

**Status:** NOT STARTED
**Priority:** Critical (prevents the entire class of stale number bugs)
**Triggered by:** External review found tool/test/dimension counts inconsistent across 30+ pages. Root cause: numbers hardcoded in 6+ independent locations with no single source of truth.

## The Problem

Every time a test is added, a tool is registered, or a dimension is created, someone has to manually update the README, llms.txt, homepage, engine page, footer, vs/ pages, blog posts, docs, and integrations page. They never do. The numbers drift within hours.

This is not a discipline problem. This is an architecture problem. Hardcoded numbers in multiple locations will always drift. The fix is to eliminate the hardcoding.

## The Fix

One command computes the truth. The build injects it. No hardcoded numbers anywhere.

```
cargo run -- metrics > metrics.json    (compute from code)
astro build                             (reads metrics.json, injects into templates)
deploy.sh                              (GTV verifies, deploys)
```

## Phase 1: Metrics Extractor (Rust)

**File:** `rust/src/metrics/mod.rs`

### 1a. Compute metrics from code
- [ ] `count_tests()` → run `cargo test --list`, count lines ending in `: test`
- [ ] `count_tools()` → spawn MCP server, call `tools/list`, count results
- [ ] `count_dimensions()` → parse router.rs constants, count total entries
- [ ] `count_dimension_sets()` → count `*_DIMENSIONS` constants
- [ ] `count_patterns()` → glob `docs/*PATTERNS*.md`, count files
- [ ] `count_pages()` → glob website `src/pages/**/*.{astro,md}`, count files
- [ ] `count_blog_posts()` → glob `src/pages/blog/*.md`, count files
- [ ] `count_vs_pages()` → glob `src/pages/vs/*.astro`, count non-index
- [ ] `binary_size()` → stat the release binary

### 1b. Output metrics.json
- [ ] Write to `.cruxdev/metrics.json` (atomic write)
- [ ] Format: `{ "tests": 520, "tools": 63, "dimensions": 146, "dimension_sets": 21, ... }`
- [ ] Include `computed_at` timestamp

### 1c. CLI command
- [ ] `cruxdev metrics` → compute and print metrics.json
- [ ] `cruxdev metrics --output path` → write to specific file

### 1d. Tests
- [ ] test_count_tests_returns_nonzero
- [ ] test_count_dimensions_matches_router
- [ ] test_metrics_json_valid_schema
- [ ] test_metrics_output_atomic_write

## Phase 2: Astro Build Integration

**File:** `astro.config.mjs`, new `src/data/metrics.ts`

### 2a. Metrics loader
- [ ] `src/data/metrics.ts` — reads metrics.json at build time
- [ ] Exports typed constants: `TESTS`, `TOOLS`, `DIMENSIONS`, `DIMENSION_SETS`, `PATTERNS`, `PAGES`, `BLOG_POSTS`
- [ ] Fallback: if metrics.json missing, use last-known values with warning

### 2b. Replace hardcoded numbers
- [ ] `src/layouts/Base.astro` footer: `{TESTS} tests · {TOOLS} MCP tools`
- [ ] `src/pages/index.astro` meta: `{DIMENSIONS} audit dimensions across {DIMENSION_SETS} dimension sets`
- [ ] `src/pages/engine.astro` heading + meta: `{TOOLS} MCP Tools`
- [ ] `src/pages/for/software-engineers.astro`: `{DIMENSIONS}-dimension audit`
- [ ] `src/pages/vs/*.astro`: all dimension/tool references
- [ ] `src/pages/lp/*.astro`: all dimension references
- [ ] `src/pages/docs/*.astro`: tool count references
- [ ] `src/pages/integrations/index.astro`: tool count

### 2c. llms.txt generation
- [ ] Generate llms.txt from template + metrics at build time (not static file)
- [ ] Move template to `src/data/llms.txt.template`
- [ ] Build script replaces `{{TESTS}}`, `{{TOOLS}}`, `{{DIMENSIONS}}` etc.

### 2d. README generation
- [ ] `scripts/update-readme.sh` — reads metrics.json, updates README.md numbers section
- [ ] Or: README uses a template with markers that get replaced

## Phase 3: Build Pipeline Integration

### 3a. Pre-build step
- [ ] `deploy.sh`: run `cruxdev metrics` before `npm run build`
- [ ] metrics.json is fresh for every deploy

### 3b. GTV integration
- [ ] After build: run `gtv_scan_all` on dist/
- [ ] Compare extracted claims against metrics.json
- [ ] Block deploy if any mismatch

### 3c. CI integration
- [ ] GitHub Action: on push to website repo, compute metrics, build, GTV scan
- [ ] PR check: fail if any hardcoded number detected in .astro files

## Phase 4: Blog Post Strategy

### 4a. Snapshot posts (historical)
- [ ] Add banner to old posts: "Numbers in this post reflect the state at time of writing. Current: [link to live metrics]"
- [ ] Do NOT auto-update old blog posts — they're historical records

### 4b. Feature/announcement posts
- [ ] Use relative language: "over 500 tests" instead of "520 tests"
- [ ] Or reference the metrics endpoint: "see current numbers at /metrics"

### 4c. Marketing pages
- [ ] Template variables only. Zero hardcoded numbers.
- [ ] If a number appears on a marketing page, it MUST come from metrics.json

## Phase 5: CruxBot Enforcement

### 5a. Weekly metric recomputation
- [ ] CruxBot sentinel watches for new commits
- [ ] After any commit that changes tests/tools/dimensions: recompute metrics
- [ ] If website is stale: auto-rebuild and deploy

### 5b. PR-level check
- [ ] If a PR adds a test/tool/dimension: verify metrics.json is updated
- [ ] If a PR touches a marketing page: verify no hardcoded numbers introduced

## Verification

```bash
# Compute metrics
cd rust && cargo run -- metrics

# Verify no hardcoded numbers in templates
grep -rn '[0-9]\+ tests\|[0-9]\+ tools\|[0-9]\+ dimensions' ../cruxdev-dev/src/ | grep -v 'metrics\.' | grep -v 'node_modules'

# Build with metrics
cd ../cruxdev-dev && npm run build

# GTV verify
cd ../cruxdev/rust && cargo run -- mcp gtv_scan_all ../cruxdev-dev/dist/
```

## The Standard

If a number appears on a public page, it came from `metrics.json`. If `metrics.json` was computed by running actual code. If the code changed, the number changes automatically. No human updates. No drift. No stale claims. Ever.
