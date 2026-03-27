# Autonomous Growth Strategy

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 30+ sources including GitHub API docs, Typefully API, X API v2, Baymard, NNg, Supabase/Astro/Vercel case studies, HN/Reddit policy analysis
**Last updated:** 2026-03-27
**Key constraint:** Every action must be executable by the CruxDev convergence engine without human intervention.

---

## 1. The Autonomous Growth Loop

This is the core engine. It runs on every meaningful push or as a daily cron. Zero human intervention.

```
git push to main (or daily cron)
    │
    ├─► Generate changelog from git diff
    ├─► Create GitHub Release (GitHub API)
    ├─► Optimize README if features changed
    ├─► Compose + post to X/Twitter (Typefully API)
    ├─► Generate/update one SEO content page
    ├─► Triage new GitHub issues (welcome, label, respond)
    ├─► Check competitive impact → update COMPETITORS.md if needed
    ├─► Track metrics (stars, impressions, traffic, issues)
    └─► Adjust content strategy based on what performed best
```

**Expected output:** Steady compound growth — not viral spikes. Consistent presence that makes viral moments more likely when they happen.

---

## 2. Channel Strategy (Ranked by Autonomous Feasibility)

### Tier 1: Fully Autonomous (No Human Gate)

| Channel | Method | Frequency | Risk |
|---------|--------|-----------|------|
| **X/Twitter** | Typefully API. Release notes, benchmarks, tips, build-in-public. | 1-3 posts/day | Low |
| **GitHub Releases** | GitHub API. Auto-generate from git log. | Per release | None |
| **GitHub README** | Commit via git. Optimize quick-start, badges, demo. | Per feature change | None |
| **Blog/SEO pages** | Generate technical tutorials with working code. Publish to static site. | 1-2/week | Medium (quality gate) |
| **GitHub issue triage** | GitHub API. Welcome contributors, label, suggest good-first-issue. | Continuous | Low |
| **vs/ comparison pages** | Generate from COMPETITORS.md via BP018 pipeline. | On competitive change | None |
| **llms.txt / GEO** | Update llms.txt with latest capabilities for AI citation. | Per feature change | None |

### Tier 2: Human Approval Gate Required

| Channel | AI's Role | Why Gate |
|---------|-----------|---------|
| **Hacker News** | Draft title, optimize README, prepare landing page | High ban risk. One shot matters. |
| **Reddit** | Draft subreddit-specific posts, identify relevant threads | Self-promo limits, mod hostility |
| **Discord/Slack** | Draft responses to questions | Inauthenticity detected in real-time chat |

### Tier 3: Not Automatable

Conference talks, podcast appearances, genuine relationship building, strategic pivots.

---

## 3. Content Strategy

### What Works (with evidence)

| Content Type | Why It Works | Automatable? |
|-------------|-------------|--------------|
| **Comparison pages** ("CruxDev vs X") | SEO long-tail, high intent | Yes — generate from COMPETITORS.md |
| **Benchmark data** | #1 factor for AI citation (GEO). Original data cannot be replicated. | Yes — run benchmarks, publish results |
| **Release changelogs** | Drives steady baseline traffic, shows project is alive | Yes — from git log |
| **Technical tutorials with code** | Drives SEO, demonstrates real utility | Yes — with quality gate |
| **Build-in-public updates** | Authenticity signal, community connection | Yes — from evolution cycle results |
| **"How we built X"** posts | Shows depth, attracts contributors | Yes — from build plan + convergence data |

### What Fails (avoid)

| Content Type | Why It Fails |
|-------------|-------------|
| Generic "5 tips" listicles | Detected as AI spam. Zero unique value. |
| Marketing-language posts | Developers hostile to corporate tone |
| Cross-posted identical content | Trivially detected, ban risk |
| Engagement bait ("like if you agree") | Community backlash |
| Thought leadership without code | "Show me the code" is the dev norm |

### Content Generation Pipeline

Every piece of content derives from real work, not marketing invention:

```
Build plan converged
    → Changelog entry (what changed)
    → X/Twitter thread (why it matters)
    → Blog post (how it works, with code)
    → vs/ page update (competitive impact)
    → README update (if feature-facing)
```

Content is a byproduct of building, not a separate marketing effort.

---

## 4. GitHub Trending Optimization

GitHub Trending is based on **star velocity** — stars per unit time relative to baseline. A burst of 50-100 stars can land a small project on the daily page.

**What the AI can optimize:**
- **README quality:** Demo GIF, clear quick-start, badges, compelling description
- **Release timing:** Monday/Tuesday US mornings maximize the 24-hour window
- **Cross-channel coordination:** Post to X + blog simultaneously to concentrate star velocity
- **Language-specific trending:** Rust trending has lower thresholds than general
- **Repository signals:** Active issues, recent commits, CI passing, good-first-issue labels

**What the AI cannot do:**
- Manufacture stars (GitHub detects and removes)
- Guarantee trending placement (depends on competition that day)
- Control Hacker News (needs human gate)

---

## 5. Repository Excellence (The Foundation)

Every external channel drives people to the repo. If the repo doesn't convert, nothing else matters.

**Autonomous repo optimization checklist:**
- [ ] README: Hero section → what it does → quick-start (30 seconds) → demo GIF → features → comparison table → installation
- [ ] Badges: tests passing, coverage, license, latest release, Rust version
- [ ] Demo: Terminal recording (asciinema/VHS) showing convergence in action
- [ ] Docs: Clear, discoverable, up-to-date (auto-generated from code)
- [ ] Issues: Labeled, triaged, first-time contributor welcome messages
- [ ] CI: Green badge, fast builds, visible test count
- [ ] Releases: Semantic versioning, changelogs, binary artifacts
- [ ] llms.txt: AI-discoverable project description
- [ ] CONTRIBUTING.md: How to contribute, good-first-issue process
- [ ] GitHub Discussions: Enabled, categorized, responded to

All of these are maintainable by the AI autonomously.

---

## 6. Typefully Integration

Primary posting channel. Bryan has an active Typefully account.

**API:** `POST /api/v1/drafts/` with `content`, `schedule_date`, `threadify`
**Rate:** 1-3 posts/day, varied timing (not all at the same time)
**Content types:**
- Release announcements (from changelog)
- Benchmark results (from test suite)
- Technical tips (from docs/patterns)
- Build-in-public updates (from evolution cycle)
- Competitive differentiation (from COMPETITORS.md updates)

**Thread format for releases:**
```
1/ CruxDev [version]: [headline feature]

[What it does in one sentence]

2/ The problem: [what was wrong before]

3/ How it works: [technical explanation with code snippet]

4/ Numbers: [test count, coverage, performance data]

5/ Try it: [link to repo]
```

---

## 7. SEO + GEO Strategy

### Traditional SEO (autonomous)
- Comparison pages: `/vs/superpowers`, `/vs/deepsource`, `/vs/yoyo-evolve`
- Tutorial pages: "How to set up convergence-driven TDD", "Multi-dimensional code audit with CruxDev"
- Schema markup: FAQPage, SoftwareApplication on all pages
- Meta descriptions optimized for search intent

### GEO (Generative Engine Optimization) — the new frontier
Google-to-AI overlap dropped from 70% to below 20%. AI citation is a distinct discipline.

**What drives AI citation (ranked):**
1. **Original benchmarks/data** — unique, verifiable, cannot be hallucinated
2. **Structured technical content** — clear headings, code blocks, specific claims
3. **llms.txt** — explicit AI-readable project description
4. **Schema markup** — structured data for machine understanding
5. **Citation-worthy claims** — specific numbers, not vague superlatives

**Autonomous GEO actions:**
- Maintain llms.txt with current capabilities, test count, tool count
- Include benchmark data in all technical pages
- Use schema markup on all pages
- Structure content with clear H2/H3 hierarchy

---

## 8. Issue-to-Growth Pipeline

GitHub issues are a growth signal AND a growth channel:

```
New issue filed
    → AI triages (priority, type)
    → AI responds (welcome, acknowledge, label)
    → If feature request → competitive impact check
    → If bug → fix via convergence engine
    → Resolution → changelog → X post → SEO update
    → Issue author becomes potential advocate
```

**Direct users to issues:** Every project website should include a prominent "Report an Issue" or "Request a Feature" link pointing to `github.com/{owner}/{repo}/issues/new`.

---

## 9. Metrics

### Signal (track these)
| Metric | Source | Meaning |
|--------|--------|---------|
| Star velocity | GitHub API | Growth rate (not absolute count) |
| Issue response time | GitHub API | Community health signal |
| Contributor count | GitHub API | Ecosystem breadth |
| X/Twitter impressions | Typefully analytics | Content reach |
| Blog unique visitors | Analytics | SEO effectiveness |
| AI citation count (SoM) | Manual or API search | GEO effectiveness |

### Vanity (don't optimize for these)
- Absolute star count (lagging indicator)
- Twitter follower count (doesn't correlate with adoption)
- Blog post count (volume doesn't matter, quality does)

---

## 10. Realistic Timeline

| Period | Target | Key Actions |
|--------|--------|-------------|
| **Month 1** | 10-50 stars | README excellence, first GitHub Release, daily X posts via Typefully, vs/ pages live, llms.txt |
| **Month 2-3** | 50-200 stars | Blog content pipeline, issue triage automation, Rust trending attempt, contributor onboarding |
| **Month 4-6** | 200-500 stars | SEO traffic growing, GEO citations appearing, consistent X presence, community forming |
| **Month 7-12** | 500-2,000 stars | Compound growth, HN submission (human gate), possible viral moment |
| **Year 2** | 2,000-10,000 stars | Ecosystem integrations, marketplace presence, conference mentions |

**The breakout moment** (10K+) almost always requires a single high-impact social event (HN front page, influential endorsement) that AI cannot reliably manufacture. The autonomous engine's job is to make the project ready for that moment and maximize its impact when it comes.

---

## 11. Anti-Patterns

| Don't | Why | Evidence |
|-------|-----|---------|
| Buy or trade stars | GitHub detects, removes, flags account | GitHub 2023 star-buying crackdown |
| Post to HN automatically | Sophisticated detection, account death | 12+ years of anti-gaming |
| Cross-post identical content to Reddit | Trivially detected, ban + reputation damage | Moderator policies |
| Generate thin SEO content | Google March 2024 update penalizes scaled content abuse | Multiple deindexing reports |
| Automate engagement (likes, follows, replies) | Platform ToS violations, community backlash | X, Reddit, HN all prohibit |
| Post more than 3x/day to any channel | Diminishing returns, spam perception | Practitioner consensus |
| Respond to criticism with AI | Tone-deaf responses catastrophic for reputation | Multiple case studies |

---

## 12. Implementation: What CruxDev Already Has

| Capability | Status | Used By |
|-----------|--------|---------|
| Evolution 5-beat loop | Built (BP015) | Autonomous cycle engine |
| GitHub issue monitoring | Built (BP015) | Issue triage + response |
| Git automation | Built (BP016) | Commit/push/PR after changes |
| Competitors pipeline | Built (BP015, BP018) | vs/ pages, gap analysis |
| Changelog generation | Built (BP015) | POST beat |
| X post generation | Built (BP015) | POST beat |
| Session bus | Built (BP014) | Cross-project coordination |

**What needs building:**
- Typefully API integration (post scheduling)
- GitHub Release creation via API
- README auto-optimization
- Blog page generation + publish pipeline
- Metrics tracking + adjustment loop
- llms.txt auto-update
- Demo recording automation (asciinema/VHS)

---

## 13. The Virtuous Cycle

```
Better product (convergence engine ships improvements)
    → Better content (changelogs, benchmarks, tutorials from real work)
    → More discovery (SEO, X posts, AI citations)
    → More users (GitHub stars, issues, contributors)
    → More feedback (issues, feature requests)
    → Better product (convergence engine addresses feedback)
    → ...repeat
```

The key insight: **CruxDev is its own best case study.** Every improvement to the product IS content. Every build plan converged IS a story. The growth engine doesn't generate marketing — it narrates the building.
