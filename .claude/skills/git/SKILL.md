---
name: git
description: "Manage git operations with safety gates — commit, push, create PR, merge. Use when the user asks to commit, push, create a PR, or merge. Calls git_status_check, git_commit_changes, git_push_changes, create_pull_request, merge_pull_request MCP tools. All operations dry-run by default."
disable-model-invocation: true
---

# /git — Git Operations with Safety Gates

## Arguments

$ARGUMENTS = operation (commit, push, pr, merge) and optional message

## Safety Rules

- NEVER use git add -A or git add .
- NEVER force push
- Pre-commit: reject secrets, binaries >1MB, gitignored files
- Pre-push: all tests must pass
- All operations dry-run by default unless user specifies live

## Protocol

### Step 1: Check status

Call `git_status_check(project_dir)` to see staged, unstaged, untracked files.

### Step 2: Execute requested operation

**Commit:** Call `git_commit_changes(message, files, project_dir, dry_run)` with specific files (not all).

**Push:** Call `git_push_changes(remote, branch, project_dir, test_command, dry_run)` with optional test gate.

**PR:** Call `create_pull_request(title, body, base, head, repo, dry_run)`.

**Merge:** Call `merge_pull_request(pr_number, repo, method, dry_run)`.

### Step 3: Report result

Show what was done (or would be done in dry-run mode).
