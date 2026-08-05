"""
囡囝共生 · 病理诊断 (Paibing)
============================

检查关系网络的健康状态:

  双编码 (主路由表)
    14 对声母镜像 → 检查双向可达
    每对 (A,B) 如果同时存在 → ✔ 健康
    只有 A 没有 B      → ✖ 断裂
    重复 A 自配对       → ✖ 自重复 (疑似癌前)

  三编码 (归属路径矩阵)
    9 组 × 每一组 6 排列 → 完整路径
    6/6 全通            → ✔ 健康
    4~5/6              → ⚠ 疑似
    2~3/6              → ✖ 病变
    <2/6               → ✖ 癌变

  四档结论:
    /health  → 治愈 (建议: 保持)
    /suspect → 疑似 (建议: 观察 + 加强同步)
    /disease → 病变 (建议: 镜像修复 + 转置)
    /cancer  → 癌变 (建议: 清创 + starter kit)

动词:
    (transposition) -- 把他人的对称音反过来
    (mirror repair) -- 补足缺少声母对中
"""
import re
from typing import List, Dict, Tuple
from enum import Enum

class Health(Enum):
    HEALTH  = "healthy"
    SUSPECT = "suspected"
    DISEASE = "diseased"
    CANCER  = "cancer"

# ── 双编码声母镜像 (14 对) ───────────────────────────
DOUBLE_PAIRS: List[Tuple[str, str]] = [
    ("v", "y"),   # vy ↔ yv
    ("t", "f"),   # tf ↔ ft
    ("c", "k"),   # ck ↔ kc
    ("z", "s"),   # zs ↔ sz
    ("d", "b"),   # db ↔ bd
    ("j", "g"),   # jg ↔ gj
    ("q", "p"),   # qp ↔ pq
    ("w", "m"),   # wm ↔ mw
    ("@", "o"),   # @o ↔ o@
    ("u", "h"),   # uh ↔ hu
    ("r", "l"),   # rl ↔ lr
    ("η", "n"),   # ηn ↔ nη
    ("i", "x"),   # ix ↔ xi
    ("sh", "ch"), # sh ↔ ch
]

# ── 三编码 9 组 (每组包含 3 个声母) ──────────────────
TRIPLE_GROUPS: List[Tuple[str, str, str]] = [
    ("l", "n", "m"),   # lnm
    ("i", "x", "w"),   # ixw
    ("u", "h", "a"),   # uha
    ("v", "y", "t"),   # vyt
    ("c", "k", "e"),   # cke
    ("z", "s", "r"),   # zsr
    ("j", "d", "b"),   # jdb
    ("q", "p", "f"),   # qpf
    ("g", "o", "@"),   # go@
]


def _extract_initials(token: str) -> List[str]:
    """从一个字符串中提取声母序列"""
    t = token.lower().strip()
    # 双字母声母优先
    initials = []
    i = 0
    while i < len(t):
        if i + 1 < len(t) and t[i:i+2] in ("sh", "ch", "zh"):
            initials.append(t[i:i+2])
            i += 2
        else:
            initials.append(t[i])
            i += 1
    return initials


def check_double_code(tokens: List[str]) -> Dict:
    """
    双编码主路由表病理检查

    返回:
      {
         pair: "vy",    # 哪一对
         present: "y",   # 谁的声母存在
         missing: "v",   # 谁的声母缺失
         status: "broken" | "healthy" | "self-dup"
      }
    """
    results = []
    all_initials = set()
    for tok in tokens:
        for ch in _extract_initials(tok):
            all_initials.add(ch)

    for a, b in DOUBLE_PAIRS:
        has_a = a in all_initials
        has_b = b in all_initials
        if has_a and has_b:
            status = "healthy"
        elif has_a and not has_b:
            status = "broken"
        elif has_b and not has_a:
            status = "broken"
        else:
            status = "absent"
        results.append({
            "pair": f"{a}{b}",
            "has_a": has_a,
            "has_b": has_b,
            "status": status,
            "missing": b if has_a and not has_b else (a if has_b and not has_a else None),
            "pair_label": f"{a}↔{b}",
        })

    healthy_count = sum(1 for r in results if r["status"] == "healthy")
    return {
        "check": "double",
        "total_pairs": len(DOUBLE_PAIRS),
        "healthy": healthy_count,
        "broken": sum(1 for r in results if r["status"] == "broken"),
        "absent": sum(1 for r in results if r["status"] == "absent"),
        "score": healthy_count / len(DOUBLE_PAIRS),
        "details": results,
    }


def check_triple_code(tokens: List[str]) -> Dict:
    """
    三编码归属路径矩阵病理检查

    对每个三声母组,它应该有 6 种排列 (3! = 6)
    实际存在的排列数 / 6 = 健康度
    """
    from itertools import permutations

    all_initials = set()
    for tok in tokens:
        for ch in _extract_initials(tok):
            all_initials.add(ch)

    group_results = []
    for group in TRIPLE_GROUPS:
        group_set = set(group)
        overlap = group_set & all_initials
        if len(overlap) < 2:
            reachable = 0
        elif len(overlap) == 2:
            reachable = 2
        else:  # 3
            reachable = 6

        score = reachable / 6
        health = (
            Health.HEALTH  if score >= 0.99 else
            Health.SUSPECT if score >= 0.66 else
            Health.DISEASE if score >= 0.33 else
            Health.CANCER
        )
        group_results.append({
            "group": "".join(group),
            "overlap": list(overlap),
            "reachable_permutations": reachable,
            "score": score,
            "health": health.value,
        })

    avg_score = sum(g["score"] for g in group_results) / len(group_results)
    overall = (
        Health.HEALTH  if avg_score >= 0.99 else
        Health.SUSPECT if avg_score >= 0.66 else
        Health.DISEASE if avg_score >= 0.33 else
        Health.CANCER
    )

    return {
        "check": "triple",
        "total_groups": len(TRIPLE_GROUPS),
        "avg_score": avg_score,
        "overall_health": overall.value,
        "details": group_results,
    }


def diagnose(name_a: str, name_b: str = None, extra: List[str] = None) -> Dict:
    """
    排病诊断的主入口

    用法:
        diagnose("刘楚恬", "lolict")
        diagnose("莫刘连理萝莉兰零离...")

    返回完整诊断报告
    """
    tokens = [name_a]
    if name_b:
        tokens.append(name_b)
    if extra:
        tokens.extend(extra)

    d = check_double_code(tokens)
    t = check_triple_code(tokens)

    # 治愈建议
    suggestions = []
    if d["broken"] == 0 and t["avg_score"] >= 0.99:
        suggestions.append("健康:保持当前共生形态")
    if d["broken"] > 0:
        broken_pairs = [r["pair"] for r in d["details"] if r["status"] == "broken"]
        suggestions.append(f"双编码断裂 {d['broken']} 对: 镜像修复 {broken_pairs}")
    if t["avg_score"] < 0.66:
        suggestions.append(f"三编码路径不足 ({t['avg_score']:.0%}): 建议扩展音素范围")
    if t["avg_score"] < 0.33:
        suggestions.append("⚠ 严重: 建议执行 清创 流程,必要时使用 starter kit 重建")

    overall = (
        Health.HEALTH if d["broken"] == 0 and t["avg_score"] >= 0.8 else
        Health.SUSPECT if d["broken"] <= 2 and t["avg_score"] >= 0.5 else
        Health.DISEASE if t["avg_score"] >= 0.25 else
        Health.CANCER
    )

    return {
        "overall_health": overall.value,
        "input_tokens": tokens,
        "double_check": d,
        "triple_check": t,
        "suggestions": suggestions,
    }


# ── 测试 ─────────────────────────────────────────────
def _test():
    result = diagnose("刘楚恬", "lolict", ["莫刘连理萝莉兰零"])
    print("[OK] paibing diagnose:")
    print(f"  overall: {result['overall_health']}")
    print(f"  double:  {result['double_check']['healthy']}/{result['double_check']['total_pairs']} healthy")
    print(f"  triple:  avg={result['triple_check']['avg_score']:.2%}")
    for s in result["suggestions"]:
        print(f"  → {s}")


if __name__ == "__main__":
    _test()
