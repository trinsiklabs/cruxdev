# X/Twitter Posting Schedule Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Buffer (8.7M tweets analyzed), Sprout Social (2.7B engagements / 470K profiles), Hootsuite, SocialBee, SocialPilot (50K+ accounts), PostEverywhere (700K posts), Tweet Archivist, RecurPost, Distribution.ai
**Last updated:** 2026-03-28
**Audience:** Developer/tech, B2B SaaS, open-source tool builders
**Integration:** CruxDev BIP pipeline via Typefully API

---

## 1. Optimal Posting Times by Audience Type

### 1.1 Developer/Tech Audience (CruxDev Primary)

Developers check X during three windows: morning ramp-up, lunch break, and late-afternoon wind-down.

| Window | Time (ET) | Time (PT) | Time (UTC) | Why It Works |
|--------|-----------|-----------|------------|--------------|
| **Morning prime** | 9:00-11:00 AM | 6:00-8:00 AM | 14:00-16:00 | Engineers settled into work, checking tech news before deep work |
| **Lunch scroll** | 12:00-1:00 PM | 9:00-10:00 AM | 17:00-18:00 | Break from coding, catching up on timeline |
| **Afternoon wind-down** | 3:00-5:00 PM | 12:00-2:00 PM | 20:00-22:00 | Context-switching, browsing trends and commentary |
| **Sunday evening** | 7:00-9:00 PM | 4:00-6:00 PM | 00:00-02:00 Mon | Engineers winding down before workweek — threads perform 3x better here |

**Key insight:** Dev-focused threads posted Sunday evening outperform the same content posted Tuesday morning by up to 3x, because engineers are in a reflective, planning mindset before Monday.

### 1.2 B2B SaaS Audience

B2B buyers and decision-makers overlap heavily with the developer audience but skew slightly later in the morning.

| Window | Time (ET) | Why It Works |
|--------|-----------|--------------|
| **Decision-maker morning** | 10:00 AM-12:00 PM | After morning meetings, checking industry news |
| **Post-lunch research** | 1:00-3:00 PM | Afternoon evaluation window — comparison content performs here |
| **Thursday planning** | 10:00 AM-2:00 PM Thu | Teams planning next sprint, evaluating tools |

**Best days for B2B:** Monday and Thursday (Hootsuite), Tuesday-Thursday (Sprout Social). The overlap is Tuesday and Thursday.

### 1.3 Global Audience Across Timezones

If targeting a global developer audience (open-source projects), you must hit multiple timezone windows.

| Target Region | Peak Window (Local) | UTC Equivalent |
|--------------|--------------------|----|
| US East Coast | 9:00-11:00 AM ET | 14:00-16:00 |
| US West Coast | 9:00-11:00 AM PT | 17:00-19:00 |
| Western Europe (UK/DE/FR) | 10:00 AM-12:00 PM CET | 09:00-11:00 |
| India | 10:00 AM-12:00 PM IST | 04:30-06:30 |
| East Asia (JP/KR) | 10:00 AM-12:00 PM JST | 01:00-03:00 |

**Strategy for global reach:** Post at 9:00 AM ET (catches US East + late European afternoon) and again at 12:00 PM ET (catches US West morning + US East lunch). Two posts cover the largest developer populations.

### 1.4 Best Days of Week

Ranked by engagement data from Buffer (8.7M tweets) and Sprout Social (2.7B engagements):

| Rank | Day | Relative Engagement | Notes |
|------|-----|-------------------|-------|
| 1 | **Wednesday** | Highest | 9 AM is the single best slot across all studies |
| 2 | **Tuesday** | Very high | Runner-up. 8-10 AM window strong for clicks and retweets |
| 3 | **Thursday** | High | Particularly good for B2B — teams planning next week |
| 4 | **Monday** | Moderate-high | "Back-to-work" scroll at 8-10 AM. Good for announcements |
| 5 | **Friday** | Moderate | Morning only. Afternoon engagement drops sharply |
| 6 | **Sunday** | Low overall, niche high | Developer threads at 7-9 PM perform surprisingly well |
| 7 | **Saturday** | Lowest | Only casual/experimental content. 10 AM or 4 PM if needed |

**Summary:** Tuesday-Thursday is the power zone. Monday is solid for announcements. Friday morning only. Weekends are experimental territory except Sunday evening for dev threads.

### 1.5 Weekend vs Weekday Performance

Weekday engagement is 22% higher than weekends on average (Buffer). However, this varies by content type:

| Content Type | Weekday Performance | Weekend Performance |
|-------------|--------------------|--------------------|
| Product announcements | Strong (Tue-Thu) | Weak — gets buried |
| Technical tutorials | Strong (Tue-Thu morning) | Moderate Sunday evening |
| Build-in-public updates | Strong (Mon-Wed morning) | Moderate Sunday PM |
| Casual/personality posts | Moderate | Better relative performance |
| Dev threads (deep technical) | Good | Sunday evening outperforms |

**Recommendation:** Reduce weekend posting to 0-1 posts per day. Reserve weekends for experimental content or Sunday evening dev threads. Never post product announcements on weekends.

---

## 2. Posting Frequency

### 2.1 Optimal Posts Per Day

Research consensus across Buffer, Hootsuite, Sprout Social, and Tweet Archivist:

| Posts/Day | Effect | Source |
|-----------|--------|--------|
| 1 | Baseline. Consistent presence without fatigue | All sources agree |
| 2-3 | **Sweet spot for business accounts.** Maximum engagement per post | Hootsuite, Buffer |
| 3-5 | Acceptable for tech brands with diverse content | Sprout Social |
| 5-10 | Viable only with dedicated social team + high-quality content | Hootsuite caveat |
| 10+ | **Negative returns.** Self-competition, audience fatigue, unfollows | Tweet Archivist |

**CruxDev config:** `max_posts_per_day = 3` in growth.toml. This is correct and research-backed.

### 2.2 Diminishing Returns Threshold

The data is clear on where returns diminish:

- **Posts 1-3:** Each additional post adds meaningful incremental reach
- **Posts 4-5:** Diminishing returns begin — each post gets less engagement than the previous
- **Posts 6-10:** Self-competition kicks in — your new tweet pushes your previous tweet down the feed, and the algorithm spends less ranking effort on each post
- **Posts 10+:** Negative returns — looks spammy, triggers unfollows, algorithm may deprioritize

**The self-competition problem:** When you post multiple times rapidly, each new tweet competes with your previous tweet for timeline space. The algorithm distributes less ranking power to each individual post. Three well-spaced posts outperform six clustered posts every time.

### 2.3 Thread vs Single Post Timing

Threads and single posts serve different purposes and have different timing rules:

| Format | Best Time to Post | Why |
|--------|------------------|-----|
| **Thread (4-8 posts)** | 9:00-10:00 AM ET weekday, or Sunday 7-9 PM ET | Accumulates engagement throughout the day. Morning threads get shared during work hours |
| **Single post (announcement)** | 9:00-11:00 AM ET Tue-Thu | Quick hit, needs immediate engagement burst |
| **Single post (link to blog)** | 12:00-1:00 PM ET | Lunch break = time to click through and read |
| **Single post (hot take/opinion)** | 3:00-5:00 PM ET | Afternoon wind-down, people want quick commentary |

**Thread-specific rules:**
- Threads get 40-60% more total impressions than the equivalent number of standalone posts on the same topic
- Threads are 60% more likely to generate profile visits
- Optimal thread length: 4-8 posts (completion drops sharply after post 7)
- Post the full thread at once (not staggered) — Typefully handles this

### 2.4 Spacing Between Posts

Minimum spacing between separate posts (not thread posts):

| Spacing | Effect |
|---------|--------|
| < 30 min | **Bad.** Posts compete directly. Second post kills first post's momentum |
| 30-60 min | Risky. Only if content types are very different |
| **2-4 hours** | **Optimal.** Each post gets its own engagement window |
| 4-6 hours | Good. Hits different timezone audiences |
| 8+ hours | Fine but may miss the daily window overlap |

**CruxDev rule:** Minimum 2 hours between posts. 3-4 hours preferred. This means with max 3 posts/day, a typical schedule is: 9 AM, 12 PM, 4 PM ET.

---

## 3. Content Type Timing

Different content types perform best at different times. This maps directly to CruxDev's content pipeline event types.

### 3.1 Feature Announcements (Product Launches)

**When:** Tuesday-Thursday, 9:00-10:00 AM ET
**Why:** Peak attention, workday just started, maximum first-hour engagement window
**Format:** Thread (4-6 posts) with code example or screenshot
**Queue priority:** HIGH — these should get the best slot of the day

```
EventType::FeatureShipped → Slot: Tue-Thu 9:00 AM ET
EventType::ProductLaunch  → Slot: Tue-Wed 9:00 AM ET (never Friday)
```

### 3.2 Technical Content (Tutorials, Patterns, Stack Docs)

**When:** Tuesday-Thursday, 10:00-11:00 AM ET or 12:00-1:00 PM ET
**Why:** Engineers in work mode, willing to engage with technical depth
**Format:** Thread for tutorials, single post with link for blog posts
**Queue priority:** MEDIUM — these are evergreen, can wait for a good slot

```
EventType::MethodologyDoc → Slot: Tue-Thu 10:00 AM or 12:00 PM ET
```

### 3.3 Build-in-Public Updates

**When:** Monday-Wednesday, 9:00-10:00 AM ET (start of week energy) or 4:00-5:00 PM ET (end of day reflection)
**Why:** BIP content thrives on momentum narrative — "here's what we shipped today"
**Format:** Single post with metrics (test count, findings closed, tools added)
**Queue priority:** MEDIUM — consistent cadence matters more than perfect timing

```
EventType::GapClosed       → Slot: Mon-Wed 9:00 AM or 4:00 PM ET
EventType::MilestoneReached → Slot: Tue-Thu 9:00 AM ET (treat as announcement)
```

### 3.4 Competitive Analysis

**When:** Tuesday-Thursday, 10:00 AM-2:00 PM ET
**Why:** Business hours when decision-makers are evaluating tools. This is B2B content.
**Format:** Thread comparing approaches, or single post linking to vs/ page
**Queue priority:** MEDIUM — time to when competitor news is fresh matters

```
EventType::CompetitorDiscovered → Slot: Tue-Thu 10:00 AM-2:00 PM ET
```

### 3.5 Milestone Celebrations

**When:** Tuesday-Wednesday, 9:00-10:00 AM ET
**Why:** Maximum eyeballs. Milestones deserve peak slots.
**Format:** Single post with specific numbers. "314 tests. 55 MCP tools. 100% coverage. Zero clippy warnings."
**Queue priority:** HIGH — these are rare and high-impact

```
EventType::MilestoneReached → Slot: Tue-Wed 9:00 AM ET
```

### 3.6 Release Notes / Changelogs

**When:** Wednesday-Thursday, 9:00-11:00 AM ET
**Why:** Mid-week is when developers are most likely to try new versions
**Format:** Thread with key changes + link to GitHub release
**Queue priority:** HIGH — time-sensitive, should post within 24h of release

```
EventType::ReleasePublished → Slot: Wed-Thu 9:00-11:00 AM ET
```

### 3.7 Bug Fixes / Refactors

**When:** Any weekday, any available slot (low priority)
**Why:** These are hygiene posts — they show the project is alive but don't drive engagement
**Format:** Single post, brief
**Queue priority:** LOW — fill empty slots, never displace higher-priority content

```
EventType::BugFix   → Slot: any available weekday slot
EventType::Refactor → Slot: any available weekday slot (or skip)
```

---

## 4. Scheduling Strategy for the BIP Pipeline

### 4.1 The Burst Problem

On a productive day, the BIP pipeline may generate 10-30 draft posts (e.g., 27 stack pattern announcements in one session). Posting them all at once would be catastrophic:

- Self-competition destroys per-post engagement
- Looks like a bot flooding the timeline
- Followers unfollow spam accounts
- Algorithm deprioritizes accounts that burst-post
- First-hour engagement on each post is zero because the next post immediately buries it

**Rule: Never post more than 3 posts in a single day, regardless of how many drafts exist.**

### 4.2 Queue Architecture

The BIP pipeline must implement a FIFO queue with priority scheduling:

```
┌─────────────────────────────────────────┐
│            Draft Generation             │
│  (BIP pipeline produces N drafts)       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│           Priority Queue                │
│  HIGH: features, milestones, releases   │
│  MEDIUM: patterns, BIP updates, comp    │
│  LOW: bug fixes, refactors              │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Daily Slot Allocator            │
│  Slot 1: 9:00 AM ET  (best content)    │
│  Slot 2: 12:00 PM ET (second best)     │
│  Slot 3: 4:00 PM ET  (third/BIP)       │
│  Max 3 per day. Min 2h spacing.         │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Typefully API                   │
│  Schedule via schedule_date parameter   │
│  social_set_id = 288244                 │
└─────────────────────────────────────────┘
```

### 4.3 Daily Slot Allocation

Three slots per day, Tuesday through Friday. Reduced schedule on Monday and weekends.

| Day | Slot 1 (9:00 AM ET) | Slot 2 (12:00 PM ET) | Slot 3 (4:00 PM ET) |
|-----|---------------------|----------------------|---------------------|
| Monday | BIP update or announcement | Technical content | -- (2 max) |
| **Tuesday** | **Highest-priority draft** | Technical/competitive | BIP update |
| **Wednesday** | **Highest-priority draft** | Technical/competitive | BIP update |
| **Thursday** | Announcement or release | Technical/competitive | BIP update |
| Friday | Technical content | -- | -- (1 max) |
| Saturday | -- | -- | -- (0 posts) |
| Sunday | -- | -- | Dev thread 7-9 PM (0-1 max) |

**Weekly cadence:** 12-15 posts per week at maximum. 8-10 is more sustainable.

### 4.4 Handling Burst Days (27 Drafts Generated)

When the pipeline generates a large batch (like 27 stack pattern announcements):

**Step 1: Prioritize.** Score each draft:
- Feature shipped / milestone → HIGH (post within 1-2 days)
- New stack pattern → MEDIUM (can wait 3-7 days)
- Bug fix / refactor → LOW (can wait 7-14 days or skip)

**Step 2: Deduplicate.** If 18 stack patterns shipped in one session, do NOT post 18 separate announcements. Instead:
- Post 1 summary thread: "18 stack patterns shipped in one week. Here's the full list."
- Pick 3-4 most interesting for individual posts over the following week
- The rest get a mention in a weekly roundup thread

**Step 3: Spread across 7-10 days.**

Example schedule for 27 drafts generated on a Saturday:

| Day | Posts | Content |
|-----|-------|---------|
| Sunday | 1 | Summary thread: "Shipped 18 stack patterns this week" (7 PM ET) |
| Monday | 2 | Best individual pattern + BIP metrics update |
| Tuesday | 3 | Second-best pattern + competitive angle + BIP update |
| Wednesday | 3 | Third pattern + technical deep-dive + milestone |
| Thursday | 3 | Fourth pattern + release notes + BIP update |
| Friday | 1 | Weekly roundup or "what's next" preview |
| Sat-Sun | 0-1 | Rest or Sunday evening thread |

**Total: 13-14 posts across 7 days** from 27 drafts. The remaining 13 drafts are either consolidated into the summary thread, saved for next week, or deprioritized.

### 4.5 Content Type Variety Per Day

Never post the same content type twice in one day. Vary across the three slots:

**Good day:**
- 9 AM: Feature announcement (thread)
- 12 PM: Technical tutorial link
- 4 PM: Build-in-public metrics update

**Bad day:**
- 9 AM: Stack pattern announcement
- 12 PM: Stack pattern announcement
- 4 PM: Stack pattern announcement

**Rule: Each daily slot must be a different content type.** The slot allocator should enforce this by checking the content type of already-scheduled posts for that day.

### 4.6 Weekend Strategy

Weekends are rest days for the queue, not zero days:

- **Saturday:** 0 posts. Let the timeline breathe. Exception: urgent product fix announcement.
- **Sunday:** 0-1 posts. Sunday evening (7-9 PM ET) is a premium dev thread slot. Use it for deep technical content or weekly previews.

This creates a rhythm followers can predict: consistent weekday presence, quiet weekends, Sunday evening deep-dive.

---

## 5. Engagement Optimization

### 5.1 The First-Hour Rule

The single most important factor for post reach on X:

> **Engagement within the first 30 minutes is the biggest predictor of long-tail reach.**

The algorithm measures engagement velocity — how fast a post accumulates likes, replies, retweets, and bookmarks — not total engagement. A post with 100 likes in 10 minutes outperforms a post with 500 likes over 3 days.

**Implications for scheduling:**
- Post when your audience is most active (the windows above)
- Never post at 2 AM hoping for "less competition" — there is no competition advantage with zero audience
- The first 2 hours are critical — everything after that is algorithmic amplification of early signal

### 5.2 Maximizing First-Hour Engagement

Tactics that boost engagement velocity:

| Tactic | Effect |
|--------|--------|
| **Strong hook** (see X_POST_PATTERNS.md) | Stops the scroll, gets first engagement faster |
| **Post at peak time** | More eyeballs = faster engagement accumulation |
| **Include a question** | Drives replies, which the algorithm weights heavily |
| **Reply to your own post** | Adds context, bumps visibility in followers' feeds |
| **Bookmark bait** | "Bookmark this for later" — bookmarks are a strong signal |

### 5.3 Thread vs Single Post Decision Matrix

| Scenario | Format | Reason |
|----------|--------|--------|
| Product launch with multiple features | Thread (5-7 posts) | Each post is sharable individually + full narrative |
| Single benchmark result | Single post | Quick, punchy, shareable |
| "How we built X" | Thread (6-8 posts) | Story arc needs multiple beats |
| Link to blog post | Single post | Hook + link. Don't make people read a thread AND an article |
| Hot take / opinion | Single post | Quick engagement, conversation starter |
| Weekly roundup | Thread (4-5 posts) | Structured summary of the week |
| Competitive comparison | Thread (4-6 posts) | Needs setup, evidence, conclusion |
| Bug fix / patch note | Single post | Brief, low-ceremony |

### 5.4 Hashtag Strategy

Research consensus (2025-2026): less is more.

| Hashtag Count | Engagement Effect |
|---------------|------------------|
| 0 | Baseline. Fine for established accounts |
| **1-2** | **+21-55% engagement** vs 3+ hashtags |
| 3 | Diminishing returns begin |
| 4-5 | Looks promotional, engagement drops |
| 6+ | Spam signal. Algorithm may deprioritize |

**CruxDev hashtag rules:**
- Maximum 2 hashtags per post
- Always at the end, never mid-sentence
- Use specific community tags: `#rustlang`, `#devtools`, `#opensource`, `#buildinpublic`
- Avoid generic: `#coding`, `#programming`, `#tech`, `#AI`
- Rotate hashtags — don't use the same pair on every post

**Recommended hashtag pairs by content type:**

| Content Type | Hashtags |
|-------------|----------|
| Rust/systems content | `#rustlang` |
| DevTools / product | `#devtools` `#opensource` |
| Build-in-public | `#buildinpublic` |
| Stack patterns | `#webdev` or stack-specific (`#nextjs`, `#django`, etc.) |
| Competitive analysis | None — comparison content doesn't need hashtags |
| Milestones | `#opensource` `#buildinpublic` |

### 5.5 Media and Image Impact

The data on media is nuanced and has shifted in 2025-2026:

| Format | Median Engagement | Notes |
|--------|------------------|-------|
| **Text-only** | Highest median | Buffer 2026: text-only beats all other formats in median engagement on X |
| **Image** | High | 2.8x more engagement than text-only in some studies, but median is lower |
| **GIF** | Moderate-high | +55% engagement boost. Good for demos |
| **Video** | Variable | 2-3x impression priority from algorithm, but lower median engagement |
| **Link** | Lowest | Algorithm deprioritizes external links. Post insight first, link in reply |

**Key insight:** The "text is king" finding from Buffer's 8.7M tweet analysis means well-crafted text posts outperform lazy image posts. But a strong text hook + relevant image/GIF still wins overall. The takeaway is: don't add media just to add media. Add it when it strengthens the content.

**CruxDev media rules:**
- Terminal screenshots for benchmark results and test output
- Code snippets as text (accessible, copyable) for 1-5 lines
- Animated GIFs (<30 sec) for workflow demonstrations
- Architecture diagrams for system design posts
- Never add a stock photo. Developers see through it instantly.

### 5.6 Link Placement Strategy

X's algorithm deprioritizes posts with external links in the main body. Research-backed approach:

1. **Main post:** Hook + key insight (no link)
2. **Self-reply:** "Link: [url]" or "Full blog post: [url]"

This two-step pattern gets 2-3x more impressions than putting the link in the main post, because:
- The main post gets full algorithmic distribution
- The reply shows up attached to the main post for anyone who sees it
- Replies with links don't penalize the parent post's reach

---

## 6. Anti-Patterns

### 6.1 Content Anti-Patterns

| Anti-Pattern | Why It Fails | What To Do Instead |
|-------------|-------------|-------------------|
| **Same content type all day** | Audience fatigue, looks automated | Vary: announcement, tutorial, BIP update |
| **Posting at midnight** | Zero audience unless targeting APAC | Post at 9 AM ET or schedule for next morning |
| **5+ posts per day** | Looks spammy, self-competition, unfollows | Max 3 posts/day, well-spaced |
| **No spacing between posts** | Each post buries the previous one | Minimum 2 hours between posts |
| **Identical messages across days** | Algorithm detects and suppresses duplicates | Rephrase, different angle, new hook |
| **Link in main post body** | Algorithm deprioritizes external links | Link in self-reply |
| **"Excited to announce..."** | Generic opener, zero scroll-stopping power | Lead with the result or metric |
| **3+ hashtags** | Looks promotional, engagement drops | Max 2, specific community tags only |

### 6.2 Scheduling Anti-Patterns

| Anti-Pattern | Why It Fails | What To Do Instead |
|-------------|-------------|-------------------|
| **Posting all drafts at once** | Self-competition, spam appearance, unfollows | Queue with 3/day max, spread across days |
| **Inconsistent cadence** | 10 posts Monday, 0 posts Tue-Fri. Algorithm and followers confused | Consistent daily presence (1-3/day) |
| **Only posting on "best" days** | Missing Monday/Friday audience entirely | Post every weekday, weight toward Tue-Thu |
| **Ignoring queue during crisis** | Scheduled posts during outage or bad news = tone-deaf | Pause queue, address situation first |
| **Weekend blasts** | Low engagement, wasted content | Save good content for weekdays |
| **Never posting on weekends** | Missing Sunday evening dev thread opportunity | 0-1 posts, Sunday PM for deep threads |

### 6.3 Automation Anti-Patterns

| Anti-Pattern | Why It Fails | What To Do Instead |
|-------------|-------------|-------------------|
| **Bulk posting across accounts** | X API violation, ban risk | One account, one queue |
| **Identical cross-posting** | Detected as spam by platform | Customize per platform |
| **No human review of queue** | Tone-deaf posts during inappropriate moments | Weekly queue review |
| **Posting during major news events** | Scheduled marketing post during tragedy = PR disaster | Pause queue, review before resuming |

---

## 7. The Algorithm: What Matters in 2025-2026

Understanding X's ranking signals helps optimize posting strategy:

### 7.1 Engagement Velocity Signals (Ranked by Weight)

| Signal | Weight | Notes |
|--------|--------|-------|
| **Replies** | Highest | Conversation signals relevance. Ask questions to drive replies |
| **Retweets/Reposts** | Very high | Distribution signal — your content reaches new audiences |
| **Bookmarks** | High | Strong "save for later" intent signal, growing in weight |
| **Likes** | Moderate | Basic approval signal, easiest to get |
| **Dwell time** | Moderate | How long someone stops scrolling to read your post |
| **Profile clicks** | Moderate | Signals curiosity about the author |
| **Link clicks** | Low-moderate | Valuable but algorithm prefers on-platform engagement |

### 7.2 Account Health Signals

| Signal | Impact |
|--------|--------|
| **Consistent posting cadence** | Algorithm learns when to distribute your content |
| **Reply-to-post ratio** | Accounts that only post and never reply get deprioritized |
| **Follower engagement rate** | High % of followers engaging = more distribution |
| **Account age + verification** | Older, verified accounts get baseline trust |

### 7.3 Negative Signals

| Signal | Impact |
|--------|--------|
| **Burst posting** | Multiple posts in short window = deprioritization |
| **High unfollow rate** | Content is annoying audience = less distribution |
| **Link-heavy posts** | Algorithm prefers on-platform content |
| **Duplicate content** | Same text repeated = suppression |

---

## 8. Concrete Recommended Schedule for CruxDev

### 8.1 Standard Week Schedule

This is the default schedule for the BIP pipeline when the queue has mixed content:

```
MONDAY (2 posts)
  09:00 AM ET — BIP update or week-opening announcement
  12:00 PM ET — Technical content (pattern doc, tutorial link)

TUESDAY (3 posts)
  09:00 AM ET — Highest-priority: feature announcement or milestone
  12:00 PM ET — Technical deep-dive or competitive comparison
  04:00 PM ET — Build-in-public update with metrics

WEDNESDAY (3 posts)
  09:00 AM ET — Highest-priority: feature announcement or release
  12:00 PM ET — Stack pattern announcement or methodology post
  04:00 PM ET — Build-in-public update with metrics

THURSDAY (3 posts)
  09:00 AM ET — Feature or release announcement
  12:00 PM ET — Competitive analysis or comparison thread
  04:00 PM ET — Build-in-public update or "how we built X"

FRIDAY (1 post)
  09:00 AM ET — Weekly roundup thread or "what's coming next" preview

SATURDAY (0 posts)
  Queue rests.

SUNDAY (0-1 posts)
  07:00 PM ET — Deep technical thread (optional, high-impact slot for devs)
```

**Weekly total:** 12-13 posts. Sustainable, consistent, diverse.

### 8.2 Content Type Mapping to Slots

| Slot | Best Content Types | Worst Content Types |
|------|-------------------|---------------------|
| **9 AM ET** (prime) | Feature launches, milestones, releases | Bug fixes, refactors |
| **12 PM ET** (lunch) | Technical tutorials, blog links, comparisons | Raw metrics updates |
| **4 PM ET** (wind-down) | BIP updates, hot takes, "today we shipped" | Long threads (people are leaving work) |
| **7 PM ET Sun** (dev evening) | Deep technical threads, weekly previews | Product announcements (low reach) |

### 8.3 Feature Announcement Playbook

When a significant feature ships:

1. **Day of ship (or next business day):** 9:00 AM ET — Thread (5-7 posts) with hook, problem, solution, code example, results, CTA
2. **Day after:** 12:00 PM ET — Single post with different angle (benchmark, comparison, or user quote)
3. **3 days later:** 4:00 PM ET — "Here's what we learned building X" BIP post
4. **1 week later:** Include in Friday roundup thread

### 8.4 Stack Pattern Announcement Playbook

When new stack patterns are converged (e.g., BP069 Nuxt + BP072 GoTH):

1. **Group related patterns:** "Shipped 2 new stack patterns: Nuxt.js (2,536 lines) + GoTH (2,695 lines)"
2. **Post one summary per batch** — not one post per pattern
3. **Schedule at 12 PM ET** (technical content slot)
4. **Pick the most interesting pattern for a standalone deep-dive thread** later in the week
5. **If 10+ patterns ship at once:** Summary thread at 9 AM ET, individual highlights spread over 5-7 days

### 8.5 Competitive Analysis Playbook

When a new competitor is discovered or analysis is updated:

1. **Schedule at 10 AM-2 PM ET, Tuesday-Thursday** (B2B decision-maker hours)
2. **Format as thread:** "We compared CruxDev to [competitor]. Here's what we found."
3. **Lead with the insight, not the product name**
4. **Link to vs/ comparison page in self-reply**
5. **Never post competitive content on weekends** — decision-makers aren't evaluating tools

### 8.6 Build-in-Public Update Playbook

Daily or every-other-day BIP posts:

1. **Default slot:** 4:00 PM ET (end-of-day "here's what we built")
2. **Alternative slot:** 9:00 AM ET Monday (week-opening "here's what we shipped last week")
3. **Format:** Single post with specific numbers: tests, tools, findings closed, lines of code
4. **Vary the framing:** Don't repeat "Shipped X. Y tests passing." every time. Rotate between:
   - Metric-forward: "314 tests. 55 MCP tools. 100% coverage."
   - Problem-forward: "Audit found 12 findings in the new module. All 12 closed."
   - Narrative-forward: "Day 47 of building CruxDev in public. Today: [specific thing]."

### 8.7 Burst Day Protocol

When the pipeline generates 10+ drafts in a single session:

```
IF drafts_generated > 10:
    1. Create ONE summary thread covering the batch
       → Schedule for next available 9 AM ET slot (Tue-Thu preferred)
    2. Pick top 3-4 most interesting individual items
       → Schedule as individual posts over the next 5-7 days
    3. Remaining items:
       → Include in weekly roundup thread
       → OR save for next week's queue
       → OR consolidate into a blog post (higher-value format)
    4. NEVER exceed 3 posts/day regardless of queue depth
    5. Estimate drain time: queue_size / 3 posts_per_day = days to drain
       → If > 14 days, aggressively consolidate
```

**Example: 27 stack pattern drafts**

```
Day 1 (Sunday):   1 post  — Summary thread: "18 stack patterns converged"
Day 2 (Monday):   2 posts — Best pattern deep-dive + BIP metrics
Day 3 (Tuesday):  3 posts — Second pattern + competitive angle + BIP
Day 4 (Wednesday):3 posts — Third pattern + technical tutorial + BIP
Day 5 (Thursday): 3 posts — Fourth pattern + release notes + BIP
Day 6 (Friday):   1 post  — Weekly roundup
Day 7-8 (Sat-Sun):0-1 post
                  ─────────
                  13-14 posts from 27 drafts over 8 days
                  Remaining 13 drafts → consolidated or next week
```

---

## 9. Implementation Notes for the Rust Engine

### 9.1 Queue State Schema

The BIP pipeline's `BipState` struct should be extended to support scheduling:

```rust
pub struct PostSlot {
    pub day: Weekday,       // Mon-Sun
    pub time_et: String,    // "09:00", "12:00", "16:00"
    pub content_type: ContentCategory,
    pub draft_id: Option<String>,
}

pub enum ContentCategory {
    FeatureAnnouncement,
    TechnicalContent,
    BuildInPublic,
    CompetitiveAnalysis,
    Milestone,
    Release,
    BugFix,
    WeeklyRoundup,
}

pub struct PostQueue {
    pub pending: Vec<QueuedDraft>,
    pub scheduled: Vec<ScheduledPost>,
    pub posts_today: u32,
    pub last_posted_at: Option<DateTime<Utc>>,
}
```

### 9.2 Scheduling Algorithm

```
fn next_available_slot(queue: &PostQueue, draft: &Draft) -> PostSlot:
    1. Get current day and time in ET
    2. If posts_today >= 3, advance to next day
    3. Find next empty slot that matches draft.content_type rules:
       - Feature/Milestone/Release → prefer 9 AM slot
       - Technical → prefer 12 PM slot
       - BIP → prefer 4 PM slot
       - BugFix → any available slot
    4. Ensure no content_type collision with same-day scheduled posts
    5. Ensure minimum 2h gap from last_posted_at
    6. Return slot with schedule_date for Typefully API
```

### 9.3 Typefully API Integration

The `schedule_date` parameter in the Typefully API accepts ISO 8601 timestamps. The engine should:

1. Convert slot time to UTC before sending to API
2. Use `schedule_date` (not `auto_schedule`) for precise control
3. Verify the social_set_id (288244) is correct for the target account
4. Log all scheduled posts to `metrics.jsonl` for performance tracking

### 9.4 Cooldown Enforcement

```rust
pub fn can_post_now(state: &BipState) -> bool {
    let now = Utc::now();
    let last = parse_datetime(&state.last_posted_at);
    let gap_minutes = (now - last).num_minutes();
    gap_minutes >= 120 && state.posts_today < 3
}
```

---

## 10. Measurement and Iteration

### 10.1 Key Metrics to Track Per Post

| Metric | Source | Why |
|--------|--------|-----|
| Impressions | Typefully analytics / X API | Reach measurement |
| Engagements | Typefully analytics / X API | Total interactions |
| Engagement rate | Calculated: engagements / impressions | Quality signal |
| Link clicks | Typefully analytics | Conversion to website/repo |
| Profile visits | X analytics | Brand awareness signal |
| Follower delta | X analytics | Growth measurement |

### 10.2 A/B Testing Schedule

Test one variable at a time, minimum 2 weeks per test:

| Test | Variable | Control | Variant |
|------|----------|---------|---------|
| Time test | Posting time | 9 AM ET | 10 AM ET |
| Format test | Thread vs single | Thread for features | Single for features |
| Hook test | Opening line | Number hook | Contrarian hook |
| Media test | Image inclusion | Text only | Text + screenshot |
| Hashtag test | Tag count | 0 hashtags | 2 hashtags |

### 10.3 Weekly Review Checklist

Every Friday, the growth cycle should evaluate:

- [ ] Total posts this week vs target (12-13)
- [ ] Average engagement rate vs previous week
- [ ] Best-performing post (time, format, content type) — schedule more like it
- [ ] Worst-performing post — diagnose why (bad timing? weak hook? wrong content type?)
- [ ] Queue depth — how many days until drained?
- [ ] Content type distribution — balanced across categories?
- [ ] Any posts that should have been consolidated?

---

## 11. Quick Reference Card

### The 3-3-2 Rule

- **3** posts per day maximum
- **3** hours minimum spacing between posts
- **2** hashtags maximum per post

### The Best Slots

| Slot | Time (ET) | Best For |
|------|-----------|----------|
| Prime | 9:00 AM Tue-Thu | Features, milestones, releases |
| Lunch | 12:00 PM Tue-Thu | Tutorials, comparisons, blog links |
| Wind-down | 4:00 PM Mon-Thu | BIP updates, hot takes |
| Dev evening | 7:00 PM Sun | Deep technical threads |

### The Priority Stack

1. Feature announcements and milestones (always get prime slot)
2. Releases and changelogs (prime or lunch slot)
3. Competitive analysis (lunch slot, Tue-Thu)
4. Technical tutorials and patterns (lunch slot)
5. Build-in-public updates (wind-down slot)
6. Bug fixes and refactors (any remaining slot, or skip)

### The Burst Protocol

```
> 10 drafts → summary thread + 3-4 individual highlights over 5-7 days
> 20 drafts → summary thread + consolidate into blog post + 4-5 highlights over 7-10 days
> 30 drafts → something went wrong. Review content generation triggers.
```

---

## Sources

- [Buffer: The Best Time to Post on Twitter/X in 2026 (8.7M Tweets Analyzed)](https://buffer.com/resources/best-time-to-post-on-twitter-x/)
- [Sprout Social: Best Times to Post on Twitter/X in 2025 (2.7B Engagements, 470K Profiles)](https://sproutsocial.com/insights/best-times-to-post-on-twitter/)
- [Sprout Social: Best Times to Post on Social Media in 2025](https://sproutsocial.com/insights/best-times-to-post-on-social-media/)
- [Hootsuite: How Often to Post on Social Media (2025)](https://blog.hootsuite.com/how-often-to-post-on-social-media/)
- [Hootsuite: Best Time to Post on Social Media (2025)](https://blog.hootsuite.com/best-time-to-post-on-social-media/)
- [SocialPilot: Best Time to Post on Twitter/X (50K+ Accounts)](https://www.socialpilot.co/insights/best-time-to-post-on-twitter)
- [PostEverywhere: Best Time to Post on X (700K Posts Analyzed)](https://posteverywhere.ai/blog/best-time-to-schedule-x-posts)
- [Tweet Archivist: Twitter Posting Frequency Guide 2026](https://www.tweetarchivist.com/twitter-posting-frequency-guide-2025)
- [Tweet Archivist: How the Twitter Algorithm Works (2026)](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025)
- [Buffer: How the Twitter/X Algorithm Works (2025)](https://buffer.com/library/twitter-timeline-algorithm/)
- [SocialBee: Understanding the X Algorithm (2026)](https://socialbee.com/blog/twitter-algorithm/)
- [Distribution.ai: Best Time to Post by Industry (2026)](https://www.distribution.ai/blog/best-time-to-post-on-twitter)
- [RecurPost: Best Time to Post on Twitter/X (2026)](https://recurpost.com/blog/best-time-to-post-on-twitter/)
- [Buffer: Best Content Format on Social Platforms (45M+ Posts)](https://buffer.com/resources/data-best-content-format-social-media/)
- [Sprout Social: How the Twitter Algorithm Works (2026)](https://sproutsocial.com/insights/twitter-algorithm/)
- [PostNext: X Algorithm Explained (2025)](https://postnext.io/blog/x-twitter-algorithm-explained/)
- [Hashtag Tools: Optimal Hashtag Count on X (2026)](https://hashtagtools.io/blog/x-twitter-hashtag-trending-guide)
