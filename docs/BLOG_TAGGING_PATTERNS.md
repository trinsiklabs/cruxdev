# Blog Tagging Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** WPBeginner, Jetpack, Finsweet, Smashing Magazine, CSS-Tricks, Google Search Central, rootid.com, SEO research blogs
**Last updated:** 2026-03-27

---

## 1. Principles

1. **Taxonomy serves navigation.** If a taxonomy does not help users find content, it should not exist.
2. **Categories are the table of contents. Tags are the index.** Categories organize broadly (5-10 max). Tags describe specifically (20-30 max across the blog).
3. **Every taxonomy page must earn its existence.** A tag page with 1 post is SEO-harmful and user-hostile. Minimum 3 posts before a tag gets its own page.
4. **Taxonomy is a design decision, not an afterthought.** Plan it before writing the first post. Refactor it reluctantly.
5. **Flat over deep.** One level of categories is almost always enough. Nested categories add complexity without proportional value.

---

## 2. Tags vs Categories

### 2.1 When to Use Each

| Aspect | Categories | Tags |
|--------|-----------|------|
| **Purpose** | Broad content type/topic area | Specific subjects within a post |
| **Hierarchy** | Hierarchical (parent/child supported) | Flat (no hierarchy) |
| **Assignment** | One per post (recommended) | Multiple per post (3-7 ideal) |
| **Required** | Yes, every post needs a category | No, but recommended |
| **Total count** | 5-10 across the entire blog | 20-30 across the entire blog |
| **URL** | `/blog/category/engineering` | `/blog/tag/rust` |
| **Analogy** | Chapters in a book | Index entries at the back |
| **Changes** | Rarely (restructuring is expensive) | Occasionally (new topics emerge) |

### 2.2 Example Taxonomy for a Dev Tools Blog

**Categories (6):**
- Engineering (how we build things)
- Product (what we ship, announcements)
- Methodology (processes, patterns)
- Research (industry analysis, benchmarks)
- Announcement (releases, milestones)
- Tutorial (step-by-step guides)

**Tags (sample, grows organically):**
- rust, typescript, python
- convergence, testing, CI/CD
- performance, security, accessibility
- open-source, architecture, DX

### 2.3 Decision Framework

```
Is this a BROAD topic that will have 10+ posts over the blog's lifetime?
  YES → Category
  NO → Tag (or nothing)

Would a reader want to see ALL posts on this topic as a browsable collection?
  YES → Category or prominent tag
  NO → Tag (for cross-referencing only)

Does this term already exist in your taxonomy?
  YES → Reuse it. Do not create "Rust" and "rust" and "Rust language"
  NO → Add it only if you expect 3+ posts to use it
```

---

## 3. Tag Page Patterns

### 3.1 Individual Tag Page

```
URL: /blog/tag/rust
```

```html
<main>
  <header class="tag-header">
    <h1>Posts tagged "rust"</h1>
    <p class="tag-count">12 posts</p>
    <p class="tag-description">
      Posts about the Rust programming language, including our migration
      from Python and performance benchmarks.
    </p>
    <link rel="alternate" type="application/atom+xml"
          title="CruxDev - Rust posts"
          href="/blog/tag/rust/feed.xml" />
  </header>

  <div class="post-list">
    <!-- Standard post cards, newest first -->
  </div>

  <nav class="pagination" aria-label="Rust posts pagination">
    <!-- Pagination if > 12 posts -->
  </nav>
</main>
```

**Requirements:**
- Descriptive `<h1>` that includes the tag name
- Post count
- Optional: tag description (useful for SEO, set in tag metadata)
- Sorted newest first (default) with sort option
- Paginated if more than 12 posts
- RSS feed for the tag (optional but appreciated by power users)

### 3.2 Tag Index Page (All Tags)

```
URL: /blog/tags
```

```html
<main>
  <h1>All Topics</h1>
  <p>Browse posts by topic. Numbers show post count.</p>

  <div class="tag-index" role="list">
    <a href="/blog/tag/rust" class="tag-index-item" role="listitem">
      <span class="tag-name">rust</span>
      <span class="tag-count" aria-label="12 posts">12</span>
    </a>
    <a href="/blog/tag/convergence" class="tag-index-item" role="listitem">
      <span class="tag-name">convergence</span>
      <span class="tag-count" aria-label="8 posts">8</span>
    </a>
    <!-- ... -->
  </div>
</main>
```

```css
.tag-index {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag-index-item {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem 0.75rem;
  border-radius: 9999px;
  background: var(--color-surface-alt);
  text-decoration: none;
  color: var(--color-text);
  font-size: 0.875rem;
  transition: background-color 0.15s;
}

.tag-index-item:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.tag-count {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  background: var(--color-surface);
  border-radius: 9999px;
  padding: 0.125rem 0.375rem;
  min-width: 1.25rem;
  text-align: center;
}
```

### 3.3 Tag Cloud (Weighted by Count)

```html
<div class="tag-cloud" role="list" aria-label="Topics by popularity">
  <a href="/blog/tag/rust" role="listitem"
     style="--weight: 1.5" class="tag-cloud-item">rust (12)</a>
  <a href="/blog/tag/testing" role="listitem"
     style="--weight: 1.2" class="tag-cloud-item">testing (8)</a>
  <a href="/blog/tag/css" role="listitem"
     style="--weight: 0.8" class="tag-cloud-item">css (3)</a>
</div>
```

```css
.tag-cloud {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem 0.75rem;
  justify-content: center;
  align-items: center;
}

.tag-cloud-item {
  font-size: calc(0.75rem + var(--weight) * 0.5rem);
  /* Range: 0.75rem (smallest) to ~1.5rem (largest) */
  text-decoration: none;
  color: var(--color-text-secondary);
  transition: color 0.15s;
}

.tag-cloud-item:hover {
  color: var(--color-primary);
}
```

**Weight calculation:**

```typescript
function calculateTagWeights(tags: { name: string; count: number }[]) {
  const counts = tags.map(t => t.count);
  const min = Math.min(...counts);
  const max = Math.max(...counts);
  const range = max - min || 1;

  return tags.map(tag => ({
    ...tag,
    weight: 0.5 + ((tag.count - min) / range) * 1.5, // Range: 0.5 to 2.0
  }));
}
```

**When to use tag clouds:**
- Blog has 15+ tags with varied counts
- Discovery and exploration are primary goals
- Sidebar or footer widget

**When NOT to use tag clouds:**
- Blog has fewer than 10 tags (flat list is better)
- All tags have similar counts (no visual differentiation)
- Mobile-primary audience (tag clouds are hard to tap)

---

## 4. Category Page Patterns

### 4.1 Individual Category Page

```
URL: /blog/category/engineering
```

```html
<main>
  <header class="category-header">
    <h1>Engineering</h1>
    <p class="category-description">
      How we build CruxDev: architecture decisions, performance optimization,
      and technical deep dives.
    </p>
    <p class="post-count">24 posts</p>
  </header>

  <!-- Optional: subcategory navigation -->
  <nav class="subcategories" aria-label="Subcategories">
    <a href="/blog/category/engineering" class="active" aria-current="page">All</a>
    <a href="/blog/category/engineering/architecture">Architecture</a>
    <a href="/blog/category/engineering/performance">Performance</a>
  </nav>

  <div class="post-list">
    <!-- Post cards -->
  </div>
</main>
```

### 4.2 Category Index (All Categories)

```html
<main>
  <h1>Categories</h1>
  <div class="category-grid">
    <a href="/blog/category/engineering" class="category-card">
      <h2>Engineering</h2>
      <p>Architecture, performance, and technical deep dives.</p>
      <span class="post-count">24 posts</span>
    </a>
    <a href="/blog/category/product" class="category-card">
      <h2>Product</h2>
      <p>Features, releases, and what we're shipping.</p>
      <span class="post-count">18 posts</span>
    </a>
  </div>
</main>
```

```css
.category-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.category-card {
  padding: 1.5rem;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  text-decoration: none;
  color: inherit;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.category-card:hover {
  border-color: var(--color-primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.category-card h2 {
  font-size: 1.25rem;
  margin: 0 0 0.5rem;
}

.category-card p {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  margin: 0 0 0.75rem;
}

.category-card .post-count {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}
```

### 4.3 Nested Categories

**Recommendation: Avoid nesting deeper than 2 levels.**

```
engineering/               → /blog/category/engineering
  architecture/            → /blog/category/engineering/architecture
  performance/             → /blog/category/engineering/performance
```

**Why limit depth:**
- Deep hierarchies confuse users
- Deep URLs dilute SEO value
- Most blogs never have enough content to justify 3+ levels
- Tags handle the specificity that nested categories try to provide

---

## 5. Tag Cloud Designs

### 5.1 Flat List (Simplest, Recommended)

```html
<ul class="tag-list" role="list">
  <li><a href="/blog/tag/rust">rust <span class="count">(12)</span></a></li>
  <li><a href="/blog/tag/testing">testing <span class="count">(8)</span></a></li>
  <li><a href="/blog/tag/css">css <span class="count">(3)</span></a></li>
</ul>
```

Alphabetically sorted. Count shown in parentheses. Simplest to scan and use on any device.

### 5.2 Weighted Cloud (Variable Font Size)

See section 3.3 above. Visual weight proportional to post count. Good for discovery, poor for precise navigation.

### 5.3 Grouped by Letter

```html
<div class="tag-alphabet">
  <section>
    <h2 id="letter-a">A</h2>
    <ul aria-labelledby="letter-a">
      <li><a href="/blog/tag/accessibility">accessibility (5)</a></li>
      <li><a href="/blog/tag/architecture">architecture (7)</a></li>
      <li><a href="/blog/tag/astro">astro (4)</a></li>
    </ul>
  </section>
  <section>
    <h2 id="letter-c">C</h2>
    <ul aria-labelledby="letter-c">
      <li><a href="/blog/tag/convergence">convergence (8)</a></li>
      <li><a href="/blog/tag/css">css (3)</a></li>
    </ul>
  </section>
</div>
```

Best for blogs with 30+ tags. Provides quick alphabetical navigation.

### 5.4 Colored Tags (By Category)

```html
<a href="/blog/tag/rust" class="tag-pill tag-language">rust</a>
<a href="/blog/tag/convergence" class="tag-pill tag-concept">convergence</a>
<a href="/blog/tag/testing" class="tag-pill tag-practice">testing</a>
```

```css
.tag-pill {
  display: inline-flex;
  align-items: center;
  padding: 0.25rem 0.625rem;
  border-radius: 9999px;
  font-size: 0.8125rem;
  text-decoration: none;
  font-weight: 500;
}

/* Color by meta-category */
.tag-language {
  background: #dbeafe;
  color: #1d4ed8;
}
.tag-concept {
  background: #f3e8ff;
  color: #7c3aed;
}
.tag-practice {
  background: #dcfce7;
  color: #16a34a;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .tag-language {
    background: #1e3a5f;
    color: #93c5fd;
  }
  .tag-concept {
    background: #3b1f5e;
    color: #c4b5fd;
  }
  .tag-practice {
    background: #14532d;
    color: #86efac;
  }
}
```

**Accessibility requirement:** Color must not be the only differentiator. Add an icon, prefix, or tooltip. Test all color combinations for 4.5:1 contrast ratio.

---

## 6. Tag UI Components

### 6.1 Pill Badges (In Post Cards)

```html
<div class="post-tags">
  <a href="/blog/tag/rust" class="tag-pill">rust</a>
  <a href="/blog/tag/performance" class="tag-pill">performance</a>
  <a href="/blog/tag/convergence" class="tag-pill">convergence</a>
</div>
```

```css
.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.375rem;
}

.tag-pill {
  padding: 0.125rem 0.5rem;
  border-radius: 9999px;
  background: var(--color-surface-alt);
  color: var(--color-text-secondary);
  font-size: 0.75rem;
  text-decoration: none;
  transition: background-color 0.15s, color 0.15s;
  white-space: nowrap;
}

.tag-pill:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.tag-pill:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}
```

### 6.2 Removable Filter Chips

```html
<div class="active-filters" aria-label="Active tag filters">
  <button class="filter-chip" aria-label="Remove rust filter">
    rust
    <svg class="chip-close" aria-hidden="true" viewBox="0 0 16 16" width="14" height="14">
      <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5"
            stroke-linecap="round" />
    </svg>
  </button>
  <button class="filter-chip" aria-label="Remove performance filter">
    performance
    <svg class="chip-close" aria-hidden="true" viewBox="0 0 16 16" width="14" height="14">
      <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5"
            stroke-linecap="round" />
    </svg>
  </button>
</div>
```

```css
.filter-chip {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem 0.25rem 0.625rem;
  border: 1px solid var(--color-primary);
  border-radius: 9999px;
  background: var(--color-primary-light);
  color: var(--color-primary);
  font-size: 0.8125rem;
  cursor: pointer;
  transition: background-color 0.15s;
}

.filter-chip:hover {
  background: var(--color-primary);
  color: white;
}

.chip-close {
  flex-shrink: 0;
}
```

### 6.3 Tag Input (Admin/Editor)

For content management systems where authors select tags:

```html
<div class="tag-input-container">
  <label for="tag-input">Tags</label>
  <div class="tag-input-field" role="combobox" aria-expanded="false">
    <span class="selected-tag">
      rust <button aria-label="Remove rust">&times;</button>
    </span>
    <input type="text" id="tag-input"
           placeholder="Add tag..."
           autocomplete="off"
           aria-autocomplete="list"
           aria-controls="tag-suggestions" />
  </div>
  <ul id="tag-suggestions" role="listbox" hidden>
    <li role="option">convergence (8 posts)</li>
    <li role="option">testing (6 posts)</li>
  </ul>
  <p class="help-text">Select from existing tags or create new ones. 3-7 tags per post.</p>
</div>
```

---

## 7. Taxonomy URL Structure

### 7.1 URL Patterns

| Pattern | Example | Recommendation |
|---------|---------|----------------|
| `/blog/tag/rust` | Tag under blog | **Recommended** — scoped to blog |
| `/tag/rust` | Tag at root | Acceptable for blog-only sites |
| `/blog/tags/rust` | Plural prefix | Inconsistent (is it tags or tag?) |
| `/topics/rust` | Alternative naming | Fine if "topics" is your brand term |

**Chosen convention:** `/blog/tag/slug` for tags, `/blog/category/slug` for categories.

### 7.2 Tag Slugs

- Lowercase, hyphen-separated: `rust`, `ci-cd`, `open-source`
- No special characters, no spaces
- Match the display name as closely as possible
- Singular preferred: `tutorial` not `tutorials`

### 7.3 Index Pages

```
/blog/tags          → all tags with counts
/blog/categories    → all categories with descriptions
```

---

## 8. Tag Management

### 8.1 Preventing Tag Proliferation

Tag proliferation is the most common taxonomy failure. Symptoms: 50+ tags, most with 1-2 posts, tag pages that are thin content.

**Prevention rules:**

1. **Minimum post threshold.** A tag page should only be generated when 3+ posts use that tag. Below that, the tag appears on post pages but does not get its own listing page.
2. **Tag registry.** Maintain a canonical list of allowed tags. New tags require review.
3. **Annual audit.** Review all tags yearly. Merge, rename, or retire low-use tags.
4. **Cap per post.** Enforce 3-7 tags per post. More than 7 dilutes each tag's signal.
5. **No overlapping tags.** Do not have both "JavaScript" and "JS". Pick one canonical form.

### 8.2 Tag Merging

When two tags mean the same thing:

```typescript
// Tag alias configuration
const tagAliases: Record<string, string> = {
  'js': 'javascript',
  'ts': 'typescript',
  'ci-cd': 'ci/cd',        // normalized form
  'continuous-integration': 'ci/cd',
  'react-js': 'react',
};

function normalizeTag(tag: string): string {
  const lower = tag.toLowerCase().trim();
  return tagAliases[lower] || lower;
}
```

### 8.3 Tag Aliases and Synonyms

```yaml
# tags.yaml — tag registry with metadata
tags:
  rust:
    display: "Rust"
    slug: "rust"
    description: "Posts about the Rust programming language"
    aliases: ["rustlang", "rust-lang"]
    color: "orange"

  convergence:
    display: "Convergence"
    slug: "convergence"
    description: "Autonomous convergence patterns and engine design"
    aliases: ["convergence-engine", "auto-convergence"]
    color: "purple"

  testing:
    display: "Testing"
    slug: "testing"
    description: "Test-driven development, coverage, and testing strategies"
    aliases: ["tdd", "tests", "unit-testing"]
    color: "green"
```

When a post uses an alias (e.g., "TDD"), it resolves to the canonical tag ("testing"). The alias is not exposed in URLs or tag pages.

---

## 9. SEO for Tag and Category Pages

### 9.1 Canonical Tags

Every tag/category page gets a self-referencing canonical:

```html
<link rel="canonical" href="https://cruxdev.com/blog/tag/rust" />
```

### 9.2 Noindex for Low-Count Tags

Tags with fewer than 3 posts create thin content pages that can dilute SEO.

```html
<!-- Tag page with < 3 posts -->
<meta name="robots" content="noindex, follow" />
```

`follow` ensures links on the page are still crawled. `noindex` prevents the thin page from appearing in search results.

### 9.3 Meta Descriptions for Tag Pages

Auto-generate meaningful descriptions:

```typescript
function tagMetaDescription(tag: TagMetadata, count: number): string {
  if (tag.description) return tag.description;
  return `Browse ${count} posts about ${tag.display} on the CruxDev blog. ${tag.description || ''}`.trim();
}
```

**Do not use:** "Posts tagged rust" as a meta description. It is not useful to searchers.

### 9.4 Schema.org for Collection Pages

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "CollectionPage",
  "name": "Posts tagged 'Rust'",
  "description": "Posts about the Rust programming language on the CruxDev blog",
  "url": "https://cruxdev.com/blog/tag/rust",
  "mainEntity": {
    "@type": "ItemList",
    "numberOfItems": 12,
    "itemListElement": [
      {
        "@type": "ListItem",
        "position": 1,
        "url": "https://cruxdev.com/blog/autonomous-convergence"
      }
    ]
  }
}
</script>
```

### 9.5 Sitemap Inclusion

Include tag and category pages in your sitemap, but only pages that are indexed (3+ posts):

```xml
<url>
  <loc>https://cruxdev.com/blog/tag/rust</loc>
  <lastmod>2026-03-15</lastmod>
  <changefreq>weekly</changefreq>
  <priority>0.6</priority>
</url>
```

---

## 10. Cross-Referencing

### 10.1 Tags in Post Listings

Every post card in a listing should show its tags (2-3 max in compact view):

```html
<div class="post-tags">
  <a href="/blog/tag/rust">rust</a>
  <a href="/blog/tag/performance">performance</a>
  <span class="more-tags" title="convergence, testing">+2</span>
</div>
```

### 10.2 Tags in Post Pages

Show all tags on the full post page (no truncation):

```html
<footer class="post-footer">
  <div class="post-tags" aria-label="Post tags">
    <svg aria-hidden="true" class="tag-icon"><!-- tag icon --></svg>
    <a href="/blog/tag/rust" class="tag-pill">rust</a>
    <a href="/blog/tag/performance" class="tag-pill">performance</a>
    <a href="/blog/tag/convergence" class="tag-pill">convergence</a>
    <a href="/blog/tag/testing" class="tag-pill">testing</a>
  </div>
</footer>
```

### 10.3 Tag-Based Related Posts

See BLOG_PATTERNS.md section 2.4 for the full algorithm. Tags are the primary signal for related post selection, weighted by specificity (rare tags create stronger connections than common tags).

```typescript
function tagSpecificity(tag: string, totalPosts: number, tagCount: number): number {
  // Inverse document frequency: rare tags are more specific
  return Math.log(totalPosts / tagCount);
}

function relatedByTags(current: Post, allPosts: Post[]): ScoredPost[] {
  const totalPosts = allPosts.length;
  const tagCounts = countTagOccurrences(allPosts);

  return allPosts
    .filter(p => p.slug !== current.slug)
    .map(post => {
      const sharedTags = current.tags.filter(t => post.tags.includes(t));
      const score = sharedTags.reduce(
        (sum, tag) => sum + tagSpecificity(tag, totalPosts, tagCounts[tag]),
        0
      );
      return { post, score };
    })
    .filter(p => p.score > 0)
    .sort((a, b) => b.score - a.score);
}
```

---

## 11. Faceted Navigation

### 11.1 Combining Filters

```
/blog?category=engineering&tag=rust&tag=performance
```

**Logic:**
- Category filter: AND (only one category at a time)
- Tag filters: OR within tags (posts with rust OR performance)
- Category + Tags: AND (engineering posts that have rust OR performance)

### 11.2 Faceted Navigation UI

```html
<aside class="faceted-nav" aria-label="Filter posts">
  <div class="facet-group">
    <h3>Category</h3>
    <ul>
      <li>
        <a href="/blog?category=engineering"
           class="facet-link active"
           aria-current="true">
          Engineering <span class="facet-count">(24)</span>
        </a>
      </li>
      <li>
        <a href="/blog?category=product" class="facet-link">
          Product <span class="facet-count">(18)</span>
        </a>
      </li>
    </ul>
  </div>

  <div class="facet-group">
    <h3>Tags</h3>
    <ul>
      <li>
        <label class="facet-checkbox">
          <input type="checkbox" name="tag" value="rust" checked />
          rust <span class="facet-count">(12)</span>
        </label>
      </li>
      <li>
        <label class="facet-checkbox">
          <input type="checkbox" name="tag" value="performance" />
          performance <span class="facet-count">(8)</span>
        </label>
      </li>
    </ul>
  </div>
</aside>
```

**Principle:** Facet counts must update to reflect the current filter state. If "Engineering" is selected, tag counts should show only engineering posts, not all posts.

### 11.3 Responsive Faceted Navigation

- **Desktop:** Sidebar with always-visible facets
- **Tablet:** Collapsible sidebar or horizontal filter bar
- **Mobile:** Full-screen filter drawer triggered by a "Filter" button, with "Apply" and "Clear" actions

```html
<!-- Mobile filter trigger -->
<button class="filter-trigger" aria-expanded="false"
        aria-controls="filter-drawer">
  <svg aria-hidden="true"><!-- filter icon --></svg>
  Filters
  <span class="active-count" aria-label="2 active filters">2</span>
</button>

<dialog id="filter-drawer" class="filter-drawer">
  <header>
    <h2>Filter Posts</h2>
    <button aria-label="Close filters">
      <svg aria-hidden="true"><!-- close icon --></svg>
    </button>
  </header>
  <!-- Facet groups -->
  <footer>
    <button class="btn-secondary" onclick="clearAllFilters()">Clear All</button>
    <button class="btn-primary" onclick="applyFilters()">
      Show Results (12)
    </button>
  </footer>
</dialog>
```

---

## 12. Accessibility

### 12.1 Tag Links

- All tag links must have sufficient contrast (4.5:1 minimum)
- Tag pills must have a minimum touch target of 44x44px on mobile (use padding)
- Color-coded tags must not rely on color alone (include text label always)

### 12.2 Tag Cloud

- Use `role="list"` and `role="listitem"` for tag clouds
- Include post count in accessible label: `aria-label="rust, 12 posts"`
- Weighted font sizes must stay within readable range (minimum 0.75rem / 12px)

### 12.3 Faceted Navigation

- Checkbox state changes must be announced by screen readers (native `<input type="checkbox">` handles this)
- Result count must update via `aria-live="polite"` region
- Filter drawer must trap focus when open (dialog element handles this natively)

### 12.4 Removable Filter Chips

- Each chip must have an accessible label: `aria-label="Remove rust filter"`
- Focus must move to the next chip (or the first remaining chip) when a chip is removed
- Announce removal: "rust filter removed. 1 filter active."

---

## 13. Anti-Patterns

| Anti-Pattern | Why It Is Wrong | Do Instead |
|---|---|---|
| 50+ tags with 1-2 posts each | Thin content pages hurt SEO | Limit to 20-30 tags, minimum 3 posts per tag page |
| Generic tags ("blog", "post", "update") | Zero navigation value, zero SEO value | Use specific, meaningful tags |
| Both "JavaScript" and "JS" as separate tags | Splits related content, confuses users | Canonical tag with aliases |
| Deeply nested categories (3+ levels) | Complex URLs, confusing navigation | Maximum 2 levels |
| Tags that duplicate categories | Redundant taxonomy, user confusion | Tags for specifics, categories for broad topics |
| Tag cloud as primary navigation | Poor mobile UX, hard to scan | Flat list as primary, cloud as supplementary |
| No tag descriptions | Missed SEO opportunity, thin meta descriptions | Write a description for each tag |
| Allowing arbitrary tag creation without review | Leads to proliferation and inconsistency | Tag registry with review process |
| Using tags only for display, not linking to tag pages | Wasted navigation opportunity | Every tag is a link to its listing page |
| Paginating tag pages at `/blog/tag/rust/page/2` but not canonicalizing properly | Duplicate content confusion | Self-referencing canonical on each page |

---

## 14. Audit Dimensions

1. **taxonomy-design** — categories are broad (5-10), tags are specific (20-30), no overlap
2. **tag-quality** — no generic tags, no duplicates, aliases resolve correctly
3. **tag-pages** — every tag with 3+ posts has a page; pages below threshold are noindexed
4. **category-pages** — every category has a page with description and proper SEO
5. **urls** — clean slugs, consistent convention (`/blog/tag/slug`), no broken links
6. **cross-referencing** — tags shown in listings and post pages, tag-based related posts work
7. **seo** — canonical tags, meta descriptions, schema.org CollectionPage, sitemap inclusion
8. **management** — tag registry exists, alias system works, proliferation controlled
9. **accessibility** — contrast ratios, touch targets, screen reader labels, focus management
10. **faceted-nav** — filters in URL, counts update, mobile drawer works, clear-all available
