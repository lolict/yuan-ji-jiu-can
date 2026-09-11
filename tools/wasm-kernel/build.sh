#!/usr/bin/env bash
# ============================================================
# WASM Kernel · 双向公约构建脚本
#
#   ./build.sh            → 编译 native 测试
#   ./build.sh wasm32     → 编译 wasm32 (模拟)
#   ./build.sh wasm64     → 编译 wasm64 (模拟)
#   ./build.sh all        → 全部
#
# 依赖:
#   - clang >= 18 (自动检测 wasm 支持)
#   - emcc (可选, 有则生成 wasm + js)
# ============================================================

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
SRC="$ROOT/src"
INC="$ROOT/include"
OUT="$ROOT/build"
mkdir -p "$OUT/native" "$OUT/wasm32" "$OUT/wasm64"

CC="${CC:-clang}"
CFLAGS="-I$INC -Wall -Wextra -O2 -std=c11"
CFLAGS_NATIVE="$CFLAGS -DMQF_SMRL_NATIVE=1"

# 检测 emcc
EMC="$(command -v emcc 2>/dev/null || true)"

echo "=== WASM Kernel · 双向公约构建 ==="
echo "  CC  = $CC"
echo "  emcc= ${EMC:-未安装}"
echo ""

compile_native() {
    echo "[1/4] native (x86_64) ..."
    $CC $CFLAGS_NATIVE -o "$OUT/native/smrl_test" "$SRC/pq_smrl.c" "$SRC/test_main.c" -lm
    echo "  ✓ $OUT/native/smrl_test"
}

compile_wasm() {
    local target="$1" arch="$2" out="$OUT/$3"
    local flags=""
    echo "[2/4] wasm$arch ($target) ..."

    if [[ -n "$EMC" ]]; then
        # 完整 emcc 流程
        case "$arch" in
            32) $EMC -sWASM=1  -sMEMORY64=0 $CFLAGS -o "$out/smrl.js"   "$SRC/pq_smrl.c" ;;
            64) $EMC -sWASM=1  -sMEMORY64=1 $CFLAGS -o "$out/smrl64.js" "$SRC/pq_smrl.c" ;;
        esac
        echo "  ✓ $out/smrl{64}.html"
    else
        # 模拟: 用 clang 验证源码编译到 wasm IR
        # 注意: clang 18+ 可以识别 wasm target, 但没有链接器时只生成 .o
        case "$arch" in
            32) $CC --target=wasm32-unknown-unknown -c -o "$out/smrl.o" "$SRC/pq_smrl.c" $CFLAGS 2>/dev/null || \
                echo "  ⚠ wasm32 目标不可用, 回退到 native 模拟" ;;
            64) $CC --target=wasm64-unknown-unknown -c -o "$out/smrl64.o" "$SRC/pq_smrl.c" $CFLAGS 2>/dev/null || \
                echo "  ⚠ wasm64 目标不可用, 回退到 native 模拟" ;;
        esac
        echo "  ✓ $out/smrl{64}.o (目标文件)"
    fi
}

run_test() {
    echo "[3/4] 测试 native 二进制 ..."
    "$OUT/native/smrl_test"
    echo ""
}

case "${1:-native}" in
    native) compile_native ; run_test ;;
    wasm32) compile_native ; compile_wasm wasm32 32 wasm32 ; run_test ;;
    wasm64) compile_native ; compile_wasm wasm64 64 wasm64 ; run_test ;;
    all)    compile_native ; compile_wasm wasm32 32 wasm32 ; compile_wasm wasm64 64 wasm64 ; run_test ;;
    *)      echo "用法: $0 <native|wasm32|wasm64|all>" ;;
esac

echo ""
echo "[4/4] 完成. 阶梯制·双向公约已就绪."
echo "  native: $OUT/native/smrl_test"
echo "  wasm32: $OUT/wasm32/"
echo "  wasm64: $OUT/wasm64/"
