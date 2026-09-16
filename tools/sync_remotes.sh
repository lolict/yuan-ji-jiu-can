#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════════════════════
# sync_remotes.sh — 安全的多 remote 同步脚本
#
# 策略:
#   1. 遍历所有 remote, 逐一尝试 push
#   2. 对于 non-fast-forward 的 remote:
#      a. 如果 merge-base 存在 (同源但分叉): 尝试 pull --rebase 后 push
#      b. 如果 merge-base 不存在 (完全不同项目): 跳过并记录警告
#   3. 对于不可达的 remote: 跳过不阻塞
#
# 用法: bash tools/sync_remotes.sh [--force-unrelated]
#   --force-unrelated: 对无共同历史的 remote 也 force-push (危险, 会覆盖远程)
# ══════════════════════════════════════════════════════════════════════════

set -uo pipefail

cd "$(dirname "$0")/.."

FORCE_UNRELATED=false
[[ "${1:-}" == "--force-unrelated" ]] && FORCE_UNRELATED=true

BRANCH="main"

echo "sync start - branch: $BRANCH, remotes: $(git remote | wc -l)"
echo ""

SUCCESS=()
FAILED=()
SKIPPED=()

for REMOTE in $(git remote); do
    echo "--- $REMOTE ---"

    URL=$(git remote get-url "$REMOTE" 2>/dev/null || echo "unknown")
    echo "  REMOTE: $REMOTE"

    # Fetch (with timeout, non-blocking)
    if ! timeout 15 git fetch "$REMOTE" "$BRANCH" 2>/dev/null; then
        echo "  WARN fetch failed or skipped"
        SKIPPED+=("$REMOTE|fetch_failed")
        continue
    fi

    LOCAL_REF=$(git rev-parse "$BRANCH" 2>/dev/null || echo "")
    REMOTE_REF=$(git rev-parse "$REMOTE/$BRANCH" 2>/dev/null || echo "")

    if [ -z "$LOCAL_REF" ] || [ -z "$REMOTE_REF" ]; then
        echo "  WARN branch not found, skip"
        SKIPPED+=("$REMOTE|no_branch")
        continue
    fi

    # Already in sync
    if [ "$LOCAL_REF" = "$REMOTE_REF" ]; then
        echo "  OK already up-to-date"
        SUCCESS+=("$REMOTE|uptodate")
        continue
    fi

    # Check fast-forward
    if git merge-base --is-ancestor "$REMOTE_REF" "$LOCAL_REF" 2>/dev/null; then
        if git push "$REMOTE" "$BRANCH" 2>&1; then
            echo "  OK pushed (fast-forward)"
            SUCCESS+=("$REMOTE|pushed")
        else
            echo "  FAIL push failed"
            FAILED+=("$REMOTE|push_failed")
        fi
        continue
    fi

    # Check merge-base
    MERGE_BASE=$(git merge-base "$BRANCH" "$REMOTE/$BRANCH" 2>/dev/null || echo "")

    if [ -z "$MERGE_BASE" ]; then
        # No common history - completely unrelated project
        if $FORCE_UNRELATED; then
            echo "  WARN no common history, force-pushing..."
            if git push --force "$REMOTE" "$BRANCH" 2>&1; then
                echo "  OK force-pushed"
                SUCCESS+=("$REMOTE|force_pushed")
            else
                echo "  FAIL force-push failed"
                FAILED+=("$REMOTE|force_push_failed")
            fi
        else
            echo "  SKIP no common history (unrelated project)"
            SKIPPED+=("$REMOTE|unrelated_history")
        fi
        continue
    fi

    # Common ancestor but diverged - try rebase then push
    echo "  MERGE rebased and pushing..."
    # Stash any local changes to allow rebase
    STASHED=false
    if ! git diff --quiet HEAD 2>/dev/null; then
        echo "  STASH local changes for rebase"
        git stash push -m "sync_remotes_auto_stash_$(date +%s)" 2>/dev/null && STASHED=true
    fi
    if git pull --rebase "$REMOTE" "$BRANCH" 2>&1; then
        if git push "$REMOTE" "$BRANCH" 2>&1; then
            echo "  OK rebased & pushed"
            SUCCESS+=("$REMOTE|rebased_pushed")
        else
            echo "  FAIL push after rebase failed"
            FAILED+=("$REMOTE|push_after_rebase_failed")
            git rebase --abort 2>/dev/null || true
        fi
    else
        echo "  FAIL rebase conflict, needs manual fix"
        git rebase --abort 2>/dev/null || true
        FAILED+=("$REMOTE|rebase_conflict")
    fi
    # Restore stashed changes
    if $STASHED; then
        git stash pop 2>/dev/null || echo "  WARN stash pop failed"
    fi
done

# Summary
echo ""
echo "================================"
echo "sync complete"
echo "  success: ${#SUCCESS[@]}"
echo "  failed:  ${#FAILED[@]}"
echo "  skipped: ${#SKIPPED[@]}"
echo "================================"

if [ ${#FAILED[@]} -gt 0 ]; then
    echo ""
    echo "WARN remotes:"
    for entry in "${FAILED[@]}"; do
        echo "  $entry"
    done
    # Only count push failures as fatal, rebase_conflict is resolvable
    FATAL=0
    for entry in "${FAILED[@]}"; do
        case "$entry" in
            *|push_failed|*|fetch_failed|*|force_push_failed) FATAL=$((FATAL+1)) ;;
        esac
    done
    if [ $FATAL -gt 0 ]; then
        exit 1
    fi
    echo "  (no fatal errors, continuing)"
fi

exit 0
