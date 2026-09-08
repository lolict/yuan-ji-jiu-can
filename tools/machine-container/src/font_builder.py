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
    from fontTools.pens.t2Pen import T2Pen
except ImportError:
    print("需要安装 fontTools: pip install fonttools")
    sys.exit(1)

from mcod_spec import (
    SemanticId, RV32I_TEMPLATES, Platform, Polarity, Ownership, InsnFormat,
)

# ── Unicode 私有区码点分配 ──
PRIVATE_START = 0xE000


def build_mcod_entries():
    """构建 MCOD 表数据"""
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


def _make_box_glyph():
    """创建一个简单的方框 glyph"""
    pen = T2Pen(None)
    pen.moveTo((100, 0))
    pen.lineTo((100, 700))
    pen.lineTo((500, 700))
    pen.lineTo((500, 0))
    pen.closePath()
    return pen.glyph()


def create_machine_container_font(output_path: str = "/tmp/machine_container.ttf"):
    """创建机器码容器字体"""
    entries = build_mcod_entries()
    glyph_names = [".notdef"] + [f"sem_{e['semantic_name']}" for e in entries]
    char_map = {e['codepoint']: f"sem_{e['semantic_name']}" for e in entries}

    fb = FontBuilder(1024, isTTF=True)
    fb.setupGlyphOrder(glyph_names)
    fb.setupCharacterMap(char_map)

    # ── Glyphs ──
    notdef_glyph = _make_box_glyph()
    sem_glyphs = {f"sem_{e['semantic_name']}": _make_box_glyph() for e in entries}
    all_glyphs = {".notdef": notdef_glyph}
    all_glyphs.update(sem_glyphs)
    fb.setupGlyf(all_glyphs)

    # ── 水平度量 ──
    metrics = {".notdef": (600, 0)}
    metrics.update({f"sem_{e['semantic_name']}": (600, 0) for e in entries})
    fb.setupHorizontalMetrics(metrics)

    fb.setupHorizontalHeader()
    fb.setupNameTable({"familyName": "MachineContainer", "styleName": "Regular"})
    fb.setupOs2()
    fb.setupPost()
    font = fb.font

    # ── MCOD 自定义表 ──
    mcod_json = json.dumps({
        'version': 1,
        'platform': 'RISC-V RV32I',
        'entry_count': len(entries),
        'entries': entries,
    }, ensure_ascii=False)

    mcod = newTable("MCOD")
    mcod.data = mcod_json.encode('utf-8')
    font["MCOD"] = mcod

    font.save(output_path)
    print(f"机器码容器字体: {output_path}")
    print(f"  字形数: {len(entries)}")
    print(f"  MCOD 映射: {len(entries)} 条")
    print(f"  码点范围: U+{PRIVATE_START:04X} - U+{PRIVATE_START + len(entries) - 1:04X}")
    return font


def load_mcod_from_font(font_path: str) -> list:
    """从字体文件读取 MCOD 表"""
    font = TTFont(font_path)
    if "MCOD" not in font:
        return []
    return json.loads(font["MCOD"].data.decode('utf-8')).get('entries', [])


if __name__ == '__main__':
    output = sys.argv[1] if len(sys.argv) > 1 else "/tmp/machine_container.ttf"
    create_machine_container_font(Path(output).resolve().as_posix())
    print("\n=== 验证 ===")
    for e in load_mcod_from_font(output)[:5]:
        print(f"  {e['semantic_name']:8s} opcode={e['opcode']:07b} -> U+{e['codepoint']:04X}")
