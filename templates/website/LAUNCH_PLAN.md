---
title: Launch Plan
last_updated: [YYYY-MM-DD]
project: [Project Name]
launch_date: [YYYY-MM-DD]
launch_time: [HH:MM timezone]
---

# Launch Plan

> Step-by-step launch day execution, DNS cutover, monitoring, and post-launch verification.

## 1. Launch Overview

| Property | Value |
|---|---|
| Launch date | [YYYY-MM-DD] |
| Launch time | [e.g., "Tuesday 10:00 AM ET" — avoid Fridays, weekends] |
| Launch type | [New site / Migration / Redesign] |
| DNS cutover required | [Yes/No] |
| Expected downtime | [None / X minutes during DNS propagation] |
| Rollback plan | [Revert DNS to old hosting / restore from backup] |
| Launch lead | [Name] |
| On-call team | [Names and contact info] |

---

## 2. Pre-Launch (Day Before)

- [ ] Pre-launch checklist 100% complete (PRELAUNCH_CHECKLIST.md)
- [ ] Final content review completed and approved
- [ ] Stakeholder sign-off received
- [ ] DNS records prepared (but not yet switched)
- [ ] SSL certificate verified on new hosting
- [ ] Analytics verified working on staging/preview URL
- [ ] Team briefed on timeline and responsibilities
- [ ] Rollback procedure documented and understood by team
- [ ] Status page or maintenance notice prepared (if applicable)
- [ ] Social media / announcement posts drafted (if applicable)

---

## 3. Launch Sequence

### Phase 1: Pre-Cutover Checks (T-60 minutes)

| Time | Action | Who | Verified |
|---|---|---|---|
| T-60 min | Final check of staging/preview site | [QA] | [ ] |
| T-60 min | Verify all environment variables set in production | [Dev] | [ ] |
| T-45 min | Trigger production build and verify success | [Dev] | [ ] |
| T-30 min | Verify production site accessible via direct URL (before DNS) | [Dev] | [ ] |
| T-15 min | Notify team: launch proceeding as planned | [Lead] | [ ] |
| T-5 min | Open monitoring dashboards (analytics real-time, uptime, errors) | [Lead] | [ ] |

### Phase 2: DNS Cutover (T-0)

| Time | Action | Who | Verified |
|---|---|---|---|
| T-0 | Update DNS records to point to new hosting | [Dev/Ops] | [ ] |
| T+1 min | Verify DNS change accepted by registrar | [Dev/Ops] | [ ] |
| T+5 min | Test site from multiple locations (use whatsmydns.net) | [QA] | [ ] |
| T+5 min | Verify HTTPS working on production domain | [Dev] | [ ] |
| T+10 min | Verify redirects working (HTTP→HTTPS, www→apex) | [Dev] | [ ] |
| T+10 min | Test forms on production domain | [QA] | [ ] |
| T+15 min | Verify analytics receiving data (real-time view) | [Dev] | [ ] |

### Phase 3: Post-Cutover Verification (T+15 to T+60)

| Time | Action | Who | Verified |
|---|---|---|---|
| T+15 min | Walk through all critical user journeys on production | [QA] | [ ] |
| T+20 min | Submit sitemap to Google Search Console | [SEO] | [ ] |
| T+20 min | Verify robots.txt accessible and correct | [SEO] | [ ] |
| T+30 min | Test all forms with real submissions | [QA] | [ ] |
| T+30 min | Verify email routing works (@domain.com) | [Dev] | [ ] |
| T+45 min | Check CDN caching (response headers, cache status) | [Dev] | [ ] |
| T+60 min | Spot-check 10 pages: content, images, links, performance | [QA] | [ ] |
| T+60 min | Confirm: GO or ROLLBACK decision | [Lead] | [ ] |

---

## 4. Post-Launch Monitoring

### Day 1

| Check | Frequency | Tool | Who |
|---|---|---|---|
| Uptime | Continuous | [Uptime monitor] | [Auto-alert to team] |
| Error rate | Hourly | [Error tracking / server logs] | [Dev] |
| Analytics data flowing | Every 2 hours | [GA4 / analytics real-time] | [Dev] |
| Form submissions working | 3x during day | [Test submissions] | [QA] |
| DNS propagation complete | End of day | [whatsmydns.net] | [Dev] |
| User-reported issues | Continuous | [Email, chat, social] | [All] |

### Week 1

| Check | When | Tool | Who |
|---|---|---|---|
| Core Web Vitals (lab) | Day 2 | [PageSpeed Insights] | [Dev] |
| Search Console coverage | Day 3 | [GSC] | [SEO] |
| Organic traffic trend | Day 5 | [GA4] | [SEO] |
| Redirect verification | Day 3 | [Full crawl of old URLs] | [SEO] |
| Performance regression | Day 5 | [Lighthouse, WebPageTest] | [Dev] |
| Cross-browser issues | Day 2-3 | [Manual testing] | [QA] |

### Month 1

| Check | When | Tool | Who |
|---|---|---|---|
| Core Web Vitals (field data) | Week 4 | [CrUX / GSC] | [Dev] |
| Organic traffic vs. baseline | Week 4 | [GA4 + GSC] | [SEO] |
| Keyword rankings vs. baseline | Week 4 | [Ranking tracker] | [SEO] |
| Conversion rate | Week 4 | [GA4] | [Marketing] |
| Index coverage (all pages indexed) | Week 2-4 | [GSC] | [SEO] |
| Backlink transfer (if migration) | Week 4 | [Ahrefs / SEMrush] | [SEO] |

---

## 5. Rollback Plan

### Trigger Conditions

Rollback if ANY of these occur within 60 minutes of launch:
- [ ] Site is unreachable for >5 minutes
- [ ] Critical conversion forms are broken
- [ ] HTTPS not working
- [ ] Major content errors on homepage or key pages
- [ ] Analytics completely non-functional

### Rollback Procedure

| Step | Action | Time Required |
|---|---|---|
| 1 | Decision to rollback — approved by [Launch Lead] | Immediate |
| 2 | Revert DNS records to previous hosting | [~5 minutes] |
| 3 | Verify old site accessible | [~5 minutes + propagation] |
| 4 | Notify team of rollback | Immediate |
| 5 | Investigate root cause | ASAP |
| 6 | Schedule re-launch after fix | TBD |

---

## 6. Communication Plan

| When | Audience | Channel | Message |
|---|---|---|---|
| Pre-launch | Team | [Slack/email] | "Launching at [time]. Monitoring plan active." |
| Launch confirmed | Team | [Slack/email] | "Site is live. Monitoring in progress." |
| Post-launch all clear | Stakeholders | [Email] | "Launch successful. Key metrics nominal." |
| If rollback | Team + stakeholders | [Slack/email] | "Rollback initiated. [Reason]. ETA for re-launch: TBD." |
| Public announcement | Customers / public | [Social, blog, email] | [Announcement post] |

---

## 7. Related Documents

- [Pre-Launch Checklist](PRELAUNCH_CHECKLIST.md)
- [Hosting Spec](../technical/HOSTING_SPEC.md)
- [Redirect Map](../seo/REDIRECT_MAP.md)
- [Maintenance Plan](../operations/MAINTENANCE_PLAN.md)
- [Analytics Plan](../strategy/ANALYTICS_PLAN.md)
