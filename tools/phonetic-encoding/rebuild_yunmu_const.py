#!/usr/bin/env python3
"""
从 z-api.txt 里确认的最新编码串重建 FINAL_ENCODING_STRING 与 YUNMU_NAMES。
原则：不猜测、不补充、严格对齐编码串实际包含的对。
"""
import unicodedata

# z-api.txt line 281 确认的 45 对编码串（含 ☁ 以对齐主体变更 log）
FINAL = (
    "Ȍ}ȍ}Ꙙ}ꙙ}Ȑ}ȑ}Ӱ}ӱ}Ü}ü}"
    "Ѧ}ѧ}Ӗ}ӗ}Ə}ə}Ѽ}ѽ}Ѱ}ѱ}Ꜵ}ꜵ}Ȣ}ȣ}Ꙍ}ꙍ}"
    "Ⴘ}Ⴏ}Ⴎ}Ѥ}ѥ}Ё}ё}ᵫ}ꭂ}Є}є}Ꙗ}ꙗ}"
    "ⱄ}ⱅ}ѩ}Ǟ}ǟ}ꬰ}ꬱ}Ꜻ}ꜻ}Ȫ}ȫ}Ⴊ}Ⴆ}Ѿ}ѿ}ꭃ}ꭄ}"
    "Ꙙ}ꙙ}ɤ}ɣ}ꙝ}Ꙝ}Ⴄ}Ⴂ}Ѫ}ѫ}ꭌ}Ⴋ}Ⴉ}"
    "ӫ}Ӭ}Ꙟ}ꙟ}Ȏ}ȏ}Ⴓ}Ⴍ}ꭀ}ꭁ}ѭ}Ѭ}ꙛ}Ꙛ}Ⴔ}Ⴇ}Ȱ}ȱ}☁}Ѻ}ѻ"
)

tokens = FINAL.split("}")
tokens = [t for t in tokens if t]
assert len(tokens) % 2 == 0, f"token 数量 {len(tokens)} 不是偶数: {tokens}"

pairs = []
for i in range(0, len(tokens), 2):
    hi, lo = tokens[i], tokens[i+1]
    cp_hi, cp_lo = ord(hi), ord(lo)
    pairs.append((cp_hi, cp_lo, hi, lo))

print(f"总对数: {len(pairs)}")
print()
print(f"{'序号':>4s}  {'大写':>8s}  {'小写':>8s}  {'字符':>6s}  Unicode 名称")
print("─" * 70)
for idx, (chi, clo, hi, lo) in enumerate(pairs):
    try:
        nd = unicodedata.name(hi, "?")[:30]
    except ValueError:
        nd = "?"
    print(f"{idx:4d}  U+{chi:04X}  U+{clo:04X}  {hi}{lo}  {nd}")

# Nim const 数组
print()
print("─" * 70)
print(f"作为 Nim 数组（{len(pairs)} 对）:")
print(f"const FINAL_PAIRS*: array[{len(pairs)}, (int32, int32)] = [")
for i, (chi, clo, _, _) in enumerate(pairs):
    comma = "," if i < len(pairs)-1 else " "
    print(f"  (0x{chi:04X}'i32, 0x{clo:04X}'i32){comma}")
print("]")
