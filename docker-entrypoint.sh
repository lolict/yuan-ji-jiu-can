#!/bin/bash
# 满全法 · 容器入口脚本
# ManQuanFa · Container Entrypoint

set -e

echo "══════════════════════════════════════════════════"
echo "  满全法 · 囡囝共生元  |  ManQuanFa NanJian"
echo "  执行内核容器 v0.8"
echo "══════════════════════════════════════════════════"
echo ""

MODE="${1:---run-demo}"

case "$MODE" in

  --run-demo)
    echo "[启动] 裁判者运行全套规则演示..."
    echo ""
    /mqf/kernel/src/mqf_kernel.elf
    ;;

  --shell)
    echo "[启动] 进入裁判者 shell(开发者模式)..."
    bash
    ;;

  --version)
    echo "满全法 执行内核 v0.8.0"
    echo "层: 编码·契约·空间代数·时间切片·WASM·规则引擎·金壳封装·统计锥"
    ;;

  --help)
    cat <<'HELP'
满全法 · 囡囝共生元 容器

用法:
  docker run --rm mqf-kernel:0.8 --run-demo      运行全套规则演示(默认)
  docker run --rm -it mqf-kernel:0.8 --shell       进入开发者 shell
  docker run --rm mqf-kernel:0.8 --version        打印版本
  docker run --rm mqf-kernel:0.8 --help           查看帮助
HELP
    ;;

  *)
    echo "[启动] 自定义模式: $@"
    exec /mqf/kernel/src/mqf_kernel.elf "$@"
    ;;
esac
