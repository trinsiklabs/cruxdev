---
title: Hosting & Infrastructure Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Hosting & Infrastructure Specification

> Domain, DNS, SSL, hosting, CDN, and build/deploy configuration for the website.

## 1. Domain Configuration

### 1.1 Domains

| Domain | Purpose | Registrar | Expiry | Auto-Renew |
|---|---|---|---|---|
| [example.com] | Primary marketing site | [e.g., Cloudflare / Namecheap / Google Domains] | [YYYY-MM-DD] | [Yes/No] |
| [www.example.com] | Redirect to apex (or vice versa) | [Same registrar] | [Same] | [Yes] |
| [docs.example.com] | Documentation site | [Same] | [Same] | [Yes] |

### 1.2 DNS Configuration

| Record | Type | Name | Value | TTL | Purpose |
|---|---|---|---|---|---|
| [A] | A | @ | [IP address or alias] | [3600] | [Points apex to hosting] |
| [CNAME] | CNAME | www | [e.g., "example.com" or "alias.vercel-dns.com"] | [3600] | [www subdomain] |
| [CNAME] | CNAME | docs | [e.g., "docs-hosting-alias"] | [3600] | [Docs subdomain] |
| [MX] | MX | @ | [e.g., "mx1.emailprovider.com"] | [3600] | [Email routing] |
| [TXT] | TXT | @ | [e.g., "v=spf1 include:..."] | [3600] | [SPF record] |
| [TXT] | TXT | @ | [e.g., "google-site-verification=..."] | [3600] | [Search Console verification] |
| [CNAME] | CNAME | _dmarc | [DMARC policy] | [3600] | [Email authentication] |

### 1.3 DNS Propagation Notes

- Current nameservers: [e.g., "Cloudflare"]
- DNS management: [e.g., "Cloudflare dashboard" — access: [who has access]]
- Propagation monitoring: [e.g., "Use whatsmydns.net to verify propagation"]

---

## 2. SSL / TLS

| Property | Value |
|---|---|
| Certificate provider | [e.g., "Let's Encrypt / Cloudflare / AWS ACM"] |
| Certificate type | [e.g., "DV (Domain Validation)"] |
| Domains covered | [List all domains and subdomains on the cert] |
| Auto-renewal | [Yes — via [provider mechanism]] |
| HTTPS enforcement | [301 redirect from HTTP to HTTPS — configured at [CDN/server level]] |
| HSTS | [Enabled: max-age=31536000; includeSubDomains; preload] |
| TLS minimum version | [TLS 1.2] |

---

## 3. Hosting

### 3.1 Hosting Platform

| Property | Value |
|---|---|
| Platform | [e.g., "Vercel / Netlify / Cloudflare Pages / AWS S3+CloudFront / VPS"] |
| Plan | [e.g., "Pro / Free / Enterprise"] |
| Account | [Account owner / email] |
| Region | [e.g., "US East, with CDN for global delivery"] |
| Build command | [e.g., "npm run build" or "hugo --minify"] |
| Output directory | [e.g., "dist/" or "public/" or ".next/"] |
| Node version | [e.g., "20.x" — if applicable] |

### 3.2 Environments

| Environment | URL | Branch | Auto-deploy | Purpose |
|---|---|---|---|---|
| Production | [https://example.com] | [main] | [Yes] | [Live site] |
| Staging | [https://staging.example.com] | [staging] | [Yes] | [Pre-production review] |
| Preview | [Auto-generated per PR] | [Feature branches] | [Yes] | [PR previews] |

### 3.3 Build Configuration

| Setting | Value |
|---|---|
| Static site generator / framework | [e.g., "Astro / Hugo / Next.js / Gatsby / 11ty"] |
| Build time (approximate) | [e.g., "30 seconds"] |
| Environment variables | [List non-secret env vars needed for build] |
| Secrets management | [e.g., "Stored in Vercel environment variables, not in repo"] |

---

## 4. CDN Configuration

| Property | Value |
|---|---|
| CDN provider | [e.g., "Cloudflare / CloudFront / Vercel Edge / Fastly"] |
| Cache strategy | [e.g., "Static assets: 1 year. HTML: revalidate on deploy."] |
| Cache-Control headers | [e.g., "public, max-age=31536000, immutable" for hashed assets] |
| Purge on deploy | [Yes — automatic / manual] |
| Image optimization | [e.g., "Cloudflare Polish / Vercel Image Optimization / built-in"] |
| Edge functions | [e.g., "Redirects, A/B testing, geolocation" or "None"] |

### 4.1 Cache Rules

| Path Pattern | Cache-Control | Notes |
|---|---|---|
| `/_astro/*` or `/assets/*` | `public, max-age=31536000, immutable` | [Hashed static assets] |
| `/*.html` | `public, max-age=0, must-revalidate` | [HTML pages — always fresh] |
| `/sitemap.xml` | `public, max-age=3600` | [Refresh hourly] |
| `/robots.txt` | `public, max-age=86400` | [Refresh daily] |

---

## 5. Email Configuration (If Applicable)

| Property | Value |
|---|---|
| Email provider | [e.g., "Google Workspace / Fastmail / Zoho / Forward-only"] |
| Addresses | [e.g., "hello@, support@, press@"] |
| SPF record | [Configured — see DNS section] |
| DKIM | [Configured — selector: [selector name]] |
| DMARC | [Policy: [quarantine/reject], rua: [reporting email]] |

---

## 6. Third-Party Services

| Service | Purpose | Integration Method | Account Owner |
|---|---|---|---|
| [e.g., Google Analytics] | Analytics | [Script tag / GTM] | [Owner] |
| [e.g., Formspree] | Form backend | [Form action URL] | [Owner] |
| [e.g., Mailchimp] | Newsletter | [Embedded form / API] | [Owner] |
| [e.g., Calendly] | Meeting scheduling | [Embed / link] | [Owner] |

---

## 7. Security

| Measure | Implementation |
|---|---|
| HTTPS | [Enforced — see SSL section] |
| Security headers | [Content-Security-Policy, X-Frame-Options, X-Content-Type-Options] |
| Rate limiting | [e.g., "Cloudflare rate limiting on form endpoints"] |
| DDoS protection | [e.g., "Cloudflare — included in plan"] |
| Dependency scanning | [e.g., "Dependabot / Renovate for dependency updates"] |
| Secrets | [No secrets in repo — env vars in hosting platform] |

### 7.1 Security Headers

```
Content-Security-Policy: default-src 'self'; script-src 'self' [analytics domains]; style-src 'self' 'unsafe-inline'; img-src 'self' data: [image CDN domains]; font-src 'self' [font domains]
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

---

## 8. Backup & Recovery

| Item | Strategy | Frequency | Location |
|---|---|---|---|
| Source code | [Git — hosted on GitHub/GitLab] | [Every commit] | [Repository URL] |
| CMS content | [e.g., "CMS export / database backup"] | [Daily / weekly] | [Backup location] |
| DNS config | [Exported zone file] | [After changes] | [Backup location] |
| Form submissions | [e.g., "Forwarded to email + stored in Formspree"] | [Per submission] | [Email archive] |

---

## 9. Deployment Checklist

- [ ] Domain registered and DNS configured
- [ ] SSL certificate provisioned and HTTPS enforced
- [ ] Hosting platform account created and configured
- [ ] Build command and output directory verified
- [ ] Environment variables set in hosting platform
- [ ] CDN caching rules configured
- [ ] Security headers configured
- [ ] Email routing configured (SPF, DKIM, DMARC)
- [ ] Preview/staging environment working
- [ ] Auto-deploy from main branch enabled
- [ ] Backup strategy documented and tested

---

## 10. Related Documents

- [Performance Budget](PERFORMANCE_BUDGET.md)
- [CMS Spec](CMS_SPEC.md)
- [Integrations](INTEGRATIONS.md)
- [Pre-Launch Checklist](../launch/PRELAUNCH_CHECKLIST.md)
- [Maintenance Plan](../operations/MAINTENANCE_PLAN.md)
