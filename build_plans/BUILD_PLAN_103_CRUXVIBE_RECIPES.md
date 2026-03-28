# BUILD_PLAN_103: CruxVibe — Mix-and-Match Recipe Platform

**Status:** IN PROGRESS
**Priority:** Critical (REVENUE — this is the business model)

## The Vision

Authors, creators, and entrepreneurs are paying 30-50% of their revenue to platforms (Patreon 8-12%, Amazon 30-65%, Substack 10%). CruxVibe gives them the same capabilities for $100/month flat — they keep everything else.

**How:** Closed-source "recipes" — pre-built, CruxDev-converged modules that snap together. Pick your recipes, we build your customized platform, we host it, we manage it. You create, we handle the infrastructure.

## The Recipe Model

A recipe is a self-contained, convergence-verified module that solves one problem:

### Creator Economy Recipes

| Recipe | Replaces | What It Does | Saves |
|---|---|---|---|
| **Patreon Recipe** | Patreon ($500-5000/month for mid-tier creators) | Subscription tiers, early access chapters, member management, Stripe billing | 8-12% platform fee |
| **ePublishing Recipe** | Amazon KDP ($0.35-2.10 per sale in fees) | Generate .epub, .mobi, .pdf from manuscript → direct sales | 30-65% Amazon cut |
| **Storefront Recipe** | Gumroad, Shopify ($30-300/month + fees) | Product pages, checkout, digital delivery, Stripe | Platform fees + monthly |
| **Reader App Recipe** | Kindle app lock-in | iOS + Android app for reading subscriptions, point at your site | Kindle ecosystem lock-in |
| **Newsletter Recipe** | ConvertKit ($300+/month at scale), Substack (10%) | Email sequences, subscriber management, paid tiers | Platform fees |
| **Course Recipe** | Teachable ($149-399/month), Kajabi ($199-499) | Video hosting, curriculum, student progress, certificates | $200-500/month |
| **Podcast Recipe** | Patreon for audio + Buzzsprout ($24/month) | Premium audio feed, subscriber-only episodes, RSS | Platform fees |
| **Community Recipe** | Discord (free but unmonetized), Circle ($89-399) | Forums, chat, member directory, gated content | Platform fees |

### Technical Foundation Recipes

| Recipe | What It Does |
|---|---|
| **Stripe Integration** | Payment processing, subscriptions, one-time purchases, webhooks |
| **Auth Recipe** | User registration, login, OAuth, magic links, session management |
| **Media Storage** | S3-compatible storage, CDN, signed URLs for premium content |
| **Analytics Recipe** | Privacy-respecting analytics (no Google), creator dashboard |
| **SEO Recipe** | Sitemap, meta tags, structured data, social cards |
| **Email Recipe** | Transactional email (SES/Postmark), newsletter delivery |

## The Author Journey (Zogarth Example)

```
Author writes on RoyalRoad (free)
  → Story takes off, 10K readers
  → "I need to monetize"

WITHOUT CruxVibe:
  → Patreon: $5K/month, Patreon takes $600 (12%)
  → Kindle Unlimited: $3K/month, Amazon takes $1,950 (65%)
  → Author keeps: $5,450/month from $8,000 revenue
  → Locked into two platforms that can change terms anytime

WITH CruxVibe:
  → Pick recipes: Patreon Recipe + ePublishing Recipe + Reader App
  → CruxVibe builds customized site, hosts it
  → $100/month flat + Stripe processing (2.9%)
  → $5K subscriptions: Stripe takes $145, CruxVibe takes $100 = author keeps $4,755
  → $3K book sales: Stripe takes $87, author keeps $2,913
  → Author keeps: $7,668/month from $8,000 revenue
  → SAVINGS: $2,218/month ($26,616/year)
  → Author OWNS the relationship with their readers
  → No platform can delist them, change terms, or take a bigger cut
```

## Architecture

```
CruxVibe Platform
├── Recipe Registry (closed source — the moat)
│   ├── Patreon Recipe (Elixir/Phoenix/Ash)
│   ├── ePublishing Recipe (epub/mobi/pdf generation)
│   ├── Storefront Recipe (Stripe checkout)
│   ├── Reader App Recipe (React Native/Expo)
│   ├── Newsletter Recipe (email delivery)
│   ├── Course Recipe (video + curriculum)
│   ├── Podcast Recipe (audio feeds)
│   └── Community Recipe (forums + chat)
├── Recipe Combinator (picks + customizes recipes per creator)
├── Hosting Platform (managed infrastructure — $100/month)
├── CruxDev Engine (convergence-verifies everything)
├── CruxBot (autonomous maintenance, updates, monitoring)
└── Creator Dashboard (analytics, revenue, content management)
```

## Phase 0: Project Setup

- [x] 0.1 Phoenix project: `mix phx.new cruxvibe --live --no-dashboard`
- [x] 0.2 Add deps: ash, ash_postgres, ash_authentication, ash_phoenix, oban, stripity_stripe
- [ ] 0.3 Configure Ash domains, Ecto repos, tenant context
- [ ] 0.4 Multi-tenant setup per MULTI_TENANT_PATTERNS.md (shared DB + tenant_id + RLS)
- [ ] 0.5 Stripe Connect setup per STRIPE_CONNECT_PATTERNS.md (Express accounts)
- [ ] 0.6 Integrate BP002 recipe lifecycle (issues, voting, versioning, auto-evolution)

**Reference patterns:** MULTI_TENANT_PATTERNS.md, STRIPE_CONNECT_PATTERNS.md, SUBSCRIPTION_BILLING_PATTERNS.md, DEVELOPMENT_PATTERNS_PETAL.md

## Phase 1: Core Platform

- [ ] 1.1 Recipe specification format (what a recipe contains, how recipes compose)
- [ ] 1.2 Patreon Recipe v1 (Elixir/Phoenix/Ash/LiveView)
  - Subscription tiers (define N tiers with prices)
  - Stripe integration (checkout, billing portal, webhooks)
  - Member management (who has access to what)
  - Content gating (chapters visible per tier)
  - Dashboard (subscriber count, revenue, churn)
- [ ] 1.3 ePublishing Recipe v1
  - Manuscript → .epub generation (from markdown)
  - .mobi generation (Kindle-compatible)
  - .pdf generation (print-ready)
  - Cover image integration
  - Metadata (ISBN, author, description)
  - Direct download with purchase verification

## Phase 2: Hosting Platform

- [ ] 2.1 Multi-tenant infrastructure (one instance per creator, isolated)
- [ ] 2.2 Custom domain support (author's own domain)
- [ ] 2.3 SSL certificate provisioning (Let's Encrypt)
- [ ] 2.4 CDN for media (Cloudflare or S3+CloudFront)
- [ ] 2.5 Automated deployment from CruxDev convergence
- [ ] 2.6 Monitoring + alerting (uptime, errors, performance)
- [ ] 2.7 Backup + restore

## Phase 3: Reader App (DEFERRED to v2 — web-only for v1)

- [ ] 3.1 React Native/Expo app (iOS + Android)
- [ ] 3.2 Add subscription endpoint (point at your CruxVibe-hosted site)
- [ ] 3.3 Chapter list, reading progress, bookmarks
- [ ] 3.4 Offline reading (download chapters)
- [ ] 3.5 Push notifications (new chapter available)
- [ ] 3.6 Publish to App Store + Google Play

## Phase 4: Creator Onboarding

- [ ] 4.1 "I'm an author starting a business" flow
- [ ] 4.2 Recipe picker (which recipes do you need?)
- [ ] 4.3 Customization wizard (brand colors, domain, tier structure)
- [ ] 4.4 CruxDev adoption of the creator's content (books, chapters)
- [ ] 4.5 Site generation + deployment
- [ ] 4.6 Stripe Connect onboarding (creator's own Stripe account)
- [ ] 4.7 Go-live checklist

## Phase 5: More Recipes

- [ ] 5.1 Newsletter Recipe
- [ ] 5.2 Course Recipe
- [ ] 5.3 Podcast Recipe
- [ ] 5.4 Community Recipe
- [ ] 5.5 Each recipe follows the same pattern: CruxDev-converged, Stripe-integrated, managed hosting

## Business Model

| Tier | What They Get | Price |
|---|---|---|
| Starter | 1 recipe + hosting + custom domain | $49/month |
| Creator | 3 recipes + hosting + reader app listing | $99/month |
| Pro | Unlimited recipes + priority support + analytics | $199/month |
| Enterprise | White-label + API access + dedicated instance | Custom |

**Revenue per creator:** $100/month average
**At 1,000 creators:** $100K/month ($1.2M/year)
**At 10,000 creators:** $1M/month ($12M/year)
**Margin:** High — hosting costs ~$5-10/creator/month

## What Makes This Defensible

1. **Recipes are closed source** — can't be replicated without CruxDev's convergence methodology
2. **CruxDev converges quality** — every recipe is verified to the same standard
3. **CruxBot maintains everything** — autonomous updates, security patches, monitoring
4. **Network effects** — reader app becomes a discovery platform (readers find new authors)
5. **Switching cost** — once set up, creators don't want to migrate (but they CAN export everything)

## The Tagline

**CruxVibe: Keep your money. We handle the rest.**

## Verification

```bash
mix test --trace
mix credo --strict
mix dialyzer
```
