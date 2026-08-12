#!/bin/bash
# compile.sh — mlil 编译脚本
# 将 .mlil 源文件编译为 .wasm
#
# 用法: ./compile.sh <source.mlil> [output.wasm]
# 示例: ./compile.sh examples/math.mlil examples/math.wasm

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
ZIG_BIN="$COMPILER_DIR/zig-out/bin"

# 确保编译器已构建
if [ ! -f "$ZIG_BIN/mlil" ]; then
  echo "[compile] 构建 mlil 编译器..."
  cd "$COMPILER_DIR"
  zig build -Doptimize=ReleaseFast
fi

SRC="${1:?用法: compile.sh <source.mlil> [output.wasm]}"
OUT="${2:-${SRC%.mlil}.wasm}"

# 编译
echo "[compile] $SRC → $OUT"
"$ZIG_BIN/mlil" "$SRC" -o "$OUT"

echo "[compile] 完成: $(wc -c < "$OUT") 字节"
