# DRY UI Component Patterns

How to apply Don't Repeat Yourself principles to UI development without over-abstracting. Covers component API design, composition patterns, design token systems, configuration-driven variants, and framework-specific guidance for Phoenix/LiveView.

**Audience:** Developers building component libraries or design systems in any framework, with special attention to the PETAL stack (Phoenix, Elixir, Tailwind, Ash, LiveView).

---

## Table of Contents

1. [Principles](#1-principles)
2. [Component API Design Patterns](#2-component-api-design-patterns)
3. [Composition Patterns](#3-composition-patterns)
4. [Design Token Integration](#4-design-token-integration)
5. [Configuration-Driven Variants](#5-configuration-driven-variants)
6. [Anti-Patterns](#6-anti-patterns)
7. [Framework-Specific Guidance](#7-framework-specific-guidance)
8. [Real-World Library Architectures](#8-real-world-library-architectures)
9. [Audit Dimensions](#9-audit-dimensions)

---

## 1. Principles

### 1.1 Separate Structure/Behavior from Presentation

Every well-designed component library enforces a clear boundary between what a component **does** (state management, accessibility, keyboard navigation, ARIA attributes) and how it **looks** (colors, spacing, typography, visual states).

**Rationale:** When logic and styling are coupled, changing the visual design requires touching behavioral code, and vice versa. This coupling multiplies maintenance cost and increases the risk of breaking accessibility when "just changing styles."

**Evidence:** This is the foundational insight behind Radix UI, Headless UI, React Aria, and every successful headless component library. shadcn/ui makes it explicit with a two-layer architecture: Radix primitives handle behavior, Tailwind + CVA handle presentation.

### 1.2 Composition Over Configuration

Prefer composing small, focused components over building one mega-component with dozens of props. Radix UI's design philosophy: "You don't pass 30 props to toggle behavior---you compose smaller primitives into custom patterns."

**Rationale:** Configuration-heavy components (the "god component") create APIs that are hard to learn, hard to type-check, and hard to extend. Composition creates APIs that read like HTML and can be extended by adding children rather than props.

**The rule:** If a component has more than 7-8 props (excluding HTML globals), it probably needs to be decomposed.

### 1.3 Duplicate First, Abstract Second

This is the most counterintuitive DRY principle for UI work. shadcn/ui's ownership model embodies it: copy the component, use it, observe where real patterns emerge, then abstract.

**Rationale:** Premature abstraction in UI code is more expensive than duplication. A wrong abstraction forces every consumer to work around it, while duplication can be mechanically refactored once the true pattern is clear. The "Rule of Three" applies: don't abstract until you have at least three concrete instances of the same pattern.

**Quote from the shadcn/ui philosophy:** "A better approach is to duplicate first, observe patterns, and abstract only when repetition is proven."

### 1.4 Tokens Over Hardcoded Values

Every visual decision (color, spacing, font size, shadow, border radius) should reference a design token, not a literal value. Components should never contain `#CF2030` or `16px`---they should reference `--color-brand-primary` or `spacing-4`.

**Rationale:** When a design decision changes (and it will), you change one token and every component updates. Without tokens, you grep-and-replace across hundreds of files.

### 1.5 Accessibility Is Not Optional

Headless/unstyled libraries (Radix, Headless UI, React Aria) exist because building accessible interactive components is genuinely hard. Focus management, keyboard navigation, ARIA attributes, and screen reader behavior must be handled at the structural layer, not patched on after styling.

**Rationale:** If your DRY strategy produces inaccessible components, it has failed at its primary job. Use established primitives for interactive patterns (dialogs, dropdowns, tabs, comboboxes) rather than building from scratch.

### 1.6 Own Your Components

Whether using shadcn/ui's copy-paste model or building from scratch, the team that ships the product should own the component source code. Wrapping a third-party library in thin wrappers creates the worst of both worlds: maintenance burden without control.

**Rationale:** True ownership means you can modify any component to meet your exact needs without fighting upstream abstractions. It means your components evolve with your product.

---

## 2. Component API Design Patterns

### 2.1 Enum Props for Constrained Variants

Instead of boolean props (`isLarge`, `isSmall`, `isCompact`), use a single enum prop with named values.

**Bad -- boolean explosion:**
```jsx
<Button isLarge isPrimary isOutline />  // What if isLarge AND isSmall?
```

**Good -- enum variants:**
```jsx
<Button size="lg" color="primary" variant="outline" />
```

**Phoenix/LiveView equivalent:**
```elixir
attr :size, :string, default: "md", values: ["xs", "sm", "md", "lg", "xl"]
attr :color, :string, default: "primary", values: ["primary", "secondary", "danger", "success"]
attr :variant, :string, default: "solid", values: ["solid", "outline", "ghost", "light"]

def button(assigns) do
  ~H"""
  <button class={button_classes(@size, @color, @variant)} {@rest}>
    {render_slot(@inner_block)}
  </button>
  """
end
```

**Why this works:**
- Compile-time validation (LiveView warns on invalid `values`)
- Self-documenting API (the `values` list IS the documentation)
- Impossible invalid states (you cannot be both `"lg"` and `"sm"`)
- Easy to extend (add `"2xl"` to the list)

### 2.2 Global Attributes with Rest

Always accept a `:global` rest attribute so consumers can pass standard HTML attributes without the component explicitly declaring each one.

```elixir
attr :size, :string, default: "md", values: ["xs", "sm", "md", "lg", "xl"]
attr :rest, :global, include: ~w(disabled form name value type)

def button(assigns) do
  ~H"""
  <button class={["btn", "btn-#{@size}"]} {@rest}>
    {render_slot(@inner_block)}
  </button>
  """
end
```

**Usage:**
```heex
<.button size="lg" disabled phx-click="submit" data-testid="save-btn">
  Save
</.button>
```

This pattern is equivalent to React's `{...rest}` spread and ensures forward-compatibility with any HTML attribute.

### 2.3 Default Slot + Named Slots

Use `@inner_block` for primary content and named slots for structured regions.

```elixir
slot :icon, doc: "Optional icon displayed before the label"
slot :inner_block, required: true, doc: "Button label content"

def button(assigns) do
  ~H"""
  <button class="btn">
    <span :if={@icon != []} class="btn-icon">{render_slot(@icon)}</span>
    {render_slot(@inner_block)}
  </button>
  """
end
```

**Usage:**
```heex
<.button>
  <:icon><.icon name="hero-check" /></:icon>
  Save Changes
</.button>
```

**Why slots beat props for content:** Slots accept arbitrary HEEx markup. A `label` string prop cannot contain icons, links, or formatted text. Slots can.

### 2.4 Class Merging Pattern

Provide strong defaults but allow class extension (not replacement).

```elixir
attr :class, :string, default: nil

def card(assigns) do
  ~H"""
  <div class={["rounded-lg border bg-card p-6 shadow-sm", @class]}>
    {render_slot(@inner_block)}
  </div>
  """
end
```

The list syntax in LiveView concatenates classes. For the JavaScript ecosystem, the `cn()` utility (clsx + tailwind-merge) resolves Tailwind class conflicts:

```typescript
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

### 2.5 Extending Native Element Props

Components should extend the native element they wrap, not replace its API.

**React/TypeScript:**
```typescript
interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}
```

**Phoenix/LiveView:**
```elixir
attr :rest, :global, include: ~w(disabled form name value type autofocus)
```

This ensures consumers can use `disabled`, `autofocus`, `data-*`, `aria-*`, and any future HTML attributes without the component author predicting them.

---

## 3. Composition Patterns

### 3.1 Compound Components

A group of components that share implicit state and work together as a unit. The parent manages state via context; children consume it.

**When to use:** Components with interdependent parts (Tabs + TabList + TabPanel, Accordion + AccordionItem, Select + Option).

**React implementation:**
```jsx
const TabsContext = createContext();

function Tabs({ defaultValue, children }) {
  const [activeTab, setActiveTab] = useState(defaultValue);
  return (
    <TabsContext.Provider value={{ activeTab, setActiveTab }}>
      <div role="tablist">{children}</div>
    </TabsContext.Provider>
  );
}

function Tab({ value, children }) {
  const { activeTab, setActiveTab } = useContext(TabsContext);
  return (
    <button
      role="tab"
      aria-selected={activeTab === value}
      onClick={() => setActiveTab(value)}
    >
      {children}
    </button>
  );
}

function TabPanel({ value, children }) {
  const { activeTab } = useContext(TabsContext);
  if (activeTab !== value) return null;
  return <div role="tabpanel">{children}</div>;
}

// Attach as properties for clean imports
Tabs.Tab = Tab;
Tabs.Panel = TabPanel;
```

**Usage:**
```jsx
<Tabs defaultValue="general">
  <Tabs.Tab value="general">General</Tabs.Tab>
  <Tabs.Tab value="advanced">Advanced</Tabs.Tab>
  <Tabs.Panel value="general">General settings...</Tabs.Panel>
  <Tabs.Panel value="advanced">Advanced settings...</Tabs.Panel>
</Tabs>
```

**Phoenix/LiveView equivalent** -- LiveView does not have React-style context, so compound patterns use explicit assigns or LiveComponents with send/handle_info:

```elixir
# Parent manages state, passes it to children via assigns
attr :active_tab, :string, required: true
slot :tab, required: true do
  attr :value, :string, required: true
  attr :label, :string, required: true
end
slot :panel, required: true do
  attr :value, :string, required: true
end

def tabs(assigns) do
  ~H"""
  <div>
    <div role="tablist">
      <button
        :for={tab <- @tab}
        role="tab"
        aria-selected={@active_tab == tab.value}
        phx-click="switch_tab"
        phx-value-tab={tab.value}
      >
        {tab.label}
      </button>
    </div>
    <div :for={panel <- @panel} :if={@active_tab == panel.value} role="tabpanel">
      {render_slot(panel)}
    </div>
  </div>
  """
end
```

**Usage:**
```heex
<.tabs active_tab={@active_tab}>
  <:tab value="general" label="General" />
  <:tab value="advanced" label="Advanced" />
  <:panel value="general">General settings here</:panel>
  <:panel value="advanced">Advanced settings here</:panel>
</.tabs>
```

### 3.2 Headless Components (Logic-Only)

Provide behavior and state management with zero visual opinions. The consumer provides all rendering.

**React hook-based pattern:**
```jsx
function useToggle(initial = false) {
  const [on, setOn] = useState(initial);
  const toggle = useCallback(() => setOn(prev => !prev), []);
  const buttonProps = {
    "aria-pressed": on,
    onClick: toggle,
  };
  return { on, toggle, buttonProps };
}

// Consumer renders whatever they want
function CustomSwitch() {
  const { on, buttonProps } = useToggle();
  return (
    <button {...buttonProps} className={on ? "bg-green-500" : "bg-gray-300"}>
      {on ? "ON" : "OFF"}
    </button>
  );
}
```

**Phoenix/LiveView equivalent:** Headless logic lives in the LiveView's `handle_event` callbacks, not in the component. Components in LiveView are inherently "presentation-only" function components. The LiveView itself is the "headless" state manager.

### 3.3 Polymorphic Components

A component that can render as different HTML elements or other components.

**React -- `as` prop pattern:**
```tsx
type ButtonProps<C extends React.ElementType = "button"> = {
  as?: C;
  variant?: "primary" | "secondary";
} & React.ComponentPropsWithoutRef<C>;

function Button<C extends React.ElementType = "button">({
  as,
  variant = "primary",
  ...props
}: ButtonProps<C>) {
  const Component = as || "button";
  return <Component className={`btn btn-${variant}`} {...props} />;
}

// Renders as <a> with all anchor props
<Button as="a" href="/dashboard" variant="primary">Go</Button>

// Renders as Next.js Link
<Button as={Link} to="/dashboard">Go</Button>
```

**React -- `asChild` prop pattern (Radix UI):**
```tsx
<Dialog.Trigger asChild>
  <Button variant="outline">Open Dialog</Button>
</Dialog.Trigger>
```

When `asChild` is true, the component merges its props onto its single child element instead of rendering its own DOM node. This avoids the TypeScript complexity of the `as` prop.

**Phoenix/LiveView:** Polymorphism is typically handled with conditional rendering:
```elixir
attr :navigate, :string, default: nil
attr :href, :string, default: nil

def button_or_link(assigns) do
  ~H"""
  <.link :if={@navigate || @href} navigate={@navigate} href={@href} class="btn">
    {render_slot(@inner_block)}
  </.link>
  <button :if={!@navigate && !@href} class="btn" {@rest}>
    {render_slot(@inner_block)}
  </button>
  """
end
```

### 3.4 Slot Composition (Vue / Svelte / LiveView)

Slots are the framework-native way to compose components in template-driven systems. They express **structure** rather than **data**.

**Vue -- named + scoped slots:**
```vue
<template>
  <div class="card">
    <div class="card-header">
      <slot name="header" />
    </div>
    <div class="card-body">
      <slot :item="currentItem" />  <!-- scoped slot -->
    </div>
    <div class="card-footer">
      <slot name="footer" />
    </div>
  </div>
</template>
```

**Svelte -- named slots:**
```svelte
<div class="card">
  <div class="card-header">
    <slot name="header" />
  </div>
  <div class="card-body">
    <slot />
  </div>
</div>
```

**LiveView -- named slots with attributes:**
```elixir
slot :header, doc: "Card header content"
slot :inner_block, required: true, doc: "Card body"
slot :footer, doc: "Card footer"
slot :action do
  attr :label, :string, required: true
  attr :navigate, :string
end

def card(assigns) do
  ~H"""
  <div class="card">
    <div :if={@header != []} class="card-header">
      {render_slot(@header)}
    </div>
    <div class="card-body">
      {render_slot(@inner_block)}
    </div>
    <div :if={@footer != []} class="card-footer">
      {render_slot(@footer)}
    </div>
    <div :if={@action != []} class="card-actions">
      <.link :for={action <- @action} navigate={action.navigate} class="btn">
        {action.label}
      </.link>
    </div>
  </div>
  """
end
```

**Usage:**
```heex
<.card>
  <:header>User Profile</:header>
  Profile content here...
  <:action label="Edit" navigate={~p"/users/#{@user}/edit"} />
  <:action label="Delete" navigate={~p"/users/#{@user}/delete"} />
</.card>
```

**Key insight:** Slot attributes in LiveView provide compile-time validation that arbitrary `assigns` do not. Always prefer `slot :name do attr ... end` over passing raw maps.

---

## 4. Design Token Integration

### 4.1 What Are Design Tokens?

Design tokens are named values representing design decisions: colors, spacing, typography, shadows, border radii, animation durations. They are the single source of truth for visual consistency.

The W3C Design Tokens Community Group published the first stable specification (2025.10) defining a vendor-neutral JSON format (`.tokens.json`) for interoperability across tools and platforms.

### 4.2 Token Architecture

```
Source of Truth (Figma / .tokens.json)
        |
        v
  Style Dictionary / Build Tool
        |
    +---+---+---+
    |       |       |
    v       v       v
  CSS     Tailwind  Native
  vars    config    (iOS/Android)
```

**Three tiers of tokens:**

| Tier | Example | Purpose |
|------|---------|---------|
| **Global** | `--color-blue-500: #3B82F6` | Raw palette values |
| **Semantic** | `--color-primary: var(--color-blue-500)` | Intent-based aliases |
| **Component** | `--button-bg: var(--color-primary)` | Component-scoped bindings |

### 4.3 CSS Custom Properties (Recommended Foundation)

```css
/* Global tokens */
:root {
  --color-blue-500: #3B82F6;
  --color-red-500: #EF4444;
  --spacing-1: 0.25rem;
  --spacing-2: 0.5rem;
  --spacing-4: 1rem;
  --radius-md: 0.375rem;
  --font-sans: "Inter", system-ui, sans-serif;
}

/* Semantic tokens */
:root {
  --color-primary: var(--color-blue-500);
  --color-danger: var(--color-red-500);
  --color-bg: white;
  --color-text: #1a1a1a;
}

/* Dark theme override */
[data-theme="dark"] {
  --color-bg: #0a0a0a;
  --color-text: #fafafa;
  --color-primary: var(--color-blue-400);
}
```

### 4.4 Tailwind 4.x Integration

Tailwind 4 introduced the `@theme` directive for defining tokens directly in CSS:

```css
/* assets/css/app.css */
@import "tailwindcss";

@theme {
  --color-primary: #3B82F6;
  --color-secondary: #6B7280;
  --color-danger: #EF4444;
  --color-success: #10B981;
  --color-warning: #F59E0B;

  --spacing-page: 2rem;
  --radius-card: 0.5rem;

  --font-heading: "Inter", system-ui, sans-serif;
  --font-body: "Inter", system-ui, sans-serif;
}
```

These tokens become Tailwind utilities automatically: `bg-primary`, `text-danger`, `rounded-card`, `p-page`.

### 4.5 Style Dictionary Pipeline

For multi-platform projects or Figma sync:

```json
// tokens/color.tokens.json (W3C DTCG format)
{
  "color": {
    "primary": {
      "$value": "#3B82F6",
      "$type": "color"
    },
    "danger": {
      "$value": "#EF4444",
      "$type": "color"
    }
  }
}
```

Style Dictionary transforms this into CSS variables, Tailwind config values, iOS/Android constants, or any other format needed.

### 4.6 Token Rules

1. **Components never hardcode values.** Every color, spacing, and size references a token.
2. **Semantic tokens always.** Components reference `--color-primary`, never `--color-blue-500`.
3. **Theme-ability comes free.** If all components use semantic tokens, theming is just reassigning the semantic layer.
4. **Tokens are the API contract** between design and engineering. Change a token value, not a component.

---

## 5. Configuration-Driven Variants

### 5.1 Class Variance Authority (CVA)

CVA is framework-agnostic. It lets you declare component variants as a configuration object rather than conditional logic.

```typescript
import { cva, type VariantProps } from "class-variance-authority";

const button = cva(
  // Base classes (always applied)
  "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2",
  {
    variants: {
      variant: {
        solid: "bg-primary text-white hover:bg-primary/90",
        outline: "border border-input bg-transparent hover:bg-accent",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        sm: "h-8 px-3 text-xs",
        md: "h-10 px-4 text-sm",
        lg: "h-12 px-6 text-base",
      },
    },
    compoundVariants: [
      {
        variant: "solid",
        size: "lg",
        class: "text-lg font-semibold",
      },
    ],
    defaultVariants: {
      variant: "solid",
      size: "md",
    },
  }
);

// Usage
button({ variant: "outline", size: "lg" });
// => "inline-flex items-center ... border border-input ... h-12 px-6 text-base"

// TypeScript: variant props are auto-typed
type ButtonProps = VariantProps<typeof button>;
// { variant?: "solid" | "outline" | "ghost" | "link"; size?: "sm" | "md" | "lg" }
```

**Key features:**
- **Compound variants:** Apply classes when multiple conditions are met simultaneously
- **Default variants:** Fallback values when no variant is specified
- **Type inference:** TypeScript types generated automatically from the config
- **Framework agnostic:** Works with Tailwind, CSS Modules, or plain class strings

### 5.2 CVA-like Pattern in Phoenix/LiveView

LiveView does not have CVA, but the same pattern can be built with pure Elixir:

```elixir
defmodule MyAppWeb.Variants do
  @moduledoc "CVA-like variant configuration for LiveView components."

  def button_classes(opts \\ []) do
    variant = Keyword.get(opts, :variant, "solid")
    size = Keyword.get(opts, :size, "md")
    color = Keyword.get(opts, :color, "primary")

    base = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2"

    variant_classes = %{
      "solid"   => "text-white shadow",
      "outline" => "border bg-transparent",
      "ghost"   => "hover:bg-accent",
      "link"    => "underline-offset-4 hover:underline"
    }

    size_classes = %{
      "xs" => "h-7 px-2 text-xs",
      "sm" => "h-8 px-3 text-xs",
      "md" => "h-10 px-4 text-sm",
      "lg" => "h-12 px-6 text-base",
      "xl" => "h-14 px-8 text-lg"
    }

    color_classes = %{
      {"primary", "solid"} => "bg-primary-600 hover:bg-primary-700",
      {"primary", "outline"} => "border-primary-600 text-primary-600 hover:bg-primary-50",
      {"danger", "solid"} => "bg-red-600 hover:bg-red-700",
      {"danger", "outline"} => "border-red-600 text-red-600 hover:bg-red-50"
      # ... additional combinations
    }

    [
      base,
      Map.get(variant_classes, variant, ""),
      Map.get(size_classes, size, ""),
      Map.get(color_classes, {color, variant}, "")
    ]
    |> Enum.join(" ")
  end
end
```

**Usage in component:**
```elixir
attr :variant, :string, default: "solid", values: ~w(solid outline ghost link)
attr :size, :string, default: "md", values: ~w(xs sm md lg xl)
attr :color, :string, default: "primary", values: ~w(primary secondary danger success)
attr :class, :string, default: nil
attr :rest, :global, include: ~w(disabled form name value type)
slot :inner_block, required: true

def button(assigns) do
  ~H"""
  <button
    class={[
      Variants.button_classes(variant: @variant, size: @size, color: @color),
      @class
    ]}
    {@rest}
  >
    {render_slot(@inner_block)}
  </button>
  """
end
```

### 5.3 Petal Components Pattern (BEM-like Interpolation)

Petal Components uses a different strategy: attr values interpolate directly into class names following a BEM convention, with actual styles defined in a separate CSS layer:

```elixir
# Component generates class names like:
#   pc-button pc-button--primary pc-button--md pc-button--solid
color_class = "pc-button--#{String.replace(color, "_", "-")}"
size_class  = "pc-button--#{size}"
variant_class = if variant == "solid", do: "", else: "pc-button--#{variant}"
```

```css
/* CSS defines what those classes mean */
@layer components {
  .pc-button--primary { @apply bg-primary-600 text-white; }
  .pc-button--primary-outline { @apply border-primary-600 text-primary-600 bg-transparent; }
  .pc-button--md { @apply h-10 px-4 text-sm; }
  .pc-button--lg { @apply h-12 px-6 text-base; }
}
```

**Trade-off:** The BEM approach centralizes variant styling in CSS (easy to theme via CSS overrides) but adds a layer of indirection between the component and its actual visual output. The inline map approach (section 5.2) keeps everything in Elixir but requires recompilation for styling changes.

### 5.4 Stitches Variant API (Historical Reference)

Stitches (now unmaintained but influential) pioneered first-class variant APIs in CSS-in-JS:

```typescript
const Button = styled("button", {
  // base styles
  padding: "10px 16px",
  borderRadius: "4px",

  variants: {
    color: {
      primary: { backgroundColor: "blue", color: "white" },
      secondary: { backgroundColor: "gray", color: "black" },
    },
    size: {
      sm: { fontSize: "12px" },
      lg: { fontSize: "18px" },
    },
  },
  compoundVariants: [
    { color: "primary", size: "lg", css: { fontWeight: "bold" } },
  ],
  defaultVariants: {
    color: "primary",
    size: "sm",
  },
});
```

Stitches automatically generated TypeScript types from variants, avoiding the prop-type drift that plagues manually typed component libraries. CVA inherited this design philosophy.

---

## 6. Anti-Patterns

### 6.1 Boolean Prop Explosion

**The problem:**
```jsx
<Button isLarge isPrimary isOutline isLoading isDisabled isFullWidth />
```

Booleans do not compose. What is `isLarge && isSmall`? What is `isPrimary && isSecondary`? Every boolean doubles the component's state space, most combinations are meaningless, and there is no type-level prevention of conflicts.

**The fix:** Replace boolean groups with constrained enum props:
```jsx
<Button size="lg" color="primary" variant="outline" loading fullWidth disabled />
```

Keep booleans only for truly binary states (`disabled`, `loading`, `fullWidth`) that are independent of each other.

### 6.2 Premature Abstraction

**The problem:** Building a `<GenericLayout>` component before you have three layouts. Building an `<AbstractForm>` before you have three forms. Building a "universal card" that handles 12 different content shapes via props.

**The damage:** Every new use case requires changing the abstraction, adding props, adding conditionals. The abstraction becomes harder to understand than the duplication it replaced. Eventually no one dares touch it because everything depends on it.

**The fix:** Apply the Rule of Three. Allow duplication until you have three concrete cases, then extract the common pattern. The abstraction you build from three real examples will be dramatically better than the one you guess from one.

### 6.3 Prop Drilling Hell

**The problem:** Passing props through 4+ levels of intermediate components that do not use them:
```jsx
<Page user={user}>
  <Sidebar user={user}>
    <UserPanel user={user}>
      <Avatar user={user} />
    </UserPanel>
  </Sidebar>
</Page>
```

**The fix:**
- **React:** Context API, Zustand, or component composition (pass `<Avatar />` as children)
- **Vue:** provide/inject
- **Phoenix/LiveView:** Assigns are scoped to the LiveView; child function components receive only what they need. For deeply nested state, use `assign` in the LiveView and pass specific values to each component. For truly global state, use `Phoenix.PubSub` or a process-based store.

### 6.4 Over-Configuration (The God Component)

**The problem:**
```jsx
<DataDisplay
  type="table"
  columns={cols}
  data={rows}
  sortable
  filterable
  paginated
  selectable
  expandable
  groupBy="category"
  renderHeader={fn}
  renderRow={fn}
  renderFooter={fn}
  renderEmpty={fn}
  onSort={fn}
  onFilter={fn}
  onSelect={fn}
  onExpand={fn}
  // ... 25 more props
/>
```

**The damage:** The component is impossible to type correctly, impossible to document, impossible to test exhaustively, and impossible for a new developer to learn in under an hour.

**The fix:** Decompose into composed primitives:
```jsx
<Table data={rows}>
  <Table.Header>
    <Table.SortableColumn field="name">Name</Table.SortableColumn>
  </Table.Header>
  <Table.Body renderRow={(row) => <Table.Row key={row.id}>{...}</Table.Row>} />
  <Table.Pagination />
</Table>
```

### 6.5 Wrapper Component Proliferation

**The problem:** Creating thin wrappers around library components that add no real value:
```jsx
// MyButton.tsx -- adds nothing
const MyButton = (props) => <ChakraButton {...props} />;

// MyModal.tsx -- adds nothing
const MyModal = (props) => <RadixDialog {...props} />;
```

50 wrapper files that do nothing but forward props, each requiring maintenance when the upstream API changes.

**The fix:** If you are using a component library, use it directly. If you need to customize, fork the component (shadcn/ui model) or build a genuinely different component. Wrappers are justified only when they add real value: default props that your app always needs, domain-specific behavior, or an abstraction boundary for swapping the underlying library later.

### 6.6 Conditional Rendering Spaghetti

**The problem:**
```elixir
def component(assigns) do
  ~H"""
  <div>
    <%= if @type == "card" do %>
      <div class="card">
        <%= if @show_header do %>
          <div class="header"><%= @title %></div>
        <% end %>
        <%= if @variant == "detailed" do %>
          <div class="details"><%= @details %></div>
        <% else %>
          <div class="summary"><%= @summary %></div>
        <% end %>
      </div>
    <% else %>
      <!-- completely different markup -->
    <% end %>
  </div>
  """
end
```

**The fix:** If the markup diverges significantly based on a prop, these are different components. Make them separate function components and dispatch at the call site, not inside a single template.

---

## 7. Framework-Specific Guidance

### 7.1 Phoenix / LiveView (Primary Target)

Phoenix function components are inherently well-suited to DRY patterns because:
- `attr` with `values:` provides compile-time variant validation
- Named slots with slot attributes provide compile-time composition validation
- `:global` rest attributes handle HTML extensibility
- HEEx templates are simple functions---no class hierarchy, no lifecycle complexity

**Component structure convention:**

```
lib/my_app_web/
  components/
    core_components.ex      # App-wide primitives (button, input, card, badge)
    layouts.ex              # Layout components (page, sidebar, header)
    domain/
      seat_roster.ex        # Domain-specific: chapter seat grid
      visitor_pipeline.ex   # Domain-specific: visitor funnel visualization
```

**Pattern for configurable components:**

```elixir
defmodule MyAppWeb.CoreComponents do
  use Phoenix.Component

  # --- Badge ---

  attr :color, :string,
    default: "gray",
    values: ~w(gray primary success warning danger info)
  attr :size, :string, default: "md", values: ~w(sm md lg)
  attr :class, :string, default: nil
  attr :rest, :global
  slot :inner_block, required: true

  def badge(assigns) do
    ~H"""
    <span
      class={[
        "inline-flex items-center rounded-full font-medium",
        badge_size_classes(@size),
        badge_color_classes(@color),
        @class
      ]}
      {@rest}
    >
      {render_slot(@inner_block)}
    </span>
    """
  end

  defp badge_size_classes("sm"), do: "px-2 py-0.5 text-xs"
  defp badge_size_classes("md"), do: "px-2.5 py-0.5 text-sm"
  defp badge_size_classes("lg"), do: "px-3 py-1 text-base"

  defp badge_color_classes("gray"), do: "bg-gray-100 text-gray-700"
  defp badge_color_classes("primary"), do: "bg-primary-100 text-primary-700"
  defp badge_color_classes("success"), do: "bg-green-100 text-green-700"
  defp badge_color_classes("warning"), do: "bg-yellow-100 text-yellow-700"
  defp badge_color_classes("danger"), do: "bg-red-100 text-red-700"
  defp badge_color_classes("info"), do: "bg-blue-100 text-blue-700"
end
```

**Why pattern matching for variants:** Elixir's multi-clause function pattern is the natural CVA equivalent. It is:
- Exhaustive (the compiler warns if you miss a clause)
- Fast (compiled to a jump table)
- Clear (each variant is a separate, readable clause)
- Extensible (add a clause, not a conditional branch)

**When to use Petal Components vs custom:** Use Petal for standard UI (buttons, forms, modals, tables, alerts). Build custom components only for domain-specific UI that Petal does not cover. Never rebuild what Petal provides.

**LiveComponent vs Function Component:**
- **Function component** (stateless): Use for all presentational components. Buttons, badges, cards, layouts. These are the DRY workhorses.
- **LiveComponent** (stateful): Use only when a component needs its own state and event handling independent of the parent LiveView. Examples: a search autocomplete, an inline editor, a chat widget.

### 7.2 React

**Key patterns:**
- `forwardRef` for all components that wrap DOM elements (enables ref forwarding to the underlying element)
- Compound components with Context API for multi-part interactive components
- CVA for variant management
- `cn()` utility (clsx + tailwind-merge) for class composition
- Polymorphic `as` prop or Radix's `asChild` for element type flexibility

**shadcn/ui component template:**
```tsx
import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva("inline-flex items-center justify-center ...", {
  variants: {
    variant: { default: "...", destructive: "...", outline: "...", ghost: "..." },
    size: { default: "h-10 px-4", sm: "h-9 px-3", lg: "h-11 px-8" },
  },
  defaultVariants: { variant: "default", size: "default" },
});

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => (
    <button
      className={cn(buttonVariants({ variant, size, className }))}
      ref={ref}
      {...props}
    />
  )
);
Button.displayName = "Button";

export { Button, buttonVariants };
```

### 7.3 Vue

**Key patterns:**
- Named slots for content composition
- Scoped slots for passing data back to the parent
- `provide` / `inject` for compound component state sharing (Vue's equivalent of React Context)
- `v-bind="$attrs"` for attribute forwarding (equivalent of `{@rest}`)

```vue
<!-- BaseButton.vue -->
<script setup>
defineProps({
  variant: { type: String, default: 'solid', validator: v => ['solid','outline','ghost'].includes(v) },
  size: { type: String, default: 'md', validator: v => ['sm','md','lg'].includes(v) },
});
</script>

<template>
  <button :class="['btn', `btn-${variant}`, `btn-${size}`]" v-bind="$attrs">
    <slot />
  </button>
</template>
```

### 7.4 Svelte

**Key patterns:**
- Named slots (`<slot name="header" />`) for structural composition
- `$$restProps` for attribute forwarding
- `$$slots` for conditional slot rendering

```svelte
<!-- Button.svelte -->
<script>
  export let variant = 'solid';
  export let size = 'md';
</script>

<button class="btn btn-{variant} btn-{size}" {...$$restProps}>
  {#if $$slots.icon}
    <span class="btn-icon"><slot name="icon" /></span>
  {/if}
  <slot />
</button>
```

---

## 8. Real-World Library Architectures

### 8.1 shadcn/ui -- Copy-Paste Ownership

**Architecture:** Not a library. A CLI that copies component source code into your project.

**Layers:**
1. Radix UI primitives (behavior, accessibility)
2. CVA (variant configuration)
3. Tailwind (utility styling)
4. CSS variables (design tokens)
5. `cn()` (class merging)

**What makes it effective:**
- Full source ownership -- you can modify anything
- No version lock-in -- components evolve with your project
- Consistent patterns -- every component follows the same template (forwardRef + CVA + cn + Radix)
- Good defaults -- works out of the box, customizable by changing CSS variables

**Trade-off:** No automatic updates. When Radix ships a fix, you must manually update your copy. This is the cost of ownership.

### 8.2 Radix UI -- Unstyled Primitives

**Architecture:** 32+ headless primitives shipping behavior and accessibility with zero styles.

**Key design decisions:**
- Each component is split into granular parts (`Dialog.Root`, `Dialog.Trigger`, `Dialog.Portal`, `Dialog.Overlay`, `Dialog.Content`, `Dialog.Title`, `Dialog.Description`, `Dialog.Close`)
- `asChild` prop for composition instead of wrapper DOM nodes
- WAI-ARIA compliance built in
- Controlled and uncontrolled modes for every stateful component

**What makes it effective:**
- Separates the genuinely hard part (accessibility, focus management, keyboard nav) from the easy part (styling)
- Granular parts allow fine-grained styling control
- Works with any styling solution

**Trade-off:** More verbose than styled libraries. You must provide all styles yourself. This is the cost of flexibility.

### 8.3 Headless UI (Tailwind Labs)

**Architecture:** Similar philosophy to Radix but with a smaller component set, designed specifically for Tailwind CSS integration.

**Key differences from Radix:**
- Fewer components (focused on the most common interactive patterns)
- Transition component built in
- Official React and Vue packages (Radix is React-only)
- Tighter Tailwind integration in documentation and examples

### 8.4 Chakra UI -- Styled with Escape Hatches

**Architecture:** Fully styled components with a theme system and style props.

**Variant system:**
```jsx
<Button colorScheme="blue" size="lg" variant="outline">Click</Button>
```

**What makes it effective:**
- Theme-first: all styling derives from a customizable theme object
- Style props: `<Box p={4} bg="gray.100">` -- CSS as props
- Custom variant definitions per component in the theme

**Trade-off:** Opinionated styling. Fighting the theme system is painful. The style-props approach generates large runtime overhead.

### 8.5 Ant Design -- Enterprise Configuration

**Architecture:** Complete, opinionated component set for enterprise applications. 50+ components with extensive configuration.

**What makes it effective for enterprise:**
- Enormous component breadth (Table, Tree, DatePicker, Form, Upload, etc.)
- Built-in form validation and complex data management
- ConfigProvider for global theming

**Trade-off:** Fighting the default styling is difficult. The design is unmistakably "Ant Design." Less suited for consumer-facing products where brand differentiation matters.

### 8.6 Petal Components -- LiveView-Native

**Architecture:** HEEX components styled with Tailwind, built for Phoenix LiveView.

**Variant system:** BEM-like class generation from `attr` values, with actual styles in a CSS layer:
```elixir
attr :size, :string, default: "md", values: ["xs", "sm", "md", "lg", "xl"]
attr :color, :string, default: "primary", values: ["primary", "secondary", ...]
attr :variant, :string, default: "solid", values: ["solid", "outline", ...]
```

**What makes it effective for Phoenix:**
- Native HEEx -- no JavaScript component layer
- Works with LiveView.JS and Alpine.js
- Tailwind-native styling
- Compile-time attr validation

**Trade-off:** Smaller component set than React ecosystems. Some components require Alpine.js or LiveView.JS for interactivity.

---

## 9. Audit Dimensions

Use these dimensions to evaluate whether a codebase's component architecture follows DRY principles effectively.

### 9.1 Variant Consistency

| Question | Good | Bad |
|----------|------|-----|
| How are variants defined? | Enum props with `values` constraint | Boolean props or unconstrained strings |
| Are variant names consistent across components? | `size="md"` everywhere | `size="medium"` here, `sz="m"` there |
| Do similar components use the same variant options? | Button, Badge, Input all use `sm/md/lg` | Button uses `small/medium/large`, Badge uses `s/m/l` |

### 9.2 Token Usage

| Question | Good | Bad |
|----------|------|-----|
| Do components reference tokens? | `bg-primary`, `var(--color-primary)` | `bg-blue-600`, `#3B82F6` |
| Is theming possible without touching components? | Change CSS variables or Tailwind theme | Grep-and-replace hex codes |
| Are spacing values from the scale? | `p-4`, `gap-2`, `mt-6` | `p-[17px]`, `gap-[0.35rem]` |

### 9.3 Composition Quality

| Question | Good | Bad |
|----------|------|-----|
| Do components accept `@inner_block` / children? | Content via slots | Content only via string props |
| Can components be nested freely? | Composable primitives | Rigid single-use layouts |
| Are named slots used for structured regions? | `<:header>`, `<:footer>`, `<:action>` | `header_text`, `footer_html` props |
| Do components accept global/rest attributes? | `{@rest}` / `{...rest}` | Fixed set of declared attributes only |

### 9.4 Duplication Detection

| Question | Good | Bad |
|----------|------|-----|
| Are there near-duplicate components? | One `button` with variants | `PrimaryButton`, `SecondaryButton`, `DangerButton` |
| Are class strings repeated across components? | Shared variant helper or token classes | Same `"rounded-lg border p-4 shadow"` in 12 files |
| Is conditional rendering minimal? | Pattern-matched variant functions | 6 nested `if/else` branches in one template |

### 9.5 API Surface

| Question | Good | Bad |
|----------|------|-----|
| How many props does the most complex component have? | Under 10 (excluding globals) | 20+ props |
| Are required vs optional props clear? | `required: true` on essentials | Everything optional with unclear defaults |
| Is the component self-documenting? | `values:` constraints, `doc:` strings | Magic strings with no validation |

### 9.6 Accessibility

| Question | Good | Bad |
|----------|------|-----|
| Do interactive components use established primitives? | Radix/Headless UI for dialogs, menus, tabs | Custom div with onClick for dropdown |
| Are ARIA attributes handled by the component? | Built into the component | Consumer must remember to add them |
| Is keyboard navigation supported? | Tab, Enter, Escape, Arrow keys work | Mouse-only interaction |

### 9.7 Maintenance Cost

| Question | Good | Bad |
|----------|------|-----|
| Can a design change be made in one place? | Token change or variant helper update | Touch 20+ files |
| Can a new variant be added without modifying existing code? | Add a clause / extend the config | Modify conditionals throughout |
| How many files change for a "simple" UI update? | 1-2 (token or variant definition) | 10+ component files |

---

## Summary: Decision Framework

When building or evaluating a component:

1. **Does Petal (or your component library) already provide this?** If yes, use it. Do not rebuild.
2. **Is this a standard interactive pattern** (dialog, dropdown, tabs, combobox)? Use an established headless primitive for behavior. Style it yourself.
3. **Does this component need variants?** Define them as enum attrs with constrained `values`. Map to classes via pattern-matched functions (Elixir) or CVA (JavaScript).
4. **Does this component need flexible content regions?** Use named slots, not string/HTML props.
5. **Does this component need to render as different elements?** Use conditional rendering (LiveView) or polymorphic props (React).
6. **Are you building this for the third time?** Now extract the abstraction. Not before.
7. **Does every visual value reference a token?** If you see a hex code or pixel value in a component, extract it to a token.

---

## Sources

### Component Libraries
- [shadcn/ui](https://ui.shadcn.com/) -- Copy-paste component collection
- [Radix UI Primitives](https://www.radix-ui.com/primitives) -- Headless accessible primitives
- [Headless UI](https://headlessui.com/) -- Tailwind Labs unstyled components
- [Petal Components](https://petal.build/components) -- Phoenix LiveView HEEX components
- [Chakra UI](https://v2.chakra-ui.com/) -- Styled React component library
- [Ant Design](https://ant.design/) -- Enterprise React components

### Variant Tools
- [Class Variance Authority (CVA)](https://cva.style/docs) -- Declarative variant configuration
- [Stitches Variants](https://stitches.dev/docs/variants) -- CSS-in-JS variant system (historical reference)

### Design Tokens
- [W3C Design Tokens Community Group](https://www.designtokens.org/) -- Specification (2025.10 stable)
- [Style Dictionary](https://styledictionary.com/) -- Multi-platform token transformer

### Patterns & Architecture
- [The Anatomy of shadcn/ui](https://manupa.dev/blog/anatomy-of-shadcn-ui) -- Architectural deep dive
- [Headless Component Pattern (Martin Fowler)](https://martinfowler.com/articles/headless-component.html) -- Separation of logic and presentation
- [Compound Pattern (patterns.dev)](https://www.patterns.dev/react/compound-pattern/) -- Compound component implementation
- [Radix Composition Guide](https://www.radix-ui.com/primitives/docs/guides/composition) -- `asChild` pattern
- [Phoenix.Component (HexDocs)](https://hexdocs.pm/phoenix_live_view/Phoenix.Component.html) -- Official attr/slot documentation
- [Function Components and Slots (Fly.io)](https://fly.io/phoenix-files/function-components/) -- Phoenix-specific patterns
- [Customizable Classes in LV Components (Fly.io)](https://fly.io/phoenix-files/customizable-classes-lv-component/) -- Class extension patterns
- [Avoiding Premature Abstraction (Build UI)](https://buildui.com/posts/avoiding-premature-abstraction-with-unstyled-react-components) -- Anti-pattern analysis
- [Polymorphic Components with forwardRef](https://www.benmvp.com/blog/forwarding-refs-polymorphic-react-component-typescript/) -- TypeScript patterns
- [Tailwind Design Tokens Guide](https://nicolalazzari.ai/articles/integrating-design-tokens-with-tailwind-css) -- Token integration
- [How Radix Changed Component Thinking](https://blog.rad-ui.com/blog/frontend-architecture/how-radix-changed-way-we-think-of-components) -- Architectural philosophy
