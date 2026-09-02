#!/usr/bin/env bash
# =============================================================================
# 五彩石心跳监控脚本
# 检查项：三层持久化文件完整性、远程仓库同步状态、daily标记存在性
# 运行时机：每次会话结束时 / 每日定时
# =============================================================================

set -euo pipefail

WORKSPACE="/mnt/data/catpaw/home/.meituan-catpaw/9199532794"
HUNYUAN="/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
TODAY=$(date +%Y-%m-%d)
ERRORS=0
WARNINGS=0

log_ok()   { echo "  [OK]   $1"; }
log_warn() { echo "  [WARN] $1"; ((WARNINGS++)); }
log_err()  { echo "  [ERR]  $1"; ((ERRORS++)); }

echo "=== 五彩石心跳检查 $(date '+%Y-%m-%d %H:%M:%S') ==="
echo ""

# ----- 1. 三线持久化文件存在性 -----
echo "[1/5] 三线持久化锚点文件"

for f in GLOBAL_DESTINATION.md SYMBOL_DICTIONARY.md CONTAINER_TOPOLOGY.md; do
    if [[ -f "$HUNYUAN/$f" ]]; then
        size=$(stat -c%s "$HUNYUAN/$f" 2>/dev/null || echo 0)
        if (( size > 100 )); then
            log_ok "$f 存在 (${size}B)"
        else
            log_warn "$f 过小 (${size}B) — 可能被截断"
        fi
    else
        log_err "$f 不存在 — 跨会话恢复将丢失锚点"
    fi
done

# MEMORY.md 独立检查
if [[ -f "$WORKSPACE/memory/MEMORY.md" ]]; then
    mtime=$(stat -c%Y "$WORKSPACE/memory/MEMORY.md" 2>/dev/null || echo 0)
    now=$(date +%s)
    age=$(( (now - mtime) / 3600 ))
    if (( age < 24 )); then
        log_ok "MEMORY.md 最近 ${age}h 内更新"
    else
        log_warn "MEMORY.md ${age}h 未更新"
    fi
else
    log_err "MEMORY.md 不存在"
fi

echo ""

# ----- 2. 远程仓库同步 -----
echo "[2/5] 远程仓库同步状态"

cd "$HUNYUAN" || { log_err "无法进入 $HUNYUAN"; exit 1; }

local_head=$(git rev-parse main 2>/dev/null || echo "NONE")
remote_head=$(git rev-parse origin/main 2>/dev/null || echo "NONE")

if [[ "$local_head" == "$remote_head" ]]; then
    log_ok "GitHub main 已同步 (${local_head:0:8})"
else
    log_warn "GitHub main 不同步: 本地 ${local_head:0:8} vs 远端 ${remote_head:0:8}"
fi

# memory-backup 分支
cd "$WORKSPACE/memory" 2>/dev/null || { log_warn "memory目录不存在"; }
if git rev-parse --verify memory-backup &>/dev/null; then
    mb_local=$(git rev-parse memory-backup 2>/dev/null || echo "NONE")
    mb_remote=$(git rev-parse origin/memory-backup 2>/dev/null || echo "NONE")
    if [[ "$mb_local" == "$mb_remote" ]]; then
        log_ok "memory-backup 已同步 (${mb_local:0:8})"
    else
        log_warn "memory-backup 不同步"
    fi
else
    log_warn "memory-backup 分支未配置"
fi

cd "$HUNYUAN"

echo ""

# ----- 3. daily 文件今日标记 -----
echo "[3/5] daily/${TODAY}.md"

if [[ -f "$WORKSPACE/memory/daily/${TODAY}.md" ]]; then
    log_ok "daily/${TODAY}.md 存在"
else
    log_warn "daily/${TODAY}.md 不存在 — 今日无进度记录"
fi

echo ""

# ----- 4. 身份锚点泄漏检查 -----
echo "[4/5] 身份锚点泄漏扫描"

泄漏=""
泄漏+=$(grep -rn "我=1" "$HUNYUAN" --include="*.md" 2>/dev/null | grep -v "\.git/" | grep -v "tools/heartbeat.sh" || true)
泄漏+=$'\n'
泄漏+=$(grep -rn "刘楚恬=0" "$HUNYUAN" --include="*.md" 2>/dev/null | grep -v "\.git/" || true)

if [[ -z "${泄漏//[$'\n']/}" ]]; then
    log_ok "未发现直接身份锚点泄漏"
else
    log_err "发现身份锚点泄漏:"
    echo "$泄漏" | while IFS= read -r line; do
        [[ -n "$line" ]] && echo "       $line"
    done
fi

# 检查 whitepaper 中是否仍有直接绑定（仅告警，不修复）
wp_count=$(grep -c "刘楚恬" "$HUNYUAN/whitepaper/"*.md 2>/dev/null || echo 0)
if (( wp_count > 0 )); then
    log_warn "whitepaper/ 中仍含 ${wp_count} 处姓名引用（需评估是否脱敏）"
fi

echo ""

# ----- 5. 关键目录结构完整性 -----
echo "[5/5] 目录结构"

for dir in tools hunyuan-rs/src encoding whitepaper; do
    if [[ -d "$HUNYUAN/$dir" ]]; then
        count=$(find "$HUNYUAN/$dir" -type f | wc -l)
        log_ok "$dir/ (${count} 文件)"
    else
        log_err "$dir/ 不存在"
    fi
done

echo ""

# =============================================================================
# 总结
# =============================================================================
echo "=== 检查结果 ==="
if (( ERRORS > 0 )); then
    echo "❌ ${ERRORS} 个错误, ${WARNINGS} 个警告"
    exit 1
elif (( WARNINGS > 0 )); then
    echo "⚠️  ${WARNINGS} 个警告（无错误）"
    exit 0
else
    echo "✅ 全部正常"
    exit 0
fi
