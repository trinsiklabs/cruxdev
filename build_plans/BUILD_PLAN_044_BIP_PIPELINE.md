# BUILD_PLAN_044: Build-In-Public Pipeline — Blog + X Posts from Build Plans

**Status:** NOT STARTED
**Priority:** High
**Depends on:** BP022 (growth engine), BP018 (competitive feedback loop)

## Context

Every build plan convergence, every issue resolved, every new competitor discovered is content. Currently this content is generated but goes nowhere — changelog entries and X posts saved to `.cruxdev/evolution/posts/` but never published. BIP (build-in-public) from Crux's unused implementation provides the trigger system. CruxDev needs the full pipeline: event → classify → generate → publish.

## Content Triggers

| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Build plan converged (new feature) | YES | YES | "How we built X" / feature announcement |
| New competitor discovered | YES | YES | Competitive analysis + comparison |
| Competitive gap closed | YES | YES | "We now match X on Y" |
| GitHub issue resolved | CANDIDATE | YES | "Fixed: [title]" acknowledgment |
| New integration | YES | YES | Integration announcement |
| New methodology doc | YES | YES | Thought leadership / deep dive |
| Bug fix (minor) | NO | MAYBE | Only if publicly reported |
| Internal refactor | NO | NO | Nobody cares |

## Phase 1: Deep Research — Pattern Docs

### 1.1 BLOG_POST_PATTERNS.md
- [ ] 5-pass research on developer blog best practices
- [ ] Structure, length, SEO optimization, code examples
- [ ] Technical blog vs announcement vs comparison vs tutorial
- [ ] Publishing cadence, distribution channels
- [ ] Audit dimensions for blog convergence

### 1.2 X_POST_PATTERNS.md
- [ ] 5-pass research on X/Twitter best practices for developer tools
- [ ] Thread structure, hook writing, engagement patterns
- [ ] Technical content formatting (code snippets, metrics, links)
- [ ] Posting cadence, timing, hashtags
- [ ] Audit dimensions for X post convergence

## Phase 2: Port BIP Trigger System from Crux

- [ ] 2.1 Port `bip/triggers.rs` — event classification, threshold evaluation
- [ ] 2.2 Port `bip/config.rs` — trigger config in growth.toml
- [ ] 2.3 Port `bip/gather.rs` — gather recent events for content candidates
- [ ] 2.4 Extend triggers beyond commit count: build plan type, issue type, competitive event type
- [ ] 2.5 Remove BIP from Crux (or deprecate with pointer to CruxDev)

## Phase 3: Blog Post Generation

- [ ] 3.1 Template per event type:
  - Feature announcement: title, what it does, why it matters, how it works, numbers
  - Competitive analysis: who, what changed, how we compare, what's next
  - Issue resolution: what was reported, what we found, how we fixed it
  - Methodology deep-dive: the research, the findings, how it's integrated
- [ ] 3.2 Generate to website blog directory
- [ ] 3.3 Auto-commit + deploy (via growth engine)
- [ ] 3.4 Blog post links back to build plan on GitHub

## Phase 4: X Post Generation

- [ ] 4.1 Template per event type:
  - Feature: hook → what → numbers → link
  - Competitor: hook → comparison → differentiator → link
  - Issue fix: "Fixed: [title]" → brief → link to issue
- [ ] 4.2 Post via Typefully (when API auth is resolved)
- [ ] 4.3 Thread format for features (1/ hook, 2/ problem, 3/ solution, 4/ numbers, 5/ link)

## Phase 5: Wire Into Convergence Lifecycle

- [ ] 5.1 After build plan CONVERGED → classify event → generate blog + X post
- [ ] 5.2 After GitHub issue closed → classify → generate if candidate
- [ ] 5.3 After competitive impact detected (BP018) → generate comparison content
- [ ] 5.4 Content goes through quality convergence (blog + X post audit dimensions)
- [ ] 5.5 Then publish (deploy blog, post to Typefully)

## Phase 6: MCP Tools

- [ ] 6.1 `generate_blog_post(event_type, build_plan, project_dir)` — generate from template
- [ ] 6.2 `generate_x_post(event_type, build_plan)` — generate from template
- [ ] 6.3 `publish_content(blog_path, x_post_content, dry_run)` — deploy + post

## Existing Code Impact

- growth.toml needs BIP trigger config section
- Evolution POST beat needs to use BIP pipeline instead of raw file writes
- Website needs blog post pages (currently placeholder "coming soon")

## Verification

```bash
./scripts/convergence_gate.sh
# Plus: blog post generates correctly, X post follows patterns
```
