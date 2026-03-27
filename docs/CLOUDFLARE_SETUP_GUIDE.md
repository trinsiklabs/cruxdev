# Cloudflare Setup Guide for cruxdev.dev

**Last verified:** 2026-03-27
**Audience:** Human or AI agent (Claude Code cowork session) — every step specifies exact location and action.

---

## Prerequisites

- Domain: `cruxdev.dev`
- Current hosting: vh1.trinsik.io (get the IP with `dig vh1.trinsik.io +short`)
- Domain registrar access (wherever cruxdev.dev DNS is managed)
- A browser session (Cloudflare dashboard is web-only — no CLI for initial setup)

---

## Step 1: Create Cloudflare Account

**Where:** Browser → https://dash.cloudflare.com/sign-up
**Action:** Create account with email + password. If account exists, log in.

---

## Step 2: Add Site

**Where:** Cloudflare dashboard → top nav → "Add a site" button (or https://dash.cloudflare.com/?to=/:account/add-site)
**Actions:**
1. Enter `cruxdev.dev` in the domain field
2. Click "Continue"
3. Select **Free** plan → click "Continue"
4. Cloudflare auto-scans DNS records. Review the list — you should see at least one A record pointing to vh1.trinsik.io's IP
5. Click "Continue"

---

## Step 3: Update Nameservers

**Where:** Cloudflare shows you two nameservers (e.g., `aria.ns.cloudflare.com` and `bob.ns.cloudflare.com`). Write these down.

**Then go to your domain registrar** (the company where cruxdev.dev is registered — could be Namecheap, Cloudflare Registrar, GoDaddy, Porkbun, etc.):
1. Log in to your registrar
2. Find the DNS/Nameserver settings for cruxdev.dev
3. Replace ALL existing nameservers with Cloudflare's two
4. Save

**Back in Cloudflare dashboard:**
1. Click "Done, check nameservers"
2. Cloudflare will show "Pending" until propagation completes
3. Propagation takes 15 min to 48 hours (usually < 1 hour)
4. You can click "Re-check now" periodically
5. Status changes to **"Active"** when done — do NOT proceed until Active

---

## Step 4: Verify DNS Records

**Where:** Cloudflare dashboard → left sidebar → **DNS** → **Records**

**Check:**
- There should be an **A record** for `cruxdev.dev` pointing to vh1.trinsik.io's IP address
- The **Proxy status** column should show an orange cloud icon (Proxied = ON)
- If the cloud is gray (DNS only), click the record → Edit → toggle Proxy to ON → Save

**Add if missing:**
- Type: **CNAME**, Name: **www**, Target: **cruxdev.dev**, Proxy: ON

---

## Step 5: SSL/TLS Configuration

**Where:** Cloudflare dashboard → left sidebar → **SSL/TLS**

### 5a: SSL Mode
**Location:** SSL/TLS → **Overview** (the default page when you click SSL/TLS)
**Action:** Click the radio button for **Full (strict)**

### 5b: Edge Certificates
**Location:** SSL/TLS → **Edge Certificates** (sub-menu item)
**Actions (scroll down the page for each):**
1. **Always Use HTTPS** → find the toggle → switch to **ON**
2. **Automatic HTTPS Rewrites** → find the toggle → switch to **ON**
3. **Minimum TLS Version** → find the dropdown → select **TLS 1.2**
4. **TLS 1.3** → find the toggle → switch to **ON**

---

## Step 6: HSTS

**Where:** Cloudflare dashboard → left sidebar → **SSL/TLS** → **Edge Certificates** → scroll down to **HTTP Strict Transport Security (HSTS)** section

**Actions:**
1. Click **"Enable HSTS"** button
2. Read the acknowledgment → click "I understand"
3. Set these values:
   - **Max-Age:** 6 months (the slider or dropdown — choose 6 months / 15768000 seconds)
   - **Include subdomains:** toggle ON
   - **Preload:** toggle ON
   - **No-Sniff:** toggle ON
4. Click **Save**

---

## Step 7: Speed / Performance

### 7a: Auto Minify
**Where:** Cloudflare dashboard → left sidebar → **Speed** → **Optimization** → **Content Optimization**
**Actions:** Toggle ON for all three: **JavaScript**, **CSS**, **HTML**

### 7b: Brotli
**Where:** Same page (Speed → Optimization → Content Optimization), scroll down
**Action:** **Brotli** → toggle ON

### 7c: Protocol Optimization
**Where:** Cloudflare dashboard → left sidebar → **Speed** → **Optimization** → **Protocol Optimization**
**Actions:**
1. **HTTP/2** → toggle ON (may be on by default)
2. **HTTP/3 (with QUIC)** → toggle ON
3. **Early Hints (103)** → toggle ON

---

## Step 8: Caching

### 8a: Browser Cache TTL
**Where:** Cloudflare dashboard → left sidebar → **Caching** → **Configuration**
**Action:** Set **Browser Cache TTL** → **Respect Existing Headers**

### 8b: Cache Rules
**Where:** Cloudflare dashboard → left sidebar → **Rules** → **Cache Rules**
**Action:** Create rules for specific URL patterns:

**Rule 1: Bypass cache for install script**
1. Click "Create rule"
2. Rule name: "Bypass install.sh"
3. When: Custom filter expression → URI Path equals `/install.sh`
4. Then: Bypass cache
5. Save

**Rule 2: Short cache for llms.txt**
1. Create rule → name: "Short cache llms.txt"
2. When: URI Path equals `/llms.txt`
3. Then: Eligible for cache, Edge TTL: 1 hour
4. Save

**Rule 3: Long cache for static assets**
1. Create rule → name: "Long cache Astro assets"
2. When: URI Path starts with `/_astro/`
3. Then: Eligible for cache, Edge TTL: 1 month, Browser TTL: 1 year
4. Save

---

## Step 9: Security Headers

**Where:** Cloudflare dashboard → left sidebar → **Rules** → **Transform Rules** → **Modify Response Header** tab

**Action:** Click "Create rule"
1. Rule name: "Security Headers"
2. When: All incoming requests (select "All incoming requests" from the dropdown)
3. Then → **Set static** for each header:

| Operation | Header name | Value |
|-----------|------------|-------|
| Set static | `X-Content-Type-Options` | `nosniff` |
| Set static | `X-Frame-Options` | `DENY` |
| Set static | `Referrer-Policy` | `strict-origin-when-cross-origin` |
| Set static | `Permissions-Policy` | `camera=(), microphone=(), geolocation=()` |

4. Click "Deploy" to save and activate

(HSTS header is already handled by Step 6 — don't add it here)

---

## Step 10: Get API Credentials (for deploy script)

### 10a: Zone ID
**Where:** Cloudflare dashboard → left sidebar → **Overview** (the main page for cruxdev.dev)
**Action:** Look at the **right sidebar** → scroll down to **API** section → copy the **Zone ID**

### 10b: API Token
**Where:** https://dash.cloudflare.com/profile/api-tokens
**Actions:**
1. Click "Create Token"
2. Use the **"Edit zone DNS"** template (or create custom)
3. For custom: Permissions → Zone → Cache Purge → Purge. Zone Resources → Include → Specific zone → cruxdev.dev
4. Click "Continue to summary" → "Create Token"
5. **Copy the token immediately** — it's shown only once

### 10c: Store credentials
**Where:** Terminal on your machine (NOT in any repo file)
```bash
echo '' >> ~/.zshenv
echo '# Cloudflare (cruxdev.dev)' >> ~/.zshenv
echo 'export CLOUDFLARE_ZONE_ID="paste-zone-id-here"' >> ~/.zshenv
echo 'export CLOUDFLARE_API_TOKEN="paste-token-here"' >> ~/.zshenv
source ~/.zshenv
```

---

## Step 11: Update Deploy Script

**Where:** Terminal → edit `/Users/user/personal/cruxdev-dev/deploy.sh`

Add after the rsync command:
```bash
# Purge Cloudflare cache
if [ -n "$CLOUDFLARE_ZONE_ID" ] && [ -n "$CLOUDFLARE_API_TOKEN" ]; then
  curl -s -X POST "https://api.cloudflare.com/client/v4/zones/$CLOUDFLARE_ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{"purge_everything": true}'
  echo "Cloudflare cache purged."
fi
```

---

## Step 12: Verify Everything Works

**Where:** Terminal

```bash
# 1. Check Cloudflare is serving (look for cf-ray and server: cloudflare)
curl -I https://cruxdev.dev 2>&1 | grep -i "cf-ray\|server:\|strict-transport"
# Expected: server: cloudflare, cf-ray: [hash], strict-transport-security: ...

# 2. Check HTTPS redirect
curl -I http://cruxdev.dev 2>&1 | head -3
# Expected: 301 redirect to https://

# 3. Check security headers
curl -I https://cruxdev.dev 2>&1 | grep -i "x-content-type\|x-frame\|referrer-policy\|permissions-policy"
# Expected: all 4 headers present

# 4. SSL Labs test (browser)
# Visit: https://www.ssllabs.com/ssltest/analyze.html?d=cruxdev.dev
# Target: A+ rating

# 5. Re-run PageSpeed
# Visit: https://pagespeed.web.dev/analysis/https-cruxdev-dev/
# Target: Performance 99-100 (CDN should improve server response time)
```

---

## Step 13: Register with Search Engines

**Only after Cloudflare is verified active:**

### Google Search Console
**Where:** Browser → https://search.google.com/search-console
1. Click "Add property" → URL prefix → enter `https://cruxdev.dev`
2. Verify: choose **DNS** method → add TXT record in **Cloudflare DNS** (dashboard → DNS → Records → Add record → Type: TXT, Name: @, Content: the verification string Google gives you)
3. Click "Verify" in Google Search Console
4. Go to "Sitemaps" (left sidebar) → enter `sitemap-index.xml` → Submit
5. Go to "URL Inspection" → enter `/` → "Request Indexing"
6. Repeat URL Inspection for: `/docs`, `/engine`, `/methodology`, `/integrations`, `/vs/superpowers`, `/docs/openclaw`

### Bing Webmaster Tools
**Where:** Browser → https://www.bing.com/webmasters
1. Sign in with Microsoft account
2. Click "Import from GSC" (easiest — pulls everything from Google Search Console)
3. Authorize the connection
4. Done — Bing, DuckDuckGo, Yahoo, and ChatGPT search now know about the site

---

## Troubleshooting

**"Pending Nameserver Update" for more than 24 hours:**
- Go back to your registrar and verify the nameservers were saved correctly
- Some registrars have a separate "DNS management" vs "nameserver" setting — make sure you changed the nameservers, not just DNS records

**SSL errors after switching to Cloudflare:**
- Make sure SSL mode is "Full (strict)" not "Flexible"
- Check that the origin server (vh1.trinsik.io) has a valid SSL certificate

**Site shows Cloudflare error page:**
- Check DNS records — the A record must point to the correct IP
- Check the origin server is running
