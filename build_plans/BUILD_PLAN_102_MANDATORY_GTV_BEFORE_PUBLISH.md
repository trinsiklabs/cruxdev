# BUILD_PLAN_102: Mandatory GTV Before Any Public Content

**Status:** ESCALATED
**Priority:** Critical (trust issue — we shipped misleading content to production)
**Triggered by:** Engine page claimed model tier routing was active when Claude Code ignores the tier field entirely. Misleading copy on a live website.

## The Problem

We write docs and website pages that describe what the code SHOULD do, not what it ACTUALLY does. Then we publish them. This is the worst kind of failure for a product that claims "ground truth verification."

Examples from THIS SESSION:
- Engine page: "Tasks are routed to the appropriate model tier" — false in Claude Code
- Homepage: "419 tests" when actual was 462 — stale stat shipped to production
- Blog post linked to wrong URL slug — link was 404 until caught
- robots.txt referenced sitemap.xml instead of sitemap-index.xml
- llms.txt said "52 MCP tools" when actual was 61
- Footer said "451 tests" across multiple deploys while actual was higher

## The Rule

**Nothing publishes without GTV.** Not a deploy. Not a doc update. Not a blog post. Not a pattern doc. Every claim verified against the running code before it goes live.

## Phase 1: Pre-Publish GTV Gate

Before ANY deploy to cruxdev.dev:
- [x] 1.1 Scan all pages for numeric claims (test count, tool count, coverage %)
- [x] 1.2 Verify each claim against the actual source (`cargo test | grep "test result"`, count tools in server.rs)
- [x] 1.3 Scan for feature claims ("supports X", "routes to Y", "integrates with Z")
- [x] 1.4 For each feature claim: grep the codebase for the implementation. If not found → flag
- [x] 1.5 Scan for status claims ("active", "live", "working") — verify each
- [x] 1.6 Block deploy if any claim fails GTV

## Phase 2: Pre-Commit GTV for Docs

Before ANY commit to docs/:
- [ ] 2.1 If the doc describes code behavior → verify the code actually does that
- [ ] 2.2 If the doc references a file path → verify the file exists
- [ ] 2.3 If the doc references a function → verify the function exists and has the claimed signature
- [ ] 2.4 If the doc references a tool → verify the tool is registered in server.rs
- [ ] 2.5 If the doc makes a performance claim → verify with a benchmark or test
- [ ] 2.6 If the doc references a competitor → verify the claim is current (check date)

## Phase 3: Automated GTV Scanner

**Files:**
- `rust/src/gtv/mod.rs` — module root, Claim/Verification types
- `rust/src/gtv/scanner.rs` — claim extraction from markdown/HTML
- `rust/src/gtv/verifier.rs` — verification logic per claim type
- `rust/src/server.rs` — new MCP tool: `gtv_scan_content`
- `rust/tests/mcp_e2e.rs` — E2E test for the MCP tool

### 3a. Claim extraction (`scanner.rs`)
Supports both `.md` (markdown) and `.astro` (Astro components — scan frontmatter variables and HTML body) file formats.
- [x] 3.1 Regex extraction for numeric claims: `(\d+)\s+(tests?|tools?|pages?|coverage|dimensions?)`
- [x] 3.2 Regex extraction for feature claims: `(supports?|routes?|integrates?|enables?|provides?)\s+`
- [x] 3.3 Regex extraction for status claims: `(active|live|working|running|deployed)`
- [x] 3.4 Regex extraction for file/path references: backtick-wrapped paths
- [x] 3.5 Regex extraction for URL references: `https?://`
- [x] 3.6 Return `Vec<Claim>` with type, value, line number, source text

### 3b. Verification (`verifier.rs`)
- [x] 3.7 Numeric: run command (e.g., `cargo test 2>&1 | grep "test result"`), parse count, compare
- [x] 3.8 Feature: grep codebase for implementation keyword, return found/not-found. **Known limitation v1:** grep is fuzzy — may false-positive on comments or false-negative on differently-named implementations. Acceptable for v1; flag uncertain matches for human review.
- [x] 3.9 Status: check if referenced service/endpoint responds (HTTP GET or process check)
- [x] 3.10 Path: `std::path::Path::new(path).exists()`
- [x] 3.11 URL: HTTP HEAD request, check status code
- [x] 3.12 Return `Vec<VerificationResult>` with claim, verified: bool, actual_value, message

### 3c. MCP tool
- [x] 3.13 `gtv_scan_content(file_path, project_dir?)` → returns scan results as JSON
- [x] 3.14 `gtv_scan_all(directory, project_dir?)` → scans all .md/.astro files in directory

### 3d. Integration
- [ ] 3.15 Wire into deploy.sh: run `gtv_scan_all` on changed pages before rsync, block deploy on failures
- [ ] 3.16 Wire into convergence post-actions: GTV scan required after content generation

### 3e. Tests
- [x] test_extract_numeric_claims
- [x] test_extract_feature_claims
- [x] test_extract_status_claims
- [x] test_extract_path_references
- [x] test_extract_url_references
- [x] test_verify_numeric_claim
- [x] test_verify_path_exists
- [x] test_verify_path_not_exists
- [x] test_verify_url (mock)
- [x] test_scan_content_e2e
- [x] test_extract_claims_from_astro_file
- [x] test_feature_claim_uncertain_flagged

## Phase 4: GTV for Pattern Docs

Pattern docs are the most dangerous — they're authoritative "best practices" that other projects use. If they contain wrong information:
- [ ] 4.1 Code examples: Rust examples must `cargo check`. Other languages (Elixir, JS, Python) get syntax validation only in v1. Full compile-check deferred to v2.
- [ ] 4.2 Every API reference must be verified against current API docs
- [ ] 4.3 Every version number must be checked against latest release
- [ ] 4.4 Every "recommended" library must still be maintained
- [ ] 4.5 Stale pattern docs flagged with "Last verified: [date]" header

## Phase 5: GTV for Blog Posts

Blog posts are public marketing content:
- [ ] 5.1 Every stat in a blog post verified before publish
- [ ] 5.2 Every feature claim verified before publish
- [ ] 5.3 Every link checked (internal and external)
- [ ] 5.4 Every screenshot/code example verified against current state

## The Standard

If CruxDev says it on a public page, it must be TRUE RIGHT NOW. Not "we're working on it." Not "it's in the plan." Not "the field exists but nobody reads it." TRUE. VERIFIED. NOW.

Aspirational features get labeled "Coming in CruxBot" or "Planned" — never presented as current capabilities.

## Verification

```bash
cd rust && cargo test gtv -- --nocapture
cd rust && cargo clippy -- -D warnings
cd rust && cargo test --test mcp_e2e -- gtv --nocapture
```
