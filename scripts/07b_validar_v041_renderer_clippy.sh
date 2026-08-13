#!/usr/bin/env bash
set -euo pipefail
echo "=== ARANDU CORE v0.4.1 — Renderer Clippy Gate ==="
cargo fmt --all
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
echo
echo "ARANDU_CORE_0_4_1_RENDERER_CLIPPY_OK=true"
