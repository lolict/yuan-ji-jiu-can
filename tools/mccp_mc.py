#!/usr/bin/env python3
"""
MCCP Machine Code CLI tool
Encode semantic ID + operands -> machine code hex
Decode machine code hex -> semantic ID + operands

Usage:
    python3 mccp_mc.py encode <semantic_id> <isa> [operands...]
    python3 mccp_mc.py decode <hex_string> <isa>
    python3 mccp_mc.py lookup <mnemonic> [isa]
    python3 mccp_mc.py list [semantic_id]
"""

import json
import sys
import os
import struct

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
JSON_PATH = os.path.join(os.path.dirname(SCRIPT_DIR), "docs", "machine_code_db.json")

def load_db():
    with open(JSON_PATH, "r") as f:
        return json.load(f)

def sign_extend(value, bits):
    if value & (1 << (bits - 1)):
        return value - (1 << bits)
    return value

def to_little_endian(hex_str):
    """Convert big-endian hex to little-endian bytes."""
    raw = bytes.fromhex(hex_str.replace(" ", ""))
    return raw[::-1].hex().upper()

# ---- Encoders ----

def encode_x86_64(semantic_id, operands):
    """Encode x86-64 instruction."""
    db = load_db()
    encs = db["encodings"].get(semantic_id, {}).get("x86-64", {})
    if not encs:
        return None, f"No x86-64 encoding for {semantic_id}"
    
    if semantic_id == "ADD" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 01 {modrm:02X}", None
    elif semantic_id == "MOV" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 89 {modrm:02X}", None
    elif semantic_id == "SUB" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 29 {modrm:02X}", None
    elif semantic_id == "AND" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 21 {modrm:02X}", None
    elif semantic_id == "OR" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 09 {modrm:02X}", None
    elif semantic_id == "XOR" and len(operands) >= 2:
        rd, rs = operands[0], operands[1]
        rd_num = int(rd.replace("r", "") or "0")
        rs_num = int(rs.replace("r", "") or "0")
        modrm = 0xC0 | (rs_num << 3) | rd_num
        return f"48 31 {modrm:02X}", None
    elif semantic_id == "RET":
        return "C3", None
    
    return None, f"Unsupported operands for x86-64 {semantic_id}"

def encode_arm64(semantic_id, operands):
    """Encode ARM64 instruction (AArch64)."""
    if semantic_id == "ADD" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rn = int(operands[1].replace("x", ""))
        rm = int(operands[2].replace("x", ""))
        if rd < 0 or rd > 31 or rn < 0 or rn > 31 or rm < 0 or rm > 31:
            return None, "Register number out of range (0-31)"
        bits = (1 << 31) | (0 << 30) | (0 << 29) | (0x8B << 21) | (rm << 16) | (0 << 10) | (rn << 5) | rd
        return f"{bits:08X}", None
    elif semantic_id == "SUB" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rn = int(operands[1].replace("x", ""))
        rm = int(operands[2].replace("x", ""))
        bits = (1 << 31) | (1 << 30) | (0 << 29) | (0x8B << 21) | (rm << 16) | (0 << 10) | (rn << 5) | rd
        return f"{bits:08X}", None
    elif semantic_id == "ADDI" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rn = int(operands[1].replace("x", ""))
        imm = int(operands[2])
        if imm < 0 or imm > 0xFFF:
            return None, "Immediate out of range (0-4095)"
        bits = (1 << 31) | (0 << 30) | (0 << 29) | (0x24 << 23) | (0 << 22) | (imm << 10) | (rn << 5) | rd
        return f"{bits:08X}", None
    elif semantic_id == "LOAD" and len(operands) >= 2:
        rt = int(operands[0].replace("x", ""))
        base = operands[1].strip("[]")
        rn = int(base.replace("x", ""))
        offset = 0
        if len(operands) > 2:
            offset = int(operands[2])
        elif "+" in operands[1]:
            pass
        if offset % 8 != 0:
            return None, "Offset must be 8-byte aligned"
        imm12 = offset // 8
        if imm12 < 0 or imm12 > 0xFFF:
            return None, "Offset too large"
        bits = (3 << 30) | (0x7C0 << 22) | (0x4 << 20) | (imm12 << 10) | (rn << 5) | rt
        return f"{bits:08X}", None
    elif semantic_id == "STORE" and len(operands) >= 2:
        rt = int(operands[0].replace("x", ""))
        base = operands[1].strip("[]")
        rn = int(base.replace("x", ""))
        offset = 0
        if len(operands) > 2:
            offset = int(operands[2])
        if offset % 8 != 0:
            return None, "Offset must be 8-byte aligned"
        imm12 = offset // 8
        if imm12 < 0 or imm12 > 0xFFF:
            return None, "Offset too large"
        bits = (3 << 30) | (0x7C0 << 22) | (0x0 << 20) | (imm12 << 10) | (rn << 5) | rt
        return f"{bits:08X}", None
    elif semantic_id == "RET":
        return "D65F03C0", None
    
    return None, f"Unsupported or wrong operands for ARM64 {semantic_id}"

def encode_riscv32(semantic_id, operands):
    """Encode RISC-V RV32I instruction."""
    if semantic_id == "ADD" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        rs2 = int(operands[2].replace("x", ""))
        bits = (0x33 << 0) | (rd << 7) | (0x0 << 12) | (rs1 << 15) | (rs2 << 20) | (0x0 << 25)
        return f"{bits:08X}", None
    elif semantic_id == "ADDI" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        imm = int(operands[2])
        if imm < -2048 or imm > 2047:
            return None, "Immediate out of 12-bit signed range"
        imm &= 0xFFF
        bits = (0x13 << 0) | (rd << 7) | (0x0 << 12) | (rs1 << 15) | (imm << 20)
        return f"{bits:08X}", None
    elif semantic_id == "SUB" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        rs2 = int(operands[2].replace("x", ""))
        bits = (0x33 << 0) | (rd << 7) | (0x0 << 12) | (rs1 << 15) | (rs2 << 20) | (0x20 << 25)
        return f"{bits:08X}", None
    elif semantic_id == "AND" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        rs2 = int(operands[2].replace("x", ""))
        bits = (0x33 << 0) | (rd << 7) | (0x7 << 12) | (rs1 << 15) | (rs2 << 20) | (0x0 << 25)
        return f"{bits:08X}", None
    elif semantic_id == "OR" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        rs2 = int(operands[2].replace("x", ""))
        bits = (0x33 << 0) | (rd << 7) | (0x6 << 12) | (rs1 << 15) | (rs2 << 20) | (0x0 << 25)
        return f"{bits:08X}", None
    elif semantic_id == "XOR" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        rs2 = int(operands[2].replace("x", ""))
        bits = (0x33 << 0) | (rd << 7) | (0x4 << 12) | (rs1 << 15) | (rs2 << 20) | (0x0 << 25)
        return f"{bits:08X}", None
    elif semantic_id == "LOAD" and len(operands) >= 2:
        rd = int(operands[0].replace("x", ""))
        rest = operands[1]
        if "(" in rest and ")" in rest:
            parts = rest.split("(")
            offset = int(parts[0]) if parts[0] else 0
            rs1 = int(parts[1].rstrip(")").replace("x", ""))
        elif "[" in rest and "]" in rest:
            parts = rest.split("[")
            offset = int(parts[0]) if parts[0] else 0
            rs1 = int(parts[1].rstrip("]").replace("x", ""))
        else:
            rs1 = int(rest.replace("x", ""))
            offset = 0
        if offset < -2048 or offset > 2047:
            return None, "Offset out of 12-bit signed range"
        offset &= 0xFFF
        bits = (0x03 << 0) | (rd << 7) | (0x2 << 12) | (rs1 << 15) | (offset << 20)
        return f"{bits:08X}", None
    elif semantic_id == "STORE" and len(operands) >= 2:
        rs2 = int(operands[0].replace("x", ""))
        rest = operands[1]
        if "(" in rest and ")" in rest:
            parts = rest.split("(")
            offset = int(parts[0]) if parts[0] else 0
            rs1 = int(parts[1].rstrip(")").replace("x", ""))
        elif "[" in rest and "]" in rest:
            parts = rest.split("[")
            offset = int(parts[0]) if parts[0] else 0
            rs1 = int(parts[1].rstrip("]").replace("x", ""))
        else:
            rs1 = int(rest.replace("x", ""))
            offset = 0
        if offset < -2048 or offset > 2047:
            return None, "Offset out of 12-bit signed range"
        offset &= 0xFFF
        bits = (0x23 << 0) | ((offset & 0x1F) << 7) | (0x2 << 12) | (rs1 << 15) | (rs2 << 20) | ((offset >> 5) << 25)
        return f"{bits:08X}", None
    elif semantic_id == "BNE" and len(operands) >= 3:
        rs1 = int(operands[0].replace("x", ""))
        rs2 = int(operands[1].replace("x", ""))
        offset = int(operands[2])
        if offset < -4096 or offset > 4094 or offset % 2 != 0:
            return None, "Offset out of 13-bit signed range or misaligned"
        imm12 = (offset >> 12) & 1
        imm11 = (offset >> 11) & 1
        imm10_5 = (offset >> 5) & 0x3F
        imm4_1 = (offset >> 1) & 0xF
        bits = (0x63 << 0) | (imm11 << 7) | (imm4_1 << 8) | (0x1 << 12) | (rs1 << 15) | (rs2 << 20) | (imm10_5 << 25) | (imm12 << 31)
        return f"{bits:08X}", None
    elif semantic_id == "JAL" and len(operands) >= 2:
        rd = int(operands[0].replace("x", ""))
        offset = int(operands[1])
        if offset < -1048576 or offset > 1048574 or offset % 2 != 0:
            return None, "Offset out of 21-bit signed range or misaligned"
        imm20 = (offset >> 20) & 1
        imm10_1 = (offset >> 1) & 0x3FF
        imm11 = (offset >> 11) & 1
        imm19_12 = (offset >> 12) & 0xFF
        bits = (0x6F << 0) | (rd << 7) | (imm19_12 << 12) | (imm11 << 20) | (imm10_1 << 21) | (imm20 << 31)
        return f"{bits:08X}", None
    elif semantic_id == "JALR" and len(operands) >= 3:
        rd = int(operands[0].replace("x", ""))
        rs1 = int(operands[1].replace("x", ""))
        offset = int(operands[2])
        if offset < -2048 or offset > 2047:
            return None, "Offset out of 12-bit signed range"
        offset &= 0xFFF
        bits = (0x67 << 0) | (rd << 7) | (0x0 << 12) | (rs1 << 15) | (offset << 20)
        return f"{bits:08X}", None
    elif semantic_id == "LUI" and len(operands) >= 2:
        rd = int(operands[0].replace("x", ""))
        imm20 = int(operands[1])
        bits = (0x37 << 0) | (rd << 7) | (imm20 << 12)
        return f"{bits:08X}", None
    elif semantic_id == "RET":
        return "00008067", None
    elif semantic_id == "NOP":
        return "00000013", None
    
    return None, f"Unsupported or wrong operands for RISC-V {semantic_id}"

def encode_mips32(semantic_id, operands):
    """Encode MIPS32 instruction."""
    def reg_num(r):
        r = r.strip("$")
        if r.isdigit():
            return int(r)
        names = {"zero":0,"at":1,"v0":2,"v1":3,"a0":4,"a1":5,"a2":6,"a3":7,
                 "t0":8,"t1":9,"t2":10,"t3":11,"t4":12,"t5":13,"t6":14,"t7":15,
                 "s0":16,"s1":17,"s2":18,"s3":19,"s4":20,"s5":21,"s6":22,"s7":23,
                 "t8":24,"t9":25,"k0":26,"k1":27,"gp":28,"sp":29,"fp":30,"ra":31}
        return names.get(r, 0)
    
    if semantic_id == "ADD" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x20)
        return f"{bits:08X}", None
    elif semantic_id == "ADDU" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x21)
        return f"{bits:08X}", None
    elif semantic_id == "SUB" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x22)
        return f"{bits:08X}", None
    elif semantic_id == "AND" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x24)
        return f"{bits:08X}", None
    elif semantic_id == "OR" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x25)
        return f"{bits:08X}", None
    elif semantic_id == "XOR" and len(operands) >= 3:
        rd = reg_num(operands[0])
        rs = reg_num(operands[1])
        rt = reg_num(operands[2])
        bits = (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (0 << 6) | (0x26)
        return f"{bits:08X}", None
    elif semantic_id == "ADDI" and len(operands) >= 3:
        rt = reg_num(operands[0])
        rs = reg_num(operands[1])
        imm = int(operands[2])
        if imm < -32768 or imm > 32767:
            return None, "Immediate out of 16-bit signed range"
        imm &= 0xFFFF
        bits = (0x08 << 26) | (rs << 21) | (rt << 16) | imm
        return f"{bits:08X}", None
    elif semantic_id == "LW" and len(operands) >= 2:
        rt = reg_num(operands[0])
        rest = operands[1]
        if "(" in rest and ")" in rest:
            parts = rest.split("(")
            offset = int(parts[0]) if parts[0] else 0
            rs = reg_num(parts[1].rstrip(")"))
        else:
            rs = reg_num(rest)
            offset = 0
        if offset < -32768 or offset > 32767:
            return None, "Offset out of 16-bit signed range"
        offset &= 0xFFFF
        bits = (0x23 << 26) | (rs << 21) | (rt << 16) | offset
        return f"{bits:08X}", None
    elif semantic_id == "SW" and len(operands) >= 2:
        rt = reg_num(operands[0])
        rest = operands[1]
        if "(" in rest and ")" in rest:
            parts = rest.split("(")
            offset = int(parts[0]) if parts[0] else 0
            rs = reg_num(parts[1].rstrip(")"))
        else:
            rs = reg_num(rest)
            offset = 0
        if offset < -32768 or offset > 32767:
            return None, "Offset out of 16-bit signed range"
        offset &= 0xFFFF
        bits = (0x2B << 26) | (rs << 21) | (rt << 16) | offset
        return f"{bits:08X}", None
    elif semantic_id == "LUI" and len(operands) >= 2:
        rt = reg_num(operands[0])
        imm = int(operands[1])
        bits = (0x0F << 26) | (0 << 21) | (rt << 16) | (imm & 0xFFFF)
        return f"{bits:08X}", None
    elif semantic_id == "BNE" and len(operands) >= 3:
        rs = reg_num(operands[0])
        rt = reg_num(operands[1])
        offset = int(operands[2])
        if offset < -131072 or offset > 131071 or offset % 2 != 0:
            return None, "Offset out of 18-bit signed range or misaligned"
        offset = (offset >> 1) & 0xFFFF
        bits = (0x05 << 26) | (rs << 21) | (rt << 16) | offset
        return f"{bits:08X}", None
    elif semantic_id == "BEQ" and len(operands) >= 3:
        rs = reg_num(operands[0])
        rt = reg_num(operands[1])
        offset = int(operands[2])
        if offset < -131072 or offset > 131071 or offset % 2 != 0:
            return None, "Offset out of 18-bit signed range or misaligned"
        offset = (offset >> 1) & 0xFFFF
        bits = (0x04 << 26) | (rs << 21) | (rt << 16) | offset
        return f"{bits:08X}", None
    elif semantic_id == "J" and len(operands) >= 1:
        target = int(operands[0])
        bits = (0x02 << 26) | ((target >> 1) & 0x3FFFFFF)
        return f"{bits:08X}", None
    elif semantic_id == "JAL" and len(operands) >= 1:
        target = int(operands[0])
        bits = (0x03 << 26) | ((target >> 1) & 0x3FFFFFF)
        return f"{bits:08X}", None
    elif semantic_id == "JR" and len(operands) >= 1:
        rs = reg_num(operands[0])
        bits = (0 << 26) | (rs << 21) | (0 << 11) | (0x08)
        return f"{bits:08X}", None
    elif semantic_id == "NOP":
        return "00000000", None
    
    return None, f"Unsupported or wrong operands for MIPS32 {semantic_id}"

# ---- Decoders (ISA-specific) ----

def decode_riscv32(hex_str):
    """Decode RV32I machine code, return (semantic_id, operands)."""
    raw = hex_str.replace(" ", "")
    bits = int(raw, 16)
    opcode = bits & 0x7F
    rd = (bits >> 7) & 0x1F
    funct3 = (bits >> 12) & 0x7
    rs1 = (bits >> 15) & 0x1F
    rs2 = (bits >> 20) & 0x1F
    funct7 = (bits >> 25) & 0x7F
    
    if opcode == 0x13 and funct3 == 0x0 and rd == 0 and rs1 == 0 and (bits >> 20) == 0:
        return "NOP", []
    
    if opcode == 0x33:
        if funct3 == 0x0:
            if funct7 == 0x00:
                return "ADD", [f"x{rd}", f"x{rs1}", f"x{rs2}"]
            elif funct7 == 0x20:
                return "SUB", [f"x{rd}", f"x{rs1}", f"x{rs2}"]
        elif funct3 == 0x7 and funct7 == 0x00:
            return "AND", [f"x{rd}", f"x{rs1}", f"x{rs2}"]
        elif funct3 == 0x6 and funct7 == 0x00:
            return "OR", [f"x{rd}", f"x{rs1}", f"x{rs2}"]
        elif funct3 == 0x4 and funct7 == 0x00:
            return "XOR", [f"x{rd}", f"x{rs1}", f"x{rs2}"]
    elif opcode == 0x13:
        if funct3 == 0x0:
            imm = sign_extend((bits >> 20) & 0xFFF, 12)
            return "ADDI", [f"x{rd}", f"x{rs1}", str(imm)]
    elif opcode == 0x03:
        if funct3 == 0x2:
            imm = sign_extend((bits >> 20) & 0xFFF, 12)
            return "LOAD", [f"x{rd}", f"{imm}(x{rs1})"]
    elif opcode == 0x23:
        if funct3 == 0x2:
            imm = sign_extend((((bits >> 25) & 0x7F) << 5) | ((bits >> 7) & 0x1F), 12)
            return "STORE", [f"x{rs2}", f"{imm}(x{rs1})"]
    elif opcode == 0x63:
        if funct3 == 0x1:
            imm = sign_extend(
                ((bits >> 31) << 12) | (((bits >> 7) & 1) << 11) |
                (((bits >> 25) & 0x3F) << 5) | (((bits >> 8) & 0xF) << 1), 13)
            return "BNE", [f"x{rs1}", f"x{rs2}", str(imm)]
    elif opcode == 0x6F:
        imm = sign_extend(
            ((bits >> 31) << 20) | (((bits >> 21) & 0x3FF) << 1) |
            (((bits >> 20) & 1) << 11) | (((bits >> 12) & 0xFF) << 12), 21)
        return "JAL", [f"x{rd}", str(imm)]
    elif opcode == 0x67 and funct3 == 0x0:
        imm = sign_extend((bits >> 20) & 0xFFF, 12)
        if rd == 0 and rs1 == 1 and imm == 0:
            return "RET", []
        return "JALR", [f"x{rd}", f"{imm}(x{rs1})"]
    elif opcode == 0x37:
        return "LUI", [f"x{rd}", str((bits >> 12) & 0xFFFFF)]
    return f"UNKNOWNopcode={opcode:02x}", []

# ---- Main CLI ----

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 mccp_mc.py <cmd> [args...]")
        sys.exit(1)
    
    cmd = sys.argv[1]
    
    # Map common mnemonics to semantic IDs
    mnemonic_to_semantic = {
        "LOAD": "LOAD", "LW": "LOAD", "LD": "LOAD", "LB": "LOAD", "LBU": "LOAD", "LH": "LOAD",
        "STORE": "STORE", "SW": "STORE", "SD": "STORE", "SB": "STORE", "SH": "STORE",
        "ADD": "ADD", "ADDU": "ADD",
        "ADDI": "ADDI", "SUBI": "SUB",
        "SUB": "SUB", "SUBU": "SUB",
        "AND": "AND", "OR": "OR", "XOR": "XOR",
        "JMP": "JMP", "J": "JMP",
        "BNE": "BNE", "BEQ": "BNE",
        "CALL": "CALL", "JAL": "CALL", "BL": "CALL", "JALR": "CALL",
        "RET": "RET",
        "MOV": "MOV",
        "MOVI": "MOVI",
        "CMP": "CMP",
        "SLL": "SLL", "SRL": "SRL", "SRA": "SRA",
        "SLT": "SLT",
        "NOP": "NOP",
        "LUI": "MOVI",
    }
    
    if cmd == "encode":
        if len(sys.argv) < 4:
            print("Usage: encode <semantic_id/mnemonic> <isa> [operands...]")
            sys.exit(1)
        raw_sem = sys.argv[2].upper()
        sem_id = mnemonic_to_semantic.get(raw_sem, raw_sem)
        isa = sys.argv[3]
        operands = sys.argv[4:]
        
        encoders = {
            "x86-64": encode_x86_64,
            "ARM64": encode_arm64,
            "RISC-V": encode_riscv32,
            "MIPS32": encode_mips32,
        }
        
        if isa not in encoders:
            print(f"Unsupported ISA: {isa}")
            sys.exit(1)
        
        result, err = encoders[isa](sem_id, operands)
        if err:
            print(f"Error: {err}")
            sys.exit(1)
        print(result)
    
    elif cmd == "decode":
        if len(sys.argv) < 4:
            print("Usage: decode <hex_string> <isa>")
            sys.exit(1)
        hex_str = sys.argv[2]
        isa = sys.argv[3]
        
        if isa.startswith("RISC-V"):
            sem_id, operands = decode_riscv32(hex_str)
        else:
            print(f"Decode not implemented for {isa}")
            sys.exit(1)
        
        print(f"{sem_id} {', '.join(operands)}")
    
    elif cmd == "lookup":
        if len(sys.argv) < 3:
            print("Usage: lookup <mnemonic> [isa]")
            sys.exit(1)
        mnemonic = sys.argv[2].upper()
        isa_filter = sys.argv[3] if len(sys.argv) > 3 else None
        db = load_db()
        
        for sem_id, isas in db.get("encodings", {}).items():
            for isa, encs in isas.items():
                if isa_filter and isa != isa_filter:
                    continue
                for name, enc in encs.items():
                    if name.startswith(mnemonic.lower()) or mnemonic in enc.get("asm", "").upper():
                        print(f"{sem_id:8s} {isa:10s} {enc['hex']:20s} {enc.get('asm', '')}")
    
    elif cmd == "list":
        db = load_db()
        sem_filter = sys.argv[2].upper() if len(sys.argv) > 2 else None
        
        for sem_id, isas in db.get("encodings", {}).items():
            if sem_filter and sem_id != sem_filter:
                continue
            sem_info = db.get("semantic_ids", {}).get(sem_id, {})
            desc = sem_info.get("desc", "")
            print(f"=== {sem_id} ({desc}) ===")
            for isa, encs in isas.items():
                print(f"  [{isa}]")
                for name, enc in encs.items():
                    print(f"    {name:25s} {enc.get('hex', 'N/A'):30s} {enc.get('asm', '')}")
    
    else:
        print(f"Unknown command: {cmd}")
        sys.exit(1)

if __name__ == "__main__":
    main()
