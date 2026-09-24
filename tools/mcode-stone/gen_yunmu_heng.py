#!/usr/bin/env python3
# 满全法 · 韵母横版注册生成器
# 从原始数据生成 yunmu_heng.json （权威分身注册表）
# 再从JSON生成 yunmu_heng.nim （Nim原生格式）
#
# 权威数据源：主人的DeepSeek参考链接 + yunmu66.nim竖版

import json
import os

# ================================================================
# 横版分身数据（主人手书，一字不改）
# 结构：{韵母: [分身1, 分身2, ...]}
# 每列表第一项 = 竖版（规范选择），其余 = 分身
# ================================================================

VARIANTS = {
    # ── 基础复韵母 ──
    "ai": ["ꬰ", "ꬱ", "Ѧ", "ѧ", "Ⴇ", "∀", "ɐ", "Ɐ", "ꜽ", "Ꜽ"],
    "ei": ["ɘ", "Ǝ", "ə", "Ə", "ӭ", "Ӭ"],
    "ui": ["ѱ", "Ѱ", "Ⴤ", "ⴤ", "ѽ", "Ѽ", "ѿ", "Ѿ"],
    "ao": ["ꜵ", "Ꜵ", "ɒ", "Ɒ"],
    "ou": ["ȣ", "Ȣ", "ꙍ", "Ꙍ", "႘"],
    "iu": ["ꭎ", "Ⴞ", "ⴞ", "⊱", "⊰"],

    # ── 卷舌/特殊 ──
    "ie": ["ɤ", "Ɣ", "ɣ", "Ꙋ", "ѥ", "Ѥ"],
    "üe": ["ᵫ", "ꭂ", "ⱷ"],
    "er": ["ɛ", "Ɛ", "є", "Є", "ɘ", "Ǝ"],

    # ── 前鼻音 ──
    "an": ["ꙙ", "Ꙙ", "ã", "Ã"],
    "en": ["ⱄ", "Ⱄ", "ⱅ", "Ⱅ", "ⰶ", "Ⰶ", "ẽ", "Ẽ"],
    "in": ["ꞃ", "Ꞃ", "ꙉ", "Ꙉ", "ꭅ", "ႁ", "ⲗ", "Ⲗ", "ĩ", "Ĩ"],
    "un": ["տ", "໙", "ꦮ", "ꦱ", "Ⴐ", "ⴐ"],
    "ün": ["ỹ", "Ỹ", "ꞟ", "Ꞟ", "ઇ", "ଓ"],

    # ── 后鼻音 ──
    "ang": ["Ⴆ", "Ⴊ", "Ⴉ", "ⴉ", "ᶏ", "ᶐ"],
    "eng": ["ӫ", "Ӫ", "Ⱞ", "ⱞ", "ᶒ", "ᶓ"],
    "ing": ["ꙟ", "Ꙟ", "ꞃ", "Ꞃ"],
    "ong": ["ꭁ", "ꭀ", "Ⴍ", "Ⴓ", "ǫ", "Ǫ", "ȏ", "Ȏ"],

    # ── ia/ua组 ──
    "ia": ["ꙗ", "Ꙗ"],
    "iao": ["ᧀ", "ᦿ", "ꮂ", "ᴥ", "ᦾ"],
    "ua": ["ã", "Ã", "ʉ", "Ʉ", "ᳫ", "ᳪ", "ᰝ", "ꭏ", "ǟ", "Ǟ"],
    "uai": ["ꜻ", "Ꜻ", "Ꜽ", "ꜽ", "ɝ", "ꝡ", "Ꝡ"],
    "uo": ["ơ", "Ơ", "ꭃ", "ꭄ", "ꜿ", "Ꜿ", "ȫ", "Ȫ", "ꙍ", "Ꙍ"],

    # ── ian/uan组 ──
    "ian": ["ꙝ", "Ꙝ"],
    "uan": ["Ⴄ", "Ⴂ", "Ⴆ", "Ⴊ", "ꭋ", "ꭌ", "ꦰ", "ꦲ"],
    "üan": ["ѫ", "Ѫ", "ʚ", "ɞ"],
    "iang": ["ѭ", "Ѭ", "ѩ", "Ѩ"],
    "uang": ["ꙛ", "Ꙛ", "๓", "ღ"],
    "iong": ["ȱ", "Ȱ", "ю", "Ю", "ѻ", "Ѻ"],

    # ── 特殊补录 ──
    "duang": ["ⴋ", "Ⴋ"],
    "huang": ["ⴌ", "Ⴌ"],
}

# ================================================================
# 生成权威JSON
# ================================================================

def gen_json():
    # 1. 原始变体数据
    data = {
        "meta": {
            "system": "满全法·韵母横版注册",
            "version": "1.0",
            "architecture": "横版=多分身共存, 竖版=规范选择, 分身=同身份替身",
            "principle": "任意横版符号→同一韵母身份→同一地址, 不否定不覆盖",
        },
        "variants": VARIANTS,
    }

    # 2. 反向索引：符号 → 韵母
    symbol_to_yunmu = {}
    for ym, syms in VARIANTS.items():
        for s in syms:
            # 若符号已成注册：首归属优先（孙悟空私有权）
            if s not in symbol_to_yunmu:
                symbol_to_yunmu[s] = ym
    data["avatar_index"] = symbol_to_yunmu

    # 3. 竖版速查：韵母 → 规范符号
    vertical = {ym: syms[0] for ym, syms in VARIANTS.items()}
    data["vertical"] = vertical

    # 4. 统计
    data["stats"] = {
        "yunmu_count": len(VARIANTS),
        "total_avatars": sum(len(s) for s in VARIANTS.values()),
        "unique_symbols": len(symbol_to_yunmu),
    }

    out = os.path.join(os.path.dirname(__file__), "yunmu_heng.json")
    with open(out, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print(f"✓ 生成 {out}")
    print(f"  韵母: {data['stats']['yunmu_count']}")
    print(f"  分身: {data['stats']['total_avatars']}")
    print(f"  唯一符号: {data['stats']['unique_symbols']}")

# ================================================================
# 生成Nim文件
# ================================================================

def gen_nim():
    out = os.path.join(os.path.dirname(__file__), "yunmu_heng.nim")

    lines = []
    lines.append("# 满全法 · 韵母横版注册 (自动生成，勿手改)")
    lines.append(f"# 韵母数: {len(VARIANTS)}, 分身总数: {sum(len(s) for s in VARIANTS.values())}")
    lines.append("# 架构: 横版=多分身共存, 竖版=规范选择, 分身=同身份替身")
    lines.append("")
    lines.append("import tables")
    lines.append("")

    # ── TYPES ──
    lines.append("type")
    lines.append("  AvatarIndex* = Table[string, string]")
    lines.append("")

    # ── CONSTANTS (UTF-8原字，Nim支持源文件UTF-8) ──
    for ym, syms in VARIANTS.items():
        nim_syms = ", ".join(f'"{s}"' for s in syms)
        lines.append(f"# {ym}")
        lines.append(f"const YV_{ym.upper().replace('Ü','U')}* = @[{nim_syms}]")
        lines.append("")

    # ── MASTER TABLE ──
    lines.append("# 韵母 → 分身序表")
    lines.append("const YUNMU_VARIANTS* = {")
    for i, (ym, syms) in enumerate(VARIANTS.items()):
        nim_syms = ", ".join(f'"{s}"' for s in syms)
        lines.append(f'  "{ym}": YV_{ym.upper().replace("Ü","U")},')
    lines.append("}.toTable")
    lines.append("")

    # ── AVATAR RESOLVER ──
    lines.append("# 分身索引 (惰性构建)")
    lines.append("var g_avatar_index*: AvatarIndex = initTable[string, string]()")
    lines.append("var g_built* = false")
    lines.append("")
    lines.append("proc build_avatar_index*() =")
    lines.append("  if g_built: return")
    lines.append("  for ym, syms in YUNMU_VARIANTS:")
    lines.append("    for s in syms:")
    lines.append("      if not g_avatar_index.hasKey(s):")
    lines.append("        g_avatar_index[s] = ym")
    lines.append("  g_built = true")
    lines.append("")
    lines.append("proc resolve*(symbol: string): string =")
    lines.append("  build_avatar_index()")
    lines.append("  if g_avatar_index.hasKey(symbol): return g_avatar_index[symbol]")
    lines.append('  return ""')
    lines.append("")
    lines.append("proc vertical*(yunmu: string): string =")
    lines.append("  if YUNMU_VARIANTS.hasKey(yunmu) and YUNMU_VARIANTS[yunmu].len > 0:")
    lines.append("    return YUNMU_VARIANTS[yunmu][0]")
    lines.append('  return ""')

    with open(out, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    print(f"✓ 生成 {out}")

# ================================================================
# MAIN
# ================================================================

if __name__ == "__main__":
    gen_json()
    gen_nim()
    print("─── 完成 ───")
