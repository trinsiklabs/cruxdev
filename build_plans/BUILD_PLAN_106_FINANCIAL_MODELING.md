# BUILD_PLAN_106: Financial Modeling Recipe — Entrepreneur Vertical

**Created:** 2026-03-28
**Status:** RESEARCH COMPLETE
**Priority:** High (VERTICAL GAP — listed gap on /for/entrepreneurs page)
**Goal:** Determine whether to build a Financial Modeling Recipe or integrate with existing tools, and provide implementation plan for the chosen path.

---

## Executive Summary

Entrepreneurs and startup founders currently juggle 3-8 disconnected financial tools for revenue projections, runway calculations, cap tables, and investor-ready reports. The market is fragmented: no single tool covers the full startup financial lifecycle from napkin math to Series A pitch deck financials. Existing tools range from $15/month (LivePlan, basic) to $250+/month (Causal, advanced). Most are spreadsheet-adjacent and none integrate with the kind of autonomous project convergence CruxDev provides.

**Recommendation:** BUILD a Financial Modeling Recipe in Phoenix/Elixir. The opportunity is not to compete with Causal on enterprise FP&A, but to provide startup founders with AI-generated financial models as a natural output of CruxDev project convergence. When CruxBot converges a project, it already understands the tech stack, team size, and business model — generating financial projections from that context is a unique capability no competitor offers.

---

## Competitive Landscape

### Tier 1: Startup-Focused Financial Modeling

| Tool | What It Does | Pricing | Strengths | Weaknesses |
|------|-------------|---------|-----------|------------|
| **Causal** | Multi-dimensional financial modeling, scenario analysis, sensitivity testing, collaborative dashboards | Free tier; Pro $250/mo; Business/Enterprise custom | Best-in-class formula language; multi-dimensional data; real-time collaboration; strong visualization | Expensive; overkill for early-stage; acquired by Lucanet (enterprise pivot); steep learning curve |
| **Finmark** | Revenue modeling, cash flow forecasting, hiring plans, integration with QuickBooks/Xero/Stripe | Started at $49/mo (tiered by revenue) | Y Combinator backed; startup-native UX; real-time actuals sync; investor-ready reports | **SUNSET** — acquired by BILL, original product ending April 2026. Market gap opening |
| **LivePlan** | Business plan writing, auto-financials, 500+ sample plans, performance tracking | Standard $20/mo; Premium $40/mo (annual discounts available) | Cheapest option; 500+ templates; AI benchmarks; QuickBooks/Xero integration; good for SBA loans | Focused on traditional business plans, not startup models; limited scenario analysis; not investor-grade for tech startups |
| **ProjectionHub** | Industry-specific financial projection templates (100+), CPA-reviewed, Excel export | Template-based pricing (per-template purchase model) | CPA-developed templates; covers niche industries; expert review available; good for loan applications | Template-only (not a modeling platform); no real-time collaboration; no integrations; Excel-centric |
| **Brixx** | 10-year financial forecasting, scenario planning, Xero sync, auto-generated reports | $42/mo ($33.60/mo annual); extra plans $7/mo each | Long forecast window (10 years); Xero integration; reforecasting from actuals; industry templates | UK-focused; limited integrations beyond Xero; no startup-specific features; no cap table |
| **PlanGuru** | Budgeting, forecasting, 20+ forecasting methods, 10-year projections | $99/mo; additional users $25-29/mo | Deep forecasting methods; balance sheet + P&L + cash flow; QuickBooks/Xero/Excel integration | SMB/nonprofit focused; dated UI; no startup-specific features; no cap table; no AI |

### Tier 2: Adjacent Tools (Cap Tables, Runway)

| Tool | What It Does | Pricing | Notes |
|------|-------------|---------|-------|
| **Carta** | Cap table management, 409A valuations, equity plans | $1,000+/yr | Industry standard but expensive; does not do revenue modeling |
| **Pulley** | Cap table for startups | ~$43/mo starter | Simpler than Carta; no financial modeling |
| **Cake Equity** | Cap table + scenario modeling | Tiered pricing | Good for early-stage; limited to equity, not full financials |
| **Foresight** | Runway budgeting + cash forecasting | Spreadsheet templates | Template-based; not a platform |
| **Puzzle** | Accounting + runway dashboards | Free tier available | Accounting-first, not modeling-first |

### Tier 3: AI-Native Entrants (2025-2026)

| Tool | What It Does | Notes |
|------|-------------|-------|
| **Vena Copilot** | AI in Excel for FP&A, natural language queries | Enterprise-focused, not startup |
| **ForecastMaster Pro** | AI cash flow projections (93% accuracy claim for 90-day) | Emerging; limited track record |
| **GrowthPlan AI** | Multi-year roadmaps from data sources | Emerging; startup-focused |
| **Datarails** | AI financial modeling on top of Excel | Mid-market; $1,000+/mo |

---

## Gap Assessment

### What Exists (Well-Served)

1. **Enterprise FP&A** — Causal, Datarails, Vena handle this well at $250+/mo
2. **Traditional business plans** — LivePlan dominates at $20-40/mo with 500+ templates
3. **Cap table management** — Carta and Pulley own this space
4. **Basic budgeting** — PlanGuru and Brixx cover SMB forecasting

### What Is Poorly Served

1. **Startup-native financial modeling** — Finmark was the best option and is being **sunset**. This creates a significant market gap in 2026.
2. **AI-generated projections from project context** — No tool generates financial models from knowledge of the actual project being built. Every tool starts from a blank spreadsheet or template.
3. **Integrated founder workflow** — Founders build a product, then context-switch to a completely separate tool to model finances. No connection between "what I'm building" and "what it will cost/earn."
4. **Revenue model validation** — Tools let you input assumptions but none validate them against market data or comparable startups.
5. **Convergence between plan and execution** — Business plans are static documents. No tool continuously updates projections as the actual project evolves.

### The CruxDev Unique Angle

CruxDev already knows, for every project it manages:
- The tech stack and architecture complexity
- The team size and composition (from project config)
- The business model type (SaaS, marketplace, e-commerce, etc.)
- The feature roadmap and build timeline
- The competitive landscape (from COMPETITORS.md)

**No financial modeling tool has this context.** They all start from zero. A CruxDev Financial Recipe starts from everything CruxDev already knows and generates projections that are grounded in the actual project reality.

---

## Build vs. Integrate Decision

### Option A: Integrate with Existing Tool

| Pro | Con |
|-----|-----|
| Faster to market | Best option (Finmark) is being sunset |
| Less engineering | Remaining tools are template-based or enterprise-priced |
| Proven financial logic | No tool accepts project context as input |
| | Dependency on third-party pricing/availability |
| | No differentiation — just another integration |

### Option B: Build a Financial Recipe

| Pro | Con |
|-----|-----|
| Unique value prop: context-aware financial models | Significant engineering effort |
| Natural extension of CruxDev convergence | Financial logic must be correct (regulatory risk if used for fundraising) |
| Revenue opportunity as CruxVibe recipe | Needs accounting review/validation |
| Finmark sunset creates market timing | Crowded adjacent market |
| Full control over UX and data | |
| Elixir/Phoenix ideal for real-time dashboards | |

### Verdict: BUILD

The Finmark sunset in April 2026 creates a market gap at exactly the right time. The unique differentiator — generating financial models from project context — is impossible to achieve through integration. The recipe model means this is a revenue-generating module, not just a feature.

**Scope control:** Build for startup founders doing seed/Series A planning, NOT enterprise FP&A. Stay in the $49-99/mo range. Let Causal and Datarails own the $250+/mo enterprise tier.

---

## Architecture: Financial Modeling Recipe (Phoenix/Elixir)

### Stack

```
Financial Recipe (Phoenix/Elixir + Ash Framework)
├── Core Engine
│   ├── Projection Engine (Elixir GenServer — time-series calculations)
│   ├── Scenario Manager (branching financial models, compare scenarios)
│   ├── Double-Entry Ledger (Fuentes library or custom Ecto schema)
│   └── Formula Engine (custom DSL for financial formulas, not Excel)
├── AI Layer
│   ├── Context Extractor (reads CruxDev project state → financial assumptions)
│   ├── Revenue Model Generator (LLM generates revenue model from business type)
│   ├── Assumption Validator (LLM validates assumptions against market data)
│   └── Narrative Generator (LLM writes investor-ready financial narrative)
├── Data Models (Ash Resources)
│   ├── Company (entity, stage, industry, location)
│   ├── RevenueStream (type, pricing, growth assumptions)
│   ├── CostCenter (personnel, infrastructure, marketing, etc.)
│   ├── FundingRound (type, amount, valuation, dilution)
│   ├── Scenario (named branching point with variable overrides)
│   ├── Projection (time-series output: P&L, cash flow, balance sheet)
│   └── CapTable (shareholders, shares, options, SAFEs, convertible notes)
├── Integrations
│   ├── Stripe (actual revenue data sync)
│   ├── QuickBooks/Xero (accounting actuals import)
│   ├── CruxDev Engine (project context: stack, team, roadmap, competitors)
│   └── Export (PDF investor deck, Excel, CSV)
├── LiveView UI
│   ├── Dashboard (KPIs: runway, burn rate, MRR, growth rate)
│   ├── Model Builder (visual revenue/cost model editor)
│   ├── Scenario Comparison (side-by-side scenario overlay)
│   ├── Cap Table Visualizer (dilution waterfall, ownership pie)
│   ├── Investor Report Generator (one-click PDF/deck export)
│   └── Real-Time Collaboration (Phoenix Presence + CRDT)
└── CruxVibe Integration
    ├── Recipe Registry entry (snap into any CruxVibe site)
    ├── Entrepreneur Dashboard (public or gated financial overview)
    └── Billing (included in CruxVibe $100/mo or standalone $49-99/mo)
```

### Why Elixir Is Ideal Here

1. **GenServer for projection calculations** — Each financial model is a supervised process. Time-series calculations, scenario branching, and sensitivity analysis map naturally to GenServer state management. No need for separate calculation service.

2. **LiveView for real-time dashboards** — Financial dashboards that update as you change assumptions, with zero JavaScript framework overhead. Drag a revenue growth slider, see P&L update instantly via WebSocket.

3. **Phoenix Presence for collaboration** — Multiple founders editing the same model, seeing each other's cursors and changes in real-time. Built into Phoenix, no third-party dependency.

4. **Ash Framework for data modeling** — Financial entities (companies, revenue streams, cost centers, funding rounds) are naturally resource-oriented. Ash's authorization, calculation, and API generation fit perfectly.

5. **Broadway for data sync pipelines** — Stripe webhook processing, QuickBooks/Xero sync, and CruxDev context extraction are all streaming data problems. Broadway handles backpressure and fault tolerance.

### Data Model Detail

```elixir
# Core financial projection time-series
defmodule FinancialRecipe.Projection do
  use Ash.Resource

  attributes do
    uuid_primary_key :id
    attribute :scenario_id, :uuid
    attribute :period, :date          # Monthly granularity
    attribute :revenue, :decimal      # Total revenue
    attribute :cogs, :decimal         # Cost of goods sold
    attribute :gross_margin, :decimal # Calculated
    attribute :opex, :decimal         # Operating expenses
    attribute :ebitda, :decimal       # Calculated
    attribute :net_income, :decimal   # After tax
    attribute :cash_balance, :decimal # Running balance
    attribute :burn_rate, :decimal    # Monthly burn
    attribute :runway_months, :integer # Calculated from cash/burn
    attribute :headcount, :integer
    attribute :mrr, :decimal          # Monthly recurring revenue
    attribute :arr, :decimal          # Annual recurring revenue
    attribute :ltv, :decimal          # Customer lifetime value
    attribute :cac, :decimal          # Customer acquisition cost
    attribute :churn_rate, :decimal
  end
end

# Cap table entry
defmodule FinancialRecipe.CapTableEntry do
  use Ash.Resource

  attributes do
    uuid_primary_key :id
    attribute :company_id, :uuid
    attribute :holder_name, :string
    attribute :holder_type, :atom  # :founder, :investor, :employee, :advisor
    attribute :instrument, :atom   # :common, :preferred, :option, :safe, :convertible_note
    attribute :shares, :integer
    attribute :price_per_share, :decimal
    attribute :vesting_schedule, :map  # cliff, duration, acceleration
    attribute :valuation_cap, :decimal  # For SAFEs
    attribute :discount_rate, :decimal  # For convertible notes
    attribute :conversion_trigger, :atom # :qualified_financing, :ipo, :dissolution
  end
end

# Revenue stream model
defmodule FinancialRecipe.RevenueStream do
  use Ash.Resource

  attributes do
    uuid_primary_key :id
    attribute :name, :string
    attribute :model_type, :atom  # :subscription, :transactional, :marketplace, :usage_based
    attribute :pricing, :map      # %{tiers: [...], base_price: ..., per_unit: ...}
    attribute :growth_assumptions, :map  # %{initial_customers: N, monthly_growth_rate: 0.15, churn: 0.05}
    attribute :seasonality, :map  # Optional monthly multipliers
  end
end
```

### AI Context Extraction Flow

```
CruxDev Project State
  │
  ├─ project.toml → business_type, team_size, stage
  ├─ COMPETITORS.md → market positioning, pricing benchmarks
  ├─ build_plans/ → feature roadmap, timeline estimates
  ├─ tech stack detection → infrastructure cost estimates
  │
  ▼
Context Extractor (Elixir module)
  │
  ├─ Determines revenue model type (SaaS, marketplace, etc.)
  ├─ Estimates infrastructure costs from stack
  ├─ Derives team cost from headcount + market rates
  ├─ Pulls competitor pricing for revenue benchmarking
  │
  ▼
LLM Revenue Model Generator
  │
  ├─ Input: structured context + business type
  ├─ Output: revenue streams, growth assumptions, cost structure
  ├─ Validation: assumptions checked against industry benchmarks
  │
  ▼
Projection Engine (GenServer)
  │
  ├─ Generates 36-month (3-year) time-series projections
  ├─ P&L, Cash Flow, Balance Sheet
  ├─ SaaS metrics: MRR, ARR, LTV, CAC, churn, payback period
  ├─ Runway calculation with burn rate
  │
  ▼
Financial Dashboard (LiveView)
  │
  ├─ Interactive KPI dashboard
  ├─ Scenario comparison
  ├─ Investor report export (PDF)
  └─ Cap table visualization
```

---

## Implementation Phases

### Phase 1: Core Projection Engine (2 weeks)

**Goal:** Generate 3-year financial projections from manual input.

- [ ] Ash resources: Company, RevenueStream, CostCenter, Projection
- [ ] Projection GenServer: time-series calculation from assumptions
- [ ] Revenue model types: subscription (SaaS), transactional, marketplace, usage-based
- [ ] Cost model: personnel (salary bands), infrastructure (cloud cost curves), marketing (CAC-based)
- [ ] Output: monthly P&L, cash flow statement, balance sheet
- [ ] SaaS metrics calculator: MRR, ARR, LTV, CAC, churn, payback period, quick ratio
- [ ] Runway calculator: months of runway from cash balance and burn rate
- [ ] Tests: 100% coverage, verified calculation accuracy against known financial models
- [ ] Validation: all monetary calculations use Decimal, never float

### Phase 2: Scenario Engine + Cap Table (2 weeks)

**Goal:** Multiple scenarios and equity modeling.

- [ ] Scenario branching: fork a model, override variables, compare outcomes
- [ ] Scenario comparison: side-by-side P&L/cash flow/runway for up to 4 scenarios
- [ ] Sensitivity analysis: vary one input (e.g., churn rate), see range of outcomes
- [ ] Cap table model: common shares, preferred shares, options, SAFEs, convertible notes
- [ ] Dilution calculator: model funding rounds and see ownership changes
- [ ] Vesting schedule tracker: cliff, monthly/quarterly vesting, acceleration triggers
- [ ] Tests: cap table math verified against known dilution examples

### Phase 3: LiveView Dashboard (2 weeks)

**Goal:** Real-time interactive financial dashboard.

- [ ] KPI dashboard: runway, burn rate, MRR, growth rate, LTV/CAC ratio
- [ ] Model builder: visual editor for revenue streams and cost centers
- [ ] Chart components: revenue over time, expense breakdown, cash runway projection
- [ ] Scenario toggle: switch between scenarios, overlay on same chart
- [ ] Cap table pie chart and dilution waterfall
- [ ] Responsive design (founder checks runway on phone at 2am — this is real)
- [ ] Phoenix Presence: real-time collaboration indicators

### Phase 4: AI Context Integration (2 weeks)

**Goal:** CruxDev generates financial models from project context.

- [ ] Context extractor: read project.toml, COMPETITORS.md, build plans, tech stack
- [ ] Business type classifier: SaaS, marketplace, e-commerce, services, hybrid
- [ ] Infrastructure cost estimator: stack → estimated monthly cloud costs (AWS/GCP/Fly.io)
- [ ] Team cost estimator: roles + locations → salary ranges from market data
- [ ] Competitor pricing analyzer: extract pricing from COMPETITORS.md for revenue benchmarking
- [ ] LLM prompt pipeline: context → revenue model assumptions → validation
- [ ] One-click model generation: "Generate financial model for this project"
- [ ] Assumption explainability: every AI-generated number has a cited rationale

### Phase 5: Integrations + Export (1 week)

**Goal:** Connect to real financial data and produce investor-ready output.

- [ ] Stripe integration: sync actual MRR, churn, revenue by plan
- [ ] QuickBooks Online integration: import actuals for plan-vs-actual comparison
- [ ] Xero integration: same as QuickBooks
- [ ] Actuals overlay: show projected vs. actual on same charts
- [ ] Reforecasting: update projections based on actual performance
- [ ] PDF export: investor-ready financial summary (branded, professional)
- [ ] Excel export: full model in spreadsheet format for investors who want to poke at it
- [ ] CSV export: raw data for custom analysis

### Phase 6: CruxVibe Recipe Integration (1 week)

**Goal:** Package as a CruxVibe recipe for the entrepreneur vertical.

- [ ] Recipe manifest: dependencies, configuration, pricing tier
- [ ] Recipe combinator integration: works alongside other recipes
- [ ] Entrepreneur dashboard widget: financial KPIs on CruxVibe site
- [ ] Gated access: public summary metrics, detailed model behind auth
- [ ] Standalone mode: works without CruxVibe for CruxDev-only users
- [ ] Pricing: included in CruxVibe $100/mo plan; standalone at $49/mo or $79/mo with AI features

---

## Pricing Strategy

| Tier | Price | Features |
|------|-------|----------|
| **Starter** | Free | 1 model, 1 scenario, basic P&L, runway calculator, no export |
| **Founder** | $49/mo | Unlimited models, 4 scenarios, cap table, PDF/Excel export, Stripe sync |
| **Pro** | $79/mo | Everything in Founder + AI model generation, assumption validation, QuickBooks/Xero sync, collaboration |
| **CruxVibe Bundle** | Included in $100/mo | Full Pro features bundled into CruxVibe entrepreneur plan |

**Positioning:** Slots into the gap left by Finmark ($49/mo) at the same price point, but with AI generation that no competitor offers. Cheaper than Causal ($250/mo) while serving the same early-stage audience Causal is abandoning as it pivots to enterprise via Lucanet.

---

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Financial calculations must be correct | HIGH | All monetary math uses Decimal (never float); test suite includes known financial models with verified outputs; CPA review before launch |
| Regulatory risk if models used for fundraising | MEDIUM | Clear disclaimers: "projections, not guarantees"; not a financial advisor; not audited financials |
| Scope creep toward enterprise FP&A | MEDIUM | Hard scope boundary: seed to Series A only; say no to multi-entity consolidation, departmental budgeting, GAAP compliance |
| AI-generated assumptions could be wrong | MEDIUM | Every assumption is editable; AI explains its reasoning; founder always has final say; "AI-suggested" label on all generated numbers |
| Market timing if Finmark un-sunsets | LOW | Our differentiator (context-aware) exists regardless; Finmark pivot to BILL means startup market is abandoned |

---

## Success Metrics

| Metric | Target | Timeframe |
|--------|--------|-----------|
| Recipe convergence (all tests passing, 100% coverage) | Phase 1-6 complete | 10 weeks |
| Projection accuracy vs. manual Excel models | <2% variance on same inputs | Phase 1 |
| AI model generation acceptance rate | >60% of generated assumptions kept by founders | Phase 4 |
| Time to first financial model (manual) | <15 minutes | Phase 3 |
| Time to first financial model (AI-generated) | <2 minutes | Phase 4 |
| Standalone signups (post-launch) | 100 in first 90 days | Launch + 90d |

---

## Competitive Moat Summary

1. **Context-aware generation** — No competitor generates financial models from project knowledge. They all start from blank.
2. **Convergence-verified** — The financial model is tested, validated, and maintained by the same engine that builds the software. Plan and execution stay in sync.
3. **Finmark market gap** — The best startup-focused tool is being sunset. The market needs a replacement.
4. **Recipe economics** — At $49-79/mo standalone or bundled in CruxVibe $100/mo, this is priced for founders, not CFOs.
5. **Elixir/LiveView UX** — Real-time dashboard updates without JavaScript framework complexity. Collaboration built into the platform layer.

---

## References

- [Causal Pricing](https://causal.app/pricing) — Pro at $250/mo; acquired by Lucanet for enterprise xP&A
- [Causal Review 2026 (Research.com)](https://research.com/software/reviews/causal)
- [Finmark (Y Combinator)](https://www.ycombinator.com/companies/finmark) — Acquired by BILL; original product being sunset April 2026
- [Finmark Review 2026 (Research.com)](https://research.com/software/reviews/finmark)
- [LivePlan Pricing](https://www.liveplan.com/pricing) — Standard $20/mo, Premium $40/mo
- [LivePlan Review 2026 (TRUiC)](https://startupsavant.com/liveplan-review)
- [ProjectionHub](https://www.projectionhub.com/) — 100+ CPA-developed templates
- [Brixx Pricing](https://brixx.com/pricing/) — $42/mo ($33.60/mo annual)
- [PlanGuru](https://www.planguru.com/) — $99/mo, 20+ forecasting methods
- [PlanGuru Review 2026 (Research.com)](https://research.com/software/reviews/planguru)
- [Fuentes — Double-Entry Accounting for Elixir](https://github.com/davidkuhta/fuentes)
- [ExLedger — Double-Entry in Elixir](https://github.com/tristanperalta/exledger)
- [Building a Double-Entry Ledger with Elixir and TigerBeetle](https://medium.com/@altuntasfatih42/building-a-double-entry-ledger-with-elixir-and-tigerbeetle-f0f9fcc37408)
- [Cap Table Management Software Market (Qapita)](https://www.qapita.com/blog/top-cap-table-management-software-for-startups-2026)
- [AI Financial Modeling Tools 2026 (Drivetrain)](https://www.drivetrain.ai/solutions/ai-financial-modeling-tools-for-businesses)
- [AI Financial Modeling (Datarails)](https://www.datarails.com/ai-financial-modeling/)
- [Financial Planning Software Overview 2026 (re:cap)](https://www.re-cap.com/blog/financial-planning-software)
- [Best AI Tools for Financial Modeling 2026 (Wall Street Prep)](https://www.wallstreetprep.com/knowledge/ranking-the-best-ai-tools-for-financial-modeling-2026/)
