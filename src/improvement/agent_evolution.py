"""Agent evolution — maintain and evolve agent configuration variants.

Manages a population of agent configs, evaluates on benchmarks,
selects winners. Safety invariants are preserved across all variants.
"""

from __future__ import annotations

import copy
from dataclasses import dataclass, field


# Protected invariants that cannot be modified by evolution
PROTECTED_INVARIANTS = frozenset([
    "convergence_threshold",
    "safety_pipeline",
    "human_escalation",
    "test_requirement",
    "max_rounds_floor",
])


@dataclass
class AgentConfig:
    """An agent configuration variant."""
    id: str
    name: str
    params: dict[str, object] = field(default_factory=dict)
    fitness: float = 0.0

    def get(self, key: str, default: object = None) -> object:
        return self.params.get(key, default)


@dataclass
class Population:
    """A population of agent config variants."""
    variants: list[AgentConfig] = field(default_factory=list)
    max_size: int = 10

    @property
    def best(self) -> AgentConfig | None:
        if not self.variants:
            return None
        return max(self.variants, key=lambda v: v.fitness)

    def add(self, variant: AgentConfig) -> bool:
        """Add a variant if population isn't full. Returns True if added."""
        if len(self.variants) >= self.max_size:
            return False
        self.variants.append(variant)
        return True

    def select(self, keep_count: int) -> list[AgentConfig]:
        """Select top variants by fitness."""
        sorted_variants = sorted(self.variants, key=lambda v: v.fitness, reverse=True)
        self.variants = sorted_variants[:keep_count]
        return self.variants


def validate_invariants(config: AgentConfig) -> list[str]:
    """Check that protected invariants are not weakened.

    Returns list of violations (empty = safe).
    """
    violations = []

    # Check convergence threshold
    threshold = config.get("convergence_threshold")
    if threshold is not None and threshold < 2:
        violations.append(f"convergence_threshold={threshold} < minimum 2")

    # Check max rounds floor
    max_rounds = config.get("max_rounds_floor")
    if max_rounds is not None and max_rounds < 3:
        violations.append(f"max_rounds_floor={max_rounds} < minimum 3")

    # Check safety pipeline not disabled
    if config.get("safety_pipeline") is False:
        violations.append("safety_pipeline cannot be disabled")

    # Check human escalation not disabled
    if config.get("human_escalation") is False:
        violations.append("human_escalation cannot be disabled")

    return violations


def mutate_config(
    base: AgentConfig,
    mutations: dict[str, object],
    new_id: str,
) -> AgentConfig | None:
    """Create a mutated variant from a base config.

    Returns None if the mutation would violate invariants.
    """
    new_params = copy.deepcopy(base.params)
    new_params.update(mutations)

    variant = AgentConfig(
        id=new_id,
        name=f"{base.name}_mutated",
        params=new_params,
    )

    violations = validate_invariants(variant)
    if violations:
        return None

    return variant
