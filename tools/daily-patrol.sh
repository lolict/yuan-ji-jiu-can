#!/usr/bin/env bash
# daily-patrol.sh — 每日巡守: fetch + 三端比对 + 测试 + memory-backup 推送
# 由 cron 在 02:00 触发, 不创建 CatPaw 新会话

trap "" SIGPIPE
set -uo pipefail

export PATH="/mnt/data/catpaw/home/.cargo/bin:$PATH"

WORKSPACE="/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
MEMDIR="/mnt/data/catpaw/home/.meituan-catpaw/9199532794/memory/daily"
DATE="$(date +%Y-%m-%d)"
LOGFILE="${MEMDIR}/${DATE}-patrol.log"

mkdir -p "$MEMDIR"
cd "$WORKSPACE"

{
    echo "═══════════════════════════════════════════"
    echo "NJSG 巡守 · ${DATE} $(date +%H:%M:%S)"
    echo "═══════════════════════════════════════════"

    # 1. gitcode remote 检查
    GITCODE_URL=$(git remote get-url gitcode 2>/dev/null || echo "")
    if [[ "$GITCODE_URL" != https://* ]]; then
        echo "[WARN] gitcode remote 不是 HTTPS token 地址, 尝试恢复..."
        git remote set-url gitcode "https://lolict:Csqp3bfteRyNqoRTgLRqPAry@gitcode.com/lolict/hunyuan-os.git"
    else
        echo "[OK] gitcode remote HTTPS token 正常"
    fi

    # 2. fetch --all
    echo ""
    echo "--- git fetch --all --prune ---"
    git fetch --all --prune 2>&1 | grep -v "^$" || true

    # 3. 三端比对
    echo ""
    echo "--- 三端 HEAD 比对 ---"
    LOCAL=$(git rev-parse HEAD)
    ORIGIN=$(git rev-parse origin/main 2>/dev/null || echo "UNREACHABLE")
    GITEE=$(git rev-parse gitee-hunyuan/main 2>/dev/null || echo "UNREACHABLE")
    GITCODE=$(git rev-parse gitcode/main 2>/dev/null || echo "UNREACHABLE")
    echo "local:          $LOCAL"
    echo "origin/main:     $ORIGIN"
    echo "gitee-hunyuan:   $GITEE"
    echo "gitcode/main:    $GITCODE"

    for remote_ref in origin/main gitee-hunyuan/main gitcode/main; do
        ref_val=$(git rev-parse "$remote_ref" 2>/dev/null || echo "UNREACHABLE")
        if [ "$ref_val" = "UNREACHABLE" ]; then
            echo "[FAIL] $remote_ref 不可达"
        elif [ "$ref_val" = "$LOCAL" ]; then
            echo "[OK] $remote_ref 齐平"
        else
            ahead_behind=$(git rev-list --left-right --count "${remote_ref}..." 2>/dev/null | tr '\t' '/')
            echo "[WARN] $remote_ref 不齐平 (ahead/behind: $ahead_behind)"
        fi
    done

    # 4. cargo test --lib
    echo ""
    echo "--- cargo test --lib --no-fail-fast ---"
    TEST_OUTPUT=$(cargo test --lib --no-fail-fast 2>&1)
    TEST_EXIT=$?
    PASS=$(echo "$TEST_OUTPUT" | grep -oP '\d+ passed' | tail -1 || echo "unknown")
    FAIL=$(echo "$TEST_OUTPUT" | grep -oP '\d+ failed' | tail -1 || echo "0 failed")
    echo "exit_code: $TEST_EXIT"
    echo "$PASS | $FAIL"

    if [ "$TEST_EXIT" -ne 0 ]; then
        echo ""
        echo "--- 失败用例 ---"
        echo "$TEST_OUTPUT" | grep '^    ' | grep '::' | head -10
    fi

    # 5. memory-backup 推送
    echo ""
    echo "--- git push origin memory-backup ---"
    git push origin memory-backup 2>&1 || echo "[WARN] memory-backup push failed"

    echo ""
    echo "巡守完成 $(date +%H:%M:%S)"

} >> "$LOGFILE" 2>&1

echo "[patrol $(date +%H:%M)] done, log: $LOGFILE"
