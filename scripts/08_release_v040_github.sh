#!/usr/bin/env bash
set -euo pipefail
REPO="werlangunipampa-web/arandu-core"
TAG="v0.4.0"

./scripts/07_validar_v040_renderer.sh
gh auth status

git add .
if ! git diff --cached --quiet; then
  git commit -m "feat: ARANDU CORE v0.4.0 renderer contract"
fi

if ! git rev-parse "$TAG" >/dev/null 2>&1; then
  git tag -a "$TAG" -m "ARANDU CORE v0.4.0 — Renderer Contract"
fi

git push origin main
git push origin "$TAG"
gh repo view "$REPO" --json nameWithOwner,url,visibility,defaultBranchRef
echo
echo "ARANDU_CORE_0_4_0_GITHUB_RELEASE_OK=true"
