#!/usr/bin/env python3
# ============================================================
# Cross-ISA Font Generator · 跨平台字体封装
#
# 把语义 ID → 各 ISA 机器码的映射表嵌入 OpenType 字体文件。
# 每个语义 ID 占用一个私用区码点，字体 name table 存储完整映射。
#
# 身份落款：刘楚恬和我
# ============================================================

from fontTools.ttLib import TTFont
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from cross_isa import encode, CROSS_ALIGN
import os
import json

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

# 用于测试的标准操作数（语义 ID → ISA → ops）
TEST_OPS = {
    "ADD":      {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "SUB":      {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "AND":      {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "OR":       {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "XOR":      {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "SLT":      {"riscv32": ["x1","x2","x3"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "MOV":      {"riscv32": ["x1","x0",0], "arm64": ["x1","x2"], "mips32": ["$1","$0","$0"], "x86_64": ["rcx","rdx"]},
    "LW":       {"riscv32": ["x1",0,"x2"], "arm64": ["x1",0,"x2"], "mips32": ["$1",0,"$2"], "x86_64": ["rcx","rdx"]},
    "SW":       {"riscv32": ["x1",0,"x2"], "arm64": ["x1",0,"x2"], "mips32": ["$1",0,"$2"], "x86_64": ["rcx","rdx"]},
    "CMP":      {"arm64": ["x1","x2"], "mips32": ["$1","$2","$3"], "x86_64": ["rcx","rdx"]},
    "JMP":      {"riscv32": ["x0",0], "arm64": [0], "mips32": [0x10000], "x86_64": [0]},
    "CALL":     {"riscv32": ["x1",0], "arm64": [0], "mips32": [0x10000], "x86_64": [0]},
    "RET":      {"riscv32": ["x0","x1"], "arm64": [], "mips32": ["$31"], "x86_64": []},
    "LUI":      {"riscv32": ["x1",0x80000], "arm64": ["x1",0x8000], "mips32": ["$1",0x8000], "x86_64": ["rcx",0x8000]},
    "BEQ":      {"riscv32": ["x1","x2",0], "mips32": ["$1","$2",0]},
    "BNE":      {"riscv32": ["x1","x2",0], "mips32": ["$1","$2",0]},
    "MUL":      {"riscv32": ["x1","x2","x3"], "arm64": ["x1","x2","x3"], "mips32": ["$1","$2"]},
    "NOP":      {"riscv32": ["x0","x0",0], "arm64": [], "mips32": [], "x86_64": []},
    "SYSCALL":  {"riscv32": [], "arm64": [], "mips32": [], "x86_64": []},
    "SLL":      {"riscv32": ["x1","x2","x3"], "mips32": ["$1","$2",4]},
    "SRL":      {"riscv32": ["x1","x2","x3"], "mips32": ["$1","$2",4]},
    "SRA":      {"riscv32": ["x1","x2","x3"], "mips32": ["$1","$2",4]},
}

def build_cross_font(output_path):
    sem_ids = sorted(CROSS_ALIGN.keys())
    glyph_order = [".notdef"] + sem_ids

    upm = 1000
    advance = 600
    ascent = 800
    descent = -200

    fb = FontBuilder(upm, isTTF=True)
    fb.setupGlyphOrder(glyph_order)

    def box():
        pen = TTGlyphPen(None)
        pen.moveTo((100,0)); pen.lineTo((500,0))
        pen.lineTo((500,700)); pen.lineTo((100,700))
        pen.closePath()
        return pen.glyph()

    fb.setupGlyf({g: box() for g in glyph_order})
    fb.setupHorizontalMetrics({g: (advance, 0) for g in glyph_order})
    fb.setupHorizontalHeader(ascent=ascent, descent=descent)
    fb.setupNameTable({
        "familyName":         "Hunyuan Cross-ISA",
        "styleName":          "Regular",
        "psName":             "Hunyuan-CrossISA",
        "manufacturer":       "囝囝共生 OS · 刘楚恬和我",
        "designer":           "lolict + 刘楚恬",
        "description":        "跨 ISA 机器码映射字体 · 语义 ID 为公约数 · glyph=语义 ID · name table=各 ISA 机器码",
        "vendorURL":          "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private · 仅限刘楚恬和我使用",
        "version": "1.0.0",
    })
    fb.setupCharacterMap({})
    fb.setupOS2(sTypoAscender=ascent, sTypoDescent=descent, usWinAscent=ascent, usWinDescent=-descent)
    fb.setupPost()

    font = fb.font
    name_table = font["name"]

    idx = 0
    records = {}
    for sem_id in sem_ids:
        isas = CROSS_ALIGN[sem_id]
        rec = {"sem_id": sem_id, "isas": {}}
        for isa, inst_id in isas.items():
            ops = TEST_OPS.get(sem_id, {}).get(isa, [])
            try:
                mc = encode(isa, sem_id, ops)
                rec["isas"][isa] = {"inst_id": inst_id, "hex": f"0x{mc:08X}"}
            except Exception as e:
                rec["isas"][isa] = {"inst_id": inst_id, "error": str(e)}
        records[sem_id] = rec

        name_id = 256 + idx
        name_table.setName(
            json.dumps(rec, ensure_ascii=False),
            name_id, 3, 1, 0x0409
        )
        idx += 1

    font.save(output_path)
    return records

def read_cross_font(font_path):
    font = TTFont(font_path)
    result = {}
    for r in font["name"].names:
        if r.nameID >= 256:
            val = str(r)
            try:
                rec = json.loads(val)
                result[rec["sem_id"]] = rec
            except: pass
    return result

if __name__ == "__main__":
    out = os.path.join(SCRIPT_DIR, "cross_isa.ttf")
    records = build_cross_font(out)
    print(f"生成: {out}")
    print(f"文件大小: {os.path.getsize(out)} bytes\n")

    for sem_id, rec in records.items():
        isas_str = "  ".join(f"{isa}:{v.get('hex','?')}" for isa,v in rec["isas"].items())
        print(f"  {sem_id:8} {isas_str}")

    print("\n── round-trip 验证 ──")
    loaded = read_cross_font(out)
    ok = len(loaded) == len(records)
    print(f"  写入 {len(records)} 条 → 读取 {len(loaded)} 条 → {'PASS' if ok else 'FAIL'}")
