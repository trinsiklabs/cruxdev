# Blog Pagination Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Smashing Magazine, CSS-Tricks, web.dev, MDN, Google Search Central, framerbite.com, Dev.to, Hashnode, Ghost, Medium
**Last updated:** 2026-03-27

---

## 1. Principles

1. **Users must always know where they are.** Page numbers, post counts, and position indicators are mandatory.
2. **Every viewable state must have a URL.** If a user sees a set of posts, they must be able to share that exact view.
3. **Performance scales with visible content, not total content.** Never load 500 post cards to show 12.
4. **Pagination is a navigation pattern, not a design afterthought.** It deserves the same attention as the main nav.
5. **Accessibility is non-negotiable.** Keyboard navigation, screen reader announcements, and focus management for every pagination method.

---

## 2. Layout Styles

### 2.1 Traditional List

Vertical stack of posts with title, excerpt, date, and metadata. The simplest and most scannable layout.

```html
<article class="post-list-item">
  <time datetime="2026-03-15">March 15, 2026</time>
  <h2><a href="/blog/autonomous-convergence">Autonomous Convergence</a></h2>
  <p class="excerpt">CruxDev's convergence engine runs audit-fix-re-audit loops...</p>
  <div class="meta">
    <span class="tag">engineering</span>
    <span class="read-time">8 min read</span>
  </div>
</article>
```

```css
.post-list-item {
  padding: 1.5rem 0;
  border-bottom: 1px solid var(--color-border);
}

.post-list-item time {
  font-size: 0.875rem;
  color: var(--color-text-muted);
}

.post-list-item h2 {
  margin: 0.25rem 0 0.5rem;
  font-size: 1.25rem;
}

.post-list-item .excerpt {
  color: var(--color-text-secondary);
  line-height: 1.6;
}
```

**Best for:** Text-focused blogs, engineering blogs, minimalist designs. Stripe, Vercel, and Cloudflare engineering blogs use this pattern.

### 2.2 Card Grid

Posts displayed as cards in a CSS Grid. Each card contains an image, title, excerpt, and metadata.

```html
<div class="post-grid" role="list">
  <article class="post-card" role="listitem">
    <a href="/blog/autonomous-convergence" class="post-card-link">
      <img src="/blog/convergence-thumb.webp"
           alt="Convergence loop diagram"
           width="400" height="225"
           loading="lazy" />
      <div class="post-card-body">
        <span class="category-badge">Engineering</span>
        <h2>Autonomous Convergence</h2>
        <p>CruxDev's convergence engine runs audit-fix-re-audit loops...</p>
        <footer>
          <time datetime="2026-03-15">Mar 15, 2026</time>
          <span class="read-time">8 min</span>
        </footer>
      </div>
    </a>
  </article>
  <!-- more cards -->
</div>
```

```css
.post-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

@media (min-width: 640px) {
  .post-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (min-width: 1024px) {
  .post-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

.post-card {
  border-radius: 0.5rem;
  overflow: hidden;
  background: var(--color-surface);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.2s, transform 0.2s;
}

.post-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.post-card-link {
  text-decoration: none;
  color: inherit;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.post-card img {
  width: 100%;
  aspect-ratio: 16/9;
  object-fit: cover;
}

.post-card-body {
  padding: 1rem;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.post-card-body h2 {
  font-size: 1.125rem;
  margin: 0.5rem 0;
}

.post-card-body p {
  flex: 1;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  line-height: 1.5;
  /* Clamp to 3 lines */
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-card-body footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--color-text-muted);
  margin-top: 0.75rem;
}
```

**Best for:** Visually rich blogs with hero images, product blogs, marketing content.

### 2.3 Masonry Grid

Variable-height cards arranged in a masonry (Pinterest-like) layout. Items fill available vertical space without uniform row heights.

**CSS-only approach (CSS columns — works today):**

```css
.masonry-grid {
  column-count: 1;
  column-gap: 1.5rem;
}

@media (min-width: 640px) {
  .masonry-grid { column-count: 2; }
}

@media (min-width: 1024px) {
  .masonry-grid { column-count: 3; }
}

.masonry-grid .post-card {
  break-inside: avoid;
  margin-bottom: 1.5rem;
}
```

**CSS Grid Masonry (experimental, not yet in stable browsers):**

```css
/* Firefox behind flag, Safari Technology Preview 234+ */
.masonry-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  grid-template-rows: masonry;
  gap: 1.5rem;
}
```

**JavaScript fallback (for production today):**

```typescript
// Lightweight masonry calculation
function applyMasonry(container: HTMLElement, columns: number) {
  const items = Array.from(container.children) as HTMLElement[];
  const colHeights = new Array(columns).fill(0);
  const gap = 24; // 1.5rem

  items.forEach(item => {
    const shortest = colHeights.indexOf(Math.min(...colHeights));
    item.style.position = 'absolute';
    item.style.left = `${(shortest / columns) * 100}%`;
    item.style.top = `${colHeights[shortest]}px`;
    item.style.width = `${100 / columns}%`;
    colHeights[shortest] += item.offsetHeight + gap;
  });

  container.style.position = 'relative';
  container.style.height = `${Math.max(...colHeights)}px`;
}
```

**Status of native CSS Masonry (March 2026):**
- `grid-template-rows: masonry` — behind flags in Firefox (since v77) and Safari Technology Preview
- `display: grid-lanes` — newer spec from WebKit, in Safari Technology Preview 234+
- No stable browser ships masonry layout unflagged yet
- **Recommendation:** Use CSS columns for simple masonry, JS library for complex layouts, and progressive-enhance with native masonry when it ships

### 2.4 Magazine Layout

Featured post (large) with supporting posts (smaller) in a mixed grid.

```html
<div class="magazine-grid">
  <article class="featured-post">
    <!-- Large card with hero image -->
  </article>
  <article class="secondary-post"><!-- Smaller card --></article>
  <article class="secondary-post"><!-- Smaller card --></article>
  <article class="secondary-post"><!-- Smaller card --></article>
  <article class="secondary-post"><!-- Smaller card --></article>
</div>
```

```css
.magazine-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

@media (min-width: 768px) {
  .magazine-grid {
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: auto auto;
  }

  .featured-post {
    grid-column: 1 / -1;
  }
}

@media (min-width: 1024px) {
  .magazine-grid {
    grid-template-columns: 2fr 1fr;
    grid-template-rows: auto auto;
  }

  .featured-post {
    grid-column: 1;
    grid-row: 1 / 3;
  }
}
```

**Best for:** Homepage blog sections, company blogs with featured content.

### 2.5 Timeline Layout

Posts arranged chronologically with date markers.

```html
<div class="timeline">
  <div class="timeline-year">
    <h2>2026</h2>
  </div>
  <div class="timeline-month">
    <h3>March</h3>
    <ul class="timeline-posts">
      <li>
        <time datetime="2026-03-15">15</time>
        <a href="/blog/autonomous-convergence">Autonomous Convergence</a>
        <span class="tag">engineering</span>
      </li>
    </ul>
  </div>
</div>
```

**Best for:** Archives, changelogs, personal blogs with chronological focus.

---

## 3. Pagination Methods

### 3.1 Numbered Pages (Recommended Default)

```
/blog           → page 1
/blog/page/2    → page 2
/blog/page/3    → page 3
```

```html
<nav class="pagination" aria-label="Blog pagination">
  <a href="/blog"
     class="pagination-btn"
     aria-label="Go to first page"
     aria-current="false">1</a>
  <a href="/blog/page/2"
     class="pagination-btn active"
     aria-label="Page 2, current page"
     aria-current="page">2</a>
  <a href="/blog/page/3"
     class="pagination-btn"
     aria-label="Go to page 3">3</a>
  <span class="pagination-ellipsis" aria-hidden="true">&hellip;</span>
  <a href="/blog/page/12"
     class="pagination-btn"
     aria-label="Go to last page, page 12">12</a>
  <a href="/blog/page/3"
     class="pagination-btn pagination-next"
     aria-label="Go to next page">
    Next &rarr;
  </a>
</nav>
```

```css
.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.25rem;
  margin: 2rem 0;
}

.pagination-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2.5rem;
  height: 2.5rem;
  padding: 0 0.75rem;
  border-radius: 0.375rem;
  text-decoration: none;
  color: var(--color-text);
  font-size: 0.875rem;
  transition: background-color 0.15s;
}

.pagination-btn:hover {
  background-color: var(--color-surface-hover);
}

.pagination-btn.active {
  background-color: var(--color-primary);
  color: white;
  font-weight: 600;
}

.pagination-btn:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

/* Hide on mobile, show abbreviated */
@media (max-width: 640px) {
  .pagination-ellipsis,
  .pagination-btn:not(.active):not(.pagination-next):not(:first-child):not(:last-child) {
    display: none;
  }
}
```

**Pagination number display logic:**

```
Total pages: 12, Current: 6
Show: [1] ... [5] [6] [7] ... [12]

Total pages: 5, Current: 3
Show: [1] [2] [3] [4] [5]

Total pages: 12, Current: 1
Show: [1] [2] [3] ... [12]

Total pages: 12, Current: 12
Show: [1] ... [10] [11] [12]
```

Always show: first page, last page, current page, one neighbor on each side. Ellipsis for gaps.

### 3.2 Load More Button

```html
<div id="post-container" aria-live="polite">
  <!-- Posts rendered here -->
</div>

<button id="load-more"
        class="load-more-btn"
        aria-label="Load more posts"
        data-next-page="2">
  Load More Posts
</button>

<p class="post-count" aria-live="polite">
  Showing 12 of 48 posts
</p>
```

```javascript
const button = document.getElementById('load-more');
const container = document.getElementById('post-container');

button.addEventListener('click', async () => {
  const page = button.dataset.nextPage;
  button.disabled = true;
  button.textContent = 'Loading...';

  try {
    const response = await fetch(`/api/posts?page=${page}`);
    const { posts, hasMore } = await response.json();

    const fragment = document.createDocumentFragment();
    posts.forEach(post => {
      const article = createPostCard(post);
      fragment.appendChild(article);
    });

    container.appendChild(fragment);

    // Focus the first new post for accessibility
    const firstNew = container.querySelector(`[data-page="${page}"]`);
    if (firstNew) firstNew.focus();

    // Update URL without reload (enables sharing)
    history.replaceState(null, '', `/blog?page=${page}`);

    button.dataset.nextPage = String(Number(page) + 1);
    button.disabled = false;
    button.textContent = 'Load More Posts';

    if (!hasMore) button.remove();
  } catch (err) {
    button.disabled = false;
    button.textContent = 'Failed to load. Try again.';
  }
});
```

**When to use:** Content-browsing contexts where users want to see more without page navigation. Good compromise between numbered pages and infinite scroll.

### 3.3 Infinite Scroll

Auto-loads content when the user scrolls near the bottom.

```javascript
const observer = new IntersectionObserver(
  (entries) => {
    if (entries[0].isIntersecting) {
      loadNextPage();
    }
  },
  { rootMargin: '200px' }
);

const sentinel = document.getElementById('scroll-sentinel');
observer.observe(sentinel);
```

```html
<div id="post-container" aria-live="polite">
  <!-- Posts -->
</div>
<div id="scroll-sentinel" aria-hidden="true"></div>
<p class="scroll-status" role="status" aria-live="polite">
  Loading more posts...
</p>
```

**Critical requirements for infinite scroll:**
- MUST update URL with History API so users can share/bookmark position
- MUST provide a footer escape (stop auto-loading after N batches, show "Load More" button)
- MUST announce new content to screen readers via `aria-live`
- MUST include a "Back to top" button
- SHOULD preserve scroll position on back-button navigation

**When to use:** Image-heavy browsing (Pinterest-style), casual content discovery. Generally NOT recommended for text blogs (users want to reach the footer, share specific pages).

### 3.4 Cursor-Based Pagination

```
/api/posts?cursor=eyJkIjoiMjAyNi0wMy0xNSJ9&limit=12
```

Used for API-driven pagination where offset-based pagination is unreliable (new content being added shifts offsets). The cursor encodes the last item's sort key.

**When to use:** Real-time feeds, API-first architectures, databases where OFFSET is expensive.

---

## 4. Card Designs

### 4.1 Minimal Card (Title + Date)

```html
<article class="card-minimal">
  <time datetime="2026-03-15">Mar 15</time>
  <h3><a href="/blog/autonomous-convergence">Autonomous Convergence</a></h3>
</article>
```

```css
.card-minimal {
  display: flex;
  align-items: baseline;
  gap: 1rem;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--color-border);
}

.card-minimal time {
  flex-shrink: 0;
  font-size: 0.875rem;
  color: var(--color-text-muted);
  font-variant-numeric: tabular-nums;
  min-width: 4rem;
}

.card-minimal h3 {
  font-size: 1rem;
  margin: 0;
}
```

**Best for:** Archive pages, sidebar "recent posts", minimal designs (Dan Abramov's blog, Paul Graham's essays).

### 4.2 Standard Card (Title + Excerpt + Date + Tags)

```html
<article class="card-standard">
  <div class="card-meta">
    <time datetime="2026-03-15">March 15, 2026</time>
    <span class="separator" aria-hidden="true">&middot;</span>
    <span class="read-time">8 min read</span>
  </div>
  <h3><a href="/blog/autonomous-convergence">Autonomous Convergence</a></h3>
  <p>CruxDev's convergence engine runs audit-fix-re-audit loops without human intervention...</p>
  <div class="card-tags">
    <a href="/blog/tag/convergence" class="tag-pill">convergence</a>
    <a href="/blog/tag/automation" class="tag-pill">automation</a>
  </div>
</article>
```

**Best for:** Most blog listing pages. Good balance of information density and scannability.

### 4.3 Rich Card (Image + Title + Excerpt + Author + Date + Tags + Read Time)

```html
<article class="card-rich">
  <a href="/blog/autonomous-convergence" class="card-image-link">
    <img src="/blog/convergence-thumb.webp"
         alt="Convergence loop diagram"
         width="400" height="225"
         loading="lazy" />
  </a>
  <div class="card-content">
    <div class="card-meta">
      <span class="category-badge">Engineering</span>
      <time datetime="2026-03-15">Mar 15, 2026</time>
    </div>
    <h3><a href="/blog/autonomous-convergence">Autonomous Convergence</a></h3>
    <p>CruxDev's convergence engine runs audit-fix-re-audit loops...</p>
    <footer class="card-footer">
      <div class="author">
        <img src="/authors/bryan.webp" alt="" width="24" height="24"
             class="author-avatar" />
        <span>Bryan</span>
      </div>
      <div class="card-meta-right">
        <span class="read-time">8 min read</span>
      </div>
    </footer>
    <div class="card-tags">
      <a href="/blog/tag/convergence" class="tag-pill">convergence</a>
      <a href="/blog/tag/automation" class="tag-pill">automation</a>
    </div>
  </div>
</article>
```

**Best for:** Marketing blogs, product blogs, sites where visual richness drives engagement.

---

## 5. Responsive Patterns

### 5.1 Column Adaptation

```
Mobile  (< 640px):  1 column, full-width cards
Tablet  (640-1023px): 2 columns
Desktop (1024px+):   3 columns
Wide    (1280px+):   3 columns with max-width container (don't go to 4)
```

**Principle:** 3 columns is the maximum for blog cards. 4+ columns makes excerpts too narrow to read.

### 5.2 Card Size Adaptation

On mobile:
- Image switches from side-by-side to stacked on top
- Excerpts may be hidden or truncated to 2 lines
- Tags shown as max 2 visible + "+N more"
- Author avatar may be hidden

```css
/* Mobile-first: stacked card */
.card-rich {
  display: flex;
  flex-direction: column;
}

/* Tablet+: horizontal card for list view */
@media (min-width: 640px) {
  .card-rich.horizontal {
    flex-direction: row;
    gap: 1rem;
  }

  .card-rich.horizontal .card-image-link {
    flex-shrink: 0;
    width: 200px;
  }
}
```

### 5.3 Pagination on Mobile

- Show only: Previous, Current Page, Next, and Last
- Make touch targets at least 44x44px
- Consider "Load More" button instead of numbered pages on mobile
- Stack pagination controls vertically if needed

---

## 6. Sort Options

| Sort | URL Parameter | Default | Notes |
|------|--------------|---------|-------|
| Newest first | `?sort=newest` | **Yes** | Almost always the right default |
| Oldest first | `?sort=oldest` | No | Useful for series/tutorials |
| Most popular | `?sort=popular` | No | Requires view/engagement tracking |
| Alphabetical | `?sort=alpha` | No | Rarely useful for blogs |

```html
<div class="sort-controls" role="group" aria-label="Sort posts">
  <label for="sort-select" class="sr-only">Sort by</label>
  <select id="sort-select" onchange="updateSort(this.value)">
    <option value="newest" selected>Newest first</option>
    <option value="oldest">Oldest first</option>
    <option value="popular">Most popular</option>
  </select>
</div>
```

**Principle:** Sort state must be reflected in the URL so it persists on page refresh and can be shared.

---

## 7. Filter and Facet Patterns

### 7.1 Filter Types

```html
<aside class="blog-filters" aria-label="Filter posts">
  <!-- Category filter (single select) -->
  <fieldset>
    <legend>Category</legend>
    <label><input type="radio" name="category" value="all" checked /> All</label>
    <label><input type="radio" name="category" value="engineering" /> Engineering</label>
    <label><input type="radio" name="category" value="product" /> Product</label>
  </fieldset>

  <!-- Tag filter (multi-select) -->
  <fieldset>
    <legend>Tags</legend>
    <label><input type="checkbox" name="tag" value="rust" /> Rust</label>
    <label><input type="checkbox" name="tag" value="convergence" /> Convergence</label>
    <label><input type="checkbox" name="tag" value="testing" /> Testing</label>
  </fieldset>

  <!-- Date range -->
  <fieldset>
    <legend>Date range</legend>
    <label>From: <input type="date" name="from" /></label>
    <label>To: <input type="date" name="to" /></label>
  </fieldset>
</aside>
```

### 7.2 Active Filter Chips

```html
<div class="active-filters" role="region" aria-label="Active filters">
  <span class="filter-chip">
    engineering
    <button aria-label="Remove engineering filter">&times;</button>
  </span>
  <span class="filter-chip">
    rust
    <button aria-label="Remove rust filter">&times;</button>
  </span>
  <button class="clear-all">Clear all filters</button>
</div>
```

```css
.filter-chip {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 9999px;
  background: var(--color-surface-alt);
  font-size: 0.8125rem;
}

.filter-chip button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0 0.125rem;
  font-size: 1rem;
  line-height: 1;
  color: var(--color-text-muted);
}
```

### 7.3 URL Structure for Filters

```
/blog?category=engineering&tag=rust&tag=convergence&sort=newest&page=1
```

**Rules:**
- All filter state in URL parameters (not hash fragments)
- Reset to page 1 when filters change
- Combined filters use AND logic (narrowing)
- Multiple tags use OR logic within the tag group

---

## 8. Empty States

### 8.1 No Posts Yet

```html
<div class="empty-state" role="status">
  <h2>No posts yet</h2>
  <p>We're working on our first blog post. Check back soon!</p>
</div>
```

### 8.2 No Results for Filter

```html
<div class="empty-state" role="status">
  <h2>No posts match your filters</h2>
  <p>Try removing some filters or <button class="link-btn" onclick="clearFilters()">clear all filters</button>.</p>
</div>
```

### 8.3 Search with No Matches

```html
<div class="empty-state" role="status">
  <h2>No results for "quamtum computing"</h2>
  <p>Did you mean <a href="/blog?q=quantum+computing">"quantum computing"</a>?</p>
  <p>Try different keywords or <a href="/blog">browse all posts</a>.</p>
</div>
```

**Principle:** Empty states must always offer a next action. Never show just "No results."

---

## 9. Loading States

### 9.1 Skeleton Screens

```html
<div class="post-grid" aria-busy="true" aria-label="Loading posts">
  <div class="skeleton-card" aria-hidden="true">
    <div class="skeleton skeleton-image"></div>
    <div class="skeleton skeleton-title"></div>
    <div class="skeleton skeleton-text"></div>
    <div class="skeleton skeleton-text short"></div>
  </div>
  <!-- Repeat for expected card count -->
</div>
```

```css
.skeleton {
  background: linear-gradient(
    90deg,
    var(--color-surface-alt) 25%,
    var(--color-surface-hover) 50%,
    var(--color-surface-alt) 75%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
  border-radius: 0.25rem;
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

.skeleton-image {
  aspect-ratio: 16/9;
  width: 100%;
}

.skeleton-title {
  height: 1.5rem;
  width: 75%;
  margin: 1rem 0 0.5rem;
}

.skeleton-text {
  height: 1rem;
  width: 100%;
  margin: 0.25rem 0;
}

.skeleton-text.short {
  width: 60%;
}
```

### 9.2 Progressive Loading

For "Load More" and infinite scroll, show a loading spinner below existing content:

```html
<div class="loading-indicator" role="status" aria-live="polite">
  <svg class="spinner" viewBox="0 0 24 24" aria-hidden="true">
    <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor"
            stroke-width="2" stroke-dasharray="31.4" stroke-dashoffset="10" />
  </svg>
  <span>Loading more posts...</span>
</div>
```

**Principle:** Match skeleton card count to the expected number of results. Show 6 skeletons if each page has 6 posts. This reduces perceived layout shift.

---

## 10. URL Structure for Paginated Pages

| Approach | URL | SEO Impact | User Experience |
|----------|-----|------------|-----------------|
| Path-based (recommended) | `/blog/page/2` | Clean, crawlable | Shareable, bookmarkable |
| Query parameter | `/blog?page=2` | Works fine | Shareable |
| No URL change (infinite scroll) | `/blog` | **Problem**: user cannot share position | Cannot share, back button breaks |

**Recommendation:** `/blog/page/2` for SSG blogs, `/blog?page=2` for dynamic blogs. Never infinite scroll without URL updates.

**Page 1 should be `/blog` not `/blog/page/1`.** Redirect `/blog/page/1` to `/blog` with 301.

---

## 11. SEO for Pagination

### 11.1 Canonical Tags

```html
<!-- Page 1: /blog -->
<link rel="canonical" href="https://cruxdev.com/blog" />

<!-- Page 2: /blog/page/2 -->
<link rel="canonical" href="https://cruxdev.com/blog/page/2" />
```

**Every paginated page gets a self-referencing canonical.** Do NOT canonical all pages to page 1 (this tells Google pages 2+ are duplicates, which they are not).

### 11.2 rel=prev/next (Deprecated but Harmless)

Google deprecated `rel=prev/next` in 2019 and no longer uses it. However, other search engines (Bing) and some feed readers still respect it.

```html
<!-- Page 2 -->
<link rel="prev" href="https://cruxdev.com/blog" />
<link rel="next" href="https://cruxdev.com/blog/page/3" />
```

Include it if easy to generate (most SSGs do). It does not hurt and may help non-Google crawlers.

### 11.3 Noindex Considerations

**Do NOT noindex paginated pages by default.** Google discovers posts through paginated listings. If you noindex page 2+, posts only linked from those pages may not get crawled.

**Exception:** Noindex filter/sort variations that create near-duplicate listings (e.g., `/blog?sort=oldest&page=2`).

### 11.4 Posts Per Page

- **12 posts per page** is a good default for card grids
- **10 posts per page** for list layouts
- **20-25 posts per page** for minimal/archive layouts
- Never fewer than 6 (too many pages for small blogs)
- Never more than 50 (too slow to load, too much to scan)

---

## 12. Performance

### 12.1 Virtualized Lists

For blogs with 100+ posts on a single page (archive/search results), use virtual scrolling:

```typescript
// Concept: only render visible items + buffer
function VirtualList({ items, itemHeight, containerHeight }) {
  const [scrollTop, setScrollTop] = useState(0);

  const startIndex = Math.floor(scrollTop / itemHeight);
  const endIndex = Math.min(
    startIndex + Math.ceil(containerHeight / itemHeight) + 2, // +2 buffer
    items.length
  );

  const visibleItems = items.slice(startIndex, endIndex);
  const totalHeight = items.length * itemHeight;
  const offsetY = startIndex * itemHeight;

  return (
    <div style={{ height: containerHeight, overflow: 'auto' }}
         onScroll={e => setScrollTop(e.currentTarget.scrollTop)}>
      <div style={{ height: totalHeight, position: 'relative' }}>
        <div style={{ transform: `translateY(${offsetY}px)` }}>
          {visibleItems.map(item => <PostCard key={item.slug} post={item} />)}
        </div>
      </div>
    </div>
  );
}
```

**In practice:** Prefer server-side pagination over virtual lists. Virtual lists are a last resort for all-on-one-page requirements.

### 12.2 Lazy Image Loading

All post card images below the fold must use `loading="lazy"`. First 3-6 visible cards (above fold) use `loading="eager"`.

### 12.3 Prefetch Next Page

```html
<!-- On page 2, prefetch page 3 -->
<link rel="prefetch" href="/blog/page/3" />
```

Only prefetch the immediate next page. Do not prefetch all pages.

---

## 13. Accessibility

### 13.1 ARIA Live Regions

When content loads dynamically (Load More, infinite scroll), announce it:

```html
<div aria-live="polite" aria-atomic="false" class="sr-only" id="pagination-status">
  <!-- Updated by JavaScript -->
</div>
```

```javascript
function announceNewContent(count, total) {
  document.getElementById('pagination-status').textContent =
    `Loaded ${count} more posts. Showing ${total} of ${totalPosts} total posts.`;
}
```

### 13.2 Focus Management

After loading new content via "Load More":
1. Announce the load via `aria-live`
2. Move focus to the first new post card
3. Do NOT move focus on infinite scroll (disorienting)

After navigating to a new page (page 2):
1. Focus moves to the first post on the new page
2. Or to a "skip to content" link if the page reloads

### 13.3 Keyboard Navigation

- All pagination controls must be keyboard-accessible
- Current page must be indicated with `aria-current="page"`
- Disabled previous/next must use `aria-disabled="true"`, not remove the element
- Tab order: filters, sort, post cards, pagination controls

### 13.4 Pagination Landmark

```html
<nav aria-label="Blog pagination">
  <!-- pagination controls -->
</nav>
```

Use `<nav>` with a descriptive `aria-label` so screen reader users can jump to pagination.

---

## 14. Anti-Patterns

| Anti-Pattern | Why It Is Wrong | Do Instead |
|---|---|---|
| Infinite scroll without URL updates | Users cannot share position, back button breaks | Update URL with History API |
| Infinite scroll without stop | Users can never reach footer | Stop after 3-5 loads, show "Load More" |
| Paginating with only Prev/Next (no numbers) | Users cannot jump to a specific page or see total | Show page numbers with ellipsis |
| Loading full post content in card listings | Massive page weight, slow rendering | Load title + excerpt + thumbnail only |
| Different card heights in a grid without masonry | Ugly gaps, wasted space | Use masonry layout or equalize heights |
| No empty state | Users think the page is broken | Always show a helpful empty state with next action |
| Skeletons that do not match final layout | Jarring layout shift when content loads | Match skeleton dimensions to real cards |
| Filter state only in JavaScript (not URL) | Filters lost on refresh, not shareable | Sync all filter state to URL parameters |
| Noindexing all pages beyond page 1 | Google may not discover posts linked from later pages | Self-referencing canonical on each page |
| Page 1 at both `/blog` and `/blog/page/1` | Duplicate content | 301 redirect `/blog/page/1` to `/blog` |

---

## 15. Audit Dimensions

1. **layout** — appropriate style for content type (list/grid/masonry/magazine)
2. **pagination-method** — URL-backed, appropriate for content volume, SEO-safe
3. **card-design** — consistent, appropriate information density, responsive
4. **responsive** — 1-col mobile, 2-col tablet, 3-col desktop; touch-friendly controls
5. **sort-filter** — state in URL, appropriate options, combined filters work correctly
6. **empty-states** — helpful message with next action for every empty scenario
7. **loading-states** — skeletons match final layout, announcements for dynamic loading
8. **seo** — self-referencing canonical, no unnecessary noindex, valid pagination URLs
9. **performance** — lazy images, prefetch next page, no unnecessary JS
10. **accessibility** — ARIA live regions, focus management, keyboard navigation, current page indicated
