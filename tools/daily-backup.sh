#!/usr/bin/env bash
# daily-backup.sh — 每日备份: git add/commit + sync_remotes + memory-backup
# 由 cron 在 23:30 触发, 不创建 CatPaw 新会话

set -uo pipefail

WORKSPACE="/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
MEMDIR="/mnt/data/catpaw/home/.meituan-catpaw/9199532794/memory/daily"
DATE="$(date +%Y-%m-%d)"
LOGFILE="${MEMDIR}/${DATE}-backup.log"

mkdir -p "$MEMDIR"
cd "$WORKSPACE"

{
    echo "═══════════════════════════════════════════"
    echo "每日备份 · ${DATE} $(date +%H:%M:%S)"
    echo "═══════════════════════════════════════════"

    # 1. git add + commit (仅当有变动时)
    echo "--- git add & commit ---"
    git add -A
    if git diff --cached --quiet; then
        echo "[OK] 无变动, 跳过提交"
    else
        git commit -m "backup: ${DATE}" && echo "[OK] 提交成功: backup: ${DATE}"
    fi

    # 2. sync_remotes
    echo ""
    echo "--- sync_remotes.sh ---"
    bash tools/sync_remotes.sh 2>&1 || echo "[WARN] sync_remotes 部分失败"

    # 3. memory-backup 分支推送
    echo ""
    echo "--- git push origin memory-backup ---"
    git push origin memory-backup 2>&1 || echo "[WARN] memory-backup push failed"

    echo ""
    echo "备份完成 $(date +%H:%M:%S)"

} >> "$LOGFILE" 2>&1

echo "[backup $(date +%H:%M)] done, log: $LOGFILE"
