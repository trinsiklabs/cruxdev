---
title: Integration Guide
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Integration Guide

> Third-party service integrations used by [project name]: what they do, how they connect, and how they fail.

## Integration Inventory

| Service | Purpose | Protocol | Criticality | Fallback |
|---|---|---|---|---|
| [Service A] | [What it does for us] | [REST / gRPC / SDK] | Critical | [None / degrade gracefully / queue] |
| [Service B] | [What it does for us] | [REST / SDK] | Important | [Cache / default values] |
| [Service C] | [What it does for us] | [Webhook] | Nice-to-have | [Skip / retry later] |

## [Service A Name]

### Overview

- **Provider:** [Company name]
- **Purpose:** [What this service does for our system]
- **Documentation:** [Link to provider's API docs]
- **Dashboard / Console:** [Link to management console]
- **Status page:** [Link to service status page]
- **Support contact:** [Email / chat / support portal]

### Authentication

| Credential | Type | Storage | Rotation |
|---|---|---|---|
| `[SERVICE_A_API_KEY]` | API Key | [Vault / env var] | [Frequency] |
| `[SERVICE_A_SECRET]` | Shared secret | [Vault / env var] | [Frequency] |

### Configuration

| Variable | Default | Description |
|---|---|---|
| `SERVICE_A_API_URL` | `https://api.service-a.com/v2` | Base URL |
| `SERVICE_A_API_KEY` | (required) | API key |
| `SERVICE_A_TIMEOUT` | `10000` | Request timeout in ms |
| `SERVICE_A_RETRY_COUNT` | `3` | Max retry attempts |

### Endpoints Used

| Our Action | Their Endpoint | Method | Purpose |
|---|---|---|---|
| [Create widget] | `POST /widgets` | REST | [Create a widget in Service A] |
| [Get widget] | `GET /widgets/:id` | REST | [Retrieve widget details] |
| [List events] | `GET /events?since=:ts` | REST | [Poll for new events] |

### Data Mapping

| Our Field | Their Field | Transform | Notes |
|---|---|---|---|
| `user.email` | `customer.email_address` | None | Direct mapping |
| `order.total` | `payment.amount_cents` | `* 100` (dollars to cents) | They use cents |
| `status` | `state` | Enum mapping (see below) | Different status names |

**Status Mapping:**

| Our Status | Their Status |
|---|---|
| `active` | `ENABLED` |
| `suspended` | `PAUSED` |
| `deleted` | `ARCHIVED` |

### Rate Limits

| Tier | Limit | Window | Our Usage |
|---|---|---|---|
| [Our tier] | [X] requests | per [minute/hour] | [Y] requests typical |

### Error Handling

| Their Error | Our Response | Retry | Notes |
|---|---|---|---|
| `401 Unauthorized` | Log + alert (credential issue) | No | Credential rotation needed |
| `429 Too Many Requests` | Backoff and retry | Yes (exponential) | Respect `Retry-After` header |
| `500 Internal Server Error` | Retry | Yes (3 attempts) | Transient; alert if persistent |
| `503 Service Unavailable` | Circuit break; use fallback | Yes (after circuit resets) | Check their status page |
| Timeout | Retry | Yes (2 attempts) | May indicate their load issues |

### Failure Mode

- **If Service A is down:** [What happens to our system. What features degrade. What the user sees.]
- **Circuit breaker:** [Configuration: trips after N failures in M seconds; resets after T seconds]
- **Fallback:** [What we do when the circuit is open: cached data / default values / error message]
- **Recovery:** [What happens when Service A comes back. Is there a reconciliation step?]

### Testing

- **Sandbox/Test environment:** [URL or how to access]
- **Test credentials:** [How to obtain; stored where]
- **Mock strategy:** [How integration tests handle this service: mock server / recorded responses / sandbox]

---

## [Service B Name]

### Overview

- **Provider:** [Company name]
- **Purpose:** [What this service does for our system]
- **Documentation:** [Link to provider's API docs]
- **Status page:** [Link]

### Authentication

[Same structure as Service A]

### Configuration

[Same structure as Service A]

### Endpoints Used

[Same structure as Service A]

### Error Handling

[Same structure as Service A]

### Failure Mode

[Same structure as Service A]

---

## Webhooks We Receive

### [Webhook from Service A]

| Property | Value |
|---|---|
| **Endpoint** | `POST /webhooks/service-a` |
| **Authentication** | [HMAC signature in `X-Signature` header / shared secret / etc.] |
| **Events** | [List of event types we handle] |

**Verification:**

```
[How to verify webhook authenticity, e.g.:]
signature = HMAC-SHA256(webhook_secret, request_body)
Compare signature with X-Signature header
```

**Events:**

| Event Type | Trigger | Our Action |
|---|---|---|
| `widget.created` | A widget was created in Service A | [What we do] |
| `widget.updated` | A widget was modified | [What we do] |
| `payment.completed` | Payment succeeded | [What we do] |

**Retry Policy (from provider):**

- [Provider retries N times with exponential backoff]
- [We must return 2xx within X seconds or they retry]

**Idempotency:**

- [How we handle duplicate webhook deliveries: idempotency key / deduplication / etc.]

## Webhooks We Send

### [Webhook to External Consumer]

| Property | Value |
|---|---|
| **Events** | [List of events we emit] |
| **Payload format** | JSON |
| **Signature** | [HMAC-SHA256 in `X-Webhook-Signature` header] |
| **Retry policy** | [N retries, exponential backoff, X max wait] |
| **Timeout** | [X seconds per delivery attempt] |

See [API.md](API.md) for full webhook documentation.

## Integration Health Monitoring

| Service | Health Check | Frequency | Alert Threshold |
|---|---|---|---|
| [Service A] | [Ping endpoint / test call] | [Every X minutes] | [N failures in M minutes] |
| [Service B] | [Status page check] | [Every X minutes] | [Any failure] |

## Data Synchronization

[If any integrations involve data sync, document the sync model here.]

| Data | Direction | Frequency | Conflict Resolution |
|---|---|---|---|
| [Users] | Our system → Service A | [Real-time / hourly / daily] | [Our system is source of truth] |
| [Events] | Service A → Our system | [Webhook / polling every X min] | [Service A is source of truth] |

---

## Related Documents

- [Architecture](ARCHITECTURE.md) — How integrations fit into the system
- [API Reference](API.md) — Our webhooks and API that integrations use
- [Configuration](CONFIGURATION.md) — Integration configuration settings
- [Security](../SECURITY.md) — Credential management
- [Operations](OPERATIONS.md) — Integration failure runbooks
