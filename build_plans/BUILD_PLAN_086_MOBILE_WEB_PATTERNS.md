# BUILD_PLAN_086: Mobile Web/Webapp Design Patterns

**Status:** CONVERGED
**Priority:** High
**Triggered by:** cruxdev.dev had no mobile navigation — hamburger menu missing

## Context

The website patterns doc (WEBSITE_PLANNING.md) doesn't cover mobile-specific design. The site had desktop-only nav with `hidden md:flex` and no mobile fallback. This needs a research-converged patterns doc covering mobile web and progressive web app design.

## Phase 1: Deep Research

- [ ] 1.1 Mobile navigation patterns (hamburger, bottom nav, tab bar, drawer, full-screen overlay)
- [ ] 1.2 Touch target sizing (WCAG 2.5.8: 24x24 minimum, 44x44 recommended)
- [ ] 1.3 Responsive breakpoints (mobile-first, Tailwind breakpoints, container queries)
- [ ] 1.4 Mobile typography (fluid type, viewport units, readability)
- [ ] 1.5 Mobile forms (input zoom prevention, keyboard types, autocomplete)
- [ ] 1.6 Mobile performance (critical rendering path, above-the-fold, lazy loading)
- [ ] 1.7 PWA patterns (manifest, service worker, install prompts, offline)
- [ ] 1.8 Mobile-specific accessibility (screen readers, reduced motion, dark mode)
- [ ] 1.9 Mobile testing tools (Chrome DevTools device mode, Lighthouse mobile, BrowserStack)

## Phase 2: Write MOBILE_WEB_PATTERNS.md

Target 800+ lines covering all research areas with code examples.

## Phase 3: Engine Integration

- [ ] 3.1 Add MOBILE_DIMENSIONS to router.rs
- [ ] 3.2 Wire into WebsiteConvergence phase
- [ ] 3.3 Add mobile nav check to SEO health checker

## Phase 4: Content Generation

- [ ] 4.1 Blog post + X post via BIP pipeline

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
