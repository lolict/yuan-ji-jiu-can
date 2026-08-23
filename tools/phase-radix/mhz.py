#!/usr/bin/env python3
"""
mhz — 满全法颜色进制 CLI

直接调用 C 核心 libcolor.so。
Python 只是壳，审判/衍射/注册全部由 C 执行。

用法:
  mhz trial <a> <b>     审判一对符号
  mhz diffract <name>   衍射分析一个符号
  mhz pair <a> <b>      配对衍射分析
  mhz register <name> <type> <level>  注册符号
  mhz demo              运行演示
  mhz status            系统状态
"""

import sys as _sys
import os

# 确保能 import 包
_sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from phase_radix._native import (
    ColorSystemRef,
    TYPE_BASE, TYPE_TONE, TYPE_COMPOUND, TYPE_SYMBOL,
    TYPE_EXTRA, TYPE_SHENG_MU, TYPE_MODIFIER,
    LEVEL_L1, LEVEL_L2, LEVEL_L3,
    type_str,
    SoundRadixRef,
    GongyueshuRef,
    DimensionRef,
    HanziGeomRef,
    NegativeRef,
    STRUCT_LEFT_RIGHT, STRUCT_TOP_BOTTOM, STRUCT_SURROUND, STRUCT_SEMI,
    STRUCT_LCR, STRUCT_TMB, STRUCT_PINZI, STRUCT_SINGLE,
    STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_ZHE,
    REASON_CROSS_LEVEL, REASON_CROSS_TYPE, REASON_CROSS_ANCHOR,
    CONTOUR_FLAT, CONTOUR_RISE, CONTOUR_FALL, CONTOUR_DIP,
)


# ── 声母表 ──────────────────────────────────────────────────
SHENGMU_48 = [
    "b","p","m","f","d","t","n","l",
    "g","k","h","j","q","x","zh","ch",
    "sh","r","z","c","s","y","w","y",
    "y","w","ȵ","ŋ","ɕ","ʈʂ","ʈʂʰ","ʂ",
    "ts","tsʰ","tɕ","tɕʰ","x","ɣ","ɲ","ʑ",
    "tʂ","tʂʰ","ɖ","ɳ","ɭ","ʐ","ɻ","ʔ",
]


def cmd_trial(sys: ColorSystemRef, a: str, b: str):
    """审判一对符号"""
    v = sys.trial(a, b)
    label = sys.verdict_str(v)
    print(f"{a} vs {b}: {label}")
    return v


def cmd_diffract(sys: ColorSystemRef, name: str):
    """衍射分析"""
    d = sys.diffract(name)
    print(f"符号: {d['name']}")
    print(f"  类型: {d['type_str']}")
    print(f"  锚点: 0x{d['anchor']:02X}")
    print(f"  审美分: {d['aesthetic']:.2f}")
    print(f"  出力: {d['pressure_given']:.2f}")
    print(f"  受力: {d['pressure_received']:.2f}")
    print(f"  失衡: {d['equilibrium_delta']:.2f}")
    print(f"  平衡: {'是' if d['balanced'] else '否'}")


def cmd_pair(sys: ColorSystemRef, a: str, b: str):
    """配对衍射"""
    pd = sys.pair_diffract(a, b)
    print(f"配对: {a} ↔ {b}")
    print(f"  类型匹配: {'是' if pd['type_match'] else '否'}")
    print(f"  锚点匹配: {'是' if pd['anchor_match'] else '否'}")
    print(f"  对称度: {pd['symmetry']:.2f}")
    print(f"  比例: {pd['proportion']:.2f}")
    print(f"  密度平衡: {pd['density_balance']:.2f}")
    print(f"  整体审美: {pd['overall_aesthetic']:.2f}")
    print(f"  编织合格: {'是' if pd['weaving_ok'] else '否'}")


def cmd_register(sys: ColorSystemRef, name: str, type_name: str, level_name: str):
    """注册符号"""
    type_map = {
        "base": TYPE_BASE, "tone": TYPE_TONE,
        "compound": TYPE_COMPOUND, "symbol": TYPE_SYMBOL,
        "extra": TYPE_EXTRA, "shengmu": TYPE_SHENG_MU,
        "modifier": TYPE_MODIFIER,
    }
    level_map = {"L1": LEVEL_L1, "L2": LEVEL_L2, "L3": LEVEL_L3}
    
    t = type_map.get(type_name.lower())
    l = level_map.get(level_name.upper())
    
    if t is None:
        print(f"未知类型: {type_name}")
        print(f"可用: {', '.join(type_map.keys())}")
        return
    if l is None:
        print(f"未知层级: {level_name}")
        print(f"可用: {', '.join(level_map.keys())}")
        return
    
    ok = sys.register(name, t, l)
    print(f"注册 {'成功' if ok else '失败'}: {name} = {type_name}/{level_name}")


def cmd_demo(sys: ColorSystemRef):
    """运行演示"""
    print("=" * 50)
    print("  满全法 · 颜色进制 — 演示")
    print("=" * 50)
    
    # 注册声母
    count = sys.register_shengmu_batch(SHENGMU_48)
    print(f"\n声母注册: {count}/48")
    
    # 审判测试
    print("\n─── 审判测试 ───")
    tests = [
        ("a", "á", "基韵锚 vs 声调 → 跨层级"),
        ("a", "A", "基韵锚 vs 大写 → 通过"),
        ("a", "o", "基韵锚 vs 基韵锚 → 跨基调"),
        ("b", "p", "声母 vs 声母 → 通过"),
        ("d", "t", "声母 vs 声母 → 通过"),
        ("zh", "ch", "翘舌音 vs 翘舌音 → 通过"),
        ("g", "k", "舌根音 vs 舌根音 → 通过"),
        ("m", "f", "唇音 vs 唇音 → 通过"),
    ]
    for a, b, desc in tests:
        v = sys.trial(a, b)
        label = sys.verdict_str(v)
        print(f"  {a:4} vs {b:4} → {label:20} [{desc}]")
    
    # 衍射
    print("\n─── 衍射分析 ───")
    for name in ["a", "á", "b", "zh"]:
        d = sys.diffract(name)
        print(f"  {name:4}: 类型={d['type_str']:6} 出力={d['pressure_given']:.2f} 失衡={d['equilibrium_delta']:.2f}")
    
    # 配对衍射
    print("\n─── 配对衍射 ───")
    for a, b in [("a", "A"), ("b", "p"), ("a", "á")]:
        pd = sys.pair_diffract(a, b)
        print(f"  {a} ↔ {b}: 类型匹配={'是' if pd['type_match'] else '否'} 审美={pd['overall_aesthetic']:.2f}")
    
    print("\n" + "=" * 50)
    print("  演示完成")
    print("=" * 50)


def create_system() -> ColorSystemRef:
    """创建并初始化完整系统 (声母+韵母+声调)"""
    s = ColorSystemRef()
    s.register_shengmu_batch(SHENGMU_48)
    return s


def cmd_addr(ga: GongyueshuRef, args: list):
    """公约数地址编码: mhz addr <声母> <韵母> <声调>"""
    if len(args) < 3:
        print("用法: mhz addr <声母> <韵母> <声调>")
        print("示例: mhz addr b a 1    → bā 的公约数地址")
        print("      mhz addr sh uang 1 → shuāng 的缩写地址")
        print("      mhz addr \"\" i 4      → ì")
        return
    shengmu = args[0] if args[0] else ""
    yunmu = args[1]
    tone = int(args[2])
    
    ok = ga.encode(shengmu, yunmu, tone)
    if ok:
        idx = ga.count - 1
        val, bits = ga.get_packed(idx)
        full_chars = len(shengmu) + len(yunmu)
        full_bits = full_chars * 8 + 2
        saving = (100 - bits * 100 // full_bits) if full_bits > 0 else 0
        print(f"编码: {shengmu}+{yunmu}+声{tone}")
        print(f"  全长: {full_chars}字符 = {full_bits}bit")
        print(f"  公约数地址: {bits}bit (0x{val:04X})")
        print(f"  压缩: 省{saving}%")
    else:
        print(f"编码失败: {shengmu}+{yunmu}")


def cmd_dim(ds: DimensionRef, args: list):
    """多维几何: mhz dim <声母索引> <韵母索引> <声调>"""
    if len(args) < 3:
        print("用法: mhz dim <声母索引0-47> <韵母索引0-40> <声调1-4>")
        print("示例: mhz dim 0 0 1    → (b, a, 一声) 的线性地址")
        print("      mhz dim 14 30 2  → (zh, ang, 二声)")
        return
    sm = int(args[0])
    ym = int(args[1])
    tone = int(args[2])
    
    addr = ds.linear_addr(sm, ym, tone)
    shengmu_names = ["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch",
                     "sh","r","z","c","s","y","w","y"]
    yunmu_names = ["a","o","e","i","u","ü","ai","ei","ao","ou","ia","ie","io","iu",
                   "ua","uo","ue","üe","iao","iou","uai","uei","an","en","in","un",
                   "ün","ian","uan","üan","uen","ang","eng","ing","ong","iang",
                   "iong","uang","ueng"]
    
    sm_name = shengmu_names[sm] if sm < len(shengmu_names) else f"?{sm}"
    ym_name = yunmu_names[ym] if ym < len(yunmu_names) else f"?{ym}"
    
    print(f"坐标: ({sm_name}, {ym_name}, 声{tone})")
    print(f"  → 线性地址: {addr}")
    print(f"  → 这是从0维起点到3维空间的一个点")
    print(f"  → 遍历路径: 声母{sm_name} → 遍历韵母{ym_name} → 叠加声调{tone}")


def cmd_negative(ns: NegativeRef, args: list):
    """负样本: mhz neg <符号A> <符号B> [reason]"""
    if len(args) < 2:
        print("用法: mhz neg <符号A hex> <符号B hex> [拒绝原因]")
        print("示例: mhz neg 0x61 0xE1    → 裁定 a vs á")
        print("      mhz neg 0x61 0xE1 1  → 回写 a vs á 到记忆库(原因=跨层级)")
        return
    sym_a = int(args[0], 0)
    sym_b = int(args[1], 0)
    
    if len(args) >= 3:
        # 直接回写
        reason = int(args[2])
        ns.writeback(sym_a, sym_b, reason)
        reason_names = ["无","跨层级","跨类型","跨锚点","审美失败",
                        "编织失败","黑名单","重复","降解"]
        print(f"回写记忆库: ({hex(sym_a)},{hex(sym_b)}) 原因={reason_names[reason]}")
    else:
        # 裁定
        v, reason = ns.judge(sym_a, sym_b)
        verdict_names = ["通过","已知负样本","新拒绝","申诉"]
        reason_names = ["无","跨层级","跨类型","跨锚点","审美失败",
                        "编织失败","黑名单","重复","降解"]
        print(f"裁定 ({hex(sym_a)} vs {hex(sym_b)}):")
        print(f"  判定: {verdict_names[v]}")
        if v > 0:
            print(f"  原因: {reason_names[reason] if reason < len(reason_names) else '?'}")


def cmd_law(ns: NegativeRef, args: list):
    """判定树: mhz law <vA> <vB>"""
    if len(args) < 2:
        print("用法: mhz law <vA> <vB>")
        print("示例: mhz law 100 100  → vA+vB == 2·v_core ?")
        return
    vA = int(args[0])
    vB = int(args[1])
    result = ns.law_judge(vA, vB)
    print(f"{vA} + {vB} = {vA+vB}")
    print(f"  2·v_core = 200")
    print(f"  → {'通过 (产权合法)' if result else '负样本 (侵权)'}")


def cmd_hanzi(hg: HanziGeomRef, args: list):
    """汉字几何: mhz hanzi <Unicode> <部首1> <部首2> <结构> <笔画数> <笔顺...>"""
    if len(args) < 5:
        print("用法: mhz hanzi <Unicode> <部首1> <部首2> <结构> <笔画数> <笔顺...>")
        print("  结构: 1=左右 2=上下 3=全包围 4=半包围 5=左中右 6=上中下 7=品字 8=独体")
        print("  笔顺: 0=横 1=竖 2=撇 3=捺 4=折")
        print("示例: mhz hanzi 0x660E 72 74 1 8 2 1 2 1 1 1 1 1")
        return
    char_code = int(args[0], 0)
    radical1 = int(args[1])
    radical2 = int(args[2])
    structure = int(args[3])
    stroke_count = int(args[4])
    strokes = [int(x) for x in args[5:5+stroke_count]] if stroke_count > 0 else []
    
    ok = hg.add(char_code, radical1, radical2, structure, stroke_count, strokes)
    if ok:
        idx = len(strokes)  # 简化
        packed = hg.get_packed(0)  # 获取第一个
        struct_names = ["?","左右","上下","全包围","半包围","左中右","上中下","品字","独体"]
        print(f"编码: U+{char_code:04X}")
        print(f"  部首: {radical1} + {radical2}")
        print(f"  结构: {struct_names[structure]}")
        print(f"  笔画: {stroke_count}画")
        print(f"  紧凑编码: {packed:08X}")
    else:
        print("编码失败")


def cmd_sound(sound: SoundRadixRef, args: list):
    """编码音节: mhz sound <声母> <韵母> <声调>"""
    if len(args) < 3:
        print("用法: mhz sound <声母> <韵母> <声调1-4>")
        print("示例: mhz sound b a 1    → bā")
        print("      mhz sound zh ang 3  → ǎng")
        print("      mhz sound \"\" i 4       → ì (零声母)")
        return
    shengmu = args[0] if args[0] else ""
    yunmu = args[1]
    tone = int(args[2])
    
    ok = sound.encode_syllable(shengmu, yunmu, tone)
    dur = sound.duration_ms()
    contour_names = ["平(一声)", "升(二声)", "凹(三声)", "降(四声)"]
    print(f"编码 {'成功' if ok else '失败'}: {shengmu}+{yunmu}+声{tone}")
    print(f"  声调轮廓: {contour_names[tone-1] if 1<=tone<=4 else '?'}")
    print(f"  总时长: {dur}ms")


def cmd_sing(sound: SoundRadixRef, args: list):
    """编码多个音节并输出 WAV: mhz sing <音节1> <音节2> ..."""
    if len(args) < 1:
        print("用法: mhz sing <声母:韵母:声调> ...")
        print("示例: mhz sing b:a:1 m:a:3 zh:ang:2")
        return
    
    for syllable in args:
        parts = syllable.split(":")
        if len(parts) != 3:
            print(f"忽略无效音节: {syllable}")
            continue
        s, y, t = parts[0], parts[1], int(parts[2])
        sound.encode_syllable(s, y, t)
    
    dur = sound.duration_ms()
    print(f"编码 {len(args)} 个音节, 总时长 {dur}ms")
    print("(渲染到 WAV 需要更多内存, 请使用 Python API)")


def cmd_status(sys: ColorSystemRef):
    """系统状态"""
    print("─── 满全法系统状态 ───")
    print(f"C 核心: libcolor.so")
    print(f"类型系统: 7类 × 3层")
    print(f"审判五关: 层级→类型→锚→审美→编织")
    print(f"衍射维度: 6维")
    print(f"集装箱: 1bit/16字 × 254框")
    print(f"声音进制: 事件编码 (频率+音色+时长+轮廓+强度)")


def main():
    if len(_sys.argv) < 2:
        print(__doc__)
        return
    
    command = _sys.argv[1]
    args = _sys.argv[2:]
    
    # 需要系统实例的命令
    if command == "trial":
        if len(args) < 2:
            print("用法: mhz trial <a> <b>")
            return
        sys = create_system()
        cmd_trial(sys, args[0], args[1])
    
    elif command == "diffract":
        if len(args) < 1:
            print("用法: mhz diffract <name>")
            return
        sys = create_system()
        cmd_diffract(sys, args[0])
    
    elif command == "pair":
        if len(args) < 2:
            print("用法: mhz pair <a> <b>")
            return
        sys = create_system()
        cmd_pair(sys, args[0], args[1])
    
    elif command == "register":
        if len(args) < 3:
            print("用法: mhz register <name> <type> <level>")
            print("  type: base/tone/compound/symbol/extra/shengmu/modifier")
            print("  level: L1/L2/L3")
            return
        sys = create_system()
        cmd_register(sys, args[0], args[1], args[2])
    
    elif command == "demo":
        sys = create_system()
        cmd_demo(sys)
    
    elif command == "status":
        sys = create_system()
        cmd_status(sys)
    
    elif command == "addr":
        ga = GongyueshuRef()
        cmd_addr(ga, args)
    
    elif command == "dim":
        ds = DimensionRef()
        cmd_dim(ds, args)
    
    elif command == "hanzi":
        hg = HanziGeomRef()
        cmd_hanzi(hg, args)
    
    elif command == "neg":
        ns = NegativeRef()
        cmd_negative(ns, args)
    
    elif command == "law":
        ns = NegativeRef()
        cmd_law(ns, args)
    
    elif command == "sound":
        sound = SoundRadixRef()
        cmd_sound(sound, args)
    
    elif command == "sing":
        sound = SoundRadixRef()
        cmd_sing(sound, args)
    
    else:
        print(f"未知命令: {command}")
        print(__doc__)


if __name__ == "__main__":
    main()
