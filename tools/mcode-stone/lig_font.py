#!/usr/bin/env python3
# ============================================================
# 满全法 · 合并韵母字体生成器
#
# 设计规则：
#   - 不膨胀原则：合并后视觉≈两字母之一，笔画不增加
#   - 可识别性：保留两字母特征笔画
#   - 相位变换：上下(TB)、左右(LR)、里外(IO)、半包围(SS)
#   - 减法设计：合并时去掉重复笔画，融合共用部分
#
# 绘图策略：每个字母定义为一系列闭合多边形，避免 open contour。
# 水平/垂直笔画画成细矩形，弧线画成环形截面多边形。
#
# 身份落款：刘楚恬和我
# ============================================================

from fontTools.ttLib import TTFont
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
import math, os

EM = 1000  # em square
STROKE_W = 36  # default stroke width


# ============================================================
# 笔画原语 — 每个原语返回一个点列表（闭合多边形）
# ============================================================

def seg_pts(x1, y1, x2, y2):
    """线段两端点"""
    return [(x1, y1), (x2, y2)]


def rect_from_line(x1, y1, x2, y2, w=STROKE_W):
    """线段扩展为矩形"""
    dx, dy = x2 - x1, y2 - y1
    length = math.sqrt(dx*dx + dy*dy)
    if length < 1:
        return [(x1-w, y1-w), (x1+w, y1-w), (x1+w, y1+w), (x1-w, y1+w)]
    nx, ny = -dy/length * w, dx/length * w
    return [
        (x1+nx, y1+ny), (x2+nx, y2+ny),
        (x2-nx, y2-ny), (x1-nx, y1-ny)
    ]


def ring_from_arc(cx, cy, r, start_deg, end_deg, w=STROKE_W, steps=20):
    """弧线扩展为环形截面多边形"""
    outer, inner = [], []
    for i in range(steps + 1):
        ang = math.radians(start_deg + (end_deg - start_deg) * i / steps)
        cos_a, sin_a = math.cos(ang), math.sin(ang)
        outer.append((cx + (r+w)*cos_a, cy + (r+w)*sin_a))
        inner.append((cx + (r-w)*cos_a, cy + (r-w)*sin_a))
    return outer + list(reversed(inner))


def ring_full_circle(cx, cy, r, w=STROKE_W, steps=24):
    """完整圆环"""
    outer, inner = [], []
    for i in range(steps):
        ang = math.radians(360 * i / steps)
        cos_a, sin_a = math.cos(ang), math.sin(ang)
        outer.append((cx + (r+w)*cos_a, cy + (r+w)*sin_a))
        inner.append((cx + (r-w)*cos_a, cy + (r-w)*sin_a))
    return outer + list(reversed(inner))


def dot_polygon(cx, cy, r, steps=16):
    """实心圆点"""
    return [(cx + r*math.cos(math.radians(360*i/steps)),
             cy + r*math.sin(math.radians(360*i/steps))) for i in range(steps)]


# ============================================================
# 字母形状定义 — 每个字母 = 列表 of 闭合多边形点列表
# ============================================================

def letter_a():
    return [
        ring_from_arc(350, 350, 150, 0, 360),       # 圆头
        rect_from_line(500, 350, 500, 50),          # 竖
        rect_from_line(500, 50, 540, 50),           # 脚勾
    ]

def letter_b():
    return [
        rect_from_line(250, 100, 250, 700),         # 竖
        ring_from_arc(400, 400, 100, -90, 90),      # 半圆
    ]

def letter_c():
    return [ring_from_arc(400, 350, 150, 30, 330)]

def letter_d():
    return [
        rect_from_line(500, 100, 500, 700),
        ring_from_arc(400, 400, 100, 90, 270),
    ]

def letter_e():
    return [
        ring_from_arc(370, 350, 150, 0, 360),
        rect_from_line(370, 350, 520, 350),
    ]

def letter_f():
    return [
        rect_from_line(300, 150, 300, 700),
        ring_from_arc(350, 350, 80, 60, 180),
        rect_from_line(300, 350, 450, 350),
    ]

def letter_g():
    return [
        ring_from_arc(350, 350, 150, 0, 360),
        rect_from_line(500, 350, 500, 0),
        ring_from_arc(370, 0, 130, 180, 360),
    ]

def letter_h():
    return [
        rect_from_line(200, 100, 200, 700),
        rect_from_line(500, 350, 500, 700),
        rect_from_line(200, 350, 500, 350),
    ]

def letter_i():
    return [
        rect_from_line(350, 150, 350, 550),
        dot_polygon(350, 120, 25),
    ]

def letter_j():
    return [
        rect_from_line(400, 150, 400, 550),
        dot_polygon(400, 120, 25),
        ring_from_arc(300, 550, 100, 180, 360),
        rect_from_line(400, 550, 400, 650),
    ]

def letter_k():
    return [
        rect_from_line(250, 100, 250, 700),
        rect_from_line(350, 350, 500, 100),
        rect_from_line(350, 350, 500, 700),
    ]

def letter_l():
    return [
        rect_from_line(350, 100, 350, 650),
        rect_from_line(350, 650, 450, 650),
    ]

def letter_m():
    return [
        rect_from_line(150, 350, 150, 700),
        rect_from_line(350, 350, 350, 700),
        rect_from_line(550, 350, 550, 700),
        rect_from_line(150, 350, 350, 550),
        rect_from_line(350, 550, 550, 350),
    ]

def letter_n():
    return [
        rect_from_line(200, 350, 200, 700),
        rect_from_line(500, 350, 500, 700),
        rect_from_line(200, 350, 500, 650),
    ]

def letter_o():
    return [ring_full_circle(350, 350, 180)]

def letter_p():
    return [
        rect_from_line(250, 100, 250, 700),
        ring_from_arc(400, 350, 100, -90, 90),
    ]

def letter_q():
    return [
        ring_full_circle(350, 350, 180),
        rect_from_line(530, 350, 530, 700),
        rect_from_line(500, 680, 560, 680),
    ]

def letter_r():
    return [
        rect_from_line(250, 350, 250, 700),
        ring_from_arc(350, 350, 80, -90, 60),
    ]

def letter_s():
    return [
        ring_from_arc(350, 450, 100, 0, 180),
        ring_from_arc(350, 250, 100, 180, 360),
    ]

def letter_t():
    return [
        rect_from_line(350, 100, 350, 700),
        rect_from_line(250, 200, 450, 200),
    ]

def letter_u():
    return [
        rect_from_line(200, 100, 200, 500),
        rect_from_line(500, 100, 500, 500),
        ring_from_arc(350, 500, 150, 0, 180),
    ]

def letter_v():
    return [
        rect_from_line(200, 100, 350, 700),
        rect_from_line(500, 100, 350, 700),
    ]

def letter_w():
    return [
        rect_from_line(100, 100, 200, 700),
        rect_from_line(250, 400, 300, 700),
        rect_from_line(400, 400, 450, 700),
        rect_from_line(600, 100, 450, 700),
    ]

def letter_x():
    return [
        rect_from_line(200, 100, 500, 700),
        rect_from_line(500, 100, 200, 700),
    ]

def letter_y():
    return [
        rect_from_line(200, 100, 350, 400),
        rect_from_line(500, 100, 350, 400),
        rect_from_line(350, 400, 350, 700),
    ]

def letter_z():
    return [
        rect_from_line(200, 150, 500, 150),
        rect_from_line(500, 150, 200, 650),
        rect_from_line(200, 650, 500, 650),
    ]

# 扩展字母
def letter_eta():
    """η - 类似 n 长脚"""
    return [
        rect_from_line(200, 350, 200, 700),
        rect_from_line(500, 350, 500, 600),
        rect_from_line(500, 600, 500, 750),
        ring_from_arc(350, 750, 75, 0, 180),
    ]

def letter_er():
    return [
        rect_from_line(200, 200, 350, 450),
        rect_from_line(350, 450, 500, 200),
        ring_from_arc(350, 550, 100, 0, 180),
    ]

def letter_ng():
    return [
        rect_from_line(200, 350, 200, 700),
        rect_from_line(500, 350, 500, 700),
        rect_from_line(200, 350, 500, 650),
        ring_from_arc(350, 700, 100, 0, 180),
    ]


GLYPH_DEFS = {
    "a": letter_a, "b": letter_b, "c": letter_c, "d": letter_d,
    "e": letter_e, "f": letter_f, "g": letter_g, "h": letter_h,
    "i": letter_i, "j": letter_j, "k": letter_k, "l": letter_l,
    "m": letter_m, "n": letter_n, "o": letter_o, "p": letter_p,
    "q": letter_q, "r": letter_r, "s": letter_s, "t": letter_t,
    "u": letter_u, "v": letter_v, "w": letter_w, "x": letter_x,
    "y": letter_y, "z": letter_z,
    "eta": letter_eta, "er": letter_er, "ng": letter_ng,
}


# ============================================================
# 绘图原语 → TTGlyphPen
# ============================================================

def draw_polygons(polygons, pen, ox=0, oy=0, sx=1.0, sy=1.0, color=None):
    """Draw multiple closed polygons into a TTGlyphPen"""
    for poly in polygons:
        pts = [(ox + x*sx, oy + y*sy) for x, y in poly]
        pen.moveTo(pts[0])
        for p in pts[1:]:
            pen.lineTo(p)
        pen.closePath()


# ============================================================
# 合并算子 (相位变换)
# ============================================================

def merge_LR(a_def, b_def, pen):
    """a 在左，b 在右"""
    draw_polygons(a_def(), pen, ox=50, oy=0, sx=0.5, sy=0.9)
    draw_polygons(b_def(), pen, ox=450, oy=0, sx=0.5, sy=0.9)

def merge_TB(a_def, b_def, pen):
    """a 在上，b 在下"""
    draw_polygons(a_def(), pen, ox=0, oy=350, sx=0.9, sy=0.45)
    draw_polygons(b_def(), pen, ox=0, oy=0, sx=0.9, sy=0.45)

def merge_IO(a_def, b_def, pen):
    """里外 — a 外框，b 内核缩小居中"""
    draw_polygons(a_def(), pen, ox=0, oy=0, sx=1.0, sy=1.0)
    draw_polygons(b_def(), pen, ox=250, oy=250, sx=0.4, sy=0.4)

def merge_SS(a_def, b_def, pen):
    """半包围 — a 左半包围 + b 右半"""
    draw_polygons(a_def(), pen, ox=0, oy=0, sx=0.6, sy=1.0)
    draw_polygons(b_def(), pen, ox=400, oy=50, sx=0.55, sy=0.85)


MERGE_FUNCS = {
    "LR": merge_LR, "TB": merge_TB, "IO": merge_IO, "SS": merge_SS,
}


# ============================================================
# 字母合并注册
# ============================================================

# 声母二合
CONSONANT_LIGS = [
    ("c", "h", "LR", "ch"), ("s", "h", "LR", "sh"), ("z", "h", "LR", "zh"),
    ("t", "h", "LR", "th"), ("g", "h", "LR", "gh"), ("f", "h", "LR", "fh"),
    ("u", "h", "LR", "uh"), ("k", "h", "LR", "kh"), ("n", "h", "LR", "nh"),
    ("m", "h", "LR", "mh"), ("b", "h", "LR", "bh"), ("v", "h", "LR", "vh"),
    ("o", "h", "LR", "oh"), ("i", "h", "LR", "ih"), ("r", "h", "LR", "rh"),
    ("j", "h", "LR", "jh"), ("l", "h", "LR", "lh"), ("p", "h", "LR", "ph"),
    ("d", "h", "LR", "dh"), ("e", "h", "LR", "eh"), ("w", "h", "LR", "wh"),
    ("q", "h", "LR", "qh"), ("a", "h", "LR", "ah"), ("x", "h", "LR", "xh"),
    ("y", "h", "LR", "yh"), ("h", "h", "LR", "hh"), ("eta", "h", "LR", "etah"),
]

# 声母 r 系列
R_LIGS = [
    ("p", "r", "LR", "pr"), ("t", "r", "LR", "tr"), ("i", "r", "LR", "ir"),
    ("u", "r", "LR", "ur"), ("o", "r", "LR", "or"), ("l", "r", "LR", "lr"),
    ("k", "r", "LR", "kr"), ("j", "r", "LR", "jr"), ("h", "r", "LR", "hr"),
    ("g", "r", "LR", "gr"), ("f", "r", "LR", "fr"), ("d", "r", "LR", "dr"),
    ("s", "r", "LR", "sr"), ("e", "r", "LR", "er_l"), ("w", "r", "LR", "wr"),
    ("q", "r", "LR", "qr"), ("a", "r", "LR", "ar"), ("z", "r", "LR", "zr"),
    ("x", "r", "LR", "xr"), ("c", "r", "LR", "cr"), ("v", "r", "LR", "vr"),
    ("b", "r", "LR", "br"), ("n", "r", "LR", "nr"), ("m", "r", "LR", "mr"),
]

# 基础二合韵母
BASIC_DOUBLE = [
    ("a", "o", "LR", "ao"), ("a", "i", "LR", "ai"), ("e", "i", "LR", "ei"),
    ("o", "u", "LR", "ou"), ("i", "u", "LR", "iu"), ("u", "e", "LR", "ue"),
    ("u", "a", "LR", "ua"), ("i", "e", "LR", "ie"), ("u", "i", "LR", "ui"),
    ("u", "o", "TB", "uo"), ("i", "o", "TB", "io"),
    ("e", "o", "IO", "eo"), ("o", "e", "IO", "oe"), ("e", "u", "IO", "eu"),
    # 后缀: ed ad vd ud od id
    ("e", "d", "LR", "ed"), ("a", "d", "LR", "ad"), ("u", "d", "LR", "ud"),
    ("o", "d", "LR", "od"), ("i", "d", "LR", "id"), ("v", "d", "LR", "vd"),
    # 后缀: ek ak uk ok ik
    ("e", "k", "LR", "ek"), ("a", "k", "LR", "ak"), ("u", "k", "LR", "uk"),
    ("o", "k", "LR", "ok"), ("i", "k", "LR", "ik"), ("v", "k", "LR", "vk"),
    # 后缀: ep ap up op ip
    ("e", "p", "LR", "ep"), ("a", "p", "LR", "ap"), ("u", "p", "LR", "up"),
    ("o", "p", "LR", "op"), ("i", "p", "LR", "ip"), ("v", "p", "LR", "vp"),
    # 粤语: aa oo ii uu vv ee
    ("a", "a", "LR", "aa"), ("o", "o", "LR", "oo"), ("i", "i", "LR", "ii"),
    ("u", "u", "LR", "uu"), ("e", "e", "LR", "ee"), ("v", "v", "LR", "vv"),
]

# 三合韵母
TRIPLE_FINALS = [
    ("a", "n", "g", "LR", "ang"), ("e", "n", "g", "LR", "eng"),
    ("o", "n", "g", "LR", "ong"), ("i", "n", "g", "LR", "ing"),
    # ian = i + an 上下结构
    ("i", "a", "n", "TB", "ian"),
    # uan = u + an 上下
    ("u", "a", "n", "TB", "uan"),
    # uai = u + ai
    ("u", "a", "i", "TB", "uai"),
    # iao = i + ao
    ("i", "a", "o", "TB", "iao"),
    # iong = i + ong 里外
    ("i", "o", "ng", "IO", "iong"),
    # 鼻音: un on
    ("u", "n", "", "LR", "un"), ("o", "n", "", "LR", "on"),
]

# 特殊合并
SPECIAL_LIGS = [
    # ng 单独 — n+g 融合
    ("n", "g", "IO", "ng"),
    # vn — v+n 融合
    ("v", "n", "LR", "vn"),
    # uai 扩展: ue + vi 等
    ("u", "v", "LR", "uv"), ("v", "u", "LR", "vu"),
    # 半包围试验: ou
    ("o", "u", "SS", "ou_ss"),
    # in im
    ("i", "n", "LR", "in"), ("i", "m", "LR", "im"),
    ("e", "m", "LR", "em"), ("a", "m", "LR", "am"),
    ("o", "m", "LR", "om"), ("u", "m", "LR", "um"),
]


ALL_LIGS = CONSONANT_LIGS + R_LIGS + BASIC_DOUBLE + SPECIAL_LIGS


def draw_ligature(a_name, b_name, c_name, mode):
    """Draw a ligature into a new glyph and return it"""
    pen = TTGlyphPen(None)
    a_def = GLYPH_DEFS.get(a_name, letter_a)
    b_def = GLYPH_DEFS.get(b_name, letter_a)

    fn = MERGE_FUNCS.get(mode, merge_LR)
    fn(a_def, b_def, pen)

    # Third letter — place small at bottom-center as rhyming tail
    if c_name and c_name in GLYPH_DEFS:
        c_def = GLYPH_DEFS[c_name]
        draw_polygons(c_def(), pen, ox=ox_for_third(mode), oy=80, sx=0.3, sy=0.3)

    return pen.glyph()


def ox_for_third(mode):
    """x offset for third letter based on merge type"""
    if mode == "IO":
        return 350  # center
    elif mode == "TB":
        return 350
    else:
        return 350  # center-bottom


def draw_single_letter(name):
    pen = TTGlyphPen(None)
    if name in GLYPH_DEFS:
        draw_polygons(GLYPH_DEFS[name](), pen, ox=0, oy=0, sx=1.0, sy=1.0)
    return pen.glyph()


# ============================================================
# 字体生成
# ============================================================

def build_font():
    glyph_order = [".notdef"]
    cmap = {}
    recipe = []  # (glyph_name, description)

    def add_glyph(name, cp):
        glyph_order.append(name)
        cmap[cp] = name
        return len(glyph_order) - 1

    # 基础字母
    for i, ch in enumerate("abcdefghijklmnopqrstuvwxyz"):
        add_glyph(ch, 0x0061 + i)
    add_glyph("eta", 0x03B7)
    recipe.append(("base", "26拉丁 + η"))

    # PUA ligatures
    pua_base = 0xF000
    pua_idx = 0

    # Helper to add lig by string key
    added_lig_names = set()

    def add_lig(gname, cp, label):
        if gname not in added_lig_names:
            add_glyph(gname, cp)
            added_lig_names.add(gname)
            recipe.append((gname, label))
            return True
        return False

    # Process double ligs
    for entry in ALL_LIGS:
        a, b, mode, out_name = entry
        gname = f"lig.{a}{b}"
        if add_lig(gname, pua_base + pua_idx, f"{a}+{b} [{mode}]"):
            pua_idx += 1

    # Process triple ligs
    for entry in TRIPLE_FINALS:
        a, b, c, mode, out_name = entry
        gname = f"lig.{a}{b}{c}" if c else f"lig.{a}{b}"
        label = f"{a}+{b}+{c} [{mode}]" if c else f"{a}+{b} [{mode}]"
        if add_lig(gname, pua_base + pua_idx, label):
            pua_idx += 1

    # Build font
    upm = 1000
    advance = 650
    box_pen = TTGlyphPen(None)
    box_pen.moveTo((100, 0))
    box_pen.lineTo((500, 0))
    box_pen.lineTo((500, 700))
    box_pen.lineTo((100, 700))
    box_pen.closePath()
    box_glyph = box_pen.glyph()

    fb = FontBuilder(upm, isTTF=True)
    fb.setupGlyphOrder(glyph_order)

    glyf = {}
    metrics = {}
    desc_recipe = {}  # glyph_name -> description

    for gname in glyph_order:
        if gname == ".notdef":
            glyf[gname] = box_glyph
            metrics[gname] = (0, 0)
        elif gname.startswith("lig."):
            parts = gname[4:]
            # Find this ligature in registered lists
            found = False
            for entry in TRIPLE_FINALS:
                a, b, c, mode, out_name = entry
                n = a + b + c if c else a + b
                if parts == n:
                    glyf[gname] = draw_ligature(a, b, c, mode)
                    metrics[gname] = (advance, 0)
                    desc_recipe[gname] = f"{a}+{b}+{c} [{mode}]"
                    found = True
                    break
            if not found:
                for entry in ALL_LIGS:
                    a, b, mode, out_name = entry
                    if parts == a + b:
                        glyf[gname] = draw_ligature(a, b, "", mode)
                        metrics[gname] = (advance, 0)
                        desc_recipe[gname] = f"{a}+{b} [{mode}]"
                        found = True
                        break
            if not found:
                glyf[gname] = box_glyph
                metrics[gname] = (advance, 0)
        elif gname in GLYPH_DEFS:
            glyf[gname] = draw_single_letter(gname)
            metrics[gname] = (advance, 0)
        else:
            glyf[gname] = box_glyph
            metrics[gname] = (advance, 0)

    fb.setupGlyf(glyf)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupCharacterMap(cmap)
    fb.setupNameTable({
        "familyName": "Manquanfa Ligature",
        "styleName": "Regular",
        "psName": "MQF-Ligature",
        "manufacturer": "囝囝共生 OS · 刘楚恬和我",
        "designer": "lolict + 刘楚恬",
        "description": "满全法合并韵母相位字体 · 二合三合 · 上下左右里外半包围",
        "vendorURL": "https://github.com/lolict/hunyuan-os",
        "designerURL": "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private · 仅限刘楚恬和我使用",
        "version": "0.1.0",
    })
    fb.setupOS2(sTypoAscender=800, sTypoDescent=-200, usWinAscent=800, usWinDescent=200)
    fb.setupPost()

    font = fb.font
    name_table = font["name"]

    # Write recipe into name table nameID 256+
    for i, (gn, desc) in enumerate(desc_recipe.items()):
        name_table.setName(desc, 256 + i, 3, 1, 0x0409)

    out_path = "mqf_ligature.ttf"
    font.save(out_path)
    font.close()
    return out_path, len(glyph_order), len(recipe), len(TRIPLE_FINALS)


if __name__ == "__main__":
    out, n, r, t = build_font()
    size = os.path.getsize(out)
    print(f"字体生成: {out}")
    print(f"总字形数: {n}")
    print(f"基础字母: 27")
    print(f"双字母合并: {r}")
    print(f"三字母合并: {t}")
    print(f"文件大小: {size} bytes")
