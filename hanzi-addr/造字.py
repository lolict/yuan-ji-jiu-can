#!/usr/bin/env python3
# 造字.py · 码位表 → 私有字体（PUA）
# 读 码位表.toml，把每个门禁入口符号的私有码位写进一个字体。
# 有底字形的直接用（拉丁 a-z 取现存底字体）；没有的指向一个新画的「空框」字形。
# 这样私有码位立即可用、可排序；字形美术待有底字库再补。

import re
from fontTools.ttLib import TTFont
from fontTools.pens.ttGlyphPen import TTGlyphPen

HERE = "/workspace/hanzi-addr"
BASE = "/workspace/archive/ttf/mqf_radix_fused.ttf"
OUT = HERE + "/hanzi_addr_pua.ttf"
MAP = HERE + "/码位表.toml"
BOX = "uniBOX"


def 读码位(path):
    items = []
    sec = ""
    for line in open(path, encoding="utf-8"):
        s = line.strip()
        if not s or s.startswith("#"):
            continue
        if s.startswith("["):
            sec = s.strip("[]")
            continue
        m = re.match(r'^(.*?)\s*=\s*"U\+([0-9A-Fa-f]+)"', s)
        if m:
            items.append((sec, m.group(1), int(m.group(2), 16)))
    return items


def 加空框(font):
    upm = font["head"].unitsPerEm
    w = int(upm * 0.5)
    pen = TTGlyphPen(None)
    x0, y0 = int(upm * 0.08), 0
    x1, y1 = w - int(upm * 0.08), int(upm * 0.7)
    pen.moveTo((x0, y0))
    pen.lineTo((x0, y1))
    pen.lineTo((x1, y1))
    pen.lineTo((x1, y0))
    pen.closePath()
    font["glyf"][BOX] = pen.glyph()
    font["hmtx"][BOX] = (w, x0)
    order = font.getGlyphOrder()
    if BOX not in order:
        font.setGlyphOrder(order + [BOX])
    return w


def main():
    items = 读码位(MAP)
    font = TTFont(BASE)
    cmap = font.getBestCmap()
    have = set(cmap.keys())
    加空框(font)
    n_alias = n_box = 0
    for 门, sym, cp in items:
        if len(sym) == 1 and ord(sym) in have:
            gname = cmap[ord(sym)]
            n_alias += 1
        else:
            gname = BOX
            n_box += 1
        for table in font["cmap"].tables:
            table.cmap[cp] = gname
    font.save(OUT)
    print("造字完成：", OUT)
    print("  码位", len(items), "个；别名真形", n_alias, "；空框占位", n_box)
    print("  范围 U+%04X..U+%04X" % (items[0][2], items[-1][2]))


if __name__ == "__main__":
    main()
