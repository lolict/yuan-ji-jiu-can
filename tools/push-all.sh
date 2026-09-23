#!/usr/bin/env bash
# push-all.sh — 全仓库全平台推送
# 用法: bash tools/push-all.sh [branch]
# 默认推当前分支; 传入参数则推指定分支

set -euo pipefail

BRANCH="${1:-$(git symbolic-ref --short HEAD 2>/dev/null || echo main)}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo ">>> push-all: branch=$BRANCH"
echo ">>> $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 颜色
RED='\033[0;31m'
GRN='\033[0;32m'
YLW='\033[1;33m'
NC='\033[0m'

ok=0
fail=0

for remote in $(git remote); do
  printf "→ %-20s " "$remote"
  if git push "$remote" "$BRANCH" --quiet 2>/dev/null; then
    echo -e "${GRN}✓${NC}"
    ((ok++))
  else
    echo -e "${RED}✗${NC}"
    ((fail++))
  fi
done

echo ""
echo "=============================="
echo -e "成功: ${GRN}$ok${NC}  失败: ${RED}$fail${NC}"
echo "=============================="
[[ $fail -eq 0 ]] && exit 0 || exit 1
