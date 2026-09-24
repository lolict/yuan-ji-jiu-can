#!/usr/bin/env bash
# local-build.sh — 无 Docker 环境的容器构建验证脚本
# ------------------------------------
# 用法:
#   ./scripts/local-build.sh                # 三场景独立验证 (默认)
#   ./scripts/local-build.sh triune         # 只验证 triune
#   ./scripts/local-build.sh bag            # 只验证 bag
#   ./scripts/local-build.sh offline        # 只验证 offline
#   ./scripts/local-build.sh all            # 三场景全跑
#
# 依赖: clang (无 Docker,无 git archive)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTAINER_DIR="$(dirname "$SCRIPT_DIR")"
HUNYUAN_DIR="$(dirname "$CONTAINER_DIR")"
BUILD_DIR="${SCRIPT_DIR}/.build-$$"
SCENARIO="${1:-all}"

cleanup() { rm -rf "$BUILD_DIR"; }
trap cleanup EXIT

echo "━━━ 满全法本地构建验证 ━━━"
echo "源码: ${HUNYUAN_DIR}"
echo "场景: ${SCENARIO}"
echo ""

CC="${CC:-clang}"
CFLAGS="-std=c11 -O2 -D_GNU_SOURCE"

# === 编译 ===
mkdir -p "$BUILD_DIR"
echo "[1/2] 盟机合引擎编译..."

$CC $CFLAGS \
    -I"${HUNYUAN_DIR}/kernel" \
    -I"${HUNYUAN_DIR}/kernel/src" \
    "${CONTAINER_DIR}/entrypoint.c" \
    "${HUNYUAN_DIR}/kernel/src/njsg_mengji.c" \
    -lpthread -lm \
    -o "${BUILD_DIR}/njsg-mengji-demo" 2>&1 | tail -5

echo "[2/2] 里程碑验证..."

run_check() {
    local name="$1"
    local logfile="${BUILD_DIR}/verify-${name}.log"
    echo "  ▸ ${name}"
    MENGJI_SCENARIO="$name" stdbuf -oL "${BUILD_DIR}/njsg-mengji-demo" 2>&1 | tee "$logfile" | grep -E "PASS|FAIL|失败|★" | head -3
    if ! grep -q "PASS" "$logfile"; then
        echo "  ✗ ${name} 未通过"
        return 1
    fi
    echo "  ✓ ${name} PASS"
    return 0
}

case "$SCENARIO" in
    all)
        run_check triune
        run_check bag
        run_check offline
        ;;
    triune|bag|offline)
        run_check "$SCENARIO"
        ;;
    *)
        echo "未知场景: $SCENARIO (可用: triune / bag / offline / all)"
        exit 1
        ;;
esac

echo ""
echo "━━━ 满全法本地验证完成 ━━━"
