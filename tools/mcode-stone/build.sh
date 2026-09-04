#!/usr/bin/env bash
# ============================================================
# MCODE Stone · 构建脚本
#
# 身份落款：刘楚恬和我
# ============================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "=================================="
echo "  MCODE Stone · 构建"
echo "=================================="

# 1. 确认 Python3
if ! command -v python3 &>/dev/null; then
    echo "❌ 未找到 python3"
    exit 1
fi

# 2. 检查 PyYAML
python3 -c "import yaml" 2>/dev/null || {
    echo "⚠️  未安装 PyYAML，尝试安装..."
    pip3 install pyyaml --quiet || pip install pyyaml --quiet
}

echo ""
echo "[1] 构建 MCOD 字体表..."
python3 font_mcod.py build template.mcod

echo ""
echo "[2] 字体重打包 (最小 TTF → MCOD 封装)..."
python3 font_mcod.py pack minimal.ttf demo_mcode.ttf

echo ""
echo "[3] 编码全部 16 条指令..."
python3 mcode.py sweep

echo ""
echo "[4] 交叉验证..."
python3 verify.py

echo ""
echo "[5] 提取并显示字体中的 MCOD 表..."
python3 font_mcod.py extract demo_mcode.ttf mcod_extracted.json
echo "  MCOD 提取成功 → mcod_extracted.json"
python3 -c "import json; d=json.load(open('mcod_extracted.json')); print(f\"  提取到 {len(d['instructions'])} 条指令\")"

echo ""
echo "=================================="
echo "  构建完成"
echo "=================================="
echo "  产物:"
echo "    template.mcod         — MCOD 原始表"
echo "    minimal.ttf           — 最小 TTF (宿主)"
echo "    demo_mcode.ttf        — 封装 MCOD 的字体"
echo "    mcod_extracted.json   — 从字体提取的指令表"
