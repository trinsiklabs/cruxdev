---
title: Database Schema
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Database Schema

> Database schema documentation for [project name].

## Overview

- **Database:** [PostgreSQL / MySQL / SQLite / MongoDB / etc.] [version]
- **ORM/Query Builder:** [Tool name and version, if applicable]
- **Migration Tool:** [Tool name and version]
- **Schema Version:** [Current migration version or count]

## Entity-Relationship Overview

```
[ASCII ERD or description of the main entity relationships]

┌──────────┐     ┌──────────────┐     ┌──────────┐
│  users   │────<│ user_roles   │>────│  roles   │
│          │     └──────────────┘     │          │
│          │                          └──────────┘
│          │     ┌──────────────┐
│          │────<│   orders     │
└──────────┘     │              │
                 │              │────<┌──────────────┐
                 └──────────────┘     │ order_items  │
                                      │              │>────┌──────────┐
                                      └──────────────┘     │ products │
                                                           └──────────┘

Legend: ────< one-to-many    >────< many-to-many
```

## Tables

### users

[Description of what this table stores and its role in the system.]

| Column | Type | Nullable | Default | Description |
|---|---|---|---|---|
| `id` | uuid | No | `gen_random_uuid()` | Primary key |
| `email` | varchar(255) | No | | User's email address; unique |
| `name` | varchar(255) | No | | Display name |
| `password_hash` | varchar(255) | No | | Bcrypt password hash |
| `status` | varchar(20) | No | `'active'` | Account status: active, suspended, deleted |
| `email_verified_at` | timestamp | Yes | `null` | When email was verified; null = unverified |
| `last_login_at` | timestamp | Yes | `null` | Most recent login timestamp |
| `created_at` | timestamp | No | `now()` | Row creation timestamp |
| `updated_at` | timestamp | No | `now()` | Last modification timestamp |

**Indexes:**

| Name | Columns | Type | Purpose |
|---|---|---|---|
| `users_pkey` | `id` | Primary | Primary key |
| `users_email_unique` | `email` | Unique | Email uniqueness |
| `users_status_idx` | `status` | B-tree | Filter by status |
| `users_created_at_idx` | `created_at` | B-tree | Sort by creation date |

**Constraints:**

| Name | Type | Definition |
|---|---|---|
| `users_email_check` | Check | `email ~* '^[^@]+@[^@]+$'` |
| `users_status_check` | Check | `status IN ('active', 'suspended', 'deleted')` |

**Relationships:**

| Relationship | Table | Type | FK Column |
|---|---|---|---|
| Has many | `orders` | One-to-many | `orders.user_id` |
| Has many | `user_roles` | One-to-many | `user_roles.user_id` |

---

### roles

[Description of what this table stores.]

| Column | Type | Nullable | Default | Description |
|---|---|---|---|---|
| `id` | uuid | No | `gen_random_uuid()` | Primary key |
| `name` | varchar(50) | No | | Role name; unique |
| `description` | text | Yes | | Human-readable description |
| `permissions` | jsonb | No | `'{}'` | Permission set as JSON |
| `created_at` | timestamp | No | `now()` | Row creation timestamp |

**Indexes:**

| Name | Columns | Type | Purpose |
|---|---|---|---|
| `roles_pkey` | `id` | Primary | Primary key |
| `roles_name_unique` | `name` | Unique | Role name uniqueness |

---

### [additional tables follow the same pattern]

---

## Enumerations / Lookup Values

### User Status Values

| Value | Description | Transitions To |
|---|---|---|
| `active` | Normal active account | `suspended`, `deleted` |
| `suspended` | Temporarily disabled | `active`, `deleted` |
| `deleted` | Soft-deleted; data retained [X] days | (terminal) |

### [Other Enumerations]

| Value | Description |
|---|---|
| `[value]` | [description] |

## JSON Column Schemas

### roles.permissions

```json
{
  "resources": {
    "read": true,
    "write": true,
    "delete": false
  },
  "admin": {
    "manage_users": false,
    "manage_roles": false
  }
}
```

| Path | Type | Description |
|---|---|---|
| `resources.read` | boolean | Can read resources |
| `resources.write` | boolean | Can create/update resources |
| `resources.delete` | boolean | Can delete resources |
| `admin.manage_users` | boolean | Can manage user accounts |
| `admin.manage_roles` | boolean | Can modify roles |

## Views

### [view_name]

- **Purpose:** [What this view provides]
- **Definition:** [Brief description or actual SQL]
- **Used by:** [Which parts of the application use this view]

## Functions / Stored Procedures

### [function_name]

- **Purpose:** [What this function does]
- **Parameters:** [Input parameters]
- **Returns:** [Return type]
- **Used by:** [Where this is called from]

## Triggers

### [trigger_name]

- **Table:** [Which table]
- **Event:** [INSERT / UPDATE / DELETE]
- **Timing:** [BEFORE / AFTER]
- **Purpose:** [What it does]

## Migration History

| Version | Date | Description | Reversible |
|---|---|---|---|
| `001` | [YYYY-MM-DD] | Create users table | Yes |
| `002` | [YYYY-MM-DD] | Create roles and user_roles tables | Yes |
| `003` | [YYYY-MM-DD] | Add email_verified_at to users | Yes |
| `004` | [YYYY-MM-DD] | Add permissions jsonb to roles | Yes |

## Data Retention

| Table | Retention Period | Deletion Method | Notes |
|---|---|---|---|
| `users` | Soft-delete + 90 days | Background job purges | GDPR compliance |
| `orders` | 7 years | Archive to cold storage | Tax/audit requirements |
| `[table]` | [period] | [method] | [notes] |

## Performance Considerations

### Large Tables

| Table | Expected Row Count | Growth Rate | Partitioning |
|---|---|---|---|
| `[table]` | [count] | [rows/day] | [None / by date / by ID range] |

### Query Patterns

| Pattern | Frequency | Index Used | Notes |
|---|---|---|---|
| Lookup user by email | Very high | `users_email_unique` | Primary auth path |
| List orders by user | High | `orders_user_id_idx` | Dashboard query |
| [Pattern] | [Frequency] | [Index] | [Notes] |

## Backup and Recovery

- **Backup frequency:** [How often: continuous WAL / hourly / daily]
- **Backup retention:** [How long backups are kept]
- **Recovery procedure:** See [OPERATIONS.md](OPERATIONS.md)
- **Point-in-time recovery:** [Supported / Not supported]

---

## Related Documents

- [Architecture](ARCHITECTURE.md) — System design context for data decisions
- [Migration Guide](MIGRATION.md) — Version upgrade procedures
- [API Reference](API.md) — How the API maps to schema entities
- [Security](../SECURITY.md) — Data protection and access controls
