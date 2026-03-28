# BUILD_PLAN_096: Core Bot Loop — Software-Driven Continuous Agent

**Status:** NOT STARTED
**Priority:** Critical (THE product)
**Depends on:** BP095 (bot architecture)

## Context

Right now CruxDev is an MCP server that waits for a client (Claude Code) to call it. The client drives the loop. INVERT THIS. The Rust binary drives the loop. It makes LLM API calls directly. No Claude Code. No MCP client. The binary IS the bot.

```
Current:  Claude Code → calls CruxDev MCP tools → CruxDev responds
Inverted: CruxDev binary → calls LLM API → processes response → calls LLM again → converges
```

## The Core Loop (Rust)

```rust
// cruxdev daemon
loop {
    // 1. SCAN — what needs doing?
    let items = priority::scan_work_sources(&project_dir, &github_repo);
    if items.is_empty() {
        sleep(backoff);
        continue;
    }

    // 2. PICK — highest priority item
    let task = priority::pick_next(&items);

    // 3. PLAN — if it's a new issue, create a build plan
    if task.action == "fix_issue" {
        let plan = llm::call("Create a build plan for: {task.title}");
        write_build_plan(&plan);
    }

    // 4. CONVERGE — drive the convergence loop
    let mut state = start_convergence(&task.plan_file);
    while !state.is_terminal() {
        let next_task = get_next_task(&mut state);

        // Call LLM with the task + dimensions
        let result = llm::call(&format!(
            "Audit this code against dimensions: {:?}\nFiles: {:?}",
            next_task.dimensions, next_task.files
        ));

        // Parse findings from LLM response
        let findings = parse_findings(&result);

        // If findings, ask LLM to fix them
        for finding in &findings {
            let fix = llm::call(&format!("Fix: {}", finding.description));
            apply_fix(&fix);
        }

        // Submit results
        submit_result(&mut state, &findings);
    }

    // 5. DELIVER — post-convergence actions
    git_commit_and_push();
    generate_and_publish_content();
    close_github_issue();
    check_competitive_impact();

    // 6. REFLECT — self-adopt
    self_adopt();

    // 7. REPORT — update terminal viewer
    sync_stream();
}
```

## What Changes

| Component | Current (MCP Server) | New (Bot) |
|---|---|---|
| Loop driver | Claude Code (external) | Rust binary (internal) |
| LLM calls | Claude Code makes them | Binary calls Anthropic API directly |
| File edits | Claude Code Write tool | Binary reads/writes files directly |
| Git operations | Claude Code Bash tool | Binary calls git CLI directly |
| Decision making | LLM decides everything | Engine decides structure, LLM does language |
| Running time | Per session (hours) | Continuous (days/weeks/months) |

## Phase 1: Multi-Provider LLM Client

Leverage CruxCLI's existing provider infrastructure (Anthropic, OpenAI, OpenRouter, local LLM).

- [ ] 1.1 Import provider architecture from CruxCLI: API key management, model routing, auth
- [ ] 1.2 Model optimization by task tier:
  - **Core loop** (CHECK/PRIORITIZE/REFLECT): local LLM or cheapest (Haiku, Gemini Flash, local Llama)
  - **Auditing**: standard (Sonnet) — workhorse
  - **Security/architecture**: frontier (Opus) — highest capability
  - **Content generation**: standard (Sonnet)
  - **GTV checks**: code-only or cheapest (parse output, call APIs, check files — often no LLM needed)
- [ ] 1.3 Cost tracking: per-call, per-convergence, per-day, per-model breakdown
- [ ] 1.4 Auto-downgrade: if daily budget hit, switch to cheaper models or pause
- [ ] 1.5 Local LLM fallback: if API down or rate-limited, use local for non-critical tasks
- [ ] 1.6 Performance/cost efficiency scoring: track which model gives best results per dollar
- [ ] 1.7 KV-cache optimization: stable prefix, append-only context

## Phase 2: Autonomous Task Execution

- [ ] 2.1 File reading/writing without MCP — direct filesystem access
- [ ] 2.2 Code modification: parse LLM response for file edits, apply them
- [ ] 2.3 Test running: execute test commands, parse results
- [ ] 2.4 Git operations: commit, push, PR — already implemented
- [ ] 2.5 Build commands: cargo test, npm run build, etc.

## Phase 3: The Daemon

- [ ] 3.1 `cruxdev daemon --project-dir . --repo owner/repo`
- [ ] 3.2 PID file for process management
- [ ] 3.3 Signal handling: SIGTERM for graceful shutdown, SIGHUP for reload config
- [ ] 3.4 Log rotation for long-running operation
- [ ] 3.5 Health endpoint (HTTP) for monitoring
- [ ] 3.6 Metrics endpoint (Prometheus format)
- [ ] 3.7 Adaptive sleep cycle:
  - No work → 1 min → 5 min → 15 min → 1 hr (exponential backoff, cap at 1 hr)
  - Work found → reset to 1 min
  - Webhook received → wake immediately (0 sleep)
  - High-activity burst → 10 sec between cycles
  - Tracks work arrival rate over 24h rolling window
  - Auto-adjusts: if work arrives every 30 min avg → sleep 10 min
  - Self-tuning: measures response time vs idle cost, optimizes the ratio
  - Dormant when idle, responsive when busy. Minimal token burn.

## Phase 4: Safety for Unsupervised Operation

- [ ] 4.1 Max LLM calls per hour (cost control)
- [ ] 4.2 Max file modifications per cycle (blast radius)
- [ ] 4.3 Dry-run mode for first N cycles
- [ ] 4.4 Allowlist of files/dirs the bot can modify
- [ ] 4.5 Denylist of operations (never force push, never delete branches)
- [ ] 4.6 Emergency stop (STOP file, SIGTERM, kill switch endpoint)
- [ ] 4.7 All changes on branches, never direct to main
- [ ] 4.8 Human approval gate for PRs (bot creates, human merges)

## Phase 5: Multi-Project Daemon

- [ ] 5.1 Config file listing all managed projects
- [ ] 5.2 Shared LLM budget across projects
- [ ] 5.3 Cross-project issue filing (already implemented)
- [ ] 5.4 Shared pattern improvements

## The Result

A single Rust binary that:
- Runs continuously
- Monitors GitHub issues across all your projects
- Creates build plans from issues
- Converges each plan (audit → fix → re-audit → converge)
- Commits, pushes, creates PRs
- Generates blog posts and X content
- Deploys websites
- Monitors SEO/GEO
- Self-improves after every convergence
- Reports progress in real-time via terminal viewer
- Costs are tracked and capped
- Can be stopped at any time safely

**That's the CruxDev bot.**

## Phase 7: Self-Evolution

The bot doesn't just run tasks — it evolves itself.

```
Core evolution loop:
1. OBSERVE — what went wrong? what took too long? what patterns repeated?
2. DIAGNOSE — extract the root cause (missing pattern? missing dimension? missing template?)
3. PLAN — write a build plan to fix the root cause
4. CONVERGE — converge its own build plan (self-modifying code)
5. VERIFY — run all tests, verify nothing regressed
6. ADOPT — self-adoption: check if the fix improves the system
7. ANNOUNCE — blog + X post about the self-improvement
```

### What Self-Evolution Looks Like

| Observation | Self-Written Build Plan | Result |
|---|---|---|
| "Book audits keep flagging voice drift" | BP: Add voice fingerprinting dimension | New audit catches drift earlier |
| "Astro pages fail with bare braces in JSON" | BP: Add JSON escaping to page generator | No more build failures |
| "Evolution loop spammed 6000 posts" | BP: Add dedup + max-actions-per-cycle | Spam prevented |
| "Form patterns missing label positioning" | BP: Research and add 10 form design sections | Better form audits |
| "Typefully API returns 403" | BP: Research v2 API, fix social_set_id | Typefully working |

Every one of those happened THIS SESSION. The bot already self-evolved — it just did it through a human-mediated Claude Code session instead of autonomously.

### Safety for Self-Modification

- All self-modifications on feature branches (never main)
- PRs created, human reviews before merge
- Full test suite must pass before PR is created
- Rollback if tests fail after merge
- Self-modification budget: max 3 self-evolution plans per day
- Architecture tests verify core invariants survive modification
- "Protected files" list: core engine files require human approval

### The Compounding Advantage

Each self-evolution cycle makes the next cycle better:
- Better patterns → better audits → fewer issues → faster convergence
- Better templates → better project setup → fewer adoption problems
- Better classifiers → better dimension selection → more relevant audits
- Better safety gates → fewer failures → more trust → more autonomy

**No other bot does this.** AutoGPT loops. Devin runs once. Sweep fixes one issue. Crux Bot fixes the issue AND improves its ability to fix similar issues forever.
