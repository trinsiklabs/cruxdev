---
title: Architecture Overview
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Architecture Overview

> [One-line summary of the system's architectural purpose and style.]

## System Context

### What This System Does

[Describe the system's purpose at the highest level. What business capability does it provide? Who uses it?]

### System Boundary

[Describe what is inside the system boundary and what is outside. What does this system own vs. delegate to external services?]

```
[ASCII diagram or description of the system in its environment]

┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│   Client     │────▶│   This System    │────▶│  Database   │
│   (Browser)  │◀────│                  │◀────│  (Postgres) │
└─────────────┘     │  ┌────────────┐  │     └─────────────┘
                    │  │ Component A │  │
                    │  │ Component B │  │     ┌─────────────┐
                    │  │ Component C │  │────▶│ External API│
                    │  └────────────┘  │     └─────────────┘
                    └──────────────────┘
```

## Components

### [Component A Name]

- **Purpose:** [What this component does]
- **Technology:** [Language, framework, key libraries]
- **Owns:** [What data/behavior this component is responsible for]
- **Communicates with:** [Other components it talks to and how]
- **Location:** `[path/to/component/]`

### [Component B Name]

- **Purpose:** [What this component does]
- **Technology:** [Language, framework, key libraries]
- **Owns:** [What data/behavior this component is responsible for]
- **Communicates with:** [Other components it talks to and how]
- **Location:** `[path/to/component/]`

### [Component C Name]

- **Purpose:** [What this component does]
- **Technology:** [Language, framework, key libraries]
- **Owns:** [What data/behavior this component is responsible for]
- **Communicates with:** [Other components it talks to and how]
- **Location:** `[path/to/component/]`

## Data Flow

### Primary Data Path

[Describe the main flow of data through the system, from input to output.]

```
1. [Client sends request to ...]
2. [Component A receives and ...]
3. [Component A calls Component B to ...]
4. [Component B queries database for ...]
5. [Response flows back through ...]
```

### Secondary Data Paths

[Describe any secondary flows: background jobs, event processing, cache warming, etc.]

## Data Storage

### Primary Data Store

- **Type:** [PostgreSQL / Redis / S3 / etc.]
- **Purpose:** [What data lives here]
- **Schema:** See [SCHEMA.md](SCHEMA.md)

### Caches

- **Type:** [Redis / Memcached / in-memory / etc.]
- **Purpose:** [What is cached and why]
- **Invalidation:** [How cache invalidation works]

### File Storage

- **Type:** [Local filesystem / S3 / etc.]
- **Purpose:** [What files are stored]

## Key Design Decisions

[Summary of the most important architectural decisions. Each should link to its full ADR.]

| Decision | Summary | ADR |
|---|---|---|
| [Decision name] | [One-line summary of what was decided] | [ADR-001](adr/ADR-001_decision-name.md) |
| [Decision name] | [One-line summary of what was decided] | [ADR-002](adr/ADR-002_decision-name.md) |
| [Decision name] | [One-line summary of what was decided] | [ADR-003](adr/ADR-003_decision-name.md) |

## Communication Patterns

### Synchronous

| From | To | Protocol | Purpose |
|---|---|---|---|
| [Component A] | [Component B] | [HTTP/gRPC/function call] | [Purpose] |
| [Component A] | [External API] | [HTTPS] | [Purpose] |

### Asynchronous

| Producer | Consumer | Mechanism | Purpose |
|---|---|---|---|
| [Component A] | [Component B] | [Queue/Event bus/Pub-sub] | [Purpose] |

## Security Architecture

[High-level security design. For full details, see [SECURITY.md](../SECURITY.md).]

- **Authentication:** [How users/services authenticate]
- **Authorization:** [How permissions are enforced]
- **Data protection:** [Encryption at rest/in transit]
- **Network boundaries:** [What is exposed, what is internal-only]

## Scalability

### Current Scale

- [Number of users / requests per second / data volume]

### Scaling Strategy

- **Horizontal:** [What can be scaled horizontally and how]
- **Vertical:** [What requires vertical scaling]
- **Bottlenecks:** [Known scaling bottlenecks]

## Error Handling

### Error Propagation

[How errors flow through the system. What happens when a component fails?]

### Retry Strategy

[What operations are retried? With what backoff? What is the idempotency model?]

### Circuit Breakers

[Where are circuit breakers? What triggers them? How do they recover?]

## Dependencies

### Runtime Dependencies

| Dependency | Version | Purpose | Criticality |
|---|---|---|---|
| [Dependency] | [version] | [why it's used] | [Critical / Important / Convenience] |

### External Services

| Service | Purpose | Failure Impact | Fallback |
|---|---|---|---|
| [Service] | [why it's used] | [what happens if it's down] | [fallback behavior] |

## Constraints and Limitations

- [Known architectural constraint and why it exists]
- [Known limitation and what would need to change to remove it]
- [Deliberate trade-off and what was traded for what]

## Future Considerations

[Technical debt, potential improvements, or evolution paths. Note: detailed roadmap and plans live in `/srv/sync/<domain>/plans/`, not here. This section covers architectural evolution only.]

- [Potential architectural change and what would trigger it]
- [Technical debt item and its impact]

---

## Related Documents

- [API Reference](API.md)
- [Schema](SCHEMA.md)
- [Deployment](DEPLOYMENT.md)
- [Security](../SECURITY.md)
- [ADR Index](adr/INDEX.md)
