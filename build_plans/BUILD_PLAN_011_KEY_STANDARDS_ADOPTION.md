# BUILD_PLAN_011: Adopt Key Standards into CruxDev

**Created:** 2026-03-24
**Status:** NOT STARTED
**Goal:** Integrate Key's template system, gap analysis, and migration methodology into CruxDev's adoption process. Every CruxDev-managed project gets: project classification, document inventory, template mapping, gap detection (GAPS.md), and convergence audit — all driven by the CruxDev convergence engine.

**Derived from:** Key management system migration methodology (MIGRATE_TO_KEY.md).
**Rule:** TDD. Tests before code. 100% coverage.
**Rule:** Every adoption step audited to convergence (two consecutive clean passes per step).
**Rule:** Zero content loss during normalization — verified by inventory cross-check.

---

## Document Alignment

### Product Docs:
- docs/ADOPTION_PROCESS.md — current adoption process (will be superseded)
- docs/ADOPTION_PLAYBOOK.md — current 9-phase playbook (will be extended)
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — convergence methodology
- docs/RESEARCH_PATTERNS.md — research methodology (used during gap analysis)

---

## What Changes

### Current Adoption Process
```
1. Install Crux
2. Install CruxDev
3. Create CLAUDE.md
4. Configure coverage
5. Website planning
6. Run adoption playbook (9 phases)
```

### New Adoption Process (Key-Enhanced)
```
1. Classify project (type taxonomy + maturity level)
2. Inventory everything (documents, code, assets, decisions)
3. Map to templates (code, business, product, website, research — per project type)
4. Organize and normalize (folder structure, markdown conversion, frontmatter)
5. Create GAPS.md (critical, high, medium, low gaps with template references)
6. Install Crux + CruxDev
7. Audit to convergence (each step verified — two clean passes)
8. Run adoption playbook (code hardening, test build-out, doc convergence)
9. Post-adoption: continuous gap monitoring
```

Each step is driven by the convergence engine. The engine audits the output of each step before advancing to the next.

---

## Phase 1: Project Classification Engine

**Purpose:** Automated project classification — determine type(s), maturity, and required templates.

- [ ] 1.1 `src/adoption/classify.py` — project type taxonomy (10 types: software-existing, software-greenfield, business-existing, business-new, product-saas, website, infrastructure, consulting-client, research, campaign)
- [ ] 1.2 Composite type detection (most projects are multi-type)
- [ ] 1.3 Maturity assessment (idea → minimal → growing → production → mature)
- [ ] 1.4 Template category resolution from project types (required, recommended, optional)
- [ ] 1.5 Generate `intake-classification.md` from analysis
- [ ] 1.6 MCP tool: `classify_project(project_dir)` — auto-detect type + maturity
- [ ] 1.7 Tests for classification logic
- [ ] 1.8 Tests for composite type resolution
- [ ] 1.9 Coverage ≥ 100%

---

## Phase 2: Document Inventory Engine

**Purpose:** Automated inventory of all existing project materials.

- [ ] 2.1 `src/adoption/inventory.py` — scan for all documents, code, assets
- [ ] 2.2 Format detection (markdown, docx, pdf, spreadsheet, code, config)
- [ ] 2.3 Quality classification per item (usable, reference-only, extract-info, obsolete)
- [ ] 2.4 Last-modified tracking
- [ ] 2.5 Generate `intake-inventory.md` from scan
- [ ] 2.6 MCP tool: `inventory_project(project_dir)` — produce full inventory
- [ ] 2.7 Tests for inventory scanning
- [ ] 2.8 Tests for format detection
- [ ] 2.9 Coverage ≥ 100%

---

## Phase 3: Template System

**Purpose:** Maintain the full Key template library within CruxDev.

- [ ] 3.1 `src/adoption/templates.py` — template registry with all document types
- [ ] 3.2 Template categories: domains, projects/code, projects/business, projects/website, product, financial, legal, operations, people, research, campaigns, customer, content, governance, communications
- [ ] 3.3 Document requirement levels per maturity (R=required, P=production, M=mature, O=optional)
- [ ] 3.4 Folder structure generation per project type
- [ ] 3.5 Template-to-project mapping logic
- [ ] 3.6 MCP tool: `get_templates(project_type, maturity)` — return required templates
- [ ] 3.7 MCP tool: `generate_folder_structure(project_type)` — create folder layout
- [ ] 3.8 Tests for template resolution
- [ ] 3.9 Tests for folder generation
- [ ] 3.10 Coverage ≥ 100%

---

## Phase 4: Gap Analysis Engine (GAPS.md)

**Purpose:** Automated gap detection — compare project state against template requirements.

- [ ] 4.1 `src/adoption/gaps.py` — compare inventory against template requirements
- [ ] 4.2 Gap classification: critical, high, medium, low
- [ ] 4.3 Stub document detection (documents with TODO markers)
- [ ] 4.4 Known deficiency detection (content exists but is weak/outdated)
- [ ] 4.5 Not-applicable tracking (templates that don't apply, with justification)
- [ ] 4.6 Generate `GAPS.md` from analysis
- [ ] 4.7 MCP tool: `analyze_gaps(project_dir)` — produce gap analysis
- [ ] 4.8 MCP tool: `gap_status(project_dir)` — show current gap counts by priority
- [ ] 4.9 Tests for gap detection
- [ ] 4.10 Tests for gap classification
- [ ] 4.11 Coverage ≥ 100%

---

## Phase 4B: Document Normalization Engine

**Purpose:** Convert and normalize documents to Key template standards.

- [ ] 4B.1 `src/adoption/normalize.py` — format conversion + frontmatter injection
- [ ] 4B.2 Markdown conversion helpers (PDF→md, DOCX→md, HTML→md via pandoc subprocess)
- [ ] 4B.3 YAML frontmatter injection (title, last_updated, source, migration_date, migration_status)
- [ ] 4B.4 Content splitting for documents that span multiple templates
- [ ] 4B.5 Stub paragraph generator (TODO markers with template reference, priority, estimated effort)
- [ ] 4B.6 Original content archival to `artifacts/intake-originals/`
- [ ] 4B.7 MCP tool: `normalize_document(source_path, template_type)` — convert + normalize
- [ ] 4B.8 Tests for conversion and normalization
- [ ] 4B.9 Coverage ≥ 100%

---

## Phase 5: Convergence-Audited Adoption Steps

**Purpose:** Each adoption step is verified by the convergence engine before advancing.

- [ ] 5.1 New convergence phase: `ADOPTION_CLASSIFY` — verify classification is complete + accurate
- [ ] 5.2 New convergence phase: `ADOPTION_INVENTORY` — verify inventory accounts for all materials
- [ ] 5.3 New convergence phase: `ADOPTION_NORMALIZE` — verify normalization preserved all content
- [ ] 5.4 New convergence phase: `ADOPTION_GAPS` — verify GAPS.md is comprehensive
- [ ] 5.5 Zero content loss verification gate (inventory cross-check after normalization)
- [ ] 5.6 Integration with existing adoption playbook (phases 2-9 run after classification/inventory)
- [ ] 5.7 Tests for each adoption convergence phase
- [ ] 5.8 Coverage ≥ 100%

---

## Phase 6: Slash Commands + MCP Integration

**Purpose:** Expose the new adoption process as commands.

- [ ] 6.1 Update `/adopt` command to use Key-enhanced process
- [ ] 6.2 New slash command: `/classify` — classify a project
- [ ] 6.3 New slash command: `/inventory` — inventory project materials
- [ ] 6.4 New slash command: `/gaps` — show current gaps
- [ ] 6.5 Update `docs/ADOPTION_PROCESS.md` with new 9-step process
- [ ] 6.6 Update `docs/SESSION_UPGRADE.md` with new adoption capabilities
- [ ] 6.7 Tests for slash command behavior
- [ ] 6.8 Coverage ≥ 100%

---

## Phase 7: Continuous Gap Monitoring

**Purpose:** After adoption, continuously monitor for new gaps.

- [ ] 7.1 Integration with evolution pipeline: Beat 2 (Evaluate) checks for new gaps
- [ ] 7.2 Integration with doc drift detection: new code → new potential gaps
- [ ] 7.3 GAPS.md auto-update when gaps are detected
- [ ] 7.4 Session bus notification when critical gaps appear
- [ ] 7.5 Tests for continuous monitoring
- [ ] 7.6 Coverage ≥ 100%

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
- ADOPTION_PROCESS.md fully rewritten for Key-enhanced process
- Template library covers all 15 categories
- Gap analysis produces actionable GAPS.md for any project type
- Every adoption step audited by convergence engine

---

## Test Commands

```bash
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

**Total: 64 checkboxes**
