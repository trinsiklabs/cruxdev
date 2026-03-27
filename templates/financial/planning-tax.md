# Tax Planning Documentation Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Review Cadence:** Annual + per significant transaction

---

## TAX PLANNING — {{COMPANY_NAME}} — {{TAX_YEAR}}

**Prepared by:** {{AUTHOR}}
**Tax Advisor:** {{TAX_ADVISOR}}
**Entity Type:** {{ENTITY_TYPE: C-Corp / S-Corp / LLC / Sole Proprietorship}}
**Tax ID (EIN):** {{EIN}}
**Fiscal Year End:** {{FISCAL_YEAR_END}}
**State of Incorporation:** {{INCORPORATION_STATE}}

---

### 1. ENTITY STRUCTURE

| Entity | Type | Jurisdiction | Tax ID | Ownership | Purpose |
|--------|------|-------------|--------|-----------|---------|
| {{ENTITY_1}} | {{TYPE_1}} | {{JURISDICTION_1}} | {{TID_1}} | {{OWNERSHIP_1}} | {{PURPOSE_1}} |
| {{ENTITY_2}} | {{TYPE_2}} | {{JURISDICTION_2}} | {{TID_2}} | {{OWNERSHIP_2}} | {{PURPOSE_2}} |

**Structure notes:** {{STRUCTURE_NOTES}}

---

### 2. TAX CALENDAR

| Date | Filing / Payment | Entity | Form | Status |
|------|-----------------|--------|------|--------|
| Jan 15 | Q4 estimated tax payment | | {{FORM_1}} | |
| Jan 31 | W-2s and 1099s due | | | |
| Mar 15 | S-Corp / Partnership return (or extension) | | 1120-S / 1065 | |
| Apr 15 | C-Corp return (or extension) / Q1 estimated payment | | 1120 / 1040-ES | |
| Jun 15 | Q2 estimated tax payment | | | |
| Sep 15 | Extended S-Corp / Partnership return / Q3 estimated payment | | | |
| Oct 15 | Extended C-Corp / Individual return | | | |
| {{STATE_DATE}} | {{STATE_FILING}} | | {{STATE_FORM}} | |

---

### 3. INCOME TAX PROJECTION

#### 3.1 Federal

| Line Item | Projected | Prior Year | Notes |
|-----------|----------|-----------|-------|
| Gross revenue | ${{GROSS_REVENUE}} | ${{PY_REVENUE}} | |
| Cost of goods sold | ${{COGS}} | ${{PY_COGS}} | |
| Gross profit | ${{GROSS_PROFIT}} | | |
| Total deductions | ${{TOTAL_DEDUCTIONS}} | ${{PY_DEDUCTIONS}} | |
| Taxable income | ${{TAXABLE_INCOME}} | ${{PY_TAXABLE}} | |
| Federal tax rate | {{FED_RATE}}% | | |
| **Federal tax liability** | **${{FED_TAX}}** | **${{PY_FED_TAX}}** | |
| Credits | ${{CREDITS}} | | See Section 5 |
| Estimated payments made | ${{EST_PAYMENTS}} | | |
| **Remaining liability / (refund)** | **${{REMAINING}}** | | |

#### 3.2 State

| State | Taxable Income | Rate | Tax | Credits | Net Tax |
|-------|---------------|------|-----|---------|---------|
| {{STATE_1}} | ${{STATE1_INCOME}} | {{STATE1_RATE}}% | ${{STATE1_TAX}} | ${{STATE1_CREDITS}} | ${{STATE1_NET}} |
| {{STATE_2}} | ${{STATE2_INCOME}} | {{STATE2_RATE}}% | ${{STATE2_TAX}} | ${{STATE2_CREDITS}} | ${{STATE2_NET}} |
| **Total State** | | | | | **${{TOTAL_STATE_TAX}}** |

---

### 4. KEY DEDUCTIONS

| Deduction | Amount | Category | Documentation | Notes |
|-----------|--------|----------|--------------|-------|
| Salaries and wages | ${{SALARY_DEDUCTION}} | Ordinary | W-2s, payroll records | |
| Employee benefits | ${{BENEFITS_DEDUCTION}} | Ordinary | Plan documents | |
| Rent / lease payments | ${{RENT_DEDUCTION}} | Ordinary | Lease agreement | |
| Depreciation | ${{DEPRECIATION}} | See schedule | Fixed asset register | Section 179 / bonus |
| R&D expenses | ${{RD_EXPENSES}} | May qualify for credit | Time tracking, project docs | See Section 5 |
| Marketing / advertising | ${{MARKETING_DEDUCTION}} | Ordinary | Invoices | |
| Professional services | ${{PROFESSIONAL_DEDUCTION}} | Ordinary | Invoices | Legal, accounting |
| Insurance | ${{INSURANCE_DEDUCTION}} | Ordinary | Policies | |
| Travel | ${{TRAVEL_DEDUCTION}} | Ordinary | Receipts, itineraries | Business purpose documented |
| Meals (50%) | ${{MEALS_DEDUCTION}} | 50% deductible | Receipts w/ business purpose | |
| Home office | ${{HOME_OFFICE_DEDUCTION}} | {{METHOD: Simplified / Actual}} | Measurements, bills | |
| Interest expense | ${{INTEREST_DEDUCTION}} | Ordinary | Loan statements | |
| Bad debts | ${{BAD_DEBT_DEDUCTION}} | Ordinary | Write-off documentation | |
| **Total Deductions** | **${{TOTAL_DEDUCTIONS}}** | | | |

---

### 5. TAX CREDITS

| Credit | Estimated Amount | Requirements | Status | Documentation |
|--------|-----------------|-------------|--------|--------------|
| R&D Tax Credit (Section 41) | ${{RD_CREDIT}} | Qualified research activities | {{RD_STATUS}} | {{RD_DOCS}} |
| Work Opportunity Tax Credit | ${{WOTC}} | Qualifying employees | {{WOTC_STATUS}} | |
| {{STATE_CREDIT_1}} | ${{STATE_CREDIT_1_AMT}} | {{STATE_CREDIT_1_REQ}} | {{STATE_CREDIT_1_STATUS}} | |
| **Total Credits** | **${{TOTAL_CREDITS}}** | | | |

**R&D Credit Detail:**
- Qualified research expenses: ${{QRE}}
- Base amount: ${{BASE_AMOUNT}}
- Credit calculation method: {{RD_METHOD: Regular / ASC}}
- Applicable credit rate: {{RD_RATE}}%

---

### 6. DEPRECIATION SCHEDULE

| Asset | Date Acquired | Cost | Method | Life | Section 179? | Bonus? | Annual Depreciation | Accumulated | NBV |
|-------|-------------|------|--------|------|-------------|--------|--------------------|-----------|----|
| {{ASSET_1}} | {{DATE_1}} | ${{COST_1}} | {{METHOD_1}} | {{LIFE_1}} | {{S179_1}} | {{BONUS_1}} | ${{DEPR_1}} | ${{ACCUM_1}} | ${{NBV_1}} |
| {{ASSET_2}} | {{DATE_2}} | ${{COST_2}} | {{METHOD_2}} | {{LIFE_2}} | {{S179_2}} | {{BONUS_2}} | ${{DEPR_2}} | ${{ACCUM_2}} | ${{NBV_2}} |

---

### 7. ESTIMATED TAX PAYMENTS

| Quarter | Due Date | Federal | State | Total | Paid? | Check/Ref |
|---------|----------|---------|-------|-------|-------|-----------|
| Q1 | {{Q1_DATE}} | ${{Q1_FED}} | ${{Q1_STATE}} | ${{Q1_TOTAL}} | {{Q1_PAID}} | {{Q1_REF}} |
| Q2 | {{Q2_DATE}} | ${{Q2_FED}} | ${{Q2_STATE}} | ${{Q2_TOTAL}} | {{Q2_PAID}} | {{Q2_REF}} |
| Q3 | {{Q3_DATE}} | ${{Q3_FED}} | ${{Q3_STATE}} | ${{Q3_TOTAL}} | {{Q3_PAID}} | {{Q3_REF}} |
| Q4 | {{Q4_DATE}} | ${{Q4_FED}} | ${{Q4_STATE}} | ${{Q4_TOTAL}} | {{Q4_PAID}} | {{Q4_REF}} |
| **Total** | | **${{TOTAL_EST_FED}}** | **${{TOTAL_EST_STATE}}** | **${{TOTAL_EST}}** | | |

---

### 8. PLANNING STRATEGIES

| Strategy | Estimated Savings | Effort | Timeline | Status | Notes |
|----------|------------------|--------|----------|--------|-------|
| {{STRATEGY_1}} | ${{SAVINGS_1}} | {{EFFORT_1}} | {{TIMELINE_1}} | {{STATUS_1}} | {{NOTES_1}} |
| {{STRATEGY_2}} | ${{SAVINGS_2}} | {{EFFORT_2}} | {{TIMELINE_2}} | {{STATUS_2}} | {{NOTES_2}} |
| {{STRATEGY_3}} | ${{SAVINGS_3}} | {{EFFORT_3}} | {{TIMELINE_3}} | {{STATUS_3}} | {{NOTES_3}} |

**Common strategies to evaluate:**
- [ ] Maximize Section 179 / bonus depreciation
- [ ] R&D tax credit study
- [ ] Retirement plan contributions (SEP-IRA, Solo 401(k), etc.)
- [ ] Health insurance deductions (self-employed)
- [ ] Qualified Business Income (QBI) deduction (pass-through entities)
- [ ] Entity type optimization (LLC vs S-Corp election)
- [ ] State nexus optimization
- [ ] Income timing (deferral / acceleration)
- [ ] Charitable contributions strategy
- [ ] Capital gains planning (QSBS Section 1202 exclusion)

---

### 9. SALES TAX

| State | Nexus Type | Registration | Rate | Filing Frequency | Next Filing |
|-------|-----------|-------------|------|-----------------|------------|
| {{SALES_STATE_1}} | {{NEXUS_1: Physical / Economic}} | {{REG_1}} | {{RATE_1}}% | {{FREQ_1}} | {{NEXT_1}} |
| {{SALES_STATE_2}} | {{NEXUS_2}} | {{REG_2}} | {{RATE_2}}% | {{FREQ_2}} | {{NEXT_2}} |

**Economic nexus thresholds met:** {{NEXUS_STATES}}
**SaaS taxability notes:** {{SAAS_TAX_NOTES}}

---

### 10. COMPLIANCE CHECKLIST

- [ ] All 1099s issued by January 31
- [ ] W-2s issued by January 31
- [ ] Quarterly payroll tax filings current
- [ ] Sales tax filings current
- [ ] Estimated tax payments on schedule
- [ ] Annual return filed / extended
- [ ] State returns filed for all nexus states
- [ ] Fixed asset register updated
- [ ] R&D credit documentation maintained
- [ ] Business use of home documented (if applicable)
- [ ] Mileage logs maintained (if applicable)
- [ ] All deductions documented with receipts/records
