# Unit Economics Analysis Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Review Cadence:** Quarterly

---

## UNIT ECONOMICS — {{COMPANY_NAME}}

**Period:** {{ANALYSIS_PERIOD}}
**Analyst:** {{AUTHOR}}
**Date:** {{DATE}}

---

### 1. CORE METRICS

| Metric | Current | Prior Period | Trend | Target | Status |
|--------|---------|-------------|-------|--------|--------|
| **CAC** (Customer Acquisition Cost) | ${{CAC}} | ${{CAC_PRIOR}} | {{CAC_TREND}} | ${{CAC_TARGET}} | {{CAC_STATUS}} |
| **LTV** (Lifetime Value) | ${{LTV}} | ${{LTV_PRIOR}} | {{LTV_TREND}} | ${{LTV_TARGET}} | {{LTV_STATUS}} |
| **LTV:CAC Ratio** | {{LTV_CAC_RATIO}}x | {{LTV_CAC_PRIOR}}x | {{RATIO_TREND}} | >{{LTV_CAC_TARGET}}x | {{RATIO_STATUS}} |
| **Payback Period** | {{PAYBACK}} months | {{PAYBACK_PRIOR}} months | {{PAYBACK_TREND}} | <{{PAYBACK_TARGET}} months | {{PAYBACK_STATUS}} |
| **Monthly Churn** | {{MONTHLY_CHURN}}% | {{CHURN_PRIOR}}% | {{CHURN_TREND}} | <{{CHURN_TARGET}}% | {{CHURN_STATUS}} |
| **ARPU** (monthly) | ${{ARPU}} | ${{ARPU_PRIOR}} | {{ARPU_TREND}} | ${{ARPU_TARGET}} | {{ARPU_STATUS}} |
| **Gross Margin** | {{GROSS_MARGIN}}% | {{GM_PRIOR}}% | {{GM_TREND}} | >{{GM_TARGET}}% | {{GM_STATUS}} |

**Health check:** LTV:CAC > 3x and Payback < 18 months indicates healthy unit economics.

---

### 2. CUSTOMER ACQUISITION COST (CAC) BREAKDOWN

#### 2.1 Blended CAC

| Component | Amount | % of Total |
|-----------|--------|-----------|
| Marketing spend (paid) | ${{PAID_MARKETING}} | |
| Marketing salaries | ${{MARKETING_SALARIES}} | |
| Sales salaries + commissions | ${{SALES_COMP}} | |
| Sales tools (CRM, etc.) | ${{SALES_TOOLS}} | |
| Content / SEO investment | ${{CONTENT_SPEND}} | |
| Events / conferences | ${{EVENTS_SPEND}} | |
| Other acquisition costs | ${{OTHER_ACQ}} | |
| **Total acquisition spend** | **${{TOTAL_ACQ_SPEND}}** | 100% |
| New customers acquired | {{NEW_CUSTOMERS}} | |
| **Blended CAC** | **${{BLENDED_CAC}}** | |

#### 2.2 CAC by Channel

| Channel | Spend | Customers | CAC | % of Customers | Trend |
|---------|-------|-----------|-----|---------------|-------|
| Organic / SEO | ${{ORGANIC_SPEND}} | {{ORGANIC_CUST}} | ${{ORGANIC_CAC}} | {{ORGANIC_PCT}}% | {{ORGANIC_TREND}} |
| Paid search | ${{PAID_SEARCH_SPEND}} | {{PAID_SEARCH_CUST}} | ${{PAID_SEARCH_CAC}} | | |
| Paid social | ${{PAID_SOCIAL_SPEND}} | {{PAID_SOCIAL_CUST}} | ${{PAID_SOCIAL_CAC}} | | |
| Referral | ${{REFERRAL_SPEND}} | {{REFERRAL_CUST}} | ${{REFERRAL_CAC}} | | |
| Direct sales | ${{DIRECT_SPEND}} | {{DIRECT_CUST}} | ${{DIRECT_CAC}} | | |
| Partnerships | ${{PARTNER_SPEND}} | {{PARTNER_CUST}} | ${{PARTNER_CAC}} | | |
| **Blended** | | | **${{BLENDED_CAC}}** | 100% | |

#### 2.3 CAC by Segment

| Segment | CAC | LTV | LTV:CAC | Payback | Notes |
|---------|-----|-----|---------|---------|-------|
| {{SEGMENT_1: SMB}} | ${{SMB_CAC}} | ${{SMB_LTV}} | {{SMB_RATIO}}x | {{SMB_PAYBACK}} mo | |
| {{SEGMENT_2: Mid-market}} | ${{MM_CAC}} | ${{MM_LTV}} | {{MM_RATIO}}x | {{MM_PAYBACK}} mo | |
| {{SEGMENT_3: Enterprise}} | ${{ENT_CAC}} | ${{ENT_LTV}} | {{ENT_RATIO}}x | {{ENT_PAYBACK}} mo | |

---

### 3. LIFETIME VALUE (LTV) CALCULATION

#### 3.1 Method: Revenue-Based

```
LTV = ARPU x Gross Margin % x Average Customer Lifetime

Where:
  ARPU (monthly)           = ${{ARPU}}
  Gross Margin             = {{GROSS_MARGIN}}%
  Monthly Churn Rate       = {{MONTHLY_CHURN}}%
  Average Lifetime (months)= 1 / Monthly Churn = {{AVG_LIFETIME}} months

LTV = ${{ARPU}} x {{GROSS_MARGIN}}% x {{AVG_LIFETIME}}
LTV = ${{LTV}}
```

#### 3.2 Method: Cohort-Based (Preferred for accuracy)

| Cohort Month | Starting | Month 3 | Month 6 | Month 12 | Month 18 | Month 24 | Month 36 |
|-------------|----------|---------|---------|----------|----------|----------|----------|
| Retention % | 100% | | | | | | |
| Cumulative Revenue / Customer | | | | | | | |

**Projected LTV (36-month):** ${{COHORT_LTV_36}}
**Projected LTV (60-month):** ${{COHORT_LTV_60}}

#### 3.3 Net Revenue Retention

| Metric | Value |
|--------|-------|
| Beginning MRR (cohort) | ${{BEG_MRR}} |
| Expansion MRR | ${{EXPANSION_MRR}} |
| Contraction MRR | ${{CONTRACTION_MRR}} |
| Churned MRR | ${{CHURNED_MRR}} |
| Ending MRR | ${{END_MRR}} |
| **Net Revenue Retention** | **{{NRR}}%** |

NRR > 100% means existing customers grow revenue even without new customers.

---

### 4. PAYBACK PERIOD

```
Payback Period = CAC / (ARPU x Gross Margin)

= ${{CAC}} / (${{ARPU}} x {{GROSS_MARGIN}}%)
= {{PAYBACK}} months
```

**Cash flow breakeven per customer:**

| Month | Cumulative Revenue | Cumulative Margin | CAC Recovered? |
|-------|-------------------|-------------------|---------------|
| 0 (acquisition) | $0 | -${{CAC}} | No |
| 3 | | | |
| 6 | | | |
| 9 | | | |
| 12 | | | |
| {{PAYBACK}} | | $0 | **Yes** |

---

### 5. FUNNEL ECONOMICS

| Stage | Volume | Conversion | Cost | Cost Per |
|-------|--------|-----------|------|----------|
| Impressions / reach | {{IMPRESSIONS}} | — | ${{IMPRESSION_COST}} | ${{CPM}} CPM |
| Website visitors | {{VISITORS}} | {{VISIT_RATE}}% | | ${{COST_PER_VISIT}} |
| Leads (signups / MQLs) | {{LEADS}} | {{LEAD_RATE}}% | | ${{CPL}} per lead |
| Qualified leads (SQLs) | {{SQLS}} | {{SQL_RATE}}% | | ${{COST_PER_SQL}} |
| Trials / demos | {{TRIALS}} | {{TRIAL_RATE}}% | | ${{COST_PER_TRIAL}} |
| Customers | {{CUSTOMERS}} | {{CLOSE_RATE}}% | | **${{CAC}} CAC** |

---

### 6. MARGIN ANALYSIS

| Revenue Component | Amount / Customer / Month | % of ARPU |
|-------------------|--------------------------|-----------|
| Gross revenue (ARPU) | ${{ARPU}} | 100% |
| (-) Hosting / delivery | ${{HOSTING_PER_CUST}} | {{HOSTING_PCT}}% |
| (-) Support cost | ${{SUPPORT_PER_CUST}} | {{SUPPORT_PCT}}% |
| (-) Payment processing | ${{PROCESSING_PER_CUST}} | {{PROCESSING_PCT}}% |
| (-) Other COGS | ${{OTHER_COGS_PER_CUST}} | {{OTHER_COGS_PCT}}% |
| **= Gross margin per customer** | **${{GM_PER_CUST}}** | **{{GROSS_MARGIN}}%** |

---

### 7. IMPROVEMENT LEVERS

| Lever | Current | Target | Impact on LTV:CAC | Priority | Action Plan |
|-------|---------|--------|-------------------|----------|-------------|
| Reduce churn | {{MONTHLY_CHURN}}% | {{CHURN_TARGET}}% | +{{CHURN_IMPACT}}x | {{CHURN_PRIORITY}} | {{CHURN_ACTION}} |
| Increase ARPU | ${{ARPU}} | ${{ARPU_TARGET}} | +{{ARPU_IMPACT}}x | {{ARPU_PRIORITY}} | {{ARPU_ACTION}} |
| Reduce CAC | ${{CAC}} | ${{CAC_TARGET}} | +{{CAC_IMPACT}}x | {{CAC_PRIORITY}} | {{CAC_ACTION}} |
| Improve gross margin | {{GROSS_MARGIN}}% | {{GM_TARGET}}% | +{{GM_IMPACT}}x | {{GM_PRIORITY}} | {{GM_ACTION}} |
| Increase NRR | {{NRR}}% | {{NRR_TARGET}}% | +{{NRR_IMPACT}}x | {{NRR_PRIORITY}} | {{NRR_ACTION}} |

---

### 8. BENCHMARKS

| Metric | Our Value | Top Quartile SaaS | Median SaaS | Source |
|--------|-----------|-------------------|-------------|--------|
| LTV:CAC | {{LTV_CAC_RATIO}}x | >5x | 3x | {{BENCH_SOURCE}} |
| Payback Period | {{PAYBACK}} mo | <12 mo | 15-18 mo | |
| Net Revenue Retention | {{NRR}}% | >120% | 100-110% | |
| Gross Margin | {{GROSS_MARGIN}}% | >80% | 70-75% | |
| Monthly Logo Churn | {{MONTHLY_CHURN}}% | <1% | 2-3% | |
| CAC:ARPU Ratio | {{CAC_ARPU}}x | <12x | 12-18x | |
