---
title: "Open Source Governance: [Project Name]"
last_updated: YYYY-MM-DD
license: "[MIT / Apache 2.0 / GPL / etc.]"
---

# Open Source Governance Model

## Project Identity

**Project name:** [Name]

**Repository:** [URL]

**License:** [License type]

**One-sentence description:** [What the project does]

## Governance Type

**Model:** [BDFL (Benevolent Dictator) / Meritocracy / Foundation-governed / Corporate-backed]

**Decision-making process:**
- **Minor changes** (bug fixes, small features): Maintainer approval
- **Medium changes** (new features, API changes): 2 maintainer approvals
- **Major changes** (architecture, breaking changes): RFC process + core team vote
- **Governance changes**: Requires [supermajority / unanimous] core team agreement

## Roles

| Role | Count | Responsibilities | How Appointed |
|---|---|---|---|
| Project lead | 1 | Vision, final decisions, conflict resolution | [Founder / elected] |
| Core maintainer | [X] | Code review, merge authority, release management | Invitation after sustained contribution |
| Maintainer | [X] | Code review for specific areas, triage | Invitation after 5+ merged PRs |
| Contributor | Open | Submit PRs, file issues, improve docs | Self-selected |
| Community member | Open | Use project, report bugs, participate in discussions | Self-selected |

## RFC Process (for significant changes)

1. Author opens an RFC issue/document describing the proposed change
2. Discussion period: [2 weeks minimum]
3. Core team vote: [simple majority / supermajority]
4. If approved: implementation begins with the RFC as the spec
5. If rejected: documented rationale, can be re-proposed after [6 months]

## Code of Conduct

[Link to CODE_OF_CONDUCT.md or state which standard code of conduct is adopted, e.g., Contributor Covenant]

**Enforcement:** [Who handles reports, escalation path, consequences]

## Release Authority

| Release Type | Who Can Release | Process |
|---|---|---|
| Patch (x.x.X) | Any maintainer | PR merged + CI green + changelog entry |
| Minor (x.X.0) | Core maintainer | Release branch + testing + changelog + announcement |
| Major (X.0.0) | Project lead + 2 core | RFC + migration guide + extended testing + announcement |

## Template Version

- **Version:** 1.0
- **Created:** 2026-03-25
- **Last Updated:** 2026-03-25
