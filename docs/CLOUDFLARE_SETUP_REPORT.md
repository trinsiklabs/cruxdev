# Cloudflare Setup Report — cruxdev.dev

**Date:** 2026-03-27
**Domain:** cruxdev.dev
**Plan:** Free
**Account:** Bryan@trinsiklabs.com

## Completed Steps

### Step 4: Verify DNS Records
- Site is active and proxied through Cloudflare
- DNS Setup confirmed: Full
- A record pointing to `172.105.158.126` (vh1.trinsik.io) with proxy enabled

### Step 5: SSL/TLS Configuration
- Encryption mode: **Full (strict)**
- Always Use HTTPS: **Enabled**
- Automatic HTTPS Rewrites: **Enabled**
- Minimum TLS Version: **TLS 1.2**
- TLS 1.3: **Enabled**

### Step 6: HSTS (HTTP Strict Transport Security)
- Enabled with:
  - Max-Age: **6 months** (15552000 seconds)
  - Include subdomains: **Yes**
  - Preload: **Yes**
  - No-Sniff: **Yes**
- Verified in response headers: `strict-transport-security: max-age=15552000; includeSubDomains; preload`

### Step 7: Speed / Performance
- Early Hints: **Enabled**
- HTTP/2: **Enabled** (default)
- HTTP/3 (QUIC): **Enabled**
- Verified via `alt-svc: h3=":443"; ma=86400` header

### Step 8: Caching & Cache Rules
Three cache rules deployed (all Active):

| # | Name | Match | Action |
|---|------|-------|--------|
| 1 | Bypass install.sh | URI Path equals `/install.sh` | Bypass cache |
| 2 | Short cache llms.txt | URI Path equals `/llms.txt` | Eligible for cache, Edge TTL 1 hour |
| 3 | Long cache Astro assets | URI Path starts with `/_astro/` | Eligible for cache, Edge TTL 30 days, Browser TTL 1 year |

### Step 9: Security Headers (Transform Rule)
One Response Header Transform Rule deployed (Active), applied to **all incoming requests**:

| Header | Value |
|--------|-------|
| X-Content-Type-Options | `nosniff` |
| X-Frame-Options | `DENY` |
| Referrer-Policy | `strict-origin-when-cross-origin` |
| Permissions-Policy | `camera=(), microphone=(), geolocation=()` |

### Step 10: API Credentials
- **Zone ID:** `6c75370165ba58ecc958519bee95fc37`
- **Account ID:** `7dd7080e1819f80753e7a393cd0dda52`
- **API Token created:** `cruxdev.dev Cache Purge`
  - Permission: Zone → Cache Purge → Purge
  - Scope: Specific zone → cruxdev.dev
  - Token value: `cfut_9b1JuNuy7yjUWIM3LupqsXzR2rc2c59uOLX9Mw7r03588807`

### Step 12: Verification (Partial)
Verified via browser fetch that all headers are present:

```
server: cloudflare
cf-ray: 9e3292027aa6f494-ATL
strict-transport-security: max-age=15552000; includeSubDomains; preload
x-content-type-options: nosniff
x-frame-options: DENY
referrer-policy: strict-origin-when-cross-origin
permissions-policy: camera=(), microphone=(), geolocation=()
alt-svc: h3=":443"; ma=86400
cf-cache-status: DYNAMIC
```

## Remaining Steps (Manual)

### Store API credentials in shell environment
```bash
echo '' >> ~/.zshenv
echo '# Cloudflare (cruxdev.dev)' >> ~/.zshenv
echo 'export CLOUDFLARE_ZONE_ID="6c75370165ba58ecc958519bee95fc37"' >> ~/.zshenv
echo 'export CLOUDFLARE_API_TOKEN="cfut_9b1JuNuy7yjUWIM3LupqsXzR2rc2c59uOLX9Mw7r03588807"' >> ~/.zshenv
source ~/.zshenv
```

### Step 11: Update deploy script with cache purge
Add to `deploy.sh` after successful deploy:
```bash
# Purge Cloudflare cache
echo "Purging Cloudflare cache..."
curl -s -X POST "https://api.cloudflare.com/client/v4/zones/${CLOUDFLARE_ZONE_ID}/purge_cache" \
  -H "Authorization: Bearer ${CLOUDFLARE_API_TOKEN}" \
  -H "Content-Type: application/json" \
  --data '{"purge_everything":true}' | jq .
```

### Step 12: Full verification from local machine
```bash
# Check Cloudflare is proxying
curl -sI https://cruxdev.dev | grep -i "cf-ray\|server:"

# Check HTTPS redirect
curl -sI http://cruxdev.dev | head -3

# Check security headers
curl -sI https://cruxdev.dev | grep -i "x-content-type\|x-frame\|referrer-policy\|permissions-policy\|strict-transport"

# Test API token
curl "https://api.cloudflare.com/client/v4/user/tokens/verify" \
  -H "Authorization: Bearer ${CLOUDFLARE_API_TOKEN}"
```

### Step 13: Search Engine Registration
- [ ] Google Search Console — verify ownership at https://search.google.com/search-console
- [ ] Bing Webmaster Tools — submit at https://www.bing.com/webmasters
