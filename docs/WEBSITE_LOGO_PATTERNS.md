# Website Logo Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Google Search Central, Apple HIG, NNg, MDN, WCAG 2.2, Andrey Sitnik favicon research, production site analysis (Stripe, GitHub, Vercel, Linear)
**Last updated:** 2026-03-27

## 1. Header Logo Sizing

| Context | Height | Format | Notes |
|---------|--------|--------|-------|
| Desktop header | 32–40px | Inline SVG | Full wordmark + icon |
| Mobile header | 28–32px | Inline SVG | Icon-only (wordmark illegible below 20px) |
| Sticky/scrolled | 24–32px | Inline SVG | Optional shrink on scroll |
| Footer | 24–32px | Inline SVG or img | Smaller, less prominent |

**Minimum clickable area:** 44x44px (Apple HIG / WCAG 2.5.8). Pad the `<a>` wrapper even if the visual logo is smaller.

**Reference sizes from production sites:** Stripe 32px, GitHub 32px, Vercel 26px, Linear 28px.

## 2. Responsive Logo

- **Desktop (>768px):** Icon + wordmark (horizontal lockup)
- **Mobile (<768px):** Icon-only — do NOT scale down the wordmark; it becomes illegible

```html
<!-- Full logo on desktop, icon-only on mobile -->
<a href="/" class="flex items-center gap-2">
  <img src="/logo-icon.svg" alt="CruxDev" class="h-8 w-8" />
  <span class="hidden md:inline text-xl font-bold">CruxDev</span>
</a>
```

## 3. Favicon — Complete Set (5 Files)

Every website needs exactly these 5 favicon files:

| File | Size | Format | Purpose |
|------|------|--------|---------|
| `favicon.ico` | 32x32 | ICO | Legacy browsers, `/favicon.ico` root fallback |
| `favicon.svg` | scalable | SVG | Modern browsers, dark mode support via embedded CSS |
| `apple-touch-icon.png` | 180x180 | PNG (solid bg) | iOS home screen |
| `icon-192.png` | 192x192 | PNG | PWA manifest |
| `icon-512.png` | 512x512 | PNG | PWA manifest, splash screens |

### HTML markup (in `<head>`):
```html
<link rel="icon" href="/favicon.ico" sizes="32x32">
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<link rel="manifest" href="/manifest.webmanifest">
```

### SVG favicon with dark mode:
```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
  <style>
    path { fill: #094CA2; }
    @media (prefers-color-scheme: dark) {
      path { fill: #5496E8; }
    }
  </style>
  <path d="..."/>
</svg>
```

### Apple touch icon:
- **No transparency** — iOS fills transparent areas with black
- Use the logo on a solid background (brand color or white)
- Place at `/apple-touch-icon.png` (iOS checks this path directly)

### Google Search favicon requirements:
- Must be a multiple of 48x48px
- Must be "visually representative of your brand"
- URL must be crawlable (not blocked by robots.txt)
- Displayed as small as 16x16 in search results

### What you DON'T need:
- 16x16 standalone PNG (covered by .ico and .svg)
- 96x96, 128x128, 144x144 PNGs (legacy, handled by manifest)
- Multiple Apple touch icon sizes (180x180 covers all current devices)

## 4. Dark/Light Mode Logo

### Best approach: SVG with `currentColor`
```html
<svg fill="currentColor" ...>
  <path d="..."/>
</svg>
```
Logo automatically inherits text color — no image swapping, no flicker.

### If logo uses multiple colors:
```html
<!-- Light mode logo -->
<img src="/logo-light.svg" class="dark:hidden" alt="CruxDev" />
<!-- Dark mode logo -->
<img src="/logo-dark.svg" class="hidden dark:block" alt="CruxDev" />
```

### Do NOT:
- Use `filter: invert(1)` — produces imprecise colors
- Use transparent-background logos without checking contrast in both modes

## 5. Accessibility

- **Alt text:** `alt="CruxDev"` or `alt="CruxDev — home"`. Never `alt="logo"`.
- **Link to home:** Logo MUST link to `/`. Universal user expectation (NNg: >90% expect it).
- **Focus indicator:** Logo link must have visible focus ring for keyboard navigation.
- **Inline SVG:** Add `role="img"` and `aria-label="CruxDev"` to the SVG element.
- **Logo contrast:** Exempt from WCAG SC 1.4.3, but good practice to ensure visibility in both modes.

## 6. Performance

- **SVG preferred** — resolution-independent, 1–5KB, CSS-styleable
- **Do NOT lazy-load** — logo is above the fold, critical to perceived page load
- **Inline SVG** eliminates a network request entirely
- **Preload if external:** `<link rel="preload" href="/logo.svg" as="image" type="image/svg+xml">`

## 7. Common Mistakes

| Mistake | Fix |
|---------|-----|
| Logo too small on mobile | Use icon-only variant, not scaled-down wordmark |
| Default framework favicon (Astro rocket, Next.js, etc.) | Replace with actual brand favicon before any public launch |
| Missing favicon.ico | Some tools/crawlers check `/favicon.ico` directly |
| Transparent Apple touch icon | iOS fills with black — use solid background |
| Dark logo invisible in dark mode tabs | Use SVG favicon with embedded dark mode CSS |
| `alt="logo"` | Use company name: `alt="CruxDev"` |
| Logo not clickable | Must link to home page |
| Using PNG for header logo | Blurry on Retina — use SVG |
| Missing manifest.webmanifest | PWA "Add to Home Screen" has no icon |

## 8. Audit Dimensions

For convergence engine integration — audit logos against:

1. **sizing** — Header logo 32–40px desktop, 28–32px mobile, 44x44 min click target
2. **favicon_set** — All 5 files present (ico, svg, apple-touch, 192, 512)
3. **dark_mode** — Logo visible in both modes (currentColor or dual variants)
4. **accessibility** — Alt text, home link, focus indicator
5. **performance** — SVG format, not lazy-loaded, appropriately sized
