"""Tests for timeout enforcement."""

import time

from src.engine.state import ConvergenceState
from src.engine.timeout import (
    clear_deadline,
    is_expired,
    remaining_seconds,
    set_phase_deadline,
    set_task_deadline,
)


def test_set_task_deadline():
    s = ConvergenceState(plan_file="p.md", timeout_per_task=60.0)
    before = time.time()
    set_task_deadline(s)
    after = time.time()
    assert s.deadline is not None
    assert before + 60.0 <= s.deadline <= after + 60.0


def test_set_phase_deadline():
    s = ConvergenceState(plan_file="p.md")
    before = time.time()
    set_phase_deadline(s, 120.0)
    after = time.time()
    assert s.deadline is not None
    assert before + 120.0 <= s.deadline <= after + 120.0


def test_remaining_seconds_no_deadline():
    s = ConvergenceState(plan_file="p.md")
    assert remaining_seconds(s) == float("inf")


def test_remaining_seconds_with_deadline():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() + 100)
    r = remaining_seconds(s)
    assert 99 <= r <= 100


def test_remaining_seconds_expired():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() - 10)
    assert remaining_seconds(s) == 0


def test_is_expired_no_deadline():
    s = ConvergenceState(plan_file="p.md")
    assert is_expired(s) is False


def test_is_expired_future():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() + 100)
    assert is_expired(s) is False


def test_is_expired_past():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() - 1)
    assert is_expired(s) is True


def test_clear_deadline():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() + 100)
    clear_deadline(s)
    assert s.deadline is None
    assert is_expired(s) is False
