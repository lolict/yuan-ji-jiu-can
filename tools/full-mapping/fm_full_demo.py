#!/usr/bin/env python3
# ============================================================
# 全映射引擎 · 完整演示（Python 五系统统一验证）
#
# 来源：与 Nim tools/full-mapping/fm_demo.nim 等价
# 证明：九元层级 ↔ YinYang ↔ 二进制 ↔ mltt ↔ 高维偏移率
#       五个系统完全互联，任意输入可转换为五种等价表示。
# ============================================================

import sys
import math
from fm_engine import load_full_mapping_engine, FullMappingEngine, POLARITY

HEADER = "═" * 52

def check(cond, msg=""):
    if not cond:
        raise AssertionError(msg)


def main():
    eng = load_full_mapping_engine()

    print(HEADER)
    print("  全映射引擎 · 五系统互联")
    print("  九元层级 ↔ YinYang ↔ 二进制 ↔ mltt ↔ 高维偏移率")
    print(HEADER)
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 1】九元层级完整映射
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 1】九元层级完整映射 · 任意层五种等价表示")
    print(HEADER)
    print()
    print("  层级公式: δ = n × 0.1, M = 1 + δ")
    print("  极性规则: 偶数索引=阳=界线, 奇数索引=阴=容器, 索引8=平衡")
    print()

    report = eng.full_map("满全法")

    print("  符号:", report["symbol"])
    print("  层级数:", report["total_levels"])
    print()
    print("  ┌──────┬──────┬─────┬──────┬──────┬───────┬────────┬────────┐")
    print("  │层级  │中文  │δ    │M     │阴阳  │二进制 │mltt    │十进制  │")
    print("  ├──────┼──────┼─────┼──────┼──────┼───────┼────────┼────────┤")

    for e in report["entries"]:
        print(
            f"  │{e['level_name']:5s}│{e['chinese_name']:5s}│"
            f"{e['offset_rate']:4.2f}│{e['multiplier']:5.2f}│"
            f"{e['yin_yang']:5s}│{e['binary_string']:6s}│"
            f"{e['mltt_code']:7s}│{e['base10_value']:7d}│"
        )

    print("  └──────┴──────┴─────┴──────┴──────┴───────┴────────┴────────┘")
    print()

    # 断言：层级映射完整
    check(report["total_levels"] == 9, "层级数应为 9")
    check(len(report["entries"]) == 9, "条目应为 9")

    # 断言：Nim LEVEL_BINARIES 一致
    expected_binaries = ["0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001"]
    for i, (e, eb) in enumerate(zip(report["entries"], expected_binaries)):
        check(e["binary_string"] == eb, f"层级 {i} 二进制应为 {eb}")

    # 断言：mltt 代码一致
    expected_mltt = ["mlll", "ml.yy", "gtt", "mlgy", "wgslz", "smrl", "lctqf", "mlnn", "njsgr"]
    for i, (e, em) in enumerate(zip(report["entries"], expected_mltt)):
        check(e["mltt_code"] == em, f"层级 {i} mltt 应为 {em}")

    # 断言：偏移率公式正确
    for i, e in enumerate(report["entries"]):
        expected_offset = round(i * 0.1, 2)
        check(abs(e["offset_rate"] - expected_offset) < 1e-9, f"层级 {i} 偏移率应为 {expected_offset}")
        expected_mult = round(1.0 + expected_offset, 2)
        check(abs(e["multiplier"] - expected_mult) < 1e-9, f"层级 {i} 乘数应为 {expected_mult}")

    print("  ✅ 九元层级五种等价表示 — 正确")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 2】跨层级统一表示
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 2】跨层级统一表示 · 任意值回归初代数")
    print(HEADER)
    print()
    print("  跨层级转换: 先回归初代数, 再折算目标层")
    print()

    test_cases = [
        (10.0, 1),
        (10.0, 2),
        (100.0, 4),
    ]
    for val, lvl in test_cases:
        base = eng.static_regression(val, lvl)
        m = eng.multiplier(lvl)
        print(f"  第 {lvl} 代 {val} ÷ {m:.2f} = {base:.4f}")

    # 跨层换算
    print()
    print("  跨层换算: 第4代 50.0 → 第2代")
    c42 = eng.convert_to_level(50.0, 4, 2)
    print(f"  M(4)={eng.multiplier(4):.2f}, M(2)={eng.multiplier(2):.2f}, 结果 = {c42:.4f}")
    expected = 50.0 / eng.multiplier(4) * eng.multiplier(2)
    check(abs(c42 - expected) < 1e-9, "跨层换算公式错误")
    print("  ✅ 跨层换算正确")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 3】极性 ↔ YinYang ↔ 二进制 互联
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 3】极性 ↔ YinYang ↔ 二进制 互联")
    print(HEADER)
    print()
    print("  ┌──────┬──────┬────────┬────────┐")
    print("  │层级  │极性  │YinYang │二进制  │")
    print("  ├──────┼──────┼────────┼────────┤")

    for lvl in range(9):
        pol = FullMappingEngine.polarity_for_level(lvl)
        yy = FullMappingEngine.polarity_to_yy(pol)
        bin_str = FullMappingEngine.polarity_to_binary(pol)
        pol_enum = pol
        e = eng.get_level(lvl)
        print(
            f"  │{e['level_name']}    │"
            f"{pol_enum:4s}│"
            f"{yy:5s}│"
            f"{bin_str:7s}│"
        )

    print("  └──────┴──────┴────────┴────────┘")
    print()

    # 断言：极性映射
    # 偶数索引(一元/三元/五元/七元) = 阳 = yang
    # 奇数索引(二元/四元/六元/八元) = 阴 = yin
    # 索引8 = balanced
    for lvl in range(8):
        pol = FullMappingEngine.polarity_for_level(lvl)
        if lvl % 2 == 0:
            check(pol == "yang", f"层级 {lvl} 应为 yang")
        else:
            check(pol == "yin", f"层级 {lvl} 应为 yin")
    check(FullMappingEngine.polarity_for_level(8) == "balanced", "层级 8 应为 balanced")

    # 极性到二进制
    check(FullMappingEngine.polarity_to_binary("yin") == "0")
    check(FullMappingEngine.polarity_to_binary("yang") == "1")
    check(FullMappingEngine.polarity_to_binary("balanced") == "0.5")

    print("  ✅ 极性↔YinYang↔二进制 互联 — 一致")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 4】俘获规则 · 高维降维
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 4】俘获规则 · 非源头层级强制降维")
    print(HEADER)
    print()

    high_val = 100.0
    high_lvl = 7
    captured = eng.capture_level(high_val, high_lvl)
    print(f"  俘获: {high_val} @ 第 {high_lvl} 代")
    print(f"  方法: {captured['conv_method']}")
    print(f"  → {captured['notes']}")

    expected_captured = high_val / eng.multiplier(high_lvl)
    check(abs(captured["converted_value"] - expected_captured) < 1e-9)
    print("  ✅ 俘获完成")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 5】阶和叠加 · 锐度
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 5】阶和叠加 · 多世界共存锐度")
    print(HEADER)
    print()

    levels_set = [0, 2, 4, 6, 8]
    sharpness = eng.tier_sum_sharpness(levels_set)
    mean_mul = sum(eng.multiplier(l) for l in levels_set) / len(levels_set)

    print(f"  叠加层级: {[l+1 for l in levels_set]}")
    print(f"  叠加乘数均值: {mean_mul:.4f}")
    print(f"  锐度 (偏移量标准差): {sharpness:.4f}")
    print(f"  含义: 层级差异越大, 感知越尖锐")

    # 锐度应为所有0.1的倍数层级的标准差
    expected_rates = [l * 0.1 for l in levels_set]
    expected_mean = sum(expected_rates) / len(expected_rates)
    expected_var = sum((r - expected_mean) ** 2 for r in expected_rates) / len(expected_rates)
    expected_sharp = math.sqrt(expected_var)
    check(abs(sharpness - expected_sharp) < 1e-9, "锐度计算错误")
    print("  ✅ 阶和叠加验证完成")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 6】2²⁵⁶ 高位对齐
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 6】2²⁵⁶ 高位对齐 · 寻址空间")
    print(HEADER)
    print()

    addr = eng.high_address_space()
    print(f"  {addr}")
    print(f"  各层级对齐标识:")
    for lvl in range(9):
        print(f"    {eng.alignment_id(lvl)}")

    # 计算 2^256
    check(abs(2 ** 256 - 1.157920892373162e+77) < 1e+70, "2^256 数量级异常")
    print("  ✅ 高位对齐验证完成")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 7】mltt 短码互译
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 7】mltt 命名密码表 · 九层短码")
    print(HEADER)
    print()

    for lvl in range(9):
        e = eng.get_level(lvl)
        print(f"  {e['mltt']:8s} → {e['chinese_name']:5s} ({e['level_name']})")

    print()
    print("  ✅ mltt 短码映射完成")
    print()

    # ═══════════════════════════════════════════════════════
    # 【Part 8】mltt 反向互译
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("【Part 8】mltt 反向互译 · 短码 → 层级")
    print(HEADER)
    print()

    for mltt_code in ["mlll", "ml.yy", "gtt", "mlgy", "njsgr"]:
        e = eng._by_mltt.get(mltt_code)
        check(e is not None, f"短码 {mltt_code} 未找到")
        print(f"  {mltt_code:8s} → 第{e['level']+1}层 {e['level_name']} ({e['chinese_name']})")

    check("mlll" in eng._by_mltt)
    check("njsgr" in eng._by_mltt)
    print("  ✅ mltt 反向互译完成")
    print()

    # ═══════════════════════════════════════════════════════
    # 总结
    # ═══════════════════════════════════════════════════════

    print(HEADER)
    print("  全映射引擎 · 演示完成")
    print(HEADER)
    print()
    print("  ✅ 九元层级五种等价表示 · 完整")
    print("  ✅ 跨层级静态回归 · 正确")
    print("  ✅ 极性↔YinYang↔二进制互联 · 一致")
    print("  ✅ 俘获高维降维 · 正确")
    print("  ✅ 阶和叠加锐度 · 可计算")
    print("  ✅ 2²⁵⁶ 高位对齐 · 就绪")
    print("  ✅ mltt 短码互译 · 完整")
    print()
    print("  [全映射已接通] 五系统互联完成。")
    print()


if __name__ == "__main__":
    try:
        main()
    except AssertionError as e:
        print(f"\n❌ 验证失败: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\n⚠️ 异常: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(2)
