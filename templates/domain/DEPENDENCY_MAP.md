# Domain Dependency Map: [Domain Name]

> **Domain:** [domain-name]
> **Owner:** [Name]
> **Last Updated:** YYYY-MM-DD
> **Last Validated:** YYYY-MM-DD

---

## 1. Dependency Overview

```
                    UPSTREAM                          DOWNSTREAM
                 (we depend on)                    (depends on us)

    [domain-a] -----> +------------------+ -----> [domain-x]
    [domain-b] -----> |  [DOMAIN NAME]   | -----> [domain-y]
    [domain-c] -----> +------------------+ -----> [domain-z]
```

**Total upstream dependencies:** [N]
**Total downstream dependents:** [N]
**Circular dependencies:** [N -- should be 0]

---

## 2. Upstream Dependencies (We Depend On)

### Critical Dependencies

| Source Domain | What We Need | Interface Type | Failure Impact | Fallback |
|---|---|---|---|---|
| [domain] | [Data/Service/API/Config] | [Sync point / API / Direct] | [What breaks] | [Degraded mode or none] |

### Standard Dependencies

| Source Domain | What We Need | Interface Type | Failure Impact | Fallback |
|---|---|---|---|---|
| [domain] | [Data/Service/API/Config] | [Sync point / API / Direct] | [What breaks] | [Degraded mode or none] |

### External Dependencies

| Source | What We Need | Interface Type | SLA | Fallback |
|---|---|---|---|---|
| [external service] | [What] | [API / Library / File] | [Availability %] | [Fallback] |

---

## 3. Downstream Dependents (Depend On Us)

### Critical Dependents

| Consuming Domain | What They Need | Interface Type | Our SLA | Breakage Impact |
|---|---|---|---|---|
| [domain] | [Data/Service/API/Config] | [Sync point / API / Direct] | [Commitment] | [What breaks for them] |

### Standard Dependents

| Consuming Domain | What They Need | Interface Type | Our SLA | Breakage Impact |
|---|---|---|---|---|
| [domain] | [Data/Service/API/Config] | [Sync point / API / Direct] | [Commitment] | [What breaks for them] |

---

## 4. Shared Resources

<!-- Resources used by multiple domains, where contention is possible. -->

| Resource | Shared With | Type | Contention Risk | Governance |
|---|---|---|---|---|
| [resource] | [domains] | [Compute/Storage/Lock/Data] | Low/Med/High | [How contention is resolved] |

---

## 5. Data Flows

### Inbound Data

| Source | Data Description | Format | Frequency | Volume | Validation |
|---|---|---|---|---|---|
| [domain/service] | [What data] | [YAML/JSON/SQL/Files] | [Real-time/Batch/On-demand] | [Approx volume] | [How validated] |

### Outbound Data

| Destination | Data Description | Format | Frequency | Volume | Contract |
|---|---|---|---|---|---|
| [domain/service] | [What data] | [YAML/JSON/SQL/Files] | [Real-time/Batch/On-demand] | [Approx volume] | [Schema/spec ref] |

---

## 6. Dependency Health Matrix

| Dependency | Direction | Health | Last Tested | Risk Level |
|---|---|---|---|---|
| [domain/service] | UP/DOWN | GREEN/YELLOW/RED | YYYY-MM-DD | Low/Med/High |

---

## 7. Circular Dependency Analysis

<!-- Circular dependencies are architectural debt. Document any that exist and the plan to break them. -->

| Cycle | Domains Involved | Severity | Breaking Plan |
|---|---|---|---|
| [Cycle description] | [A -> B -> C -> A] | Low/Med/High | [How to break the cycle] |

If no circular dependencies exist, state: **No circular dependencies detected.**

---

## 8. Dependency Risk Assessment

### Single Points of Failure

| Dependency | Why Critical | Redundancy | Remediation Priority |
|---|---|---|---|
| [dependency] | [Why losing it is catastrophic] | [None / Partial / Full] | [P1/P2/P3] |

### Fragile Dependencies

| Dependency | Why Fragile | Failure Frequency | Hardening Plan |
|---|---|---|---|
| [dependency] | [Why it breaks often] | [How often] | [PLAN-XXXX or description] |

---

## 9. Dependency Change Protocol

When a dependency changes (upstream or downstream):

1. **Notification:** The changing domain notifies all affected domains via [mechanism]
2. **Impact Assessment:** Each affected domain evaluates impact within [timeframe]
3. **Coordination:** Breaking changes require [approval process]
4. **Testing:** Integration tests must pass before change is accepted
5. **Rollback:** [Rollback procedure if dependency change causes issues]

---

## 10. Validation Schedule

| Check | Frequency | Method | Last Run | Result |
|---|---|---|---|---|
| Upstream availability | [Daily/Weekly] | [How checked] | YYYY-MM-DD | PASS/FAIL |
| Downstream contract compliance | [Weekly/Monthly] | [How checked] | YYYY-MM-DD | PASS/FAIL |
| Data flow integrity | [Daily/Weekly] | [How checked] | YYYY-MM-DD | PASS/FAIL |
| Circular dependency scan | [Monthly] | [How checked] | YYYY-MM-DD | PASS/FAIL |
