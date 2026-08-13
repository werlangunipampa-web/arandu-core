#!/usr/bin/env bash
set -euo pipefail

echo "=== ARANDU CORE v0.3.0 — Geometria Adaptativa Gate ==="
echo

echo "[1/5] cargo fmt --all"
cargo fmt --all

echo
echo "[2/5] cargo fmt --all -- --check"
cargo fmt --all -- --check

echo
echo "[3/5] cargo check --workspace"
cargo check --workspace

echo
echo "[4/5] cargo test --workspace"
cargo test --workspace

echo
echo "[5/5] cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo
echo "ARANDU_CORE_0_3_0_GEOMETRIA_OK=true"
