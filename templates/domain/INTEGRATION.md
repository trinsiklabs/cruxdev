# Inter-Domain Integration: [Domain Name]

> **Domain:** [domain-name]
> **Owner:** [Name]
> **Created:** YYYY-MM-DD
> **Last Updated:** YYYY-MM-DD
> **Integration Points:** [N total]

---

## 1. Integration Overview

<!-- High-level summary of how this domain connects to the rest of the swarm. -->

[Domain Name] integrates with [N] other domains. The primary integration pattern is [sync-point based / event-driven / API / direct file access]. This document serves as the authoritative reference for any domain seeking to integrate with [domain-name].

---

## 2. Integration Points

### Provided Interfaces (Others Consume From Us)

#### Interface: [Interface Name]

- **Type:** Sync Point | API | File | Event | Tool
- **Location:** [Path, URL, or sync point reference]
- **Format:** [YAML / JSON / Markdown / Binary]
- **Schema:** [Reference to schema file or inline description]
- **Access:** [Read-only / Read-write / Append-only]
- **Authentication:** [None / Token / Certificate]
- **Rate Limits:** [If applicable]

**Contract:**
```yaml
# What consumers can expect
availability: [always / best-effort / scheduled]
update_frequency: [real-time / hourly / daily / on-demand]
backward_compatibility: [guaranteed / best-effort / none]
breaking_change_notice: [N days/weeks]
```

**Current Consumers:**

| Domain | What They Use | Since | Contact |
|---|---|---|---|
| [domain] | [What specifically] | YYYY-MM-DD | [Who to talk to] |

---

#### Interface: [Interface Name 2]

<!-- Repeat the block above for each provided interface. -->

---

### Consumed Interfaces (We Consume From Others)

#### From: [Source Domain] -- [Interface Name]

- **Type:** Sync Point | API | File | Event | Tool
- **Location:** [Path, URL, or sync point reference]
- **What We Use:** [Specific data/functionality we consume]
- **How We Use It:** [Read pattern -- polling, watch, event-driven]
- **Failure Handling:** [What happens when this interface is unavailable]

---

#### From: [Source Domain 2] -- [Interface Name]

<!-- Repeat for each consumed interface. -->

---

## 3. Sync Point Integration Patterns

### Reading from Our Sync Point

Other domains read from `/srv/sync/[domain-name]/` using these patterns:

```
# Pattern 1: Direct file read
cat /srv/sync/[domain-name]/docs/CHARTER.md

# Pattern 2: Config consumption
yq '.key' /srv/sync/[domain-name]/config/domain.yaml

# Pattern 3: Plan discovery
ls /srv/sync/[domain-name]/plans/

# Pattern 4: Artifact retrieval
cp /srv/sync/[domain-name]/artifacts/reports/latest.md ./
```

### Writing to Our Sync Point

| Who Can Write | Where | What | Governance |
|---|---|---|---|
| Domain owner | Anywhere | Any content | Full authority |
| Assigned bots | plans/, artifacts/ | Plan updates, work outputs | Per bot_policy.yaml |
| Other domains | artifacts/incoming/ | Cross-domain deliverables | Must follow integration contract |

---

## 4. Event-Based Integration

### Events We Emit

| Event | Trigger | Payload | Consumers |
|---|---|---|---|
| [event-name] | [What causes it] | [What data is included] | [Who listens] |

### Events We Consume

| Event | Source | Our Handler | Failure Mode |
|---|---|---|---|
| [event-name] | [Source domain] | [What we do with it] | [What if we miss it] |

---

## 5. Cross-Domain Workflows

### Workflow: [Workflow Name]

```
[Domain A] -- (trigger) --> [This Domain] -- (output) --> [Domain B]
     ^                                                         |
     |                                                         |
     +---------------------- (feedback) -----------------------+
```

**Steps:**
1. [Domain A] produces [trigger] at [location]
2. [This Domain] detects trigger via [mechanism]
3. [This Domain] processes and produces [output]
4. [Domain B] consumes output via [mechanism]

**Error Handling:**
- If step 2 fails: [What happens]
- If step 3 fails: [What happens]
- Retry policy: [How retries work]

---

## 6. Integration Testing

### Test Matrix

| Integration | Test Type | Test Location | Frequency | Last Passed |
|---|---|---|---|---|
| [Domain A] -> Us | [Smoke/Contract/E2E] | [Test file path] | [On change/Daily/Weekly] | YYYY-MM-DD |
| Us -> [Domain B] | [Smoke/Contract/E2E] | [Test file path] | [On change/Daily/Weekly] | YYYY-MM-DD |

### Contract Tests

<!-- Tests that verify integration contracts are maintained. -->

```bash
# Example: Verify our sync point has expected structure
test -f /srv/sync/[domain-name]/config/domain.yaml || echo "FAIL: domain.yaml missing"

# Example: Verify consumed interface is available
test -f /srv/sync/[source-domain]/artifacts/[expected-file] || echo "FAIL: dependency missing"
```

---

## 7. Integration Versioning

| Interface | Current Version | Supported Versions | Deprecation Schedule |
|---|---|---|---|
| [interface] | [v1.2] | [v1.0+] | [v1.0 deprecated YYYY-MM-DD] |

### Breaking Change History

| Date | Interface | Change | Migration Guide | Affected Domains |
|---|---|---|---|---|
| YYYY-MM-DD | [interface] | [What changed] | [How to adapt] | [Who was affected] |

---

## 8. Troubleshooting

### Common Integration Issues

| Symptom | Likely Cause | Resolution |
|---|---|---|
| [What you observe] | [Why it happens] | [How to fix] |

### Diagnostic Commands

```bash
# Check sync point health
ls -la /srv/sync/[domain-name]/

# Verify config is parseable
yq '.' /srv/sync/[domain-name]/config/domain.yaml

# Check for stale data
find /srv/sync/[domain-name]/ -name "*.md" -mtime +30

# Verify permissions
stat /srv/sync/[domain-name]/
```

---

## 9. Integration Contact Matrix

| Domain | Primary Contact | Escalation | Preferred Channel |
|---|---|---|---|
| [domain] | [Who] | [Who if primary unavailable] | [How to reach] |
