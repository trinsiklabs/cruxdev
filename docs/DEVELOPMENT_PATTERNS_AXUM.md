# Development Patterns — Rust + Axum Stack

Rust / Axum / Tokio / Tower / SQLx (or SeaORM)

This document captures stack-specific patterns, conventions, and decisions for Rust web services built on Axum. It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **This file** — stack-specific patterns. How we structure Axum routers, write extractors, test with cargo test, deploy with Docker, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's tested in CI. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Rust | 1.82+ (2024 edition) | MSRV pinned in `rust-toolchain.toml` |
| Axum | 0.8+ | Web framework — extractors, routing, state |
| Tokio | 1.x | Async runtime — `rt-multi-thread`, `macros` |
| Tower | 0.5+ | Middleware layer — timeout, rate-limit, compression |
| Tower-HTTP | 0.6+ | HTTP-specific Tower middleware — CORS, tracing, compression |
| Hyper | 1.x | HTTP implementation (used by Axum internally) |
| SQLx | 0.8+ | Async database driver — compile-time checked queries |
| SeaORM | 1.x | Alternative ORM layer (if compile-time queries not needed) |
| PostgreSQL | 15+ | Primary datastore |
| Redis | 7+ | Session store, caching (optional) |
| Serde | 1.x | Serialization/deserialization |
| Tracing | 0.1.x | Structured logging and diagnostics |
| Tracing-subscriber | 0.3.x | Log formatting and output |
| cargo-llvm-cov | latest | Code coverage via LLVM instrumentation |
| Docker | 24+ | Containerized deployment |

### Version Constraint Policy

Use exact minor version constraints in `Cargo.toml` for core dependencies:

```toml
# Good — pinned to minor, allows patch updates
axum = "0.8"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace", "compression-gzip"] }

# Bad — wildcard allows breaking changes
axum = "*"

# Bad — too tight, blocks patch fixes
axum = "=0.8.1"
```

### Rust Toolchain Pinning

Pin the toolchain at the project root so every developer and CI gets the same compiler:

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.82"
components = ["rustfmt", "clippy"]
```

### Feature Flags

Only enable features you actually use. Tokio's `full` feature is acceptable for applications (not libraries):

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# For libraries, be specific:
# tokio = { version = "1", features = ["rt", "macros"] }
```

---

## 2. Project Structure

### Binary Application Layout

```
src/
├── main.rs                  # Entry point — tokio::main, tracing init, server start
├── lib.rs                   # Re-exports, app-wide types
├── config.rs                # Configuration loading (env vars, files)
├── app_state.rs             # Shared application state (AppState struct)
├── router.rs                # Top-level Router assembly
├── routes/                  # Route modules (one per domain)
│   ├── mod.rs
│   ├── health.rs            # GET /health, GET /ready
│   ├── users.rs             # /api/v1/users routes
│   ├── auth.rs              # /api/v1/auth routes
│   └── admin.rs             # /api/v1/admin routes
├── handlers/                # Request handlers (business logic entry points)
│   ├── mod.rs
│   ├── users.rs
│   ├── auth.rs
│   └── admin.rs
├── models/                  # Domain models and database types
│   ├── mod.rs
│   ├── user.rs
│   └── session.rs
├── db/                      # Database queries and repository layer
│   ├── mod.rs
│   ├── users.rs
│   └── sessions.rs
├── extractors/              # Custom Axum extractors
│   ├── mod.rs
│   ├── auth.rs              # AuthUser extractor
│   ├── json.rs              # Validated JSON extractor
│   └── pagination.rs        # Pagination query params
├── middleware/               # Tower middleware and layers
│   ├── mod.rs
│   ├── auth.rs              # JWT/session verification layer
│   ├── request_id.rs        # Request ID injection
│   └── logging.rs           # Request/response logging
├── errors/                  # Error types and IntoResponse impls
│   ├── mod.rs
│   └── api_error.rs         # Unified API error type
├── services/                # Business logic (pure, testable)
│   ├── mod.rs
│   ├── auth_service.rs
│   └── user_service.rs
└── utils/                   # Shared utilities
    ├── mod.rs
    └── crypto.rs
```

**Conventions:**
- `routes/` defines which paths map to which handlers. No business logic.
- `handlers/` contains the async functions that Axum calls. They extract, validate, delegate to services, and return responses.
- `services/` contains business logic. No Axum types. No HTTP concepts. Pure domain logic that takes domain types in and returns `Result<DomainType, DomainError>`.
- `db/` contains all database queries. Services call into `db/`, never construct queries inline.
- `models/` contains the structs that bridge the database and the API. Serde derives for both `sqlx::FromRow` and `serde::Serialize`.
- `extractors/` contains custom `FromRequestParts` / `FromRequest` implementations.
- `middleware/` contains Tower `Layer` and `Service` implementations.
- `errors/` contains the unified error type that implements `IntoResponse`.

### Workspace Layout (Multi-Crate)

For larger projects, use a Cargo workspace:

```
project-root/
├── Cargo.toml               # [workspace] definition
├── crates/
│   ├── api/                  # Axum application (binary)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── domain/               # Business logic (library, no Axum deps)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── db/                   # Database layer (library)
│   │   ├── Cargo.toml
│   │   └── src/
│   └── shared/               # Shared types, errors, config
│       ├── Cargo.toml
│       └── src/
├── migrations/               # SQLx migrations
├── tests/                    # Integration tests
└── docker/
    └── Dockerfile
```

**Convention:** The `domain` crate must never depend on `axum`, `sqlx`, or any infrastructure crate. It defines traits that `db` and `api` implement. This enforces the dependency inversion principle at the crate level.

### Test Structure

```
tests/                        # Integration tests (separate binary)
├── common/
│   ├── mod.rs                # Shared test helpers
│   ├── fixtures.rs           # Test data factories
│   └── test_app.rs           # Test application builder
├── api/
│   ├── health_test.rs
│   ├── users_test.rs
│   └── auth_test.rs
└── db/
    └── users_test.rs
```

Unit tests live inline in `src/` modules using `#[cfg(test)] mod tests { ... }`.

---

## 3. Router & Route Patterns

### Router Assembly

Build the router from composable sub-routers using `Router::merge` and `Router::nest`:

```rust
use axum::{Router, routing::{get, post, put, delete}};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(health_routes())
        .nest("/api/v1", api_routes())
        .with_state(state)
}

fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::liveness))
        .route("/ready", get(health::readiness))
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .merge(admin_routes())
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(handlers::users::list))
        .route("/users", post(handlers::users::create))
        .route("/users/{id}", get(handlers::users::get_by_id))
        .route("/users/{id}", put(handlers::users::update))
        .route("/users/{id}", delete(handlers::users::delete))
}
```

**Axum 0.8 change:** Path parameters use `{id}` syntax instead of `:id`. The old `:id` syntax is removed.

### Route Grouping with Middleware

Apply middleware selectively using `Router::layer`:

```rust
fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/admin/users", get(handlers::admin::list_users))
        .route("/admin/settings", put(handlers::admin::update_settings))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::require_admin,
        ))
}
```

### Path Parameters (Axum 0.8)

Axum 0.8 uses `Path` extractor with `{param}` syntax in routes:

```rust
use axum::extract::Path;
use uuid::Uuid;

async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    // user_id is already parsed as Uuid
}

// Multiple path params
async fn get_user_post(
    Path((user_id, post_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    // ...
}
```

### Method Routing

Use Axum's `MethodRouter` for multiple methods on the same path:

```rust
use axum::routing::{get, post, put, delete, MethodRouter};

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users",
            get(handlers::users::list)
            .post(handlers::users::create)
        )
        .route("/users/{id}",
            get(handlers::users::get_by_id)
            .put(handlers::users::update)
            .delete(handlers::users::delete)
        )
}
```

---

## 4. State Management

### Application State

Define a single `AppState` struct that holds all shared resources:

```rust
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub redis: Option<deadpool_redis::Pool>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, anyhow::Error> {
        let db = PgPool::connect(&config.database_url).await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self {
            db,
            config: Arc::new(config),
            redis: None,
            http_client: reqwest::Client::new(),
        })
    }
}
```

**Conventions:**
- `AppState` must implement `Clone`. All inner types must be cheaply cloneable (`Arc`, `Pool`, etc.).
- Never put `Mutex<T>` directly in `AppState` for request-scoped data. Use extractors instead.
- Configuration is wrapped in `Arc<Config>` — parsed once at startup, shared read-only.
- Database pool (`PgPool`) is already `Arc` internally; no extra wrapping needed.

### Accessing State in Handlers

```rust
use axum::extract::State;

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = db::users::list_all(&state.db).await?;
    Ok(Json(users))
}
```

### Sub-State Pattern

When handlers only need a subset of state, use `FromRef`:

```rust
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

// Handler only receives the pool
async fn list_users(
    State(db): State<PgPool>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = db::users::list_all(&db).await?;
    Ok(Json(users))
}
```

---

## 5. Extractors

### Built-in Extractors

| Extractor | Source | Use For |
|---|---|---|
| `Path<T>` | URL path segments | Resource IDs, slugs |
| `Query<T>` | Query string | Pagination, filtering, sorting |
| `Json<T>` | Request body (JSON) | Create/update payloads |
| `State<T>` | Application state | DB pool, config, shared services |
| `HeaderMap` | All headers | Low-level header access |
| `TypedHeader<T>` | Typed header | `Authorization`, `Content-Type` |
| `Extension<T>` | Request extensions | Middleware-injected data |
| `ConnectInfo<T>` | Connection info | Client IP address |
| `OriginalUri` | Original URI | Pre-routing URI (for logging) |

### Extractor Ordering

Axum applies extractors in argument order. The `Body`-consuming extractor (`Json<T>`) must come last:

```rust
// Correct — Path before Json
async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, ApiError> { ... }

// Wrong — Json before Path won't compile (body consumed)
async fn update_user(
    Json(payload): Json<UpdateUserRequest>,
    Path(user_id): Path<Uuid>,
) -> ... { ... }
```

### Custom Auth Extractor

Implement `FromRequestParts` for authentication:

```rust
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;

pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: Role,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Unauthorized("Missing authorization header".into()))?;

        // Validate the token
        let app_state = AppState::from_ref(state);
        let claims = validate_jwt(bearer.token(), &app_state.config.jwt_secret)
            .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

        Ok(AuthUser {
            user_id: claims.sub,
            email: claims.email,
            role: claims.role,
        })
    }
}
```

Usage in handlers:

```rust
async fn get_profile(auth: AuthUser) -> Result<Json<UserProfile>, ApiError> {
    // auth.user_id, auth.email, auth.role are available
    Ok(Json(UserProfile { /* ... */ }))
}

async fn admin_action(auth: AuthUser) -> Result<(), ApiError> {
    if auth.role != Role::Admin {
        return Err(ApiError::Forbidden("Admin access required".into()));
    }
    // ...
    Ok(())
}
```

### Validated JSON Extractor

Wrap `Json` with validation using the `validator` crate:

```rust
use axum::{extract::rejection::JsonRejection, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| ApiError::BadRequest(e.body_text()))?;

        value.validate()
            .map_err(|e| ApiError::ValidationError(e))?;

        Ok(ValidatedJson(value))
    }
}
```

Usage:

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    // payload is already validated
    let user = services::auth::register(&state, payload).await?;
    Ok(Json(user))
}
```

### Pagination Extractor

```rust
#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

impl Pagination {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.per_page
    }

    pub fn limit(&self) -> u32 {
        self.per_page.min(100) // Hard cap
    }
}

async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<User>>, ApiError> {
    let users = db::users::list_paginated(
        &state.db,
        pagination.limit(),
        pagination.offset(),
    ).await?;
    Ok(Json(PaginatedResponse {
        data: users,
        page: pagination.page,
        per_page: pagination.per_page,
    }))
}
```

---

## 6. Error Handling

### Unified Error Type

Define a single error type that implements `IntoResponse`:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    // Client errors
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    ValidationError(validator::ValidationErrors),
    UnprocessableEntity(String),

    // Server errors
    Internal(anyhow::Error),
    DatabaseError(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg.clone())
            }
            ApiError::ValidationError(errors) => {
                let body = json!({
                    "error": "Validation failed",
                    "details": errors.field_errors(),
                });
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response();
            }
            ApiError::Internal(err) => {
                tracing::error!(error = %err, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            ApiError::DatabaseError(err) => {
                tracing::error!(error = %err, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = json!({ "error": message });
        (status, Json(body)).into_response()
    }
}
```

### Error Conversion with `From`

Implement `From` for automatic `?` operator usage:

```rust
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound("Resource not found".to_string())
            }
            sqlx::Error::Database(ref db_err) => {
                if db_err.is_unique_violation() {
                    ApiError::Conflict("Resource already exists".to_string())
                } else {
                    ApiError::DatabaseError(err)
                }
            }
            _ => ApiError::DatabaseError(err),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err)
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        ApiError::Unauthorized("Invalid token".to_string())
    }
}
```

### Handler Return Types

Every handler returns `Result<impl IntoResponse, ApiError>`:

```rust
async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
    let user = db::users::find_by_id(&state.db, user_id)
        .await?  // sqlx::Error auto-converts to ApiError
        .ok_or_else(|| ApiError::NotFound(format!("User {user_id} not found")))?;

    Ok(Json(user))
}
```

### Never Leak Internal Details

The error response for 500s must never include stack traces, query text, or internal paths. Log them server-side with `tracing::error!`, return a generic message to the client.

---

## 7. Middleware (Tower Layers)

### Standard Middleware Stack

Apply middleware in the correct order (outermost runs first):

```rust
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer, PropagateRequestIdLayer},
    limit::RequestBodyLimitLayer,
};
use tower::ServiceBuilder;
use std::time::Duration;

pub fn build_router(state: AppState) -> Router {
    let middleware_stack = ServiceBuilder::new()
        // Outermost — runs first on request, last on response
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let request_id = request.headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        request_id = %request_id,
                    )
                })
        )
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024)) // 2 MB
        .layer(CompressionLayer::new())
        .layer(build_cors_layer(&state.config));
        // Innermost — runs last on request, first on response

    Router::new()
        .merge(health_routes())
        .nest("/api/v1", api_routes())
        .layer(middleware_stack)
        .with_state(state)
}
```

### CORS Configuration

```rust
use tower_http::cors::{CorsLayer, Any};
use axum::http::{HeaderName, Method};

fn build_cors_layer(config: &Config) -> CorsLayer {
    if config.is_development() {
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(config.allowed_origins.clone())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
            ])
            .allow_headers([
                HeaderName::from_static("content-type"),
                HeaderName::from_static("authorization"),
                HeaderName::from_static("x-request-id"),
            ])
            .allow_credentials(true)
            .max_age(Duration::from_secs(3600))
    }
}
```

### Custom Middleware with `from_fn`

For simple middleware, use `axum::middleware::from_fn`:

```rust
use axum::middleware::{self, Next};
use axum::http::Request;

async fn require_auth<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing authorization".into()))?;

    let claims = validate_jwt(auth_header, &state.config.jwt_secret)?;

    // Inject claims into request extensions
    let mut request = request;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Apply to specific routes
fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", get(handlers::users::profile))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ))
}
```

### Tower Service Middleware (Full Control)

For reusable middleware with configuration, implement the Tower `Layer` and `Service` traits:

```rust
use tower::{Layer, Service};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;

#[derive(Clone)]
pub struct RateLimitLayer {
    max_requests: u64,
    window: Duration,
}

impl RateLimitLayer {
    pub fn new(max_requests: u64, window: Duration) -> Self {
        Self { max_requests, window }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            max_requests: self.max_requests,
            window: self.window,
            // ... state for tracking request counts
        }
    }
}
```

### Security Headers Middleware

```rust
use axum::{http::header, middleware::Next, response::Response};

pub async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        "DENY".parse().unwrap(),
    );
    headers.insert(
        header::REFERRER_POLICY,
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'; frame-ancestors 'none'".parse().unwrap(),
    );

    response
}
```

---

## 8. Database Patterns (SQLx)

### Connection Pool Setup

```rust
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::str::FromStr;

pub async fn create_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    let connect_options = PgConnectOptions::from_str(&config.database_url)?
        .application_name("my-service");

    PgPoolOptions::new()
        .max_connections(config.db_max_connections.unwrap_or(10))
        .min_connections(config.db_min_connections.unwrap_or(2))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect_with(connect_options)
        .await
}
```

### Compile-Time Checked Queries

Use `sqlx::query!` and `sqlx::query_as!` for compile-time SQL verification:

```rust
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, name, role as "role: Role", created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, input: &CreateUserInput) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, name, password_hash, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, name, role as "role: Role", created_at, updated_at
        "#,
        Uuid::new_v4(),
        input.email,
        input.name,
        input.password_hash,
        input.role as Role,
    )
    .fetch_one(pool)
    .await
}
```

**Convention:** The `"column: Type"` annotation (e.g., `"role: Role"`) is required for custom enum types so SQLx knows how to deserialize them.

### Migrations

```bash
# Create a new migration
sqlx migrate add create_users_table

# Run pending migrations
sqlx migrate run

# Revert last migration (development only)
sqlx migrate revert
```

Migration files live in `migrations/` at the project root:

```sql
-- migrations/20240101000000_create_users_table.sql
CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_users_email ON users (email);
```

**Convention:** Never edit a committed migration. Write a new corrective migration instead.

### Transactions

```rust
pub async fn transfer_ownership(
    pool: &PgPool,
    from_id: Uuid,
    to_id: Uuid,
    resource_id: Uuid,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE resources SET owner_id = $1 WHERE id = $2 AND owner_id = $3",
        to_id, resource_id, from_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT INTO audit_log (action, resource_id, from_user, to_user) VALUES ($1, $2, $3, $4)",
        "transfer_ownership", resource_id, from_id, to_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

### SeaORM Alternative

If compile-time checked queries are not needed (e.g., dynamic queries, rapid prototyping), use SeaORM:

```rust
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};

pub async fn find_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

pub async fn create(
    db: &DatabaseConnection,
    input: CreateUserInput,
) -> Result<user::Model, DbErr> {
    let model = user::ActiveModel {
        email: Set(input.email),
        name: Set(input.name),
        password_hash: Set(input.password_hash),
        ..Default::default()
    };

    model.insert(db).await
}
```

**Choosing between SQLx and SeaORM:**
- SQLx: compile-time query checks, raw SQL control, zero runtime overhead. Best for performance-critical services.
- SeaORM: ORM ergonomics, dynamic query building, entity generation. Best for CRUD-heavy applications.
- Never mix both in the same project without explicit justification.

---

## 9. Authentication & Authorization

### JWT Authentication

```rust
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,       // user ID
    pub email: String,
    pub role: Role,
    pub exp: usize,      // expiration (Unix timestamp)
    pub iat: usize,      // issued at
}

pub fn create_token(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        role: user.role,
        exp: (now + chrono::Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
```

### Password Hashing

Use `argon2` (preferred) or `bcrypt`:

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
```

### Role-Based Authorization

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
}

/// Extractor that requires a specific minimum role
pub struct RequireRole<const ROLE: u8>;

impl<S, const ROLE: u8> FromRequestParts<S> for RequireRole<ROLE>
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth = AuthUser::from_request_parts(parts, state).await?;
        let required = Role::from_u8(ROLE);
        if auth.role < required {
            return Err(ApiError::Forbidden(
                format!("Requires {} role", required),
            ));
        }
        Ok(Self)
    }
}
```

---

## 10. Tracing & Observability

### Tracing Setup

Initialize tracing in `main.rs`:

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Default levels: info for our code, warn for dependencies
            "my_app=debug,tower_http=debug,axum=info,sqlx=warn".into()
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()
            .json()                    // JSON output for production
            .with_target(true)         // Include module path
            .with_thread_ids(true)     // Include thread IDs
            .with_file(true)           // Include source file
            .with_line_number(true))   // Include line number
        .init();
}
```

### Structured Logging in Handlers

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(state), fields(user_id = %user_id))]
async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
    info!("Fetching user");

    let user = db::users::find_by_id(&state.db, user_id)
        .await?
        .ok_or_else(|| {
            warn!("User not found");
            ApiError::NotFound(format!("User {user_id} not found"))
        })?;

    info!(email = %user.email, "User found");
    Ok(Json(user))
}
```

**Conventions:**
- Use `#[instrument]` on every handler and service function.
- `skip(state)` to avoid logging the entire app state.
- Add `fields(...)` for key identifiers (user_id, request_id).
- Use `info!` for normal operations, `warn!` for recoverable issues, `error!` for failures.
- Never log passwords, tokens, or PII at any level.

### Request/Response Tracing with Tower-HTTP

The `TraceLayer` from section 7 automatically logs:
- Request method, URI, version
- Response status code
- Request duration
- Request ID (if `SetRequestIdLayer` is used)

### Health Check Endpoints

```rust
async fn liveness() -> StatusCode {
    StatusCode::OK
}

async fn readiness(State(state): State<AppState>) -> StatusCode {
    // Check database connectivity
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = %e, "Database health check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
```

### Metrics (Optional)

For Prometheus metrics, use `metrics` + `metrics-exporter-prometheus`:

```rust
use metrics::{counter, histogram};
use std::time::Instant;

pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    counter!("http_requests_total", "method" => method.clone(), "path" => path.clone(), "status" => status).increment(1);
    histogram!("http_request_duration_seconds", "method" => method, "path" => path).record(duration);

    response
}
```

---

## 11. Testing Patterns

### Test Pyramid (Axum-specific)

```
        /\
       /  \          E2E (external HTTP tests against running server)
      /    \
     /------\
    /        \        Integration Tests (test server + real DB)
   /          \       Handler → service → DB round-trip, auth flows
  /------------\
 /              \      Unit Tests (cargo test, inline #[cfg(test)])
/                \     Pure functions, validation, serialization, error mapping
/------------------\
```

### Unit Tests (Inline)

```rust
// src/services/auth_service.rs

pub fn validate_password_strength(password: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if password.len() < 8 { errors.push("Must be at least 8 characters".into()); }
    if !password.chars().any(|c| c.is_uppercase()) { errors.push("Must contain uppercase".into()); }
    if !password.chars().any(|c| c.is_ascii_digit()) { errors.push("Must contain a digit".into()); }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_password_passes() {
        assert!(validate_password_strength("SecurePass1").is_ok());
    }

    #[test]
    fn test_short_password_fails() {
        let result = validate_password_strength("Ab1");
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("8 characters")));
    }

    #[test]
    fn test_no_uppercase_fails() {
        let result = validate_password_strength("lowercase123");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_digit_fails() {
        let result = validate_password_strength("NoDigitsHere");
        assert!(result.is_err());
    }
}
```

### Integration Tests with Test Server

Build a reusable test application:

```rust
// tests/common/test_app.rs

use axum::Router;
use axum::body::Body;
use http::Request;
use sqlx::PgPool;
use tower::ServiceExt; // for oneshot
use hyper::StatusCode;

pub struct TestApp {
    pub pool: PgPool,
    pub router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        // Use a test-specific database (created per test or shared)
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/myapp_test".into());

        let pool = PgPool::connect(&database_url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        let config = Config::test_defaults();
        let state = AppState {
            db: pool.clone(),
            config: Arc::new(config),
            redis: None,
            http_client: reqwest::Client::new(),
        };

        let router = build_router(state);

        Self { pool, router }
    }

    pub async fn request(&self, request: Request<Body>) -> hyper::Response<Body> {
        self.router
            .clone()
            .oneshot(request)
            .await
            .unwrap()
    }

    pub async fn get(&self, uri: &str) -> hyper::Response<Body> {
        self.request(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .unwrap()
        ).await
    }

    pub async fn post_json(&self, uri: &str, body: serde_json::Value) -> hyper::Response<Body> {
        self.request(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap()
        ).await
    }

    pub async fn authenticated_get(&self, uri: &str, token: &str) -> hyper::Response<Body> {
        self.request(
            Request::builder()
                .uri(uri)
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap()
        ).await
    }
}
```

### Integration Test Examples

```rust
// tests/api/health_test.rs

use crate::common::TestApp;
use hyper::StatusCode;

#[tokio::test]
async fn health_returns_200() {
    let app = TestApp::new().await;
    let response = app.get("/health").await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn readiness_returns_200_when_db_connected() {
    let app = TestApp::new().await;
    let response = app.get("/ready").await;
    assert_eq!(response.status(), StatusCode::OK);
}
```

```rust
// tests/api/users_test.rs

use crate::common::TestApp;
use hyper::StatusCode;
use serde_json::json;

#[tokio::test]
async fn create_user_returns_201() {
    let app = TestApp::new().await;

    let response = app.post_json("/api/v1/users", json!({
        "email": "test@example.com",
        "name": "Test User",
        "password": "SecurePass1"
    })).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: serde_json::Value = read_body_json(response).await;
    assert_eq!(body["email"], "test@example.com");
    assert!(body.get("password_hash").is_none()); // Never expose hash
}

#[tokio::test]
async fn create_user_with_duplicate_email_returns_409() {
    let app = TestApp::new().await;

    let payload = json!({
        "email": "dup@example.com",
        "name": "First User",
        "password": "SecurePass1"
    });

    app.post_json("/api/v1/users", payload.clone()).await;
    let response = app.post_json("/api/v1/users", payload).await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_user_with_invalid_email_returns_422() {
    let app = TestApp::new().await;

    let response = app.post_json("/api/v1/users", json!({
        "email": "not-an-email",
        "name": "Test",
        "password": "SecurePass1"
    })).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn get_user_requires_auth() {
    let app = TestApp::new().await;
    let response = app.get("/api/v1/users/me").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_user_with_valid_token_returns_200() {
    let app = TestApp::new().await;
    let (user, token) = create_test_user_and_token(&app).await;

    let response = app.authenticated_get("/api/v1/users/me", &token).await;
    assert_eq!(response.status(), StatusCode::OK);
}
```

### Database Test Isolation

Use transactions that roll back after each test:

```rust
use sqlx::PgPool;

/// Run a test inside a transaction that rolls back
pub async fn with_test_tx<F, Fut>(pool: &PgPool, f: F)
where
    F: FnOnce(sqlx::Transaction<'_, sqlx::Postgres>) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let tx = pool.begin().await.unwrap();
    f(tx).await;
    // Transaction is dropped without commit, rolling back all changes
}
```

Or use the `#[sqlx::test]` macro for automatic database management:

```rust
#[sqlx::test(migrations = "./migrations")]
async fn test_create_user(pool: PgPool) {
    let user = db::users::create(&pool, &CreateUserInput {
        email: "test@example.com".into(),
        name: "Test".into(),
        password_hash: "hashed".into(),
        role: Role::User,
    }).await.unwrap();

    assert_eq!(user.email, "test@example.com");
}
```

### Mocking External Services

Use traits for external service clients, swap implementations in tests:

```rust
// Production trait
#[async_trait::async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), anyhow::Error>;
}

// Production implementation
pub struct SmtpEmailSender { /* ... */ }

#[async_trait::async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), anyhow::Error> {
        // Real SMTP logic
        Ok(())
    }
}

// Test mock
pub struct MockEmailSender {
    pub sent: std::sync::Mutex<Vec<(String, String, String)>>,
}

#[async_trait::async_trait]
impl EmailSender for MockEmailSender {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), anyhow::Error> {
        self.sent.lock().unwrap().push((to.into(), subject.into(), body.into()));
        Ok(())
    }
}
```

In `AppState`, use `Arc<dyn EmailSender>`:

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub email: Arc<dyn EmailSender>,
}
```

### Test Commands

```bash
# Run all tests
cargo test

# Run with output (see println! and tracing output)
cargo test -- --nocapture

# Run specific test
cargo test test_create_user

# Run tests in a specific file
cargo test --test users_test

# Run only unit tests (skip integration)
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run with coverage (requires cargo-llvm-cov)
cargo llvm-cov --html

# Clippy (zero warnings enforced)
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check
```

---

## 12. Configuration

### Environment-Based Configuration

Load configuration from environment variables with sensible defaults:

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Server
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    // Database
    pub database_url: String,

    // Auth
    pub jwt_secret: String,
    #[serde(default = "default_token_expiry")]
    pub token_expiry_hours: u64,

    // External services
    pub redis_url: Option<String>,

    // Environment
    #[serde(default = "default_environment")]
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Test,
    Staging,
    Production,
}

fn default_host() -> String { "0.0.0.0".into() }
fn default_port() -> u16 { 3000 }
fn default_token_expiry() -> u64 { 24 }
fn default_environment() -> Environment { Environment::Development }

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }

    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    #[cfg(test)]
    pub fn test_defaults() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 0, // OS-assigned
            database_url: std::env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/myapp_test".into()),
            jwt_secret: "test-secret-do-not-use-in-production".into(),
            token_expiry_hours: 1,
            redis_url: None,
            environment: Environment::Test,
        }
    }
}
```

### `.env` File (Development Only)

Use `dotenvy` to load `.env` in development:

```rust
fn main() {
    // Load .env file if it exists (development only)
    dotenvy::dotenv().ok();

    init_tracing();

    let config = Config::from_env()
        .expect("Failed to load configuration");

    // ...
}
```

**Convention:** Never commit `.env` files. Add `.env` to `.gitignore`. Provide a `.env.example` with placeholder values.

### Secret Management

- Development: `.env` file (never committed)
- CI: GitHub Actions secrets or environment variables
- Production: Container orchestrator secrets (Docker secrets, Kubernetes secrets, Fly.io secrets)
- Never hardcode secrets in source code
- Never log secrets at any level

---

## 13. Graceful Shutdown

### Tokio Signal Handling

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env()?;
    let state = AppState::new(config.clone()).await?;
    let router = build_router(state.clone());

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!(address = %addr, "Server starting");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shut down gracefully");

    // Clean up resources
    state.db.close().await;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received Ctrl+C"),
        _ = terminate => tracing::info!("Received SIGTERM"),
    }
}
```

---

## 14. Docker Deployment

### Multi-Stage Dockerfile

```dockerfile
# Stage 1: Build
FROM rust:1.82-bookworm AS builder

WORKDIR /app

# Cache dependencies by copying manifests first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy source and build for real
COPY . .
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --create-home appuser
USER appuser

WORKDIR /app

COPY --from=builder /app/target/release/my-service .
COPY --from=builder /app/migrations ./migrations

ENV RUST_LOG=info
EXPOSE 3000

ENTRYPOINT ["./my-service"]
```

### Docker Compose (Development)

```yaml
# docker-compose.yml
services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      DATABASE_URL: postgres://postgres:postgres@db:5432/myapp
      REDIS_URL: redis://redis:6379
      ENVIRONMENT: development
      JWT_SECRET: dev-secret-change-in-production
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  postgres_data:
```

### CI/CD Pipeline (GitHub Actions)

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  SQLX_OFFLINE: true   # Use cached query metadata in CI

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Build
        run: cargo build --release

  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_DB: myapp_test
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Run migrations
        run: cargo sqlx migrate run
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/myapp_test

      - name: Run tests
        run: cargo test --all
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/myapp_test
          JWT_SECRET: test-secret
          ENVIRONMENT: test

  deploy:
    needs: [check, test]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
```

### SQLx Offline Mode

For CI without a live database, use SQLx's offline mode:

```bash
# Generate query metadata during development (requires running DB)
cargo sqlx prepare

# This creates .sqlx/ directory with cached query data
# Commit .sqlx/ to version control
```

In CI, set `SQLX_OFFLINE=true` to use cached metadata instead of connecting to a database during compilation.

### Fly.io Deployment

```toml
# fly.toml
app = "my-service"
primary_region = "iad"

[build]

[deploy]
  release_command = "./my-service migrate"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = "stop"
  auto_start_machines = true
  min_machines_running = 1

[[vm]]
  size = "shared-cpu-1x"
  memory = "512mb"
```

---

## 15. Security Headers & Hardening

### Response Security Headers

Applied via middleware (see section 7):

| Header | Value | Purpose |
|---|---|---|
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Force HTTPS for 1 year |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referer leakage |
| `Content-Security-Policy` | `default-src 'self'; frame-ancestors 'none'` | Restrict content sources |
| `X-Request-Id` | `<uuid>` | Request correlation |
| `Cache-Control` | `no-store` (for API responses) | Prevent sensitive data caching |

### Request Validation

- **Body size limit:** 2 MB default via `RequestBodyLimitLayer` (section 7).
- **Timeout:** 30-second request timeout via `TimeoutLayer`.
- **Input validation:** All payloads validated via `ValidatedJson` extractor (section 5).
- **SQL injection:** Prevented by parameterized queries (SQLx `$1`, `$2` placeholders). Never interpolate user input into SQL strings.
- **Path traversal:** Axum's router does not serve files by default. If serving static files, use `ServeDir` with a restricted root.

### Rate Limiting

Use `tower_governor` or a custom Tower layer:

```rust
use tower_governor::{GovernorLayer, GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(30)
    .finish()
    .unwrap();

let governor_layer = GovernorLayer {
    config: Arc::new(governor_conf),
};

Router::new()
    .route("/api/v1/auth/login", post(handlers::auth::login))
    .layer(governor_layer)
```

### Dependency Auditing

```bash
# Check for known vulnerabilities
cargo audit

# Add to CI
cargo install cargo-audit
cargo audit --deny warnings
```

---

## 16. Anti-Patterns

| Anti-Pattern | Do This Instead |
|---|---|
| Blocking the Tokio runtime with synchronous I/O | Use `tokio::task::spawn_blocking` for CPU-heavy or sync I/O work |
| Using `unwrap()` or `expect()` in handlers | Return `Result<_, ApiError>` and use `?` operator |
| Putting business logic in handlers | Handlers extract + validate + delegate to services; services contain logic |
| Using `String` for all IDs | Use `Uuid` — type-safe, parseable via `Path<Uuid>` |
| Leaking internal error details to clients | Log internally with `tracing::error!`, return generic message |
| Using `Mutex` for request-scoped data | Use extractors or request extensions instead |
| Hardcoding configuration values | Load from environment via `Config` struct |
| Sharing mutable state without `Arc<Mutex<T>>` or `Arc<RwLock<T>>` | Use proper synchronization or actor model (tokio channels) |
| Using `.clone()` on `PgPool` in hot loops | `PgPool` is already `Arc`-wrapped; cloning is cheap, but hold a reference when possible |
| Skipping migrations in tests | Always run migrations in test setup — tests must use the real schema |
| Using `#[tokio::test]` without `#[sqlx::test]` for DB tests | Use `#[sqlx::test]` for automatic test database management |
| Mixing SQLx and SeaORM in one project | Pick one and stick with it |
| Using `CorsLayer::permissive()` in production | Configure explicit allowed origins, methods, and headers |
| Catching panics to "handle errors" | Use `Result` types; panics indicate bugs, not expected errors |
| Logging passwords, tokens, or PII | Never log sensitive data at any tracing level |
| Using `axum::serve` without graceful shutdown | Always use `.with_graceful_shutdown(shutdown_signal())` |
| Writing raw SQL strings with format! | Use parameterized queries (`$1`, `$2`) to prevent SQL injection |
| Implementing `Clone` on types with `Mutex` | Use `Arc<Mutex<T>>` as a field instead |
| Missing request body size limits | Always apply `RequestBodyLimitLayer` to prevent DoS |
| Not setting request timeouts | Always apply `TimeoutLayer` to prevent slow-loris attacks |
| Using `StatusCode::OK` for all responses | Use appropriate status codes: 201 Created, 204 No Content, 404 Not Found, etc. |
| Putting `#[tokio::main]` on library code | Only use `#[tokio::main]` in `main.rs`; libraries should be runtime-agnostic |
| Using `sleep` for retry logic | Use exponential backoff with jitter (e.g., `backoff` crate) |
| Committing `.env` files | Add `.env` to `.gitignore`; provide `.env.example` with placeholders |
| Not running `cargo clippy` in CI | Enforce `cargo clippy -- -D warnings` in CI pipeline |
| Ignoring compiler warnings | Treat warnings as errors; fix them immediately |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report an Axum patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:axum&title=[Axum]%20)**

Use the `patterns:axum` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
