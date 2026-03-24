"""CruxDev status — health checks for the installation.

Verifies MCP server, tools, state directory, config, dependencies,
methodology files, active convergences, and slash commands.
"""

from __future__ import annotations

import importlib
import os
import sys
from dataclasses import dataclass, field


CRUXDEV_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

EXPECTED_DOCS = [
    "docs/DEVELOPMENT_PATTERNS_CRUXDEV.md",
    "docs/ADOPTION_PROCESS.md",
    "docs/WEBSITE_PLANNING.md",
    "docs/RESEARCH_PATTERNS.md",
    "docs/COMPETITORS_PATTERN.md",
]

EXPECTED_COMMANDS = [
    ".claude/commands/converge.md",
    ".claude/commands/plan.md",
    ".claude/commands/adopt.md",
    ".claude/commands/status.md",
    ".claude/commands/inbox.md",
    ".claude/commands/competitor-add.md",
]


@dataclass
class Check:
    name: str
    passed: bool
    message: str


@dataclass
class StatusReport:
    healthy: bool = True
    checks: list[Check] = field(default_factory=list)
    warnings: list[Check] = field(default_factory=list)
    active_convergences: list[dict] = field(default_factory=list)
    versions: dict = field(default_factory=dict)


def check_mcp_server() -> Check:
    """Check if the MCP server module loads."""
    try:
        from src.mcp_server import mcp
        return Check("MCP server", True, "Module loads successfully")
    except Exception as e:  # pragma: no cover — import failure
        return Check("MCP server", False, f"Failed to load: {e}")


def check_tool_count() -> Check:
    """Check expected MCP tool count."""
    try:
        server_path = os.path.join(CRUXDEV_ROOT, "src", "mcp_server.py")
        with open(server_path) as f:
            count = f.read().count("@mcp.tool()")
        return Check("Tool count", True, f"{count} tools registered")
    except Exception as e:
        return Check("Tool count", False, f"Cannot count tools: {e}")


def check_state_directory(project_dir: str) -> Check:
    """Check if .cruxdev/ exists."""
    state_dir = os.path.join(project_dir, ".cruxdev")
    if os.path.isdir(state_dir):
        return Check("State directory", True, f".cruxdev/ exists at {project_dir}")
    return Check("State directory", False, f".cruxdev/ not found at {project_dir}")


def check_mcp_config(project_dir: str) -> Check:
    """Check if cruxdev is in .mcp.json."""
    mcp_path = os.path.join(project_dir, ".mcp.json")
    if not os.path.exists(mcp_path):
        return Check("MCP config", False, f".mcp.json not found at {project_dir}")
    try:
        import json
        with open(mcp_path) as f:
            config = json.load(f)
        servers = config.get("mcpServers", {})
        if "cruxdev" in servers:
            return Check("MCP config", True, "cruxdev in .mcp.json")
        return Check("MCP config", False, "cruxdev not in .mcp.json")
    except Exception as e:
        return Check("MCP config", False, f"Cannot read .mcp.json: {e}")


def check_crux_integration(project_dir: str) -> Check:
    """Check if Crux MCP is also configured."""
    mcp_path = os.path.join(project_dir, ".mcp.json")
    if not os.path.exists(mcp_path):
        return Check("Crux integration", False, "No .mcp.json")
    try:
        import json
        with open(mcp_path) as f:
            config = json.load(f)
        if "crux" in config.get("mcpServers", {}):
            return Check("Crux integration", True, "Crux MCP also configured (full stack)")
        return Check("Crux integration", False, "Crux MCP not configured — recommended for full stack")
    except Exception:
        return Check("Crux integration", False, "Cannot read .mcp.json")


def check_python_version() -> Check:
    """Check Python version >= 3.11."""
    v = sys.version_info
    if v >= (3, 11):
        return Check("Python version", True, f"{v.major}.{v.minor}.{v.micro}")
    return Check("Python version", False, f"{v.major}.{v.minor}.{v.micro} (need >= 3.11)")  # pragma: no cover


def check_dependencies() -> Check:
    """Check required Python packages."""
    missing = []
    for pkg in ["pydantic", "mcp"]:
        try:
            importlib.import_module(pkg)
        except ImportError:
            missing.append(pkg)
    if missing:
        return Check("Dependencies", False, f"Missing: {', '.join(missing)}")
    return Check("Dependencies", True, "pydantic, mcp available")


def check_methodology_docs() -> Check:
    """Check methodology docs are accessible."""
    missing = []
    for doc in EXPECTED_DOCS:
        if not os.path.exists(os.path.join(CRUXDEV_ROOT, doc)):
            missing.append(doc)
    if missing:
        return Check("Methodology docs", False, f"Missing: {', '.join(missing)}")
    return Check("Methodology docs", True, f"{len(EXPECTED_DOCS)} docs accessible")


def check_slash_commands() -> Check:
    """Check slash commands exist."""
    missing = []
    for cmd in EXPECTED_COMMANDS:
        if not os.path.exists(os.path.join(CRUXDEV_ROOT, cmd)):
            missing.append(cmd)
    if missing:
        return Check("Slash commands", False, f"Missing: {', '.join(missing)}")
    return Check("Slash commands", True, f"{len(EXPECTED_COMMANDS)} commands available")


def check_active_convergences(project_dir: str) -> list[dict]:
    """List any active convergence runs."""
    state_dir = os.path.join(project_dir, ".cruxdev", "convergence_state")
    if not os.path.isdir(state_dir):
        return []
    import json
    runs = []
    for f in os.listdir(state_dir):
        if not f.endswith(".json"):
            continue
        try:
            with open(os.path.join(state_dir, f)) as fh:
                data = json.load(fh)
            runs.append({
                "id": f[:-5],
                "phase": data.get("phase", "unknown"),
                "round": data.get("round", 0),
            })
        except Exception:
            pass
    return runs


def get_status(project_dir: str = ".") -> StatusReport:
    """Run all health checks and return a status report."""
    project_dir = os.path.abspath(project_dir)
    report = StatusReport()

    # Critical checks
    checks = [
        check_mcp_server(),
        check_tool_count(),
        check_state_directory(project_dir),
        check_mcp_config(project_dir),
        check_python_version(),
        check_dependencies(),
        check_methodology_docs(),
        check_slash_commands(),
    ]

    for c in checks:
        report.checks.append(c)
        if not c.passed:
            report.healthy = False

    # Warnings (non-critical)
    crux = check_crux_integration(project_dir)
    if not crux.passed:
        report.warnings.append(crux)

    # Active convergences
    report.active_convergences = check_active_convergences(project_dir)

    # Versions
    report.versions = {
        "python": f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
        "cruxdev_root": CRUXDEV_ROOT,
    }

    return report
