---
title: Migration Guide
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Migration Guide

> Version upgrade and migration procedures for [project name].

## Migration Index

| From Version | To Version | Breaking Changes | Difficulty | Estimated Time |
|---|---|---|---|---|
| [vX.Y] | [vX.Y+1] | [Yes / No] | [Low / Medium / High] | [X minutes / hours] |
| [vX.Y-1] | [vX.Y] | [Yes / No] | [Low / Medium / High] | [X minutes / hours] |

---

## Migrating from vX.Y to vX.Y+1

### Summary

[One paragraph describing what changed and why migration is needed.]

### Breaking Changes

| Change | Impact | Action Required |
|---|---|---|
| [API endpoint renamed] | [Clients using old endpoint will get 404] | [Update client to use new endpoint] |
| [Config option removed] | [Application won't start with old config] | [Remove option or replace with new one] |
| [Database column type changed] | [Data migration required] | [Run migration script] |

### Prerequisites

- [ ] Current version is [vX.Y] (migrations cannot skip versions)
- [ ] Database backup completed
- [ ] Maintenance window scheduled (if needed): [estimated downtime]
- [ ] All clients notified of breaking changes
- [ ] [Other prerequisites]

### Pre-Migration Checklist

```bash
# 1. Verify current version
[version check command]

# 2. Create database backup
[backup command]

# 3. Verify backup is restorable
[backup verification command]

# 4. Check for deprecated feature usage
[deprecation check command, if applicable]
```

### Migration Steps

#### Step 1: Update Application Code

```bash
# Pull the new version
[update command, e.g.: git pull / docker pull / package update]

# Verify the new version is ready
[verification command]
```

#### Step 2: Update Configuration

[Describe configuration changes needed.]

```diff
# config.yaml changes:
- old_setting: value
+ new_setting: value

# Environment variable changes:
- OLD_ENV_VAR=value          # Remove this
+ NEW_ENV_VAR=value          # Add this
```

| Old Setting | New Setting | Notes |
|---|---|---|
| `old_name` | `new_name` | Renamed for clarity |
| `removed_setting` | (removed) | No longer needed; behavior is now default |
| (new) | `new_setting` | New option with default `[value]` |

#### Step 3: Run Database Migrations

```bash
# Check pending migrations
[pending check command]

# Run migrations
[migration command]

# Verify migration success
[verification command]
```

**Migration details:**

| Migration | Description | Reversible | Data Impact |
|---|---|---|---|
| `[migration_name]` | [What it does] | [Yes / No] | [None / transforms X / deletes Y] |

#### Step 4: Run Data Transformations

[If there are data transformations beyond schema changes.]

```bash
# Transform existing data
[data transformation command]

# Verify data integrity
[verification command]
```

#### Step 5: Deploy Updated Application

```bash
# Deploy (follow standard deployment procedure)
[deploy command]

# Verify deployment
[health check command]
```

#### Step 6: Post-Migration Verification

```bash
# Verify version
[version check command]

# Run smoke tests
[smoke test command]

# Check for errors
[error log check command]
```

**Verification checklist:**

- [ ] Health endpoint returns new version
- [ ] No errors in logs
- [ ] API responses are correct format
- [ ] [Feature-specific checks]
- [ ] Performance is within expected range

### Rollback Procedure

If the migration fails:

```bash
# 1. Stop the new version
[stop command]

# 2. Roll back database migrations (if reversible)
[migration rollback command]

# 3. Restore database from backup (if migrations are not reversible)
[restore command]

# 4. Deploy the previous version
[deploy previous version command]

# 5. Verify rollback
[health check command]
```

**Rollback window:** [How long after migration you can still roll back safely]

### API Migration Guide

[For clients that need to update their API usage.]

#### Endpoint Changes

| Old Endpoint | New Endpoint | Notes |
|---|---|---|
| `GET /api/v1/widgets` | `GET /api/v2/widgets` | Response format changed |
| `POST /api/v1/orders` | `POST /api/v2/orders` | New required field: `currency` |

#### Request Format Changes

```json
// Old format (vX.Y)
{
  "name": "Example",
  "type": 1
}

// New format (vX.Y+1)
{
  "name": "Example",
  "type": "widget",         // Changed from integer to string enum
  "currency": "USD"          // New required field
}
```

#### Response Format Changes

```json
// Old format (vX.Y)
{
  "id": 123,
  "data": { ... }
}

// New format (vX.Y+1)
{
  "id": "wgt_abc123",       // Changed from integer to string ID
  "data": { ... },
  "meta": { ... }           // New metadata envelope
}
```

### Deprecation Timeline

| Deprecated Feature | Deprecated In | Removed In | Replacement |
|---|---|---|---|
| [Feature/endpoint] | [vX.Y] | [vX.Y+2] | [Replacement feature] |
| [Feature/endpoint] | [vX.Y] | [vX.Y+2] | [Replacement feature] |

---

## General Migration Principles

### Before Any Migration

1. **Read the changelog:** [CHANGELOG.md](../CHANGELOG.md)
2. **Back up everything:** Database, configuration, any persistent data
3. **Test in staging first:** Never migrate production without testing in staging
4. **Schedule maintenance window** if downtime is required
5. **Notify stakeholders** of the migration schedule and expected impact

### Migration Safety Rules

- **Never skip versions.** Migrate sequentially: v1 → v2 → v3, not v1 → v3
- **Always back up before migrating.** Verify the backup is restorable
- **Test rollback before production.** Know that you CAN roll back
- **Monitor after migration.** Watch error rates and latency for at least [X] minutes

---

## Related Documents

- [Changelog](../CHANGELOG.md) — What changed in each version
- [Deployment](DEPLOYMENT.md) — Deployment procedures
- [Schema](SCHEMA.md) — Database schema details
- [API Reference](API.md) — API endpoint documentation
- [Configuration](CONFIGURATION.md) — Configuration reference
