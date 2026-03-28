# KV-Cache-Aware Context Engineering Patterns

**Purpose:** Maximize KV-cache hit rates in convergence loops to reduce token costs by up to 10x.
**Source:** Manus proved these patterns at production scale (4x speed, 10x cost reduction).
**Applies to:** Any system that makes repeated LLM calls with similar prefixes (convergence loops, audit cycles, research passes).

---

## Core Principle

Cached tokens cost $0.30/MTok vs $3.00/MTok uncached (Claude Sonnet, March 2026). The KV-cache stores the key-value pairs from previous prompt processing. If the beginning of a new prompt is identical to a cached prompt, those tokens are "cache hits" — processed 10x cheaper and faster.

**The rule:** Keep prompt prefixes stable. Push variable content to the end. Never modify earlier messages.

---

## Pattern 1: Append-Only Context

**Rule:** In convergence loops, NEVER modify or delete earlier messages. Only append.

```
Round 1: [System] [CLAUDE.md] [Methodology] [Task: audit round 1]
Round 2: [System] [CLAUDE.md] [Methodology] [Task: audit round 1] [Result 1] [Task: audit round 2]
Round 3: [System] [CLAUDE.md] [Methodology] [Task: audit round 1] [Result 1] [Task: audit round 2] [Result 2] [Task: fix round 3]
```

The prefix `[System] [CLAUDE.md] [Methodology]` is identical across all rounds → cache hit.

**Anti-pattern:** Summarizing and replacing earlier rounds. This destroys the cache prefix.

---

## Pattern 2: Stable Prompt Prefix

**Rule:** System prompt, CLAUDE.md, methodology docs, and tool schemas must be frozen at loop start. No timestamps, no round numbers, no variable content in the first N tokens.

**Good:**
```
System: You are CruxDev convergence engine...
CLAUDE.md: [frozen at session start]
Methodology: [frozen at session start]
Tools: [all 60 tools, frozen schema]
---
[Variable content starts here: round number, findings, task description]
```

**Bad:**
```
System: You are CruxDev convergence engine. Current time: 2026-03-28T22:15:03Z. Round 3 of 5.
```

The timestamp and round number change every call → cache miss on the first token.

---

## Pattern 3: Tool Masking (Not Removal)

**Rule:** When a convergence phase doesn't need certain tools, mark them as unavailable in their description rather than removing them from the schema.

**Why:** Removing a tool changes the tool schema, which changes the prompt prefix, which invalidates the cache.

**Good:** All 60 tools present in every call. Unavailable tools have `[NOT AVAILABLE IN THIS PHASE]` in their description.

**Bad:** Coding phase has 40 tools, doc phase has 25 tools → different schema → cache miss.

---

## Pattern 4: Error Preservation

**Rule:** Keep failed attempts in context. Don't clean them up.

**Why:**
1. The model learns from errors without re-encountering them
2. Removing errors changes the message history → cache miss
3. Error context helps the model avoid repeating mistakes

**When to break this rule:** After 3+ identical failures, add a "controlled variation" message that explicitly asks the model to try a different approach. This is the only case where you add non-standard content to break a loop.

---

## Pattern 5: Cache Hit Rate Tracking

Track these metrics from API responses:

```json
{
  "usage": {
    "input_tokens": 15000,
    "cache_creation_input_tokens": 12000,
    "cache_read_input_tokens": 3000,
    "output_tokens": 500
  }
}
```

**Cache hit rate** = `cache_read_input_tokens / (cache_read_input_tokens + cache_creation_input_tokens + input_tokens)`

**Targets:**
- Round 1: 0% (cold cache, expected)
- Round 2+: >70% (prefix should be cached)
- Sustained: >80% across a convergence run

**Alert if:** Cache hit rate drops below 50% after round 2 — something is breaking the prefix.

---

## Implementation for CruxDev

CruxDev is an MCP server — it doesn't control the prompt directly. The MCP client (Claude Code, Cursor, Codex) owns the prompt construction. What CruxDev controls:

| What CruxDev Controls | What the Client Controls |
|---|---|
| Task descriptions in `convergence_next_task` | System prompt construction |
| Tool schemas (stable by default) | Message ordering |
| Finding format in `convergence_submit_result` | Context window management |
| Metadata in task responses | Compaction strategy |

**CruxDev's responsibility:**
1. Return consistent task descriptions (same format every round)
2. Never change tool schemas mid-convergence
3. Include round number and variable data in task metadata, not in tool descriptions
4. Document these patterns so MCP clients can optimize

**MCP client's responsibility:**
1. Freeze system prompt at session start
2. Append-only message history
3. Push variable content to end of messages
4. Track cache hit rate from API responses
5. Don't compact mid-convergence (destroys cache)

---

## Anti-Patterns

| Anti-Pattern | Impact | Fix |
|---|---|---|
| Timestamps in system prompt | Cache miss every call | Move timestamps to final message |
| Compacting mid-convergence | Destroys entire cache | Only compact between convergence runs |
| Removing tools per phase | Schema change = cache miss | Mask tools instead |
| Summarizing earlier rounds | Prefix changes = cache miss | Append-only |
| Different model per round | Each model has own cache | Use same model throughout |
| Changing temperature | Some providers key cache on params | Keep params stable |

---

## Audit Dimensions

| Dimension | Good | Bad |
|---|---|---|
| Prefix stability | >95% tokens identical across rounds | <80% identical |
| Cache hit rate | >70% after round 2 | <50% after round 2 |
| Tool schema stability | Identical across all rounds | Changes per phase |
| Variable content position | End of prompt | Beginning or middle |
| Compaction timing | Between runs only | Mid-convergence |
