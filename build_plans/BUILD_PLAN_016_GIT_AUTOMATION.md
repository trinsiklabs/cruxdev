# BUILD_PLAN_016: Git Workflow Automation in Convergence Pipeline

**Status:** CONVERGED
**Priority:** High
**Depends on:** BUILD_PLAN_015 (Rust migration complete, 41 MCP tools)

## Context

Build plans execute code changes but never commit, push, or create PRs. The loop doesn't close — human intervention is required for every git operation. This breaks the autonomous convergence model. CruxDev needs to own the full lifecycle: plan → code → test → commit → push → PR → merge.

The rust/target/ incident (build artifacts accidentally committed, blocking push) also shows the need for pre-commit safety checks.

## Phase 1: Git Operations Module

### 1.1 Core git operations
- [ ] 1.1.1 New module: `rust/src/git.rs`
- [ ] 1.1.2 `git_status(project_dir)` — staged, unstaged, untracked files
- [ ] 1.1.3 `git_diff_stat(project_dir)` — summary of changes
- [ ] 1.1.4 `git_log(project_dir, count)` — recent commit messages + style detection
- [ ] 1.1.5 `git_add(project_dir, files)` — stage specific files (NEVER `git add -A`)
- [ ] 1.1.6 `git_commit(project_dir, message)` — create commit
- [ ] 1.1.7 `git_push(project_dir, remote, branch)` — push with safety checks
- [ ] 1.1.8 `git_current_branch(project_dir)` — get current branch name
- [ ] 1.1.9 `git_create_branch(project_dir, name)` — create and switch to branch

### 1.2 Safety checks
- [ ] 1.2.1 Pre-commit: reject files matching `.gitignore` patterns (defense against target/ incident)
- [ ] 1.2.2 Pre-commit: reject files > 1MB (catch accidental binaries)
- [ ] 1.2.3 Pre-commit: reject `.env`, credentials, keys
- [ ] 1.2.4 Pre-push: verify all tests pass before allowing push
- [ ] 1.2.5 Pre-push: verify branch is not diverged from remote
- [ ] 1.2.6 Never force push to main/master

### 1.3 Tests
- [ ] 1.3.1 Unit tests for each git operation (tempdir + git init)
- [ ] 1.3.2 Test: safety check rejects target/ files
- [ ] 1.3.3 Test: safety check rejects .env files
- [ ] 1.3.4 Test: safety check rejects files > 1MB

## Phase 2: PR Operations

### 2.1 GitHub PR via `gh` CLI
- [ ] 2.1.1 `create_pr(repo, title, body, base, head)` — create PR
- [ ] 2.1.2 `pr_status(repo, pr_number)` — check CI status
- [ ] 2.1.3 `merge_pr(repo, pr_number, method)` — merge (squash default)
- [ ] 2.1.4 `close_pr(repo, pr_number)` — close without merge

### 2.2 PR content generation
- [ ] 2.2.1 Auto-generate PR title from build plan name
- [ ] 2.2.2 Auto-generate PR body: summary bullets, test plan, file list
- [ ] 2.2.3 Include convergence status in PR body

### 2.3 Tests
- [ ] 2.3.1 Unit test: PR body generation format
- [ ] 2.3.2 E2E test: create_pr via MCP tool (dry-run)

## Phase 3: MCP Tools

### 3.1 New tools
- [ ] 3.1.1 `git_commit_changes(message, files, project_dir)` — stage + commit with safety checks
- [ ] 3.1.2 `git_push_changes(remote, branch, project_dir)` — push with test gate
- [ ] 3.1.3 `create_pull_request(title, body, base, head, repo)` — create PR
- [ ] 3.1.4 `merge_pull_request(pr_number, repo)` — merge with CI check
- [ ] 3.1.5 `git_status_check(project_dir)` — full status for decision-making

### 3.2 Dry-run support
- [ ] 3.2.1 All tools default to `dry_run=true`
- [ ] 3.2.2 Dry run returns what WOULD happen without executing

## Phase 4: Convergence Pipeline Integration

### 4.1 Wire into convergence lifecycle
- [ ] 4.1.1 After CONVERGED state → auto-commit with build plan reference
- [ ] 4.1.2 After commit → auto-push to feature branch
- [ ] 4.1.3 After push → auto-create PR
- [ ] 4.1.4 After CI passes → offer to merge (or auto-merge if configured)
- [ ] 4.1.5 After merge → update build plan status to MERGED

### 4.2 Commit message generation
- [ ] 4.2.1 Read git log for style (conventional commits, etc.)
- [ ] 4.2.2 Generate message from build plan + convergence results
- [ ] 4.2.3 Include test count, coverage, and tool count in message
- [ ] 4.2.4 Include Co-Authored-By trailer

### 4.3 Evolution integration
- [ ] 4.3.1 Evolution INTEGRATE beat calls git operations
- [ ] 4.3.2 Evolution POST beat references commit/PR in changelog
- [ ] 4.3.3 Evolution ENGAGE beat comments on related issues

## Phase 5: Tests + Convergence

### 5.1 Unit tests
- [ ] 5.1.1 Git operations with real git repos in tempdir
- [ ] 5.1.2 Safety check enforcement
- [ ] 5.1.3 PR body generation

### 5.2 E2E tests
- [ ] 5.2.1 `test_git_commit_via_mcp` — commit through MCP tool
- [ ] 5.2.2 `test_git_safety_rejects_binary` — large file rejection
- [ ] 5.2.3 `test_create_pr_dry_run` — PR creation dry run

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo test --test mcp_e2e -- --nocapture
cd rust && cargo clippy -- -D warnings
```

## Key Design Decisions

- **Never `git add -A`** — always stage specific files to prevent accidental commits
- **Never force push** — especially not to main/master
- **Pre-commit safety gate** — reject binaries, secrets, gitignored files
- **Pre-push test gate** — all tests must pass before push
- **Dry-run default** — all git/PR operations require explicit `live_mode=true`
- **Feature branch workflow** — convergence creates branches, PRs merge to main
- **Commit message from convergence** — includes plan reference, test count, findings closed
