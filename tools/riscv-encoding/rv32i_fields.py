#!/usr/bin/env python3
# ============================================================
# RV32I 字段编码 · 纯加法实现（禁用乘除）
#
# 所有 encode_* 函数接受 opcode 参数，由调用者查表传入，
# 确保不同opcode的I/S/U/J指令能获得正确的opcode字段。
# ============================================================

RV32I = {
    "LUI":    {"opcode": 0x37, "funct3": -1, "funct7": -1, "fmt": "U"},
    "AUIPC":  {"opcode": 0x17, "funct3": -1, "funct7": -1, "fmt": "U"},
    "JAL":    {"opcode": 0x6F, "funct3": -1, "funct7": -1, "fmt": "J"},
    "JALR":   {"opcode": 0x67, "funct3": 0,  "funct7": -1, "fmt": "I"},
    "BEQ":    {"opcode": 0x63, "funct3": 0,  "funct7": -1, "fmt": "B"},
    "BNE":    {"opcode": 0x63, "funct3": 1,  "funct7": -1, "fmt": "B"},
    "BLT":    {"opcode": 0x63, "funct3": 4,  "funct7": -1, "fmt": "B"},
    "BGE":    {"opcode": 0x63, "funct3": 5,  "funct7": -1, "fmt": "B"},
    "BLTU":   {"opcode": 0x63, "funct3": 6,  "funct7": -1, "fmt": "B"},
    "BGEU":   {"opcode": 0x63, "funct3": 7,  "funct7": -1, "fmt": "B"},
    "LB":     {"opcode": 0x03, "funct3": 0,  "funct7": -1, "fmt": "I"},
    "LH":     {"opcode": 0x03, "funct3": 1,  "funct7": -1, "fmt": "I"},
    "LW":     {"opcode": 0x03, "funct3": 2,  "funct7": -1, "fmt": "I"},
    "LBU":    {"opcode": 0x03, "funct3": 4,  "funct7": -1, "fmt": "I"},
    "LHU":    {"opcode": 0x03, "funct3": 5,  "funct7": -1, "fmt": "I"},
    "SB":     {"opcode": 0x23, "funct3": 0,  "funct7": -1, "fmt": "S"},
    "SH":     {"opcode": 0x23, "funct3": 1,  "funct7": -1, "fmt": "S"},
    "SW":     {"opcode": 0x23, "funct3": 2,  "funct7": -1, "fmt": "S"},
    "ADDI":   {"opcode": 0x13, "funct3": 0,  "funct7": -1, "fmt": "I"},
    "SLTI":   {"opcode": 0x13, "funct3": 2,  "funct7": -1, "fmt": "I"},
    "SLTIU":  {"opcode": 0x13, "funct3": 3,  "funct7": -1, "fmt": "I"},
    "XORI":   {"opcode": 0x13, "funct3": 4,  "funct7": -1, "fmt": "I"},
    "ORI":    {"opcode": 0x13, "funct3": 6,  "funct7": -1, "fmt": "I"},
    "ANDI":   {"opcode": 0x13, "funct3": 7,  "funct7": -1, "fmt": "I"},
    "SLLI":   {"opcode": 0x13, "funct3": 1,  "funct7": 0,   "fmt": "I"},
    "SRLI":   {"opcode": 0x13, "funct3": 5,  "funct7": 0,   "fmt": "I"},
    "SRAI":   {"opcode": 0x13, "funct3": 5,  "funct7": 0x20,"fmt": "I"},
    "ADD":    {"opcode": 0x33, "funct3": 0,  "funct7": 0,   "fmt": "R"},
    "SUB":    {"opcode": 0x33, "funct3": 0,  "funct7": 0x20,"fmt": "R"},
    "SLL":    {"opcode": 0x33, "funct3": 1,  "funct7": 0,   "fmt": "R"},
    "SLT":    {"opcode": 0x33, "funct3": 2,  "funct7": 0,   "fmt": "R"},
    "SLTU":   {"opcode": 0x33, "funct3": 3,  "funct7": 0,   "fmt": "R"},
    "XOR":    {"opcode": 0x33, "funct3": 4,  "funct7": 0,   "fmt": "R"},
    "SRL":    {"opcode": 0x33, "funct3": 5,  "funct7": 0,   "fmt": "R"},
    "SRA":    {"opcode": 0x33, "funct3": 5,  "funct7": 0x20,"fmt": "R"},
    "OR":     {"opcode": 0x33, "funct3": 6,  "funct7": 0,   "fmt": "R"},
    "AND":    {"opcode": 0x33, "funct3": 7,  "funct7": 0,   "fmt": "R"},
}

BASIC16 = ["ADD","SUB","AND","OR","XOR","SLT","SLTU","SLL","SRL","SRA",
           "ADDI","LW","SW","BEQ","JAL","LUI"]


def _bv_set(code, val, lo, width):
    """把 val 的低 width 位写入 code 的 [lo, lo+width) 位区间"""
    mask = 0
    for i in range(width):
        mask = mask | (1 << (lo + i))
    kept = code & (~mask)
    twisted = 0
    for i in range(width):
        if (val >> i) & 1:
            twisted = twisted | (1 << (lo + i))
    return kept | twisted


def encode_r(rd, rs1, rs2, funct3, funct7, opcode=0x33):
    code = 0
    code = _bv_set(code, rs2, 20, 5)
    code = _bv_set(code, rs1, 15, 5)
    code = _bv_set(code, funct3, 12, 3)
    code = _bv_set(code, rd, 7, 5)
    code = _bv_set(code, funct7, 25, 7)
    code = _bv_set(code, opcode, 0, 7)
    return code


def encode_i(rd, rs1, imm, funct3, opcode=0x13):
    code = 0
    code = _bv_set(code, imm & 0xFFF, 20, 12)
    code = _bv_set(code, rs1, 15, 5)
    code = _bv_set(code, funct3, 12, 3)
    code = _bv_set(code, rd, 7, 5)
    code = _bv_set(code, opcode, 0, 7)
    return code


def encode_s(rs1, rs2, imm, funct3, opcode=0x23):
    code = 0
    lo5 = imm & 0x1F
    hi7 = (imm >> 5) & 0x7F
    code = _bv_set(code, lo5, 7, 5)
    code = _bv_set(code, funct3, 12, 3)
    code = _bv_set(code, rs1, 15, 5)
    code = _bv_set(code, rs2, 20, 5)
    code = _bv_set(code, hi7, 25, 7)
    code = _bv_set(code, opcode, 0, 7)
    return code


def encode_b(rs1, rs2, imm, funct3, opcode=0x63):
    code = 0
    b12   = (imm >> 12) & 1
    b11   = (imm >> 11) & 1
    b10_5 = (imm >> 5) & 0x3F
    b4_1  = (imm >> 1) & 0xF
    code = _bv_set(code, b4_1, 8, 4)
    code = _bv_set(code, funct3, 12, 3)
    code = _bv_set(code, rs1, 15, 5)
    code = _bv_set(code, rs2, 20, 5)
    code = _bv_set(code, b10_5, 25, 6)
    code = _bv_set(code, b11, 7, 1)
    code = _bv_set(code, b12, 31, 1)
    code = _bv_set(code, opcode, 0, 7)
    return code


def encode_u(rd, imm, opcode=0x37):
    """U 型：imm 是 20 位值，放到 bits [31:12]"""
    shifted = 0
    for i in range(20):
        if (imm >> i) & 1:
            shifted = shifted | (1 << (i + 12))
    code = _bv_set(shifted, rd, 7, 5)
    code = _bv_set(code, opcode, 0, 7)
    return code


def encode_j(rd, imm, opcode=0x6F):
    code = 0
    b20    = (imm >> 20) & 1
    b10_1  = (imm >> 1) & 0x3FF
    b11    = (imm >> 11) & 1
    b19_12 = (imm >> 12) & 0xFF
    code = _bv_set(code, b19_12, 12, 8)
    code = _bv_set(code, b11, 20, 1)
    code = _bv_set(code, b10_1, 21, 10)
    code = _bv_set(code, b20, 31, 1)
    code = _bv_set(code, rd, 7, 5)
    code = _bv_set(code, opcode, 0, 7)
    return code


def assemble_line(asm_str):
    tok = asm_str.replace(",", " ").replace("(", " ").replace(")", " ").split()
    mnem = tok[0].upper()
    if mnem not in RV32I:
        raise ValueError(f"未知助记符: {mnem}")
    info = RV32I[mnem]
    fmt  = info["fmt"]
    f3   = info["funct3"]
    f7   = info["funct7"]
    opc  = info["opcode"]

    if fmt == "R":
        rd = int(tok[1][1:]); rs1 = int(tok[2][1:]); rs2 = int(tok[3][1:])
        return encode_r(rd, rs1, rs2, f3, f7 if f7 >= 0 else 0, opc)
    if fmt == "I":
        rd = int(tok[1][1:])
        if tok[2].startswith("x"):
            rs1 = int(tok[2][1:])
            imm = int(tok[3], 0) if tok[3].startswith("0x") else int(tok[3])
        else:
            # LW x5, 0(x6)  →  tok = ["LW", "x5", "0", "x6"]
            imm = int(tok[2], 0) if tok[2].startswith("0x") else int(tok[2])
            rs1_s = tok[3]
            if rs1_s.startswith("x"): rs1_s = rs1_s[1:]
            rs1_s = rs1_s.rstrip(")")
            rs1 = int(rs1_s)
        return encode_i(rd, rs1, imm, f3, opc)
    if fmt == "S":
        # SW x5, 8(x6)  →  tok = ["SW", "x5", "8", "x6"]
        rs2 = int(tok[1][1:])
        imm = int(tok[2], 0) if tok[2].startswith("0x") else int(tok[2])
        rs1_s = tok[3]
        if rs1_s.startswith("x"): rs1_s = rs1_s[1:]
        rs1_s = rs1_s.rstrip(")")
        rs1 = int(rs1_s)
        return encode_s(rs1, rs2, imm, f3, opc)
    if fmt == "B":
        rs1 = int(tok[1][1:]); rs2 = int(tok[2][1:])
        imm = int(tok[3], 0) if tok[3].startswith("0x") else int(tok[3])
        return encode_b(rs1, rs2, imm, f3, opc)
    if fmt == "U":
        rd = int(tok[1][1:])
        imm = int(tok[2], 0) if tok[2].startswith("0x") else int(tok[2])
        return encode_u(rd, imm, opc)
    if fmt == "J":
        rd = int(tok[1][1:])
        imm = int(tok[2], 0) if tok[2].startswith("0x") else int(tok[2])
        return encode_j(rd, imm, opc)
    raise ValueError(f"未支持格式: {fmt}")


def decode_to_str(word):
    """机器码 → 可读的汇编字符串（用于自测 round-trip）"""
    opcode = word & 0x7F
    rd     = (word >> 7) & 0x1F
    funct3 = (word >> 12) & 0x7
    rs1    = (word >> 15) & 0x1F
    rs2    = (word >> 20) & 0x1F
    funct7 = (word >> 25) & 0x7F
    if opcode == 0x33:
        for mnem, info in RV32I.items():
            if info["fmt"] == "R" and info["funct3"] == funct3 and info["funct7"] == funct7:
                return f"{mnem} x{rd}, x{rs1}, x{rs2}"
    elif opcode in (0x13, 0x03, 0x67):
        for mnem, info in RV32I.items():
            if info["opcode"] == opcode and info["fmt"] == "I" and info["funct3"] == funct3:
                imm = (word >> 20) & 0xFFF
                if imm >= 0x800: imm = imm - 0x1000
                base = (word >> 15) & 0x1F
                if opcode == 0x03:
                    return f"{mnem} x{rd}, {imm}(x{base})"
                return f"{mnem} x{rd}, x{rs1}, {imm}"
    elif opcode == 0x23:
        for mnem, info in RV32I.items():
            if info["fmt"] == "S" and info["funct3"] == funct3:
                imm = ((word >> 25) << 5) | ((word >> 7) & 0x1F)
                return f"{mnem} x{rs2}, {imm}(x{rs1})"
    elif opcode == 0x63:
        for mnem, info in RV32I.items():
            if info["fmt"] == "B" and info["funct3"] == funct3:
                imm = (((word >> 31) & 1) << 12) | (((word >> 7) & 1) << 11) | \
                      (((word >> 25) & 0x3F) << 5) | (((word >> 8) & 0xF) << 1)
                if imm >= 0x1000: imm = imm - 0x2000
                return f"{mnem} x{rs1}, x{rs2}, {imm}"
    elif opcode in (0x37, 0x17):
        imm = word & 0xFFFFF000
        if opcode == 0x37:
            return f"LUI x{rd}, 0x{imm >> 12:05X}"
        return f"AUIPC x{rd}, 0x{imm >> 12:05X}"
    elif opcode == 0x6F:
        imm = (((word >> 31) & 1) << 20) | (((word >> 21) & 0x3FF) << 1) | \
              (((word >> 20) & 1) << 11) | (((word >> 12) & 0xFF) << 12)
        if imm >= 0x100000: imm = imm - 0x200000
        return f"JAL x{rd}, {imm}"
    return f"??? 0x{word:08X}"


if __name__ == "__main__":
    cases = [
        ("ADD x5, x6, x7",    0x007302B3),
        ("SUB x5, x6, x7",    0x407302B3),
        ("AND x5, x6, x7",    0x007372B3),
        ("OR  x9, x10, x11",  0x00B564B3),
        ("XOR x12, x13, x14", 0x00E6C633),
        ("SLT x5, x6, x7",    0x007322B3),
        ("SLTU x5, x6, x7",   0x007332B3),
        ("SLL x5, x6, x7",    0x007312B3),
        ("SRL x11, x24, x27", 0x01BC55B3),
        ("SRA x11, x24, x27", 0x41BC55B3),
        ("ADDI x5, x6, 100",  0x06430293),
        ("LW x5, 0(x6)",      0x00032283),
        ("SW x5, 8(x6)",      0x00532423),
        ("BEQ x5, x6, 16",    0x00628863),
        ("JAL x8, 1024",      0x4000046F),
        ("LUI x5, 0x12345",   0x123452B7),
    ]
    pass_n = 0
    for asm, expected in cases:
        got = assemble_line(asm)
        ok = got == expected
        mark = "PASS" if ok else "FAIL"
        if ok: pass_n = pass_n + 1
        print(f"  [{mark}] {asm:22s} -> 0x{got:08X} (期望 0x{expected:08X})")
    print(f"\n{pass_n}/{len(cases)} PASS")

    print("\n往返测试 (decode):")
    rt_ok = 0
    for asm, _ in cases:
        w = assemble_line(asm)
        dec = decode_to_str(w)
        if dec and not dec.startswith("???"):
            rt_ok = rt_ok + 1
            print(f"  {asm:22s} -> 0x{w:08X} -> {dec}")
    print(f"  round-trip: {rt_ok}/{len(cases)}")
