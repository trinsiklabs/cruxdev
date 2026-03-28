# External Review — Ground Truth Verification

Verified: 2026-03-28

Each claim from the external review is checked against the actual codebase and live site.

---

## 1. MCP Tool Count Discrepancies

**Claim:** "61 MCP tools" on website vs "52" in README vs "60" in blog.

**Actual:** 63 tools (counted via `#[tool(` attributes in `rust/src/server.rs`).

| Source | Claims | Actual | Status |
|--------|--------|--------|--------|
| Website homepage (index.astro) | 61 | 63 | **STALE** — needs update to 63 |
| README.md | 63 | 63 | **ACCURATE** (recently updated) |
| llms.txt | 61 | 63 | **STALE** — needs update to 63 |
| engine.astro | 63 | 63 | **ACCURATE** |
| Blog (session-kickoff) | 60 | 63 | **STALE** — historical blog post, acceptable |
| Blog (crux-ecosystem) | 61 | 63 | **STALE** — recent blog post, should fix |
| CHANGELOG v0.2.0 | 52 | 63 | **STALE** — was accurate at time of v0.2.0 release |

**Action needed:** Update homepage (index.astro) and llms.txt from 61 to 63.

---

## 2. Test Count Discrepancies

**Claim:** "487 tests on website, 402 in README, 481 in blog, 485 in another."

**Actual:** 520 Rust tests (502 unit + 18 E2E). Also 1,128 Python tests still exist (legacy, not counted in official numbers).

| Source | Claims | Actual | Status |
|--------|--------|--------|--------|
| Website homepage (index.astro) | 487 | 520 | **STALE** — needs update |
| README.md | 520 | 520 | **ACCURATE** (recently updated) |
| llms.txt | 485 | 520 | **STALE** — needs update |
| Blog (485-tests.md) | 485 | 520 | **STALE** — historical, acceptable |
| Blog (github-issue-monitoring) | 485 | 520 | **STALE** |
| CHANGELOG v0.2.0 | "was 39" tools | n/a | Historical, acceptable |

**Action needed:** Update homepage (index.astro) from 487 to 520. Update llms.txt from 485 to 520.

---

## 3. Audit Dimensions Count

**Claim:** "39 audit dimensions across 10 audit sets."

**Actual:** 146 dimensions across 21 dimension sets (in `rust/src/engine/router.rs`).

The "39 across 10 sets" number is deeply stale. It was accurate at an earlier point but the codebase now has:
- PLAN (5), CODE (9), DOC (5), FORM (17), METRICS (7), DASHBOARD (9), MCP_SERVER (7), SKILL (7), CONTENT (8), BUSINESS (6), MEDIA (6), UI_COMPONENT (7), COLOR_CONTRAST (5), LOGO (4), POST_DEPLOYMENT (7), E2E_TEST (6), MOBILE_WEB (6), GEO (6), UAT_TEST (5), BDD (5), GTV (9)

| Source | Claims | Actual | Status |
|--------|--------|--------|--------|
| README.md | "137 audit dimensions across 21 dimension sets" | 146 across 21 | **STALE** — 137 is behind, 21 sets is correct |
| Website meta description (index.astro) | "137 audit dimensions across 20 dimension sets" | 146 across 21 | **STALE** |
| llms.txt | "39+" | 146 | **STALE** |
| engine.astro header | "61 MCP tools" (not dimension count) | n/a | see #1 |
| /for/software-engineers | "39 dimensions across 10 audit sets" | 146 across 21 | **STALE** |
| /vs/ pages (cursor, codex, manus, claude-code, deepagents) | "39 dimensions" | 146 | **STALE** |
| Blog (harness-competitive-landscape) | "39 audit dimensions" | 146 | **STALE** |

**Action needed:** Major update across website. The "39 across 10" number is on multiple /vs/ pages, the /for/software-engineers page, and blog posts. README says 137 but actual is now 146.

---

## 4. README Numbers

**Claim:** README says "52 MCP tools" and "402 tests."

**Actual:** README now says "63 MCP tools" and "520 tests (502 unit + 18 E2E)."

**Status: FALSE** — the review's claim is itself stale. README has been updated since the review was written.

---

## 5. Codebase Language Ratio

**Claim:** "The codebase is 50.3% Rust and 49% Python."

**Actual:** Rust is ~21,762 lines across `rust/src/`. Python is ~21,822 lines across `src/` and `tests/`. Ratio is roughly 50/50 still, but the Python is legacy (tests still run but migration is to Rust). The README and site present CruxDev as a "single Rust binary."

**Status: ACCURATE** by line count, but **MISLEADING** — the Rust code is the shipping product; the Python is legacy being maintained in parallel. The review's framing is factually correct but misses the migration context.

---

## 6. Release Tags

**Claim:** "Two releases total, v0.1.0 and v0.2.0."

**Actual:** `git tag` shows only `v0.2.0`. GitHub releases show only `v0.2.0`.

CHANGELOG.md documents both v0.1.0 and v0.2.0, but v0.1.0 was never tagged as a git release.

**Status: FALSE** — there is only 1 tagged release (v0.2.0), not 2. v0.1.0 exists only in the CHANGELOG as a historical marker.

---

## 7. GitHub Stats

**Claim:** "0 stars, 0 forks, 1 contributor."

**Actual:** 0 stars, 0 forks, 1 contributor (confirmed via `gh api`).

**Status: ACCURATE**

---

## 8. llms.txt Tool Count

**Claim:** "llms.txt said 52 MCP tools."

**Actual:** llms.txt currently says "61 MCP tools" (not 52). The review itself may have been written when it said 52, or the reviewer misread.

**Status: FALSE** — llms.txt says 61, not 52. However, 61 is itself stale (actual is 63). See claim #1.

---

## 9. Windows Support

**Claim:** "No Windows support."

**Actual:** No Windows target in CI workflows (`.github/workflows/`). Release binaries are macOS ARM, macOS Intel, Linux x86_64 only. README install section lists only these three platforms.

**Status: ACCURATE**

---

## 10. Jest/Vitest, Istanbul, ESLint, npm audit

**Claim:** "Explicitly listed as planned but not yet built."

**Actual:** All four are **implemented** in `rust/src/engine/toolchain.rs`:
- Jest/Vitest JSON output parsing (line 101+)
- Istanbul/NYC coverage-summary.json parsing (line 142+)
- ESLint detection (line 312+)
- npm audit JSON parsing (line 179+)
- Tests exist for all parsers (lines 408+, 425+, 369+)

**Status: FALSE** — these features ARE built and tested. The review is wrong.

---

## 11. Engineer-Facing Page Dimensions

**Claim:** "The engineer-facing page describes 39 audit dimensions across 10 audit sets."

**Actual:** `/for/software-engineers.astro` says "39-dimension audit" and "10 audit sets." The actual codebase has 146 dimensions across 21 sets.

**Status: ACCURATE** about what the page says, and correctly identifies it as a discrepancy with the codebase.

---

## 12. Project Classification — "8 of 18 Need Manual"

**Claim:** "8 of 18 project types need manual classification."

**Actual:** In `rust/src/adoption/classify.rs`, the `type_signals()` function defines file-pattern signals for 16 of 18 types. Two types have empty signal vectors (`SoftwareGreenfield`, `BusinessNew`) and `Composite` is not in the signal list at all. That's 3 types with no auto-detection signals, not 8. However, some types have very specific signals (e.g., `ConsultingClient` needs `clients/`, `proposals/`, or `deliverables/` directories) that many projects won't match, so in practice more could fall back to `SoftwareGreenfield` default.

**Status: FALSE** — only 2-3 types strictly require manual classification (SoftwareGreenfield, BusinessNew, Composite). The reviewer likely counted types with narrow signals as "manual" but the engine does auto-detect them when signals are present.

---

## 13. /for/entrepreneurs "What's Not Covered" Items

**Claim:** Check if any "not covered yet" items are now covered.

**Actual items listed:**
1. Financial modeling — NOT built (no financial modeling in codebase)
2. Pitch deck generation — NOT built
3. Team management (user roles, permissions) — NOT built
4. Customer analytics — NOT built

**Status: ACCURATE** — all four items remain uncovered. No action needed.

---

## 14. Comparison Page "Agent = Model + Harness" Framing

**Claim:** "The comparison page frames the market as: Agent = Model + Harness."

**Actual:** `/vs/index.astro` line 60: "The industry equation (Q1 2026): Agent = Model + Harness. The model is commodity. The harness is moat."

**Status: ACCURATE**

---

## 15. GStack Coverage

**Claim:** "CruxDev doesn't cover GStack anywhere on the site."

**Actual:** No matches for "GStack" or "gstack" anywhere in the cruxdev-dev site.

**Status: ACCURATE** — GStack is not mentioned on the site.

---

## 16. "30% Miss Rate" — No Linked Primary Source

**Claim:** "The 30% miss rate claim has no linked primary source."

**Actual:** The ~30% claim appears on:
- Homepage (index.astro)
- Methodology page
- Multiple landing pages (stop-re-prompting, when-is-code-done, ai-coding-mistakes)
- Schema.org FAQ markup in when-is-code-done.astro

Only the landing pages have inline citations to "research on code review effectiveness" but none link to a specific study URL. The internal docs reference it as an empirical finding (CruxDev.md line 571: "~30% false-negative rate (empirical: R2 was 'clean'...)").

**Status: ACCURATE** — the 30% claim is used extensively but never links to a specific published study. It appears to be derived from internal empirical observation, not external research. The phrasing "research shows" is misleading without a citation.

---

## 17. Performance Statistics on Research Page

**Claim:** "90.2% better than single-agent, 35.9% cost reduction, 94-99% citation accuracy."

**Actual:** All three numbers appear in `/docs/research.astro`:
- "This multi-agent approach outperforms single-agent by 90.2%" (line 52)
- "reduces cost by 35.9% and time by 40.6% vs. sequential approaches" (line 21)
- "schema-driven extraction reaches 94-99% accuracy" (line 49)

These are stated as facts without linked citations. The page does reference Perplexity/ChatGPT/Grok hallucination rates (37%, 67%, 94%) which are verifiable, but the 90.2%, 35.9%, and 94-99% numbers have no source links.

**Status: ACCURATE** — the numbers exist on the site. The review's concern about missing sources is valid.

---

## 18. CHANGELOG Accuracy

**Claim:** Check CHANGELOG for accuracy of v0.1.0 and v0.2.0 contents.

**v0.1.0 claims:**
- "39 tools" — plausible for initial release
- "271 tests" — plausible for initial release
- "0 clippy warnings" — likely accurate

**v0.2.0 claims:**
- "52 MCP tools (was 39)" — **STALE** (actual is now 63, but was 52 at time of v0.2.0 release; the issue is that v0.2.0 was the last release and development continued)
- "TypeScript toolchain detection (Jest/Vitest, ESLint/Biome, coverage, npm audit, tsconfig)" — **ACCURATE** (verified in toolchain.rs)
- "228 templates" — would need separate verification
- Other features listed — generally accurate per code review

**Status: ACCURATE** for what was true at release time. The issue is that no v0.3.0 release has been cut despite significant development since v0.2.0.

---

## Summary of Fixes Needed

### Critical (numbers wrong on live site)

| File | Issue | Fix |
|------|-------|-----|
| `cruxdev-dev/src/pages/index.astro` | Shows 487 tests | Update to 520 |
| `cruxdev-dev/src/pages/index.astro` | Shows 61 MCP tools | Update to 63 |
| `cruxdev-dev/public/llms.txt` | Shows 485 tests, 61 tools | Update to 520 tests, 63 tools |
| `cruxdev-dev/src/pages/for/software-engineers.astro` | Shows "39 dimensions across 10 audit sets" | Update to 146 across 21 |
| `cruxdev-dev/src/pages/vs/*.astro` (5 files) | Shows "39 dimensions" | Update to 146 |
| `cruxdev-dev/src/pages/vs/index.astro` | Shows "39 dimensions across 10 sets" | Update to 146 across 21 |
| `cruxdev/README.md` | Shows "137 audit dimensions" | Update to 146 |

### Medium (missing citations)

| Page | Issue |
|------|-------|
| Homepage, methodology, landing pages | "~30% miss rate" has no linked primary source |
| /docs/research.astro | 90.2%, 35.9%, 94-99% numbers have no source links |

### Low (historical blog posts)

Blog posts with stale numbers are historical records and generally acceptable, but recent posts (crux-ecosystem with "61 tools") could be updated.

### Process Gap

No v0.3.0 release has been cut despite 63 tools (was 52 at v0.2.0), 520 tests (was implied ~402-485), and 146 dimensions (was 39). A new release would align the CHANGELOG with reality.
