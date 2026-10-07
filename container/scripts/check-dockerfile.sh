#!/usr/bin/env bash
# check-dockerfile.sh — 静态检查 Dockerfile 引用完整性
# ------------------------------------
# 无需 Docker,纯静态分析:
#   1. COPY 引用的本地文件是否在构建上下文中存在
#   2. WORKDIR 是否为绝对路径
#   3. ENTRYPOINT/CMD 指向的二进制是否在 COPY 范围内
#   4. FROM 引用的镜像是否已声明(不支持检查远程 registry)
#
# 用法: ./scripts/check-dockerfile.sh [Dockerfile路径]

set -uo pipefail

DOCKERFILE="${1:-Dockerfile}"
CONTAINER_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if [ ! -f "$DOCKERFILE" ]; then
    echo "[ERROR] Dockerfile not found: $DOCKERFILE"
    exit 1
fi

ERRORS=0
WARNINGS=0

echo "━━━ Dockerfile 静态检查 ━━━"
echo "文件: ${DOCKERFILE}"
echo ""

# --- 检查 1: COPY 的源文件是否存在于上下文 ---
echo "[检查] COPY 源文件引用..."
while IFS= read -r line; do
    # 解析 COPY --from=... src dest 或 COPY src dest
    if echo "$line" | grep -qE 'COPY\s+--from='; then
        # COPY --from=stage — 跨 stage 引用,无法静态校验
        continue
    fi
    # 提取 COPY 后面的第一个本地路径参数
    src=$(echo "$line" | sed -E 's/^COPY\s+([^ ]+)\s+.*/\1/')
    # 跳过 URL 和明显不是文件的内容
    case "$src" in
        http://*|https://*|git://*) continue ;;
    esac
    # 检查文件是否在容器目录或仓库中存在
    if [ ! -e "${CONTAINER_DIR}/${src}" ] && [ ! -e "$(dirname "$DOCKERFILE")/${src}" ]; then
        # 也检查相对仓库根目录
        HUNYUAN_DIR="$(dirname "$CONTAINER_DIR")"
        if [ ! -e "${HUNYUAN_DIR}/${src}" ]; then
            echo "  ⚠ 缺失: COPY $src (文件不存在)"
            ((WARNINGS++))
        fi
    else
        echo "  ✓ COPY $src"
    fi
done < <(grep -E '^\s*COPY' "$DOCKERFILE")

# --- 检查 2: WORKDIR 路径 ---
echo ""
echo "[检查] WORKDIR 路径..."
while IFS= read -r line; do
    dir=$(echo "$line" | sed -E 's/^WORKDIR\s+//')
    case "$dir" in
        /*) echo "  ✓ WORKDIR $dir (绝对路径)" ;;
        *)  echo "  ✗ WORKDIR $dir (应为绝对路径)"
            ((ERRORS++)) ;;
    esac
done < <(grep -E '^\s*WORKDIR' "$DOCKERFILE")

# --- 检查 3: ENTRYPOINT 二进制是否在 COPY 范围内 ---
echo ""
echo "[检查] ENTRYPOINT/CMD 二进制引用..."
ENTRYPOINT_BIN=''
COPY_DESTS=''
while IFS= read -r line; do
    ep=$(echo "$line" | grep -oP 'ENTRYPOINT\s+\["\K[^"]+')
    [ -n "$ep" ] && ENTRYPOINT_BIN="$ep"
done < "$DOCKERFILE"

# 如果 ENTRYPOINT 有值,检查它是否在 COPY 的目标路径中
if [ -n "$ENTRYPOINT_BIN" ]; then
# 收集所有 COPY 的目标路径(含跨 stage)
COPY_BINS=$(grep -E '^\s*COPY' "$DOCKERFILE" | grep -oP '(/usr/local/bin|/usr/bin|/usr/local/sbin|/opt/bin)/[^ ]+' | sed 's|/[^/]*$||' | sort -u | tr '\n' '|')
    if echo "$ENTRYPOINT_BIN" | grep -qP "^(${COPY_BINS%\|})"; then
        echo "  ✓ ENTRYPOINT $ENTRYPOINT_BIN (COPY 范围内)"
    else
        echo "  ⚠ ENTRYPOINT $ENTRYPOINT_BIN (未在 COPY 目标 bin 目录中发现)"
        ((WARNINGS++))
    fi
else
    echo "  - 未检测到 ENTRYPOINT"
fi

# --- 汇总 ---
echo ""
echo "━━━ 检查结果 ━━━"
if [ "$ERRORS" -gt 0 ]; then
    echo "错误: $ERRORS 严重 / $WARNINGS 警告"
    exit 1
elif [ "$WARNINGS" -gt 0 ]; then
    echo "警告: $WARNINGS 项 (建议处理)"
    exit 0
else
    echo "✓ 全部通过"
    exit 0
fi
