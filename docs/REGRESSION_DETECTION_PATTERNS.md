# Regression Detection Between Convergence Passes

**Source:** IEEE-ISTAS 2025 (arxiv 2506.11022) — 37.6% increase in critical security vulnerabilities after 5 LLM iterations.

---

## The Problem

Each LLM refinement pass can fix issues AND introduce new ones. The IEEE paper found security vulnerabilities INCREASED 37.6% after 5 iterations. CruxDev's two-consecutive-clean-pass criterion catches new findings but doesn't detect regressions in previously-clean dimensions.

## Detection Rules

1. Track dimension scores per round: `HashMap<String, Vec<f64>>`
2. After each round, compare current scores against previous round
3. If ANY dimension score decreases: flag as regression finding (HIGH severity)
4. Security dimension regression → immediate escalation
5. Before declaring convergence, verify no regressions across entire history

## Implementation

```rust
// In convergence state
pub dimension_scores: HashMap<String, Vec<f64>>,

// After each audit round
fn detect_regressions(current: &HashMap<String, f64>, previous: &HashMap<String, f64>) -> Vec<Finding> {
    current.iter()
        .filter(|(dim, score)| {
            previous.get(*dim).map(|prev| *score < prev).unwrap_or(false)
        })
        .map(|(dim, score)| Finding {
            dimension: dim.clone(),
            severity: if dim == "security" { High } else { Medium },
            description: format!("Regression: {} dropped from {:.1} to {:.1}", dim, previous[dim], score),
        })
        .collect()
}
```

## Convergence Gate Enhancement

Current: two consecutive clean passes.
Enhanced: two consecutive clean passes WITH no regressions from any prior pass.

## Anti-Patterns

- Ignoring dimension score trends (only looking at pass/fail)
- Treating all regressions equally (security is higher severity)
- Allowing convergence when a dimension regressed then recovered (may indicate instability)
