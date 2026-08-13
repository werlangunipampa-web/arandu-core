#!/usr/bin/env bash
set -euo pipefail
REPO="werlangunipampa-web/arandu-core"
TAG="v0.4.1"

./scripts/07b_validar_v041_renderer_clippy.sh
gh auth status

git add .
if ! git diff --cached --quiet; then
  git commit -m "fix: ARANDU CORE v0.4.1 renderer clippy gate"
fi

if ! git rev-parse "$TAG" >/dev/null 2>&1; then
  git tag -a "$TAG" -m "ARANDU CORE v0.4.1 — Renderer Contract Clippy Fix"
fi

git push origin main
git push origin "$TAG"
gh repo view "$REPO" --json nameWithOwner,url,visibility,defaultBranchRef
echo
echo "ARANDU_CORE_0_4_1_GITHUB_RELEASE_OK=true"
