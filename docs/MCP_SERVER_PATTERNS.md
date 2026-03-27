# MCP Server Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 60+ sources including MCP specification (modelcontextprotocol.io), OWASP MCP Top 10, rmcp SDK, Anthropic docs, Block engineering, Pragmatic Engineer, practitioner analysis
**Last updated:** 2026-03-27

## Core Principle

**An MCP server without a corresponding skill is a toolbox without a carpenter.** Tools provide capability. Skills provide intent. Every MCP server should ship with a skill that teaches agents when and how to use its tools. See AI_SKILLS_PATTERNS.md.

---

## 1. Tool Design

### Naming
- Pattern: `domain_noun_verb` — `github_issue_create`, `convergence_submit_result`
- Convention: **snake_case** (90%+ of tools, best tokenization)
- Regex: `^[a-zA-Z0-9_-]{1,64}$`
- Related operations cluster alphabetically when named consistently

### Descriptions Are Prompts
- Tool descriptions are prompts for the LLM, not human API docs
- Explain **when** to use the tool, not just what it does
- Include parameter-by-parameter documentation
- Keep under ~2KB (truncation risk). Critical details first.
- Neon improved tool selection from 60% to 100% accuracy through description optimization alone — zero code changes

### Parameter Schemas
- Use JSON Schema with `type`, `format`, `pattern`, `maxLength`, `minimum`, `maximum`
- **Prefer top-level primitives** over nested objects — LLMs struggle with deep structures
- 5 tools with 6 params each > 1 tool with 30 params
- Mark required vs optional. Provide defaults in descriptions.
- Use enums for constrained types

### Return Values
- Return structured JSON with enough context for the LLM to decide next steps
- `isError: true` for application failures (not protocol errors) — these go back into the LLM context for recovery
- Error messages: what happened, why, what's the valid format

---

## 2. Server Architecture

### Initialization
- 3-step handshake: client `initialize` → server responds with capabilities → client `notifications/initialized`
- No requests before handshake completes
- `ServerInfo.instructions` — the server's "system prompt" for the LLM. Concise bootstrap directive.

### State Management
- Atomic writes (write-then-rename) for all state files
- WAL (write-ahead log) for crash recovery in stateful workflows
- File-based state (JSON, SQLite) for local/embedded use
- External databases for production scale

### Design Philosophy
- MCP is a **UI for AI agents**, not a REST API wrapper
- Do NOT convert REST endpoints 1:1 into MCP tools
- Design composite, goal-oriented tools — do orchestration in your code, not the LLM's context window
- Start with 3-4 focused tools. Add iteratively.

---

## 3. Transport

| Transport | Latency | Use Case | Status |
|-----------|---------|----------|--------|
| **stdio** | Microseconds | Local tools, CLI integrations | Primary |
| **Streamable HTTP** | Network | Remote/cloud, multi-client, browser | Primary |
| **SSE** | Network | Legacy | Deprecated (2025-03-26) |

### stdio Rules
- NEVER write to stdout except JSON-RPC messages — debug logging corrupts the protocol. Use stderr.
- Shutdown: client closes stdin → wait for exit → SIGTERM if needed

### Streamable HTTP
- Single HTTP endpoint (POST + GET), optional SSE for streaming
- OAuth 2.1 mandatory for HTTP transports
- Can operate fully stateless behind load balancers

### Recommendation
Expose both stdio and Streamable HTTP. Server logic is identical; only transport differs.

---

## 4. Security

### The Landscape
- 66% of 1,808 scanned MCP servers had at least one security finding (2026 audit)
- 43% shell/command injection, 20% tooling infrastructure, 13% auth bypass, 10% path traversal
- OWASP ranks prompt injection as #1 LLM vulnerability

### Input Validation (Non-Negotiable)
- Validate ALL inputs with JSON Schema constraints
- Sanitize: strip control characters, validate types/formats, reject oversized payloads
- **Allowlists** for structured fields, not denylists
- Prevent directory traversal: validate and canonicalize paths
- Apply size limits to prevent memory exhaustion

### Credential Management
- 53% of MCP servers use insecure long-lived static secrets
- **Never** hardcode secrets in config files or source code
- Use environment variable references: `api_key_env = "TYPEFULLY_API_KEY"` (name, not value)
- OAuth 2.1 for HTTP transports. Short-lived tokens.

### Prompt Injection Defense
- Tool poisoning: attackers embed malicious instructions in tool descriptions/metadata
- Treat all tool metadata as untrusted
- Validate parameter values don't contain hidden instructions
- Defense in depth: input validation + least privilege + monitoring

### OWASP MCP Top 10
1. Token mismanagement / secret exposure
2. Excessive permissions
3. Tool poisoning
4. Context over-sharing between sessions
5. Command injection via unsanitized inputs

---

## 5. Testing

### Unit Testing (In-Memory)
- Pass server instance directly to a client in-process — no subprocess needed
- Test: tool discovery, parameter validation, error handling, response format, idempotency

### E2E Testing (Subprocess)
- Use client SDK to spawn the binary (stdio transport) and exercise full protocol
- Rust: `rmcp` with `transport-child-process` feature
- Must test the actual handshake: initialize → tools/list → tool/call

### Test Coverage Areas
- [ ] `listTools()` returns expected tools with correct schemas
- [ ] Missing required params → structured error
- [ ] Wrong types → structured error
- [ ] External service failures → graceful degradation
- [ ] Same inputs → same outputs (idempotency)
- [ ] Tool descriptions match actual behavior (skill accuracy)

---

## 6. Performance

### Key Facts
- Average unoptimized server: ~12 req/s. Properly optimized: 1,000+ req/s
- **Connection pooling is 80% of the performance win**
- Rust: sub-millisecond latency, lowest memory

### The "Too Many Tools" Problem
- GitHub's MCP server: 42,000 tokens just for tool definitions
- Client limits: Cursor caps at 40 tools, Claude Desktop at ~100
- Build focused, domain-specific servers (3-10 tools each)
- CruxDev at 52 tools is at the upper edge — monitor for client truncation

### Long-Running Tasks
- MCP Task primitive (SEP-1686): create task → get taskId → poll → retrieve
- Use `progressToken` for real-time status during long operations
- Break monolithic tasks into logical chunks

---

## 7. Error Handling

### Protocol Errors (JSON-RPC)
- `-32700` Parse error, `-32601` Method not found, `-32602` Invalid params, `-32603` Internal

### Application Errors (Tool Level)
- Return as successful JSON-RPC responses with `isError: true`
- Do NOT return tool failures as protocol errors
- Messages must be specific and actionable: "Invalid date format. Use YYYY-MM-DD."

### Idempotency
- Make all tools idempotent — agents may retry or parallelize
- Accept client-generated request IDs for deduplication
- Return deterministic results for same inputs

---

## 8. Documentation

### Server Instructions
- `instructions` field in ServerInfo: the server's system prompt for the LLM
- Explain purpose, bootstrap sequence, tool calling patterns, security rules
- Critical information first (may be truncated)

### Tool Descriptions
- Every description is a prompt — optimize for LLM tool selection
- Include: what, when, parameters, expected output, edge cases
- Iterate using evaluation: measure selection accuracy, refine

### The Skill Requirement
- **Every MCP server must ship with a corresponding skill file**
- The skill encodes the workflow: which tools to call, in what order, with what parameters
- Without the skill, agents treat tools as independent functions instead of a coordinated workflow
- When the server changes, the skill must be re-audited against the actual tool list

---

## 9. Versioning

### Protocol
- Date-based: `YYYY-MM-DD` (e.g., `2025-11-25`)
- Negotiated during initialization

### Server
- Semantic versioning: MAJOR.MINOR.PATCH
- MAJOR: incompatible tool changes. MINOR: new tools. PATCH: bug fixes.

### Tool Annotations (Spec 2025-03-26+)
- `readOnlyHint`, `destructiveHint`, `idempotentHint`, `openWorldHint`
- Hints, not guarantees — clients treat as untrusted

### Backward Compatibility
- Never remove tools without a major version bump
- Add optional parameters rather than changing required ones

---

## 10. Skill-Server Synchronization (Critical)

When an MCP server changes, corresponding skills become stale immediately:

1. **Tool renamed** → every skill referencing old name breaks silently
2. **Parameter changed** → skill sends wrong params, tool returns error or wrong result
3. **New tool added** → agents don't know to use it until skill is updated
4. **Tool removed** → skill references nonexistent tool
5. **Workflow changed** → skill encodes old sequence

### Convergence Rules
- After ANY server change, all corresponding skills must be re-audited
- Skill files must reference tools by exact name — verify against `listTools()` output
- New tools without skill coverage are effectively invisible
- Build freshness gate applies: if server source is newer than skill file, skill is stale

---

## 11. Anti-Patterns

| Anti-Pattern | Fix |
|-------------|-----|
| Too many tools (42K+ tokens) | Focused servers, 3-10 tools each |
| 1:1 REST-to-MCP mapping | Composite, goal-oriented tools |
| Poor/missing descriptions | Descriptions are prompts — optimize for selection |
| Missing JSON Schema | Full schema with types, constraints, enums |
| Blocking synchronous calls | Async I/O throughout |
| stdout logging (stdio) | Log to stderr only |
| Hardcoded secrets | Env var references, OAuth 2.1 |
| No idempotency | Deterministic results, request ID dedup |
| Raw stack traces in errors | Structured: what/why/fix |
| Monolithic server | Split into domain-focused servers |
| No corresponding skill | Ship skill file with every server |

---

## 12. Audit Dimensions

For convergence engine integration — audit MCP servers against:

1. **tool_design** — names follow convention, descriptions are prompts, schemas complete
2. **security** — input validation, no hardcoded secrets, injection defense
3. **testing** — unit + E2E, schema validation, idempotency verified
4. **error_handling** — isError for app errors, specific messages, no raw traces
5. **performance** — async I/O, connection pooling, tool count within limits
6. **documentation** — server instructions, tool descriptions, skill file exists and is current
7. **skill_sync** — corresponding skill verified against actual tool list

---

## References

- MCP Specification — modelcontextprotocol.io
- OWASP MCP Top 10 — owasp.org/www-project-mcp-top-10/
- Block's Playbook for Designing MCP Servers — engineering.block.xyz
- MCP is Not the Problem — philschmid.de/mcp-best-practices
- Building MCP Servers in the Real World — Pragmatic Engineer
- rmcp SDK — docs.rs/rmcp/
- MCP Server Security State 2025 — Astrix Security
