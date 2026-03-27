# Expense Tracking Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24

---

## EXPENSE TRACKING — {{COMPANY_NAME}}

**Period:** {{TRACKING_PERIOD}}
**Currency:** {{CURRENCY}}
**Maintained by:** {{OWNER}}

---

### 1. EXPENSE LOG

| Date | Vendor/Payee | Category | Description | Amount | Payment Method | Receipt? | Approved By | Project/Dept | Tax Deductible? |
|------|-------------|----------|-------------|--------|---------------|----------|-------------|-------------|----------------|
| {{DATE}} | {{VENDOR}} | {{CATEGORY}} | {{DESCRIPTION}} | ${{AMOUNT}} | {{METHOD}} | {{Y/N}} | {{APPROVER}} | {{PROJECT}} | {{Y/N}} |

---

### 2. EXPENSE CATEGORIES

| Category Code | Category | Budget (Monthly) | Typical Vendors | Approval Threshold |
|--------------|----------|-----------------|----------------|-------------------|
| PAY | Payroll & Benefits | ${{PAY_BUDGET}} | ADP, Gusto | Per comp plan |
| CON | Contractors | ${{CON_BUDGET}} | Various | ${{CON_THRESHOLD}} |
| HOST | Hosting / Cloud | ${{HOST_BUDGET}} | AWS, GCP, Azure | ${{HOST_THRESHOLD}} |
| SOFT | Software / SaaS | ${{SOFT_BUDGET}} | Various | ${{SOFT_THRESHOLD}} |
| MKT | Marketing | ${{MKT_BUDGET}} | Google, Meta | ${{MKT_THRESHOLD}} |
| RENT | Rent / Office | ${{RENT_BUDGET}} | Landlord | Per lease |
| TRAV | Travel | ${{TRAV_BUDGET}} | Airlines, hotels | ${{TRAV_THRESHOLD}} |
| PROF | Professional Services | ${{PROF_BUDGET}} | Law firms, CPAs | ${{PROF_THRESHOLD}} |
| INS | Insurance | ${{INS_BUDGET}} | Various | Per policy |
| HW | Hardware / Equipment | ${{HW_BUDGET}} | Apple, Dell | ${{HW_THRESHOLD}} |
| MISC | Miscellaneous | ${{MISC_BUDGET}} | Various | ${{MISC_THRESHOLD}} |

---

### 3. MONTHLY SUMMARY

| Category | Budget | Actual | Variance | % of Total | Notes |
|----------|--------|--------|----------|-----------|-------|
| Payroll & Benefits | | | | | |
| Contractors | | | | | |
| Hosting / Cloud | | | | | |
| Software / SaaS | | | | | |
| Marketing | | | | | |
| Rent / Office | | | | | |
| Travel | | | | | |
| Professional Services | | | | | |
| Insurance | | | | | |
| Hardware / Equipment | | | | | |
| Miscellaneous | | | | | |
| **Total** | | | | 100% | |

---

### 4. RECURRING EXPENSES

| Vendor | Service | Amount | Frequency | Category | Auto-Pay? | Contract End | Cancel Notice |
|--------|---------|--------|-----------|----------|-----------|-------------|--------------|
| {{VENDOR_1}} | {{SERVICE_1}} | ${{AMOUNT_1}} | {{FREQ_1}} | {{CAT_1}} | {{Y/N}} | {{END_1}} | {{NOTICE_1}} |
| {{VENDOR_2}} | {{SERVICE_2}} | ${{AMOUNT_2}} | {{FREQ_2}} | {{CAT_2}} | {{Y/N}} | {{END_2}} | {{NOTICE_2}} |

**Total monthly recurring:** ${{TOTAL_RECURRING}}
**Total annual recurring:** ${{TOTAL_ANNUAL_RECURRING}}

---

### 5. EXPENSE POLICY

#### 5.1 Approval Thresholds

| Amount | Approval Required |
|--------|------------------|
| < ${{THRESHOLD_1}} | No approval needed (within budget) |
| ${{THRESHOLD_1}} - ${{THRESHOLD_2}} | Manager approval |
| ${{THRESHOLD_2}} - ${{THRESHOLD_3}} | Director approval |
| > ${{THRESHOLD_3}} | Executive approval |

#### 5.2 Reimbursement Rules

- Receipts required for all expenses over ${{RECEIPT_THRESHOLD}}
- Expense reports due within {{REPORT_DEADLINE}} of expenditure
- Reimbursement processed within {{REIMBURSEMENT_TIMELINE}}
- Mileage rate: ${{MILEAGE_RATE}}/mile (IRS standard rate)
- Per diem: ${{PER_DIEM}}/day (or actual with receipts)
- International travel requires pre-approval

#### 5.3 Credit Card Policy

- Corporate cards issued to: {{CARD_ELIGIBILITY}}
- Monthly limit: ${{CARD_LIMIT}}
- Personal use prohibited
- Statements reconciled {{RECONCILIATION_FREQUENCY}}

---

### 6. TAX-RELEVANT CATEGORIZATION

| Expense | Tax Treatment | Notes |
|---------|-------------|-------|
| Business meals | 50% deductible | Must document business purpose and attendees |
| Travel | 100% deductible | Must be business-related |
| Office supplies | 100% deductible | |
| Equipment (< ${{DEMINIMIS_THRESHOLD}}) | 100% expensed | De minimis safe harbor |
| Equipment (>= ${{DEMINIMIS_THRESHOLD}}) | Capitalized / depreciated | {{DEPRECIATION_METHOD}} |
| Software subscriptions | 100% deductible | |
| Home office | Simplified: $5/sq ft up to 300 sq ft | Or actual expense method |
| Health insurance | Deductible | Self-employed: above-the-line deduction |
| Retirement contributions | Deductible | Up to limits |
| Entertainment | Not deductible | Post-TCJA |

---

### 7. VENDOR MANAGEMENT

| Vendor | Annual Spend | Contract Status | Last Reviewed | Next Review | Owner |
|--------|-------------|----------------|---------------|------------|-------|
| {{VENDOR_1}} | ${{SPEND_1}} | {{STATUS_1}} | {{REVIEWED_1}} | {{NEXT_1}} | {{OWNER_1}} |
| {{VENDOR_2}} | ${{SPEND_2}} | {{STATUS_2}} | {{REVIEWED_2}} | {{NEXT_2}} | {{OWNER_2}} |

**Top 10 vendors by spend:** Review quarterly for negotiation opportunities.
