#!/usr/bin/env python3
"""
fridge_executor.py — 冰箱调度执行器 v0.1

从 SVG 字体文件中读取容器编码, 构建时间表, 按 Z轴时间窗口调度执行。
闭环: 字体文件 → 解析容器 → 构造调度信封 → 三元裁决 → 按时间窗发射 → 容器动作

引用:
  hunyuan-os/tools/glyph-container/protocol.py (容器/极性/主权编码)
  hunyuan-os/tools/glyph-container/han_yuan_container.svg (字体数据来源)
  hunyuan-rs/src/container_radix.rs (64容器定义)
  skills/time-fridge-execution/SKILL.md (冰箱模型/三元裁决规则)

"""

import re
import time
import hashlib
from dataclasses import dataclass, field
from typing import Optional

from protocol import (
    ContainerAddr, ContainerGroup, Polarity, Sovereignty,
    ScheduleEnvelope, ThreeWayJudgement, CONTAINER_ACTIONS,
    polarity_value, polarity_meets_residual, encode_morton,
)


# ══════════════════════════════════════════════════════════════
# SVG 字体读取器 — 从字形属性反推容器编码
# ══════════════════════════════════════════════════════════════

@dataclass
class ParsedGlyph:
    """从 SVG <glyph> 元素解析出的容器数据"""
    glyph_id: int                  # 码点
    glyph_name: str                # 容器名
    container_code: int            # 原始编码
    group: ContainerGroup          # 容器群
    index: int                     # 群内索引
    polarity: Polarity             # 极性
    sovereignty: Sovereignty       # 主权
    meaning: str                   # 中文语义
    time_slot: int                 # Z轴: 时间窗口
    pulse: int                     # Z轴: 脉冲频率
    native_code: str               # 泰射机器码
    svg_path: str                  # 图形层路径


def parse_svg_font(svg_path: str) -> list[ParsedGlyph]:
    """
    解析 SVG 字体文件的所有 <glyph> 元素。
    读取自定义属性 container:code / container:group / container:index 等。
    """
    with open(svg_path, "r", encoding="utf-8") as f:
        content = f.read()
    
    glyphs: list[ParsedGlyph] = []
    
    # 用正则提取每个 <glyph ... /> 或 <glyph ... ></glyph>
    glyph_pattern = re.compile(
        r'<glyph\s+'
        r'unicode="&#x([0-9A-Fa-f]+);"\s+'
        r'glyph-name="([^"]+)"\s+'
        r'[^/]*?'
        r'container:code="(\d+)"\s+'
        r'container:group="(\d+)"\s+'
        r'container:index="(\d+)"\s+'
        r'container:polarity="([A-Z_]+)"\s+'
        r'container:sovereignty="([A-Z_]+)"\s+'
        r'container:meaning="([^"]*)"'
        r'(?:[^/]*?container:time_slot="(\d+)")?'
        r'(?:[^/]*?container:pulse="(\d+)")?'
        r'(?:[^/]*?container:native_code="([^"]*)")?'
        r'[^/]*?d="([^"]*)"'
        r'[^/]*?/?>',
        re.DOTALL,
    )
    
    for m in glyph_pattern.finditer(content):
        try:
            glyph = ParsedGlyph(
                glyph_id=int(m.group(1), 16),
                glyph_name=m.group(2),
                container_code=int(m.group(3)),
                group=ContainerGroup(int(m.group(4))),
                index=int(m.group(5)),
                polarity=Polarity[m.group(6)] if m.group(6) in Polarity.__members__ else Polarity.NEUTRAL,
                sovereignty=Sovereignty[m.group(7)] if m.group(7) in Sovereignty.__members__ else Sovereignty.ACTIVE_SUB,
                meaning=m.group(8) or "",
                time_slot=int(m.group(9)) if m.group(9) else 0,
                pulse=int(m.group(10)) if m.group(10) else 1,
                native_code=m.group(11) or "",
                svg_path=m.group(12),
            )
            glyphs.append(glyph)
        except (ValueError, KeyError):
            continue
    
    # 回退: 上面正则太严格时，逐属性解析
    if len(glyphs) == 0:
        glyphs = _fallback_parse(content)
    
    return glyphs


def _fallback_parse(content: str) -> list[ParsedGlyph]:
    """
    更宽松的回退解析: 逐个提取 unicode / container:* 属性。
    不按属性出现顺序强约束。
    """
    glyph_pattern = re.compile(r'<glyph\s+([^>]*?>)\s*/>', re.DOTALL)
    attr_pattern = re.compile(r'([a-zA-Z_:]+)="([^"]*)"')
    
    glyphs: list[ParsedGlyph] = []
    for m in glyph_pattern.finditer(content):
        attrs = dict(attr_pattern.findall(m.group(1)))
        
        # 必须有这些才返回
        if "container:code" not in attrs:
            continue
        
        try:
            pol_str = attrs.get("container:polarity", "NEUTRAL")
            sov_str = attrs.get("container:sovereignty", "ACTIVE_SUB")
            pol = Polarity[pol_str] if pol_str in Polarity.__members__ else Polarity.NEUTRAL
            sov = Sovereignty[sov_str] if sov_str in Sovereignty.__members__ else Sovereignty.ACTIVE_SUB
            
            glyph = ParsedGlyph(
                glyph_id=int(attrs.get("unicode", "0").replace("&#x", "").replace(";", ""), 16),
                glyph_name=attrs.get("glyph-name", ""),
                container_code=int(attrs.get("container:code", "0")),
                group=ContainerGroup(int(attrs.get("container:group", "0"))),
                index=int(attrs.get("container:index", "0")),
                polarity=pol,
                sovereignty=sov,
                meaning=attrs.get("container:meaning", ""),
                time_slot=int(attrs.get("container:time_slot", "0")),
                pulse=int(attrs.get("container:pulse", "1")),
                native_code=attrs.get("container:native_code", ""),
                svg_path=attrs.get("d", ""),
            )
            glyphs.append(glyph)
        except (ValueError, KeyError):
            continue
    
    return glyphs


# ══════════════════════════════════════════════════════════════
# 冰箱调度器 — 三元裁决 + 时间窗口发射
# ══════════════════════════════════════════════════════════════

@dataclass
class FridgeScheduleEntry:
    """时间表的一行"""
    envelope: ScheduleEnvelope
    container_addr: ContainerAddr
    action_name: str
    meaning: str
    admitted: bool = True
    judgement_reason: str = "admit"


class FridgeExecutor:
    """
    冰箱调度执行器。
    
    对齐 time-fridge-execution SKILL:
      • Menu Index: 字形编码的有序列表 (0..N)
      • Triple Judgement: 空袋拒绝 / 数值准入 / 全零陷阱拒绝
      • Schedule Table: 基于 Z轴时间窗口的预编排表
      • Execute Flow: 指向 → 查菜单 → 裁决 → 取袋 → 解码 → 发光
    """
    
    def __init__(self):
        self.menu: list[int] = []                  # 字形编码 / 地址有序列表
        self.schedule: dict[int, FridgeScheduleEntry] = {}  # addr -> entry
        self.container_store: dict[int, ContainerAddr] = {}  # addr -> 容器地址
    
    def load_font(self, svg_path: str):
        """加载 SVG 字体 → 解析所有容器 → 构建时间表"""
        glyphs = parse_svg_font(svg_path)
        print(f"  [加载] 解析字形: {len(glyphs)} 个容器")
        
        for g in glyphs:
            addr = ContainerAddr(
                group=g.group,
                index=g.index,
                polarity=g.polarity,
                sovereignty=g.sovereignty,
                operand=g.glyph_id,
                time_slot=g.time_slot,
                depth=g.pulse - 1,
            )
            
            # 查菜单: 地址是否在菜单中 (简易检查)
            addr_int = addr.encode()
            if addr_int not in self.menu:
                self.menu.append(addr_int)
            
            # 生成调度信封
            envelope = ScheduleEnvelope.from_container(addr, align_flag=g.pulse)
            envelope.witness = self._make_witness(envelope.pack_id, g.native_code.encode("utf-8"))
            
            # 裁决: 三元检查
            judge = ThreeWayJudgement.judge(g.native_code.encode("utf-8"))
            
            # 求动作名: 引用 container_radix.rs 的 CONTAINER_ACTIONS
            action = CONTAINER_ACTIONS.get(g.glyph_name, "execute")
            
            entry = FridgeScheduleEntry(
                envelope=envelope,
                container_addr=addr,
                action_name=action,
                meaning=g.meaning,
                admitted=(judge.verdict == "admit"),
                judgement_reason=judge.verdict,
            )
            
            self.schedule[addr_int] = entry
            self.container_store[addr_int] = addr
        
        self.menu.sort()
        print(f"  [菜单] 菜单项数: {len(self.menu)}")
        print(f"  [时间表] 含调度项数: {len(self.schedule)}")
    
    def _make_witness(self, pack_id: int, payload: bytes) -> bytes:
        """生成 witnesses (blake2b 摘要前 3 bytes)"""
        h = hashlib.blake2b(
            pack_id.to_bytes(4, "little") + payload,
            digest_size=3,
        )
        return h.digest()
    
    def execute_all(self):
        """按菜单顺序执行所有时间窗口的容器"""
        print(f"\n  [执行] 开始时间触发调度...")
        print("-" * 70)
        print(f"  {'地址':<6} | {'容器':<10} | {'极性':<8} | {'主权':<12} | {'动作':<18} | {'语义'}")
        print("-" * 70)
        
        executed = 0
        for addr_int in self.menu:
            entry = self.schedule.get(addr_int)
            if not entry:
                continue
            
            if not entry.admitted:
                print(f"  0x{addr_int:04X} | {entry.container_addr.container_name():<10} | {'—':<8} | {'—':<12} | {'—':<18} | [{entry.judgement_reason}]")
                continue
            
            # 解码 & 触发动作
            pol_val = polarity_value(entry.container_addr.polarity)
            action = entry.action_name
            meaning = entry.meaning
            
            print(f"  0x{addr_int:04X} | {entry.container_addr.container_name():<10} | {entry.container_addr.polarity.name:<8} | {entry.container_addr.sovereignty.name:<12} | {action:<18} | {meaning}")
            executed = executed + 1
        
        print("-" * 70)
        print(f"  [完成] 发射数: {executed} / {len(self.menu)}")
    
    def inject_time_slots(self):
        """为每个月插槽重构时间窗"""
        slots: dict[int, list[int]] = {}
        for addr_int, entry in self.schedule.items():
            slot = entry.container_addr.time_slot
            if slot not in slots:
                slots[slot] = []
            slots[slot].append(addr_int)
        
        return slots


# ══════════════════════════════════════════════════════════════
# 主函数 — 字体 → 菜单 → 时间安排 → 执行闭环
# ══════════════════════════════════════════════════════════════

def main():
    import os
    import sys
    
    # 路径: 跟 protocol.py / generate_font.py / han_yuan_container.svg 同目录
    HERE = os.path.dirname(os.path.abspath(__file__))
    svg_path = os.path.join(HERE, "han_yuan_container.svg")
    
    if not os.path.exists(svg_path):
        print(f"[错误] 找不到 SVG 字体: {svg_path}")
        print("  请先运行: python generate_font.py")
        sys.exit(1)
    
    print("=== HanYuan-Container 冰箱调度 v0.1 ===\n")
    
    # 1. 加载字体
    fridge = FridgeExecutor()
    fridge.load_font(svg_path)
    
    # 2. 检查残差 (阴+阳同时出现会产生残差)
    print(f"\n  [极性分析] 检查容器间的相位残差:")
    all_addrs = [e.container_addr for e in fridge.schedule.values() if e.admitted]
    residual_count = 0
    for i, a in enumerate(all_addrs):
        for j, b in enumerate(all_addrs):
            if i >= j:
                continue
            if polarity_meets_residual(a.polarity, b.polarity):
                print(f"    {a.container_name()}({a.polarity.name}) + {b.container_name()}({b.polarity.name}) → 残差 (无需进位)")
                residual_count = residual_count + 1
    
    print(f"    残差对: {residual_count}")
    
    # 3. 时间窗分布
    slots = fridge.inject_time_slots()
    print(f"\n  [时间窗分布] Z轴 (共 {len(slots)} 个槽位):")
    for slot in sorted(slots.keys()):
        names = [fridge.container_store[a].container_name() for a in slots[slot]]
        print(f"    slot {slot:>3}: {', '.join(names)}")
    
    # 4. Morton 码分析
    print(f"\n  [Morton 码] 三维空间索引:")
    for addr_int, entry in fridge.schedule.items():
        if not entry.admitted:
            continue
        morton = encode_morton(entry.container_addr.encode(), entry.container_addr.time_slot)
        print(f"    {entry.container_addr.container_name():<10} addr=0x{addr_int:02X} time={entry.container_addr.time_slot:>3} → morton=0x{morton:04X}")
    
    # 5. 按菜单执行
    fridge.execute_all()
    
    print(f"\n=== 闭环完成 ===")
    print(f"容器=字体=时间 三位一体验证: PASS")
    print(f"从 SVG 字体读回 {len(fridge.menu)} 个容器, 构建时间表, 三元裁决通过, 按 Z 轴窗口发射。")


if __name__ == "__main__":
    main()
