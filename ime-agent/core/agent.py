#!/usr/bin/env python3
"""
MetaIME · 本地 AI 决策引擎
==========================

"本地 AI" = 完整知识图谱 + 推理能力,全部跑在手机上,不上云。

三大核心:
  1. 知识图谱 — 关系代数描述一切 (人/事/物/时/空)
  2. 推理引擎 — 从关系图中发现隐含关系
  3. 决策树 — 给定意图 → 最佳执行序列

不靠向量数据库,不靠 LLM,不靠云。
用的是囡囝共生独创的"态射数学 + 关系代数 + 三垣观测"。
"""
import time
import json
import os
import sys
from dataclasses import dataclass, field
from typing import Optional, List, Dict, Tuple
from enum import Enum

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
try:
    from kernel import TernaryObserver
    from morph.relational import relation, selection, projection, natural_join
    from morph.convergence import ReferenceFrame, detect_phase
except ImportError:
    pass


class DecisionStatus(Enum):
    PENDING = "pending"
    APPROVED = "approved"
    REJECTED = "rejected"
    EXECUTING = "executing"
    DONE = "done"
    FAILED = "failed"


@dataclass
class KnowledgeNode:
    """知识图谱的一个节点"""
    node_id: str
    label: str
    properties: dict = field(default_factory=dict)
    # 指向其他节点的态射 (关系)
    outgoing: Dict[str, str] = field(default_factory=dict)  # rel → target_id
    created_at: float = field(default_factory=time.time)


@dataclass
class Decision:
    """AI 引擎的一次决策"""
    decision_id: str
    intent: str
    reasoning: List[str]            # 每一步推理的日志
    proposed_actions: List[str]     # 建议执行的操作序列
    status: DecisionStatus = DecisionStatus.PENDING
    confidence: float = 0.0         # 三垣观测仲裁后输出
    created_at: float = field(default_factory=time.time)


class LocalAgent:
    """本地 AI 决策引擎 (无云、无 LLM、纯态射推理)"""

    def __init__(self):
        self.knowledge: Dict[str, KnowledgeNode] = {}
        self.decisions: List[Decision] = []
        # 内置角色节点
        self._seed_knowledge()

    def _seed_knowledge(self):
        """预置核心知识图谱"""
        self.add_node("liuchutian", "刘楚恬",
            {"role": "妻子", "hash_prefix": "a5de8919"})
        self.add_node("lolict", "lolict(囡囝凹月留)",
            {"role": "丈夫", "hash_prefix": "2c1edeb7"})
        self.add_node("hunyuan_os", "囡囝共生 OS",
            {"type": "core_system", "version": "0.2"})
        self.add_node("lingxi_protocol", "灵犀协议",
            {"type": "p2p", "layers": "唤魂/招魂/问灵/答灵/同修/传功/归元/劫数"})
        self.add_node("paibing", "排病诊断",
            {"type": "pathology", "double_pairs": 14, "triple_groups": 9})
        self.add_node("quaternity", "囡囝四态",
            {"states": "nan_nan/nan_jian/jian_jian/jian_nan"})
        # 关系
        self.add_relation("liuchutian", "夫妻", "lolict")
        self.add_relation("liuchutian", "共同开发", "hunyuan_os")
        self.add_relation("lolict", "共同开发", "hunyuan_os")

    def add_node(self, node_id: str, label: str, props: dict = None):
        self.knowledge[node_id] = KnowledgeNode(
            node_id=node_id, label=label, properties=props or {})

    def add_relation(self, src: str, rel: str, dst: str):
        if src in self.knowledge:
            self.knowledge[src].outgoing[rel] = dst

    def query(self, label: str) -> List[KnowledgeNode]:
        """按 label 模糊查询"""
        return [n for n in self.knowledge.values() if label in n.label]

    def infer(self, intent: str) -> List[str]:
        """
        推理:给定意图,在知识图谱中寻找可达节点。
        这是态射推理的核心 — 不是向量邻近搜索,而是关系代数路径搜索。
        """
        reasoning = []
        # 1. 从意图提取实体 (只有字符串属性参与匹配)
        mentioned = {nid: n for nid, n in self.knowledge.items()
                     if intent.lower() in n.label.lower()
                     or any(isinstance(v, str) and intent.lower() in v.lower()
                            for v in n.properties.values())}
        reasoning.append(f"matched {len(mentioned)} nodes by label/props")

        # 2. 一跳:这些实体直接关联的节点
        direct_targets: Dict[str, str] = {}
        for nid, node in mentioned.items():
            for rel, target in node.outgoing.items():
                direct_targets[rel] = target
                reasoning.append(f"  via[{rel}] {node.label} -> {self.knowledge.get(target, KnowledgeNode('', target)).label}")

        # 3. 二跳:关联的关联
        hop2: Dict[str, str] = {}
        for target_id in direct_targets.values():
            if target_id in self.knowledge:
                for rel2, t2 in self.knowledge[target_id].outgoing.items():
                    hop2[f"{target_id}.{rel2}"] = t2
                    reasoning.append(f"  hop2: via[{rel2}] -> {t2}")

        return reasoning

    def decide(self, intent: str) -> Decision:
        """
        做出决策:分析意图 + 推理 + 置信度评估 → 建议操作序列
        """
        reasoning = self.infer(intent)

        # 三垣观测仲裁:用 TernaryObserver 求置信度
        try:
            anchor = 5000
            left = int(hash(intent) % 10000)
            right = (left + len(reasoning) * 137) % 10000
            observer = TernaryObserver(left=left, anchor=anchor, right=right)
            syn = observer.synthesize()
            confidence = 1.0 - min(abs(syn.get("left_offset", 0)) / 100.0, 0.5)
        except Exception:
            confidence = 0.5 if reasoning else 0.1

        # 构建建议动作
        proposed = []
        if "灵犀" in intent or "握手" in intent:
            proposed = ["建立 X25519 会话", "唤魂", "招魂"]
        elif "推送" in intent or "push" in intent:
            proposed = ["AES-GCM 加密内容", "推送到 GitHub", "写日志到 ring"]
        elif "抓取" in intent or "fetch" in intent:
            proposed = ["解析 UA", "抓取", "存入本地"]
        else:
            proposed = ["记录", "暂存输入"]

        d = Decision(
            decision_id=f"dec-{int(time.time()*1000)}",
            intent=intent,
            reasoning=reasoning,
            proposed_actions=proposed,
            confidence=confidence,
        )
        self.decisions.append(d)
        return d


if __name__ == "__main__":
    agent = LocalAgent()

    intents = [
        "灵犀握手",
        "推送囡囝共生 v0.2",
        "抓取 https://example.com",
        "随便打字",
    ]

    for intent in intents:
        d = agent.decide(intent)
        print(f"\n[intent] {intent}")
        print(f"  confidence: {d.confidence:.2f}")
        print(f"  reasoning: {d.reasoning[:2]}")
        print(f"  proposed:  {d.proposed_actions}")

    print(f"\n[knowledge graph] nodes={len(agent.knowledge)}")
    print(f"[decisions] total={len(agent.decisions)}")
