# BUILD_PLAN_111: Marketing Plan Generator

**Status:** NOT STARTED
**Priority:** High (second-highest impact capability for creator vertical)
**Created:** 2026-03-28
**Triggered by:** Careiance/Zephyr Oakhaven capabilities wishlist — currently building marketing plans manually in Google Docs

## Problem

Creators and coaches spend 4-8 hours per product launch building marketing plans in Google Docs. These plans are static, disconnected from execution tools, and go stale within a week. No tool generates a complete marketing plan (audience + messaging + channels + calendar + KPIs) from a product description. Creators either skip planning entirely or plan once and abandon.

## Solution

A new `generate_marketing_plan` MCP tool that takes a product description and generates a structured, executable marketing plan. The plan is a first-class CruxDev artifact — it can be converged (iterated until quality thresholds are met), tracked, and connected to the content pipeline for execution.

## Architecture

```
Product Description (name, type, audience, channels)
    │
    ▼
Marketing Plan Generator (LLM + templates)
    │
    ▼
marketing_plan.json (structured plan file)
    │
    ├──▶ Audience segments with pain points and messaging hooks
    ├──▶ Messaging matrix (core message + per-channel adaptations)
    ├──▶ Channel strategy (which platforms, posting frequency, content types)
    ├──▶ Editorial calendar (week-by-week with platform assignments)
    ├──▶ Timeline with milestones (pre-launch, launch day, post-launch)
    └──▶ KPIs with targets and measurement methods
```

## Phase 1: Marketing Plan Schema and Template

### 1a. Define MarketingPlan schema
- File: `rust/src/growth/marketing_plan.rs`
- Struct: `MarketingPlan` with fields: product, audience_segments, messaging, channels, calendar, timeline, kpis
- Struct: `AudienceSegment` { name, demographics, pain_points, messaging_hooks, channels }
- Struct: `MessagingMatrix` { core_message, tagline, per_channel: HashMap<Channel, String> }
- Struct: `ChannelStrategy` { channel, frequency, content_types, best_times, audience_overlap }
- Struct: `CalendarEntry` { week, day, channel, content_type, topic, status }
- Struct: `Milestone` { name, date, dependencies, success_criteria }
- Struct: `KPI` { metric, target, measurement_method, current_value }
- All serializable to JSON, all with atomic write support
- [ ] Schema structs defined and documented
- [ ] Serde serialization/deserialization tests
- [ ] Atomic write/read for marketing_plan.json

### 1b. Marketing plan templates by product type
- File: `rust/src/growth/marketing_templates.rs`
- Template: `book_launch` — 12-week arc (4 pre-launch, 1 launch, 7 post-launch)
- Template: `course_launch` — 8-week arc (3 pre-launch, 1 launch, 4 post-launch)
- Template: `coaching_program` — ongoing with quarterly campaigns
- Template: `membership` — evergreen funnel with seasonal pushes
- Template: `general` — flexible 4-12 week framework
- Each template pre-populates: typical audience segments, recommended channels, calendar skeleton, standard KPIs
- [ ] All 5 templates defined
- [ ] Template selection logic (product_type -> template)
- [ ] Tests for each template's structural validity

### 1c. Plan validation
- File: `rust/src/growth/marketing_validator.rs`
- Validate: all calendar entries have assigned channels
- Validate: every audience segment has at least one messaging hook
- Validate: KPIs are measurable (not vague)
- Validate: no calendar gaps longer than 7 days during active campaign
- Validate: channel strategy covers all channels mentioned in calendar
- [ ] Validator function with structured error reporting
- [ ] Tests for each validation rule
- [ ] Tests for valid plans passing validation

## Phase 2: MCP Tool Integration

### 2a. generate_marketing_plan tool
- File: `rust/src/server.rs` (add tool)
- Params: product_name, product_type, target_audience (optional), channels (optional), duration_weeks (optional), voice_guide (optional), project_dir (optional)
- Flow:
  1. Select template based on product_type
  2. Fill template with product-specific details (LLM call via CruxDev dispatch)
  3. Validate generated plan
  4. Write to `.cruxdev/growth/marketing_plans/{product_slug}.json`
  5. Return plan summary + file path
- [ ] Tool parameter struct defined
- [ ] Tool handler implemented
- [ ] Integration test: generate plan for each product type
- [ ] Error handling for invalid product types

### 2b. marketing_plan_status tool
- File: `rust/src/server.rs` (add tool)
- Shows: active marketing plans, upcoming calendar entries, KPI progress, overdue items
- [ ] Tool parameter struct defined
- [ ] Tool handler implemented
- [ ] Test: status with no plans returns empty
- [ ] Test: status with active plan returns correct data

## Phase 3: Convergence Integration

### 3a. Marketing plan as convergence target
- The convergence engine can iterate on a marketing plan:
  - Round 1: Generate initial plan from template + product description
  - Round 2: Audit plan for gaps (missing audience segments, thin messaging, calendar gaps)
  - Round 3: Fix gaps and re-audit
  - Converge when: validation passes + no audit findings
- File: `rust/src/engine/convergence.rs` (extend)
- [ ] Marketing plan convergence task type
- [ ] Audit dimension: plan_completeness (all sections filled)
- [ ] Audit dimension: plan_coherence (messaging aligns with audience segments)
- [ ] Audit dimension: plan_actionability (calendar entries are specific enough to execute)
- [ ] Two consecutive clean passes = converged

### 3b. Connect to content pipeline
- When a marketing plan is active, the content pipeline's `prioritize_work` tool should surface calendar entries that are due
- The `generate_content` tool should accept a calendar entry reference and generate content matching the plan's messaging and voice
- [ ] prioritize_work returns upcoming marketing calendar entries
- [ ] generate_content accepts marketing_plan_ref parameter
- [ ] Generated content matches plan's messaging matrix

## Phase 4: Tests

- [ ] Unit tests for all schema structs (serialization roundtrip)
- [ ] Unit tests for all templates (structural validity)
- [ ] Unit tests for validator (all rules)
- [ ] Integration tests for generate_marketing_plan tool
- [ ] Integration tests for marketing_plan_status tool
- [ ] Convergence tests for marketing plan iteration
- [ ] End-to-end: product description → converged marketing plan → content generation from calendar entry

## File Locations

| File | Purpose |
|------|---------|
| `rust/src/growth/marketing_plan.rs` | Schema structs and read/write |
| `rust/src/growth/marketing_templates.rs` | Product-type templates |
| `rust/src/growth/marketing_validator.rs` | Plan validation rules |
| `rust/src/growth/mod.rs` | Module registration |
| `rust/src/server.rs` | MCP tool handlers |
| `rust/tests/marketing_plan_tests.rs` | All tests |

## Success Criteria

1. A creator can describe a product and get a complete, validated marketing plan in under 60 seconds.
2. The plan includes all 6 sections: audience, messaging, channels, calendar, timeline, KPIs.
3. The plan passes validation with zero findings on first generation for common product types.
4. The convergence engine can iterate on the plan and fix gaps autonomously.
5. Generated content from the plan's calendar entries maintains voice consistency.
