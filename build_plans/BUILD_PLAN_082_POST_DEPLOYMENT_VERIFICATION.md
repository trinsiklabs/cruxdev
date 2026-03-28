# BUILD_PLAN_082: Post-Deployment Verification Patterns

**Status:** CONVERGED
**Priority:** High (triggered by production 500 error on UAT deployment)
**Triggered by:** Westlake Select Phoenix app returning 500 after Fly.io deployment

## Context

A Phoenix app deployed to Fly.io returned a 500 error post-deployment. No automated verification caught it. Every deployment — website or webapp, UAT or production — needs automated post-deployment health checks that verify the deployment succeeded and the application is fully operational.

This is a new patterns doc that must be integrated into the convergence process: when a project deploys, post-deployment verification runs automatically.

## Phase 1: Deep Research

- [ ] 1.1 Research post-deployment verification best practices (smoke tests, health checks, canary deployments)
- [ ] 1.2 Research by platform: Fly.io (health checks, release commands), Vercel (preview deployments), AWS (ALB health), Cloudflare Pages, bare metal
- [ ] 1.3 Research by framework: Phoenix (health endpoint, Ecto migration checks), Next.js (ISR warmup), Rails (boot check), Django
- [ ] 1.4 Research rollback strategies: automatic rollback on health check failure
- [ ] 1.5 Research monitoring integration: uptime checks, error rate spike detection, log analysis

## Phase 2: Write POST_DEPLOYMENT_PATTERNS.md

- [ ] 2.1 Health endpoint patterns (HTTP 200 with JSON status: DB connected, migrations current, cache warm)
- [ ] 2.2 Smoke test suite (critical paths verified: homepage, login, key API endpoints)
- [ ] 2.3 Database verification (migrations applied, no pending, seed data present)
- [ ] 2.4 Asset verification (CSS/JS loaded, no 404s on static assets, images render)
- [ ] 2.5 SSL/TLS verification (cert valid, HSTS active, no mixed content)
- [ ] 2.6 Performance baseline (response time < threshold, no memory leaks on startup)
- [ ] 2.7 Rollback triggers (what conditions trigger automatic rollback)
- [ ] 2.8 Platform-specific patterns (Fly.io release_command, Vercel checks, Docker health)
- [ ] 2.9 Notification patterns (Slack/email on deploy success/failure)
- [ ] 2.10 Anti-patterns (deploying without health checks, ignoring migration errors, manual-only verification)

## Phase 3: Engine Integration

- [ ] 3.1 Add POST_DEPLOYMENT_DIMENSIONS to router.rs
- [ ] 3.2 Detect deployable projects (has Dockerfile, fly.toml, vercel.json, deploy.sh)
- [ ] 3.3 Add post-deployment check to website convergence phase
- [ ] 3.4 Extend check_seo_health to verify post-deployment health endpoint
- [ ] 3.5 New MCP tool: `verify_deployment(url, checks)` — runs health checks against a deployed URL

## Phase 4: Content Generation

- [ ] 4.1 Blog post: "Post-Deployment Verification — Never Ship a 500 Again"
- [ ] 4.2 X post announcing the pattern
- [ ] 4.3 Publish via BIP pipeline

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
