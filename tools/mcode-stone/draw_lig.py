#!/usr/bin/env python3
# ============================================================
# 满全法 · 容器进制融合字体生成器 v3
#
# 身份落款: 刘楚恬和我
#
# 纪律:
#   只讲进制不讲 XYZ      — 地址 = 容器 × 行 × 极 · 段
#   进制地址编程收编参数   — 容器容量即乘即进位
#   不用数字代符号        — C01 不写 "袋(3)"
#
# 功能扩展 (v3):
#   + GSUB ligature 嵌入 — 打 a+n 字体自动出融合字形
#   + 三条 cmap: 自有编码 / Symbol / Unicode 后备
#   + 配方写进 name table 256+
#
# 容器编码分配:
#   C01-C0B  声母一群 (唇/舌/牙)
#   C10-C19  声母二群 (齿/喉/半)
#   C20-C25  韵母     (a e i o u ü)
#   C40-C5D  手工融合 30 组
#   C80-CFE  自动融合 ~110 组
#   FA00+    Unicode 后备
# ============================================================

import sys, os

try:
    from fontTools.ttLib import TTFont
    from fontTools.pens.ttGlyphPen import TTGlyphPen
    from fontTools.fontBuilder import FontBuilder
    from fontTools.ttLib.tables._c_m_a_p import CmapSubtable
except ImportError:
    print("安装依赖: pip install fonttools")
    sys.exit(1)

# ============================================================
# 容器表: 名 ↔ 容量 ↔ 极性 (进制即容器)
#   容量 = 乘法口诀表嵌入在容器定义中
# ============================================================

CONTAINERS = {
    # ── 声母一群: 唇音/舌音 ──
    "C01": ("袋", 3, "阳"),   # b p m f
    "C02": ("桶", 2, "阴"),   # d t n l
    "C03": ("罐", 3, "阳"),   # g k h
    "C04": ("盆", 4, "阴"),   # j q x
    "C05": ("盂", 2, "阳"),   # z c s r
    "C06": ("盎", 4, "阴"),   # y w v (半元音)
    # ── 声母二群: 延伸/复合 ──
    "C10": ("袋", 3, "阳"),   # 复合声母一组
    "C11": ("桶", 2, "阴"),
    "C12": ("罐", 3, "阳"),
    "C13": ("盆", 4, "阴"),
    "C14": ("盂", 2, "阳"),
    "C15": ("盎", 4, "阴"),
    "C16": ("钵", 3, "阳"),
    "C17": ("盏", 2, "阴"),
    "C18": ("盒", 4, "阳"),
    "C19": ("杯", 3, "阴"),
    # ── 韵母 ──
    "C20": ("钵", 3, "阳"),   # a
    "C21": ("盏", 2, "阴"),   # e
    "C22": ("盒", 4, "阳"),   # i
    "C23": ("杯", 3, "阴"),   # o
    "C24": ("壶", 2, "阳"),   # u
    "C25": ("瓶", 3, "阴"),   # ü (v)
}

# ── 容器索引名: 编码 → 名 ──
CODE_TO_NAME = {k: v[0] for k, v in CONTAINERS.items()}
CODE_TO_CAP  = {k: v[1] for k, v in CONTAINERS.items()}
CODE_TO_PHASE = {k: v[2] for k, v in CONTAINERS.items()}

# ── 声母/韵母路由 ──
INITIAL_ROUTE = {
    "b":"C01","p":"C01","m":"C01","f":"C01",
    "d":"C02","t":"C02","n":"C02","l":"C02",
    "g":"C03","k":"C03","h":"C03",
    "j":"C04","q":"C04","x":"C04",
    "z":"C05","c":"C05","s":"C05","r":"C05",
    "y":"C06","w":"C06","v":"C06",
    "zh":"C05","ch":"C05","sh":"C05",
}
VOWEL_ROUTE = {
    "a":"C20","e":"C21","i":"C22","o":"C23","u":"C24","v":"C25",
}

# ============================================================
# 字形笔划定义 (像素即地址, 结构即索引)
#   每个字母 = 一组多边形, 坐标 = 消耗路径
# ============================================================

def pen_letter(pen, ch):
    """单字母笔绘, 按消耗走: 起 → 消 → 换行 → 收"""
    W, H = 400, 700
    bx, by = 100, 0  # 字框起点

    # ── 封闭型 ──
    if ch == "a":
        pen.moveTo((bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+350))
        pen.lineTo((bx+350, by+700))
        pen.lineTo((bx+50, by+700))
        pen.lineTo((bx+50, by+350))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.closePath()
        pen.moveTo((bx+200, by+400))
        pen.lineTo((bx+350, by+400))
        pen.lineTo((bx+350, by+550))
        pen.lineTo((bx+200, by+550))
        pen.closePath()
    elif ch == "b":
        pen.moveTo((bx+50, by)); pen.lineTo((bx+50, by+700)); pen.closePath()
        pen.moveTo((bx+50, by+350))
        pen.qCurveTo((bx+350, by+350), (bx+350, by+525))
        pen.qCurveTo((bx+350, by+700), (bx+50, by+700))
        pen.closePath()
    elif ch == "d":
        pen.moveTo((bx+350, by)); pen.lineTo((bx+350, by+700)); pen.closePath()
        pen.moveTo((bx+350, by+350))
        pen.qCurveTo((bx+50, by+350), (bx+50, by+525))
        pen.qCurveTo((bx+50, by+700), (bx+350, by+700))
        pen.closePath()
    elif ch == "o":
        pen.moveTo((bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+450))
        pen.qCurveTo((bx+350, by+700), (bx+200, by+700))
        pen.qCurveTo((bx+50, by+700), (bx+50, by+450))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.closePath()
    elif ch == "g":
        pen.moveTo((bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+450))
        pen.qCurveTo((bx+350, by+600), (bx+200, by+600))
        pen.qCurveTo((bx+50, by+600), (bx+50, by+450))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.closePath()
        pen.moveTo((bx+250, by+600))
        pen.lineTo((bx+250, by+750))
        pen.lineTo((bx+100, by+750))
        pen.closePath()
    elif ch == "p":
        pen.moveTo((bx+50, by+350))
        pen.qCurveTo((bx+350, by+350), (bx+350, by+525))
        pen.qCurveTo((bx+350, by+700), (bx+50, by+700))
        pen.closePath()
    elif ch == "q":
        pen.moveTo((bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+450))
        pen.qCurveTo((bx+350, by+600), (bx+200, by+600))
        pen.qCurveTo((bx+50, by+600), (bx+50, by+450))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.closePath()
        pen.moveTo((bx+300, by+600))
        pen.lineTo((bx+300, by+750))
        pen.lineTo((bx+380, by+750))
        pen.closePath()
    # ── 开放竖型 ──
    elif ch == "h":
        pen.moveTo((bx+50, by)); pen.lineTo((bx+50, by+350))
        pen.qCurveTo((bx+50, by+525), (bx+200, by+525))
        pen.qCurveTo((bx+350, by+525), (bx+350, by+700))
        pen.lineTo((bx+350, by)); pen.closePath()
    elif ch == "n":
        pen.moveTo((bx+50, by+700)); pen.lineTo((bx+50, by+350))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+350))
        pen.lineTo((bx+350, by+700)); pen.closePath()
    elif ch == "m":
        pen.moveTo((bx+50, by+700)); pen.lineTo((bx+50, by+350))
        pen.qCurveTo((bx+50, by+200), (bx+150, by+200))
        pen.qCurveTo((bx+250, by+200), (bx+250, by+350))
        pen.lineTo((bx+250, by+400))
        pen.qCurveTo((bx+250, by+550), (bx+350, by+550))
        pen.qCurveTo((bx+350, by+700), (bx+350, by+350))
        pen.lineTo((bx+350, by+700)); pen.closePath()
    elif ch == "u":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+50, by+550))
        pen.qCurveTo((bx+50, by+700), (bx+200, by+700))
        pen.qCurveTo((bx+350, by+700), (bx+350, by+550))
        pen.lineTo((bx+350, by+200)); pen.closePath()
    elif ch == "l":
        pen.moveTo((bx+200, by)); pen.lineTo((bx+200, by+700))
        pen.lineTo((bx+50, by+700)); pen.closePath()
    elif ch == "i":
        pen.moveTo((bx+200, by+200)); pen.lineTo((bx+200, by+700)); pen.closePath()
        pen.moveTo((bx+180, by+150)); pen.lineTo((bx+220, by+150))
        pen.lineTo((bx+220, by+180)); pen.lineTo((bx+180, by+180)); pen.closePath()
    # ── 封 + 竖组合 ──
    elif ch == "c":
        pen.moveTo((bx+200, by+350))
        pen.qCurveTo((bx+350, by+350), (bx+350, by+450))
        pen.lineTo((bx+50, by+450))
        pen.qCurveTo((bx+50, by+200), (bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+300))
        pen.lineTo((bx+50, by+300))
        pen.qCurveTo((bx+50, by+350), (bx+200, by+350))
        pen.closePath()
    elif ch == "e":
        pen.moveTo((bx+50, by+350)); pen.lineTo((bx+350, by+350))
        pen.lineTo((bx+350, by+500))
        pen.qCurveTo((bx+350, by+700), (bx+200, by+700))
        pen.qCurveTo((bx+50, by+700), (bx+50, by+500))
        pen.lineTo((bx+50, by+350)); pen.closePath()
        pen.moveTo((bx+50, by+400)); pen.lineTo((bx+350, by+400)); pen.closePath()
    elif ch == "s":
        pen.moveTo((bx+200, by+200))
        pen.qCurveTo((bx+350, by+200), (bx+350, by+300))
        pen.lineTo((bx+50, by+500))
        pen.qCurveTo((bx+50, by+700), (bx+200, by+700))
        pen.closePath()
    elif ch == "t":
        pen.moveTo((bx+200, by)); pen.lineTo((bx+200, by+700)); pen.closePath()
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+350, by+200)); pen.closePath()
    elif ch == "f":
        pen.moveTo((bx+200, by+200)); pen.lineTo((bx+200, by+750))
        pen.qCurveTo((bx+200, by+800), (bx+100, by+800)); pen.closePath()
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+350, by+200)); pen.closePath()
        pen.moveTo((bx+50, by+400)); pen.lineTo((bx+300, by+400)); pen.closePath()
    elif ch == "r":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+50, by+700)); pen.closePath()
        pen.moveTo((bx+50, by+250))
        pen.qCurveTo((bx+200, by+200), (bx+200, by+350)); pen.closePath()
    elif ch == "y":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+50, by+350))
        pen.lineTo((bx+200, by+500)); pen.closePath()
        pen.moveTo((bx+350, by+200)); pen.lineTo((bx+350, by+350))
        pen.lineTo((bx+200, by+500)); pen.closePath()
        pen.moveTo((bx+200, by+500)); pen.lineTo((bx+200, by+750)); pen.closePath()
    elif ch == "w":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+120, by+700))
        pen.lineTo((bx+200, by+350))
        pen.lineTo((bx+280, by+700)); pen.lineTo((bx+350, by+200)); pen.closePath()
    elif ch == "k":
        pen.moveTo((bx+50, by)); pen.lineTo((bx+50, by+700)); pen.closePath()
        pen.moveTo((bx+300, by)); pen.lineTo((bx+50, by+350)); pen.closePath()
        pen.moveTo((bx+300, by+700)); pen.lineTo((bx+50, by+350)); pen.closePath()
    elif ch == "x":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+350, by+700)); pen.closePath()
        pen.moveTo((bx+350, by+200)); pen.lineTo((bx+50, by+700)); pen.closePath()
    elif ch == "v":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+200, by+700)); pen.closePath()
        pen.lineTo((bx+350, by+200)); pen.closePath()
    elif ch == "z":
        pen.moveTo((bx+50, by+200)); pen.lineTo((bx+350, by+200))
        pen.lineTo((bx+50, by+700)); pen.lineTo((bx+350, by+700)); pen.closePath()
    elif ch == "j":
        pen.moveTo((bx+250, by+200)); pen.lineTo((bx+250, by+600))
        pen.qCurveTo((bx+250, by+750), (bx+150, by+750)); pen.closePath()
        pen.moveTo((bx+230, by+150)); pen.lineTo((bx+270, by+150))
        pen.lineTo((bx+270, by+180)); pen.lineTo((bx+230, by+180)); pen.closePath()
    elif ch == "A":
        pen.moveTo((bx+200, by)); pen.lineTo((bx+50, by+700)); pen.closePath()
        pen.moveTo((bx+200, by)); pen.lineTo((bx+350, by+700)); pen.closePath()
        pen.moveTo((bx+125, by+450)); pen.lineTo((bx+275, by+450)); pen.closePath()
    elif ch in "BCDFGHIJKLMNOPQRSTUVWXYZ":
        # 其余大写字: 简化方框
        pen.moveTo((bx+50, by)); pen.lineTo((bx+350, by))
        pen.lineTo((bx+350, by+700)); pen.lineTo((bx+50, by+700)); pen.closePath()
    else:
        # 未定义: 空白字框
        pen.moveTo((bx+50, by)); pen.lineTo((bx+350, by))
        pen.lineTo((bx+350, by+700)); pen.lineTo((bx+50, by+700)); pen.closePath()

# ============================================================
# 融合模式: 消耗的编排方式
#   LR = 左右  (接笔消耗)
#   TB = 上下  (堆叠消耗)
#   IO = 里外  (包围消耗)
#   SS = 半包围 (包围留口)
# ============================================================

def fuse_lr(left_pieces, right_pieces, pen):
    """左右融合: 右件偏移至左件右端, 笔划共享接头"""
    for poly in left_pieces:
        pts = list(poly)
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()
    for poly in right_pieces:
        pts = [(p[0]+280, p[1]) for p in poly]  # 右偏, 共享10px
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()

def fuse_tb(top_pieces, bottom_pieces, pen):
    """上下融合"""
    for poly in top_pieces:
        pts = list(poly)
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()
    for poly in bottom_pieces:
        pts = [(p[0], p[1]+350) for p in poly]
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()

def fuse_io(outer_pieces, inner_pieces, pen):
    """里外融合: 内件嵌在外件内"""
    for poly in outer_pieces:
        pts = list(poly)
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()
    for poly in inner_pieces:
        center_x, center_y = 200, 350
        pts = [(center_x + (p[0]-200)*0.6, center_y + (p[1]-350)*0.6) for p in poly]
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()

def fuse_ss(host_pieces, guest_pieces, pen):
    """半包围: 主件留口, 客件嵌入开口处"""
    for poly in host_pieces:
        pts = list(poly)
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()
    for poly in guest_pieces:
        pts = [(p[0]*0.7+100, p[1]*0.5+200) for p in poly]
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()

FUSE_MODES = {
    "LR": fuse_lr, "TB": fuse_tb,
    "IO": fuse_io, "SS": fuse_ss,
}

def auto_fuse(name):
    """自动融合: 拆字为两部分, 选模式, 调用融合"""
    if len(name) < 2:
        return None, None, None
    # 识别声韵拆分点
    left, right = name[0], name[1:]
    if left not in "abcdefghijklmnopqrstuvwxyz":
        left, right = name[:2], name[2:]
        if left not in "zh ch sh".split() and len(name) > 2:
            left, right = name[0], name[1:]
    lp = TTGlyphPen(None)
    pen_letter(lp, left)
    rp = TTGlyphPen(None)
    pen_letter(rp, right)
    return lp.glyph(), rp.glyph()

# ============================================================
# 拼音 → 融合条目表 (反向索引)
# ============================================================

PINYIN_INDEX = {
    # 手工融合 30
    "an": ["an","an","LR"], "en": ["en","en","LR"],
    "in": ["in","in","LR"], "un": ["un","un","LR"],
    "vn": ["vn","ün","LR"],
    "ai": ["ai","ai","LR"], "ei": ["ei","ei","LR"],
    "ao": ["ao","ao","LR"], "ou": ["ou","ou","SS"],
    "ie": ["ie","ie","TB"], "uo": ["uo","uo","TB"],
    "io": ["io","io","TB"],
    "ue": ["ue","üe","LR"], "ui": ["ui","ui","LR"],
    "iu": ["iu","iu","LR"],
    "eo": ["eo","eo","IO"], "oe": ["oe","oe","IO"], "eu": ["eu","eu","IO"],
    "ang":["ang","ang","LR"], "eng":["eng","eng","LR"],
    "ing":["ing","ing","LR"], "ong":["ong","ong","LR"],
    "ian":["ian","ian","LR"], "uan":["uan","uan","TB"],
    "iao":["iao","iao","TB"], "uai":["uai","uai","TB"],
    "iong":["iong","iong","IO"],
    "ch":["ch","ch","LR"], "sh":["sh","sh","LR"], "zh":["zh","zh","LR"],
    # 扩展自动融合 ~96
    **{k:[k,k,"LR"] for k in [
        "aa","oo","ii","uu","ee","vv",  # 粤语
        "th","gh","fh","ph","ng","nk","nt","nd",  # 辅音
        "ed","ad","uk","ok","ek","ep",  # 后缀
        "ar","er","ir","or","ur","yr",  # r-变音
        "pr","tr","kr","br","dr","sr","fr","gr",  # r-丛
        "fl","gl","pl","bl","cl",  # l-丛
        "sk","sp","st","sm","sn","sw","tw","dw","kw","qu",  # s-丛/w
        "am","im","om","em","on",
        "ae","ua","ia","ve","uei","iou",
        "iang","uang","ueng",
        "ia","ua",
        "id","od","ud","eg","ig","ug","ag",
        "iz","oz","uz",
        "ivr","evr",
        "nch","nsh","nth",
        "str","spr","spl",
        "ld","lt","lk","lf","lm",
        "pt","ct","ft","xt",
        "zz","xx","kk",
    ]},
}

# ============================================================
# 字体生成主函数
# ============================================================

def build_font(out="mqf_radix_fused.ttf"):
    glyph_order = [".notdef", "space"]
    cmap_own = {}
    cmap_unicode = {}
    cmap_symbol = {}
    recipe = []
    gsub_rules = []  # ligature substitution rules

    # ── 定义自有编码范围 ──
    own_codes = {}
    # 声母一群: C01-C0B
    for i, ch in enumerate("bmpfdtnlgkh"):
        own_codes[ch] = 0xC01 + i
    # 声母二群: C10-C19
    for i, ch in enumerate("jqxzcsrywv"):
        own_codes[ch] = 0xC10 + i
    # 韵母: C20-C25
    for i, v in enumerate("aeiouv"):
        own_codes[v] = 0xC20 + i
    # 手工融合: C40-C5D
    ht_base = 0xC40
    ht_list = [k for k in PINYIN_INDEX if k in [
        "an","en","in","un","vn","ai","ei","ao","ou",
        "ie","uo","io","ue","ui","iu","eo","oe","eu",
        "ang","eng","ing","ong","ian","uan","iao","uai","iong",
        "ch","sh","zh"
    ]]
    ht_codes = {}
    for i, name in enumerate(ht_list):
        ht_codes[name] = ht_base + i
    # 自动融合: C80-CFE
    auto_names = [k for k in PINYIN_INDEX if k not in ht_list]
    auto_base = 0xC80
    auto_codes = {}
    for i, name in enumerate(auto_names):
        auto_codes[name] = auto_base + i

    def add_glyph(gname):
        if gname not in glyph_order:
            glyph_order.append(gname)
        return gname

    # ── 绘制 notdef 和 space ──
    fb0 = FontBuilder(1000, isTTF=True)

    # ── 基础字母字形 ──
    letter_glyphs = {}
    for ch in "abcdefghijklmnopqrstuvwxyz":
        pen = TTGlyphPen(None)
        pen_letter(pen, ch)
        letter_glyphs[ch] = pen.glyph()
        gname = add_glyph(ch)
        cmap_own[own_codes.get(ch, 0xC20)] = gname
        cmap_unicode[ord(ch)] = gname

    # ── Base glyphs (uppercase) for Latin fallback ──
    for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
        pen = TTGlyphPen(None)
        pen_letter(pen, ch)
        letter_glyphs[ch] = pen.glyph()
        gname = add_glyph(ch)
        cmap_unicode[ord(ch)] = gname

    # ── 手工融合字形绘制 ──
    def draw_ligature(name):
        """绘制一个融合字形的笔划"""
        mode = PINYIN_INDEX[name][2]
        pen = TTGlyphPen(None)
        if len(name) == 1:
            pen_letter(pen, name)
            return pen.glyph()
        # 拆分为首末件
        left_c, right_c = name[0], name[1]
        lp = TTGlyphPen(None)
        pen_letter(lp, left_c)
        rp = TTGlyphPen(None)
        pen_letter(rp, right_c)
        fuse_fn = FUSE_MODES.get(mode, fuse_lr)
        fuse_fn(lp.glyph().drawPoints(None) if False else [], rp.glyph().drawPoints(None) if False else [], pen)
        return pen.glyph()

    # 简化: 直接用 pen 绘制, 不通过 pen 的中介
    def draw_fused(name):
        mode = PINYIN_INDEX[name][2]
        pen = TTGlyphPen(None)
        if len(name) < 2:
            pen_letter(pen, name)
            return pen.glyph()
        left_c = name[0]
        right_c = name[1]

        # 绘制左件 (原位)
        pen_letter(pen, left_c)

        # 绘制右件 (偏移到右边, 实现 LR 融合)
        savedPen = pen
        if mode == "LR":
            # 创建第二个 pen 绘制右件, 然后我们把两 pen 的内容合并 —
            # 简化方案: 直接用 offset 方式在同一个 pen 中偏移画右件
            rp = TTGlyphPen(None)
            pen_letter(rp, right_c)
            # 这里不实现合并, 直接画左件 + (偏移)右件
            # 真正的字体设计会精细调坐标, 这里用偏移作为原型
        return pen.glyph()

    # ── 简化方案: 所有融合字都按左右结构画 ──
    def make_lig_pen(name):
        """生成融合字的 pen 字形"""
        pen = TTGlyphPen(None)
        mode = PINYIN_INDEX[name][2] if name in PINYIN_INDEX else "LR"

        if len(name) == 1:
            pen_letter(pen, name)
            return pen.glyph()

        # 双字母融合: 左右小幅重叠
        chars = list(name)
        for idx, ch in enumerate(chars):
            offset_x = idx * 280  # 每个字母偏移 280 units
            # 使用 pen_letter 的逻辑, 但需要 translate
            # 简化: 直接用 pen.moveTo/lineTo 并以 offx 位移
            _draw_letter_at(pen, ch, ox=offset_x)

        return pen.glyph()

    def _draw_letter_at(pen, ch, ox=0, oy=0):
        """在指定偏移位置画一个字母 (简化版)"""
        # 直接产生基础笔画并移动
        W, H = 400, 700
        bx, by = 100 + ox, 0 + oy

        if ch == "a":
            pen.moveTo((bx+W//2, by+H//3))
            pen.qCurveTo((bx+W, by+H//3), (bx+W, by+H//2))
            pen.lineTo((bx+W, by+H))
            pen.lineTo((bx, by+H))
            pen.lineTo((bx, by+H//2))
            pen.qCurveTo((bx, by+H//3), (bx+W//2, by+H//3))
            pen.closePath()
        elif ch == "o":
            pen.moveTo((bx+W//2, by+H//3))
            pen.qCurveTo((bx+W, by+H//3), (bx+W, by+2*H//3))
            pen.qCurveTo((bx+W, by+H), (bx+W//2, by+H))
            pen.qCurveTo((bx, by+H), (bx, by+2*H//3))
            pen.qCurveTo((bx, by+H//3), (bx+W//2, by+H//3))
            pen.closePath()
        elif ch == "e":
            pen.moveTo((bx, by+H//2))
            pen.lineTo((bx+W, by+H//2))
            pen.lineTo((bx+W, by+5*H//6))
            pen.qCurveTo((bx+W, by+H), (bx+W//2, by+H))
            pen.qCurveTo((bx, by+H), (bx, by+5*H//6))
            pen.lineTo((bx, by+H//3))
            pen.qCurveTo((bx, by+H//4), (bx+W//2, by+H//4))
            pen.qCurveTo((bx+W, by+H//4), (bx+W, by+H//3))
            pen.closePath()
        elif ch == "n":
            pen.moveTo((bx, by+H)); pen.lineTo((bx, by+H//3))
            pen.qCurveTo((bx, by+H//4), (bx+W//2, by+H//4))
            pen.qCurveTo((bx+W, by+H//4), (bx+W, by+H//3))
            pen.lineTo((bx+W, by+H)); pen.closePath()
        elif ch == "u":
            pen.moveTo((bx, by+H//3)); pen.lineTo((bx, by+5*H//6))
            pen.qCurveTo((bx, by+H), (bx+W//2, by+H))
            pen.qCurveTo((bx+W, by+H), (bx+W, by+5*H//6))
            pen.lineTo((bx+W, by+H//3)); pen.closePath()
        elif ch == "i":
            pen.moveTo((bx+W//2, by+H//4)); pen.lineTo((bx+W//2, by+H)); pen.closePath()
            pen.moveTo((bx+W//2-10, by+H//6)); pen.lineTo((bx+W//2+10, by+H//6))
            pen.lineTo((bx+W//2+10, by+H//4-10)); pen.lineTo((bx+W//2-10, by+H//4-10))
            pen.closePath()
        elif ch in "bdtqp":
            pen.moveTo((bx, by)); pen.lineTo((bx, by+H)); pen.closePath()
            if ch in "bd":
                pen.moveTo((bx, by+H//3))
                pen.qCurveTo((bx+W, by+H//3), (bx+W, by+2*H//3))
                pen.qCurveTo((bx+W, by+H), (bx, by+H))
                pen.closePath()
            else:
                pen.moveTo((bx+W, by+H//3))
                pen.qCurveTo((bx+W-H//10, by+H//2), (bx, by+H//2))
                pen.closePath()
        elif ch in "hl":
            pen.moveTo((bx, by)); pen.lineTo((bx, by+H)); pen.closePath()
            if ch == "h":
                pen.moveTo((bx, by+H//2))
                pen.qCurveTo((bx, by+2*H//3), (bx+W//2, by+2*H//3))
                pen.qCurveTo((bx+W, by+2*H//3), (bx+W, by+H))
                pen.closePath()
            else:
                pen.moveTo((bx, by+H)); pen.lineTo((bx+W, by+H)); pen.closePath()
        elif ch == "m":
            pen.moveTo((bx, by+H)); pen.lineTo((bx, by+H//3))
            pen.qCurveTo((bx, by+H//4), (bx+W//3, by+H//4))
            pen.qCurveTo((bx+W//2, by+H//4), (bx+W//2, by+H//3))
            pen.lineTo((bx+W//2, by+H//2))
            pen.qCurveTo((bx+W//2, by+3*H//4), (bx+5*W//6, by+3*H//4))
            pen.qCurveTo((bx+W, by+3*H//4), (bx+W, by+H//3))
            pen.lineTo((bx+W, by+H)); pen.closePath()
        elif ch == "r":
            pen.moveTo((bx, by+H//3)); pen.lineTo((bx, by+H)); pen.closePath()
            pen.moveTo((bx, by+H//3))
            pen.qCurveTo((bx+W//2, by+H//3), (bx+W//2, by+H//2)); pen.closePath()
        elif ch == "s":
            pen.moveTo((bx+W//2, by+H//4))
            pen.qCurveTo((bx+W, by+H//4), (bx+W, by+H//3))
            pen.lineTo((bx, by+2*H//3))
            pen.qCurveTo((bx, by+H), (bx+W//2, by+H)); pen.closePath()
        elif ch == "c":
            pen.moveTo((bx+W//2, by+H//3))
            pen.qCurveTo((bx+W, by+H//3), (bx+W, by+H//2))
            pen.lineTo((bx, by+H//2))
            pen.qCurveTo((bx, by+H//3), (bx+W//2, by+H//3))
            pen.closePath()
        elif ch == "f":
            pen.moveTo((bx+W//2, by+H//4)); pen.lineTo((bx+W//2, by+5*H//6))
            pen.qCurveTo((bx+W//2, by+H), (bx+W//3, by+H)); pen.closePath()
            pen.moveTo((bx, by+H//4)); pen.lineTo((bx+W, by+H//4)); pen.closePath()
            pen.moveTo((bx, by+H//2)); pen.lineTo((bx+3*W//4, by+H//2)); pen.closePath()
        elif ch == "g":
            pen.moveTo((bx+W//2, by+H//3))
            pen.qCurveTo((bx+W, by+H//3), (bx+W, by+2*H//3))
            pen.qCurveTo((bx+W, by+H), (bx+W//2, by+H))
            pen.qCurveTo((bx, by+H), (bx, by+2*H//3))
            pen.qCurveTo((bx, by+H//3), (bx+W//2, by+H//3))
            pen.closePath()
            pen.moveTo((bx+W//2, by+H)); pen.lineTo((bx+W//2, by+H+H//4))
            pen.lineTo((bx+W//4, by+H+H//4)); pen.closePath()
        elif ch == "j":
            pen.moveTo((bx+W//2, by+H//4)); pen.lineTo((bx+W//2, by+5*H//6))
            pen.qCurveTo((bx+W//2, by+H+H//8), (bx+W//3, by+H+H//8)); pen.closePath()
            pen.moveTo((bx+W//2-10, by+H//6)); pen.lineTo((bx+W//2+10, by+H//6))
            pen.lineTo((bx+W//2+10, by+H//4-10)); pen.lineTo((bx+W//2-10, by+H//4-10))
            pen.closePath()
        elif ch == "k":
            pen.moveTo((bx, by)); pen.lineTo((bx, by+H)); pen.closePath()
            pen.moveTo((bx+W, by)); pen.lineTo((bx, by+H//2)); pen.closePath()
            pen.moveTo((bx+W, by+H)); pen.lineTo((bx, by+H//2)); pen.closePath()
        elif ch == "v":
            pen.moveTo((bx, by+H//4)); pen.lineTo((bx+W//2, by+H)); pen.closePath()
            pen.lineTo((bx+W, by+H//4)); pen.closePath()
        elif ch == "w":
            pen.moveTo((bx, by+H//4)); pen.lineTo((bx+W//4, by+H))
            pen.lineTo((bx+W//2, by+H//2))
            pen.lineTo((bx+3*W//4, by+H)); pen.lineTo((bx+W, by+H//4)); pen.closePath()
        elif ch == "x":
            pen.moveTo((bx, by+H//3)); pen.lineTo((bx+W, by+2*H//3)); pen.closePath()
            pen.moveTo((bx+W, by+H//3)); pen.lineTo((bx, by+2*H//3)); pen.closePath()
        elif ch == "y":
            pen.moveTo((bx, by+H//3)); pen.lineTo((bx, by+H//2))
            pen.lineTo((bx+W//2, by+3*H//4)); pen.closePath()
            pen.moveTo((bx+W, by+H//3)); pen.lineTo((bx+W, by+H//2))
            pen.lineTo((bx+W//2, by+3*H//4)); pen.closePath()
            pen.moveTo((bx+W//2, by+3*H//4)); pen.lineTo((bx+W//2, by+H)); pen.closePath()
        elif ch == "z":
            pen.moveTo((bx, by+H//3)); pen.lineTo((bx+W, by+H//3))
            pen.lineTo((bx, by+2*H//3)); pen.lineTo((bx+W, by+2*H//3)); pen.closePath()
        elif ch == "l":
            pen.moveTo((bx+W//2, by)); pen.lineTo((bx+W//2, by+H))
            pen.lineTo((bx, by+H)); pen.closePath()
        else:
            pen.moveTo((bx, by)); pen.lineTo((bx+W, by))
            pen.lineTo((bx+W, by+H)); pen.lineTo((bx, by+H)); pen.closePath()

    # ── 生成手工融合字形 ──
    lig_glyphs = {}
    for name in ht_list:
        pen = TTGlyphPen(None)
        _ = make_lig_pen(name)  # generate
        # Let's use the proper approach:
        glyph = make_lig_pen(name)
        lig_glyphs[name] = glyph
        gname = f"lig.{name}"
        add_glyph(gname)
        code = ht_codes[name]
        cmap_own[code] = gname
        cmap_unicode[0xFA00 + len([r for r in recipe if not r[0].startswith("lig.")])] = gname
        recipe.append((name, "fused", PINYIN_INDEX[name][2], f"C{code:03X}"))
        # 记录 GSUB 规则: 输入各字母 → 输出融合字
        input_glyphs = [ch for ch in name if ch in letter_glyphs]
        if len(input_glyphs) >= 2:
            gsub_rules.append((input_glyphs, gname))

    # ── 生成自动融合字形 ──
    for name in auto_names:
        glyph = make_lig_pen(name)
        lig_glyphs[name] = glyph
        gname = f"lig.{name}"
        add_glyph(gname)
        code = auto_codes[name]
        cmap_own[code] = gname
        unicode_slot = 0xFA20 + len([r for r in recipe if not r[0].startswith("lig.")])
        cmap_unicode[unicode_slot] = gname
        recipe.append((name, "auto", PINYIN_INDEX[name][2], f"C{code:03X}"))
        input_glyphs = [ch for ch in name if ch in letter_glyphs]
        if len(input_glyphs) >= 2:
            gsub_rules.append((input_glyphs, gname))

    # ── GSUB: 连字规则嵌入 ──
    # 使用 feaLib 生成 OpenType Feature 并编译进字体
    # 比手动拼接 otTables 更可靠
    def build_gsub(font):
        """用 feaLib 把连字规则编译进字体 GSUB 表"""
        try:
            from fontTools.feaLib.builder import addOpenTypeFeatures
        except ImportError:
            return None

        # 生成 feature 文本: sub a n by lig.an;
        lines = ["feature liga {"]
        for input_glyphs, output_glyph in gsub_rules:
            if len(input_glyphs) >= 2:
                seq = " ".join(input_glyphs)
                lines.append(f"    sub {seq} by {output_glyph};")
        lines.append("} liga;")
        fea_code = "\n".join(lines)

        # 写入临时 .fea 文件并编译
        import tempfile, os as _os
        fea_path = _os.path.join(tempfile.gettempdir(), "mqf_ligatures.fea")
        with open(fea_path, "w") as f:
            f.write(fea_code)

        addOpenTypeFeatures(font, fea_path)
        _os.unlink(fea_path)  # 清理临时文件
        return font.get("GSUB")

    # ── 构建字体骨架 ──
    upm = 1000
    advance = 600

    fb = FontBuilder(upm, isTTF=True)
    fb.setupGlyphOrder(glyph_order)

    # notdef
    pen_nd = TTGlyphPen(None)
    pen_nd.moveTo((100,0)); pen_nd.lineTo((500,0))
    pen_nd.lineTo((500,700)); pen_nd.lineTo((100,700)); pen_nd.closePath()
    pen_nd.moveTo((140,40)); pen_nd.lineTo((140,660))
    pen_nd.lineTo((360,660)); pen_nd.lineTo((360,40)); pen_nd.closePath()

    # space
    pen_sp = TTGlyphPen(None)

    glyf = {".notdef": pen_nd.glyph(), "space": pen_sp.glyph()}
    metrics = {".notdef": (advance, 0), "space": (advance//2, 0)}

    # 基础字母
    for ch, g in letter_glyphs.items():
        glyf[ch] = g
        metrics[ch] = (advance, 0)

    # 融合字
    for name, g in lig_glyphs.items():
        gname = f"lig.{name}"
        glyf[gname] = g
        metrics[gname] = (advance, 0)

    fb.setupGlyf(glyf)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)

    # 主 cmap (Unicode)
    fb.setupCharacterMap(cmap_unicode)

    fb.setupNameTable({
        "familyName": "MQF Container Radix Fused v3",
        "styleName": "FusedGSUB",
        "psName": "MQF-ContainerRadix-Fused-v3",
        "manufacturer": "囝囝共生 · 刘楚恬和我",
        "designer": "lolict + 刘楚恬",
        "description": f"满全法容器进制融合 v3: {len(ht_list)} 手工 + {len(auto_names)} 自动 = {len(ht_list)+len(auto_names)} 组, 共 {len(gsub_rules)} 条 GSUB 连字规则",
        "vendorURL": "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private",
        "version": "0.3.0",
    })
    fb.setupOS2(sTypoAscender=800, sTypoDescent=-200, usWinAscent=800, usWinDescent=200)
    fb.setupPost()

    font = fb.font

    # ── 嵌入自有编码 cmap (Symbol) ──
    sub_sym = CmapSubtable.newSubtable(12)
    sub_sym.platformID = 3
    sub_sym.platEncID = 0  # Symbol
    sub_sym.format = 12
    sub_sym.reserved = 0
    sub_sym.length = 0
    sub_sym.language = 0
    sub_sym.cmap = cmap_own
    font["cmap"].tables.insert(0, sub_sym)

    # ── 嵌入 GSUB 连字表 ──
    try:
        build_gsub(font)
        gsub_ok = font.get("GSUB") is not None
    except Exception as e:
        gsub_ok = False

    # ── 配方写进 name table ──
    for i, (name, comp, mode, code) in enumerate(recipe):
        rec = f"{name}={comp}@{code}[{mode}]"
        try:
            font["name"].setName(rec, 256+i, 3, 1, 0x0409)
        except:
            pass

    font.save(out)
    font.close()

    size = os.path.getsize(out)
    return {
        "file": out,
        "size": size,
        "total_glyphs": len(glyph_order),
        "hand_tuned": len(ht_list),
        "auto_fused": len(auto_names),
        "gsub_rules": len(gsub_rules),
        "gsub_embedded": gsub_ok,
        "recipe": recipe[:5],
    }

# ============================================================
# CLI
# ============================================================

if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "mqf_radix_fused.ttf"
    r = build_font(out)
    print(f"字体: {r['file']} ({r['size']} bytes)")
    print(f"总字形: {r['total_glyphs']}")
    print(f"  手工融合: {r['hand_tuned']}")
    print(f"  自动融合: {r['auto_fused']}")
    print(f"  GSUB 连字规则: {r['gsub_rules']} 条")
    print(f"  GSUB 嵌入成功: {r['gsub_embedded']}")
    print(f"\n容器编码:")
    print(f"  声母一群: C01-C0B (袋桶罐盆盂盎)")
    print(f"  声母二群: C10-C19")
    print(f"  韵母:     C20-C25 (钵盏盒杯壶瓶)")
    print(f"  手工融合: C40-C5D ({r['hand_tuned']}组)")
    print(f"  自动融合: C80-CFE ({r['auto_fused']}组)")
    print(f"\n配方示例:")
    for name, comp, mode, code in r['recipe']:
        print(f"  {name:8s} → {comp:6s} @ {code} [{mode}]")
    print(f"\n后备 Unicode: FA00..")
