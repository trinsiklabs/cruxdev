#!/bin/bash
# Sync Claude Code session transcript to the web server for live terminal viewer
# Parses the actual JSONL transcript — shows real tool calls, text, agent launches

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SESSION_DIR="$HOME/.claude/projects/-Users-user-personal-cruxdev"
STREAM_FILE="/tmp/cruxdev-stream.json"
REMOTE="cruxdev.dev@vh1.trinsik.io"
SSH_KEY="$HOME/.ssh/cruxdev_deploy"

# Find the most recently modified session transcript
TRANSCRIPT=$(ls -t "${SESSION_DIR}"/*.jsonl 2>/dev/null | head -1)

if [ -z "$TRANSCRIPT" ]; then
    echo '{"lines":[],"updated":"","active":false}' > "$STREAM_FILE"
else
    tail -300 "$TRANSCRIPT" | python3 "${SCRIPT_DIR}/parse-transcript.py" > "$STREAM_FILE" 2>/dev/null
fi

# Upload
rsync -az -e "ssh -i $SSH_KEY -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new" \
    "$STREAM_FILE" "${REMOTE}:~/public_html/api/stream.json" 2>/dev/null
