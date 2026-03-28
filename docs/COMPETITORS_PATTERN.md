# Competitors Pattern

**Version:** 1.0
**Created:** 2026-03-22
**Scope:** Universal — applies to any CruxDev-managed project that has competitors.

This document defines the methodology for competitive research, tracking, gap analysis, and automated gap closure. It parallels `DEVELOPMENT_PATTERNS_CRUXDEV.md` (for code) and `WEBSITE_PLANNING.md` (for websites).

---

## Table of Contents

1. [Phase 1: Competitive Discovery](#phase-1-competitive-discovery)
2. [Phase 2: Deep Research](#phase-2-deep-research)
3. [Phase 3: Selection & Tracking](#phase-3-selection--tracking)
4. [Phase 4: Gap Analysis](#phase-4-gap-analysis)
5. [Phase 5: Gap Closure](#phase-5-gap-closure)
6. [Phase 6: Website Integration](#phase-6-website-integration)
7. [Phase 7: Continuous Monitoring](#phase-7-continuous-monitoring)
8. [Appendix A: COMPETITORS.md Template](#appendix-a-competitorsmd-template)
9. [Appendix B: Verification Standards](#appendix-b-verification-standards)
10. [Appendix C: Anti-Patterns](#appendix-c-anti-patterns)

---

## Phase 1: Competitive Discovery

**Purpose:** Cast a wide net. Find every product that occupies the same category, adjacent categories, or could plausibly be compared by a user evaluating your project.

### 1.1 Discovery Sources

Search across all of these. Do not stop at the first few results.

| Source | What to search | Why |
|--------|---------------|-----|
| GitHub | Category keywords, "awesome-X" lists, trending repos | Direct competitors, open source alternatives |
| Product Hunt | Category and related tags | Commercial competitors, emerging products |
| Google Search | "[your category] tools", "[your category] alternatives", "best [X] 2026" | What users actually search for |
| AI Search | Same queries in ChatGPT, Claude, Perplexity | What AI recommends (future discovery channel) |
| Reddit | r/programming, r/[language], subreddits for your domain | What developers actually use and recommend |
| Hacker News | Search for category keywords, "Show HN" posts | Technical community signal |
| Twitter/X | Category keywords, tool mentions | Developer mindshare |
| Stack Overflow | Tags related to your domain | What tools people ask about |
| G2 / Capterra | Category listings | Commercial/enterprise competitors |
| Competitor websites | "Alternatives to X" pages, comparison pages | Competitors self-identify their competitive set |

### 1.2 Discovery Criteria

Include a project as a potential competitor if ANY of these are true:
- It solves the same core problem
- A user evaluating your project would also evaluate this one
- It appears in the same "awesome" lists or comparison articles
- It targets the same audience, even with a different approach
- It could expand into your space (adjacent competitors)

### 1.3 Discovery Output

A raw list of every potential competitor found, with:
- Name and URL
- GitHub stars (if open source)
- One-line description
- Where you found it
- Category: direct / adjacent / aspirational

**Convergence criterion:** Two consecutive discovery passes (different search terms, different sources) find zero new competitors not already on the list.

---

## Phase 2: Deep Research

**Purpose:** For each potential competitor, build a comprehensive profile. Every claim must be verified. Every link must be tested.

### 2.1 Research Per Competitor

For each competitor on the discovery list, document:

**Identity:**
- Official name, URL, GitHub URL
- Company/creator
- License (open source? commercial? freemium?)
- Pricing (free tier? paid plans? enterprise?)
- First release date, latest release date
- Community size (GitHub stars, Discord members, npm downloads, etc.)

**Product:**
- One-paragraph description of what it does
- Key features (list of 5-10 capabilities)
- Architecture/approach (how it works technically)
- Supported platforms/languages/integrations
- Documentation quality (rate 1-5: absent, basic, adequate, good, excellent)

**Traction:**
- GitHub stars and growth rate (use star-history.com)
- npm/pip/brew download counts if available
- Community activity (issues opened/closed per month, PRs, Discord activity)
- Notable users/companies (verified — check their public repos or statements)
- Funding (if known)

**Strengths:**
- What do they do better than you? Be honest.
- What unique capabilities do they have?
- What do users praise in reviews/discussions?

**Weaknesses:**
- What do they do poorly?
- What do users complain about?
- What gaps or missing features are frequently requested?

### 2.2 Verification Standards

Every piece of information must be verified:

| Claim type | Verification method |
|-----------|---------------------|
| Feature exists | Test it yourself OR find it in their docs/code |
| GitHub stars | Check github.com directly (not cached counts) |
| Download counts | Check npm/pypi/brew directly |
| Pricing | Check their pricing page directly |
| User/company claims | Find public evidence (blog post, repo, tweet) |
| Performance claims | Find benchmarks or test yourself |
| Links | Every URL tested — must return 200, not redirect to 404 |

**If a claim cannot be verified, it must be marked as unverified.**

### 2.2b Strategic Analysis (Per Competitor)

Beyond product features, assess each competitor on these strategic dimensions:

**Moat Analysis — What makes them defensible?**

Score each moat type 0-3 (0=none, 1=weak, 2=moderate, 3=strong):

| Moat Type | What to look for |
|-----------|-----------------|
| Network Effects | Value increases as more users join (marketplaces, platforms, communities) |
| Switching Costs | How painful/expensive is it to leave? (data lock-in, workflow integration, learning curve) |
| Brand | Do users search for the brand name instead of the category? Mindshare dominance. |
| Data Flywheel | Product improves as more data flows through it. Dynamic, not static data. |
| Regulatory/Compliance | Certifications, licenses, security approvals that take years to obtain. |
| Execution Speed | Ship velocity, iteration cycles, time-to-market advantage. |
| Cost Advantage | Can they deliver at lower cost due to scale, architecture, or efficiency? |

**Threat Assessment**

| Dimension | Score 1-5 | Question |
|-----------|-----------|----------|
| Market Overlap | | How much of your TAM do they serve? |
| Growth Velocity | | How fast are they growing? (stars/month, downloads/month, funding) |
| Resource Asymmetry | | Do they have more funding/people/distribution than you? |
| Technical Proximity | | How close is their architecture to yours? Could they replicate your features? |
| Time to Relevance | | How soon could they become a serious threat? (months) |

**Threat Level** = average score. >=4 = existential, 3-4 = significant, 2-3 = moderate, <2 = low.

**Market Sizing (TAM/SAM/SOM)**

For each competitor (and yourself), estimate:
- **TAM:** Total addressable market — everyone who could use this category of tool
- **SAM:** Serviceable available market — segment you can realistically reach
- **SOM:** Serviceable obtainable market — what you can capture in 12-24 months

Use both top-down (market reports) and bottom-up (customer count × price) approaches.

**Business Model Analysis**

| Element | What to document |
|---------|-----------------|
| Revenue Model | How do they make money? (SaaS, usage, freemium, open-core, services) |
| Pricing Tiers | Free/pro/enterprise, per-seat/per-usage/flat |
| Unit Economics | CAC, LTV if estimatable from public data |
| Sustainability | Is their model self-sustaining or VC-subsidized? |

**Value Curve (Blue Ocean Strategy Canvas)**

For your category, identify 5-8 key factors of competition. Score each competitor and yourself 1-5 on each factor. Plot as a line chart. Look for:
- Where your curve diverges (differentiation)
- Factors to eliminate, reduce, raise, or create
- Uncontested market space

**Jobs-to-be-Done**

For each competitor, map which customer jobs they address:
- Core job (what the user is hired to do)
- Related jobs (adjacent needs they solve)
- Emotional/social jobs (how they make users feel)
- Satisfaction level per job (underserved, adequately served, overserved)

### 2.3 Convergence Criterion

Two consecutive research passes on each competitor find zero new significant information not already documented.

---

## Phase 3: Selection & Tracking

**Purpose:** From the full discovery list, select which competitors to officially track, compete with, and build comparison pages for.

### 3.1 Selection Criteria

Score each competitor on these dimensions (1-5 each):

| Dimension | Question |
|-----------|----------|
| Relevance | How directly do they compete with your product? |
| Threat | How much market share could they take from you? |
| Mindshare | How often do users mention them when evaluating your category? |
| Learning | How much can you learn from their approach? |
| Differentiation | How clearly can you differentiate from them? |

**Official competitors** (track actively, build comparison pages): Score >= 15
**Watch list** (monitor periodically): Score 10-14
**Noted** (aware of, no active tracking): Score < 10

### 3.2 Tracking Tiers

| Tier | Cadence | Actions |
|------|---------|---------|
| Official | Daily | Update COMPETITORS.md, refresh comparison page, check for new features, update gap analysis, auto-converge new gaps |
| Watch | Weekly | Quick review — any major changes? Promote to official if needed. |
| Noted | Monthly | Still exists? Still relevant? |

### 3.3 Output

- Ranked list of competitors by tier
- Recommendation for which to track (with rationale)
- Presented to user for approval before proceeding

---

## Phase 4: Gap Analysis

**Purpose:** For each official competitor, identify what they have that you don't, and what you have that they don't. This drives the gap closure plans.

### 4.1 Feature Matrix

Build a feature-by-feature comparison matrix:

```
| Feature              | Your Project | Competitor A | Competitor B |
|----------------------|:----------:|:----------:|:----------:|
| Feature 1            | ✓          | ✓          | —          |
| Feature 2            | —          | ✓          | ✓          |
| Feature 3            | ✓          | —          | —          |
```

### 4.2 Gap Classification

For each gap (feature they have, you don't):

| Classification | Criteria | Action |
|---------------|----------|--------|
| **Must close** | Users explicitly ask for this; losing deals/adoption over it | Generate build plan immediately |
| **Should close** | Would strengthen competitive position; users would benefit | Generate build plan, prioritize by effort/impact |
| **Nice to have** | Marginal value; not a differentiator | Track, build if low effort |
| **Intentional gap** | You deliberately don't have this (design philosophy) | Document the rationale in COMPETITORS.md |
| **Moat** | You have it, they don't — your differentiator | Protect and promote on website |

### 4.3 Gap Analysis Output

- Feature matrix
- Classified gaps with rationale
- Recommended build plan queue (ordered by priority)
- Moat inventory (your unique advantages to promote)

---

## Phase 4B: Integration Ecosystem Analysis

**Purpose:** Track what platforms/tools each competitor integrates with and at what depth. Integration ecosystems are often the deciding factor for adoption.

### 4B.1 Integration Inventory

For each competitor, document:

| Field | Description |
|-------|-------------|
| **name** | Platform/tool name (e.g., "VSCode", "Slack", "GitHub") |
| **depth** | native (core feature), plugin (official), api (REST/webhook), community (third-party) |
| **description** | What the integration does |

### 4B.2 Integration Comparison Matrix

Build a matrix: rows = integrations, columns = us + competitors.

Example:
| Integration | Us | Superpowers | OpenClaw |
|-------------|-----|-------------|----------|
| MCP | native | N/A | native |
| GitHub Issues | native | N/A | community |
| Slack | N/A | N/A | native |

### 4B.3 Integration Gap Classification

Same priority system as feature gaps:
- **must-close:** 2+ official competitors have the integration
- **should-close:** 1 official competitor has it
- **nice-to-have:** only watch/noted competitors have it

### 4B.4 Integration-Driven Build Plans

Each integration gap generates a build plan — same as feature gaps. Integration pages on the website are the output: per-integration setup guide, verified claims only, roadmap clearly separated.

### 4B.5 Per-Integration Documentation Rule

Integration specifics must NOT be inline in core docs. Each integration gets:
- Its own page on the website (/integrations/[name])
- Its own config section in install guide
- Its own troubleshooting section
- Managed as a sub-entity with its own convergence state

## Phase 5: Gap Closure

**Purpose:** Automatically generate and queue build plans to close competitive gaps.

### 5.1 Build Plan Generation

For each "must close" and "should close" gap:

1. Create `build_plans/BUILD_PLAN_NNN_GAP_<competitor>_<feature>.md`
2. Follow the standard build plan template (`/plan`)
3. Include in the plan:
   - Which competitor(s) have this feature
   - How they implement it (reference their docs/approach)
   - How you will implement it (your approach, fitting your architecture)
   - Success criteria (when is the gap closed?)
   - Verification: your implementation matches or exceeds the competitor's

### 5.2 Execution Priority

```
1. Must-close gaps (losing users over this)
2. Moat reinforcement (protect what you uniquely have)
3. Should-close gaps (ordered by effort/impact ratio)
4. Nice-to-have (only if low effort)
```

### 5.3 Post-Closure Update

After each gap is closed:
- Update COMPETITORS.md feature matrix
- Update comparison page on website
- Remove from gap list, add to moat inventory if now unique

---

## Phase 6: Website Integration

**Purpose:** Generate comparison pages on the project website. One page per official competitor.

### 6.1 Comparison Page Structure

Each `/vs/<competitor>` page should contain:

1. **Title**: "[Your Product] vs [Competitor]" (SEO-optimized)
2. **One-paragraph summary**: Honest, balanced framing
3. **Feature comparison table**: The feature matrix for this pair
4. **Key differences**: 3-5 bullet points highlighting where you win
5. **Where they win**: 1-2 bullet points — be honest, builds trust
6. **Migration guide** (if applicable): How to switch from them to you
7. **CTA**: "Try [Your Product]" → quickstart

### 6.2 SEO for Comparison Pages

- Title tag: "[Your Product] vs [Competitor] — Comparison [Year]"
- Target keywords: "[competitor] alternative", "[your product] vs [competitor]"
- Schema.org: `FAQPage` with "How does X compare to Y?" questions
- Update annually (add year to title, refresh data)

### 6.3 Verification

- Every feature claim verified against both products
- Every link tested (competitor docs, pricing, etc.)
- Page reviewed for fairness — biased comparison pages erode trust
- Two consecutive clean passes

---

## Phase 7: Continuous Monitoring

**Purpose:** Keep competitive intelligence current. Detect changes early.

### 7.1 Monitoring Triggers

Set up monitoring for each official competitor:

| What to monitor | How | Cadence |
|----------------|-----|---------|
| New releases | GitHub releases, changelog, blog | Daily |
| Star growth | star-history.com / GitHub API | Daily |
| New features | Release notes, docs changes | Daily |
| Pricing changes | Pricing page snapshot | Weekly |
| Community sentiment | Reddit, HN, Twitter mentions | Daily |
| New competitors | Repeat Phase 1 discovery | Weekly |

### 7.2 Autonomous Update Protocol

When a change is detected, the system acts autonomously:

1. Update COMPETITORS.md with new information
2. Re-run gap analysis for affected competitor
3. If new gap is "must close" → auto-generate build plan → auto-converge it
4. If gap is "should close" → auto-generate build plan → auto-converge it
5. Update comparison page on website
6. Deploy website updates
7. Converge documentation (two clean passes)

No human approval required. The system self-improves continuously. This is the autonomous evolution loop — the same principle as CruxDev's convergence engine applied to competitive position.

### 7.3 Weekly Review

Every week:
- Review all official competitors — any tier changes?
- Review watch list — any promotions to official?
- Re-run discovery (Phase 1) — any new entrants?
- Refresh all comparison pages with current data
- Update feature matrix

---

## Appendix A: COMPETITORS.md Template

```markdown
# Competitors

**Last Updated:** [date]
**Project:** [project name]

## Official Competitors (Actively Tracked)

### [Competitor Name]
- **URL:** [verified link]
- **GitHub:** [verified link] ([star count] stars)
- **Category:** [direct / adjacent]
- **License:** [MIT / Commercial / etc.]
- **One-line:** [what it does]

**Strengths:**
- [strength 1]
- [strength 2]

**Weaknesses:**
- [weakness 1]
- [weakness 2]

**Our moat vs them:**
- [what we have that they don't]

**Their moat vs us:**
- [what they have that we don't — classified as must-close / should-close / intentional gap]

---

## Watch List

| Name | URL | Stars | Why watching |
|------|-----|-------|-------------|
| [name] | [url] | [stars] | [reason] |

## Feature Matrix

| Feature | Our Project | Competitor A | Competitor B |
|---------|:---------:|:----------:|:----------:|
| [feature] | ✓ | ✓ | — |

## Gap Closure Queue

| Gap | Competitor | Classification | Build Plan | Status |
|-----|-----------|---------------|------------|--------|
| [feature] | [name] | Must close | BUILD_PLAN_NNN | Not started |
```

---

## Appendix B: Verification Standards

| Standard | Requirement |
|----------|------------|
| All URLs | Must return HTTP 200 (test with curl or fetch) |
| GitHub stars | Checked directly, not from cached badges |
| Feature claims | Verified in docs, code, or by testing |
| Pricing | Verified on pricing page (screenshot date-stamped) |
| User claims | Public evidence required (link to source) |
| Unverified claims | Must be explicitly marked "[unverified]" |
| Comparison pages | Reviewed for factual accuracy and fairness |

---

## Appendix C: Anti-Patterns

| Anti-Pattern | Why it's wrong | What to do instead |
|-------------|----------------|-------------------|
| Cherry-picking features | Users notice and lose trust | Include features where you lose, too |
| Stale comparison pages | Wrong information is worse than none | Set quarterly refresh cadence |
| Ignoring new competitors | Blindsided by entrants you didn't track | Quarterly discovery re-runs |
| Closing every gap | Spreading too thin, losing differentiation | Classify gaps — some are intentional |
| Only tracking direct competitors | Miss adjacent threats and inspiration | Include adjacent and aspirational |
| FUD (fear, uncertainty, doubt) | Erodes your credibility, not theirs | State facts, let users decide |
| No moat inventory | Don't know what to protect or promote | Track your unique advantages actively |
| Manual-only monitoring | Falls behind immediately | Automate what you can, schedule the rest |

---

## Phase 8: Vertical Content Expansion

**Every project lives in one or more verticals.** A vertical is the market category where the project's users are choosing between alternatives. Competitive analysis isn't just about direct competitors — it's about understanding the entire vertical and using content to own it.

### 8.1 Vertical Identification

For each project, identify its verticals:
- A book project → "AI for authors", "self-publishing tools", "writing assistants"
- A SaaS product → "AI coding tools", "developer productivity", "code quality"
- A coaching business → "AI for coaches", "coaching platforms", "client management"
- A podcast → "AI for podcasters", "podcast production", "content repurposing"

### 8.2 Content Dimensions per Vertical

Each vertical generates 5 types of pages:

| Dimension | Template | Example |
|---|---|---|
| **Feature/tool pages** | One page per major feature, in-depth with examples | `/features/manuscript-tracking/` |
| **Persona pages** | One page per target user type | `/for/authors/`, `/for/entrepreneurs/` |
| **Vertical overview** | "AI tools for X" comparison page | `/vs/ai-for-authors/` |
| **Per-competitor vs pages** | Individual matchup with feature matrix | `/vs/sudowrite/`, `/vs/descript/` |
| **Landing pages** | SEO/GEO pages for search queries in the vertical | `/lp/how-to-use-ai-to-write-a-book/` |

### 8.3 The Content-Product Feedback Loop

```
Identify vertical
  → Research competitors in that vertical
  → Build per-competitor vs pages with feature matrices
  → Gaps in the matrix = features we don't have
  → Gaps become build plans
  → Build plans converge → features ship
  → Update vs pages (gap closed)
  → Blog + X post about the new capability
  → New pages attract users in that vertical
  → Users file issues → new gaps identified
  → Repeat
```

**The vs page IS the gap analysis.** It's public, indexed, and accountable. If we claim parity on a feature, it better work. If we show a gap, it better have a build plan.

### 8.4 Vertical-Specific Audit Dimensions

When a project has competitive pages, the convergence engine should audit:
- Feature matrix accuracy (are our claims verifiable?)
- Competitor info freshness (when was this last verified?)
- Gap-to-build-plan traceability (does every gap have a plan?)
- Landing page SEO (are we ranking for target queries?)
- Persona coverage (do all target personas have a page?)

### 8.5 Scale Formula

For a project with N competitors across M verticals:
- Feature pages: ~20 per project (major features only)
- Persona pages: 5-10 per project
- Vertical overviews: M pages
- Per-competitor vs pages: N pages (across all verticals)
- Landing pages: 5 × (20 + 10 + M + N) queries
- **Total: approximately 150-650 pages per project**

This is not manual — the convergence engine generates page templates, the LLM fills them during research convergence, and the BIP pipeline publishes them.

### 8.6 This Is Universal

This pattern applies to EVERY CruxDev-managed project with competitors and a website:
- A Careiance book series → vs pages against other women's wellness authors
- A SaaS product → vs pages against competing SaaS tools
- A coaching practice → vs pages against other coaching platforms
- An open source tool → vs pages against alternative tools

The methodology is the same. The content adapts to the vertical.
