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
    """单次往返验证。接受 encode 返回 bytes 或 int。"""
    global PASS, FAIL
    try:
        code = encode(semantic, ops_orig, arch=arch)
        # 统一为 bytes
        if isinstance(code, int):
            code_bytes = code.to_bytes(4, 'little')
        elif isinstance(code, (bytes, bytearray)):
            code_bytes = bytes(code)
        else:
            raise TypeError(f"encode 返回类型无法识别: {type(code)}")
        dec = decode(code_bytes, arch=arch)
        ops_round = [str(o) for o in dec["operands"]]
        ops_orig_str = [str(o) for o in ops_orig]
        ok = (dec["semantic"] == semantic) and (ops_round == ops_orig_str)
        if expected_hex and dec.get("machine_code") != expected_hex:
            ok = False
        status = "✅" if ok else "❌"
        PASS += ok
        FAIL += not ok
        hex_display = "0x" + code_bytes.hex().upper()
        print(f"{status} {arch:8s} {semantic:8s}({', '.join(ops_orig_str):20s}) "
              f"→ {hex_display:20s} → {dec['semantic']:8s}({', '.join(ops_round)})")
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

# ── RV64I ──
print("\n═══ RV64I 往返测试 ═══")
# 基本 RV32I 兼容性（编码一致）
check("rv64i", "ADD",  ["x1", "x2", "x3"])
check("rv64i", "SUB",  ["x10", "x11", "x12"])
check("rv64i", "ADDI", ["x5", "x0", 42])
check("rv64i", "LUI",  ["x1", 74565])
# RV64I 新增字操作
check("rv64i", "ADDIW", ["x5", "x0", 100])
check("rv64i", "ADDW",  ["x1", "x2", "x3"])
check("rv64i", "SUBW",  ["x10", "x11", "x12"])
check("rv64i", "SLLW",  ["x1", "x2", "x3"])
check("rv64i", "SRLW",  ["x1", "x2", "x3"])
check("rv64i", "SRAW",  ["x1", "x2", "x3"])
check("rv64i", "SLLIW", ["x5", "x6", 4])
check("rv64i", "SRAIW", ["x5", "x6", 8])
# 64位加载/存储
check("rv64i", "LD",    ["x8", 0, "x9"])
check("rv64i", "SD",    ["x10", 0, "x2"])
check("rv64i", "LWU",   ["x8", 0, "x9"])

# ── ARM64 ──
print("\n═══ ARM64 往返测试 ═══")
# DS 型 (Rd, Rn, Rm)
check("arm64", "ADD", ["x1", "x2", "x3"])
check("arm64", "SUB", ["x10", "x11", "x12"])
check("arm64", "AND", ["x7", "x8", "x9"])
check("arm64", "OR",  ["x4", "x5", "x6"])
check("arm64", "XOR", ["x0", "x1", "x2"])

# DI 型 (Rd, Rn, #imm12)
check("arm64", "ADDI", ["x5", "x0", 42])
check("arm64", "ADDI", ["x10", "x0", 0])
check("arm64", "SUBI", ["x6", "x7", 0xFF])

# LSU 型 (Rt, imm12, Rn)
check("arm64", "LW", ["x8", 0, "x9"])
check("arm64", "LW", ["x10", 4, "x2"])
check("arm64", "SW", ["x3", 0, "x4"])
check("arm64", "SW", ["x5", 8, "x6"])

# MOV_IMM 型
check("arm64", "MOV", ["x5", 0])
check("arm64", "MOV", ["x10", 42])
check("arm64", "LUI", ["x1", 4660])

# B 型 (无条件跳转)
check("arm64", "JMP", [0])
check("arm64", "JMP", [4])

# BC 型 (条件跳转)
check("arm64", "BEQ", [8])
check("arm64", "BNE", [0xF00])

# ── x86-64 ──
print("\n═══ x86-64 往返测试 ═══")
# R 型: dst, src (REX.W + opcode + ModRM → 3 bytes)
check("x86_64", "ADD", ["rax", "rbx"])
check("x86_64", "SUB", ["rcx", "rdx"])
check("x86_64", "AND", ["rsi", "rdi"])
check("x86_64", "XOR", ["rax", "rax"])
check("x86_64", "MOV_REG", ["rbx", "rax"])
check("x86_64", "CMP", ["rax", "rbx"])
# I 型: dst, imm32 (REX.W + opcode + ModRM(/digit) + imm32 → 7 bytes)
check("x86_64", "ADDI", ["rax", 42])
check("x86_64", "SUBI", ["rcx", 0xFF])
# MOV_IMM32: dst, imm32 (REX.W + B8+rd + imm32 → 6 bytes)
check("x86_64", "MOV_IMM", ["rax", 0x1234])
check("x86_64", "MOV_IMM", ["rbx", 0])
# J32 型: rel32 → 5 bytes
check("x86_64", "JMP", [0])
check("x86_64", "JMP", [4])
# B8 型 (条件跳转 rel8 → 2 bytes)
check("x86_64", "BEQ", [8])
check("x86_64", "BNE", [0x7F])

# ── 结果 ──
print(f"\n结果: {PASS} 通过, {FAIL} 失败")
sys.exit(0 if FAIL == 0 else 1)
