#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# fqm.sh — 容器系统统一调度器 (Shell版)
# ═══════════════════════════════════════════════════════════════════════════
#
#   用法:
#     ./fqm.sh interp <符号>              查符号信息
#     ./fqm.sh judge <A> <B>              配对审判
#     ./fqm.sh chain                      全局互锁链条报告
#     ./fqm.sh symbols                    显示64进制符号表
#     ./fqm.sh version                    显示版本时间链概览
#     ./fqm.sh help                       显示帮助
#
# ═══════════════════════════════════════════════════════════════════════════

cd "$(dirname "$0")"

case "$1" in
    interp)
        if [ -z "$2" ]; then echo "用法: fqm interp <符号>"; exit 1; fi
        ./interp "$2"
        ;;
    judge)
        if [ -z "$2" ] || [ -z "$3" ]; then echo "用法: fqm judge <A> <B>"; exit 1; fi
        ./interp "$2" "$3"
        ;;
    chain)
        ./fqmchain
        ;;
    symbols)
        ./fqm64 -t
        ;;
    version)
        head -40 VERSION_CHAIN.md
        ;;
    help|--help|-h)
        echo ""
        echo "容器系统统一调度器"
        echo ""
        echo "命令:"
        echo "  interp <符号>             查符号信息"
        echo "  judge <A> <B>             配对审判"
        echo "  chain                     全局互锁链条"
        echo "  symbols                   显示64进制符号表"
        echo "  version                   版本时间链概览"
        echo "  help                      显示帮助"
        echo ""
        ;;
    *)
        echo "未知命令: $1"
        echo "用 'fqm help' 查看帮助"
        exit 1
        ;;
esac
