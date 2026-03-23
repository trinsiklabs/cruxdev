"""Beat 4: POST — generate content from evolution results.

Produces build-in-public content from what was gathered, evaluated,
and integrated. Content is a byproduct of building, not a separate effort.
"""

from __future__ import annotations

import os
import time
from dataclasses import dataclass

from .state import EvolutionCycle


@dataclass
class PostContent:
    title: str
    body: str
    post_type: str  # "x_post", "blog_draft", "changelog_entry"


def generate_cycle_summary(cycle: EvolutionCycle) -> str:
    """Generate a human-readable summary of an evolution cycle."""
    parts = [f"Evolution cycle #{cycle.cycle_id}:"]

    if cycle.gathered:
        parts.append(f"Gathered {len(cycle.gathered)} inputs")
    if cycle.evaluated:
        parts.append(f"Evaluated {len(cycle.evaluated)} items")
    if cycle.integrated:
        parts.append(f"Integrated {len(cycle.integrated)} changes")
    if cycle.error:
        parts.append(f"Error: {cycle.error}")

    return ". ".join(parts) + "."


def generate_x_post(cycle: EvolutionCycle, project: str) -> PostContent:
    """Generate an X/Twitter post from evolution cycle results."""
    summary = generate_cycle_summary(cycle)

    # Keep under 280 chars
    body = f"Autonomous evolution cycle #{cycle.cycle_id} for {project}: {summary}"
    if len(body) > 280:
        body = body[:277] + "..."

    return PostContent(
        title=f"Evolution #{cycle.cycle_id}",
        body=body,
        post_type="x_post",
    )


def generate_changelog_entry(cycle: EvolutionCycle) -> PostContent:
    """Generate a changelog entry from evolution results."""
    lines = [f"## Evolution Cycle #{cycle.cycle_id}", ""]
    if cycle.integrated:
        lines.append("### Changes")
        for change in cycle.integrated:
            lines.append(f"- {change}")
    if cycle.gathered:
        lines.append("")
        lines.append(f"*Gathered {len(cycle.gathered)} inputs, "
                     f"evaluated {len(cycle.evaluated)} items*")

    return PostContent(
        title=f"Cycle #{cycle.cycle_id}",
        body="\n".join(lines),
        post_type="changelog_entry",
    )


def save_post(post: PostContent, output_dir: str) -> str:
    """Save a post to the output directory."""
    os.makedirs(output_dir, exist_ok=True)
    timestamp = time.strftime("%Y%m%d-%H%M%S")
    filename = f"{timestamp}-{post.post_type}.md"
    filepath = os.path.join(output_dir, filename)
    with open(filepath, "w") as f:
        f.write(f"# {post.title}\n\n{post.body}\n")
    return filepath
