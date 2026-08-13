#!/usr/bin/env bash
set -euo pipefail
echo "=== ARANDU CORE v0.4.2 — Renderer Final Clippy Gate ==="
cargo fmt --all
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
echo
echo "ARANDU_CORE_0_4_2_RENDERER_OK=true"
