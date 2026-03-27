# Cloudflare Setup Guide for cruxdev.dev

**Last verified:** 2026-03-27

## Step 1: Create Account / Login

1. Go to https://dash.cloudflare.com
2. Sign up or log in

## Step 2: Add Site

1. Click "Add a site"
2. Enter `cruxdev.dev`
3. Select the **Free** plan
4. Cloudflare scans your existing DNS records — verify they're correct

## Step 3: Update Nameservers

1. Cloudflare gives you two nameservers (e.g., `aria.ns.cloudflare.com`, `bob.ns.cloudflare.com`)
2. Go to your domain registrar (wherever cruxdev.dev is registered)
3. Replace the current nameservers with Cloudflare's two
4. Wait for propagation (usually 15 min - 48 hours, typically < 1 hour)
5. Cloudflare dashboard will show "Active" when propagation completes

## Step 4: DNS Records

Verify these records exist (Cloudflare should have imported them):

| Type | Name | Content | Proxy |
|------|------|---------|-------|
| A | cruxdev.dev | [vh1.trinsik.io IP] | Proxied (orange cloud ON) |
| CNAME | www | cruxdev.dev | Proxied |

**Important:** The orange cloud (Proxied) must be ON — this routes traffic through Cloudflare's CDN.

## Step 5: SSL/TLS

1. Go to **SSL/TLS** → **Overview**
2. Set mode to **Full (Strict)**
3. Go to **SSL/TLS** → **Edge Certificates**
4. Enable **Always Use HTTPS** → ON
5. Enable **Automatic HTTPS Rewrites** → ON
6. Set **Minimum TLS Version** → TLS 1.2
7. Enable **TLS 1.3** → ON

## Step 6: HSTS

1. Go to **SSL/TLS** → **Edge Certificates** → scroll to **HTTP Strict Transport Security (HSTS)**
2. Click **Enable HSTS**
3. Settings:
   - Max-Age: 6 months (15768000)
   - Include subdomains: Yes
   - Preload: Yes
   - No-Sniff: Yes

## Step 7: Speed / Performance

1. Go to **Speed** → **Optimization**
2. Enable **Auto Minify**: HTML ✓, CSS ✓, JavaScript ✓
3. Enable **Brotli** compression → ON
4. Go to **Speed** → **Optimization** → **Protocol Optimization**
5. Enable **HTTP/2** → ON
6. Enable **HTTP/3 (with QUIC)** → ON
7. Enable **Early Hints** → ON

## Step 8: Caching

1. Go to **Caching** → **Configuration**
2. Set **Browser Cache TTL** → Respect Existing Headers
3. Go to **Rules** → **Page Rules** (or **Cache Rules**):

| URL Pattern | Setting |
|-------------|---------|
| `cruxdev.dev/install.sh` | Bypass Cache |
| `cruxdev.dev/llms.txt` | Edge TTL: 1 hour |
| `cruxdev.dev/sitemap*` | Edge TTL: 1 hour |
| `cruxdev.dev/_astro/*` | Edge TTL: 1 month, Browser TTL: 1 year |

## Step 9: Security Headers

1. Go to **Rules** → **Transform Rules** → **Modify Response Header**
2. Create rule "Security Headers" matching all traffic:

| Header | Value |
|--------|-------|
| `X-Content-Type-Options` | `nosniff` |
| `X-Frame-Options` | `DENY` |
| `Referrer-Policy` | `strict-origin-when-cross-origin` |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=()` |

(HSTS is already handled by Step 6)

## Step 10: Update Deploy Script

After Cloudflare is active, update `deploy.sh` to purge cache on deploy:

```bash
# Add after rsync:
# Purge Cloudflare cache
if [ -n "$CLOUDFLARE_ZONE_ID" ] && [ -n "$CLOUDFLARE_API_TOKEN" ]; then
  curl -s -X POST "https://api.cloudflare.com/client/v4/zones/$CLOUDFLARE_ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{"purge_everything": true}'
  echo "Cloudflare cache purged."
fi
```

Store in `~/.zshenv`:
```bash
export CLOUDFLARE_ZONE_ID="your-zone-id"       # From Cloudflare dashboard → Overview → right sidebar
export CLOUDFLARE_API_TOKEN="your-api-token"    # Create at dash.cloudflare.com/profile/api-tokens
```

## Step 11: Verify

```bash
# Check Cloudflare is serving
curl -I https://cruxdev.dev 2>&1 | grep -i "cf-ray\|server:\|strict-transport"

# Expected:
# server: cloudflare
# cf-ray: [hash]
# strict-transport-security: max-age=15768000

# SSL Labs test
# Visit: https://www.ssllabs.com/ssltest/analyze.html?d=cruxdev.dev
# Target: A+ rating
```

## Step 12: Re-run PageSpeed

After Cloudflare is active, re-run PageSpeed Insights. CDN should push Performance from 97 to 99-100.
