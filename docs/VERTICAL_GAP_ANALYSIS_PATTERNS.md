# Vertical Gap Analysis Patterns

**Version:** 1.0
**Created:** 2026-03-28
**Scope:** Universal — applies to ANY creator/business vertical. Authors, podcasters, course creators, coaches, newsletter writers, musicians, YouTubers, entrepreneurs, agencies, and any future vertical.
**Companion:** `COMPETITORS_PATTERN.md` (for competitive intelligence), `RESEARCH_PATTERNS.md` (for deep research), `WEBSITE_PLANNING.md` (for building the solutions).

This document defines the methodology for analyzing any vertical's complete lifecycle, identifying every gap where creators lose money, time, or control, scoring those gaps as product opportunities, and producing actionable build plans for both CruxDev and CruxVibe. Every vertical analyzed with this pattern produces build plans for both products.

---

## Table of Contents

1. [Core Principle](#1-core-principle)
2. [Vertical Lifecycle Mapping](#2-vertical-lifecycle-mapping)
3. [Tool and Platform Inventory](#3-tool-and-platform-inventory)
4. [Gap Classification System](#4-gap-classification-system)
5. [Recipe Opportunity Scoring](#5-recipe-opportunity-scoring)
6. [CruxDev Capability Mapping](#6-cruxdev-capability-mapping)
7. [CruxVibe Recipe Mapping](#7-cruxvibe-recipe-mapping)
8. [Competitive Moat Analysis](#8-competitive-moat-analysis)
9. [The Cannibalization Playbook](#9-the-cannibalization-playbook)
10. [Cross-Vertical Synthesis](#10-cross-vertical-synthesis)
11. [Analysis Lifecycle and Convergence](#11-analysis-lifecycle-and-convergence)
12. [Output Artifacts](#12-output-artifacts)
13. [Appendix A: Vertical-Specific Lifecycle Templates](#appendix-a-vertical-specific-lifecycle-templates)
14. [Appendix B: Scoring Worked Example](#appendix-b-scoring-worked-example)
15. [Appendix C: Anti-Patterns](#appendix-c-anti-patterns)

---

## 1. Core Principle

**Creators should keep their money. Platforms should not exist between a creator and their audience.**

Every platform taking a percentage — Patreon (8-12%), Amazon KDP (30-65%), Substack (10%), Teachable ($400/mo), Kajabi ($500/mo), Gumroad (10%) — is a gap waiting to be filled by a recipe that the creator owns and we host. The analysis methodology exists to find these gaps systematically, score them, and convert them into build plans.

The value chain for any vertical looks like this:

```
Creator → [Platform Tax] → Audience
Creator → [CruxVibe Recipe] → Audience  (no tax, creator keeps revenue)
```

Every dollar of platform tax is a dollar of addressable market for CruxVibe. Every gap in creator tooling is a recipe waiting to be built. This pattern finds every gap in every vertical, systematically.

### 1.1 The Two Products This Pattern Feeds

| Product | What it needs from this analysis |
|---------|----------------------------------|
| **CruxDev** | Audit dimensions, pattern docs, templates, MCP tools — everything needed to BUILD recipes to convergence |
| **CruxVibe** | Recipe definitions, stack decisions, pricing models, hosting requirements — everything needed to SELL and OPERATE recipes |

Every vertical analysis produces outputs for both. Neither product roadmap exists in isolation.

### 1.2 Analysis Scope

A vertical is defined as a category of creator or business operator who:
- Creates intellectual property or content
- Has a repeating lifecycle from ideation to revenue
- Currently depends on 3+ platforms/tools to operate
- Loses 10%+ of revenue to platform fees, or pays $200+/mo in tool subscriptions

If a vertical meets these criteria, it is worth analyzing. If it does not, the analysis stops at Section 2 with a "not viable" determination.

---

## 2. Vertical Lifecycle Mapping

**Purpose:** Map the complete lifecycle for any vertical from first idea to scaled operation. Every vertical follows the same 7-stage macro-lifecycle, but the specifics differ. This section defines how to discover those specifics.

### 2.1 The Universal Creator Lifecycle

Every creator vertical maps onto these 7 stages. No exceptions have been found across authors, podcasters, course creators, coaches, newsletter writers, musicians, YouTubers, entrepreneurs, and agencies.

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌───────────────┐
│  1. IDEATION │───▸│ 2. CREATION │───▸│3. PRODUCTION│───▸│4. DISTRIBUTION│
└─────────────┘    └─────────────┘    └─────────────┘    └───────────────┘
                                                                 │
┌─────────────┐    ┌─────────────┐    ┌───────────────┐          │
│  7. SCALING  │◂───│ 6. COMMUNITY│◂───│5. MONETIZATION│◂────────┘
└─────────────┘    └─────────────┘    └───────────────┘
```

The lifecycle is NOT linear in practice — creators loop back constantly. But analysis must map each stage independently before mapping the loops.

### 2.2 Stage Definitions

**Stage 1: Ideation** — Generating, validating, and selecting ideas to pursue.

| Dimension | What to map |
|-----------|------------|
| Inputs | Where do ideas come from? (audience requests, trends, personal expertise, market gaps) |
| Activities | Brainstorming, research, validation, outlining, prototyping |
| Tools used | Note-taking apps, research tools, trend analysis, audience surveys |
| Outputs | A validated idea ready for creation |
| Time spent | Hours/days per ideation cycle |
| Pain points | Where does the creator get stuck, waste time, or make bad decisions? |

**Stage 2: Creation** — Producing the raw content or product.

| Dimension | What to map |
|-----------|------------|
| Inputs | Validated idea, reference materials, existing content library |
| Activities | Writing, recording, designing, coding, composing |
| Tools used | Writing software, DAWs, video editors, design tools, IDEs |
| Outputs | Raw content ready for production |
| Time spent | Hours/days per creation cycle |
| Pain points | Writer's block, tool friction, version management, collaboration |

**Stage 3: Production** — Transforming raw content into deliverable formats.

| Dimension | What to map |
|-----------|------------|
| Inputs | Raw content from creation stage |
| Activities | Editing, formatting, encoding, packaging, quality assurance |
| Tools used | Editors, formatters, encoders, packagers, QA tools |
| Outputs | Production-ready deliverables in final format |
| Time spent | Hours/days per production cycle |
| Pain points | Format conversion, quality control, multi-format output |

**Stage 4: Distribution** — Getting the product to the audience.

| Dimension | What to map |
|-----------|------------|
| Inputs | Production-ready deliverables |
| Activities | Publishing, uploading, syndicating, marketing, launching |
| Tools used | Publishing platforms, email tools, social media, ad platforms |
| Outputs | Content available to audience, marketing in flight |
| Time spent | Hours/days per distribution cycle |
| Pain points | Platform lock-in, algorithm dependence, multi-channel management |

**Stage 5: Monetization** — Converting audience attention into revenue.

| Dimension | What to map |
|-----------|------------|
| Inputs | Audience attention, distributed content |
| Activities | Pricing, checkout, payment processing, upselling, bundling |
| Tools used | Payment processors, subscription platforms, storefronts |
| Outputs | Revenue collected, financial reporting |
| Time spent | Hours/month on monetization management |
| Pain points | Platform fees, payment delays, tax complexity, refund handling |

**Stage 6: Community** — Building and maintaining the audience relationship.

| Dimension | What to map |
|-----------|------------|
| Inputs | Customers and followers from distribution/monetization |
| Activities | Engagement, support, feedback collection, community moderation |
| Tools used | Forums, chat platforms, email, social media, CRM |
| Outputs | Loyal audience, feedback for next creation cycle, referrals |
| Time spent | Hours/week on community management |
| Pain points | Platform fragmentation, moderation burden, engagement decay |

**Stage 7: Scaling** — Growing beyond individual capacity.

| Dimension | What to map |
|-----------|------------|
| Inputs | Proven lifecycle, revenue, audience |
| Activities | Hiring, automating, delegating, expanding product lines, entering new markets |
| Tools used | Project management, hiring platforms, automation tools, analytics |
| Outputs | Increased output without proportional increase in creator time |
| Time spent | Strategic planning hours/month |
| Pain points | Delegation difficulty, quality control at scale, tooling costs that scale with revenue |

### 2.3 Lifecycle Mapping Procedure

For each vertical being analyzed:

**Step 1: Research the vertical.** Use `RESEARCH_PATTERNS.md` 5-pass methodology. Search terms include:
- "[vertical] workflow tools"
- "[vertical] tech stack"
- "how [vertical]s make money"
- "[vertical] biggest frustrations"
- "[vertical] platform fees"
- "[vertical] alternatives to [dominant platform]"

**Step 2: Fill the 7-stage template.** For each stage, document every dimension from Section 2.2. Do not leave blanks — mark unknowns as "NEEDS RESEARCH" with a follow-up query.

**Step 3: Map the loops.** Identify where creators loop back:
- Creation ↔ Production (revision cycles)
- Distribution → Ideation (audience feedback drives new ideas)
- Community → Creation (community requests become products)
- Monetization → Distribution (revenue data drives marketing decisions)

**Step 4: Identify the bottleneck stage.** Which stage takes the most time, costs the most money, or causes the most frustration? This is where the highest-value recipes live.

**Step 5: Convergence check.** Two independent passes through the lifecycle (different research sources, different practitioner interviews) produce no new stages, tools, or pain points. If new information surfaces, loop back to Step 1.

### 2.4 Lifecycle Output Format

Store the completed lifecycle map as:

```
.cruxdev/verticals/{vertical_name}/lifecycle.md
```

One file per vertical. This file is the input to every subsequent analysis phase.

---

## 3. Tool and Platform Inventory

**Purpose:** For each lifecycle stage, catalog every tool and platform the vertical currently uses. This is the landscape that CruxVibe recipes must replace or improve upon.

### 3.1 Inventory Dimensions

For every tool discovered during lifecycle mapping, document:

| Dimension | Description | Example |
|-----------|-------------|---------|
| **Name** | Tool/platform name | Patreon |
| **Stage(s)** | Which lifecycle stages it covers | Monetization, Community |
| **Category** | Primary function | Subscription management |
| **Pricing model** | How they charge | 8% of revenue + payment processing |
| **Monthly cost** | At $5K, $10K, $25K, $50K creator revenue | $400, $800, $2000, $4000 |
| **Annual cost** | Total annual cost at each revenue tier | $4,800 - $48,000 |
| **Transaction fees** | Per-transaction costs | 2.9% + $0.30 (Stripe) + 8% (Patreon) |
| **Lock-in mechanisms** | What keeps creators trapped | Subscriber list not exportable, URL structure, SEO juice |
| **Data portability** | Can you export everything? | Partial — subscriber emails yes, payment history no |
| **API availability** | Can we integrate or migrate? | REST API, limited webhooks |
| **What's good** | Genuine strengths | Easy setup, built-in discovery, social proof |
| **What's terrible** | Real pain points (verified, not assumed) | Fee increases, algorithm changes, no customization |
| **What's missing** | Features users want but don't have | Multi-currency, bundle pricing, affiliate system |
| **Market position** | Dominant, challenger, niche, dying | Dominant (8M+ creators) |
| **User sentiment** | Overall creator satisfaction | Declining — fee backlash, exodus to alternatives |

### 3.2 Inventory Procedure

**Step 1: Start with lifecycle map.** For each stage, list every tool mentioned during research.

**Step 2: Expand with competitive discovery.** Use `COMPETITORS_PATTERN.md` Phase 1 methodology to find tools the lifecycle research missed. Search specifically for:
- "[vertical] tools [stage name]"
- "best [stage name] tools for [vertical]s"
- "alternatives to [dominant tool] for [vertical]s"
- "[vertical] [stage name] open source"

**Step 3: Price each tool at 4 revenue tiers.** Always calculate at $5K, $10K, $25K, and $50K monthly creator revenue. This reveals tools with predatory scaling (fees that grow faster than revenue).

**Step 4: Verify lock-in claims.** Actually test data export for the top 3 tools per stage. Do not assume portability — verify it. Document:
- What can be exported (formats, completeness)
- What cannot be exported
- How long export takes
- Whether export is actively hindered (rate limits, missing features, buried UI)

**Step 5: Calculate the "Platform Tax Stack."** Sum all platform fees across all stages for a creator at each revenue tier. This is the total addressable savings CruxVibe can offer.

### 3.3 Platform Tax Stack Calculation

For each vertical, produce this table:

```markdown
## Platform Tax Stack: [Vertical Name]

| Stage | Tool | Fee Type | $5K/mo | $10K/mo | $25K/mo | $50K/mo |
|-------|------|----------|--------|---------|---------|---------|
| Monetization | Patreon | % of revenue | $400 | $800 | $2,000 | $4,000 |
| Monetization | Stripe | Transaction | $175 | $350 | $875 | $1,750 |
| Distribution | Amazon KDP | % of revenue | $1,500 | $3,000 | $7,500 | $15,000 |
| Community | Circle | Flat fee | $400 | $400 | $400 | $400 |
| Newsletter | ConvertKit | Subscriber tier | $79 | $166 | $375 | $375 |
| **TOTAL** | | | **$2,554** | **$4,716** | **$11,150** | **$21,525** |
| **% of revenue** | | | **51%** | **47%** | **45%** | **43%** |
```

This table is the single most powerful sales artifact for CruxVibe. It shows creators exactly how much they are losing.

### 3.4 Inventory Output Format

Store the completed inventory as:

```
.cruxdev/verticals/{vertical_name}/tool_inventory.md
```

One file per vertical. Reference tools by name in all subsequent analysis phases.

---

## 4. Gap Classification System

**Purpose:** Classify every gap found during lifecycle mapping and tool inventory into one of 5 types. Classification determines the recipe strategy — different gap types require different solutions.

### 4.1 The Five Gap Types

#### 4.1.1 Capability Gap — No Tool Exists

**Definition:** The creator has a real need and no tool exists to address it. They use manual processes, spreadsheets, or simply do without.

**Identification signals:**
- Creator describes a manual workflow that should be automated
- "I wish there was a tool that..."
- Creator built their own hacky solution (Google Sheets, Zapier chains, custom scripts)
- The need is obvious in retrospect but no one has productized it

**Recipe strategy:** Greenfield build. No migration needed. Pure value creation. These are the easiest sells but may have smaller markets (if the market were large, someone would have built it).

**Example:** No tool exists to automatically generate book metadata (ISBNs, BISAC codes, keywords, descriptions) from a manuscript. Authors do this manually for every publishing platform.

#### 4.1.2 Quality Gap — Tools Exist But Are Bad

**Definition:** Tools exist for this need but they are poorly built, unreliable, have terrible UX, or produce subpar results.

**Identification signals:**
- 1-3 star reviews on G2/Capterra with specific complaints
- Reddit threads titled "Is [tool] as bad as I think it is?"
- Creators apologize for the tool while recommending it ("It's not great, but it's the best we have")
- High churn rates (users constantly switching between bad options)

**Recipe strategy:** Build a better version. Migration path required. Compete on quality, not price (though price is often better too). Must demonstrably solve the specific complaints — do not assume you know what is bad, verify from user reviews.

**Example:** eBook formatting tools (Vellum, Calibre) either cost $250+ with no updates or are free but require technical skill. Neither handles all formats well. Authors constantly complain about formatting breaking across devices.

#### 4.1.3 Cost Gap — Tools Exist But Are Too Expensive

**Definition:** Good tools exist but their pricing is predatory, especially at scale. The tool works fine — the problem is purely economic.

**Identification signals:**
- Percentage-of-revenue pricing (fees grow with success)
- Steep tier jumps ($0 → $400/mo with nothing in between)
- "Hidden" fees discovered after commitment (transaction fees on top of subscription)
- Creator revenue milestone triggers dramatic cost increases
- Creators actively seeking cheaper alternatives

**Recipe strategy:** Build equivalent functionality, charge flat or lower percentage. The migration pitch is pure economics: "Same features, keep $X,000 more per year." Must match feature parity on the things that matter — do not build a cheaper but worse tool.

**Example:** Kajabi charges $149-$399/mo for course hosting. The technology is straightforward (video hosting, progress tracking, payment). A CruxVibe recipe at $49/mo replaces $4,788/year with $588/year — a $4,200/year saving per creator.

#### 4.1.4 Integration Gap — Tools Exist But Don't Connect

**Definition:** Individual tools work fine in isolation, but creators need 5-10 of them and they do not talk to each other. The gap is in the connections, not the components.

**Identification signals:**
- Creators manually copying data between platforms
- Zapier/Make.com chains holding the workflow together
- "I spend 2 hours a week just syncing my subscriber list across platforms"
- Data inconsistencies between platforms (subscriber count differs everywhere)
- Creators hiring VAs specifically to manage tool synchronization

**Recipe strategy:** Two approaches:
1. **Unified platform** — replace multiple tools with one recipe that covers multiple stages
2. **Integration recipe** — build the connective tissue between existing tools (less revenue but faster to build)

Prefer the unified platform approach when 3+ tools are being replaced. The integration approach is a stepping stone — get creators into the ecosystem, then migrate them to full recipes.

**Example:** An author uses Amazon KDP (distribution), Mailchimp (newsletter), Patreon (community), and Gumroad (direct sales). None share subscriber data. A new reader on Amazon never gets added to the newsletter. A patron never gets the direct-sale discount. A CruxVibe ecosystem recipe unifies all four.

#### 4.1.5 AI Readiness Gap — AI Could Do This But Nobody Has Built It

**Definition:** A task in the creator lifecycle could be dramatically improved or automated by AI, but no tool has integrated AI for this purpose yet (or existing AI integrations are superficial).

**Identification signals:**
- The task is repetitive and pattern-based (AI can learn it)
- The task requires language understanding (summarization, classification, generation)
- The task involves content transformation (format conversion, repurposing)
- Existing tools added "AI features" that are just ChatGPT wrappers with no domain knowledge
- The creator currently pays a human to do something AI can do at 90%+ quality

**Recipe strategy:** Build AI-native. Not "existing tool + AI bolted on" but "reimagined workflow assuming AI exists." These recipes have the strongest moats because AI capability is hard to replicate without the domain expertise embedded in the recipe.

**AI readiness timeline classification:**
- **Now:** GPT-4/Claude-class models can do this today with proper prompting and tooling
- **6 months:** Requires fine-tuning, multimodal, or agent capabilities in active development
- **12 months:** Requires capabilities on research roadmaps but not yet in production
- **Never:** Requires human judgment, taste, or relationships that AI cannot replicate

**Example:** Podcast show notes are manually written by hosts or VAs. AI can transcribe, summarize, extract timestamps, identify topics, generate SEO-optimized descriptions, and create social media clips — all from the audio file. No podcast tool does this end-to-end today.

### 4.2 Gap Identification Procedure

**Step 1: Walk the lifecycle stage by stage.** For each stage, ask:
- What does the creator wish they could do that no tool supports? (Capability gap)
- What tools do they use but complain about? (Quality gap)
- What tools work but cost too much? (Cost gap)
- What data do they manually move between tools? (Integration gap)
- What tasks are manual but could be automated with AI? (AI readiness gap)

**Step 2: Validate gaps with evidence.** Every gap must be supported by at least 2 independent sources:
- User reviews (G2, Capterra, Reddit, Twitter)
- Practitioner interviews or blog posts
- Your own testing of the tools
- Market data (pricing pages, feature comparisons)

Do not classify gaps based on assumption. "I think this tool is bad" is not a gap. "47 Reddit posts in the last 6 months complain about this tool's export functionality" is a gap.

**Step 3: Assign primary classification.** Every gap gets exactly one primary type. If a gap spans multiple types (e.g., the tool is both bad AND expensive), classify by the PRIMARY pain:
- If creators would pay more for a better version → Quality gap
- If creators would accept the same quality for less → Cost gap
- If creators need the tool to work with others → Integration gap

**Step 4: Tag secondary classifications.** A gap can have secondary tags. Example: "Primary: Cost gap. Secondary: Quality gap, Integration gap." The primary classification drives recipe strategy; secondary tags inform feature prioritization.

### 4.3 Gap Output Format

Store gaps as:

```
.cruxdev/verticals/{vertical_name}/gaps.md
```

Each gap entry follows this template:

```markdown
### GAP-{NNN}: {Short Description}

- **Stage:** {Lifecycle stage}
- **Primary type:** {Capability | Quality | Cost | Integration | AI Readiness}
- **Secondary types:** {list or "none"}
- **Current tools:** {What creators use now, or "manual process"}
- **Pain evidence:** {2+ sources with links/quotes}
- **Affected population:** {Estimate of how many creators face this}
- **Revenue at stake:** {How much money flows through this gap annually}
- **Notes:** {Additional context}
```

---

## 5. Recipe Opportunity Scoring

**Purpose:** Score every identified gap on 6 dimensions to produce a prioritized recipe backlog. Not every gap is worth building for — scoring separates the high-value opportunities from the noise.

### 5.1 Scoring Dimensions

#### 5.1.1 Pain Level (1-5)

How much does this gap hurt the creator?

| Score | Definition | Signals |
|-------|-----------|---------|
| 1 | Minor annoyance | Creators mention it but work around it easily |
| 2 | Moderate friction | Creators spend 1-2 hours/week on workarounds |
| 3 | Significant pain | Creators actively search for solutions, post in forums |
| 4 | Major pain | Creators cite this as a top-3 frustration, some quit over it |
| 5 | Existential | Creators cannot function without solving this, it blocks revenue |

**Scoring rule:** Use the highest pain level supported by evidence. Do not average across creators — if 20% of creators rate this a 5 and 80% rate it a 2, score it as a 5 with a note about the affected segment.

#### 5.1.2 Market Size

How many creators have this problem?

| Tier | Range | Examples |
|------|-------|---------|
| Niche | <10,000 creators | Technical book authors, classical musicians |
| Small | 10,000 - 50,000 | Indie podcasters, specialized coaches |
| Medium | 50,000 - 250,000 | Course creators, newsletter writers |
| Large | 250,000 - 1,000,000 | YouTubers, general authors |
| Massive | 1,000,000+ | All content creators, all entrepreneurs |

**Scoring rule:** Count the number of creators who HAVE the problem, not the total vertical size. If only 30% of podcasters face this issue, the market size is 30% of all podcasters, not 100%.

Estimate sources:
- Platform user counts (Patreon: 8M+ creators, YouTube: 50M+ channels)
- Industry reports (Creator Economy reports from SignalFire, Linktree, etc.)
- Subreddit subscriber counts as a proxy for active community size
- Job board listings for the vertical (indicates professional population)

#### 5.1.3 AI Readiness

Can AI solve this problem, and when?

| Score | Timeline | Meaning |
|-------|----------|---------|
| 5 | Now | Current LLMs solve this with proper tooling. Build immediately. |
| 4 | 3-6 months | Requires capabilities in beta/preview. Start building, ship when ready. |
| 3 | 6-12 months | Requires capabilities on roadmaps. Design now, build later. |
| 2 | 12-24 months | Requires research breakthroughs. Monitor, do not build. |
| 1 | Never | Requires human judgment/taste/relationships. AI assists but does not replace. |

**Scoring rule:** Score based on the MINIMUM viable AI capability, not the ideal. If AI can do 80% of the task now and needs a human for 20%, score it a 4 or 5 with a note about the human-in-the-loop component.

**Important:** A score of 1 (Never) does not mean the gap is not worth addressing. It means the recipe should focus on workflow optimization and tool consolidation rather than AI automation.

#### 5.1.4 Revenue Potential

Would creators pay for this, and how much?

| Tier | Annual revenue per creator | Signals |
|------|---------------------------|---------|
| Low | <$100/year | Nice-to-have, unlikely to drive subscriptions |
| Medium | $100-$500/year | Would subscribe if bundled with other recipes |
| High | $500-$2,000/year | Would subscribe standalone |
| Very High | $2,000-$10,000/year | Replaces expensive platform, immediate ROI |
| Extreme | $10,000+/year | Replaces major platform fees at scale |

**Scoring rule:** Revenue potential = MIN(what they currently pay for alternatives, what they would pay for a better solution). Creators will not pay more than their current costs unless the solution is dramatically better AND saves them significant time.

**Willingness-to-pay validation:**
- If creators currently pay $X/mo for a bad tool, they will pay $X/mo for a good one
- If creators currently spend Y hours/mo on manual work, they will pay up to (Y * their hourly rate * 0.3) per month to eliminate it
- If creators currently lose Z% of revenue to platform fees, they will pay a flat fee up to 50% of Z to eliminate the percentage-based fee

#### 5.1.5 Platform to Cannibalize

Which existing platform does this recipe replace?

| Score | Cannibalization potential |
|-------|--------------------------|
| 5 | Replaces a dominant platform with >50% market share and high fees (Patreon, Amazon KDP) |
| 4 | Replaces a major platform with 20-50% market share (Teachable, ConvertKit) |
| 3 | Replaces a significant platform with 5-20% market share (Gumroad, Circle) |
| 2 | Replaces a niche tool or consolidates multiple small tools |
| 1 | No clear platform to cannibalize — greenfield capability |

**Scoring rule:** Higher scores mean easier marketing ("stop paying Patreon 12%") but harder migration. Lower scores mean harder marketing but easier implementation. Both extremes have value.

Document for each cannibalization target:
- Platform name and fee structure
- Estimated annual fees paid by the target creator segment
- Migration complexity (data export, subscriber notification, URL redirect)
- Switching cost for the creator (time, risk, learning curve)

#### 5.1.6 Implementation Complexity

How hard is this to build as a CruxVibe recipe?

| Score | Complexity | Meaning |
|-------|-----------|---------|
| 1 | Trivial | <40 hours. Static pages, simple CRUD, basic Stripe integration |
| 2 | Low | 40-120 hours. Standard web app, email integration, basic AI |
| 3 | Medium | 120-400 hours. Complex workflows, real-time features, mobile app |
| 4 | High | 400-1000 hours. Multi-service architecture, heavy AI, custom infrastructure |
| 5 | Extreme | 1000+ hours. Novel technology, regulatory compliance, hardware integration |

**Scoring rule:** Estimate based on CruxDev-converged development, not general development. CruxDev convergence is 3-5x faster than traditional development for PADL-stack recipes. Adjust accordingly.

### 5.2 Composite Score Calculation

```
Opportunity Score = (Pain * 3) + (Market * 2) + (AI Readiness * 2) + (Revenue * 3) + (Cannibalization * 2) - (Complexity * 2)
```

**Weight rationale:**
- Pain (3x): High pain = urgent need = faster adoption = lower acquisition cost
- Revenue (3x): Must be financially viable to build
- Market (2x): Size matters but a small market with extreme pain is still viable
- AI Readiness (2x): AI-ready gaps have stronger moats and lower marginal costs
- Cannibalization (2x): Clear "switch from X" narrative accelerates marketing
- Complexity (-2x): Harder to build = longer to revenue = higher risk

**Score interpretation:**

| Score range | Priority | Action |
|-------------|----------|--------|
| 40+ | P0 — Build now | Create build plan immediately. This is a high-value, feasible recipe. |
| 30-39 | P1 — Build next quarter | Strong opportunity. Queue after P0 items complete. |
| 20-29 | P2 — Plan and design | Worth doing but not urgent. Design the recipe, build when capacity allows. |
| 10-19 | P3 — Monitor | Viable but low priority. Re-score in 6 months (market conditions change). |
| <10 | Pass | Not worth building. Archive the analysis for future reference. |

### 5.3 Scoring Procedure

**Step 1: Score each gap independently.** Do not batch-score. Each gap gets its own analysis with evidence for each dimension.

**Step 2: Normalize across verticals.** When comparing gaps across different verticals, normalize market size to the same base. A "Large" market in podcasting is different from "Large" in e-commerce.

**Step 3: Sanity check the top 5.** For the top 5 scoring opportunities, do a deeper dive:
- Is the pain score inflated by vocal minority?
- Is the market size estimate based on solid data or guesses?
- Is the AI readiness score realistic given current model capabilities?
- Is the revenue potential validated by what creators actually spend today?
- Is the complexity estimate based on actual architecture analysis or optimism?

**Step 4: Produce the ranked backlog.** Sort all gaps by composite score, descending. This is the recipe priority backlog.

### 5.4 Scoring Output Format

Store scores as:

```
.cruxdev/verticals/{vertical_name}/opportunity_scores.md
```

Each scored gap follows this template:

```markdown
### GAP-{NNN}: {Short Description}

| Dimension | Score | Evidence |
|-----------|-------|----------|
| Pain Level | 4/5 | 47 Reddit posts, 3.1/5 G2 rating, top complaint in 2 surveys |
| Market Size | Medium (120K) | Based on Teachable's reported active creator count |
| AI Readiness | 5 (Now) | GPT-4 + Whisper handles transcription + summarization today |
| Revenue Potential | High ($1,200/yr) | Replaces $149/mo Teachable subscription |
| Cannibalization | 4 | Teachable has ~25% market share in course hosting |
| Complexity | 2 (Low) | Standard PADL recipe, video hosting via S3/CloudFront |

**Composite Score:** (4*3) + (3*2) + (5*2) + (4*3) + (4*2) - (2*2) = 12+6+10+12+8-4 = **44 (P0)**

**Recommended action:** Build immediately. Create BUILD_PLAN_NNN_COURSE_RECIPE.md.
```

---

## 6. CruxDev Capability Mapping

**Purpose:** For every gap scored P0-P2, determine what CruxDev needs to build recipes that address it. CruxDev is the factory — this section maps what the factory needs to produce the product.

### 6.1 Capability Dimensions

For each gap, assess needs across 4 CruxDev capability areas:

#### 6.1.1 Audit Dimensions

What should CruxDev's convergence engine audit when building a recipe for this gap?

| Category | Examples |
|----------|---------|
| **Functional** | Payment flow correctness, subscription lifecycle, content delivery |
| **Performance** | Page load time, video streaming latency, API response time |
| **Security** | Payment data handling, user authentication, content protection |
| **Accessibility** | WCAG compliance, screen reader support, keyboard navigation |
| **SEO** | Creator storefront discoverability, content indexing, structured data |
| **Mobile** | Responsive design, native app functionality, offline access |
| **Data portability** | Export completeness, format standards, migration tooling |
| **Integration** | Stripe webhook handling, email provider integration, API surface |

**Procedure:** For each gap, list the audit dimensions that a recipe must pass to be considered converged. These become convergence criteria in the build plan.

**Output format:**

```markdown
### Audit Dimensions for GAP-{NNN}

| Dimension | Convergence criterion | Test method |
|-----------|----------------------|-------------|
| Payment flow | All Stripe webhook events handled, refunds work, subscription upgrades/downgrades work | Automated test suite + manual Stripe test mode verification |
| Content delivery | 95th percentile page load < 2s, video start < 1s | Lighthouse CI + custom performance tests |
| Data portability | Full export of all creator data in standard formats within 5 minutes | Export test with 10K records |
```

#### 6.1.2 Pattern Documents

What development patterns must exist for building recipes that address this gap?

Patterns are reusable methodology documents (like this one) that guide development across multiple recipes. If a pattern does not exist and is needed by 2+ recipes, it must be created.

| Pattern type | When needed | Example |
|-------------|-------------|---------|
| **Stack pattern** | New technology combination | PADL + Stripe Connect pattern |
| **Domain pattern** | New business domain | Subscription billing lifecycle pattern |
| **Integration pattern** | New external service | Email provider integration pattern |
| **AI pattern** | New AI capability | Audio-to-content pipeline pattern |
| **Migration pattern** | Platform displacement | Patreon-to-self-hosted migration pattern |

**Procedure:** Check if existing patterns cover this gap. If not, create a pattern backlog item.

#### 6.1.3 Templates

What project templates should CruxDev provide to accelerate recipe development?

Templates are starting points — scaffolded projects that include boilerplate, standard configurations, and baseline tests. Unlike patterns (which are documents), templates are code.

| Template type | Contents | Example |
|--------------|----------|---------|
| **Recipe starter** | PADL scaffold + Stripe + auth + admin panel | `templates/padl-stripe-recipe/` |
| **Mobile companion** | Expo scaffold + API client + push notifications | `templates/expo-companion/` |
| **AI pipeline** | Whisper + GPT-4 + structured output + queue | `templates/ai-pipeline/` |
| **Migration tool** | Platform API client + data transformer + import wizard | `templates/migration-tool/` |

**Procedure:** For each P0-P1 gap, determine which templates would reduce implementation from weeks to days. If a template does not exist and would serve 3+ recipes, create a template backlog item.

#### 6.1.4 MCP Tools

What MCP (Model Context Protocol) tools should CruxDev provide for building and operating recipes?

MCP tools are capabilities exposed to the AI agent during development and operation. They extend what the agent can do autonomously.

| Tool category | Purpose | Example |
|--------------|---------|---------|
| **Audit tools** | Verify recipe correctness | `audit_stripe_integration`, `audit_content_delivery` |
| **Generation tools** | Scaffold recipe components | `generate_subscription_flow`, `generate_admin_panel` |
| **Migration tools** | Help creators move data | `import_patreon_subscribers`, `import_mailchimp_list` |
| **Monitoring tools** | Track recipe health in production | `check_payment_health`, `check_delivery_stats` |
| **AI tools** | Wrap AI capabilities for recipes | `transcribe_audio`, `generate_show_notes` |

**Procedure:** For each P0-P1 gap, list the MCP tools that would be needed. Cross-reference with existing tools (avoid duplication). New tools go on the MCP tool backlog.

### 6.2 Capability Mapping Output

Store capability maps as:

```
.cruxdev/verticals/{vertical_name}/cruxdev_capabilities.md
```

Summary table format:

```markdown
## CruxDev Capability Map: {Vertical Name}

| Gap | Audit Dimensions | Patterns Needed | Templates Needed | MCP Tools Needed |
|-----|-----------------|-----------------|------------------|-----------------|
| GAP-001 | Payment, Content, Mobile | Stripe Connect, PADL | padl-stripe-recipe | audit_stripe, generate_subscription |
| GAP-002 | AI, Performance | AI Pipeline | ai-pipeline | transcribe_audio, generate_notes |
```

---

## 7. CruxVibe Recipe Mapping

**Purpose:** For every gap scored P0-P2, define the CruxVibe recipe that addresses it. This is the product specification that feeds into build plans.

### 7.1 Recipe Definition Template

Every recipe must be fully defined before development begins.

```markdown
## Recipe: {Recipe Name}

### Identity
- **Name:** {Human-readable name, e.g., "Course Platform"}
- **Slug:** {URL-safe identifier, e.g., "course"}
- **Tagline:** {One-line value prop, e.g., "Host your courses. Keep your revenue."}
- **Replaces:** {Platform(s) this cannibalizes, e.g., "Teachable, Kajabi, Thinkific"}
- **Vertical(s):** {Which verticals use this, e.g., "Course creators, coaches, educators"}

### Stack
- **Primary:** {PADL | Expo | PADL+Expo | Other}
- **Database:** {PostgreSQL (default) | SQLite | Other}
- **AI components:** {None | Transcription | Generation | Classification | Other}
- **External services:** {Stripe, S3, CloudFront, SendGrid, etc.}

### Stripe Integration
- **Required:** {Yes | No}
- **Type:** {Stripe Connect (marketplace) | Direct (single merchant) | Both}
- **Payment models:** {One-time | Subscription | Both | Usage-based}
- **Payout frequency:** {Instant | Daily | Weekly | Monthly}
- **Fee structure:** {CruxVibe's take rate and how it's applied}

### Hosting Requirements
- **Compute:** {Small (1 vCPU, 512MB) | Medium (2 vCPU, 2GB) | Large (4 vCPU, 8GB)}
- **Storage:** {Minimal (<1GB) | Medium (1-50GB) | Heavy (50GB+) | Unlimited (S3)}
- **Bandwidth:** {Low | Medium | High (streaming) | Extreme (live video)}
- **Custom domain:** {Required | Optional}
- **SSL:** {Let's Encrypt (default) | Custom cert support}
- **CDN:** {Not needed | CloudFront | Other}

### Estimated Development
- **CruxDev-converged hours:** {Estimate with CruxDev}
- **Traditional hours:** {Estimate without CruxDev, for comparison}
- **Dependencies:** {Other recipes or infrastructure that must exist first}
- **Target ship date:** {Based on priority and dependencies}

### Migration Path
- **From:** {Platform name}
- **Data to migrate:** {Subscribers, content, payment history, etc.}
- **Migration tool needed:** {Yes/No — if yes, reference MCP tool}
- **Estimated migration time per creator:** {Minutes/hours}
- **Risk level:** {Low (data export easy) | Medium | High (data locked in)}

### Revenue Model
- **CruxVibe pricing:** {What we charge the creator}
- **Creator savings vs incumbent:** {Annual savings at each revenue tier}
- **Breakeven for creator:** {When does switching become net positive}
- **LTV estimate:** {Expected lifetime value per creator}
```

### 7.2 Recipe Mapping Procedure

**Step 1: One recipe per P0-P1 gap.** Every high-priority gap gets a recipe definition. Some recipes may address multiple gaps (a unified recipe that covers 3 integration gaps, for example).

**Step 2: Identify recipe bundles.** Recipes that are commonly used together become bundles. Example: Author Bundle = ePublishing + Newsletter + Storefront + Community. Bundles have their own pricing (discounted vs. a la carte).

**Step 3: Map recipe dependencies.** Some recipes require shared infrastructure:
- Stripe Connect setup (shared across all payment recipes)
- User authentication system (shared across all recipes)
- Admin panel framework (shared across all recipes)
- AI pipeline infrastructure (shared across AI-powered recipes)

These shared components are "foundation recipes" that must be built first.

**Step 4: Sequence the build order.** Based on:
1. Dependencies (foundation recipes first)
2. Opportunity score (P0 before P1)
3. Cross-vertical reuse (recipes that serve 3+ verticals before vertical-specific ones)
4. Revenue potential (higher revenue recipes first when scores are tied)

### 7.3 Recipe Mapping Output

Store recipe definitions as:

```
.cruxdev/verticals/{vertical_name}/recipes.md
```

Cross-vertical recipe index stored as:

```
.cruxdev/verticals/_index/recipe_registry.md
```

---

## 8. Competitive Moat Analysis

**Purpose:** For every recipe, determine what makes CruxVibe's solution defensible against competitors who will inevitably try to replicate it.

### 8.1 Moat Types

#### 8.1.1 Cost Structure Moat

**Definition:** CruxVibe's cost to serve is structurally lower than incumbents, enabling permanently lower pricing.

**How to build it:**
- No venture capital subsidies to repay (no need to extract 30% margins)
- AI automation reduces operational cost per creator
- CruxDev convergence reduces development cost per recipe
- Shared infrastructure across recipes amortizes fixed costs

**How to assess it:** Calculate the fully-loaded cost per creator per month for the incumbent vs. CruxVibe. If CruxVibe's cost is <50% of the incumbent's, the cost moat is strong.

#### 8.1.2 Integration Moat

**Definition:** CruxVibe recipes work together in ways that standalone tools cannot replicate.

**How to build it:**
- Unified data model across all recipes (one subscriber = one record everywhere)
- Cross-recipe automations (new course sale triggers newsletter welcome sequence)
- Single admin panel for all recipes
- Unified analytics across all revenue streams

**How to assess it:** Count the number of cross-recipe integrations. Each integration is a switching cost — the more integrations a creator uses, the harder it is to leave.

#### 8.1.3 AI Moat

**Definition:** CruxVibe's AI capabilities improve with usage in ways competitors cannot easily replicate.

**How to build it:**
- Domain-specific fine-tuning on creator workflows (with permission)
- Aggregate pattern learning across creators (anonymized)
- AI that understands the creator's specific audience, style, and content
- Proprietary AI pipelines that combine multiple models for creator-specific tasks

**How to assess it:** Does the AI get better the longer a creator uses it? If yes, the moat deepens over time. If the AI is just a ChatGPT wrapper, there is no moat.

#### 8.1.4 Data Moat

**Definition:** CruxVibe accumulates data that improves the product and cannot be replicated by competitors.

**How to build it:**
- Creator performance benchmarks (anonymized: "your course completion rate is in the top 20%")
- Pricing optimization data ("creators in your niche who price at $X see Y% more sales")
- Audience behavior patterns ("your readers engage most on Tuesdays at 10am")
- Content performance data ("chapters of this length get the highest completion rates")

**How to assess it:** What decisions could CruxVibe help creators make that require aggregate data no single creator has? Each such decision is a data moat.

#### 8.1.5 Migration Moat

**Definition:** CruxVibe makes it easy to migrate IN and hard for competitors to migrate creators OUT.

**How to build it:**
- One-click migration from every major competitor (we invest in import tools)
- Standard data formats internally (no proprietary lock-in posturing — creators can always export)
- Migration concierge service for high-value creators
- The paradox: making export easy builds trust, which reduces churn more than lock-in would

**How to assess it:** Measure migration friction score: time + risk + effort to move to CruxVibe vs. away from CruxVibe. Ideal: low friction in, moderate friction out (not from lock-in but from integration depth).

### 8.2 Moat Assessment Procedure

For each recipe:

**Step 1:** Score each moat type as Strong / Medium / Weak / None.

**Step 2:** Identify the primary moat (the one most likely to prevent competitor replication).

**Step 3:** Identify moat gaps (types where the moat is Weak or None) and determine if they can be strengthened.

**Step 4:** Estimate the moat's time horizon. How long until a well-funded competitor could replicate this? Answers should be in months:
- <6 months: No real moat. Compete on execution speed.
- 6-12 months: Moderate moat. First-mover advantage matters.
- 12-24 months: Strong moat. Sustainable competitive advantage.
- 24+ months: Deep moat. Structural advantage.

### 8.3 Moat Analysis Output

Store moat analysis as:

```
.cruxdev/verticals/{vertical_name}/moat_analysis.md
```

---

## 9. The Cannibalization Playbook

**Purpose:** A step-by-step methodology for identifying a platform that takes a cut of creator revenue, building a recipe that replaces it, and migrating creators off the platform.

### 9.1 Target Identification

Not every platform is worth cannibalizing. The ideal target meets ALL of these criteria:

| Criterion | Threshold | Why |
|-----------|-----------|-----|
| Fee percentage | >5% of creator revenue | Below 5%, the savings aren't compelling enough to switch |
| Creator dissatisfaction | >30% negative sentiment | Creators must want to leave, not just save money |
| Data exportability | Partial or better | If no data can be exported, migration is too painful |
| Technical complexity | Low to medium | The platform's technology must be replicable |
| Market size | >50,000 active creators | Must be worth the development investment |
| Switching cost | Low to medium | High switching costs kill migration regardless of savings |

**Scoring:** Rate each criterion as Pass/Fail. A target must pass at least 5 of 6 criteria. All 6 is ideal.

### 9.2 Intelligence Gathering

Before building the replacement, deeply understand the target platform.

**Step 1: Feature audit.** Create a complete feature list of the platform being cannibalized. Categorize each feature as:
- **Must-have:** Creators will not switch without this (payment processing, content hosting)
- **Nice-to-have:** Creators want this but will switch without it (analytics, integrations)
- **Not needed:** Platform added this for upselling, creators do not use it (AI features nobody asked for)

**Step 2: Pricing analysis.** Map the complete fee structure including hidden fees:
- Base subscription (if any)
- Transaction fees (percentage + fixed per transaction)
- Payment processing fees (often passed through but sometimes marked up)
- Feature tier upsells
- Bandwidth/storage overage charges
- Custom domain fees
- API access fees

**Step 3: Migration path assessment.** What data can creators export?
- Subscriber/customer list (emails, names, subscription status)
- Content (posts, courses, files)
- Payment history (transactions, subscriptions, refunds)
- Analytics (views, engagement, revenue reports)
- URL redirects (can we preserve SEO and bookmarks?)

**Step 4: Creator sentiment analysis.** Using `RESEARCH_PATTERNS.md`, research:
- "[platform] alternatives"
- "[platform] too expensive"
- "leaving [platform]"
- "[platform] migration"
- "[platform] complaints"

Document the top 10 complaints by frequency. These become feature priorities.

### 9.3 Recipe Development

**Step 1: Feature parity on must-haves.** The recipe must match or exceed the platform on every must-have feature. No exceptions. A creator who switches and loses a must-have feature will switch back and never return.

**Step 2: Exceed on the top 3 complaints.** Whatever the platform's most common complaints are, the recipe must demonstrably solve them. This is the marketing narrative: "We fixed what [Platform] got wrong."

**Step 3: AI advantage on 2+ features.** Identify at least 2 features where AI can provide a dramatically better experience than the platform offers. These become the "wow" moments in the migration pitch.

**Step 4: Price at 50-70% savings.** The recipe must save creators at least 50% compared to the platform at their revenue level. Use the Platform Tax Stack calculation from Section 3.3 to determine exact savings messaging per revenue tier.

### 9.4 Migration Execution

**Phase 1: Migration Tool** (build before launch)
- One-click data import from the target platform
- Subscriber notification system ("I've moved to my own platform")
- URL redirect setup (preserve SEO and bookmarks)
- Parallel running support (run both platforms for 30-60 days during transition)

**Phase 2: Migration Concierge** (for first 50 creators)
- Personal migration assistance for early adopters
- White-glove data transfer verification
- 30-day money-back guarantee
- Case study creation (document savings, migration time, creator satisfaction)

**Phase 3: Self-Service Migration** (after 50 successful migrations)
- Automated migration wizard
- Video walkthrough of migration process
- Migration time estimator ("Based on your account size, migration takes ~15 minutes")
- Community forum for migration support

**Phase 4: Marketing Engine** (ongoing)
- "[Platform] alternative" landing pages (SEO)
- "[Platform] vs CruxVibe" comparison pages
- Creator testimonials with real savings numbers
- Fee calculator tool ("See how much [Platform] costs you per year")

### 9.5 Cannibalization Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Migration conversion rate | >15% of creators who visit comparison page | Analytics |
| Migration completion rate | >90% of creators who start migration | Funnel tracking |
| 30-day retention | >95% of migrated creators still active | Cohort analysis |
| Revenue savings claimed | >$500/year average per migrated creator | Savings calculator verification |
| NPS of migrated creators | >50 | Survey at 30 and 90 days |
| Time to migrate | <30 minutes for 90% of creators | Migration tool telemetry |

### 9.6 Cannibalization Output

Store the playbook for each target as:

```
.cruxdev/verticals/{vertical_name}/cannibalization/{platform_name}.md
```

---

## 10. Cross-Vertical Synthesis

**Purpose:** After analyzing individual verticals, synthesize findings across all verticals to identify shared gaps, shared recipes, and the optimal build order for maximum cross-vertical impact.

### 10.1 Shared Gap Identification

Many gaps appear across multiple verticals. These are the highest-priority recipes because one build serves many markets.

**Procedure:**

**Step 1:** Export all gaps from all analyzed verticals into a single list.

**Step 2:** Cluster gaps by function (payment, content delivery, community, etc.) regardless of vertical.

**Step 3:** For each cluster, determine:
- How many verticals share this gap?
- Is the gap identical across verticals or does each vertical need customization?
- What is the aggregate market size (sum of all vertical-specific market sizes)?
- What is the aggregate revenue potential?

**Step 4:** Re-score shared gaps using the composite scoring formula from Section 5, but with market size = aggregate market size and revenue potential = aggregate revenue potential.

### 10.2 Shared Recipe Identification

Some recipes serve multiple verticals with zero or minimal customization.

| Recipe | Verticals served | Customization needed |
|--------|-----------------|---------------------|
| Subscription/Patreon | All | Tier names, reward types |
| Newsletter | All | Template designs |
| Storefront | All | Product type (digital/physical) |
| Community | All | Forum categories, roles |
| Course | Coaches, educators, experts | Curriculum structure |
| Podcast | Podcasters, musicians, creators | Audio format handling |
| ePublishing | Authors | Format conversion |

Recipes serving 5+ verticals with minimal customization are **foundation recipes** — build them first.

### 10.3 Build Order Optimization

The optimal build order maximizes cumulative value delivered across all verticals over time.

**Algorithm:**

```
1. Score each recipe: value = opportunity_score * verticals_served
2. Adjust for dependencies: if recipe A requires recipe B, B's adjusted_value += A.value * 0.5
3. Sort by adjusted_value descending
4. For each recipe in order:
   a. If all dependencies are built or in progress → schedule it
   b. If dependencies are missing → schedule dependencies first
5. Output: ordered build plan with estimated ship dates
```

### 10.4 Synthesis Output

Store cross-vertical synthesis as:

```
.cruxdev/verticals/_index/cross_vertical_synthesis.md
```

This file is the master recipe priority list and the primary input to product roadmap planning.

---

## 11. Analysis Lifecycle and Convergence

**Purpose:** Define the complete lifecycle of a vertical gap analysis, from initiation to actionable build plans, with convergence criteria at each phase.

### 11.1 Analysis Phases

```
Phase 1: Lifecycle Mapping        → Output: lifecycle.md
Phase 2: Tool Inventory           → Output: tool_inventory.md
Phase 3: Gap Identification       → Output: gaps.md
Phase 4: Opportunity Scoring      → Output: opportunity_scores.md
Phase 5: CruxDev Capability Map   → Output: cruxdev_capabilities.md
Phase 6: Recipe Definitions       → Output: recipes.md
Phase 7: Moat Analysis            → Output: moat_analysis.md
Phase 8: Cannibalization Plans    → Output: cannibalization/{platform}.md
Phase 9: Cross-Vertical Synthesis → Output: cross_vertical_synthesis.md (shared)
```

### 11.2 Convergence Criteria Per Phase

| Phase | Converged when |
|-------|---------------|
| 1. Lifecycle | Two independent research passes find no new stages, tools, or pain points |
| 2. Inventory | Every tool mentioned in Phase 1 is fully documented with all dimensions from Section 3.1 |
| 3. Gaps | Every lifecycle stage has been examined for all 5 gap types; every gap has 2+ evidence sources |
| 4. Scoring | Every gap has all 6 dimensions scored with evidence; top 5 have passed sanity check |
| 5. Capabilities | Every P0-P2 gap has audit dimensions, pattern needs, template needs, and tool needs documented |
| 6. Recipes | Every P0-P1 gap has a complete recipe definition using Section 7.1 template |
| 7. Moat | Every recipe has all 5 moat types assessed with time horizon estimate |
| 8. Cannibalization | Every Cost gap target platform has a complete cannibalization plan |
| 9. Synthesis | All verticals have been cross-referenced; shared recipes identified; build order produced |

### 11.3 Time Estimates Per Phase

| Phase | Estimated time | Parallelizable with |
|-------|---------------|-------------------|
| 1. Lifecycle | 4-8 hours | Nothing (must complete first) |
| 2. Inventory | 4-8 hours | Nothing (requires Phase 1) |
| 3. Gaps | 2-4 hours | Nothing (requires Phase 2) |
| 4. Scoring | 2-4 hours | Nothing (requires Phase 3) |
| 5. Capabilities | 2-4 hours | Phase 6, 7 |
| 6. Recipes | 4-8 hours | Phase 5, 7 |
| 7. Moat | 2-4 hours | Phase 5, 6 |
| 8. Cannibalization | 4-8 hours per platform | Phase 9 (for other verticals) |
| 9. Synthesis | 2-4 hours | Nothing (requires all verticals) |

**Total per vertical:** 24-48 hours of analysis work.

### 11.4 Iteration Protocol

After initial convergence, re-analyze each vertical every 6 months or when:
- A major platform changes pricing (triggers re-scoring of Cost gaps)
- A new AI capability ships (triggers re-scoring of AI Readiness gaps)
- A new competitor enters the market (triggers new gap identification)
- Creator sentiment shifts significantly (triggers re-assessment of Pain scores)
- CruxVibe ships a new recipe (triggers re-assessment of Integration gaps)

---

## 12. Output Artifacts

**Purpose:** Define the complete set of files produced by a vertical gap analysis and where they live.

### 12.1 File Structure

```
.cruxdev/verticals/
├── _index/
│   ├── recipe_registry.md          # All recipes across all verticals
│   ├── cross_vertical_synthesis.md  # Cross-vertical analysis
│   └── build_order.md              # Prioritized build sequence
├── authors/
│   ├── lifecycle.md
│   ├── tool_inventory.md
│   ├── gaps.md
│   ├── opportunity_scores.md
│   ├── cruxdev_capabilities.md
│   ├── recipes.md
│   ├── moat_analysis.md
│   └── cannibalization/
│       ├── amazon_kdp.md
│       ├── patreon.md
│       └── substack.md
├── podcasters/
│   ├── lifecycle.md
│   ├── ...
├── course_creators/
│   ├── lifecycle.md
│   ├── ...
└── {vertical_name}/
    ├── lifecycle.md
    ├── ...
```

### 12.2 Build Plan Generation

For every P0 recipe, generate a CruxDev build plan:

```
build_plans/BUILD_PLAN_NNN_RECIPE_NAME.md
```

The build plan follows the standard CruxDev build plan format (see `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md`) and includes:
- Recipe definition (from Section 7.1)
- Audit dimensions (from Section 6.1.1)
- Pattern dependencies (from Section 6.1.2)
- Template dependencies (from Section 6.1.3)
- MCP tool dependencies (from Section 6.1.4)
- Migration requirements (from Section 9)
- Convergence criteria (from the recipe definition + audit dimensions)

### 12.3 Artifact Quality Standards

Every artifact must meet these standards before the analysis is considered converged:

| Standard | Requirement |
|----------|------------|
| **Evidence-based** | Every claim cites a source. No unsupported assumptions. |
| **Quantified** | Revenue, market size, time, and cost are numbers, not adjectives. |
| **Actionable** | Every gap leads to a scored opportunity. Every P0-P1 opportunity leads to a recipe definition. Every recipe definition leads to a build plan. |
| **Dated** | Every artifact has a creation date and "last verified" date. |
| **Internally consistent** | Gap counts in gaps.md match gap counts in opportunity_scores.md. Recipe counts in recipes.md match P0-P1 counts in scores. |

---

## Appendix A: Vertical-Specific Lifecycle Templates

Pre-populated lifecycle templates for common verticals. These are starting points — every template must be validated and expanded through the research process in Section 2.3.

### A.1 Authors

```
Ideation:      Topic research → outline → proposal → validation
Creation:      Manuscript writing → revision → beta readers → final draft
Production:    Formatting (epub/mobi/pdf) → cover design → metadata → ISBN
Distribution:  Amazon KDP → IngramSpark → direct sales → library → audiobook
Monetization:  Book sales → subscription (serialized) → courses → speaking
Community:     Newsletter → reader group → launch team → ARC readers
Scaling:       Series → co-authoring → ghostwriting management → publishing imprint
```

### A.2 Podcasters

```
Ideation:      Topic selection → guest research → episode planning → scheduling
Creation:      Recording → interview → co-host sessions
Production:    Editing → mixing → mastering → show notes → transcription → clips
Distribution:  RSS → Apple Podcasts → Spotify → YouTube → social clips
Monetization:  Sponsorships → premium episodes → merchandise → live events → courses
Community:     Discord/forum → listener Q&A → live recordings → meetups
Scaling:       Network → multiple shows → team hiring → ad sales team
```

### A.3 Course Creators

```
Ideation:      Topic validation → curriculum design → learning outcomes → pricing research
Creation:      Video recording → slide creation → worksheet design → quiz creation
Production:    Video editing → encoding → captioning → platform upload → quality check
Distribution:  Course platform → email launch → webinar funnel → affiliate program
Monetization:  One-time sale → subscription → cohort-based → certification → licensing
Community:     Student forum → office hours → peer groups → alumni network
Scaling:       Course library → instructors → enterprise licensing → accreditation
```

### A.4 Coaches

```
Ideation:      Methodology development → framework creation → niche selection → positioning
Creation:      Program design → session structure → workbook creation → assessment tools
Production:    Video/audio of frameworks → workbook formatting → client portal setup
Distribution:  Discovery call funnel → referral system → content marketing → partnerships
Monetization:  1:1 coaching → group programs → courses → retreats → certification
Community:     Client community → peer mastermind → alumni group → practitioner network
Scaling:       Certified coaches → group programs → digital products → licensing
```

### A.5 Newsletter Writers

```
Ideation:      Topic curation → research → angle development → headline testing
Creation:      Writing → editing → formatting → link curation
Production:    Template design → personalization → segmentation → A/B testing
Distribution:  Email send → archive publication → social sharing → cross-promotion
Monetization:  Paid subscriptions → sponsorships → affiliate → products → consulting
Community:     Reply engagement → reader meetups → referral program → community chat
Scaling:       Multiple newsletters → team writers → ad network → media company
```

### A.6 Musicians

```
Ideation:      Songwriting → composition → arrangement → collaboration
Creation:      Recording → performance → production → session musicians
Production:    Mixing → mastering → artwork → metadata → ISRC codes
Distribution:  Streaming (Spotify/Apple) → Bandcamp → direct sales → sync licensing
Monetization:  Streaming royalties → direct sales → merchandise → live → licensing
Community:     Fan club → social media → newsletter → exclusive content → meetups
Scaling:       Label → management → touring team → merchandise line → publishing
```

### A.7 YouTubers

```
Ideation:      Topic research → SEO analysis → trending topics → audience polling
Creation:      Scripting → filming → B-roll → graphics → thumbnail design
Production:    Editing → color grading → sound design → captioning → thumbnail A/B
Distribution:  YouTube upload → shorts → social clips → blog post → podcast version
Monetization:  Ad revenue → sponsorships → merchandise → memberships → courses
Community:     Comments → Discord → livestreams → meetups → collaborations
Scaling:       Team (editor, thumbnail, writer) → multiple channels → media company
```

### A.8 Agencies

```
Ideation:      Service design → niche selection → productization → pricing model
Creation:      Client deliverables → templates → SOPs → case studies
Production:    Quality assurance → client review → revision → delivery
Distribution:  Outbound → content marketing → referrals → partnerships → directories
Monetization:  Project fees → retainers → productized services → SaaS → licensing
Community:     Client community → referral network → industry presence → thought leadership
Scaling:       Hiring → subcontracting → white-labeling → franchising → SaaS pivot
```

---

## Appendix B: Scoring Worked Example

### B.1 Vertical: Authors

**Gap: eBook Formatting and Multi-Platform Publishing**

Authors must format manuscripts into .epub, .mobi, .pdf, and print-ready formats, then upload to 5+ platforms (Amazon KDP, IngramSpark, Apple Books, Google Play, direct sales), each with different requirements.

**Current tools:**
- Vellum ($250 one-time, Mac only)
- Calibre (free, steep learning curve)
- Draft2Digital (free formatting, takes 10% of sales)
- Atticus ($148 one-time, newer)
- Reedsy (free formatter, limited features)

**Gap classification:** Integration gap (primary), Quality gap (secondary), Cost gap (secondary)

**Scoring:**

| Dimension | Score | Evidence |
|-----------|-------|----------|
| Pain Level | 4/5 | r/selfpublish top-10 complaint; 200+ posts/year about formatting issues; authors report spending 10-40 hours per book on formatting across platforms |
| Market Size | Large (800K+) | Amazon KDP has 4M+ titles published annually; estimated 800K active self-published authors worldwide |
| AI Readiness | 4 (3-6mo) | AI can handle format conversion now; intelligent layout decisions need multimodal understanding in beta |
| Revenue Potential | High ($600/yr) | Replaces Draft2Digital 10% (at $500/mo revenue = $600/yr) or Vellum amortized cost; saves 20+ hours per book |
| Cannibalization | 4 | Draft2Digital has ~20% of self-pub distribution; Amazon KDP has 70%+ |
| Complexity | 2 (Low) | Format conversion is well-understood; API integrations exist for all platforms; PADL recipe with file processing pipeline |

**Composite Score:** (4*3) + (4*2) + (4*2) + (4*3) + (4*2) - (2*2) = 12+8+8+12+8-4 = **44 (P0)**

**Recommended action:** Build immediately. This is a high-value, cross-vertical recipe (also serves technical writers, academics, newsletter writers who bundle into books). Create BUILD_PLAN for ePublishing recipe.

---

## Appendix C: Anti-Patterns

Common mistakes that invalidate a vertical gap analysis.

### C.1 Assumption-Based Gaps

**Anti-pattern:** "I think creators would want this" without evidence.
**Correction:** Every gap requires 2+ independent evidence sources. If you cannot find evidence, the gap may not exist.

### C.2 Solution-First Thinking

**Anti-pattern:** Starting with "We should build X" and working backward to justify it.
**Correction:** Start with the lifecycle map. Discover gaps. Score gaps. THEN define recipes. The methodology prevents solution-first bias by forcing evidence collection before scoring.

### C.3 Inflated Pain Scores

**Anti-pattern:** Scoring every gap as 4 or 5 pain because you want to build it.
**Correction:** Pain scores must be calibrated to the definitions in Section 5.1.1. A pain score of 5 means creators CANNOT FUNCTION without solving this. Most gaps are 2-3.

### C.4 Ignoring Switching Costs

**Anti-pattern:** Assuming creators will switch platforms because the math is better.
**Correction:** Switching costs are real. A creator with 10,000 Patreon subscribers faces massive risk in migrating. The cannibalization playbook (Section 9) exists specifically to address this. Never assume "better" automatically means "they will switch."

### C.5 Single-Vertical Thinking

**Anti-pattern:** Building a recipe for one vertical when it could serve five.
**Correction:** Cross-vertical synthesis (Section 10) must happen before build order decisions. A recipe that scores 30 in one vertical but serves 5 verticals is more valuable than a recipe that scores 40 in one vertical.

### C.6 Underestimating Incumbents

**Anti-pattern:** "Patreon is terrible, everyone will switch."
**Correction:** Incumbents have distribution, brand recognition, network effects, and creator inertia. Respect these advantages. The cannibalization playbook exists because displacing incumbents is hard, not because it is easy.

### C.7 Overestimating AI Readiness

**Anti-pattern:** Scoring every gap as AI Readiness 5 because "AI can do everything."
**Correction:** Test the AI capability before scoring. Can GPT-4/Claude actually do this task reliably today? Try it. If it fails 30% of the time, the readiness score is 3 or 4, not 5.

### C.8 Skipping the Platform Tax Stack

**Anti-pattern:** Talking about savings without calculating exact numbers.
**Correction:** The Platform Tax Stack table (Section 3.3) must be completed with real pricing data at all 4 revenue tiers. "Save money" is not a pitch. "Save $26,616/year" is a pitch.

---

*This pattern is maintained by the CruxDev convergence engine. Last verified: 2026-03-28.*
