# Capitalization Table Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Review Cadence:** Per transaction event

---

## CAP TABLE — {{COMPANY_NAME}}

**As of:** {{AS_OF_DATE}}
**Total Authorized Shares:** {{AUTHORIZED_SHARES}}
**Prepared by:** {{AUTHOR}}

---

### 1. SHARE SUMMARY

| Share Class | Authorized | Issued | Outstanding | Reserved (Option Pool) | Available |
|------------|-----------|--------|-------------|----------------------|-----------|
| Common | {{COMMON_AUTHORIZED}} | {{COMMON_ISSUED}} | {{COMMON_OUTSTANDING}} | — | {{COMMON_AVAILABLE}} |
| Series Seed Preferred | {{SEED_AUTHORIZED}} | {{SEED_ISSUED}} | {{SEED_OUTSTANDING}} | — | {{SEED_AVAILABLE}} |
| Series A Preferred | {{SERIESA_AUTHORIZED}} | {{SERIESA_ISSUED}} | {{SERIESA_OUTSTANDING}} | — | {{SERIESA_AVAILABLE}} |
| Option Pool | {{POOL_AUTHORIZED}} | — | {{POOL_GRANTED}} | {{POOL_RESERVED}} | {{POOL_AVAILABLE}} |
| **Total** | **{{TOTAL_AUTHORIZED}}** | | **{{TOTAL_OUTSTANDING}}** | | |

---

### 2. OWNERSHIP TABLE

| Shareholder | Share Class | Shares | % Ownership (Basic) | % Ownership (Fully Diluted) | Investment | Price/Share |
|-------------|-----------|--------|--------------------|-----------------------------|-----------|------------|
| **Founders** | | | | | | |
| {{FOUNDER_1}} | Common | {{F1_SHARES}} | {{F1_BASIC}}% | {{F1_FD}}% | — | — |
| {{FOUNDER_2}} | Common | {{F2_SHARES}} | {{F2_BASIC}}% | {{F2_FD}}% | — | — |
| **Investors** | | | | | | |
| {{INVESTOR_1}} | Series Seed | {{I1_SHARES}} | {{I1_BASIC}}% | {{I1_FD}}% | ${{I1_INVESTMENT}} | ${{I1_PPS}} |
| {{INVESTOR_2}} | Series Seed | {{I2_SHARES}} | {{I2_BASIC}}% | {{I2_FD}}% | ${{I2_INVESTMENT}} | ${{I2_PPS}} |
| {{INVESTOR_3}} | Series A | {{I3_SHARES}} | {{I3_BASIC}}% | {{I3_FD}}% | ${{I3_INVESTMENT}} | ${{I3_PPS}} |
| **Employees/Advisors** | | | | | | |
| {{EMPLOYEE_1}} | Options (Common) | {{E1_OPTIONS}} | — | {{E1_FD}}% | — | ${{E1_STRIKE}} |
| {{EMPLOYEE_2}} | Options (Common) | {{E2_OPTIONS}} | — | {{E2_FD}}% | — | ${{E2_STRIKE}} |
| **Unallocated Pool** | Options | {{UNALLOCATED}} | — | {{UNALLOC_FD}}% | — | — |
| **TOTAL** | | **{{TOTAL_SHARES_FD}}** | **100%** | **100%** | **${{TOTAL_RAISED}}** | |

---

### 3. FINANCING HISTORY

| Round | Date | Pre-Money Valuation | Amount Raised | Post-Money Valuation | Price/Share | Lead Investor | Shares Issued |
|-------|------|--------------------|--------------|--------------------|------------|--------------|--------------|
| Incorporation | {{INC_DATE}} | — | — | — | ${{PAR_VALUE}} | — | {{INC_SHARES}} |
| Seed | {{SEED_DATE}} | ${{SEED_PRE}} | ${{SEED_RAISED}} | ${{SEED_POST}} | ${{SEED_PPS}} | {{SEED_LEAD}} | {{SEED_SHARES}} |
| Series A | {{SA_DATE}} | ${{SA_PRE}} | ${{SA_RAISED}} | ${{SA_POST}} | ${{SA_PPS}} | {{SA_LEAD}} | {{SA_SHARES}} |

---

### 4. OPTION POOL DETAIL

| Grant # | Recipient | Grant Date | Shares | Strike Price | Vesting Schedule | Cliff | Vested | Exercised | Unvested | Status |
|---------|-----------|-----------|--------|-------------|-----------------|-------|--------|-----------|----------|--------|
| {{GRANT_1}} | {{RECIPIENT_1}} | {{GDATE_1}} | {{GSHARES_1}} | ${{STRIKE_1}} | {{VESTING_1}} | {{CLIFF_1}} | {{VESTED_1}} | {{EXERCISED_1}} | {{UNVESTED_1}} | {{STATUS_1}} |
| {{GRANT_2}} | {{RECIPIENT_2}} | {{GDATE_2}} | {{GSHARES_2}} | ${{STRIKE_2}} | {{VESTING_2}} | {{CLIFF_2}} | {{VESTED_2}} | {{EXERCISED_2}} | {{UNVESTED_2}} | {{STATUS_2}} |

**Pool Summary:**

| Metric | Shares |
|--------|--------|
| Total pool authorized | {{POOL_TOTAL}} |
| Granted (outstanding) | {{POOL_GRANTED}} |
| Exercised | {{POOL_EXERCISED}} |
| Cancelled / returned | {{POOL_CANCELLED}} |
| Available for grant | {{POOL_AVAILABLE}} |
| Pool as % of fully diluted | {{POOL_FD_PCT}}% |

---

### 5. PREFERRED STOCK TERMS

| Term | Series Seed | Series A |
|------|------------|----------|
| Price per share | ${{SEED_PPS}} | ${{SA_PPS}} |
| Liquidation preference | {{SEED_LIQ_PREF}}x | {{SA_LIQ_PREF}}x |
| Participation | {{SEED_PARTICIPATION: Non-participating / Participating}} | {{SA_PARTICIPATION}} |
| Participation cap | {{SEED_CAP}} | {{SA_CAP}} |
| Dividend rate | {{SEED_DIVIDEND}} | {{SA_DIVIDEND}} |
| Cumulative? | {{SEED_CUMULATIVE}} | {{SA_CUMULATIVE}} |
| Conversion ratio | {{SEED_CONVERSION}} | {{SA_CONVERSION}} |
| Anti-dilution | {{SEED_ANTIDILUTION: Broad-based weighted average / Narrow-based / Full ratchet}} | {{SA_ANTIDILUTION}} |
| Voting rights | {{SEED_VOTING}} | {{SA_VOTING}} |
| Board seat | {{SEED_BOARD}} | {{SA_BOARD}} |

---

### 6. WATERFALL ANALYSIS

*How proceeds are distributed at various exit valuations.*

| Exit Valuation | ${{EXIT_1}}M | ${{EXIT_2}}M | ${{EXIT_3}}M | ${{EXIT_4}}M | ${{EXIT_5}}M |
|---------------|-------------|-------------|-------------|-------------|-------------|
| **Series A Preferred** | | | | | |
| Liquidation preference | | | | | |
| Participation (if any) | | | | | |
| Total to Series A | | | | | |
| Per share | | | | | |
| **Series Seed Preferred** | | | | | |
| Liquidation preference | | | | | |
| Participation (if any) | | | | | |
| Total to Seed | | | | | |
| Per share | | | | | |
| **Common (incl. options)** | | | | | |
| Total to Common | | | | | |
| Per share | | | | | |
| **Founder 1 payout** | | | | | |
| **Founder 2 payout** | | | | | |

---

### 7. PRO-FORMA (NEXT ROUND)

| Metric | Current | After Series {{NEXT_ROUND}} |
|--------|---------|---------------------------|
| Pre-money valuation | — | ${{NEXT_PRE_MONEY}} |
| New investment | — | ${{NEXT_RAISE}} |
| Post-money valuation | — | ${{NEXT_POST_MONEY}} |
| New shares issued | — | {{NEXT_SHARES}} |
| Price per share | — | ${{NEXT_PPS}} |
| New option pool (pre-money) | — | {{NEXT_POOL}}% |

**Pro-Forma Ownership:**

| Shareholder Group | Before Round | After Round | Dilution |
|-------------------|-------------|-------------|----------|
| Founders | {{FOUNDERS_BEFORE}}% | {{FOUNDERS_AFTER}}% | {{FOUNDERS_DILUTION}}% |
| Existing investors | {{EXISTING_BEFORE}}% | {{EXISTING_AFTER}}% | {{EXISTING_DILUTION}}% |
| New investor(s) | 0% | {{NEW_INVESTOR_PCT}}% | — |
| Option pool | {{POOL_BEFORE}}% | {{POOL_AFTER}}% | — |

---

### 8. NOTES

- 409A valuation as of {{VALUATION_DATE}}: ${{FMV_PER_SHARE}}/share
- Next 409A valuation due: {{NEXT_VALUATION_DATE}}
- Cap table maintained in: {{CAP_TABLE_TOOL: Carta / Pulley / spreadsheet / etc.}}
- Legal counsel: {{LEGAL_COUNSEL}}
