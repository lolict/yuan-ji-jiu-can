#!/usr/bin/env python3
# ============================================================
# 满全法 · 三维田字格 · 寻址表与运算口诀
#
# 三维 (非 XYZ):
#   维 1 — 容器横档: 袋桶罐盆盎 钵盏盒杯壶瓶 ... (类型维)
#   维 2 — 磁带纵行: 第几行 (起→消耗→用尽→换行)
#   维 3 — 极性层  : 阳层 / 阴层 (相位维)
#
# 后缀记法: {容器}-{行}-{极层}·{段相位}
#   段相位: 起 / 消1 / 消2 / ... / 尽
#
# 身份落款：刘楚恬和我
# ============================================================

from __future__ import annotations
from dataclasses import dataclass
import sys

# ============================================================
# 64 容器寻址 (声母群 + 韵母群, 各 8 档)
# ============================================================

@dataclass
class Container:
    name: str
    capacity: int        # 每行的段数 (起 + capacity 个消耗 + 尽 = capacity+2 段)
    phase: str           # 阳/阴
    role: str            # 声母/韵母
    polarity: str        # 外向/纵向/斜出/内敛/标记/回流 / 收口/变向/包围/承载/上升/方圆

def build_64_containers() -> dict:
    """
    完整 64 容器表:
      声母 16 档 (C01..C10 横档一 + C11..C1A 横档二)
      韵母 16 档 (C21..C26 横档一 + C27..C2C 横档二)
      + 42 个扩展容器 (笔画/偏旁/功能扩展)
    """
    # 声母第一群: 唇/舌/牙/齿/喉 ... (按频率排列, 容量递增)
    shengmu_g1 = [
        ("袋", 1, "声母", "外向"),    # C01
        ("桶", 2, "声母", "纵向"),
        ("罐", 3, "声母", "斜出"),
        ("盆", 4, "声母", "内敛"),
        ("盂", 5, "声母", "标记"),
        ("盎", 6, "声母", "回流"),
        ("钵_声", 7, "声母", "收容"),  # C07
        ("盏_声", 8, "声母", "变向"),
        ("盒_声", 9, "声母", "半围"),
        ("杯_声", 10, "声母", "承载"),  # C0A
    ]
    # 声母第二群: 腭/唇齿/边/鼻/擦/半元音
    shengmu_g2 = [
        ("壶_声", 11, "声母", "上升"),  # C10
        ("瓶_声", 12, "声母", "方圆"),
        ("篓", 13, "声母", "半包"),
        ("筐", 14, "声母", "容纳"),
        ("篮", 15, "声母", "回流"),
        ("箕", 16, "声母", "斜展"),
        ("笼", 17, "声母", "深揽"),
        ("箩", 18, "声母", "筛转"),
        ("瓮", 19, "声母", "深藏"),
        ("坛", 20, "声母", "围封"),     # C19
    ]
    # 韵母群 (对应声母但极性反转)
    yunmu_g1 = [
        ("钵", 1, "韵母", "收口"),     # C21
        ("盏", 2, "韵母", "变向"),
        ("盒", 3, "韵母", "包围"),
        ("杯", 4, "韵母", "承载"),
        ("壶", 5, "韵母", "上升"),
        ("瓶", 6, "韵母", "方圆"),
        ("盎_韵", 7, "韵母", "会厌"),
    ]
    yunmu_g2 = [
        ("罐_韵", 8, "韵母", "斜包"),  # C28
        ("盆_韵", 9, "韵母", "内蕴"),
        ("盂_韵", 10, "韵母", "轻点"),
    ]

    all_defs = shengmu_g1 + shengmu_g2 + yunmu_g1 + yunmu_g2
    containers = {}
    for i, (name, cap, role, polarity) in enumerate(all_defs):
        code = f"C{i+1:02X}"
        phase = "阳" if role == "声母" else "阴"
        containers[code] = Container(name, cap, phase, role, polarity)
        containers[name] = containers[code]  # 双向索引

    return containers

CONTAINERS = build_64_containers()

# ============================================================
# 磁带模型 — 一行的消耗序列
# ============================================================

def tape_row(container_code: str, row: int) -> list[str]:
    """生成一个容器在某行的完整磁带: [起, 消1, ..., 尽]"""
    c = CONTAINERS[container_code]
    segs = [f"{c.name}-{row}·起"]
    for i in range(c.capacity):
        segs.append(f"{c.name}-{row}·消{i}" if i > 0 else f"{c.name}-{row}·消")
    segs.append(f"{c.name}-{row}·尽")
    return segs

# ============================================================
# 三维寻址后缀记法
# ============================================================

@dataclass
class Address3D:
    """一个三维田字格地址 (无 XYZ)"""
    container_code: str
    row: int
    phase: str           # 阳/阴
    segment: str         # 起/消/尽

    def suffix(self) -> str:
        """后缀记法: 如 '袋-3-阳·起'"""
        c = CONTAINERS[self.container_code]
        return f"{c.name}-{self.row}-{self.phase}·{self.segment}"

    def address_code(self) -> str:
        """数字地址: C01-3-0-0 = 容器1, 3行, 阳, 起段"""
        phase_code = "0" if self.phase == "阳" else "1"
        seg_code = {"起": "0", "消": "1", "尽": "2"}.get(self.segment, "?")
        return f"{self.container_code}-{self.row}-{phase_code}-{seg_code}"

# ============================================================
# 加法运算: 消耗 + 消耗 → 进位
# ============================================================

def addr_plus(container_code: str, row_a: int, row_b: int) -> dict:
    """
    同容器内的两行相加: 消耗合并, 用尽换行进位

    规则: 把两行磁带逐段对齐, 同时消耗
          某段用尽 → 触发换行
          优先消耗较短的, 剩余转入下一行
    """
    c = CONTAINERS[container_code]
    tape_a = tape_row(container_code, row_a)
    tape_b = tape_row(container_code, row_b)

    # 对齐消耗段 (跳过 起/尽, 只对齐 消 段)
    body_a = [s for s in tape_a if "消" in s]
    body_b = [s for s in tape_b if "消" in s]

    max_len = max(len(body_a), len(body_b))
    result = []
    carry_used = False

    for i in range(max_len):
        a_has = i < len(body_a)
        b_has = i < len(body_b)
        if a_has and b_has:
            result.append(f"({body_a[i]}⊕{body_b[i]})")
        elif a_has:
            result.append(f"余{a_body[i]}")
        elif b_has:
            result.append(f"余{body_b[i]}")

        # 当这一档用尽 (达到 capacity), 触发进位
        if i == c.capacity - 1:
            result.append(f"→[换行→{c.name}-{max(row_a,row_b)+1}]")
            carry_used = True

    return {
        "container": c.name,
        "tape_a": tape_a,
        "tape_b": tape_b,
        "addition_trace": result,
        "carry": carry_used,
        "capacity": c.capacity,
        "phase": c.phase,
        "_legal_lengths": {
            "with_carry": len(result),
            "without_carry": max_len,
        },
    }

# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("═══════════════════════════════════════════════════")
    print("  满全法三维田字格 · 寻址表与加法运算")
    print("  容器类型 × 行 × 极层 · 无 XYZ · 无字面量")
    print("  身份落款: 刘楚恬和我")
    print("═══════════════════════════════════════════════════")

    # 展示所有容器清单
    print("\n【64容器清单】")
    print(f"{'码位':6s} {'容器':8s} {'容量':4s} {'相位':4s} {'声母/韵母':8s}")
    print("─" * 45)
    for code in sorted(set(k for k in CONTAINERS if k.startswith("C"))):
        c = CONTAINERS[code]
        print(f"{code:6s} {c.name:8s} {c.capacity:4d} {c.phase:4s} {c.role:8s}")

    # 加法演示: 同容器内两行相加
    print("\n【加法演示】")
    demos = [
        ("C01", 0, 1, "袋档 + 袋档 (逢一进一)"),
        ("C02", 0, 1, "桶档 + 桶档 (逢二进一)"),
        ("C03", 1, 2, "罐档 + 罐档 (逢三进一)"),
        ("C04", 0, 0, "盆档 + 盆档 (逢四进一)"),
    ]
    for code, ra, rb, label in demos:
        print(f"\n【{label}】")
        r = addr_plus(code, ra, rb)
        print(f"  行 A : {r['tape_a']}")
        print(f"  行 B : {r['tape_b']}")
        for i, step in enumerate(r["addition_trace"]):
            print(f"    [{i:2d}] {step}")
        print(f"  进位: {'触发' if r['carry'] else '未触发'} | 容量: {r['capacity']}")

    # 三维寻址示例
    print("\n【三维寻址示例】(容器-行-极层·段)")
    samples = [
        Address3D("C01", 0, "阳", "起"),
        Address3D("C02", 3, "阴", "消"),
        Address3D("C03", 1, "阳", "尽"),
        Address3D("C04", 7, "阴", "起"),
        Address3D("C07", 0, "阳", "消"),
    ]
    for a in samples:
        print(f"  {a.suffix():20s} → 地址 {a.address_code()}")

    print("\n═══════════════════════════════════════════════════")
    print("  运算规则:")
    print("  · 同档相加, 段位对齐")
    print("  · 用尽触发换行进位")
    print("  · 叠加余数转入下一行")
    print("═══════════════════════════════════════════════════")
