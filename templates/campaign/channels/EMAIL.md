# Email Channel Plan: [Campaign Name]

> **Campaign:** [Link to CAMPAIGN_BRIEF.md]
> **Email Owner:** [Name]
> **Platform:** [e.g., ConvertKit, Mailchimp, SendGrid, HubSpot]

---

## 1. Email Strategy

### Role in Campaign

- **Funnel stage:** [Consideration -> Conversion / Retention / etc.]
- **Goal:** [e.g., "Convert blog subscribers into trial users"]
- **Target metric:** [e.g., "15% email-to-trial conversion rate"]

### Audience Segments

| Segment | List Size | Source | Description |
|---|---|---|---|
| [Segment 1] | [X] | [Where the list came from] | [Who these people are] |
| [Segment 2] | [X] | | |
| [Segment 3] | [X] | | |

### Suppression List

- [ ] Previous unsubscribes excluded
- [ ] Hard bounces excluded
- [ ] Recent purchasers/converters excluded (if applicable)
- [ ] Complaint addresses excluded
- [ ] Competitor domains excluded (if applicable)

---

## 2. Email Sequence

### Sequence Overview

```
[Trigger Event] -> Email 1 (Day 0) -> [Wait] -> Email 2 (Day 3) -> [Wait] -> Email 3 (Day 7) -> ...
                       |                             |                              |
                       v                             v                              v
                 [If clicks: fast track]      [If opens, no click:       [If no open: re-send
                                               different CTA]            with new subject]
```

### Email Schedule

| # | Email Name | Purpose | Send Trigger | Day | Time | Segment |
|---|---|---|---|---|---|---|
| 1 | [Launch Announcement] | Awareness + CTA | Campaign launch | Day 0 | [Time TZ] | All |
| 2 | [Feature Deep Dive] | Education | 3 days after E1 | Day 3 | [Time TZ] | Opened E1 |
| 3 | [Social Proof] | Trust building | 3 days after E2 | Day 7 | [Time TZ] | Opened E1 or E2 |
| 4 | [Objection Handling] | Conversion | 3 days after E3 | Day 10 | [Time TZ] | Clicked but not converted |
| 5 | [Last Chance / Urgency] | Final push | 4 days after E4 | Day 14 | [Time TZ] | Not yet converted |

### Non-Opener Re-Send Strategy

- **Re-send window:** [X days after original send]
- **New subject line:** [Yes — different angle]
- **Same body:** [Yes/No]

---

## 3. Email Content

### Email 1: [Launch Announcement]

| Field | Content |
|---|---|
| **Subject Line A** | [Variant A — benefit focused] |
| **Subject Line B** | [Variant B — curiosity driven] |
| **Preview Text** | [90 chars — extends the subject line story] |
| **From** | [Name <email@domain.com>] |
| **Reply-To** | [email@domain.com] |

**Body Structure:**
1. Personal greeting with {{first_name}}
2. Hook — connect to their problem (2 sentences)
3. Announce the solution/offer (2-3 sentences)
4. Key benefit with proof point
5. Clear CTA button
6. P.S. with secondary hook or urgency

**CTA:** [Button text] -> [URL with UTMs]

### Email 2: [Feature Deep Dive]

| Field | Content |
|---|---|
| **Subject Line A** | |
| **Subject Line B** | |
| **Preview Text** | |

**Body Structure:**
1. Reference previous email or their interest
2. Deep dive into one key feature/benefit
3. How-to or use case example
4. CTA to try it
5. Social proof element

### Email 3-5: [Continue for each email]

---

## 4. Automation Rules

### Trigger-Based Emails

| Trigger | Action | Delay | Email |
|---|---|---|---|
| Form submission on landing page | Enter nurture sequence | Immediate | Email 1 |
| Trial signup | Move to onboarding sequence | Immediate | [Exit this campaign sequence] |
| Clicked CTA but didn't convert | Send targeted follow-up | 24 hours | Objection handler |
| No opens after 2 emails | Re-engage with new subject | 5 days | Re-engagement email |
| Unsubscribe | Remove from all campaign sends | Immediate | — |

### Exit Conditions

| Condition | Action |
|---|---|
| Converts (starts trial / purchases) | Exit campaign sequence; enter onboarding |
| Unsubscribes | Remove from all lists; log |
| Hard bounce | Remove permanently |
| Completes full sequence without action | Move to long-term nurture |

---

## 5. Testing Plan

| Test | Variable | Variants | Sample Size | Metric |
|---|---|---|---|---|
| Email 1 subject | Subject line | A: [benefit] / B: [curiosity] | [X per variant] | Open rate |
| Email 1 CTA | Button text | A: [Start Trial] / B: [See Demo] | [X] | Click rate |
| Email 3 social proof | Testimonial vs data | A: [quote] / B: [statistic] | [X] | Click rate |
| Send time | Time of day | A: [9am] / B: [2pm] | [X] | Open rate |

---

## 6. Deliverability Checklist

- [ ] SPF record configured for sending domain
- [ ] DKIM signing enabled
- [ ] DMARC policy set
- [ ] Sending domain warmed up (if new — plan X weeks warmup)
- [ ] List hygiene performed (remove inactive >12 months)
- [ ] Spam score checked for all emails (<5.0 on SpamAssassin)
- [ ] Text-to-image ratio is reasonable (not image-heavy)
- [ ] Unsubscribe link is visible and one-click
- [ ] Physical address included in footer (CAN-SPAM)
- [ ] Reply-to address is monitored

---

## 7. Performance Tracking

| Email | Sent | Delivered | Opens | Open Rate | Clicks | CTR | Conversions | Conv Rate | Unsubs |
|---|---|---|---|---|---|---|---|---|---|
| Email 1 | | | | | | | | | |
| Email 2 | | | | | | | | | |
| Email 3 | | | | | | | | | |
| Email 4 | | | | | | | | | |
| Email 5 | | | | | | | | | |
| **Total** | | | | | | | | | |

---

## Related Documents

- [COPY.md](../COPY.md) — Full email copy
- [AUDIENCE.md](../AUDIENCE.md) — Segment details
- [TRACKING.md](../TRACKING.md) — UTM parameters for email links
- [TESTING.md](../TESTING.md) — A/B test methodology
