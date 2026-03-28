# Blog Design Implementation — Astro + Tailwind

**Purpose:** Exact implementation specifications for premium editorial blog styling in our Astro + Tailwind 4.x stack.
**Companion:** BLOG_DESIGN_PATTERNS.md (research and rationale), COLOR_CONTRAST_PATTERNS.md (accessibility tokens)
**Last updated:** 2026-03-28

---

## 1. Font Loading

### 1.1 Font Selection

| Role | Font | Weight(s) | Format |
|------|------|-----------|--------|
| Headings + Body | Inter Variable | 400, 500, 600, 700, 800 | woff2 variable |
| Code | JetBrains Mono | 400, 500 | woff2 variable |

### 1.2 Self-Host Setup

Download from Google Fonts or fontsource. Self-hosting avoids third-party requests and GDPR issues.

```bash
# Install via fontsource (recommended for Astro)
npm install @fontsource-variable/inter @fontsource/jetbrains-mono
```

### 1.3 Import in Base Layout

In the base Astro layout (e.g., `src/layouts/BaseLayout.astro` or `src/layouts/Layout.astro`):

```astro
---
// In frontmatter
import '@fontsource-variable/inter';
import '@fontsource/jetbrains-mono/400.css';
import '@fontsource/jetbrains-mono/500.css';
---
```

### 1.4 Font-Face Fallback (Prevent Layout Shift)

Add `font-display: swap` and size-adjusted system fallback:

```css
/* In global.css or base layer */
@layer base {
  :root {
    --font-sans: 'Inter Variable', 'Inter', ui-sans-serif, system-ui,
      -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    --font-mono: 'JetBrains Mono', ui-monospace, SFMono-Regular,
      'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
  }
}
```

In Tailwind 4.x (CSS-based config), set the theme font:

```css
/* In your main CSS file that imports Tailwind */
@import "tailwindcss";

@theme {
  --font-sans: 'Inter Variable', 'Inter', ui-sans-serif, system-ui,
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', ui-monospace, SFMono-Regular,
    'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
}
```

---

## 2. Color Variables

### 2.1 CSS Custom Properties (Light + Dark)

These values come from COLOR_CONTRAST_PATTERNS.md. Define once in global CSS:

```css
@layer base {
  :root {
    /* Backgrounds */
    --color-bg-page: #FAFAFA;
    --color-bg-surface: #FFFFFF;
    --color-bg-elevated: #F3F4F6;
    --color-bg-code: #0D1117;
    --color-bg-code-inline: #F1F5F9;

    /* Text */
    --color-text-primary: #111827;
    --color-text-secondary: #4B5563;
    --color-text-tertiary: #6B7280;
    --color-text-on-code: #E6EDF3;

    /* Borders */
    --color-border: #E5E7EB;
    --color-border-strong: #D1D5DB;

    /* Accent */
    --color-accent: #2563EB;
    --color-accent-hover: #1D4ED8;

    /* Semantic */
    --color-info-bg: #EFF6FF;
    --color-info-border: #3B82F6;
    --color-info-text: #1E40AF;
    --color-tip-bg: #F0FDF4;
    --color-tip-border: #22C55E;
    --color-tip-text: #166534;
    --color-warning-bg: #FFFBEB;
    --color-warning-border: #F59E0B;
    --color-warning-text: #92400E;
    --color-danger-bg: #FEF2F2;
    --color-danger-border: #EF4444;
    --color-danger-text: #991B1B;

    /* Callout shared */
    --callout-border-width: 4px;
    --callout-radius: 0.5rem;
    --callout-padding: 1rem 1rem 1rem 1.25rem;
  }

  .dark {
    /* Backgrounds */
    --color-bg-page: #0F172A;
    --color-bg-surface: #1E293B;
    --color-bg-elevated: #334155;
    --color-bg-code: #0D1117;
    --color-bg-code-inline: #1E293B;

    /* Text */
    --color-text-primary: #F1F5F9;
    --color-text-secondary: #CBD5E1;
    --color-text-tertiary: #94A3B8;
    --color-text-on-code: #E6EDF3;

    /* Borders */
    --color-border: #334155;
    --color-border-strong: #475569;

    /* Accent */
    --color-accent: #60A5FA;
    --color-accent-hover: #93C5FD;

    /* Semantic */
    --color-info-bg: #1E3A5F;
    --color-info-border: #60A5FA;
    --color-info-text: #BFDBFE;
    --color-tip-bg: #14532D;
    --color-tip-border: #4ADE80;
    --color-tip-text: #BBF7D0;
    --color-warning-bg: #451A03;
    --color-warning-border: #FBBF24;
    --color-warning-text: #FEF3C7;
    --color-danger-bg: #450A0A;
    --color-danger-border: #F87171;
    --color-danger-text: #FECACA;
  }
}
```

---

## 3. BlogPost.astro Layout — Full Redesign

### 3.1 Complete Layout Template

```astro
---
import type { CollectionEntry } from 'astro:content';
import BaseLayout from './BaseLayout.astro';
import AuthorCard from '../components/blog/AuthorCard.astro';
import TableOfContents from '../components/blog/TableOfContents.astro';
import ReadingProgress from '../components/blog/ReadingProgress.astro';
import ShareButtons from '../components/blog/ShareButtons.astro';

interface Props {
  post: CollectionEntry<'blog'>;
  headings: { depth: number; slug: string; text: string }[];
}

const { post, headings } = Astro.props;
const { title, description, pubDate, updatedDate, author, tags, category, heroImage, readTime } = post.data;
const showToc = headings.length > 4;
---

<BaseLayout title={title} description={description}>
  <ReadingProgress />

  <article
    class="blog-post"
    itemscope
    itemtype="https://schema.org/BlogPosting"
  >
    {/* --- HEADER --- */}
    <header class="post-header">
      <div class="post-header-inner">
        {category && (
          <span class="post-category">{category}</span>
        )}

        <h1 class="post-title" itemprop="headline">{title}</h1>

        {description && (
          <p class="post-subtitle">{description}</p>
        )}

        <div class="post-meta">
          <time datetime={pubDate.toISOString()} itemprop="datePublished">
            {pubDate.toLocaleDateString('en-US', {
              year: 'numeric',
              month: 'long',
              day: 'numeric',
            })}
          </time>
          {readTime && (
            <>
              <span class="meta-separator" aria-hidden="true">&middot;</span>
              <span>{readTime} min read</span>
            </>
          )}
        </div>
      </div>
    </header>

    {/* --- HERO IMAGE --- */}
    {heroImage && (
      <figure class="post-hero">
        <img
          src={heroImage}
          alt=""
          width="1200"
          height="675"
          loading="eager"
          fetchpriority="high"
          decoding="async"
        />
      </figure>
    )}

    {/* --- CONTENT GRID --- */}
    <div class:list={['post-body', { 'has-toc': showToc }]}>
      {/* TOC sidebar (desktop) */}
      {showToc && (
        <aside class="post-toc-sidebar" aria-label="Table of contents">
          <TableOfContents headings={headings} />
        </aside>
      )}

      {/* Inline TOC (mobile) */}
      {showToc && (
        <details class="post-toc-inline">
          <summary>Table of contents</summary>
          <TableOfContents headings={headings} />
        </details>
      )}

      {/* Article prose */}
      <div class="post-content prose" itemprop="articleBody">
        <slot />
      </div>
    </div>

    {/* --- FOOTER --- */}
    <footer class="post-footer">
      {tags && tags.length > 0 && (
        <div class="post-tags">
          {tags.map((tag) => (
            <a href={`/blog/tag/${tag}`} class="tag-link">
              {tag}
            </a>
          ))}
        </div>
      )}

      <hr class="post-divider" />

      <AuthorCard author={author} />

      <ShareButtons title={title} url={Astro.url.href} />
    </footer>
  </article>
</BaseLayout>

<script>
  // Copy button on code blocks
  document.querySelectorAll('pre').forEach((pre) => {
    const wrapper = document.createElement('div');
    wrapper.className = 'code-block-wrapper';
    pre.parentNode?.insertBefore(wrapper, pre);
    wrapper.appendChild(pre);

    const button = document.createElement('button');
    button.className = 'copy-button';
    button.textContent = 'Copy';
    button.setAttribute('aria-label', 'Copy code to clipboard');
    wrapper.appendChild(button);

    button.addEventListener('click', async () => {
      const code = pre.querySelector('code')?.textContent ?? '';
      await navigator.clipboard.writeText(code);
      button.textContent = 'Copied!';
      button.classList.add('copied');
      setTimeout(() => {
        button.textContent = 'Copy';
        button.classList.remove('copied');
      }, 2000);
    });
  });

  // Reading progress
  const article = document.querySelector('.post-content');
  const bar = document.querySelector('.reading-progress-bar') as HTMLElement;
  if (article && bar) {
    function updateProgress() {
      const rect = article!.getBoundingClientRect();
      const viewH = window.innerHeight;
      const start = 0;
      const end = article!.scrollHeight - viewH;
      const scrolled = -rect.top;
      const pct = Math.min(Math.max(scrolled / end, 0), 1) * 100;
      bar!.style.width = `${pct}%`;
    }
    window.addEventListener('scroll', updateProgress, { passive: true });
    updateProgress();
  }

  // TOC active section tracking
  const tocLinks = document.querySelectorAll('.toc-link');
  if (tocLinks.length > 0) {
    const headings = document.querySelectorAll('.post-content h2, .post-content h3');
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            tocLinks.forEach((link) => link.classList.remove('active'));
            const active = document.querySelector(
              `.toc-link[href="#${entry.target.id}"]`
            );
            active?.classList.add('active');
          }
        });
      },
      { rootMargin: '-80px 0px -80% 0px' }
    );
    headings.forEach((h) => observer.observe(h));
  }
</script>
```

### 3.2 Styles

Put these in `src/styles/blog-post.css` (or a `<style is:global>` block in the layout). All values are exact — no guesswork needed.

```css
/* ==========================================================================
   Blog Post — Premium Editorial Styling
   ========================================================================== */

/* --- Reading Progress Bar --- */
.reading-progress {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 3px;
  z-index: 50;
  background: transparent;
}

.reading-progress-bar {
  height: 100%;
  width: 0%;
  background: var(--color-accent);
  transition: width 0.1s linear;
}

/* --- Article Container --- */
.blog-post {
  background: var(--color-bg-surface);
  max-width: 100%;
}

/* --- Header --- */
.post-header {
  padding: 4rem 1rem 2rem;
  text-align: center;
}

.post-header-inner {
  max-width: 42rem;
  margin: 0 auto;
}

.post-category {
  display: inline-block;
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--color-accent);
  margin-bottom: 1rem;
}

.post-title {
  font-size: clamp(2rem, 5vw, 3rem);
  font-weight: 800;
  line-height: 1.1;
  letter-spacing: -0.03em;
  color: var(--color-text-primary);
  margin: 0 0 1rem;
  text-wrap: balance;
}

.post-subtitle {
  font-size: clamp(1.125rem, 2vw, 1.25rem);
  line-height: 1.5;
  color: var(--color-text-secondary);
  margin: 0 0 1.5rem;
  text-wrap: balance;
}

.post-meta {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--color-text-tertiary);
}

.meta-separator {
  user-select: none;
}

/* --- Hero Image --- */
.post-hero {
  max-width: 56rem;
  margin: 0 auto 3rem;
  padding: 0 1rem;
}

.post-hero img {
  width: 100%;
  height: auto;
  border-radius: 0.75rem;
  aspect-ratio: 16 / 9;
  object-fit: cover;
}

@media (max-width: 639px) {
  .post-hero {
    padding: 0;
  }
  .post-hero img {
    border-radius: 0;
  }
}

/* --- Content Body Grid --- */
.post-body {
  max-width: 42rem;
  margin: 0 auto;
  padding: 0 1rem 4rem;
}

.post-body.has-toc {
  max-width: 72rem;
  display: grid;
  grid-template-columns: 1fr;
  gap: 2rem;
}

@media (min-width: 1280px) {
  .post-body.has-toc {
    grid-template-columns: 1fr 14rem;
  }
}

/* --- TOC Sidebar (Desktop) --- */
.post-toc-sidebar {
  display: none;
}

@media (min-width: 1280px) {
  .post-toc-sidebar {
    display: block;
    order: 2;
  }

  .post-toc-sidebar nav {
    position: sticky;
    top: 5rem;
  }
}

.post-toc-sidebar .toc-title {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--color-text-tertiary);
  margin-bottom: 0.75rem;
}

.post-toc-sidebar .toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.post-toc-sidebar .toc-list li {
  margin: 0;
}

.toc-link {
  display: block;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
  line-height: 1.4;
  color: var(--color-text-tertiary);
  text-decoration: none;
  border-left: 2px solid transparent;
  padding-left: 0.75rem;
  transition: color 0.15s, border-color 0.15s;
}

.toc-link:hover {
  color: var(--color-text-primary);
}

.toc-link.active {
  color: var(--color-accent);
  border-left-color: var(--color-accent);
  font-weight: 500;
}

.toc-link[data-depth="3"] {
  padding-left: 1.5rem;
}

/* --- TOC Inline (Mobile) --- */
.post-toc-inline {
  display: block;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 0.75rem 1rem;
  margin-bottom: 2rem;
}

.post-toc-inline summary {
  font-weight: 600;
  font-size: 0.875rem;
  cursor: pointer;
  user-select: none;
  color: var(--color-text-secondary);
}

.post-toc-inline[open] summary {
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border);
}

@media (min-width: 1280px) {
  .post-toc-inline {
    display: none;
  }
}

/* ==========================================================================
   Prose — The Core Typography System
   ========================================================================== */

.prose {
  font-family: var(--font-sans);
  font-size: clamp(1.0625rem, 1.5vw, 1.125rem); /* 17-18px */
  line-height: 1.75;
  color: var(--color-text-primary);
  max-width: 42rem;
}

/* Vertical rhythm: lobotomized owl */
.prose > * + * {
  margin-top: 1.5em;
}

/* --- Headings --- */
.prose h2 {
  font-size: clamp(1.5rem, 3vw, 1.875rem);
  font-weight: 700;
  line-height: 1.25;
  letter-spacing: -0.02em;
  color: var(--color-text-primary);
  margin-top: 2.5em;
  margin-bottom: 0.75em;
  scroll-margin-top: 5rem;
}

.prose h3 {
  font-size: clamp(1.25rem, 2.5vw, 1.5rem);
  font-weight: 600;
  line-height: 1.3;
  letter-spacing: -0.015em;
  color: var(--color-text-primary);
  margin-top: 2em;
  margin-bottom: 0.5em;
  scroll-margin-top: 5rem;
}

.prose h4 {
  font-size: 1.125rem;
  font-weight: 600;
  line-height: 1.4;
  letter-spacing: -0.01em;
  color: var(--color-text-primary);
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  scroll-margin-top: 5rem;
}

/* Heading anchor links */
.prose :is(h2, h3, h4) .heading-anchor {
  opacity: 0;
  margin-left: 0.375em;
  color: var(--color-text-tertiary);
  text-decoration: none;
  transition: opacity 0.15s;
}

.prose :is(h2, h3, h4):hover .heading-anchor {
  opacity: 1;
}

/* Remove top margin if heading is first child */
.prose > :first-child {
  margin-top: 0;
}

/* --- Paragraphs --- */
.prose p {
  margin-top: 0;
  margin-bottom: 0;
}

/* --- Links --- */
.prose a {
  color: var(--color-accent);
  text-decoration: underline;
  text-decoration-color: color-mix(in srgb, var(--color-accent) 40%, transparent);
  text-underline-offset: 3px;
  text-decoration-thickness: 1px;
  transition: text-decoration-color 0.15s;
}

.prose a:hover {
  text-decoration-color: var(--color-accent);
}

/* --- Lists --- */
.prose ul,
.prose ol {
  padding-left: 1.5em;
  margin-top: 1.5em;
  margin-bottom: 0;
}

.prose li {
  margin-top: 0.5em;
}

.prose li::marker {
  color: var(--color-text-tertiary);
}

.prose li > p {
  margin-top: 0.5em;
}

.prose li > ul,
.prose li > ol {
  margin-top: 0.5em;
}

/* --- Blockquotes --- */
.prose blockquote {
  border-left: 3px solid var(--color-accent);
  padding-left: 1.25em;
  margin-left: 0;
  margin-right: 0;
  color: var(--color-text-secondary);
  font-style: italic;
}

.prose blockquote p:first-child {
  margin-top: 0;
}

/* --- Pull Quotes --- */
.prose .pull-quote {
  font-size: 1.375em;
  font-weight: 500;
  line-height: 1.4;
  color: var(--color-text-secondary);
  border-left: 4px solid var(--color-accent);
  padding: 0.25em 0 0.25em 1.25em;
  margin: 2em 0;
  font-style: normal;
}

/* --- Horizontal Rules --- */
.prose hr {
  border: none;
  height: 1px;
  background: var(--color-border);
  margin: 3em 0;
}

/* --- Tables --- */
.prose .table-wrapper {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
  margin: 1.5em 0;
}

.prose table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875em;
  line-height: 1.5;
}

.prose thead th {
  text-align: left;
  font-weight: 600;
  padding: 0.75em 1em;
  border-bottom: 2px solid var(--color-border-strong);
  color: var(--color-text-primary);
}

.prose tbody td {
  padding: 0.75em 1em;
  border-bottom: 1px solid var(--color-border);
}

.prose tbody tr:last-child td {
  border-bottom: none;
}

/* --- Inline Code --- */
.prose code:not(pre code) {
  background: var(--color-bg-code-inline);
  color: var(--color-text-primary);
  padding: 0.125em 0.375em;
  border-radius: 0.25rem;
  font-size: 0.875em;
  font-family: var(--font-mono);
  word-break: break-word;
}

/* --- Code Blocks --- */
.code-block-wrapper {
  position: relative;
  margin: 2em 0;
}

.prose pre {
  background: var(--color-bg-code);
  color: var(--color-text-on-code);
  border-radius: 0.75rem;
  padding: 1.25rem 1.5rem;
  overflow-x: auto;
  font-size: 0.875rem;
  line-height: 1.7;
  font-family: var(--font-mono);
  tab-size: 2;
  margin: 0; /* Wrapper handles margin */
}

/* Code block breakout — wider than prose on desktop */
@media (min-width: 768px) {
  .code-block-wrapper {
    margin-left: -2rem;
    margin-right: -2rem;
  }
}

/* Full bleed on mobile */
@media (max-width: 639px) {
  .code-block-wrapper {
    margin-left: -1rem;
    margin-right: -1rem;
  }

  .prose pre {
    border-radius: 0;
    padding: 1rem;
  }
}

/* Copy button */
.copy-button {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  padding: 0.25rem 0.625rem;
  font-size: 0.75rem;
  font-family: var(--font-sans);
  font-weight: 500;
  color: #94A3B8;
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.375rem;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s, background 0.15s, color 0.15s;
}

.code-block-wrapper:hover .copy-button {
  opacity: 1;
}

.copy-button:hover {
  background: rgba(255, 255, 255, 0.15);
  color: #E2E8F0;
}

.copy-button.copied {
  color: #4ADE80;
  border-color: rgba(74, 222, 128, 0.3);
}

/* Filename header for code blocks */
.code-filename {
  display: inline-block;
  background: var(--color-bg-code);
  color: #94A3B8;
  font-size: 0.75rem;
  font-family: var(--font-mono);
  padding: 0.375rem 1rem;
  border-radius: 0.75rem 0.75rem 0 0;
  margin-bottom: -0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

/* Line highlighting within code blocks */
.prose pre .highlight-line {
  background: rgba(96, 165, 250, 0.1);
  margin: 0 -1.5rem;
  padding: 0 1.5rem;
  display: block;
  border-left: 3px solid var(--color-accent);
}

/* --- Images --- */
.prose img {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  margin: 2em 0;
}

.prose figure {
  margin: 2em 0;
}

.prose figcaption {
  font-size: 0.875rem;
  color: var(--color-text-tertiary);
  text-align: center;
  margin-top: 0.75rem;
}

/* --- Strong / Emphasis --- */
.prose strong {
  font-weight: 600;
  color: var(--color-text-primary);
}

.prose em {
  font-style: italic;
}

/* --- Details/Summary (Expandable) --- */
.prose details {
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 0.75rem 1rem;
  margin: 1.5em 0;
}

.prose summary {
  font-weight: 600;
  cursor: pointer;
  user-select: none;
}

.prose details[open] summary {
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border);
}

/* ==========================================================================
   Callout / Admonition Boxes
   ========================================================================== */

/*
  Usage in Markdown (via remark plugin or custom component):

  :::note
  This is a note callout.
  :::

  Or as Astro component:
  <Callout type="note">This is a note.</Callout>
*/

.callout {
  border-left: var(--callout-border-width) solid;
  border-radius: var(--callout-radius);
  padding: var(--callout-padding);
  margin: 1.5em 0;
  font-size: 0.9375rem;
  line-height: 1.6;
}

.callout-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
  font-size: 0.8125rem;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  margin-bottom: 0.5rem;
}

.callout-title svg {
  width: 1rem;
  height: 1rem;
  flex-shrink: 0;
}

/* Variants */
.callout-note {
  background: var(--color-info-bg);
  border-left-color: var(--color-info-border);
}
.callout-note .callout-title {
  color: var(--color-info-text);
}

.callout-tip {
  background: var(--color-tip-bg);
  border-left-color: var(--color-tip-border);
}
.callout-tip .callout-title {
  color: var(--color-tip-text);
}

.callout-warning {
  background: var(--color-warning-bg);
  border-left-color: var(--color-warning-border);
}
.callout-warning .callout-title {
  color: var(--color-warning-text);
}

.callout-danger {
  background: var(--color-danger-bg);
  border-left-color: var(--color-danger-border);
}
.callout-danger .callout-title {
  color: var(--color-danger-text);
}

/* ==========================================================================
   Key Takeaway Box
   ========================================================================== */

.key-takeaway {
  border: 2px solid var(--color-accent);
  border-radius: 0.5rem;
  padding: 1.25rem 1.5rem;
  margin: 2em 0;
  background: color-mix(in srgb, var(--color-accent) 5%, var(--color-bg-surface));
}

.key-takeaway-label {
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--color-accent);
  margin-bottom: 0.5rem;
}

.key-takeaway p {
  margin: 0;
  font-size: 1rem;
  line-height: 1.6;
}

/* ==========================================================================
   Numbered Steps
   ========================================================================== */

.step-list {
  list-style: none;
  padding: 0;
  counter-reset: step-counter;
}

.step-item {
  counter-increment: step-counter;
  display: grid;
  grid-template-columns: 2.5rem 1fr;
  gap: 1rem;
  margin-top: 2rem;
  align-items: start;
}

.step-item::before {
  content: counter(step-counter);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  background: var(--color-accent);
  color: white;
  font-weight: 700;
  font-size: 0.875rem;
  flex-shrink: 0;
}

.step-title {
  font-weight: 600;
  font-size: 1.125rem;
  margin-bottom: 0.5rem;
}

/* ==========================================================================
   Post Footer
   ========================================================================== */

.post-footer {
  max-width: 42rem;
  margin: 0 auto;
  padding: 0 1rem 4rem;
}

.post-divider {
  border: none;
  height: 1px;
  background: var(--color-border);
  margin: 2rem 0;
}

/* Tags */
.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

.tag-link {
  display: inline-block;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  background: var(--color-bg-elevated);
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  text-decoration: none;
  transition: background 0.15s, color 0.15s;
}

.tag-link:hover {
  background: var(--color-border);
  color: var(--color-text-primary);
}

/* Author Card */
.author-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem 0;
}

.author-avatar {
  width: 3.5rem;
  height: 3.5rem;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
}

.author-name {
  font-weight: 600;
  font-size: 1rem;
  color: var(--color-text-primary);
}

.author-bio {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.4;
  margin-top: 0.125rem;
}

.author-social {
  font-size: 0.8125rem;
  color: var(--color-accent);
  text-decoration: none;
  margin-top: 0.25rem;
  display: inline-block;
}

.author-social:hover {
  text-decoration: underline;
}

/* Share Buttons */
.share-buttons {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 0;
}

.share-label {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--color-text-tertiary);
}

.share-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.25rem;
  height: 2.25rem;
  border-radius: 0.375rem;
  border: 1px solid var(--color-border);
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: border-color 0.15s, color 0.15s;
}

.share-btn:hover {
  border-color: var(--color-border-strong);
  color: var(--color-text-primary);
}

.share-btn svg {
  width: 1rem;
  height: 1rem;
}

/* ==========================================================================
   Scroll Behavior
   ========================================================================== */

html {
  scroll-behavior: smooth;
  scroll-padding-top: 5rem;
}
```

---

## 4. Component Patterns

### 4.1 Callout Component

```astro
---
// src/components/blog/Callout.astro
interface Props {
  type?: 'note' | 'tip' | 'warning' | 'danger';
  title?: string;
}

const { type = 'note', title } = Astro.props;

const defaults = {
  note: { label: 'Note', icon: 'info' },
  tip: { label: 'Tip', icon: 'lightbulb' },
  warning: { label: 'Warning', icon: 'alert-triangle' },
  danger: { label: 'Danger', icon: 'alert-circle' },
};

const config = defaults[type];
const label = title ?? config.label;
---

<aside class={`callout callout-${type}`} role="note">
  <div class="callout-title">
    <svg aria-hidden="true" viewBox="0 0 16 16" fill="currentColor">
      {type === 'note' && (
        <path d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm6.5-.25A.75.75 0 017.25 7h1a.75.75 0 01.75.75v2.75h.25a.75.75 0 010 1.5h-2a.75.75 0 010-1.5h.25v-2h-.25a.75.75 0 01-.75-.75zM8 6a1 1 0 100-2 1 1 0 000 2z"/>
      )}
      {type === 'tip' && (
        <path d="M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 01-1.484.211c-.04-.282-.163-.547-.37-.847a8.695 8.695 0 00-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.75.75 0 01-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848l.213-.253c.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75zM6 15.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75zM5.75 12a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5z"/>
      )}
      {type === 'warning' && (
        <path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575zM8 5a.75.75 0 00-.75.75v2.5a.75.75 0 001.5 0v-2.5A.75.75 0 008 5zm1 6a1 1 0 11-2 0 1 1 0 012 0z"/>
      )}
      {type === 'danger' && (
        <path d="M2.343 13.657A8 8 0 1113.657 2.343 8 8 0 012.343 13.657zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z"/>
      )}
    </svg>
    {label}
  </div>
  <div class="callout-body">
    <slot />
  </div>
</aside>
```

Usage in MDX:

```mdx
import Callout from '../../components/blog/Callout.astro';

<Callout type="tip">
  You can use `clamp()` for fluid typography without media queries.
</Callout>

<Callout type="warning" title="Breaking Change">
  The `v3` API removes the `legacy` parameter.
</Callout>
```

### 4.2 Key Takeaway Component

```astro
---
// src/components/blog/KeyTakeaway.astro
interface Props {
  label?: string;
}

const { label = 'Key Takeaway' } = Astro.props;
---

<div class="key-takeaway">
  <div class="key-takeaway-label">{label}</div>
  <slot />
</div>
```

### 4.3 Author Card Component

```astro
---
// src/components/blog/AuthorCard.astro
interface Props {
  author: string;
}

const { author } = Astro.props;

// Map author names to metadata. Extend as needed.
const authors: Record<string, { name: string; bio: string; avatar: string; social?: string; handle?: string }> = {
  bryan: {
    name: 'Bryan',
    bio: 'Founder, Trinsik Labs. Building autonomous dev tools.',
    avatar: '/authors/bryan.webp',
    social: 'https://x.com/trinsiklabs',
    handle: '@trinsiklabs',
  },
};

const data = authors[author.toLowerCase()] ?? {
  name: author,
  bio: '',
  avatar: '/authors/default.webp',
};
---

<div class="author-card">
  <img
    src={data.avatar}
    alt={`${data.name}'s avatar`}
    width="56"
    height="56"
    loading="lazy"
    class="author-avatar"
  />
  <div>
    <div class="author-name">{data.name}</div>
    {data.bio && <div class="author-bio">{data.bio}</div>}
    {data.social && data.handle && (
      <a href={data.social} class="author-social" rel="noopener noreferrer">
        {data.handle}
      </a>
    )}
  </div>
</div>
```

### 4.4 Table of Contents Component

```astro
---
// src/components/blog/TableOfContents.astro
interface Props {
  headings: { depth: number; slug: string; text: string }[];
}

const { headings } = Astro.props;

// Only show h2 and h3
const filtered = headings.filter((h) => h.depth === 2 || h.depth === 3);
---

<nav aria-label="Table of contents">
  <div class="toc-title">On this page</div>
  <ul class="toc-list">
    {filtered.map((heading) => (
      <li>
        <a
          href={`#${heading.slug}`}
          class="toc-link"
          data-depth={heading.depth}
        >
          {heading.text}
        </a>
      </li>
    ))}
  </ul>
</nav>
```

### 4.5 Reading Progress Bar Component

```astro
---
// src/components/blog/ReadingProgress.astro
---

<div
  class="reading-progress"
  role="progressbar"
  aria-label="Reading progress"
  aria-valuenow="0"
  aria-valuemin="0"
  aria-valuemax="100"
>
  <div class="reading-progress-bar"></div>
</div>
```

### 4.6 Share Buttons Component

```astro
---
// src/components/blog/ShareButtons.astro
interface Props {
  title: string;
  url: string;
}

const { title, url } = Astro.props;

const xUrl = `https://x.com/intent/tweet?text=${encodeURIComponent(title)}&url=${encodeURIComponent(url)}`;
const linkedInUrl = `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`;
---

<div class="share-buttons">
  <span class="share-label">Share</span>

  <button
    class="share-btn"
    aria-label="Copy link"
    data-copy-url={url}
  >
    <svg viewBox="0 0 16 16" fill="currentColor">
      <path d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-.025 9.45a.75.75 0 01-1.06-1.06l-1.25 1.25a2 2 0 01-2.83-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25z"/>
    </svg>
  </button>

  <a
    href={xUrl}
    class="share-btn"
    aria-label="Share on X"
    target="_blank"
    rel="noopener noreferrer"
  >
    <svg viewBox="0 0 16 16" fill="currentColor">
      <path d="M12.6.75h2.454l-5.36 6.142L16 15.25h-4.937l-3.867-5.07-4.425 5.07H.316l5.733-6.57L0 .75h5.063l3.495 4.633L12.601.75Zm-.86 13.028h1.36L4.323 2.145H2.865l8.875 11.633Z"/>
    </svg>
  </a>

  <a
    href={linkedInUrl}
    class="share-btn"
    aria-label="Share on LinkedIn"
    target="_blank"
    rel="noopener noreferrer"
  >
    <svg viewBox="0 0 16 16" fill="currentColor">
      <path d="M13.632 13.635h-2.37V9.922c0-.886-.018-2.025-1.234-2.025-1.235 0-1.424.964-1.424 1.96v3.778H6.234V6h2.274v1.042h.033c.317-.6 1.092-1.233 2.247-1.233 2.4 0 2.845 1.58 2.845 3.637v4.189zM3.558 4.955a1.376 1.376 0 110-2.752 1.376 1.376 0 010 2.752zM4.742 13.635H2.372V6h2.37v7.635zM14.816 0H1.182C.528 0 0 .516 0 1.153v13.694C0 15.484.528 16 1.182 16h13.634c.654 0 1.184-.516 1.184-1.153V1.153C16 .516 15.47 0 14.816 0z"/>
    </svg>
  </a>
</div>

<script>
  document.querySelectorAll('[data-copy-url]').forEach((btn) => {
    btn.addEventListener('click', async () => {
      const url = btn.getAttribute('data-copy-url') ?? window.location.href;
      await navigator.clipboard.writeText(url);
      const original = btn.innerHTML;
      btn.innerHTML = '<svg viewBox="0 0 16 16" fill="currentColor" style="color:#4ADE80"><path d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"/></svg>';
      setTimeout(() => { btn.innerHTML = original; }, 2000);
    });
  });
</script>
```

### 4.7 Numbered Steps Component

```astro
---
// src/components/blog/Steps.astro
---

<ol class="step-list">
  <slot />
</ol>
```

```astro
---
// src/components/blog/Step.astro
interface Props {
  title: string;
}

const { title } = Astro.props;
---

<li class="step-item">
  <div>
    <div class="step-title">{title}</div>
    <slot />
  </div>
</li>
```

Usage in MDX:

```mdx
import Steps from '../../components/blog/Steps.astro';
import Step from '../../components/blog/Step.astro';

<Steps>
  <Step title="Install the CLI">
    ```bash
    npm install -g cruxdev
    ```
  </Step>
  <Step title="Initialize your project">
    ```bash
    cruxdev init
    ```
  </Step>
  <Step title="Run convergence">
    ```bash
    cruxdev converge
    ```
  </Step>
</Steps>
```

---

## 5. Astro Configuration for Syntax Highlighting

Astro uses Shiki natively. Configure in `astro.config.mjs`:

```javascript
import { defineConfig } from 'astro/config';

export default defineConfig({
  markdown: {
    shikiConfig: {
      theme: 'github-dark',
      // Or for light/dark switching:
      // themes: { light: 'github-light', dark: 'github-dark' },
      wrap: false,
    },
  },
});
```

`github-dark` is the recommended theme — it matches our `#0D1117` code block background exactly (this is GitHub's own dark background color).

---

## 6. Tailwind 4.x Integration Notes

### 6.1 Avoid @tailwindcss/typography

The `@tailwindcss/typography` plugin's `prose` class conflicts with our custom prose system. We define our own `.prose` class with exact values. If the plugin is installed, either remove it or scope carefully:

```css
/* If you must keep @tailwindcss/typography, override everything */
.prose {
  --tw-prose-body: var(--color-text-primary);
  --tw-prose-headings: var(--color-text-primary);
  --tw-prose-links: var(--color-accent);
  --tw-prose-code: var(--color-text-primary);
  --tw-prose-pre-bg: var(--color-bg-code);
  /* ... etc */
}
```

**Recommendation:** Do not use `@tailwindcss/typography`. Our custom prose CSS is more precise and avoids the plugin's opinionated defaults that we would override 80% of anyway.

### 6.2 Using Tailwind Utility Classes Alongside Custom CSS

The blog post styles above are pure CSS by design — they form the article's typographic foundation. Use Tailwind utilities for layout (the grid, flexbox wrappers) and the custom CSS for prose styling. This separation keeps the prose predictable regardless of Tailwind version changes.

### 6.3 Dark Mode

Tailwind 4.x uses the `dark` variant which maps to `.dark` class or `@media (prefers-color-scheme: dark)`. We use the class strategy for user-toggleable dark mode:

```css
/* Already handled by our CSS custom properties */
/* .dark class on <html> flips all --color-* vars */
```

No additional Tailwind configuration needed — our semantic tokens handle everything.

---

## 7. Migration Checklist

For upgrading existing blog posts to this design system:

- [ ] Install Inter Variable and JetBrains Mono fonts
- [ ] Add CSS custom properties (section 2) to global stylesheet
- [ ] Replace existing BlogPost layout with new template (section 3)
- [ ] Add blog-post.css styles (section 3.2)
- [ ] Create component files: Callout, KeyTakeaway, AuthorCard, TableOfContents, ReadingProgress, ShareButtons, Steps/Step
- [ ] Configure Shiki theme to `github-dark` in astro.config.mjs
- [ ] Add author data (avatar images, metadata)
- [ ] Test in both light and dark modes
- [ ] Test on mobile (320px), tablet (768px), desktop (1280px+)
- [ ] Run Lighthouse accessibility audit — target 100
- [ ] Run axe-core browser extension — zero violations
- [ ] Verify code block copy button works
- [ ] Verify TOC highlights active section on scroll
- [ ] Verify reading progress bar tracks article, not full page

---

## 8. Quick Reference — All Exact Values

```
FONT SANS:        Inter Variable, system-ui fallback
FONT MONO:        JetBrains Mono, monospace fallback
BODY SIZE:        clamp(1.0625rem, 1.5vw, 1.125rem)  → 17-18px
BODY LINE-HEIGHT: 1.75
BODY COLOR:       #111827 (light) / #F1F5F9 (dark)
H1 SIZE:          clamp(2rem, 5vw, 3rem)  → 32-48px
H1 WEIGHT:        800
H1 LINE-HEIGHT:   1.1
H1 TRACKING:      -0.03em
H2 SIZE:          clamp(1.5rem, 3vw, 1.875rem)  → 24-30px
H2 WEIGHT:        700
H2 ABOVE:         2.5em
H2 BELOW:         0.75em
H3 SIZE:          clamp(1.25rem, 2.5vw, 1.5rem)  → 20-24px
H3 WEIGHT:        600
H3 ABOVE:         2em
H3 BELOW:         0.5em
PROSE MAX-WIDTH:  42rem (672px)
WIDE MAX-WIDTH:   56rem (896px)
PAGE MAX-WIDTH:   72rem (1152px)
CODE BG:          #0D1117 (both modes)
CODE FONT-SIZE:   0.875rem (14px)
CODE LINE-HEIGHT: 1.7
LINK COLOR:       #2563EB (light) / #60A5FA (dark)
PAGE BG:          #FAFAFA (light) / #0F172A (dark)
SURFACE BG:       #FFFFFF (light) / #1E293B (dark)
BORDER:           #E5E7EB (light) / #334155 (dark)
SECTION GAP:      3em (hr margin)
PARAGRAPH GAP:    1.5em (owl selector)
BLOCK GAP:        2em (code, images, callouts)
HEADING RATIO:    above:below = ~3:1
TRANSITION:       0.15s for hover states
SCROLL OFFSET:    5rem (fixed nav height)
PROGRESS BAR:     3px height, accent color
```
