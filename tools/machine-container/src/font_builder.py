#!/usr/bin/env python3
# ============================================================
# 字体容器构建器 · Font Container Builder
#
# 生成包含 MCOD 表的 TTF 字体文件
#
# 用法: python3 font_builder.py [output.ttf]
# ============================================================

import sys
import json
from pathlib import Path

try:
    from fontTools.ttLib import TTFont, newTable
    from fontTools.fontBuilder import FontBuilder
    from fontTools.pens.ttGlyphPen import TTGlyphPen
except ImportError:
    print("需要安装 fontTools: pip install fonttools")
    sys.exit(1)

from mcod_spec import SemanticId, RV32I_TEMPLATES, Platform, Polarity, Ownership, InsnFormat

PRIVATE_START = 0xE000


def build_mcod_entries():
    entries = []
    for sid, (fmt, opcode, funct3, funct7) in RV32I_TEMPLATES.items():
        entries.append({
            'semantic_id': sid.value,
            'semantic_name': sid.name,
            'platform': Platform.RISCV,
            'format': fmt.value,
            'opcode': opcode,
            'funct3': funct3 if funct3 is not None else 0,
            'funct7': funct7 if funct7 is not None else 0,
            'codepoint': PRIVATE_START + sid.value,
            'operand_cnt': 3 if fmt == InsnFormat.R else 2 if fmt == InsnFormat.I else 0,
        })
    return entries


def _box_glyph(w=400, h=600):
    """简单方框 glyph pen"""
    pen = TTGlyphPen(None)
    pen.moveTo((50, 0))
    pen.lineTo((50, h))
    pen.lineTo((50 + w, h))
    pen.lineTo((50 + w, 0))
    pen.closePath()
    return pen.glyph()


def create_machine_container_font(output_path="/tmp/machine_container.ttf"):
    entries = build_mcod_entries()
    glyph_names = [".notdef"] + [f"sem_{e['semantic_name']}" for e in entries]
    char_map = {e['codepoint']: f"sem_{e['semantic_name']}" for e in entries}

    fb = FontBuilder(1024, isTTF=True)
    fb.setupGlyphOrder(glyph_names)
    fb.setupCharacterMap(char_map)
    fb.setupGlyf({".notdef": _box_glyph(300, 500), **{f"sem_{e['semantic_name']}": _box_glyph() for e in entries}})
    fb.setupHorizontalMetrics({".notdef": (400, 0), **{f"sem_{e['semantic_name']}": (500, 0) for e in entries}})
    fb.setupHorizontalHeader()
    fb.setupNameTable({"familyName": "MachineContainer", "styleName": "Regular"})
    fb.setupOS2()
    fb.setupPost()
    font = fb.font

    # MCOD 自定义表
    mcod = newTable("MCOD")
    mcod.data = json.dumps({
        'version': 1, 'platform': 'RISC-V RV32I',
        'entry_count': len(entries), 'entries': entries,
    }, ensure_ascii=False).encode('utf-8')
    font["MCOD"] = mcod
    font.save(output_path)
    print(f"字体: {output_path}  MCOD: {len(entries)} 条映射")
    return font


def load_mcod(font_path):
    return json.loads(TTFont(font_path)["MCOD"].data.decode('utf-8')).get('entries', [])


if __name__ == '__main__':
    output = sys.argv[1] if len(sys.argv) > 1 else "/tmp/machine_container.ttf"
    create_machine_container_font(Path(output).resolve().as_posix())
    print("验证:", [(e['semantic_name'], hex(e['codepoint'])) for e in load_mcod(output)[:3]])
