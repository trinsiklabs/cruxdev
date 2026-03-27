---
title: Operations Runbook
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Operations Runbook

> Operational procedures for [project name]. Used by on-call engineers and operators.

## Service Overview

| Property | Value |
|---|---|
| **Service name** | [project name] |
| **Owner** | [Team / individual] |
| **On-call rotation** | [PagerDuty / OpsGenie / schedule link] |
| **Escalation path** | [On-call → team lead → engineering manager] |
| **Critical dependencies** | [Database, cache, external APIs] |
| **SLO** | [99.9% availability / p99 < 200ms / etc.] |

## Health Checks

### Endpoints

| Endpoint | Expected Response | Checks |
|---|---|---|
| `GET /health` | `200 OK` | Application is running |
| `GET /health/ready` | `200 OK` | Application + all dependencies healthy |
| `GET /health/live` | `200 OK` | Process is alive (liveness probe) |

### Manual Health Check

```bash
# Quick health check
curl -sf https://[host]/health | jq .

# Full readiness check
curl -sf https://[host]/health/ready | jq .

# Check from specific host
ssh [host] 'curl -sf http://localhost:[port]/health'
```

## Common Operational Tasks

### Restart Service

```bash
# Graceful restart (finishes in-flight requests)
[restart command, e.g.: systemctl restart project-name]

# Verify after restart
[health check command]

# Check logs for startup errors
[log command, e.g.: journalctl -u project-name --since "5 minutes ago"]
```

### View Logs

```bash
# Tail live logs
[log tail command, e.g.: journalctl -u project-name -f]

# Search for errors in last hour
[log search command, e.g.: journalctl -u project-name --since "1 hour ago" | grep ERROR]

# Structured log query (if using structured logging)
[structured log query command]
```

### Check Resource Usage

```bash
# CPU and memory
[resource check command, e.g.: systemctl status project-name]

# Disk usage
df -h [relevant mount points]

# Open connections
[connection check command, e.g.: ss -tlnp | grep [port]]

# Database connection pool
[pool check command]
```

### Scale Up / Down

```bash
# Scale to N instances
[scale command]

# Verify scaling
[verification command]
```

### Database Operations

```bash
# Check database connectivity
[db ping command]

# Check connection pool status
[pool status command]

# Run pending migrations
[migration command]

# Check migration status
[migration status command]

# Emergency: read-only mode
[read-only mode command, if applicable]
```

### Cache Operations

```bash
# Check cache connectivity
[cache ping command]

# Flush cache (CAUTION: causes temporary performance degradation)
[cache flush command]

# Check cache hit rate
[cache stats command]
```

### Queue / Background Job Operations

```bash
# Check queue depth
[queue depth command]

# Check worker status
[worker status command]

# Retry failed jobs
[retry command]

# Purge dead letter queue (after investigation)
[purge DLQ command]
```

## Incident Response Runbooks

### Runbook: Service Down (5xx or Unreachable)

```
SEVERITY: Critical
RESPONSE TIME: Immediate

1. VERIFY: Is the service actually down?
   - Check health endpoint: curl -sf https://[host]/health
   - Check from multiple locations (not just your machine)
   - Check monitoring dashboard: [dashboard URL]

2. IDENTIFY: What's causing the outage?
   a. Check service logs: [log command]
   b. Check resource usage: [resource command]
   c. Check dependency health:
      - Database: [db check command]
      - Cache: [cache check command]
      - External APIs: [external check command]
   d. Check recent deployments: [deploy history command]

3. MITIGATE:
   - If recent deployment caused it → ROLLBACK: [rollback command]
   - If resource exhaustion → SCALE: [scale command]
   - If dependency down → Check dependency status page / contact vendor
   - If configuration issue → Fix config and restart

4. VERIFY RECOVERY:
   - Health endpoint returns 200
   - Error rate back to baseline
   - No data loss or corruption

5. POST-INCIDENT:
   - Create incident report
   - Schedule post-mortem within 48 hours
```

### Runbook: High Error Rate

```
SEVERITY: High
RESPONSE TIME: < 15 minutes

1. CHECK: What errors are occurring?
   - Dashboard: [error rate dashboard URL]
   - Logs: [command to filter recent errors]
   - Categorize: Is it one error type or many?

2. SCOPE: How many users are affected?
   - [command or dashboard to check affected user count]

3. CORRELATE: What changed?
   - Recent deployments: [deploy history command]
   - Dependency issues: [dependency check command]
   - Traffic spike: [traffic dashboard URL]

4. ACT:
   - If deployment-related → Rollback
   - If dependency-related → Fail gracefully / circuit break
   - If traffic-related → Scale up / rate limit
   - If bug → Hotfix if possible; rollback if not

5. MONITOR: Watch error rate return to baseline
```

### Runbook: High Latency

```
SEVERITY: Medium to High
RESPONSE TIME: < 30 minutes

1. MEASURE: What's the current latency?
   - Dashboard: [latency dashboard URL]
   - Manual: curl -w "%{time_total}\n" -o /dev/null -sf https://[host]/health

2. ISOLATE: Where is the latency?
   - Application processing time: [check APM / application metrics]
   - Database query time: [check slow query log / pg_stat_activity]
   - Network latency: [network check]
   - External API latency: [external API metrics]

3. ACT:
   - Slow queries → Check for missing indexes, lock contention, table bloat
   - Application CPU → Scale horizontally or investigate hot path
   - External API → Enable circuit breaker / use cached response
   - Memory pressure → Check for leaks; restart if needed

4. VERIFY: Latency returns to normal
```

### Runbook: Disk Space Low

```
SEVERITY: Medium (High if < 5%)
RESPONSE TIME: < 1 hour

1. CHECK: What's consuming space?
   du -sh [data directories]/* | sort -rh | head -20

2. SAFE CLEANUPS:
   - Log rotation: [force log rotation command]
   - Temp files: [cleanup command]
   - Old artifacts: [cleanup command]
   - Database vacuum: [vacuum command, if applicable]

3. UNSAFE CLEANUPS (require approval):
   - Old backups: [backup cleanup command]
   - Old data: [data archival command]

4. LONG-TERM: If recurring, increase disk allocation or add archival policy
```

### Runbook: Database Connection Exhaustion

```
SEVERITY: High
RESPONSE TIME: < 15 minutes

1. CHECK: Current connection count
   [command to check active connections, e.g.:]
   SELECT count(*) FROM pg_stat_activity WHERE datname = '[db_name]';

2. IDENTIFY: What's holding connections?
   [command to see connection details, e.g.:]
   SELECT pid, usename, state, query_start, query
   FROM pg_stat_activity WHERE datname = '[db_name]'
   ORDER BY query_start;

3. MITIGATE:
   - Kill long-running idle connections: [kill command]
   - Restart application (refreshes pool): [restart command]
   - If due to traffic spike: scale application (more instances, smaller pools)

4. PREVENT: Review connection pool settings in [CONFIGURATION.md](CONFIGURATION.md)
```

## Scheduled Maintenance

### Recurring Tasks

| Task | Frequency | Command | Purpose |
|---|---|---|---|
| Database vacuum | [Weekly] | `[command]` | Reclaim space, update statistics |
| Log rotation | [Daily] | `[command or automatic]` | Prevent disk fill |
| Certificate renewal | [Before expiry] | `[command]` | Maintain TLS |
| Dependency updates | [Weekly/Monthly] | `[command]` | Security patches |
| Backup verification | [Monthly] | `[command]` | Confirm backups are restorable |

### Maintenance Window Procedure

```
1. Notify users: [notification method and timing]
2. Enable maintenance mode: [command]
3. Perform maintenance tasks
4. Verify system health
5. Disable maintenance mode: [command]
6. Verify user-facing functionality
7. Send all-clear notification
```

## Monitoring and Alerting

[For full monitoring details, see [MONITORING.md](MONITORING.md) if it exists.]

### Key Metrics

| Metric | Normal Range | Warning | Critical | Dashboard |
|---|---|---|---|---|
| Error rate | < 0.1% | > 1% | > 5% | [URL] |
| p99 latency | < [X]ms | > [Y]ms | > [Z]ms | [URL] |
| CPU usage | < 70% | > 80% | > 95% | [URL] |
| Memory usage | < 70% | > 80% | > 95% | [URL] |
| Disk usage | < 70% | > 80% | > 95% | [URL] |
| Queue depth | < [X] | > [Y] | > [Z] | [URL] |
| DB connections | < [X] | > [Y] | > max | [URL] |

### Alert Response

When you receive an alert:

1. Acknowledge the alert in [alerting system]
2. Find the matching runbook above
3. Follow the runbook steps
4. If no runbook matches, escalate to [escalation target]

## Emergency Procedures

### Emergency Stop

```bash
# Stop all application instances immediately
[emergency stop command]

# Verify stopped
[verification command]
```

### Emergency Database Access

```bash
# Direct database access (production — use only in emergencies)
[database connection command with appropriate warnings]
```

### Emergency Rollback

See [DEPLOYMENT.md](DEPLOYMENT.md) — Rollback section.

---

## Related Documents

- [Deployment](DEPLOYMENT.md) — Deployment and rollback procedures
- [Architecture](ARCHITECTURE.md) — System design context
- [Monitoring](MONITORING.md) — Detailed monitoring setup
- [Configuration](CONFIGURATION.md) — Configuration reference
- [Security](../SECURITY.md) — Security incident procedures
- [Troubleshooting](TROUBLESHOOTING.md) — Problem diagnosis
