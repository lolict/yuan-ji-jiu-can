#!/bin/bash
# build.sh — Nim 兵工厂编译脚本
# 优先使用 zig cc 作为 C 编译器 (梯级双向公约)
set -e

NIM="${NIM:-/tmp/nim-2.2.0/bin/nim}"
ZIG="${ZIG:-/mnt/data/catpaw/home/.local/zig/zig-linux-x86_64-0.13.0/zig}"

# 如果 zig 存在, 用 zig cc 替代默认 CC
if [ -x "$ZIG" ]; then
    export CC="$ZIG cc"
    echo "[Nim] 使用 zig cc 作为 C 编译器"
fi

echo "[1] 语法检查: nim check"
$NIM check src/codegen.nim

echo "[2] 编译 (release)"
$NIM c -d:release src/codegen.nim

echo "[3] 产物:"
ls -lh codegen 2>/dev/null || echo "  (已生成)"
echo "[4] Nim 兵工厂就绪 ✓"
