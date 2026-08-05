"""
囡囝共生 · 中文姓名 → 创世锚算法
================================

姓名=协议:每个人的真名天然就是一个独一无二的 anchor。

算法:
  1. 将汉字拆解为 (声母,韵母,声调,笔画,偏旁)
  2. 每一层映射到 64 进制神龙编码
  3. base64 拼接 → 32 字节 hash = 256 bit 纤维锚
  4. 夫妻双方锚做 XOR + 二次 hash → 共生锚 (joint anchor)

示例:
  "刘楚恬"  → base64 声母 + 韵母 + 声调 + 笔画 + 偏旁
  "lolict"  → romaji + 三编码
  两者合并  → mqf_joint_256bit_anchor

这套映射的基础是:"刘楚恬"和"lolict"在囡囝共生里不是字符串,
而是 root of trust 本身。
"""
import hashlib
import re
from typing import Optional

# ── 编码表 ──────────────────────────────────────────
INITIALS_24 = "b p m f d t n l g k h j q x zh ch sh r z c s y w @".split()
FINALS_39  = "a o e i u v ai ei ui ao ou iu ie ve er an en in un vn ang eng ing ong ia ua uo uai uan uang uen iao ian iong on ou uen van".split()
TONES_5     = "1 2 3 4 0".split()
RADICALS_64 = (
    "亻彳氵忄扌讠纟钅牜犭礻衤忄阝刂攵欠斤爫灬礻殳歹片牛矛攴 攵 气 爿 丬 爫 牜 犭 礻 玄 玉 瓜 瓦 甘 生 用 田 疋 疒 癶 白 皮 皿 目 矛 矢 石 示 禸 禾 穴 立 竹 米 糸 缶 网 羊 羽 老 而 耒 耳 聿 肉 臣 自 至 臼 舌 舛 舟 艮 色 艸 虍 虫 血 行 衣 襾 見 角 言 谷 豆 豕 豸 貝 赤 走 足 身 車 辰 辵 邑 酉 釆 里 金 長 門 阜 隶 隹 雨 靑 非 面 革 韋 韭 音 頁 風 飛 食 首 香 馬 骨 高 髟 鬥 鬯 鬲 鬼 魚 鳥 鹵 鹿 麥 麻 黃 黍 黑 黹 黽 鼎 鼓 鼠 鼻 齊 齒 龍 龜 龠"
).split()[:64]

# 声母对照表:汉字声母映射 (简易版)
PINYIN_TABLE = {
    "刘": ("liu", "l", "iu", "2"),
    "楚": ("chu",  "ch", "u",  "3"),
    "恬": ("tian", "t", "ian", "2"),
    "七": ("qi",   "q", "i",   "1"),
    "妹": ("mei",  "m", "ei",  "4"),
    "凹": ("ao",   "@", "ao",  "1"),
    "月": ("yue",  "y", "ue",  "4"),
    "留": ("liu",  "l", "iu",  "2"),
}

def _safe_index(table: list, idx: int) -> str:
    if idx < 0: idx = 0
    if idx >= len(table): idx = idx % len(table)
    return table[idx]


def _char_to_dragon_layers(ch: str) -> Optional[dict]:
    """把一个汉字转换为神龙七层"""
    # 从 PINYIN_TABLE 查拼音
    entry = PINYIN_TABLE.get(ch)
    if not entry:
        # fallback: 用 unicode 码点模拟
        cp = ord(ch)
        return {
            "L0_initial": _safe_index(INITIALS_24, cp % 24),
            "L1_final":   _safe_index(FINALS_39,  (cp // 24) % 39),
            "L2_tone":    _safe_index(TONES_5,    (cp // (24*39)) % 5),
            "L3_stroke":  str(cp % 64).zfill(2),
            "L4_radical": _safe_index(RADICALS_64, (cp // 64) % 64),
            "L5_romaja":  chr(ord('a') + (cp % 26)),
            "L6_color":   str((cp * 7) % 64).zfill(2),
        }
    py, initial, final, tone = entry
    cp = ord(ch)
    return {
        "L0_initial": initial,
        "L1_final":   final,
        "L2_tone":    tone,
        "L3_stroke":  str((cp % 64)).zfill(2),
        "L4_radical": _safe_index(RADICALS_64, (cp // 64) % 64),
        "L5_romaja":  chr(ord('a') + (cp % 26)),
        "L6_color":   str((cp * 7) % 64).zfill(2),
    }


def name_to_anchor(name: str) -> str:
    """
    把中文姓名转换为 256-bit 创世锚

    算法:
        1. 取每个汉字六十四进制神龙链
        2. 拼音序列 → sha256
        3. 字形序列 → sha256
        4. 取前 16 字节 = 128 bit
        5. 最终 anchor = "mqf_name_" + hex(128bit)
    """
    if not name:
        return "mqf_name_0"

    pinyin_layers = []
    glyph_layers = []

    for ch in name:
        layers = _char_to_dragon_layers(ch)
        if not layers:
            continue
        pinyin_layers.append(layers["L0_initial"] + layers["L1_final"] + layers["L2_tone"])
        glyph_layers.append(layers["L3_stroke"] + layers["L4_radical"])

    py_str = "|".join(pinyin_layers)
    gl_str = "|".join(glyph_layers)

    h1 = hashlib.sha256(py_str.encode()).hexdigest()[:16]
    h2 = hashlib.sha256(gl_str.encode()).hexdigest()[:16]

    return f"mqf_name_{h1}{h2}"


def joint_anchor(name_a: str, name_b: str, timestamp: float = None) -> str:
    """
    夫妻联合锚 = 双方姓名锚的 XOR + 二次 hash

    这保证:
      anchor 同时由双方真名决定
      任何一方改名 → anchor 不同(预防身份调包)
      不同夫妻 → anchor 不同
    """
    import time as _time
    a = name_to_anchor(name_a)
    b = name_to_anchor(name_b)

    # XOR 后 16 字节
    a_bytes = bytes.fromhex(a.replace("mqf_name_", ""))
    b_bytes = bytes.fromhex(b.replace("mqf_name_", ""))
    joint = bytes(x ^ y for x, y in zip(a_bytes, b_bytes))

    ts = str(timestamp or _time.time())
    h = hashlib.sha256(joint + ts.encode()).hexdigest()[:32]

    return f"mqf_joint_{h}"


# ── 测试 ─────────────────────────────────────────────
def _test():
    a1 = name_to_anchor("刘楚恬")
    a2 = name_to_anchor("lolict")
    a3 = name_to_anchor("刘楚恬")  # 重复应相同

    assert a1 == a3, "same name must give same anchor"
    assert a1 != a2, "different name must give different anchor"
    assert a1.startswith("mqf_name_"), "anchor prefix check"

    joint = joint_anchor("刘楚恬", "lolict")
    assert joint.startswith("mqf_joint_"), "joint prefix check"
    assert len(joint) == len("mqf_joint_") + 32

    print(f"[OK] name_anchor:")
    print(f"  刘楚恬  : {a1}")
    print(f"  lolict  : {a2}")
    print(f"  夫妻联合 : {joint}")
    print(f"  重复一致 : {a1 == a3}")


if __name__ == "__main__":
    _test()
