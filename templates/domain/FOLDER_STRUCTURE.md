# Domain Folder Structure

> Canonical layout for a domain's sync point at `/srv/sync/<domain>/`.

## Standard Structure

```
/srv/sync/<domain>/
  docs/                              # Domain documentation
    CHARTER.md                       # Purpose, scope, ownership, boundaries
    STRATEGY.md                      # Goals, approach, timeline, success criteria
    INVENTORY.md                     # Repos, services, tools, bots, data stores
    HEALTH_REPORT.md                 # Current status, metrics, risks, blockers
    DEPENDENCY_MAP.md                # What this domain needs and provides
    INTEGRATION.md                   # How other domains connect to this one
    HANDOFF.md                       # Ownership transfer documentation
    INIT_CHECKLIST.md                # Domain initialization tracking
  plans/                             # Active and queued plans for this domain
  artifacts/                         # Outputs produced by work in this domain
  config/                            # Domain-specific configuration
    domain.yaml                      # Machine-readable domain metadata
    bot_policy.yaml                  # Bot behavior rules for this domain
    thresholds.yaml                  # Autonomy thresholds and escalation rules
```

## File-to-Folder Promotion Rules

When a single file grows beyond manageable size, promote it to a folder:

| Original File | Promotes To | When |
|---|---|---|
| `INVENTORY.md` | `inventory/` folder with per-category files | >20 items across categories |
| `DEPENDENCY_MAP.md` | `dependencies/` folder per direction | >10 dependencies in either direction |
| `INTEGRATION.md` | `integrations/` folder per consuming domain | >5 integration points |
| `HEALTH_REPORT.md` | `health/` folder with dated reports | Domain under active remediation |

After promotion, the original file becomes an index:

```markdown
# Inventory

This domain's inventory has been organized into per-category files:

- [Repositories](inventory/repos.md)
- [Services](inventory/services.md)
- [Tools](inventory/tools.md)
- [Data Stores](inventory/data.md)
```

## Minimal Domain (Lightweight Init)

For new domains that are still being scoped, the minimum viable documentation is:

```
/srv/sync/<domain>/
  docs/
    CHARTER.md             # At minimum: purpose and owner
    INIT_CHECKLIST.md      # Track what still needs setup
  plans/
  artifacts/
  config/
```

## Config Directory Detail

The `config/` directory holds machine-readable files that bots and tools consume:

```
config/
  domain.yaml              # Domain metadata (name, owner, type, status, keywords)
  bot_policy.yaml          # What bots can/cannot do in this domain autonomously
  thresholds.yaml          # Autonomy levels, escalation triggers, rate limits
  alerts.yaml              # Alert routing (who gets notified, for what conditions)
```

## Plans Directory Convention

Plans placed in a domain's `plans/` directory follow the standard plan schema. They can be:

- **Symlinks** to the canonical plan in the main plans/ tree
- **Copies** when offline/disconnected operation is needed
- **Stubs** that reference the canonical location

Preferred approach: symlinks to maintain single source of truth.

```
plans/
  PLAN-1234_feature-x.md -> /home/key/repos/claude_code_swarm/plans/3_in_progress/PLAN-1234_feature-x.md
  PLAN-5678_bugfix-y.md  -> /home/key/repos/claude_code_swarm/plans/3_in_progress/PLAN-5678_bugfix-y.md
```

## Artifacts Directory Convention

Artifacts are domain-specific outputs. Organize by type:

```
artifacts/
  reports/                 # Generated reports, audits, analyses
  exports/                 # Data exports, API responses, snapshots
  builds/                  # Build outputs, packages, binaries
  logs/                    # Structured logs from domain-specific processes
```

## Domain Archive

When a domain is retired:

1. Move the entire sync point to `/srv/sync/_archived/<domain>_YYYY-MM-DD/`
2. Ensure HANDOFF.md is complete with all disposition decisions
3. Ensure HEALTH_REPORT.md has final status
4. Remove any symlinks in plans/ (they will break after move)

## Template Version

- **Version:** 1.0
- **Created:** 2026-03-24
- **Last Updated:** 2026-03-24
