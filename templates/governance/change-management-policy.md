# Change Management Policy Template

<!--
  CLASSIFICATION: POLICY
  Use this template for defining change management policies — how changes to systems,
  processes, configurations, or governance are proposed, reviewed, approved, and implemented.
  Complements governance/BOT_CHANGE_MANAGEMENT.md with broader organizational scope.
  24-hour cooling period after Trio approval.
-->

---

## Frontmatter

| Field | Value |
|-------|-------|
| **Policy ID** | POLICY-NNN |
| **Title** | _[e.g., "Infrastructure Change Management Policy"]_ |
| **Created** | YYYY-MM-DD |
| **Updated** | YYYY-MM-DD |
| **Owner** | _[Named officer — responsible for enforcement]_ |
| **Approved By** | _[Trio]_ |
| **Effective Date** | YYYY-MM-DD |
| **Review Cycle** | Quarterly |
| **Next Review** | YYYY-MM-DD |
| **Scope** | _[What domain this policy covers — infrastructure, bots, governance, operations]_ |

---

## Purpose

_Why does this change management policy exist? What risks does unmanaged change create in this domain?_

---

## Change Categories

| Category | Definition | Examples | Risk Level |
|----------|-----------|---------|------------|
| **Standard** | Pre-approved, low-risk, reversible changes | _[Examples]_ | Low |
| **Normal** | Planned changes requiring review and approval | _[Examples]_ | Medium |
| **Major** | High-impact changes affecting multiple systems or domains | _[Examples]_ | High |
| **Emergency** | Unplanned changes to resolve active incidents | _[Examples]_ | Variable |

---

## Approval Matrix

| Change Category | Proposer | Reviewer | Approver | Notification |
|----------------|----------|----------|----------|-------------|
| Standard | Any officer | N/A (pre-approved) | Auto-approved | Log only |
| Normal | Domain officer | Peer officer | Domain lead | Domain lead + affected officers |
| Major | Domain lead | Cross-domain review | Trio | All officers |
| Emergency | Any officer | Post-hoc (within 24h) | Stream (post-hoc) | Trio within 1h |

---

## Change Request Process

### Step 1: Change Proposal

**Required fields:**
- Change title and description
- Category (standard / normal / major / emergency)
- Justification — why this change is needed
- Impact assessment — what is affected
- Rollback plan — how to reverse the change
- Testing plan — how the change will be validated
- Timeline — when the change will be implemented

### Step 2: Impact Assessment

| Impact Dimension | Assessment |
|-----------------|-----------|
| **Affected systems** | _[List all systems impacted]_ |
| **Affected officers** | _[Who needs to know or adjust]_ |
| **Downstream effects** | _[What else changes as a result]_ |
| **Reversibility** | _[fully reversible / partially reversible / irreversible]_ |
| **Downtime required** | _[Yes/No — duration if yes]_ |

### Step 3: Review and Approval

- Reviewer verifies impact assessment is complete and accurate
- Reviewer confirms rollback plan is viable
- Approver signs off based on category-appropriate authority
- For major changes: minimum 48h review period before implementation

### Step 4: Implementation

- Implement during approved change window
- Follow the testing plan
- Monitor for unexpected side effects
- Log all actions in audit trail

### Step 5: Verification

- Confirm change achieves intended outcome
- Verify no unintended side effects
- Close the change request with outcome documentation

### Step 6: Post-Implementation Review (Major changes only)

- Conducted within 7 days of implementation
- Compares actual outcomes to expected outcomes
- Documents lessons learned
- Updates change management process if needed

---

## Change Windows

| Window Type | Schedule | Allowed Categories |
|------------|----------|-------------------|
| **Standard window** | _[e.g., "Any time during business hours"]_ | Standard |
| **Scheduled window** | _[e.g., "Tuesdays and Thursdays, 02:00-06:00"]_ | Normal, Major |
| **Emergency window** | Any time | Emergency only |
| **Freeze period** | _[Define blackout periods]_ | None except Emergency |

---

## Rollback Policy

1. Every non-standard change MUST have a documented rollback plan
2. Rollback must be tested before implementation when feasible
3. Rollback decision authority: change owner for normal, Trio for major
4. Maximum time to rollback decision: _[e.g., "30 minutes after anomaly detection"]_
5. Failed changes that cannot be rolled back trigger incident response

---

## Emergency Change Protocol

1. **Act first, document within 1h** — emergency changes prioritize resolution over process
2. **Notify Trio within 1h** of the emergency change
3. **Post-hoc review within 24h** — full change documentation completed retroactively
4. **Stream approval within 24h** — Stream reviews and formally approves or requires correction
5. **Lessons learned within 7 days** — what caused the emergency, how to prevent recurrence

---

## Metrics and Reporting

| Metric | Target | Measurement | Frequency |
|--------|--------|-------------|-----------|
| Change success rate | _[e.g., "> 95%"]_ | Successful changes / total changes | Monthly |
| Emergency change ratio | _[e.g., "< 10%"]_ | Emergency changes / total changes | Monthly |
| Mean time to implement | _[Target]_ | Average implementation duration | Monthly |
| Rollback frequency | _[e.g., "< 5%"]_ | Rolled-back changes / total changes | Monthly |

---

## Related Documents

- `governance/BOT_CHANGE_MANAGEMENT.md` — bot-specific change management
- _[Related policies]_
- _[Related decision records]_

---

## Change History

| Date | Version | Author | Description |
|------|---------|--------|-------------|
| _[Date]_ | 1.0 | _[Author]_ | Initial policy |
