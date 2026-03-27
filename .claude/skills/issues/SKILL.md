---
name: issues
description: "Monitor GitHub issues with prompt injection defense — fetch, sanitize, evaluate, respond. Use when the user asks to check issues, monitor issues, or triage issues. Calls monitor_issues, issue_audit_log MCP tools."
---

# /issues — GitHub Issue Monitoring

## Arguments

$ARGUMENTS = optional: repo name, "audit" for audit log

## Protocol

### Monitor issues

Call `monitor_issues(repo, dry_run)`:
- Fetches open issues via gh CLI
- Sanitizes against 5-layer prompt injection defense
- Evaluates priority (code-first, labels → priority)
- Generates responses (dry-run by default)

### View audit log

Call `issue_audit_log(limit, project_dir)` to see recent evaluations.

### Report

Show: issues processed, priority triage, suspicious patterns detected, responses generated.
