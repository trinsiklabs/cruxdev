# /adopt — Adopt a project into CruxDev

Set up CruxDev on a project so it can use the convergence engine.

## Arguments

$ARGUMENTS = the project directory path (default: current directory)

## Protocol

### Step 1: Get the adoption process

Call `get_adoption_process()` and follow the steps.

### Step 2: Install CruxDev

Call `install_cruxdev($ARGUMENTS)` to add CruxDev MCP to the project's `.claude/mcp.json`.

### Step 3: Verify installation

Check that `.claude/mcp.json` has the cruxdev server entry. Check that `.cruxdev/` directory was created.

### Step 4: Configure test commands

Find the project's test command (pytest, bun test, cargo test, etc.). Verify it runs and passes. This is needed for convergence.

### Step 5: Create first plan

If the user has a goal in mind, use `/plan` to create a build plan.

### Step 6: Report

Tell the user:
- What was installed
- Whether Crux is also configured (recommended for full stack)
- How to start converging: `/converge BUILD_PLAN_001.md`
- Remind them to restart Claude Code to activate the new MCP tools
