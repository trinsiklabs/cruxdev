# Development Patterns — TALL Stack

Tailwind CSS / Alpine.js / Livewire / Laravel

This document captures stack-specific patterns, conventions, and decisions for TALL stack projects (Tailwind/Alpine.js/Livewire/Laravel). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Laravel apps, test with Pest, use Livewire components, integrate Alpine.js, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| PHP | 8.3+ | Minimum 8.3 for typed class constants, `#[Override]` attribute |
| Laravel | 12.x | Released February 2025; requires PHP 8.2+ |
| Livewire | 3.x | Full-page components, SPA mode, form objects |
| Alpine.js | 3.14+ | Bundled with Livewire — do NOT install separately |
| Tailwind CSS | 4.x | CSS-based config, no `tailwind.config.js` |
| Filament | 3.x | Admin panel, forms, tables, actions, notifications |
| Pest | 3.x / 4.x | Elegant PHP testing framework with architecture tests |
| Laravel Sanctum | 4.x | API token + SPA session authentication |
| Laravel Horizon | 5.x | Redis queue dashboard and monitoring |
| PostgreSQL | 16+ | Primary database |
| Redis | 7+ | Cache, queues, sessions, broadcasting |
| Node.js | 22+ | Vite asset pipeline only |
| Vite | 6.x | Asset bundling via `laravel-vite-plugin` |

### Version Constraint Policy

Use caret (`^`) constraints in `composer.json` pinned to the minor version:

```json
{
    "require": {
        "php": "^8.3",
        "laravel/framework": "^12.0",
        "livewire/livewire": "^3.5",
        "filament/filament": "^3.2",
        "laravel/sanctum": "^4.0",
        "laravel/horizon": "^5.29"
    },
    "require-dev": {
        "pestphp/pest": "^3.7",
        "pestphp/pest-plugin-laravel": "^3.1",
        "larastan/larastan": "^3.0",
        "laravel/pint": "^1.18"
    }
}
```

```json
// Good — allows patch and minor updates within major
"laravel/framework": "^12.0"

// Bad — too tight, blocks minor updates with new features
"laravel/framework": "12.3.1"

// Bad — too loose, could pull in next major
"laravel/framework": ">=12.0"
```

Exception: for beta packages or packages with known instability, pin exact.

### PHP Version Enforcement

Lock the PHP version in `composer.json` and enforce in CI:

```json
{
    "require": {
        "php": "^8.3"
    },
    "config": {
        "platform": {
            "php": "8.3.0"
        }
    }
}
```

---

## 2. Project Structure

### Laravel Directory Conventions

Each domain is organized within Laravel's standard directory structure, with domain grouping inside `app/`:

```
app/
├── Console/                    # Artisan commands
│   └── Commands/
│       ├── SyncContacts.php
│       └── PruneExpiredTokens.php
├── Enums/                      # PHP 8.1+ backed enums
│   ├── UserRole.php
│   ├── OrderStatus.php
│   └── SubscriptionTier.php
├── Events/                     # Domain events
│   ├── UserRegistered.php
│   └── OrderCompleted.php
├── Exceptions/                 # Custom exceptions
│   └── PaymentFailedException.php
├── Http/
│   ├── Controllers/            # Thin controllers (API only)
│   │   └── Api/
│   │       └── WebhookController.php
│   ├── Middleware/
│   │   └── EnsureTeamMember.php
│   └── Requests/               # Form request validation (non-Livewire)
│       └── Api/
│           └── StoreContactRequest.php
├── Jobs/                       # Queue jobs
│   ├── ProcessWebhook.php
│   ├── SyncContactToProvider.php
│   └── GenerateReport.php
├── Listeners/                  # Event listeners
│   ├── SendWelcomeEmail.php
│   └── ProvisionDefaultResources.php
├── Livewire/                   # Livewire components
│   ├── Dashboard/
│   │   ├── Index.php
│   │   └── StatsWidget.php
│   ├── Users/
│   │   ├── Index.php
│   │   ├── Show.php
│   │   └── EditProfile.php
│   ├── Forms/                  # Livewire form objects
│   │   ├── UserForm.php
│   │   ├── ContactForm.php
│   │   └── SettingsForm.php
│   └── Components/             # Reusable Livewire sub-components
│       ├── DataTable.php
│       └── SearchFilter.php
├── Mail/                       # Mailable classes
│   └── WelcomeEmail.php
├── Models/                     # Eloquent models
│   ├── User.php
│   ├── Team.php
│   ├── Contact.php
│   ├── Order.php
│   └── Concerns/               # Shared model traits
│       ├── HasSlug.php
│       └── HasUuids.php
├── Notifications/              # Notification classes
│   └── OrderShipped.php
├── Observers/                  # Model observers
│   └── UserObserver.php
├── Policies/                   # Authorization policies
│   ├── UserPolicy.php
│   ├── TeamPolicy.php
│   └── ContactPolicy.php
├── Providers/                  # Service providers
│   └── AppServiceProvider.php
├── Services/                   # Business logic services
│   ├── PaymentService.php
│   ├── ReportGenerator.php
│   └── ExternalApi/
│       ├── ProviderClient.php
│       └── ProviderClientInterface.php
└── View/
    └── Components/             # Blade view components (anonymous + class)
        ├── AppLayout.php
        ├── GuestLayout.php
        └── StatusBadge.php
```

### Blade & Livewire View Structure

```
resources/
├── views/
│   ├── components/             # Blade components (anonymous)
│   │   ├── layouts/
│   │   │   ├── app.blade.php
│   │   │   └── guest.blade.php
│   │   ├── status-badge.blade.php
│   │   ├── confirm-modal.blade.php
│   │   └── empty-state.blade.php
│   ├── livewire/               # Livewire component views
│   │   ├── dashboard/
│   │   │   ├── index.blade.php
│   │   │   └── stats-widget.blade.php
│   │   ├── users/
│   │   │   ├── index.blade.php
│   │   │   ├── show.blade.php
│   │   │   └── edit-profile.blade.php
│   │   └── pages/              # Volt single-file components
│   │       ├── settings.blade.php
│   │       └── notifications.blade.php
│   ├── emails/                 # Email templates
│   │   └── welcome.blade.php
│   └── filament/               # Filament view overrides
│       └── pages/
│           └── auth/
│               └── login.blade.php
├── css/
│   └── app.css                 # Tailwind 4 entry point
└── js/
    └── app.js                  # Alpine.js + Livewire bootstrap
```

### Test Mirror Structure

Tests mirror the `app/` structure:

```
tests/
├── Feature/                    # Feature tests (HTTP, Livewire, end-to-end flows)
│   ├── Auth/
│   │   ├── LoginTest.php
│   │   ├── RegistrationTest.php
│   │   └── PasswordResetTest.php
│   ├── Livewire/
│   │   ├── Dashboard/
│   │   │   └── IndexTest.php
│   │   └── Users/
│   │       ├── IndexTest.php
│   │       └── EditProfileTest.php
│   ├── Jobs/
│   │   └── ProcessWebhookTest.php
│   └── Api/
│       └── WebhookControllerTest.php
├── Unit/                       # Unit tests (models, services, enums)
│   ├── Models/
│   │   ├── UserTest.php
│   │   └── ContactTest.php
│   ├── Services/
│   │   └── PaymentServiceTest.php
│   └── Enums/
│       └── UserRoleTest.php
├── Architecture/               # Pest architecture tests
│   └── ArchTest.php
├── Pest.php                    # Pest configuration
└── TestCase.php                # Base test case
```

**Convention:** Livewire components go in `app/Livewire/`, grouped by domain. Form objects go in `app/Livewire/Forms/`. Volt single-file components go in `resources/views/livewire/pages/`.

**Convention:** One Livewire component per user-facing page. Sub-components for reusable widgets (data tables, filters, charts). Never nest Livewire components more than two levels deep.

---

## 3. Eloquent Model Patterns

### Model Template

Every Eloquent model follows this structure:

```php
<?php

namespace App\Models;

use App\Enums\UserRole;
use App\Models\Concerns\HasSlug;
use App\Observers\UserObserver;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Foundation\Auth\User as Authenticatable;

#[ObservedBy(UserObserver::class)]
class User extends Authenticatable
{
    use HasFactory;
    use HasUuids;
    use SoftDeletes;

    /**
     * The attributes that are mass assignable.
     */
    protected $fillable = [
        'name',
        'email',
        'password',
        'role',
        'team_id',
    ];

    /**
     * The attributes that should be hidden for serialization.
     */
    protected $hidden = [
        'password',
        'remember_token',
    ];

    /**
     * Get the attributes that should be cast.
     */
    protected function casts(): array
    {
        return [
            'email_verified_at' => 'datetime',
            'password' => 'hashed',
            'role' => UserRole::class,
            'settings' => 'array',
        ];
    }

    // ─── Relationships ───────────────────────────────────────

    public function team(): BelongsTo
    {
        return $this->belongsTo(Team::class);
    }

    public function orders(): HasMany
    {
        return $this->hasMany(Order::class);
    }

    public function contacts(): HasMany
    {
        return $this->hasMany(Contact::class);
    }

    // ─── Accessors & Mutators ────────────────────────────────

    protected function fullName(): Attribute
    {
        return Attribute::make(
            get: fn () => "{$this->first_name} {$this->last_name}",
        );
    }

    // ─── Scopes ──────────────────────────────────────────────

    public function scopeActive($query)
    {
        return $query->whereNotNull('email_verified_at');
    }

    public function scopeRole($query, UserRole $role)
    {
        return $query->where('role', $role);
    }

    public function scopeSearch($query, ?string $term)
    {
        return $query->when($term, fn ($q) => $q
            ->where('name', 'ilike', "%{$term}%")
            ->orWhere('email', 'ilike', "%{$term}%")
        );
    }
}
```

**Conventions:**
- Always use UUIDs for primary keys (`HasUuids` trait) unless there is a specific performance reason for auto-incrementing integers
- Always include `HasFactory` for test factories
- Always use `SoftDeletes` for user-facing data — hard deletes only for transient data (sessions, cache)
- Cast enums using PHP 8.1+ backed enums — never store roles/statuses as raw strings
- Use `protected function casts(): array` (Laravel 12 method syntax), not the `$casts` property
- Group model code in sections: relationships, accessors/mutators, scopes, model events
- Use `ilike` for PostgreSQL case-insensitive search, `like` for MySQL

### Migrations

Always use Laravel's migration generator:

```bash
php artisan make:migration create_contacts_table
php artisan make:migration add_team_id_to_users_table
php artisan migrate
```

Migration conventions:

```php
<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('contacts', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('user_id')->constrained()->cascadeOnDelete();
            $table->foreignUuid('team_id')->constrained()->cascadeOnDelete();
            $table->string('name');
            $table->string('email')->unique();
            $table->string('phone')->nullable();
            $table->string('company')->nullable();
            $table->string('status')->default('active');
            $table->json('metadata')->nullable();
            $table->timestamps();
            $table->softDeletes();

            // Composite indexes for common queries
            $table->index(['team_id', 'status']);
            $table->index(['user_id', 'created_at']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('contacts');
    }
};
```

**Rules:**
- Never edit a migration after it's been committed. Write a new corrective migration instead.
- Always include `down()` for rollback capability in development.
- Always add indexes for foreign keys and frequently filtered columns.
- Use `foreignUuid()` with `constrained()` for type-safe foreign keys.
- Use `->nullable()` explicitly — never rely on implicit nullable behavior.

### Relationships

```php
// One-to-Many
public function orders(): HasMany
{
    return $this->hasMany(Order::class);
}

// Belongs-To
public function team(): BelongsTo
{
    return $this->belongsTo(Team::class);
}

// Many-to-Many with pivot
public function tags(): BelongsToMany
{
    return $this->belongsToMany(Tag::class)
        ->withPivot('order')
        ->withTimestamps();
}

// Has-Many-Through
public function orderItems(): HasManyThrough
{
    return $this->hasManyThrough(OrderItem::class, Order::class);
}

// Polymorphic
public function comments(): MorphMany
{
    return $this->morphMany(Comment::class, 'commentable');
}
```

**Convention:** Always declare return types on relationships. Always eager-load relationships you know you need — never rely on lazy loading in production (see anti-patterns).

### Model Events and Observers

Use observers for side effects. Keep models clean:

```php
#[ObservedBy(UserObserver::class)]
class User extends Authenticatable
{
    // ...
}
```

```php
<?php

namespace App\Observers;

use App\Models\User;
use App\Jobs\ProvisionDefaultResources;

class UserObserver
{
    public function created(User $user): void
    {
        ProvisionDefaultResources::dispatch($user);
    }

    public function deleting(User $user): void
    {
        // Clean up related resources before soft delete
        $user->tokens()->delete();
    }
}
```

**Convention:** Observers for side effects (sending emails, dispatching jobs, syncing external systems). Never put business logic in observers — that belongs in services or actions.

### Enums

Use PHP 8.1+ backed enums for all status/type fields:

```php
<?php

namespace App\Enums;

enum UserRole: string
{
    case Admin = 'admin';
    case Manager = 'manager';
    case Member = 'member';
    case Viewer = 'viewer';

    public function label(): string
    {
        return match ($this) {
            self::Admin => 'Administrator',
            self::Manager => 'Manager',
            self::Member => 'Team Member',
            self::Viewer => 'Viewer',
        };
    }

    public function color(): string
    {
        return match ($this) {
            self::Admin => 'red',
            self::Manager => 'amber',
            self::Member => 'blue',
            self::Viewer => 'gray',
        };
    }

    /**
     * Roles that can manage team members.
     */
    public static function managementRoles(): array
    {
        return [self::Admin, self::Manager];
    }
}
```

---

## 4. Authentication

### Starter Kit Selection

Laravel 12 provides official starter kits. Choose based on architecture:

| Starter Kit | Stack | Use When |
|---|---|---|
| **Laravel Breeze (Livewire)** | TALL | Building a Livewire-first app — the default for TALL projects |
| **Laravel Breeze (Inertia)** | Vue/React SPA | Building an SPA with a Laravel API backend |
| **Laravel Jetstream** | Livewire or Inertia | Need teams, 2FA, API tokens, session management out of the box |
| **Custom + Fortify** | Any | Need full control over auth UI with Fortify handling the backend |

### Breeze + Livewire Installation

```bash
composer require laravel/breeze --dev
php artisan breeze:install livewire

# Includes:
# - Login, registration, password reset, email verification
# - Profile management page
# - Livewire components for all auth flows
# - Tailwind CSS styling
```

### Fortify (Backend Auth Logic)

All starter kits use Fortify under the hood. For custom auth UIs, use Fortify directly:

```php
// config/fortify.php
'features' => [
    Features::registration(),
    Features::resetPasswords(),
    Features::emailVerification(),
    Features::updateProfileInformation(),
    Features::updatePasswords(),
    Features::twoFactorAuthentication([
        'confirm' => true,
        'confirmPassword' => true,
    ]),
],
```

### Sanctum (API Authentication)

Use Sanctum for API token authentication and SPA session authentication:

```php
// API token authentication
$token = $user->createToken('api-token', ['contacts:read', 'contacts:write']);

// In API routes
Route::middleware('auth:sanctum')->group(function () {
    Route::apiResource('contacts', ContactController::class);
});

// Token ability checks
if ($request->user()->tokenCan('contacts:write')) {
    // ...
}
```

SPA authentication uses Sanctum's cookie-based session auth:

```php
// config/sanctum.php
'stateful' => explode(',', env('SANCTUM_STATEFUL_DOMAINS',
    'localhost,localhost:3000,127.0.0.1,127.0.0.1:8000,::1'
)),
```

### Socialite (OAuth / Social Login)

Add social login providers with Socialite:

```bash
composer require laravel/socialite
```

```php
// config/services.php
'google' => [
    'client_id' => env('GOOGLE_CLIENT_ID'),
    'client_secret' => env('GOOGLE_CLIENT_SECRET'),
    'redirect' => env('GOOGLE_REDIRECT_URI'),
],

// Routes
Route::get('/auth/google', [SocialAuthController::class, 'redirect']);
Route::get('/auth/google/callback', [SocialAuthController::class, 'callback']);

// Controller
public function redirect(): RedirectResponse
{
    return Socialite::driver('google')->redirect();
}

public function callback(): RedirectResponse
{
    $googleUser = Socialite::driver('google')->user();

    $user = User::updateOrCreate(
        ['email' => $googleUser->getEmail()],
        [
            'name' => $googleUser->getName(),
            'google_id' => $googleUser->getId(),
            'avatar' => $googleUser->getAvatar(),
        ]
    );

    Auth::login($user, remember: true);

    return redirect()->intended('/dashboard');
}
```

### Role-Based Authorization

Use policies for all authorization. Never check roles in controllers or views directly:

```php
// app/Policies/ContactPolicy.php
class ContactPolicy
{
    public function viewAny(User $user): bool
    {
        return true; // All authenticated users can list
    }

    public function view(User $user, Contact $contact): bool
    {
        return $user->team_id === $contact->team_id;
    }

    public function create(User $user): bool
    {
        return in_array($user->role, UserRole::managementRoles());
    }

    public function delete(User $user, Contact $contact): bool
    {
        return $user->role === UserRole::Admin
            && $user->team_id === $contact->team_id;
    }
}
```

Register policies in `AppServiceProvider` or use auto-discovery:

```php
// Livewire component usage
public function mount(): void
{
    $this->authorize('viewAny', Contact::class);
}

public function deleteContact(Contact $contact): void
{
    $this->authorize('delete', $contact);
    $contact->delete();
}
```

---

## 5. Component Library

### Philosophy

Use established component libraries for all standard UI. Do NOT build custom components when a library provides one. Only build custom components for domain-specific UI.

### Livewire Volt (Single-File Components)

Volt allows PHP logic and Blade template in the same file. Use for simple, self-contained pages:

```php
<?php
// resources/views/livewire/pages/settings.blade.php

use App\Models\User;
use Livewire\Volt\Component;

new class extends Component
{
    public string $name = '';
    public string $email = '';

    public function mount(): void
    {
        $this->name = auth()->user()->name;
        $this->email = auth()->user()->email;
    }

    public function save(): void
    {
        $this->validate([
            'name' => 'required|string|max:255',
            'email' => 'required|email|unique:users,email,' . auth()->id(),
        ]);

        auth()->user()->update([
            'name' => $this->name,
            'email' => $this->email,
        ]);

        $this->dispatch('profile-updated');
    }
}; ?>

<div>
    <form wire:submit="save">
        <div>
            <label for="name">Name</label>
            <input id="name" wire:model="name" type="text" autocomplete="name" />
            @error('name') <span class="text-red-500 text-sm">{{ $message }}</span> @enderror
        </div>

        <div>
            <label for="email">Email</label>
            <input id="email" wire:model="email" type="email" autocomplete="email" />
            @error('email') <span class="text-red-500 text-sm">{{ $message }}</span> @enderror
        </div>

        <button type="submit" wire:loading.attr="disabled">
            <span wire:loading.remove>Save Changes</span>
            <span wire:loading>Saving...</span>
        </button>
    </form>
</div>
```

**When to use Volt:** Simple pages with minimal logic (settings, profile, notifications). For complex pages with multiple interactions, use standard class-based Livewire components.

### Blade Components

Use anonymous Blade components for presentational elements:

```html
<!-- resources/views/components/status-badge.blade.php -->
@props([
    'status' => 'active',
    'size' => 'sm',
])

@php
    $classes = match ($status) {
        'active' => 'bg-green-100 text-green-800',
        'inactive' => 'bg-gray-100 text-gray-800',
        'pending' => 'bg-yellow-100 text-yellow-800',
        'suspended' => 'bg-red-100 text-red-800',
        default => 'bg-gray-100 text-gray-800',
    };

    $sizeClasses = match ($size) {
        'xs' => 'px-1.5 py-0.5 text-xs',
        'sm' => 'px-2 py-1 text-xs',
        'md' => 'px-2.5 py-1 text-sm',
        'lg' => 'px-3 py-1.5 text-sm',
    };
@endphp

<span {{ $attributes->merge(['class' => "inline-flex items-center rounded-full font-medium {$classes} {$sizeClasses}"]) }}>
    {{ $slot }}
</span>
```

Usage:

```html
<x-status-badge status="active">Active</x-status-badge>
<x-status-badge status="pending" size="lg">Pending Review</x-status-badge>
```

### Wire Elements Pro

Wire Elements Pro provides premium Livewire components for common UI patterns:

| Component | Use For |
|---|---|
| **Modal** | Confirmation dialogs, detail views, forms in overlay |
| **Slide-over** | Side panels for editing, detail views |
| **Spotlight** | Cmd+K / Alfred-like search and navigation |
| **Autocomplete** | Mentions, hashtags, inline search |

Installation:

```bash
composer require wire-elements/pro
```

Usage:

```php
// Trigger a modal from any Livewire component
$this->dispatch('openModal', component: 'edit-contact', arguments: ['contact' => $contactId]);
```

### Filament Components (Admin Panel)

Filament provides a full admin panel built on Livewire + Alpine.js + Tailwind:

| Category | Components | Use For |
|---|---|---|
| **Resources** | CRUD pages (list, create, edit, view) | Admin data management |
| **Forms** | TextInput, Select, RichEditor, FileUpload, Repeater, Builder | Complex admin forms |
| **Tables** | Columns, Filters, Actions, BulkActions, Grouping | Data browsing and management |
| **Infolists** | TextEntry, IconEntry, ImageEntry, RepeatableEntry | Read-only record views |
| **Notifications** | Toast, database, broadcast | In-app feedback |
| **Actions** | Modal confirmations, form actions, table actions | User interactions |
| **Widgets** | StatsOverview, Chart, Table | Dashboard metrics |
| **Navigation** | Sidebar, top bar, breadcrumbs | Admin navigation |

Filament resource example:

```php
<?php

namespace App\Filament\Resources;

use App\Filament\Resources\ContactResource\Pages;
use App\Models\Contact;
use Filament\Forms;
use Filament\Forms\Form;
use Filament\Resources\Resource;
use Filament\Tables;
use Filament\Tables\Table;

class ContactResource extends Resource
{
    protected static ?string $model = Contact::class;
    protected static ?string $navigationIcon = 'heroicon-o-users';
    protected static ?string $navigationGroup = 'CRM';

    public static function form(Form $form): Form
    {
        return $form->schema([
            Forms\Components\TextInput::make('name')
                ->required()
                ->maxLength(255),
            Forms\Components\TextInput::make('email')
                ->email()
                ->required()
                ->unique(ignoreRecord: true),
            Forms\Components\TextInput::make('phone')
                ->tel(),
            Forms\Components\Select::make('status')
                ->options(ContactStatus::class)
                ->required(),
        ]);
    }

    public static function table(Table $table): Table
    {
        return $table
            ->columns([
                Tables\Columns\TextColumn::make('name')->searchable()->sortable(),
                Tables\Columns\TextColumn::make('email')->searchable(),
                Tables\Columns\TextColumn::make('status')->badge(),
                Tables\Columns\TextColumn::make('created_at')->dateTime()->sortable(),
            ])
            ->filters([
                Tables\Filters\SelectFilter::make('status')
                    ->options(ContactStatus::class),
            ])
            ->actions([
                Tables\Actions\EditAction::make(),
                Tables\Actions\DeleteAction::make(),
            ])
            ->bulkActions([
                Tables\Actions\BulkActionGroup::make([
                    Tables\Actions\DeleteBulkAction::make(),
                ]),
            ]);
    }

    public static function getPages(): array
    {
        return [
            'index' => Pages\ListContacts::route('/'),
            'create' => Pages\CreateContact::route('/create'),
            'edit' => Pages\EditContact::route('/{record}/edit'),
        ];
    }
}
```

### Tailwind 4 Integration

Tailwind CSS 4 uses CSS-based configuration — no `tailwind.config.js`:

```css
/* resources/css/app.css */
@import "tailwindcss";

/* Custom theme */
@theme {
    --color-primary-50: #eff6ff;
    --color-primary-100: #dbeafe;
    --color-primary-500: #3b82f6;
    --color-primary-600: #2563eb;
    --color-primary-700: #1d4ed8;
    --color-primary-900: #1e3a5f;

    --font-sans: "Inter", ui-sans-serif, system-ui, sans-serif;
    --font-mono: "JetBrains Mono", ui-monospace, monospace;
}
```

**Tailwind conventions:**
- Use Tailwind utilities for all styling — no custom CSS unless absolutely necessary
- Use `@theme` for design tokens (colors, fonts, spacing) — not arbitrary values
- Use `@apply` sparingly — only for base element styles or third-party library overrides
- Use responsive prefixes (`sm:`, `md:`, `lg:`) consistently with mobile-first design

---

## 6. Testing Patterns

### Test Pyramid (TALL-specific)

```
        /\
       /  \          Browser Tests (Pest v4 Dusk) — for critical user journeys
      /    \
     /------\
    /        \        Livewire Feature Tests
   /          \       Component rendering, interactions, form submissions
  /------------\
 /              \      Feature Tests (HTTP)
/                \     API endpoints, middleware, authentication flows
/------------------\
/                    \   Unit Tests
/                      \  Models, services, enums, value objects
/------------------------\
```

### Pest Configuration

```php
// tests/Pest.php
<?php

use Illuminate\Foundation\Testing\LazilyRefreshDatabase;
use Tests\TestCase;

pest()
    ->extend(TestCase::class)
    ->use(LazilyRefreshDatabase::class)
    ->in('Feature', 'Unit');
```

### Unit Tests

Test models, services, enums, and pure functions in isolation:

```php
// tests/Unit/Models/UserTest.php

use App\Enums\UserRole;
use App\Models\User;

describe('User model', function () {

    it('casts role to UserRole enum', function () {
        $user = User::factory()->create(['role' => 'admin']);

        expect($user->role)->toBe(UserRole::Admin);
    });

    it('scopes active users to verified emails', function () {
        User::factory()->create(['email_verified_at' => now()]);
        User::factory()->create(['email_verified_at' => null]);

        expect(User::active()->count())->toBe(1);
    });

    it('computes full name from first and last', function () {
        $user = User::factory()->make([
            'first_name' => 'Jane',
            'last_name' => 'Smith',
        ]);

        expect($user->full_name)->toBe('Jane Smith');
    });

});
```

### Livewire Feature Tests

Test Livewire components with the built-in testing API:

```php
// tests/Feature/Livewire/Users/EditProfileTest.php

use App\Livewire\Users\EditProfile;
use App\Models\User;
use Livewire\Livewire;

describe('EditProfile component', function () {

    it('renders for authenticated user', function () {
        $user = User::factory()->create();

        Livewire::actingAs($user)
            ->test(EditProfile::class)
            ->assertStatus(200)
            ->assertSee($user->name);
    });

    it('validates required fields', function () {
        $user = User::factory()->create();

        Livewire::actingAs($user)
            ->test(EditProfile::class)
            ->set('form.name', '')
            ->set('form.email', '')
            ->call('save')
            ->assertHasErrors(['form.name' => 'required', 'form.email' => 'required']);
    });

    it('updates user profile on valid submission', function () {
        $user = User::factory()->create();

        Livewire::actingAs($user)
            ->test(EditProfile::class)
            ->set('form.name', 'Updated Name')
            ->set('form.email', 'updated@example.com')
            ->call('save')
            ->assertHasNoErrors()
            ->assertDispatched('profile-updated');

        expect($user->fresh())
            ->name->toBe('Updated Name')
            ->email->toBe('updated@example.com');
    });

    it('rejects duplicate email', function () {
        $existing = User::factory()->create(['email' => 'taken@example.com']);
        $user = User::factory()->create();

        Livewire::actingAs($user)
            ->test(EditProfile::class)
            ->set('form.email', 'taken@example.com')
            ->call('save')
            ->assertHasErrors(['form.email' => 'unique']);
    });

    it('denies access to unauthenticated users', function () {
        Livewire::test(EditProfile::class)
            ->assertForbidden();
    });

});
```

### HTTP Feature Tests

Test API endpoints and controller actions:

```php
// tests/Feature/Api/WebhookControllerTest.php

use App\Jobs\ProcessWebhook;
use Illuminate\Support\Facades\Queue;

describe('Webhook API', function () {

    it('accepts valid webhook and dispatches job', function () {
        Queue::fake();

        $payload = [
            'event' => 'contact.created',
            'data' => ['id' => 'ext_123', 'name' => 'Jane Doe'],
        ];

        $this->postJson('/api/webhooks', $payload, [
            'X-Webhook-Signature' => generateSignature($payload),
        ])
            ->assertOk();

        Queue::assertPushed(ProcessWebhook::class, function ($job) {
            return $job->event === 'contact.created';
        });
    });

    it('rejects webhook with invalid signature', function () {
        $this->postJson('/api/webhooks', ['event' => 'test'], [
            'X-Webhook-Signature' => 'invalid',
        ])
            ->assertForbidden();
    });

});
```

### Factories

Use Laravel model factories for all test data:

```php
// database/factories/UserFactory.php

namespace Database\Factories;

use App\Enums\UserRole;
use App\Models\Team;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Str;

class UserFactory extends Factory
{
    public function definition(): array
    {
        return [
            'name' => fake()->name(),
            'email' => fake()->unique()->safeEmail(),
            'email_verified_at' => now(),
            'password' => '$2y$12$somehashedpassword', // password
            'role' => UserRole::Member,
            'team_id' => Team::factory(),
            'remember_token' => Str::random(10),
        ];
    }

    public function unverified(): static
    {
        return $this->state(fn () => ['email_verified_at' => null]);
    }

    public function admin(): static
    {
        return $this->state(fn () => ['role' => UserRole::Admin]);
    }

    public function manager(): static
    {
        return $this->state(fn () => ['role' => UserRole::Manager]);
    }
}
```

Usage in tests:

```php
// Simple creation
$user = User::factory()->create();

// With state
$admin = User::factory()->admin()->create();

// With relationships
$team = Team::factory()
    ->has(User::factory()->count(5))
    ->create();

// With specific attributes
$user = User::factory()->create([
    'name' => 'Jane Smith',
    'email' => 'jane@example.com',
]);
```

### Database Testing

Use `LazilyRefreshDatabase` for optimal performance — it only resets the database when a test actually touches it:

```php
// tests/Pest.php
pest()
    ->extend(TestCase::class)
    ->use(LazilyRefreshDatabase::class)
    ->in('Feature', 'Unit');
```

For tests that need transaction isolation without full migration:

```php
use Illuminate\Foundation\Testing\DatabaseTransactions;

// Wraps each test in a transaction and rolls back after
pest()
    ->extend(TestCase::class)
    ->use(DatabaseTransactions::class)
    ->in('Feature');
```

### Architecture Tests

Pest supports architecture tests that enforce structural rules:

```php
// tests/Architecture/ArchTest.php

arch('models extend Model or Authenticatable')
    ->expect('App\Models')
    ->toExtend('Illuminate\Database\Eloquent\Model')
    ->ignoring('App\Models\User');

arch('controllers are invokable or extend Controller')
    ->expect('App\Http\Controllers')
    ->toExtendNothing()
    ->toHaveSuffix('Controller');

arch('enums are string-backed')
    ->expect('App\Enums')
    ->toBeStringBackedEnums();

arch('no debugging functions in production code')
    ->expect(['dd', 'dump', 'ray', 'var_dump'])
    ->not->toBeUsed();

arch('jobs implement ShouldQueue')
    ->expect('App\Jobs')
    ->toImplement('Illuminate\Contracts\Queue\ShouldQueue');

arch('livewire components live in the right namespace')
    ->expect('App\Livewire')
    ->toExtend('Livewire\Component');

arch('no env() calls outside config files')
    ->expect('env')
    ->not->toBeUsed()
    ->ignoring('App\Providers');
```

### Test Configuration

```php
// phpunit.xml (environment variables for testing)
<php>
    <env name="APP_ENV" value="testing"/>
    <env name="DB_CONNECTION" value="pgsql"/>
    <env name="DB_DATABASE" value="testing"/>
    <env name="BCRYPT_ROUNDS" value="4"/>
    <env name="CACHE_STORE" value="array"/>
    <env name="MAIL_MAILER" value="array"/>
    <env name="QUEUE_CONNECTION" value="sync"/>
    <env name="SESSION_DRIVER" value="array"/>
    <env name="TELESCOPE_ENABLED" value="false"/>
</php>
```

---

## 7. Livewire Patterns

### Component Structure

Every Livewire component follows this structure:

```php
<?php

namespace App\Livewire\Contacts;

use App\Livewire\Forms\ContactForm;
use App\Models\Contact;
use Illuminate\Contracts\View\View;
use Livewire\Attributes\Computed;
use Livewire\Attributes\Layout;
use Livewire\Attributes\Title;
use Livewire\Attributes\Url;
use Livewire\Component;
use Livewire\WithPagination;

#[Layout('components.layouts.app')]
#[Title('Contacts')]
class Index extends Component
{
    use WithPagination;

    public ContactForm $form;

    #[Url(as: 'q')]
    public string $search = '';

    #[Url]
    public string $status = '';

    #[Url]
    public string $sortBy = 'created_at';

    #[Url]
    public string $sortDir = 'desc';

    // ─── Computed Properties ─────────────────────────────────

    #[Computed]
    public function contacts()
    {
        return Contact::query()
            ->search($this->search)
            ->when($this->status, fn ($q) => $q->where('status', $this->status))
            ->orderBy($this->sortBy, $this->sortDir)
            ->paginate(25);
    }

    // ─── Actions ─────────────────────────────────────────────

    public function sort(string $column): void
    {
        if ($this->sortBy === $column) {
            $this->sortDir = $this->sortDir === 'asc' ? 'desc' : 'asc';
        } else {
            $this->sortBy = $column;
            $this->sortDir = 'asc';
        }

        $this->resetPage();
    }

    public function updatedSearch(): void
    {
        $this->resetPage();
    }

    public function delete(Contact $contact): void
    {
        $this->authorize('delete', $contact);
        $contact->delete();
    }

    // ─── Render ──────────────────────────────────────────────

    public function render(): View
    {
        return view('livewire.contacts.index');
    }
}
```

### Reactive Properties and Wire Directives

```html
<!-- wire:model — deferred by default (batched with next request) -->
<input wire:model="search" type="text" />

<!-- wire:model.live — sends request on every keystroke (debounced 250ms) -->
<input wire:model.live="search" type="text" />

<!-- wire:model.live.debounce.500ms — custom debounce -->
<input wire:model.live.debounce.500ms="search" type="text" />

<!-- wire:model.blur — sends request when field loses focus -->
<input wire:model.blur="email" type="email" />

<!-- wire:click — trigger an action -->
<button wire:click="delete({{ $contact->id }})">Delete</button>

<!-- wire:click with confirmation -->
<button wire:click="delete({{ $contact->id }})"
    wire:confirm="Are you sure you want to delete this contact?">
    Delete
</button>

<!-- wire:submit — form submission -->
<form wire:submit="save">
    <!-- fields -->
    <button type="submit">Save</button>
</form>
```

### Lazy Loading

Defer expensive component rendering until visible:

```html
<!-- Lazy load — renders placeholder until component is in viewport -->
<livewire:dashboard.stats-widget lazy />

<!-- Custom placeholder -->
<livewire:contacts.data-table lazy>
    <x-slot:placeholder>
        <div class="animate-pulse h-64 bg-gray-100 rounded-lg"></div>
    </x-slot:placeholder>
</livewire:contacts.data-table>
```

In the component:

```php
use Livewire\Attributes\Lazy;

#[Lazy]
class StatsWidget extends Component
{
    public function placeholder(): string
    {
        return <<<'HTML'
        <div class="animate-pulse h-32 bg-gray-100 rounded-lg"></div>
        HTML;
    }
}
```

### Polling

Update a component on an interval:

```html
<!-- Poll every 5 seconds -->
<div wire:poll.5s>
    {{ $notificationCount }} unread
</div>

<!-- Poll only when tab is visible -->
<div wire:poll.visible.15s>
    Last synced: {{ $lastSyncAt }}
</div>

<!-- Poll with a specific method -->
<div wire:poll.10s="refreshMetrics">
    <!-- metrics display -->
</div>
```

### Events and Communication

Components communicate via events:

```php
// Dispatching an event
$this->dispatch('contact-saved', contactId: $contact->id);

// Dispatching to a specific component
$this->dispatch('refresh')->to(ContactList::class);

// Dispatching to self
$this->dispatch('reset-form')->self();
```

```php
// Listening to events
use Livewire\Attributes\On;

#[On('contact-saved')]
public function handleContactSaved(int $contactId): void
{
    $this->contacts = Contact::all();
}
```

### SPA Mode (wire:navigate)

Use `wire:navigate` for SPA-like page transitions without full reloads:

```html
<!-- In your layout -->
<nav>
    <a href="/dashboard" wire:navigate>Dashboard</a>
    <a href="/contacts" wire:navigate>Contacts</a>
    <a href="/settings" wire:navigate>Settings</a>
</nav>
```

```php
// Prefetch on hover for instant navigation
<a href="/contacts" wire:navigate.hover>Contacts</a>
```

**Convention:** Use `wire:navigate` on all internal navigation links. Use standard `<a href>` for external links. This gives SPA-like speed without an SPA framework.

### Loading States

```html
<!-- Show loading indicator during any action -->
<button wire:click="save">
    <span wire:loading.remove wire:target="save">Save Contact</span>
    <span wire:loading wire:target="save">Saving...</span>
</button>

<!-- Disable button during action -->
<button wire:click="save" wire:loading.attr="disabled" wire:target="save">
    Save Contact
</button>

<!-- Loading overlay on a section -->
<div wire:loading.class="opacity-50 pointer-events-none" wire:target="search">
    <!-- content that dims during search -->
</div>
```

### File Uploads

```php
use Livewire\WithFileUploads;

class EditProfile extends Component
{
    use WithFileUploads;

    public $avatar;

    public function save(): void
    {
        $this->validate([
            'avatar' => 'nullable|image|max:2048', // 2MB max
        ]);

        if ($this->avatar) {
            $path = $this->avatar->store('avatars', 'public');
            auth()->user()->update(['avatar_path' => $path]);
        }
    }
}
```

```html
<input type="file" wire:model="avatar" accept="image/*" />

<div wire:loading wire:target="avatar">Uploading...</div>

@if ($avatar)
    <img src="{{ $avatar->temporaryUrl() }}" alt="Preview" class="h-20 w-20 rounded-full" />
@endif
```

---

## 8. Alpine.js Integration

### Core Directives

Alpine.js is bundled with Livewire. Use it for client-side interactivity that does not need the server:

```html
<!-- x-data: declare reactive state -->
<div x-data="{ open: false }">
    <button @click="open = !open">Toggle</button>
    <div x-show="open" x-transition>
        Dropdown content
    </div>
</div>

<!-- x-show: conditionally show/hide (CSS display) -->
<div x-show="isVisible" x-transition.duration.200ms>
    Content
</div>

<!-- x-if: conditionally render (DOM insertion/removal) -->
<template x-if="showAdvanced">
    <div>Advanced options...</div>
</template>

<!-- x-for: loop rendering -->
<template x-for="item in items" :key="item.id">
    <div x-text="item.name"></div>
</template>

<!-- x-on / @: event handling -->
<button @click="count++" @click.outside="open = false">
    Click me
</button>

<!-- x-model: two-way binding -->
<input x-model="query" type="text" placeholder="Search..." />

<!-- x-bind / :: dynamic attributes -->
<button :class="{ 'bg-blue-500': active, 'bg-gray-300': !active }">
    Toggle
</button>

<!-- x-text / x-html: dynamic content -->
<span x-text="count"></span>
<div x-html="renderedMarkdown"></div>
```

### Entangle with Livewire

Entangle synchronizes Alpine state with Livewire properties:

```html
<!-- Deferred entangle — syncs on next Livewire request (default) -->
<div x-data="{ open: $wire.entangle('showModal') }">
    <button @click="open = true">Open Modal</button>

    <div x-show="open" @keydown.escape.window="open = false"
         x-transition:enter="ease-out duration-300"
         x-transition:leave="ease-in duration-200">
        <div class="fixed inset-0 bg-black/50" @click="open = false"></div>
        <div class="relative bg-white rounded-lg p-6">
            Modal content managed by Livewire
        </div>
    </div>
</div>

<!-- Live entangle — syncs immediately -->
<div x-data="{ search: $wire.entangle('search').live }">
    <input x-model="search" type="text" />
</div>
```

### Transitions

```html
<!-- Basic transition -->
<div x-show="open" x-transition>Content</div>

<!-- Custom transition classes (Tailwind-friendly) -->
<div x-show="open"
     x-transition:enter="transition ease-out duration-200"
     x-transition:enter-start="opacity-0 -translate-y-1"
     x-transition:enter-end="opacity-100 translate-y-0"
     x-transition:leave="transition ease-in duration-150"
     x-transition:leave-start="opacity-100 translate-y-0"
     x-transition:leave-end="opacity-0 -translate-y-1">
    Animated content
</div>

<!-- Scale transition -->
<div x-show="open"
     x-transition:enter="transition ease-out duration-200"
     x-transition:enter-start="opacity-0 scale-95"
     x-transition:enter-end="opacity-100 scale-100"
     x-transition:leave="transition ease-in duration-75"
     x-transition:leave-start="opacity-100 scale-100"
     x-transition:leave-end="opacity-0 scale-95">
    Modal panel
</div>
```

### Alpine Plugins

Alpine ships with several official plugins. Install only what you need:

```html
<!-- In resources/js/app.js -->
import Alpine from 'alpinejs';
import collapse from '@alpinejs/collapse';
import focus from '@alpinejs/focus';
import intersect from '@alpinejs/intersect';

Alpine.plugin(collapse);
Alpine.plugin(focus);
Alpine.plugin(intersect);
```

| Plugin | Use For |
|---|---|
| **collapse** | Smooth accordion-style open/close animations |
| **focus** | Focus trapping in modals, dropdowns |
| **intersect** | Lazy loading content when element enters viewport |
| **persist** | Persist Alpine state to localStorage |
| **morph** | DOM morphing for advanced update strategies |
| **mask** | Input masking (phone numbers, dates) |

### When to Use Alpine vs. Livewire

| Scenario | Use |
|---|---|
| Toggle visibility (dropdown, modal, accordion) | Alpine (`x-show`) |
| Client-side form validation preview | Alpine (`x-data`) |
| Keyboard shortcuts | Alpine (`@keydown`) |
| Animations and transitions | Alpine (`x-transition`) |
| Fetch/save data to the database | Livewire (`wire:click`) |
| Real-time validation against the server | Livewire (`wire:model.live`) |
| Pagination, sorting, filtering with DB queries | Livewire |
| File uploads | Livewire (`WithFileUploads`) |
| State that must survive page navigation | Livewire |

**Rule:** If the interaction does not need the server, use Alpine. If it needs the server, use Livewire. If it needs both (e.g., a modal whose visibility is client-side but whose content is server-rendered), use `$wire.entangle()`.

---

## 9. Inertia.js Alternative

### When to Use Inertia vs. Livewire

Both are first-party Laravel options for building modern UIs. Choose based on the project's needs:

| Factor | Livewire (TALL) | Inertia (Vue/React) |
|---|---|---|
| **Rendering** | Server-rendered HTML | Client-side SPA |
| **State** | Server-side (PHP) | Client-side (JS) |
| **Reactivity** | Server round-trip per interaction | Instant client-side |
| **Learning curve** | PHP + Blade (stay in Laravel) | Vue/React + JS ecosystem |
| **SEO** | Server-rendered by default | Requires SSR setup |
| **Team skills** | Backend-heavy team | Full-stack or frontend-heavy team |
| **Offline support** | None (requires server) | Possible with service workers |
| **Complex UI** | Good for most apps | Better for highly interactive UIs |
| **Mobile app** | Not applicable | Can share components with React Native |
| **Real-time** | Built-in polling, events | WebSocket + Pusher/Reverb |
| **Bundle size** | Minimal JS (~30KB Livewire + Alpine) | Vue ~50KB / React ~45KB + app code |
| **File uploads** | Built into Livewire | Requires custom handling |

### Choose Livewire When

- The team is primarily PHP/Laravel developers
- The app is CRUD-heavy (admin panels, dashboards, data management)
- SEO matters and you want server-rendered HTML by default
- You want to stay in a single language (PHP) for both logic and rendering
- The app does not require complex client-side state management
- You value rapid prototyping and less JavaScript build tooling

### Choose Inertia When

- The team has strong Vue or React experience
- The app requires highly interactive, complex client-side UIs (drag-and-drop, canvas, real-time collaboration)
- You need to share UI components with a mobile app (React Native)
- Offline-first or PWA capabilities are required
- The app benefits from client-side routing and state management (Pinia, Zustand)

### Hybrid Approach

It is possible (though not common) to use both in the same Laravel app:

```php
// routes/web.php

// Livewire routes (admin, CRUD pages)
Route::middleware(['auth'])->group(function () {
    Route::get('/admin/contacts', \App\Livewire\Contacts\Index::class);
});

// Inertia routes (public-facing SPA pages)
Route::get('/app/{any}', function () {
    return Inertia::render('App');
})->where('any', '.*');
```

**Convention:** Pick one for a project and stick with it. Mixing adds complexity. The only acceptable hybrid is Livewire for admin + Inertia for public-facing SPA, with clear boundary between them.

---

## 10. Queue / Jobs

### Laravel Queue Architecture

```
HTTP Request → Controller/Livewire → dispatch(Job) → Queue (Redis)
                                                        ↓
                                              Queue Worker (Horizon)
                                                        ↓
                                                   Job::handle()
                                                        ↓
                                              Success → done
                                              Failure → retry or fail
```

### Job Template

```php
<?php

namespace App\Jobs;

use App\Models\Contact;
use App\Services\ExternalApi\ProviderClientInterface;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\Middleware\RateLimited;
use Illuminate\Queue\Middleware\WithoutOverlapping;
use Illuminate\Queue\SerializesModels;

class SyncContactToProvider implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    /**
     * Number of times the job may be attempted.
     */
    public int $tries = 3;

    /**
     * Seconds to wait before retrying.
     */
    public array $backoff = [10, 60, 300];

    /**
     * Seconds after which the job should timeout.
     */
    public int $timeout = 30;

    public function __construct(
        public readonly Contact $contact,
    ) {}

    /**
     * Job middleware.
     */
    public function middleware(): array
    {
        return [
            new RateLimited('provider-api'),
            (new WithoutOverlapping($this->contact->id))->dontRelease(),
        ];
    }

    /**
     * Execute the job.
     */
    public function handle(ProviderClientInterface $client): void
    {
        $client->upsertContact([
            'external_id' => $this->contact->external_id,
            'name' => $this->contact->name,
            'email' => $this->contact->email,
        ]);

        $this->contact->update(['synced_at' => now()]);
    }

    /**
     * Handle a job failure.
     */
    public function failed(\Throwable $exception): void
    {
        $this->contact->update(['sync_failed_at' => now()]);
        report($exception);
    }
}
```

### Horizon Configuration

```php
// config/horizon.php
'environments' => [
    'production' => [
        'supervisor-1' => [
            'connection' => 'redis',
            'queue' => ['default', 'high', 'low'],
            'balance' => 'auto',
            'autoScalingStrategy' => 'time',
            'maxProcesses' => 10,
            'minProcesses' => 1,
            'balanceMaxShift' => 1,
            'balanceCooldown' => 3,
            'tries' => 3,
            'timeout' => 60,
        ],
    ],
    'local' => [
        'supervisor-1' => [
            'connection' => 'redis',
            'queue' => ['default', 'high', 'low'],
            'balance' => 'simple',
            'processes' => 3,
            'tries' => 3,
            'timeout' => 60,
        ],
    ],
],
```

### Job Batching

Process large operations in batches with progress tracking:

```php
use Illuminate\Bus\Batch;
use Illuminate\Support\Facades\Bus;

$contacts = Contact::where('synced_at', null)->get();

$jobs = $contacts->map(fn ($contact) => new SyncContactToProvider($contact));

$batch = Bus::batch($jobs)
    ->name('Sync unsynced contacts')
    ->allowFailures()
    ->then(function (Batch $batch) {
        // All jobs completed successfully
        Notification::send(admin(), new BatchCompleted($batch));
    })
    ->catch(function (Batch $batch, \Throwable $e) {
        // First batch job failure detected
        report($e);
    })
    ->finally(function (Batch $batch) {
        // Batch finished (even with failures)
        Log::info("Batch {$batch->id}: {$batch->processedJobs()}/{$batch->totalJobs} processed");
    })
    ->onQueue('high')
    ->dispatch();
```

### Rate Limiting Jobs

Define rate limiters in `AppServiceProvider`:

```php
use Illuminate\Cache\RateLimiting\Limit;
use Illuminate\Support\Facades\RateLimiter;

public function boot(): void
{
    RateLimiter::for('provider-api', function ($job) {
        return Limit::perMinute(60);
    });
}
```

### Queue Priority

```php
// Dispatch to specific queue
SyncContactToProvider::dispatch($contact)->onQueue('high');

// Process queues in priority order
// php artisan horizon (respects config)
// or manually: php artisan queue:work --queue=high,default,low
```

### Testing Jobs

```php
use Illuminate\Support\Facades\Queue;

it('dispatches sync job when contact is created', function () {
    Queue::fake();

    $contact = Contact::factory()->create();

    Queue::assertPushed(SyncContactToProvider::class, function ($job) use ($contact) {
        return $job->contact->id === $contact->id;
    });
});

it('retries on failure with backoff', function () {
    Queue::fake();

    $contact = Contact::factory()->create();
    $job = new SyncContactToProvider($contact);

    expect($job->tries)->toBe(3);
    expect($job->backoff)->toBe([10, 60, 300]);
});
```

---

## 11. Development Workflow

### Feature Development Cycle (TALL-specific)

```
1. Write failing test (Pest)
2. Create migration if new table/columns needed
3. Create/update Eloquent model
4. Create Livewire component + form object
5. Create Blade view
6. Run: php artisan test
7. Run: ./vendor/bin/pint
8. Refactor while green
9. Run: ./vendor/bin/phpstan analyse
```

### Artisan Commands

```bash
# ─── Development ──────────────────────────────────────────
php artisan serve                           # Start dev server (port 8000)
php artisan tinker                          # REPL with full app context

# ─── Code Generation ─────────────────────────────────────
php artisan make:model Contact -mfs         # Model + migration + factory + seeder
php artisan make:livewire Contacts/Index    # Livewire component + view
php artisan make:livewire-form ContactForm  # Livewire form object
php artisan make:policy ContactPolicy       # Authorization policy
php artisan make:job SyncContact            # Queue job
php artisan make:event ContactCreated       # Event class
php artisan make:listener SendWelcome       # Event listener
php artisan make:mail WelcomeEmail          # Mailable
php artisan make:enum UserRole              # PHP enum (custom command)
php artisan make:test ContactTest           # Pest test (Feature by default)
php artisan make:test ContactTest --unit    # Pest unit test

# ─── Database ─────────────────────────────────────────────
php artisan migrate                         # Run pending migrations
php artisan migrate:rollback                # Rollback last batch (dev only)
php artisan migrate:fresh --seed            # Drop all + migrate + seed
php artisan db:seed                         # Run seeders
php artisan db:seed --class=ContactSeeder   # Run specific seeder

# ─── Cache & Config ───────────────────────────────────────
php artisan config:cache                    # Cache config (production)
php artisan route:cache                     # Cache routes (production)
php artisan view:cache                      # Cache views (production)
php artisan optimize                        # Cache config + routes + views
php artisan optimize:clear                  # Clear all caches

# ─── Queue ────────────────────────────────────────────────
php artisan horizon                         # Start Horizon dashboard + workers
php artisan queue:work --queue=high,default # Manual queue worker
php artisan queue:failed                    # List failed jobs
php artisan queue:retry all                 # Retry all failed jobs

# ─── Filament ─────────────────────────────────────────────
php artisan make:filament-resource Contact  # Filament CRUD resource
php artisan make:filament-page Settings     # Custom Filament page
php artisan make:filament-widget StatsOverview # Dashboard widget
```

### Laravel Pint (Code Style)

Pint is Laravel's official code style fixer, built on PHP-CS-Fixer:

```bash
# Fix all files
./vendor/bin/pint

# Check without fixing (CI mode)
./vendor/bin/pint --test

# Fix specific path
./vendor/bin/pint app/Models/

# Show diff of changes
./vendor/bin/pint -v
```

Configuration in `pint.json`:

```json
{
    "preset": "laravel",
    "rules": {
        "declare_strict_types": true,
        "final_class": false,
        "void_return": true,
        "types_spaces": {
            "space": "none"
        }
    },
    "exclude": [
        "bootstrap/cache",
        "storage"
    ]
}
```

### Larastan (Static Analysis)

Larastan extends PHPStan with Laravel-specific rules:

```bash
# Run static analysis
./vendor/bin/phpstan analyse

# At specific level (0-9, higher = stricter)
./vendor/bin/phpstan analyse --level=8
```

Configuration in `phpstan.neon`:

```neon
includes:
    - vendor/larastan/larastan/extension.neon

parameters:
    level: 8
    paths:
        - app/
    excludePaths:
        - app/Console/Kernel.php
    checkMissingIterableValueType: false
    treatPhpDocTypesAsCertain: false
```

### IDE Helper

Generate IDE autocompletion for Laravel facades, models, and macros:

```bash
composer require --dev barryvdh/laravel-ide-helper

# Generate helper files
php artisan ide-helper:generate       # Facades
php artisan ide-helper:models -M      # Model PHPDocs (write to _ide_helper_models.php)
php artisan ide-helper:eloquent       # Eloquent methods
```

Add to `.gitignore`:

```
_ide_helper.php
_ide_helper_models.php
.phpstorm.meta.php
```

### Composer Scripts

```json
{
    "scripts": {
        "post-autoload-dump": [
            "Illuminate\\Foundation\\ComposerScripts::postAutoloadDump",
            "@php artisan package:discover --ansi"
        ],
        "lint": "./vendor/bin/pint --test",
        "fix": "./vendor/bin/pint",
        "analyse": "./vendor/bin/phpstan analyse",
        "test": "php artisan test",
        "test:coverage": "php artisan test --coverage --min=80",
        "test:parallel": "php artisan test --parallel",
        "quality": [
            "@lint",
            "@analyse",
            "@test"
        ],
        "ci": [
            "@lint",
            "@analyse",
            "@test:coverage"
        ]
    }
}
```

---

## 12. Deployment

### Platform Selection

| Platform | Type | Best For |
|---|---|---|
| **Laravel Forge** | VPS provisioning | Teams wanting server control with managed setup |
| **Laravel Cloud** | Fully managed (EC2) | Production Laravel apps with zero ops (launched Feb 2025) |
| **Laravel Vapor** | Serverless (Lambda) | Spiky traffic, event-driven workloads, auto-scaling |
| **Docker + Fly.io** | Container | Multi-region, edge deployment, custom runtime needs |
| **Envoyer** | Zero-downtime deploy | Add-on for any server — deploys without downtime |

### Laravel Forge

Forge provisions and manages VPS servers on DigitalOcean, AWS, Linode, or Hetzner:

```bash
# Forge handles:
# - Server provisioning (Nginx, PHP, MySQL/PostgreSQL, Redis, Node)
# - SSL certificates (Let's Encrypt auto-renewal)
# - Deploy scripts (git pull, composer install, migrate, cache)
# - Queue worker management (Horizon supervisor)
# - Scheduled task configuration (cron)
# - Server monitoring and notifications
```

Deploy script (configured in Forge dashboard):

```bash
cd /home/forge/app.example.com
git pull origin main
composer install --no-dev --no-interaction --prefer-dist --optimize-autoloader
php artisan migrate --force
php artisan config:cache
php artisan route:cache
php artisan view:cache
php artisan event:cache
php artisan queue:restart
npm ci
npm run build
```

### Laravel Vapor (Serverless)

Vapor deploys to AWS Lambda with auto-scaling:

```yaml
# vapor.yml
id: 12345
name: my-app
environments:
    production:
        runtime: php-8.3
        region: us-east-1
        memory: 1024
        cli-memory: 512
        build:
            - 'ASSET_URL=https://cdn.example.com composer install --no-dev'
            - 'npm ci && npm run build && rm -rf node_modules'
        deploy:
            - 'php artisan migrate --force'
            - 'php artisan config:cache'
        storage: my-app-production
        database: my-app-db
        cache: my-app-cache
        queues:
            - default
            - high
    staging:
        runtime: php-8.3
        region: us-east-1
        memory: 512
        database: my-app-staging-db
        cache: my-app-staging-cache
```

```bash
vapor deploy production
vapor deploy staging
vapor logs production
vapor tail production       # Stream live logs
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM php:8.3-fpm-alpine AS base

RUN apk add --no-cache \
    postgresql-dev \
    libzip-dev \
    && docker-php-ext-install pdo_pgsql zip opcache

COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# ─── Build stage ──────────────────────────────────────────
FROM base AS build

WORKDIR /app
COPY composer.json composer.lock ./
RUN composer install --no-dev --no-scripts --prefer-dist

COPY . .
RUN composer dump-autoload --optimize

# Node assets
FROM node:22-alpine AS assets
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

# ─── Production stage ────────────────────────────────────
FROM base AS production

WORKDIR /app
COPY --from=build /app /app
COPY --from=assets /app/public/build /app/public/build

RUN php artisan config:cache \
    && php artisan route:cache \
    && php artisan view:cache

EXPOSE 8000
CMD ["php", "artisan", "serve", "--host=0.0.0.0", "--port=8000"]
```

### CI/CD Pipeline (GitHub Actions)

```yaml
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
          POSTGRES_DB: testing
          POSTGRES_USER: forge
          POSTGRES_PASSWORD: secret
        ports: ['5432:5432']
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        ports: ['6379:6379']

    steps:
      - uses: actions/checkout@v4

      - name: Setup PHP
        uses: shivammathur/setup-php@v2
        with:
          php-version: '8.3'
          extensions: pdo_pgsql, redis
          coverage: pcov

      - name: Install dependencies
        run: composer install --prefer-dist --no-progress

      - name: Pint (code style)
        run: ./vendor/bin/pint --test

      - name: Larastan (static analysis)
        run: ./vendor/bin/phpstan analyse

      - name: Run tests
        run: php artisan test --coverage --min=80
        env:
          DB_CONNECTION: pgsql
          DB_HOST: 127.0.0.1
          DB_PORT: 5432
          DB_DATABASE: testing
          DB_USERNAME: forge
          DB_PASSWORD: secret

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Deploy
        run: |
          # Forge: curl trigger
          # Vapor: vapor deploy production
          # Fly.io: flyctl deploy
```

---

## 13. Security

### CSRF Protection

Laravel automatically generates and validates CSRF tokens for all POST/PUT/PATCH/DELETE requests:

```html
<!-- Blade forms include the token automatically -->
<form method="POST" action="/contacts">
    @csrf
    <!-- fields -->
</form>
```

Livewire handles CSRF automatically — no manual `@csrf` needed in `wire:submit` forms.

Exclude routes from CSRF in `bootstrap/app.php`:

```php
->withMiddleware(function (Middleware $middleware) {
    $middleware->validateCsrfTokens(except: [
        'api/webhooks/*', // External webhooks
    ]);
})
```

### XSS Prevention

Blade auto-escapes all output by default:

```html
<!-- Escaped (safe) — use for all user content -->
{{ $user->name }}
{{ $user->bio }}

<!-- Unescaped (dangerous) — ONLY for trusted, sanitized HTML -->
{!! $page->sanitized_html !!}
```

**Rule:** Never use `{!! !!}` with user-provided content. If you must render HTML from users, sanitize with a library like `mews/purifier`:

```php
use Mews\Purifier\Facades\Purifier;

$clean = Purifier::clean($userInput);
```

### SQL Injection Prevention

Eloquent and the query builder use parameterized queries by default:

```php
// Safe — parameterized
User::where('email', $email)->first();
DB::table('users')->where('email', $email)->first();

// Safe — parameterized binding
DB::select('SELECT * FROM users WHERE email = ?', [$email]);

// DANGEROUS — raw SQL with interpolation
DB::select("SELECT * FROM users WHERE email = '$email'");  // NEVER DO THIS

// Safe raw expression when needed
User::whereRaw('LOWER(email) = ?', [strtolower($email)])->first();
```

### Mass Assignment Protection

Always define `$fillable` or `$guarded` on every model:

```php
// Good — explicit whitelist
protected $fillable = ['name', 'email', 'phone'];

// Acceptable — explicit blacklist (less preferred)
protected $guarded = ['id', 'role', 'is_admin'];

// DANGEROUS — never do this
protected $guarded = [];
```

### Rate Limiting

Apply rate limits to routes and API endpoints:

```php
// bootstrap/app.php
->withMiddleware(function (Middleware $middleware) {
    $middleware->throttleApi('api', 60); // 60 requests/minute for API
})

// Custom rate limiters in AppServiceProvider
RateLimiter::for('login', function (Request $request) {
    return [
        Limit::perMinute(5)->by($request->input('email')),
        Limit::perMinute(30)->by($request->ip()),
    ];
});

// Apply to routes
Route::post('/login', LoginController::class)->middleware('throttle:login');
```

### Encryption

```php
use Illuminate\Support\Facades\Crypt;

// Encrypt sensitive data at rest
$encrypted = Crypt::encryptString($apiKey);
$decrypted = Crypt::decryptString($encrypted);

// Or use Eloquent casts for automatic encryption
protected function casts(): array
{
    return [
        'api_key' => 'encrypted',
        'settings' => 'encrypted:array',
    ];
}
```

### Security Headers

Add security headers via middleware:

```php
// app/Http/Middleware/SecurityHeaders.php
class SecurityHeaders
{
    public function handle(Request $request, Closure $next): Response
    {
        $response = $next($request);

        return $response
            ->header('Strict-Transport-Security', 'max-age=31536000; includeSubDomains')
            ->header('X-Content-Type-Options', 'nosniff')
            ->header('X-Frame-Options', 'SAMEORIGIN')
            ->header('Referrer-Policy', 'strict-origin-when-cross-origin')
            ->header('Permissions-Policy', 'camera=(), microphone=(), geolocation=()')
            ->header('Content-Security-Policy', implode('; ', [
                "default-src 'self'",
                "script-src 'self' 'unsafe-inline' 'unsafe-eval'",  // Required by Livewire/Alpine
                "style-src 'self' 'unsafe-inline'",                  // Required by Tailwind
                "img-src 'self' data: https:",
                "font-src 'self'",
                "connect-src 'self' wss:",                           // Required by Livewire WebSocket
                "frame-ancestors 'self'",
            ]));
    }
}
```

**Notes:**
- `unsafe-inline` and `unsafe-eval` in `script-src` are required for Livewire and Alpine.js
- `unsafe-inline` in `style-src` is required for Tailwind's dynamic styles
- `wss:` in `connect-src` is required for Livewire's WebSocket connection
- HSTS is set to 1 year with `includeSubDomains`

### Livewire-Specific Security

```php
// Lock sensitive properties — prevents client-side manipulation
use Livewire\Attributes\Locked;

#[Locked]
public int $userId;

#[Locked]
public string $orderId;

// Always authorize actions
public function deleteContact(Contact $contact): void
{
    $this->authorize('delete', $contact);   // Gate check
    $contact->delete();
}

// Validate all input — never trust client data
public function save(): void
{
    $this->validate();  // Uses form object rules or component rules
    // ... save logic
}
```

---

## 14. Coverage Enforcement

### Pest Coverage

Coverage is enforced via Pest's built-in coverage support (requires PCOV or Xdebug):

```bash
# Terminal coverage report
php artisan test --coverage

# With minimum threshold (CI gate)
php artisan test --coverage --min=80

# Exact coverage requirement
php artisan test --coverage --exactly=95.5

# HTML report
php artisan test --coverage --coverage-html=coverage/

# Parallel execution with coverage
php artisan test --parallel --coverage --min=80
```

### Coverage Configuration

```xml
<!-- phpunit.xml — coverage configuration -->
<coverage>
    <include>
        <directory suffix=".php">./app</directory>
    </include>
    <exclude>
        <directory suffix=".php">./app/Console</directory>
        <directory suffix=".php">./app/Exceptions</directory>
        <directory suffix=".php">./app/Providers</directory>
        <file>./app/Http/Middleware/TrustProxies.php</file>
    </exclude>
    <report>
        <text outputFile="php://stdout" showOnlySummary="true"/>
    </report>
</coverage>
```

### Coverage Targets

| Phase | Minimum | Enforced By |
|---|---|---|
| **Development** | 80% | `php artisan test --coverage --min=80` |
| **Pre-merge** | 80% | CI pipeline (GitHub Actions) |
| **Mature project** | 90%+ | Increase `--min` as codebase stabilizes |
| **Target** | 100% | Per CLAUDE.md core rules |

### Parallel Testing

Use parallel testing to speed up large test suites:

```bash
# Install paratest
composer require --dev brianium/paratest

# Run tests in parallel
php artisan test --parallel

# Parallel with coverage
php artisan test --parallel --coverage --min=80

# Specify process count
php artisan test --parallel --processes=8
```

**Note:** Parallel testing creates separate database instances per process. Ensure your test database can handle the concurrency. Use `LazilyRefreshDatabase` instead of `RefreshDatabase` for optimal parallel performance.

### Type Coverage

Pest v4 includes type coverage analysis:

```bash
# Check type coverage
php artisan test --type-coverage

# With minimum threshold
php artisan test --type-coverage --min=90
```

Type coverage ensures that all function parameters, return types, and properties have explicit type declarations.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with fieldsets |
| **labels** | Top-aligned, visible `<label>`, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | Inline + error summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Create Account" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | `wire:model` deferred by default, `.live` or `.blur` only where needed |

### Livewire Form Objects

Always extract form properties into a Form Object for maintainability:

```php
<?php

namespace App\Livewire\Forms;

use Livewire\Attributes\Rule;
use Livewire\Attributes\Validate;
use Livewire\Form;

class ContactForm extends Form
{
    #[Validate('required|string|max:255')]
    public string $name = '';

    #[Validate('required|email|max:255|unique:contacts,email')]
    public string $email = '';

    #[Validate('nullable|string|max:20')]
    public ?string $phone = null;

    #[Validate('nullable|string|max:255')]
    public ?string $company = null;

    #[Validate('required|string|in:active,inactive,pending')]
    public string $status = 'active';

    public function store(): void
    {
        $this->validate();

        Contact::create($this->all());

        $this->reset();
    }

    public function setContact(Contact $contact): void
    {
        $this->name = $contact->name;
        $this->email = $contact->email;
        $this->phone = $contact->phone;
        $this->company = $contact->company;
        $this->status = $contact->status->value;
    }

    public function update(Contact $contact): void
    {
        $this->validate([
            'email' => "required|email|unique:contacts,email,{$contact->id}",
        ]);

        $contact->update($this->all());
    }
}
```

### Real-Time Validation Pattern

Use `wire:model.blur` for real-time validation that does not overwhelm the server:

```html
<form wire:submit="save" novalidate>
    <fieldset>
        <legend class="text-sm font-semibold text-gray-700">Contact Information</legend>

        <div class="space-y-4">
            <div>
                <label for="name" class="block text-sm font-medium text-gray-700">
                    Full Name
                </label>
                <input
                    id="name"
                    wire:model.blur="form.name"
                    type="text"
                    autocomplete="name"
                    required
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm
                           focus:border-primary-500 focus:ring-primary-500 sm:text-sm
                           @error('form.name') border-red-500 @enderror"
                />
                @error('form.name')
                    <p class="mt-1 text-sm text-red-600" role="alert">
                        <span aria-hidden="true" class="mr-1">&#9888;</span>
                        {{ $message }}
                    </p>
                @enderror
            </div>

            <div>
                <label for="email" class="block text-sm font-medium text-gray-700">
                    Email Address
                </label>
                <input
                    id="email"
                    wire:model.blur="form.email"
                    type="email"
                    autocomplete="email"
                    required
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm
                           focus:border-primary-500 focus:ring-primary-500 sm:text-sm
                           @error('form.email') border-red-500 @enderror"
                />
                @error('form.email')
                    <p class="mt-1 text-sm text-red-600" role="alert">
                        <span aria-hidden="true" class="mr-1">&#9888;</span>
                        {{ $message }}
                    </p>
                @enderror
            </div>

            <div>
                <label for="phone" class="block text-sm font-medium text-gray-700">
                    Phone <span class="text-gray-400">(optional)</span>
                </label>
                <input
                    id="phone"
                    wire:model="form.phone"
                    type="tel"
                    autocomplete="tel"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm
                           focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                />
            </div>
        </div>
    </fieldset>

    <!-- Error Summary (for screen readers) -->
    @if ($errors->any())
        <div role="alert" aria-live="assertive"
             class="rounded-md bg-red-50 p-4 mt-4">
            <h3 class="text-sm font-medium text-red-800">
                Please correct the following errors:
            </h3>
            <ul class="mt-2 text-sm text-red-700 list-disc list-inside">
                @foreach ($errors->all() as $error)
                    <li>{{ $error }}</li>
                @endforeach
            </ul>
        </div>
    @endif

    <div class="mt-6">
        <button
            type="submit"
            wire:loading.attr="disabled"
            class="inline-flex items-center justify-center h-12 px-6
                   rounded-md bg-primary-600 text-white font-medium
                   hover:bg-primary-700 focus:outline-none focus:ring-2
                   focus:ring-primary-500 focus:ring-offset-2
                   disabled:opacity-50 disabled:cursor-not-allowed"
        >
            <span wire:loading.remove wire:target="save">Save Contact</span>
            <span wire:loading wire:target="save">
                <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white inline" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                </svg>
                Saving...
            </span>
        </button>
    </div>
</form>
```

### Validation Timing Guide

| Field Count | Strategy | Implementation |
|---|---|---|
| 1-6 fields | Submit-only | `wire:model` (deferred) + validate on `wire:submit` |
| 7+ fields | Reward-early, punish-late | `wire:model.blur` on required fields, validate on blur after first submit |
| Email/username | Live uniqueness check | `wire:model.live.debounce.500ms` for the unique field only |
| Password | Live strength feedback | Alpine.js client-side (no server round-trip) |

### Accessibility Checklist (TALL-specific)

- [ ] `novalidate` on every `<form>` tag — HTML5 native validation is unreliable across assistive technologies
- [ ] `autocomplete` attribute on every input (`name`, `email`, `tel`, `street-address`, etc.)
- [ ] Every `<label>` has a matching `for`/`id` pair
- [ ] Error messages use `role="alert"` and `aria-live="assertive"`
- [ ] Error summary exists for forms with 3+ fields
- [ ] Submit button minimum height 48px (touch target)
- [ ] Loading state shown with both visual indicator and `disabled` attribute
- [ ] Optional fields marked with "(optional)" text

---

## 16. Anti-Patterns (TALL-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | **N+1 queries** — lazy loading relationships in loops | Use `->with()` eager loading: `Contact::with('team', 'tags')->paginate()` |
| 2 | **Fat controllers** — business logic in controllers | Move logic to services, actions, or form objects. Controllers dispatch, they do not compute. |
| 3 | **Missing authorization** on Livewire actions | Always call `$this->authorize()` before mutations. Livewire actions are public by default. |
| 4 | **Passing Eloquent models to Livewire public properties** | Pass IDs or use primitive types. Livewire serializes/deserializes the full model on every request. For large models this causes performance degradation. |
| 5 | **Using `wire:model.live` on every input** | Default to `wire:model` (deferred). Use `.blur` for validation. Use `.live` only for search/filter fields that need immediate response. |
| 6 | **Deeply nested Livewire components** (3+ levels) | Never nest Livewire components more than 2 levels deep. Use Blade components for presentation-only nesting. |
| 7 | **Missing `wire:key` in loops** | Always add `wire:key="{{ $item->id }}"` to elements inside `@foreach` loops. Livewire needs this for correct DOM diffing. |
| 8 | **Installing Alpine.js separately** when using Livewire | Livewire 3 bundles Alpine. Installing a separate Alpine causes conflicts. Remove any standalone Alpine `<script>` tags. |
| 9 | **Using `$guarded = []`** on models | Always define explicit `$fillable` arrays. Empty guarded allows mass assignment of any attribute including `role`, `is_admin`, etc. |
| 10 | **Raw SQL with string interpolation** | Use parameterized queries: `DB::select('... WHERE id = ?', [$id])` or Eloquent query builder. |
| 11 | **`{!! $userContent !!}`** (unescaped output) | Use `{{ }}` for all user content. Sanitize with `Purifier::clean()` if HTML rendering is required. |
| 12 | **Business logic in Blade templates** | Move to computed properties, model accessors, or view composers. Blade is for presentation only. |
| 13 | **Not using Form Objects** in Livewire | Always extract form properties to a `Livewire\Form` class for components with 3+ form fields. |
| 14 | **Calling `env()` outside config files** | Cache breaks `env()`. Always use `config()` in application code. `env()` only in `config/*.php` files. |
| 15 | **"Submit" button text** | Use outcome-focused CTAs: "Save Contact", "Create Account", "Send Message" — not "Submit" or "Save". |
| 16 | **Missing `novalidate`** on forms | Always add `novalidate` to `<form>` tags. HTML5 native validation is inconsistent and interferes with server-side validation UX. |
| 17 | **No loading states** on actions | Always show `wire:loading` feedback on buttons and sections. Users need to know the server is processing. |
| 18 | **Testing with `DB::` instead of factories** | Use model factories for all test data. Factories ensure consistent, valid data and make tests readable. |
| 19 | **Storing secrets in config files or `.env.example`** | Use `env()` in config, actual values in `.env` only (never committed). Use encrypted environment variables in CI. |
| 20 | **Not using policies for authorization** | Never check roles inline (`if ($user->role === 'admin')`). Define policies and use `$this->authorize()` or `@can` directives. |
| 21 | **Ignoring Livewire `#[Locked]` for sensitive properties** | Any property that should not be tampered with from the client (IDs, prices, permissions) must use `#[Locked]`. |
| 22 | **Missing `autocomplete` attributes** on form inputs | Always add `autocomplete="name"`, `autocomplete="email"`, `autocomplete="tel"`, etc. Improves UX and accessibility. |
| 23 | **Not handling failed jobs** | Always implement `failed()` method on jobs. Log, notify, or take corrective action. Never let jobs fail silently. |
| 24 | **Deploying without running migrations** | Deploy scripts must always run `php artisan migrate --force` before the app serves traffic. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a TALL patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:tall&title=[TALL]%20)**

Use the `patterns:tall` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
