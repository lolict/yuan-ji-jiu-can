#!/bin/bash
# 民联浏览器壳 · WASM 内核编译脚本 v1.7
# ====================================
# 编译 mqf_wasm.c → njsg_core.wasm (供浏览器壳使用)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")/.."
SRC="${PROJECT_DIR}/../kernel/wasm/mqf_wasm.c"
OUT="${PROJECT_DIR}/browser/njsg_core.wasm"

echo "=== 民联浏览器壳 WASM 编译 ==="
echo "源文件: ${SRC}"
echo "输出:   ${OUT}"

# 检查源文件
if [ ! -f "$SRC" ]; then
  echo "❌ 找不到源文件: $SRC"
  exit 1
fi

# 尝试多种编译器
if [ -x "/opt/wasi-sdk/bin/clang" ]; then
  echo ">> 使用 WASI SDK"
  /opt/wasi-sdk/bin/clang \
    --target=wasm32-wasi \
    -O2 \
    -z stack-size=1048576 \
    -Wl,--initial-memory=65536 \
    -o "$OUT" \
    "$SRC"
elif command -v clang &> /dev/null; then
  echo ">> 使用系统 clang (尝试 wasm32 目标)"
  clang --target=wasm32 -O2 -nostdlib -Wl,--no-entry -Wl,--export-all -o "$OUT" "$SRC" 2>/dev/null || {
    echo "⚠ 系统 clang 不支持 wasm,仅编译 ELF 测试版"
    clang -O2 -std=c17 -o "${OUT%.wasm}_test" "$SRC" -lm
    echo "✅ 已编译测试版: ${OUT%.wasm}_test"
    exit 0
  }
else
  echo "❌ 找不到 clang/WASI SDK"
  echo "请安装: https://github.com/WebAssembly/wasi-sdk"
  exit 1
fi

echo "✅ WASM 内核已生成: $OUT"
ls -lh "$OUT"

# 验证
if command -v wasmtime &> /dev/null; then
  echo ">> 运行测试: wasmtime njsg_core.wasm"
  echo '{"entity":"ios","compliance_a":0.9,"compliance_b":0.85,"force":15,"angle":45}' | wasmtime "$OUT" 2>/dev/null || echo "WASI stdin 测试失败,但这不影响浏览器壳回退到 JS"
fi
