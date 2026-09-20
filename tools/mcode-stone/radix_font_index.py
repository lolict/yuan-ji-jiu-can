#!/usr/bin/env python3
# ============================================================
# 满全法 · 字体自带索引层 (简化版)
#
# 身份落款: 刘楚恬和我
#
# 四种查字, 统一用 radix_3d_tianzi 的 addr/parse/route:
#   A. 地址查:   "C03-1" → 罐第1行
#   B. 拼音查:   "ch"    → 袋第1行 → glyph
#   C. 结构查:   (2笔, 封闭) → 桶档
#   D. GSUB查:   a,n     → lig.an
# ============================================================

import sys

from radix_3d_tianzi import (
    CONTAINERS, INITIAL_ROUTE, VOWEL_ROUTE, VOWEL_CLUSTER,
    addr, parse, route, add,
)

# ── 拼音全表 ──
PINYIN_INDEX = {
    "an": "LR","en": "LR","in": "LR","un": "LR","vn": "LR",
    "ai": "LR","ei": "LR","ao": "LR","ou": "SS",
    "ie": "TB","uo": "TB","io": "TB",
    "ue": "LR","ui": "LR","iu": "LR",
    "eo": "IO","oe": "IO","eu": "IO",
    "ang": "LR","eng": "LR","ing": "LR","ong": "LR",
    "ian": "LR","uan": "TB","iao": "TB","uai": "TB","iong": "IO",
    "ch": "LR","sh": "LR","zh": "LR",
    **{k: "LR" for k in [
        "aa","oo","ii","uu","ee","vv",
        "th","gh","fh","ph","ng","nk","nt","nd",
        "ed","ad","uk","ok","ek","ep",
        "ar","er","ir","or","ur","yr",
        "pr","tr","kr","br","dr","fr","gr","sr",
        "fl","gl","pl","bl","cl",
        "sk","sp","st","sm","sn","sw","tw","dw","kw","qu",
        "am","im","om","em","on","ae","ua","ia","ve",
        "iang","uang","ueng","uei","iou",
        "id","od","ud","eg","ig","ug","ag",
        "nch","nsh","nth","str","spl",
        "ld","lt","lk","lf","lm","pt","ct","ft","xt",
    ]},
}

def lookup_address(code_str):
    """A. 地址查: 容器行号 → 容器名 + 推导属性"""
    return parse(code_str)

def lookup_pinyin(pinyin):
    """B. 拼音查 → 地址 + 字形"""
    r = route(pinyin)
    if "error" in r:
        return r
    mode = PINYIN_INDEX.get(pinyin, "LR")
    return {**r, "mode": mode, "full_pinyin": pinyin}

def lookup_structure(strokes, closed):
    """C. 结构查: 笔画统计 → 容量匹配"""
    candidates = [(code, name, cap) for code,(name,cap,_) in CONTAINERS.items()]
    candidates = list(dict.fromkeys(candidates))  # 去同名容器
    if closed:
        best = min(candidates, key=lambda x: abs(x[2] - strokes + 1))
    else:
        best = min(candidates, key=lambda x: abs(x[2] + 1 - strokes))
    code, name, cap = best
    return {"container": code, "container_name": name, "capacity": cap,
            "strokes": strokes, "closed": closed}

def lookup_gsub(chars):
    """D. GSUB查: 输入序列 → 融合字形"""
    key = "".join(chars)
    if key in PINYIN_INDEX:
        return {"input": chars, "output": f"lig.{key}",
                "mode": PINYIN_INDEX[key]}
    return None

def split_pinyin(s):
    """把字符串分为地址/拼音: 含短横且C开头 = 地址, 否则 = 拼音"""
    if "-" in s and s[0] == "C":
        return "address"
    return "pinyin"

def query(key):
    """统一入口"""
    if isinstance(key, str):
        kind = split_pinyin(key)
        if kind == "address":
            return {"type": "address", **parse(key)}
        else:
            return {"type": "pinyin", **lookup_pinyin(key)}
    elif isinstance(key, (list, tuple)):
        r = lookup_gsub(key)
        return {"type": "gsub", **r} if r else {"type": "gsub", "input": key, "output": None}
    return {"error": f"未知查询: {key!r}"}

if __name__ == "__main__":
    print("═══════════════════════════════════════════════════")
    print("  满全法 · 字体自带索引 (简化版)")
    print("  地址 = 容器-行号 (所有属性推导)")
    print("  身份落款: 刘楚恬和我")
    print("═══════════════════════════════════════════════════")

    print("\n【A. 地址查】地址是数值, 属性是推导")
    for a in ["C03-1","C04-2","C02-5","C20-3"]:
        r = query(a)
        print(f"  {a:6s} → {r.get('container_name','?')}-{r.get('row','?')}"
              f"-{r.get('derived_phase','?')}·{r.get('derived_segment','?')}")

    print("\n【B. 拼音查】拼音 → 容器档")
    for py in ["an","ch","zh","yao","shuang","jiang","ba","yu"]:
        r = query(py)
        print(f"  {py:8s} → {r.get('address','?'):6s}"
              f" {r.get('suffix',''):20s} 字形={r.get('glyph','?')}")

    print("\n【C. 结构查】笔画特征 → 容器匹配")
    for s, c in [(2,True),(3,False),(1,False),(4,True)]:
        r = lookup_structure(s, c)
        print(f"  笔={s} 封={'✓' if c else '·'} → {r['container']:3s}"
              f" {r['container_name']:2s} (容={r['capacity']})")

    print("\n【D. GSUB查】输入序列 → 融合字形")
    for chars in [["a","n"],["c","h"],["b","r"],["a","o"],["r","q"]]:
        r = query(chars)
        out = r.get("output","无") or "无"
        print(f"  {' '.join(chars):6s} → {out}")

    print("\n═══════════════════════════════════════════════════")
