#!/usr/bin/env bash
set -u

cd "$(dirname "$0")/.." || exit 1

echo '=== ARANDU CORE v0.1.1 — Clippy Gate ==='
echo

echo '[1/3] cargo check --workspace'
cargo check --workspace || exit 10

echo
echo '[2/3] cargo test --workspace'
cargo test --workspace || exit 20

echo
echo '[3/3] cargo clippy --workspace --all-targets -- -D warnings'
cargo clippy --workspace --all-targets -- -D warnings || exit 30

echo
echo 'ARANDU_CORE_0_1_1_CLIPPY_OK=true'
