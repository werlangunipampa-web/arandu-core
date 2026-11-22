#!/usr/bin/env python3
"""A1.6: repeatable subprocess smoke and latency benchmark for local Rust frames.

No GUI session or Seed data is involved. Numbers measure end-to-end batch
throughput (including process launch and pipes), not input-to-photon latency.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target" / "debug" / "arandu-cli"


def benchmark(frames: int, build: bool) -> dict[str, object]:
    if frames < 1 or frames > 10000:
        raise ValueError("frames must be between 1 and 10000")
    if build:
        subprocess.run(
            ["cargo", "build", "--locked", "-p", "arandu-cli"],
            cwd=ROOT,
            check=True,
        )
    if not BIN.is_file():
        raise FileNotFoundError("arandu-cli binary absent: run with --build")
    requests = ("2\n1\n" * (frames // 2)) + ("2\n" if frames % 2 else "")
    requests += "quit\n"
    start = time.perf_counter()
    result = subprocess.run(
        [str(BIN), "--window-protocol"],
        cwd=ROOT,
        input=requests,
        text=True,
        capture_output=True,
        check=True,
        timeout=30,
    )
    elapsed = time.perf_counter() - start
    lines = result.stdout.splitlines()
    expected = frames + 1  # includes initial collapsed snapshot
    if len(lines) != expected or not all(
        line.startswith("SCENE <svg ") and line.endswith("</svg>") for line in lines
    ):
        raise ValueError(
            f"Invalid protocol: expected {expected} complete frames, got {len(lines)}"
        )
    return {
        "schema": "arandu-window-benchmark/v1",
        "data": "synthetic",
        "frames_requested": frames,
        "frames_received": len(lines),
        "total_seconds_including_startup": round(elapsed, 6),
        "batch_frames_per_second": round(len(lines) / elapsed, 2),
        "note": "Batch protocol throughput only; NOT graphics FPS or input latency",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frames", type=int, default=100)
    parser.add_argument("--build", action="store_true")
    args = parser.parse_args()
    try:
        output = benchmark(args.frames, args.build)
    except (ValueError, OSError, subprocess.SubprocessError) as exc:
        print(f"Benchmark failed: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(output, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
