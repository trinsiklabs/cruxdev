# BUILD_PLAN_030: OpenClaw Page Ground Truth + TypeScript Toolchain Integration

**Status:** NOT STARTED
**Priority:** Critical (public claims that aren't true)
**Depends on:** BUILD_PLAN_029 (Intel binary needed for OpenClaw users)

## Context

The /docs/openclaw page makes 8 FALSE claims and 2 PARTIAL claims about CruxDev's capabilities. These were written aspirationally without ground truth verification. This is a convergence process failure — the page should never have shipped with unverified claims.

### Audit Results

| Claim | Status | Issue |
|-------|--------|-------|
| 9 code dimensions | TRUE | router.rs:15 |
| 5 doc dimensions | TRUE | router.rs:16 |
| Two consecutive clean passes | TRUE | convergence.rs |
| Safety gates (3-failure, timeout, net-negative) | TRUE | convergence.rs, state.rs |
| 52 MCP tools | TRUE | server.rs |
| Adoption playbook | TRUE | ADOPTION_PROCESS.md |
| 100% test coverage per skill | **FALSE** | No per-skill measurement |
| Security on risky skills | **PARTIAL** | Dimension exists, no conditional logic |
| Input validation convergence | **FALSE** | No dedicated mechanism |
| Jest/Vitest integration | **FALSE** | Generic subprocess only |
| ESLint/Biome integration | **FALSE** | No linter integration |
| TypeScript strict mode | **FALSE** | No TS compilation |
| npm audit integration | **FALSE** | Not built-in |
| Multi-channel testing | **FALSE** | No channel awareness |
| API contract convergence | **FALSE** | No schema handling |
| Docker deployment testing | **FALSE** | Detection only, no testing |

## Phase 1: Fix the OpenClaw Page (Immediate)

- [ ] 1.1 Remove all FALSE claims
- [ ] 1.2 Reframe PARTIAL claims honestly ("the security dimension is included in every audit" not "security audit on every skill that touches filesystem")
- [ ] 1.3 Clearly separate "works today" from "roadmap"
- [ ] 1.4 State what the engine ACTUALLY does: runs any test command via subprocess, audits via LLM against dimensions, converges through phases

## Phase 2: Add Ground Truth Verification to Website Convergence Process

- [ ] 2.1 Add to WEBSITE_PLANNING.md §10.2.1:
  "Ground truth verification: every technical capability claim must reference the specific file and function that implements it. If no code implements the claim, the claim cannot be on the page."
- [ ] 2.2 Add to DEVELOPMENT_PATTERNS_CRUXDEV.md anti-patterns:
  "Aspirational documentation: writing claims about features that don't exist yet as if they're implemented. Ship the code first, then document it."

## Phase 3: Implement TypeScript/Node.js Toolchain Integration

### 3.1 Test runner enhancements
- [ ] 3.1.1 Parse Jest/Vitest JSON output (--reporter=json) for structured results
- [ ] 3.1.2 Parse coverage reports (istanbul JSON summary) for per-file/per-function coverage
- [ ] 3.1.3 Detect test framework from package.json (jest/vitest/mocha)
- [ ] 3.1.4 Auto-configure test command from package.json scripts

### 3.2 Linter integration
- [ ] 3.2.1 Detect ESLint/Biome from package.json or config files
- [ ] 3.2.2 Run linter as part of code audit, parse results
- [ ] 3.2.3 Map lint findings to code dimensions (security rules → security, complexity → maintainability)

### 3.3 TypeScript support
- [ ] 3.3.1 Detect tsconfig.json, check if strict is enabled
- [ ] 3.3.2 Run `tsc --noEmit` as compilation check
- [ ] 3.3.3 Report type errors as findings

### 3.4 Dependency security
- [ ] 3.4.1 Run `npm audit --json` and parse results
- [ ] 3.4.2 Map vulnerability severities to finding severities
- [ ] 3.4.3 Add "dependency_security" to CODE_DIMENSIONS when package.json detected

### 3.5 Coverage enforcement
- [ ] 3.5.1 Parse istanbul coverage-summary.json
- [ ] 3.5.2 Report uncovered files/functions as findings
- [ ] 3.5.3 Configurable threshold (default 100% for CruxDev projects)

## Phase 4: Tests

- [ ] 4.1 Unit tests for Jest/Vitest JSON parsing
- [ ] 4.2 Unit tests for coverage report parsing
- [ ] 4.3 Unit tests for package.json detection
- [ ] 4.4 E2E test: convergence on a TypeScript project

## Phase 5: Update OpenClaw Page with Verified Claims

- [ ] 5.1 Re-add TypeScript claims ONLY after Phase 3 implementation
- [ ] 5.2 Each claim references the specific code that implements it
- [ ] 5.3 Full content convergence audit on the updated page

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
# Verify no false claims remain
grep -c "Jest\|Vitest\|ESLint\|Biome\|TypeScript strict\|npm audit" cruxdev-dev/src/pages/docs/openclaw.astro
```
