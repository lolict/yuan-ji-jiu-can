#!/usr/bin/env python3
# ============================================================
# 满全法 · 字体自带索引层
#
# 目标: 让字体文件能按"容器地址 / 拼音 / 结构"三种方式查到字形
#       不需要外部数据库, 字体打开时索引已在内部
#
# 三种查询路径:
#   A. 容器地址 → 字形 (结构即索引, 直接查)
#   B. 拼音     → 容器地址 → 字形 (拼音做入口, 容器做路由)
#   C. 结构指纹 → 字形 (统计消耗特征反查)
#
# 身份落款：刘楚恬和我
# ============================================================

import sys, os

# ============================================================
# 拼音路由表: pinyin → 融合字条目
#   每个融合字 = 结构 + 模式, 反向注册它的拼音
# ============================================================

# 拼音 → [融合名, 拼音全拼, 模式]
PINYIN_INDEX = {
    # 纯手工 30 (hand-tuned)
    "an":  ["an",  "an",  "LR"], "en":  ["en",  "en",  "LR"],
    "in":  ["in",  "in",  "LR"], "un":  ["un",  "un",  "LR"],
    "vn":  ["vn",  "ün",  "LR"],
    "ai":  ["ai",  "ai",  "LR"], "ei":  ["ei",  "ei",  "LR"],
    "ao":  ["ao",  "ao",  "LR"], "ou":  ["ou",  "ou",  "SS"],
    "ie":  ["ie",  "ie",  "TB"], "uo":  ["uo",  "uo",  "TB"],
    "io":  ["io",  "io",  "TB"],
    "ue":  ["ue",  "üe",  "LR"], "ui":  ["ui",  "ui",  "LR"],
    "iu":  ["iu",  "iu",  "LR"],
    "eo":  ["eo",  "eo",  "IO"], "oe":  ["oe",  "oe",  "IO"],
    "eu":  ["eu",  "eu",  "IO"],
    "ang": ["ang", "ang", "LR"], "eng": ["eng", "eng", "LR"],
    "ing": ["ing", "ing", "LR"], "ong": ["ong", "ong", "LR"],
    "ian": ["ian", "ian", "LR"], "uan": ["uan", "uan", "TB"],
    "iao": ["iao", "iao", "TB"], "uai": ["uai", "uai", "TB"],
    "iong":["iong","iong","IO"],
    "ch":  ["ch",  "ch",  "LR"], "sh":  ["sh",  "sh",  "LR"],
    "zh":  ["zh",  "zh",  "LR"],
}

# 扩展: 自动融合字也注册拼音
EXTENDED_PINYIN = {
    # 粤语
    "aa":"aa",  "oo":"oo",  "ii":"ii",  "uu":"uu",  "ee":"ee",  "vv":"vv",
    # 辅音缀
    "th":"th",  "gh":"gh",  "fh":"fh",  "ph":"ph",  "ng":"ng",
    "nk":"nk",  "nt":"nt",  "nd":"nd",
    # 后缀
    "ed":"ed",  "ad":"ad",  "uk":"uk",  "ok":"ok",  "ek":"ek",  "ep":"ep",
    # r-变音
    "ar":"ar",  "er":"er",  "ir":"ir",  "or":"or",  "ur":"ur",  "yr":"yr",
    "pr":"pr",  "tr":"tr",  "kr":"kr",  "br":"br",  "dr":"dr",  "sr":"sr",
    "fr":"fr",  "gr":"gr",
    # 其他辅音丛
    "fl":"fl",  "gl":"gl",  "pl":"pl",  "bl":"bl",  "cl":"cl",
    "sk":"sk",  "sp":"sp",  "st":"st",  "sm":"sm",  "sn":"sn",  "sw":"sw",
    "tw":"tw",  "dw":"dw",  "kw":"kw",  "qu":"qu",
    # 鼻化韵
    "am":"am",  "im":"im",  "om":"om",  "em":"em",  "on":"on",
    # 双元音
    "ae":"ae",  "ua":"ua",  "ia":"ia",  "ve":"ve",  "uei":"uei",  "iou":"iou",
    # 三字母
    "iang":"iang", "uang":"uang", "ueng":"ueng",
}

# 合并
for pinyin, name in EXTENDED_PINYIN.items():
    if pinyin not in PINYIN_INDEX:
        PINYIN_INDEX[pinyin] = [name, pinyin, "LR"]

# ============================================================
# 容器路由: 声母/韵母字母 → 容器类型
#   决定了这个符号在寻址表中属于哪个横档
# ============================================================

INITIAL_TO_CONTAINER = {
    "b":"袋",  "p":"袋",  "m":"袋",  "f":"袋",
    "d":"桶",  "t":"桶",  "n":"桶",  "l":"桶",
    "g":"罐",  "k":"罐",  "h":"罐",
    "j":"盆",  "q":"盆",  "x":"盆",
    "z":"盂",  "c":"盂",  "s":"盂",  "r":"盂",
    "y":"盎",  "w":"盎",  "v":"盎",
    # 声母复合
    "zh":"盂","ch":"盂","sh":"盂",
}

VOWEL_TO_CONTAINER = {
    "a":"钵",  "e":"盏",  "i":"盒",
    "o":"杯",  "u":"壶",  "v":"瓶",  # v = ü
}

# ============================================================
# 反查: 容器地址 (结构指纹) → 可能的字形
# ============================================================

def address_lookup(container_code: str, row: int, phase: str, segment: str) -> dict:
    """
    给一个三维田字格地址, 反查它对应的字形信息。
    不依赖拼音 — 纯靠容器容量 + 行号 + 极层 + 段位
    """
    from radix_3d_tianzi import CONTAINERS, tape_row
    c = CONTAINERS.get(container_code)
    if not c:
        return {"error": f"unknown container {container_code}"}

    suffix = f"{c.name}-{row}-{phase}·{segment}"
    return {
        "container_code": container_code,
        "container_name": c.name,
        "capacity": c.capacity,
        "phase": c.phase,
        "row": row,
        "requested_phase": phase,
        "segment": segment,
        "suffix": suffix,
        "suffix_code": f"{container_code}-{row}-{'0' if phase=='阳' else '1'}-{'0' if segment=='起' else ('2' if segment=='尽' else '1')}",
        "match_type": "structure_index",
        "tape": tape_row(container_code, row),
    }

# ============================================================
# 拼音查字: pinyin → 字形 → 容器地址
# ============================================================

def pinyin_lookup(pinyin: str) -> dict:
    """
    查拼音 → 字形 → 所属的容器地址
    """
    entry = PINYIN_INDEX.get(pinyin)
    if not entry:
        return {"error": f"pinyin '{pinyin}' not indexed", "match_type": "none"}

    name = entry[0]
    full_pinyin = entry[1]
    mode = entry[2]

    # 拆分声母/韵母 → 路由到容器横档
    initial = None
    for init in ["zh","ch","sh","b","p","m","f","d","t","n","l","g","k","h","j","q","x","z","c","s","r","y","w"]:
        if name.startswith(init):
            initial = init
            break

    return {
        "pinyin": full_pinyin,
        "fusion": name,
        "mode": mode,
        "initial": initial,
        "initial_container": INITIAL_TO_CONTAINER.get(initial, "?"),
        "match_type": "pinyin_index",
    }

# ============================================================
# 结构指纹查字: 统计字形容貌 → 反推容器地址
# ============================================================

def structure_lookup(stroke_count: int, has_closed_loop: bool, has_vertical: bool, has_horizontal: bool) -> dict:
    """
    通过字形特征反推容器:
      总笔画数 → 匹配容量接近的容器
      闭合结构 (a/o/d/p/q/g) → 韵母群 (包围型)
      开放结构 (i/l/f/t) → 声母群 (外向型)
      竖笔 → 桶档 (纵向容器)
      横笔 → 袋档 (横向容器)
    """
    # 容器容量表 (内嵌, 不依赖外部导入)
    _caps = [
        ("C01","袋",3,"阳"), ("C02","桶",2,"阴"), ("C03","罐",3,"阳"), ("C04","盆",4,"阴"),
        ("C05","盂",2,"阳"), ("C06","盎",4,"阴"), ("C07","钵",3,"阳"), ("C08","盏",2,"阴"),
        ("C09","盒",4,"阳"), ("C0A","杯",3,"阴"), ("C0B","壶",2,"阳"), ("C0C","瓶",3,"阴"),
        ("C10","袋",3,"阳"), ("C11","桶",2,"阴"), ("C12","罐",3,"阳"), ("C13","盆",4,"阴"),
        ("C14","盂",2,"阳"), ("C15","盎",4,"阴"), ("C16","钵",3,"阳"), ("C17","盏",2,"阴"),
        ("C18","盒",4,"阳"), ("C19","杯",3,"阴"),
        ("C20","钵",3,"阳"), ("C21","盏",2,"阴"), ("C22","盒",4,"阳"), ("C23","杯",3,"阴"),
        ("C24","壶",2,"阳"), ("C25","瓶",3,"阴"),
    ]
    caps = [(code, cap, name, phase) for code, name, cap, phase in _caps]
    # 找容量最接近的
    best = min(caps, key=lambda x: abs(x[1] - stroke_count))

    role_hint = "声母" if (has_closed_loop is False and has_vertical) else "韵母"
    phase_hint = "阳" if has_vertical else "阴"

    return {
        "stroke_count": stroke_count,
        "matched_container": best[2],
        "capacity_diff": abs(best[1] - stroke_count),
        "best_code": best[0],
        "role_hint": role_hint,
        "phase_hint": phase_hint,
        "match_type": "structure_heuristic",
    }

# ============================================================
# GSUB Feature 建议: 如何把索引编译进字体
# ============================================================

def build_gsub_suggestion() -> dict:
    """
    返回写入 OpenType GSUB 表的建议规则
    """
    rules = {}
    for pinyin, entry in PINYIN_INDEX.items():
        name = entry[0]
        # GSUB 'liga' 标准连系替换: 输入连写 → 替换为融合字形
        # 例: 用户输入 "a" + "n" → GSUB 替换为 lig.an 字形
        components = list(pinyin) if len(pinyin) <= 3 else [pinyin]
        rules[name] = {
            "feature": "liga",
            "input_sequence": components,
            "output_glyph": f"lig.{name}",
            "description": f"{pinyin} → {name} [{entry[2]}]",
        }
    return rules

# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("═══════════════════════════════════════════════════")
    print("  满全法 · 字体自带索引层")
    print("  三种寻址: 容器地址 · 拼音 · 结构指纹")
    print("  身份落款: 刘楚恬和我")
    print("═══════════════════════════════════════════════════")

    print("\n【A. 容器地址寻址】")
    for code, row, phase, seg in [("C03", 1, "阳", "消"), ("C04", 2, "阴", "尽"), ("C02", 5, "阳", "起")]:
        r = address_lookup(code, row, phase, seg)
        print(f"  {r['suffix']:25s} → code={r['suffix_code']} cap={r['capacity']}")

    print("\n【B. 拼音寻址】")
    for py in ["an", "ong", "ch", "ar", "oo", "ng", "iang"]:
        r = pinyin_lookup(py)
        if "error" not in r:
            print(f"  {py:8s} → {r['fusion']:10s} {r['mode']} 声母:{r['initial'] or '-':3s} 档:{r['initial_container']}")
        else:
            print(f"  {py:8s} → {r['error']}")

    print("\n【C. 结构指纹寻址】")
    for strokes, loop, vert, horiz in [(2, True, True, False), (3, False, True, True), (1, False, False, True)]:
        r = structure_lookup(strokes, loop, vert, horiz)
        print(f"  笔数={strokes} 闭合={loop} 竖={vert} 横={horiz} → {r['matched_container']} (差{r['capacity_diff']}) 推测={r['role_hint']}")

    gsub = build_gsub_suggestion()
    print("\n【D. GSUB 建议规则 (前 8 条示例)】")
    for i, (name, rule) in enumerate(list(gsub.items())[:8]):
        print(f"  {rule['description']:30s} input={rule['input_sequence']} → {rule['output_glyph']}")
    print(f"  ... 共 {len(gsub)} 条")

    print("\n═══════════════════════════════════════════════════")
    print("  自带索引层: 字体打开时即携带")
    print("  · 查容器地址: 直接结构反查")
    print("  · 查拼音: PINYIN_INDEX 路由到容器档")
    print("  · 查结构: 笔画特征 → 容量匹配")
    print("  · GSUB: a+n 输入自动变 an 融合字形")
    print("═══════════════════════════════════════════════════")
