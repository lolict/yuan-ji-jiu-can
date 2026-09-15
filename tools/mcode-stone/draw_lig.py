#!/usr/bin/env python3
# ============================================================
# 满全法 · 真正融合造字引擎
#
# 真正的造字：两个/三个字母笔画融合成一个新字形。
#
# 融合规则：
#   - 共用笔画：相邻字母衔接处的独立笔画合并为一
#   - 相位变换：上下(TB)、左右(LR)、里外(IO)、半包围(SS)
#   - 减法设计：去掉冗余笔画
#   - 容器编码：基于满全法 radix-hanzi 系统
#
# 与 lig_font.py 的区别：
#   lig_font.py 只是并置摆放
#   draw_lig.py 真正设计融合后的新笔画路径
#
# 身份落款：刘楚恬和我
# ============================================================

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
import math, os

EM = 1000

# ============================================================
# 笔画到容器的映射
# ============================================================
STROKE_TO_CONTAINER = {
    "横": ("袋", 0), "竖": ("桶", 1), "撇": ("罐", 2),
    "捺": ("盆", 3), "点": ("盂", 4), "提": ("壶", 5),
    "横折": ("盒", 6), "竖折": ("瓶", 7), "横撇": ("盏", 8),
    "竖钩": ("盎", 9), "横钩": ("篓", 10), "弯钩": ("筐", 11),
    "竖弯钩": ("篮", 12), "横折折": ("箩", 13),
}

# ============================================================
# PIECE — 基本几何单元
#
# 每个 piece 是闭合的轮廓点列表 [(x,y), ...]。
# 字母和融合字都由这些轮廓叠加组成。
# ============================================================

STROKE_W = 30  # 笔画宽度

def seg(x1, y1, x2, y2, w=STROKE_W):
    """线段 → 矩形轮廓"""
    dx, dy = x2-x1, y2-y1
    ln = max(math.sqrt(dx*dx+dy*dy), 0.001)
    nx, ny = -dy/ln*w/2, dx/ln*w/2
    return [(x1+nx,y1+ny),(x2+nx,y2+ny),(x2-nx,y2-ny),(x1-nx,y1-ny)]

def arc_outline(cx, cy, r, a1, a2, w=STROKE_W, steps=18):
    """弧线 → 环形轮廓"""
    out, inn = [], []
    for i in range(steps+1):
        ang = math.radians(a1+(a2-a1)*i/steps)
        c, s = math.cos(ang), math.sin(ang)
        out.append((cx+(r+w/2)*c, cy+(r+w/2)*s))
        inn.append((cx+(r-w/2)*c, cy+(r-w/2)*s))
    return out + list(reversed(inn))

def dot_outline(cx, cy, r=28):
    """圆点"""
    return [(cx+r*math.cos(math.radians(360*i/16)),
             cy+r*math.sin(math.radians(360*i/16))) for i in range(16)]


# ============================================================
# 字母轮廓定义 — 每个字母 = 轮廓列表
# 尺寸归一化到 宽600、高700, 居中于 x∈[200,800], y∈[100,700]
# ============================================================

LETTER_PIECES = {
    "a": lambda: [
        arc_outline(350, 380, 130, 0, 360),   # 圆
        seg(480, 380, 480, 100),               # 竖
        seg(480, 100, 520, 80),                # 勾
    ],
    "b": lambda: [
        seg(280, 150, 280, 700),
        arc_outline(400, 400, 90, -90, 90),
    ],
    "c": lambda: [arc_outline(400, 380, 130, 40, 320)],
    "d": lambda: [
        seg(480, 150, 480, 700),
        arc_outline(380, 400, 90, 90, 270),
    ],
    "e": lambda: [
        arc_outline(370, 380, 130, 20, 340),
        seg(370, 380, 500, 380),
    ],
    "f": lambda: [
        seg(320, 150, 320, 700),
        arc_outline(370, 350, 60, 60, 180),
        seg(320, 380, 460, 380),
    ],
    "g": lambda: [
        arc_outline(350, 380, 130, 0, 360),
        seg(480, 380, 480, 50),
        arc_outline(370, 50, 110, 180, 360),
    ],
    "h": lambda: [
        seg(240, 150, 240, 700),
        seg(480, 380, 480, 700),
        seg(240, 380, 480, 380),
    ],
    "i": lambda: [
        seg(350, 200, 350, 600),
        dot_outline(350, 170, 24),
    ],
    "j": lambda: [
        seg(400, 200, 400, 600),
        dot_outline(400, 170, 24),
        arc_outline(320, 600, 80, 200, 360),
    ],
    "k": lambda: [
        seg(280, 150, 280, 700),
        seg(360, 380, 500, 150),
        seg(360, 380, 500, 700),
    ],
    "l": lambda: [
        seg(350, 150, 350, 700),
        seg(350, 700, 440, 700),
    ],
    "m": lambda: [
        seg(180, 380, 180, 700),
        seg(350, 380, 350, 700),
        seg(520, 380, 520, 700),
        seg(180, 380, 380, 580),
        seg(380, 580, 520, 380),
    ],
    "n": lambda: [
        seg(240, 380, 240, 700),
        seg(480, 380, 480, 700),
        seg(240, 380, 500, 650),
    ],
    "o": lambda: [arc_outline(370, 380, 150, 0, 360)],
    "p": lambda: [
        seg(280, 150, 280, 700),
        arc_outline(400, 380, 90, -90, 90),
    ],
    "q": lambda: [
        arc_outline(370, 380, 150, 0, 360),
        seg(520, 380, 520, 700),
        seg(500, 680, 540, 680),
    ],
    "r": lambda: [
        seg(280, 380, 280, 700),
        arc_outline(350, 380, 70, -90, 60),
    ],
    "s": lambda: [
        arc_outline(370, 480, 90, 0, 180),
        arc_outline(370, 280, 90, 180, 360),
    ],
    "t": lambda: [
        seg(350, 150, 350, 700),
        seg(270, 280, 430, 280),
    ],
    "u": lambda: [
        seg(240, 200, 240, 500),
        seg(480, 200, 480, 500),
        arc_outline(360, 500, 120, 0, 180),
    ],
    "v": lambda: [
        seg(240, 200, 360, 700),
        seg(480, 200, 360, 700),
    ],
    "w": lambda: [
        seg(150, 200, 230, 700),
        seg(310, 450, 390, 700),
        seg(470, 450, 550, 700),
        seg(630, 200, 550, 700),
    ],
    "x": lambda: [
        seg(260, 200, 480, 700),
        seg(480, 200, 260, 700),
    ],
    "y": lambda: [
        seg(240, 200, 360, 450),
        seg(480, 200, 360, 450),
        seg(360, 450, 360, 700),
    ],
    "z": lambda: [
        seg(260, 220, 480, 220),
        seg(480, 220, 260, 650),
        seg(260, 650, 480, 650),
    ],
}


# ============================================================
# 融合路径生成器 — 每个融合字 = 轮廓列表（已融合）
#
# 这里设计真正的融合：不是把两个字母并排放，
# 而是重新设计共用笔画后的完整轮廓。
#
# 例：an = a 的尾勾 + n 的左竖合并为同一笔
#     → 圆头 + 共用竖 + 右半边 n
# ============================================================

def an_pieces():
    """an: a的圆 + n的拱, a的尾竖即n的左竖 (共用)"""
    return [
        arc_outline(300, 380, 110, 0, 360),     # a 圆头 (左)
        seg(410, 380, 410, 650),                 # 共用竖
        seg(550, 380, 550, 650),                 # n 右竖
        seg(410, 380, 560, 600),                 # n 斜
    ]

def en_pieces():
    """en: e 的开口对接 n 的左竖"""
    return [
        arc_outline(320, 380, 110, 25, 335),     # e 大半圆
        seg(430, 380, 430, 650),                 # e 的封口 = n 的左竖 (共用)
        seg(550, 380, 550, 650),                 # n 右竖
        seg(430, 380, 560, 600),                 # n 斜
    ]

def in_pieces():
    """in: i 点 + n"""
    return [
        dot_outline(350, 170, 22),
        seg(350, 200, 350, 650),                 # i 竖 = n 左竖
        seg(500, 380, 500, 650),                 # n 右竖
        seg(350, 380, 510, 600),                 # n 斜
    ]

def un_pieces():
    """un: u 的右竖 = n 的左竖"""
    return [
        seg(240, 200, 240, 500),
        seg(480, 200, 480, 500),
        arc_outline(360, 500, 120, 0, 180),
        # n
        seg(520, 380, 520, 650),
        seg(405, 380, 530, 600),
    ]

def vn_pieces():
    """ün: v(两点) + n"""
    return [
        seg(240, 200, 360, 700),
        seg(480, 200, 360, 700),
        # n
        seg(360, 380, 360, 650),
        seg(500, 380, 500, 650),
        seg(360, 380, 510, 600),
    ]

def ai_pieces():
    """ai: a 尾竖顶端加点"""
    return [
        arc_outline(320, 380, 110, 0, 360),
        seg(430, 380, 430, 120),
        dot_outline(430, 95, 22),
    ]

def ao_pieces():
    """ao: a 的尾勾延伸成 o 的左半"""
    return [
        arc_outline(300, 380, 110, 0, 360),       # a
        arc_outline(470, 380, 110, 0, 360),       # o
        # 中间的衔接横
        seg(410, 380, 430, 380),
    ]

def ou_pieces():
    """ou: o 右竖 = u 左竖 (半包围 SS)"""
    return [
        arc_outline(320, 380, 110, 0, 360),       # o
        # 共用竖: o 右 = u 左
        seg(430, 200, 430, 500),
        seg(550, 200, 550, 500),
        arc_outline(490, 500, 60, 0, 180),        # u 底
    ]

def ei_pieces():
    """ei: e 封口延伸成 i 竖"""
    return [
        arc_outline(320, 380, 110, 25, 335),
        seg(430, 380, 430, 120),                  # 共用竖
        dot_outline(430, 95, 22),
    ]

def ie_pieces():
    """ie: i 点在上 + e 在下"""
    return [
        dot_outline(350, 170, 22),
        seg(350, 200, 350, 280),                  # i 短竖
        arc_outline(370, 430, 100, 20, 340),      # e
        seg(370, 430, 470, 430),
    ]

def uo_pieces():
    """uo: u 在下, o 在上"""
    return [
        arc_outline(360, 300, 90, 0, 360),        # o 小在上
        seg(240, 450, 240, 650),
        seg(480, 450, 480, 650),
        arc_outline(360, 650, 120, 0, 180),
    ]

def io_pieces():
    """io: i 点 + o"""
    return [
        dot_outline(350, 170, 22),
        seg(350, 200, 350, 280),
        arc_outline(420, 430, 80, 0, 360),
    ]

def ue_pieces():
    """ue: u 左 + e 右"""
    return [
        seg(240, 200, 240, 500),
        seg(480, 200, 480, 500),
        arc_outline(360, 500, 120, 0, 180),       # u 底
        arc_outline(430, 350, 80, 20, 340),       # e
        seg(430, 350, 500, 350),
    ]

def ui_pieces():
    """ui: u + i"""
    return [
        seg(240, 200, 240, 500),
        seg(480, 200, 480, 500),
        arc_outline(360, 500, 120, 0, 180),
        dot_outline(550, 170, 22),
        seg(550, 200, 550, 300),
    ]

def iu_pieces():
    """iu: i + u"""
    return [
        dot_outline(350, 170, 22),
        seg(350, 200, 350, 300),
        seg(450, 300, 450, 550),
        seg(580, 300, 580, 550),
        arc_outline(515, 550, 65, 0, 180),
    ]

def eo_pieces():
    """eo: e 里包 o (IO)"""
    return [
        arc_outline(370, 380, 140, 25, 335),      # e 外
        arc_outline(370, 380, 60, 0, 360),        # o 内核
    ]

def oe_pieces():
    """oe: o 里包 e"""
    return [
        arc_outline(370, 380, 140, 0, 360),
        arc_outline(370, 380, 60, 20, 340),
    ]

def eu_pieces():
    """eu: e 里包 u (IO)"""
    return [
        arc_outline(370, 380, 150, 25, 335),      # e 外
        seg(370-50, 380, 370-50, 500),            # u 左
        seg(370+50, 380, 370+50, 500),            # u 右
    ]

def ch_pieces():
    return [
        arc_outline(350, 380, 100, 40, 320),      # c
        seg(480, 280, 480, 650),                  # h 右竖
        seg(380, 380, 480, 380),                  # h 横
    ]

def sh_pieces():
    return [
        arc_outline(350, 450, 80, 0, 180),
        arc_outline(350, 300, 80, 180, 360),
        seg(480, 280, 480, 650),
        seg(380, 380, 480, 380),
    ]

def zh_pieces():
    return [
        seg(280, 220, 480, 220),
        seg(480, 220, 280, 400),
        seg(280, 400, 480, 400),
        seg(540, 280, 540, 650),
        seg(440, 380, 540, 380),
    ]

def ang_pieces():
    """ang: a + ng (LR)"""
    return [
        # a
        arc_outline(250, 380, 110, 0, 360),
        seg(360, 380, 360, 650),                  # a 尾 = n 左竖
        # ng
        seg(500, 380, 500, 650),
        seg(360, 380, 510, 600),
        seg(500, 650, 460, 700),                  # g 的钩
    ]

def eng_pieces():
    return [
        arc_outline(320, 380, 110, 25, 335),
        seg(430, 380, 430, 650),
        seg(550, 380, 550, 650),
        seg(430, 380, 560, 600),
        seg(550, 650, 510, 700),
    ]

def ing_pieces():
    return [
        dot_outline(350, 170, 22),
        seg(350, 200, 350, 300),
        seg(350, 300, 350, 650),
        seg(500, 300, 500, 650),
        seg(350, 300, 510, 550),
        seg(500, 650, 460, 700),
    ]

def ong_pieces():
    """ong: o + ng"""
    return [
        arc_outline(280, 380, 100, 0, 360),
        # ng
        seg(420, 380, 420, 650),
        seg(550, 380, 550, 650),
        seg(420, 380, 560, 600),
        seg(550, 650, 510, 700),

    ]

def ian_pieces():
    """ian: i + an"""
    return [
        dot_outline(310, 170, 20),
        seg(310, 200, 310, 260),
        # an
        arc_outline(380, 380, 110, 0, 360),
        seg(490, 380, 490, 650),
        seg(490, 380, 490, 650),
        seg(610, 380, 610, 650),
        seg(490, 380, 620, 600),
    ]

def uan_pieces():
    return [
        seg(240, 200, 240, 400),
        seg(380, 200, 380, 400),
        arc_outline(310, 400, 70, 0, 180),
        # an
        arc_outline(410, 380, 110, 0, 360),
        seg(520, 380, 520, 650),
        seg(640, 380, 640, 650),
        seg(520, 380, 650, 600),
    ]

def iao_pieces():
    return [
        dot_outline(280, 170, 20),
        seg(280, 200, 280, 260),
        arc_outline(350, 350, 90, 0, 360),
        arc_outline(470, 350, 90, 0, 360),
        seg(440, 350, 450, 350),
    ]

def uai_pieces():
    """uai: 上下结构，u 在下, ai 概括"""
    return [
        seg(240, 400, 240, 550),
        seg(360, 400, 360, 550),
        arc_outline(300, 550, 60, 0, 180),
        # ai
        arc_outline(410, 350, 90, 0, 360),
        seg(500, 350, 500, 100),
        dot_outline(500, 75, 18),
    ]

def iong_pieces():
    return [
        dot_outline(310, 170, 20),
        seg(310, 200, 310, 260),
        # ong
        arc_outline(380, 380, 80, 0, 360),
        seg(460, 380, 460, 650),
        seg(580, 380, 580, 650),
        seg(460, 380, 590, 600),
        seg(580, 650, 540, 700),
    ]


# ============================================================
# 融合字注册表 + 字体生成
# ============================================================

LIGATURE_DEFS = [
    # name, piece_fn, pinyin, components, mode
    ("an",   an_pieces,   "an",    "a+n",    "LR"),
    ("en",   en_pieces,   "en",    "e+n",    "LR"),
    ("in",   in_pieces,   "in",    "i+n",    "LR"),
    ("un",   un_pieces,   "un",    "u+n",    "LR"),
    ("vn",   vn_pieces,   "ün",    "v+n",    "LR"),
    ("ai",   ai_pieces,   "ai",    "a+i",    "LR"),
    ("ei",   ei_pieces,   "ei",    "e+i",    "LR"),
    ("ao",   ao_pieces,   "ao",    "a+o",    "LR"),
    ("ou",   ou_pieces,   "ou",    "o+u",    "SS"),
    ("ie",   ie_pieces,   "ie",    "i+e",    "TB"),
    ("uo",   uo_pieces,   "uo",    "u+o",    "TB"),
    ("io",   io_pieces,   "io",    "i+o",    "TB"),
    ("ue",   ue_pieces,   "ue",    "u+e",    "LR"),
    ("ui",   ui_pieces,   "ui",    "u+i",    "LR"),
    ("iu",   iu_pieces,   "iu",    "i+u",    "LR"),
    ("eo",   eo_pieces,   "eo",    "e+o",    "IO"),
    ("oe",   oe_pieces,   "oe",    "o+e",    "IO"),
    ("eu",   eu_pieces,   "eu",    "e+u",    "IO"),
    ("ang",  ang_pieces,  "ang",   "a+ng",  "LR"),
    ("eng",  eng_pieces,  "eng",   "e+ng",  "LR"),
    ("ing",  ing_pieces,  "ing",   "i+ng",  "LR"),
    ("ong",  ong_pieces,  "ong",   "o+ng",  "LR"),
    ("ian",  ian_pieces,  "ian",   "i+an",  "LR"),
    ("uan",  uan_pieces,  "uan",   "u+an",  "TB"),
    ("iao",  iao_pieces,  "iao",   "i+ao",  "TB"),
    ("uai",  uai_pieces,  "uai",   "u+ai",  "TB"),
    ("iong", iong_pieces, "iong",  "i+ong", "IO"),
    ("ch",   ch_pieces,   "ch",    "c+h",    "LR"),
    ("sh",   sh_pieces,   "sh",    "s+h",    "LR"),
    ("zh",   zh_pieces,   "zh",    "z+h",    "LR"),
]


def pieces_to_glyph(pieces_list, pen):
    """将一系列轮廓绘制到 TTGlyphPen"""
    for poly in pieces_list:
        pts = list(poly)
        if len(pts) < 3:
            continue
        pen.moveTo(pts[0])
        for p in pts[1:]:
            pen.lineTo(p)
        pen.closePath()


def build_ligature_font():
    """
    构建字体 — 封装在我们自己的编码里 (container_radix)

    编码方案 (Symbol font 语义, platformID=3, encodingID=0):
      0x20-0x2F: 声母容器 (b p m f / d t n l / g k h / j q x / z c s r y w, 共16码)
      0x30-0x3F: 韵母容器 (a e i o u ü + 复韵母空间)
      0xD0-0xED: 融合字容器 (an en in un ün ai ei ao ou ie uo ue ui iu eo oe eu ang eng ing ong ian uan iao uai iong ch sh zh, 共30码)

    编码语义与 hunyuan-rs/src/container_radix.rs 对齐:
      声母=丈夫主动=阳, 韵母=妻子包容=阴, 融合=双方共生=元
    构建第二条后备映射表 (encodingID=10 UCS-4), 使 Unicode 兼容层也能被硬件渲染, 但真正数据码点是上面这些。
    """

    glyph_order = [".notdef"]
    recipe = []

    # ── 自有码点表 (platformID=3, encodingID=0 — 纯 Symbol, 我们自己的编码) ──
    OWN_CODE = {}

    # 声母容器 0x20-0x16 (16码, 覆盖 20 个基本声母)
    initials = "bpfdtnlgkhjqxzcsryw"
    for i, ch in enumerate(initials):
        OWN_CODE[ch] = 0x20 + i  # 0x20..0x35

    # 韵母容器: 6 个基本元音 + 复韵母占位
    vowels = "aeiouv"  # v = ü
    for i, v in enumerate(vowels):
        OWN_CODE[v] = 0x40 + i  # 0x40..0x45 (避开 0x30-0x3F, 留作未来扩展)

    # 融合字容器 0xD0-0xED (30 码)
    LIG_CODES = {}
    lig_base = 0xD0
    for i, entry in enumerate(LIGATURE_DEFS):
        name = entry[0]
        LIG_CODES[name] = lig_base + i  # 0xD0..0xED

    # ── 建立 glyph_order + 双 cmap ──
    # 主 cmap (自有 cmapID=0): 编码码点
    cmap_own = {}
    # 后备 cmap (Unicode BMP): 给没有 Symbol 渲染引擎的系统用
    cmap_unicode = {}

    def add_glyph(gname):
        glyph_order.append(gname)
        return len(glyph_order) - 1

    # 声母字
    for ch in initials:
        add_glyph(ch)
        cmap_own[OWN_CODE[ch]] = ch
        cmap_unicode[ord(ch)] = ch  # 备用

    # 韵母字
    for v in vowels:
        vname = "u_umlaut" if v == "v" else v
        add_glyph(vname)
        cmap_own[OWN_CODE[v]] = vname
        cmap_unicode[ord(v)] = vname

    # 融合字
    LIG_NAMES = set()
    for entry in LIGATURE_DEFS:
        name, fn, pinyin, components, mode = entry
        if name not in LIG_NAMES:
            gname = f"lig.{name}"
            add_glyph(gname)
            code = LIG_CODES[name]
            cmap_own[code] = gname
            # Unicode 后备: 用 FA00+ 区域, 仅作兼容显示用
            cmap_unicode[0xFA00 + len(LIG_NAMES)] = gname
            recipe.append((name, pinyin, components, mode, f"C{code:02X}"))
            LIG_NAMES.add(name)

    # ── 构建字体表面 ──
    upm = 1000
    advance = 650
    box_pen = TTGlyphPen(None)
    box_pen.moveTo((100,0)); box_pen.lineTo((500,0))
    box_pen.lineTo((500,700)); box_pen.lineTo((100,700)); box_pen.closePath()
    box_glyph = box_pen.glyph()

    fb = FontBuilder(upm, isTTF=True)
    fb.setupGlyphOrder(glyph_order)

    glyf = {}
    metrics = {}

    for gname in glyph_order:
        if gname == ".notdef":
            glyf[gname] = box_glyph; metrics[gname] = (0, 0)
        elif gname.startswith("lig."):
            name = gname[4:]
            p = TTGlyphPen(None)
            piece_fn = next((fn for n, fn, *_ in LIGATURE_DEFS if n == name), None)
            if piece_fn:
                pieces_to_glyph(piece_fn(), p)
            glyf[gname] = p.glyph(); metrics[gname] = (advance, 0)
        elif gname in LETTER_PIECES:
            p = TTGlyphPen(None)
            pieces_to_glyph(LETTER_PIECES[gname](), p)
            glyf[gname] = p.glyph(); metrics[gname] = (advance, 0)
        elif gname == "u_umlaut":
            p = TTGlyphPen(None)
            pieces_to_glyph(LETTER_PIECES["v"](), p)
            glyf[gname] = p.glyph(); metrics[gname] = (advance, 0)
        else:
            glyf[gname] = box_glyph; metrics[gname] = (advance, 0)

    fb.setupGlyf(glyf)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)

    # 双 cmap 策略:
    #   1. 自有编码表 (platformID=3, encodingID=0, Symbol 语义)
    #   2. Unicode BMP 后备 (platformID=3, encodingID=1, UCS-2)
    # FontBuilder 的 setupCharacterMap 默认建 Unicode BMP, 之后通过字体对象插入 Symbol subtable

    fb.setupCharacterMap(cmap_unicode)  # 后备表
    fb.setupNameTable({
        "familyName": "MQF Container Radix Fused",
        "styleName": "Fused",
        "psName": "MQF-ContainerRadix-Fused",
        "manufacturer": "囝囝共生 · 刘楚恬和我",
        "designer": "lolict + 刘楚恬",
        "description": "满全法容器进制融合造字: Symbol cmap 0x20-0xED · 容器自有编码",
        "vendorURL": "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private",
        "version": "0.1.0",
    })
    fb.setupOS2(sTypoAscender=800, sTypoDescent=-200, usWinAscent=800, usWinDescent=200)
    fb.setupPost()

    font = fb.font

    # 插入自有 Symbol cmap subtable (platformID=3, encodingID=0)，放在字体 cmap 表最前面，使其优先于 Unicode 后备表
    from fontTools.ttLib.tables._c_m_a_p import CmapSubtable
    cmap_table = font["cmap"]
    sym = CmapSubtable.newSubtable(4)
    sym.platformID = 3
    sym.platEncID = 0   # Symbol semantics
    sym.format = 4
    sym.reserved = 0
    sym.length = 0
    sym.language = 0
    sym.cmap = cmap_own
    cmap_table.tables.insert(0, sym)

    name_table = font["name"]
    for i, (name, pinyin, comp, mode, cp) in enumerate(recipe):
        rec = f"{name}({pinyin})={comp}[{mode}]@{cp}"
        name_table.setName(rec, 256 + i, 3, 1, 0x0409)

    out = "mqf_radix_fused.ttf"
    font.save(out)
    font.close()
    return out, len(glyph_order), len(recipe), recipe


if __name__ == "__main__":
    out, total, n, rec = build_ligature_font()
    size = os.path.getsize(out)
    print(f"字体: {out} ({size} bytes)")
    print(f"总字形: {total}")
    print(f"融合字: {n}")
    print("\n融合字列表:")
    for name, pinyin, comp, mode, cp in rec:
        print(f"  {name:8s} ({pinyin:4s}) = {comp:8s} [{mode}] @ {cp}")
