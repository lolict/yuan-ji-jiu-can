"""
分布式编织制衡网络 · 天平称量

维度之间互相拉扯、互相校验、互相压制
  - 任一方面发现问题就输出否决拉力
  - 不是单条规则独大
  - 最终是多方拉力平衡的结果
"""

from __future__ import annotations
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass, field
from .pixel import PhasePixel, PhaseCanvas


@dataclass
class BalanceVerdict:
    """审判结果"""
    passed: bool
    dimension_votes: Dict[str, Tuple[bool, float, str]]
    global_score: float
    rejecting_dimensions: List[str]
    summary: str


class BalanceDimension:
    """单个制衡维度"""
    
    def __init__(self, name: str, weight: float = 1.0, veto_power: float = 1.0):
        self.name = name
        self.weight = weight
        self.veto_power = veto_power
    
    def check(self, base: PhasePixel, target: PhasePixel) -> Tuple[bool, float, str]:
        raise NotImplementedError


class LevelDimension(BalanceDimension):
    """层级栈校验: 只能同级配对"""
    
    def __init__(self):
        super().__init__("层级栈", weight=1.0, veto_power=1.0)
    
    def check(self, base: PhasePixel, target: PhasePixel):
        if base.level == target.level:
            return True, 1.0, f"同级L{base.level}"
        elif abs(base.level - target.level) == 1:
            return False, 0.3, f"跨一级(L{base.level}↔L{target.level})"
        else:
            return False, 0.0, f"层级跳跃(L{base.level}↔L{target.level})"


class TypeDimension(BalanceDimension):
    """类型/家族校验"""
    
    def __init__(self):
        super().__init__("类型身份", weight=1.0, veto_power=1.0)
    
    def check(self, base: PhasePixel, target: PhasePixel):
        if base.family == target.family:
            return True, 1.0, f"同族{base.family}"
        compatible_families = [
            ("ipa_extend", "latin"),
            ("ipa_extend", "cyrillic"),
            ("latin", "ipa_extend"),
        ]
        pair = (base.family, target.family)
        if pair in compatible_families:
            return True, 0.7, "跨族兼容"
        return False, 0.2, f"家族不同({base.family}↔{target.family})"


class AestheticDimension(BalanceDimension):
    """视觉审美校验（胖瘦/权重）"""
    
    def __init__(self, threshold: float = 0.35):
        super().__init__("视觉审美", weight=1.0, veto_power=1.0)
        self.threshold = threshold
    
    def check(self, base: PhasePixel, target: PhasePixel):
        diff = abs(base.weight - target.weight)
        if diff <= 0.15:
            return True, 1.0, f"视觉协调(diff={diff:.2f})"
        elif diff <= self.threshold:
            return True, 0.6, f"勉强协调(diff={diff:.2f})"
        else:
            return False, 0.0, f"视觉差大(diff={diff:.2f},瘦子配胖子)"


class OwnershipDimension(BalanceDimension):
    """归属权校验"""
    
    def __init__(self):
        super().__init__("归属权", weight=1.0, veto_power=0.9)
    
    def check(self, base: PhasePixel, target: PhasePixel):
        if base.owner == "SELF" and target.owner == "SELF":
            return True, 1.0, "双方均为己方"
        elif base.owner == "EXTERNAL" or target.owner == "EXTERNAL":
            return False, 0.2, "含外来素材(需先解构重归属)"
        return True, 0.8, "归属正常"


class DiffractionDimension(BalanceDimension):
    """衍射规则校验: 同基础+变音不是独立配对"""
    
    def __init__(self):
        super().__init__("衍射规则", weight=1.0, veto_power=1.0)
    
    def check(self, base: PhasePixel, target: PhasePixel):
        if base.phase == target.phase and base.level != target.level:
            return False, 0.0, "同基础跨层级(应触发衍射)"
        return True, 1.0, "独立基元配对"


class BlacklistDimension(BalanceDimension):
    """黑名单反面样本检测"""
    
    def __init__(self):
        super().__init__("黑名单", weight=1.0, veto_power=1.0)
    
    def check(self, base: PhasePixel, target: PhasePixel):
        if base.blacklisted or target.blacklisted:
            return False, 0.0, "黑名单标记"
        return True, 1.0, "无黑名单"


class BalanceNetwork:
    """分布式编织制衡网络"""
    
    def __init__(self):
        self.dimensions: List[BalanceDimension] = []
    
    def add_dimension(self, dim: BalanceDimension):
        self.dimensions.append(dim)
    
    def default_setup(self):
        """默认配置: 六个维度"""
        self.dimensions = [
            LevelDimension(),
            TypeDimension(),
            AestheticDimension(),
            OwnershipDimension(),
            DiffractionDimension(),
            BlacklistDimension(),
        ]
    
    def judge(self, base: PhasePixel, target: PhasePixel) -> BalanceVerdict:
        """运行全部维度的审判"""
        votes = {}
        rejecting = []
        all_passed = True
        weighted_sum = 0.0
        total_weight = 0.0
        
        for dim in self.dimensions:
            passed, confidence, reason = dim.check(base, target)
            votes[dim.name] = (passed, confidence, reason)
            weighted_sum += confidence * dim.weight
            total_weight += dim.weight
            
            if not passed and dim.veto_power >= 1.0:
                rejecting.append(dim.name)
                all_passed = False
            elif not passed:
                all_passed = False
        
        global_score = weighted_sum / max(total_weight, 0.001)
        
        if rejecting:
            passed = False
        elif global_score < 0.5:
            passed = False
            rejecting.append("全局协调分不足")
        else:
            passed = all_passed
        
        summary = "通过" if passed else f"拒绝({','.join(rejecting)})"
        
        return BalanceVerdict(
            passed=passed,
            dimension_votes=votes,
            global_score=global_score,
            rejecting_dimensions=rejecting,
            summary=summary,
        )
    
    def __repr__(self):
        return f"BalanceNetwork({len(self.dimensions)} dims)"
