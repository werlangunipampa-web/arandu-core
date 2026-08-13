#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo '=== ARANDU CORE 0.1 — validação ==='
command -v cargo >/dev/null || {
  echo 'ERRO: cargo não encontrado. Instale Rust com rustup antes de validar.' >&2
  exit 1
}

cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

echo 'ARANDU_CORE_0_1_OK=true'
