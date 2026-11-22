#!/usr/bin/env bash
# ARANDU A1.7 — autoteste local e CI. Não instala pacotes nem altera sessão Linux.
set -Eeuo pipefail
export LC_ALL=C.UTF-8

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

GUI_MODE=auto
if [[ "${1:-}" == "--require-gui" ]]; then GUI_MODE=require
elif [[ "${1:-}" == "--no-gui" ]]; then GUI_MODE=off
elif [[ -n "${1:-}" ]]; then echo "Uso: bash scripts/testar_tudo.sh [--no-gui|--require-gui]" >&2; exit 2
fi

command -v cargo >/dev/null || { echo "ERRO: cargo ausente" >&2; exit 1; }
command -v python3 >/dev/null || { echo "ERRO: python3 ausente" >&2; exit 1; }
command -v git >/dev/null || { echo "ERRO: git ausente" >&2; exit 1; }

LOG_DIR="$(mktemp -d "${TMPDIR:-/tmp}/arandu-autoteste.XXXXXXXX")"
chmod 700 "$LOG_DIR"
LAST_STEP="início"
report_failure() {
  local code=$?
  printf '\n[FALHOU] %s (código %s). Logs locais: %s\n' "$LAST_STEP" "$code" "$LOG_DIR" >&2
  exit "$code"
}
trap report_failure ERR

step() {
  LAST_STEP="$1"; shift
  printf '\n=== %s ===\n' "$LAST_STEP"
  "$@" 2>&1 | tee "$LOG_DIR/$(printf '%s' "$LAST_STEP" | tr ' /' '__').log"
}

step "versoes" bash -c 'rustc --version; cargo --version; python3 --version; git --version'
step "rustfmt" cargo fmt --all -- --check
step "cargo_check" cargo check --workspace --locked
step "cargo_test" cargo test --workspace --locked
step "clippy" cargo clippy --workspace --all-targets --locked -- -D warnings
step "python_syntax" python3 -c 'import ast,pathlib; files=sorted(pathlib.Path("scripts").glob("*.py"))+sorted(pathlib.Path("tests").glob("*.py")); [ast.parse(p.read_text(encoding="utf8"), filename=str(p)) for p in files]; print(f"Python sintaxe OK: {len(files)} arquivos")'
step "cargo_build_cli" cargo build --locked -p arandu-cli
step "python_unit" python3 -m unittest discover -s tests -p 'test_*.py' -v
step "preview_protocol" python3 tests/test_visual_smoke.py
step "benchmark" python3 scripts/benchmark_window_protocol.py --frames 100

if [[ "$GUI_MODE" != off ]]; then
  if command -v xvfb-run >/dev/null 2>&1 && python3 -c 'import tkinter' >/dev/null 2>&1; then
    step "janela_virtual" xvfb-run -a python3 tests/test_mint_gui_smoke.py
    step "janela_async_virtual" xvfb-run -a python3 tests/test_mint_async_gui_smoke.py
  elif [[ "$GUI_MODE" == require ]]; then
    echo "ERRO: --require-gui requer xvfb-run e Python tkinter." >&2
    exit 1
  else
    printf '\n[SKIP] Janela virtual: requer xvfb-run e tkinter; testes de núcleo e protocolo continuam válidos.\n'
  fi
fi

printf '\n[OK] AUTOTESTE CONCLUÍDO. Logs: %s\n' "$LOG_DIR"
printf 'Não foram instalados pacotes, criadas sessões nem acessada a Semente oficial.\n'
