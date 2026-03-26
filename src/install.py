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

SECURITY_GITIGNORE_PATTERNS = """
# CruxDev security — NEVER commit these
**/tasks/*.output
**/-Users-*
*.key
*.pem
*_deploy
.env
.env.local
.env.production
.env.*.local
.crux/
.cruxdev/convergence_state/
*.jsonl
"""


def _install_secret_scanner(project_dir: str) -> bool:
    """Install pre-commit hook that scans for secrets.

    Copies scripts/pre-commit-secrets to .git/hooks/pre-commit if a .git
    directory exists. Returns True if installed, False otherwise.
    """
    git_dir = os.path.join(project_dir, ".git")
    if not os.path.isdir(git_dir):
        return False

    hooks_dir = os.path.join(git_dir, "hooks")
    os.makedirs(hooks_dir, exist_ok=True)

    hook_src = os.path.join(CRUXDEV_ROOT, "scripts", "pre-commit-secrets")
    hook_dst = os.path.join(hooks_dir, "pre-commit")

    if not os.path.exists(hook_src):
        return False

    # Don't overwrite existing pre-commit hook
    if os.path.exists(hook_dst):
        return False

    import shutil
    shutil.copy2(hook_src, hook_dst)
    os.chmod(hook_dst, 0o755)
    return True


def _ensure_gitignore_security(project_dir: str) -> bool:
    """Ensure .gitignore has security patterns to prevent secret leaks.

    Appends security patterns if not already present. Returns True if modified.
    """
    gitignore = os.path.join(project_dir, ".gitignore")
    existing = ""
    if os.path.exists(gitignore):
        with open(gitignore) as f:
            existing = f.read()

    # Check if security section already exists
    if "CruxDev security" in existing:
        return False

    with open(gitignore, "a") as f:
        f.write(SECURITY_GITIGNORE_PATTERNS)
    return True


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

    # Copy slash commands to target project
    commands_src = os.path.join(CRUXDEV_ROOT, ".claude", "commands")
    commands_dst = os.path.join(project_dir, ".claude", "commands")
    if os.path.isdir(commands_src):
        os.makedirs(commands_dst, exist_ok=True)
        import shutil
        for cmd_file in os.listdir(commands_src):
            if cmd_file.endswith(".md"):
                src_path = os.path.join(commands_src, cmd_file)
                dst_path = os.path.join(commands_dst, cmd_file)
                shutil.copy2(src_path, dst_path)

    # Ensure .gitignore has security patterns
    _ensure_gitignore_security(project_dir)

    # Install pre-commit secret scanner
    _install_secret_scanner(project_dir)

    items = [
        f"Created .cruxdev/ in {project_dir}",
        f"Added cruxdev to .mcp.json",
        f"Added session bus hook to .claude/settings.local.json",
        f"Copied slash commands to .claude/commands/",
        f"Updated .gitignore with security patterns",
        f"Installed pre-commit secret scanner",
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
