# BUILD_PLAN_102: Mandatory GTV Before Any Public Content

**Status:** NOT STARTED
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
- [ ] 1.1 Scan all pages for numeric claims (test count, tool count, coverage %)
- [ ] 1.2 Verify each claim against the actual source (`cargo test | grep "test result"`, count tools in server.rs)
- [ ] 1.3 Scan for feature claims ("supports X", "routes to Y", "integrates with Z")
- [ ] 1.4 For each feature claim: grep the codebase for the implementation. If not found → flag
- [ ] 1.5 Scan for status claims ("active", "live", "working") — verify each
- [ ] 1.6 Block deploy if any claim fails GTV

## Phase 2: Pre-Commit GTV for Docs

Before ANY commit to docs/:
- [ ] 2.1 If the doc describes code behavior → verify the code actually does that
- [ ] 2.2 If the doc references a file path → verify the file exists
- [ ] 2.3 If the doc references a function → verify the function exists and has the claimed signature
- [ ] 2.4 If the doc references a tool → verify the tool is registered in server.rs
- [ ] 2.5 If the doc makes a performance claim → verify with a benchmark or test
- [ ] 2.6 If the doc references a competitor → verify the claim is current (check date)

## Phase 3: Automated GTV Scanner

- [ ] 3.1 New MCP tool: `gtv_scan_content(file_path)` → extracts claims, verifies each
- [ ] 3.2 Claim extraction: regex for numbers ("N tests", "N tools"), feature verbs ("supports", "routes", "integrates"), status words ("active", "live")
- [ ] 3.3 Verification per claim type:
  - Numeric → run command, compare
  - Feature → grep codebase for implementation
  - Status → check if service/endpoint responds
  - Path → check file exists
  - URL → fetch and check status
- [ ] 3.4 Wire into deploy.sh: `gtv_scan_content` runs on all changed pages before rsync
- [ ] 3.5 Wire into git pre-commit hook for docs/: verify before allowing commit

## Phase 4: GTV for Pattern Docs

Pattern docs are the most dangerous — they're authoritative "best practices" that other projects use. If they contain wrong information:
- [ ] 4.1 Every code example in a pattern doc must compile/run
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
