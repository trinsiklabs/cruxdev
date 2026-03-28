# Autonomous Self-Improvement Bootstrap Playbook

**Purpose:** Take any project in the Crux ecosystem from its current state to fully autonomous self-improvement with BIP (Build-in-Public) pipeline. Feed this playbook to a Claude Code session and say: "Follow AUTONOMOUS_BOOTSTRAP_PLAYBOOK.md. Get to autonomous mode."

**Proven on:** CruxDev (2026-03-27, 85 build plans, 470 tests, 58 MCP tools)

---

## Prerequisites

Before starting, the project needs:
- A git repo (local, pushed to GitHub)
- CruxDev MCP server accessible (binary at `/Users/user/personal/cruxdev/rust/target/release/cruxdev`)
- Claude Code session with MCP access

---

## Phase 1: Install CruxDev

### 1.1 Add MCP configuration

Create or update `.mcp.json` in the project root:

```json
{
  "mcpServers": {
    "cruxdev": {
      "type": "stdio",
      "command": "/Users/user/personal/cruxdev/rust/target/release/cruxdev",
      "args": ["mcp", "start"]
    }
  }
}
```

### 1.2 Create .cruxdev/ directory

```bash
mkdir -p .cruxdev/evolution/posts
```

### 1.3 Create growth.toml

```toml
[project]
name = "<PROJECT_NAME>"
repo = "trinsiklabs/<repo>"
url = "<website_url_if_any>"

[typefully]
api_key_env = "TYPEFULLY_API_KEY"
social_set_id = 288244
max_posts_per_day = 3
enabled = true

[github]
repos = ["trinsiklabs/<repo>"]
issue_monitoring_enabled = true
issue_dry_run = false

[content]
website_repo = ""
blog_dir = ""
vs_dir = "docs/vs"

[metrics]
tracking_enabled = true
metrics_file = ".cruxdev/growth/metrics.jsonl"
```

### 1.4 Verify installation

Call `cruxdev_status` via MCP. Should return project classification and health status.

---

## Phase 2: GitHub Issues as Communication Channel

### 2.1 Enable issue monitoring

Ensure `github.issue_monitoring_enabled = true` in growth.toml.

### 2.2 Create issue labels

On the GitHub repo, create these labels:
- `bug` — defects
- `enhancement` — feature requests
- `patterns:<project>` — patterns doc improvements
- `adoption` — adoption process improvements
- `ecosystem` — cross-project coordination

### 2.3 Test issue round-trip

File a test issue, verify the evolution cycle picks it up, triage it, close it.

---

## Phase 3: Self-Adoption Audit

### 3.1 Run the project classifier

Call `classify_project` on the project directory. Verify it detects the correct project type(s).

### 3.2 Check patterns doc coverage

For every patterns doc in `docs/*PATTERNS*.md`:
- Does a corresponding `*_DIMENSIONS` constant exist in the convergence router?
- Is it wired into the appropriate convergence phase?
- If not: file a GitHub issue on CruxDev with label `patterns:<project>`

### 3.3 Check dimension integration

Run the architecture test (if available) that verifies all dimension sets are wired. If gaps found, fix them.

### 3.4 Check existing docs against reality

Every claim in README, CLAUDE.md, and docs/ must be verifiable. Run ground truth verification on the top 5 most-read docs.

---

## Phase 4: BIP Pipeline Setup

### 4.1 Content generation on convergence

When a build plan converges, the post-convergence hook in `convergence_submit_result` auto-generates:
- A blog post draft (written to `.cruxdev/evolution/posts/`)
- An X post draft

This is built into CruxDev — no project-specific setup needed.

### 4.1b Close originating GitHub issues

If a build plan has `Triggered by: #N` or `Triggered by: https://github.com/.../issues/N`, the post-convergence actions require closing that issue:

```bash
gh issue close N --repo owner/repo --comment "Fixed in BUILD_PLAN_NNN. Blog: https://cruxdev.dev/blog/..."
```

This closes the feedback loop: issue filed → build plan created → converged → issue closed with link to fix.

### 4.2 Typefully posting

Ensure `TYPEFULLY_API_KEY` is set in the shell environment:
```bash
echo 'export TYPEFULLY_API_KEY="<key>"' >> ~/.zshenv
source ~/.zshenv
```

The social_set_id (288244) is shared across all Trinsik projects.

### 4.3 Blog publishing (if project has a website)

Set `content.blog_dir` and `content.website_repo` in growth.toml. Blog posts auto-publish when convergence completes.

### 4.4 Test the pipeline

Converge any small build plan. Verify:
- Blog draft appears in `.cruxdev/evolution/posts/`
- X draft posted to Typefully (check https://typefully.com)
- Blog appears on website (if configured)

---

## Phase 5: Priority Engine

### 5.1 Verify work sources are scannable

Run:
```bash
cruxdev prioritize <project_dir> --repo trinsiklabs/<repo>
```

Should return a ranked list of work items from:
- Unconverged build plans
- Open GitHub issues
- Competitive gaps (if COMPETITORS.md exists)
- Content backlog
- Self-adoption findings

### 5.2 Create initial build plans for known work

Write `BUILD_PLAN_NNN_*.md` files for any known work items. The priority engine will pick them up and rank them.

---

## Phase 6: Autonomous Evolution Cron

### 6.1 Create evolve.sh

Copy from CruxDev's `scripts/evolve.sh` and adapt paths:

```bash
#!/bin/bash
set -e
PROJECT_DIR="<project_dir>"
LOG_DIR="${PROJECT_DIR}/.cruxdev/evolution"
STOP_FILE="${LOG_DIR}/STOP"

mkdir -p "${LOG_DIR}"

if [ -f "${STOP_FILE}" ]; then
    echo "[$(date)] STOP file detected." >> "${LOG_DIR}/cron.log"
    exit 0
fi

echo "[$(date)] Evolution cycle starting." >> "${LOG_DIR}/cron.log"
/Users/user/personal/cruxdev/rust/target/release/cruxdev evolve "${PROJECT_DIR}" \
    --repo trinsiklabs/<repo> --dry-run false --continuous >> "${LOG_DIR}/cron.log" 2>&1
echo "[$(date)] Evolution cycle complete." >> "${LOG_DIR}/cron.log"
```

### 6.2 Install cron

```bash
(crontab -l; echo "0 */4 * * * /path/to/evolve.sh") | crontab -
```

### 6.3 Test with dry run first

Set `--dry-run true` for the first 3 cycles. Check `.cruxdev/evolution/cron.log` for output.

### 6.4 Emergency stop

To halt autonomous mode: `touch .cruxdev/evolution/STOP`
To resume: `rm .cruxdev/evolution/STOP`

---

## Phase 7: Cross-Project Communication

### 7.1 Filing issues on other projects

When this project needs something from another project (e.g., CruxDev needs a new tool, CruxCLI needs a new mode):
- File a GitHub issue on the target project's repo
- Include: what's needed, why, and which build plan triggered it
- Label: `ecosystem`

### 7.2 Receiving issues from other projects

The evolution cycle monitors GitHub issues. When an `ecosystem` issue arrives:
- Triage: is this our responsibility?
- If yes: create a build plan, converge it
- If not: comment explaining why and suggest the right project

### 7.3 The coordination pattern

```
Project A discovers it needs Feature X from Project B
  → Files GitHub issue on Project B: "Need Feature X for [reason]"
  → Project B's evolution cycle picks it up
  → Project B creates build plan, converges it
  → Project B closes issue with implementation details
  → Project A's evolution cycle sees the closed issue
  → Project A integrates the feature
```

No session bus needed. GitHub is the coordination layer.

---

## Phase 8: Verify Autonomous Mode

Run through this checklist:

- [ ] `cruxdev prioritize` returns work items
- [ ] `cruxdev evolve --continuous` completes a cycle without error
- [ ] Cron is installed and running every 4 hours
- [ ] STOP file halts execution
- [ ] GitHub issues are detected and triaged
- [ ] Converged build plans generate blog + X posts
- [ ] Cross-project issues can be filed and received
- [ ] Priority engine ranks work correctly (bugs > gaps > features)

When all checks pass: **autonomous self-improvement is live.**

---

## Maintenance

- **Weekly:** Check `.cruxdev/evolution/cron.log` for errors
- **Monthly:** Review priority queue — are the right things getting prioritized?
- **Per adoption:** Feed learnings back (ADOPTION_PROCESS.md Step 7)
- **Per build plan convergence:** Self-adopt, check for pattern improvements

---

## Reference

| Resource | Path |
|---|---|
| CruxDev binary | `/Users/user/personal/cruxdev/rust/target/release/cruxdev` |
| Priority engine | `cruxdev prioritize <dir> --repo <repo>` |
| Evolution cycle | `cruxdev evolve <dir> --repo <repo> --continuous` |
| Growth config | `.cruxdev/growth.toml` |
| Cron log | `.cruxdev/evolution/cron.log` |
| Emergency stop | `.cruxdev/evolution/STOP` |
| Typefully API key | `TYPEFULLY_API_KEY` env var |
| Social set ID | 288244 (shared across ecosystem) |
