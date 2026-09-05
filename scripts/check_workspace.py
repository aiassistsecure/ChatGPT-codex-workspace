#!/usr/bin/env python3
"""Validate project catalog paths and minimum project handoff files."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    catalog = json.loads((ROOT / "projects.json").read_text())
    if catalog.get("schema_version") != 1:
        raise SystemExit("Unsupported catalog schema")
    seen = set()
    for project in catalog["projects"]:
        slug = project["id"]
        if slug in seen:
            raise SystemExit(f"Duplicate project id: {slug}")
        seen.add(slug)
        path = (ROOT / project["path"]).resolve()
        if path.parent != ROOT / "prototypes" or path.name != slug:
            raise SystemExit(f"Invalid project path: {project['path']}")
        for required in ("README.md", "AGENTS.md", "docs/implementation-plan.md"):
            if not (path / required).is_file():
                raise SystemExit(f"Missing {project['path']}/{required}")
        if project["stage"] == "scaffold" and project["release_enabled"]:
            raise SystemExit(f"Scaffold cannot enable releases: {slug}")
    print(f"Validated {len(seen)} project(s)")


if __name__ == "__main__":
    main()
