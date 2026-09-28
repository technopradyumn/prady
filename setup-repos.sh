#!/usr/bin/env bash
# setup-repos.sh — Run this ONCE on a machine that has git installed.
# Creates three separate git repos from the current monorepo folder.
# Usage: cd "Programming Language" && bash setup-repos.sh
# Prerequisites: git, GitHub account https://github.com/technopradyumn
set -euo pipefail

ROOT="$(pwd)"
PARENT="$(dirname "$ROOT")"
DEST="$PARENT/technopradyumn-prady"

echo "==> Creating repos staging directory..."
mkdir -p "$DEST"

# ─────────────────────────────────────────────────────────────
# 1. prady — compiler, CLI, language core
# ─────────────────────────────────────────────────────────────
echo ""
echo "==> Setting up: prady (compiler repo)"
cp -r "$ROOT" "$DEST/prady"
cd "$DEST/prady"
# Remove things that belong in other repos or should not be committed
rm -rf target editors/.github website/.github docs editors website STEPS.MD
# Keep .gitignore (already created), .github/workflows (ci + release)
git init -b main
git add .
git commit -m "feat: initial import — lexer, AST, parser, diagnostics, CLI (Phase 0)"
echo "  Created: $DEST/prady"
echo "  Next: git remote add origin https://github.com/technopradyumn/prady.git && git push -u origin main"

# ─────────────────────────────────────────────────────────────
# 2. vscode-prady — VS Code extension
# ─────────────────────────────────────────────────────────────
echo ""
echo "==> Setting up: vscode-prady (extension repo)"
cp -r "$ROOT/editors/vscode-prady" "$DEST/vscode-prady"
cd "$DEST/vscode-prady"
# Move the workflow from nested .github/ into the repo root
if [ -d ".github" ]; then
  mv .github/workflows/extension-release.yml /tmp/_ext_workflow.yml
  rm -rf .github
  mkdir -p .github/workflows
  mv /tmp/_ext_workflow.yml .github/workflows/extension-release.yml
fi
cat > .gitignore << 'EOF'
node_modules/
*.vsix
out/
.vscode-test/
EOF
cp "$ROOT/LICENSE" . 2>/dev/null || true
git init -b main
git add .
git commit -m "feat: initial import — syntax highlighting, snippets, problem matchers for .pr files"
echo "  Created: $DEST/vscode-prady"
echo "  Next: git remote add origin https://github.com/technopradyumn/vscode-prady.git && git push -u origin main"

# ─────────────────────────────────────────────────────────────
# 3. prady-website — static site + playground
# ─────────────────────────────────────────────────────────────
echo ""
echo "==> Setting up: prady-website (website repo)"
# Use the website/ folder (already separated from docs/)
SRC="$ROOT/website"
if [ ! -d "$SRC" ] || [ -z "$(ls -A "$SRC")" ]; then
  # Fall back to docs/ if website/ is empty
  SRC="$ROOT/docs"
fi
cp -r "$SRC" "$DEST/prady-website"
cd "$DEST/prady-website"
cp "$ROOT/LICENSE" . 2>/dev/null || true
cat > .gitignore << 'EOF'
.vercel/
node_modules/
EOF
git init -b main
git add .
git commit -m "feat: initial import — static website with playground and install guide"
echo "  Created: $DEST/prady-website"
echo "  Next: git remote add origin https://github.com/technopradyumn/prady-website.git && git push -u origin main"
echo "  Then:   Import at https://vercel.com/new — Root Directory: . , Framework: Other"

echo ""
echo "==> Done. Three repos created in: $DEST"
echo ""
echo "Summary of GitHub steps:"
echo "  1. On your account https://github.com/technopradyumn:"
echo "  2. Create 3 new public repositories at https://github.com/new :"
echo "     - prady"
echo "     - vscode-prady"
echo "     - prady-website"
echo "     (Leave 'Add a README' unchecked)"
echo "  3. Push each repo using the 'Next:' commands above."
echo "  4. Add secrets in each repo's Settings → Secrets:"
echo "     prady:         (none needed for CI; GitHub token is built-in)"
echo "     vscode-prady:  VSCE_PAT, OVSX_PAT"
echo "     prady-website: VERCEL_TOKEN, VERCEL_ORG_ID, VERCEL_PROJECT_ID"
