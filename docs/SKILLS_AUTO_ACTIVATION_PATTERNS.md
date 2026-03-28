# Skills Auto-Activation Patterns

**Gap:** Superpowers auto-detects which skill to activate based on task context.
**Current:** CruxDev skills are manually invoked.

## Pattern: Trigger-Based Skill Matching

Each SKILL.md includes a `triggers` field:

```yaml
triggers:
  keywords: ["convergence", "audit", "two clean passes"]
  file_patterns: ["build_plans/*.md", "*.convergence.json"]
  phases: ["CodeAuditing", "DocAuditing"]
```

When `convergence_next_task` returns a task:
1. Scan task description for trigger keywords
2. Check task files against file patterns
3. Match current phase against phase triggers
4. Return `suggested_skills` in task response

## Implementation

Add to `router.rs` `Task` struct:
```rust
pub suggested_skills: Vec<String>,
```

Scan `.claude/skills/*/SKILL.md` for trigger definitions. Rank by match score. Include top 1-2 in task response.

## When NOT to Auto-Activate

- No triggers match → no suggestion (don't guess)
- Multiple skills match equally → suggest both, don't pick
- User has disabled auto-activation in config
