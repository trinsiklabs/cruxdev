# Domain Initialization Checklist: [Domain Name]

> **Domain:** [domain-name]
> **Initiated By:** [Name]
> **Date Started:** YYYY-MM-DD
> **Date Completed:** YYYY-MM-DD (fill when done)
> **Type:** Product | Service | Coordination | Infrastructure

---

## Phase 1: Approval and Setup

- [ ] **Domain creation approved by splntrb**
  - Date: YYYY-MM-DD
  - Context: [Why this domain is being created]

- [ ] **Sync point created**
  ```bash
  mkdir -p /srv/sync/[domain-name]/{docs,plans,artifacts,config}
  ```

- [ ] **Domain registered in swarm registry**
  - Registry location: [path to domain config]
  - Keywords assigned: [keyword1, keyword2, ...]
  - Type set: [Product / Service / Coordination / Infrastructure]

---

## Phase 2: Core Documentation

- [ ] **CHARTER.md created**
  - Template: `/home/key/swarm_sync/templates/domains/docs/CHARTER.md`
  - Minimum: purpose, scope (in/out), and owner defined
  - Approved by: [Name, Date]

- [ ] **INVENTORY.md created**
  - Template: `/home/key/swarm_sync/templates/domains/docs/INVENTORY.md`
  - Minimum: repos and services listed (even if empty at first)

- [ ] **DEPENDENCY_MAP.md created**
  - Template: `/home/key/swarm_sync/templates/domains/docs/DEPENDENCY_MAP.md`
  - Minimum: known upstream and downstream dependencies listed

---

## Phase 3: Configuration

- [ ] **domain.yaml created**
  ```yaml
  # /srv/sync/[domain-name]/config/domain.yaml
  name: [domain-name]
  type: [product|service|coordination|infrastructure]
  owner: [owner]
  status: active
  created: YYYY-MM-DD
  keywords:
    - [keyword1]
    - [keyword2]
  repos: []
  services: []
  ```

- [ ] **bot_policy.yaml created**
  ```yaml
  # /srv/sync/[domain-name]/config/bot_policy.yaml
  domain: [domain-name]
  default_autonomy: supervised
  policies:
    - bot: "*"
      permissions:
        read: [docs, plans, artifacts, config]
        write: [artifacts]
        create_plans: false
      escalation:
        trigger: [any destructive action]
        target: [owner]
  ```

- [ ] **thresholds.yaml created**
  ```yaml
  # /srv/sync/[domain-name]/config/thresholds.yaml
  domain: [domain-name]
  autonomy_levels:
    full: []           # Bots that can act without approval
    supervised: ["*"]  # Bots that need approval for writes
    readonly: []       # Bots that can only read
  escalation:
    default_target: [owner]
    timeout_minutes: 30
  rate_limits:
    max_plans_per_day: 10
    max_artifacts_per_hour: 50
  ```

---

## Phase 4: Integration

- [ ] **Upstream dependencies documented**
  - Each dependency has: source, interface type, failure handling

- [ ] **Downstream consumers notified**
  - Each consumer knows: what's available, how to access, what the contract is

- [ ] **INTEGRATION.md created** (if applicable)
  - Template: `/home/key/swarm_sync/templates/domains/docs/INTEGRATION.md`
  - Skip if domain has no cross-domain integration points yet

---

## Phase 5: Operational Readiness

- [ ] **First HEALTH_REPORT.md generated**
  - Template: `/home/key/swarm_sync/templates/domains/docs/HEALTH_REPORT.md`
  - Baseline health established

- [ ] **Health review cadence set**
  - Frequency: [Weekly / Bi-weekly / Monthly]
  - Reviewer: [Who]
  - Reminder mechanism: [Timer / Calendar / Manual]

- [ ] **STRATEGY.md created** (if applicable)
  - Template: `/home/key/swarm_sync/templates/domains/docs/STRATEGY.md`
  - Skip if domain is purely infrastructure with no strategic goals

---

## Phase 6: Verification

- [ ] **Sync point structure verified**
  ```bash
  # All four directories exist
  test -d /srv/sync/[domain-name]/docs && echo "OK: docs"
  test -d /srv/sync/[domain-name]/plans && echo "OK: plans"
  test -d /srv/sync/[domain-name]/artifacts && echo "OK: artifacts"
  test -d /srv/sync/[domain-name]/config && echo "OK: config"
  ```

- [ ] **Config files are parseable**
  ```bash
  yq '.' /srv/sync/[domain-name]/config/domain.yaml > /dev/null && echo "OK: domain.yaml"
  yq '.' /srv/sync/[domain-name]/config/bot_policy.yaml > /dev/null && echo "OK: bot_policy.yaml"
  yq '.' /srv/sync/[domain-name]/config/thresholds.yaml > /dev/null && echo "OK: thresholds.yaml"
  ```

- [ ] **Charter is complete and approved**
  - Purpose defined: [ ]
  - Scope (in/out) defined: [ ]
  - Owner assigned: [ ]
  - Approval recorded: [ ]

- [ ] **Domain is discoverable**
  - Appears in domain registry listing
  - Keywords route correctly
  - Bots can find and connect to sync point

---

## Post-Init: First 30 Days

- [ ] **Week 1:** First plan created and assigned to this domain
- [ ] **Week 2:** First artifact produced by bot working in this domain
- [ ] **Week 2:** Any integration issues discovered and documented
- [ ] **Week 4:** First health report reviewed
- [ ] **Week 4:** Charter reviewed and updated based on first month's experience
- [ ] **Week 4:** Init checklist marked complete

---

## Notes

<!-- Any observations, deviations from template, or context for future reference. -->

[Notes here]
