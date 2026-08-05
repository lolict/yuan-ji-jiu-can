#!/bin/bash
# 囡囝共生 · 一键启动浏览器壳 Demo
# ===================================
# 用法: ./run-demo.sh
# 自动启动本地 HTTP 服务器并打开浏览器

set -e

PORT="${1:-8080}"
SHELL_DIR="$(cd "$(dirname "$0")/shell/browser" && pwd)"
URL="http://localhost:${PORT}"

echo "=== 囡囝共生 · 共生共识引擎 Demo ==="
echo "启动目录: ${SHELL_DIR}"
echo "访问地址: ${URL}"
echo ""
echo "按 Ctrl+C 停止服务器"
echo ""

# 尝试打开浏览器(安静失败)
if command -v xdg-open &> /dev/null; then
  (sleep 1 && xdg-open "$URL" &> /dev/null) &
elif command -v open &> /dev/null; then
  (sleep 1 && open "$URL" &> /dev/null) &
fi

# 启动 HTTP 服务器
cd "$SHELL_DIR"
python3 -m http.server "$PORT"
