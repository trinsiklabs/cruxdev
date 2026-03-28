# Development Patterns — Expo Stack

React Native / Expo / TypeScript / Expo Router / EAS Build / Zustand / Jotai

This document captures stack-specific patterns, conventions, and decisions for React Native + Expo projects. It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building cross-platform mobile apps in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Expo Router screens, test with React Native Testing Library, build with EAS, manage state with Zustand/Jotai, write native modules, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Node.js | 22+ LTS | Required by Expo CLI and Metro bundler |
| Expo SDK | 55+ | New Architecture only (Legacy Architecture removed in SDK 55) |
| React Native | 0.83+ | New Architecture always-on since RN 0.82 |
| React | 19+ | `use()`, Actions, `useOptimistic`, `useFormStatus`, `useActionState` |
| TypeScript | 5.6+ | `satisfies`, `const` type parameters, strict mode required |
| Expo Router | 4+ | File-based routing, typed routes, API routes, deep linking |
| Expo CLI | latest | `npx expo` — bundling, dev server, prebuild |
| EAS CLI | latest | `eas build`, `eas submit`, `eas update` |
| Hermes | v1 | Default JS engine, ES6+ support, improved perf in SDK 55+ |
| Metro | latest | Bundler — fast resolution enabled by default since RN 0.76 |
| Zustand | 5+ | Lightweight client state (UI state, app preferences) |
| Jotai | 2+ | Atomic state for complex interdependent state graphs |
| TanStack Query | 5+ | Server state — caching, revalidation, optimistic updates |
| React Hook Form | 7.54+ | Form state management |
| Zod | 3.24+ | Runtime schema validation for forms and API responses |
| React Native Testing Library | 12+ | Component testing — user-centric queries |
| Jest | 30+ | Test runner — improved RN support, faster execution |
| Detox | 20+ | Gray-box E2E testing for iOS and Android |
| Maestro | 1.39+ | Alternative E2E — declarative YAML-based flows |
| React Native Reanimated | 3+ | UI-thread animations, 60+ FPS |
| React Native Gesture Handler | 2+ | Native gesture system |
| FlashList | 2+ | High-performance list rendering (replaces FlatList) |
| Nativewind | 4+ | Tailwind CSS for React Native |
| expo-image | latest | Performant image component (replaces RN Image) |
| expo-video | latest | Video playback (replaces expo-av) |
| pnpm | 9+ | Package manager — strict, fast, disk-efficient |

### Version Constraint Policy

Use exact versions in `package.json` for production dependencies, range for dev:

```jsonc
{
  "dependencies": {
    // Good — exact for production stability
    "expo": "~55.0.0",
    "react-native": "0.83.0",
    "zustand": "5.0.2",
    "expo-router": "~4.0.0"
  },
  "devDependencies": {
    // Range OK for dev tools
    "@testing-library/react-native": "^12.0.0",
    "jest": "^30.0.0",
    "detox": "^20.0.0"
  }
}
```

**Expo convention:** Use `~` (tilde) for Expo packages — Expo SDK versions are coordinated and patch updates are tested together. Use exact versions for non-Expo production dependencies. Use `^` (caret) for dev dependencies.

### React Native New Architecture

As of React Native 0.82+, the New Architecture is always on and cannot be disabled. This means:

- **Fabric** is the rendering system (no more Paper)
- **TurboModules** replace the legacy Native Modules bridge
- **Bridgeless mode** is the only mode — no bridge initialization
- **JSI (JavaScript Interface)** provides synchronous native calls
- **Concurrent React** features are fully available

All third-party libraries must support the New Architecture. Check compatibility at [reactnative.directory](https://reactnative.directory) before adding any dependency.

### Hermes v1

Expo SDK 55+ ships with Hermes v1, which provides:

- Better support for modern JavaScript (ES6 classes, const/let, async/await)
- Meaningful performance improvements across scenarios
- Improved debugging experience with Chrome DevTools
- Bytecode precompilation for faster startup

Hermes v1 is opt-in in SDK 55 but will become the default. Enable it early:

```json
{
  "expo": {
    "jsEngine": "hermes"
  }
}
```

---

## 2. Project Structure

### Expo Router File-Based Organization

Every screen in the app is a file in the `app/` directory. The file system IS the route structure:

```
my-app/
├── app/                          # Routes (file-based routing)
│   ├── _layout.tsx               # Root layout (providers, nav container)
│   ├── index.tsx                 # Home screen (/)
│   ├── +not-found.tsx            # 404 fallback
│   ├── +html.tsx                 # Custom HTML wrapper (web only)
│   ├── (tabs)/                   # Tab group layout
│   │   ├── _layout.tsx           # Tab navigator config
│   │   ├── index.tsx             # First tab (home)
│   │   ├── explore.tsx           # Second tab
│   │   └── profile.tsx           # Third tab
│   ├── (auth)/                   # Auth group (unauthenticated screens)
│   │   ├── _layout.tsx           # Stack navigator for auth flow
│   │   ├── sign-in.tsx           # Sign in screen
│   │   ├── sign-up.tsx           # Sign up screen
│   │   └── forgot-password.tsx   # Password recovery
│   ├── (app)/                    # Authenticated app group
│   │   ├── _layout.tsx           # Protected layout (auth guard)
│   │   ├── settings/
│   │   │   ├── _layout.tsx       # Settings stack
│   │   │   ├── index.tsx         # Settings home
│   │   │   ├── profile.tsx       # Edit profile
│   │   │   └── notifications.tsx # Notification prefs
│   │   └── [id]/                 # Dynamic route segment
│   │       ├── index.tsx         # Detail view (/app/:id)
│   │       └── edit.tsx          # Edit view (/app/:id/edit)
│   └── api/                      # API routes (server-side)
│       ├── auth+api.ts           # Auth endpoint
│       └── webhook+api.ts        # Webhook handler
├── components/                   # Shared UI components
│   ├── ui/                       # Base UI primitives
│   │   ├── Button.tsx
│   │   ├── Input.tsx
│   │   ├── Card.tsx
│   │   └── Text.tsx
│   ├── forms/                    # Form components
│   │   ├── FormField.tsx
│   │   └── FormError.tsx
│   └── layout/                   # Layout components
│       ├── SafeArea.tsx
│       ├── KeyboardAvoiding.tsx
│       └── ScreenContainer.tsx
├── hooks/                        # Custom hooks
│   ├── useAuth.ts
│   ├── useColorScheme.ts
│   └── useDebounce.ts
├── stores/                       # State management
│   ├── auth.store.ts             # Zustand auth store
│   ├── preferences.store.ts      # Zustand user preferences
│   └── atoms/                    # Jotai atoms (if using Jotai)
│       ├── filter.atoms.ts
│       └── cart.atoms.ts
├── services/                     # API and external services
│   ├── api.ts                    # Base API client (fetch wrapper)
│   ├── auth.service.ts           # Auth API calls
│   └── user.service.ts           # User API calls
├── lib/                          # Utilities and helpers
│   ├── constants.ts              # App-wide constants
│   ├── storage.ts                # AsyncStorage/SecureStore helpers
│   ├── validation.ts             # Zod schemas
│   └── cn.ts                     # Class name utility (Nativewind)
├── types/                        # Shared TypeScript types
│   ├── api.d.ts
│   └── navigation.d.ts
├── modules/                      # Expo native modules (Swift/Kotlin)
│   └── my-native-feature/
│       ├── index.ts              # JS entry
│       ├── src/
│       │   └── MyNativeModule.ts # Module definition
│       ├── ios/
│       │   └── MyNativeModule.swift
│       └── android/
│           └── MyNativeModule.kt
├── assets/                       # Static assets
│   ├── images/
│   ├── fonts/
│   └── animations/               # Lottie files, etc.
├── e2e/                          # E2E tests (Detox or Maestro)
│   ├── .detoxrc.js               # Detox config
│   ├── jest.config.js            # Jest config for Detox
│   └── tests/
│       ├── auth.e2e.ts
│       └── onboarding.e2e.ts
├── __tests__/                    # Unit/component tests
│   ├── components/
│   ├── hooks/
│   ├── stores/
│   └── services/
├── app.config.ts                 # Dynamic Expo config (NOT app.json)
├── eas.json                      # EAS Build/Submit/Update profiles
├── metro.config.js               # Metro bundler config
├── babel.config.js               # Babel config (Reanimated plugin, etc.)
├── tsconfig.json                 # TypeScript config
├── jest.config.ts                # Jest config
├── .env                          # Environment variables (gitignored)
├── .env.example                  # Environment variable template
└── package.json
```

**Conventions:**
- Always use `app.config.ts` over `app.json` — dynamic config enables environment-specific logic, conditional plugins, and computed values
- Route groups `(groupName)` organize navigation without affecting URLs
- Prefix API route files with `+api.ts` suffix
- Components directory is flat categories, not nested by feature — prevents import hell
- Stores use the `.store.ts` suffix for Zustand, `.atoms.ts` for Jotai
- Native modules live in `modules/` with the standard Expo Modules scaffold

### Test Mirror Structure

Tests mirror the source structure:

```
__tests__/
├── components/
│   ├── ui/
│   │   ├── Button.test.tsx
│   │   └── Input.test.tsx
│   └── forms/
│       └── FormField.test.tsx
├── hooks/
│   ├── useAuth.test.ts
│   └── useDebounce.test.ts
├── stores/
│   ├── auth.store.test.ts
│   └── preferences.store.test.ts
├── services/
│   ├── api.test.ts
│   └── auth.service.test.ts
└── lib/
    ├── validation.test.ts
    └── storage.test.ts
```

E2E tests live separately in `e2e/` because they require different config, different runners, and different CI pipelines.

---

## 3. Expo Router Patterns

### Root Layout

The root `_layout.tsx` sets up providers, fonts, and the navigation container:

```tsx
// app/_layout.tsx
import { Stack } from "expo-router";
import { QueryClientProvider } from "@tanstack/react-query";
import { useFonts } from "expo-font";
import * as SplashScreen from "expo-splash-screen";
import { useEffect } from "react";
import { queryClient } from "@/lib/query-client";
import { AuthProvider } from "@/hooks/useAuth";

SplashScreen.preventAutoHideAsync();

export default function RootLayout() {
  const [fontsLoaded] = useFonts({
    SpaceMono: require("../assets/fonts/SpaceMono-Regular.ttf"),
  });

  useEffect(() => {
    if (fontsLoaded) {
      SplashScreen.hideAsync();
    }
  }, [fontsLoaded]);

  if (!fontsLoaded) return null;

  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <Stack screenOptions={{ headerShown: false }} />
      </AuthProvider>
    </QueryClientProvider>
  );
}
```

**Conventions:**
- Providers wrap the entire app at the root layout level
- Splash screen stays visible until fonts and critical data are loaded
- Stack is the default navigator — override in group layouts

### Typed Routes

Enable typed routes for compile-time route safety:

```json
{
  "expo": {
    "experiments": {
      "typedRoutes": true
    }
  }
}
```

This generates route types automatically. Use them everywhere:

```tsx
import { Link, router } from "expo-router";

// Compile-time type checking — invalid routes are TS errors
<Link href="/settings/profile">Edit Profile</Link>

// Programmatic navigation — also type-checked
router.push("/settings/profile");
router.push({ pathname: "/[id]", params: { id: "123" } });

// WRONG — TS error: "/setings/profle" is not a valid route
router.push("/setings/profle");
```

### Dynamic Routes

Use bracket syntax for dynamic segments:

```tsx
// app/items/[id].tsx — matches /items/123
import { useLocalSearchParams } from "expo-router";

export default function ItemDetail() {
  const { id } = useLocalSearchParams<{ id: string }>();
  // id is typed as string
  return <ItemView itemId={id} />;
}
```

```tsx
// app/[...missing].tsx — catch-all route
import { useLocalSearchParams } from "expo-router";

export default function CatchAll() {
  const { missing } = useLocalSearchParams();
  // missing is string[]
  return <NotFoundScreen segments={missing} />;
}
```

### Protected Routes (Auth Guard)

Use a layout to protect authenticated routes:

```tsx
// app/(app)/_layout.tsx
import { Redirect, Stack } from "expo-router";
import { useAuth } from "@/hooks/useAuth";

export default function AppLayout() {
  const { user, isLoading } = useAuth();

  if (isLoading) return <LoadingScreen />;
  if (!user) return <Redirect href="/sign-in" />;

  return <Stack />;
}
```

**Convention:** Never check auth inside individual screens. The layout handles it once for all child routes.

### API Routes

Server-side API routes for sensitive operations:

```tsx
// app/api/auth+api.ts
import type { ExpoRequest, ExpoResponse } from "expo-router/server";

export async function POST(request: ExpoRequest): Promise<ExpoResponse> {
  const body = await request.json();

  // Validate with Zod
  const result = authSchema.safeParse(body);
  if (!result.success) {
    return ExpoResponse.json(
      { error: result.error.flatten() },
      { status: 400 }
    );
  }

  // Handle auth — API keys stay server-side
  const token = await authenticateUser(result.data);

  return ExpoResponse.json({ token }, { status: 200 });
}
```

**Convention:** API routes are for operations that require server-side secrets (API keys, database credentials). Never expose secrets to the client bundle.

### Deep Linking

Expo Router provides automatic deep linking. Configure the scheme in `app.config.ts`:

```ts
export default {
  expo: {
    scheme: "myapp",
    // Universal links (iOS) and App Links (Android)
    plugins: [
      [
        "expo-router",
        {
          origin: "https://myapp.com",
        },
      ],
    ],
  },
};
```

Every file route automatically gets a deep link. `/settings/profile` is accessible via `myapp://settings/profile` and `https://myapp.com/settings/profile`.

---

## 4. App Configuration

### app.config.ts Over app.json

Always use `app.config.ts` for dynamic configuration:

```ts
// app.config.ts
import type { ExpoConfig } from "expo/config";

const IS_DEV = process.env.APP_VARIANT === "development";
const IS_PREVIEW = process.env.APP_VARIANT === "preview";

const getUniqueIdentifier = () => {
  if (IS_DEV) return "com.company.myapp.dev";
  if (IS_PREVIEW) return "com.company.myapp.preview";
  return "com.company.myapp";
};

const getAppName = () => {
  if (IS_DEV) return "MyApp (Dev)";
  if (IS_PREVIEW) return "MyApp (Preview)";
  return "MyApp";
};

const config: ExpoConfig = {
  name: getAppName(),
  slug: "my-app",
  version: "1.0.0",
  orientation: "portrait",
  icon: "./assets/images/icon.png",
  scheme: "myapp",
  userInterfaceStyle: "automatic",
  newArchEnabled: true,
  jsEngine: "hermes",

  ios: {
    supportsTablet: true,
    bundleIdentifier: getUniqueIdentifier(),
    infoPlist: {
      NSCameraUsageDescription: "Camera access for profile photos",
      NSPhotoLibraryUsageDescription: "Photo library for profile photos",
    },
  },

  android: {
    adaptiveIcon: {
      foregroundImage: "./assets/images/adaptive-icon.png",
      backgroundColor: "#ffffff",
    },
    package: getUniqueIdentifier(),
    permissions: ["CAMERA", "READ_EXTERNAL_STORAGE"],
  },

  web: {
    bundler: "metro",
    output: "server",
    favicon: "./assets/images/favicon.png",
  },

  plugins: [
    "expo-router",
    "expo-font",
    "expo-secure-store",
    [
      "expo-camera",
      {
        cameraPermission: "Allow camera access for profile photos",
      },
    ],
    [
      "expo-notifications",
      {
        icon: "./assets/images/notification-icon.png",
        color: "#ffffff",
      },
    ],
  ],

  experiments: {
    typedRoutes: true,
  },

  extra: {
    eas: {
      projectId: process.env.EAS_PROJECT_ID,
    },
  },

  updates: {
    url: `https://u.expo.dev/${process.env.EAS_PROJECT_ID}`,
  },

  runtimeVersion: {
    policy: "appVersion",
  },
};

export default config;
```

**Conventions:**
- Never hardcode bundle identifiers — derive from `APP_VARIANT`
- Always set `newArchEnabled: true` (required for SDK 55+)
- Always set `jsEngine: "hermes"`
- Always enable `typedRoutes` experiment
- Use `runtimeVersion` policy for EAS Update compatibility
- Declare all native permissions explicitly — never rely on auto-linking defaults
- Use `process.env` for secrets — never commit API keys

### Config Plugins

Config plugins modify native project files at prebuild time. Use them for native configuration that `app.config.ts` properties do not cover:

```ts
// plugins/withCustomScheme.ts
import { ConfigPlugin, withInfoPlist } from "expo/config-plugins";

const withCustomScheme: ConfigPlugin<{ scheme: string }> = (config, { scheme }) => {
  return withInfoPlist(config, (config) => {
    config.modResults.CFBundleURLTypes = [
      ...(config.modResults.CFBundleURLTypes || []),
      {
        CFBundleURLSchemes: [scheme],
      },
    ];
    return config;
  });
};

export default withCustomScheme;
```

**Plugin rules:**
- Avoid regex for native file modifications — use static modification (gradle.properties, JSON)
- Never make network requests in plugins
- Never add interactive prompts
- Test plugins with `npx expo prebuild --clean` and inspect the generated native projects
- Chain plugins in the `plugins` array in `app.config.ts` — order matters

---

## 5. EAS Build, Submit & Update

### eas.json Configuration

```json
{
  "cli": {
    "version": ">= 14.0.0",
    "appVersionSource": "remote"
  },
  "build": {
    "development": {
      "developmentClient": true,
      "distribution": "internal",
      "channel": "development",
      "ios": {
        "simulator": true
      },
      "env": {
        "APP_VARIANT": "development"
      }
    },
    "preview": {
      "distribution": "internal",
      "channel": "preview",
      "env": {
        "APP_VARIANT": "preview"
      }
    },
    "production": {
      "channel": "production",
      "autoIncrement": true,
      "env": {
        "APP_VARIANT": "production"
      }
    }
  },
  "submit": {
    "production": {
      "ios": {
        "appleId": "developer@company.com",
        "ascAppId": "1234567890",
        "appleTeamId": "ABCDE12345"
      },
      "android": {
        "serviceAccountKeyPath": "./google-service-account.json",
        "track": "internal"
      }
    }
  }
}
```

### Build Profiles

| Profile | Use Case | Distribution | Channel |
|---|---|---|---|
| `development` | Local dev with dev client | internal | development |
| `preview` | QA/stakeholder testing | internal | preview |
| `production` | App Store / Play Store release | store | production |

### Build Commands

```bash
# Development build (iOS simulator)
eas build --profile development --platform ios

# Development build (Android emulator)
eas build --profile development --platform android

# Preview build (internal distribution — shareable link)
eas build --profile preview --platform all

# Production build
eas build --profile production --platform all

# Production build + auto-submit to stores
eas build --profile production --platform all --auto-submit
```

### EAS Update (Over-the-Air)

Deploy JS-only updates without a new binary:

```bash
# Publish update to preview channel
eas update --channel preview --message "Fix login button alignment"

# Publish update to production channel
eas update --channel production --message "Patch: handle null user gracefully"

# Roll back an update
eas update:rollback --channel production
```

**OTA update rules:**
- OTA updates can only change JavaScript and assets — not native code
- If you add a new native module, config plugin, or change native permissions, you MUST do a full binary build
- Always test OTA updates on the preview channel before production
- Use `runtimeVersion` policy to ensure updates only apply to compatible binaries
- Keep update messages descriptive — they appear in the EAS dashboard

### Versioning Strategy

```ts
// app.config.ts
{
  version: "1.2.0",           // User-facing version (SemVer)
  ios: {
    buildNumber: "1",          // Incremented per binary build
  },
  android: {
    versionCode: 1,            // Incremented per binary build
  },
  runtimeVersion: {
    policy: "appVersion",      // Ties OTA updates to version
  },
}
```

Use `"appVersionSource": "remote"` in `eas.json` + `"autoIncrement": true` in the production profile to let EAS manage build numbers automatically.

---

## 6. Testing Patterns

### Test Pyramid (Expo-specific)

```
        /\
       /  \          E2E (Detox / Maestro)
      /    \         Critical user journeys — login, purchase, onboarding
     /------\
    /        \        Component Tests (React Native Testing Library)
   /          \       Screen rendering, user interactions, form submissions
  /------------\
 /              \      Integration Tests (Jest + mocked services)
/                \     API client behavior, store logic, hook composition
/------------------\
/                    \   Unit Tests (Jest)
/                      \  Pure functions, validators, formatters, calculations
/------------------------\
```

**Target ratio:** 70% unit, 20% component, 10% E2E.

### Jest Configuration

```ts
// jest.config.ts
import type { Config } from "jest";

const config: Config = {
  preset: "jest-expo",
  setupFilesAfterSetup: ["<rootDir>/jest.setup.ts"],
  transformIgnorePatterns: [
    "node_modules/(?!((jest-)?react-native|@react-native(-community)?)|expo(nent)?|@expo(nent)?/.*|@expo-google-fonts/.*|react-navigation|@react-navigation/.*|@sentry/react-native|native-base|react-native-svg|@shopify/flash-list)",
  ],
  moduleNameMapper: {
    "^@/(.*)$": "<rootDir>/$1",
  },
  collectCoverageFrom: [
    "components/**/*.{ts,tsx}",
    "hooks/**/*.{ts,tsx}",
    "stores/**/*.{ts,tsx}",
    "services/**/*.{ts,tsx}",
    "lib/**/*.{ts,tsx}",
    "!**/*.d.ts",
    "!**/index.ts",
  ],
  coverageThreshold: {
    global: {
      branches: 90,
      functions: 90,
      lines: 90,
      statements: 90,
    },
  },
};

export default config;
```

### Jest Setup

```ts
// jest.setup.ts
import "@testing-library/react-native/extend-expect";

// Mock expo-router
jest.mock("expo-router", () => ({
  useRouter: () => ({
    push: jest.fn(),
    replace: jest.fn(),
    back: jest.fn(),
  }),
  useLocalSearchParams: () => ({}),
  useSegments: () => [],
  Link: ({ children }: { children: React.ReactNode }) => children,
  Redirect: () => null,
}));

// Mock expo-secure-store
jest.mock("expo-secure-store", () => ({
  getItemAsync: jest.fn(),
  setItemAsync: jest.fn(),
  deleteItemAsync: jest.fn(),
}));

// Mock expo-font
jest.mock("expo-font", () => ({
  useFonts: () => [true],
  isLoaded: () => true,
}));

// Silence LogBox in tests
jest.mock("react-native/Libraries/LogBox/LogBox", () => ({
  __esModule: true,
  default: {
    ignoreLogs: jest.fn(),
    ignoreAllLogs: jest.fn(),
  },
}));
```

### Component Testing with React Native Testing Library

Test components the way users interact with them — by text, role, and accessibility labels:

```tsx
// __tests__/components/ui/Button.test.tsx
import { render, screen, fireEvent } from "@testing-library/react-native";
import { Button } from "@/components/ui/Button";

describe("Button", () => {
  it("renders label text", () => {
    render(<Button label="Save" onPress={jest.fn()} />);
    expect(screen.getByText("Save")).toBeOnTheScreen();
  });

  it("calls onPress when pressed", () => {
    const onPress = jest.fn();
    render(<Button label="Save" onPress={onPress} />);

    fireEvent.press(screen.getByText("Save"));

    expect(onPress).toHaveBeenCalledTimes(1);
  });

  it("does not call onPress when disabled", () => {
    const onPress = jest.fn();
    render(<Button label="Save" onPress={onPress} disabled />);

    fireEvent.press(screen.getByText("Save"));

    expect(onPress).not.toHaveBeenCalled();
  });

  it("shows loading indicator when loading", () => {
    render(<Button label="Save" onPress={jest.fn()} loading />);

    expect(screen.getByRole("progressbar")).toBeOnTheScreen();
    expect(screen.queryByText("Save")).toBeNull();
  });
});
```

### Screen Testing

```tsx
// __tests__/screens/SignIn.test.tsx
import { render, screen, fireEvent, waitFor } from "@testing-library/react-native";
import SignIn from "@/app/(auth)/sign-in";

// Mock the auth service
jest.mock("@/services/auth.service", () => ({
  signIn: jest.fn(),
}));

import { signIn } from "@/services/auth.service";

describe("SignIn Screen", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it("renders email and password fields", () => {
    render(<SignIn />);

    expect(screen.getByLabelText("Email")).toBeOnTheScreen();
    expect(screen.getByLabelText("Password")).toBeOnTheScreen();
    expect(screen.getByText("Sign In")).toBeOnTheScreen();
  });

  it("shows validation errors on empty submit", async () => {
    render(<SignIn />);

    fireEvent.press(screen.getByText("Sign In"));

    await waitFor(() => {
      expect(screen.getByText("Email is required")).toBeOnTheScreen();
      expect(screen.getByText("Password is required")).toBeOnTheScreen();
    });
  });

  it("calls signIn with credentials on valid submit", async () => {
    (signIn as jest.Mock).mockResolvedValue({ token: "abc123" });
    render(<SignIn />);

    fireEvent.changeText(screen.getByLabelText("Email"), "user@test.com");
    fireEvent.changeText(screen.getByLabelText("Password"), "password123");
    fireEvent.press(screen.getByText("Sign In"));

    await waitFor(() => {
      expect(signIn).toHaveBeenCalledWith({
        email: "user@test.com",
        password: "password123",
      });
    });
  });

  it("displays API error message on failure", async () => {
    (signIn as jest.Mock).mockRejectedValue(new Error("Invalid credentials"));
    render(<SignIn />);

    fireEvent.changeText(screen.getByLabelText("Email"), "user@test.com");
    fireEvent.changeText(screen.getByLabelText("Password"), "wrong");
    fireEvent.press(screen.getByText("Sign In"));

    await waitFor(() => {
      expect(screen.getByText("Invalid credentials")).toBeOnTheScreen();
    });
  });
});
```

### Hook Testing

```tsx
// __tests__/hooks/useDebounce.test.ts
import { renderHook, act } from "@testing-library/react-native";
import { useDebounce } from "@/hooks/useDebounce";

describe("useDebounce", () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("returns initial value immediately", () => {
    const { result } = renderHook(() => useDebounce("hello", 500));
    expect(result.current).toBe("hello");
  });

  it("debounces value changes", () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebounce(value, delay),
      { initialProps: { value: "hello", delay: 500 } }
    );

    rerender({ value: "world", delay: 500 });
    expect(result.current).toBe("hello");

    act(() => {
      jest.advanceTimersByTime(500);
    });

    expect(result.current).toBe("world");
  });
});
```

### Store Testing (Zustand)

```tsx
// __tests__/stores/auth.store.test.ts
import { useAuthStore } from "@/stores/auth.store";

describe("AuthStore", () => {
  beforeEach(() => {
    // Reset store between tests
    useAuthStore.setState({
      user: null,
      token: null,
      isAuthenticated: false,
    });
  });

  it("sets user on login", () => {
    const user = { id: "1", email: "test@test.com" };
    useAuthStore.getState().login(user, "token123");

    expect(useAuthStore.getState().user).toEqual(user);
    expect(useAuthStore.getState().token).toBe("token123");
    expect(useAuthStore.getState().isAuthenticated).toBe(true);
  });

  it("clears state on logout", () => {
    useAuthStore.getState().login({ id: "1", email: "test@test.com" }, "token123");
    useAuthStore.getState().logout();

    expect(useAuthStore.getState().user).toBeNull();
    expect(useAuthStore.getState().token).toBeNull();
    expect(useAuthStore.getState().isAuthenticated).toBe(false);
  });
});
```

### Test Configuration

```ts
// package.json scripts
{
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "test:ci": "jest --ci --coverage --forceExit",
    "e2e:build:ios": "detox build --configuration ios.sim.debug",
    "e2e:test:ios": "detox test --configuration ios.sim.debug",
    "e2e:build:android": "detox build --configuration android.emu.debug",
    "e2e:test:android": "detox test --configuration android.emu.debug"
  }
}
```

---

## 7. E2E Testing

### Detox Setup

Detox is the gray-box E2E framework for React Native. It provides synchronization with the app, so tests wait for animations, network requests, and timers automatically.

```js
// .detoxrc.js
module.exports = {
  testRunner: {
    args: {
      config: "e2e/jest.config.js",
      _: ["e2e/tests"],
    },
    jest: {
      setupTimeout: 120000,
    },
  },
  apps: {
    "ios.debug": {
      type: "ios.app",
      binaryPath: "ios/build/Build/Products/Debug-iphonesimulator/MyApp.app",
      build: "xcodebuild -workspace ios/MyApp.xcworkspace -scheme MyApp -configuration Debug -sdk iphonesimulator -derivedDataPath ios/build",
    },
    "android.debug": {
      type: "android.apk",
      binaryPath: "android/app/build/outputs/apk/debug/app-debug.apk",
      build: "cd android && ./gradlew assembleDebug assembleAndroidTest -DtestBuildType=debug",
    },
  },
  devices: {
    simulator: {
      type: "ios.simulator",
      device: { type: "iPhone 16" },
    },
    emulator: {
      type: "android.emulator",
      device: { avdName: "Pixel_7_API_35" },
    },
  },
  configurations: {
    "ios.sim.debug": {
      device: "simulator",
      app: "ios.debug",
    },
    "android.emu.debug": {
      device: "emulator",
      app: "android.debug",
    },
  },
};
```

### Detox Test Example

```tsx
// e2e/tests/auth.e2e.ts
import { device, element, by, expect } from "detox";

describe("Authentication Flow", () => {
  beforeAll(async () => {
    await device.launchApp({ newInstance: true });
  });

  beforeEach(async () => {
    await device.reloadReactNative();
  });

  it("should sign in with valid credentials", async () => {
    await element(by.label("Email")).typeText("user@test.com");
    await element(by.label("Password")).typeText("password123");
    await element(by.text("Sign In")).tap();

    await expect(element(by.text("Welcome back"))).toBeVisible();
  });

  it("should show error for invalid credentials", async () => {
    await element(by.label("Email")).typeText("user@test.com");
    await element(by.label("Password")).typeText("wrong");
    await element(by.text("Sign In")).tap();

    await expect(element(by.text("Invalid credentials"))).toBeVisible();
  });

  it("should navigate to forgot password", async () => {
    await element(by.text("Forgot Password?")).tap();

    await expect(element(by.text("Reset Password"))).toBeVisible();
  });
});
```

### Maestro Alternative

For teams preferring declarative E2E tests, Maestro uses YAML:

```yaml
# e2e/maestro/auth-flow.yaml
appId: com.company.myapp
---
- launchApp
- tapOn: "Email"
- inputText: "user@test.com"
- tapOn: "Password"
- inputText: "password123"
- tapOn: "Sign In"
- assertVisible: "Welcome back"
```

**When to use which:**
- **Detox** — full programmatic control, CI integration, complex assertions, mock servers
- **Maestro** — rapid test authoring, visual test recording, simple flows, team members who prefer YAML over code

### testID Convention

Every interactive element must have a `testID` for E2E reliability:

```tsx
<TextInput
  testID="email-input"
  accessibilityLabel="Email"
  // ...
/>
<Pressable testID="sign-in-button" accessibilityRole="button">
  <Text>Sign In</Text>
</Pressable>
```

**Convention:** `testID` uses kebab-case, matches the component's purpose: `sign-in-button`, `email-input`, `user-avatar`, `settings-list`.

---

## 8. State Management

### Philosophy: Separate Server State from Client State

This is the single most important state management decision. Server state (API data) and client state (UI state, preferences) have fundamentally different lifecycles, caching needs, and update patterns.

| State Type | Tool | Examples |
|---|---|---|
| **Server state** | TanStack Query | User profile, feed items, search results, notifications |
| **Client state (global)** | Zustand | Auth token, theme preference, onboarding completed |
| **Client state (atomic)** | Jotai | Filter selections, cart items, complex form wizards |
| **Client state (local)** | `useState` / `useReducer` | Modal visibility, input text, accordion open/closed |
| **Form state** | React Hook Form + Zod | Form values, validation errors, dirty tracking |

### Zustand Store Pattern

```ts
// stores/auth.store.ts
import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";
import AsyncStorage from "@react-native-async-storage/async-storage";

interface User {
  id: string;
  email: string;
  name: string;
}

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (user: User, token: string) => void;
  logout: () => void;
  updateUser: (updates: Partial<User>) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      isAuthenticated: false,

      login: (user, token) =>
        set({ user, token, isAuthenticated: true }),

      logout: () =>
        set({ user: null, token: null, isAuthenticated: false }),

      updateUser: (updates) =>
        set((state) => ({
          user: state.user ? { ...state.user, ...updates } : null,
        })),
    }),
    {
      name: "auth-storage",
      storage: createJSONStorage(() => AsyncStorage),
      partialize: (state) => ({
        user: state.user,
        token: state.token,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);
```

**Zustand conventions:**
- One store per domain (auth, preferences, cart) — not one mega-store
- Always use `persist` middleware for state that survives app restarts
- Use `partialize` to exclude functions and transient state from persistence
- Use `createJSONStorage(() => AsyncStorage)` for React Native
- For sensitive data (tokens), use `expo-secure-store` instead of AsyncStorage
- Use selectors to prevent unnecessary re-renders: `useAuthStore((s) => s.user)`

### Jotai Atom Pattern

Use Jotai for complex interdependent state where atoms derive from other atoms:

```ts
// stores/atoms/filter.atoms.ts
import { atom } from "jotai";

// Base atoms
export const searchQueryAtom = atom("");
export const categoryAtom = atom<string | null>(null);
export const sortByAtom = atom<"name" | "date" | "price">("date");
export const pageAtom = atom(1);

// Derived atom — reset page when filters change
export const filtersAtom = atom(
  (get) => ({
    query: get(searchQueryAtom),
    category: get(categoryAtom),
    sortBy: get(sortByAtom),
    page: get(pageAtom),
  }),
  (_get, set, newFilters: Partial<FilterState>) => {
    if (newFilters.query !== undefined) set(searchQueryAtom, newFilters.query);
    if (newFilters.category !== undefined) set(categoryAtom, newFilters.category);
    if (newFilters.sortBy !== undefined) set(sortByAtom, newFilters.sortBy);
    // Reset to page 1 on any filter change
    set(pageAtom, newFilters.page ?? 1);
  }
);

// Read-only derived atom
export const hasActiveFiltersAtom = atom(
  (get) => get(searchQueryAtom) !== "" || get(categoryAtom) !== null
);
```

**Jotai conventions:**
- Base atoms are simple values
- Derived atoms compose base atoms — no duplication of state
- Write atoms handle coordinated updates (like resetting page on filter change)
- Atoms are granular — only components reading a specific atom re-render when it changes

### TanStack Query for Server State

```tsx
// hooks/useUser.ts
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { userService } from "@/services/user.service";

export function useUser(userId: string) {
  return useQuery({
    queryKey: ["user", userId],
    queryFn: () => userService.getUser(userId),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

export function useUpdateUser() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: userService.updateUser,
    onSuccess: (data, variables) => {
      queryClient.setQueryData(["user", variables.id], data);
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
  });
}
```

**Convention:** Every API call goes through TanStack Query. Never call `fetch` directly in components. Never store API response data in Zustand or useState.

---

## 9. Expo Modules (Native Code)

### When to Write Native Modules

Write a native module only when:
1. No existing Expo SDK package or community library covers the need
2. Platform-specific APIs must be accessed (HealthKit, CoreML, ARKit, ML Kit)
3. Performance-critical computation must run on the native thread
4. Wrapping a third-party native SDK (payment processor, analytics)

### Creating a Local Module

```bash
npx create-expo-module@latest --local modules/my-native-feature
```

This scaffolds:

```
modules/my-native-feature/
├── index.ts                      # JS entry — auto-linked by Expo
├── src/
│   └── MyNativeFeatureModule.ts  # Module definition
├── ios/
│   └── MyNativeFeatureModule.swift
├── android/
│   └── MyNativeFeatureModule.kt
└── expo-module.config.json
```

### Swift Module Example

```swift
// modules/my-native-feature/ios/MyNativeFeatureModule.swift
import ExpoModulesCore

public class MyNativeFeatureModule: Module {
  public func definition() -> ModuleDefinition {
    Name("MyNativeFeature")

    // Synchronous function — runs on JS thread
    Function("getDeviceModel") { () -> String in
      return UIDevice.current.model
    }

    // Async function — runs on background thread
    AsyncFunction("processImage") { (uri: String) -> [String: Any] in
      guard let url = URL(string: uri),
            let data = try? Data(contentsOf: url),
            let image = UIImage(data: data) else {
        throw Exception(name: "InvalidImage", description: "Could not load image at \(uri)")
      }

      return [
        "width": image.size.width,
        "height": image.size.height,
        "hasAlpha": image.cgImage?.alphaInfo != .none,
      ]
    }

    // Events — emit to JS
    Events("onProgressUpdate")

    AsyncFunction("startLongTask") { () in
      for i in 0..<100 {
        self.sendEvent("onProgressUpdate", ["progress": Double(i) / 100.0])
        try await Task.sleep(nanoseconds: 100_000_000)
      }
    }
  }
}
```

### Kotlin Module Example

```kotlin
// modules/my-native-feature/android/MyNativeFeatureModule.kt
package expo.modules.mynativefeature

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import android.os.Build

class MyNativeFeatureModule : Module() {
  override fun definition() = ModuleDefinition {
    Name("MyNativeFeature")

    Function("getDeviceModel") {
      return@Function Build.MODEL
    }

    AsyncFunction("processImage") { uri: String ->
      // Process image on background thread
      mapOf(
        "width" to 0,
        "height" to 0,
      )
    }

    Events("onProgressUpdate")
  }
}
```

### TypeScript Module Interface

```ts
// modules/my-native-feature/src/MyNativeFeatureModule.ts
import { NativeModule, requireNativeModule } from "expo";

declare class MyNativeFeatureModule extends NativeModule<{
  onProgressUpdate: (event: { progress: number }) => void;
}> {
  getDeviceModel(): string;
  processImage(uri: string): Promise<{
    width: number;
    height: number;
    hasAlpha: boolean;
  }>;
  startLongTask(): Promise<void>;
}

export default requireNativeModule<MyNativeFeatureModule>("MyNativeFeature");
```

```ts
// modules/my-native-feature/index.ts
export { default } from "./src/MyNativeFeatureModule";
```

**Conventions:**
- Module API is consistent across Swift and Kotlin — same function signatures
- Use `AsyncFunction` for I/O, network, or heavy computation
- Use `Function` only for fast synchronous reads (device info, cached values)
- Events for long-running operations — never poll from JS
- TypeScript interface provides full type safety for consumers
- All modules automatically support New Architecture (TurboModules/JSI)

---

## 10. Performance Optimization

### List Rendering

Always use FlashList over FlatList for lists with more than ~20 items:

```tsx
import { FlashList } from "@shopify/flash-list";

function ItemList({ items }: { items: Item[] }) {
  return (
    <FlashList
      data={items}
      renderItem={({ item }) => <ItemCard item={item} />}
      estimatedItemSize={80}
      keyExtractor={(item) => item.id}
    />
  );
}
```

**FlashList vs FlatList:**

| Feature | FlatList | FlashList |
|---|---|---|
| Blank cells during scroll | Common | Rare (recycling) |
| Memory usage | Creates new views | Recycles views |
| estimatedItemSize | Not needed | Required (improves perf) |
| New Architecture | Partial support | Full support (v2) |
| API compatibility | N/A | Drop-in replacement |

**FlatList optimization rules (when you must use FlatList):**
- Set `windowSize` to 10-15 (default 21 is too large)
- Set `maxToRenderPerBatch` to 5-10
- Set `initialNumToRender` to match visible items
- Use `getItemLayout` for fixed-height items — skips measurement
- Memoize `renderItem` with `useCallback`
- Memoize list items with `React.memo`

### Image Performance

Use `expo-image` instead of React Native's `Image`:

```tsx
import { Image } from "expo-image";

function Avatar({ uri }: { uri: string }) {
  return (
    <Image
      source={{ uri }}
      style={{ width: 48, height: 48, borderRadius: 24 }}
      placeholder={blurhash}
      contentFit="cover"
      transition={200}
      cachePolicy="memory-disk"
    />
  );
}
```

**Why expo-image:**
- Blurhash/thumbhash placeholder support
- Disk + memory caching built-in
- Animated image support (GIF, APNG, WebP)
- Shared element transitions
- Better memory management — prevents OOM on long lists

### Animation Performance

Use Reanimated for animations — they run on the UI thread, not the JS thread:

```tsx
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withSpring,
} from "react-native-reanimated";

function AnimatedCard({ isExpanded }: { isExpanded: boolean }) {
  const height = useSharedValue(80);

  React.useEffect(() => {
    height.value = withSpring(isExpanded ? 200 : 80);
  }, [isExpanded]);

  const animatedStyle = useAnimatedStyle(() => ({
    height: height.value,
  }));

  return <Animated.View style={animatedStyle}>{/* content */}</Animated.View>;
}
```

**Animation rules:**
- Always use `useSharedValue` + `useAnimatedStyle` — never `Animated.Value` from RN core
- Use `withSpring` for natural feeling, `withTiming` for precise control
- Complex gestures use `react-native-gesture-handler` + Reanimated together
- Layout animations use `entering`/`exiting` props on `Animated.View`
- Never animate `opacity: 0` to hide elements — use `display: 'none'` or conditional rendering

### Memoization

```tsx
// Memoize expensive components
const ItemCard = React.memo(function ItemCard({ item }: { item: Item }) {
  return (
    <View>
      <Text>{item.title}</Text>
    </View>
  );
});

// Memoize callbacks passed to child components
function ParentScreen() {
  const handlePress = useCallback((id: string) => {
    router.push(`/items/${id}`);
  }, []);

  return <ItemList onItemPress={handlePress} />;
}
```

**Memoization rules:**
- `React.memo` for list items and any component receiving stable props
- `useCallback` for event handlers passed as props
- `useMemo` for expensive calculations — NOT for object/array creation in simple cases
- Do NOT wrap everything in memo — measure first, optimize where it matters
- Profile with React DevTools and Flipper before optimizing

### Bundle Size

```bash
# Analyze bundle size
npx expo export --dump-sourcemap
npx react-native-bundle-visualizer

# Tree-shaking: import only what you need
import { format } from "date-fns/format";      // Good — ~2KB
import { format } from "date-fns";              // Bad — imports entire library

import { MapPin } from "lucide-react-native";   // Good — single icon
import * as Icons from "lucide-react-native";   // Bad — all icons
```

### Startup Performance

- Use `expo-splash-screen` to keep splash visible during initialization
- Defer non-critical initialization with `InteractionManager.runAfterInteractions`
- Lazy-load heavy screens with `React.lazy` + `Suspense`
- Minimize the root layout — do not initialize analytics, crash reporting, etc. synchronously
- Use Hermes bytecode precompilation (automatic with EAS Build)

---

## 11. Form Patterns

### React Hook Form + Zod

All forms use React Hook Form for state management and Zod for validation:

```tsx
// lib/validation.ts
import { z } from "zod";

export const signUpSchema = z.object({
  name: z
    .string()
    .min(2, "Name must be at least 2 characters")
    .max(100, "Name is too long"),
  email: z
    .string()
    .email("Please enter a valid email"),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters")
    .regex(/[A-Z]/, "Must contain an uppercase letter")
    .regex(/[0-9]/, "Must contain a number"),
  confirmPassword: z.string(),
}).refine((data) => data.password === data.confirmPassword, {
  message: "Passwords do not match",
  path: ["confirmPassword"],
});

export type SignUpFormData = z.infer<typeof signUpSchema>;
```

```tsx
// app/(auth)/sign-up.tsx
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { signUpSchema, type SignUpFormData } from "@/lib/validation";
import { FormField } from "@/components/forms/FormField";
import { Button } from "@/components/ui/Button";

export default function SignUpScreen() {
  const {
    control,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<SignUpFormData>({
    resolver: zodResolver(signUpSchema),
    defaultValues: {
      name: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onSubmit = async (data: SignUpFormData) => {
    try {
      await authService.signUp(data);
      router.replace("/sign-in");
    } catch (error) {
      // Handle API error
    }
  };

  return (
    <ScreenContainer>
      <KeyboardAvoiding>
        <Controller
          control={control}
          name="name"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField
              label="Full Name"
              value={value}
              onChangeText={onChange}
              onBlur={onBlur}
              error={errors.name?.message}
              autoComplete="name"
              textContentType="name"
              returnKeyType="next"
            />
          )}
        />

        <Controller
          control={control}
          name="email"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField
              label="Email"
              value={value}
              onChangeText={onChange}
              onBlur={onBlur}
              error={errors.email?.message}
              keyboardType="email-address"
              autoCapitalize="none"
              autoComplete="email"
              textContentType="emailAddress"
              returnKeyType="next"
            />
          )}
        />

        <Controller
          control={control}
          name="password"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField
              label="Password"
              value={value}
              onChangeText={onChange}
              onBlur={onBlur}
              error={errors.password?.message}
              secureTextEntry
              autoComplete="new-password"
              textContentType="newPassword"
              returnKeyType="next"
            />
          )}
        />

        <Controller
          control={control}
          name="confirmPassword"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField
              label="Confirm Password"
              value={value}
              onChangeText={onChange}
              onBlur={onBlur}
              error={errors.confirmPassword?.message}
              secureTextEntry
              autoComplete="new-password"
              textContentType="newPassword"
              returnKeyType="done"
            />
          )}
        />

        <Button
          label="Create Account"
          onPress={handleSubmit(onSubmit)}
          loading={isSubmitting}
          disabled={isSubmitting}
        />
      </KeyboardAvoiding>
    </ScreenContainer>
  );
}
```

### Form Compliance (Mobile-Specific)

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`, plus these mobile-specific requirements:

| Dimension | Mobile Requirement |
|---|---|
| **Keyboard** | Set `keyboardType` (email-address, phone-pad, numeric), `returnKeyType` (next/done), `autoCapitalize` |
| **autoComplete** | Always set `autoComplete` and `textContentType` (iOS) for autofill |
| **Touch targets** | Minimum 48x48dp touch area for all interactive elements |
| **Keyboard avoidance** | Wrap forms in `KeyboardAvoidingView` — fields must be visible when keyboard is open |
| **Secure input** | Use `secureTextEntry` for passwords, never `textContentType="password"` without it |
| **Focus management** | Chain `returnKeyType="next"` with `ref.focus()` to move between fields |
| **Error display** | Inline errors below each field, accessible via `accessibilityLiveRegion="polite"` |
| **Loading state** | Disable submit button and show activity indicator during submission |
| **Offline** | Queue submissions if offline, retry when connectivity resumes |

### KeyboardAvoidingView Wrapper

```tsx
// components/layout/KeyboardAvoiding.tsx
import { KeyboardAvoidingView, Platform, ScrollView } from "react-native";

interface Props {
  children: React.ReactNode;
}

export function KeyboardAvoiding({ children }: Props) {
  return (
    <KeyboardAvoidingView
      behavior={Platform.OS === "ios" ? "padding" : "height"}
      style={{ flex: 1 }}
      keyboardVerticalOffset={Platform.OS === "ios" ? 88 : 0}
    >
      <ScrollView
        contentContainerStyle={{ flexGrow: 1 }}
        keyboardShouldPersistTaps="handled"
        showsVerticalScrollIndicator={false}
      >
        {children}
      </ScrollView>
    </KeyboardAvoidingView>
  );
}
```

**Convention:** `keyboardShouldPersistTaps="handled"` is mandatory — without it, tapping the submit button while the keyboard is open dismisses the keyboard instead of submitting.

---

## 12. Navigation Patterns

### Tab Navigator

```tsx
// app/(tabs)/_layout.tsx
import { Tabs } from "expo-router";
import { Home, Search, User } from "lucide-react-native";

export default function TabLayout() {
  return (
    <Tabs
      screenOptions={{
        tabBarActiveTintColor: "#007AFF",
        tabBarInactiveTintColor: "#8E8E93",
        headerShown: false,
      }}
    >
      <Tabs.Screen
        name="index"
        options={{
          title: "Home",
          tabBarIcon: ({ color, size }) => <Home color={color} size={size} />,
        }}
      />
      <Tabs.Screen
        name="explore"
        options={{
          title: "Explore",
          tabBarIcon: ({ color, size }) => <Search color={color} size={size} />,
        }}
      />
      <Tabs.Screen
        name="profile"
        options={{
          title: "Profile",
          tabBarIcon: ({ color, size }) => <User color={color} size={size} />,
        }}
      />
    </Tabs>
  );
}
```

### Stack with Modal

```tsx
// app/(app)/_layout.tsx
import { Stack } from "expo-router";

export default function AppLayout() {
  return (
    <Stack>
      <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
      <Stack.Screen
        name="modal"
        options={{
          presentation: "modal",
          headerTitle: "Settings",
        }}
      />
    </Stack>
  );
}
```

### Navigation Conventions

- **Tabs** for top-level sections (3-5 tabs max)
- **Stack** for hierarchical drill-down within a section
- **Modal** for focused tasks that interrupt the main flow (create, edit, filter)
- **Drawer** only if the app has 6+ top-level sections (rare in modern mobile)
- **Deep links** work automatically with Expo Router — every screen has a URL
- **Back behavior** is handled by the navigator — never manually manage back state

### Preventing Flash of Unauthenticated Content

```tsx
// app/_layout.tsx
import { Slot, useRouter, useSegments } from "expo-router";
import { useAuth } from "@/hooks/useAuth";
import { useEffect } from "react";

export default function RootLayout() {
  const { user, isLoading } = useAuth();
  const segments = useSegments();
  const router = useRouter();

  useEffect(() => {
    if (isLoading) return;

    const inAuthGroup = segments[0] === "(auth)";

    if (!user && !inAuthGroup) {
      router.replace("/sign-in");
    } else if (user && inAuthGroup) {
      router.replace("/");
    }
  }, [user, isLoading, segments]);

  if (isLoading) return <SplashScreen />;

  return <Slot />;
}
```

---

## 13. Security

### Sensitive Data Storage

```tsx
// lib/storage.ts
import * as SecureStore from "expo-secure-store";
import AsyncStorage from "@react-native-async-storage/async-storage";

// Sensitive data — tokens, credentials, PII
export const secureStorage = {
  async get(key: string): Promise<string | null> {
    return SecureStore.getItemAsync(key);
  },
  async set(key: string, value: string): Promise<void> {
    return SecureStore.setItemAsync(key, value);
  },
  async remove(key: string): Promise<void> {
    return SecureStore.deleteItemAsync(key);
  },
};

// Non-sensitive data — preferences, cache, UI state
export const storage = {
  async get(key: string): Promise<string | null> {
    return AsyncStorage.getItem(key);
  },
  async set(key: string, value: string): Promise<void> {
    return AsyncStorage.setItem(key, value);
  },
  async remove(key: string): Promise<void> {
    return AsyncStorage.removeItem(key);
  },
};
```

**Storage rules:**
- Tokens, API keys, credentials, PII -> `expo-secure-store` (Keychain/Keystore)
- Theme preference, onboarding state, cached data -> `AsyncStorage`
- Never store secrets in `app.config.ts`, `.env` files shipped in the bundle, or JavaScript constants
- Environment variables in Expo are embedded in the JS bundle and ARE visible to users — use API routes or server-side endpoints for secrets

### API Security

```tsx
// services/api.ts
import { useAuthStore } from "@/stores/auth.store";

const BASE_URL = process.env.EXPO_PUBLIC_API_URL;

async function fetchWithAuth(path: string, options: RequestInit = {}) {
  const token = useAuthStore.getState().token;

  const response = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...options.headers,
    },
  });

  if (response.status === 401) {
    useAuthStore.getState().logout();
    throw new AuthError("Session expired");
  }

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new ApiError(response.status, error.message || "Request failed");
  }

  return response.json();
}
```

### Security Headers (API Routes)

```ts
// app/api/auth+api.ts
export async function POST(request: ExpoRequest): Promise<ExpoResponse> {
  // Rate limiting
  const ip = request.headers.get("x-forwarded-for");
  if (await isRateLimited(ip)) {
    return ExpoResponse.json({ error: "Too many requests" }, { status: 429 });
  }

  // Input validation with Zod
  const body = await request.json();
  const result = authSchema.safeParse(body);
  if (!result.success) {
    return ExpoResponse.json(
      { error: "Invalid input" }, // Never expose Zod details to clients
      { status: 400 }
    );
  }

  // ... handle auth

  return new ExpoResponse(JSON.stringify({ token }), {
    status: 200,
    headers: {
      "Content-Type": "application/json",
      "X-Content-Type-Options": "nosniff",
      "Cache-Control": "no-store",
      "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
    },
  });
}
```

### Certificate Pinning

For high-security apps, pin SSL certificates:

```ts
// app.config.ts plugin or expo-network config
// Use a library like react-native-ssl-pinning or expo-certificate-transparency
```

**Security checklist:**
- [ ] Tokens stored in SecureStore, not AsyncStorage
- [ ] API keys never in client bundle — use API routes or backend proxy
- [ ] HTTPS enforced for all API calls
- [ ] Input validated with Zod on both client and server
- [ ] Auth token refreshed before expiry, cleared on 401
- [ ] Biometric authentication for sensitive actions (expo-local-authentication)
- [ ] App Transport Security (ATS) enabled on iOS (default in Expo)
- [ ] ProGuard/R8 enabled for Android release builds (default in EAS)
- [ ] No sensitive data in console.log (stripped in production builds)

---

## 14. Development Workflow

### Feature Development Cycle

```
1. Write Zod schemas for data shapes
2. Write failing component tests (React Native Testing Library)
3. Write the component/screen
4. Write failing store tests (Zustand/Jotai)
5. Write store logic
6. Write failing service tests (API client)
7. Write service layer
8. Run: npx jest
9. Run: npx tsc --noEmit
10. Run: npx expo lint
11. Refactor while green
12. Write E2E test for critical path (Detox/Maestro)
```

### Common Commands

```bash
# Development
npx expo start                              # Start dev server (Expo Go or dev client)
npx expo start --clear                      # Start with cleared Metro cache
npx expo start --dev-client                 # Start for development build
npx expo run:ios                            # Run on iOS simulator (requires prebuild)
npx expo run:android                        # Run on Android emulator (requires prebuild)

# Prebuild (generate native projects)
npx expo prebuild                           # Generate ios/ and android/
npx expo prebuild --clean                   # Clean and regenerate

# Testing
npx jest                                    # Run all tests
npx jest --watch                            # Watch mode
npx jest --coverage                         # Coverage report
npx jest __tests__/components/              # Run specific directory

# Type checking
npx tsc --noEmit                            # TypeScript check (no output)

# Linting
npx expo lint                               # Lint with Expo's ESLint config
npx expo lint --fix                         # Auto-fix lint issues

# EAS
eas build --profile development --platform ios
eas build --profile preview --platform all
eas build --profile production --platform all --auto-submit
eas update --channel preview --message "description"

# Dependencies
npx expo install package-name               # Install with Expo version resolution
npx expo doctor                             # Check for version mismatches

# E2E
detox build --configuration ios.sim.debug
detox test --configuration ios.sim.debug
```

### Environment Variables

```bash
# .env (gitignored)
EXPO_PUBLIC_API_URL=https://api.myapp.com
EXPO_PUBLIC_SENTRY_DSN=https://abc@sentry.io/123

# Access in code (EXPO_PUBLIC_ prefix required for client-side)
const apiUrl = process.env.EXPO_PUBLIC_API_URL;
```

**Convention:** Only variables prefixed with `EXPO_PUBLIC_` are available in client code. All others are only available in `app.config.ts`, config plugins, and API routes. This is a security boundary — respect it.

### package.json Scripts

```json
{
  "scripts": {
    "start": "expo start",
    "start:clear": "expo start --clear",
    "ios": "expo run:ios",
    "android": "expo run:android",
    "web": "expo start --web",
    "prebuild": "expo prebuild",
    "prebuild:clean": "expo prebuild --clean",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "test:ci": "jest --ci --coverage --forceExit",
    "typecheck": "tsc --noEmit",
    "lint": "expo lint",
    "lint:fix": "expo lint --fix",
    "quality": "npm run typecheck && npm run lint && npm run test",
    "e2e:build:ios": "detox build --configuration ios.sim.debug",
    "e2e:test:ios": "detox test --configuration ios.sim.debug",
    "e2e:build:android": "detox build --configuration android.emu.debug",
    "e2e:test:android": "detox test --configuration android.emu.debug",
    "doctor": "expo doctor"
  }
}
```

---

## 15. CI/CD Pipeline

### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - run: pnpm install --frozen-lockfile

      - name: TypeScript check
        run: pnpm typecheck

      - name: Lint
        run: pnpm lint

      - name: Unit & Component Tests
        run: pnpm test:ci

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          token: ${{ secrets.CODECOV_TOKEN }}

  eas-build:
    needs: quality
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - run: pnpm install --frozen-lockfile

      - uses: expo/expo-github-action@v8
        with:
          eas-version: latest
          token: ${{ secrets.EXPO_TOKEN }}

      - name: Build preview
        run: eas build --profile preview --platform all --non-interactive

  e2e:
    needs: quality
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - run: pnpm install --frozen-lockfile
      - run: brew tap wix/brew && brew install applesimutils

      - name: Prebuild iOS
        run: npx expo prebuild --platform ios --clean

      - name: Build Detox
        run: pnpm e2e:build:ios

      - name: Run E2E tests
        run: pnpm e2e:test:ios

  deploy:
    needs: [quality, eas-build]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - run: pnpm install --frozen-lockfile

      - uses: expo/expo-github-action@v8
        with:
          eas-version: latest
          token: ${{ secrets.EXPO_TOKEN }}

      - name: Submit to stores
        run: eas submit --profile production --platform all --non-interactive
```

### EAS Build on PR

For faster PR feedback, use EAS Build with fingerprint:

```yaml
# .github/workflows/pr-preview.yml
name: PR Preview

on:
  pull_request:
    branches: [main]

jobs:
  preview:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: expo/expo-github-action@v8
        with:
          eas-version: latest
          token: ${{ secrets.EXPO_TOKEN }}

      - name: Check fingerprint
        id: fingerprint
        run: |
          npx @expo/fingerprint ./
          # Only build if native code changed

      - name: Build preview
        if: steps.fingerprint.outputs.changed == 'true'
        run: eas build --profile preview --platform all --non-interactive

      - name: Publish OTA update
        if: steps.fingerprint.outputs.changed != 'true'
        run: eas update --branch pr-${{ github.event.number }} --message "PR #${{ github.event.number }}"
```

### Release Strategy

| Trigger | Action | Channel |
|---|---|---|
| PR opened/updated | OTA update to PR branch (if no native changes) | pr-{number} |
| PR merged to main | EAS Build preview | preview |
| Manual approval | EAS Build production + submit | production |
| Hotfix | EAS Update to production | production |

---

## 16. Anti-Patterns (Expo-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using `app.json` for configuration | Use `app.config.ts` — enables dynamic config, environment variables, computed values |
| 2 | Storing tokens in AsyncStorage | Use `expo-secure-store` (Keychain/Keystore) for tokens, credentials, and PII |
| 3 | Putting API keys in `EXPO_PUBLIC_` env vars | API keys go server-side — use API routes (`+api.ts`) or a backend proxy |
| 4 | Using React Native `Image` component | Use `expo-image` — better caching, blurhash placeholders, memory management |
| 5 | Using FlatList for large lists | Use `@shopify/flash-list` — view recycling, better scroll performance |
| 6 | Using `expo-av` for video | Use `expo-video` — more reliable, performant, and actively maintained |
| 7 | Storing server state in Zustand/useState | Use TanStack Query for server state — caching, revalidation, optimistic updates are handled |
| 8 | Creating styles inside render/return | Use `StyleSheet.create()` outside the component — prevents object recreation every render |
| 9 | Using `Animated` from React Native core | Use `react-native-reanimated` — UI thread animations, 60+ FPS, worklet support |
| 10 | Not setting `keyboardShouldPersistTaps` on ScrollViews wrapping forms | Set `keyboardShouldPersistTaps="handled"` — otherwise button presses dismiss keyboard instead of firing |
| 11 | Checking auth in every screen component | Use a layout-level auth guard (`_layout.tsx` with `<Redirect>`) — one check protects all child routes |
| 12 | Importing entire libraries (`import * as Icons`) | Import individual exports (`import { MapPin }`) — prevents bundling unused code |
| 13 | Using `console.log` for debugging in production | Use a proper logging library with log levels; `console.log` is stripped in Hermes production builds but pollutes dev output |
| 14 | Managing navigation state manually | Let Expo Router handle it — file-based routing means the file system IS the navigation state |
| 15 | Running native code in `app.config.ts` or config plugins | Config plugins run at prebuild time, not runtime — no network calls, no async, no interactive prompts |
| 16 | Skipping `npx expo doctor` after dependency changes | Always run `expo doctor` — catches version mismatches between Expo SDK and community packages |
| 17 | Not using `estimatedItemSize` with FlashList | Always provide `estimatedItemSize` — FlashList needs it for efficient view recycling |
| 18 | Hardcoding platform checks with `Platform.OS` everywhere | Use platform-specific file extensions (`.ios.tsx`, `.android.tsx`) or Expo's platform-specific config |
| 19 | Using `useEffect` for data fetching | Use TanStack Query's `useQuery` — handles loading, error, caching, deduplication, and revalidation |
| 20 | Not testing with `accessibilityLabel` and roles | Use accessibility queries in tests (`getByLabelText`, `getByRole`) — ensures accessibility AND better test reliability |
| 21 | Ejecting from Expo (`expo eject`) | Use `npx expo prebuild` with config plugins — continuous native generation, never eject |
| 22 | Ignoring the New Architecture | RN 0.82+ is New Architecture only — all dependencies must support Fabric and TurboModules |
| 23 | Using `@react-native-community/async-storage` for sensitive data | Use `expo-secure-store` — AsyncStorage is unencrypted plaintext on both platforms |
| 24 | Not setting `textContentType` and `autoComplete` on form inputs | Always set both — enables password managers, autofill, and one-time code auto-fill |
| 25 | Building separate apps for dev/preview/production without app variants | Use `APP_VARIANT` env variable in `app.config.ts` to derive bundle ID, app name, and icon per environment |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report an Expo patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:expo&title=[Expo]%20)**

Use the `patterns:expo` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
