---
title: Security Documentation
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Security Documentation

> Security posture, threat model, and vulnerability reporting for [project name].

## Reporting Vulnerabilities

**DO NOT file security vulnerabilities as public issues.**

To report a security vulnerability:

1. Email: [security@example.com]
2. Include: description of the vulnerability, steps to reproduce, potential impact
3. Expected response time: [X business days] for acknowledgment, [Y days] for resolution
4. We follow [responsible disclosure / coordinated disclosure] practices

## Security Overview

### Authentication

| Component | Method | Details |
|---|---|---|
| User authentication | [Password + MFA / OAuth2 / SSO] | [Implementation details] |
| API authentication | [API key / Bearer token / mTLS] | [Implementation details] |
| Service-to-service | [mTLS / shared secret / JWT] | [Implementation details] |
| Admin access | [Method] | [Additional controls] |

### Authorization

| Model | Description |
|---|---|
| Authorization type | [RBAC / ABAC / ACL / custom] |
| Permission granularity | [Per-resource / per-action / per-field] |
| Default policy | [Deny-all unless explicitly granted / etc.] |
| Privilege escalation prevention | [How the system prevents unauthorized privilege escalation] |

### Roles and Permissions

| Role | Permissions | Assignment Method |
|---|---|---|
| `admin` | Full system access | Manual assignment by superadmin |
| `user` | CRUD own resources; read shared resources | Default on registration |
| `readonly` | Read-only access to authorized resources | Manual assignment |
| `[role]` | [permissions] | [how assigned] |

## Data Protection

### Data Classification

| Classification | Examples | Handling Requirements |
|---|---|---|
| **Public** | Marketing content, public API docs | No restrictions |
| **Internal** | Internal metrics, non-sensitive configs | Authenticated access only |
| **Confidential** | User PII, API keys, financial data | Encrypted at rest and in transit; access logged |
| **Restricted** | Passwords, secrets, encryption keys | Never stored in plaintext; minimal access; audit trail |

### Encryption

| Context | Method | Details |
|---|---|---|
| Data in transit | TLS [version] | [Certificate management: Let's Encrypt / internal CA / etc.] |
| Data at rest | [AES-256 / database-level / disk-level] | [Key management: KMS / Vault / etc.] |
| Passwords | [bcrypt / argon2id] with cost factor [N] | Never stored in plaintext or reversible form |
| API keys | [Hashing method] | Only hash stored; plaintext shown once at creation |
| Backups | [Encryption method] | [Key management for backup encryption] |

### PII Handling

| Data Element | Stored | Encrypted | Retention | Deletion Method |
|---|---|---|---|---|
| Email address | Yes | [At rest] | Account lifetime + [X days] | Hard delete on purge |
| Name | Yes | [At rest] | Account lifetime + [X days] | Hard delete on purge |
| IP address | Logs only | No | [X days] log retention | Log rotation |
| [Element] | [Yes/No] | [Method] | [Period] | [Method] |

## Threat Model

### Attack Surface

| Surface | Exposure | Controls |
|---|---|---|
| Public API | Internet | Rate limiting, authentication, input validation, WAF |
| Admin interface | [Internal network / VPN] | MFA, IP allowlist, audit logging |
| Database | Internal network only | Network segmentation, strong credentials, encrypted connections |
| File uploads | Public API | File type validation, size limits, malware scanning, isolated storage |
| [Surface] | [Exposure] | [Controls] |

### OWASP Top 10 Mitigations

| Risk | Mitigation | Status |
|---|---|---|
| A01: Broken Access Control | RBAC enforcement on every endpoint; default deny | [Implemented / Partial / Planned] |
| A02: Cryptographic Failures | TLS everywhere; secrets in Vault; bcrypt for passwords | [Implemented / Partial / Planned] |
| A03: Injection | Parameterized queries; ORM; input validation | [Implemented / Partial / Planned] |
| A04: Insecure Design | Threat modeling; security reviews; ADRs for security decisions | [Implemented / Partial / Planned] |
| A05: Security Misconfiguration | Hardened defaults; security headers; no debug in prod | [Implemented / Partial / Planned] |
| A06: Vulnerable Components | Dependency scanning; automated updates; SBOM | [Implemented / Partial / Planned] |
| A07: Auth Failures | Rate limiting on login; account lockout; MFA | [Implemented / Partial / Planned] |
| A08: Data Integrity Failures | Signed artifacts; verified dependencies; CI/CD security | [Implemented / Partial / Planned] |
| A09: Logging Failures | Structured logging; security event alerting; log integrity | [Implemented / Partial / Planned] |
| A10: SSRF | URL validation; allowlisted domains; network segmentation | [Implemented / Partial / Planned] |

### Known Risks and Accepted Residual Risk

| Risk | Severity | Mitigation | Residual Risk | Acceptance |
|---|---|---|---|---|
| [Risk description] | [High/Med/Low] | [What we do about it] | [What remains] | [Accepted by whom, when] |

## Security Headers

| Header | Value | Purpose |
|---|---|---|
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Force HTTPS |
| `Content-Security-Policy` | `[policy]` | Prevent XSS and data injection |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME type sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referrer information |
| `Permissions-Policy` | `[policy]` | Control browser features |

## Dependency Security

### Scanning

- **Tool:** [Dependabot / Snyk / Trivy / etc.]
- **Frequency:** [Every PR / daily / weekly]
- **Policy:** [Critical: fix within 24h; High: fix within 7d; Medium: fix within 30d]

### SBOM

- **Format:** [CycloneDX / SPDX]
- **Location:** [Where the SBOM is generated and stored]
- **Update frequency:** [Every release / every build]

## Audit Logging

### What Is Logged

| Event | Data Captured | Retention |
|---|---|---|
| Authentication (success/failure) | User ID, IP, timestamp, method | [X days] |
| Authorization failure | User ID, resource, action, IP | [X days] |
| Data access (sensitive) | User ID, resource type, resource ID | [X days] |
| Admin actions | Admin ID, action, target, before/after | [X days] |
| Configuration changes | User ID, setting, old value, new value | [X days] |
| [Event] | [Data] | [Retention] |

### What Is NOT Logged

- Passwords or password hashes
- Full API keys or secrets
- PII beyond what is needed for audit (no logging of request bodies containing PII)

### Log Integrity

[How log integrity is ensured: append-only storage, cryptographic chaining, SIEM integration, etc.]

## Incident Response

### Security Incident Classification

| Severity | Definition | Response Time | Example |
|---|---|---|---|
| **Critical** | Active exploitation; data breach confirmed | Immediate (< 1 hour) | Database breach, credential leak |
| **High** | Exploitable vulnerability; no active exploitation | < 4 hours | Unpatched critical CVE, auth bypass |
| **Medium** | Vulnerability with limited exploitability | < 24 hours | XSS in admin panel, info disclosure |
| **Low** | Minor security concern | < 7 days | Missing security header, verbose errors |

### Response Procedure

```
1. DETECT: Alert triggered or report received
2. TRIAGE: Classify severity (table above)
3. CONTAIN: Isolate affected systems if needed
4. INVESTIGATE: Determine scope and root cause
5. REMEDIATE: Fix the vulnerability
6. RECOVER: Restore normal operations
7. REVIEW: Post-incident review and lessons learned
8. COMMUNICATE: Notify affected parties per disclosure policy
```

## Compliance

| Standard | Applicability | Status | Evidence |
|---|---|---|---|
| [GDPR / SOC2 / HIPAA / PCI-DSS / etc.] | [Which parts] | [Compliant / In progress / N/A] | [Where evidence lives] |

---

## Related Documents

- [Architecture](docs/ARCHITECTURE.md) — Security architecture context
- [API Reference](docs/API.md) — API authentication details
- [Operations](docs/OPERATIONS.md) — Security-related runbooks
- [Configuration](docs/CONFIGURATION.md) — Security-related configuration
