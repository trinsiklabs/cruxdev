# Development Patterns — GOTH Stack

Go / Gin (Echo/Fiber) / Templ / HTMX

This document captures stack-specific patterns, conventions, and decisions for GOTH stack projects (Go + Gin/Echo/Fiber web frameworks + Templ typed templates + HTMX hypermedia). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Go services, write Templ components, integrate HTMX, test with testify/httptest, deploy with Docker, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Go | 1.24+ | Minimum 1.23; prefer latest stable (1.24 as of Feb 2025) |
| Gin | v1.10+ | Most popular Go web framework (~78k GitHub stars) |
| Echo | v4.12+ | Alternative — excellent error handling, returns `error` from handlers |
| Fiber | v2.52+ | Alternative — built on fasthttp, Express.js-like API |
| Templ | v0.3+ | Typed HTML templating — compiles to Go code |
| HTMX | 2.0+ | Hypermedia-driven interactivity (~14KB min+gzip) |
| sqlc | v1.28+ | SQL-to-Go code generator — type-safe queries |
| GORM | v2.0+ | ORM alternative when sqlc is not appropriate |
| pgx | v5.7+ | PostgreSQL driver (used by sqlc and standalone) |
| golang-migrate | v4.18+ | Database migration tool |
| testify | v1.10+ | Assertion + mock + suite packages |
| Air | v1.61+ | Live reload for development |
| Tailwind CSS | 4.x | Utility-first CSS framework |
| Alpine.js | 3.x | Optional — lightweight JS for client-side state HTMX does not cover |
| Docker | 27+ | Multi-stage builds for production |
| PostgreSQL | 16+ | Primary database |

### Version Constraint Policy

Use Go modules with explicit version tags in `go.mod`:

```go
// Good — pinned to minor version, go get updates patch
require (
    github.com/gin-gonic/gin v1.10.0
    github.com/a-h/templ v0.3.819
    github.com/stretchr/testify v1.10.0
    github.com/jackc/pgx/v5 v5.7.2
)

// Bad — using pseudo-versions when a tag exists
require (
    github.com/gin-gonic/gin v0.0.0-20240813123456-abcdef123456
)

// Bad — importing without version suffix for v2+ modules
require (
    github.com/jackc/pgx v5.7.2  // missing /v5 path suffix
)
```

Run `go mod tidy` after every dependency change. Commit both `go.mod` and `go.sum`.

### Go 1.23+ Features to Use

| Feature | Use Case |
|---|---|
| Range-over-func iterators | Custom collection iteration (e.g., paginated DB results) |
| `unique` package | Interning/canonicalizing comparable values (cache keys, slugs) |
| `iter` package | Standard iterator patterns for sequences |
| Generic type aliases (1.24) | Cleaner type definitions in generic code |
| Swiss Table maps (1.24) | Automatic — 2-3% CPU improvement, no code changes |
| `testing/synctest` (1.24) | Testing concurrent code with fake clocks |
| `os.Root` (1.24) | Sandboxed filesystem operations |
| Tool directives in go.mod (1.24) | Track `sqlc`, `templ`, `air` as tool dependencies |

---

## 2. Project Structure

### Standard Layout

```
project/
├── cmd/
│   └── server/
│       └── main.go              # Entry point — wires dependencies, starts server
├── internal/
│   ├── config/
│   │   └── config.go            # Environment-based configuration
│   ├── database/
│   │   ├── database.go          # Connection pool setup
│   │   ├── migrations/          # SQL migration files (golang-migrate)
│   │   │   ├── 000001_init.up.sql
│   │   │   └── 000001_init.down.sql
│   │   └── queries/             # sqlc SQL files
│   │       ├── users.sql
│   │       ├── posts.sql
│   │       └── sqlc.yaml        # sqlc configuration
│   ├── domain/                  # Domain models (plain structs, no framework deps)
│   │   ├── user.go
│   │   ├── post.go
│   │   └── errors.go            # Domain-specific error types
│   ├── handler/                 # HTTP handlers (thin — delegate to service)
│   │   ├── handler.go           # Handler struct with dependencies
│   │   ├── user_handler.go
│   │   ├── post_handler.go
│   │   └── middleware.go        # Custom middleware
│   ├── service/                 # Business logic layer
│   │   ├── user_service.go
│   │   ├── user_service_test.go
│   │   └── post_service.go
│   ├── repository/              # Data access interfaces + implementations
│   │   ├── interfaces.go        # Repository interfaces
│   │   ├── user_repo.go         # sqlc-backed implementation
│   │   └── user_repo_test.go
│   └── sqlcgen/                 # sqlc generated code (DO NOT EDIT)
│       ├── db.go
│       ├── models.go
│       ├── querier.go
│       └── users.sql.go
├── templates/                   # Templ files (.templ)
│   ├── layouts/
│   │   ├── base.templ           # HTML shell — <html>, <head>, <body>
│   │   └── app.templ            # Authenticated layout with nav
│   ├── components/
│   │   ├── button.templ
│   │   ├── form.templ
│   │   ├── alert.templ
│   │   └── pagination.templ
│   ├── pages/
│   │   ├── home.templ
│   │   ├── login.templ
│   │   └── dashboard.templ
│   └── partials/                # HTMX partial responses
│       ├── user_row.templ
│       ├── user_list.templ
│       └── toast.templ
├── static/
│   ├── css/
│   │   └── app.css              # Tailwind input file
│   ├── js/
│   │   └── app.js               # HTMX + Alpine.js imports
│   └── dist/                    # Built assets (gitignored in dev, copied in Docker)
├── go.mod
├── go.sum
├── Makefile                     # Build, test, generate commands
├── Dockerfile                   # Multi-stage production build
├── docker-compose.yml           # Local dev (app + postgres + mailpit)
├── .air.toml                    # Live reload configuration
├── sqlc.yaml                    # Root sqlc config (if not in internal/database/)
└── tailwind.config.js           # Tailwind configuration (v3) or CSS-based (v4)
```

**Conventions:**
- `cmd/` — entry points only. No business logic. Wire dependencies and start the server.
- `internal/` — all application code. The Go compiler enforces that nothing outside the module imports from `internal/`.
- `templates/` — all Templ files live here, organized by purpose (layouts, components, pages, partials).
- `static/` — static assets. `dist/` is the build output directory.
- Domain models in `internal/domain/` have zero framework dependencies — pure Go structs and interfaces.
- Handlers are thin. They parse HTTP requests, call services, and render responses. No business logic in handlers.
- Services contain business logic. They depend on repository interfaces, not implementations.
- Repositories implement data access behind interfaces for testability.

### Test Structure

Tests live alongside the code they test (Go convention), with integration tests in a separate directory:

```
internal/
├── handler/
│   ├── user_handler.go
│   └── user_handler_test.go      # Handler tests (httptest)
├── service/
│   ├── user_service.go
│   └── user_service_test.go      # Service tests (mock repos)
├── repository/
│   ├── user_repo.go
│   └── user_repo_test.go         # Repo tests (test DB or sqlc mock)
test/
├── integration/
│   ├── api_test.go               # Full API integration tests
│   └── testutil.go               # Shared test helpers
├── e2e/
│   └── ... (Playwright, deferred)
└── testdata/
    ├── fixtures/                  # JSON/SQL test fixtures
    └── golden/                    # Golden file snapshots
```

---

## 3. Framework Selection: Gin vs Echo vs Fiber

### Decision Matrix

| Criteria | Gin | Echo | Fiber |
|---|---|---|---|
| **GitHub Stars** | ~78k | ~30k | ~34k |
| **HTTP Engine** | `net/http` | `net/http` | `fasthttp` |
| **Handler Signature** | `func(c *gin.Context)` | `func(c echo.Context) error` | `func(c *fiber.Ctx) error` |
| **Error Handling** | Manual (call `c.JSON` then return) | Return `error` — centralized handler | Return `error` — centralized handler |
| **Middleware Ecosystem** | Largest | Large | Growing |
| **stdlib Compatibility** | Full `net/http` | Full `net/http` | Limited — `fasthttp` diverges |
| **Performance (req/sec)** | ~50-70k | ~55-75k | ~80-120k |
| **Learning Curve** | Low (1-2 days) | Medium (2-3 days) | Low if from Express.js |
| **Best For** | APIs, team familiarity | Enterprise APIs, clean error flow | High-throughput, real-time |

### Recommendation

**Default choice: Gin** — largest ecosystem, most Go developers already know it, full `net/http` compatibility.

**Choose Echo when:** you need centralized error handling (handlers return `error`), built-in request validation, or prefer its middleware chaining model.

**Choose Fiber when:** you need maximum throughput, WebSocket-heavy workloads, or your team comes from Express.js. Accept the trade-off: `fasthttp` is not fully compatible with `net/http` middleware and some standard library packages.

### Gin Handler Pattern

```go
// Handler struct — holds all dependencies
type Handler struct {
    userService  service.UserService
    postService  service.PostService
    logger       *slog.Logger
}

func NewHandler(us service.UserService, ps service.PostService, logger *slog.Logger) *Handler {
    return &Handler{
        userService: us,
        postService: ps,
        logger:      logger,
    }
}

// Route registration — clean separation from handler logic
func (h *Handler) RegisterRoutes(r *gin.Engine) {
    api := r.Group("/api/v1")
    {
        api.GET("/users", h.ListUsers)
        api.POST("/users", h.CreateUser)
        api.GET("/users/:id", h.GetUser)
        api.PUT("/users/:id", h.UpdateUser)
        api.DELETE("/users/:id", h.DeleteUser)
    }

    // HTMX page routes
    pages := r.Group("/")
    pages.Use(h.AuthMiddleware())
    {
        pages.GET("/dashboard", h.DashboardPage)
        pages.GET("/users", h.UsersPage)
    }
}
```

### Echo Handler Pattern

```go
// Echo handlers return error — framework handles error responses
func (h *Handler) GetUser(c echo.Context) error {
    id, err := uuid.Parse(c.Param("id"))
    if err != nil {
        return echo.NewHTTPError(http.StatusBadRequest, "invalid user ID")
    }

    user, err := h.userService.GetByID(c.Request().Context(), id)
    if err != nil {
        if errors.Is(err, domain.ErrNotFound) {
            return echo.NewHTTPError(http.StatusNotFound, "user not found")
        }
        return fmt.Errorf("get user: %w", err)
    }

    // Render Templ component for HTMX requests, JSON for API
    if c.Request().Header.Get("HX-Request") == "true" {
        return Render(c, http.StatusOK, templates.UserDetail(user))
    }
    return c.JSON(http.StatusOK, user)
}
```

### Fiber Handler Pattern

```go
// Fiber uses fasthttp context — different from net/http
func (h *Handler) GetUser(c *fiber.Ctx) error {
    id, err := uuid.Parse(c.Params("id"))
    if err != nil {
        return fiber.NewError(fiber.StatusBadRequest, "invalid user ID")
    }

    user, err := h.userService.GetByID(c.Context(), id)
    if err != nil {
        if errors.Is(err, domain.ErrNotFound) {
            return fiber.NewError(fiber.StatusNotFound, "user not found")
        }
        return fmt.Errorf("get user: %w", err)
    }

    return c.JSON(user)
}
```

---

## 4. Templ Typed Templates

### Why Templ Over html/template

| Aspect | html/template | Templ |
|---|---|---|
| **Type safety** | None — runtime panics on missing data | Full — compile-time errors |
| **IDE support** | Minimal | LSP, autocomplete, go-to-definition |
| **Composition** | Nested `template` calls, string-based | Go function calls, type-checked |
| **Performance** | Reflection-heavy rendering | Compiled to `io.Writer` calls — zero reflection |
| **Refactoring** | Find-and-replace strings | Standard Go refactoring tools |
| **Error messages** | Runtime: "can't evaluate field X" | Compile-time: "undefined: X" |

### Templ File Structure

Templ files use the `.templ` extension and compile to `_templ.go` files:

```
templates/
├── layouts/
│   ├── base.templ           → base_templ.go
│   └── app.templ            → app_templ.go
├── components/
│   ├── button.templ         → button_templ.go
│   └── form.templ           → form_templ.go
├── pages/
│   ├── home.templ           → home_templ.go
│   └── dashboard.templ      → dashboard_templ.go
└── partials/
    ├── user_row.templ       → user_row_templ.go
    └── toast.templ          → toast_templ.go
```

**Convention:** Never edit `_templ.go` files. They are generated. Add them to `.gitignore` or commit them — either approach works, but be consistent. Committing them avoids requiring `templ generate` in CI.

### Base Layout

```templ
// templates/layouts/base.templ
package layouts

templ Base(title string) {
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <title>{ title }</title>
        <link rel="stylesheet" href="/static/dist/app.css"/>
        <script src="/static/js/htmx.min.js"></script>
        <script src="/static/js/alpine.min.js" defer></script>
    </head>
    <body class="min-h-screen bg-gray-50" hx-boost="true">
        { children... }
        <!-- Toast container for HTMX OOB swaps -->
        <div id="toast-container" class="fixed top-4 right-4 z-50 space-y-2"></div>
    </body>
    </html>
}
```

### App Layout (Authenticated)

```templ
// templates/layouts/app.templ
package layouts

import "myapp/internal/domain"

templ App(title string, user *domain.User) {
    @Base(title) {
        <nav class="bg-white shadow-sm border-b">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16 items-center">
                    <a href="/" class="text-xl font-bold">MyApp</a>
                    <div class="flex items-center gap-4">
                        <span class="text-sm text-gray-600">{ user.Email }</span>
                        <button
                            hx-post="/auth/logout"
                            hx-redirect="/"
                            class="text-sm text-red-600 hover:text-red-800"
                        >
                            Log Out
                        </button>
                    </div>
                </div>
            </div>
        </nav>
        <main class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
            { children... }
        </main>
    }
}
```

### Page Component

```templ
// templates/pages/dashboard.templ
package pages

import (
    "myapp/internal/domain"
    "myapp/templates/layouts"
    "myapp/templates/components"
)

templ Dashboard(user *domain.User, stats domain.DashboardStats) {
    @layouts.App("Dashboard", user) {
        <h1 class="text-2xl font-bold mb-6">Dashboard</h1>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            @components.StatCard("Users", stats.UserCount)
            @components.StatCard("Posts", stats.PostCount)
            @components.StatCard("Comments", stats.CommentCount)
        </div>
        <div class="mt-8">
            <h2 class="text-lg font-semibold mb-4">Recent Activity</h2>
            <div
                hx-get="/partials/activity-feed"
                hx-trigger="load"
                hx-swap="innerHTML"
            >
                @components.Spinner()
            </div>
        </div>
    }
}
```

### Reusable Component

```templ
// templates/components/button.templ
package components

type ButtonVariant string

const (
    ButtonPrimary   ButtonVariant = "primary"
    ButtonSecondary ButtonVariant = "secondary"
    ButtonDanger    ButtonVariant = "danger"
)

func buttonClasses(variant ButtonVariant) string {
    base := "inline-flex items-center justify-center px-4 py-2 rounded-lg font-medium text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2"
    switch variant {
    case ButtonPrimary:
        return base + " bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500"
    case ButtonSecondary:
        return base + " bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-blue-500"
    case ButtonDanger:
        return base + " bg-red-600 text-white hover:bg-red-700 focus:ring-red-500"
    default:
        return base + " bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500"
    }
}

templ Button(text string, variant ButtonVariant) {
    <button type="button" class={ buttonClasses(variant) }>
        { text }
    </button>
}

templ SubmitButton(text string, loadingText string) {
    <button
        type="submit"
        class="inline-flex items-center justify-center px-4 py-2 rounded-lg font-medium text-sm bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
    >
        <span class="htmx-indicator hidden">
            @Spinner()
            { loadingText }
        </span>
        <span class="htmx-indicator-hide">
            { text }
        </span>
    </button>
}
```

### HTMX Partial (Fragment Response)

```templ
// templates/partials/user_row.templ
package partials

import (
    "myapp/internal/domain"
    "fmt"
)

templ UserRow(user domain.User) {
    <tr id={ fmt.Sprintf("user-%s", user.ID) }>
        <td class="px-6 py-4 whitespace-nowrap text-sm">{ user.Name }</td>
        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{ user.Email }</td>
        <td class="px-6 py-4 whitespace-nowrap text-sm">
            <button
                hx-delete={ fmt.Sprintf("/users/%s", user.ID) }
                hx-target={ fmt.Sprintf("#user-%s", user.ID) }
                hx-swap="outerHTML swap:0.3s"
                hx-confirm="Delete this user?"
                class="text-red-600 hover:text-red-800"
            >
                Delete
            </button>
        </td>
    </tr>
}
```

### Rendering Templ in Handlers

```go
// Gin — render Templ component to response
func Render(c *gin.Context, status int, component templ.Component) {
    c.Status(status)
    c.Header("Content-Type", "text/html; charset=utf-8")
    if err := component.Render(c.Request.Context(), c.Writer); err != nil {
        c.String(http.StatusInternalServerError, "render error")
    }
}

// Usage in handler
func (h *Handler) DashboardPage(c *gin.Context) {
    user := getUserFromContext(c)
    stats, err := h.dashService.GetStats(c.Request.Context())
    if err != nil {
        h.logger.Error("failed to get stats", "error", err)
        Render(c, http.StatusInternalServerError, pages.ErrorPage("Something went wrong"))
        return
    }
    Render(c, http.StatusOK, pages.Dashboard(user, stats))
}

// Echo — render Templ component
func Render(c echo.Context, status int, component templ.Component) error {
    c.Response().Header().Set("Content-Type", "text/html; charset=utf-8")
    c.Response().WriteHeader(status)
    return component.Render(c.Request().Context(), c.Response().Writer)
}
```

### Templ Code Generation

```bash
# Generate Go code from .templ files
templ generate

# Watch mode for development (use with Air)
templ generate --watch

# Format templ files
templ fmt .

# Validate templ files without generating
templ generate --fail-on-error
```

Add `templ generate` to your build pipeline. If committing generated files, run it pre-commit.

---

## 5. HTMX Integration Patterns

### Core Philosophy

HTMX extends HTML with attributes that let any element make HTTP requests and swap content. The server returns HTML fragments, not JSON. This is the fundamental shift: **the server is the source of truth for both data and presentation**.

### Setup

Include HTMX in the base layout. Use the CDN for prototyping, vendor the file for production:

```html
<!-- Vendored (preferred for production) -->
<script src="/static/js/htmx.min.js"></script>

<!-- CDN (prototyping only) -->
<script src="https://unpkg.com/htmx.org@2.0.4"></script>
```

### HTMX Attribute Reference (Most Used)

| Attribute | Purpose | Example |
|---|---|---|
| `hx-get` | GET request | `hx-get="/users"` |
| `hx-post` | POST request | `hx-post="/users"` |
| `hx-put` | PUT request | `hx-put="/users/123"` |
| `hx-delete` | DELETE request | `hx-delete="/users/123"` |
| `hx-target` | Where to swap response | `hx-target="#user-list"` |
| `hx-swap` | How to swap | `hx-swap="innerHTML"`, `outerHTML`, `beforeend` |
| `hx-trigger` | When to fire | `hx-trigger="click"`, `load`, `revealed` |
| `hx-indicator` | Loading indicator | `hx-indicator="#spinner"` |
| `hx-confirm` | Confirmation dialog | `hx-confirm="Are you sure?"` |
| `hx-boost` | Progressive enhancement | `hx-boost="true"` on `<body>` |
| `hx-push-url` | Update browser URL | `hx-push-url="true"` |
| `hx-select` | Pick fragment from response | `hx-select="#content"` |
| `hx-vals` | Include extra values | `hx-vals='{"key":"val"}'` |
| `hx-headers` | Custom headers | `hx-headers='{"X-Custom":"val"}'` |
| `hx-include` | Include inputs in request | `hx-include="[name='search']"` |

### Pattern: Inline Editing

```templ
// Display mode
templ UserNameDisplay(user domain.User) {
    <span
        hx-get={ fmt.Sprintf("/users/%s/edit-name", user.ID) }
        hx-swap="outerHTML"
        class="cursor-pointer hover:bg-yellow-50 px-1 rounded"
    >
        { user.Name }
    </span>
}

// Edit mode (returned by /users/:id/edit-name)
templ UserNameEdit(user domain.User) {
    <form
        hx-put={ fmt.Sprintf("/users/%s/name", user.ID) }
        hx-swap="outerHTML"
        class="inline-flex items-center gap-2"
    >
        <input
            type="text"
            name="name"
            value={ user.Name }
            class="border rounded px-2 py-1 text-sm"
            autofocus
        />
        <button type="submit" class="text-green-600 text-sm">Save</button>
        <button
            type="button"
            hx-get={ fmt.Sprintf("/users/%s/display-name", user.ID) }
            hx-swap="outerHTML"
            hx-target="closest form"
            class="text-gray-400 text-sm"
        >
            Cancel
        </button>
    </form>
}
```

### Pattern: Infinite Scroll

```templ
templ UserList(users []domain.User, nextCursor string) {
    for _, user := range users {
        @UserRow(user)
    }
    if nextCursor != "" {
        <tr
            hx-get={ fmt.Sprintf("/partials/users?cursor=%s", nextCursor) }
            hx-trigger="revealed"
            hx-swap="afterend"
        >
            <td colspan="3" class="text-center py-4">
                <span class="htmx-indicator">Loading more...</span>
            </td>
        </tr>
    }
}
```

### Pattern: Live Search with Debounce

```templ
templ SearchBar() {
    <input
        type="search"
        name="q"
        placeholder="Search users..."
        hx-get="/partials/search-results"
        hx-trigger="input changed delay:300ms, search"
        hx-target="#search-results"
        hx-indicator="#search-spinner"
        class="w-full border rounded-lg px-4 py-2"
    />
    <span id="search-spinner" class="htmx-indicator">Searching...</span>
    <div id="search-results"></div>
}
```

### Pattern: Out-of-Band (OOB) Swaps

OOB swaps update multiple parts of the page from a single response:

```go
// Handler returns main content + OOB toast notification
func (h *Handler) DeleteUser(c *gin.Context) {
    id := c.Param("id")
    if err := h.userService.Delete(c.Request.Context(), id); err != nil {
        Render(c, http.StatusInternalServerError, partials.Toast("error", "Failed to delete user"))
        return
    }

    // Return empty (removes the row via hx-target) + OOB toast
    c.Status(http.StatusOK)
    c.Header("Content-Type", "text/html; charset=utf-8")
    // The target element is removed by outerHTML swap with empty response
    // Plus an OOB swap for the toast
    partials.Toast("success", "User deleted").Render(c.Request.Context(), c.Writer)
}
```

```templ
// templates/partials/toast.templ
package partials

templ Toast(level string, message string) {
    <div
        id="toast-container"
        hx-swap-oob="afterbegin"
    >
        <div
            class={ "p-4 rounded-lg shadow-lg text-white text-sm",
                templ.KV("bg-green-600", level == "success"),
                templ.KV("bg-red-600", level == "error"),
                templ.KV("bg-yellow-500", level == "warning") }
            x-data="{ show: true }"
            x-show="show"
            x-init="setTimeout(() => show = false, 3000)"
        >
            { message }
        </div>
    </div>
}
```

### Pattern: Server-Sent Events (SSE) for Real-Time

```templ
templ NotificationBell() {
    <div
        hx-ext="sse"
        sse-connect="/events/notifications"
        sse-swap="notification"
    >
        <span id="notification-count" class="badge">0</span>
    </div>
}
```

```go
// SSE endpoint
func (h *Handler) NotificationSSE(c *gin.Context) {
    c.Header("Content-Type", "text/event-stream")
    c.Header("Cache-Control", "no-cache")
    c.Header("Connection", "keep-alive")

    userID := getUserIDFromContext(c)
    ch := h.notifService.Subscribe(userID)
    defer h.notifService.Unsubscribe(userID, ch)

    c.Stream(func(w io.Writer) bool {
        select {
        case notif := <-ch:
            var buf bytes.Buffer
            partials.NotificationBadge(notif.Count).Render(c.Request.Context(), &buf)
            c.SSEvent("notification", buf.String())
            return true
        case <-c.Request.Context().Done():
            return false
        }
    })
}
```

### HTMX Response Headers

Use response headers to control HTMX behavior from the server:

```go
// Redirect after form submission (HTMX-aware)
func htmxRedirect(c *gin.Context, url string) {
    if c.GetHeader("HX-Request") == "true" {
        c.Header("HX-Redirect", url)
        c.Status(http.StatusNoContent)
    } else {
        c.Redirect(http.StatusSeeOther, url)
    }
}

// Trigger client-side event
c.Header("HX-Trigger", `{"showToast": "User created successfully"}`)

// Refresh the page
c.Header("HX-Refresh", "true")

// Retarget the swap
c.Header("HX-Retarget", "#different-element")

// Push URL to browser history
c.Header("HX-Push-Url", "/users/123")
```

### HTMX Middleware: Detect HTMX Requests

```go
// Middleware to detect HTMX requests and set context
func HTMXMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        c.Set("isHTMX", c.GetHeader("HX-Request") == "true")
        c.Set("htmxTarget", c.GetHeader("HX-Target"))
        c.Set("htmxTrigger", c.GetHeader("HX-Trigger"))
        c.Set("htmxCurrentURL", c.GetHeader("HX-Current-URL"))
        c.Next()
    }
}

// Helper
func isHTMX(c *gin.Context) bool {
    val, exists := c.Get("isHTMX")
    return exists && val.(bool)
}
```

---

## 6. Data Access: sqlc vs GORM

### Decision Matrix

| Criteria | sqlc | GORM |
|---|---|---|
| **Approach** | SQL-first — write SQL, generate Go | Code-first — write Go, generate SQL |
| **Type Safety** | Compile-time (generated code) | Runtime (reflection) |
| **Performance** | Minimal overhead — raw queries | ORM overhead — reflection, hooks |
| **Learning Curve** | Know SQL, learn sqlc config | Learn GORM API, understand magic |
| **Complex Queries** | Natural — it's just SQL | Painful — drop to raw SQL for complex joins |
| **Migrations** | Separate tool (golang-migrate) | Built-in AutoMigrate (not production-safe) |
| **Best For** | Read-heavy, complex queries, performance | Rapid prototyping, simple CRUD |

### Recommendation

**Default choice: sqlc** — aligns with Go's explicit philosophy. You write SQL, you get type-safe Go code. No magic.

**Choose GORM when:** rapid prototyping, very simple CRUD with no complex queries, or team is more comfortable with ORM patterns.

### sqlc Setup

```yaml
# sqlc.yaml
version: "2"
sql:
  - engine: "postgresql"
    queries: "internal/database/queries/"
    schema: "internal/database/migrations/"
    gen:
      go:
        package: "sqlcgen"
        out: "internal/sqlcgen"
        sql_package: "pgx/v5"
        emit_json_tags: true
        emit_empty_slices: true
        emit_pointers_for_null_types: true
        overrides:
          - db_type: "uuid"
            go_type: "github.com/google/uuid.UUID"
          - db_type: "timestamptz"
            go_type: "time.Time"
```

### sqlc Query File

```sql
-- internal/database/queries/users.sql

-- name: GetUserByID :one
SELECT id, email, name, role, created_at, updated_at
FROM users
WHERE id = $1;

-- name: ListUsers :many
SELECT id, email, name, role, created_at, updated_at
FROM users
ORDER BY created_at DESC
LIMIT $1 OFFSET $2;

-- name: CreateUser :one
INSERT INTO users (id, email, name, password_hash, role)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, email, name, role, created_at, updated_at;

-- name: UpdateUser :one
UPDATE users
SET name = $2, email = $3, updated_at = NOW()
WHERE id = $1
RETURNING id, email, name, role, created_at, updated_at;

-- name: DeleteUser :exec
DELETE FROM users WHERE id = $1;

-- name: SearchUsers :many
SELECT id, email, name, role, created_at, updated_at
FROM users
WHERE name ILIKE '%' || $1 || '%' OR email ILIKE '%' || $1 || '%'
ORDER BY name
LIMIT $2 OFFSET $3;

-- name: CountUsers :one
SELECT COUNT(*) FROM users;
```

### Repository Layer Over sqlc

Wrap sqlc-generated code in a repository interface for testability:

```go
// internal/repository/interfaces.go
package repository

import (
    "context"
    "myapp/internal/domain"
    "github.com/google/uuid"
)

type UserRepository interface {
    GetByID(ctx context.Context, id uuid.UUID) (*domain.User, error)
    List(ctx context.Context, limit, offset int32) ([]domain.User, error)
    Create(ctx context.Context, user *domain.User) error
    Update(ctx context.Context, user *domain.User) error
    Delete(ctx context.Context, id uuid.UUID) error
    Search(ctx context.Context, query string, limit, offset int32) ([]domain.User, error)
    Count(ctx context.Context) (int64, error)
}
```

```go
// internal/repository/user_repo.go
package repository

import (
    "context"
    "errors"
    "fmt"
    "myapp/internal/domain"
    "myapp/internal/sqlcgen"
    "github.com/google/uuid"
    "github.com/jackc/pgx/v5"
)

type userRepo struct {
    queries *sqlcgen.Queries
}

func NewUserRepository(db sqlcgen.DBTX) UserRepository {
    return &userRepo{queries: sqlcgen.New(db)}
}

func (r *userRepo) GetByID(ctx context.Context, id uuid.UUID) (*domain.User, error) {
    row, err := r.queries.GetUserByID(ctx, id)
    if err != nil {
        if errors.Is(err, pgx.ErrNoRows) {
            return nil, domain.ErrNotFound
        }
        return nil, fmt.Errorf("get user by id: %w", err)
    }
    return mapRowToUser(row), nil
}

func (r *userRepo) Create(ctx context.Context, user *domain.User) error {
    row, err := r.queries.CreateUser(ctx, sqlcgen.CreateUserParams{
        ID:           user.ID,
        Email:        user.Email,
        Name:         user.Name,
        PasswordHash: user.PasswordHash,
        Role:         string(user.Role),
    })
    if err != nil {
        return fmt.Errorf("create user: %w", err)
    }
    user.CreatedAt = row.CreatedAt
    user.UpdatedAt = row.UpdatedAt
    return nil
}

func mapRowToUser(row sqlcgen.GetUserByIDRow) *domain.User {
    return &domain.User{
        ID:        row.ID,
        Email:     row.Email,
        Name:      row.Name,
        Role:      domain.Role(row.Role),
        CreatedAt: row.CreatedAt,
        UpdatedAt: row.UpdatedAt,
    }
}
```

### GORM Setup (When Used)

```go
// internal/database/database.go
package database

import (
    "fmt"
    "myapp/internal/config"
    "gorm.io/driver/postgres"
    "gorm.io/gorm"
    "gorm.io/gorm/logger"
)

func NewGormDB(cfg config.Database) (*gorm.DB, error) {
    dsn := fmt.Sprintf(
        "host=%s port=%d user=%s password=%s dbname=%s sslmode=%s",
        cfg.Host, cfg.Port, cfg.User, cfg.Password, cfg.Name, cfg.SSLMode,
    )

    db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{
        Logger:                 logger.Default.LogMode(logger.Warn),
        SkipDefaultTransaction: true, // Performance: skip wrapping each query in a tx
        PrepareStmt:            true, // Performance: prepare statements once
    })
    if err != nil {
        return nil, fmt.Errorf("open database: %w", err)
    }

    sqlDB, _ := db.DB()
    sqlDB.SetMaxOpenConns(cfg.MaxOpenConns)
    sqlDB.SetMaxIdleConns(cfg.MaxIdleConns)
    sqlDB.SetConnMaxLifetime(cfg.ConnMaxLifetime)

    return db, nil
}
```

### Migrations with golang-migrate

```bash
# Install
go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest

# Create a new migration
migrate create -ext sql -dir internal/database/migrations -seq create_users

# Run migrations
migrate -database "postgres://user:pass@localhost:5432/mydb?sslmode=disable" \
    -path internal/database/migrations up

# Rollback last migration
migrate -database "..." -path internal/database/migrations down 1
```

```sql
-- internal/database/migrations/000001_create_users.up.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- internal/database/migrations/000001_create_users.down.sql
DROP TABLE IF EXISTS users;
```

**Convention:** Never edit a migration after it's been applied to any environment. Write a new corrective migration instead.

---

## 7. Authentication & Authorization

### Session-Based Auth with Cookies

For HTMX-driven apps, session-based auth with cookies is the natural fit. HTMX requests include cookies automatically — no token management needed on the client.

```go
// internal/handler/auth_handler.go
package handler

import (
    "net/http"
    "myapp/internal/service"
    "myapp/templates/pages"
    "github.com/gin-gonic/gin"
    "github.com/gorilla/sessions"
)

type AuthHandler struct {
    authService service.AuthService
    store       sessions.Store
}

func NewAuthHandler(as service.AuthService, sessionSecret string) *AuthHandler {
    store := sessions.NewCookieStore([]byte(sessionSecret))
    store.Options = &sessions.Options{
        Path:     "/",
        MaxAge:   86400 * 7, // 7 days
        HttpOnly: true,
        Secure:   true,
        SameSite: http.SameSiteLaxMode,
    }
    return &AuthHandler{authService: as, store: store}
}

func (h *AuthHandler) Login(c *gin.Context) {
    email := c.PostForm("email")
    password := c.PostForm("password")

    user, err := h.authService.Authenticate(c.Request.Context(), email, password)
    if err != nil {
        Render(c, http.StatusUnauthorized, pages.LoginPage("Invalid email or password"))
        return
    }

    session, _ := h.store.Get(c.Request, "session")
    session.Values["user_id"] = user.ID.String()
    session.Values["role"] = string(user.Role)
    if err := session.Save(c.Request, c.Writer); err != nil {
        Render(c, http.StatusInternalServerError, pages.ErrorPage("Session error"))
        return
    }

    htmxRedirect(c, "/dashboard")
}

func (h *AuthHandler) Logout(c *gin.Context) {
    session, _ := h.store.Get(c.Request, "session")
    session.Options.MaxAge = -1
    session.Save(c.Request, c.Writer)
    htmxRedirect(c, "/")
}
```

### Auth Middleware

```go
func (h *AuthHandler) RequireAuth() gin.HandlerFunc {
    return func(c *gin.Context) {
        session, _ := h.store.Get(c.Request, "session")
        userID, ok := session.Values["user_id"].(string)
        if !ok || userID == "" {
            if isHTMX(c) {
                c.Header("HX-Redirect", "/login")
                c.AbortWithStatus(http.StatusUnauthorized)
            } else {
                c.Redirect(http.StatusSeeOther, "/login")
                c.Abort()
            }
            return
        }
        c.Set("user_id", userID)
        c.Set("role", session.Values["role"])
        c.Next()
    }
}

func (h *AuthHandler) RequireRole(roles ...string) gin.HandlerFunc {
    return func(c *gin.Context) {
        role, _ := c.Get("role")
        roleStr, _ := role.(string)
        for _, r := range roles {
            if roleStr == r {
                c.Next()
                return
            }
        }
        if isHTMX(c) {
            Render(c, http.StatusForbidden, partials.Toast("error", "Insufficient permissions"))
        } else {
            Render(c, http.StatusForbidden, pages.ErrorPage("Forbidden"))
        }
        c.Abort()
    }
}
```

### JWT for API Endpoints

When the project also exposes a JSON API (mobile clients, third-party integrations), use JWT alongside session auth:

```go
// internal/handler/middleware.go

func JWTAuth(secret string) gin.HandlerFunc {
    return func(c *gin.Context) {
        tokenStr := extractBearerToken(c)
        if tokenStr == "" {
            c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "missing token"})
            return
        }

        claims, err := validateJWT(tokenStr, secret)
        if err != nil {
            c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "invalid token"})
            return
        }

        c.Set("user_id", claims.UserID)
        c.Set("role", claims.Role)
        c.Next()
    }
}

func extractBearerToken(c *gin.Context) string {
    auth := c.GetHeader("Authorization")
    if len(auth) > 7 && auth[:7] == "Bearer " {
        return auth[7:]
    }
    return ""
}
```

### Password Hashing

Always use bcrypt or argon2id. Never store plaintext passwords.

```go
import "golang.org/x/crypto/bcrypt"

func HashPassword(password string) (string, error) {
    hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
    return string(hash), err
}

func CheckPassword(hash, password string) error {
    return bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
}
```

---

## 8. Testing Patterns

### Test Pyramid (GOTH-specific)

```
        ╱╲
       ╱  ╲          E2E (Playwright) — deferred to later slices
      ╱    ╲
     ╱──────╲
    ╱        ╲        Integration Tests (httptest + real DB)
   ╱          ╲       Full HTTP request → handler → service → DB → response
  ╱────────────╲
 ╱              ╲      Service Tests (testify + mock repos)
╱                ╲     Business logic with mocked dependencies
╱──────────────────╲
╱                    ╲   Unit Tests (testify)
╱                      ╲  Pure functions, validators, mappers
╱────────────────────────╲
```

### Unit Tests with Testify

```go
// internal/service/user_service_test.go
package service_test

import (
    "context"
    "testing"
    "myapp/internal/domain"
    "myapp/internal/service"
    "myapp/internal/repository/mocks"
    "github.com/google/uuid"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/mock"
    "github.com/stretchr/testify/require"
)

func TestUserService_GetByID(t *testing.T) {
    t.Run("returns user when found", func(t *testing.T) {
        repo := mocks.NewMockUserRepository(t)
        svc := service.NewUserService(repo)
        ctx := context.Background()

        expected := &domain.User{
            ID:    uuid.New(),
            Email: "jane@example.com",
            Name:  "Jane Smith",
        }

        repo.EXPECT().GetByID(ctx, expected.ID).Return(expected, nil)

        user, err := svc.GetByID(ctx, expected.ID)
        require.NoError(t, err)
        assert.Equal(t, expected.Email, user.Email)
        assert.Equal(t, expected.Name, user.Name)
    })

    t.Run("returns error when not found", func(t *testing.T) {
        repo := mocks.NewMockUserRepository(t)
        svc := service.NewUserService(repo)
        ctx := context.Background()
        id := uuid.New()

        repo.EXPECT().GetByID(ctx, id).Return(nil, domain.ErrNotFound)

        user, err := svc.GetByID(ctx, id)
        assert.Nil(t, user)
        assert.ErrorIs(t, err, domain.ErrNotFound)
    })
}
```

### Table-Driven Tests

```go
func TestValidateEmail(t *testing.T) {
    tests := []struct {
        name    string
        email   string
        wantErr bool
    }{
        {"valid email", "user@example.com", false},
        {"valid with plus", "user+tag@example.com", false},
        {"empty string", "", true},
        {"no at sign", "userexample.com", true},
        {"no domain", "user@", true},
        {"no local part", "@example.com", true},
        {"double at", "user@@example.com", true},
        {"spaces", "user @example.com", true},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := domain.ValidateEmail(tt.email)
            if tt.wantErr {
                assert.Error(t, err)
            } else {
                assert.NoError(t, err)
            }
        })
    }
}
```

### Handler Tests with httptest

```go
// internal/handler/user_handler_test.go
package handler_test

import (
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "strings"
    "testing"
    "myapp/internal/handler"
    "myapp/internal/service/mocks"
    "myapp/internal/domain"
    "github.com/gin-gonic/gin"
    "github.com/google/uuid"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

func setupTestRouter(h *handler.Handler) *gin.Engine {
    gin.SetMode(gin.TestMode)
    r := gin.New()
    h.RegisterRoutes(r)
    return r
}

func TestHandler_GetUser(t *testing.T) {
    t.Run("returns 200 with user JSON", func(t *testing.T) {
        userSvc := mocks.NewMockUserService(t)
        h := handler.NewHandler(userSvc, nil, nil)
        router := setupTestRouter(h)

        userID := uuid.New()
        expected := &domain.User{
            ID:    userID,
            Email: "jane@example.com",
            Name:  "Jane Smith",
        }
        userSvc.EXPECT().
            GetByID(mock.Anything, userID).
            Return(expected, nil)

        req := httptest.NewRequest(http.MethodGet, "/api/v1/users/"+userID.String(), nil)
        w := httptest.NewRecorder()
        router.ServeHTTP(w, req)

        assert.Equal(t, http.StatusOK, w.Code)

        var resp domain.User
        err := json.Unmarshal(w.Body.Bytes(), &resp)
        require.NoError(t, err)
        assert.Equal(t, expected.Email, resp.Email)
    })

    t.Run("returns 404 when user not found", func(t *testing.T) {
        userSvc := mocks.NewMockUserService(t)
        h := handler.NewHandler(userSvc, nil, nil)
        router := setupTestRouter(h)

        userID := uuid.New()
        userSvc.EXPECT().
            GetByID(mock.Anything, userID).
            Return(nil, domain.ErrNotFound)

        req := httptest.NewRequest(http.MethodGet, "/api/v1/users/"+userID.String(), nil)
        w := httptest.NewRecorder()
        router.ServeHTTP(w, req)

        assert.Equal(t, http.StatusNotFound, w.Code)
    })
}

func TestHandler_CreateUser_HTMX(t *testing.T) {
    t.Run("returns HTML partial for HTMX request", func(t *testing.T) {
        userSvc := mocks.NewMockUserService(t)
        h := handler.NewHandler(userSvc, nil, nil)
        router := setupTestRouter(h)

        userSvc.EXPECT().
            Create(mock.Anything, mock.AnythingOfType("*domain.CreateUserInput")).
            Return(&domain.User{ID: uuid.New(), Name: "Jane", Email: "jane@example.com"}, nil)

        body := strings.NewReader("name=Jane&email=jane@example.com&password=secret123")
        req := httptest.NewRequest(http.MethodPost, "/users", body)
        req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
        req.Header.Set("HX-Request", "true")
        w := httptest.NewRecorder()
        router.ServeHTTP(w, req)

        assert.Equal(t, http.StatusOK, w.Code)
        assert.Contains(t, w.Header().Get("Content-Type"), "text/html")
        assert.Contains(t, w.Body.String(), "Jane")
    })
}
```

### Templ Component Tests

Test Templ components by rendering them and checking the output:

```go
// templates/components/button_test.go
package components_test

import (
    "bytes"
    "context"
    "testing"
    "myapp/templates/components"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

func TestButton(t *testing.T) {
    t.Run("renders primary button", func(t *testing.T) {
        var buf bytes.Buffer
        err := components.Button("Click Me", components.ButtonPrimary).Render(context.Background(), &buf)
        require.NoError(t, err)

        html := buf.String()
        assert.Contains(t, html, "Click Me")
        assert.Contains(t, html, "bg-blue-600")
        assert.Contains(t, html, `type="button"`)
    })

    t.Run("renders danger button", func(t *testing.T) {
        var buf bytes.Buffer
        err := components.Button("Delete", components.ButtonDanger).Render(context.Background(), &buf)
        require.NoError(t, err)

        html := buf.String()
        assert.Contains(t, html, "Delete")
        assert.Contains(t, html, "bg-red-600")
    })
}
```

### Integration Tests with Real Database

```go
// test/integration/api_test.go
package integration_test

import (
    "context"
    "net/http"
    "net/http/httptest"
    "testing"
    "myapp/internal/config"
    "myapp/internal/database"
    "myapp/internal/handler"
    "myapp/internal/repository"
    "myapp/internal/service"
    "github.com/gin-gonic/gin"
    "github.com/stretchr/testify/suite"
)

type APITestSuite struct {
    suite.Suite
    server *httptest.Server
    db     *database.DB
}

func (s *APITestSuite) SetupSuite() {
    cfg := config.LoadTest()
    db, err := database.New(cfg.Database)
    s.Require().NoError(err)
    s.db = db

    // Run migrations
    err = database.Migrate(db, cfg.Database.MigrationsPath)
    s.Require().NoError(err)

    // Wire dependencies
    userRepo := repository.NewUserRepository(db.Pool)
    userSvc := service.NewUserService(userRepo)
    h := handler.NewHandler(userSvc, nil, nil)

    gin.SetMode(gin.TestMode)
    router := gin.New()
    h.RegisterRoutes(router)
    s.server = httptest.NewServer(router)
}

func (s *APITestSuite) TearDownSuite() {
    s.server.Close()
    s.db.Close()
}

func (s *APITestSuite) SetupTest() {
    // Truncate tables between tests
    _, err := s.db.Pool.Exec(context.Background(), "TRUNCATE users CASCADE")
    s.Require().NoError(err)
}

func (s *APITestSuite) TestCreateAndGetUser() {
    // Create user
    resp, err := http.Post(
        s.server.URL+"/api/v1/users",
        "application/json",
        strings.NewReader(`{"name":"Jane","email":"jane@example.com","password":"secret123"}`),
    )
    s.Require().NoError(err)
    s.Equal(http.StatusCreated, resp.StatusCode)

    var created domain.User
    json.NewDecoder(resp.Body).Decode(&created)
    resp.Body.Close()

    // Get user
    resp, err = http.Get(s.server.URL + "/api/v1/users/" + created.ID.String())
    s.Require().NoError(err)
    s.Equal(http.StatusOK, resp.StatusCode)

    var fetched domain.User
    json.NewDecoder(resp.Body).Decode(&fetched)
    resp.Body.Close()

    s.Equal("Jane", fetched.Name)
    s.Equal("jane@example.com", fetched.Email)
}

func TestAPISuite(t *testing.T) {
    if testing.Short() {
        t.Skip("skipping integration tests in short mode")
    }
    suite.Run(t, new(APITestSuite))
}
```

### Mock Generation with mockery

Use mockery to generate testify mocks from interfaces:

```bash
# Install mockery
go install github.com/vektra/mockery/v2@latest

# Generate mocks for all interfaces in repository package
mockery --dir=internal/repository --name=UserRepository --output=internal/repository/mocks

# Or configure in .mockery.yaml for all interfaces
```

```yaml
# .mockery.yaml
with-expecter: true
packages:
  myapp/internal/repository:
    interfaces:
      UserRepository:
      PostRepository:
  myapp/internal/service:
    interfaces:
      UserService:
      PostService:
```

### Test Configuration

```go
// test/integration/testutil.go
package integration_test

import (
    "os"
    "myapp/internal/config"
)

func init() {
    // Use test database
    os.Setenv("DATABASE_URL", "postgres://test:test@localhost:5432/myapp_test?sslmode=disable")
    os.Setenv("APP_ENV", "test")
}
```

### Coverage Commands

```bash
# Run all tests with coverage
go test ./... -coverprofile=coverage.out -covermode=atomic

# View coverage report
go tool cover -func=coverage.out

# HTML coverage report
go tool cover -html=coverage.out -o coverage.html

# Run tests excluding integration
go test -short ./...

# Run only integration tests
go test -run TestAPISuite ./test/integration/
```

---

## 9. Configuration

### Environment-Based Configuration

```go
// internal/config/config.go
package config

import (
    "fmt"
    "log/slog"
    "os"
    "strconv"
    "time"
)

type Config struct {
    App      App
    Database Database
    Session  Session
    Server   Server
}

type App struct {
    Env     string // "development", "test", "production"
    Debug   bool
    BaseURL string
}

type Database struct {
    URL             string
    MaxOpenConns    int
    MaxIdleConns    int
    ConnMaxLifetime time.Duration
    MigrationsPath  string
}

type Session struct {
    Secret   string
    MaxAge   int
    Secure   bool
    HTTPOnly bool
}

type Server struct {
    Host         string
    Port         int
    ReadTimeout  time.Duration
    WriteTimeout time.Duration
    IdleTimeout  time.Duration
}

func Load() Config {
    return Config{
        App: App{
            Env:     getEnv("APP_ENV", "development"),
            Debug:   getEnvBool("APP_DEBUG", true),
            BaseURL: getEnv("APP_BASE_URL", "http://localhost:8080"),
        },
        Database: Database{
            URL:             mustGetEnv("DATABASE_URL"),
            MaxOpenConns:    getEnvInt("DB_MAX_OPEN_CONNS", 25),
            MaxIdleConns:    getEnvInt("DB_MAX_IDLE_CONNS", 5),
            ConnMaxLifetime: time.Duration(getEnvInt("DB_CONN_MAX_LIFETIME_MIN", 5)) * time.Minute,
            MigrationsPath:  getEnv("DB_MIGRATIONS_PATH", "internal/database/migrations"),
        },
        Session: Session{
            Secret:   mustGetEnv("SESSION_SECRET"),
            MaxAge:   getEnvInt("SESSION_MAX_AGE", 86400*7),
            Secure:   getEnvBool("SESSION_SECURE", true),
            HTTPOnly: true,
        },
        Server: Server{
            Host:         getEnv("SERVER_HOST", "0.0.0.0"),
            Port:         getEnvInt("SERVER_PORT", 8080),
            ReadTimeout:  time.Duration(getEnvInt("SERVER_READ_TIMEOUT_SEC", 10)) * time.Second,
            WriteTimeout: time.Duration(getEnvInt("SERVER_WRITE_TIMEOUT_SEC", 30)) * time.Second,
            IdleTimeout:  time.Duration(getEnvInt("SERVER_IDLE_TIMEOUT_SEC", 120)) * time.Second,
        },
    }
}

func mustGetEnv(key string) string {
    val := os.Getenv(key)
    if val == "" {
        slog.Error("required environment variable not set", "key", key)
        os.Exit(1)
    }
    return val
}

func getEnv(key, fallback string) string {
    if val := os.Getenv(key); val != "" {
        return val
    }
    return fallback
}

func getEnvInt(key string, fallback int) int {
    val := os.Getenv(key)
    if val == "" {
        return fallback
    }
    n, err := strconv.Atoi(val)
    if err != nil {
        return fallback
    }
    return n
}

func getEnvBool(key string, fallback bool) bool {
    val := os.Getenv(key)
    if val == "" {
        return fallback
    }
    b, err := strconv.ParseBool(val)
    if err != nil {
        return fallback
    }
    return b
}
```

### .env Files (Development Only)

Use `.env` files for local development. Never commit them. Load with `godotenv` or direnv:

```bash
# .env (gitignored)
APP_ENV=development
APP_DEBUG=true
DATABASE_URL=postgres://dev:dev@localhost:5432/myapp_dev?sslmode=disable
SESSION_SECRET=dev-secret-change-in-production
```

```go
// cmd/server/main.go — load .env in development only
import "github.com/joho/godotenv"

func main() {
    if os.Getenv("APP_ENV") != "production" {
        godotenv.Load() // Ignore error — file may not exist
    }
    cfg := config.Load()
    // ...
}
```

---

## 10. Structured Logging

### slog (Standard Library)

Go 1.21+ includes `log/slog` in the standard library. Use it everywhere.

```go
// cmd/server/main.go — configure logger
func setupLogger(env string) *slog.Logger {
    var handler slog.Handler
    if env == "production" {
        handler = slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
            Level: slog.LevelInfo,
        })
    } else {
        handler = slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{
            Level: slog.LevelDebug,
        })
    }
    return slog.New(handler)
}

// Usage throughout the application
func (s *UserService) GetByID(ctx context.Context, id uuid.UUID) (*domain.User, error) {
    user, err := s.repo.GetByID(ctx, id)
    if err != nil {
        s.logger.Error("failed to get user",
            "user_id", id,
            "error", err,
        )
        return nil, fmt.Errorf("get user %s: %w", id, err)
    }
    s.logger.Debug("user retrieved", "user_id", id, "email", user.Email)
    return user, nil
}
```

### Request Logging Middleware

```go
func RequestLogger(logger *slog.Logger) gin.HandlerFunc {
    return func(c *gin.Context) {
        start := time.Now()
        path := c.Request.URL.Path
        query := c.Request.URL.RawQuery

        c.Next()

        latency := time.Since(start)
        status := c.Writer.Status()

        attrs := []any{
            "method", c.Request.Method,
            "path", path,
            "status", status,
            "latency_ms", latency.Milliseconds(),
            "ip", c.ClientIP(),
            "user_agent", c.Request.UserAgent(),
        }
        if query != "" {
            attrs = append(attrs, "query", query)
        }
        if c.GetHeader("HX-Request") == "true" {
            attrs = append(attrs, "htmx", true)
            attrs = append(attrs, "hx_target", c.GetHeader("HX-Target"))
        }

        if status >= 500 {
            logger.Error("request completed", attrs...)
        } else if status >= 400 {
            logger.Warn("request completed", attrs...)
        } else {
            logger.Info("request completed", attrs...)
        }
    }
}
```

---

## 11. Development Workflow

### Feature Development Cycle (GOTH-specific)

```
1. Write acceptance criteria (docs/scenarios/*.md or GitHub issue)
2. Design test levels (unit / handler / integration)
3. Write failing tests (testify + httptest)
4. Write domain models and interfaces
5. Implement repository (sqlc queries → generate → implement)
6. Implement service layer
7. Write handler (thin — delegates to service)
8. Write Templ templates (layouts, pages, partials)
9. Wire HTMX attributes for interactivity
10. Run: go test ./...
11. Run: golangci-lint run
12. Refactor while green
```

### Common Commands

```bash
# Development
make dev                               # Start Air live-reload server
make generate                          # Run all code generation (sqlc + templ)
make test                              # Run all tests
make test-short                        # Skip integration tests
make lint                              # Run golangci-lint
make fmt                               # Format code (gofmt + templ fmt)

# Code generation
templ generate                         # Generate Go from .templ files
sqlc generate                          # Generate Go from SQL queries
go generate ./...                      # Run all go:generate directives

# Database
make migrate-up                        # Run all pending migrations
make migrate-down                      # Rollback last migration
make migrate-create name=add_posts     # Create new migration pair

# Build
make build                             # Build production binary
make docker-build                      # Build Docker image

# Quality
go vet ./...                           # Built-in static analysis
golangci-lint run                      # Comprehensive linting
go test -race ./...                    # Race condition detection
```

### Makefile

```makefile
.PHONY: dev build test lint fmt generate migrate-up migrate-down docker-build

# Development
dev:
	air

build:
	CGO_ENABLED=0 go build -ldflags="-s -w" -o bin/server ./cmd/server

# Code generation
generate: generate-sqlc generate-templ

generate-sqlc:
	sqlc generate

generate-templ:
	templ generate

# Testing
test:
	go test -v -coverprofile=coverage.out -covermode=atomic ./...

test-short:
	go test -short -v ./...

test-race:
	go test -race ./...

# Quality
lint:
	golangci-lint run

fmt:
	gofmt -w .
	templ fmt .

vet:
	go vet ./...

# Database
migrate-up:
	migrate -database "$(DATABASE_URL)" -path internal/database/migrations up

migrate-down:
	migrate -database "$(DATABASE_URL)" -path internal/database/migrations down 1

migrate-create:
	migrate create -ext sql -dir internal/database/migrations -seq $(name)

# Docker
docker-build:
	docker build -t myapp:latest .

docker-run:
	docker-compose up -d

# All checks (CI equivalent)
ci: lint vet test
```

### Air Configuration (Live Reload)

```toml
# .air.toml
root = "."
tmp_dir = "tmp"

[build]
  cmd = "templ generate && go build -o ./tmp/main ./cmd/server"
  bin = "tmp/main"
  include_ext = ["go", "templ", "css", "js"]
  exclude_dir = ["tmp", "node_modules", "static/dist", "vendor"]
  delay = 1000

[screen]
  clear_on_rebuild = true

[log]
  time = true
```

### golangci-lint Configuration

```yaml
# .golangci.yml
run:
  timeout: 5m

linters:
  enable:
    - errcheck
    - govet
    - staticcheck
    - unused
    - gosimple
    - ineffassign
    - typecheck
    - bodyclose
    - contextcheck
    - errname
    - errorlint
    - exhaustive
    - gocritic
    - godot
    - gofmt
    - misspell
    - nilerr
    - noctx
    - prealloc
    - predeclared
    - revive
    - rowserrcheck
    - sqlclosecheck
    - unconvert
    - unparam
    - wastedassign

linters-settings:
  errcheck:
    check-type-assertions: true
    check-blank: true
  govet:
    enable-all: true
  revive:
    rules:
      - name: var-naming
        disabled: false

issues:
  exclude-dirs:
    - internal/sqlcgen  # Generated code
```

---

## 12. Deployment

### Multi-Stage Docker Build

```dockerfile
# Dockerfile
# Stage 1: Build Templ + Go binary
FROM golang:1.24-alpine AS builder

RUN apk add --no-cache git ca-certificates

WORKDIR /app

# Install templ CLI
RUN go install github.com/a-h/templ/cmd/templ@latest

# Cache dependencies
COPY go.mod go.sum ./
RUN go mod download

# Copy source
COPY . .

# Generate templ code
RUN templ generate

# Build binary
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build -ldflags="-s -w" -o /app/bin/server ./cmd/server

# Stage 2: Build CSS (Tailwind)
FROM node:22-alpine AS css-builder

WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci --production=false
COPY tailwind.config.js ./
COPY static/css/ static/css/
COPY templates/ templates/
RUN npx tailwindcss -i static/css/app.css -o static/dist/app.css --minify

# Stage 3: Minimal runtime
FROM alpine:3.21

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

# Copy binary
COPY --from=builder /app/bin/server /app/server

# Copy static assets
COPY --from=css-builder /app/static/dist/ /app/static/dist/
COPY static/js/ /app/static/js/

# Copy migrations (for runtime migration)
COPY internal/database/migrations/ /app/migrations/

# Non-root user
RUN adduser -D -u 1000 appuser
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -qO- http://localhost:8080/health || exit 1

ENTRYPOINT ["/app/server"]
```

### Docker Compose (Local Development)

```yaml
# docker-compose.yml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      - APP_ENV=development
      - DATABASE_URL=postgres://dev:dev@postgres:5432/myapp_dev?sslmode=disable
      - SESSION_SECRET=dev-secret-not-for-production
    depends_on:
      postgres:
        condition: service_healthy

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: dev
      POSTGRES_DB: myapp_dev
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U dev -d myapp_dev"]
      interval: 5s
      timeout: 5s
      retries: 5

  mailpit:
    image: axllent/mailpit
    ports:
      - "1025:1025"  # SMTP
      - "8025:8025"  # Web UI

volumes:
  pgdata:
```

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
        image: postgres:16-alpine
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: myapp_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: '1.24'

      - name: Install tools
        run: |
          go install github.com/a-h/templ/cmd/templ@latest
          go install github.com/sqlc-dev/sqlc/cmd/sqlc@latest
          go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest

      - name: Generate code
        run: |
          templ generate
          sqlc generate

      - name: Verify generated code is up to date
        run: git diff --exit-code

      - name: Lint
        uses: golangci/golangci-lint-action@v6
        with:
          version: latest

      - name: Run migrations
        env:
          DATABASE_URL: postgres://test:test@localhost:5432/myapp_test?sslmode=disable
        run: migrate -database "$DATABASE_URL" -path internal/database/migrations up

      - name: Test
        env:
          DATABASE_URL: postgres://test:test@localhost:5432/myapp_test?sslmode=disable
          APP_ENV: test
          SESSION_SECRET: test-secret
        run: go test -v -race -coverprofile=coverage.out -covermode=atomic ./...

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          file: coverage.out

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build and push Docker image
        run: |
          docker build -t myapp:${{ github.sha }} .
          # Push to container registry (ACR, ECR, GCR, GHCR, etc.)

      # Deploy to your target (Fly.io, Railway, K8s, etc.)
```

### Health Check Endpoint

```go
// internal/handler/health.go
func (h *Handler) HealthCheck(c *gin.Context) {
    ctx, cancel := context.WithTimeout(c.Request.Context(), 3*time.Second)
    defer cancel()

    if err := h.db.Ping(ctx); err != nil {
        c.JSON(http.StatusServiceUnavailable, gin.H{
            "status":   "unhealthy",
            "database": "unreachable",
        })
        return
    }

    c.JSON(http.StatusOK, gin.H{
        "status":   "healthy",
        "database": "connected",
    })
}
```

---

## 13. Security Headers

Every response includes security headers via middleware:

```go
// internal/handler/middleware.go

func SecurityHeaders() gin.HandlerFunc {
    return func(c *gin.Context) {
        c.Header("Strict-Transport-Security", "max-age=31536000; includeSubDomains")
        c.Header("X-Content-Type-Options", "nosniff")
        c.Header("X-Frame-Options", "SAMEORIGIN")
        c.Header("Referrer-Policy", "strict-origin-when-cross-origin")
        c.Header("Permissions-Policy", "camera=(), microphone=(), geolocation=()")
        c.Header("Content-Security-Policy",
            "default-src 'self'; "+
                "script-src 'self' 'unsafe-inline'; "+  // unsafe-inline needed for HTMX inline handlers and Alpine.js
                "style-src 'self' 'unsafe-inline'; "+    // unsafe-inline needed for Tailwind
                "img-src 'self' data:; "+
                "font-src 'self'; "+
                "connect-src 'self'; "+
                "frame-ancestors 'self'",
        )
        c.Next()
    }
}

// CSRF protection for non-GET requests
func CSRFProtection(secret string) gin.HandlerFunc {
    return func(c *gin.Context) {
        if c.Request.Method == http.MethodGet ||
            c.Request.Method == http.MethodHead ||
            c.Request.Method == http.MethodOptions {
            c.Next()
            return
        }

        // HTMX sends the token via hx-headers or a meta tag
        token := c.GetHeader("X-CSRF-Token")
        if token == "" {
            token = c.PostForm("csrf_token")
        }

        if !validateCSRFToken(token, secret) {
            c.AbortWithStatus(http.StatusForbidden)
            return
        }
        c.Next()
    }
}
```

### CSRF Token in Templ

```templ
// Include CSRF token in all forms via a meta tag (HTMX reads it)
templ Base(title string, csrfToken string) {
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <meta name="csrf-token" content={ csrfToken }/>
        <title>{ title }</title>
        <script src="/static/js/htmx.min.js"></script>
        <script>
            // Automatically include CSRF token in HTMX requests
            document.body.addEventListener('htmx:configRequest', function(evt) {
                evt.detail.headers['X-CSRF-Token'] =
                    document.querySelector('meta[name="csrf-token"]').content;
            });
        </script>
    </head>
    <body hx-boost="true">
        { children... }
    </body>
    </html>
}

// Hidden CSRF field for standard forms
templ CSRFField(token string) {
    <input type="hidden" name="csrf_token" value={ token }/>
}
```

### Rate Limiting

```go
import "golang.org/x/time/rate"

func RateLimiter(rps int, burst int) gin.HandlerFunc {
    limiters := sync.Map{}

    return func(c *gin.Context) {
        ip := c.ClientIP()
        limiterI, _ := limiters.LoadOrStore(ip, rate.NewLimiter(rate.Limit(rps), burst))
        limiter := limiterI.(*rate.Limiter)

        if !limiter.Allow() {
            if isHTMX(c) {
                Render(c, http.StatusTooManyRequests, partials.Toast("error", "Too many requests. Please slow down."))
            } else {
                c.AbortWithStatusJSON(http.StatusTooManyRequests, gin.H{
                    "error": "rate limit exceeded",
                })
            }
            return
        }
        c.Next()
    }
}
```

---

## 14. Coverage Enforcement

Test coverage is enforced in CI with threshold gates:

```bash
# Run tests with coverage
go test ./... -coverprofile=coverage.out -covermode=atomic

# Check coverage meets threshold
COVERAGE=$(go tool cover -func=coverage.out | grep total | awk '{print $3}' | sed 's/%//')
THRESHOLD=80

if (( $(echo "$COVERAGE < $THRESHOLD" | bc -l) )); then
    echo "FAIL: Coverage ${COVERAGE}% is below threshold ${THRESHOLD}%"
    exit 1
fi
echo "PASS: Coverage ${COVERAGE}% meets threshold ${THRESHOLD}%"
```

### Coverage in Makefile

```makefile
COVERAGE_THRESHOLD=80

coverage:
	go test ./... -coverprofile=coverage.out -covermode=atomic
	go tool cover -func=coverage.out
	@COVERAGE=$$(go tool cover -func=coverage.out | grep total | awk '{print $$3}' | sed 's/%//'); \
	if [ $$(echo "$$COVERAGE < $(COVERAGE_THRESHOLD)" | bc -l) -eq 1 ]; then \
		echo "FAIL: Coverage $$COVERAGE% < $(COVERAGE_THRESHOLD)%"; exit 1; \
	fi
	@echo "Coverage gate passed"

coverage-html:
	go test ./... -coverprofile=coverage.out -covermode=atomic
	go tool cover -html=coverage.out -o coverage.html
```

### Exclude Generated Code

Generated code (sqlc output, templ output) should be excluded from coverage analysis. Use build tags or coverage filters:

```bash
# Exclude generated directories from coverage
go test $(go list ./... | grep -v /sqlcgen/ | grep -v _templ) -coverprofile=coverage.out
```

Target is 100% (per CLAUDE.md core rules). The threshold in CI is the hard gate — CI fails below it.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with fieldset + legend |
| **labels** | Visible `<label>`, associated via `for` attribute, optional fields marked "(optional)" |
| **validation** | Server-side always; client-side optional via HTMX `hx-post` for real-time feedback |
| **errors** | Inline next to field + summary, multi-cue (icon + text + border), HTMX partial swap |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-describedby` for errors |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Create Account" not "Submit"), loading state via htmx-indicator |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | Debounced validation via `hx-trigger="change delay:300ms"`, no unnecessary requests |

### GOTH-Specific Form Pattern

```templ
// templates/pages/register.templ
package pages

import (
    "myapp/templates/layouts"
    "myapp/templates/components"
)

type RegisterFormData struct {
    Name     string
    Email    string
    Errors   map[string]string
    CSRFToken string
}

templ RegisterPage(data RegisterFormData) {
    @layouts.Base("Create Account", data.CSRFToken) {
        <div class="max-w-md mx-auto mt-12">
            <h1 class="text-2xl font-bold mb-6">Create Your Account</h1>
            <form
                hx-post="/register"
                hx-target="#form-container"
                hx-swap="outerHTML"
                novalidate
                id="form-container"
            >
                @components.CSRFField(data.CSRFToken)
                <fieldset class="space-y-4">
                    <legend class="sr-only">Account Information</legend>

                    @components.FormField(components.FieldOpts{
                        Label:        "Full Name",
                        Name:         "name",
                        Type:         "text",
                        Value:        data.Name,
                        Required:     true,
                        Autocomplete: "name",
                        Error:        data.Errors["name"],
                    })

                    @components.FormField(components.FieldOpts{
                        Label:        "Email Address",
                        Name:         "email",
                        Type:         "email",
                        Value:        data.Email,
                        Required:     true,
                        Autocomplete: "email",
                        Error:        data.Errors["email"],
                    })

                    @components.FormField(components.FieldOpts{
                        Label:        "Password",
                        Name:         "password",
                        Type:         "password",
                        Required:     true,
                        Autocomplete: "new-password",
                        Error:        data.Errors["password"],
                    })
                </fieldset>

                <div class="mt-6">
                    @components.SubmitButton("Create Account", "Creating...")
                </div>
            </form>
        </div>
    }
}
```

### Reusable Form Field Component

```templ
// templates/components/form.templ
package components

import "fmt"

type FieldOpts struct {
    Label        string
    Name         string
    Type         string
    Value        string
    Placeholder  string
    Required     bool
    Autocomplete string
    Error        string
    HelpText     string
}

templ FormField(opts FieldOpts) {
    <div>
        <label for={ opts.Name } class="block text-sm font-medium text-gray-700 mb-1">
            { opts.Label }
            if !opts.Required {
                <span class="text-gray-400 font-normal">(optional)</span>
            }
        </label>
        <input
            type={ opts.Type }
            id={ opts.Name }
            name={ opts.Name }
            value={ opts.Value }
            if opts.Placeholder != "" {
                placeholder={ opts.Placeholder }
            }
            if opts.Required {
                required
            }
            if opts.Autocomplete != "" {
                autocomplete={ opts.Autocomplete }
            }
            if opts.Error != "" {
                aria-invalid="true"
                aria-describedby={ fmt.Sprintf("%s-error", opts.Name) }
            }
            class={ "block w-full rounded-lg border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-offset-1",
                templ.KV("border-gray-300 focus:ring-blue-500 focus:border-blue-500", opts.Error == ""),
                templ.KV("border-red-500 focus:ring-red-500 focus:border-red-500", opts.Error != "") }
        />
        if opts.Error != "" {
            <p id={ fmt.Sprintf("%s-error", opts.Name) } class="mt-1 text-sm text-red-600 flex items-center gap-1" role="alert">
                <svg class="h-4 w-4 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                </svg>
                { opts.Error }
            </p>
        }
        if opts.HelpText != "" && opts.Error == "" {
            <p class="mt-1 text-sm text-gray-500">{ opts.HelpText }</p>
        }
    </div>
}
```

### Server-Side Validation with HTMX Response

```go
func (h *Handler) Register(c *gin.Context) {
    input := domain.RegisterInput{
        Name:     strings.TrimSpace(c.PostForm("name")),
        Email:    strings.TrimSpace(c.PostForm("email")),
        Password: c.PostForm("password"),
    }

    errs := input.Validate()
    if len(errs) > 0 {
        data := pages.RegisterFormData{
            Name:      input.Name,
            Email:     input.Email,
            Errors:    errs,
            CSRFToken: getCSRFToken(c),
        }
        Render(c, http.StatusUnprocessableEntity, pages.RegisterPage(data))
        return
    }

    user, err := h.authService.Register(c.Request.Context(), input)
    if err != nil {
        if errors.Is(err, domain.ErrEmailTaken) {
            data := pages.RegisterFormData{
                Name:      input.Name,
                Email:     input.Email,
                Errors:    map[string]string{"email": "This email is already registered"},
                CSRFToken: getCSRFToken(c),
            }
            Render(c, http.StatusUnprocessableEntity, pages.RegisterPage(data))
            return
        }
        h.logger.Error("registration failed", "error", err)
        Render(c, http.StatusInternalServerError, pages.ErrorPage("Something went wrong"))
        return
    }

    // Set session and redirect
    h.setSession(c, user)
    htmxRedirect(c, "/dashboard")
}
```

---

## 16. Anti-Patterns (GOTH-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using `html/template` for new projects | Use Templ — type-safe, composable, IDE support |
| 2 | Putting business logic in handlers | Handlers are thin — delegate to service layer |
| 3 | Returning JSON from HTMX endpoints | Return HTML fragments — that is the HTMX contract |
| 4 | Using `interface{}` / `any` for template data | Use typed structs — Templ components take explicit parameters |
| 5 | Writing raw SQL strings in handler/service code | Use sqlc — SQL-to-Go code generation with type safety |
| 6 | Using GORM's `AutoMigrate` in production | Use golang-migrate with versioned SQL migration files |
| 7 | Global database connections | Inject `*pgx.Pool` or `*sql.DB` via constructor — testable, explicit |
| 8 | Testing with a real database in unit tests | Mock the repository interface with testify/mock or mockery |
| 9 | Using `panic` for error handling | Return `error` — use `errors.Is`/`errors.As` for matching |
| 10 | Ignoring `context.Context` cancellation | Always pass `ctx` through; check `ctx.Err()` in long operations |
| 11 | `log.Println` for logging | Use `log/slog` — structured, leveled, JSON in production |
| 12 | Storing secrets in code or config files | Use environment variables — never commit `.env` files |
| 13 | Forms without `novalidate` attribute | Always add `novalidate` — HTML5 native validation is unreliable across assistive technologies |
| 14 | Forms without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, etc. |
| 15 | "Submit" button text | Use outcome-focused CTA: "Create Account", "Log In", "Save Changes" |
| 16 | Missing CSRF protection on mutations | All POST/PUT/DELETE must validate a CSRF token |
| 17 | Using `gin.Default()` in production | Use `gin.New()` + explicit middleware — `Default()` includes debug logger and recovery that may leak info |
| 18 | Committing `go.sum` conflicts without `go mod tidy` | Always run `go mod tidy` before committing |
| 19 | Wrapping every error with `fmt.Errorf` | Only add context when it helps — avoid `get user: get user: get user:` chains |
| 20 | Using Fiber and expecting `net/http` middleware compatibility | Fiber uses `fasthttp` — most `net/http` middleware will not work without adaptation |
| 21 | Fat `main.go` with all wiring inline | Extract dependency wiring to a `newApp()` or `wire()` function — keep `main.go` under 30 lines |
| 22 | Skipping `defer rows.Close()` on database queries | Always close rows/statements — sqlc handles this, but manual queries need explicit cleanup |
| 23 | Using `hx-boost="true"` on forms with file uploads | HTMX boost uses AJAX which requires `hx-encoding="multipart/form-data"` for file uploads, or disable boost on that form |
| 24 | HTMX swap without specifying target | Always set explicit `hx-target` — relying on defaults causes unexpected DOM mutations |
| 25 | Deploying without health check endpoint | Every service needs `/health` — load balancers and orchestrators depend on it |
| 26 | Missing security headers | Every endpoint needs HSTS, CSP, X-Content-Type-Options, X-Frame-Options at minimum |
| 27 | `<label>` without `for` attribute | Always bind `<label for="id">` to `<input id="id">` — WCAG 1.3.1 requirement |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a GOTH patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:goth&title=[GOTH]%20)**

Use the `patterns:goth` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
