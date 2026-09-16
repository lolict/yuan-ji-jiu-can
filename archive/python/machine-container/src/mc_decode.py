#!/usr/bin/env python3
# ============================================================
# 机器码容器解码器 · Machine Container Decoder
#
# 用法:
#   mc_decode <机器码十六进制>
#   mc_decode 0x003100B3
#   mc_decode 3100B3
#
# 输出: 语义 ID + 操作数
# ============================================================

import sys
from mcod_spec import Container, decode_container

def main():
    if len(sys.argv) < 2:
        print("机器码容器解码器 · Machine Container Decoder")
        print()
        print("用法:")
        print("  mc_decode <机器码十六进制>")
        print()
        print("示例:")
        print("  mc_decode 0x003100B3  → ADD x1, x2, x3")
        print("  mc_decode 0x40940533  → SUB x10, x8, x9")
        sys.exit(0)

    mc_str = sys.argv[1]
    mc = int(mc_str, 16) & 0xFFFFFFFF

    # 构造容器（仅价值字段，其他默认）
    container = Container(
        platform=0x0001,  # RISC-V
        polarity=0x01,    # 主人
        ownership=0x01,   # 公有
        value=mc,
    )

    result = decode_container(container)

    if result is None:
        print(f"未知机器码: 0x{mc:08X}")
        sys.exit(1)

    print(f"机器码:     0x{mc:08X}")
    print(f"语义 ID:    {result['semantic_name']} (0x{result['semantic_id']:04X})")
    print(f"格式:       {result['format']}")
    print(f"操作数:     rd=x{result['rd']}, rs1=x{result['rs1']}", end="")
    if 'rs2' in result:
        print(f", rs2=x{result['rs2']}")
    elif result.get('imm') is not None:
        print(f", imm={result['imm']}")
    else:
        print()

if __name__ == '__main__':
    main()
