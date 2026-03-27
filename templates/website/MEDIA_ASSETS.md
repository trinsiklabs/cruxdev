---
title: Image & Media Asset Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Image & Media Asset Specification

> All visual and media assets needed for the website: photography, illustrations, screenshots, video, icons.

## 1. Asset Inventory

### 1.1 Images Needed

| # | Asset | Type | Page(s) | Dimensions | Format | Status |
|---|---|---|---|---|---|---|
| 1 | [Hero image] | Photo/Illustration | Homepage | 1920x1080 | WebP | [Needed/In-progress/Ready] |
| 2 | [Product screenshot — dashboard] | Screenshot | Features | 1200x800 | WebP | [Status] |
| 3 | [Team photo — founder] | Headshot | About | 400x400 | WebP | [Status] |
| 4 | [Customer logo — Company A] | Logo | Homepage, Pricing | SVG | SVG | [Status] |
| 5 | [Blog featured image template] | Template | Blog | 1200x630 | WebP | [Status] |
| 6 | [OG image — default] | Social card | All pages (fallback) | 1200x630 | PNG/WebP | [Status] |

### 1.2 Videos Needed

| # | Asset | Duration | Page(s) | Hosting | Status |
|---|---|---|---|---|---|
| 1 | [Product demo video] | [e.g., "2 minutes"] | Homepage, Features | [YouTube/Vimeo/Self-hosted] | [Status] |
| 2 | [Explainer animation] | [e.g., "60 seconds"] | Homepage | [Hosting] | [Status] |

### 1.3 Icons Needed

| Icon Set | Source | Count | Format | Usage |
|---|---|---|---|---|
| [e.g., "Feature icons"] | [e.g., "Lucide / custom"] | [e.g., 12] | SVG | [Feature cards, benefits sections] |
| [e.g., "Social icons"] | [e.g., "Simple Icons"] | [e.g., 5] | SVG | [Footer social links] |

---

## 2. Image Specifications

### 2.1 Format & Size Guidelines

| Image Type | Format | Max File Size | Responsive Srcset |
|---|---|---|---|
| Hero / banner | WebP (AVIF if supported) | 200KB | 480w, 768w, 1200w, 1920w |
| Content images | WebP | 100KB | 480w, 768w, 1200w |
| Thumbnails | WebP | 50KB | 300w, 600w |
| Logos | SVG (PNG fallback) | 20KB | N/A |
| Icons | Inline SVG | 5KB | N/A |
| OG / social images | PNG or WebP | 200KB | N/A (fixed 1200x630) |

### 2.2 Optimization Requirements

- [ ] All images served in modern formats (WebP minimum, AVIF preferred)
- [ ] Responsive srcset with appropriate breakpoints
- [ ] Lazy loading for all images below the fold (`loading="lazy"`)
- [ ] Explicit width and height attributes to prevent CLS
- [ ] Alt text on every image (see section 3)
- [ ] EXIF data stripped (privacy, file size)
- [ ] Color profile: sRGB

---

## 3. Alt Text Requirements

| Image Type | Alt Text Pattern | Example |
|---|---|---|
| Product screenshot | Describe what the user sees | "Dashboard showing deployment pipeline with three stages: build, test, deploy" |
| Team headshot | Person name and role | "Jane Smith, CTO and co-founder" |
| Decorative illustration | Empty alt="" | `alt=""` (screen readers skip it) |
| Logo | Company name | "Acme Corp logo" |
| Chart / diagram | Describe the data or relationship | "Bar chart showing 80% reduction in deploy time after switching to [Product]" |
| Icon (with label) | Empty alt="" (label provides context) | `alt=""` |
| Icon (no label) | Describe the meaning | "Checkmark — feature included" |

---

## 4. Photography Guidelines

| Guideline | Specification |
|---|---|
| Style | [e.g., "Natural, candid, not stock-photo. Real people in real contexts."] |
| Lighting | [e.g., "Natural lighting preferred, warm tones"] |
| Subjects | [e.g., "Diverse representation across age, gender, ethnicity"] |
| Backgrounds | [e.g., "Clean, uncluttered, on-brand colors"] |
| Source | [e.g., "Custom photography / Unsplash / specific stock provider"] |
| License | [e.g., "Unsplash License / purchased royalty-free / custom"] |

---

## 5. Screenshot Guidelines

| Guideline | Specification |
|---|---|
| Browser frame | [e.g., "Include minimal browser chrome — address bar with URL, no tabs"] |
| Resolution | [e.g., "2x retina capture, exported at 1x and 2x"] |
| Data | [e.g., "Use realistic demo data, never real customer data"] |
| Annotations | [e.g., "Subtle callout arrows if needed, brand color, consistent style"] |
| Refresh cadence | [e.g., "Screenshots updated with every major UI release"] |

---

## 6. Asset Storage & Delivery

| Concern | Specification |
|---|---|
| Source files | [e.g., "Stored in [Figma/Google Drive/Dropbox] — link: [URL]"] |
| Exported assets | [e.g., "In repo under /public/images/ or /static/img/"] |
| CDN | [e.g., "Served via [Cloudflare/CloudFront/Vercel Edge]"] |
| Image optimization pipeline | [e.g., "Built into build process: sharp/squoosh auto-optimizes on build"] |

---

## 7. Related Documents

- [Style Guide](STYLE_GUIDE.md)
- [Performance Budget](../technical/PERFORMANCE_BUDGET.md)
- [Homepage Spec](../pages/HOMEPAGE_SPEC.md)
