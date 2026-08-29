#!/bin/bash
# 编译符号表为最小二进制
# 输出: WASM / 共享库 / 静态库

set -e

echo "=== 编译符号表 ==="

# 1. 编译为共享库 (.so)
echo "[1/3] 编译共享库..."
gcc -shared -fPIC -O2 -o libsymbol_table.so symbol_table.c

# 2. 编译为静态库 (.a)
echo "[2/3] 编译静态库..."
gcc -c -O2 -o symbol_table.o symbol_table.c
ar rcs libsymbol_table.a symbol_table.o

# 3. 编译为WASM (需要wasm32-wasi目标)
echo "[3/3] 编译WASM..."
if command -v clang &> /dev/null; then
    clang --target=wasm32-wasi -O2 -o symbol_table.wasm symbol_table.c 2>/dev/null || echo "  (跳过: 需要wasi-sdk)"
else
    echo "  (跳过: 需要clang + wasi-sdk)"
fi

echo "=== 完成 ==="
echo "输出文件:"
ls -la libsymbol_table.* symbol_table.wasm 2>/dev/null || true
