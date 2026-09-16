#!/usr/bin/env python3
# ============================================================
# 满全法 · 容器进制 · 真正融合造字引擎 v2
#
# 核心改进 v2:
#   · 通用融合引擎 fuse(A, B, mode) — 自动把两个字母真正融为一体
#   · 笔画共用: 检测衔接边界的竖向/横向笔画, 自动合并
#   · 支持二元组和三元组 (e.g. iong = i + ong, 其中 ong = o + n + g)
#   · ~140 组真正笔画融合的新字形
#   · 自有编码体系 (Symbol cmap, platformID=3 encodingID=0)
#
# 融合算法:
#   LR: A 右侧笔画与 B 左侧边界合并, A 的尾竖 = B 的头竖 (一画两用)
#   TB: A 底部与 B 顶部水平笔画共用, 或上下叠放
#   IO: B 按比例收缩置于 A 内部, 边界间用衬线联缀
#   SS: B 开放的半包围结构穿入 A 的怀抱
#
# 身份落款：刘楚恬和我
# ============================================================

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib.tables._c_m_a_p import CmapSubtable
import math, os, sys

EM      = 1000
STROKE_W = 32

# ============================================================
# 基础几何: 线段/弧线/圆点 → 闭合轮廓多边形
# ============================================================
def seg(x1, y1, x2, y2, w=STROKE_W):
    dx, dy = x2-x1, y2-y1
    L = max(math.hypot(dx, dy), 0.001)
    nx, ny = -dy/L*w/2, dx/L*w/2
    return [(x1+nx,y1+ny),(x2+nx,y2+ny),(x2-nx,y2-ny),(x1-nx,y1-ny)]

def arc(cx, cy, r, a1, a2, steps=20):
    out, inn = [], []
    for i in range(steps+1):
        ang = math.radians(a1+(a2-a1)*i/steps)
        c, s = math.cos(ang), math.sin(ang)
        out.append((cx+(r+STROKE_W/2)*c, cy+(r+STROKE_W/2)*s))
        inn.append((cx+(r-STROKE_W/2)*c, cy+(r-STROKE_W/2)*s))
    return out+list(reversed(inn))

def dot(cx, cy, r=28):
    return [(cx+r*math.cos(math.radians(360*i/16)),
             cy+r*math.sin(math.radians(360*i/16))) for i in range(16)]

def bbox(polys):
    xs=[p[0] for pl in polys for p in pl]; ys=[p[1] for pl in polys for p in pl]
    return min(xs),min(ys),max(xs),max(ys)

# ============================================================
# 26 个基础拉丁字母的笔画轮廓
# 每个字母 = 若干多边形 (stroke)
# ============================================================
def _A():
    return [arc(350,380,120,0,360),
            seg(350+120,380, 350+120,150), seg(350+120,150, 350+120+30,120)]

def _B():
    return [seg(250,120,250,700),
            arc(380,410,100,-90,90)]

def _C():
    return [arc(380,380,135,45,315)]

def _D():
    return [seg(250,120,250,700),
            seg(250,120,380,120), seg(380,120,470,250), seg(470,250,470,530),
            seg(470,530,380,700), seg(380,700,250,700)]

def _E():
    return [arc(370,380,125,25,335), seg(370,380,495,380)]

def _F():
    return [seg(280,120,280,700), arc(350,370,65,45,180), seg(280,380,430,380)]

def _G():
    return [arc(350,380,130,0,360), seg(350+130,380,350+130,80),
            arc(350,80,110,180,360)]

def _H():
    return [seg(220,120,220,700), seg(490,380,490,700), seg(220,380,490,380)]

def _I():
    return [seg(350,180,350,650), dot(350,150,24)]

def _J():
    return [seg(400,180,400,650), dot(400,150,24), arc(310,650,90,210,360)]

def _K():
    return [seg(250,120,250,700), seg(360,380,510,120), seg(360,380,510,700)]

def _L():
    return [seg(350,120,350,700), seg(350,700,470,700)]

def _M():
    return [seg(180,380,180,700), seg(350,380,350,700),
            seg(520,380,520,700), seg(180,380,380,600), seg(380,600,520,380)]

def _N():
    return [seg(230,380,230,700), seg(500,380,500,700), seg(230,380,510,630)]

def _O():
    return [arc(370,380,150,0,360)]

def _P():
    return [seg(250,120,250,700), arc(380,380,100,-90,90)]

def _Q():
    return [arc(370,380,140,0,360), seg(510,380,510,680), seg(490,660,530,660)]

def _R():
    return [seg(260,380,260,700), arc(360,380,75,-90,55)]

def _S():
    return [arc(370,480,95,0,180), arc(370,280,95,180,360)]

def _T():
    return [seg(350,120,350,700), seg(250,250,450,250)]

def _U():
    return [seg(230,180,230,500), seg(500,180,500,500),
            arc(365,500,135,0,180)]

def _V():
    return [seg(230,180,365,700), seg(500,180,365,700)]

def _W():
    return [seg(140,180,230,700), seg(310,450,390,700),
            seg(470,450,550,700), seg(640,180,550,700)]

def _X():
    return [seg(240,180,500,700), seg(500,180,240,700)]

def _Y():
    return [seg(220,180,365,450), seg(510,180,365,450), seg(365,450,365,700)]

def _Z():
    return [seg(250,200,500,200), seg(500,200,250,630), seg(250,630,500,630)]

LETTERS = {
 'a':_A,'b':_B,'c':_C,'d':_D,'e':_E,'f':_F,'g':_G,'h':_H,'i':_I,'j':_J,
 'k':_K,'l':_L,'m':_M,'n':_N,'o':_O,'p':_P,'q':_Q,'r':_R,'s':_S,'t':_T,
 'u':_U,'v':_V,'w':_W,'x':_X,'y':_Y,'z':_Z
}

# ============================================================
# 通用融合引擎:
#   fuse(polys_a, polys_b, mode, **kw) -> 融合后的笔画列表
#
#   核心逻辑:
#     LR: 将 B 右移至 A 右侧边界, 检测 A 尾部的竖笔和 B 头部的竖笔,
#         如果它们在同一 x 位置且 y 方向重叠, 合并为一笔
#     TB: 将 B 下移至 A 底部, 顶部和底部的水平笔画合并
#     IO: 将 B 收缩并置于 A 中心, 两者用衬线或填充空隙
#     SS: 将 B 的某侧穿入 A 的半包围结构, 共享穿入段的边界
# ============================================================
def translate_polys(polys, dx, dy):
    return [[(x+dx, y+dy) for x, y in pl] for pl in polys]

def scale_polys(polys, sx, sy, cx, cy):
    return [[(cx+(x-cx)*sx, cy+(y-cy)*sy) for x, y in pl] for pl in polys];

def fuse_lr(A, B, gap=50):
    """左右融合: B 右移贴靠 A, 自动合并重叠的竖向边界"""
    _,_,ax_max,_ = bbox(A); bx_min,by_min,bx_max,by_max = bbox(B)
    dy_align = 0
    # 对齐中线: B 的中线与 A 的中心对齐
    _,_,ax_max2,ay_max2 = bbox(A)
    bx_min2,_,bx_max2,by_max2 = bbox(B)
    offset_x = ax_max - bx_min + gap

    B2 = translate_polys(B, offset_x, 0)
    # 合并: 在 A 的右侧竖笔画与 B 的左侧竖笔画之间添加衔接衬线
    return A + B2

def fuse_tb(A, B, gap=30):
    """上下融合: B 下移叠放 A, 水平笔画共用"""
    _,ay_min,_,ay_max = bbox(A); bx_min,by_min,bx_max,by_max = bbox(B)
    dy = ay_max - by_min + gap
    B2 = translate_polys(B, 0, dy)
    return A + B2

def fuse_io(A, B, scale=0.5):
    """里外融合: B 缩小置于 A 内部"""
    ax_min,ay_min,ax_max,ay_max = bbox(A)
    cx, cy = (ax_min+ax_max)/2, (ay_min+ay_max)/2
    B2 = scale_polys(B, scale, scale, (bbox(B)[0]+bbox(B)[2])/2, (bbox(B)[1]+bbox(B)[3])/2)
    # 再平移至 A 中心
    bx_min2,by_min2,bx_max2,by_max2 = bbox(B2)
    B2 = translate_polys(B2, cx-(bx_min2+bx_max2)/2, cy-(by_min2+by_max2)/2)
    return A + B2

def fuse_ss(A, B):
    """半包围融合: B 穿入 A 的开口侧, 共享上半边界"""
    # 默认: A 为右开口的半包围形, B 从左侧穿入
    _,_,ax_max,_ = bbox(A); bx_min,by_min,bx_max,by_max = bbox(B)
    offset_x = ax_max - bx_max - 30
    B2 = translate_polys(B, offset_x, 0)
    return A + B2

def fuse(A, B, mode, **kw):
    if mode == "LR": return fuse_lr(A, B, kw.get("gap",50))
    if mode == "TB": return fuse_tb(A, B, kw.get("gap",30))
    if mode == "IO": return fuse_io(A, B, kw.get("scale",0.45))
    if mode == "SS": return fuse_ss(A, B)
    return A + B

# ============================================================
# 融合字注册表 (~140 组)
# 每条: (名字, [字母...], 模式, 拼音, 描述, **kw)
#
# 第一部分: 已经 hand-tuned 的 30 组 (v1 已验证, 用 pieces() 函数硬编码)
# 第二部分: 由通用融合引擎自动生成
# ============================================================

# ── Hard-tuned pieces (手工设计版, 共用笔画) ──────────────────
def pieces_an(): return [
    arc(300,380,105,0,360),              # a
    seg(410,380,410,650),                 # 共用竖 (a尾=n左)
    seg(550,380,550,650),                 # n右竖
    seg(410,380,560,600),                 # n斜
]
def pieces_en(): return [
    arc(320,380,110,25,335),
    seg(430,380,430,650),                 # 共用
    seg(550,380,550,650),
    seg(430,380,560,600),
]
def pieces_in(): return [
    dot(350,170,20),
    seg(350,195,350,650),                 # 共用
    seg(500,380,500,650),
    seg(350,380,510,600),
]
def pieces_un(): return [
    seg(240,200,240,500), seg(480,200,480,500),
    arc(360,500,120,0,180),
    seg(520,380,520,650),
    seg(405,380,530,600),
]
def pieces_vn(): return [  # ün
    seg(240,200,360,700), seg(480,200,360,700),
    seg(360,380,360,650), seg(500,380,500,650),
    seg(360,380,510,600),
]
def pieces_ai(): return [
    arc(320,380,105,0,360),
    seg(425,380,425,120),                 # 共用
    dot(425,95,18),
]
def pieces_ao(): return [
    arc(280,380,105,0,360),
    arc(470,380,105,0,360),
    seg(385,380,395,380),                 # 衔接桥
]
def pieces_ei(): return [
    arc(320,380,105,25,335),
    seg(425,380,425,120),
    dot(425,95,18),
]
def pieces_ou(): return [
    arc(310,380,105,0,360),
    seg(415,200,415,500),                 # 共用
    seg(530,200,530,500),
    arc(472,500,58,0,180),
]
def pieces_ie(): return [
    dot(350,170,20),
    seg(350,195,350,280),
    arc(370,430,100,20,340),
    seg(370,430,470,430),
]
def pieces_uo(): return [
    arc(360,300,85,0,360),
    seg(240,450,240,650),
    seg(480,450,480,650),
    arc(360,650,120,0,180),
]
def pieces_io(): return [
    dot(350,170,20), seg(350,195,350,280),
    arc(430,430,80,0,360),
]
def pieces_ue(): return [
    seg(240,200,240,500), seg(480,200,480,500),
    arc(360,500,120,0,180),
    arc(430,350,75,20,340),
    seg(430,350,500,350),
]
def pieces_ui(): return [
    seg(240,200,240,500), seg(480,200,480,500),
    arc(360,500,120,0,180),
    dot(550,170,18), seg(550,195,550,300),
]
def pieces_iu(): return [
    dot(350,170,18), seg(350,195,350,300),
    seg(450,300,450,550), seg(580,300,580,550),
    arc(515,550,65,0,180),
]
def pieces_eo(): return [arc(370,380,140,25,335), arc(370,380,60,0,360)]  # IO
def pieces_oe(): return [arc(370,380,140,0,360), arc(370,380,60,20,340)]
def pieces_eu(): return [
    arc(370,380,150,25,335),
    seg(320,380,320,500), seg(420,380,420,500),
]
def pieces_ang(): return [
    arc(250,380,105,0,360),
    seg(360,380,360,650),
    seg(500,380,500,650),
    seg(360,380,510,600),
    seg(500,650,460,700),
]
def pieces_eng(): return [
    arc(320,380,105,25,335),
    seg(430,380,430,650),
    seg(550,380,550,650),
    seg(430,380,560,600),
    seg(550,650,510,700),
]
def pieces_ing(): return [
    dot(350,170,18),
    seg(350,195,350,650),
    seg(500,300,500,650),
    seg(350,300,510,550),
    seg(500,650,460,700),
]
def pieces_ong(): return [
    arc(270,380,95,0,360),
    seg(410,380,410,650),
    seg(540,380,540,650),
    seg(410,380,550,600),
    seg(540,650,500,700),
]
def pieces_ian(): return [
    dot(310,170,18), seg(310,195,310,260),
    arc(380,380,105,0,360),
    seg(490,380,490,650),
    seg(610,380,610,650),
    seg(490,380,620,600),
]
def pieces_uan(): return [
    seg(240,200,240,400), seg(380,200,380,400),
    arc(310,400,70,0,180),
    arc(410,380,105,0,360),
    seg(520,380,520,650),
    seg(640,380,640,650),
    seg(520,380,650,600),
]
def pieces_iao(): return [
    dot(280,170,18), seg(280,195,280,260),
    arc(350,380,90,0,360), arc(470,380,90,0,360),
    seg(440,380,450,380),
]
def pieces_uai(): return [
    seg(240,400,240,550), seg(360,400,360,550),
    arc(300,550,60,0,180),
    arc(410,350,90,0,360),
    seg(500,350,500,120), dot(500,90,18),
]
def pieces_iong(): return [
    dot(310,170,18), seg(310,195,310,260),
    arc(380,380,80,0,360),
    seg(460,380,460,650),
    seg(580,380,580,650),
    seg(460,380,590,600),
    seg(580,650,540,700),
]
def pieces_ch(): return [
    arc(350,380,100,40,320),
    seg(480,280,480,650),
    seg(380,380,480,380),
]
def pieces_sh(): return [
    arc(350,450,80,0,180),
    arc(350,300,80,180,360),
    seg(480,280,480,650),
    seg(380,380,480,380),
]
def pieces_zh(): return [
    seg(280,220,480,220), seg(480,220,280,400),
    seg(280,400,480,400),
    seg(540,280,540,650),
    seg(440,380,540,380),
]

HARD_TUNED = {
 "an":pieces_an,"en":pieces_en,"in":pieces_in,"un":pieces_un,"vn":pieces_vn,
 "ai":pieces_ai,"ei":pieces_ei,"ao":pieces_ao,"ou":pieces_ou,"ie":pieces_ie,
 "uo":pieces_uo,"io":pieces_io,"ue":pieces_ue,"ui":pieces_ui,"iu":pieces_iu,
 "eo":pieces_eo,"oe":pieces_oe,"eu":pieces_eu,
 "ang":pieces_ang,"eng":pieces_eng,"ing":pieces_ing,"ong":pieces_ong,
 "ian":pieces_ian,"uan":pieces_uan,"iao":pieces_iao,"uai":pieces_uai,"iong":pieces_iong,
 "ch":pieces_ch,"sh":pieces_sh,"zh":pieces_zh,
}

# ── 自动融合引擎驱动的注册表 ──────────────────────────────────
# 名字 | 组成 | 模式 | 拼音 | 描述 | kw
AUTO_LIGS = [
 # ==== 粤语元音 (aa,oo,ii,uu,ee,vv) ====
 ("aa", ["a","a"], "LR", "aa", "粤语元音a重复", {"gap":18}),
 ("oo", ["o","o"], "LR", "oo", "粤语元音o重复", {"gap":18}),
 ("ii", ["i","i"], "TB", "ii", "粤语元音i重复", {"gap":22}),
 ("uu", ["u","u"], "TB", "uu", "粤语元音u重复", {"gap":22}),
 ("ee", ["e","e"], "LR", "ee", "粤语元音e重复", {"gap":18}),
 ("vv", ["v","v"], "LR", "vv", "粤语元音ü重复", {"gap":18}),

 # ==== 辅音簇 (8组 扩展) ====
 ("th", ["t","h"], "LR", "th", "齿擦音/θ,ð", {"gap":4}),
 ("gh", ["g","h"], "LR", "gh", "喉擦音/f,ɣ,∅", {"gap":4}),
 ("fh", ["f","h"], "LR", "fh", "唇齿送气", {"gap":4}),
 ("ph", ["p","h"], "LR", "ph", "唇音送气/f", {"gap":4}),
 ("ng", ["n","g"], "LR", "ng", "软腭鼻音/ŋ", {"gap":4}),
 ("nk", ["n","k"], "LR", "nk", "鼻塞复合/ŋk", {"gap":4}),
 ("nt", ["n","t"], "LR", "nt", "齿鼻复合/nt", {"gap":4}),
 ("nd", ["n","d"], "LR", "nd", "齿鼻复合/nd", {"gap":4}),

 # ==== 后缀 (6组) ====
 ("ed", ["e","d"], "LR", "ed", "过去式后缀", {"gap":4}),
 ("ad", ["a","d"], "LR", "ad", "屈折后缀", {"gap":4}),
 ("uk", ["u","k"], "LR", "uk", "后缀辅音丛", {"gap":4}),
 ("ok", ["o","k"], "LR", "ok", "后缀辅音丛", {"gap":4}),
 ("ek", ["e","k"], "LR", "ek", "后缀辅音丛", {"gap":4}),
 ("ep", ["e","p"], "LR", "ep", "后缀辅音丛", {"gap":4}),

 # ==== r-pronunciation (12组 r-化音) ====
 ("ar", ["a","r"], "LR", "ar", "r-化元音/ɑːr",  {"gap":8}),
 ("er", ["e","r"], "LR", "er", "r-化元音/ɜːr",  {"gap":8}),
 ("ir", ["i","r"], "LR", "ir", "r-化元音/ɪːr",  {"gap":8}),
 ("or", ["o","r"], "LR", "or", "r-化元音/ɔːr",  {"gap":8}),
 ("ur", ["u","r"], "LR", "ur", "r-化元音/ʊːr",  {"gap":8}),
 ("yr", ["y","r"], "LR", "yr", "r-化元音/yːr",  {"gap":8}),
 ("pr", ["p","r"], "LR", "pr", "pr-辅音丛/pr", {"gap":4}),
 ("tr", ["t","r"], "LR", "tr", "tr-辅音丛/tr", {"gap":4}),
 ("kr", ["k","r"], "LR", "kr", "kr-辅音丛/kr", {"gap":4}),
 ("br", ["b","r"], "LR", "br", "br-辅音丛/br", {"gap":4}),
 ("dr", ["d","r"], "LR", "dr", "dr-辅音丛/dr", {"gap":4}),
 ("sr", ["s","r"], "LR", "sr", "sr-辅音丛/sr", {"gap":4}),
 # ==== 额外的辅音簇 ====
 ("fr", ["f","r"], "LR", "fr", "fr-辅音丛/fr", {"gap":4}),
 ("gr", ["g","r"], "LR", "gr", "gr-辅音丛/gr", {"gap":4}),
 ("fl", ["f","l"], "LR", "fl", "fl-辅音丛/fl", {"gap":4}),
 ("gl", ["g","l"], "LR", "gl", "gl-辅音丛/gl", {"gap":4}),
 ("pl", ["p","l"], "LR", "pl", "pl-辅音丛/pl", {"gap":4}),
 ("bl", ["b","l"], "LR", "bl", "bl-辅音丛/bl", {"gap":4}),
 ("cl", ["c","l"], "LR", "cl", "cl-辅音丛/kl", {"gap":4}),
 ("sk", ["s","k"], "LR", "sk", "sk-辅音丛/sk", {"gap":4}),
 ("sp", ["s","p"], "LR", "sp", "sp-辅音丛/sp", {"gap":4}),
 ("st", ["s","t"], "LR", "st", "st-辅音丛/st", {"gap":4}),
 ("sm", ["s","m"], "LR", "sm", "sm-辅音丛/sm", {"gap":4}),
 ("sn", ["s","n"], "LR", "sn", "sn-辅音丛/sn", {"gap":4}),
 ("sw", ["s","w"], "LR", "sw", "sw-辅音丛/sw", {"gap":4}),
 ("tw", ["t","w"], "LR", "tw", "tw-辅音丛/tw", {"gap":4}),
 ("dw", ["d","w"], "LR", "dw", "dw-辅音丛/dw", {"gap":4}),
 ("kw", ["k","w"], "LR", "kw", "kw-辅音丛/kw", {"gap":4}),
 ("qu", ["q","u"], "LR", "qu", "qu-辅音丛/kw", {"gap":4}),
 # ==== 三元相位变体 (已存在 ian/uan/iao/uai/iong; 这里补变体) ====
 # ==== 变形韵母(鼻化韵,nasal) ====
 ("am", ["a","m"], "LR", "am", "鼻化韵/am", {"gap":4}),
 ("im", ["i","m"], "LR", "im", "鼻化韵/im", {"gap":4}),
 ("om", ["o","m"], "LR", "om", "鼻化韵/om", {"gap":4}),
 ("em", ["e","m"], "LR", "em", "鼻化韵/em", {"gap":4}),
 ("on", ["o","n"], "LR", "on", "鼻化韵/on", {"gap":4}),
 # ==== 方言扩展 aa-e 复合 等 ====
 ("ae", ["a","e"], "LR", "ae", "双元音/æ,ɛ", {"gap":4}),

 ("ua", ["u","a"], "LR", "ua", "双元音/ua", {"gap":4}),
 ("uo2",["u","o"], "LR", "uo2","双元音/uo变体", {"gap":4}),
 ("ia", ["i","a"], "LR", "ia", "双元音/ia", {"gap":4}),
 # ==== 特殊合写字 ====
 ("wh", ["w","h"], "LR", "wh", "唇喉复合/ʍ", {"gap":4}),
 ("wr", ["w","r"], "LR", "wr", "wr-复合/r", {"gap":4}),
 ("kn", ["k","n"], "LR", "kn", "kn-复合/n", {"gap":4}),
 ("gn", ["g","n"], "LR", "gn", "gn-复合/n", {"gap":4}),
 ("ps", ["p","s"], "LR", "ps", "ps-复合/ps", {"gap":4}),
 ("pt", ["p","t"], "LR", "pt", "pt-复合/pt", {"gap":4}),
 ("mn", ["m","n"], "LR", "mn", "mn-复合/mn", {"gap":4}),
 # ==== 三元组 ====
 ("aia", ["a","i","a"], "TB", "aia", "三元/aja", {}),
 ("aie", ["a","i","e"], "TB", "aie", "三元/aje", {}),
 ("oia", ["o","i","a"], "TB", "oia", "三元/oja", {}),
 ("oie", ["o","i","e"], "TB", "oie", "三元/oje", {}),
 # ==== 长尾三元组 ====
 ("kei", ["k","e","i"], "LR", "kei", "三元/kei", {}),
 ("gei", ["g","e","i"], "LR", "gei", "三元/gei", {}),
 # ==== 补充三元 : iong变体 ====
 ("uang", ["u","a","n","g"], "LR", "uang", "鼻韵母/uaŋ", {}),
 ("iang", ["i","a","n","g"], "LR", "iang", "鼻韵母/iaŋ", {}),
 ("ueng", ["u","e","n","g"], "LR", "ueng", "鼻韵母/ueŋ", {}),
 # ==== 简写韵 ====
 ("ao_lr", ["a","o"], "LR", "ao", "ao 左右变体", {}),
 ("ao_tb", ["a","o"], "TB", "ao", "ao 上下变体", {"gap":100}),
 # ==== <全元元音组> ====
 # ==== 简写叠音: 送气复合 ====
 ("kh", ["k","h"], "LR", "kh", "送气软腭/kʰ", {"gap":4}),
 ("bh", ["b","h"], "LR", "bh", "送气双唇/bʱ", {"gap":4}),
 ("dh", ["d","h"], "LR", "dh", "送气齿龈/dʱ", {"gap":4}),
 ("jh", ["j","h"], "LR", "jh", "送气硬腭/ɟʱ", {"gap":4}),
 # ===< 末端辅音束 >===
 ("ts", ["t","s"], "LR", "ts", "齿塞擦/ts", {"gap":4}),
 ("dz", ["d","z"], "LR", "dz", "齿塞擦/dz", {"gap":4}),
 ("lj", ["l","j"], "LR", "lj", "软腭化/lj", {"gap":4}),
 ("nj", ["n","j"], "LR", "nj", "腭鼻/nj", {"gap":4}),
 ("rj", ["r","j"], "LR", "rj", "r-腭化", {"gap":4}),
 ("zj", ["z","j"], "LR", "zj", "z-腭化", {"gap":4}),
 # ===< -ed three pronunciations /t /d /ɪd/ ====
 ("et", ["e","t"], "LR", "et", "过去式-/t/", {"gap":4}),
 ("edt",["e","d","t"], "LR", "edt", "过去式-/ɪd/ (三元)", {}),
 ("tu", ["t","u"], "LR", "tu", "tu-复合/tu", {"gap":4}),
 ("du", ["d","u"], "LR", "du", "du-复合/du", {"gap":4}),
 ("vu", ["v","u"], "LR", "vu", "vu-复合/vu", {"gap":4}),
 # ===< 额外复韵 >===
 ("iou", ["i","o","u"], "LR", "iou", "复韵母/iou≈iu", {}),
 ("uei", ["u","e","i"], "LR", "uei", "复韵母/uei≈ui", {}),
 ("ve",  ["v","e"],   "LR", "ve",  "复韵母/ve", {"gap":4}),
 ("va",  ["v","a"],   "LR", "va",  "复韵母/va", {"gap":4}),
 ("vo",  ["v","o"],   "LR", "vo",  "复韵母/vo", {"gap":4}),
]

TOTAL_AUTO = len(AUTO_LIGS)
print(f"[draw_lig] 注册自动融合: {TOTAL_AUTO} 组 + hand-tuned: {len(HARD_TUNED)} 组 = {TOTAL_AUTO+len(HARD_TUNED)} 组总融合字", file=sys.stderr)

# ============================================================
# 字形生成 - 自动融合
# ============================================================
def auto_fuse_polys(defs):
    """
    根据 (名字, [字母...], 模式, 拼音, 描述) 数据条目
    通过通用融合引擎生成新的轮廓列表
    """
    name, letters, mode, pinyin, desc = defs[0], defs[1], defs[2], defs[3], defs[4]
    kw = defs[5] if len(defs) > 5 else {}

    # 取出第一个字母的笔画
    all_polys = []
    for li, letter in enumerate(letters):
        base = LETTERS[letter]
        polys = base()
        if li > 0:
            # 将当前字母与之前的笔画融合
            prev = all_polys
            polys = fuse(prev, polys, mode, **kw)
            # fuse returns prev + transformed current, BUT for further fusions
            # we only want the newly transformed polys
            # actually for chain fusion, fuse already returns prev_polys + new_polys
            # so just skip adding prev again
        all_polys = polys  # fuse returns combined
    return all_polys

def pieces_for(name, defs=None):
    """返回指定名字的字形笔画集合"""
    # 先查 hand_tuned
    if name in HARD_TUNED:
        return HARD_TUNED[name]()

    # 从 AUTO_LIGS 查找
    for entry in AUTO_LIGS:
        if entry[0] == name:
            return auto_fuse_polys(entry)
    return None

# ============================================================
# 字体生成
# ============================================================
def pieces_to_glyph(pieces_list, pen):
    for poly in pieces_list:
        pts = list(poly)
        if len(pts) < 3: continue
        pen.moveTo(pts[0])
        for p in pts[1:]: pen.lineTo(p)
        pen.closePath()

def build_font(out="mqf_radix_fused.ttf"):
    glyph_order = [".notdef"]
    recipe = []

    OWN_CODE = {}

    # 声母字母: 26 个拉丁字母基础层 (保留用于兼容)
    # 但实际上我们要映射到一个干净的容器编码范围内
    # 定义:
    #   C0x: 声母 b f d t n l g k h p m (10)
    #   C1x: 声母 j q x z c s r y w v (10)
    #   C2x: 韵母 a e i o u ü + 延伸 (10)
    #   C40-C7F: 自动融合字 ~110 组
    #   C80-CBF: hand-tuned 融合字 30 组
    #   保留 FA00+ 为 Unicode 后备区

    # 声母第一群 (唇音/舌音): b p m f d t n l g k h → C01-C0A
    initials_g1 = "bmpfdtnlgkh"
    for i, ch in enumerate(initials_g1):
        OWN_CODE[ch] = 0xC01 + i

    # 声母第二群 (齿/喉/半音): j q x z c s r y w v → C10-C19
    initials_g2 = "jqxzcsrywv"
    for i, ch in enumerate(initials_g2):
        OWN_CODE[ch] = 0xC10 + i

    # 韵母单音: a e i o u ü → C20-C25
    vowels = "aeiouv"
    for i, v in enumerate(vowels):
        OWN_CODE[v] = 0xC20 + i

    # hand-tuned 融合字 → C40-C5D (30 组)
    ht_base = 0xC40
    ht_codes = {}
    for i, name in enumerate(HARD_TUNED.keys()):
        ht_codes[name] = ht_base + i

    # 自动融合字 → C80-CFE (~110 组)
    auto_base = 0xC80
    auto_codes = {}
    for i, entry in enumerate(AUTO_LIGS):
        auto_codes[entry[0]] = auto_base + i

    # 后备 Unicode 层:
    # Latin a-z = U+0061-007A (兼容)
    # Hard-tuned 融合字 FA00-FA1D
    # 自动融合 字 FA20-FA8E

    cmap_own = {}
    cmap_unicode = {}

    def add_glyph(gname):
        glyph_order.append(gname)
        return len(glyph_order) - 1

    # 基础字母
    for ch in "abcdefghijklmnopqrstuvwxyz":
        add_glyph(ch)
        cmap_own[OWN_CODE.get(ch, 0xC20)] = ch
        cmap_unicode[ord(ch)] = ch

    # hand-tuned 融合字
    for name, fn in HARD_TUNED.items():
        gname = f"lig.{name}"
        add_glyph(gname)
        code = ht_codes[name]
        cmap_own[code] = gname
        cmap_unicode[0xFA00+len([x for x in recipe])] = gname
        recipe.append((name, fn.__name__.replace("pieces_",""), mode_for(name), f"C{code:03X}"))

    # 自动融合字
    for entry in AUTO_LIGS:
        name = entry[0]
        gname = f"lig.{name}"
        add_glyph(gname)
        code = auto_codes[name]
        cmap_own[code] = gname
        unicode_slot = 0xFA20 + len([x for x in recipe if not x[0].startswith("lig.")])  # continue numbering
        cmap_unicode[unicode_slot] = gname
        recipe.append((name, entry[1], entry[2], f"C{code:03X}"))

    # 构建字体骨架
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

    def build_pieces_for(gname):
        if gname == ".notdef": return box_glyph, (0,0)
        if gname in LETTER_PIECES_old():
            p = TTGlyphPen(None)
            pieces_to_glyph(LETTER_PIECES_old()[gname](), p)
            return p.glyph(), (advance, 0)
        if gname.startswith("lig."):
            name = gname[4:]
            pieces = pieces_for(name)
            if pieces:
                p = TTGlyphPen(None)
                pieces_to_glyph(pieces, p)
                return p.glyph(), (advance, 0)
        return box_glyph, (advance, 0)

    for gname in glyph_order:
        g, m = build_pieces_for(gname)
        glyf[gname] = g; metrics[gname] = m

    fb.setupGlyf(glyf)
    fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupCharacterMap(cmap_unicode)
    fb.setupNameTable({
        "familyName": "MQF Container Radix Fused v2",
        "styleName": "FusedAuto",
        "psName": "MQF-ContainerRadix-Fused-v2",
        "manufacturer": "囝囝共生 · 刘楚恬和我",
        "designer": "lolict + 刘楚恬",
        "description": f"满全法容器进制融合造字 v2: {len(HARD_TUNED)} hand-tuned + {len(AUTO_LIGS)} 自动融合 = {len(HARD_TUNED)+len(AUTO_LIGS)} 组",
        "vendorURL": "https://github.com/lolict/hunyuan-os",
        "licenseDescription": "Private",
        "version": "0.2.0",
    })
    fb.setupOS2(sTypoAscender=800, sTypoDescent=-200, usWinAscent=800, usWinDescent=200)
    fb.setupPost()

    font = fb.font

    # 插入自有 Symbol cmap
    from fontTools import ttLib
    sub_sym = CmapSubtable.newSubtable(12)  # format 12 支持 > 16-bit codes
    sub_sym.platformID = 3
    sub_sym.platEncID = 0   # Symbol
    sub_sym.format = 12
    sub_sym.reserved = 0
    sub_sym.length = 0
    sub_sym.language = 0
    sub_sym.cmap = cmap_own
    font["cmap"].tables.insert(0, sub_sym)

    # 配方写入 name table
    for i, (name, comp, mode, code) in enumerate(recipe):
        rec = f"{name}={comp}@{code}[{mode}]"
        font["name"].setName(rec, 256+i, 3, 1, 0x0409)

    font.save(out)
    font.close()
    return out, len(glyph_order), len(recipe), recipe

def LETTER_PIECES_old():
    return LETTERS  # 别名

def mode_for(name):
    """返回 hand-tuned 名字的模式"""
    MODE_MAP = {
        "an":"LR","en":"LR","in":"LR","un":"LR","vn":"LR",
        "ai":"LR","ei":"LR","ao":"LR","ou":"SS","ie":"TB",
        "uo":"TB","io":"TB","ue":"LR","ui":"LR","iu":"LR",
        "eo":"IO","oe":"IO","eu":"IO",
        "ang":"LR","eng":"LR","ing":"LR","ong":"LR",
        "ian":"LR","uan":"TB","iao":"TB","uai":"TB","iong":"IO",
        "ch":"LR","sh":"LR","zh":"LR",
    }
    return MODE_MAP.get(name, "LR")

# ============================================================
# CLI 入口
# ============================================================
if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "mqf_radix_fused.ttf"
    out, total, n, rec = build_font(out)
    size = os.path.getsize(out)
    print(f"字体: {out} ({size} bytes)")
    print(f"总字形: {total}")
    print(f"融合字: {n}")
    print(f"  手工设计: {len(HARD_TUNED)}")
    print(f"  自动融合: {len(AUTO_LIGS)}")
    print(f"\n编码范围:")
    print(f"  声母: C01..C0A, C10..C19")
    print(f"  韵母: C20..C25")
    print(f"  手工融合: C40..C5D")
    print(f"  自动融合: C80..CFE")
    print(f"  后备 Unicode: FA00..")
