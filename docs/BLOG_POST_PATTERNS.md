# Blog Post Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Stripe, Vercel, Cloudflare engineering blogs, Smashing Magazine, web.dev, MDN, CSS-Tricks, Dev.to, Medium, Hashnode, Ghost, SEO research, developer content analysis
**Last updated:** 2026-03-27

---

## 1. Structure (In Order)

1. **Hook / Problem Statement** (1-2 paragraphs) — Start with the reader's pain. NOT "We're excited to announce."
2. **The Insight** — What changed, why it matters.
3. **How It Works** — Technical explanation with code, architecture, benchmarks.
4. **Show, Don't Tell** — Before/after, demo, real use case. Ideally runnable.
5. **Results / Data** — Numbers. "40% faster" beats "significantly faster."
6. **Getting Started / CTA** — Install command, link to docs.
7. **What's Next** (optional) — Roadmap tease.

## 2. Length

| Type | Words | When |
|------|-------|------|
| Announcement | 800-1,200 | Launch, release |
| Tutorial | 2,000-3,000 | Onboarding |
| Deep dive / "How we built X" | 1,500-2,500 | After shipping something hard |
| Comparison | 1,500-2,000 | Competing |
| Benchmark / Data | 1,000-1,500 | Impressive numbers |

## 3. SEO

- **Title:** 50-60 chars. Primary keyword near front. "[Benefit]" not "[Feature]."
- **Meta description:** 150-160 chars. Value proposition, not feature list.
- **Headings:** Every 200-400 words. Scannable alone.
- **Internal links:** 3-5 per post minimum.
- **URL slug:** Short, keyword-rich. `/blog/autonomous-convergence` not `/blog/v1-release-march-2026`.
- **Images:** At least one. Alt text matters.

## 4. Code Examples

- Include code in EVERY technical post (2-3x engagement).
- Minimal but complete — copy-paste runnable.
- Syntax-highlighted blocks, never screenshots of code.
- Annotate with comments.
- Before/after comparisons extremely effective.

## 5. Publishing Cadence

- Early stage: 2-4 posts/month (build SEO footprint).
- Growth: 1-2 posts/month (quality over quantity).
- Best days: Tuesday-Thursday.
- Consistency > frequency.

## 6. Anti-Patterns

| Don't | Do Instead |
|-------|-----------|
| "We're excited to announce..." | Lead with reader's problem |
| Feature dump without context | "You can now do Y because X" |
| No code examples | Include runnable code |
| Walls of text | H2/H3 every 200-300 words |
| Bury the lede | Most important thing in first 2 sentences |
| No CTA | End with what the reader can do |

---

## 7. Post Layout Patterns

### 7.1 Full-Width Layout

Content spans the full container width (typically 720-800px max). No sidebar.

```html
<article class="post-layout-full">
  <header class="post-header">
    <div class="post-meta">
      <time datetime="2026-03-15">March 15, 2026</time>
      <span class="separator">&middot;</span>
      <span class="read-time">8 min read</span>
    </div>
    <h1>Autonomous Convergence</h1>
    <p class="post-subtitle">How we eliminated manual QA loops with CruxDev's convergence engine</p>
  </header>

  <figure class="hero-image">
    <img src="/blog/convergence-hero.webp"
         alt="Convergence loop architecture diagram"
         width="1200" height="675"
         loading="eager"
         fetchpriority="high" />
  </figure>

  <div class="post-content prose">
    <!-- Markdown-rendered content -->
  </div>

  <footer class="post-footer">
    <!-- Tags, author, sharing, related posts -->
  </footer>
</article>
```

```css
.post-layout-full {
  max-width: 48rem; /* 768px */
  margin: 0 auto;
  padding: 0 1rem;
}

.post-header {
  text-align: center;
  margin-bottom: 2rem;
}

.post-header h1 {
  font-size: clamp(1.75rem, 4vw, 2.5rem);
  line-height: 1.2;
  margin: 0.5rem 0;
}

.hero-image {
  margin: 0 -1rem 2rem;
}

@media (min-width: 768px) {
  .hero-image {
    margin: 0 -4rem 2rem;
    border-radius: 0.5rem;
    overflow: hidden;
  }
}
```

**Best for:** Most blog posts. Clean, focused reading experience. Stripe and Vercel blogs use this pattern.

### 7.2 Sidebar Layout

Content on left with a sticky sidebar on the right containing TOC, author info, or related links.

```html
<div class="post-layout-sidebar">
  <article class="post-main">
    <header><!-- header --></header>
    <div class="post-content prose"><!-- content --></div>
  </article>

  <aside class="post-sidebar" aria-label="Article navigation">
    <div class="sidebar-sticky">
      <nav class="toc" aria-label="Table of contents">
        <h2>On this page</h2>
        <ul><!-- TOC links --></ul>
      </nav>

      <div class="sidebar-author">
        <img src="/authors/bryan.webp" alt="" width="48" height="48" />
        <div>
          <strong>Bryan</strong>
          <p>Founder, Trinsik Labs</p>
        </div>
      </div>
    </div>
  </aside>
</div>
```

```css
.post-layout-sidebar {
  display: grid;
  grid-template-columns: 1fr;
  gap: 2rem;
  max-width: 72rem;
  margin: 0 auto;
  padding: 0 1rem;
}

@media (min-width: 1024px) {
  .post-layout-sidebar {
    grid-template-columns: 1fr 16rem;
  }
}

.post-main {
  max-width: 48rem;
  min-width: 0; /* Prevent overflow */
}

.sidebar-sticky {
  position: sticky;
  top: 5rem;
}

/* Hide sidebar on mobile, show TOC inline instead */
@media (max-width: 1023px) {
  .post-sidebar {
    display: none;
  }
}
```

**Best for:** Long-form posts (2000+ words), documentation-style articles, tutorial series.

### 7.3 Two-Column Layout

Hero image or key visual on one side, content on the other. Used primarily for the header area, reverting to single column for the body.

```css
.post-header-two-col {
  display: grid;
  grid-template-columns: 1fr;
  gap: 2rem;
  align-items: center;
  margin-bottom: 3rem;
}

@media (min-width: 768px) {
  .post-header-two-col {
    grid-template-columns: 1fr 1fr;
  }
}
```

**Best for:** Product announcements, launch posts with prominent visuals.

---

## 8. Typography Scale (Prose Styling)

### 8.1 Base Typography

```css
.prose {
  font-size: clamp(1rem, 1.5vw, 1.125rem); /* 16-18px */
  line-height: 1.75;
  color: var(--color-text);
  max-width: 65ch; /* Optimal line length */
}

.prose > * + * {
  margin-top: 1.5em;
}
```

### 8.2 Heading Scale

```css
.prose h2 {
  font-size: 1.5em;      /* ~24-27px */
  font-weight: 700;
  line-height: 1.3;
  margin-top: 2.5em;
  margin-bottom: 0.75em;
  letter-spacing: -0.02em;
}

.prose h3 {
  font-size: 1.25em;     /* ~20-22.5px */
  font-weight: 600;
  line-height: 1.4;
  margin-top: 2em;
  margin-bottom: 0.5em;
}

.prose h4 {
  font-size: 1.125em;    /* ~18-20px */
  font-weight: 600;
  line-height: 1.4;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}
```

### 8.3 Code Blocks

```css
/* Inline code */
.prose code:not(pre code) {
  background: var(--color-surface-alt);
  padding: 0.125em 0.375em;
  border-radius: 0.25em;
  font-size: 0.875em;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  word-break: break-word;
}

/* Code blocks */
.prose pre {
  background: var(--color-code-bg);
  border-radius: 0.5rem;
  padding: 1rem 1.25rem;
  overflow-x: auto;
  font-size: 0.875em;
  line-height: 1.7;
  margin: 1.5em -1rem; /* Bleed slightly on mobile */
  tab-size: 2;
}

@media (min-width: 768px) {
  .prose pre {
    margin: 1.5em -1.5rem;
    border-radius: 0.5rem;
  }
}

/* Line numbers */
.prose pre .line-number {
  display: inline-block;
  width: 2.5em;
  text-align: right;
  padding-right: 1em;
  color: var(--color-text-muted);
  user-select: none;
}
```

### 8.4 Blockquotes

```css
.prose blockquote {
  border-left: 3px solid var(--color-primary);
  padding-left: 1.25em;
  margin-left: 0;
  color: var(--color-text-secondary);
  font-style: italic;
}

.prose blockquote p:first-child {
  margin-top: 0;
}
```

### 8.5 Tables

```css
.prose table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875em;
  margin: 1.5em 0;
}

.prose th {
  text-align: left;
  font-weight: 600;
  padding: 0.75em 1em;
  border-bottom: 2px solid var(--color-border);
}

.prose td {
  padding: 0.75em 1em;
  border-bottom: 1px solid var(--color-border);
}

/* Responsive: horizontal scroll on mobile */
.prose .table-wrapper {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}
```

### 8.6 Lists

```css
.prose ul, .prose ol {
  padding-left: 1.5em;
}

.prose li {
  margin-top: 0.5em;
}

.prose li > p {
  margin-top: 0.5em;
}

/* Nested list indentation */
.prose li > ul, .prose li > ol {
  margin-top: 0.5em;
}
```

---

## 9. Reading Progress Indicator

A horizontal bar at the top of the viewport showing scroll progress through the article.

```html
<div class="reading-progress" role="progressbar"
     aria-label="Reading progress"
     aria-valuenow="0" aria-valuemin="0" aria-valuemax="100">
  <div class="reading-progress-bar"></div>
</div>
```

```css
.reading-progress {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 3px;
  background: transparent;
  z-index: 50;
}

.reading-progress-bar {
  height: 100%;
  background: var(--color-primary);
  width: 0%;
  transition: width 0.1s linear;
}
```

```javascript
function initReadingProgress() {
  const article = document.querySelector('.post-content');
  const bar = document.querySelector('.reading-progress-bar');
  const progressContainer = document.querySelector('.reading-progress');

  if (!article || !bar) return;

  function updateProgress() {
    const articleTop = article.offsetTop;
    const articleHeight = article.offsetHeight;
    const windowHeight = window.innerHeight;
    const scrollTop = window.scrollY;

    // Calculate progress relative to the article, not the full page
    const start = articleTop;
    const end = articleTop + articleHeight - windowHeight;
    const progress = Math.min(Math.max((scrollTop - start) / (end - start), 0), 1);

    const percent = Math.round(progress * 100);
    bar.style.width = `${percent}%`;
    progressContainer.setAttribute('aria-valuenow', String(percent));
  }

  window.addEventListener('scroll', updateProgress, { passive: true });
  updateProgress();
}
```

**Key design decisions:**
- Track progress against the **article element** specifically, not the full page height. Exclude the footer, comments, and related posts from the calculation.
- Use `position: fixed` at the very top of the viewport, above all other content.
- 3px height is enough to be visible without being distracting.
- `{ passive: true }` on the scroll listener for performance.

---

## 10. Estimated Read Time

### 10.1 Calculation

```typescript
function estimateReadTime(content: string): number {
  // Average reading speed: 238 words per minute (research-backed)
  const WPM = 238;

  // Count words (strip HTML/markdown)
  const text = content.replace(/<[^>]*>/g, '').replace(/[#*_`~\[\]()]/g, '');
  const words = text.trim().split(/\s+/).length;

  // Add time for code blocks (readers slow down for code)
  const codeBlocks = (content.match(/```[\s\S]*?```/g) || []).length;
  const codeTime = codeBlocks * 0.5; // 30 seconds per code block

  // Add time for images
  const images = (content.match(/<img|!\[/g) || []).length;
  const imageTime = images * (12 / 60); // 12 seconds per image

  const minutes = Math.ceil(words / WPM + codeTime + imageTime);
  return Math.max(1, minutes);
}
```

### 10.2 Display

```html
<span class="read-time">
  <svg aria-hidden="true" width="16" height="16" viewBox="0 0 16 16">
    <circle cx="8" cy="8" r="7" fill="none" stroke="currentColor" stroke-width="1.5"/>
    <path d="M8 4v4l3 2" stroke="currentColor" stroke-width="1.5"
          stroke-linecap="round" stroke-linejoin="round" fill="none"/>
  </svg>
  8 min read
</span>
```

### 10.3 Range Display (Optional)

For very long posts, show a range to set expectations:

```
Short posts (< 5 min): "3 min read"
Medium posts (5-15 min): "8 min read"
Long posts (> 15 min): "15-20 min read"
```

---

## 11. Table of Contents

### 11.1 Auto-Generated from Headings

```typescript
interface TOCItem {
  id: string;
  text: string;
  level: 2 | 3;     // Only h2 and h3 (h4+ is too deep for TOC)
  children: TOCItem[];
}

function generateTOC(content: string): TOCItem[] {
  const headings = content.match(/<h[23][^>]*id="([^"]*)"[^>]*>(.*?)<\/h[23]>/g) || [];
  const toc: TOCItem[] = [];
  let currentH2: TOCItem | null = null;

  headings.forEach(heading => {
    const level = heading.startsWith('<h2') ? 2 : 3;
    const id = heading.match(/id="([^"]*)"/)?.[1] || '';
    const text = heading.replace(/<[^>]*>/g, '').trim();

    const item: TOCItem = { id, text, level, children: [] };

    if (level === 2) {
      toc.push(item);
      currentH2 = item;
    } else if (currentH2) {
      currentH2.children.push(item);
    }
  });

  return toc;
}
```

### 11.2 Sticky Sidebar TOC

```html
<nav class="toc" aria-label="Table of contents">
  <h2 class="toc-title">On this page</h2>
  <ul class="toc-list">
    <li>
      <a href="#architecture" class="toc-link active">Architecture</a>
      <ul>
        <li><a href="#islands" class="toc-link">Islands Pattern</a></li>
        <li><a href="#hydration" class="toc-link">Partial Hydration</a></li>
      </ul>
    </li>
    <li>
      <a href="#performance" class="toc-link">Performance</a>
    </li>
    <li>
      <a href="#results" class="toc-link">Results</a>
    </li>
  </ul>
</nav>
```

```css
.toc {
  font-size: 0.8125rem;
}

.toc-title {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-muted);
  margin-bottom: 0.75rem;
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
  border-left: 1px solid var(--color-border);
}

.toc-link {
  display: block;
  padding: 0.25rem 0 0.25rem 0.75rem;
  color: var(--color-text-secondary);
  text-decoration: none;
  line-height: 1.4;
  border-left: 2px solid transparent;
  margin-left: -1px;
  transition: color 0.15s, border-color 0.15s;
}

.toc-link:hover {
  color: var(--color-text);
}

.toc-link.active {
  color: var(--color-primary);
  border-left-color: var(--color-primary);
  font-weight: 500;
}

/* Nested links */
.toc-list .toc-list {
  border-left: none;
  padding-left: 0.75rem;
}

.toc-list .toc-list .toc-link {
  font-size: 0.75rem;
}
```

### 11.3 Active Section Tracking with IntersectionObserver

```javascript
function initTOCHighlighting() {
  const headings = document.querySelectorAll('.post-content h2, .post-content h3');
  const tocLinks = document.querySelectorAll('.toc-link');

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          // Remove active from all
          tocLinks.forEach(link => link.classList.remove('active'));
          // Add active to matching TOC link
          const activeLink = document.querySelector(`.toc-link[href="#${entry.target.id}"]`);
          if (activeLink) activeLink.classList.add('active');
        }
      });
    },
    {
      rootMargin: '-80px 0px -80% 0px', // Trigger when heading is near top
      threshold: 0,
    }
  );

  headings.forEach(heading => observer.observe(heading));
}
```

### 11.4 Smooth Scroll

```css
html {
  scroll-behavior: smooth;
}

/* Offset for fixed header */
[id] {
  scroll-margin-top: 5rem;
}
```

```javascript
// Respect user preferences
if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
  document.documentElement.style.scrollBehavior = 'auto';
}
```

### 11.5 Mobile TOC

On mobile, show TOC as a collapsible section at the top of the article (before the prose content), not as a sidebar.

```html
<details class="toc-mobile" open>
  <summary>Table of contents</summary>
  <nav aria-label="Table of contents">
    <ul class="toc-list"><!-- same as above --></ul>
  </nav>
</details>
```

**Show TOC only for posts with 4+ headings.** Shorter posts do not need it.

---

## 12. Social Sharing Buttons

### 12.1 Placement Strategies

| Placement | Conversion | Notes |
|-----------|-----------|-------|
| End of post | Highest | Reader has received value, highest intent |
| Floating sidebar (desktop) | Medium | Always visible, sticky on scroll |
| Below title (before content) | Low | Reader hasn't received value yet |
| Inline (next to key content) | Context-dependent | Great for pull quotes or data points |

**Recommendation:** End of post (always) + floating sidebar (desktop only, for long posts).

### 12.2 Which Platforms

For a developer/tech blog:
1. **X (Twitter)** — primary sharing platform for tech content
2. **LinkedIn** — professional sharing, good for methodology/product posts
3. **Copy link** — always include, universal
4. **Hacker News** — optional, for deeply technical content
5. **Reddit** — optional, for community-oriented content

**Do not include:** Facebook (low engagement for tech), Pinterest (wrong audience), WhatsApp (use native share API instead).

### 12.3 Implementation

```html
<div class="share-buttons" aria-label="Share this post">
  <span class="share-label">Share</span>

  <a href="https://twitter.com/intent/tweet?text=Autonomous%20Convergence&url=https://cruxdev.com/blog/autonomous-convergence"
     target="_blank"
     rel="noopener noreferrer"
     aria-label="Share on X (Twitter)"
     class="share-btn">
    <svg aria-hidden="true" width="20" height="20"><!-- X icon --></svg>
  </a>

  <a href="https://www.linkedin.com/shareArticle?mini=true&url=https://cruxdev.com/blog/autonomous-convergence&title=Autonomous%20Convergence"
     target="_blank"
     rel="noopener noreferrer"
     aria-label="Share on LinkedIn"
     class="share-btn">
    <svg aria-hidden="true" width="20" height="20"><!-- LinkedIn icon --></svg>
  </a>

  <button class="share-btn"
          aria-label="Copy link to clipboard"
          onclick="copyLink()">
    <svg aria-hidden="true" width="20" height="20"><!-- Link icon --></svg>
  </button>
</div>
```

```javascript
async function copyLink() {
  try {
    await navigator.clipboard.writeText(window.location.href);
    // Show "Copied!" feedback
    const btn = event.currentTarget;
    btn.setAttribute('aria-label', 'Link copied!');
    setTimeout(() => btn.setAttribute('aria-label', 'Copy link to clipboard'), 2000);
  } catch {
    // Fallback for older browsers
    const input = document.createElement('input');
    input.value = window.location.href;
    document.body.appendChild(input);
    input.select();
    document.execCommand('copy');
    document.body.removeChild(input);
  }
}
```

```css
.share-buttons {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.share-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text-muted);
}

.share-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  background: var(--color-surface-alt);
  color: var(--color-text-secondary);
  border: none;
  cursor: pointer;
  text-decoration: none;
  transition: background-color 0.15s, color 0.15s;
}

.share-btn:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}
```

### 12.4 Share Counts

**Recommendation: Do not show share counts** unless the count is impressive (50+). Low share counts discourage sharing (social proof works both ways). If you do show counts, set a threshold: display only after reaching 10+ shares.

### 12.5 Native Share API (Mobile)

```javascript
// Use Web Share API on supported devices (mobile, some desktop)
if (navigator.share) {
  navigator.share({
    title: document.title,
    text: 'Autonomous Convergence — How we eliminated manual QA loops',
    url: window.location.href,
  });
}
```

---

## 13. Author Bio Box

### 13.1 Compact (End of Post)

```html
<div class="author-bio" aria-label="About the author">
  <img src="/authors/bryan.webp"
       alt=""
       width="64" height="64"
       class="author-avatar"
       loading="lazy" />
  <div class="author-info">
    <div class="author-name">
      <strong>Bryan</strong>
      <span class="author-role">Founder, Trinsik Labs</span>
    </div>
    <p class="author-description">
      Building autonomous development tools. Obsessed with convergence,
      code quality, and eliminating toil.
    </p>
    <div class="author-links">
      <a href="https://twitter.com/bryan" rel="noopener">X</a>
      <a href="https://github.com/bryan" rel="noopener">GitHub</a>
    </div>
  </div>
</div>
```

```css
.author-bio {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--color-surface-alt);
  border-radius: 0.5rem;
  margin: 2rem 0;
}

.author-avatar {
  border-radius: 50%;
  flex-shrink: 0;
  width: 64px;
  height: 64px;
  object-fit: cover;
}

.author-name {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.author-name strong {
  font-size: 1rem;
}

.author-role {
  font-size: 0.8125rem;
  color: var(--color-text-muted);
}

.author-description {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.5;
  margin: 0.25rem 0 0.5rem;
}

.author-links {
  display: flex;
  gap: 0.75rem;
  font-size: 0.8125rem;
}

.author-links a {
  color: var(--color-primary);
  text-decoration: none;
}

@media (max-width: 480px) {
  .author-bio {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }
}
```

---

## 14. Related Posts Section

### 14.1 Algorithm

Priority order for selecting related posts (see BLOG_PATTERNS.md section 2.4 for full implementation):

1. **Same series** — always surface other parts (+100 points)
2. **Shared tags** — weighted by tag specificity (rare tags score higher) (+10 per tag)
3. **Same category** — same broad topic (+5 points)
4. **Recency** — tiebreaker, prefer newer posts (+0-3 points)

### 14.2 Display

```html
<section class="related-posts" aria-labelledby="related-heading">
  <h2 id="related-heading">Related Posts</h2>
  <div class="related-grid">
    <a href="/blog/convergence-engine-design" class="related-card">
      <img src="/blog/engine-thumb.webp" alt="" width="300" height="168"
           loading="lazy" />
      <h3>Convergence Engine Design</h3>
      <time datetime="2026-03-10">Mar 10, 2026</time>
    </a>
    <a href="/blog/tdd-for-everything" class="related-card">
      <img src="/blog/tdd-thumb.webp" alt="" width="300" height="168"
           loading="lazy" />
      <h3>TDD for Everything</h3>
      <time datetime="2026-02-28">Feb 28, 2026</time>
    </a>
    <a href="/blog/rust-migration" class="related-card">
      <img src="/blog/rust-thumb.webp" alt="" width="300" height="168"
           loading="lazy" />
      <h3>Why We Migrated to Rust</h3>
      <time datetime="2026-02-15">Feb 15, 2026</time>
    </a>
  </div>
</section>
```

```css
.related-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

@media (min-width: 640px) {
  .related-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

.related-card {
  text-decoration: none;
  color: inherit;
}

.related-card img {
  width: 100%;
  aspect-ratio: 16/9;
  object-fit: cover;
  border-radius: 0.375rem;
}

.related-card h3 {
  font-size: 0.9375rem;
  margin: 0.5rem 0 0.25rem;
  line-height: 1.3;
}

.related-card time {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}
```

**Show 3 related posts.** 2 feels sparse, 4 pushes important content below the fold.

---

## 15. Previous / Next Navigation

```html
<nav class="post-nav" aria-label="Post navigation">
  <a href="/blog/convergence-engine-design" class="post-nav-link prev">
    <span class="post-nav-label">&larr; Previous</span>
    <span class="post-nav-title">Convergence Engine Design</span>
  </a>
  <a href="/blog/tdd-for-everything" class="post-nav-link next">
    <span class="post-nav-label">Next &rarr;</span>
    <span class="post-nav-title">TDD for Everything</span>
  </a>
</nav>
```

```css
.post-nav {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin: 3rem 0 2rem;
  padding-top: 2rem;
  border-top: 1px solid var(--color-border);
}

.post-nav-link {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 1rem;
  border-radius: 0.5rem;
  text-decoration: none;
  color: inherit;
  transition: background-color 0.15s;
}

.post-nav-link:hover {
  background: var(--color-surface-alt);
}

.post-nav-link.next {
  text-align: right;
}

.post-nav-label {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.post-nav-title {
  font-weight: 500;
  color: var(--color-primary);
}

@media (max-width: 480px) {
  .post-nav {
    grid-template-columns: 1fr;
  }
}
```

**Ordering:** Chronological (previous = older, next = newer). Within a series, follow series order instead.

---

## 16. Series Navigation

### 16.1 Series Header (Top of Post)

```html
<div class="series-nav" aria-label="Series navigation">
  <div class="series-info">
    <span class="series-label">Series</span>
    <strong class="series-name">Building CruxDev</strong>
    <span class="series-position">Part 3 of 5</span>
  </div>
  <div class="series-progress">
    <div class="series-progress-bar" style="width: 60%" aria-hidden="true"></div>
  </div>
  <details class="series-list">
    <summary>View all parts</summary>
    <ol>
      <li><a href="/blog/why-cruxdev">Why CruxDev</a></li>
      <li><a href="/blog/convergence-engine-design">Convergence Engine Design</a></li>
      <li><a href="/blog/autonomous-convergence" aria-current="page">
        <strong>Autonomous Convergence</strong> (current)
      </a></li>
      <li><a href="/blog/tdd-for-everything">TDD for Everything</a></li>
      <li><a href="/blog/rust-migration">Migrating to Rust</a></li>
    </ol>
  </details>
</div>
```

```css
.series-nav {
  background: var(--color-surface-alt);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 1rem 1.25rem;
  margin-bottom: 2rem;
}

.series-info {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.series-label {
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-muted);
}

.series-position {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.series-progress {
  height: 3px;
  background: var(--color-border);
  border-radius: 2px;
  margin: 0.75rem 0;
  overflow: hidden;
}

.series-progress-bar {
  height: 100%;
  background: var(--color-primary);
  border-radius: 2px;
  transition: width 0.3s;
}

.series-list summary {
  font-size: 0.8125rem;
  color: var(--color-primary);
  cursor: pointer;
}

.series-list ol {
  margin: 0.5rem 0 0;
  padding-left: 1.5rem;
  font-size: 0.875rem;
}

.series-list li {
  padding: 0.25rem 0;
}

.series-list a[aria-current="page"] {
  font-weight: 600;
  color: var(--color-text);
}
```

### 16.2 Series Footer (Bottom of Post)

```html
<div class="series-next" aria-label="Next in series">
  <p class="series-next-label">
    Next in <strong>Building CruxDev</strong>
  </p>
  <a href="/blog/tdd-for-everything" class="series-next-link">
    <span class="series-next-part">Part 4 of 5</span>
    <span class="series-next-title">TDD for Everything</span>
    <span class="series-next-arrow">&rarr;</span>
  </a>
</div>
```

---

## 17. Code Syntax Highlighting

### 17.1 Highlighter Comparison

| Highlighter | Engine | Bundle Size | Rendering | Quality | Best For |
|-------------|--------|-------------|-----------|---------|----------|
| **Shiki** | TextMate (VS Code engine) | ~250KB + WASM | Build-time | Best | SSG blogs (Astro, Hugo) |
| **Prism** | Regex tokenizer | ~2KB core + plugins | Client-side | Good | Lightweight needs |
| **highlight.js** | Regex tokenizer | ~30KB | Client-side | Good | Auto-detection needed |

**Recommendation:** Shiki for static sites (run at build time, ship zero JS for highlighting). Prism or highlight.js for dynamic/client-rendered sites.

### 17.2 Shiki Integration (Astro)

Astro uses Shiki by default. Configuration in `astro.config.mjs`:

```javascript
export default defineConfig({
  markdown: {
    shikiConfig: {
      theme: 'github-dark',          // or 'one-dark-pro', 'dracula', etc.
      themes: {
        light: 'github-light',
        dark: 'github-dark',
      },
      wrap: true,                     // word wrap in code blocks
      transformers: [],               // custom transformers
    },
  },
});
```

### 17.3 Code Block Enhancements

**Copy button:**

```html
<div class="code-block">
  <div class="code-header">
    <span class="code-language">rust</span>
    <button class="copy-btn" aria-label="Copy code">
      <svg aria-hidden="true"><!-- copy icon --></svg>
      <span class="copy-text">Copy</span>
    </button>
  </div>
  <pre><code class="language-rust">fn main() {
    println!("Hello, convergence!");
}</code></pre>
</div>
```

```javascript
document.querySelectorAll('.copy-btn').forEach(btn => {
  btn.addEventListener('click', async () => {
    const code = btn.closest('.code-block').querySelector('code').textContent;
    await navigator.clipboard.writeText(code);
    btn.querySelector('.copy-text').textContent = 'Copied!';
    setTimeout(() => {
      btn.querySelector('.copy-text').textContent = 'Copy';
    }, 2000);
  });
});
```

**Line highlighting:**

```css
/* Highlight specific lines (set by build-time transformer) */
.code-block .line.highlighted {
  background: rgba(var(--color-primary-rgb), 0.1);
  border-left: 3px solid var(--color-primary);
  margin-left: -3px;
}

.code-block .line.diff-add {
  background: rgba(0, 255, 0, 0.1);
}

.code-block .line.diff-remove {
  background: rgba(255, 0, 0, 0.1);
}
```

**Filename display:**

```html
<div class="code-block" data-filename="src/engine/convergence.rs">
  <div class="code-header">
    <span class="code-filename">src/engine/convergence.rs</span>
    <button class="copy-btn">Copy</button>
  </div>
  <pre><code>...</code></pre>
</div>
```

---

## 18. Image Gallery / Lightbox Patterns

### 18.1 Inline Image Gallery

```html
<div class="image-gallery" role="region" aria-label="Image gallery">
  <figure class="gallery-item">
    <button class="gallery-trigger" aria-haspopup="dialog">
      <img src="/blog/screenshot-1-thumb.webp"
           alt="Dashboard showing convergence metrics"
           width="400" height="300"
           loading="lazy" />
    </button>
    <figcaption>Convergence metrics dashboard</figcaption>
  </figure>
  <figure class="gallery-item">
    <button class="gallery-trigger" aria-haspopup="dialog">
      <img src="/blog/screenshot-2-thumb.webp"
           alt="Code diff showing before and after"
           width="400" height="300"
           loading="lazy" />
    </button>
    <figcaption>Before and after comparison</figcaption>
  </figure>
</div>
```

```css
.image-gallery {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 1rem;
  margin: 1.5rem 0;
}

.gallery-trigger {
  background: none;
  border: none;
  padding: 0;
  cursor: zoom-in;
  border-radius: 0.375rem;
  overflow: hidden;
}

.gallery-trigger img {
  width: 100%;
  aspect-ratio: 4/3;
  object-fit: cover;
  transition: transform 0.2s;
}

.gallery-trigger:hover img {
  transform: scale(1.03);
}
```

### 18.2 Lightbox

```html
<dialog class="lightbox" aria-label="Image viewer">
  <div class="lightbox-content">
    <img src="" alt="" class="lightbox-image" />
    <p class="lightbox-caption"></p>
  </div>
  <button class="lightbox-close" aria-label="Close image viewer">&times;</button>
  <button class="lightbox-prev" aria-label="Previous image">&larr;</button>
  <button class="lightbox-next" aria-label="Next image">&rarr;</button>
</dialog>
```

**Accessibility requirements:**
- Lightbox must trap focus when open (use `<dialog>` element)
- Escape key closes the lightbox
- Arrow keys navigate between images
- All images have alt text
- Close button is the first focusable element

---

## 19. Embedded Content

### 19.1 YouTube Embeds

```html
<!-- Lazy-loaded YouTube embed (no iframe until click) -->
<div class="video-embed" data-video-id="dQw4w9WgXcQ">
  <button class="video-play" aria-label="Play video: Building CruxDev">
    <img src="https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg"
         alt="Video thumbnail: Building CruxDev"
         loading="lazy"
         width="640" height="360" />
    <svg class="play-icon" aria-hidden="true" viewBox="0 0 68 48">
      <path d="M66.52 7.74c-.78-2.93-2.49-5.41-5.42-6.19C55.79.13 34 0 34 0S12.21.13 6.9 1.55c-2.93.78-4.63 3.26-5.42 6.19C.06 13.05 0 24 0 24s.06 10.95 1.48 16.26c.78 2.93 2.49 5.41 5.42 6.19C12.21 47.87 34 48 34 48s21.79-.13 27.1-1.55c2.93-.78 4.64-3.26 5.42-6.19C67.94 34.95 68 24 68 24s-.06-10.95-1.48-16.26z" fill="red"/>
      <path d="M45 24L27 14v20" fill="white"/>
    </svg>
  </button>
</div>
```

```javascript
document.querySelectorAll('.video-embed').forEach(container => {
  container.querySelector('.video-play').addEventListener('click', () => {
    const id = container.dataset.videoId;
    container.innerHTML = `
      <iframe
        src="https://www.youtube-nocookie.com/embed/${id}?autoplay=1"
        width="640" height="360"
        frameborder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
        allowfullscreen
        title="YouTube video"
        loading="lazy"
      ></iframe>
    `;
  });
});
```

```css
.video-embed {
  position: relative;
  aspect-ratio: 16/9;
  background: #000;
  border-radius: 0.5rem;
  overflow: hidden;
  margin: 1.5rem 0;
}

.video-embed iframe {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.video-play {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  width: 100%;
  height: 100%;
}

.video-play img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.play-icon {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 68px;
  opacity: 0.9;
  transition: opacity 0.15s;
}

.video-play:hover .play-icon {
  opacity: 1;
}
```

**Key:** Use `youtube-nocookie.com` for privacy. Lazy-load the iframe (show thumbnail + play button first).

### 19.2 CodePen / CodeSandbox Embeds

```html
<div class="embed-container">
  <iframe
    src="https://codepen.io/user/embed/abcdef?default-tab=result"
    loading="lazy"
    title="CodePen: Convergence Animation"
    style="width: 100%; aspect-ratio: 16/9; border: 1px solid var(--color-border); border-radius: 0.5rem;"
    sandbox="allow-scripts allow-same-origin"
  ></iframe>
</div>
```

### 19.3 Tweet / X Embeds

Prefer static rendering over embedded scripts. Render the tweet content as a styled blockquote:

```html
<blockquote class="tweet-embed" cite="https://twitter.com/user/status/123456">
  <p>Just shipped autonomous convergence in CruxDev. 2 clean passes = convergence. No more manual QA loops.</p>
  <footer>
    <img src="/authors/bryan.webp" alt="" width="24" height="24" />
    <strong>Bryan</strong>
    <a href="https://twitter.com/user/status/123456" rel="noopener">March 15, 2026</a>
  </footer>
</blockquote>
```

**Why static over embedded:** Twitter's embed script is ~1MB, blocks rendering, and tracks users. A styled blockquote is instant, private, and accessible.

---

## 20. Post Page Element Order

The complete order of elements on a blog post page:

1. Reading progress bar (fixed, top of viewport)
2. Series navigation (if part of a series)
3. Post header (category, date, read time, title, subtitle)
4. Hero image (if present)
5. Table of contents (mobile: collapsible; desktop: sidebar)
6. Post content (prose)
7. Tags
8. Share buttons
9. Author bio box
10. Previous/next navigation (or series next)
11. Related posts (3 cards)
12. Newsletter CTA
13. Comments (if enabled)

---

## 21. Audit Dimensions (Updated)

1. **hook** — starts with reader's problem, not product excitement
2. **structure** — follows 7-section order, headings scannable
3. **code** — examples present, minimal, complete, annotated, with copy button
4. **data** — specific numbers, not vague claims
5. **seo** — title, meta, headings, links, slug, structured data optimized
6. **cta** — clear next action at the end
7. **layout** — appropriate layout pattern (full-width, sidebar, two-column)
8. **typography** — prose-optimized scale, 65ch max width, proper heading hierarchy
9. **progress** — reading progress indicator present for posts 1500+ words
10. **read-time** — estimated read time shown, calculation accounts for code and images
11. **toc** — auto-generated TOC for posts with 4+ headings, active section tracking
12. **sharing** — share buttons present at end of post, copy link always included
13. **author** — author bio box with avatar, name, role, bio, and social links
14. **related** — 3 related posts shown, algorithm uses tags + category + recency
15. **navigation** — previous/next links present; series navigation if applicable
16. **syntax** — code blocks have highlighting, copy button, filename, line highlighting
17. **media** — images optimized, YouTube lazy-loaded, embeds sandboxed
18. **accessibility** — heading hierarchy, alt text, focus management, reduced motion support
