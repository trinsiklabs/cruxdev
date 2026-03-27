---
title: Troubleshooting Guide
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Troubleshooting Guide

> Common problems, error messages, and solutions for [project name].

## Quick Diagnostic Checklist

When something goes wrong, check these first:

1. **Is the service running?** `[health check command]`
2. **Are dependencies up?** `[dependency check command]`
3. **What do the logs say?** `[log tail command]`
4. **Was there a recent deployment?** `[deploy history command]`
5. **Is it environment-specific?** Test in another environment

## Error Messages

### "[Exact error message or pattern]"

**Symptom:** [What the user/operator sees]

**Cause:** [Why this happens]

**Solution:**

```bash
# Step-by-step fix
[command 1]
[command 2]
```

**Prevention:** [How to prevent this from happening again]

---

### "Connection refused" / "ECONNREFUSED"

**Symptom:** Application fails to start or intermittently drops requests

**Causes and Solutions:**

| Cause | How to Verify | Solution |
|---|---|---|
| Database not running | `[db status command]` | Start database: `[start command]` |
| Wrong connection string | Check `DATABASE_URL` value | Fix the connection string in configuration |
| Port conflict | `lsof -i :[port]` | Kill the conflicting process or change port |
| Firewall blocking | `[firewall check command]` | Open the required port |

---

### "Out of memory" / OOM Killed

**Symptom:** Application process is killed by the OS; service restarts

**Cause:** Application memory usage exceeds the available memory or cgroup limit

**Solution:**

1. Check current memory usage: `[memory check command]`
2. Check for memory leaks: `[profiling command]`
3. Short-term: Increase memory limit or restart the service
4. Long-term: Profile and fix the memory leak

**Common memory leak sources:**
- [Unbounded caches or maps]
- [Unclosed database connections]
- [Large file reads into memory]
- [Event listeners not removed]

---

### "Permission denied"

**Symptom:** Application cannot read/write files, connect to sockets, or bind to ports

**Causes and Solutions:**

| Cause | How to Verify | Solution |
|---|---|---|
| File permissions | `ls -la [path]` | `chmod`/`chown` to correct permissions |
| Port < 1024 | Check configured port | Use port > 1024 or grant capability: `setcap cap_net_bind_service=+ep [binary]` |
| SELinux/AppArmor | `[security module status command]` | Add appropriate policy |
| Database user permissions | `[db permission check]` | Grant required permissions |

---

### "Migration failed"

**Symptom:** Database migration exits with an error; application refuses to start

**Solution:**

1. Check migration status: `[migration status command]`
2. Read the error message — it usually identifies the failing migration and SQL statement
3. Common causes:
   - **Column already exists:** Migration was partially applied. Check schema state and either manually complete or roll back
   - **Foreign key violation:** Data exists that violates a new constraint. Fix data first, then retry
   - **Lock timeout:** Another process holds a lock. Retry when database is less busy
4. If stuck: `[manual migration fix procedure]`

---

### "[API returns 500 Internal Server Error]"

**Symptom:** API requests return 500 errors

**Diagnostic Steps:**

1. Check logs for the full error: `[log command with request ID filter]`
2. Is it all endpoints or just one? Test with: `curl -v [health endpoint]`
3. Is it all requests or intermittent? Check error rate dashboard
4. Common causes:
   - Unhandled exception in application code (check stack trace in logs)
   - Database connection failure (check DB connectivity)
   - External service failure (check integration health)
   - Configuration error after deployment (check recent config changes)

---

### "[Slow response times]"

**Symptom:** Requests take much longer than normal

**Diagnostic Steps:**

1. Check current latency: `[latency dashboard or curl timing command]`
2. Identify slow component:
   - Application processing: `[APM or profiling command]`
   - Database queries: `[slow query log command]`
   - External services: `[external service latency check]`
   - Network: `[network diagnostic command]`
3. Common causes:
   - Missing database index (check query plans)
   - N+1 query pattern (check query count per request)
   - Lock contention (check database locks)
   - Resource exhaustion (check CPU/memory/connections)
   - External service degradation

See [PERFORMANCE.md](PERFORMANCE.md) for profiling instructions.

---

## Setup Issues

### "Cannot install dependencies"

**Causes and Solutions:**

| Cause | Solution |
|---|---|
| Wrong language/runtime version | Install correct version (see [DEVELOPMENT.md](DEVELOPMENT.md)) |
| Network/proxy issues | Configure proxy: `[proxy config command]` |
| Native dependency missing | Install system package: `[package install command]` |
| Lockfile conflict | Delete lockfile and reinstall: `[commands]` |

### "Database setup fails"

**Causes and Solutions:**

| Cause | Solution |
|---|---|
| Database server not installed | Install: `[install command]` |
| Database server not running | Start: `[start command]` |
| Insufficient privileges | Create user with permissions: `[SQL commands]` |
| Database already exists | Drop and recreate: `[commands]` |

### "Tests fail on fresh setup"

**Causes and Solutions:**

1. Test database not created: `[create test db command]`
2. Migrations not run on test db: `[test migration command]`
3. Required services not running: `[start services command]`
4. Environment variables not set: Copy `.env.example` to `.env.test` and configure
5. Port conflicts: Check that test ports are available

## Runtime Issues

### Application won't start

**Diagnostic steps:**

1. Check the startup log output: `[startup log command]`
2. Common causes:
   - Missing required environment variable → Set it (see [CONFIGURATION.md](CONFIGURATION.md))
   - Port already in use → `lsof -i :[port]` to find conflicting process
   - Database not reachable → Check connectivity and credentials
   - Invalid configuration → Review configuration against validation rules

### High CPU usage

1. Profile the application: `[CPU profiling command]`
2. Common causes:
   - Infinite loop or runaway recursion
   - Expensive computation without caching
   - Excessive logging
   - GC pressure from high allocation rate

### Connections exhausted

1. Check connection count: `[connection check command]`
2. Common causes:
   - Connection leak (connections not returned to pool)
   - Traffic spike exceeding pool size
   - Long-running queries holding connections
3. Immediate mitigation: Restart application to reset pool
4. Long-term: Fix leak or increase pool size

## Known Issues

| Issue | Impact | Workaround | Tracking |
|---|---|---|---|
| [Description of known issue] | [What it affects] | [How to work around it] | [Issue link] |
| [Description of known issue] | [What it affects] | [How to work around it] | [Issue link] |

## Getting Help

If this guide does not resolve your issue:

1. **Search existing issues:** [Issue tracker URL]
2. **Check logs:** Gather relevant log output before asking for help
3. **File a new issue:** Include:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Environment details (OS, version, configuration)
   - Relevant log output
4. **Contact the team:** [Channel / email]

---

## Related Documents

- [Development Guide](DEVELOPMENT.md) — Setup and development workflow
- [Configuration](CONFIGURATION.md) — All configuration options
- [Operations](OPERATIONS.md) — Operational runbooks
- [Performance](PERFORMANCE.md) — Performance profiling
- [API Reference](API.md) — API error codes
