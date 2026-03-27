# BUILD_PLAN_033: Site Registration & Indexing + SEO Pattern Updates

**Status:** CONVERGED
**Priority:** High
**Depends on:** BUILD_PLAN_024 (SEO patterns)

## Context

cruxdev.dev is live but not registered with any search engine or AI discovery system. Without registration, indexing depends on inbound links (slow — weeks to months). With registration, Google indexes in days, Bing in hours (via IndexNow).

## Phase 1: Manual One-Time Registrations

- [ ] 1.1 **Google Search Console** — verify site ownership (DNS TXT record or HTML file), submit sitemap.xml
- [ ] 1.2 **Bing Webmaster Tools** — register (can import from GSC), submit sitemap, enable IndexNow
- [ ] 1.3 **Google Search Console URL Inspection** — submit priority pages: /, /docs, /engine, /methodology, /vs/superpowers, /integrations, /docs/openclaw

## Phase 2: Automated Indexing on Deploy

- [ ] 2.1 **IndexNow** integration in deploy script — ping Bing/Yandex/Naver on every deploy with changed URLs
  - POST to https://api.indexnow.org/IndexNow with key + URL list
  - Key file served at cruxdev.dev/{key}.txt
  - Add to deploy.sh: after rsync, submit changed URLs
- [ ] 2.2 **Google Indexing API** — for priority pages (limited to 200 URLs/day)
- [ ] 2.3 **Sitemap ping** — `curl "https://www.google.com/ping?sitemap=https://cruxdev.dev/sitemap-index.xml"` on deploy

## Phase 3: AI Discovery Optimization

- [ ] 3.1 **llms.txt** — already exists, verify it's current (done in BP032)
- [ ] 3.2 **robots.txt** — allow AI search bots, consider policy on training bots:
  - Allow: OAI-SearchBot, PerplexityBot, ClaudeBot-Search, GoogleOther
  - Decision needed: allow or block training bots (GPTBot, CCBot, ClaudeBot, Google-Extended)
- [ ] 3.3 **Schema.org** — already has SoftwareApplication, verify FAQPage on vs/ pages
- [ ] 3.4 **AGENTS.md** — for the GitHub repo (code discovery by AI agents)

## Phase 4: Update SEO Patterns in WEBSITE_PLANNING.md

- [ ] 4.1 Add "Site Registration Checklist" section to Phase 5 (SEO & AI Visibility):
  - Google Search Console registration
  - Bing Webmaster Tools registration
  - IndexNow key generation and deployment
  - Sitemap submission
  - Priority page submission
  - robots.txt AI bot policy
  - llms.txt maintenance
  - Timeline expectations (Google: 4-14 days, Bing: 24-48h with IndexNow)
- [ ] 4.2 Add "Post-Deploy Indexing" to Phase 11 (Launch):
  - IndexNow ping on every deploy
  - Sitemap ping to Google
  - Verify new pages indexed within 1 week
- [ ] 4.3 Add automation guidance:
  - What the growth engine can do: IndexNow ping, sitemap ping, llms.txt update
  - What requires manual setup: GSC/Bing registration (one-time)

## Phase 5: Immediate Actions for cruxdev.dev

- [ ] 5.1 Register cruxdev.dev in Google Search Console
- [ ] 5.2 Register cruxdev.dev in Bing Webmaster Tools
- [ ] 5.3 Generate IndexNow key, add to deploy script
- [ ] 5.4 Submit sitemap to both
- [ ] 5.5 Submit priority URLs via GSC URL Inspection

## Key Findings from Research

- ChatGPT search uses Bing's index — being in Bing is critical for AI visibility
- 60% of AI-cited sources are NOT in Google's top 10 — traditional SEO rank isn't the main driver
- IndexNow gives near-instant Bing notification (Google still doesn't support it)
- No AI system has an official submission portal — discovery is through crawling
- AI citation typically takes 2-3 months to begin
- Google explicitly does not support llms.txt (but it doesn't hurt)

## Verification

```bash
# Verify sitemap accessible
curl -s -o /dev/null -w "%{http_code}" https://cruxdev.dev/sitemap-index.xml
# Verify llms.txt accessible
curl -s -o /dev/null -w "%{http_code}" https://cruxdev.dev/llms.txt
# Verify robots.txt accessible
curl -s https://cruxdev.dev/robots.txt
```
