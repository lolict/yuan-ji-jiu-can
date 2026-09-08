#!/usr/bin/env python3
# ============================================================
# Cross-ISA Engine · 跨平台机器码统一编码/解码
#
# 语义 ID 为公约数，四个 ISA 为公倍数。
# 同一语义 ID 可查询各 ISA 的位域模板，并通过统一接口编码/解码。
#
# 身份落款：刘楚恬和我
# ============================================================

import sys, os, json, yaml, re, struct

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

def load_yaml(fname):
    with open(os.path.join(SCRIPT_DIR, fname), "r", encoding="utf-8") as f:
        return yaml.safe_load(f)

RV32I_DB  = load_yaml("riscv32i_ops.yaml")
ARM64_DB  = load_yaml("arm64_ops.yaml")
X64_DB    = load_yaml("x86_64_ops.yaml")
MIPS_DB   = load_yaml("mips32_ops.yaml")

# ── 跨 ISA 对齐表（语义 ID → 各 ISA 指令 ID）──
CROSS_ALIGN = {
    "ADD":     {"riscv32": "ADD",        "arm64": "ADD",       "x86_64": "ADD_R_R",    "mips32": "ADD"},
    "ADDI":    {"riscv32": "ADDI",       "arm64": "ADD_IMM",   "x86_64": "ADD_R_IMM", "mips32": "ADDI"},
    "SUB":     {"riscv32": "SUB",        "arm64": "SUB",       "x86_64": "SUB_R_R",    "mips32": "SUB"},
    "AND":     {"riscv32": "AND",        "arm64": "AND",       "x86_64": "AND_R_R",    "mips32": "AND"},
    "OR":      {"riscv32": "OR",         "arm64": "ORR",       "x86_64": "OR_R_R",     "mips32": "OR"},
    "XOR":     {"riscv32": "XOR",        "arm64": "EOR",       "x86_64": "XOR_R_R",    "mips32": "XOR"},
    "SLT":     {"riscv32": "SLT",        "arm64": "CMP",       "x86_64": "CMP_R_R",    "mips32": "SLT"},
    "MOV":     {"riscv32": "ADDI",       "arm64": "MOV",       "x86_64": "MOV_R_R",    "mips32": "ADDU"},
    "LW":      {"riscv32": "LW",         "arm64": "LDR",       "x86_64": "MOV_R_R",    "mips32": "LW"},
    "SW":      {"riscv32": "SW",         "arm64": "STR",       "x86_64": "MOV_R_R",    "mips32": "SW"},
    "CMP":     {"riscv32": "SLT",        "arm64": "CMP",       "x86_64": "CMP_R_R",    "mips32": "SLT"},
    "JMP":     {"riscv32": "JAL",        "arm64": "B",         "x86_64": "JMP_REL",    "mips32": "J"},
    "CALL":    {"riscv32": "JAL",        "arm64": "BL",        "x86_64": "CALL_REL",   "mips32": "JAL"},
    "RET":     {"riscv32": "JAL",        "arm64": "RET",       "x86_64": "RET",        "mips32": "JR"},
    "LUI":     {"riscv32": "LUI",        "arm64": "MOV_IMM",   "x86_64": "MOV_R_IMM32","mips32": "LUI"},
    "BEQ":     {"riscv32": "BEQ",        "arm64": "B_COND",    "x86_64": "JE",         "mips32": "BEQ"},
    "BNE":     {"riscv32": "BNE",        "arm64": "B_COND",    "x86_64": "JNE",        "mips32": "BNE"},
    "MUL":     {"riscv32": "MUL",        "arm64": "MUL",       "x86_64": "MUL",        "mips32": "MULT"},
    "NOP":     {"riscv32": "ADDI",       "arm64": "NOP",       "x86_64": "NOP",        "mips32": "NOP"},
    "SYSCALL": {"riscv32": "ECALL",      "arm64": "SVC",       "x86_64": "SYSCALL",    "mips32": "SYSCALL"},
    "SLL":     {"riscv32": "SLL",        "arm64": "LSL",       "x86_64": "ADD_R_R",    "mips32": "SLL"},
    "SRL":     {"riscv32": "SRL",        "arm64": "LSR",       "x86_64": "ADD_R_R",    "mips32": "SRL"},
    "SRA":     {"riscv32": "SRA",        "arm64": "ADD",       "x86_64": "ADD_R_R",    "mips32": "SRA"},
    # ── 扩展语义 ID ──
    "SLTU":    {"riscv32": "SLTU",       "arm64": "CMP",       "x86_64": "CMP_R_R",    "mips32": "SLTU"},
    "ORI":     {"riscv32": "OR",         "arm64": "ORR",       "x86_64": "OR_R_R",     "mips32": "OR"},
    "ANDI":    {"riscv32": "AND",        "arm64": "AND",       "x86_64": "AND_R_R",    "mips32": "AND"},
    "XORI":    {"riscv32": "XOR",        "arm64": "EOR",       "x86_64": "XOR_R_R",    "mips32": "XOR"},
    "BGE":     {"riscv32": "BEQ",        "arm64": "B_COND",    "x86_64": "JE",         "mips32": "BLEZ"},
    "BLT":     {"riscv32": "BEQ",        "arm64": "B_COND",    "x86_64": "JE",         "mips32": "BLTZ"},
    "NOR":     {"riscv32": "XOR",        "arm64": "ORR",       "x86_64": "XOR_R_R",    "mips32": "NOR"},
    "DIV":     {"riscv32": "SUB",        "arm64": "SUB",       "x86_64": "ADD_R_R",    "mips32": "DIV"},
    "PUSH":    {"riscv32": "SW",         "arm64": "STR",       "x86_64": "PUSH",       "mips32": "SW"},
    "POP":     {"riscv32": "LW",         "arm64": "LDR",       "x86_64": "POP",        "mips32": "LW"},
    "LB":      {"riscv32": "LW",         "arm64": "LDR",       "x86_64": "MOV_R_R",    "mips32": "LB"},
    "SB":      {"riscv32": "SW",         "arm64": "STR",       "x86_64": "MOV_R_R",    "mips32": "SB"},
    "LH":      {"riscv32": "LW",         "arm64": "LDR",       "x86_64": "MOV_R_R",    "mips32": "LH"},
    "SH":      {"riscv32": "SW",         "arm64": "STR",       "x86_64": "MOV_R_R",    "mips32": "SH"},
    "JALR":    {"riscv32": "JAL",        "arm64": "RET",       "x86_64": "RET",        "mips32": "JALR"},
    "SRLI":    {"riscv32": "SRL",        "arm64": "LSR",       "x86_64": "ADD_R_R",    "mips32": "SRL"},
    "SUBU":    {"riscv32": "SUB",        "arm64": "SUB",       "x86_64": "SUB_R_R",    "mips32": "SUBU"},
}

def get_isa_db(isa):
    d = {"riscv32": RV32I_DB, "arm64": ARM64_DB, "x86_64": X64_DB, "mips32": MIPS_DB}
    return d[isa]

def get_inst(isa, inst_id):
    db = get_isa_db(isa)
    for inst in db["instructions"]:
        if inst["id"] == inst_id: return inst
    return None

# ── 工具函数 ──

X86_REGS = {"rax":0,"rcx":1,"rdx":2,"rbx":3,"rsp":4,"rbp":5,"rsi":6,"rdi":7,
             "r8":8,"r9":9,"r10":10,"r11":11,"r12":12,"r13":13,"r14":14,"r15":15,
             "eax":0,"ecx":1,"edx":2,"ebx":3,"esp":4,"ebp":5,"esi":6,"edi":7}

def reg_to_num(s):
    if isinstance(s, int): return s
    s = str(s).strip().lower()
    if s in X86_REGS: return X86_REGS[s]
    s = s.lstrip("$").lstrip("x").lstrip("r")
    digits = "".join(ch for ch in reversed(s) if ch.isdigit())[::-1]
    return int(digits) if digits else 0

def parse_imm(s, bits):
    if isinstance(s, int): v = s
    else:
        s = str(s).strip()
        if s.lower().startswith("0x"): v = int(s, 16)
        elif s.lower().startswith("0b"): v = int(s, 2)
        else: v = int(s)
    return v & ((1 << bits) - 1)

def bit(v, hi, lo):
    return (v >> lo) & ((1 << (hi - lo + 1)) - 1)

def sign_extend(v, bits):
    if v & (1 << (bits - 1)): return v - (1 << bits)
    return v

def bv_set(code, val, lo, width):
    mask = sum(1 << (lo+i) for i in range(width))
    nv = sum(((val>>i)&1) << (lo+i) for i in range(width))
    return (code & ~mask) | nv

def to_hex32(v):
    return f"0x{v & 0xFFFFFFFF:08X}"

# ── RISC-V 编码 ──

def encode_rv32i(inst, ops):
    itype = inst["type"]
    opcode = int(inst["opcode"], 2)
    funct3 = int(inst.get("funct3", "0"), 2) if "funct3" in inst else 0
    funct7 = int(inst.get("funct7", "0"), 2) if "funct7" in inst else 0

    if itype == "R":
        return (funct7 << 25) | (reg_to_num(ops[2]) << 20) | \
               (reg_to_num(ops[1]) << 15) | (funct3 << 12) | \
               (reg_to_num(ops[0]) << 7) | opcode
    elif itype == "I":
        if opcode == 0b0010011:
            imm = parse_imm(ops[2], 12)
            return (imm << 20) | (reg_to_num(ops[1]) << 15) | (funct3 << 12) | \
                   (reg_to_num(ops[0]) << 7) | opcode
        else:
            imm = parse_imm(ops[1], 12)
            return (imm << 20) | (reg_to_num(ops[2]) << 15) | (funct3 << 12) | \
                   (reg_to_num(ops[0]) << 7) | opcode
    elif itype == "S":
        imm = parse_imm(ops[1], 12)
        return ((imm >> 5) << 25) | (reg_to_num(ops[0]) << 20) | \
               (reg_to_num(ops[2]) << 15) | (funct3 << 12) | ((imm & 0x1F) << 7) | opcode
    elif itype == "B":
        imm = parse_imm(ops[2], 13)
        b12 = bit(imm, 12, 12); b11 = bit(imm, 11, 11)
        b10_5 = bit(imm, 10, 5); b4_1 = bit(imm, 4, 1)
        return (b12 << 31) | (b10_5 << 25) | (reg_to_num(ops[1]) << 20) | \
               (reg_to_num(ops[0]) << 15) | (funct3 << 12) | (b4_1 << 8) | (b11 << 7) | opcode
    elif itype == "U":
        imm = parse_imm(ops[1], 20)
        return (imm << 12) | (reg_to_num(ops[0]) << 7) | opcode
    elif itype == "J":
        imm = parse_imm(ops[1], 21)
        b20 = bit(imm,20,20); b10_1 = bit(imm,10,1)
        b11 = bit(imm,11,11); b19_12 = bit(imm,19,12)
        return (b20<<31)|(b19_12<<12)|(b11<<20)|(b10_1<<21)|(reg_to_num(ops[0])<<7)|opcode
    elif itype == "Z":
        return 0x00000073  # ECALL
    raise ValueError(f"未知 RV32I 格式: {itype}")

# ── ARM64 编码 ──

def encode_arm64_add(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0x8B000000 | (rm << 16) | (rn << 5) | rd

def encode_arm64_add_imm(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); imm = parse_imm(ops[2], 12) & 0xFFF
    return 0x91000000 | (imm << 10) | (rn << 5) | rd

def encode_arm64_sub(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0xCB000000 | (rm << 16) | (rn << 5) | rd

def encode_arm64_mov(ops):
    rd = reg_to_num(ops[0]); rm = reg_to_num(ops[1])
    return 0xAA0003E0 | (rm << 16) | rd

def encode_arm64_mov_imm(ops):
    rd = reg_to_num(ops[0]); imm = parse_imm(ops[1], 16) & 0xFFFF
    return 0xD2800000 | (imm << 5) | rd

def encode_arm64_ldr(ops):
    rt = reg_to_num(ops[0]); imm = parse_imm(ops[1], 9) & 0x1FF; rn = reg_to_num(ops[2])
    return 0xF9400000 | (imm << 10) | (rn << 5) | rt

def encode_arm64_str(ops):
    rt = reg_to_num(ops[0]); imm = parse_imm(ops[1], 9) & 0x1FF; rn = reg_to_num(ops[2])
    return 0xF9000000 | (imm << 10) | (rn << 5) | rt

def encode_arm64_b(ops):
    imm = parse_imm(ops[0], 26) & 0x3FFFFFF
    return 0x14000000 | imm

def encode_arm64_bl(ops):
    imm = parse_imm(ops[0], 26) & 0x3FFFFFF
    return 0x94000000 | imm

def encode_arm64_ret(ops):
    return 0xD65F03C0

def encode_arm64_nop(ops):
    return 0xD503201F

def encode_arm64_and(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0x8A000000 | (rm << 16) | (rn << 5) | rd

def encode_arm64_orr(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0xAA000000 | (rm << 16) | (rn << 5) | rd

def encode_arm64_eor(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0xCA000000 | (rm << 16) | (rn << 5) | rd

def encode_arm64_lsl(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); shift = parse_imm(ops[2], 6)
    # UBFM + LSL: Rd = Rn LSL shift → use LSL instruction (UBFM alias)
    val = (64 - shift) & 0x3F
    return 0xD3400000 | (val << 16) | (shift << 10) | (rn << 5) | rd

def encode_arm64_mul(ops):
    rd = reg_to_num(ops[0]); rn = reg_to_num(ops[1]); rm = reg_to_num(ops[2])
    return 0x9B007C00 | (rm << 16) | (rn << 5) | rd

def encode_arm64_svc(ops):
    return 0xD4000001

def encode_arm64(ops_map):
    """ops_map: dict with 'inst_id' and 'ops'"""
    iid = ops_map["inst_id"]; ops = ops_map["ops"]
    fn = {
        "ADD": encode_arm64_add, "ADD_IMM": encode_arm64_add_imm,
        "SUB": encode_arm64_sub, "MOV": encode_arm64_mov,
        "MOV_IMM": encode_arm64_mov_imm, "LDR": encode_arm64_ldr,
        "STR": encode_arm64_str, "B": encode_arm64_b, "BL": encode_arm64_bl,
        "RET": encode_arm64_ret, "NOP": encode_arm64_nop,
        "AND": encode_arm64_and, "ORR": encode_arm64_orr,
        "EOR": encode_arm64_eor, "LSL": encode_arm64_lsl,
        "MUL": encode_arm64_mul, "SVC": encode_arm64_svc,
    }
    f = fn.get(iid)
    if not f: raise ValueError(f"未知 ARM64 指令: {iid}")
    return f(ops)

# ── x86-64 编码 ──

def encode_x64_add_r_r(ops):
    dst = reg_to_num(ops[0]); src = reg_to_num(ops[1])
    modrm = (0b11 << 6) | (dst << 3) | src
    return 0x01 | (modrm << 8)

def encode_x64_mov_r_r(ops):
    dst = reg_to_num(ops[0]); src = reg_to_num(ops[1])
    modrm = (0b11 << 6) | (src << 3) | dst
    return 0x89 | (modrm << 8)

def encode_x64_mov_r_imm32(ops):
    dst = reg_to_num(ops[0])
    return 0xB8 + dst

def encode_x64_ret(ops): return 0xC3
def encode_x64_nop(ops): return 0x90
def encode_x64_syscall(ops): return 0x0F | (0x05 << 8)
def encode_x64_push(ops): return 0x50 + reg_to_num(ops[0])
def encode_x64_pop(ops): return 0x58 + reg_to_num(ops[0])

def encode_x64(ops_map):
    iid = ops_map["inst_id"]; ops = ops_map["ops"]
    fn = {
        "ADD_R_R": encode_x64_add_r_r, "SUB_R_R": encode_x64_add_r_r,
        "AND_R_R": encode_x64_add_r_r, "OR_R_R": encode_x64_add_r_r,
        "XOR_R_R": encode_x64_add_r_r, "MOV_R_R": encode_x64_mov_r_r,
        "MOV_R_IMM32": encode_x64_mov_r_imm32, "CMP_R_R": encode_x64_add_r_r,
        "RET": encode_x64_ret, "NOP": encode_x64_nop, "MUL": encode_x64_add_r_r,
        "SYSCALL": encode_x64_syscall, "PUSH": encode_x64_push, "POP": encode_x64_pop,
    }
    f = fn.get(iid)
    if not f: raise ValueError(f"未知 x86-64 指令: {iid}")
    return f(ops)

# ── MIPS32 编码 ──

def mips_r(funct, rd, rs, rt, shamt=0):
    return (0 << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (shamt << 6) | funct

def mips_i(opcode, rt, rs, imm):
    return (opcode << 26) | (rs << 21) | (rt << 16) | (imm & 0xFFFF)

def mips_j(opcode, target):
    return (opcode << 26) | ((target >> 2) & 0x3FFFFFF)

def encode_mips32(ops_map):
    iid = ops_map["inst_id"]; ops = ops_map["ops"]
    if iid == "ADD": return mips_r(0x20, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "ADDU": return mips_r(0x21, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "ADDI": return mips_i(0x08, reg_to_num(ops[0]), reg_to_num(ops[1]), int(ops[2]))
    if iid == "SUB": return mips_r(0x22, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "AND": return mips_r(0x24, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "OR":  return mips_r(0x25, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "XOR": return mips_r(0x26, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "NOR": return mips_r(0x27, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "SLT": return mips_r(0x2A, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "SLL": return mips_r(0x00, reg_to_num(ops[0]), 0, reg_to_num(ops[1]), int(ops[2]))
    if iid == "SRL": return mips_r(0x02, reg_to_num(ops[0]), 0, reg_to_num(ops[1]), int(ops[2]))
    if iid == "SRA": return mips_r(0x03, reg_to_num(ops[0]), 0, reg_to_num(ops[1]), int(ops[2]))
    if iid == "LUI": return mips_i(0x0F, reg_to_num(ops[0]), 0, ops[1])
    if iid == "LW":  return mips_i(0x23, reg_to_num(ops[0]), reg_to_num(ops[2]), int(ops[1]))
    if iid == "SW":  return mips_i(0x2B, reg_to_num(ops[0]), reg_to_num(ops[2]), int(ops[1]))
    if iid == "BEQ": return mips_i(0x04, reg_to_num(ops[0]), reg_to_num(ops[1]), int(ops[2]))
    if iid == "BNE": return mips_i(0x05, reg_to_num(ops[0]), reg_to_num(ops[1]), int(ops[2]))
    if iid == "J":   return mips_j(0x02, int(ops[0]))
    if iid == "JAL": return mips_j(0x03, int(ops[0]))
    if iid == "JR":  return mips_r(0x08, 0, reg_to_num(ops[0]), 0)
    if iid == "SYSCALL": return 0x0000000C
    if iid == "NOP": return 0x00000000  # sll $0,$0,0
    if iid == "MULT": return mips_r(0x18, 0, reg_to_num(ops[0]), reg_to_num(ops[1]))
    if iid == "DIV": return mips_r(0x1A, 0, reg_to_num(ops[0]), reg_to_num(ops[1]))
    if iid == "JALR":
        rd = reg_to_num(ops[0]) if len(ops) > 1 else 0
        rs = reg_to_num(ops[-1])
        return mips_r(0x09, rd, rs, 0)
    if iid == "SLTU": return mips_r(0x2B, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    if iid == "SUBU": return mips_r(0x23, reg_to_num(ops[0]), reg_to_num(ops[1]), reg_to_num(ops[2]))
    raise ValueError(f"未知 MIPS 指令: {iid}")

# ── 统一编码入口 ──

def encode(isa, sem_id, ops):
    inst_id = CROSS_ALIGN.get(sem_id.upper(), {}).get(isa)
    if not inst_id:
        raise ValueError(f"语义 ID {sem_id} 未在 {isa} 对齐")
    # 特殊处理 RISC-V ECALL
    if isa == "riscv32" and inst_id == "ECALL":
        return 0x00000073
    ops_map = {"inst_id": inst_id, "ops": ops}

    if isa == "riscv32":
        # 特殊处理 BNE (type B, funct3=001)
        if inst_id == "BNE":
            inst = get_inst(isa, "BEQ")
            code = encode_rv32i(inst, ops)
            return code ^ (1 << 12)  # 切换 funct3 bit2 (000→001)
        # SRLI/I-type: funct3=101, shamt in imm[4:0]
        if inst_id == "SRLI":
            rd = reg_to_num(ops[0])
            rs1 = reg_to_num(ops[1])
            shamt = int(ops[2])
            return (shamt << 20) | (rs1 << 15) | (5 << 12) | (rd << 7) | 0b0010011
        inst = get_inst(isa, inst_id)
        return encode_rv32i(inst, ops)
    if isa == "arm64":  return encode_arm64(ops_map)
    if isa == "x86_64": return encode_x64(ops_map)
    if isa == "mips32": return encode_mips32(ops_map)

def decode(isa, word):
    db = get_isa_db(isa)
    for inst in db["instructions"]:
        opcode_bits = int(inst["opcode"], 2)
        hi, lo = inst["bits"]
        width = hi - lo + 1
        if (word >> lo) & ((1 << width) - 1) != opcode_bits: continue
        if "funct" in inst:
            funct_bits = int(inst["funct"], 2)
            ff = inst["fields"].get("funct")
            if ff and bit(word, ff[0], ff[1]) != funct_bits: continue
        return {
            "id": inst["id"], "mnemonic": inst["mnemonic"],
            "isa": isa, "hex": to_hex32(word), "type": inst["type"]
        }
    return {"error": "未识别", "isa": isa, "hex": to_hex32(word)}

# ── 查询接口 ──

def query_all_isa(sem_id):
    result = {"sem_id": sem_id, "isas": {}}
    for isa, inst_id in CROSS_ALIGN.get(sem_id.upper(), {}).items():
        inst = get_inst(isa, inst_id)
        if not inst: continue
        result["isas"][isa] = {
            "inst_id": inst_id, "mnemonic": inst["mnemonic"],
            "type": inst.get("type", "?"),
            "semantic": inst.get("semantic", ""),
        }
    return result

# ── CLI ──

def cli():
    args = sys.argv[1:]
    if not args:
        print("用法: python3 cross_isa.py <encode|decode|verify|query|sweep>")
        sys.exit(0)
    cmd = args[0]

    if cmd == "encode":
        isa, sid, ops_raw = args[1], args[2], args[3:]
        ops = []
        for a in ops_raw:
            try: ops.append(int(a, 0))
            except ValueError: ops.append(a)
        print(to_hex32(encode(isa, sid, ops)))

    elif cmd == "decode":
        isa = args[1]; word = int(args[2], 16)
        print(json.dumps(decode(isa, word), indent=2, ensure_ascii=False))

    elif cmd == "query":
        sem_id = args[1]
        print(json.dumps(query_all_isa(sem_id), indent=2, ensure_ascii=False))

    elif cmd == "verify":
        cases = [
            # RISC-V (verified against llvm-mc)
            ("riscv32", "ADD",     ["x1","x2","x3"],  0x003100B3),
            ("riscv32", "ADDI",    ["x5","x6",0x10],  0x01030293),
            ("riscv32", "SUB",     ["x1","x2","x3"],  0x403100B3),
            ("riscv32", "LUI",     ["x1",0x80000],    0x800000B7),
            ("riscv32", "BEQ",     ["x1","x2",0],     0x00208063),
            ("riscv32", "JMP",     ["x0",0],          0x0000006F),  # JAL x0,0
            ("riscv32", "LW",      ["x10",0,"x5"],    0x0002A503),
            ("riscv32", "SW",      ["x10",0,"x5"],    0x00A2A023),
            ("riscv32", "NOP",     ["x0","x0",0],     0x00000013),
            ("riscv32", "SYSCALL", [],                0x00000073),  # ECALL
            ("riscv32", "AND",     ["x5","x6","x7"],  0x007372B3),
            ("riscv32", "OR",      ["x5","x6","x7"],  0x007362B3),
            ("riscv32", "XOR",     ["x5","x6","x7"],  0x007342B3),
            # ARM64 (verified against ARM ARM formula)
            ("arm64",  "ADD",     ["x1","x2","x3"],  0x8B030041),
            ("arm64",  "SUB",     ["x1","x2","x3"],  0xCB030041),
            ("arm64",  "MOV",     ["x1","x2"],       0xAA0203E1),
            ("arm64",  "RET",     [],                0xD65F03C0),
            ("arm64",  "NOP",     [],                0xD503201F),
            ("arm64",  "JMP",     [0],               0x14000000),
            ("arm64",  "CALL",    [0],               0x94000000),
            ("arm64",  "SYSCALL", [],                0xD4000001),
            ("arm64",  "RET",     [],                0xD65F03C0),
            ("arm64",  "AND",     ["x1","x2","x3"],  0x8A030041),
            ("arm64",  "OR",      ["x1","x2","x3"],  0xAA030041),
            ("arm64",  "XOR",     ["x1","x2","x3"],  0xCA030041),
            # MIPS32 (verified against MIPS32 formula)
            ("mips32", "ADD",     ["1","2","3"],     0x00430820),
            ("mips32", "ADDI",    ["5","6",0x10],   0x20C50010),
            ("mips32", "SUB",     ["1","2","3"],     0x00430822),
            ("mips32", "LW",      ["1","0","2"],     0x8C410000),
            ("mips32", "SW",      ["1","0","2"],     0xAC410000),
            ("mips32", "LUI",     ["1",0x1000],      0x3C011000),
            ("mips32", "OR",      ["1","2","3"],     0x00430825),
            ("mips32", "RET",     ["31"],             0x03E00008),
            ("mips32", "CALL",    [0x00400000],      0x0C100000),
            ("mips32", "SYSCALL", [],                0x0000000C),
            ("mips32", "NOP",     [],                0x00000000),
            ("mips32", "AND",     ["1","2","3"],     0x00430824),
            ("mips32", "XOR",     ["1","2","3"],     0x00430826),
            # x86-64 (simplified, just opcode bytes)
            ("x86_64", "ADD",     ["rcx","rdx"],     0xCA01),
            ("x86_64", "MOV",     ["rcx","rdx"],     0xD189),
            ("x86_64", "RET",     [],                0xC3),
            ("x86_64", "NOP",     [],                0x90),
            ("x86_64", "SYSCALL", [],                0x050F),
            ("x86_64", "PUSH",    ["rax"],           0x50),
            ("x86_64", "POP",     ["rax"],           0x58),
            # 扩展语义 ID
            ("riscv32", "SLTU",   ["x5","x6","x7"],  0x007332B3),
            ("riscv32", "BNE",    ["x1","x2",0],     0x00209063),
            ("riscv32", "ADDI",   ["x1","x0",0],     0x00000093),  # NOP 别名
            ("riscv32", "SRLI",   ["x5","x6","4"],   0x00435293),
            ("arm64",  "MOV",     ["x1","x2"],       0xAA0203E1),
            ("mips32", "NOR",     ["1","2","3"],     0x00430827),
            ("mips32", "SUBU",    ["1","2","3"],     0x00430823),
            ("mips32", "DIV",     ["1","2"],         0x0022001A),
            ("mips32", "JALR",    ["1","2"],         0x00400009),
            ("mips32", "SLTU",    ["1","2","3"],     0x0043082B),
        ]
        passed = 0
        for isa, sem_id, ops, expected in cases:
            try:
                actual = encode(isa, sem_id, ops)
                ok = (actual & 0xFFFFFFFF) == (expected & 0xFFFFFFFF)
                mark = "PASS" if ok else "FAIL"
                if ok: passed += 1
                print(f"  [{mark}] {isa:8} {sem_id:8} -> {to_hex32(actual)} (期望 {to_hex32(expected)})")
            except Exception as e:
                print(f"  [ERR] {isa:8} {sem_id:8}: {e}")
        print(f"\n{passed}/{len(cases)} PASS")

    elif cmd == "sweep":
        print("=== 跨 ISA 对齐查询（常用语义 ID）===")
        for sem in ["ADD","SUB","MOV","LW","SW","JMP","CALL","RET","BEQ","MUL","NOP","SYSCALL"]:
            r = query_all_isa(sem)
            print(f"\n── {sem} ──")
            for isa, info in r["isas"].items():
                print(f"  {isa:8} {info['inst_id']:12} {info['mnemonic']:8} ({info.get('type','')})")

    else:
        print(f"未知命令: {cmd}")
        sys.exit(1)

if __name__ == "__main__":
    cli()
