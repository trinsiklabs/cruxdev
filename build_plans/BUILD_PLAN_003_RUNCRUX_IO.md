# BUILD_PLAN_003: runcrux.io — Crux Platform Website

**Created:** 2026-03-22
**Status:** NOT STARTED
**Goal:** Build the marketing and documentation site for Crux — the self-improving intelligence layer for AI coding tools.
**Methodology:** Follow `docs/WEBSITE_PLANNING.md`. SEO details in `docs/SEO_AND_GEO_REFERENCE.md`.

**Domain:** runcrux.io
**Product:** Crux — MCP-based intelligence layer (modes, memory, safety, corrections)
**Category:** Developer platform / infrastructure (like Vercel, Supabase pattern)
**Current state:** Shipped. 1,561 tests, 100% coverage, 37 MCP tools, 24 modes, MIT licensed.

---

## Audiences

| Audience | Priority | They want | Entry point |
|----------|----------|-----------|-------------|
| AI coding tool users (Claude Code, Cursor, Windsurf) | Primary | Context that persists across sessions and tools | "Stop losing context when you switch tools" |
| Team leads evaluating AI coding tools | Secondary | Consistency, safety, learning across the team | Case studies, safety pipeline docs |
| Open source contributors | Tertiary | Architecture understanding, contribution guide | GitHub, docs |

---

## Site Map

```
runcrux.io/
├── / (homepage)
├── /features
│   ├── /features/modes (24 specialized modes)
│   ├── /features/knowledge (organic knowledge generation)
│   ├── /features/safety (7-gate safety pipeline)
│   └── /features/analytics (daily self-assessment digest)
├── /docs
│   ├── /docs/quickstart (5-min install)
│   ├── /docs/modes (mode reference)
│   ├── /docs/mcp-tools (37 tools reference)
│   ├── /docs/configuration
│   ├── /docs/architecture
│   └── /docs/api-reference
├── /guides
│   ├── /guides/getting-started
│   ├── /guides/adopting-a-project
│   ├── /guides/switching-modes
│   ├── /guides/cross-tool-workflow
│   └── /guides/writing-knowledge-entries
├── /blog
├── /changelog
├── /community (Discord, GitHub, contributing)
├── /about
├── /llms.txt
└── /llms-full.txt
```

---

## Homepage Sections

1. **Hero**: "The self-improving operating system for AI coding" / "Your corrections become institutional knowledge. Your context follows you across tools." / CTA: "Get Started" + "View on GitHub"
2. **Trust**: GitHub stars, test count (1,561), coverage (100%), tool count (37 MCP tools)
3. **Problem**: "Every AI tool starts from zero. Every session. Crux accumulates and improves." — 3 pain points: context loss, repeated corrections, no safety gates
4. **Features**: 4 cards — Modes (24 specialized), Knowledge (organic, cross-project), Safety (7-gate pipeline), Analytics (daily digest)
5. **How it works**: 3 steps — Install (one command) → Connect (MCP to your tool) → Work (context persists, corrections accumulate)
6. **Integrations**: Logos — Claude Code, OpenCode, Cursor, Windsurf
7. **Testimonial/quote**: From Bryan or early users
8. **Final CTA**: "Install Crux in 5 minutes" → quickstart

---

## SEO Targets

| Keyword | Intent | Content |
|---------|--------|---------|
| "AI coding tool memory" | Informational | Blog: how AI tools lose context and how to fix it |
| "MCP server for coding" | Navigational | Features page |
| "Claude Code plugins" | Commercial | Guide: using Crux with Claude Code |
| "AI coding safety" | Informational | Safety pipeline feature page |
| "cross-session context AI" | Informational | Blog: why AI tools don't remember |
| "OpenCode vs Claude Code" | Commercial | Guide: using Crux across both |

---

## AI Visibility

- `/llms.txt`: Describe Crux, its capabilities, how to install, what it does
- Answer-first content on all feature pages
- Schema.org: `SoftwareApplication` on homepage, `HowTo` on guides, `FAQPage` on FAQ
- Specific metrics in content: "1,561 tests", "24 modes", "37 MCP tools", "100% coverage"

---

## Checklist

### Phase 1: Content & Structure
- [ ] 1.1 Write homepage copy (hero, features, how-it-works)
- [ ] 1.2 Write quickstart guide (5 minutes to first value)
- [ ] 1.3 Write features pages (modes, knowledge, safety, analytics)
- [ ] 1.4 Write 3 initial blog posts (context loss, corrections as knowledge, safety pipeline)
- [ ] 1.5 Create llms.txt and llms-full.txt
- [ ] 1.6 Define all meta tags and structured data

### Phase 2: Design & Build
- [ ] 2.1 Design system (tokens, components)
- [ ] 2.2 Homepage design (desktop + mobile)
- [ ] 2.3 Docs layout (navigation, search, code blocks)
- [ ] 2.4 Build site (static site generator — Astro, Next.js, or SolidJS)
- [ ] 2.5 Implement responsive design
- [ ] 2.6 Implement dark mode

### Phase 3: Technical
- [ ] 3.1 Performance: all Core Web Vitals green
- [ ] 3.2 SEO: sitemap, robots.txt, structured data, canonicals
- [ ] 3.3 Accessibility: WCAG 2.1 AA, Lighthouse 95+
- [ ] 3.4 Security: HTTPS, CSP, security headers
- [ ] 3.5 Analytics: privacy-respecting (Plausible or Fathom)
- [ ] 3.6 Cookie consent (if needed)

### Phase 4: Launch
- [ ] 4.1 DNS configured for runcrux.io
- [ ] 4.2 Deploy to CDN (Vercel, Cloudflare Pages, or Netlify)
- [ ] 4.3 Submit to Google Search Console
- [ ] 4.4 Pre-launch QA (full checklist from WEBSITE_PLANNING.md Phase 10)
- [ ] 4.5 Launch announcement
- [ ] 4.6 Post-launch verification (48 hours)

**Total: 22 checkboxes**
