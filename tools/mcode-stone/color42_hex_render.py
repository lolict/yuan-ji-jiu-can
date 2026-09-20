#!/usr/bin/env python3
# ============================================================
# COLOR42 · 六边形蜂窝渲染器
#
# 身份落款: 刘楚恬和我
#
# 三维结构:
#   维度1 = 六边形 x 坐标 (容器码奇数位 bit1,3,5)
#   维度2 = 六边形 y 坐标 (容器码偶数位 bit0,2,4)
#   维度3 = 色系 (6 向之一 → 6 行语义层)
#
# COLOR42 = 6(行) × 7(列) = 42
#   6 行 = 六边形 6 个向度 (红/黄/绿/蓝/泽/光)
#   7 列 = 该向度内 7 阶深浅
#
# 这样 COLOR42 不是 "42 个平坑",而是铺在六边形蜂窝上的
# 二维坐标+一向色系的  三维地址空间。
#
# 用法:
#   python3 color42_hex_render.py            # 六边形蜂窝全图
#   python3 color42_hex_render.py C23        # 查容器 C23 的蜂窝位置
#   python3 color42_hex_render.py grid       # 42 色蜂窝矩阵(平面)
# ============================================================

import sys

# ── 42 色表 ──
COLOR42 = [
    # 红·暖 (R)
    ["妃","粉","彤","赤","棕","绛","赭"],
    # 黄·光 (Y)
    ["缃","金","黄","褐","黧","乌","黑"],
    # 绿·生 (G)
    ["缥","翠","绿","青","苍","黛","玄"],
    # 蓝·冷 (B)
    ["素","银","蓝","紫","靛","绀","黯"],
    # 泽·石 (S)
    ["玉","琅","晶","璃","珀","瑙","璧"],
    # 光·韵 (L)
    ["曦","辉","霓","旖","靡","暝","黟"],
]

SYSTEM_NAMES = ["暖","光","生","冷","石","韵"]
SYSTEM_CODES = ["R","Y","G","B","S","L"]

# 每行 RGB 起止 (6 行 × 7 阶插值)
_RGB_RANGE = [
    [(255,102,153),(139,0,0)],     # 红:玫→赭
    [(255,215,0),(20,20,20)],       # 黄:金→黑
    [(180,255,200),(0,60,30)],      # 绿:白绿→玄
    [(220,230,255),(40,20,60)],     # 蓝:素→黯
    [(240,255,240),(80,120,100)],   # 泽:玉→璧
    [(255,200,220),(30,20,40)],     # 光:曦→黟
]

def rgb_at(row, col):
    a, b = _RGB_RANGE[row]
    t = col / 6
    return (int(a[0]+(b[0]-a[0])*t), int(a[1]+(b[1]-a[1])*t), int(a[2]+(b[2]-a[2])*t))

def hex_at(row, col):
    r,g,b = rgb_at(row, col)
    return f"#{r:02X}{g:02X}{b:02X}"

# ── ANSI 控制 ──
def fg(r,g,b): return f"\033[38;2;{r};{g};{b}m"
def bg(r,g,b): return f"\033[48;2;{r};{g};{b}m"
def bold():    return "\033[1m"
def reset():   return "\033[0m"

# ── 六边形坐标转换 ──
def addr_to_xy(addr):
    """
    容器编码 → 六边形 (x,y)
      bit0,2,4 → y
      bit1,3,5 → x
    """
    if isinstance(addr, str) and addr.startswith("C"):
        n = int(addr[1:])
    elif isinstance(addr, int):
        n = addr
    else:
        n = int(addr)
    x = ((n>>1)&1) | ((n>>3)&2) | ((n>>5)&4)
    y = (n&1) | ((n>>2)&2) | ((n>>4)&4)
    return n, x, y

def xy_to_color(row, col):
    """色系(行) × 色阶(列) → RGB + 汉字"""
    r,g,b = rgb_at(row, col)
    zi = COLOR42[row][col]
    return zi, r, g, b

# ── 渲染 ──

def render_hex_grid():
    """
    六边形蜂窝图:
      - 6 个色系对应六边形 6 个向
      - 每系 7 阶深浅从中心向外辐射 或 从左到右渐变
      - 42 个小六边形拼成大蜂窝
    """
    out = []
    out.append("═══ COLOR42 六边形蜂窝 ═══\n")
    out.append("  6 向度 × 7 阶 × 42 色 = 六边形三维地址空间\n")

    # 布局: 6 行蜂窝,每行 7 个"小六边形块"
    for row in range(6):
        # 每个色系一行,每行 7 个色块
        # 六边形交错: 奇数行缩进
        indent = "   " if row % 2 == 1 else ""
        line_top = indent
        line_mid = indent
        line_bot = indent
        for col in range(7):
            r,g,b = rgb_at(row, col)
            zi = COLOR42[row][col]
            lum = (r*299+g*587+b*114)/1000
            fgr = "\033[38;2;0;0;0m" if lum>128 else "\033[38;2;255;255;255m"
            # 小六边形 3 行高
            c = f"{bg(r,g,b)}{fgr}"
            idx = row*7 + col
            line_top += f"{c} ▲{idx:02d} {reset()}"
            line_mid += f"{c}  {zi}  {reset()}"
            line_bot += f"{c}  ▼  {reset()}"
        out.append(f"  [{SYSTEM_CODES[row]}·{SYSTEM_NAMES[row]}]")
        out.append(line_top)
        out.append(line_mid)
        out.append(line_bot)
        out.append("")

    return "\n".join(out)


def render_flat_hex():
    """
    平面六边形坐标映射:
     8×8 的矩阵,42 个色按六边形 xy 填入,空白处留白
     """
    out = []
    out.append("═══ COLOR42 六边形平面投影 ═══\n")
    out.append("  地址(0-41) → 六边形( x,y ) → 填颜色\n")
    out.append("     ┌───六 边 形 网 格 (x→ y↓)───┐\n")

    # 用六边形轴向坐标的 odd-r 水平布局
    # 偶数行半格缩进
    for y in range(8):
        indent = "     " if y % 2 == 1 else "    "
        line = indent
        for x in range(8):
            # 反求: 是否有哪个 addr 落到这个 xy?
            found = None
            for addr in range(42):
                _, ax, ay = addr_to_xy(addr)
                if ax == x and ay == y:
                    found = addr
                    break
            if found is not None:
                row = found // 7
                col = found % 7
                r,g,b = rgb_at(row, col)
                zi = COLOR42[row][col]
                lum = (r*299+g*587+b*114)/1000
                fgr = "\033[38;2;0;0;0m" if lum>128 else "\033[38;2;255;255;255m"
                line += f"{bg(r,g,b)}{fgr} {found:02d}{zi}{reset()} "
            else:
                line += "  ·   "
        out.append(line)
    out.append("")
    return "\n".join(out)


def render_addr(addr):
    """查单个地址的六边形定位+颜色"""
    n, x, y = addr_to_xy(addr)
    if isinstance(addr, str) and addr.startswith("C"):
        n = int(addr[1:])
    else:
        n = int(addr)
    row = n // 7
    col = n % 7
    zi = COLOR42[row][col]
    r,g,b = rgb_at(row, col)
    hx = hex_at(row, col)
    lum = (r*299+g*587+b*114)/1000
    fgr = "\033[38;2;0;0;0m" if lum>128 else "\033[38;2;255;255;255m"

    out = []
    out.append(f"  地址 C{n:02d} ({n})\n")
    out.append(f"  容器 6bit: {n:06b}")
    out.append(f"  x(奇位): {x:03b} → {x}")
    out.append(f"  y(偶位): {y:03b} → {y}")
    out.append(f"  六边形:({x},{y})")
    out.append(f"  色系:第{row}向·{SYSTEM_CODES[row]}({SYSTEM_NAMES[row]})")
    out.append(f"  色阶:{col}/6")
    out.append(f"  色字:{zi}")
    out.append(f"  RGB:({r},{g},{b})")
    out.append(f"  HEX:{hx}")
    out.append(f"  颜色块: {bg(r,g,b)}{fgr}   {zi} C{n:02d}   {reset()}")
    return "\n".join(out)


# ── CLI ──

if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv)>1 else "hex"
    print()
    if cmd == "hex":
        print(render_hex_grid())
    elif cmd == "grid" or cmd == "flat":
        print(render_flat_hex())
    elif cmd.startswith("C") or cmd.isdigit():
        print(render_addr(cmd))
    else:
        print(render_hex_grid())
    print("  ══ 颜色进制 = 六边形 6 向度 + 色阶 + xy 投影 = 三维 ══")
    print()
