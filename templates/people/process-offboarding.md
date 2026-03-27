# Offboarding Checklist Template

**Template Version:** 1.0
**Last Updated:** 2026-03-24

---

## OFFBOARDING CHECKLIST — {{EMPLOYEE_NAME}}

| Field | Value |
|-------|-------|
| **Name** | {{EMPLOYEE_NAME}} |
| **Title** | {{JOB_TITLE}} |
| **Department** | {{DEPARTMENT}} |
| **Manager** | {{MANAGER_NAME}} |
| **Separation type** | {{SEPARATION_TYPE: Voluntary resignation / Involuntary termination / Layoff / End of contract / Retirement}} |
| **Last day of work** | {{LAST_DAY}} |
| **Notice date** | {{NOTICE_DATE}} |
| **Notice period** | {{NOTICE_PERIOD}} |

---

### IMMEDIATE (Upon Notice)

**Manager:**
- [ ] Confirm last day of work with employee and HR
- [ ] Discuss transition plan and knowledge transfer
- [ ] Identify critical work in progress and reassignment plan
- [ ] Determine if the departing employee will work through notice period
- [ ] Communicate departure to team (coordinate timing with employee)

**HR:**
- [ ] Process separation in HRIS
- [ ] Calculate final pay (including accrued PTO per policy: {{PTO_PAYOUT}})
- [ ] Prepare separation agreement (if applicable)
- [ ] Schedule exit interview
- [ ] Prepare COBRA notification
- [ ] Prepare benefits termination notice
- [ ] Review any restrictive covenants (non-compete, non-solicitation)
- [ ] Confirm any repayment obligations (signing bonus clawback, relocation, tuition)

---

### KNOWLEDGE TRANSFER (During Notice Period)

- [ ] Document all current projects and status

| Project | Status | Handed Off To | Documentation Location |
|---------|--------|--------------|----------------------|
| {{PROJECT_1}} | {{STATUS_1}} | {{HANDOFF_1}} | {{DOCS_1}} |
| {{PROJECT_2}} | {{STATUS_2}} | {{HANDOFF_2}} | {{DOCS_2}} |
| {{PROJECT_3}} | {{STATUS_3}} | {{HANDOFF_3}} | {{DOCS_3}} |

- [ ] Document processes only this employee knows
- [ ] Transfer ownership of accounts, subscriptions, and tools
- [ ] Update documentation and wikis
- [ ] Introduce replacements/backups to key contacts
- [ ] Record walkthroughs or training videos if appropriate
- [ ] Transfer customer/vendor relationships to designated successor

---

### LAST DAY

**IT / Access Revocation:**
- [ ] Disable email account ({{DISABLE_TIMING: at end of last day / immediately upon notification}})
- [ ] Disable SSO / directory account
- [ ] Revoke VPN access
- [ ] Remove from all SaaS applications:
  - [ ] {{APP_1}}
  - [ ] {{APP_2}}
  - [ ] {{APP_3}}
  - [ ] {{APP_4}}
  - [ ] {{APP_5}}
- [ ] Revoke source code repository access
- [ ] Revoke cloud infrastructure access (AWS, GCP, etc.)
- [ ] Remove from communication channels (Slack, Teams, etc.)
- [ ] Remove from distribution lists and shared calendars
- [ ] Disable building access / badge
- [ ] Change shared passwords/credentials the employee had access to
- [ ] Review and revoke any API keys or tokens
- [ ] Forward email to {{EMAIL_FORWARD_TO}} for {{EMAIL_FORWARD_PERIOD}}

**Property Return:**
- [ ] Laptop
- [ ] Monitor(s)
- [ ] Keyboard, mouse, headset
- [ ] Phone
- [ ] Building access badge / keys
- [ ] Parking pass
- [ ] Credit card / purchasing card
- [ ] Company-branded materials
- [ ] Any other Company property: {{OTHER_PROPERTY}}

**Shipping (if remote):**
- [ ] Prepaid shipping label sent
- [ ] Shipping box provided if needed
- [ ] Expected return date: {{RETURN_DATE}}
- [ ] Follow up if not received by: {{FOLLOWUP_DATE}}

---

### HR / ADMIN (Within 5 Business Days)

**Compensation:**
- [ ] Final paycheck processed (per state law: {{STATE_FINAL_PAY_REQUIREMENT}})
- [ ] Commission / bonus settlement (if applicable): {{COMMISSION_SETTLEMENT}}
- [ ] Expense reimbursement for outstanding claims
- [ ] Clawback amounts deducted (if applicable): {{CLAWBACK_AMOUNT}}

**Benefits:**
- [ ] COBRA notification sent (within 14 days)
- [ ] Health insurance termination date: {{HEALTH_END_DATE}}
- [ ] Life/disability insurance termination
- [ ] 401(k) / retirement plan notification and rollover information
- [ ] HSA/FSA balance notification
- [ ] EAP access communication (if extended post-separation)

**Legal / Compliance:**
- [ ] Separation agreement signed (if applicable)
- [ ] Remind employee of ongoing obligations:
  - [ ] Confidentiality / NDA obligations
  - [ ] Non-compete / non-solicitation (if applicable)
  - [ ] IP assignment confirmation
- [ ] Collect signed acknowledgment of post-employment obligations
- [ ] Update I-9 records (retain per requirements)

**Administrative:**
- [ ] Update org chart
- [ ] Update team directory
- [ ] Remove from company website (bio, team page)
- [ ] Update any public-facing directories
- [ ] Cancel any future travel or event registrations
- [ ] Redirect mail (physical)
- [ ] Update emergency contact lists
- [ ] File all documentation in employee's personnel file

---

### EXIT INTERVIEW

**Conducted by:** {{EXIT_INTERVIEWER}}
**Date:** {{EXIT_DATE}}

| Topic | Response |
|-------|----------|
| Reason for leaving | |
| What did you enjoy most? | |
| What would you improve? | |
| How was your relationship with your manager? | |
| Did you feel you had opportunities for growth? | |
| Would you recommend this company to others? | |
| Would you consider returning in the future? | |
| Any other feedback? | |

**Key themes:** {{EXIT_THEMES}}
**Action items from exit interview:** {{EXIT_ACTIONS}}

---

### INVOLUNTARY TERMINATION — ADDITIONAL STEPS

*Complete these only for involuntary separations.*

- [ ] Termination decision reviewed by {{REVIEW_CHAIN: HR / Legal / Manager's manager}}
- [ ] Documentation of performance issues / cause on file
- [ ] Termination meeting conducted (attendees: {{ATTENDEES}})
- [ ] Severance package offered (if applicable): {{SEVERANCE_TERMS}}
- [ ] Release agreement provided (if applicable)
- [ ] Outplacement services offered (if applicable)
- [ ] Unemployment insurance information provided
- [ ] Same-day access revocation completed

---

### COMPLETION SIGN-OFF

| Step | Owner | Completed | Date |
|------|-------|-----------|------|
| Knowledge transfer complete | Manager | [ ] | |
| IT access revoked | IT | [ ] | |
| Property returned | Employee/IT | [ ] | |
| Final pay processed | HR/Payroll | [ ] | |
| Benefits terminated | HR | [ ] | |
| Exit interview complete | HR | [ ] | |
| All documentation filed | HR | [ ] | |

**Offboarding complete:** [ ] Yes
**Completed by:** {{HR_PERSON}}
**Date:** {{COMPLETION_DATE}}
