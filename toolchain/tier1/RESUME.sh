#!/bin/bash
# 新会话启动时运行此脚本恢复状态

REG="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry"
TOOL="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1"
COMPILER="$TOOL/registry_compiler/target/release/registry_compiler"

echo "═══ 状态恢复 ═══"
$COMPILER stats
echo ""
echo "最近5条:"
$COMPILER list | tail -7
echo ""
echo "BMP产物:"
ls -lh $TOOL/*.bmp 2>/dev/null
echo ""
echo "Git状态:"
cd /mnt/data/catpaw/home/workspace/硅碳心源-果套循因/hunyuan-os-registry && git log --oneline -3
echo ""
echo "═══ 恢复完毕 ═══"
