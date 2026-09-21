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

    if arch in ("rv32i", "rv64i"):
        return _decode_rv32i(machine_code, mappings)
    elif arch == "mips32":
        return _decode_mips32(machine_code, mappings)
    elif arch == "arm64":
        return _decode_arm64(machine_code, mappings)
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


def _decode_mips32(code: int, mappings: dict) -> dict:
    """MIPS32 解码：按 R/I/J 格式提取操作数。"""
    opcode = (code >> 26) & 0x3F
    rs     = (code >> 21) & 0x1F
    rt     = (code >> 16) & 0x1F
    rd     = (code >> 11) & 0x1F
    shamt  = (code >> 6) & 0x1F
    funct  = code & 0x3F
    regs   = mappings.get("registers", {})

    for sid, inst in mappings["instructions"].items():
        if inst["opcode"] != f"{opcode:06b}":
            continue
        if inst["format"] == "R" and "funct" in inst:
            if inst["funct"] != f"{funct:06b}":
                continue
        return _mips32_operands(sid, inst["format"], code, rs, rt, rd, shamt, funct, regs)

    raise ValueError(f"机器码 0x{code:08X} 在 MIPS32 映射中未找到")


def _mips32_operands(sid, fmt, code, rs, rt, rd, shamt, funct, regs) -> dict:
    """根据格式还原为 MIPS32 操作数（"$N" 形式）。"""
    reg_name = lambda n: f"${n}"
    imm16 = code & 0xFFFF
    if imm16 & 0x8000:
        imm16 -= 0x10000
    machine_hex = f"0x{code:08X}"

    if fmt == "R":
        if sid in ("SLL", "SRL", "SRA"):
            return {
                "semantic": sid, "format": "R",
                "operands": [reg_name(rd), reg_name(rt), shamt],
                "fields": {"rd": rd, "rt": rt, "shamt": shamt, "funct": funct},
                "machine_code": machine_hex
            }
        return {
            "semantic": sid, "format": "R",
            "operands": [reg_name(rd), reg_name(rs), reg_name(rt)],
            "fields": {"rd": rd, "rs": rs, "rt": rt, "funct": funct},
            "machine_code": machine_hex
        }
    elif fmt == "I":
        if sid in ("LW", "SW"):
            return {
                "semantic": sid, "format": "I-LOAD",
                "operands": [reg_name(rt), str(imm16), reg_name(rs)],
                "fields": {"rt": rt, "rs": rs, "imm16": imm16},
                "machine_code": machine_hex
            }
        elif sid in ("BEQ", "BNE"):
            return {
                "semantic": sid, "format": "I",
                "operands": [reg_name(rs), reg_name(rt), str(imm16)],
                "fields": {"rs": rs, "rt": rt, "imm16": imm16},
                "machine_code": machine_hex
            }
        elif sid == "LUI":
            return {
                "semantic": sid, "format": "I",
                "operands": [reg_name(rt), imm16 & 0xFFFF],
                "fields": {"rt": rt, "imm16": imm16 & 0xFFFF},
                "machine_code": machine_hex
            }
        return {
            "semantic": sid, "format": "I",
            "operands": [reg_name(rt), reg_name(rs), str(imm16)],
            "fields": {"rt": rt, "rs": rs, "imm16": imm16},
            "machine_code": machine_hex
        }
    elif fmt == "J":
        target = code & 0x3FFFFFF
        return {
            "semantic": sid, "format": "J",
            "operands": [target],
            "fields": {"target": target},
            "machine_code": machine_hex
        }
    return {"semantic": sid, "format": fmt, "operands": [], "machine_code": machine_hex}


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
        # 判断是否为存储加载类
        is_load = sid.startswith("L")
        # 判断是否移位指令
        is_shift = sid in ("SLLI", "SRLI", "SRAI", "SLLIW", "SRLIW", "SRAIW")
        if is_load:
            imm = (code >> 20) & 0xFFF
            if imm & 0x800:
                imm -= 0x1000
            return {
                "semantic": sid, "format": "I-LOAD",
                "operands": [reg_name(rd), str(imm), reg_name(rs1)],
                "fields": {"rd": rd, "rs1": rs1, "imm12": imm},
                "machine_code": machine_hex
            }
        if is_shift:
            shamt = (code >> 20) & 0x3F
            return {
                "semantic": sid, "format": "I-SHIFT",
                "operands": [reg_name(rd), reg_name(rs1), shamt],
                "fields": {"rd": rd, "rs1": rs1, "shamt": shamt, "funct7": funct7},
                "machine_code": machine_hex
            }
        imm = (code >> 20) & 0xFFF
        if imm & 0x800:
            imm -= 0x1000
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
        imm20 = (code >> 12) & 0xFFFFF
        return {
            "semantic": sid, "format": "U",
            "operands": [reg_name(rd), imm20],
            "fields": {"rd": rd, "imm20": imm20},
            "machine_code": machine_hex
        }
    return {"semantic": sid, "format": fmt, "operands": [], "machine_code": machine_hex}


def _alias(num: int, regs: dict) -> str:
    """寄存器编号 → 规范名（始终 xN 形式，确保往返一致）。"""
    return f"x{num}"


def _decode_arm64(code: int, mappings: dict) -> dict:
    """ARM64 AArch64 解码：前缀匹配 + 位域提取。

    前缀字符串为变长位串，高位对齐。共享 opkey 的指令（MOV/LUI, BEQ/BNE）
    由子字段（hw[22:21] 或 cond[3:0]）区分。
    """
    # 1. 收集所有 (opkey_str, fmt, Rw_semantics) 候选
    candidates = []
    for opkey, op_str in mappings["opcodes"].items():
        prefix = int(op_str, 2)
        p_len = len(op_str)
        if (code >> (32 - p_len)) == prefix:
            # 找哪些指令引用此 opkey
            for sid, inst in mappings["instructions"].items():
                if inst["opkey"] != opkey:
                    continue
                # 子字段消歧
                if opkey == "MOV_IMM":
                    hw = (code >> 21) & 0x3
                    semantic = "LUI" if hw == 1 else "MOV"
                elif opkey == "B_COND":
                    cond = code & 0xF
                    semantic = "BEQ" if cond == 0x0 else "BNE"
                else:
                    semantic = sid
                candidates.append((semantic, inst["format"], code))

    if not candidates:
        raise ValueError(f"机器码 0x{code:08X} 在 ARM64 映射中未找到")

    semantic, fmt, code = candidates[0]
    machine_hex = f"0x{code:08X}"
    regs = mappings.get("registers", {})

    if fmt == "DS":
        Rd = code & 0x1F
        Rn = (code >> 5) & 0x1F
        Rm = (code >> 16) & 0x1F
        return {
            "semantic": semantic, "format": fmt,
            "operands": [_alias(Rd, regs), _alias(Rn, regs), _alias(Rm, regs)],
            "fields": {"Rd": Rd, "Rn": Rn, "Rm": Rm},
            "machine_code": machine_hex
        }
    elif fmt == "DI":
        Rd = code & 0x1F
        Rn = (code >> 5) & 0x1F
        imm12 = (code >> 10) & 0xFFF
        return {
            "semantic": semantic, "format": fmt,
            "operands": [_alias(Rd, regs), _alias(Rn, regs), str(imm12)],
            "fields": {"Rd": Rd, "Rn": Rn, "imm12": imm12},
            "machine_code": machine_hex
        }
    elif fmt == "LSU":
        Rt = code & 0x1F
        Rn = (code >> 5) & 0x1F
        imm12 = (code >> 10) & 0xFFF
        return {
            "semantic": semantic, "format": fmt,
            "operands": [_alias(Rt, regs), str(imm12), _alias(Rn, regs)],
            "fields": {"Rt": Rt, "Rn": Rn, "imm12": imm12},
            "machine_code": machine_hex
        }
    elif fmt == "MOV_IMM":
        Rd = code & 0x1F
        imm16 = (code >> 5) & 0xFFFF
        hw = (code >> 21) & 0x3
        return {
            "semantic": semantic, "format": fmt,
            "operands": [_alias(Rd, regs), imm16],
            "fields": {"Rd": Rd, "imm16": imm16, "hw": hw},
            "machine_code": machine_hex
        }
    elif fmt == "B":
        imm26 = code & 0x3FFFFFF
        if imm26 & 0x2000000:  # 符号扩展
            imm26 -= 0x4000000
        return {
            "semantic": semantic, "format": fmt,
            "operands": [imm26],
            "fields": {"imm26": imm26},
            "machine_code": machine_hex
        }
    elif fmt == "BC":
        cond = code & 0xF
        imm19 = (code >> 5) & 0x7FFFF
        if imm19 & 0x40000:  # 符号扩展
            imm19 -= 0x80000
        return {
            "semantic": semantic, "format": fmt,
            "operands": [imm19],
            "fields": {"cond": cond, "imm19": imm19},
            "machine_code": machine_hex
        }

    raise ValueError(f"机器码 0x{code:08X} ARM64 格式 {fmt} 解码未实现")


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
