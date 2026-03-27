# BUILD_PLAN_024: Dark Mode + Quick Start/Install Patterns in WEBSITE_PLANNING.md

**Status:** CONVERGED
**Priority:** High

## Context

WEBSITE_PLANNING.md is missing three critical patterns:
1. Dark mode with automatic OS detection — every modern site needs this
2. Quick Install page — get software installed in < 60 seconds
3. Quick Start guide — first meaningful result in < 5 minutes

All three should be evaluated during website convergence.

## Phase 1: Add Dark Mode Section to WEBSITE_PLANNING.md

- [ ] 1.1 Detection: prefers-color-scheme + matchMedia listener
- [ ] 1.2 CSS strategy: class-based with CSS custom properties, Tailwind dark: variant
- [ ] 1.3 Color design: #121212 base (not #000), surface elevation hierarchy, desaturated accents
- [ ] 1.4 Toggle UX: 3-state (system/light/dark), localStorage persistence
- [ ] 1.5 FOUC prevention: inline blocking script in <head>
- [ ] 1.6 Images: currentColor for icons, dual logo variants
- [ ] 1.7 Accessibility: WCAG AA in both modes, no pure white body text
- [ ] 1.8 Anti-patterns: no pure inversion, no #000, no same shadows

## Phase 2: Add Quick Install Section to WEBSITE_PLANNING.md

- [ ] 2.1 Page structure: prerequisites → one-liner install → verify → next
- [ ] 2.2 Platform tabs: OS detection, package manager options
- [ ] 2.3 Verification step: command + expected output
- [ ] 2.4 Copy-to-clipboard on all code blocks
- [ ] 2.5 Troubleshooting: collapsible, top 3 issues
- [ ] 2.6 Target: completable in < 60 seconds

## Phase 3: Add Quick Start Section to WEBSITE_PLANNING.md

- [ ] 3.1 7-section structure: description → prerequisites → install → setup → first action → see result → what's next
- [ ] 3.2 TTFV target: < 5 minutes
- [ ] 3.3 Code examples: copy-paste complete, runnable, with file paths
- [ ] 3.4 Progressive disclosure: one path, no branching
- [ ] 3.5 "What's next" section: 3-5 links
- [ ] 3.6 Anti-patterns checklist

## Phase 4: Engine Integration

- [ ] 4.1 Add WEBSITE_ESSENTIALS to router — pages every project website should have
- [ ] 4.2 Website convergence checks for: install page, quick start page, dark mode support
