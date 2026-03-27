# Metrics Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 40+ sources including Google SRE, DORA 2024-2025, OpenTelemetry, Brendan Gregg (USE), SPACE framework, CHAOSS, FinOps Foundation, WCAG
**Last updated:** 2026-03-27

## Why This Matters

You cannot improve what you don't measure. But measuring the wrong things is worse than measuring nothing — Goodhart's Law ("when a measure becomes a target, it ceases to be a good measure") is pervasive in software. This document defines what to measure, why, and how, organized by the authoritative frameworks.

---

## 1. Framework Overview

| Framework | Domain | Core Principle | When to Use |
|-----------|--------|---------------|-------------|
| **Four Golden Signals** | Service monitoring | Latency, Traffic, Errors, Saturation | Any request-serving system |
| **USE Method** | Infrastructure | Utilization, Saturation, Errors per resource | Capacity planning, performance debugging |
| **RED Method** | Microservices | Rate, Errors, Duration per service | Request-driven architectures |
| **DORA** | Software delivery | 5 metrics across throughput + stability | Engineering team health |
| **SPACE** | Developer productivity | 5 dimensions, measure >= 3 | Avoid single-metric traps |
| **Core Web Vitals** | User experience | LCP, INP, CLS at p75 | Any web application |
| **CHAOSS** | Open source health | Response time, closure ratio, bus factor | Open source projects |
| **FinOps** | Cost | Cost per business unit | Cloud infrastructure |

**Rule: Use at least 2 frameworks.** No single framework covers all dimensions. Infrastructure teams use USE + Golden Signals. Product teams use DORA + Core Web Vitals. Open source projects use CHAOSS + DORA.

---

## 2. Infrastructure Metrics (USE Method)

For every resource, check Utilization, Saturation, and Errors.

| Resource | Utilization | Saturation | Errors | Collect Via |
|----------|-------------|------------|--------|-------------|
| **CPU** | % busy per core | Run-queue length | ECC errors | node_exporter, OTel |
| **Memory** | Used/available | Swap activity, paging | Failed malloc | /proc/meminfo, cgroups |
| **Disk I/O** | Device busy % | Wait queue length | Device errors | iostat, node_exporter |
| **Disk capacity** | % used | N/A | FS errors | df |
| **Network** | RX/TX vs bandwidth | Dropped packets | Link errors | /proc/net/dev |
| **Containers** | CPU/mem vs limits | Throttling, OOM kills | Restart count | cAdvisor |

**Thresholds:**
- CPU > 70% sustained → investigate (queuing begins)
- Memory > 85% → warning; any swap = critical
- Disk I/O > 80% → bottleneck
- Network loss > 0.1% → investigate
- Any non-zero saturation → investigate

**Automation:** Fully automatable via Prometheus node_exporter or OpenTelemetry host metrics.

---

## 3. Application Metrics (Golden Signals + RED)

| Metric | What It Measures | Good | Bad | Collect Via |
|--------|-----------------|------|-----|-------------|
| **Latency** | Time to serve request | p50 < 100ms, p99 < 500ms | p99 > 1s or growing | OTel traces, histograms |
| **Throughput** | Requests/sec | Stable or proportional growth | Sudden drops/spikes | Counters, LB logs |
| **Error rate** | % failed requests | < 0.1% | > 1% | HTTP status codes |
| **Saturation** | How "full" the service | < 70% capacity | > 85% | Queue depths, pool usage |
| **Apdex** | User satisfaction ratio | > 0.9 | < 0.7 | APM tools |
| **Dependency health** | Downstream latency/errors | All green | Any degraded | Circuit breaker metrics |

**Key insight (Google SRE):** Track error latency separately from success latency. A slow error is worse than a fast error.

**Automation:** OpenTelemetry auto-instrumentation for most frameworks. Zero custom code needed.

---

## 4. Business Metrics

| Metric | What | Why | Good | Bad |
|--------|------|-----|------|-----|
| **MRR/ARR** | Recurring revenue | Core SaaS health | Growing MoM | Declining |
| **NRR** | Net revenue retention | Expansion vs churn | > 110% | < 90% |
| **Churn** | Customers lost/period | Retention health | < 5% annual | > 7% annual |
| **LTV:CAC** | Lifetime value vs acquisition cost | Unit economics | > 3:1 | < 1:1 |
| **CAC payback** | Months to recover acquisition cost | Cash flow | < 12 months | > 24 months |
| **NPS** | Would you recommend? | Satisfaction proxy | > 30 | < 0 |
| **Time to Value** | Signup → first outcome | Onboarding quality | Minutes | Days |
| **Activation rate** | % reaching "aha" moment | Funnel health | > 40% | < 20% |

**Pre-PMF focus:** Weekly active %, feature adoption, day-7/14/30 retention. Retention matters more than growth before product-market fit.

---

## 5. Developer & Project Metrics (DORA + SPACE)

### DORA (2025 — now 5 metrics)

| Metric | Elite | High | Medium | Low |
|--------|-------|------|--------|-----|
| **Deployment frequency** | Multiple/day | Daily-weekly | Weekly-monthly | Monthly+ |
| **Lead time** | < 1 day | < 1 week | < 1 month | 1-6 months |
| **Change failure rate** | 5% | 10% | 15% | 46% |
| **Recovery time** | < 1 hour | < 1 day | < 1 week | > 1 week |
| **Rework rate** (new) | (benchmarks TBD) | | | |

**2025 finding:** DORA abandoned elite/high/medium/low rankings, replacing them with 7 team archetypes blending delivery + human factors.

**AI impact (DORA 2025):** 95% AI adoption. Individual: +21% tasks, +98% PRs merged. Organizational: flat delivery. Review time up 91%, PR size up 154%. AI amplifies existing capability — it doesn't create it.

### SPACE (measure >= 3 dimensions)

| Dimension | Examples |
|-----------|---------|
| **Satisfaction** | Developer satisfaction, burnout |
| **Performance** | Business impact, reliability |
| **Activity** | Commits, PRs, issues resolved |
| **Communication** | Response time, knowledge sharing |
| **Efficiency** | Cycle time, flow state, onboarding time |

### Additional developer metrics

| Metric | Good | Collect Via |
|--------|------|-------------|
| PR cycle time | < 24 hours | Git platform API |
| PR review time | < 4 hours | Git platform API |
| Test coverage | > 80% (CruxDev: 100%) | CI pipeline |
| Build time | < 10 minutes | CI platform |
| Tech debt ratio | < 20% sprint capacity | Sprint tracking |

---

## 6. User Experience Metrics (Core Web Vitals)

| Metric | What | Good | Needs Work | Poor |
|--------|------|------|------------|------|
| **LCP** | Loading | <= 2.5s | 2.5-4.0s | > 4.0s |
| **INP** | Responsiveness | <= 200ms | 200-500ms | > 500ms |
| **CLS** | Stability | <= 0.1 | 0.1-0.25 | > 0.25 |

**Supplementary:** TTFB (< 800ms), FCP (< 1.8s), TBT (< 200ms).

**RUM + Synthetic:** Use both. Synthetic catches regressions in CI. RUM captures real-world across actual devices.

**Automation:** `web-vitals` JS library for RUM. Lighthouse CI for synthetic. CrUX for field data.

---

## 7. Open Source Metrics (CHAOSS)

| Metric | Good Signal | Bad Signal |
|--------|-------------|------------|
| **Time to first response** | < 2 business days | > 1 week |
| **PR closure ratio** | Trending toward 1:1 | Growing backlog |
| **Bus factor** | >= 5 people = 50% commits | 1-2 = 50% |
| **Release frequency** | Regular, predictable | Irregular |
| **Star velocity** | Steady or accelerating | Declining |
| **Issue response time** | < 48 hours | > 1 week |

**Automation:** Fully automatable via GitHub API, CHAOSS tools.

---

## 8. Security Metrics

| Metric | Good | Bad | Collect Via |
|--------|------|-----|-------------|
| **Vuln count (critical)** | 0 in production | Any unpatched > 7 days | Snyk, Trivy, Dependabot |
| **MTTR (security)** | Critical < 24h | Critical > 72h | Issue tracking |
| **Dependency freshness** | < 1 major behind | > 2 major behind | Renovate, Dependabot |
| **SBOM coverage** | 100% | Incomplete | Syft, CycloneDX |

**Key insight:** Speed of remediation matters more than vulnerability count. You'll always have vulnerabilities; what matters is MTTR.

---

## 9. Cost Metrics (FinOps)

| Metric | Good | Bad | Collect Via |
|--------|------|-----|-------------|
| **Cost per user** | Trending down at scale | Increasing | Billing API / user count |
| **Cloud waste** | < 15% | > 30% (industry avg) | Cost tools |
| **Compute utilization** | > 60-70% | < 40% | Cloud metrics |
| **Build time** | < 10 min | > 30 min | CI platform |
| **Budget variance** | Within 5% | > 20% | Finance data |

---

## 10. Anti-Patterns

| Anti-Pattern | Problem | Fix |
|-------------|---------|-----|
| **Goodhart's Law** | Metric becomes target, gets gamed | Measure multiple dimensions (SPACE) |
| **Activity = impact** | LOC, commits reward bloat | Measure outcomes, not outputs |
| **Alert fatigue** | Thousands of unactionable alerts | Every alert must be urgent + actionable |
| **Single metrics** | One metric hides damage elsewhere | Require >= 3 dimensions |
| **Evaluating individuals** | Causes gaming, disengagement | Use metrics for systemic bottlenecks |
| **Vanity metrics** | Star count, follower count | Track velocity and engagement instead |

---

## 11. Collection Architecture (OpenTelemetry)

OpenTelemetry is the vendor-neutral standard. Four signal types:
- **Traces** (stable) — distributed request tracking
- **Metrics** (stable) — counters, gauges, histograms
- **Logs** (stable) — structured event records
- **Profiles** (experimental) — runtime performance

**Naming convention (Prometheus):**
- Lowercase with underscores: `http_request_duration_seconds`
- Include base units: `_seconds`, `_bytes`, `_total`
- Avoid high-cardinality labels (no user IDs, emails)

---

## 12. Audit Dimensions

For convergence engine integration — audit metrics against these dimensions:

1. **coverage** — Are all 8 categories represented? (infra, app, business, developer, UX, OSS, security, cost)
2. **collection** — Is collection automated? Manual metrics drift and die.
3. **actionability** — Does every metric drive a specific decision or action?
4. **thresholds** — Are good/bad thresholds defined and alertable?
5. **freshness** — Are metrics collected at appropriate frequency?
6. **anti-gaming** — Are multiple dimensions measured to prevent Goodhart effects?
7. **accessibility** — Are metrics visible to the teams that need them?

---

## References

- Google SRE Book — sre.google/sre-book/monitoring-distributed-systems/
- Brendan Gregg USE Method — brendangregg.com/usemethod.html
- DORA 2024-2025 — dora.dev/research/
- SPACE Framework — queue.acm.org/detail.cfm?id=3454124
- OpenTelemetry — opentelemetry.io/docs/
- Prometheus Naming — prometheus.io/docs/practices/naming/
- Core Web Vitals — web.dev/articles/vitals
- CHAOSS — chaoss.community/
- FinOps Foundation — finops.org/
