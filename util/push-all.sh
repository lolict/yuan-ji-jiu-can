#!/bin/bash
# push-all.sh · 把 origin/main 推送到所有镜像远端
# 用法(在 hunyuan-rs 或 hunyuan-os 仓库根):
#   bash util/push-all.sh
#
# 安全约束:
#   - 只推送 main 分支
#   - 只推送 fast-forward(默认);使用 --force 强制覆盖(需手动)
#   - 失败不中断,逐个报告

set -euo pipefail

BRANCH="${1:-main}"
REMOTE_LIST=$(git remote)

echo "=== push-all.sh · branch=$BRANCH ==="
echo "remotes: $REMOTE_LIST"

for remote in $REMOTE_LIST; do
    echo ""
    echo ">>> pushing to $remote ..."
    if git push "$remote" "$BRANCH" 2>&1; then
        echo "    $remote OK"
    else
        echo "    $remote FAILED (non-fast-forward or network)"
        echo "    consider: git push $remote $BRANCH --force-with-lease"
    fi
done

echo ""
echo "=== push-all done ==="
