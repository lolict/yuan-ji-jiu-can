#!/usr/bin/env python3
# ============================================================
# MCODE Stone · 双向转换引擎
#
# 编码：语义 ID + 操作数 → 32 位机器码
# 解码：32 位机器码 → 语义 ID + 操作数
#
# 身份落款：刘楚恬和我
# ============================================================

import sys
import os
import json
import yaml
import re

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OPS_FILE = os.path.join(SCRIPT_DIR, "riscv32i_ops.yaml")

with open(OPS_FILE, "r", encoding="utf-8") as f:
    OPS_DB = yaml.safe_load(f)

INST_LIST = OPS_DB["instructions"]
ID2INST = {inst["id"]: inst for inst in INST_LIST}
MNE2INST = {inst["mnemonic"]: inst for inst in INST_LIST}

def reg_to_num(reg_str):
    if isinstance(reg_str, int):
        return reg_str
    s = str(reg_str).strip().lower()
    if s.startswith("x"):
        return int(s[1:])
    return int(s)

def parse_imm(imm_str, bits):
    if isinstance(imm_str, int):
        val = imm_str
    else:
        s = str(imm_str).strip()
        if s.lower().startswith("0x"):
            val = int(s, 16)
        elif s.lower().startswith("0b"):
            val = int(s, 2)
        else:
            val = int(s)
    mask = (1 << bits) - 1
    return val & mask

def bit(val, hi, lo):
    width = hi - lo + 1
    return (val >> lo) & ((1 << width) - 1)

def sign_extend(val, bits):
    if val & (1 << (bits - 1)):
        return val - (1 << bits)
    return val

def to_bin32(val):
    return format(val & 0xFFFFFFFF, "032b")

def to_hex32(val):
    return "0x" + format(val & 0xFFFFFFFF, "08x")

def encode_R(inst, ops):
    funct7 = int(inst["funct7"], 2)
    funct3 = int(inst["funct3"], 2)
    rd  = reg_to_num(ops[0])
    rs1 = reg_to_num(ops[1])
    rs2 = reg_to_num(ops[2])
    return (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | int(inst["opcode"], 2)

def encode_I(inst, ops):
    funct3 = int(inst["funct3"], 2)
    opcode = inst["opcode"]
    if opcode == "0010011":
        rd  = reg_to_num(ops[0])
        rs1 = reg_to_num(ops[1])
        imm = parse_imm(ops[2], 12)
    elif opcode == "0000011":
        rd  = reg_to_num(ops[0])
        imm = parse_imm(ops[1], 12)
        rs1 = reg_to_num(ops[2])
    else:
        raise ValueError(f"未知 I-type opcode: {opcode}")
    return (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | int(opcode, 2)

def encode_S(inst, ops):
    funct3 = int(inst["funct3"], 2)
    rs2 = reg_to_num(ops[0])
    imm = parse_imm(ops[1], 12)
    rs1 = reg_to_num(ops[2])
    imm_hi = (imm >> 5) & 0x7F
    lo = imm & 0x1F
    return (imm_hi << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (lo << 7) | int(inst["opcode"], 2)

def encode_B(inst, ops):
    funct3 = int(inst["funct3"], 2)
    rs1 = reg_to_num(ops[0])
    rs2 = reg_to_num(ops[1])
    imm = parse_imm(ops[2], 13)
    b12   = bit(imm, 12, 12)
    b11   = bit(imm, 11, 11)
    b10_5 = bit(imm, 10, 5)
    b4_1  = bit(imm, 4, 1)
    return (b12 << 31) | (b10_5 << 25) | (rs2 << 20) | (rs1 << 15) | \
           (funct3 << 12) | (b4_1 << 8) | (b11 << 7) | int(inst["opcode"], 2)

def encode_U(inst, ops):
    rd  = reg_to_num(ops[0])
    imm = parse_imm(ops[1], 20)
    return (imm << 12) | (rd << 7) | int(inst["opcode"], 2)

def encode_J(inst, ops):
    rd  = reg_to_num(ops[0])
    imm = parse_imm(ops[1], 21)
    b20    = bit(imm, 20, 20)
    b10_1  = bit(imm, 10, 1)
    b11    = bit(imm, 11, 11)
    b19_12 = bit(imm, 19, 12)
    val = (b20 << 31) | (b19_12 << 12) | (b11 << 20) | (b10_1 << 21) | (rd << 7) | int(inst["opcode"], 2)
    return val & 0xFFFFFFFF

ENCODE_TABLE = {
    "R": encode_R, "I": encode_I, "S": encode_S,
    "B": encode_B, "U": encode_U, "J": encode_J,
}

def encode(semantic_id, ops):
    sid = semantic_id.upper().strip()
    inst = ID2INST.get(sid)
    if not inst:
        inst = MNE2INST.get(semantic_id.lower().strip())
    if not inst:
        raise ValueError(f"未知指令: {semantic_id}")
    itype = inst["type"]
    return ENCODE_TABLE[itype](inst, ops)

def decode(machine_code):
    if isinstance(machine_code, str):
        machine_code = int(machine_code, 16) if machine_code.startswith("0x") else int(machine_code, 2)
    opcode = bit(machine_code, 6, 0)
    rd     = bit(machine_code, 11, 7)
    funct3 = bit(machine_code, 14, 12)
    rs1    = bit(machine_code, 19, 15)
    rs2    = bit(machine_code, 24, 20)
    funct7 = bit(machine_code, 31, 25)

    for inst in INST_LIST:
        if int(inst["opcode"], 2) != opcode:
            continue
        if "funct3" in inst and int(inst["funct3"], 2) != funct3:
            continue
        if "funct7" in inst and int(inst["funct7"], 2) != funct7:
            continue

        itype = inst["type"]
        ops = []

        if itype == "R":
            ops = [f"x{rd}", f"x{rs1}", f"x{rs2}"]
        elif itype == "I":
            imm_raw = bit(machine_code, 31, 20)
            imm = sign_extend(imm_raw, 12)
            if opcode == 0b0010011:
                ops = [f"x{rd}", f"x{rs1}", str(imm)]
            elif opcode == 0b0000011:
                ops = [f"x{rd}", str(imm), f"x{rs1}"]
            else:
                ops = [f"x{rd}", f"x{rs1}", str(imm)]
        elif itype == "S":
            imm_hi = bit(machine_code, 31, 25)
            lo     = bit(machine_code, 11, 7)
            imm = sign_extend((imm_hi << 5) | lo, 12)
            ops = [f"x{rs2}", str(imm), f"x{rs1}"]
        elif itype == "B":
            b12   = bit(machine_code, 31, 31)
            b10_5 = bit(machine_code, 30, 25)
            b4_1  = bit(machine_code, 11, 8)
            b11   = bit(machine_code, 7, 7)
            imm = sign_extend((b12 << 12) | (b11 << 11) | (b10_5 << 5) | (b4_1 << 1), 13)
            ops = [f"x{rs1}", f"x{rs2}", str(imm)]
        elif itype == "U":
            imm = bit(machine_code, 31, 12)
            ops = [f"x{rd}", str(imm)]
        elif itype == "J":
            b20    = bit(machine_code, 31, 31)
            b19_12 = bit(machine_code, 19, 12)
            b11    = bit(machine_code, 20, 20)
            b10_1  = bit(machine_code, 30, 21)
            imm = sign_extend((b20 << 20) | (b19_12 << 12) | (b11 << 11) | (b10_1 << 1), 21)
            ops = [f"x{rd}", str(imm)]

        return {
            "id": inst["id"], "mnemonic": inst["mnemonic"],
            "type": itype, "opcode": format(opcode, "07b"),
            "operands": ops, "semantic": inst.get("semantic", ""),
            "hex": to_hex32(machine_code), "bin": to_bin32(machine_code),
        }

    return {"error": f"无法解码 opcode={format(opcode,'07b')}", "hex": to_hex32(machine_code)}

# ── 验证向量 ──

VERIFY_VECTORS = [
    ("ADD",  ["x1", "x2", "x3"],    "0x003100b3"),
    ("SUB",  ["x1", "x2", "x3"],    "0x403100b3"),
    ("AND",  ["x5", "x6", "x7"],    "0x007372b3"),
    ("OR",   ["x5", "x6", "x7"],    "0x007362b3"),
    ("XOR",  ["x5", "x6", "x7"],    "0x007342b3"),
    ("SLT",  ["x5", "x6", "x7"],    "0x007322b3"),
    ("SLTU", ["x5", "x6", "x7"],    "0x007332b3"),
    ("SLL",  ["x5", "x6", "x7"],    "0x007312b3"),
    ("SRL",  ["x5", "x6", "x7"],    "0x007352b3"),
    ("SRA",  ["x5", "x6", "x7"],    "0x407352b3"),
    ("ADDI", ["x5", "x6", 100],      "0x06430293"),
    ("LW",   ["x10", 0, "x5"],      "0x0002a503"),
    ("SW",   ["x10", 0, "x5"],      "0x00a2a023"),
    ("LUI",  ["x1", 0x12345],       "0x123450b7"),
    ("BEQ",  ["x1", "x2", 0],       "0x00208063"),
    ("JAL",  ["x1", 0],             "0x000000ef"),
]

def cli():
    args = sys.argv[1:]
    if not args:
        print("用法: python3 mcode.py <encode|decode|verify|sweep>")
        sys.exit(0)

    cmd = args[0]

    if cmd == "encode":
        sid = args[1]
        ops = []
        for a in args[2:]:
            try: ops.append(int(a))
            except ValueError: ops.append(a)
        print(to_hex32(encode(sid, ops)))

    elif cmd == "decode":
        result = decode(args[1])
        print(json.dumps(result, indent=2, ensure_ascii=False))

    elif cmd == "verify":
        passed = 0
        for sid, ops, expected in VERIFY_VECTORS:
            actual = to_hex32(encode(sid, ops))
            ok = actual == expected
            print(f"{'✅' if ok else '❌'} {sid:6} {str(ops):25} → {actual} (期望 {expected})")
            if ok: passed += 1
        print(f"\n通过: {passed}/{len(VERIFY_VECTORS)}")

    elif cmd == "sweep":
        for inst in INST_LIST:
            test_ops = []
            for op in inst["operands"]:
                if op.startswith("imm"): test_ops.append(42)
                else: test_ops.append("x5")
            try:
                val = encode(inst["id"], test_ops)
                print(f"{inst['id']:6} {to_hex32(val)} {inst['mnemonic']}")
            except Exception as e:
                print(f"{inst['id']:6} 错误: {e}")

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)

if __name__ == "__main__":
    cli()
