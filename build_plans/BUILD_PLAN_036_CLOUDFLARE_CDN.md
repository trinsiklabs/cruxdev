# BUILD_PLAN_036: Cloudflare CDN Setup for cruxdev.dev

**Status:** NOT STARTED
**Priority:** High (blocks search engine registration per §11.0)

## Context

cruxdev.dev is served directly from vh1.trinsik.io with no CDN. PageSpeed is 97 but server response time and caching are suboptimal. Cloudflare's free tier provides CDN, SSL management, HTTP/2-3, DDoS protection, caching, and security headers — all prerequisites for search engine registration per WEBSITE_PLANNING.md §11.0.

## Phase 1: DNS Migration to Cloudflare

- [ ] 1.1 Create Cloudflare account (if not existing)
- [ ] 1.2 Add cruxdev.dev site to Cloudflare
- [ ] 1.3 Cloudflare scans existing DNS records
- [ ] 1.4 Update domain registrar nameservers to Cloudflare's
- [ ] 1.5 Verify DNS propagation (may take up to 48h)
- [ ] 1.6 Ensure A record points to vh1.trinsik.io with proxy enabled (orange cloud)

## Phase 2: SSL/TLS Configuration

- [ ] 2.1 SSL mode: Full (Strict) — requires valid cert on origin
- [ ] 2.2 Enable "Always Use HTTPS"
- [ ] 2.3 Enable HSTS (min 6 months, include subdomains)
- [ ] 2.4 Enable "Automatic HTTPS Rewrites"
- [ ] 2.5 TLS minimum version: 1.2

## Phase 3: Performance

- [ ] 3.1 Enable HTTP/2 and HTTP/3 (QUIC)
- [ ] 3.2 Enable Brotli compression
- [ ] 3.3 Caching rules:
  - Static assets (CSS, JS, images, fonts): Browser TTL 1 year, Edge TTL 1 month
  - HTML pages: Browser TTL 1 hour, Edge TTL 4 hours
  - API/dynamic: bypass cache
- [ ] 3.4 Enable "Auto Minify" for HTML, CSS, JS
- [ ] 3.5 Enable Early Hints (103)

## Phase 4: Security Headers

- [ ] 4.1 Add via Cloudflare Transform Rules or Workers:
  - `Strict-Transport-Security: max-age=31536000; includeSubDomains`
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY`
  - `Referrer-Policy: strict-origin-when-cross-origin`
  - `Permissions-Policy: camera=(), microphone=(), geolocation=()`
- [ ] 4.2 Enable Cloudflare WAF (free tier rules)

## Phase 5: Page Rules / Cache Rules

- [ ] 5.1 `cruxdev.dev/install.sh` — bypass cache (always serve latest)
- [ ] 5.2 `cruxdev.dev/llms.txt` — short cache (1 hour) so AI crawlers get fresh data
- [ ] 5.3 `cruxdev.dev/sitemap*` — short cache (1 hour)
- [ ] 5.4 Everything else — standard cache

## Phase 6: Update Deploy Script

- [ ] 6.1 After rsync, purge Cloudflare cache for changed URLs:
  ```bash
  curl -X POST "https://api.cloudflare.com/client/v4/zones/{zone_id}/purge_cache" \
    -H "Authorization: Bearer {api_token}" \
    -d '{"purge_everything": true}'
  ```
- [ ] 6.2 Store Cloudflare zone ID and API token in env vars (not in repo)

## Phase 7: Verification

- [ ] 7.1 `curl -I https://cruxdev.dev` shows Cloudflare headers (`cf-ray`, `server: cloudflare`)
- [ ] 7.2 HTTP/2 or HTTP/3 confirmed
- [ ] 7.3 HSTS header present
- [ ] 7.4 Security headers present
- [ ] 7.5 Re-run PageSpeed Insights — target Performance 99-100 with CDN
- [ ] 7.6 SSL Labs test: A+ rating
