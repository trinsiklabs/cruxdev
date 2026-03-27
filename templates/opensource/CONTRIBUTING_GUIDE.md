---
title: "Contributing Guide: [Project Name]"
last_updated: YYYY-MM-DD
---

# Contributing to [Project Name]

Thank you for wanting to contribute. This guide explains how.

## Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USER/[project].git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `[test command]`
6. Push: `git push origin feature/your-feature-name`
7. Open a Pull Request against `main`

## What We Accept

| Contribution Type | Welcome? | Process |
|---|---|---|
| Bug fixes | Always | Open PR with test proving the fix |
| Documentation improvements | Always | Open PR |
| New features (small) | Usually | Open issue first to discuss |
| New features (large) | Sometimes | Open RFC issue, wait for approval before building |
| Performance improvements | Yes | Include benchmarks before and after |
| Refactoring | Case by case | Open issue explaining the benefit first |
| Dependency updates | Usually | Ensure tests pass, note breaking changes |

## Before You Start

- **Check existing issues** — someone may already be working on this
- **Open an issue first** for features — saves you from building something we cannot merge
- **Read the architecture docs** — understand how the codebase is organized

## Development Setup

```bash
# Prerequisites
[list required tools and versions]

# Setup
[step-by-step setup commands]

# Run tests
[test command]

# Run locally
[local dev command]
```

## Code Standards

- **Style:** [linter/formatter used — e.g., rustfmt, prettier, black]
- **Tests:** Every PR must include tests for new functionality
- **Documentation:** Public APIs must have doc comments
- **Commits:** [Conventional commits / descriptive messages / squash on merge]
- **Branch naming:** `feature/`, `fix/`, `docs/`, `refactor/`

## Pull Request Process

1. Fill out the PR template (describe what and why)
2. Ensure CI passes (tests, lint, build)
3. Request review from a maintainer
4. Address review feedback
5. Maintainer merges when approved

**PR review SLA:** We aim to review PRs within [48 hours / 1 week]. If you have not heard back, comment on the PR to ping us.

## Reporting Bugs

Use the bug report issue template. Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, language version, project version)

## Getting Help

- [GitHub Discussions / Discord / Slack] — for questions about contributing
- [Issue tracker] — for bugs and feature requests

## Template Version

- **Version:** 1.0
- **Created:** 2026-03-25
- **Last Updated:** 2026-03-25
