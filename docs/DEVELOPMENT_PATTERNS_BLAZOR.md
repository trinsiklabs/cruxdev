# Development Patterns — Blazor Stack

ASP.NET Core / Blazor / MudBlazor / Entity Framework Core / ASP.NET Identity

This document captures stack-specific patterns, conventions, and decisions for Blazor stack projects (ASP.NET Core + Blazor SSR/WASM/Hybrid + MudBlazor + Entity Framework Core). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Blazor components, test with xUnit/bUnit, use MudBlazor, deploy with Docker, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| .NET SDK | 9.0+ | LTS preferred; `global.json` pins the exact version |
| ASP.NET Core | 9.0 | Unified Blazor Web App template |
| Blazor | 9.0 | SSR + Interactive Server + Interactive WASM + Auto rendering modes |
| MudBlazor | 9.x | Material Design component library; full .NET 9 support |
| Entity Framework Core | 9.0 | ORM with migrations, compiled models, pre-compiled queries |
| ASP.NET Core Identity | 9.0 | Authentication + authorization + 2FA + external providers |
| SQL Server / PostgreSQL | 16+ / 17+ | EF Core supports both; pick one per project |
| xUnit | 2.9+ | Test framework |
| bUnit | 1.x | Blazor component unit testing |
| Playwright .NET | 1.49+ | End-to-end browser testing |
| Docker | 27+ | Multi-stage build for deployment |
| Node.js | N/A | Not required — Blazor eliminates JS build pipelines |

### Version Constraint Policy

Pin the SDK version in `global.json` at the project root:

```json
{
  "sdk": {
    "version": "9.0.200",
    "rollForward": "latestPatch"
  }
}
```

Use exact or minor-range version constraints in `.csproj`:

```xml
<!-- Good — allows patch updates within the minor version -->
<PackageReference Include="MudBlazor" Version="9.2.*" />
<PackageReference Include="Microsoft.EntityFrameworkCore" Version="9.0.*" />

<!-- Good — exact pin for known-stable version -->
<PackageReference Include="MudBlazor" Version="9.2.0" />

<!-- Bad — too loose, allows breaking minor updates -->
<PackageReference Include="MudBlazor" Version="9.*" />

<!-- Bad — floating major, invites breaking changes -->
<PackageReference Include="MudBlazor" Version="*" />
```

Use `dotnet outdated` (global tool) to audit dependency freshness. Run `dotnet list package --vulnerable` to check for known CVEs.

---

## 2. Project Structure

### Blazor Web App Organization

The .NET 9 Blazor Web App template produces a unified project that supports all rendering modes. For larger applications, split into a server project and a client (WASM) project:

```
Solution/
├── src/
│   ├── MyApp/                         # Server project (hosts everything)
│   │   ├── Program.cs                 # Service registration + middleware pipeline
│   │   ├── Components/
│   │   │   ├── App.razor              # Root component (defines <HeadOutlet>, routes)
│   │   │   ├── Routes.razor           # Router configuration
│   │   │   ├── Layout/
│   │   │   │   ├── MainLayout.razor   # MudBlazor layout (MudLayout, MudAppBar, etc.)
│   │   │   │   └── NavMenu.razor      # Navigation (MudNavMenu)
│   │   │   ├── Pages/                 # Server-rendered pages (SSR + Interactive Server)
│   │   │   │   ├── Home.razor
│   │   │   │   ├── Dashboard.razor
│   │   │   │   ├── Admin/
│   │   │   │   │   ├── Users.razor
│   │   │   │   │   └── Settings.razor
│   │   │   │   ├── Account/
│   │   │   │   │   ├── Login.razor
│   │   │   │   │   ├── Register.razor
│   │   │   │   │   └── Manage.razor
│   │   │   │   └── Error.razor
│   │   │   └── Shared/               # Shared non-page components
│   │   │       ├── ConfirmDialog.razor
│   │   │       └── StatusBadge.razor
│   │   ├── Data/
│   │   │   ├── AppDbContext.cs        # EF Core DbContext
│   │   │   ├── Entities/             # Domain entities
│   │   │   │   ├── Project.cs
│   │   │   │   ├── Task.cs
│   │   │   │   └── AuditLog.cs
│   │   │   ├── Configurations/       # EF Fluent API configurations
│   │   │   │   ├── ProjectConfiguration.cs
│   │   │   │   └── TaskConfiguration.cs
│   │   │   └── Migrations/           # EF Core migrations (auto-generated)
│   │   ├── Services/                 # Business logic services
│   │   │   ├── IProjectService.cs
│   │   │   ├── ProjectService.cs
│   │   │   └── EmailService.cs
│   │   ├── Extensions/              # Service registration extensions
│   │   │   ├── ServiceCollectionExtensions.cs
│   │   │   └── MiddlewareExtensions.cs
│   │   └── wwwroot/                 # Static assets
│   │       ├── css/
│   │       └── images/
│   └── MyApp.Client/                 # Client project (WASM components only)
│       ├── Pages/                    # WASM-interactive pages
│       │   ├── RealTimeChart.razor
│       │   └── OfflineEditor.razor
│       ├── Services/                 # Client-side services (HttpClient wrappers)
│       │   └── ApiClient.cs
│       └── _Imports.razor
├── tests/
│   ├── MyApp.Tests/                  # Unit + integration tests
│   │   ├── Services/
│   │   ├── Data/
│   │   └── Components/              # bUnit component tests
│   └── MyApp.E2E/                   # Playwright end-to-end tests
│       ├── PageTests/
│       └── PlaywrightFixture.cs
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── global.json
├── Directory.Build.props             # Shared build properties
└── MyApp.sln
```

**Convention:** Server-rendered pages live in the server project's `Components/Pages/`. WASM-interactive pages live in the `.Client` project's `Pages/`. This separation makes render mode boundaries explicit.

**Convention:** One service interface + implementation per bounded context. Services are registered via extension methods in `Extensions/ServiceCollectionExtensions.cs`, not inline in `Program.cs`.

**Convention:** EF entity configurations use the Fluent API in separate `IEntityTypeConfiguration<T>` classes, never data annotations on entities. This keeps entities clean and testable.

### Test Mirror Structure

Tests mirror the `src/` structure:

```
tests/
├── MyApp.Tests/                      # Unit + integration + bUnit
│   ├── Services/
│   │   ├── ProjectServiceTests.cs
│   │   └── EmailServiceTests.cs
│   ├── Data/
│   │   ├── AppDbContextTests.cs
│   │   └── Entities/
│   │       └── ProjectTests.cs
│   ├── Components/
│   │   ├── Pages/
│   │   │   ├── HomeTests.cs
│   │   │   └── DashboardTests.cs
│   │   └── Shared/
│   │       └── ConfirmDialogTests.cs
│   ├── Fixtures/
│   │   ├── DatabaseFixture.cs        # Shared EF test database
│   │   └── ServiceFixture.cs
│   └── Helpers/
│       ├── TestDataBuilder.cs        # Builder pattern for test entities
│       └── MudBlazorTestExtensions.cs
├── MyApp.E2E/                        # Playwright E2E
│   ├── PageTests/
│   │   ├── LoginPageTests.cs
│   │   ├── DashboardPageTests.cs
│   │   └── AdminPageTests.cs
│   ├── Helpers/
│   │   └── AuthenticationHelper.cs
│   └── PlaywrightFixture.cs
└── Directory.Build.props
```

---

## 3. Blazor Rendering Modes

### The Four Modes

.NET 9 Blazor supports four rendering modes. Choosing the right mode per component is a critical architectural decision:

| Mode | Attribute | Where It Runs | Interactive | Use When |
|---|---|---|---|---|
| Static SSR | (default) | Server | No | Content pages, SEO-critical pages, login/register |
| Interactive Server | `@rendermode InteractiveServer` | Server (SignalR) | Yes | Admin dashboards, data grids, real-time updates |
| Interactive WASM | `@rendermode InteractiveWebAssembly` | Browser | Yes | Offline-capable, CPU-intensive client work |
| Interactive Auto | `@rendermode InteractiveAuto` | Server then Browser | Yes | Best of both — fast first load, then client-side |

### Setting Render Modes

Render modes can be set globally (in `App.razor`) or per-component:

```razor
@* App.razor — global default: Static SSR with Interactive Server for routed pages *@
<!DOCTYPE html>
<html lang="en">
<head>
    <HeadOutlet @rendermode="InteractiveServer" />
</head>
<body>
    <Routes @rendermode="InteractiveServer" />
    <script src="_framework/blazor.web.js"></script>
</body>
</html>
```

```razor
@* Per-component override — this page is WASM-interactive *@
@page "/realtime-chart"
@rendermode InteractiveWebAssembly

<MudChart ChartType="ChartType.Line" ... />
```

```razor
@* Per-component override — this page is Static SSR (opt out of global interactivity) *@
@page "/privacy"
@attribute [ExcludeFromInteractiveRouting]

<MudText Typo="Typo.h3">Privacy Policy</MudText>
<MudText>This page is server-rendered with no interactive runtime.</MudText>
```

### Render Mode Decision Tree

```
Is the page content-only (no user interaction beyond links)?
  YES → Static SSR (default, no attribute needed)
  NO  → Does it need real-time server data (SignalR, live updates)?
           YES → InteractiveServer
           NO  → Does it need to work offline or do heavy client computation?
                    YES → InteractiveWebAssembly
                    NO  → InteractiveAuto (fast first paint, then client-side)
```

### Static SSR Pages with Identity

Pages that depend on HTTP cookies (login, register, account management) must use Static SSR. In .NET 9, use `[ExcludeFromInteractiveRouting]` to opt these pages out of global interactivity:

```csharp
@page "/Account/Login"
@attribute [ExcludeFromInteractiveRouting]

@* This page uses cookies — it MUST be Static SSR *@
```

### Streaming Rendering

For Static SSR pages with slow data loads, use streaming rendering to send the shell immediately and stream content as it becomes available:

```razor
@page "/reports"
@attribute [StreamRendering]

@if (reports is null)
{
    <MudProgressLinear Indeterminate="true" />
}
else
{
    <MudDataGrid Items="@reports" ... />
}

@code {
    private List<Report>? reports;

    protected override async Task OnInitializedAsync()
    {
        reports = await ReportService.GetAllAsync();
    }
}
```

### Pre-rendering Considerations

Interactive components are pre-rendered by default (server renders HTML, then the interactive runtime takes over). This means `OnInitializedAsync` runs twice — once during pre-render, once when interactive. Guard against double-loading:

```csharp
@code {
    private List<Project>? projects;
    private bool isLoaded;

    protected override async Task OnInitializedAsync()
    {
        if (!isLoaded)
        {
            projects = await ProjectService.GetAllAsync();
            isLoaded = true;
        }
    }
}
```

To disable pre-rendering for a specific component:

```razor
<Dashboard @rendermode="new InteractiveServerRenderMode(prerender: false)" />
```

---

## 4. Authentication & Authorization

### ASP.NET Core Identity Setup

Authentication uses ASP.NET Core Identity — no custom auth code. The .NET 9 template includes Identity UI scaffolding:

```csharp
// Program.cs — service registration
builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlServer(builder.Configuration.GetConnectionString("Default")));

builder.Services.AddIdentity<ApplicationUser, IdentityRole>(options =>
{
    // Password policy
    options.Password.RequireDigit = true;
    options.Password.RequiredLength = 12;
    options.Password.RequireNonAlphanumeric = true;
    options.Password.RequireUppercase = true;
    options.Password.RequireLowercase = true;

    // Lockout policy
    options.Lockout.DefaultLockoutTimeSpan = TimeSpan.FromMinutes(15);
    options.Lockout.MaxFailedAccessAttempts = 5;
    options.Lockout.AllowedForNewUsers = true;

    // User settings
    options.User.RequireUniqueEmail = true;
    options.SignIn.RequireConfirmedEmail = true;
})
.AddEntityFrameworkStores<AppDbContext>()
.AddDefaultTokenProviders();

// Cookie configuration
builder.Services.ConfigureApplicationCookie(options =>
{
    options.Cookie.HttpOnly = true;
    options.Cookie.SecurePolicy = CookieSecurePolicy.Always;
    options.Cookie.SameSite = SameSiteMode.Strict;
    options.ExpireTimeSpan = TimeSpan.FromHours(2);
    options.SlidingExpiration = true;
    options.LoginPath = "/Account/Login";
    options.AccessDeniedPath = "/Account/AccessDenied";
});
```

### ApplicationUser Extension

Extend the default IdentityUser with domain-specific properties:

```csharp
public class ApplicationUser : IdentityUser
{
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
    public string FullName => $"{FirstName} {LastName}";
    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
    public DateTimeOffset? LastLoginAt { get; set; }
    public bool IsActive { get; set; } = true;
}
```

### Role Model

Define roles as constants, seeded at startup:

| Role | Constant | Access |
|---|---|---|
| `User` | `AppRoles.User` | Own profile, own data, public pages |
| `Manager` | `AppRoles.Manager` | Team data, reports, user management within scope |
| `Admin` | `AppRoles.Admin` | Full system access, settings, all users |

```csharp
public static class AppRoles
{
    public const string User = "User";
    public const string Manager = "Manager";
    public const string Admin = "Admin";

    public static readonly string[] All = [User, Manager, Admin];
}
```

Seed roles at startup:

```csharp
// Program.cs — after app.Build()
using (var scope = app.Services.CreateScope())
{
    var roleManager = scope.ServiceProvider.GetRequiredService<RoleManager<IdentityRole>>();
    foreach (var role in AppRoles.All)
    {
        if (!await roleManager.RoleExistsAsync(role))
        {
            await roleManager.CreateAsync(new IdentityRole(role));
        }
    }
}
```

### Authorization Policies

Define policies in `Program.cs` and enforce them declaratively:

```csharp
builder.Services.AddAuthorizationBuilder()
    .AddPolicy("RequireAdmin", policy => policy.RequireRole(AppRoles.Admin))
    .AddPolicy("RequireManager", policy => policy.RequireRole(AppRoles.Manager, AppRoles.Admin))
    .AddPolicy("RequireAuthenticated", policy => policy.RequireAuthenticatedUser());
```

Apply to pages:

```razor
@page "/admin/users"
@attribute [Authorize(Policy = "RequireAdmin")]

<MudText Typo="Typo.h4">User Management</MudText>
```

Apply to components conditionally:

```razor
<AuthorizeView Policy="RequireManager">
    <Authorized>
        <MudButton OnClick="@DeleteProject" Color="Color.Error">Delete</MudButton>
    </Authorized>
</AuthorizeView>
```

### Two-Factor Authentication

Enable 2FA with TOTP (Time-based One-Time Password):

```csharp
builder.Services.AddIdentity<ApplicationUser, IdentityRole>(options =>
{
    options.Tokens.AuthenticatorTokenProvider = TokenOptions.DefaultAuthenticatorProvider;
})
.AddEntityFrameworkStores<AppDbContext>()
.AddDefaultTokenProviders();
```

The Identity scaffolded pages handle the 2FA UI. For custom flows, use `UserManager<ApplicationUser>.GenerateNewAuthenticatorKey()` and `VerifyTwoFactorTokenAsync()`.

---

## 5. MudBlazor Components

### Philosophy

Use MudBlazor for all standard UI. Do NOT build custom components when MudBlazor provides one. Only build custom components for domain-specific UI that MudBlazor cannot express (workflow visualizations, specialized editors, domain-specific widgets).

### Installation

```xml
<!-- .csproj -->
<PackageReference Include="MudBlazor" Version="9.2.*" />
```

```csharp
// Program.cs
builder.Services.AddMudServices(config =>
{
    config.SnackbarConfiguration.PositionClass = Defaults.Classes.Position.BottomRight;
    config.SnackbarConfiguration.PreventDuplicates = true;
    config.SnackbarConfiguration.NewestOnTop = true;
    config.SnackbarConfiguration.ShowCloseIcon = true;
    config.SnackbarConfiguration.VisibleStateDuration = 5000;
});
```

```razor
@* App.razor — add MudBlazor providers *@
<MudThemeProvider />
<MudPopoverProvider />
<MudDialogProvider />
<MudSnackbarProvider />
```

```html
<!-- _Host.cshtml or App.razor <head> section -->
<link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap" rel="stylesheet" />
<link href="_content/MudBlazor/MudBlazor.min.css" rel="stylesheet" />
<script src="_content/MudBlazor/MudBlazor.min.js"></script>
```

### Layout Template

Every app uses `MudLayout` as the root layout:

```razor
@inherits LayoutComponentBase

<MudThemeProvider Theme="@_theme" IsDarkMode="@_isDarkMode" />
<MudPopoverProvider />
<MudDialogProvider />
<MudSnackbarProvider />

<MudLayout>
    <MudAppBar Elevation="1">
        <MudIconButton Icon="@Icons.Material.Filled.Menu"
                       Color="Color.Inherit"
                       Edge="Edge.Start"
                       OnClick="@ToggleDrawer" />
        <MudText Typo="Typo.h6" Class="ml-3">My Application</MudText>
        <MudSpacer />
        <AuthorizeView>
            <Authorized>
                <MudMenu Icon="@Icons.Material.Filled.Person"
                         Color="Color.Inherit">
                    <MudMenuItem Href="/account/manage">Profile</MudMenuItem>
                    <MudMenuItem Href="/account/logout">Logout</MudMenuItem>
                </MudMenu>
            </Authorized>
            <NotAuthorized>
                <MudButton Href="/account/login" Color="Color.Inherit">Login</MudButton>
            </NotAuthorized>
        </AuthorizeView>
    </MudAppBar>

    <MudDrawer @bind-Open="_drawerOpen" ClipMode="DrawerClipMode.Always" Elevation="2">
        <NavMenu />
    </MudDrawer>

    <MudMainContent>
        <MudContainer MaxWidth="MaxWidth.Large" Class="my-4">
            @Body
        </MudContainer>
    </MudMainContent>
</MudLayout>

@code {
    private bool _drawerOpen = true;
    private bool _isDarkMode;
    private readonly MudTheme _theme = new();

    private void ToggleDrawer() => _drawerOpen = !_drawerOpen;
}
```

### Component Inventory (What MudBlazor Gives Us)

| Category | Components | Use For |
|---|---|---|
| **Forms** | MudTextField, MudSelect, MudAutocomplete, MudCheckBox, MudRadio, MudSwitch, MudDatePicker, MudTimePicker, MudFileUpload | All forms |
| **Buttons** | MudButton, MudIconButton, MudFab, MudToggleIconButton, MudButtonGroup | Actions, CTAs, toggles |
| **Feedback** | MudAlert, MudSnackbar, MudProgressLinear, MudProgressCircular, MudSkeleton | Status, loading, notifications |
| **Layout** | MudLayout, MudAppBar, MudDrawer, MudMainContent, MudContainer, MudGrid, MudItem, MudPaper, MudCard | Page structure, content grouping |
| **Navigation** | MudNavMenu, MudNavLink, MudBreadcrumbs, MudTabs, MudPagination, MudLink | Site navigation, breadcrumbs |
| **Data Display** | MudDataGrid, MudTable, MudSimpleTable, MudList, MudTreeView, MudChip, MudAvatar, MudBadge | Lists, grids, trees, status |
| **Overlay** | MudDialog, MudMenu, MudPopover, MudTooltip, MudOverlay | Dialogs, context menus, tooltips |
| **Charts** | MudChart (Line, Bar, Pie, Donut, Stacked) | Data visualization |
| **Utility** | MudDivider, MudSpacer, MudHighlighter, MudElement, MudFocusTrap | Layout helpers |

### Custom Components (Build Only These)

Only build custom components when MudBlazor cannot express the UI:

- **Domain-specific visualizations** — workflow state machines, pipeline stages, custom timelines
- **Composite forms** — multi-step wizards that combine several MudBlazor components with custom orchestration
- **Specialized editors** — code editors, rich text editors, diagram editors

### Theme Customization

Define a custom theme once and apply it globally:

```csharp
private readonly MudTheme _theme = new()
{
    PaletteLight = new PaletteLight
    {
        Primary = "#1976D2",
        Secondary = "#424242",
        AppbarBackground = "#1976D2",
        Background = "#F5F5F5",
        Surface = "#FFFFFF",
        Error = "#D32F2F",
        Success = "#388E3C",
        Warning = "#F57C00",
        Info = "#1976D2"
    },
    PaletteDark = new PaletteDark
    {
        Primary = "#90CAF9",
        Secondary = "#CE93D8",
        AppbarBackground = "#1E1E2E",
        Background = "#1E1E2E",
        Surface = "#2D2D3F",
    },
    Typography = new Typography
    {
        Default = new DefaultTypography
        {
            FontFamily = ["Roboto", "Helvetica", "Arial", "sans-serif"]
        }
    }
};
```

---

## 6. Entity Framework Core Patterns

### DbContext Setup

```csharp
public class AppDbContext : IdentityDbContext<ApplicationUser>
{
    public AppDbContext(DbContextOptions<AppDbContext> options) : base(options) { }

    public DbSet<Project> Projects => Set<Project>();
    public DbSet<TaskItem> Tasks => Set<TaskItem>();
    public DbSet<AuditLog> AuditLogs => Set<AuditLog>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder); // Required for Identity tables

        // Apply all IEntityTypeConfiguration<T> from this assembly
        modelBuilder.ApplyConfigurationsFromAssembly(typeof(AppDbContext).Assembly);
    }

    // EF9: UseSeeding for deterministic seed data
    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        base.OnConfiguring(optionsBuilder);
    }
}
```

### Entity Configuration (Fluent API Only)

Never use data annotations on entities. Use separate configuration classes:

```csharp
public class ProjectConfiguration : IEntityTypeConfiguration<Project>
{
    public void Configure(EntityTypeBuilder<Project> builder)
    {
        builder.ToTable("Projects");

        builder.HasKey(p => p.Id);

        builder.Property(p => p.Name)
            .IsRequired()
            .HasMaxLength(200);

        builder.Property(p => p.Description)
            .HasMaxLength(2000);

        builder.Property(p => p.Status)
            .HasConversion<string>()
            .HasMaxLength(50);

        builder.HasOne(p => p.Owner)
            .WithMany(u => u.Projects)
            .HasForeignKey(p => p.OwnerId)
            .OnDelete(DeleteBehavior.Restrict);

        builder.HasMany(p => p.Tasks)
            .WithOne(t => t.Project)
            .HasForeignKey(t => t.ProjectId)
            .OnDelete(DeleteBehavior.Cascade);

        builder.HasIndex(p => p.Name);
        builder.HasIndex(p => new { p.OwnerId, p.Status });

        // EF9: query filter for soft-delete
        builder.HasQueryFilter(p => !p.IsDeleted);
    }
}
```

### Entity Base Class

Standardize common properties:

```csharp
public abstract class BaseEntity
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
    public DateTimeOffset? UpdatedAt { get; set; }
    public bool IsDeleted { get; set; }

    // Audit fields
    public string? CreatedBy { get; set; }
    public string? UpdatedBy { get; set; }
}
```

### Migrations

Always use CLI-generated migrations:

```bash
# Generate a migration
dotnet ef migrations add DescribeTheChange --project src/MyApp --output-dir Data/Migrations

# Apply migrations
dotnet ef database update --project src/MyApp

# Generate SQL script (for production)
dotnet ef migrations script --idempotent --project src/MyApp --output migrations.sql
```

**Rules:**
- Never edit a migration after it's been committed. Write a new corrective migration.
- Always review generated migration code before committing.
- Use idempotent SQL scripts for production deployments, not `dotnet ef database update`.
- EF9 throws if pending model changes exist when calling `Migrate()` — this is a safety feature, not a bug.

### EF9-Specific Patterns

**UseSeeding / UseAsyncSeeding** — seed data during migration:

```csharp
protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
{
    optionsBuilder.UseSeeding((context, _) =>
    {
        var adminRole = context.Set<IdentityRole>().FirstOrDefault(r => r.Name == "Admin");
        if (adminRole is null)
        {
            context.Set<IdentityRole>().Add(new IdentityRole("Admin"));
            context.SaveChanges();
        }
    });

    optionsBuilder.UseAsyncSeeding(async (context, _, ct) =>
    {
        // Async seeding for larger datasets
        if (!await context.Set<IdentityRole>().AnyAsync(ct))
        {
            context.Set<IdentityRole>().AddRange(
                AppRoles.All.Select(r => new IdentityRole(r)));
            await context.SaveChangesAsync(ct);
        }
    });
}
```

**Compiled Models** — for large models (100+ entity types), generate compiled models to speed up startup:

```bash
dotnet ef dbcontext optimize --project src/MyApp --output-dir Data/CompiledModels
```

```csharp
// Program.cs
builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlServer(connectionString)
           .UseModel(AppDbContextModel.Instance)); // compiled model
```

**Pre-compiled Queries** — EF9 AOT support for performance-critical queries:

```csharp
public static readonly Func<AppDbContext, Guid, Task<Project?>> GetProjectById =
    EF.CompileAsyncQuery((AppDbContext db, Guid id) =>
        db.Projects
            .Include(p => p.Tasks)
            .FirstOrDefault(p => p.Id == id));
```

### Repository Pattern — When and When Not

For most Blazor apps, inject `AppDbContext` directly into services. The DbContext IS the Unit of Work + Repository. Do NOT add a generic repository layer unless:

- You need to swap the data layer (e.g., EF Core to Dapper for reads)
- You have complex cross-cutting concerns (multi-tenant filtering, audit logging)
- The project is large enough (50+ entities) to benefit from abstraction

```csharp
// Good — direct DbContext injection for most apps
public class ProjectService : IProjectService
{
    private readonly AppDbContext _db;

    public ProjectService(AppDbContext db) => _db = db;

    public async Task<List<Project>> GetAllAsync()
        => await _db.Projects.OrderBy(p => p.Name).ToListAsync();

    public async Task<Project?> GetByIdAsync(Guid id)
        => await _db.Projects.Include(p => p.Tasks).FirstOrDefaultAsync(p => p.Id == id);
}
```

---

## 7. MudBlazor Form Patterns

### EditForm + MudBlazor Components

Use `EditForm` with MudBlazor input components and `FluentValidation`:

```razor
@page "/projects/create"
@attribute [Authorize]
@inject IProjectService ProjectService
@inject ISnackbar Snackbar
@inject NavigationManager Navigation

<MudText Typo="Typo.h4" Class="mb-4">Create Project</MudText>

<EditForm Model="@_model" OnValidSubmit="@HandleSubmit">
    <FluentValidationValidator />

    <MudGrid>
        <MudItem xs="12" md="8">
            <MudCard>
                <MudCardContent>
                    <MudTextField @bind-Value="_model.Name"
                                  Label="Project Name"
                                  Required="true"
                                  RequiredError="Project name is required"
                                  For="@(() => _model.Name)"
                                  Immediate="false" />

                    <MudTextField @bind-Value="_model.Description"
                                  Label="Description"
                                  Lines="4"
                                  For="@(() => _model.Description)"
                                  Class="mt-3" />

                    <MudSelect @bind-Value="_model.Priority"
                               Label="Priority"
                               For="@(() => _model.Priority)"
                               Class="mt-3">
                        @foreach (var priority in Enum.GetValues<Priority>())
                        {
                            <MudSelectItem Value="@priority">@priority</MudSelectItem>
                        }
                    </MudSelect>

                    <MudDatePicker @bind-Date="_model.DueDate"
                                   Label="Due Date"
                                   For="@(() => _model.DueDate)"
                                   MinDate="@DateTime.Today"
                                   Class="mt-3" />
                </MudCardContent>

                <MudCardActions>
                    <MudButton ButtonType="ButtonType.Submit"
                               Variant="Variant.Filled"
                               Color="Color.Primary"
                               Disabled="@_submitting">
                        @if (_submitting)
                        {
                            <MudProgressCircular Size="Size.Small" Indeterminate="true" Class="mr-2" />
                        }
                        Create Project
                    </MudButton>
                    <MudButton Href="/projects"
                               Variant="Variant.Text"
                               Class="ml-2">
                        Cancel
                    </MudButton>
                </MudCardActions>
            </MudCard>
        </MudItem>
    </MudGrid>
</EditForm>

@code {
    private readonly CreateProjectModel _model = new();
    private bool _submitting;

    private async Task HandleSubmit()
    {
        _submitting = true;
        try
        {
            await ProjectService.CreateAsync(_model);
            Snackbar.Add("Project created successfully", Severity.Success);
            Navigation.NavigateTo("/projects");
        }
        catch (Exception ex)
        {
            Snackbar.Add($"Error: {ex.Message}", Severity.Error);
        }
        finally
        {
            _submitting = false;
        }
    }
}
```

### FluentValidation Integration

Use FluentValidation instead of DataAnnotations for complex validation:

```csharp
// Install: Blazored.FluentValidation
public class CreateProjectModelValidator : AbstractValidator<CreateProjectModel>
{
    public CreateProjectModelValidator()
    {
        RuleFor(x => x.Name)
            .NotEmpty().WithMessage("Project name is required")
            .MaximumLength(200).WithMessage("Name must be 200 characters or fewer")
            .MustAsync(BeUniqueName).WithMessage("A project with this name already exists");

        RuleFor(x => x.Description)
            .MaximumLength(2000);

        RuleFor(x => x.DueDate)
            .GreaterThanOrEqualTo(DateTime.Today)
            .When(x => x.DueDate.HasValue)
            .WithMessage("Due date cannot be in the past");
    }

    private async Task<bool> BeUniqueName(string name, CancellationToken ct)
    {
        // Injected via DI in real code
        return true; // placeholder
    }
}
```

### MudDataGrid Pattern

For data-heavy pages, use `MudDataGrid` with server-side pagination:

```razor
<MudDataGrid T="Project"
             ServerData="@LoadServerData"
             Filterable="true"
             SortMode="SortMode.Multiple"
             Hideable="true"
             Loading="@_loading">
    <Columns>
        <PropertyColumn Property="x => x.Name" Title="Name" />
        <PropertyColumn Property="x => x.Status" Title="Status">
            <CellTemplate>
                <MudChip T="string"
                         Color="@GetStatusColor(context.Item.Status)"
                         Size="Size.Small">
                    @context.Item.Status
                </MudChip>
            </CellTemplate>
        </PropertyColumn>
        <PropertyColumn Property="x => x.DueDate" Title="Due" Format="yyyy-MM-dd" />
        <PropertyColumn Property="x => x.Owner.FullName" Title="Owner" />
        <TemplateColumn Title="Actions" Sortable="false" Filterable="false">
            <CellTemplate>
                <MudIconButton Icon="@Icons.Material.Filled.Edit"
                               Size="Size.Small"
                               Href="@($"/projects/{context.Item.Id}/edit")" />
                <MudIconButton Icon="@Icons.Material.Filled.Delete"
                               Size="Size.Small"
                               Color="Color.Error"
                               OnClick="@(() => ConfirmDelete(context.Item))" />
            </CellTemplate>
        </TemplateColumn>
    </Columns>
    <PagerContent>
        <MudDataGridPager T="Project" />
    </PagerContent>
</MudDataGrid>

@code {
    private bool _loading;

    private async Task<GridData<Project>> LoadServerData(GridState<Project> state)
    {
        _loading = true;
        try
        {
            var (items, totalCount) = await ProjectService.GetPagedAsync(
                state.Page,
                state.PageSize,
                state.SortDefinitions,
                state.FilterDefinitions);

            return new GridData<Project> { Items = items, TotalItems = totalCount };
        }
        finally
        {
            _loading = false;
        }
    }
}
```

### MudDialog Pattern

Use `MudDialog` for confirmations and inline editing:

```razor
@* ConfirmDeleteDialog.razor *@
<MudDialog>
    <DialogContent>
        <MudText>
            Are you sure you want to delete <strong>@ProjectName</strong>?
            This action cannot be undone.
        </MudText>
    </DialogContent>
    <DialogActions>
        <MudButton OnClick="Cancel">Cancel</MudButton>
        <MudButton Color="Color.Error"
                   Variant="Variant.Filled"
                   OnClick="Confirm">
            Delete
        </MudButton>
    </DialogActions>
</MudDialog>

@code {
    [CascadingParameter] private MudDialogInstance MudDialog { get; set; } = null!;
    [Parameter] public string ProjectName { get; set; } = string.Empty;

    private void Cancel() => MudDialog.Cancel();
    private void Confirm() => MudDialog.Close(DialogResult.Ok(true));
}
```

Invoke from a parent component:

```csharp
private async Task ConfirmDelete(Project project)
{
    var parameters = new DialogParameters
    {
        { nameof(ConfirmDeleteDialog.ProjectName), project.Name }
    };

    var dialog = await DialogService.ShowAsync<ConfirmDeleteDialog>("Confirm Delete", parameters);
    var result = await dialog.Result;

    if (!result.Canceled)
    {
        await ProjectService.DeleteAsync(project.Id);
        Snackbar.Add("Project deleted", Severity.Success);
    }
}
```

---

## 8. Service Layer Patterns

### Service Interface Convention

Every service has an interface for testability:

```csharp
public interface IProjectService
{
    Task<List<Project>> GetAllAsync();
    Task<Project?> GetByIdAsync(Guid id);
    Task<Project> CreateAsync(CreateProjectModel model);
    Task UpdateAsync(Guid id, UpdateProjectModel model);
    Task DeleteAsync(Guid id);
    Task<(List<Project> Items, int TotalCount)> GetPagedAsync(
        int page, int pageSize,
        ICollection<SortDefinition<Project>> sorts,
        ICollection<IFilterDefinition<Project>> filters);
}
```

### Service Registration

Register services via extension methods, not inline in `Program.cs`:

```csharp
// Extensions/ServiceCollectionExtensions.cs
public static class ServiceCollectionExtensions
{
    public static IServiceCollection AddApplicationServices(this IServiceCollection services)
    {
        services.AddScoped<IProjectService, ProjectService>();
        services.AddScoped<IEmailService, EmailService>();
        services.AddScoped<IAuditService, AuditService>();
        return services;
    }

    public static IServiceCollection AddInfrastructure(
        this IServiceCollection services,
        IConfiguration configuration)
    {
        services.AddDbContext<AppDbContext>(options =>
            options.UseSqlServer(configuration.GetConnectionString("Default")));

        services.AddIdentity<ApplicationUser, IdentityRole>()
            .AddEntityFrameworkStores<AppDbContext>()
            .AddDefaultTokenProviders();

        return services;
    }
}

// Program.cs — clean and readable
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddRazorComponents()
    .AddInteractiveServerComponents()
    .AddInteractiveWebAssemblyComponents();
builder.Services.AddMudServices();
builder.Services.AddInfrastructure(builder.Configuration);
builder.Services.AddApplicationServices();
```

### Error Handling in Services

Services return `Result<T>` for operations that can fail, throw only for programmer errors:

```csharp
public class Result<T>
{
    public bool IsSuccess { get; }
    public T? Value { get; }
    public string? Error { get; }

    private Result(T value) { IsSuccess = true; Value = value; }
    private Result(string error) { IsSuccess = false; Error = error; }

    public static Result<T> Success(T value) => new(value);
    public static Result<T> Failure(string error) => new(error);
}
```

```csharp
public async Task<Result<Project>> CreateAsync(CreateProjectModel model)
{
    if (await _db.Projects.AnyAsync(p => p.Name == model.Name))
        return Result<Project>.Failure("A project with this name already exists");

    var project = new Project
    {
        Name = model.Name,
        Description = model.Description,
        OwnerId = _currentUser.Id
    };

    _db.Projects.Add(project);
    await _db.SaveChangesAsync();

    return Result<Project>.Success(project);
}
```

---

## 9. Testing Patterns

### Test Pyramid (Blazor-specific)

```
        /\
       /  \          E2E (Playwright) — full browser, critical user journeys
      /    \
     /------\
    /        \        Component Tests (bUnit)
   /          \       Blazor components, form validation, UI state
  /------------\
 /              \      Integration Tests (xUnit + EF in-memory/TestContainers)
/                \     Services through DB, auth flows, API endpoints
/------------------\
/                    \   Unit Tests (xUnit)
/                      \  Pure functions, validators, mappers, business rules
/------------------------\
```

### xUnit Unit Tests

```csharp
public class CreateProjectModelValidatorTests
{
    private readonly CreateProjectModelValidator _validator = new();

    [Fact]
    public async Task Name_WhenEmpty_ShouldHaveValidationError()
    {
        var model = new CreateProjectModel { Name = "" };
        var result = await _validator.ValidateAsync(model);

        result.ShouldHaveValidationErrorFor(x => x.Name)
              .WithErrorMessage("Project name is required");
    }

    [Theory]
    [InlineData("Valid Name", true)]
    [InlineData("", false)]
    [InlineData(null, false)]
    public async Task Name_Validation(string? name, bool shouldBeValid)
    {
        var model = new CreateProjectModel { Name = name! };
        var result = await _validator.ValidateAsync(model);

        Assert.Equal(shouldBeValid, result.IsValid || !result.Errors.Any(e => e.PropertyName == "Name"));
    }
}
```

### Integration Tests with EF Core

Use a real database via TestContainers (preferred) or the EF in-memory provider (fast but limited):

```csharp
// Using WebApplicationFactory for full integration tests
public class ProjectServiceTests : IClassFixture<WebApplicationFactory<Program>>, IAsyncLifetime
{
    private readonly WebApplicationFactory<Program> _factory;
    private AsyncServiceScope _scope;
    private AppDbContext _db = null!;
    private IProjectService _service = null!;

    public ProjectServiceTests(WebApplicationFactory<Program> factory)
    {
        _factory = factory.WithWebHostBuilder(builder =>
        {
            builder.ConfigureServices(services =>
            {
                // Replace real DB with in-memory for tests
                services.RemoveAll<DbContextOptions<AppDbContext>>();
                services.AddDbContext<AppDbContext>(options =>
                    options.UseInMemoryDatabase($"TestDb_{Guid.NewGuid()}"));
            });
        });
    }

    public async Task InitializeAsync()
    {
        _scope = _factory.Services.CreateAsyncScope();
        _db = _scope.ServiceProvider.GetRequiredService<AppDbContext>();
        _service = _scope.ServiceProvider.GetRequiredService<IProjectService>();
        await _db.Database.EnsureCreatedAsync();
    }

    public async Task DisposeAsync()
    {
        await _db.Database.EnsureDeletedAsync();
        await _scope.DisposeAsync();
    }

    [Fact]
    public async Task CreateAsync_WithValidModel_ReturnsProject()
    {
        var model = new CreateProjectModel
        {
            Name = "Test Project",
            Description = "A test project"
        };

        var result = await _service.CreateAsync(model);

        Assert.True(result.IsSuccess);
        Assert.Equal("Test Project", result.Value!.Name);
        Assert.Single(await _db.Projects.ToListAsync());
    }
}
```

### bUnit Component Tests

bUnit tests render Blazor components in isolation:

```csharp
public class HomePageTests : BunitContext
{
    [Fact]
    public void Home_RendersWelcomeMessage()
    {
        // Arrange — register required services
        Services.AddSingleton<IProjectService>(
            Substitute.For<IProjectService>()); // NSubstitute mock

        // Act
        var cut = Render<Home>();

        // Assert — semantic HTML comparison
        cut.Find("h3").TextContent.Should().Contain("Welcome");
    }

    [Fact]
    public void Home_WhenAuthenticated_ShowsDashboardLink()
    {
        Services.AddSingleton<IProjectService>(Substitute.For<IProjectService>());

        // Set up authentication context
        var authContext = this.AddTestAuthorization();
        authContext.SetAuthorized("testuser@example.com");
        authContext.SetRoles("User");

        var cut = Render<Home>();

        cut.Find("a[href='/dashboard']").Should().NotBeNull();
    }
}
```

### bUnit with MudBlazor

MudBlazor components require additional setup in bUnit:

```csharp
public class MudBlazorTestBase : BunitContext
{
    public MudBlazorTestBase()
    {
        // Register MudBlazor services
        Services.AddMudServices();

        // Add MudBlazor providers to the render tree
        // MudPopoverProvider is required for menus, tooltips, etc.
        JSInterop.SetupVoid("mudPopover.initialize", _ => true);
        JSInterop.SetupVoid("mudKeyInterceptor.connect", _ => true);
        JSInterop.SetupVoid("mudElementReference.saveFocus", _ => true);
        JSInterop.SetupVoid("mudScrollManager.scrollTo", _ => true);
        JSInterop.Mode = JSRuntimeMode.Loose; // Allow unmatched JS calls
    }
}
```

```csharp
public class CreateProjectFormTests : MudBlazorTestBase
{
    [Fact]
    public void Form_SubmitWithEmptyName_ShowsValidationError()
    {
        var mockService = Substitute.For<IProjectService>();
        Services.AddSingleton(mockService);

        var cut = Render<CreateProjectPage>();

        // Find and click submit button
        var submitButton = cut.Find("button[type='submit']");
        submitButton.Click();

        // Verify validation error appears
        cut.Find(".mud-input-error").TextContent.Should().Contain("required");
    }

    [Fact]
    public async Task Form_SubmitWithValidData_CallsService()
    {
        var mockService = Substitute.For<IProjectService>();
        mockService.CreateAsync(Arg.Any<CreateProjectModel>())
            .Returns(Result<Project>.Success(new Project { Name = "Test" }));
        Services.AddSingleton(mockService);

        var navManager = Services.GetRequiredService<FakeNavigationManager>();

        var cut = Render<CreateProjectPage>();

        // Fill in the form
        cut.Find("input[label='Project Name']").Change("Test Project");

        // Submit
        cut.Find("button[type='submit']").Click();

        // Verify service was called
        await mockService.Received(1).CreateAsync(
            Arg.Is<CreateProjectModel>(m => m.Name == "Test Project"));
    }
}
```

### Playwright End-to-End Tests

For critical user journeys, use Playwright:

```csharp
// PlaywrightFixture.cs — shared test fixture
public class PlaywrightFixture : IAsyncLifetime
{
    public IPlaywright Playwright { get; private set; } = null!;
    public IBrowser Browser { get; private set; } = null!;
    public string BaseUrl { get; } = "https://localhost:5001";

    public async Task InitializeAsync()
    {
        Playwright = await Microsoft.Playwright.Playwright.CreateAsync();
        Browser = await Playwright.Chromium.LaunchAsync(new BrowserTypeLaunchOptions
        {
            Headless = true
        });
    }

    public async Task DisposeAsync()
    {
        await Browser.DisposeAsync();
        Playwright.Dispose();
    }
}
```

```csharp
public class LoginPageTests : IClassFixture<PlaywrightFixture>
{
    private readonly PlaywrightFixture _fixture;

    public LoginPageTests(PlaywrightFixture fixture) => _fixture = fixture;

    [Fact]
    public async Task Login_WithValidCredentials_RedirectsToDashboard()
    {
        var page = await _fixture.Browser.NewPageAsync();
        await page.GotoAsync($"{_fixture.BaseUrl}/Account/Login");

        // Fill login form
        await page.FillAsync("input[name='Email']", "admin@example.com");
        await page.FillAsync("input[name='Password']", "TestPassword123!");
        await page.ClickAsync("button[type='submit']");

        // Wait for navigation
        await page.WaitForURLAsync("**/dashboard");

        // Verify dashboard loaded
        var heading = await page.TextContentAsync("h4");
        Assert.Contains("Dashboard", heading);
    }

    [Fact]
    public async Task Login_WithInvalidCredentials_ShowsError()
    {
        var page = await _fixture.Browser.NewPageAsync();
        await page.GotoAsync($"{_fixture.BaseUrl}/Account/Login");

        await page.FillAsync("input[name='Email']", "wrong@example.com");
        await page.FillAsync("input[name='Password']", "WrongPassword!");
        await page.ClickAsync("button[type='submit']");

        // Should stay on login page with error
        var error = await page.TextContentAsync(".mud-alert-text");
        Assert.Contains("Invalid login attempt", error);
    }
}
```

### Test Configuration

```csharp
// appsettings.Testing.json
{
  "ConnectionStrings": {
    "Default": "Server=(localdb)\\mssqllocaldb;Database=MyApp_Test;Trusted_Connection=True"
  },
  "Logging": {
    "LogLevel": {
      "Default": "Warning"
    }
  }
}
```

### Test Data Builders

Use the Builder pattern for test entities:

```csharp
public class TestDataBuilder
{
    private readonly AppDbContext _db;

    public TestDataBuilder(AppDbContext db) => _db = db;

    public ProjectBuilder Project() => new(_db);
    public UserBuilder User() => new(_db);

    public class ProjectBuilder
    {
        private readonly AppDbContext _db;
        private string _name = $"Project-{Guid.NewGuid():N[..8]}";
        private string _description = "Test project";
        private ProjectStatus _status = ProjectStatus.Active;
        private string? _ownerId;

        internal ProjectBuilder(AppDbContext db) => _db = db;

        public ProjectBuilder WithName(string name) { _name = name; return this; }
        public ProjectBuilder WithStatus(ProjectStatus status) { _status = status; return this; }
        public ProjectBuilder OwnedBy(string userId) { _ownerId = userId; return this; }

        public async Task<Project> BuildAsync()
        {
            var project = new Project
            {
                Name = _name,
                Description = _description,
                Status = _status,
                OwnerId = _ownerId ?? (await new UserBuilder(_db).BuildAsync()).Id
            };
            _db.Projects.Add(project);
            await _db.SaveChangesAsync();
            return project;
        }
    }
}
```

---

## 10. Seed Data

### Seeding Strategy

EF9 introduced `UseSeeding` and `UseAsyncSeeding` for database seeding. Use these for system data (roles, permissions, default settings). Use a separate seed script for development/demo data.

```csharp
// System data — seeded via UseAsyncSeeding (runs on every migration)
protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
{
    optionsBuilder.UseAsyncSeeding(async (context, _, ct) =>
    {
        // Roles
        var roleStore = context.Set<IdentityRole>();
        foreach (var role in AppRoles.All)
        {
            if (!await roleStore.AnyAsync(r => r.Name == role, ct))
            {
                roleStore.Add(new IdentityRole(role) { NormalizedName = role.ToUpperInvariant() });
            }
        }
        await context.SaveChangesAsync(ct);
    });
}
```

```csharp
// Development seed data — run explicitly via CLI
// dotnet run --project src/MyApp -- --seed
public static class DevelopmentSeeder
{
    public static async Task SeedAsync(IServiceProvider services)
    {
        using var scope = services.CreateScope();
        var db = scope.ServiceProvider.GetRequiredService<AppDbContext>();
        var userManager = scope.ServiceProvider.GetRequiredService<UserManager<ApplicationUser>>();

        // Create admin user
        if (await userManager.FindByEmailAsync("admin@example.com") is null)
        {
            var admin = new ApplicationUser
            {
                UserName = "admin@example.com",
                Email = "admin@example.com",
                FirstName = "Admin",
                LastName = "User",
                EmailConfirmed = true
            };
            await userManager.CreateAsync(admin, "Admin123!@#");
            await userManager.AddToRoleAsync(admin, AppRoles.Admin);
        }

        // Create sample projects
        if (!await db.Projects.AnyAsync())
        {
            db.Projects.AddRange(
                new Project { Name = "Website Redesign", Status = ProjectStatus.Active },
                new Project { Name = "Mobile App", Status = ProjectStatus.Planning },
                new Project { Name = "API Integration", Status = ProjectStatus.Completed }
            );
            await db.SaveChangesAsync();
        }
    }
}
```

Seed data is idempotent — safe to run multiple times. Check for existence before inserting.

---

## 11. Development Workflow

### Feature Development Cycle (Blazor-specific)

```
1. Define requirements (user stories, acceptance criteria)
2. Design test levels (unit / integration / bUnit / E2E)
3. Write failing tests (xUnit + bUnit)
4. Write entity + EF configuration + migration
5. Write service layer (interface + implementation)
6. Write Blazor component (using MudBlazor)
7. Run: dotnet test
8. Run: dotnet format
9. Refactor while green
10. Run: dotnet build --warnaserror
```

### Common Commands

```bash
# Development
dotnet run --project src/MyApp                          # Start dev server
dotnet watch --project src/MyApp                        # Hot reload dev server
dotnet test                                             # Run all tests
dotnet test --filter "FullyQualifiedName~ProjectService" # Run specific tests
dotnet format                                           # Format code

# Entity Framework
dotnet ef migrations add DescribeChange --project src/MyApp
dotnet ef database update --project src/MyApp
dotnet ef migrations script --idempotent --project src/MyApp

# Quality
dotnet format --verify-no-changes                       # CI format check
dotnet build --warnaserror                              # Zero warnings
dotnet list package --vulnerable                        # CVE check
dotnet list package --outdated                          # Dependency freshness

# Playwright (first run)
pwsh bin/Debug/net9.0/playwright.ps1 install            # Install browsers

# Docker
docker build -t myapp -f docker/Dockerfile .
docker compose -f docker/docker-compose.yml up
```

### Directory.Build.props

Shared build properties for all projects in the solution:

```xml
<Project>
  <PropertyGroup>
    <TargetFramework>net9.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
    <AnalysisLevel>latest-recommended</AnalysisLevel>
    <EnforceCodeStyleInBuild>true</EnforceCodeStyleInBuild>
  </PropertyGroup>
</Project>
```

### .editorconfig

Enforce consistent code style:

```ini
[*.cs]
# Naming conventions
dotnet_naming_rule.private_fields_should_be_camel_case.severity = error
dotnet_naming_rule.private_fields_should_be_camel_case.symbols = private_fields
dotnet_naming_rule.private_fields_should_be_camel_case.style = camel_case_with_underscore

dotnet_naming_symbols.private_fields.applicable_kinds = field
dotnet_naming_symbols.private_fields.applicable_accessibilities = private

dotnet_naming_style.camel_case_with_underscore.capitalization = camel_case
dotnet_naming_style.camel_case_with_underscore.required_prefix = _

# Formatting
csharp_new_line_before_open_brace = all
csharp_indent_case_contents = true
csharp_indent_switch_labels = true

# var preferences
csharp_style_var_for_built_in_types = false:suggestion
csharp_style_var_when_type_is_apparent = true:suggestion
```

---

## 12. Deployment

### Docker Multi-Stage Build

```dockerfile
# docker/Dockerfile
# Stage 1: Build
FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
WORKDIR /src

# Copy csproj files and restore (layer caching)
COPY ["src/MyApp/MyApp.csproj", "src/MyApp/"]
COPY ["src/MyApp.Client/MyApp.Client.csproj", "src/MyApp.Client/"]
COPY ["global.json", "."]
RUN dotnet restore "src/MyApp/MyApp.csproj"

# Copy everything and publish
COPY . .
RUN dotnet publish "src/MyApp/MyApp.csproj" \
    -c Release \
    -o /app/publish \
    --no-restore \
    /p:UseAppHost=false

# Stage 2: Runtime
FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS runtime
WORKDIR /app

# Security: run as non-root
RUN groupadd -r appgroup && useradd -r -g appgroup -d /app -s /sbin/nologin appuser
USER appuser

COPY --from=build /app/publish .

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

EXPOSE 8080
ENV ASPNETCORE_URLS=http://+:8080
ENV ASPNETCORE_ENVIRONMENT=Production
ENV DOTNET_EnableDiagnostics=0

ENTRYPOINT ["dotnet", "MyApp.dll"]
```

### Docker Compose

```yaml
# docker/docker-compose.yml
services:
  app:
    build:
      context: ..
      dockerfile: docker/Dockerfile
    ports:
      - "8080:8080"
    environment:
      - ASPNETCORE_ENVIRONMENT=Production
      - ConnectionStrings__Default=Server=db;Database=MyApp;User=sa;Password=${DB_PASSWORD};TrustServerCertificate=true
    depends_on:
      db:
        condition: service_healthy
    restart: unless-stopped

  db:
    image: mcr.microsoft.com/mssql/server:2022-latest
    environment:
      - ACCEPT_EULA=Y
      - MSSQL_SA_PASSWORD=${DB_PASSWORD}
    ports:
      - "1433:1433"
    volumes:
      - sqldata:/var/opt/mssql
    healthcheck:
      test: /opt/mssql-tools18/bin/sqlcmd -S localhost -U sa -P "$$MSSQL_SA_PASSWORD" -Q "SELECT 1" -C
      interval: 10s
      timeout: 5s
      retries: 5

volumes:
  sqldata:
```

### CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
name: CI/CD

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      sqlserver:
        image: mcr.microsoft.com/mssql/server:2022-latest
        env:
          ACCEPT_EULA: Y
          MSSQL_SA_PASSWORD: TestPassword123!
        ports:
          - 1433:1433

    steps:
      - uses: actions/checkout@v4

      - name: Setup .NET
        uses: actions/setup-dotnet@v4
        with:
          global-json-file: global.json

      - name: Restore
        run: dotnet restore

      - name: Format check
        run: dotnet format --verify-no-changes

      - name: Build (zero warnings)
        run: dotnet build --no-restore --warnaserror

      - name: Test with coverage
        run: dotnet test --no-build --collect:"XPlat Code Coverage" --results-directory ./coverage

      - name: Vulnerability check
        run: dotnet list package --vulnerable --include-transitive 2>&1 | tee vuln.txt && ! grep -q "has the following vulnerable packages" vuln.txt

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build and push Docker image
        run: |
          docker build -t myapp:${{ github.sha }} -f docker/Dockerfile .
          # Push to your container registry
```

### Health Check Endpoint

```csharp
// Program.cs
builder.Services.AddHealthChecks()
    .AddDbContextCheck<AppDbContext>("database")
    .AddCheck("self", () => HealthCheckResult.Healthy());

// After app.Build()
app.MapHealthChecks("/health", new HealthCheckOptions
{
    ResponseWriter = UIResponseWriter.WriteHealthCheckUIResponse
});
```

### Production Configuration

```csharp
// Program.cs — production-specific middleware
if (app.Environment.IsProduction())
{
    app.UseExceptionHandler("/Error");
    app.UseHsts();
    app.UseResponseCompression();
}

app.UseHttpsRedirection();
app.UseStaticFiles(new StaticFileOptions
{
    OnPrepareResponse = ctx =>
    {
        ctx.Context.Response.Headers.Append("Cache-Control", "public,max-age=31536000,immutable");
    }
});
app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();
app.UseAntiforgery();
```

### EF Migration in Production

Never run `dotnet ef database update` in production. Use one of:

1. **Idempotent SQL scripts** (preferred):
```bash
dotnet ef migrations script --idempotent --project src/MyApp -o migrations.sql
# Apply via your DB tooling / CI pipeline
```

2. **Migration bundle** (self-contained executable):
```bash
dotnet ef migrations bundle --project src/MyApp --output efbundle
# Run in production: ./efbundle --connection "your-connection-string"
```

3. **Startup migration** (small apps only):
```csharp
// Program.cs — auto-migrate on startup (NOT recommended for large/team projects)
using (var scope = app.Services.CreateScope())
{
    var db = scope.ServiceProvider.GetRequiredService<AppDbContext>();
    await db.Database.MigrateAsync();
}
```

---

## 13. Security

### Antiforgery

Blazor in .NET 9 enables antiforgery protection automatically. When you call `AddRazorComponents()`, antiforgery services are registered. When you call `UseAntiforgery()`, the middleware validates tokens.

```csharp
// Program.cs — middleware order matters
app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();
app.UseAntiforgery(); // Must be after UseAuthorization

app.MapRazorComponents<App>()
    .AddInteractiveServerRenderMode()
    .AddInteractiveWebAssemblyRenderMode();
```

**Key rules:**
- `EditForm` automatically includes the antiforgery token — no manual work needed
- Static SSR forms MUST include `<AntiforgeryToken />` if not using `EditForm`
- API endpoints that accept form POSTs from non-Blazor clients need explicit token validation
- The `[RequireAntiforgeryToken]` attribute is applied automatically to Razor component endpoints

### CORS Policy

```csharp
builder.Services.AddCors(options =>
{
    options.AddPolicy("ApiPolicy", policy =>
    {
        policy.WithOrigins(
                "https://yourdomain.com",
                "https://app.yourdomain.com")
            .AllowAnyHeader()
            .AllowAnyMethod()
            .AllowCredentials();
    });

    // Never do this in production:
    // policy.AllowAnyOrigin().AllowAnyHeader().AllowAnyMethod();
});

// Program.cs — apply CORS before auth
app.UseCors("ApiPolicy");
app.UseAuthentication();
app.UseAuthorization();
```

### Security Headers

Every response includes security headers via middleware:

```csharp
// Extensions/MiddlewareExtensions.cs
public static class MiddlewareExtensions
{
    public static IApplicationBuilder UseSecurityHeaders(this IApplicationBuilder app)
    {
        return app.Use(async (context, next) =>
        {
            var headers = context.Response.Headers;

            headers.Append("X-Content-Type-Options", "nosniff");
            headers.Append("X-Frame-Options", "DENY");
            headers.Append("X-XSS-Protection", "0"); // Disabled — CSP is the modern protection
            headers.Append("Referrer-Policy", "strict-origin-when-cross-origin");
            headers.Append("Permissions-Policy",
                "accelerometer=(), camera=(), geolocation=(), gyroscope=(), " +
                "magnetometer=(), microphone=(), payment=(), usb=()");
            headers.Append("Strict-Transport-Security",
                "max-age=31536000; includeSubDomains; preload");

            // CSP for Blazor — must allow inline styles for MudBlazor
            // and wss: for SignalR WebSocket
            headers.Append("Content-Security-Policy",
                "default-src 'self'; " +
                "script-src 'self' 'unsafe-inline' 'unsafe-eval'; " +
                "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; " +
                "font-src 'self' https://fonts.gstatic.com; " +
                "img-src 'self' data:; " +
                "connect-src 'self' wss:; " +
                "frame-ancestors 'none'");

            await next();
        });
    }
}

// Program.cs
app.UseSecurityHeaders();
```

**Notes:**
- `unsafe-inline` and `unsafe-eval` in `script-src` are currently required for Blazor's JS interop and MudBlazor. This is a known limitation.
- `unsafe-inline` in `style-src` is required for MudBlazor's dynamic styling.
- `wss:` in `connect-src` is required for Blazor Server's SignalR WebSocket connection.
- `frame-ancestors 'none'` prevents clickjacking (equivalent to `X-Frame-Options: DENY`).

### Authentication Security Checklist

| Control | Implementation |
|---|---|
| Password hashing | ASP.NET Identity uses PBKDF2 by default (configurable to Argon2id) |
| Account lockout | Configured via `IdentityOptions.Lockout` (15 min, 5 attempts) |
| Email confirmation | `options.SignIn.RequireConfirmedEmail = true` |
| 2FA support | TOTP via `AddDefaultTokenProviders()` |
| Cookie security | `HttpOnly`, `Secure`, `SameSite=Strict` |
| HTTPS enforcement | `UseHttpsRedirection()` + HSTS header |
| Session timeout | `ExpireTimeSpan` + `SlidingExpiration` on cookie |
| CSRF protection | Automatic via `UseAntiforgery()` |
| Input validation | FluentValidation + server-side checks (never trust client) |

### Secrets Management

```bash
# Development: use User Secrets (never commit secrets to source control)
dotnet user-secrets init --project src/MyApp
dotnet user-secrets set "ConnectionStrings:Default" "Server=...;Password=..."
dotnet user-secrets set "Smtp:ApiKey" "SG.xxxx"

# Production: use environment variables or a vault
# Docker: pass via docker-compose environment or Docker secrets
# Azure: use Azure Key Vault
# AWS: use AWS Secrets Manager
```

```csharp
// Program.cs — secret sources in priority order
builder.Configuration
    .AddJsonFile("appsettings.json", optional: false)
    .AddJsonFile($"appsettings.{builder.Environment.EnvironmentName}.json", optional: true)
    .AddUserSecrets<Program>(optional: true) // dev only
    .AddEnvironmentVariables();              // production override
```

---

## 14. Coverage Enforcement

### Coverlet Configuration

```xml
<!-- tests/MyApp.Tests/MyApp.Tests.csproj -->
<PackageReference Include="coverlet.collector" Version="6.0.*">
    <PrivateAssets>all</PrivateAssets>
    <IncludeAssets>runtime; build; native; contentfiles; analyzers</IncludeAssets>
</PackageReference>
<PackageReference Include="coverlet.msbuild" Version="6.0.*">
    <PrivateAssets>all</PrivateAssets>
    <IncludeAssets>runtime; build; native; contentfiles; analyzers</IncludeAssets>
</PackageReference>
```

### Coverage Commands

```bash
# Terminal report
dotnet test --collect:"XPlat Code Coverage" \
    --results-directory ./coverage \
    -- DataCollectionRunSettings.DataCollectors.DataCollector.Configuration.Format=cobertura

# Generate HTML report (install: dotnet tool install -g dotnet-reportgenerator-globaltool)
reportgenerator \
    -reports:coverage/**/coverage.cobertura.xml \
    -targetdir:coverage/report \
    -reporttypes:Html

# Enforce minimum coverage in CI
reportgenerator \
    -reports:coverage/**/coverage.cobertura.xml \
    -targetdir:coverage/report \
    -reporttypes:TextSummary

# Parse and enforce threshold
COVERAGE=$(grep "Line coverage" coverage/report/Summary.txt | grep -oP '\d+\.?\d*')
if (( $(echo "$COVERAGE < 80" | bc -l) )); then
    echo "Coverage $COVERAGE% is below 80% threshold"
    exit 1
fi
```

### Exclusions

Exclude generated code, migrations, and Program.cs from coverage:

```xml
<!-- Directory.Build.props -->
<PropertyGroup>
  <ExcludeFromCodeCoverage Condition="'$(Configuration)' == 'Release'">true</ExcludeFromCodeCoverage>
</PropertyGroup>
```

```csharp
// Exclude specific classes
[ExcludeFromCodeCoverage]
public static class Program { /* ... */ }

// Exclude migrations (they're auto-generated)
// coverlet.runsettings
```

```xml
<!-- coverlet.runsettings -->
<?xml version="1.0" encoding="utf-8" ?>
<RunSettings>
  <DataCollectionRunSettings>
    <DataCollectors>
      <DataCollector friendlyName="XPlat Code Coverage">
        <Configuration>
          <Format>cobertura</Format>
          <Exclude>
            [MyApp]MyApp.Data.Migrations.*,
            [MyApp]MyApp.Program,
            [MyApp]MyApp.Extensions.*
          </Exclude>
          <ExcludeByAttribute>
            ExcludeFromCodeCoverage,GeneratedCodeAttribute,CompilerGeneratedAttribute
          </ExcludeByAttribute>
        </Configuration>
      </DataCollector>
    </DataCollectors>
  </DataCollectionRunSettings>
</RunSettings>
```

Target is 100% (per CLAUDE.md core rules). The `minimum_coverage` threshold is the hard gate — CI fails below it.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with `MudCard` sections |
| **labels** | MudBlazor `Label` prop on all inputs, optional fields marked "(optional)" |
| **validation** | FluentValidation, server-side always, client-side for UX |
| **errors** | MudBlazor inline errors via `For` prop + `MudAlert` for form-level errors |
| **accessibility** | `aria-label` where needed, focus management on errors, keyboard navigation |
| **mobile** | MudBlazor is responsive by default, verify touch targets >= 48px |
| **cta** | Outcome-focused text ("Create Project" not "Submit"), loading state with `MudProgressCircular` |
| **trust** | Minimal fields, "(optional)" markers, post-submit confirmation via `MudSnackbar` |
| **performance** | Use `Immediate="false"` on MudTextField (validates on blur, not keystroke) |

**Blazor-specific form pattern:**

```razor
<EditForm Model="@_model" OnValidSubmit="@HandleSubmit">
    <FluentValidationValidator />

    <MudCard>
        <MudCardContent>
            @* Group related fields *@
            <MudText Typo="Typo.h6" Class="mb-3">Contact Information</MudText>

            <MudTextField @bind-Value="_model.FullName"
                          Label="Full Name"
                          Required="true"
                          For="@(() => _model.FullName)"
                          Immediate="false"
                          autocomplete="name" />

            <MudTextField @bind-Value="_model.Email"
                          Label="Email Address"
                          Required="true"
                          InputType="InputType.Email"
                          For="@(() => _model.Email)"
                          Immediate="false"
                          autocomplete="email"
                          Class="mt-3" />

            <MudTextField @bind-Value="_model.Phone"
                          Label="Phone (optional)"
                          InputType="InputType.Telephone"
                          For="@(() => _model.Phone)"
                          Immediate="false"
                          autocomplete="tel"
                          Class="mt-3" />
        </MudCardContent>

        <MudCardActions>
            <MudButton ButtonType="ButtonType.Submit"
                       Variant="Variant.Filled"
                       Color="Color.Primary"
                       Disabled="@_submitting"
                       Class="ml-auto">
                @if (_submitting)
                {
                    <MudProgressCircular Size="Size.Small"
                                         Indeterminate="true"
                                         Class="mr-2" />
                }
                Reserve My Free Visit
            </MudButton>
        </MudCardActions>
    </MudCard>
</EditForm>
```

**Key conventions:**
- Always use `For="@(() => _model.Property)"` on every input — this enables MudBlazor's automatic error display
- Use `Immediate="false"` for short forms (validates on blur/submit, not every keystroke)
- Use `Required="true"` for required fields — MudBlazor shows the asterisk indicator
- Always provide `autocomplete` attributes for better mobile UX and accessibility
- Use `InputType` enum to set correct mobile keyboard (Email, Telephone, Password)
- Show loading state on submit button with `MudProgressCircular` inside the button
- Use outcome-focused CTA text: "Create Account", "Save Changes", "Reserve Visit"

---

## 16. Anti-Patterns (Blazor-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Building custom modal/dialog components | Use `MudDialog` / `MudDialogService` — handles overlay, focus trap, ESC key, backdrop |
| 2 | Building custom data tables | Use `MudDataGrid` with server-side pagination — handles sorting, filtering, paging |
| 3 | Building custom form inputs | Use MudBlazor form components (`MudTextField`, `MudSelect`, etc.) with `For` binding |
| 4 | Writing raw SQL queries | Use EF Core LINQ queries, compiled queries, or `FromSqlInterpolated` for complex cases |
| 5 | Custom authentication / password hashing | Use ASP.NET Core Identity — it handles hashing, lockout, 2FA, token management |
| 6 | Using `OnInitializedAsync` for data that needs interactivity guard | Guard with `isLoaded` flag or disable pre-rendering to prevent double-loading |
| 7 | Calling `StateHasChanged()` after `await` in event handlers | Blazor calls `StateHasChanged` automatically after event handler completion — manual calls cause double rendering |
| 8 | Using `IJSRuntime` for things MudBlazor already provides | MudBlazor handles scroll, focus, clipboard, snackbar — check MudBlazor docs first |
| 9 | Catching generic `Exception` in services | Use `Result<T>` pattern — return typed failures, throw only for programmer errors |
| 10 | Registering services as Singleton when they use DbContext | DbContext is Scoped — services that depend on it must also be Scoped |
| 11 | Using `[Parameter]` for internal component state | `[Parameter]` is for parent-to-child data flow — use private fields for internal state |
| 12 | Data annotations on EF entities | Use Fluent API in separate `IEntityTypeConfiguration<T>` classes — keeps entities clean |
| 13 | Running `dotnet ef database update` in production | Use idempotent SQL scripts or migration bundles — never run CLI tools against production |
| 14 | Storing secrets in `appsettings.json` | Use User Secrets (dev), environment variables (prod), or a vault service |
| 15 | Using `AllowAnyOrigin()` in CORS policy | Whitelist specific origins — `AllowAnyOrigin` defeats the purpose of CORS |
| 16 | "Submit" button text | Use outcome-focused CTA: "Create Project", "Save Changes", "Send Invitation" |
| 17 | Inline CSS styles in Blazor components | Use MudBlazor's `Class` and `Style` props with utility classes, or CSS isolation (`.razor.css`) |
| 18 | Placing all services in `Program.cs` | Use `IServiceCollection` extension methods — keeps `Program.cs` clean and services organized |
| 19 | Not using `@key` directive on list-rendered components | Always use `@key` when rendering lists of components to help Blazor's diffing algorithm |
| 20 | Skipping antiforgery middleware | Always call `UseAntiforgery()` after `UseAuthorization()` — Blazor relies on it for form safety |
| 21 | Generic repository wrapping DbContext | DbContext IS the Unit of Work + Repository — avoid wrapping unless you have a specific need |
| 22 | Deploying without a CI test gate | CI must run tests + format check + vulnerability scan before deploy |
| 23 | Missing security headers | Every endpoint needs HSTS, CSP, X-Content-Type-Options, X-Frame-Options at minimum |
| 24 | Using `InteractiveServer` for cookie-dependent pages (login, register) | Identity pages MUST use Static SSR — use `[ExcludeFromInteractiveRouting]` |
| 25 | Not disposing `IAsyncDisposable` components | Implement `IAsyncDisposable` and cancel `CancellationTokenSource` in components with async operations |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Blazor patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:blazor&title=[Blazor]%20)**

Use the `patterns:blazor` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
