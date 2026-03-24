# BUILD_PLAN_010: Universal Research Engine

**Created:** 2026-03-24
**Status:** NOT STARTED
**Goal:** Bake the 5-pass research methodology into CruxDev as a universal research engine. All research activities — competitive analysis, feature evaluation, technology decisions, content writing — use the same convergence-detected, quality-scored, counter-researched pipeline.

**Derived from:** Production-proven V3 research methodology.
**Methodology:** Follow `docs/RESEARCH_PATTERNS.md` + `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md`.

**Rule:** TDD. Tests before code. 100% coverage.
**Rule:** Every claim verified. Every link tested. Counter-research mandatory.
**Rule:** Research is permanent. Content is derived.

---

## Document Alignment

### Product Docs:
- docs/RESEARCH_PATTERNS.md — the methodology this plan implements
- docs/COMPETITORS_PATTERN.md — competitors system must use research engine
- docs/WEBSITE_PLANNING.md — website content must use research-first architecture
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — convergence rules apply

### Memory Files:
None — new capability.

---

## Phase 1: Research Engine Core

**Purpose:** Build the 5-pass search system with convergence detection and quality scoring.

- [ ] 1.1 `src/research/__init__.py`
- [ ] 1.2 `src/research/session.py` — research session state, checkpointing, recovery
- [ ] 1.3 `src/research/passes.py` — 5-pass execution (broad, academic, practitioner, contrarian, primary)
- [ ] 1.4 `src/research/convergence.py` — novelty tracking, coverage map, convergence detection
- [ ] 1.5 `src/research/quality.py` — 3-tier quality scoring (fast gate, standard, deep)
- [ ] 1.6 `src/research/queries.py` — query construction (5 variants per sub-topic)
- [ ] 1.7 `src/research/sources.py` — source fetching, archival, deduplication
- [ ] 1.8 Tests for each module
- [ ] 1.9 Tests for convergence detection (natural + budget exhausted)
- [ ] 1.10 Tests for quality scoring (all 3 tiers)
- [ ] 1.11 Coverage ≥ 100%

---

## Phase 2: Counter-Research + Verification

**Purpose:** Adversarial verification and source verification pipeline.

- [ ] 2.1 `src/research/counter.py` — negation search, alternative explanations, replication check
- [ ] 2.2 `src/research/verify.py` — URL check, claim-source match, recency check
- [ ] 2.3 Robustness ratings (robust, moderate, fragile, contested)
- [ ] 2.4 Integration with convergence: unresolved contradictions block convergence
- [ ] 2.5 Tests for counter-research
- [ ] 2.6 Tests for source verification
- [ ] 2.7 Coverage ≥ 100%

---

## Phase 3: MCP Tools + Slash Commands

**Purpose:** Expose research capabilities as MCP tools and slash commands.

- [ ] 3.1 MCP tool: `research_topic(topic, search_terms, max_searches)` — run full 5-pass research
- [ ] 3.2 MCP tool: `research_status(session_id)` — check convergence status
- [ ] 3.3 MCP tool: `verify_sources(findings_json)` — run source verification pipeline
- [ ] 3.4 MCP tool: `counter_research(claim)` — run adversarial verification on a claim
- [ ] 3.5 Slash command: `/research "topic"` — research a topic to convergence
- [ ] 3.6 Slash command: `/verify "claim"` — verify a specific claim
- [ ] 3.7 Tests for all MCP tools
- [ ] 3.8 Coverage ≥ 100%

---

## Phase 4: Wire into Existing Systems

**Purpose:** Make competitive analysis, website content, and evolution pipeline use the research engine.

- [ ] 4.1 Competitors system: `discover_competitors()` uses 5-pass research
- [ ] 4.2 Competitors system: `research_competitor()` uses quality scoring + counter-research
- [ ] 4.3 Competitors system: verification standards use source verification pipeline
- [ ] 4.4 Website content: research-first architecture for all content pages
- [ ] 4.5 Evolution pipeline: Gather beat uses research passes 1-3
- [ ] 4.6 Evolution pipeline: Evaluate beat uses quality scoring
- [ ] 4.7 Build plans: research phase uses 5-pass system when research is needed
- [ ] 4.8 `/competitor-add` uses research engine instead of ad-hoc search
- [ ] 4.9 Tests for integration points
- [ ] 4.10 Coverage ≥ 100%

---

## Phase 5: Research Telemetry

**Purpose:** Track research quality, detect improvement signals, enable self-improvement.

- [ ] 5.1 `src/research/telemetry.py` — per-search, per-fetch, per-session metrics
- [ ] 5.2 Novelty trajectory tracking (should decline over time)
- [ ] 5.3 Source diversity monitoring
- [ ] 5.4 Cost tracking (tokens per quality point)
- [ ] 5.5 Improvement signal detection (premature convergence, budget exhaustion, etc.)
- [ ] 5.6 Tests for telemetry
- [ ] 5.7 Coverage ≥ 100%

---

## Phase 6: Research Storage

**Purpose:** Permanent research archive with regenerable output.

- [ ] 6.1 `src/research/archive.py` — store research as markdown + sources JSON
- [ ] 6.2 Archive structure: `research_archives/{slug}/research.md`, `sources.json`, `meta.json`
- [ ] 6.3 Deduplication: don't re-research topics with fresh archives
- [ ] 6.4 Staleness detection: flag archives older than configurable threshold
- [ ] 6.5 Generate-from-research: load archive + apply template
- [ ] 6.6 Tests for archive operations
- [ ] 6.7 Coverage ≥ 100%

---

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: audit all docs against code, two clean passes
- [ ] Website convergence: update metrics, audit content accuracy, two clean passes
- [ ] Deployment: deploy per docs/DEPLOYMENT.md
- [ ] Patterns update: capture learnings if novel
- [ ] Inbox check: process messages from other sessions

## Convergence Criteria

- All checklist items complete (including post-execution items above)
- All tests pass
- Coverage ≥ 100%
- Two consecutive clean audit passes
- Documentation verified against code
- Website metrics current
- Competitive analysis system uses research engine
- All research activities use 5-pass system
- Counter-research mandatory for all claims in competitor comparisons

---

## Test Commands

```bash
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

**Total: 47 checkboxes**
