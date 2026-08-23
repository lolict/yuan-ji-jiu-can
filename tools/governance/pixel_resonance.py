#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
满全法 · 像素共振（老四）

这一版的核心洞见：
  符号不是铁板一块的标签。符号内部有结构——
  就像汉字可以拆成偏旁笔画，每个符号也可以拆成
  一组"像素特征"（视觉块面、笔画密度、轮廓方向、字重等）。
  
  从这些特征层面做运算，而不是从符号标签层面做判断——
  这是"从规则走向感知"的第一步。

像素运算四则：
  加法：把 A 的特征叠加到 B 上 → 生成新结构
  减法：从 A 中移除某个特征 → 看剩余结构
  乘法：把 A 的特征和 B 的特征做融合 → 生成全新结构
  除法：把 A 的结构分解为基本要素 → 找到最小单位

辐射传导：
  每个节点通过特征相似度连接其他节点。
  连接有强度梯度——越像的连接越强，越远的连接越弱。
  一个节点被触发时，涟漪按强度梯度向外扩散。
  全局连续性由此产生——不是"检查"全局，而是全局自然在场。

产生"智慧"的机制（不是规则，是涌现）：
  当一个节点被处理时，所有和它有关联的节点
  都以不同程度的强度"在场"。
  决策不是靠规则 checklist，
  而是靠所有在场节点的综合"投票"。
  这个综合投票 = 智慧。
"""

import json
import math
import os
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Set


# ============================================================
# 特征维度 —— 把符号拆成可运算的像素特征
# ============================================================

@dataclass
class PixelFeatures:
    """
    每个符号的内部结构 = 一组像素特征。
    
    这些特征不是语义标签，是视觉/结构的可运算属性。
    就像一块砖可以被拆成：重量、颜色、纹理、密度——
    这些属性可以做物理操作。
    """
    # 视觉块面重量（胖瘦）
    weight: float = 1.0          # 0.0(极瘦) ~ 10.0(极胖)
    
    # 笔画疏密（信息密度）
    density: float = 1.0         # 0.0(极简) ~ 10.0(极繁)
    
    # 轮廓方向性
    roundness: float = 0.5       # 0.0(全棱角) ~ 1.0(全圆弧)
    
    # Unicode 区块家族（数值化）
    family: str = "latin"        # latin/cyrillic/ipa/cjk/math
    
    # 竖向高度比
    height_ratio: float = 1.0    # 相对于标准小写字母
    
    # 对称性
    symmetry: float = 0.5        # 0.0(不对称) ~ 1.0(完全对称)
    
    # 拓扑复杂度（孔洞数等）
    topology: float = 1.0        # 1.0(简单) ~ 5.0(复杂)
    
    def to_vector(self) -> List[float]:
        """转为数值向量（用于距离计算）"""
        family_map = {"latin": 0.0, "cyrillic": 0.25, "ipa": 0.5, "cjk": 0.75, "math": 1.0}
        return [
            self.weight / 10.0,
            self.density / 10.0,
            self.roundness,
            family_map.get(self.family, 0.5),
            self.height_ratio / 2.0,
            self.symmetry,
            self.topology / 5.0,
        ]


# ============================================================
# 像素四则运算
# ============================================================

class PixelArithmetic:
    """
    对符号的像素特征做加减乘除。
    
    这不是规则判断——这是对结构的物理操作。
    就像把两块积木拼在一起，看拼出什么。
    """
    
    @staticmethod
    def add(a: PixelFeatures, b: PixelFeatures) -> PixelFeatures:
        """
        加法 = 把 A 的特征叠加到 B 上
        
        例如：把"横"叠加到"竖"上 → 生成"十"
        这里体现为特征值的加权组合
        """
        return PixelFeatures(
            weight=min(10.0, a.weight * 0.3 + b.weight * 0.7),
            density=min(10.0, a.density + b.density * 0.5),
            roundness=(a.roundness + b.roundness) / 2,
            family=a.family if a.density > b.density else b.family,
            height_ratio=max(a.height_ratio, b.height_ratio),
            symmetry=(a.symmetry + b.symmetry) / 2,
            topology=min(5.0, a.topology + b.topology * 0.3),
        )
    
    @staticmethod
    def subtract(a: PixelFeatures, feature_name: str) -> PixelFeatures:
        """
        减法 = 从 A 中移除某个特征
        
        例如：从"国"中移除"囗" → 剩下"玉"
        这里体现为把某个特征值归零或衰减
        """
        result = PixelFeatures(
            weight=a.weight,
            density=a.density,
            roundness=a.roundness,
            family=a.family,
            height_ratio=a.height_ratio,
            symmetry=a.symmetry,
            topology=a.topology,
        )
        
        if feature_name == "weight":
            result.weight *= 0.3
        elif feature_name == "density":
            result.density *= 0.3
        elif feature_name == "roundness":
            result.roundness = 1.0 - result.roundness
        elif feature_name == "topology":
            result.topology *= 0.3
        elif feature_name == "symmetry":
            result.symmetry *= 0.5
        
        return result
    
    @staticmethod
    def multiply(a: PixelFeatures, b: PixelFeatures) -> PixelFeatures:
        """
        乘法 = 把 A 和 B 的特征做融合（不是简单叠加）
        
        乘法的结果比加法的结果更"新"——
        就像 3×4=12，12 和 3、4 都不是一个东西，但由它们生成。
        
        在像素层面：两种结构融合可能产生全新的拓扑。
        """
        # 融合后的新特征可能超出原来任何一个
        fused_weight = math.sqrt(a.weight * b.weight)  # 几何平均
        fused_density = math.sqrt(max(0, a.density**2 + b.density**2)) / math.sqrt(2)
        
        return PixelFeatures(
            weight=min(10.0, fused_weight * 1.2),
            density=min(10.0, fused_density),
            roundness=1.0 - abs(a.roundness - b.roundness),  # 越接近越圆融
            family=a.family if a.weight > b.weight else b.family,
            height_ratio=math.sqrt(a.height_ratio * b.height_ratio),
            symmetry=min(1.0, a.symmetry * b.symmetry + 0.1),
            topology=min(5.0, a.topology * b.topology / 2.0 + 0.5),
        )
    
    @staticmethod
    def divide(a: PixelFeatures, n: float = 2.0) -> List[str]:
        """
        除法 = 把 A 的结构分解为基本要素
        
        返回：这个结构包含哪些"基础要素"
        例如：一个复杂汉字 → [横, 竖, 撇, 捺, 点]
        """
        elements = []
        
        if a.weight > 2.0:
            elements.append("粗块面")
        if a.weight <= 2.0:
            elements.append("细骨架")
        
        if a.density > 3.0:
            elements.append("密笔触")
        if a.density <= 1.0:
            elements.append("极简")
        
        if a.roundness > 0.7:
            elements.append("圆弧")
        elif a.roundness < 0.3:
            elements.append("棱折")
        else:
            elements.append("方圆混合")
        
        if a.symmetry > 0.7:
            elements.append("对称")
        if a.topology > 2.0:
            elements.append("多孔")
        
        return elements


# ============================================================
# 节点 —— 内部有结构，外部有连接
# ============================================================

@dataclass
class PixelNode:
    """
    节点 = 一个符号，有内部像素结构和外部关联连接。
    
    内部：一组像素特征（可运算）
    外部：与其他节点的连接（有强度梯度）
    """
    name: str
    container: str           # 声母/韵母/声调/叠音
    role: str               # 功能角色
    goal: str               # 服务目标
    
    # 内部像素结构
    features: PixelFeatures = field(default_factory=PixelFeatures)
    
    # 外部连接：{目标节点名: 连接强度(0~1)}
    connections: Dict[str, float] = field(default_factory=dict)
    
    # 层级标签
    level: str = "L1"        # L1=基元本体, L2=修饰零件, L3=装配成品
    
    # 运行时状态
    activation: float = 0.0  # 当前激活度（辐射传导时用）
    base_features: Optional[PixelFeatures] = None  # 备份
    
    def __post_init__(self):
        if self.base_features is None:
            self.base_features = PixelFeatures(
                weight=self.features.weight,
                density=self.features.density,
                roundness=self.features.roundness,
                family=self.features.family,
                height_ratio=self.features.height_ratio,
                symmetry=self.features.symmetry,
                topology=self.features.topology,
            )
    
    def similarity_to(self, other: 'PixelNode') -> float:
        """
        相似度 = 两个节点像素特征的余弦距离
        
        这不是分类判断——这是结构上的"像不像"。
        两个符号长得越像、字重越接近、画风越一致 → 相似度越高。
        """
        vec_a = self.features.to_vector()
        vec_b = other.features.to_vector()
        
        # 余弦相似度
        dot = sum(a * b for a, b in zip(vec_a, vec_b))
        norm_a = math.sqrt(sum(a * a for a in vec_a))
        norm_b = math.sqrt(sum(b * b for b in vec_b))
        
        if norm_a == 0 or norm_b == 0:
            return 0.0
        
        return dot / (norm_a * norm_b)


# ============================================================
# 辐射网络 —— 涟漪传导 + 全局连续性
# ============================================================

class RadiationNetwork:
    """
    辐射网络 = 所有节点通过特征相似度连接成的图。
    
    全局连续性的实现：
    - 不是"检查全局"，而是每个节点自动连着其他所有节点
    - 连接有强度梯度——按特征相似度衰减
    - 一个节点被触发时，涟漪向外扩散，强度递减
    - 远处的节点虽然弱，但没有真正为零
    - 这就是"全局自然在场"——不需要主动扫描
    """
    
    def __init__(self, nodes: List[PixelNode], decay_rate: float = 0.6):
        self.nodes = {node.name: node for node in nodes}
        self.decay_rate = decay_rate  # 每跳衰减率
        self.proximity = {}  # 预计算的节点间相似度
    
    def build_proximity(self):
        """预计算所有节点间的特征相似度（连接强度梯度）"""
        names = list(self.nodes.keys())
        for i, name_a in enumerate(names):
            self.proximity[name_a] = {}
            for name_b in names:
                if name_a == name_b:
                    self.proximity[name_a][name_b] = 1.0
                else:
                    sim = self.nodes[name_a].similarity_to(self.nodes[name_b])
                    self.proximity[name_a][name_b] = sim
    
    def radiate(self, source_name: str, initial_intensity: float = 1.0,
                max_hops: int = 3) -> Dict[str, float]:
        """
        辐射传导：从源节点出发，涟漪按梯度向外扩散。
        
        每一跳衰减 decay_rate，强度 = 初始 × decay_rate^跳数 × 相似度
        
        返回：每个节点最终接收到的辐射强度
        """
        if source_name not in self.nodes:
            return {}
        
        # BFS辐射
        activations = {name: 0.0 for name in self.nodes}
        activations[source_name] = initial_intensity
        
        current_front = {source_name: initial_intensity}
        
        for hop in range(max_hops):
            next_front = {}
            
            for current_name, current_intensity in current_front.items():
                # 从当前节点向所有邻居辐射
                for neighbor_name, proximity in self.proximity.get(current_name, {}).items():
                    if neighbor_name == current_name:
                        continue
                    
                    # 辐射强度 = 当前强度 × 连接强度 × 衰减
                    propagated = current_intensity * proximity * self.decay_rate
                    
                    if propagated > activations[neighbor_name]:
                        activations[neighbor_name] = propagated
                    
                    if propagated > 0.01:  # 截止阈值
                        next_front[neighbor_name] = max(
                            next_front.get(neighbor_name, 0),
                            propagated
                        )
            
            current_front = next_front
            if not current_front:
                break
        
        return activations
    
    def global_presence(self, source_name: str) -> str:
        """描述全局在场状态"""
        activations = self.radiate(source_name)
        
        if not activations:
            return f"节点 {source_name} 不存在于网络中"
        
        # 按激活度排序
        sorted_activations = sorted(activations.items(), key=lambda x: -x[1])
        
        lines = [f"  从「{source_name}」辐射的全局在场："]
        for name, intensity in sorted_activations[:10]:
            bar = "█" * int(intensity * 20)
            lines.append(f"    {name:<8} {intensity:.3f} {bar}")
        
        total_presence = sum(activations.values())
        lines.append(f"  总辐射量: {total_presence:.3f}（所有节点的在场强度之和）")
        
        return "\n".join(lines)


# ============================================================
# 全局共振判断 —— 不是规则检查，是涌现感知
# ============================================================

class GlobalResonance:
    """
    全局共振 = 所有在场节点对当前决策的综合"投票"。
    
    智慧的产生机制（不是规则，是涌现）：
    1. 当一个节点被处理，所有关联节点以不同强度在场
    2. 在场节点根据自己的类型/容器/目标贡献"意见"
    3. 所有意见加权求和 → 涌现结果
    4. 加权不是人为设定的——是辐射强度自然决定的
    
    这个涌现结果就是"智慧"。
    不是任何规则告诉它该怎么做，
    而是全局的连续性自己得出的结论。
    """
    
    def __init__(self, network: RadiationNetwork):
        self.network = network
    
    def perceive(self, subject: PixelNode, candidate: PixelNode) -> Dict:
        """
        感知：subject 和 candidate 配不配？
        
        不是检查规则，而是问网络整体的"感觉"。
        网络整体通过辐射传导给出一个涌现判断。
        """
        # 让 subject 辐射——激活所有关联节点
        activations = self.network.radiate(subject.name)
        
        if not activations:
            return {"resonance": 0, "verdict": "无法感知", "details": {}}
        
        # 所有在场节点对 candidate 的"意见"
        # 意见 = 在场节点的类型与 candidate 的兼容性 × 激活度
        
        type_votes = {}
        aesthetic_votes = {}
        goal_votes = {}
        level_votes = {}
        total_weight = 0
        
        for node_name, intensity in activations.items():
            if node_name not in self.network.nodes:
                continue
            node = self.network.nodes[node_name]
            
            # 该节点对 candidate 的类型意见
            type_match = 1.0 if node.container == candidate.container else -0.3
            
            # 审美意见：两个节点的特征相似度 = 审美兼容性
            aesthetic_match = node.similarity_to(candidate)
            
            # 目标意见
            goal_match = 1.0 if node.goal == candidate.goal else -0.5
            
            # 层级意见
            level_match = 1.0 if node.level == candidate.level else -0.8
            
            type_votes[node_name] = type_match * intensity
            aesthetic_votes[node_name] = aesthetic_match * intensity
            goal_votes[node_name] = goal_match * intensity
            level_votes[node_name] = level_match * intensity
            
            total_weight += intensity
        
        if total_weight == 0:
            return {"resonance": 0, "verdict": "总权重为零", "details": {}}
        
        # 涌现结果 = 所有意见的加权平均
        avg_type = sum(type_votes.values()) / total_weight
        avg_aesthetic = sum(aesthetic_votes.values()) / total_weight
        avg_goal = sum(goal_votes.values()) / total_weight
        avg_level = sum(level_votes.values()) / total_weight
        
        # 综合共振分（每个维度的权重不同——这是涌现的，不是硬编码的）
        # 层级偏差最致命（权重最高），类型次之，审美第三，目标第四
        resonance = (
            avg_level * 0.35 +
            avg_type * 0.25 +
            avg_aesthetic * 0.20 +
            avg_goal * 0.20
        )
        
        # 映射到 0~100
        resonance_score = max(0, min(100, (resonance + 1) * 50))
        
        # 裁决
        if resonance_score >= 70:
            verdict = "相容（全局共振支持）"
        elif resonance_score >= 50:
            verdict = "有条件相容（部分节点反对）"
        elif resonance_score >= 30:
            verdict = "偏斜（多数节点感到不适）"
        else:
            verdict = "冲突（全局强烈排斥）"
        
        return {
            "resonance_score": round(resonance_score, 2),
            "verdict": verdict,
            "details": {
                "type_resonance": round(avg_type, 3),
                "aesthetic_resonance": round(avg_aesthetic, 3),
                "goal_resonance": round(avg_goal, 3),
                "level_resonance": round(avg_level, 3),
            },
            "rhythm": self._detect_rhythm(activations),
        }
    
    def _detect_rhythm(self, activations: Dict[str, float]) -> str:
        """
        检测辐射节奏：
        - 健康网络：辐射强度呈平滑梯度（近强远弱，无突变）
        - 病态网络：辐射强度出现异常波动（某些不该强的突然强）
        
        这就是"大海病了"的辐射信号。
        """
        if len(activations) < 3:
            return "节点太少，无法检测节奏"
        
        intensities = sorted(activations.values(), reverse=True)
        
        # 检查梯度是否平滑
        diffs = [intensities[i] - intensities[i+1] for i in range(len(intensities)-1)]
        
        if not diffs:
            return "平滑"
        
        avg_diff = sum(diffs) / len(diffs)
        
        # 如果有任何一步的跳跃超过平均值的3倍 = 异常突变
        max_diff = max(diffs) if diffs else 0
        if max_diff > avg_diff * 3 and max_diff > 0.05:
            return "异常突变（某个节点被异常激活）"
        
        # 如果强度分布过于均匀（没有梯度）= 一团浆液
        variance = sum((i - sum(intensities)/len(intensities))**2 for i in intensities) / len(intensities)
        if variance < 0.001:
            return "均匀浆糊（没有梯度，节点没有主次）"
        
        return "正常梯度（近强远弱，平滑衰减）"


# ============================================================
# 像素共振法官 —— 总控
# ============================================================

class PixelResonanceJudge:
    """
    像素共振法官（老四）
    
    从外部规则 → 内部涌现。
    决策不是靠规则 checklist，
    而是靠全局节点的辐射共振。
    """
    
    def __init__(self, storage_dir: str):
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.counter = 0
        self.network = None
    
    def build_network(self, nodes: List[PixelNode]) -> RadiationNetwork:
        """构建辐射网络"""
        network = RadiationNetwork(nodes)
        network.build_proximity()
        self.network = network
        return network
    
    def judge_pair(self, subject: PixelNode, candidate: PixelNode,
                   case_name: str = "") -> Dict:
        """
        判断一对节点是否相容。
        
        不是检查规则，而是：
        1. 让 subject 辐射，激活全局
        2. 所有在场节点对 candidate 给出意见
        3. 涌现出共振判断
        """
        if not self.network:
            self.build_network([subject, candidate])
        
        resonance = GlobalResonance(self.network)
        result = resonance.perceive(subject, candidate)
        
        # 附加信息
        result["subject"] = subject.name
        result["candidate"] = candidate.name
        result["subject_level"] = subject.level
        result["candidate_level"] = candidate.level
        result["similarity"] = round(subject.similarity_to(candidate), 4)
        result["case_name"] = case_name
        
        # 辐射可视化
        result["radiation"] = self.network.radiate(subject.name)
        
        return result
    
    def judge_system(self, nodes: List[PixelNode], case_name: str = "") -> Dict:
        """
        判断整个系统的全局共振状态。
        
        不是逐对检查节点，而是：
        1. 让网络自然辐射
        2. 计算全局的涌现和谐度
        """
        if not self.network or not all(n.name in self.network.nodes for n in nodes):
            self.build_network(nodes)
        
        # 全局涌现判断：从每个节点辐射，看整体
        all_perceptions = []
        for node in nodes:
            resonance = GlobalResonance(self.network)
            for other in nodes:
                if other.name != node.name:
                    p = resonance.perceive(node, other)
                    all_perceptions.append(p)
        
        if not all_perceptions:
            return {"resonance_score": 0, "verdict": "节点不足"}
        
        avg_resonance = sum(p["resonance_score"] for p in all_perceptions) / len(all_perceptions)
        
        # 检测节奏
        # 从中心节点（第一个）辐射
        activations = self.network.radiate(nodes[0].name)
        rhythm = GlobalResonance(self.network)._detect_rhythm(activations)
        
        # 维度分解
        avg_details = {}
        for key in ["type_resonance", "aesthetic_resonance", "goal_resonance", "level_resonance"]:
            vals = [p["details"].get(key, 0) for p in all_perceptions if "details" in p]
            avg_details[key] = round(sum(vals) / len(vals), 3) if vals else 0
        
        if avg_resonance >= 70:
            verdict = "系统健康（全局共振谐调）"
        elif avg_resonance >= 50:
            verdict = "系统亚健康（局部偏斜）"
        elif avg_resonance >= 30:
            verdict = "系统病态（全局失谐）"
        else:
            verdict = "系统崩溃（整体排斥）"
        
        self.counter += 1
        
        case = {
            "case_name": case_name,
            "version": f"老{self.counter}",
            "node_count": len(nodes),
            "resonance_score": round(avg_resonance, 2),
            "verdict": verdict,
            "rhythm": rhythm,
            "details": avg_details,
            "timestamp": time.time(),
        }
        
        self._save(case)
        return case
    
    def _save(self, case: Dict):
        """版本化保存"""
        version_dir = self.storage_dir / f"case_{self.counter:04d}"
        version_dir.mkdir(exist_ok=True)
        with open(version_dir / "record.json", "w", encoding="utf-8") as f:
            json.dump(case, f, ensure_ascii=False, indent=2)
    
    def render_verdict(self, case: Dict) -> str:
        """渲染判决"""
        lines = [
            f"\n{'='*60}",
            f"  像素共振法官 · 判决书",
            f"  案件：{case['case_name']}  [{case['version']}]",
            f"{'='*60}",
            f"  全局共振分：{case['resonance_score']}/100",
            f"  裁决：{case['verdict']}",
            f"  辐射节奏：{case['rhythm']}",
            f"  节点数：{case['node_count']}",
            f"  ─── 维度共振 ───",
        ]
        
        for dim, val in case.get("details", {}).items():
            bar = "█" * int((val + 1) * 25)  # -1~1 映射到 0~50
            lines.append(f"    {dim:<25} {val:+.3f} {bar}")
        
        lines.append(f"{'='*60}")
        return "\n".join(lines)
    
    def demonstrate_arithmetic(self, a: PixelNode, b: PixelNode) -> str:
        """演示像素四则运算"""
        arith = PixelArithmetic()
        
        added = arith.add(a.features, b.features)
        subtracted = arith.subtract(a.features, "weight")
        multiplied = arith.multiply(a.features, b.features)
        divided = arith.divide(a.features)
        
        lines = [
            f"\n  像素四则运算演示：{a.name} + {b.name}",
            f"  {'─'*40}",
            f"  {a.name} 特征: 重{a.features.weight:.1f} 密{a.features.density:.1f} 圆{a.features.roundness:.1f}",
            f"  {b.name} 特征: 重{b.features.weight:.1f} 密{b.features.density:.1f} 圆{b.features.roundness:.1f}",
            f"",
            f"  加法结果: 重{added.weight:.1f} 密{added.density:.1f} 圆{added.roundness:.1f}",
            f"  减法(移除weight): 重{subtracted.weight:.1f} 密{subtracted.density:.1f}",
            f"  乘法结果: 重{multiplied.weight:.1f} 密{multiplied.density:.1f} 圆{multiplied.roundness:.1f}",
            f"  除法(分解要素): {divided}",
        ]
        
        return "\n".join(lines)


# ============================================================
# 演示数据
# ============================================================

def build_phonetic_fragments() -> Dict[str, PixelFeatures]:
    """
    为常见符号构建像素特征。
    
    这些数据的依据：
    - 视觉块面：符号看起来"胖不胖"
    - 笔画疏密：结构复杂度
    - 圆弧/棱角：整体轮廓倾向
    """
    return {
        # 声母 - 相对均匀，标准圆润
        "b": PixelFeatures(weight=3.0, density=2.0, roundness=0.6, family="latin",
                           height_ratio=1.4, symmetry=0.4, topology=1.5),
        "p": PixelFeatures(weight=3.0, density=2.0, roundness=0.6, family="latin",
                           height_ratio=1.2, symmetry=0.4, topology=1.5),
        "m": PixelFeatures(weight=3.5, density=1.5, roundness=0.7, family="latin",
                           height_ratio=1.0, symmetry=0.5, topology=1.0),
        "f": PixelFeatures(weight=2.0, density=1.5, roundness=0.3, family="latin",
                           height_ratio=1.4, symmetry=0.3, topology=1.2),
        
        # 韵母 - 细骨架，风格各异
        "a": PixelFeatures(weight=2.0, density=1.5, roundness=0.7, family="latin",
                           height_ratio=1.0, symmetry=0.6, topology=1.0),
        "o": PixelFeatures(weight=2.5, density=1.0, roundness=0.95, family="latin",
                           height_ratio=1.0, symmetry=0.95, topology=1.0),
        "e": PixelFeatures(weight=2.0, density=1.0, roundness=0.8, family="latin",
                           height_ratio=1.0, symmetry=0.5, topology=1.0),
        
        # 瘦子
        "ɛ": PixelFeatures(weight=1.5, density=1.0, roundness=0.4, family="ipa",
                           height_ratio=1.2, symmetry=0.4, topology=1.0),
        
        # 胖子
        "☁": PixelFeatures(weight=8.0, density=6.0, roundness=0.6, family="cjk",
                           height_ratio=2.0, symmetry=0.3, topology=2.5),
        
        # 西里尔 - 块面更大
        "Ӗ": PixelFeatures(weight=4.0, density=2.5, roundness=0.5, family="cyrillic",
                           height_ratio=1.3, symmetry=0.7, topology=1.2),
        "Ɛ": PixelFeatures(weight=4.5, density=2.5, roundness=0.3, family="cyrillic",
                           height_ratio=1.3, symmetry=0.5, topology=1.2),
    }


def build_healthy_system() -> List[PixelNode]:
    """健康系统：所有节点风格统一"""
    frag = build_phonetic_fragments()
    
    nodes = [
        PixelNode("b", "声母", "基底层", "编码系统",
                  features=frag["b"], level="L1"),
        PixelNode("p", "声母", "基底层", "编码系统",
                  features=frag["p"], level="L1"),
        PixelNode("m", "声母", "基底层", "编码系统",
                  features=frag["m"], level="L1"),
    ]
    
    # 建立连接：同容器节点间连接较强
    for i, a in enumerate(nodes):
        for b in nodes:
            if a.name != b.name:
                a.connections[b.name] = 0.7 + 0.2 * a.similarity_to(b)
    
    return nodes


def build_thin_fat_mismatch() -> List[PixelNode]:
    """瘦子配胖子：审美冲突"""
    frag = build_phonetic_fragments()
    
    nodes = [
        PixelNode("a", "韵母", "基底层", "编码系统",
                  features=frag["a"], level="L1"),
        PixelNode("o", "韵母", "基底层", "编码系统",
                  features=frag["o"], level="L1"),
        PixelNode("ɛ", "韵母", "基底层", "编码系统",
                  features=frag["ɛ"], level="L1"),  # 瘦子
        PixelNode("☁", "韵母", "基底层", "编码系统",
                  features=frag["☁"], level="L1"),  # 胖子
    ]
    
    for i, a in enumerate(nodes):
        for b in nodes:
            if a.name != b.name:
                a.connections[b.name] = 0.5 + 0.3 * a.similarity_to(b)
    
    return nodes


def build_family_conflict() -> List[PixelNode]:
    """画风冲突：IPA + 西里尔混用"""
    frag = build_phonetic_fragments()
    
    nodes = [
        PixelNode("a", "韵母", "基底层", "编码系统",
                  features=frag["a"], level="L1"),
        PixelNode("ɛ", "韵母", "基底层", "编码系统",
                  features=frag["ɛ"], level="L1"),   # IPA 瘦子
        PixelNode("Ӗ", "韵母", "基底层", "编码系统",
                  features=frag["Ӗ"], level="L1"),  # 西里尔 胖子
        PixelNode("Ɛ", "韵母", "基底层", "编码系统",
                  features=frag["Ɛ"], level="L1"),  # 西里尔 胖子
    ]
    
    for i, a in enumerate(nodes):
        for b in nodes:
            if a.name != b.name:
                a.connections[b.name] = 0.5 + 0.3 * a.similarity_to(b)
    
    return nodes


def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions_pixel"
    judge = PixelResonanceJudge(storage)
    arith = PixelArithmetic()
    
    print("\n" + "█" * 60)
    print("  满全法 · 像素共振法官（老四）")
    print("  像素运算 + 辐射网络 + 全局涌现")
    print("  智慧不是规则——智慧是全局共振的涌现结果")
    print("█" * 60)
    
    frag = build_phonetic_fragments()
    
    # 演示1：像素四则运算
    print(judge.demonstrate_arithmetic(
        PixelNode("a", "韵母", "", "", features=frag["a"]),
        PixelNode("e", "韵母", "", "", features=frag["e"])
    ))
    
    # 演示2：单个节点的除法分解
    print(f"\n  符号 ☁ 的除法分解：")
    elements = arith.divide(frag["☁"])
    print(f"    基础要素: {elements}")
    
    print(f"\n  符号 a 的除法分解：")
    elements = arith.divide(frag["a"])
    print(f"    基础要素: {elements}")
    
    # 案例一：健康系统
    print(f"\n{'━'*60}")
    print(f"【案例一】健康系统 — 风格统一")
    nodes = build_healthy_system()
    case1 = judge.judge_system(nodes, "健康系统")
    print(judge.render_verdict(case1))
    
    # 显示辐射网络
    judge.network = judge.build_network(nodes)
    print(judge.network.global_presence("b"))
    
    # 案例二：瘦子配胖子
    print(f"\n{'━'*60}")
    print(f"【案例二】瘦子配胖子 — 审美冲突")
    nodes = build_thin_fat_mismatch()
    case2 = judge.judge_system(nodes, "瘦子配胖子")
    print(judge.render_verdict(case2))
    
    judge.network = judge.build_network(nodes)
    print(judge.network.global_presence("a"))
    
    # 案例三：画风冲突
    print(f"\n{'━'*60}")
    print(f"【案例三】IPA + 西里尔混用 — 画风冲突")
    nodes = build_family_conflict()
    case3 = judge.judge_system(nodes, "画风冲突")
    print(judge.render_verdict(case3))
    
    # 总结
    print(f"\n{'='*60}")
    print(f"  三个案例对比")
    print(f"{'='*60}")
    print(f"  {'案件':<20} {'共振分':>8} {'裁决'}")
    print(f"  {'─'*50}")
    for case in [case1, case2, case3]:
        print(f"  {case['case_name']:<20} {case['resonance_score']:>6.1f}   {case['verdict']}")
    
    print(f"\n  关键机制：")
    print(f"  1. 每个节点内部有像素特征（可加减乘除）")
    print(f"  2. 节点间通过相似度连接（有强度梯度）")
    print(f"  3. 处理一个节点时，涟漪辐射到所有关联节点")
    print(f"  4. 所有在场节点对目标给出意见（加权投票）")
    print(f"  5. 投票的涌现结果 = 判断（不是规则判的，是共振出来的）")
    print(f"  6. 辐射节奏检测：能感知大海是否病了")
    
    print(f"\n  版本存储：{storage}")


if __name__ == "__main__":
    main()
