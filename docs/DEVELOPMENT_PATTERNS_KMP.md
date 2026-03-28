# Development Patterns — KMP Stack

Kotlin Multiplatform / Compose Multiplatform / Ktor / SQLDelight / Koin

This document captures stack-specific patterns, conventions, and decisions for Kotlin Multiplatform (KMP) + Compose Multiplatform projects. It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building cross-platform apps in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure KMP shared modules, write expect/actual declarations, build Compose Multiplatform UI, manage state, test across platforms, integrate with Ktor for networking, persist data with SQLDelight, inject dependencies with Koin, and ship to Android and iOS.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Kotlin | 2.2+ | K2 compiler (default), context parameters (preview), explicit backing fields |
| Kotlin Multiplatform | 2.2+ | Plugin: `org.jetbrains.kotlin.multiplatform` |
| Compose Multiplatform | 1.8+ | JetBrains Compose for Android, iOS, Desktop, Web |
| Compose Compiler | Kotlin 2.2+ bundled | Integrated into Kotlin compiler since 2.0 |
| Android Gradle Plugin | 8.8+ | Required for Kotlin 2.2 compatibility |
| Ktor Client | 3.1+ | Multiplatform HTTP client with engine-per-platform |
| SQLDelight | 2.1+ | Multiplatform SQL persistence with type-safe queries |
| Koin | 4.1+ | Multiplatform dependency injection, Compose integration |
| Kotlinx Serialization | 1.8+ | Multiplatform JSON/CBOR/Protobuf serialization |
| Kotlinx Coroutines | 1.10+ | Structured concurrency, Flow, platform dispatchers |
| Kotlinx DateTime | 0.6+ | Multiplatform date/time (replaces java.time in shared) |
| Kotlinx IO | 0.7+ | Multiplatform I/O primitives |
| Jetpack Navigation | 2.9+ | Type-safe navigation for Compose (Android + multiplatform) |
| Coil | 3.1+ | Multiplatform image loading for Compose |
| Multiplatform Settings | 1.3+ | Key-value storage (SharedPreferences / NSUserDefaults) |
| Android minSdk | 24 | Android 7.0+ |
| Android targetSdk | 35 | Android 15 |
| iOS deployment target | 16.0 | iPhone 8+ |
| Xcode | 16+ | Required for iOS builds |
| JDK | 21+ | LTS, required by AGP 8.8+ |
| Gradle | 8.12+ | Kotlin DSL only, version catalogs required |

### Version Constraint Policy

Use Gradle version catalogs (`libs.versions.toml`) with exact version pins:

```toml
# gradle/libs.versions.toml

[versions]
kotlin = "2.2.0"
compose-multiplatform = "1.8.0"
agp = "8.8.2"
ktor = "3.1.1"
sqldelight = "2.1.0"
koin = "4.1.0"
kotlinx-serialization = "1.8.0"
kotlinx-coroutines = "1.10.1"
kotlinx-datetime = "0.6.2"
coil = "3.1.0"

[libraries]
ktor-client-core = { module = "io.ktor:ktor-client-core", version.ref = "ktor" }
ktor-client-content-negotiation = { module = "io.ktor:ktor-client-content-negotiation", version.ref = "ktor" }
ktor-serialization-json = { module = "io.ktor:ktor-serialization-kotlinx-json", version.ref = "ktor" }
ktor-client-okhttp = { module = "io.ktor:ktor-client-okhttp", version.ref = "ktor" }
ktor-client-darwin = { module = "io.ktor:ktor-client-darwin", version.ref = "ktor" }
sqldelight-runtime = { module = "app.cash.sqldelight:runtime", version.ref = "sqldelight" }
sqldelight-coroutines = { module = "app.cash.sqldelight:coroutines-extensions", version.ref = "sqldelight" }
sqldelight-android-driver = { module = "app.cash.sqldelight:android-driver", version.ref = "sqldelight" }
sqldelight-native-driver = { module = "app.cash.sqldelight:native-driver", version.ref = "sqldelight" }
koin-core = { module = "io.insert-koin:koin-core", version.ref = "koin" }
koin-compose = { module = "io.insert-koin:koin-compose", version.ref = "koin" }
koin-compose-viewmodel = { module = "io.insert-koin:koin-compose-viewmodel", version.ref = "koin" }
kotlinx-serialization-json = { module = "org.jetbrains.kotlinx:kotlinx-serialization-json", version.ref = "kotlinx-serialization" }
kotlinx-coroutines-core = { module = "org.jetbrains.kotlinx:kotlinx-coroutines-core", version.ref = "kotlinx-coroutines" }
kotlinx-coroutines-test = { module = "org.jetbrains.kotlinx:kotlinx-coroutines-test", version.ref = "kotlinx-coroutines" }
kotlinx-datetime = { module = "org.jetbrains.kotlinx:kotlinx-datetime", version.ref = "kotlinx-datetime" }
coil-compose = { module = "io.coil-kt.coil3:coil-compose", version.ref = "coil" }
coil-network-ktor = { module = "io.coil-kt.coil3:coil-network-ktor3", version.ref = "coil" }

[plugins]
kotlin-multiplatform = { id = "org.jetbrains.kotlin.multiplatform", version.ref = "kotlin" }
kotlin-serialization = { id = "org.jetbrains.kotlin.plugin.serialization", version.ref = "kotlin" }
compose-multiplatform = { id = "org.jetbrains.compose", version.ref = "compose-multiplatform" }
compose-compiler = { id = "org.jetbrains.kotlin.plugin.compose", version.ref = "kotlin" }
android-application = { id = "com.android.application", version.ref = "agp" }
android-library = { id = "com.android.library", version.ref = "agp" }
sqldelight = { id = "app.cash.sqldelight", version.ref = "sqldelight" }
```

**Convention:** Always use version catalogs. Never declare dependency versions inline in `build.gradle.kts` files. All version bumps happen in one file (`libs.versions.toml`).

### Kotlin 2.2+ Language Features

Kotlin 2.2 ships with K2 compiler by default and introduces:

- **Context parameters** (preview) — replace context receivers with a stable design
- **Explicit backing fields** — custom getter/setter with a distinct backing field type
- **Guard conditions in when** — `when` branches with `if` guards
- **Non-local break/continue** — break/continue from enclosing loops inside lambdas
- **Multi-dollar string interpolation** — configurable `$` prefix for raw strings
- **UUID** — `kotlin.uuid.Uuid` in the stdlib

Use these features where they improve clarity:

```kotlin
// Context parameters (preview, opt-in with -Xcontext-parameters)
context(logger: Logger, auth: AuthContext)
fun fetchUserProfile(userId: String): UserProfile {
    logger.info("Fetching profile for $userId")
    require(auth.hasPermission("read:profiles"))
    // ...
}

// Guard conditions in when
fun classify(response: HttpResponse) = when (response.status) {
    HttpStatusCode.OK if response.body.isNotEmpty() -> Result.Success
    HttpStatusCode.OK -> Result.Empty
    HttpStatusCode.NotFound -> Result.NotFound
    else -> Result.Error
}

// Explicit backing fields
class UserPreferences {
    val theme: Theme
        field = Theme.System  // backing field has specific default
        get() = if (field == Theme.System) detectSystemTheme() else field
}

// UUID in stdlib
val sessionId: Uuid = Uuid.random()
```

### K2 Compiler

K2 is the default compiler in Kotlin 2.2+. Key implications:

- **2x faster compilation** for clean builds, even more for incremental
- **Unified frontend** — same analysis for JVM, JS, Native, and WASM targets
- **Better type inference** — smart casts work across more control flow patterns
- **Improved multiplatform** — expect/actual matching is stricter and more correct
- No action needed — K2 is the default. If a library does not compile with K2, file a bug on that library; do not downgrade the compiler.

---

## 2. Project Structure

### KMP Project Layout

Every KMP project uses the standard three-module structure with Compose Multiplatform:

```
project-root/
├── gradle/
│   ├── libs.versions.toml          # Version catalog (single source of truth)
│   └── wrapper/
│       └── gradle-wrapper.properties
├── build.gradle.kts                # Root build file — plugins only
├── settings.gradle.kts             # Module declarations, repository config
├── shared/                         # KMP shared module
│   ├── build.gradle.kts            # Multiplatform plugin config
│   └── src/
│       ├── commonMain/             # Shared code — business logic, UI, data
│       │   └── kotlin/
│       │       └── com/example/app/
│       │           ├── App.kt                  # Root composable
│       │           ├── di/                     # Koin modules
│       │           │   ├── AppModule.kt
│       │           │   ├── NetworkModule.kt
│       │           │   └── DatabaseModule.kt
│       │           ├── data/                   # Data layer
│       │           │   ├── remote/             # Ktor API clients
│       │           │   │   ├── ApiClient.kt
│       │           │   │   ├── dto/            # Data transfer objects
│       │           │   │   └── interceptor/    # Auth, logging interceptors
│       │           │   ├── local/              # SQLDelight DAOs
│       │           │   │   ├── Database.kt
│       │           │   │   └── dao/
│       │           │   └── repository/         # Repository implementations
│       │           │       ├── UserRepositoryImpl.kt
│       │           │       └── PostRepositoryImpl.kt
│       │           ├── domain/                 # Domain layer (pure Kotlin)
│       │           │   ├── model/              # Domain models
│       │           │   ├── repository/         # Repository interfaces
│       │           │   └── usecase/            # Use cases
│       │           ├── ui/                     # Compose Multiplatform screens
│       │           │   ├── navigation/         # Navigation graph
│       │           │   │   └── AppNavigation.kt
│       │           │   ├── theme/              # Material 3 theme
│       │           │   │   ├── Theme.kt
│       │           │   │   ├── Color.kt
│       │           │   │   └── Type.kt
│       │           │   ├── component/          # Reusable composables
│       │           │   └── screen/             # Feature screens
│       │           │       ├── home/
│       │           │       │   ├── HomeScreen.kt
│       │           │       │   └── HomeViewModel.kt
│       │           │       ├── auth/
│       │           │       └── profile/
│       │           └── platform/               # expect declarations
│       │               └── Platform.kt
│       ├── commonTest/             # Shared tests
│       │   └── kotlin/
│       │       └── com/example/app/
│       │           ├── data/
│       │           ├── domain/
│       │           └── ui/
│       ├── androidMain/            # Android-specific implementations
│       │   └── kotlin/
│       │       └── com/example/app/
│       │           └── platform/
│       │               └── Platform.android.kt
│       ├── androidUnitTest/        # Android-specific tests
│       ├── iosMain/                # iOS-specific implementations
│       │   └── kotlin/
│       │       └── com/example/app/
│       │           └── platform/
│       │               └── Platform.ios.kt
│       └── iosTest/                # iOS-specific tests
├── androidApp/                     # Android application module
│   ├── build.gradle.kts
│   └── src/main/
│       ├── AndroidManifest.xml
│       └── kotlin/
│           └── com/example/app/
│               └── MainActivity.kt
├── iosApp/                         # iOS application (Xcode project)
│   ├── iosApp.xcodeproj/
│   ├── iosApp/
│   │   ├── AppDelegate.swift       # or use SwiftUI App lifecycle
│   │   ├── ContentView.swift       # Hosts ComposeUIViewController
│   │   └── Info.plist
│   └── Podfile                     # If using CocoaPods (alternative: SPM)
└── sqldelight/                     # SQLDelight schema files
    └── com/example/app/
        └── AppDatabase.sq
```

### Module Dependency Rules

```
androidApp ──depends-on──→ shared
iosApp ────uses-via-framework──→ shared (exported as XCFramework)

shared/commonMain ──NO dependency on──→ androidMain or iosMain
shared/androidMain ──depends-on──→ commonMain (automatic)
shared/iosMain ──depends-on──→ commonMain (automatic)
```

**Convention:** The `shared` module exports everything the platform apps need. Platform app modules (`androidApp`, `iosApp`) are thin shells — they host the Compose entry point and provide platform-specific configuration (permissions, entitlements, manifest entries). Business logic, UI, networking, and persistence all live in `shared/commonMain`.

### Build Configuration — shared module

```kotlin
// shared/build.gradle.kts
plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.compose.multiplatform)
    alias(libs.plugins.compose.compiler)
    alias(libs.plugins.android.library)
    alias(libs.plugins.sqldelight)
}

kotlin {
    // Target declarations
    androidTarget {
        compilations.all {
            compilerOptions {
                jvmTarget.set(JvmTarget.JVM_21)
            }
        }
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach { target ->
        target.binaries.framework {
            baseName = "shared"
            isStatic = true  // Static framework for Compose Multiplatform
        }
    }

    sourceSets {
        commonMain.dependencies {
            // Compose Multiplatform
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material3)
            implementation(compose.ui)
            implementation(compose.components.resources)

            // Networking
            implementation(libs.ktor.client.core)
            implementation(libs.ktor.client.content.negotiation)
            implementation(libs.ktor.serialization.json)

            // Persistence
            implementation(libs.sqldelight.runtime)
            implementation(libs.sqldelight.coroutines)

            // DI
            implementation(libs.koin.core)
            implementation(libs.koin.compose)
            implementation(libs.koin.compose.viewmodel)

            // Serialization & Coroutines
            implementation(libs.kotlinx.serialization.json)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.kotlinx.datetime)

            // Image loading
            implementation(libs.coil.compose)
            implementation(libs.coil.network.ktor)
        }

        commonTest.dependencies {
            implementation(kotlin("test"))
            implementation(libs.kotlinx.coroutines.test)
        }

        androidMain.dependencies {
            implementation(libs.ktor.client.okhttp)
            implementation(libs.sqldelight.android.driver)
        }

        iosMain.dependencies {
            implementation(libs.ktor.client.darwin)
            implementation(libs.sqldelight.native.driver)
        }
    }
}

android {
    namespace = "com.example.app.shared"
    compileSdk = 35

    defaultConfig {
        minSdk = 24
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

sqldelight {
    databases {
        create("AppDatabase") {
            packageName.set("com.example.app.db")
        }
    }
}
```

### Source Set Hierarchy

KMP uses a hierarchical source set structure. Understanding the hierarchy prevents duplication:

```
                    commonMain
                   /          \
             androidMain     iosMain
                            /   |   \
                   iosX64Main iosArm64Main iosSimulatorArm64Main
```

The `iosMain` source set is an intermediate that covers all iOS targets. Code placed here is shared across `iosX64`, `iosArm64`, and `iosSimulatorArm64`. Never put iOS-specific code in individual target source sets unless it truly differs per architecture (extremely rare).

```kotlin
// In shared/build.gradle.kts — create intermediate source sets
kotlin {
    sourceSets {
        // iosMain already exists as an intermediate
        // Kotlin Gradle plugin creates it automatically for grouped targets
    }
}
```

---

## 3. Expect/Actual Pattern

### The Mechanism

`expect`/`actual` is KMP's mechanism for platform-specific implementations behind a common API. The shared module declares `expect` signatures in `commonMain`; each platform provides `actual` implementations.

### Rules

1. **Every `expect` must have an `actual` in every target.** Missing actuals are compile errors.
2. **The `actual` must match the `expect` signature exactly.** Same visibility, same type parameters, same return type.
3. **Keep `expect` surface area minimal.** Most code should be pure common Kotlin with no expect/actual. Only platform APIs (file system, crypto, biometrics, notifications) require expect/actual.
4. **Prefer interfaces + DI over expect/actual for testability.** An interface injected via Koin is easier to mock than an `actual` function.
5. **Use `expect`/`actual` for types and functions, not for business logic.** The actual should be a thin wrapper around platform SDK calls.

### Expect/Actual for Objects and Classes

```kotlin
// commonMain — expect declaration
expect class PlatformContext

expect fun getPlatformName(): String

expect class SecureStorage(context: PlatformContext) {
    fun getString(key: String): String?
    fun putString(key: String, value: String)
    fun remove(key: String)
    fun clear()
}

// androidMain — actual implementation
actual typealias PlatformContext = android.content.Context

actual fun getPlatformName(): String = "Android ${Build.VERSION.SDK_INT}"

actual class SecureStorage actual constructor(private val context: PlatformContext) {
    private val prefs = EncryptedSharedPreferences.create(
        context,
        "secure_prefs",
        MasterKey.Builder(context).setKeyScheme(MasterKey.KeyScheme.AES256_GCM).build(),
        EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
        EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
    )

    actual fun getString(key: String): String? = prefs.getString(key, null)
    actual fun putString(key: String, value: String) { prefs.edit().putString(key, value).apply() }
    actual fun remove(key: String) { prefs.edit().remove(key).apply() }
    actual fun clear() { prefs.edit().clear().apply() }
}

// iosMain — actual implementation
actual class PlatformContext  // No-op on iOS, no application context needed

actual fun getPlatformName(): String = UIDevice.currentDevice.run {
    "$systemName $systemVersion"
}

actual class SecureStorage actual constructor(context: PlatformContext) {
    actual fun getString(key: String): String? {
        val query = keychainQuery(key) + mapOf(kSecReturnData to kCFBooleanTrue, kSecMatchLimit to kSecMatchLimitOne)
        val result = CFBridgingRelease(SecItemCopyMatching(query.toCFDictionary(), null))
        return (result as? NSData)?.toKString()
    }

    actual fun putString(key: String, value: String) {
        remove(key)
        val query = keychainQuery(key) + mapOf(kSecValueData to value.toNSData())
        SecItemAdd(query.toCFDictionary(), null)
    }

    actual fun remove(key: String) {
        SecItemDelete(keychainQuery(key).toCFDictionary())
    }

    actual fun clear() {
        val query = mapOf(kSecClass to kSecClassGenericPassword)
        SecItemDelete(query.toCFDictionary())
    }

    private fun keychainQuery(key: String) = mapOf(
        kSecClass to kSecClassGenericPassword,
        kSecAttrAccount to key,
        kSecAttrService to "com.example.app"
    )
}
```

### Expect/Actual for Database Drivers

```kotlin
// commonMain
expect class DatabaseDriverFactory(context: PlatformContext) {
    fun create(): SqlDriver
}

// androidMain
actual class DatabaseDriverFactory actual constructor(private val context: PlatformContext) {
    actual fun create(): SqlDriver {
        return AndroidSqliteDriver(AppDatabase.Schema, context, "app.db")
    }
}

// iosMain
actual class DatabaseDriverFactory actual constructor(context: PlatformContext) {
    actual fun create(): SqlDriver {
        return NativeSqliteDriver(AppDatabase.Schema, "app.db")
    }
}
```

### When to Use expect/actual vs. Interface + DI

| Use Case | Approach | Rationale |
|---|---|---|
| Platform name, OS version | `expect fun` | Trivial, no testing benefit from interface |
| Database driver creation | `expect class` | Platform SDK constructor, thin wrapper |
| HTTP engine selection | DI module per platform | Ktor engines are already separate deps |
| Analytics provider | Interface + DI | Need to mock in tests, swap implementations |
| File system access | Interface + DI | Complex operations, need test doubles |
| Biometric auth | Interface + DI | UI interaction, need to stub in tests |
| Crypto operations | `expect fun` for primitives, interface for complex | Small primitives are fine as expect/actual |

---

## 4. Compose Multiplatform UI

### Architecture

Compose Multiplatform allows writing a single UI codebase in `commonMain` that renders natively on Android, iOS, Desktop, and Web. Since Compose Multiplatform 1.8+, it achieves near-complete API parity with Jetpack Compose.

### Root Composable

```kotlin
// shared/src/commonMain/kotlin/com/example/app/App.kt
@Composable
fun App() {
    val koinApplication = KoinApplication.init {
        modules(appModule, networkModule, databaseModule)
    }

    KoinContext(koinApplication) {
        AppTheme {
            Surface(
                modifier = Modifier.fillMaxSize(),
                color = MaterialTheme.colorScheme.background
            ) {
                AppNavigation()
            }
        }
    }
}
```

### Android Host

```kotlin
// androidApp/src/main/kotlin/.../MainActivity.kt
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            App()
        }
    }
}
```

### iOS Host

```swift
// iosApp/iosApp/ContentView.swift
import SwiftUI
import shared

struct ComposeView: UIViewControllerRepresentable {
    func makeUIViewController(context: Context) -> UIViewController {
        MainViewControllerKt.MainViewController()
    }

    func updateUIViewController(_ uiViewController: UIViewController, context: Context) {}
}

struct ContentView: View {
    var body: some View {
        ComposeView()
            .ignoresSafeArea(.all)
    }
}
```

```kotlin
// shared/src/iosMain/kotlin/.../MainViewController.kt
fun MainViewController(): UIViewController {
    return ComposeUIViewController {
        App()
    }
}
```

### Material 3 Theming

```kotlin
// shared/src/commonMain/kotlin/.../ui/theme/Theme.kt
private val LightColorScheme = lightColorScheme(
    primary = Color(0xFF1B6B4A),
    onPrimary = Color.White,
    primaryContainer = Color(0xFFA3F2C6),
    onPrimaryContainer = Color(0xFF002112),
    secondary = Color(0xFF4E6355),
    onSecondary = Color.White,
    background = Color(0xFFFBFDF8),
    surface = Color(0xFFFBFDF8),
    error = Color(0xFFBA1A1A),
)

private val DarkColorScheme = darkColorScheme(
    primary = Color(0xFF88D6AB),
    onPrimary = Color(0xFF003822),
    primaryContainer = Color(0xFF005234),
    onPrimaryContainer = Color(0xFFA3F2C6),
    secondary = Color(0xFFB5CCB9),
    onSecondary = Color(0xFF213528),
    background = Color(0xFF191C1A),
    surface = Color(0xFF191C1A),
    error = Color(0xFFFFB4AB),
)

@Composable
fun AppTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = if (darkTheme) DarkColorScheme else LightColorScheme

    MaterialTheme(
        colorScheme = colorScheme,
        typography = AppTypography,
        content = content
    )
}
```

### Screen Pattern

Every screen follows the same structure:

```kotlin
// shared/src/commonMain/kotlin/.../ui/screen/home/HomeScreen.kt
@Composable
fun HomeScreen(
    viewModel: HomeViewModel = koinViewModel(),
    onNavigateToDetail: (String) -> Unit = {},
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    HomeScreenContent(
        uiState = uiState,
        onRefresh = viewModel::refresh,
        onItemClick = onNavigateToDetail,
    )
}

@Composable
private fun HomeScreenContent(
    uiState: HomeUiState,
    onRefresh: () -> Unit,
    onItemClick: (String) -> Unit,
) {
    when (uiState) {
        is HomeUiState.Loading -> {
            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                CircularProgressIndicator()
            }
        }
        is HomeUiState.Success -> {
            LazyColumn(
                modifier = Modifier.fillMaxSize(),
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(uiState.items, key = { it.id }) { item ->
                    ItemCard(
                        item = item,
                        onClick = { onItemClick(item.id) }
                    )
                }
            }
        }
        is HomeUiState.Error -> {
            ErrorState(
                message = uiState.message,
                onRetry = onRefresh
            )
        }
    }
}
```

**Convention:** Always split screens into a stateful wrapper (`HomeScreen`) and a stateless content composable (`HomeScreenContent`). The stateful wrapper collects ViewModel state and passes lambdas. The stateless composable receives only data and callbacks — this makes it previewable and testable.

### ViewModel Pattern

```kotlin
// shared/src/commonMain/kotlin/.../ui/screen/home/HomeViewModel.kt
class HomeViewModel(
    private val getPostsUseCase: GetPostsUseCase,
) : ViewModel() {

    private val _uiState = MutableStateFlow<HomeUiState>(HomeUiState.Loading)
    val uiState: StateFlow<HomeUiState> = _uiState.asStateFlow()

    init {
        refresh()
    }

    fun refresh() {
        viewModelScope.launch {
            _uiState.value = HomeUiState.Loading
            getPostsUseCase()
                .onSuccess { posts ->
                    _uiState.value = HomeUiState.Success(posts)
                }
                .onFailure { error ->
                    _uiState.value = HomeUiState.Error(error.message ?: "Unknown error")
                }
        }
    }
}

sealed interface HomeUiState {
    data object Loading : HomeUiState
    data class Success(val items: List<Post>) : HomeUiState
    data class Error(val message: String) : HomeUiState
}
```

**Convention:** UI state is always a sealed interface. Every screen has exactly one `UiState` type with `Loading`, `Success`, and `Error` variants at minimum. ViewModel exposes `StateFlow<UiState>`, never `MutableStateFlow`.

### Navigation

```kotlin
// shared/src/commonMain/kotlin/.../ui/navigation/AppNavigation.kt
@Serializable
sealed interface Route {
    @Serializable data object Home : Route
    @Serializable data class Detail(val id: String) : Route
    @Serializable data object Profile : Route
    @Serializable data object Settings : Route
}

@Composable
fun AppNavigation() {
    val navController = rememberNavController()

    NavHost(navController = navController, startDestination = Route.Home) {
        composable<Route.Home> {
            HomeScreen(
                onNavigateToDetail = { id ->
                    navController.navigate(Route.Detail(id))
                }
            )
        }

        composable<Route.Detail> { backStackEntry ->
            val detail: Route.Detail = backStackEntry.toRoute()
            DetailScreen(
                itemId = detail.id,
                onNavigateBack = { navController.popBackStack() }
            )
        }

        composable<Route.Profile> {
            ProfileScreen()
        }

        composable<Route.Settings> {
            SettingsScreen()
        }
    }
}
```

**Convention:** Routes are `@Serializable` data objects/classes in a sealed interface. Type-safe navigation (Jetpack Navigation 2.9+) eliminates string-based route matching. Never pass complex objects through navigation — pass IDs and let the destination screen load the data.

### Compose Resources

Compose Multiplatform has its own resource system for strings, images, fonts, and files:

```
shared/src/commonMain/composeResources/
├── drawable/          # Images (PNG, SVG, WebP)
│   ├── ic_logo.xml
│   └── bg_header.webp
├── values/            # Strings, colors
│   └── strings.xml
├── values-es/         # Localized strings
│   └── strings.xml
└── font/              # Custom fonts
    ├── Inter-Regular.ttf
    └── Inter-Bold.ttf
```

```kotlin
// Usage in composables
@Composable
fun LogoImage() {
    Image(
        painter = painterResource(Res.drawable.ic_logo),
        contentDescription = stringResource(Res.string.app_name)
    )
}
```

**Convention:** Always use Compose resources for anything that needs localization or platform-appropriate resolution. Never hardcode strings in composables. Never put images in platform-specific asset directories when they can go in shared resources.

---

## 5. Networking — Ktor Client

### Architecture

Ktor is a first-class multiplatform HTTP client. The core API lives in `commonMain`; platform-specific engines (OkHttp for Android, Darwin/URLSession for iOS) are injected per-platform via DI.

### Client Configuration

```kotlin
// shared/src/commonMain/kotlin/.../data/remote/ApiClient.kt
class ApiClient(engine: HttpClientEngine, private val tokenProvider: TokenProvider) {

    val httpClient = HttpClient(engine) {
        install(ContentNegotiation) {
            json(Json {
                ignoreUnknownKeys = true
                isLenient = false
                encodeDefaults = true
                prettyPrint = false
                coerceInputValues = true
            })
        }

        install(HttpTimeout) {
            requestTimeoutMillis = 30_000
            connectTimeoutMillis = 10_000
            socketTimeoutMillis = 30_000
        }

        install(Logging) {
            logger = Logger.DEFAULT
            level = LogLevel.HEADERS  // Use LogLevel.NONE in production
        }

        install(Auth) {
            bearer {
                loadTokens {
                    tokenProvider.getTokens()?.let { tokens ->
                        BearerTokens(tokens.accessToken, tokens.refreshToken)
                    }
                }
                refreshTokens {
                    val refreshResult = tokenProvider.refreshTokens(oldTokens?.refreshToken ?: "")
                    refreshResult?.let { tokens ->
                        BearerTokens(tokens.accessToken, tokens.refreshToken)
                    }
                }
            }
        }

        defaultRequest {
            url("https://api.example.com/v1/")
            header("Accept", "application/json")
        }

        HttpResponseValidator {
            handleResponseExceptionWithRequest { cause, _ ->
                throw mapException(cause)
            }
        }
    }

    private fun mapException(cause: Throwable): AppException = when (cause) {
        is ClientRequestException -> when (cause.response.status) {
            HttpStatusCode.Unauthorized -> AppException.Unauthorized
            HttpStatusCode.Forbidden -> AppException.Forbidden
            HttpStatusCode.NotFound -> AppException.NotFound
            else -> AppException.Server(cause.response.status.value, cause.message)
        }
        is ServerResponseException -> AppException.Server(cause.response.status.value, cause.message)
        is IOException -> AppException.Network(cause.message ?: "Network error")
        else -> AppException.Unknown(cause.message ?: "Unknown error")
    }
}
```

### API Service Pattern

```kotlin
// shared/src/commonMain/kotlin/.../data/remote/PostApiService.kt
class PostApiService(private val apiClient: ApiClient) {

    suspend fun getPosts(page: Int = 1, limit: Int = 20): List<PostDto> {
        return apiClient.httpClient.get("posts") {
            parameter("page", page)
            parameter("limit", limit)
        }.body()
    }

    suspend fun getPost(id: String): PostDto {
        return apiClient.httpClient.get("posts/$id").body()
    }

    suspend fun createPost(request: CreatePostRequest): PostDto {
        return apiClient.httpClient.post("posts") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }.body()
    }

    suspend fun deletePost(id: String) {
        apiClient.httpClient.delete("posts/$id")
    }
}
```

### DTOs with Kotlinx Serialization

```kotlin
// shared/src/commonMain/kotlin/.../data/remote/dto/PostDto.kt
@Serializable
data class PostDto(
    val id: String,
    val title: String,
    val body: String,
    @SerialName("author_id")
    val authorId: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String?,
    val tags: List<String> = emptyList(),
)

@Serializable
data class CreatePostRequest(
    val title: String,
    val body: String,
    val tags: List<String> = emptyList(),
)

// Mapper — DTO to domain model (always explicit, never use DTO as domain model)
fun PostDto.toDomain(): Post = Post(
    id = id,
    title = title,
    body = body,
    authorId = authorId,
    createdAt = Instant.parse(createdAt),
    updatedAt = updatedAt?.let { Instant.parse(it) },
    tags = tags,
)
```

**Convention:** DTOs are `@Serializable` data classes that mirror the API response shape. Domain models are separate classes in `domain/model/`. Always map between them explicitly. Never use DTOs in the UI layer. Never use domain models as serialization targets.

### Engine Injection via Koin

```kotlin
// shared/src/commonMain/kotlin/.../di/NetworkModule.kt
val networkModule = module {
    single { ApiClient(engine = get(), tokenProvider = get()) }
    single { PostApiService(apiClient = get()) }
    single<TokenProvider> { TokenProviderImpl(secureStorage = get()) }
}

// shared/src/androidMain/kotlin/.../di/PlatformModule.android.kt
actual val platformModule = module {
    single<HttpClientEngine> { OkHttp.create() }
}

// shared/src/iosMain/kotlin/.../di/PlatformModule.ios.kt
actual val platformModule = module {
    single<HttpClientEngine> { Darwin.create() }
}
```

### Error Handling

```kotlin
// shared/src/commonMain/kotlin/.../domain/model/AppException.kt
sealed class AppException(message: String, cause: Throwable? = null) : Exception(message, cause) {
    data object Unauthorized : AppException("Unauthorized — token expired or invalid")
    data object Forbidden : AppException("Forbidden — insufficient permissions")
    data object NotFound : AppException("Resource not found")
    data class Server(val code: Int, override val message: String) : AppException(message)
    data class Network(override val message: String) : AppException(message)
    data class Unknown(override val message: String) : AppException(message)
}

// Repository wraps API calls in Result
class PostRepositoryImpl(
    private val apiService: PostApiService,
    private val postDao: PostDao,
) : PostRepository {

    override suspend fun getPosts(): Result<List<Post>> = runCatching {
        val remote = apiService.getPosts()
        val posts = remote.map { it.toDomain() }
        postDao.insertAll(posts)  // Cache locally
        posts
    }
}
```

**Convention:** Repositories return `Result<T>`. Use cases return `Result<T>`. ViewModels handle `Result` by mapping success/failure to `UiState`. Never let exceptions propagate to the UI layer unhandled.

---

## 6. Dependency Injection — Koin

### Why Koin

Koin is multiplatform-native, has first-class Compose support, requires no code generation or annotation processing, and has simpler syntax than Kodein. For KMP projects, it is the default choice.

### Module Organization

```kotlin
// shared/src/commonMain/kotlin/.../di/AppModule.kt
val appModule = module {
    // Use cases
    factory { GetPostsUseCase(postRepository = get()) }
    factory { GetPostDetailUseCase(postRepository = get()) }
    factory { CreatePostUseCase(postRepository = get()) }
    factory { LoginUseCase(authRepository = get()) }

    // ViewModels
    viewModel { HomeViewModel(getPostsUseCase = get()) }
    viewModel { params -> DetailViewModel(postId = params.get(), getPostDetailUseCase = get()) }
    viewModel { ProfileViewModel(authRepository = get()) }

    // Repositories
    single<PostRepository> { PostRepositoryImpl(apiService = get(), postDao = get()) }
    single<AuthRepository> { AuthRepositoryImpl(apiService = get(), tokenProvider = get()) }
}

val networkModule = module {
    single { ApiClient(engine = get(), tokenProvider = get()) }
    single { PostApiService(apiClient = get()) }
    single { AuthApiService(apiClient = get()) }
    single<TokenProvider> { TokenProviderImpl(secureStorage = get()) }
}

val databaseModule = module {
    single { DatabaseDriverFactory(context = get()).create() }
    single { AppDatabase(driver = get()) }
    single { PostDao(database = get()) }
    single { UserDao(database = get()) }
}

// Platform-specific module (expect/actual)
expect val platformModule: Module
```

### Koin with Compose

```kotlin
// In App.kt root composable
@Composable
fun App(platformContext: PlatformContext) {
    KoinApplication(application = {
        modules(platformModule, appModule, networkModule, databaseModule)
    }) {
        AppTheme {
            AppNavigation()
        }
    }
}

// In screen composables — inject ViewModel
@Composable
fun HomeScreen(
    viewModel: HomeViewModel = koinViewModel(),
) {
    // viewModel is scoped to the navigation destination lifecycle
}

// ViewModel with parameters
@Composable
fun DetailScreen(itemId: String) {
    val viewModel: DetailViewModel = koinViewModel { parametersOf(itemId) }
    // ...
}
```

### Scoping Rules

| Scope | Use For | Koin DSL |
|---|---|---|
| `single` | Singletons: database, HTTP client, API services | `single { ApiClient(...) }` |
| `factory` | New instance per injection: use cases, mappers | `factory { GetPostsUseCase(...) }` |
| `viewModel` | Compose ViewModel lifecycle | `viewModel { HomeViewModel(...) }` |
| `scope` | Feature-scoped: auth session, wizard flow state | `scope<AuthScope> { scoped { ... } }` |

**Convention:** Repositories, API services, and database instances are `single`. Use cases are `factory` (stateless, no reason to share). ViewModels use `viewModel` DSL for proper lifecycle integration.

### Kodein Alternative

If Kodein is preferred (some teams choose it for its more explicit binding syntax):

```kotlin
// Kodein equivalent
val appDI = DI {
    bindProvider { GetPostsUseCase(instance()) }
    bindSingleton { PostRepositoryImpl(instance(), instance()) }
    bindFactory { params: String -> DetailViewModel(params, instance()) }
}
```

The patterns in this document use Koin. If a project uses Kodein, the architectural patterns (module organization, scoping, lifecycle management) apply identically — only the binding DSL differs.

---

## 7. Persistence — SQLDelight

### Schema Definition

SQLDelight generates type-safe Kotlin APIs from `.sq` files containing raw SQL:

```sql
-- shared/src/commonMain/sqldelight/com/example/app/db/Post.sq

CREATE TABLE post (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    is_bookmarked INTEGER AS Boolean NOT NULL DEFAULT 0
);

CREATE INDEX post_author_id ON post(author_id);
CREATE INDEX post_created_at ON post(created_at);

-- Named queries generate type-safe Kotlin functions

selectAll:
SELECT * FROM post
ORDER BY created_at DESC;

selectById:
SELECT * FROM post
WHERE id = :id;

selectByAuthor:
SELECT * FROM post
WHERE author_id = :authorId
ORDER BY created_at DESC;

selectBookmarked:
SELECT * FROM post
WHERE is_bookmarked = 1
ORDER BY created_at DESC;

insert:
INSERT OR REPLACE INTO post(id, title, body, author_id, created_at, updated_at, is_bookmarked)
VALUES (?, ?, ?, ?, ?, ?, ?);

deleteById:
DELETE FROM post
WHERE id = :id;

deleteAll:
DELETE FROM post;

countAll:
SELECT COUNT(*) FROM post;
```

### DAO Pattern

```kotlin
// shared/src/commonMain/kotlin/.../data/local/dao/PostDao.kt
class PostDao(private val database: AppDatabase) {

    private val queries = database.postQueries

    fun getAll(): Flow<List<Post>> {
        return queries.selectAll()
            .asFlow()
            .mapToList(Dispatchers.IO)
            .map { rows -> rows.map { it.toDomain() } }
    }

    fun getById(id: String): Flow<Post?> {
        return queries.selectById(id)
            .asFlow()
            .mapToOneOrNull(Dispatchers.IO)
            .map { it?.toDomain() }
    }

    suspend fun insert(post: Post) {
        queries.insert(
            id = post.id,
            title = post.title,
            body = post.body,
            author_id = post.authorId,
            created_at = post.createdAt.toString(),
            updated_at = post.updatedAt?.toString(),
            is_bookmarked = post.isBookmarked,
        )
    }

    suspend fun insertAll(posts: List<Post>) {
        database.transaction {
            posts.forEach { post -> insert(post) }
        }
    }

    suspend fun deleteById(id: String) {
        queries.deleteById(id)
    }

    suspend fun deleteAll() {
        queries.deleteAll()
    }
}
```

### Migrations

SQLDelight handles migrations through numbered `.sqm` files:

```sql
-- shared/src/commonMain/sqldelight/migrations/1.sqm
ALTER TABLE post ADD COLUMN tags TEXT;

-- shared/src/commonMain/sqldelight/migrations/2.sqm
CREATE TABLE user_preference (
    key TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
);
```

**Convention:** Never modify a committed `.sq` file's `CREATE TABLE` statement. Write a `.sqm` migration file instead. SQLDelight generates migration verification tests automatically.

### Offline-First Pattern

```kotlin
// Repository with offline-first strategy
class PostRepositoryImpl(
    private val apiService: PostApiService,
    private val postDao: PostDao,
) : PostRepository {

    override fun getPosts(): Flow<List<Post>> {
        return postDao.getAll()  // Always return cached data as a Flow
    }

    override suspend fun refreshPosts(): Result<Unit> = runCatching {
        val remote = apiService.getPosts()
        val posts = remote.map { it.toDomain() }
        postDao.deleteAll()
        postDao.insertAll(posts)
    }

    override suspend fun getPost(id: String): Flow<Post?> {
        // Trigger a refresh in the background, return cache immediately
        viewModelScope.launch {
            runCatching {
                val remote = apiService.getPost(id)
                postDao.insert(remote.toDomain())
            }
        }
        return postDao.getById(id)
    }
}
```

---

## 8. Testing Patterns

### Test Pyramid (KMP-specific)

```
        /\
       /  \          E2E (Maestro / XCUITest / Espresso)
      /    \         Full device, real network
     /------\
    /        \        Integration Tests (commonTest)
   /          \       Repository + API + DB through real implementations
  /------------\
 /              \      Unit Tests (commonTest)
/                \     ViewModels, use cases, mappers — pure functions
/------------------\
```

### Test Source Sets

```
shared/src/
├── commonTest/         # Shared tests — run on all platforms
│   └── kotlin/
│       └── com/example/app/
│           ├── data/
│           │   ├── repository/
│           │   │   └── PostRepositoryTest.kt
│           │   └── remote/
│           │       └── PostApiServiceTest.kt
│           ├── domain/
│           │   └── usecase/
│           │       └── GetPostsUseCaseTest.kt
│           └── ui/
│               └── screen/
│                   └── HomeViewModelTest.kt
├── androidUnitTest/    # Android-specific tests
│   └── kotlin/
│       └── com/example/app/
│           └── platform/
│               └── SecureStorageAndroidTest.kt
└── iosTest/            # iOS-specific tests
    └── kotlin/
        └── com/example/app/
            └── platform/
                └── SecureStorageIosTest.kt
```

### Kotlin Test (commonTest)

```kotlin
// shared/src/commonTest/kotlin/.../domain/usecase/GetPostsUseCaseTest.kt
class GetPostsUseCaseTest {

    private val fakeRepository = FakePostRepository()
    private val useCase = GetPostsUseCase(fakeRepository)

    @Test
    fun `returns posts from repository`() = runTest {
        val expected = listOf(
            Post(id = "1", title = "First", body = "Body 1", authorId = "a1",
                 createdAt = Clock.System.now(), updatedAt = null, tags = emptyList()),
            Post(id = "2", title = "Second", body = "Body 2", authorId = "a2",
                 createdAt = Clock.System.now(), updatedAt = null, tags = emptyList()),
        )
        fakeRepository.setPosts(expected)

        val result = useCase()

        assertTrue(result.isSuccess)
        assertEquals(expected, result.getOrNull())
    }

    @Test
    fun `returns failure when repository fails`() = runTest {
        fakeRepository.setShouldFail(true)

        val result = useCase()

        assertTrue(result.isFailure)
    }
}
```

### ViewModel Testing

```kotlin
// shared/src/commonTest/kotlin/.../ui/screen/HomeViewModelTest.kt
class HomeViewModelTest {

    @Test
    fun `initial state is Loading then transitions to Success`() = runTest {
        val fakeRepository = FakePostRepository()
        val posts = listOf(testPost())
        fakeRepository.setPosts(posts)

        val viewModel = HomeViewModel(GetPostsUseCase(fakeRepository))

        val states = mutableListOf<HomeUiState>()
        val job = launch(UnconfinedTestDispatcher(testScheduler)) {
            viewModel.uiState.toList(states)
        }

        // Assert Loading was emitted first, then Success
        assertTrue(states.any { it is HomeUiState.Loading })
        val success = states.filterIsInstance<HomeUiState.Success>().first()
        assertEquals(posts, success.items)

        job.cancel()
    }

    @Test
    fun `refresh after error transitions back to Loading then Success`() = runTest {
        val fakeRepository = FakePostRepository()
        fakeRepository.setShouldFail(true)

        val viewModel = HomeViewModel(GetPostsUseCase(fakeRepository))

        // Wait for error state
        advanceUntilIdle()
        assertTrue(viewModel.uiState.value is HomeUiState.Error)

        // Fix the repository and refresh
        fakeRepository.setShouldFail(false)
        fakeRepository.setPosts(listOf(testPost()))
        viewModel.refresh()
        advanceUntilIdle()

        assertTrue(viewModel.uiState.value is HomeUiState.Success)
    }
}
```

### Fake vs. Mock Pattern

```kotlin
// Prefer fakes over mocks for KMP (mocking libraries have limited multiplatform support)

// Fake repository (in commonTest)
class FakePostRepository : PostRepository {
    private var posts: List<Post> = emptyList()
    private var shouldFail = false

    fun setPosts(posts: List<Post>) { this.posts = posts }
    fun setShouldFail(fail: Boolean) { shouldFail = fail }

    override suspend fun getPosts(): Result<List<Post>> {
        return if (shouldFail) Result.failure(AppException.Network("Fake error"))
        else Result.success(posts)
    }

    override suspend fun getPost(id: String): Result<Post> {
        return if (shouldFail) Result.failure(AppException.Network("Fake error"))
        else posts.find { it.id == id }?.let { Result.success(it) }
            ?: Result.failure(AppException.NotFound)
    }

    override suspend fun createPost(post: Post): Result<Post> {
        return if (shouldFail) Result.failure(AppException.Network("Fake error"))
        else Result.success(post)
    }
}

// Fake API service for repository tests
class FakePostApiService : PostApiService {
    var responses: MutableList<PostDto> = mutableListOf()
    var shouldFail = false

    override suspend fun getPosts(page: Int, limit: Int): List<PostDto> {
        if (shouldFail) throw IOException("Fake network error")
        return responses
    }

    override suspend fun getPost(id: String): PostDto {
        if (shouldFail) throw IOException("Fake network error")
        return responses.first { it.id == id }
    }
}
```

**Convention:** Use fakes (hand-written test doubles implementing the interface) for all `commonTest` tests. Fakes work on all KMP targets. Mocking libraries like Mockito or MockK are JVM-only and do not work in `commonTest`. If you need a mocking library on Android specifically, use MockK in `androidUnitTest` only.

### Kotest Integration

Kotest provides property-based testing and rich assertion DSL for Kotlin:

```kotlin
// build.gradle.kts — add Kotest
commonTest.dependencies {
    implementation("io.kotest:kotest-assertions-core:6.0.0.M1")
    implementation("io.kotest:kotest-property:6.0.0.M1")
    implementation("io.kotest:kotest-framework-engine:6.0.0.M1")
}

// Property-based testing
class PostMapperPropertyTest {

    @Test
    fun `PostDto roundtrips through domain model`() = runTest {
        checkAll(Arb.postDto()) { dto ->
            val domain = dto.toDomain()
            domain.id shouldBe dto.id
            domain.title shouldBe dto.title
            domain.body shouldBe dto.body
        }
    }
}

// Custom Arb generators
fun Arb.Companion.postDto(): Arb<PostDto> = arbitrary {
    PostDto(
        id = Arb.uuid().bind().toString(),
        title = Arb.string(1..100).bind(),
        body = Arb.string(1..500).bind(),
        authorId = Arb.uuid().bind().toString(),
        createdAt = Clock.System.now().toString(),
        updatedAt = null,
        tags = Arb.list(Arb.string(1..20), 0..5).bind(),
    )
}
```

### Platform-Specific Testing

```kotlin
// Android instrumentation tests — test platform-specific implementations
// androidApp/src/androidTest/.../SecureStorageAndroidTest.kt
@RunWith(AndroidJUnit4::class)
class SecureStorageAndroidTest {

    @get:Rule
    val activityRule = ActivityScenarioRule(MainActivity::class.java)

    @Test
    fun storesAndRetrievesValue() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val storage = SecureStorage(context)

        storage.putString("test_key", "test_value")
        assertEquals("test_value", storage.getString("test_key"))

        storage.remove("test_key")
        assertNull(storage.getString("test_key"))
    }
}

// iOS tests — run via Xcode or Gradle iosSimulatorArm64Test
// shared/src/iosTest/kotlin/.../platform/PlatformIosTest.kt
class PlatformIosTest {

    @Test
    fun platformNameContainsIOS() {
        val name = getPlatformName()
        assertTrue(name.contains("iOS") || name.contains("iPadOS"))
    }
}
```

### Ktor MockEngine for API Tests

```kotlin
// shared/src/commonTest/kotlin/.../data/remote/PostApiServiceTest.kt
class PostApiServiceTest {

    @Test
    fun `getPosts deserializes response`() = runTest {
        val mockEngine = MockEngine { request ->
            assertEquals("/v1/posts", request.url.encodedPath)
            respond(
                content = """[{"id":"1","title":"Test","body":"Body","author_id":"a1","created_at":"2026-01-01T00:00:00Z","updated_at":null,"tags":[]}]""",
                status = HttpStatusCode.OK,
                headers = headersOf(HttpHeaders.ContentType, "application/json")
            )
        }

        val client = ApiClient(engine = mockEngine, tokenProvider = FakeTokenProvider())
        val service = PostApiService(client)

        val posts = service.getPosts()
        assertEquals(1, posts.size)
        assertEquals("1", posts.first().id)
        assertEquals("Test", posts.first().title)
    }

    @Test
    fun `handles 404 with NotFound exception`() = runTest {
        val mockEngine = MockEngine {
            respond(content = "", status = HttpStatusCode.NotFound)
        }

        val client = ApiClient(engine = mockEngine, tokenProvider = FakeTokenProvider())
        val service = PostApiService(client)

        val result = runCatching { service.getPost("nonexistent") }
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is AppException.NotFound)
    }
}
```

### SQLDelight In-Memory Testing

```kotlin
// shared/src/commonTest/kotlin/.../data/local/PostDaoTest.kt
class PostDaoTest {

    private lateinit var database: AppDatabase
    private lateinit var dao: PostDao

    @BeforeTest
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        AppDatabase.Schema.create(driver)
        database = AppDatabase(driver)
        dao = PostDao(database)
    }

    @Test
    fun `insert and retrieve post`() = runTest {
        val post = testPost(id = "1", title = "Test Post")
        dao.insert(post)

        val result = dao.getById("1").first()
        assertNotNull(result)
        assertEquals("Test Post", result.title)
    }

    @Test
    fun `deleteAll removes all posts`() = runTest {
        dao.insertAll(listOf(testPost(id = "1"), testPost(id = "2")))
        dao.deleteAll()

        val result = dao.getAll().first()
        assertTrue(result.isEmpty())
    }
}
```

---

## 9. iOS Interop

### Kotlin/Native to Swift

When the shared module is compiled for iOS, it produces an Objective-C framework. Swift interacts with this framework through Objective-C interop. Key rules:

1. **Sealed classes become enums with associated values in Swift** (with some limitations).
2. **Kotlin interfaces become Objective-C protocols.**
3. **Kotlin coroutines require wrappers for Swift `async/await`.**
4. **Kotlin generics are erased in Objective-C** — use SKIE or explicit wrappers for type safety.
5. **Kotlin `Flow` does not bridge directly** — use SKIE or a helper function.

### SKIE — Swift-Kotlin Interface Enhancer

SKIE (by Touchlab) generates proper Swift wrappers for KMP APIs:

```kotlin
// build.gradle.kts
plugins {
    id("co.touchlab.skie") version "0.10.1"
}

skie {
    features {
        enableSwiftUIObservingPreview = true
        coroutinesInterop.set(true)
    }
}
```

With SKIE:
- Kotlin `suspend` functions become Swift `async` functions
- Kotlin `Flow` becomes Swift `AsyncSequence`
- Kotlin sealed classes become proper Swift enums
- Kotlin default parameter values are preserved

```swift
// Without SKIE — manual callback wrapper
SharedModule.shared.getPostsUseCase().invoke(
    completionHandler: { result, error in
        // Ugly callback-based API
    }
)

// With SKIE — native Swift async/await
let result = try await GetPostsUseCase().invoke()
// Clean, idiomatic Swift
```

### Manual Flow Wrapper (without SKIE)

```kotlin
// shared/src/iosMain/kotlin/.../util/FlowWrapper.kt
class FlowWrapper<T>(private val flow: Flow<T>) {
    fun collect(onEach: (T) -> Unit, onError: (Throwable) -> Unit, onComplete: () -> Unit): Cancellable {
        val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main)
        scope.launch {
            try {
                flow.collect { onEach(it) }
                onComplete()
            } catch (e: Throwable) {
                onError(e)
            }
        }
        return object : Cancellable {
            override fun cancel() { scope.cancel() }
        }
    }
}

interface Cancellable {
    fun cancel()
}
```

```swift
// Swift usage of FlowWrapper
let wrapper = viewModel.uiStateFlow()
cancellable = wrapper.collect(
    onEach: { state in
        self.updateUI(state: state)
    },
    onError: { error in
        print("Error: \(error)")
    },
    onComplete: {
        print("Flow completed")
    }
)

// In deinit
cancellable?.cancel()
```

### Kotlin/Native Memory Model

Since Kotlin 1.7.20+, the new memory model is the default. Key implications:

- **No more freezing** — objects can be shared across threads without `freeze()`
- **No more `InvalidMutabilityException`** — mutable state works across threads
- **GC handles deallocation** — but be aware of reference cycles between Kotlin and Swift objects
- **`@SharedImmutable` and `@ThreadLocal` are deprecated** — remove these annotations

### Exporting Framework

```kotlin
// shared/build.gradle.kts
kotlin {
    listOf(iosX64(), iosArm64(), iosSimulatorArm64()).forEach { target ->
        target.binaries.framework {
            baseName = "shared"
            isStatic = true  // Required for Compose Multiplatform

            // Export transitive dependencies if needed
            export(libs.kotlinx.datetime)
        }
    }
}
```

**Convention:** Use static frameworks (`isStatic = true`) for Compose Multiplatform. Dynamic frameworks cause issues with Compose resource bundling. Only `export()` dependencies that the Swift code needs to access directly — do not over-export.

### CocoaPods vs. SPM vs. Direct Framework

| Method | Use When | Setup |
|---|---|---|
| Direct XCFramework | Simple projects, CI builds | `./gradlew :shared:assembleXCFramework` |
| CocoaPods | Existing CocoaPods project, need iOS deps | `kotlin-cocoapods` Gradle plugin |
| SPM | Modern Swift project, no CocoaPods baggage | Manual Package.swift pointing to XCFramework |

**Convention:** Default to direct framework embedding for new projects. Use CocoaPods only when the iOS project already uses it for other dependencies. SPM support for KMP is still maturing — use it only if the team has SPM expertise.

---

## 10. Domain Layer

### Clean Architecture in KMP

The domain layer is pure Kotlin — no platform dependencies, no framework imports, no annotations:

```kotlin
// shared/src/commonMain/kotlin/.../domain/model/Post.kt
data class Post(
    val id: String,
    val title: String,
    val body: String,
    val authorId: String,
    val createdAt: Instant,
    val updatedAt: Instant?,
    val tags: List<String>,
    val isBookmarked: Boolean = false,
)

data class User(
    val id: String,
    val name: String,
    val email: String,
    val avatarUrl: String?,
    val role: UserRole,
)

enum class UserRole {
    VIEWER,
    EDITOR,
    ADMIN,
}
```

### Repository Interfaces

```kotlin
// shared/src/commonMain/kotlin/.../domain/repository/PostRepository.kt
interface PostRepository {
    fun getPosts(): Flow<List<Post>>
    suspend fun refreshPosts(): Result<Unit>
    suspend fun getPost(id: String): Result<Post>
    suspend fun createPost(title: String, body: String, tags: List<String>): Result<Post>
    suspend fun deletePost(id: String): Result<Unit>
    suspend fun toggleBookmark(id: String): Result<Post>
}

interface AuthRepository {
    val isAuthenticated: StateFlow<Boolean>
    suspend fun login(email: String, password: String): Result<User>
    suspend fun logout(): Result<Unit>
    suspend fun refreshToken(): Result<Unit>
    fun getCurrentUser(): Flow<User?>
}
```

### Use Cases

```kotlin
// shared/src/commonMain/kotlin/.../domain/usecase/GetPostsUseCase.kt
class GetPostsUseCase(private val postRepository: PostRepository) {
    suspend operator fun invoke(): Result<List<Post>> {
        return postRepository.refreshPosts().map {
            postRepository.getPosts().first()
        }
    }
}

class CreatePostUseCase(private val postRepository: PostRepository) {
    suspend operator fun invoke(title: String, body: String, tags: List<String>): Result<Post> {
        require(title.isNotBlank()) { "Title must not be blank" }
        require(body.isNotBlank()) { "Body must not be blank" }
        require(title.length <= 200) { "Title must be 200 characters or less" }
        return postRepository.createPost(title, body, tags)
    }
}

class ToggleBookmarkUseCase(private val postRepository: PostRepository) {
    suspend operator fun invoke(postId: String): Result<Post> {
        return postRepository.toggleBookmark(postId)
    }
}
```

**Convention:** Use cases have a single `invoke` operator. Validation logic lives in use cases, not in repositories or ViewModels. Use cases compose repository calls and enforce business rules.

---

## 11. Development Workflow

### Feature Development Cycle (KMP-specific)

```
1. Define domain model and repository interface
2. Write failing tests (commonTest) — use case, ViewModel, mapper
3. Implement domain layer (use case)
4. Implement data layer (DTO, API service, DAO, repository)
5. Write Compose UI (screen + content composable)
6. Wire navigation
7. Run: ./gradlew :shared:allTests
8. Run: ./gradlew :androidApp:connectedAndroidTest (if platform-specific)
9. Run: ./gradlew detekt (static analysis)
10. Build iOS: ./gradlew :shared:assembleXCFramework
11. Test iOS: open iosApp in Xcode, run tests
```

### Common Commands

```bash
# Build & Test
./gradlew :shared:allTests                    # Run all shared tests (all platforms)
./gradlew :shared:jvmTest                     # Run shared tests on JVM (fastest)
./gradlew :shared:iosSimulatorArm64Test       # Run shared tests on iOS simulator
./gradlew :androidApp:assembleDebug           # Build Android debug APK
./gradlew :androidApp:connectedAndroidTest    # Android instrumentation tests
./gradlew :shared:assembleXCFramework         # Build iOS framework

# Code Quality
./gradlew detekt                              # Static analysis (Kotlin)
./gradlew :shared:lintKotlin                  # Lint (ktlint via Kotlinter)
./gradlew :shared:formatKotlin                # Auto-format

# SQLDelight
./gradlew :shared:generateCommonMainAppDatabaseInterface  # Regenerate after .sq changes
./gradlew :shared:verifySqlDelightMigration               # Verify migrations

# Dependency Management
./gradlew dependencyUpdates                   # Check for version updates
./gradlew :shared:dependencies                # Full dependency tree

# Clean
./gradlew clean                               # Clean all build outputs
```

### Gradle Configuration

```kotlin
// settings.gradle.kts
pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "MyApp"
include(":shared")
include(":androidApp")
```

### Detekt Configuration

```yaml
# detekt.yml
complexity:
  LongMethod:
    threshold: 30
  LongParameterList:
    functionThreshold: 8
    constructorThreshold: 10  # Compose composables often have many params
  ComplexCondition:
    threshold: 4
  TooManyFunctions:
    thresholdInFiles: 20
    thresholdInClasses: 15
    thresholdInInterfaces: 10

style:
  MagicNumber:
    ignoreNumbers:
      - '-1'
      - '0'
      - '1'
      - '2'
      - '16'  # Common dp values
      - '24'
      - '48'
    ignoreHashCodeFunction: true
    ignorePropertyDeclaration: true
    ignoreAnnotation: true
    ignoreCompanionObjectPropertyDeclaration: true
  MaxLineLength:
    maxLineLength: 120
    excludeCommentStatements: true
  ForbiddenComment:
    values:
      - 'TODO:'
      - 'FIXME:'
      - 'HACK:'

formatting:
  active: true
  android: true
  autoCorrect: true

compose:
  # Compose-specific rules (via detekt-compose plugin)
  ComposableNaming:
    active: true
  MutableParams:
    active: true
  ViewModelForwarding:
    active: true
  ViewModelInjection:
    active: true
```

### EditorConfig

```ini
# .editorconfig
[*.kt]
ktlint_code_style = android_studio
max_line_length = 120
indent_size = 4
insert_final_newline = true

# Compose
ktlint_function_naming_ignore_when_annotated_with = Composable
```

---

## 12. CI/CD Pipeline

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
  test:
    runs-on: macos-latest  # Required for iOS simulator tests
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-java@v4
        with:
          distribution: 'zulu'
          java-version: '21'

      - uses: gradle/actions/setup-gradle@v4

      - name: Run shared tests (JVM)
        run: ./gradlew :shared:jvmTest

      - name: Run shared tests (iOS simulator)
        run: ./gradlew :shared:iosSimulatorArm64Test

      - name: Run Android unit tests
        run: ./gradlew :androidApp:testDebugUnitTest

      - name: Detekt
        run: ./gradlew detekt

      - name: Lint
        run: ./gradlew :shared:lintKotlin

      - name: Build Android APK
        run: ./gradlew :androidApp:assembleDebug

      - name: Build iOS framework
        run: ./gradlew :shared:assembleXCFramework

  android-deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          distribution: 'zulu'
          java-version: '21'
      - uses: gradle/actions/setup-gradle@v4

      - name: Build release AAB
        run: ./gradlew :androidApp:bundleRelease
        env:
          KEYSTORE_PASSWORD: ${{ secrets.KEYSTORE_PASSWORD }}
          KEY_ALIAS: ${{ secrets.KEY_ALIAS }}
          KEY_PASSWORD: ${{ secrets.KEY_PASSWORD }}

      - name: Upload to Play Store
        uses: r0adkll/upload-google-play@v1
        with:
          serviceAccountJsonPlainText: ${{ secrets.PLAY_SERVICE_ACCOUNT }}
          packageName: com.example.app
          releaseFiles: androidApp/build/outputs/bundle/release/*.aab
          track: internal

  ios-deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          distribution: 'zulu'
          java-version: '21'
      - uses: gradle/actions/setup-gradle@v4

      - name: Build shared framework
        run: ./gradlew :shared:assembleXCFramework

      - name: Build and archive iOS app
        run: |
          cd iosApp
          xcodebuild archive \
            -project iosApp.xcodeproj \
            -scheme iosApp \
            -archivePath build/iosApp.xcarchive \
            -destination 'generic/platform=iOS' \
            CODE_SIGN_IDENTITY="${{ secrets.CODE_SIGN_IDENTITY }}" \
            PROVISIONING_PROFILE="${{ secrets.PROVISIONING_PROFILE }}"

      - name: Export IPA
        run: |
          cd iosApp
          xcodebuild -exportArchive \
            -archivePath build/iosApp.xcarchive \
            -exportPath build/ipa \
            -exportOptionsPlist ExportOptions.plist

      - name: Upload to TestFlight
        uses: apple-actions/upload-testflight-build@v1
        with:
          app-path: iosApp/build/ipa/iosApp.ipa
          issuer-id: ${{ secrets.APP_STORE_CONNECT_ISSUER_ID }}
          api-key-id: ${{ secrets.APP_STORE_CONNECT_KEY_ID }}
          api-private-key: ${{ secrets.APP_STORE_CONNECT_PRIVATE_KEY }}
```

### Signing Configuration

```kotlin
// androidApp/build.gradle.kts
android {
    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("KEYSTORE_PATH") ?: "keystore.jks")
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
            keyAlias = System.getenv("KEY_ALIAS") ?: ""
            keyPassword = System.getenv("KEY_PASSWORD") ?: ""
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }
}
```

**Convention:** Never commit signing keys, keystore files, or provisioning profiles to version control. Use CI secrets and environment variables exclusively. Add `*.jks`, `*.keystore`, `*.p12`, `*.mobileprovision` to `.gitignore`.

---

## 13. Security

### Secrets Management

```kotlin
// Never hardcode secrets in source code
// Bad
private const val API_KEY = "sk-1234567890"

// Good — read from platform-secure storage at runtime
class ApiKeyProvider(private val secureStorage: SecureStorage) {
    fun getApiKey(): String {
        return secureStorage.getString("api_key")
            ?: throw IllegalStateException("API key not configured")
    }
}

// Good — inject via build config for non-sensitive keys
// androidApp/build.gradle.kts
android {
    defaultConfig {
        buildConfigField("String", "BASE_URL", "\"${System.getenv("BASE_URL")}\"")
    }
}
```

### Network Security

```kotlin
// Ktor — always enforce HTTPS
defaultRequest {
    url {
        protocol = URLProtocol.HTTPS
    }
}

// Certificate pinning (OkHttp engine on Android)
// shared/src/androidMain/kotlin/.../di/PlatformModule.android.kt
actual val platformModule = module {
    single<HttpClientEngine> {
        OkHttp.create {
            val certificatePinner = CertificatePinner.Builder()
                .add("api.example.com", "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
                .build()
            preconfigured = OkHttpClient.Builder()
                .certificatePinner(certificatePinner)
                .build()
        }
    }
}
```

### Android-Specific Security

```xml
<!-- AndroidManifest.xml -->
<application
    android:networkSecurityConfig="@xml/network_security_config"
    android:allowBackup="false"
    android:fullBackupContent="false"
    ...>
```

```xml
<!-- res/xml/network_security_config.xml -->
<network-security-config>
    <base-config cleartextTrafficPermitted="false">
        <trust-anchors>
            <certificates src="system" />
        </trust-anchors>
    </base-config>
    <!-- Debug-only exception for local development -->
    <debug-overrides>
        <trust-anchors>
            <certificates src="user" />
        </trust-anchors>
    </debug-overrides>
</network-security-config>
```

### iOS-Specific Security

```xml
<!-- Info.plist — App Transport Security -->
<key>NSAppTransportSecurity</key>
<dict>
    <!-- ATS is enabled by default — do NOT add NSAllowsArbitraryLoads -->
    <!-- Only add exceptions for specific domains if absolutely necessary -->
</dict>
```

### ProGuard / R8 Rules

```
# proguard-rules.pro

# Kotlinx Serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt
-keepclassmembers class kotlinx.serialization.json.** { *** Companion; }
-keepclasseswithmembers class kotlinx.serialization.json.** {
    kotlinx.serialization.KSerializer serializer(...);
}

# Ktor
-keep class io.ktor.** { *; }
-dontwarn io.ktor.**

# SQLDelight
-keep class com.example.app.db.** { *; }

# Keep data classes used in serialization
-keep class com.example.app.data.remote.dto.** { *; }
```

**Convention:** Always enable R8 (minification + obfuscation) for release builds. Test the release build on a real device after adding ProGuard rules — serialization and reflection-heavy code frequently breaks with aggressive shrinking.

---

## 14. Coverage Enforcement

### Kover (JetBrains Coverage)

Kover is the official Kotlin coverage tool with multiplatform support:

```kotlin
// build.gradle.kts (root)
plugins {
    id("org.jetbrains.kotlinx.kover") version "0.9.1"
}

// shared/build.gradle.kts
kover {
    reports {
        total {
            filters {
                excludes {
                    classes(
                        "*ComposableSingletons*",
                        "*_Impl*",
                        "*BuildConfig*",
                        "*_Factory*",
                        "*.di.*Module*",
                        "*.platform.*",  // Platform-specific expect/actual implementations
                    )
                    annotatedBy(
                        "androidx.compose.runtime.Composable",  // Composable functions
                        "androidx.compose.ui.tooling.preview.Preview",
                    )
                }
            }

            html { onCheck = true }
            xml { onCheck = true }

            verify {
                rule {
                    minBound(80)  // CI gate — fail below 80%
                }
            }
        }
    }
}
```

### Commands

```bash
./gradlew koverHtmlReport     # Generate HTML report
./gradlew koverXmlReport      # Generate XML report (for CI)
./gradlew koverVerify          # Verify coverage meets minimum
./gradlew koverLog             # Print coverage to stdout
```

### Coverage Strategy

| Layer | Target | Rationale |
|---|---|---|
| Domain (use cases, models) | 100% | Pure Kotlin, no excuses |
| Data (repositories, DAOs, mappers) | 90%+ | Some error paths are hard to trigger |
| ViewModels | 90%+ | All state transitions covered |
| Compose UI | Excluded from line coverage | Test via snapshot tests and E2E |
| Platform (expect/actual) | Per-platform tests | Tested in platform-specific test source sets |
| DI modules | Excluded | Configuration, not logic |

**Convention:** Target 100% for the domain layer. Exclude Compose UI functions from line-coverage metrics — they are validated through snapshot tests and manual QA. The Kover `verify` rule is the CI gate.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with labeled sections |
| **labels** | Always visible, above field, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms, inline validation on blur for long forms |
| **errors** | Inline per field + error summary, multi-cue (icon + text + border color) |
| **accessibility** | `contentDescription` on all interactive elements, semantic roles |
| **mobile** | Correct `KeyboardType` per field, minimum 48dp touch targets |
| **cta** | Outcome-focused text ("Create Account" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, clear post-submit feedback |
| **performance** | Debounce validation, no recomposition on every keystroke |

### Compose Form Pattern

```kotlin
@Composable
fun RegistrationForm(
    onSubmit: (RegistrationData) -> Unit,
    isLoading: Boolean = false,
) {
    var name by remember { mutableStateOf("") }
    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var nameError by remember { mutableStateOf<String?>(null) }
    var emailError by remember { mutableStateOf<String?>(null) }
    var passwordError by remember { mutableStateOf<String?>(null) }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp)
            .verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text(
            text = "Create Your Account",
            style = MaterialTheme.typography.headlineMedium,
        )

        // Name field
        OutlinedTextField(
            value = name,
            onValueChange = { name = it; nameError = null },
            label = { Text("Full Name") },
            isError = nameError != null,
            supportingText = nameError?.let { { Text(it, color = MaterialTheme.colorScheme.error) } },
            singleLine = true,
            keyboardOptions = KeyboardOptions(
                capitalization = KeyboardCapitalization.Words,
                imeAction = ImeAction.Next,
                keyboardType = KeyboardType.Text,
            ),
            modifier = Modifier
                .fillMaxWidth()
                .semantics { contentDescription = "Full Name input field" },
        )

        // Email field
        OutlinedTextField(
            value = email,
            onValueChange = { email = it; emailError = null },
            label = { Text("Email Address") },
            isError = emailError != null,
            supportingText = emailError?.let { { Text(it, color = MaterialTheme.colorScheme.error) } },
            singleLine = true,
            keyboardOptions = KeyboardOptions(
                keyboardType = KeyboardType.Email,
                imeAction = ImeAction.Next,
            ),
            modifier = Modifier
                .fillMaxWidth()
                .semantics { contentDescription = "Email address input field" },
        )

        // Password field
        var passwordVisible by remember { mutableStateOf(false) }
        OutlinedTextField(
            value = password,
            onValueChange = { password = it; passwordError = null },
            label = { Text("Password") },
            isError = passwordError != null,
            supportingText = passwordError?.let { { Text(it, color = MaterialTheme.colorScheme.error) } },
            singleLine = true,
            visualTransformation = if (passwordVisible) VisualTransformation.None
                                   else PasswordVisualTransformation(),
            trailingIcon = {
                IconButton(onClick = { passwordVisible = !passwordVisible }) {
                    Icon(
                        imageVector = if (passwordVisible) Icons.Filled.VisibilityOff
                                      else Icons.Filled.Visibility,
                        contentDescription = if (passwordVisible) "Hide password" else "Show password"
                    )
                }
            },
            keyboardOptions = KeyboardOptions(
                keyboardType = KeyboardType.Password,
                imeAction = ImeAction.Done,
            ),
            modifier = Modifier
                .fillMaxWidth()
                .semantics { contentDescription = "Password input field" },
        )

        // Submit button — outcome-focused text, loading state
        Button(
            onClick = {
                val errors = validate(name, email, password)
                nameError = errors["name"]
                emailError = errors["email"]
                passwordError = errors["password"]
                if (errors.isEmpty()) {
                    onSubmit(RegistrationData(name, email, password))
                }
            },
            enabled = !isLoading,
            modifier = Modifier
                .fillMaxWidth()
                .height(48.dp),
        ) {
            if (isLoading) {
                CircularProgressIndicator(
                    modifier = Modifier.size(20.dp),
                    strokeWidth = 2.dp,
                    color = MaterialTheme.colorScheme.onPrimary,
                )
            } else {
                Text("Create Account")
            }
        }
    }
}

private fun validate(name: String, email: String, password: String): Map<String, String> {
    val errors = mutableMapOf<String, String>()
    if (name.isBlank()) errors["name"] = "Name is required"
    if (!email.matches(Regex("^[^@]+@[^@]+\\.[^@]+$"))) errors["email"] = "Enter a valid email address"
    if (password.length < 8) errors["password"] = "Password must be at least 8 characters"
    return errors
}
```

**Convention:** Always set `keyboardOptions` with the correct `KeyboardType` and `ImeAction`. Always provide `contentDescription` for accessibility. Always show loading state on the submit button. Validation on submit for forms under 7 fields, inline validation on focus loss for longer forms.

---

## 16. Anti-Patterns (KMP-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Putting business logic in `androidMain` or `iosMain` | All business logic goes in `commonMain` — platform source sets are for platform API wrappers only |
| Using `java.time` or `NSDate` in shared code | Use `kotlinx-datetime` in `commonMain` — it works on all platforms |
| Using `java.io.File` in shared code | Use `kotlinx-io` or expect/actual with platform file APIs |
| Passing complex objects through navigation | Pass IDs (String/Int), let the destination load the data via ViewModel |
| Using `MutableStateFlow` in public ViewModel API | Expose `StateFlow` publicly, keep `MutableStateFlow` private |
| Exposing `suspend` functions to Swift without SKIE | Use SKIE or wrap suspend functions in Flow/callback helpers for iOS consumption |
| Using MockK or Mockito in `commonTest` | Use hand-written fakes — mocking libraries are JVM-only |
| Declaring dependency versions inline in build.gradle.kts | Use `gradle/libs.versions.toml` version catalog exclusively |
| Using `GlobalScope.launch` | Use `viewModelScope` in ViewModels, inject `CoroutineScope` elsewhere |
| Putting UI state in multiple StateFlows per screen | One sealed interface `UiState` per screen with `Loading`, `Success`, `Error` variants |
| Using `LaunchedEffect(Unit)` for data loading | Load data in ViewModel `init` block — `LaunchedEffect(Unit)` re-triggers on recomposition in some cases |
| Sharing mutable state between Compose and platform code | Use unidirectional data flow — state flows down, events flow up |
| Writing `actual` implementations with business logic | `actual` should be a thin wrapper around platform SDK calls — all logic stays in `commonMain` |
| Using `Thread.sleep` or `delay` in tests | Use `runTest` with `advanceUntilIdle()` or `advanceTimeBy()` from kotlinx-coroutines-test |
| Storing tokens in SharedPreferences / NSUserDefaults | Use `EncryptedSharedPreferences` (Android) or Keychain (iOS) via expect/actual SecureStorage |
| Using `remember` for state that outlives the composable | Use ViewModel for state that must survive configuration changes and navigation |
| Dynamic framework for Compose Multiplatform iOS | Use `isStatic = true` — dynamic frameworks cause Compose resource bundling issues |
| Importing platform-specific libraries in `commonMain` | Use expect/actual or interface+DI to abstract platform libraries |
| Skipping ProGuard / R8 for release builds | Always enable minification — test release builds on real devices after adding rules |
| Running iOS tests only in Xcode | Run `./gradlew :shared:iosSimulatorArm64Test` in CI — do not rely on manual Xcode runs |
| Using `String` for route definitions | Use `@Serializable` sealed interface routes with type-safe navigation (Navigation 2.9+) |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a KMP patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:kmp&title=[KMP]%20)**

Use the `patterns:kmp` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
