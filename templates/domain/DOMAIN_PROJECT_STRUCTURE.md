---
title: "Domain Project Structure: [Domain Name]"
last_updated: YYYY-MM-DD
last_audit_plan: PLAN-XXXX
audit_status: current
---

# Domain Project Structure Template

> Canonical structure for how projects are organized within a Key domain.
> Every domain follows this dual-root pattern: `/srv/git/<domain>/` for code,
> `/srv/sync/<domain>/` for operational docs, plans, and artifacts.

---

## 1. The Dual-Root Pattern

Every Key domain has TWO root directories:

```
/srv/git/<domain>/           # Git-managed (code repos)
/srv/sync/<domain>/          # Syncthing-managed (docs, plans, artifacts, config)
```

**Rule:** Code goes in git. Everything else goes in sync. Never mix them.

**Why two roots:**
- Git repos need `.git/`, branches, PRs, CI — Syncthing would corrupt `.git/`
- Docs, plans, and artifacts need to be visible across all machines via Syncthing
- Bots working on intents access sync points for context, git repos for code

---

## 2. Git Root Structure (`/srv/git/<domain>/`)

Each project within a domain gets its own git repo:

```
/srv/git/<domain>/
  <project-name>/                    # One git repo per project
    README.md                        # Project overview (Key template)
    CHANGELOG.md                     # Version history
    CONTRIBUTING.md                  # How to contribute
    SECURITY.md                      # Security policy
    LICENSE                          # License text
    GAPS.md                          # Documentation gaps tracker

    docs/                            # Code-coupled documentation
      ARCHITECTURE.md                # System design, components, data flow
      API.md                         # API reference (promotes to api/ folder)
      CONFIGURATION.md               # All config options
      DEPLOYMENT.md                  # Deploy guide
      DEVELOPMENT.md                 # Dev setup, build, workflow
      SCHEMA.md                      # Database schema
      TESTING.md                     # Test strategy and execution
      TROUBLESHOOTING.md             # Common problems and solutions
      OPERATIONS.md                  # Operational runbook
      MONITORING.md                  # Metrics, alerts, dashboards
      PERFORMANCE.md                 # Benchmarks, profiling
      INTEGRATION.md                 # Third-party integrations
      MIGRATION.md                   # Version upgrade guide
      adr/                           # Architecture Decision Records
        INDEX.md
        ADR-001_<decision>.md

    build_plans/                     # Build plans linked to Onelist
      PLAN-XXXX_<description>.md     # One file per plan (YAML frontmatter + content)

    src/                             # Source code (language-specific structure)
    tests/                           # Test suite
    config/                          # Configuration files
    ...                              # Other project-specific directories

  <another-project>/                 # Additional projects in this domain
    ...
```

### Key Rules for Git Root

1. **One repo per project** — don't cram multiple projects into one repo
2. **docs/ follows Key template standard** — use templates from `templates/projects/code/docs/`
3. **build_plans/ links to Onelist** — every plan file has a PLAN-XXXX ID matching Onelist
4. **GAPS.md at root** — tracks what documentation is missing or incomplete
5. **No operational docs in git** — operational/runbook docs go to sync point
6. **Roadmaps go to sync** — future plans are NOT in git docs/

---

## 3. Sync Root Structure (`/srv/sync/<domain>/`)

```
/srv/sync/<domain>/
  .stignore                          # Syncthing exclusion rules (auto-deployed)

  docs/                              # Operational documentation (domain-level)
    CHARTER.md                       # Domain purpose, scope, ownership
    STRATEGY.md                      # Domain strategic plan
    INVENTORY.md                     # All repos, services, tools in this domain
    HEALTH_REPORT.md                 # Domain health dashboard
    DEPENDENCY_MAP.md                # Inter-domain dependencies
    INTEGRATION.md                   # Cross-domain integration docs
    <PROJECT>.md                     # Per-project operational overview
    <PROJECT>_RUNBOOK.md             # Per-project ops runbook

  plans/                             # Plans and roadmaps
    ROADMAP.md                       # Domain roadmap (auto-generated or manual)
    <project>/                       # Project-specific plans
      PLAN-XXXX_<description>.md     # Plans in flight
    backlog/                         # Future work
      features/
      bugs/
    imports/                         # Import plans (e.g., obsidian, evernote)

  artifacts/                         # Build outputs, reports, generated content
    <project>/                       # Per-project artifacts
      test-reports/
      audit-reports/
      build-outputs/

  config/                            # Shared configuration (non-secret)
    domain-keywords.yaml             # Domain keyword registry for intake routing
    <project>-config.yaml            # Per-project shared config

  intake/                            # Dropbox for new materials
    _intaked/                        # Successfully processed
    _failed/                         # Processing errors
    _conflicts/                      # Content conflicts needing resolution
    _processing/                     # Currently being processed
    _misrouted/                      # Wrong domain, awaiting reroute
    _crossover/                      # Incoming from other domains
    _partial/                        # Originals where only parts were extracted
    _new-domains/                    # Content suggesting new domains needed
```

### Key Rules for Sync Root

1. **Operational docs here** — anything a human or bot needs to SEE about the project (not build)
2. **Per-project subdirs in plans/ and artifacts/** — organize by project within the domain
3. **CHARTER.md is mandatory** — every domain must have a charter defining scope
4. **INVENTORY.md tracks everything** — repos, services, tools, bots assigned to this domain
5. **intake/ is the dropbox** — drop files from any machine, they get auto-processed
6. **No code here** — code goes in git, period. No exceptions.
7. **No secrets here** — .stignore blocks .env, credentials, keys

---

## 4. Multi-Project Domains

A domain often has multiple projects. Example: the **arch** domain:

```
/srv/git/arch/
  swarm-sentinel/              # Standalone Rust binary
    docs/                      # Sentinel code docs
    build_plans/               # Sentinel build plans
    GAPS.md
  onelist-local/               # Phoenix/Elixir web app
    docs/                      # Onelist code docs
    build_plans/
    GAPS.md
  arch-runtime/                # Future: Elixir/OTP bot runtime
    docs/
    build_plans/
    GAPS.md

/srv/sync/arch/
  docs/
    CHARTER.md                 # Arch domain charter
    STRATEGY.md                # Arch strategy
    INVENTORY.md               # Lists all 3 repos + services
    SENTINEL.md                # Sentinel operational overview
    SENTINEL_RUNBOOK.md        # Sentinel ops runbook
    ONELIST.md                 # Onelist operational overview
    ONELIST_RUNBOOK.md         # Onelist ops runbook
    ARCHITECTURE-v4.md         # Cross-project architecture
    DUAL_TRACK_DEVELOPMENT.md  # Cross-project development guide
  plans/
    sentinel/                  # Sentinel plans
    onelist/                   # Onelist plans
    runtime/                   # Arch runtime plans
  artifacts/
    sentinel/                  # Sentinel artifacts
    onelist/                   # Onelist artifacts
```

### Rules for Multi-Project Domains

1. **Each project = own git repo** under `/srv/git/<domain>/`
2. **Each project = own subdirs** under `plans/` and `artifacts/` in sync
3. **Domain-level docs** (CHARTER, STRATEGY, cross-project architecture) live at `docs/` root
4. **Per-project operational docs** (`<PROJECT>.md`, `<PROJECT>_RUNBOOK.md`) live at `docs/` root
5. **A project can be standalone** — sentinel runs independently but is organizationally part of arch
6. **Cross-project docs** (architecture spanning projects) live at sync `docs/` root

---

## 5. Single-Project Domains

Some domains have only one project. Example: **keyvibe**:

```
/srv/git/keyvibe/
  keyvibe/                     # The one repo
    docs/
    build_plans/
    GAPS.md

/srv/sync/keyvibe/
  docs/
    CHARTER.md
    STRATEGY.md
    INVENTORY.md
    KEYVIBE.md                 # No separate project prefix needed
    KEYVIBE_RUNBOOK.md
  plans/
  artifacts/
  intake/
```

Simpler — no per-project subdirs needed in plans/ and artifacts/.

---

## 6. What Goes Where (Decision Guide)

| Content Type | Location | Why |
|---|---|---|
| Source code | `/srv/git/<domain>/<project>/src/` | Needs git history, branches, PRs |
| Unit tests | `/srv/git/<domain>/<project>/tests/` | Runs with code |
| Architecture doc | `/srv/git/<domain>/<project>/docs/ARCHITECTURE.md` | Changes with code |
| API reference | `/srv/git/<domain>/<project>/docs/API.md` | Changes with code |
| Schema doc | `/srv/git/<domain>/<project>/docs/SCHEMA.md` | Changes with code |
| Build plan | `/srv/git/<domain>/<project>/build_plans/PLAN-XXXX.md` | Links commits to plans |
| Ops runbook | `/srv/sync/<domain>/docs/<PROJECT>_RUNBOOK.md` | Operational, not code-coupled |
| Domain charter | `/srv/sync/<domain>/docs/CHARTER.md` | Domain-level, not project-level |
| Roadmap | `/srv/sync/<domain>/plans/ROADMAP.md` | Future plans, not code |
| Test reports | `/srv/sync/<domain>/artifacts/<project>/test-reports/` | Generated output |
| Configuration | `/srv/git/<domain>/<project>/config/` | Code-coupled config |
| Shared config | `/srv/sync/<domain>/config/` | Cross-project or operational config |
| New materials to process | `/srv/sync/<domain>/intake/` | Dropbox for intake pipeline |

---

## 7. Domain Initialization Checklist

When creating a new domain or adding a project to an existing domain:

### New Domain
- [ ] `domain-sync-provision <domain>` — creates sync structure + .stignore
- [ ] `mkdir -p /srv/git/<domain>` — create git root
- [ ] Create `docs/CHARTER.md` from `templates/domains/docs/CHARTER.md`
- [ ] Create `docs/STRATEGY.md` from `templates/domains/docs/STRATEGY.md`
- [ ] Create `docs/INVENTORY.md` from `templates/domains/docs/INVENTORY.md`
- [ ] Register domain in Onelist
- [ ] Add domain to `DOMAIN_MAP.md`
- [ ] Create `intake/` subfolders if not auto-created

### New Project in Existing Domain
- [ ] `git clone <repo> /srv/git/<domain>/<project>/` — or `git init`
- [ ] Create `docs/` folder from `templates/projects/code/FOLDER_STRUCTURE.md`
- [ ] Create `build_plans/` directory
- [ ] Create `GAPS.md` at project root
- [ ] Create `<PROJECT>.md` in `/srv/sync/<domain>/docs/`
- [ ] Create `<PROJECT>_RUNBOOK.md` in `/srv/sync/<domain>/docs/`
- [ ] Create project subdirs in `plans/` and `artifacts/`
- [ ] Update `docs/INVENTORY.md` with new project
- [ ] Run `doc-audit /srv/git/<domain>/<project>/` — identify gaps

### Migration of Existing Project
- [ ] Follow `MIGRATE_TO_KEY.md` process
- [ ] Move/clone repo to `/srv/git/<domain>/<project>/`
- [ ] Normalize existing docs to Key templates (zero content loss)
- [ ] Create operational docs in `/srv/sync/<domain>/docs/`
- [ ] Create `GAPS.md` with known deficiencies
- [ ] Run convergence audit
- [ ] Symlink old location if needed for backward compatibility

---

## 8. Naming Conventions

| Item | Convention | Example |
|---|---|---|
| Domain name | lowercase, single word or hyphenated | `arch`, `keyvibe`, `crux` |
| Project/repo name | lowercase, hyphenated | `swarm-sentinel`, `onelist-local` |
| Sync docs (domain-level) | UPPER_CASE.md | `CHARTER.md`, `STRATEGY.md` |
| Sync docs (project) | PROJECT_NAME.md | `SENTINEL.md`, `SENTINEL_RUNBOOK.md` |
| Git docs | UPPER_CASE.md | `ARCHITECTURE.md`, `API.md` |
| Build plans | PLAN-XXXX_description.md | `PLAN-2749_memory-leak-detection.md` |
| ADRs | ADR-NNN_description.md | `ADR-001_use-postgresql.md` |
| Config files | lowercase, hyphenated | `domain-keywords.yaml` |
| Plan subdirs | lowercase project name | `plans/sentinel/`, `plans/onelist/` |
| Artifact subdirs | lowercase project name | `artifacts/sentinel/` |

---

## 9. Template References

All templates referenced in this structure:

| Template | Path (relative to swarm_sync root) |
|---|---|
| Domain charter | `templates/domains/docs/CHARTER.md` |
| Domain strategy | `templates/domains/docs/STRATEGY.md` |
| Domain inventory | `templates/domains/docs/INVENTORY.md` |
| Domain health report | `templates/domains/docs/HEALTH_REPORT.md` |
| Domain dependency map | `templates/domains/docs/DEPENDENCY_MAP.md` |
| Code project folder structure | `templates/projects/code/FOLDER_STRUCTURE.md` |
| Code project doc standard | `templates/projects/code/CODE_PROJECT_DOCUMENTATION_STANDARD.md` |
| All code doc templates (18) | `templates/projects/code/docs/*.md` |
| Build plan template | `templates/projects/code/build_plans/BUILD_PLAN_TEMPLATE.md` |
| Build plan standard | `templates/projects/code/build_plans/BUILD_PLAN_STANDARD.md` |
| Operations runbook | `templates/operations/docs/SERVICE_RUNBOOK.md` |
| Intake standard | `templates/intake/INTAKE_STANDARD.md` |
| Cross-domain routing | `templates/intake/CROSS_DOMAIN_ROUTING.md` |
| GAPS.md guide | See `documentation/MIGRATE_TO_KEY.md` Section 5 |
