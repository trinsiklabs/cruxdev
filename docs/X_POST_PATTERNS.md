# X/Twitter Post Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Developer tool X/Twitter analysis, engagement data, thread format studies
**Last updated:** 2026-03-27

## 1. Hook Writing

The hook is everything. Developers scroll fast — 1.5 seconds to stop them.

**Patterns that work:**

| Pattern | Example |
|---------|---------|
| **Number hook** | "We reduced build times from 47 min to 90 sec. Here's how." |
| **Contrarian** | "Most CI/CD pipelines are solving the wrong problem." |
| **Before/After** | "Before: 200 lines of retry logic. After: 3 lines." |
| **Problem** | "Every code review, I waste 40 min on the same 5 issues." |
| **Result** | "451 tests. Zero clippy warnings. 5.1MB binary. Shipped." |

**Patterns that fail:** "Excited to announce...", "Check out our new...", starting with product name.

## 2. Thread vs Single Post

| Use Single Post | Use Thread (4-8 posts) |
|-----------------|----------------------|
| Quick result, benchmark | Launch story, "how we built X" |
| Link to blog post | Tutorial walkthrough |
| Release announcement | Lessons learned |
| Hot take | Deep explanation |

Threads over 10 posts lose readers — completion drops sharply after post 7.

## 3. Thread Structure (6-Post Formula)

| Post | Content | Purpose |
|------|---------|---------|
| 1 | Hook + bold claim/result | Stop the scroll |
| 2 | The problem / context | Why this matters |
| 3 | The insight / approach | What's different |
| 4 | Code or demo | Show, don't tell |
| 5 | Results / data | Proof |
| 6 | CTA + link | Try it, star it |

**Rules:**
- Each post must stand alone (Twitter shows individual posts in timelines)
- Link/CTA in the LAST post, not the first
- At least one visual (screenshot, GIF, diagram) in the thread

## 4. Presenting Numbers

- Specific, not vague. "3.5x faster" beats "significantly faster."
- Comparison format: "Before: X. After: Y."
- Line breaks for readability
- Avoid percentages without absolutes. "80% faster" → "from 500ms to 100ms"

## 5. Code in Posts

- Text code blocks for 1-5 lines (accessible, copyable)
- Terminal screenshots for colorized/complex output
- Max 4-6 lines per post
- Animated GIFs (<30 sec) outperform static screenshots

## 6. Hashtags

- 0-2 maximum. More = spam.
- Useful: #rustlang, #python, #devtools, #opensource
- Useless: #coding, #programming, #tech
- At the end, never mid-sentence.

## 7. Timing

- Best: Tuesday-Thursday, 9-11 AM ET
- Also good: Tue-Thu 12-1 PM ET (lunch scroll)
- Worst: Friday afternoon, weekends
- Threads: post morning (9-10 AM ET) to accumulate during workday

## 8. Anti-Patterns

| Don't | Do Instead |
|-------|-----------|
| "Just shipped!" with no context | Lead with the result or problem solved |
| Link with no hook | Give key insight FIRST, then link |
| 3+ emojis per post | 0-2, developer Twitter is low-emoji |
| "Like and retweet!" | "Bookmark if useful" (reason-based) |
| Same content posted repeatedly | Rephrase, different angle |
| Thread posts under 80 chars | Merge with next post |
| Text-only threads | Include at least one visual |

## 9. Audit Dimensions

1. **hook** — first line creates curiosity (number, contrarian, problem, result)
2. **structure** — thread follows 6-post formula, each post stands alone
3. **data** — specific numbers, before/after comparisons
4. **visual** — at least one image/GIF/screenshot
5. **cta** — link in last post, not first
6. **timing** — scheduled for peak engagement window
