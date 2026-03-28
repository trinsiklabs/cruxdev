# /cruxdev-plan — Create a CruxDev build plan

Create a structured build plan that the convergence engine can execute.

## Arguments

$ARGUMENTS = the goal (what to build, fix, or migrate)

## Protocol

### Step 1: Get methodology

Call `get_methodology()` to load the CruxDev development patterns. Read them — they define how plans should be structured.

### Step 2: Generate template

Call `create_plan_template($ARGUMENTS)` to get a plan skeleton.

### Step 3: Research and fill in

1. Read the codebase to understand current state
2. Fill in each phase with specific, actionable tasks
3. Add checklist items (`- [ ]`) for every task
4. Add test commands
5. Add convergence criteria
6. Write the plan to: `build_plans/BUILD_PLAN_NNN_<slug>.md` (all build plans live in `build_plans/`)

### Step 4: Validate

Call `validate_plan_structure(plan_file)` to check the plan has everything the engine needs.

Fix any errors. Address warnings if possible.

### Step 5: Confirm with user

Show the plan to the user. Ask if they want to adjust anything before convergence.

## Plan quality guidelines

- Each phase should be completable in one session
- Checklists should be specific enough that "done" is unambiguous
- Test commands should exist and pass before convergence starts
- Dependencies between phases should be explicit
- Include a "Definition of Done" section
