# Development Patterns — Spring Stack

Spring Boot / Kotlin & Java / Thymeleaf / HTMX / PostgreSQL

This document captures stack-specific patterns, conventions, and decisions for Spring stack projects (Spring Boot with Kotlin/Java, Thymeleaf, HTMX, Spring Data JPA, Spring Security, and PostgreSQL). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Spring Boot apps, test with JUnit 5 + Testcontainers, use Thymeleaf + HTMX, deploy with Docker/Kubernetes, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Kotlin | 2.2+ | Primary language — prefer Kotlin over Java for new code |
| Java | 21+ | LTS release, virtual threads, pattern matching, record patterns |
| Spring Boot | 3.4+ | Auto-configuration, actuator, devtools |
| Spring Framework | 6.2+ | Ships with Boot 3.4 |
| Spring Data JPA | 3.4+ | Repository abstraction over Hibernate |
| Hibernate | 6.6+ | JPA implementation, ships with Boot 3.4 |
| Spring Security | 6.4+ | Authentication, authorization, CSRF, headers |
| Thymeleaf | 3.1+ | Server-side HTML templates |
| thymeleaf-extras-springsecurity6 | 3.1+ | Security dialect for templates |
| HTMX | 2.0+ | Hypermedia-driven interactivity |
| Tailwind CSS | 4.x | Utility-first CSS |
| PostgreSQL | 16+ | Primary database |
| Flyway | 10+ | Schema migrations |
| Gradle | 8.12+ | Build tool, Kotlin DSL |
| Testcontainers | 1.20+ | Docker-based integration testing |
| JUnit 5 | 5.11+ | Test framework |
| MockMvc | (Spring) | Web layer testing |
| Spring WebFlux | 6.2+ | Reactive web (optional — use only where needed) |
| Docker | 27+ | Container builds |
| Kubernetes | 1.30+ | Production orchestration |

### Version Constraint Policy

Use Gradle version catalogs (`gradle/libs.versions.toml`) for centralized dependency management:

```toml
# gradle/libs.versions.toml — Good: centralized, explicit
[versions]
spring-boot = "3.4.3"
kotlin = "2.2.0"
htmx = "2.0.4"
testcontainers = "1.20.4"
flyway = "10.22.0"

[libraries]
spring-boot-starter-web = { module = "org.springframework.boot:spring-boot-starter-web" }
spring-boot-starter-data-jpa = { module = "org.springframework.boot:spring-boot-starter-data-jpa" }
spring-boot-starter-security = { module = "org.springframework.boot:spring-boot-starter-security" }
spring-boot-starter-thymeleaf = { module = "org.springframework.boot:spring-boot-starter-thymeleaf" }
testcontainers-postgresql = { module = "org.testcontainers:postgresql", version.ref = "testcontainers" }
flyway-core = { module = "org.flywaydb:flyway-core", version.ref = "flyway" }
flyway-postgresql = { module = "org.flywaydb:flyway-database-postgresql", version.ref = "flyway" }

[plugins]
spring-boot = { id = "org.springframework.boot", version.ref = "spring-boot" }
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
kotlin-spring = { id = "org.jetbrains.kotlin.plugin.spring", version.ref = "kotlin" }
kotlin-jpa = { id = "org.jetbrains.kotlin.plugin.jpa", version.ref = "kotlin" }
```

```kotlin
// build.gradle.kts — Good: references version catalog
plugins {
    alias(libs.plugins.spring.boot)
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.spring)
    alias(libs.plugins.kotlin.jpa)
}

dependencies {
    implementation(libs.spring.boot.starter.web)
    implementation(libs.spring.boot.starter.data.jpa)
    implementation(libs.spring.boot.starter.security)
    implementation(libs.spring.boot.starter.thymeleaf)
    runtimeOnly(libs.flyway.core)
    runtimeOnly(libs.flyway.postgresql)
    testImplementation(libs.testcontainers.postgresql)
}
```

**Bad — hardcoded versions scattered across build files:**
```kotlin
// Bad — versions duplicated, hard to update
implementation("org.springframework.boot:spring-boot-starter-web:3.4.3")
implementation("org.testcontainers:postgresql:1.20.4")
```

### Spring Boot 3.4+ Features to Use

| Feature | Version | Use For |
|---|---|---|
| Virtual threads | 3.2+ | `spring.threads.virtual.enabled=true` — lightweight concurrency for blocking I/O |
| Structured logging | 3.4+ | JSON log output via `logging.structured.format.console=ecs` |
| `@HttpExchange` proxies | 3.2+ | Declarative HTTP clients replacing RestTemplate |
| `RestClient` | 3.2+ | Fluent synchronous HTTP client replacing RestTemplate |
| Observability (Micrometer) | 3.0+ | Metrics, tracing, and logging correlation |
| Docker Compose support | 3.1+ | Auto-start `compose.yaml` in dev via `spring-boot-docker-compose` |
| SSL bundles | 3.1+ | Centralized TLS/SSL configuration |
| GraalVM native image | 3.0+ | AOT compilation for fast startup (optional) |
| Testcontainers at dev time | 3.1+ | `@ServiceConnection` for auto-configured containers |

### Kotlin Compiler Configuration

```kotlin
// build.gradle.kts
kotlin {
    compilerOptions {
        freeCompilerArgs.addAll("-Xjsr305=strict") // Strict null-safety for Spring annotations
        jvmTarget.set(JvmTarget.JVM_21)
    }
}
```

The `-Xjsr305=strict` flag makes Spring's `@Nullable` and `@NonNull` annotations visible to the Kotlin compiler. Without it, all Spring types appear as platform types (no null-safety guarantees).

---

## 2. Project Structure

### Standard Spring Boot Layout

```
src/
├── main/
│   ├── kotlin/com/example/app/
│   │   ├── Application.kt                  # @SpringBootApplication entry point
│   │   ├── config/                          # Configuration classes
│   │   │   ├── SecurityConfig.kt            # Spring Security configuration
│   │   │   ├── WebConfig.kt                 # MVC configuration, interceptors
│   │   │   ├── JpaConfig.kt                 # JPA auditing, custom converters
│   │   │   └── AsyncConfig.kt               # Virtual thread executor config
│   │   ├── domain/                          # Domain layer (entities, value objects)
│   │   │   ├── user/
│   │   │   │   ├── User.kt                  # JPA entity
│   │   │   │   ├── UserRepository.kt        # Spring Data repository
│   │   │   │   ├── UserService.kt           # Business logic
│   │   │   │   └── UserRole.kt              # Enum
│   │   │   ├── chapter/
│   │   │   │   ├── Chapter.kt
│   │   │   │   ├── ChapterRepository.kt
│   │   │   │   ├── ChapterService.kt
│   │   │   │   └── Seat.kt
│   │   │   └── visitor/
│   │   │       ├── Visitor.kt
│   │   │       ├── VisitorRepository.kt
│   │   │       ├── VisitorService.kt
│   │   │       └── VisitStatus.kt
│   │   ├── web/                             # Web layer (controllers, DTOs)
│   │   │   ├── controller/
│   │   │   │   ├── HomeController.kt
│   │   │   │   ├── ChapterController.kt
│   │   │   │   ├── VisitorController.kt
│   │   │   │   └── AdminController.kt
│   │   │   ├── dto/                         # Request/response DTOs
│   │   │   │   ├── VisitorRegistrationDto.kt
│   │   │   │   └── ChapterDetailDto.kt
│   │   │   ├── advice/                      # Global exception handlers
│   │   │   │   └── GlobalExceptionHandler.kt
│   │   │   └── htmx/                        # HTMX fragment controllers
│   │   │       ├── SeatRosterFragment.kt
│   │   │       └── VisitorPipelineFragment.kt
│   │   └── infrastructure/                  # External integrations
│   │       ├── email/
│   │       │   └── EmailService.kt
│   │       ├── scheduling/
│   │       │   └── ScheduledTasks.kt
│   │       └── external/
│   │           └── ExternalApiClient.kt
│   ├── resources/
│   │   ├── application.yaml                 # Main config
│   │   ├── application-dev.yaml             # Dev profile
│   │   ├── application-test.yaml            # Test profile
│   │   ├── application-prod.yaml            # Production profile
│   │   ├── db/migration/                    # Flyway migrations
│   │   │   ├── V1__create_users.sql
│   │   │   ├── V2__create_chapters.sql
│   │   │   └── V3__create_visitors.sql
│   │   ├── templates/                       # Thymeleaf templates
│   │   │   ├── layout/
│   │   │   │   ├── base.html               # Master layout
│   │   │   │   └── fragments.html          # Reusable fragments
│   │   │   ├── home/
│   │   │   │   └── index.html
│   │   │   ├── chapter/
│   │   │   │   ├── list.html
│   │   │   │   └── detail.html
│   │   │   ├── visitor/
│   │   │   │   └── register.html
│   │   │   └── error/
│   │   │       ├── 404.html
│   │   │       └── 500.html
│   │   └── static/                          # Static assets
│   │       ├── css/
│   │       ├── js/
│   │       └── images/
│   └── docker/
│       └── Dockerfile
├── test/
│   └── kotlin/com/example/app/
│       ├── domain/                          # Unit tests
│       │   ├── user/
│       │   │   ├── UserServiceTest.kt
│       │   │   └── UserRepositoryTest.kt
│       │   └── chapter/
│       │       └── ChapterServiceTest.kt
│       ├── web/                             # Web layer tests
│       │   ├── controller/
│       │   │   ├── HomeControllerTest.kt
│       │   │   ├── ChapterControllerTest.kt
│       │   │   └── VisitorControllerTest.kt
│       │   └── htmx/
│       │       └── SeatRosterFragmentTest.kt
│       ├── integration/                     # Full integration tests
│       │   ├── AbstractIntegrationTest.kt   # Base class with Testcontainers
│       │   ├── VisitorRegistrationIT.kt
│       │   └── SecurityIT.kt
│       └── support/
│           ├── TestFactory.kt               # Test data builders
│           └── TestContainerConfig.kt       # Shared container config
└── build.gradle.kts
```

**Conventions:**
- Package-by-feature inside `domain/` — each bounded context gets its own package with entity, repository, and service.
- `web/` layer holds controllers and DTOs only — no business logic.
- `infrastructure/` holds external integrations (email, external APIs, file storage).
- `config/` holds `@Configuration` classes only.
- Test structure mirrors source structure. Integration tests in a separate `integration/` package.
- Thymeleaf templates mirror the controller structure under `resources/templates/`.

### Kotlin-Specific Conventions

- Use `data class` for DTOs and value objects.
- Use `class` (not `data class`) for JPA entities — `data class` generates `equals`/`hashCode` from all fields, which breaks Hibernate's identity semantics.
- Use `companion object` for factory methods and constants, never for stateful singletons.
- Use extension functions for cross-cutting concerns (logging, mapping).
- Use sealed classes/interfaces for domain result types.

```kotlin
// Good — sealed interface for operation results
sealed interface RegistrationResult {
    data class Success(val visitor: Visitor) : RegistrationResult
    data class ValidationError(val errors: Map<String, String>) : RegistrationResult
    data class DuplicateEmail(val email: String) : RegistrationResult
}

// Good — extension function for logging
inline val <reified T> T.logger: Logger
    get() = LoggerFactory.getLogger(T::class.java)
```

---

## 3. JPA Entity Patterns

### Entity Template (Kotlin)

Every JPA entity follows this structure:

```kotlin
@Entity
@Table(name = "chapters")
class Chapter(
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    val id: UUID? = null,

    @Column(nullable = false, length = 200)
    var name: String,

    @Column(nullable = false, unique = true, length = 100)
    var slug: String,

    @Enumerated(EnumType.STRING)
    @Column(nullable = false, length = 20)
    var meetingDay: DayOfWeek,

    @Column(nullable = false)
    var meetingTime: LocalTime,

    @Column(nullable = false, length = 500)
    var location: String,

    @OneToMany(mappedBy = "chapter", cascade = [CascadeType.ALL], orphanRemoval = true)
    val seats: MutableList<Seat> = mutableListOf(),

    @Column(nullable = false, updatable = false)
    val createdAt: Instant = Instant.now(),

    @Column(nullable = false)
    var updatedAt: Instant = Instant.now()
) {
    @PreUpdate
    fun onUpdate() {
        updatedAt = Instant.now()
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is Chapter) return false
        return id != null && id == other.id
    }

    override fun hashCode(): Int = javaClass.hashCode()

    override fun toString(): String = "Chapter(id=$id, name='$name', slug='$slug')"
}
```

**Conventions:**
- Always use `GenerationType.UUID` for primary keys — never `IDENTITY` or `SEQUENCE` unless there is a specific performance requirement.
- Always override `equals`/`hashCode` based on the entity identifier — never use all fields (Hibernate proxy issues).
- Always use `Instant` for timestamps, never `LocalDateTime` — timestamps must be timezone-aware.
- Use `var` for mutable fields, `val` for immutable identifiers and collections.
- Use `@PreUpdate` for automatic `updatedAt` — or JPA auditing with `@EnableJpaAuditing` and `@LastModifiedDate`.
- Declare `nullable = false` on all non-optional columns at the JPA level, and enforce the same at the database level with Flyway.
- Never use `data class` for entities. The generated `equals`/`hashCode` breaks Hibernate identity semantics and lazy loading.

### Entity Template (Java)

```java
@Entity
@Table(name = "chapters")
public class Chapter {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @Column(nullable = false, length = 200)
    private String name;

    @Column(nullable = false, unique = true, length = 100)
    private String slug;

    @Enumerated(EnumType.STRING)
    @Column(nullable = false, length = 20)
    private DayOfWeek meetingDay;

    @Column(nullable = false)
    private LocalTime meetingTime;

    @Column(nullable = false, length = 500)
    private String location;

    @OneToMany(mappedBy = "chapter", cascade = CascadeType.ALL, orphanRemoval = true)
    private List<Seat> seats = new ArrayList<>();

    @CreatedDate
    @Column(nullable = false, updatable = false)
    private Instant createdAt;

    @LastModifiedDate
    @Column(nullable = false)
    private Instant updatedAt;

    protected Chapter() {} // JPA required no-arg constructor

    public Chapter(String name, String slug, DayOfWeek meetingDay,
                   LocalTime meetingTime, String location) {
        this.name = name;
        this.slug = slug;
        this.meetingDay = meetingDay;
        this.meetingTime = meetingTime;
        this.location = location;
    }

    // Getters, equals/hashCode by id, toString
}
```

**Note:** In Java, the no-arg constructor must be `protected` (not `private`) for Hibernate proxy generation. Kotlin handles this via the `kotlin-jpa` plugin, which generates synthetic no-arg constructors automatically.

### Spring Data Repository

```kotlin
interface ChapterRepository : JpaRepository<Chapter, UUID> {

    fun findBySlug(slug: String): Chapter?

    @Query("SELECT c FROM Chapter c LEFT JOIN FETCH c.seats WHERE c.slug = :slug")
    fun findBySlugWithSeats(@Param("slug") slug: String): Chapter?

    fun findByMeetingDay(day: DayOfWeek): List<Chapter>

    @Query("SELECT c FROM Chapter c WHERE LOWER(c.name) LIKE LOWER(CONCAT('%', :query, '%'))")
    fun search(@Param("query") query: String): List<Chapter>
}
```

**Conventions:**
- Use derived query methods for simple queries.
- Use `@Query` with JPQL for anything with joins, aggregations, or complex conditions.
- Use `JOIN FETCH` to prevent N+1 queries when loading associations.
- Return `Optional<T>` in Java repositories, nullable types in Kotlin.
- Never use native SQL in `@Query` unless JPQL cannot express the query.

### Flyway Migrations

```sql
-- V1__create_chapters.sql
CREATE TABLE chapters (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(200) NOT NULL,
    slug        VARCHAR(100) NOT NULL UNIQUE,
    meeting_day VARCHAR(20)  NOT NULL,
    meeting_time TIME        NOT NULL,
    location    VARCHAR(500) NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chapters_slug ON chapters(slug);
CREATE INDEX idx_chapters_meeting_day ON chapters(meeting_day);
```

**Conventions:**
- Use `TIMESTAMPTZ` (not `TIMESTAMP`) for all time columns — timezone-aware.
- Use `gen_random_uuid()` as the database default, matching JPA's `GenerationType.UUID`.
- Add indexes for all foreign keys, columns used in `WHERE` clauses, unique constraints, and sort columns.
- Never edit a migration after it has been committed. Write a new corrective migration.
- Use descriptive migration names: `V4__add_visitor_status_column.sql`, not `V4__update.sql`.
- Flyway configuration in `application.yaml`:

```yaml
spring:
  flyway:
    enabled: true
    locations: classpath:db/migration
    baseline-on-migrate: false
    validate-on-migrate: true
```

---

## 4. Authentication & Authorization

### Spring Security 6 Configuration

Spring Security 6 uses the component-based `SecurityFilterChain` approach. No more extending `WebSecurityConfigurerAdapter`.

```kotlin
@Configuration
@EnableWebSecurity
@EnableMethodSecurity
class SecurityConfig(
    private val userDetailsService: AppUserDetailsService
) {

    @Bean
    fun securityFilterChain(http: HttpSecurity): SecurityFilterChain {
        http {
            authorizeHttpRequests {
                authorize("/", permitAll)
                authorize("/register", permitAll)
                authorize("/login", permitAll)
                authorize("/css/**", permitAll)
                authorize("/js/**", permitAll)
                authorize("/images/**", permitAll)
                authorize("/actuator/health", permitAll)
                authorize("/admin/**", hasRole("ADMIN"))
                authorize("/api/**", authenticated)
                authorize(anyRequest, authenticated)
            }
            formLogin {
                loginPage = "/login"
                defaultSuccessUrl("/dashboard", false)
                failureUrl = "/login?error"
            }
            logout {
                logoutSuccessUrl = "/"
                invalidateHttpSession = true
                deleteCookies("JSESSIONID")
            }
            csrf {
                // CSRF enabled by default — required for Thymeleaf forms
                // For HTMX: include CSRF token via meta tag
            }
            headers {
                contentSecurityPolicy {
                    policyDirectives = "default-src 'self'; script-src 'self' 'unsafe-inline'; " +
                        "style-src 'self' 'unsafe-inline'; img-src 'self' data:; " +
                        "font-src 'self'; connect-src 'self'; frame-ancestors 'self'"
                }
                httpStrictTransportSecurity {
                    includeSubDomains = true
                    maxAgeInSeconds = 31536000
                }
                referrerPolicy {
                    policy = ReferrerPolicyHeaderWriter.ReferrerPolicy.STRICT_ORIGIN_WHEN_CROSS_ORIGIN
                }
            }
        }
        return http.build()
    }

    @Bean
    fun passwordEncoder(): PasswordEncoder = BCryptPasswordEncoder()
}
```

### UserDetailsService

```kotlin
@Service
class AppUserDetailsService(
    private val userRepository: UserRepository
) : UserDetailsService {

    override fun loadUserByUsername(email: String): UserDetails {
        val user = userRepository.findByEmail(email)
            ?: throw UsernameNotFoundException("User not found: $email")

        return org.springframework.security.core.userdetails.User.builder()
            .username(user.email)
            .password(user.passwordHash)
            .roles(user.role.name)
            .build()
    }
}
```

### Role Model

Four roles, enforced through Spring Security:

| Role | Authority | Access |
|---|---|---|
| `ANONYMOUS` | (unauthenticated) | Public pages, registration form |
| `MEMBER` | `ROLE_MEMBER` | Own profile, roster, claim seats |
| `CHAPTER_ADMIN` | `ROLE_CHAPTER_ADMIN` | Chapter management, visitor pipeline, user admin |
| `PLATFORM_ADMIN` | `ROLE_PLATFORM_ADMIN` | Everything, all chapters |

### Method-Level Security

```kotlin
@Service
class ChapterService(
    private val chapterRepository: ChapterRepository
) {
    @PreAuthorize("hasAnyRole('CHAPTER_ADMIN', 'PLATFORM_ADMIN')")
    fun updateChapter(id: UUID, dto: ChapterUpdateDto): Chapter {
        val chapter = chapterRepository.findById(id)
            .orElseThrow { EntityNotFoundException("Chapter not found: $id") }
        chapter.name = dto.name
        chapter.location = dto.location
        return chapterRepository.save(chapter)
    }

    @PreAuthorize("#userId == authentication.principal.id or hasRole('PLATFORM_ADMIN')")
    fun getUserProfile(userId: UUID): UserProfileDto {
        // Only the user themselves or platform admins can view
    }
}
```

### CSRF with HTMX

HTMX requires the CSRF token in request headers. Include it via a `<meta>` tag and configure HTMX to send it:

```html
<!-- In base.html layout -->
<meta name="_csrf" th:content="${_csrf.token}" />
<meta name="_csrf_header" th:content="${_csrf.headerName}" />

<script>
  document.body.addEventListener('htmx:configRequest', function(event) {
    const csrfToken = document.querySelector('meta[name="_csrf"]').content;
    const csrfHeader = document.querySelector('meta[name="_csrf_header"]').content;
    event.detail.headers[csrfHeader] = csrfToken;
  });
</script>
```

---

## 5. Component Library

### Thymeleaf Layout Dialect

Use `thymeleaf-layout-dialect` for template inheritance:

```html
<!-- templates/layout/base.html -->
<!DOCTYPE html>
<html xmlns:th="http://www.thymeleaf.org"
      xmlns:layout="http://www.ultraq.net.nz/thymeleaf/layout"
      xmlns:sec="http://www.thymeleaf.org/extras/spring-security">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title layout:title-pattern="$CONTENT_TITLE - $LAYOUT_TITLE">App Name</title>
    <meta name="_csrf" th:content="${_csrf.token}" />
    <meta name="_csrf_header" th:content="${_csrf.headerName}" />
    <link rel="stylesheet" th:href="@{/css/app.css}" />
    <script src="https://unpkg.com/htmx.org@2.0.4"></script>
</head>
<body>
    <nav th:replace="~{layout/fragments :: navbar}"></nav>

    <main class="container mx-auto px-4 py-8">
        <div layout:fragment="content"></div>
    </main>

    <footer th:replace="~{layout/fragments :: footer}"></footer>
    <script th:src="@{/js/app.js}"></script>
</body>
</html>
```

```html
<!-- templates/chapter/detail.html -->
<!DOCTYPE html>
<html xmlns:th="http://www.thymeleaf.org"
      xmlns:layout="http://www.ultraq.net.nz/thymeleaf/layout"
      layout:decorate="~{layout/base}">
<head>
    <title th:text="${chapter.name}">Chapter</title>
</head>
<body>
    <div layout:fragment="content">
        <h1 th:text="${chapter.name}">Chapter Name</h1>
        <!-- Page-specific content -->
    </div>
</body>
</html>
```

### Thymeleaf Fragment Components

Reusable UI fragments act as the component library:

```html
<!-- templates/layout/fragments.html -->

<!-- Alert component -->
<div th:fragment="alert(type, message)"
     th:class="'alert alert-' + ${type} + ' mb-4 rounded-lg p-4'"
     role="alert">
    <span th:text="${message}">Alert message</span>
</div>

<!-- Card component -->
<div th:fragment="card(title)" class="card bg-base-100 shadow-md">
    <div class="card-body">
        <h2 class="card-title" th:text="${title}">Title</h2>
        <div th:insert="~{:: content}"></div>
    </div>
</div>

<!-- Pagination component -->
<nav th:fragment="pagination(page)" aria-label="Page navigation">
    <div class="btn-group">
        <a th:each="i : ${#numbers.sequence(0, page.totalPages - 1)}"
           th:href="@{''(page=${i})}"
           th:text="${i + 1}"
           th:classappend="${i == page.number} ? 'btn-active'"
           class="btn btn-sm">1</a>
    </div>
</nav>

<!-- Form field with error display -->
<div th:fragment="field(fieldName, label, type)"
     class="form-control w-full mb-4">
    <label class="label" th:for="${fieldName}">
        <span class="label-text" th:text="${label}">Label</span>
    </label>
    <input th:type="${type ?: 'text'}"
           th:id="${fieldName}"
           th:name="${fieldName}"
           th:field="*{__${fieldName}__}"
           th:errorclass="input-error"
           class="input input-bordered w-full"
           th:attr="autocomplete=${fieldName}" />
    <label class="label" th:if="${#fields.hasErrors(fieldName)}">
        <span class="label-text-alt text-error"
              th:errors="*{__${fieldName}__}">Error</span>
    </label>
</div>
```

### HTMX Fragment Controllers

Controllers that serve HTMX partial responses return fragments, not full pages:

```kotlin
@Controller
@RequestMapping("/fragments")
class SeatRosterFragment(
    private val chapterService: ChapterService
) {
    @GetMapping("/seats/{chapterId}")
    fun seatRoster(
        @PathVariable chapterId: UUID,
        model: Model
    ): String {
        model.addAttribute("seats", chapterService.getSeatsForChapter(chapterId))
        return "chapter/fragments :: seatRoster"
    }

    @PostMapping("/seats/{seatId}/claim")
    fun claimSeat(
        @PathVariable seatId: UUID,
        @AuthenticationPrincipal user: UserDetails,
        model: Model
    ): String {
        val seat = chapterService.claimSeat(seatId, user.username)
        model.addAttribute("seat", seat)
        return "chapter/fragments :: seatRow"
    }
}
```

### Tailwind CSS Integration

For Spring Boot projects, use either the Tailwind standalone CLI or the Node.js-based build:

**Option 1: Standalone CLI (preferred — no Node.js required)**

```kotlin
// build.gradle.kts — Tailwind standalone CLI task
tasks.register<Exec>("tailwindBuild") {
    commandLine("npx", "tailwindcss", "-i", "src/main/resources/static/css/input.css",
                "-o", "src/main/resources/static/css/app.css", "--minify")
}

tasks.named("processResources") {
    dependsOn("tailwindBuild")
}
```

**Option 2: Gradle Node Plugin**

```kotlin
// build.gradle.kts
plugins {
    id("com.github.node-gradle.node") version "7.1.0"
}

node {
    version.set("22.12.0")
    download.set(true)
}
```

---

## 6. Testing Patterns

### Test Pyramid (Spring-specific)

```
        /\
       /  \          E2E (Selenium/Playwright) — deferred to slice 2+
      /    \
     /------\
    /        \        Controller Tests (MockMvc + Thymeleaf assertions)
   /          \       HTTP requests, response codes, view rendering, HTMX
  /------------\
 /              \      Integration Tests (Testcontainers + @SpringBootTest)
/                \     Full context, real DB, service interactions, transactions
/------------------\
/                    \   Unit Tests (JUnit 5 + Mockito)
/                      \  Services, entities, mappers, validators — no Spring context
/------------------------\
```

### Unit Tests (JUnit 5 + MockK/Mockito)

```kotlin
@ExtendWith(MockKExtension::class)
class ChapterServiceTest {

    @MockK
    private lateinit var chapterRepository: ChapterRepository

    @InjectMockKs
    private lateinit var chapterService: ChapterService

    @Test
    fun `findBySlug returns chapter when exists`() {
        val chapter = Chapter(
            name = "Westlake Select",
            slug = "westlake-select",
            meetingDay = DayOfWeek.WEDNESDAY,
            meetingTime = LocalTime.of(7, 0),
            location = "Denver, NC"
        )
        every { chapterRepository.findBySlug("westlake-select") } returns chapter

        val result = chapterService.findBySlug("westlake-select")

        assertThat(result).isNotNull
        assertThat(result!!.name).isEqualTo("Westlake Select")
        verify(exactly = 1) { chapterRepository.findBySlug("westlake-select") }
    }

    @Test
    fun `findBySlug returns null when not found`() {
        every { chapterRepository.findBySlug("nonexistent") } returns null

        val result = chapterService.findBySlug("nonexistent")

        assertThat(result).isNull()
    }
}
```

**Convention:** Use Kotlin backtick method names for descriptive test names. Use MockK in Kotlin projects (idiomatic), Mockito in Java projects.

### Controller Tests (MockMvc)

```kotlin
@WebMvcTest(ChapterController::class)
@Import(SecurityConfig::class)
class ChapterControllerTest {

    @Autowired
    private lateinit var mockMvc: MockMvc

    @MockkBean
    private lateinit var chapterService: ChapterService

    @Test
    @WithMockUser(roles = ["MEMBER"])
    fun `GET chapters shows chapter list`() {
        val chapters = listOf(
            Chapter(name = "Westlake Select", slug = "westlake-select",
                    meetingDay = DayOfWeek.WEDNESDAY, meetingTime = LocalTime.of(7, 0),
                    location = "Denver, NC")
        )
        every { chapterService.findAll() } returns chapters

        mockMvc.get("/chapters") {
            accept(MediaType.TEXT_HTML)
        }.andExpect {
            status { isOk() }
            view { name("chapter/list") }
            model { attributeExists("chapters") }
            content { string(containsString("Westlake Select")) }
        }
    }

    @Test
    fun `GET chapters redirects to login when unauthenticated`() {
        mockMvc.get("/chapters")
            .andExpect {
                status { is3xxRedirection() }
                redirectedUrlPattern("**/login")
            }
    }

    @Test
    @WithMockUser(roles = ["MEMBER"])
    fun `POST visitor registration validates input`() {
        mockMvc.post("/visitors/register") {
            contentType = MediaType.APPLICATION_FORM_URLENCODED
            param("name", "")
            param("email", "invalid-email")
            with(csrf())
        }.andExpect {
            status { isOk() } // Re-renders form with errors
            model { attributeHasFieldErrors("visitorDto", "name", "email") }
        }
    }
}
```

### HTMX Controller Tests

```kotlin
@Test
@WithMockUser(roles = ["CHAPTER_ADMIN"])
fun `HTMX seat claim returns fragment`() {
    val seat = Seat(id = UUID.randomUUID(), classification = "Plumber", status = SeatStatus.OPEN)
    every { chapterService.claimSeat(any(), any()) } returns seat

    mockMvc.post("/fragments/seats/${seat.id}/claim") {
        header("HX-Request", "true")
        with(csrf())
    }.andExpect {
        status { isOk() }
        // Verify partial HTML fragment, not full page
        content { string(containsString("Plumber")) }
        content { string(not(containsString("<html"))) }
    }
}
```

### Integration Tests (Testcontainers)

```kotlin
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@Testcontainers
abstract class AbstractIntegrationTest {

    companion object {
        @Container
        @JvmStatic
        @ServiceConnection
        val postgres = PostgreSQLContainer("postgres:16-alpine")
            .withDatabaseName("testdb")
            .withUsername("test")
            .withPassword("test")
    }
}
```

```kotlin
class VisitorRegistrationIT : AbstractIntegrationTest() {

    @Autowired
    private lateinit var visitorService: VisitorService

    @Autowired
    private lateinit var visitorRepository: VisitorRepository

    @Test
    fun `registering a visitor persists to database`() {
        val dto = VisitorRegistrationDto(
            name = "Jane Smith",
            email = "jane@example.com",
            phone = "+1-555-0100",
            chapterSlug = "westlake-select"
        )

        val result = visitorService.register(dto)

        assertThat(result).isInstanceOf(RegistrationResult.Success::class.java)
        val visitor = visitorRepository.findByEmail("jane@example.com")
        assertThat(visitor).isNotNull
        assertThat(visitor!!.name).isEqualTo("Jane Smith")
    }

    @Test
    fun `duplicate email registration returns error`() {
        val dto = VisitorRegistrationDto(
            name = "Jane Smith",
            email = "duplicate@example.com",
            phone = "+1-555-0100",
            chapterSlug = "westlake-select"
        )
        visitorService.register(dto) // First registration

        val result = visitorService.register(dto)

        assertThat(result).isInstanceOf(RegistrationResult.DuplicateEmail::class.java)
    }
}
```

### Testcontainers at Development Time

Spring Boot 3.1+ supports `@ServiceConnection` for automatic container configuration. Use a separate `TestApplication.kt` for dev-time containers:

```kotlin
// src/test/kotlin/com/example/app/TestApplication.kt
@TestConfiguration(proxyBeanMethods = false)
class TestContainerConfig {

    @Bean
    @ServiceConnection
    fun postgresContainer(): PostgreSQLContainer<*> {
        return PostgreSQLContainer("postgres:16-alpine")
    }
}

fun main(args: Array<String>) {
    fromApplication<Application>()
        .with(TestContainerConfig::class.java)
        .run(*args)
}
```

Run with `./gradlew bootTestRun` to start the app with Testcontainers-backed database — no local PostgreSQL required.

### Test Data Builders

```kotlin
object TestFactory {

    fun chapter(
        name: String = "Test Chapter",
        slug: String = "test-chapter-${UUID.randomUUID().toString().take(8)}",
        meetingDay: DayOfWeek = DayOfWeek.WEDNESDAY,
        meetingTime: LocalTime = LocalTime.of(7, 0),
        location: String = "Test Location"
    ): Chapter = Chapter(
        name = name,
        slug = slug,
        meetingDay = meetingDay,
        meetingTime = meetingTime,
        location = location
    )

    fun user(
        email: String = "user-${UUID.randomUUID().toString().take(8)}@example.com",
        role: UserRole = UserRole.MEMBER,
        passwordHash: String = BCryptPasswordEncoder().encode("password123")
    ): User = User(
        email = email,
        passwordHash = passwordHash,
        role = role
    )

    fun visitor(
        name: String = "Test Visitor",
        email: String = "visitor-${UUID.randomUUID().toString().take(8)}@example.com",
        phone: String = "+1-555-0100",
        status: VisitStatus = VisitStatus.REGISTERED
    ): Visitor = Visitor(
        name = name,
        email = email,
        phone = phone,
        status = status
    )
}
```

### Test Configuration

```yaml
# application-test.yaml
spring:
  jpa:
    hibernate:
      ddl-auto: validate  # Flyway manages schema, Hibernate only validates
    show-sql: true
    properties:
      hibernate:
        format_sql: true
  flyway:
    enabled: true
  mail:
    host: localhost
    port: 3025  # GreenMail test SMTP

logging:
  level:
    org.springframework.security: DEBUG
    org.hibernate.SQL: DEBUG
    org.hibernate.type.descriptor.sql.BasicBinder: TRACE
```

---

## 7. Thymeleaf + HTMX Patterns

### HTMX Integration Philosophy

Use Thymeleaf for server-rendered HTML. Use HTMX for progressive enhancement — partial page updates without full page reloads. No JavaScript frameworks, no build steps for JS, no client-side routing.

### Core HTMX Attributes

```html
<!-- Inline editing with HTMX -->
<div id="chapter-name">
    <h1 th:text="${chapter.name}"
        hx-get="/fragments/chapters/${chapter.id}/edit-name"
        hx-trigger="click"
        hx-swap="outerHTML">
        Chapter Name
    </h1>
</div>

<!-- Search with debounce -->
<input type="search"
       name="query"
       placeholder="Search members..."
       hx-get="/fragments/members/search"
       hx-trigger="input changed delay:300ms"
       hx-target="#member-results"
       hx-indicator="#search-spinner"
       autocomplete="off" />
<span id="search-spinner" class="htmx-indicator loading loading-spinner"></span>
<div id="member-results"></div>

<!-- Infinite scroll -->
<div hx-get="/fragments/visitors?page=2"
     hx-trigger="revealed"
     hx-swap="afterend">
    Loading more...
</div>

<!-- Delete with confirmation -->
<button hx-delete="/api/seats/${seat.id}"
        hx-confirm="Remove this seat?"
        hx-target="closest tr"
        hx-swap="outerHTML swap:500ms"
        class="btn btn-error btn-sm">
    Remove
</button>
```

### HTMX Response Headers

Use response headers to control HTMX behavior from the server:

```kotlin
@Controller
class VisitorController(
    private val visitorService: VisitorService
) {
    @PostMapping("/visitors/register")
    fun register(
        @Valid @ModelAttribute("visitorDto") dto: VisitorRegistrationDto,
        bindingResult: BindingResult,
        model: Model,
        response: HttpServletResponse,
        request: HttpServletRequest
    ): String {
        if (bindingResult.hasErrors()) {
            return if (isHtmxRequest(request)) {
                "visitor/fragments :: registrationForm"
            } else {
                "visitor/register"
            }
        }

        visitorService.register(dto)

        return if (isHtmxRequest(request)) {
            response.setHeader("HX-Redirect", "/registered")
            "visitor/fragments :: registrationSuccess"
        } else {
            "redirect:/registered"
        }
    }

    private fun isHtmxRequest(request: HttpServletRequest): Boolean {
        return request.getHeader("HX-Request") == "true"
    }
}
```

### Thymeleaf HTMX Helper

Create a utility for common HTMX patterns:

```kotlin
@Component
class HtmxHelper {

    fun isHtmxRequest(request: HttpServletRequest): Boolean =
        request.getHeader("HX-Request") == "true"

    fun isBoosted(request: HttpServletRequest): Boolean =
        request.getHeader("HX-Boosted") == "true"

    fun setRedirect(response: HttpServletResponse, url: String) {
        response.setHeader("HX-Redirect", url)
    }

    fun setTrigger(response: HttpServletResponse, event: String) {
        response.setHeader("HX-Trigger", event)
    }

    fun setRetarget(response: HttpServletResponse, selector: String) {
        response.setHeader("HX-Retarget", selector)
    }

    fun setReswap(response: HttpServletResponse, strategy: String) {
        response.setHeader("HX-Reswap", strategy)
    }
}
```

### Out-of-Band Swaps

Update multiple page regions from a single HTMX response:

```html
<!-- Primary response — replaces the target -->
<div id="visitor-row-123">
    <td th:text="${visitor.name}">Jane Smith</td>
    <td><span class="badge badge-success">Visited</span></td>
</div>

<!-- Out-of-band update — updates the stats counter -->
<div id="visitor-count" hx-swap-oob="innerHTML">
    <span th:text="${totalVisitors}">42</span> visitors
</div>

<!-- Out-of-band update — updates a notification -->
<div id="notifications" hx-swap-oob="afterbegin">
    <div class="alert alert-success">Visitor status updated.</div>
</div>
```

### Form Handling with Thymeleaf

```html
<!-- templates/visitor/register.html -->
<form th:action="@{/visitors/register}"
      th:object="${visitorDto}"
      method="post"
      novalidate
      hx-post="/visitors/register"
      hx-target="#registration-form"
      hx-swap="outerHTML">

    <div id="registration-form">
        <fieldset>
            <legend class="text-lg font-semibold mb-4">Your Information</legend>

            <div class="form-control w-full mb-4">
                <label class="label" for="name">
                    <span class="label-text">Full Name</span>
                </label>
                <input type="text" id="name" th:field="*{name}"
                       th:errorclass="input-error"
                       class="input input-bordered w-full"
                       autocomplete="name" required />
                <label class="label" th:if="${#fields.hasErrors('name')}">
                    <span class="label-text-alt text-error" th:errors="*{name}">Error</span>
                </label>
            </div>

            <div class="form-control w-full mb-4">
                <label class="label" for="email">
                    <span class="label-text">Email</span>
                </label>
                <input type="email" id="email" th:field="*{email}"
                       th:errorclass="input-error"
                       class="input input-bordered w-full"
                       autocomplete="email" required />
                <label class="label" th:if="${#fields.hasErrors('email')}">
                    <span class="label-text-alt text-error" th:errors="*{email}">Error</span>
                </label>
            </div>

            <div class="form-control w-full mb-4">
                <label class="label" for="phone">
                    <span class="label-text">Phone</span>
                </label>
                <input type="tel" id="phone" th:field="*{phone}"
                       th:errorclass="input-error"
                       class="input input-bordered w-full"
                       autocomplete="tel" required />
                <label class="label" th:if="${#fields.hasErrors('phone')}">
                    <span class="label-text-alt text-error" th:errors="*{phone}">Error</span>
                </label>
            </div>
        </fieldset>

        <button type="submit" class="btn btn-primary w-full h-12 mt-4">
            Reserve My Free Visit
        </button>
    </div>
</form>
```

---

## 8. Background Jobs

### Spring Scheduling

For simple recurring tasks, use `@Scheduled` with virtual threads:

```kotlin
@Configuration
@EnableScheduling
class SchedulingConfig {

    @Bean
    fun taskScheduler(): TaskScheduler {
        val scheduler = ThreadPoolTaskScheduler()
        scheduler.poolSize = 4
        scheduler.setVirtualThreads(true) // Spring Boot 3.2+
        scheduler.setThreadNamePrefix("scheduled-")
        return scheduler
    }
}

@Component
class ScheduledTasks(
    private val visitorService: VisitorService,
    private val emailService: EmailService
) {
    private val logger = LoggerFactory.getLogger(javaClass)

    @Scheduled(cron = "0 0 8 * * MON") // Every Monday at 8 AM
    fun sendWeeklyVisitorReport() {
        logger.info("Sending weekly visitor report")
        visitorService.generateWeeklyReport()
    }

    @Scheduled(fixedDelay = 300_000) // Every 5 minutes
    fun processStaleVisitors() {
        visitorService.processStale()
    }
}
```

### Spring Async Tasks

For fire-and-forget operations:

```kotlin
@Configuration
@EnableAsync
class AsyncConfig {

    @Bean
    fun asyncExecutor(): Executor {
        val executor = ThreadPoolTaskExecutor()
        executor.corePoolSize = 4
        executor.maxPoolSize = 8
        executor.queueCapacity = 100
        executor.setThreadNamePrefix("async-")
        executor.setVirtualThreads(true) // Spring Boot 3.2+
        executor.initialize()
        return executor
    }
}

@Service
class EmailService(
    private val mailSender: JavaMailSender
) {
    @Async
    fun sendWelcomeEmail(visitor: Visitor) {
        // Runs on a virtual thread, does not block the request
        val message = mailSender.createMimeMessage()
        // ... compose and send
        mailSender.send(message)
    }
}
```

### For Robust Job Queues: Consider Spring Batch or External Queues

For jobs that need persistence, retries, and monitoring at scale, use one of:

| Approach | When to Use |
|---|---|
| `@Scheduled` + `@Async` | Simple periodic tasks, fire-and-forget |
| Spring Batch | Large data processing, ETL, batch imports |
| Spring Integration | Complex message-driven workflows |
| External queue (RabbitMQ, Kafka) | Distributed systems, cross-service communication |
| Database-backed queue (custom or ShedLock) | Persistent jobs with leader election in a single service |

### ShedLock for Clustered Scheduling

When running multiple instances, `@Scheduled` runs on every instance. Use ShedLock to ensure only one runs:

```kotlin
@Configuration
@EnableSchedulerLock(defaultLockAtMostFor = "PT10M")
class ShedLockConfig {

    @Bean
    fun lockProvider(dataSource: DataSource): LockProvider {
        return JdbcTemplateLockProvider(
            JdbcTemplateLockProvider.Configuration.builder()
                .withJdbcTemplate(JdbcTemplate(dataSource))
                .usingDbTime()
                .build()
        )
    }
}

@Component
class ScheduledTasks(private val visitorService: VisitorService) {

    @Scheduled(cron = "0 0 8 * * MON")
    @SchedulerLock(name = "weeklyReport", lockAtLeastFor = "PT5M")
    fun sendWeeklyVisitorReport() {
        // Only one instance runs this
        visitorService.generateWeeklyReport()
    }
}
```

---

## 9. REST API Patterns

### Controller Structure

```kotlin
@RestController
@RequestMapping("/api/v1/visitors")
class VisitorApiController(
    private val visitorService: VisitorService
) {
    @PostMapping
    fun register(
        @Valid @RequestBody dto: VisitorRegistrationDto
    ): ResponseEntity<VisitorResponseDto> {
        return when (val result = visitorService.register(dto)) {
            is RegistrationResult.Success -> ResponseEntity
                .status(HttpStatus.CREATED)
                .body(VisitorResponseDto.from(result.visitor))
            is RegistrationResult.DuplicateEmail -> ResponseEntity
                .status(HttpStatus.CONFLICT)
                .body(null)
            is RegistrationResult.ValidationError -> ResponseEntity
                .badRequest()
                .body(null)
        }
    }

    @GetMapping("/{id}")
    fun getVisitor(@PathVariable id: UUID): ResponseEntity<VisitorResponseDto> {
        val visitor = visitorService.findById(id) ?: return ResponseEntity.notFound().build()
        return ResponseEntity.ok(VisitorResponseDto.from(visitor))
    }
}
```

### DTO Validation (Jakarta Bean Validation)

```kotlin
data class VisitorRegistrationDto(
    @field:NotBlank(message = "Name is required")
    @field:Size(max = 200, message = "Name must be 200 characters or fewer")
    val name: String,

    @field:NotBlank(message = "Email is required")
    @field:Email(message = "Must be a valid email address")
    val email: String,

    @field:NotBlank(message = "Phone is required")
    @field:Pattern(regexp = "^\\+?[0-9\\-\\s()]+$", message = "Must be a valid phone number")
    val phone: String,

    @field:NotBlank(message = "Chapter is required")
    val chapterSlug: String
)
```

**Note:** In Kotlin, use `@field:` prefix for validation annotations. Without it, the annotation targets the constructor parameter, not the field, and Bean Validation does not see it.

### Global Exception Handler

```kotlin
@RestControllerAdvice
class GlobalExceptionHandler {

    private val logger = LoggerFactory.getLogger(javaClass)

    @ExceptionHandler(MethodArgumentNotValidException::class)
    fun handleValidationErrors(ex: MethodArgumentNotValidException): ResponseEntity<ErrorResponse> {
        val errors = ex.bindingResult.fieldErrors.associate { it.field to (it.defaultMessage ?: "Invalid") }
        return ResponseEntity.badRequest().body(
            ErrorResponse(
                status = 400,
                message = "Validation failed",
                errors = errors
            )
        )
    }

    @ExceptionHandler(EntityNotFoundException::class)
    fun handleNotFound(ex: EntityNotFoundException): ResponseEntity<ErrorResponse> {
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(
            ErrorResponse(status = 404, message = ex.message ?: "Not found")
        )
    }

    @ExceptionHandler(AccessDeniedException::class)
    fun handleAccessDenied(ex: AccessDeniedException): ResponseEntity<ErrorResponse> {
        return ResponseEntity.status(HttpStatus.FORBIDDEN).body(
            ErrorResponse(status = 403, message = "Access denied")
        )
    }

    @ExceptionHandler(Exception::class)
    fun handleGeneral(ex: Exception): ResponseEntity<ErrorResponse> {
        logger.error("Unhandled exception", ex)
        return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(
            ErrorResponse(status = 500, message = "Internal server error")
        )
    }
}

data class ErrorResponse(
    val status: Int,
    val message: String,
    val errors: Map<String, String> = emptyMap(),
    val timestamp: Instant = Instant.now()
)
```

### Declarative HTTP Clients (`@HttpExchange`)

Spring Boot 3.2+ supports declarative HTTP clients for external API calls:

```kotlin
@HttpExchange("/api/v1")
interface ExternalApiClient {

    @PostExchange("/contacts")
    fun createContact(@RequestBody request: ContactRequest): ContactResponse

    @GetExchange("/contacts/{id}")
    fun getContact(@PathVariable id: String): ContactResponse

    @PutExchange("/contacts/{id}")
    fun updateContact(@PathVariable id: String, @RequestBody request: ContactRequest): ContactResponse
}

@Configuration
class HttpClientConfig {

    @Bean
    fun externalApiClient(
        @Value("\${external.api.base-url}") baseUrl: String,
        @Value("\${external.api.token}") token: String
    ): ExternalApiClient {
        val restClient = RestClient.builder()
            .baseUrl(baseUrl)
            .defaultHeader(HttpHeaders.AUTHORIZATION, "Bearer $token")
            .defaultHeader(HttpHeaders.CONTENT_TYPE, MediaType.APPLICATION_JSON_VALUE)
            .build()

        val factory = HttpServiceProxyFactory
            .builderFor(RestClientAdapter.create(restClient))
            .build()

        return factory.createClient(ExternalApiClient::class.java)
    }
}
```

---

## 10. Seed Data

### Spring Boot Data Initialization

Use `CommandLineRunner` or `ApplicationRunner` for idempotent seed data:

```kotlin
@Component
@Profile("dev", "local")
class DataSeeder(
    private val chapterRepository: ChapterRepository,
    private val seatRepository: SeatRepository,
    private val userRepository: UserRepository,
    private val passwordEncoder: PasswordEncoder
) : ApplicationRunner {

    private val logger = LoggerFactory.getLogger(javaClass)

    override fun run(args: ApplicationArguments) {
        if (chapterRepository.count() > 0) {
            logger.info("Seed data already exists, skipping")
            return
        }

        logger.info("Seeding development data")
        seedChapters()
        seedAdminUser()
        logger.info("Seed data complete")
    }

    private fun seedChapters() {
        val chapter = chapterRepository.save(
            Chapter(
                name = "Westlake Select",
                slug = "westlake-select",
                meetingDay = DayOfWeek.WEDNESDAY,
                meetingTime = LocalTime.of(7, 0),
                location = "Denver, NC"
            )
        )

        val seats = listOf(
            Seat(classification = "Plumber", category = "Home Services", status = SeatStatus.OPEN, chapter = chapter),
            Seat(classification = "Electrician", category = "Home Services", status = SeatStatus.FILLED, chapter = chapter),
            Seat(classification = "Real Estate Agent", category = "Professional Services", status = SeatStatus.OPEN, chapter = chapter),
            // ... full seat roster
        )
        seatRepository.saveAll(seats)
    }

    private fun seedAdminUser() {
        if (userRepository.findByEmail("admin@example.com") != null) return

        userRepository.save(
            User(
                email = "admin@example.com",
                passwordHash = passwordEncoder.encode("admin123"),
                role = UserRole.PLATFORM_ADMIN
            )
        )
    }
}
```

**Conventions:**
- Always guard with `@Profile("dev", "local")` — never run seed data in production.
- Make seeding idempotent — check if data exists before inserting.
- Use the same entity classes and repositories as production code, not raw SQL.
- Actual seat roster data comes from the project owner — this is configuration, not generated data.

### Flyway-Based Reference Data

For reference data that must exist in all environments (including production), use Flyway repeatable migrations:

```sql
-- R__seed_roles.sql (repeatable migration — re-runs when content changes)
INSERT INTO roles (name, description) VALUES ('MEMBER', 'Standard member')
    ON CONFLICT (name) DO NOTHING;
INSERT INTO roles (name, description) VALUES ('CHAPTER_ADMIN', 'Chapter administrator')
    ON CONFLICT (name) DO NOTHING;
INSERT INTO roles (name, description) VALUES ('PLATFORM_ADMIN', 'Platform administrator')
    ON CONFLICT (name) DO NOTHING;
```

---

## 11. Development Workflow

### Feature Development Cycle (Spring-specific)

```
1. Write acceptance criteria / BDD scenarios
2. Design test levels (unit / controller / integration)
3. Write failing tests (JUnit 5)
4. Write entity + repository + service code
5. Write controller + Thymeleaf template + HTMX
6. Run: ./gradlew test
7. Run: ./gradlew ktlintCheck (or detekt)
8. Refactor while green
9. Run: ./gradlew build (full build including all checks)
```

### Common Commands

```bash
# Development
./gradlew bootRun                           # Start dev server
./gradlew bootRun --args='--spring.profiles.active=dev'  # With profile
./gradlew bootTestRun                       # Start with Testcontainers (no local DB needed)

# Testing
./gradlew test                              # Run all tests
./gradlew test --tests "*.ChapterServiceTest"  # Run specific test class
./gradlew test --tests "*registration*"     # Run by pattern
./gradlew test --info                       # Verbose output

# Quality
./gradlew ktlintCheck                       # Kotlin lint check
./gradlew ktlintFormat                      # Auto-fix lint issues
./gradlew detekt                            # Static analysis
./gradlew dependencyCheckAnalyze            # CVE vulnerability scan

# Build
./gradlew build                             # Full build (compile + test + lint)
./gradlew bootJar                           # Build fat JAR
./gradlew bootBuildImage                    # Build OCI image via Paketo buildpacks

# Database
./gradlew flywayMigrate                     # Run migrations
./gradlew flywayInfo                        # Show migration status
./gradlew flywayClean                       # Drop all objects (dev only!)
./gradlew flywayRepair                      # Fix failed migrations
```

### Gradle Task Configuration

```kotlin
// build.gradle.kts

tasks.withType<Test> {
    useJUnitPlatform()
    jvmArgs("-XX:+EnableDynamicAgentLoading") // Suppress Mockito/ByteBuddy warnings on Java 21+
    systemProperty("spring.profiles.active", "test")
}

tasks.withType<KotlinCompile> {
    compilerOptions {
        freeCompilerArgs.addAll("-Xjsr305=strict")
        jvmTarget.set(JvmTarget.JVM_21)
    }
}

// Separate integration tests from unit tests
tasks.register<Test>("integrationTest") {
    description = "Runs integration tests"
    group = "verification"
    useJUnitPlatform {
        includeTags("integration")
    }
    shouldRunAfter(tasks.test)
}

tasks.named("check") {
    dependsOn("integrationTest")
}
```

### Application Configuration

```yaml
# application.yaml
spring:
  application:
    name: my-app
  threads:
    virtual:
      enabled: true
  jpa:
    open-in-view: false  # Disable OSIV — always. See anti-patterns.
    hibernate:
      ddl-auto: validate
    properties:
      hibernate:
        jdbc:
          time_zone: UTC
  flyway:
    enabled: true
  jackson:
    default-property-inclusion: non_null
    serialization:
      write-dates-as-timestamps: false

server:
  shutdown: graceful

management:
  endpoints:
    web:
      exposure:
        include: health,info,metrics,prometheus
  endpoint:
    health:
      show-details: when-authorized
```

```yaml
# application-dev.yaml
spring:
  jpa:
    show-sql: true
  docker:
    compose:
      enabled: true  # Auto-start compose.yaml
  devtools:
    restart:
      enabled: true
    livereload:
      enabled: true

logging:
  level:
    com.example.app: DEBUG
    org.hibernate.SQL: DEBUG
```

---

## 12. Deployment

### Docker Multi-Stage Build

```dockerfile
# Build stage
FROM eclipse-temurin:21-jdk-alpine AS builder
WORKDIR /app
COPY gradle/ gradle/
COPY gradlew build.gradle.kts settings.gradle.kts gradle/libs.versions.toml ./
RUN ./gradlew dependencies --no-daemon
COPY src/ src/
RUN ./gradlew bootJar --no-daemon -x test

# Runtime stage
FROM eclipse-temurin:21-jre-alpine
WORKDIR /app
RUN addgroup -S app && adduser -S app -G app
COPY --from=builder /app/build/libs/*.jar app.jar
USER app
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
    CMD wget -qO- http://localhost:8080/actuator/health || exit 1

ENTRYPOINT ["java", "-jar", "app.jar"]
```

**Conventions:**
- Multi-stage build: builder stage with JDK, runtime stage with JRE only.
- Copy `gradle/` and build files first for Docker layer caching of dependencies.
- Run as non-root user (`app`).
- Include a `HEALTHCHECK` for container orchestrators.
- Use Eclipse Temurin (Adoptium) base images — production-quality, well-maintained.

### Spring Boot Buildpacks (Alternative)

Spring Boot can build OCI images without a Dockerfile via Cloud Native Buildpacks:

```bash
./gradlew bootBuildImage --imageName=myapp:latest
```

```kotlin
// build.gradle.kts — customize buildpack image
tasks.named<BootBuildImage>("bootBuildImage") {
    imageName.set("registry.example.com/myapp")
    environment.set(mapOf(
        "BP_JVM_VERSION" to "21",
        "BPE_DELIM_JAVA_TOOL_OPTIONS" to " ",
        "BPE_APPEND_JAVA_TOOL_OPTIONS" to "-XX:MaxRAMPercentage=75"
    ))
}
```

### Kubernetes Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  labels:
    app: myapp
spec:
  replicas: 2
  selector:
    matchLabels:
      app: myapp
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: myapp
          image: registry.example.com/myapp:latest
          ports:
            - containerPort: 8080
          env:
            - name: SPRING_PROFILES_ACTIVE
              value: "prod"
            - name: SPRING_DATASOURCE_URL
              valueFrom:
                secretKeyRef:
                  name: myapp-secrets
                  key: database-url
            - name: SPRING_DATASOURCE_USERNAME
              valueFrom:
                secretKeyRef:
                  name: myapp-secrets
                  key: database-username
            - name: SPRING_DATASOURCE_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: myapp-secrets
                  key: database-password
          resources:
            requests:
              memory: "512Mi"
              cpu: "250m"
            limits:
              memory: "1Gi"
              cpu: "1000m"
          readinessProbe:
            httpGet:
              path: /actuator/health/readiness
              port: 8080
            initialDelaySeconds: 15
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /actuator/health/liveness
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
          startupProbe:
            httpGet:
              path: /actuator/health
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 5
            failureThreshold: 30
---
apiVersion: v1
kind: Service
metadata:
  name: myapp
spec:
  selector:
    app: myapp
  ports:
    - port: 80
      targetPort: 8080
  type: ClusterIP
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: myapp
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
    - hosts:
        - app.example.com
      secretName: myapp-tls
  rules:
    - host: app.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: myapp
                port:
                  number: 80
```

### CI/CD Pipeline (GitHub Actions)

```yaml
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
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_DB: testdb
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          distribution: temurin
          java-version: '21'
          cache: gradle
      - name: Lint
        run: ./gradlew ktlintCheck detekt
      - name: Compile
        run: ./gradlew compileKotlin compileTestKotlin
      - name: Test
        run: ./gradlew test
        env:
          SPRING_DATASOURCE_URL: jdbc:postgresql://localhost:5432/testdb
          SPRING_DATASOURCE_USERNAME: test
          SPRING_DATASOURCE_PASSWORD: test
      - name: Integration Test
        run: ./gradlew integrationTest
      - name: Build
        run: ./gradlew bootJar

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          distribution: temurin
          java-version: '21'
          cache: gradle
      - name: Build Image
        run: ./gradlew bootBuildImage --imageName=registry.example.com/myapp:${{ github.sha }}
      - name: Push Image
        run: docker push registry.example.com/myapp:${{ github.sha }}
      - name: Deploy to Kubernetes
        run: |
          kubectl set image deployment/myapp myapp=registry.example.com/myapp:${{ github.sha }}
          kubectl rollout status deployment/myapp --timeout=300s
```

---

## 13. Security

### Security Headers

Spring Security 6 configures security headers declaratively in the `SecurityFilterChain`. The configuration in Section 4 covers the essentials. Additional hardening:

```kotlin
http {
    headers {
        contentSecurityPolicy {
            policyDirectives = buildString {
                append("default-src 'self'; ")
                append("script-src 'self' 'unsafe-inline' https://unpkg.com; ") // HTMX CDN
                append("style-src 'self' 'unsafe-inline'; ")
                append("img-src 'self' data:; ")
                append("font-src 'self'; ")
                append("connect-src 'self'; ")
                append("frame-ancestors 'self'")
            }
        }
        httpStrictTransportSecurity {
            includeSubDomains = true
            maxAgeInSeconds = 31536000
            preload = true
        }
        referrerPolicy {
            policy = ReferrerPolicyHeaderWriter.ReferrerPolicy.STRICT_ORIGIN_WHEN_CROSS_ORIGIN
        }
        permissionsPolicy {
            policy = "camera=(), microphone=(), geolocation=()"
        }
        frameOptions { deny() }
    }
}
```

### Input Validation

Validate at every boundary:

```kotlin
// 1. DTO-level validation (Jakarta Bean Validation)
data class VisitorRegistrationDto(
    @field:NotBlank @field:Size(max = 200) val name: String,
    @field:NotBlank @field:Email val email: String,
    @field:NotBlank @field:Pattern(regexp = "^\\+?[0-9\\-\\s()]+$") val phone: String
)

// 2. Service-level validation (business rules)
fun register(dto: VisitorRegistrationDto): RegistrationResult {
    if (visitorRepository.existsByEmail(dto.email)) {
        return RegistrationResult.DuplicateEmail(dto.email)
    }
    // ...
}

// 3. Database-level constraints (Flyway migrations)
// NOT NULL, UNIQUE, CHECK constraints, foreign keys
```

### Secrets Management

```yaml
# application-prod.yaml — NEVER commit secrets
spring:
  datasource:
    url: ${DATABASE_URL}
    username: ${DATABASE_USERNAME}
    password: ${DATABASE_PASSWORD}

external:
  api:
    token: ${EXTERNAL_API_TOKEN}
```

**Conventions:**
- All secrets come from environment variables in production.
- Use Kubernetes Secrets or a secrets manager (Vault, AWS Secrets Manager) to inject them.
- Never commit `application-prod.yaml` with real values. Use placeholder `${VAR}` syntax.
- For development, use `application-dev.yaml` with local-only credentials (not committed).
- Spring Boot's `@ConfigurationProperties` with `@Validated` ensures required config is present at startup.

### CSRF Protection

CSRF is enabled by default in Spring Security. For Thymeleaf forms, the token is injected automatically:

```html
<!-- Thymeleaf auto-injects the CSRF hidden input -->
<form th:action="@{/visitors/register}" method="post">
    <!-- Hidden CSRF input added automatically by Thymeleaf + Spring Security -->
</form>
```

For HTMX and AJAX, see the meta-tag approach in Section 4.

### Rate Limiting

Use Spring Boot's built-in rate limiting or a library like Bucket4j:

```kotlin
@Configuration
class RateLimitConfig {

    @Bean
    fun rateLimiterFilter(): FilterRegistrationBean<RateLimitFilter> {
        val registration = FilterRegistrationBean(RateLimitFilter())
        registration.addUrlPatterns("/api/*", "/visitors/register")
        return registration
    }
}

class RateLimitFilter : OncePerRequestFilter() {

    private val buckets = ConcurrentHashMap<String, Bucket>()

    override fun doFilterInternal(
        request: HttpServletRequest,
        response: HttpServletResponse,
        filterChain: FilterChain
    ) {
        val clientIp = request.remoteAddr
        val bucket = buckets.computeIfAbsent(clientIp) { createBucket() }

        if (bucket.tryConsume(1)) {
            filterChain.doFilter(request, response)
        } else {
            response.status = HttpStatus.TOO_MANY_REQUESTS.value()
            response.writer.write("Rate limit exceeded")
        }
    }

    private fun createBucket(): Bucket {
        return Bucket.builder()
            .addLimit(Bandwidth.classic(60, Refill.intervally(60, Duration.ofMinutes(1))))
            .build()
    }
}
```

---

## 14. Coverage Enforcement

### JaCoCo Configuration

```kotlin
// build.gradle.kts
plugins {
    jacoco
}

jacoco {
    toolVersion = "0.8.12"
}

tasks.jacocoTestReport {
    dependsOn(tasks.test)
    reports {
        xml.required.set(true)
        html.required.set(true)
        csv.required.set(false)
    }
    classDirectories.setFrom(
        files(classDirectories.files.map {
            fileTree(it) {
                exclude(
                    "**/config/**",
                    "**/Application*",
                    "**/dto/**",
                    "**/*Dto*"
                )
            }
        })
    )
}

tasks.jacocoTestCoverageVerification {
    violationRules {
        rule {
            limit {
                minimum = "0.80".toBigDecimal()
            }
        }
        rule {
            element = "CLASS"
            excludes = listOf(
                "*.config.*",
                "*.Application",
                "*.dto.*"
            )
            limit {
                counter = "LINE"
                minimum = "0.80".toBigDecimal()
            }
        }
    }
}

tasks.named("check") {
    dependsOn("jacocoTestCoverageVerification")
}
```

**Commands:**
```bash
./gradlew test jacocoTestReport              # Generate coverage report
./gradlew jacocoTestCoverageVerification     # Enforce minimum coverage
# HTML report at: build/reports/jacoco/test/html/index.html
```

Target is 100% (per CLAUDE.md core rules). The `minimum` in `jacocoTestCoverageVerification` is the hard gate — CI fails below this threshold.

### Coverage Exclusions

Exclude from coverage enforcement (not from testing):
- `@SpringBootApplication` entry point
- `@Configuration` classes (tested implicitly by integration tests)
- DTOs (data-only classes with no logic)
- Generated code (Lombok, MapStruct)

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with `<fieldset>` + `<legend>` |
| **labels** | Top-aligned, visible `<label>`, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | Inline + error summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Reserve My Free Visit" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | No unnecessary client-side validation, debounce HTMX triggers if needed |

**Spring + Thymeleaf form pattern:**

```html
<form th:action="@{/visitors/register}"
      th:object="${visitorDto}"
      method="post"
      novalidate>

    <!-- Error summary for screen readers -->
    <div th:if="${#fields.hasAnyErrors()}"
         class="alert alert-error mb-6"
         role="alert"
         aria-live="polite">
        <ul>
            <li th:each="err : ${#fields.allErrors()}" th:text="${err}">Error</li>
        </ul>
    </div>

    <fieldset>
        <legend class="text-lg font-semibold mb-4">Your Information</legend>

        <div class="form-control w-full mb-4">
            <label class="label" for="name">
                <span class="label-text">Full Name</span>
            </label>
            <input type="text" id="name" th:field="*{name}"
                   th:errorclass="input-error"
                   class="input input-bordered w-full"
                   autocomplete="name" required />
            <label class="label" th:if="${#fields.hasErrors('name')}">
                <span class="label-text-alt text-error" th:errors="*{name}">Error</span>
            </label>
        </div>

        <div class="form-control w-full mb-4">
            <label class="label" for="email">
                <span class="label-text">Email</span>
            </label>
            <input type="email" id="email" th:field="*{email}"
                   th:errorclass="input-error"
                   class="input input-bordered w-full"
                   autocomplete="email" required />
            <label class="label" th:if="${#fields.hasErrors('email')}">
                <span class="label-text-alt text-error" th:errors="*{email}">Error</span>
            </label>
        </div>

        <div class="form-control w-full mb-4">
            <label class="label" for="phone">
                <span class="label-text">Phone</span>
            </label>
            <input type="tel" id="phone" th:field="*{phone}"
                   th:errorclass="input-error"
                   class="input input-bordered w-full"
                   autocomplete="tel" required />
            <label class="label" th:if="${#fields.hasErrors('phone')}">
                <span class="label-text-alt text-error" th:errors="*{phone}">Error</span>
            </label>
        </div>
    </fieldset>

    <button type="submit" class="btn btn-primary w-full h-12 mt-4">
        Reserve My Free Visit
    </button>
</form>
```

**Thymeleaf-specific notes:**
- Thymeleaf's `th:field` binds `name`, `id`, and `value` automatically.
- `th:errorclass` adds CSS classes when the field has validation errors.
- `th:errors` renders field-specific error messages from `BindingResult`.
- `th:if="${#fields.hasAnyErrors()}"` shows the error summary.
- CSRF token is injected automatically by Thymeleaf's Spring Security integration — no manual hidden field needed.

---

## 16. Anti-Patterns (Spring-specific)

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | **Open Session in View (OSIV)** — leaving `spring.jpa.open-in-view=true` (the default) | Set `spring.jpa.open-in-view=false`. OSIV holds a database connection for the entire HTTP request, including view rendering. Use `JOIN FETCH` or DTOs to load data eagerly in the service layer. |
| 2 | **`data class` for JPA entities** — using Kotlin data classes for Hibernate entities | Use regular `class` for entities. `data class` generates `equals`/`hashCode` from all fields, breaking Hibernate identity semantics and lazy loading proxies. |
| 3 | **N+1 queries** — loading collections in loops without fetch joins | Use `JOIN FETCH` in JPQL, `@EntityGraph`, or Spring Data `@Query` with explicit joins. Enable Hibernate's `hibernate.generate_statistics=true` in dev to detect. |
| 4 | **Business logic in controllers** — complex conditionals, multi-repository calls in `@Controller` | Move business logic to `@Service` classes. Controllers should validate input, call service, return response. |
| 5 | **`@Autowired` field injection** — `@Autowired private lateinit var repo: Repository` | Use constructor injection (Kotlin primary constructor). Field injection hides dependencies, prevents immutability, and makes testing harder. |
| 6 | **Extending `WebSecurityConfigurerAdapter`** — deprecated since Spring Security 5.7 | Use `SecurityFilterChain` bean in a `@Configuration` class. The adapter pattern is removed in Spring Security 6. |
| 7 | **`RestTemplate` for new code** — using the deprecated HTTP client | Use `RestClient` (synchronous) or `WebClient` (reactive) or `@HttpExchange` (declarative). `RestTemplate` is in maintenance mode. |
| 8 | **Catching `Exception` broadly** — `catch (Exception e)` in service methods | Catch specific exceptions. Use `@RestControllerAdvice` for centralized exception handling. |
| 9 | **Missing `@Transactional` boundaries** — relying on implicit transactions | Annotate service methods with `@Transactional`. For read-only operations, use `@Transactional(readOnly = true)` — enables Hibernate query optimizations. |
| 10 | **`spring.jpa.hibernate.ddl-auto=update` in production** — letting Hibernate modify the schema | Use Flyway or Liquibase for all schema changes. Set `ddl-auto=validate` in production. `update` can silently corrupt data or miss destructive changes. |
| 11 | **Blocking calls in WebFlux** — calling JDBC, `Thread.sleep`, or blocking I/O in reactive pipelines | Use R2DBC for database access in reactive code. Use `.subscribeOn(Schedulers.boundedElastic())` if blocking is unavoidable. Or just use Spring MVC with virtual threads instead. |
| 12 | **God `@Configuration` class** — one file with 500 lines of beans | Split into focused config classes: `SecurityConfig`, `JpaConfig`, `AsyncConfig`, `WebConfig`. |
| 13 | **Testing with `@SpringBootTest` everywhere** — loading full context for unit tests | Use `@WebMvcTest` for controller tests, `@DataJpaTest` for repository tests, plain JUnit for services. Reserve `@SpringBootTest` for integration tests. |
| 14 | **Hardcoded URLs in templates** — `<a href="/chapters/1">` | Use `th:href="@{/chapters/{id}(id=${chapter.id})}"`. Thymeleaf URL expressions handle context paths and encoding. |
| 15 | **Missing `@field:` on Kotlin validation annotations** — `@NotBlank val name: String` | Use `@field:NotBlank val name: String`. Without `@field:`, the annotation targets the constructor parameter, not the field, and Bean Validation ignores it. |
| 16 | **Returning entities from controllers** — exposing JPA entities as JSON/HTML model objects | Use DTOs for all controller responses. Entities expose internal structure, lazy-loading proxies, and bidirectional relationships that cause infinite recursion. |
| 17 | **`@RequestMapping` without HTTP method** — `@RequestMapping("/users")` on handler methods | Use `@GetMapping`, `@PostMapping`, etc. Unmapped methods accept all HTTP methods, which is a security risk. |
| 18 | **Missing CSRF in HTMX requests** — forgetting to send CSRF token in HTMX headers | Include CSRF meta tag and configure `htmx:configRequest` listener (see Section 4). Without it, all HTMX POST/PUT/DELETE fail with 403. |
| 19 | **Secrets in `application.yaml`** — committing database passwords, API keys in config files | Use environment variables (`${DB_PASSWORD}`), Spring Cloud Config, or Kubernetes Secrets. Never commit real credentials. |
| 20 | **No database indexes on foreign keys** — relying on JPA-generated schema without indexes | Add indexes in Flyway migrations for all foreign keys, `WHERE` clause columns, unique constraints, and sort columns. JPA `@Index` annotation is not enough. |
| 21 | **`@Component` on everything** — making every class a Spring bean | Only register classes as beans if they need dependency injection or lifecycle management. Utility classes, mappers, and value objects should be plain classes. |
| 22 | **Ignoring virtual threads** — using large thread pools on Spring Boot 3.2+ with Java 21 | Enable `spring.threads.virtual.enabled=true`. Virtual threads eliminate the need for complex thread pool tuning for I/O-bound workloads. |
| 23 | **`@Lazy` to break circular dependencies** — using `@Lazy` instead of fixing the design | Refactor to break the cycle. Extract shared logic into a new service. Circular dependencies indicate a design problem. |
| 24 | **`System.out.println` for logging** — using print statements instead of SLF4J | Use `LoggerFactory.getLogger(javaClass)` or Kotlin's logger extension. Structured logging with proper levels is essential for production debugging. |
| 25 | **Missing graceful shutdown** — not configuring `server.shutdown=graceful` | Set `server.shutdown=graceful` and `spring.lifecycle.timeout-per-shutdown-phase=30s`. Without it, in-flight requests are killed during deployment. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Spring patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:spring&title=[Spring]%20)**

Use the `patterns:spring` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
