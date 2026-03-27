# Compliance Mapping Template

<!--
  CLASSIFICATION: COMPLIANCE
  Use this template to map regulatory/framework obligations to swarm implementations.
  Supports GDPR, SOC 2, ISO 27001, or any other compliance framework.
  Review quarterly or immediately on regulatory change.
-->

---

## Frontmatter

| Field | Value |
|-------|-------|
| **Mapping ID** | COMPLIANCE-NNN |
| **Framework** | _[GDPR / SOC 2 / ISO 27001 / PCI-DSS / Custom]_ |
| **Framework Version** | _[Version or date of framework reference]_ |
| **Created** | YYYY-MM-DD |
| **Updated** | YYYY-MM-DD |
| **Owner** | _[Primary compliance officer — Tron, Cipher, etc.]_ |
| **Secondary Owner** | _[Backup compliance officer]_ |
| **Evidence Collector** | _[Officer responsible for gathering audit evidence]_ |
| **Approved By** | _[Trio]_ |
| **Next Review** | YYYY-MM-DD |
| **Source Plan** | PLAN-XXXX _(if applicable)_ |

---

## Framework Summary

_Brief description of the compliance framework, its applicability to the swarm, and the scope of this mapping. 2-3 sentences._

---

## Scope of Applicability

**In scope:**
- _[Systems, data types, processes, officers subject to this framework]_

**Out of scope:**
- _[What is explicitly excluded and why]_

---

## Control Mapping

### Category: _[Framework category name, e.g., "Data Protection"]_

| Control ID | Control Description | Implementation | Owner | Evidence | Status | Gap |
|-----------|-------------------|----------------|-------|----------|--------|-----|
| _[XX.1]_ | _[What the control requires]_ | _[How the swarm implements it]_ | _[Officer]_ | _[What proves compliance]_ | _[compliant / partial / non-compliant / N/A]_ | _[Gap description if not fully compliant]_ |
| _[XX.2]_ | _[Control description]_ | _[Implementation]_ | _[Officer]_ | _[Evidence]_ | _[Status]_ | _[Gap]_ |

### Category: _[Next category]_

| Control ID | Control Description | Implementation | Owner | Evidence | Status | Gap |
|-----------|-------------------|----------------|-------|----------|--------|-----|
| _[YY.1]_ | _[Control description]_ | _[Implementation]_ | _[Officer]_ | _[Evidence]_ | _[Status]_ | _[Gap]_ |

---

## Gap Analysis Summary

| Gap ID | Control | Current State | Required State | Remediation Plan | Owner | Target Date | Priority |
|--------|---------|--------------|----------------|-----------------|-------|-------------|----------|
| _[GAP-01]_ | _[Control ID]_ | _[What exists today]_ | _[What is required]_ | _[Plan reference]_ | _[Officer]_ | _[Date]_ | _[critical / high / medium / low]_ |

---

## Evidence Collection Schedule

| Evidence Type | Source | Collection Method | Frequency | Retention Period | Collector |
|--------------|--------|-------------------|-----------|-----------------|-----------|
| _[Access logs]_ | _[System]_ | _[Automated export]_ | _[Monthly]_ | _[1 year]_ | _[Officer]_ |
| _[Change records]_ | _[Git history]_ | _[Automated script]_ | _[On change]_ | _[Permanent]_ | _[Officer]_ |

---

## Audit History

| Audit Date | Auditor | Scope | Findings | Corrective Actions | Status |
|-----------|---------|-------|----------|-------------------|--------|
| _[Date]_ | _[Internal/External auditor]_ | _[What was audited]_ | _[Summary]_ | _[Actions taken]_ | _[open / closed]_ |

---

## Regulatory Change Tracking

| Date | Change Description | Impact Assessment | Action Required | Status |
|------|-------------------|-------------------|-----------------|--------|
| _[Date]_ | _[What changed in the regulation]_ | _[How it affects the swarm]_ | _[What we need to do]_ | _[pending / complete]_ |

---

## Related Documents

- `governance/COMPLIANCE_FRAMEWORK.md` — master compliance framework
- _[Other related compliance mappings]_
- _[Related policies]_
