---
title: "ADR-[NNN]: [Decision Title]"
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
status: [proposed | accepted | deprecated | superseded]
superseded_by: [ADR-NNN, if applicable]
---

# ADR-[NNN]: [Decision Title]

**Date:** [YYYY-MM-DD]
**Status:** [Proposed | Accepted | Deprecated | Superseded by [ADR-NNN](ADR-NNN_title.md)]
**Decision Makers:** [Names or roles of people who made this decision]

## Context

[Describe the situation that prompted this decision. What problem are we facing? What forces are at play? What constraints exist?

Be specific and factual. Include:
- What is the current state?
- What is not working or needs to change?
- What requirements or constraints must be satisfied?
- What quality attributes are most important (performance, maintainability, security, etc.)?
]

## Decision

[State the decision clearly and concisely. Use active voice.

Example: "We will use PostgreSQL as the primary data store for all transactional data."

If the decision has multiple parts, list them:
1. [First part of the decision]
2. [Second part of the decision]
3. [Third part of the decision]
]

## Alternatives Considered

### [Alternative A: Name]

- **Description:** [What this alternative entails]
- **Pros:** [Advantages of this alternative]
- **Cons:** [Disadvantages of this alternative]
- **Why rejected:** [Specific reason this was not chosen]

### [Alternative B: Name]

- **Description:** [What this alternative entails]
- **Pros:** [Advantages of this alternative]
- **Cons:** [Disadvantages of this alternative]
- **Why rejected:** [Specific reason this was not chosen]

### [Alternative C: Do Nothing / Status Quo]

- **Description:** [Keep things as they are]
- **Pros:** [No migration cost, no risk]
- **Cons:** [The problems described in Context remain]
- **Why rejected:** [Why inaction is not acceptable]

## Consequences

### Positive

- [Benefit that follows from this decision]
- [Another benefit]

### Negative

- [Cost or trade-off that follows from this decision]
- [Another cost or trade-off]

### Neutral

- [Consequence that is neither clearly positive nor negative]
- [Teams or components affected by this decision]

## Implementation Notes

[Optional. Any notes about how this decision should be implemented.]

- [Implementation detail or constraint]
- [Migration steps if changing from a previous approach]
- [Timeline or phasing if the decision is implemented gradually]

## Review Triggers

[Under what circumstances should this decision be revisited?]

- [Trigger condition, e.g., "If data volume exceeds 10TB"]
- [Trigger condition, e.g., "If the vendor changes pricing"]
- [Trigger condition, e.g., "At the next architecture review in Q4"]

---

<!--
## ADR INDEX.md Template

Use this template for the adr/INDEX.md file that lists all ADRs:

# Architecture Decision Records

| ADR | Title | Status | Date |
|---|---|---|---|
| [ADR-001](ADR-001_decision-name.md) | [Decision Title] | Accepted | YYYY-MM-DD |
| [ADR-002](ADR-002_decision-name.md) | [Decision Title] | Accepted | YYYY-MM-DD |
| [ADR-003](ADR-003_decision-name.md) | [Decision Title] | Superseded by ADR-005 | YYYY-MM-DD |
| [ADR-004](ADR-004_decision-name.md) | [Decision Title] | Proposed | YYYY-MM-DD |
| [ADR-005](ADR-005_decision-name.md) | [Decision Title] | Accepted | YYYY-MM-DD |

## Status Legend
- **Proposed**: Under discussion, not yet decided
- **Accepted**: Decision made and in effect
- **Deprecated**: Decision no longer relevant (system removed, etc.)
- **Superseded**: Replaced by a newer ADR (linked)
-->
