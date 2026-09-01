#!/usr/bin/env python3
# ============================================================
# mltt 命名密码表 · 完整演示（158 条数据验证）
#
# 验证项目：
#   1. 加载 158 条数据完整性
#   2. 家庭命名双向查询
#   3. 历史符号（容器符/操作符/极性符/层级符/审判符/产权符）
#   4. 近义词链接
#   5. 极性中和检查
#   6. 推荐机制
#   7. 动态新增 + 撤销
# ============================================================

import sys
from mltt_engine import load_engine, POLARITY_LABEL

HEADER = "═" * 50

def main():
    eng = load_engine()

    print(HEADER)
    print("  mltt 命名密码表 · 完整数据驱动演示")
    print(HEADER)
    print()

    # ── 阶段 1 ──
    print("【阶段 1】数据完整性验证")
    print(eng.stats())
    assert eng.total >= 145, f"条目不足: {eng.total}"
    print(f"  ✅ 加载 {eng.total} 条（阈值 145+）\n")

    # ── 阶段 2: 家庭命名双向查询 ──
    print("【阶段 2】家庭命名双向查询")
    for code in ["mlll", "mltt", "gtt", "mlnn", "lctqf", "maolilan", "smrl", "yinyang"]:
        e = eng.lookup(code)
        assert e is not None, f"未找到 {code}"
        print(f"  {e['code']} → {e['meaning']} | {POLARITY_LABEL[e['polarity']]} | {e['category']}")

    # 反向查询
    for meaning in ["集装箱", "从体", "囡囝", "SMRL引擎"]:
        code = eng.reverse_lookup(meaning)
        assert code, f"反查失败: {meaning}"
        print(f"  [反查] {meaning} → {code}")
    print()

    # ── 阶段 3: 历史符号各家族 ──
    print("【阶段 3】历史符号家族验证")
    families = [
        ("容器符", "袋"),
        ("操作符", "照"),
        ("极性符", "阴"),
        ("层级符", "元"),
        ("审判符", "裁"),
        ("产权符", "产"),
    ]
    for cat, sample_code in families:
        group = eng.filter_by_category(cat)
        sample = eng.lookup(sample_code)
        assert sample is not None, f"缺少样本 {sample_code}"
        assert sample["category"] == cat, f"类别错误: {sample_code} 不在 {cat}"
        print(f"  {cat}: {len(group)} 条 | 样本 {sample_code} = {sample['meaning']}")
    print()

    # ── 阶段 4: 极性分布验证 ──
    print("【阶段 4】极性分布验证")
    ps = eng.polarity_stats()
    balance = eng.polarity_balance_check()
    print("  阴: {}  阳: {}  平衡: {}".format(ps['yin'], ps['yang'], ps['balanced']))
    ratio = balance['ratios']['yang'] / max(balance['ratios']['yin'], 0.001)
    print("  极性比 阳/阴 = {:.2f}".format(ratio))
    print("  状态: {}".format(balance['advice'][0]))
    # 阴符应当多于阳符（容器侧为主导）
    assert ps["yin"] > ps["yang"] * 1.2, "阴应显著多于阳（容器侧主导）"
    print("  ✅ 阴主导极性成立\n")

    # ── 阶段 5: 近义词 ──
    print("【阶段 5】近义词链接测试")
    assert eng.link_synonyms("mlll", "集装箱", "254+2", "袋子根基")
    assert eng.link_synonyms("mltt", "从体", " sacrifice后状态")
    syns = eng.get_synonyms("mlll")
    assert "254+2" in syns
    print(f"  mlll 近义词: {syns}")
    print(f"  mltt 近义词: {eng.get_synonyms('mltt')}")

    # 近义词搜索：搜"袋子根基"应能找到 mlll
    hits = eng.search("袋子根基")
    assert any(h["code"] == "mlll" for h in hits), "近义词搜索失败"
    print(f"  [搜 '袋子根基'] 命中: {[h['code'] for h in hits]}")
    print()

    # ── 阶段 6: 推荐机制 ──
    print("【阶段 6】同极性+同类别推荐")
    recs = eng.recommend("mlll", limit=5)
    print(f"  mlll 的近邻:")
    for r in recs:
        print(f"    {r['code']} = {r['meaning']} ({r['polarity']}/{r['category']})")
    # mlll 是容器/yin → 推荐应包含其他 容器/yin 条目（如 mlnn 或容器符中的袋桶罐等）
    rec_codes = {r["code"] for r in recs}
    assert any(eng.lookup(c)["category"] == "容器" for c in rec_codes if eng.lookup(c)), "推荐不含容器"
    print("  ✅ 推荐命中同类别\n")

    # ── 阶段 7: 动态新增 ──
    print("【阶段 7】动态新增条目")
    new_code = "full_test_001"
    assert eng.register(new_code, "测试新增", "balanced", "测试", "演示用新增条目")
    assert eng.lookup(new_code) is not None
    assert eng.total >= 158
    print(f"  ✅ 注册 {new_code}，当前总数: {eng.total}")

    # 撤销
    assert eng.unregister(new_code)
    assert eng.lookup(new_code) is None
    print(f"  ✅ 撤销 {new_code}，恢复总数: {eng.total}")
    print()

    # ── 阶段 8: 囡囝四态 ──
    print("【阶段 8】囡囝四态极性验证")
    states = [
        ("mlnn", "囡囝", "balanced"),
        ("mlnnn", "囡囡", "yin"),
        ("mlnnnn", "囝囝", "yang"),
        ("mlnnnnn", "囝囡", "balanced"),
    ]
    for code, name, expected_pol in states:
        e = eng.lookup(code)
        assert e["meaning"] == name
        assert e["polarity"] == expected_pol, f"{code} 极性应 {expected_pol}，实际 {e['polarity']}"
        print(f"  {code} = {name} | {POLARITY_LABEL[e['polarity']]} ✓")
    # 四态极性之和应为平衡（balanced 占一半，yin/yang 对称）
    pols = [eng.lookup(c)["polarity"] for c, _, _ in states]
    assert pols.count("yin") == 1, "四态应有 1 阴"
    assert pols.count("yang") == 1, "四态应有 1 阳"
    assert pols.count("balanced") == 2, "四态应有 2 平衡"
    print("  ✅ 囡囝四态极性对称\n")

    # ── 完成 ──
    print(HEADER)
    print("  mltt 命名密码表 · 完整实现验证通过")
    print(f"  总计: {eng.total} 条 | 极性分布健康")
    print(HEADER)


if __name__ == "__main__":
    try:
        main()
    except AssertionError as e:
        print(f"\n❌ 验证失败: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\n⚠️ 异常: {e}")
        sys.exit(2)
