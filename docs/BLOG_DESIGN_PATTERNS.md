# Blog Design Patterns — Premium Editorial Quality

**Research method:** Direct analysis of Stripe, Vercel, Linear, Tailwind CSS, Josh Comeau, Dan Abramov (overreacted.io), Substack, Ghost default themes, Medium
**Complements:** BLOG_POST_PATTERNS.md (content structure), BLOG_PATTERNS.md (architecture), COLOR_CONTRAST_PATTERNS.md (accessibility)
**Last updated:** 2026-03-28

---

## 1. Typography System

### 1.1 Font Stacks

Premium blogs converge on two patterns: a geometric sans-serif for headings paired with either itself or a serif for body text.

| Role | Tier 1 (Premium) | Tier 2 (Clean) | Tier 3 (System) |
|------|------------------|-----------------|------------------|
| **Headings** | Inter Variable | Geist (Vercel) | system-ui, -apple-system |
| **Body** | Source Serif 4, Lora | Inter Variable | Georgia, serif |
| **Code** | JetBrains Mono, IBM Plex Mono | Fira Code | ui-monospace, Menlo |

**What the reference sites use:**

| Site | Heading Font | Body Font | Mono Font |
|------|-------------|-----------|-----------|
| Stripe | Custom (Stripe) | System + custom | Menlo-based |
| Vercel | Geist Sans | Geist Sans | Geist Mono |
| Linear | Inter Variable | Inter Variable | Custom mono |
| Tailwind blog | Inter | Inter | IBM Plex Mono |
| Josh Comeau | Wotfard (custom) | System sans | Fira Code |
| overreacted.io | System serif | System serif | System mono |
| Substack | Charter (serif) | Charter (serif) | System mono |

**Recommendation for CruxDev:** Inter Variable for headings and body (developer audience, clean, free, variable font = single file). Source Serif 4 as optional serif alternative for long-form. JetBrains Mono for code.

### 1.2 Size Scale

Use a modular scale anchored at 18px body text. `clamp()` for fluid sizing.

| Element | Size | Weight | Line Height | Letter Spacing |
|---------|------|--------|-------------|----------------|
| Hero title (h1) | `clamp(2rem, 5vw, 3rem)` (32-48px) | 800 | 1.1 | -0.03em |
| Section heading (h2) | `clamp(1.5rem, 3vw, 1.875rem)` (24-30px) | 700 | 1.25 | -0.02em |
| Subsection (h3) | `clamp(1.25rem, 2.5vw, 1.5rem)` (20-24px) | 600 | 1.3 | -0.015em |
| Minor heading (h4) | `1.125rem` (18px) | 600 | 1.4 | -0.01em |
| Body text | `clamp(1.0625rem, 1.5vw, 1.125rem)` (17-18px) | 400 | 1.75 | 0 |
| Small text / captions | `0.875rem` (14px) | 400 | 1.5 | 0.01em |
| Overline / label | `0.75rem` (12px) | 600 (uppercase) | 1.5 | 0.08em |
| Code (block) | `0.875rem` (14px) | 400 | 1.7 | 0 |
| Code (inline) | `0.875em` | 400 | inherit | 0 |

**Key findings from research:**

- **Body at 17-18px, not 16px.** Every premium blog studied uses 17-18px. The web's 16px default is too small for sustained reading.
- **Line height 1.7-1.8 for body.** Stripe, Tailwind, and overreacted all use generous line height. 1.5 is too tight for long-form; 2.0 is too loose.
- **Tighter letter-spacing on headings.** All studied sites use negative tracking (-0.02em to -0.04em) on headings above 24px. This prevents large text from looking too airy.
- **Heavier headings, lighter body.** Hero titles use 700-800; body stays at 400. Never use 300 (too thin on lower-DPI screens). 500 for emphasis within body text (strong).

### 1.3 Maximum Line Width

| Source | Recommendation |
|--------|---------------|
| Robert Bringhurst (Elements of Typographic Style) | 45-75 characters, ideal 66 |
| Baymard Institute (research) | 50-75 characters |
| Stripe blog | ~720px container (~68ch at 18px) |
| Tailwind blog | ~768px container (~72ch at 18px) |
| overreacted.io | ~672px container (~63ch at 18px) |
| Josh Comeau | ~692px container (~65ch at 18px) |

**Recommendation:** `max-width: 42rem` (672px) or `max-width: 65ch` for the prose container. This is the sweet spot: wide enough for code blocks, narrow enough for comfortable scanning.

---

## 2. Color and Contrast

Refer to COLOR_CONTRAST_PATTERNS.md for the full token system. This section covers blog-specific color decisions.

### 2.1 Background Philosophy

| Pattern | Background | Used By | Feel |
|---------|-----------|---------|------|
| Off-white | `#FAFAFA` / `#F9FAFB` | Stripe, Medium | Clean, paper-like |
| Pure white | `#FFFFFF` | Vercel, Tailwind | Crisp, modern |
| Warm cream | `#FFFDF7` / `#FBF9F1` | Substack, some Ghost themes | Editorial, warm |
| Dark | `#08090A` - `#0F172A` | Linear (default dark) | Developer, premium |

**Recommendation:** `#FAFAFA` page background with `#FFFFFF` article surface (subtle card effect). Already defined in our token system. Avoids the "clinical" feel of pure white and the "old paper" feel of cream.

### 2.2 Text Colors for Reading

**Never use pure black (#000000) on white.** The 21:1 contrast ratio causes halation — thin fonts appear to vibrate. Every premium blog uses a softened black.

| Site | Body Text Color | Approximate |
|------|----------------|-------------|
| Stripe | `#425466` (blue-gray) | Tailwind slate-600 |
| Vercel | `#171717` | Tailwind neutral-900 |
| Tailwind blog | `#0f172a` | Tailwind slate-950 |
| overreacted.io | `#1a1a2e` | Custom dark indigo |
| Josh Comeau | `hsl(210deg 15% 20%)` | Dark blue-gray |
| Medium | `#242424` | Near-black neutral |

**Recommendation:** `#111827` (Tailwind gray-900) for primary body text. Provides 15.4:1 on `#FAFAFA` — AAA compliant while avoiding halation. Already in our token system.

### 2.3 Code Block Backgrounds

| Mode | Background | Used By |
|------|-----------|---------|
| Light mode, light code bg | `#F1F5F9` (slate-100) | Tailwind blog, docs |
| Light mode, dark code bg | `#0D1117` / `#1E1E3F` | GitHub, Shiki night themes |
| Dark mode | `#0D1117` / `#111827` | Universal |

**Recommendation:** Dark code blocks in both modes. Reason: syntax highlighting has better contrast and visual distinctiveness against dark backgrounds. Use `#0D1117` (GitHub dark) for code blocks. Code blocks "pop" from the page regardless of surrounding theme.

### 2.4 Callout/Highlight Box Colors

Use the semantic color palette from COLOR_CONTRAST_PATTERNS.md with very low opacity backgrounds:

| Type | Light BG | Light Border | Dark BG | Dark Border | Icon |
|------|----------|-------------|---------|-------------|------|
| Note | `#EFF6FF` (blue-50) | `#3B82F6` (blue-500) | `#1E3A5F` | `#60A5FA` | info circle |
| Tip | `#F0FDF4` (green-50) | `#22C55E` (green-500) | `#14532D` | `#4ADE80` | lightbulb |
| Warning | `#FFFBEB` (amber-50) | `#F59E0B` (amber-500) | `#451A03` | `#FBBF24` | triangle |
| Danger | `#FEF2F2` (red-50) | `#EF4444` (red-500) | `#450A0A` | `#F87171` | exclamation |

---

## 3. Spacing Rhythm

### 3.1 Vertical Rhythm System

Premium blogs use consistent spacing multiples. The base unit is 4px (0.25rem), with a primary rhythm of 8px steps.

| Spacing | Value | Use |
|---------|-------|-----|
| `xs` | `0.5rem` (8px) | Inline gaps, icon-to-text |
| `sm` | `0.75rem` (12px) | List item gaps, tight groups |
| `md` | `1rem` (16px) | Paragraph-to-paragraph within a section |
| `lg` | `1.5rem` (24px) | Between block elements (code, images, callouts) |
| `xl` | `2rem` (32px) | Between sections (below headings) |
| `2xl` | `3rem` (48px) | Above h2 headings (section breaks) |
| `3xl` | `4rem` (64px) | Hero spacing, major dividers |
| `4xl` | `6rem` (96px) | Page-level spacing (above/below article) |

### 3.2 Heading Spacing Rule

**More space above than below.** This is the single most important spacing rule for editorial quality. It creates visual grouping — the heading "belongs to" the content below it, not the content above.

```
Content of previous section ends here.
                                          ← 3rem (48px) above h2
## New Section Heading
                                          ← 0.75rem (12px) below h2
Content of new section starts here.
```

| Heading | Space Above | Space Below | Ratio |
|---------|------------|-------------|-------|
| h2 | `2.5em` (~45px) | `0.75em` (~13px) | 3.3:1 |
| h3 | `2em` (~36px) | `0.5em` (~9px) | 4:1 |
| h4 | `1.5em` (~27px) | `0.5em` (~9px) | 3:1 |

All premium blogs studied follow this pattern. Stripe uses approximately 3:1, Tailwind uses 3-4:1, Josh Comeau uses approximately 3:1.

### 3.3 Paragraph Spacing

Two approaches observed:

| Pattern | Spacing | Used By |
|---------|---------|---------|
| **Margin-based** | `margin-top: 1.5em` between paragraphs | Stripe, Tailwind, overreacted |
| **First-line indent** | No margin, 1.5em text-indent | Traditional print, Substack (some) |

**Recommendation:** Margin-based. Web convention. `> * + *` lobotomized owl selector with `margin-top: 1.5em`.

### 3.4 Block Element Spacing

Code blocks, images, callouts, and other block-level elements need more breathing room than paragraphs:

```
Previous paragraph text.
                          ← 2em (36px)
┌─────────────────────────┐
│  Code block / Callout   │
└─────────────────────────┘
                          ← 2em (36px)
Following paragraph text.
```

### 3.5 Section Dividers (Horizontal Rules)

| Pattern | Style | Used By |
|---------|-------|---------|
| Thin line | 1px solid, muted color | Stripe, Tailwind |
| Centered dots | `* * *` or `...` | overreacted, Substack |
| Gradient fade | Color fades to transparent at edges | Linear |
| Extra spacing only | No visible line, just 4rem gap | Josh Comeau |

**Recommendation:** Thin 1px line in border color with generous padding (3rem above, 3rem below). The line acts as a breathing pause.

---

## 4. Special Elements

### 4.1 Callout / Admonition Boxes

Every premium developer blog has these. Pattern from GitHub docs, Stripe, Tailwind:

```
┌─ Note ──────────────────────────────┐
│ (i) This is important context the   │
│     reader should know.             │
└─────────────────────────────────────┘
```

**Design rules:**
- Left border accent (3-4px), matching semantic color
- Subtle background fill (5-8% opacity of the accent color)
- Icon + label on top or inline-start
- Same font as body, optionally slightly smaller (0.9375em / 15px)
- Rounded corners (0.5rem)
- No border on top/right/bottom — left border only (cleaner than full border)

### 4.2 Pull Quotes

Large, emphasized quotes from the article text. Used to break up long sections and highlight key insights.

```css
.pull-quote {
  font-size: 1.5em;       /* 27px */
  font-weight: 500;
  line-height: 1.4;
  color: var(--color-text-secondary);
  border-left: 4px solid var(--color-accent);
  padding: 0.5em 0 0.5em 1.5em;
  margin: 2em 0;
}
```

**Pattern from reference sites:**
- Stripe: Centered, larger font, no border, italic
- Josh Comeau: Left-bordered, accent color, slightly indented
- Substack: Centered, serif font, decorative quotes

### 4.3 Code Blocks

The most scrutinized element on developer blogs. Premium patterns:

**Required features:**
1. Syntax highlighting (Shiki or Prism, Shiki preferred — used by Astro natively)
2. Language label (top-right or top-left corner)
3. Filename/path header when relevant (e.g., `src/config.ts`)
4. Copy button (top-right, appears on hover)
5. Line highlighting (accent background on specific lines)
6. Dark background regardless of page theme

**Optional features:**
- Line numbers (useful for tutorials, distracting for short snippets)
- Diff highlighting (+/- lines with green/red backgrounds)
- Word-level highlighting
- Terminal prompt styling ($ prefix, non-selectable)

**Layout:**
- Slight negative margin on mobile to use full viewport width (bleed effect)
- Rounded corners (0.5-0.75rem) on desktop
- Horizontal scroll, never wrap
- Bottom-right: language badge or line count

### 4.4 Styled Horizontal Rules

```css
.prose hr {
  border: none;
  height: 1px;
  background: var(--color-border);
  margin: 3rem 0;
}

/* Alternative: centered asterisks */
.prose hr.decorative {
  background: none;
  text-align: center;
  height: auto;
}
.prose hr.decorative::after {
  content: '***';
  letter-spacing: 1em;
  color: var(--color-text-tertiary);
}
```

### 4.5 Author Card

Appears at the bottom of every post (some sites also show at top).

```
┌──────────────────────────────────────┐
│  [Photo]  Bryan                      │
│           Founder, Trinsik Labs      │
│           Building autonomous dev    │
│           tools. @handle             │
└──────────────────────────────────────┘
```

**Design rules:**
- 48-64px circular avatar
- Name in semibold (600)
- Role/bio in secondary text color
- Social link(s) with subtle hover
- Subtle top border or background to separate from content

### 4.6 Reading Progress Bar

Already documented in BLOG_POST_PATTERNS.md section 9. Key addition: use `accent` color, 3px height, fixed to viewport top.

### 4.7 Table of Contents

Two patterns:

| Pattern | When | How |
|---------|------|-----|
| **Floating sidebar** | Desktop, long posts (2000+ words) | Sticky position in right column, highlights current section |
| **Inline collapsible** | Mobile, or all posts | `<details>` element at top of article |

**Sidebar TOC rules:**
- Only show on viewports >= 1280px
- Sticky at `top: 5rem` (below fixed nav)
- Active section highlighted with accent color and font-weight 500
- Smooth scroll with `scroll-behavior: smooth` and offset for fixed header
- Uses `IntersectionObserver` for active section tracking

### 4.8 Key Takeaway Box

A summary box placed at the end of a section or article:

```
╔══════════════════════════════════════╗
║  KEY TAKEAWAY                        ║
║                                      ║
║  The most important insight from     ║
║  this section in 1-2 sentences.      ║
╚══════════════════════════════════════╝
```

**Design:** Full border (not just left), subtle accent background, bold label, slightly smaller body text.

### 4.9 Numbered Step Blocks

For tutorials and how-to posts:

```
  ①  Step Title
     Description text for this step.
     Code block if needed.

  ②  Step Title
     Description text for this step.
```

**Design:** Large circled number (accent color), step title in semibold, content indented to align with title.

---

## 5. Layout

### 5.1 Content Width

| Zone | Width | Purpose |
|------|-------|---------|
| **Prose container** | `max-width: 42rem` (672px) or `65ch` | Body text, headings |
| **Wide container** | `max-width: 56rem` (896px) | Code blocks, images, tables |
| **Full bleed** | `100vw` | Hero images, full-width banners |
| **Page container** | `max-width: 72rem` (1152px) | Overall page (with sidebar) |

The "breakout" pattern: prose is narrow for readability, but code blocks and images can extend wider. This is the pattern used by Stripe, Josh Comeau, and CSS-Tricks.

```css
.article-content {
  --prose-width: 42rem;
  --wide-width: 56rem;
  display: grid;
  grid-template-columns:
    1fr
    min(var(--prose-width), 100%)
    1fr;
}

.article-content > * {
  grid-column: 2;
}

.article-content > .wide {
  grid-column: 1 / -1;
  max-width: var(--wide-width);
  margin-inline: auto;
  width: 100%;
  padding-inline: 1rem;
}

.article-content > .full-bleed {
  grid-column: 1 / -1;
  width: 100%;
}
```

### 5.2 Sidebar vs No Sidebar

| Pattern | Best For | Sites Using |
|---------|----------|-------------|
| No sidebar | Short-medium posts, clean aesthetic | Stripe, overreacted, Substack |
| Right sidebar (TOC) | Long technical posts, documentation | Tailwind docs, MDN |
| Right sidebar (TOC + author) | Long editorial + branding | Josh Comeau, dev.to |

**Recommendation:** No sidebar by default. Add floating TOC for posts over 2000 words, activated by a frontmatter flag (`toc: true`).

### 5.3 Responsive Breakpoints

| Breakpoint | Width | Layout Changes |
|-----------|-------|----------------|
| Mobile | < 640px | Single column, code blocks full-bleed, larger tap targets, stacked metadata |
| Tablet | 640-1023px | Single column, code blocks slightly wider than prose, inline TOC |
| Desktop | 1024-1279px | Single column, code blocks break out, optional sidebar |
| Wide | >= 1280px | Sidebar visible, full breakout widths, comfortable margins |

### 5.4 Mobile-Specific Considerations

- Code blocks: remove border-radius, go edge-to-edge (negative margin)
- Images: full width, no rounded corners
- Font size: keep 17-18px (do NOT reduce for mobile — research shows mobile readers hold screens closer)
- Heading sizes: reduce via `clamp()` but not below 1.5rem for h1
- TOC: collapsible at top, not sidebar
- Share buttons: bottom of article (not floating)

---

## 6. Interactive Elements

### 6.1 Smooth Scroll to Headings

```css
html {
  scroll-behavior: smooth;
  scroll-padding-top: 5rem; /* Account for fixed navbar */
}
```

Add anchor links to headings. Show on hover, not by default:

```css
.heading-anchor {
  opacity: 0;
  transition: opacity 0.15s;
  margin-left: 0.5em;
  color: var(--color-text-tertiary);
}
h2:hover .heading-anchor,
h3:hover .heading-anchor {
  opacity: 1;
}
```

### 6.2 Copy Button on Code Blocks

Appears on hover in top-right corner. Shows "Copied!" feedback for 2 seconds.

```javascript
function initCopyButtons() {
  document.querySelectorAll('pre').forEach(pre => {
    const button = document.createElement('button');
    button.className = 'copy-button';
    button.textContent = 'Copy';
    button.setAttribute('aria-label', 'Copy code to clipboard');
    pre.style.position = 'relative';
    pre.appendChild(button);

    button.addEventListener('click', async () => {
      const code = pre.querySelector('code')?.textContent ?? '';
      await navigator.clipboard.writeText(code);
      button.textContent = 'Copied!';
      setTimeout(() => { button.textContent = 'Copy'; }, 2000);
    });
  });
}
```

### 6.3 Expandable Sections

Use native `<details>/<summary>` with styling:

```css
details {
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 0.75rem 1rem;
  margin: 1.5em 0;
}

summary {
  font-weight: 600;
  cursor: pointer;
  user-select: none;
}

details[open] summary {
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border);
}
```

### 6.4 Dark/Light Mode Toggle

Use `class="dark"` on `<html>` (Tailwind convention). Persist preference in `localStorage`. Respect `prefers-color-scheme` as default.

```javascript
function initThemeToggle() {
  const stored = localStorage.getItem('theme');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const theme = stored ?? (prefersDark ? 'dark' : 'light');
  document.documentElement.classList.toggle('dark', theme === 'dark');
}
```

### 6.5 Share Buttons

Position: bottom of article, after author card. Never floating/sticky — too distracting for editorial content.

Minimal set: Copy link, X/Twitter, LinkedIn. No Facebook, no Pinterest (developer audience).

---

## 7. What Makes Premium Blogs "Feel" Premium

After studying all reference sites, the differentiators that separate premium from basic are:

### 7.1 The Non-Obvious Details

1. **Optical alignment.** Headings with tight letter-spacing appear to start slightly before the body text margin. Compensate with `-0.04em` at large sizes.
2. **Consistent vertical rhythm.** Every spacing value is a multiple of 4px. Nothing is arbitrary.
3. **Restraint with color.** Premium blogs use accent color in exactly two places: links and interactive elements. Everything else is gray scale.
4. **Typography does the heavy lifting.** Size, weight, and spacing create hierarchy — not color, not decoration.
5. **Generous whitespace.** When in doubt, add more space. Premium = breathing room.
6. **Subtle transitions.** Hover states, focus rings, and interactive elements transition over 150ms, never instant.
7. **System font fallbacks that match.** If Inter is loading, the fallback system font should have similar metrics to prevent layout shift.

### 7.2 The Anti-Patterns (What Makes Blogs Look Cheap)

| Cheap Signal | Premium Alternative |
|-------------|-------------------|
| Body text at 16px | 17-18px |
| Line height 1.4-1.5 | 1.7-1.8 |
| Pure black text on white | Softened black (#111827) on off-white (#FAFAFA) |
| Equal spacing above and below headings | 3:1 ratio (more above) |
| Headings same weight as body | Clear weight progression (800 -> 700 -> 600 -> 400) |
| Rainbow of accent colors | Single accent + neutrals |
| Borders everywhere | Spacing and weight for separation |
| Small, tight code blocks | Generous padding, dark background, breakout width |
| Decorative elements (gradients, shadows) | Typography and spacing |
| Font size changes on mobile | Same base size, `clamp()` for headings only |

---

## 8. Comprehensive Reference Summary

### Quick-Reference Card

```
FONTS:     Inter Variable (body+headings) + JetBrains Mono (code)
BODY:      17-18px, weight 400, line-height 1.75, color #111827
H1:        32-48px, weight 800, line-height 1.1, tracking -0.03em
H2:        24-30px, weight 700, line-height 1.25, tracking -0.02em
H3:        20-24px, weight 600, line-height 1.3, tracking -0.015em
WIDTH:     42rem prose / 56rem wide / 72rem page
BG:        #FAFAFA page / #FFFFFF surface / #0D1117 code
SPACING:   heading above:below = 3:1, paragraph gap 1.5em, section gap 3rem
COLORS:    One accent (#2563EB light / #60A5FA dark) + gray scale
DARK BG:   #0F172A page / #1E293B surface / #0D1117 code
```

---

## References

- Bringhurst, Robert. *The Elements of Typographic Style.* Optimal line length 45-75ch.
- Baymard Institute. "Readability: the Optimal Line Length." 50-75ch for web.
- WCAG 2.2. W3C. Contrast requirements SC 1.4.3, 1.4.6, 1.4.11.
- Stripe Engineering Blog. stripe.com/blog — typography, layout, spacing patterns.
- Vercel Blog. vercel.com/blog — Geist font, minimal design.
- Linear Blog. linear.app/blog — Inter Variable, dark-first, card grid.
- Tailwind CSS Blog. tailwindcss.com/blog — Inter + IBM Plex Mono, prose system.
- Josh Comeau Blog. joshwcomeau.com — Interactive elements, custom callouts, dark code.
- Dan Abramov / overreacted.io — Minimal serif, constrained width, generous spacing.
- Substack default styles — Charter serif, editorial warmth.
- Ghost default themes (Casper, Edition) — Balanced editorial design.
