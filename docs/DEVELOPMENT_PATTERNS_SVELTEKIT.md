# Development Patterns — SvelteKit Stack

SvelteKit / Svelte 5 / Tailwind CSS / TypeScript

This document captures stack-specific patterns, conventions, and decisions for SvelteKit projects (SvelteKit 2+, Svelte 5 with runes, Tailwind CSS, TypeScript). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure SvelteKit routes, use Svelte 5 runes, test with Vitest + Playwright, choose component libraries, deploy with adapters, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| SvelteKit | 2.x | Full-stack framework, file-based routing |
| Svelte | 5.x | Runes reactivity model (`$state`, `$derived`, `$effect`) |
| TypeScript | 5.x | Strict mode enabled |
| Tailwind CSS | 4.x | CSS-based config (not JS-based from v3) |
| Vite | 6.x | Build tool, bundled with SvelteKit |
| Vitest | 3.x | Unit + integration testing |
| Playwright | 1.x | E2E / browser testing |
| Node.js | 22.x LTS | Runtime |
| pnpm | 9.x | Package manager (preferred over npm/yarn) |

### Component Library Options

Choose one per project based on requirements:

| Library | Best For | Notes |
|---|---|---|
| shadcn-svelte | Custom design systems, maximum control | Copies components into your project — you own the code. Uses Bits UI primitives. |
| Skeleton | Opinionated full-featured apps | Theming system, design tokens, Tailwind plugin. |
| Flowbite-Svelte | Rapid prototyping, admin dashboards | Pre-built components, Flowbite ecosystem. |
| Bits UI | Headless/unstyled primitives | Accessibility-first, style yourself. Foundation for shadcn-svelte. |
| Melt UI | Headless builder pattern | Lower-level than Bits UI, maximum flexibility. |

**Convention:** Pick one library at project start. Do not mix multiple component libraries in the same project — their CSS strategies conflict and maintenance burden multiplies.

### Version Constraint Policy

Use exact or tilde ranges in `package.json`:

```json
{
  "dependencies": {
    "@sveltejs/kit": "^2.16.0",
    "svelte": "^5.20.0"
  },
  "devDependencies": {
    "@sveltejs/adapter-auto": "^4.0.0",
    "tailwindcss": "^4.0.0",
    "vitest": "^3.0.0",
    "@playwright/test": "^1.50.0"
  }
}
```

```
# Good — allows patch and minor updates within major
"svelte": "^5.20.0"

# Good — allows patch updates only
"svelte": "~5.20.0"

# Bad — too loose, allows any version
"svelte": "*"

# Bad — too tight for most deps, blocks patches
"svelte": "5.20.2"
```

Exception: for pre-release or known-unstable packages, pin exact.

Use `pnpm-lock.yaml` for reproducible installs. Never delete the lockfile to "fix" dependency issues — resolve the conflict properly.

---

## 2. Project Structure

### SvelteKit Route Organization

```
project-root/
├── src/
│   ├── app.html                     # HTML shell
│   ├── app.css                      # Global styles (Tailwind imports)
│   ├── app.d.ts                     # Global type declarations
│   ├── hooks.server.ts              # Server hooks (auth, logging, security headers)
│   ├── hooks.client.ts              # Client hooks (error handling)
│   ├── lib/
│   │   ├── components/              # Reusable UI components
│   │   │   ├── ui/                  # Generic UI (Button, Card, Modal, etc.)
│   │   │   ├── forms/               # Form components (ContactForm, LoginForm)
│   │   │   └── layout/              # Layout components (Header, Footer, Sidebar)
│   │   ├── server/                  # Server-only code ($lib/server/*)
│   │   │   ├── db.ts                # Database client
│   │   │   ├── auth.ts              # Auth utilities
│   │   │   └── email.ts             # Email service
│   │   ├── stores/                  # Svelte stores (global state)
│   │   ├── utils/                   # Pure utility functions
│   │   ├── types/                   # Shared TypeScript types
│   │   └── index.ts                 # Barrel exports for $lib
│   ├── params/                      # Route param matchers
│   │   └── slug.ts                  # e.g., export const match = (p) => /^[a-z0-9-]+$/.test(p)
│   └── routes/
│       ├── +layout.svelte           # Root layout
│       ├── +layout.server.ts        # Root layout data (auth session, etc.)
│       ├── +page.svelte             # Homepage
│       ├── +page.server.ts          # Homepage data
│       ├── +error.svelte            # Global error page
│       ├── (marketing)/             # Route group — no URL segment
│       │   ├── about/
│       │   │   └── +page.svelte
│       │   ├── pricing/
│       │   │   └── +page.svelte
│       │   └── +layout.svelte       # Marketing layout (different nav)
│       ├── (app)/                   # Route group — authenticated app
│       │   ├── dashboard/
│       │   │   ├── +page.svelte
│       │   │   └── +page.server.ts
│       │   ├── settings/
│       │   │   ├── +page.svelte
│       │   │   └── +page.server.ts
│       │   └── +layout.server.ts    # Auth guard for all (app) routes
│       └── api/                     # API routes
│           └── v1/
│               ├── users/
│               │   └── +server.ts
│               └── webhooks/
│                   └── +server.ts
├── static/                          # Static assets (favicon, robots.txt, images)
├── tests/                           # Playwright E2E tests
│   ├── e2e/
│   │   ├── auth.spec.ts
│   │   ├── dashboard.spec.ts
│   │   └── marketing.spec.ts
│   └── fixtures/
│       └── test-data.ts
├── svelte.config.js                 # SvelteKit config
├── vite.config.ts                   # Vite config (Vitest integration)
├── tailwind.config.ts               # Tailwind config (v3) or app.css (v4)
├── tsconfig.json                    # TypeScript config
├── playwright.config.ts             # Playwright config
└── package.json
```

**Conventions:**
- Use **route groups** `(groupName)` to share layouts without affecting URLs
- Use **`$lib`** alias for all imports from `src/lib/` — never use relative paths like `../../lib/`
- Server-only code lives in `$lib/server/` — SvelteKit enforces this boundary at build time
- One component per file. File name matches component name: `Button.svelte`, `UserCard.svelte`
- Co-locate route-specific components in the route directory if used nowhere else
- API routes go under `src/routes/api/` — use `+server.ts` files

### Test Mirror Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── ui/
│   │   │   ├── Button.svelte
│   │   │   └── Button.test.ts       # Co-located unit test
│   │   └── forms/
│   │       ├── LoginForm.svelte
│   │       └── LoginForm.test.ts
│   ├── utils/
│   │   ├── format.ts
│   │   └── format.test.ts           # Co-located unit test
│   └── server/
│       ├── auth.ts
│       └── auth.test.ts
tests/
├── e2e/                             # Playwright E2E (separate from src/)
│   ├── auth.spec.ts
│   └── dashboard.spec.ts
└── integration/                     # Integration tests (API routes, load functions)
    ├── api.test.ts
    └── load-functions.test.ts
```

**Convention:** Unit tests are co-located with source files (`.test.ts` next to `.ts`/`.svelte`). E2E tests live in `tests/e2e/`. Integration tests live in `tests/integration/`.

---

## 3. Svelte 5 Runes

### The Reactivity Model

Svelte 5 replaces the implicit reactivity of Svelte 4 (`$:` labels, `let` declarations) with explicit **runes** — compiler-level primitives that make reactivity visible and predictable.

**Core runes:**

| Rune | Purpose | Replaces (Svelte 4) |
|---|---|---|
| `$state` | Declare reactive state | `let x = 0` at top level |
| `$state.raw` | Non-deeply-reactive state (for large objects) | N/A |
| `$derived` | Computed value from other reactive values | `$: doubled = count * 2` |
| `$derived.by` | Computed value with complex logic (function body) | `$: { ... }` blocks |
| `$effect` | Side effects that run when dependencies change | `$: { ... }` side-effect blocks |
| `$effect.pre` | Effect that runs before DOM updates | `beforeUpdate()` lifecycle |
| `$props` | Declare component props | `export let prop` |
| `$bindable` | Declare a prop as bindable | `export let prop` + `bind:prop` |
| `$inspect` | Debug reactive values (dev only, stripped in prod) | `$: console.log(x)` |

### $state — Reactive State

```svelte
<script lang="ts">
  let count = $state(0);
  let user = $state<User | null>(null);
  let items = $state<string[]>([]);

  function increment() {
    count++;  // Direct mutation — runes make this reactive
  }

  function addItem(item: string) {
    items.push(item);  // Array mutations are reactive with $state
  }
</script>

<button onclick={increment}>Count: {count}</button>
```

**Key behavior:** `$state` creates deeply reactive state. Mutating nested properties of objects and arrays triggers updates. For large data structures where deep reactivity adds overhead, use `$state.raw` instead.

```svelte
<script lang="ts">
  // Deep reactivity — every nested property is tracked
  let form = $state({ name: '', email: '', address: { city: '', zip: '' } });
  form.address.city = 'Denver';  // This triggers a re-render

  // Raw state — only reassignment triggers updates, not mutation
  let largeDataset = $state.raw<DataPoint[]>(fetchInitialData());
  // largeDataset.push(item)  // This does NOT trigger update
  largeDataset = [...largeDataset, item];  // This does
</script>
```

### $derived — Computed Values

```svelte
<script lang="ts">
  let items = $state<Item[]>([]);
  let filter = $state('all');

  // Simple derivation — expression
  let total = $derived(items.length);
  let hasItems = $derived(items.length > 0);

  // Complex derivation — function body
  let filteredItems = $derived.by(() => {
    if (filter === 'all') return items;
    return items.filter(item => item.status === filter);
  });

  // Derived from derived — chains work naturally
  let filteredCount = $derived(filteredItems.length);
</script>

<p>{filteredCount} of {total} items shown</p>
```

**Rule:** If a value can be computed from other state, it must be `$derived`, not `$state`. Do not manually synchronize state — let the compiler track dependencies.

### $effect — Side Effects

```svelte
<script lang="ts">
  let searchQuery = $state('');
  let results = $state<SearchResult[]>([]);

  // Runs whenever searchQuery changes (after DOM update)
  $effect(() => {
    if (searchQuery.length < 3) {
      results = [];
      return;
    }

    const controller = new AbortController();

    fetch(`/api/search?q=${encodeURIComponent(searchQuery)}`, {
      signal: controller.signal
    })
      .then(r => r.json())
      .then(data => { results = data; })
      .catch(() => {});

    // Cleanup function — runs before next effect execution or on unmount
    return () => controller.abort();
  });
</script>
```

**Rules for `$effect`:**
1. Never write to `$state` that is read by the same effect — this causes infinite loops
2. Use the return function for cleanup (event listeners, timers, abort controllers)
3. Prefer `$derived` over `$effect` when the goal is computing a value — effects are for side effects (DOM manipulation, API calls, subscriptions)
4. `$effect` runs in the browser only — it does not run during SSR
5. `$effect.pre` runs before DOM updates — use it for things like preserving scroll position

### $props — Component Props

```svelte
<!-- Button.svelte -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    onclick?: (e: MouseEvent) => void;
    children: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    onclick,
    children
  }: Props = $props();
</script>

<button
  class="btn btn-{variant} btn-{size}"
  {disabled}
  {onclick}
>
  {@render children()}
</button>
```

**Convention:** Always define a `Props` interface. Use TypeScript for prop types — do not rely on runtime validation alone.

### $bindable — Two-Way Binding Props

```svelte
<!-- TextInput.svelte -->
<script lang="ts">
  interface Props {
    value: string;
    label: string;
    error?: string;
  }

  let { value = $bindable(), label, error }: Props = $props();
</script>

<label>
  {label}
  <input type="text" bind:value />
  {#if error}<span class="text-red-500 text-sm">{error}</span>{/if}
</label>

<!-- Usage -->
<TextInput bind:value={username} label="Username" />
```

### Snippets (Replacing Slots)

Svelte 5 replaces slots with **snippets** — typed, composable content blocks:

```svelte
<!-- Card.svelte -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    children: Snippet;
    footer?: Snippet;
  }

  let { title, children, footer }: Props = $props();
</script>

<div class="card">
  <h2>{title}</h2>
  <div class="card-body">
    {@render children()}
  </div>
  {#if footer}
    <div class="card-footer">
      {@render footer()}
    </div>
  {/if}
</div>

<!-- Usage -->
<Card title="Profile">
  <p>Card content here</p>
  {#snippet footer()}
    <button>Save</button>
  {/snippet}
</Card>
```

**Snippets with parameters:**

```svelte
<!-- List.svelte -->
<script lang="ts" generics="T">
  import type { Snippet } from 'svelte';

  interface Props {
    items: T[];
    row: Snippet<[T, number]>;
    empty?: Snippet;
  }

  let { items, row, empty }: Props = $props();
</script>

{#if items.length === 0}
  {#if empty}
    {@render empty()}
  {:else}
    <p>No items found.</p>
  {/if}
{:else}
  {#each items as item, index}
    {@render row(item, index)}
  {/each}
{/if}

<!-- Usage -->
<List items={users}>
  {#snippet row(user, i)}
    <div class="flex gap-2">
      <span>{i + 1}.</span>
      <span>{user.name}</span>
    </div>
  {/snippet}
  {#snippet empty()}
    <p>No users yet. Invite someone!</p>
  {/snippet}
</List>
```

---

## 4. SvelteKit Routing & Data Loading

### Load Functions

Every route can have a `load` function that runs before the page renders. Two variants:

**Universal load (`+page.ts`)** — runs on server during SSR, then on client during navigation:

```typescript
// src/routes/blog/[slug]/+page.ts
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
  const response = await fetch(`/api/posts/${params.slug}`);
  if (!response.ok) {
    throw error(404, 'Post not found');
  }
  const post = await response.json();
  return { post };
};
```

**Server load (`+page.server.ts`)** — runs only on the server (access DB, secrets, etc.):

```typescript
// src/routes/dashboard/+page.server.ts
import type { PageServerLoad } from './$types';
import { db } from '$lib/server/db';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.user) {
    redirect(303, '/login');
  }

  const stats = await db.query.dashboardStats.findFirst({
    where: eq(dashboardStats.userId, locals.user.id)
  });

  if (!stats) {
    throw error(500, 'Failed to load dashboard data');
  }

  return {
    user: locals.user,
    stats
  };
};
```

**Convention:** Use `+page.server.ts` by default. Only use `+page.ts` (universal) when:
- The data fetch uses the public `fetch` API (no secrets needed)
- You want client-side navigation to avoid a server round-trip
- The data source is a public API or CDN

### Layout Data Inheritance

Data from layout load functions is available to all child routes:

```typescript
// src/routes/(app)/+layout.server.ts
import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ locals }) => {
  if (!locals.user) {
    redirect(303, '/login');
  }

  return {
    user: locals.user,
    notifications: await getNotifications(locals.user.id)
  };
};
```

```svelte
<!-- src/routes/(app)/dashboard/+page.svelte -->
<script lang="ts">
  // data includes both layout data (user, notifications) and page data
  let { data } = $props();
</script>

<h1>Welcome, {data.user.name}</h1>
<p>You have {data.notifications.length} notifications</p>
```

### Streaming with `await`

For slow data sources, stream them to avoid blocking the entire page:

```typescript
// src/routes/dashboard/+page.server.ts
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
  return {
    // Fast data — available immediately
    user: locals.user,
    // Slow data — streamed, does not block initial render
    analytics: getAnalytics(locals.user.id),  // Returns a Promise, not awaited
    recommendations: getRecommendations(locals.user.id)
  };
};
```

```svelte
<!-- +page.svelte -->
<script lang="ts">
  let { data } = $props();
</script>

<h1>Dashboard for {data.user.name}</h1>

{#await data.analytics}
  <div class="animate-pulse h-48 bg-gray-200 rounded" />
{:then analytics}
  <AnalyticsChart {analytics} />
{:catch error}
  <p class="text-red-500">Failed to load analytics.</p>
{/await}
```

### Page Options

Control SSR, CSR, and prerendering per route:

```typescript
// src/routes/blog/[slug]/+page.ts

// Prerender at build time (static pages)
export const prerender = true;

// src/routes/app/dashboard/+page.ts

// Server-side render only (no client hydration — rare)
export const ssr = true;
export const csr = false;

// src/routes/embed/widget/+page.ts

// Client-side only (no SSR — SPAs, embeds)
export const ssr = false;
```

**Convention:** Leave defaults (`ssr: true`, `csr: true`, `prerender: false`) unless you have a specific reason. Prerender marketing pages and blog posts. Never disable SSR for SEO-critical pages.

### Route Parameters and Matchers

```typescript
// src/params/slug.ts — route param matcher
import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (param) => {
  return /^[a-z0-9][a-z0-9-]*[a-z0-9]$/.test(param);
};

// Usage in route: src/routes/blog/[slug=slug]/+page.svelte
// Only matches if param passes the "slug" matcher
```

**Convention:** Always use param matchers for user-facing slugs and IDs. Prevents invalid routes from reaching your load functions.

---

## 5. Form Actions

### Server-Side Form Handling

SvelteKit form actions handle form submissions on the server with progressive enhancement:

```typescript
// src/routes/contact/+page.server.ts
import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { db } from '$lib/server/db';
import { z } from 'zod';

const contactSchema = z.object({
  name: z.string().min(1, 'Name is required').max(100),
  email: z.string().email('Invalid email address'),
  message: z.string().min(10, 'Message must be at least 10 characters').max(5000)
});

export const load: PageServerLoad = async () => {
  return {};
};

export const actions: Actions = {
  default: async ({ request }) => {
    const formData = await request.formData();
    const raw = Object.fromEntries(formData);

    const result = contactSchema.safeParse(raw);

    if (!result.success) {
      return fail(400, {
        data: raw,
        errors: result.error.flatten().fieldErrors
      });
    }

    try {
      await db.insert(contacts).values(result.data);
    } catch (e) {
      return fail(500, {
        data: raw,
        errors: { _form: ['Failed to send message. Please try again.'] }
      });
    }

    redirect(303, '/contact/thank-you');
  }
};
```

```svelte
<!-- src/routes/contact/+page.svelte -->
<script lang="ts">
  import { enhance } from '$app/forms';

  let { form } = $props();
  let loading = $state(false);
</script>

<form
  method="POST"
  use:enhance={() => {
    loading = true;
    return async ({ update }) => {
      loading = false;
      await update();
    };
  }}
  novalidate
>
  <fieldset>
    <legend class="text-lg font-semibold">Contact Us</legend>

    <label for="name">Full Name</label>
    <input
      id="name"
      name="name"
      type="text"
      autocomplete="name"
      value={form?.data?.name ?? ''}
      aria-invalid={form?.errors?.name ? 'true' : undefined}
      aria-describedby={form?.errors?.name ? 'name-error' : undefined}
      required
    />
    {#if form?.errors?.name}
      <p id="name-error" class="text-red-500 text-sm" role="alert">{form.errors.name[0]}</p>
    {/if}

    <label for="email">Email</label>
    <input
      id="email"
      name="email"
      type="email"
      autocomplete="email"
      value={form?.data?.email ?? ''}
      aria-invalid={form?.errors?.email ? 'true' : undefined}
      aria-describedby={form?.errors?.email ? 'email-error' : undefined}
      required
    />
    {#if form?.errors?.email}
      <p id="email-error" class="text-red-500 text-sm" role="alert">{form.errors.email[0]}</p>
    {/if}

    <label for="message">Message</label>
    <textarea
      id="message"
      name="message"
      autocomplete="off"
      aria-invalid={form?.errors?.message ? 'true' : undefined}
      aria-describedby={form?.errors?.message ? 'message-error' : undefined}
      required
    >{form?.data?.message ?? ''}</textarea>
    {#if form?.errors?.message}
      <p id="message-error" class="text-red-500 text-sm" role="alert">{form.errors.message[0]}</p>
    {/if}
  </fieldset>

  {#if form?.errors?._form}
    <div class="bg-red-50 border border-red-200 rounded p-4" role="alert">
      <p class="text-red-700">{form.errors._form[0]}</p>
    </div>
  {/if}

  <button type="submit" class="btn btn-primary h-12" disabled={loading}>
    {#if loading}Sending...{:else}Send Message{/if}
  </button>
</form>
```

### Named Actions

For pages with multiple forms (e.g., login + register):

```typescript
// src/routes/auth/+page.server.ts
export const actions: Actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData();
    const email = data.get('email') as string;
    const password = data.get('password') as string;

    const user = await verifyCredentials(email, password);
    if (!user) {
      return fail(401, { email, loginError: 'Invalid credentials' });
    }

    cookies.set('session', await createSession(user.id), {
      path: '/',
      httpOnly: true,
      sameSite: 'lax',
      secure: true,
      maxAge: 60 * 60 * 24 * 30 // 30 days
    });

    redirect(303, '/dashboard');
  },

  register: async ({ request }) => {
    // ... registration logic
  }
};
```

```svelte
<!-- Two forms on the same page, each targeting a named action -->
<form method="POST" action="?/login" use:enhance>
  <!-- login fields -->
</form>

<form method="POST" action="?/register" use:enhance>
  <!-- registration fields -->
</form>
```

### Progressive Enhancement with `use:enhance`

The `use:enhance` action provides client-side form submission without full page reloads while maintaining form functionality when JavaScript is disabled:

```svelte
<script lang="ts">
  import { enhance } from '$app/forms';
  import { invalidateAll } from '$app/navigation';

  let { form } = $props();
</script>

<!-- Default enhance — handles everything automatically -->
<form method="POST" use:enhance>
  <!-- ... -->
</form>

<!-- Custom enhance — control loading state, optimistic updates, redirects -->
<form
  method="POST"
  use:enhance={({ formData, cancel }) => {
    // Before submission — validate, modify formData, or cancel()
    const name = formData.get('name');
    if (!name) {
      cancel();
      return;
    }

    return async ({ result, update }) => {
      // After response — handle success/failure
      if (result.type === 'success') {
        await invalidateAll();  // Refresh all load functions
      }
      await update();  // Apply default behavior (update form prop, etc.)
    };
  }}
>
  <!-- ... -->
</form>
```

**Convention:** Always use `use:enhance` on forms. Without it, form submission triggers a full page reload. With it, SvelteKit intercepts the submission, sends it via fetch, and updates the page reactively.

### Validation Strategy

Use **Zod** for server-side validation in form actions. This provides:
- Type-safe parsing
- Detailed error messages
- Schema reuse between client and server

```typescript
// $lib/schemas/contact.ts — shared schema
import { z } from 'zod';

export const contactSchema = z.object({
  name: z.string().min(1, 'Name is required').max(100, 'Name too long'),
  email: z.string().email('Please enter a valid email'),
  phone: z.string().regex(/^\+?[\d\s-()]+$/, 'Invalid phone number').optional(),
  message: z.string().min(10, 'Please write at least 10 characters').max(5000, 'Message too long')
});

export type ContactForm = z.infer<typeof contactSchema>;
```

**Superforms alternative:** For projects with many forms, consider `sveltekit-superforms` which provides:
- Automatic client-side validation from Zod schemas
- Form state management
- Tainted field tracking
- Multi-step form support
- Built-in rate limiting

```typescript
// With superforms
import { superValidate, message } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { contactSchema } from '$lib/schemas/contact';

export const load: PageServerLoad = async () => {
  const form = await superValidate(zod(contactSchema));
  return { form };
};

export const actions: Actions = {
  default: async ({ request }) => {
    const form = await superValidate(request, zod(contactSchema));
    if (!form.valid) return fail(400, { form });

    await saveContact(form.data);
    return message(form, 'Message sent successfully!');
  }
};
```

---

## 6. Authentication & Authorization

### Auth Strategy

Use one of these battle-tested libraries. Do NOT roll your own authentication:

| Library | Best For | Notes |
|---|---|---|
| Lucia | Session-based auth, full control | Library, not a framework. You own the schema. Deprecated v3 but patterns live on as `oslo` utilities. |
| Auth.js (SvelteKit) | OAuth providers, minimal setup | `@auth/sveltekit` adapter. Good for "Login with Google/GitHub" flows. |
| Better Auth | Modern full-featured auth | Session management, 2FA, organization support. Growing ecosystem. |
| Supabase Auth | Supabase projects | Built into Supabase client. Row-level security integration. |
| Clerk | Managed auth, fast to ship | Drop-in components, hosted user management. |

### Session-Based Auth Pattern (Lucia-style)

```typescript
// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { validateSession } from '$lib/server/auth';

export const handle: Handle = async ({ event, resolve }) => {
  const sessionId = event.cookies.get('session');

  if (sessionId) {
    const { user, session } = await validateSession(sessionId);
    if (session) {
      event.locals.user = user;
      event.locals.session = session;

      // Refresh session if close to expiry
      if (session.expiresAt.getTime() - Date.now() < 1000 * 60 * 60 * 24 * 15) {
        const newExpiry = new Date(Date.now() + 1000 * 60 * 60 * 24 * 30);
        await extendSession(session.id, newExpiry);
        event.cookies.set('session', session.id, {
          path: '/',
          httpOnly: true,
          sameSite: 'lax',
          secure: true,
          maxAge: 60 * 60 * 24 * 30
        });
      }
    }
  }

  return resolve(event);
};
```

```typescript
// src/app.d.ts — augment SvelteKit's types
declare global {
  namespace App {
    interface Locals {
      user: import('$lib/server/auth').User | null;
      session: import('$lib/server/auth').Session | null;
    }
    interface PageData {
      user: import('$lib/server/auth').User | null;
    }
  }
}

export {};
```

### Auth Guard Pattern

Protect routes at the layout level:

```typescript
// src/routes/(app)/+layout.server.ts
import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ locals }) => {
  if (!locals.user) {
    redirect(303, '/login');
  }

  return {
    user: locals.user
  };
};
```

**Convention:** Never check auth in individual pages if you can guard at the layout level. The `(app)` route group pattern means one auth check covers all authenticated routes.

### Role-Based Access Control

```typescript
// $lib/server/auth.ts
export type Role = 'user' | 'admin' | 'superadmin';

export function requireRole(locals: App.Locals, ...roles: Role[]) {
  if (!locals.user) {
    redirect(303, '/login');
  }
  if (!roles.includes(locals.user.role)) {
    throw error(403, 'Insufficient permissions');
  }
}

// Usage in load function
export const load: PageServerLoad = async ({ locals }) => {
  requireRole(locals, 'admin', 'superadmin');
  // ... admin-only data
};
```

---

## 7. Component Library Patterns

### shadcn-svelte (Recommended for Custom Design Systems)

shadcn-svelte copies components into your project. You own the source code and can modify freely.

**Installation:**

```bash
npx shadcn-svelte@latest init
npx shadcn-svelte@latest add button card dialog input label
```

**Usage:**

```svelte
<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card';
  import * as Dialog from '$lib/components/ui/dialog';
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>Settings</Card.Title>
    <Card.Description>Manage your account settings.</Card.Description>
  </Card.Header>
  <Card.Content>
    <!-- form content -->
  </Card.Content>
  <Card.Footer>
    <Button variant="outline">Cancel</Button>
    <Button>Save Changes</Button>
  </Card.Footer>
</Card.Root>

<Dialog.Root>
  <Dialog.Trigger>
    <Button variant="destructive">Delete Account</Button>
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Are you sure?</Dialog.Title>
      <Dialog.Description>This action cannot be undone.</Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline">Cancel</Button>
      <Button variant="destructive">Delete</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
```

### Skeleton UI

```bash
pnpm add -D @skeletonlabs/skeleton @skeletonlabs/tw-plugin
```

```svelte
<script lang="ts">
  import { AppBar, AppShell, LightSwitch } from '@skeletonlabs/skeleton';
</script>

<AppShell>
  <svelte:fragment slot="header">
    <AppBar>
      <svelte:fragment slot="lead">
        <strong>My App</strong>
      </svelte:fragment>
      <svelte:fragment slot="trail">
        <LightSwitch />
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>

  <slot />
</AppShell>
```

**Note:** Skeleton v2+ is being rebuilt for Svelte 5. Check migration guides before adopting.

### Flowbite-Svelte

```bash
pnpm add -D flowbite-svelte flowbite
```

```svelte
<script lang="ts">
  import { Button, Card, Modal, Toast } from 'flowbite-svelte';

  let showModal = $state(false);
</script>

<Card>
  <h5 class="text-2xl font-bold">Dashboard</h5>
  <p>Welcome back.</p>
  <Button on:click={() => showModal = true}>Open Settings</Button>
</Card>

<Modal bind:open={showModal} title="Settings">
  <p>Modal content here.</p>
</Modal>
```

### Custom Component Pattern

For domain-specific components not covered by any library:

```svelte
<!-- $lib/components/PricingCard.svelte -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    name: string;
    price: number;
    interval: 'month' | 'year';
    popular?: boolean;
    features: string[];
    cta: string;
    onselect: () => void;
    badge?: Snippet;
  }

  let {
    name,
    price,
    interval,
    popular = false,
    features,
    cta,
    onselect,
    badge
  }: Props = $props();

  let formattedPrice = $derived(
    new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 0
    }).format(price)
  );
</script>

<div
  class="rounded-2xl border p-8 {popular ? 'border-primary-500 ring-2 ring-primary-500' : 'border-gray-200'}"
>
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-semibold">{name}</h3>
    {#if badge}
      {@render badge()}
    {/if}
  </div>

  <p class="mt-4">
    <span class="text-4xl font-bold">{formattedPrice}</span>
    <span class="text-gray-500">/{interval}</span>
  </p>

  <ul class="mt-6 space-y-3">
    {#each features as feature}
      <li class="flex items-center gap-2">
        <svg class="h-5 w-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        {feature}
      </li>
    {/each}
  </ul>

  <button
    class="mt-8 w-full rounded-lg px-4 py-3 font-semibold {popular ? 'bg-primary-600 text-white hover:bg-primary-700' : 'bg-gray-100 text-gray-900 hover:bg-gray-200'}"
    onclick={onselect}
  >
    {cta}
  </button>
</div>
```

---

## 8. Server Hooks & Middleware

### hooks.server.ts

The server hooks file is the central middleware for all requests:

```typescript
// src/hooks.server.ts
import type { Handle, HandleServerError } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { validateSession } from '$lib/server/auth';
import { dev } from '$app/environment';

// 1. Security headers
const securityHeaders: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);

  response.headers.set('X-Content-Type-Options', 'nosniff');
  response.headers.set('X-Frame-Options', 'SAMEORIGIN');
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');

  if (!dev) {
    response.headers.set(
      'Strict-Transport-Security',
      'max-age=31536000; includeSubDomains'
    );
    response.headers.set(
      'Content-Security-Policy',
      "default-src 'self'; script-src 'self' 'unsafe-inline'; " +
      "style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; " +
      "font-src 'self'; connect-src 'self' wss:; frame-ancestors 'self'"
    );
  }

  return response;
};

// 2. Authentication
const auth: Handle = async ({ event, resolve }) => {
  const sessionId = event.cookies.get('session');
  event.locals.user = null;
  event.locals.session = null;

  if (sessionId) {
    const result = await validateSession(sessionId);
    if (result.session) {
      event.locals.user = result.user;
      event.locals.session = result.session;
    }
  }

  return resolve(event);
};

// 3. Rate limiting (API routes)
const rateLimit: Handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith('/api/')) {
    const ip = event.getClientAddress();
    const allowed = await checkRateLimit(ip, event.url.pathname);
    if (!allowed) {
      return new Response('Too Many Requests', { status: 429 });
    }
  }

  return resolve(event);
};

// Compose hooks in order
export const handle = sequence(securityHeaders, auth, rateLimit);

// Global server error handler
export const handleError: HandleServerError = async ({ error, event, status, message }) => {
  const errorId = crypto.randomUUID();

  console.error(`[${errorId}] ${status} ${event.url.pathname}:`, error);

  // Report to error tracking service in production
  if (!dev) {
    await reportError({ errorId, error, url: event.url.pathname, status });
  }

  return {
    message: 'An unexpected error occurred.',
    errorId
  };
};
```

### hooks.client.ts

```typescript
// src/hooks.client.ts
import type { HandleClientError } from '@sveltejs/kit';

export const handleError: HandleClientError = async ({ error, event, status, message }) => {
  const errorId = crypto.randomUUID();

  console.error(`[${errorId}] Client error:`, error);

  // Report to error tracking (Sentry, etc.)
  if (typeof window !== 'undefined' && window.Sentry) {
    window.Sentry.captureException(error, {
      extra: { errorId, url: event.url.pathname, status }
    });
  }

  return {
    message: 'Something went wrong.',
    errorId
  };
};
```

---

## 9. API Routes

### REST API Pattern

```typescript
// src/routes/api/v1/users/+server.ts
import type { RequestHandler } from './$types';
import { json, error } from '@sveltejs/kit';
import { db } from '$lib/server/db';
import { z } from 'zod';

const createUserSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email()
});

export const GET: RequestHandler = async ({ url, locals }) => {
  if (!locals.user) {
    throw error(401, 'Unauthorized');
  }

  const page = parseInt(url.searchParams.get('page') ?? '1');
  const limit = Math.min(parseInt(url.searchParams.get('limit') ?? '20'), 100);
  const offset = (page - 1) * limit;

  const [users, total] = await Promise.all([
    db.query.users.findMany({ limit, offset }),
    db.select({ count: count() }).from(usersTable)
  ]);

  return json({
    data: users,
    pagination: { page, limit, total: total[0].count }
  });
};

export const POST: RequestHandler = async ({ request, locals }) => {
  if (!locals.user || locals.user.role !== 'admin') {
    throw error(403, 'Forbidden');
  }

  const body = await request.json();
  const result = createUserSchema.safeParse(body);

  if (!result.success) {
    throw error(400, {
      message: 'Validation failed',
      errors: result.error.flatten().fieldErrors
    });
  }

  const user = await db.insert(usersTable).values(result.data).returning();

  return json({ data: user[0] }, { status: 201 });
};
```

### Webhook Handler Pattern

```typescript
// src/routes/api/webhooks/stripe/+server.ts
import type { RequestHandler } from './$types';
import { error, json } from '@sveltejs/kit';
import { STRIPE_WEBHOOK_SECRET } from '$env/static/private';
import Stripe from 'stripe';

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY!);

export const POST: RequestHandler = async ({ request }) => {
  const body = await request.text();
  const signature = request.headers.get('stripe-signature');

  if (!signature) {
    throw error(400, 'Missing signature');
  }

  let event: Stripe.Event;

  try {
    event = stripe.webhooks.constructEvent(body, signature, STRIPE_WEBHOOK_SECRET);
  } catch (err) {
    console.error('Webhook signature verification failed:', err);
    throw error(400, 'Invalid signature');
  }

  // Process event types
  switch (event.type) {
    case 'checkout.session.completed':
      await handleCheckoutCompleted(event.data.object);
      break;
    case 'customer.subscription.updated':
      await handleSubscriptionUpdated(event.data.object);
      break;
    case 'customer.subscription.deleted':
      await handleSubscriptionDeleted(event.data.object);
      break;
    default:
      console.log(`Unhandled event type: ${event.type}`);
  }

  return json({ received: true });
};
```

**Convention:** Always verify webhook signatures before processing. Return 200/json immediately — process heavy work asynchronously via a job queue if needed.

---

## 10. State Management

### When to Use What

| Mechanism | Scope | Use For |
|---|---|---|
| `$state` in component | Single component | Local UI state (open/closed, form values) |
| `$props` | Parent to child | Passing data down the component tree |
| Load function `data` | Page + children | Server data (DB results, API responses) |
| URL search params | Shareable | Filters, pagination, sort order |
| Svelte context (`setContext`/`getContext`) | Component subtree | Avoiding prop drilling for theme, config |
| Svelte stores (`$lib/stores/`) | App-wide | Auth state, feature flags, global preferences |
| Cookies | Server + client | Session tokens, preferences |

### Svelte 5 Store Pattern (Rune-Based)

Svelte 5 allows creating reactive state outside components using `$state` in `.svelte.ts` files:

```typescript
// $lib/stores/cart.svelte.ts
export function createCart() {
  let items = $state<CartItem[]>([]);

  let total = $derived(
    items.reduce((sum, item) => sum + item.price * item.quantity, 0)
  );

  let count = $derived(
    items.reduce((sum, item) => sum + item.quantity, 0)
  );

  return {
    get items() { return items; },
    get total() { return total; },
    get count() { return count; },

    add(product: Product, quantity = 1) {
      const existing = items.find(i => i.productId === product.id);
      if (existing) {
        existing.quantity += quantity;
      } else {
        items.push({
          productId: product.id,
          name: product.name,
          price: product.price,
          quantity
        });
      }
    },

    remove(productId: string) {
      items = items.filter(i => i.productId !== productId);
    },

    clear() {
      items = [];
    }
  };
}

// Singleton instance for app-wide use
export const cart = createCart();
```

```svelte
<!-- Usage in any component -->
<script lang="ts">
  import { cart } from '$lib/stores/cart.svelte';
</script>

<button onclick={() => cart.add(product)}>
  Add to Cart ({cart.count})
</button>
```

### Context API (Avoiding Prop Drilling)

```svelte
<!-- ThemeProvider.svelte -->
<script lang="ts">
  import { setContext } from 'svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    theme: 'light' | 'dark';
    children: Snippet;
  }

  let { theme, children }: Props = $props();

  setContext('theme', {
    get current() { return theme; }
  });
</script>

{@render children()}

<!-- DeepChild.svelte — no prop drilling needed -->
<script lang="ts">
  import { getContext } from 'svelte';

  const theme = getContext<{ current: 'light' | 'dark' }>('theme');
</script>

<div class={theme.current === 'dark' ? 'bg-gray-900 text-white' : 'bg-white text-gray-900'}>
  Content
</div>
```

### URL State for Shareable Filters

```svelte
<script lang="ts">
  import { page } from '$app/state';
  import { goto } from '$app/navigation';

  let { data } = $props();

  let filter = $derived(page.url.searchParams.get('filter') ?? 'all');
  let sort = $derived(page.url.searchParams.get('sort') ?? 'newest');

  function updateFilter(newFilter: string) {
    const url = new URL(page.url);
    url.searchParams.set('filter', newFilter);
    goto(url.toString(), { replaceState: true, noScroll: true });
  }
</script>
```

**Convention:** Anything the user might want to share via URL (filters, search, pagination, tab selection) must live in URL search params, not component state.

---

## 11. Testing Patterns

### Test Pyramid (SvelteKit-specific)

```
        /\
       /  \          E2E (Playwright)
      /    \         Full browser, real server, critical user paths
     /------\
    /        \        Integration Tests (Vitest + @testing-library/svelte)
   /          \       Component rendering, form actions, load functions
  /------------\
 /              \      Unit Tests (Vitest)
/                \     Pure functions, utilities, schemas, stores
/------------------\
```

### Vitest Configuration

```typescript
// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['src/**/*.test.ts', 'tests/integration/**/*.test.ts'],
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      include: ['src/lib/**/*.ts', 'src/lib/**/*.svelte'],
      exclude: [
        'src/lib/**/*.test.ts',
        'src/lib/components/ui/**',  // shadcn components — tested upstream
        'src/**/*.d.ts'
      ],
      thresholds: {
        statements: 90,
        branches: 85,
        functions: 90,
        lines: 90
      }
    }
  }
});
```

### Unit Tests (Pure Functions)

```typescript
// src/lib/utils/format.test.ts
import { describe, it, expect } from 'vitest';
import { formatCurrency, formatDate, slugify } from './format';

describe('formatCurrency', () => {
  it('formats USD with 2 decimal places', () => {
    expect(formatCurrency(1234.5, 'USD')).toBe('$1,234.50');
  });

  it('handles zero', () => {
    expect(formatCurrency(0, 'USD')).toBe('$0.00');
  });

  it('handles negative values', () => {
    expect(formatCurrency(-50, 'USD')).toBe('-$50.00');
  });
});

describe('slugify', () => {
  it('lowercases and hyphenates', () => {
    expect(slugify('Hello World')).toBe('hello-world');
  });

  it('strips special characters', () => {
    expect(slugify('Hello! @World#')).toBe('hello-world');
  });

  it('collapses multiple hyphens', () => {
    expect(slugify('hello   world')).toBe('hello-world');
  });

  it('trims leading/trailing hyphens', () => {
    expect(slugify(' -hello world- ')).toBe('hello-world');
  });
});
```

### Component Tests

```typescript
// src/lib/components/ui/Button.test.ts
import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders with default props', () => {
    render(Button, { props: { children: 'Click me' } });
    expect(screen.getByRole('button')).toHaveTextContent('Click me');
  });

  it('applies variant classes', () => {
    render(Button, { props: { variant: 'destructive', children: 'Delete' } });
    const button = screen.getByRole('button');
    expect(button.className).toContain('destructive');
  });

  it('handles click events', async () => {
    const onclick = vi.fn();
    render(Button, { props: { onclick, children: 'Click' } });
    await fireEvent.click(screen.getByRole('button'));
    expect(onclick).toHaveBeenCalledOnce();
  });

  it('disables when disabled prop is true', () => {
    render(Button, { props: { disabled: true, children: 'Nope' } });
    expect(screen.getByRole('button')).toBeDisabled();
  });
});
```

### Load Function Tests

```typescript
// tests/integration/dashboard-load.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the db module before importing the load function
vi.mock('$lib/server/db', () => ({
  db: {
    query: {
      dashboardStats: {
        findFirst: vi.fn()
      }
    }
  }
}));

import { load } from '../../src/routes/(app)/dashboard/+page.server';
import { db } from '$lib/server/db';

describe('dashboard load function', () => {
  const mockUser = { id: '1', name: 'Test', role: 'user' as const };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns user and stats for authenticated user', async () => {
    const mockStats = { views: 100, revenue: 5000 };
    vi.mocked(db.query.dashboardStats.findFirst).mockResolvedValue(mockStats);

    const result = await load({
      locals: { user: mockUser, session: { id: 's1' } }
    } as any);

    expect(result).toEqual({
      user: mockUser,
      stats: mockStats
    });
  });

  it('redirects unauthenticated users', async () => {
    await expect(
      load({ locals: { user: null, session: null } } as any)
    ).rejects.toMatchObject({ status: 303, location: '/login' });
  });
});
```

### Form Action Tests

```typescript
// tests/integration/contact-action.test.ts
import { describe, it, expect, vi } from 'vitest';
import { actions } from '../../src/routes/contact/+page.server';

describe('contact form action', () => {
  function createFormData(data: Record<string, string>): Request {
    const formData = new FormData();
    for (const [key, value] of Object.entries(data)) {
      formData.set(key, value);
    }
    return { formData: async () => formData } as unknown as Request;
  }

  it('returns validation errors for empty name', async () => {
    const result = await actions.default({
      request: createFormData({ name: '', email: 'test@example.com', message: 'Hello there!' })
    } as any);

    expect(result?.status).toBe(400);
    expect(result?.data?.errors?.name).toBeDefined();
  });

  it('returns validation errors for invalid email', async () => {
    const result = await actions.default({
      request: createFormData({ name: 'Test', email: 'not-an-email', message: 'Hello there!' })
    } as any);

    expect(result?.status).toBe(400);
    expect(result?.data?.errors?.email).toBeDefined();
  });

  it('succeeds with valid data', async () => {
    // Mock db.insert to succeed
    vi.mocked(db.insert).mockReturnValue({
      values: vi.fn().mockResolvedValue(undefined)
    } as any);

    // Successful submission redirects — SvelteKit throws a redirect
    await expect(
      actions.default({
        request: createFormData({
          name: 'Jane',
          email: 'jane@example.com',
          message: 'Hello, this is a valid message.'
        })
      } as any)
    ).rejects.toMatchObject({ status: 303, location: '/contact/thank-you' });
  });
});
```

### Playwright E2E Tests

```typescript
// playwright.config.ts
import type { PlaywrightTestConfig } from '@playwright/test';

const config: PlaywrightTestConfig = {
  testDir: 'tests/e2e',
  testMatch: '**/*.spec.ts',
  fullyParallel: true,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: process.env.CI ? 'github' : 'html',
  use: {
    baseURL: 'http://localhost:4173',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure'
  },
  webServer: {
    command: 'pnpm run build && pnpm run preview',
    port: 4173,
    reuseExistingServer: !process.env.CI,
    timeout: 120000
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } }
  ]
};

export default config;
```

```typescript
// tests/e2e/auth.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Authentication', () => {
  test('user can log in and see dashboard', async ({ page }) => {
    await page.goto('/login');

    await page.getByLabel('Email').fill('user@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    await expect(page).toHaveURL('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('invalid credentials show error', async ({ page }) => {
    await page.goto('/login');

    await page.getByLabel('Email').fill('user@example.com');
    await page.getByLabel('Password').fill('wrong');
    await page.getByRole('button', { name: 'Log In' }).click();

    await expect(page.getByRole('alert')).toContainText('Invalid credentials');
    await expect(page).toHaveURL('/login');
  });

  test('unauthenticated user is redirected from dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page).toHaveURL('/login');
  });
});

// tests/e2e/contact.spec.ts
test.describe('Contact Form', () => {
  test('submits successfully with valid data', async ({ page }) => {
    await page.goto('/contact');

    await page.getByLabel('Full Name').fill('Jane Smith');
    await page.getByLabel('Email').fill('jane@example.com');
    await page.getByLabel('Message').fill('Hello, I have a question about your product.');
    await page.getByRole('button', { name: 'Send Message' }).click();

    await expect(page).toHaveURL('/contact/thank-you');
    await expect(page.getByText('Message sent')).toBeVisible();
  });

  test('shows validation errors for empty fields', async ({ page }) => {
    await page.goto('/contact');
    await page.getByRole('button', { name: 'Send Message' }).click();

    await expect(page.getByText('Name is required')).toBeVisible();
    await expect(page.getByText('Please enter a valid email')).toBeVisible();
  });

  test('form works without JavaScript (progressive enhancement)', async ({ page }) => {
    // Disable JavaScript
    await page.context().route('**/*', async (route) => {
      if (route.request().resourceType() === 'script') {
        return route.abort();
      }
      return route.continue();
    });

    await page.goto('/contact');
    await page.getByLabel('Full Name').fill('Jane Smith');
    await page.getByLabel('Email').fill('jane@example.com');
    await page.getByLabel('Message').fill('Testing without JS.');
    await page.getByRole('button', { name: 'Send Message' }).click();

    // Should still work — form submits via standard HTTP POST
    await expect(page).toHaveURL('/contact/thank-you');
  });
});
```

### Test Commands

```bash
# Unit + integration tests
pnpm test                              # Run once
pnpm test:unit                         # Vitest only
pnpm test -- --watch                   # Watch mode
pnpm test -- --coverage                # With coverage report

# E2E tests
pnpm test:e2e                          # All browsers
pnpm test:e2e -- --project=chromium    # Single browser
pnpm test:e2e -- --ui                  # Interactive UI mode
pnpm test:e2e -- --debug               # Debug mode (headed + inspector)

# CI — run everything
pnpm test && pnpm test:e2e
```

---

## 12. Deployment & Adapters

### Adapter Selection

| Adapter | Target | When to Use |
|---|---|---|
| `adapter-auto` | Auto-detected | Default. Detects Vercel, Netlify, Cloudflare. Good for getting started. |
| `adapter-vercel` | Vercel | Production Vercel deployments. Supports Edge/Serverless/ISR. |
| `adapter-node` | Node.js server | Self-hosted, Docker, Fly.io, Railway, any VPS. |
| `adapter-cloudflare-workers` | Cloudflare Workers | Edge deployment. Limited Node.js API support. |
| `adapter-cloudflare` | Cloudflare Pages | Static + Functions. Good balance of edge + full stack. |
| `adapter-netlify` | Netlify | Netlify deployments. Serverless functions. |
| `adapter-static` | Static hosting | Fully prerendered sites. No server needed. |

### adapter-node (Self-Hosted / Docker)

```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      out: 'build',
      precompress: true,  // Generate .gz and .br files
      envPrefix: 'APP_'   // Only expose APP_* env vars
    })
  }
};

export default config;
```

**Dockerfile:**

```dockerfile
# Build stage
FROM node:22-alpine AS builder

RUN corepack enable && corepack prepare pnpm@9 --activate

WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm run build
RUN pnpm prune --prod

# Runtime stage
FROM node:22-alpine

RUN addgroup -g 1001 -S appgroup && adduser -S appuser -u 1001

WORKDIR /app
COPY --from=builder --chown=appuser:appgroup /app/build ./build
COPY --from=builder --chown=appuser:appgroup /app/node_modules ./node_modules
COPY --from=builder --chown=appuser:appgroup /app/package.json ./

USER appuser
ENV NODE_ENV=production
ENV PORT=3000
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:3000/api/health || exit 1

CMD ["node", "build/index.js"]
```

### adapter-vercel

```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-vercel';

const config = {
  kit: {
    adapter: adapter({
      runtime: 'nodejs22.x',
      regions: ['iad1'],           // US East
      split: false                  // Single function (simpler)
    })
  }
};
```

Per-route configuration:

```typescript
// src/routes/api/heavy-computation/+server.ts
export const config = {
  runtime: 'nodejs22.x',
  maxDuration: 60  // Allow up to 60 seconds
};

// src/routes/api/fast-edge/+server.ts
export const config = {
  runtime: 'edge'  // Run at the edge
};
```

### adapter-static (Prerendered Sites)

```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-static';

const config = {
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: '404.html',    // SPA fallback (optional)
      precompress: true
    }),
    prerender: {
      entries: ['*'],           // Prerender all discoverable routes
      handleHttpError: 'warn'   // Don't fail build on broken links (configurable)
    }
  }
};
```

**Convention:** Use `adapter-static` only for sites that are fully prerenderable (blogs, marketing sites, documentation). If any route requires server-side logic (auth, form actions, API routes), use `adapter-node` or `adapter-vercel`.

### CI/CD Pipeline (GitHub Actions)

```yaml
name: CI/CD

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'pnpm'

      - run: pnpm install --frozen-lockfile

      # Quality gates
      - run: pnpm run check          # svelte-check (type checking)
      - run: pnpm run lint            # ESLint
      - run: pnpm run format:check    # Prettier
      - run: pnpm run test            # Vitest

      # E2E tests
      - run: pnpm exec playwright install --with-deps
      - run: pnpm run test:e2e

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'pnpm'

      - run: pnpm install --frozen-lockfile
      - run: pnpm run build

      # Deploy step depends on adapter:
      # adapter-vercel: use Vercel GitHub integration (no manual deploy step)
      # adapter-node: push Docker image, deploy to Fly/Railway/etc.
      # adapter-static: deploy to Cloudflare Pages / Netlify / S3+CloudFront
```

---

## 13. Security

### Environment Variables

SvelteKit distinguishes between public and private env vars:

```typescript
// Private — server only, NEVER sent to browser
import { DATABASE_URL, STRIPE_SECRET_KEY } from '$env/static/private';
import { env } from '$env/dynamic/private';

// Public — available in browser, prefixed with PUBLIC_
import { PUBLIC_APP_URL, PUBLIC_POSTHOG_KEY } from '$env/static/public';
import { env as publicEnv } from '$env/dynamic/public';
```

**Convention:**
- Private secrets: `DATABASE_URL`, `STRIPE_SECRET_KEY`, `SESSION_SECRET`
- Public config: `PUBLIC_APP_URL`, `PUBLIC_POSTHOG_KEY`, `PUBLIC_SENTRY_DSN`
- Never import from `$env/static/private` in client-side code — SvelteKit will throw a build error
- Never prefix secrets with `PUBLIC_` — they will be exposed to the browser

### CSRF Protection

SvelteKit has built-in CSRF protection for form actions. It checks the `Origin` header against the host. Do NOT disable it.

```javascript
// svelte.config.js — DO NOT DO THIS
const config = {
  kit: {
    csrf: {
      checkOrigin: false  // NEVER disable this
    }
  }
};
```

For API routes that accept cross-origin requests (webhooks, public APIs), implement your own verification:

```typescript
// src/routes/api/webhooks/+server.ts
export const POST: RequestHandler = async ({ request }) => {
  // Verify webhook signature instead of CSRF
  const signature = request.headers.get('x-webhook-signature');
  const body = await request.text();

  if (!verifySignature(body, signature, WEBHOOK_SECRET)) {
    throw error(401, 'Invalid signature');
  }

  // Process webhook...
};
```

### Content Security Policy

```typescript
// src/hooks.server.ts
const csp: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);

  // SvelteKit handles CSP for inline scripts via nonces when configured
  // Additional CSP headers for defense-in-depth
  const cspDirectives = [
    "default-src 'self'",
    "script-src 'self' 'unsafe-inline'",  // Required for SvelteKit hydration
    "style-src 'self' 'unsafe-inline'",   // Required for Tailwind
    "img-src 'self' data: https:",
    "font-src 'self'",
    "connect-src 'self' wss:",
    "frame-ancestors 'self'",
    "base-uri 'self'",
    "form-action 'self'"
  ].join('; ');

  response.headers.set('Content-Security-Policy', cspDirectives);

  return response;
};
```

### Input Sanitization

```typescript
// $lib/server/sanitize.ts
import DOMPurify from 'isomorphic-dompurify';

// For user-generated content that will be rendered as HTML
export function sanitizeHtml(dirty: string): string {
  return DOMPurify.sanitize(dirty, {
    ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a', 'p', 'br', 'ul', 'ol', 'li'],
    ALLOWED_ATTR: ['href', 'target', 'rel']
  });
}

// For plain text fields — strip HTML entirely
export function sanitizeText(input: string): string {
  return DOMPurify.sanitize(input, { ALLOWED_TAGS: [] });
}
```

**Convention:** Always sanitize user input that will be rendered as HTML. For plain text displayed via `{text}` in Svelte templates, Svelte auto-escapes by default — no extra sanitization needed. Only use `{@html content}` when you have explicitly sanitized the HTML.

---

## 14. Coverage Enforcement

### Vitest Coverage Configuration

```typescript
// vite.config.ts — coverage section
test: {
  coverage: {
    provider: 'v8',
    reporter: ['text', 'text-summary', 'html', 'lcov'],
    reportsDirectory: './coverage',
    include: [
      'src/lib/**/*.ts',
      'src/lib/**/*.svelte',
      'src/routes/**/+page.server.ts',
      'src/routes/**/+server.ts',
      'src/hooks.server.ts'
    ],
    exclude: [
      '**/*.test.ts',
      '**/*.spec.ts',
      '**/*.d.ts',
      'src/lib/components/ui/**',     // shadcn — tested upstream
      'src/app.html',
      'src/hooks.client.ts'           // Client error handler — tested via E2E
    ],
    thresholds: {
      statements: 90,
      branches: 85,
      functions: 90,
      lines: 90
    }
  }
}
```

### Coverage Commands

```bash
# Terminal report
pnpm test -- --coverage

# HTML report (opens in browser)
pnpm test -- --coverage && open coverage/index.html

# CI enforcement — fails if below thresholds
pnpm test -- --coverage  # Vitest exits non-zero if thresholds not met
```

### What to Test vs What to Skip

| Test | Skip |
|---|---|
| Load functions (server + universal) | Generated types (`$types`) |
| Form actions | shadcn/Skeleton/Flowbite components (tested upstream) |
| API route handlers | `app.html` shell |
| Server hooks | Svelte template rendering (test via E2E) |
| Utility functions | Type declarations (`.d.ts`) |
| Store logic | Third-party library wrappers |
| Custom components | Static config files |

**Convention:** Target 90%+ coverage for business logic. 100% is ideal for `$lib/server/` (where your business logic lives). Component visual testing is covered by E2E/Playwright, not unit coverage.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with `<fieldset>` + `<legend>` |
| **labels** | Top-aligned, visible `<label>`, optional fields marked "(optional)" |
| **validation** | Server-side via form actions (Zod), client-side optional (progressive) |
| **errors** | Inline + error summary, multi-cue (icon + text + border), `role="alert"` |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-invalid`, `aria-describedby` |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets (h-12), `autocomplete` |
| **cta** | Outcome-focused text ("Send Message" not "Submit"), loading state via `use:enhance` |
| **trust** | Minimal fields, "(optional)" markers, clear post-submit feedback |
| **performance** | Progressive enhancement (`use:enhance`), no unnecessary client-side validation |

**SvelteKit-specific form pattern:**

```svelte
<script lang="ts">
  import { enhance } from '$app/forms';
  let { form } = $props();
  let loading = $state(false);
</script>

<form
  method="POST"
  use:enhance={() => {
    loading = true;
    return async ({ update }) => {
      loading = false;
      await update();
    };
  }}
  novalidate
>
  <fieldset>
    <legend class="text-sm font-semibold">Your Information</legend>

    <div class="space-y-4">
      <div>
        <label for="name" class="block text-sm font-medium">Full Name</label>
        <input
          id="name"
          name="name"
          type="text"
          autocomplete="name"
          required
          value={form?.data?.name ?? ''}
          class="mt-1 block w-full rounded-lg border px-3 py-3 {form?.errors?.name ? 'border-red-500' : 'border-gray-300'}"
          aria-invalid={form?.errors?.name ? 'true' : undefined}
          aria-describedby={form?.errors?.name ? 'name-error' : undefined}
        />
        {#if form?.errors?.name}
          <p id="name-error" class="mt-1 flex items-center gap-1 text-sm text-red-600" role="alert">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
            </svg>
            {form.errors.name[0]}
          </p>
        {/if}
      </div>

      <div>
        <label for="email" class="block text-sm font-medium">Email</label>
        <input
          id="email"
          name="email"
          type="email"
          autocomplete="email"
          required
          value={form?.data?.email ?? ''}
          class="mt-1 block w-full rounded-lg border px-3 py-3 {form?.errors?.email ? 'border-red-500' : 'border-gray-300'}"
          aria-invalid={form?.errors?.email ? 'true' : undefined}
          aria-describedby={form?.errors?.email ? 'email-error' : undefined}
        />
        {#if form?.errors?.email}
          <p id="email-error" class="mt-1 flex items-center gap-1 text-sm text-red-600" role="alert">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
            </svg>
            {form.errors.email[0]}
          </p>
        {/if}
      </div>

      <div>
        <label for="phone" class="block text-sm font-medium">
          Phone <span class="text-gray-400 font-normal">(optional)</span>
        </label>
        <input
          id="phone"
          name="phone"
          type="tel"
          autocomplete="tel"
          value={form?.data?.phone ?? ''}
          class="mt-1 block w-full rounded-lg border border-gray-300 px-3 py-3"
        />
      </div>
    </div>
  </fieldset>

  <button
    type="submit"
    class="mt-6 w-full rounded-lg bg-primary-600 px-4 py-3 text-white font-semibold hover:bg-primary-700 disabled:opacity-50 h-12"
    disabled={loading}
  >
    {#if loading}Sending...{:else}Send Message{/if}
  </button>
</form>
```

---

## 16. Anti-Patterns

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using Svelte 4 syntax (`export let`, `$:` labels, slots) in Svelte 5 projects | Use runes: `$props()`, `$state`, `$derived`, `$effect`, snippets |
| 2 | Using `$effect` to synchronize state (`$effect(() => { b = a * 2 })`) | Use `$derived`: `let b = $derived(a * 2)` |
| 3 | Mutating `$state` inside `$derived` | `$derived` must be pure — no side effects, no state mutation |
| 4 | Reading and writing the same `$state` in a single `$effect` | This creates infinite loops. Split into separate state or use `$derived` |
| 5 | Fetching data in `onMount` or `$effect` instead of load functions | Use `+page.server.ts` / `+page.ts` load functions for initial data |
| 6 | Putting secrets in `+page.ts` (universal load) | Use `+page.server.ts` for anything touching secrets, DB, or private APIs |
| 7 | Disabling CSRF protection (`csrf: { checkOrigin: false }`) | Keep CSRF enabled. For webhooks, use signature verification instead |
| 8 | Using `{@html userContent}` without sanitization | Always sanitize with DOMPurify before rendering raw HTML |
| 9 | Skipping `use:enhance` on forms | Always use `use:enhance` — without it, form submission reloads the page |
| 10 | Importing from `$env/static/private` in client code | Use `$env/static/public` (with `PUBLIC_` prefix) for client-accessible config |
| 11 | Mixing multiple component libraries (shadcn + Skeleton + Flowbite) | Pick one at project start. They have conflicting CSS and conventions |
| 12 | Using relative imports (`../../lib/utils`) instead of `$lib` | Always use `$lib/utils`, `$lib/components`, `$lib/server` aliases |
| 13 | Putting business logic in `+page.svelte` components | Keep logic in `+page.server.ts` (load/actions) or `$lib/server/`. Components are for presentation |
| 14 | Writing custom auth from scratch | Use Lucia, Auth.js, Better Auth, or another proven library |
| 15 | Forms without `novalidate` attribute | Always add `novalidate` — HTML5 native validation is inconsistent across assistive technologies |
| 16 | Forms without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, `autocomplete="tel"` etc. |
| 17 | "Submit" button text | Use outcome-focused CTA: "Send Message", "Create Account", "Save Changes" |
| 18 | Deploying without quality gates (lint + typecheck + test) | CI must run `svelte-check`, ESLint, Prettier, Vitest, and Playwright before deploy |
| 19 | Using `goto()` inside load functions | Use `redirect()` from `@sveltejs/kit` — `goto` is client-only |
| 20 | Storing filterable/shareable state in component `$state` | Use URL search params for anything the user might bookmark or share |
| 21 | Creating stores for data that comes from load functions | Use the `data` prop from load functions. Stores are for client-only global state |
| 22 | Using `fetch` without SvelteKit's `fetch` in load functions | Use the `fetch` from the load event — it handles cookies, relative URLs, and SSR correctly |
| 23 | Missing error boundaries (`+error.svelte`) | Every route group should have an `+error.svelte` page for graceful error handling |
| 24 | Ignoring TypeScript strict mode warnings | Fix all `svelte-check` errors — they catch real bugs (type mismatches, missing props) |
| 25 | Using `any` type to silence TypeScript | Define proper types. Use `unknown` if the type is truly unknown, then narrow it |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a SvelteKit patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:sveltekit&title=[SvelteKit]%20)**

Use the `patterns:sveltekit` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
