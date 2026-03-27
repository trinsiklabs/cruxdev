# Color, Contrast & Readability Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** WCAG 2.2, APCA (WCAG 3.0 candidate), Material Design 3, Apple HIG, Radix Colors, GitHub Primer, Tailwind Slate palette
**Last updated:** 2026-03-27

## Why This Matters

Low-contrast text is the single most common accessibility failure on the web. 96% of top-million homepages fail basic contrast compliance. Getting color right in both light and dark modes requires specific hex values, not guesswork.

---

## 1. Contrast Requirements

### WCAG 2.2 (Current Legal Standard)

| Element | AA (Minimum) | AAA (Enhanced) |
|---------|-------------|----------------|
| Normal text | 4.5:1 | 7:1 |
| Large text (24px+ or 18.66px+ bold) | 3:1 | 4.5:1 |
| UI components, icons, borders | 3:1 | — |

### APCA (WCAG 3.0 Candidate)

APCA is polarity-aware (dark-on-light vs light-on-dark) and font-size sensitive. It more accurately reflects human perception, especially for dark mode. Use APCA as a quality check alongside WCAG ratios.

**Practical rule:** Design to WCAG 2.2 AA as floor. Use APCA as quality check. Aim for AAA on body text.

---

## 2. Light Mode Palette

**Background:** NOT pure white. Pure white (#FFF) causes glare.
**Text:** NOT pure black. Pure black (#000) on white causes halation and eye strain at 21:1 contrast.

| Role | Hex | Contrast on #FAFAFA |
|------|-----|-------------------|
| **Page background** | `#FAFAFA` | — |
| **Surface / card** | `#FFFFFF` | — |
| **Code block bg** | `#F1F5F9` | — |
| **Primary text** | `#111827` | 15.4:1 (AAA) |
| **Secondary text** | `#4B5563` | 7.0:1 (AAA) |
| **Tertiary text** | `#6B7280` | 4.6:1 (AA) |
| **Disabled text** | `#9CA3AF` | 3.0:1 (large only) |
| **Borders** | `#E5E7EB` | — |
| **Strong borders** | `#D1D5DB` | 3:1 non-text |
| **Links** | `#2563EB` | 4.6:1 (AA) |

---

## 3. Dark Mode Palette

**Background:** NOT pure black. #000 causes OLED smearing and excessive contrast.
**Text:** NOT pure white. #FFF on dark causes halation, especially for thin fonts.

| Role | Hex | Contrast on #0F172A |
|------|-----|-------------------|
| **Page background** | `#0F172A` | — |
| **Surface / card** | `#1E293B` | — |
| **Code block bg** | `#0D1117` | — |
| **Primary text** | `#F1F5F9` | 14.5:1 (AAA) |
| **Secondary text** | `#CBD5E1` | 9.7:1 (AAA) |
| **Tertiary text** | `#94A3B8` | 5.5:1 (AA) |
| **Disabled text** | `#64748B` | 3.1:1 (large only) |
| **Borders** | `#334155` | — |
| **Strong borders** | `#475569` | 3:1+ non-text |
| **Links** | `#60A5FA` | 5.2:1 (AA) |

### Why Slate (Blue-Gray) Not Neutral Gray

Pure neutral grays feel "dead." Slight blue undertone (Tailwind's slate palette) adds warmth and is easier on eyes for extended reading. Used by GitHub, Tailwind, Radix.

---

## 4. Semantic Colors (Both Modes)

Colors shift from 600-shade (light mode) to 400-shade (dark mode).

| Role | Light Mode | Dark Mode |
|------|-----------|-----------|
| **Error** | `#DC2626` (4.5:1 on white) | `#F87171` (5.2:1 on #0F172A) |
| **Success** | `#16A34A` (4.6:1 on white) | `#4ADE80` (6.3:1 on #0F172A) |
| **Warning** | `#D97706` (text), `#F59E0B` (bg) | `#FBBF24` (9.3:1 on #0F172A) |
| **Info** | `#2563EB` (4.6:1 on white) | `#60A5FA` (5.2:1 on #0F172A) |

**Never use color alone as an indicator.** Always pair with icon, text, border, or pattern (WCAG SC 1.4.1).

---

## 5. Token Architecture

Three-layer system (Material Design, Radix, GitHub Primer consensus):

### Layer 1: Primitive Tokens (mode-independent)
Raw values: `slate-50: #F8FAFC`, `slate-900: #0F172A`, `blue-400: #60A5FA`, etc.

### Layer 2: Semantic Tokens (switch between modes)
```css
:root {
  --color-bg-page: #FAFAFA;
  --color-text-primary: #111827;
  --color-accent: #2563EB;
}
.dark {
  --color-bg-page: #0F172A;
  --color-text-primary: #F1F5F9;
  --color-accent: #60A5FA;
}
```

### Layer 3: Component Tokens (reference semantic)
`button-bg: var(--color-accent)`, `card-bg: var(--color-bg-surface)`, etc.

---

## 6. Logo Handling

1. **SVG with `currentColor`** — best for monochrome logos. Auto-adapts.
2. **Two SVG variants + CSS class toggle** — for multi-color logos:
```html
<img src="/logo-light.svg" class="dark:hidden" alt="Logo" />
<img src="/logo-dark.svg" class="hidden dark:block" alt="Logo" />
```
3. **SVG with CSS custom properties** — most flexible for complex logos.
4. **Never:** PNG/JPEG with baked-in colors, transparency-dependent logos.

---

## 7. Testing

| Tool | Type | Use |
|------|------|-----|
| WebAIM Contrast Checker | Web | Quick ratio check |
| axe DevTools | Browser extension | Automated WCAG audit |
| Chrome DevTools | Built-in | Vision deficiency simulation |
| APCA Calculator | Web | Perceptual contrast check |
| Lighthouse | Chrome | Automated accessibility score |

**CI integration:** axe-core npm package in test suite catches contrast regressions.

---

## 8. Common Mistakes

| Mistake | Fix |
|---------|-----|
| Pure white background (#FFF) | Use #FAFAFA or #F8F9FA |
| Pure black text (#000) | Use #111827 |
| Pure black background (#000) | Use #0F172A or #111827 |
| Pure white text (#FFF) on dark | Use #F1F5F9 or #E2E8F0 |
| Low-contrast secondary text | Minimum #4B5563 light / #CBD5E1 dark |
| Same colors in both modes | Shift: 600-shade light, 400-shade dark |
| Color-only indicators | Always pair with icon + text |
| Same shadows in dark mode | Use surface lightness for elevation |
| Insufficient focus indicators | 2px ring, 3:1+ contrast |

---

## 9. Audit Dimensions

For convergence engine integration — audit color/contrast against:

1. **text_contrast** — all text meets WCAG AA (4.5:1 normal, 3:1 large)
2. **non_text_contrast** — borders, icons, focus rings meet 3:1
3. **semantic_color** — error/success/warning use correct shades per mode
4. **color_independence** — no color-only indicators
5. **dual_mode** — all elements verified in both light and dark
6. **token_architecture** — semantic tokens used, not hardcoded colors

---

## Full Token Reference

| Token | Light | Dark |
|-------|-------|------|
| `--color-bg-page` | `#FAFAFA` | `#0F172A` |
| `--color-bg-surface` | `#FFFFFF` | `#1E293B` |
| `--color-bg-elevated` | `#F3F4F6` | `#334155` |
| `--color-bg-code` | `#F1F5F9` | `#0D1117` |
| `--color-text-primary` | `#111827` | `#F1F5F9` |
| `--color-text-secondary` | `#4B5563` | `#CBD5E1` |
| `--color-text-tertiary` | `#6B7280` | `#94A3B8` |
| `--color-border` | `#E5E7EB` | `#334155` |
| `--color-border-strong` | `#D1D5DB` | `#475569` |
| `--color-accent` | `#2563EB` | `#60A5FA` |
| `--color-error` | `#DC2626` | `#F87171` |
| `--color-success` | `#16A34A` | `#4ADE80` |
| `--color-warning` | `#D97706` | `#FBBF24` |
| `--color-info` | `#2563EB` | `#60A5FA` |

---

## References

- WCAG 2.2 — w3.org/TR/WCAG22/ (SC 1.4.3, 1.4.6, 1.4.11)
- APCA — git.apcacontrast.com
- Material Design 3 — m3.material.io/styles/color/
- Apple HIG — developer.apple.com/design/human-interface-guidelines/color
- Radix Colors — radix-ui.com/colors
- GitHub Primer — primer.style/foundations/color
- Tailwind Slate — tailwindcss.com/docs/customizing-colors
- WebAIM Contrast Checker — webaim.org/resources/contrastchecker/
