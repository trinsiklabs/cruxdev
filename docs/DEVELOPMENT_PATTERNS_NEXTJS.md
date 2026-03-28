# Development Patterns — Next.js Stack

Next.js / React / Tailwind CSS / shadcn/ui / Prisma / Drizzle / TypeScript

This document captures stack-specific patterns, conventions, and decisions for Next.js stack projects (Next.js/React/Tailwind/shadcn/ui/Prisma or Drizzle/TypeScript). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure App Router pages, test with Vitest, use shadcn/ui, handle server actions, manage data with Prisma/Drizzle, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Node.js | 22+ LTS | Minimum 22 for native fetch, Web Crypto, test runner |
| Next.js | 15+ | App Router, React Server Components, Server Actions |
| React | 19+ | `use()`, Actions, `useOptimistic`, `useFormStatus`, `useActionState` |
| TypeScript | 5.6+ | `satisfies`, `const` type parameters, `NoInfer` |
| Tailwind CSS | 4.x | CSS-based configuration, `@theme`, `@variant` directives |
| shadcn/ui | latest | Radix UI + Tailwind — copy-paste components, full ownership |
| Prisma | 6+ | Type-safe ORM, migrations, Prisma Accelerate for connection pooling |
| Drizzle ORM | 0.38+ | Alternative: SQL-like type-safe ORM, zero abstraction overhead |
| NextAuth.js / Auth.js | 5.x (beta) | OAuth, credentials, JWT/session strategies |
| Zod | 3.24+ | Runtime schema validation for forms, server actions, API routes |
| React Hook Form | 7.54+ | Client-side form state management |
| Vitest | 3.x | Unit and integration test runner |
| React Testing Library | 16+ | Component testing — user-centric queries |
| Playwright | 1.50+ | End-to-end browser testing |
| MSW | 2.7+ | API mocking for tests and development |
| pnpm | 9+ | Package manager — strict, fast, disk-efficient |
| Zustand | 5+ | Lightweight client state management (when needed) |
| TanStack Query | 5+ | Server state management for client components |
| class-variance-authority | 0.7+ | Component variant API for Tailwind classes |
| clsx / tailwind-merge | latest | Class name composition — the `cn()` utility |

### Version Constraint Policy

Use exact versions in `package.json` for production dependencies, range for dev:

```jsonc
{
  "dependencies": {
    // Good — exact for production stability
    "next": "15.2.4",
    "react": "19.1.0",
    "@prisma/client": "6.4.1"
  },
  "devDependencies": {
    // Good — caret for dev tools (allows minor updates)
    "vitest": "^3.0.0",
    "typescript": "^5.6.0",
    "playwright": "^1.50.0"
  }
}
```

Exception: for packages that follow strict semver (Radix UI, Zod), caret ranges are acceptable in production.

### Next.js 15 Features to Use

| Feature | Use For |
|---|---|
| App Router (stable) | All routing — layouts, loading, error boundaries, parallel routes |
| React Server Components | Default for all components — data fetching at the component level |
| Server Actions | Mutations, form handling — replaces API routes for most writes |
| Partial Prerendering | Combine static shell with dynamic streaming content |
| `unstable_cache` / `cacheLife` | Fine-grained data caching with revalidation |
| `after()` | Run code after response is sent (analytics, logging) |
| Turbopack (dev) | Faster dev server HMR — enabled via `next dev --turbopack` |
| `forbidden()` / `unauthorized()` | HTTP 403/401 responses with custom boundary support |

### React 19 Features to Use

| Feature | Use For |
|---|---|
| `use()` | Read promises and context in render (replaces some `useEffect` patterns) |
| `useActionState` | Form action state management with pending/error states |
| `useFormStatus` | Pending state for submit buttons inside forms |
| `useOptimistic` | Optimistic UI updates during server action execution |
| Actions (`action` prop) | Native form submissions that work without JavaScript |
| `<form action={fn}>` | Progressive enhancement — server actions as form actions |
| `ref` as prop | No more `forwardRef` boilerplate |

---

## 2. Project Structure

### App Router Organization

Use the `src/` directory convention with route groups for logical separation:

```
project-name/
├── src/
│   ├── app/                          # App Router — routes and layouts
│   │   ├── layout.tsx                # Root layout (html, body, providers)
│   │   ├── page.tsx                  # Home page
│   │   ├── not-found.tsx             # Global 404
│   │   ├── error.tsx                 # Global error boundary
│   │   ├── loading.tsx               # Global loading UI
│   │   ├── global-error.tsx          # Root error boundary (catches layout errors)
│   │   ├── (marketing)/              # Route group — public pages, no auth
│   │   │   ├── layout.tsx            # Marketing layout (nav, footer)
│   │   │   ├── page.tsx              # Landing page (overrides root page)
│   │   │   ├── pricing/
│   │   │   │   └── page.tsx
│   │   │   ├── blog/
│   │   │   │   ├── page.tsx          # Blog index
│   │   │   │   └── [slug]/
│   │   │   │       └── page.tsx      # Blog post (dynamic segment)
│   │   │   └── about/
│   │   │       └── page.tsx
│   │   ├── (dashboard)/              # Route group — authenticated area
│   │   │   ├── layout.tsx            # Dashboard layout (sidebar, auth check)
│   │   │   ├── dashboard/
│   │   │   │   └── page.tsx
│   │   │   ├── settings/
│   │   │   │   ├── page.tsx
│   │   │   │   └── profile/
│   │   │   │       └── page.tsx
│   │   │   └── projects/
│   │   │       ├── page.tsx          # List
│   │   │       ├── new/
│   │   │       │   └── page.tsx      # Create form
│   │   │       └── [id]/
│   │   │           ├── page.tsx      # Detail
│   │   │           └── edit/
│   │   │               └── page.tsx  # Edit form
│   │   ├── api/                      # API routes (only when needed)
│   │   │   ├── webhooks/
│   │   │   │   └── stripe/
│   │   │   │       └── route.ts      # Webhook handler
│   │   │   └── cron/
│   │   │       └── route.ts          # Cron job endpoint
│   │   └── auth/                     # Auth pages
│   │       ├── login/
│   │       │   └── page.tsx
│   │       ├── register/
│   │       │   └── page.tsx
│   │       └── error/
│   │           └── page.tsx
│   ├── components/                   # Shared components
│   │   ├── ui/                       # shadcn/ui components (generated)
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   ├── form.tsx
│   │   │   ├── input.tsx
│   │   │   ├── select.tsx
│   │   │   └── ...
│   │   ├── layout/                   # Layout components
│   │   │   ├── header.tsx
│   │   │   ├── footer.tsx
│   │   │   ├── sidebar.tsx
│   │   │   └── mobile-nav.tsx
│   │   ├── forms/                    # Domain form components
│   │   │   ├── project-form.tsx
│   │   │   └── profile-form.tsx
│   │   └── shared/                   # Cross-cutting components
│   │       ├── data-table.tsx
│   │       ├── pagination.tsx
│   │       └── empty-state.tsx
│   ├── lib/                          # Utilities and configuration
│   │   ├── utils.ts                  # cn() and general utilities
│   │   ├── db.ts                     # Prisma/Drizzle client singleton
│   │   ├── auth.ts                   # Auth.js configuration
│   │   ├── auth-client.ts            # Client-side auth helpers
│   │   ├── validations/              # Zod schemas
│   │   │   ├── project.ts
│   │   │   ├── user.ts
│   │   │   └── shared.ts
│   │   └── constants.ts              # App-wide constants
│   ├── server/                       # Server-only code
│   │   ├── actions/                  # Server actions
│   │   │   ├── project.ts
│   │   │   ├── user.ts
│   │   │   └── auth.ts
│   │   ├── queries/                  # Data fetching functions
│   │   │   ├── project.ts
│   │   │   └── user.ts
│   │   └── services/                 # Business logic
│   │       ├── billing.ts
│   │       └── email.ts
│   ├── hooks/                        # Custom React hooks
│   │   ├── use-debounce.ts
│   │   ├── use-media-query.ts
│   │   └── use-copy-to-clipboard.ts
│   ├── types/                        # TypeScript type definitions
│   │   ├── index.ts
│   │   └── api.ts
│   ├── config/                       # App configuration
│   │   ├── site.ts                   # Site metadata
│   │   ├── nav.ts                    # Navigation config
│   │   └── dashboard.ts              # Dashboard sidebar config
│   ├── styles/                       # Global styles
│   │   └── globals.css               # Tailwind imports, CSS variables
│   └── middleware.ts                  # Next.js middleware (auth, redirects)
├── prisma/                           # Prisma schema and migrations
│   ├── schema.prisma
│   ├── seed.ts
│   └── migrations/
├── drizzle/                          # Drizzle schema and migrations (alternative)
│   ├── schema.ts
│   ├── seed.ts
│   └── migrations/
├── public/                           # Static assets
│   ├── images/
│   └── fonts/
├── tests/                            # Test files
│   ├── setup.ts                      # Global test setup
│   ├── utils.tsx                     # Test utilities (render with providers)
│   ├── mocks/                        # MSW handlers
│   │   ├── handlers.ts
│   │   └── server.ts
│   ├── unit/                         # Unit tests (mirror src/ structure)
│   │   ├── lib/
│   │   └── components/
│   ├── integration/                  # Integration tests
│   │   ├── actions/
│   │   └── api/
│   └── e2e/                          # Playwright E2E tests
│       ├── auth.spec.ts
│       ├── dashboard.spec.ts
│       └── fixtures/
├── next.config.ts                    # Next.js configuration
├── tailwind.config.ts                # Tailwind 4 — only needed for plugins
├── tsconfig.json
├── vitest.config.ts
├── playwright.config.ts
├── package.json
├── pnpm-lock.yaml
├── .env.local                        # Local secrets (git-ignored)
├── .env.example                      # Template for env vars (committed)
└── components.json                   # shadcn/ui configuration
```

**Convention:** Route groups `(marketing)` and `(dashboard)` keep URL paths clean while allowing different layouts. The parentheses mean the group name does not appear in the URL.

**Convention:** Colocate page-specific components inside the route directory. Shared components go in `src/components/`. Server-only code goes in `src/server/`.

**Convention:** The `src/lib/` directory is for framework configuration and utilities. Business logic lives in `src/server/services/`. Data fetching lives in `src/server/queries/`.

### File Naming Conventions

| Pattern | Convention | Example |
|---|---|---|
| Components | kebab-case file, PascalCase export | `data-table.tsx` exports `DataTable` |
| Hooks | kebab-case with `use-` prefix | `use-debounce.ts` exports `useDebounce` |
| Server actions | kebab-case, grouped by domain | `server/actions/project.ts` |
| Zod schemas | kebab-case, grouped by domain | `lib/validations/project.ts` |
| Route files | Next.js conventions | `page.tsx`, `layout.tsx`, `loading.tsx`, `error.tsx` |
| Test files | mirror source with `.test.ts(x)` | `data-table.test.tsx` |

### Import Aliases

Configure path aliases in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"],
      "@/components/*": ["./src/components/*"],
      "@/lib/*": ["./src/lib/*"],
      "@/server/*": ["./src/server/*"],
      "@/hooks/*": ["./src/hooks/*"],
      "@/types/*": ["./src/types/*"]
    }
  }
}
```

Always use `@/` aliases. Never use relative paths that traverse more than one level (`../../`).

---

## 3. React Server Components

### The Mental Model

In Next.js 15 with the App Router, every component is a **Server Component** by default. This is the single most important architectural decision in the stack. Server Components:

- Execute on the server only — never shipped to the client bundle
- Can directly access databases, file systems, and secrets
- Cannot use hooks (`useState`, `useEffect`, etc.)
- Cannot use browser APIs (`window`, `document`, etc.)
- Cannot use event handlers (`onClick`, `onChange`, etc.)

### When to Use `"use client"`

Add `"use client"` only when the component needs interactivity. The decision tree:

```
Does the component need...
├── useState, useEffect, useReducer, useRef?  → "use client"
├── onClick, onChange, onSubmit handlers?      → "use client"
├── Browser APIs (window, localStorage)?      → "use client"
├── Third-party hooks (useForm, useQuery)?    → "use client"
├── Context providers?                        → "use client"
│
├── Database access?                          → Server Component
├── Fetching data from APIs?                  → Server Component (preferred)
├── Accessing env vars / secrets?             → Server Component
├── Heavy dependencies (marked, highlight)?   → Server Component (keeps bundle small)
└── Static rendering with no interactivity?   → Server Component
```

### Server Component Data Fetching

Fetch data directly in Server Components — no `useEffect`, no loading states needed at the component level:

```tsx
// src/app/(dashboard)/projects/page.tsx
// This is a Server Component by default — no "use client" directive

import { db } from "@/lib/db"
import { ProjectCard } from "@/components/project-card"

export default async function ProjectsPage() {
  const projects = await db.project.findMany({
    where: { archived: false },
    orderBy: { updatedAt: "desc" },
    include: { owner: { select: { name: true, image: true } } },
  })

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {projects.map((project) => (
        <ProjectCard key={project.id} project={project} />
      ))}
    </div>
  )
}
```

### Server-Only Imports

Mark modules that must never be imported on the client:

```ts
// src/server/queries/project.ts
import "server-only"  // Throws build error if imported in client component

import { db } from "@/lib/db"
import { auth } from "@/lib/auth"

export async function getProjects() {
  const session = await auth()
  if (!session?.user?.id) throw new Error("Unauthorized")

  return db.project.findMany({
    where: { ownerId: session.user.id },
    orderBy: { updatedAt: "desc" },
  })
}
```

Install the `server-only` package: `pnpm add server-only`. Use it in every file under `src/server/`.

### Composition Pattern: Server + Client

The key pattern is to keep Server Components as parents and push `"use client"` to leaf components:

```tsx
// Server Component — fetches data
// src/app/(dashboard)/projects/[id]/page.tsx
import { getProject } from "@/server/queries/project"
import { ProjectEditor } from "@/components/forms/project-editor"
import { notFound } from "next/navigation"

export default async function ProjectPage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const { id } = await params
  const project = await getProject(id)
  if (!project) notFound()

  // Pass server-fetched data to client component as props
  return <ProjectEditor project={project} />
}
```

```tsx
// Client Component — handles interactivity
// src/components/forms/project-editor.tsx
"use client"

import { useState } from "react"
import { updateProject } from "@/server/actions/project"
import type { Project } from "@prisma/client"

export function ProjectEditor({ project }: { project: Project }) {
  const [name, setName] = useState(project.name)

  return (
    <form action={updateProject}>
      <input type="hidden" name="id" value={project.id} />
      <input
        name="name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <SubmitButton />
    </form>
  )
}
```

### Streaming with Suspense

Use `<Suspense>` to stream slow data independently:

```tsx
// src/app/(dashboard)/dashboard/page.tsx
import { Suspense } from "react"
import { StatsCards } from "@/components/dashboard/stats-cards"
import { RecentActivity } from "@/components/dashboard/recent-activity"
import { Skeleton } from "@/components/ui/skeleton"

export default function DashboardPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Dashboard</h1>

      {/* Stats load fast — no Suspense needed */}
      <Suspense fallback={<StatsCardsSkeleton />}>
        <StatsCards />
      </Suspense>

      {/* Activity is slow — stream independently */}
      <Suspense fallback={<Skeleton className="h-96 w-full" />}>
        <RecentActivity />
      </Suspense>
    </div>
  )
}
```

Each `<Suspense>` boundary streams its content independently. The page shell renders immediately, and each section fills in as its data resolves.

---

## 4. Server Actions

### Architecture

Server Actions are async functions that run on the server, invoked from client or server components. They replace API routes for most mutations.

```tsx
// src/server/actions/project.ts
"use server"

import { revalidatePath } from "next/cache"
import { redirect } from "next/navigation"
import { auth } from "@/lib/auth"
import { db } from "@/lib/db"
import { projectSchema } from "@/lib/validations/project"

export async function createProject(formData: FormData) {
  const session = await auth()
  if (!session?.user?.id) {
    throw new Error("Unauthorized")
  }

  const raw = {
    name: formData.get("name"),
    description: formData.get("description"),
  }

  const validated = projectSchema.parse(raw)

  const project = await db.project.create({
    data: {
      ...validated,
      ownerId: session.user.id,
    },
  })

  revalidatePath("/dashboard/projects")
  redirect(`/dashboard/projects/${project.id}`)
}
```

### Server Action Conventions

1. **Always validate input with Zod.** Never trust `FormData` — parse and validate before using.
2. **Always check authentication.** Every action must verify the session.
3. **Always check authorization.** Verify the user has permission for the specific resource.
4. **Return typed results.** Use a consistent return type for error handling.
5. **Revalidate after mutations.** Call `revalidatePath` or `revalidateTag` to update cached data.
6. **Keep actions thin.** Move business logic to `src/server/services/`. Actions orchestrate.

### Type-Safe Action Return Pattern

Use a consistent action result type instead of throwing errors:

```ts
// src/types/index.ts
export type ActionResult<T = void> =
  | { success: true; data: T }
  | { success: false; error: string; fieldErrors?: Record<string, string[]> }
```

```ts
// src/server/actions/project.ts
"use server"

import { auth } from "@/lib/auth"
import { db } from "@/lib/db"
import { projectSchema } from "@/lib/validations/project"
import type { ActionResult } from "@/types"

export async function createProject(
  _prevState: ActionResult,
  formData: FormData,
): Promise<ActionResult<{ id: string }>> {
  const session = await auth()
  if (!session?.user?.id) {
    return { success: false, error: "You must be logged in." }
  }

  const parsed = projectSchema.safeParse({
    name: formData.get("name"),
    description: formData.get("description"),
  })

  if (!parsed.success) {
    return {
      success: false,
      error: "Validation failed.",
      fieldErrors: parsed.error.flatten().fieldErrors,
    }
  }

  try {
    const project = await db.project.create({
      data: { ...parsed.data, ownerId: session.user.id },
    })

    revalidatePath("/dashboard/projects")
    return { success: true, data: { id: project.id } }
  } catch {
    return { success: false, error: "Failed to create project. Please try again." }
  }
}
```

### Using Actions with `useActionState`

```tsx
// src/components/forms/create-project-form.tsx
"use client"

import { useActionState } from "react"
import { createProject } from "@/server/actions/project"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import type { ActionResult } from "@/types"

const initialState: ActionResult = { success: false, error: "" }

export function CreateProjectForm() {
  const [state, action, isPending] = useActionState(createProject, initialState)

  return (
    <form action={action} noValidate>
      <div className="space-y-4">
        <div>
          <Label htmlFor="name">Project Name</Label>
          <Input
            id="name"
            name="name"
            required
            aria-describedby={state.fieldErrors?.name ? "name-error" : undefined}
            aria-invalid={!!state.fieldErrors?.name}
          />
          {state.fieldErrors?.name && (
            <p id="name-error" className="text-sm text-destructive mt-1">
              {state.fieldErrors.name[0]}
            </p>
          )}
        </div>

        <div>
          <Label htmlFor="description">Description</Label>
          <Input id="description" name="description" />
        </div>

        {!state.success && state.error && (
          <p className="text-sm text-destructive" role="alert">
            {state.error}
          </p>
        )}

        <Button type="submit" disabled={isPending}>
          {isPending ? "Creating..." : "Create Project"}
        </Button>
      </div>
    </form>
  )
}
```

### Optimistic Updates

Use `useOptimistic` for instant UI feedback:

```tsx
"use client"

import { useOptimistic } from "react"
import { toggleFavorite } from "@/server/actions/project"

export function FavoriteButton({
  projectId,
  isFavorited,
}: {
  projectId: string
  isFavorited: boolean
}) {
  const [optimisticFavorited, setOptimisticFavorited] = useOptimistic(isFavorited)

  return (
    <form
      action={async () => {
        setOptimisticFavorited(!optimisticFavorited)
        await toggleFavorite(projectId)
      }}
    >
      <button type="submit">
        {optimisticFavorited ? "★" : "☆"}
      </button>
    </form>
  )
}
```

---

## 5. Authentication

### Auth.js v5 (NextAuth.js v5) Setup

Authentication is handled by Auth.js — no custom auth code:

```ts
// src/lib/auth.ts
import NextAuth from "next-auth"
import GitHub from "next-auth/providers/github"
import Google from "next-auth/providers/google"
import Credentials from "next-auth/providers/credentials"
import { PrismaAdapter } from "@auth/prisma-adapter"
import { db } from "@/lib/db"
import bcrypt from "bcryptjs"
import { loginSchema } from "@/lib/validations/user"

export const { handlers, auth, signIn, signOut } = NextAuth({
  adapter: PrismaAdapter(db),
  session: { strategy: "jwt" },
  pages: {
    signIn: "/auth/login",
    error: "/auth/error",
  },
  providers: [
    GitHub({
      clientId: process.env.GITHUB_CLIENT_ID,
      clientSecret: process.env.GITHUB_CLIENT_SECRET,
    }),
    Google({
      clientId: process.env.GOOGLE_CLIENT_ID,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    }),
    Credentials({
      async authorize(credentials) {
        const parsed = loginSchema.safeParse(credentials)
        if (!parsed.success) return null

        const user = await db.user.findUnique({
          where: { email: parsed.data.email },
        })

        if (!user?.hashedPassword) return null

        const valid = await bcrypt.compare(parsed.data.password, user.hashedPassword)
        if (!valid) return null

        return { id: user.id, email: user.email, name: user.name, role: user.role }
      },
    }),
  ],
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        token.id = user.id
        token.role = user.role
      }
      return token
    },
    async session({ session, token }) {
      if (token) {
        session.user.id = token.id as string
        session.user.role = token.role as string
      }
      return session
    },
  },
})
```

### API Route Handler

```ts
// src/app/api/auth/[...nextauth]/route.ts
import { handlers } from "@/lib/auth"
export const { GET, POST } = handlers
```

### Middleware-Based Route Protection

```ts
// src/middleware.ts
import { auth } from "@/lib/auth"
import { NextResponse } from "next/server"

const publicRoutes = ["/", "/pricing", "/blog", "/auth/login", "/auth/register"]
const authRoutes = ["/auth/login", "/auth/register"]

export default auth((req) => {
  const { pathname } = req.nextUrl
  const isLoggedIn = !!req.auth

  // Redirect authenticated users away from auth pages
  if (isLoggedIn && authRoutes.some((route) => pathname.startsWith(route))) {
    return NextResponse.redirect(new URL("/dashboard", req.url))
  }

  // Protect all non-public routes
  if (!isLoggedIn && !publicRoutes.some((route) => pathname === route || pathname.startsWith(route + "/"))) {
    const loginUrl = new URL("/auth/login", req.url)
    loginUrl.searchParams.set("callbackUrl", pathname)
    return NextResponse.redirect(loginUrl)
  }

  return NextResponse.next()
})

export const config = {
  matcher: ["/((?!api/webhooks|_next/static|_next/image|favicon.ico|images/).*)"],
}
```

### Role-Based Access in Server Components

```tsx
// src/server/queries/user.ts
import "server-only"
import { auth } from "@/lib/auth"
import { redirect } from "next/navigation"

export async function requireAuth() {
  const session = await auth()
  if (!session?.user?.id) redirect("/auth/login")
  return session
}

export async function requireRole(role: "admin" | "member") {
  const session = await requireAuth()
  if (session.user.role !== role && session.user.role !== "admin") {
    redirect("/dashboard")
  }
  return session
}
```

```tsx
// Usage in a Server Component
import { requireRole } from "@/server/queries/user"

export default async function AdminPage() {
  await requireRole("admin")
  // ... admin content
}
```

### Type Augmentation

Extend the Auth.js types to include custom fields:

```ts
// src/types/next-auth.d.ts
import type { DefaultSession } from "next-auth"

declare module "next-auth" {
  interface Session {
    user: {
      id: string
      role: string
    } & DefaultSession["user"]
  }

  interface User {
    role: string
  }
}

declare module "next-auth/jwt" {
  interface JWT {
    id: string
    role: string
  }
}
```

---

## 6. Component Library — shadcn/ui

### Philosophy

Use shadcn/ui for all standard UI. shadcn/ui is not a traditional component library — it is a collection of copy-paste components built on Radix UI primitives and Tailwind CSS. You own the code. Customize freely.

Only build custom components for domain-specific UI (project dashboards, analytics visualizations, etc.).

### Installation

```bash
pnpm dlx shadcn@latest init
```

This creates `components.json` and sets up the `src/components/ui/` directory. Add components as needed:

```bash
pnpm dlx shadcn@latest add button card dialog form input label select textarea
pnpm dlx shadcn@latest add dropdown-menu navigation-menu sheet tabs table
pnpm dlx shadcn@latest add alert badge skeleton spinner toast
```

### The `cn()` Utility

The foundation of component styling — merges Tailwind classes with conflict resolution:

```ts
// src/lib/utils.ts
import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

Use `cn()` in every component that accepts `className`:

```tsx
import { cn } from "@/lib/utils"

export function Card({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      className={cn("rounded-lg border bg-card p-6 shadow-sm", className)}
      {...props}
    />
  )
}
```

### class-variance-authority (CVA) for Variants

Use CVA for components with multiple visual variants:

```tsx
// src/components/ui/button.tsx
import { cva, type VariantProps } from "class-variance-authority"
import { cn } from "@/lib/utils"

const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/90",
        destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        outline: "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        default: "h-10 px-4 py-2",
        sm: "h-9 rounded-md px-3",
        lg: "h-11 rounded-md px-8",
        icon: "h-10 w-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
)

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

export function Button({ className, variant, size, ...props }: ButtonProps) {
  return (
    <button
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}

export { buttonVariants }
```

### Extending shadcn/ui Components

When you need functionality beyond the base component, extend — do not fork:

```tsx
// src/components/ui/submit-button.tsx
"use client"

import { useFormStatus } from "react-dom"
import { Button, type ButtonProps } from "@/components/ui/button"
import { Loader2 } from "lucide-react"

export function SubmitButton({ children, ...props }: ButtonProps) {
  const { pending } = useFormStatus()

  return (
    <Button type="submit" disabled={pending} {...props}>
      {pending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
      {children}
    </Button>
  )
}
```

### Component Inventory (What shadcn/ui Gives Us)

| Category | Components | Use For |
|---|---|---|
| **Forms** | Input, Textarea, Select, Checkbox, Switch, RadioGroup, Slider, DatePicker | All forms |
| **Buttons** | Button (6 variants, 4 sizes), Toggle, ToggleGroup | Actions, CTAs |
| **Feedback** | Alert, Badge, Progress, Skeleton, Sonner (toast) | Status, notifications, loading |
| **Layout** | Card, Separator, Accordion, Collapsible, Tabs, ResizablePanel | Content grouping |
| **Navigation** | NavigationMenu, Breadcrumb, Pagination, Command (search) | Page navigation |
| **Overlay** | Dialog, Sheet, Drawer, DropdownMenu, ContextMenu, Popover, Tooltip, HoverCard | Modals, menus |
| **Data** | Table, DataTable (with TanStack Table) | Lists, admin tables |
| **Typography** | Heading, Paragraph (via Tailwind prose) | Content pages |

### Tailwind CSS 4 Integration

```css
/* src/styles/globals.css — Tailwind 4.x */
@import "tailwindcss";

@theme {
  --color-background: hsl(0 0% 100%);
  --color-foreground: hsl(222.2 84% 4.9%);
  --color-card: hsl(0 0% 100%);
  --color-card-foreground: hsl(222.2 84% 4.9%);
  --color-popover: hsl(0 0% 100%);
  --color-popover-foreground: hsl(222.2 84% 4.9%);
  --color-primary: hsl(222.2 47.4% 11.2%);
  --color-primary-foreground: hsl(210 40% 98%);
  --color-secondary: hsl(210 40% 96.1%);
  --color-secondary-foreground: hsl(222.2 47.4% 11.2%);
  --color-muted: hsl(210 40% 96.1%);
  --color-muted-foreground: hsl(215.4 16.3% 46.9%);
  --color-accent: hsl(210 40% 96.1%);
  --color-accent-foreground: hsl(222.2 47.4% 11.2%);
  --color-destructive: hsl(0 84.2% 60.2%);
  --color-destructive-foreground: hsl(210 40% 98%);
  --color-border: hsl(214.3 31.8% 91.4%);
  --color-input: hsl(214.3 31.8% 91.4%);
  --color-ring: hsl(222.2 84% 4.9%);
  --radius: 0.5rem;
}

.dark {
  --color-background: hsl(222.2 84% 4.9%);
  --color-foreground: hsl(210 40% 98%);
  /* ... dark theme overrides */
}
```

---

## 7. Testing Patterns

### Test Pyramid (Next.js-specific)

```
        /\
       /  \          E2E (Playwright) — critical user flows
      /    \
     /------\
    /        \        Integration Tests (Vitest + Testing Library)
   /          \       Server actions, API routes, component interactions
  /------------\
 /              \      Unit Tests (Vitest)
/                \     Pure functions, utilities, Zod schemas, hooks
/------------------\
```

### Vitest Configuration

```ts
// vitest.config.ts
import { defineConfig } from "vitest/config"
import react from "@vitejs/plugin-react"
import tsconfigPaths from "vite-tsconfig-paths"

export default defineConfig({
  plugins: [react(), tsconfigPaths()],
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./tests/setup.ts"],
    include: ["tests/**/*.test.{ts,tsx}", "src/**/*.test.{ts,tsx}"],
    exclude: ["tests/e2e/**"],
    coverage: {
      provider: "v8",
      reporter: ["text", "text-summary", "lcov", "html"],
      include: ["src/**/*.{ts,tsx}"],
      exclude: [
        "src/**/*.test.{ts,tsx}",
        "src/**/*.d.ts",
        "src/types/**",
        "src/app/**/layout.tsx",
        "src/app/**/loading.tsx",
        "src/app/**/not-found.tsx",
        "src/app/**/error.tsx",
        "src/components/ui/**",   // shadcn/ui generated — tested upstream
        "src/lib/utils.ts",       // trivial wrapper
      ],
      thresholds: {
        statements: 100,
        branches: 100,
        functions: 100,
        lines: 100,
      },
    },
  },
})
```

### Test Setup

```ts
// tests/setup.ts
import "@testing-library/jest-dom/vitest"
import { cleanup } from "@testing-library/react"
import { afterEach, vi } from "vitest"
import { server } from "./mocks/server"

// MSW server lifecycle
beforeAll(() => server.listen({ onUnhandledRequest: "error" }))
afterEach(() => {
  cleanup()
  server.resetHandlers()
})
afterAll(() => server.close())

// Mock next/navigation
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    refresh: vi.fn(),
    back: vi.fn(),
  }),
  usePathname: () => "/",
  useSearchParams: () => new URLSearchParams(),
  redirect: vi.fn(),
  notFound: vi.fn(),
}))
```

### Test Utilities

```tsx
// tests/utils.tsx
import { render, type RenderOptions } from "@testing-library/react"
import type { ReactElement } from "react"

// Add providers that wrap all components (theme, query client, etc.)
function AllProviders({ children }: { children: React.ReactNode }) {
  return <>{children}</>
}

function customRender(
  ui: ReactElement,
  options?: Omit<RenderOptions, "wrapper">,
) {
  return render(ui, { wrapper: AllProviders, ...options })
}

export * from "@testing-library/react"
export { customRender as render }
```

### Unit Tests (Pure Functions, Zod Schemas)

```ts
// tests/unit/lib/validations/project.test.ts
import { describe, it, expect } from "vitest"
import { projectSchema } from "@/lib/validations/project"

describe("projectSchema", () => {
  it("accepts valid project data", () => {
    const result = projectSchema.safeParse({
      name: "My Project",
      description: "A test project",
    })
    expect(result.success).toBe(true)
  })

  it("rejects empty name", () => {
    const result = projectSchema.safeParse({
      name: "",
      description: "A test project",
    })
    expect(result.success).toBe(false)
    if (!result.success) {
      expect(result.error.flatten().fieldErrors.name).toBeDefined()
    }
  })

  it("rejects name over 200 characters", () => {
    const result = projectSchema.safeParse({
      name: "a".repeat(201),
    })
    expect(result.success).toBe(false)
  })
})
```

### Component Tests (React Testing Library)

```tsx
// tests/unit/components/project-card.test.tsx
import { describe, it, expect } from "vitest"
import { render, screen } from "@/tests/utils"
import { ProjectCard } from "@/components/project-card"

const mockProject = {
  id: "1",
  name: "Test Project",
  description: "A test description",
  updatedAt: new Date("2025-01-01"),
  owner: { name: "Jane", image: null },
}

describe("ProjectCard", () => {
  it("renders project name and description", () => {
    render(<ProjectCard project={mockProject} />)
    expect(screen.getByText("Test Project")).toBeInTheDocument()
    expect(screen.getByText("A test description")).toBeInTheDocument()
  })

  it("renders owner name", () => {
    render(<ProjectCard project={mockProject} />)
    expect(screen.getByText("Jane")).toBeInTheDocument()
  })

  it("links to project detail page", () => {
    render(<ProjectCard project={mockProject} />)
    const link = screen.getByRole("link", { name: /test project/i })
    expect(link).toHaveAttribute("href", "/dashboard/projects/1")
  })
})
```

### Server Action Tests

Test server actions as async functions, mocking dependencies:

```ts
// tests/integration/actions/project.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest"
import { createProject } from "@/server/actions/project"

// Mock auth
vi.mock("@/lib/auth", () => ({
  auth: vi.fn(),
}))

// Mock db
vi.mock("@/lib/db", () => ({
  db: {
    project: {
      create: vi.fn(),
    },
  },
}))

// Mock next/cache
vi.mock("next/cache", () => ({
  revalidatePath: vi.fn(),
}))

import { auth } from "@/lib/auth"
import { db } from "@/lib/db"

describe("createProject", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("creates a project for authenticated user", async () => {
    vi.mocked(auth).mockResolvedValue({
      user: { id: "user-1", role: "member" },
    } as any)

    vi.mocked(db.project.create).mockResolvedValue({
      id: "project-1",
      name: "New Project",
    } as any)

    const formData = new FormData()
    formData.set("name", "New Project")
    formData.set("description", "A description")

    const result = await createProject({ success: false, error: "" }, formData)

    expect(result.success).toBe(true)
    expect(db.project.create).toHaveBeenCalledWith(
      expect.objectContaining({
        data: expect.objectContaining({
          name: "New Project",
          ownerId: "user-1",
        }),
      }),
    )
  })

  it("returns error for unauthenticated user", async () => {
    vi.mocked(auth).mockResolvedValue(null)

    const formData = new FormData()
    formData.set("name", "New Project")

    const result = await createProject({ success: false, error: "" }, formData)

    expect(result.success).toBe(false)
    expect(result.error).toContain("logged in")
  })
})
```

### MSW Handlers

```ts
// tests/mocks/handlers.ts
import { http, HttpResponse } from "msw"

export const handlers = [
  http.get("/api/projects", () => {
    return HttpResponse.json([
      { id: "1", name: "Project One" },
      { id: "2", name: "Project Two" },
    ])
  }),

  http.post("/api/projects", async ({ request }) => {
    const body = await request.json()
    return HttpResponse.json(
      { id: "3", ...body },
      { status: 201 },
    )
  }),
]
```

```ts
// tests/mocks/server.ts
import { setupServer } from "msw/node"
import { handlers } from "./handlers"

export const server = setupServer(...handlers)
```

### Playwright E2E Tests

```ts
// tests/e2e/auth.spec.ts
import { test, expect } from "@playwright/test"

test.describe("Authentication", () => {
  test("redirects unauthenticated user to login", async ({ page }) => {
    await page.goto("/dashboard")
    await expect(page).toHaveURL(/.*\/auth\/login/)
  })

  test("logs in with email and password", async ({ page }) => {
    await page.goto("/auth/login")
    await page.getByLabel("Email").fill("test@example.com")
    await page.getByLabel("Password").fill("password123")
    await page.getByRole("button", { name: "Log In" }).click()
    await expect(page).toHaveURL(/.*\/dashboard/)
    await expect(page.getByText("Dashboard")).toBeVisible()
  })
})
```

```ts
// playwright.config.ts
import { defineConfig, devices } from "@playwright/test"

export default defineConfig({
  testDir: "./tests/e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
  use: {
    baseURL: "http://localhost:3000",
    trace: "on-first-retry",
  },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] } },
    { name: "firefox", use: { ...devices["Desktop Firefox"] } },
    { name: "webkit", use: { ...devices["Desktop Safari"] } },
    { name: "mobile-chrome", use: { ...devices["Pixel 5"] } },
  ],
  webServer: {
    command: "pnpm run build && pnpm start",
    port: 3000,
    reuseExistingServer: !process.env.CI,
  },
})
```

---

## 8. Data Layer

### Prisma vs Drizzle — Decision Guide

| Factor | Prisma | Drizzle |
|---|---|---|
| **Learning curve** | Lower — declarative schema DSL | Higher — SQL-like TypeScript API |
| **Type safety** | Generated types from schema | Types from schema definition |
| **Query API** | High-level, ORM-like | SQL-like, composable |
| **Raw SQL** | `$queryRaw` / `$executeRaw` | First-class SQL support |
| **Performance** | Slightly higher overhead (Rust engine) | Zero overhead — compiles to SQL directly |
| **Migrations** | `prisma migrate` — declarative, auto-generated | `drizzle-kit` — diff-based, SQL output |
| **Edge runtime** | Prisma Accelerate required | Native edge support |
| **Connection pooling** | Prisma Accelerate or PgBouncer | PgBouncer, Neon serverless driver |
| **Relations** | Implicit many-to-many, nested writes | Explicit join tables, manual relations |
| **Ecosystem** | Larger — more adapters, auth integrations | Growing — lighter footprint |

**Decision rule:** Use Prisma when the team prefers ORM-style APIs and wants the broadest ecosystem support. Use Drizzle when the team wants SQL-like control, edge runtime support, or minimal runtime overhead.

### Prisma Setup

```prisma
// prisma/schema.prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id             String    @id @default(cuid())
  name           String?
  email          String    @unique
  emailVerified  DateTime?
  image          String?
  hashedPassword String?
  role           Role      @default(MEMBER)
  createdAt      DateTime  @default(now())
  updatedAt      DateTime  @updatedAt

  accounts Account[]
  sessions Session[]
  projects Project[]

  @@map("users")
}

model Project {
  id          String   @id @default(cuid())
  name        String
  description String?
  archived    Boolean  @default(false)
  ownerId     String
  createdAt   DateTime @default(now())
  updatedAt   DateTime @updatedAt

  owner User @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@map("projects")
}

enum Role {
  MEMBER
  ADMIN
}
```

### Prisma Client Singleton

```ts
// src/lib/db.ts
import { PrismaClient } from "@prisma/client"

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined
}

export const db =
  globalForPrisma.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === "development" ? ["query", "error", "warn"] : ["error"],
  })

if (process.env.NODE_ENV !== "production") globalForPrisma.prisma = db
```

**Why the singleton?** Next.js hot-reloads in development, which creates new `PrismaClient` instances on each reload. Without the global cache, you exhaust database connections.

### Prisma Migrations

```bash
# Generate migration from schema changes
pnpm prisma migrate dev --name describe_the_change

# Apply migrations in production
pnpm prisma migrate deploy

# Reset database (dev only)
pnpm prisma migrate reset

# Generate Prisma Client (runs automatically after migrate)
pnpm prisma generate

# Open Prisma Studio (visual DB browser)
pnpm prisma studio
```

Never edit a migration after it has been committed. Write a new corrective migration instead.

### Drizzle Setup (Alternative)

```ts
// drizzle/schema.ts
import { pgTable, text, timestamp, boolean, pgEnum } from "drizzle-orm/pg-core"
import { createId } from "@paralleldrive/cuid2"

export const roleEnum = pgEnum("role", ["MEMBER", "ADMIN"])

export const users = pgTable("users", {
  id: text("id").primaryKey().$defaultFn(() => createId()),
  name: text("name"),
  email: text("email").unique().notNull(),
  emailVerified: timestamp("email_verified"),
  image: text("image"),
  hashedPassword: text("hashed_password"),
  role: roleEnum("role").default("MEMBER").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  updatedAt: timestamp("updated_at").defaultNow().notNull(),
})

export const projects = pgTable("projects", {
  id: text("id").primaryKey().$defaultFn(() => createId()),
  name: text("name").notNull(),
  description: text("description"),
  archived: boolean("archived").default(false).notNull(),
  ownerId: text("owner_id").notNull().references(() => users.id, { onDelete: "cascade" }),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  updatedAt: timestamp("updated_at").defaultNow().notNull(),
})
```

```ts
// src/lib/db.ts (Drizzle variant)
import { drizzle } from "drizzle-orm/node-postgres"
import * as schema from "../../drizzle/schema"

export const db = drizzle(process.env.DATABASE_URL!, { schema })
```

### Drizzle Queries

```ts
// SQL-like query API
import { db } from "@/lib/db"
import { projects, users } from "../../drizzle/schema"
import { eq, desc } from "drizzle-orm"

// Simple select
const allProjects = await db
  .select()
  .from(projects)
  .where(eq(projects.archived, false))
  .orderBy(desc(projects.updatedAt))

// Join
const projectsWithOwners = await db
  .select({
    project: projects,
    ownerName: users.name,
  })
  .from(projects)
  .leftJoin(users, eq(projects.ownerId, users.id))

// Relational query API (Prisma-like)
const result = await db.query.projects.findMany({
  where: eq(projects.archived, false),
  with: { owner: { columns: { name: true, image: true } } },
  orderBy: desc(projects.updatedAt),
})
```

### Drizzle Migrations

```bash
# Generate migration from schema diff
pnpm drizzle-kit generate

# Apply migrations
pnpm drizzle-kit migrate

# Open Drizzle Studio (visual DB browser)
pnpm drizzle-kit studio
```

### Connection Pooling

For serverless deployments (Vercel, AWS Lambda), connection pooling is mandatory:

| Approach | When to Use |
|---|---|
| **Prisma Accelerate** | Prisma projects on Vercel — managed connection pool + edge caching |
| **Neon serverless driver** | Drizzle with Neon Postgres — HTTP-based, no persistent connections |
| **PgBouncer** | Self-hosted — sits between app and PostgreSQL |
| **Supabase Supavisor** | Supabase-hosted Postgres — built-in pooler on port 6543 |

```ts
// Prisma with Accelerate
// .env
// DATABASE_URL="prisma://accelerate.prisma-data.net/?api_key=..."

// Drizzle with Neon serverless
import { neon } from "@neondatabase/serverless"
import { drizzle } from "drizzle-orm/neon-http"

const sql = neon(process.env.DATABASE_URL!)
export const db = drizzle(sql)
```

### Seed Data

```ts
// prisma/seed.ts
import { PrismaClient } from "@prisma/client"
import bcrypt from "bcryptjs"

const prisma = new PrismaClient()

async function main() {
  // Upsert to make seeds idempotent
  await prisma.user.upsert({
    where: { email: "admin@example.com" },
    update: {},
    create: {
      email: "admin@example.com",
      name: "Admin User",
      hashedPassword: await bcrypt.hash("password123", 12),
      role: "ADMIN",
    },
  })

  console.log("Seed complete.")
}

main()
  .catch((e) => {
    console.error(e)
    process.exit(1)
  })
  .finally(async () => {
    await prisma.$disconnect()
  })
```

```json
// package.json
{
  "prisma": {
    "seed": "tsx prisma/seed.ts"
  }
}
```

---

## 9. State Management

### State Management Decision Tree

```
What kind of state is it?
│
├── URL state (filters, search, pagination, tabs)?
│   → useSearchParams + URL manipulation
│   → nuqs library for type-safe URL state
│
├── Server state (data from DB/API)?
│   ├── In Server Component → fetch directly (no state needed)
│   └── In Client Component → TanStack Query (useQuery / useMutation)
│
├── Form state?
│   → React Hook Form + Zod resolver
│   → Or useActionState for server action forms
│
├── Ephemeral UI state (modal open, accordion, hover)?
│   → useState (local to the component)
│
├── Shared client state (theme, sidebar collapsed, toast queue)?
│   ├── Small + stable → React Context
│   └── Complex + frequently updating → Zustand
│
└── Optimistic state?
    → useOptimistic (React 19)
```

### URL State (Preferred for Filters, Search, Pagination)

URL state is shareable, bookmarkable, and survives refresh. Prefer it over React state for any user-facing filter or navigation:

```tsx
// src/app/(dashboard)/projects/page.tsx
import { Suspense } from "react"

// Server Component reads search params
export default async function ProjectsPage({
  searchParams,
}: {
  searchParams: Promise<{ q?: string; page?: string; sort?: string }>
}) {
  const { q, page = "1", sort = "updatedAt" } = await searchParams

  const projects = await db.project.findMany({
    where: q ? { name: { contains: q, mode: "insensitive" } } : undefined,
    orderBy: { [sort]: "desc" },
    skip: (parseInt(page) - 1) * 20,
    take: 20,
  })

  return (
    <>
      <Suspense fallback={null}>
        <SearchFilter />
      </Suspense>
      <ProjectList projects={projects} />
    </>
  )
}
```

```tsx
// src/components/search-filter.tsx
"use client"

import { useSearchParams, useRouter, usePathname } from "next/navigation"
import { useDebouncedCallback } from "use-debounce"
import { Input } from "@/components/ui/input"

export function SearchFilter() {
  const searchParams = useSearchParams()
  const pathname = usePathname()
  const { replace } = useRouter()

  const handleSearch = useDebouncedCallback((term: string) => {
    const params = new URLSearchParams(searchParams.toString())
    if (term) {
      params.set("q", term)
      params.set("page", "1") // Reset pagination on new search
    } else {
      params.delete("q")
    }
    replace(`${pathname}?${params.toString()}`)
  }, 300)

  return (
    <Input
      placeholder="Search projects..."
      defaultValue={searchParams.get("q") ?? ""}
      onChange={(e) => handleSearch(e.target.value)}
    />
  )
}
```

### Zustand (When Client State Is Needed)

```ts
// src/stores/sidebar-store.ts
import { create } from "zustand"
import { persist } from "zustand/middleware"

interface SidebarState {
  isCollapsed: boolean
  toggle: () => void
  setCollapsed: (collapsed: boolean) => void
}

export const useSidebarStore = create<SidebarState>()(
  persist(
    (set) => ({
      isCollapsed: false,
      toggle: () => set((state) => ({ isCollapsed: !state.isCollapsed })),
      setCollapsed: (collapsed) => set({ isCollapsed: collapsed }),
    }),
    { name: "sidebar-state" },
  ),
)
```

### TanStack Query (Server State in Client Components)

Use when a client component needs to fetch or mutate server data independently of RSC:

```tsx
// src/hooks/use-projects.ts
"use client"

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"

export function useProjects() {
  return useQuery({
    queryKey: ["projects"],
    queryFn: async () => {
      const res = await fetch("/api/projects")
      if (!res.ok) throw new Error("Failed to fetch projects")
      return res.json()
    },
  })
}

export function useDeleteProject() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (id: string) => {
      const res = await fetch(`/api/projects/${id}`, { method: "DELETE" })
      if (!res.ok) throw new Error("Failed to delete project")
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["projects"] })
    },
  })
}
```

**Rule:** Prefer Server Components and server actions for data fetching and mutations. Only use TanStack Query when you need real-time polling, infinite scroll, or complex client-side cache management.

---

## 10. Performance

### Rendering Strategy Decision Tree

```
How often does the content change?
│
├── Never (legal pages, docs, marketing)
│   → Static (SSG) — generateStaticParams + no revalidate
│   → Build time only, fastest possible
│
├── Rarely (blog posts, product pages)
│   → ISR — revalidate: 3600 (or on-demand with revalidateTag)
│   → Static at build, rebuilt on schedule or trigger
│
├── Per-request but cacheable (dashboard, user-specific)
│   → SSR with caching — dynamic + cache headers
│   → Or use unstable_cache with user-scoped keys
│
├── Real-time (chat, notifications, live data)
│   → SSR + client-side polling/WebSocket
│   → Or streaming with Suspense boundaries
│
└── Mixed (static shell + dynamic content)
    → Partial Prerendering (Next.js 15)
    → Static shell streams, dynamic holes fill in
```

### Static Generation (SSG)

```tsx
// src/app/blog/[slug]/page.tsx

// Generate static pages at build time
export async function generateStaticParams() {
  const posts = await db.post.findMany({ select: { slug: true } })
  return posts.map((post) => ({ slug: post.slug }))
}

// No revalidate = fully static
export default async function BlogPost({
  params,
}: {
  params: Promise<{ slug: string }>
}) {
  const { slug } = await params
  const post = await db.post.findUnique({ where: { slug } })
  if (!post) notFound()

  return <article>{post.content}</article>
}
```

### Incremental Static Regeneration (ISR)

```tsx
// Revalidate every hour
export const revalidate = 3600

// Or use on-demand revalidation in a server action
import { revalidateTag } from "next/cache"

export async function updatePost(formData: FormData) {
  // ... update post in DB
  revalidateTag("posts")  // Invalidate all pages tagged "posts"
}

// Tag data fetching functions
import { unstable_cache } from "next/cache"

const getPosts = unstable_cache(
  async () => db.post.findMany({ orderBy: { createdAt: "desc" } }),
  ["posts"],
  { tags: ["posts"], revalidate: 3600 },
)
```

### Dynamic Imports

Lazy-load heavy client components:

```tsx
import dynamic from "next/dynamic"

// Heavy charting library — only loaded when rendered
const Chart = dynamic(() => import("@/components/chart"), {
  loading: () => <Skeleton className="h-64 w-full" />,
  ssr: false, // Client-only — no server rendering
})

// Code editor — heavy, client-only
const CodeEditor = dynamic(() => import("@/components/code-editor"), {
  loading: () => <Skeleton className="h-96 w-full" />,
  ssr: false,
})
```

### Image Optimization

Always use `next/image` — never raw `<img>` tags:

```tsx
import Image from "next/image"

// Local image — automatic optimization
<Image
  src="/images/hero.png"
  alt="Hero illustration"
  width={1200}
  height={630}
  priority           // Above-the-fold — preload
  className="rounded-lg"
/>

// Remote image — must configure domains in next.config.ts
<Image
  src={user.image}
  alt={user.name}
  width={40}
  height={40}
  className="rounded-full"
/>

// Fill mode — responsive container
<div className="relative aspect-video">
  <Image
    src={post.coverImage}
    alt={post.title}
    fill
    sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
    className="object-cover rounded-lg"
  />
</div>
```

```ts
// next.config.ts
import type { NextConfig } from "next"

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "avatars.githubusercontent.com",
      },
      {
        protocol: "https",
        hostname: "lh3.googleusercontent.com",
      },
    ],
  },
}

export default nextConfig
```

### Streaming and Suspense Boundaries

Place Suspense boundaries at data-fetching boundaries, not around every component:

```tsx
// Good — each section streams independently
export default function DashboardPage() {
  return (
    <div className="space-y-6">
      <Suspense fallback={<StatsSkeleton />}>
        <StatsCards />
      </Suspense>
      <div className="grid gap-6 md:grid-cols-2">
        <Suspense fallback={<ChartSkeleton />}>
          <RevenueChart />
        </Suspense>
        <Suspense fallback={<ListSkeleton />}>
          <RecentOrders />
        </Suspense>
      </div>
    </div>
  )
}

// Bad — single Suspense wrapping everything (no streaming benefit)
export default function DashboardPage() {
  return (
    <Suspense fallback={<FullPageSkeleton />}>
      <StatsCards />
      <RevenueChart />
      <RecentOrders />
    </Suspense>
  )
}
```

### Bundle Analysis

```bash
# Analyze bundle size
ANALYZE=true pnpm build

# next.config.ts
import withBundleAnalyzer from "@next/bundle-analyzer"

const nextConfig: NextConfig = {
  // ...
}

export default process.env.ANALYZE === "true"
  ? withBundleAnalyzer({ enabled: true })(nextConfig)
  : nextConfig
```

---

## 11. Deployment

### Vercel (Primary)

```json
// vercel.json (only needed for overrides — Vercel auto-detects Next.js)
{
  "buildCommand": "pnpm build",
  "installCommand": "pnpm install",
  "framework": "nextjs",
  "crons": [
    {
      "path": "/api/cron",
      "schedule": "0 * * * *"
    }
  ]
}
```

Environment variables:
- Set secrets in Vercel dashboard (Settings > Environment Variables)
- Use Preview/Production/Development scoping
- Never prefix secrets with `NEXT_PUBLIC_` — those are exposed to the client

### Self-Hosted Docker

```dockerfile
# Dockerfile
FROM node:22-alpine AS base

# Install dependencies
FROM base AS deps
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable pnpm && pnpm install --frozen-lockfile

# Build
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN corepack enable pnpm && pnpm build

# Production
FROM base AS runner
WORKDIR /app
ENV NODE_ENV=production

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs
EXPOSE 3000
ENV PORT=3000
CMD ["node", "server.js"]
```

```ts
// next.config.ts — enable standalone output for Docker
const nextConfig: NextConfig = {
  output: "standalone",
}
```

### Edge Runtime

Use edge runtime for middleware and latency-sensitive API routes:

```ts
// src/app/api/geo/route.ts
export const runtime = "edge"

export async function GET(request: Request) {
  const country = request.headers.get("x-vercel-ip-country") ?? "US"
  return Response.json({ country })
}
```

**Limitations of edge runtime:**
- No Node.js APIs (`fs`, `crypto` beyond Web Crypto, `child_process`)
- No Prisma Client (use Prisma Accelerate or Drizzle with serverless drivers)
- 128KB code size limit after bundling on some platforms
- No `eval()` or `new Function()`

### CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "22"
          cache: "pnpm"
      - run: pnpm install --frozen-lockfile
      - run: pnpm prisma migrate deploy
        env:
          DATABASE_URL: postgresql://test:test@localhost:5432/test
      - run: pnpm lint
      - run: pnpm typecheck
      - run: pnpm test:coverage
      - run: pnpm build

  e2e:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "22"
          cache: "pnpm"
      - run: pnpm install --frozen-lockfile
      - run: pnpm exec playwright install --with-deps
      - run: pnpm test:e2e
```

### Common Commands

```bash
# Development
pnpm dev                              # Start dev server (Turbopack)
pnpm dev --turbopack                  # Explicit Turbopack (default in Next 15)
pnpm build                            # Production build
pnpm start                            # Start production server
pnpm lint                             # ESLint
pnpm typecheck                        # tsc --noEmit

# Testing
pnpm test                             # Run Vitest
pnpm test:watch                       # Vitest in watch mode
pnpm test:coverage                    # Vitest with coverage
pnpm test:e2e                         # Playwright E2E tests
pnpm test:e2e --ui                    # Playwright with UI

# Database
pnpm prisma migrate dev               # Create + apply migration
pnpm prisma migrate deploy            # Apply pending migrations (production)
pnpm prisma migrate reset             # Drop + recreate + seed
pnpm prisma studio                    # Visual database browser
pnpm prisma db seed                   # Run seed script

# Components
pnpm dlx shadcn@latest add button     # Add shadcn/ui component

# Quality
pnpm lint                             # ESLint
pnpm typecheck                        # TypeScript check
pnpm test:coverage                    # Coverage report
pnpm build                            # Build (catches SSR errors)
```

### Package.json Scripts

```json
{
  "scripts": {
    "dev": "next dev --turbopack",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "typecheck": "tsc --noEmit",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage",
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "db:migrate": "prisma migrate dev",
    "db:push": "prisma db push",
    "db:seed": "prisma db seed",
    "db:studio": "prisma studio",
    "quality": "pnpm lint && pnpm typecheck && pnpm test:coverage && pnpm build"
  }
}
```

---

## 12. Security

### Server Action Validation with Zod

Every server action must validate all input. Never trust client-side validation alone:

```ts
// src/lib/validations/project.ts
import { z } from "zod"

export const projectSchema = z.object({
  name: z
    .string()
    .min(1, "Name is required")
    .max(200, "Name must be 200 characters or less")
    .trim(),
  description: z
    .string()
    .max(2000, "Description must be 2000 characters or less")
    .optional()
    .transform((val) => val || undefined),
})

export type ProjectInput = z.infer<typeof projectSchema>
```

```ts
// In the server action — always parse, never trust
const parsed = projectSchema.safeParse({
  name: formData.get("name"),
  description: formData.get("description"),
})

if (!parsed.success) {
  return {
    success: false,
    error: "Validation failed.",
    fieldErrors: parsed.error.flatten().fieldErrors,
  }
}
```

### Environment Variable Security

```
.env.local              → Server-only secrets (DATABASE_URL, AUTH_SECRET)
.env                    → Non-secret defaults (NEXT_PUBLIC_APP_URL)
.env.example            → Template (committed, no real values)
```

**Critical rule:** Variables prefixed with `NEXT_PUBLIC_` are bundled into the client JavaScript and visible to anyone. Never put secrets there.

```bash
# GOOD — server-only
DATABASE_URL="postgresql://..."
AUTH_SECRET="super-secret-key"
STRIPE_SECRET_KEY="sk_live_..."

# GOOD — public (safe to expose)
NEXT_PUBLIC_APP_URL="https://myapp.com"
NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY="pk_live_..."

# BAD — secret exposed to client
NEXT_PUBLIC_DATABASE_URL="..."       # NEVER
NEXT_PUBLIC_AUTH_SECRET="..."        # NEVER
NEXT_PUBLIC_STRIPE_SECRET_KEY="..."  # NEVER
```

### Runtime Environment Validation

Validate environment variables at build time with `@t3-oss/env-nextjs`:

```ts
// src/lib/env.ts
import { createEnv } from "@t3-oss/env-nextjs"
import { z } from "zod"

export const env = createEnv({
  server: {
    DATABASE_URL: z.string().url(),
    AUTH_SECRET: z.string().min(32),
    STRIPE_SECRET_KEY: z.string().startsWith("sk_"),
  },
  client: {
    NEXT_PUBLIC_APP_URL: z.string().url(),
    NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY: z.string().startsWith("pk_"),
  },
  runtimeEnv: {
    DATABASE_URL: process.env.DATABASE_URL,
    AUTH_SECRET: process.env.AUTH_SECRET,
    STRIPE_SECRET_KEY: process.env.STRIPE_SECRET_KEY,
    NEXT_PUBLIC_APP_URL: process.env.NEXT_PUBLIC_APP_URL,
    NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY: process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY,
  },
})
```

### CSRF Protection

Server Actions have built-in CSRF protection — Next.js automatically validates the `Origin` header. For custom API routes, add explicit checks:

```ts
// src/app/api/webhooks/stripe/route.ts
import { headers } from "next/headers"
import Stripe from "stripe"

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY!)

export async function POST(request: Request) {
  const body = await request.text()
  const headersList = await headers()
  const signature = headersList.get("stripe-signature")

  if (!signature) {
    return new Response("Missing signature", { status: 400 })
  }

  try {
    const event = stripe.webhooks.constructEvent(
      body,
      signature,
      process.env.STRIPE_WEBHOOK_SECRET!,
    )

    // Process webhook event
    switch (event.type) {
      case "checkout.session.completed":
        // Handle payment
        break
    }

    return new Response("OK", { status: 200 })
  } catch (err) {
    return new Response("Invalid signature", { status: 400 })
  }
}
```

### Content Security Policy

```ts
// next.config.ts
const cspHeader = `
  default-src 'self';
  script-src 'self' 'unsafe-eval' 'unsafe-inline';
  style-src 'self' 'unsafe-inline';
  img-src 'self' blob: data: https:;
  font-src 'self';
  connect-src 'self' https:;
  frame-ancestors 'self';
  form-action 'self';
  base-uri 'self';
`

const nextConfig: NextConfig = {
  async headers() {
    return [
      {
        source: "/(.*)",
        headers: [
          {
            key: "Content-Security-Policy",
            value: cspHeader.replace(/\n/g, ""),
          },
          {
            key: "X-Content-Type-Options",
            value: "nosniff",
          },
          {
            key: "Referrer-Policy",
            value: "strict-origin-when-cross-origin",
          },
          {
            key: "X-Frame-Options",
            value: "DENY",
          },
          {
            key: "Strict-Transport-Security",
            value: "max-age=31536000; includeSubDomains",
          },
        ],
      },
    ]
  },
}
```

**Notes:**
- `unsafe-eval` is required by Next.js in development — remove in production if possible
- `unsafe-inline` is needed for Tailwind and Next.js inline styles
- Nonce-based CSP is preferred for production — use `next/headers` to generate nonces

### Rate Limiting

For API routes and server actions, use `@upstash/ratelimit` or a similar library:

```ts
// src/lib/rate-limit.ts
import { Ratelimit } from "@upstash/ratelimit"
import { Redis } from "@upstash/redis"

export const ratelimit = new Ratelimit({
  redis: Redis.fromEnv(),
  limiter: Ratelimit.slidingWindow(10, "10 s"),
  analytics: true,
})
```

```ts
// In a server action
import { ratelimit } from "@/lib/rate-limit"
import { headers } from "next/headers"

export async function submitForm(formData: FormData) {
  const headersList = await headers()
  const ip = headersList.get("x-forwarded-for") ?? "127.0.0.1"

  const { success } = await ratelimit.limit(ip)
  if (!success) {
    return { success: false, error: "Too many requests. Please try again later." }
  }

  // ... process form
}
```

---

## 13. Coverage Enforcement

Test coverage is enforced via Vitest with v8 provider:

```ts
// vitest.config.ts — coverage section
coverage: {
  provider: "v8",
  reporter: ["text", "text-summary", "lcov", "html"],
  include: ["src/**/*.{ts,tsx}"],
  exclude: [
    "src/**/*.test.{ts,tsx}",
    "src/**/*.d.ts",
    "src/types/**",
    "src/app/**/layout.tsx",       // Thin wrappers
    "src/app/**/loading.tsx",      // Skeleton UI
    "src/app/**/not-found.tsx",    // Static pages
    "src/app/**/error.tsx",        // Error boundaries
    "src/components/ui/**",        // shadcn/ui generated code
    "src/lib/utils.ts",            // Trivial utility
    "src/middleware.ts",           // Tested via E2E
  ],
  thresholds: {
    statements: 100,
    branches: 100,
    functions: 100,
    lines: 100,
  },
},
```

**Commands:**

```bash
pnpm test:coverage                    # Coverage report to terminal
pnpm vitest run --coverage            # Same as above
# HTML report generated at coverage/index.html
```

Target is 100% (per CLAUDE.md core rules). The `thresholds` in `vitest.config.ts` are the hard gate — CI fails below these numbers.

### What to Exclude from Coverage

| Exclusion | Reason |
|---|---|
| `src/components/ui/**` | shadcn/ui generated code — tested by the upstream project |
| `src/app/**/layout.tsx` | Thin wrapper files — provider composition only |
| `src/app/**/loading.tsx` | Static skeleton UI — no logic to test |
| `src/app/**/error.tsx` | Error boundary boilerplate — tested via E2E |
| `src/types/**` | Type definitions — no runtime code |
| `src/middleware.ts` | Best tested via E2E — Vitest cannot simulate edge runtime |

### Coverage in CI

```yaml
# In GitHub Actions
- run: pnpm test:coverage
  env:
    DATABASE_URL: postgresql://test:test@localhost:5432/test
```

The coverage command fails the build if any threshold is not met. No separate enforcement step needed.

---

## 14. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|---|---|---|
| **layout** | Single column, logical grouping with fieldsets |
| **labels** | Top-aligned, visible `<Label>`, optional fields marked "(optional)" |
| **validation** | Server-side Zod validation, client-side progressive enhancement |
| **errors** | Inline + summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `noValidate` on form, `autoComplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets (h-12), `autoComplete` |
| **cta** | Outcome-focused text ("Create Project" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | Progressive enhancement — forms work without JavaScript |

### React Hook Form + Zod Pattern

```tsx
// src/components/forms/project-form.tsx
"use client"

import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { projectSchema, type ProjectInput } from "@/lib/validations/project"
import { createProject } from "@/server/actions/project"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { useRouter } from "next/navigation"
import { toast } from "sonner"

export function ProjectForm() {
  const router = useRouter()

  const form = useForm<ProjectInput>({
    resolver: zodResolver(projectSchema),
    defaultValues: {
      name: "",
      description: "",
    },
  })

  async function onSubmit(values: ProjectInput) {
    const formData = new FormData()
    formData.set("name", values.name)
    if (values.description) formData.set("description", values.description)

    const result = await createProject(
      { success: false, error: "" },
      formData,
    )

    if (result.success) {
      toast.success("Project created!")
      router.push(`/dashboard/projects/${result.data.id}`)
    } else {
      toast.error(result.error)
      // Map server field errors back to form
      if (result.fieldErrors) {
        for (const [field, messages] of Object.entries(result.fieldErrors)) {
          form.setError(field as keyof ProjectInput, {
            message: messages[0],
          })
        }
      }
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        noValidate
        className="space-y-6"
      >
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Project Name</FormLabel>
              <FormControl>
                <Input
                  {...field}
                  autoComplete="organization"
                  aria-required="true"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="description"
          render={({ field }) => (
            <FormItem>
              <FormLabel>
                Description <span className="text-muted-foreground">(optional)</span>
              </FormLabel>
              <FormControl>
                <Textarea {...field} rows={4} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button
          type="submit"
          className="h-12"
          disabled={form.formState.isSubmitting}
        >
          {form.formState.isSubmitting ? "Creating..." : "Create Project"}
        </Button>
      </form>
    </Form>
  )
}
```

### Progressive Enhancement Pattern

For forms that must work without JavaScript (login, registration), use server actions directly with `useActionState`:

```tsx
"use client"

import { useActionState } from "react"
import { login } from "@/server/actions/auth"
import { SubmitButton } from "@/components/ui/submit-button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

export function LoginForm() {
  const [state, action] = useActionState(login, { success: false, error: "" })

  return (
    <form action={action} noValidate>
      <div className="space-y-4">
        <div>
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            name="email"
            type="email"
            autoComplete="email"
            required
          />
        </div>

        <div>
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            name="password"
            type="password"
            autoComplete="current-password"
            required
          />
        </div>

        {state.error && (
          <p className="text-sm text-destructive" role="alert" aria-live="polite">
            {state.error}
          </p>
        )}

        <SubmitButton className="w-full h-12">Log In</SubmitButton>
      </div>
    </form>
  )
}
```

This form works without JavaScript because `<form action={action}>` triggers the server action as a standard form submission. When JavaScript loads, React enhances it with client-side state management.

---

## 15. T3 Stack Variant

The T3 Stack (create-t3-app) is an opinionated subset of this stack: Next.js + tRPC + Prisma + NextAuth + Tailwind. Use T3 when building a full-stack app where the frontend and backend are tightly coupled.

### tRPC Setup

```ts
// src/server/trpc/init.ts
import { initTRPC, TRPCError } from "@trpc/server"
import superjson from "superjson"
import { ZodError } from "zod"
import { auth } from "@/lib/auth"
import { db } from "@/lib/db"

const createTRPCContext = async () => {
  const session = await auth()
  return { db, session }
}

const t = initTRPC.context<typeof createTRPCContext>().create({
  transformer: superjson,
  errorFormatter({ shape, error }) {
    return {
      ...shape,
      data: {
        ...shape.data,
        zodError:
          error.cause instanceof ZodError ? error.cause.flatten() : null,
      },
    }
  },
})

export const createCallerFactory = t.createCallerFactory
export const createTRPCRouter = t.router

// Middleware
const enforceAuth = t.middleware(({ ctx, next }) => {
  if (!ctx.session?.user) {
    throw new TRPCError({ code: "UNAUTHORIZED" })
  }
  return next({
    ctx: { session: { ...ctx.session, user: ctx.session.user } },
  })
})

export const publicProcedure = t.procedure
export const protectedProcedure = t.procedure.use(enforceAuth)
```

### tRPC Router

```ts
// src/server/trpc/routers/project.ts
import { z } from "zod"
import { createTRPCRouter, protectedProcedure } from "../init"
import { projectSchema } from "@/lib/validations/project"

export const projectRouter = createTRPCRouter({
  list: protectedProcedure.query(async ({ ctx }) => {
    return ctx.db.project.findMany({
      where: { ownerId: ctx.session.user.id },
      orderBy: { updatedAt: "desc" },
    })
  }),

  byId: protectedProcedure
    .input(z.object({ id: z.string() }))
    .query(async ({ ctx, input }) => {
      const project = await ctx.db.project.findUnique({
        where: { id: input.id, ownerId: ctx.session.user.id },
      })
      if (!project) throw new TRPCError({ code: "NOT_FOUND" })
      return project
    }),

  create: protectedProcedure
    .input(projectSchema)
    .mutation(async ({ ctx, input }) => {
      return ctx.db.project.create({
        data: { ...input, ownerId: ctx.session.user.id },
      })
    }),

  update: protectedProcedure
    .input(z.object({ id: z.string() }).merge(projectSchema.partial()))
    .mutation(async ({ ctx, input }) => {
      const { id, ...data } = input
      return ctx.db.project.update({
        where: { id, ownerId: ctx.session.user.id },
        data,
      })
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.string() }))
    .mutation(async ({ ctx, input }) => {
      return ctx.db.project.delete({
        where: { id: input.id, ownerId: ctx.session.user.id },
      })
    }),
})
```

### Root Router

```ts
// src/server/trpc/routers/_app.ts
import { createTRPCRouter } from "../init"
import { projectRouter } from "./project"

export const appRouter = createTRPCRouter({
  project: projectRouter,
})

export type AppRouter = typeof appRouter
```

### tRPC Client Setup

```ts
// src/lib/trpc/client.ts
"use client"

import { createTRPCClient, httpBatchLink } from "@trpc/client"
import superjson from "superjson"
import type { AppRouter } from "@/server/trpc/routers/_app"

export const trpc = createTRPCClient<AppRouter>({
  links: [
    httpBatchLink({
      url: "/api/trpc",
      transformer: superjson,
    }),
  ],
})
```

### tRPC with React Query

```tsx
// src/lib/trpc/react.tsx
"use client"

import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { createTRPCReact, httpBatchLink } from "@trpc/react-query"
import superjson from "superjson"
import { useState } from "react"
import type { AppRouter } from "@/server/trpc/routers/_app"

export const api = createTRPCReact<AppRouter>()

export function TRPCProvider({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => new QueryClient())
  const [trpcClient] = useState(() =>
    api.createClient({
      links: [
        httpBatchLink({
          url: "/api/trpc",
          transformer: superjson,
        }),
      ],
    }),
  )

  return (
    <api.Provider client={trpcClient} queryClient={queryClient}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </api.Provider>
  )
}
```

### tRPC Usage in Components

```tsx
"use client"

import { api } from "@/lib/trpc/react"

export function ProjectList() {
  const { data: projects, isLoading } = api.project.list.useQuery()

  if (isLoading) return <Skeleton />

  return (
    <div>
      {projects?.map((project) => (
        <ProjectCard key={project.id} project={project} />
      ))}
    </div>
  )
}
```

### When to Use tRPC vs Server Actions

| Use Case | tRPC | Server Actions |
|---|---|---|
| Full-stack type safety | Yes — end-to-end types | Yes — TypeScript infers return types |
| Real-time data (polling) | Yes — with React Query | No — need manual polling |
| Batch requests | Yes — httpBatchLink | No — each action is individual |
| Progressive enhancement | No — requires JavaScript | Yes — works without JS |
| Form mutations | Overkill | Preferred |
| Complex queries with pagination | Preferred | Possible but verbose |
| Public API consumed by others | Yes | No — internal only |

**Decision rule:** Use tRPC when building an SPA-like experience with heavy client-side interaction. Use Server Actions when building a document-centric app with progressive enhancement.

---

## 16. Anti-Patterns (Next.js-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Adding `"use client"` to every component | Default to Server Components. Only add `"use client"` for interactivity. |
| `useEffect` for data fetching | Fetch in Server Components or use TanStack Query in client components. |
| Missing error boundaries | Every route segment needs `error.tsx`. Every Suspense boundary needs error handling. |
| N+1 queries in React Server Components | Use `include` / `with` in your ORM queries. Each RSC renders independently — avoid sequential awaits. |
| Barrel files (`index.ts` re-exports) | Import directly from the source file. Barrel files break tree-shaking and slow builds. |
| Putting secrets in `NEXT_PUBLIC_` env vars | Only public, non-sensitive values get the `NEXT_PUBLIC_` prefix. |
| Raw `<img>` tags | Always use `next/image` for automatic optimization, lazy loading, and responsive sizing. |
| Fetching data in layout.tsx then passing via props | Fetch in the page or component that needs it. Next.js deduplicates fetch calls automatically. |
| `router.push` for form submissions | Use server actions with `redirect()`. Client-side navigation after mutation misses revalidation. |
| Giant `page.tsx` files with all logic | Extract data fetching to `src/server/queries/`, business logic to `src/server/services/`, UI to components. |
| Storing server state in Zustand/Redux | Use Server Components for server data. Only use client state stores for UI state (sidebar, theme). |
| `any` types | Use proper TypeScript types. Infer from Zod schemas with `z.infer<typeof schema>`. Infer from Prisma with generated types. |
| Forms without `noValidate` attribute | Always add `noValidate` — HTML5 native validation is unreliable across assistive technologies. |
| Forms without `autoComplete` attributes | Always add `autoComplete="email"`, `autoComplete="name"`, etc. |
| "Submit" button text | Use outcome-focused CTA: "Create Project", "Log In", "Save Changes". |
| Deploying without test gate | CI must run lint + typecheck + test + build before deploy. |
| Missing security headers | Every deployment needs HSTS, CSP, X-Content-Type-Options, X-Frame-Options at minimum. |
| `suppressHydrationWarning` everywhere | Fix the hydration mismatch. Only use `suppressHydrationWarning` for known-dynamic content (timestamps, theme). |
| Wrapping entire app in `"use client"` provider at root | Push client boundaries down. Use the `children` prop pattern to keep the provider client but children server. |
| Using `fetch` in server actions | Use your ORM directly. Server actions run on the server — no need for HTTP round-trips to your own API. |
| Creating API routes for mutations | Use server actions. API routes are for webhooks, cron jobs, and third-party integrations. |
| Dynamic `import()` for everything | Only dynamic-import heavy libraries that are not needed on initial render. Small components should be statically imported. |
| Ignoring the `key` prop in lists | Always use stable, unique keys. Never use array index as key for dynamic lists. |
| `localStorage` for auth tokens | Use HTTP-only cookies via Auth.js. Client-accessible tokens are vulnerable to XSS. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Next.js patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:nextjs&title=[Next.js]%20)**

Use the `patterns:nextjs` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
