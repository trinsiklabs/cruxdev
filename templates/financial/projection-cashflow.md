# Cash Flow Projection Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Review Cadence:** Monthly

---

## CASH FLOW PROJECTION — {{COMPANY_NAME}}

**Period:** {{FISCAL_YEAR}}
**Currency:** {{CURRENCY}}
**Prepared by:** {{AUTHOR}}

---

### MONTHLY CASH FLOW

| Line Item | Jan | Feb | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dec | **FY Total** |
|-----------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-------------|
| **BEGINNING CASH** | | | | | | | | | | | | | |
| | | | | | | | | | | | | | |
| **CASH INFLOWS** | | | | | | | | | | | | | |
| Collections from customers | | | | | | | | | | | | | |
| New sales (collected) | | | | | | | | | | | | | |
| Service/consulting revenue | | | | | | | | | | | | | |
| Interest income | | | | | | | | | | | | | |
| Other income | | | | | | | | | | | | | |
| **Total Cash Inflows** | | | | | | | | | | | | | |
| | | | | | | | | | | | | | |
| **CASH OUTFLOWS** | | | | | | | | | | | | | |
| *Payroll & People* | | | | | | | | | | | | | |
| Gross payroll | | | | | | | | | | | | | |
| Payroll taxes | | | | | | | | | | | | | |
| Benefits (health, dental, vision) | | | | | | | | | | | | | |
| Contractor payments | | | | | | | | | | | | | |
| *Operations* | | | | | | | | | | | | | |
| Rent / office | | | | | | | | | | | | | |
| Hosting / cloud infrastructure | | | | | | | | | | | | | |
| Software subscriptions | | | | | | | | | | | | | |
| Insurance | | | | | | | | | | | | | |
| *Sales & Marketing* | | | | | | | | | | | | | |
| Advertising spend | | | | | | | | | | | | | |
| Sales commissions | | | | | | | | | | | | | |
| Events / travel | | | | | | | | | | | | | |
| *Professional Services* | | | | | | | | | | | | | |
| Legal | | | | | | | | | | | | | |
| Accounting | | | | | | | | | | | | | |
| Other professional | | | | | | | | | | | | | |
| *Other* | | | | | | | | | | | | | |
| Equipment / hardware | | | | | | | | | | | | | |
| Loan repayments | | | | | | | | | | | | | |
| Interest payments | | | | | | | | | | | | | |
| Tax payments | | | | | | | | | | | | | |
| Other expenses | | | | | | | | | | | | | |
| **Total Cash Outflows** | | | | | | | | | | | | | |
| | | | | | | | | | | | | | |
| **NET OPERATING CASH FLOW** | | | | | | | | | | | | | |
| | | | | | | | | | | | | | |
| **FINANCING ACTIVITIES** | | | | | | | | | | | | | |
| Equity investment received | | | | | | | | | | | | | |
| Debt drawn | | | | | | | | | | | | | |
| Debt repaid | | | | | | | | | | | | | |
| **Net Financing Cash Flow** | | | | | | | | | | | | | |
| | | | | | | | | | | | | | |
| **NET CASH FLOW** | | | | | | | | | | | | | |
| **ENDING CASH BALANCE** | | | | | | | | | | | | | |

---

### RUNWAY ANALYSIS

| Metric | Value |
|--------|-------|
| Current cash balance | ${{CURRENT_CASH}} |
| Average monthly burn (last 3 months) | ${{AVG_BURN_3M}} |
| Average monthly burn (last 6 months) | ${{AVG_BURN_6M}} |
| Monthly burn trend | {{BURN_TREND: decreasing / stable / increasing}} |
| **Runway at current burn rate** | **{{RUNWAY_MONTHS}} months** |
| Runway at projected burn rate | {{PROJECTED_RUNWAY}} months |
| Cash-flow positive month (projected) | {{CASHFLOW_POSITIVE_MONTH}} |
| Minimum cash balance (projected) | ${{MIN_CASH}} in {{MIN_CASH_MONTH}} |

---

### COLLECTIONS SCHEDULE

| Customer / Cohort | Amount Owed | Payment Terms | Expected Collection Month | Confidence |
|-------------------|-------------|---------------|--------------------------|------------|
| {{CUSTOMER_1}} | ${{AMOUNT_1}} | {{TERMS_1}} | {{COLLECTION_MONTH_1}} | {{CONFIDENCE_1: High/Medium/Low}} |
| {{CUSTOMER_2}} | ${{AMOUNT_2}} | {{TERMS_2}} | {{COLLECTION_MONTH_2}} | {{CONFIDENCE_2}} |
| Recurring subscriptions | ${{MRR}} / month | Prepaid | Monthly | High |

**Accounts Receivable Aging:**

| Aging Bucket | Amount | % of Total |
|-------------|--------|-----------|
| Current (0-30 days) | ${{AR_CURRENT}} | |
| 31-60 days | ${{AR_31_60}} | |
| 61-90 days | ${{AR_61_90}} | |
| 90+ days | ${{AR_90_PLUS}} | |
| **Total A/R** | ${{AR_TOTAL}} | 100% |

---

### COMMITTED FUTURE EXPENDITURES

| Obligation | Monthly Amount | Start | End | Total Remaining | Cancelable? |
|-----------|---------------|-------|-----|----------------|------------|
| Office lease | ${{LEASE_MONTHLY}} | {{LEASE_START}} | {{LEASE_END}} | ${{LEASE_REMAINING}} | {{LEASE_CANCEL}} |
| Cloud contract | ${{CLOUD_MONTHLY}} | {{CLOUD_START}} | {{CLOUD_END}} | ${{CLOUD_REMAINING}} | {{CLOUD_CANCEL}} |
| {{OTHER_COMMITMENT}} | ${{OTHER_MONTHLY}} | | | ${{OTHER_REMAINING}} | |
| **Total Committed** | | | | ${{TOTAL_COMMITTED}} | |

---

### SCENARIO ANALYSIS

| Scenario | Monthly Burn | Runway | Cash at Year-End | Key Assumption Change |
|----------|-------------|--------|------------------|---------------------|
| **Pessimistic** | ${{PESSIMISTIC_BURN}} | {{PESSIMISTIC_RUNWAY}} months | ${{PESSIMISTIC_CASH}} | {{PESSIMISTIC_CHANGE}} |
| **Base** | ${{BASE_BURN}} | {{BASE_RUNWAY}} months | ${{BASE_CASH}} | Current plan |
| **Optimistic** | ${{OPTIMISTIC_BURN}} | {{OPTIMISTIC_RUNWAY}} months | ${{OPTIMISTIC_CASH}} | {{OPTIMISTIC_CHANGE}} |
| **Emergency** | ${{EMERGENCY_BURN}} | {{EMERGENCY_RUNWAY}} months | — | Minimum viable operations |

---

### CASH MANAGEMENT POLICIES

| Policy | Threshold | Action |
|--------|-----------|--------|
| Minimum cash reserve | ${{MIN_RESERVE}} | Do not spend below this level |
| Fundraise trigger | {{FUNDRAISE_TRIGGER_MONTHS}} months runway | Begin fundraising process |
| Emergency trigger | {{EMERGENCY_TRIGGER_MONTHS}} months runway | Implement cost reduction plan |
| Surplus threshold | ${{SURPLUS_THRESHOLD}} | Consider short-term investments |

---

### ASSUMPTIONS

- Collection lag: {{COLLECTION_LAG}} days average from invoice to payment
- Payroll timing: {{PAYROLL_TIMING: bi-weekly / semi-monthly / monthly}}
- Annual contracts paid: {{ANNUAL_BILLING: upfront / quarterly / monthly}}
- Seasonal patterns: {{SEASONAL_CASH_NOTES}}
