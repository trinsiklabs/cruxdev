#!/usr/bin/env python3
"""Parse Claude Code JSONL transcript into terminal viewer JSON."""
import sys, json
from datetime import datetime

lines = []

for raw in sys.stdin:
    raw = raw.strip()
    if not raw:
        continue
    try:
        d = json.loads(raw)
    except json.JSONDecodeError:
        continue

    msg_type = d.get('type', '')

    if msg_type == 'assistant':
        content = d.get('message', {}).get('content', [])
        if not isinstance(content, list):
            continue
        for block in content:
            if not isinstance(block, dict):
                continue
            btype = block.get('type', '')

            if btype == 'text':
                text = block.get('text', '').strip()
                if not text or len(text) > 1000:
                    continue
                for line in text.split('\n'):
                    line = line.strip()
                    if not line:
                        continue
                    if len(line) > 300:
                        line = line[:297] + '...'
                    lower = line.lower()
                    if 'error' in lower or 'failed' in lower:
                        t = 'error'
                    elif any(w in lower for w in ['converged', 'passed', 'pushed', 'purged', 'deployed']):
                        t = 'success'
                    elif line.startswith('**') or line.startswith('##') or 'BP0' in line or 'BP1' in line:
                        t = 'heading'
                    else:
                        t = 'info'
                    lines.append({'text': line, 'line_type': t, 'timestamp': ''})

            elif btype == 'tool_use':
                name = block.get('name', '?')
                inp = block.get('input', {})
                if name == 'Bash':
                    cmd = inp.get('command', '').split('\n')[0][:200]
                    desc = inp.get('description', '')
                    display = desc if desc else cmd
                    if display:
                        lines.append({'text': '$ ' + display, 'line_type': 'tool', 'timestamp': ''})
                elif name in ('Write', 'Edit'):
                    path = inp.get('file_path', '')
                    short = path.split('/')[-1] if '/' in path else path
                    if short:
                        lines.append({'text': f'[{name}] {short}', 'line_type': 'tool', 'timestamp': ''})
                elif name == 'Agent':
                    desc = inp.get('description', '')[:100]
                    lines.append({'text': f'[Agent] {desc}', 'line_type': 'heading', 'timestamp': ''})
                elif name.startswith('mcp__'):
                    short_name = name.replace('mcp__cruxdev__', '').replace('mcp__crux__', 'crux:')
                    lines.append({'text': f'[MCP] {short_name}', 'line_type': 'tool', 'timestamp': ''})
                elif name in ('Read', 'Glob', 'Grep'):
                    pass  # Skip read operations — too noisy
                else:
                    lines.append({'text': f'[{name}]', 'line_type': 'tool', 'timestamp': ''})

    elif msg_type == 'user':
        msg = d.get('message', {})
        content = msg.get('content', '')
        if isinstance(content, str) and content.strip():
            text = content.strip()
            if len(text) > 200:
                text = text[:197] + '...'
            lines.append({'text': '> ' + text, 'line_type': 'heading', 'timestamp': ''})
        elif isinstance(content, list):
            for block in content:
                if isinstance(block, dict) and block.get('type') == 'text':
                    text = block.get('text', '').strip()[:200]
                    if text:
                        lines.append({'text': '> ' + text, 'line_type': 'heading', 'timestamp': ''})

lines = lines[-200:]

json.dump({
    'lines': lines,
    'updated': datetime.now().isoformat(),
    'active': True,
}, sys.stdout)
