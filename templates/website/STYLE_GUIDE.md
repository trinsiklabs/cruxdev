---
title: Web Style Guide
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Web Style Guide

> Web-specific visual standards: typography, colors, spacing, components, responsive behavior.
> This extends the brand guidelines for web-specific implementation.

## 1. Typography

### 1.1 Font Stack

| Usage | Font Family | Fallback | Weight | Source |
|---|---|---|---|---|
| Headings | [e.g., "Inter"] | [e.g., "system-ui, -apple-system, sans-serif"] | [Bold (700)] | [Google Fonts / self-hosted] |
| Body text | [e.g., "Inter"] | [Same fallback] | [Regular (400), Medium (500)] | [Same] |
| Code / monospace | [e.g., "JetBrains Mono"] | [e.g., "ui-monospace, monospace"] | [Regular (400)] | [Source] |

### 1.2 Type Scale

| Element | Size (Desktop) | Size (Mobile) | Line Height | Letter Spacing | Weight |
|---|---|---|---|---|---|
| H1 | [e.g., 48px / 3rem] | [e.g., 36px / 2.25rem] | [1.1] | [-0.02em] | [700] |
| H2 | [e.g., 36px / 2.25rem] | [e.g., 28px / 1.75rem] | [1.2] | [-0.01em] | [700] |
| H3 | [e.g., 24px / 1.5rem] | [e.g., 20px / 1.25rem] | [1.3] | [0] | [600] |
| H4 | [e.g., 20px / 1.25rem] | [e.g., 18px / 1.125rem] | [1.4] | [0] | [600] |
| Body (large) | [e.g., 18px / 1.125rem] | [e.g., 16px / 1rem] | [1.6] | [0] | [400] |
| Body | [e.g., 16px / 1rem] | [e.g., 16px / 1rem] | [1.6] | [0] | [400] |
| Small / Caption | [e.g., 14px / 0.875rem] | [e.g., 14px / 0.875rem] | [1.5] | [0.01em] | [400] |
| Button text | [e.g., 16px / 1rem] | [e.g., 16px / 1rem] | [1] | [0.02em] | [500] |
| Nav link | [e.g., 15px / 0.9375rem] | [e.g., 16px / 1rem] | [1] | [0.01em] | [500] |

### 1.3 Typography Rules

- Maximum line width for body text: [e.g., 65-75 characters / ~700px]
- Paragraph spacing: [e.g., 1.5em margin-bottom]
- Heading spacing: [e.g., 2em margin-top, 0.5em margin-bottom]
- No orphaned headings at bottom of viewport (CSS: break-after: avoid)

---

## 2. Color System

### 2.1 Brand Colors

| Name | Hex | RGB | Usage |
|---|---|---|---|
| Primary | [e.g., #2563EB] | [e.g., 37, 99, 235] | [Primary buttons, links, accents] |
| Primary Dark | [e.g., #1D4ED8] | [Values] | [Primary button hover] |
| Primary Light | [e.g., #DBEAFE] | [Values] | [Backgrounds, highlights] |
| Secondary | [e.g., #0F172A] | [Values] | [Headings, dark backgrounds] |
| Accent | [e.g., #F59E0B] | [Values] | [Highlights, badges, warnings] |

### 2.2 Neutral Colors

| Name | Hex | Usage |
|---|---|---|
| Gray 900 | [e.g., #0F172A] | [Heading text, dark UI elements] |
| Gray 700 | [e.g., #334155] | [Body text] |
| Gray 500 | [e.g., #64748B] | [Secondary text, placeholders] |
| Gray 300 | [e.g., #CBD5E1] | [Borders, dividers] |
| Gray 100 | [e.g., #F1F5F9] | [Light backgrounds, alternating rows] |
| White | [#FFFFFF] | [Page background, cards] |

### 2.3 Semantic Colors

| Name | Hex | Usage |
|---|---|---|
| Success | [e.g., #16A34A] | [Success messages, positive indicators] |
| Warning | [e.g., #EAB308] | [Warnings, caution indicators] |
| Error | [e.g., #DC2626] | [Error messages, destructive actions] |
| Info | [e.g., #2563EB] | [Informational messages, help text] |

### 2.4 Contrast Requirements

All color combinations must meet WCAG 2.1 AA contrast ratios:
- Normal text: minimum 4.5:1
- Large text (18px+ bold or 24px+ regular): minimum 3:1
- UI components: minimum 3:1

---

## 3. Spacing System

### 3.1 Spacing Scale

| Token | Value | Usage |
|---|---|---|
| `space-1` | 4px / 0.25rem | [Tight spacing: icon-label gaps] |
| `space-2` | 8px / 0.5rem | [Small gaps: button icon spacing, tight padding] |
| `space-3` | 12px / 0.75rem | [Medium-small: card padding (mobile)] |
| `space-4` | 16px / 1rem | [Base: button padding, input padding] |
| `space-6` | 24px / 1.5rem | [Medium: card padding, section content gaps] |
| `space-8` | 32px / 2rem | [Large: between content blocks] |
| `space-12` | 48px / 3rem | [Section padding (mobile)] |
| `space-16` | 64px / 4rem | [Section padding (desktop)] |
| `space-24` | 96px / 6rem | [Major section breaks] |
| `space-32` | 128px / 8rem | [Hero padding, major visual breaks] |

### 3.2 Layout

| Property | Value |
|---|---|
| Max content width | [e.g., 1200px / 75rem] |
| Max text width | [e.g., 700px / 43.75rem] |
| Grid columns | [e.g., 12-column grid] |
| Gutter width | [e.g., 24px / 1.5rem] |
| Container padding | [e.g., 16px mobile, 24px tablet, 32px desktop] |

---

## 4. Components

### 4.1 Buttons

| Variant | Background | Text Color | Border | Hover State | Padding |
|---|---|---|---|---|---|
| Primary | [Brand primary] | [White] | [None] | [Primary dark] | [12px 24px] |
| Secondary | [Transparent] | [Brand primary] | [1px solid primary] | [Primary light bg] | [12px 24px] |
| Tertiary / Ghost | [Transparent] | [Gray 700] | [None] | [Gray 100 bg] | [8px 16px] |
| Destructive | [Error red] | [White] | [None] | [Darker red] | [12px 24px] |

Button rules:
- Border radius: [e.g., 8px / 0.5rem]
- Minimum tap target: 44x44px (mobile accessibility)
- Icon + text gap: 8px
- Always use button element for actions, anchor for navigation

### 4.2 Cards

| Property | Value |
|---|---|
| Background | [White or Gray 100] |
| Border | [1px solid Gray 300 or none] |
| Border radius | [e.g., 12px / 0.75rem] |
| Shadow | [e.g., "0 1px 3px rgba(0,0,0,0.1)" or none] |
| Padding | [e.g., 24px] |
| Hover (if clickable) | [e.g., "Shadow increases, subtle translate-y"] |

### 4.3 Forms

| Element | Specification |
|---|---|
| Input height | [e.g., 44px / 2.75rem] |
| Input padding | [e.g., 12px 16px] |
| Border | [1px solid Gray 300] |
| Focus state | [e.g., "2px solid Primary, no outline"] |
| Error state | [e.g., "1px solid Error red, error message below in red"] |
| Label | [Above input, 14px, Gray 700, 4px margin-bottom] |
| Placeholder | [Gray 500, italic or regular — describe expected input] |
| Border radius | [e.g., 8px / 0.5rem] |

### 4.4 Navigation

| Element | Specification |
|---|---|
| Height | [e.g., 64px desktop, 56px mobile] |
| Background | [White or translucent blur] |
| Position | [Sticky top] |
| Link style | [Gray 700, hover: Primary, active: Primary + underline] |
| Mobile breakpoint | [e.g., 768px — hamburger menu below this] |
| CTA button in nav | [Primary button style, smaller padding] |

---

## 5. Imagery

### 5.1 Image Guidelines

| Type | Format | Max Size | Dimensions | Notes |
|---|---|---|---|---|
| Hero images | WebP (AVIF fallback) | 200KB | [e.g., 1920x1080 max] | Use responsive srcset |
| Card thumbnails | WebP | 50KB | [e.g., 600x400] | Consistent aspect ratio |
| Blog featured | WebP | 100KB | [1200x630] | Doubles as OG image |
| Team photos | WebP | 50KB | [400x400, square] | Consistent crop and style |
| Icons | SVG | 5KB | N/A | Inline SVG preferred |
| Logos (customer) | SVG or PNG | 20KB | [Height: 32-48px] | Grayscale on dark/light bg |

### 5.2 Image Treatment

- Product screenshots: [e.g., "Drop shadow, rounded corners, browser frame mockup"]
- Illustrations: [e.g., "Flat style, brand colors, consistent stroke width"]
- Photography: [e.g., "Natural lighting, no stock-photo feel, diverse representation"]

---

## 6. Responsive Breakpoints

| Name | Breakpoint | Target Devices |
|---|---|---|
| Mobile | [e.g., <640px] | [Phones] |
| Tablet | [e.g., 640px-1023px] | [Tablets, small laptops] |
| Desktop | [e.g., 1024px-1279px] | [Laptops] |
| Wide | [e.g., 1280px+] | [Desktop monitors] |

### Responsive Rules

- All layouts must be functional at 320px minimum width
- Touch targets: minimum 44x44px on mobile
- Font sizes: never below 16px for body text on mobile (prevents iOS zoom)
- Images: responsive srcset, lazy loading below the fold
- Tables: horizontal scroll wrapper on mobile

---

## 7. Animation & Motion

| Type | Duration | Easing | Usage |
|---|---|---|---|
| Hover transitions | 150ms | ease-out | Buttons, links, cards |
| Page transitions | 200-300ms | ease-in-out | Route changes (if SPA) |
| Scroll reveals | 400ms | ease-out | Content blocks entering viewport |
| Loading states | 200ms | ease-in-out | Skeleton screens, spinners |

Rules:
- Respect `prefers-reduced-motion: reduce` — disable non-essential animations
- No animations that block interaction
- No auto-playing carousels (accessibility and UX)

---

## 8. Figma / Design Tool Reference

| Resource | Link |
|---|---|
| Design system file | [Figma/Sketch/etc. URL] |
| Component library | [URL] |
| Brand assets | [URL or local path] |
| Icon set | [e.g., "Lucide Icons" or custom set URL] |

---

## 9. Related Documents

- [Brand Guidelines](Link to brand guidelines if separate)
- [Media Assets](MEDIA_ASSETS.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md) (accessibility section)
