---
title: Website Maintenance Plan
last_updated: [YYYY-MM-DD]
project: [Project Name]
site_url: [https://example.com]
---

# Website Maintenance Plan

> Ongoing maintenance schedule, procedures, and responsibilities for the live website.

## 1. Maintenance Overview

| Property | Value |
|---|---|
| Site | [https://example.com] |
| Hosting platform | [e.g., Vercel / Netlify / etc.] |
| CMS | [e.g., Astro + Markdown / WordPress / Headless CMS] |
| Maintenance owner | [Name/role] |
| Emergency contact | [Name — phone/email for urgent issues] |

---

## 2. Scheduled Maintenance

### Weekly

| Task | Description | Owner | Day |
|---|---|---|---|
| Uptime review | Check uptime monitor for any incidents | [Name] | Monday |
| Analytics review | Review weekly traffic, conversions, anomalies | [Name] | Monday |
| Form test | Submit a test through each form, verify delivery | [Name] | Wednesday |
| Content review | Check for any content that needs updating | [Name] | Friday |
| Security scan | Review dependency alerts, security advisories | [Name] | Friday |

### Monthly

| Task | Description | Owner | Week |
|---|---|---|---|
| Performance check | Run PageSpeed Insights on key pages, compare to budget | [Name] | Week 1 |
| Broken link scan | Full site crawl for 404s (Screaming Frog / similar) | [Name] | Week 1 |
| SEO review | Check GSC for errors, keyword ranking changes | [Name] | Week 2 |
| Content update | Update any time-sensitive content (dates, stats, team) | [Name] | Week 2 |
| Dependency update | Update packages/dependencies, test, deploy | [Name] | Week 3 |
| Backup verification | Verify backups are running and restorable | [Name] | Week 4 |
| SSL certificate check | Verify certificate validity (>30 days remaining) | [Name] | Week 4 |

### Quarterly

| Task | Description | Owner | Month |
|---|---|---|---|
| Content audit | Review all pages for accuracy, relevance, and freshness | [Name] | [Month] |
| Technical SEO audit | Run full technical SEO audit (see TECHNICAL_SEO_AUDIT.md) | [Name] | [Month] |
| Performance deep-dive | Full performance audit, compare to budget, optimize | [Name] | [Month] |
| Accessibility review | Run automated + manual accessibility checks | [Name] | [Month] |
| Analytics review | Deep-dive into traffic trends, conversion paths, drop-offs | [Name] | [Month] |
| Competitive review | Check competitor sites for changes, new features | [Name] | [Month] |
| Third-party audit | Review all integrations — still needed? Still working? | [Name] | [Month] |

### Annually

| Task | Description | Owner | Month |
|---|---|---|---|
| Full website audit | Comprehensive audit (see ANNUAL_AUDIT.md) | [Name] | [Month] |
| Domain renewal verification | Ensure domain auto-renew is active | [Name] | [Month] |
| Legal content review | Privacy policy, terms, cookie policy reviewed by legal | [Name] | [Month] |
| Strategy review | Review website against business goals, plan updates | [Name] | [Month] |
| CMS/platform evaluation | Is current platform still the best choice? | [Name] | [Month] |

---

## 3. Content Update Procedures

### Blog Posts

| Step | Procedure |
|---|---|
| 1 | Draft post in CMS or markdown |
| 2 | SEO review: keyword, meta title, meta description, alt text |
| 3 | Peer review for accuracy and quality |
| 4 | Add internal links (to and from existing content) |
| 5 | Publish and verify on production |
| 6 | Share on social channels |
| 7 | Verify analytics tracking on new post |

### Page Updates

| Step | Procedure |
|---|---|
| 1 | Make changes in CMS or code |
| 2 | Review on staging/preview |
| 3 | Verify SEO elements unchanged (or intentionally updated) |
| 4 | Deploy to production |
| 5 | Verify on production |
| 6 | Update CONTENT_INVENTORY.md if structure changed |

---

## 4. Security Update Procedures

| Trigger | Procedure | Timeline |
|---|---|---|
| Critical vulnerability in dependency | Update immediately, test, deploy | Within 24 hours |
| Non-critical dependency update | Batch with monthly dependency update | Within 30 days |
| CMS security patch | Apply patch, test, deploy | Within 48 hours |
| SSL certificate expiring | Renew (usually auto-renew, verify) | Before expiry |
| Suspicious traffic / DDoS | Enable rate limiting, review WAF rules | Immediate |

---

## 5. Uptime & Monitoring

| Monitor | Tool | Check Frequency | Alert Channel | Alertee |
|---|---|---|---|---|
| Uptime | [e.g., UptimeRobot / Pingdom] | [Every 1-5 minutes] | [Email/SMS/Slack] | [Name] |
| SSL certificate | [e.g., UptimeRobot SSL check] | [Daily] | [Email] | [Name] |
| Domain expiry | [e.g., Registrar notifications] | [Monthly] | [Email] | [Name] |
| Performance | [e.g., SpeedCurve / Lighthouse CI] | [Per deploy + weekly] | [Slack/email] | [Name] |
| Errors (JS) | [e.g., Sentry / none] | [Real-time] | [Slack/email] | [Dev] |

---

## 6. Backup & Disaster Recovery

| Item | Backup Method | Frequency | Retention | Recovery Time |
|---|---|---|---|---|
| Source code | Git (GitHub/GitLab) | Every commit | Permanent | Minutes |
| CMS content | [Export/API backup / DB dump] | [Daily/Weekly] | [30 days] | [Time estimate] |
| Media assets | [CDN + backup storage] | [Per upload + weekly sync] | [30 days] | [Time estimate] |
| DNS configuration | [Exported zone file] | [After changes] | [Keep latest] | [Minutes] |
| Analytics data | [Platform retains — not backed up] | N/A | [Per platform retention] | N/A |

### Recovery Procedure

| Scenario | Steps | RTO |
|---|---|---|
| Site down (hosting issue) | 1. Check hosting status page. 2. Contact hosting support. 3. If extended: point DNS to backup. | [e.g., 1 hour] |
| Accidental content deletion | 1. Restore from Git history or CMS backup. 2. Redeploy. | [e.g., 30 minutes] |
| Domain hijacking | 1. Contact registrar. 2. Lock domain. 3. DNS recovery. | [e.g., 4-24 hours] |
| Malware / defacement | 1. Take site offline. 2. Restore from clean backup. 3. Investigate vector. 4. Patch and redeploy. | [e.g., 2-4 hours] |

---

## 7. Related Documents

- [Hosting Spec](../technical/HOSTING_SPEC.md)
- [Annual Audit](ANNUAL_AUDIT.md)
- [Performance Budget](../technical/PERFORMANCE_BUDGET.md)
- [Technical SEO Audit](../seo/TECHNICAL_SEO_AUDIT.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md)
