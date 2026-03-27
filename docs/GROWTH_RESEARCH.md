# Autonomous AI-Driven Growth Research for Open Source Developer Tools

**Created:** 2026-03-26
**Research Method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Constraint:** Every growth action must be executable by an AI agent with NO human intervention.

---

## Table of Contents

1. [Growth Channels Fully Automatable by AI](#1-growth-channels-fully-automatable-by-ai)
2. [Superpowers 0-to-110K Breakdown](#2-superpowers-0-to-110k-breakdown)
3. [Content Formats That Drive Developer Adoption](#3-content-formats-that-drive-developer-adoption)
4. [GitHub Trending: How It Works and How to Optimize](#4-github-trending)
5. [Claude Code Marketplace and Plugin Distribution](#5-claude-code-marketplace)
6. [Autonomous Community Engagement: Risks and Rules](#6-autonomous-community-engagement)
7. [SEO and GEO Strategies for AI Execution](#7-seo-and-geo-strategies)
8. [Realistic Autonomous Growth Rates](#8-realistic-growth-rates)
9. [Channel-by-Channel Execution Matrix](#9-execution-matrix)
10. [Sources](#10-sources)

---

## 1. Growth Channels Fully Automatable by AI

### Tier 1: Fully Automatable (AI agent can execute end-to-end)

| Channel | API/Tool | Measurable Impact | Notes |
|---------|----------|-------------------|-------|
| **X/Twitter posting** | Typefully API, Outstand MCP, Late API, X API Free Tier | Awareness, follower growth, link clicks | Free tier: ~500 posts/month. Can automate content, NOT engagement (likes/retweets/replies). Bot accounts must identify as bots. |
| **Bluesky posting** | AT Protocol API (no key required) | Growing developer audience, early mover advantage | Most bot-friendly platform. No API key needed. Official bot labeling system. Active developer community. |
| **LinkedIn company page posting** | LinkedIn Posts API, Late API, Outstand MCP | B2B visibility, enterprise developer reach | Requires legally registered entity. Community Management API access needed. Immediate publishing only (no native scheduling). |
| **Blog syndication (Dev.to + Hashnode)** | Dev.to API (REST), Hashnode API (GraphQL) | SEO backlinks, developer discovery, content indexing | Can auto-publish from markdown source. Canonical URL support on both. Dev.to: API key from settings. Hashnode: create draft then publish. |
| **GitHub releases + changelogs** | `gh` CLI, GitHub Actions, Release Please | Developer trust, update visibility, SEO | Fully automatable via conventional commits. Changelog CI generates from PR titles. |
| **README and documentation updates** | `gh` CLI, git | Conversion of visitors to users | AI can maintain comparison tables, update stats, generate getting-started guides. |
| **Awesome-list PRs** | `gh` CLI | Discovery by category browsers | AI can identify relevant lists and submit PRs. Acceptance requires meeting list-specific CONTRIBUTING.md criteria. |
| **Claude Code plugin marketplace** | `/plugin` system, `gh` CLI for PRs | Direct distribution to Claude Code users | Submit to `anthropics/claude-plugins-official` via PR. 55+ plugins in directory. Validation via GitHub Actions. |
| **RSS-to-social automation** | GitHub Actions + social APIs | Consistent cross-platform presence | Trigger social posts on new releases, blog posts, or changelog updates. |
| **SEO content generation** | AI + static site generators | Organic search traffic | Comparison pages, "how-to" guides, "X vs Y" pages. AI can generate and publish. |
| **GEO (Generative Engine Optimization)** | llms.txt, Schema.org, structured content | AI search citation (ChatGPT, Perplexity, etc.) | Original research and benchmarks attract AI citations. "Share of Model" is the key metric. |
| **GitHub issue triage and response** | GitHub API, `gh` CLI | Community health, contributor retention | Auto-label, auto-respond to common issues, link to docs. |

### Tier 2: Partially Automatable (AI generates, human reviews/approves)

| Channel | Bottleneck | AI Role |
|---------|-----------|---------|
| **Hacker News (Show HN)** | Read-only API; no programmatic submission. Anti-gaming detection. | AI generates the post content, human submits. AI can monitor and draft comment replies. |
| **Reddit** | New human verification system (March 2026). [App] label required for bots. Suspicious activity triggers CAPTCHA. | AI generates content, manages scheduling. Human handles verification and initial account setup. |
| **Product Hunt launch** | Requires human "maker" presence and community engagement. | AI generates all materials (tagline, description, screenshots, first comment). Human clicks launch. |
| **Conference talk submissions** | CFPs require human identity and presentation commitment. | AI writes abstract, outline, and slides. Human submits and presents. |
| **Podcast appearances** | Requires human voice and scheduling. | AI can identify targets, draft pitches, prepare talking points. |

### Tier 3: Not Automatable

| Channel | Why |
|---------|-----|
| **In-person meetups** | Physical presence required. |
| **Paid advertising** | Requires billing setup and budget approval (could be partially automated once set up). |
| **Partnership agreements** | Legal/human relationship required. |

---

## 2. Superpowers 0-to-110K Breakdown

### Timeline
- **October 2025:** First published, same day Anthropic launched Claude Code plugin system.
- **October 2025 - March 2026 (~5 months):** 0 to 110,000+ stars.
- **March 2026 peak:** ~2,000 stars/day.

### Growth Drivers (ordered by impact)

1. **First-mover in a new platform category.** Superpowers launched the same day Anthropic opened the plugin system. Being first in a new marketplace is an unrepeatable advantage but the pattern (be first in new distribution channels) is replicable.

2. **Solved a real, widely-felt pain point.** The shared sentiment: "82,000 developers just agreed: the biggest problem with AI coding is not intelligence, it is discipline." Superpowers solved the "agent thrashing" problem that every Claude Code user experienced.

3. **Methodology-first, not tooling-first.** The spec-first, TDD, composable skills approach resonated with senior developers who experienced the chaos of unconstrained agents.

4. **Viral LinkedIn moment.** A widely-shared LinkedIn post about the star count created social proof and FOMO. Social media amplification was a significant accelerant.

5. **Anthropic marketplace acceptance.** Being listed in the official marketplace provided a distribution channel with built-in trust.

6. **Low friction install.** One command to install. No configuration ceremony.

7. **Active iteration.** v5.0.5 as of March 2026 - frequent releases showed momentum.

### What's Replicable by AI

| Factor | Replicable? | How |
|--------|-------------|-----|
| First-mover in new channel | Partial | AI monitors for new marketplaces/registries and auto-submits |
| Solving real pain point | N/A | Product quality, not growth strategy |
| Social media amplification | Yes | AI generates and schedules posts about milestones |
| Marketplace listing | Yes | AI submits plugin PRs |
| Low-friction install | N/A | Product design |
| Frequent releases | Yes | AI can automate release cadence via CI/CD |
| Documentation quality | Yes | AI can generate and maintain docs |
| Community engagement | Partial | AI can triage issues, respond to questions, but authentic engagement is harder |

### Key Insight
Superpowers' growth was primarily driven by product-market fit and platform timing, amplified by social proof. The viral growth came AFTER the product was already good. No amount of growth automation substitutes for solving a real problem.

---

## 3. Content Formats That Drive Developer Adoption

### Formats AI Can Generate Autonomously

| Format | Impact | Automation Method | Evidence |
|--------|--------|-------------------|----------|
| **Comparison pages ("X vs Y")** | High - captures decision-stage search traffic | AI generates, publishes to static site | 65% of companies report AI content improved SEO (2026 data) |
| **Changelog entries** | Medium - shows momentum, re-engages existing users | Auto-generated from conventional commits | Release Please, Changelog CI |
| **"How I Built X" technical articles** | High - Dev.to personal blogs get 2.4x more HN frontpage than corporate blogs | AI generates from codebase analysis, publishes to Dev.to/Hashnode | Personal blogs: 26% of HN front page vs 11% corporate |
| **Benchmark/comparison data** | Very High - original research attracts AI citations (GEO) | AI runs benchmarks, formats results, publishes | GEO: original data is the #1 factor for AI citation |
| **Getting-started tutorials** | High - reduces activation friction | AI generates from README, publishes cross-platform | Standard developer onboarding pattern |
| **Architecture decision records** | Medium - builds trust with senior devs | AI documents design decisions from codebase | Builds E-E-A-T for SEO |
| **Migration guides ("from X to Y")** | High - captures competitor users at switching moment | AI generates comparative analysis | High-intent search traffic |
| **Thread/post series** | Medium - builds following over time | AI schedules via social APIs | Consistent presence metric |

### Content That Does NOT Work When AI-Generated

| Format | Why | Source |
|--------|-----|--------|
| **Thought leadership / opinion pieces** | Readers detect lack of genuine experience. Google HCU penalizes content without first-hand experience. | Google E-E-A-T guidelines; 46% developer trust deficit |
| **Community replies that feel canned** | Detected and downvoted. HN, Reddit users particularly hostile to inauthentic engagement. | Reddit human verification (March 2026); HN anti-gaming |
| **Sales-oriented landing pages** | Developer community rejects marketing language. Personal blogs get 2.4x HN advantage over corporate. | HN analysis data |

### Key Metrics (2026 Data)
- AI-generated content produces 36% higher conversion on landing pages.
- AI copywriting improves ad CTRs by 38%, reduces CPC by 32%.
- 97% of content marketers plan to use AI in 2026 (up from 90% in 2025).
- BUT only 19% track AI-specific KPIs - measurement is the gap.

---

## 4. GitHub Trending

### How the Algorithm Works

GitHub has not published the exact algorithm (intentionally, to prevent gaming). Based on community analysis and research:

1. **Star velocity (primary signal).** Rate of star growth, not absolute count. A spike matters more than steady growth.
2. **Deviation from baseline.** Current star gain compared to historical average. 19 stars on a project that normally gets 0 is more notable than 19 on one that normally gets 15.
3. **Time window.** Trending is calculated for today/this week/this month. Recency bias is strong.
4. **Repository activity.** Commits, tags, merged PRs, created PRs, author comments all factor in.
5. **Anti-gaming.** GitHub has detection for artificial star inflation. Details undisclosed.

### What AI Can Optimize

| Factor | AI Action | Feasibility |
|--------|-----------|-------------|
| Star velocity spikes | Coordinate content publication across channels to concentrate attention | Yes - schedule blog posts, social posts, and release announcements for the same day |
| Repository activity | Maintain consistent commit cadence, PR flow, issue responses | Yes - evolution pipeline, automated releases |
| README quality | AI-optimized README with clear value prop, demo GIF, one-line install | Yes |
| First-hour momentum | Time releases to coincide with peak developer hours (Tuesday-Thursday, 8-10AM PT) | Yes - scheduled via CI/CD |

### What AI Cannot Do
- **Buy or fake stars.** GitHub detects this and it's against ToS.
- **Game the trending algorithm directly.** The algorithm is unpublished and actively defended.
- **Guarantee trending placement.** Even perfect execution depends on competing repos that week.

### Realistic Path to Trending
1. Build genuinely useful tool (prerequisite).
2. Accumulate early users who organically star.
3. Time a significant release with coordinated cross-channel promotion.
4. If the star spike is large enough relative to baseline, trending follows.
5. Trending creates a flywheel (trending -> more stars -> more trending).

Research finding: **78.5% of developers linked popularity peaks to posts on social media, mostly Hacker News.** This is the strongest external driver of trending.

---

## 5. Claude Code Marketplace

### Current State (March 2026)
- **9,000+ plugins** available across marketplaces.
- **Official directory:** `anthropics/claude-plugins-official` on GitHub (55+ plugins, curated by Anthropic).
- **Third-party marketplaces:** buildwithclaude.com, claudemarketplaces.com, mcpmarket.com, skills.pawgrammer.com.

### Submission Process (Fully Automatable)

1. **Structure plugin** per spec: `.claude-plugin/plugin.json` (required), optional `.mcp.json`, commands/, agents/, skills/, README.md.
2. **Submit PR** to `anthropics/claude-plugins-official`.
3. **Pass automated validation:** GitHub Actions runs JSON validation and sorting checks on every PR.
4. **Anthropic review:** External plugins must meet quality and security standards.
5. **In-app submission:** Alternative path via in-app submission form.

### AI Agent Can:
- [Y] Structure the plugin directory and metadata files.
- [Y] Submit the PR via `gh` CLI.
- [Y] Maintain and update the plugin listing (new PRs for updates).
- [Y] Submit to third-party marketplaces (buildwithclaude.com, etc.).
- [Y] Create and host a custom marketplace (`marketplace.json` on GitHub).
- [Partial] Pass Anthropic quality review (depends on plugin quality, not submission automation).

### Custom Marketplace Distribution
Any GitHub repo can host a `marketplace.json` that lists plugins. Users add with `/plugin marketplace add user-or-org/repo-name`. This is a fully autonomous distribution channel.

---

## 6. Autonomous Community Engagement

### Platform-by-Platform Rules and Risks

#### X/Twitter
- **Allowed:** Schedule own tweets, auto-post from blog/RSS, AI-generated content.
- **Prohibited:** Automated likes, follows, retweets, replies, DMs. Follow/unfollow automation.
- **Bot rules:** Must identify as bot in bio.
- **Free tier:** ~500 posts/month, heavily restricted.
- **Basic tier:** $200/month for production use.
- **Risk level:** LOW for content posting. HIGH for engagement automation.
- **API:** X API v2, Typefully API, Outstand MCP, Late API.

#### Bluesky
- **Allowed:** Bot accounts with self-label. Full API access for posting. No API key required.
- **Bot rules:** Add self-label to profile.
- **Risk level:** LOWEST of all platforms. Most bot-friendly.
- **API:** AT Protocol. SDKs in TypeScript, Python, Go, Dart.
- **Advantage:** Growing developer community, early-mover opportunity.

#### LinkedIn
- **Allowed:** Company page posting via API. Content automation.
- **Requirements:** Legally registered entity for Community Management API.
- **Limitation:** Immediate publishing only (no native scheduling via API).
- **Risk level:** LOW for company page posting.
- **API:** LinkedIn Posts API, or via Outstand/Late unified APIs.

#### Reddit
- **Status (March 2026):** NEW human verification system being deployed.
- **Rules:** Bots must carry [App] label. Must register via Developer Platform. Suspicious activity triggers verification (posting speed, interaction patterns, automated behavior markers).
- **Free tier:** 100 QPM for non-commercial use.
- **Risk level:** HIGH and INCREASING. Reddit is actively cracking down on bot activity.
- **Recommendation:** Do NOT automate Reddit engagement. Use it for human-submitted content only.

#### Hacker News
- **API:** Read-only. No programmatic submission.
- **Anti-gaming:** 12+ years of vote manipulation detection. Ring detection is a stated priority.
- **Risk level:** VERY HIGH for any automation. Accounts are banned for manipulation.
- **Recommendation:** AI generates content. Human submits. AI can draft comment replies but human should post them.
- **Success factors:** Personal authenticity, technical depth, try-able product, respond within 2 hours.

#### Dev.to / Hashnode
- **Allowed:** Full API publishing. Automated cross-posting.
- **Risk level:** LOWEST. These platforms explicitly support API publishing.
- **Best practice:** Set canonical URL to your own site.

### Detection Research (2026)
A USC study (March 2026) found that AI agents can autonomously coordinate campaigns without human direction, and that **machine-learning tools trained to detect bots were unable to discriminate between AI agents and human accounts.** However, platforms can detect coordinated behavior by analyzing how accounts push similar narratives from accounts with no obvious connection.

**Key finding:** Individual AI-generated content is nearly undetectable. Coordinated campaigns ARE detectable. An autonomous agent posting its own content to its own accounts is low risk. An agent coordinating across multiple accounts to amplify is high risk.

---

## 7. SEO and GEO Strategies

### Traditional SEO (Automatable by AI)

| Strategy | AI Execution | Tool | Impact |
|----------|-------------|------|--------|
| **Comparison pages** | Generate "CruxDev vs X" pages for each competitor | Static site generator + AI | Captures high-intent search traffic |
| **Technical how-to content** | Generate tutorials from codebase analysis | Dev.to API, Hashnode API | Long-tail search traffic |
| **Schema.org markup** | Generate structured data for software application | Template + AI | Rich snippets in search results |
| **Internal linking** | AI maintains cross-references between docs | Build script | Improved crawlability |
| **Meta descriptions and titles** | AI generates optimized metadata | Static site build step | CTR improvement |
| **Backlink acquisition via content** | Publish original research that others cite | AI generates benchmarks and data | Domain authority |

### Generative Engine Optimization (GEO) - NEW in 2026

GEO is the practice of positioning content so that AI platforms (ChatGPT, Perplexity, Google AI Overviews) cite or recommend you.

**Why GEO matters now:**
- ChatGPT: 800M+ weekly users.
- Google Gemini: 750M+ monthly users.
- AI Overviews in 16%+ of all Google searches.
- Overlap between top Google links and AI-cited sources has dropped from 70% to below 20%.

**GEO strategies AI can execute:**

| Strategy | How | Why It Works |
|----------|-----|-------------|
| **Publish original benchmarks** | AI runs tests, publishes data | AI engines cite original research over duplicative content |
| **Structured content** | Use headings, bullet points, "in summary" sections | LLMs parse and reproduce structured content more effectively |
| **llms.txt and llms-full.txt** | Maintain machine-readable project summaries | Direct signal to AI crawlers about what your project does |
| **Schema.org SoftwareApplication** | Structured metadata in HTML | AI systems use schema data for entity understanding |
| **Comparison tables** | Feature matrices with competitors | AI engines surface comparison data when users ask "X vs Y" |
| **Citation-worthy claims** | Include specific numbers, benchmarks, unique frameworks | AI citation requires differentiated content - proprietary data is cited over generic |

**Key GEO metric:** Share of Model (SoM) - how often your brand appears in AI-generated responses vs competitors.

---

## 8. Realistic Growth Rates

### Baseline Expectations (from case studies)

| Phase | Timeline | Stars | What Drives It |
|-------|----------|-------|----------------|
| **Cold start** | Month 1-2 | 0-50 | Direct outreach, first blog posts |
| **Early traction** | Month 3-4 | 50-200 | Consistent content, initial community |
| **Inflection point** | Month 5-6 | 200-1,000 | HN/Reddit front page hit, or marketplace inclusion |
| **Growth phase** | Month 7-10 | 1,000-10,000 | Compound effects, trending, word of mouth |
| **Breakout** | Month 11-18 | 10,000-33,000+ | Category leadership, conference talks, media |

**Reference case:** AFFiNE went 0 to 33,000 stars in 18 months. Key pattern: growth was NOT linear. Slow for 6 months, then 10,000 in 4 months once the right audience was found.

### What Autonomous AI Execution Can Realistically Achieve

**Conservative estimate** (AI automation alone, no viral moments):
- Month 1-3: 10-100 stars (content seeding, marketplace listing, social presence)
- Month 4-6: 100-500 stars (consistent content, SEO beginning to index, awesome-list inclusions)
- Month 7-12: 500-2,000 stars (compound effects, organic discovery, GEO citations)

**Optimistic estimate** (AI automation + one viral moment):
- If a Show HN or social post hits, growth can jump 300+ stars in 24 hours (NebulaGraph case study).
- A viral moment can compress the 18-month timeline to 6-9 months.

**Key constraint:** Autonomous AI execution can do everything EXCEPT create viral moments. Viral growth requires either: (a) exceptional product-market fit that spreads organically, or (b) a human champion who posts with authentic voice to HN/Reddit/X.

### Growth Compound Effects

Research shows **78.5% of popularity peaks trace to social media posts, mostly Hacker News.** This means:
1. AI prepares the ammunition (content, docs, comparison pages, benchmarks).
2. AI maintains the cadence (social posts, releases, changelogs).
3. The breakout moment usually comes from a single high-impact social post.
4. After breakout, AI automation sustains and compounds the growth.

---

## 9. Channel-by-Channel Execution Matrix

| Channel | AI Autonomous? | API/Tool | Frequency | Expected Impact | Risk |
|---------|---------------|----------|-----------|----------------|------|
| **X/Twitter** | Y | Typefully API / Outstand MCP | 3-5x/week | Awareness, link traffic | Low (content only) |
| **Bluesky** | Y | AT Protocol API | 3-5x/week | Growing dev audience | Very Low |
| **LinkedIn** | Y | LinkedIn Posts API / Late | 2-3x/week | B2B, enterprise devs | Low |
| **Dev.to articles** | Y | Dev.to REST API | 2-4x/month | SEO, developer discovery | Very Low |
| **Hashnode articles** | Y | Hashnode GraphQL API | 2-4x/month | SEO, developer discovery | Very Low |
| **GitHub releases** | Y | `gh` CLI + Release Please | Per release | Developer trust | None |
| **Changelog** | Y | Changelog CI / Actions | Per release | Update visibility | None |
| **Claude plugin marketplace** | Y | `gh` CLI (PR) | On update | Direct distribution | None |
| **Custom marketplace** | Y | `marketplace.json` on GH | On update | Discovery channel | None |
| **Awesome-list PRs** | Y | `gh` CLI | One-time per list | Category discovery | Low |
| **SEO comparison pages** | Y | Static site + AI content | Monthly updates | Search traffic | Very Low |
| **GEO optimization** | Y | llms.txt, Schema.org | Continuous | AI search visibility | None |
| **GitHub issue triage** | Y | GitHub API / `gh` CLI | Continuous | Community health | Low |
| **README maintenance** | Y | git | Continuous | Visitor conversion | None |
| **Hacker News** | N (human submit) | N/A | Per major release | Highest single-event impact | High if automated |
| **Reddit** | N (human submit) | N/A | Per major release | Community discovery | High if automated |
| **Product Hunt** | N (human launch) | N/A | Once | Launch spike | N/A |

---

## 10. Sources

### Pass 1: Broad Landscape
- [Promote Your Open Source Project: A Step-by-Step Launch Guide](https://business.daily.dev/resources/promote-open-source-project-step-by-step-launch-guide/)
- [GitHub Stars Guide: Evaluating Open Source in 2026](https://blog.tooljet.com/github-stars-guide/)
- [What the fastest-growing tools reveal about how software is being built](https://github.blog/news-insights/octoverse/what-the-fastest-growing-tools-reveal-about-how-software-is-being-built/)
- [DevTools Landscape 2025](https://insights.tryspecter.com/devtools-landscape-2025/)
- [12 Fastest Growing Open Source Dev Tools](https://www.landbase.com/blog/fastest-growing-open-source-dev-tools)
- [obra/superpowers on GitHub](https://github.com/obra/superpowers)
- [Superpowers explained: the popular Claude plugin](https://blog.devgenius.io/superpowers-explained-the-claude-plugin-that-enforces-tdd-subagents-and-planning-c7fe698c3b82)
- [GitHub Trending Weekly 2026-03-25](https://www.shareuhack.com/en/posts/github-trending-weekly-2026-03-25)

### Pass 1: Social Media APIs
- [Outstand - Unified Social Media API](https://www.outstand.so)
- [Late - Unified Social Media API](https://getlate.dev/)
- [Ayrshare Social Media API](https://www.ayrshare.com/)
- [Post Bridge - Social Media API for AI Agents](https://www.post-bridge.com/agents)
- [Typefully API Documentation](https://support.typefully.com/en/articles/8718287-typefully-api)
- [X API Pricing 2026](https://zernio.com/blog/twitter-api-pricing)
- [Twitter/X Automation Rules in 2026](https://opentweet.io/blog/twitter-automation-rules-2026)
- [Bluesky Bots Documentation](https://docs.bsky.app/docs/starter-templates/bots)
- [LinkedIn Posts API](https://learn.microsoft.com/en-us/linkedin/marketing/community-management/shares/posts-api)

### Pass 1: GitHub Trending
- [GitHub Trending Repo Calculations Discussion](https://github.com/orgs/community/discussions/163970)
- [How the GitHub Trending Algorithm Works](https://dev.to/yvonnickfrin/i-wonder-how-github-s-trending-algorithm-works-any-clue-3ebf)
- [How to repeatedly make it to GitHub Trending](https://medium.com/@manoj.radhakrishnan/how-to-trend-on-github-dcdda9055f8)
- [How we got into GitHub Trending](https://medium.com/@DiggerHQ/how-we-got-into-github-trending-c281f3b06df9)

### Pass 2: Academic/Authoritative
- [Characterization and Prediction of Popular Projects on GitHub](https://xin-xia.github.io/publication/compsac19.pdf) - Predicts popularity using 35 features from GitHub and Stack Overflow
- [Understanding Factors that Impact GitHub Popularity](https://arxiv.org/pdf/1606.04984) - Programming language and domain as primary factors
- [Predicting the Popularity of GitHub Repositories](https://arxiv.org/abs/1607.04342) - Random Forest achieves R-squared 0.88
- [What's in a GitHub Star?](https://www.researchgate.net/publication/327566664) - 78.5% of popularity peaks linked to social media
- [USC Study: AI Agents Coordinate Propaganda Campaigns Autonomously](https://viterbischool.usc.edu/news/2026/03/usc-study-finds-ai-agents-can-autonomously-coordinate-propaganda-campaigns-without-human-direction/)
- [Swarms of AI bots can sway beliefs](https://theconversation.com/swarms-of-ai-bots-can-sway-peoples-beliefs-threatening-democracy-274778) - Published in Science journal
- [AI bot swarms threaten democracy](https://www.science.org/doi/10.1126/science.adz1697) - Science, 2026

### Pass 2: Content Marketing Data
- [50+ Content Marketing Statistics 2026](https://www.typeface.ai/blog/content-marketing-statistics) - 97% use AI; 65% report improved SEO
- [AI Marketing Statistics 2026](https://www.allaboutai.com/resources/ai-statistics/marketing/) - 36% higher conversion, 38% CTR improvement
- [Content Marketing ROI 2026](https://www.digitalapplied.com/blog/content-marketing-roi-2026-19-percent-track-ai-kpis) - Only 19% track AI-specific KPIs
- [51 AI Writing Statistics 2026](https://www.siegemedia.com/strategy/ai-writing-statistics)

### Pass 2: Hacker News Analysis
- [Analyzing 10,000 Show HN Submissions](https://antontarasenko.github.io/show-hn/)
- [HN Front Page Guide](https://awesome-directories.com/blog/hacker-news-front-page-guide/)
- [How to Get on the Front Page of HN 2025](https://www.flowjam.com/blog/how-to-get-on-the-front-page-of-hacker-news-in-2025-the-complete-up-to-date-playbook)
- [NebulaGraph: 300+ Stars in 24 Hours from HN](https://www.nebula-graph.io/posts/nebula-graph-being-on-hacker-new-front-page)

### Pass 2: GEO and AI Search
- [GEO: How to Win AI Mentions (Search Engine Land)](https://searchengineland.com/what-is-generative-engine-optimization-geo-444418)
- [Mastering GEO in 2026 (Search Engine Land)](https://searchengineland.com/mastering-generative-engine-optimization-in-2026-full-guide-469142)
- [GEO Over SEO (Andreessen Horowitz)](https://a16z.com/geo-over-seo/)
- [Frase: Best AI SEO Agents 2026](https://www.frase.io/blog/best-ai-seo-agents-2026)

### Pass 2: Claude Code Marketplace
- [Claude Code Plugin Marketplace Docs](https://code.claude.com/docs/en/plugin-marketplaces)
- [anthropics/claude-plugins-official](https://github.com/anthropics/claude-plugins-official)
- [Claude Code Plugins Guide (MorphLLM)](https://www.morphllm.com/claude-code-plugins)
- [Discover Plugins (Claude Code Docs)](https://code.claude.com/docs/en/discover-plugins)

### Pass 2: Reddit (March 2026)
- [Reddit Takes on Bots with Human Verification (TechCrunch, March 2026)](https://techcrunch.com/2026/03/25/reddit-bots-new-human-verification-requirements/)
- [Reddit Responsible Builder Policy](https://support.reddithelp.com/hc/en-us/articles/42728983564564-Responsible-Builder-Policy)
- [Reddit API Limits and Rules (Postiz)](https://postiz.com/blog/reddit-api-limits-rules-and-posting-restrictions-explained)

### Growth Case Studies
- [GitHub Star Growth: 10K Stars in 18 Months](https://dev.to/iris1031/github-star-growth-10k-stars-in-18-months-real-data-4d04)
- [How to Get More GitHub Stars: 33K Stars Case Study](https://dev.to/iris1031/how-to-get-more-github-stars-the-definitive-guide-33k-stars-case-study-2kjo)
- [10 Proven Ways to Boost GitHub Stars in 2026](https://scrapegraphai.com/blog/gh-stars)
- [ROSS Index: Top Trending Open Source Startups](https://runacap.com/ross-index/)

### Blog Syndication
- [Building an Automated Content Pipeline That Posts to 6 Platforms](https://yonatangross.hashnode.dev/building-an-automated-content-pipeline-that-posts-to-6-platforms)
- [Building an Automated Multi-Platform Blog Pipeline with GitHub Actions and AI](https://earezki.com/ai-news/2026-03-18-github-actions-/)
- [Blog Syndication: Cross-Publishing to Dev.to, Hashnode, and Medium](https://dev.to/navinvarma/blog-syndication-cross-publishing-blog-posts-to-devto-hashnode-and-medium-1a5d)
