#!/usr/bin/env python3
"""
gen_rv32i_font.py — 从 RV32I 符号映射生成 TTF 字体

用法:
  python3 gen_rv32i_font.py <mapping.json> <output.ttf>
  python3 gen_rv32i_font.py                    # demo 模式
"""

import sys
import json


def make_simple_glyph(pen, x0, y0, x1, y1):
    """用矩形绘制简单字形占位符。"""
    pen.moveTo((x0, y0))
    pen.lineTo((x0, y1))
    pen.lineTo((x1, y1))
    pen.lineTo((x1, y0))
    pen.closePath()


def make_rv32i_font(mapping: dict, out_path: str):
    """生成 RV32I 机器码字体。"""
    from fontTools.fontBuilder import FontBuilder
    from fontTools.pens.ttGlyphPen import TTGlyphPen

    sorted_items = sorted(mapping.items(), key=lambda x: int(x[0], 16))
    n = len(sorted_items)

    glyph_order = [".notdef"] + [f"sym_{i:02X}" for i in range(n)]
    cmap = {}
    metrics = {".notdef": (500, 0)}
    glyphs = {}

    # .notdef 字形
    pen = TTGlyphPen(None)
    make_simple_glyph(pen, 50, 0, 450, 700)
    glyphs[".notdef"] = pen.glyph()

    for idx, (cp_str, meta) in enumerate(sorted_items):
        cp = int(cp_str, 16)
        glyph_name = f"sym_{idx:02X}"
        cmap[cp] = glyph_name
        metrics[glyph_name] = (700, 0)

        # 简单矩形占位符
        pen = TTGlyphPen(None)
        make_simple_glyph(pen, 100, 0, 600, 700)
        glyphs[glyph_name] = pen.glyph()

    # 构建字体
    fb = FontBuilder(1024, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    fb.setupGlyf(glyphs)
    fb.setupCharacterMap(cmap)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupNameTable({
        "familyName": "RV32I MachineCode",
        "styleName": "Regular",
    })
    fb.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWinAscent=1000, usWinDescent=200)
    fb.setupPost(formatType=2.0)

    fb.font.save(out_path)

    # 验证
    from fontTools.ttLib import TTFont
    font = TTFont(out_path)
    cmap_table = font.getBestCmap()
    print(f"  ✓ 字体已生成: {out_path}")
    print(f"  ✓ 包含字形数: {n}")
    print(f"  ✓ cmap 映射数: {len(cmap_table)}")
    print()
    print("  【符号码点 → 字形 → 指令 → 机器码】")
    for cp in sorted(cmap_table.keys()):
        gn = cmap_table[cp]
        # 查找元数据：先精确再忽略大小写
        meta = mapping.get(hex(cp), mapping.get(f"0x{cp:04X}", {}))
        if not meta:
            for k, v in mapping.items():
                if int(k, 16) == cp:
                    meta = v
                    break
        name = meta.get("name", "?")
        machine = meta.get("machine", "?")
        print(f"    U+{cp:04X} → {gn:8s} → {name:6s} → 0x{machine}")

    return {"glyph_count": n, "cmap_count": len(cmap_table)}


def main():
    if len(sys.argv) < 3:
        print("用法: python3 gen_rv32i_font.py <mapping.json> <output.ttf>")
        print()
        print("→ 生成示例字体 (demo 模式)...")
        demo_mapping = {
            "0xF000": {"name": "add",  "machine": "00000033", "format": "R", "funct3": "000", "funct7": "0000000"},
            "0xF001": {"name": "sub",  "machine": "40000033", "format": "R", "funct3": "000", "funct7": "0100000"},
            "0xF002": {"name": "xor",  "machine": "00004033", "format": "R", "funct3": "100", "funct7": "0000000"},
            "0xF003": {"name": "or",   "machine": "00006033", "format": "R", "funct3": "110", "funct7": "0000000"},
            "0xF004": {"name": "and",  "machine": "00007033", "format": "R", "funct3": "111", "funct7": "0000000"},
            "0xF005": {"name": "sll",  "machine": "00001033", "format": "R", "funct3": "001", "funct7": "0000000"},
            "0xF006": {"name": "srl",  "machine": "00005033", "format": "R", "funct3": "101", "funct7": "0000000"},
            "0xF007": {"name": "sra",  "machine": "40005033", "format": "R", "funct3": "101", "funct7": "0100000"},
            "0xF008": {"name": "addi", "machine": "00000013", "format": "I", "funct3": "000"},
            "0xF009": {"name": "slti", "machine": "00002013", "format": "I", "funct3": "010"},
            "0xF00A": {"name": "andi", "machine": "00007013", "format": "I", "funct3": "111"},
            "0xF00B": {"name": "lw",   "machine": "00002003", "format": "I", "funct3": "010"},
            "0xF00C": {"name": "sw",   "machine": "00002023", "format": "S", "funct3": "010"},
            "0xF00D": {"name": "beq",  "machine": "00000063", "format": "B", "funct3": "000"},
            "0xF00E": {"name": "bne",  "machine": "00001063", "format": "B", "funct3": "001"},
            "0xF00F": {"name": "jal",  "machine": "0000006F", "format": "J"},
            "0xF010": {"name": "jalr", "machine": "00000067", "format": "I", "funct3": "000"},
            "0xF011": {"name": "lui",  "machine": "00000037", "format": "U"},
            "0xF012": {"name": "auipc", "machine": "00000017", "format": "U"},
        }
        make_rv32i_font(demo_mapping, "rv32i_mc.ttf")
        return

    mapping_file = sys.argv[1]
    out_path = sys.argv[2]
    with open(mapping_file, "r", encoding="utf-8") as f:
        mapping = json.load(f)
    make_rv32i_font(mapping, out_path)


if __name__ == "__main__":
    main()
