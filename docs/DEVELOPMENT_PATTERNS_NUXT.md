# Development Patterns — Nuxt Stack

Vue 3 / Nuxt 3 / Vuetify / PrimeVue / Naive UI / Pinia / Nitro / TypeScript

This document captures stack-specific patterns, conventions, and decisions for Nuxt 3 stack projects (Vue 3/Nuxt 3/Vuetify or PrimeVue/Pinia/Nitro/TypeScript). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Nuxt 3 pages, use Composition API, test with Vitest + Playwright, choose component libraries (Vuetify vs PrimeVue vs Naive UI), manage state with Pinia, handle data fetching with useFetch/useAsyncData, deploy with Nitro, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Node.js | 22+ LTS | Minimum 22 for native fetch, Web Crypto |
| Nuxt | 3.16+ | Nitro server engine, auto-imports, file-based routing |
| Vue | 3.5+ | Composition API, `<script setup>`, Teleport, Suspense |
| TypeScript | 5.6+ | Strict mode enabled, `satisfies`, `NoInfer` |
| Nitro | 2.10+ | Universal server engine (Nuxt's backend) |
| Pinia | 2.3+ | Official Vue state management |
| VueUse | 12+ | Composition API utility collection |
| Zod | 3.24+ | Runtime schema validation for forms, API routes, server utils |
| Vitest | 3.x | Unit and integration test runner |
| @vue/test-utils | 2.4+ | Component mounting and interaction |
| @nuxt/test-utils | 3.17+ | Nuxt-specific testing (renderSuspended, mockNuxtImport) |
| Playwright | 1.50+ | End-to-end browser testing |
| MSW | 2.7+ | API mocking for tests and development |
| pnpm | 9+ | Package manager — strict, fast, disk-efficient |
| ESLint | 9+ | Flat config, `@nuxt/eslint` preset |
| Tailwind CSS | 4.x | Optional — used when not using a component library's style system |

### Component Library Selection

Choose one per project. Do not mix multiple component libraries — their CSS strategies conflict and maintenance burden multiplies.

| Library | Best For | Strengths | Weaknesses |
|---|---|---|---|
| **Vuetify 3** | Enterprise/admin apps, Material Design | Complete component set (80+), built-in a11y, SASS theming, grid system, form validation, icon system | Large bundle (~300KB gzip), opinionated Material Design, slower HMR |
| **PrimeVue 4** | Data-heavy apps, maximum flexibility | 100+ components, unstyled mode, Tailwind passthrough, DataTable/TreeTable, chart integration | Docs fragmented across styled/unstyled modes, some components have edge-case bugs |
| **Naive UI** | Developer-focused tools, TypeScript-first | Full TypeScript, tree-shakeable, built-in dark mode, clean API, CSR focus | Weak SSR support (hydration mismatches), smaller ecosystem, CJK-centric docs |
| **Radix Vue + shadcn-vue** | Custom design systems, maximum control | Headless/unstyled primitives, Tailwind styling, copy-paste ownership | Fewer pre-built components, more assembly required |
| **Element Plus** | Chinese enterprise ecosystems, rapid admin | Mature, complete component set, form validation, i18n | Heavy, tightly coupled styles, Material/Ant alternative aesthetic |

**Decision matrix:**

```
Need Material Design?                    → Vuetify 3
Need DataTable + unstyled/Tailwind?      → PrimeVue 4 (unstyled mode)
Need TypeScript-first + tree-shaking?    → Naive UI (CSR only)
Need headless primitives + full control? → Radix Vue / shadcn-vue
Need rapid Chinese enterprise admin?     → Element Plus
```

### Version Constraint Policy

Use exact versions in `package.json` for production dependencies, caret for dev:

```jsonc
{
  "dependencies": {
    // Good — exact for production stability
    "nuxt": "3.16.1",
    "vue": "3.5.13",
    "pinia": "2.3.1",
    "vuetify": "3.7.9"
  },
  "devDependencies": {
    // Good — caret for dev tools (allows minor updates)
    "vitest": "^3.0.0",
    "typescript": "^5.6.0",
    "@playwright/test": "^1.50.0"
  }
}
```

Exception: for packages that follow strict semver (VueUse, Zod), caret ranges are acceptable in production.

### Nuxt 3 Features to Use

| Feature | Use For |
|---|---|
| Auto-imports | Components, composables, utils — zero manual imports |
| File-based routing | `pages/` directory defines routes automatically |
| Nitro server engine | API routes, server middleware, server utils |
| `useFetch` / `useAsyncData` | SSR-safe data fetching with caching and deduplication |
| `useState` | SSR-safe shared state (cross-component, cross-request safe) |
| Hybrid rendering | Per-route SSR, SSG, SPA, ISR via `routeRules` |
| Nuxt Layers | Shared code, themes, config across multiple Nuxt apps |
| Nuxt DevTools | Component inspector, route visualization, Pinia state |
| `definePageMeta` | Per-page layout, middleware, transitions |
| `useRuntimeConfig` | Environment-aware configuration (public + private) |

### Vue 3 Features to Use

| Feature | Use For |
|---|---|
| `<script setup>` | All components — less boilerplate, better TypeScript inference |
| `defineProps` / `defineEmits` | Type-safe component interfaces |
| `defineModel` | Two-way binding with `v-model` (Vue 3.4+) |
| `computed` / `watch` / `watchEffect` | Reactive derivations and side effects |
| `provide` / `inject` | Dependency injection for deep component trees |
| Teleport | Modals, tooltips, popovers rendered at document root |
| Suspense | Async component boundaries with fallback content |
| `useTemplateRef` | Type-safe template refs (Vue 3.5+) |

---

## 2. Project Structure

### Nuxt 3 Directory Convention

```
project-root/
├── app/                        # Application source (Nuxt 3.14+ default srcDir)
│   ├── assets/                 # Uncompiled assets (SCSS, images, fonts)
│   │   ├── styles/
│   │   │   ├── main.scss       # Global styles, Vuetify overrides
│   │   │   └── variables.scss  # SASS variables for Vuetify theme
│   │   └── images/
│   ├── components/             # Auto-imported Vue components
│   │   ├── base/               # Reusable primitives (BaseCard, BaseModal)
│   │   ├── domain/             # Domain-specific (VisitorForm, SeatGrid)
│   │   ├── layout/             # Layout parts (AppHeader, AppFooter, AppSidebar)
│   │   └── ui/                 # Generic UI (DataTable wrapper, ChartWidget)
│   ├── composables/            # Auto-imported composables (useAuth, useApi)
│   │   ├── useAuth.ts
│   │   ├── useNotification.ts
│   │   └── usePagination.ts
│   ├── layouts/                # Nuxt layouts
│   │   ├── default.vue         # Main app layout
│   │   ├── auth.vue            # Login/register (minimal chrome)
│   │   └── embed.vue           # Iframe embed (no nav/footer)
│   ├── middleware/             # Route middleware (auth, role guards)
│   │   ├── auth.ts
│   │   └── admin.ts
│   ├── pages/                  # File-based routes
│   │   ├── index.vue           # /
│   │   ├── login.vue           # /login
│   │   ├── dashboard/
│   │   │   └── index.vue       # /dashboard
│   │   ├── chapters/
│   │   │   ├── index.vue       # /chapters
│   │   │   └── [slug].vue      # /chapters/:slug
│   │   └── admin/
│   │       ├── index.vue       # /admin
│   │       └── users.vue       # /admin/users
│   ├── plugins/                # Nuxt plugins (vuetify, pinia persist, etc.)
│   │   ├── vuetify.ts
│   │   └── api.ts
│   ├── stores/                 # Pinia stores
│   │   ├── auth.ts
│   │   ├── chapters.ts
│   │   └── notifications.ts
│   └── utils/                  # Auto-imported utility functions
│       ├── formatters.ts
│       ├── validators.ts
│       └── constants.ts
├── server/                     # Nitro server (API routes, middleware, utils)
│   ├── api/                    # API routes (/api/*)
│   │   ├── auth/
│   │   │   ├── login.post.ts
│   │   │   ├── register.post.ts
│   │   │   └── me.get.ts
│   │   ├── chapters/
│   │   │   ├── index.get.ts
│   │   │   └── [slug].get.ts
│   │   └── webhooks/
│   │       └── stripe.post.ts
│   ├── middleware/             # Server middleware (CORS, logging)
│   │   └── log.ts
│   ├── utils/                  # Server utilities (DB client, auth helpers)
│   │   ├── db.ts              # Database client (Drizzle/Prisma)
│   │   ├── auth.ts            # Token verification
│   │   └── validators.ts      # Zod schemas for API validation
│   └── plugins/               # Nitro plugins (startup hooks)
│       └── migrations.ts
├── public/                     # Static assets (served as-is)
│   ├── favicon.ico
│   └── robots.txt
├── tests/                      # Test files
│   ├── unit/                   # Vitest unit tests
│   │   ├── components/
│   │   ├── composables/
│   │   ├── stores/
│   │   └── utils/
│   ├── integration/            # API + server integration tests
│   │   └── api/
│   └── e2e/                    # Playwright E2E tests
│       ├── auth.spec.ts
│       └── dashboard.spec.ts
├── nuxt.config.ts              # Nuxt configuration
├── app.config.ts               # Runtime app config (client-safe)
├── package.json
├── tsconfig.json
├── vitest.config.ts            # Vitest configuration
└── playwright.config.ts        # Playwright configuration
```

**Conventions:**
- `app/` is the source directory (Nuxt 3.14+ convention). All client code lives here.
- `server/` is the Nitro server directory. All server-side code lives here.
- Components are organized by role: `base/` (reusable), `domain/` (feature-specific), `layout/` (app shell), `ui/` (generic wrappers).
- API route files are named with HTTP method suffix: `login.post.ts`, `chapters.get.ts`, `[id].delete.ts`.
- Pinia stores live in `app/stores/` — one file per store, named after the domain concept.
- Composables in `app/composables/` are auto-imported — no manual import statements needed.

### Auto-Import Rules

Nuxt 3 auto-imports from these directories:

| Directory | What's Auto-Imported | Import Style |
|---|---|---|
| `app/composables/` | All exported functions | `useAuth()` — no import needed |
| `app/utils/` | All exported functions | `formatDate()` — no import needed |
| `app/components/` | All `.vue` files | `<BaseCard />` — no import needed |
| `server/utils/` | All exported functions (server only) | `getDb()` — no import in API routes |

**Convention:** Only put genuinely reusable composables in `composables/`. Page-specific logic stays in the page component's `<script setup>`. If a composable is only used by one component, it belongs in that component.

### Component Naming Convention

```
components/
├── base/
│   ├── BaseButton.vue        # <BaseButton />
│   ├── BaseCard.vue          # <BaseCard />
│   └── BaseModal.vue         # <BaseModal />
├── domain/
│   ├── ChapterSeatGrid.vue   # <ChapterSeatGrid />
│   └── VisitorPipeline.vue   # <VisitorPipeline />
└── layout/
    ├── LayoutHeader.vue       # <LayoutHeader />
    └── LayoutFooter.vue       # <LayoutFooter />
```

Nuxt auto-imports use the directory path as prefix. A component at `components/base/Button.vue` becomes `<BaseButton />`. This is preferred over flat naming because it avoids collisions and communicates component role.

---

## 3. Composition API Patterns

### `<script setup>` as Default

Every component uses `<script setup lang="ts">`. Never use the Options API in Nuxt 3 projects.

```vue
<script setup lang="ts">
// Props with TypeScript types
const props = defineProps<{
  title: string
  count?: number
  items: ReadonlyArray<Item>
}>()

// Props with defaults
const props = withDefaults(defineProps<{
  variant?: 'primary' | 'secondary'
  size?: 'sm' | 'md' | 'lg'
}>(), {
  variant: 'primary',
  size: 'md',
})

// Emits with typed payloads
const emit = defineEmits<{
  (e: 'update', id: string): void
  (e: 'delete', id: string, confirm: boolean): void
}>()

// Two-way binding (Vue 3.4+)
const model = defineModel<string>({ required: true })

// Expose for parent ref access (rare — avoid when possible)
defineExpose({ validate })
</script>
```

### Composable Structure

Every composable follows this pattern:

```typescript
// composables/useCounter.ts
export function useCounter(initial: number = 0) {
  const count = ref(initial)
  const doubled = computed(() => count.value * 2)

  function increment() {
    count.value++
  }

  function decrement() {
    count.value--
  }

  function reset() {
    count.value = initial
  }

  return {
    count: readonly(count),   // Expose as readonly when callers should not mutate
    doubled,
    increment,
    decrement,
    reset,
  }
}
```

**Conventions:**
- Name composables with `use` prefix: `useAuth`, `useApi`, `usePagination`.
- Return an object with named properties — never return a tuple/array (unlike React hooks, Vue composables benefit from named destructuring).
- Expose refs as `readonly()` when the composable owns the mutation logic.
- Accept configuration via a single options object for composables with more than 2 parameters.
- Keep composables stateless (new state per call) unless explicitly designed as singletons.

### Reactive State Patterns

```typescript
// ref — for primitive values and replaceable objects
const count = ref(0)
const user = ref<User | null>(null)

// reactive — for objects where you mutate properties (NOT replace the whole object)
const form = reactive({
  name: '',
  email: '',
  phone: '',
})

// computed — for derived values (cached, dependency-tracked)
const fullName = computed(() => `${form.firstName} ${form.lastName}`)

// watch — for side effects when specific values change
watch(
  () => route.params.slug,
  async (newSlug) => {
    await fetchChapter(newSlug as string)
  },
)

// watchEffect — for side effects that auto-track dependencies
watchEffect(() => {
  console.log(`Chapter: ${chapter.value?.name}`)
})
```

**When to use `ref` vs `reactive`:**
- `ref` for primitives, nullable values, values you replace entirely.
- `reactive` for form objects, configuration objects where you mutate individual properties.
- Never use `reactive` for a value you need to reassign entirely — reactivity is lost on reassignment.

### Provide/Inject for Deep Trees

```typescript
// Parent component — provide
import type { InjectionKey } from 'vue'

export const ThemeKey: InjectionKey<Ref<'light' | 'dark'>> = Symbol('theme')

const theme = ref<'light' | 'dark'>('light')
provide(ThemeKey, theme)

// Deep child component — inject
const theme = inject(ThemeKey)
if (!theme) throw new Error('ThemeKey not provided')
```

**Convention:** Always use typed `InjectionKey` symbols. Never use string keys — they are not type-safe and collide easily.

---

## 4. Data Fetching

### `useFetch` vs `useAsyncData`

Both are SSR-safe. Choose based on the data source:

| Composable | Use When | Key Difference |
|---|---|---|
| `useFetch` | Calling Nuxt API routes or external APIs | Wraps `$fetch` — handles URL, method, body automatically |
| `useAsyncData` | Custom async logic (DB query, complex transforms) | Takes any async function — you control the fetching |
| `$fetch` | Server-side only (API routes, server utils) | Raw fetch — NOT SSR-safe in components |

### `useFetch` Patterns

```vue
<script setup lang="ts">
// Basic fetch with type inference from API route
const { data: chapters, status, error, refresh } = await useFetch('/api/chapters')

// With query parameters (reactive)
const page = ref(1)
const search = ref('')
const { data: results } = await useFetch('/api/chapters', {
  query: { page, search, limit: 20 },  // Reactive — refetches on change
})

// POST with body
const { data: session } = await useFetch('/api/auth/login', {
  method: 'POST',
  body: { email: 'user@example.com', password: 'secret' },
})

// With transform (shape data before storing)
const { data: names } = await useFetch('/api/users', {
  transform: (users) => users.map(u => u.name),
})

// With pick (reduce payload — only transfer selected fields)
const { data: chapter } = await useFetch(`/api/chapters/${slug}`, {
  pick: ['id', 'name', 'slug', 'meetingDay'],
})

// Lazy fetch (does not block SSR navigation — fetches client-side)
const { data, status } = useLazyFetch('/api/analytics/dashboard')

// Watch for reactive key changes
const slug = computed(() => route.params.slug as string)
const { data: chapter } = await useFetch(() => `/api/chapters/${slug.value}`)
```

### `useAsyncData` Patterns

```vue
<script setup lang="ts">
// Custom async logic
const { data: stats } = await useAsyncData('dashboard-stats', () => {
  return $fetch('/api/stats').then(raw => ({
    ...raw,
    computed: raw.visitors * raw.conversionRate,
  }))
})

// Parallel fetches with useAsyncData
const [{ data: chapters }, { data: stats }] = await Promise.all([
  useAsyncData('chapters', () => $fetch('/api/chapters')),
  useAsyncData('stats', () => $fetch('/api/stats')),
])

// Manual refresh
const { data, refresh } = await useAsyncData('key', fetchFn)
// Later: await refresh()

// Server-only fetch (never re-runs on client navigation)
const { data } = await useAsyncData('config', () => $fetch('/api/config'), {
  server: true,
  lazy: false,
})
```

### Caching and Deduplication

Nuxt deduplicates identical `useFetch`/`useAsyncData` calls during SSR. The `key` parameter controls caching:

```typescript
// These share a cache entry (same auto-generated key)
const { data } = await useFetch('/api/chapters')
const { data } = await useFetch('/api/chapters')  // Deduplicated — one request

// Force unique keys when same URL but different contexts
const { data: active } = await useFetch('/api/chapters', {
  key: 'active-chapters',
  query: { status: 'active' },
})
const { data: archived } = await useFetch('/api/chapters', {
  key: 'archived-chapters',
  query: { status: 'archived' },
})
```

### `getCachedData` for Instant Navigation

```typescript
const { data } = await useFetch(`/api/chapters/${slug}`, {
  key: `chapter-${slug}`,
  getCachedData(key, nuxtApp) {
    return nuxtApp.payload.data[key] || nuxtApp.static.data[key]
  },
})
```

This pattern enables instant page transitions by returning cached data immediately while revalidating in the background.

### Error Handling

```vue
<script setup lang="ts">
const { data, error, status } = await useFetch('/api/chapters')

// Handle errors in template
</script>

<template>
  <div v-if="status === 'pending'">
    <LoadingSpinner />
  </div>

  <div v-else-if="error">
    <ErrorMessage :error="error" />
  </div>

  <div v-else-if="data">
    <ChapterList :chapters="data" />
  </div>
</template>
```

**Convention:** Always handle all three states (pending, error, success) in the template. Never assume data is always available.

---

## 5. Authentication

### Server-Side Authentication with Nitro

Authentication is handled server-side via Nitro. The client never touches tokens directly.

```typescript
// server/utils/auth.ts
import { H3Event } from 'h3'
import jwt from 'jsonwebtoken'

const config = useRuntimeConfig()

export interface AuthUser {
  id: string
  email: string
  role: 'member' | 'admin' | 'platform_admin'
}

export function verifyAuth(event: H3Event): AuthUser {
  const token = getCookie(event, 'auth_token')
  if (!token) {
    throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })
  }

  try {
    return jwt.verify(token, config.jwtSecret) as AuthUser
  } catch {
    throw createError({ statusCode: 401, statusMessage: 'Invalid token' })
  }
}

export function requireRole(event: H3Event, role: string): AuthUser {
  const user = verifyAuth(event)
  if (user.role !== role && user.role !== 'platform_admin') {
    throw createError({ statusCode: 403, statusMessage: 'Forbidden' })
  }
  return user
}
```

### Login API Route

```typescript
// server/api/auth/login.post.ts
import { z } from 'zod'

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
})

export default defineEventHandler(async (event) => {
  const body = await readValidatedBody(event, loginSchema.parse)
  const user = await verifyPassword(body.email, body.password)

  if (!user) {
    throw createError({ statusCode: 401, statusMessage: 'Invalid credentials' })
  }

  const token = generateToken(user)

  setCookie(event, 'auth_token', token, {
    httpOnly: true,
    secure: true,
    sameSite: 'lax',
    maxAge: 60 * 60 * 24 * 7,  // 7 days
    path: '/',
  })

  return { user: { id: user.id, email: user.email, role: user.role } }
})
```

### Auth Middleware (Client-Side Route Guard)

```typescript
// middleware/auth.ts
export default defineNuxtRouteMiddleware((to) => {
  const { loggedIn } = useAuth()

  if (!loggedIn.value) {
    return navigateTo('/login', { replace: true })
  }
})
```

```vue
<!-- pages/dashboard/index.vue -->
<script setup lang="ts">
definePageMeta({
  middleware: 'auth',
  layout: 'default',
})
</script>
```

### Auth Composable

```typescript
// composables/useAuth.ts
export function useAuth() {
  const user = useState<AuthUser | null>('auth-user', () => null)
  const loggedIn = computed(() => !!user.value)

  async function login(email: string, password: string) {
    const { data, error } = await useFetch('/api/auth/login', {
      method: 'POST',
      body: { email, password },
    })

    if (error.value) throw error.value
    user.value = data.value?.user ?? null
  }

  async function logout() {
    await $fetch('/api/auth/logout', { method: 'POST' })
    user.value = null
    await navigateTo('/login')
  }

  async function fetchUser() {
    const { data } = await useFetch('/api/auth/me')
    user.value = data.value?.user ?? null
  }

  return {
    user: readonly(user),
    loggedIn,
    login,
    logout,
    fetchUser,
  }
}
```

### Role-Based Access

```typescript
// middleware/admin.ts
export default defineNuxtRouteMiddleware(() => {
  const { user } = useAuth()

  if (!user.value || !['admin', 'platform_admin'].includes(user.value.role)) {
    throw createError({ statusCode: 403, statusMessage: 'Forbidden' })
  }
})
```

```vue
<!-- Conditional UI based on role -->
<template>
  <div>
    <AdminPanel v-if="user?.role === 'platform_admin'" />
    <MemberDashboard v-else />
  </div>
</template>
```

### nuxt-auth-utils Module

For projects that want a batteries-included auth solution, use `nuxt-auth-utils`:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['nuxt-auth-utils'],

  runtimeConfig: {
    session: {
      maxAge: 60 * 60 * 24 * 7,  // 7 days
    },
    oauth: {
      github: {
        clientId: '',
        clientSecret: '',
      },
    },
  },
})
```

This provides `useUserSession()`, `requireUserSession()`, OAuth handlers, and sealed cookie sessions out of the box.

---

## 6. Component Library Patterns

### Vuetify 3 Setup

```typescript
// plugins/vuetify.ts
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

export default defineNuxtPlugin((nuxtApp) => {
  const vuetify = createVuetify({
    ssr: true,
    components,
    directives,
    icons: {
      defaultSet: 'mdi',
      aliases,
      sets: { mdi },
    },
    theme: {
      defaultTheme: 'light',
      themes: {
        light: {
          colors: {
            primary: '#1867C0',
            secondary: '#5CBBF6',
            error: '#B00020',
          },
        },
        dark: {
          colors: {
            primary: '#2196F3',
            secondary: '#90CAF9',
          },
        },
      },
    },
    defaults: {
      VBtn: { variant: 'flat', rounded: 'lg' },
      VTextField: { variant: 'outlined', density: 'comfortable' },
      VCard: { rounded: 'lg', elevation: 2 },
    },
  })

  nuxtApp.vueApp.use(vuetify)
})
```

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  css: ['vuetify/styles'],
  build: {
    transpile: ['vuetify'],
  },
})
```

### Vuetify Tree-Shaking (Production Optimization)

For production, import only used components to reduce bundle size:

```typescript
// plugins/vuetify.ts (production-optimized)
import { createVuetify } from 'vuetify'
import { VBtn, VCard, VTextField, VDataTable } from 'vuetify/components'
import { Ripple } from 'vuetify/directives'

export default defineNuxtPlugin((nuxtApp) => {
  const vuetify = createVuetify({
    ssr: true,
    components: { VBtn, VCard, VTextField, VDataTable },
    directives: { Ripple },
    // ... theme config
  })
  nuxtApp.vueApp.use(vuetify)
})
```

Alternatively, use the `vuetify-nuxt-module` for automatic tree-shaking:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['vuetify-nuxt-module'],
  vuetify: {
    moduleOptions: {
      treeshaking: true,
      useIconCDN: false,
    },
    vuetifyOptions: {
      // ... same config as above
    },
  },
})
```

### PrimeVue 4 Setup (Unstyled + Tailwind)

```typescript
// plugins/primevue.ts
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        darkModeSelector: '.dark',
        cssLayer: {
          name: 'primevue',
          order: 'tailwind-base, primevue, tailwind-utilities',
        },
      },
    },
  })
})
```

Or use the official Nuxt module:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['@primevue/nuxt-module'],
  primevue: {
    autoImport: true,
    options: {
      theme: {
        preset: Aura,
      },
    },
  },
})
```

### PrimeVue Passthrough (Tailwind Customization)

PrimeVue 4's passthrough API lets you apply Tailwind classes to every internal element:

```vue
<template>
  <DataTable
    :value="chapters"
    :pt="{
      root: { class: 'rounded-lg border border-gray-200' },
      header: { class: 'bg-gray-50 px-4 py-3 font-semibold' },
      bodyRow: { class: 'hover:bg-gray-50 transition-colors' },
      column: {
        headerCell: { class: 'px-4 py-3 text-left text-sm font-medium text-gray-500' },
        bodyCell: { class: 'px-4 py-3 text-sm text-gray-900' },
      },
    }"
  >
    <Column field="name" header="Chapter Name" />
    <Column field="meetingDay" header="Meeting Day" />
  </DataTable>
</template>
```

### Naive UI Setup (CSR Projects Only)

```typescript
// plugins/naive-ui.client.ts (client-only — Naive UI has SSR limitations)
import { setup } from '@css-render/vue3-ssr'

export default defineNuxtPlugin((nuxtApp) => {
  if (import.meta.server) {
    const { collect } = setup(nuxtApp.vueApp)
    nuxtApp.ssrContext?.head.push({
      style: () => collect()
        .split('</style>')
        .map(block => {
          const id = block.match(/cssr-id="([^"]+)"/)?.[1]
          const style = (block.match(/>([\s\S]+)/)?.[1] ?? '').trim()
          return { 'cssr-id': id, children: style }
        })
    })
  }
})
```

**Warning:** Naive UI is designed for CSR-first applications. SSR support requires manual CSS collection and can produce hydration mismatches. For SSR-heavy applications, prefer Vuetify or PrimeVue.

---

## 7. Testing Patterns

### Test Pyramid (Nuxt-specific)

```
        /\
       /  \          E2E (Playwright)
      /    \         Full browser flows, critical paths
     /------\
    /        \        Component Tests (@nuxt/test-utils)
   /          \       Mounted components with Nuxt context
  /------------\
 /              \      Composable Tests (Vitest)
/                \     useAuth, usePagination, custom composables
/------------------\
/                    \   Unit Tests (Vitest)
/                      \  Pure functions, utils, validators, Pinia stores
/------------------------\
```

### Vitest Configuration

```typescript
// vitest.config.ts
import { defineVitestConfig } from '@nuxt/test-utils/config'

export default defineVitestConfig({
  test: {
    environment: 'nuxt',
    environmentOptions: {
      nuxt: {
        domEnvironment: 'happy-dom',  // Faster than jsdom
      },
    },
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      include: ['app/**/*.{ts,vue}', 'server/**/*.ts'],
      exclude: [
        'app/plugins/**',
        '**/*.d.ts',
        '**/*.config.*',
        '**/types/**',
      ],
      thresholds: {
        statements: 80,
        branches: 80,
        functions: 80,
        lines: 80,
      },
    },
    globals: true,
    setupFiles: ['./tests/setup.ts'],
  },
})
```

### Unit Tests — Utils and Pure Functions

```typescript
// tests/unit/utils/formatters.test.ts
import { describe, it, expect } from 'vitest'
import { formatCurrency, formatDate, slugify } from '~/utils/formatters'

describe('formatCurrency', () => {
  it('formats USD with two decimal places', () => {
    expect(formatCurrency(1234.5)).toBe('$1,234.50')
  })

  it('handles zero', () => {
    expect(formatCurrency(0)).toBe('$0.00')
  })
})

describe('slugify', () => {
  it('converts to lowercase kebab-case', () => {
    expect(slugify('Westlake Select Chapter')).toBe('westlake-select-chapter')
  })

  it('removes special characters', () => {
    expect(slugify("O'Brien & Associates")).toBe('obrien-associates')
  })
})
```

### Pinia Store Tests

```typescript
// tests/unit/stores/auth.test.ts
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAuthStore } from '~/stores/auth'

describe('useAuthStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('starts with no user', () => {
    const store = useAuthStore()
    expect(store.user).toBeNull()
    expect(store.isLoggedIn).toBe(false)
  })

  it('sets user on login', async () => {
    const store = useAuthStore()
    // Mock the API call
    vi.stubGlobal('$fetch', vi.fn().mockResolvedValue({
      user: { id: '1', email: 'test@example.com', role: 'member' },
    }))

    await store.login('test@example.com', 'password')
    expect(store.user?.email).toBe('test@example.com')
    expect(store.isLoggedIn).toBe(true)
  })

  it('clears user on logout', async () => {
    const store = useAuthStore()
    store.user = { id: '1', email: 'test@example.com', role: 'member' }

    vi.stubGlobal('$fetch', vi.fn().mockResolvedValue(undefined))
    await store.logout()

    expect(store.user).toBeNull()
    expect(store.isLoggedIn).toBe(false)
  })
})
```

### Composable Tests

```typescript
// tests/unit/composables/usePagination.test.ts
import { describe, it, expect } from 'vitest'
import { usePagination } from '~/composables/usePagination'

describe('usePagination', () => {
  it('calculates total pages', () => {
    const { totalPages } = usePagination({ totalItems: 100, perPage: 10 })
    expect(totalPages.value).toBe(10)
  })

  it('clamps current page to valid range', () => {
    const { currentPage, goToPage, totalPages } = usePagination({
      totalItems: 50,
      perPage: 10,
    })

    goToPage(999)
    expect(currentPage.value).toBe(totalPages.value)

    goToPage(-5)
    expect(currentPage.value).toBe(1)
  })

  it('computes offset correctly', () => {
    const { offset, goToPage } = usePagination({ totalItems: 100, perPage: 10 })

    goToPage(3)
    expect(offset.value).toBe(20)
  })
})
```

### Component Tests with @nuxt/test-utils

```typescript
// tests/unit/components/ChapterCard.test.ts
import { describe, it, expect } from 'vitest'
import { mountSuspended } from '@nuxt/test-utils/runtime'
import ChapterCard from '~/components/domain/ChapterCard.vue'

describe('ChapterCard', () => {
  it('renders chapter name and meeting day', async () => {
    const wrapper = await mountSuspended(ChapterCard, {
      props: {
        chapter: {
          id: '1',
          name: 'Westlake Select',
          slug: 'westlake-select',
          meetingDay: 'Wednesday',
          meetingTime: '7:00 AM',
        },
      },
    })

    expect(wrapper.text()).toContain('Westlake Select')
    expect(wrapper.text()).toContain('Wednesday')
  })

  it('emits select event on click', async () => {
    const wrapper = await mountSuspended(ChapterCard, {
      props: { chapter: mockChapter },
    })

    await wrapper.find('[data-testid="chapter-card"]').trigger('click')
    expect(wrapper.emitted('select')).toHaveLength(1)
    expect(wrapper.emitted('select')![0]).toEqual([mockChapter.id])
  })
})
```

### Mocking Nuxt Auto-Imports in Tests

```typescript
// tests/unit/components/Dashboard.test.ts
import { describe, it, expect, vi } from 'vitest'
import { mockNuxtImport } from '@nuxt/test-utils/runtime'

// Mock auto-imported composables
const { useAuthMock } = vi.hoisted(() => ({
  useAuthMock: vi.fn(() => ({
    user: ref({ id: '1', email: 'test@example.com', role: 'admin' }),
    loggedIn: ref(true),
  })),
}))

mockNuxtImport('useAuth', () => useAuthMock)

describe('Dashboard', () => {
  it('shows admin panel for admin users', async () => {
    const wrapper = await mountSuspended(Dashboard)
    expect(wrapper.find('[data-testid="admin-panel"]').exists()).toBe(true)
  })
})
```

### API Route Tests

```typescript
// tests/integration/api/chapters.test.ts
import { describe, it, expect, beforeAll } from 'vitest'
import { setup, $fetch } from '@nuxt/test-utils/e2e'

describe('/api/chapters', async () => {
  await setup({ host: 'http://localhost:3000' })

  it('returns list of chapters', async () => {
    const chapters = await $fetch('/api/chapters')
    expect(chapters).toBeInstanceOf(Array)
    expect(chapters[0]).toHaveProperty('name')
    expect(chapters[0]).toHaveProperty('slug')
  })

  it('returns 404 for nonexistent slug', async () => {
    try {
      await $fetch('/api/chapters/nonexistent')
    } catch (error: any) {
      expect(error.statusCode).toBe(404)
    }
  })
})
```

### Playwright E2E Tests

```typescript
// tests/e2e/auth.spec.ts
import { test, expect } from '@playwright/test'

test.describe('Authentication', () => {
  test('user can log in and see dashboard', async ({ page }) => {
    await page.goto('/login')

    await page.fill('[data-testid="email-input"]', 'admin@example.com')
    await page.fill('[data-testid="password-input"]', 'password123')
    await page.click('[data-testid="login-button"]')

    await expect(page).toHaveURL('/dashboard')
    await expect(page.getByText('Welcome')).toBeVisible()
  })

  test('invalid credentials show error', async ({ page }) => {
    await page.goto('/login')

    await page.fill('[data-testid="email-input"]', 'bad@example.com')
    await page.fill('[data-testid="password-input"]', 'wrong')
    await page.click('[data-testid="login-button"]')

    await expect(page.getByText('Invalid credentials')).toBeVisible()
    await expect(page).toHaveURL('/login')
  })
})

// tests/e2e/chapters.spec.ts
test.describe('Chapter browsing', () => {
  test('user can navigate to chapter detail', async ({ page }) => {
    await page.goto('/chapters')
    await page.click('text=Westlake Select')
    await expect(page).toHaveURL(/\/chapters\/westlake-select/)
    await expect(page.getByRole('heading', { name: 'Westlake Select' })).toBeVisible()
  })
})
```

### Playwright Configuration

```typescript
// playwright.config.ts
import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: process.env.CI ? 'github' : 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'mobile', use: { ...devices['Pixel 5'] } },
  ],
  webServer: {
    command: 'pnpm dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
})
```

---

## 8. Nitro Server Engine

### API Route Patterns

API routes live in `server/api/`. The file name determines the path and HTTP method.

```
server/api/
├── auth/
│   ├── login.post.ts          → POST /api/auth/login
│   ├── register.post.ts       → POST /api/auth/register
│   ├── logout.post.ts         → POST /api/auth/logout
│   └── me.get.ts              → GET  /api/auth/me
├── chapters/
│   ├── index.get.ts           → GET  /api/chapters
│   ├── index.post.ts          → POST /api/chapters
│   ├── [slug].get.ts          → GET  /api/chapters/:slug
│   ├── [slug].put.ts          → PUT  /api/chapters/:slug
│   └── [slug].delete.ts       → DELETE /api/chapters/:slug
└── webhooks/
    └── stripe.post.ts         → POST /api/webhooks/stripe
```

### API Route Template

```typescript
// server/api/chapters/index.get.ts
export default defineEventHandler(async (event) => {
  // 1. Authentication (if needed)
  const user = verifyAuth(event)

  // 2. Validate input
  const query = getQuery(event)

  // 3. Business logic
  const chapters = await getDb()
    .select()
    .from(chaptersTable)
    .where(eq(chaptersTable.active, true))
    .limit(Number(query.limit) || 20)
    .offset(Number(query.offset) || 0)

  // 4. Return response
  return chapters
})
```

```typescript
// server/api/chapters/index.post.ts
import { z } from 'zod'

const createChapterSchema = z.object({
  name: z.string().min(2).max(100),
  slug: z.string().regex(/^[a-z0-9-]+$/),
  meetingDay: z.enum(['monday', 'tuesday', 'wednesday', 'thursday', 'friday']),
  meetingTime: z.string(),
  location: z.string().min(2),
})

export default defineEventHandler(async (event) => {
  const user = requireRole(event, 'admin')
  const body = await readValidatedBody(event, createChapterSchema.parse)

  const chapter = await getDb()
    .insert(chaptersTable)
    .values(body)
    .returning()

  setResponseStatus(event, 201)
  return chapter[0]
})
```

### Server Middleware

```typescript
// server/middleware/log.ts
export default defineEventHandler((event) => {
  const start = Date.now()

  event.node.res.on('finish', () => {
    const duration = Date.now() - start
    console.log(`${event.method} ${event.path} ${event.node.res.statusCode} ${duration}ms`)
  })
})
```

### Server Utils (Auto-Imported)

```typescript
// server/utils/db.ts
import { drizzle } from 'drizzle-orm/node-postgres'
import * as schema from '~/server/database/schema'

let _db: ReturnType<typeof drizzle> | null = null

export function getDb() {
  if (!_db) {
    const config = useRuntimeConfig()
    _db = drizzle(config.databaseUrl, { schema })
  }
  return _db
}
```

```typescript
// server/utils/validators.ts
import { z } from 'zod'

export const paginationSchema = z.object({
  page: z.coerce.number().int().positive().default(1),
  limit: z.coerce.number().int().min(1).max(100).default(20),
})

export const slugSchema = z.string().regex(/^[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/)
```

### Nitro Storage (KV)

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  nitro: {
    storage: {
      cache: { driver: 'redis', url: process.env.REDIS_URL },
      sessions: { driver: 'redis', url: process.env.REDIS_URL },
    },
  },
})

// server/api/config.get.ts
export default defineEventHandler(async () => {
  const cached = await useStorage('cache').getItem('app-config')
  if (cached) return cached

  const config = await fetchConfig()
  await useStorage('cache').setItem('app-config', config, { ttl: 3600 })
  return config
})
```

### Error Handling in API Routes

```typescript
// Structured errors — Nitro automatically serializes these to JSON
export default defineEventHandler(async (event) => {
  const slug = getRouterParam(event, 'slug')
  const chapter = await getDb()
    .select()
    .from(chaptersTable)
    .where(eq(chaptersTable.slug, slug))
    .limit(1)

  if (!chapter[0]) {
    throw createError({
      statusCode: 404,
      statusMessage: 'Chapter not found',
      data: { slug },
    })
  }

  return chapter[0]
})
```

---

## 9. State Management (Pinia)

### Store Design

One store per domain concept. Stores live in `app/stores/`.

```typescript
// stores/auth.ts
export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<AuthUser | null>(null)
  const token = ref<string | null>(null)

  // Getters (computed)
  const isLoggedIn = computed(() => !!user.value)
  const isAdmin = computed(() =>
    user.value?.role === 'admin' || user.value?.role === 'platform_admin'
  )
  const displayName = computed(() => user.value?.name ?? user.value?.email ?? 'Guest')

  // Actions
  async function login(email: string, password: string) {
    const response = await $fetch('/api/auth/login', {
      method: 'POST',
      body: { email, password },
    })
    user.value = response.user
  }

  async function logout() {
    await $fetch('/api/auth/logout', { method: 'POST' })
    user.value = null
    token.value = null
  }

  async function fetchUser() {
    try {
      const response = await $fetch('/api/auth/me')
      user.value = response.user
    } catch {
      user.value = null
    }
  }

  return { user, token, isLoggedIn, isAdmin, displayName, login, logout, fetchUser }
})
```

**Convention:** Always use the setup syntax (`defineStore('id', () => { ... })`) instead of the options syntax. It provides better TypeScript inference and is consistent with `<script setup>`.

### Store Composition

Stores can use other stores:

```typescript
// stores/chapters.ts
export const useChapterStore = defineStore('chapters', () => {
  const authStore = useAuthStore()

  const chapters = ref<Chapter[]>([])
  const loading = ref(false)

  const userChapters = computed(() => {
    if (authStore.isAdmin) return chapters.value
    return chapters.value.filter(c => c.memberIds.includes(authStore.user?.id ?? ''))
  })

  async function fetchChapters() {
    loading.value = true
    try {
      chapters.value = await $fetch('/api/chapters')
    } finally {
      loading.value = false
    }
  }

  return { chapters, loading, userChapters, fetchChapters }
})
```

### Pinia Persistence

For stores that should survive page refresh:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['@pinia/nuxt', 'pinia-plugin-persistedstate/nuxt'],
})
```

```typescript
// stores/preferences.ts
export const usePreferencesStore = defineStore('preferences', () => {
  const theme = ref<'light' | 'dark'>('light')
  const sidebarCollapsed = ref(false)
  const locale = ref('en')

  return { theme, sidebarCollapsed, locale }
}, {
  persist: {
    pick: ['theme', 'sidebarCollapsed', 'locale'],
  },
})
```

### `useState` vs Pinia

| Use Case | `useState` | Pinia |
|---|---|---|
| Simple SSR-safe shared state | Yes | Overkill |
| Complex business logic with actions | No | Yes |
| State that needs persistence | No | Yes (with plugin) |
| DevTools inspection | Limited | Full Pinia DevTools |
| Store composition (store uses store) | Manual | Built-in |

**Convention:** Use `useState` for simple cross-component reactivity (user, theme, locale). Use Pinia when you need actions, getters, persistence, or DevTools.

### Hydration Safety

Pinia stores are automatically SSR-safe when used through `@pinia/nuxt`. However, avoid initializing stores with browser-only APIs:

```typescript
// BAD — window is not available during SSR
const useUiStore = defineStore('ui', () => {
  const width = ref(window.innerWidth)  // Crashes on server
  return { width }
})

// GOOD — guard browser APIs
const useUiStore = defineStore('ui', () => {
  const width = ref(0)

  if (import.meta.client) {
    width.value = window.innerWidth
    window.addEventListener('resize', () => {
      width.value = window.innerWidth
    })
  }

  return { width }
})
```

---

## 10. Performance

### Route Rules (Hybrid Rendering)

Nuxt 3 supports per-route rendering strategies via `routeRules`:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  routeRules: {
    // Static generation (SSG) — built at deploy time
    '/': { prerender: true },
    '/about': { prerender: true },
    '/blog/**': { prerender: true },

    // Incremental Static Regeneration (ISR) — revalidate every 60 seconds
    '/chapters': { isr: 60 },
    '/chapters/**': { isr: 300 },

    // Server-side rendering (SSR) — fresh on every request
    '/dashboard/**': { ssr: true },
    '/admin/**': { ssr: true },

    // Client-side only (SPA) — no server rendering
    '/app/**': { ssr: false },

    // Cache headers
    '/api/chapters': { headers: { 'cache-control': 'public, max-age=60, s-maxage=600' } },

    // Redirect
    '/old-path': { redirect: '/new-path' },

    // CORS for API
    '/api/**': {
      cors: true,
      headers: { 'access-control-allow-methods': 'GET, POST, PUT, DELETE' },
    },
  },
})
```

### Component Lazy Loading

```vue
<template>
  <!-- Lazy-loaded components (loaded only when visible / needed) -->
  <LazyHeavyChart v-if="showChart" :data="chartData" />

  <!-- Nuxt auto-prefixes with "Lazy" for dynamic imports -->
  <LazyDomainVisitorPipeline v-if="pipeline.length" :stages="pipeline" />
</template>
```

Any component can be lazy-loaded by prefixing with `Lazy`. Nuxt generates a dynamic import automatically.

### Image Optimization

Use `@nuxt/image` for automatic optimization:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['@nuxt/image'],
  image: {
    quality: 80,
    formats: ['avif', 'webp'],
    screens: {
      xs: 320,
      sm: 640,
      md: 768,
      lg: 1024,
      xl: 1280,
    },
  },
})
```

```vue
<template>
  <!-- Optimized image with responsive sizes -->
  <NuxtImg
    src="/images/hero.jpg"
    alt="Hero image"
    width="1200"
    height="600"
    sizes="sm:100vw md:100vw lg:1200px"
    loading="lazy"
    format="webp"
  />

  <!-- Background-style image -->
  <NuxtPicture
    src="/images/banner.jpg"
    alt="Banner"
    sizes="sm:100vw lg:1400px"
  />
</template>
```

### Bundle Analysis

```bash
# Analyze client bundle
npx nuxi analyze

# Check build output size
npx nuxi build --analyze
```

### Key Performance Rules

1. **Lazy-load below-the-fold components** — prefix with `Lazy` to code-split automatically.
2. **Use `useLazyFetch` for non-critical data** — does not block SSR, loads client-side.
3. **Set `routeRules` for every route** — default to ISR for public pages, SSR for authenticated pages.
4. **Use `<NuxtImg>` instead of `<img>`** — automatic responsive sizing, format conversion, lazy loading.
5. **Tree-shake component libraries** — import only what you use from Vuetify/PrimeVue.
6. **Enable payload extraction** — `experimental: { payloadExtraction: true }` to deduplicate SSR payloads.
7. **Avoid large client-side stores** — fetch data server-side and pass via SSR. Pinia is for UI state, not server cache.
8. **Use `getCachedData`** — return stale data instantly while revalidating in background for smooth navigation.

### Lighthouse Targets

| Metric | Target | How |
|---|---|---|
| LCP | < 2.5s | Prerender above-the-fold, optimize images, font preloading |
| FID / INP | < 100ms | Minimize client-side JS, lazy-load heavy components |
| CLS | < 0.1 | Set explicit width/height on images, avoid layout shifts |
| TTI | < 3.5s | Code-split, tree-shake, use ISR for static content |
| Bundle size | < 200KB gzip (initial) | Analyze with `nuxi analyze`, lazy-load aggressively |

---

## 11. Deployment

### Nitro Presets

Nitro compiles the server for any deployment target:

| Target | Preset | Command |
|---|---|---|
| Node.js server | `node-server` | `NITRO_PRESET=node-server nuxt build` |
| Vercel | `vercel` | Auto-detected |
| Netlify | `netlify` | Auto-detected |
| Cloudflare Workers | `cloudflare` | `NITRO_PRESET=cloudflare nuxt build` |
| AWS Lambda | `aws-lambda` | `NITRO_PRESET=aws-lambda nuxt build` |
| Deno Deploy | `deno-deploy` | `NITRO_PRESET=deno-deploy nuxt build` |
| Docker / self-hosted | `node-server` | Dockerfile below |
| Static (SSG) | `static` | `nuxt generate` |

### Dockerfile (Node.js)

```dockerfile
# Stage 1: Build
FROM node:22-alpine AS builder
WORKDIR /app
RUN corepack enable && corepack prepare pnpm@latest --activate

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# Stage 2: Runtime
FROM node:22-alpine AS runtime
WORKDIR /app

COPY --from=builder /app/.output /app/.output

ENV NODE_ENV=production
ENV NITRO_PORT=3000
ENV NITRO_HOST=0.0.0.0

EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]
```

### Vercel Configuration

```json
// vercel.json (usually not needed — auto-detected)
{
  "buildCommand": "pnpm build",
  "outputDirectory": ".output",
  "framework": "nuxtjs"
}
```

```typescript
// nuxt.config.ts — Vercel-specific
export default defineNuxtConfig({
  nitro: {
    preset: 'vercel',
  },
})
```

### Environment Configuration

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  runtimeConfig: {
    // Server-only (never exposed to client)
    jwtSecret: process.env.JWT_SECRET,
    databaseUrl: process.env.DATABASE_URL,
    stripeSecretKey: process.env.STRIPE_SECRET_KEY,

    // Client-safe (exposed via useRuntimeConfig().public)
    public: {
      appUrl: process.env.NUXT_PUBLIC_APP_URL || 'http://localhost:3000',
      stripePublicKey: process.env.NUXT_PUBLIC_STRIPE_KEY,
    },
  },
})
```

**Convention:** Server-only secrets go in `runtimeConfig` (top level). Client-safe values go in `runtimeConfig.public`. Environment variables override at runtime with `NUXT_` prefix: `NUXT_JWT_SECRET`, `NUXT_PUBLIC_APP_URL`.

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
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'pnpm'
      - run: pnpm install --frozen-lockfile
      - run: pnpm lint                   # ESLint + Prettier
      - run: pnpm typecheck              # nuxi typecheck
      - run: pnpm test:unit              # Vitest
      - run: pnpm test:e2e               # Playwright
      - run: pnpm build                  # Verify build succeeds

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: amondnet/vercel-action@v25  # Or your deployment method
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          vercel-args: '--prod'
```

### Common Commands

```bash
# Development
pnpm dev                              # Start dev server (port 3000)
pnpm dev --host                       # Expose to network (mobile testing)
pnpm build                            # Production build
pnpm preview                          # Preview production build locally
pnpm generate                         # Static site generation (SSG)

# Quality
pnpm lint                             # ESLint (flat config)
pnpm lint --fix                       # Auto-fix lint issues
pnpm typecheck                        # nuxi typecheck (vue-tsc)
pnpm format                           # Prettier

# Testing
pnpm test:unit                        # Vitest unit tests
pnpm test:unit --coverage             # With coverage report
pnpm test:e2e                         # Playwright E2E tests
pnpm test:e2e --headed                # With visible browser
pnpm test                             # All tests

# Utilities
npx nuxi analyze                      # Bundle analysis
npx nuxi cleanup                      # Clear .nuxt, .output, node_modules/.cache
npx nuxi module add @nuxt/image       # Add a Nuxt module
npx nuxi upgrade                      # Upgrade Nuxt to latest
```

---

## 12. Security

### Runtime Config — Never Expose Secrets

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  runtimeConfig: {
    // PRIVATE — only available in server/ code via useRuntimeConfig()
    jwtSecret: '',
    databaseUrl: '',
    stripeSecretKey: '',
    emailApiKey: '',

    // PUBLIC — available everywhere, sent to client
    public: {
      appUrl: '',
      stripePublicKey: '',
    },
  },
})
```

**Rule:** Never put secrets in `runtimeConfig.public`. Never read `runtimeConfig` (non-public) in client components — it throws an error in production and leaks secrets in development.

### Input Validation (Zod on Server)

Every API route validates input with Zod:

```typescript
// server/api/chapters/index.post.ts
import { z } from 'zod'

const schema = z.object({
  name: z.string().min(2).max(100).trim(),
  slug: z.string().regex(/^[a-z0-9-]+$/).max(50),
  meetingDay: z.enum(['monday', 'tuesday', 'wednesday', 'thursday', 'friday']),
})

export default defineEventHandler(async (event) => {
  // readValidatedBody throws 422 with Zod errors if validation fails
  const body = await readValidatedBody(event, schema.parse)
  // body is fully typed and validated
})
```

```typescript
// Query parameter validation
import { z } from 'zod'

const querySchema = z.object({
  page: z.coerce.number().int().positive().default(1),
  search: z.string().optional(),
})

export default defineEventHandler(async (event) => {
  const query = await getValidatedQuery(event, querySchema.parse)
})
```

### CSRF Protection

Nuxt does not include CSRF protection by default. For cookie-based auth, add it:

```typescript
// server/middleware/csrf.ts
export default defineEventHandler((event) => {
  if (['POST', 'PUT', 'DELETE', 'PATCH'].includes(event.method)) {
    const origin = getHeader(event, 'origin')
    const host = getHeader(event, 'host')

    if (origin && !origin.includes(host!)) {
      throw createError({ statusCode: 403, statusMessage: 'CSRF check failed' })
    }
  }
})
```

For more robust CSRF protection, use the `nuxt-csurf` module:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['nuxt-csurf'],
  csurf: {
    https: true,
    methodsToProtect: ['POST', 'PUT', 'DELETE', 'PATCH'],
  },
})
```

### Security Headers

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  routeRules: {
    '/**': {
      headers: {
        'strict-transport-security': 'max-age=31536000; includeSubDomains; preload',
        'x-content-type-options': 'nosniff',
        'x-frame-options': 'SAMEORIGIN',
        'referrer-policy': 'strict-origin-when-cross-origin',
        'permissions-policy': 'camera=(), microphone=(), geolocation=()',
        'content-security-policy': [
          "default-src 'self'",
          "script-src 'self' 'unsafe-inline'",    // Required for Nuxt inline scripts
          "style-src 'self' 'unsafe-inline'",      // Required for Vuetify/PrimeVue
          "img-src 'self' data: https:",
          "font-src 'self'",
          "connect-src 'self' wss:",               // WebSocket for HMR in dev
          "frame-ancestors 'self'",
        ].join('; '),
      },
    },
  },
})
```

Or use the `nuxt-security` module for comprehensive defaults:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['nuxt-security'],
  security: {
    headers: {
      crossOriginEmbedderPolicy: 'unsafe-none',  // Required for external images
      contentSecurityPolicy: {
        'default-src': ["'self'"],
        'script-src': ["'self'", "'unsafe-inline'"],
        'style-src': ["'self'", "'unsafe-inline'"],
      },
    },
    rateLimiter: {
      tokensPerInterval: 100,
      interval: 60000,
    },
  },
})
```

### Rate Limiting

```typescript
// server/middleware/rate-limit.ts
const ipCounts = new Map<string, { count: number; resetAt: number }>()

export default defineEventHandler((event) => {
  if (!event.path.startsWith('/api/')) return

  const ip = getRequestIP(event) ?? 'unknown'
  const now = Date.now()
  const entry = ipCounts.get(ip)

  if (!entry || now > entry.resetAt) {
    ipCounts.set(ip, { count: 1, resetAt: now + 60_000 })
    return
  }

  entry.count++
  if (entry.count > 100) {
    throw createError({ statusCode: 429, statusMessage: 'Too many requests' })
  }
})
```

### Webhook Signature Verification

```typescript
// server/api/webhooks/stripe.post.ts
import Stripe from 'stripe'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const stripe = new Stripe(config.stripeSecretKey)

  const body = await readRawBody(event)
  const signature = getHeader(event, 'stripe-signature')

  if (!body || !signature) {
    throw createError({ statusCode: 400, statusMessage: 'Missing body or signature' })
  }

  let stripeEvent: Stripe.Event
  try {
    stripeEvent = stripe.webhooks.constructEvent(body, signature, config.stripeWebhookSecret)
  } catch {
    throw createError({ statusCode: 400, statusMessage: 'Invalid signature' })
  }

  // Process verified event
  switch (stripeEvent.type) {
    case 'checkout.session.completed':
      await handleCheckoutComplete(stripeEvent.data.object)
      break
  }

  return { received: true }
})
```

---

## 13. Coverage Enforcement

### Vitest Coverage Configuration

```typescript
// vitest.config.ts
import { defineVitestConfig } from '@nuxt/test-utils/config'

export default defineVitestConfig({
  test: {
    coverage: {
      provider: 'v8',
      reporter: ['text', 'text-summary', 'html', 'lcov'],
      reportsDirectory: './coverage',
      include: [
        'app/composables/**/*.ts',
        'app/stores/**/*.ts',
        'app/utils/**/*.ts',
        'app/components/**/*.vue',
        'server/**/*.ts',
      ],
      exclude: [
        '**/*.d.ts',
        '**/*.config.*',
        '**/types/**',
        'server/plugins/**',
        'app/plugins/**',
      ],
      thresholds: {
        statements: 80,
        branches: 80,
        functions: 80,
        lines: 80,
      },
    },
  },
})
```

### Package.json Scripts

```json
{
  "scripts": {
    "test:unit": "vitest run",
    "test:unit:watch": "vitest",
    "test:unit:coverage": "vitest run --coverage",
    "test:e2e": "playwright test",
    "test:e2e:headed": "playwright test --headed",
    "test": "vitest run && playwright test",
    "lint": "eslint .",
    "typecheck": "nuxi typecheck",
    "format": "prettier --write .",
    "quality": "pnpm lint && pnpm typecheck && pnpm test:unit:coverage",
    "ci": "pnpm quality && pnpm test:e2e && pnpm build"
  }
}
```

**Commands:**
```bash
pnpm test:unit:coverage               # Coverage report to terminal + HTML
open coverage/index.html               # Visual coverage report
```

Target is 100% (per CLAUDE.md core rules). The `thresholds` in vitest.config.ts is the hard gate — CI fails below this threshold. Start at 80%, ratchet up as coverage grows. Never decrease.

---

## 14. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with fieldsets |
| **labels** | Top-aligned, visible labels, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | Inline + error summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Reserve My Free Visit" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | Debounce validation, no unnecessary watchers |

### Form Pattern with Vuetify

```vue
<script setup lang="ts">
import { z } from 'zod'

const schema = z.object({
  name: z.string().min(2, 'Name is required'),
  email: z.string().email('Invalid email address'),
  phone: z.string().min(10, 'Phone number is required'),
})

type FormData = z.infer<typeof schema>

const form = reactive<FormData>({
  name: '',
  email: '',
  phone: '',
})

const errors = ref<Record<string, string>>({})
const submitting = ref(false)
const submitted = ref(false)

async function handleSubmit() {
  errors.value = {}

  const result = schema.safeParse(form)
  if (!result.success) {
    for (const issue of result.error.issues) {
      errors.value[issue.path[0] as string] = issue.message
    }
    return
  }

  submitting.value = true
  try {
    await $fetch('/api/visitors', {
      method: 'POST',
      body: result.data,
    })
    submitted.value = true
  } catch (err: any) {
    errors.value.form = err.data?.message || 'Something went wrong'
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <v-form @submit.prevent="handleSubmit" novalidate>
    <v-alert v-if="errors.form" type="error" class="mb-4" aria-live="assertive">
      {{ errors.form }}
    </v-alert>

    <fieldset>
      <legend class="text-subtitle-1 font-weight-bold mb-4">Your Information</legend>

      <v-text-field
        v-model="form.name"
        label="Full Name"
        required
        autocomplete="name"
        :error-messages="errors.name"
        class="mb-3"
      />

      <v-text-field
        v-model="form.email"
        label="Email Address"
        type="email"
        required
        autocomplete="email"
        :error-messages="errors.email"
        class="mb-3"
      />

      <v-text-field
        v-model="form.phone"
        label="Phone Number"
        type="tel"
        required
        autocomplete="tel"
        :error-messages="errors.phone"
        class="mb-3"
      />
    </fieldset>

    <v-btn
      type="submit"
      color="primary"
      size="large"
      block
      :loading="submitting"
      :disabled="submitting"
      min-height="48"
    >
      Reserve My Free Visit
    </v-btn>
  </v-form>
</template>
```

### Form Pattern with PrimeVue

```vue
<script setup lang="ts">
import { z } from 'zod'

const schema = z.object({
  name: z.string().min(2, 'Name is required'),
  email: z.string().email('Invalid email address'),
  message: z.string().min(10, 'Message must be at least 10 characters'),
})

const form = reactive({ name: '', email: '', message: '' })
const errors = ref<Record<string, string>>({})
const submitting = ref(false)

async function handleSubmit() {
  errors.value = {}
  const result = schema.safeParse(form)
  if (!result.success) {
    for (const issue of result.error.issues) {
      errors.value[issue.path[0] as string] = issue.message
    }
    return
  }

  submitting.value = true
  try {
    await $fetch('/api/contact', { method: 'POST', body: result.data })
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" novalidate>
    <fieldset class="space-y-4">
      <legend class="text-lg font-semibold mb-4">Contact Us</legend>

      <div>
        <label for="name" class="block text-sm font-medium mb-1">Full Name</label>
        <InputText
          id="name"
          v-model="form.name"
          autocomplete="name"
          :invalid="!!errors.name"
          class="w-full"
        />
        <small v-if="errors.name" class="text-red-500">{{ errors.name }}</small>
      </div>

      <div>
        <label for="email" class="block text-sm font-medium mb-1">Email Address</label>
        <InputText
          id="email"
          v-model="form.email"
          type="email"
          autocomplete="email"
          :invalid="!!errors.email"
          class="w-full"
        />
        <small v-if="errors.email" class="text-red-500">{{ errors.email }}</small>
      </div>

      <div>
        <label for="message" class="block text-sm font-medium mb-1">Message</label>
        <Textarea
          id="message"
          v-model="form.message"
          rows="4"
          :invalid="!!errors.message"
          class="w-full"
        />
        <small v-if="errors.message" class="text-red-500">{{ errors.message }}</small>
      </div>
    </fieldset>

    <Button
      type="submit"
      label="Send Message"
      :loading="submitting"
      :disabled="submitting"
      class="w-full mt-4"
      style="min-height: 48px"
    />
  </form>
</template>
```

### VeeValidate Integration (Complex Forms)

For forms with many fields or complex validation logic, use VeeValidate with Zod:

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['@vee-validate/nuxt'],
  veeValidate: {
    autoImports: true,
  },
})
```

```vue
<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod'
import { z } from 'zod'

const schema = toTypedSchema(
  z.object({
    name: z.string().min(2),
    email: z.string().email(),
    password: z.string().min(8),
    confirmPassword: z.string(),
  }).refine(data => data.password === data.confirmPassword, {
    message: 'Passwords must match',
    path: ['confirmPassword'],
  })
)

const { handleSubmit, isSubmitting } = useForm({ validationSchema: schema })

const onSubmit = handleSubmit(async (values) => {
  await $fetch('/api/auth/register', { method: 'POST', body: values })
  navigateTo('/dashboard')
})
</script>

<template>
  <form @submit="onSubmit" novalidate>
    <Field v-slot="{ field, errorMessage }" name="name">
      <v-text-field v-bind="field" label="Full Name" :error-messages="errorMessage" autocomplete="name" />
    </Field>

    <Field v-slot="{ field, errorMessage }" name="email">
      <v-text-field v-bind="field" label="Email" type="email" :error-messages="errorMessage" autocomplete="email" />
    </Field>

    <Field v-slot="{ field, errorMessage }" name="password">
      <v-text-field v-bind="field" label="Password" type="password" :error-messages="errorMessage" autocomplete="new-password" />
    </Field>

    <Field v-slot="{ field, errorMessage }" name="confirmPassword">
      <v-text-field v-bind="field" label="Confirm Password" type="password" :error-messages="errorMessage" autocomplete="new-password" />
    </Field>

    <v-btn type="submit" color="primary" size="large" block :loading="isSubmitting" min-height="48">
      Create Account
    </v-btn>
  </form>
</template>
```

---

## 15. Nuxt Modules & Ecosystem

### Essential Modules

| Module | Purpose | Install |
|---|---|---|
| `@nuxt/image` | Image optimization, responsive sizes, format conversion | `npx nuxi module add @nuxt/image` |
| `@nuxt/fonts` | Automatic font optimization (Google Fonts, local) | `npx nuxi module add @nuxt/fonts` |
| `@nuxt/content` | Markdown/MDC content management, blog engine | `npx nuxi module add @nuxt/content` |
| `@nuxt/eslint` | ESLint flat config with Vue + Nuxt rules | `npx nuxi module add @nuxt/eslint` |
| `@nuxt/test-utils` | Testing utilities (mountSuspended, mockNuxtImport) | Dev dependency |
| `@nuxt/devtools` | Visual dev tools (component inspector, route viewer) | Built into Nuxt 3.8+ |
| `@pinia/nuxt` | Pinia state management integration | `npx nuxi module add @pinia/nuxt` |
| `@vueuse/nuxt` | VueUse composables auto-imported | `npx nuxi module add @vueuse/nuxt` |

### Data & Auth Modules

| Module | Purpose | When to Use |
|---|---|---|
| `nuxt-auth-utils` | Session + OAuth auth (sealed cookies) | Simple auth needs |
| `@sidebase/nuxt-auth` | Auth.js (NextAuth) port for Nuxt | Complex OAuth + multiple providers |
| `nuxt-csurf` | CSRF protection | Cookie-based auth |
| `nuxt-security` | Security headers, rate limiting, CORS | Every production app |

### UI & Content Modules

| Module | Purpose | When to Use |
|---|---|---|
| `vuetify-nuxt-module` | Vuetify auto-config, tree-shaking | Vuetify projects |
| `@primevue/nuxt-module` | PrimeVue auto-import, theming | PrimeVue projects |
| `@nuxtjs/color-mode` | Dark/light mode with system preference | Theme switching |
| `@nuxtjs/i18n` | Internationalization, locale routing | Multi-language apps |
| `@nuxt/content` | Markdown/MDC-based content | Blogs, docs, marketing pages |

### SEO & Analytics Modules

| Module | Purpose | When to Use |
|---|---|---|
| `@nuxtjs/seo` | Meta pack (sitemap, robots, OG, schema.org) | Every public-facing site |
| `@nuxtjs/sitemap` | Auto-generated sitemap.xml | SEO |
| `nuxt-simple-robots` | robots.txt management | SEO |
| `nuxt-og-image` | Dynamic OG images | Social sharing |
| `nuxt-gtag` | Google Analytics 4 | Analytics |

### Module Configuration Pattern

```typescript
// nuxt.config.ts — real-world example
export default defineNuxtConfig({
  modules: [
    '@nuxt/image',
    '@nuxt/fonts',
    '@nuxt/eslint',
    '@pinia/nuxt',
    '@vueuse/nuxt',
    'nuxt-security',
    '@nuxtjs/color-mode',
    'vuetify-nuxt-module',
    '@nuxtjs/seo',
  ],

  // Module-specific config
  image: {
    quality: 80,
    formats: ['avif', 'webp'],
  },

  colorMode: {
    preference: 'system',
    fallback: 'light',
    classSuffix: '',
  },

  site: {
    url: 'https://example.com',
    name: 'My App',
    description: 'App description for SEO',
  },
})
```

### Nuxt Layers (Shared Code)

Nuxt Layers let you share components, composables, and config across projects:

```
// Base layer (e.g., a shared design system)
layers/
└── base-ui/
    ├── nuxt.config.ts
    ├── components/
    │   ├── BaseButton.vue
    │   └── BaseCard.vue
    ├── composables/
    │   └── useTheme.ts
    └── assets/
        └── styles/
            └── tokens.css
```

```typescript
// nuxt.config.ts (consuming project)
export default defineNuxtConfig({
  extends: [
    './layers/base-ui',               // Local layer
    'github:org/shared-ui#main',      // Remote layer (GitHub)
    '@my-org/nuxt-base',              // Published npm layer
  ],
})
```

**Convention:** Use layers for cross-project shared code (design systems, auth modules, common API patterns). Use modules for third-party integrations and framework extensions.

---

## 16. Anti-Patterns

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using Options API (`data()`, `methods`, `computed`) in Nuxt 3 | Use `<script setup lang="ts">` with Composition API everywhere |
| 2 | Using `$fetch` in components (causes double-fetch on SSR) | Use `useFetch` or `useAsyncData` — they deduplicate SSR/client fetches |
| 3 | Fetching data in `onMounted` | Use `useFetch`/`useAsyncData` in `<script setup>` — data loads during SSR |
| 4 | Accessing `window`/`document` without guards | Wrap in `if (import.meta.client)` or use `onMounted` lifecycle |
| 5 | Putting secrets in `runtimeConfig.public` | Secrets go in `runtimeConfig` (top level) — only server code can read them |
| 6 | Using `process.env` directly in client code | Use `useRuntimeConfig().public` — Nuxt handles environment injection |
| 7 | Manual component imports when auto-import works | Let Nuxt auto-import from `components/`, `composables/`, `utils/` |
| 8 | Naming composables without `use` prefix | Always prefix: `useAuth`, `useApi`, `usePagination` — convention and auto-import depend on it |
| 9 | Using `reactive()` for values you reassign entirely | Use `ref()` — reactive objects lose reactivity on reassignment |
| 10 | Creating global Pinia stores for data that should be SSR-fetched | Use `useFetch`/`useAsyncData` for server data. Pinia is for client UI state |
| 11 | Mixing multiple component libraries (Vuetify + PrimeVue + Naive UI) | Pick one at project start. They have conflicting CSS, themes, and conventions |
| 12 | Importing all Vuetify components (`import * as components`) in production | Use tree-shaking: import only used components or use `vuetify-nuxt-module` |
| 13 | Using Naive UI for SSR-heavy applications | Use Vuetify or PrimeVue — Naive UI has known hydration mismatch issues with SSR |
| 14 | Writing API routes without input validation | Always validate with Zod: `readValidatedBody(event, schema.parse)` |
| 15 | Storing auth tokens in `localStorage` | Use HTTP-only cookies via server-side auth. Client-accessible tokens are XSS-vulnerable |
| 16 | Using `navigateTo` in server API routes | `navigateTo` is for middleware/pages. In API routes, use `sendRedirect(event, url)` |
| 17 | Forms without `novalidate` attribute | Always add `novalidate` — HTML5 native validation is unreliable across assistive technologies |
| 18 | Forms without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, `autocomplete="tel"`, etc. |
| 19 | "Submit" button text | Use outcome-focused CTA: "Reserve My Free Visit", "Create Account", "Save Changes" |
| 20 | Deploying without quality gates | CI must run lint + typecheck + test + build before deploy |
| 21 | Missing security headers | Every deployment needs HSTS, CSP, X-Content-Type-Options at minimum |
| 22 | Using `watch` when `computed` suffices | If you are watching A to set B, use `computed` instead — it is declarative and cached |
| 23 | Deep watchers on large objects without specific path | Watch the specific property: `watch(() => state.user.name, ...)` not `watch(state, ..., { deep: true })` |
| 24 | Using `v-html` with user-supplied content | Always sanitize with DOMPurify. `v-html` is a raw innerHTML injection vector |
| 25 | Ignoring TypeScript strict mode | Fix all `nuxi typecheck` errors — they catch real bugs (null refs, wrong types, missing props) |
| 26 | Using `any` type to silence TypeScript | Define proper types. Use `unknown` + type narrowing if the type is truly unknown |
| 27 | Not setting `key` on `useFetch` when same URL with different params | Always provide explicit `key` when the same endpoint is called with different parameters |
| 28 | Using `useAsyncData` without a unique key | Always provide a globally unique string key as the first argument — collisions cause stale data |
| 29 | Creating a single monolithic Pinia store | One store per domain concept: `useAuthStore`, `useChapterStore`, `useNotificationStore` |
| 30 | Not handling all three states (pending/error/success) in templates | Always render loading, error, and success states — never assume data is available |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Nuxt patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:nuxt&title=[Nuxt]%20)**

Use the `patterns:nuxt` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
