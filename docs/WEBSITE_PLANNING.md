# Website Planning Methodology

**Version:** 1.0
**Created:** 2026-03-22
**Scope:** Universal — applies to any website regardless of type, industry, or scale.
**Companion:** `SEO_AND_GEO_REFERENCE.md` for detailed search engine and AI optimization techniques.

This document parallels `DEVELOPMENT_PATTERNS.md` (for code) and `E2E_TEST_PATTERNS.md` (for testing). It is the definitive methodology for planning, building, and maintaining websites. Follow it sequentially on new projects. Reference individual sections when iterating on existing sites.

---

## Table of Contents

1. [Phase 1: Discovery & Research](#phase-1-discovery--research)
2. [Phase 2: Strategy & Goals](#phase-2-strategy--goals)
3. [Phase 3: Content Strategy](#phase-3-content-strategy)
4. [Phase 4: Information Architecture](#phase-4-information-architecture)
5. [Phase 5: SEO & AI Visibility Architecture](#phase-5-seo--ai-visibility-architecture)
6. [Phase 6: UX Design & Wireframing](#phase-6-ux-design--wireframing)
7. [Phase 7: Design System & Visual Design](#phase-7-design-system--visual-design)
8. [Phase 8: Technical Architecture & Performance](#phase-8-technical-architecture--performance)
9. [Phase 9: Development & Content Entry](#phase-9-development--content-entry)
10. [Phase 10: Pre-Launch QA](#phase-10-pre-launch-qa)
11. [Phase 11: Launch](#phase-11-launch)
12. [Phase 12: Post-Launch Operations](#phase-12-post-launch-operations)
13. [Appendix A: Legal & Compliance](#appendix-a-legal--compliance)
14. [Appendix B: Internationalization](#appendix-b-internationalization)
15. [Appendix C: Universal Principles](#appendix-c-universal-principles)
16. [Appendix D: Common Mistakes](#appendix-d-common-mistakes)

---

## Phase 0: Pre-Build Setup

**Purpose:** Collect essential project identity and select the website engine BEFORE starting any build work.

### 0.1 Product Identity (Required)

- **Go-to-market name:** The public-facing product name. This may differ from the project folder name. Examples: project folder `cruxdev` → site name "CruxDev", project `trueassess` → site name "TrueAssess.me"
- **Project name:** The folder/repo name (used internally for paths, configs)
- **Tagline:** One-line description for meta tags and hero section
- **Domain:** Target domain if known (can be decided later)

### 0.2 Logo (Optional but Flagged)

- **Logo file:** SVG preferred, PNG acceptable. Provide the path to the logo file.
- The engine will proceed without a logo but will flag it as a gap in GAPS.md.
- If no logo is provided, use text-based branding with the go-to-market name.
- **Favicon:** Auto-generated from logo SVG if provided.

### 0.3 Engine Selection (Required)

**Consult `docs/WEBSITE_ENGINES.md` for the full ecosystem matrix with pros, cons, migration paths, and the decision tree.**

Present the user with engine options based on their project type. Do NOT default to any engine without asking. The user must consciously choose, understanding the trade-offs.

Quick recommendations:
- Content/marketing site → Astro or 11ty
- Blog with memberships → Ghost
- Docs site → Astro (Starlight) or 11ty
- Interactive webapp → SvelteKit, Next.js, or Phoenix LiveView
- Visual editing needed → TinaCMS + Astro, or Ghost
- Elixir project → Phoenix or Still
- Rust project → Zola
- Maximum portability → 11ty

**Deliverables:** Product identity, logo (or gap), engine selection with rationale.

---

## Phase 1: Discovery & Research

**Purpose:** Understand who the site is for, what it must accomplish, and what already exists. No design, no code — only research.

### 1.1 Stakeholder Discovery

- Business goals: What must this site accomplish? (Revenue, signups, awareness, support, education)
- Constraints: Budget, timeline, brand guidelines, technology preferences, regulatory requirements
- Success metrics: How will we know the site is working? Define KPIs before anything else.
- Internal audit: Who owns content? Who approves? What's the governance structure?

### 1.2 User Research

- **Personas**: Who are the primary, secondary, and tertiary audiences? Include: demographics, goals, frustrations, technical proficiency, context of use (mobile? at work? in a rush?)
- **Jobs-to-Be-Done (JTBD)**: What is the user trying to accomplish that leads them here? Personas describe *who*, JTBD describes *why*. Both are needed.
- **User journeys**: Map touchpoints from first awareness through conversion through retention. Identify friction and opportunity at each stage.
- **Existing analytics**: If a site exists, mine it. Top pages, bounce rates, search queries, conversion paths, device breakdown, exit pages.
- **User interviews**: Talk to 5-15 actual users. What do they need? What frustrates them? How do they currently solve the problem?

### 1.3 Competitive Analysis

- Audit 5-10 competitor or comparable sites
- Document for each: navigation, content types, features, visual approach, conversion strategy, SEO positioning, social proof patterns
- Identify gaps: what are competitors doing poorly or not at all?
- Note: also check how competitors appear in AI responses (search for your category in ChatGPT, Claude, Perplexity)

**If the project uses CruxDev's competitive analysis system** (`docs/COMPETITORS.md` exists):
- Use the competitive data as the primary input for this phase — don't duplicate research
- The feature matrix, gap analysis, moat analysis, and threat scores from COMPETITORS.md feed directly into comparison page content
- Call `setup_competitive_analysis` or use existing COMPETITORS.md data to auto-generate `/vs/<competitor>` comparison pages (see §4.6 Comparison Pages)

### 1.4 Requirements

- **Functional**: What the site must do (forms, search, auth, e-commerce, etc.)
- **Content**: What information must exist (pages, posts, media, downloads)
- **Technical**: Performance targets, integrations, accessibility level, hosting requirements
- **Legal/compliance**: Privacy law, accessibility law, industry regulations (see Appendix A)

**Deliverables**: Research report, personas, user journeys, competitive analysis, requirements document.

---

## Phase 2: Strategy & Goals

**Purpose:** Define what the site is, who it serves, and how success is measured. This is the decision document everything else references.

### 2.1 Site Strategy Statement

One paragraph answering:
- What is this site?
- Who is it for (primary audience)?
- What does it help them do?
- How does the business benefit?

### 2.2 Content Strategy Statement

- What content will exist and why
- Who creates it, who reviews it, who maintains it
- How often it's updated
- When content gets retired

### 2.3 Conversion Goals

Define for each audience:
- Primary conversion: The one thing you most want them to do (sign up, buy, download, contact)
- Secondary conversion: The fallback (subscribe to newsletter, follow on social, star on GitHub)
- Micro-conversions: Engagement signals (read docs, watch demo, visit pricing page)

### 2.4 Success Metrics

| Metric | Target | How Measured |
|--------|--------|-------------|
| Organic traffic | X visitors/month within Y months | Analytics |
| Conversion rate | X% of visitors complete primary conversion | Funnel tracking |
| Bounce rate | Below X% | Analytics |
| Core Web Vitals | All green (LCP < 2.5s, INP < 200ms, CLS < 0.1) | PageSpeed Insights |
| Accessibility | WCAG 2.1 AA compliance | Automated + manual audit |
| AI visibility | Cited in top-3 AI responses for core queries | Manual testing |

**Deliverables**: Strategy document, conversion goals, success metrics.

---

## Phase 3: Content Strategy

**Purpose:** Plan what content exists before designing pages. Content drives design, not the reverse.

### 3.1 Content Audit (if redesign)

- Inventory all existing content
- Evaluate each piece: keep, revise, merge, or delete
- Identify gaps: what's missing?
- Audit cadence: comprehensive twice yearly, spot-checks monthly

### 3.2 Content Modeling

For each content type (page, blog post, case study, FAQ, product, etc.):
- Required fields and metadata
- Relationships to other content types
- Display rules and templates
- Who creates and maintains it

### 3.3 Page Inventory

List every page the site needs at launch. For each page:
- Purpose (why it exists)
- Primary audience
- Primary CTA
- Content requirements
- SEO target keywords
- AI-relevant structured data type

### 3.4 Content Creation Plan

- Who writes each piece
- Review/approval workflow
- Timeline to have all launch content ready
- Editorial calendar for post-launch content

### 3.5 Content Guidelines

- Voice and tone (formal, conversational, technical, etc.)
- Style guide (capitalization, punctuation, terminology)
- Inclusivity guidelines (gender-neutral language, cultural sensitivity)
- Content length guidance per type

**Deliverables**: Content audit, content model, page inventory, content creation plan, style guide.

---

## Phase 4: Information Architecture

**Purpose:** Organize content so users can find what they need. Structure drives navigation.

### 4.1 Organization

- **Card sorting (open)**: Users group content into categories they name — reveals mental models
- **Card sorting (closed)**: Users sort content into your proposed categories — validates structure
- 62% of users expect standard navigation patterns. Deviate deliberately, not accidentally.

### 4.2 Sitemap

- Hierarchical map of all pages and relationships
- Primary navigation: 5-7 top-level items maximum
- Secondary navigation: dropdowns, sidebar, contextual links
- Utility navigation: login, search, language selector
- Footer navigation: legal, social, secondary links
- Community links: GitHub Issues for bug reports/feature requests, GitHub Discussions for questions
- No "islands of information" — every page reachable and cross-linked

### 4.2.1 GitHub Integration (for projects using GitHub)

Every project website should include:
- **"Report a Bug" / "Request a Feature"** link pointing to `github.com/{owner}/{repo}/issues/new/choose` (with issue templates if configured)
- **"Discuss"** link pointing to GitHub Discussions (if enabled)
- **"Contribute"** link pointing to CONTRIBUTING.md
- **Star count badge** (live via GitHub API or shields.io)
- **Placement:** Footer (always visible), docs sidebar, and help/support page
- These links feed the autonomous issue monitoring pipeline (see GROWTH_STRATEGY.md §8)

### 4.3 Navigation Design

- Primary nav visible on every page
- Breadcrumbs for deep hierarchies (3+ levels)
- Search as first-class navigation for content-heavy sites (50+ pages)
- Mobile: hamburger menu at ~810px breakpoint, full navigation accessible

### 4.4 URL Structure

- Human-readable, hierarchical URLs mirroring the IA
- Lowercase, hyphens not underscores
- No unnecessary parameters or IDs
- Plan 301 redirects from any existing/legacy URLs

### 4.5 Comparison Pages (if competitive analysis active)

If the project has a `docs/COMPETITORS.md` (from CruxDev's competitive analysis system), generate comparison pages:

**Navigation:**
- Add a "Compare" or "vs" top-level or secondary nav item
- Each official/watch competitor gets its own `/vs/<competitor-slug>` page
- Index page at `/compare` or `/vs` listing all comparisons

**Per-competitor page structure:**
- SEO-optimized title: "[Our Product] vs [Competitor] — Features, Pricing & Comparison"
- Meta description targeting "[our product] vs [competitor]" search queries
- Feature comparison table (from COMPETITORS.md feature matrix)
- Strengths/weaknesses side-by-side
- Pricing comparison (if applicable)
- "When to choose [us]" vs "When to choose [them]" sections
- FAQPage structured data (schema.org) for AI/search visibility
- CTA at bottom

**Data source:** All comparison content is generated from COMPETITORS.md data via CruxDev's `generate_comparison_page()` tool. When COMPETITORS.md is updated (daily competitive monitoring), comparison pages must be regenerated and redeployed.

**URL pattern:** `/vs/competitor-name` (lowercase, hyphens, no special chars)

### 4.6 Validation

- **Tree testing**: Users find content in proposed hierarchy (no visual design, just the tree)
- Iterate until success rate > 80%

**Deliverables**: Sitemap, navigation model, URL map, redirect plan, comparison pages, tree test results.

---

## Phase 5: SEO & AI Visibility Architecture

**Purpose:** Build discoverability into the site's DNA — not bolted on after launch. Plan for both search engines and AI systems.

*Full technical details in `SEO_AND_GEO_REFERENCE.md`. This section covers the strategic decisions.*

### 5.1 Keyword Strategy

- Identify 20-50 target keywords across the funnel:
  - **Awareness**: What problems do users search for? ("how to manage API keys", "best CI/CD tools")
  - **Consideration**: What categories do they compare? ("X vs Y", "top 10 Z tools")
  - **Decision**: What do they search when ready to buy/adopt? ("X pricing", "X tutorial", "X getting started")
- For each keyword: search volume, difficulty, current ranking, content to create
- Focus on long-tail (100-1,000 monthly searches) over competitive head terms
- **Comparison keywords are high-value consideration-stage content**: "[product] vs [competitor]" queries have strong purchase intent. If comparison pages exist (§4.5), each one targets its own keyword cluster.

### 5.2 Content Cluster Architecture

- **Pillar pages** (2,500-4,000 words): Comprehensive coverage of a core topic
- **Cluster pages**: Deep dives on subtopics, linked back to the pillar
- **Internal linking**: Every cluster page links to its pillar and to 2-3 related cluster pages
- Build topical authority by covering a topic thoroughly, not superficially across many topics

### 5.3 Structured Data Plan

For each page type, define the Schema.org markup:
- Homepage: `Organization`, `WebSite` with `SearchAction`
- Blog posts: `Article` with `author`, `datePublished`, `dateModified`
- FAQs: `FAQPage` with `Question` and `Answer`
- How-to content: `HowTo` with `step`
- Products/tools: `SoftwareApplication` with `offers`, `aggregateRating`
- Documentation: `TechArticle`

### 5.4 AI Visibility Strategy

- **llms.txt**: Create `/llms.txt` and `/llms-full.txt` files that explain the site's content to AI systems
- **Answer-first content**: Lead paragraphs answer the question directly in 40-80 words (AI systems extract these)
- **Statistics and specifics**: Include concrete numbers, percentages, benchmarks — AI systems cite specific claims 40% more than vague ones
- **Expert framing**: "According to [specific source]..." and first-person experience signals boost AI citation
- **Structured headings**: Clear H2/H3 hierarchy lets AI parse topic boundaries
- **Brand consistency**: Use exact brand name consistently (AI builds entity recognition from repetition)

### 5.5 Technical SEO Requirements

- Meta tags: unique `<title>` (50-60 chars) and `<meta description>` (150-160 chars) per page
- Open Graph and Twitter Card tags for social sharing
- XML sitemap (auto-generated, submitted to Search Console)
- robots.txt (allow crawling of all public content)
- Canonical URLs on every page
- Semantic HTML (proper heading hierarchy, landmark elements, `<article>`, `<section>`, `<nav>`)

### 5.6 AI Crawler Policy

Decide and document: which AI crawlers are allowed?
- `GPTBot` (OpenAI), `ClaudeBot` (Anthropic), `Google-Extended` (Gemini training)
- Trade-off: blocking preserves content exclusivity; allowing increases AI visibility
- Recommendation for most sites: allow all AI crawlers (visibility > exclusivity)

**Deliverables**: Keyword strategy, content cluster map, structured data plan, llms.txt, technical SEO requirements.

---

## Phase 6: UX Design & Wireframing

**Purpose:** Define layout, hierarchy, and interaction before visual design. Changes here take minutes; changes in code take hours.

### 6.1 Page Templates

Wireframe one template per unique page type (not per page). Common templates:
- Homepage
- Interior/content page
- Blog post / article
- Product / feature page
- Landing page (minimal nav, single CTA)
- Documentation page
- Pricing page
- Contact / form page
- 404 page

### 6.2 Homepage Structure

Research across 100+ successful sites shows this formula:
1. **Hero section**: Headline (5-8 words) + subheadline (1-2 sentences) + primary CTA + visual
2. **Trust block**: Logos, metrics, star counts, or testimonials
3. **Feature/value block**: 3-6 key benefits with supporting detail
4. **Social proof block**: Testimonials, case studies, or user quotes
5. **How it works**: 3-4 step visual walkthrough
6. **Final CTA**: Repeated primary conversion, visually distinct

### 6.3 CTA Strategy

- **One primary CTA per page.** Landing pages with a single CTA convert 32% better than those with five or more.
- Primary CTA: Bold, specific language ("Start building", "Deploy now", "Get the report" — not generic "Submit" or "Click here")
- Secondary CTA: Outlined/lighter style for the alternative action
- Personalized CTAs convert 202% better than generic defaults
- Final CTA at page bottom catches full-page scrollers

### 6.4 Responsive Strategy

- **Mobile-first**: Design for smallest screen first, enhance upward
- Define breakpoints based on content needs, not device sizes
- Touch targets: minimum 44x44 CSS pixels
- Decide per element: does it adapt, reorder, or hide?

### 6.5 Interaction Design

Define all interactive states for every component:
- Default, hover, active, focus, disabled, loading, empty, error, success
- Form validation: inline validation, error messages, success feedback
- Micro-interactions: transitions, progress indicators, feedback animations

### 6.6 Prototype & Test

- Interactive prototype for key user flows
- Usability test with 5-8 users (5 users find 85% of usability problems)
- Iterate before visual design begins

**Deliverables**: Wireframes, interaction specs, responsive strategy, prototype, usability test results.

---

## Phase 7: Design System & Visual Design

**Purpose:** Create a consistent visual language. Build the system, then compose pages from it.

### 7.1 Design Tokens

The atomic units of visual design:
- Colors (primary, secondary, accent, semantic: success/warning/error/info)
- Typography scale (modular ratio, e.g., 1.25: 16, 20, 25, 31, 39px)
- Spacing scale (4, 8, 12, 16, 24, 32, 48, 64, 96px)
- Border radii, shadows, breakpoints
- Single source of truth — change a token, change everywhere

### 7.2 Typography

- Maximum 2 typefaces (heading + body, or one versatile family)
- Body text: minimum 16px on screen
- Line height: 1.4-1.6 for body, 1.1-1.3 for headings
- Line length: 60-80 characters maximum
- Font loading strategy: `font-display: swap` or `optional` to prevent CLS

### 7.3 Color

- All combinations must meet WCAG AA contrast ratios:
  - Normal text: 4.5:1
  - Large text (18px+ bold or 24px+): 3:1
  - UI components and graphical objects: 3:1
- Dark mode: design as a separate theme, not an afterthought
- Don't use color alone to convey meaning (colorblind users)

### 7.4 Component Library

Build these components, each with all states and accessibility requirements:
- Buttons (primary, secondary, outline, ghost, icon-only)
- Forms (inputs, selects, checkboxes, radios, toggles, text areas)
- Cards, modals, dialogs, tooltips, popovers
- Navigation (header, footer, sidebar, breadcrumbs, tabs)
- Tables (sortable, filterable, responsive)
- Alerts, toasts, badges, progress bars
- Code blocks (if applicable — syntax highlighting, copy button)

### 7.5 High-Fidelity Mockups

- Apply design system to wireframes
- Design key pages at mobile, tablet, and desktop breakpoints
- Stakeholder sign-off before development

**Deliverables**: Design system (tokens, components, patterns, guidelines), high-fidelity mockups, approval.

---

## Phase 8: Technical Architecture & Performance

**Purpose:** Define the technical foundation. Performance is a feature — it affects SEO ranking, conversion rate, and user satisfaction.

### 8.1 Performance Budgets

| Metric | Target | Impact |
|--------|--------|--------|
| LCP (Largest Contentful Paint) | < 2.5s | Google ranking factor. 1-second delay = 7% conversion loss |
| INP (Interaction to Next Paint) | < 200ms | Google ranking factor |
| CLS (Cumulative Layout Shift) | < 0.1 | Google ranking factor |
| TTFB (Time to First Byte) | < 800ms | Foundational — affects all other metrics |
| FCP (First Contentful Paint) | < 1.8s | Perceived speed |
| Total page weight | < 500KB initial load | Mobile users on slow connections |
| JavaScript budget | < 200KB compressed | Heavy JS kills INP and delays rendering |

Only 47% of sites pass all Core Web Vitals thresholds. Passing is a competitive advantage.

### 8.2 Image Strategy

- **Formats**: WebP (30% smaller than JPEG) or AVIF (50% smaller). JPEG/PNG as fallback.
- **Responsive**: `srcset` and `sizes` attributes for resolution switching
- **Lazy loading**: `loading="lazy"` for below-the-fold images
- **Dimensions**: Always specify `width` and `height` to prevent CLS
- **Hero image**: Preload the LCP image (`<link rel="preload">`)

### 8.3 Hosting & CDN

- CDN for static assets (CSS, JS, images, fonts)
- Edge caching for HTML where possible
- Geographic distribution — serve from nodes near users
- TTFB improvement: CDN typically reduces TTFB by 40-60%

### 8.4 Security

- **HTTPS**: Mandatory. HSTS header to prevent downgrades.
- **CSP (Content Security Policy)**: Prevent XSS. Avoid `unsafe-inline` and `unsafe-eval`.
- **Security headers**: X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Permissions-Policy
- **Dependency scanning**: Automated vulnerability checks in CI

### 8.5 Accessibility Requirements

WCAG 2.1 AA is the minimum legal standard (US ADA, EU EAA). 5,000+ accessibility lawsuits filed in 2025.

**POUR principles**:
- **Perceivable**: Alt text, captions, contrast, resizable text
- **Operable**: Keyboard navigable, no time traps, skip navigation
- **Understandable**: Clear language, predictable navigation, error prevention
- **Robust**: Valid HTML, correct ARIA, works with assistive tech

### 8.6 Analytics Plan

- Choose a privacy-respecting analytics tool (Plausible, Fathom, PostHog) or GA4
- Define event tracking: what user actions to measure, naming conventions
- Conversion tracking: funnel steps defined before development
- No tracking before consent (GDPR requires opt-in for non-essential cookies)

**Deliverables**: Technical architecture doc, performance budget, security plan, analytics tracking plan.

---

## Phase 9: Development & Content Entry

**Purpose:** Build the site and enter real content. No lorem ipsum.

### 9.1 Development Approach

- Component-driven: build the design system in code first, then assemble pages
- Progressive enhancement: core content works without JavaScript
- Real content from day one — design and development validated against actual text and images
- Continuous integration: automated testing (accessibility, performance, links) on every commit

### 9.2 Content Entry

- All launch content written, reviewed, and entered by content owners
- All images optimized, properly attributed, with alt text
- All metadata filled in (title tags, descriptions, structured data)
- All internal links verified

---

## Phase 10: Pre-Launch QA

**Purpose:** Verify everything works before going live. Use this as a checklist — every item must be verified.

### 10.1 Functional

- [ ] All links work (no 404s)
- [ ] All forms submit correctly
- [ ] All interactive elements function (dropdowns, modals, search, filters)
- [ ] Conversion paths work end-to-end
- [ ] Error states display correctly
- [ ] 404 page exists and is helpful

### 10.2 Content

- [ ] All copy proofread (no placeholder text)
- [ ] All images display and have alt text
- [ ] All media plays correctly
- [ ] Favicon and social sharing images present

### 10.3 Cross-Browser & Device

- [ ] Chrome, Firefox, Safari, Edge tested
- [ ] iOS Safari and Android Chrome tested on real devices
- [ ] Responsive behavior verified at all breakpoints

### 10.4 Performance

- [ ] Core Web Vitals pass on mobile AND desktop
- [ ] Tested on throttled connection (3G simulation)
- [ ] Images compressed and properly sized
- [ ] Unused CSS/JS removed
- [ ] Caching configured

### 10.5 SEO

- [ ] XML sitemap generated and valid
- [ ] robots.txt correct
- [ ] All pages have unique title and description
- [ ] Structured data validates (Rich Results Test)
- [ ] Canonical URLs set
- [ ] Redirects from old URLs tested
- [ ] Google Search Console configured
- [ ] llms.txt published

### 10.6 Security

- [ ] SSL valid
- [ ] Security headers configured
- [ ] No mixed content warnings
- [ ] Form inputs sanitized
- [ ] Staging URLs not publicly accessible

### 10.7 Accessibility

- [ ] Lighthouse accessibility score 95+
- [ ] Keyboard navigation: every interactive element reachable
- [ ] Screen reader tested (VoiceOver or NVDA)
- [ ] Color contrast verified
- [ ] Focus indicators visible
- [ ] Skip navigation link present

### 10.8 Legal

- [ ] Privacy policy published and linked
- [ ] Cookie consent banner functional (opt-in)
- [ ] Terms of service published (if applicable)
- [ ] Accessibility statement published
- [ ] Copyright notices current

### 10.9 Analytics

- [ ] Analytics tracking verified (page views, events, conversions)
- [ ] Uptime monitoring configured
- [ ] Error tracking configured

**Deliverables**: QA report with all items checked, stakeholder sign-off.

---

## Phase 11: Launch

### 11.1 Launch Day

- [ ] Remove `noindex` directives
- [ ] Submit sitemap to Google Search Console and Bing Webmaster Tools
- [ ] Verify redirects active
- [ ] Monitor server performance and error logs in real-time
- [ ] Have rollback plan ready
- [ ] Announce on planned channels

### 11.2 Post-Launch Verification (First 48 Hours)

- [ ] All pages indexing correctly
- [ ] Analytics data flowing
- [ ] Error logs clean
- [ ] Forms and conversions tested on production
- [ ] Social sharing verified (OG images, descriptions)

---

## Phase 12: Post-Launch Operations

**Purpose:** A launched site is the starting line, not the finish line.

### 12.1 Monitoring

- Uptime: target 99.9%+
- Core Web Vitals: continuous monitoring for regressions
- Error rates: server errors, JavaScript errors, broken links
- Security: vulnerability scanning, dependency updates

### 12.2 Analytics Review Cadence

- **Weekly** (first month): Traffic, conversions, bounce rate, top pages, errors
- **Monthly** (ongoing): Trends, content performance, SEO rankings, AI visibility
- **Quarterly**: A/B test results, competitive analysis refresh, strategy review
- **Annually**: Full audit — accessibility, performance, content freshness, IA review

### 12.3 Comparison Page Maintenance (if competitive analysis active)

If the project uses CruxDev's competitive analysis system:
- When `docs/COMPETITORS.md` is updated (daily competitive monitoring), comparison pages must be regenerated
- The WEBSITE_CONVERGENCE phase in the convergence engine auto-triggers this
- Regeneration uses `generate_comparison_page()` from the updated competitor data
- Deploy after regeneration — stale comparison pages damage SEO and credibility
- New competitors added to COMPETITORS.md should get comparison pages added to the nav

### 12.3 Content Maintenance

- Review all content every 3 months minimum
- Update statistics, screenshots, and examples that become stale
- Unpublish or redirect obsolete content — don't leave it to decay
- Track content freshness as a metric

### 12.4 Continuous Improvement

- A/B test one element at a time: headlines, CTAs, layouts, imagery
- Quarterly UX reviews: heatmaps, session recordings, user surveys
- Annual accessibility audit
- Annual performance budget review
- Content cluster expansion based on keyword opportunities

---

## Appendix A: Legal & Compliance

### Privacy

- **GDPR** (EU): Opt-in consent before non-essential cookies. Fines up to 20M EUR or 4% global revenue.
- **CCPA/CPRA** (California): "Do Not Sell My Personal Information" link required. Opt-out rights.
- **Privacy policy**: Required by law. Describe: what data collected, why, how stored, who has access, user rights.
- **Cookie consent**: Must be opt-in (not just notification). No pre-checked boxes. No asymmetric buttons (dark patterns).

### Accessibility

- **ADA** (US): WCAG 2.1 AA is the de facto standard. 5,000+ lawsuits in 2025.
- **European Accessibility Act**: WCAG 2.1 AA mandatory for products/services in the EU (effective June 2025).
- **Section 508** (US federal): All federal government websites.

### Industry-Specific

- **E-commerce**: PCI DSS for payments, consumer protection, return policies
- **Healthcare**: HIPAA
- **Finance**: SEC, FINRA, SOX
- **Education**: FERPA, COPPA (under 13)
- **AI systems**: EU AI Act (August 2026) — penalties up to 35M EUR or 7% turnover

---

## Appendix B: Internationalization

### When to Plan

If there is *any* possibility of international users, architect for i18n from day one. Retrofitting is 5-10x more expensive.

### Technical Requirements

- Externalize all user-facing strings (no hardcoded text)
- UTF-8 encoding (non-negotiable)
- Locale-aware formatting (dates, numbers, currencies) via `Intl` APIs
- RTL layout support if targeting Arabic/Hebrew
- URL strategy: subdirectories (`/en/`, `/fr/`) for most sites, subdomains for large sites
- Text expansion: German ~30% longer than English. Design must accommodate.
- `hreflang` tags for all language versions

### Translation

- Professional translators for quality (machine translation only for drafts)
- Structured file formats (JSON, XLIFF) with unique keys
- Context annotations for translators

---

## Appendix C: Universal Principles

| Principle | Details |
|-----------|---------|
| Content before design | Write real content first. Design serves content, not the reverse. |
| Mobile-first | Design for smallest screen first, enhance upward. |
| Progressive enhancement | Core function works without JS. Enhance with it. |
| One primary action per page | Every page has one clear thing you want users to do. |
| Users scan, not read | Use headings, bullets, bold, short paragraphs. |
| 3-click myth is false | Depth doesn't matter; clarity of each click does. |
| Test with real users | 5 users find 85% of usability problems. |
| Performance is a feature | Speed directly affects conversion, SEO, and satisfaction. |
| Accessibility is universal design | Accessible sites are better for everyone. |
| Ship and iterate | Launch an excellent minimum. Improve based on data. |
| Optimize for AI, not just Google | Content structure, answer-first writing, and structured data serve both. |

---

## Appendix D: Common Mistakes

1. **No discovery phase** — jumping to design without understanding users or goals
2. **Design before content** — lorem ipsum leads to layouts that break with real text
3. **Too many CTAs** — competing actions reduce conversion by up to 266%
4. **No mobile-first** — 60%+ of traffic is mobile
5. **Slow load times** — 53% of mobile users abandon after 3 seconds
6. **Accessibility failures** — 96% of top-million homepages fail basic compliance
7. **No content maintenance** — sites decay within months without governance
8. **Gated content** — email walls for docs alienate users
9. **Marketing buzzwords** — developers (and increasingly all users) detect and reject vague claims
10. **No post-launch iteration** — treating launch as the finish line
11. **Missing structured data** — invisible to rich search results and AI citation
12. **No AI visibility strategy** — optimizing only for Google ignores a growing discovery channel
13. **Dark patterns in consent** — erode trust and increasingly carry legal penalties
14. **Islands of information** — related content scattered with no cross-linking
15. **No dark mode** — users expect OS-respecting themes; pure-light sites feel outdated
16. **No quick start page** — users who can't get started in 5 minutes leave
17. **No install page** — burying install instructions in a README loses users at the first step

---

## Appendix E: Dark Mode

**Research basis:** Material Design dark theme guidelines, Apple HIG, web.dev, WCAG 2.1/2.2, Tailwind CSS v4

### E.1 Detection
- Use `prefers-color-scheme` media query for OS-level detection
- Listen for live changes via `matchMedia('(prefers-color-scheme: dark)').addEventListener('change', ...)`
- Set `<meta name="color-scheme" content="light dark">` and `:root { color-scheme: light dark; }` for native element theming (scrollbars, form controls)

### E.2 CSS Strategy
- Class-based with CSS custom properties (design tokens). NOT media-query-only (can't support toggle).
- Tailwind v4: use `selector` or `class` strategy with `@custom-variant dark`
- Define semantic tokens (bg-primary, text-primary, border-default) that switch between themes
- Token architecture: Primitive → Semantic → Component layers

### E.3 Color Design
| Attribute | Light | Dark |
|-----------|-------|------|
| Background | #ffffff | **#121212** (NOT #000) |
| Text body | #1a1a1a | **#e0e0e0** (NOT #fff) |
| Text secondary | #666666 | #a0a0a0 |
| Surface elevated | white + shadow | **lighter shade** (shadow invisible on dark) |
| Accent colors | Full saturation | **Desaturate 20-30%**, increase lightness |

- Pure black (#000) causes OLED smearing and excessive contrast
- Pure white (#fff) body text causes halation for ~33% of users (astigmatism)
- Surface hierarchy via lightness tiers (Material Design elevation overlay table: 0dp=#121212, 1dp=#1e1e1e, 4dp=#272727, 8dp=#2e2e2e)

### E.4 Toggle UX
- **Three-state:** System (default) / Light / Dark
- Persist choice in `localStorage`
- Icons: sun (light) / moon (dark) / monitor (system)

### E.5 FOUC Prevention (Critical)
Inline blocking script in `<head>` BEFORE stylesheets:
```html
<script>
(function() {
  const stored = localStorage.getItem('theme');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  if (stored === 'dark' || (stored !== 'light' && prefersDark))
    document.documentElement.classList.add('dark');
})();
</script>
```

### E.6 Images
- SVG icons: use `currentColor` (auto-adapts)
- Logos: provide light/dark variants (never CSS invert)
- Photos: optionally reduce brightness 10% on dark backgrounds

### E.7 Accessibility
- WCAG AA (4.5:1 normal text, 3:1 large text) in BOTH modes
- Test every color pair independently for both themes
- Support `forced-colors` media query for Windows High Contrast
- Never color-only status indicators

### E.8 Anti-Patterns
- Pure inversion (`filter: invert(1)`) — breaks images, brand colors
- Pure black background — OLED smearing, eye strain
- Same shadow values — invisible on dark
- Binary toggle only — can't return to OS preference
- Saturated colors on dark — optical vibration
- Different font sizes per theme — causes CLS

---

## Appendix F: Quick Install Page

**Research basis:** Stripe, Vercel, Tailwind CSS, Astro documentation patterns; Google Tech Writing; Write the Docs

Every software project website must have a dedicated install page. Target: **completable in under 60 seconds.**

### F.1 Page Structure
1. **Prerequisites** — exact versions (e.g., "Node.js 18+ required"), with install links
2. **Install command** — one-liner, primary package manager, copy button
3. **Verification** — command + expected output ("you should see:")
4. **Next step** — link to Quick Start guide

### F.2 Platform Tabs
- Auto-detect OS via `navigator.userAgent`, pre-select matching tab
- Tabs: macOS / Linux / Windows (for CLI tools) or npm / yarn / pnpm / cargo (for packages)
- Tab selection persists via localStorage
- Each tab is independently complete
- Maximum 4 options (more causes decision paralysis)

### F.3 Verification Step (Required)
Every install page must end with:
- "Run this:" → `command --version`
- "You should see:" → expected output block
- This is the single most impactful element for user confidence

### F.4 Copy-to-Clipboard
- Every code block gets a copy button (top-right corner)
- "Copied!" feedback for 1-2 seconds
- Exclude shell prompts (`$`, `>`) from copied text

### F.5 Troubleshooting
- Collapsible section at page bottom
- Top 3 issues only (from real support data)
- Format: symptom → cause → fix (one command)
- Final item: "Still stuck? [File an issue]"

### F.6 Anti-Patterns
- Prerequisites assumed, not stated
- Multi-step install when one-liner is possible
- No verification step
- No "what's next" link
- Showing all 6 package managers without recommendation

---

## Appendix G: Quick Start Guide

**Research basis:** Stripe, Vercel, Tailwind, Astro, Supabase documentation; developer onboarding research; TTFV studies

Every software project website must have a quick start guide. Target: **first meaningful result in under 5 minutes.**

### G.1 The 7-Section Structure
1. **One-sentence description** — what this tool does, for whom
2. **Prerequisites** — what must be installed (bulleted, with versions)
3. **Install** — the install command (or link to install page)
4. **Initialize/Setup** — create project or config file (1-2 commands)
5. **First meaningful action** — the "hello world" demonstrating core value
6. **See the result** — expected output, screenshot, or URL
7. **What's next** — 3-5 links to logical next topics

### G.2 Time-to-First-Value
| Rating | Time | Examples |
|--------|------|---------|
| Gold | < 2 min | Vercel |
| Good | < 5 min | Stripe, Astro |
| Acceptable | < 10 min | Complex tools |
| Failure | > 15 min | Users abandon |

Measure with a stopwatch on a fresh machine, zero prior knowledge.

### G.3 Code Examples
- Every block must include imports/requires
- Every block must be runnable as-is (no `...` or `// your code here`)
- Show file path above code block (e.g., `src/main.rs`)
- Use realistic, domain-relevant data (not `foo`/`bar`)
- Show expected output after execution
- Test code examples in CI

### G.4 Progressive Disclosure
- Quick start shows ONE use case: the most common one
- One linear path, no branching
- Advanced options link to separate pages
- Maximum 1-2 callout boxes ("Note" / "Pro tip")
- If a step has options, pick the recommended one

### G.5 "What's Next" Section (Required)
- 3-5 links, ordered by likely next action
- Each link gets a one-line description
- Include: deeper tutorial, core concepts, API reference, examples
- This prevents the "now what?" drop-off (#1 cause of post-install abandonment)

### G.6 Framework-Specific Paths
- If multiple frameworks supported, offer separate paths from the start
- Each path is complete and self-contained
- Use tab groups or separate pages
- Limit to 4-6 options; "Other" for the rest

### G.7 Keeping Current
- Test all code examples in CI
- Pin versions, update with releases
- Include snippet files from tested sources (not inline-only code)
- Review quick start in every release checklist

### G.8 Anti-Patterns
- Wall of text before first action
- Outdated code examples
- Incomplete snippets (missing imports)
- No verification step
- No "what's next" section
- Using "just" or "simply" (minimizes difficulty)
- Mixing conceptual and procedural content
- Too many choices too early

---

## Appendix H: Website Essential Pages Checklist

Every project website must evaluate whether it needs these pages. Website convergence checks for their presence:

| Page | Required When | Key Criteria |
|------|-------------|-------------|
| **Homepage** | Always | Hero, value prop, trust metrics, primary CTA |
| **Quick Install** | Software projects | < 60 seconds, verification step, copy buttons |
| **Quick Start** | Software projects | < 5 minutes TTFV, 7-section structure, runnable code |
| **Documentation** | Projects with API/config | Organized, searchable, current |
| **Comparison (vs/)** | Projects with competitors | Feature table, honest strengths/weaknesses |
| **Blog/Changelog** | Active projects | Regular updates, build-in-public content |
| **About/Team** | Projects seeking trust | Who builds this, why |
| **llms.txt** | All projects | AI discoverability |
| **robots.txt + sitemap** | All projects | Search engine access |
