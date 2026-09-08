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


def _parse_operand(op: str, arch_registers: dict):
    """解析单个操作数：返回 ('reg', 编号) 或 ('imm', 整数值)。"""
    name = op.strip()
    # 尝试纯数字立即数
    try:
        return ("imm", int(name, 0))
    except ValueError:
        pass
    # 寄存器编号 xN
    low = name.lower()
    if low.startswith("x") and low[1:].isdigit():
        return ("reg", int(low[1:]))
    # 寄存器别名
    for num_str, info in arch_registers.items():
        alias = info.get("alias", "").lower()
        if alias == low:
            return ("reg", int(num_str[1:]))
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

    if arch == "rv32i":
        return _encode_rv32i(semantic_id, operands, mappings, registers)
    elif arch == "x86_64":
        return _encode_x86_64(semantic_id, operands, mappings, registers)
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
        rd = _parse_operand(operands[0], registers)[1]
        rs1 = _parse_operand(operands[1], registers)[1]
        imm = _parse_operand(operands[2], registers)[1]
        code |= (rd & 0x1F) << 7
        code |= (rs1 & 0x1F) << 15
        code |= (imm & 0xFFF) << 20
    elif fmt == "S":
        rs2 = _parse_operand(operands[0], registers)[1]
        rs1 = _parse_operand(operands[1], registers)[1]
        imm = _parse_operand(operands[2], registers)[1]
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
