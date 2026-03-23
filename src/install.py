"""Install CruxDev MCP server into a project.

Adds CruxDev to the project's .claude/mcp.json alongside any existing
MCP servers (like Crux). Non-destructive — preserves existing config.

Usage:
    python3 -m src.install                    # Install in current directory
    python3 -m src.install /path/to/project   # Install in specified directory
"""

import json
import os
import sys


CRUXDEV_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def get_python_path() -> str:
    """Get the Python interpreter path."""
    return sys.executable


def install(project_dir: str = ".") -> dict:
    """Install CruxDev MCP server into a project.

    Creates or updates .claude/mcp.json with the cruxdev server config.
    Preserves any existing MCP servers (e.g., Crux).

    Returns dict with status and what was done.
    """
    project_dir = os.path.abspath(project_dir)
    # Claude Code reads MCP config from .mcp.json at project root (not .claude/mcp.json)
    mcp_json_path = os.path.join(project_dir, ".mcp.json")
    state_dir = os.path.join(project_dir, ".cruxdev")

    os.makedirs(state_dir, exist_ok=True)

    # Load existing config or create new
    existing = {}
    if os.path.exists(mcp_json_path):
        with open(mcp_json_path) as f:
            existing = json.load(f)

    servers = existing.get("mcpServers", {})

    # Add CruxDev server
    servers["cruxdev"] = {
        "command": get_python_path(),
        "args": ["-m", "src.mcp_server"],
        "cwd": CRUXDEV_ROOT,
        "env": {
            "PYTHONPATH": CRUXDEV_ROOT,
        },
    }

    existing["mcpServers"] = servers

    with open(mcp_json_path, "w") as f:
        json.dump(existing, f, indent=2)

    # Install session bus hook for push notifications
    project_name = os.path.basename(project_dir)
    hook_script = os.path.join(CRUXDEV_ROOT, "src", "bus", "hook_runner.py")
    settings_path = os.path.join(project_dir, ".claude", "settings.local.json")
    os.makedirs(os.path.dirname(settings_path), exist_ok=True)

    settings = {}
    if os.path.exists(settings_path):
        with open(settings_path) as f:
            try:
                settings = json.load(f)
            except json.JSONDecodeError:
                settings = {}

    hooks = settings.get("hooks", {})
    post_tool = hooks.get("PostToolUse", [])

    # Add bus hook if not already present
    hook_cmd = f"{get_python_path()} {hook_script} {project_name}"
    if not any(hook_cmd in str(h) for h in post_tool):
        post_tool.append({"command": hook_cmd})
        hooks["PostToolUse"] = post_tool
        settings["hooks"] = hooks
        with open(settings_path, "w") as f:
            json.dump(settings, f, indent=2)

    items = [
        f"Created .cruxdev/ in {project_dir}",
        f"Added cruxdev to .mcp.json",
        f"Added session bus hook to .claude/settings.local.json",
        f"CruxDev root: {CRUXDEV_ROOT}",
        f"Python: {get_python_path()}",
    ]

    # Check if Crux is also configured
    if "crux" in servers:
        items.append("Crux MCP server also configured (good — full stack)")
    else:
        items.append(
            "Note: Crux MCP server not configured. For full stack "
            "(modes, memory, safety), also install Crux."
        )

    return {"status": "installed", "items": items}


def uninstall(project_dir: str = ".") -> dict:
    """Remove CruxDev from a project's .mcp.json.

    Does not delete .cruxdev/ state — preserves convergence history.
    """
    project_dir = os.path.abspath(project_dir)
    mcp_json_path = os.path.join(project_dir, ".mcp.json")

    if not os.path.exists(mcp_json_path):
        return {"status": "not_installed", "message": "No .mcp.json found"}

    with open(mcp_json_path) as f:
        config = json.load(f)

    servers = config.get("mcpServers", {})
    if "cruxdev" not in servers:
        return {"status": "not_installed", "message": "cruxdev not in mcp.json"}

    del servers["cruxdev"]
    config["mcpServers"] = servers

    with open(mcp_json_path, "w") as f:
        json.dump(config, f, indent=2)

    return {"status": "uninstalled", "message": "Removed cruxdev from mcp.json"}


if __name__ == "__main__":  # pragma: no cover
    target = sys.argv[1] if len(sys.argv) > 1 else "."
    result = install(target)
    for item in result["items"]:
        print(f"  ✓ {item}")
    print(f"\nCruxDev installed. Restart Claude Code to activate.")
