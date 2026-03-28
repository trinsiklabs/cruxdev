# Development Patterns — PADL Stack

Phoenix / Ash / DaisyUI / LiveView

This document captures stack-specific patterns, conventions, and decisions for projects using Phoenix + Ash Framework + DaisyUI + LiveView (the "PADL" stack). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, convergence. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_PETAL.md** — the Petal Components variant of this document. If your project uses Petal Components instead of DaisyUI, use that file.
- **This file** — stack-specific patterns for DaisyUI-based PADL projects. How we structure Ash resources, test with ExUnit, use DaisyUI components, handle forms, deploy to Fly.io, etc.
- **Build plan files** — per-task actionable plans with checkboxes. Created using the methodology from DEVELOPMENT_PATTERNS.md and the technical patterns from this file.

**Origin:** Derived from DEVELOPMENT_PATTERNS_PETAL.md with adaptations learned during the Redoubt Capital adoption (real estate investment platform, 1436 tests, 100% coverage, 31 E2E features, 10-role UAT testing).

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Elixir | 1.18+ | 1.19 preferred if Docker images available |
| Erlang/OTP | 27+ | |
| Phoenix | 1.8+ | LiveView bundled |
| Phoenix LiveView | ~> 1.0 | Bundled with Phoenix 1.8 |
| Ash Framework | ~> 3.15 | Domain framework — resources, actions, authorization |
| AshPostgres | ~> 2.6 | PostgreSQL data layer |
| AshPhoenix | ~> 2.1 | Phoenix/LiveView integration |
| AshAuthentication | ~> 4.0 | Password + OAuth authentication |
| AshStateMachine | ~> 0.2 | State machine extension |
| AshPaperTrail | ~> 0.4 | Audit trail extension |
| DaisyUI | 5.x | Tailwind CSS component library (via JS plugin) |
| PostgreSQL | 16+ | |
| Tailwind CSS | 4.x | Via Phoenix asset pipeline (CSS-based config) |
| Oban | ~> 2.19 | Background job processing |

### Version Constraint Policy

Use `~>` (pessimistic) constraints in `mix.exs` pinned to the minor version:

```elixir
# Good — allows patch updates, blocks minor/major
{:ash, "~> 3.19"},
{:ash_postgres, "~> 2.6"},
{:phoenix, "~> 1.8"},

# Bad — too loose, allows breaking minor updates
{:ash, "~> 3.0"},

# Bad — too tight, blocks patch fixes
{:ash, "3.19.3"},
```

Exception: for release candidates or packages with known instability, pin exact.

---

## 2. Project Structure

### Ash Domain Organization

Each Ash domain is a directory under `lib/<app>/` containing a domain module and its resources:

```
lib/<app>/
├── accounts/              # Auth domain
│   ├── accounts.ex        # Domain module (use Ash.Domain)
│   ├── user.ex            # Ash resource
│   ├── token.ex           # Ash resource (password reset)
│   └── auth_token.ex      # Ash resource (session tokens)
├── projects/              # Project management domain
│   ├── projects.ex
│   ├── project.ex
│   └── milestone.ex
├── finance/               # Financial domain
│   ├── finance.ex
│   ├── invoice.ex
│   └── revenue.ex
├── partners/              # Partner management domain
│   ├── partners.ex
│   ├── assignment.ex
│   └── scope_of_work.ex
└── system/                # Cross-cutting system services
    ├── system.ex
    ├── notification_preference.ex
    └── sms_log.ex
```

**Convention:** One Ash domain per bounded context. The domain module (`accounts.ex`) defines the domain and lists its resources. Resources are never accessed directly — always through Ash actions.

### Web Layer Organization

```
lib/<app>_web/
├── components/
│   ├── core_components.ex     # Phoenix-generated (modal, flash, table, form helpers)
│   ├── layouts.ex             # Layout function components
│   ├── <app>_components.ex    # Domain-specific components (nav, sidebar, footer, stat_card)
│   └── layouts/
│       ├── root.html.heex     # HTML shell (head, body, fonts, meta)
│       ├── portal.html.heex   # Authenticated investor layout
│       ├── admin.html.heex    # Admin panel layout
│       ├── partner.html.heex  # Partner portal layout
│       └── public.html.heex   # Public marketing layout
├── live/
│   ├── admin/                 # Admin LiveViews
│   ├── portal/                # Investor portal LiveViews
│   ├── partner/               # Partner portal LiveViews
│   ├── public/                # Public marketing LiveViews
│   ├── advantage/             # Information/marketing LiveViews
│   └── auth/                  # Auth hooks (on_mount)
├── controllers/               # Non-LiveView controllers (health, auth, webhooks)
└── plugs/                     # Security headers, rate limiting, session timeout
```

### Test Mirror Structure

Tests mirror the `lib/` structure:

```
test/
├── <app>/                 # Unit + integration tests (Ash resources, workers)
├── <app>_web/             # LiveView, controller, component, plug tests
│   ├── live/
│   ├── components/
│   └── plugs/
├── e2e/                   # Wallaby E2E browser tests
│   ├── public/
│   ├── portal/
│   ├── partner/
│   ├── admin/
│   └── auth/
└── support/
    ├── auth_helpers.ex    # Ash action-based user factories
    ├── conn_case.ex       # ConnCase (LiveView + controller tests)
    ├── data_case.ex       # DataCase (domain tests)
    └── feature_case.ex    # FeatureCase (Wallaby E2E)
```

---

## 3. Ash Resource Patterns

### Resource Template

Every Ash resource follows this structure:

```elixir
defmodule MyApp.Projects.Project do
  @moduledoc "Real estate project with lifecycle tracking."
  use Ash.Resource,
    otp_app: :my_app,
    domain: MyApp.Projects,
    data_layer: AshPostgres.DataLayer,
    authorizers: [Ash.Policy.Authorizer]

  postgres do
    table "projects"
    repo MyApp.Repo
  end

  attributes do
    uuid_primary_key :id

    attribute :name, :string do
      allow_nil? false
      public? true
    end

    attribute :status, :atom do
      constraints one_of: [:planning, :active, :completed]
      default :planning
      allow_nil? false
      public? true
    end

    create_timestamp :inserted_at
    update_timestamp :updated_at
  end

  relationships do
    belongs_to :investor, MyApp.Accounts.User do
      allow_nil? false
      public? true
    end
  end

  actions do
    defaults [:read, :destroy]

    create :create do
      primary? true
      accept [:name, :status, :investor_id]
    end

    update :update do
      primary? true
      accept :*
    end
  end

  policies do
    bypass always() do
      authorize_if actor_attribute_equals(:role, :admin)
    end

    policy action_type(:read) do
      authorize_if expr(investor_id == ^actor(:id))
    end
  end
end
```

**Conventions:**
- Always use `uuid_primary_key :id`
- Always include `create_timestamp` and `update_timestamp`
- Always include `authorizers: [Ash.Policy.Authorizer]` — no resource without policies
- Always include `@moduledoc` — even a one-liner
- Admin bypass policy at the top of every resource
- Default to allowing `:read` only — all mutations are named actions with explicit policies
- Use `public? true` on attributes and relationships that should be accessible via actions

### Domain Module Template

```elixir
defmodule MyApp.Projects do
  @moduledoc "Real estate project lifecycle management."
  use Ash.Domain,
    otp_app: :my_app,
    extensions: [AshPaperTrail.Domain]

  resources do
    resource MyApp.Projects.Project
    resource MyApp.Projects.Milestone
  end
end
```

### Migrations

Always use Ash-generated migrations:

```bash
mix ash_postgres.generate_migrations --name describe_the_change
# Review the generated migration
mix ash_postgres.migrate
```

Never edit a migration after it's been committed. Write a new corrective migration instead.

### `require_atomic? false` for Function-Based Changes

Update actions that use anonymous function changes must include `require_atomic? false`:

```elixir
update :complete do
  require_atomic? false
  accept []
  change set_attribute(:status, :completed)

  change fn changeset, _context ->
    Ash.Changeset.force_change_attribute(
      changeset,
      :completed_at,
      DateTime.utc_now() |> DateTime.truncate(:second)
    )
  end
end
```

This is required because `Ash.Resource.Change.Function` does not implement the `atomic/3` callback. Without this option, you get a runtime error.

### `require Ash.Query` in Non-Resource Modules

When using `Ash.Query.filter/2` outside of an Ash resource module (LiveViews, scripts, workers), you must call `require Ash.Query`:

```elixir
defmodule MyAppWeb.Admin.UsersLive do
  use MyAppWeb, :live_view

  require Ash.Query

  def mount(_params, _session, socket) do
    users =
      MyApp.Accounts.User
      |> Ash.Query.filter(role == :investor)
      |> Ash.read!(authorize?: false)

    {:ok, assign(socket, users: users)}
  end
end
```

### Atom Naming in Constraints

Avoid atoms starting with digits in `one_of` constraints:

```elixir
# Good
attribute :budget_range, :atom do
  constraints one_of: [:range_100k_200k, :range_200k_500k]
end

# Bad — syntax error
constraints one_of: [:"100k_200k", :"200k_500k"]
```

### Ash Reads with Arguments

Use the `for_read` pattern, not keyword args:

```elixir
# Good
Resource
|> Ash.Query.for_read(:by_project, %{project_id: id})
|> Ash.read!(authorize?: false)

# Bad — not supported
Ash.read(Resource, args: [project_id: id])
```

### `Ash.load!` in LiveViews

Always pass `authorize?: false` when loading relationships in LiveViews, or it will try to authorize the relationship load with no actor:

```elixir
user = Ash.load!(user, [:investor_profile, :agreements], authorize?: false)
```

---

## 4. Authentication & Authorization

### AshAuthentication Setup

Authentication is handled entirely by AshAuthentication:

```elixir
defmodule MyApp.Accounts.User do
  use Ash.Resource,
    extensions: [AshAuthentication, AshPaperTrail.Resource],
    authorizers: [Ash.Policy.Authorizer]

  authentication do
    tokens do
      enabled? true
      token_resource MyApp.Accounts.AuthToken
      require_token_presence_for_authentication? true

      signing_secret fn _, _ ->
        Application.fetch_env(:my_app, :token_signing_secret)
      end
    end

    strategies do
      password :password do
        identity_field :email
        register_action_accept [:first_name, :last_name]
      end
    end
  end
end
```

### Sign-In with Policies

AshAuthentication's `sign_in_with_password` action is a `:read` action. If your User resource has restrictive policies (e.g., `forbid_if always()` on `:create`), sign-in may fail with `Forbidden`. Use `authorize?: false` in the authentication controller:

```elixir
case AshAuthentication.Strategy.action(strategy, :sign_in, params, authorize?: false) do
  {:ok, user} -> # set session, redirect
  {:error, _} -> # show error
end
```

### LiveView Auth via on_mount

Use `on_mount` hooks in `live_session` blocks for role-based access:

```elixir
# router.ex
live_session :portal,
  layout: {MyAppWeb.Layouts, :portal},
  on_mount: [{MyAppWeb.Live.Auth.RequireAuth, :require_investor_or_admin}] do
  live "/", DashboardLive, :index
end
```

The hook reads `user_id` from session, loads the user via Ash, and assigns `:current_user`:

```elixir
defp get_user_from_session(%{"user_id" => user_id}) when is_binary(user_id) do
  case Ash.get(MyApp.Accounts.User, user_id, authorize?: false) do
    {:ok, %{status: :active} = user} -> user
    _ -> nil
  end
end
```

---

## 5. DaisyUI Components

### Philosophy

Use DaisyUI for all standard UI patterns (buttons, cards, badges, modals, tables, form controls). Build custom Phoenix function components only for domain-specific UI (partner dashboards, pipeline views, stat cards with business logic).

### Installation

DaisyUI is a Tailwind CSS plugin. In Phoenix 1.8 with Tailwind 4.x:

```css
/* assets/css/app.css */
@import "tailwindcss" source(none);
@source "../css";
@source "../js";
@source "../../lib/my_app_web";

@plugin "../vendor/daisyui" {
  themes: false;
}

@plugin "../vendor/daisyui-theme" {
  name: "light";
  default: true;
  /* Override primary color to brand */
  --color-primary: oklch(70% 0.213 47.604);
}
```

### DaisyUI vs. Custom Components Decision Matrix

| Need | Use DaisyUI | Build Custom |
|------|------------|--------------|
| Buttons, links | `btn`, `btn-primary`, `btn-outline` | Never |
| Form controls | `input`, `select`, `textarea`, `checkbox`, `toggle` | Only for Ash-backed forms with AshPhoenix.Form |
| Badges/status | `badge`, `badge-success`, `badge-error` | When badge needs business logic (dynamic color) |
| Cards | `card`, `card-body` | When card has interactive behavior (stat cards) |
| Modals | `modal` | When modal needs LiveView.JS integration |
| Tables | `table`, `table-zebra` | When table needs sorting, filtering, pagination |
| Navigation | `navbar`, `menu` | Always custom — nav is domain-specific |
| Sidebar | `menu` | Always custom — sidebar varies by role |
| Footer | N/A | Always custom — footer has business content |
| Alerts/flash | `alert` | Usually custom for LiveView flash integration |

### DaisyUI + Custom Brand Colors

Define brand colors in the Tailwind `@theme` block so they work alongside DaisyUI's semantic colors:

```css
@theme {
  /* Brand colors (used in custom components) */
  --color-gold: #C8A84E;
  --color-gold-light: #E5D49F;
  --color-gold-dark: #9A7B2C;
  --color-charcoal: #2B2B2B;
  --color-charcoal-deep: #1A1A1A;
  --color-neutral-light: #F5F5F2;
  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
}
```

Use `text-gold`, `bg-charcoal`, etc. in custom components. Use DaisyUI's `btn-primary`, `badge-success` in standard UI. Both coexist without conflict.

### Custom CSS for Interactions

DaisyUI handles component styling, but you'll need custom CSS for interaction patterns that Tailwind utilities can't express. **Use CSS custom properties or DaisyUI's oklch variables — never hardcode hex colors that won't adapt to dark mode.**

```css
/* Sidebar active link — uses @theme custom properties */
.sidebar-link.active {
  border-left: 3px solid var(--color-gold, oklch(70% 0.15 80));
  background-color: oklch(from var(--color-gold, oklch(70% 0.15 80)) l c h / 0.08);
  color: var(--color-gold, oklch(70% 0.15 80));
}

/* Stat card hover lift */
.stat-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 24px oklch(0% 0 0 / 0.15);
}

/* Data table with themed header */
.data-table thead th {
  position: sticky;
  top: 0;
  background-color: oklch(from var(--color-base-content) l c h / 1);
  color: var(--color-base-100);
}
.data-table tbody tr:hover {
  background-color: oklch(from var(--color-gold, oklch(70% 0.15 80)) l c h / 0.06);
}
```

### Dark Mode: Semantic Color Tokens (CRITICAL)

**Never use hardcoded Tailwind color utilities for text or backgrounds in LiveView templates.** DaisyUI activates dark mode via `prefersdark: true` in the theme config. When a user's OS is in dark mode, the page background becomes dark — but hardcoded classes like `text-gray-900` or `bg-gray-50` don't adapt, producing invisible or illegible text.

**The rule:** Use DaisyUI semantic color classes for everything that must work in both themes. Use Tailwind utilities only for layout, spacing, sizing, and responsive design.

| Purpose | WRONG (hardcoded) | RIGHT (semantic) |
|---------|-------------------|-----------------|
| Primary text | `text-gray-900` | `text-base-content` |
| Secondary text | `text-gray-700` | `text-base-content/80` |
| Tertiary text | `text-gray-600` | `text-base-content/70` |
| Muted text | `text-gray-500` | `text-base-content/60` |
| Disabled text | `text-gray-400` | `text-base-content/50` |
| Page background | `bg-white` | `bg-base-100` |
| Surface/card bg | `bg-gray-50`, `bg-gray-100` | `bg-base-200` |
| Borders | `border-gray-200`, `border-gray-300` | `border-base-300` |
| Brand/accent text | `text-red-700` | `text-primary` |
| Brand background | `bg-red-700` | `bg-primary` |
| Brand tinted bg | `bg-red-50` | `bg-primary/10` |
| Brand tinted border | `border-red-100` | `border-primary/20` |
| Link text | `text-blue-600` | `text-info` |
| Success | `text-green-600`, `bg-green-100` | `text-success`, `bg-success/15` |
| Error | `text-red-600`, `bg-red-50` | `text-error`, `bg-error/10` |
| Warning | `text-yellow-600`, `bg-yellow-50` | `text-warning`, `bg-warning/10` |
| Hover states | `hover:bg-gray-50` | `hover:bg-base-200` |
| White on brand bg | `text-white` | `text-primary-content` or keep `text-white` |

**Opacity modifiers** (`/80`, `/70`, `/60`, etc.) work with DaisyUI semantic colors and are the correct way to create lighter variants: `text-base-content/70` for secondary text, `bg-primary/10` for a tinted background.

**Exceptions where hardcoded colors are acceptable:**
- Static HTML not rendered through LiveView (embed forms in iframes, error pages without the theme system)
- Decorative color dots used as legend indicators (e.g., `bg-green-500` for a tiny status dot)
- Skip-nav links (`sr-only`) only visible on keyboard focus

**How dark mode activates:**

```css
@plugin "../vendor/daisyui-theme" {
  name: "dark";
  prefersdark: true;          /* Activates when OS prefers dark */
  color-scheme: "dark";
  --color-base-100: oklch(30.33% 0.016 252.42);   /* Dark background */
  --color-base-content: oklch(97.8% 0.029 256.8);  /* Light text */
  --color-primary: oklch(58% 0.233 277.117);
}

@plugin "../vendor/daisyui-theme" {
  name: "light";
  default: true;
  color-scheme: "light";
  --color-base-100: oklch(98% 0 0);                /* Light background */
  --color-base-content: oklch(21% 0.006 285.885);  /* Dark text */
  --color-primary: oklch(70% 0.213 47.604);
}
```

When `prefersdark: true` is set, any user with dark OS settings gets the dark theme automatically. **Every visible element must render correctly in both themes.**

**Audit: Dark Mode Compliance**

Search for these patterns in `.ex` and `.heex` files — any match in a LiveView template is a dark mode failure:

```
text-gray-[0-9]    → should be text-base-content or text-base-content/NN
bg-gray-50          → should be bg-base-200
bg-white            → should be bg-base-100
text-red-*          → should be text-primary or text-error
bg-red-*            → should be bg-primary/NN or bg-error/NN
text-blue-*         → should be text-info
text-green-*        → should be text-success
border-gray-*       → should be border-base-300
hover:bg-gray-*     → should be hover:bg-base-200
hover:text-gray-*   → should be hover:text-base-content/NN
```

### Key Lesson: DaisyUI Components Don't Fire LiveView Events

DaisyUI's interactive components (dropdowns, modals, accordions) use CSS-only or native HTML behaviors. They do NOT fire `phx-click` or `phx-change` events. For LiveView interactivity:

- **Dropdowns/menus:** Use CSS `:hover` with `group-hover:block` or `Phoenix.LiveView.JS` commands
- **Modals:** Use `Phoenix.LiveView.JS.push` + `phx-click` instead of DaisyUI's checkbox-based modal
- **Tabs:** Use `phx-click` to set an assign, conditionally render with `:if` — DaisyUI's radio-based tabs don't work with LiveView

```elixir
# Tab switching pattern (DaisyUI + LiveView)
def handle_event("switch_tab", %{"tab" => tab}, socket) do
  {:noreply, assign(socket, active_tab: tab)}
end
```

```heex
<div class="tabs tabs-bordered">
  <button phx-click="switch_tab" phx-value-tab="overview"
    class={"tab " <> if(@active_tab == "overview", do: "tab-active", else: "")}>
    Overview
  </button>
  <button phx-click="switch_tab" phx-value-tab="budget"
    class={"tab " <> if(@active_tab == "budget", do: "tab-active", else: "")}>
    Budget
  </button>
</div>

<div :if={@active_tab == "overview"}>Overview content</div>
<div :if={@active_tab == "budget"}>Budget content</div>
```

---

## 6. Form Patterns

### AshPhoenix.Form (Recommended for Ash-Backed Forms)

For any form that creates or updates an Ash resource, use `AshPhoenix.Form`:

```elixir
def mount(_params, _session, socket) do
  form =
    MyApp.Visitors.Visitor
    |> AshPhoenix.Form.for_create(:register)
    |> to_form()

  {:ok, assign(socket, form: form)}
end

def handle_event("validate", %{"form" => params}, socket) do
  form = AshPhoenix.Form.validate(socket.assigns.form, params)
  {:noreply, assign(socket, form: form)}
end

def handle_event("submit", %{"form" => params}, socket) do
  case AshPhoenix.Form.submit(socket.assigns.form, params: params) do
    {:ok, record} -> {:noreply, redirect(socket, to: ~p"/success")}
    {:error, form} -> {:noreply, assign(socket, form: form)}
  end
end
```

This gives you automatic changeset validation, error rendering, and type coercion.

### Simple Forms (Non-Ash)

For forms that don't back to Ash resources (search, filters, contact forms), use plain `to_form`:

```elixir
def mount(_params, _session, socket) do
  {:ok, assign(socket, form: to_form(%{"query" => ""}, as: :search))}
end
```

### DaisyUI Form Styling with AshPhoenix.Form

Combine DaisyUI's form classes with Phoenix form helpers:

```heex
<.form for={@form} phx-change="validate" phx-submit="submit" class="space-y-4">
  <div class="form-control">
    <label class="label"><span class="label-text">Email</span></label>
    <input type="email" name={@form[:email].name} value={@form[:email].value}
      class={"input input-bordered " <> if(@form[:email].errors != [], do: "input-error", else: "")} />
    <label :for={error <- @form[:email].errors} class="label">
      <span class="label-text-alt text-error">{error}</span>
    </label>
  </div>

  <button type="submit" class="btn btn-primary">Submit</button>
</.form>
```

### Filter Pattern (Hardcoded Sample Data)

For pages with hardcoded sample data and filter dropdowns, use the `all_*` + filtered pattern:

```elixir
def mount(_params, _session, socket) do
  all_items = sample_items()
  {:ok, assign(socket, all_items: all_items, items: all_items, filter: "all")}
end

def handle_event("filter", %{"status" => status}, socket) do
  filtered = if status == "all" do
    socket.assigns.all_items
  else
    Enum.filter(socket.assigns.all_items, &(&1.status == status))
  end
  {:noreply, assign(socket, items: filtered, filter: status)}
end
```

**Key lesson:** Never re-call `sample_items()` in the filter handler — it discards any in-memory mutations (advance, approve, reject) that happened since mount.

---

## 7. Testing Patterns

### Test Pyramid (PADL-specific)

```
           ╱╲
          ╱  ╲         E2E (Wallaby) — browser tests, 30+ features
         ╱    ╲
        ╱──────╲
       ╱        ╲      LiveView Tests (ConnCase + Phoenix.LiveViewTest)
      ╱          ╲     Form submissions, event handling, page rendering
     ╱────────────╲
    ╱              ╲    Integration Tests (DataCase + Ash sandbox)
   ╱                ╲   Ash actions through DB, policies, state machines
  ╱──────────────────╲
 ╱                    ╲  Unit Tests (ExUnit)
╱                      ╲ Pure functions, calculations, validations
╱────────────────────────╲
```

### Ash Action-Based Factories

ExMachina doesn't work with Ash resources — ExMachina's Ecto strategy crashes on `%Ash.NotLoaded{}` relationship structs. Use direct Ash action-based factory functions:

```elixir
defmodule MyAppWeb.AuthHelpers do
  alias MyApp.Accounts.User

  def create_user(attrs \\ %{}) do
    defaults = %{
      email: "user-#{System.unique_integer([:positive])}@test.example.com",
      first_name: "Test",
      last_name: "User",
      role: :prospect,
      status: :active
    }

    {:ok, user} = Ash.create(User, Map.merge(defaults, attrs), authorize?: false)
    user
  end

  def create_admin(attrs \\ %{}), do: create_user(Map.put(attrs, :role, :admin))
  def create_investor(attrs \\ %{}), do: create_user(Map.put(attrs, :role, :investor))
  def create_partner(attrs \\ %{}) do
    create_user(Map.merge(%{role: :partner, partner_category: :construction}, attrs))
  end

  def log_in_user(conn, user) do
    conn
    |> Phoenix.ConnTest.init_test_session(%{})
    |> Plug.Conn.put_session(:user_id, user.id)
    |> Plug.Conn.put_session(:live_socket_id, "users_sessions:#{user.id}")
  end
end
```

### E2E Testing with Wallaby

#### Sandbox Hook for LiveView

LiveView processes don't share the test sandbox by default. Create an `on_mount` hook:

```elixir
if Application.compile_env(:my_app, :sql_sandbox) do
  defmodule MyAppWeb.Live.SandboxHook do
    @moduledoc false
    import Phoenix.LiveView
    import Phoenix.Component

    def on_mount(:default, _params, _session, socket) do
      socket =
        assign_new(socket, :phoenix_ecto_sandbox, fn ->
          if connected?(socket), do: get_connect_info(socket, :user_agent)
        end)

      Phoenix.Ecto.SQL.Sandbox.allow(
        socket.assigns.phoenix_ecto_sandbox,
        Ecto.Adapters.SQL.Sandbox
      )

      {:cont, socket}
    end
  end
end
```

Add to all `live_session` blocks **before** auth hooks:

```elixir
live_session :portal,
  on_mount:
    if(Application.compile_env(:my_app, :sql_sandbox),
      do: [{MyAppWeb.Live.SandboxHook, :default}],
      else: []
    ) ++ [{MyAppWeb.Live.Auth.RequireAuth, :require_investor}] do
  # routes
end
```

#### Test Auth Controller

For E2E tests, create a test-only login endpoint:

```elixir
if Mix.env() == :test do
  defmodule MyAppWeb.TestAuthController do
    @moduledoc false
    use MyAppWeb, :controller

    def login(conn, params) do
      conn
      |> put_session(:user_id, Map.fetch!(params, "user_id"))
      |> put_session(:live_socket_id, "users_sessions:#{params["user_id"]}")
      |> redirect(to: Map.get(params, "redirect_to", "/"))
    end
  end
end
```

### Test Configuration

```elixir
# config/test.exs

# SQL sandbox for test isolation
config :my_app, :sql_sandbox, true

# Oban: manual testing mode
config :my_app, Oban, testing: :manual

# Wallaby
config :wallaby,
  otp_app: :my_app,
  driver: Wallaby.Chrome,
  screenshot_on_failure: true

# Server only for E2E
config :my_app, MyAppWeb.Endpoint,
  server: System.get_env("E2E") == "1"
```

### Coverage Configuration

```json
{
  "coverage_options": {
    "minimum_coverage": 100,
    "treat_no_relevant_lines_as_covered": true
  },
  "skip_files": [
    "lib/my_app_web/telemetry.ex",
    "lib/my_app/release.ex",
    "test/"
  ]
}
```

**Skip-file policy:** Only skip genuinely untestable code:
- Compile-time macros (Ash domain `use` lines, Waffle definitions)
- External service dependencies (ChromicPDF)
- Unreachable defensive catch-all clauses (`defp status_class(_), do: "badge-ghost"`)

---

## 8. Layout Patterns

### Fixed Nav + Fixed Sidebar + Content Offset

The standard authenticated layout uses a fixed top nav and fixed left sidebar:

```heex
<%!-- Top nav (fixed, h-16, z-40) --%>
<nav class="fixed top-0 left-0 right-0 h-16 bg-base-100 border-b border-base-300 shadow-sm z-40">
  <!-- Logo, nav items, user menu -->
</nav>

<%!-- Content area with sidebar --%>
<div class="pt-16 flex min-h-screen">
  <%!-- Sidebar (fixed, w-64, below nav) --%>
  <aside class="fixed top-16 left-0 w-64 h-[calc(100vh-4rem)] bg-base-100 border-r border-base-300 z-20
    transform -translate-x-full lg:translate-x-0 transition-transform duration-300">
    <!-- Sidebar content -->
  </aside>

  <%!-- Main content (offset by sidebar width on desktop) --%>
  <div class="flex-1 ml-0 lg:ml-64 flex flex-col">
    <main id="main-content" class="flex-1">
      <div class="p-6 lg:p-8">
        {@inner_content}
      </div>
    </main>
    <footer><!-- Footer --></footer>
  </div>
</div>
```

### CSS-Only Hover Dropdowns

For navigation dropdowns in LiveView, avoid `phx-click` (server round-trip for hover menus is too slow). Use CSS `group-hover:`:

```heex
<div class="relative group">
  <a href="/portal" class="px-3 py-2 text-sm">Portal</a>
  <div class="absolute top-full left-0 w-56 bg-base-100 border border-base-300 rounded-lg shadow-xl
    hidden group-hover:block z-50">
    <a href="/portal/dashboard" class="block px-4 py-2 text-sm hover:bg-base-200">Dashboard</a>
    <a href="/portal/financials" class="block px-4 py-2 text-sm hover:bg-base-200">Financials</a>
  </div>
</div>
```

---

## 9. Deployment (Fly.io)

### Dockerfile Essentials

```dockerfile
ARG ELIXIR_VERSION=1.18.3
ARG OTP_VERSION=27.2
ARG DEBIAN_VERSION=bookworm-20260316-slim

FROM hexpm/elixir:${ELIXIR_VERSION}-erlang-${OTP_VERSION}-debian-${DEBIAN_VERSION} AS builder
# ... standard Phoenix release build

FROM debian:${DEBIAN_VERSION}
# Use CMD not ENTRYPOINT — Fly's release_command needs to replace it
CMD ["/app/bin/server"]
```

**Key lesson:** Use `CMD` not `ENTRYPOINT`. Fly's `release_command` (for migrations) replaces CMD but appends to ENTRYPOINT, causing OOM when the server starts alongside migrations.

### Fly.io Database SSL

Fly internal Postgres doesn't use SSL. Make it configurable:

```elixir
# runtime.exs
db_ssl = System.get_env("DATABASE_SSL") != "false"
ssl_config = if db_ssl, do: [ssl: true, ssl_opts: [verify: :verify_peer, cacerts: :public_key.cacerts_get()]], else: [ssl: false]
```

### Static File Serving

Add new static files (sitemap.xml, llms.txt) to the static_paths:

```elixir
def static_paths, do: ~w(assets fonts images favicon.ico robots.txt sitemap.xml llms.txt)
```

---

## 10. Anti-Patterns (PADL-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Accessing resources via `Repo.get` / raw Ecto | Always go through Ash domain actions |
| Writing raw Ecto queries | Use Ash actions, calculations, and aggregates |
| Custom auth code | Use AshAuthentication — it handles everything |
| Testing with `Repo.insert!` directly | Use Ash action-based factory functions |
| Using ExMachina with Ash resources | Use direct Ash action-based factories — ExMachina fails on NotLoaded |
| Building forms without AshPhoenix.Form | Use AshPhoenix.Form for all Ash-backed forms |
| `phx-click` / `phx-change` without `handle_event` | Every phx binding MUST have a handler — UAT agents catch this |
| Re-calling `sample_data()` in filter handlers | Store full list in `all_*` assign; filter from that |
| DaisyUI tabs with radio inputs | Use `phx-click` + conditional `:if` rendering |
| DaisyUI modal with checkbox toggle | Use `Phoenix.LiveView.JS` for modal show/hide |
| Hover menus with `phx-click` | Use CSS `group-hover:block` — no server round-trip |
| Using `ENTRYPOINT` in Dockerfile | Use `CMD` — Fly's release_command needs to replace it |
| `authorize?: true` on sign-in | Use `authorize?: false` — policies may block the read action |
| `Ash.load!` without `authorize?: false` in LiveViews | Always pass `authorize?: false` in LiveView loads |
| Hardcoding test data in inline scripts for UAT | Use `fly ssh console` + `rpc` to run code in the running app |
| Forgetting `require Ash.Query` in LiveViews | Add `require Ash.Query` at the top of any module using `Ash.Query.filter` |
| Hardcoded Tailwind colors (`text-gray-*`, `bg-white`, `bg-gray-50`) in LiveView templates | Use DaisyUI semantic tokens (`text-base-content`, `bg-base-100`, `bg-base-200`). Hardcoded colors break dark mode — see Section 5 "Dark Mode: Semantic Color Tokens" |
| Hardcoded hex colors (`#2B2B2B`, `rgba(...)`) in custom CSS | Use CSS custom properties or DaisyUI oklch variables that adapt to theme |
| Forms without `novalidate` attribute | Always add `novalidate` — HTML5 native validation is unreliable across assistive technologies |
| Forms without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, `autocomplete="tel"`, etc. |
| "Submit" button text | Use outcome-focused CTA: "Reserve My Free Visit", "Log In", "Create Account" |

---

## 11. Security Patterns

### SecurityHeaders Plug

Every Phoenix app should have a security headers plug:

```elixir
defmodule MyAppWeb.Plugs.SecurityHeaders do
  import Plug.Conn

  def init(opts), do: opts

  def call(conn, _opts) do
    nonce = Base.encode16(:crypto.strong_rand_bytes(16), case: :lower)

    conn
    |> put_resp_header("content-security-policy",
      "default-src 'self'; script-src 'self' 'nonce-#{nonce}'; " <>
      "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; " <>
      "font-src 'self' https://fonts.gstatic.com; " <>
      "img-src 'self' data:; connect-src 'self' wss:; frame-ancestors 'none'")
    |> put_resp_header("strict-transport-security", "max-age=31536000; includeSubDomains")
    |> put_resp_header("x-frame-options", "DENY")
    |> put_resp_header("x-content-type-options", "nosniff")
    |> put_resp_header("referrer-policy", "strict-origin-when-cross-origin")
    |> put_resp_header("permissions-policy", "camera=(), microphone=(), geolocation=(), payment=()")
  end
end
```

### Production Secrets

Never commit secrets. Use environment variables:

```elixir
# runtime.exs
secret_key_base = System.get_env("SECRET_KEY_BASE") || raise "SECRET_KEY_BASE not set"
token_signing_secret = System.get_env("TOKEN_SIGNING_SECRET") || secret_key_base
live_view_salt = System.get_env("LIVE_VIEW_SIGNING_SALT") || "fallback_dev_salt"
```

---

## 12. Development Workflow

### Common Commands

```bash
# Development
mix phx.server                          # Start dev server
mix test                                # Run all tests (except E2E)
mix test test/my_app/projects/          # Run specific domain tests
E2E=1 mix test test/e2e/               # Run E2E browser tests
mix format                              # Format code

# Ash-specific
mix ash_postgres.generate_migrations --name description
mix ash_postgres.migrate

# Quality
mix credo --strict                      # Lint
mix sobelow                             # Security analysis
mix coveralls                           # Coverage (100% enforced)
mix coveralls.detail                    # Line-by-line coverage

# Database
mix ecto.reset                          # Drop + create + migrate + seed
```

### CI Pipeline

```yaml
steps:
  - mix test                    # All tests pass
  - mix coveralls               # 100% coverage
  - mix credo --strict          # Code quality
  - mix sobelow                 # Security analysis
  - mix format --check-formatted # Formatting
```

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a PADL patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:padl&title=[PADL]%20)**

Use the `patterns:padl` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
