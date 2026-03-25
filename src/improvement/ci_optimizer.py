"""CI/CD self-optimization — analyze GitHub Actions timing, identify improvements.

Tracks build times, finds parallelization and caching opportunities,
generates optimized workflow suggestions.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class WorkflowStep:
    """A single CI workflow step."""
    name: str
    duration_seconds: float
    cacheable: bool = False
    parallelizable: bool = False


@dataclass
class OptimizationSuggestion:
    """A suggested CI optimization."""
    step_name: str
    suggestion_type: str  # cache, parallelize, remove, combine
    description: str
    estimated_savings_seconds: float = 0.0


@dataclass
class CIAnalysis:
    """Result of CI workflow analysis."""
    total_duration: float = 0.0
    steps: list[WorkflowStep] = field(default_factory=list)
    suggestions: list[OptimizationSuggestion] = field(default_factory=list)

    @property
    def potential_savings(self) -> float:
        return sum(s.estimated_savings_seconds for s in self.suggestions)


def analyze_workflow(steps: list[WorkflowStep]) -> CIAnalysis:
    """Analyze a CI workflow for optimization opportunities.

    Identifies:
    - Cacheable steps (install, build)
    - Parallelizable steps (independent test suites)
    - Slow steps (> 60s)

    Never removes test steps.
    """
    analysis = CIAnalysis(
        total_duration=sum(s.duration_seconds for s in steps),
        steps=steps,
    )

    for step in steps:
        # Cacheable detection
        name_lower = step.name.lower()
        if any(kw in name_lower for kw in ("install", "dependencies", "setup")):
            if not step.cacheable:
                analysis.suggestions.append(OptimizationSuggestion(
                    step_name=step.name,
                    suggestion_type="cache",
                    description=f"Cache {step.name} to avoid re-running",
                    estimated_savings_seconds=step.duration_seconds * 0.8,
                ))

        # Parallelizable detection
        if step.parallelizable and step.duration_seconds > 30:
            analysis.suggestions.append(OptimizationSuggestion(
                step_name=step.name,
                suggestion_type="parallelize",
                description=f"Run {step.name} in parallel with other steps",
                estimated_savings_seconds=step.duration_seconds * 0.5,
            ))

        # Slow step warning
        if step.duration_seconds > 120:
            analysis.suggestions.append(OptimizationSuggestion(
                step_name=step.name,
                suggestion_type="optimize",
                description=f"{step.name} takes {step.duration_seconds:.0f}s — investigate optimization",
                estimated_savings_seconds=step.duration_seconds * 0.3,
            ))

    return analysis
