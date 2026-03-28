# BUILD_PLAN_004: cruxvibe.io — CruxCLI Terminal Agent Website

**Created:** 2026-03-22
**Status:** CONVERGED
**Goal:** Build the marketing and download site for CruxCLI — the terminal-first AI coding agent with native Crux intelligence.
**Methodology:** Follow `docs/WEBSITE_PLANNING.md`. SEO details in `docs/SEO_AND_GEO_REFERENCE.md`.

**Domain:** cruxvibe.io
**Product:** CruxCLI — hard fork of OpenCode with native Crux integration
**Category:** CLI/terminal tool (like Warp, Cursor, Aider pattern)
**Current state:** Binary compiles and runs. Rebrand complete. Prompt replacement, bridge absorption, token budgets done. Not yet publicly distributed.

---

## Audiences

| Audience | Priority | They want | Entry point |
|----------|----------|-----------|-------------|
| Terminal-first developers (neovim, SSH, tmux) | Primary | Powerful coding agent that stays in the terminal | "AI coding that lives where you live — the terminal" |
| Ollama/local model users | Primary | Zero-cost inference without losing intelligence | "Run local. Think global." |
| OpenCode users wanting more | Secondary | Native intelligence layer, not plugin bolt-on | "OpenCode + Crux intelligence, built in" |
| Developers evaluating AI coding tools | Tertiary | Feature comparison, what makes this different | Comparison pages, feature grid |

---

## Site Map

```
cruxvibe.io/
├── / (homepage)
├── /features
│   ├── /features/modes (24 Crux modes built in)
│   ├── /features/local-models (Ollama, zero cost)
│   ├── /features/token-budget (per-mode budget enforcement)
│   └── /features/tui (terminal UI, client/server)
├── /download (platform-specific binaries)
├── /docs
│   ├── /docs/quickstart
│   ├── /docs/configuration
│   ├── /docs/modes
│   ├── /docs/providers (Anthropic, OpenAI, Ollama, etc.)
│   └── /docs/plugins
├── /guides
│   ├── /guides/getting-started
│   ├── /guides/local-models
│   ├── /guides/crux-integration
│   └── /guides/migrating-from-opencode
├── /blog
├── /changelog
├── /community
├── /vs (comparison pages)
│   ├── /vs/opencode
│   ├── /vs/aider
│   ├── /vs/claude-code
│   └── /vs/cursor
├── /llms.txt
└── /llms-full.txt
```

---

## Homepage Sections

1. **Hero**: Dark background (terminal aesthetic). Animated TUI screenshot showing CruxCLI in action. Headline: "AI coding, terminal-native." Subheadline: "OpenCode + Crux intelligence. 24 modes. Local or cloud. Zero API cost with Ollama." CTA: "Download for macOS" + "View on GitHub"
2. **Install command**: Prominently displayed one-liner: `curl -fsSL https://cruxvibe.io/install | bash`
3. **Trust**: GitHub stars, test count (1,193), provider count (19), mode count (24)
4. **Why CruxCLI**: 3 pain points → solutions:
   - "Generic prompts" → "24 specialized modes"
   - "API costs" → "Local models, zero cost"
   - "Context loss" → "Crux knowledge persists"
5. **Features**: 6 cards — TUI, Modes, Local Models, Token Budgets, 19 Providers, Plugin System
6. **Terminal demo**: Animated GIF or video showing mode switching, code generation, test running
7. **Comparison table**: CruxCLI vs OpenCode vs Aider vs Claude Code (feature grid)
8. **Final CTA**: "Install CruxCLI" → download page

---

## SEO Targets

| Keyword | Intent | Content |
|---------|--------|---------|
| "terminal AI coding" | Informational | Blog: terminal-first AI development |
| "OpenCode alternative" | Commercial | /vs/opencode comparison |
| "Aider alternative" | Commercial | /vs/aider comparison |
| "local LLM coding agent" | Informational | Guide: local models with CruxCLI |
| "AI coding zero cost" | Commercial | Feature: local models page |
| "AI coding terminal TUI" | Informational | Feature: TUI page |
| "Claude Code vs OpenCode" | Commercial | Blog: comparison (mentions CruxCLI as option) |

---

## AI Visibility

- `/llms.txt`: CruxCLI identity, capabilities, how to install, provider support
- Terminal-specific imagery won't render in AI — use text descriptions alongside screenshots
- Schema.org: `SoftwareApplication` with `operatingSystem`, `downloadUrl`, `applicationCategory`
- Comparison pages structured as `FAQPage` for AI extraction

---

## Checklist

### Phase 1: Content & Structure
- [ ] 1.1 Homepage copy (hero, features, comparison)
- [ ] 1.2 Download page (macOS, Linux, Windows binaries)
- [ ] 1.3 Quickstart guide (install → first conversation)
- [ ] 1.4 Features pages (modes, local models, TUI, token budgets)
- [ ] 1.5 Comparison pages (vs OpenCode, vs Aider, vs Claude Code, vs Cursor)
- [ ] 1.6 Migration guide from OpenCode
- [ ] 1.7 llms.txt and llms-full.txt
- [ ] 1.8 All meta tags and structured data

### Phase 2: Design & Build
- [ ] 2.1 Design system (dark theme primary, terminal aesthetic)
- [ ] 2.2 Homepage with animated TUI demo
- [ ] 2.3 Download page with platform detection
- [ ] 2.4 Docs layout
- [ ] 2.5 Build site
- [ ] 2.6 Responsive design (terminal screenshots need mobile care)

### Phase 3: Technical
- [ ] 3.1 Core Web Vitals green
- [ ] 3.2 SEO: sitemap, structured data, canonicals
- [ ] 3.3 Accessibility: WCAG 2.1 AA
- [ ] 3.4 Security: HTTPS, CSP
- [ ] 3.5 Analytics
- [ ] 3.6 Binary hosting/CDN for downloads

### Phase 4: Launch
- [ ] 4.1 DNS for cruxvibe.io
- [ ] 4.2 Deploy
- [ ] 4.3 Search Console
- [ ] 4.4 Pre-launch QA
- [ ] 4.5 Launch announcement
- [ ] 4.6 Post-launch verification

**Total: 24 checkboxes**
