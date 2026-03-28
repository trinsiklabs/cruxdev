# Development Patterns — Flutter Stack

Flutter / Dart / Material 3 / Riverpod / Bloc / go_router / freezed

This document captures stack-specific patterns, conventions, and decisions for Flutter + Dart projects. It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building cross-platform apps in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Flutter apps, manage state with Riverpod/Bloc, navigate with go_router, generate code with freezed/json_serializable, test with flutter_test and integration_test, write platform channels, profile performance, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Flutter | 3.29+ | Stable channel, Impeller rendering engine default |
| Dart | 3.7+ | Records, patterns, sealed classes, macros (preview) |
| Material 3 | Default | `useMaterial3: true` is default since Flutter 3.16 |
| Riverpod | 2.6+ | Reactive state management with code generation |
| flutter_bloc | 8.1+ | BLoC pattern for event-driven state management |
| go_router | 14+ | Declarative routing with deep linking, ShellRoute |
| freezed | 2.5+ | Immutable data classes, unions, sealed classes |
| json_serializable | 6.9+ | JSON serialization code generation |
| build_runner | 2.4+ | Code generation orchestrator |
| flutter_test | SDK | Widget testing, golden tests |
| integration_test | SDK | On-device integration testing |
| mockito | 5.4+ | Mock generation for unit testing |
| flutter_lints | 5+ | Recommended lint rules |
| very_good_analysis | 7+ | Stricter lint rules (preferred) |
| dio | 5.7+ | HTTP client with interceptors |
| retrofit | 4.4+ | Type-safe REST client code generation |
| flutter_secure_storage | 9+ | Platform-specific secure key-value storage |
| cached_network_image | 3.4+ | Image caching and loading |
| flutter_localizations | SDK | Internationalization (i18n) |
| intl | 0.19+ | Date, number, and message formatting |
| equatable | 2.0+ | Value equality for Dart classes |
| dartz / fpdart | 1.1+ / 2+ | Functional programming (Either, Option) |
| hive / isar | 4+ / 4+ | Local database (choose one per project) |
| flutter_hooks | 0.20+ | React-style hooks for widgets |
| auto_route | 9+ | Alternative to go_router for complex navigation |

### Version Constraint Policy

Use caret syntax (`^`) in `pubspec.yaml` pinned to the minor version:

```yaml
# Good — allows patch updates, blocks breaking changes
dependencies:
  flutter_riverpod: ^2.6.0
  go_router: ^14.0.0
  freezed_annotation: ^2.5.0
  dio: ^5.7.0

dev_dependencies:
  freezed: ^2.5.0
  json_serializable: ^6.9.0
  build_runner: ^2.4.0
  mockito: ^5.4.0
  very_good_analysis: ^7.0.0

# Bad — too loose, allows breaking minor updates
dependencies:
  flutter_riverpod: ^2.0.0

# Bad — too tight, blocks patch fixes
dependencies:
  flutter_riverpod: 2.6.1
```

Exception: for pre-release packages or packages with known instability, pin exact.

### Dart Language Features (3.7+)

Flutter 3.29+ ships with Dart 3.7+, which provides:

- **Records** — lightweight tuples: `(String, int)`, named fields `({String name, int age})`
- **Patterns** — destructuring, switch expressions, `if-case`, guard clauses
- **Sealed classes** — exhaustive pattern matching for ADTs
- **Class modifiers** — `sealed`, `final`, `base`, `interface`, `mixin`
- **Extension types** — zero-cost wrapper types for APIs
- **Macros** (preview) — compile-time metaprogramming (experimental, track progress)

Use these features aggressively — they reduce boilerplate and improve type safety:

```dart
// Records for multiple return values
(User, List<Permission>) loadUserWithPermissions(String id) { ... }

// Sealed classes for domain modeling
sealed class AuthState {}
final class Authenticated extends AuthState {
  final User user;
  Authenticated(this.user);
}
final class Unauthenticated extends AuthState {}
final class AuthLoading extends AuthState {}

// Exhaustive switch expressions
String label(AuthState state) => switch (state) {
  Authenticated(:final user) => 'Welcome, ${user.name}',
  Unauthenticated() => 'Please sign in',
  AuthLoading() => 'Loading...',
};

// if-case for safe extraction
if (response case {'data': {'user': Map userJson}}) {
  final user = User.fromJson(userJson);
}
```

### Impeller Rendering Engine

Flutter 3.29+ uses Impeller by default on iOS and Android. Key implications:

- **No runtime shader compilation** — eliminates first-frame jank
- **Predictable performance** — all shaders are pre-compiled at build time
- **Metal on iOS, Vulkan/OpenGL ES on Android** — hardware-accelerated
- Skia is deprecated and will be removed — do not rely on Skia-specific behaviors
- Custom shader effects use the `FragmentProgram` API, not Skia's `ImageFilter`

```dart
// Verify Impeller is active in debug builds
// flutter run --enable-impeller (default on)
// flutter run --no-enable-impeller (fallback to Skia, debug only)
```

---

## 2. Project Structure

### Feature-First Organization

Every Flutter project uses a feature-first directory structure. Features are self-contained modules with their own models, state management, and UI:

```
lib/
├── app/                        # App-level configuration
│   ├── app.dart                # MaterialApp.router setup
│   ├── router.dart             # go_router configuration
│   └── theme.dart              # Material 3 theme definition
├── core/                       # Shared infrastructure
│   ├── constants/              # App-wide constants
│   │   ├── api_constants.dart
│   │   └── ui_constants.dart
│   ├── errors/                 # Error handling
│   │   ├── failures.dart       # Domain failure types
│   │   └── exceptions.dart     # Data layer exceptions
│   ├── extensions/             # Dart extension methods
│   │   ├── context_extensions.dart
│   │   └── string_extensions.dart
│   ├── network/                # HTTP client, interceptors
│   │   ├── api_client.dart
│   │   ├── auth_interceptor.dart
│   │   └── error_interceptor.dart
│   ├── storage/                # Local persistence
│   │   ├── secure_storage.dart
│   │   └── preferences.dart
│   ├── utils/                  # Pure utility functions
│   │   ├── validators.dart
│   │   └── formatters.dart
│   └── widgets/                # Shared reusable widgets
│       ├── app_bar.dart
│       ├── error_view.dart
│       ├── loading_indicator.dart
│       └── responsive_layout.dart
├── features/                   # Feature modules
│   ├── auth/
│   │   ├── data/
│   │   │   ├── datasources/
│   │   │   │   ├── auth_remote_datasource.dart
│   │   │   │   └── auth_local_datasource.dart
│   │   │   ├── models/
│   │   │   │   ├── user_model.dart
│   │   │   │   └── user_model.g.dart
│   │   │   └── repositories/
│   │   │       └── auth_repository_impl.dart
│   │   ├── domain/
│   │   │   ├── entities/
│   │   │   │   └── user.dart
│   │   │   ├── repositories/
│   │   │   │   └── auth_repository.dart
│   │   │   └── usecases/
│   │   │       ├── login_usecase.dart
│   │   │       ├── logout_usecase.dart
│   │   │       └── register_usecase.dart
│   │   └── presentation/
│   │       ├── providers/       # Riverpod providers (or blocs/)
│   │       │   └── auth_provider.dart
│   │       ├── screens/
│   │       │   ├── login_screen.dart
│   │       │   └── register_screen.dart
│   │       └── widgets/
│   │           ├── login_form.dart
│   │           └── social_login_buttons.dart
│   ├── home/
│   │   ├── data/
│   │   ├── domain/
│   │   └── presentation/
│   ├── profile/
│   │   ├── data/
│   │   ├── domain/
│   │   └── presentation/
│   └── settings/
│       ├── data/
│       ├── domain/
│       └── presentation/
├── l10n/                       # Localization
│   ├── app_en.arb
│   └── app_es.arb
└── main.dart                   # Entry point
```

**Conventions:**
- Every feature has three layers: `data/`, `domain/`, `presentation/`
- `domain/` contains entities (pure Dart classes), repository interfaces, and use cases
- `data/` contains models (with serialization), datasources, and repository implementations
- `presentation/` contains screens, widgets, and state management (providers or blocs)
- Shared code lives in `core/` — never import from one feature into another
- Cross-feature communication goes through domain interfaces or app-level state

### Test Mirror Structure

Tests mirror the `lib/` structure:

```
test/
├── core/
│   ├── network/
│   │   └── api_client_test.dart
│   ├── utils/
│   │   ├── validators_test.dart
│   │   └── formatters_test.dart
│   └── widgets/
│       └── error_view_test.dart
├── features/
│   ├── auth/
│   │   ├── data/
│   │   │   ├── datasources/
│   │   │   │   └── auth_remote_datasource_test.dart
│   │   │   └── repositories/
│   │   │       └── auth_repository_impl_test.dart
│   │   ├── domain/
│   │   │   └── usecases/
│   │   │       ├── login_usecase_test.dart
│   │   │       └── register_usecase_test.dart
│   │   └── presentation/
│   │       ├── providers/
│   │       │   └── auth_provider_test.dart
│   │       ├── screens/
│   │       │   └── login_screen_test.dart
│   │       └── widgets/
│   │           └── login_form_test.dart
│   └── home/
│       └── ...
├── fixtures/                   # JSON fixtures for serialization tests
│   ├── user.json
│   └── error_response.json
├── helpers/                    # Test utilities
│   ├── pump_app.dart           # Wraps widgets in MaterialApp for testing
│   ├── mocks.dart              # @GenerateMocks annotations
│   ├── fakes.dart              # Manual fakes for complex dependencies
│   └── golden_toolkit.dart     # Golden test configuration
└── integration_test/
    ├── app_test.dart           # Full app integration tests
    ├── auth_flow_test.dart
    └── robots/                 # Page Object / Robot pattern
        ├── login_robot.dart
        └── home_robot.dart
```

### Barrel Files

Use barrel files (`index.dart` or feature-named exports) sparingly. Prefer explicit imports for IDE performance and tree-shaking:

```dart
// Acceptable: feature-level barrel for public API
// lib/features/auth/auth.dart
export 'domain/entities/user.dart';
export 'domain/repositories/auth_repository.dart';
export 'domain/usecases/login_usecase.dart';

// Avoid: barrel files in every subdirectory — creates import cycles and slows analysis
```

---

## 3. State Management — Riverpod

### Philosophy

Riverpod is the default state management solution. It provides compile-time safety, testability, and provider scoping that `InheritedWidget`, `Provider`, and `setState` cannot match.

Use Riverpod when:
- The app needs reactive state that multiple widgets consume
- State outlives a single widget tree
- Dependency injection and testability matter

### Provider Types

| Provider | Use Case | Example |
|---|---|---|
| `Provider` | Computed/derived values, DI | Repository instances, formatted strings |
| `StateProvider` | Simple mutable state | Toggle flags, selected index |
| `StateNotifierProvider` | Complex mutable state with logic | Form state, filters |
| `FutureProvider` | Async one-shot data | Fetch user profile, load config |
| `StreamProvider` | Reactive streams | WebSocket data, Firestore streams |
| `NotifierProvider` | Riverpod 2.0+ replacement for StateNotifier | All new complex state |
| `AsyncNotifierProvider` | Async state with mutations | CRUD operations, paginated lists |
| `ChangeNotifierProvider` | Legacy migration only | **Never use for new code** |

### Code Generation (Recommended)

Use `riverpod_generator` for type-safe, boilerplate-free providers:

```dart
// lib/features/auth/presentation/providers/auth_provider.dart
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'auth_provider.g.dart';

@riverpod
class AuthNotifier extends _$AuthNotifier {
  @override
  FutureOr<AuthState> build() async {
    final authRepo = ref.watch(authRepositoryProvider);
    final user = await authRepo.getCurrentUser();
    return user != null ? Authenticated(user) : Unauthenticated();
  }

  Future<void> login(String email, String password) async {
    state = const AsyncLoading();
    state = await AsyncValue.guard(() async {
      final authRepo = ref.read(authRepositoryProvider);
      final user = await authRepo.login(email, password);
      return Authenticated(user);
    });
  }

  Future<void> logout() async {
    final authRepo = ref.read(authRepositoryProvider);
    await authRepo.logout();
    state = const AsyncData(Unauthenticated());
  }
}

@riverpod
AuthRepository authRepository(Ref ref) {
  final client = ref.watch(apiClientProvider);
  final storage = ref.watch(secureStorageProvider);
  return AuthRepositoryImpl(client: client, storage: storage);
}
```

### Provider Scoping

Scope providers to the narrowest possible tree:

```dart
// Global providers — app-level singletons
// Defined at top level, used everywhere
@riverpod
ApiClient apiClient(Ref ref) => ApiClient(baseUrl: ApiConstants.baseUrl);

// Feature providers — scoped to feature
// Only imported within the feature
@riverpod
class ProfileNotifier extends _$ProfileNotifier { ... }

// Screen-local state — use autoDispose (default with code gen)
// Automatically cleaned up when no longer listened to
@riverpod
class SearchNotifier extends _$SearchNotifier { ... }
```

### Riverpod Testing

Override providers in tests — no DI framework needed:

```dart
void main() {
  group('LoginScreen', () {
    late MockAuthRepository mockAuthRepo;

    setUp(() {
      mockAuthRepo = MockAuthRepository();
    });

    testWidgets('shows error on invalid credentials', (tester) async {
      when(() => mockAuthRepo.login(any(), any()))
          .thenThrow(InvalidCredentialsException());

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            authRepositoryProvider.overrideWithValue(mockAuthRepo),
          ],
          child: const MaterialApp(home: LoginScreen()),
        ),
      );

      await tester.enterText(find.byKey(const Key('email')), 'test@test.com');
      await tester.enterText(find.byKey(const Key('password')), 'wrong');
      await tester.tap(find.byKey(const Key('login_button')));
      await tester.pumpAndSettle();

      expect(find.text('Invalid email or password'), findsOneWidget);
    });
  });
}
```

---

## 4. State Management — Bloc

### When to Use Bloc

Use Bloc when:
- The team has existing Bloc expertise
- Event sourcing and event traceability are required
- Complex state transitions need explicit documentation
- The project requires strict separation of events and state

Bloc and Riverpod can coexist in the same project. Use Riverpod for DI and simple reactive state, Bloc for complex domain logic with event tracing.

### Bloc Structure

```dart
// Events — sealed class hierarchy
sealed class AuthEvent {}
final class LoginRequested extends AuthEvent {
  final String email;
  final String password;
  LoginRequested({required this.email, required this.password});
}
final class LogoutRequested extends AuthEvent {}
final class AuthCheckRequested extends AuthEvent {}

// State — sealed class hierarchy
sealed class AuthState {}
final class AuthInitial extends AuthState {}
final class AuthLoading extends AuthState {}
final class AuthAuthenticated extends AuthState {
  final User user;
  AuthAuthenticated(this.user);
}
final class AuthUnauthenticated extends AuthState {}
final class AuthError extends AuthState {
  final String message;
  AuthError(this.message);
}

// Bloc
class AuthBloc extends Bloc<AuthEvent, AuthState> {
  final LoginUseCase _loginUseCase;
  final LogoutUseCase _logoutUseCase;
  final GetCurrentUserUseCase _getCurrentUserUseCase;

  AuthBloc({
    required LoginUseCase loginUseCase,
    required LogoutUseCase logoutUseCase,
    required GetCurrentUserUseCase getCurrentUserUseCase,
  })  : _loginUseCase = loginUseCase,
        _logoutUseCase = logoutUseCase,
        _getCurrentUserUseCase = getCurrentUserUseCase,
        super(AuthInitial()) {
    on<LoginRequested>(_onLoginRequested);
    on<LogoutRequested>(_onLogoutRequested);
    on<AuthCheckRequested>(_onAuthCheckRequested);
  }

  Future<void> _onLoginRequested(
    LoginRequested event,
    Emitter<AuthState> emit,
  ) async {
    emit(AuthLoading());
    final result = await _loginUseCase(
      LoginParams(email: event.email, password: event.password),
    );
    result.fold(
      (failure) => emit(AuthError(failure.message)),
      (user) => emit(AuthAuthenticated(user)),
    );
  }

  Future<void> _onLogoutRequested(
    LogoutRequested event,
    Emitter<AuthState> emit,
  ) async {
    await _logoutUseCase();
    emit(AuthUnauthenticated());
  }

  Future<void> _onAuthCheckRequested(
    AuthCheckRequested event,
    Emitter<AuthState> emit,
  ) async {
    final user = await _getCurrentUserUseCase();
    emit(user != null ? AuthAuthenticated(user) : AuthUnauthenticated());
  }
}
```

### Bloc Testing

Bloc's event-driven architecture makes testing deterministic:

```dart
void main() {
  group('AuthBloc', () {
    late AuthBloc authBloc;
    late MockLoginUseCase mockLogin;
    late MockLogoutUseCase mockLogout;
    late MockGetCurrentUserUseCase mockGetUser;

    setUp(() {
      mockLogin = MockLoginUseCase();
      mockLogout = MockLogoutUseCase();
      mockGetUser = MockGetCurrentUserUseCase();
      authBloc = AuthBloc(
        loginUseCase: mockLogin,
        logoutUseCase: mockLogout,
        getCurrentUserUseCase: mockGetUser,
      );
    });

    tearDown(() => authBloc.close());

    blocTest<AuthBloc, AuthState>(
      'emits [AuthLoading, AuthAuthenticated] on successful login',
      build: () {
        when(() => mockLogin(any())).thenAnswer(
          (_) async => Right(User(id: '1', name: 'Test')),
        );
        return authBloc;
      },
      act: (bloc) => bloc.add(
        LoginRequested(email: 'test@test.com', password: 'pass123'),
      ),
      expect: () => [
        isA<AuthLoading>(),
        isA<AuthAuthenticated>(),
      ],
    );

    blocTest<AuthBloc, AuthState>(
      'emits [AuthLoading, AuthError] on failed login',
      build: () {
        when(() => mockLogin(any())).thenAnswer(
          (_) async => Left(ServerFailure('Server error')),
        );
        return authBloc;
      },
      act: (bloc) => bloc.add(
        LoginRequested(email: 'test@test.com', password: 'wrong'),
      ),
      expect: () => [
        isA<AuthLoading>(),
        isA<AuthError>(),
      ],
    );
  });
}
```

### BlocObserver for Debugging

```dart
class AppBlocObserver extends BlocObserver {
  @override
  void onEvent(Bloc bloc, Object? event) {
    super.onEvent(bloc, event);
    debugPrint('${bloc.runtimeType} | $event');
  }

  @override
  void onTransition(Bloc bloc, Transition transition) {
    super.onTransition(bloc, transition);
    debugPrint('${bloc.runtimeType} | $transition');
  }

  @override
  void onError(BlocBase bloc, Object error, StackTrace stackTrace) {
    super.onError(bloc, error, stackTrace);
    debugPrint('${bloc.runtimeType} | $error\n$stackTrace');
  }
}

// main.dart
void main() {
  Bloc.observer = AppBlocObserver();
  runApp(const MyApp());
}
```

---

## 5. Navigation — go_router

### Configuration

Centralize all routing in `app/router.dart`:

```dart
import 'package:go_router/go_router.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'router.g.dart';

@riverpod
GoRouter router(Ref ref) {
  final authState = ref.watch(authNotifierProvider);

  return GoRouter(
    initialLocation: '/',
    debugLogDiagnostics: kDebugMode,
    redirect: (context, state) {
      final isAuthenticated = authState.valueOrNull is Authenticated;
      final isAuthRoute = state.matchedLocation.startsWith('/auth');

      if (!isAuthenticated && !isAuthRoute) return '/auth/login';
      if (isAuthenticated && isAuthRoute) return '/';
      return null;
    },
    routes: [
      // Auth routes — no shell
      GoRoute(
        path: '/auth/login',
        name: 'login',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/auth/register',
        name: 'register',
        builder: (context, state) => const RegisterScreen(),
      ),

      // Main app — ShellRoute with bottom navigation
      StatefulShellRoute.indexedStack(
        builder: (context, state, navigationShell) =>
            MainShell(navigationShell: navigationShell),
        branches: [
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/',
                name: 'home',
                builder: (context, state) => const HomeScreen(),
                routes: [
                  GoRoute(
                    path: 'details/:id',
                    name: 'details',
                    builder: (context, state) => DetailsScreen(
                      id: state.pathParameters['id']!,
                    ),
                  ),
                ],
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/profile',
                name: 'profile',
                builder: (context, state) => const ProfileScreen(),
                routes: [
                  GoRoute(
                    path: 'edit',
                    name: 'editProfile',
                    builder: (context, state) => const EditProfileScreen(),
                  ),
                ],
              ),
            ],
          ),
          StatefulShellBranch(
            routes: [
              GoRoute(
                path: '/settings',
                name: 'settings',
                builder: (context, state) => const SettingsScreen(),
              ),
            ],
          ),
        ],
      ),
    ],
    errorBuilder: (context, state) => ErrorScreen(error: state.error),
  );
}
```

### Navigation Patterns

```dart
// Named navigation (preferred — refactor-safe)
context.goNamed('details', pathParameters: {'id': item.id});

// Path navigation (simpler for top-level routes)
context.go('/settings');

// Push (adds to stack, shows back button)
context.pushNamed('editProfile');

// Pop (go back)
context.pop();

// Replace (no back)
context.pushReplacementNamed('home');
```

### Deep Linking

go_router handles deep linking automatically on iOS (Universal Links) and Android (App Links). Configure:

```xml
<!-- android/app/src/main/AndroidManifest.xml -->
<intent-filter android:autoVerify="true">
  <action android:name="android.intent.action.VIEW" />
  <category android:name="android.intent.category.DEFAULT" />
  <category android:name="android.intent.category.BROWSABLE" />
  <data android:scheme="https" android:host="app.example.com" />
</intent-filter>
```

```plist
<!-- ios/Runner/Info.plist — associated domains -->
<key>com.apple.developer.associated-domains</key>
<array>
  <string>applinks:app.example.com</string>
</array>
```

### Type-Safe Routes (Code Generation)

For large apps, use `go_router_builder` for compile-time route safety:

```dart
@TypedGoRoute<HomeRoute>(
  path: '/',
  routes: [
    TypedGoRoute<DetailsRoute>(path: 'details/:id'),
  ],
)
class HomeRoute extends GoRouteData {
  const HomeRoute();
  @override
  Widget build(BuildContext context, GoRouterState state) => const HomeScreen();
}

class DetailsRoute extends GoRouteData {
  final String id;
  const DetailsRoute({required this.id});
  @override
  Widget build(BuildContext context, GoRouterState state) =>
      DetailsScreen(id: id);
}

// Usage — compile-time checked
const DetailsRoute(id: '123').go(context);
```

---

## 6. Data Layer — freezed & json_serializable

### Model Definition with freezed

Use `freezed` for all domain models and data transfer objects. It generates `copyWith`, `==`, `hashCode`, `toString`, and serialization:

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'user.freezed.dart';
part 'user.g.dart';

@freezed
class User with _$User {
  const factory User({
    required String id,
    required String name,
    required String email,
    @Default('') String avatarUrl,
    @Default(UserRole.member) UserRole role,
    DateTime? lastLoginAt,
  }) = _User;

  factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);
}

enum UserRole {
  @JsonValue('admin') admin,
  @JsonValue('member') member,
  @JsonValue('guest') guest,
}
```

### Union Types with freezed

Use freezed unions for sealed type hierarchies that need serialization:

```dart
@freezed
sealed class ApiResponse<T> with _$ApiResponse<T> {
  const factory ApiResponse.success({required T data}) = ApiSuccess<T>;
  const factory ApiResponse.error({
    required String message,
    required int code,
  }) = ApiError<T>;
  const factory ApiResponse.loading() = ApiLoading<T>;
}

// Exhaustive pattern matching
Widget build(BuildContext context) {
  return switch (state) {
    ApiSuccess(:final data) => DataView(data: data),
    ApiError(:final message) => ErrorView(message: message),
    ApiLoading() => const CircularProgressIndicator(),
  };
}
```

### Repository Pattern

Repositories abstract the data layer. The domain defines the interface, data implements it:

```dart
// domain/repositories/user_repository.dart
abstract interface class UserRepository {
  Future<Either<Failure, User>> getUser(String id);
  Future<Either<Failure, List<User>>> getUsers({int page = 1, int limit = 20});
  Future<Either<Failure, User>> updateUser(String id, UpdateUserRequest request);
  Future<Either<Failure, void>> deleteUser(String id);
}

// data/repositories/user_repository_impl.dart
class UserRepositoryImpl implements UserRepository {
  final UserRemoteDataSource _remoteDataSource;
  final UserLocalDataSource _localDataSource;
  final NetworkInfo _networkInfo;

  UserRepositoryImpl({
    required UserRemoteDataSource remoteDataSource,
    required UserLocalDataSource localDataSource,
    required NetworkInfo networkInfo,
  })  : _remoteDataSource = remoteDataSource,
        _localDataSource = localDataSource,
        _networkInfo = networkInfo;

  @override
  Future<Either<Failure, User>> getUser(String id) async {
    if (await _networkInfo.isConnected) {
      try {
        final user = await _remoteDataSource.getUser(id);
        await _localDataSource.cacheUser(user);
        return Right(user.toEntity());
      } on ServerException catch (e) {
        return Left(ServerFailure(e.message));
      }
    } else {
      try {
        final cachedUser = await _localDataSource.getCachedUser(id);
        return Right(cachedUser.toEntity());
      } on CacheException {
        return Left(const CacheFailure('No cached data available'));
      }
    }
  }
}
```

### HTTP Client (Dio + Retrofit)

```dart
// core/network/api_client.dart
@RestApi(baseUrl: ApiConstants.baseUrl)
abstract class ApiClient {
  factory ApiClient(Dio dio, {String baseUrl}) = _ApiClient;

  @GET('/users/{id}')
  Future<UserModel> getUser(@Path('id') String id);

  @GET('/users')
  Future<PaginatedResponse<UserModel>> getUsers(
    @Query('page') int page,
    @Query('limit') int limit,
  );

  @PUT('/users/{id}')
  Future<UserModel> updateUser(
    @Path('id') String id,
    @Body() UpdateUserRequest request,
  );

  @DELETE('/users/{id}')
  Future<void> deleteUser(@Path('id') String id);
}

// Dio configuration with interceptors
Dio createDio(SecureStorage storage) {
  final dio = Dio(BaseOptions(
    baseUrl: ApiConstants.baseUrl,
    connectTimeout: const Duration(seconds: 10),
    receiveTimeout: const Duration(seconds: 15),
    contentType: 'application/json',
  ));

  dio.interceptors.addAll([
    AuthInterceptor(storage),
    ErrorInterceptor(),
    if (kDebugMode) LogInterceptor(requestBody: true, responseBody: true),
  ]);

  return dio;
}
```

### Code Generation Commands

```bash
# One-shot generation (run after model changes)
dart run build_runner build --delete-conflicting-outputs

# Watch mode (during development)
dart run build_runner watch --delete-conflicting-outputs

# Generate for specific directory only
dart run build_runner build --build-filter="lib/features/auth/**"
```

**Convention:** Always commit generated files (`.g.dart`, `.freezed.dart`). The CI pipeline does not run code generation — it expects committed artifacts. This avoids build_runner as a CI dependency and ensures reproducible builds.

---

## 7. Material 3 Theming

### Theme Configuration

Define the theme in `app/theme.dart` using Material 3 `ColorScheme.fromSeed`:

```dart
import 'package:flutter/material.dart';

class AppTheme {
  static ThemeData light() {
    final colorScheme = ColorScheme.fromSeed(
      seedColor: const Color(0xFF1A73E8),
      brightness: Brightness.light,
    );

    return ThemeData(
      useMaterial3: true,
      colorScheme: colorScheme,
      textTheme: _textTheme(colorScheme),
      appBarTheme: AppBarTheme(
        centerTitle: false,
        backgroundColor: colorScheme.surface,
        foregroundColor: colorScheme.onSurface,
        elevation: 0,
        scrolledUnderElevation: 1,
      ),
      cardTheme: CardTheme(
        elevation: 0,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        color: colorScheme.surfaceContainerLow,
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: colorScheme.surfaceContainerHighest.withValues(alpha: 0.3),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.outline),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.outline),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.primary, width: 2),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: colorScheme.error),
        ),
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          minimumSize: const Size(double.infinity, 48),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        ),
      ),
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(
          minimumSize: const Size(double.infinity, 48),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        ),
      ),
      chipTheme: ChipThemeData(
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      ),
      snackBarTheme: SnackBarThemeData(
        behavior: SnackBarBehavior.floating,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      ),
    );
  }

  static ThemeData dark() {
    final colorScheme = ColorScheme.fromSeed(
      seedColor: const Color(0xFF1A73E8),
      brightness: Brightness.dark,
    );

    return ThemeData(
      useMaterial3: true,
      colorScheme: colorScheme,
      textTheme: _textTheme(colorScheme),
      // ... same component themes as light, adapted for dark
    );
  }

  static TextTheme _textTheme(ColorScheme colorScheme) {
    return TextTheme(
      displayLarge: TextStyle(
        fontSize: 57,
        fontWeight: FontWeight.w400,
        letterSpacing: -0.25,
        color: colorScheme.onSurface,
      ),
      headlineLarge: TextStyle(
        fontSize: 32,
        fontWeight: FontWeight.w400,
        color: colorScheme.onSurface,
      ),
      titleLarge: TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w400,
        color: colorScheme.onSurface,
      ),
      bodyLarge: TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w400,
        letterSpacing: 0.5,
        color: colorScheme.onSurface,
      ),
      labelLarge: TextStyle(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        letterSpacing: 0.1,
        color: colorScheme.onSurface,
      ),
    );
  }
}
```

### Dynamic Color (Material You)

On Android 12+, use the device wallpaper-derived color scheme:

```dart
import 'package:dynamic_color/dynamic_color.dart';

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return DynamicColorBuilder(
      builder: (lightDynamic, darkDynamic) {
        return MaterialApp.router(
          theme: lightDynamic != null
              ? ThemeData(colorScheme: lightDynamic, useMaterial3: true)
              : AppTheme.light(),
          darkTheme: darkDynamic != null
              ? ThemeData(colorScheme: darkDynamic, useMaterial3: true)
              : AppTheme.dark(),
          themeMode: ThemeMode.system,
          routerConfig: router,
        );
      },
    );
  }
}
```

### Consistent Spacing and Sizing

Never use magic numbers. Define a spacing scale:

```dart
abstract class AppSpacing {
  static const double xs = 4;
  static const double sm = 8;
  static const double md = 16;
  static const double lg = 24;
  static const double xl = 32;
  static const double xxl = 48;
}

// Usage
Padding(
  padding: const EdgeInsets.all(AppSpacing.md),
  child: Column(
    spacing: AppSpacing.sm, // Dart 3.7+ Column/Row spacing parameter
    children: [...],
  ),
)
```

---

## 8. Testing Patterns

### Test Pyramid (Flutter-specific)

```
        /\
       /  \          E2E (integration_test on device/emulator, Patrol, Maestro)
      /    \
     /------\
    /        \        Widget Tests (flutter_test)
   /          \       Component rendering, user interactions, golden tests
  /------------\
 /              \      Unit Tests (flutter_test + mockito)
/                \     Pure functions, blocs, notifiers, repositories, use cases
/------------------\
```

### Unit Tests

Test all business logic — use cases, repositories, blocs, notifiers, utility functions:

```dart
// test/features/auth/domain/usecases/login_usecase_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAuthRepository extends Mock implements AuthRepository {}

void main() {
  late LoginUseCase loginUseCase;
  late MockAuthRepository mockRepository;

  setUp(() {
    mockRepository = MockAuthRepository();
    loginUseCase = LoginUseCase(repository: mockRepository);
  });

  group('LoginUseCase', () {
    const email = 'test@example.com';
    const password = 'Password123!';
    final user = User(id: '1', name: 'Test User', email: email);

    test('returns User on successful login', () async {
      when(() => mockRepository.login(email, password))
          .thenAnswer((_) async => Right(user));

      final result = await loginUseCase(
        LoginParams(email: email, password: password),
      );

      expect(result, Right(user));
      verify(() => mockRepository.login(email, password)).called(1);
    });

    test('returns ServerFailure when repository throws', () async {
      when(() => mockRepository.login(email, password))
          .thenAnswer((_) async => Left(ServerFailure('Connection failed')));

      final result = await loginUseCase(
        LoginParams(email: email, password: password),
      );

      expect(result, isA<Left<Failure, User>>());
    });

    test('returns ValidationFailure for empty email', () async {
      final result = await loginUseCase(
        LoginParams(email: '', password: password),
      );

      expect(result, Left(ValidationFailure('Email is required')));
      verifyNever(() => mockRepository.login(any(), any()));
    });
  });
}
```

### Widget Tests

Test widget rendering, user interactions, and state changes:

```dart
// test/features/auth/presentation/screens/login_screen_test.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

void main() {
  group('LoginScreen', () {
    late MockAuthRepository mockAuthRepo;

    setUp(() {
      mockAuthRepo = MockAuthRepository();
    });

    Future<void> pumpLoginScreen(WidgetTester tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            authRepositoryProvider.overrideWithValue(mockAuthRepo),
          ],
          child: const MaterialApp(home: LoginScreen()),
        ),
      );
    }

    testWidgets('renders email and password fields', (tester) async {
      await pumpLoginScreen(tester);

      expect(find.byKey(const Key('email_field')), findsOneWidget);
      expect(find.byKey(const Key('password_field')), findsOneWidget);
      expect(find.byKey(const Key('login_button')), findsOneWidget);
    });

    testWidgets('shows validation errors for empty fields', (tester) async {
      await pumpLoginScreen(tester);

      await tester.tap(find.byKey(const Key('login_button')));
      await tester.pumpAndSettle();

      expect(find.text('Email is required'), findsOneWidget);
      expect(find.text('Password is required'), findsOneWidget);
    });

    testWidgets('shows loading indicator during login', (tester) async {
      when(() => mockAuthRepo.login(any(), any()))
          .thenAnswer((_) async {
        await Future.delayed(const Duration(seconds: 2));
        return Right(User(id: '1', name: 'Test', email: 'test@test.com'));
      });

      await pumpLoginScreen(tester);

      await tester.enterText(
        find.byKey(const Key('email_field')), 'test@test.com',
      );
      await tester.enterText(
        find.byKey(const Key('password_field')), 'Password123!',
      );
      await tester.tap(find.byKey(const Key('login_button')));
      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('navigates to home on successful login', (tester) async {
      when(() => mockAuthRepo.login(any(), any()))
          .thenAnswer((_) async => Right(
            User(id: '1', name: 'Test', email: 'test@test.com'),
          ));

      await pumpLoginScreen(tester);

      await tester.enterText(
        find.byKey(const Key('email_field')), 'test@test.com',
      );
      await tester.enterText(
        find.byKey(const Key('password_field')), 'Password123!',
      );
      await tester.tap(find.byKey(const Key('login_button')));
      await tester.pumpAndSettle();

      // Verify navigation occurred (depends on test setup)
    });
  });
}
```

### Golden Tests

Golden tests capture pixel-perfect screenshots and detect visual regressions:

```dart
testWidgets('UserProfileCard matches golden', (tester) async {
  await tester.pumpWidget(
    MaterialApp(
      theme: AppTheme.light(),
      home: Scaffold(
        body: UserProfileCard(
          user: User(
            id: '1',
            name: 'Jane Doe',
            email: 'jane@example.com',
            avatarUrl: '',
          ),
        ),
      ),
    ),
  );

  await expectLater(
    find.byType(UserProfileCard),
    matchesGoldenFile('goldens/user_profile_card.png'),
  );
});

// Update goldens when intentional UI changes are made:
// flutter test --update-goldens
```

### Integration Tests (On-Device)

```dart
// integration_test/auth_flow_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Auth Flow', () {
    testWidgets('complete login flow', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Should be on login screen
      expect(find.byKey(const Key('login_screen')), findsOneWidget);

      // Enter credentials
      await tester.enterText(
        find.byKey(const Key('email_field')), 'test@example.com',
      );
      await tester.enterText(
        find.byKey(const Key('password_field')), 'Password123!',
      );
      await tester.tap(find.byKey(const Key('login_button')));
      await tester.pumpAndSettle();

      // Should navigate to home
      expect(find.byKey(const Key('home_screen')), findsOneWidget);
    });
  });
}
```

### Robot Pattern for Integration Tests

Encapsulate screen interactions in robot classes for readable, maintainable tests:

```dart
// integration_test/robots/login_robot.dart
class LoginRobot {
  final WidgetTester tester;
  LoginRobot(this.tester);

  Future<void> enterEmail(String email) async {
    await tester.enterText(find.byKey(const Key('email_field')), email);
  }

  Future<void> enterPassword(String password) async {
    await tester.enterText(find.byKey(const Key('password_field')), password);
  }

  Future<void> tapLogin() async {
    await tester.tap(find.byKey(const Key('login_button')));
    await tester.pumpAndSettle();
  }

  Future<void> login({
    required String email,
    required String password,
  }) async {
    await enterEmail(email);
    await enterPassword(password);
    await tapLogin();
  }

  void expectLoginScreen() {
    expect(find.byKey(const Key('login_screen')), findsOneWidget);
  }

  void expectError(String message) {
    expect(find.text(message), findsOneWidget);
  }
}

// Usage in tests
testWidgets('login with invalid credentials shows error', (tester) async {
  app.main();
  await tester.pumpAndSettle();

  final login = LoginRobot(tester);
  login.expectLoginScreen();
  await login.login(email: 'bad@test.com', password: 'wrong');
  login.expectError('Invalid email or password');
});
```

### Test Configuration

```dart
// test/helpers/pump_app.dart
extension PumpApp on WidgetTester {
  Future<void> pumpApp(
    Widget widget, {
    List<Override> overrides = const [],
    GoRouter? router,
  }) async {
    await pumpWidget(
      ProviderScope(
        overrides: overrides,
        child: MaterialApp(
          theme: AppTheme.light(),
          home: widget,
        ),
      ),
    );
  }

  Future<void> pumpRouterApp({
    List<Override> overrides = const [],
    required GoRouter router,
  }) async {
    await pumpWidget(
      ProviderScope(
        overrides: overrides,
        child: MaterialApp.router(
          theme: AppTheme.light(),
          routerConfig: router,
        ),
      ),
    );
  }
}
```

### Test Commands

```bash
# Run all unit and widget tests
flutter test

# Run with coverage
flutter test --coverage

# Run specific test file
flutter test test/features/auth/domain/usecases/login_usecase_test.dart

# Run tests matching a pattern
flutter test --name "LoginUseCase"

# Run integration tests on connected device
flutter test integration_test/

# Run integration tests on specific device
flutter test integration_test/ -d <device_id>

# Generate coverage report (requires lcov)
flutter test --coverage
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html

# Update golden files
flutter test --update-goldens
```

---

## 9. Platform Channels

### Method Channels

Use platform channels to call native APIs not exposed through Flutter plugins:

```dart
// lib/core/platform/battery_channel.dart
import 'package:flutter/services.dart';

class BatteryChannel {
  static const _channel = MethodChannel('com.example.app/battery');

  Future<int> getBatteryLevel() async {
    try {
      final level = await _channel.invokeMethod<int>('getBatteryLevel');
      return level ?? -1;
    } on PlatformException catch (e) {
      throw BatteryException('Failed to get battery level: ${e.message}');
    }
  }

  Future<bool> isCharging() async {
    try {
      final charging = await _channel.invokeMethod<bool>('isCharging');
      return charging ?? false;
    } on PlatformException catch (e) {
      throw BatteryException('Failed to get charging status: ${e.message}');
    }
  }
}
```

### Android Implementation (Kotlin)

```kotlin
// android/app/src/main/kotlin/com/example/app/MainActivity.kt
package com.example.app

import android.os.BatteryManager
import android.content.Context
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel

class MainActivity : FlutterActivity() {
    private val CHANNEL = "com.example.app/battery"

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, CHANNEL)
            .setMethodCallHandler { call, result ->
                when (call.method) {
                    "getBatteryLevel" -> {
                        val batteryManager =
                            getSystemService(Context.BATTERY_SERVICE) as BatteryManager
                        val level = batteryManager
                            .getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY)
                        result.success(level)
                    }
                    "isCharging" -> {
                        val batteryManager =
                            getSystemService(Context.BATTERY_SERVICE) as BatteryManager
                        val charging = batteryManager
                            .isCharging
                        result.success(charging)
                    }
                    else -> result.notImplemented()
                }
            }
    }
}
```

### iOS Implementation (Swift)

```swift
// ios/Runner/AppDelegate.swift
import Flutter
import UIKit

@main
@objc class AppDelegate: FlutterAppDelegate {
    override func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        let controller = window?.rootViewController as! FlutterViewController
        let batteryChannel = FlutterMethodChannel(
            name: "com.example.app/battery",
            binaryMessenger: controller.binaryMessenger
        )

        batteryChannel.setMethodCallHandler { (call, result) in
            switch call.method {
            case "getBatteryLevel":
                UIDevice.current.isBatteryMonitoringEnabled = true
                let level = Int(UIDevice.current.batteryLevel * 100)
                result(level)
            case "isCharging":
                UIDevice.current.isBatteryMonitoringEnabled = true
                let state = UIDevice.current.batteryState
                result(state == .charging || state == .full)
            default:
                result(FlutterMethodNotImplemented)
            }
        }

        GeneratedPluginRegistrant.register(with: self)
        return super.application(application, didFinishLaunchingWithOptions: launchOptions)
    }
}
```

### Event Channels (Streaming)

For continuous data streams from native to Dart:

```dart
// lib/core/platform/location_channel.dart
class LocationChannel {
  static const _eventChannel = EventChannel('com.example.app/location');

  Stream<LocationData> get locationStream {
    return _eventChannel.receiveBroadcastStream().map((event) {
      final map = Map<String, dynamic>.from(event as Map);
      return LocationData(
        latitude: map['latitude'] as double,
        longitude: map['longitude'] as double,
        accuracy: map['accuracy'] as double,
      );
    });
  }
}
```

### Pigeon (Type-Safe Code Generation)

For complex platform channel interfaces, use Pigeon to generate type-safe bindings:

```dart
// pigeons/battery.dart
import 'package:pigeon/pigeon.dart';

@ConfigurePigeon(PigeonOptions(
  dartOut: 'lib/core/platform/battery_api.g.dart',
  kotlinOut: 'android/app/src/main/kotlin/com/example/app/BatteryApi.g.kt',
  swiftOut: 'ios/Runner/BatteryApi.g.swift',
))

class BatteryInfo {
  final int level;
  final bool isCharging;

  BatteryInfo({required this.level, required this.isCharging});
}

@HostApi()
abstract class BatteryApi {
  BatteryInfo getBatteryInfo();
}
```

```bash
# Generate platform bindings
dart run pigeon --input pigeons/battery.dart
```

### Testing Platform Channels

```dart
void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  group('BatteryChannel', () {
    late BatteryChannel batteryChannel;

    setUp(() {
      batteryChannel = BatteryChannel();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
        const MethodChannel('com.example.app/battery'),
        (message) async {
          if (message.method == 'getBatteryLevel') return 85;
          if (message.method == 'isCharging') return true;
          return null;
        },
      );
    });

    test('returns battery level', () async {
      final level = await batteryChannel.getBatteryLevel();
      expect(level, 85);
    });

    test('returns charging status', () async {
      final charging = await batteryChannel.isCharging();
      expect(charging, isTrue);
    });
  });
}
```

### Federated Plugin Architecture

For reusable platform-specific functionality, create federated plugins:

```
my_plugin/
├── my_plugin/                   # App-facing package (API + platform interface)
│   ├── lib/
│   │   └── my_plugin.dart
│   └── pubspec.yaml
├── my_plugin_platform_interface/ # Platform interface (abstract class)
│   ├── lib/
│   │   └── my_plugin_platform_interface.dart
│   └── pubspec.yaml
├── my_plugin_android/           # Android implementation
│   ├── android/
│   ├── lib/
│   └── pubspec.yaml
├── my_plugin_ios/               # iOS implementation
│   ├── ios/
│   ├── lib/
│   └── pubspec.yaml
└── my_plugin_web/               # Web implementation (optional)
    ├── lib/
    └── pubspec.yaml
```

---

## 10. Performance Profiling

### Flutter DevTools

Flutter DevTools is the primary profiling tool. Launch it from the IDE or CLI:

```bash
# Open DevTools in browser
flutter pub global activate devtools
dart devtools

# Run app in profile mode (required for accurate performance data)
flutter run --profile

# Run app in release mode (production performance)
flutter run --release
```

### Performance Checklist

| Check | Tool | Target |
|---|---|---|
| Frame rendering | DevTools Performance tab | 16ms per frame (60 FPS), 8ms for 120 FPS |
| Jank detection | DevTools Performance overlay | Zero red frames in critical flows |
| Widget rebuilds | DevTools Inspector | Minimal unnecessary rebuilds |
| Memory leaks | DevTools Memory tab | Stable heap after navigation cycles |
| Network latency | DevTools Network tab | API calls under 500ms P95 |
| App size | `flutter build --analyze-size` | Under 20MB for iOS, 15MB for Android |
| Startup time | `flutter run --trace-startup` | Under 2 seconds cold start |
| Shader warmup | DevTools Timeline | No shader compilation jank with Impeller |

### Common Performance Patterns

**const constructors** — Mark widgets `const` to prevent unnecessary rebuilds:

```dart
// Good — widget instance is canonicalized, never rebuilt
const SizedBox(height: 16);
const Text('Static text');
const Icon(Icons.check);

// Bad — creates new instance on every build
SizedBox(height: 16);         // Missing const
Text('Static text');           // Missing const
```

**RepaintBoundary** — Isolate expensive paint operations:

```dart
// Wrap animated or frequently-updating widgets
RepaintBoundary(
  child: CustomPaint(
    painter: ComplexChartPainter(data: chartData),
  ),
)
```

**ListView.builder** — Always use for long or dynamic lists:

```dart
// Good — lazy, only builds visible items
ListView.builder(
  itemCount: items.length,
  itemBuilder: (context, index) => ItemTile(item: items[index]),
)

// Bad — builds all items eagerly, even off-screen
ListView(
  children: items.map((item) => ItemTile(item: item)).toList(),
)
```

**Avoid rebuilding the entire tree** — Use `select` with Riverpod or `BlocSelector`:

```dart
// Good — only rebuilds when name changes
final name = ref.watch(userProvider.select((u) => u.name));

// Bad — rebuilds on any user property change
final user = ref.watch(userProvider);
// Only uses user.name in the build
```

**Image optimization:**

```dart
// Specify exact dimensions to avoid layout shifts and memory waste
CachedNetworkImage(
  imageUrl: url,
  width: 200,
  height: 200,
  memCacheWidth: 400,  // 2x for high-DPI
  memCacheHeight: 400,
  fit: BoxFit.cover,
  placeholder: (_, __) => const ShimmerPlaceholder(),
  errorWidget: (_, __, ___) => const Icon(Icons.broken_image),
)
```

**Compute-heavy work off the main isolate:**

```dart
// Use Isolate.run for expensive computations
final parsed = await Isolate.run(() {
  return heavyJsonParsing(rawData);
});

// Or use compute() for top-level functions
final result = await compute(expensiveFunction, inputData);
```

### Profile Mode Best Practices

- Always profile in profile mode (`--profile`), never debug mode
- Profile on real devices, not emulators — emulator performance is not representative
- Profile on the lowest-spec target device to find real bottlenecks
- Use `Timeline.startSync` / `Timeline.finishSync` for custom trace events
- Enable the performance overlay in the app: `MaterialApp(showPerformanceOverlay: true)`

### App Size Analysis

```bash
# Build with size analysis
flutter build apk --analyze-size
flutter build ios --analyze-size

# The output shows a treemap of what contributes to app size
# Common culprits:
# - Unoptimized images (use WebP, compress PNGs)
# - Unused fonts (only include needed weights)
# - Native libraries from unused plugins
# - Generated code bloat (review freezed/json_serializable output)

# Deferred components (Android app bundles)
# Split features into deferred libraries for on-demand download
```

---

## 11. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`.

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping, consistent spacing |
| **labels** | Above field, always visible, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | Inline below field, multi-cue (icon + text + border color), scroll to first error |
| **accessibility** | Semantic labels, focus traversal order, screen reader announcements |
| **mobile** | Min 48dp touch targets, appropriate keyboard types, autocomplete hints |
| **cta** | Outcome-focused text ("Create Account" not "Submit"), loading state on button |
| **trust** | Minimal fields, "(optional)" markers, post-submit confirmation |
| **performance** | Debounce validation, no unnecessary rebuilds, lazy validation |

### Form Implementation

```dart
class LoginForm extends ConsumerStatefulWidget {
  const LoginForm({super.key});

  @override
  ConsumerState<LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends ConsumerState<LoginForm> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _emailFocusNode = FocusNode();
  final _passwordFocusNode = FocusNode();
  bool _submitted = false;
  bool _isLoading = false;

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    _emailFocusNode.dispose();
    _passwordFocusNode.dispose();
    super.dispose();
  }

  String? _validateEmail(String? value) {
    if (!_submitted) return null; // No validation before first submit
    if (value == null || value.isEmpty) return 'Email is required';
    if (!RegExp(r'^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$').hasMatch(value)) {
      return 'Enter a valid email address';
    }
    return null;
  }

  String? _validatePassword(String? value) {
    if (!_submitted) return null;
    if (value == null || value.isEmpty) return 'Password is required';
    if (value.length < 8) return 'Password must be at least 8 characters';
    return null;
  }

  Future<void> _onSubmit() async {
    setState(() => _submitted = true);

    if (!_formKey.currentState!.validate()) {
      // Focus the first field with an error
      if (_validateEmail(_emailController.text) != null) {
        _emailFocusNode.requestFocus();
      } else {
        _passwordFocusNode.requestFocus();
      }
      return;
    }

    setState(() => _isLoading = true);

    try {
      await ref.read(authNotifierProvider.notifier).login(
        _emailController.text,
        _passwordController.text,
      );
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(e.toString())),
        );
      }
    } finally {
      if (mounted) setState(() => _isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: AutofillGroup(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          spacing: AppSpacing.md,
          children: [
            TextFormField(
              key: const Key('email_field'),
              controller: _emailController,
              focusNode: _emailFocusNode,
              decoration: const InputDecoration(
                labelText: 'Email',
                hintText: 'you@example.com',
                prefixIcon: Icon(Icons.email_outlined),
              ),
              keyboardType: TextInputType.emailAddress,
              textInputAction: TextInputAction.next,
              autofillHints: const [AutofillHints.email],
              validator: _validateEmail,
              onFieldSubmitted: (_) => _passwordFocusNode.requestFocus(),
            ),
            TextFormField(
              key: const Key('password_field'),
              controller: _passwordController,
              focusNode: _passwordFocusNode,
              decoration: const InputDecoration(
                labelText: 'Password',
                prefixIcon: Icon(Icons.lock_outlined),
              ),
              obscureText: true,
              textInputAction: TextInputAction.done,
              autofillHints: const [AutofillHints.password],
              validator: _validatePassword,
              onFieldSubmitted: (_) => _onSubmit(),
            ),
            const SizedBox(height: AppSpacing.sm),
            FilledButton(
              key: const Key('login_button'),
              onPressed: _isLoading ? null : _onSubmit,
              child: _isLoading
                  ? const SizedBox(
                      height: 20,
                      width: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Text('Log In'),
            ),
          ],
        ),
      ),
    );
  }
}
```

### Form Accessibility Requirements

- Every `TextFormField` must have a `labelText` or `Semantics` label
- Focus traversal must follow visual order — use `FocusTraversalGroup` if needed
- Error messages must be announced to screen readers — Flutter's `TextFormField` does this automatically via `Semantics`
- Touch targets must be at minimum 48x48dp — enforce via `InputDecorationTheme` with `contentPadding`
- Use `AutofillGroup` to enable platform autofill suggestions
- Use correct `TextInputType` for each field (email, phone, number, etc.)
- Use `textInputAction` to control keyboard action button (next, done, go)

---

## 12. Security

### Secure Storage

Never store sensitive data in `SharedPreferences` — it is plaintext on disk. Use `flutter_secure_storage`:

```dart
class SecureStorageService {
  final FlutterSecureStorage _storage;

  SecureStorageService({FlutterSecureStorage? storage})
      : _storage = storage ?? const FlutterSecureStorage(
          aOptions: AndroidOptions(encryptedSharedPreferences: true),
          iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
        );

  Future<void> saveToken(String token) async {
    await _storage.write(key: 'auth_token', value: token);
  }

  Future<String?> getToken() async {
    return _storage.read(key: 'auth_token');
  }

  Future<void> deleteToken() async {
    await _storage.delete(key: 'auth_token');
  }

  Future<void> clearAll() async {
    await _storage.deleteAll();
  }
}
```

### Network Security

```dart
// Auth interceptor — attach tokens to requests
class AuthInterceptor extends Interceptor {
  final SecureStorageService _storage;

  AuthInterceptor(this._storage);

  @override
  Future<void> onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    final token = await _storage.getToken();
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    if (err.response?.statusCode == 401) {
      _storage.deleteToken();
      // Navigate to login — use a global navigator key or event bus
    }
    handler.next(err);
  }
}

// Certificate pinning (for high-security apps)
Dio createPinnedDio() {
  final dio = Dio();
  (dio.httpClientAdapter as IOHttpClientAdapter).createHttpClient = () {
    final client = HttpClient();
    client.badCertificateCallback = (cert, host, port) {
      // Compare cert fingerprint against known pins
      return verifyPin(cert);
    };
    return client;
  };
  return dio;
}
```

### Input Validation

Validate all user input on the client and the server. Client validation is a UX convenience, not a security boundary:

```dart
abstract class Validators {
  static String? email(String? value) {
    if (value == null || value.isEmpty) return 'Email is required';
    if (!RegExp(r'^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$').hasMatch(value)) {
      return 'Enter a valid email address';
    }
    return null;
  }

  static String? password(String? value) {
    if (value == null || value.isEmpty) return 'Password is required';
    if (value.length < 8) return 'Must be at least 8 characters';
    if (!value.contains(RegExp(r'[A-Z]'))) return 'Must contain an uppercase letter';
    if (!value.contains(RegExp(r'[0-9]'))) return 'Must contain a number';
    return null;
  }

  static String? notEmpty(String? value, String fieldName) {
    if (value == null || value.trim().isEmpty) return '$fieldName is required';
    return null;
  }
}
```

### Obfuscation and ProGuard

```bash
# Build with Dart obfuscation (release builds)
flutter build apk --obfuscate --split-debug-info=debug-info/
flutter build ios --obfuscate --split-debug-info=debug-info/

# Keep debug symbols for crash reporting (upload to Firebase Crashlytics, Sentry, etc.)
# The debug-info/ directory contains the symbol map
```

### Platform-Specific Security

| Platform | Requirement | Implementation |
|---|---|---|
| Android | Network security config | `android/app/src/main/res/xml/network_security_config.xml` — disallow cleartext traffic |
| Android | ProGuard rules | `android/app/proguard-rules.pro` — keep serialization classes |
| iOS | App Transport Security | `Info.plist` — `NSAppTransportSecurity` set to require HTTPS |
| iOS | Keychain access | `flutter_secure_storage` with appropriate accessibility level |
| Both | Root/jailbreak detection | Use `flutter_jailbreak_detection` or `safe_device` for sensitive apps |
| Both | Screenshot prevention | Platform channel to `FLAG_SECURE` (Android) / `UITextField.isSecureTextEntry` trick (iOS) |

### Secrets Management

- **Never** hardcode API keys, secrets, or tokens in Dart source
- Use `--dart-define` for build-time configuration:

```bash
flutter run --dart-define=API_KEY=abc123 --dart-define=ENV=staging

# Access in Dart
const apiKey = String.fromEnvironment('API_KEY');
const environment = String.fromEnvironment('ENV', defaultValue: 'production');
```

- For complex configs, use `--dart-define-from-file`:

```bash
flutter run --dart-define-from-file=config/dev.json
```

- Store secrets in CI/CD environment variables, not in the repository

---

## 13. Development Workflow

### Feature Development Cycle (Flutter-specific)

```
1. Create feature branch from main
2. Define domain entities and repository interfaces
3. Write failing unit tests for use cases
4. Implement use cases
5. Write failing widget tests for screens
6. Implement data layer (models, datasources, repository impls)
7. Implement presentation layer (screens, providers/blocs, widgets)
8. Run code generation: dart run build_runner build --delete-conflicting-outputs
9. Run: flutter test
10. Run: flutter analyze
11. Run: dart format .
12. Refactor while green
13. Run integration tests on device
14. Create PR
```

### Common Commands

```bash
# Development
flutter run                              # Run on connected device (debug)
flutter run --profile                    # Profile mode (performance testing)
flutter run --release                    # Release mode
flutter run -d chrome                    # Run on web
flutter run -d macos                     # Run on macOS desktop

# Testing
flutter test                             # Run all unit and widget tests
flutter test --coverage                  # Generate coverage report
flutter test test/features/auth/         # Run feature-specific tests
flutter test integration_test/           # Run integration tests on device
flutter test --update-goldens            # Update golden test files

# Code Quality
flutter analyze                          # Static analysis (lint rules)
dart format .                            # Format all Dart files
dart format --set-exit-if-changed .      # CI check — fail if unformatted
dart fix --apply                         # Apply automated fixes

# Code Generation
dart run build_runner build --delete-conflicting-outputs
dart run build_runner watch --delete-conflicting-outputs

# Building
flutter build apk                        # Android APK
flutter build appbundle                  # Android App Bundle (Play Store)
flutter build ios                        # iOS (requires Xcode)
flutter build web                        # Web
flutter build macos                      # macOS desktop
flutter build apk --analyze-size         # Build with size analysis

# Dependencies
flutter pub get                          # Install dependencies
flutter pub upgrade                      # Upgrade to latest compatible versions
flutter pub outdated                     # Check for outdated packages
flutter pub deps                         # Dependency tree

# Device management
flutter devices                          # List connected devices
flutter emulators                        # List available emulators
flutter emulators --launch <name>        # Launch an emulator

# Cleaning
flutter clean                            # Remove build artifacts
flutter pub cache clean                  # Clear pub cache
```

### Analysis Options

```yaml
# analysis_options.yaml
include: package:very_good_analysis/analysis_options.yaml

analyzer:
  exclude:
    - "**/*.g.dart"
    - "**/*.freezed.dart"
    - "lib/generated/**"
  errors:
    invalid_annotation_target: ignore  # Required for freezed + json_serializable
  language:
    strict-casts: true
    strict-raw-types: true
    strict-inference: true

linter:
  rules:
    # Additional rules beyond very_good_analysis
    - always_declare_return_types
    - avoid_dynamic_calls
    - avoid_type_to_string
    - cancel_subscriptions
    - close_sinks
    - literal_only_boolean_expressions
    - no_adjacent_strings_in_list
    - prefer_final_in_for_each
    - prefer_final_locals
    - test_types_in_equals
    - throw_in_finally
    - unnecessary_statements
    - unsafe_html
```

---

## 14. CI/CD Pipeline

### GitHub Actions

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.29.0'
          channel: 'stable'
          cache: true
      - run: flutter pub get
      - run: dart format --set-exit-if-changed .
      - run: flutter analyze --fatal-infos
      - run: flutter test --coverage
      - name: Check coverage threshold
        run: |
          COVERAGE=$(lcov --summary coverage/lcov.info 2>&1 | grep "lines" | grep -o '[0-9.]*%' | head -1)
          echo "Coverage: $COVERAGE"

  test-android:
    needs: analyze
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.29.0'
          channel: 'stable'
          cache: true
      - run: flutter pub get
      - run: flutter build apk --debug
      # Integration tests run on Firebase Test Lab or a self-hosted runner

  test-ios:
    needs: analyze
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.29.0'
          channel: 'stable'
          cache: true
      - run: flutter pub get
      - run: flutter build ios --no-codesign
      # Integration tests run on physical devices via Xcode Cloud or Firebase Test Lab

  deploy-android:
    if: github.ref == 'refs/heads/main'
    needs: [test-android, test-ios]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.29.0'
          channel: 'stable'
          cache: true
      - run: flutter pub get
      - run: flutter build appbundle --release --obfuscate --split-debug-info=debug-info/
      - name: Upload to Play Store
        uses: r0adkll/upload-google-play@v1
        with:
          serviceAccountJsonPlainText: ${{ secrets.PLAY_STORE_SERVICE_ACCOUNT }}
          packageName: com.example.app
          releaseFiles: build/app/outputs/bundle/release/app-release.aab
          track: internal

  deploy-ios:
    if: github.ref == 'refs/heads/main'
    needs: [test-android, test-ios]
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.29.0'
          channel: 'stable'
          cache: true
      - run: flutter pub get
      - run: flutter build ipa --release --obfuscate --split-debug-info=debug-info/
      - name: Upload to App Store Connect
        run: xcrun altool --upload-app --file build/ios/ipa/*.ipa --apiKey ${{ secrets.APP_STORE_API_KEY }} --apiIssuer ${{ secrets.APP_STORE_API_ISSUER }}
```

### Fastlane (Alternative)

For more complex release workflows, use Fastlane:

```ruby
# android/fastlane/Fastfile
default_platform(:android)

platform :android do
  desc "Deploy to internal testing track"
  lane :internal do
    sh("flutter build appbundle --release --obfuscate --split-debug-info=debug-info/")
    upload_to_play_store(
      track: 'internal',
      aab: '../build/app/outputs/bundle/release/app-release.aab'
    )
  end

  desc "Promote internal to production"
  lane :promote do
    upload_to_play_store(
      track: 'internal',
      track_promote_to: 'production'
    )
  end
end
```

```ruby
# ios/fastlane/Fastfile
default_platform(:ios)

platform :ios do
  desc "Deploy to TestFlight"
  lane :beta do
    sh("flutter build ipa --release --obfuscate --split-debug-info=debug-info/")
    upload_to_testflight(
      ipa: '../build/ios/ipa/Runner.ipa'
    )
  end

  desc "Promote to App Store"
  lane :release do
    deliver(
      submit_for_review: true,
      automatic_release: false
    )
  end
end
```

### Version Management

Use a single source of truth for versioning in `pubspec.yaml`:

```yaml
# pubspec.yaml
version: 1.2.3+45
# 1.2.3 = marketing version (semver)
# 45 = build number (monotonically increasing, used by stores)
```

Automate build number in CI:

```bash
# Set build number from CI run number
flutter build appbundle --build-number=${{ github.run_number }}
```

---

## 15. Coverage Enforcement

### Configuration

```yaml
# pubspec.yaml — add coverage dependencies
dev_dependencies:
  very_good_analysis: ^7.0.0
  test: any
  mocktail: ^1.0.0
```

### Coverage Commands

```bash
# Generate coverage
flutter test --coverage

# View coverage report (requires lcov)
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html

# Filter out generated code from coverage
lcov --remove coverage/lcov.info \
  '**/*.g.dart' \
  '**/*.freezed.dart' \
  '**/generated/**' \
  '**/l10n/**' \
  -o coverage/lcov_filtered.info

# Check coverage threshold in CI
lcov --summary coverage/lcov_filtered.info
```

### Coverage Exclusions

Exclude generated code and platform-specific bootstrapping from coverage:

```bash
# Exclude patterns (use in CI script)
EXCLUDE_PATTERNS=(
  '**/*.g.dart'
  '**/*.freezed.dart'
  '**/generated/**'
  '**/l10n/**'
  '**/main.dart'
  '**/firebase_options.dart'
  '**/app/router.g.dart'
)
```

### Minimum Coverage Gate

Enforce a minimum coverage threshold in CI. Target is 100% (per CLAUDE.md core rules):

```bash
#!/bin/bash
# scripts/check_coverage.sh
set -e

flutter test --coverage

# Filter generated files
lcov --remove coverage/lcov.info \
  '**/*.g.dart' \
  '**/*.freezed.dart' \
  '**/generated/**' \
  '**/l10n/**' \
  -o coverage/lcov_filtered.info

# Extract coverage percentage
COVERAGE=$(lcov --summary coverage/lcov_filtered.info 2>&1 | grep "lines" | grep -oP '[0-9.]+%' | head -1 | tr -d '%')

THRESHOLD=100

if (( $(echo "$COVERAGE < $THRESHOLD" | bc -l) )); then
  echo "FAIL: Coverage $COVERAGE% is below threshold $THRESHOLD%"
  exit 1
fi

echo "PASS: Coverage $COVERAGE% meets threshold $THRESHOLD%"
```

---

## 16. Anti-Patterns (Flutter-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Using `setState` for app-wide state | Use Riverpod providers or Bloc for any state shared across widgets |
| Putting business logic in widgets | Extract to use cases, notifiers, or blocs — widgets only render and dispatch |
| Using `BuildContext` in async gaps | Capture the ref/bloc/callback before `await`, check `mounted` before using context |
| Nesting `FutureBuilder` or `StreamBuilder` | Use Riverpod's `FutureProvider` / `StreamProvider` or Bloc pattern instead |
| Building custom HTTP wrappers | Use Dio with interceptors, or Retrofit for type-safe API clients |
| Skipping `const` constructors | Mark every possible widget and value `const` — the compiler cannot do this for you |
| Using `MediaQuery.of(context)` in build methods | Use `MediaQuery.sizeOf(context)` or `MediaQuery.paddingOf(context)` to avoid unnecessary rebuilds when unrelated media query properties change |
| Importing across feature boundaries | Features must not import from each other — communicate through domain interfaces or app-level state |
| Using `dynamic` types | Enable `strict-casts`, `strict-raw-types`, `strict-inference` in analysis_options.yaml and fix all issues |
| Putting API keys in Dart source | Use `--dart-define` or `--dart-define-from-file` for build-time secrets, never hardcode |
| Using `SharedPreferences` for tokens | Use `flutter_secure_storage` for any sensitive data (tokens, credentials, PII) |
| Testing with `find.text()` for form fields | Use `find.byKey(const Key('field_name'))` — text content changes with i18n, keys do not |
| Using `ListView` for large lists | Use `ListView.builder` (or `FlashList`) for lazy rendering — `ListView(children:)` builds all items eagerly |
| Not disposing controllers | Always dispose `TextEditingController`, `ScrollController`, `AnimationController`, `FocusNode` in `dispose()` |
| Blocking the main isolate with heavy computation | Use `Isolate.run()` or `compute()` for JSON parsing, image processing, or any work over 16ms |
| Using `Color.withOpacity()` | Use `Color.withValues(alpha:)` (Flutter 3.29+) — `withOpacity` is deprecated and creates unnecessary allocations |
| Writing widget tests without `pumpAndSettle` | After tapping buttons or entering text, call `await tester.pumpAndSettle()` to process animations and async frame callbacks |
| Ignoring `flutter analyze` warnings | Treat all analysis warnings as errors in CI (`flutter analyze --fatal-infos`) |
| Building forms without `AutofillGroup` | Always wrap forms in `AutofillGroup` for platform autofill support — required for password managers |
| Missing error boundaries | Wrap critical subtrees in `ErrorWidget.builder` customization and use `runZonedGuarded` in main |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Flutter patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:flutter&title=[Flutter]%20)**

Use the `patterns:flutter` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
