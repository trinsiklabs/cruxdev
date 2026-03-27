---
title: "Release Process: [Project Name]"
last_updated: YYYY-MM-DD
versioning: "semver"
---

# Release Process

## Versioning

**Scheme:** [Semantic Versioning (semver): MAJOR.MINOR.PATCH]

| Version Component | When to Increment | Example |
|---|---|---|
| MAJOR | Breaking changes to public API | 1.0.0 -> 2.0.0 |
| MINOR | New features, backward-compatible | 1.0.0 -> 1.1.0 |
| PATCH | Bug fixes, backward-compatible | 1.0.0 -> 1.0.1 |

## Release Checklist

### Pre-Release
- [ ] All targeted issues/PRs merged
- [ ] CI passes on main branch
- [ ] CHANGELOG.md updated with all changes
- [ ] Version number bumped in [files: Cargo.toml, package.json, etc.]
- [ ] Migration guide written (for breaking changes)
- [ ] Documentation updated for new features
- [ ] Release branch created (for minor/major): `release/vX.Y.Z`
- [ ] Release candidate tagged: `vX.Y.Z-rc.1`
- [ ] RC tested in staging / by beta users

### Release
- [ ] Final tag created: `vX.Y.Z`
- [ ] Release notes written on GitHub
- [ ] Binaries / packages built and published
- [ ] Package registry updated ([crates.io / npm / PyPI / etc.])
- [ ] Docker image published (if applicable)
- [ ] Documentation site updated

### Post-Release
- [ ] Announcement posted ([blog / social / Discord / mailing list])
- [ ] Verify package is installable from registry
- [ ] Monitor for critical bug reports (48h watch period)
- [ ] Backport critical fixes to release branch if needed

## Changelog Format

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- Description of new feature (#PR)

### Changed
- Description of behavior change (#PR)

### Fixed
- Description of bug fix (#PR)

### Removed
- Description of removed feature (#PR)

### Security
- Description of security fix (#PR)
```

## Release Schedule

| Type | Cadence | Notes |
|---|---|---|
| Patch releases | [As needed / weekly] | Bug fixes only |
| Minor releases | [Monthly / quarterly] | New features + fixes |
| Major releases | [As needed, minimum 6-month gap] | Breaking changes with migration guide |

## Template Version

- **Version:** 1.0
- **Created:** 2026-03-25
- **Last Updated:** 2026-03-25
