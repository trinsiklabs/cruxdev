# Mobile Web & Webapp Design Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** W3C WCAG 2.2, Apple HIG, Material Design 3, Google Web Fundamentals, web.dev, Smashing Magazine, NNg, MDN Web Docs
**Last updated:** 2026-03-28

## Why This Matters

Mobile traffic accounts for 60%+ of global web traffic (Statcounter, 2025). Google uses mobile-first indexing exclusively. A one-second delay in mobile load time reduces conversions by 20% (Google). Yet most mobile experiences are afterthoughts — desktop layouts crammed into small screens. This document codifies the patterns that separate production-grade mobile web from "it technically works on a phone."

---

## 1. Mobile Navigation Patterns

Navigation is the single highest-impact mobile UX decision. Get it wrong and users cannot find anything. Get it right and it disappears — users just flow.

### 1.1 Hamburger Menu (Slide Drawer)

**When to use:** Apps with 5+ top-level sections, content-heavy sites, admin dashboards. The hamburger hides navigation behind a tap, maximizing content space.

**Trade-off:** Discoverability suffers. NNg research shows hamburger menus reduce feature discovery by ~21%. Use when content space matters more than navigation visibility.

```html
<!-- Hamburger Menu -->
<nav class="mobile-nav" aria-label="Main navigation">
  <button
    class="hamburger"
    aria-expanded="false"
    aria-controls="nav-drawer"
    aria-label="Open menu"
  >
    <span class="hamburger__line"></span>
    <span class="hamburger__line"></span>
    <span class="hamburger__line"></span>
  </button>

  <div id="nav-drawer" class="nav-drawer" role="dialog" aria-modal="true" aria-label="Navigation menu">
    <button class="nav-drawer__close" aria-label="Close menu">&times;</button>
    <ul class="nav-drawer__list">
      <li><a href="/" aria-current="page">Home</a></li>
      <li><a href="/products">Products</a></li>
      <li><a href="/about">About</a></li>
      <li><a href="/contact">Contact</a></li>
    </ul>
  </div>
  <div class="nav-overlay" aria-hidden="true"></div>
</nav>
```

```css
/* Slide drawer */
.nav-drawer {
  position: fixed;
  top: 0;
  left: -280px;
  width: 280px;
  height: 100%;
  background: #fff;
  z-index: 1000;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
}

.nav-drawer.is-open {
  transform: translateX(280px);
}

.nav-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.3s ease;
}

.nav-overlay.is-visible {
  opacity: 1;
  pointer-events: auto;
}

/* Hamburger icon */
.hamburger {
  display: flex;
  flex-direction: column;
  gap: 5px;
  padding: 12px;
  background: none;
  border: none;
  cursor: pointer;
  min-width: 44px;
  min-height: 44px;
  justify-content: center;
  align-items: center;
}

.hamburger__line {
  display: block;
  width: 24px;
  height: 2px;
  background: #333;
  transition: transform 0.3s ease;
}

/* Animate to X when open */
.hamburger[aria-expanded="true"] .hamburger__line:nth-child(1) {
  transform: translateY(7px) rotate(45deg);
}
.hamburger[aria-expanded="true"] .hamburger__line:nth-child(2) {
  opacity: 0;
}
.hamburger[aria-expanded="true"] .hamburger__line:nth-child(3) {
  transform: translateY(-7px) rotate(-45deg);
}

/* Nav items need adequate touch targets */
.nav-drawer__list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-drawer__list a {
  display: block;
  padding: 16px 24px;
  font-size: 1rem;
  color: #333;
  text-decoration: none;
  min-height: 48px;
  display: flex;
  align-items: center;
}
```

```js
// Hamburger drawer controller
class MobileNav {
  constructor() {
    this.button = document.querySelector('.hamburger');
    this.drawer = document.querySelector('.nav-drawer');
    this.overlay = document.querySelector('.nav-overlay');
    this.closeBtn = document.querySelector('.nav-drawer__close');
    this.focusableEls = this.drawer.querySelectorAll(
      'a, button, [tabindex]:not([tabindex="-1"])'
    );

    this.button.addEventListener('click', () => this.open());
    this.closeBtn.addEventListener('click', () => this.close());
    this.overlay.addEventListener('click', () => this.close());
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isOpen()) this.close();
    });
  }

  isOpen() {
    return this.drawer.classList.contains('is-open');
  }

  open() {
    this.drawer.classList.add('is-open');
    this.overlay.classList.add('is-visible');
    this.button.setAttribute('aria-expanded', 'true');
    // Trap focus inside drawer
    this.closeBtn.focus();
    document.body.style.overflow = 'hidden';
  }

  close() {
    this.drawer.classList.remove('is-open');
    this.overlay.classList.remove('is-visible');
    this.button.setAttribute('aria-expanded', 'false');
    this.button.focus();
    document.body.style.overflow = '';
  }
}

document.addEventListener('DOMContentLoaded', () => new MobileNav());
```

### 1.2 Bottom Tab Bar

**When to use:** Apps with 3-5 primary destinations that users switch between frequently. This is the dominant pattern for native-feeling webapps. Thumb-friendly — sits in the natural reach zone.

**Rules:**
- Maximum 5 tabs (4 is ideal)
- Always show labels — icon-only tabs reduce usability by 75% (NNg)
- Active state must be visually distinct (color + weight, not just color)
- Fixed to viewport bottom, not document bottom

```html
<nav class="bottom-tabs" aria-label="Primary navigation">
  <a href="/" class="bottom-tabs__item is-active" aria-current="page">
    <svg class="bottom-tabs__icon" aria-hidden="true" width="24" height="24">
      <use href="#icon-home" />
    </svg>
    <span class="bottom-tabs__label">Home</span>
  </a>
  <a href="/search" class="bottom-tabs__item">
    <svg class="bottom-tabs__icon" aria-hidden="true" width="24" height="24">
      <use href="#icon-search" />
    </svg>
    <span class="bottom-tabs__label">Search</span>
  </a>
  <a href="/cart" class="bottom-tabs__item">
    <svg class="bottom-tabs__icon" aria-hidden="true" width="24" height="24">
      <use href="#icon-cart" />
    </svg>
    <span class="bottom-tabs__label">Cart</span>
    <span class="bottom-tabs__badge" aria-label="3 items">3</span>
  </a>
  <a href="/profile" class="bottom-tabs__item">
    <svg class="bottom-tabs__icon" aria-hidden="true" width="24" height="24">
      <use href="#icon-profile" />
    </svg>
    <span class="bottom-tabs__label">Profile</span>
  </a>
</nav>
```

```css
.bottom-tabs {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  background: #fff;
  border-top: 1px solid #e0e0e0;
  z-index: 100;
  /* Safe area for notched phones */
  padding-bottom: env(safe-area-inset-bottom);
}

.bottom-tabs__item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 8px 0;
  min-height: 56px;
  text-decoration: none;
  color: #666;
  font-size: 0.75rem;
  position: relative;
  /* Touch target: full width of tab, at least 48px tall */
  -webkit-tap-highlight-color: transparent;
}

.bottom-tabs__item.is-active {
  color: #1a73e8;
  font-weight: 600;
}

.bottom-tabs__icon {
  width: 24px;
  height: 24px;
  margin-bottom: 4px;
}

.bottom-tabs__badge {
  position: absolute;
  top: 4px;
  right: 50%;
  transform: translateX(18px);
  background: #e53935;
  color: #fff;
  font-size: 0.625rem;
  font-weight: 700;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 4px;
}

/* Ensure page content does not hide behind the tab bar */
body {
  padding-bottom: calc(56px + env(safe-area-inset-bottom));
}
```

### 1.3 Full-Screen Overlay Navigation

**When to use:** Marketing sites, portfolios, landing pages — where navigation is infrequent and visual impact matters. The overlay takes over the entire screen, providing a focused navigation moment.

```html
<button class="overlay-trigger" aria-expanded="false" aria-controls="overlay-nav">
  Menu
</button>

<div id="overlay-nav" class="overlay-nav" role="dialog" aria-modal="true" aria-label="Site navigation">
  <div class="overlay-nav__header">
    <button class="overlay-nav__close" aria-label="Close navigation">&times;</button>
  </div>
  <nav class="overlay-nav__links" aria-label="Main">
    <a href="/" class="overlay-nav__link">Home</a>
    <a href="/work" class="overlay-nav__link">Work</a>
    <a href="/about" class="overlay-nav__link">About</a>
    <a href="/contact" class="overlay-nav__link">Contact</a>
  </nav>
</div>
```

```css
.overlay-nav {
  position: fixed;
  inset: 0;
  background: #111;
  color: #fff;
  z-index: 2000;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.4s ease, visibility 0.4s ease;
}

.overlay-nav.is-open {
  opacity: 1;
  visibility: visible;
}

.overlay-nav__header {
  position: absolute;
  top: 0;
  right: 0;
  padding: 16px;
}

.overlay-nav__close {
  background: none;
  border: none;
  color: #fff;
  font-size: 2rem;
  min-width: 48px;
  min-height: 48px;
  cursor: pointer;
}

.overlay-nav__links {
  display: flex;
  flex-direction: column;
  gap: 24px;
  text-align: center;
}

.overlay-nav__link {
  font-size: 2rem;
  font-weight: 700;
  color: #fff;
  text-decoration: none;
  padding: 12px 24px;
  transition: opacity 0.2s;
}

.overlay-nav__link:hover,
.overlay-nav__link:focus-visible {
  opacity: 0.7;
}
```

### 1.4 Progressive Disclosure Navigation

**When to use:** Complex information architectures (e-commerce, documentation, enterprise apps). Reveal sub-navigation on demand rather than showing everything at once.

```html
<nav class="accordion-nav" aria-label="Main navigation">
  <ul class="accordion-nav__list">
    <li class="accordion-nav__item">
      <button
        class="accordion-nav__trigger"
        aria-expanded="false"
        aria-controls="subnav-products"
      >
        Products
        <svg class="accordion-nav__chevron" aria-hidden="true" width="16" height="16">
          <use href="#icon-chevron-down" />
        </svg>
      </button>
      <ul id="subnav-products" class="accordion-nav__subnav" hidden>
        <li><a href="/products/widgets">Widgets</a></li>
        <li><a href="/products/gadgets">Gadgets</a></li>
        <li><a href="/products/tools">Tools</a></li>
      </ul>
    </li>
    <li class="accordion-nav__item">
      <a href="/pricing" class="accordion-nav__link">Pricing</a>
    </li>
    <li class="accordion-nav__item">
      <a href="/docs" class="accordion-nav__link">Docs</a>
    </li>
  </ul>
</nav>
```

```css
.accordion-nav__trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 16px 24px;
  background: none;
  border: none;
  border-bottom: 1px solid #eee;
  font-size: 1rem;
  color: #333;
  cursor: pointer;
  min-height: 48px;
}

.accordion-nav__chevron {
  transition: transform 0.2s ease;
}

.accordion-nav__trigger[aria-expanded="true"] .accordion-nav__chevron {
  transform: rotate(180deg);
}

.accordion-nav__subnav {
  list-style: none;
  padding: 0;
  margin: 0;
  background: #f9f9f9;
}

.accordion-nav__subnav a {
  display: block;
  padding: 14px 24px 14px 40px;
  color: #555;
  text-decoration: none;
  min-height: 48px;
  display: flex;
  align-items: center;
}
```

```js
// Accordion navigation — toggle sub-menus
document.querySelectorAll('.accordion-nav__trigger').forEach((trigger) => {
  trigger.addEventListener('click', () => {
    const expanded = trigger.getAttribute('aria-expanded') === 'true';
    const subnav = document.getElementById(trigger.getAttribute('aria-controls'));

    trigger.setAttribute('aria-expanded', String(!expanded));
    subnav.hidden = expanded;
  });
});
```

### 1.5 Navigation Decision Matrix

| Pattern | Best for | Destinations | Discoverability | Content space |
|---------|----------|-------------|-----------------|---------------|
| Hamburger drawer | Content-heavy, 5+ sections | 5-15 | Low | Maximum |
| Bottom tab bar | Core app flows, frequent switching | 3-5 | High | Moderate |
| Full-screen overlay | Marketing, portfolios | 3-7 | Low | Maximum |
| Progressive disclosure | Deep IA, e-commerce | Unlimited | Medium | Maximum |

---

## 2. Touch Targets

The number one mobile usability failure is targets that are too small to tap accurately. Fat-finger errors cause rage taps, accidental navigation, and abandonment.

### 2.1 Size Standards

| Standard | Minimum size | Recommended | Source |
|----------|-------------|-------------|--------|
| WCAG 2.5.8 (AA) | 24x24 CSS px | — | W3C WAI, 2023 |
| WCAG 2.5.5 (AAA) | 44x44 CSS px | — | W3C WAI |
| Apple HIG | 44x44 pt | — | Apple, 2024 |
| Material Design 3 | 48x48 dp | — | Google, 2024 |

**Practical rule:** 44x44px minimum for all interactive elements. 48x48px for primary actions. The WCAG 2.5.8 24x24px minimum is a floor, not a target.

### 2.2 Spacing Between Targets

Adjacent touch targets must have at least 8px of non-interactive space between them. Without spacing, users hit the wrong target even when individual targets meet size requirements.

```css
/* Touch target sizing utility */
.touch-target {
  min-width: 44px;
  min-height: 44px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

/* Expand small icons into adequate touch targets using padding */
.icon-button {
  /* The icon is 24x24, but the touch target is 48x48 */
  width: 24px;
  height: 24px;
  padding: 12px;
  box-sizing: content-box;
  background: none;
  border: none;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

/* Expand touch target with pseudo-element (invisible hit area) */
.small-link {
  position: relative;
}

.small-link::after {
  content: '';
  position: absolute;
  inset: -12px;  /* Expands hit area by 12px in each direction */
  /* No background — invisible but tappable */
}

/* Spacing between adjacent targets in a toolbar */
.toolbar {
  display: flex;
  gap: 8px;  /* Minimum 8px between targets */
  align-items: center;
}

/* List items as touch targets */
.list-item {
  padding: 12px 16px;
  min-height: 48px;
  display: flex;
  align-items: center;
  /* Full-width tap target for list items */
}
```

### 2.3 Thumb Zone Design

Research (Steven Hoober, "How Do Users Really Hold Mobile Devices?") shows:
- **75% of all touch interactions** are single-thumb
- The natural reach zone is an arc from bottom-center
- Top corners are hardest to reach (especially top-left for right-handed users)

**Placement rules:**
- Primary actions go in the bottom half of the screen
- Destructive actions go in hard-to-reach areas (top corners)
- Bottom sheets > top modals for action menus
- FABs (floating action buttons) belong at bottom-right

```css
/* Bottom-anchored action bar — primary actions in thumb zone */
.action-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 12px 16px;
  padding-bottom: calc(12px + env(safe-area-inset-bottom));
  background: #fff;
  border-top: 1px solid #e0e0e0;
  display: flex;
  gap: 12px;
}

.action-bar__primary {
  flex: 1;
  min-height: 48px;
  border-radius: 8px;
  background: #1a73e8;
  color: #fff;
  border: none;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
}

/* FAB positioning */
.fab {
  position: fixed;
  bottom: calc(24px + env(safe-area-inset-bottom));
  right: 24px;
  width: 56px;
  height: 56px;
  border-radius: 16px;
  background: #1a73e8;
  color: #fff;
  border: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  z-index: 50;
}
```

---

## 3. Responsive Design

### 3.1 Mobile-First Approach

Write base styles for mobile, then layer on complexity for larger screens. This is not optional — it produces smaller CSS payloads (mobile devices get only what they need) and forces design discipline.

```css
/* Mobile-first: base styles are mobile */
.card-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
  padding: 16px;
}

/* Tablet: 2 columns */
@media (min-width: 768px) {
  .card-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 24px;
    padding: 24px;
  }
}

/* Desktop: 3 columns */
@media (min-width: 1024px) {
  .card-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 32px;
    max-width: 1200px;
    margin: 0 auto;
  }
}

/* Large desktop: 4 columns */
@media (min-width: 1280px) {
  .card-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}
```

### 3.2 Tailwind Breakpoints

Tailwind uses mobile-first breakpoints by default. Un-prefixed utilities apply at all sizes. Prefixed utilities apply at that breakpoint and up.

| Prefix | Min-width | Typical device |
|--------|-----------|----------------|
| (none) | 0px | Mobile (base) |
| `sm:` | 640px | Large phone / small tablet |
| `md:` | 768px | Tablet |
| `lg:` | 1024px | Laptop |
| `xl:` | 1280px | Desktop |
| `2xl:` | 1536px | Large desktop |

```html
<!-- Tailwind responsive grid -->
<div class="grid grid-cols-1 gap-4 p-4 sm:grid-cols-2 md:gap-6 lg:grid-cols-3 xl:grid-cols-4">
  <div class="rounded-lg border p-4">Card 1</div>
  <div class="rounded-lg border p-4">Card 2</div>
  <div class="rounded-lg border p-4">Card 3</div>
  <div class="rounded-lg border p-4">Card 4</div>
</div>

<!-- Responsive text sizing -->
<h1 class="text-2xl font-bold sm:text-3xl lg:text-4xl xl:text-5xl">
  Responsive Heading
</h1>

<!-- Hide/show elements by breakpoint -->
<nav class="hidden md:flex">Desktop nav</nav>
<button class="md:hidden">Mobile menu</button>

<!-- Responsive padding/margin -->
<section class="px-4 py-8 md:px-8 md:py-12 lg:px-16 lg:py-20">
  Content with responsive spacing
</section>
```

### 3.3 Container Queries

Container queries let components respond to their container's size, not the viewport. This is essential for reusable components that appear in different contexts (sidebar vs. main content).

```css
/* Define a containment context */
.card-container {
  container-type: inline-size;
  container-name: card;
}

/* Component adapts to its container, not viewport */
.product-card {
  display: flex;
  flex-direction: column;
}

@container card (min-width: 400px) {
  .product-card {
    flex-direction: row;
    gap: 16px;
  }

  .product-card__image {
    width: 40%;
    flex-shrink: 0;
  }
}

@container card (min-width: 600px) {
  .product-card {
    gap: 24px;
  }

  .product-card__title {
    font-size: 1.5rem;
  }
}
```

```html
<!-- Tailwind v3.4+ container query support -->
<div class="@container">
  <div class="flex flex-col @md:flex-row @md:gap-4">
    <img class="w-full @md:w-2/5" src="product.jpg" alt="Product" />
    <div class="p-4">
      <h3 class="text-lg @lg:text-xl">Product Name</h3>
    </div>
  </div>
</div>
```

### 3.4 Fluid Typography

Stop using fixed font sizes with breakpoint jumps. Use `clamp()` for smooth scaling between minimum and maximum sizes.

```css
/* clamp(minimum, preferred, maximum) */
/* preferred uses viewport-relative units for fluid scaling */

:root {
  /* Body text: 16px at 320px viewport, scales to 18px at 1200px */
  --text-body: clamp(1rem, 0.943rem + 0.28vw, 1.125rem);

  /* H1: 28px at 320px, scales to 48px at 1200px */
  --text-h1: clamp(1.75rem, 1.18rem + 2.27vw, 3rem);

  /* H2: 22px at 320px, scales to 36px at 1200px */
  --text-h2: clamp(1.375rem, 0.977rem + 1.59vw, 2.25rem);

  /* H3: 18px at 320px, scales to 24px at 1200px */
  --text-h3: clamp(1.125rem, 0.955rem + 0.68vw, 1.5rem);
}

body {
  font-size: var(--text-body);
  line-height: 1.6;
}

h1 { font-size: var(--text-h1); line-height: 1.2; }
h2 { font-size: var(--text-h2); line-height: 1.3; }
h3 { font-size: var(--text-h3); line-height: 1.4; }

/* Fluid spacing using clamp */
.section {
  padding: clamp(1.5rem, 1rem + 2.5vw, 4rem) clamp(1rem, 0.5rem + 2.5vw, 3rem);
}
```

**How to calculate `clamp()` values:**

The formula for the preferred value: `(minSize - slope * minViewport) / 16 + slope * 100vw`
where `slope = (maxSize - minSize) / (maxViewport - minViewport)`

Or use a tool: [Utopia Fluid Type Calculator](https://utopia.fyi/type/calculator/).

### 3.5 Responsive Images

```html
<!-- srcset with width descriptors — browser picks optimal size -->
<img
  src="hero-800.jpg"
  srcset="
    hero-400.jpg   400w,
    hero-800.jpg   800w,
    hero-1200.jpg 1200w,
    hero-1600.jpg 1600w
  "
  sizes="
    (max-width: 640px) 100vw,
    (max-width: 1024px) 50vw,
    33vw
  "
  alt="Hero image"
  width="1600"
  height="900"
  loading="lazy"
  decoding="async"
/>

<!-- Art direction with <picture> — different crops per breakpoint -->
<picture>
  <source
    media="(max-width: 639px)"
    srcset="hero-mobile.webp"
    type="image/webp"
  />
  <source
    media="(max-width: 639px)"
    srcset="hero-mobile.jpg"
  />
  <source
    media="(min-width: 640px)"
    srcset="hero-desktop.webp"
    type="image/webp"
  />
  <img
    src="hero-desktop.jpg"
    alt="Hero image"
    width="1600"
    height="900"
    loading="lazy"
    decoding="async"
  />
</picture>
```

```css
/* Responsive image defaults */
img {
  max-width: 100%;
  height: auto;
  display: block;
}

/* Aspect ratio containers — prevent layout shift */
.aspect-video {
  aspect-ratio: 16 / 9;
  overflow: hidden;
}

.aspect-video img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* Responsive background images */
.hero {
  background-image: url('hero-mobile.jpg');
  background-size: cover;
  background-position: center;
  min-height: 50vh;
}

@media (min-width: 768px) {
  .hero {
    background-image: url('hero-desktop.jpg');
    min-height: 70vh;
  }
}
```

---

## 4. Mobile Forms

Forms are where mobile UX either shines or collapses. Every extra tap, every wrong keyboard, every zoom-on-focus incident is friction that kills conversions.

### 4.1 Input Types for Correct Keyboard

The `type` and `inputmode` attributes control which virtual keyboard the device presents. Using the wrong one forces users to switch keyboard layouts manually.

| Data needed | `type` | `inputmode` | Keyboard shown |
|-------------|--------|-------------|----------------|
| Email | `email` | — | @ and . prominent |
| Phone | `tel` | — | Numeric dialer |
| URL | `url` | — | / and .com keys |
| Integer | `text` | `numeric` | Number pad |
| Decimal | `text` | `decimal` | Number pad + decimal |
| Search | `search` | — | Search/Go action key |
| Password | `password` | — | Standard + show/hide |
| Credit card | `text` | `numeric` | Number pad |
| ZIP/postal | `text` | `numeric` | Number pad |
| One-time code | `text` | `numeric` | Number pad |

**Why `inputmode="numeric"` instead of `type="number"`?** `type="number"` adds spinners, strips leading zeros (bad for ZIP codes), fires differently on change, and doesn't support `pattern`. Use `type="text"` + `inputmode="numeric"` for numeric data that isn't a "number" mathematically.

```html
<!-- Phone number -->
<label for="phone">Phone number</label>
<input
  type="tel"
  id="phone"
  name="phone"
  autocomplete="tel"
  placeholder="(555) 123-4567"
/>

<!-- Email -->
<label for="email">Email address</label>
<input
  type="email"
  id="email"
  name="email"
  autocomplete="email"
  inputmode="email"
/>

<!-- Credit card number -->
<label for="cc-number">Card number</label>
<input
  type="text"
  id="cc-number"
  name="cc-number"
  inputmode="numeric"
  pattern="[0-9\s]{13,19}"
  autocomplete="cc-number"
  maxlength="19"
/>

<!-- One-time code (SMS verification) -->
<label for="otp">Verification code</label>
<input
  type="text"
  id="otp"
  name="otp"
  inputmode="numeric"
  autocomplete="one-time-code"
  pattern="[0-9]{6}"
  maxlength="6"
/>

<!-- ZIP code (leading zeros matter) -->
<label for="zip">ZIP code</label>
<input
  type="text"
  id="zip"
  name="zip"
  inputmode="numeric"
  autocomplete="postal-code"
  pattern="[0-9]{5}"
  maxlength="5"
/>
```

### 4.2 Autocomplete Attributes

Autocomplete is the single biggest mobile form optimization. It lets browsers and password managers fill fields instantly. Omitting it forces manual entry on a tiny keyboard.

```html
<form>
  <!-- Name -->
  <input type="text" name="name" autocomplete="name" />
  <input type="text" name="given-name" autocomplete="given-name" />
  <input type="text" name="family-name" autocomplete="family-name" />

  <!-- Address -->
  <input type="text" name="address" autocomplete="street-address" />
  <input type="text" name="city" autocomplete="address-level2" />
  <input type="text" name="state" autocomplete="address-level1" />
  <input type="text" name="zip" autocomplete="postal-code" inputmode="numeric" />
  <select name="country" autocomplete="country">...</select>

  <!-- Payment -->
  <input type="text" name="cc-name" autocomplete="cc-name" />
  <input type="text" name="cc-number" autocomplete="cc-number" inputmode="numeric" />
  <input type="text" name="cc-exp" autocomplete="cc-exp" />
  <input type="text" name="cc-csc" autocomplete="cc-csc" inputmode="numeric" />

  <!-- Login -->
  <input type="email" name="email" autocomplete="username" />
  <input type="password" name="password" autocomplete="current-password" />
  <input type="password" name="new-password" autocomplete="new-password" />
</form>
```

### 4.3 Preventing iOS Zoom on Focus

Safari on iOS zooms in on any input with `font-size` below 16px when it receives focus. This is disorienting and users must manually zoom back out. The fix is simple — never use `font-size` below 16px on inputs.

```css
/* WRONG — triggers iOS zoom */
input, select, textarea {
  font-size: 14px;
}

/* CORRECT — prevents iOS zoom */
input, select, textarea {
  font-size: 16px;  /* Minimum to prevent zoom */
  /* Or use rem equivalent */
  font-size: 1rem;  /* Assuming root is 16px */
}

/* If your design requires smaller text, you can set a global
   minimum and override the visual appearance */
input {
  font-size: max(16px, 1em);
}

/* DO NOT use the meta viewport hack to "fix" this: */
/* <meta name="viewport" content="..., maximum-scale=1"> */
/* This breaks pinch-to-zoom accessibility. WCAG failure. */
```

### 4.4 Floating Labels vs. Stacked Labels

**Stacked labels (label above input) are the default recommendation.** They have the best usability scores across all research (Penzo eye-tracking, Baymard, NNg). They are always visible, have no animation concerns, and work perfectly with screen readers.

**Floating labels** (placeholder that animates up on focus) are popular in Material Design but have real problems:
- Shrunk label text can be too small for low-vision users
- Animation triggers motion sensitivity
- Initial state looks like a filled field (confusing)
- More complex to implement accessibly

If you must use floating labels:

```html
<div class="float-field">
  <input
    type="email"
    id="email"
    name="email"
    class="float-field__input"
    placeholder=" "
    required
  />
  <label for="email" class="float-field__label">Email address</label>
</div>
```

```css
.float-field {
  position: relative;
  margin-top: 16px;
}

.float-field__input {
  width: 100%;
  padding: 20px 16px 8px;
  font-size: 16px; /* Prevent iOS zoom */
  border: 1px solid #ccc;
  border-radius: 8px;
  outline: none;
  background: #fff;
  transition: border-color 0.2s;
}

.float-field__label {
  position: absolute;
  left: 16px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 16px;
  color: #666;
  pointer-events: none;
  transition: all 0.2s ease;
  /* The label must remain readable when floated */
}

/* Float the label when input has focus or content */
.float-field__input:focus + .float-field__label,
.float-field__input:not(:placeholder-shown) + .float-field__label {
  top: 8px;
  transform: translateY(0);
  font-size: 12px;
  color: #1a73e8;
  /* Ensure contrast: 12px text needs higher contrast ratio */
}

.float-field__input:focus {
  border-color: #1a73e8;
  box-shadow: 0 0 0 1px #1a73e8;
}

/* Respect reduced motion */
@media (prefers-reduced-motion: reduce) {
  .float-field__label,
  .float-field__input {
    transition: none;
  }
}
```

### 4.5 Mobile Form Layout Best Practices

```css
/* Full-width inputs on mobile — no reason for narrow inputs */
.form-field {
  margin-bottom: 20px;
}

.form-field label {
  display: block;
  margin-bottom: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  color: #333;
}

.form-field input,
.form-field select,
.form-field textarea {
  width: 100%;
  padding: 12px 16px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 8px;
  -webkit-appearance: none;
  appearance: none;
}

/* Larger checkbox/radio targets on mobile */
.form-check {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 0;
  min-height: 48px;
}

.form-check input[type="checkbox"],
.form-check input[type="radio"] {
  width: 24px;
  height: 24px;
  margin: 0;
  flex-shrink: 0;
  accent-color: #1a73e8;
}

/* Submit button — full width, prominent, in thumb zone */
.form-submit {
  width: 100%;
  padding: 16px;
  font-size: 1rem;
  font-weight: 600;
  color: #fff;
  background: #1a73e8;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  min-height: 48px;
  margin-top: 24px;
}
```

---

## 5. Mobile Performance

Mobile devices have less CPU, less memory, and slower networks than desktops. Performance is not a polish step — it is a design constraint that shapes every decision.

### 5.1 Critical Rendering Path

The browser must download HTML, discover CSS/JS, download those, parse them, build the DOM and CSSOM, and paint. Every blocking resource delays first paint.

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />

  <!-- Preconnect to critical origins -->
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://cdn.example.com" crossorigin />

  <!-- Preload critical resources -->
  <link rel="preload" href="/fonts/inter-var.woff2" as="font" type="font/woff2" crossorigin />
  <link rel="preload" href="/css/critical.css" as="style" />

  <!-- Critical CSS inlined for fastest first paint -->
  <style>
    /* Above-the-fold styles only — ~14KB max */
    *,*::before,*::after{box-sizing:border-box}
    body{margin:0;font-family:Inter,system-ui,sans-serif;line-height:1.6}
    .hero{min-height:50vh;display:flex;align-items:center;padding:2rem}
    /* ... minimal above-fold styles ... */
  </style>

  <!-- Non-critical CSS loaded asynchronously -->
  <link rel="preload" href="/css/main.css" as="style" onload="this.onload=null;this.rel='stylesheet'" />
  <noscript><link rel="stylesheet" href="/css/main.css" /></noscript>

  <!-- Defer all non-critical JS -->
  <script src="/js/app.js" defer></script>
</head>
<body>
  <!-- Above-the-fold content first in DOM -->
  <header>...</header>
  <section class="hero">...</section>

  <!-- Below-the-fold content -->
  <main>...</main>
</body>
</html>
```

### 5.2 Lazy Loading

Defer loading of off-screen images, iframes, and heavy components until they are needed.

```html
<!-- Native lazy loading — supported in all modern browsers -->
<img
  src="product.jpg"
  alt="Product photo"
  width="400"
  height="300"
  loading="lazy"
  decoding="async"
/>

<!-- Lazy load iframes (e.g., YouTube embeds) -->
<iframe
  src="https://www.youtube.com/embed/dQw4w9WgXcQ"
  title="Video title"
  width="560"
  height="315"
  loading="lazy"
  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope"
  allowfullscreen
></iframe>

<!-- Do NOT lazy load above-the-fold images -->
<img
  src="hero.jpg"
  alt="Hero"
  width="1200"
  height="600"
  loading="eager"
  fetchpriority="high"
  decoding="async"
/>
```

```js
// Intersection Observer for lazy-loading components
const lazyComponents = document.querySelectorAll('[data-lazy-component]');

const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const component = entry.target;
        const src = component.dataset.lazyComponent;

        // Dynamically import component
        import(src).then((module) => {
          module.init(component);
        });

        observer.unobserve(component);
      }
    });
  },
  { rootMargin: '200px' } // Start loading 200px before viewport
);

lazyComponents.forEach((el) => observer.observe(el));
```

### 5.3 Prefetching and Prerendering

```html
<!-- Prefetch resources likely needed on next navigation -->
<link rel="prefetch" href="/next-page.html" />
<link rel="prefetch" href="/api/products.json" />

<!-- DNS prefetch for third-party domains -->
<link rel="dns-prefetch" href="https://analytics.example.com" />

<!-- Speculation Rules API (Chrome 109+) — prerender next page -->
<script type="speculationrules">
{
  "prerender": [
    {
      "where": {
        "href_matches": "/products/*"
      },
      "eagerness": "moderate"
    }
  ],
  "prefetch": [
    {
      "where": {
        "selector_matches": "a[href]"
      },
      "eagerness": "conservative"
    }
  ]
}
</script>
```

```js
// Prefetch on hover/touch — simple and effective
document.querySelectorAll('a[href^="/"]').forEach((link) => {
  let prefetched = false;

  const prefetch = () => {
    if (prefetched) return;
    prefetched = true;

    const prefetchLink = document.createElement('link');
    prefetchLink.rel = 'prefetch';
    prefetchLink.href = link.href;
    document.head.appendChild(prefetchLink);
  };

  link.addEventListener('mouseenter', prefetch, { once: true });
  link.addEventListener('touchstart', prefetch, { once: true, passive: true });
});
```

### 5.4 Service Worker Caching

```js
// sw.js — Service worker with cache-first strategy for static assets
const CACHE_NAME = 'app-v1';
const STATIC_ASSETS = [
  '/',
  '/css/main.css',
  '/js/app.js',
  '/fonts/inter-var.woff2',
  '/offline.html',
];

// Install: pre-cache critical assets
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(STATIC_ASSETS))
  );
  self.skipWaiting();
});

// Activate: clean up old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(
        keys
          .filter((key) => key !== CACHE_NAME)
          .map((key) => caches.delete(key))
      )
    )
  );
  self.clients.claim();
});

// Fetch: cache-first for static, network-first for API
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  if (url.pathname.startsWith('/api/')) {
    // Network-first for API calls
    event.respondWith(
      fetch(request)
        .then((response) => {
          const clone = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(request, clone));
          return response;
        })
        .catch(() => caches.match(request))
    );
  } else {
    // Cache-first for static assets
    event.respondWith(
      caches.match(request).then((cached) => {
        if (cached) return cached;
        return fetch(request).then((response) => {
          const clone = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(request, clone));
          return response;
        });
      })
    );
  }
});
```

### 5.5 Core Web Vitals — Mobile Targets

| Metric | Good | Needs Improvement | Poor |
|--------|------|-------------------|------|
| LCP (Largest Contentful Paint) | <= 2.5s | 2.5s - 4.0s | > 4.0s |
| INP (Interaction to Next Paint) | <= 200ms | 200ms - 500ms | > 500ms |
| CLS (Cumulative Layout Shift) | <= 0.1 | 0.1 - 0.25 | > 0.25 |

**Mobile-specific LCP concerns:**
- Mobile LCP is typically 1.5-2x slower than desktop due to CPU and network constraints
- Hero images must use `fetchpriority="high"` and not `loading="lazy"`
- Web fonts should be preloaded, with `font-display: swap` or `font-display: optional`

**Preventing CLS on mobile:**
```css
/* Always set explicit dimensions on images and video */
img, video {
  max-width: 100%;
  height: auto;
  /* aspect-ratio prevents layout shift even before load */
  aspect-ratio: attr(width) / attr(height);
}

/* Reserve space for ads/embeds */
.ad-slot {
  min-height: 250px;
  background: #f5f5f5;
}

/* Prevent CLS from web fonts */
@font-face {
  font-family: 'Inter';
  src: url('/fonts/inter-var.woff2') format('woff2');
  font-display: swap; /* or 'optional' for less CLS */
  font-weight: 100 900;
}

/* Prevent CLS from dynamic content insertion */
.notification-bar {
  /* Use transform instead of changing height/margin */
  transform: translateY(-100%);
  transition: transform 0.3s ease;
}

.notification-bar.is-visible {
  transform: translateY(0);
}
```

**Improving INP on mobile:**
```js
// Break up long tasks to keep the main thread responsive
function processLargeDataset(items) {
  const CHUNK_SIZE = 50;
  let index = 0;

  function processChunk() {
    const end = Math.min(index + CHUNK_SIZE, items.length);
    for (; index < end; index++) {
      processItem(items[index]);
    }
    if (index < items.length) {
      // Yield to the browser between chunks
      setTimeout(processChunk, 0);
    }
  }

  processChunk();
}

// Use scheduler.yield() (Chrome 129+) for cooperative scheduling
async function handleClick() {
  // Do critical visual update first
  updateUI();

  // Yield to let the browser paint
  if ('scheduler' in globalThis && 'yield' in scheduler) {
    await scheduler.yield();
  }

  // Then do heavy work
  doExpensiveComputation();
}

// Debounce scroll/resize handlers
function debounce(fn, ms) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
}

window.addEventListener('resize', debounce(handleResize, 150));
```

---

## 6. PWA Patterns

Progressive Web Apps let mobile web apps behave like native apps — installable, offline-capable, with push notifications.

### 6.1 Web App Manifest

```json
{
  "name": "My App — Full Name",
  "short_name": "My App",
  "description": "A brief description of the app",
  "start_url": "/?source=pwa",
  "display": "standalone",
  "orientation": "portrait",
  "theme_color": "#1a73e8",
  "background_color": "#ffffff",
  "scope": "/",
  "icons": [
    {
      "src": "/icons/icon-192.png",
      "sizes": "192x192",
      "type": "image/png",
      "purpose": "any"
    },
    {
      "src": "/icons/icon-512.png",
      "sizes": "512x512",
      "type": "image/png",
      "purpose": "any"
    },
    {
      "src": "/icons/icon-maskable-512.png",
      "sizes": "512x512",
      "type": "image/png",
      "purpose": "maskable"
    }
  ],
  "screenshots": [
    {
      "src": "/screenshots/mobile.png",
      "sizes": "1080x1920",
      "type": "image/png",
      "form_factor": "narrow",
      "label": "Home screen"
    }
  ],
  "shortcuts": [
    {
      "name": "New Item",
      "short_name": "New",
      "url": "/new?source=shortcut",
      "icons": [{ "src": "/icons/shortcut-new.png", "sizes": "96x96" }]
    }
  ]
}
```

```html
<!-- Link manifest in <head> -->
<link rel="manifest" href="/manifest.json" />

<!-- iOS-specific meta tags (Safari doesn't fully support manifest) -->
<meta name="apple-mobile-web-app-capable" content="yes" />
<meta name="apple-mobile-web-app-status-bar-style" content="default" />
<meta name="apple-mobile-web-app-title" content="My App" />
<link rel="apple-touch-icon" href="/icons/apple-touch-icon-180.png" />

<!-- Theme color for browser chrome -->
<meta name="theme-color" content="#1a73e8" />
```

### 6.2 Service Worker Lifecycle

```js
// Register service worker
if ('serviceWorker' in navigator) {
  window.addEventListener('load', async () => {
    try {
      const reg = await navigator.serviceWorker.register('/sw.js', {
        scope: '/',
      });

      // Listen for updates
      reg.addEventListener('updatefound', () => {
        const newWorker = reg.installing;
        newWorker.addEventListener('statechange', () => {
          if (
            newWorker.state === 'activated' &&
            navigator.serviceWorker.controller
          ) {
            // New version available — prompt user to refresh
            showUpdateBanner();
          }
        });
      });
    } catch (err) {
      console.error('SW registration failed:', err);
    }
  });
}

function showUpdateBanner() {
  const banner = document.createElement('div');
  banner.className = 'update-banner';
  banner.innerHTML = `
    <p>A new version is available.</p>
    <button onclick="window.location.reload()">Refresh</button>
  `;
  document.body.appendChild(banner);
}
```

### 6.3 Install Prompt

```js
// Capture the beforeinstallprompt event
let deferredPrompt = null;

window.addEventListener('beforeinstallprompt', (e) => {
  // Prevent the default browser mini-infobar
  e.preventDefault();
  deferredPrompt = e;

  // Show your custom install button
  document.getElementById('install-button').hidden = false;
});

document.getElementById('install-button').addEventListener('click', async () => {
  if (!deferredPrompt) return;

  deferredPrompt.prompt();
  const result = await deferredPrompt.userChoice;

  if (result.outcome === 'accepted') {
    console.log('User accepted install');
  }

  deferredPrompt = null;
  document.getElementById('install-button').hidden = true;
});

// Detect if already installed
window.addEventListener('appinstalled', () => {
  deferredPrompt = null;
  document.getElementById('install-button').hidden = true;
  console.log('App installed');
});

// Check display mode to detect standalone
if (window.matchMedia('(display-mode: standalone)').matches) {
  console.log('Running as installed PWA');
}
```

### 6.4 Offline Fallback

```js
// In sw.js — serve offline page when network fails
self.addEventListener('fetch', (event) => {
  if (event.request.mode === 'navigate') {
    event.respondWith(
      fetch(event.request).catch(() => caches.match('/offline.html'))
    );
  }
});
```

```html
<!-- offline.html — minimal, self-contained (no external deps) -->
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Offline — My App</title>
  <style>
    body {
      font-family: system-ui, sans-serif;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      min-height: 100vh;
      margin: 0;
      padding: 24px;
      text-align: center;
      background: #f5f5f5;
      color: #333;
    }
    h1 { font-size: 1.5rem; margin-bottom: 8px; }
    p { color: #666; margin-bottom: 24px; }
    button {
      padding: 12px 24px;
      font-size: 1rem;
      background: #1a73e8;
      color: #fff;
      border: none;
      border-radius: 8px;
      cursor: pointer;
      min-height: 48px;
    }
  </style>
</head>
<body>
  <h1>You're offline</h1>
  <p>Check your connection and try again.</p>
  <button onclick="window.location.reload()">Retry</button>
</body>
</html>
```

### 6.5 Push Notifications

```js
// Request notification permission (only after user action)
async function requestNotificationPermission() {
  if (!('Notification' in window) || !('PushManager' in window)) {
    console.warn('Push notifications not supported');
    return false;
  }

  const permission = await Notification.requestPermission();
  if (permission !== 'granted') return false;

  const reg = await navigator.serviceWorker.ready;
  const subscription = await reg.pushManager.subscribe({
    userVisibleOnly: true,
    applicationServerKey: urlBase64ToUint8Array(VAPID_PUBLIC_KEY),
  });

  // Send subscription to your server
  await fetch('/api/push/subscribe', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(subscription),
  });

  return true;
}

function urlBase64ToUint8Array(base64String) {
  const padding = '='.repeat((4 - (base64String.length % 4)) % 4);
  const base64 = (base64String + padding).replace(/-/g, '+').replace(/_/g, '/');
  const raw = atob(base64);
  return Uint8Array.from([...raw].map((c) => c.charCodeAt(0)));
}
```

```js
// In sw.js — handle push events
self.addEventListener('push', (event) => {
  const data = event.data?.json() ?? {
    title: 'Notification',
    body: 'You have an update',
    url: '/',
  };

  event.waitUntil(
    self.registration.showNotification(data.title, {
      body: data.body,
      icon: '/icons/icon-192.png',
      badge: '/icons/badge-72.png',
      data: { url: data.url },
      actions: [
        { action: 'open', title: 'Open' },
        { action: 'dismiss', title: 'Dismiss' },
      ],
    })
  );
});

self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  if (event.action === 'dismiss') return;

  const url = event.notification.data?.url ?? '/';
  event.waitUntil(
    clients.matchAll({ type: 'window' }).then((windowClients) => {
      // Focus existing window if open
      for (const client of windowClients) {
        if (client.url === url && 'focus' in client) {
          return client.focus();
        }
      }
      // Otherwise open new window
      return clients.openWindow(url);
    })
  );
});
```

---

## 7. Mobile-Specific Accessibility

### 7.1 VoiceOver (iOS) and TalkBack (Android)

Screen readers on mobile work fundamentally differently than desktop. Users swipe to navigate between elements sequentially. Every interactive element must have a programmatic name.

```html
<!-- Every image needs alt text -->
<img src="chart.png" alt="Revenue grew 23% from Q1 to Q2 2025" />

<!-- Decorative images get empty alt -->
<img src="decorative-line.svg" alt="" role="presentation" />

<!-- Icon buttons MUST have labels -->
<button aria-label="Close dialog">
  <svg aria-hidden="true">...</svg>
</button>

<!-- Custom components need ARIA roles -->
<div
  role="tablist"
  aria-label="Product details"
>
  <button role="tab" aria-selected="true" aria-controls="panel-1" id="tab-1">
    Description
  </button>
  <button role="tab" aria-selected="false" aria-controls="panel-2" id="tab-2">
    Reviews
  </button>
</div>

<div role="tabpanel" id="panel-1" aria-labelledby="tab-1">
  Description content
</div>
<div role="tabpanel" id="panel-2" aria-labelledby="tab-2" hidden>
  Reviews content
</div>

<!-- Live regions for dynamic content -->
<div aria-live="polite" aria-atomic="true" class="sr-only" id="status">
  <!-- JS updates this when items are added to cart, forms submit, etc. -->
</div>

<!-- Skip repetitive content -->
<a href="#main-content" class="skip-link">Skip to content</a>
```

```css
/* Screen-reader-only class — visually hidden but announced */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Skip link — visible on focus */
.skip-link {
  position: absolute;
  top: -100%;
  left: 16px;
  background: #1a73e8;
  color: #fff;
  padding: 12px 24px;
  border-radius: 0 0 8px 8px;
  z-index: 10000;
  font-weight: 600;
  text-decoration: none;
}

.skip-link:focus {
  top: 0;
}
```

### 7.2 Reduced Motion

Some users experience motion sickness, seizures, or distraction from animations. The `prefers-reduced-motion` media query lets you respect their OS setting.

```css
/* Default: animations on */
.card {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

.slide-in {
  animation: slideIn 0.5s ease-out;
}

@keyframes slideIn {
  from { transform: translateX(100%); opacity: 0; }
  to { transform: translateX(0); opacity: 1; }
}

/* Reduced motion: disable or minimize animations */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }

  .card:hover {
    transform: none;
  }

  .slide-in {
    animation: none;
    /* Use opacity-only crossfade instead */
    opacity: 1;
  }
}
```

```js
// Check preference in JavaScript
const prefersReducedMotion = window.matchMedia(
  '(prefers-reduced-motion: reduce)'
).matches;

if (prefersReducedMotion) {
  // Skip animation, show final state
  element.style.opacity = '1';
} else {
  element.animate(
    [
      { opacity: 0, transform: 'translateY(20px)' },
      { opacity: 1, transform: 'translateY(0)' },
    ],
    { duration: 300, easing: 'ease-out', fill: 'forwards' }
  );
}
```

### 7.3 Dark Mode

```css
/* Light mode defaults */
:root {
  --color-bg: #ffffff;
  --color-surface: #f5f5f5;
  --color-text: #1a1a1a;
  --color-text-secondary: #666666;
  --color-border: #e0e0e0;
  --color-primary: #1a73e8;
  --color-primary-text: #ffffff;
}

/* Dark mode overrides */
@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #121212;
    --color-surface: #1e1e1e;
    --color-text: #e0e0e0;
    --color-text-secondary: #a0a0a0;
    --color-border: #333333;
    --color-primary: #8ab4f8;
    --color-primary-text: #1a1a1a;
  }

  /* Images: reduce brightness to avoid eye strain */
  img:not([src*=".svg"]) {
    filter: brightness(0.9);
  }
}

/* Apply through CSS variables */
body {
  background: var(--color-bg);
  color: var(--color-text);
}

.card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
}

.btn-primary {
  background: var(--color-primary);
  color: var(--color-primary-text);
}
```

```html
<!-- Manual dark mode toggle (with system default) -->
<button id="theme-toggle" aria-label="Toggle dark mode">
  <span class="theme-icon-light" aria-hidden="true">&#9728;</span>
  <span class="theme-icon-dark" aria-hidden="true">&#9790;</span>
</button>
```

```js
// Dark mode toggle with system preference fallback
class ThemeManager {
  constructor() {
    this.key = 'theme-preference';
    this.toggle = document.getElementById('theme-toggle');

    // Load saved preference, or fall back to system
    const saved = localStorage.getItem(this.key);
    if (saved) {
      this.set(saved);
    } else {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this.set(prefersDark ? 'dark' : 'light');
    }

    this.toggle.addEventListener('click', () => {
      const next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
      this.set(next);
      localStorage.setItem(this.key, next);
    });

    // React to OS theme change
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (!localStorage.getItem(this.key)) {
        this.set(e.matches ? 'dark' : 'light');
      }
    });
  }

  set(theme) {
    document.documentElement.dataset.theme = theme;
    // Update meta theme-color for browser chrome
    document.querySelector('meta[name="theme-color"]')
      ?.setAttribute('content', theme === 'dark' ? '#121212' : '#ffffff');
  }
}

new ThemeManager();
```

```css
/* Use data attribute for manual toggle (overrides media query) */
[data-theme="dark"] {
  --color-bg: #121212;
  --color-surface: #1e1e1e;
  --color-text: #e0e0e0;
  --color-text-secondary: #a0a0a0;
  --color-border: #333333;
  --color-primary: #8ab4f8;
  --color-primary-text: #1a1a1a;
}
```

### 7.4 Focus Management

Mobile keyboards and switch access devices use focus navigation. Focus must be managed correctly for modals, route changes, and dynamic content.

```js
// Trap focus in modal
function trapFocus(modal) {
  const focusable = modal.querySelectorAll(
    'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
  );
  const first = focusable[0];
  const last = focusable[focusable.length - 1];

  modal.addEventListener('keydown', (e) => {
    if (e.key !== 'Tab') return;

    if (e.shiftKey) {
      if (document.activeElement === first) {
        e.preventDefault();
        last.focus();
      }
    } else {
      if (document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  });

  first.focus();
}

// Move focus on SPA route change
function onRouteChange() {
  const main = document.getElementById('main-content');
  main.setAttribute('tabindex', '-1');
  main.focus();
  // Remove tabindex after blur to keep natural tab order
  main.addEventListener('blur', () => main.removeAttribute('tabindex'), {
    once: true,
  });
}
```

```css
/* Visible focus indicators — essential for keyboard/switch users */
:focus-visible {
  outline: 2px solid #1a73e8;
  outline-offset: 2px;
}

/* Remove default focus ring only if focus-visible is supported */
:focus:not(:focus-visible) {
  outline: none;
}
```

### 7.5 Orientation and Viewport

```css
/* Support both orientations gracefully */
@media (orientation: landscape) and (max-height: 500px) {
  /* Landscape on phone — reduce vertical spacing */
  .hero {
    min-height: auto;
    padding: 1rem 2rem;
  }

  .bottom-tabs {
    /* Consider hiding or minimizing in landscape */
    display: none;
  }
}

/* Never lock orientation in CSS — it breaks accessibility */
/* Users with motor disabilities may mount their device in a fixed orientation */
```

---

## 8. Mobile Testing

### 8.1 Chrome DevTools Device Mode

The fastest feedback loop for mobile development. Open DevTools (F12), click the device toggle toolbar, or press `Ctrl+Shift+M` (Windows/Linux) / `Cmd+Shift+M` (Mac).

**What DevTools simulates well:**
- Viewport size and pixel ratio
- Touch events (tap, swipe)
- Throttled CPU and network
- User-agent string
- Media queries

**What DevTools does NOT simulate:**
- Actual mobile browser rendering engines (especially Safari/WebKit)
- Virtual keyboard behavior and its effect on viewport
- iOS-specific quirks (rubber-band scrolling, safe areas, zoom on input focus)
- Real-world performance characteristics (thermal throttling, memory pressure)
- Haptic feedback, GPS, accelerometer

**Key device presets to test:**
- iPhone SE (375x667) — small screen baseline
- iPhone 14 Pro (393x852) — common iOS device
- Pixel 7 (412x915) — common Android device
- iPad Mini (768x1024) — small tablet
- Samsung Galaxy Fold (280x653 folded) — foldable edge case

### 8.2 Lighthouse Mobile Audit

```bash
# Run from command line
npx lighthouse https://example.com \
  --form-factor=mobile \
  --throttling.cpuSlowdownMultiplier=4 \
  --output=html \
  --output-path=./lighthouse-mobile.html

# Key mobile-specific audits Lighthouse checks:
# - Tap targets sized appropriately
# - Content sized to viewport
# - Font sizes legible
# - Viewport meta tag present
# - No horizontal scrolling
# - Core Web Vitals (LCP, INP, CLS)
```

**Target scores for mobile:**
| Category | Target | Minimum |
|----------|--------|---------|
| Performance | 90+ | 70 |
| Accessibility | 100 | 95 |
| Best Practices | 100 | 90 |
| SEO | 100 | 90 |

### 8.3 Real Device Testing

DevTools is for development. Real devices are for validation. You need both.

**Minimum real device test matrix:**

| Category | Devices | Why |
|----------|---------|-----|
| iOS Safari | Latest iPhone | WebKit rendering, iOS-specific bugs |
| iOS Safari | iPhone SE (small) | Small screen edge cases |
| Android Chrome | Mid-range Android (e.g., Pixel 6a) | Performance on real hardware |
| Android Chrome | Samsung Galaxy (One UI) | Samsung's browser modifications |
| Tablet | iPad | Tablet layout breakpoints |

**Remote debugging:**
- **iOS:** Safari > Develop > [device name] (requires Mac + cable)
- **Android:** Chrome DevTools > `chrome://inspect` (requires USB + ADB)

### 8.4 Cloud Device Testing

For broader coverage without buying dozens of devices:

| Service | Strength | Use case |
|---------|----------|----------|
| BrowserStack | Real devices, live + automated | Cross-device regression testing |
| Sauce Labs | CI integration, wide device range | Automated test suites |
| LambdaTest | Budget-friendly, good device coverage | Manual cross-browser testing |

```js
// Example: BrowserStack WebDriver config for mobile
const capabilities = {
  'bstack:options': {
    os: 'ios',
    osVersion: '17',
    deviceName: 'iPhone 15',
    realMobile: true,
    local: false,
    buildName: 'Mobile Regression',
  },
  browserName: 'safari',
};
```

### 8.5 Automated Mobile Testing

```js
// Playwright mobile testing
import { test, devices } from '@playwright/test';

// Use predefined device profiles
test.use(devices['iPhone 14']);

test('mobile navigation works', async ({ page }) => {
  await page.goto('/');

  // Hamburger should be visible on mobile
  await expect(page.locator('.hamburger')).toBeVisible();

  // Desktop nav should be hidden
  await expect(page.locator('.desktop-nav')).toBeHidden();

  // Open mobile menu
  await page.locator('.hamburger').tap();
  await expect(page.locator('.nav-drawer')).toBeVisible();

  // Navigate
  await page.locator('.nav-drawer a[href="/products"]').tap();
  await expect(page).toHaveURL('/products');
});

test('touch targets are large enough', async ({ page }) => {
  await page.goto('/');

  const buttons = page.locator('button, a, [role="button"]');
  const count = await buttons.count();

  for (let i = 0; i < count; i++) {
    const box = await buttons.nth(i).boundingBox();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  }
});
```

```js
// Cypress mobile testing
// cypress.config.js
export default {
  e2e: {
    viewportWidth: 375,
    viewportHeight: 667,
  },
};

// cypress/e2e/mobile.cy.js
describe('Mobile experience', () => {
  beforeEach(() => {
    cy.viewport('iphone-x');
  });

  it('shows mobile navigation', () => {
    cy.visit('/');
    cy.get('.hamburger').should('be.visible');
    cy.get('.desktop-nav').should('not.be.visible');
  });

  it('form inputs trigger correct keyboards', () => {
    cy.visit('/contact');
    cy.get('input[name="email"]').should('have.attr', 'type', 'email');
    cy.get('input[name="phone"]').should('have.attr', 'type', 'tel');
  });
});
```

---

## 9. Common Mobile Anti-Patterns

### 9.1 Hover-Dependent Interactions

Hover does not exist on touch devices. Any content or functionality hidden behind `:hover` is inaccessible to mobile users.

```css
/* ANTI-PATTERN: content only accessible via hover */
.dropdown-menu {
  display: none;
}
.dropdown:hover .dropdown-menu {
  display: block;
}

/* FIX: use click/tap with aria-expanded */
.dropdown-menu {
  display: none;
}
.dropdown-menu.is-open {
  display: block;
}
```

```css
/* ANTI-PATTERN: tooltip only on hover */
.tooltip {
  display: none;
}
.trigger:hover + .tooltip {
  display: block;
}

/* FIX: use focus as well, or use a tap-to-toggle pattern */
.trigger:hover + .tooltip,
.trigger:focus + .tooltip,
.trigger[aria-expanded="true"] + .tooltip {
  display: block;
}

/* Or detect touch capability */
@media (hover: none) {
  /* Touch device — show info inline instead of tooltip */
  .tooltip-inline {
    display: block;
  }
  .tooltip-hover {
    display: none;
  }
}

@media (hover: hover) {
  /* Mouse device — hover tooltip is fine */
  .tooltip-inline {
    display: none;
  }
}
```

### 9.2 Tiny Touch Targets

The most common mobile failure. Links inside paragraphs, small icon buttons, and densely packed navigation all suffer from this.

```css
/* ANTI-PATTERN: link in running text with no target expansion */
p a {
  /* Text size link — maybe 14px tall, impossible to tap accurately */
}

/* FIX: increase tap target for inline links */
p a {
  padding: 4px 0;
  margin: -4px 0;
  /* Or use ::after pseudo-element to expand target */
}

/* ANTI-PATTERN: social icons with no padding */
.social-icon {
  width: 16px;
  height: 16px;
  /* 16x16 tap target = user rage */
}

/* FIX: adequate padding around small icons */
.social-icon {
  width: 16px;
  height: 16px;
  padding: 16px;
  box-sizing: content-box;
  /* Actual tap target: 48x48 */
}
```

### 9.3 Horizontal Scroll

Unexpected horizontal scrolling is disorienting on mobile and breaks the scroll model users expect.

```css
/* ANTI-PATTERN: elements wider than viewport */
.wide-table {
  width: 900px; /* Forces horizontal scroll */
}

.absolute-positioned {
  position: absolute;
  right: -50px; /* Extends past viewport */
}

/* FIX: contain overflow */
html, body {
  overflow-x: hidden; /* Last resort — find and fix the actual overflow */
}

/* FIX: responsive tables */
.table-wrapper {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
  max-width: 100%;
}

/* Or stack table cells on mobile */
@media (max-width: 640px) {
  table, thead, tbody, th, td, tr {
    display: block;
  }

  thead {
    /* Hide header row — use data attributes instead */
    position: absolute;
    width: 1px;
    height: 1px;
    clip: rect(0, 0, 0, 0);
  }

  td::before {
    content: attr(data-label);
    font-weight: 600;
    display: block;
    margin-bottom: 4px;
  }

  td {
    padding: 8px 16px;
    border-bottom: 1px solid #eee;
  }
}
```

### 9.4 The `100vh` Problem

On mobile browsers, `100vh` includes the browser chrome (URL bar, navigation bar), causing content to overflow the actual visible area.

```css
/* ANTI-PATTERN: 100vh for full-screen sections */
.hero {
  height: 100vh; /* Extends behind browser chrome on mobile */
}

/* FIX: use dvh (dynamic viewport height) */
.hero {
  height: 100dvh; /* Adjusts as browser chrome shows/hides */
}

/* FIX: fallback for browsers without dvh support */
.hero {
  height: 100vh; /* Fallback */
  height: 100dvh; /* Modern browsers */
}

/* Other viewport units to know: */
/* svh — small viewport height (browser chrome visible, smallest viewport) */
/* lvh — large viewport height (browser chrome hidden, largest viewport) */
/* dvh — dynamic viewport height (current actual viewport) */

/* Use svh for fixed-position elements to avoid overflow */
.modal {
  max-height: 100svh;
}

/* Use dvh for full-height layouts */
.app-shell {
  height: 100dvh;
  display: flex;
  flex-direction: column;
}
```

### 9.5 Fixed Position Issues

Fixed positioning on mobile is fragile. The virtual keyboard, browser chrome, and rubber-band scrolling all cause problems.

```css
/* ANTI-PATTERN: fixed header + fixed footer + scrollable content */
/* This breaks when the virtual keyboard opens on iOS */

/* FIX: use viewport-relative positioning and handle keyboard */
.app-shell {
  height: 100dvh;
  display: flex;
  flex-direction: column;
}

.app-header {
  flex-shrink: 0;
  position: sticky;
  top: 0;
  z-index: 100;
}

.app-content {
  flex: 1;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
}

.app-footer {
  flex-shrink: 0;
}
```

```js
// Handle virtual keyboard on iOS/Android
// The Visual Viewport API gives the actual visible area
if (window.visualViewport) {
  window.visualViewport.addEventListener('resize', () => {
    const keyboardHeight =
      window.innerHeight - window.visualViewport.height;

    if (keyboardHeight > 150) {
      // Keyboard is open
      document.documentElement.style.setProperty(
        '--keyboard-height',
        `${keyboardHeight}px`
      );
      document.body.classList.add('keyboard-open');
    } else {
      document.documentElement.style.setProperty('--keyboard-height', '0px');
      document.body.classList.remove('keyboard-open');
    }
  });
}
```

```css
/* Adjust layout when keyboard is open */
.keyboard-open .bottom-tabs {
  display: none; /* Hide tab bar when keyboard covers it */
}

.keyboard-open .action-bar {
  /* Move above keyboard */
  bottom: var(--keyboard-height);
}
```

### 9.6 Viewport Meta Mistakes

```html
<!-- CORRECT viewport meta -->
<meta name="viewport" content="width=device-width, initial-scale=1" />

<!-- ANTI-PATTERN: disabling zoom — WCAG failure -->
<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no" />
<!-- Users with low vision NEED pinch-to-zoom. Never disable it. -->

<!-- ANTI-PATTERN: fixed width -->
<meta name="viewport" content="width=1024" />
<!-- Forces desktop layout on mobile, user must zoom and pan -->

<!-- ANTI-PATTERN: missing viewport meta entirely -->
<!-- Mobile browsers will use a default ~980px viewport, shrinking the page -->
```

### 9.7 Blocking Touch Interactions

```css
/* ANTI-PATTERN: disabling selection and callouts globally */
* {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  user-select: none;
}
/* This prevents users from copying text, which is an accessibility issue */

/* FIX: only disable on interactive elements that need it */
button, .drag-handle {
  -webkit-user-select: none;
  user-select: none;
}

/* DO allow text selection on content */
p, h1, h2, h3, span, li {
  -webkit-user-select: auto;
  user-select: auto;
}
```

```js
// ANTI-PATTERN: preventing all touch defaults
document.addEventListener('touchmove', (e) => e.preventDefault());
// This breaks scrolling entirely

// FIX: only prevent default on specific elements
dragHandle.addEventListener('touchmove', (e) => e.preventDefault(), {
  passive: false,
});

// Use passive event listeners for scroll-related events
window.addEventListener('scroll', onScroll, { passive: true });
window.addEventListener('touchstart', onTouchStart, { passive: true });
```

### 9.8 Ignoring Safe Areas (Notch/Dynamic Island)

```css
/* ANTI-PATTERN: content hidden behind notch/rounded corners */

/* FIX: use env() safe area insets */
body {
  padding-top: env(safe-area-inset-top);
  padding-right: env(safe-area-inset-right);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
}

/* Required: tell the browser you handle safe areas */
/* In <head>: */
/* <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" /> */

/* Fixed elements need safe area padding */
.bottom-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 12px 16px;
  padding-bottom: calc(12px + env(safe-area-inset-bottom));
}

.top-bar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  padding: 12px 16px;
  padding-top: calc(12px + env(safe-area-inset-top));
}
```

---

## 10. Audit Dimensions for Mobile Readiness

Use this checklist to evaluate any website or webapp for mobile readiness. Each dimension scores 0-3 (0 = absent, 1 = partial, 2 = adequate, 3 = exemplary). Total score /60.

### 10.1 Viewport and Layout (0-3)

| Score | Criteria |
|-------|----------|
| 0 | No viewport meta tag, or `width=1024` style fixed width |
| 1 | Viewport meta present but layout overflows on small screens |
| 2 | Responsive layout works on common devices, no horizontal scroll |
| 3 | Fluid layout with container queries, all content accessible at 320px width |

**What to check:**
- [ ] `<meta name="viewport" content="width=device-width, initial-scale=1">` present
- [ ] No horizontal scrollbar at 320px viewport width
- [ ] No content clipped or overlapping at any standard breakpoint
- [ ] Zoom to 200% does not break layout (WCAG 1.4.4)
- [ ] `user-scalable=no` or `maximum-scale=1` NOT present in viewport meta

### 10.2 Touch Targets (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Multiple targets below 24x24px, no spacing between adjacent targets |
| 1 | Most targets 24x24+ but many below 44x44px |
| 2 | All interactive targets 44x44px+, adequate spacing |
| 3 | All targets 48x48px+, 8px+ spacing, primary actions in thumb zone |

**What to check:**
- [ ] All buttons, links, form controls meet 44x44px minimum
- [ ] Adjacent targets have at least 8px gap
- [ ] No overlapping touch targets
- [ ] Primary actions are in bottom half of screen (thumb zone)
- [ ] Checkbox/radio controls are adequately sized

### 10.3 Navigation (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Desktop-only navigation, broken or absent on mobile |
| 1 | Navigation present but hard to use (tiny targets, no close, no escape) |
| 2 | Functional mobile navigation with proper open/close, focus management |
| 3 | Native-feeling navigation, appropriate pattern for IA, smooth transitions |

**What to check:**
- [ ] Mobile navigation pattern appropriate for the site's IA depth
- [ ] Hamburger/overlay has close button AND escape key support
- [ ] Focus is trapped in open navigation overlay
- [ ] `aria-expanded` toggles correctly on trigger button
- [ ] Navigation is keyboard-accessible

### 10.4 Forms (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Forms unusable on mobile (tiny inputs, wrong keyboards, zoom issues) |
| 1 | Forms work but use generic text inputs for all fields |
| 2 | Correct input types, autocomplete, font-size >= 16px |
| 3 | Optimized mobile forms with input modes, autocomplete, validation UX |

**What to check:**
- [ ] All inputs have `font-size: 16px` or greater (no iOS zoom)
- [ ] `type="email"` for email, `type="tel"` for phone, etc.
- [ ] `autocomplete` attributes on name, address, payment fields
- [ ] `inputmode="numeric"` for numeric-but-not-number fields
- [ ] Form submit button is full-width and in thumb zone

### 10.5 Performance (0-3)

| Score | Criteria |
|-------|----------|
| 0 | LCP > 4s on mobile, heavy unoptimized assets |
| 1 | LCP 2.5-4s, some optimization but significant gaps |
| 2 | LCP < 2.5s, INP < 200ms, CLS < 0.1 on mobile |
| 3 | All CWV green on mobile, service worker caching, critical CSS inlined |

**What to check:**
- [ ] Lighthouse mobile performance score >= 70 (target 90+)
- [ ] LCP <= 2.5s on simulated mobile (4x CPU throttle)
- [ ] INP <= 200ms
- [ ] CLS <= 0.1
- [ ] Images use `loading="lazy"` (except above-fold)
- [ ] Above-fold images use `fetchpriority="high"`
- [ ] CSS/JS not render-blocking where avoidable

### 10.6 Accessibility (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Major accessibility barriers (no alt text, no focus management, zoom disabled) |
| 1 | Basic alt text and ARIA, but VoiceOver/TalkBack experience is poor |
| 2 | Good screen reader experience, focus management, visible focus indicators |
| 3 | Full WCAG 2.2 AA compliance, reduced motion support, dark mode, orientation support |

**What to check:**
- [ ] All images have meaningful `alt` text (or `alt=""` for decorative)
- [ ] `prefers-reduced-motion` respected for all animations
- [ ] `prefers-color-scheme` supported (dark mode)
- [ ] Focus indicators visible on all interactive elements
- [ ] Skip link present and functional
- [ ] Content works in both portrait and landscape orientation
- [ ] Zoom to 400% does not lose content or functionality

### 10.7 PWA Readiness (0-3)

| Score | Criteria |
|-------|----------|
| 0 | No manifest, no service worker, no HTTPS |
| 1 | Manifest present but incomplete, basic service worker |
| 2 | Installable PWA with offline fallback page |
| 3 | Full PWA with offline functionality, push notifications, update flow |

**What to check:**
- [ ] HTTPS enforced
- [ ] Web app manifest with required fields (name, icons, start_url, display)
- [ ] Service worker registered and caching critical assets
- [ ] Offline fallback page served when network fails
- [ ] Custom install prompt (not relying on browser's default)
- [ ] App update notification when new version available

### 10.8 Content Adaptation (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Desktop content dumped into mobile viewport, truncated or overflowing |
| 1 | Content fits but is not optimized (long paragraphs, small images) |
| 2 | Mobile-appropriate content sizing, responsive images, readable typography |
| 3 | Content strategy adapted for mobile context (shorter, scannable, action-oriented) |

**What to check:**
- [ ] Responsive images with `srcset` and `sizes`
- [ ] Text readable without zooming (body >= 16px)
- [ ] Line length reasonable on mobile (< 80 characters)
- [ ] Tables have mobile-friendly treatment (scroll or stack)
- [ ] Videos/embeds are responsive
- [ ] No content hidden from mobile users that is available on desktop

### 10.9 Safe Areas and Device Quirks (0-3)

| Score | Criteria |
|-------|----------|
| 0 | Content hidden behind notch, bottom bar overlaps system UI |
| 1 | Basic awareness but inconsistent safe area handling |
| 2 | `env(safe-area-inset-*)` used for fixed elements, `viewport-fit=cover` |
| 3 | Full safe area support, virtual keyboard handling, foldable device support |

**What to check:**
- [ ] `viewport-fit=cover` in viewport meta tag
- [ ] `env(safe-area-inset-*)` used on fixed positioned elements
- [ ] Content not obscured by notch or Dynamic Island
- [ ] Virtual keyboard does not cover input being typed in
- [ ] `100dvh` used instead of `100vh` for full-height layouts

### 10.10 Testing Coverage (0-3)

| Score | Criteria |
|-------|----------|
| 0 | No mobile testing beyond "it looks okay on my laptop with DevTools" |
| 1 | DevTools testing with a couple device sizes |
| 2 | DevTools + Lighthouse + at least one real iOS and one real Android device |
| 3 | Automated mobile tests in CI, real device lab, Lighthouse in CI, visual regression |

**What to check:**
- [ ] Lighthouse mobile audit run and scores documented
- [ ] Tested on real iOS Safari (not just Chrome DevTools iPhone mode)
- [ ] Tested on real Android Chrome
- [ ] Automated tests include mobile viewport assertions
- [ ] Visual regression tests at mobile breakpoints

### Scoring Summary

| Dimension | Score |
|-----------|-------|
| 10.1 Viewport and Layout | /3 |
| 10.2 Touch Targets | /3 |
| 10.3 Navigation | /3 |
| 10.4 Forms | /3 |
| 10.5 Performance | /3 |
| 10.6 Accessibility | /3 |
| 10.7 PWA Readiness | /3 |
| 10.8 Content Adaptation | /3 |
| 10.9 Safe Areas and Device Quirks | /3 |
| 10.10 Testing Coverage | /3 |
| **Total** | **/30** |

**Ratings:**
- **27-30:** Production-grade mobile experience
- **21-26:** Good mobile support, address specific gaps
- **15-20:** Functional but needs significant mobile optimization
- **Below 15:** Mobile experience is a liability — prioritize remediation

---

## Quick Reference: Mobile Meta Tags

```html
<head>
  <!-- Viewport — never restrict zoom -->
  <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />

  <!-- Theme color for browser chrome -->
  <meta name="theme-color" content="#1a73e8" media="(prefers-color-scheme: light)" />
  <meta name="theme-color" content="#121212" media="(prefers-color-scheme: dark)" />

  <!-- iOS webapp -->
  <meta name="apple-mobile-web-app-capable" content="yes" />
  <meta name="apple-mobile-web-app-status-bar-style" content="default" />
  <meta name="apple-mobile-web-app-title" content="App Name" />
  <link rel="apple-touch-icon" href="/icons/apple-touch-icon-180.png" />

  <!-- PWA manifest -->
  <link rel="manifest" href="/manifest.json" />

  <!-- Prevent phone number auto-detection (when numbers are not phone numbers) -->
  <meta name="format-detection" content="telephone=no" />

  <!-- Preconnect to critical origins -->
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://cdn.example.com" crossorigin />

  <!-- Preload critical fonts -->
  <link rel="preload" href="/fonts/main.woff2" as="font" type="font/woff2" crossorigin />
</head>
```

---

## Quick Reference: CSS Reset for Mobile

```css
/* Mobile-first CSS reset — apply to every project */
*,
*::before,
*::after {
  box-sizing: border-box;
}

* {
  margin: 0;
}

html {
  /* Prevent font-size inflation on orientation change */
  -webkit-text-size-adjust: 100%;
  text-size-adjust: 100%;
}

body {
  min-height: 100dvh;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
}

img, picture, video, canvas, svg {
  display: block;
  max-width: 100%;
}

input, button, textarea, select {
  font: inherit;
  font-size: max(16px, 1em); /* Prevent iOS zoom */
}

button {
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

p, h1, h2, h3, h4, h5, h6 {
  overflow-wrap: break-word;
}

/* Smooth scrolling, but respect reduced motion */
@media (prefers-reduced-motion: no-preference) {
  html {
    scroll-behavior: smooth;
  }
}
```
