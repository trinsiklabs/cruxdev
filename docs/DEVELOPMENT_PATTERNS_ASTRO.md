# Development Patterns — Astro Stack

Astro / Content Collections / Islands Architecture / Tailwind / React / Vue / Svelte / Vitest / Playwright

This document captures stack-specific patterns, conventions, and decisions for Astro-based projects (static sites, content-driven sites, hybrid SSR apps). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in Astro and integrating frontend frameworks via islands architecture.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **COLOR_CONTRAST_PATTERNS.md** — color contrast requirements, WCAG compliance, light/dark mode tokens.
- **This file** — stack-specific patterns. How we structure Astro projects, use content collections, integrate UI frameworks, test with Vitest and Playwright, deploy to Cloudflare/Vercel/Node, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Astro | 5+ (currently 6.x) | Static-first, islands architecture, content collections |
| Node.js | 22+ | LTS required, ES modules (`"type": "module"`) |
| TypeScript | 5.x | Strict mode via `astro/tsconfigs/strict` |
| Tailwind CSS | 4.x | Via `@tailwindcss/vite` plugin — CSS-based config, no JS config file |
| @astrojs/sitemap | 3.x | Auto-generates sitemap.xml from all pages |
| @astrojs/rss | 4.x | RSS feed generation for blog/content |
| @astrojs/react | 4.x | React islands (when needed) |
| @astrojs/vue | 5.x | Vue islands (when needed) |
| @astrojs/svelte | 7.x | Svelte islands (when needed) |
| @astrojs/cloudflare | 12.x | Cloudflare Pages/Workers adapter |
| @astrojs/vercel | 8.x | Vercel adapter |
| @astrojs/node | 9.x | Self-hosted Node adapter |
| Pagefind | 1.x | Post-build static search index |
| Vitest | 3.x | Unit + component testing |
| Playwright | 1.x | E2E testing |
| sharp | 0.34+ | Image optimization (auto-detected by Astro) |

### Version Constraint Policy

Use caret (`^`) constraints in `package.json` pinned to the minor version:

```json
{
  "dependencies": {
    "astro": "^6.0.8",
    "@astrojs/sitemap": "^3.7.1",
    "@tailwindcss/vite": "^4.2.2",
    "tailwindcss": "^4.2.2"
  }
}
```

**Rules:**
- Pin major version — Astro has breaking changes between majors (4 -> 5, 5 -> 6).
- Use `^` for patch/minor safety within a major.
- Lock file (`package-lock.json` or `pnpm-lock.yaml`) is committed and authoritative.
- `engines.node` field is mandatory — prevents accidental use of unsupported Node versions.

```json
{
  "engines": {
    "node": ">=22.12.0"
  }
}
```

### ES Module Configuration

All Astro projects must use ES modules:

```json
{
  "type": "module"
}
```

This is non-negotiable. Astro's build pipeline, Vite, and all modern tooling require ESM. CommonJS (`require()`) is not supported in Astro config files, layouts, or components.

---

## 2. Project Structure

### Standard Astro Project Layout

```
project-root/
├── astro.config.mjs          # Astro configuration (integrations, adapters, Vite plugins)
├── tsconfig.json              # TypeScript config (extends astro/tsconfigs/strict)
├── package.json               # Dependencies, scripts, engine constraints
├── public/                    # Static assets (copied verbatim to output)
│   ├── favicon.ico
│   ├── favicon.svg
│   ├── apple-touch-icon.png
│   ├── manifest.webmanifest
│   ├── robots.txt
│   └── og-image.png
├── src/
│   ├── components/            # Reusable Astro + framework components
│   │   ├── Header.astro       # Site-wide navigation
│   │   ├── Footer.astro       # Site-wide footer
│   │   ├── SEOHead.astro      # Reusable SEO meta tags
│   │   ├── Card.astro         # UI components
│   │   └── react/             # React island components (when needed)
│   │       └── SearchWidget.tsx
│   ├── content/               # Content collections (Astro 5+ type-safe content)
│   │   └── config.ts          # Collection schemas (Zod-based)
│   ├── content/blog/          # Blog collection entries
│   │   ├── first-post.md
│   │   └── second-post.mdx
│   ├── layouts/               # Page layouts (composition, not inheritance)
│   │   ├── Base.astro         # Root HTML shell (head, nav, footer, scripts)
│   │   ├── Doc.astro          # Documentation pages
│   │   └── BlogPost.astro     # Blog post layout
│   ├── pages/                 # File-based routing
│   │   ├── index.astro        # Homepage
│   │   ├── blog/
│   │   │   ├── index.astro    # Blog listing
│   │   │   ├── [slug].astro   # Dynamic blog post pages (content collection)
│   │   │   └── tag/
│   │   │       └── [tag].astro # Tag-filtered listing
│   │   ├── docs/
│   │   │   ├── index.astro
│   │   │   └── quickstart.astro
│   │   └── rss.xml.ts         # RSS feed endpoint
│   ├── styles/
│   │   └── global.css         # Global styles, CSS custom properties, Tailwind imports
│   └── lib/                   # Shared utilities (pure TypeScript, no framework deps)
│       ├── constants.ts       # Site metadata, URLs, shared strings
│       ├── utils.ts           # Date formatting, slug generation, helpers
│       └── types.ts           # Shared TypeScript interfaces
├── tests/                     # Test files
│   ├── unit/                  # Vitest unit tests
│   │   └── utils.test.ts
│   ├── components/            # Vitest component tests
│   │   └── Card.test.ts
│   └── e2e/                   # Playwright E2E tests
│       ├── navigation.spec.ts
│       └── blog.spec.ts
├── vitest.config.ts           # Vitest configuration
├── playwright.config.ts       # Playwright configuration
└── .github/
    └── workflows/
        └── ci.yml             # CI pipeline
```

### Conventions

- **`src/components/`** — reusable UI. Astro components for static UI, framework components (React/Vue/Svelte) only when interactivity requires client-side JS.
- **`src/content/`** — type-safe content collections. Markdown, MDX, JSON, YAML. Never put content in `src/pages/` when it should be a collection.
- **`src/layouts/`** — page shells. Layouts compose via slot, not inheritance. A layout wraps content; it does not extend another layout.
- **`src/pages/`** — file-based routing only. Each `.astro`, `.md`, `.mdx`, or `.ts` file becomes a route.
- **`src/lib/`** — pure TypeScript utilities. No Astro imports, no framework imports. Testable in isolation with Vitest.
- **`src/styles/`** — global CSS only. Component-scoped styles use `<style>` blocks inside `.astro` files.
- **`public/`** — static assets served verbatim. No processing. Use `src/assets/` for images that need optimization.
- **`tests/`** — mirrors `src/` structure for discoverability.

### Layout Composition Pattern

Layouts compose through Astro's `<slot />` mechanism. The Base layout provides the HTML shell; specialized layouts extend it by wrapping content:

```astro
---
// src/layouts/Doc.astro
import Base from './Base.astro'

interface Props {
  title: string
  description: string
  breadcrumb?: string
}

const { title, description, breadcrumb } = Astro.props
---

<Base title={title} description={description}>
  <section class="py-16">
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
      {breadcrumb && (
        <nav class="text-sm mb-6" aria-label="Breadcrumb">
          <a href="/">Home</a> / <a href="/docs">Docs</a> / <span>{breadcrumb}</span>
        </nav>
      )}
      <article class="doc-content">
        <slot />
      </article>
    </div>
  </section>
</Base>
```

**Convention:** The Base layout owns `<html>`, `<head>`, `<body>`, `<nav>`, `<footer>`, and global scripts. Specialized layouts (Doc, BlogPost) wrap their content within Base. This is composition, not template inheritance.

---

## 3. Content Collections

### Overview

Astro 5+ content collections provide type-safe content management with Zod schemas. They replace the older `Astro.glob()` pattern for structured content.

### Collection Schema Definition

```typescript
// src/content/config.ts
import { defineCollection, z } from 'astro:content';

const blog = defineCollection({
  type: 'content',  // Markdown/MDX content
  schema: z.object({
    title: z.string(),
    date: z.string().datetime(),
    summary: z.string(),
    tags: z.array(z.string()).default([]),
    draft: z.boolean().default(false),
    author: z.string().default('Trinsik Labs'),
    image: z.string().optional(),  // OG image path
    canonical: z.string().url().optional(),  // External canonical URL
  }),
});

const docs = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    description: z.string(),
    order: z.number().default(999),
    section: z.string().default('general'),
  }),
});

const changelog = defineCollection({
  type: 'data',  // JSON/YAML data
  schema: z.object({
    version: z.string(),
    date: z.string().datetime(),
    changes: z.array(z.object({
      type: z.enum(['added', 'changed', 'fixed', 'removed']),
      description: z.string(),
    })),
  }),
});

export const collections = { blog, docs, changelog };
```

### Querying Collections

```astro
---
// src/pages/blog/index.astro
import { getCollection } from 'astro:content';

const posts = (await getCollection('blog', ({ data }) => !data.draft))
  .sort((a, b) => new Date(b.data.date).getTime() - new Date(a.data.date).getTime());
---
```

### Dynamic Routes from Collections

```astro
---
// src/pages/blog/[slug].astro
import { getCollection, render } from 'astro:content';
import BlogPost from '../../layouts/BlogPost.astro';

export async function getStaticPaths() {
  const posts = await getCollection('blog', ({ data }) => !data.draft);
  return posts.map(post => ({
    params: { slug: post.id },
    props: { post },
  }));
}

const { post } = Astro.props;
const { Content } = await render(post);
---

<BlogPost frontmatter={post.data}>
  <Content />
</BlogPost>
```

### Content Collection vs Astro.glob

| Use Case | Approach | Why |
|---|---|---|
| Structured content (blog, docs, changelog) | Content collections | Type-safe, validated, queryable |
| One-off pages (about, pricing, landing) | `src/pages/*.astro` | Direct page files — no collection needed |
| Dynamic data from API | `fetch()` in frontmatter | Content collections are for local files only |
| Legacy migration (no schema yet) | `import.meta.glob()` | Temporary — migrate to collections |

**Convention:** All new content must use content collections. `import.meta.glob()` is only acceptable during migration from legacy patterns. The `Astro.glob()` function was removed in Astro 5 — use `import.meta.glob()` as the Vite-native alternative.

### Frontmatter Standards

Every content collection entry must have:

```yaml
---
title: "Descriptive title for the page and OG tags"
date: "2026-03-27T00:00:00Z"   # ISO 8601, always with timezone
summary: "One sentence. Used for meta description and listing cards."
tags: ["astro", "patterns"]      # Lowercase, hyphenated
draft: false                      # Explicit — never rely on absence
---
```

**Rules:**
- Dates are always ISO 8601 with timezone. Never `2026-03-27` without a time component in the schema (use `.datetime()` in Zod).
- Tags are lowercase, hyphenated, and consistent across the collection. No duplicates (`astro` and `Astro` are not allowed).
- The `draft` field is explicit. A missing `draft` field does not mean "published" — the schema default handles this.
- Summaries are exactly one sentence. They appear in meta descriptions, OG tags, and listing cards.

---

## 4. Islands Architecture

### Philosophy

Astro's core innovation is the islands architecture: pages are static HTML by default, with interactive "islands" of JavaScript where needed. This delivers zero-JS pages unless you explicitly opt in.

### The Hydration Spectrum

| Directive | Behavior | Use Case |
|---|---|---|
| (none) | Server-rendered only, zero JS sent | Static content, layout, cards, headers |
| `client:load` | Hydrates immediately on page load | Above-the-fold interactive elements, critical UI |
| `client:idle` | Hydrates when browser is idle | Below-the-fold widgets, analytics, chat |
| `client:visible` | Hydrates when scrolled into viewport | Lazy-loaded interactive sections, carousels |
| `client:media` | Hydrates when media query matches | Mobile-only or desktop-only interactive features |
| `client:only="react"` | Renders only on client, no SSR | Components that depend on browser APIs (localStorage, window) |

### Framework Integration

Install framework integrations as needed:

```bash
# React islands
npx astro add react

# Vue islands
npx astro add vue

# Svelte islands
npx astro add svelte

# Multiple frameworks in one project (valid and supported)
npx astro add react vue svelte
```

Configuration:

```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import vue from '@astrojs/vue';
import svelte from '@astrojs/svelte';

export default defineConfig({
  integrations: [react(), vue(), svelte()],
});
```

### Island Component Patterns

**React island:**

```tsx
// src/components/react/SearchWidget.tsx
import { useState, useEffect } from 'react';

interface Props {
  placeholder?: string;
  apiEndpoint: string;
}

export default function SearchWidget({ placeholder = 'Search...', apiEndpoint }: Props) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);

  useEffect(() => {
    if (query.length < 2) return;
    const controller = new AbortController();
    fetch(`${apiEndpoint}?q=${encodeURIComponent(query)}`, { signal: controller.signal })
      .then(r => r.json())
      .then(setResults)
      .catch(() => {});
    return () => controller.abort();
  }, [query, apiEndpoint]);

  return (
    <div>
      <input
        type="search"
        value={query}
        onChange={e => setQuery(e.target.value)}
        placeholder={placeholder}
        aria-label="Search"
      />
      <ul role="listbox">
        {results.map((r: any) => (
          <li key={r.id} role="option">{r.title}</li>
        ))}
      </ul>
    </div>
  );
}
```

**Using the island in an Astro page:**

```astro
---
import SearchWidget from '../components/react/SearchWidget';
---

<!-- Static content: zero JS -->
<h1>Documentation</h1>
<p>Browse our documentation or search for what you need.</p>

<!-- Interactive island: hydrates when visible -->
<SearchWidget
  client:visible
  apiEndpoint="/api/search"
  placeholder="Search docs..."
/>

<!-- More static content: still zero JS -->
<footer>Built with Astro</footer>
```

### When to Use Islands vs Pure Astro

| Requirement | Solution |
|---|---|
| Static text, images, cards, grids | Pure Astro component (zero JS) |
| Toggle visibility (accordion, FAQ) | `<details>` / `<summary>` HTML (zero JS) |
| Theme toggle | Inline `<script is:inline>` (minimal JS, no framework) |
| Dark/light mode | CSS `prefers-color-scheme` + inline script for toggle |
| Navigation menu (mobile) | Inline `<script>` with event listener (no framework needed) |
| Search with autocomplete | React/Vue/Svelte island with `client:visible` |
| Form with complex validation | React/Vue/Svelte island with `client:load` |
| Real-time dashboard | React/Vue/Svelte island with `client:load` |
| Animation on scroll | CSS `@keyframes` + `IntersectionObserver` (inline script) |
| Data visualization (charts) | React/Svelte island with `client:visible` |

**Decision rule:** If it can be done with HTML, CSS, or a small inline script, do NOT use a framework island. Every island adds JS bundle weight. A page with zero islands loads faster than any SPA.

### Sharing State Between Islands

Islands are isolated by default. If two islands need shared state:

1. **Custom Events** — vanilla DOM events. Best for loose coupling.
2. **Nano Stores** — framework-agnostic reactive stores. Works across React, Vue, Svelte islands.
3. **URL state** — query parameters or hash. Best for shareable/bookmarkable state.

```typescript
// src/lib/stores.ts (using nanostores)
import { atom } from 'nanostores';

export const searchQuery = atom('');
export const selectedTag = atom<string | null>(null);
```

```tsx
// React island reads/writes the store
import { useStore } from '@nanostores/react';
import { searchQuery } from '../../lib/stores';

export function SearchInput() {
  const query = useStore(searchQuery);
  return <input value={query} onChange={e => searchQuery.set(e.target.value)} />;
}
```

```svelte
<!-- Svelte island reads the same store -->
<script>
  import { searchQuery } from '../../lib/stores';
</script>

<p>Searching for: {$searchQuery}</p>
```

**Convention:** Prefer custom DOM events for simple communication. Only use nanostores when multiple islands need reactive shared state. Never use React Context or Vue Provide/Inject across islands — they don't work across framework boundaries.

---

## 5. Tailwind CSS Integration

### Astro + Tailwind 4.x Setup

Tailwind 4.x uses CSS-based configuration (not the JavaScript `tailwind.config.js` from v3). In Astro, Tailwind is integrated via the Vite plugin:

```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  vite: {
    plugins: [tailwindcss()],
  },
});
```

```css
/* src/styles/global.css */
@import "tailwindcss";

@theme {
  --color-accent: #2563EB;
  --color-accent-hover: #1D4ED8;
  /* Project-specific design tokens */
}
```

Import the global CSS in the Base layout:

```astro
---
// src/layouts/Base.astro
import '../styles/global.css'
---
```

### Design Token Pattern (CSS Custom Properties)

Define a comprehensive token system for light and dark modes:

```css
/* src/styles/global.css */
@import "tailwindcss";

@theme {
  --color-accent: #2563EB;
  --color-accent-hover: #1D4ED8;
}

/* Light mode (default) */
:root {
  --bg-primary: #FAFAFA;
  --bg-secondary: #F3F4F6;
  --bg-surface: #FFFFFF;
  --bg-nav: rgba(250, 250, 250, 0.95);
  --text-primary: #111827;
  --text-secondary: #4B5563;
  --text-tertiary: #6B7280;
  --text-heading: #09090B;
  --border-color: #E5E7EB;
  --border-subtle: #F3F4F6;
  --border-strong: #D1D5DB;
  --shadow-nav: 0 1px 3px rgba(0,0,0,0.06);
}

/* Dark mode */
.dark {
  --bg-primary: #09090B;
  --bg-secondary: #18181B;
  --bg-surface: #18181B;
  --bg-nav: rgba(9, 9, 11, 0.95);
  --text-primary: #E4E4E7;
  --text-secondary: #A1A1AA;
  --text-tertiary: #71717A;
  --text-heading: #FAFAFA;
  --border-color: #27272A;
  --border-subtle: #18181B;
  --border-strong: #3F3F46;
  --shadow-nav: 0 1px 3px rgba(0,0,0,0.3);
}
```

**Convention:** Use CSS custom properties for all colors that change between light and dark modes. Use Tailwind's `@theme` for static design tokens (accent colors, spacing scales). Reference custom properties with `var()` in component styles or via Tailwind's arbitrary value syntax: `text-[var(--text-heading)]`, `bg-[var(--bg-surface)]`.

### Scoped Styles vs Global Styles

| Scope | When to Use | How |
|---|---|---|
| Component-scoped | Styles specific to one Astro component | `<style>` block in `.astro` file (auto-scoped) |
| Global (targeted) | Styles for rendered Markdown/MDX content | `<style is:global>` in layout components |
| Global (site-wide) | Design tokens, resets, base typography | `src/styles/global.css` |

```astro
<!-- Component-scoped: only affects this component -->
<style>
  .card { border-radius: 0.75rem; }
</style>

<!-- Global: affects all children (needed for Markdown rendering) -->
<style is:global>
  .doc-content h2 { font-size: 1.5rem; font-weight: 700; }
  .doc-content p { line-height: 1.75; }
</style>
```

**Convention:** Use `<style is:global>` sparingly and only in layout components that render user content (Markdown, MDX). All other components use scoped styles or Tailwind utility classes.

---

## 6. SEO & Meta Tags

### Base Layout SEO Pattern

Every page must have proper meta tags. The Base layout handles the common ones:

```astro
---
// src/layouts/Base.astro
interface Props {
  title: string
  description: string
  image?: string
  type?: string
  canonical?: string
}

const { title, description, image, type = 'website', canonical } = Astro.props
const siteUrl = 'https://example.com'
const fullTitle = title === 'Home' ? 'Site Name' : `${title} | Site Name`
const canonicalUrl = canonical || new URL(Astro.url.pathname, siteUrl).href
const ogImage = image ? new URL(image, siteUrl).href : new URL('/og-image.png', siteUrl).href
---

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{fullTitle}</title>
    <meta name="description" content={description} />
    <link rel="canonical" href={canonicalUrl} />

    <!-- Open Graph -->
    <meta property="og:type" content={type} />
    <meta property="og:url" content={canonicalUrl} />
    <meta property="og:title" content={fullTitle} />
    <meta property="og:description" content={description} />
    <meta property="og:image" content={ogImage} />

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content={fullTitle} />
    <meta name="twitter:description" content={description} />
    <meta name="twitter:image" content={ogImage} />

    <!-- Favicons -->
    <link rel="icon" href="/favicon.ico" sizes="32x32" />
    <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
    <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
    <link rel="manifest" href="/manifest.webmanifest" />

    <!-- RSS -->
    <link rel="alternate" type="application/rss+xml" title="Blog" href="/blog/rss.xml" />
  </head>
  <body>
    <slot />
  </body>
</html>
```

### Structured Data (JSON-LD)

Every page type gets appropriate structured data:

```astro
<!-- Homepage: SoftwareApplication -->
<script type="application/ld+json" set:html={JSON.stringify({
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "ProjectName",
  "applicationCategory": "DeveloperApplication",
  "url": siteUrl,
  "description": description,
  "author": { "@type": "Organization", "name": "Company Name" },
  "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD" }
})} />

<!-- Blog post: BlogPosting -->
<script type="application/ld+json" set:html={JSON.stringify({
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": title,
  "description": summary,
  "datePublished": date,
  "author": { "@type": "Organization", "name": "Company Name" },
  "publisher": { "@type": "Organization", "name": "Company Name", "url": siteUrl },
  "mainEntityOfPage": canonicalUrl,
  "keywords": tags.join(', '),
})} />

<!-- Documentation: TechArticle -->
<script type="application/ld+json" set:html={JSON.stringify({
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "headline": title,
  "description": description,
  "author": { "@type": "Organization", "name": "Company Name" },
})} />
```

**Convention:** Use `set:html` for JSON-LD injection. This bypasses Astro's HTML escaping, which would corrupt the JSON. Never use `{JSON.stringify(...)}` inside a text node — it will be escaped.

### @astrojs/sitemap Integration

```javascript
// astro.config.mjs
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://example.com',  // REQUIRED for sitemap generation
  integrations: [
    sitemap({
      filter: (page) => !page.includes('/draft/') && !page.includes('/internal/'),
      changefreq: 'weekly',
      priority: 0.7,
      lastmod: new Date(),
    }),
  ],
});
```

**Rules:**
- The `site` field in `astro.config.mjs` is mandatory. Without it, sitemap generation silently fails.
- Filter out draft pages, internal pages, and any routes not meant for search engines.
- Submit the generated `sitemap-index.xml` to Google Search Console and Bing Webmaster Tools.
- Reference the sitemap in `public/robots.txt`:

```
User-agent: *
Allow: /

Sitemap: https://example.com/sitemap-index.xml
```

### RSS Feed

```typescript
// src/pages/blog/rss.xml.ts
import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';

export async function GET(context: any) {
  const posts = (await getCollection('blog', ({ data }) => !data.draft))
    .sort((a, b) => new Date(b.data.date).getTime() - new Date(a.data.date).getTime());

  return rss({
    title: 'Blog',
    description: 'Latest posts',
    site: context.site,
    items: posts.map(post => ({
      title: post.data.title,
      pubDate: new Date(post.data.date),
      description: post.data.summary,
      link: `/blog/${post.id}/`,
    })),
  });
}
```

---

## 7. Pagefind Search Integration

### Overview

Pagefind is a static search library that indexes your built site and provides instant search with zero server infrastructure. It runs as a post-build step.

### Installation & Setup

```bash
npm install -D pagefind
```

Add the post-build indexing step:

```json
{
  "scripts": {
    "dev": "astro dev",
    "build": "astro build && npx pagefind --site dist",
    "preview": "astro preview"
  }
}
```

### Adding Search UI

Pagefind ships a default UI that works without any framework:

```astro
---
// src/components/Search.astro
---

<div id="search" class="search-container"></div>

<script>
  // Pagefind CSS and JS are loaded from the built index
  async function initSearch() {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = '/pagefind/pagefind-ui.css';
    document.head.appendChild(link);

    // @ts-ignore — Pagefind is injected at build time
    const { PagefindUI } = await import('/pagefind/pagefind-ui.js');

    new PagefindUI({
      element: '#search',
      showSubResults: true,
      showImages: false,
    });
  }

  // Initialize when the component is visible
  const observer = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
      initSearch();
      observer.disconnect();
    }
  });
  observer.observe(document.getElementById('search')!);
</script>

<style is:global>
  .pagefind-ui__search-input {
    background: var(--bg-surface) !important;
    color: var(--text-primary) !important;
    border: 1px solid var(--border-color) !important;
    border-radius: 0.5rem !important;
  }
  .pagefind-ui__result-link {
    color: var(--color-accent) !important;
  }
</style>
```

### Controlling What Gets Indexed

Use `data-pagefind-body` to mark indexable sections and `data-pagefind-ignore` to exclude content:

```astro
<!-- Only index the main content, not nav/footer -->
<nav data-pagefind-ignore>...</nav>

<main data-pagefind-body>
  <article>
    <h1 data-pagefind-meta="title">{title}</h1>
    <p data-pagefind-meta="description">{description}</p>
    <slot />
  </article>
</main>

<footer data-pagefind-ignore>...</footer>
```

### Custom Search with Pagefind API

For advanced search UIs (e.g., a React island):

```tsx
// src/components/react/CustomSearch.tsx
import { useState, useEffect, useRef } from 'react';

export default function CustomSearch() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any[]>([]);
  const pagefindRef = useRef<any>(null);

  useEffect(() => {
    async function load() {
      // @ts-ignore
      pagefindRef.current = await import('/pagefind/pagefind.js');
      await pagefindRef.current.init();
    }
    load();
  }, []);

  useEffect(() => {
    if (!pagefindRef.current || query.length < 2) {
      setResults([]);
      return;
    }
    pagefindRef.current.search(query).then((search: any) => {
      Promise.all(search.results.slice(0, 10).map((r: any) => r.data()))
        .then(setResults);
    });
  }, [query]);

  return (
    <div>
      <input
        type="search"
        value={query}
        onChange={e => setQuery(e.target.value)}
        placeholder="Search..."
        aria-label="Search"
      />
      <ul>
        {results.map((r, i) => (
          <li key={i}>
            <a href={r.url}>{r.meta?.title || r.url}</a>
            <p dangerouslySetInnerHTML={{ __html: r.excerpt }} />
          </li>
        ))}
      </ul>
    </div>
  );
}
```

---

## 8. Image Optimization

### Astro Image Component

Astro provides built-in image optimization via the `<Image />` and `<Picture />` components:

```astro
---
import { Image, Picture } from 'astro:assets';
import heroImage from '../assets/hero.png';
---

<!-- Optimized image: auto-generates WebP/AVIF, sets width/height to prevent CLS -->
<Image src={heroImage} alt="Hero illustration" width={1200} height={630} />

<!-- Responsive picture: serves different formats based on browser support -->
<Picture
  src={heroImage}
  formats={['avif', 'webp']}
  alt="Hero illustration"
  widths={[400, 800, 1200]}
  sizes="(max-width: 640px) 400px, (max-width: 1024px) 800px, 1200px"
/>
```

### Image Rules

- **Always use `<Image>` or `<Picture>` for local images.** Never use raw `<img>` for images in `src/assets/` — you lose optimization.
- **Always include `alt` text.** Empty `alt=""` is valid for decorative images, but never omit the attribute.
- **Always specify `width` and `height`** (or use imported images which have intrinsic dimensions). This prevents Cumulative Layout Shift (CLS).
- **Remote images** need explicit `width` and `height` since Astro cannot introspect them.
- **Use `public/` for images that must not be processed** (favicons, social media avatars, third-party logos with specific format requirements).

```astro
<!-- Remote image: must specify dimensions -->
<Image
  src="https://cdn.example.com/avatar.jpg"
  alt="User avatar"
  width={64}
  height={64}
  inferSize={false}
/>

<!-- Decorative image: empty alt, aria-hidden -->
<Image src={decorativePattern} alt="" aria-hidden="true" width={200} height={200} />
```

### Image Configuration

```javascript
// astro.config.mjs
export default defineConfig({
  image: {
    // Use sharp for local development (default)
    // Cloudflare/Vercel use their own image services in production
    domains: ['cdn.example.com'],  // Allow remote image optimization
    remotePatterns: [
      { protocol: 'https', hostname: '**.example.com' },
    ],
  },
});
```

---

## 9. Dark Mode & Theming

### Flash-Free Dark Mode Pattern

Dark mode must be applied before the first paint to prevent flash-of-wrong-theme (FOWT). This requires an inline script in `<head>`:

```astro
<!-- In Base.astro <head> — MUST be inline, MUST be before any CSS -->
<meta name="color-scheme" content="light dark" />
<script is:inline>
  (function() {
    var s = localStorage.getItem('theme');
    var d = window.matchMedia('(prefers-color-scheme: dark)').matches;
    if (s === 'dark' || (s !== 'light' && d)) document.documentElement.classList.add('dark');
  })();
</script>
```

**Why `is:inline`?** Astro normally bundles scripts into modules that load asynchronously. An async script cannot prevent FOWT — the page renders before the script executes. `is:inline` injects the script directly into the HTML, making it synchronous and blocking. This is the one legitimate use of render-blocking JS.

### Theme Toggle

The toggle cycles through three states: system, light, dark.

```astro
<button id="theme-toggle" aria-label="Toggle theme" title="Toggle theme">
  <svg id="icon-sun" class="w-5 h-5 hidden"><!-- sun icon --></svg>
  <svg id="icon-moon" class="w-5 h-5 hidden"><!-- moon icon --></svg>
  <svg id="icon-system" class="w-5 h-5 hidden"><!-- monitor icon --></svg>
</button>

<script is:inline>
  (function() {
    var themes = ['system', 'light', 'dark'];
    var btn = document.getElementById('theme-toggle');

    function getStored() { return localStorage.getItem('theme') || 'system'; }

    function apply(theme) {
      var isDark = theme === 'dark' ||
        (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
      document.documentElement.classList.toggle('dark', isDark);
      document.getElementById('icon-sun').classList.toggle('hidden', theme !== 'light');
      document.getElementById('icon-moon').classList.toggle('hidden', theme !== 'dark');
      document.getElementById('icon-system').classList.toggle('hidden', theme !== 'system');
    }

    apply(getStored());

    if (btn) btn.addEventListener('click', function() {
      var current = getStored();
      var next = themes[(themes.indexOf(current) + 1) % themes.length];
      localStorage.setItem('theme', next);
      apply(next);
    });

    // Respond to OS preference changes when in system mode
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function() {
      if (getStored() === 'system') apply('system');
    });
  })();
</script>
```

**Convention:** The theme toggle is always an inline script with no framework dependency. It works on every page, including pages with zero JS islands. Never put the theme toggle in a React/Vue/Svelte component — it would not be available until hydration.

---

## 10. Testing Patterns

### Test Pyramid (Astro-specific)

```
        /\
       /  \          E2E (Playwright)
      /    \         Full page loads, navigation, SEO validation, visual regression
     /------\
    /        \        Component Tests (Vitest + testing-library)
   /          \       Astro component rendering, React/Vue/Svelte island testing
  /------------\
 /              \      Unit Tests (Vitest)
/                \     Pure functions, utilities, data transformations
/------------------\
```

### Vitest Configuration

```typescript
// vitest.config.ts
import { getViteConfig } from 'astro/config';

export default getViteConfig({
  test: {
    include: ['tests/unit/**/*.test.ts', 'tests/components/**/*.test.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      include: ['src/lib/**/*.ts'],
      thresholds: {
        statements: 100,
        branches: 100,
        functions: 100,
        lines: 100,
      },
    },
  },
});
```

### Unit Testing (Pure Functions)

Test all utilities in `src/lib/` — these are framework-agnostic and trivially testable:

```typescript
// tests/unit/utils.test.ts
import { describe, it, expect } from 'vitest';
import { formatDate, slugify, estimateReadTime } from '../../src/lib/utils';

describe('formatDate', () => {
  it('formats ISO date to human-readable string', () => {
    expect(formatDate('2026-03-27T00:00:00Z')).toBe('March 27, 2026');
  });

  it('handles invalid date gracefully', () => {
    expect(formatDate('')).toBe('');
    expect(formatDate('not-a-date')).toBe('');
  });
});

describe('slugify', () => {
  it('converts title to URL-safe slug', () => {
    expect(slugify('Hello World!')).toBe('hello-world');
  });

  it('handles consecutive special characters', () => {
    expect(slugify('foo---bar')).toBe('foo-bar');
  });
});

describe('estimateReadTime', () => {
  it('returns minimum 1 min for short content', () => {
    expect(estimateReadTime('Hello world')).toBe('1 min read');
  });

  it('calculates based on 250 words per minute', () => {
    const words = Array(500).fill('word').join(' ');
    expect(estimateReadTime(words)).toBe('2 min read');
  });
});
```

### Component Testing

For React/Vue/Svelte island components, use testing-library:

```typescript
// tests/components/SearchWidget.test.tsx
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import SearchWidget from '../../src/components/react/SearchWidget';

describe('SearchWidget', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('renders search input with placeholder', () => {
    render(<SearchWidget apiEndpoint="/api/search" placeholder="Find docs..." />);
    expect(screen.getByPlaceholderText('Find docs...')).toBeInTheDocument();
  });

  it('fetches results on input', async () => {
    const mockResults = [{ id: '1', title: 'Result One' }];
    vi.spyOn(global, 'fetch').mockResolvedValue({
      json: () => Promise.resolve(mockResults),
    } as Response);

    render(<SearchWidget apiEndpoint="/api/search" />);
    await userEvent.type(screen.getByRole('searchbox'), 'test');

    await waitFor(() => {
      expect(screen.getByText('Result One')).toBeInTheDocument();
    });
  });

  it('debounces fetch calls', async () => {
    const fetchSpy = vi.spyOn(global, 'fetch').mockResolvedValue({
      json: () => Promise.resolve([]),
    } as Response);

    render(<SearchWidget apiEndpoint="/api/search" />);
    await userEvent.type(screen.getByRole('searchbox'), 'ab');

    // Should not fetch until minimum query length
    expect(fetchSpy).not.toHaveBeenCalled();
  });
});
```

### E2E Testing (Playwright)

```typescript
// playwright.config.ts
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  webServer: {
    command: 'npm run preview',
    port: 4321,
    reuseExistingServer: !process.env.CI,
  },
  use: {
    baseURL: 'http://localhost:4321',
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
```

```typescript
// tests/e2e/navigation.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('homepage loads with correct title', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/CruxDev/);
  });

  test('navigation links work', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Docs');
    await expect(page).toHaveURL('/docs');
    await expect(page.locator('h1')).toBeVisible();
  });

  test('mobile menu toggles', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    // Test mobile menu behavior
  });
});

test.describe('SEO', () => {
  test('every page has meta description', async ({ page }) => {
    const pages = ['/', '/docs', '/blog', '/methodology'];
    for (const path of pages) {
      await page.goto(path);
      const desc = await page.getAttribute('meta[name="description"]', 'content');
      expect(desc).toBeTruthy();
      expect(desc!.length).toBeGreaterThan(50);
      expect(desc!.length).toBeLessThan(160);
    }
  });

  test('every page has canonical URL', async ({ page }) => {
    await page.goto('/docs');
    const canonical = await page.getAttribute('link[rel="canonical"]', 'href');
    expect(canonical).toContain('https://');
  });

  test('structured data is valid JSON-LD', async ({ page }) => {
    await page.goto('/');
    const jsonLd = await page.$eval(
      'script[type="application/ld+json"]',
      el => JSON.parse(el.textContent || '{}')
    );
    expect(jsonLd['@context']).toBe('https://schema.org');
    expect(jsonLd['@type']).toBeTruthy();
  });

  test('sitemap.xml exists and is valid', async ({ request }) => {
    const response = await request.get('/sitemap-index.xml');
    expect(response.ok()).toBe(true);
    const body = await response.text();
    expect(body).toContain('<sitemapindex');
  });

  test('RSS feed exists and is valid', async ({ request }) => {
    const response = await request.get('/blog/rss.xml');
    expect(response.ok()).toBe(true);
    const body = await response.text();
    expect(body).toContain('<rss');
  });
});

test.describe('Performance', () => {
  test('no layout shift on page load', async ({ page }) => {
    await page.goto('/');
    // Wait for full load
    await page.waitForLoadState('networkidle');
    const cls = await page.evaluate(() => {
      return new Promise<number>((resolve) => {
        let clsValue = 0;
        const observer = new PerformanceObserver((list) => {
          for (const entry of list.getEntries()) {
            if (!(entry as any).hadRecentInput) {
              clsValue += (entry as any).value;
            }
          }
        });
        observer.observe({ type: 'layout-shift', buffered: true });
        setTimeout(() => {
          observer.disconnect();
          resolve(clsValue);
        }, 1000);
      });
    });
    expect(cls).toBeLessThan(0.1);  // Good CLS threshold
  });
});

test.describe('Accessibility', () => {
  test('page has lang attribute', async ({ page }) => {
    await page.goto('/');
    const lang = await page.getAttribute('html', 'lang');
    expect(lang).toBe('en');
  });

  test('images have alt text', async ({ page }) => {
    await page.goto('/');
    const images = await page.locator('img').all();
    for (const img of images) {
      const alt = await img.getAttribute('alt');
      expect(alt).not.toBeNull();  // alt="" is valid for decorative, but attribute must exist
    }
  });

  test('interactive elements have accessible names', async ({ page }) => {
    await page.goto('/');
    const buttons = await page.locator('button').all();
    for (const button of buttons) {
      const name = await button.getAttribute('aria-label') || await button.textContent();
      expect(name?.trim()).toBeTruthy();
    }
  });
});
```

### Test Commands

```json
{
  "scripts": {
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage",
    "test:e2e": "playwright test",
    "test:e2e:headed": "playwright test --headed",
    "test:all": "vitest run && playwright test"
  }
}
```

---

## 11. Deployment Patterns

### Output Modes

Astro supports three output modes:

| Mode | Config | Adapter Needed | Use Case |
|---|---|---|---|
| `static` (default) | `output: 'static'` | No | Fully static site, pre-rendered at build |
| `server` | `output: 'server'` | Yes | Full SSR, every request hits the server |
| `hybrid` | `output: 'hybrid'` | Yes | Mostly static with some SSR routes |

**Convention:** Default to `static` unless you have a specific need for server-side rendering (auth, personalization, real-time data). Static sites are faster, cheaper, and more reliable.

### Cloudflare Pages Deployment

```bash
npx astro add cloudflare
```

```javascript
// astro.config.mjs
import cloudflare from '@astrojs/cloudflare';

export default defineConfig({
  output: 'server',  // or 'hybrid' for mostly static with some SSR
  adapter: cloudflare({
    platformProxy: {
      enabled: true,  // Local dev emulation of Cloudflare bindings
    },
  }),
});
```

```yaml
# .github/workflows/deploy-cloudflare.yml
name: Deploy to Cloudflare Pages
on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22 }
      - run: npm ci
      - run: npm run test
      - run: npx playwright install --with-deps
      - run: npm run test:e2e

  deploy:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22 }
      - run: npm ci
      - run: npm run build
      - uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          command: pages deploy dist --project-name=my-site
```

### Vercel Deployment

```bash
npx astro add vercel
```

```javascript
// astro.config.mjs
import vercel from '@astrojs/vercel';

export default defineConfig({
  output: 'server',
  adapter: vercel({
    webAnalytics: { enabled: true },
    imageService: true,  // Use Vercel's image optimization
    isr: {
      expiration: 60 * 60,  // 1 hour ISR for dynamic pages
    },
  }),
});
```

```json
// vercel.json
{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "X-Frame-Options", "value": "DENY" },
        { "key": "Referrer-Policy", "value": "strict-origin-when-cross-origin" }
      ]
    },
    {
      "source": "/assets/(.*)",
      "headers": [
        { "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }
      ]
    }
  ]
}
```

### Node.js Self-Hosted Deployment

```bash
npx astro add node
```

```javascript
// astro.config.mjs
import node from '@astrojs/node';

export default defineConfig({
  output: 'server',
  adapter: node({
    mode: 'standalone',  // Bundles everything into a single server
  }),
});
```

```dockerfile
# Dockerfile for self-hosted Node deployment
FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:22-alpine AS runtime
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

ENV HOST=0.0.0.0
ENV PORT=4321
EXPOSE 4321

CMD ["node", "./dist/server/entry.mjs"]
```

### Static Site Deployment (No Adapter)

For fully static sites, no adapter is needed. Build and deploy the `dist/` directory to any static host:

```bash
# Build
npm run build

# The dist/ directory is a complete static site
# Deploy to: Cloudflare Pages, Vercel, Netlify, GitHub Pages, S3+CloudFront, etc.
```

For GitHub Pages:

```yaml
# .github/workflows/deploy-ghpages.yml
name: Deploy to GitHub Pages
on:
  push:
    branches: [main]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22 }
      - run: npm ci
      - run: npm run build
      - uses: actions/upload-pages-artifact@v3
        with: { path: dist }

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - id: deployment
        uses: actions/deploy-pages@v4
```

### CI/CD Pipeline (Generic)

Every push runs a three-stage pipeline:

```yaml
jobs:
  lint:
    steps:
      - npm ci
      - npx astro check         # TypeScript + Astro template checking
      - npx tsc --noEmit         # Pure TypeScript checking

  test:
    steps:
      - npm ci
      - npm run test             # Vitest unit + component tests
      - npm run build            # Must build before E2E (Playwright needs the built site)
      - npx playwright install --with-deps
      - npm run test:e2e         # Playwright E2E tests

  deploy:
    needs: [lint, test]
    if: github.ref == 'refs/heads/main'
    steps:
      - npm ci
      - npm run build
      - # Deploy to target platform
```

**Convention:** The build step runs before E2E tests because Playwright tests against the built site (`npm run preview` serves the `dist/` directory). Never skip `astro check` — it catches template errors that TypeScript alone misses.

---

## 12. Performance Patterns

### Core Web Vitals Targets

| Metric | Target | How Astro Helps |
|---|---|---|
| LCP (Largest Contentful Paint) | < 2.5s | Zero JS by default, static HTML served from edge |
| FID (First Input Delay) | < 100ms | Islands architecture: only interactive elements load JS |
| CLS (Cumulative Layout Shift) | < 0.1 | `<Image>` component sets width/height, fonts use `font-display: swap` |
| INP (Interaction to Next Paint) | < 200ms | Minimal JS, framework hydration only where needed |
| TTFB (Time to First Byte) | < 800ms | Static files served from CDN edge, no server rendering for static pages |

### Zero-JS-by-Default Discipline

The #1 performance advantage of Astro is zero JavaScript shipped by default. Protect this aggressively:

```
Page loads with 0 KB JS → Astro default
Add one React island → ~45-80 KB JS (React runtime)
Add Tailwind → 0 KB JS (CSS only)
Add Pagefind → ~10 KB JS (lazy-loaded)
```

**Rules:**
- Track JS bundle size in CI. Alert if any page exceeds 100 KB transferred JS.
- Audit every `client:*` directive. Each one adds framework runtime to that page.
- Prefer `client:visible` over `client:load` for below-the-fold components.
- Prefer `client:idle` over `client:load` for non-critical interactive elements.
- Never use `client:load` for components that are not visible on initial page load.

### Font Loading

```css
/* Use system font stack for maximum performance */
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
    'Helvetica Neue', Arial, sans-serif;
}

/* If custom fonts are required, use font-display: swap */
@font-face {
  font-family: 'CustomFont';
  src: url('/fonts/custom.woff2') format('woff2');
  font-display: swap;
  font-weight: 400;
}
```

**Convention:** Prefer the system font stack. Custom fonts add 20-100 KB and cause FOUT/FOIT. If a custom font is required, self-host it (never load from Google Fonts CDN — it adds an extra DNS lookup and connection).

### Prefetching

Astro 5+ includes built-in prefetching with the `<ClientRouter />` component or the `prefetch` configuration:

```javascript
// astro.config.mjs
export default defineConfig({
  prefetch: {
    prefetchAll: false,           // Don't prefetch every link
    defaultStrategy: 'viewport',  // Prefetch links as they enter viewport
  },
});
```

For individual links:

```astro
<!-- Prefetch on hover (default) -->
<a href="/docs">Documentation</a>

<!-- Prefetch as soon as visible -->
<a href="/docs" data-astro-prefetch="viewport">Documentation</a>

<!-- Never prefetch (external links, heavy pages) -->
<a href="/large-report" data-astro-prefetch="false">Heavy Report</a>
```

### Asset Caching

```
# Cache immutable assets aggressively
/_astro/*   → Cache-Control: public, max-age=31536000, immutable
/pagefind/* → Cache-Control: public, max-age=86400
/*.html     → Cache-Control: public, max-age=0, must-revalidate
/rss.xml    → Cache-Control: public, max-age=3600
```

Astro's build output includes content-hashed filenames in `/_astro/` — these are safe to cache forever.

---

## 13. Security Patterns

### Security Headers

Every deployed Astro site must include security headers. Configuration depends on the deployment target.

**Cloudflare (`_headers` file in `public/`):**

```
/*
  X-Content-Type-Options: nosniff
  X-Frame-Options: DENY
  Referrer-Policy: strict-origin-when-cross-origin
  Permissions-Policy: camera=(), microphone=(), geolocation=()
  Strict-Transport-Security: max-age=31536000; includeSubDomains; preload

/_astro/*
  Cache-Control: public, max-age=31536000, immutable
```

**Vercel (`vercel.json`):** See deployment section above.

**Node.js (middleware):**

```typescript
// src/middleware.ts (Astro middleware for SSR mode)
import { defineMiddleware } from 'astro:middleware';

export const onRequest = defineMiddleware(async ({ request }, next) => {
  const response = await next();

  response.headers.set('X-Content-Type-Options', 'nosniff');
  response.headers.set('X-Frame-Options', 'DENY');
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
  response.headers.set('Permissions-Policy', 'camera=(), microphone=(), geolocation=()');
  response.headers.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');

  return response;
});
```

### Content Security Policy

For static Astro sites, a strict CSP is achievable:

```
Content-Security-Policy:
  default-src 'self';
  script-src 'self' 'unsafe-inline';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data:;
  font-src 'self';
  connect-src 'self';
  frame-ancestors 'none';
  base-uri 'self';
  form-action 'self';
```

**Notes:**
- `unsafe-inline` for scripts is needed if using `<script is:inline>` (e.g., dark mode toggle). If all scripts are bundled modules, you can drop this.
- `unsafe-inline` for styles is needed for Astro's scoped styles and Tailwind's runtime injection. Cannot be avoided with current Tailwind 4.x.
- `frame-ancestors 'none'` prevents clickjacking. Relax to `'self'` if embedding is needed.

### Environment Variables

```bash
# .env (local development only — never committed)
PUBLIC_SITE_URL=https://example.com
API_KEY=secret_key_here

# .env.example (committed — documents required variables)
PUBLIC_SITE_URL=
API_KEY=
```

**Rules:**
- Only variables prefixed with `PUBLIC_` are exposed to client-side code.
- Server-side secrets (API keys, database URLs) must never use the `PUBLIC_` prefix.
- Always add `.env` to `.gitignore`.
- Document required environment variables in `.env.example`.

```astro
---
// Server-side: access any variable
const apiKey = import.meta.env.API_KEY;  // Only available in frontmatter/server code

// Client-side: only PUBLIC_ variables
const siteUrl = import.meta.env.PUBLIC_SITE_URL;  // Available everywhere
---
```

### Dependency Auditing

```json
{
  "scripts": {
    "audit": "npm audit --production",
    "audit:fix": "npm audit fix"
  }
}
```

Run `npm audit` in CI. Block deployment on critical or high vulnerabilities.

---

## 14. Accessibility Patterns

### Minimum Requirements

Every Astro page must meet WCAG 2.1 AA:

| Requirement | Implementation |
|---|---|
| Language declaration | `<html lang="en">` in Base layout |
| Document title | Unique `<title>` per page |
| Heading hierarchy | Single `<h1>` per page, no skipped levels |
| Alt text on images | `alt` attribute on every `<img>` and `<Image>` |
| Keyboard navigation | All interactive elements reachable via Tab |
| Focus indicators | Visible focus ring on all interactive elements |
| Color contrast | 4.5:1 for text, 3:1 for large text (per COLOR_CONTRAST_PATTERNS.md) |
| ARIA landmarks | `<nav>`, `<main>`, `<footer>` semantic elements |
| Skip to content | Skip link as first focusable element |
| Form labels | Every input has a visible `<label>` or `aria-label` |

### Skip Navigation Link

```astro
<!-- First element in Base.astro <body> -->
<a href="#main-content" class="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-[100] focus:bg-accent focus:text-white focus:px-4 focus:py-2 focus:rounded-lg">
  Skip to main content
</a>

<!-- Main content target -->
<main id="main-content" class="flex-1">
  <slot />
</main>
```

### Touch Target Sizing

All interactive elements must meet the 44x44px minimum touch target:

```css
/* Minimum touch targets */
a, button, [role="button"], input, select, textarea {
  min-height: 44px;
  min-width: 44px;
}

/* Navigation links need padding to meet touch target */
nav a {
  min-height: 44px;
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 0.75rem;
}
```

### Focus Management

```css
/* Visible focus ring on all interactive elements */
:focus-visible {
  outline: 2px solid var(--color-accent);
  outline-offset: 2px;
}

/* Remove default outline (replaced by :focus-visible) */
:focus:not(:focus-visible) {
  outline: none;
}
```

### Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

---

## 15. Development Workflow

### Local Development

```bash
# Start dev server with hot reload
npm run dev

# Type-check Astro templates + TypeScript
npx astro check

# Build for production
npm run build

# Preview production build locally
npm run preview

# Full quality check
npm run test && npx astro check && npx tsc --noEmit
```

### Recommended Scripts

```json
{
  "scripts": {
    "dev": "astro dev",
    "build": "astro build && npx pagefind --site dist",
    "preview": "astro preview",
    "check": "astro check && tsc --noEmit",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage",
    "test:e2e": "playwright test",
    "test:e2e:headed": "playwright test --headed",
    "lint": "astro check && tsc --noEmit",
    "quality": "npm run lint && npm run test && npm run test:e2e",
    "ci": "npm run lint && npm run test && npm run build && npm run test:e2e"
  }
}
```

### Feature Development Cycle (Astro-specific)

```
1. Plan the page/feature (which routes, which collections, which islands)
2. Write failing E2E test (Playwright: "this page should exist and contain X")
3. Write failing unit tests for utility functions (Vitest)
4. Create the content collection schema (if new content type)
5. Create the Astro page/component (static first)
6. Add islands only if interactivity is required (document why)
7. Run: npx astro check
8. Run: npm run test
9. Run: npm run build
10. Run: npm run test:e2e
11. Verify Lighthouse scores (Performance 95+, Accessibility 100, SEO 100)
12. Deploy
```

### Astro CLI Commands

```bash
# Add an integration (interactive, updates config automatically)
npx astro add react
npx astro add sitemap
npx astro add cloudflare

# Check for Astro updates
npx astro@latest

# Debug: check installed integrations and config
npx astro info

# Type-check templates
npx astro check
```

---

## 16. Anti-Patterns (Astro-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using `client:load` on every component | Default to zero JS. Use `client:visible` or `client:idle` for below-the-fold interactivity. Most components need no hydration directive. |
| 2 | Building the entire site as React/Vue/Svelte components | Use Astro components (`.astro`) for all static content. Only use framework components for genuinely interactive islands. |
| 3 | Using `Astro.glob()` in Astro 5+ | Use `import.meta.glob()` (Vite-native) or content collections (preferred for structured content). `Astro.glob()` was removed in Astro 5. |
| 4 | Putting content in `src/pages/` that should be a collection | Blog posts, docs, changelog entries belong in `src/content/` with Zod schemas. Pages are for routing; collections are for content. |
| 5 | Loading fonts from Google Fonts CDN | Self-host fonts in `public/fonts/`. External font loading adds DNS lookup, connection time, and a render-blocking request. |
| 6 | Missing `site` in `astro.config.mjs` | Always set `site: 'https://yourdomain.com'`. Required for sitemap generation, canonical URLs, and RSS feeds. Without it, these features silently fail. |
| 7 | Using `<img>` tags for local images | Use Astro's `<Image>` or `<Picture>` components. They generate optimized WebP/AVIF, set correct dimensions (preventing CLS), and lazy-load by default. |
| 8 | Bundled script for dark mode toggle | Use `<script is:inline>` in `<head>` for the initial theme check. Bundled scripts load asynchronously, causing flash-of-wrong-theme (FOWT). |
| 9 | Global `<style>` for component-specific styles | Use scoped `<style>` in `.astro` files. Only use `<style is:global>` for Markdown/MDX content rendered by layouts. |
| 10 | Missing `alt` attribute on images | Every `<img>` and `<Image>` must have `alt`. Use `alt=""` with `aria-hidden="true"` for decorative images. Never omit the attribute. |
| 11 | Skipping `astro check` in CI | Always run `astro check` in CI. It catches template errors, type mismatches in props, and collection schema violations that `tsc` alone misses. |
| 12 | Using CommonJS (`require()`) in config files | All Astro projects must use ESM (`import`/`export`). Set `"type": "module"` in `package.json`. CommonJS causes cryptic build failures. |
| 13 | Deploying without security headers | Every deployment target needs security headers (HSTS, CSP, X-Content-Type-Options, X-Frame-Options). See Security Patterns section. |
| 14 | Using React Context/Vue Provide across islands | Islands are isolated runtime contexts. Use nanostores or custom DOM events for cross-island state. Framework-specific state management does not work across island boundaries. |
| 15 | Missing `width`/`height` on remote images | Remote images have no intrinsic dimensions. Always specify `width` and `height` to prevent CLS. Use `inferSize` only when you cannot know dimensions ahead of time. |
| 16 | Committing `.env` files | Add `.env` to `.gitignore`. Only `PUBLIC_`-prefixed variables are safe for client-side code. Document required variables in `.env.example`. |
| 17 | Using `tailwind.config.js` with Tailwind 4.x | Tailwind 4.x uses CSS-based configuration (`@theme`, `@plugin`). The JavaScript config file is a v3 pattern. Use `@tailwindcss/vite` and configure in `global.css`. |
| 18 | Skipping the `engines` field in `package.json` | Always specify `"engines": { "node": ">=22.12.0" }`. Prevents accidental deployment on unsupported Node versions that cause cryptic build failures. |
| 19 | Using `set:html` with user-provided content | `set:html` bypasses Astro's HTML escaping. Only use it for trusted content (JSON-LD, pre-sanitized HTML). Never use with user input — it creates XSS vulnerabilities. |
| 20 | Testing Astro components with just `tsc` | `tsc` checks TypeScript but not Astro template syntax. Use `astro check` for full template validation including prop types, slot usage, and collection schemas. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report an Astro patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:astro&title=[Astro]%20)**

Use the `patterns:astro` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
