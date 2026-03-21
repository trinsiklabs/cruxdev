# /converge — Run the CruxDev convergence engine

Execute the full convergence loop on a build plan. The engine owns all termination logic — you execute the tasks it gives you.

## Arguments

$ARGUMENTS = the build plan file path (e.g., BUILD_PLAN_001.md)

If no argument provided, look for BUILD_PLAN_*.md files in the current directory.

## Protocol

### Step 1: Start convergence

```
Call start_convergence(plan_file=$ARGUMENTS)
```

Pass source_files, doc_files, and test_command if you know them. If not, read the plan file first to find them.

### Step 2: Convergence loop

Repeat until done:

1. Call `convergence_next_task(convergence_id)` to get your task
2. Read the task:
   - **"audit"**: Read the listed files. Check each dimension listed. Report findings as JSON.
   - **"fix"**: Fix the specific finding. Run tests if test_command is provided.
   - **"test"**: Run the test command. Report pass/fail.
   - **"write"**: Write or update the specified file.
   - **"done"**: Convergence complete. Report the final status.
   - **"escalated"**: Engine stopped. Report why and what to do next.
3. Execute the task thoroughly — read files, analyze code, fix issues
4. Call `convergence_submit_result(convergence_id, findings_json)`:
   - Clean pass: `"[]"`
   - Found issues: `[{"id": "f1", "file": "path", "dimension": "correctness", "severity": "high", "description": "what's wrong", "suggested_fix": "how to fix", "fixed": true}]`
5. Go to step 1

### Step 3: Report

When you get "done" or "escalated", call `convergence_status(convergence_id)` and report:
- Total rounds completed
- Total findings found and fixed
- Time elapsed
- Final phase

## Rules

- **Do NOT decide when to stop.** The engine decides. Keep looping until task_type is "done" or "escalated".
- **Do NOT skip dimensions.** The engine tells you which dimensions to audit. Check all of them.
- **Report honestly.** If you find issues, report them. Don't mark findings as "fixed" unless you actually fixed them.
- **Two consecutive clean passes = convergence.** One clean pass is not enough.
- **If escalated, explain why** and suggest what the user should do.
