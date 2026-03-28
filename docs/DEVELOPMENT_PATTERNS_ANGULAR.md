# Development Patterns — Angular

Angular 19+ / Angular Material / TypeScript / RxJS / NgRx

This document captures stack-specific patterns, conventions, and decisions for Angular projects (Angular 19+, Angular Material, TypeScript, RxJS, NgRx). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Angular components, test with Jest/Playwright, use Angular Material, manage state with NgRx/Signals, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Angular | 19.x | Standalone components by default, Signal-based reactivity |
| Angular CLI | 19.x | `ng` commands, schematics, builders |
| TypeScript | 5.6+ | Strict mode required (`strict: true`) |
| Node.js | 22.x LTS | Runtime for dev tooling, SSR |
| Angular Material | 19.x | Material Design 3 (M3) components, CDK |
| Angular CDK | 19.x | Low-level UI primitives (overlay, drag-drop, virtual scroll, a11y) |
| RxJS | 7.x | Reactive extensions, used alongside Signals |
| NgRx | 19.x | Signal Store for state management |
| Jest | 29.x | Unit/integration test runner (replaces Karma) |
| Playwright | 1.49+ | E2E testing (replaces Protractor) |
| ESLint | 9.x | Flat config, `@angular-eslint` |
| Prettier | 3.x | Code formatting |
| Tailwind CSS | 4.x | Utility-first CSS (optional, alongside Material) |

### Version Constraint Policy

Use exact or tilde constraints in `package.json`:

```json
{
  "dependencies": {
    "@angular/core": "~19.1.0",
    "@angular/material": "~19.1.0",
    "@ngrx/signals": "~19.0.0",
    "rxjs": "~7.8.0"
  }
}
```

```jsonc
// Good — allows patch updates, blocks minor/major
"@angular/core": "~19.1.0"

// Bad — too loose, allows breaking minor updates
"@angular/core": "^19.0.0"

// Bad — too tight, blocks security patches
"@angular/core": "19.1.3"
```

Exception: for release candidates or packages with known instability, pin exact.

### Angular Update Policy

Use `ng update` for all Angular version bumps. Never manually edit Angular package versions:

```bash
# Check what can be updated
ng update

# Update Angular core + CLI together (they must stay in sync)
ng update @angular/core @angular/cli

# Update Angular Material
ng update @angular/material

# Update NgRx
ng update @ngrx/signals
```

---

## 2. Project Structure

### Application Organization

Angular projects use a feature-based directory structure with standalone components:

```
src/
├── app/
│   ├── core/                      # Singleton services, guards, interceptors
│   │   ├── auth/
│   │   │   ├── auth.guard.ts
│   │   │   ├── auth.interceptor.ts
│   │   │   └── auth.service.ts
│   │   ├── http/
│   │   │   ├── api.interceptor.ts
│   │   │   ├── error.interceptor.ts
│   │   │   └── api.service.ts
│   │   └── layout/
│   │       ├── header.component.ts
│   │       ├── sidebar.component.ts
│   │       └── layout.component.ts
│   ├── shared/                    # Reusable components, directives, pipes
│   │   ├── components/
│   │   │   ├── confirm-dialog/
│   │   │   ├── loading-spinner/
│   │   │   └── empty-state/
│   │   ├── directives/
│   │   │   ├── auto-focus.directive.ts
│   │   │   └── click-outside.directive.ts
│   │   ├── pipes/
│   │   │   ├── relative-time.pipe.ts
│   │   │   └── truncate.pipe.ts
│   │   └── validators/
│   │       ├── match-fields.validator.ts
│   │       └── unique-async.validator.ts
│   ├── features/                  # Feature modules (lazy-loaded)
│   │   ├── dashboard/
│   │   │   ├── dashboard.component.ts
│   │   │   ├── dashboard.component.html
│   │   │   ├── dashboard.component.spec.ts
│   │   │   ├── dashboard.routes.ts
│   │   │   ├── widgets/
│   │   │   │   ├── stats-card.component.ts
│   │   │   │   └── activity-feed.component.ts
│   │   │   └── store/
│   │   │       ├── dashboard.store.ts
│   │   │       └── dashboard.store.spec.ts
│   │   ├── users/
│   │   │   ├── user-list/
│   │   │   │   ├── user-list.component.ts
│   │   │   │   ├── user-list.component.html
│   │   │   │   └── user-list.component.spec.ts
│   │   │   ├── user-detail/
│   │   │   ├── user-form/
│   │   │   ├── models/
│   │   │   │   └── user.model.ts
│   │   │   ├── services/
│   │   │   │   └── user.service.ts
│   │   │   ├── store/
│   │   │   │   ├── users.store.ts
│   │   │   │   └── users.store.spec.ts
│   │   │   └── users.routes.ts
│   │   └── settings/
│   │       ├── settings.component.ts
│   │       ├── settings.routes.ts
│   │       └── store/
│   ├── app.component.ts
│   ├── app.config.ts              # Application configuration (providers)
│   ├── app.routes.ts              # Top-level routes
│   └── app.config.server.ts       # SSR configuration
├── assets/
├── environments/
│   ├── environment.ts
│   └── environment.prod.ts
├── styles/
│   ├── _material-theme.scss       # Angular Material M3 theme
│   ├── _variables.scss
│   └── styles.scss
├── index.html
└── main.ts                        # bootstrapApplication()
```

**Conventions:**
- Every feature is a self-contained directory under `features/`
- Each feature has its own `routes.ts` for lazy loading
- Store files live inside their feature directory under `store/`
- Shared code goes in `shared/` — it must have no feature-specific dependencies
- Core services are singletons provided in `root` — they go in `core/`
- Models/interfaces live in `models/` within their feature
- One component per file. No multi-component files.

### File Naming Convention

```
component:    user-list.component.ts        user-list.component.html
service:      user.service.ts
store:        users.store.ts
guard:        auth.guard.ts
interceptor:  api.interceptor.ts
directive:    auto-focus.directive.ts
pipe:         relative-time.pipe.ts
model:        user.model.ts
validator:    match-fields.validator.ts
test:         user-list.component.spec.ts
route:        users.routes.ts
```

All filenames use kebab-case. All class names use PascalCase. All suffixes match the Angular artifact type.

### Test Mirror Structure

Tests are co-located with their source files (Angular convention):

```
src/app/features/users/
├── user-list/
│   ├── user-list.component.ts
│   ├── user-list.component.html
│   ├── user-list.component.spec.ts      # Co-located unit test
│   └── user-list.component.scss
├── services/
│   ├── user.service.ts
│   └── user.service.spec.ts             # Co-located service test
└── store/
    ├── users.store.ts
    └── users.store.spec.ts              # Co-located store test

e2e/                                      # E2E tests (project root)
├── fixtures/
│   └── test-data.json
├── pages/                                # Page Object Model
│   ├── login.page.ts
│   ├── dashboard.page.ts
│   └── users.page.ts
├── specs/
│   ├── auth.spec.ts
│   ├── dashboard.spec.ts
│   └── users.spec.ts
├── playwright.config.ts
└── global-setup.ts
```

---

## 3. Component Patterns

### Standalone Components (Default in Angular 19)

All components are standalone. NgModules are not used for new code:

```typescript
import { Component, input, output, computed, signal } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { DatePipe } from '@angular/common';

@Component({
  selector: 'app-user-card',
  standalone: true,
  imports: [MatCardModule, MatButtonModule, MatIconModule, DatePipe],
  template: `
    <mat-card>
      <mat-card-header>
        <mat-card-title>{{ user().name }}</mat-card-title>
        <mat-card-subtitle>{{ user().email }}</mat-card-subtitle>
      </mat-card-header>
      <mat-card-content>
        <p>Joined: {{ user().createdAt | date:'mediumDate' }}</p>
        <p>Role: {{ formattedRole() }}</p>
      </mat-card-content>
      <mat-card-actions>
        <button mat-button (click)="edit.emit(user())">
          <mat-icon>edit</mat-icon> Edit
        </button>
        <button mat-button color="warn" (click)="delete.emit(user().id)">
          <mat-icon>delete</mat-icon> Delete
        </button>
      </mat-card-actions>
    </mat-card>
  `,
})
export class UserCardComponent {
  // Signal-based inputs (required by default)
  user = input.required<User>();

  // Signal-based outputs
  edit = output<User>();
  delete = output<string>();

  // Computed signals derived from inputs
  formattedRole = computed(() => {
    const role = this.user().role;
    return role.charAt(0).toUpperCase() + role.slice(1);
  });
}
```

**Conventions:**
- Always declare `standalone: true` (even though it's the default in Angular 19, be explicit for clarity)
- Use `input()` and `input.required()` signal-based inputs, not `@Input()`
- Use `output()` signal-based outputs, not `@Output() + EventEmitter`
- Use `computed()` for derived state, not getters
- Import only what you need in `imports` — no barrel imports of entire modules
- Use `signal()` for local mutable state, not class properties

### Signal-Based Reactivity

Angular 19 uses Signals as the primary reactivity model. Signals replace zone.js-based change detection:

```typescript
import { Component, signal, computed, effect } from '@angular/core';

@Component({
  selector: 'app-counter',
  standalone: true,
  template: `
    <div>
      <p>Count: {{ count() }}</p>
      <p>Double: {{ doubled() }}</p>
      <button (click)="increment()">+</button>
      <button (click)="decrement()">-</button>
      <button (click)="reset()">Reset</button>
    </div>
  `,
})
export class CounterComponent {
  // Writable signal — local mutable state
  count = signal(0);

  // Computed signal — derived, read-only
  doubled = computed(() => this.count() * 2);

  // Effect — side effect that runs when dependencies change
  private logEffect = effect(() => {
    console.log(`Count changed to: ${this.count()}`);
  });

  increment(): void {
    this.count.update(c => c + 1);
  }

  decrement(): void {
    this.count.update(c => c - 1);
  }

  reset(): void {
    this.count.set(0);
  }
}
```

**Signal Rules:**
- Use `signal()` for local component state
- Use `computed()` for anything derived from other signals
- Use `effect()` sparingly — only for side effects (logging, localStorage sync, analytics)
- Never mutate signal values in place — always use `.set()` or `.update()`
- Prefer `input()` signals over `@Input()` decorators
- Read signals by calling them as functions: `this.count()`, not `this.count`

### Resource API for Async Data

Angular 19 introduces `resource()` and `rxResource()` for declarative async data loading:

```typescript
import { Component, signal, resource } from '@angular/core';
import { rxResource } from '@angular/core/rxjs-interop';
import { UserService } from '../services/user.service';

@Component({
  selector: 'app-user-detail',
  standalone: true,
  imports: [/* ... */],
  template: `
    @if (userResource.isLoading()) {
      <mat-spinner />
    }

    @if (userResource.hasValue()) {
      <app-user-card [user]="userResource.value()!" />
    }

    @if (userResource.error()) {
      <app-error-state [error]="userResource.error()" (retry)="userResource.reload()" />
    }
  `,
})
export class UserDetailComponent {
  private userService = inject(UserService);

  userId = input.required<string>();

  // rxResource — uses Observable-based service methods
  userResource = rxResource({
    request: () => this.userId(),
    loader: ({ request: id }) => this.userService.getUser(id),
  });
}
```

**Resource Rules:**
- Use `rxResource()` when wrapping existing Observable-based services
- Use `resource()` when working with Promise-based APIs
- Always handle loading, value, and error states in the template
- Resources automatically re-fetch when their `request` signal changes
- Use `.reload()` for manual refresh (e.g., after mutations)

### Smart vs. Presentational Components

**Smart components** (containers): connect to stores/services, manage state, handle routing.

```typescript
// Smart component — injects services, manages state
@Component({
  selector: 'app-user-list-page',
  standalone: true,
  imports: [UserListComponent, UserFilterComponent, MatPaginatorModule],
  template: `
    <app-user-filter (filterChange)="onFilterChange($event)" />
    <app-user-list
      [users]="store.filteredUsers()"
      [loading]="store.loading()"
      (edit)="onEdit($event)"
      (delete)="onDelete($event)"
    />
    <mat-paginator
      [length]="store.totalCount()"
      [pageSize]="store.pageSize()"
      (page)="store.setPage($event)"
    />
  `,
})
export class UserListPageComponent {
  protected store = inject(UsersStore);

  onFilterChange(filter: UserFilter): void {
    this.store.setFilter(filter);
  }

  onEdit(user: User): void {
    this.router.navigate(['/users', user.id, 'edit']);
  }

  onDelete(userId: string): void {
    this.store.deleteUser(userId);
  }
}
```

**Presentational components** (dumb): receive data via inputs, emit events via outputs. No injected services. No side effects.

```typescript
// Presentational component — pure inputs/outputs
@Component({
  selector: 'app-user-list',
  standalone: true,
  imports: [MatTableModule, MatButtonModule, MatIconModule],
  template: `
    @if (loading()) {
      <mat-spinner />
    } @else if (users().length === 0) {
      <app-empty-state message="No users found" />
    } @else {
      <mat-table [dataSource]="users()">
        <!-- columns -->
      </mat-table>
    }
  `,
})
export class UserListComponent {
  users = input.required<User[]>();
  loading = input(false);
  edit = output<User>();
  delete = output<string>();
}
```

### Control Flow Syntax (Angular 19)

Use the built-in control flow syntax, not `*ngIf`, `*ngFor`, `*ngSwitch`:

```html
<!-- Conditionals -->
@if (user()) {
  <app-user-card [user]="user()!" />
} @else if (loading()) {
  <mat-spinner />
} @else {
  <p>No user found</p>
}

<!-- Loops with track -->
@for (user of users(); track user.id) {
  <app-user-card [user]="user" />
} @empty {
  <app-empty-state message="No users yet" />
}

<!-- Switch -->
@switch (user().role) {
  @case ('admin') {
    <mat-icon>admin_panel_settings</mat-icon>
  }
  @case ('editor') {
    <mat-icon>edit</mat-icon>
  }
  @default {
    <mat-icon>person</mat-icon>
  }
}

<!-- Deferred loading -->
@defer (on viewport) {
  <app-heavy-chart [data]="chartData()" />
} @placeholder {
  <div class="chart-placeholder">Chart loading...</div>
} @loading (minimum 300ms) {
  <mat-spinner />
} @error {
  <p>Failed to load chart</p>
}
```

**Control Flow Rules:**
- Always use `@if`/`@for`/`@switch` — never `*ngIf`/`*ngFor`/`*ngSwitch`
- Always provide `track` in `@for` loops — use a unique identifier, not the index
- Use `@empty` blocks for empty state handling in loops
- Use `@defer` for heavy components that are below the fold or behind interaction
- Use `@placeholder` and `@loading` with `@defer` for progressive rendering

---

## 4. Authentication & Authorization

### Auth Service Pattern

Authentication is handled via a centralized `AuthService` backed by JWT tokens:

```typescript
import { Injectable, signal, computed } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { jwtDecode } from 'jwt-decode';

interface AuthTokens {
  accessToken: string;
  refreshToken: string;
}

interface TokenPayload {
  sub: string;
  email: string;
  roles: string[];
  exp: number;
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly http = inject(HttpClient);
  private readonly router = inject(Router);

  private readonly tokenPayload = signal<TokenPayload | null>(null);

  readonly isAuthenticated = computed(() => {
    const payload = this.tokenPayload();
    return payload !== null && payload.exp * 1000 > Date.now();
  });

  readonly currentUser = computed(() => {
    const payload = this.tokenPayload();
    if (!payload) return null;
    return { id: payload.sub, email: payload.email, roles: payload.roles };
  });

  readonly hasRole = (role: string) =>
    computed(() => this.currentUser()?.roles.includes(role) ?? false);

  constructor() {
    // Restore session from stored token
    const stored = localStorage.getItem('access_token');
    if (stored) {
      try {
        this.tokenPayload.set(jwtDecode<TokenPayload>(stored));
      } catch {
        this.clearTokens();
      }
    }
  }

  login(credentials: { email: string; password: string }) {
    return this.http.post<AuthTokens>('/api/auth/login', credentials).pipe(
      tap(tokens => this.storeTokens(tokens)),
    );
  }

  logout(): void {
    this.clearTokens();
    this.router.navigate(['/login']);
  }

  getAccessToken(): string | null {
    return localStorage.getItem('access_token');
  }

  refreshAccessToken() {
    const refreshToken = localStorage.getItem('refresh_token');
    return this.http.post<AuthTokens>('/api/auth/refresh', { refreshToken }).pipe(
      tap(tokens => this.storeTokens(tokens)),
    );
  }

  private storeTokens(tokens: AuthTokens): void {
    localStorage.setItem('access_token', tokens.accessToken);
    localStorage.setItem('refresh_token', tokens.refreshToken);
    this.tokenPayload.set(jwtDecode<TokenPayload>(tokens.accessToken));
  }

  private clearTokens(): void {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    this.tokenPayload.set(null);
  }
}
```

### Auth Guard (Functional)

Angular 19 uses functional guards, not class-based guards:

```typescript
import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { AuthService } from './auth.service';

export const authGuard: CanActivateFn = () => {
  const auth = inject(AuthService);
  const router = inject(Router);

  if (auth.isAuthenticated()) {
    return true;
  }

  return router.createUrlTree(['/login'], {
    queryParams: { returnUrl: router.url },
  });
};

export const roleGuard = (requiredRole: string): CanActivateFn => {
  return () => {
    const auth = inject(AuthService);
    const router = inject(Router);

    if (auth.hasRole(requiredRole)()) {
      return true;
    }

    return router.createUrlTree(['/unauthorized']);
  };
};
```

### Auth Interceptor (Functional)

```typescript
import { HttpInterceptorFn, HttpErrorResponse } from '@angular/common/http';
import { inject } from '@angular/core';
import { AuthService } from '../auth/auth.service';
import { catchError, switchMap, throwError } from 'rxjs';

export const authInterceptor: HttpInterceptorFn = (req, next) => {
  const auth = inject(AuthService);
  const token = auth.getAccessToken();

  // Skip auth header for public endpoints
  if (req.url.includes('/api/auth/')) {
    return next(req);
  }

  const authReq = token
    ? req.clone({ setHeaders: { Authorization: `Bearer ${token}` } })
    : req;

  return next(authReq).pipe(
    catchError((error: HttpErrorResponse) => {
      if (error.status === 401) {
        return auth.refreshAccessToken().pipe(
          switchMap(tokens => {
            const retryReq = req.clone({
              setHeaders: { Authorization: `Bearer ${tokens.accessToken}` },
            });
            return next(retryReq);
          }),
          catchError(() => {
            auth.logout();
            return throwError(() => error);
          }),
        );
      }
      return throwError(() => error);
    }),
  );
};
```

### Route Configuration with Guards

```typescript
// app.routes.ts
import { Routes } from '@angular/router';
import { authGuard, roleGuard } from './core/auth/auth.guard';

export const routes: Routes = [
  {
    path: '',
    redirectTo: 'dashboard',
    pathMatch: 'full',
  },
  {
    path: 'login',
    loadComponent: () =>
      import('./features/auth/login.component').then(m => m.LoginComponent),
  },
  {
    path: 'dashboard',
    canActivate: [authGuard],
    loadComponent: () =>
      import('./features/dashboard/dashboard.component').then(m => m.DashboardComponent),
  },
  {
    path: 'users',
    canActivate: [authGuard, roleGuard('admin')],
    loadChildren: () =>
      import('./features/users/users.routes').then(m => m.USERS_ROUTES),
  },
  {
    path: 'settings',
    canActivate: [authGuard],
    loadChildren: () =>
      import('./features/settings/settings.routes').then(m => m.SETTINGS_ROUTES),
  },
  {
    path: '**',
    loadComponent: () =>
      import('./shared/components/not-found/not-found.component').then(m => m.NotFoundComponent),
  },
];
```

---

## 5. Angular Material & CDK

### Material Design 3 Theming

Angular Material 19 uses Material Design 3 (M3). Configure theming via Sass:

```scss
// styles/_material-theme.scss
@use '@angular/material' as mat;

// Define M3 theme with custom colors
$theme: mat.define-theme((
  color: (
    theme-type: light,
    primary: mat.$azure-palette,
    tertiary: mat.$blue-palette,
  ),
  typography: (
    brand-family: 'Inter, sans-serif',
    plain-family: 'Inter, sans-serif',
  ),
  density: (
    scale: 0,
  ),
));

$dark-theme: mat.define-theme((
  color: (
    theme-type: dark,
    primary: mat.$azure-palette,
    tertiary: mat.$blue-palette,
  ),
));

// Apply theme globally
html {
  @include mat.all-component-themes($theme);
  @include mat.typography-hierarchy($theme);

  // Dark mode via media query or class
  @media (prefers-color-scheme: dark) {
    @include mat.all-component-colors($dark-theme);
  }

  &.dark-mode {
    @include mat.all-component-colors($dark-theme);
  }
}
```

### Component Usage Philosophy

Use Angular Material for all standard UI. Do NOT build custom components when Material provides one. Only build custom components for domain-specific UI.

| Category | Material Components | Use For |
|---|---|---|
| **Forms** | `mat-form-field`, `mat-input`, `mat-select`, `mat-checkbox`, `mat-radio`, `mat-slide-toggle`, `mat-datepicker`, `mat-autocomplete` | All forms — registration, profile edit, admin |
| **Buttons** | `mat-button`, `mat-raised-button`, `mat-fab`, `mat-icon-button`, `mat-mini-fab` | Actions, CTAs, navigation |
| **Feedback** | `mat-snackbar`, `mat-progress-bar`, `mat-progress-spinner`, `mat-badge` | Status indicators, notifications |
| **Layout** | `mat-card`, `mat-expansion-panel`, `mat-divider`, `mat-toolbar`, `mat-sidenav` | Page sections, content grouping |
| **Navigation** | `mat-tab-group`, `mat-stepper`, `mat-paginator`, `mat-menu` | Page navigation, wizards, pagination |
| **Overlay** | `mat-dialog`, `mat-bottom-sheet`, `mat-tooltip`, `mat-menu` | Confirmations, menus, detail panels |
| **Data** | `mat-table`, `mat-sort`, `mat-paginator`, `mat-chip-set` | Data grids, lists, filters |
| **Misc** | `mat-icon`, `mat-list`, `mat-tree`, `mat-button-toggle` | Icons, navigation lists, hierarchies |

### CDK (Component Dev Kit) Patterns

The CDK provides low-level primitives for building custom components. Use CDK when Material does not have the exact component you need:

```typescript
// Virtual scrolling for large lists
import { ScrollingModule } from '@angular/cdk/scrolling';

@Component({
  selector: 'app-large-list',
  standalone: true,
  imports: [ScrollingModule],
  template: `
    <cdk-virtual-scroll-viewport itemSize="48" class="viewport">
      <div *cdkVirtualFor="let item of items()" class="item">
        {{ item.name }}
      </div>
    </cdk-virtual-scroll-viewport>
  `,
  styles: [`
    .viewport { height: 400px; }
    .item { height: 48px; }
  `],
})
export class LargeListComponent {
  items = input.required<Item[]>();
}
```

```typescript
// Drag and drop
import { CdkDragDrop, DragDropModule, moveItemInArray } from '@angular/cdk/drag-drop';

@Component({
  selector: 'app-sortable-list',
  standalone: true,
  imports: [DragDropModule],
  template: `
    <div cdkDropList (cdkDropListDropped)="drop($event)">
      @for (item of items(); track item.id) {
        <div cdkDrag class="drag-item">
          <mat-icon cdkDragHandle>drag_handle</mat-icon>
          {{ item.name }}
        </div>
      }
    </div>
  `,
})
export class SortableListComponent {
  items = signal<Item[]>([]);

  drop(event: CdkDragDrop<Item[]>): void {
    this.items.update(items => {
      const copy = [...items];
      moveItemInArray(copy, event.previousIndex, event.currentIndex);
      return copy;
    });
  }
}
```

```typescript
// Overlay (custom popover)
import { Overlay, OverlayRef } from '@angular/cdk/overlay';
import { ComponentPortal } from '@angular/cdk/portal';

@Injectable({ providedIn: 'root' })
export class PopoverService {
  private overlay = inject(Overlay);

  open<T>(component: Type<T>, origin: ElementRef): OverlayRef {
    const positionStrategy = this.overlay
      .position()
      .flexibleConnectedTo(origin)
      .withPositions([
        { originX: 'start', originY: 'bottom', overlayX: 'start', overlayY: 'top' },
      ]);

    const overlayRef = this.overlay.create({
      positionStrategy,
      hasBackdrop: true,
      backdropClass: 'cdk-overlay-transparent-backdrop',
    });

    overlayRef.attach(new ComponentPortal(component));
    overlayRef.backdropClick().subscribe(() => overlayRef.dispose());

    return overlayRef;
  }
}
```

### CDK Accessibility (a11y)

```typescript
import { A11yModule, LiveAnnouncer } from '@angular/cdk/a11y';

@Component({
  selector: 'app-notification',
  standalone: true,
  imports: [A11yModule],
  template: `
    <div cdkTrapFocus cdkTrapFocusAutoCapture>
      <!-- Focus is trapped inside this dialog-like component -->
      <h2>Notification</h2>
      <p>{{ message() }}</p>
      <button mat-button (click)="dismiss()">Dismiss</button>
    </div>
  `,
})
export class NotificationComponent {
  private announcer = inject(LiveAnnouncer);
  message = input.required<string>();

  ngOnInit(): void {
    // Announce to screen readers
    this.announcer.announce(this.message(), 'polite');
  }
}
```

---

## 6. Testing Patterns

### Test Pyramid (Angular-specific)

```
        /\
       /  \          E2E (Playwright)
      /    \         Full user flows, cross-page navigation
     /------\
    /        \        Integration Tests (Jest + TestBed)
   /          \       Component interactions, template rendering, services with HTTP
  /------------\
 /              \      Unit Tests (Jest)
/                \     Pure functions, pipes, services, store logic
/------------------\
```

### Jest Configuration (Karma Replacement)

Angular 19 projects use Jest instead of Karma. Configure via `jest.config.ts`:

```typescript
// jest.config.ts
import type { Config } from 'jest';

const config: Config = {
  preset: 'jest-preset-angular',
  setupFilesAfterSetup: ['<rootDir>/setup-jest.ts'],
  testPathIgnorePatterns: ['/node_modules/', '/e2e/'],
  collectCoverageFrom: [
    'src/app/**/*.ts',
    '!src/app/**/*.spec.ts',
    '!src/app/**/*.routes.ts',
    '!src/app/**/*.config.ts',
    '!src/app/**/*.model.ts',
    '!src/main.ts',
  ],
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
  moduleNameMapper: {
    '@app/(.*)': '<rootDir>/src/app/$1',
    '@env/(.*)': '<rootDir>/src/environments/$1',
  },
};

export default config;
```

```typescript
// setup-jest.ts
import 'jest-preset-angular/setup-jest';

// Mock window.matchMedia for Material components
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});
```

### Karma to Jest Migration Steps

If migrating an existing Karma/Jasmine project to Jest:

1. Remove Karma dependencies:
```bash
npm uninstall karma karma-chrome-launcher karma-coverage karma-jasmine \
  karma-jasmine-html-reporter @types/jasmine jasmine-core
rm karma.conf.js
```

2. Install Jest dependencies:
```bash
npm install --save-dev jest jest-preset-angular @types/jest ts-node
```

3. Update `tsconfig.spec.json`:
```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "./out-tsc/spec",
    "types": ["jest"],
    "esModuleInterop": true,
    "emitDecoratorMetadata": true
  },
  "include": ["src/**/*.spec.ts", "src/**/*.d.ts"]
}
```

4. Update `angular.json` — replace the `test` builder:
```json
{
  "test": {
    "builder": "@angular-builders/jest:run",
    "options": {
      "configPath": "jest.config.ts"
    }
  }
}
```

5. Migrate test syntax (Jasmine to Jest):
```typescript
// Jasmine (old)
beforeEach(waitForAsync(() => { ... }));
expect(value).toEqual(jasmine.objectContaining({ ... }));
spyOn(service, 'method').and.returnValue(of(mockData));

// Jest (new)
beforeEach(async () => { ... });
expect(value).toEqual(expect.objectContaining({ ... }));
jest.spyOn(service, 'method').mockReturnValue(of(mockData));
```

### Component Testing

```typescript
import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideNoopAnimations } from '@angular/platform-browser/animations';
import { UserCardComponent } from './user-card.component';

describe('UserCardComponent', () => {
  let component: UserCardComponent;
  let fixture: ComponentFixture<UserCardComponent>;

  const mockUser: User = {
    id: '1',
    name: 'John Doe',
    email: 'john@example.com',
    role: 'admin',
    createdAt: new Date('2024-01-15'),
  };

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UserCardComponent],
      providers: [provideNoopAnimations()],
    }).compileComponents();

    fixture = TestBed.createComponent(UserCardComponent);
    component = fixture.componentInstance;

    // Set required signal inputs via ComponentRef
    fixture.componentRef.setInput('user', mockUser);
    fixture.detectChanges();
  });

  it('should display user name', () => {
    const title = fixture.nativeElement.querySelector('mat-card-title');
    expect(title.textContent).toContain('John Doe');
  });

  it('should emit edit event when edit button clicked', () => {
    const editSpy = jest.fn();
    component.edit.subscribe(editSpy);

    const editButton = fixture.nativeElement.querySelector('button');
    editButton.click();

    expect(editSpy).toHaveBeenCalledWith(mockUser);
  });

  it('should display formatted role', () => {
    expect(component.formattedRole()).toBe('Admin');
  });
});
```

### Service Testing

```typescript
import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting, HttpTestingController } from '@angular/common/http/testing';
import { UserService } from './user.service';

describe('UserService', () => {
  let service: UserService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        UserService,
        provideHttpClient(),
        provideHttpClientTesting(),
      ],
    });

    service = TestBed.inject(UserService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify(); // Ensures no unmatched requests
  });

  it('should fetch users', () => {
    const mockUsers: User[] = [
      { id: '1', name: 'John', email: 'john@example.com', role: 'admin', createdAt: new Date() },
    ];

    service.getUsers().subscribe(users => {
      expect(users).toEqual(mockUsers);
    });

    const req = httpMock.expectOne('/api/users');
    expect(req.request.method).toBe('GET');
    req.flush(mockUsers);
  });

  it('should handle error when fetching users', () => {
    service.getUsers().subscribe({
      error: (error) => {
        expect(error.status).toBe(500);
      },
    });

    const req = httpMock.expectOne('/api/users');
    req.flush('Server error', { status: 500, statusText: 'Internal Server Error' });
  });
});
```

### Signal Store Testing

```typescript
import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting, HttpTestingController } from '@angular/common/http/testing';
import { UsersStore } from './users.store';

describe('UsersStore', () => {
  let store: InstanceType<typeof UsersStore>;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [
        UsersStore,
        provideHttpClient(),
        provideHttpClientTesting(),
      ],
    });

    store = TestBed.inject(UsersStore);
    httpMock = TestBed.inject(HttpTestingController);
  });

  it('should start with empty users', () => {
    expect(store.users()).toEqual([]);
    expect(store.loading()).toBe(false);
  });

  it('should load users', () => {
    const mockUsers = [{ id: '1', name: 'John' }];

    store.loadUsers();

    const req = httpMock.expectOne('/api/users');
    req.flush(mockUsers);

    expect(store.users()).toEqual(mockUsers);
    expect(store.loading()).toBe(false);
  });

  it('should filter users by search term', () => {
    // Populate store
    store.loadUsers();
    httpMock.expectOne('/api/users').flush([
      { id: '1', name: 'John' },
      { id: '2', name: 'Jane' },
    ]);

    store.setFilter({ search: 'John' });

    expect(store.filteredUsers()).toEqual([{ id: '1', name: 'John' }]);
  });
});
```

### Playwright E2E Testing

```typescript
// playwright.config.ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e/specs',
  fullyParallel: true,
  forbidOnly: !!process.env['CI'],
  retries: process.env['CI'] ? 2 : 0,
  workers: process.env['CI'] ? 1 : undefined,
  reporter: [
    ['html'],
    ['junit', { outputFile: 'test-results/e2e-results.xml' }],
  ],
  use: {
    baseURL: 'http://localhost:4200',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
    { name: 'mobile-chrome', use: { ...devices['Pixel 5'] } },
  ],
  webServer: {
    command: 'ng serve',
    url: 'http://localhost:4200',
    reuseExistingServer: !process.env['CI'],
  },
});
```

### Page Object Model for E2E

```typescript
// e2e/pages/login.page.ts
import { Page, Locator } from '@playwright/test';

export class LoginPage {
  readonly emailInput: Locator;
  readonly passwordInput: Locator;
  readonly submitButton: Locator;
  readonly errorMessage: Locator;

  constructor(private readonly page: Page) {
    this.emailInput = page.getByLabel('Email');
    this.passwordInput = page.getByLabel('Password');
    this.submitButton = page.getByRole('button', { name: 'Log In' });
    this.errorMessage = page.getByRole('alert');
  }

  async goto(): Promise<void> {
    await this.page.goto('/login');
  }

  async login(email: string, password: string): Promise<void> {
    await this.emailInput.fill(email);
    await this.passwordInput.fill(password);
    await this.submitButton.click();
  }
}

// e2e/pages/dashboard.page.ts
export class DashboardPage {
  readonly heading: Locator;
  readonly statsCards: Locator;
  readonly userCount: Locator;

  constructor(private readonly page: Page) {
    this.heading = page.getByRole('heading', { name: 'Dashboard' });
    this.statsCards = page.locator('app-stats-card');
    this.userCount = page.getByTestId('user-count');
  }

  async waitForLoad(): Promise<void> {
    await this.heading.waitFor();
  }
}
```

```typescript
// e2e/specs/auth.spec.ts
import { test, expect } from '@playwright/test';
import { LoginPage } from '../pages/login.page';
import { DashboardPage } from '../pages/dashboard.page';

test.describe('Authentication', () => {
  test('should login with valid credentials', async ({ page }) => {
    const loginPage = new LoginPage(page);
    const dashboardPage = new DashboardPage(page);

    await loginPage.goto();
    await loginPage.login('admin@example.com', 'password123');

    await dashboardPage.waitForLoad();
    await expect(dashboardPage.heading).toBeVisible();
  });

  test('should show error for invalid credentials', async ({ page }) => {
    const loginPage = new LoginPage(page);

    await loginPage.goto();
    await loginPage.login('wrong@example.com', 'wrong');

    await expect(loginPage.errorMessage).toContainText('Invalid credentials');
  });

  test('should redirect to login when unauthenticated', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page).toHaveURL(/\/login/);
  });
});
```

### Cypress Alternative (When Cypress Is Preferred)

If the project uses Cypress instead of Playwright:

```typescript
// cypress.config.ts
import { defineConfig } from 'cypress';

export default defineConfig({
  e2e: {
    baseUrl: 'http://localhost:4200',
    specPattern: 'cypress/e2e/**/*.cy.ts',
    supportFile: 'cypress/support/e2e.ts',
    viewportWidth: 1280,
    viewportHeight: 720,
    video: false,
    screenshotOnRunFailure: true,
    retries: { runMode: 2, openMode: 0 },
  },
  component: {
    devServer: {
      framework: 'angular',
      bundler: 'webpack',
    },
    specPattern: 'src/**/*.cy.ts',
  },
});
```

```typescript
// cypress/e2e/auth.cy.ts
describe('Authentication', () => {
  it('should login with valid credentials', () => {
    cy.visit('/login');
    cy.findByLabelText('Email').type('admin@example.com');
    cy.findByLabelText('Password').type('password123');
    cy.findByRole('button', { name: 'Log In' }).click();

    cy.findByRole('heading', { name: 'Dashboard' }).should('be.visible');
    cy.url().should('include', '/dashboard');
  });

  it('should show error for invalid credentials', () => {
    cy.visit('/login');
    cy.findByLabelText('Email').type('wrong@example.com');
    cy.findByLabelText('Password').type('wrong');
    cy.findByRole('button', { name: 'Log In' }).click();

    cy.findByRole('alert').should('contain.text', 'Invalid credentials');
  });
});
```

**Recommendation:** Prefer Playwright for new projects. It is faster, more reliable, and has first-class support for multiple browsers. Use Cypress only if the team already has extensive Cypress infrastructure.

---

## 7. State Management — NgRx Signal Store

### Why Signal Store

Angular 19 projects use `@ngrx/signals` (Signal Store), not the classic `@ngrx/store` (Redux pattern). Signal Store is lighter, type-safe, and integrates natively with Angular Signals:

| Feature | Classic NgRx Store | NgRx Signal Store |
|---|---|---|
| Boilerplate | High (actions, reducers, effects, selectors) | Low (single store file) |
| Reactivity | RxJS-based | Signal-based |
| Type Safety | Manual with action creators | Automatic with `signalStore()` |
| Learning Curve | Steep | Moderate |
| Best For | Very large apps with complex state | Most Angular apps |

### Signal Store Pattern

```typescript
// features/users/store/users.store.ts
import {
  signalStore,
  withState,
  withComputed,
  withMethods,
  patchState,
  withHooks,
} from '@ngrx/signals';
import { rxMethod } from '@ngrx/signals/rxjs-interop';
import { computed, inject } from '@angular/core';
import { pipe, switchMap, tap, catchError, EMPTY } from 'rxjs';
import { tapResponse } from '@ngrx/operators';
import { UserService } from '../services/user.service';

// State interface
interface UsersState {
  users: User[];
  loading: boolean;
  error: string | null;
  filter: UserFilter;
  selectedId: string | null;
}

// Initial state
const initialState: UsersState = {
  users: [],
  loading: false,
  error: null,
  filter: { search: '', role: null },
  selectedId: null,
};

export const UsersStore = signalStore(
  { providedIn: 'root' },

  withState(initialState),

  withComputed(({ users, filter, selectedId }) => ({
    filteredUsers: computed(() => {
      const allUsers = users();
      const { search, role } = filter();

      return allUsers.filter(user => {
        const matchesSearch = !search ||
          user.name.toLowerCase().includes(search.toLowerCase()) ||
          user.email.toLowerCase().includes(search.toLowerCase());
        const matchesRole = !role || user.role === role;
        return matchesSearch && matchesRole;
      });
    }),

    selectedUser: computed(() => {
      const id = selectedId();
      return id ? users().find(u => u.id === id) ?? null : null;
    }),

    totalCount: computed(() => users().length),
  })),

  withMethods((store, userService = inject(UserService)) => ({
    setFilter(filter: Partial<UserFilter>): void {
      patchState(store, state => ({
        filter: { ...state.filter, ...filter },
      }));
    },

    selectUser(id: string | null): void {
      patchState(store, { selectedId: id });
    },

    loadUsers: rxMethod<void>(
      pipe(
        tap(() => patchState(store, { loading: true, error: null })),
        switchMap(() =>
          userService.getUsers().pipe(
            tapResponse({
              next: (users) => patchState(store, { users, loading: false }),
              error: (error: Error) =>
                patchState(store, { loading: false, error: error.message }),
            }),
          ),
        ),
      ),
    ),

    deleteUser: rxMethod<string>(
      pipe(
        switchMap((id) =>
          userService.deleteUser(id).pipe(
            tapResponse({
              next: () =>
                patchState(store, state => ({
                  users: state.users.filter(u => u.id !== id),
                })),
              error: (error: Error) =>
                patchState(store, { error: error.message }),
            }),
          ),
        ),
      ),
    ),

    updateUser: rxMethod<{ id: string; data: Partial<User> }>(
      pipe(
        switchMap(({ id, data }) =>
          userService.updateUser(id, data).pipe(
            tapResponse({
              next: (updated) =>
                patchState(store, state => ({
                  users: state.users.map(u => (u.id === id ? updated : u)),
                })),
              error: (error: Error) =>
                patchState(store, { error: error.message }),
            }),
          ),
        ),
      ),
    ),
  })),

  withHooks({
    onInit(store) {
      store.loadUsers();
    },
  }),
);
```

### Store Feature Composition

Break large stores into reusable features:

```typescript
// shared/store/with-loading.ts
import { signalStoreFeature, withState, withComputed, withMethods, patchState } from '@ngrx/signals';
import { computed } from '@angular/core';

export function withLoading() {
  return signalStoreFeature(
    withState({ loading: false, error: null as string | null }),

    withComputed(({ loading, error }) => ({
      hasError: computed(() => error() !== null),
    })),

    withMethods((store) => ({
      setLoading(): void {
        patchState(store, { loading: true, error: null });
      },
      setLoaded(): void {
        patchState(store, { loading: false });
      },
      setError(message: string): void {
        patchState(store, { loading: false, error: message });
      },
    })),
  );
}

// shared/store/with-pagination.ts
export function withPagination<T>() {
  return signalStoreFeature(
    withState({
      currentPage: 1,
      pageSize: 25,
      totalItems: 0,
    }),

    withComputed(({ currentPage, pageSize, totalItems }) => ({
      totalPages: computed(() => Math.ceil(totalItems() / pageSize())),
      hasNextPage: computed(() => currentPage() < Math.ceil(totalItems() / pageSize())),
      hasPrevPage: computed(() => currentPage() > 1),
    })),

    withMethods((store) => ({
      setPage(page: number): void {
        patchState(store, { currentPage: page });
      },
      setPageSize(size: number): void {
        patchState(store, { pageSize: size, currentPage: 1 });
      },
    })),
  );
}

// Usage in a feature store
export const UsersStore = signalStore(
  { providedIn: 'root' },
  withState({ users: [] as User[] }),
  withLoading(),
  withPagination(),
  withMethods((store) => ({
    // store now has loading(), setLoading(), currentPage(), etc.
  })),
);
```

### When to Use Classic NgRx Store

Use the classic `@ngrx/store` (actions, reducers, effects, selectors) only when:
- The application has deeply shared global state across many unrelated features
- You need Redux DevTools time-travel debugging across the entire app
- The team has extensive NgRx/Redux experience and prefers the pattern
- You need `@ngrx/entity` for normalized collections with hundreds of thousands of records

For most Angular 19 applications, Signal Store is the correct choice.

---

## 8. Reactive Forms — Typed Forms

### Typed Reactive Forms (Angular 14+)

All forms use strictly typed `FormGroup` and `FormControl`. Never use untyped `FormGroup`:

```typescript
import { Component, inject } from '@angular/core';
import {
  FormBuilder,
  FormGroup,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';

interface UserForm {
  name: FormControl<string>;
  email: FormControl<string>;
  role: FormControl<'admin' | 'editor' | 'viewer'>;
  bio: FormControl<string | null>;
}

@Component({
  selector: 'app-user-form',
  standalone: true,
  imports: [
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatButtonModule,
  ],
  template: `
    <form [formGroup]="form" (ngSubmit)="onSubmit()" novalidate>
      <mat-form-field appearance="outline">
        <mat-label>Name</mat-label>
        <input matInput formControlName="name" autocomplete="name" />
        @if (form.controls.name.hasError('required')) {
          <mat-error>Name is required</mat-error>
        }
        @if (form.controls.name.hasError('minlength')) {
          <mat-error>Name must be at least 2 characters</mat-error>
        }
      </mat-form-field>

      <mat-form-field appearance="outline">
        <mat-label>Email</mat-label>
        <input matInput formControlName="email" type="email" autocomplete="email" />
        @if (form.controls.name.hasError('required')) {
          <mat-error>Email is required</mat-error>
        }
        @if (form.controls.email.hasError('email')) {
          <mat-error>Enter a valid email address</mat-error>
        }
      </mat-form-field>

      <mat-form-field appearance="outline">
        <mat-label>Role</mat-label>
        <mat-select formControlName="role">
          <mat-option value="admin">Admin</mat-option>
          <mat-option value="editor">Editor</mat-option>
          <mat-option value="viewer">Viewer</mat-option>
        </mat-select>
      </mat-form-field>

      <mat-form-field appearance="outline">
        <mat-label>Bio (optional)</mat-label>
        <textarea matInput formControlName="bio" rows="4"></textarea>
      </mat-form-field>

      <button mat-raised-button color="primary" type="submit"
              [disabled]="form.invalid || submitting()">
        @if (submitting()) {
          <mat-spinner diameter="20" />
        } @else {
          Save Profile
        }
      </button>
    </form>
  `,
})
export class UserFormComponent {
  private fb = inject(FormBuilder);

  submitting = signal(false);

  form: FormGroup<UserForm> = this.fb.nonNullable.group({
    name: ['', [Validators.required, Validators.minLength(2)]],
    email: ['', [Validators.required, Validators.email]],
    role: ['viewer' as const, Validators.required],
    bio: [null as string | null],
  });

  onSubmit(): void {
    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }

    // Type-safe access — TypeScript knows the exact shape
    const value = this.form.getRawValue();
    // value.name is string, value.email is string, value.role is 'admin' | 'editor' | 'viewer'
    this.submitting.set(true);
    // ... submit logic
  }
}
```

### Dynamic Form Arrays (Typed)

```typescript
import { FormArray, FormControl, FormGroup, FormBuilder } from '@angular/forms';

interface AddressForm {
  street: FormControl<string>;
  city: FormControl<string>;
  state: FormControl<string>;
  zip: FormControl<string>;
}

interface ProfileForm {
  name: FormControl<string>;
  addresses: FormArray<FormGroup<AddressForm>>;
}

@Component({
  selector: 'app-profile-form',
  standalone: true,
  imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatIconModule],
  template: `
    <form [formGroup]="form" (ngSubmit)="onSubmit()" novalidate>
      <mat-form-field appearance="outline">
        <mat-label>Name</mat-label>
        <input matInput formControlName="name" />
      </mat-form-field>

      <div formArrayName="addresses">
        @for (address of form.controls.addresses.controls; track $index; let i = $index) {
          <div [formGroupName]="i" class="address-group">
            <h3>Address {{ i + 1 }}</h3>
            <mat-form-field appearance="outline">
              <mat-label>Street</mat-label>
              <input matInput formControlName="street" />
            </mat-form-field>
            <mat-form-field appearance="outline">
              <mat-label>City</mat-label>
              <input matInput formControlName="city" />
            </mat-form-field>
            <button mat-icon-button type="button" (click)="removeAddress(i)">
              <mat-icon>delete</mat-icon>
            </button>
          </div>
        }
      </div>

      <button mat-stroked-button type="button" (click)="addAddress()">
        <mat-icon>add</mat-icon> Add Address
      </button>

      <button mat-raised-button color="primary" type="submit">Save</button>
    </form>
  `,
})
export class ProfileFormComponent {
  private fb = inject(FormBuilder);

  form = this.fb.nonNullable.group<ProfileForm>({
    name: this.fb.nonNullable.control('', Validators.required),
    addresses: this.fb.array<FormGroup<AddressForm>>([]),
  });

  addAddress(): void {
    const addressGroup = this.fb.nonNullable.group<AddressForm>({
      street: this.fb.nonNullable.control('', Validators.required),
      city: this.fb.nonNullable.control('', Validators.required),
      state: this.fb.nonNullable.control('', Validators.required),
      zip: this.fb.nonNullable.control('', [Validators.required, Validators.pattern(/^\d{5}$/)]),
    });
    this.form.controls.addresses.push(addressGroup);
  }

  removeAddress(index: number): void {
    this.form.controls.addresses.removeAt(index);
  }
}
```

### Custom Validators (Typed)

```typescript
// shared/validators/match-fields.validator.ts
import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';

export function matchFields(field1: string, field2: string): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    const value1 = control.get(field1)?.value;
    const value2 = control.get(field2)?.value;

    if (value1 !== value2) {
      control.get(field2)?.setErrors({ mismatch: true });
      return { mismatch: true };
    }
    return null;
  };
}

// Async validator for unique email check
export function uniqueEmailValidator(userService: UserService): AsyncValidatorFn {
  return (control: AbstractControl): Observable<ValidationErrors | null> => {
    if (!control.value) return of(null);

    return userService.checkEmailExists(control.value).pipe(
      map(exists => (exists ? { emailTaken: true } : null)),
      catchError(() => of(null)),
      debounceTime(300),
      first(),
    );
  };
}
```

---

## 9. HTTP & API Patterns

### API Service Layer

All HTTP calls go through typed service methods. Never call `HttpClient` directly from components:

```typescript
// core/http/api.service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '@env/environment';

@Injectable({ providedIn: 'root' })
export class ApiService {
  private readonly http = inject(HttpClient);
  private readonly baseUrl = environment.apiUrl;

  get<T>(path: string, params?: Record<string, string | number>): Observable<T> {
    let httpParams = new HttpParams();
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        httpParams = httpParams.set(key, String(value));
      });
    }
    return this.http.get<T>(`${this.baseUrl}${path}`, { params: httpParams });
  }

  post<T>(path: string, body: unknown): Observable<T> {
    return this.http.post<T>(`${this.baseUrl}${path}`, body);
  }

  put<T>(path: string, body: unknown): Observable<T> {
    return this.http.put<T>(`${this.baseUrl}${path}`, body);
  }

  patch<T>(path: string, body: unknown): Observable<T> {
    return this.http.patch<T>(`${this.baseUrl}${path}`, body);
  }

  delete<T>(path: string): Observable<T> {
    return this.http.delete<T>(`${this.baseUrl}${path}`);
  }
}

// features/users/services/user.service.ts
@Injectable({ providedIn: 'root' })
export class UserService {
  private readonly api = inject(ApiService);

  getUsers(params?: { page?: number; size?: number }): Observable<User[]> {
    return this.api.get<User[]>('/users', params);
  }

  getUser(id: string): Observable<User> {
    return this.api.get<User>(`/users/${id}`);
  }

  createUser(data: CreateUserDto): Observable<User> {
    return this.api.post<User>('/users', data);
  }

  updateUser(id: string, data: Partial<User>): Observable<User> {
    return this.api.patch<User>(`/users/${id}`, data);
  }

  deleteUser(id: string): Observable<void> {
    return this.api.delete<void>(`/users/${id}`);
  }

  checkEmailExists(email: string): Observable<boolean> {
    return this.api.get<boolean>('/users/check-email', { email });
  }
}
```

### Error Interceptor

```typescript
// core/http/error.interceptor.ts
import { HttpInterceptorFn, HttpErrorResponse } from '@angular/common/http';
import { inject } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import { catchError, throwError } from 'rxjs';

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
  const snackBar = inject(MatSnackBar);

  return next(req).pipe(
    catchError((error: HttpErrorResponse) => {
      let message: string;

      switch (error.status) {
        case 0:
          message = 'Unable to connect to server. Check your internet connection.';
          break;
        case 400:
          message = error.error?.message ?? 'Invalid request. Please check your input.';
          break;
        case 403:
          message = 'You do not have permission to perform this action.';
          break;
        case 404:
          message = 'The requested resource was not found.';
          break;
        case 422:
          message = error.error?.message ?? 'Validation failed.';
          break;
        case 429:
          message = 'Too many requests. Please try again later.';
          break;
        case 500:
        case 502:
        case 503:
          message = 'Server error. Please try again later.';
          break;
        default:
          message = 'An unexpected error occurred.';
      }

      // Don't show snackbar for 401 — auth interceptor handles that
      if (error.status !== 401) {
        snackBar.open(message, 'Dismiss', {
          duration: 5000,
          panelClass: ['error-snackbar'],
        });
      }

      return throwError(() => error);
    }),
  );
};
```

### Provider Configuration

```typescript
// app.config.ts
import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideRouter, withComponentInputBinding, withViewTransitions } from '@angular/router';
import { provideHttpClient, withInterceptors, withFetch } from '@angular/common/http';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { routes } from './app.routes';
import { authInterceptor } from './core/auth/auth.interceptor';
import { errorInterceptor } from './core/http/error.interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes, withComponentInputBinding(), withViewTransitions()),
    provideHttpClient(
      withFetch(),
      withInterceptors([authInterceptor, errorInterceptor]),
    ),
    provideAnimationsAsync(),
  ],
};
```

---

## 10. Routing & Navigation

### Lazy Loading

All feature routes are lazy-loaded. Never eagerly import feature components in the root route config:

```typescript
// app.routes.ts
export const routes: Routes = [
  {
    path: '',
    redirectTo: 'dashboard',
    pathMatch: 'full',
  },
  {
    path: 'dashboard',
    canActivate: [authGuard],
    loadComponent: () =>
      import('./features/dashboard/dashboard.component').then(m => m.DashboardComponent),
  },
  {
    path: 'users',
    canActivate: [authGuard],
    loadChildren: () =>
      import('./features/users/users.routes').then(m => m.USERS_ROUTES),
  },
];
```

```typescript
// features/users/users.routes.ts
import { Routes } from '@angular/router';
import { roleGuard } from '../../core/auth/auth.guard';

export const USERS_ROUTES: Routes = [
  {
    path: '',
    loadComponent: () =>
      import('./user-list/user-list.component').then(m => m.UserListPageComponent),
  },
  {
    path: 'new',
    canActivate: [roleGuard('admin')],
    loadComponent: () =>
      import('./user-form/user-form.component').then(m => m.UserFormComponent),
  },
  {
    path: ':id',
    loadComponent: () =>
      import('./user-detail/user-detail.component').then(m => m.UserDetailComponent),
  },
  {
    path: ':id/edit',
    canActivate: [roleGuard('admin')],
    loadComponent: () =>
      import('./user-form/user-form.component').then(m => m.UserFormComponent),
  },
];
```

### Route Parameter Binding (Component Input Binding)

With `withComponentInputBinding()` enabled in the router config, route params are automatically bound to component inputs:

```typescript
// The router automatically maps :id to the id input
@Component({
  selector: 'app-user-detail',
  standalone: true,
  template: `<!-- ... -->`,
})
export class UserDetailComponent {
  // Automatically populated from route param :id
  id = input.required<string>();

  // Also works with query params and route data
  // ?returnUrl=/users → returnUrl input
  returnUrl = input<string>();
}
```

### Route Resolvers (Functional)

```typescript
import { ResolveFn } from '@angular/router';
import { inject } from '@angular/core';
import { UserService } from '../services/user.service';

export const userResolver: ResolveFn<User> = (route) => {
  const userService = inject(UserService);
  return userService.getUser(route.paramMap.get('id')!);
};

// Usage in routes
{
  path: ':id',
  resolve: { user: userResolver },
  loadComponent: () =>
    import('./user-detail/user-detail.component').then(m => m.UserDetailComponent),
}

// In the component — resolved data comes via input
@Component({ /* ... */ })
export class UserDetailComponent {
  user = input.required<User>(); // Populated by resolver via component input binding
}
```

### View Transitions

Angular 19 supports the View Transitions API for smooth page transitions:

```typescript
// Enabled via withViewTransitions() in router config

// Customize transitions per route
{
  path: 'users/:id',
  loadComponent: () => import('./user-detail.component'),
  data: { animation: 'detail-page' },
}
```

```css
/* Global view transition styles */
::view-transition-old(root) {
  animation: fade-out 0.2s ease-out;
}

::view-transition-new(root) {
  animation: fade-in 0.3s ease-in;
}

@keyframes fade-out {
  from { opacity: 1; }
  to { opacity: 0; }
}

@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}
```

---

## 11. Development Workflow

### Feature Development Cycle (Angular-specific)

```
1. Write acceptance criteria (BDD scenarios)
2. Design test levels (unit / integration / E2E)
3. Write failing tests (Jest)
4. Write service / store code
5. Write component (using Angular Material)
6. Run: ng test
7. Run: ng lint
8. Refactor while green
9. Run: npx playwright test (E2E)
```

### Common Commands

```bash
# Development
ng serve                                    # Start dev server (http://localhost:4200)
ng serve --open                             # Start and open browser
ng test                                     # Run unit tests (Jest, watch mode)
ng test --no-watch --code-coverage          # Run tests once with coverage
ng lint                                     # ESLint
ng build                                    # Production build

# Code generation
ng generate component features/users/user-card --standalone
ng generate service features/users/services/user
ng generate guard core/auth/auth --functional
ng generate pipe shared/pipes/truncate --standalone
ng generate directive shared/directives/auto-focus --standalone

# E2E (Playwright)
npx playwright test                         # Run all E2E tests
npx playwright test --ui                    # Open Playwright UI
npx playwright test --headed                # Run with browser visible
npx playwright show-report                  # View HTML report

# Quality
ng lint --fix                               # Auto-fix lint issues
npx prettier --write "src/**/*.{ts,html,scss}"  # Format code
npm audit                                   # Dependency vulnerability check

# Build analysis
ng build --stats-json                       # Generate stats for bundle analysis
npx webpack-bundle-analyzer dist/*/stats.json  # Visualize bundle
```

### ESLint Configuration (Flat Config)

```typescript
// eslint.config.js
import angular from '@angular-eslint/eslint-plugin';
import angularTemplate from '@angular-eslint/eslint-plugin-template';
import tsParser from '@typescript-eslint/parser';
import templateParser from '@angular-eslint/template-parser';

export default [
  {
    files: ['**/*.ts'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: './tsconfig.json',
      },
    },
    plugins: {
      '@angular-eslint': angular,
    },
    rules: {
      '@angular-eslint/component-selector': ['error', {
        type: 'element',
        prefix: 'app',
        style: 'kebab-case',
      }],
      '@angular-eslint/directive-selector': ['error', {
        type: 'attribute',
        prefix: 'app',
        style: 'camelCase',
      }],
      '@angular-eslint/prefer-standalone': 'error',
      '@angular-eslint/prefer-signals': 'warn',
      '@angular-eslint/no-empty-lifecycle-method': 'error',
      'no-console': ['warn', { allow: ['warn', 'error'] }],
    },
  },
  {
    files: ['**/*.html'],
    languageOptions: {
      parser: templateParser,
    },
    plugins: {
      '@angular-eslint/template': angularTemplate,
    },
    rules: {
      '@angular-eslint/template/banana-in-box': 'error',
      '@angular-eslint/template/no-negated-async': 'error',
      '@angular-eslint/template/eqeqeq': 'error',
      '@angular-eslint/template/accessibility-alt-text': 'error',
      '@angular-eslint/template/accessibility-label-for': 'error',
      '@angular-eslint/template/click-events-have-key-events': 'error',
      '@angular-eslint/template/mouse-events-have-key-events': 'error',
      '@angular-eslint/template/no-autofocus': 'warn',
    },
  },
];
```

---

## 12. Server-Side Rendering (SSR) with Angular Universal

### SSR Setup (Angular 19)

Angular 19 uses `@angular/ssr` (successor to Angular Universal). SSR is configured during project creation or added later:

```bash
ng add @angular/ssr
```

This generates:
- `src/app/app.config.server.ts` — server-specific providers
- `src/main.server.ts` — server entry point
- `server.ts` — Express server

### Server Configuration

```typescript
// src/app/app.config.server.ts
import { mergeApplicationConfig, ApplicationConfig } from '@angular/core';
import { provideServerRendering } from '@angular/platform-server';
import { provideServerRoutesConfig } from '@angular/ssr';
import { appConfig } from './app.config';
import { serverRoutes } from './app.routes.server';

const serverConfig: ApplicationConfig = {
  providers: [
    provideServerRendering(),
    provideServerRoutesConfig(serverRoutes),
  ],
};

export const config = mergeApplicationConfig(appConfig, serverConfig);
```

```typescript
// src/app/app.routes.server.ts
import { RenderMode, ServerRoute } from '@angular/ssr';

export const serverRoutes: ServerRoute[] = [
  {
    path: '',
    renderMode: RenderMode.Prerender,  // Static at build time
  },
  {
    path: 'login',
    renderMode: RenderMode.Server,     // Dynamic at request time
  },
  {
    path: 'dashboard',
    renderMode: RenderMode.Client,     // Client-only (skip SSR)
  },
  {
    path: 'users/:id',
    renderMode: RenderMode.Server,     // Dynamic with route params
  },
  {
    path: '**',
    renderMode: RenderMode.Server,     // Default for unmatched routes
  },
];
```

### SSR-Safe Code Patterns

Code that accesses browser-only APIs must be guarded:

```typescript
import { Component, inject, PLATFORM_ID, afterNextRender } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

@Component({
  selector: 'app-chart',
  standalone: true,
  template: `
    <div #chartContainer class="chart-container">
      @if (!chartReady()) {
        <mat-spinner />
      }
    </div>
  `,
})
export class ChartComponent {
  private platformId = inject(PLATFORM_ID);
  chartReady = signal(false);

  constructor() {
    // afterNextRender only runs in the browser, never on the server
    afterNextRender(() => {
      this.initChart();
      this.chartReady.set(true);
    });
  }

  private initChart(): void {
    // Safe — this code only runs in the browser
    const canvas = document.querySelector('.chart-container');
    // ... D3 or Chart.js initialization
  }
}
```

**SSR Rules:**
- Never access `window`, `document`, `localStorage`, `sessionStorage` directly — guard with `isPlatformBrowser()` or use `afterNextRender()`
- Use `afterNextRender()` for DOM manipulation and browser-only library initialization
- Use `afterRender()` for code that should run after every render (rare)
- Use `TransferState` to avoid duplicate API calls (server fetches, client reuses)
- Set render mode per route — not everything needs SSR
- Use `RenderMode.Prerender` for static content (landing pages, about, docs)
- Use `RenderMode.Client` for fully interactive pages that don't need SEO (dashboards, admin panels)

### Transfer State (Avoid Double Fetching)

```typescript
import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { TransferState, makeStateKey } from '@angular/core';
import { Observable, of, tap } from 'rxjs';
import { isPlatformServer } from '@angular/common';

const USERS_KEY = makeStateKey<User[]>('users');

@Injectable({ providedIn: 'root' })
export class UserService {
  private http = inject(HttpClient);
  private transferState = inject(TransferState);
  private platformId = inject(PLATFORM_ID);

  getUsers(): Observable<User[]> {
    // Check if data was already fetched during SSR
    const cached = this.transferState.get(USERS_KEY, null);
    if (cached) {
      this.transferState.remove(USERS_KEY); // Use once
      return of(cached);
    }

    return this.http.get<User[]>('/api/users').pipe(
      tap(users => {
        if (isPlatformServer(this.platformId)) {
          // Store for transfer to client
          this.transferState.set(USERS_KEY, users);
        }
      }),
    );
  }
}
```

---

## 13. Security

### Security Headers

Configure security headers on the server (Express for SSR, or reverse proxy):

```typescript
// server.ts — Express SSR server
import helmet from 'helmet';

app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      scriptSrc: ["'self'"],
      styleSrc: ["'self'", "'unsafe-inline'"],  // Required for Angular Material
      imgSrc: ["'self'", 'data:', 'https:'],
      fontSrc: ["'self'", 'https://fonts.gstatic.com'],
      connectSrc: ["'self'", environment.apiUrl],
      frameSrc: ["'none'"],
      frameAncestors: ["'self'"],
      objectSrc: ["'none'"],
      baseUri: ["'self'"],
    },
  },
  hsts: {
    maxAge: 31536000,
    includeSubDomains: true,
    preload: true,
  },
  referrerPolicy: { policy: 'strict-origin-when-cross-origin' },
}));
```

**Notes:**
- `unsafe-inline` for `styleSrc` is required for Angular Material's dynamic styles
- Use nonce-based CSP for scripts when possible (Angular CLI supports `ngCspNonce`)
- HSTS is set to 1 year with `includeSubDomains` and `preload`
- Frame embedding is denied by default (`frameSrc: 'none'`, `frameAncestors: 'self'`)

### Angular-Specific Security

```typescript
// CSP nonce for inline scripts (Angular 19)
// In index.html:
// <script nonce="random-nonce">...</script>

// In app.config.ts — tell Angular about the nonce
import { CSP_NONCE } from '@angular/core';

providers: [
  { provide: CSP_NONCE, useValue: 'random-nonce' },  // Set by server
]
```

### XSS Prevention

Angular auto-escapes template interpolation (`{{ }}`). Additional rules:

```typescript
// NEVER bypass sanitization unless absolutely necessary
// BAD — opens XSS vector
@Component({
  template: `<div [innerHTML]="untrustedHtml"></div>`,
})

// If you must render HTML, sanitize it explicitly
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';

@Component({
  template: `<div [innerHTML]="safeHtml()"></div>`,
})
export class RichContentComponent {
  private sanitizer = inject(DomSanitizer);
  rawHtml = input.required<string>();

  safeHtml = computed((): SafeHtml => {
    // Only use bypassSecurityTrustHtml for content YOU control
    // For user content, rely on Angular's built-in sanitization
    return this.sanitizer.sanitize(SecurityContext.HTML, this.rawHtml()) ?? '';
  });
}
```

### CSRF Protection

```typescript
// Angular's HttpClient automatically reads XSRF tokens from cookies
// Configure the cookie and header names to match your backend:

import { provideHttpClient, withXsrfConfiguration } from '@angular/common/http';

providers: [
  provideHttpClient(
    withXsrfConfiguration({
      cookieName: 'XSRF-TOKEN',
      headerName: 'X-XSRF-TOKEN',
    }),
  ),
]
```

---

## 14. Coverage Enforcement

### Jest Coverage Configuration

Coverage is enforced via Jest configuration with 100% thresholds:

```typescript
// jest.config.ts (relevant section)
const config: Config = {
  collectCoverage: true,
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov', 'html', 'json-summary'],
  collectCoverageFrom: [
    'src/app/**/*.ts',
    '!src/app/**/*.spec.ts',
    '!src/app/**/*.routes.ts',
    '!src/app/**/*.config.ts',
    '!src/app/**/*.model.ts',
    '!src/app/**/index.ts',
    '!src/main.ts',
    '!src/main.server.ts',
  ],
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
};
```

### Coverage Commands

```bash
# Run tests with coverage report
ng test --no-watch --code-coverage

# View HTML coverage report
open coverage/lcov-report/index.html

# Check coverage meets threshold (CI)
# Jest exits non-zero if thresholds are not met — no separate check needed

# Per-file coverage (find uncovered lines)
ng test --no-watch --code-coverage 2>&1 | grep -E "Uncovered"
```

### CI Coverage Gate

```yaml
# .github/workflows/ci.yml (relevant section)
- name: Run tests with coverage
  run: ng test --no-watch --code-coverage

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v4
  with:
    file: ./coverage/lcov.info
    fail_ci_if_error: true
```

Target is 100% (per CLAUDE.md core rules). Jest's `coverageThreshold` is the hard gate — CI fails below this threshold.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with Angular Material sections |
| **labels** | `mat-label` inside `mat-form-field`, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | `mat-error` inline + error summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets via Material, `autocomplete` |
| **cta** | Outcome-focused text ("Save Profile" not "Submit"), loading state with `mat-spinner` |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | No unnecessary change detection, `OnPush` strategy on form components |

### Angular Material Form Pattern

```html
<form [formGroup]="form" (ngSubmit)="onSubmit()" novalidate>
  <!-- Error summary (announced to screen readers) -->
  @if (submitted && form.invalid) {
    <div role="alert" aria-live="assertive" class="error-summary">
      <mat-icon>error</mat-icon>
      <p>Please fix the following errors:</p>
      <ul>
        @if (form.controls.name.hasError('required')) {
          <li><a (click)="focusField('name')">Name is required</a></li>
        }
        @if (form.controls.email.hasError('email')) {
          <li><a (click)="focusField('email')">Enter a valid email</a></li>
        }
      </ul>
    </div>
  }

  <mat-form-field appearance="outline">
    <mat-label>Full Name</mat-label>
    <input matInput formControlName="name" autocomplete="name" #nameInput />
    @if (form.controls.name.hasError('required')) {
      <mat-error>
        <mat-icon>error</mat-icon> Name is required
      </mat-error>
    }
  </mat-form-field>

  <mat-form-field appearance="outline">
    <mat-label>Email</mat-label>
    <input matInput formControlName="email" type="email" autocomplete="email" #emailInput />
    @if (form.controls.email.hasError('required')) {
      <mat-error>
        <mat-icon>error</mat-icon> Email is required
      </mat-error>
    }
    @if (form.controls.email.hasError('email')) {
      <mat-error>
        <mat-icon>error</mat-icon> Enter a valid email address
      </mat-error>
    }
  </mat-form-field>

  <mat-form-field appearance="outline">
    <mat-label>Phone</mat-label>
    <input matInput formControlName="phone" type="tel" autocomplete="tel" />
  </mat-form-field>

  <mat-form-field appearance="outline">
    <mat-label>Notes (optional)</mat-label>
    <textarea matInput formControlName="notes" rows="4"></textarea>
  </mat-form-field>

  <button mat-raised-button color="primary" type="submit"
          [disabled]="submitting()"
          class="min-h-[48px]">
    @if (submitting()) {
      <mat-spinner diameter="20" class="inline-spinner" /> Saving...
    } @else {
      Save Profile
    }
  </button>
</form>
```

**Angular-specific form rules:**
- Always use `mat-form-field` with `appearance="outline"` for consistency
- Always use `mat-label` inside `mat-form-field` — never external labels
- Use `mat-error` for inline error messages — they are only shown when the control is invalid and touched
- Use `novalidate` on the `<form>` tag — HTML5 native validation is unreliable across assistive technologies
- Always provide `autocomplete` attributes on inputs
- Use `mat-spinner` with small `diameter` for button loading states
- Ensure touch targets are at least 48px (Material handles this for most components)

---

## 16. Anti-Patterns (Angular-specific)

### Component Architecture Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Using `@Input()` and `@Output()` decorators | Use `input()`, `input.required()`, and `output()` signal-based APIs |
| Using `*ngIf`, `*ngFor`, `*ngSwitch` structural directives | Use `@if`, `@for`, `@switch` built-in control flow |
| Creating NgModules for new feature areas | Use standalone components with lazy-loaded routes |
| Putting logic in `ngOnInit` that depends on inputs | Use `effect()` or `computed()` — inputs may not be set in `ngOnInit` with signal inputs |
| Using `ngOnChanges` to react to input changes | Use `computed()` to derive state from signal inputs |
| Massive `ngOnInit` with multiple subscriptions | Decompose into `computed()` signals and focused `effect()` calls |
| Manual `subscribe()` calls without cleanup | Use `rxResource()`, `toSignal()`, or `takeUntilDestroyed()` |

### State Management Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Storing state in component class properties | Use `signal()` for local state, Signal Store for shared state |
| Using classic NgRx (actions/reducers/effects) for simple feature state | Use `@ngrx/signals` Signal Store — less boilerplate, same type safety |
| Sharing state via services with `BehaviorSubject` | Use Signal Store — it provides computed state, methods, and hooks |
| Mutating signal values in place (`this.items().push(x)`) | Use `.update()` with immutable operations (`this.items.update(i => [...i, x])`) |
| Using `effect()` to set other signals (signal ping-pong) | Use `computed()` for derived state — effects are for side effects only |
| Putting HTTP calls directly in components | Put HTTP calls in services, triggered from store methods |

### Reactivity Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Calling `subscribe()` in components and manually unsubscribing | Use `toSignal()`, `rxResource()`, or the `async` pipe |
| Using `ChangeDetectorRef.detectChanges()` manually | Use signals — Angular's signal-based change detection handles updates |
| Relying on zone.js for change detection in new code | Use signals and `provideZoneChangeDetection({ eventCoalescing: true })` |
| Using `setTimeout` or `setInterval` without cleanup | Use `afterNextRender()` or RxJS `timer()`/`interval()` with `takeUntilDestroyed()` |
| Using `EventEmitter` with `.subscribe()` outside templates | `EventEmitter` is for template bindings only — use RxJS subjects for service events |

### Template Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Calling methods in templates (`{{ getFullName() }}`) | Use `computed()` signals — method calls trigger on every change detection cycle |
| Complex logic in templates | Move logic to `computed()` signals in the component class |
| Using `@for` without `track` | Always provide `track` — use a unique identifier, never the index |
| Using `[hidden]` to conditionally show content | Use `@if` — `[hidden]` still instantiates the component and runs its lifecycle |
| Inline styles in templates | Use component styles or Tailwind utility classes |
| Using `| async` and `.subscribe()` for the same observable | Pick one — prefer `toSignal()` in Angular 19 |

### Testing Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Using Karma and Jasmine for new projects | Use Jest with `jest-preset-angular` |
| Using Protractor for E2E testing | Use Playwright or Cypress — Protractor is deprecated and unmaintained |
| Testing implementation details (private methods, internal state) | Test behavior through public API and rendered output |
| Using `fixture.detectChanges()` after every signal update | Signal-based change detection is automatic — only call after `TestBed` setup |
| Creating deep TestBed configurations with real services | Use `jest.fn()` mocks for dependencies, real services only for integration tests |
| Skipping `httpMock.verify()` in service tests | Always call `httpMock.verify()` in `afterEach` to catch unmatched HTTP requests |

### Build & Performance Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Eagerly importing all feature components | Lazy-load features via `loadComponent` and `loadChildren` in routes |
| Importing entire Material modules when using one component | Import only the specific component module (`MatButtonModule`, not all of Material) |
| Using `Default` change detection strategy everywhere | Use `OnPush` for presentational components, signals for smart components |
| Not using `@defer` for heavy below-fold content | Use `@defer (on viewport)` for charts, maps, and heavy components |
| Bundle sizes over 250 KB initial load (gzipped) | Analyze with `ng build --stats-json` and optimize lazy loading |
| Deploying without SSR for public-facing pages | Use `@angular/ssr` with appropriate render modes per route |

### Form Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Using untyped `FormGroup` (the `UntypedFormGroup` import) | Use typed `FormGroup<T>` with `FormBuilder.nonNullable` |
| Using template-driven forms for complex forms | Use reactive forms with typed `FormGroup` — template-driven lacks type safety |
| Forms without `novalidate` attribute | Always add `novalidate` — HTML5 native validation is unreliable |
| Forms without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, etc. |
| "Submit" button text | Use outcome-focused CTA: "Save Profile", "Create Account", "Log In" |
| Building custom form controls when Material provides them | Use `mat-form-field`, `mat-select`, `mat-datepicker`, etc. |
| Not using `mat-error` for validation messages | Always use `mat-error` inside `mat-form-field` — it handles show/hide and accessibility |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report an Angular patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:angular&title=[Angular]%20)**

Use the `patterns:angular` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
