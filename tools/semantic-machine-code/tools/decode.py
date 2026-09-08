"""
decode.py — 语义解码器（预加载式·无运行时计算）

解码链路（反向冰箱）：
  32位机器码
    → 提取 opcode[6:0]
      → 查 mappings/<arch>.json opcode 表，候选指令集
        → 按 funct3/funct7 比对确认语义ID
          → 按 format 位域拆分寄存器/立即数
            → 返回 {semantic, operands, format}

容器落款：刘楚恬和我 · 私有编码系统
"""

import json
import os

CONTAINER_ROOT = os.path.dirname(os.path.abspath(__file__))
SMC_ROOT = os.path.dirname(CONTAINER_ROOT)


def _load_json(relative_path: str) -> dict:
    path = os.path.join(SMC_ROOT, relative_path)
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def decode(machine_code: int, arch: str = "rv32i", mappings: dict = None) -> dict:
    """将32位机器码解码为语义ID + 操作数列表。"""
    if mappings is None:
        mappings = _load_json(f"mappings/{arch}.json")

    if arch == "rv32i":
        return _decode_rv32i(machine_code, mappings)
    raise ValueError(f"暂不支持架构: {arch}")


def _decode_rv32i(code: int, mappings: dict) -> dict:
    """RV32I 解码：按位域提取。"""
    opcode = code & 0x7F
    rd     = (code >> 7) & 0x1F
    funct3 = (code >> 12) & 0x7
    rs1    = (code >> 15) & 0x1F
    rs2    = (code >> 20) & 0x1F
    funct7 = (code >> 25) & 0x7F

    # 反向查找：opcode+funct3+funct7 → 语义ID
    for sid, inst in mappings["instructions"].items():
        if inst["opcode"] != f"{opcode:07b}":
            continue
        if "funct3" in inst and inst["funct3"] != f"{funct3:03b}":
            continue
        if "funct7" in inst and inst["funct7"] != f"{funct7:07b}":
            continue
        # 匹配成功
        fmt = inst["format"]
        regs = mappings.get("registers", {})
        return _rv32i_operands(sid, fmt, code, rd, rs1, rs2, funct7, regs)

    raise ValueError(f"机器码 0x{code:08X} 在 RV32I 映射中未找到")


def _rv32i_operands(sid, fmt, code, rd, rs1, rs2, funct7, regs) -> dict:
    """根据格式还原操作数。"""
    reg_name = lambda n: _alias(n, regs)
    machine_hex = f"0x{code:08X}"

    if fmt == "R":
        return {
            "semantic": sid, "format": "R",
            "operands": [reg_name(rd), reg_name(rs1), reg_name(rs2)],
            "fields": {"rd": rd, "rs1": rs1, "rs2": rs2,
                       "funct3": (code >> 12) & 0x7, "funct7": funct7},
            "machine_code": machine_hex
        }
    elif fmt == "I":
        imm = (code >> 20) & 0xFFF
        if imm & 0x800:
            imm -= 0x1000  # 符号扩展
        if sid in ("LW", "LH", "LBU", "LHU", "LB"):
            return {
                "semantic": sid, "format": "I-LOAD",
                "operands": [reg_name(rd), str(imm), reg_name(rs1)],
                "fields": {"rd": rd, "rs1": rs1, "imm12": imm},
                "machine_code": machine_hex
            }
        return {
            "semantic": sid, "format": "I",
            "operands": [reg_name(rd), reg_name(rs1), str(imm)],
            "fields": {"rd": rd, "rs1": rs1, "imm12": imm},
            "machine_code": machine_hex
        }
    elif fmt == "S":
        imm_lo = (code >> 7) & 0x1F
        imm_hi = (code >> 25) & 0x7F
        imm = (imm_hi << 5) | imm_lo
        if imm & 0x800:
            imm -= 0x1000
        return {
            "semantic": sid, "format": "S",
            "operands": [reg_name(rs2), str(imm), reg_name(rs1)],
            "fields": {"rs1": rs1, "rs2": rs2, "imm12": imm},
            "machine_code": machine_hex
        }
    elif fmt == "U":
        imm = code & 0xFFFFF000
        return {
            "semantic": sid, "format": "U",
            "operands": [reg_name(rd), f"0x{imm:08X}"],
            "fields": {"rd": rd, "imm20": imm},
            "machine_code": machine_hex
        }
    return {"semantic": sid, "format": fmt, "operands": [], "machine_code": machine_hex}


def _alias(num: int, regs: dict) -> str:
    """寄存器编号 → 别名。如 1 → ra, 2 → sp。"""
    key = f"x{num}"
    return regs.get(key, {}).get("alias", f"x{num}")


# —— CLI 入口 ——
if __name__ == "__main__":
    import sys
    if len(sys.argv) < 2:
        print("用法: python decode.py <machine_hex> [arch]")
        print("示例: python decode.py 0x003100B3 rv32i")
        sys.exit(1)

    code = int(sys.argv[1], 16)
    arch = sys.argv[2] if len(sys.argv) > 2 else "rv32i"

    result = decode(code, arch=arch)
    print(f"机器码: {result['machine_code']}")
    print(f"架构:   {arch}")
    print(f"语义:   {result['semantic']}")
    print(f"格式:   {result['format']}")
    print(f"操作数: {', '.join(str(o) for o in result['operands'])}")
    if "fields" in result:
        print(f"位域:   {result['fields']}")
