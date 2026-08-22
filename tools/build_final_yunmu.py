#!/usr/bin/env python3
"""
从三份附件 + 最终编码串 提取韵母定义，去重，补全66码位。
生成：final_yunmu_table.md（最终韵母编码表）
"""

import json

# ============================================================
# 1. 最终编码串（file3 中段，用户确认的权威版本）
# 格式：大写}小写}大写}小写}...
# ============================================================
FINAL_STRING = (
    "Ȍ}ȍ}Ꙙ}ꙙ}Ȑ}ȑ}Ӱ}ӱ}Ü}ü}"
    "Ѧ}ѧ}Ӗ}ӗ}Ə}ə}Ѽ}ѽ}Ѱ}ѱ}Ꜵ}ꜵ}Ȣ}ȣ}Ꙍ}ꙍ}"
    "Ⴘ}Ⴏ}Ⴎ}Ѥ}ѥ}Ё}ё}ᵫ}ꭂ}Є}є}"
    "Ꙗ}ꙗ}ⱄ}ⱅ}ѩ}Ǟ}ǟ}ꬰ}ꬱ}Ꜻ}ꜻ}Ȫ}ȫ}"
    "Ⴊ}Ⴆ}Ѿ}ѿ}ꭃ}ꭄ}Ꙙ}ꙙ}ɤ}ɣ}ꙝ}Ꙝ}Ⴄ}Ⴂ}Ѫ}ѫ}ꭌ}Ⴋ}Ⴉ}"
    "ӫ}Ӭ}Ꙟ}ꙟ}Ȏ}ȏ}Ⴓ}Ⴍ}ꭀ}ꭁ}ѭ}Ѭ}ꙛ}Ꙛ}Ⴔ}Ⴇ}Ȱ}ȱ}☁}Ѻ}ѻ"
)

# 解析为 (大写, 小写) 对
pairs_raw = FINAL_STRING.split("}")
assert len(pairs_raw) % 2 == 0, f"编码串长度不是偶数: {len(pairs_raw)}"
pairs = [(pairs_raw[i], pairs_raw[i+1]) for i in range(0, len(pairs_raw), 2)]
print(f"编码串共 {len(pairs)} 对符号")

# ============================================================
# 2. 编码串中每对对应的韵母（按 file3 定稿表）
# ============================================================
CODE_STR_YUNMU = [
    "o",    # Ȍ/ȍ
    "i",    # Ꙙ/ꙙ  (注意：这里 i 用 Ꙙ/ꙙ)
    "r",    # Ȑ/ȑ  (er 的 r 部分)
    "ü",    # Ӱ/ӱ
    "ü2",   # Ü/ü  (ü 的另一种写法)
    "ai",   # Ѧ/ѧ
    "ei",   # Ӗ/ӗ
    "ei2",  # Ə/ə  (ei 的另一种写法)
    "ui",   # Ѽ/ѽ
    "ui2",  # Ѱ/ѱ  (ui 的另一种写法)
    "ao",   # Ꜵ/ꜵ
    "ou",   # Ȣ/ȣ
    "ou2",  # Ꙍ/ꙍ  (ou 的另一种写法)
    "y",    # Ⴘ    (y 声母/半元音)
    "iu2",  # Ⴏ/Ⴎ  (iu 变体)
    "iu",   # Ⴞ/ꭎ  (iu)
    "ie",   # Ѥ/ѥ
    "üe",   # Ё/ё
    "üe2",  # ᵫ/ꭂ  (üe 变体)
    "er",   # Є/є
    "ia",   # Ꙗ/ꙗ
    "en",   # ⱄ/ⱅ
    "iao",  # ѩ    (iao 的简化)
    "ua",   # Ǟ/ǟ
    "ai2",  # ꬰ/ꬱ  (ai 变体)
    "ai3",  # ꬰ/ꬱ  (ai 变体, 实际与 ai2 同)
    "uai",  # Ꜻ/ꜻ
    "uo",   # Ȫ/ȫ
    "ang",  # Ⴊ/Ⴆ
    "ui3",  # Ѿ/ѿ  (ui 变体)
    "uo2",  # ꭃ/ꭄ  (uo 变体)
    "an",   # Ꙙ/ꙙ  (an 与 i 共用符号对，但位置不同)
    "ie2",  # ɤ/ɣ  (ie 变体)
    "ian",  # ꙝ/Ꙝ
    "uan",  # Ⴄ/Ⴂ
    "üan",  # Ѫ/ѫ
    "uan2", # ꭌ    (uan 变体)
    "duang",# Ⴋ    (duang)
    "üan2", # Ⴉ    (üan 变体)
    "eng",  # ӫ/Ӭ
    "ing",  # Ꙟ/ꙟ
    "ong",  # Ȏ/ȏ
    "ong2", # Ⴓ/Ⴍ  (ong 变体)
    "ong3", # ꭀ/ꭁ  (ong 变体)
    "iang", # ѭ/Ѭ
    "uang", # ꙛ/Ꙛ
    "qp",   # Ⴔ    (qp 组合)
    "AI",   # Ⴇ    (AI 特写)
    "iong", # Ȱ/ȱ
    "iong2",# ☁    (iong 变体)
    "jiong",# Ѻ/ѻ
]

# ============================================================
# 3. 从三份附件提取的韵母定义（合并去重）
# ============================================================

# 3.1 file1 提取（粤语 + 普通话变体）
file1_yunmu = {
    # 普通话韵母（file1 第一段）
    "er": ["ə"],
    "ai": ["æ"],
    "ei": ["ɘ"],
    "ao": ["ɒ"],
    "ou": ["ɵ"],
    "ia": ["ѧ"],
    "ua": ["ʉ"],
    "iao": ["ᴥ"],
    "iou": ["ø"],
    "in": ["ɤ"],
    "uo": ["œ"],
    "uai": ["ɞ"],
    "uei": ["ʚ"],
    "ue": ["ʊ"],
    "an": ["ã"],
    "en": ["ẽ"],
    "vn": ["ʯ"],
    "ang": ["ą"],
    "eng": ["ę"],
    "ing": ["į"],
    "ong": ["ɷ"],
    "ian": ["ʌ"],
    "uan": ["๑"],
    "üan": ["ʮ"],
    "uang": ["ѽ"],
    "ueng": ["ɔ"],
    "iang": ["ѫ"],
    "iong": ["ѻ"],
    "ê": ["ɝ"],
    "iai": ["ɚ"],
    # 粤语韵母（最终定稿）
    "oi_yy": ["ʝ"],
    "eu_yy": ["ʅ"],
    "am": ["ɲ"],
    "im": ["ɳ"],
    "ap": ["ʕ"],
    "ip": ["ɸ"],
    "at": ["ʡ"],
    "ot": ["ʢ"],
    "ak": ["ʞ"],
    "ok": ["ʠ"],
}

# 3.2 file2 提取（DeepSeek 对话中的定稿表）
file2_yunmu = {
    "o": ["Ȍ", "ȍ"],
    "e": ["ӗ", "Ӗ"],
    "i": ["႞", "ɨ"],
    "u": ["Ʊ", "ʊ"],
    "ü": ["Ӱ", "ӱ", "Ü", "ü"],
    "ai": ["Ѧ", "ѧ", "Ɐ", "ɐ", "𐆗", "Ꜽ", "ꜽ", "ꬰ", "ꬱ"],
    "ei": ["Ə", "ə", "Ӭ", "ӭ"],
    "ui": ["Ѽ", "ѽ", "Ѱ", "ѱ", "Ѿ", "ѿ"],
    "ao": ["Ꜵ", "ꜵ"],
    "ou": ["Ȣ", "ȣ", "႘", "ꙕ", "Ꙕ"],
    "iu": ["Ⴞ", "ꭎ", "Ⴏ", "Ⴎ"],
    "ie": ["ɤ", "ɣ", "Ꙋ", "Ɣ", "Ѥ", "ѥ", "ɬ", "ꝷ"],
    "üe": ["Ё", "ё", "ᵫ", "ꭂ", "ʊ̈"],
    "er": ["ɛ", "Ɛ", "Ʝ", "ɘ", "Ǝ", "ꞟ", "Ꞟ"],
    "an": ["Ꙙ", "ꙙ", "Ã", "ã"],
    "en": ["ⱄ", "ⱅ", "ⱺ", "ꬷ", "ꬹ", "ꬸ", "ꬾ", "ꬳ", "ꬲ"],
    "in": ["ⴶ", "ꬽ", "ꭅ", "Ⰰ", "ꞃ", "Ꞃ", "ⲗ", "Ⲗ", "ꙉ", "Ꙉ", "ႁ", "႔"],
    "un": ["տ", "໙", "ꦮ", "ꦱ", "ꦯ"],
    "ün": ["Ⴐ"],
    "ang": ["Ⴊ", "Ⴆ", "ᶏ", "ᶐ"],
    "eng": ["Ӫ", "ӫ", "ᶒ"],
    "ing": ["Ꙟ", "ꙟ", "ꙗ"],
    "ong": ["Ȏ", "ȏ", "Ⴓ", "Ⴍ", "ꭀ", "ꭁ", "Ǫ", "ǫ"],
    "ia": ["Ꙗ", "ꙗ"],
    "ian": ["Ꙝ", "ꙝ"],
    "iang": ["ѭ", "Ѭ", "Ѩ", "ѩ"],
    "iong": ["Ȱ", "ȱ", "Ѻ", "ѻ"],
    "ua": ["Ǟ", "ǟ", "ꭏ", "ᰝ", "ᳫ", "ᳪ", "ʉ", "Ʉ"],
    "uo": ["Ȫ", "ȫ", "Ⱋ", "Ⰲ", "Ꜿ", "ꜿ", "Ꙍ", "ꙍ"],
    "uai": ["Ꜻ", "ꜻ", "ꬰ", "ꬱ", "Ⱋ", "ႀ"],
    "uan": ["Ⴄ", "Ⴂ", "ꭌ", "ꦰ", "ꦲ", "ꭏ", "ꭋ"],
    "uang": ["ꙛ", "Ꙛ", "ⱄ", "ⱅ"],
    "üan": ["ѫ", "Ѫ", "ⴉ", "Ⴉ"],
    "ueng": ["ɝ", "ɞ"],
    "duang": ["Ⴋ"],
    "huang": ["Ⴌ"],
}

# 3.3 file3 提取（最终定稿表）
file3_yunmu = {
    "o": ["Ȍ", "ȍ"],
    "e": ["ӗ", "Ӗ"],
    "i": ["႞"],
    "u": ["Ʊ", "ʊ"],
    "ü": ["Ӱ", "ӱ", "Ü", "ü"],
    "ai": ["Ѧ", "ѧ", "Ɐ", "ɐ", "𐆗", "Ꜽ", "ꜽ", "ꬰ", "ꬱ"],
    "ei": ["Ə", "ə", "Ӭ", "ӭ"],
    "ui": ["Ѽ", "ѽ", "Ѱ", "ѱ", "Ѿ", "ѿ"],
    "ao": ["Ꜵ", "ꜵ"],
    "ou": ["Ȣ", "ȣ", "႘", "ꙕ", "Ꙕ"],
    "iu": ["Ⴞ", "ꭎ", "Ⴏ", "Ⴎ"],
    "ie": ["ɤ", "ɣ", "Ꙋ", "Ɣ", "Ѥ", "ѥ"],
    "üe": ["Ё", "ё", "ᵫ", "ꭂ"],
    "er": ["ɛ", "Ɛ", "Ʝ"],
    "an": ["Ꙙ", "ꙙ"],
    "en": ["ⱄ", "ⱅ"],
    "in": ["ĩ", "Ĩ", "Ⰰ", "ꭅ", "ꞃ", "Ꞃ"],
    "un": ["տ", "໙", "ꦮ", "ꦱ"],
    "ün": ["Ⴐ"],
    "ang": ["Ⴊ", "Ⴆ"],
    "eng": ["Ӫ", "ӫ"],
    "ing": ["Ꙟ", "ꙟ"],
    "ong": ["Ȏ", "ȏ", "Ⴓ", "Ⴍ", "ꭀ", "ꭁ", "Ǫ", "ǫ"],
    "ia": ["Ꙗ", "ꙗ"],
    "ian": ["Ꙝ", "ꙝ"],
    "iang": ["ѭ", "Ѭ", "Ѩ", "ѩ"],
    "iong": ["Ȱ", "ȱ"],
    "ua": ["Ǟ", "ǟ", "ᰝ", "ᳫ", "ᳪ", "ʉ", "Ʉ"],
    "uo": ["Ȫ", "ȫ", "Ꙍ", "ꙍ", "ꭃ", "ꭄ", "Ꜿ", "ꜿ"],
    "uai": ["Ꜻ", "ꜻ", "ꬰ", "ꬱ"],
    "uan": ["Ⴄ", "Ⴂ", "Ⴋ", "ꭌ", "ꭋ", "ꭏ"],
    "uang": ["ꙛ", "Ꙛ"],
    "ueng": ["ɝ", "ɞ"],
    "üan": ["ѫ", "Ѫ", "Ⴉ"],
    "duang": ["Ⴋ"],
    "huang": ["Ⴌ"],
}

# ============================================================
# 4. 合并策略：file3 > file2 > file1（优先级递减）
#    同一韵母，取最高优先级的定义
# ============================================================
merged = {}
for src_name, src_dict in [("file1", file1_yunmu), ("file2", file2_yunmu), ("file3", file3_yunmu)]:
    for yunmu, symbols in src_dict.items():
        if yunmu not in merged:
            merged[yunmu] = {"symbols": list(symbols), "source": src_name}
        else:
            # 高优先级覆盖
            if src_name == "file3":
                merged[yunmu] = {"symbols": list(symbols), "source": src_name}
            elif src_name == "file2" and merged[yunmu]["source"] != "file3":
                merged[yunmu] = {"symbols": list(symbols), "source": src_name}
            # file1 最低，只在没有时添加

print(f"\n合并后共 {len(merged)} 个韵母定义：")
for y, info in sorted(merged.items()):
    print(f"  {y:10s} → {info['symbols']}  (来源: {info['source']})")

# ============================================================
# 5. 检测冲突：同一符号被多个韵母使用
# ============================================================
sym_to_yunmu = {}
for yunmu, info in merged.items():
    for sym in info["symbols"]:
        if sym not in sym_to_yunmu:
            sym_to_yunmu[sym] = []
        if yunmu not in sym_to_yunmu[sym]:
            sym_to_yunmu[sym].append(yunmu)

conflicts = {s: ym for s, ym in sym_to_yunmu.items() if len(ym) > 1}
print(f"\n冲突检测：{len(conflicts)} 个符号被多个韵母使用")
for sym, yunmus in sorted(conflicts.items()):
    codepoints = " ".join(f"U+{ord(c):04X}" for c in sym)
    print(f"  {sym} ({codepoints}) → {yunmus}")

# ============================================================
# 6. 生成最终 66 码位韵母表
#    按标准汉语拼音韵母顺序排列
# ============================================================

# 标准韵母顺序（39个普通话韵母 + 粤语10个 + 特殊组合）
STANDARD_ORDER = [
    # 单韵母 (6)
    "a", "o", "e", "i", "u", "ü",
    # 复韵母 (13)
    "ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er",
    "ia", "iao", "ua", "uo", "uai", "uei",
    # 前鼻韵母 (8)
    "an", "en", "in", "un", "ün", "ian", "uan", "üan",
    # 后鼻韵母 (8)
    "ang", "eng", "ing", "ong", "iang", "uang", "ueng", "iong",
    # 特殊 (4)
    "ê", "iai", "duang", "huang",
    # 粤语韵母 (10)
    "oi_yy", "eu_yy", "am", "im", "ap", "ip", "at", "ot", "ak", "ok",
    # 其他组合（补足到66）
    "io", "ue2", "üeng", "io2", "iou", "uei2", "vn", "ong2", "ing2", "eng2",
    "ang2", "ueng2", "iao2", "uan2", "üan2", "iang2", "uang2", "iong2", "ai2", "ei2",
]

# 构建最终表
final_table = []
used_symbols = set()

for idx, yunmu in enumerate(STANDARD_ORDER, 1):
    if yunmu in merged:
        info = merged[yunmu]
        symbols = info["symbols"]
        # 取第一个作为主符号
        primary = symbols[0]
        alt = symbols[1:] if len(symbols) > 1 else []
        final_table.append({
            "code": f"F{idx:02d}",
            "yunmu": yunmu,
            "primary": primary,
            "alternate": alt,
            "source": info["source"],
        })
        used_symbols.update(symbols)
    else:
        # 未定义，留空待补
        final_table.append({
            "code": f"F{idx:02d}",
            "yunmu": yunmu,
            "primary": "—",
            "alternate": [],
            "source": "待补",
        })

# 输出
print(f"\n最终韵母表（{len(final_table)} 位）：")
for entry in final_table:
    alt_str = f" (异体: {entry['alternate']})" if entry['alternate'] else ""
    print(f"  {entry['code']} {entry['yunmu']:8s} → {entry['primary']}{alt_str} [{entry['source']}]")

# 保存
with open("/tmp/final_yunmu_table.json", "w", encoding="utf-8") as f:
    json.dump(final_table, f, ensure_ascii=False, indent=2)

print(f"\n已保存到 /tmp/final_yunmu_table.json")
print(f"已用符号数: {len(used_symbols)}")
