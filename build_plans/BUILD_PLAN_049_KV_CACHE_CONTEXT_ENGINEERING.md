# BUILD_PLAN_049: KV-Cache-Aware Context Engineering

**Status:** NOT STARTED
**Priority:** Must Close
**Depends on:** BP047 (harness gap closure)
**Competitor:** Manus (proven 10x cost savings, 4x speed improvement)

## Context

Cached tokens cost $0.30/MTok vs $3.00/MTok uncached — 10x difference. Convergence loops repeat similar prompts many times (audit → fix → re-audit). This workload benefits most from cache-friendly context design. Manus proved this at production scale.

## Phase 1: Append-Only Context in Convergence Loops

- [ ] 1.1 Ensure convergence loop context is append-only (never modify earlier messages)
- [ ] 1.2 Move all variable content to the END of prompts (timestamps, round numbers)
- [ ] 1.3 Stable prompt prefixes: system prompt + CLAUDE.md + methodology docs are frozen at loop start
- [ ] 1.4 Test: verify prompt prefix is identical across rounds (byte-compare first N tokens)

## Phase 2: Tool Masking Instead of Tool Removal

- [ ] 2.1 When tools aren't needed for a phase, mask them (mark unavailable) rather than removing from schema
- [ ] 2.2 Preserves tool schema in context → preserves KV cache
- [ ] 2.3 Test: tool schema identical across phases (only availability changes)

## Phase 3: Error Preservation

- [ ] 3.1 Keep failed attempts in context (don't clean up errors)
- [ ] 3.2 Model learns from errors without re-encountering them
- [ ] 3.3 Add controlled variation after 3+ identical attempts to break repetitive drift

## Phase 4: Cache Hit Rate Tracking

- [ ] 4.1 Track cache_creation_input_tokens and cache_read_input_tokens from API responses
- [ ] 4.2 Compute cache hit rate as first-class metric
- [ ] 4.3 Log per-round and per-convergence cache metrics
- [ ] 4.4 Alert if cache hit rate drops below 50%

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
