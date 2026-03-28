# BUILD_PLAN_088: Audit Trail UI / Observability

**Status:** CONVERGED
**Priority:** Should Close
**Competitive gap:** Developer trust gap — 46% distrust AI code. Convergence is verified but not visible.

## Context

CruxDev produces extensive state files (WAL, checkpoints, convergence history, findings) but none of it is human-readable without parsing JSON. Developers need to SEE the convergence happening — which dimensions were audited, what findings were discovered, how they were fixed, why convergence was declared.

## Phase 1: Convergence Report Generator

- [ ] 1.1 New Rust module: `rust/src/engine/report.rs`
- [ ] 1.2 `generate_convergence_report(state) -> ConvergenceReport`
- [ ] 1.3 Report includes: plan name, phases traversed, rounds per phase, findings by dimension, fixes applied, final scores, timeline
- [ ] 1.4 Output formats: markdown, JSON, HTML
- [ ] 1.5 MCP tool: `convergence_report(convergence_id)` — returns formatted report

## Phase 2: Per-Round Summary

- [ ] 2.1 After each round, generate one-line summary: "Round 3: 2 findings in security, 1 in maintainability. 3 fixed."
- [ ] 2.2 Include in convergence_submit_result response
- [ ] 2.3 Accumulate in state for final report

## Phase 3: Web Dashboard (Future)

- [ ] 3.1 Static HTML report generated after convergence
- [ ] 3.2 Timeline visualization (phases, rounds, findings)
- [ ] 3.3 Dimension score heatmap
- [ ] 3.4 Deployable alongside project docs

## Phase 4: Content Generation

- [ ] 4.1 Blog post + X post via BIP pipeline

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
