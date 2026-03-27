---
title: Pre-Launch Checklist
last_updated: [YYYY-MM-DD]
project: [Project Name]
target_launch_date: [YYYY-MM-DD]
checklist_owner: [Name]
---

# Pre-Launch Checklist

> Comprehensive verification before going live. Every item must be checked. No exceptions.

## 1. Content

- [ ] All placeholder/lorem ipsum text replaced with final copy
- [ ] All pages proofread for spelling and grammar
- [ ] All headings follow proper hierarchy (H1 → H2 → H3)
- [ ] Contact information is correct (email, phone, address)
- [ ] Copyright year is current ([Current Year])
- [ ] Legal pages present and reviewed: Privacy Policy, Terms of Service, Cookie Policy
- [ ] Social media links point to correct profiles
- [ ] Team bios and photos are current and approved
- [ ] Pricing information is accurate and current
- [ ] All downloadable files (PDFs, etc.) are present and correct
- [ ] No internal notes, TODO comments, or debug text visible

---

## 2. Links & Navigation

- [ ] All internal links work (no 404s) — run full site crawl
- [ ] All external links work and open in new tab where appropriate
- [ ] Navigation works on all pages (desktop and mobile)
- [ ] Logo links to homepage from every page
- [ ] Breadcrumbs work correctly on all pages
- [ ] Footer links are all functional
- [ ] Pagination works (blog, resource lists)
- [ ] Search functionality works (if applicable)
- [ ] 404 page exists and is helpful (includes navigation, search, key links)
- [ ] No broken anchor links (#section links)

---

## 3. Forms

- [ ] All forms submit successfully
- [ ] Form validation works (required fields, email format, etc.)
- [ ] Error messages are clear and helpful
- [ ] Success/confirmation messages display correctly
- [ ] Form submissions arrive at the correct destination (email, CRM, etc.)
- [ ] Auto-response emails send correctly (if configured)
- [ ] Spam protection active (honeypot, reCAPTCHA, rate limiting)
- [ ] Forms work on mobile
- [ ] Form data is stored/forwarded correctly
- [ ] Double opt-in works (if applicable for email signups)

---

## 4. SEO

- [ ] Every page has a unique, descriptive title tag (50-60 chars)
- [ ] Every page has a unique meta description (120-160 chars)
- [ ] Every page has exactly one H1
- [ ] All images have alt text
- [ ] XML sitemap generated and accessible at /sitemap.xml
- [ ] robots.txt is correct (allows crawling, references sitemap)
- [ ] Canonical tags present on all pages (self-referencing)
- [ ] Open Graph tags set for all pages (title, description, image)
- [ ] Twitter Card tags set for all pages
- [ ] Structured data validates (Google Rich Results Test)
- [ ] Google Search Console property verified
- [ ] Bing Webmaster Tools verified (optional but recommended)
- [ ] No accidental noindex tags on production pages
- [ ] Redirects configured (if migration — see REDIRECT_MAP.md)
- [ ] Old sitemap URLs redirect correctly (if migration)

---

## 5. Analytics & Tracking

- [ ] Analytics tool installed and receiving data (verify with real-time view)
- [ ] All custom events firing correctly (test each one)
- [ ] Goals/conversions configured and tracking
- [ ] Google Tag Manager working (if used)
- [ ] UTM parameters tracking correctly
- [ ] Internal IP filtered from analytics
- [ ] Cross-domain tracking configured (if applicable)
- [ ] Cookie consent banner works and respects choices
- [ ] Analytics only fires after consent (for consent-required regions)
- [ ] Search Console linked to analytics
- [ ] Dashboards created and accessible to team

---

## 6. Performance

- [ ] PageSpeed Insights score: Mobile ≥ [Target, e.g., 90], Desktop ≥ [Target, e.g., 95]
- [ ] LCP ≤ 2.5s on all page types
- [ ] CLS ≤ 0.1 on all page types
- [ ] INP ≤ 200ms on all page types
- [ ] Total page weight within budget (see PERFORMANCE_BUDGET.md)
- [ ] Images optimized (WebP/AVIF, compressed, responsive)
- [ ] Fonts optimized (WOFF2, subset, preloaded)
- [ ] No render-blocking resources
- [ ] Lazy loading working for below-fold images
- [ ] CDN active and caching correctly
- [ ] Gzip/Brotli compression enabled

---

## 7. Mobile & Responsive

- [ ] All pages tested on mobile viewport (375px width minimum)
- [ ] All pages tested on tablet viewport
- [ ] Touch targets are 44x44px minimum
- [ ] No horizontal scrolling on any page
- [ ] Mobile navigation works smoothly
- [ ] Forms are usable on mobile (appropriate keyboard types, autocomplete)
- [ ] Text is readable without zooming (16px minimum body text)
- [ ] Images scale correctly on all viewports
- [ ] No content hidden on mobile that should be visible
- [ ] Tested on actual devices (not just browser emulation):
  - [ ] iOS Safari
  - [ ] Android Chrome

---

## 8. Browser Compatibility

- [ ] Chrome (latest 2 versions)
- [ ] Firefox (latest 2 versions)
- [ ] Safari (latest 2 versions)
- [ ] Edge (latest 2 versions)
- [ ] Safari iOS (latest 2 versions)
- [ ] Chrome Android (latest 2 versions)
- [ ] No critical layout breaks in any supported browser
- [ ] JavaScript functionality works in all supported browsers
- [ ] Fonts render correctly across browsers

---

## 9. Accessibility (WCAG 2.1 AA)

- [ ] Automated accessibility scan run (e.g., axe, Lighthouse, WAVE)
- [ ] All images have meaningful alt text (decorative images: alt="")
- [ ] Color contrast meets 4.5:1 for text, 3:1 for large text and UI
- [ ] All interactive elements are keyboard accessible (Tab, Enter, Escape)
- [ ] Focus indicators visible on all interactive elements
- [ ] Skip navigation link present
- [ ] Heading hierarchy is logical (no skipped levels)
- [ ] Form inputs have associated labels
- [ ] Error messages are programmatically associated with form fields
- [ ] ARIA attributes used correctly (no ARIA is better than bad ARIA)
- [ ] Page language set (`<html lang="en">`)
- [ ] No content flashes more than 3 times per second
- [ ] Video has captions/subtitles (if applicable)
- [ ] Screen reader tested on at least one page flow

---

## 10. Security

- [ ] HTTPS enforced on all pages (HTTP → HTTPS redirect)
- [ ] SSL certificate valid and not expiring within 30 days
- [ ] No mixed content warnings (HTTP resources on HTTPS pages)
- [ ] Security headers configured (CSP, X-Frame-Options, HSTS, etc.)
- [ ] No sensitive data in client-side code (API keys, secrets)
- [ ] Form submissions use HTTPS
- [ ] Dependencies up to date (no known vulnerabilities)
- [ ] Admin/staging URLs not publicly accessible
- [ ] File upload restrictions in place (if applicable)

---

## 11. Legal & Compliance

- [ ] Privacy Policy published and linked in footer
- [ ] Terms of Service published and linked in footer
- [ ] Cookie Policy published (if using cookies)
- [ ] Cookie consent banner implemented and functional
- [ ] Cookie consent respects user choice (no tracking before consent)
- [ ] Contact information accessible (email, form, or physical address)
- [ ] Accessibility statement published (recommended for WCAG compliance)
- [ ] GDPR compliance verified (if applicable — see COMPLIANCE_CHECKLIST.md)
- [ ] CCPA compliance verified (if applicable)

---

## 12. Infrastructure

- [ ] Production DNS configured correctly
- [ ] Domain pointing to correct hosting
- [ ] CDN configured and cache warming complete
- [ ] SSL certificate active on production domain
- [ ] Email routing works (@domain.com)
- [ ] Uptime monitoring configured and alerting
- [ ] Error monitoring active (if applicable)
- [ ] Backup strategy confirmed
- [ ] Staging environment preserved (for post-launch fixes)
- [ ] Deployment pipeline tested (can deploy fixes quickly)

---

## 13. Final Verification

- [ ] All sections above reviewed and checked
- [ ] Stakeholder sign-off obtained
- [ ] Launch plan reviewed (see LAUNCH_PLAN.md)
- [ ] Team briefed on launch timeline
- [ ] Rollback plan documented (how to revert if critical issues)
- [ ] Post-launch monitoring plan in place

**Checklist completed by:** [Name] — [Date]
**Approved for launch by:** [Name] — [Date]

---

## Related Documents

- [Launch Plan](LAUNCH_PLAN.md)
- [Performance Budget](../technical/PERFORMANCE_BUDGET.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md)
- [Technical SEO Audit](../seo/TECHNICAL_SEO_AUDIT.md)
- [Hosting Spec](../technical/HOSTING_SPEC.md)
