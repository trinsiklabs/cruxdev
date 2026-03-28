#!/bin/bash
# CruxDev Autonomous Evolution — runs every 4 hours via cron
# Single cycle per run (NOT continuous — avoids spam loop from own git changes)
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

# Run ONE evolution cycle (not continuous)
export PATH="$HOME/.cargo/bin:$PATH"
"${PROJECT_DIR}/rust/target/release/cruxdev" evolve "${PROJECT_DIR}" \
    --repo trinsiklabs/cruxdev \
    --live >> "${LOG_FILE}" 2>&1

echo "[$(date)] Evolution cycle complete." >> "${LOG_FILE}"

# Sync stream to web server for live terminal viewer
"${PROJECT_DIR}/scripts/sync-stream.sh" 2>/dev/null
