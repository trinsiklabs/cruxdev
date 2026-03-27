# Domain Inventory: [Domain Name]

> **Domain:** [domain-name]
> **Owner:** [Name]
> **Last Audited:** YYYY-MM-DD
> **Next Audit Due:** YYYY-MM-DD

---

## 1. Repositories

| Repository | Location | Purpose | Primary Language | Status |
|---|---|---|---|---|
| [repo-name] | [URL or path] | [What it does] | [Language] | Active / Archived / Deprecated |

---

## 2. Services

### Running Services

| Service | Type | Host | Port | Systemd Unit | Status |
|---|---|---|---|---|---|
| [service-name] | Daemon / Timer / One-shot | [hostname] | [port or N/A] | [unit-name.service] | Running / Stopped / Degraded |

### Timers and Scheduled Jobs

| Timer | Schedule | Triggers | Purpose | Last Success |
|---|---|---|---|---|
| [timer-name] | [OnCalendar or cron expression] | [service-name] | [What it does] | YYYY-MM-DD HH:MM |

---

## 3. Tools

| Tool | Path | Purpose | Inputs | Outputs |
|---|---|---|---|---|
| [tool-name] | [/usr/local/bin/... or relative path] | [What it does] | [What it takes] | [What it produces] |

---

## 4. Bots

| Bot | Role | Autonomy Level | Primary Domain? | Connection Pattern |
|---|---|---|---|---|
| [bot-name] | [What it does in this domain] | Full / Supervised / Read-only | Yes / No | Persistent / On-demand / Scheduled |

---

## 5. Data Stores

| Store | Type | Location | Size | Backup? | Retention |
|---|---|---|---|---|---|
| [store-name] | SQLite / Postgres / Files / KV | [path or connection] | [Approx size] | Yes / No | [Policy] |

---

## 6. Configuration Files

| File | Location | Purpose | Sensitive? |
|---|---|---|---|
| [config-name] | [path] | [What it configures] | Yes / No |

---

## 7. External Dependencies

| Dependency | Type | Provider | Used For | Fallback |
|---|---|---|---|---|
| [dep-name] | API / Library / Service | [Who provides it] | [What we use it for] | [What happens if unavailable] |

---

## 8. Sync Point Contents

Current state of this domain's sync point at `/srv/sync/[domain-name]/`:

```
docs/
  [list current files]
plans/
  [list current plans]
artifacts/
  [list current artifacts]
config/
  [list current config files]
```

---

## 9. Inventory Health

### Missing Items
<!-- Things that should exist in this domain but do not yet. -->

- [ ] [Missing item 1]
- [ ] [Missing item 2]

### Deprecated Items
<!-- Things that exist but should be removed. -->

| Item | Type | Deprecation Date | Removal Plan |
|---|---|---|---|
| [item] | [type] | YYYY-MM-DD | [When and how to remove] |

### Orphaned Items
<!-- Things that exist but have no clear owner or purpose. -->

| Item | Type | Last Known Purpose | Disposition |
|---|---|---|---|
| [item] | [type] | [What it was for] | Keep / Remove / Investigate |

---

## 10. Audit Trail

| Date | Auditor | Findings | Actions Taken |
|---|---|---|---|
| YYYY-MM-DD | [Who] | [What was found] | [What was done] |
