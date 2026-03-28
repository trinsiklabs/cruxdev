# BIP Patterns — Blog It, Post It

## The Rule

**If you did it, you blog about it. You post about it.** Every convergence completion triggers BIP. No exceptions.

BIP is not optional. It is a REQUIRED post-convergence action, enforced by the convergence engine. A build plan is not converged until its BIP is generated and queued.

## What BIP Produces

Every convergence completion generates:

1. **Blog post** — published to the project's website (Astro, Phoenix, etc.)
2. **X post** — queued through the X posting engine (rate-limited, scheduled)
3. **Changelog entry** — appended to `.cruxdev/evolution/posts/`

## When BIP Fires

| Event | Blog | X Post | Changelog |
|-------|------|--------|-----------|
| Build plan converged | Required | Required | Required |
| Major bug fixed | Required | Required | Required |
| New pattern doc published | Recommended | Required | Required |
| Competitive analysis updated | Recommended | Recommended | Required |
| Dependency update | Optional | Optional | Required |
| Routine maintenance | No | No | Required |

## Blog Post Requirements

Per BLOG_POST_PATTERNS.md:

### Structure
```markdown
---
layout: ../../layouts/BlogPost.astro
title: "[Action verb] — [What was done]"
date: "USE $(date +%Y-%m-%dT%H:%M) — actual system time, NEVER guess"
slug: "[YYYYMMDD-kebab-case-slug]"
tags: ["relevant", "tags"]
summary: "[1-2 sentence summary with key metrics]"
---

[Opening: what changed and why it matters — 1-2 sentences, no preamble]

## [Section describing what was built/fixed/shipped]

[Technical details, architecture decisions, code examples if relevant]

## Ground Truth

- [Verified metric 1]
- [Verified metric 2]
- [Test count, all passing]
```

### Rules
- **Date MUST use system time**: run `date +%Y-%m-%dT%H:%M` and use that exact output. NEVER guess, estimate, or hardcode a time. The system clock is the source of truth.
- Title starts with action verb (Built, Fixed, Shipped, Added, Converged)
- Summary includes at least one numeric metric
- Ground Truth section verifies claims against reality (GTV)
- No aspirational language — only what IS, not what WILL BE
- Tags include project name and category (milestone, fix, feature, pattern)

## X Post Requirements

Per X_POST_PATTERNS.md:

### Structure
```
[What happened — 1 line]

[Key details — 2-4 bullet points or short lines]

[Link to blog post or project page]
```

### Rules
- Under 280 characters for single tweets
- Threads for complex topics (max 5 tweets)
- Posted through the queue (30-min minimum interval)
- No emojis unless the project brand uses them
- Include a link to the blog post or relevant page
- Technical content preferred over marketing language

## Convergence Engine Integration

### Post-convergence actions in server.rs

The convergence engine's `Converged` phase includes these REQUIRED actions:

```
post_convergence_actions:
  git_commit: REQUIRED
  blog_post: REQUIRED — generate and queue
  x_post: REQUIRED — generate and queue through x_queue
  competitive_impact: REQUIRED
  priority_check: RECOMMENDED
```

### How BIP is enforced

1. **In the convergence engine**: When a build plan reaches `Converged` status, the engine generates content drafts (blog + X post) using the `generate_content` MCP tool.

2. **Blog post generation**: The `generate_content` tool creates a markdown file following BLOG_POST_PATTERNS.md structure. The file is written to the website's blog directory.

3. **X post generation**: The `generate_content` tool creates an X post following X_POST_PATTERNS.md structure. The post is enqueued via `x_queue::enqueue()` — NOT posted directly.

4. **Verification**: The blog post goes through GTV before deploy. The X post goes through the queue's rate limiter.

5. **Deployment**: Blog post is deployed with the next site build. X post is sent when the queue scheduler fires.

### Content generation prompt template

For blog posts:
```
Generate a blog post about: [build plan title]
What was done: [convergence summary]
Key metrics: [test count, files changed, tools added]
Follow BLOG_POST_PATTERNS.md structure exactly.
Include a Ground Truth section with verified metrics.
No aspirational language. Only verified facts.
```

For X posts:
```
Generate an X post about: [build plan title]
Key achievement: [1 sentence]
Metrics: [test count or key number]
Link: [blog post URL]
Under 280 characters.
```

## Adoption Integration

When a project goes through CruxDev adoption:

1. **Step 1**: Adoption process checks if project has a blog/website
2. **Step 2**: If yes, BIP is wired into the convergence post-actions
3. **Step 3**: If no, adoption creates the blog infrastructure (Astro pages, RSS, etc.)
4. **Step 4**: X posting credentials are configured (or flagged for manual setup)
5. **Step 5**: First BIP fires on adoption completion itself ("Project X adopted by CruxDev")

### Adoption checklist addition
- [ ] Blog infrastructure exists (blog directory, layout, index page, RSS)
- [ ] X posting credentials configured (or queued for setup)
- [ ] BIP post-convergence actions verified in project config
- [ ] Test BIP: generate a blog post and X post for the adoption event

## Anti-Patterns

| Anti-Pattern | Why It's Wrong | Correct Approach |
|---|---|---|
| Posting directly to X API | Bypasses rate limiter | Use x_queue::enqueue() |
| Skipping blog for "small" changes | Every convergence gets BIP | At minimum, a changelog entry |
| Writing aspirational content | GTV will catch false claims | Only verified facts |
| Batch-posting multiple X posts | Looks spammy, violates 30-min rule | Queue respects intervals |
| Writing blog post before code ships | Content describes what SHOULD exist | Write after convergence, verify with GTV |
| Manual blog/X posting | Breaks autonomous pipeline | Engine generates, queue posts |

## File Locations

- Blog posts: `[website]/src/pages/blog/YYYYMMDD-slug.md`
- X post queue: `.cruxdev/growth/x_queue.jsonl`
- Content drafts: `.cruxdev/evolution/posts/`
- BIP config: `.cruxdev/growth.toml` (blog_dir, x_credentials section)

## The Standard

If CruxDev converged it, CruxDev tells the story. Blog post + X post. Every time. The engine generates it. The queue schedules it. GTV verifies it. No human in the loop.
