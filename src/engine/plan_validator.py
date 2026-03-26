"""Plan validator — code checks on plan structure.

Validates that a plan has the required sections for the engine to converge it.
No LLM needed — this is structural validation.
"""

import os
import re


class PlanValidationResult:
    def __init__(self):
        self.errors: list[str] = []
        self.warnings: list[str] = []

    @property
    def valid(self) -> bool:
        return len(self.errors) == 0

    def to_dict(self) -> dict:
        return {
            "valid": self.valid,
            "errors": self.errors,
            "warnings": self.warnings,
        }


def validate_plan(plan_path: str) -> PlanValidationResult:
    """Validate a build plan's structure for convergence."""
    result = PlanValidationResult()

    if not os.path.exists(plan_path):
        result.errors.append(f"Plan file not found: {plan_path}")
        return result

    with open(plan_path) as f:
        content = f.read()

    # Must have a title
    if not re.search(r"^#\s+", content, re.MULTILINE):
        result.errors.append("Plan must have a title (# heading)")

    # Must have phases or sections
    phase_matches = re.findall(r"^##\s+.*(?:Phase|Step)\s+\d", content, re.MULTILINE | re.IGNORECASE)
    if not phase_matches:
        result.warnings.append("Plan has no numbered phases/steps (## Phase N or ## Step N)")

    # Must have checklists
    checklist_items = re.findall(r"^\s*-\s*\[\s*[xX ]?\s*\]", content, re.MULTILINE)
    if not checklist_items:
        result.errors.append("Plan must have checklist items (- [ ] item)")

    # Should have test commands
    if not re.search(r"test|pytest|bun test|npm test|cargo test", content, re.IGNORECASE):
        result.warnings.append("Plan does not reference test commands")

    # Should reference convergence criteria
    if not re.search(r"converge|convergence|clean pass|coverage", content, re.IGNORECASE):
        result.warnings.append("Plan does not reference convergence criteria")

    # Must have Document Alignment section
    if not re.search(r"##\s+Document Alignment", content, re.IGNORECASE):
        result.warnings.append(
            "Plan has no '## Document Alignment' section. If this project has product docs "
            "or memory files with decisions the plan must respect, list them. "
            "For new projects with no existing docs, add the section with 'None — new project'."
        )

    # Should have post-execution convergence section
    if not re.search(r"Post-Execution Convergence|Documentation convergence|Website convergence", content, re.IGNORECASE):
        result.warnings.append(
            "Plan has no post-execution convergence checklist. Every plan should include: "
            "documentation convergence, website convergence, deployment, patterns update, inbox check. "
            "NOTE: If the project has a docs/ folder, documentation convergence is auto-enforced by the engine "
            "even if not listed here — but listing it makes the requirement explicit."
        )

    # Check for empty plan
    if len(content.strip()) < 50:
        result.errors.append("Plan is too short (< 50 characters)")

    return result


def get_plan_template(goal: str) -> str:
    """Return a structured plan template for a given goal."""
    return f"""# BUILD_PLAN: {goal}

**Created:** [date]
**Status:** NOT STARTED
**Goal:** {goal}

**Rule:** TDD. Tests before code.
**Rule:** 100% coverage enforced.
**Rule:** Two consecutive clean passes = convergence.

---

## Phase 1: [First phase name]

**Purpose:** [What this phase accomplishes]

### Checklist — Phase 1

- [ ] 1.1 [First task]
- [ ] 1.2 [Second task]
- [ ] 1.3 Tests pass, coverage ≥ 100%

---

## Phase 2: [Second phase name]

**Purpose:** [What this phase accomplishes]

### Checklist — Phase 2

- [ ] 2.1 [First task]
- [ ] 2.2 [Second task]
- [ ] 2.3 Tests pass, coverage ≥ 100%

---

## Document Alignment

### Product Docs (this plan must conform to):
- [path/to/relevant_doc.md] — [what decisions it contains]

### Memory Files (captured decisions this plan must respect):
- [path/to/memory_file.md] — [what decision it captures]

<!-- For new projects with no existing docs, replace the lists above with:
     None — new project, no existing product docs or decisions to align against.
     The engine will skip the document alignment phase. -->

---

## Test Commands

```bash
# Run tests with coverage
[test command here]
```

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: audit all docs against code, two clean passes
- [ ] Website convergence (if applicable): update metrics, audit content accuracy, two clean passes
- [ ] Deployment (if applicable): deploy per docs/DEPLOYMENT.md
- [ ] Patterns update: capture learnings if novel
- [ ] Inbox check: process messages from other sessions

## Convergence Criteria

- All checklist items complete (including post-execution items above)
- All tests pass
- Coverage ≥ 100%
- Two consecutive clean audit passes
- Documentation verified against code
- Website metrics current (if applicable)
"""
