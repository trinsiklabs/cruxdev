# BUILD_PLAN_098: Ground Truth Verification to Convergence

**Status:** CONVERGED
**Priority:** Critical (correctness foundation for Crux Bot)

## Context

GTV is the difference between "the LLM said it's done" and "it's actually done." Every claim, assumption, and assertion must be verifiable against reality. No guessing. No assuming. Maximum reality alignment.

Currently GTV exists as a checklist in the adoption process. It needs to become a first-class convergence phase — the FIRST thing that runs, and the LAST thing that checks.

## The Problem

LLMs hallucinate. They claim files exist that don't. They assert functions work that don't compile. They say "tests pass" without running them. They reference API endpoints that return 404. They cite competitor features that were deprecated. They write docs that describe aspirational behavior, not actual behavior.

Every one of these happened in THIS SESSION:
- Website claimed 419 tests when actual was 462
- Blog post linked to `/blog/atomic-writes-protocol-versioning/` but page was at `/blog/20260327-atomic-writes-protocol-versioning/`
- robots.txt referenced `sitemap.xml` but actual was `sitemap-index.xml`
- Typefully API called with v1 schema when v2 was required
- evolve CLI passed `--dry-run false` which isn't valid clap syntax
- Tool pages referenced `Doc.astro` layout — sometimes existed, sometimes didn't
- Homepage said "57 MCP tools" when actual was 61
- Footer said "451 tests" when actual was 485

## GTV as a Convergence Phase

```
Current phases:
Planning → PlanAuditing → DocAlignment → Executing → CodeAuditing → DocAuditing → WebsiteConvergence → E2eTesting → PatternsUpdate → Converged

With GTV:
GTV_PRE → Planning → PlanAuditing → DocAlignment → Executing → CodeAuditing → DocAuditing → WebsiteConvergence → E2eTesting → PatternsUpdate → GTV_POST → Converged
```

### GTV_PRE: Before convergence begins

Verify every assumption in the build plan:
- [ ] Files referenced in the plan exist
- [ ] APIs referenced are accessible and respond as expected
- [ ] Dependencies are installed and at correct versions
- [ ] Test commands work (`cargo test`, `npm test`, etc.)
- [ ] Referenced competitors/features are current (not deprecated)
- [ ] Build commands work
- [ ] Deploy targets are accessible

### GTV_POST: After convergence, before declaring done

Verify every claim made during convergence:
- [ ] All modified files compile/parse
- [ ] Test suite passes (not just "I ran it" — actually execute and check exit code)
- [ ] Website pages return 200 (not just "I deployed" — actually curl every page)
- [ ] Links in docs/website resolve (not just "I added links" — actually check them)
- [ ] Stats in UI match reality (test count, tool count, coverage number)
- [ ] API integrations work (Typefully, GitHub, Cloudflare — actually call them)
- [ ] Blog posts render correctly (not just "I wrote them" — actually fetch and check)
- [ ] Git is clean and pushed (not just "I committed" — actually check remote)

## GTV Categories

### 1. Code GTV
- File exists at claimed path
- Function/struct exists with claimed signature
- Code compiles without errors
- Tests pass with exit code 0
- Coverage number matches claim

### 2. Web GTV
- URL returns expected status code
- Page contains expected content
- Links resolve (no 404s)
- Structured data validates
- SSL certificate is valid

### 3. API GTV
- Endpoint responds
- Authentication works
- Request/response schema matches documentation
- Rate limits are respected

### 4. Data GTV
- Config files parse correctly
- JSON/TOML/YAML are valid
- Database migrations applied
- Seed data present

### 5. Claim GTV
- "X tests passing" → actually run tests, count matches
- "Deployed to production" → actually curl the URL
- "Git pushed" → actually check remote HEAD
- "Blog post live" → actually fetch the page
- "Typefully posted" → actually check draft exists

### 6. Competitive GTV
- Competitor feature claims are current (not deprecated)
- Star counts are recent (not from 3 months ago)
- Pricing is current
- URLs resolve

## Files

- `rust/src/gtv/cache.rs` — GTV cache with TTL per check type
- `rust/src/gtv/mod.rs` — extend with GtvCheck, GtvResult, cache types (already exists)
- `rust/src/engine/router.rs` — add GTV_DIMENSIONS constant
- `rust/src/engine/convergence.rs` — add GTV_PRE and GTV_POST phases
- `rust/src/server.rs` — wire GTV phases into convergence tool responses

## Tests

- [ ] test_gtv_cache_stores_and_retrieves
- [ ] test_gtv_cache_ttl_expiry
- [ ] test_gtv_cache_invalidation_on_file_change
- [ ] test_gtv_pre_blocks_on_failure
- [ ] test_gtv_post_verifies_claims
- [ ] test_gtv_dimensions_in_router
- [ ] test_gtv_force_bypasses_cache
- [ ] test_gtv_cache_persistence_roundtrip

## Implementation for Crux Bot

```rust
/// Ground truth verification — verify claims against reality
pub struct GtvCheck {
    pub claim: String,        // What was claimed
    pub verification: String, // How to verify
    pub result: GtvResult,    // Pass/Fail/Skip
    pub evidence: String,     // What we found
}

pub enum GtvResult {
    Pass,                     // Claim matches reality
    Fail(String),            // Claim doesn't match — here's what's actually true
    Skip(String),            // Can't verify — reason
}

/// Run all GTV checks for a convergence state
pub fn verify_ground_truth(state: &ConvergenceState) -> Vec<GtvCheck> {
    let mut checks = vec![];

    // Code GTV
    checks.extend(verify_files_exist(&state.modified_files));
    checks.extend(verify_tests_pass(&state.test_command));
    checks.extend(verify_builds(&state.build_command));

    // Web GTV
    if let Some(url) = &state.deploy_url {
        checks.extend(verify_urls_respond(url));
        checks.extend(verify_internal_links(url));
    }

    // Claim GTV
    checks.extend(verify_stat_claims(&state));

    checks
}
```

## GTV Dimensions for Router

```rust
pub const GTV_DIMENSIONS: &[&str] = &[
    "file_existence",
    "compilation",
    "test_execution",
    "url_accessibility",
    "link_integrity",
    "stat_accuracy",
    "api_connectivity",
    "config_validity",
    "claim_verification",
];
```

## Anti-Patterns (What GTV Prevents)

| Anti-Pattern | Example | GTV Check |
|---|---|---|
| Aspirational docs | "Supports 20 languages" (actually 1) | Verify each claimed language has content |
| Phantom features | "Real-time dashboard" (no dashboard exists) | Verify URL exists and renders |
| Stale stats | "451 tests" (actually 485) | Run tests, compare count |
| Broken links | Blog index links to wrong slug | Fetch every link, check 200 |
| Dead APIs | Typefully returns 403 | Actually call the API |
| Assumed deploys | "Deployed and live" (never deployed) | Curl the URL |
| Config drift | growth.toml says 52 tools (actually 61) | Count tools, compare |

## The Crux Bot GTV Loop

```
Before EVERY convergence cycle:
  1. Read the build plan
  2. Extract every assumption and referenced resource
  3. Verify each one against reality
  4. If ANY fail → fix the plan before starting convergence

After EVERY convergence cycle:
  1. Collect every claim made during convergence
  2. Verify each claim against reality
  3. If ANY fail → NOT CONVERGED, go back and fix
  4. Only declare converged when ALL claims pass GTV
```

## GTV Cache — Don't Re-Verify What Was Just Verified

GTV checks are expensive (API calls, HTTP requests, test runs). Cache results with TTL:

```rust
pub struct GtvCache {
    checks: HashMap<String, GtvCacheEntry>,
}

pub struct GtvCacheEntry {
    result: GtvResult,
    verified_at: DateTime,
    ttl: Duration,
}
```

### TTL by Check Type

| Check Type | TTL | Rationale |
|---|---|---|
| File existence | 5 minutes | Files change during convergence |
| Test execution | 10 minutes | Tests don't change between rounds |
| URL accessibility | 1 hour | Websites don't go down frequently |
| API connectivity | 30 minutes | APIs are stable short-term |
| Stat accuracy | Per convergence | Stats change when code changes |
| Competitor info | 7 days | Competitor features change slowly |
| Link integrity | 1 hour | Links don't break often |

### Cache Rules

1. GTV_PRE can use cached results if within TTL
2. GTV_POST always runs fresh for claims made THIS convergence
3. Cache is per-project, persisted to `.cruxdev/gtv_cache.json`
4. Cache is invalidated when relevant files change (file watcher)
5. `--force-gtv` flag bypasses cache for full re-verification
6. Stale cache entries are logged as warnings, not errors

### Cost Impact

Without cache: every convergence cycle runs 20-50 GTV checks (~30 seconds)
With cache: first cycle runs all, subsequent cycles run 3-5 (changed items only)

## This Is What Makes the Bot Reliable

AutoGPT says "done" when the LLM says done.
Devin says "done" when the tests pass.
Crux Bot says "done" when EVERY CLAIM IS VERIFIED AGAINST REALITY.

That's the difference between 14% success rate and actual reliability.
