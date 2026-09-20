#!/usr/bin/env python3
# ============================================================
# 满全法 · 三维田字格寻址
#
# 身份落款: 刘楚恬和我
#
# 地址只有两段: 容器 + 行号
#   C03-1    罐第1行
#   C02-5    桶第5行
#   C20-2    钵第2行
#
# 极性 = 行号推导 (奇=阳, 偶=阴)
# 段位 = 容器容量推导
# 容量 = 容器自带 (乘法口诀表嵌入容器表)
#
# 纪律:
#   地址是数值, 不是参数包
#   参数是属性的推导, 不写在地址里
#   进制即容器, 容量即乘, 行号即加
# ============================================================

import sys

# ============================================================
# 容器表: 编码 → (名称, 容量, 极性)
# 容量 = 乘法口诀表嵌入
# 极性 = 容器本身的阴阳属性
# ============================================================

CONTAINERS = {
    "C01": ("袋", 3, "阳"),   # b p m f
    "C02": ("桶", 2, "阴"),   # d t n l
    "C03": ("罐", 3, "阳"),   # g k h
    "C04": ("盆", 4, "阴"),   # j q x
    "C05": ("盂", 2, "阳"),   # z c s r
    "C06": ("盎", 4, "阴"),   # y w v
    "C10": ("袋", 3, "阳"),   # zh ch sh
    "C11": ("桶", 2, "阴"),
    "C12": ("罐", 3, "阳"),
    "C13": ("盆", 4, "阴"),
    "C14": ("盂", 2, "阳"),
    "C15": ("盎", 4, "阴"),
    "C16": ("钵", 3, "阳"),
    "C17": ("盏", 2, "阴"),
    "C18": ("盒", 4, "阳"),
    "C19": ("杯", 3, "阴"),
    "C20": ("钵", 3, "阳"),   # a
    "C21": ("盏", 2, "阴"),   # e
    "C22": ("盒", 4, "阳"),   # i
    "C23": ("杯", 3, "阴"),   # o
    "C24": ("壶", 2, "阳"),   # u
    "C25": ("瓶", 3, "阴"),   # ü
}

# ── 声母/韵母路由 ──
INITIAL_ROUTE = {
    "b":"C01","p":"C01","m":"C01","f":"C01",
    "d":"C02","t":"C02","n":"C02","l":"C02",
    "g":"C03","k":"C03","h":"C03",
    "j":"C04","q":"C04","x":"C04",
    "z":"C05","c":"C05","s":"C05","r":"C05",
    "y":"C06","w":"C06","v":"C06",
    "zh":"C10","ch":"C10","sh":"C10",
}
VOWEL_ROUTE = {
    "a":"C20","e":"C21","i":"C22","o":"C23","u":"C24",
}
VOWEL_CLUSTER = {
    "ai": ["C20","C22"], "ei": ["C21","C22"], "ao": ["C20","C23"],
    "ou": ["C23","C24"], "ie": ["C22","C21"], "ue": ["C24","C21"],
    "ui": ["C24","C22"], "iu": ["C22","C24"], "uo": ["C23","C24"],
    "ia": ["C22","C20"], "ua": ["C24","C20"], "ve": ["C25","C21"],
    "an": ["C20","C02"], "en": ["C21","C02"], "in": ["C22","C02"],
    "un": ["C24","C02"], "vn": ["C25","C02"],
    "ang": ["C20","C03"],"eng": ["C21","C03"],"ing": ["C22","C03"],
    "ong": ["C23","C03"],
    "ian": ["C22","C20","C02"],"uan": ["C24","C20","C02"],
    "iao": ["C22","C20","C23"],"uai": ["C24","C20","C23"],
    "iong": ["C22","C23","C03"],
    "iang": ["C22","C20","C03"],"uang": ["C24","C20","C03"],
    "ueng": ["C24","C21","C03"],
}

def addr(container_code, row):
    """地址 = 容器 + 行号 (两个字段, 地址是数值)"""
    return f"{container_code}-{row}"

def parse(address):
    """解析地址 → 容器编码 + 行号 + 推导属性"""
    parts = address.split("-")
    if len(parts) != 2:
        return {"error": f"地址格式应为 Cx-Y, 得到: {address}"}
    code, row = parts[0], int(parts[1])
    c = CONTAINERS.get(code)
    if not c:
        return {"error": f"未知容器 {code}"}
    name, cap, base_phase = c
    # 极性 = 行号奇偶 (阳=奇, 偶=阴), 叠加容器极性
    row_phase = "阳" if row % 2 == 1 else "阴"
    # 段位 = 容器容量推导 (当前行消耗进度)
    seg = "起" if row % cap == 1 else ("消" if row % cap == 2 else "尽")
    return {
        "container": code,
        "container_name": name,
        "capacity": cap,
        "base_phase": base_phase,
        "row": row,
        "derived_phase": row_phase,
        "derived_segment": seg,
        "suffix": f"{name}-{row}-{row_phase}·{seg}",
    }

def add(address, units):
    """
    地址加法: 消耗单位数。
    做加法时, 容量决定要不要进位换行。
    容量 = 乘法口诀表: 袋=3 代表一行能装 3 单位, 装满就进位。
    """
    p = parse(address)
    if "error" in p:
        return p
    code, row, cap = p["container"], p["row"], p["capacity"]
    # 行内剩余
    used_in_row = row % cap  # 0=新行起, 1=消, 2=尽
    if used_in_row == 0:
        used_in_row = cap
    remain = cap - used_in_row
    # 如果加的不超行内剩余 → 不进位
    if units <= remain:
        return {
            "result": addr(code, row),
            "suffix": CONTAINERS[code][0] + f"-{row}-{'阳' if row%2==1 else '阴'}·{ {1:'起',2:'消',0:'尽'}[row%cap or cap] }",
            "no_carry": True,
        }
    # 超出 → 进位换行
    new_row = row + 1
    return {
        "result": addr(code, new_row),
        "suffix": CONTAINERS[code][0] + f"-{new_row}-{'阳' if new_row%2==1 else '阴'}·起",
        "carry": True,
        "new_row": new_row,
    }

def route(pinyin):
    """
    拼音 → 地址: 声母定容器, 韵母定行号。
    纯韵母无声母 → 行号 = 1, 容器 = 韵母档。
    纯声母无韵母 → 行号 = 1。
    """
    # 拆声母
    init = None
    vowel = pinyin
    for i in ["zh","ch","sh"]:
        if pinyin.startswith(i):
            init, vowel = i, pinyin[len(i):]
            break
    if not init:
        for i in ["b","p","m","f","d","t","n","l","g","k","h",
                  "j","q","x","z","c","s","r","y","w"]:
            if pinyin.startswith(i):
                init, vowel = i, pinyin[len(i):]
                break

    init_code = INITIAL_ROUTE.get(init) if init else None

    # 行号
    if vowel:
        chain = VOWEL_CLUSTER.get(vowel, [VOWEL_ROUTE.get(vowel, "C20")])
        row = len(chain)  # 韵母复杂度 → 行数
    else:
        row = 1

    code = init_code or VOWEL_ROUTE.get(vowel[:1] if vowel else "a", "C20")
    a = parse(addr(code, row))
    return {
        **a,
        "pinyin": pinyin,
        "initial": init or "-",
        "vowel": vowel or "-",
        "address": addr(code, row),
        "glyph": f"lig.{pinyin}",
    }

# ============================================================
# CLI
# ============================================================

if __name__ == "__main__":
    print("═══════════════════════════════════════════════════")
    print("  满全法 · 容器寻址")
    print("  地址 = 容器 + 行号 (其余推导)")
    print("  身份落款: 刘楚恬和我")
    print("═══════════════════════════════════════════════════")

    print("\n【地址格式: 容器-行号 (极性/段位推导)】")
    for a in ["C01-1","C01-2","C04-1","C02-3","C20-2","C22-4"]:
        p = parse(a)
        print(f"  {a:6s} → {p['container_name']}-{p['row']}-{p['derived_phase']}·{p['derived_segment']:2s}  "
              f"(容={p['capacity']}, 基={p['base_phase']})")

    print("\n【地址加法 (容量=乘, 超容量=进位换行)】")
    for a0, addends in [("C01-1",[1,2,3,5]), ("C02-1",[1,2,3]), ("C04-1",[1,3,6])]:
        for u in addends:
            r = add(a0, u)
            sign = " + " + str(u)
            print(f"  {a0}{sign:5s} → {r.get('result','?'):6s} {r.get('suffix',''):20s} "
                  f"{'(进位!)' if r.get('carry') else ''}")

    print("\n【拼音 → 地址】")
    for py in ["an","ch","zh","yu","yao","shuang","jiang","ba","lü","er"]:
        r = route(py)
        init_c = CONTAINERS.get(INITIAL_ROUTE.get(r['initial']), (r['initial'],0,''))[0] if r['initial'] != '-' else '-'
        print(f"  {py:8s} → {r['address']:6s} {r['suffix']:20s} 声母={r['initial']:2s} 韵={r['vowel']}")

    print("\n═══════════════════════════════════════════════════  ")
    print("  地址 = 数值 = 容器编码 + 行号")
    print("  极性/段位 = 属性 = 行号推导 (偶阴奇阳)")
    print("  容量 = 乘法口诀表 (容器自带)")
    print("═══════════════════════════════════════════════════  ")
