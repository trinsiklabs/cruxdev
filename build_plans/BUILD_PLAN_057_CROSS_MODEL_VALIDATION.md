# BUILD_PLAN_057: Cross-Model Validation for Audit

**Status:** CONVERGED
**Priority:** Should Close
**Source:** COCO paper (arxiv 2508.13815) — correlated errors when same LLM generates and audits

## Context

Using the same LLM to generate code and audit it creates correlated errors. The COCO paper recommends using different models for generation vs audit. CruxDev's dispatch layer already supports multiple providers — this wires cross-model validation.

## Phase 1: Audit Model Configuration

- [ ] 1.1 Add `audit_model` config to convergence settings (default: same as generation)
- [ ] 1.2 When audit_model differs from generation model, route audit prompts to different provider
- [ ] 1.3 Support: audit with Sonnet while generating with Opus, or vice versa

## Phase 2: Dispatch Layer Integration

- [ ] 2.1 Extend dispatch to tag requests as "generation" or "audit"
- [ ] 2.2 Route based on tag: generation → model A, audit → model B
- [ ] 2.3 Log which model handled each request

## Phase 3: Tests

- [ ] 3.1 Test: generation and audit routed to different models when configured
- [ ] 3.2 Test: default behavior unchanged (same model for both)
- [ ] 3.3 Test: dispatch logging includes model tag

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
