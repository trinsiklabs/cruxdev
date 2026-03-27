---
title: Monitoring and Observability
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Monitoring and Observability

> What is monitored, how alerts are configured, and where to find dashboards for [project name].

## Observability Stack

| Layer | Tool | Purpose |
|---|---|---|
| Metrics | [Prometheus / Datadog / CloudWatch / etc.] | Numeric time-series data |
| Logging | [ELK / Loki / CloudWatch Logs / etc.] | Structured log aggregation |
| Tracing | [Jaeger / Zipkin / Datadog APM / etc.] | Distributed request tracing |
| Alerting | [PagerDuty / OpsGenie / Alertmanager / etc.] | Alert routing and escalation |
| Dashboards | [Grafana / Datadog / etc.] | Visualization |

## Dashboards

| Dashboard | URL | Purpose |
|---|---|---|
| Service Overview | [URL] | High-level health: traffic, errors, latency |
| Infrastructure | [URL] | CPU, memory, disk, network by host/container |
| Database | [URL] | Query performance, connections, replication lag |
| Business Metrics | [URL] | [Domain-specific metrics: signups, orders, etc.] |
| [Dashboard] | [URL] | [Purpose] |

## Metrics

### Application Metrics

| Metric | Type | Labels | Description |
|---|---|---|---|
| `http_requests_total` | Counter | `method`, `path`, `status` | Total HTTP requests |
| `http_request_duration_seconds` | Histogram | `method`, `path` | Request latency |
| `http_requests_in_flight` | Gauge | | Currently processing requests |
| `[custom_metric]` | [type] | [labels] | [description] |

### Infrastructure Metrics

| Metric | Source | Warning | Critical |
|---|---|---|---|
| CPU usage | [Agent / cAdvisor] | > 80% for 5m | > 95% for 2m |
| Memory usage | [Agent / cAdvisor] | > 80% | > 95% |
| Disk usage | [Agent] | > 80% | > 90% |
| Network errors | [Agent] | > 0.1% | > 1% |

### Database Metrics

| Metric | Warning | Critical | Notes |
|---|---|---|---|
| Active connections | > 80% of max | > 95% of max | Max is [N] |
| Replication lag | > [X]s | > [Y]s | [If applicable] |
| Slow queries (> [X]ms) | > [N]/min | > [M]/min | Check slow query log |
| Deadlocks | > 0 | > [N]/min | Investigate immediately |

### Business Metrics

| Metric | Description | Normal Range |
|---|---|---|
| [signups_total] | New user registrations | [X-Y per hour] |
| [orders_total] | Completed orders | [X-Y per hour] |
| [payment_failures_total] | Failed payment attempts | [< X per hour] |

## Alerts

### Alert Inventory

| Alert | Severity | Condition | Notification | Runbook |
|---|---|---|---|---|
| Service Down | Critical | Health check fails for > 2m | [PagerDuty] | [OPERATIONS.md #service-down](OPERATIONS.md) |
| High Error Rate | High | 5xx rate > 5% for 5m | [PagerDuty] | [OPERATIONS.md #high-error-rate](OPERATIONS.md) |
| High Latency | Medium | p99 > [X]ms for 10m | [Slack #alerts] | [OPERATIONS.md #high-latency](OPERATIONS.md) |
| Disk Space Low | Medium | Disk > 85% | [Slack #alerts] | [OPERATIONS.md #disk-space-low](OPERATIONS.md) |
| Certificate Expiry | Medium | Cert expires in < 14d | [Email] | Renew certificate |
| DB Connections High | High | > 90% of max pool | [PagerDuty] | [OPERATIONS.md #db-connections](OPERATIONS.md) |
| [Alert name] | [Severity] | [Condition] | [Channel] | [Runbook link] |

### Alert Configuration

[Where alert rules are defined and how to modify them.]

```yaml
# Example alert rule (Prometheus format)
- alert: HighErrorRate
  expr: rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m]) > 0.05
  for: 5m
  labels:
    severity: high
  annotations:
    summary: "High error rate on {{ $labels.instance }}"
    runbook: "docs/OPERATIONS.md#high-error-rate"
```

### Alert Routing

| Severity | Time | Channel | Escalation |
|---|---|---|---|
| Critical | Any time | PagerDuty (on-call) | Auto-escalate after 15m |
| High | Business hours | Slack #alerts + PagerDuty | Auto-escalate after 30m |
| High | Off hours | PagerDuty (on-call) | Auto-escalate after 15m |
| Medium | Business hours | Slack #alerts | Manual escalation |
| Low | Business hours | Slack #monitoring | No escalation; review weekly |

## Logging

### Log Format

```json
{
  "timestamp": "2026-03-24T12:00:00.000Z",
  "level": "info",
  "message": "Request processed",
  "service": "[project-name]",
  "request_id": "abc-123",
  "method": "GET",
  "path": "/api/resource",
  "status": 200,
  "duration_ms": 42,
  "user_id": "usr_def456"
}
```

### Log Levels

| Level | Usage | Example |
|---|---|---|
| `error` | Unexpected failures requiring attention | Database connection lost, unhandled exception |
| `warn` | Degraded behavior, not a failure | Retrying failed request, approaching rate limit |
| `info` | Normal operations worth recording | Request processed, job completed, config loaded |
| `debug` | Detailed diagnostic information | Query plans, cache hits/misses, serialization details |
| `trace` | Very verbose; rarely enabled | Function entry/exit, full request/response bodies |

### Log Access

```bash
# Tail live logs
[log tail command]

# Search logs by request ID
[search command, e.g.: journalctl | jq 'select(.request_id == "abc-123")']

# Search errors in time range
[time-range search command]

# Full-text search
[full-text search command]
```

### Log Retention

| Log Type | Retention | Storage |
|---|---|---|
| Application logs | [30 days] | [Log aggregation platform] |
| Access logs | [90 days] | [Log aggregation platform] |
| Audit logs | [1 year] | [Secure audit log storage] |
| Debug logs | [7 days] | [Local / short-term storage] |

## Tracing

### Trace Propagation

- **Header:** `[traceparent / X-Request-ID / X-Trace-ID]`
- **Format:** [W3C Trace Context / B3 / custom]
- **Sampling rate:** [100% / 10% / adaptive]

### Key Traces

| Operation | Expected Spans | Normal Duration |
|---|---|---|
| [API request] | [HTTP → handler → DB → response] | [X]ms |
| [Background job] | [Dequeue → process → DB → complete] | [X]ms |
| [External API call] | [Prepare → HTTP out → parse → respond] | [X]ms |

### Viewing Traces

1. Find the request ID from logs or API response headers
2. Open [tracing UI URL]
3. Search by trace ID or request ID
4. Examine span durations to identify bottlenecks

## Health Checks

| Endpoint | Checks | Expected Response |
|---|---|---|
| `GET /health` | Application is running | `200 OK` |
| `GET /health/ready` | App + DB + cache + dependencies | `200 OK` or `503` with failing checks |
| `GET /health/live` | Process is alive | `200 OK` |

```json
// /health/ready response when degraded
{
  "status": "degraded",
  "checks": {
    "database": { "status": "healthy", "latency_ms": 2 },
    "cache": { "status": "unhealthy", "error": "connection refused" },
    "external_api": { "status": "healthy", "latency_ms": 45 }
  }
}
```

## On-Call Expectations

### Before Your On-Call Shift

- [ ] Review recent deployments and known issues
- [ ] Verify you can access all dashboards and log systems
- [ ] Verify PagerDuty/OpsGenie is routing alerts to you
- [ ] Have [OPERATIONS.md](OPERATIONS.md) bookmarked

### During an Alert

1. Acknowledge the alert within [X minutes]
2. Check the linked dashboard and runbook
3. Follow the runbook in [OPERATIONS.md](OPERATIONS.md)
4. If resolution takes > [X minutes], post status update in [channel]
5. After resolution, create incident report if severity >= [threshold]

---

## Related Documents

- [Operations](OPERATIONS.md) — Runbooks for alert response
- [Performance](PERFORMANCE.md) — Performance targets and benchmarks
- [Architecture](ARCHITECTURE.md) — System context for monitoring
- [Configuration](CONFIGURATION.md) — Monitoring-related configuration
- [Security](../SECURITY.md) — Security monitoring and audit logging
