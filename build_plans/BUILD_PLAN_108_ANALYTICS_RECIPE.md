# BUILD_PLAN_108: Analytics Recipe — Privacy-Respecting Product Analytics

**Created:** 2026-03-28
**Status:** RESEARCH COMPLETE
**Priority:** High (fills explicit gap on cruxdev.dev/for/entrepreneurs; foundational recipe for CruxVibe)
**Dual role:** CruxDev capability (analytics integration for managed projects) AND CruxVibe recipe (creator analytics dashboard)

---

## Executive Summary

CruxDev-managed projects have no product analytics, user tracking, or conversion funnel analysis — called out explicitly on the /for/entrepreneurs page. The privacy-first analytics market has matured significantly: Plausible, PostHog, Umami, Fathom, and Pirsch all offer cookie-free, GDPR/CCPA-compliant tracking without the surveillance model of Google Analytics. Rather than building analytics from scratch, CruxDev should integrate with these tools where they excel and build only the creator-specific layer that none of them provide — subscriber lifecycle analytics, revenue attribution across recipes, and content performance dashboards for CruxVibe creators.

**Recommendation:** Integrate (not build from scratch) for web analytics and product analytics. Build a thin creator analytics layer in Elixir/Phoenix/LiveView that aggregates data from Stripe, email providers, and the integrated analytics tool into a single creator dashboard.

---

## 1. Competitive Landscape

### 1.1 Tool Comparison Matrix

| Tool | Type | Pricing (entry) | Self-hosted | Funnels | A/B Testing | Session Replay | API | Cookie-free | SDK Size |
|------|------|-----------------|-------------|---------|-------------|----------------|-----|-------------|----------|
| **Plausible** | Web analytics | $9/mo (10K pageviews) | Yes (AGPL, free) | Cloud only | No | No | Yes | Yes | ~1 KB |
| **PostHog** | Product analytics | Free (1M events/mo) | Yes (MIT core) | Yes | Yes (paid) | Yes (paid) | Yes | Configurable | ~52 KB |
| **Fathom** | Web analytics | $15/mo (100K pageviews) | No (Lite abandoned) | No | No | No | Yes | Yes | ~2 KB |
| **Umami** | Web analytics | Free cloud (1M events/mo) | Yes (MIT, free) | No | No | No | Yes | Yes | ~2 KB |
| **Pirsch** | Web analytics | $6/mo (10K pageviews) | Yes (paid) | No | No | No | Yes | Yes | 0 KB (server-side) |
| **OpenPanel** | Product analytics | Free (10K events/mo) | Yes (single Docker) | Yes | No | No | Yes | Yes | ~2.3 KB |

### 1.2 Detailed Assessment

#### Plausible Analytics
- **Strengths:** Simplest dashboard in the category. Single-page view shows everything. Revenue tracking, UTM campaigns, scroll depth, Google Search Console integration. EU-hosted (Germany). Conversion funnels on cloud plans. Automatic channel grouping for traffic sources.
- **Weaknesses:** Self-hosted Community Edition excludes funnels and ecommerce tracking (deliberately gated to protect cloud business). No product analytics (user journeys, retention cohorts, feature usage). No A/B testing. No session replay.
- **Best for:** Content sites, marketing pages, blogs — anywhere simple pageview/referrer/conversion tracking suffices.
- **Pricing at scale:** ~$9/mo (10K), ~$19/mo (100K), ~$69/mo (1M). Predictable, pageview-based.

#### PostHog
- **Strengths:** The most complete product analytics platform in the privacy-respecting space. Product analytics, web analytics, session replay, feature flags, A/B testing, surveys, CDP, data warehouse, and AI assistant — all in one. 1M free events/month covers 90%+ of companies. Unlimited seats on all plans.
- **Weaknesses:** Heavy — requires ClickHouse, Kafka, Redis, PostgreSQL for self-hosting. 52KB SDK is 25-50x larger than alternatives. Complexity is overkill for simple sites. A/B testing and session replay require paid plan. Cookie-based by default (must configure cookieless mode).
- **Best for:** SaaS products, apps with user accounts, teams that need funnels + retention + feature flags. The "all-in-one product OS" positioning.
- **Pricing at scale:** Free up to 1M events. ~$50/1M events after. Volume discounts at scale.

#### Fathom Analytics
- **Strengths:** Clean, sustainable business. Same features on every plan. 50 sites included. Forever data retention. EU isolation option. Custom events and ecommerce tracking. Excellent uptime track record.
- **Weaknesses:** No self-hosted option (Fathom Lite was abandoned). No funnels, no A/B testing, no session replay. Higher entry price ($15/mo) for less capability than Plausible ($9/mo with funnels).
- **Best for:** Businesses that want "set it and forget it" analytics with no self-hosting burden and strong privacy guarantees.
- **Pricing at scale:** $15/mo (100K), $25/mo (200K), $45/mo (500K). Simple but more expensive per pageview than Plausible.

#### Umami
- **Strengths:** Fully open source (MIT). Easiest self-hosting — single Docker container, minimal resources. Free cloud tier (1M events/month). Clean UI. Custom event tracking. Full REST API.
- **Weaknesses:** No funnels. No A/B testing. No session replay. Less mature than Plausible (fewer integrations, smaller ecosystem). Cloud pricing unclear beyond free tier.
- **Best for:** Developers who want to self-host with minimal overhead. Budget-conscious teams. Privacy-first sites that only need basic analytics.
- **Pricing:** Self-hosted = free. Cloud free tier = 1M events/mo. Pro ~$20/mo.

#### Pirsch
- **Strengths:** Server-side only (zero JavaScript — genuinely zero KB client payload). Built and hosted in Germany. GDPR/CCPA/Schrems II compliant. Conversion goals. Dashboard sharing. Cheapest entry ($6/mo).
- **Weaknesses:** Server-side means no client-side event tracking without extra work. No funnels. No A/B testing. No session replay. Smaller community. Self-hosted requires paid license.
- **Best for:** Sites where zero-JavaScript tracking is a hard requirement. EU-based businesses with strict compliance needs.
- **Pricing at scale:** $6/mo (10K), $10/mo (100K). Cheapest at every tier.

#### OpenPanel (notable newcomer)
- **Strengths:** Combines web analytics AND product analytics. 2.3KB SDK. Cookie-free by default. Single Docker container for self-hosting. Funnels, retention, user journeys included at every tier. Positions as "PostHog without the complexity."
- **Weaknesses:** Newer, smaller community. No A/B testing. No session replay. 10K free events/mo (vs PostHog's 1M). Less battle-tested at scale.
- **Best for:** Teams that want product analytics depth without PostHog's operational complexity.

### 1.3 Competitive Positioning Map

```
                    Simple ←————————————————→ Complex
                        |                        |
    Privacy-first       |  Pirsch   Plausible    |  PostHog
    (cookie-free)       |  Umami    Fathom       |  OpenPanel
                        |                        |
    Configurable        |                        |  Mixpanel
    (can be private)    |                        |  Amplitude
                        |                        |
    Surveillance        |                        |  Google Analytics
    (cookies + PII)     |                        |  Hotjar
                        |                        |
                    Web analytics            Product analytics
```

---

## 2. Gap Assessment

### 2.1 What CruxDev-Managed Projects Need

| Need | Who | Existing Tools Cover It? | Gap? |
|------|-----|--------------------------|------|
| Pageviews, referrers, UTMs | All projects | Yes — Plausible, Umami, Fathom, Pirsch | No |
| Conversion funnels | SaaS, ecommerce | Partially — PostHog (paid), Plausible Cloud, OpenPanel | Small |
| User journeys | SaaS with accounts | PostHog, OpenPanel | No |
| A/B testing | SaaS, landing pages | PostHog (paid) only | Medium |
| Session replay | Debugging, UX research | PostHog (paid) only | Medium |
| Revenue attribution | All monetized projects | None of them | **Yes — big gap** |
| Subscriber lifecycle | Creators (CruxVibe) | None of them | **Yes — big gap** |
| Content performance | Creators (CruxVibe) | None of them | **Yes — big gap** |
| Cross-recipe analytics | CruxVibe creators | None of them — they don't know what recipes are | **Yes — unique to us** |
| SEO + analytics correlation | All websites | None integrate SEO health with analytics | **Yes — CruxDev differentiator** |

### 2.2 The Build vs. Integrate Decision

**Integrate (do not build):**
- Pageview tracking, visitor counting, device/browser/location breakdowns
- UTM campaign tracking, referrer analysis
- Basic conversion funnels
- Session replay (if needed)
- Feature flags and A/B testing

These are commodity capabilities. Every tool above does them well. Building our own would take 6-12 months and produce something worse than Plausible's 5-year-old codebase.

**Build (unique to CruxDev/CruxVibe):**
- Creator analytics dashboard (subscriber growth, revenue by recipe, content performance)
- Revenue attribution across Stripe + analytics events (which page led to which purchase)
- Cross-recipe analytics (how newsletter subscribers convert to course students)
- SEO health correlation (GSC data + analytics data in one view)
- Churn prediction and subscriber health scoring
- Content ROI calculator (time invested in content vs. revenue generated)

This is the layer that no analytics tool provides because they don't understand the creator business model. This is where we build.

---

## 3. Recommended Architecture

### 3.1 Integration Layer (CruxDev capability)

CruxDev should offer first-class integration with privacy-respecting analytics as part of project adoption. The adoption process should include analytics setup.

**Recommended default stack:**
- **Plausible** for web analytics (simple sites, blogs, marketing pages)
- **PostHog** for product analytics (SaaS, apps with user accounts)
- **Choice is per-project** — CruxDev classifies and recommends during adoption

**Integration approach:**
1. Adoption classifier detects project type (content site vs. SaaS vs. creator platform)
2. CruxDev generates analytics config (tracking snippet, event definitions, conversion goals)
3. Analytics recipe installs the tracking code, configures events, and sets up dashboards
4. Growth engine correlates SEO data with analytics data in health checks

### 3.2 Creator Analytics Layer (CruxVibe recipe)

This is the custom-built component. An Elixir/Phoenix/LiveView application that aggregates data from multiple sources into a single creator dashboard.

```
Creator Analytics Recipe (Elixir/Phoenix/LiveView)
├── Data Ingestion
│   ├── Stripe webhooks → revenue events (subscriptions, one-time, refunds)
│   ├── Analytics API → pageviews, referrers, conversions (Plausible/Umami API)
│   ├── Email provider → subscriber events (signups, opens, clicks, unsubs)
│   ├── Recipe events → content published, courses completed, chapters read
│   └── Oban workers → scheduled data pulls, aggregation jobs
│
├── Analytics Engine
│   ├── Revenue attribution (which traffic source → which purchase)
│   ├── Subscriber lifecycle (free → trial → paid → churned, with timestamps)
│   ├── Content performance (views → engagement → conversion per piece)
│   ├── Cross-recipe flows (newsletter reader → course student → subscriber)
│   ├── Cohort analysis (when did subscribers join, how do cohorts behave)
│   └── Churn prediction (engagement decay → risk scoring)
│
├── LiveView Dashboard
│   ├── Hero KPI: Monthly Recurring Revenue (OMTM per DASHBOARD_PATTERNS.md)
│   ├── Revenue panel: MRR, ARR, net revenue, refunds, growth rate
│   ├── Subscriber panel: total, new, churned, net growth, LTV
│   ├── Content panel: top content by views, by conversions, by revenue
│   ├── Funnel panel: visitor → free signup → paid conversion
│   ├── Traffic panel: top sources, UTM performance, organic vs. paid
│   └── Alerts: churn spike, revenue drop, content going viral
│
└── API
    ├── REST API for programmatic access (JSON)
    ├── Webhook notifications (revenue milestones, churn alerts)
    └── CSV/PDF export for tax reporting
```

### 3.3 Data Model

```elixir
# Core schemas (Ash resources)

# Raw events from all sources
defmodule CruxVibe.Analytics.Event do
  # id, tenant_id, source (stripe|analytics|email|recipe),
  # event_type, properties (jsonb), occurred_at, inserted_at
end

# Aggregated daily metrics per creator
defmodule CruxVibe.Analytics.DailyMetric do
  # id, tenant_id, date, metric_type, value, dimensions (jsonb)
  # metric_type: :pageviews, :unique_visitors, :signups,
  #              :paid_conversions, :revenue, :churn, :email_opens
end

# Subscriber lifecycle tracking
defmodule CruxVibe.Analytics.SubscriberEvent do
  # id, tenant_id, subscriber_id, event_type
  # event_type: :signed_up, :activated, :subscribed, :upgraded,
  #             :downgraded, :churned, :reactivated
  # metadata (jsonb), occurred_at
end

# Content performance
defmodule CruxVibe.Analytics.ContentMetric do
  # id, tenant_id, content_id, content_type (chapter|course|podcast|post),
  # date, views, unique_views, avg_read_time, completions,
  # conversions, revenue_attributed
end

# Revenue attribution
defmodule CruxVibe.Analytics.Attribution do
  # id, tenant_id, purchase_id, traffic_source, utm_campaign,
  # utm_medium, utm_source, landing_page, referrer,
  # amount_cents, currency, attributed_at
end
```

### 3.4 Technology Choices

| Component | Technology | Why |
|-----------|-----------|-----|
| Dashboard | Phoenix LiveView | Real-time updates, no JS framework needed, matches CruxVibe stack |
| Charts | VegaLite (via LiveView) or Chart.js hooks | VegaLite has Elixir bindings; Chart.js is proven |
| Data store | PostgreSQL | Already in the stack. TimescaleDB extension for time-series if needed |
| Background jobs | Oban | Already in the CruxVibe stack. Handles scheduled data pulls, aggregation |
| External analytics | Plausible API / Umami API | Pull pageview data into creator dashboard |
| Revenue data | Stripe webhooks + API | Already integrated via Stripe Connect recipe |
| Email data | Provider webhooks (SES/Postmark) | Already integrated via Email recipe |
| Caching | ETS / Cachex | Dashboard queries can be expensive; cache with 5-min TTL |

---

## 4. Self-Hosted vs. SaaS Tradeoffs

### For CruxDev-Managed Projects (the analytics tool itself)

| Factor | Self-Hosted | SaaS (Cloud) |
|--------|-------------|---------------|
| **Cost at small scale** | $5-10/mo (VPS) | $6-15/mo |
| **Cost at large scale** | $20-50/mo (VPS) | $50-200/mo |
| **Data ownership** | Full — data stays on your infra | Third-party servers (EU options available) |
| **Maintenance** | You handle updates, backups, uptime | Provider handles everything |
| **Compliance** | Maximum control for regulated industries | Depends on provider's DPA |
| **Feature gating** | Plausible CE excludes funnels; PostHog CE excludes A/B | All features on paid plans |
| **Setup time** | 1-4 hours with Docker | 5 minutes |

**Recommendation by project type:**
- **Solo creator (CruxVibe):** SaaS — creators should not manage analytics infrastructure. Use Plausible Cloud or Umami Cloud.
- **Startup/SaaS:** PostHog Cloud free tier, upgrade when needed. Self-host only if data sovereignty is a hard requirement.
- **Enterprise:** Self-hosted PostHog or Plausible behind their firewall.
- **CruxDev itself:** Self-hosted Umami or Plausible CE (we are developers, we can manage it, and it dogfoods the pattern).

### For the Creator Analytics Layer (what we build)

This runs as part of the CruxVibe platform — it is not a separate deployment. Each creator's analytics dashboard is a LiveView within their CruxVibe-hosted site. Data is stored in their tenant's PostgreSQL schema. No separate analytics infrastructure to manage.

---

## 5. CruxVibe Creator Analytics: What Creators Need

### 5.1 By Creator Type

| Creator Type | Primary Metrics | Secondary Metrics | Dashboard Focus |
|-------------|----------------|-------------------|-----------------|
| **Authors** | Subscriber count, MRR, chapter reads | Read completion rate, free-to-paid conversion, per-chapter retention | Content performance — which chapters retain, which convert |
| **Course creators** | Enrollment count, completion rate, revenue | Student progress, lesson drop-off, certification rate | Funnel: landing → enroll → complete → review |
| **Newsletter writers** | Subscriber count, open rate, paid conversion | Click rate, growth rate, churn rate, revenue per subscriber | Growth curve + engagement health |
| **Podcasters** | Downloads, subscriber count, premium conversion | Listener retention, episode completion, ad revenue | Content performance — which episodes drive growth |
| **Coaches** | Client count, session revenue, LTV | Booking rate, cancellation rate, referral rate | Revenue pipeline — leads → booked → recurring |

### 5.2 Universal Creator Metrics (every CruxVibe dashboard)

1. **Monthly Recurring Revenue (MRR)** — the hero KPI
2. **Net subscriber growth** — new minus churned this period
3. **Lifetime value (LTV)** — average revenue per subscriber over their lifetime
4. **Churn rate** — % of subscribers lost per month
5. **Conversion rate** — free visitors → paid subscribers
6. **Top content** — which pieces drive the most conversions (not just views)
7. **Traffic sources** — where paying subscribers come from
8. **Revenue by recipe** — how much from subscriptions vs. courses vs. books vs. storefront

### 5.3 Alerts Creators Need

| Alert | Trigger | Action |
|-------|---------|--------|
| Revenue milestone | MRR crosses $1K, $5K, $10K, $50K, $100K | Celebrate + suggest next growth lever |
| Churn spike | Churn rate > 2x 30-day average | Investigate — content gap? billing issue? |
| Content going viral | Pageviews > 10x average for a piece | Promote it — add CTA, share on social, make it a lead magnet |
| Subscriber milestone | 100, 500, 1K, 5K, 10K, 50K subscribers | Celebrate + suggest tier/pricing adjustment |
| Revenue decline | MRR drops > 10% month-over-month | Root cause analysis — churn? fewer new? lower ARPU? |
| Failed payments | > 5% of subscription renewals failing | Dunning email sequence, payment method update prompt |

---

## 6. Implementation Phases

### Phase 1: CruxDev Analytics Integration (2-3 days)

**Goal:** Every CruxDev-managed project can have privacy-respecting analytics installed during adoption.

- [ ] 1.1 Add analytics classification to adoption classifier (content site → Plausible, SaaS → PostHog, creator → CruxVibe recipe)
- [ ] 1.2 Analytics integration template: tracking snippet installation, event definitions, conversion goals
- [ ] 1.3 Growth engine integration: correlate GSC data with analytics API data in SEO health checks
- [ ] 1.4 Documentation: ANALYTICS_PATTERNS.md covering tool selection, integration, event naming conventions
- [ ] 1.5 Update /for/entrepreneurs page: remove "No product analytics" gap — replaced with "Privacy-first analytics, integrated during adoption"

**Tests:** Integration template generates valid configs for Plausible, PostHog, and Umami. Growth engine can pull from analytics APIs.

### Phase 2: Creator Analytics Data Layer (1 week)

**Goal:** CruxVibe can ingest and store analytics events from all recipe sources.

- [ ] 2.1 Ash resources: Event, DailyMetric, SubscriberEvent, ContentMetric, Attribution
- [ ] 2.2 Stripe webhook handler: revenue events → Event + Attribution tables
- [ ] 2.3 Email webhook handler: subscriber events → SubscriberEvent table
- [ ] 2.4 Analytics API poller (Oban worker): pull pageview data from Plausible/Umami API → DailyMetric
- [ ] 2.5 Recipe event hooks: content published/read/completed → ContentMetric
- [ ] 2.6 Daily aggregation worker (Oban cron): roll up raw events into DailyMetric
- [ ] 2.7 Subscriber lifecycle state machine: track transitions (free → paid → churned → reactivated)

**Tests:** Event ingestion from all sources. Aggregation produces correct daily metrics. Subscriber state machine handles all valid transitions. Attribution links purchases to traffic sources.

### Phase 3: Creator Dashboard v1 (1 week)

**Goal:** Creators see a single LiveView dashboard with all their key metrics.

- [ ] 3.1 Dashboard layout per DASHBOARD_PATTERNS.md (bento grid, F-pattern, hero KPI)
- [ ] 3.2 Hero KPI: MRR with sparkline trend
- [ ] 3.3 Revenue panel: MRR, net revenue, refunds, growth rate, revenue by recipe
- [ ] 3.4 Subscriber panel: total, new, churned, net growth, LTV
- [ ] 3.5 Content panel: top content by conversions (not just views)
- [ ] 3.6 Traffic panel: top sources, UTM breakdown (pulled from analytics API)
- [ ] 3.7 Date range selector: 7d, 30d, 90d, 12mo, all time
- [ ] 3.8 Real-time updates via LiveView (new subscriber, new purchase → dashboard updates)
- [ ] 3.9 Mobile-responsive layout (creators check stats on phone)

**Tests:** Dashboard renders with test data. All panels display correct aggregations. Date range filtering works. LiveView updates propagate.

### Phase 4: Funnels + Attribution (3-5 days)

**Goal:** Creators understand their conversion funnel and what drives revenue.

- [ ] 4.1 Funnel builder: define steps (visit landing → view pricing → start checkout → complete purchase)
- [ ] 4.2 Funnel visualization: step-by-step conversion rates with drop-off percentages
- [ ] 4.3 Revenue attribution: which traffic sources produce paying subscribers (not just visitors)
- [ ] 4.4 Content attribution: which pieces of content drive the most conversions
- [ ] 4.5 Cross-recipe flow: newsletter subscriber → course student → subscriber (recipe-to-recipe conversion)
- [ ] 4.6 UTM tracking integration: creators can tag their social/email links and see which campaigns convert

**Tests:** Funnel calculations match expected conversion rates. Attribution correctly links traffic sources to purchases. Cross-recipe flows track multi-step journeys.

### Phase 5: Alerts + Insights (2-3 days)

**Goal:** Creators get proactive notifications instead of checking dashboards.

- [ ] 5.1 Alert engine: configurable thresholds per creator
- [ ] 5.2 Revenue milestone alerts (MRR crosses thresholds)
- [ ] 5.3 Churn spike detection (statistical anomaly, not just threshold)
- [ ] 5.4 Content virality detection (traffic surge on specific content)
- [ ] 5.5 Failed payment alerting with suggested dunning actions
- [ ] 5.6 Weekly digest email: key metrics, trends, suggested actions
- [ ] 5.7 Notification delivery: in-app (LiveView), email, webhook

**Tests:** Alerts fire at correct thresholds. Churn detection identifies statistical anomalies. Weekly digest generates correctly.

### Phase 6: API + Export (2-3 days)

**Goal:** Creators can access their data programmatically and export for tax/reporting.

- [ ] 6.1 REST API: all metrics accessible via authenticated JSON API
- [ ] 6.2 CSV export: revenue reports, subscriber lists, content performance
- [ ] 6.3 PDF export: monthly revenue summary for bookkeeping
- [ ] 6.4 Webhook notifications: configurable events pushed to creator's endpoint
- [ ] 6.5 API documentation and rate limiting

**Tests:** API returns correct data for all endpoints. CSV/PDF exports match dashboard data. Webhooks deliver reliably.

---

## 7. What We Explicitly Do NOT Build

| Capability | Why Not | Use Instead |
|-----------|---------|-------------|
| Pageview tracking | Commodity, solved by 5+ tools | Plausible, Umami, or PostHog |
| Session replay | Complex, PostHog does it well | PostHog |
| A/B testing framework | Complex, PostHog does it well | PostHog |
| Feature flags | Complex, PostHog does it well | PostHog |
| Bot detection / fingerprinting | Deep specialty, not our domain | Integrated tool handles it |
| Real-time visitor map | Vanity metric, low value | Plausible has it if needed |
| Heatmaps | Specialty UX tool | PostHog or Hotjar |

---

## 8. Pricing Impact

### CruxDev
Analytics integration becomes part of standard adoption — no separate pricing. It's a feature that closes a documented gap and makes the /for/entrepreneurs page stronger.

### CruxVibe
Creator analytics dashboard is included in Creator ($99/mo) and Pro ($199/mo) tiers per BUILD_PLAN_103. The Starter tier ($49/mo) gets basic metrics (MRR, subscriber count). Full dashboard with funnels, attribution, and alerts requires Creator or Pro.

| Feature | Starter ($49/mo) | Creator ($99/mo) | Pro ($199/mo) |
|---------|------------------|-------------------|---------------|
| MRR + subscriber count | Yes | Yes | Yes |
| Content performance | Top 5 only | Full | Full |
| Traffic sources | Basic | Full with UTM | Full with UTM |
| Funnels | No | Yes | Yes |
| Revenue attribution | No | Yes | Yes |
| Cross-recipe analytics | No | No | Yes |
| Alerts | Revenue milestones only | All alerts | All alerts + custom |
| API access | No | Read-only | Full read/write |
| Export | No | CSV | CSV + PDF + API |

---

## 9. Success Metrics

| Metric | Target | Measured By |
|--------|--------|-------------|
| Gap closed | /for/entrepreneurs no longer lists analytics as a gap | Website audit |
| Adoption integration | 100% of new CruxDev adoptions include analytics setup | Adoption checklist |
| Creator dashboard load time | < 500ms p95 | LiveView telemetry |
| Data freshness | Revenue events < 30s, pageview data < 5min | Oban job monitoring |
| Creator engagement | > 60% of creators check dashboard weekly | Dashboard access logs |
| Churn alert accuracy | > 80% of churn alerts precede actual churn by 7+ days | Alert → churn correlation |

---

## 10. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Analytics API rate limits | Medium | Medium | Cache aggressively, pull data on schedule (not real-time), respect rate limits |
| Plausible/Umami API changes | Low | Medium | Abstract behind integration interface, swap providers without dashboard changes |
| Dashboard performance at scale | Medium | High | PostgreSQL materialized views for heavy aggregations, ETS caching, TimescaleDB if needed |
| Creator data privacy | Low | Critical | All data is per-tenant (RLS), no cross-creator analytics, creators own their data |
| Stripe webhook reliability | Low | High | Idempotent processing, webhook retry handling, reconciliation job |

---

## 11. Dependencies

| Dependency | Status | Needed By |
|-----------|--------|-----------|
| CruxVibe Phoenix project (BP103 Phase 0) | IN PROGRESS | Phase 2 |
| Stripe Connect integration (BP103 Phase 1) | IN PROGRESS | Phase 2 |
| Auth recipe (BP103) | IN PROGRESS | Phase 3 |
| Email recipe (BP103) | Planned | Phase 2 |
| DASHBOARD_PATTERNS.md | COMPLETE | Phase 3 |
| METRICS_PATTERNS.md | COMPLETE | Phase 3 |

---

## 12. Verification

```bash
# CruxDev integration tests
cd cruxdev && cargo test analytics

# CruxVibe recipe tests
cd cruxvibe && mix test test/analytics/ --trace

# Dashboard performance
cd cruxvibe && mix test test/analytics/dashboard_performance_test.exs

# Data aggregation accuracy
cd cruxvibe && mix test test/analytics/aggregation_test.exs
```

**Convergence criteria:** Two consecutive independent clean passes of all analytics tests. Dashboard renders all panels with correct data. Integration templates generate valid configs for Plausible, PostHog, and Umami.
