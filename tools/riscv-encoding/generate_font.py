#!/usr/bin/env python3
# ============================================================
# RISC-V 指令字体生成器 · 编码表嵌入字体私有区
#
# 把 BASIC16 指令名:机器码对嵌入 TrueType name table 的 nameID 256+
# ============================================================

from fontTools.ttLib import TTFont
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen

BASIC16_OPCODES = {
    "ADD":   "0x007302B3",
    "SUB":   "0x407302B3",
    "AND":   "0x007372B3",
    "OR":    "0x00B564B3",
    "XOR":   "0x00E6C633",
    "SLT":   "0x007322B3",
    "SLTU":  "0x007332B3",
    "SLL":   "0x007312B3",
    "SRL":   "0x01BC55B3",
    "SRA":   "0x41BC55B3",
    "ADDI":  "0x06430293",
    "LW":    "0x00032283",
    "SW":    "0x00532423",
    "BEQ":   "0x00628863",
    "JAL":   "0x4000046F",
    "LUI":   "0x123452B7",
}

def _make_box_pen():
    pen = TTGlyphPen(None)
    pen.moveTo((100, 0))
    pen.lineTo((500, 0))
    pen.lineTo((500, 700))
    pen.lineTo((100, 700))
    pen.closePath()
    return pen.glyph()

def build_riscv_font(output_path: str) -> str:
    upm = 1000
    glyph_order = [".notdef"] + sorted(BASIC16_OPCODES.keys())
    advance = 600
    ascent  = 800
    descent = -200

    fb = FontBuilder(upm, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    box = _make_box_pen()
    fb.setupGlyf({g: box for g in glyph_order})
    fb.setupHorizontalMetrics({g: (advance, 0) for g in glyph_order})
    fb.setupHorizontalHeader(ascent=ascent, descent=descent)
    fb.setupNameTable({
        "familyName":         "RISC-V Instructions",
        "styleName":          "Regular",
        "psName":             "RV32I-Instructions",
        "manufacturer":       "囝囝共生 OS · 刘楚恬和我",
        "designer":           "lolict + 刘楚恬",
        "description":         "RV32I BASIC16 指令字体 · glyph=语义ID · name table 嵌入机器码表",
        "vendorURL":          "https://github.com/lolict/hunyuan-os",
        "designerURL":        "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private · 仅限刘楚恬和我使用",
        "licenseInfoURL":     "-",
        "version":            "0.1.0",
    })
    fb.setupCharacterMap({})
    fb.setupOS2(sTypoAscender=ascent, sTypoDescent=descent, usWinAscent=ascent, usWinDescent=-descent)
    fb.setupPost()

    font = fb.font
    name_table = font["name"]
    for idx, (op_name, opcode_hex) in enumerate(BASIC16_OPCODES.items()):
        name_id = 256 + idx
        name_table.setName(f"{op_name}:{opcode_hex}", name_id, 1, 0, 0)
        name_table.setName(f"{op_name}:{opcode_hex}", name_id, 3, 1, 0x0409)
    font.save(output_path)
    return output_path

def read_font_table(font_path: str) -> dict:
    font = TTFont(font_path)
    result = {}
    for r in font["name"].names:
        if r.nameID >= 256:
            val = str(r)
            if ":" in val:
                op, code = val.split(":", 1)
                result[op] = code
    return result

if __name__ == "__main__":
    out = "rv32i_instructions.ttf"
    build_riscv_font(out)
    print(f"Generated: {out}")
    print(f"File size: {__import__('os').path.getsize(out)} bytes\n")
    table = read_font_table(out)
    print(f"name table round-trip ({len(table)} ops):")
    for op, code in sorted(table.items()):
        print(f"  {op:6s} -> {code}")
    ok = all(BASIC16_OPCODES.get(op) == code for op, code in table.items())
    print(f"\nround-trip: {'PASS' if ok else 'FAIL'}")
