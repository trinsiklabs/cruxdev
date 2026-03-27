---
title: Performance Documentation
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Performance Documentation

> Performance characteristics, benchmarks, and capacity planning for [project name].

## Performance Targets

### Service Level Objectives (SLOs)

| Metric | Target | Measurement | Current |
|---|---|---|---|
| Availability | [99.9%] | [Uptime over rolling 30 days] | [actual] |
| p50 latency | [< Xms] | [Median response time] | [actual] |
| p95 latency | [< Xms] | [95th percentile response time] | [actual] |
| p99 latency | [< Xms] | [99th percentile response time] | [actual] |
| Throughput | [X req/s] | [Sustained request rate] | [actual] |
| Error rate | [< 0.1%] | [5xx responses / total responses] | [actual] |

### Error Budget

- **Budget:** [100% - SLO target = error budget, e.g., 0.1% for 99.9% SLO]
- **Window:** [Rolling 30 days]
- **Current consumption:** [X% of budget consumed]
- **Policy when exhausted:** [Feature freeze / reduced deployments / etc.]

## Benchmarks

### Methodology

- **Tool:** [wrk / k6 / locust / ab / custom]
- **Environment:** [Hardware specs of the test environment]
- **Data set:** [Size and characteristics of test data]
- **Warmup:** [Duration and method of warmup before measurement]
- **Duration:** [How long each benchmark runs]
- **Last run:** [Date of most recent benchmark]

### Endpoint Benchmarks

| Endpoint | Method | Concurrency | Avg Latency | p99 Latency | Throughput | Error Rate |
|---|---|---|---|---|---|---|
| `/health` | GET | 100 | [X]ms | [X]ms | [X] req/s | 0% |
| `/api/resource` | GET | 100 | [X]ms | [X]ms | [X] req/s | [X]% |
| `/api/resource` | POST | 50 | [X]ms | [X]ms | [X] req/s | [X]% |
| `/api/resource/:id` | GET | 100 | [X]ms | [X]ms | [X] req/s | [X]% |
| `[endpoint]` | [method] | [concurrency] | [avg] | [p99] | [throughput] | [error] |

### Reproducing Benchmarks

```bash
# Install benchmark tool
[install command]

# Run the full benchmark suite
[benchmark command]

# Run a specific benchmark
[specific benchmark command]

# Example with wrk:
wrk -t4 -c100 -d30s --latency http://localhost:8080/api/resource
```

## Resource Consumption

### Memory

| Component | Baseline | Under Load | Peak | Notes |
|---|---|---|---|---|
| Application process | [X] MB | [X] MB | [X] MB | [Notes on growth pattern] |
| Database connections | [X] MB | [X] MB | [X] MB | [Per-connection overhead] |
| Cache | [X] MB | [X] MB | [X] MB | [Bounded by config] |
| Total per instance | [X] MB | [X] MB | [X] MB | |

### CPU

| Component | Idle | Under Load | Peak | Notes |
|---|---|---|---|---|
| Application | [X]% | [X]% | [X]% | [Notes] |
| Database | [X]% | [X]% | [X]% | [Notes] |

### Disk I/O

| Component | Read | Write | Notes |
|---|---|---|---|
| Database | [X] IOPS | [X] IOPS | [Query patterns] |
| Logging | N/A | [X] MB/hr | [Log volume] |
| [Component] | [read] | [write] | [notes] |

### Network

| Path | Bandwidth | Latency | Notes |
|---|---|---|---|
| Client → Application | [X] Mbps typical | [X]ms | [Notes] |
| Application → Database | [X] Mbps typical | [X]ms | [Notes] |
| Application → Cache | [X] Mbps typical | [X]ms | [Notes] |
| Application → External API | [X] Mbps typical | [X]ms | [Notes] |

## Known Bottlenecks

| Bottleneck | Impact | Trigger | Mitigation | Status |
|---|---|---|---|---|
| [Database query X] | [Latency spike on endpoint Y] | [When table exceeds N rows] | [Add index / partition / cache] | [Mitigated / Open] |
| [External API rate limit] | [429 errors on feature Z] | [> N requests/minute] | [Request batching / cache] | [Mitigated / Open] |
| [Single-threaded component] | [CPU saturation] | [> N concurrent requests] | [Parallelize / scale out] | [Open] |

## Profiling

### How to Profile

```bash
# CPU profiling
[profiling command, e.g.: go tool pprof http://localhost:8080/debug/pprof/profile?seconds=30]

# Memory profiling
[memory profiling command]

# Trace
[tracing command]
```

### Recent Profiling Results

**Date:** [YYYY-MM-DD]
**Scenario:** [What was being profiled]

| Hot Path | % CPU | Action Taken |
|---|---|---|
| [Function/module] | [X]% | [Optimized / Accepted / Needs work] |
| [Function/module] | [X]% | [Optimized / Accepted / Needs work] |

## Database Performance

### Slow Queries

| Query Pattern | Avg Time | p99 Time | Frequency | Index Used | Action |
|---|---|---|---|---|---|
| [SELECT from table WHERE ...] | [X]ms | [X]ms | [X/min] | [Yes/No] | [Added index / Optimized / Acceptable] |

### Connection Pool Tuning

| Parameter | Value | Rationale |
|---|---|---|
| Pool size | [N] | [Based on: expected concurrency / DB max connections / etc.] |
| Idle timeout | [N]s | [Why this value] |
| Max lifetime | [N]m | [Why this value] |

## Capacity Planning

### Current Capacity

| Resource | Capacity | Current Usage | Headroom |
|---|---|---|---|
| Application instances | [N] instances | [X]% utilized | [Y]% |
| Database connections | [N] max | [X] average | [Y] remaining |
| Disk space | [N] GB | [X] GB used | [Y] GB free |
| Memory | [N] GB per instance | [X] GB used | [Y] GB free |

### Growth Projections

| Metric | Current | 3 Months | 6 Months | 12 Months | Source |
|---|---|---|---|---|---|
| Requests/day | [X] | [X] | [X] | [X] | [Traffic growth trend] |
| Data volume | [X] GB | [X] GB | [X] GB | [X] GB | [Data growth rate] |
| Users | [X] | [X] | [X] | [X] | [User growth rate] |

### Scaling Triggers

| Trigger | Threshold | Action | Lead Time |
|---|---|---|---|
| CPU sustained > [X]% | [X]% for [N] minutes | Add application instance | [Immediate / X minutes] |
| Memory > [X]% | [X]% | Investigate leaks or scale vertically | [Hours] |
| Disk > [X]% | [X]% | Expand storage or archive old data | [Days] |
| Database connections > [X]% | [X]% of max | Increase pool or add read replica | [Hours] |
| Response latency p99 > [X]ms | [X]ms for [N] minutes | Profile and optimize or scale | [Days] |

## Optimization History

| Date | Optimization | Impact | Plan |
|---|---|---|---|
| [YYYY-MM-DD] | [What was optimized] | [Measured improvement] | [PLAN-XXXX] |
| [YYYY-MM-DD] | [What was optimized] | [Measured improvement] | [PLAN-XXXX] |

---

## Related Documents

- [Architecture](ARCHITECTURE.md) — System design affecting performance
- [Schema](SCHEMA.md) — Database design and indexing
- [Configuration](CONFIGURATION.md) — Performance-related configuration
- [Operations](OPERATIONS.md) — Performance incident runbooks
- [Monitoring](MONITORING.md) — Performance dashboards and alerts
