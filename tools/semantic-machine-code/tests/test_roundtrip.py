"""
test_roundtrip.py — 编码→解码 往返闭环（三元裁判验证）

验证原则：
  语义 ID + 操作数 → encode → 机器码 → decode → 语义 ID + 操作数
  正（编码）、反（解码）、裁判（双向一致）三者闭合。
"""

import sys
import os
PARENT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, PARENT)

from tools.encode import encode
from tools.decode import decode

PASS = 0
FAIL = 0

def check(arch, semantic, ops_orig, expected_hex=None):
    """单次往返验证。"""
    global PASS, FAIL
    try:
        code = encode(semantic, ops_orig, arch=arch)
        dec = decode(code, arch=arch)
        # 规范化操作数（整型转字符串）
        ops_round = [str(o) for o in dec["operands"]]
        ops_orig_str = [str(o) for o in ops_orig]
        ok = (dec["semantic"] == semantic) and (ops_round == ops_orig_str)
        if expected_hex and f"0x{code:08X}" != expected_hex:
            ok = False
        status = "✅" if ok else "❌"
        PASS += ok
        FAIL += not ok
        print(f"{status} {arch:8s} {semantic:8s}({', '.join(ops_orig_str):20s}) "
              f"→ 0x{code:08X} → {dec['semantic']:8s}({', '.join(ops_round)})")
    except Exception as e:
        FAIL += 1
        print(f"❌ {arch:8s} {semantic:8s}({', '.join(str(o) for o in ops_orig):20s}) "
              f"→ 错误: {e}")


# ── RV32I ──
print("═══ RV32I 往返测试 ═══")

# R 型
check("rv32i", "ADD", ["x1", "x2", "x3"])
check("rv32i", "SUB", ["x10", "x11", "x12"])
check("rv32i", "XOR", ["x4", "x5", "x6"])
check("rv32i", "AND", ["x7", "x8", "x9"])
check("rv32i", "SLL", ["x1", "x2", "x3"])
check("rv32i", "SRL", ["x1", "x2", "x3"])
check("rv32i", "SRA", ["x1", "x2", "x3"])

# I 型（寄存器-立即数）
check("rv32i", "ADDI", ["x5", "x0", 42])
check("rv32i", "ADDI", ["x10", "x0", 0])
check("rv32i", "ANDI", ["x6", "x7", 0xFF])

# I 型（载荷 — 操序: rd, imm, rs1）
check("rv32i", "LW", ["x8", 0, "x9"])
check("rv32i", "LW", ["x10", 4, "x2"])

# U 型
check("rv32i", "LUI", ["x1", 0x12345])

# ── MIPS32 ──
print("\n═══ MIPS32 往返测试 ═══")

# R 型（rd, rs, rt — 除 SLL/SRL/SRA 特殊）
check("mips32", "ADD", ["$1", "$2", "$3"])
check("mips32", "SUB", ["$10", "$11", "$12"])
check("mips32", "AND", ["$7", "$8", "$9"])
check("mips32", "OR",  ["$4", "$5", "$6"])
check("mips32", "SLL", ["$8", "$9", 4])
check("mips32", "SRL", ["$10", "$11", 2])

# I 型（rt, rs, imm — 载荷/分支/立即数）
check("mips32", "ADDI", ["$8", "$9", 42])
check("mips32", "ADDI", ["$5", "$0", 0])
check("mips32", "ANDI", ["$6", "$7", 0xFF])
check("mips32", "LW",   ["$8", 0, "$9"])
check("mips32", "LW",   ["$10", 4, "$2"])
check("mips32", "BEQ",  ["$1", "$2", 8])
check("mips32", "BNE",  ["$3", "$4", 16])
check("mips32", "LUI",  ["$1", 0x1234])

# J 型
check("mips32", "JMP",  [0x1000])

# ── 结果 ──
print(f"\n结果: {PASS} 通过, {FAIL} 失败")
sys.exit(0 if FAIL == 0 else 1)
