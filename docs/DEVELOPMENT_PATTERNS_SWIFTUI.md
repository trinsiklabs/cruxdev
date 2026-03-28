# Development Patterns — SwiftUI Stack

Swift / SwiftUI / SwiftData / Combine / Swift Testing

This document captures stack-specific patterns, conventions, and decisions for SwiftUI stack projects (Swift 6+, SwiftUI 6+, SwiftData, async/await, Swift Testing). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure SwiftUI apps, test with Swift Testing, use @Observable, integrate with UIKit, deploy to the App Store, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to the current Xcode release. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Swift | 6.1+ | Strict concurrency checking enabled |
| SwiftUI | 6+ | Ships with iOS 18 / macOS 15 / watchOS 11 / tvOS 18 / visionOS 2 |
| SwiftData | 2+ | Declarative persistence (replaces Core Data for new projects) |
| Xcode | 16.3+ | Latest stable release |
| iOS deployment target | 17.0+ | Minimum deployment target — covers 95%+ of active devices |
| macOS deployment target | 14.0+ | Sonoma and later |
| Swift Testing | 1.0+ | Modern test framework (replaces XCTest for new tests) |
| XCTest | Built-in | Legacy tests and UI testing (XCUITest) |
| Swift Package Manager | Built-in | Dependency management — no CocoaPods, no Carthage |
| swift-format | 6.1+ | Official Swift formatter |
| SwiftLint | 0.58+ | Lint enforcement |

### Version Constraint Policy

Use exact or range-based version constraints in `Package.swift`:

```swift
// Good — pinned to minor version, allows patch updates
.package(url: "https://github.com/vendor/lib.git", from: "2.3.0"),

// Good — pinned to exact version for critical dependencies
.package(url: "https://github.com/vendor/lib.git", exact: "1.5.2"),

// Good — explicit range when you need to cap the upper bound
.package(url: "https://github.com/vendor/lib.git", "2.0.0"..<"3.0.0"),

// Bad — branch-based, non-reproducible builds
.package(url: "https://github.com/vendor/lib.git", branch: "main"),
```

Exception: for pre-release packages or active development forks, branch pinning is acceptable with a TODO comment and tracking issue.

### Swift 6 Strict Concurrency

Swift 6 enables strict concurrency checking by default. All new code must compile cleanly under strict concurrency mode. This is not optional.

```swift
// Package.swift — enforce strict concurrency
swiftSettings: [
    .strictConcurrency(.complete)
]
```

In Xcode project settings: `SWIFT_STRICT_CONCURRENCY = complete`.

---

## 2. Project Structure

### App Architecture (MVVM + Coordinator)

SwiftUI projects use MVVM with an optional Coordinator layer for complex navigation. The file system mirrors the architecture:

```
MyApp/
├── App/
│   ├── MyApp.swift              # @main entry point
│   ├── AppDelegate.swift        # UIKit lifecycle hooks (if needed)
│   └── Configuration/
│       ├── AppEnvironment.swift  # Environment configuration (dev/staging/prod)
│       └── Constants.swift       # App-wide constants
├── Features/                    # Feature modules (one directory per feature)
│   ├── Authentication/
│   │   ├── Views/
│   │   │   ├── LoginView.swift
│   │   │   ├── SignUpView.swift
│   │   │   └── Components/
│   │   │       └── SocialLoginButton.swift
│   │   ├── ViewModels/
│   │   │   ├── LoginViewModel.swift
│   │   │   └── SignUpViewModel.swift
│   │   ├── Models/
│   │   │   └── AuthCredentials.swift
│   │   └── Services/
│   │       └── AuthService.swift
│   ├── Dashboard/
│   │   ├── Views/
│   │   ├── ViewModels/
│   │   └── Models/
│   ├── Settings/
│   │   ├── Views/
│   │   ├── ViewModels/
│   │   └── Models/
│   └── Profile/
│       ├── Views/
│       ├── ViewModels/
│       └── Models/
├── Core/                        # Shared infrastructure
│   ├── Navigation/
│   │   ├── Router.swift          # Central navigation router
│   │   ├── Route.swift           # Route enum definitions
│   │   └── NavigationCoordinator.swift
│   ├── Networking/
│   │   ├── APIClient.swift       # Protocol-based HTTP client
│   │   ├── HTTPClient.swift      # Production URLSession implementation
│   │   ├── Endpoint.swift        # Endpoint protocol
│   │   └── APIError.swift        # Typed error hierarchy
│   ├── Persistence/
│   │   ├── ModelContainer+App.swift  # SwiftData container config
│   │   ├── MigrationPlan.swift       # SwiftData schema migrations
│   │   └── Models/                   # @Model types (SwiftData)
│   │       ├── UserProfile.swift
│   │       ├── CachedItem.swift
│   │       └── AppSettings.swift
│   ├── Extensions/
│   │   ├── View+Extensions.swift
│   │   ├── Color+Brand.swift
│   │   └── Date+Formatting.swift
│   ├── Components/              # Reusable UI components
│   │   ├── LoadingView.swift
│   │   ├── ErrorView.swift
│   │   ├── EmptyStateView.swift
│   │   └── AsyncButton.swift
│   └── Utilities/
│       ├── Logger+App.swift      # os.Logger wrappers
│       ├── Keychain.swift        # Keychain access
│       └── FeatureFlags.swift
├── Resources/
│   ├── Assets.xcassets           # Images, colors, app icon
│   ├── Localizable.xcstrings     # String catalogs (Xcode 15+)
│   └── Info.plist
└── Preview Content/
    └── PreviewSampleData.swift   # Preview-only mock data
```

**Convention:** One feature per directory. Each feature contains its own Views, ViewModels, Models, and Services. Cross-feature dependencies go through `Core/`.

**Convention:** Views are SwiftUI structs. ViewModels are `@Observable` classes. Models are value types (structs) or `@Model` classes (for SwiftData).

### Test Structure

Tests mirror the feature structure:

```
MyAppTests/
├── Features/
│   ├── Authentication/
│   │   ├── LoginViewModelTests.swift
│   │   ├── SignUpViewModelTests.swift
│   │   └── AuthServiceTests.swift
│   ├── Dashboard/
│   │   └── DashboardViewModelTests.swift
│   └── Settings/
│       └── SettingsViewModelTests.swift
├── Core/
│   ├── Networking/
│   │   ├── APIClientTests.swift
│   │   └── EndpointTests.swift
│   └── Persistence/
│       └── SwiftDataTests.swift
├── Mocks/
│   ├── MockAPIClient.swift
│   ├── MockAuthService.swift
│   └── MockModelContainer.swift
└── Helpers/
    ├── TestModelContainer.swift   # In-memory SwiftData for tests
    └── XCTestCase+Async.swift     # Async test helpers

MyAppUITests/
├── Flows/
│   ├── LoginFlowTests.swift
│   ├── OnboardingFlowTests.swift
│   └── SettingsFlowTests.swift
├── Pages/                        # Page Object pattern
│   ├── LoginPage.swift
│   ├── DashboardPage.swift
│   └── SettingsPage.swift
└── Helpers/
    └── XCUIApplication+Launch.swift
```

---

## 3. @Observable and State Management

### The @Observable Macro (Swift 5.9+ / iOS 17+)

`@Observable` is the standard for view models and shared state. It replaces `ObservableObject` + `@Published` with automatic observation tracking.

```swift
import Observation

@Observable
final class DashboardViewModel {
    var items: [Item] = []
    var isLoading = false
    var errorMessage: String?

    private let apiClient: APIClientProtocol

    init(apiClient: APIClientProtocol = HTTPClient.shared) {
        self.apiClient = apiClient
    }

    func loadItems() async {
        isLoading = true
        defer { isLoading = false }

        do {
            items = try await apiClient.fetchItems()
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}
```

### @Observable vs ObservableObject Decision Matrix

| Scenario | Use | Reason |
|---|---|---|
| New view model (iOS 17+) | `@Observable` | Modern, less boilerplate, better performance |
| Shared state across views (iOS 17+) | `@Observable` in `@Environment` | Automatic dependency tracking |
| Legacy code targeting iOS 16 | `ObservableObject` + `@Published` | `@Observable` requires iOS 17 |
| SwiftData `@Model` classes | `@Model` (inherits observation) | SwiftData handles observation |
| Simple value state in a view | `@State` | Local view state, no class needed |
| Parent-to-child binding | `@Binding` | Two-way connection to parent state |

### Injecting @Observable Objects

Use `@Environment` for dependency injection of `@Observable` objects:

```swift
// Define environment key (not needed if using .environment(viewModel) directly)
@Observable
final class AppState {
    var currentUser: User?
    var isAuthenticated: Bool { currentUser != nil }
}

// Inject at app root
@main
struct MyApp: App {
    @State private var appState = AppState()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(appState)
        }
    }
}

// Consume in any descendant view
struct ProfileView: View {
    @Environment(AppState.self) private var appState

    var body: some View {
        if let user = appState.currentUser {
            Text("Hello, \(user.name)")
        }
    }
}
```

### @State for View-Local Observable Objects

When a view owns the lifecycle of an `@Observable` object:

```swift
struct DashboardView: View {
    @State private var viewModel = DashboardViewModel()

    var body: some View {
        List(viewModel.items) { item in
            ItemRow(item: item)
        }
        .overlay {
            if viewModel.isLoading {
                ProgressView()
            }
        }
        .task {
            await viewModel.loadItems()
        }
    }
}
```

### @Bindable for Mutable Bindings from @Observable

When you need to create `Binding` values from an `@Observable` object:

```swift
struct EditProfileView: View {
    @Bindable var viewModel: EditProfileViewModel

    var body: some View {
        Form {
            TextField("Name", text: $viewModel.name)
            TextField("Email", text: $viewModel.email)
            Toggle("Notifications", isOn: $viewModel.notificationsEnabled)
        }
    }
}
```

### Avoiding Unnecessary Observation

Use `@ObservationIgnored` for properties that should not trigger view updates:

```swift
@Observable
final class SearchViewModel {
    var query = ""
    var results: [SearchResult] = []

    @ObservationIgnored
    private var searchTask: Task<Void, Never>?

    @ObservationIgnored
    private let debounceInterval: Duration = .milliseconds(300)
}
```

---

## 4. NavigationStack and Routing

### Type-Safe Navigation with NavigationStack

Every app uses `NavigationStack` with a typed navigation path. No raw `NavigationLink(destination:)`.

```swift
// Define routes as an enum
enum Route: Hashable {
    case itemDetail(Item.ID)
    case userProfile(User.ID)
    case settings
    case settingsDetail(SettingsRoute)
}

enum SettingsRoute: Hashable {
    case account
    case notifications
    case privacy
    case about
}
```

### Navigation Router

Centralize navigation state in a router:

```swift
@Observable
final class Router {
    var path = NavigationPath()
    var sheet: Sheet?
    var fullScreenCover: FullScreenCover?

    enum Sheet: Identifiable {
        case newItem
        case editItem(Item)
        case filter

        var id: String {
            switch self {
            case .newItem: "newItem"
            case .editItem(let item): "editItem-\(item.id)"
            case .filter: "filter"
            }
        }
    }

    enum FullScreenCover: Identifiable {
        case onboarding
        case imageViewer(URL)

        var id: String {
            switch self {
            case .onboarding: "onboarding"
            case .imageViewer(let url): "imageViewer-\(url)"
            }
        }
    }

    func navigate(to route: Route) {
        path.append(route)
    }

    func popToRoot() {
        path = NavigationPath()
    }

    func pop() {
        if !path.isEmpty {
            path.removeLast()
        }
    }

    func present(_ sheet: Sheet) {
        self.sheet = sheet
    }

    func presentFullScreen(_ cover: FullScreenCover) {
        self.fullScreenCover = cover
    }
}
```

### Wiring NavigationStack

```swift
struct ContentView: View {
    @State private var router = Router()

    var body: some View {
        NavigationStack(path: $router.path) {
            DashboardView()
                .navigationDestination(for: Route.self) { route in
                    switch route {
                    case .itemDetail(let id):
                        ItemDetailView(itemID: id)
                    case .userProfile(let id):
                        UserProfileView(userID: id)
                    case .settings:
                        SettingsView()
                    case .settingsDetail(let settingsRoute):
                        SettingsDetailView(route: settingsRoute)
                    }
                }
        }
        .sheet(item: $router.sheet) { sheet in
            switch sheet {
            case .newItem:
                NewItemView()
            case .editItem(let item):
                EditItemView(item: item)
            case .filter:
                FilterView()
            }
        }
        .fullScreenCover(item: $router.fullScreenCover) { cover in
            switch cover {
            case .onboarding:
                OnboardingView()
            case .imageViewer(let url):
                ImageViewerView(url: url)
            }
        }
        .environment(router)
    }
}
```

### Deep Linking

Support deep links by parsing URLs into routes:

```swift
extension Router {
    func handle(url: URL) {
        guard let components = URLComponents(url: url, resolvingAgainstBaseURL: true) else { return }

        switch components.path {
        case _ where components.path.hasPrefix("/items/"):
            let id = String(components.path.dropFirst("/items/".count))
            navigate(to: .itemDetail(id))
        case "/settings":
            navigate(to: .settings)
        default:
            break
        }
    }
}

// In the App
WindowGroup {
    ContentView()
        .onOpenURL { url in
            router.handle(url: url)
        }
}
```

### TabView with Per-Tab Navigation

Each tab maintains its own `NavigationStack`:

```swift
struct MainTabView: View {
    @State private var selectedTab = Tab.dashboard

    enum Tab: Hashable {
        case dashboard, search, profile
    }

    var body: some View {
        TabView(selection: $selectedTab) {
            Tab("Dashboard", systemImage: "house", value: .dashboard) {
                NavigationStack {
                    DashboardView()
                }
            }
            Tab("Search", systemImage: "magnifyingglass", value: .search) {
                NavigationStack {
                    SearchView()
                }
            }
            Tab("Profile", systemImage: "person", value: .profile) {
                NavigationStack {
                    ProfileView()
                }
            }
        }
    }
}
```

---

## 5. SwiftData Patterns

### Model Definition

Every persisted model uses the `@Model` macro. Models are classes, not structs.

```swift
import SwiftData

@Model
final class UserProfile {
    var name: String
    var email: String
    var avatarURL: URL?
    var createdAt: Date
    var updatedAt: Date

    @Relationship(deleteRule: .cascade, inverse: \Post.author)
    var posts: [Post] = []

    @Relationship(deleteRule: .nullify, inverse: \Tag.users)
    var tags: [Tag] = []

    // Transient properties (not persisted)
    @Transient
    var isEditing = false

    init(name: String, email: String, avatarURL: URL? = nil) {
        self.name = name
        self.email = email
        self.avatarURL = avatarURL
        self.createdAt = .now
        self.updatedAt = .now
    }
}

@Model
final class Post {
    var title: String
    var body: String
    var publishedAt: Date?
    var author: UserProfile?

    var isPublished: Bool { publishedAt != nil }

    init(title: String, body: String, author: UserProfile) {
        self.title = title
        self.body = body
        self.author = author
    }
}
```

### ModelContainer Configuration

Configure the container at the app root:

```swift
@main
struct MyApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(for: [
            UserProfile.self,
            Post.self,
            Tag.self
        ])
    }
}
```

For custom configurations (migration, storage location):

```swift
@main
struct MyApp: App {
    let container: ModelContainer

    init() {
        let schema = Schema([UserProfile.self, Post.self, Tag.self])
        let config = ModelConfiguration(
            "MyApp",
            schema: schema,
            isStoredInMemoryOnly: false,
            groupContainer: .automatic,
            cloudKitDatabase: .private("iCloud.com.company.myapp")
        )

        do {
            container = try ModelContainer(
                for: schema,
                migrationPlan: MyAppMigrationPlan.self,
                configurations: [config]
            )
        } catch {
            fatalError("Failed to configure SwiftData: \(error)")
        }
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(container)
    }
}
```

### Schema Migration

Always define a migration plan when evolving the schema:

```swift
enum MyAppMigrationPlan: SchemaMigrationPlan {
    static var schemas: [any VersionedSchema.Type] {
        [SchemaV1.self, SchemaV2.self]
    }

    static var stages: [MigrationStage] {
        [migrateV1toV2]
    }

    static let migrateV1toV2 = MigrationStage.lightweight(
        fromVersion: SchemaV1.self,
        toVersion: SchemaV2.self
    )
}

enum SchemaV1: VersionedSchema {
    static var versionIdentifier: Schema.Version = Schema.Version(1, 0, 0)
    static var models: [any PersistentModel.Type] {
        [UserProfile.self]
    }

    @Model
    final class UserProfile {
        var name: String
        var email: String
        init(name: String, email: String) {
            self.name = name
            self.email = email
        }
    }
}

enum SchemaV2: VersionedSchema {
    static var versionIdentifier: Schema.Version = Schema.Version(2, 0, 0)
    static var models: [any PersistentModel.Type] {
        [UserProfile.self]
    }

    @Model
    final class UserProfile {
        var name: String
        var email: String
        var avatarURL: URL?   // Added in V2
        init(name: String, email: String, avatarURL: URL? = nil) {
            self.name = name
            self.email = email
            self.avatarURL = avatarURL
        }
    }
}
```

### Querying with @Query

Use `@Query` in views for automatic observation:

```swift
struct PostListView: View {
    @Query(
        filter: #Predicate<Post> { $0.publishedAt != nil },
        sort: [SortDescriptor(\Post.publishedAt, order: .reverse)],
        animation: .default
    )
    private var posts: [Post]

    var body: some View {
        List(posts) { post in
            PostRow(post: post)
        }
    }
}
```

### Dynamic Queries

For queries that depend on runtime values, use `@Query` with an `init`:

```swift
struct FilteredPostsView: View {
    @Query private var posts: [Post]

    init(authorName: String) {
        let predicate = #Predicate<Post> { post in
            post.author?.name == authorName
        }
        _posts = Query(filter: predicate, sort: \.publishedAt)
    }

    var body: some View {
        List(posts) { post in
            PostRow(post: post)
        }
    }
}
```

### ModelContext Operations

For mutations, use `modelContext` from the environment:

```swift
struct NewPostView: View {
    @Environment(\.modelContext) private var modelContext
    @Environment(\.dismiss) private var dismiss

    @State private var title = ""
    @State private var body = ""

    func save() {
        let post = Post(title: title, body: body, author: currentUser)
        modelContext.insert(post)
        // SwiftData auto-saves; explicit save only if needed:
        // try? modelContext.save()
        dismiss()
    }
}
```

### SwiftData in ViewModels

When using SwiftData from a ViewModel (outside views), pass the `ModelContext`:

```swift
@Observable
final class PostsViewModel {
    var posts: [Post] = []

    private let modelContext: ModelContext

    init(modelContext: ModelContext) {
        self.modelContext = modelContext
    }

    func fetchPosts() throws {
        let descriptor = FetchDescriptor<Post>(
            predicate: #Predicate { $0.publishedAt != nil },
            sortBy: [SortDescriptor(\.publishedAt, order: .reverse)]
        )
        posts = try modelContext.fetch(descriptor)
    }

    func deletePost(_ post: Post) {
        modelContext.delete(post)
    }
}
```

---

## 6. Testing Patterns

### Test Pyramid (SwiftUI-specific)

```
        ╱╲
       ╱  ╲          E2E (XCUITest) — critical user flows only
      ╱    ╲
     ╱──────╲
    ╱        ╲        Snapshot Tests (swift-snapshot-testing)
   ╱          ╲       Visual regression for key screens
  ╱────────────╲
 ╱              ╲      Integration Tests (Swift Testing + SwiftData)
╱                ╲     ViewModel + Service through persistence, network mocks
╱──────────────────╲
╱                    ╲   Unit Tests (Swift Testing)
╱                      ╲  Pure functions, ViewModels, model logic
╱────────────────────────╲
```

### Swift Testing Framework (Primary)

Use Swift Testing for all new tests. It is the default test framework.

```swift
import Testing
@testable import MyApp

struct LoginViewModelTests {
    let mockAuthService = MockAuthService()

    @Test("Login succeeds with valid credentials")
    func loginSuccess() async throws {
        mockAuthService.loginResult = .success(User(name: "Test"))
        let viewModel = LoginViewModel(authService: mockAuthService)

        viewModel.email = "test@example.com"
        viewModel.password = "password123"
        await viewModel.login()

        #expect(viewModel.isAuthenticated == true)
        #expect(viewModel.errorMessage == nil)
    }

    @Test("Login fails with invalid credentials")
    func loginFailure() async throws {
        mockAuthService.loginResult = .failure(AuthError.invalidCredentials)
        let viewModel = LoginViewModel(authService: mockAuthService)

        viewModel.email = "test@example.com"
        viewModel.password = "wrong"
        await viewModel.login()

        #expect(viewModel.isAuthenticated == false)
        #expect(viewModel.errorMessage == "Invalid email or password")
    }

    @Test("Email validation rejects invalid emails", arguments: [
        "",
        "not-an-email",
        "missing@tld",
        "@no-local.com"
    ])
    func emailValidation(email: String) {
        let viewModel = LoginViewModel(authService: mockAuthService)
        viewModel.email = email

        #expect(viewModel.isEmailValid == false)
    }
}
```

### Swift Testing Features

**Parameterized tests** — test multiple inputs with a single test method:

```swift
@Test("Price formatting", arguments: [
    (1000, "$10.00"),
    (999, "$9.99"),
    (50, "$0.50"),
    (0, "$0.00"),
])
func priceFormatting(cents: Int, expected: String) {
    #expect(Price(cents: cents).formatted == expected)
}
```

**Tags for test organization:**

```swift
extension Tag {
    @Tag static var networking: Self
    @Tag static var persistence: Self
    @Tag static var viewModel: Self
}

@Test("Fetch items from API", .tags(.networking, .viewModel))
func fetchItems() async throws {
    // ...
}
```

**Traits for conditional execution:**

```swift
@Test("iCloud sync works", .enabled(if: ProcessInfo.processInfo.environment["CI"] == nil))
func iCloudSync() async throws {
    // Skip on CI where iCloud is unavailable
}

@Test("Known crash on iOS 17.0", .bug("https://github.com/org/repo/issues/42"))
func knownBug() {
    // Documents known issue
}
```

**Suites for grouping:**

```swift
@Suite("Dashboard ViewModel")
struct DashboardViewModelTests {
    let viewModel: DashboardViewModel
    let mockClient: MockAPIClient

    init() {
        mockClient = MockAPIClient()
        viewModel = DashboardViewModel(apiClient: mockClient)
    }

    @Test func loadItems() async throws { /* ... */ }
    @Test func refreshItems() async throws { /* ... */ }
    @Test func deleteItem() async throws { /* ... */ }
}
```

### XCTest (Legacy and UI Tests)

XCTest is still used for:
1. **XCUITest** (UI automation) — Swift Testing does not support UI tests
2. **Performance tests** — `measure {}` blocks
3. **Legacy code** — existing XCTest suites that haven't been migrated

```swift
import XCTest
@testable import MyApp

final class DashboardViewModelXCTests: XCTestCase {
    var viewModel: DashboardViewModel!
    var mockClient: MockAPIClient!

    override func setUp() {
        super.setUp()
        mockClient = MockAPIClient()
        viewModel = DashboardViewModel(apiClient: mockClient)
    }

    override func tearDown() {
        viewModel = nil
        mockClient = nil
        super.tearDown()
    }

    func testLoadItemsPerformance() {
        mockClient.fetchItemsResult = .success(Array(repeating: Item.sample, count: 1000))

        measure {
            let expectation = expectation(description: "load")
            Task {
                await viewModel.loadItems()
                expectation.fulfill()
            }
            wait(for: [expectation], timeout: 5)
        }
    }
}
```

### Protocol-Based Mocking

All external dependencies use protocols for testability:

```swift
// Protocol
protocol APIClientProtocol: Sendable {
    func fetchItems() async throws -> [Item]
    func createItem(_ item: NewItem) async throws -> Item
    func deleteItem(id: String) async throws
}

// Production implementation
final class HTTPClient: APIClientProtocol {
    static let shared = HTTPClient()

    func fetchItems() async throws -> [Item] {
        let (data, _) = try await URLSession.shared.data(from: Endpoint.items.url)
        return try JSONDecoder().decode([Item].self, from: data)
    }
    // ...
}

// Test mock
final class MockAPIClient: APIClientProtocol, @unchecked Sendable {
    var fetchItemsResult: Result<[Item], Error> = .success([])
    var createItemResult: Result<Item, Error> = .success(Item.sample)
    var deleteItemCalled = false
    var deleteItemID: String?

    func fetchItems() async throws -> [Item] {
        try fetchItemsResult.get()
    }

    func createItem(_ item: NewItem) async throws -> Item {
        try createItemResult.get()
    }

    func deleteItem(id: String) async throws {
        deleteItemCalled = true
        deleteItemID = id
    }
}
```

### SwiftData Testing

Use in-memory containers for fast, isolated tests:

```swift
@Suite("UserProfile persistence")
struct UserProfileTests {
    @Test func createAndFetch() throws {
        let config = ModelConfiguration(isStoredInMemoryOnly: true)
        let container = try ModelContainer(
            for: UserProfile.self,
            configurations: config
        )
        let context = container.mainContext

        let profile = UserProfile(name: "Alice", email: "alice@example.com")
        context.insert(profile)
        try context.save()

        let descriptor = FetchDescriptor<UserProfile>(
            predicate: #Predicate { $0.email == "alice@example.com" }
        )
        let fetched = try context.fetch(descriptor)

        #expect(fetched.count == 1)
        #expect(fetched.first?.name == "Alice")
    }

    @Test func cascadeDelete() throws {
        let config = ModelConfiguration(isStoredInMemoryOnly: true)
        let container = try ModelContainer(
            for: UserProfile.self, Post.self,
            configurations: config
        )
        let context = container.mainContext

        let author = UserProfile(name: "Bob", email: "bob@example.com")
        let post = Post(title: "Hello", body: "World", author: author)
        context.insert(author)
        context.insert(post)
        try context.save()

        context.delete(author)
        try context.save()

        let posts = try context.fetch(FetchDescriptor<Post>())
        #expect(posts.isEmpty, "Cascade delete should remove posts")
    }
}
```

### XCUITest (End-to-End)

Use the Page Object pattern for readable, maintainable UI tests:

```swift
// Page Object
struct LoginPage {
    let app: XCUIApplication

    var emailField: XCUIElement { app.textFields["email_field"] }
    var passwordField: XCUIElement { app.secureTextFields["password_field"] }
    var loginButton: XCUIElement { app.buttons["login_button"] }
    var errorLabel: XCUIElement { app.staticTexts["error_label"] }

    @discardableResult
    func typeEmail(_ email: String) -> Self {
        emailField.tap()
        emailField.typeText(email)
        return self
    }

    @discardableResult
    func typePassword(_ password: String) -> Self {
        passwordField.tap()
        passwordField.typeText(password)
        return self
    }

    @discardableResult
    func tapLogin() -> Self {
        loginButton.tap()
        return self
    }
}

// UI Test
final class LoginFlowTests: XCTestCase {
    let app = XCUIApplication()

    override func setUp() {
        continueAfterFailure = false
        app.launchArguments = ["--ui-testing"]
        app.launch()
    }

    func testSuccessfulLogin() {
        let login = LoginPage(app: app)

        login
            .typeEmail("test@example.com")
            .typePassword("password123")
            .tapLogin()

        XCTAssertTrue(app.tabBars.firstMatch.waitForExistence(timeout: 5))
    }
}
```

### Test Configuration

```swift
// In scheme settings or via test plan:
// - Enable code coverage for the app target
// - Enable parallel testing for unit tests
// - Disable parallel testing for UI tests

// Launch arguments for test environments
extension XCUIApplication {
    func launchForTesting() {
        launchArguments += [
            "--ui-testing",
            "--reset-state",
            "--disable-animations"
        ]
        launchEnvironment["API_BASE_URL"] = "http://localhost:8080"
        launch()
    }
}
```

---

## 7. Async/Await and Concurrency

### Task Lifecycle in SwiftUI

Use `.task` modifier for async work tied to view lifecycle:

```swift
struct ItemListView: View {
    @State private var viewModel = ItemListViewModel()

    var body: some View {
        List(viewModel.items) { item in
            ItemRow(item: item)
        }
        .task {
            // Cancelled automatically when view disappears
            await viewModel.loadItems()
        }
        .refreshable {
            await viewModel.refresh()
        }
        .task(id: viewModel.selectedCategory) {
            // Re-runs when selectedCategory changes
            await viewModel.loadItems(category: viewModel.selectedCategory)
        }
    }
}
```

### Structured Concurrency

Use `TaskGroup` for parallel operations:

```swift
func loadDashboard() async throws -> Dashboard {
    async let profile = apiClient.fetchProfile()
    async let items = apiClient.fetchItems()
    async let notifications = apiClient.fetchNotifications()

    return try await Dashboard(
        profile: profile,
        items: items,
        notifications: notifications
    )
}
```

For dynamic numbers of concurrent tasks:

```swift
func downloadImages(urls: [URL]) async throws -> [URL: Data] {
    try await withThrowingTaskGroup(of: (URL, Data).self) { group in
        for url in urls {
            group.addTask {
                let (data, _) = try await URLSession.shared.data(from: url)
                return (url, data)
            }
        }

        var results: [URL: Data] = [:]
        for try await (url, data) in group {
            results[url] = data
        }
        return results
    }
}
```

### Actor Isolation

Use actors for shared mutable state:

```swift
actor ImageCache {
    private var cache: [URL: Data] = [:]
    private var inFlight: [URL: Task<Data, Error>] = [:]

    func image(for url: URL) async throws -> Data {
        if let cached = cache[url] {
            return cached
        }

        if let existingTask = inFlight[url] {
            return try await existingTask.value
        }

        let task = Task {
            let (data, _) = try await URLSession.shared.data(from: url)
            return data
        }

        inFlight[url] = task

        do {
            let data = try await task.value
            cache[url] = data
            inFlight[url] = nil
            return data
        } catch {
            inFlight[url] = nil
            throw error
        }
    }

    func clear() {
        cache.removeAll()
    }
}
```

### MainActor for UI Updates

ViewModels that update UI must be `@MainActor`:

```swift
@Observable
@MainActor
final class DashboardViewModel {
    var items: [Item] = []
    var isLoading = false

    private let apiClient: APIClientProtocol

    init(apiClient: APIClientProtocol = HTTPClient.shared) {
        self.apiClient = apiClient
    }

    func loadItems() async {
        isLoading = true
        defer { isLoading = false }

        do {
            items = try await apiClient.fetchItems()
        } catch {
            // Handle error
        }
    }
}
```

### Cancellation

Always check for cancellation in long-running tasks:

```swift
func processItems(_ items: [Item]) async throws -> [ProcessedItem] {
    var results: [ProcessedItem] = []

    for item in items {
        try Task.checkCancellation()
        let processed = try await process(item)
        results.append(processed)
    }

    return results
}
```

### AsyncSequence for Streaming

```swift
func observeChanges() -> AsyncStream<[Item]> {
    AsyncStream { continuation in
        let observer = NotificationCenter.default.addObserver(
            forName: .itemsDidChange,
            object: nil,
            queue: .main
        ) { notification in
            if let items = notification.userInfo?["items"] as? [Item] {
                continuation.yield(items)
            }
        }

        continuation.onTermination = { _ in
            NotificationCenter.default.removeObserver(observer)
        }
    }
}

// Usage in a view
.task {
    for await items in viewModel.observeChanges() {
        self.items = items
    }
}
```

---

## 8. UIKit Interop

### UIViewRepresentable

Wrap UIKit views for use in SwiftUI:

```swift
struct MapView: UIViewRepresentable {
    let region: MKCoordinateRegion
    let annotations: [MKPointAnnotation]

    func makeUIView(context: Context) -> MKMapView {
        let mapView = MKMapView()
        mapView.delegate = context.coordinator
        return mapView
    }

    func updateUIView(_ mapView: MKMapView, context: Context) {
        mapView.setRegion(region, animated: true)
        mapView.removeAnnotations(mapView.annotations)
        mapView.addAnnotations(annotations)
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    final class Coordinator: NSObject, MKMapViewDelegate {
        let parent: MapView

        init(_ parent: MapView) {
            self.parent = parent
        }

        func mapView(_ mapView: MKMapView, viewFor annotation: MKAnnotation) -> MKAnnotationView? {
            let view = MKMarkerAnnotationView(
                annotation: annotation,
                reuseIdentifier: "marker"
            )
            view.canShowCallout = true
            return view
        }
    }
}
```

### UIViewControllerRepresentable

Wrap UIKit view controllers:

```swift
struct DocumentPicker: UIViewControllerRepresentable {
    @Binding var selectedURL: URL?

    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        let picker = UIDocumentPickerViewController(forOpeningContentTypes: [.pdf, .plainText])
        picker.delegate = context.coordinator
        picker.allowsMultipleSelection = false
        return picker
    }

    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    final class Coordinator: NSObject, UIDocumentPickerDelegate {
        let parent: DocumentPicker

        init(_ parent: DocumentPicker) {
            self.parent = parent
        }

        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            parent.selectedURL = urls.first
        }
    }
}
```

### Hosting SwiftUI in UIKit

Embed SwiftUI views within existing UIKit apps:

```swift
// In a UIViewController
let settingsView = SettingsView()
let hostingController = UIHostingController(rootView: settingsView)

addChild(hostingController)
view.addSubview(hostingController.view)
hostingController.view.translatesAutoresizingMaskIntoConstraints = false

NSLayoutConstraint.activate([
    hostingController.view.topAnchor.constraint(equalTo: view.topAnchor),
    hostingController.view.bottomAnchor.constraint(equalTo: view.bottomAnchor),
    hostingController.view.leadingAnchor.constraint(equalTo: view.leadingAnchor),
    hostingController.view.trailingAnchor.constraint(equalTo: view.trailingAnchor),
])

hostingController.didMove(toParent: self)
```

### UIAppDelegate Adapter

For push notifications, deep links, and other UIKit lifecycle events:

```swift
@main
struct MyApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

final class AppDelegate: NSObject, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        // Configure push notifications, analytics, etc.
        UNUserNotificationCenter.current().delegate = self
        return true
    }

    func application(
        _ application: UIApplication,
        didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data
    ) {
        let token = deviceToken.map { String(format: "%02.2hhx", $0) }.joined()
        // Send token to server
    }
}

extension AppDelegate: UNUserNotificationCenterDelegate {
    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification
    ) async -> UNNotificationPresentationOptions {
        [.banner, .sound, .badge]
    }
}
```

### When to Use UIKit Interop

| Use Case | Approach |
|---|---|
| Camera / photo picker | `PHPickerViewController` via `UIViewControllerRepresentable` |
| Map views | `MKMapView` via `UIViewRepresentable` (until MapKit for SwiftUI is sufficient) |
| Document picker | `UIDocumentPickerViewController` via `UIViewControllerRepresentable` |
| Web views | `WKWebView` via `UIViewRepresentable` |
| Text view with advanced editing | `UITextView` via `UIViewRepresentable` |
| Push notification setup | `UIApplicationDelegateAdaptor` |
| Third-party UIKit SDKs | `UIViewRepresentable` or `UIViewControllerRepresentable` |
| Custom gesture recognizers | Prefer SwiftUI gestures; fall back to `UIViewRepresentable` |

**Rule:** If SwiftUI provides a native equivalent that meets requirements, use it. Only bridge to UIKit when SwiftUI's API is insufficient.

---

## 9. Networking Layer

### Protocol-Based API Client

```swift
protocol APIClientProtocol: Sendable {
    func request<T: Decodable>(_ endpoint: any Endpoint) async throws -> T
    func upload(data: Data, to endpoint: any Endpoint) async throws -> UploadResponse
}

protocol Endpoint: Sendable {
    var baseURL: URL { get }
    var path: String { get }
    var method: HTTPMethod { get }
    var headers: [String: String] { get }
    var queryItems: [URLQueryItem]? { get }
    var body: Data? { get }
}

enum HTTPMethod: String, Sendable {
    case get = "GET"
    case post = "POST"
    case put = "PUT"
    case patch = "PATCH"
    case delete = "DELETE"
}
```

### Production HTTP Client

```swift
final class HTTPClient: APIClientProtocol {
    static let shared = HTTPClient()

    private let session: URLSession
    private let decoder: JSONDecoder

    init(session: URLSession = .shared) {
        self.session = session
        self.decoder = JSONDecoder()
        self.decoder.dateDecodingStrategy = .iso8601
        self.decoder.keyDecodingStrategy = .convertFromSnakeCase
    }

    func request<T: Decodable>(_ endpoint: any Endpoint) async throws -> T {
        var urlRequest = try buildRequest(endpoint)

        let (data, response) = try await session.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw APIError.httpError(
                statusCode: httpResponse.statusCode,
                data: data
            )
        }

        return try decoder.decode(T.self, from: data)
    }

    private func buildRequest(_ endpoint: any Endpoint) throws -> URLRequest {
        var components = URLComponents(url: endpoint.baseURL.appendingPathComponent(endpoint.path), resolvingAgainstBaseURL: true)
        components?.queryItems = endpoint.queryItems

        guard let url = components?.url else {
            throw APIError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = endpoint.method.rawValue
        request.httpBody = endpoint.body

        for (key, value) in endpoint.headers {
            request.setValue(value, forHTTPHeaderField: key)
        }

        return request
    }
}
```

### Typed Error Hierarchy

```swift
enum APIError: LocalizedError, Sendable {
    case invalidURL
    case invalidResponse
    case httpError(statusCode: Int, data: Data)
    case decodingError(Error)
    case networkError(Error)
    case unauthorized
    case rateLimited(retryAfter: TimeInterval?)

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            "The request URL is invalid."
        case .invalidResponse:
            "Received an invalid response from the server."
        case .httpError(let statusCode, _):
            "Server returned error \(statusCode)."
        case .decodingError:
            "Failed to process the server response."
        case .networkError:
            "A network error occurred. Check your connection."
        case .unauthorized:
            "Your session has expired. Please log in again."
        case .rateLimited:
            "Too many requests. Please try again later."
        }
    }
}
```

### Endpoint Pattern

```swift
enum ItemsEndpoint: Endpoint {
    case list(page: Int, perPage: Int)
    case detail(id: String)
    case create(NewItem)
    case update(id: String, UpdateItem)
    case delete(id: String)

    var baseURL: URL { URL(string: AppEnvironment.current.apiBaseURL)! }

    var path: String {
        switch self {
        case .list: "/api/v1/items"
        case .detail(let id): "/api/v1/items/\(id)"
        case .create: "/api/v1/items"
        case .update(let id, _): "/api/v1/items/\(id)"
        case .delete(let id): "/api/v1/items/\(id)"
        }
    }

    var method: HTTPMethod {
        switch self {
        case .list, .detail: .get
        case .create: .post
        case .update: .put
        case .delete: .delete
        }
    }

    var headers: [String: String] {
        ["Content-Type": "application/json", "Accept": "application/json"]
    }

    var queryItems: [URLQueryItem]? {
        switch self {
        case .list(let page, let perPage):
            [URLQueryItem(name: "page", value: "\(page)"),
             URLQueryItem(name: "per_page", value: "\(perPage)")]
        default:
            nil
        }
    }

    var body: Data? {
        switch self {
        case .create(let item):
            try? JSONEncoder().encode(item)
        case .update(_, let item):
            try? JSONEncoder().encode(item)
        default:
            nil
        }
    }
}
```

### Authentication Token Management

```swift
actor TokenManager {
    private var accessToken: String?
    private var refreshToken: String?
    private var refreshTask: Task<String, Error>?

    func setTokens(access: String, refresh: String) {
        accessToken = access
        refreshToken = refresh
        try? KeychainHelper.save(key: "access_token", value: access)
        try? KeychainHelper.save(key: "refresh_token", value: refresh)
    }

    func validAccessToken() async throws -> String {
        if let token = accessToken, !isExpired(token) {
            return token
        }

        // Coalesce concurrent refresh attempts
        if let existingTask = refreshTask {
            return try await existingTask.value
        }

        let task = Task { () -> String in
            defer { refreshTask = nil }
            guard let refresh = refreshToken else {
                throw APIError.unauthorized
            }
            let response: TokenResponse = try await HTTPClient.shared.request(
                AuthEndpoint.refresh(token: refresh)
            )
            setTokens(access: response.accessToken, refresh: response.refreshToken)
            return response.accessToken
        }

        refreshTask = task
        return try await task.value
    }

    func clearTokens() {
        accessToken = nil
        refreshToken = nil
        try? KeychainHelper.delete(key: "access_token")
        try? KeychainHelper.delete(key: "refresh_token")
    }

    private func isExpired(_ token: String) -> Bool {
        // Decode JWT and check expiration
        guard let payload = decodeJWTPayload(token),
              let exp = payload["exp"] as? TimeInterval else {
            return true
        }
        return Date(timeIntervalSince1970: exp) < Date.now.addingTimeInterval(60) // 60s buffer
    }
}
```

---

## 10. Accessibility

### VoiceOver Support

Every interactive element must have meaningful accessibility labels:

```swift
struct ItemRow: View {
    let item: Item

    var body: some View {
        HStack {
            Image(systemName: item.icon)
                .accessibilityHidden(true) // Decorative

            VStack(alignment: .leading) {
                Text(item.title)
                Text(item.subtitle)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            if item.isFavorite {
                Image(systemName: "heart.fill")
                    .foregroundStyle(.red)
                    .accessibilityHidden(true) // Included in parent label
            }
        }
        .accessibilityElement(children: .combine)
        .accessibilityLabel("\(item.title), \(item.subtitle)\(item.isFavorite ? ", favorited" : "")")
        .accessibilityHint("Double tap to view details")
        .accessibilityAddTraits(.isButton)
    }
}
```

### Dynamic Type

Support all Dynamic Type sizes:

```swift
struct ProfileHeader: View {
    let user: User
    @Environment(\.dynamicTypeSize) private var dynamicTypeSize

    var body: some View {
        if dynamicTypeSize.isAccessibilitySize {
            // Vertical layout for accessibility sizes
            VStack(alignment: .center) {
                avatar
                userInfo
            }
        } else {
            // Horizontal layout for standard sizes
            HStack {
                avatar
                userInfo
            }
        }
    }

    private var avatar: some View {
        AsyncImage(url: user.avatarURL) { image in
            image.resizable().scaledToFill()
        } placeholder: {
            ProgressView()
        }
        .frame(width: 60, height: 60)
        .clipShape(Circle())
    }

    private var userInfo: some View {
        VStack(alignment: .leading) {
            Text(user.name)
                .font(.headline)
            Text(user.email)
                .font(.subheadline)
                .foregroundStyle(.secondary)
        }
    }
}
```

### Color Contrast

Never rely on color alone to convey information:

```swift
// Bad — color is the only differentiator
Circle()
    .fill(status == .active ? .green : .red)

// Good — shape + color + label
HStack(spacing: 4) {
    Image(systemName: status == .active ? "checkmark.circle.fill" : "xmark.circle.fill")
        .foregroundStyle(status == .active ? .green : .red)
    Text(status == .active ? "Active" : "Inactive")
        .font(.caption)
}
.accessibilityElement(children: .combine)
.accessibilityLabel(status == .active ? "Status: Active" : "Status: Inactive")
```

### Accessibility Audit Checklist

| Requirement | Implementation |
|---|---|
| VoiceOver labels | `.accessibilityLabel()` on all interactive elements |
| VoiceOver hints | `.accessibilityHint()` for non-obvious actions |
| Traits | `.accessibilityAddTraits()` — `.isButton`, `.isHeader`, `.isLink` |
| Dynamic Type | Test at all sizes, use `AX` layout for accessibility sizes |
| Color contrast | 4.5:1 minimum for normal text, 3:1 for large text (WCAG AA) |
| Reduce Motion | Respect `.accessibilityReduceMotion` preference |
| Reduce Transparency | Respect `.accessibilityReduceTransparency` preference |
| Touch targets | Minimum 44x44 points |
| Focus order | Logical reading order, use `.accessibilitySortPriority()` if needed |
| Grouping | `.accessibilityElement(children: .combine)` for related content |

### Respecting System Preferences

```swift
struct AnimatedView: View {
    @Environment(\.accessibilityReduceMotion) private var reduceMotion
    @Environment(\.accessibilityReduceTransparency) private var reduceTransparency
    @Environment(\.colorSchemeContrast) private var contrast

    var body: some View {
        RoundedRectangle(cornerRadius: 12)
            .fill(reduceTransparency ? .background : .ultraThinMaterial)
            .animation(reduceMotion ? nil : .spring(), value: isExpanded)
            .overlay {
                Text("Content")
                    .foregroundStyle(contrast == .increased ? .primary : .secondary)
            }
    }
}
```

---

## 11. Development Workflow

### Feature Development Cycle (SwiftUI-specific)

```
1. Write acceptance criteria (ticket / build plan)
2. Design test levels (unit / integration / UI)
3. Write failing tests (Swift Testing for logic, XCUITest for flows)
4. Write Model / ViewModel code
5. Write SwiftUI View
6. Run: swift test (or Cmd+U in Xcode)
7. Run: swift-format lint --recursive Sources/
8. Refactor while green
9. Run: swiftlint
10. Preview in Xcode — test Dynamic Type, Dark Mode, landscape
11. Test on physical device
```

### Common Commands

```bash
# Build and test
swift build                              # Build (SPM project)
swift test                               # Run all tests (SPM project)
xcodebuild test -scheme MyApp \
    -destination 'platform=iOS Simulator,name=iPhone 16' \
    -resultBundlePath TestResults.xcresult    # Xcode project tests

# Formatting and linting
swift-format lint --recursive Sources/   # Check formatting
swift-format format --recursive Sources/ --in-place  # Auto-format
swiftlint                                # Lint
swiftlint --fix                          # Auto-fix lint issues

# Xcode
xcodebuild -list                         # List schemes and targets
xcodebuild clean build \
    -scheme MyApp \
    -destination generic/platform=iOS    # Clean build for iOS

# Archive and export
xcodebuild archive \
    -scheme MyApp \
    -archivePath build/MyApp.xcarchive \
    -destination generic/platform=iOS
xcodebuild -exportArchive \
    -archivePath build/MyApp.xcarchive \
    -exportPath build/export \
    -exportOptionsPlist ExportOptions.plist

# SwiftData
# No CLI migrations — schema changes are automatic via @Model
# For debugging: Use Xcode's SwiftData debugging tools

# Dependency management
swift package resolve                    # Resolve SPM dependencies
swift package update                     # Update to latest compatible versions
swift package show-dependencies          # Show dependency tree
```

### Xcode Schemes and Build Configurations

```
Schemes:
├── MyApp              # Development build
├── MyApp-Staging      # Staging environment
└── MyApp-Production   # Production build (App Store)

Build Configurations:
├── Debug              # Local development
├── Staging            # Staging server, debug symbols
└── Release            # Production, optimized, no debug symbols
```

Use `.xcconfig` files for per-environment settings:

```
// Debug.xcconfig
API_BASE_URL = http:/$()/localhost:8080
SWIFT_ACTIVE_COMPILATION_CONDITIONS = DEBUG
ENABLE_TESTABILITY = YES

// Staging.xcconfig
API_BASE_URL = https:/$()/staging-api.example.com
SWIFT_ACTIVE_COMPILATION_CONDITIONS = STAGING

// Release.xcconfig
API_BASE_URL = https:/$()/api.example.com
SWIFT_ACTIVE_COMPILATION_CONDITIONS = RELEASE
SWIFT_OPTIMIZATION_LEVEL = -O
```

Access in code:

```swift
enum AppEnvironment {
    static var apiBaseURL: String {
        Bundle.main.infoDictionary?["API_BASE_URL"] as? String ?? "http://localhost:8080"
    }

    static var isDebug: Bool {
        #if DEBUG
        true
        #else
        false
        #endif
    }
}
```

---

## 12. App Store Deployment

### Pre-Submission Checklist

| Item | Requirement |
|---|---|
| App icon | All sizes in `Assets.xcassets/AppIcon` |
| Launch screen | Storyboard or `Info.plist`-based configuration |
| Privacy manifest | `PrivacyInfo.xcprivacy` with required API declarations |
| App Transport Security | HTTPS only; exceptions must be justified |
| Version / build | Semantic versioning; build number increments monotonically |
| Screenshots | Required sizes for all device families |
| Privacy policy URL | Required for all apps |
| App review notes | Explain test credentials, special configuration |
| Entitlements | Only request entitlements the app actually uses |
| Code signing | Automatic signing or manual with valid profiles |

### Privacy Manifest (Required since Spring 2024)

```xml
<!-- PrivacyInfo.xcprivacy -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>NSPrivacyTracking</key>
    <false/>
    <key>NSPrivacyTrackingDomains</key>
    <array/>
    <key>NSPrivacyCollectedDataTypes</key>
    <array>
        <dict>
            <key>NSPrivacyCollectedDataType</key>
            <string>NSPrivacyCollectedDataTypeEmailAddress</string>
            <key>NSPrivacyCollectedDataTypeLinked</key>
            <true/>
            <key>NSPrivacyCollectedDataTypeTracking</key>
            <false/>
            <key>NSPrivacyCollectedDataTypePurposes</key>
            <array>
                <string>NSPrivacyCollectedDataTypePurposeAppFunctionality</string>
            </array>
        </dict>
    </array>
    <key>NSPrivacyAccessedAPITypes</key>
    <array>
        <dict>
            <key>NSPrivacyAccessedAPIType</key>
            <string>NSPrivacyAccessedAPICategoryUserDefaults</string>
            <key>NSPrivacyAccessedAPITypeReasons</key>
            <array>
                <string>CA92.1</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
```

### CI/CD Pipeline (GitHub Actions + Fastlane)

```yaml
name: CI/CD

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: macos-15
    steps:
      - uses: actions/checkout@v4
      - name: Select Xcode
        run: sudo xcode-select -s /Applications/Xcode_16.3.app
      - name: Resolve packages
        run: xcodebuild -resolvePackageDependencies -scheme MyApp
      - name: Build and test
        run: |
          xcodebuild test \
            -scheme MyApp \
            -destination 'platform=iOS Simulator,name=iPhone 16' \
            -resultBundlePath TestResults.xcresult \
            CODE_SIGN_IDENTITY="" \
            CODE_SIGNING_REQUIRED=NO
      - name: SwiftLint
        run: swiftlint --strict
      - name: swift-format check
        run: swift-format lint --recursive Sources/ --strict

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: macos-15
    steps:
      - uses: actions/checkout@v4
      - name: Install Fastlane
        run: bundle install
      - name: Deploy to TestFlight
        run: bundle exec fastlane beta
        env:
          APP_STORE_CONNECT_API_KEY_ID: ${{ secrets.ASC_KEY_ID }}
          APP_STORE_CONNECT_ISSUER_ID: ${{ secrets.ASC_ISSUER_ID }}
          APP_STORE_CONNECT_API_KEY: ${{ secrets.ASC_API_KEY }}
          MATCH_PASSWORD: ${{ secrets.MATCH_PASSWORD }}
```

### Fastlane Configuration

```ruby
# Fastfile
default_platform(:ios)

platform :ios do
  desc "Run tests"
  lane :test do
    run_tests(
      scheme: "MyApp",
      devices: ["iPhone 16"],
      code_coverage: true
    )
  end

  desc "Deploy to TestFlight"
  lane :beta do
    setup_ci
    match(type: "appstore", readonly: true)
    increment_build_number(
      build_number: latest_testflight_build_number + 1
    )
    build_app(
      scheme: "MyApp-Production",
      export_method: "app-store"
    )
    upload_to_testflight(
      skip_waiting_for_build_processing: true
    )
  end

  desc "Deploy to App Store"
  lane :release do
    setup_ci
    match(type: "appstore", readonly: true)
    build_app(
      scheme: "MyApp-Production",
      export_method: "app-store"
    )
    upload_to_app_store(
      submit_for_review: true,
      automatic_release: false,
      precheck_include_in_app_purchases: false
    )
  end
end
```

### Version Management

```swift
// Use marketing version (CFBundleShortVersionString) for user-facing version
// Use build number (CFBundleVersion) for internal tracking

// Xcode project settings:
// MARKETING_VERSION = 1.2.0      (semantic version)
// CURRENT_PROJECT_VERSION = 42    (monotonically increasing integer)
```

Convention: bump version according to semver. Build number increments on every CI build.

---

## 13. Security

### Keychain for Sensitive Data

Never store tokens, passwords, or secrets in `UserDefaults`. Always use Keychain:

```swift
enum KeychainHelper {
    static func save(key: String, value: String) throws {
        let data = Data(value.utf8)

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
        ]

        SecItemDelete(query as CFDictionary) // Remove existing

        let status = SecItemAdd(query as CFDictionary, nil)
        guard status == errSecSuccess else {
            throw KeychainError.saveFailed(status)
        }
    }

    static func load(key: String) throws -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        guard status == errSecSuccess, let data = result as? Data else {
            if status == errSecItemNotFound { return nil }
            throw KeychainError.loadFailed(status)
        }

        return String(data: data, encoding: .utf8)
    }

    static func delete(key: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key
        ]

        let status = SecItemDelete(query as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.deleteFailed(status)
        }
    }
}

enum KeychainError: Error {
    case saveFailed(OSStatus)
    case loadFailed(OSStatus)
    case deleteFailed(OSStatus)
}
```

### App Transport Security

All network requests must use HTTPS. No exceptions without explicit justification in `Info.plist`:

```xml
<!-- Default: ATS is enabled, HTTPS required everywhere -->
<!-- Only add exceptions if absolutely necessary (e.g., local dev server) -->
<key>NSAppTransportSecurity</key>
<dict>
    <key>NSExceptionDomains</key>
    <dict>
        <key>localhost</key>
        <dict>
            <key>NSExceptionAllowsInsecureHTTPLoads</key>
            <true/>
            <!-- Only for Debug builds via #if DEBUG -->
        </dict>
    </dict>
</dict>
```

### Certificate Pinning

For high-security apps, implement certificate pinning:

```swift
final class PinnedURLSessionDelegate: NSObject, URLSessionDelegate {
    private let pinnedHashes: Set<String>

    init(pinnedHashes: Set<String>) {
        self.pinnedHashes = pinnedHashes
    }

    func urlSession(
        _ session: URLSession,
        didReceive challenge: URLAuthenticationChallenge
    ) async -> (URLSession.AuthChallengeDisposition, URLCredential?) {
        guard let serverTrust = challenge.protectionSpace.serverTrust,
              let certificate = SecTrustCopyCertificateChain(serverTrust)?.first else {
            return (.cancelAuthenticationChallenge, nil)
        }

        let serverHash = sha256Hash(of: certificate)

        if pinnedHashes.contains(serverHash) {
            return (.useCredential, URLCredential(trust: serverTrust))
        }

        return (.cancelAuthenticationChallenge, nil)
    }
}
```

### Data Protection

```swift
// File-level encryption — files encrypted when device is locked
try data.write(to: fileURL, options: .completeFileProtection)

// SwiftData: stored in app's sandboxed container, encrypted at rest by iOS
// No additional configuration needed for standard data protection

// UserDefaults: NEVER for sensitive data
// Use for: preferences, feature flags, non-sensitive app state
```

### Security Checklist

| Category | Requirement |
|---|---|
| Storage | Keychain for tokens/secrets, never UserDefaults |
| Network | HTTPS only via ATS, certificate pinning for high-security |
| Auth tokens | Short-lived access + refresh token pattern |
| Biometrics | `LAContext` for local auth, Keychain with `.biometryAny` access control |
| Logging | Never log tokens, passwords, PII in production |
| Clipboard | Clear sensitive data from clipboard after use |
| Screenshots | Use `.privacySensitive()` modifier on sensitive views |
| Jailbreak | Detect and warn (not block) on jailbroken devices |
| Obfuscation | Strip debug symbols in release builds |
| Dependencies | Audit SPM dependencies, pin versions, review updates |

---

## 14. Coverage Enforcement

### Xcode Code Coverage

Enable code coverage in the scheme's Test action (Edit Scheme > Test > Options > Code Coverage).

```bash
# Generate coverage report via xcodebuild
xcodebuild test \
    -scheme MyApp \
    -destination 'platform=iOS Simulator,name=iPhone 16' \
    -enableCodeCoverage YES \
    -resultBundlePath TestResults.xcresult

# Extract coverage percentage
xcrun xccov view --report TestResults.xcresult --json | \
    python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d[\"lineCoverage\"]*100:.1f}%')"
```

### Coverage in CI

```yaml
- name: Check coverage threshold
  run: |
    COVERAGE=$(xcrun xccov view --report TestResults.xcresult --json | \
        python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d[\"lineCoverage\"]*100:.1f}')")
    echo "Coverage: ${COVERAGE}%"
    if (( $(echo "$COVERAGE < 80" | bc -l) )); then
        echo "::error::Coverage ${COVERAGE}% is below 80% threshold"
        exit 1
    fi
```

### What to Test vs What to Skip

| Test | Skip |
|---|---|
| ViewModel logic (all methods) | Generated code (SwiftGen, etc.) |
| Model validation and computed properties | Preview providers |
| Service layer / networking (with mocks) | `@main` app entry point |
| Navigation routing logic | Pure SwiftUI layout (test visually) |
| Error handling paths | Third-party library internals |
| Accessibility labels and traits | Xcode-generated boilerplate |
| SwiftData CRUD operations | Trivial getters/setters |

Target is 100% (per CLAUDE.md core rules) for all testable code. Use `@testable import` for internal access.

---

## 15. Component Design

### Reusable Component Pattern

Build components as generic, configurable SwiftUI views:

```swift
struct AsyncButton<Label: View>: View {
    let action: () async -> Void
    let label: () -> Label

    @State private var isPerforming = false

    init(
        action: @escaping () async -> Void,
        @ViewBuilder label: @escaping () -> Label
    ) {
        self.action = action
        self.label = label
    }

    var body: some View {
        Button {
            isPerforming = true
            Task {
                await action()
                isPerforming = false
            }
        } label: {
            if isPerforming {
                ProgressView()
            } else {
                label()
            }
        }
        .disabled(isPerforming)
    }
}

// Usage
AsyncButton {
    await viewModel.save()
} label: {
    Text("Save")
}
```

### Error State View

```swift
struct ErrorView: View {
    let message: String
    let retryAction: (() async -> Void)?

    var body: some View {
        ContentUnavailableView {
            Label("Something Went Wrong", systemImage: "exclamationmark.triangle")
        } description: {
            Text(message)
        } actions: {
            if let retryAction {
                AsyncButton(action: retryAction) {
                    Text("Try Again")
                }
                .buttonStyle(.bordered)
            }
        }
    }
}
```

### Loading State Pattern

Standardize loading/error/empty/content states:

```swift
enum LoadingState<Value> {
    case idle
    case loading
    case loaded(Value)
    case error(Error)
}

extension LoadingState {
    var isLoading: Bool {
        if case .loading = self { return true }
        return false
    }

    var value: Value? {
        if case .loaded(let value) = self { return value }
        return nil
    }

    var error: Error? {
        if case .error(let error) = self { return error }
        return nil
    }
}

// Usage in ViewModel
@Observable
@MainActor
final class ItemsViewModel {
    var state: LoadingState<[Item]> = .idle

    func load() async {
        state = .loading
        do {
            let items = try await apiClient.fetchItems()
            state = .loaded(items)
        } catch {
            state = .error(error)
        }
    }
}

// Usage in View
struct ItemsView: View {
    @State private var viewModel = ItemsViewModel()

    var body: some View {
        Group {
            switch viewModel.state {
            case .idle, .loading:
                ProgressView()
            case .loaded(let items) where items.isEmpty:
                ContentUnavailableView("No Items", systemImage: "tray")
            case .loaded(let items):
                List(items) { item in ItemRow(item: item) }
            case .error(let error):
                ErrorView(message: error.localizedDescription) {
                    await viewModel.load()
                }
            }
        }
        .task { await viewModel.load() }
    }
}
```

### ViewModifier Pattern

Extract reusable view modifications:

```swift
struct CardModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .padding()
            .background(.background)
            .clipShape(RoundedRectangle(cornerRadius: 12))
            .shadow(color: .black.opacity(0.1), radius: 4, y: 2)
    }
}

extension View {
    func cardStyle() -> some View {
        modifier(CardModifier())
    }
}

// Usage
Text("Hello")
    .cardStyle()
```

### Style Configuration with Environment

```swift
struct BrandStyle {
    let primaryColor: Color
    let cornerRadius: CGFloat
    let spacing: CGFloat

    static let `default` = BrandStyle(
        primaryColor: .blue,
        cornerRadius: 12,
        spacing: 16
    )
}

private struct BrandStyleKey: EnvironmentKey {
    static let defaultValue = BrandStyle.default
}

extension EnvironmentValues {
    var brandStyle: BrandStyle {
        get { self[BrandStyleKey.self] }
        set { self[BrandStyleKey.self] = newValue }
    }
}

// Inject at app root
ContentView()
    .environment(\.brandStyle, .default)

// Consume anywhere
struct StyledButton: View {
    @Environment(\.brandStyle) private var style
    let title: String
    let action: () -> Void

    var body: some View {
        Button(title, action: action)
            .padding()
            .background(style.primaryColor)
            .foregroundStyle(.white)
            .clipShape(RoundedRectangle(cornerRadius: style.cornerRadius))
    }
}
```

---

## 16. Anti-Patterns (SwiftUI-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | Using `ObservableObject` + `@Published` for iOS 17+ targets | Use `@Observable` macro — less boilerplate, automatic tracking, better performance |
| 2 | Putting business logic in SwiftUI views | Extract to `@Observable` ViewModels — views are for layout only |
| 3 | Using `NavigationLink(destination:)` | Use `NavigationStack` + `navigationDestination(for:)` with typed routes |
| 4 | Force unwrapping optionals in production code | Use `guard let`, `if let`, or nil-coalescing. Force unwrap only in tests and previews |
| 5 | Storing tokens or secrets in `UserDefaults` | Use Keychain with `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly` |
| 6 | Using `@StateObject` for iOS 17+ targets | Use `@State` with `@Observable` classes — `@StateObject` is for `ObservableObject` only |
| 7 | Massive views with 200+ lines | Extract subviews, use computed properties, break into `ViewModifier` and components |
| 8 | Using `AnyView` for type erasure | Use `@ViewBuilder`, `Group`, or `some View` return types. `AnyView` defeats diffing |
| 9 | Ignoring `@MainActor` on ViewModels | Annotate ViewModels with `@MainActor` — UI updates must happen on main thread |
| 10 | Network calls directly in views | Always go through a ViewModel or Service layer |
| 11 | Hardcoding strings in views | Use `String(localized:)` or String Catalogs (`Localizable.xcstrings`) for all user-facing text |
| 12 | Using `Timer` for periodic updates | Use `.task` with `AsyncTimerSequence` or `TimelineView` for clock-driven updates |
| 13 | Ignoring `Task` cancellation | Always call `try Task.checkCancellation()` in loops and between async operations |
| 14 | Testing views directly with unit tests | Test ViewModels for logic; use XCUITest for UI flows; use snapshot tests for visual regression |
| 15 | Using Core Data in new projects (iOS 17+) | Use SwiftData — native Swift, macro-based, automatic CloudKit sync |
| 16 | Blocking the main thread with synchronous I/O | Use `async/await` for all file, network, and database operations |
| 17 | Using `DispatchQueue.main.async` in Swift 6 | Use `@MainActor` or `MainActor.run {}` — GCD is legacy in strict concurrency |
| 18 | Missing accessibility labels on interactive elements | Every button, link, and control needs `.accessibilityLabel()` |
| 19 | Ignoring Dynamic Type | Test all views at accessibility sizes; use `@Environment(\.dynamicTypeSize)` for layout adaptation |
| 20 | Using `onAppear` for async work | Use `.task` modifier — it handles cancellation automatically when the view disappears |
| 21 | Passing `ModelContext` through init chains | Use `@Environment(\.modelContext)` — SwiftUI propagates it automatically |
| 22 | Creating `ModelContainer` per-view | Create once at app root via `.modelContainer()`, share via environment |
| 23 | Using `@EnvironmentObject` for iOS 17+ targets | Use `@Environment` with `@Observable` objects — `@EnvironmentObject` is for `ObservableObject` |
| 24 | Disabling ATS for convenience | Keep ATS enabled; configure `localhost` exception only for debug builds |
| 25 | Using `print()` for logging | Use `os.Logger` with categories and levels — it is structured, filterable, and privacy-aware |
| 26 | Not defining a SwiftData `MigrationPlan` | Always define versioned schemas and migration plan before shipping — silent data loss otherwise |
| 27 | CocoaPods or Carthage for dependency management | Use Swift Package Manager — it is built into Xcode and the standard for Swift |
| 28 | Catching all errors with empty `catch {}` | Handle errors explicitly or propagate with `throws`. Empty catch blocks hide bugs |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a SwiftUI patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:swiftui&title=[SwiftUI]%20)**

Use the `patterns:swiftui` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
