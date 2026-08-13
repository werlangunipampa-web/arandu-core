#!/usr/bin/env bash
set -u

echo "=== ARANDU CORE v0.1.2 — Final Clippy Gate ==="
echo

echo "[1/3] cargo check --workspace"
cargo check --workspace || exit $?

echo
echo "[2/3] cargo test --workspace"
cargo test --workspace || exit $?

echo
echo "[3/3] cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings || exit $?

echo
echo "ARANDU_CORE_0_1_2_CLIPPY_OK=true"
