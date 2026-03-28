# BUILD_PLAN_090: Enterprise Readiness Foundation

**Status:** NOT STARTED
**Priority:** Future (but documented now for roadmap clarity)
**Competitive gap:** Claude Code (HIPAA, SOC2, SSO), Codex (custom CA, sandbox policies)

## Context

Enterprise adoption requires compliance certifications, access controls, and audit capabilities. Not immediate priority but defines what the enterprise sales conversation will need. Documenting now so the architecture supports it later.

## Phase 1: Audit Trail (Foundation for Compliance)

- [ ] 1.1 Every MCP tool call logged: who, what, when, result, duration
- [ ] 1.2 Immutable append-only log (JSONL with checksums)
- [ ] 1.3 Export: JSONL, CSV, PDF report
- [ ] 1.4 Retention policy configurable (default 90 days)

## Phase 2: Access Controls

- [ ] 2.1 Tool-level permissions (which tools a session can use)
- [ ] 2.2 Project-level access (which projects a session can converge)
- [ ] 2.3 Read-only audit mode (observe convergence without write access)
- [ ] 2.4 API key scoping (per-project, per-tool)

## Phase 3: Compliance Documentation

- [ ] 3.1 Data flow diagram (what data CruxDev processes, where it goes)
- [ ] 3.2 Security model doc (MCP transport, no network by default, local-only)
- [ ] 3.3 SOC2 Type II readiness checklist
- [ ] 3.4 HIPAA considerations (if processing PHI in convergence)

## Phase 4: Multi-Tenant Support

- [ ] 4.1 Isolated convergence state per team/org
- [ ] 4.2 Shared patterns docs with org-specific overrides
- [ ] 4.3 Usage tracking and billing integration points

## Not Now, But Designed For

This plan documents the ARCHITECTURE needed, not the implementation timeline. The Rust binary, MCP protocol, and convergence state machine should not make decisions that prevent enterprise features later.
