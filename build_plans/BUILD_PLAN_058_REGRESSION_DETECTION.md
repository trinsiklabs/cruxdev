# BUILD_PLAN_058: Regression Detection Between Passes

**Status:** CONVERGED
**Priority:** Should Close
**Source:** IEEE-ISTAS 2025 (arxiv 2506.11022) — 37.6% security increase after 5 LLM iterations

## Context

The IEEE paper found that iterative LLM refinement can INCREASE security vulnerabilities. CruxDev's two-consecutive-clean-pass criterion catches new findings but doesn't detect regressions in previously-clean dimensions.

## Phase 1: Per-Dimension Score History

- [ ] 1.1 Track dimension scores per round (not just findings count)
- [ ] 1.2 Store in convergence state: `dimension_scores: HashMap<String, Vec<f64>>`
- [ ] 1.3 Compare current round's scores against previous round

## Phase 2: Regression Detection

- [ ] 2.1 If any dimension score decreases between rounds, flag as regression
- [ ] 2.2 Regression findings get HIGH severity automatically
- [ ] 2.3 Regression in security dimension triggers immediate escalation
- [ ] 2.4 New finding type: `regression` (distinct from `new_finding`)

## Phase 3: Regression Prevention

- [ ] 3.1 Before declaring convergence, verify no regressions across all rounds
- [ ] 3.2 If regression detected in final pass, require additional clean pass
- [ ] 3.3 Log regression patterns for analysis

## Phase 4: Tests

- [ ] 4.1 Test: regression detected when score decreases
- [ ] 4.2 Test: security regression triggers escalation
- [ ] 4.3 Test: no false positives on normal score variation

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
