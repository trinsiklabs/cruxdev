#!/bin/bash
# Sync CruxDev session log to the web server for live terminal viewer
# Runs after evolve.sh or as a separate cron job

PROJECT_DIR="/Users/user/personal/cruxdev"
LOG_FILE="${PROJECT_DIR}/.cruxdev/evolution/cron.log"
STREAM_FILE="/tmp/cruxdev-stream.json"
REMOTE="cruxdev.dev@vh1.trinsik.io"
SSH_KEY="$HOME/.ssh/cruxdev_deploy"

# Get last 200 lines, convert to JSON array
if [ -f "$LOG_FILE" ]; then
    tail -200 "$LOG_FILE" | python3 -c "
import sys, json
from datetime import datetime

lines = []
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    # Classify
    lower = line.lower()
    if 'error' in lower or 'failed' in lower:
        t = 'error'
    elif 'converged' in lower or 'passed' in lower or 'purged' in lower:
        t = 'success'
    elif 'cargo' in lower or 'npm' in lower or 'rsync' in lower or 'git' in lower:
        t = 'tool'
    elif 'build_plan' in lower or 'bp0' in lower:
        t = 'heading'
    else:
        t = 'info'

    # Extract timestamp if present
    ts = ''
    if line.startswith('[') and ']' in line:
        ts = line[1:line.index(']')]
        line = line[line.index(']')+1:].strip()

    lines.append({'text': line[:500], 'line_type': t, 'timestamp': ts})

json.dump({'lines': lines, 'updated': datetime.now().isoformat(), 'active': True}, sys.stdout)
" > "$STREAM_FILE"

    # Upload to web server
    rsync -az -e "ssh -i $SSH_KEY -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new" \
        "$STREAM_FILE" "${REMOTE}:~/public_html/api/stream.json" 2>/dev/null
fi
