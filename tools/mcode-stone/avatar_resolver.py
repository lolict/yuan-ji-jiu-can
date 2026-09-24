#!/usr/bin/env python3
# 满全法 · 分身解析器 (Avatar Resolver)
# 功能：任意横版符号 → 韵母身份 → 五维地址 + COLOR42
#
# 架构：
#   输入：任意Unicode符号（横版/分身）
#   第1步：符号 → 韵母（分身→身份确定性映射）
#   第2步：韵母 + 声母 + 声调 → SI, YI, TI 三维索引
#   第3步：三维索引 → COLOR42（层+位）+ 微位置
#   输出：完整五维地址 (shengmu, tone, yunmu, color42, micro)

import json
import os
import sys

# ── 数据路径 ──
HERE = os.path.dirname(os.path.abspath(__file__))
YUNMU_JSON = os.path.join(HERE, "yunmu_heng.json")

# ── 声母48索引 ──
SHENGMU48 = [
    "b","p","m","f","d","t","n","l",
    "g","k","h","w","j","q","x","y",
    "z","c","s","r","ȥ","ç","ş","ŋ",
]
SHENGMU48_UPPER = [s.upper() for s in SHENGMU48]

# ── 声调60结构 ──
# 小写 0-29: 0-5阴平, 6-11阳平, 12-17上声, 18-23去声, 24-29轻声
# 大写 30-59: 同上
TONE_BASE = ["a","o","e","i","u","ü"]

def tone_index(symbol):
    """声调符号 → TI (0-59)，失败返回 -1"""
    # 建立映射：带调元音 → 索引
    idx = 0
    for tone_num in [1,2,3,4,0]:  # 阴平阳平上声去声轻声
        for vow in TONE_BASE:
            # 小写
            # tone_num: 1=ˉ, 2=ˊ, 3=ˇ, 4=ˋ, 0=无
            marked = _add_tone(vow, tone_num)
            if marked == symbol:
                return idx
            idx += 1
    # 大写 (30-59)
    idx = 30
    for tone_num in [1,2,3,4,0]:
        for vow in TONE_BASE:
            marked = _add_tone(vow.upper(), tone_num)
            if marked == symbol:
                return idx
            idx += 1
    return -1

def _add_tone(vowel, tone_num):
    """给元音加声调符号"""
    tone_marks = {
        'a': ['a','ā','á','ǎ','à'], 'o': ['o','ō','ó','ǒ','ò'],
        'e': ['e','ē','é','ě','è'], 'i': ['i','ī','í','ǐ','ì'],
        'u': ['u','ū','ú','ǔ','ù'], 'ü': ['ü','ǖ','ǘ','ǚ','ǜ'],
        'A': ['A','Ā','Á','Ǎ','À'], 'O': ['O','Ō','Ó','Ǒ','Ò'],
        'E': ['E','Ē','É','Ě','È'], 'I': ['I','Ī','Í','Ǐ','Ì'],
        'U': ['U','Ū','Ú','Ǔ','Ù'], 'Ü': ['Ü','Ǖ','Ǘ','Ǚ','Ǜ'],
    }
    if vowel in tone_marks:
        return tone_marks[vowel][tone_num]  # tone_num: 0=无声, 1-4=四声
    return vowel

def shengmu_index(symbol):
    """声母符号 → SI (0-47)"""
    if symbol in SHENGMU48:
        return SHENGMU48.index(symbol)
    if symbol in SHENGMU48_UPPER:
        return SHENGMU48_UPPER.index(symbol) + 24
    return -1

# ── COLOR42 计算 ──
# 来自 hunyuan-os/zig-stem/color_custom.c 的层+系统映射
LAYER_SYSTEM = ['red', 'yellow', 'green', 'blue', 'orange', 'dark', 'light', 'stone']
LAYER_NAMES = ['夫妻', '中心', '感知', '骨干', '圆融', '场', '高维', '玄']

# symbol_color_map 简化：64符号 → 42色
# 格式：{symbol_idx: (layer, position)}
# 这是从 color_custom.c 第82-158行提取的核心映射
SYMBOL_COLOR_MAP = {
    # 夫妻红层 (0-7)
    0:(0,0),1:(0,1),2:(0,2),3:(0,3),4:(0,4),5:(0,5),6:(0,6),7:(0,0),
    # 中心黄层 (8-15)
    8:(1,0),9:(1,1),10:(1,2),11:(1,3),12:(1,4),13:(1,5),14:(1,6),15:(1,0),
    # 感知绿层 (16-23)
    16:(2,0),17:(2,1),18:(2,2),19:(2,3),20:(2,4),21:(2,5),22:(2,6),23:(2,0),
    # 骨干蓝层 (24-31)
    24:(3,0),25:(3,1),26:(3,2),27:(3,3),28:(3,4),29:(3,5),30:(3,6),31:(3,0),
    # 圆融橙层 (32-39)
    32:(4,0),33:(4,1),34:(4,2),35:(4,3),36:(4,4),37:(4,5),38:(4,6),39:(4,0),
    # 场暗层 (40-47)
    40:(5,0),41:(5,1),42:(5,2),43:(5,3),44:(5,4),45:(5,5),46:(5,6),47:(5,0),
    # 高维光层 (48-55)
    48:(6,0),49:(6,1),50:(6,2),51:(6,3),52:(6,4),53:(6,5),54:(6,6),55:(6,0),
    # 混合层 (56-63)
    56:(3,2),57:(2,2),58:(6,3),59:(6,0),60:(0,3),61:(1,2),62:(0,0),63:(3,0),
}

# 42色名称（color_custom.c CN_NAME 映射）
COLOR_NAMES = [
    "妃","粉","彤","赤","棕","绛","赭",     # 红 R0-R6
    "黄","金黄","橙","昏","珀","蜡","晶",   # 黄 Y0-Y6
    "翠","草","竹","苍","苔","灰","霓",     # 绿 G0-G6
    "素","湛","海","冰","穹","靛","旖",     # 蓝 B0-B6
    "朱","橘","杏","丹","栗","褐","暝",     # 橙 O0-O6
    "墨","缁","黝","玄","黯","黧","黟",     # 暗 D0-D6
    "曦","皓","曜","晖","昙","皑","璟",     # 光 L0-L6
]

# ── 微位置10×10 ──
MICRO_GRID = 10  # 每维10格，共100微位置

# ================================================================
# AvatarResolver 主类
# ================================================================

class AvatarResolver:
    def __init__(self):
        with open(YUNMU_JSON, 'r', encoding='utf-8') as f:
            data = json.load(f)
        self.variants = data['variants']        # 韵母 → 分身序表
        self.avatar_index = data['avatar_index'] # 符号 → 韵母（反向索引）
        self.vertical = data['vertical']        # 韵母 → 竖版符号

    def resolve_yunmu(self, symbol):
        """
        分身 → 韵母身份
        任意横版符号，只要注册过分身，就确定性映射到同一韵母
        """
        return self.avatar_index.get(symbol, "")

    def get_yunmu_id(self, yunmu_name):
        """韵母 → YI (0-31)"""
        # 与 yunmu_heng.nim 的 YunmuId 一致
        order = [
            "ai","ei","ui","ao","ou","iu",
            "ie","üe","er",
            "an","en","in","un","ün",
            "ang","eng","ing","ong",
            "ia","iao","ua","uai","uo",
            "ian","uan","üan","iang","uang",
            "iong","duang","huang",
        ]
        if yunmu_name in order:
            return order.index(yunmu_name)
        return -1

    def get_variants(self, yunmu):
        """韵母 → 所有分身"""
        return self.variants.get(yunmu, [])

    def get_vertical(self, yunmu):
        """韵母 → 竖版（规范符号）"""
        return self.vertical.get(yunmu, "")

    def compute_color(self, si, yi, ti):
        """
        三维索引 → COLOR42 (0-41)
        哈希公式：(SI×17 + YI×7 + TI×3) % 42
        """
        h = (si * 17 + yi * 7 + ti * 3) % 42
        return h

    def compute_micro(self, si, yi, ti):
        """
        三维索引 → 微位置 (0-99)
        取另一组哈希避免与color冲突：(SI×7 + YI×13 + TI×23) % 100
        """
        h = (si * 7 + yi * 13 + ti * 23) % 100
        return h

    def resolve_syllable(self, shengmu_sym, tone_sym, yunmu_sym):
        """
        完整音节解析（核心API）
        输入：声母符号, 声调符号, 韵母横版符号
        输出：五维地址 + COLOR42 + 微位置
        """
        # 第1步：声母索引
        si = shengmu_index(shengmu_sym)
        if si < 0:
            return {"error": f"未知声母: {shengmu_sym} (U+{ord(shengmu_sym):04X})"}

        # 第2步：声调索引
        ti = tone_index(tone_sym)
        if ti < 0:
            return {"error": f"未知声调: {tone_sym} (U+{ord(tone_sym):04X})"}

        # 第3步：韵母横版 → 身份（分身解析！）
        yunmu = self.resolve_yunmu(yunmu_sym)
        if not yunmu:
            return {"error": f"未知韵母分身: {yunmu_sym} (U+{ord(yunmu_sym):04X})"}

        yi = self.get_yunmu_id(yunmu)

        # 第4步：COLOR42
        color_idx = self.compute_color(si, yi, ti)

        # 第5步：微位置
        micro = self.compute_micro(si, yi, ti)

        # 第6步：层+系统
        color_info = SYMBOL_COLOR_MAP.get(color_idx % 64, (0, 0))
        layer = color_info[0]
        pos = color_info[1]

        return {
            "syllable": f"{shengmu_sym}{yunmu_sym}",
            "shengmu": {"sym": shengmu_sym, "si": si},
            "tone": {"sym": tone_sym, "ti": ti},
            "yunmu": {
                "input": yunmu_sym,
                "identity": yunmu,
                "yi": yi,
                "vertical": self.get_vertical(yunmu),
                "avatar_count": self.get_variants(yunmu).__len__(),
            },
            "color42": color_idx,
            "color_name": COLOR_NAMES[color_idx] if color_idx < len(COLOR_NAMES) else "?",
            "layer": LAYER_NAMES[layer],
            "system": LAYER_SYSTEM[layer],
            "position": pos,
            "micro": micro,
            "full_address": (si, ti, yi, color_idx, micro),
        }

    def find_yunmu_for_color(self, color_idx):
        """反查：COLOR42 → 哪些韵母可能映射到此色"""
        results = []
        for yunmu, yi_raw in [(ym, self.get_yunmu_id(ym)) for ym in self.variants]:
            for si in range(48):
                for ti in range(60):
                    if self.compute_color(si, yi_raw, ti) == color_idx:
                        results.append((si, ti, yunmu))
                        if len(results) > 10:
                            return results
        return results

# ================================================================
# 测试
# ================================================================

if __name__ == "__main__":
    resolver = AvatarResolver()
    print("═══ 满全法 · 分身解析器 ═══")
    print(f"注册韵母: {len(resolver.variants)}")
    print(f"注册分身: {len(resolver.avatar_index)}")
    print()

    print("── 分身→身份 验证 ──")
    test_pairs = [
        ("ꬰ","ai"), ("ѧ","ai"), ("ɐ","ai"),
        ("ɘ","ei"), ("ə","ei"),
        ("ȣ","ou"), ("႘","ou"),
        ("ơ","uo"), ("ȫ","uo"),
        ("ѭ","iang"), ("ѩ","iang"),
        ("ꝡ","uai"), ("Ꝡ","uai"),
    ]
    for sym, expected in test_pairs:
        result = resolver.resolve_yunmu(sym)
        ok = "✓" if result == expected else f"✗ (got {result})"
        print(f"  {ok} {sym} (U+{ord(sym):04X}) → {result}")

    print()
    print("── 完整音节解析 ──")
    # 测试: b + 1声 + ai(横版Ѧ)
    r = resolver.resolve_syllable("b", "ā", "Ѧ")
    if "error" not in r:
        print(f"  b+ā+Ѧ(ai): color42={r['color42']}({r['color_name']}) layer={r['layer']} micro={r['micro']}")
        print(f"    韵母身份: {r['yunmu']['identity']} 竖版: {r['yunmu']['vertical']} ×{r['yunmu']['avatar_count']}分身")
    else:
        print(f"  错误: {r['error']}")

    print()
    print("── 跨韵母分身共享 ──")
    # ꙍ/Ꙍ 同时是 ou 和 uo 的分身
    for ym in ["ou", "uo"]:
        variants = resolver.get_variants(ym)
        shared = [v for v in variants if v in ['ꙍ','Ꙍ']]
        if shared:
            print(f"  {ym} 拥有共享分身: {shared}")

    print()
    print("── 竖版总览 ──")
    for ym in resolver.variants:
        v = resolver.get_vertical(ym)
        c = len(resolver.get_variants(ym))
        print(f"  {ym:6s} ⟶ {v} (×{c})")
