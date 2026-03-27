#!/bin/bash
# Convergence gate — automated checks that MUST pass before any build plan
# can be marked CONVERGED. Run from the cruxdev repo root.
#
# Usage: ./scripts/convergence_gate.sh [--fix]
#   --fix: attempt to auto-fix simple issues (metric updates)
#
# Exit code: number of failures (0 = all clear)

set -o pipefail
FAILURES=0
WARNINGS=0
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$REPO_DIR/rust"
SITE_DIR="$(dirname "$REPO_DIR")/cruxdev-dev"

echo "=== CruxDev Convergence Gate ==="
echo "Repo: $REPO_DIR"
echo "Rust: $RUST_DIR"
echo "Site: $SITE_DIR"
echo ""

# ── 1. Tests pass ──
echo -n "[1/10] Tests pass... "
TEST_OUTPUT=$(cd "$RUST_DIR" && cargo test 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    ACTUAL_UNIT=$(echo "$TEST_OUTPUT" | grep "test result: ok" | head -1 | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+')
    ACTUAL_E2E=$(echo "$TEST_OUTPUT" | grep "test result: ok" | tail -1 | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+')
    ACTUAL_TOTAL=$((ACTUAL_UNIT + ACTUAL_E2E))
    echo "PASS ($ACTUAL_TOTAL tests)"
else
    echo "FAIL"
    FAILURES=$((FAILURES+1))
fi

# ── 2. Clippy clean ──
echo -n "[2/10] Clippy clean... "
if (cd "$RUST_DIR" && cargo clippy -- -D warnings 2>&1 | tail -1 | grep -q "Finished"); then
    echo "PASS"
else
    echo "FAIL"
    FAILURES=$((FAILURES+1))
fi

# ── 3. No personal names in docs ──
echo -n "[3/10] No personal names... "
NAME_COUNT=$(grep -ri "bryan" "$REPO_DIR/docs/" "$REPO_DIR/.claude/CLAUDE.md" "$REPO_DIR/README.md" 2>/dev/null | grep -v COMPETITORS | grep -v BUILD_PLAN_035 | grep -v BUILD_PLAN_041 | wc -l | tr -d ' ')
if [ "$NAME_COUNT" -eq 0 ]; then
    echo "PASS"
else
    echo "FAIL ($NAME_COUNT references)"
    FAILURES=$((FAILURES+1))
fi

# ── 4. No hardcoded dark classes on site ──
echo -n "[4/10] No dark-only classes on site... "
if [ -d "$SITE_DIR/src/pages" ]; then
    DARK_COUNT=$(grep -r "text-dev-\|bg-dev-[0-9]\|border-dev-\|text-white" "$SITE_DIR/src/pages/" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$DARK_COUNT" -eq 0 ]; then
        echo "PASS"
    else
        echo "FAIL ($DARK_COUNT hardcoded classes)"
        FAILURES=$((FAILURES+1))
    fi
else
    echo "SKIP (no site dir)"
fi

# ── 5. Test count on site matches actual ──
echo -n "[5/10] Site test count matches... "
if [ -n "$ACTUAL_TOTAL" ] && [ -f "$SITE_DIR/src/layouts/Base.astro" ]; then
    SITE_TESTS=$(grep -oE '[0-9]+ tests' "$SITE_DIR/src/layouts/Base.astro" | grep -oE '[0-9]+')
    if [ "$ACTUAL_TOTAL" = "$SITE_TESTS" ]; then
        echo "PASS ($ACTUAL_TOTAL)"
    else
        echo "FAIL (site: $SITE_TESTS, actual: $ACTUAL_TOTAL)"
        FAILURES=$((FAILURES+1))
    fi
else
    echo "SKIP"
fi

# ── 6. Tool count ──
echo -n "[6/10] Tool count... "
TOOL_COUNT=$(grep "async fn " "$RUST_DIR/src/server.rs" 2>/dev/null | grep -v run_server | wc -l | tr -d ' ')
echo "INFO ($TOOL_COUNT tools)"

# ── 7. Skill count and coverage ──
echo -n "[7/10] Skill coverage... "
SKILL_COUNT=$(ls "$REPO_DIR/.claude/skills/"*/SKILL.md 2>/dev/null | wc -l | tr -d ' ')
echo "INFO ($SKILL_COUNT skills for $TOOL_COUNT tools)"
if [ "$SKILL_COUNT" -lt 5 ]; then
    echo "  WARNING: Very low skill coverage"
    WARNINGS=$((WARNINGS+1))
fi

# ── 8. Build artifacts fresh ──
echo -n "[8/10] Build artifacts fresh... "
if [ -f "$RUST_DIR/target/release/cruxdev" ]; then
    BINARY_TIME=$(stat -f %m "$RUST_DIR/target/release/cruxdev" 2>/dev/null)
    NEWEST_SRC=$(find "$RUST_DIR/src" -name "*.rs" -newer "$RUST_DIR/target/release/cruxdev" 2>/dev/null | head -1)
    if [ -z "$NEWEST_SRC" ]; then
        echo "PASS"
    else
        echo "FAIL (binary stale, newer source: $NEWEST_SRC)"
        FAILURES=$((FAILURES+1))
    fi
else
    echo "SKIP (no release binary)"
fi

# ── 9. No stale build plan statuses ──
echo -n "[9/10] Build plan statuses... "
IN_PROGRESS=$(grep -l "IN PROGRESS" "$REPO_DIR/build_plans/"*.md 2>/dev/null | wc -l | tr -d ' ')
NOT_STARTED=$(grep -l "NOT STARTED" "$REPO_DIR/build_plans/"*.md 2>/dev/null | wc -l | tr -d ' ')
CONVERGED=$(grep -l "CONVERGED" "$REPO_DIR/build_plans/"*.md 2>/dev/null | wc -l | tr -d ' ')
echo "INFO ($CONVERGED converged, $NOT_STARTED not started, $IN_PROGRESS in progress)"

# ── 10. Ecosystem-neutral language ──
echo -n "[10/10] Ecosystem-neutral... "
CLAUDE_ONLY=$(grep -ri "prerequisite.*claude code\b" "$REPO_DIR/docs/" 2>/dev/null | grep -v COMPETITORS | grep -v MCP_SERVER | wc -l | tr -d ' ')
if [ "$CLAUDE_ONLY" -eq 0 ]; then
    echo "PASS"
else
    echo "FAIL ($CLAUDE_ONLY Claude-only prerequisite references)"
    FAILURES=$((FAILURES+1))
fi

echo ""
echo "=== RESULTS ==="
echo "Failures: $FAILURES"
echo "Warnings: $WARNINGS"
echo ""

if [ "$FAILURES" -eq 0 ]; then
    echo "CONVERGENCE GATE: PASSED"
else
    echo "CONVERGENCE GATE: FAILED ($FAILURES issues must be fixed)"
fi

exit $FAILURES
