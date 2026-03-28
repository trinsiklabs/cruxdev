# BUILD_PLAN_101: Page-Level Convergence — Audit Every Page, Not Just the Project

**Status:** IN PROGRESS
**Priority:** Critical (adoption fails without this)
**Triggered by:** Adopted project passed 9/9 form dimensions at project level, but individual pages had fields floating left, "(Optional)" everywhere, tiny textareas. Not caught until human explicitly challenged.

## The Problem

Current convergence works at the PROJECT level:
- "Does this project have forms?" → Yes → add FORM_DIMENSIONS
- LLM audits "the project" against form dimensions → "looks good" → PASS

What SHOULD happen:
- Inventory every page/route in the app
- For each page, determine which patterns apply
- Audit each page individually against applicable patterns
- Report findings PER PAGE, not per project

## Why This Keeps Happening

The LLM rubber-stamps project-level audits because it can't see every page at once. It reads the codebase, sees form components exist, sees they use the right patterns, and says "PASS." It never actually visits `/visit/westlake-select` and checks if THAT specific page's fields are floating left.

## Files

- `rust/src/engine/page_audit.rs` — route inventory, page classification, per-page audit task generation
- `rust/src/server.rs` — new MCP tool: `inventory_routes`

## Tests

- [ ] test_inventory_astro_pages
- [ ] test_inventory_phoenix_routes
- [ ] test_classify_page_type_form
- [ ] test_classify_page_type_dashboard
- [ ] test_per_page_audit_task_generation
- [ ] test_all_pages_must_pass_gate

## Verification

```bash
cd rust && cargo test page_audit -- --nocapture
cd rust && cargo clippy -- -D warnings
```

## Phase 1: Page/Route Inventory

- [ ] 1.1 New MCP tool: `inventory_routes(project_dir)` → returns all pages/routes
  - Static sites: scan `src/pages/` directory, list every .astro/.html/.md file
  - Phoenix/LiveView: parse `router.ex` for all routes
  - Next.js: scan `app/` directory for page.tsx files
  - Rails: parse `routes.rb`
  - Django: parse `urls.py`
  - Generic: check for sitemap, crawl if available
- [ ] 1.2 Classify each route by type: form, dashboard, list, detail, auth, static, API
- [ ] 1.3 Map route types to applicable pattern docs:
  - Form pages → FORM_PATTERNS (17 dimensions)
  - All pages → COLOR_CONTRAST_PATTERNS (automated scanner)
  - All pages → MOBILE_WEB_PATTERNS (touch targets, responsive)
  - Dashboard pages → DASHBOARD_PATTERNS
  - All pages → ACCESSIBILITY (WCAG AA)

## Phase 2: Per-Page Audit Task Generation

- [ ] 2.1 During WebsiteConvergence phase, generate ONE audit task PER page
- [ ] 2.2 Each task includes:
  - The specific URL/route to audit
  - The applicable pattern dimensions for that page type
  - The source code for that specific page (not the whole project)
  - Instruction: "Audit THIS PAGE against THESE dimensions. Read the actual code for this page."
- [ ] 2.3 LLM must read the SPECIFIC file for each page, not just the project structure
- [ ] 2.4 For live sites: LLM fetches the URL and checks rendered output too (not just source)

## Phase 3: Automated Per-Page Checks (No LLM Needed)

- [ ] 3.1 Contrast scanner already runs per-file — wire into per-page reporting
- [ ] 3.2 HTML validation per page (are forms using correct input types, autocomplete, labels?)
- [ ] 3.3 Accessibility lint per page (missing alt text, missing labels, missing aria)
- [ ] 3.4 Link validation per page (internal links resolve)
- [ ] 3.5 Mobile meta tag check per page (viewport, touch targets via class analysis)

## Phase 4: Per-Page Findings Report

- [ ] 4.1 Findings tagged with specific page/route, not just "the project"
- [ ] 4.2 Convergence report shows per-page breakdown:
  ```
  /visit/westlake-select: 4 findings (label_positioning, required_indicators, textarea_usage, input_sizing)
  /chapters/westlake-select: 2 findings (color_contrast, text_sizing)
  /dashboard: 0 findings (clean)
  ```
- [ ] 4.3 Convergence cannot complete until ALL pages pass, not just "the project"

## Phase 5: Adoption Integration

- [x] 5.1 Update ADOPTION_PROCESS.md: Phase 8 (Convergence Verification) must include per-page audit — DONE: Added Step 5.5 (Page-Level Audit) with full per-page form/contrast/mobile/link/SEO/security audit, live site verification, and GTV requirements
- [x] 5.2 Update ADOPTION_PLAYBOOK.md: add "page inventory" step before convergence — DONE: Phase 2 now includes route inventory (2.1b, 2.1c, 2.1d), Phase 8 rewritten with 8C (page-level), 8D (live site), 8E (GTV)
- [x] 5.3 The convergence gate: "all pages pass" not "project passes" — DONE: Phase 8F explicitly requires all pages pass all dimensions, checklist items 8.5-8.18

## Phase 6: Live Site Verification (GTV for Pages)

- [ ] 6.1 For deployed sites: fetch each page via HTTP
- [ ] 6.2 Check rendered HTML (not just source code)
- [ ] 6.3 Check contrast on rendered output
- [ ] 6.4 Check form rendering (are labels above inputs in the DOM?)
- [ ] 6.5 Compare source code claims vs rendered reality

## What This Prevents

| Failure | How It Slipped Through | Per-Page Audit Catches It |
|---|---|---|
| Fields floating left | Project has form components → "PASS" | Audits the SPECIFIC page → sees CSS layout issue |
| "(Optional)" on every field | Form helper exists with optional marking → "looks fine" | Reads the SPECIFIC template → counts optional vs required |
| Tiny textarea | Component library has textarea → "PASS" | Checks the SPECIFIC form → sees input type="text" used instead |
| White text on light bg | Contrast variables defined → "PASS" | Scans the SPECIFIC page's classes → finds text-gray-400 |
| Missing mobile nav | Nav component exists → "PASS" | Fetches page at mobile width → sees no hamburger |

## The Rule

**Convergence is not "the project looks good." Convergence is "every page passes every applicable dimension."**
