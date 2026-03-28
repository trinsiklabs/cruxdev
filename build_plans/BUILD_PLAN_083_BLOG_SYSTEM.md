# BUILD_PLAN_083: Full Blog System — Apply Blog Patterns to cruxdev.dev

**Status:** NOT STARTED
**Priority:** High
**Depends on:** Blog patterns docs (BLOG_PATTERNS.md, BLOG_PAGINATION_PATTERNS.md, BLOG_POST_PATTERNS.md, BLOG_TAGGING_PATTERNS.md)

## Context

The blog at cruxdev.dev/blog/ is a bare list with no pagination, no tags, no cards, no search, no RSS. The BIP pipeline generates posts automatically but the presentation doesn't match any modern blog standard. Four research-converged patterns docs (4,240 lines) define the target.

## Phase 1: Blog Index — Card Grid with Pagination

Per BLOG_PAGINATION_PATTERNS.md:
- [ ] 1.1 Replace plain list with responsive card grid (1 col mobile → 2 col tablet → 3 col desktop)
- [ ] 1.2 Standard card design: title, date, summary, tags, estimated read time
- [ ] 1.3 Cards link to post page
- [ ] 1.4 Newest first sort (already done)
- [ ] 1.5 Pagination: numbered pages at /blog/page/2/, /blog/page/3/ (when > 9 posts per page)
- [ ] 1.6 Empty state for no posts (already done, keep)
- [ ] 1.7 Skeleton loading state CSS (for future JS-enhanced loading)

## Phase 2: Blog Post Page — Full Layout

Per BLOG_POST_PATTERNS.md:
- [ ] 2.1 Title, date, tags, estimated read time in header
- [ ] 2.2 Reading progress bar (CSS-only or lightweight JS)
- [ ] 2.3 Table of contents auto-generated from headings (sidebar on desktop, collapsible on mobile)
- [ ] 2.4 Proper typography: prose styling for h2, h3, p, ul, ol, code, pre, blockquote, hr, table
- [ ] 2.5 Social sharing buttons (X, LinkedIn, copy link)
- [ ] 2.6 Previous/next post navigation at bottom
- [ ] 2.7 Related posts section (same tags → related)
- [ ] 2.8 Author info (Trinsik Labs, not personal name)
- [ ] 2.9 "Report improvement" link (GitHub issue with patterns:blog label)

## Phase 3: Tagging System

Per BLOG_TAGGING_PATTERNS.md:
- [ ] 3.1 Define tag taxonomy in frontmatter: `tags: ["convergence", "competitive-analysis", "rust"]`
- [ ] 3.2 Tag pages: /blog/tag/convergence/ lists all posts with that tag
- [ ] 3.3 Tags index page: /blog/tags/ shows all tags with post counts
- [ ] 3.4 Tag pill badges on cards and post pages (clickable → tag page)
- [ ] 3.5 Noindex tag pages with < 3 posts (SEO)
- [ ] 3.6 Update existing blog post with tags

## Phase 4: RSS Feed

Per BLOG_PATTERNS.md:
- [ ] 4.1 RSS 2.0 feed at /blog/rss.xml
- [ ] 4.2 Atom feed at /blog/atom.xml (optional, RSS sufficient)
- [ ] 4.3 Autodiscovery: `<link rel="alternate" type="application/rss+xml">` in `<head>`
- [ ] 4.4 Feed includes: title, description, pubDate, link, content (full or excerpt)
- [ ] 4.5 Validate with W3C Feed Validator

## Phase 5: Structured Data + SEO

Per BLOG_PATTERNS.md:
- [ ] 5.1 JSON-LD BlogPosting schema on every post page
- [ ] 5.2 JSON-LD Blog schema on index page
- [ ] 5.3 Open Graph tags (og:title, og:description, og:type=article, og:published_time)
- [ ] 5.4 Twitter Card tags (twitter:card=summary_large_image)
- [ ] 5.5 Canonical URLs on all blog pages
- [ ] 5.6 Sitemap includes all blog pages (already done via @astrojs/sitemap)

## Phase 6: BIP Pipeline Integration

- [ ] 6.1 Auto-generated posts include tags based on event type (feature_shipped → "feature", gap_closed → "competitive-analysis")
- [ ] 6.2 Auto-generated posts include proper frontmatter (layout, title, date, slug, summary, tags)
- [ ] 6.3 Update publish_blog_post() in server.rs to include tags
- [ ] 6.4 After blog post written → trigger deploy.sh automatically (already wired)
- [ ] 6.5 Verify end-to-end: convergence → blog post → deploy → live on site → linked from index

## Phase 7: Link Validation

- [ ] 7.1 Add internal link checker to website convergence phase
- [ ] 7.2 Crawl all pages from sitemap, check every `<a href>` returns 200
- [ ] 7.3 Check blog index links match actual built paths
- [ ] 7.4 Add to check_seo_health: broken internal link detection
- [ ] 7.5 Add to deploy.sh: post-deploy link check (hit every page in sitemap)

## Phase 8: Engine Integration

- [ ] 8.1 Add BLOG_DIMENSIONS to router.rs: ["content_quality", "layout", "pagination", "tagging", "rss", "structured_data", "accessibility", "seo", "link_integrity"]
- [ ] 8.2 Detect blog presence in projects (blog/ dir, /blog route)
- [ ] 8.3 Wire BLOG_DIMENSIONS into WebsiteConvergence phase
- [ ] 8.4 Architecture test: BLOG_DIMENSIONS referenced in routing

## Phase 9: Content Generation

- [ ] 9.1 Blog post: "How CruxDev's Blog Builds Itself"
- [ ] 9.2 X post announcing the blog system
- [ ] 9.3 Publish via BIP pipeline

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
# Visual: check /blog/, /blog/tag/*, /blog/rss.xml, individual post pages
# Link check: all internal links return 200
```
