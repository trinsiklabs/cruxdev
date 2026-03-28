#!/bin/bash
# CruxDev Autonomous Evolution — runs every 4 hours via cron
# Launches a Claude Code session that self-improves until nothing left to do
set -e

PROJECT_DIR="/Users/user/personal/cruxdev"
LOG_DIR="${PROJECT_DIR}/.cruxdev/evolution"
LOG_FILE="${LOG_DIR}/cron.log"
STOP_FILE="${LOG_DIR}/STOP"

mkdir -p "${LOG_DIR}"

# Check emergency stop
if [ -f "${STOP_FILE}" ]; then
    echo "[$(date)] STOP file detected. Skipping." >> "${LOG_FILE}"
    exit 0
fi

echo "[$(date)] Evolution cycle starting." >> "${LOG_FILE}"

# Run the evolution cycle (gather/evaluate/post/engage) — no LLM needed
export PATH="$HOME/.cargo/bin:$PATH"
"${PROJECT_DIR}/rust/target/release/cruxdev" evolve "${PROJECT_DIR}" \
    --repo trinsiklabs/cruxdev \
    --dry-run false \
    --continuous >> "${LOG_FILE}" 2>&1

# For full self-improvement (code changes, convergence), launch Claude Code
# Uncomment when ready for fully autonomous mode:
# claude --print -p "Self-adopt to convergence. Check GitHub issues, fix any bugs, converge any open build plans, generate blog posts, deploy the site. Stop when nothing left to do." \
#     --allowedTools "Read,Write,Edit,Bash,Glob,Grep" \
#     2>> "${LOG_FILE}"

echo "[$(date)] Evolution cycle complete." >> "${LOG_FILE}"
