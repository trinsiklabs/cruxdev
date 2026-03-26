---
title: "Website Engine Ecosystem"
last_updated: "2026-03-26"
research_method: "5-pass iterative deepening per RESEARCH_PATTERNS.md"
---

# Website Engine Ecosystem

**Purpose:** Before converging a website build plan, prompt the user to select a website engine with pros/cons. Support switching engines later if needs change. No lock-in.

**Usage:** During WEBSITE_CONVERGENCE or when creating a new website build plan, present this matrix and ask the user to choose. If the project already has a site, detect the current engine and note migration paths.

---

## Pre-Build Prompts

Before starting any website build plan, the engine must collect:

### 1. Product Identity
- **Go-to-market name:** The public-facing name (e.g., "CruxDev" not "cruxdev", "TrueAssess.me" not "trueassess")
- **Project name:** The folder/repo name (used internally)
- **Tagline:** One-line description for meta tags and hero
- **Domain:** Target domain if known

### 2. Logo
- **Logo file path:** SVG preferred, PNG acceptable. The engine will proceed without a logo but will flag it as a gap.
- **Favicon:** Auto-generated from logo if provided

### 3. Engine Selection
Present the matrix below. Recommend based on project type:
- **Content/marketing site** → Astro or 11ty
- **Blog with memberships** → Ghost
- **Docs site** → Astro (Starlight) or 11ty
- **Interactive webapp** → SvelteKit, Next.js, or Phoenix LiveView
- **Visual editing needed** → TinaCMS + Astro, or Ghost
- **Elixir project** → Phoenix with Still or Sitex
- **Rust project** → Zola
- **Maximum portability** → 11ty (pure markdown, minimal lock-in)

---

## Engine Matrix

### Static Site Generators (Content-First)

| Engine | Language | Best For | Visual Editor | Lock-in Risk | Build Speed | JS Shipped |
|--------|----------|----------|--------------|-------------|-------------|-----------|
| **Astro** | Node.js/TS | Marketing sites, docs, blogs | Via TinaCMS/Storyblok | Medium (.astro syntax) | Fast | Zero by default |
| **11ty (Eleventy)** | Node.js | Blogs, portfolios, docs | Via CloudCannon | **Low** (pure templates) | Fast | Zero |
| **Hugo** | Go | Blogs, docs, large sites | Via CloudCannon/Forestry | Low (Go templates) | **Fastest** (<1ms/page) | Zero |
| **Zola** | Rust | Blogs, docs | None built-in | Low (Tera templates) | **Fastest** (36ms/50 pages) | Zero |
| **Cobalt** | Rust | Simple sites | None | Low | Very fast | Zero |

### Full-Stack Frameworks (Hybrid SSG+SSR)

| Engine | Language | Best For | Visual Editor | Lock-in Risk | Build Speed | JS Shipped |
|--------|----------|----------|--------------|-------------|-------------|-----------|
| **Next.js** | React/TS | Full-stack apps, ISR | Via Sanity/Builder.io | High (React ecosystem) | Moderate | Framework runtime |
| **Nuxt** | Vue/TS | Vue apps, hybrid rendering | Via Nuxt Studio | Medium (Vue) | Fast (Vite) | Framework runtime |
| **SvelteKit** | Svelte/TS | Interactive sites, apps | None built-in | Low (compiles away) | Fast (Vite) | **50-70% less than React** |
| **Remix** | React/TS | Form-heavy apps, web standards | None built-in | Medium (React) | Fast | Framework runtime |

### CMS Platforms (Content Management Built-In)

| Engine | Language | Best For | Visual Editor | Lock-in Risk | Self-Hosted | Memberships |
|--------|----------|----------|--------------|-------------|-------------|------------|
| **Ghost** | Node.js | Blogs, newsletters, paid content | **Yes (built-in)** | Medium (Ghost API) | Yes | **Yes (built-in)** |
| **TinaCMS** | Node.js | Git-based visual editing | **Yes (live editing)** | Low (Git-backed) | Yes | No |
| **Builder.io** | Any | Visual page building | **Yes (drag-drop)** | High (SaaS) | No | No |
| **Webflow** | N/A | Design-first marketing sites | **Yes (full WYSIWYG)** | **High** (proprietary) | No | No |
| **Framer** | N/A | Design-first sites | **Yes (full WYSIWYG)** | **High** (proprietary) | No | No |

### Elixir Ecosystem

| Engine | Best For | Maturity | Notes |
|--------|----------|----------|-------|
| **Phoenix LiveView** | Interactive webapps | Production | Full-stack, real-time, server-rendered. Best for apps, not static sites. |
| **Still** | Static sites | Early | Composable Elixir static site generator |
| **Sitex** | Static sites | Early | SSG with Elixir & Phoenix |
| **PhoenixPrerender** | SSG from Phoenix | New (2026) | ISR for Phoenix — like Next.js ISR but on BEAM |

### Rust Ecosystem

| Engine | Best For | Maturity | Notes |
|--------|----------|----------|-------|
| **Zola** | Docs, blogs | Production | Single binary, 4x faster than Hugo in benchmarks, built-in search/i18n |
| **Cobalt** | Simple sites | Stable | Minimalist, data loading in templates |

---

## Detailed Profiles

### Astro
- **URL:** https://astro.build
- **GitHub:** 50K+ stars
- **License:** MIT (acquired by Cloudflare Jan 2026)
- **Runtime:** Node.js, Vite, TypeScript
- **Key feature:** Island architecture — interactive components hydrate independently, rest is static HTML
- **Strengths:** Zero JS by default, multi-framework (React/Vue/Svelte/Solid on same page), best-in-class SEO (Lighthouse 100), content collections, image optimization
- **Weaknesses:** Not for SPAs/interactive apps, .astro syntax is proprietary (portability concern), smaller ecosystem than Next.js, some dev/prod inconsistencies reported
- **When to use:** Marketing sites, documentation, blogs, content-heavy sites where performance and SEO matter most
- **When NOT to use:** Dashboards, admin panels, highly interactive apps, real-time features

### 11ty (Eleventy)
- **URL:** https://www.11ty.dev
- **GitHub:** 17K+ stars
- **License:** MIT
- **Runtime:** Node.js
- **Key feature:** Maximum flexibility — 11 template languages, zero opinions about CSS/JS, pure HTML output
- **Strengths:** **Lowest lock-in** of any SSG (content stays as plain markdown/HTML), zero JS by default, fast builds, progressive enhancement philosophy
- **Weaknesses:** No built-in component system (BYO), steeper learning curve for beginners, smaller community than Astro/Next.js, no built-in image optimization
- **When to use:** When portability matters most, progressive enhancement, marketing sites, blogs
- **When NOT to use:** Teams wanting batteries-included DX, projects needing interactive components

### Hugo
- **URL:** https://gohugo.io
- **GitHub:** 78K+ stars
- **License:** Apache 2.0
- **Runtime:** Go (single binary)
- **Key feature:** **Fastest build times** — claims <1ms per page, 300+ themes
- **Strengths:** Blazing fast builds, huge theme ecosystem, single binary (no npm), excellent for large sites (10K+ pages)
- **Weaknesses:** Go template syntax has steep learning curve, limited JS interactivity story, harder to extend than Node-based tools
- **When to use:** Large documentation sites, blogs with thousands of posts, teams that want speed above all
- **When NOT to use:** Teams unfamiliar with Go templates, sites needing interactive components

### Zola
- **URL:** https://www.getzola.org
- **GitHub:** 14K+ stars
- **License:** MIT
- **Runtime:** Rust (single binary)
- **Key feature:** Single binary with everything built-in — Sass, syntax highlighting, search, i18n
- **Strengths:** **4x faster than Hugo** in benchmarks (36ms for 50 pages), no dependencies, built-in search index, multilingual support
- **Weaknesses:** Smaller ecosystem, fewer themes than Hugo, Tera template language less known
- **When to use:** Rust projects, performance-critical builds, sites wanting single-binary simplicity
- **When NOT to use:** Teams needing large theme ecosystem, projects requiring custom JS build pipelines

### Ghost
- **URL:** https://ghost.org
- **GitHub:** 48K+ stars
- **License:** MIT
- **Runtime:** Node.js, MySQL
- **Key feature:** **Built-in memberships, newsletters, and paid subscriptions** — not just a CMS
- **Strengths:** Beautiful editor, built-in SEO, built-in email newsletters, paid membership support, headless API, self-hostable
- **Weaknesses:** Requires Node.js + MySQL (heavier than static generators), limited customization vs code-first tools, theme system can be constraining
- **When to use:** Blogs, newsletters, paid content, indie publishing, when you need a complete publishing platform
- **When NOT to use:** Simple marketing pages, documentation sites, projects that don't need CMS features

### SvelteKit
- **URL:** https://svelte.dev
- **GitHub:** 19K+ stars (kit), 80K+ (svelte)
- **License:** MIT
- **Runtime:** Node.js, Vite
- **Key feature:** Compiles away — **ships 50-70% less JS** than React-based frameworks
- **Strengths:** Best performance for interactive sites, excellent DX, Vite-based, SSG+SSR+SPA in one framework, web standards aligned
- **Weaknesses:** Smaller ecosystem than React, fewer tutorials/resources, less enterprise adoption
- **When to use:** Interactive sites that also need SEO, apps that must be fast, teams that value DX
- **When NOT to use:** Teams invested in React ecosystem, enterprise environments requiring React hiring pool

### Next.js
- **URL:** https://nextjs.org
- **GitHub:** 130K+ stars
- **License:** MIT
- **Runtime:** Node.js, React
- **Key feature:** Most complete full-stack React framework — SSG, SSR, ISR, edge, API routes
- **Strengths:** Largest ecosystem, most tutorials/resources, Vercel backing, ISR for dynamic content, enterprise adoption
- **Weaknesses:** Heavy JS bundle, complex configuration, Vercel lock-in concerns, slower builds than Vite-based tools
- **When to use:** Full-stack apps, teams with React expertise, enterprise projects, when ecosystem size matters
- **When NOT to use:** Simple content sites (overkill), performance-critical sites, when bundle size matters

### TinaCMS
- **URL:** https://tina.io
- **License:** Apache 2.0
- **Key feature:** **Visual editing on your live site** backed by Git — non-developers can edit content directly
- **Strengths:** Git-backed (content stays in your repo), visual editing, works with any SSG (Astro, Next.js, Hugo), open source
- **Weaknesses:** Adds complexity to your build, Git knowledge still needed for setup, smaller community
- **When to use:** When non-technical team members need to edit content on the live site
- **Pairs with:** Astro, Next.js, Hugo, any SSG

### Phoenix LiveView (Elixir)
- **URL:** https://www.phoenixframework.org
- **License:** MIT
- **Runtime:** Elixir/BEAM
- **Key feature:** Real-time interactive UIs without writing JavaScript — server-rendered with WebSocket updates
- **Strengths:** Real-time by default, fault-tolerant (BEAM), excellent for interactive apps, no JS framework needed
- **Weaknesses:** Not a static site generator (server required), Elixir talent pool smaller, hosting more complex
- **When to use:** Elixir projects, real-time features, interactive webapps, when you want server-rendered interactivity
- **When NOT to use:** Simple static marketing sites, projects that need CDN-only hosting

---

## Migration Paths

When switching engines, the key factor is **content portability**:

| From | To | Difficulty | Content Migration |
|------|-----|-----------|-------------------|
| Astro → 11ty | Easy | Markdown files copy directly. Remove .astro components, replace with 11ty templates. |
| Astro → Hugo | Moderate | Markdown copies. Frontmatter compatible. Templates must be rewritten in Go. |
| Astro → Zola | Moderate | Markdown copies. Templates rewritten in Tera. |
| 11ty → Astro | Easy | Markdown copies. Add .astro layouts. |
| Hugo → Astro | Moderate | Markdown copies. Go templates → .astro components. |
| Ghost → Any SSG | Moderate | Export via Ghost API, convert to markdown. Memberships don't migrate. |
| Next.js → Astro | Hard | React components → .astro components. State management doesn't transfer. |
| Any SSG → Ghost | Moderate | Import markdown as posts. Need to rebuild templates in Ghost theme system. |

**Lowest lock-in ranking:** 11ty > Hugo > Zola > Astro > SvelteKit > Ghost > Next.js > Webflow/Framer

---

## Engine Selection Decision Tree

```
Does the project need:
├─ Interactive app features (real-time, auth, dashboard)?
│  ├─ Elixir project? → Phoenix LiveView
│  ├─ Want minimal JS? → SvelteKit
│  └─ React team? → Next.js or Remix
├─ Built-in memberships/newsletters?
│  └─ Ghost
├─ Visual editing for non-developers?
│  ├─ Git-backed? → TinaCMS + (Astro or Hugo)
│  └─ Full WYSIWYG? → Webflow or Framer (⚠ high lock-in)
├─ Static content site (marketing, docs, blog)?
│  ├─ Maximum portability? → 11ty
│  ├─ Rust project? → Zola
│  ├─ Need multi-framework components? → Astro
│  ├─ Thousands of pages? → Hugo
│  └─ Default recommendation → Astro
└─ Unsure? → Start with Astro (good default, moderate migration path)
```

---

## Sources

- [Astro](https://astro.build) | [Docs](https://docs.astro.build)
- [11ty](https://www.11ty.dev) | [Docs](https://www.11ty.dev/docs/)
- [Hugo](https://gohugo.io) | [Docs](https://gohugo.io/documentation/)
- [Zola](https://www.getzola.org) | [Docs](https://www.getzola.org/documentation/)
- [Ghost](https://ghost.org) | [Docs](https://ghost.org/docs/)
- [SvelteKit](https://svelte.dev) | [Docs](https://svelte.dev/docs/kit)
- [Next.js](https://nextjs.org) | [Docs](https://nextjs.org/docs)
- [TinaCMS](https://tina.io) | [Docs](https://tina.io/docs/)
- [Phoenix](https://www.phoenixframework.org) | [LiveView](https://hexdocs.pm/phoenix_live_view/)
- [Cobalt](https://cobalt-org.github.io)
- [CloudCannon Comparison](https://cloudcannon.com/blog/eleventy-11ty-vs-astro/)
- [Hygraph SSG Roundup](https://hygraph.com/blog/top-12-ssgs)
- [Rust SSG Benchmarks](https://dasroot.net/posts/2026/02/static-site-generators-rust-performance-ecosystem-use-cases/)
