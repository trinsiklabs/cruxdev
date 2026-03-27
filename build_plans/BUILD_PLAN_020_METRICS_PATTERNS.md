# BUILD_PLAN_020: METRICS_PATTERNS.md + Metrics Audit Integration

**Status:** CONVERGED
**Priority:** High

## What Was Done

1. 5-pass research on application/project metrics (40+ sources: Google SRE, DORA 2024-2025, USE Method, RED Method, SPACE, CHAOSS, FinOps, OpenTelemetry, Core Web Vitals)
2. Produced `docs/METRICS_PATTERNS.md` — 12 sections covering 8 metric categories with thresholds, collection methods, and anti-patterns
3. Added `METRICS_DIMENSIONS` constant (7 dimensions) to convergence engine
4. Companion to DASHBOARD_PATTERNS.md (BP021)

## Metric Categories Covered

1. Infrastructure (USE Method)
2. Application (Golden Signals + RED)
3. Business (SaaS metrics)
4. Developer/Project (DORA + SPACE)
5. User Experience (Core Web Vitals)
6. Open Source (CHAOSS)
7. Security
8. Cost (FinOps)

## Convergence Integration

- `METRICS_DIMENSIONS` added to `router.rs`: coverage, collection, actionability, thresholds, freshness, anti_gaming, accessibility
- Available for audit when projects include metrics/monitoring infrastructure
