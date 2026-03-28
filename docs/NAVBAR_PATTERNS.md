# Navigation Bar Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** W3C WAI ARIA APG, NNGroup, Smashing Magazine, GOV.UK Design System, Apple HIG, Material Design 3, MDN, WebAIM, CSS-Tricks, web.dev, A11Y Collective, Level Access, Astro docs, React Spectrum (Adobe)
**Last updated:** 2026-03-28

---

## 1. Desktop Navigation Patterns

### 1.1 Horizontal Navigation Bar

The default pattern for marketing sites, SaaS products, and content sites. Logo left, primary links center or left-aligned, utility/CTA right.

**Structure:**

```html
<header class="site-header">
  <nav aria-label="Main">
    <a href="/" class="logo" aria-label="Home">
      <img src="/logo.svg" alt="Acme" width="120" height="32" />
    </a>

    <ul role="list" class="nav-links">
      <li><a href="/products">Products</a></li>
      <li><a href="/pricing">Pricing</a></li>
      <li><a href="/docs">Docs</a></li>
      <li><a href="/blog">Blog</a></li>
    </ul>

    <div class="nav-utility">
      <button type="button" aria-label="Search" class="nav-search-trigger">
        <svg><!-- search icon --></svg>
      </button>
      <a href="/login" class="btn-secondary">Log in</a>
      <a href="/signup" class="btn-primary">Get Started</a>
    </div>
  </nav>
</header>
```

```css
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.site-header nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 1.5rem;
  height: 4rem;
}

.nav-links {
  display: flex;
  gap: 2rem;
  list-style: none;
  margin: 0;
  padding: 0;
}

.nav-links a {
  font-size: 0.9375rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  text-decoration: none;
  padding: 0.5rem 0;
  transition: color 150ms ease;
}

.nav-links a:hover,
.nav-links a[aria-current="page"] {
  color: var(--color-text-primary);
}

.nav-utility {
  display: flex;
  align-items: center;
  gap: 1rem;
}
```

**When to use:** Sites with 3-7 top-level destinations. Most common pattern on the web. Users expect it and scan it without instruction.

### 1.2 Mega Menu

A large dropdown panel that opens from a top-level nav item, showing grouped links, descriptions, and optionally images or icons. Used when a single nav category contains 8+ destinations that benefit from visual grouping.

**When to use:**
- E-commerce (product categories)
- Enterprise SaaS with many product lines
- Documentation sites with many sections
- University/government sites with deep hierarchies

**When NOT to use:**
- Sites with fewer than 15 total pages
- Mobile-first products where the mega menu cannot translate to small screens
- When the groupings are unclear or forced

**Structure:**

```html
<li class="nav-item has-mega-menu">
  <button
    type="button"
    aria-expanded="false"
    aria-haspopup="true"
    class="nav-link"
  >
    Products
    <svg aria-hidden="true" class="chevron"><!-- down arrow --></svg>
  </button>

  <div class="mega-menu" role="region" aria-label="Products menu">
    <div class="mega-menu-grid">
      <div class="mega-menu-group">
        <h3 class="mega-menu-heading">Platform</h3>
        <ul role="list">
          <li>
            <a href="/products/analytics">
              <span class="mega-menu-icon"><!-- icon --></span>
              <span>
                <strong>Analytics</strong>
                <small>Track user behavior in real time</small>
              </span>
            </a>
          </li>
          <!-- more items -->
        </ul>
      </div>

      <div class="mega-menu-group">
        <h3 class="mega-menu-heading">Integrations</h3>
        <ul role="list">
          <!-- items -->
        </ul>
      </div>

      <div class="mega-menu-featured">
        <h3 class="mega-menu-heading">What's New</h3>
        <a href="/blog/release-4">
          <img src="/release-4-thumb.webp" alt="" width="240" height="135" />
          <strong>Release 4.0</strong>
          <small>See what's new in the latest release</small>
        </a>
      </div>
    </div>
  </div>
</li>
```

```css
.mega-menu {
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  width: min(1100px, calc(100vw - 2rem));
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 0.75rem;
  box-shadow: 0 20px 40px rgb(0 0 0 / 0.1);
  padding: 2rem;
  opacity: 0;
  visibility: hidden;
  transition: opacity 200ms ease, visibility 200ms ease;
}

.nav-item.is-open .mega-menu {
  opacity: 1;
  visibility: visible;
}

.mega-menu-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 280px;
  gap: 2rem;
}

.mega-menu-group ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.mega-menu-group a {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  border-radius: 0.5rem;
  text-decoration: none;
  color: inherit;
  transition: background 150ms ease;
}

.mega-menu-group a:hover {
  background: var(--color-surface-hover);
}

.mega-menu-group a strong {
  display: block;
  font-size: 0.9375rem;
}

.mega-menu-group a small {
  display: block;
  font-size: 0.8125rem;
  color: var(--color-text-tertiary);
  margin-top: 0.125rem;
}
```

### 1.3 Hover vs Click for Dropdowns

This is one of the most debated UX decisions in navigation design.

**Hover-triggered (desktop only):**
- Faster for expert users who know where they're going
- Risky: accidental triggers, inaccessible on touch, the diagonal problem (see section 11)
- Must include a hover delay of 200-300ms before opening to prevent accidental triggers
- Must include a grace period of 300-500ms before closing when mouse leaves, so users can move diagonally to the submenu

**Click-triggered:**
- More intentional, fewer accidental opens
- Works on all input types (mouse, touch, keyboard)
- Required as a fallback even when hover is the primary trigger
- Slightly slower for power users

**Best practice:** Use click as the primary trigger. If you add hover on desktop, always provide click as a fallback. Never rely on hover alone.

```js
// Hover with delay — desktop only enhancement
const HOVER_OPEN_DELAY = 250;  // ms before opening
const HOVER_CLOSE_DELAY = 400; // ms grace period before closing

class NavDropdown {
  constructor(trigger, menu) {
    this.trigger = trigger;
    this.menu = menu;
    this.openTimer = null;
    this.closeTimer = null;
    this.isOpen = false;

    // Click always works (primary trigger)
    this.trigger.addEventListener('click', () => this.toggle());

    // Hover is a desktop enhancement
    const container = this.trigger.closest('.nav-item');
    container.addEventListener('mouseenter', () => this.scheduleOpen());
    container.addEventListener('mouseleave', () => this.scheduleClose());

    // Keyboard
    this.trigger.addEventListener('keydown', (e) => this.handleKeydown(e));
  }

  scheduleOpen() {
    clearTimeout(this.closeTimer);
    this.openTimer = setTimeout(() => this.open(), HOVER_OPEN_DELAY);
  }

  scheduleClose() {
    clearTimeout(this.openTimer);
    this.closeTimer = setTimeout(() => this.close(), HOVER_CLOSE_DELAY);
  }

  toggle() {
    this.isOpen ? this.close() : this.open();
  }

  open() {
    clearTimeout(this.closeTimer);
    this.isOpen = true;
    this.trigger.setAttribute('aria-expanded', 'true');
    this.menu.hidden = false;

    // Focus first link in menu
    const firstLink = this.menu.querySelector('a, button');
    firstLink?.focus();
  }

  close() {
    clearTimeout(this.openTimer);
    this.isOpen = false;
    this.trigger.setAttribute('aria-expanded', 'false');
    this.menu.hidden = true;
  }

  handleKeydown(e) {
    switch (e.key) {
      case 'Enter':
      case ' ':
      case 'ArrowDown':
        e.preventDefault();
        this.open();
        break;
      case 'Escape':
        this.close();
        this.trigger.focus();
        break;
    }
  }
}
```

### 1.4 Flyout Menus (Nested Submenus)

Flyout menus appear to the side of a parent menu item, creating a multi-level hierarchy. Common in complex applications and e-commerce.

**Rules:**
- Maximum two levels deep. Three-level flyouts are nearly impossible to navigate on any input device.
- Each level must be reachable by click, not just hover.
- Submenu must open on the side with available viewport space (flip from right to left if near the edge).
- Use the safe triangle technique (section 11) to prevent accidental closure during diagonal mouse movement.

### 1.5 Grouped Navigation

When you have more than 7 top-level items, group related items under dropdown categories rather than showing them all in the horizontal bar.

**Before (too many items):**
```
Home | Products | Pricing | Docs | API | Blog | Changelog | Company | Careers | Contact
```

**After (grouped):**
```
Products (dropdown) | Pricing | Docs (dropdown) | Company (dropdown)
```

Group by user intent: what they're buying, what they're learning, who you are. NNGroup research shows users scan navigation labels and form expectations about content — unclear groupings double the cognitive analysis required.

---

## 2. Dropdown Menus

### 2.1 Single-Column Dropdown

The simplest dropdown. A vertical list of 3-8 links.

```html
<div class="dropdown" role="region" aria-label="Solutions">
  <ul role="list">
    <li><a href="/solutions/startup">For Startups</a></li>
    <li><a href="/solutions/enterprise">For Enterprise</a></li>
    <li><a href="/solutions/agency">For Agencies</a></li>
  </ul>
</div>
```

```css
.dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  left: 0;
  min-width: 200px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  box-shadow: 0 8px 24px rgb(0 0 0 / 0.08);
  padding: 0.5rem;
}

.dropdown a {
  display: block;
  padding: 0.5rem 0.75rem;
  border-radius: 0.375rem;
  text-decoration: none;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  transition: background 150ms ease, color 150ms ease;
}

.dropdown a:hover,
.dropdown a:focus-visible {
  background: var(--color-surface-hover);
  color: var(--color-text-primary);
}
```

### 2.2 Multi-Column Dropdown

For 8-15 items that can be logically grouped into 2-3 columns.

```css
.dropdown-multi {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.25rem 2rem;
  min-width: 400px;
  padding: 1rem;
}
```

### 2.3 Mega Menu with Categories

See section 1.2 for full implementation. The key distinction: mega menus use headings to group items into named categories, often with descriptions and icons. Standard dropdowns are flat lists.

### 2.4 Keyboard Navigation for Dropdowns

Per the W3C ARIA Authoring Practices Guide (APG), keyboard interaction for menu-like dropdowns follows these rules:

| Key | Action |
|-----|--------|
| Enter / Space | Opens the menu when focused on trigger. Activates the focused menuitem. |
| Escape | Closes the menu. Returns focus to the trigger button. |
| Down Arrow | Moves focus to the next item. From the last item, wraps to the first. |
| Up Arrow | Moves focus to the previous item. From the first item, wraps to the last. |
| Home | Moves focus to the first item. |
| End | Moves focus to the last item. |
| Right Arrow | In a menubar, moves to the next top-level item. In a submenu, opens a nested submenu. |
| Left Arrow | In a menubar, moves to the previous top-level item. In a submenu, closes the submenu and returns focus to the parent. |
| Tab | Exits the menu entirely. Closes all open menus. |
| Type-ahead | Moves focus to the next item starting with the typed character. |

**Focus management rules:**
- When a menu opens, focus moves to the first item (or the previously focused item).
- Only one item in the menu is in the tab order at a time (roving tabindex or aria-activedescendant).
- Items within the menu have `tabindex="-1"` except the active item which has `tabindex="0"`.

### 2.5 ARIA Markup for Navigation Menus

There is an important distinction between **site navigation** and **application menus**.

**For site navigation (most websites):** Do NOT use `role="menubar"` / `role="menu"` / `role="menuitem"`. These ARIA roles are for application menus (like a desktop app's File/Edit/View menu) and change how screen readers interpret links. When links are marked as menuitems, they no longer appear in the screen reader's links list, and users lose the ability to navigate by link shortcuts.

**Correct pattern for site navigation:**

```html
<nav aria-label="Main">
  <ul role="list">
    <li>
      <button
        aria-expanded="false"
        aria-haspopup="true"
      >
        Products
      </button>
      <div class="dropdown" hidden>
        <ul role="list">
          <li><a href="/products/analytics">Analytics</a></li>
          <li><a href="/products/monitoring">Monitoring</a></li>
        </ul>
      </div>
    </li>
    <li><a href="/pricing">Pricing</a></li>
  </ul>
</nav>
```

**For application menus (rare on websites, common in web apps):**

```html
<nav aria-label="Main">
  <ul role="menubar">
    <li role="none">
      <button
        role="menuitem"
        aria-haspopup="true"
        aria-expanded="false"
        tabindex="0"
      >
        File
      </button>
      <ul role="menu">
        <li role="none">
          <a role="menuitem" tabindex="-1" href="/new">New</a>
        </li>
        <li role="none">
          <a role="menuitem" tabindex="-1" href="/open">Open</a>
        </li>
        <li role="separator"></li>
        <li role="none">
          <a role="menuitem" tabindex="-1" href="/save">Save</a>
        </li>
      </ul>
    </li>
  </ul>
</nav>
```

**Key ARIA attributes:**

| Attribute | Where | Purpose |
|-----------|-------|---------|
| `aria-expanded="true/false"` | Trigger button | Tells screen reader whether dropdown is open |
| `aria-haspopup="true"` | Trigger button | Indicates the button opens a popup |
| `aria-label` | `<nav>` element | Labels the navigation landmark ("Main", "Footer", etc.) |
| `aria-current="page"` | Active link | Indicates the current page in navigation |
| `role="none"` | `<li>` wrapper in menubar | Removes list semantics that interfere with menu role |
| `role="separator"` | Divider `<li>` | Visual and semantic separator between groups |

**The rule of thumb:** If your "menu" is a list of links for navigating pages, use `<nav>` + `<ul>` + `<a>` with `aria-expanded` on triggers. If your "menu" is a list of actions (like right-click context menus or application menubars), use the full ARIA menu roles.

---

## 3. Mobile Hamburger Patterns

### 3.1 Slide-In Drawer

The most common mobile navigation pattern. A panel slides in from the left or right edge.

**Left drawer:** The dominant convention. Users expect it from years of Android navigation drawers and iOS apps with side menus. Left-to-right reading cultures scan from the left. Place the hamburger icon on the left or top-left.

**Right drawer:** Less common. Can feel more natural when the hamburger trigger is on the right side of the header (common in minimal designs with a centered logo). Keeps the user's thumb closer to the trigger on right-handed use.

**Implementation:**

```html
<button
  type="button"
  class="hamburger"
  aria-expanded="false"
  aria-controls="mobile-menu"
  aria-label="Open menu"
>
  <span class="hamburger-line"></span>
  <span class="hamburger-line"></span>
  <span class="hamburger-line"></span>
</button>

<div
  id="mobile-menu"
  class="mobile-drawer"
  role="dialog"
  aria-modal="true"
  aria-label="Site navigation"
  hidden
>
  <div class="drawer-backdrop"></div>
  <div class="drawer-panel">
    <button
      type="button"
      class="drawer-close"
      aria-label="Close menu"
    >
      <svg aria-hidden="true"><!-- X icon --></svg>
    </button>

    <nav aria-label="Main">
      <ul role="list" class="mobile-nav">
        <li><a href="/products">Products</a></li>
        <li>
          <button
            type="button"
            aria-expanded="false"
            class="mobile-nav-expandable"
          >
            Solutions
            <svg aria-hidden="true" class="chevron"><!-- chevron --></svg>
          </button>
          <ul role="list" class="mobile-nav-submenu" hidden>
            <li><a href="/solutions/startup">For Startups</a></li>
            <li><a href="/solutions/enterprise">For Enterprise</a></li>
          </ul>
        </li>
        <li><a href="/pricing">Pricing</a></li>
        <li><a href="/docs">Docs</a></li>
      </ul>
    </nav>

    <div class="drawer-footer">
      <a href="/login" class="btn-secondary btn-full">Log in</a>
      <a href="/signup" class="btn-primary btn-full">Get Started</a>
    </div>
  </div>
</div>
```

```css
.mobile-drawer {
  position: fixed;
  inset: 0;
  z-index: 200;
}

.drawer-backdrop {
  position: absolute;
  inset: 0;
  background: rgb(0 0 0 / 0.4);
  opacity: 0;
  transition: opacity 300ms ease;
}

.mobile-drawer:not([hidden]) .drawer-backdrop {
  opacity: 1;
}

.drawer-panel {
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  width: min(320px, 85vw);
  background: var(--color-surface);
  transform: translateX(-100%);
  transition: transform 300ms cubic-bezier(0.32, 0.72, 0, 1);
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  display: flex;
  flex-direction: column;
  padding: 1rem;
}

.mobile-drawer:not([hidden]) .drawer-panel {
  transform: translateX(0);
}

/* Right drawer variant */
.drawer-panel--right {
  left: auto;
  right: 0;
  transform: translateX(100%);
}

.mobile-drawer:not([hidden]) .drawer-panel--right {
  transform: translateX(0);
}

.mobile-nav {
  list-style: none;
  padding: 0;
  margin: 0;
}

.mobile-nav > li > a,
.mobile-nav-expandable {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 0.875rem 1rem;
  font-size: 1.0625rem;
  font-weight: 500;
  color: var(--color-text-primary);
  text-decoration: none;
  border: none;
  background: none;
  border-radius: 0.5rem;
  cursor: pointer;
  min-height: 44px; /* Touch target minimum */
}

.mobile-nav > li > a:active,
.mobile-nav-expandable:active {
  background: var(--color-surface-hover);
}

.mobile-nav-submenu {
  list-style: none;
  padding: 0 0 0 1.5rem;
  margin: 0;
}

.mobile-nav-submenu a {
  display: block;
  padding: 0.625rem 1rem;
  font-size: 0.9375rem;
  color: var(--color-text-secondary);
  text-decoration: none;
  min-height: 44px;
  display: flex;
  align-items: center;
}

.drawer-footer {
  margin-top: auto;
  padding: 1rem 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}
```

### 3.2 Full-Screen Overlay

The entire viewport becomes the menu. Popular with portfolio sites, agencies, and brands that want a dramatic transition.

```css
.fullscreen-menu {
  position: fixed;
  inset: 0;
  z-index: 200;
  background: var(--color-surface);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  opacity: 0;
  visibility: hidden;
  transition: opacity 300ms ease, visibility 300ms ease;
}

.fullscreen-menu.is-open {
  opacity: 1;
  visibility: visible;
}

.fullscreen-menu nav a {
  display: block;
  font-size: clamp(1.5rem, 5vw, 3rem);
  font-weight: 700;
  padding: 0.75rem 0;
  text-decoration: none;
  color: var(--color-text-primary);
}
```

**When to use:** Creative/portfolio sites, brands prioritizing visual impact over efficiency, sites with fewer than 6 top-level items.

**When NOT to use:** E-commerce, SaaS dashboards, content-heavy sites, or anywhere users need to toggle frequently between nav and content.

### 3.3 Bottom Sheet

A panel slides up from the bottom of the viewport. Natural for mobile because the trigger and content are in the thumb zone.

```css
.bottom-sheet {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 200;
  background: var(--color-surface);
  border-radius: 1rem 1rem 0 0;
  max-height: 80vh;
  transform: translateY(100%);
  transition: transform 300ms cubic-bezier(0.32, 0.72, 0, 1);
  overflow-y: auto;
  padding: 1rem;
  padding-bottom: env(safe-area-inset-bottom, 1rem);
}

.bottom-sheet.is-open {
  transform: translateY(0);
}

.bottom-sheet-handle {
  width: 2.5rem;
  height: 0.25rem;
  background: var(--color-border);
  border-radius: 0.125rem;
  margin: 0 auto 1rem;
}
```

**When to use:** Contextual actions, filter menus, secondary navigation. Increasingly common in progressive web apps.

### 3.4 Accordion / Expandable Sections

Within any mobile menu pattern (drawer, overlay, bottom sheet), nested items should use accordion expand/collapse, not flyout submenus. Flyout menus are unusable on touch devices.

```js
// Accordion toggle for mobile nav
document.querySelectorAll('.mobile-nav-expandable').forEach(button => {
  button.addEventListener('click', () => {
    const expanded = button.getAttribute('aria-expanded') === 'true';
    const submenu = button.nextElementSibling;

    button.setAttribute('aria-expanded', String(!expanded));
    submenu.hidden = expanded;

    // Rotate chevron
    button.querySelector('.chevron')?.classList.toggle('is-rotated', !expanded);
  });
});
```

### 3.5 Touch Targets

Apple HIG specifies 44x44pt minimum. Material Design 3 specifies 48x48dp minimum. WCAG 2.2 Success Criterion 2.5.8 (Target Size Minimum) requires at least 24x24 CSS pixels, but recommends 44x44.

**Rules:**
- Interactive elements (links, buttons) must be at least 44x44px in touch contexts.
- Spacing between tap targets must be at least 8px to prevent adjacent taps.
- Padding counts toward the touch target. A small text link with generous padding meets the requirement.

```css
/* Ensure all nav links meet minimum touch target */
@media (pointer: coarse) {
  .mobile-nav a,
  .mobile-nav button {
    min-height: 44px;
    min-width: 44px;
    display: flex;
    align-items: center;
  }
}
```

### 3.6 Closing the Mobile Menu

The menu must close on:
1. **Explicit close button** (the X button inside the menu)
2. **Outside tap** (clicking the backdrop)
3. **Escape key** (keyboard users)
4. **Back button / swipe back** (mobile navigation gesture)
5. **Navigation** (after the user taps a link and the page changes)

```js
class MobileMenu {
  constructor() {
    this.menu = document.getElementById('mobile-menu');
    this.trigger = document.querySelector('.hamburger');
    this.backdrop = this.menu.querySelector('.drawer-backdrop');
    this.closeBtn = this.menu.querySelector('.drawer-close');
    this.focusableEls = null;
    this.previousFocus = null;

    this.trigger.addEventListener('click', () => this.open());
    this.closeBtn.addEventListener('click', () => this.close());
    this.backdrop.addEventListener('click', () => this.close());
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isOpen()) this.close();
    });
  }

  isOpen() {
    return !this.menu.hidden;
  }

  open() {
    this.previousFocus = document.activeElement;
    this.menu.hidden = false;
    this.trigger.setAttribute('aria-expanded', 'true');

    // Prevent body scroll
    document.body.style.overflow = 'hidden';

    // Focus the close button (first focusable element)
    requestAnimationFrame(() => {
      this.closeBtn.focus();
      this.trapFocus();
    });
  }

  close() {
    this.menu.hidden = true;
    this.trigger.setAttribute('aria-expanded', 'false');
    document.body.style.overflow = '';

    // Restore focus to the trigger
    this.previousFocus?.focus();
  }

  trapFocus() {
    this.focusableEls = this.menu.querySelectorAll(
      'a[href], button:not([disabled]), input, textarea, select, [tabindex]:not([tabindex="-1"])'
    );
    const first = this.focusableEls[0];
    const last = this.focusableEls[this.focusableEls.length - 1];

    this.menu.addEventListener('keydown', (e) => {
      if (e.key !== 'Tab') return;

      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    });
  }
}
```

---

## 4. Responsive Transition

### 4.1 Breakpoint Strategy

The transition from desktop nav to mobile hamburger typically happens at one breakpoint. The exact value depends on the number and length of nav items.

**Common breakpoints:**
- 768px (md) — works for 4-5 short nav items
- 1024px (lg) — safer for 6-7 items or longer labels
- Custom — measure where your actual nav items start wrapping or overlapping

```css
/* Mobile-first: hamburger shown by default */
.nav-links { display: none; }
.nav-utility .btn-secondary { display: none; } /* Hide "Log in" text link */
.hamburger { display: flex; }

/* Desktop: show full nav, hide hamburger */
@media (min-width: 1024px) {
  .nav-links { display: flex; }
  .nav-utility .btn-secondary { display: inline-flex; }
  .hamburger { display: none; }
}
```

**Never use `display: none` on the mobile menu container at desktop.** The mobile menu HTML should exist in the DOM always (for accessibility tools), but be hidden via the `hidden` attribute when closed.

### 4.2 What Goes in the Hamburger vs What Stays Visible

**Always visible (both breakpoints):**
- Logo (links to home)
- Primary CTA button ("Get Started", "Sign Up", "Try Free")
- Search icon (if search is a primary action)

**Moves into hamburger:**
- All navigation links
- Secondary CTA ("Log in")
- Utility links (theme toggle, language selector)

**Rules:**
- The primary CTA should never be hidden behind the hamburger. It loses 50%+ of engagement when buried (Baymard Institute).
- If the site has a critical action (e.g., "Emergency Contact" for a hospital), it stays visible.
- Search can stay as an icon that expands, or move into the hamburger. Depends on search importance.

### 4.3 Logo Behavior

- Desktop: Full logo (wordmark + icon) or wordmark only
- Mobile: Can reduce to icon-only if space is tight
- The logo always links to the homepage
- On scroll (sticky nav), the logo can shrink slightly (see section 5)

### 4.4 CTA Button Persistence

```css
/* CTA stays visible at all breakpoints */
.nav-cta {
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 600;
  border-radius: 0.375rem;
  background: var(--color-primary);
  color: white;
  text-decoration: none;
  white-space: nowrap;
}

/* Slightly smaller on mobile */
@media (max-width: 1023px) {
  .nav-cta {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
  }
}
```

---

## 5. Sticky / Fixed Navigation

### 5.1 Always Visible (position: sticky)

The simplest pattern. The nav sticks to the top and never hides.

```css
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
  background: var(--color-surface);
  /* GPU acceleration for smooth scrolling */
  will-change: transform;
}
```

**When to use:** Short pages, dashboard apps, sites where users need constant access to navigation.

**Concern:** On mobile, a sticky header consumes 50-64px of vertical space, which is 8-10% of a typical 667px viewport. For content-heavy pages, this tax matters.

### 5.2 Hide on Scroll Down, Show on Scroll Up

The most popular pattern for content sites and blogs. Maximizes reading area while keeping nav accessible.

```js
class SmartHeader {
  constructor(header) {
    this.header = header;
    this.lastScroll = 0;
    this.headerHeight = header.offsetHeight;
    this.scrollThreshold = 5; // Ignore micro-scrolls

    // Use transform for GPU-accelerated animation
    this.header.style.willChange = 'transform';
    this.header.style.transition = 'transform 300ms ease';

    window.addEventListener('scroll', () => this.onScroll(), { passive: true });
  }

  onScroll() {
    const currentScroll = window.scrollY;
    const delta = currentScroll - this.lastScroll;

    // Ignore micro-scrolls
    if (Math.abs(delta) < this.scrollThreshold) return;

    if (delta > 0 && currentScroll > this.headerHeight) {
      // Scrolling down — hide
      this.header.style.transform = `translateY(-100%)`;
      this.header.setAttribute('data-hidden', '');
    } else {
      // Scrolling up — show
      this.header.style.transform = 'translateY(0)';
      this.header.removeAttribute('data-hidden');
    }

    this.lastScroll = currentScroll;
  }
}
```

**Critical:** Always provide a way to get the nav back. If the user scrolls to the middle of a long page and the nav is hidden, any upward scroll (even small) must bring it back. Never require scrolling all the way to the top.

### 5.3 Compact on Scroll

The header shrinks its height and reduces the logo size after scrolling past a threshold. The nav remains visible at all times.

```css
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
  height: 5rem;
  transition: height 200ms ease, box-shadow 200ms ease;
}

.site-header.is-scrolled {
  height: 3.5rem;
  box-shadow: 0 1px 3px rgb(0 0 0 / 0.06);
}

.site-header .logo img {
  height: 2rem;
  transition: height 200ms ease;
}

.site-header.is-scrolled .logo img {
  height: 1.5rem;
}
```

```js
// Add .is-scrolled class after scrolling past threshold
const header = document.querySelector('.site-header');
const observer = new IntersectionObserver(
  ([entry]) => {
    header.classList.toggle('is-scrolled', !entry.isIntersecting);
  },
  { rootMargin: '-1px 0px 0px 0px', threshold: [1] }
);

// Observe a sentinel element at the top of the page
const sentinel = document.createElement('div');
sentinel.style.height = '1px';
document.body.prepend(sentinel);
observer.observe(sentinel);
```

Using `IntersectionObserver` instead of scroll events avoids the performance cost of continuous scroll listeners.

### 5.4 Performance Considerations

**Use `transform` for hide/show, not `top` or `margin-top`.** `transform` triggers only compositing (GPU), while `top` triggers layout (CPU). On low-powered devices, the difference is visible.

**Use `will-change: transform` sparingly.** It promotes the element to its own compositing layer. Apply it to the sticky header element only, not to dozens of elements.

**Use `{ passive: true }` on scroll listeners.** Tells the browser the handler will not call `preventDefault()`, enabling scroll optimizations.

**Prefer `IntersectionObserver` over scroll events** for threshold-based changes (like adding a shadow or compact class). It fires asynchronously and does not block the main thread.

```css
/* Performance-optimized sticky header */
.site-header {
  position: sticky;
  top: 0;
  z-index: 100;
  will-change: transform;
  /* Use transform for GPU-accelerated hide/show */
  transform: translateY(0);
  transition: transform 300ms cubic-bezier(0.32, 0.72, 0, 1);
}

.site-header[data-hidden] {
  transform: translateY(-100%);
}

/* Respect reduced motion */
@media (prefers-reduced-motion: reduce) {
  .site-header {
    transition-duration: 0ms;
  }
}
```

---

## 6. Navigation Hierarchy

### 6.1 Primary Navigation (5-7 Items Max)

The main horizontal bar. Contains the site's most important destinations.

NNGroup research confirms that navigation with fewer, well-labeled items outperforms navigation with many items. Miller's Law (7 plus/minus 2) is often cited, but the real constraint is not working memory capacity — users can see the items, they don't need to memorize them. The real constraint is **scan time** and **decision difficulty**. Hick's Law: decision time increases logarithmically with the number of options. Practically, 5-7 top-level items keep scan time under 2 seconds.

**Guidelines:**
- Labels should be 1-2 words (max 3)
- Use nouns or noun phrases, not verbs ("Products" not "Explore Products")
- Labels must be mutually exclusive — no overlap in what users expect to find under each
- Order by user priority, not organizational structure
- The first and last items get the most attention (serial position effect)

### 6.2 Secondary Navigation

Lives below the primary nav or in a sidebar. Contains section-specific links.

**Common patterns:**
- **Tab bar** below the main nav (e.g., "Overview | Features | Pricing | FAQ" on a product page)
- **Sidebar nav** for documentation or settings pages
- **Breadcrumbs** for deep hierarchies

```html
<nav aria-label="Product sections" class="secondary-nav">
  <ul role="list">
    <li><a href="/products/analytics" aria-current="page">Overview</a></li>
    <li><a href="/products/analytics/features">Features</a></li>
    <li><a href="/products/analytics/pricing">Pricing</a></li>
    <li><a href="/products/analytics/docs">Documentation</a></li>
  </ul>
</nav>
```

### 6.3 Utility Navigation

Small links/icons in the top-right area. Supplementary to the main navigation.

**Common items:**
- Search (icon or input)
- Theme toggle (light/dark)
- Language/locale selector
- Account/avatar dropdown
- Cart icon (e-commerce)
- Notification bell (apps)

**Rules:**
- Utility items should be visually distinct from primary navigation (smaller, different styling)
- Group them together, right-aligned
- On mobile, search and account may stay visible; others go into the hamburger

### 6.4 Handling 10+ Nav Items

When a site genuinely has many top-level destinations:

1. **Group into dropdowns.** Combine related items under 4-5 parent categories with dropdown menus.
2. **Use a mega menu.** If one category has many sub-items, a mega menu organizes them with headings and descriptions.
3. **Split into primary and secondary nav.** Move less-critical items to a secondary bar or footer nav.
4. **Use search.** For sites with 50+ pages, search becomes a primary navigation mechanism. Promote it in the nav bar.
5. **Never use horizontal scrolling nav.** On desktop, it hides items. On mobile, there is no visual affordance that more items exist.

---

## 7. Active State and Current Page Indication

### 7.1 Visual Current Page Indicator

The active nav item must be visually distinct from inactive items. Common treatments:

```css
/* Underline indicator */
.nav-links a[aria-current="page"] {
  color: var(--color-text-primary);
  box-shadow: inset 0 -2px 0 var(--color-primary);
}

/* Background highlight */
.nav-links a[aria-current="page"] {
  color: var(--color-primary);
  background: var(--color-primary-subtle);
  border-radius: 0.375rem;
}

/* Bold text + dot indicator */
.nav-links a[aria-current="page"] {
  font-weight: 700;
  color: var(--color-text-primary);
}
.nav-links a[aria-current="page"]::after {
  content: '';
  display: block;
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: var(--color-primary);
  margin: 0.25rem auto 0;
}
```

**Rules:**
- Do not rely on color alone. Use weight, underline, or background in addition to color change.
- The current page link should still look like a link (not disabled), but should not navigate to the same page on click. It can be a `<span>` instead of `<a>` or the `<a>` can have `aria-current="page"`.
- In a mobile drawer, the current page item should also be visually marked.

### 7.2 aria-current="page"

This is the correct way to indicate the current page to assistive technology. Screen readers announce it as "current page" alongside the link text.

```html
<nav aria-label="Main">
  <ul role="list">
    <li><a href="/">Home</a></li>
    <li><a href="/products" aria-current="page">Products</a></li>
    <li><a href="/pricing">Pricing</a></li>
  </ul>
</nav>
```

**Values for `aria-current`:**
- `page` — current page in a set of pages (most common in nav)
- `step` — current step in a multi-step process
- `location` — current location in a breadcrumb
- `date` — current date in a calendar
- `true` — generic "current item" (less specific)

**CSS can target it directly:**

```css
[aria-current="page"] {
  font-weight: 700;
  color: var(--color-primary);
}
```

### 7.3 Breadcrumbs

For sites with deep hierarchy (3+ levels), breadcrumbs show the user's location in the site structure.

```html
<nav aria-label="Breadcrumb">
  <ol class="breadcrumbs">
    <li><a href="/">Home</a></li>
    <li><a href="/products">Products</a></li>
    <li><a href="/products/analytics">Analytics</a></li>
    <li><span aria-current="page">Dashboard</span></li>
  </ol>
</nav>
```

```css
.breadcrumbs {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  list-style: none;
  padding: 0;
  margin: 0;
  font-size: 0.8125rem;
  color: var(--color-text-tertiary);
}

.breadcrumbs li:not(:last-child)::after {
  content: '/';
  margin-left: 0.25rem;
  color: var(--color-border);
}

.breadcrumbs a {
  color: var(--color-text-secondary);
  text-decoration: none;
}

.breadcrumbs a:hover {
  text-decoration: underline;
}

.breadcrumbs [aria-current] {
  color: var(--color-text-primary);
  font-weight: 500;
}
```

**Breadcrumb schema.org markup** for SEO:

```html
<nav aria-label="Breadcrumb">
  <ol class="breadcrumbs" itemscope itemtype="https://schema.org/BreadcrumbList">
    <li itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem">
      <a itemprop="item" href="/"><span itemprop="name">Home</span></a>
      <meta itemprop="position" content="1" />
    </li>
    <li itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem">
      <a itemprop="item" href="/products"><span itemprop="name">Products</span></a>
      <meta itemprop="position" content="2" />
    </li>
    <li itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem">
      <span itemprop="name" aria-current="page">Analytics</span>
      <meta itemprop="position" content="3" />
    </li>
  </ol>
</nav>
```

### 7.4 Section Highlighting

When the user is on a child page (e.g., `/products/analytics`), the parent nav item ("Products") should be highlighted to show which section they are in.

```js
// Highlight parent nav item based on URL path
const currentPath = window.location.pathname;
document.querySelectorAll('.nav-links a').forEach(link => {
  const href = link.getAttribute('href');
  if (href === currentPath) {
    link.setAttribute('aria-current', 'page');
  } else if (currentPath.startsWith(href) && href !== '/') {
    // Child page: highlight parent section
    link.classList.add('is-active-section');
  }
});
```

---

## 8. Search in Navigation

### 8.1 Search Icon Expanding to Input

A compact search icon that expands to a text input on click. Saves space in the nav bar while keeping search accessible.

```html
<div class="nav-search">
  <button
    type="button"
    class="nav-search-trigger"
    aria-label="Open search"
    aria-expanded="false"
    aria-controls="nav-search-input"
  >
    <svg aria-hidden="true"><!-- search icon --></svg>
  </button>

  <form class="nav-search-form" role="search" hidden>
    <label for="nav-search-input" class="sr-only">Search</label>
    <input
      type="search"
      id="nav-search-input"
      placeholder="Search..."
      autocomplete="off"
    />
    <button type="button" class="nav-search-close" aria-label="Close search">
      <svg aria-hidden="true"><!-- X icon --></svg>
    </button>
  </form>
</div>
```

```css
.nav-search-form {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav-search-form input {
  width: 0;
  padding: 0;
  border: none;
  background: transparent;
  font-size: 0.9375rem;
  outline: none;
  transition: width 200ms ease;
}

.nav-search-form:not([hidden]) input {
  width: 240px;
  padding: 0.375rem 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-surface);
}
```

### 8.2 Command Palette (Cmd+K / Ctrl+K)

A modal search/action interface popularized by VS Code, GitHub, Linear, Vercel, and Algolia DocSearch. Users press Cmd+K (Mac) or Ctrl+K (Windows/Linux) to open a centered modal with a search input.

**Characteristics:**
- Fuzzy search across pages, actions, and content
- Recently used items shown by default
- Keyboard-driven (arrow keys to navigate, Enter to select, Escape to close)
- Often shows keyboard shortcuts next to actions
- Can combine navigation and commands ("Go to pricing" + "Toggle dark mode")

```html
<div
  class="cmd-palette"
  role="dialog"
  aria-modal="true"
  aria-label="Search and commands"
  hidden
>
  <div class="cmd-palette-backdrop"></div>
  <div class="cmd-palette-panel">
    <div class="cmd-palette-header">
      <svg aria-hidden="true" class="cmd-palette-search-icon">
        <!-- search icon -->
      </svg>
      <input
        type="text"
        class="cmd-palette-input"
        placeholder="Search pages, actions..."
        aria-label="Search"
        autocomplete="off"
        spellcheck="false"
      />
      <kbd class="cmd-palette-hint">ESC</kbd>
    </div>
    <div class="cmd-palette-results" role="listbox" aria-label="Results">
      <div class="cmd-palette-group">
        <div class="cmd-palette-group-label" id="pages-label">Pages</div>
        <div role="option" aria-selected="true" class="cmd-palette-item">
          <span>Products</span>
          <span class="cmd-palette-path">/products</span>
        </div>
        <div role="option" class="cmd-palette-item">
          <span>Pricing</span>
          <span class="cmd-palette-path">/pricing</span>
        </div>
      </div>
      <div class="cmd-palette-group">
        <div class="cmd-palette-group-label">Actions</div>
        <div role="option" class="cmd-palette-item">
          <span>Toggle dark mode</span>
          <kbd>Ctrl+D</kbd>
        </div>
      </div>
    </div>
    <div class="cmd-palette-footer">
      <span><kbd>↑↓</kbd> Navigate</span>
      <span><kbd>↵</kbd> Select</span>
      <span><kbd>esc</kbd> Close</span>
    </div>
  </div>
</div>
```

```js
// Open command palette on Cmd+K / Ctrl+K
document.addEventListener('keydown', (e) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault();
    openCommandPalette();
  }
});

// Show hint in nav bar
// <button class="cmd-k-trigger">
//   <svg><!-- search icon --></svg>
//   <span>Search</span>
//   <kbd>⌘K</kbd>
// </button>
```

### 8.3 Search Overlay

A full-width search panel that drops down from the nav bar. Common on e-commerce sites and large content sites.

```css
.search-overlay {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  box-shadow: 0 8px 24px rgb(0 0 0 / 0.08);
  padding: 2rem;
  transform: translateY(-0.5rem);
  opacity: 0;
  visibility: hidden;
  transition: transform 200ms ease, opacity 200ms ease, visibility 200ms ease;
}

.search-overlay.is-open {
  transform: translateY(0);
  opacity: 1;
  visibility: visible;
}
```

---

## 9. Accessibility

### 9.1 Skip Navigation Link

The first focusable element on the page. Allows keyboard and screen reader users to bypass the navigation and jump directly to main content.

```html
<body>
  <a href="#main-content" class="skip-link">Skip to main content</a>

  <header><!-- navigation --></header>

  <main id="main-content" tabindex="-1">
    <!-- page content -->
  </main>
</body>
```

```css
.skip-link {
  position: absolute;
  top: -100%;
  left: 1rem;
  z-index: 1000;
  padding: 0.75rem 1.5rem;
  background: var(--color-primary);
  color: white;
  font-weight: 600;
  text-decoration: none;
  border-radius: 0 0 0.5rem 0.5rem;
  transition: top 150ms ease;
}

.skip-link:focus {
  top: 0;
}
```

**Notes:**
- The target element (`#main-content`) needs `tabindex="-1"` so it can receive focus programmatically. Without this, some browsers scroll to the target but do not move keyboard focus.
- The skip link must be the first focusable element on the page.
- Multiple skip links are acceptable: "Skip to main content", "Skip to search", "Skip to footer".

### 9.2 Keyboard-Only Navigation

Every interactive element in the navigation must be reachable and operable with keyboard alone.

**Requirements:**
- All links and buttons are focusable via Tab
- Focus order matches visual order (left-to-right, top-to-bottom)
- Focus indicators are visible (never `outline: none` without a replacement)
- Dropdowns open with Enter/Space/ArrowDown
- Dropdowns close with Escape
- Focus is trapped inside open mobile menus (see section 3.6)
- Focus returns to the trigger when a menu closes

```css
/* Visible focus indicator */
:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
  border-radius: 0.25rem;
}

/* Remove outline for mouse clicks (only show for keyboard) */
:focus:not(:focus-visible) {
  outline: none;
}
```

### 9.3 Screen Reader Announcements

- Use `aria-label` on `<nav>` elements to distinguish multiple landmarks: "Main navigation", "Footer navigation", "Breadcrumb".
- Use `aria-expanded` on dropdown triggers to announce open/close state.
- Use `aria-current="page"` on the active link (screen readers announce "current page").
- Use `aria-haspopup="true"` on buttons that open dropdown menus.
- Do not use `aria-live` on menu open/close — the focus movement is sufficient announcement.

### 9.4 Focus Management

**Opening a dropdown (desktop):** Focus moves to the first item in the dropdown.
**Closing a dropdown:** Focus returns to the trigger button.
**Opening mobile menu:** Focus moves to the close button or first focusable element inside.
**Closing mobile menu:** Focus returns to the hamburger button.
**Focus trap:** While a mobile menu (dialog) is open, Tab/Shift+Tab must cycle only through elements inside the menu. Focus must not escape to elements behind the menu.

```js
// Restore focus on close — critical for accessibility
close() {
  this.menu.hidden = true;
  this.trigger.setAttribute('aria-expanded', 'false');
  // MUST restore focus to the element that opened the menu
  this.previouslyFocusedElement?.focus();
}
```

### 9.5 Reduced Motion

Users who set "Reduce motion" in their OS preferences should not see animated nav transitions (slide, fade, bounce). Respect `prefers-reduced-motion`.

```css
@media (prefers-reduced-motion: reduce) {
  .drawer-panel,
  .drawer-backdrop,
  .mega-menu,
  .dropdown,
  .site-header,
  .fullscreen-menu,
  .search-overlay {
    transition-duration: 0ms !important;
    animation-duration: 0ms !important;
  }
}
```

**In JavaScript,** check before animating:

```js
const prefersReducedMotion = window.matchMedia(
  '(prefers-reduced-motion: reduce)'
).matches;

function openDrawer() {
  if (prefersReducedMotion) {
    // Instant show, no animation
    panel.style.transition = 'none';
  }
  panel.hidden = false;
}
```

### 9.6 High Contrast and Forced Colors

Windows High Contrast Mode strips custom backgrounds and shadows. Navigation items can become invisible.

```css
@media (forced-colors: active) {
  .nav-links a[aria-current="page"] {
    border-bottom: 2px solid LinkText;
  }

  .hamburger-line {
    background: ButtonText;
  }

  .dropdown,
  .mega-menu {
    border: 1px solid ButtonText;
  }
}
```

---

## 10. Framework-Specific Patterns

### 10.1 Astro

Astro ships zero JavaScript by default. Navigation that needs interactivity (dropdowns, mobile menu) must use `is:inline` scripts, client-side framework islands, or vanilla JS with `<script>` tags.

**Vanilla JS nav (no framework dependency):**

```astro
---
// src/components/Nav.astro
interface Props {
  currentPath: string;
}

const { currentPath } = Astro.props;

const links = [
  { href: '/products', label: 'Products' },
  { href: '/pricing', label: 'Pricing' },
  { href: '/docs', label: 'Docs' },
  { href: '/blog', label: 'Blog' },
];
---

<header class="site-header">
  <nav aria-label="Main">
    <a href="/" class="logo">
      <img src="/logo.svg" alt="Acme" width="120" height="32" />
    </a>

    <ul class="nav-links" role="list">
      {links.map(link => (
        <li>
          <a
            href={link.href}
            aria-current={currentPath.startsWith(link.href) ? 'page' : undefined}
          >
            {link.label}
          </a>
        </li>
      ))}
    </ul>

    <div class="nav-utility">
      <a href="/signup" class="nav-cta">Get Started</a>

      <button
        type="button"
        class="hamburger"
        aria-expanded="false"
        aria-controls="mobile-menu"
        aria-label="Open menu"
      >
        <span class="hamburger-line" />
        <span class="hamburger-line" />
        <span class="hamburger-line" />
      </button>
    </div>
  </nav>
</header>

<div
  id="mobile-menu"
  class="mobile-drawer"
  role="dialog"
  aria-modal="true"
  aria-label="Site navigation"
  hidden
>
  <div class="drawer-backdrop"></div>
  <div class="drawer-panel">
    <button type="button" class="drawer-close" aria-label="Close menu">
      &times;
    </button>
    <nav aria-label="Main">
      <ul role="list" class="mobile-nav">
        {links.map(link => (
          <li>
            <a
              href={link.href}
              aria-current={currentPath.startsWith(link.href) ? 'page' : undefined}
            >
              {link.label}
            </a>
          </li>
        ))}
      </ul>
    </nav>
    <div class="drawer-footer">
      <a href="/login" class="btn-secondary btn-full">Log in</a>
      <a href="/signup" class="btn-primary btn-full">Get Started</a>
    </div>
  </div>
</div>

<script is:inline>
  // Mobile menu toggle — is:inline means no bundler, runs as-is
  (function() {
    const hamburger = document.querySelector('.hamburger');
    const menu = document.getElementById('mobile-menu');
    const closeBtn = menu.querySelector('.drawer-close');
    const backdrop = menu.querySelector('.drawer-backdrop');
    let previousFocus = null;

    function open() {
      previousFocus = document.activeElement;
      menu.hidden = false;
      hamburger.setAttribute('aria-expanded', 'true');
      document.body.style.overflow = 'hidden';
      requestAnimationFrame(() => closeBtn.focus());
    }

    function close() {
      menu.hidden = true;
      hamburger.setAttribute('aria-expanded', 'false');
      document.body.style.overflow = '';
      previousFocus?.focus();
    }

    hamburger.addEventListener('click', open);
    closeBtn.addEventListener('click', close);
    backdrop.addEventListener('click', close);
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && !menu.hidden) close();
    });
  })();
</script>
```

**Using the `currentPath` in layouts:**

```astro
---
// src/layouts/BaseLayout.astro
import Nav from '../components/Nav.astro';

const currentPath = Astro.url.pathname;
---

<html>
  <body>
    <a href="#main-content" class="skip-link">Skip to main content</a>
    <Nav currentPath={currentPath} />
    <main id="main-content" tabindex="-1">
      <slot />
    </main>
  </body>
</html>
```

**With view transitions:** When using Astro's View Transitions, the mobile menu script runs once on initial load. To re-initialize after navigation, listen for `astro:page-load`:

```astro
<script is:inline>
  document.addEventListener('astro:page-load', () => {
    // Re-bind all event listeners
    // Or: close the mobile menu if it's open
    const menu = document.getElementById('mobile-menu');
    if (menu && !menu.hidden) {
      menu.hidden = true;
      document.body.style.overflow = '';
    }
  });
</script>
```

### 10.2 React (Headless UI)

Headless UI (from the Tailwind CSS team) provides unstyled, accessible components including `Menu`, `Popover`, and `Dialog`.

```tsx
// React + Headless UI navigation with dropdown
import { Popover, PopoverButton, PopoverPanel } from '@headlessui/react';
import { ChevronDownIcon } from '@heroicons/react/20/solid';

function NavDropdown({ label, items }) {
  return (
    <Popover className="relative">
      <PopoverButton className="nav-link flex items-center gap-1">
        {label}
        <ChevronDownIcon className="h-4 w-4" aria-hidden="true" />
      </PopoverButton>

      <PopoverPanel
        anchor="bottom start"
        className="dropdown mt-2"
        transition
      >
        <ul role="list">
          {items.map(item => (
            <li key={item.href}>
              <a href={item.href} className="dropdown-link">
                {item.icon && <item.icon className="h-5 w-5" aria-hidden="true" />}
                <div>
                  <strong>{item.label}</strong>
                  {item.description && <small>{item.description}</small>}
                </div>
              </a>
            </li>
          ))}
        </ul>
      </PopoverPanel>
    </Popover>
  );
}

// Mobile menu using Dialog
import { Dialog, DialogPanel } from '@headlessui/react';

function MobileMenu({ isOpen, onClose }) {
  return (
    <Dialog open={isOpen} onClose={onClose} className="mobile-drawer">
      <div className="drawer-backdrop" aria-hidden="true" />
      <DialogPanel className="drawer-panel">
        <button onClick={onClose} className="drawer-close" aria-label="Close menu">
          &times;
        </button>
        {/* Nav links */}
      </DialogPanel>
    </Dialog>
  );
}
```

**Why Headless UI:** It handles `aria-expanded`, focus trapping, keyboard navigation, click-outside-to-close, and Escape-to-close out of the box. You only write styles.

### 10.3 Vue (Transition Groups)

```vue
<template>
  <nav aria-label="Main" class="site-header">
    <div class="nav-item" v-for="item in navItems" :key="item.label">
      <button
        v-if="item.children"
        @click="toggleDropdown(item.label)"
        :aria-expanded="String(openDropdown === item.label)"
        aria-haspopup="true"
        class="nav-link"
      >
        {{ item.label }}
        <ChevronIcon :class="{ 'is-rotated': openDropdown === item.label }" />
      </button>
      <a v-else :href="item.href" class="nav-link">{{ item.label }}</a>

      <Transition name="dropdown">
        <div
          v-if="item.children && openDropdown === item.label"
          class="dropdown"
        >
          <ul role="list">
            <li v-for="child in item.children" :key="child.href">
              <a :href="child.href">{{ child.label }}</a>
            </li>
          </ul>
        </div>
      </Transition>
    </div>
  </nav>
</template>

<script setup>
import { ref } from 'vue';

const openDropdown = ref(null);

function toggleDropdown(label) {
  openDropdown.value = openDropdown.value === label ? null : label;
}
</script>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 200ms ease, transform 200ms ease;
}
.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-0.5rem);
}
</style>
```

### 10.4 Tailwind CSS Patterns

Tailwind utility classes for common nav patterns:

```html
<!-- Horizontal nav bar -->
<header class="sticky top-0 z-50 bg-white border-b border-gray-200 dark:bg-gray-900 dark:border-gray-800">
  <nav class="mx-auto flex max-w-7xl items-center justify-between px-6 h-16" aria-label="Main">
    <a href="/" class="flex-shrink-0">
      <img src="/logo.svg" alt="Acme" class="h-8 w-auto" />
    </a>

    <!-- Desktop links -->
    <ul class="hidden lg:flex lg:gap-x-8" role="list">
      <li><a href="/products" class="text-sm font-semibold text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white">Products</a></li>
      <li><a href="/pricing" class="text-sm font-semibold text-gray-600 hover:text-gray-900">Pricing</a></li>
      <li><a href="/docs" class="text-sm font-semibold text-gray-600 hover:text-gray-900">Docs</a></li>
    </ul>

    <div class="flex items-center gap-4">
      <a href="/signup" class="rounded-md bg-indigo-600 px-3.5 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500">
        Get Started
      </a>

      <!-- Hamburger (mobile only) -->
      <button
        type="button"
        class="lg:hidden -m-2 p-2 text-gray-700"
        aria-label="Open menu"
      >
        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
        </svg>
      </button>
    </div>
  </nav>
</header>
```

---

## 11. Anti-Patterns

### 11.1 Hover-Only Dropdowns (No Click Fallback)

**Problem:** Users on touch devices, keyboard-only users, and screen reader users cannot access the dropdown content. On desktop, hover-only menus trigger accidentally during normal mouse movement.

**Fix:** Always use click as the primary trigger. Hover can be a progressive enhancement on devices that support it.

### 11.2 No Keyboard Support

**Problem:** Dropdown menus that only respond to mouse clicks, with no Enter/Space/Arrow key handling. Users cannot navigate with keyboard alone.

**Fix:** Implement the full keyboard interaction table (section 2.4). Test by unplugging the mouse and navigating the entire site.

### 11.3 Hamburger Menu on Desktop

**Problem:** Hiding navigation behind a hamburger icon on desktop screens where there is ample space. Users must click to discover what the site offers. NNGroup research shows hidden navigation reduces discoverability by 21% compared to visible navigation.

**Fix:** Reserve the hamburger for mobile/tablet breakpoints only. On desktop, show the full horizontal nav bar.

### 11.4 Disappearing Nav with No Recovery

**Problem:** The sticky nav hides on scroll down, but never reappears until the user scrolls all the way to the top. On a 10,000-word article, this means the user loses access to navigation.

**Fix:** Any upward scroll (even a few pixels) should bring the nav back. See section 5.2.

### 11.5 The Triangle Problem (Diagonal Submenu Closure)

**Problem:** A dropdown menu with sub-items (flyout) closes when the user moves their mouse diagonally from the parent item to the submenu. The cursor briefly leaves the parent element, triggering `mouseleave`, and the submenu vanishes.

This was famously solved by Amazon's menu team and documented by Ben Kamens. The solution uses a "safe triangle" — an invisible triangular hit area connecting the cursor position to the submenu edges.

**Solution using CSS `clip-path`:**

```css
/* Parent menu item with submenu */
.flyout-parent {
  position: relative;
}

.flyout-submenu {
  position: absolute;
  left: 100%;
  top: 0;
  opacity: 0;
  visibility: hidden;
  pointer-events: none;
  transition: opacity 150ms ease;
}

.flyout-parent:hover > .flyout-submenu,
.flyout-parent:focus-within > .flyout-submenu {
  opacity: 1;
  visibility: visible;
  pointer-events: auto;
}

/* Invisible bridge element preventing diagonal gap */
.flyout-submenu::before {
  content: '';
  position: absolute;
  top: 0;
  right: 100%;
  width: 3rem;
  height: 100%;
  /* This transparent rectangle covers the gap between parent and submenu */
}
```

**Solution using JavaScript hover intent:**

```js
class SafeSubmenu {
  constructor(parentItem, submenu) {
    this.parent = parentItem;
    this.submenu = submenu;
    this.closeTimer = null;
    this.mousePos = { x: 0, y: 0 };

    this.parent.addEventListener('mouseenter', () => {
      clearTimeout(this.closeTimer);
      this.show();
    });

    this.parent.addEventListener('mouseleave', (e) => {
      // Check if mouse is moving toward the submenu
      const submenuRect = this.submenu.getBoundingClientRect();
      const movingToward = this.isMovingToward(e, submenuRect);

      if (movingToward) {
        // Grace period — user is likely heading to submenu
        this.closeTimer = setTimeout(() => this.hide(), 400);
      } else {
        this.closeTimer = setTimeout(() => this.hide(), 100);
      }
    });

    this.submenu.addEventListener('mouseenter', () => {
      clearTimeout(this.closeTimer);
    });

    this.submenu.addEventListener('mouseleave', () => {
      this.closeTimer = setTimeout(() => this.hide(), 200);
    });
  }

  isMovingToward(event, targetRect) {
    const dx = event.clientX - this.mousePos.x;
    const dy = event.clientY - this.mousePos.y;

    // If submenu is to the right and mouse is moving right
    if (targetRect.left > event.clientX && dx > 0) return true;
    // If submenu is to the left and mouse is moving left
    if (targetRect.right < event.clientX && dx < 0) return true;

    return false;
  }

  show() {
    this.submenu.hidden = false;
  }

  hide() {
    this.submenu.hidden = true;
  }
}
```

**Adobe React Spectrum's approach** (documented on their blog): They compute a "safe zone" triangle in real time using the cursor position and the submenu bounds, and check whether each subsequent mouse position falls within that triangle. If it does, the close is deferred; if the mouse leaves the triangle, the submenu closes immediately.

### 11.6 Dropdown Covering the Trigger

**Problem:** The dropdown panel opens directly on top of the trigger button, making it impossible to close the dropdown by clicking the trigger again.

**Fix:** Position the dropdown below the trigger with a small gap (`top: calc(100% + 0.5rem)`). Ensure the trigger remains clickable to toggle the dropdown closed.

### 11.7 Slow or Janky Animations

**Problem:** Navigation transitions that use `height: auto` animations, large box shadows recalculating on every frame, or JavaScript-driven animations that block the main thread.

**Fix:** Animate only `transform` and `opacity` (GPU-composited properties). Use `max-height` with a large value as a fallback for height animations. Prefer CSS transitions over JavaScript animation loops.

### 11.8 Navigation That Requires JavaScript to Be Visible

**Problem:** The nav links are hidden by default and only shown by JavaScript, meaning users with JS disabled or slow connections see nothing.

**Fix:** Build the nav with progressive enhancement. The HTML should display all links by default. CSS hides the mobile menu. JavaScript adds interactivity (toggle, dropdowns). If JS fails, all links remain visible.

```css
/* Progressive enhancement: show all links by default */
.mobile-nav {
  display: block;
}

/* JS adds this class to enable toggle behavior */
.js .mobile-nav {
  display: none;
}

.js .hamburger {
  display: flex;
}
```

```html
<script is:inline>
  document.documentElement.classList.add('js');
</script>
```

---

## 12. Audit Dimensions

Use these dimensions to evaluate the quality of any navigation implementation. Score each 0-2 (0 = missing, 1 = partial, 2 = complete).

### 12.1 Structure and Information Architecture

| Dimension | What to check |
|-----------|---------------|
| Item count | 5-7 top-level items (not 10+) |
| Label clarity | 1-2 word labels, no jargon, mutually exclusive categories |
| Logical grouping | Related items grouped under dropdown parents |
| Hierarchy depth | Max 2 levels (3 is a red flag) |
| Consistency | Same nav structure across all pages |
| Logo placement | Top-left, links to homepage |

### 12.2 Responsive Behavior

| Dimension | What to check |
|-----------|---------------|
| Breakpoint transition | Clean switch from desktop to mobile nav |
| Mobile menu type | Drawer, overlay, or bottom sheet (not a squished desktop nav) |
| Touch targets | 44x44px minimum on mobile |
| CTA visibility | Primary CTA visible at all breakpoints |
| Logo adaptation | Appropriate sizing for viewport |
| Orientation handling | Nav works in both portrait and landscape |

### 12.3 Interaction Quality

| Dimension | What to check |
|-----------|---------------|
| Click triggers | All dropdowns open on click (not hover-only) |
| Hover delay | 200-300ms delay before hover-open (if hover is used) |
| Close grace period | 300-500ms before hover-close |
| Outside click | Clicking outside dropdown closes it |
| Escape key | Closes any open dropdown/menu |
| Triangle handling | Diagonal mouse movement to submenu does not close parent |
| Animation quality | Smooth (transform/opacity only), under 300ms |

### 12.4 Accessibility

| Dimension | What to check |
|-----------|---------------|
| Skip link | Skip to main content link present and functional |
| Keyboard navigation | All items reachable via Tab, Enter, Space, Arrow keys |
| Focus indicators | Visible focus outlines on all interactive elements |
| Focus trapping | Focus trapped inside open mobile menu |
| Focus restoration | Focus returns to trigger on menu close |
| `aria-expanded` | Present on all dropdown triggers |
| `aria-current="page"` | Present on active navigation link |
| `aria-label` on `<nav>` | Multiple nav landmarks are distinguishable |
| Screen reader flow | Logical reading order, no confusing announcements |
| Reduced motion | Animations respect `prefers-reduced-motion` |
| High contrast | Navigation visible in Windows High Contrast Mode |

### 12.5 Performance

| Dimension | What to check |
|-----------|---------------|
| No layout thrashing | Animations use `transform`/`opacity`, not `top`/`height`/`margin` |
| Passive scroll listeners | `{ passive: true }` on scroll event handlers |
| `will-change` usage | Applied only to elements that animate, not globally |
| No CLS | Nav does not shift content on load (proper height reserved) |
| IntersectionObserver | Used instead of scroll listeners for threshold-based changes |
| JS bundle size | Nav interactivity in < 5KB gzipped (vanilla JS achieves this easily) |
| Progressive enhancement | Nav links visible without JavaScript |

### 12.6 Visual Design

| Dimension | What to check |
|-----------|---------------|
| Contrast | Text meets WCAG AA (4.5:1 normal, 3:1 large) |
| Active state | Current page clearly indicated (not color-only) |
| Hover state | Visible hover effect on all interactive elements |
| Alignment | Items vertically centered, consistent spacing |
| Typography | Legible size (14-16px), appropriate weight hierarchy |
| Dark mode | Nav adapts correctly to dark color scheme |
| Z-index management | Nav overlaps content correctly, no z-index conflicts |

### 12.7 SEO and Semantics

| Dimension | What to check |
|-----------|---------------|
| Semantic HTML | Uses `<nav>`, `<ul>`, `<li>`, `<a>` (not divs with click handlers) |
| Breadcrumb schema | BreadcrumbList JSON-LD or microdata present |
| Crawlable links | All nav links are real `<a href>` (not JS-only navigation) |
| Consistent URL structure | Nav links match the site's URL hierarchy |
| No duplicate nav | Single primary nav, not two competing sets of links |

---

## 13. Reference Design Systems

### 13.1 GOV.UK Design System

GOV.UK separates the global **Header** component (showing the GOV.UK logo and sitewide tools) from the **Service Navigation** component (showing service name and section links). This separation enforces a clear hierarchy: government branding is never mixed with service-specific navigation. The Service Navigation supports slots for custom elements like language selectors, and collapses into a hamburger toggle on mobile. Updated in June 2025 for the refreshed GOV.UK branding.

**Key takeaway:** Separate your brand/org header from your product navigation. They have different purposes and different audiences.

### 13.2 Material Design 3

Material Design 3 defines the **Navigation Bar** (bottom bar) for mobile and the **Navigation Rail** for tablets. In M3 Expressive (2025), the classic bottom bar was replaced by the **Flexible Navigation Bar** with shorter height and horizontal label layout on wider screens. Maximum 5 destinations in a bottom nav. Each destination is an icon + label pair. The navigation bar should not contain menus or dialogs — destinations only.

**Key takeaway:** Mobile navigation should be at the bottom of the screen where thumbs can reach it. Limit to 5 destinations with clear icons and labels.

### 13.3 Apple Human Interface Guidelines

Apple HIG specifies 44pt minimum touch targets, recommends tab bars for iOS apps (5 items max), and emphasizes that navigation should feel "invisible" — users should focus on content, not controls. For web, the HIG's guidance on clarity, deference, and depth applies: navigation should be visually quiet, with clear labels and predictable behavior.

### 13.4 Vercel / Next.js

Vercel's site popularized the compact, monochrome nav bar with: minimal items (4-5), a prominent Cmd+K search trigger, the primary CTA always visible, and a clean hamburger-to-drawer transition on mobile. No mega menus. No hover dropdowns. Click-only interactions.

**Key takeaway:** For developer-focused products, simplicity wins. A search-forward nav with fewer items outperforms a complex mega menu.

---

## Sources

- [W3C WAI ARIA APG — Menu and Menubar Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/)
- [W3C WAI ARIA APG — Keyboard Interface](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/)
- [W3C WAI ARIA APG — Menu Button Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menu-button/)
- [W3C WAI ARIA APG — Breadcrumb Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/breadcrumb/)
- [MDN — ARIA menu role](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/menu_role)
- [MDN — aria-current attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-current)
- [NNGroup — Minimize Cognitive Load](https://www.nngroup.com/articles/minimize-cognitive-load/)
- [NNGroup — 4 Dangerous Navigation Approaches](https://www.nngroup.com/articles/navigation-cognitive-strain/)
- [NNGroup — Breakpoints in Responsive Design](https://www.nngroup.com/articles/breakpoints-in-responsive-design/)
- [Smashing Magazine — Better Context Menus With Safe Triangles](https://www.smashingmagazine.com/2023/08/better-context-menus-safe-triangles/)
- [GOV.UK Design System — Header](https://design-system.service.gov.uk/components/header/)
- [GOV.UK Design System — Service Navigation](https://design-system.service.gov.uk/components/service-navigation/)
- [GOV.UK Design System — Navigate a Service](https://design-system.service.gov.uk/patterns/navigate-a-service/)
- [Material Design 3 — Navigation Bar Guidelines](https://m3.material.io/components/navigation-bar/guidelines)
- [Level Access — Accessible Navigation Menus](https://www.levelaccess.com/blog/accessible-navigation-menus-pitfalls-and-best-practices/)
- [Level Access — Challenges with Mega Menus](https://www.levelaccess.com/blog/challenges-mega-menus-standard-menus-make-accessible/)
- [A11Y Collective — Accessible Mega Menu](https://www.a11y-collective.com/blog/accessible-mega-menu/)
- [A11Y Collective — aria-current](https://www.a11y-collective.com/blog/aria-current/)
- [WebAIM — Skip Navigation Links](https://webaim.org/techniques/skipnav/)
- [CSS-Tricks — Dropdown Menus with Forgiving Mouse Movement](https://css-tricks.com/dropdown-menus-with-more-forgiving-mouse-movement-paths/)
- [CSS-Tricks — Sticky, Smooth, Active Nav](https://css-tricks.com/sticky-smooth-active-nav/)
- [CSS-Tricks — prefers-reduced-motion](https://css-tricks.com/almanac/rules/m/media/prefers-reduced-motion/)
- [React Spectrum Blog — Pointer-Friendly Submenu Experience](https://react-spectrum.adobe.com/blog/creating-a-pointer-friendly-submenu-experience.html)
- [Webflow — Navigation Bar Design Best Practices](https://webflow.com/blog/navigation-bar-design)
- [BrowserStack — Responsive Design Breakpoints 2025](https://www.browserstack.com/guide/responsive-design-breakpoints)
- [Knowbility — Accessible Slide-Out Menus](https://knowbility.org/blog/2020/accessible-slide-menus)
- [Maggie Appleton — Command K Bars](https://maggieappleton.com/command-bar)
- [Aditus — aria-current Best Practices](https://www.aditus.io/aria/aria-current/)
- [Astro Docs — Script & Event Handling](https://docs.astro.build/en/guides/view-transitions/)
- [astro-navbar — Headless Nav for Astro](https://github.com/surjithctly/astro-navbar)
