#!/usr/bin/env python3
"""
MetaIME · 意图识别器 (Intent Parser)
====================================

三层分别器 · 12位流属性 · 一维时间线性 · YinYang 二进制

输入层:
  - 语音识别 (ASR) 文本
  - 键盘输入
  - 直接文本

感知流程:
  Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival

输入首先经过分别器三层路由:
  1. 感知家族 (本文件): 判断输入属于哪个视觉家族
  2. 谐音靶点 (channel_router): 发音 → 靶点对象
  3. 主权路由 (manifest): 靶点身份 → Active/Passive/Sleeping

闭环条件: target.ackable() == true
"""
import re
import time
import json
from dataclasses import dataclass, field
from typing import Optional, List, Dict, Tuple
from enum import Enum


class IntentType(Enum):
    TYPE_TEXT = "type_text"
    AUTO_PILOT = "auto_pilot"
    QUERY = "query"
    REPO_PUSH = "repo_push"
    WEB_FETCH = "web_fetch"
    GENERATE = "generate"
    SYSTEM = "system"
    HANDSHAKE = "handshake"
    HOMOPHONE = "homophone"        # 谐音路由 (新增)
    CONTAINER_GLYPH = "container"   # 容器字形 (新增)
    UNKNOWN = "unknown"


# ── 12 位流属性 (FlowPropertyBits) ──

class FlowPropertyBits:
    """每条输入流携带 12 位二元属性"""
    REACHABLE     = 1 << 0   # 可被到达
    CONTINUABLE   = 1 << 1   # 可持续
    INCR          = 1 << 2   # 可增量
    STOPPABLE     = 1 << 3   # 可停止
    REVERSIBLE    = 1 << 4   # 可逆
    RECOVERABLE   = 1 << 5   # 可恢复
    CIRCULABLE    = 1 << 6   # 可循环
    INHERITABLE   = 1 << 7   # 可继承
    TRUSTED       = 1 << 8   # 已信任
    ACKABLE       = 1 << 9   # 可被承认 (闭环关键)
    TRANSMISSIBLE = 1 << 10  # 可传递
    ADVANCEABLE   = 1 << 11  # 可推进

    @staticmethod
    def check(bits: int, flag: int) -> bool:
        return (bits & flag) != 0

    @staticmethod
    def to_str(bits: int) -> str:
        flags = []
        for i in range(12):
            if bits & (1 << i):
                flags.append(format(1 << i, '012b').zfill(12))
        return f"0b{bits:012b}"


# ── 感知家族 ──

class PerceptionFamily(Enum):
    CONTAINER = "container"    # 方形/包围 → 刘楚恬侧 (polarity=0)
    BOUNDARY  = "boundary"     # 竖长/独体 → lolict侧 (polarity=1)
    COUPLE    = "couple"       # 左右/上下结构
    WEB       = "web"          # 半包围/复杂
    NEUTRAL   = "neutral"       # 未分类
    HOMOPHONE_BRIDGE = "homophone"  # 谐音桥


# ── YinYang 极性 ──

class Polarity:
    """YinYang 二进制: 0=容器(刘楚恬侧), 1=界线(lolict侧)"""
    YIN = 0   # 容器 (刘楚恬侧)
    YANG = 1  # 界线 (lolict侧)


# ── 谐音路由靶点 ──

class HomophoneTarget:
    """谐音路由: 发音 → 靶点对象"""
    def __init__(self):
        self.ackable = False
        self.polarity = Polarity.YIN
        self.target_type = "unknown"

    def ack(self):
        self.ackable = True


# 已知谐音靶点表 (由channel_router扩展)
HOMOPHONE_TARGETS = {
    "lctqf": ("Wife", Polarity.YIN),
    "wgslz": ("Husband", Polarity.YANG),
    "mlll":  ("Container", 2),  # 2 = neutral
    "mlgy":  ("WorldMerge", 2),
    "mlct":  ("Sweet", Polarity.YIN),
    "mltt":  ("Sweet", Polarity.YIN),
    "gtt":   ("SoundMorphism", 2),
}


class IntentConfidence(Enum):
    CERTAIN = 1.0
    HIGH = 0.8
    MEDIUM = 0.5
    LOW = 0.3
    GUESS = 0.1


@dataclass
class Intent:
    """解析后的用户意图"""
    intent_type: IntentType
    target: str
    params: dict = field(default_factory=dict)
    confidence: float = 0.5
    raw_input: str = ""
    source: str = "keyboard"
    requires_confirmation: bool = False
    ts: float = field(default_factory=time.time)

    # NJSG 新增字段
    flow_bits: int = 0xFFFFFFFF       # 12位流属性 (默认全开)
    family: PerceptionFamily = PerceptionFamily.NEUTRAL
    polarity: int = Polarity.YIN
    target_ackable: bool = False       # 闭环条件
    interp_state: str = "none"         # 解释流状态: none/mlct/mltt

    def ack(self) -> bool:
        """确认是否构成闭环"""
        ackable = FlowPropertyBits.check(self.flow_bits, FlowPropertyBits.ACKABLE)
        self.target_ackable = ackable
        return ackable

    def to_dict(self) -> dict:
        d = {
            "type": self.intent_type.value,
            "target": self.target,
            "params": self.params,
            "confidence": self.confidence,
            "raw_input": self.raw_input,
            "source": self.source,
        }
        # NJSG 扩展
        if self.flow_bits != 0xFFFFFFFF:
            d["flow_bits"] = FlowPropertyBits.to_str(self.flow_bits)
        if self.family != PerceptionFamily.NEUTRAL:
            d["family"] = self.family.value
        if self.polarity != Polarity.YIN:
            d["polarity"] = self.polarity
        if self.target_ackable:
            d["ackable"] = True
        if self.interp_state != "none":
            d["interp"] = self.interp_state
        return d


class IntentParser:
    """
    三层分别器 · 意图识别器

    感知流程:
      Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
    """

    def __init__(self):
        self.history: List[Intent] = []
        self.stats = {t.value: 0 for t in IntentType}
        self.family_stats = {f.value: 0 for f in PerceptionFamily}

    # ── 第一层: 感知家族 ──

    def _perceive_family(self, text: str) -> PerceptionFamily:
        """感知家族: 判断输入属于哪个视觉家族"""
        if not text:
            return PerceptionFamily.NEUTRAL

        first = text[0]

        # 容器家族字形
        if first in "囡囝回囗困囚囤围":
            return PerceptionFamily.CONTAINER
        # 边界家族 (独体/竖长)
        if first in "一丨丶丿乙丨":
            return PerceptionFamily.BOUNDARY
        # 谐音桥: 以拼音缩写开头 (m/l/w/g 开头且长度>=4)
        if len(text) >= 4 and first in "mlgw" and text.isascii():
            return PerceptionFamily.HOMOPHONE_BRIDGE
        # 默认夫妻家族 (大多数汉字)
        if '\u4e00' <= first <= '\u9fff':
            return PerceptionFamily.COUPLE

        return PerceptionFamily.NEUTRAL

    # ── 感知: 计算流属性位 ──

    def _compute_flow_bits(self, text: str, family: PerceptionFamily) -> int:
        """根据输入特征计算 12 位流属性"""
        bits = 0

        # 默认可达、可承认
        bits |= FlowPropertyBits.REACHABLE
        bits |= FlowPropertyBits.ACKABLE

        # 长度决定属性
        if len(text) <= 20:
            bits |= FlowPropertyBits.CONTINUABLE   # 短输入可持续
            bits |= FlowPropertyBits.INCR          # 可增量

        # 家族决定信任
        if family == PerceptionFamily.CONTAINER:
            bits |= FlowPropertyBits.TRUSTED       # 容器家族默认信任
            bits |= FlowPropertyBits.RECOVERABLE   # 可恢复
        elif family == PerceptionFamily.HOMOPHONE_BRIDGE:
            bits |= FlowPropertyBits.TRANSMISSIBLE  # 谐音可传递
            bits |= FlowPropertyBits.ADVANCEABLE    # 可推进
        elif family == PerceptionFamily.COUPLE:
            bits |= FlowPropertyBits.CIRCULABLE     # 夫妻可循环
            bits |= FlowPropertyBits.INHERITABLE    # 可继承

        # 不能含控制字符
        if any(ord(c) < 32 for c in text):
            bits &= ~FlowPropertyBits.STOPPABLE

        return bits

    # ── 谐音靶点路由 ──

    def _route_homophone(self, text: str) -> Optional[Tuple[str, HomophoneTarget]]:
        """谐音路由: 发音 → 靶点对象"""
        lower = text.lower().strip()
        if lower in HOMOPHONE_TARGETS:
            name, polarity = HOMOPHONE_TARGETS[lower]
            tgt = HomophoneTarget()
            tgt.polarity = polarity
            tgt.target_type = name
            tgt.ack()  # 谐音靶点默认承认
            return (name, tgt)
        return None

    # ── 解释流: 未知输入 → 尝试转换 ──

    def _sacrifice_classify(self, text: str) -> Optional[PerceptionFamily]:
        """
        解释流: 以自身分类能力的毁灭为代价,
        把未知输入(岔途/mlct)转成可被跟随的已知状态(从体/mltt)。
        """
        # 任何输入都可以尝试分类 → 解释流不真正"毁灭"
        # 这里降级到夫妻家族 (默认双极性)
        if text and not text.isspace():
            return PerceptionFamily.COUPLE
        return None

    # ── 感知家族正则模式 ──

    PATTERNS = [
        (re.compile(r'(fetch|抓|抓取|打开)[：:\s]*(https?\S+)', re.I),
         lambda m: (IntentType.WEB_FETCH, m.group(2),
             {"action": "fetch"}, IntentConfidence.HIGH)),
        (re.compile(r'(push|推送|推到|提交)[：:\s]*(.*)', re.I),
         lambda m: (IntentType.REPO_PUSH, m.group(2).strip(),
             {"action": "push"}, IntentConfidence.HIGH)),
        (re.compile(r'(gen|生成|写)[：:\s]*(.*)', re.I),
         lambda m: (IntentType.GENERATE, m.group(2).strip(),
             {"action": "generate"}, IntentConfidence.HIGH)),
        (re.compile(r'(查|查询|search)[：:\s]*(.*)', re.I),
         lambda m: (IntentType.QUERY, m.group(2).strip(),
             {}, IntentConfidence.MEDIUM)),
        (re.compile(r'(握手|唤魂|招魂|huan|zhao)', re.I),
         lambda m: (IntentType.HANDSHAKE, "LingXi",
             {}, IntentConfidence.HIGH)),
        (re.compile(r'(状态|自检|report|status)', re.I),
         lambda m: (IntentType.SYSTEM, "self_check",
             {}, IntentConfidence.HIGH)),
    ]

    # ── 一维时间线性: parse 强制按时间推进 ──

    def parse(self, text: str, source: str = "keyboard") -> Intent:
        """
        Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
        """
        text = text.strip()
        if not text:
            return Intent(IntentType.UNKNOWN, "", raw_input=text)

        # Step 1: Perception — 感知家族
        family = self._perceive_family(text)

        # Step 2: Distinguish — 谐音路由
        homophone = self._route_homophone(text)
        if homophone:
            name, tgt = homophone
            intent = Intent(
                intent_type=IntentType.HOMOPHONE,
                target=name,
                confidence=IntentConfidence.CERTAIN.value,
                raw_input=text, source=source,
            )
            intent.family = PerceptionFamily.HOMOPHONE_BRIDGE
            intent.polarity = tgt.polarity
            self._record(intent)
            return intent

        # Step 3: Pattern Lookup — 模式匹配
        for pattern, factory in self.PATTERNS:
            m = pattern.search(text)
            if m:
                itype, target, params, conf = factory(m)
                intent = Intent(
                    intent_type=itype, target=target, params=params,
                    confidence=conf.value, raw_input=text, source=source,
                )
                intent.family = family
                self._record(intent)
                return intent

        # Step 4: Traverse — 解释流 (未知输入降级)
        fallback = self._sacrifice_classify(text)
        if fallback:
            intent = Intent(
                IntentType.TYPE_TEXT, text,
                confidence=IntentConfidence.LOW.value,
                raw_input=text, source=source,
            )
            # 只在感知家族不够具体时才用解释流覆盖
            if family == PerceptionFamily.NEUTRAL:
                intent.family = fallback
            else:
                intent.family = family
            intent.interp_state = "mltt"  # 已从岔途转为从体
            self._record(intent)
            return intent

        # Step 5: Arrival — 默认打字
        intent = Intent(
            IntentType.TYPE_TEXT, text,
            confidence=IntentConfidence.MEDIUM.value,
            raw_input=text, source=source,
        )
        intent.family = family
        self._record(intent)
        return intent

    def _record(self, intent: Intent):
        self.history.append(intent)
        self.stats[intent.intent_type.value] = \
            self.stats.get(intent.intent_type.value, 0) + 1
        self.family_stats[intent.family.value] = \
            self.family_stats.get(intent.family.value, 0) + 1

    def stats_report(self) -> dict:
        return {
            "total_parsed": len(self.history),
            "stats": self.stats,
            "family_stats": self.family_stats,
        }


if __name__ == "__main__":
    p = IntentParser()
    tests = [
        "抓取 https://github.com/lolict/hunyuan-os",
        "push 囡囝共生 v0.2 协议升级",
        "生成一段五言绝句",
        "查询 clv / nha",
        "握手",
        "状态",
        "今天天气真好",
        "lctqf",          # 谐音路由: Wife
        "wgslz",          # 谐音路由: Husband
        "mlll",           # 谐音路由: Container
        "回",             # 容器家族
        "你好世界",        # 夫妻家族
    ]
    for t in tests:
        i = p.parse(t)
        ack = "✓" if i.ack() else "✗"
        fam = i.family.value[:4]
        print(f"  [{ack}] [{fam:4}] [{i.intent_type.value:15}] {t[:30]}")
    print()
    print(json.dumps(p.stats_report(), ensure_ascii=False, indent=2))
