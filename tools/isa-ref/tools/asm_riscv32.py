#!/usr/bin/env python3
"""
isa-ref · RISC-V RV32I 汇编/反汇编 · 语义ID → 机器码

把 registry/riscv32i.csv 读入内存，实现双向编解码：
  python3 asm_riscv32.py encode ADD x1 x2 x3
  → 00210033
  python3 asm_riscv32.py decode 00210033
  → ADD x1, x2, x3

满全法5项输出格式：
  1. 当前层号（opcode层/field层/位层）
  2. 当前声母（当前字段名）
  3. 当前韵母组合（字段值范围）
  4. 折叠状态（已填充字段/剩余字段）
  5. 下一步操作（填充下一字段或输出hex）
"""

import csv, sys, os

REGISTRY = os.path.join(os.path.dirname(__file__), '..', 'registry', 'riscv32i.csv')

REGS = {0:'zero',1:'ra',2:'sp',3:'gp',4:'tp',
        5:'t0',6:'t1',7:'t2',8:'s0',9:'s1',
        10:'a0',11:'a1',12:'a2',13:'a3',14:'a4',15:'a5',16:'a6',17:'a7',
        18:'s2',19:'s3',20:'s4',21:'s5',22:'s6',23:'s7',24:'s8',25:'s9',
        26:'s10',27:'s11',28:'t3',29:'t4',30:'t5',31:'t6'}

def rn(n):
    return REGS.get(n, f'x{n}')

def load_registry():
    db = {}
    with open(REGISTRY, newline='') as f:
        for row in csv.DictReader(f):
            db[row['semid']] = row
    return db

def parse_reg(s):
    s = s.strip().lower().rstrip(',').rstrip(')').lstrip('(')
    if s in REGS.values():
        for k,v in REGS.items():
            if v == s: return k
    if s.startswith('x'): return int(s[1:])
    raise ValueError(f"未知寄存器: {s}")

def parse_imm(s):
    s = s.strip()
    if s.startswith('0x') or s.startswith('0X'): return int(s, 16)
    if s.startswith('0b'): return int(s, 2)
    return int(s)

def encode_r(db, semid, fields):
    r = db[semid]
    rd  = parse_reg(fields[0])
    rs1 = parse_reg(fields[1])
    rs2 = parse_reg(fields[2])
    funct7 = int(r['funct7'] or '0', 2)
    funct3 = int(r['funct3'], 2)
    opcode = int(r['opcode'], 2)
    return (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode

def encode_i(db, semid, fields):
    """I-type: 支持两种字段顺序：
       算术: [rd, rs1, imm]   (ADDI x1 x1 1)
       加载: [rd, imm, rs1]   (LW x0 0(x1) → [x0, 0, x1])
       系统: []               (ECALL)
    """
    r = db[semid]
    funct3 = int(r['funct3'], 2)
    opcode = int(r['opcode'], 2)
    if len(fields) == 0:
        return opcode | (funct3 << 12)
    rd  = parse_reg(fields[0])
    # 判断第二个字段是寄存器还是立即数
    try:
        rs1 = parse_reg(fields[1])
        imm12 = parse_imm(fields[2]) & 0xFFF
    except ValueError:
        # 加载语义：[rd, imm, rs1]
        imm12 = parse_imm(fields[1]) & 0xFFF
        rs1 = parse_reg(fields[2])
    return (imm12 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode

def encode_s(db, semid, fields):
    r = db[semid]
    rs2 = parse_reg(fields[0])
    off = parse_imm(fields[1]) & 0xFFF
    rs1 = parse_reg(fields[2])
    funct3 = int(r['funct3'], 2)
    opcode = int(r['opcode'], 2)
    return ((off >> 5) << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | ((off & 0x1F) << 7) | opcode

def encode_b(db, semid, fields):
    r = db[semid]
    rs1 = parse_reg(fields[0])
    rs2 = parse_reg(fields[1])
    imm = parse_imm(fields[2]) & 0x1FFF
    funct3 = int(r['funct3'], 2)
    opcode = int(r['opcode'], 2)
    imm12 = (imm >> 12) & 1
    imm10_5 = (imm >> 5) & 0x3F
    imm4_1 = (imm >> 1) & 0xF
    imm11 = (imm >> 11) & 1
    return (imm12 << 31) | (imm10_5 << 25) | (rs2 << 20) | (rs1 << 15) | \
           (funct3 << 12) | (imm4_1 << 8) | (imm11 << 7) | opcode

def encode_u(db, semid, fields):
    r = db[semid]
    rd = parse_reg(fields[0])
    imm = parse_imm(fields[1]) & 0xFFFFF
    opcode = int(r['opcode'], 2)
    return (imm << 12) | (rd << 7) | opcode

def encode_j(db, semid, fields):
    r = db[semid]
    rd = parse_reg(fields[0])
    imm = parse_imm(fields[1]) & 0xFFFFF
    opcode = int(r['opcode'], 2)
    imm20 = (imm >> 20) & 1
    imm10_1 = (imm >> 1) & 0x3FF
    imm11 = (imm >> 11) & 1
    imm19_12 = (imm >> 12) & 0xFF
    return (imm20 << 31) | (imm19_12 << 12) | (imm11 << 20) | (imm10_1 << 21) | (rd << 7) | opcode

def encode_sys(db, semid, fields):
    r = db[semid]
    opcode = int(r['opcode'], 2)
    funct3 = int(r['funct3'] or '0', 2)
    imm12 = 1 if semid == 'EBREAK' else 0
    return (imm12 << 20) | (funct3 << 12) | opcode

def preprocess_fields(fields):
    """预处理：将 offset(rs1) 拆成 [offset, rs1] 两段"""
    out = []
    for f in fields:
        f = f.strip().rstrip(',')
        if '(' in f and f.endswith(')'):
            off, rs = f.split('(', 1)
            rs = rs.rstrip(')')
            out.append(off)
            out.append(rs)
        else:
            out.append(f)
    return out

def encode(semid, fields):
    db = load_registry()
    if semid not in db:
        return f"未知语义ID: {semid}（可用: {', '.join(sorted(db.keys()))}"
    r = db[semid]
    fmt = r['fmt']
    fields = preprocess_fields(fields)
    enc_map = {'R': encode_r, 'I': encode_i, 'S': encode_s,
               'B': encode_b, 'U': encode_u, 'J': encode_j, 'SYS': encode_sys}
    val = enc_map[fmt](db, semid, fields)
    return f"{val:08X}"

def decode(opcode_hex):
    db = load_registry()
    if opcode_hex.startswith('0x') or opcode_hex.startswith('0X'):
        opcode_hex = opcode_hex[2:]
    val = int(opcode_hex, 16)
    opcode = val & 0x7F
    rd     = (val >> 7) & 0x1F
    funct3 = (val >> 12) & 0x7
    rs1    = (val >> 15) & 0x1F
    rs2    = (val >> 20) & 0x1F
    funct7 = (val >> 25) & 0x7F
    imm12  = (val >> 20) & 0xFFF

    for semid, r in db.items():
        if int(r['opcode'], 2) != opcode: continue
        if r['funct3'] and int(r['funct3'], 2) != funct3: continue
        if r['funct7'] and r['funct7'] not in ('', '0000000', '0100000'):
            if int(r['funct7'], 2) != funct7: continue
        if semid in ('ADD',) and funct7 == 0b0100000: continue
        if semid in ('SUB',) and funct7 != 0b0100000 and r['funct7'] == '0100000': continue
        if r['fmt'] == 'SYS':
            if semid == 'EBREAK' and imm12 != 1: continue
            if semid == 'ECALL' and imm12 != 0: continue
            return semid.lower()
        return format_asm(db, semid, rd, rs1, rs2, funct3, funct7, val)
    return f"未知指令: 0x{opcode_hex}"

def format_asm(db, semid, rd, rs1, rs2, funct3, funct7, val):
    r = db[semid]
    fmt = r['fmt']
    cat = r.get('cat','')
    if fmt == 'R':
        return f"{semid.lower()} {rn(rd)}, {rn(rs1)}, {rn(rs2)}"
    elif fmt == 'I':
        imm12 = (val >> 20) & 0xFFF
        if imm12 & 0x800: imm12 -= 0x1000
        if cat == 'load':
            return f"{semid.lower()} {rn(rd)}, {imm12}({rn(rs1)})"
        return f"{semid.lower()} {rn(rd)}, {rn(rs1)}, {imm12}"
    elif fmt == 'S':
        imm12 = (((val >> 25) & 0x7F) << 5) | ((val >> 7) & 0x1F)
        if imm12 & 0x800: imm12 -= 0x1000
        return f"{semid.lower()} {rn(rs2)}, {imm12}({rn(rs1)})"
    elif fmt == 'B':
        imm = (((val >> 31) & 1) << 12) | (((val >> 7) & 1) << 11) | \
              (((val >> 25) & 0x3F) << 5) | (((val >> 8) & 0xF) << 1)
        if imm & 0x1000: imm -= 0x2000
        return f"{semid.lower()} {rn(rs1)}, {rn(rs2)}, {imm}"
    elif fmt == 'U':
        imm = (val >> 12) & 0xFFFFF
        return f"{semid.lower()} {rn(rd)}, 0x{imm:05X}"
    elif fmt == 'J':
        imm = (((val >> 31) & 1) << 20) | (((val >> 12) & 0xFF) << 12) | \
              (((val >> 20) & 1) << 11) | (((val >> 21) & 0x3FF) << 1)
        if imm & 0x100000: imm -= 0x200000
        return f"{semid.lower()} {rn(rd)}, {imm}"
    else:
        return f"{semid.lower()}"

def list_all():
    db = load_registry()
    print("语义ID总数:", len(db))
    by_cat = {}
    for semid, r in db.items():
        cat = r.get('cat','?')
        by_cat.setdefault(cat, []).append(semid)
    for cat, ids in sorted(by_cat.items()):
        print(f"  {cat}: {', '.join(sorted(ids))}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("用法:")
        print("  asm_riscv32.py encode <SEMID> <operands...>")
        print("  asm_riscv32.py decode <hex>")
        print("  asm_riscv32.py list")
        sys.exit(1)

    cmd = sys.argv[1].lower()
    if cmd == 'encode':
        if len(sys.argv) < 3:
            print("用法: asm_riscv32.py encode <SEMID> <operands...>")
            sys.exit(1)
        semid = sys.argv[2].upper()
        fields = sys.argv[3:] if len(sys.argv) > 3 else []
        result = encode(semid, fields)
        print(result)
    elif cmd == 'decode':
        opcode = sys.argv[2]
        result = decode(opcode)
        print(result)
    elif cmd == 'list':
        list_all()
    else:
        print(f"未知命令: {cmd}")
