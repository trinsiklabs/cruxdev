# BUILD_PLAN_047: Harness Gap Closure — Learn from Competitors

**Status:** CONVERGED
**Priority:** High
**Depends on:** BP046 (harness competitive research)

## Context

Deep competitive research against harness competitors revealed three genuine gaps where CruxDev falls short. These are not aspirational — they're capabilities competitors have that we don't, validated at production scale.

## Gap 1: KV-Cache-Aware Context Engineering (from Manus)

**What Manus proved:** Cached tokens cost $0.30/MTok vs $3.00/MTok uncached — 10x difference. With Claude Sonnet, stable prompt prefixes and append-only context dramatically improve cache hit rates.

**Why it matters for CruxDev:** Convergence loops repeat similar prompts many times (audit → fix → re-audit). This is exactly the workload that benefits most from cache-friendly context design.

**What to implement:**
- [ ] Append-only context in convergence loops (never modify earlier messages)
- [ ] Stable prompt prefixes (no timestamps or variable content at the start)
- [ ] Tool masking instead of tool removal (preserve schema for cache)
- [ ] Error preservation in context (don't clean up failed attempts — model learns from them)
- [ ] Controlled variation to break repetitive drift patterns
- [ ] Measure: track KV-cache hit rate as a first-class metric

## Gap 2: Durable Execution / Checkpointing (from DeepAgents)

**What DeepAgents has:** LangGraph checkpointing — mid-task snapshot that survives crashes and resumes from last checkpoint.

**Why it matters for CruxDev:** A convergence loop crash at step 47 of 50 means restarting. With checkpointing, it resumes at 47.

**What to implement:**
- [ ] Checkpoint convergence state after each round (already have atomic writes — extend to full checkpoint)
- [ ] Resume from last checkpoint on restart (already partially have this)
- [ ] Checkpoint includes: full state, current phase, round, findings history, context
- [ ] Test: kill process mid-convergence, restart, verify resume from last round

## Gap 3: Self-Testing with Visual Verification (from Cursor)

**What Cursor has:** Cloud VMs where agents start apps, interact visually, take screenshots, record video, and verify their own changes.

**Why it matters:** For frontend/UI work, or any behavior not covered by automated tests, test suites aren't enough.

**What to implement (phased):**
- [ ] Phase 1: Screenshot capture after build (headless browser)
- [ ] Phase 2: Visual diff between before/after screenshots
- [ ] Phase 3: Agent-driven browser interaction for E2E verification
- [ ] This is a large capability — don't try to match Cursor's full cloud VM approach. Start with screenshot-based verification.

## Validation

These three gaps came from honest competitive research, not assumptions. Each is implemented and proven at production scale by a well-funded competitor. Closing them makes CruxDev's convergence harness competitive with the best runtime harnesses while maintaining the autonomous convergence advantage none of them have.
