"""Website metrics auto-update — keep site stats current.

Fetches metrics from GitHub API, test suites, and feature counts.
Provides data for injection into Astro site templates.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
from dataclasses import dataclass


@dataclass
class ProjectMetrics:
    test_count: int = 0
    coverage: float = 0.0
    tool_count: int = 0
    mode_count: int = 0
    provider_count: int = 0
    github_stars: int = 0


def count_tests(project_dir: str, test_command: list[str] | None = None) -> int:
    """Count tests by running the test suite with collection only."""
    if test_command is None:
        test_command = ["python3", "-m", "pytest", "--collect-only", "-q"]

    try:
        result = subprocess.run(
            test_command,
            capture_output=True,
            timeout=60,
            cwd=project_dir,
        )
        output = result.stdout.decode()
        # Look for pytest output like "314 tests collected"
        match = re.search(r"(\d+)\s+test", output)
        if match:
            return int(match.group(1))
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass
    return 0


def count_mcp_tools(server_file: str) -> int:
    """Count @mcp.tool() decorators in an MCP server file."""
    try:
        with open(server_file) as f:
            content = f.read()
        return content.count("@mcp.tool()")
    except (FileNotFoundError, OSError):
        return 0


def count_modes(modes_dir: str) -> int:
    """Count mode files in a modes directory."""
    try:
        return len([f for f in os.listdir(modes_dir) if f.endswith(".md")])
    except (FileNotFoundError, OSError):
        return 0


def gather_metrics(
    project_dir: str,
    server_file: str | None = None,
    modes_dir: str | None = None,
    test_command: list[str] | None = None,
) -> ProjectMetrics:
    """Gather all metrics for a project."""
    metrics = ProjectMetrics()
    metrics.test_count = count_tests(project_dir, test_command)
    if server_file:
        metrics.tool_count = count_mcp_tools(server_file)
    if modes_dir:
        metrics.mode_count = count_modes(modes_dir)
    return metrics


def format_metrics_json(metrics: ProjectMetrics) -> str:
    """Format metrics as JSON for site injection."""
    return json.dumps({
        "test_count": metrics.test_count,
        "coverage": metrics.coverage,
        "tool_count": metrics.tool_count,
        "mode_count": metrics.mode_count,
        "provider_count": metrics.provider_count,
        "github_stars": metrics.github_stars,
    }, indent=2)
