# Campaign Testing Plan: [Campaign Name]

> **Campaign:** [Link to CAMPAIGN_BRIEF.md]
> **Testing Lead:** [Name]
> **Status:** Planning | Active | Complete

---

## 1. Testing Strategy Overview

### Testing Philosophy for This Campaign

- **Test budget:** [X% of campaign budget / $X dedicated to testing]
- **Statistical approach:** [Frequentist / Bayesian]
- **Minimum confidence level:** [95% / 90%]
- **Minimum detectable effect (MDE):** [X% improvement]
- **Maximum simultaneous tests:** [X per channel to avoid interaction effects]

### Testing Priorities

| Priority | What to Test | Expected Impact | Effort |
|---|---|---|---|
| 1 | [e.g., Landing page headline] | High | Low |
| 2 | [e.g., Email subject lines] | High | Low |
| 3 | [e.g., Ad creative imagery] | Medium | Medium |
| 4 | [e.g., CTA button text/color] | Medium | Low |
| 5 | [e.g., Pricing page layout] | High | High |

---

## 2. A/B Tests

### Test 1: [Test Name — e.g., "Landing Page Headline"]

| Field | Detail |
|---|---|
| **Hypothesis** | If we [change X], then [metric Y] will [improve/increase] because [reason]. |
| **Channel** | [Where this test runs: landing page, email, ads] |
| **Metric** | [Primary metric: conversion rate, CTR, etc.] |
| **Secondary Metrics** | [Bounce rate, time on page, downstream conversion] |
| **Current Baseline** | [Current performance of the control, if known] |
| **Target Improvement** | [X% improvement over control] |
| **Traffic Split** | [50/50, 80/20, etc.] |
| **Sample Size Needed** | [Calculated minimum for statistical significance] |
| **Estimated Duration** | [Days/weeks to reach sample size] |
| **Start Date** | [Date] |
| **Decision Date** | [Date — when we call the winner] |

**Variants:**

| Variant | Description | Rationale |
|---|---|---|
| **Control (A)** | [Current/default version] | Baseline |
| **Variant B** | [What's different] | [Why we think this might win] |
| **Variant C** | [What's different] (optional) | [Why we think this might win] |

**Results:**

| Variant | Sample Size | Conversions | Conversion Rate | Confidence | Winner? |
|---|---|---|---|---|---|
| Control (A) | | | | | |
| Variant B | | | | | |
| Variant C | | | | | |

**Decision:** [Pending / Variant X wins / Inconclusive — need more data / No significant difference]

**Learnings:** [What did we learn? How does this inform future tests?]

---

### Test 2: [Test Name — e.g., "Email Subject Line"]

| Field | Detail |
|---|---|
| **Hypothesis** | |
| **Channel** | Email |
| **Metric** | Open rate |
| **Secondary Metrics** | Click rate, conversion |
| **Traffic Split** | [e.g., 25/25/50 — test on 50%, send winner to remaining 50%] |
| **Sample Size Needed** | |
| **Start Date** | |

**Variants:**

| Variant | Subject Line | Rationale |
|---|---|---|
| **A** | [Subject line] | [Benefit-focused] |
| **B** | [Subject line] | [Curiosity-driven] |
| **C** | [Subject line] | [Social proof] |

**Results:**

| Variant | Sends | Opens | Open Rate | Clicks | CTR | Winner? |
|---|---|---|---|---|---|---|
| A | | | | | | |
| B | | | | | | |
| C | | | | | | |

---

### Test 3: [Test Name — e.g., "Ad Creative"]

| Field | Detail |
|---|---|
| **Hypothesis** | |
| **Channel** | [Ad platform] |
| **Metric** | CTR / CPA |
| **Traffic Split** | [Even rotation] |
| **Budget per variant** | [$X] |
| **Minimum spend before decision** | [$X or X days] |

**Variants:**

| Variant | Creative Description | CTA | Targeting |
|---|---|---|---|
| **A** | [Description] | [CTA] | [Same targeting] |
| **B** | [Description] | [CTA] | [Same targeting] |

---

### Test 4: [Test Name]

<!-- Add more tests as needed using the structure above -->

---

## 3. Multivariate Tests (if applicable)

### MVT 1: [Test Name — e.g., "Landing Page Optimization"]

**Variables being tested simultaneously:**

| Variable | Variant 1 | Variant 2 |
|---|---|---|
| Headline | [Version A] | [Version B] |
| Hero image | [Version A] | [Version B] |
| CTA button | [Version A] | [Version B] |

**Combinations:** [2x2x2 = 8 combinations]
**Traffic required per combination:** [X visitors minimum]
**Total traffic needed:** [X visitors]
**Estimated duration:** [X weeks]

---

## 4. Pre-Launch Testing Checklist

### Technical QA

- [ ] All links work and point to correct destinations
- [ ] Forms submit correctly and data reaches CRM/database
- [ ] Email renders correctly in top 5 email clients (Gmail, Outlook, Apple Mail, Yahoo, mobile)
- [ ] Landing page loads in <3 seconds on mobile
- [ ] Landing page renders correctly on mobile, tablet, desktop
- [ ] Tracking pixels fire correctly (verify in browser dev tools)
- [ ] UTM parameters pass through correctly
- [ ] Conversion tracking fires on thank-you/confirmation page
- [ ] A/B test tool is splitting traffic correctly
- [ ] Retargeting pixels are configured
- [ ] GDPR/cookie consent banner functions correctly
- [ ] Unsubscribe links work in all emails
- [ ] Social sharing meta tags (OG tags) display correctly

### Content QA

- [ ] All copy proofread — no typos, grammar errors, broken formatting
- [ ] Brand guidelines followed (logo, colors, fonts, tone)
- [ ] Legal disclaimers present where required
- [ ] Pricing is accurate and current
- [ ] Dates and deadlines are correct
- [ ] All images have alt text
- [ ] Video captions are accurate
- [ ] Phone numbers / emails are correct and monitored

---

## 5. Testing Tools

| Tool | Purpose | Owner | Access |
|---|---|---|---|
| [e.g., Google Optimize / VWO] | Landing page A/B tests | [Name] | [URL] |
| [e.g., Email platform] | Email subject line tests | [Name] | [URL] |
| [e.g., Ad platform native] | Ad creative tests | [Name] | [URL] |
| [e.g., Sample size calculator] | Pre-test planning | [Name] | [URL] |

---

## 6. Test Results Summary

<!-- Fill in after campaign completes -->

| Test | Winner | Improvement | Confidence | Applied To Campaign? | Insight |
|---|---|---|---|---|---|
| [Test 1] | [Variant] | [+X%] | [X%] | [Yes/No] | [Learning] |
| [Test 2] | [Variant] | [+X%] | [X%] | [Yes/No] | [Learning] |
| [Test 3] | [Variant] | [+X%] | [X%] | [Yes/No] | [Learning] |

### Insights for Future Campaigns

1. [Key learning that should inform future campaign creative/copy/targeting]
2. [Key learning]
3. [Key learning]

---

## Related Documents

- [COPY.md](COPY.md) — Copy variants being tested
- [TRACKING.md](TRACKING.md) — How test results are measured
- [PERFORMANCE.md](PERFORMANCE.md) — Impact of test winners on campaign performance
- [STRATEGY.md](STRATEGY.md) — Testing informs strategy optimization
