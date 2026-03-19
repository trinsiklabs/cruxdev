"""Clock-based timeout enforcement.

Real time, not LLM estimated time. Pure code.
"""

import time

from .state import ConvergenceState


def set_task_deadline(state: ConvergenceState) -> None:
    """Set deadline for current task. Real clock."""
    state.deadline = time.time() + state.timeout_per_task


def set_phase_deadline(state: ConvergenceState, timeout: float) -> None:
    """Set deadline for entire phase."""
    state.deadline = time.time() + timeout


def remaining_seconds(state: ConvergenceState) -> float:
    """How much time is left. Real clock."""
    if state.deadline is None:
        return float("inf")
    return max(0, state.deadline - time.time())


def is_expired(state: ConvergenceState) -> bool:
    """Check if deadline has passed."""
    if state.deadline is None:
        return False
    return time.time() > state.deadline


def clear_deadline(state: ConvergenceState) -> None:
    """Remove deadline."""
    state.deadline = None
