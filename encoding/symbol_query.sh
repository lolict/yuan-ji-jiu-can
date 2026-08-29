#!/bin/bash
# 符号查询工具 — 调用 C 共享库，不重复定义符号
# 用法: ./symbol_query.sh <符号>
# 示例: ./symbol_query.sh b
#        ./symbol_query.sh 袋

SYM="$1"

if [ -z "$SYM" ]; then
    echo "用法: $0 <符号>"
    echo "示例: $0 b"
    echo "      $0 袋"
    exit 1
fi

# 调用 C 测试程序查询
cd "$(dirname "$0")"
./test_sym_query "$SYM"
