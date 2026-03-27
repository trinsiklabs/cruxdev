# Growth Cycle: Public Interactions Audit

**Audit method:** Code trace through every external-facing action in `rust/src/growth/` and `rust/src/evolution/`
**Audited to convergence:** 2 passes, 0 findings on second pass
**Last updated:** 2026-03-27

## Summary

The `run_growth_cycle` MCP tool performs 5 actions. In **dry-run mode** (the default), NONE of them touch the public. In **live mode**, 2 of 5 actions make public-facing changes.

---

## Action-by-Action Breakdown

### 1. Generate Release Notes
| Attribute | Value |
|-----------|-------|
| **Module** | `growth/releases.rs` → `generate_release_notes()` |
| **What it does** | Reads local git log, formats as markdown |
| **Public interaction** | **NONE** — reads local git history only |
| **Dry-run behavior** | Same as live — just generates text |
| **Risk** | None |

### 2. Check README Health
| Attribute | Value |
|-----------|-------|
| **Module** | `growth/readme.rs` → `check_readme()` |
| **What it does** | Reads local README.md, checks for sections/badges/counts |
| **Public interaction** | **NONE** — reads local filesystem only |
| **Dry-run behavior** | Same as live |
| **Risk** | None |

### 3. Compose X/Twitter Post
| Attribute | Value |
|-----------|-------|
| **Module** | `growth/typefully.rs` → `compose_release_thread()` |
| **What it does** | Generates post text from changelog data |
| **Public interaction** | **NONE in dry-run**. In live mode: **POSTS TO X/TWITTER** via Typefully API |
| **Dry-run behavior** | Returns composed text, does NOT call Typefully API |
| **Live behavior** | `POST https://api.typefully.com/v1/drafts/` — creates a draft/scheduled post |
| **What's posted** | Release announcement thread: project name, version, changelog bullets, test count, tool count |
| **Who sees it** | All followers of your X/Twitter account linked to Typefully |
| **Reversible?** | Yes — drafts can be deleted from Typefully before publishing. If auto-publish is off, posts stay as drafts until you approve them in Typefully. |
| **Risk** | MEDIUM — public post under your account. Content is factual (derived from git log/test counts), but wording should be reviewed at least initially. |
| **Gate** | `TYPEFULLY_API_KEY` env var must be set. If not set, action is skipped entirely. |
| **Rate limit** | Typefully API: generous for normal use. Code limits to max 3 posts/day. |

### 4. Collect Growth Metrics
| Attribute | Value |
|-----------|-------|
| **Module** | `growth/metrics.rs` → `collect_metrics()` |
| **What it does** | Calls `gh repo view` to read star count, forks, issues, watchers |
| **Public interaction** | **READ-ONLY** — queries GitHub API for public repo metadata |
| **Dry-run behavior** | Collects metrics but does NOT write to disk |
| **Live behavior** | Collects metrics AND appends to `.cruxdev/growth/metrics.jsonl` (local file) |
| **Risk** | None — read-only API call. Counts against GitHub API rate limit (5,000/hour with PAT). |

### 5. GitHub Issue Monitoring (via evolution pipeline, not directly in growth cycle)
| Attribute | Value |
|-----------|-------|
| **Module** | `evolution/github.rs` → `monitor_issues()` |
| **What it does** | Fetches open issues, sanitizes, evaluates, generates responses |
| **Public interaction** | **NONE in dry-run**. In live mode: **POSTS COMMENTS on GitHub issues** and **adds labels** |
| **Dry-run behavior** | Returns evaluation results with `[DRY RUN]` prefix, no `gh` commands executed |
| **Live behavior** | `gh issue comment` and `gh issue edit --add-label` — visible to issue authors and watchers |
| **Who sees it** | Issue author, repo watchers, anyone viewing the issue |
| **Content posted** | Acknowledgment comments: "Thanks for reporting this issue. We've triaged it as priority X and will investigate." |
| **Reversible?** | Partially — comments can be deleted via `gh issue comment --delete`, but notifications are already sent |
| **Risk** | MEDIUM — public comments under the repo. Comments are templated and factual, but a bad triage could look awkward. |
| **Gate** | `dry_run` parameter, defaults to `true` |

---

## Actions NOT in the Growth Cycle (but in other tools)

These are separate MCP tools, NOT called by `run_growth_cycle`:

| Tool | Public Interaction | Trigger |
|------|-------------------|---------|
| `git_push_changes` | Pushes commits to GitHub (public repo) | Manual call only |
| `create_pull_request` | Creates PR visible on GitHub | Manual call only |
| `merge_pull_request` | Merges PR, changes main branch | Manual call only |
| `create_github_release` (via `releases.rs`) | Creates GitHub Release visible publicly | NOT wired into growth cycle yet |

---

## Dry-Run vs Live Mode Matrix

| Action | Dry-Run (default) | Live Mode |
|--------|-------------------|-----------|
| Generate release notes | Local only | Local only |
| README health check | Local only | Local only |
| Compose X post | Returns text | **POSTS to Typefully → X/Twitter** |
| Collect metrics | Reads GitHub API | Reads GitHub API + writes local JSONL |
| Issue monitoring | Returns evaluations | **COMMENTS on GitHub issues** |

---

## Environment Variables That Control Public Access

| Variable | What It Controls | If Missing |
|----------|-----------------|------------|
| `TYPEFULLY_API_KEY` | X/Twitter posting via Typefully | Posting skipped entirely |
| GitHub CLI auth (`gh auth`) | All `gh` commands (metrics, issues, releases) | Commands fail gracefully |

---

## Recommendation for First Run

1. **Run dry-run first** — `run_growth_cycle(repo: "trinsiklabs/cruxdev", dry_run: true)`. Review the output: what release notes would be generated, what README suggestions, what X post would look like.

2. **Review the X post content** — before enabling Typefully, review the composed thread text. Adjust `compose_release_thread()` formatting if needed.

3. **Set Typefully to draft mode** — in Typefully settings, disable auto-publish so posts stay as drafts for your review before going public.

4. **Run live with metrics only** — even without `TYPEFULLY_API_KEY`, the live cycle collects metrics. This is safe (read-only GitHub API).

5. **Enable Typefully when ready** — set `TYPEFULLY_API_KEY` and run live. First post will appear as a Typefully draft if auto-publish is off.

---

## What This Cycle Does NOT Do

- Does NOT create GitHub Releases (separate tool, not wired in)
- Does NOT push code to GitHub (separate tool)
- Does NOT create PRs (separate tool)
- Does NOT post to Hacker News, Reddit, or any other platform
- Does NOT reply to X/Twitter mentions or DMs
- Does NOT modify any code in the repository
- Does NOT send emails
- Does NOT access any third-party service besides Typefully and GitHub API
