# Search Engine & AI Registration Guide

**Purpose:** Step-by-step instructions for registering a project website with search engines and AI discovery systems.
**Last verified:** 2026-03-27
**Review schedule:** Monthly (first Monday)

---

## 1. Google Search Console

**URL:** https://search.google.com/search-console

### Registration Steps
1. Go to https://search.google.com/search-console
2. Click "Add property"
3. Choose "URL prefix" and enter your full URL (e.g., `https://cruxdev.dev`)
4. Verify ownership — recommended method: **DNS TXT record**
   - Add a TXT record to your domain DNS: `google-site-verification=XXXXX`
   - Alternative: Upload an HTML file to your site root
   - Alternative: Add a `<meta>` tag to your homepage `<head>`
5. After verification, go to "Sitemaps" in the left nav
6. Submit your sitemap URL: `https://yoursite.com/sitemap-index.xml`
7. Go to "URL Inspection" and submit your priority pages one at a time

### Priority Pages to Submit
- Homepage (`/`)
- Documentation index (`/docs`)
- Quick start (`/docs/quickstart`)
- Install (`/docs/install`)
- Methodology (`/methodology`)
- Engine (`/engine`)
- Integrations (`/integrations`)
- Top comparison page (`/vs/superpowers`)

### What It Does
- Tells Google your site exists (speeds up initial crawl from weeks to days)
- Provides diagnostic tools (crawl errors, indexing status, mobile usability)
- Lets you submit individual URLs for priority indexing
- Shows search performance data (clicks, impressions, position)

### Timeline
- Verification: immediate (DNS propagation may take up to 48h)
- First crawl: 1-3 days after verification
- Full indexing: 4-14 days for a new site

---

## 2. Bing Webmaster Tools

**URL:** https://www.bing.com/webmasters

### Registration Steps
1. Go to https://www.bing.com/webmasters
2. Sign in with a Microsoft account
3. Option A: "Import from Google Search Console" (easiest — auto-imports everything)
4. Option B: Add site manually, verify via DNS CNAME record
5. Submit sitemap: `https://yoursite.com/sitemap-index.xml`
6. Enable IndexNow (Settings → IndexNow)

### What It Does
- Indexes your site in Bing's search engine
- **Also powers DuckDuckGo and Yahoo** — one registration covers three search engines
- **ChatGPT search uses Bing's index** — being in Bing is critical for AI visibility
- IndexNow support for near-instant notification on new/changed pages

### Timeline
- Import from GSC: immediate
- First crawl: 24-48 hours
- IndexNow pages: minutes to hours

---

## 3. IndexNow (Automated)

**Protocol:** https://www.indexnow.org

### What It Is
IndexNow instantly notifies participating search engines (Bing, Yandex, Naver, Seznam) when pages change. Google does NOT support IndexNow.

### Setup
1. Generate a key: any string of 8+ hex characters (e.g., `a1b2c3d4e5f6`)
2. Create a key file at your site root: `https://yoursite.com/{key}.txt` containing just the key
3. On every deploy, POST changed URLs:

```bash
curl -X POST "https://api.indexnow.org/IndexNow" \
  -H "Content-Type: application/json" \
  -d '{
    "host": "cruxdev.dev",
    "key": "YOUR_KEY",
    "urlList": [
      "https://cruxdev.dev/",
      "https://cruxdev.dev/docs/",
      "https://cruxdev.dev/integrations/"
    ]
  }'
```

### Automation
Add to deploy script after rsync:
```bash
# Ping IndexNow
curl -s -X POST "https://api.indexnow.org/IndexNow" \
  -H "Content-Type: application/json" \
  -d "{\"host\":\"$DOMAIN\",\"key\":\"$INDEXNOW_KEY\",\"urlList\":[\"https://$DOMAIN/\"]}"

# Ping Google sitemap
curl -s "https://www.google.com/ping?sitemap=https://$DOMAIN/sitemap-index.xml"
```

---

## 4. AI Discovery

### What Works
- **llms.txt** at site root — AI-readable project description. Already implemented.
- **Schema.org JSON-LD** — structured data on every page. Already implemented.
- **robots.txt** — allow AI search bots. Already implemented.

### AI Bot Policy (robots.txt)
```
# Allow all search bots (including AI search)
User-agent: *
Allow: /

# Optional: block AI training bots (keep search access)
# User-agent: GPTBot
# Disallow: /
# User-agent: Google-Extended
# Disallow: /

Sitemap: https://cruxdev.dev/sitemap-index.xml
```

### What Doesn't Work
- No official submission portal exists for any AI system
- AI citation takes 2-3 months to begin, 6 months for significant visibility
- 60% of AI-cited sources are NOT in Google's top 10 — original data and structured content matter more than traditional SEO rank

### Key Insight
ChatGPT search relies on Bing's index. Being registered in Bing Webmaster Tools is the single most important step for AI discovery.

---

## 5. Monthly Review Checklist

- [ ] Verify Google Search Console shows no crawl errors
- [ ] Check Bing Webmaster Tools for indexing issues
- [ ] Verify sitemap is current and accessible
- [ ] Check llms.txt is current with latest capabilities
- [ ] Review robots.txt AI bot policy
- [ ] Check IndexNow key file is accessible
- [ ] Review search performance data (GSC) — any pages dropping?
