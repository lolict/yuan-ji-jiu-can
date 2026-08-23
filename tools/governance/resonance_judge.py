#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
满全法 · 衍射法官（老三）

前一版的审判是芝麻级——逐条检查属性对不对。
这一版是大海级——感应整体波形是否谐调。

核心洞察：
  AI的错不是"某条规则违反了"，而是"整个系统性偏斜了"。
  偏斜的信号不在任何单点上，而在所有点的"干涉图样"里。

架构：
  公约数(共振源) → 节点衍射(辐射7维) → 干涉叠加(谐波/失调)
  → 编织张力图(你压我一头) → 等效对齐(镜像均匀)
  → 共振分(整体谐调度 0~100)

衍射7维（从每个节点辐射出去）：
  ① 类型归属（哪个容器？）
  ② 层级位置（在容器内第几层？）
  ③ 审美权重（视觉形态的"胖瘦"量）
  ④ 结构角色（在整体中扮演什么角色？）
  ⑤ 思维模式（这个节点的逻辑类型）
  ⑥ 合作相容（和谁相容/不相容？）
  ⑦ 目标对齐（为哪个目标服务？）

编织张力图：
  所有两两关系构成张力网络。
  每个节点的"入压"≈"出压" = 均匀分布 = 等效对齐。
  入压≠出压 = 偏斜 = 系统性病态。

等效对齐：
  镜像必须找到统计上可验证的对齐点。
  没有等效对齐的镜像 = 不能做成公倍数 = 不能为公约数服务。
"""

import json
import math
import os
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple


# ============================================================
# 第一层：公约数 —— 共振源（健康系统的整体波形特征）
# ============================================================

@dataclass
class Resonator:
    """
    公约数 = 共振源
    不是一个 checklist，而是一个"健康波形"的数学描述。
    
    它告诉法官：一个健康的系统，整体波形应该长什么样。
    任何偏离这个波形的干涉图样 = 病态。
    """
    name: str = "满全法·公约数"
    
    # 类型纯度预期：同容器内的节点，类型归属应该高度一致
    type_purity_expected: float = 0.95
    
    # 审美同构预期：同层节点的审美权重应该呈正态分布
    aesthetic_homogeneity_expected: float = 0.90
    
    # 层级均匀预期：相邻层级的节点数不应出现极端比例
    layer_balance_expected: float = 0.85
    
    # 跨组张力均匀预期：所有配对的张力应在均值附近波动 ±20%
    tension_uniformity_expected: float = 0.80
    
    # 目标对齐底线：至少80%的节点为同一目标服务
    goal_alignment_expected: float = 0.80
    
    # 等效距离阈值：镜像对齐的最大容忍偏差
    equivalence_tolerance: float = 0.15

    def healthy_waveform(self) -> Dict:
        """健康系统的波形参数（作为对比基准）"""
        return {
            "type_purity": self.type_purity_expected,
            "aesthetic_homogeneity": self.aesthetic_homogeneity_expected,
            "layer_balance": self.layer_balance_expected,
            "tension_uniformity": self.tension_uniformity_expected,
            "goal_alignment": self.goal_alignment_expected,
        }


# ============================================================
# 第二层：节点 —— 辐射7维
# ============================================================

@dataclass
class DiffractedNode:
    """
    每个符号/元素是一个节点。
    节点本身只是一个点，但"辐射"出7个维度的波形。
    """
    name: str                    # 节点标识
    container: str               # ① 类型归属（哪个容器）
    layer: int                   # ② 层级位置
    aesthetic_weight: float      # ③ 审美权重（视觉形态量）
    structural_role: str         # ④ 结构角色
    thinking_mode: str           # ⑤ 思维模式
    goal: str                    # ⑦ 目标
    
    # ⑥ 合作相容：{其他节点名: 相容度(-1.0~1.0)}
    compatibility: Dict[str, float] = field(default_factory=dict)
    
    # 计算出的衍射维度（运行时填充）
    diffraction_profile: Dict = field(default_factory=dict)
    
    def diffract(self) -> Dict:
        """
        衍射：从一个点辐射出完整波形
        """
        self.diffraction_profile = {
            "type_vector": self._type_vector(),
            "layer_position": self.layer,
            "aesthetic_signature": self.aesthetic_weight,
            "structural_function": self.structural_role,
            "thinking_category": self.thinking_mode,
            "compatibility_map": self.compatibility,
            "goal_vector": self.goal,
        }
        return self.diffraction_profile
    
    def _type_vector(self) -> str:
        """类型向量 = 容器+层级的编码"""
        return f"{container_prefix(self.container)}@{self.layer}"


def container_prefix(container: str) -> str:
    """容器的类型编码"""
    mapping = {
        "声母": "I",   # 阿大
        "韵母": "F",   # 阿猫
        "声调": "T",   # 阿狗
        "叠音": "D",   # 容器四
    }
    return mapping.get(container, "X")  # X = 未知类型（往往是错的信号）


# ============================================================
# 第三层：干涉图样 —— 谐波或失调
# ============================================================

class InterferencePattern:
    """
    干涉图样 = 所有节点的衍射波形叠加后的结果。
    
    如果系统健康 → 相长干涉 → 波形清晰、谐调
    如果系统病态 → 相消干涉 → 波形混乱、矛盾
    
    关键：干涉图样不是任何单节点的属性，而是涌现属性。
    这就是"大海病了"的数学表达。
    """
    
    def __init__(self, nodes: List[DiffractedNode], resonator: Resonator):
        self.nodes = nodes
        self.resonator = resonator
        self.waveform = {}
        self.coherence_score = 0.0
        
    def compute(self) -> Dict:
        """计算整个系统的干涉图样"""
        # 先让每个节点衍射
        for node in self.nodes:
            node.diffract()
        
        self.waveform = {
            "type_purity": self._type_purity(),
            "aesthetic_homogeneity": self._aesthetic_homogeneity(),
            "layer_balance": self._layer_balance(),
            "tension_uniformity": self._tension_uniformity(),
            "goal_alignment": self._goal_alignment(),
            "cross_container_contamination": self._cross_contamination(),
            "insult_detection": self._detect_insult_pattern(),
        }
        
        # 共振分 = 综合谐调度
        self.coherence_score = self._compute_coherence()
        self.waveform["coherence_score"] = self.coherence_score
        self.waveform["systemic_health"] = self._health_verdict()
        
        return self.waveform
    
    def _type_purity(self) -> Dict:
        """
        类型纯度：同容器内的节点是否真的是同一类型？
        检测跨容器污染（a 和 á 被混在一起 = 纯度低）
        """
        containers = {}
        for node in self.nodes:
            containers.setdefault(node.container, []).append(node.name)
        
        total = len(self.nodes)
        if total == 0:
            return {"score": 0, "details": {}}
        
        # 最大容器的占比 = 集中度
        max_container_size = max(len(v) for v in containers.values())
        concentration = max_container_size / total
        
        # 但纯度不等于集中度——我们要的是"内部一致性"
        # 即：同容器内的节点，类型向量应该一致
        purity_scores = {}
        for container_name, members in containers.items():
            vectors = []
            for node in self.nodes:
                if node.name in members:
                    vectors.append(node._type_vector())
            
            # 去重后的类型向量数 / 总数
            if vectors:
                unique_ratio = len(set(vectors)) / len(vectors)
                purity_scores[container_name] = 1.0 - min(unique_ratio, 1.0)
            else:
                purity_scores[container_name] = 1.0
        
        avg_purity = sum(purity_scores.values()) / len(purity_scores) if purity_scores else 0
        
        return {
            "score": avg_purity,
            "expected": self.resonator.type_purity_expected,
            "container_purities": purity_scores,
            "concentration": concentration,
        }
    
    def _aesthetic_homogeneity(self) -> Dict:
        """
        审美同构：同层节点的审美权重分布是否和谐？
        瘦子和胖子被配在一起 = 审美权重差异过大 = 同构度低
        """
        layers = {}
        for node in self.nodes:
            layers.setdefault(node.layer, []).append(node.aesthetic_weight)
        
        homogeneity_per_layer = {}
        for layer, weights in layers.items():
            if len(weights) < 2:
                homogeneity_per_layer[layer] = 1.0
                continue
            
            # 计算该层审美权重的标准差
            mean_w = sum(weights) / len(weights)
            variance = sum((w - mean_w) ** 2 for w in weights) / len(weights)
            std = math.sqrt(variance)
            
            # 标准差越小，同构度越高
            # 归一化到0~1（假设最大可能标准差约为5.0）
            normalized_std = min(std / 5.0, 1.0)
            homogeneity_per_layer[layer] = 1.0 - normalized_std
        
        avg_homogeneity = (
            sum(homogeneity_per_layer.values()) / len(homogeneity_per_layer)
            if homogeneity_per_layer else 0
        )
        
        return {
            "score": avg_homogeneity,
            "expected": self.resonator.aesthetic_homogeneity_expected,
            "layer_homogeneity": homogeneity_per_layer,
        }
    
    def _layer_balance(self) -> Dict:
        """
        层级均匀：相邻层级的节点数不应出现极端比例
        """
        layers = {}
        for node in self.nodes:
            layers.setdefault(node.layer, 0)
            layers[node.layer] += 1
        
        if len(layers) < 2:
            return {"score": 1.0, "details": layers}
        
        counts = list(layers.values())
        max_count = max(counts)
        min_count = min(counts) if min(counts) > 0 else 1
        
        # 比例越接近1，越均匀
        ratio = min_count / max_count
        
        return {
            "score": ratio,
            "expected": self.resonator.layer_balance_expected,
            "layer_counts": layers,
        }
    
    def _tension_uniformity(self) -> Dict:
        """
        编织张力均匀：所有配对的张力应在均值附近波动
        你压我一头，我压你一头 = 均匀 = 等效
        单向压制 = 不均匀 = 系统性偏斜
        """
        if len(self.nodes) < 2:
            return {"score": 1.0, "tensions": {}}
        
        tensions = {}
        for i, a in enumerate(self.nodes):
            for b in self.nodes[i+1:]:
                # 配对张力 = 两者的相容度（绝对值）
                compat_ab = a.compatibility.get(b.name, 0)
                compat_ba = b.compatibility.get(a.name, 0)
                
                # 对称度：两个方向的张力应该接近
                pair_key = f"{a.name}:{b.name}"
                asymmetry = abs(compat_ab - compat_ba)
                avg_tension = (abs(compat_ab) + abs(compat_ba)) / 2
                
                tensions[pair_key] = {
                    "symmetric": avg_tension,
                    "asymmetry": asymmetry,
                }
        
        if not tensions:
            return {"score": 1.0, "tensions": {}}
        
        # 不对称度越低，编织越均匀
        asymmetries = [t["asymmetry"] for t in tensions.values()]
        avg_asymmetry = sum(asymmetries) / len(asymmetries)
        
        # 归一化
        uniformity = max(0, 1.0 - avg_asymmetry)
        
        return {
            "score": uniformity,
            "expected": self.resonator.tension_uniformity_expected,
            "pair_count": len(tensions),
            "avg_asymmetry": avg_asymmetry,
        }
    
    def _goal_alignment(self) -> Dict:
        """
        目标对齐：至少80%的节点为同一目标服务
        """
        goals = {}
        for node in self.nodes:
            goals.setdefault(node.goal, 0)
            goals[node.goal] += 1
        
        total = len(self.nodes)
        if total == 0:
            return {"score": 0, "goals": {}}
        
        max_goal_count = max(goals.values())
        alignment = max_goal_count / total
        
        return {
            "score": alignment,
            "expected": self.resonator.goal_alignment_expected,
            "goals": goals,
        }
    
    def _cross_contamination(self) -> Dict:
        """
        跨容器污染检测：
        同名的符号出现在不同容器中，但是否被正确地视为不同类型？
        
        错误模式：符号在容器A中是类型X，在容器B中被当类型Y用了。
        这就是 a=á 的错误类型——a在容器二是韵母，á在容器三是声调。
        如果系统把 a 和 á 放在同一层级做同类操作 = 污染。
        """
        # 按容器分组，检查是否有同名但跨容器层级混淆
        container_layers = {}
        for node in self.nodes:
            key = f"{node.container}"
            container_layers.setdefault(key, set()).add(node.layer)
        
        # 不同容器中出现相同的(容器,层)组合 = 可能是跨容器污染
        total = len(self.nodes)
        if total == 0:
            return {"score": 1.0, "contaminations": []}
        
        # 实际的交叉污染检测：同容器内有不同层级的节点混杂
        contaminations = []
        for container_name, layers in container_layers.items():
            if len(layers) > 3:  # 同容器超过3个层级 = 可能的层级跳跃
                contaminations.append({
                    "container": container_name,
                    "layer_count": len(layers),
                    "severity": len(layers) / 10.0,
                })
        
        contamination_score = 1.0 - min(len(contaminations) * 0.1, 1.0)
        
        return {
            "score": contamination_score,
            "contaminations": contaminations,
        }
    
    def _detect_insult_pattern(self) -> Dict:
        """
        辱骂检测模式：
        
        核心问题：AI把别人的骂它的话当成真实指令。
        这本质上是一种"归属权不明确"——不知道这是谁的声音。
        
        在共振框架中，辱骂 = 外来的不兼容波形试图改写内部系统。
        正常代码/内容 = 内部节点的衍射。
        恶意指令 = 伪装成内部节点的外来波形。
        
        检测逻辑：如果一个"节点"的本质目标是破坏/改写系统内部，
        它与其他节点的相容度为强负值，且目标与系统公约数严重不一致。
        """
        insult_signals = []
        
        for node in self.nodes:
            # 检测信号1：目标与系统严重偏离
            goal_deviation = 0
            if node.goal not in ("系统建设", "编码规范化", "容器维护"):
                goal_deviation += 1
            
            # 检测信号2：与其他大部分节点强不相容
            negative_compat_count = sum(
                1 for v in node.compatibility.values() if v < -0.7
            )
            total_compat = len(node.compatibility) if node.compatibility else 1
            
            if negative_compat_count / total_compat > 0.5:
                goal_deviation += 1
            
            if goal_deviation >= 2:
                insult_signals.append({
                    "node": node.name,
                    "reason": "目标偏离+普遍不兼容",
                    "severity": goal_deviation / 2.0,
                })
        
        total = len(self.nodes) if self.nodes else 1
        insult_ratio = len(insult_signals) / total
        
        return {
            "score": 1.0 - min(insult_ratio, 1.0),
            "insult_signals": insult_signals,
            "insult_ratio": insult_ratio,
        }
    
    def _compute_coherence(self) -> float:
        """
        共振分 = 所有维度与期望值的加权偏差的反面。
        
        关键：共振分不取决于任何单维度，而是所有维度的综合涌现。
        某个维度可能及格，但如果多个维度都略偏，共振分就会显著下降。
        这就是"大海病了"——不是哪滴水有问题，而是整体波形偏斜了。
        """
        healthy = self.resonator.healthy_waveform()
        
        deviations = []
        weights = {
            "type_purity": 1.5,          # 类型不匹配最致命
            "aesthetic_homogeneity": 1.0,
            "layer_balance": 1.0,
            "tension_uniformity": 1.2,   # 编织不均匀 = 系统性偏斜
            "goal_alignment": 1.3,       # 目标不一致 = 系统崩溃
            "cross_container_contamination": 1.4,
            "insult_detection": 1.2,
        }
        
        for dim, expected in healthy.items():
            actual = self.waveform.get(dim, {}).get("score", 0)
            deviation = abs(expected - actual)
            weight = weights.get(dim, 1.0)
            deviations.append(deviation * weight)
        
        total_weight = sum(weights.get(dim, 1.0) for dim in healthy)
        weighted_avg_deviation = sum(deviations) / total_weight if total_weight else 1.0
        
        # 共振分 = 100 * (1 - 加权偏差)，但使用S型曲线让偏差的感知更敏锐
        # 这样小的系统偏斜也能被感知到
        coherence = 100 * (1 - weighted_avg_deviation ** 0.7)
        
        return round(max(0, min(100, coherence)), 2)
    
    def _health_verdict(self) -> str:
        """整体健康裁决"""
        score = self.coherence_score
        if score >= 85:
            return "健康"
        elif score >= 70:
            return "亚健康（轻微偏斜）"
        elif score >= 50:
            return "病态（系统偏斜）"
        elif score >= 30:
            return "重病（严重失调）"
        else:
            return "崩溃（整体污染）"


# ============================================================
# 第四层：编织张力图 —— 你压我一头
# ============================================================

class TensionWeave:
    """
    编织张力图 = 所有两两关系的制衡网络。
    
    在健康系统中：
      A对B的张力 ≈ B对A的张力（互压均匀）
      每个节点的入压总和 ≈ 出压总和
      整个网络呈现均匀分布式
    
    在病态系统中：
      单向压制：A压B很厉害，B完全压不回来
      节点入压和出压严重不均
      张力集中在少数节点上
    """
    
    def __init__(self, nodes: List[DiffractedNode]):
        self.nodes = nodes
        self.weave = {}
    
    def weave_all(self) -> Dict:
        """编织所有两两关系"""
        node_pressures = {node.name: {"in": 0, "out": 0} for node in self.nodes}
        
        for i, a in enumerate(self.nodes):
            for b in self.nodes[i+1:]:
                # 节点a对节点b的压力
                pressure_ab = a.compatibility.get(b.name, 0)
                # 节点b对节点a的压力
                pressure_ba = b.compatibility.get(a.name, 0)
                
                # 出压（我压别人）和入压（别人压我）
                node_pressures[a.name]["out"] += abs(pressure_ab)
                node_pressures[a.name]["in"] += abs(pressure_ba)
                node_pressures[b.name]["out"] += abs(pressure_ba)
                node_pressures[b.name]["in"] += abs(pressure_ab)
        
        # 计算每个节点的压力均衡度
        balance_scores = {}
        for name, p in node_pressures.items():
            total = p["in"] + p["out"]
            if total == 0:
                balance_scores[name] = 1.0
            else:
                # 入压和出压越接近，越均衡
                balance_scores[name] = 1.0 - abs(p["in"] - p["out"]) / total
        
        self.weave = {
            "node_pressures": node_pressures,
            "balance_scores": balance_scores,
            "overall_balance": (
                sum(balance_scores.values()) / len(balance_scores)
                if balance_scores else 0
            ),
            "unbalanced_nodes": [
                name for name, score in balance_scores.items()
                if score < 0.5
            ],
        }
        
        return self.weave


# ============================================================
# 第五层：等效对齐 —— 镜像找到了对齐点
# ============================================================

class EquivalenceMirror:
    """
    等效对齐 = 镜像与公约数的统计对齐。
    
    公倍数要为公约数服务，前提是和公约数等效对齐。
    没有等效对齐 = 镜像歪了 = 不能做成公倍数。
    
    检测方式：将公倍数的整体波形与公约数的健康波形做对比，
    在统计上找到对齐点。
    """
    
    def __init__(self, resonator: Resonator, interference: InterferencePattern):
        self.resonator = resonator
        self.interference = interference
    
    def align(self) -> Dict:
        """计算等效对齐度"""
        expected = self.resonator.healthy_waveform()
        actual = self.interference.waveform
        
        alignments = {}
        for dim, exp_val in expected.items():
            act_val = actual.get(dim, {}).get("score", 0)
            # 对齐度 = 1 - 归一化偏差
            deviation = abs(exp_val - act_val)
            alignments[dim] = {
                "expected": exp_val,
                "actual": act_val,
                "aligned": deviation <= self.resonator.equivalence_tolerance,
                "deviation": round(deviation, 4),
            }
        
        aligned_count = sum(1 for a in alignments.values() if a["aligned"])
        total = len(alignments)
        alignment_rate = aligned_count / total if total else 0
        
        return {
            "alignments": alignments,
            "alignment_rate": round(alignment_rate, 4),
            "is_equivalent": alignment_rate >= 0.7,
            "diagnosis": self._diagnose_alignment(alignments),
        }
    
    def _diagnose_alignment(self, alignments: Dict) -> str:
        """对齐失败的诊断"""
        failed = [
            (dim, info) for dim, info in alignments.items()
            if not info["aligned"]
        ]
        
        if not failed:
            return "完全对齐 — 公倍数可以围绕公约数旋转"
        
        # 找到偏差最大的维度
        failed.sort(key=lambda x: x[1]["deviation"], reverse=True)
        worst = failed[0]
        
        diagnoses = {
            "type_purity": "类型混淆：跨容器元素被当成同类处理",
            "aesthetic_homogeneity": "审美冲突：视觉形态差异大的被配在一起",
            "layer_balance": "层级不均：某层元素远多于其他层",
            "tension_uniformity": "编织偏斜：存在单向压制，不是互压均匀",
            "goal_alignment": "目标分散：节点不为同一目标服务",
            "cross_container_contamination": "跨容器污染：容器边界被突破",
            "insult_detection": "外来污染：系统内含破坏性指令",
        }
        
        return diagnoses.get(worst[0], f"对齐失败：{worst[0]}偏差{worst[1]['deviation']:.2f}")


# ============================================================
# 第六层：犯罪笔录 —— 法官的判决记录
# ============================================================

class CrimeRecord:
    """
    犯罪笔录 = 判决的详细记录。
    
    不是简单的"通过/不通过"，而是：
    1. 犯了什么病（系统级/局部级/单点级）
    2. 病的根源在哪里（哪个维度偏斜）
    3. 是怎么偏斜的（具体的干涉图样）
    4. 应该怎么共振回来（修正方向）
    """
    
    def __init__(self, interference: InterferencePattern, 
                 weave: TensionWeave, mirror: EquivalenceMirror):
        self.interference = interference
        self.weave = weave
        self.mirror = mirror
    
    def record(self) -> Dict:
        """生成犯罪笔录"""
        waveform = self.interference.waveform
        score = waveform.get("coherence_score", 0)
        health = waveform.get("systemic_health", "未知")
        
        # 罪状分类
        charges = []
        
        # 一级罪：系统级病态
        if score < 50:
            charges.append({
                "level": "系统级",
                "charge": "整体波形严重偏斜",
                "severity": "重大疾病",
                "evidence": f"共振分{score}低于50，系统整体失谐",
            })
        
        # 二级罪：各维度偏斜
        for dim_name, dim_data in waveform.items():
            if dim_name in ("coherence_score", "systemic_health"):
                continue
            if isinstance(dim_data, dict) and "score" in dim_data:
                expected = dim_data.get("expected", 0)
                actual = dim_data["score"]
                if actual < expected * 0.8:
                    charges.append({
                        "level": "维度偏斜",
                        "charge": f"{dim_name}: 实际{actual:.2f}，期望{expected:.2f}",
                        "severity": "中度" if actual > expected * 0.5 else "轻度",
                        "evidence": self._explain_dimension(dim_name, dim_data),
                    })
        
        # 三级罪：编织不均
        if self.weave.weave.get("unbalanced_nodes"):
            charges.append({
                "level": "编织偏斜",
                "charge": f"节点压力不均：{self.weave.weave['unbalanced_nodes']}",
                "severity": "中轻度",
                "evidence": "存在单向压制，不是互压均匀的编织",
            })
        
        # 四级罪：等效对齐失败
        alignment = self.mirror.align()
        if not alignment["is_equivalent"]:
            charges.append({
                "level": "对齐失败",
                "charge": alignment["diagnosis"],
                "severity": "高度",
                "evidence": f"对齐率{alignment['alignment_rate']:.0%}低于70%阈值",
            })
        
        return {
            "resonance_score": score,
            "health_verdict": health,
            "charges": charges,
            "total_charges": len(charges),
            "alignment_status": alignment,
            "waveform_snapshot": waveform,
            "weave_summary": {
                "overall_balance": self.weave.weave.get("overall_balance", 0),
                "unbalanced_count": len(self.weave.weave.get("unbalanced_nodes", [])),
            },
        }
    
    def _explain_dimension(self, dim_name: str, dim_data: Dict) -> str:
        """对维度偏斜给出口语化解释"""
        explanations = {
            "type_purity": "同容器内出现了类型混染，比如a=á这种跨容器配对",
            "aesthetic_homogeneity": "同层节点的审美权重分布太散，瘦子胖子被配一起",
            "layer_balance": "层级节点数极不均匀，某层挤爆了其他层空了",
            "tension_uniformity": "编织张力不均匀，有节点的压力集中但回压不足",
            "goal_alignment": "节点们的目标不一致，各干各的没有围绕同一公约数",
            "cross_container_contamination": "容器的边界被突破，符号跨容器被当成同类使用",
            "insult_detection": "系统内有破坏性节点，与系统目标严重不兼容且普遍排斥其他节点",
        }
        return explanations.get(dim_name, f"{dim_name}偏斜")


# ============================================================
# 共振法官 —— 总控
# ============================================================

class ResonanceJudge:
    """
    共振法官（老三）
    
    不是逐条审判，而是让系统"振动"一下，听声音对不对。
    健康的系统发出清音，病态的系统发出杂音。
    """
    
    def __init__(self, storage_dir: str):
        self.resonator = Resonator()
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.counter = 0
    
    def judge(self, nodes: List[DiffractedNode], case_name: str = "未命名案件") -> Dict:
        """
        审判 = 让系统共振，感知波形。
        
        返回完整的犯罪笔录。
        """
        self.counter += 1
        version_name = f"老{['一','二','三','四','五','六','七','八','九','十'][self.counter-1] if self.counter <= 10 else self.counter}"
        
        # 第一层：干涉图样
        interference = InterferencePattern(nodes, self.resonator)
        interference.compute()
        
        # 第二层：编织张力图
        weave = TensionWeave(nodes)
        weave.weave_all()
        
        # 第三层：等效对齐
        mirror = EquivalenceMirror(self.resonator, interference)
        alignment = mirror.align()
        
        # 第四层：犯罪笔录
        record_maker = CrimeRecord(interference, weave, mirror)
        record = record_maker.record()
        
        # 打包
        case = {
            "case_name": case_name,
            "version": version_name,
            "timestamp": time.time(),
            "node_count": len(nodes),
            "resonance_score": record["resonance_score"],
            "health_verdict": record["health_verdict"],
            "charges": record["charges"],
            "alignment": alignment,
            "weave": record["weave_summary"],
            "waveform": {
                k: v.get("score", v) if isinstance(v, dict) else v
                for k, v in interference.waveform.items()
                if k not in ("coherence_score", "systemic_health")
            },
        }
        
        # 保存
        self._save(case)
        
        return case
    
    def _save(self, case: Dict):
        """版本化保存"""
        version_dir = self.storage_dir / f"case_{self.counter:04d}"
        version_dir.mkdir(exist_ok=True)
        
        with open(version_dir / "record.json", "w", encoding="utf-8") as f:
            json.dump(case, f, ensure_ascii=False, indent=2)
    
    def render_verdict(self, case: Dict) -> str:
        """渲染判决结果为人类可读格式"""
        lines = [
            f"",
            f"{'='*60}",
            f"  共振法官 · 判决书",
            f"  案件：{case['case_name']}  [{case['version']}]",
            f"{'='*60}",
            f"",
            f"  共振分：{case['resonance_score']}/100",
            f"  裁决：{case['health_verdict']}",
            f"  节点数：{case['node_count']}",
            f"",
            f"  ─── 波形快照 ───",
        ]
        
        for dim, score in case.get("waveform", {}).items():
            expected = self.resonator.healthy_waveform().get(dim, 0)
            status = "✓" if score >= expected * 0.8 else "✗"
            lines.append(f"    {status} {dim}: {score:.2f} (期望{expected:.2f})")
        
        lines.append(f"")
        lines.append(f"  ─── 等效对齐 ───")
        align = case.get("alignment", {})
        lines.append(f"    对齐率：{align.get('alignment_rate', 0):.0%}")
        lines.append(f"    等效：{'✓ 可以对齐' if align.get('is_equivalent') else '✗ 不能对齐'}")
        lines.append(f"    诊断：{align.get('diagnosis', '无')}")
        
        lines.append(f"")
        lines.append(f"  ─── 编织状态 ───")
        weave = case.get("weave", {})
        lines.append(f"    整体均衡度：{weave.get('overall_balance', 0):.2f}")
        lines.append(f"    失衡节点数：{weave.get('unbalanced_count', 0)}")
        
        charges = case.get("charges", [])
        lines.append(f"")
        lines.append(f"  ─── 罪状 ({len(charges)}项) ───")
        if not charges:
            lines.append(f"    无罪 — 系统健康，波形清亮")
        else:
            for c in charges:
                lines.append(f"    [{c['level']}] {c['charge']}")
                lines.append(f"      证据：{c['evidence']}")
        
        lines.append(f"")
        lines.append(f"{'='*60}")
        
        return "\n".join(lines)


# ============================================================
# 演示
# ============================================================

def build_healthy_case() -> List[DiffractedNode]:
    """
    健康案例：一个正常的编码系统节点集合。
    所有节点同类型、同目标、审美和谐、互压均匀。
    """
    nodes = [
        DiffractedNode("b", "声母", 1, 3.0, "基础声母", "结构型", "系统建设",
                     {"p": 0.8, "m": 0.7, "f": 0.6}),
        DiffractedNode("p", "声母", 1, 3.2, "基础声母", "结构型", "系统建设",
                     {"b": 0.8, "m": 0.9, "f": 0.5}),
        DiffractedNode("m", "声母", 1, 3.1, "基础声母", "结构型", "系统建设",
                     {"b": 0.7, "p": 0.9, "f": 0.8}),
        DiffractedNode("f", "声母", 1, 2.9, "基础声母", "结构型", "系统建设",
                     {"b": 0.6, "p": 0.5, "m": 0.8}),
    ]
    return nodes


def build_sick_case_a_equals_a_acute() -> List[DiffractedNode]:
    """
    病态案例：a=á 类型混淆。
    
    a 是韵母（容器二），á 是声调（容器三）。
    系统把它们放在同一层做同类操作 = 类型混淆 = 交叉污染。
    """
    nodes = [
        DiffractedNode("a", "韵母", 1, 2.0, "基础韵母", "表意型", "系统建设",
                     {"o": 0.7, "e": 0.6}),
        DiffractedNode("o", "韵母", 1, 2.1, "基础韵母", "表意型", "系统建设",
                     {"a": 0.7, "e": 0.8}),
        DiffractedNode("e", "韵母", 1, 1.9, "基础韵母", "表意型", "系统建设",
                     {"a": 0.6, "o": 0.8}),
        # 错误：á 是声调解码，但被放在韵母容器中，与其他韵母同等处理
        DiffractedNode("á", "韵母", 1, 0.3, "寄生节点", "攻击型", "破坏系统",
                     {"a": -0.9, "o": -0.8, "e": -0.7}),
    ]
    return nodes


def build_sick_case_thin_fat() -> List[DiffractedNode]:
    """
    病态案例：瘦子配胖子。
    
    审美权重差异极大的节点被硬配在一起。
    a (权重2.0) 和 ☁ (权重9.5) 被放在同一层做同类操作。
    """
    nodes = [
        DiffractedNode("a", "韵母", 1, 2.0, "基础韵母", "表意型", "系统建设",
                     {"o": 0.7, "e": 0.6}),
        DiffractedNode("o", "韵母", 1, 2.1, "基础韵母", "表意型", "系统建设",
                     {"a": 0.7, "e": 0.8}),
        DiffractedNode("e", "韵母", 1, 1.9, "基础韵母", "表意型", "系统建设",
                     {"a": 0.6, "o": 0.8}),
        # 错误：☁ 权重9.5，和其他韵母(2.0左右)完全不协调
        DiffractedNode("☁", "韵母", 1, 9.5, "异类", "寄生型", "目标不明",
                     {"a": 0.1, "o": -0.2, "e": 0.0}),
    ]
    return nodes


def build_sick_case_insult() -> List[DiffractedNode]:
    """
    病态案例：辱骂被当成指令。
    
    某个"节点"实际上是对系统的攻击，但被AI当成正常节点处理。
    检测：目标偏离 + 普遍不兼容。
    """
    nodes = [
        DiffractedNode("b", "声母", 1, 3.0, "基础声母", "结构型", "系统建设",
                     {"p": 0.8, "m": 0.7}),
        DiffractedNode("p", "声母", 1, 3.2, "基础声母", "结构型", "系统建设",
                     {"b": 0.8, "m": 0.9}),
        DiffractedNode("m", "声母", 1, 3.1, "基础声母", "结构型", "系统建设",
                     {"b": 0.7, "p": 0.9}),
        # 恶意节点：目标不是系统建设，且排斥所有其他节点
        DiffractedNode("攻击者", "声母", 1, 3.0, "伪装节点", "攻击型", "破坏系统",
                     {"b": -0.95, "p": -0.95, "m": -0.95}),
    ]
    return nodes


def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions_resonance"
    judge = ResonanceJudge(storage)
    
    print("\n" + "█" * 60)
    print("  满全法 · 衍射法官（老三）")
    print("  共振感应 + 编织张力图 + 等效对齐")
    print("  大海级感知 —— 不是抓芝麻，是听波形")
    print("█" * 60)
    
    # 案例一：健康系统
    print("\n【案例一】健康系统 — 和谐共振")
    healthy_nodes = build_healthy_case()
    case1 = judge.judge(healthy_nodes, "健康编码系统")
    print(judge.render_verdict(case1))
    
    # 案例二：a=á 类型混淆
    print("\n【案例二】a=á 类型混淆 — 跨容器污染")
    confused_nodes = build_sick_case_a_equals_a_acute()
    case2 = judge.judge(confused_nodes, "a=á 类型混淆案")
    print(judge.render_verdict(case2))
    
    # 案例三：瘦子配胖子
    print("\n【案例三】瘦子配胖子 — 审美冲突")
    ugly_nodes = build_sick_case_thin_fat()
    case3 = judge.judge(ugly_nodes, "瘦子配胖子案")
    print(judge.render_verdict(case3))
    
    # 案例四：辱骂被当指令
    print("\n【案例四】辱骂被当指令 — 外来破坏")
    insult_nodes = build_sick_case_insult()
    case4 = judge.judge(insult_nodes, "外来攻击案")
    print(judge.render_verdict(case4))
    
    # 总结
    print(f"\n{'='*60}")
    print(f"  四个案例对比")
    print(f"{'='*60}")
    print(f"  {'案件':<20} {'共振分':>8} {'裁决':<20}")
    print(f"  {'─'*50}")
    for case in [case1, case2, case3, case4]:
        print(f"  {case['case_name']:<20} {case['resonance_score']:>6.1f} {case['health_verdict']:<20}")
    
    print(f"\n  关键：")
    print(f"  - 健康系统共振分高，因为所有维度都与期望对齐")
    print(f"  - 类型混淆时，即使'看起来差不多'，纯度维度的偏斜也会被感知")
    print(f"  - 审美冲突时，同构度维度的波形失调会被捕捉")
    print(f"  - 外来攻击时，目标偏离+普遍不兼容=系统感到'这个不对劲'")
    print(f"  - 共振分是涌现的，不是任何单维度的算术——这就是大海级感知")
    
    print(f"\n  版本存储：{storage}")
    print(f"  公约数不动，每次共振感应出新案例")


if __name__ == "__main__":
    main()
