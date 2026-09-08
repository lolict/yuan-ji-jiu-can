#!/usr/bin/env python3
# ============================================================
# 机器码容器编码器 · Machine Container Encoder
#
# 用法:
#   mc_encode <SEMANTIC_ID> <rd> <rs1> <rs2> [imm]
#   mc_encode ADD 1 2 3
#   mc_encode ADDI 5 1 0 100
#
# 输出: 64 位容器值 (十六进制)
# ============================================================

import sys
from mcod_spec import (
    SemanticId, Container, Platform, Polarity, Ownership,
    encode_container, decode_container,
)

def main():
    if len(sys.argv) < 2:
        print("机器码容器编码器 · Machine Container Encoder")
        print()
        print("用法:")
        print("  mc_encode <SEMANTIC_ID> <rd> <rs1> <rs2> [imm]")
        print()
        print("示例:")
        print("  mc_encode ADD 1 2 3       → ADD x1, x2, x3")
        print("  mc_encode ADDI 5 1 0 100  → ADDI x5, x1, 100")
        print("  mc_encode SUB 10 8 9      → SUB x10, x8, x9")
        print()
        print("支持的语义 ID:")
        for sid in SemanticId:
            print(f"  {sid.name:8s} (0x{sid.value:04X})")
        sys.exit(0)

    sid_str = sys.argv[1].upper()
    operands = [int(x) for x in sys.argv[2:]]

    try:
        sid = SemanticId[sid_str]
    except KeyError:
        print(f"未知语义 ID: {sid_str}")
        sys.exit(1)

    # 填充默认值
    while len(operands) < 4:
        operands.append(0)

    rd, rs1, rs2, imm = operands[0], operands[1], operands[2], operands[3]

    container = encode_container(sid, rd, rs1, rs2, imm)
    if container is None:
        print(f"编码失败: {sid_str} 格式暂不支持或参数错误")
        sys.exit(1)

    cell = container.encode()

    # ── 输出 ──
    decoded = decode_container(container)
    print(f"语义 ID: {sid.name} (0x{sid.value:04X})")
    print(f"操作数:  rd=x{rd}, rs1=x{rs1}, rs2=x{rs2}, imm={imm}")
    print(f"机器码:  0x{container.value:08X}")
    print(f"容器:    0x{cell:016X}")
    print(f"  (坐标=0x{container.platform:04X} 极性=0x{container.polarity:02X} 主权=0x{container.ownership:02X} 价值=0x{container.value:08X})")
    if decoded:
        print(f"解码验证: {decoded['semantic_name']} x{decoded['rd']}, x{decoded['rs1']}, x{decoded.get('rs2', '?')}")

if __name__ == '__main__':
    main()
