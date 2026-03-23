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
