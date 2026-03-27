# BUILD_PLAN_044: Universal Content Pipeline — Blog + X Posts from Any Project Event

**Status:** IN PROGRESS
**Priority:** High
**Depends on:** BP022 (growth engine), BP018 (competitive feedback loop), BP034 (universal project management)

## Context

Every project produces events that are content. A software feature shipped, a book chapter completed, a podcast episode published, a competitive gap closed — each is a blog post and/or X post waiting to happen. The content trigger system must be universal across all 18 project types, not software-specific.

Currently content is generated but goes nowhere — changelog entries and X posts saved to `.cruxdev/evolution/posts/` but never published. BIP (build-in-public) from Crux's unused implementation provides the trigger system. CruxDev needs the full pipeline: event → classify by project type → generate from type-specific template → audit → publish.

## Universal Content Triggers (All Project Types)

### Software
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Feature shipped (build plan converged) | YES | YES | "How we built X" |
| Competitive gap closed | YES | YES | "We now match X on Y" |
| New competitor discovered | YES | YES | Comparison analysis |
| GitHub issue resolved | CANDIDATE | YES | "Fixed: [title]" |
| New integration | YES | YES | Integration announcement |
| New methodology doc | YES | YES | Thought leadership |
| Bug fix (minor) | NO | MAYBE | Only if publicly reported |

### Book / Book Series
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Chapter completed | YES | YES | Teaser excerpt, writing process |
| Book published / launched | YES | YES | Launch announcement thread |
| New book in series | YES | YES | Series update, what's next |
| Cover reveal | YES | YES | Visual post |
| Milestone (word count, draft done) | NO | YES | Progress update |

### Podcast
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Episode published | YES | YES | Show notes + key takeaways |
| Notable guest booked | YES | YES | Guest preview |
| Season launched | YES | YES | Season announcement |
| Milestone (downloads, episodes) | NO | YES | Metrics post |

### Newsletter
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Issue published | MAYBE | YES | Highlight/teaser |
| Subscriber milestone | NO | YES | Growth update |
| Special edition | YES | YES | Deep content post |

### YouTube
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Video published | YES | YES | Written companion + link |
| Channel milestone | NO | YES | Metrics post |
| Series launched | YES | YES | Series announcement |

### Business
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Product launch | YES | YES | Launch announcement |
| Milestone (revenue, users) | MAYBE | YES | Growth update |
| Partnership | YES | YES | Partnership announcement |

### Open Source
| Event | Blog? | X Post? | Template |
|-------|-------|---------|----------|
| Release published | YES | YES | Release notes + highlights |
| Major contributor joined | MAYBE | YES | Welcome/spotlight |
| Milestone (stars, forks) | NO | YES | Community metrics |

## Content Trigger Registry

The system maps `(project_type, event_type) → content_templates`. Each project type registers its triggers. The engine is generic — content rules are per-type.

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
