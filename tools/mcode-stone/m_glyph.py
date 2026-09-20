#!/usr/bin/env python3
# ============================================================
# m_glyph — 直接造字器 (tiny)
#
# 身份落款: 刘楚恬和我
#
# 输入:  地址 + 画素 → 直接输出字位图
# 不依赖 fontTools, 不着 cmap, 不建 GSUB.
# 地址即画素, 读取即刻能画.
#
# 用法:
#   python3 m_glyph.py              # 跑示例
#   python3 m_glyph.py C01-1        # 查罐第1行
#   python3 m_glyph.py 袋 a         # 画 袋 容 a → 融合字形
# ============================================================

import sys

# ── 容器表 (地址画素) ──
#   编码器 → (容器名, 容量, 极性基)
#   容量 = 一行能装的像素行数
CONTAINERS = {
    "C01": ("袋", 3, "阳"), "C02": ("桶", 2, "阴"),
    "C03": ("罐", 3, "阳"), "C04": ("盆", 4, "阴"),
    "C05": ("盂", 2, "阳"), "C06": ("盎", 4, "阴"),
    "C10": ("袋", 3, "阳"), "C20": ("钵", 3, "阳"),
    "C21": ("盏", 2, "阴"), "C22": ("盒", 4, "阳"),
    "C23": ("杯", 3, "阴"), "C24": ("壶", 2, "阳"),
    "C25": ("瓶", 3, "阴"),
}

# ── 画素: 每个基本笔形 = 一组字符格 ──
#   8x6 字符位图, 直接可见
STROKES = {
    "竖": [
        "  █  ",
        "  █  ",
        "  █  ",
        "  █  ",
        "  █  ",
        "  █  ",
    ],
    "横": [
        "     ",
        "     ",
        "█████",
        "     ",
        "     ",
        "     ",
    ],
    "点": [
        "     ",
        "  █  ",
        "     ",
        "     ",
        "     ",
        "     ",
    ],
    "撇": [
        "  █  ",
        " █   ",
        "█    ",
        "     ",
        "     ",
        "     ",
    ],
    "捺": [
        "     ",
        "█    ",
        " █   ",
        "  █  ",
        "     ",
        "     ",
    ],
    "口": [
        "     ",
        "█████",
        "█   █",
        "█   █",
        "█████",
        "     ",
    ],
    "日": [
        "█████",
        "█   █",
        "█████",
        "█   █",
        "█████",
        "     ",
    ],
    "目": [
        "█████",
        "█   █",
        "█████",
        "█   █",
        "█████",
        "     ",
    ],
    "十": [
        "     ",
        "  █  ",
        "█████",
        "  █  ",
        "     ",
        "     ",
    ],
    "八": [
        "     ",
        "█   █",
        " █ █ ",
        "  █  ",
        "     ",
        "     ",
    ],
    "人": [
        "     ",
        "  █  ",
        " █ █ ",
        "█   █",
        "     ",
        "     ",
    ],
    "木": [
        "  █  ",
        "█████",
        "  █  ",
        " █ █ ",
        "█   █",
        "     ",
    ],
    "禾": [
        "  █  ",
        "█████",
        "  █  ",
        " █ █ ",
        "█   █",
        "     ",
    ],
    "火": [
        "  █  ",
        " █ █ ",
        "█ █ █",
        " █ █ ",
        "  █  ",
        "     ",
    ],
    "山": [
        "  █  ",
        " ███ ",
        "█████",
        " ███ ",
        "  █  ",
        "     ",
    ],
    "水": [
        "  █  ",
        " ███ ",
        "█████",
        "█████",
        " ███ ",
        "     ",
    ],
    "土": [
        "     ",
        "  █  ",
        "█████",
        "  █  ",
        "  █  ",
        "     ",
    ],
    "王": [
        "     ",
        "█████",
        "  █  ",
        "█████",
        "  █  ",
        "     ",
    ],
    "大": [
        "  █  ",
        "█████",
        "  █  ",
        " █ █ ",
        "█   █",
        "     ",
    ],
    "小": [
        "  █  ",
        " █ █ ",
        "█████",
        " █ █ ",
        "  █  ",
        "     ",
    ],
    "刀": [
        "   ██",
        "  █ █",
        "████ ",
        "  █  ",
        " █   ",
        "     ",
    ],
    "力": [
        "  ██ ",
        " █  █",
        "████ ",
        "  █  ",
        " █   ",
        "     ",
    ],
    "女": [
        "  █  ",
        "████ ",
        " █ █ ",
        "  █  ",
        " █ █ ",
        "     ",
    ],
    "子": [
        "     ",
        "█████",
        "  █  ",
        "█████",
        "  █  ",
        "     ",
    ],
    "馬": [
        "████ ",
        "█  █ ",
        "████ ",
        "█  █ ",
        "█  █ ",
        "     ",
    ],
    "鳥": [
        "████ ",
        "█  █ ",
        "████ ",
        "█  █ ",
        " █   ",
        "     ",
    ],
}

# ── 容器形状: 每个容器名 → 一组画素拼接 ──
CONT_SHAPE = {
    "袋": ["口", "人"],     # 外层口 + 内层人形
    "桶": ["八", "子"],     # 开张 + 子
    "罐": ["口", "木"],     # 方底 + 木枝
    "盆": ["八", "目"],     # 展开 + 目的纹路
    "盂": ["口", "十"],     # 方 + 十
    "盎": ["八", "日"],     # 展开 + 日
    "钵": ["口", "大"],     # 方 + 大
    "盏": ["口", "小"],     # 方 + 小
    "盒": ["八", "王"],     # 展 + 王
    "杯": ["口", "火"],     # 方 + 火
    "壶": ["口", "水"],     # 方 + 水
    "瓶": ["八", "刀"],     # 展 + 刀
}

# ── 字母到画素的映射 ──
LETTER_STROKE = {
    "a": ["撇", "口"],
    "b": ["竖", "口"],
    "c": ["撇", "捺"],
    "d": ["竖", "口"],
    "e": ["横", "撇"],
    "f": ["横", "竖"],
    "g": ["撇", "口"],
    "h": ["竖", "撇"],
    "i": ["竖", "点"],
    "j": ["竖", "撇"],
    "k": ["竖", "撇"],
    "l": ["竖", "横"],
    "m": ["竖", "撇"],
    "n": ["竖", "撇"],
    "o": ["口"],
    "p": ["竖", "口"],
    "q": ["撇", "口"],
    "r": ["竖", "撇"],
    "s": ["撇", "横"],
    "t": ["横", "竖"],
    "u": ["撇", "点"],
    "v": ["撇", "撇"],
    "w": ["撇", "撇"],
    "x": ["撇", "撇"],
    "y": ["撇", "竖"],
    "z": ["横", "撇"],
}

def draw(stroke_name):
    """返回一组字符行"""
    return STROKES.get(stroke_name, ["     "] * 6)

def fuse(how, a, b):
    """融合两个画素列表"""
    la, lb = draw(a), draw(b)
    if how == "LR":
        # 左右拼
        return [la[i] + lb[i] for i in range(6)]
    elif how == "TB":
        return la + lb
    elif how == "IO":
        # 里外: 外框 + 中间嵌入
        result = list(la)
        for i in range(6):
            if i >= 1 and i <= 4 and len(lb) > i:
                row = list(result[i])
                brow = lb[i]
                # 嵌入中心
                start = max(0, (len(row) - len(brow)) // 2)
                for j, ch in enumerate(brow):
                    if start + j < len(row) and ch == '█':
                        row[start + j] = ch
                result[i] = "".join(row)
        return result
    elif how == "SS":
        # 半包围: 外包 + 内下角入
        result = list(la)
        for i in range(3, 6):
            if i < len(lb):
                row = list(result[i])
                offset = 2
                for j, ch in enumerate(lb[i]):
                    if offset + j < len(row) and ch == '█':
                        row[offset + j] = ch
                result[i] = "".join(row)
        return result
    return [la[i] + lb[i] for i in range(6)]

def fuse_letters(mode, left, right):
    """融合两个字母"""
    ls = LETTER_STROKE.get(left, ["竖"])
    rs = LETTER_STROKE.get(right, ["竖"])
    # 取首画素融合
    return fuse(mode, ls[0], rs[0])

def draw_ligature(name):
    """画一个融合字"""
    mode = "LR"
    # 拆两字母
    if len(name) >= 2:
        left, right = name[0], name[1]
    else:
        left, right = name[0], "竖"
    lines = fuse_letters(mode, left, right)
    header = f"[{name}]"
    return [header] + [f"  {l}" for l in lines]

def show(container_code, row):
    """展示容器+行的内容"""
    c = CONTAINERS.get(container_code)
    if not c:
        return f"未知容器: {container_code}"
    name, cap, base_phase = c
    # 极性: 奇行=阳, 偶行=阴
    phase = "阳" if row % 2 == 1 else "阴"
    seg = ["尽", "起", "消"][row % 3]
    shape = CONT_SHAPE.get(name, ["口"])
    # 容器外框
    out = [f"┌── {name}-{row}-{phase}·{seg} (容={cap} 基={base_phase}) " + "─" * 20]
    # 画容器本身的字形
    for s in shape:
        for line in draw(s):
            out.append(f"│ {line}")
    out.append("└" + "─" * 40)
    return "\n".join(out)

if __name__ == "__main__":
    print("═══════════════════════════════════════════════════")
    print("  m_glyph — 直接造字器")
    print("  地址 = 画素, 读出即画")
    print("  身份落款: 刘楚恬和我")
    print("═══════════════════════════════════════════════════")

    if len(sys.argv) >= 2 and sys.argv[1].startswith("C"):
        # 查容器
        code = sys.argv[1].split("-")[0]
        row = int(sys.argv[1].split("-")[1]) if "-" in sys.argv[1] else 1
        print(show(code, row))
    elif len(sys.argv) >= 3 and sys.argv[1] in [v[0] for v in CONTAINERS.values()]:
        # 画指定容器+字母
        cname = sys.argv[1]
        letter = sys.argv[2]
        k = [k for k,v in CONTAINERS.items() if v[0]==cname][0]
        print(show(k, 1))
    else:
        # 跑全套示例
        print("\n【容器地址 → 直接画】")
        for code, row in [("C01",1),("C02",2),("C03",1),("C04",3)]:
            print(show(code, row))
            print()

        print("\n【融合字形 (两字母 → 合一)】")
        for name in ["an", "ch", "sh", "ou", "br", "th"]:
            lines = draw_ligature(name)
            for l in lines:
                print(l)
            print()

        print("\n═══════════════════════════════════════════════════")
        print("  不要 Python 脚本调用 fontTools")
        print("  直接一行命令 = 一个字形")
        print("  容器 = 画素, 地址 = 字, 读 = 用")
        print("═══════════════════════════════════════════════════")
