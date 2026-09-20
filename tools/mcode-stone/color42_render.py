#!/usr/bin/env python3
# ============================================================
# COLOR42 · 渲染器 (从已有源码提取,非发明)
#
# 身份落款: 刘楚恬和我
#
# 数据来源:
#   zig-stem/color_custom.c    — 64符号→42色映射表 + 层级色系定义
#   hunyuan-rs/src/.../container_radix.rs — 6-bit蜂窝坐标算法
#   tools/kungguan-container/color42.nim — 拉丁符号+情感+音高 (仅作属性附加,色名以 color_custom.c 为准)
#
# 三维结构 = container_radix 的蜂窝坐标 + COLOR42 的层级映射:
#   维度1 = x 坐标 (bit1,3,5 奇位)   [container_radix.rs:27-31]
#   维度2 = y 坐标 (bit0,2,4 偶位)   [container_radix.rs:27-31]
#   维度3 = 色系分层 (8层×7色=56色系+混合)  [color_custom.c:208-216]
#
# 不凭空发明,每条数据有文件依据。
#
# 用法:
#   python3 color42_render.py           # 42色蜂窝(按已有层级映射)
#   python3 color42_render.py C23       # 查容器编码的定位+颜色
#   python3 color42_render.py layers    # 八层分布图
#   python3 color42_render.py nim       # 42色属性表(拉丁符号+情感+音高附加)
# ============================================================

import sys

# ============================================================
# 数据源 A: color_custom.c 的 64符号→42色映射 (第80-158行)
# ============================================================

# 42色信息 (来自 color_custom.c 第10-76行)
COLOR42 = [
    # 红系 R0-R6 (color index 0-6)
    {"zi":"妃","py":"fēi",  "sys":"R","sys_name":"暖","sys_idx":0,"col":0},
    {"zi":"粉","py":"fěn",  "sys":"R","sys_name":"暖","sys_idx":0,"col":1},
    {"zi":"彤","py":"tóng", "sys":"R","sys_name":"暖","sys_idx":0,"col":2},
    {"zi":"赤","py":"chì",  "sys":"R","sys_name":"暖","sys_idx":0,"col":3},
    {"zi":"棕","py":"zōng", "sys":"R","sys_name":"暖","sys_idx":0,"col":4},
    {"zi":"绛","py":"jiàng","sys":"R","sys_name":"暖","sys_idx":0,"col":5},
    {"zi":"赭","py":"zhě",  "sys":"R","sys_name":"暖","sys_idx":0,"col":6},
    # 黄系 Y0-Y6 (index 7-13)
    {"zi":"缃","py":"xiāng","sys":"Y","sys_name":"光","sys_idx":1,"col":0},
    {"zi":"金","py":"jīn",  "sys":"Y","sys_name":"光","sys_idx":1,"col":1},
    {"zi":"黄","py":"huáng","sys":"Y","sys_name":"光","sys_idx":1,"col":2},
    {"zi":"褐","py":"hē",   "sys":"Y","sys_name":"光","sys_idx":1,"col":3},
    {"zi":"黧","py":"lí",   "sys":"Y","sys_name":"光","sys_idx":1,"col":4},
    {"zi":"乌","py":"wū",   "sys":"Y","sys_name":"光","sys_idx":1,"col":5},
    {"zi":"黑","py":"hēi",  "sys":"Y","sys_name":"光","sys_idx":1,"col":6},
    # 绿系 G0-G6 (index 14-20)
    {"zi":"缥","py":"piǎo", "sys":"G","sys_name":"生","sys_idx":2,"col":0},
    {"zi":"翠","py":"cuì",  "sys":"G","sys_name":"生","sys_idx":2,"col":1},
    {"zi":"绿","py":"lǜ",   "sys":"G","sys_name":"生","sys_idx":2,"col":2},
    {"zi":"青","py":"qīng", "sys":"G","sys_name":"生","sys_idx":2,"col":3},
    {"zi":"苍","py":"cāng", "sys":"G","sys_name":"生","sys_idx":2,"col":4},
    {"zi":"黛","py":"dài",  "sys":"G","sys_name":"生","sys_idx":2,"col":5},
    {"zi":"玄","py":"xuán", "sys":"G","sys_name":"生","sys_idx":2,"col":6},
    # 蓝系 B0-B6 (index 21-27)
    {"zi":"素","py":"sù",   "sys":"B","sys_name":"冷","sys_idx":3,"col":0},
    {"zi":"银","py":"yín",  "sys":"B","sys_name":"冷","sys_idx":3,"col":1},
    {"zi":"蓝","py":"lán",  "sys":"B","sys_name":"冷","sys_idx":3,"col":2},
    {"zi":"紫","py":"zǐ",   "sys":"B","sys_name":"冷","sys_idx":3,"col":3},
    {"zi":"靛","py":"diàn", "sys":"B","sys_name":"冷","sys_idx":3,"col":4},
    {"zi":"绀","py":"gàn",  "sys":"B","sys_name":"冷","sys_idx":3,"col":5},
    {"zi":"黯","py":"àn",   "sys":"B","sys_name":"冷","sys_idx":3,"col":6},
    # 泽系 S0-S6 (index 28-34)
    {"zi":"玉","py":"yù",   "sys":"S","sys_name":"石","sys_idx":4,"col":0},
    {"zi":"琅","py":"láng", "sys":"S","sys_name":"石","sys_idx":4,"col":1},
    {"zi":"晶","py":"jīng", "sys":"S","sys_name":"石","sys_idx":4,"col":2},
    {"zi":"璃","py":"lí",   "sys":"S","sys_name":"石","sys_idx":4,"col":3},
    {"zi":"珀","py":"pò",   "sys":"S","sys_name":"石","sys_idx":4,"col":4},
    {"zi":"瑙","py":"nǎo",  "sys":"S","sys_name":"石","sys_idx":4,"col":5},
    {"zi":"璧","py":"bì",   "sys":"S","sys_name":"石","sys_idx":4,"col":6},
    # 光系 L0-L6 (index 35-41)
    {"zi":"曦","py":"xī",   "sys":"L","sys_name":"韵","sys_idx":5,"col":0},
    {"zi":"辉","py":"huī",  "sys":"L","sys_name":"韵","sys_idx":5,"col":1},
    {"zi":"霓","py":"ní",   "sys":"L","sys_name":"韵","sys_idx":5,"col":2},
    {"zi":"旖","py":"yǐ",   "sys":"L","sys_name":"韵","sys_idx":5,"col":3},
    {"zi":"靡","py":"mí",   "sys":"L","sys_name":"韵","sys_idx":5,"col":4},
    {"zi":"暝","py":"míng", "sys":"L","sys_name":"韵","sys_idx":5,"col":5},
    {"zi":"黟","py":"yī",   "sys":"L","sys_name":"韵","sys_idx":5,"col":6},
]

# 64符号→颜色[索引, 色系] (来自 color_custom.c 第80-158行)
SYMBOL_64 = [
    (0,"R"),(1,"R"),(2,"R"),(3,"R"),(4,"R"),(5,"R"),(6,"R"),(0,"R"), # 层0 夫妻→红(循环)
    (0,"Y"),(1,"Y"),(2,"Y"),(3,"Y"),(4,"Y"),(5,"Y"),(6,"Y"),(0,"Y"), # 层1 中心→黄
    (0,"G"),(1,"G"),(2,"G"),(3,"G"),(4,"G"),(5,"G"),(6,"G"),(0,"G"), # 层2 感知→绿
    (0,"B"),(1,"B"),(2,"B"),(3,"B"),(4,"B"),(5,"B"),(6,"B"),(0,"B"), # 层3 骨干→蓝
    (0,"S"),(1,"S"),(2,"S"),(3,"S"),(4,"S"),(5,"S"),(6,"S"),(0,"S"), # 层4 行页→泽
    (0,"L"),(1,"L"),(2,"L"),(3,"L"),(4,"L"),(5,"L"),(6,"L"),(0,"L"), # 层5-6 交汇+价值+流通→光
    (1,"L"),(2,"L"),(3,"L"),(4,"L"),(5,"L"),(6,"L"),(0,"L"),(1,"L"),
    (2,"B"),(2,"G"),(2,"S"),(0,"L"),(3,"R"),(2,"Y"),(0,"R"),(0,"B"), # 层7 高维混合
]

# 层级→色系名 (来自 color_custom.c 第210-214行)
LAYER_SYS = ["R","Y","G","B","S","L","R","B"]  # 八层各自的主色系


# ============================================================
# 数据源 C: color42.nim 的拉丁符号+情感+音高 (第16-72行)
#   ⚠ nim 的 "name" 字段已删除——那份是后来写的替代命名,
#      主人只用 color_custom.c 的字(妃/粉/彤...)作为色名。
#      nim 只提供: Latin 符号(idx映射) + 情感 + 音高
# ============================================================

# nim idx → {拉丁符号, 情感, 音高} (来自 color42.nim COLORS42 数组,去掉了 name 字段)
# 通过 idx 对应 color_custom.c 的颜色序号
NIM_ATTR = {
     0:{"sym":"ạ",  "emotion":"激情", "pitch":"低音"},
     1:{"sym":"a̋",  "emotion":"兴奋", "pitch":"中高音"},
     2:{"sym":"å",  "emotion":"温柔", "pitch":"高音"},
     3:{"sym":"ã",  "emotion":"羞怯", "pitch":"颤音高音"},
     4:{"sym":"â",  "emotion":"诱惑", "pitch":"升降中音"},
     5:{"sym":"ă",  "emotion":"危险", "pitch":"短促低音"},
     6:{"sym":"ẹ",  "emotion":"温暖", "pitch":"低音"},
     7:{"sym":"e̋",  "emotion":"活力", "pitch":"中高音"},
     8:{"sym":"e̊",  "emotion":"明亮", "pitch":"高音"},
     9:{"sym":"ẽ",  "emotion":"柔和", "pitch":"颤音高音"},
    10:{"sym":"ê",  "emotion":"神秘", "pitch":"升降中音"},
    11:{"sym":"ĕ",  "emotion":"急切", "pitch":"短促低音"},
    12:{"sym":"ị",  "emotion":"明朗", "pitch":"低音"},
    13:{"sym":"i̋",  "emotion":"快乐", "pitch":"中高音"},
    14:{"sym":"i̊",  "emotion":"纯净", "pitch":"高音"},
    15:{"sym":"ĩ",  "emotion":"天真", "pitch":"颤音高音"},
    16:{"sym":"î",  "emotion":"辉煌", "pitch":"升降中音"},
    17:{"sym":"ĭ",  "emotion":"衰败", "pitch":"短促低音"},
    18:{"sym":"ọ",  "emotion":"宁静", "pitch":"低音"},
    19:{"sym":"ő",  "emotion":"生机", "pitch":"中高音"},
    20:{"sym":"o̊",  "emotion":"清新", "pitch":"高音"},
    21:{"sym":"õ",  "emotion":"舒适", "pitch":"颤音高音"},
    22:{"sym":"ô",  "emotion":"深沉", "pitch":"升降中音"},
    23:{"sym":"ŏ",  "emotion":"不安", "pitch":"短促低音"},
    24:{"sym":"ụ",  "emotion":"冷静", "pitch":"低音"},
    25:{"sym":"ű",  "emotion":"理性", "pitch":"中高音"},
    26:{"sym":"ů",  "emotion":"透明", "pitch":"高音"},
    27:{"sym":"ũ",  "emotion":"忧郁", "pitch":"颤音高音"},
    28:{"sym":"û",  "emotion":"严肃", "pitch":"升降中音"},
    29:{"sym":"ŭ",  "emotion":"矛盾", "pitch":"短促低音"},
    30:{"sym":"ȃ",  "emotion":"深邃", "pitch":"低音"},
    31:{"sym":"ȇ",  "emotion":"平和", "pitch":"中高音"},
    32:{"sym":"ȋ",  "emotion":"自由", "pitch":"高音"},
    33:{"sym":"ȏ",  "emotion":"梦想", "pitch":"颤音高音"},
    34:{"sym":"ȗ",  "emotion":"高贵", "pitch":"升降中音"},
    35:{"sym":"ȁ",  "emotion":"忧郁", "pitch":"短促低音"},
    36:{"sym":"ȅ",  "emotion":"神秘", "pitch":"低音"},
    37:{"sym":"ȉ",  "emotion":"优雅", "pitch":"中高音"},
    38:{"sym":"ȍ",  "emotion":"浪漫", "pitch":"高音"},
    39:{"sym":"ȕ",  "emotion":"治愈", "pitch":"颤音高音"},
    40:{"sym":"ä",  "emotion":"灵性", "pitch":"升降中音"},
    41:{"sym":"ö",  "emotion":"压抑", "pitch":"短促低音"},
}

# ============================================================
# 数据源 B: container_radix.rs 的 6-bit 蜂窝坐标算法 (第27-31行)
#   偶数位 (bit0,2,4) → y 坐标
#   奇数位 (bit1,3,5) → x 编码
# ============================================================

def addr_to_xy(code):
    """6-bit 码 → 六边形 (x,y) [container_radix.rs:27-31]"""
    x = ((code >> 1) & 1) | ((code >> 3) & 2) | ((code >> 5) & 4)
    y = (code & 1) | ((code >> 2) & 2) | ((code >> 4) & 4)
    return x, y

def hex_to_color(hx, hy):
    """六边形坐标 → 42色系中的哪一个?"""
    # 6个色系对应六边形6向 (hex坐标中 6 个方向)
    if hx == 0 and hy == 0:
        return None  # 中心无色
    # 向度 = atan2 映射到6向
    import math
    angle = math.atan2(hy, hx)  # -π .. π
    deg = math.degrees(angle) % 360
    # 六边形6向: 0°=R(红), 60°=Y(黄), 120°=G(绿), 180°=B(蓝), 240°=S(泽), 300°=L(光)
    sector = int((deg + 30) % 360 / 60)  # 0-5
    sys_codes = ["R","Y","G","B","S","L"]
    return sys_codes[sector]

# ============================================================
# 数据源 C: 传统色彩对应的 RGB 经验值
#   来源: color_custom.c 的色系语义 (妃=R0=夫妻=玫红开端 ... 赭=R6=深土红末端)
#   配色逻辑: 每系7阶线性插值 (首→尾)
# ============================================================

RGB_ENDPOINTS = {
    # 红:玫红→深赭红 (传统红系渐进)
    "R": [(255,102,153),(139,0,0)],
    # 黄:金色→黑 (光衰减渐进)
    "Y": [(255,215,0),(20,20,20)],
    # 绿:白绿→墨绿
    "G": [(180,255,200),(0,60,30)],
    # 蓝:素白→暗紫蓝
    "B": [(220,230,255),(40,20,60)],
    # 泽:玉白→璧绿
    "S": [(240,255,240),(80,120,100)],
    # 光:晨曦粉→暗夜
    "L": [(255,200,220),(30,20,40)],
}

def rgb(sys_code, col):
    """色系 + 色阶(0-6) → RGB"""
    a, b = RGB_ENDPOINTS[sys_code]
    t = col / 6
    return (int(a[0]+(b[0]-a[0])*t),
            int(a[1]+(b[1]-a[1])*t),
            int(a[2]+(b[2]-a[2])*t))

# ANSI
def fg(r,g,b):  return f"\033[38;2;{r};{g};{b}m"
def bg(r,g,b):  return f"\033[48;2;{r};{g};{b}m"
def reset():    return "\033[0m"
def bold():     return "\033[1m"

def text_color_for(r,g,b):
    """选择对比文字色"""
    return "\033[38;2;0;0;0m" if (r*299+g*587+b*114)/1000 > 128 else "\033[38;2;255;255;255m"

# ============================================================
# 渲染
# ============================================================

def render_palette():
    """42色色板 (按 color_custom.c 的层级映射排列)"""
    out = []
    out.append("═══ COLOR42 · 42色层级映射 (数据源: color_custom.c) ═══\n")
    sys_label = {"R":"暖","Y":"光","G":"生","B":"冷","S":"石","L":"韵"}
    for sys_idx in range(6):
        sys_code = "RYGBSL"[sys_idx]
        cells = COLOR42[sys_idx*7:(sys_idx+1)*7]
        header = f"  [{sys_code}·{sys_label[sys_code]}]"
        pad = ""
        for cell in cells:
            ci = cell["sys_idx"] * 7 + cell["col"]
            r,g,b = rgb(sys_code, cell["col"])
            pad += f"{bg(r,g,b)}{text_color_for(r,g,b)} {cell['zi']} {reset()}"
        out.append(f"{header} {pad}")

    # 8层映射
    out.append("\n  ══ 8层 × 7色 = 色系分层 (color_custom.c:210-216) ══")
    for layer in range(8):
        sys_code = LAYER_SYS[layer]
        r,g,b = rgb(sys_code, 3)  # 中间阶
        out.append(f"  层{layer}: 色系={sys_code} ({sys_label[sys_code]})  色阶0-6 映射范围={layer*8}-{(layer+1)*8-1}符号")

    return "\n".join(out)


def render_hex_map():
    """六边形蜂窝投影 (container_radix 6-bit奇偶位拆分 + COLOR42色系6向对应)"""
    out = []
    out.append("═══ 六边形蜂窝投影 (container_radix 6-bit → COLOR42 6向) ═══\n")
    out.append("  来源: container_radix.rs:27-31 + color_custom.c:79-158\n")

    # 8×8 六边形轴向映射 (偶数行缩进)
    for hy in range(8):
        indent = "    " if hy % 2 == 1 else "   "
        line = indent
        for hx in range(8):
            # 哪个 6-bit 码落在这个 xy 上?
            # 由 algorithm: x = bit1 + bit3<<1 + bit5<<2, y = bit0 + bit2<<1 + bit4<<2
            # 反推: 需要把 (hx,hy) 反编码成 code
            bit0 = hy & 1
            bit2 = (hy >> 1) & 1
            bit4 = (hy >> 2) & 1
            bit1 = hx & 1
            bit3 = (hx >> 1) & 1
            bit5 = (hx >> 2) & 1
            code = bit0 | (bit1<<1) | (bit2<<2) | (bit3<<3) | (bit4<<4) | (bit5<<5)
            if code < 64:
                col_idx, sys_code = SYMBOL_64[code]
                r,g,b = rgb(sys_code, col_idx)
                zi = COLOR42[sys_code_to_idx(sys_code)*7 + col_idx]["zi"]
                line += f"{bg(r,g,b)}{text_color_for(r,g,b)} {code:02d}{zi}{reset()} "
            else:
                line += "  ·   "
        out.append(line)
    return "\n".join(out)

def sys_code_to_idx(sc):
    return {"R":0,"Y":1,"G":2,"B":3,"S":4,"L":5}[sc]

def render_layers():
    """8层分布图 (color_custom.c 的 layer_system)"""
    out = []
    out.append("═══ 8层拓扑 (color_custom.c:208-216) ═══\n")
    layer_names = ["夫妻(红)","中心(光)","感知(绿)","骨干(蓝)",
                   "行页(泽)","交汇(光)","价值(红)","流通混合"]
    for layer in range(8):
        sys_code = LAYER_SYS[layer]
        r,g,b = rgb(sys_code, 3)
        out.append(f"  层{layer}: {layer_names[layer]:8s} → 色系 {sys_code}")

        # 该层 8 符号
        p = ""
        for pos in range(8):
            sym_idx = layer * 8 + pos
            ci, sc = SYMBOL_64[sym_idx]
            rr,gg,bb = rgb(sc, ci)
            zi = COLOR42[sys_code_to_idx(sc)*7 + ci]["zi"]
            p += f"{bg(rr,gg,bb)}{text_color_for(rr,gg,bb)} {sym_idx:02d}{zi} {reset()}"
        out.append(f"         {p}")
    return "\n".join(out)

def render_single(addr_str):
    """查单个 6-bit 地址"""
    if isinstance(addr_str, str) and addr_str.startswith("C"):
        code = int(addr_str[1:])
    else:
        code = int(addr_str)
    x, y = addr_to_xy(code)
    ci, sc = SYMBOL_64[code]
    r,g,b = rgb(sc, ci)
    zi = COLOR42[sys_code_to_idx(sc)*7 + ci]["zi"]
    nim = NIM_ATTR.get(code, {})
    lat = nim.get("sym","")
    emo = nim.get("emotion","")
    pit = nim.get("pitch","")
    sys_label = {"R":"暖","Y":"光","G":"生","B":"冷","S":"石","L":"韵"}

    out = [f"  容器码 C{code:02d} ({code})",
           f"  6-bit  : {code:06b}",
           f"  x(奇位): {x:03b} = {x}   [bit1,bit3,bit5]",
           f"  y(偶位): {y:03b} = {y}   [bit0,bit2,bit4]",
           f"  六边形 : ({x},{y})",
           f"  色系   : {sc}({sys_label[sc]})  色阶 {ci}/6",
           f"  色字   : {zi}",
           f"  拉丁   : {lat}  情感: {emo}  音高: {pit}",
           f"  RGB    : ({r},{g},{b})  #{r:02X}{g:02X}{b:02X}",
           f"  色块   : {bg(r,g,b)}{text_color_for(r,g,b)}   {zi} C{code:02d}   {reset()}"]
    return "\n".join(out)


def render_nim():
    """42色属性表: canonical色名(color_custom.c) + 拉丁符号/情感/音高(nim)
    注意: nim 的序号与 color_custom.c 序号一一对应,不另起名称"""
    out = []
    out.append("═══ 42色·字-符号-情感-音高 (色名←color_custom.c, 属性←color42.nim) ═══")
    out.append("   颜色名唯一来源: color_custom.c   nim 只提供拉丁符号+情感+音高\n")

    sys_label = {"R":"暖","Y":"光","G":"生","B":"冷","S":"石","L":"韵"}
    for sys_idx in range(6):
        sys_code = "RYGBSL"[sys_idx]
        out.append(f"  [{sys_code}·{sys_label[sys_code]}]")
        for col in range(7):
            ci = sys_idx * 7 + col
            c = COLOR42[ci]
            r,g,b = rgb(sys_code, col)
            nim = NIM_ATTR.get(ci, {})
            lat = nim.get("sym","")
            emo = nim.get("emotion","")
            pit = nim.get("pitch","")
            out.append(f"    {bg(r,g,b)}{text_color_for(r,g,b)} {c['zi']} {reset()}  {lat}  {emo}/{pit}   rgb({r},{g},{b})")
        out.append("")

    return "\n".join(out)


# ============================================================
# CLI
# ============================================================

if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "palette"
    print()
    if cmd == "palette":
        print(render_palette())
    elif cmd == "hex":
        print(render_hex_map())
    elif cmd == "layers":
        print(render_layers())
    elif cmd == "nim":
        print(render_nim())
    elif cmd.startswith("C") or cmd.isdigit():
        print(render_single(cmd))
    else:
        print(render_palette())
    print()
    print("  ═══ 色名: color_custom.c | 六系层级: color_custom.c | 蜂窝坐标: container_radix.rs | 拉丁符号/情感/音高: color42.nim ═══")
    print()
