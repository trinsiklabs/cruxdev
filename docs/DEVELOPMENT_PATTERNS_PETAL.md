# Development Patterns — PETAL Stack

Phoenix / Elixir / Tailwind / Ash / LiveView

This document captures stack-specific patterns, conventions, and decisions for the BNI Growth Platform. It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **This file** — stack-specific patterns. How we structure Ash resources, test with ExUnit, use Petal Components, integrate GHL, etc.
- **Build plan files** (`1-*.md`, `2-*.md`, etc.) — per-slice actionable plans with checkboxes. Created using the methodology from DEVELOPMENT_PATTERNS.md and the technical patterns from this file.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Elixir | 1.19.5 | |
| Erlang/OTP | 28.3.1 | |
| Phoenix | 1.8.5 | Generator: `phx_new-1.8.5` |
| Phoenix LiveView | ~> 1.0 | Bundled with Phoenix 1.8 |
| Ash Framework | ~> 3.19 | Domain framework — resources, actions, authorization |
| AshPostgres | ~> 2.8 | PostgreSQL data layer |
| AshPhoenix | ~> 2.3 | Phoenix/LiveView integration |
| AshAuthentication | ~> 4.13 | Password + OAuth authentication |
| AshAuthentication Phoenix | ~> 2.0 | Auth UI components |
| AshOban | ~> 0.4 | Background job integration |
| Petal Components | ~> 3.0 | Open source LiveView component library |
| PostgreSQL | 15+ | Running locally, standard port |
| Node.js | 25.x | Asset pipeline only (Tailwind, esbuild) |
| Tailwind CSS | 4.x | Via Phoenix asset pipeline |

### Version Constraint Policy

Use `~>` (pessimistic) constraints in `mix.exs` pinned to the minor version:

```elixir
# Good — allows patch updates, blocks minor/major
{:ash, "~> 3.19"},
{:ash_postgres, "~> 2.8"},
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

Each Ash domain is a directory under `lib/bni_platform/` containing a domain module and its resources:

```
lib/bni_platform/
├── accounts/              # Auth domain
│   ├── accounts.ex        # Domain module
│   ├── user.ex            # Ash resource
│   └── token.ex           # Ash resource (auth tokens)
├── chapters/              # Chapter management domain
│   ├── chapters.ex        # Domain module
│   ├── chapter.ex
│   └── seat.ex
├── members/               # Member profiles domain
│   ├── members.ex
│   ├── member.ex
│   └── recruitment_claim.ex
├── visitors/              # Visitor pipeline domain
│   ├── visitors.ex
│   ├── visitor.ex
│   └── visit.ex
└── ghl/                   # GHL integration (not an Ash domain)
    ├── client.ex          # Behaviour
    ├── http_client.ex     # Production Req-based impl
    └── webhook_handler.ex
```

**Convention:** One Ash domain per bounded context. The domain module (`accounts.ex`, `chapters.ex`) defines the domain and lists its resources. Resources are never accessed directly — always through the domain's actions.

### Test Mirror Structure

Tests mirror the `lib/` structure:

```
test/
├── bni_platform/          # Unit + integration tests
│   ├── accounts/
│   ├── chapters/
│   ├── members/
│   ├── visitors/
│   ├── ghl/
│   └── workers/
├── bni_platform_web/      # Feature tests (PhoenixTest)
│   ├── live/
│   └── controllers/
├── support/
│   ├── factory.ex         # Ash action-based factories
│   ├── mocks.ex           # Mox mock definitions
│   ├── conn_case.ex
│   ├── data_case.ex
│   └── feature_case.ex
└── test_helper.exs
```

---

## 3. Ash Resource Patterns

### Resource Template

Every Ash resource follows this structure:

```elixir
defmodule BniPlatform.Chapters.Chapter do
  use Ash.Resource,
    domain: BniPlatform.Chapters,
    data_layer: AshPostgres.DataLayer,
    authorizers: [Ash.Policy.Authorizer]

  postgres do
    table "chapters"
    repo BniPlatform.Repo
  end

  attributes do
    uuid_primary_key :id
    # ... attributes
    timestamps()
  end

  relationships do
    # ... belongs_to, has_many, etc.
  end

  actions do
    defaults [:read]
    # Named actions for each operation
  end

  policies do
    # Authorization rules
  end
end
```

**Conventions:**
- Always use `uuid_primary_key :id`
- Always include `timestamps()`
- Always include `authorizers: [Ash.Policy.Authorizer]` — no resource without policies
- Name actions descriptively: `:register`, `:update_profile`, not `:create`, `:update`
- Default to allowing `:read` only — all mutations are named actions with explicit policies

### Domain Module Template

```elixir
defmodule BniPlatform.Chapters do
  use Ash.Domain

  resources do
    resource BniPlatform.Chapters.Chapter
    resource BniPlatform.Chapters.Seat
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

### `require_atomic?(false)` for Function-Based Changes

Update actions that use anonymous function changes (via `change fn changeset, _context -> ... end`) must include `require_atomic?(false)` in the action definition. This is because `Ash.Resource.Change.Function` does not implement the `atomic/3` callback, so Ash cannot run the action atomically. Without this option, you'll get a runtime error.

```elixir
update :assign_member do
  require_atomic? false

  accept [:member_id]

  change fn changeset, _context ->
    # Custom logic that can't be expressed atomically
    Ash.Changeset.change_attribute(changeset, :status, :filled)
  end
end
```

### `require Ash.Query` in Non-Resource Modules

When using `Ash.Query.filter/2` (or other `Ash.Query` macros) outside of an Ash resource module — such as in LiveViews, custom changes, or scripts — you must call `require Ash.Query` before using it. The `filter/2` macro needs to be compiled, and it is only auto-required inside Ash resource modules.

```elixir
defmodule BniPlatformWeb.ChapterLive.Index do
  use BniPlatformWeb, :live_view

  require Ash.Query

  def mount(_params, _session, socket) do
    chapters =
      BniPlatform.Chapters.Chapter
      |> Ash.Query.filter(active == true)
      |> Ash.read!()

    {:ok, assign(socket, chapters: chapters)}
  end
end
```

---

## 4. Authentication & Authorization

### AshAuthentication Setup

Authentication is handled entirely by AshAuthentication — no custom auth code:

```elixir
defmodule BniPlatform.Accounts.User do
  use Ash.Resource,
    domain: BniPlatform.Accounts,
    data_layer: AshPostgres.DataLayer,
    authorizers: [Ash.Policy.Authorizer],
    extensions: [AshAuthentication]

  authentication do
    tokens do
      enabled? true
      token_resource BniPlatform.Accounts.Token
      signing_secret fn _, _ ->
        Application.fetch_env(:bni_platform, :token_signing_secret)
      end
    end

    strategies do
      password :password do
        identity_field :email
        hashed_password_field :hashed_password
      end
    end
  end
end
```

### Role Model

Four roles, enforced through Ash policies:

| Role | Value | Access |
|---|---|---|
| `visitor` | (unauthenticated) | Public pages, registration form |
| `member` | `:member` | Own profile, roster, claim seats |
| `chapter_admin` | `:chapter_admin` | Chapter management, visitor pipeline, user admin |
| `platform_admin` | `:platform_admin` | Everything, all chapters |

Stored as an atom/enum attribute on the User resource. Policies check actor role:

```elixir
policies do
  policy action_type(:read) do
    authorize_if always()  # public profiles
  end

  policy action(:update_profile) do
    authorize_if relates_to_actor_via(:user)
  end

  policy action(:manage_chapter) do
    authorize_if actor_attribute_equals(:role, :chapter_admin)
    authorize_if actor_attribute_equals(:role, :platform_admin)
  end
end
```

---

## 5. Petal Components

### Philosophy

Use `petal_components` for all standard UI. Do NOT build custom components when Petal provides one. Only build custom components for domain-specific UI (seat roster grid, visitor pipeline visualization, etc.).

### Installation

```elixir
# mix.exs
{:petal_components, "~> 3.0"}
```

### Usage

Import in the web module for global availability:

```elixir
# lib/bni_platform_web.ex
defp html_helpers do
  quote do
    use PetalComponents
    # ... other imports
  end
end
```

Or selectively import in specific LiveViews:

```elixir
use PetalComponents
# or
import PetalComponents.Button
import PetalComponents.Card
```

### Component Inventory (What Petal Gives Us)

| Category | Components | Use For |
|---|---|---|
| **Forms** | text_input, select, textarea, checkbox, radio, switch, file_upload | All forms — registration, profile edit, admin |
| **Buttons** | button, button_group, icon_button | Actions, CTAs, navigation |
| **Feedback** | alert, badge, progress, spinner | Status indicators, notifications |
| **Layout** | card, container, accordion | Page sections, content grouping |
| **Navigation** | breadcrumbs, pagination, tabs | Page navigation, list pagination |
| **Overlay** | modal, slide_over, dropdown | Confirmations, menus, detail panels |
| **Data** | table | Member directory, seat roster, admin lists |
| **Misc** | avatar, link | User display, navigation |

### Custom Components (Build Only These)

- **Seat roster grid** — domain-specific visualization of chapter seats (open/filled/claimed/split)
- **Visitor pipeline stage indicator** — shows where a visitor is in the funnel
- **Embeddable registration form** — standalone form that works in iframes

### Tailwind Integration

Petal uses Tailwind classes. Phoenix 1.8 uses Tailwind 4.x with CSS-based config (not the JS-based `tailwind.config.js` from v3):

```css
/* assets/css/app.css — Tailwind 4.x uses CSS-based config */
@import "tailwindcss" source(none);
@source "../css";
@source "../js";
@source "../../lib/bni_platform_web";
@source "../../deps/petal_components/lib";

@theme {
  --color-bni-red: #CF2030;
  --color-bni-gray: #58595B;
}

@plugin "@tailwindcss/forms";
@plugin "@tailwindcss/typography";
```

---

## 6. Testing Patterns

### Test Pyramid (PETAL-specific)

```
        ╱╲
       ╱  ╲          E2E (Playwright) — deferred to slice 2+
      ╱    ╲
     ╱──────╲
    ╱        ╲        Feature Tests (PhoenixTest + LiveView driver)
   ╱          ╲       LiveView interactions, form submissions, page flows
  ╱────────────╲
 ╱              ╲      Integration Tests (ExUnit + Ash sandbox)
╱                ╲     Ash actions through DB, policies, GHL client (Bypass)
╱──────────────────╲
╱                    ╲   Unit Tests (ExUnit)
╱                      ╲  Pure functions, calculations, validations
╱────────────────────────╲
```

### Ash-Specific Testing

**Unit tests** — test calculations, validations, and changes in isolation:

```elixir
test "seat status is :claimed when a recruitment claim exists" do
  # Test the calculation logic, not the DB
end
```

**Integration tests** — test Ash actions end-to-end through the database:

```elixir
test "registering a visitor creates a GHL contact" do
  chapter = Factory.insert(:chapter)

  assert {:ok, visitor} =
    BniPlatform.Visitors.register(%{
      name: "Jane Smith",
      email: "jane@example.com",
      chapter_id: chapter.id
    })

  assert visitor.ghl_contact_id != nil
end
```

**Feature tests** — test user-facing flows through LiveView:

```elixir
test "visitor can register from chapter page", %{conn: conn} do
  chapter = Factory.insert(:chapter)

  conn
  |> visit("/chapters/#{chapter.slug}")
  |> click_link("Register to Visit")
  |> fill_in("Name", with: "Jane Smith")
  |> fill_in("Email", with: "jane@example.com")
  |> click_button("Register")
  |> assert_has("text", "You're registered!")
end
```

### Ash Action-Based Factories

ExMachina doesn't work with Ash resources — ExMachina's Ecto strategy crashes on `%Ash.NotLoaded{}` relationship structs. Instead, use direct Ash action-based factory functions:

```elixir
defmodule BniPlatform.Factory do
  def create_user!(attrs \\ %{}) do
    defaults = %{
      email: "user#{System.unique_integer([:positive])}@example.com",
      hashed_password: Bcrypt.hash_pwd_salt("password123"),
      role: :member
    }

    BniPlatform.Accounts.User
    |> Ash.Changeset.for_create(:register, Map.merge(defaults, attrs))
    |> Ash.create!()
  end

  def create_chapter!(attrs \\ %{}) do
    defaults = %{
      name: "Chapter #{System.unique_integer([:positive])}",
      slug: "chapter-#{System.unique_integer([:positive])}",
      meeting_day: :wednesday,
      meeting_time: ~T[07:00:00],
      location: "Denver, NC"
    }

    BniPlatform.Chapters.Chapter
    |> Ash.Changeset.for_create(:create, Map.merge(defaults, attrs))
    |> Ash.create!()
  end

  def create_seat!(chapter, attrs \\ %{}) do
    defaults = %{
      classification: "Plumber",
      category: "Home Services",
      status: :open,
      chapter_id: chapter.id
    }

    BniPlatform.Chapters.Seat
    |> Ash.Changeset.for_create(:create, Map.merge(defaults, attrs))
    |> Ash.create!()
  end

  # ... more factory functions following the same pattern
end
```

### GHL Testing (Mox + Bypass)

The GHL client uses a behaviour so tests can swap implementations via Mox:

```elixir
# Behaviour (lib/bni_platform/ghl/client.ex)
defmodule BniPlatform.GHL.Client do
  @callback create_contact(map()) :: {:ok, map()} | {:error, term()}
  @callback create_opportunity(map()) :: {:ok, map()} | {:error, term()}
  # ...
end

# Define mock in test/support/mocks.ex
Mox.defmock(BniPlatform.GHL.MockClient, for: BniPlatform.GHL.Client)

# Config-based dispatch
# config/config.exs
config :bni_platform, :ghl_client, BniPlatform.GHL.HttpClient

# config/test.exs
config :bni_platform, :ghl_client, BniPlatform.GHL.MockClient
```

In tests, set expectations on the Mox-generated mock:

```elixir
test "registering a visitor creates a GHL contact" do
  expect(BniPlatform.GHL.MockClient, :create_contact, fn params ->
    {:ok, %{"id" => "ghl_123"}}
  end)

  # ... test code that triggers the GHL call
end
```

For HTTP-level integration tests against the real `HttpClient`, use Bypass:

```elixir
test "GHL HTTP client sends correct payload" do
  bypass = Bypass.open()

  Bypass.expect_once(bypass, "POST", "/contacts", fn conn ->
    {:ok, body, conn} = Plug.Conn.read_body(conn)
    decoded = Jason.decode!(body)
    assert "chapter:westlake-select" in decoded["tags"]
    Plug.Conn.resp(conn, 200, Jason.encode!(%{"contact" => %{"id" => "ghl_123"}}))
  end)

  # Test against bypass URL
end
```

### Test Configuration

```elixir
# config/test.exs

# Ash: disable async for transactional tests
config :ash, :disable_async?, true
config :ash, :missed_notifications, :ignore

# Oban: manual testing mode (allows explicit assertions via assert_enqueued
# without automatic job execution; :inline would run jobs immediately, making
# it impossible to test that the correct job was enqueued)
config :bni_platform, Oban, testing: :manual

# GHL: use Mox-generated mock client
config :bni_platform, :ghl_client, BniPlatform.GHL.MockClient
```

---

## 7. LiveView Patterns

### Page Structure

Every LiveView page follows:

```elixir
defmodule BniPlatformWeb.ChapterLive.Show do
  use BniPlatformWeb, :live_view

  @impl true
  def mount(%{"slug" => slug}, _session, socket) do
    chapter = BniPlatform.Chapters.get_by_slug!(slug)
    {:ok, assign(socket, chapter: chapter)}
  end

  @impl true
  def handle_params(params, _url, socket) do
    {:noreply, apply_action(socket, socket.assigns.live_action, params)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <.container>
      <%!-- Use Petal Components --%>
    </.container>
    """
  end
end
```

### Form Handling with AshPhoenix

Use `AshPhoenix.Form` for all Ash-backed forms:

```elixir
def mount(_params, _session, socket) do
  form =
    BniPlatform.Visitors.Visitor
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
    {:ok, visitor} -> {:noreply, redirect(socket, to: ~p"/registered")}
    {:error, form} -> {:noreply, assign(socket, form: form)}
  end
end
```

---

## 8. Embeddable Visitor Registration Form

The visitor registration form must work in three contexts:

1. **On-platform** — LiveView form on the chapter's public page
2. **Iframe embed** — standalone HTML page loaded in an iframe on external sites
3. **API** — direct POST from any frontend (future-proofing for GoHighLevel custom forms)

### Architecture

```
External site                    Platform
┌──────────────┐                ┌──────────────────────┐
│ <iframe       │──── loads ───→│ /embed/register/:slug │  (minimal layout, no nav)
│  src="...">  │                │                      │
│              │◄── redirect ──│ /embed/thank-you      │
└──────────────┘                └──────────────────────┘

                                ┌──────────────────────┐
GoHighLevel form ── POST ─────→│ /api/v1/visitors      │  (JSON API)
                                │                      │
                                │ Returns: 201 + JSON  │
                                └──────────────────────┘

                                ┌──────────────────────┐
Platform chapter page           │ /chapters/:slug       │  (full layout, LiveView)
                                │  └── registration form │
                                └──────────────────────┘
```

### Implementation

- **LiveView form** — the canonical implementation, used on-platform
- **Embed controller** — renders the same form in a minimal layout (no header/nav/footer), sets `Content-Security-Policy: frame-ancestors *` and `Referrer-Policy: no-referrer` headers. Note: `X-Frame-Options: ALLOWALL` is not a valid value — modern browsers use the CSP `frame-ancestors` directive instead.
- **API endpoint** — `POST /api/v1/visitors` accepts JSON, returns JSON. Used by external forms. Validates with the same Ash action as the LiveView form.
- **Shared Ash action** — `Visitors.register/1` is the single source of truth. All three entry points call the same action. GHL contact creation is triggered by the action, not the controller.

### Embed Snippet (Given to External Sites)

```html
<iframe
  src="https://app.bnigrowtplatform.com/embed/register/westlake-select"
  width="100%"
  height="600"
  frameborder="0"
  style="border: none;">
</iframe>
```

---

## 9. GHL Integration Patterns

### Client Behaviour

```elixir
defmodule BniPlatform.GHL.Client do
  @callback create_contact(params :: map()) :: {:ok, map()} | {:error, term()}
  @callback update_contact(id :: String.t(), params :: map()) :: {:ok, map()} | {:error, term()}
  @callback create_opportunity(params :: map()) :: {:ok, map()} | {:error, term()}
  @callback update_opportunity_stage(id :: String.t(), stage_id :: String.t()) :: {:ok, map()} | {:error, term()}
end
```

### Pipeline Stages

| Platform Status | GHL Stage | Direction |
|---|---|---|
| Registered | Registered | Platform → GHL (on visitor create) |
| Visited | Visited | Bidirectional (webhook or platform update) |
| Application | Application | GHL → Platform (webhook) |
| Approved | Approved | GHL → Platform (webhook) |
| Member | Member | Platform → GHL (on member create) |

### Webhook Processing

Inbound webhooks go through an Oban job for reliability:

```
POST /api/webhooks/ghl
  → Verify signature
  → Enqueue Oban job (WebhookProcessWorker)
  → Return 200 immediately
  → Job processes asynchronously: parse event, update platform state
```

### Tag Convention

GHL contacts are tagged for filtering and workflow triggers:

```
chapter:westlake-select
role:visitor | role:member
source:google-search | source:referral | source:website
seat:plumber (when profession matches open seat)
status:registered | status:visited | status:applied | status:approved | status:member
```

---

## 10. Seed Data

### Westlake Select Chapter

The MVP seeds one chapter with its full seat roster. The BNI classification taxonomy comes from `bni-professional-classifications-full-list.md` (companion doc in the project).

Seed data is in `priv/repo/seeds.exs` and is idempotent (safe to run multiple times).

```elixir
# Seeds create:
# 1. Westlake Select chapter (name, meeting day/time, location)
# 2. Full seat roster with classifications
# 3. Seed seat splits for any currently split seats
# 4. Platform admin user (the user)
```

The actual seat roster (which classifications, which are filled, which are split) comes from the user — this is configuration data, not generated.

---

## 11. Development Workflow

### Feature Development Cycle (PETAL-specific)

```
1. Write BDD scenarios (docs/scenarios/*.md)
2. Design test levels (unit / integration / feature)
3. Write failing tests (ExUnit)
4. Write Ash resource / action / policy code
5. Write LiveView (using Petal Components)
6. Run: mix test
7. Run: mix format
8. Refactor while green
9. Run: mix credo --strict
```

### Common Commands

```bash
# Development
mix phx.server                          # Start dev server
mix test                                # Run all tests (except E2E)
mix test test/bni_platform/chapters/    # Run specific domain tests
mix format                              # Format code
mix credo --strict                      # Lint

# Ash-specific
mix ash_postgres.generate_migrations --name description
mix ash_postgres.migrate
mix ash_postgres.rollback               # Dev only, never production

# Quality
mix format --check-formatted            # CI check
mix credo --strict                      # Lint
mix deps.audit                          # Vulnerability check

# Database
mix ecto.reset                          # Drop + create + migrate + seed
mix run priv/repo/seeds.exs             # Seed data only
```

### Mix Aliases

```elixir
defp aliases do
  [
    setup: ["deps.get", "ecto.setup", "assets.setup", "assets.build"],
    "ecto.setup": ["ecto.create", "ash_postgres.migrate", "run priv/repo/seeds.exs"],
    "ecto.reset": ["ecto.drop", "ecto.setup"],
    test: ["ecto.create --quiet", "ash_postgres.migrate --quiet", "test"],
    quality: ["format --check-formatted", "credo --strict", "deps.audit"],
    ci: ["quality", "test"]
  ]
end
```

---

## 12. Deployment (Future)

Not in scope for slice 1 — development is local only. Deployment patterns will be added when we're ready to host.

---

## 13. Anti-Patterns (PETAL-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Building custom form components | Use Petal's form components |
| Building custom modal/dropdown | Use Petal's overlay components |
| Accessing resources directly (e.g., `Repo.get`) | Always go through Ash domain actions |
| Writing raw Ecto queries | Use Ash actions, calculations, and aggregates |
| Custom auth code | Use AshAuthentication — it handles everything |
| Testing with `Repo.insert!` directly | Use Ash action-based factory functions |
| Hardcoding GHL API calls | Use the Client behaviour — swap impl in tests |
| Building forms without AshPhoenix.Form | Always use AshPhoenix.Form for Ash-backed forms |
| `phx-` events without `handle_event` | Every phx binding needs a handler |
| Inline styles | Tailwind utility classes only |
| Writing a hand-written mock module | Use Mox with the behaviour to generate mocks dynamically |
| Using ExMachina insert/build with Ash resources | Use direct Ash action-based factory functions — ExMachina's Ecto strategy fails on Ash NotLoaded relationship structs |
