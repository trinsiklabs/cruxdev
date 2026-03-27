# Financial Model Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Review Cadence:** Quarterly

---

## FINANCIAL MODEL — {{COMPANY_NAME}}

**Model Date:** {{MODEL_DATE}}
**Model Author:** {{AUTHOR}}
**Model Version:** {{VERSION}}
**Projection Period:** {{START_YEAR}} — {{END_YEAR}} ({{NUM_YEARS}} years)
**Currency:** {{CURRENCY}}

---

### 1. KEY ASSUMPTIONS

All inputs driving the model. Change these to run scenarios.

#### 1.1 Revenue Assumptions

| Assumption | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 | Source/Rationale |
|-----------|--------|--------|--------|--------|--------|-----------------|
| Total Addressable Market (TAM) | ${{TAM_Y1}} | ${{TAM_Y2}} | ${{TAM_Y3}} | ${{TAM_Y4}} | ${{TAM_Y5}} | {{TAM_SOURCE}} |
| Serviceable Addressable Market (SAM) | ${{SAM_Y1}} | | | | | {{SAM_RATIONALE}} |
| Market share target | {{SHARE_Y1}}% | {{SHARE_Y2}}% | {{SHARE_Y3}}% | {{SHARE_Y4}}% | {{SHARE_Y5}}% | {{SHARE_RATIONALE}} |
| Average revenue per user (ARPU) / month | ${{ARPU}} | ${{ARPU_Y2}} | ${{ARPU_Y3}} | ${{ARPU_Y4}} | ${{ARPU_Y5}} | {{ARPU_RATIONALE}} |
| Monthly churn rate | {{CHURN_Y1}}% | {{CHURN_Y2}}% | {{CHURN_Y3}}% | {{CHURN_Y4}}% | {{CHURN_Y5}}% | {{CHURN_RATIONALE}} |
| Annual price increase | {{PRICE_INCREASE}}% | | | | | {{PRICE_RATIONALE}} |

#### 1.2 Customer Acquisition Assumptions

| Assumption | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 | Source/Rationale |
|-----------|--------|--------|--------|--------|--------|-----------------|
| Marketing spend | ${{MKTG_Y1}} | ${{MKTG_Y2}} | ${{MKTG_Y3}} | ${{MKTG_Y4}} | ${{MKTG_Y5}} | |
| Cost per lead (CPL) | ${{CPL}} | | | | | |
| Lead-to-trial conversion | {{LEAD_TRIAL}}% | | | | | |
| Trial-to-paid conversion | {{TRIAL_PAID}}% | | | | | |
| Organic/referral % of new customers | {{ORGANIC_PCT}}% | | | | | |
| Sales cycle length (days) | {{SALES_CYCLE}} | | | | | |

#### 1.3 Cost Assumptions

| Assumption | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 | Source/Rationale |
|-----------|--------|--------|--------|--------|--------|-----------------|
| Headcount | {{HC_Y1}} | {{HC_Y2}} | {{HC_Y3}} | {{HC_Y4}} | {{HC_Y5}} | |
| Average salary + benefits | ${{AVG_COMP}} | | | | | |
| Annual raise % | {{RAISE_PCT}}% | | | | | |
| Hosting/infrastructure per user | ${{HOSTING_PER_USER}} | | | | | |
| Office/rent per month | ${{RENT}} | | | | | |
| Annual SaaS tools spend | ${{SAAS_TOOLS}} | | | | | |

#### 1.4 Financing Assumptions

| Assumption | Value | Notes |
|-----------|-------|-------|
| Starting cash | ${{STARTING_CASH}} | |
| Fundraise timing | {{FUNDRAISE_TIMING}} | |
| Fundraise amount | ${{FUNDRAISE_AMOUNT}} | |
| Pre-money valuation | ${{PRE_MONEY}} | |
| Debt facility | ${{DEBT_AMOUNT}} | |
| Interest rate | {{INTEREST_RATE}}% | |

---

### 2. REVENUE MODEL

#### 2.1 Customer Growth

| Metric | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|--------|--------|--------|--------|--------|--------|
| Beginning customers | 0 | | | | |
| New customers acquired | | | | | |
| Churned customers | | | | | |
| Ending customers | | | | | |
| Net customer growth | | | | | |

**Calculations:**
- New customers = (Marketing spend / CPL) x Lead-to-trial x Trial-to-paid / (1 - Organic %)
- Churned customers = Beginning customers x Annual churn rate
- Ending customers = Beginning + New - Churned

#### 2.2 Revenue Streams

| Stream | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|--------|--------|--------|--------|--------|--------|
| {{STREAM_1: Subscription revenue}} | | | | | |
| {{STREAM_2: Professional services}} | | | | | |
| {{STREAM_3: Other}} | | | | | |
| **Total Revenue** | | | | | |

#### 2.3 Monthly Revenue Detail (Year 1)

| Month | Customers | MRR | Cumulative Revenue |
|-------|-----------|-----|-------------------|
| M1 | | | |
| M2 | | | |
| M3 | | | |
| M4 | | | |
| M5 | | | |
| M6 | | | |
| M7 | | | |
| M8 | | | |
| M9 | | | |
| M10 | | | |
| M11 | | | |
| M12 | | | |

---

### 3. PROFIT & LOSS PROJECTION

| Line Item | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|-----------|--------|--------|--------|--------|--------|
| **Revenue** | | | | | |
| Subscription | | | | | |
| Services | | | | | |
| Other | | | | | |
| **Total Revenue** | | | | | |
| | | | | | |
| **Cost of Revenue (COGS)** | | | | | |
| Hosting / infrastructure | | | | | |
| Customer support | | | | | |
| Payment processing | | | | | |
| **Total COGS** | | | | | |
| **Gross Profit** | | | | | |
| **Gross Margin %** | | | | | |
| | | | | | |
| **Operating Expenses** | | | | | |
| Engineering (salaries + tools) | | | | | |
| Sales & Marketing | | | | | |
| General & Administrative | | | | | |
| **Total OpEx** | | | | | |
| | | | | | |
| **EBITDA** | | | | | |
| **EBITDA Margin %** | | | | | |
| Depreciation & Amortization | | | | | |
| **EBIT** | | | | | |
| Interest expense | | | | | |
| **EBT (Earnings Before Tax)** | | | | | |
| Tax provision | | | | | |
| **Net Income** | | | | | |
| **Net Margin %** | | | | | |

---

### 4. CASH FLOW PROJECTION

| Line Item | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|-----------|--------|--------|--------|--------|--------|
| **Operating Activities** | | | | | |
| Net Income | | | | | |
| + Depreciation & Amortization | | | | | |
| +/- Changes in working capital | | | | | |
| Accounts receivable change | | | | | |
| Accounts payable change | | | | | |
| Deferred revenue change | | | | | |
| **Cash from Operations** | | | | | |
| | | | | | |
| **Investing Activities** | | | | | |
| Capital expenditures | | | | | |
| **Cash from Investing** | | | | | |
| | | | | | |
| **Financing Activities** | | | | | |
| Equity raised | | | | | |
| Debt raised / (repaid) | | | | | |
| **Cash from Financing** | | | | | |
| | | | | | |
| **Net Cash Flow** | | | | | |
| Beginning Cash Balance | | | | | |
| **Ending Cash Balance** | | | | | |
| **Months of Runway** | | | | | |

---

### 5. KEY METRICS DASHBOARD

| Metric | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|--------|--------|--------|--------|--------|--------|
| ARR (Annual Recurring Revenue) | | | | | |
| MRR (end of year) | | | | | |
| ARR Growth Rate | — | | | | |
| Gross Margin % | | | | | |
| Net Revenue Retention | | | | | |
| CAC (Customer Acquisition Cost) | | | | | |
| LTV (Lifetime Value) | | | | | |
| LTV:CAC Ratio | | | | | |
| Payback Period (months) | | | | | |
| Burn Rate (monthly) | | | | | |
| Runway (months) | | | | | |
| Rule of 40 (Growth % + EBITDA Margin %) | | | | | |
| Revenue per Employee | | | | | |
| Headcount | | | | | |

---

### 6. SCENARIO ANALYSIS

#### 6.1 Scenarios Defined

| Parameter | Pessimistic | Base | Optimistic |
|-----------|------------|------|-----------|
| Customer growth rate | {{PESSIMISTIC_GROWTH}} | {{BASE_GROWTH}} | {{OPTIMISTIC_GROWTH}} |
| Churn rate | {{PESSIMISTIC_CHURN}} | {{BASE_CHURN}} | {{OPTIMISTIC_CHURN}} |
| ARPU | ${{PESSIMISTIC_ARPU}} | ${{BASE_ARPU}} | ${{OPTIMISTIC_ARPU}} |
| CAC | ${{PESSIMISTIC_CAC}} | ${{BASE_CAC}} | ${{OPTIMISTIC_CAC}} |
| Hiring pace | {{PESSIMISTIC_HIRING}} | {{BASE_HIRING}} | {{OPTIMISTIC_HIRING}} |

#### 6.2 Scenario Outcomes (Year 3)

| Metric | Pessimistic | Base | Optimistic |
|--------|------------|------|-----------|
| ARR | | | |
| Net Income | | | |
| Cash Balance | | | |
| Runway (months) | | | |
| Headcount | | | |

---

### 7. SENSITIVITY ANALYSIS

How key metrics change with assumption variations:

#### 7.1 ARR Sensitivity to Churn and ARPU

| | ARPU -20% | ARPU -10% | ARPU Base | ARPU +10% | ARPU +20% |
|---|-----------|-----------|-----------|-----------|-----------|
| Churn +2% | | | | | |
| Churn +1% | | | | | |
| Churn Base | | | | | |
| Churn -1% | | | | | |
| Churn -2% | | | | | |

---

### 8. MODEL NOTES AND METHODOLOGY

**Data Sources:**
- {{DATA_SOURCE_1}}
- {{DATA_SOURCE_2}}
- {{DATA_SOURCE_3}}

**Key Methodology Decisions:**
- Revenue recognition: {{REVENUE_RECOGNITION_METHOD}}
- Customer count: {{CUSTOMER_COUNT_METHOD: paying accounts / seats / etc.}}
- Churn calculation: {{CHURN_METHOD: logo churn / revenue churn / net revenue retention}}

**Known Limitations:**
1. {{LIMITATION_1}}
2. {{LIMITATION_2}}
3. {{LIMITATION_3}}

**Change Log:**

| Date | Version | Author | Changes |
|------|---------|--------|---------|
| {{DATE}} | 1.0 | {{AUTHOR}} | Initial model |
