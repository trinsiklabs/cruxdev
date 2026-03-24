# Research Patterns — Universal Research Methodology

**Version:** 1.0
**Created:** 2026-03-24
**Scope:** Universal — applies to ANY research activity in any CruxDev-managed project. Competitive analysis, feature research, technology evaluation, user research, academic research, market research.
**Derived from:** Production-proven V3 research methodology, validated against Anthropic's deep research system architecture.

This document parallels `DEVELOPMENT_PATTERNS_CRUXDEV.md` (for code), `WEBSITE_PLANNING.md` (for websites), and `COMPETITORS_PATTERN.md` (for competitive analysis). It defines HOW to research anything to convergence.

---

## Table of Contents

1. [Core Principle](#1-core-principle)
2. [The 5-Pass Iterative Deepening System](#2-the-5-pass-system)
3. [Convergence Detection](#3-convergence-detection)
4. [Quality Scoring — Three Tiers](#4-quality-scoring)
5. [Counter-Research Protocol](#5-counter-research-protocol)
6. [Source Verification Pipeline](#6-source-verification)
7. [Query Construction Protocol](#7-query-construction)
8. [Research Session Lifecycle](#8-session-lifecycle)
9. [Orchestrator-Worker Architecture](#9-orchestrator-worker)
10. [Model Routing for Research](#10-model-routing)
11. [Research Telemetry](#11-telemetry)
12. [Self-Improvement Protocol](#12-self-improvement)
13. [Failure Modes and Mitigations](#13-failure-modes)
14. [Integration with CruxDev Systems](#14-integration)

---

## 1. Core Principle

**Research is permanent. Content is derived.**

Separate research (expensive, stored permanently) from content generation (cheap, regenerable). The workflow:

1. Research a topic → store findings + archive sources
2. Generate output (page, report, plan) → load research + apply template
3. Regenerate → same research, new template, no new API calls

Never redo research just because an output format changes.

---

## 2. The 5-Pass Iterative Deepening System

Every research topic goes through 5 sequential passes, each with a different search strategy. This is not optional — skipping passes produces biased, shallow research.

### Pass 1 — Broad Search
- Uses base search terms directly
- 5-8 searches per sub-topic, surface-level extraction
- Goal: establish the landscape, find major sources
- Max 8 new URLs per pass

### Pass 2 — Academic/Authoritative Search
- Appends "research study 2024 2025 2026" to each term
- Focuses on peer-reviewed literature, official documentation, authoritative sources
- Goal: find evidence with specifics (numbers, benchmarks, case studies)

### Pass 3 — Practitioner/User Search
- Appends "developer experience", "user review", "case study" (or domain-appropriate terms)
- Goal: find people actually using/doing the thing in practice
- For competitive analysis: find real users comparing tools

### Pass 4 — Contrarian/Adversarial Search
- Appends "criticism limitations problems" to top search terms
- Goal: find counter-evidence, limitations, opposing viewpoints
- **NOT OPTIONAL** — research without adversarial verification has confirmation bias

### Pass 5 — Primary Sources
- Search official documentation, GitHub repos, API docs, specifications
- For academic topics: query PubMed, arXiv, Google Scholar directly
- Goal: find primary sources that web search misses

### Descending Parallel Schedule

Start with 3 parallel searches, taper to 1 as understanding deepens. This is the "explore-then-exploit" paradigm (74% accuracy, 35.9% cost reduction, 40.6% time reduction vs sequential — from W&D paper).

---

## 3. Convergence Detection

Research converges when additional searching produces diminishing returns. This is detected algorithmically, not by intuition.

### Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_searches` | 50 | Hard ceiling on total searches |
| `min_sources` | 3 | Minimum unique sources per sub-question |
| `novelty_threshold` | 10% | Minimum % new information per search to continue |
| `confidence_floor` | 0.7 | Minimum confidence to accept findings |

### Convergence Check (runs after every search batch)

```
1. COVERAGE: All sub-questions have >= min_sources unique sources?
   If no → NOT CONVERGED (target under-covered areas)

2. NOVELTY: Average of last 5 novelty scores < novelty_threshold?
   If no → NOT CONVERGED (keep searching — still finding new info)

3. CONTRADICTIONS: Unresolved contradictions with confidence > confidence_floor?
   If yes → NOT CONVERGED (resolve first)

4. CONFIDENCE: All sub-question coverage scores > confidence_floor?
   If no → NOT CONVERGED

5. HARD LIMIT: total_searches >= max_searches?
   → CONVERGED with budget_exhausted flag

CONVERGED when 1-4 all pass, or 5 triggers.
```

### Novelty Calculation

```
new_facts = facts with Jaccard similarity < 0.3 to all existing findings
novelty_score = len(new_facts) / len(all_facts_in_result)
```

A declining novelty trajectory (0.8 → 0.6 → 0.4 → 0.2 → 0.1) indicates healthy convergence. A flat trajectory indicates the search terms need variation.

---

## 4. Quality Scoring — Three Tiers

### Tier 1 — Fast Gate (No LLM, Deterministic)

Applied to ALL fetched content BEFORE any LLM processing:

| Check | Threshold |
|-------|-----------|
| Content length | > 200 chars |
| Real sentences | Segments > 20 chars ending with `.` or `?` |
| Tag-to-text ratio | < 50% (not mostly HTML/boilerplate) |
| Information density | unique_words / total_words > 0.3 |

**Score < 0.3 → REJECT.** Skip, don't waste LLM tokens.

### Tier 2 — Standard Score (6 Dimensions)

| Dimension | Check |
|-----------|-------|
| Source authority | Peer-reviewed, institutional, or established? |
| Source recency | Published within 2 years for fast-moving fields? |
| Claim specificity | Named citations (Author, Year) present? |
| Relevance | Answers at least one research question? |
| Counter-evidence | Addresses limitations or opposing views? |
| Internal consistency | No self-contradictions within the source? |

**Score = count of Yes / 6. Threshold: >= 0.5 to proceed.**

### Tier 3 — Deep Score (Full Rubric, High-Quality Findings Only)

Applied only to findings scoring > 0.7 on standard score:

| Category | Max Points | What's scored |
|----------|-----------|--------------|
| Source quality | 25 | Diversity, authority, recency, independence, named citations |
| Coverage | 25 | Sub-question coverage, depth, counter-evidence |
| Synthesis | 25 | Cross-source integration, contradiction resolution, novel connections |
| Reliability | 25 | Citation verifiability, claim-source alignment, uncertainty acknowledged |

### Evidence-Anchored Scoring

To solve the LLM-as-judge reliability problem:
1. **Rubric locking** — fixed taxonomies and binary checklists, not free-form scales
2. **Evidence-anchored** — require extractive text quotes grounding every score. No quote = no credit.
3. **Distributional calibration** — post-hoc mapping to align model scores with human standards

---

## 5. Counter-Research Protocol

### What Gets Counter-Researched

- Causal claims ("X causes Y")
- Statistical claims (effect sizes, percentages, benchmarks)
- Novel claims (not widely established)
- Claims that would propagate widely if wrong
- Claims with high confidence scores (most likely accepted uncritically)

**Skip:** established facts, definitional statements, direct quotes.

### Counter-Research Techniques

1. **Negation search** — "X is wrong", "problems with X", "criticism of X"
2. **Alternative explanation search** — for causal claims, search alternatives
3. **Replication search** — has the cited study/benchmark been replicated?
4. **Expert dissent search** — find named experts who disagree
5. **Base rate check** — verify statistical base rates
6. **Recency check** — has the claim been updated, retracted, or superseded?

### Robustness Ratings

| Rating | Criteria |
|--------|----------|
| **Robust** | Multiple independent sources, counter-evidence addressed, replication exists |
| **Moderate** | Good evidence but limited counter-evidence search |
| **Fragile** | Single-source, no counter-evidence, possibly cherry-picked |
| **Contested** | Active disagreement with credible opposing evidence |

---

## 6. Source Verification Pipeline

For each citation in research output:

1. **URL_CHECK** — fetch URL, try Wayback Machine for dead links
2. **CLAIM_MATCH** — extract claim attributed to source, compare against actual source text
3. **AUTHOR_VERIFY** — check for academic citations (Google Scholar, ORCID, DOI)
4. **RECENCY_CHECK** — flag if > 5 years old for fast-moving fields
5. **CREDIBILITY_SCORE** — authority (0.1-0.9), recency, specificity, corroboration

Every link in published research must return HTTP 200. Dead links are replaced with Wayback Machine URLs or removed.

---

## 7. Query Construction Protocol

For each sub-topic, generate 3-5 variant queries:

1. **Direct keyword** — the literal topic
2. **Semantic/conceptual** — related concepts, synonyms
3. **Named-entity** — specific companies, people, projects, papers
4. **Negation** — "limitations of X", "problems with Y"
5. **Temporal** — recent developments, "2026", "latest"

### Source Diversification Requirements

- At least 2 authoritative sources (official docs, peer-reviewed, institutional)
- At least 1 primary source (original data, official announcement)
- At least 1 contrarian/opposing viewpoint
- At least 1 practitioner/user source
- Maximum 30% from any single domain

### Query Decomposition

Complex research questions must be decomposed before searching:

1. **Simple splitting** — break compound questions into independent sub-questions
2. **Decomposition-Reflection** — decompose, answer each, self-reflect after each stage
3. **Tree-of-Reasoning** — build a reasoning tree for complex logical dependencies

Default to simple splitting. Use Tree-of-Reasoning for questions with complex interdependencies.

---

## 8. Research Session Lifecycle

### Checkpointing

Checkpoint after every meaningful state change:
- After each search batch → save findings to disk
- After synthesis of each sub-question → save
- After quality scoring → save
- After counter-research → save

### Recovery

On session start, check for existing checkpoint. If found, load state and resume rather than restarting.

### Idempotent Operations

- Maintain `seen_urls` set per session
- Deduplicate before insertion
- Don't re-fetch already-archived content

### Non-Blocking Error Architecture

Every post-synthesis phase (archival, URL following, citation verification) is independently failable. The session succeeds if findings were stored, even if secondary operations had partial failures.

---

## 9. Orchestrator-Worker Architecture

For complex research requiring multiple sub-questions:

```
Lead Agent (Opus-class)
  - Analyzes query, develops strategy
  - Decomposes into sub-questions
  - Spawns workers in parallel
  - Monitors convergence
  - Synthesizes findings
       │
  ┌────┼────┐
  ▼    ▼    ▼
Worker Workers
  - Execute search strategy
  - Fetch and filter sources
  - Report findings back
```

### Scaling Rules

| Complexity | Agents | Searches/Agent | Total |
|-----------|--------|----------------|-------|
| Simple factual | 1 | 3-10 | 3-10 |
| Moderate | 2-3 | 5-15 each | 10-45 |
| Complex multi-topic | 5-10 | 10-20 each | 50-200 |

**Critical rule:** Separate orchestration from execution. When an orchestrator also executes, implementation details pollute decision-making context. Separation reduces token usage by 60%.

---

## 10. Model Routing for Research

| Phase | Model Tier | Why |
|-------|-----------|-----|
| Query decomposition | Fast (micro/fast) | Structured task |
| Search execution | Fast | Tool calling, not synthesis |
| Content quality gate | Deterministic (no LLM) | Regex, keyword matching |
| Triage scoring | Fast | Classification task |
| Deep synthesis | Frontier | Cross-source integration |
| Counter-research | Standard | Reasoning but lower stakes |
| Quality scoring | Deterministic + standard | Structured extraction |

This maps directly to CruxDev's model tier system (micro → fast → standard → frontier).

---

## 11. Research Telemetry

### What to Measure

| Category | Metrics |
|----------|---------|
| Per-search | Query, results count, novelty score, duration |
| Per-fetch | URL, quality score, content size, accepted/rejected |
| Per-session | Convergence trajectory, quality score, source diversity |
| Cost | Tokens consumed per quality point achieved |
| Failure modes | Type, severity, frequency |

### Improvement Signals

| Signal | Severity | Trigger |
|--------|----------|---------|
| Premature convergence | High | Converged with quality < 60 |
| Budget too low | High | > 30% sessions exhaust budget |
| Source monoculture | Medium | < 3 unique domains in session |
| Counter-evidence deficit | Medium | > 50% sessions have zero counter-evidence |
| High rejection rate | Low | > 50% fetched content rejected |
| Cost inefficiency | Low | > 2x average tokens per quality point |

---

## 12. Self-Improvement Protocol

### The Inception Loop

1. **Use** the methodology to conduct research
2. **Audit** the methodology against findings
3. **Update** with corrections, additions
4. **Assess** convergence — would another cycle find significant improvements?

### What Can Be Self-Improved

| Component | Method | Validation |
|-----------|--------|------------|
| Query expansion templates | A/B test variants | Higher novelty = better |
| Convergence thresholds | Compare natural vs budget exhaustion | Target 70%+ natural |
| Quality gate thresholds | False positive/negative rates | Compare to human review |
| Model routing rules | Track per-route accuracy and cost | Lower cost at same quality |

### What Requires Human Judgment

- Research question formulation (what to research)
- Domain-specific correctness (is the finding true?)
- Ethical judgment (should we research this?)
- Contradiction resolution when sources are equally credible

---

## 13. Failure Modes and Mitigations

| Failure | Cause | Mitigation |
|---------|-------|------------|
| Compound failure | 95% per-step × 20 steps = 36% | Minimize steps, parallelize, checkpoint |
| Hallucination | LLM fabricates sources | Evidence-anchored scoring, citation verification |
| Echo chamber | Source preference bias | Enforce source diversity (min 3 domains) |
| Confident wrong answers | Multi-agent reinforcement | Require uncertainty acknowledgment |
| Silent quality degradation | No monitoring | Continuous quality scoring, trend alerts |
| Confirmation bias | Only finding supporting evidence | Mandatory adversarial Pass 4 |

---

## 14. Integration with CruxDev Systems

### Competitive Analysis (`COMPETITORS_PATTERN.md`)

The competitors system MUST use this research methodology:
- Phase 2 (Deep Research) follows the 5-pass system
- Verification standards follow the source verification pipeline
- Counter-evidence is mandatory (Pass 4 applied to competitor claims)
- Quality scoring applied to every competitor finding
- Convergence detection determines when competitor research is "done"

### Website Content (`WEBSITE_PLANNING.md`)

Content for project websites follows research-first architecture:
- Research the topic → store permanently
- Generate page from research → cheap, regenerable
- Never write content without research backing
- All claims on comparison pages verified via source verification pipeline

### Build Plans

When a build plan requires research (feature design, technology evaluation):
- Use 5-pass system for the research phase
- Store research as a permanent artifact in `research_archives/`
- Reference research in the plan's Document Alignment section
- Counter-research any claims that would drive architecture decisions

### Evolution Pipeline

The evolution pipeline's Gather beat uses this methodology:
- Upstream repo scanning follows Pass 1-2 patterns
- Community input follows Pass 3 patterns
- Adversarial verification follows Pass 4 patterns

### Session Bus

Research findings can be shared across sessions:
- `share_pattern()` for research methodology improvements
- `report_improvement()` for research infrastructure gaps
- Cross-project research deduplication via the bus

---

## Quick Reference: When to Use Which Pass

| Research Goal | Required Passes | Why |
|--------------|----------------|-----|
| Competitive analysis | All 5 | Need broad landscape + specific evidence + user perspective + counter-claims + primary sources |
| Feature evaluation | 1, 2, 4 | Need options + evidence + limitations |
| Technology decision | 1, 2, 3, 4 | Need landscape + benchmarks + practitioner experience + problems |
| Bug investigation | 1, 5 | Need context + primary sources (docs, issues) |
| Market research | 1, 2, 3 | Need landscape + data + user perspective |
| Academic research | 1, 2, 4, 5 | Need landscape + literature + counter-evidence + primary papers |
| Content writing | All 5 | Need comprehensive, balanced, well-sourced content |
