"""Automated dependency updates with verification.

Scans for outdated dependencies, classifies by update type (major/minor/patch),
and provides update+test+rollback flow.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
from dataclasses import dataclass
from enum import Enum


class UpdateType(Enum):
    PATCH = "patch"
    MINOR = "minor"
    MAJOR = "major"


@dataclass
class OutdatedDep:
    name: str
    current: str
    latest: str
    update_type: UpdateType


@dataclass
class UpdateResult:
    dep: OutdatedDep
    success: bool
    tests_passed: bool
    message: str


def classify_update(current: str, latest: str) -> UpdateType:
    """Classify version change as patch, minor, or major."""
    def parse(v: str) -> tuple[int, ...] | None:
        parts = re.findall(r'\d+', v)
        return tuple(int(p) for p in parts[:3]) if len(parts) >= 3 else None

    curr = parse(current)
    lat = parse(latest)

    if curr is None or lat is None:
        return UpdateType.MAJOR

    if curr[0] != lat[0]:
        return UpdateType.MAJOR
    if curr[1] != lat[1]:
        return UpdateType.MINOR
    return UpdateType.PATCH


def scan_python_outdated(project_dir: str) -> list[OutdatedDep]:  # pragma: no cover — subprocess
    """Scan for outdated Python packages using pip."""
    try:
        result = subprocess.run(
            ["pip", "list", "--outdated", "--format=json"],
            capture_output=True,
            timeout=60,
            cwd=project_dir,
        )
        if result.returncode != 0:
            return []

        packages = json.loads(result.stdout.decode())
        return [
            OutdatedDep(
                name=p["name"],
                current=p["version"],
                latest=p["latest_version"],
                update_type=classify_update(p["version"], p["latest_version"]),
            )
            for p in packages
        ]
    except (subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError):
        return []


def update_dependency(  # pragma: no cover — subprocess
    dep: OutdatedDep,
    project_dir: str,
    test_command: list[str] | None = None,
) -> UpdateResult:
    """Update a single dependency and verify with tests.

    Returns UpdateResult with success status.
    """
    # Install the update
    try:
        install = subprocess.run(
            ["pip", "install", f"{dep.name}=={dep.latest}"],
            capture_output=True,
            timeout=120,
            cwd=project_dir,
        )
        if install.returncode != 0:
            return UpdateResult(
                dep=dep, success=False, tests_passed=False,
                message=f"pip install failed: {install.stderr.decode()[:200]}",
            )
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return UpdateResult(
            dep=dep, success=False, tests_passed=False,
            message="pip install timed out or not found",
        )

    # Run tests if command provided
    if test_command:
        try:
            test_result = subprocess.run(
                test_command,
                capture_output=True,
                timeout=300,
                cwd=project_dir,
            )
            if test_result.returncode != 0:
                # Rollback
                subprocess.run(
                    ["pip", "install", f"{dep.name}=={dep.current}"],
                    capture_output=True,
                    timeout=120,
                    cwd=project_dir,
                )
                return UpdateResult(
                    dep=dep, success=False, tests_passed=False,
                    message=f"Tests failed after update, rolled back to {dep.current}",
                )
        except subprocess.TimeoutExpired:
            subprocess.run(
                ["pip", "install", f"{dep.name}=={dep.current}"],
                capture_output=True,
                timeout=120,
                cwd=project_dir,
            )
            return UpdateResult(
                dep=dep, success=False, tests_passed=False,
                message="Tests timed out, rolled back",
            )

    return UpdateResult(
        dep=dep, success=True, tests_passed=True,
        message=f"Updated {dep.name} from {dep.current} to {dep.latest}",
    )
