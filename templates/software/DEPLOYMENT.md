---
title: Deployment Guide
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Deployment Guide

> How to deploy [project name] to each environment.

## Environments

| Environment | URL | Purpose | Deploy Method |
|---|---|---|---|
| Development | `http://localhost:[port]` | Local development | Manual |
| Staging | `https://staging.example.com` | Pre-production testing | [CI/CD / Manual] |
| Production | `https://example.com` | Live service | [CI/CD] |

## Prerequisites

### Infrastructure

- [Server / container orchestrator / serverless platform]
- [Database: type, version, minimum resources]
- [Cache: type, version (if applicable)]
- [Message queue: type, version (if applicable)]
- [Object storage: provider (if applicable)]

### Access

- [ ] SSH access to deployment targets (or equivalent)
- [ ] Database credentials for target environment
- [ ] CI/CD pipeline access
- [ ] DNS management access (for first-time setup)
- [ ] SSL certificate provisioned

### Tools

- [Deployment tool] >= [version] (e.g., Docker >= 24.0, kubectl >= 1.28)
- [Build tool] >= [version]
- [Infrastructure tool] >= [version] (e.g., Terraform >= 1.6)

## Configuration

### Environment Variables

[All environment-specific config. For full configuration reference, see [CONFIGURATION.md](CONFIGURATION.md).]

| Variable | Required | Example | Description |
|---|---|---|---|
| `DATABASE_URL` | Yes | `postgres://user:pass@host:5432/db` | Database connection string |
| `REDIS_URL` | No | `redis://host:6379/0` | Cache connection |
| `SECRET_KEY` | Yes | `[generated]` | Application secret; must be unique per environment |
| `LOG_LEVEL` | No | `info` | Logging verbosity: debug, info, warn, error |
| `PORT` | No | `8080` | HTTP listen port |

### Secrets Management

[How secrets are stored and injected into the deployment.]

- Secrets are stored in: [Vault / AWS Secrets Manager / environment files / etc.]
- Secrets are injected via: [Environment variables / mounted files / etc.]
- Rotation policy: [How often secrets rotate and the procedure]

## Build

### Build Artifacts

```bash
# Build the application
[build command, e.g.: docker build -t project:latest .]

# Verify the build
[verification command, e.g.: docker run --rm project:latest --version]
```

### Build Outputs

| Artifact | Location | Description |
|---|---|---|
| [Container image] | `[registry/project:tag]` | Application container |
| [Binary] | `build/[binary-name]` | Compiled binary |
| [Static assets] | `build/static/` | Frontend assets |

## Deployment Procedure

### Standard Deployment (CI/CD)

```
1. Merge to [main/release branch]
2. CI pipeline runs:
   a. Lint and type checks
   b. Unit tests
   c. Integration tests
   d. Build artifacts
   e. Push artifacts to registry
3. CD pipeline runs:
   a. Deploy to staging
   b. Run smoke tests on staging
   c. [Manual approval gate / automatic promotion]
   d. Deploy to production
   e. Run smoke tests on production
   f. Monitor for [X minutes]
```

### Manual Deployment

[When CI/CD is unavailable or for the first deployment.]

```bash
# 1. Connect to deployment target
ssh deploy@[host]

# 2. Pull latest artifacts
[pull command]

# 3. Run database migrations (if any)
[migration command]

# 4. Deploy the application
[deploy command]

# 5. Verify deployment
[health check command]
```

### First-Time Setup

[Steps that only need to happen once, for a brand new environment.]

```bash
# 1. Provision infrastructure
[infrastructure provisioning commands or instructions]

# 2. Create database
[database creation commands]

# 3. Run initial migrations
[initial migration command]

# 4. Seed required data (if any)
[seed command]

# 5. Configure DNS
[DNS configuration steps]

# 6. Provision SSL
[SSL provisioning steps]

# 7. Deploy application (follow Standard or Manual procedure above)

# 8. Verify end-to-end
[full verification steps]
```

## Database Migrations

### Running Migrations

```bash
# Check pending migrations
[pending check command]

# Run all pending migrations
[migration run command]

# Verify migration status
[migration status command]
```

### Migration Safety Rules

- [ ] All migrations are backward-compatible (old code can run against new schema)
- [ ] Destructive migrations (drop table/column) are separated from deploy by at least one release
- [ ] Migrations are tested in staging before production
- [ ] Large data migrations run outside of the deploy process

## Post-Deployment Verification

### Smoke Tests

```bash
# Health check
curl -f https://[host]/health

# Key functionality check
[smoke test commands]
```

### Monitoring Checklist

After deployment, verify:

- [ ] Health endpoint returns 200
- [ ] No increase in error rate (check [monitoring dashboard URL])
- [ ] No increase in latency (check [monitoring dashboard URL])
- [ ] Log output is clean (no unexpected errors)
- [ ] Background jobs are processing (if applicable)
- [ ] [Service-specific checks]

## Rollback

### Automated Rollback

[If CI/CD supports automatic rollback, describe the trigger conditions.]

### Manual Rollback

```bash
# 1. Identify the last known good version
[command to list recent deployments/versions]

# 2. Roll back to that version
[rollback command]

# 3. Verify rollback
[health check command]

# 4. Roll back database migrations (if needed and safe)
[migration rollback command — WARNING: may cause data loss]
```

### Rollback Decision Tree

```
Is the service down?
  YES → Immediate rollback, investigate after
  NO → Is error rate > [X]%?
    YES → Rollback within [Y] minutes
    NO → Is the bug user-facing?
      YES → Evaluate severity; rollback or hotfix
      NO → Fix forward in next deployment
```

## Scaling

### Horizontal Scaling

```bash
# Scale to N instances
[scaling command, e.g.: kubectl scale deployment/project --replicas=N]
```

### Vertical Scaling

[Steps to increase resources for a single instance.]

## Maintenance Windows

- **Scheduled maintenance:** [Day/time, timezone, frequency]
- **Notification process:** [How users are notified of maintenance]
- **Zero-downtime deployments:** [Yes/No. If no, describe the maintenance window procedure.]

---

## Related Documents

- [Architecture](ARCHITECTURE.md) — System design context
- [Configuration](CONFIGURATION.md) — Full configuration reference
- [Operations](OPERATIONS.md) — Operational runbooks
- [Monitoring](MONITORING.md) — Observability setup
- [Security](../SECURITY.md) — Security considerations
