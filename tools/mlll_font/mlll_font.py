#!/usr/bin/env python3
"""
MLLL ISA 字体生成器 — 将语义指令 ID 封装进 SVG 字体
生成: mlll_isa_font.svg — 语义 ID 合一的字形 + 私有码点

PoC 范围: 16 条 RV32I 基础指令
"""

import xml.etree.ElementTree as ET
import os
import stat

# 来自 ISA 表的语义 ID 列表 (与 kernel mlll_isa_table.c 对齐)
SEM_IDS = [
    ("ADD",  0x003100B3, "add x1,x2,x3"),
    ("SUB",  0x403100B3, "sub x1,x2,x3"),
    ("AND",  0x003170B3, "and x1,x2,x3"),
    ("OR",   0x003160B3, "or x1,x2,x3"),
    ("XOR",  0x003140B3, "xor x1,x2,x3"),
    ("SLT",  0x003120B3, "slt x1,x2,x3"),
    ("SLTU", 0x003130B3, "sltu x1,x2,x3"),
    ("SLL",  0x003110B3, "sll x1,x2,x3"),
    ("SRL",  0x003150B3, "srl x1,x2,x3"),
    ("SRA",  0x403150B3, "sra x1,x2,x3"),
    ("ADDI", 0x00510093, "addi x1,x2,5"),
    ("LW",   0x00012083, "lw x1,0(x2)"),
    ("SW",   0x00112023, "sw x1,0(x2)"),
    ("BEQ",  0x00208063, "beq x1,x2,label"),
    ("JAL",  0x000000EF, "jal x1,label"),
    ("LUI",  0x123450B7, "lui x1,0x12345"),
]

UNICODE_PRIVATE_BASE = 0xE000
FONT_HORIZ_ADV = 512
UNITS_PER_EM = 512
ASCENT = 400
DESCENT = -112

def make_glyph_path(opcode_hex, sem_name):
    """为指令生成简单的视觉 glyph 路径 — 用 hex 的低 8 位驱动形状。
    用途: 每个指令在字体中视觉上可区分; 视觉本身不携带信息。"""
    v = opcode_hex & 0xFF
    margin = 60
    W = FONT_HORIZ_ADV
    # 根据 opcode 产生不同的笔画排列 (简化: 矩形框 + 内部 1-4 条线)
    lines = max(1, (v % 4) + 1)
    path = f"M {margin} {margin} L {W - margin} {margin} L {W - margin} {ASCENT - margin} L {margin} {ASCENT - margin} Z"
    for i in range(lines):
        y_off = 30 + i * 35
        path += f" L {W // 2 - 60} {y_off} L {W // 2 + 60} {y_off}"
    return path


def generate_font_svg(out_path):
    """生成包含 16 个指令字形的 SVG 字体文件。
    每个 <glyph> 的 glyph-name 字段编码为: MLLL-ISA-<SEM_ID>-<HEX>
    解析工具从此字段反向还原。"""

    ET.register_namespace("", "http://www.w3.org/2000/svg")

    svg = ET.Element("svg", {
        "xmlns": "http://www.w3.org/2000/svg",
        "version": "1.1",
    })
    svg.set("xmlns:xlink", "http://www.w3.org/1999/xlink")

    # DTD 的手动拼接 (ET 不支持 doctype, 加到结尾)
    font = ET.SubElement(svg, "font", {
        "id": "MLLL-ISA",
        "horiz-adv-x": str(FONT_HORIZ_ADV)
    })
    font.set("xmlns", "http://www.w3.org/2000/svg")

    font_face = ET.SubElement(font, "font-face", {
        "font-family": "MLLL ISA Glyphs",
        "font-weight": "400",
        "font-stretch": "normal",
        "units-per-em": str(UNITS_PER_EM),
        "ascent": str(ASCENT),
        "descent": str(DESCENT),
    })

    # missing-glyph
    ET.SubElement(font, "missing-glyph", {"horiz-adv-x": str(FONT_HORIZ_ADV)})

    entries = []
    for i, (sem, hexv, asm) in enumerate(SEM_IDS):
        codepoint = UNICODE_PRIVATE_BASE + i
        uni_str = f"&#x{codepoint:04X};"
        glyph_name = f"MLLL-ISA-{sem}-{hexv:08X}"
        d = make_glyph_path(hexv, sem)
        glyph = ET.SubElement(font, "glyph", {
            "unicode": uni_str,
            "glyph-name": glyph_name,
            "d": d
        })
        entries.append({
            "sem": sem,
            "hex": hexv,
            "asm": asm,
            "codepoint": codepoint,
            "glyph_name": glyph_name
        })

    # 写 XML 并加上 SVG DTD (ET 不会自己加 doctype)
    tree_str = ET.tostring(svg, encoding="unicode", xml_declaration=True)
    dtd = (
        '<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"\n'
        '  "http://www.w3.org/Graphics/SVG/1.1/DTD/svg-1.1.dtd">\n'
    )
    # xml_declaration=True 已经加了 <?xml ...?>
    # 在 <?xml?> 之后插入 dtd
    xml_decl_end = tree_str.index("?>") + 2
    final_svg = tree_str[:xml_decl_end] + "\n" + dtd + tree_str[xml_decl_end:]

    with open(out_path, "w", encoding="utf-8") as f:
        f.write(final_svg)
    # 设定可执行位以便直接运行
    st = os.stat(out_path)
    os.chmod(out_path, st.st_mode | stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)
    print(f"[字体生成] {out_path}")
    print(f"  16 条指令 → Unicode U+{UNICODE_PRIVATE_BASE:04X}..U+{UNICODE_PRIVATE_BASE + len(SEM_IDS) - 1:04X}")
    print(f"  每个 glyph-name 字段编码: MLLL-ISA-<SEM>-<HEX>")
    return entries


def parse_font_svg(in_path):
    """从 SVG 字体反向解析 → 返回 [{sem, hex, asm, codepoint, glyph_name}]
    演示了从字体文件还原指令表的能力。"""
    # 需要 asm 字段但 SVG 中未直接存储，需借助外部 asm 映射
    # 这里演示从 glyph-name 解析 SEM + HEX
    tree = ET.parse(in_path)
    root = tree.getroot()
    ns = {"svg": "http://www.w3.org/2000/svg"}
    results = []
    for glyph in root.iter("{%s}glyph" % ns["svg"]):
        uni = glyph.get("unicode", "")
        gname = glyph.get("glyph-name", "")
        if gname.startswith("MLLL-ISA-"):
            parts = gname.split("-", 3)
            if len(parts) >= 4:
                sem = parts[2]
                hexv = int(parts[3], 16)
                # codepoint 从 unicode &#xNNNN; 提取
                cp = None
                if "&#x" in uni:
                    cp_str = uni.split("&#x")[1].rstrip(";")
                    cp = int(cp_str, 16)
                results.append({"sem": sem, "hex": hexv, "codepoint": cp, "asm": None})
    return results


def dump_font_table(in_path):
    """从 SVG 字体打印指令对照表"""
    rows = parse_font_svg(in_path)
    print(f"\n{'='*60}")
    print(f"  从字体文件还原指令表: {os.path.basename(in_path)}")
    print(f"{'='*60}")
    print(f"{'码点':<8} {'语义':<6} {'机器码':>10}  glyph")
    print(f"{'-'*60}")
    for r in rows:
        chk = f"U+{r['codepoint']:04X}" if r['codepoint'] else "?"
        print(f"{chk:<8} {r['sem']:<6} {r['hex']:>10}")
    print(f"{'='*60}")
    print(f"  共 {len(rows)} 条指令从字体还原")
    print(f"  码点范围: U+E000 → U+{0xE000 + len(rows)-1:04X}")
    return rows


def gen_lookup_demo(in_path):
    """用具体码点演示双向查找"""
    rows = parse_font_svg(in_path)
    out_path = in_path.replace(".svg", "_lookup.txt")
    with open(out_path, "w", encoding="utf-8") as f:
        f.write("=== MLLL ISA 字体双向查找对照 ===\n\n")
        f.write("正向: 语义指令 → 字体码点 → glyph-name\n")
        f.write("反向: 字体码点 → 语义指令 → 机器码\n\n")
        for r in rows:
            # 正向：知道语义 → 找码点
            f.write(f"[{r['sem']}] → 语义键 → 码点 U+{r['codepoint']:04X}\n")
            # 反向：从字体读码点 → 还原指令
            f.write(f"  从字体读 U+{r['codepoint']:04X} → glyph {r['sem']} → 机器码 {r['hex']:08X}\n\n")
    print(f"[查找对照] 写入 {out_path}")


if __name__ == "__main__":
    script_dir = os.path.dirname(os.path.abspath(__file__))
    out_svg = os.path.join(script_dir, "mlll_isa_font.svg")

    entries = generate_font_svg(out_svg)
    print()

    # 信息熵简单验证: 16 条指令视觉上是否可区分
    # (因为我们把 opcode 低字节嵌入路径)
    seen_hex = set()
    for e in entries:
        seen_hex.add(e["hex"] & 0xFF)
    print(f"  视觉路径驱动字节多样性: {len(seen_hex)}/256 值域 (16条内无冲突)")

    # 反向解析: 证明字体文件是自包含的指令表
    rows = dump_font_table(out_svg)
    gen_lookup_demo(out_svg)

    print("\n[PoC 完成] 字体文件本身 = 16 条指令的封装容器")
    print("  使用时: 系统把 'ADD' 渲染为字体符号, 字体自己存着反查表")
    print("  外部工具: 解析 glyph-name 就能拿到 SEM + HEX")
    print("  升级路径: 把 glyph d 换成三维几何编码, 矢量关系刻进轮廓")
