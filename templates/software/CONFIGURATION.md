---
title: Configuration Reference
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Configuration Reference

> Complete configuration reference for [project name]. Every configurable option documented.

## Configuration Sources

Configuration is loaded in the following order (later sources override earlier ones):

| Priority | Source | Example | Purpose |
|---|---|---|---|
| 1 (lowest) | Built-in defaults | Compiled into application | Safe baseline |
| 2 | Configuration file | `config/default.yaml` | Per-installation settings |
| 3 | Environment-specific file | `config/production.yaml` | Per-environment overrides |
| 4 | Environment variables | `DATABASE_URL=...` | Deployment-specific settings |
| 5 (highest) | Command-line flags | `--port 8080` | Runtime overrides |

## Environment Variables

### Required

These MUST be set for the application to start.

| Variable | Type | Example | Description |
|---|---|---|---|
| `DATABASE_URL` | string | `postgres://user:pass@host:5432/dbname` | Primary database connection string |
| `SECRET_KEY` | string | `[64-char hex string]` | Application secret for signing tokens and sessions. Generate with `openssl rand -hex 32` |
| `[VARIABLE]` | [type] | `[example]` | [description] |

### Optional

These have defaults and can be omitted.

| Variable | Type | Default | Example | Description |
|---|---|---|---|---|
| `PORT` | integer | `8080` | `3000` | HTTP server listen port |
| `HOST` | string | `0.0.0.0` | `127.0.0.1` | HTTP server bind address |
| `LOG_LEVEL` | string | `info` | `debug` | Log verbosity: `trace`, `debug`, `info`, `warn`, `error` |
| `LOG_FORMAT` | string | `json` | `text` | Log output format: `json`, `text` |
| `WORKERS` | integer | `[CPU count]` | `4` | Number of worker threads/processes |
| `[VARIABLE]` | [type] | `[default]` | `[example]` | [description] |

### Database

| Variable | Type | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | string | (required) | Connection string: `postgres://user:pass@host:port/dbname` |
| `DATABASE_POOL_SIZE` | integer | `10` | Maximum connections in the pool |
| `DATABASE_POOL_TIMEOUT` | integer | `5000` | Connection acquisition timeout in milliseconds |
| `DATABASE_SSL_MODE` | string | `prefer` | SSL mode: `disable`, `prefer`, `require`, `verify-full` |
| `DATABASE_STATEMENT_TIMEOUT` | integer | `30000` | Query timeout in milliseconds (0 = no limit) |

### Cache

| Variable | Type | Default | Description |
|---|---|---|---|
| `REDIS_URL` | string | (none) | Redis connection string. If not set, caching is disabled |
| `CACHE_TTL` | integer | `3600` | Default cache entry TTL in seconds |
| `CACHE_PREFIX` | string | `[project]` | Key prefix for cache entries |

### Email / Notifications

| Variable | Type | Default | Description |
|---|---|---|---|
| `SMTP_HOST` | string | (none) | SMTP server hostname |
| `SMTP_PORT` | integer | `587` | SMTP server port |
| `SMTP_USER` | string | (none) | SMTP authentication username |
| `SMTP_PASSWORD` | string | (none) | SMTP authentication password |
| `EMAIL_FROM` | string | (none) | Default sender address |

### External Services

| Variable | Type | Default | Description |
|---|---|---|---|
| `[SERVICE]_API_URL` | string | `[default URL]` | [Service name] API base URL |
| `[SERVICE]_API_KEY` | string | (required if using [service]) | [Service name] API key |
| `[SERVICE]_TIMEOUT` | integer | `10000` | [Service name] request timeout in milliseconds |
| `[SERVICE]_RETRY_COUNT` | integer | `3` | [Service name] retry attempts on failure |

### Feature Flags

| Variable | Type | Default | Description |
|---|---|---|---|
| `FEATURE_[NAME]` | boolean | `false` | Enable [feature description] |
| `FEATURE_[NAME]` | boolean | `true` | Enable [feature description] |

## Configuration File Format

### Location

The configuration file is loaded from (first found):

1. Path specified by `--config` flag
2. `./config/default.yaml`
3. `/etc/[project-name]/config.yaml`

### Format

```yaml
# config/default.yaml

server:
  host: "0.0.0.0"
  port: 8080
  workers: 4
  request_timeout: 30s
  max_body_size: "10MB"

database:
  pool_size: 10
  pool_timeout: 5s
  statement_timeout: 30s
  ssl_mode: prefer

cache:
  enabled: true
  ttl: 3600
  prefix: "[project]"

logging:
  level: info
  format: json
  output: stdout

# [Additional sections as needed]
```

### Environment Overrides

Environment-specific files override the defaults:

```yaml
# config/production.yaml (overrides default.yaml)

server:
  workers: 8

database:
  pool_size: 25
  ssl_mode: verify-full

logging:
  level: warn
```

## Command-Line Flags

| Flag | Short | Type | Description |
|---|---|---|---|
| `--config` | `-c` | string | Path to configuration file |
| `--port` | `-p` | integer | Override HTTP listen port |
| `--log-level` | `-l` | string | Override log level |
| `--version` | `-v` | boolean | Print version and exit |
| `--help` | `-h` | boolean | Print help and exit |
| `[--flag]` | `[-f]` | [type] | [description] |

## Validation

The application validates all configuration at startup. Invalid configuration causes the application to exit with a non-zero code and a descriptive error message.

### Validation Rules

| Setting | Validation | Error |
|---|---|---|
| `PORT` | 1-65535 | "PORT must be between 1 and 65535" |
| `DATABASE_URL` | Valid connection string | "DATABASE_URL must be a valid connection string" |
| `LOG_LEVEL` | One of: trace, debug, info, warn, error | "LOG_LEVEL must be one of: trace, debug, info, warn, error" |
| `SECRET_KEY` | Minimum 32 characters | "SECRET_KEY must be at least 32 characters" |
| `[setting]` | [rule] | "[error message]" |

## Environment-Specific Notes

### Development

```bash
# Minimal .env for local development
DATABASE_URL=postgres://localhost:5432/project_dev
SECRET_KEY=dev-secret-key-not-for-production-use-minimum-32-chars
LOG_LEVEL=debug
```

### Staging

[Any staging-specific configuration notes.]

### Production

[Production-specific configuration requirements and recommendations.]

- `SECRET_KEY` MUST be a cryptographically random value
- `DATABASE_SSL_MODE` SHOULD be `verify-full`
- `LOG_LEVEL` SHOULD be `info` or `warn` (not `debug`)
- [Other production hardening requirements]

## Secrets

The following configuration values are secrets and MUST NOT be committed to version control:

| Secret | Storage | Rotation Frequency |
|---|---|---|
| `SECRET_KEY` | [Vault / Secrets Manager / env] | [Annually / on compromise] |
| `DATABASE_URL` (password portion) | [Vault / Secrets Manager / env] | [Quarterly] |
| `SMTP_PASSWORD` | [Vault / Secrets Manager / env] | [Annually] |
| `[SERVICE]_API_KEY` | [Vault / Secrets Manager / env] | [As needed] |

## Configuration Changelog

| Date | Change | Plan |
|---|---|---|
| [YYYY-MM-DD] | [Added DATABASE_POOL_SIZE option] | [PLAN-XXXX] |
| [YYYY-MM-DD] | [Changed default LOG_LEVEL from debug to info] | [PLAN-XXXX] |

---

## Related Documents

- [Deployment](DEPLOYMENT.md) — Environment-specific deployment configuration
- [Development](DEVELOPMENT.md) — Local development configuration
- [Security](../SECURITY.md) — Secrets management
- [Operations](OPERATIONS.md) — Operational configuration changes
