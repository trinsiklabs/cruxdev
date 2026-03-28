# Post-Deployment Verification Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Kubernetes docs, Fly.io docs, Vercel docs, AWS docs, Docker docs, Cloudflare docs, NGINX docs, Microsoft Azure Architecture Center, microservices.io, Better Stack, Sentry, Harness, Liquibase, CircleCI, Redgate Flyway, pgroll, web.dev, Axum/Actix docs, Phoenix/Plug docs, Next.js docs
**Last updated:** 2026-03-27

---

## Table of Contents

1. [Core Principle](#1-core-principle)
2. [Health Endpoint Patterns](#2-health-endpoint-patterns)
3. [Platform-Specific Deployment Checks](#3-platform-specific-deployment-checks)
4. [Smoke Test Patterns](#4-smoke-test-patterns)
5. [Rollback Strategies](#5-rollback-strategies)
6. [Monitoring Integration](#6-monitoring-integration)
7. [Notification Patterns](#7-notification-patterns)
8. [Anti-Patterns](#8-anti-patterns)
9. [Audit Dimensions](#9-audit-dimensions)

---

## 1. Core Principle

**A deploy is not done when the code is pushed. A deploy is done when the system is verified healthy.**

Every deployment must pass through a verification pipeline before it is considered complete. The pipeline has three layers:

1. **Infrastructure health** — the process is running, ports are bound, dependencies are reachable
2. **Application health** — critical paths respond correctly, data flows end-to-end
3. **Observability health** — monitoring is active, alerts are configured, baselines are established

Skipping any layer creates a deployment that is "Schrodinger's deploy" — you do not know if it works until a user reports it broken.

### The Verification Hierarchy

```
Layer 3: Observability    [monitors active, baselines set, alerts configured]
Layer 2: Application      [smoke tests pass, critical paths verified]
Layer 1: Infrastructure   [health endpoint OK, dependencies reachable]
Layer 0: Process          [deploy command succeeded, no error exit code]
```

Most teams stop at Layer 0. Production-grade teams verify all four layers before marking a deploy as successful.

---

## 2. Health Endpoint Patterns

### 2.1 Endpoint Naming Conventions

| Endpoint | Purpose | Audience |
|----------|---------|----------|
| `/health` | General health status | Load balancers, uptime monitors |
| `/healthz` | Kubernetes-style liveness probe | Orchestrators (deprecated in K8s v1.16+) |
| `/livez` | Liveness — is the process alive? | Kubernetes liveness probe |
| `/readyz` | Readiness — can the process serve traffic? | Kubernetes readiness probe, load balancers |
| `/startupz` | Startup — has initialization completed? | Kubernetes startup probe |

**Recommendation:** Use `/health` for general-purpose health checks. If running in Kubernetes, additionally expose `/livez` and `/readyz` with different check depths.

### 2.2 Response Schema

A proper health endpoint returns structured JSON, not just a bare 200.

```json
{
  "status": "healthy",
  "version": "1.4.2",
  "git_sha": "a3f8b2c",
  "uptime_seconds": 84210,
  "checks": {
    "database": {
      "status": "healthy",
      "latency_ms": 2.3,
      "details": "PostgreSQL 16.2, 14 active connections"
    },
    "migrations": {
      "status": "healthy",
      "version": "20260327120000",
      "pending": 0
    },
    "cache": {
      "status": "healthy",
      "latency_ms": 0.4,
      "details": "Redis 7.2, memory 48MB/256MB"
    },
    "storage": {
      "status": "healthy",
      "details": "S3 bucket reachable"
    }
  },
  "timestamp": "2026-03-27T14:30:00Z"
}
```

**Status values:** `healthy`, `degraded`, `unhealthy`. Use `degraded` when optional dependencies are down but the service can still serve requests.

**HTTP status codes:**
- `200` — all checks pass (or service is degraded but functional)
- `503` — service is unhealthy, should not receive traffic

### 2.3 Shallow vs. Deep Checks

| Check Type | What It Tests | Response Time | Use Case |
|------------|---------------|---------------|----------|
| **Shallow** | Process is alive, can respond to HTTP | < 5ms | Load balancer liveness, high-frequency polling |
| **Deep** | All dependencies reachable, queries succeed | 50-500ms | Readiness, post-deploy verification |

**Critical rule:** Separate shallow and deep checks into different endpoints. A load balancer hitting your deep check every 5 seconds will overload your database connection pool.

```
GET /health         → shallow check (process alive)
GET /health?deep=1  → deep check (all dependencies)
GET /readyz         → deep check (Kubernetes readiness)
GET /livez          → shallow check (Kubernetes liveness)
```

### 2.4 Security Considerations

- **Disable caching:** Add `Cache-Control: no-cache, no-store` to health responses. A cached 200 is a lie.
- **Limit detail exposure:** Public health endpoints should return status only, not connection counts, versions, or internal details. Expose full details only behind authentication or on internal networks.
- **Rate limit:** Health endpoints should be rate-limited to prevent abuse. 60 requests/minute per IP is reasonable.

### 2.5 Framework-Specific Examples

#### Phoenix/Elixir

```elixir
# lib/my_app_web/plugs/health_check.ex
defmodule MyAppWeb.Plugs.HealthCheck do
  import Plug.Conn

  def init(opts), do: opts

  def call(%Plug.Conn{request_path: "/health"} = conn, _opts) do
    checks = %{
      database: check_database(),
      migrations: check_migrations(),
      cache: check_cache()
    }

    status = if Enum.all?(Map.values(checks), &(&1.status == "healthy")),
      do: "healthy", else: "unhealthy"

    http_code = if status == "healthy", do: 200, else: 503

    conn
    |> put_resp_header("content-type", "application/json")
    |> put_resp_header("cache-control", "no-cache, no-store")
    |> send_resp(http_code, Jason.encode!(%{
      status: status,
      version: Application.spec(:my_app, :vsn) |> to_string(),
      uptime_seconds: :erlang.statistics(:wall_clock) |> elem(0) |> div(1000),
      checks: checks,
      timestamp: DateTime.utc_now() |> DateTime.to_iso8601()
    }))
    |> halt()
  end

  def call(conn, _opts), do: conn

  defp check_database do
    case Ecto.Adapters.SQL.query(MyApp.Repo, "SELECT 1", []) do
      {:ok, _} -> %{status: "healthy"}
      {:error, reason} -> %{status: "unhealthy", error: inspect(reason)}
    end
  end

  defp check_migrations do
    pending = Ecto.Migrator.migrations(MyApp.Repo)
              |> Enum.count(fn {status, _, _} -> status == :down end)
    if pending == 0,
      do: %{status: "healthy", pending: 0},
      else: %{status: "unhealthy", pending: pending}
  end

  defp check_cache do
    case Redix.command(:redix, ["PING"]) do
      {:ok, "PONG"} -> %{status: "healthy"}
      _ -> %{status: "unhealthy"}
    end
  end
end

# In endpoint.ex — BEFORE other plugs to short-circuit
plug MyAppWeb.Plugs.HealthCheck
```

#### Next.js (App Router)

```typescript
// app/api/health/route.ts
import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import { redis } from '@/lib/redis';

export const dynamic = 'force-dynamic';
export const revalidate = 0;

export async function GET(request: Request) {
  const url = new URL(request.url);
  const deep = url.searchParams.get('deep') === '1';

  const result: Record<string, unknown> = {
    status: 'healthy',
    version: process.env.NEXT_PUBLIC_VERSION || 'unknown',
    uptime_seconds: Math.floor(process.uptime()),
    timestamp: new Date().toISOString(),
  };

  if (deep) {
    const checks: Record<string, unknown> = {};

    // Database check
    try {
      const start = performance.now();
      await prisma.$queryRaw`SELECT 1`;
      checks.database = {
        status: 'healthy',
        latency_ms: Math.round(performance.now() - start),
      };
    } catch (err) {
      checks.database = { status: 'unhealthy', error: String(err) };
    }

    // Redis check
    try {
      const start = performance.now();
      await redis.ping();
      checks.cache = {
        status: 'healthy',
        latency_ms: Math.round(performance.now() - start),
      };
    } catch (err) {
      checks.cache = { status: 'unhealthy', error: String(err) };
    }

    result.checks = checks;
    const unhealthy = Object.values(checks)
      .some((c: any) => c.status === 'unhealthy');
    if (unhealthy) result.status = 'unhealthy';
  }

  const httpCode = result.status === 'healthy' ? 200 : 503;
  return NextResponse.json(result, {
    status: httpCode,
    headers: { 'Cache-Control': 'no-cache, no-store' },
  });
}
```

#### Rails

```ruby
# app/controllers/health_controller.rb
class HealthController < ActionController::API
  def show
    checks = {
      database: check_database,
      migrations: check_migrations,
      cache: check_cache
    }

    status = checks.values.all? { |c| c[:status] == "healthy" } ?
      "healthy" : "unhealthy"

    render json: {
      status: status,
      version: ENV.fetch("APP_VERSION", "unknown"),
      git_sha: ENV.fetch("GIT_SHA", "unknown"),
      uptime_seconds: (Time.current - Rails.application.config.boot_time).to_i,
      checks: checks,
      timestamp: Time.current.iso8601
    }, status: status == "healthy" ? :ok : :service_unavailable
  end

  private

  def check_database
    ActiveRecord::Base.connection.execute("SELECT 1")
    { status: "healthy" }
  rescue StandardError => e
    { status: "unhealthy", error: e.message }
  end

  def check_migrations
    pending = ActiveRecord::Migration.check_all_pending!
    { status: "healthy", pending: 0 }
  rescue ActiveRecord::PendingMigrationError
    { status: "unhealthy", pending: "migrations pending" }
  end

  def check_cache
    Rails.cache.write("health_check", "ok", expires_in: 10.seconds)
    Rails.cache.read("health_check") == "ok" ?
      { status: "healthy" } : { status: "unhealthy" }
  rescue StandardError => e
    { status: "unhealthy", error: e.message }
  end
end

# config/routes.rb
get "/health", to: "health#show"
```

#### Django

```python
# health/views.py
import time
from django.http import JsonResponse
from django.db import connection
from django.core.cache import cache
from django.conf import settings

_start_time = time.time()

def health_check(request):
    deep = request.GET.get("deep") == "1"

    result = {
        "status": "healthy",
        "version": getattr(settings, "APP_VERSION", "unknown"),
        "uptime_seconds": int(time.time() - _start_time),
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }

    if deep:
        checks = {}

        # Database
        try:
            with connection.cursor() as cursor:
                start = time.monotonic()
                cursor.execute("SELECT 1")
                checks["database"] = {
                    "status": "healthy",
                    "latency_ms": round((time.monotonic() - start) * 1000, 1),
                }
        except Exception as e:
            checks["database"] = {"status": "unhealthy", "error": str(e)}

        # Cache
        try:
            cache.set("health_check", "ok", timeout=10)
            if cache.get("health_check") == "ok":
                checks["cache"] = {"status": "healthy"}
            else:
                checks["cache"] = {"status": "unhealthy"}
        except Exception as e:
            checks["cache"] = {"status": "unhealthy", "error": str(e)}

        result["checks"] = checks
        if any(c["status"] == "unhealthy" for c in checks.values()):
            result["status"] = "unhealthy"

    http_code = 200 if result["status"] == "healthy" else 503
    response = JsonResponse(result, status=http_code)
    response["Cache-Control"] = "no-cache, no-store"
    return response

# urls.py
from django.urls import path
from health.views import health_check

urlpatterns = [
    path("health", health_check),
]
```

#### Go (net/http)

```go
// health/handler.go
package health

import (
    "context"
    "database/sql"
    "encoding/json"
    "net/http"
    "time"

    "github.com/redis/go-redis/v9"
)

type Checker struct {
    DB        *sql.DB
    Redis     *redis.Client
    Version   string
    GitSHA    string
    StartTime time.Time
}

type CheckResult struct {
    Status    string  `json:"status"`
    LatencyMs float64 `json:"latency_ms,omitempty"`
    Error     string  `json:"error,omitempty"`
}

type HealthResponse struct {
    Status        string                  `json:"status"`
    Version       string                  `json:"version"`
    GitSHA        string                  `json:"git_sha"`
    UptimeSeconds int64                   `json:"uptime_seconds"`
    Checks        map[string]CheckResult  `json:"checks,omitempty"`
    Timestamp     string                  `json:"timestamp"`
}

func (c *Checker) Handler() http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        deep := r.URL.Query().Get("deep") == "1"
        ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
        defer cancel()

        resp := HealthResponse{
            Status:        "healthy",
            Version:       c.Version,
            GitSHA:        c.GitSHA,
            UptimeSeconds: int64(time.Since(c.StartTime).Seconds()),
            Timestamp:     time.Now().UTC().Format(time.RFC3339),
        }

        if deep {
            checks := make(map[string]CheckResult)

            // Database
            start := time.Now()
            if err := c.DB.PingContext(ctx); err != nil {
                checks["database"] = CheckResult{
                    Status: "unhealthy", Error: err.Error(),
                }
            } else {
                checks["database"] = CheckResult{
                    Status:    "healthy",
                    LatencyMs: float64(time.Since(start).Microseconds()) / 1000,
                }
            }

            // Redis
            start = time.Now()
            if err := c.Redis.Ping(ctx).Err(); err != nil {
                checks["cache"] = CheckResult{
                    Status: "unhealthy", Error: err.Error(),
                }
            } else {
                checks["cache"] = CheckResult{
                    Status:    "healthy",
                    LatencyMs: float64(time.Since(start).Microseconds()) / 1000,
                }
            }

            resp.Checks = checks
            for _, check := range checks {
                if check.Status == "unhealthy" {
                    resp.Status = "unhealthy"
                    break
                }
            }
        }

        w.Header().Set("Content-Type", "application/json")
        w.Header().Set("Cache-Control", "no-cache, no-store")

        code := http.StatusOK
        if resp.Status == "unhealthy" {
            code = http.StatusServiceUnavailable
        }
        w.WriteHeader(code)
        json.NewEncoder(w).Encode(resp)
    }
}
```

#### Rust (Axum)

```rust
// src/health.rs
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sqlx::PgPool;
use std::time::Instant;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    version: &'static str,
    uptime_seconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    checks: Option<HealthChecks>,
    timestamp: String,
}

#[derive(Serialize)]
pub struct HealthChecks {
    database: CheckResult,
}

#[derive(Serialize)]
pub struct CheckResult {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    latency_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub start_time: Instant,
    pub version: &'static str,
}

pub async fn health_shallow(
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    let resp = HealthResponse {
        status: "healthy",
        version: state.version,
        uptime_seconds: state.start_time.elapsed().as_secs(),
        checks: None,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    (StatusCode::OK, Json(resp))
}

pub async fn health_deep(
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    let start = Instant::now();
    let db_check = match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => CheckResult {
            status: "healthy",
            latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
            error: None,
        },
        Err(e) => CheckResult {
            status: "unhealthy",
            latency_ms: None,
            error: Some(e.to_string()),
        },
    };

    let overall = if db_check.status == "healthy" {
        "healthy"
    } else {
        "unhealthy"
    };

    let code = if overall == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let resp = HealthResponse {
        status: overall,
        version: state.version,
        uptime_seconds: state.start_time.elapsed().as_secs(),
        checks: Some(HealthChecks { database: db_check }),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    (code, Json(resp))
}

// In router setup:
// .route("/health", get(health_shallow))
// .route("/readyz", get(health_deep))
```

---

## 3. Platform-Specific Deployment Checks

### 3.1 Fly.io

Fly.io supports three types of health checks and a release command for pre-deploy verification.

#### fly.toml Configuration

```toml
app = "my-app"
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

# Runs BEFORE deploy — use for migrations
# Fails the deploy on non-zero exit code
# Runs in a temporary Machine (no volumes)
# Times out after 5 minutes by default
[deploy]
  release_command = "bin/migrate"
  strategy = "rolling"

# HTTP health checks — used by load balancer and deploy
[[http_service.checks]]
  grace_period = "10s"
  interval = "15s"
  method = "GET"
  timeout = "5s"
  path = "/health"

[http_service]
  internal_port = 8080
  force_https = true

  [[http_service.checks]]
    grace_period = "10s"
    interval = "15s"
    method = "GET"
    timeout = "5s"
    path = "/health"

# Machine-level checks (TCP or HTTP)
[[vm]]
  size = "shared-cpu-1x"
  memory = "512mb"

[[checks]]
  port = 8080
  type = "http"
  interval = "15s"
  timeout = "5s"
  grace_period = "30s"
  method = "GET"
  path = "/health"
```

**Key behaviors:**
- `release_command` failure halts the deploy entirely — migrations that fail prevent bad code from going live
- Health check failure during deploy triggers automatic rollback to the previous release
- `grace_period` gives the app time to start before health checks begin
- Use `fly checks list` to view current health check status

#### Seamless Zero-Downtime Deploy

```toml
[deploy]
  strategy = "rolling"
  max_unavailable = 0

# Ensures new machines are healthy before old ones are stopped
```

### 3.2 Vercel

Vercel handles health differently than traditional platforms because it runs serverless functions.

#### Deployment Checks

Vercel supports deployment checks that set conditions which must be met before a deployment is promoted:

```json
// vercel.json
{
  "checks": {
    "myCheck": {
      "path": "/api/health",
      "method": "GET",
      "expect": {
        "status": [200]
      }
    }
  }
}
```

#### Health Endpoint for Serverless

```typescript
// app/api/health/route.ts
// This endpoint tests that serverless functions are operational
// and can reach backend services from the Vercel edge/function runtime
export async function GET() {
  const checks: Record<string, string> = {};

  // Verify database is reachable from this region
  try {
    await db.execute(sql`SELECT 1`);
    checks.database = 'healthy';
  } catch {
    checks.database = 'unhealthy';
  }

  // Verify environment variables are set
  checks.config = process.env.DATABASE_URL ? 'healthy' : 'unhealthy';

  const healthy = Object.values(checks).every(s => s === 'healthy');
  return Response.json(
    { status: healthy ? 'healthy' : 'unhealthy', checks },
    { status: healthy ? 200 : 503 }
  );
}
```

**Key considerations:**
- Serverless cold starts can cause initial health check failures — set appropriate timeouts
- Fluid compute (enabled by default since April 2025) reduces cold start issues
- Monitor your `/api/health` endpoint externally; Vercel's own status page may not reflect your function-level issues
- Preview deployments get unique URLs — verify preview health before promoting to production

### 3.3 AWS (ALB + ECS)

AWS provides multiple layers of health checking for containerized services.

#### ALB Target Group Health Check

```json
{
  "HealthCheckProtocol": "HTTP",
  "HealthCheckPort": "traffic-port",
  "HealthCheckPath": "/health",
  "HealthCheckIntervalSeconds": 15,
  "HealthCheckTimeoutSeconds": 5,
  "HealthyThresholdCount": 2,
  "UnhealthyThresholdCount": 3,
  "Matcher": {
    "HttpCode": "200-299"
  }
}
```

**Optimized settings for fast detection:**
- Interval: 5 seconds, healthy threshold: 2 reduces verification to 10 seconds (vs. default 2.5 minutes)
- Set unhealthy threshold to 3 to avoid false positives from transient network issues

#### ECS Task Definition Health Check

```json
{
  "healthCheck": {
    "command": ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"],
    "interval": 30,
    "timeout": 5,
    "retries": 3,
    "startPeriod": 60
  }
}
```

#### Dual Health Check Strategy

ECS considers a task healthy only when both container health check AND ALB health check pass. Use different check depths:

| Check | Purpose | Depth |
|-------|---------|-------|
| Container health check | Liveness — is the process alive? | Shallow |
| ALB health check | Readiness — can it serve traffic? | Deep |

**Health check grace period:** Set to at least 2x your application's startup time. If the app takes 30 seconds to start, set grace period to 60-90 seconds.

#### CloudFormation Example

```yaml
# ECS Service with health check grace period
MyService:
  Type: AWS::ECS::Service
  Properties:
    Cluster: !Ref MyCluster
    TaskDefinition: !Ref MyTaskDef
    DesiredCount: 2
    HealthCheckGracePeriodSeconds: 90
    DeploymentConfiguration:
      MinimumHealthyPercent: 100
      MaximumPercent: 200
      DeploymentCircuitBreaker:
        Enable: true
        Rollback: true
    LoadBalancers:
      - ContainerName: app
        ContainerPort: 8080
        TargetGroupArn: !Ref MyTargetGroup
```

The `DeploymentCircuitBreaker` with `Rollback: true` provides automatic rollback when new tasks fail health checks.

### 3.4 Docker

#### HEALTHCHECK Instruction

```dockerfile
FROM node:20-alpine

WORKDIR /app
COPY . .
RUN npm ci --production

HEALTHCHECK \
  --interval=30s \
  --timeout=5s \
  --start-period=15s \
  --retries=3 \
  CMD wget --spider -q http://localhost:3000/health || exit 1

CMD ["node", "server.js"]
```

**Parameters explained:**

| Parameter | Default | Purpose |
|-----------|---------|---------|
| `--interval` | 30s | Time between checks |
| `--timeout` | 30s | Max time to wait for check to complete |
| `--start-period` | 0s | Grace period for container startup |
| `--retries` | 3 | Consecutive failures before marking unhealthy |

**Tool availability matters.** `curl` is not available in all base images. Alternatives:

```dockerfile
# Alpine — use wget (included by default)
CMD wget --spider -q http://localhost:3000/health || exit 1

# Distroless — use a compiled binary
COPY --from=builder /app/healthcheck /usr/local/bin/healthcheck
CMD ["/usr/local/bin/healthcheck"]

# Python — use the stdlib
CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')"

# No HTTP tools — use TCP check with netcat
CMD nc -z localhost 8080 || exit 1
```

**For Java/JVM applications with slow startup:**

```dockerfile
HEALTHCHECK \
  --interval=30s \
  --timeout=5s \
  --start-period=120s \
  --retries=5 \
  CMD curl -f http://localhost:8080/actuator/health || exit 1
```

#### Docker Compose

```yaml
services:
  web:
    build: .
    healthcheck:
      test: ["CMD", "wget", "--spider", "-q", "http://localhost:3000/health"]
      interval: 30s
      timeout: 5s
      start_period: 15s
      retries: 3
    depends_on:
      db:
        condition: service_healthy

  db:
    image: postgres:16
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
```

### 3.5 Bare Metal (systemd + nginx)

#### systemd Service with Health Verification

```ini
# /etc/systemd/system/myapp.service
[Unit]
Description=My Application
After=network.target postgresql.service redis.service
Requires=postgresql.service

[Service]
Type=notify
User=myapp
Group=myapp
WorkingDirectory=/opt/myapp
ExecStart=/opt/myapp/bin/server
ExecStartPost=/opt/myapp/bin/health-wait.sh
Restart=on-failure
RestartSec=5
WatchdogSec=60

[Install]
WantedBy=multi-user.target
```

**health-wait.sh — Post-start verification:**

```bash
#!/usr/bin/env bash
set -euo pipefail

MAX_RETRIES=30
RETRY_INTERVAL=2
URL="http://localhost:8080/health"

for i in $(seq 1 $MAX_RETRIES); do
  if curl -sf "$URL" > /dev/null 2>&1; then
    echo "Health check passed on attempt $i"
    exit 0
  fi
  echo "Health check attempt $i/$MAX_RETRIES failed, retrying in ${RETRY_INTERVAL}s..."
  sleep "$RETRY_INTERVAL"
done

echo "FATAL: Health check failed after $MAX_RETRIES attempts"
exit 1
```

**WatchdogSec=60:** systemd sends SIGABRT if the process does not call `sd_notify(0, "WATCHDOG=1")` within 60 seconds. This catches frozen processes that are not responding but have not crashed.

#### nginx Upstream Health (Open Source)

NGINX open-source performs passive health checks by default. For active checks without NGINX Plus:

```nginx
upstream backend {
    server 127.0.0.1:8080 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8081 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    server_name myapp.com;

    location / {
        proxy_pass http://backend;
        proxy_next_upstream error timeout http_502 http_503 http_504;
        proxy_next_upstream_tries 2;
    }

    # Health endpoint bypass — goes directly to upstream
    location /health {
        proxy_pass http://backend;
        proxy_connect_timeout 2s;
        proxy_read_timeout 5s;
    }
}
```

**Parameters:**
- `max_fails=3` — mark server as unavailable after 3 failed requests
- `fail_timeout=30s` — unavailable servers are retried after 30 seconds
- `proxy_next_upstream` — which errors trigger failover to next server

For active health checks on open-source NGINX, use the `nginx_upstream_check_module` (requires compiling NGINX from source) or an external health checker (cron/systemd timer) that updates an upstream config file.

### 3.6 Cloudflare Pages/Workers

#### Workers Health Check Pattern

```typescript
// src/worker.ts
export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    if (url.pathname === '/health') {
      return handleHealth(env);
    }

    // ... normal routing
  },

  // Cron trigger for self-monitoring
  async scheduled(event: ScheduledEvent, env: Env, ctx: ExecutionContext) {
    ctx.waitUntil(selfHealthCheck(env));
  },
};

async function handleHealth(env: Env): Promise<Response> {
  const checks: Record<string, unknown> = {};

  // D1 database check
  try {
    const result = await env.DB.prepare('SELECT 1 AS ok').first();
    checks.d1 = { status: result?.ok === 1 ? 'healthy' : 'unhealthy' };
  } catch (err) {
    checks.d1 = { status: 'unhealthy', error: String(err) };
  }

  // KV check
  try {
    await env.KV.put('_health', 'ok');
    const val = await env.KV.get('_health');
    checks.kv = { status: val === 'ok' ? 'healthy' : 'unhealthy' };
  } catch (err) {
    checks.kv = { status: 'unhealthy', error: String(err) };
  }

  const healthy = Object.values(checks)
    .every((c: any) => c.status === 'healthy');

  return new Response(
    JSON.stringify({
      status: healthy ? 'healthy' : 'unhealthy',
      checks,
      timestamp: new Date().toISOString(),
    }),
    {
      status: healthy ? 200 : 503,
      headers: {
        'Content-Type': 'application/json',
        'Cache-Control': 'no-cache, no-store',
      },
    }
  );
}
```

**Cloudflare Health Checks (origin monitoring):**

Cloudflare provides standalone Health Checks that monitor your origin server(s) from Cloudflare's edge network. Configure via the dashboard or API:

```bash
# Create a health check via API
curl -X POST "https://api.cloudflare.com/client/v4/zones/{zone_id}/healthchecks" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "production-origin",
    "address": "origin.myapp.com",
    "type": "HTTPS",
    "check_regions": ["WNAM", "ENAM", "WEU"],
    "notification": {
      "suspended": false,
      "email_addresses": ["ops@myapp.com"]
    },
    "http_config": {
      "method": "GET",
      "path": "/health",
      "port": 443,
      "expected_codes": ["200"],
      "follow_redirects": false,
      "allow_insecure": false
    },
    "interval": 60,
    "retries": 2,
    "timeout": 5
  }'
```

---

## 4. Smoke Test Patterns

### 4.1 Three-Layer Smoke Test Framework

Smoke tests are not comprehensive test suites. They are fast, targeted checks that verify the most critical paths after deployment. A complete smoke suite should run in under 5 minutes.

#### Layer 1: Infrastructure Smoke (< 1 minute)

These tests verify the deployment itself succeeded and basic connectivity works.

| Check | What It Verifies | Failure Means |
|-------|------------------|---------------|
| Health endpoint returns 200 | App is running, dependencies reachable | Deploy failed or app crashed on startup |
| Homepage returns 200 | Web server is routing correctly | Routing misconfiguration |
| SSL certificate valid | TLS termination working | Certificate expired or misconfigured |
| No mixed content warnings | All assets served over HTTPS | Asset URL misconfiguration |
| DNS resolves correctly | Domain pointing to correct target | DNS propagation issue or misconfiguration |

#### Layer 2: Application Smoke (< 2 minutes)

These tests verify critical application paths work end-to-end.

| Check | What It Verifies | Failure Means |
|-------|------------------|---------------|
| Critical API endpoints respond | Backend logic is functional | Code regression or dependency failure |
| Auth flow works (login, session) | Authentication system operational | Auth config, session store, or token issue |
| Database queries succeed | Read/write path functional | Migration failure or connection issue |
| Static assets load (CSS, JS, images) | Build artifacts deployed correctly | Build failure, CDN issue, or path mismatch |
| Key pages render without errors | Template/component rendering works | Frontend build or data issue |

#### Layer 3: Integration Smoke (< 2 minutes)

These tests verify third-party integrations and edge cases.

| Check | What It Verifies | Failure Means |
|-------|------------------|---------------|
| WebSocket connections (if applicable) | Real-time features work | Proxy misconfiguration or port issue |
| Email/notification delivery | Transactional email operational | SMTP config or API key issue |
| File upload/download | Storage integration works | S3/storage credentials or CORS issue |
| Search functionality | Search index accessible | Elasticsearch/Meilisearch connection issue |
| Payment flow (test mode) | Payment integration operational | Stripe/payment API key or webhook issue |

### 4.2 Smoke Test Implementation

#### Bash (Quick and Universal)

```bash
#!/usr/bin/env bash
# smoke-test.sh — Post-deployment smoke test suite
set -euo pipefail

BASE_URL="${1:?Usage: smoke-test.sh <base-url>}"
FAILURES=0

check() {
  local name="$1" url="$2" expected_status="${3:-200}"
  local actual_status
  actual_status=$(curl -s -o /dev/null -w "%{http_code}" \
    --max-time 10 --retry 2 --retry-delay 2 "$url") || true

  if [ "$actual_status" = "$expected_status" ]; then
    echo "  PASS: $name ($actual_status)"
  else
    echo "  FAIL: $name (expected $expected_status, got $actual_status)"
    FAILURES=$((FAILURES + 1))
  fi
}

check_ssl() {
  local domain="$1"
  local expiry
  expiry=$(echo | openssl s_client -servername "$domain" \
    -connect "$domain:443" 2>/dev/null \
    | openssl x509 -noout -enddate 2>/dev/null \
    | cut -d= -f2)

  if [ -z "$expiry" ]; then
    echo "  FAIL: SSL certificate check (could not connect)"
    FAILURES=$((FAILURES + 1))
    return
  fi

  local expiry_epoch
  expiry_epoch=$(date -d "$expiry" +%s 2>/dev/null || date -j -f "%b %d %T %Y %Z" "$expiry" +%s 2>/dev/null)
  local now_epoch
  now_epoch=$(date +%s)
  local days_left=$(( (expiry_epoch - now_epoch) / 86400 ))

  if [ "$days_left" -gt 7 ]; then
    echo "  PASS: SSL certificate valid ($days_left days remaining)"
  else
    echo "  WARN: SSL certificate expires in $days_left days"
    FAILURES=$((FAILURES + 1))
  fi
}

check_no_mixed_content() {
  local url="$1"
  local body
  body=$(curl -s --max-time 10 "$url")
  if echo "$body" | grep -qi 'http://' | grep -v 'http://localhost' | grep -v 'http://127' > /dev/null 2>&1; then
    echo "  WARN: Possible mixed content detected on $url"
  else
    echo "  PASS: No mixed content on $url"
  fi
}

echo "=== Layer 1: Infrastructure ==="
check "Health endpoint" "$BASE_URL/health"
check "Homepage" "$BASE_URL/"
check_ssl "$(echo "$BASE_URL" | sed 's|https://||' | sed 's|/.*||')"

echo ""
echo "=== Layer 2: Application ==="
check "API status" "$BASE_URL/api/status"
check "Login page" "$BASE_URL/login"
check "Static CSS" "$BASE_URL/assets/main.css"
check "Static JS" "$BASE_URL/assets/main.js"

echo ""
echo "=== Layer 3: Integration ==="
# Add project-specific integration checks here

echo ""
if [ "$FAILURES" -gt 0 ]; then
  echo "RESULT: $FAILURES check(s) failed"
  exit 1
else
  echo "RESULT: All checks passed"
  exit 0
fi
```

#### TypeScript (Playwright-Based)

```typescript
// smoke.test.ts — Run with: npx playwright test smoke.test.ts
import { test, expect } from '@playwright/test';

const BASE_URL = process.env.SMOKE_URL || 'https://myapp.com';

test.describe('Layer 1: Infrastructure', () => {
  test('health endpoint returns 200 with healthy status', async ({ request }) => {
    const response = await request.get(`${BASE_URL}/health`);
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.status).toBe('healthy');
  });

  test('homepage returns 200', async ({ page }) => {
    const response = await page.goto(BASE_URL);
    expect(response?.status()).toBe(200);
  });

  test('no console errors on homepage', async ({ page }) => {
    const errors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') errors.push(msg.text());
    });
    await page.goto(BASE_URL);
    expect(errors).toEqual([]);
  });

  test('SSL certificate is valid', async ({ request }) => {
    // Playwright will fail on invalid SSL by default
    const response = await request.get(BASE_URL);
    expect(response.ok()).toBeTruthy();
  });
});

test.describe('Layer 2: Application', () => {
  test('critical API endpoint responds', async ({ request }) => {
    const response = await request.get(`${BASE_URL}/api/status`);
    expect(response.status()).toBe(200);
  });

  test('login page loads and has form', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    await expect(page.locator('form')).toBeVisible();
    await expect(page.locator('input[type="email"], input[name="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('static assets load correctly', async ({ page }) => {
    const failedRequests: string[] = [];
    page.on('requestfailed', request => {
      failedRequests.push(request.url());
    });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    expect(failedRequests).toEqual([]);
  });

  test('key pages return 200', async ({ request }) => {
    const paths = ['/', '/about', '/pricing', '/docs'];
    for (const path of paths) {
      const response = await request.get(`${BASE_URL}${path}`);
      expect(response.status(), `${path} should return 200`).toBe(200);
    }
  });
});

test.describe('Layer 3: Integration', () => {
  test('WebSocket connection establishes', async ({ page }) => {
    // Only include if your app uses WebSockets
    const wsConnected = await page.evaluate((base) => {
      return new Promise<boolean>((resolve) => {
        const wsUrl = base.replace('https://', 'wss://') + '/ws';
        const ws = new WebSocket(wsUrl);
        ws.onopen = () => { ws.close(); resolve(true); };
        ws.onerror = () => resolve(false);
        setTimeout(() => resolve(false), 5000);
      });
    }, BASE_URL);
    expect(wsConnected).toBe(true);
  });
});
```

### 4.3 CI/CD Integration

#### GitHub Actions Post-Deploy Smoke

```yaml
# .github/workflows/deploy.yml
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Deploy
        run: ./deploy.sh
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}

  smoke-test:
    needs: deploy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Wait for deployment to stabilize
        run: sleep 15

      - name: Run smoke tests
        run: ./scripts/smoke-test.sh "${{ vars.PRODUCTION_URL }}"

      - name: Notify on failure
        if: failure()
        uses: slackapi/slack-github-action@v2
        with:
          webhook: ${{ secrets.SLACK_WEBHOOK }}
          webhook-type: incoming-webhook
          payload: |
            {
              "text": "Smoke tests FAILED for ${{ github.sha }}. Deploy may need rollback.",
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": "*Smoke Test Failure*\nCommit: `${{ github.sha }}`\nBranch: `${{ github.ref_name }}`\n<${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View Logs>"
                  }
                }
              ]
            }
```

---

## 5. Rollback Strategies

### 5.1 Strategy Comparison

| Strategy | Rollback Speed | Cost | Complexity | Best For |
|----------|---------------|------|------------|----------|
| **Blue/Green** | Instant (DNS/LB switch) | High (2x infra) | Medium | Mission-critical apps, instant rollback required |
| **Canary** | Fast (stop rollout) | Low (incremental) | High | Large-scale services, risk-averse releases |
| **Rolling** | Medium (redeploy previous) | Low | Low | Standard web apps, Kubernetes deployments |
| **Immutable (revert commit)** | Minutes (CI/CD pipeline) | None | Low | Small teams, simple deployments |

### 5.2 Automatic Rollback on Health Check Failure

The gold standard: deploy fails health checks, system automatically reverts to the last known good version.

#### Fly.io Automatic Rollback

Fly.io rolls back automatically when health checks fail during deployment. No additional configuration needed beyond defining health checks in `fly.toml`. The previous release's Machines remain running until the new release is verified healthy.

#### AWS ECS Circuit Breaker

```yaml
DeploymentConfiguration:
  DeploymentCircuitBreaker:
    Enable: true
    Rollback: true
  MinimumHealthyPercent: 100
  MaximumPercent: 200
```

When enabled, ECS automatically rolls back to the previous stable deployment if the new deployment fails to reach a steady state (health checks pass consistently).

#### Kubernetes Rollback

```yaml
apiVersion: apps/v1
kind: Deployment
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 0
      maxSurge: 1
  minReadySeconds: 30
  progressDeadlineSeconds: 300
```

```bash
# Automatic: Kubernetes rolls back if progressDeadlineSeconds is exceeded
# Manual rollback to previous revision:
kubectl rollout undo deployment/myapp

# Rollback to specific revision:
kubectl rollout undo deployment/myapp --to-revision=3
```

### 5.3 Blue/Green Deployment

Two identical environments. Traffic switches atomically between them.

```
                      ┌─────────────┐
                      │ Load Balancer│
                      └──────┬──────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
        ┌─────▼─────┐  ┌────▼──────┐
        │   BLUE     │  │   GREEN    │
        │ (current)  │  │ (new)      │
        │  v1.4.1    │  │  v1.4.2    │
        └───────────┘  └───────────┘
```

**Process:**
1. Deploy new version to the inactive environment (Green)
2. Run smoke tests against Green
3. Switch load balancer to point to Green
4. Green becomes the new "current," Blue becomes standby
5. On failure: switch load balancer back to Blue (instant rollback)

**Cost trade-off:** You pay for 2x infrastructure at all times. For smaller apps, this is often acceptable. For large-scale services, consider canary instead.

### 5.4 Canary Releases

Route a small percentage of traffic to the new version. Gradually increase if metrics are healthy.

```
Traffic Split:
  Phase 1:  2% → new version,  98% → current version
  Phase 2: 10% → new version,  90% → current version
  Phase 3: 50% → new version,  50% → current version
  Phase 4: 100% → new version
```

**Promotion criteria (automated):**
- Error rate does not increase by > 0.1%
- p95 latency does not increase by > 50ms
- No new error types in logs
- Health endpoint returns 200

**Rollback trigger (automated):**
- Error rate increases by > 1%
- p95 latency increases by > 200ms
- Health endpoint returns 503
- Any 5xx spike detected

### 5.5 Database Migration Rollback

Database rollback is the hardest part of any rollback strategy because data changes are not as easily reversible as code changes.

#### Forward-Only (Recommended)

The forward-only approach deploys a subsequent migration that reverts or compensates for the original changes. This is the recommended approach for production systems because:

1. It maintains the deployment audit trail (important for compliance)
2. It works well with small, incremental migrations
3. It avoids the "did the rollback script actually work?" problem

```
# Migration 20260327120000_add_status_column
ALTER TABLE orders ADD COLUMN status VARCHAR(20) DEFAULT 'pending';

# If rollback needed, create a NEW forward migration:
# Migration 20260327130000_remove_status_column
ALTER TABLE orders DROP COLUMN status;
```

#### Expand-Contract Pattern (Safe Schema Changes)

For breaking schema changes, use expand-contract to maintain backward compatibility:

```
Phase 1 — Expand: Add new column, keep old column
  ALTER TABLE users ADD COLUMN full_name VARCHAR(255);

Phase 2 — Migrate: Copy data, update code to write to both columns
  UPDATE users SET full_name = first_name || ' ' || last_name;
  -- Deploy code that writes to both old and new columns

Phase 3 — Contract: Remove old columns (only after all code uses new column)
  ALTER TABLE users DROP COLUMN first_name;
  ALTER TABLE users DROP COLUMN last_name;
```

Each phase is independently deployable and rollback-safe. If Phase 2 fails, Phase 1 is harmless. If Phase 3 fails, the data is still in the new column.

#### Reversible Migrations (Use with Caution)

```ruby
# Rails example — reversible migration
class AddStatusToOrders < ActiveRecord::Migration[7.1]
  def change
    add_column :orders, :status, :string, default: 'pending'
    add_index :orders, :status
  end
  # Rails auto-generates the reverse (drop_column, remove_index)
end
```

**When reversible migrations break:**
- Data has been written to the new column and is needed
- Other migrations depend on this one
- The migration dropped a column (data is gone)
- The migration involved a data transformation (not reversible)

**Rule:** Always have a database backup taken before migration. Rollback scripts are not a substitute for backups. Column drops and data transformations cannot be recovered without a backup.

---

## 6. Monitoring Integration

### 6.1 Uptime Monitoring

External uptime monitors verify your application is reachable from the public internet. They catch issues that internal health checks miss: DNS failures, CDN misconfigurations, TLS problems, network routing issues.

| Tool | Strengths | Check Interval | Free Tier |
|------|-----------|----------------|-----------|
| **Better Stack** (formerly Better Uptime) | Status pages, incident management, on-call | 30 seconds | 10 monitors |
| **UptimeRobot** | Simple, reliable, long-established | 60 seconds (free), 30s (paid) | 50 monitors |
| **Pingdom** | Detailed performance reports, RUM | 60 seconds | None (paid only) |
| **Cloudflare Health Checks** | Edge-based, multi-region, integrated with CF | 60 seconds | Included with CF |

**What to monitor:**
- Health endpoint (`/health`) — primary indicator
- Homepage — catches routing/CDN issues
- Critical API endpoint — catches backend issues
- Login page — catches auth system issues

**Recommended setup:**
```
Monitor 1: GET /health         → every 60s, alert after 2 failures
Monitor 2: GET /               → every 60s, alert after 3 failures
Monitor 3: GET /api/status     → every 60s, alert after 2 failures
Monitor 4: SSL expiry check    → every 24h, alert at 14 days remaining
```

### 6.2 Error Rate Spike Detection

Deploy-correlated error spikes are the most common post-deployment failure mode. The deploy succeeds, health checks pass, but a specific code path throws errors under real traffic.

| Tool | Strengths | Cost |
|------|-----------|------|
| **Sentry** | Stack traces, release tracking, session replay | Free tier generous |
| **Honeybadger** | Rails/Ruby focus, simple, fast | From $49/mo |
| **Bugsnag** | Multi-platform, stability scores | Free tier available |
| **Better Stack** (error tracking) | Sentry-compatible, integrated with uptime | Included with plan |

**Release-aware error tracking (Sentry example):**

```typescript
// sentry.config.ts
import * as Sentry from '@sentry/nextjs';

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  release: process.env.NEXT_PUBLIC_VERSION,  // Tag errors by release
  environment: process.env.NODE_ENV,
  tracesSampleRate: 0.1,
  integrations: [
    Sentry.replayIntegration({
      maskAllText: true,
      blockAllMedia: true,
    }),
  ],
});
```

```bash
# Notify Sentry of a new release (in deploy script)
sentry-cli releases new "$VERSION"
sentry-cli releases set-commits "$VERSION" --auto
sentry-cli releases finalize "$VERSION"
sentry-cli releases deploys "$VERSION" new -e production
```

This enables Sentry to show you: "Error rate increased 340% since release v1.4.2 was deployed 15 minutes ago."

### 6.3 Structured Logging Post-Deploy

Deploy events should be visible in your log stream so you can correlate issues with deployments.

```json
{
  "level": "info",
  "event": "deploy.completed",
  "version": "1.4.2",
  "git_sha": "a3f8b2c",
  "environment": "production",
  "deployed_by": "github-actions",
  "duration_seconds": 94,
  "timestamp": "2026-03-27T14:30:00Z"
}
```

**Log-based alerts to configure post-deploy:**
- Alert on any `level: "error"` log within 15 minutes of `deploy.completed`
- Alert on error rate exceeding 2x the pre-deploy baseline
- Alert on response time p95 exceeding 2x the pre-deploy baseline

### 6.4 Performance Baseline Comparison

After every deploy, compare key metrics to the pre-deploy baseline:

| Metric | Acceptable Threshold | Alert Threshold |
|--------|---------------------|-----------------|
| p50 response time | < 1.2x baseline | > 2x baseline |
| p95 response time | < 1.5x baseline | > 3x baseline |
| Error rate | < 1.1x baseline | > 2x baseline |
| Apdex score | > 0.9x baseline | < 0.8x baseline |
| Memory usage | < 1.3x baseline | > 2x baseline |
| CPU usage | < 1.3x baseline | > 2x baseline |

**Implementation:** Most APM tools (Datadog, New Relic, Grafana Cloud) support deploy markers that automatically create before/after comparisons.

---

## 7. Notification Patterns

### 7.1 What to Notify

| Event | Priority | Channel |
|-------|----------|---------|
| Deploy started | Low | Deploy channel (Slack/Discord) |
| Deploy succeeded | Medium | Deploy channel + GitHub deployment status |
| Deploy failed | High | Deploy channel + DM to deployer + PagerDuty |
| Smoke tests passed | Medium | Deploy channel |
| Smoke tests failed | Critical | Deploy channel + DM to deployer + PagerDuty |
| Rollback triggered | Critical | Deploy channel + DM to deployer + all-eng channel |

### 7.2 Notification Content

A good deploy notification includes:

```
Deploy Succeeded
---
Environment: production
Version:     v1.4.2 (a3f8b2c)
Deployed by: @bryan via GitHub Actions
Duration:    1m 34s
Health:      All checks passing
Smoke:       12/12 passed
URL:         https://myapp.com
Commit:      fix: resolve cart calculation rounding error
```

A bad deploy notification:

```
Deploy done.
```

### 7.3 Slack Webhook Implementation

#### GitHub Actions

```yaml
# .github/workflows/deploy.yml
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Notify deploy start
        uses: slackapi/slack-github-action@v2
        with:
          webhook: ${{ secrets.SLACK_WEBHOOK }}
          webhook-type: incoming-webhook
          payload: |
            {
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": ":rocket: *Deploy started*\nVersion: `${{ github.sha }}` → `production`\nBy: ${{ github.actor }}"
                  }
                }
              ]
            }

      - name: Deploy
        id: deploy
        run: ./deploy.sh

      - name: Notify deploy success
        if: success()
        uses: slackapi/slack-github-action@v2
        with:
          webhook: ${{ secrets.SLACK_WEBHOOK }}
          webhook-type: incoming-webhook
          payload: |
            {
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": ":white_check_mark: *Deploy succeeded*\nVersion: `${{ github.sha }}`\nEnvironment: `production`\nDuration: ${{ steps.deploy.outputs.duration }}\n<${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View Logs>"
                  }
                }
              ]
            }

      - name: Notify deploy failure
        if: failure()
        uses: slackapi/slack-github-action@v2
        with:
          webhook: ${{ secrets.SLACK_WEBHOOK }}
          webhook-type: incoming-webhook
          payload: |
            {
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": ":x: *Deploy FAILED*\nVersion: `${{ github.sha }}`\nEnvironment: `production`\nBy: ${{ github.actor }}\n<${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View Logs>"
                  }
                }
              ]
            }
```

### 7.4 GitHub Deployment Status API

```yaml
# Create a deployment record in GitHub
- name: Create GitHub deployment
  uses: chrnorm/deployment-action@v2
  id: deployment
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
    environment: production

# After deploy, update status
- name: Update deployment status
  if: always()
  uses: chrnorm/deployment-status@v2
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
    deployment-id: ${{ steps.deployment.outputs.deployment_id }}
    state: ${{ job.status == 'success' && 'success' || 'failure' }}
    environment-url: https://myapp.com
```

This creates a deployment record visible in the GitHub UI under "Environments," providing a full deploy history with links to commits, PRs, and logs.

---

## 8. Anti-Patterns

| Anti-Pattern | Why It Is Wrong | Do Instead |
|---|---|---|
| Deploying without health checks | You have no signal that the deploy worked beyond "the command exited 0" | Define health checks in your platform config; verify they pass before declaring success |
| Health endpoint that always returns 200 | A health check that does not check dependencies is decorative, not functional | Check database, cache, and critical service connectivity; return 503 when unhealthy |
| Health endpoint that runs full test suite | Deep checks on every request overload dependencies and slow responses | Separate shallow (liveness) and deep (readiness) checks; deep checks on demand only |
| Ignoring migration errors | Database is in an inconsistent state; application will fail on data access | Use `release_command` or equivalent; fail the deploy if migrations fail |
| Manual-only verification | Humans forget steps, skip checks when rushed, and cannot verify at 3 AM | Automate smoke tests in CI/CD; make verification a required deploy step |
| No rollback plan | When the deploy is broken, you scramble to figure out how to undo it | Define rollback procedure before deploying; test it periodically |
| Checking only the homepage | Homepage is cached, uses no database, and tests almost nothing | Test health endpoint, critical API paths, auth flow, and at least one database-backed page |
| Hard-coding health check URLs | Environment changes break checks; staging and production diverge | Use environment variables or platform-provided URLs for health check targets |
| Caching health check responses | Load balancer gets a stale "healthy" response while the app is actually down | Add `Cache-Control: no-cache, no-store` to all health responses |
| Health check with authentication | Load balancers and uptime monitors cannot authenticate; checks fail on healthy systems | Health endpoints must be unauthenticated; limit detail exposure instead |
| Deploying on Friday afternoon | Reduced team availability for rollback; weekend traffic patterns mask issues | Deploy early in the work week; use feature flags for late-week changes |
| No deploy notifications | Team is unaware a deploy happened; correlating issues with deploys is manual | Notify on deploy start, success, and failure; include version and environment |
| Mixing infrastructure and application in one health check | A database blip makes the entire health check fail; load balancer removes healthy servers | Separate liveness (is the process alive?) from readiness (can it serve traffic?) |
| Using health checks as monitoring | Health checks verify a moment in time; they do not track trends or detect gradual degradation | Use health checks for deploy verification; use monitoring for ongoing observability |
| Rolling back code without rolling back data | Code expects old schema but database has new schema; application crashes | Use expand-contract migrations; ensure code is backward-compatible with both schema versions |
| Testing rollback for the first time during an incident | Untested rollback procedures fail when you need them most | Run rollback drills quarterly; document the exact steps |

---

## 9. Audit Dimensions

Use these dimensions to evaluate any project's post-deployment verification:

1. **health-endpoint** — structured JSON health endpoint exists, checks all critical dependencies, returns 503 when unhealthy, has `Cache-Control: no-cache`
2. **health-depth** — shallow and deep checks are separated; liveness and readiness probes exist for orchestrated environments
3. **platform-checks** — platform-native health checks configured (fly.toml checks, ECS health check, Docker HEALTHCHECK, etc.)
4. **release-command** — database migrations run before deploy via release command or equivalent; migration failure halts deploy
5. **smoke-tests** — automated smoke test suite exists covering infrastructure, application, and integration layers; runs in CI/CD post-deploy
6. **smoke-coverage** — smoke tests cover: homepage, health endpoint, critical API, auth flow, static assets, SSL validity
7. **rollback-strategy** — documented rollback procedure exists; has been tested within the last quarter
8. **rollback-automation** — platform-level automatic rollback on health check failure is enabled (circuit breaker, deployment strategy)
9. **migration-safety** — database migrations use expand-contract or forward-only patterns; destructive changes have backups
10. **uptime-monitoring** — external uptime monitor checks health endpoint and critical paths from multiple regions
11. **error-tracking** — release-aware error tracking (Sentry or equivalent) is configured; deploy markers are set
12. **performance-baseline** — post-deploy performance is compared to pre-deploy baseline; alerts fire on regression
13. **notifications** — deploy started/succeeded/failed notifications go to appropriate channels with version, environment, and duration
14. **deploy-history** — GitHub deployment status or equivalent provides a visible history of all deploys per environment
15. **incident-readiness** — rollback drills are performed periodically; on-call knows the rollback procedure
