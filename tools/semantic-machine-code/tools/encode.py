"""
encode.py — 语义编码器（预加载式·无运行时计算）

编码链路（预加载冰箱模型）：
  语义ID
    → 查 semantic_ids/*.json（确认该语义存在）
      → 查 mappings/<arch>.json（取得 opcode/funct3/funct7/format）
        → 按 format 位域拼装机器码
          → 返回 32位整数

每一环都是字典查找，无解释、无计算、无外部调用。
容器落款：刘楚恬和我 · 私有编码系统
"""

import json
import os

CONTAINER_ROOT = os.path.dirname(os.path.abspath(__file__))
SMC_ROOT = os.path.dirname(CONTAINER_ROOT)


def _load_json(relative_path: str) -> dict:
    """预加载：一次性读入，存于内存字典——冰箱开门直接取。"""
    path = os.path.join(SMC_ROOT, relative_path)
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _parse_operand(op, arch_registers: dict):
    """解析单个操作数：返回 ('reg', 编号) 或 ('imm', 整数值)。"""
    if isinstance(op, int):
        return ("imm", op)
    name = str(op).strip()
    # 尝试纯数字立即数
    try:
        return ("imm", int(name, 0))
    except ValueError:
        pass
    # 寄存器编号 xN (RISC-V/MIPS-style)
    low = name.lower()
    if low.startswith("x") and low[1:].isdigit():
        return ("reg", int(low[1:]))
    # 寄存器编号 $N (MIPS-style)
    if low.startswith("$") and low[1:].isdigit():
        return ("reg", int(low[1:]))
    # 寄存器别名（遍历各架构）
    for num_str, info in arch_registers.items():
        alias = info.get("alias", "").lower()
        if alias == low:
            # 从 key (xN 或 $N) 提取编号
            key_low = num_str.lower()
            if key_low.startswith("x") and key_low[1:].isdigit():
                return ("reg", int(key_low[1:]))
            if key_low.startswith("$") and key_low[1:].isdigit():
                return ("reg", int(key_low[1:]))
    raise ValueError(f"操作数'{op}'无法解析为寄存器或立即数")


def encode(semantic_id: str, operands: list, arch: str = "rv32i", registers: dict = None,
           semantic_families: dict = None, mappings: dict = None) -> int:
    """
    将语义ID + 操作数列表编码为指定架构的机器码。

    参数：
        semantic_id: 如 "ADD", "JAL"
        operands:    操作数列表，如 ["x1", "x2", "x3"]
        arch:        目标架构，如 "rv32i", "x86_64"
        registers:   该架构寄存器表（预加载传入，避免重复IO）
        semantic_families: 语义族定义表
        mappings:    该架构映射表

    返回：
        32位机器码整数
    """
    if mappings is None:
        mappings = _load_json(f"mappings/{arch}.json")
    if registers is None:
        registers = mappings.get("registers", {})

    if arch in ("rv32i", "rv64i"):
        return _encode_rv32i(semantic_id, operands, mappings, registers)
    elif arch == "mips32":
        return _encode_mips32(semantic_id, operands, mappings, registers)
    elif arch == "arm64":
        return _encode_arm64(semantic_id, operands, mappings, registers)
    else:
        raise ValueError(f"暂不支持架构: {arch}")


def _encode_rv32i(semantic_id, operands, mappings, registers):
    """RV32I 编码：按位域拼装。"""
    inst = mappings["instructions"][semantic_id]
    fmt = inst["format"]
    opcode_bits = int(inst["opcode"], 2)
    funct3_bits = int(inst["funct3"], 2) if "funct3" in inst else 0
    funct7_bits = int(inst["funct7"], 2) if "funct7" in inst else 0

    code = 0
    code |= (opcode_bits & 0x7F) << 0
    code |= (funct3_bits & 0x7) << 12

    if fmt == "R":
        rd = _parse_operand(operands[0], registers)[1]
        rs1 = _parse_operand(operands[1], registers)[1]
        rs2 = _parse_operand(operands[2], registers)[1]
        code |= (rd & 0x1F) << 7
        code |= (rs1 & 0x1F) << 15
        code |= (rs2 & 0x1F) << 20
        code |= (funct7_bits & 0x7F) << 25
    elif fmt == "I":
        is_load = semantic_id.startswith("L")  # LW/LWU/LD: rd, imm, rs1
        is_shift = semantic_id in ("SLLI", "SRLI", "SRAI", "SLLIW", "SRLIW", "SRAIW")
        if is_load:
            rd  = _parse_operand(operands[0], registers)[1]
            imm = _parse_operand(operands[1], registers)[1]
            rs1 = _parse_operand(operands[2], registers)[1]
            code |= (rd & 0x1F) << 7
            code |= (rs1 & 0x1F) << 15
            code |= (imm & 0xFFF) << 20
        elif is_shift:
            # SLLI/SRAI/SLLIW/SRAIW: rd, rs1, shamt → funct7[31:25] + shamt[24:20]
            rd   = _parse_operand(operands[0], registers)[1]
            rs1  = _parse_operand(operands[1], registers)[1]
            shamt = _parse_operand(operands[2], registers)[1]
            code |= (rd & 0x1F) << 7
            code |= (rs1 & 0x1F) << 15
            code |= (funct7_bits & 0x7F) << 25
            code |= (shamt & 0x3F) << 20
        else:
            rd  = _parse_operand(operands[0], registers)[1]
            rs1 = _parse_operand(operands[1], registers)[1]
            imm = _parse_operand(operands[2], registers)[1]
            code |= (rd & 0x1F) << 7
            code |= (rs1 & 0x1F) << 15
            code |= (funct7_bits & 0x7F) << 25
            code |= (imm & 0xFFF) << 20
    elif fmt == "S":
        # S 型：rs2, offset(base)
        rs2 = _parse_operand(operands[0], registers)[1]
        imm = _parse_operand(operands[1], registers)[1]
        rs1 = _parse_operand(operands[2], registers)[1]
        imm_lo = imm & 0x1F
        imm_hi = (imm >> 5) & 0x7F
        code |= (rs1 & 0x1F) << 15
        code |= (rs2 & 0x1F) << 20
        code |= (imm_lo) << 7
        code |= (imm_hi) << 25
    elif fmt == "U":
        rd = _parse_operand(operands[0], registers)[1]
        imm = _parse_operand(operands[1], registers)[1]
        code |= (rd & 0x1F) << 7
        code |= (imm & 0xFFFFF) << 12

    return code


def _encode_x86_64(semantic_id, operands, mappings, registers):
    """x86-64 编码：返回最简形式。当前仅支持 JMP rel8 示例。"""
    inst = mappings["instructions"][semantic_id]
    if semantic_id == "JMP":
        rel8 = int(operands[0])
        return 0xEB | ((rel8 & 0xFF) << 8)
    raise NotImplementedError(f"x86-64 {semantic_id} 的编码尚未在预加载中实现")


def _encode_mips32(semantic_id, operands, mappings, registers):
    """MIPS32 编码：按 R/I/J 格式拼装。"""
    inst = mappings["instructions"][semantic_id]
    fmt = inst["format"]
    opcode_val = int(inst["opcode"], 2)
    funct_val = int(inst.get("funct", "0"), 2) if "funct" in inst else 0
    shamt_val = int(inst.get("shamt", "00000"), 2)

    code = 0
    code |= (opcode_val & 0x3F) << 26

    if fmt == "R":  # opcode[31:26] | rs[25:21] | rt[20:16] | rd[15:11] | shamt[10:6] | funct[5:0]
        if semantic_id in ("SLL", "SRL", "SRA"):
            # 移位指令: rd=operands[0], rt=operands[1], shamt=operands[2]
            rd = _parse_operand(operands[0], registers)[1]
            rt = _parse_operand(operands[1], registers)[1]
            shamt = _parse_operand(operands[2], registers)[1]
            rs = 0  # 未使用
            code |= (rs & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (rd & 0x1F) << 11
            code |= (shamt & 0x1F) << 6
            code |= (funct_val & 0x3F)
        else:
            rd = _parse_operand(operands[0], registers)[1]
            rs = _parse_operand(operands[1], registers)[1]
            rt = _parse_operand(operands[2], registers)[1]
            code |= (rs & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (rd & 0x1F) << 11
            code |= (shamt_val & 0x1F) << 6
            code |= (funct_val & 0x3F)
    elif fmt == "I":  # opcode[31:26] | rs[25:21] | rt[20:16] | imm16[15:0]
        if semantic_id in ("LW", "SW"):
            # LW rt, offset(base) → operands = [rt, offset, base]
            rt = _parse_operand(operands[0], registers)[1]
            imm = _parse_operand(operands[1], registers)[1]
            rs = _parse_operand(operands[2], registers)[1]
            code |= (rs & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (imm & 0xFFFF)
        elif semantic_id in ("BEQ", "BNE"):
            # BEQ rs, rt, offset
            rs = _parse_operand(operands[0], registers)[1]
            rt = _parse_operand(operands[1], registers)[1]
            imm = _parse_operand(operands[2], registers)[1]
            code |= (rs & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (imm & 0xFFFF)
        elif semantic_id == "LUI":
            # LUI rt, imm; rs = 0
            rt = _parse_operand(operands[0], registers)[1]
            imm = _parse_operand(operands[1], registers)[1]
            code |= (0 & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (imm & 0xFFFF)
        else:
            # ADDI/ANDI/ORI: rt, rs, imm
            rt = _parse_operand(operands[0], registers)[1]
            rs = _parse_operand(operands[1], registers)[1]
            imm = _parse_operand(operands[2], registers)[1]
            code |= (rs & 0x1F) << 21
            code |= (rt & 0x1F) << 16
            code |= (imm & 0xFFFF)
    elif fmt == "J":  # opcode[31:26] | target[25:0]
        target = _parse_operand(operands[0], registers)[1]
        code |= (target & 0x3FFFFFF)

    return code


def _encode_arm64(semantic_id, operands, mappings, registers):
    """ARM64 AArch64 编码：前缀占位 + 寄存器/立即数填充。

    前缀字符串取自 opcodes 表（变长位串），高位对齐，凹槽由格式决定。
    """
    inst = mappings["instructions"][semantic_id]
    fmt = inst["format"]
    opkey_str = mappings["opcodes"][inst["opkey"]]

    prefix_val = int(opkey_str, 2)
    prefix_len = len(opkey_str)
    code = prefix_val << (32 - prefix_len)

    if fmt == "DS":
        # 前缀 11bits [31:21] → Rm[20:16] imm6[15:10] Rn[9:5] Rd[4:0]
        Rd = _parse_operand(operands[0], registers)[1]
        Rn = _parse_operand(operands[1], registers)[1]
        Rm = _parse_operand(operands[2], registers)[1]
        code |= (Rm & 0x1F) << 16
        code |= (Rn & 0x1F) << 5
        code |= (Rd & 0x1F)
    elif fmt == "DI":
        # 前缀 8bits [31:24] → shift[23:22] imm12[21:10] Rn[9:5] Rd[4:0]
        Rd = _parse_operand(operands[0], registers)[1]
        Rn = _parse_operand(operands[1], registers)[1]
        imm12 = _parse_operand(operands[2], registers)[1]
        shift = 0
        code |= (shift & 0x3) << 22
        code |= (imm12 & 0xFFF) << 10
        code |= (Rn & 0x1F) << 5
        code |= (Rd & 0x1F)
    elif fmt == "LSU":
        # LW/SW: operands = [Rt, imm12, Rn]
        # 前缀 8bits [31:24] → imm12[21:10] Rn[9:5] Rt[4:0]
        Rt = _parse_operand(operands[0], registers)[1]
        imm12 = _parse_operand(operands[1], registers)[1]
        Rn = _parse_operand(operands[2], registers)[1]
        code |= (imm12 & 0xFFF) << 10
        code |= (Rn & 0x1F) << 5
        code |= (Rt & 0x1F)
    elif fmt == "MOV_IMM":
        # 前缀 9bits [31:23] → hw[22:21] imm16[20:5] Rd[4:0]
        hw = 0 if semantic_id == "MOV" else 1  # LUI 左移16位
        Rd = _parse_operand(operands[0], registers)[1]
        imm16 = _parse_operand(operands[1], registers)[1]
        code |= (hw & 0x3) << 21
        code |= (imm16 & 0xFFFF) << 5
        code |= (Rd & 0x1F)
    elif fmt == "B":
        # JMP: 前缀 6bits [31:26] → imm26[25:0]
        imm26 = _parse_operand(operands[0], registers)[1]
        code |= (imm26 & 0x3FFFFFF)
    elif fmt == "BC":
        # BEQ/BNE: 前缀 8bits [31:24] → imm19[23:5] op1[4] cond[3:0]
        cond = 0x0 if semantic_id == "BEQ" else 0x1  # EQ=0000 NE=0001
        imm19 = _parse_operand(operands[0], registers)[1]
        code |= (imm19 & 0x7FFFF) << 5
        code |= cond
    else:
        raise ValueError(f"ARM64 格式 {fmt} 尚未实现")
    return code


# —— CLI 入口（兼容直接运行） ——
if __name__ == "__main__":
    import sys
    if len(sys.argv) < 4:
        print("用法: python encode.py <SemanticID> <arch> <operands...>")
        print("示例: python encode.py ADD rv32i x1 x2 x3")
        sys.exit(1)

    sid = sys.argv[1].upper()
    arch = sys.argv[2]
    ops = sys.argv[3:]

    code = encode(sid, ops, arch=arch)
    machine_hex = f"0x{code:08X}"
    print(f"语义: {sid}({', '.join(ops)})")
    print(f"架构: {arch}")
    print(f"机器码: {machine_hex}")
    print(f"二进制: {code:032b}")
