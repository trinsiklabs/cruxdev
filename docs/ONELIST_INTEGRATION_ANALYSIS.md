# OneList Integration Analysis — CruxBot + CruxVibe Backend Storage

**Date:** 2026-03-28
**Status:** Research Complete
**Scope:** How OneList serves as the canonical data store for CruxBot and CruxVibe

---

## 1. OneList Architecture Understanding

### 1.1 What Is OneList?

OneList is an "augmented memory" platform built on Elixir/Phoenix with PostgreSQL and pgvector. It follows an "everything is an entry" philosophy where all user data is stored in a unified `entries` table with typed metadata, rather than creating separate tables for each domain concept. It is the canonical data store for the Onestream swarm -- if OneList does not have it, it does not exist.

**Production URL:** `https://key.onelist.my/api/v1`

### 1.2 Core Data Model

The data model is radically simple:

| Table | Purpose |
|-------|---------|
| `entries` | Central table. Every piece of user content. Typed via `entry_type`. |
| `representations` | Multiple content formats per entry (markdown, HTML, text, storage_path) |
| `representation_versions` | Version history with snapshot + diff hybrid |
| `entry_links` | Typed relationships between any two entries |
| `entry_tags` / `tags` | Free-form tagging (per user) |
| `memories` | Extracted facts with vector embeddings (pgvector 1536-dim) |
| `assets` | File attachments (local or S3/GCS/R2) |
| `agents` | Agent registry |
| `agent_tasks` | Agent task tracking |

**Key principle:** New tables require explicit approval. Everything else goes in `entries` with appropriate `entry_type` and `metadata` JSONB.

### 1.3 Entry Types (Current)

| Type | Purpose | Mutability |
|------|---------|------------|
| `note` | User-created text | Mutable |
| `memory` | Captured/imported content | Immutable |
| `article` | Long-form published content | Mutable |
| `task` | GTD actionable items | Mutable |
| `project` | Collections of related work | Mutable |
| `decision` | Recorded decisions | Mutable |
| `config` | System/user configuration | Mutable |
| `person` | Human or AI agent identity | Mutable |
| `conversation` | Chat/dialogue sessions | Mutable |
| `log_event` | Audit/system events | Immutable |
| `job` | Background job tracking | Mutable |
| `page` | Static web pages (CMS) | Mutable |
| `doc` | Documentation | Mutable |
| `group` | Manual entry collections | Mutable |
| `event` | Calendar events | Mutable |

### 1.4 Schema Design Philosophy

Every entry has:
- `id` (UUID), `public_id` (nanoid for URLs), `user_id` (owner)
- `entry_type` (string, defines behavior)
- `source_type` (how created: `manual`, `api`, `web_clip`, `agent`)
- `title`, `metadata` (JSONB), `public` (visibility flag)
- `version` (optimistic locking)
- `value_tier` (`temporary`, `standard`, `valuable`, `archive`)
- `embedding` (pgvector 1536-dim for semantic search)
- `source_agent_id` (agent attribution)

Relationships between entries use `entry_links` with typed links (`blocks`, `relates_to`, `parent_of`, `group_member`, `depends_on`, etc.).

### 1.5 How It Handles Users/Accounts

- `users` table managed by `phx.gen.auth` (email/password)
- Social login: GitHub, Google, Apple OAuth
- API keys: Bearer token auth (`ol_key_*` prefix)
- Each user has their own entries, tags, API keys -- no shared entries in MVP
- Agent attribution via `X-Agent-Id`, `X-Agent-Version`, `X-Agent-Instance-Id` headers

### 1.6 Multi-Tenancy

OneList uses **namespace isolation within a single database**, not separate databases per customer. The `user_id` foreign key on entries provides tenant scoping. For the Onestream swarm (KeyVibe), each customer gets tenant-scoped data isolation (PLAN-3158).

### 1.7 API Surface

Full REST API at `/api/v1/` with:
- **Entry CRUD:** `GET/POST/PUT/DELETE /entries`, `/entries/:id`
- **Bulk ops:** `POST /entries/bulk`, `PATCH /entries/bulk`
- **Tags:** per-entry tag management
- **Links:** typed entry-to-entry relationships
- **Search:** `POST /search` (hybrid semantic 70% + keyword 30%)
- **Memories:** vector-backed extracted facts
- **Chat:** Trio Chat (multi-agent messaging), Chat Stream (agent logging)
- **River:** AI assistant with GTD, tasks, briefings, conversations
- **Bots:** Simulation, telemetry, anomaly detection, quarantine, graduation
- **Trusted Memory:** Blockchain-inspired chain integrity with audit trail
- **Assets:** Upload, download, thumbnails, mirror status
- **Embeddings:** Config, create, retrieve per entry

---

## 2. CruxVibe Integration Plan

### 2.1 Core Concept

CruxVibe creates a OneList account (user) per CruxVibe user in the background. The user's email address is the shared identity. This OneList account becomes the universal storage layer for all their data -- blog posts, recipe configs, tenant resources, project state.

### 2.2 Account Creation Flow

```
CruxVibe User Registration
    |
    v
Create OneList user via API (email + generated password)
    |
    v
Generate OneList API key for this user
    |
    v
Store API key in CruxVibe's local user record (encrypted)
    |
    v
All subsequent CruxVibe operations use this API key
```

CruxVibe's Ash resources become a **thin local layer** that delegates persistence to OneList. The local PostgreSQL can still hold tenant config and routing tables, but all user content lives in OneList.

### 2.3 Blog Posts (BP004)

Current BP004 defines a `posts` table with custom schema. With OneList, blog posts become OneList entries:

| BP004 Field | OneList Mapping |
|-------------|-----------------|
| `title` | `entry.title` |
| `slug` | `entry.metadata.slug` |
| `body_markdown` | `representation` with `type: "markdown"` |
| `body_html` | `representation` with `type: "html"` or `"public_html"` |
| `summary` | `entry.metadata.summary` |
| `tldr` | `entry.metadata.tldr` |
| `date` | `entry.content_created_at` |
| `tags` | OneList tags via `entry_tags` |
| `author_name` | `entry.metadata.author_name` |
| `status` | `entry.metadata.status` (draft/scheduled/published/archived) |
| `published_at` | `entry.metadata.published_at` |
| `x_post_queued` | `entry.metadata.x_post_queued` |
| `meta_description` | `entry.metadata.meta_description` |
| `og_image_url` | `entry.metadata.og_image_url` |
| `canonical_url` | `entry.metadata.canonical_url` |
| `ground_truth` | `entry.metadata.ground_truth` |

**Entry type:** `article` (long-form published content, already in OneList's type registry)
**Source type:** `api` (created by CruxVibe platform)

**Post versions:** OneList's `representation_versions` table already provides version history with snapshot + diff hybrid. No need for a separate `post_versions` table.

**Repurposed content:** Each platform variant becomes its own OneList entry linked via `entry_links` with `link_type: "derived_from"`. The derived entry gets `metadata.platform` = `"x_thread"`, `"x_single"`, etc.

### 2.4 Recipes (Installations, Features, Issues, Votes)

| Recipe Concept | OneList Mapping |
|----------------|-----------------|
| Recipe definition | Entry `type: "config"`, `metadata.config_type: "recipe_definition"` |
| Recipe installation (per tenant) | Entry `type: "config"`, `metadata.config_type: "recipe_installation"` |
| Feature request | Entry `type: "task"`, `metadata.bucket: "feature_request"`, `metadata.recipe_slug: "membership"` |
| Vote on feature | Entry link `type: "supports"` from voter's person entry to feature entry, `metadata.vote_weight: 1` |
| Issue/bug report | Entry `type: "task"`, `metadata.bucket: "issue"`, `metadata.recipe_slug: "membership"` |
| Recipe version | Entry `type: "log_event"`, `metadata.log_type: "release"`, `metadata.recipe_slug: "membership"` |
| Recipe settings (per tenant) | Entry `type: "config"`, `metadata.config_type: "recipe_settings"` |

### 2.5 User Accounts

CruxVibe user registration triggers:

1. Create CruxVibe local user record (tenant routing, session management)
2. Create OneList user via API (email as identifier)
3. Generate and store OneList API key
4. Create a `person` entry in OneList for the user

All user data queries go through OneList API. Local CruxVibe DB only holds:
- Session tokens
- Tenant routing config (which recipes enabled)
- OneList API key reference (encrypted)

### 2.6 Tenant Data

Each tenant's resources are OneList entries scoped by `user_id`. Tenant isolation is automatic because OneList enforces user-scoped access via API key authentication.

| Tenant Resource | OneList Entry Type | Metadata Keys |
|----------------|-------------------|---------------|
| Tenant config | `config` | `config_type: "tenant"`, `subdomain`, `plan` |
| Custom domain | `config` | `config_type: "custom_domain"`, `domain`, `verified` |
| Theme settings | `config` | `config_type: "theme"`, `colors`, `fonts`, `layout` |
| Installed recipes | `config` | `config_type: "recipe_installation"`, `recipe_slug`, `version` |

### 2.7 Payments/Subscriptions (Stripe)

Stripe data linkage in OneList:

| Payment Concept | OneList Mapping |
|----------------|-----------------|
| Stripe customer ID | `person` entry `metadata.stripe_customer_id` |
| Subscription record | Entry `type: "config"`, `metadata.config_type: "subscription"`, `metadata.stripe_subscription_id` |
| Invoice/payment event | Entry `type: "log_event"`, `metadata.log_type: "billing"` |
| Usage metering | Entry `type: "log_event"`, `metadata.log_type: "usage"` |

Stripe webhook events are stored as immutable `log_event` entries, providing a full billing audit trail.

---

## 3. CruxBot Integration Plan

### 3.1 Core Concept

CruxBot gets a dedicated OneList user account (e.g., `cruxbot@trinsiklabs.com`). All CruxBot persistent state moves from local `.cruxbot/` files to OneList entries. The `source_agent_id` on all entries is `"cruxbot"`.

### 3.2 Bot Memories (Observations, Patterns, History)

| CruxBot Data | Current Storage | OneList Mapping |
|-------------|-----------------|-----------------|
| Observations | `.cruxbot/observations.jsonl` | `memories` table with `memory_type: "observation"`, `source_agent_id: "cruxbot"` |
| Learned patterns | In-memory / scattered | Entry `type: "knowledge"`, `metadata.knowledge_type: "pattern"` |
| Convergence history | `.cruxbot/convergence_log.jsonl` | Entry `type: "log_event"`, `metadata.log_type: "convergence"` |
| Diagnoses | `.cruxbot/diagnoses.jsonl` | Entry `type: "knowledge"`, `metadata.knowledge_type: "diagnosis"` |

OneList's `memories` table is ideal for observations because it supports:
- Vector embeddings for semantic similarity (find related observations)
- Memory chains for tamper-evident audit trails
- Supersession tracking (`supersedes_id`) for evolving understanding
- Agent attribution (`source_agent_id: "cruxbot"`)

### 3.3 Script Library

| Script Data | Current Storage | OneList Mapping |
|------------|-----------------|-----------------|
| Script registry | `scripts/registry.yaml` | Entry `type: "config"`, `metadata.config_type: "script_registry"` |
| Script metadata | Per-script YAML | Entry `type: "knowledge"`, `metadata.knowledge_type: "script"`, `metadata.script_name`, `metadata.status`, `metadata.run_count` |
| Script source code | `.cruxbot/scripts/*.go` | `representation` with `type: "text"` on the script entry, or `asset` attachment |
| Script test results | In-memory | Entry `type: "log_event"`, `metadata.log_type: "script_test"` |

### 3.4 Budget Tracking

| Budget Data | Current Storage | OneList Mapping |
|------------|-----------------|-----------------|
| Daily spend records | `.cruxbot/budget/YYYY-MM-DD.jsonl` | Entry `type: "log_event"`, `metadata.log_type: "budget"`, `metadata.cost_cents`, `metadata.model` |
| Budget config | Hardcoded / env | Entry `type: "config"`, `metadata.config_type: "budget_config"` |

**Important:** The hot-path in-memory `AtomicU64` counter stays in CruxBot's process memory for performance. OneList is the durable persistence layer, loaded on startup and appended to asynchronously. The pattern from BP019 remains valid -- just swap the local JSONL file for a OneList API call.

### 3.5 Content Pipeline

| Content Data | Current Storage | OneList Mapping |
|-------------|-----------------|-----------------|
| Generated blog posts | Markdown files in project repos | Entry `type: "article"`, `source_type: "agent"` |
| X post drafts | `.cruxdev/growth/x_queue.jsonl` | Entry `type: "note"`, `metadata.platform: "x"`, `metadata.status: "queued"` |
| Content event log | `.cruxbot/content_log.jsonl` | Entry `type: "log_event"`, `metadata.log_type: "content_event"` |
| Content decisions | In-memory | Entry `type: "decision"`, `metadata.event_type`, `metadata.decision` |

Blog posts generated by CruxBot become OneList `article` entries with `source_agent_id: "cruxbot"`. The same entries that CruxVibe reads for rendering the blog.

### 3.6 Plan Queue

| Plan Data | Current Storage | OneList Mapping |
|----------|-----------------|-----------------|
| Pending plans | `.cruxbot/plans/` directory | Entry `type: "agent_plan"`, `metadata.status: "pending"` |
| Plan review status | File naming conventions | `metadata.status` (`pending`, `approved`, `rejected`, `executing`, `completed`) |
| Plan execution log | `.cruxbot/execution_log.jsonl` | Entry `type: "log_event"`, `metadata.log_type: "plan_execution"` |
| Plan dependencies | In plan files | Entry links `type: "depends_on"` between plan entries |

---

## 4. Blog Engine Recipe Revision

### 4.1 What Changes

BP004 currently defines three PostgreSQL tables (`posts`, `post_versions`, `repurposed_content`). With OneList, these are replaced by:

1. **`posts` table** becomes OneList entries with `entry_type: "article"`
2. **`post_versions` table** is eliminated -- OneList's `representation_versions` provides this natively
3. **`repurposed_content` table** becomes linked OneList entries with `link_type: "derived_from"`

### 4.2 What Stays The Same

- Phase 2 (Markdown Preprocessor) -- renders markdown to HTML, still needed
- Phase 3 (Astro Integration) -- reads from API, generates static pages. The API endpoint changes from CruxVibe's local DB to OneList's API
- Phase 4 (BIP Integration) -- CruxBot writes articles to OneList instead of local DB
- Phase 5 (Blog LiveView) -- management UI, reads/writes via OneList API

### 4.3 New Data Flow

```
CruxBot convergence → detect content event
    |
    v
LLM generates blog post
    |
    v
POST /api/v1/entries (OneList)
    entry_type: "article"
    source_type: "agent"
    source_agent_id: "cruxbot"
    representations: [
        { type: "markdown", content: "..." },
        { type: "html", content: "..." }
    ]
    metadata: { slug, status: "draft", tags, author_name, summary, ... }
    |
    v
Human reviews in CruxVibe LiveView → publishes
    |
    v
PATCH /api/v1/entries/:id (OneList)
    metadata.status: "published"
    metadata.published_at: now()
    |
    v
Astro build triggered → GET /api/v1/entries?entry_type=article&metadata.status=published
    |
    v
Static site regenerated and deployed
```

### 4.4 API Endpoints for Blog

CruxVibe's blog API controller becomes a thin proxy to OneList:

```elixir
# GET /api/v1/blog
# Translates to: GET OneList /api/v1/entries?entry_type=article&metadata filters

# GET /api/v1/blog/:slug
# Translates to: GET OneList /api/v1/entries?entry_type=article&metadata.slug=:slug

# GET /api/v1/blog/tags
# Translates to: GET OneList /api/v1/entries/:entry_id/tags (aggregated)
```

---

## 5. Key Design Decisions

### 5.1 OneList as Shared Identity Layer

A user's email creates ONE OneList account. That account is the storage layer for:
- CruxVibe tenant data (blog posts, recipe configs, themes)
- CruxBot project data (observations, scripts, plans, content)
- Future: any other Crux ecosystem product

This means CruxBot's generated blog posts and CruxVibe's human-authored blog posts live in the same OneList user account, queryable together. A user who is also a CruxVibe tenant has their bot-generated content and manual content unified.

### 5.2 Local DB vs. OneList Split

**OneList owns all content data.** CruxVibe's local PostgreSQL keeps only:
- Session management (ephemeral)
- Tenant routing (which subdomain maps to which OneList user)
- OneList API key storage (encrypted reference)
- Cache tables (optional, for performance)

CruxBot's local files keep only:
- In-memory counters (budget AtomicU64)
- Temporary sandbox files
- The compiled Go scripts themselves (binaries, not metadata)

### 5.3 Migration Path

**CruxVibe:** Currently has no production data. Clean start -- implement OneList-backed Ash resources from the beginning. No migration needed.

**CruxBot:** Currently uses local `.cruxbot/` JSONL files. Migration:
1. Add OneList client to CruxBot (Rust HTTP client)
2. Implement dual-write: write to both local files and OneList
3. Backfill existing JSONL data to OneList (one-time script)
4. Remove local file writes after verification
5. Keep local files as read-only fallback (OneList unavailable)

### 5.4 Resilience

CruxBot must function even when OneList is unreachable (network issues, OneList downtime). Strategy:
- **Write:** Buffer writes locally, sync to OneList when available (write-ahead log)
- **Read:** Cache recent entries locally, degrade gracefully to cached data
- **Budget:** In-memory counter is always authoritative; OneList is durable backup

---

## 6. Entry Type Summary

New `entry_type` values needed (or reuse of existing ones) for Crux ecosystem:

| Entry Type | Used By | Purpose |
|-----------|---------|---------|
| `article` | CruxVibe, CruxBot | Blog posts (exists in OneList) |
| `task` | CruxVibe | Feature requests, issues (exists) |
| `config` | CruxVibe, CruxBot | Tenant config, recipe config, budget config, script registry (exists) |
| `knowledge` | CruxBot | Patterns, diagnoses, scripts (exists) |
| `decision` | CruxBot | Content decisions (exists) |
| `log_event` | CruxVibe, CruxBot | Billing, convergence, plan execution, budget (exists) |
| `person` | CruxVibe | User identity, Stripe linkage (exists) |
| `agent_plan` | CruxBot | Pending/executing plans (exists) |
| `note` | CruxBot | X post drafts (exists) |

No new entry types required. OneList's existing type registry covers all Crux ecosystem needs.

---

## 7. API Key Architecture

```
CruxVibe Platform
    |
    |-- OneList API Key (platform-level, for CruxVibe backend operations)
    |
    |-- Per-User OneList API Keys
        |-- User A's key → scoped to User A's entries
        |-- User B's key → scoped to User B's entries

CruxBot
    |
    |-- OneList API Key (cruxbot@trinsiklabs.com account)
    |-- Writes with X-Agent-Id: cruxbot header
```

---

## 8. References

- OneList Unified Schema: `/Users/user/arch_swarm/onelist/docs/unified_schema.md`
- OneList API Reference: `/Users/user/arch_swarm/onelist/docs/API.md`
- OneList Architecture: `/Users/user/arch_swarm/onelist/docs/ARCHITECTURE.md`
- OneList Vision: `/Users/user/arch_swarm/onelist/docs/VISION.md`
- OneList Product Plan: `/Users/user/arch_swarm/onelist/plans/product-vision.md`
- OneList Schema (50 tables): `/Users/user/arch_swarm/onelist/docs/SCHEMA.md`
- CruxVibe BP004 Blog Recipe: `/Users/user/personal/cruxvibe/build_plans/BUILD_PLAN_004_BLOG_RECIPE.md`
- CruxBot BP019 Persistent Budget: `/Users/user/personal/cruxbot/build_plans/BUILD_PLAN_019_PERSISTENT_BUDGET.md`
- CruxBot BP022 Script Library: `/Users/user/personal/cruxbot/build_plans/BUILD_PLAN_022_SCRIPT_LIBRARY.md`
- CruxBot BP007 Content Pipeline: `/Users/user/personal/cruxbot/build_plans/BUILD_PLAN_007_CONTENT_PIPELINE.md`
- CruxBot BP009 Self-Evolution: `/Users/user/personal/cruxbot/build_plans/BUILD_PLAN_009_SELF_EVOLUTION_ENGINE.md`
