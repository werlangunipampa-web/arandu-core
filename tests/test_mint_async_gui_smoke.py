#!/usr/bin/env python3
"""Virtual-display smoke for the responsive A1.8 Rust/Tk bridge."""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    if not os.environ.get("DISPLAY"):
        print("Missing DISPLAY: use xvfb-run -a", file=sys.stderr)
        return 2
    env = dict(os.environ, ARANDU_GUI_SMOKE="1")
    try:
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "arandu_window_async_mint.py")],
            cwd=ROOT, env=env, capture_output=True,
            text=True, timeout=30, check=False,
        )
    except subprocess.TimeoutExpired:
        print("A1.8 GUI smoke timed out", file=sys.stderr)
        return 1
    if result.returncode or "ARANDU_GUI_SMOKE_OK" not in result.stdout:
        print("A1.8 GUI smoke failed", file=sys.stderr)
        print(result.stdout[-2000:], file=sys.stderr)
        print(result.stderr[-2000:], file=sys.stderr)
        return 1
    print("ARANDU A1.8 GUI: nonblocking worker + 4 Rust frames + safe close OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
