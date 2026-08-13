#!/usr/bin/env bash
set -euo pipefail

SRC="$(cd "$(dirname "$0")" && pwd)"
TARGET="$HOME/Downloads/ARANDU-CORE_v0.1.0"
STAMP="$(date +%Y%m%d-%H%M%S)"
BACKUP_ROOT="$HOME/.local/share/arandu-lab/backups/arandu-core"
BACKUP="$BACKUP_ROOT/ARANDU-CORE-v0.1.2-baseline-$STAMP"

if [[ ! -d "$TARGET/crates/arandu-core" ]]; then
  echo "ERRO: baseline não encontrado em $TARGET" >&2
  exit 1
fi

mkdir -p "$BACKUP_ROOT"
cp -a "$TARGET" "$BACKUP"
echo "Backup: $BACKUP"

cp "$SRC/Cargo.toml" "$TARGET/Cargo.toml"
cp "$SRC/README.md" "$TARGET/README.md"
cp "$SRC/CHANGELOG.md" "$TARGET/CHANGELOG.md"
mkdir -p "$TARGET/crates/arandu-confluence/src"
cp "$SRC/crates/arandu-confluence/Cargo.toml" "$TARGET/crates/arandu-confluence/Cargo.toml"
cp "$SRC/crates/arandu-confluence/src/lib.rs" "$TARGET/crates/arandu-confluence/src/lib.rs"
cp "$SRC/crates/arandu-core/Cargo.toml" "$TARGET/crates/arandu-core/Cargo.toml"
cp "$SRC/crates/arandu-core/src/lib.rs" "$TARGET/crates/arandu-core/src/lib.rs"
cp "$SRC/crates/arandu-navigation/src/lib.rs" "$TARGET/crates/arandu-navigation/src/lib.rs"
cp "$SRC/apps/arandu-cli/Cargo.toml" "$TARGET/apps/arandu-cli/Cargo.toml"
cp "$SRC/apps/arandu-cli/src/main.rs" "$TARGET/apps/arandu-cli/src/main.rs"
cp "$SRC/docs/CONFLUENCE_SEED.md" "$TARGET/docs/CONFLUENCE_SEED.md"
cp "$SRC/scripts/05_validar_v020_semente.sh" "$TARGET/scripts/05_validar_v020_semente.sh"
chmod +x "$TARGET/scripts/05_validar_v020_semente.sh"

echo
echo "ARANDU CORE v0.2.0 aplicado sobre o baseline v0.1.2."
echo "Valide com: cd '$TARGET' && ./scripts/05_validar_v020_semente.sh"
