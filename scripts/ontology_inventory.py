#!/usr/bin/env python3
"""Inventory tracked public ontology artifacts without reading Seed state.

Default output is local stdout. NEVER publish output produced against a
private repository without review: paths and hashes can be sensitive.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ONTOLOGY_EXTENSIONS = {".ttl", ".rdf", ".owl", ".jsonld", ".nt", ".nq", ".trig"}


def inventory(root: Path) -> list[dict[str, object]]:
    root = root.resolve(strict=True)
    result = subprocess.run(
        ["git", "-C", str(root), "ls-files", "-z", "--", "ontology"],
        check=True,
        capture_output=True,
    )
    found = []
    for raw in result.stdout.split(b"\0"):
        if not raw:
            continue
        relative = raw.decode("utf-8", errors="strict")
        path = root / relative
        if path.suffix.lower() not in ONTOLOGY_EXTENSIONS:
            continue
        if path.is_symlink() or not path.is_file() or not path.resolve().is_relative_to(root):
            raise ValueError(f"Unsafe tracked ontology entry: {relative}")
        data = path.read_bytes()
        found.append({
            "path": relative,
            "bytes": len(data),
            "sha256": hashlib.sha256(data).hexdigest(),
        })
    return sorted(found, key=lambda item: str(item["path"]))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        files = inventory(args.root)
    except (OSError, UnicodeError, ValueError, subprocess.CalledProcessError) as exc:
        print(f"Inventory failed: {exc}", file=sys.stderr)
        return 1
    print(json.dumps({
        "schema": "arandu-ontology-inventory/v1",
        "scope": "tracked ontology files only; no canonicality assertion",
        "files": files,
    }, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
