#!/usr/bin/env bash
set -u

HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-$HOME/Downloads/ARANDU-CORE_v0.1.0}"
BACKUP_ROOT="$HOME/.local/share/arandu-lab/backups/arandu-core"
STAMP="$(date +%Y%m%d-%H%M%S)"

if [[ ! -f "$TARGET/Cargo.toml" ]]; then
  echo "ERRO: workspace alvo não encontrado em: $TARGET" >&2
  exit 2
fi

mkdir -p "$BACKUP_ROOT"
BACKUP="$BACKUP_ROOT/ARANDU-CORE-pre-v0.1.2-$STAMP"
cp -a "$TARGET" "$BACKUP"
echo "Backup: $BACKUP"

for rel in \
  Cargo.toml \
  CHANGELOG.md \
  crates/arandu-platform-api/src/lib.rs \
  crates/arandu-navigation/src/lib.rs \
  crates/arandu-core/src/lib.rs \
  apps/arandu-cli/src/main.rs \
  scripts/04_validar_clippy_v012.sh; do
  mkdir -p "$TARGET/$(dirname "$rel")"
  cp -a "$HERE/$rel" "$TARGET/$rel"
done

echo
echo "Patch v0.1.2 aplicado."
echo "Valide com: cd '$TARGET' && ./scripts/04_validar_clippy_v012.sh"
