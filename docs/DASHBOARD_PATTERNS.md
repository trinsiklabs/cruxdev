# Dashboard Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 35+ sources including Tufte, Stephen Few, NNg, Grafana, Datadog, Material Design, Apple HIG, GOV.UK, academic pattern taxonomies
**Last updated:** 2026-03-27
**Companion:** METRICS_PATTERNS.md (what to show); this doc covers how to show it.

---

## 1. Core Principles

1. **Data-ink ratio (Tufte).** Maximize the share of ink devoted to data. Remove gridlines, borders, decorations that don't convey information.
2. **Pre-attentive processing.** Color, size, position, and orientation are processed in < 250ms before conscious thought. Use these channels for critical status — don't waste them on decoration.
3. **5-9 metrics per view.** Cognitive load research (Miller, Kahneman System 1/2) shows working memory holds 5-9 items. More requires drill-down.
4. **Connect data to action.** A dashboard that shows what happened but not what to do about it is passive wallpaper. Every metric should suggest a response.
5. **Hierarchy, not equality.** Break visual symmetry intentionally. The most important KPI gets the biggest tile, top-left position, and strongest color.

---

## 2. Layout

### Grid Systems
- **12-column grid** for web (CSS standard). Grafana uses 24-column for finer control.
- **Bento grid** (variable-size tiles) establishes hierarchy: large hero KPI, medium analytical tiles, compact utility tiles.
- Even gutters read as orderly; breaking the grid feels noisy (Microsoft Data Science).

### Scanning Patterns
- **F-pattern:** Users scan horizontally at top, then down the left side. Place critical data top-left.
- **Z-pattern:** For simpler layouts. Eye moves top-left → top-right → bottom-left → bottom-right.
- **Inverted pyramid:** General at top, detail below. Users who only glance get the headline.

### Structure Options
- **Top-rail:** Horizontal header for nav/filters, full width below for data. Best for wide dashboards.
- **Sidebar:** Vertical nav/filters on left, content right. Best for frequent view-switching.
- **Tabs:** Segment by context/role without separate dashboards.
- **Cards:** Dominant container pattern. Internal layout must be consistent: title (top-left), date (top-right), legend (bottom).

---

## 3. Visualization Types

| Type | Best For | Avoid When |
|------|----------|------------|
| **Line chart** | Trends over time | Few data points, categorical |
| **Bar chart** | Comparing categories | > 12 categories |
| **Area chart** | Volume over time, stacked parts | Too many overlapping series |
| **Sparkline** | Compact trend next to a number | Exact values needed |
| **Gauge** | Single value in known range | Multiple comparisons |
| **Table** | Precise values, multi-dimension | Quick scanning |
| **Heatmap** | Density, time-of-day patterns | Small datasets |
| **Stat panel** | Single KPI emphasis | Context/trend needed |
| **Progress bar** | Completion toward target | Unbound ranges |

### The "One Metric That Matters" (OMTM)
- From Lean Analytics: the single most important number at a given time
- Display prominently as hero tile, above all other metrics
- Supporting metrics below in smaller tiles (bento grid pattern)
- Rotate OMTM every 2-4 months as focus shifts

---

## 4. Color

### Semantic System
- **Red/Orange:** Critical, errors, negative trends
- **Green/Blue:** Healthy, success, positive
- **Yellow/Amber:** Warning, attention
- **Gray:** Historical, disabled, background
- Never use semantic colors for non-status elements (a red bar that's just "brand" reads as error)

### Accessibility
- WCAG 2.1 SC 1.4.11: **3:1 minimum contrast** for non-text elements. Target 4.5:1.
- ~8% of men are colorblind. **Never rely on color alone.** Combine with shape + text.
- Maximum **5 colors** per visualization.
- **Sequential palettes** (single hue, light→dark) for magnitude.
- **Categorical palettes** for discrete categories.
- **Diverging palettes** for data with meaningful midpoint.
- Test with colorblind simulation tools. Use ColorBrewer for safe palettes.

### Dark/Light Mode
- Test all status colors in both modes.
- Maintain contrast ratios in both.

---

## 5. Real-Time Patterns

| Aspect | Recommendation |
|--------|---------------|
| **Refresh rate** | Critical: seconds. Trends: minutes. Reports: hours. |
| **Stale data** | Always show last-updated timestamp. Flag stale data visually. |
| **Animation** | 200-400ms for value transitions. Respect `prefers-reduced-motion`. |
| **Streaming** | WebSockets for low-latency. Batch visual updates to prevent flicker. |
| **Pause** | Let users freeze real-time updates to read without data moving. |
| **Offline** | Clear banner: "Offline... Reconnecting..." not vague errors. |

### Preventing Change Blindness
- Animate number transitions (count-up) rather than instant replacement.
- Slide animations (< 300ms) when lists reorder.
- Highlight changed values briefly before settling.

---

## 6. Interaction

### Filters
- **Page-level** (affect all charts) and **widget-level** (affect one section).
- Provide useful defaults, not blank states.
- Show loading feedback during filter application.

### Drill-Down
- **Drawer:** Side panel reveals details without losing main context.
- **Detail page:** Full-screen deep-dive.
- **Tooltips:** Detail-on-demand for light exploration.

### Time Range
- Standard presets: Last hour / 6h / 24h / 7d / 30d / custom.
- Support comparison: "vs. last week" / "vs. last month."
- Limit selectable ranges to prevent illegible density.

### Personalization
- Custom metric selection, show/hide sections.
- Drag-to-reorder modules.
- Role-based defaults (manager vs developer vs analyst).

---

## 7. Mobile / Responsive

- **5 key metrics maximum** on mobile.
- Progressive disclosure: most important first, expandable cards for detail.
- Vertical stacking of cards (side-by-side on desktop).
- Touch targets >= 44pt (Apple HIG).
- Swipeable cards for horizontal navigation between metric groups.
- Consider "this chart is better on a larger screen" messaging for complex visualizations.

---

## 8. Performance

| Technique | When |
|-----------|------|
| **Lazy loading** | Load detail on demand, not upfront |
| **Virtualization** | Large tables: render only visible rows |
| **Pre-aggregation** | Aggregate server-side, not in browser |
| **Skeleton UI** | Show layout instantly, fill data as it loads |
| **Debounce** | Batch filter changes before querying |

Grafana: overloading panels can slow load times by 200-400%.

---

## 9. Accessibility

- **ARIA live regions** for announcing metric changes to screen readers.
- **Keyboard navigation** through all controls. Logical tab order.
- **Focus indicators** on interactive elements.
- Never color-only status indicators. Use color + shape + text.
- Apple Swift Charts: automatic accessibility labels and audio graphs.
- WCAG 2.2 Level AA compliance required for public sector (UK law).

---

## 10. Anti-Patterns

| Anti-Pattern | Problem | Fix |
|-------------|---------|-----|
| **Pie charts for comparison** | Humans bad at comparing angles | Use bar charts |
| **3D charts** | Distort proportions, occlude data | Always 2D |
| **Rainbow color scales** | Non-uniform perception, colorblind-hostile | Sequential single-hue |
| **Information overload** | > 9 metrics per view | Progressive disclosure |
| **Chartjunk** | Gradients, decorations, 3D effects | Maximize data-ink ratio |
| **Equal visual weight** | Users don't know where to look | Break symmetry, create hierarchy |
| **Excessive precision** | "24.5932%" when "24.6%" suffices | Round appropriately |
| **Missing context** | Numbers without baselines or trends | Always show comparison |
| **Passive dashboards** | Show data without suggesting action | Connect metrics to responses |
| **Dashboard hoarding** | Hundreds of unused dashboards | Audit and retire regularly |

### Stephen Few's Core Insight
Dashboard design skills are not intuitive. Understanding how the brain processes visual information explains why these patterns work and anti-patterns fail.

---

## 11. Dashboard vs. Automated Alerts

| Use Dashboard | Use Alerts | Use Both |
|--------------|------------|----------|
| Exploration, pattern finding | Known-bad thresholds | Critical metrics on dashboard + alert |
| Historical analysis | Time-sensitive events | Status overview + threshold pages |
| Multi-dimensional comparison | Binary good/bad | Trend dashboard + anomaly alerts |
| Team standup context | On-call response | |

**The "post-dashboard" argument:** AI anomaly detection + automated alerts can replace passive staring. The middle ground: dashboards WITH built-in threshold alerts and data storytelling.

---

## 12. Audit Dimensions

For convergence engine integration — audit dashboards against these dimensions:

1. **hierarchy** — Is there a clear visual hierarchy? OMTM prominent? F-pattern respected?
2. **density** — 5-9 metrics per view? Progressive disclosure for detail?
3. **visualization** — Correct chart types? No pie charts for comparison? No 3D?
4. **color** — Semantic, accessible, 3:1+ contrast, not color-only?
5. **real_time** — Appropriate refresh rates? Stale data indicators? Pause capability?
6. **mobile** — Responsive? Progressive disclosure? Touch targets?
7. **accessibility** — ARIA regions, keyboard nav, colorblind safe?
8. **performance** — Lazy loading? Virtualization? Pre-aggregation?
9. **actionability** — Does every metric connect to a decision or response?

---

## References

- Edward Tufte — The Visual Display of Quantitative Information
- Stephen Few — Information Dashboard Design
- NNg — 10 Usability Heuristics, F-pattern research
- Grafana — grafana.com/docs/dashboards/best-practices/
- Datadog — docs.datadoghq.com/dashboards/
- Material Design — m2.material.io/design/communication/data-visualization.html
- Apple HIG — developer.apple.com/design/human-interface-guidelines/charting-data
- GOV.UK ONS Service Manual — service-manual.ons.gov.uk/data-visualisation/
- PatternFly — patternfly.org/patterns/dashboard/
- Dashboard Design Patterns — dashboarddesignpatterns.github.io/
- Lean Analytics (OMTM) — leananalyticsbook.com/
- ColorBrewer — colorbrewer2.org/
