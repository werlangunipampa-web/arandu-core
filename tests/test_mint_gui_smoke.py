#!/usr/bin/env python3
"""A1.7 GUI smoke: exercise a real Tk window under an isolated X display.

Use under xvfb-run; no login-session modification, keyboard hooks or Seed data.
"""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    if not os.environ.get("DISPLAY"):
        print("GUI smoke requires a display (use xvfb-run -a)", file=sys.stderr)
        return 2
    env = dict(os.environ, ARANDU_GUI_SMOKE="1")
    try:
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "arandu_window_mint.py")],
            cwd=ROOT,
            env=env,
            capture_output=True,
            text=True,
            timeout=45,
            check=False,
        )
    except subprocess.TimeoutExpired:
        print("GUI smoke timeout: window or backend unresponsive", file=sys.stderr)
        return 1
    if result.returncode != 0 or "ARANDU_GUI_SMOKE_OK" not in result.stdout:
        print("GUI smoke failed.", file=sys.stderr)
        print(result.stdout[-2000:], file=sys.stderr)
        print(result.stderr[-2000:], file=sys.stderr)
        return 1
    print("ARANDU A1.7 GUI: opened, expanded, pinned, collapsed and closed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
