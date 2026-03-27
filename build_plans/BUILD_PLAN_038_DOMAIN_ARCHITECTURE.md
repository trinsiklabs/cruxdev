# BUILD_PLAN_038: Domain Architecture — Parent Projects with Typed Sub-Projects

**Status:** NOT STARTED
**Priority:** Strategic (foundational architecture shift)
**Depends on:** BP034 (universal project management), BP037 (templates)

## Context

CruxDev currently manages individual projects. The real world has **domains** — a business, ecosystem, or initiative that contains multiple projects of different types, all contributing to a larger purpose.

Example: **CruxVibe** is a domain (vibecoding platform business). It contains:
- CruxVibe app (web app — closed source SaaS product)
- CruxVibe website (marketing site)
- CruxVibe desktop/mobile apps (future)
- Crux (open source — intelligence layer, also its own domain)
- CruxDev (open source — convergence engine, also its own domain)
- CruxCLI (open source — runtime, also its own domain)
- Hosting/provisioning infrastructure
- Vibe coding translation layer
- Marketing podcast (future)
- Newsletter (future)
- Ebook (future)

Key insight: **A project can belong to multiple domains.** CruxDev is both its own independent domain AND a sub-project within CruxVibe's domain.

## Architecture

### Domain
A domain is defined by a `domain.toml` (or `CHARTER.md`) at the domain root:

```toml
[domain]
name = "CruxVibe"
description = "The vibecoding platform — AI-powered development for everyone"
owner = "Trinsik Labs"

[[domain.projects]]
name = "cruxvibe-app"
type = "product-saas"
path = "../cruxvibe"
role = "primary"  # This is what the domain exists to deliver

[[domain.projects]]
name = "cruxvibe-website"
type = "website"
path = "../cruxvibe-site"
depends_on = ["cruxvibe-app"]

[[domain.projects]]
name = "crux"
type = "open-source"
path = "../crux"
role = "infrastructure"
also_domain = true  # Has its own domain identity

[[domain.projects]]
name = "cruxdev"
type = "open-source"
path = "../cruxdev"
role = "infrastructure"
also_domain = true

[[domain.projects]]
name = "cruxcli"
type = "open-source"
path = "../cruxcli"
role = "infrastructure"
also_domain = true

[[domain.projects]]
name = "cruxvibe-podcast"
type = "podcast"
path = "../cruxvibe-podcast"
depends_on = ["cruxvibe-app"]

[[domain.projects]]
name = "cruxvibe-newsletter"
type = "newsletter"
path = "../cruxvibe-newsletter"
depends_on = ["cruxvibe-podcast"]
```

### Domain Templates (from swarm_sync)
Each domain gets these documents:
- **CHARTER.md** — purpose, scope, ownership, boundaries
- **STRATEGY.md** — goals, approach, timeline, success criteria
- **INVENTORY.md** — repos, services, tools, data stores
- **HEALTH_REPORT.md** — status, metrics, risks, blockers
- **DEPENDENCY_MAP.md** — what this domain needs and provides
- **INTEGRATION.md** — how sub-projects connect to each other
- **INIT_CHECKLIST.md** — domain initialization tracking

### Multi-Domain Membership
A project's `.cruxdev/growth.toml` can declare domain memberships:
```toml
[project]
name = "CruxDev"
domains = ["cruxdev", "cruxvibe"]  # Member of both
```

### Cross-Project Convergence
When a sub-project converges, the domain checks:
1. Does this change affect other sub-projects? (dependency map)
2. Is the domain CHARTER still accurate?
3. Does the domain HEALTH_REPORT need updating?
4. Does the domain website need updating?
5. Session bus notification to all affected sub-projects

## Phase 1: Domain Configuration

- [ ] 1.1 Define `domain.toml` schema
- [ ] 1.2 New module: `rust/src/domain/mod.rs`
- [ ] 1.3 `load_domain(domain_dir)` — parse domain.toml, resolve sub-project paths
- [ ] 1.4 `validate_domain()` — verify all sub-projects exist, deps are valid

## Phase 2: Domain Templates

- [ ] 2.1 Copy domain templates from swarm_sync: CHARTER, STRATEGY, INVENTORY, HEALTH_REPORT, DEPENDENCY_MAP, INTEGRATION, INIT_CHECKLIST
- [ ] 2.2 `init_domain(name, projects)` — create domain.toml + template docs
- [ ] 2.3 Domain classifier: detect if a directory is a domain (has domain.toml or CHARTER.md)

## Phase 3: Cross-Project Operations

- [ ] 3.1 `domain_status()` — aggregate health across all sub-projects
- [ ] 3.2 `domain_dependency_check()` — verify cross-project dependencies
- [ ] 3.3 When sub-project converges → check domain impact → notify affected sub-projects via session bus
- [ ] 3.4 Domain-level convergence: all sub-projects converged + cross-project consistency verified

## Phase 4: MCP Tools

- [ ] 4.1 `init_domain(name, description)` — create new domain with templates
- [ ] 4.2 `domain_status(domain_dir)` — aggregate health report
- [ ] 4.3 `add_project_to_domain(domain_dir, project_name, project_type, project_path)` — register sub-project
- [ ] 4.4 `domain_dependency_map(domain_dir)` — visualize dependencies

## Phase 5: Tests

- [ ] 5.1 Domain config loading and validation
- [ ] 5.2 Multi-domain membership
- [ ] 5.3 Cross-project dependency detection
- [ ] 5.4 Domain initialization from templates
