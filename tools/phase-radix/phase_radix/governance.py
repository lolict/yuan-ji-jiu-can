"""
分布式制衡校验流水线

完整的校验执行顺序（不可调换）：

第 0 关：意图甄别（区分正向指令/批判/错误样例/纯情绪）
第 1 关：层级栈校验（L1/L2/L3 不得跨层运算）
第 2 关：类型身份校验（声母/韵母/声调一物一身份）
第 3 关：视觉审美校验（胖瘦字重、画风家族、轮廓风格）
第 4 关：归属权校验（外来素材必须重定义归属）
第 5 关：衍射规则校验（只有 L1 基元本体之间允许衍射）
第 6 关：黑名单反面样本检测

最终：全局综合协调得分（模拟共振场效果）

设计理念：
- 每个维度都有平等的否决权（我压你一头，你压我一头）
- 任何一个维度投否决 → 整体拒绝
- 所有维度都勉强过关，但全局协调分太低 → 也拒绝
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
import math


# ============================================================
# 校验结果
# ============================================================

@dataclass
class ValidationResult:
    """单个维度的校验结果"""
    dimension: str          # 维度名称
    passed: bool            # 是否通过
    veto: bool              # 是否投否决票（致命问题）
    score: float            # 连续数值 (0~1, 不是布尔)
    details: str            # 详细说明
    evidence: Dict = field(default_factory=dict)  # 证据

    def __str__(self):
        status = "✓" if self.passed else "✗"
        veto_mark = " [否决!!]" if self.veto else ""
        return f"  {status} {self.dimension}: {self.score:.2f} - {self.details}{veto_mark}"


@dataclass
class GovernanceReport:
    """治理报告 = 所有维度的综合校验结果"""
    subject: str                                    # 被校验的对象
    results: List[ValidationResult] = field(default_factory=list)
    global_score: float = 0.0                        # 全局协调分
    verdict: str = ""                                # 最终裁决
    vetoed_by: List[str] = field(default_factory=list)  # 哪些维度投了否决票
    
    def summary(self) -> str:
        lines = [
            f"\n{'='*60}",
            f"  治理报告：{self.subject}",
            f"{'='*60}",
            f"  全局协调分：{self.global_score:.2f}/100",
            f"  最终裁决：{self.verdict}",
        ]
        
        if self.vetoed_by:
            lines.append(f"  被以下维度否决：{', '.join(self.vetoed_by)}")
        
        lines.append(f"\n  ─── 各维度校验详情 ───")
        for r in self.results:
            lines.append(str(r))
        
        lines.append(f"{'='*60}")
        return "\n".join(lines)


# ============================================================
# 各维度校验器
# ============================================================

class IntentionFilter:
    """
    第 0 关：意图甄别
    
    输入分类：
    - FORWARD：正向指令 → 正常进入后续校验
    - CRITICISM：批判/斥责 → 提取错误行为写入黑名单，不进入后续
    - ERROR_DEMO：错误示范/笔误 → 写入黑名单检测库
    - GARBAGE：纯情绪垃圾 → 直接丢弃
    """
    
    FORWARD = "FORWARD"
    CRITICISM = "CRITICISM"
    ERROR_DEMO = "ERROR_DEMO"
    GARBAGE = "GARBAGE"
    
    def classify(self, text: str) -> Tuple[str, Optional[str]]:
        """
        对输入文本做意图分类
        
        返回：(分类, 提取的错误行为)
        
        实际实现中这里可以用关键词启发式，也可以对接外部分类器。
        """
        text_lower = text.lower()
        
        # 简单启发式（可替换为更复杂的分类）
        criticism_markers = ["错了", "不对", "垃圾", "蠢", "恶心", "不要这样做"]
        error_demo_markers = ["这是错误示范", "不要学这个", "反例"]
        
        for marker in criticism_markers:
            if marker in text_lower:
                return self.CRITICISM, text
        
        for marker in error_demo_markers:
            if marker in text_lower:
                return self.ERROR_DEMO, text
        
        # 过短且无意义 → 垃圾
        if len(text.strip()) < 3:
            return self.GARBAGE, None
        
        return self.FORWARD, None
    
    def validate(self, text: str) -> ValidationResult:
        """对输入做意图校验"""
        intention, extracted = self.classify(text)
        
        if intention == self.FORWARD:
            return ValidationResult(
                dimension="意图甄别",
                passed=True,
                veto=False,
                score=1.0,
                details="正向指令，进入后续校验",
            )
        elif intention == self.CRITICISM:
            return ValidationResult(
                dimension="意图甄别",
                passed=False,
                veto=False,  # 不投否决（不进入后续），但不叫"否决"
                score=0.0,
                details=f"识别到批判输入，错误行为已提取并记录，不进入编码生成流程",
                evidence={"extracted_error": extracted},
            )
        elif intention == self.ERROR_DEMO:
            return ValidationResult(
                dimension="意图甄别",
                passed=False,
                veto=False,
                score=0.0,
                details="识别到错误示范，仅用于检测，不执行",
                evidence={"example": extracted},
            )
        else:
            return ValidationResult(
                dimension="意图甄别",
                passed=False,
                veto=False,
                score=0.0,
                details="无法识别的输入，丢弃",
            )


class LevelStackValidator:
    """
    第 1 关：层级栈校验
    
    L1 = 基元本体（不带任何声调修饰）
    L2 = 修饰零件（裸漂浮变音符号）
    L3 = 装配成品（L1 + L2）
    
    规则：
    - 衍射/配对/运算只允许同层级之间发生
    - L1 ↔ L3 的配对 = 层级跳跃 = 致命错误
    - L2 永远不能独立参与编码
    """
    
    def validate_pair(self, level_a: str, level_b: str) -> ValidationResult:
        """校验两个节点的层级是否允许配对"""
        
        # 任何涉及 L2 的独立配对都是错的
        if level_a == "L2" or level_b == "L2":
            return ValidationResult(
                dimension="层级栈校验",
                passed=False,
                veto=True,
                score=0.0,
                details=f"L2修饰零件不能独立参与配对 (A={level_a}, B={level_b})",
                evidence={"level_a": level_a, "level_b": level_b},
            )
        
        # L1 和 L3 不能配对（跨层级）
        if level_a != level_b:
            return ValidationResult(
                dimension="层级栈校验",
                passed=False,
                veto=True,
                score=0.0,
                details=f"跨层级错误：{level_a} ↔ {level_b} 不允许配对",
                evidence={"level_a": level_a, "level_b": level_b},
            )
        
        return ValidationResult(
            dimension="层级栈校验",
            passed=True,
            veto=False,
            score=1.0,
            details=f"同层级配对 ({level_a}) 允许",
        )


class TypeIdentityValidator:
    """
    第 2 关：类型身份校验
    
    一物一身份：
    - 声母本体 → 只能做声母
    - 韵母本体 → 只能做韵母
    - 声调成品 → 只能进声调封套
    - 修饰零件 → 只能做加工工具，不能独立编码
    """
    
    VALID_TYPES = {"声母", "韵母", "声调", "修饰零件"}
    
    def validate(self, claimed_type: str, actual_usage: str) -> ValidationResult:
        """校验声明类型与实际用途是否一致"""
        
        if claimed_type not in self.VALID_TYPES:
            return ValidationResult(
                dimension="类型身份校验",
                passed=False,
                veto=True,
                score=0.0,
                details=f"未知类型: {claimed_type}",
            )
        
        # 修饰零件不能独立使用
        if claimed_type == "修饰零件" and actual_usage != "工具":
            return ValidationResult(
                dimension="类型身份校验",
                passed=False,
                veto=True,
                score=0.0,
                details="修饰零件不能独立充当编码条目",
            )
        
        # 类型必须与用途一致
        if claimed_type != actual_usage and actual_usage != "工具":
            return ValidationResult(
                dimension="类型身份校验",
                passed=False,
                veto=True,
                score=0.1,
                details=f"类型错配：声明为{claimed_type}，实际用途为{actual_usage}",
            )
        
        return ValidationResult(
            dimension="类型身份校验",
            passed=True,
            veto=False,
            score=1.0,
            details=f"类型匹配：{claimed_type}",
        )


class AestheticValidator:
    """
    第 3 关：视觉审美校验
    
    检查：
    1. 胖瘦字重是否均衡（不匹配 = 瘦子配胖子）
    2. 画风家族是否统一（不匹配 = IPA+西里尔混用）
    3. 轮廓风格是否一致（圆 vs 棱角）
    """
    
    def validate_pair(self, weight_a: float, weight_b: float,
                      family_a: str, family_b: str,
                      round_a: float, round_b: float) -> ValidationResult:
        """校验一对符号的审美兼容性"""
        
        # 字重差异
        weight_diff = abs(weight_a - weight_b)
        weight_score = max(0, 1.0 - weight_diff / 5.0)
        
        # 家族一致性
        family_score = 1.0 if family_a == family_b else 0.2
        
        # 轮廓一致性
        round_diff = abs(round_a - round_b)
        round_score = max(0, 1.0 - round_diff)
        
        # 综合审美分
        aesthetic_score = (weight_score * 0.5 + family_score * 0.3 + round_score * 0.2)
        
        # 字重差异超过阈值 = 否决
        veto = weight_diff > 3.0
        
        return ValidationResult(
            dimension="视觉审美校验",
            passed=aesthetic_score >= 0.6,
            veto=veto,
            score=aesthetic_score,
            details=f"字重差:{weight_diff:.1f}, 家族匹配:{family_a==family_b}, 轮廓差:{round_diff:.2f}",
            evidence={
                "weight_diff": weight_diff,
                "family_match": family_a == family_b,
                "round_diff": round_diff,
            },
        )


class OwnershipValidator:
    """
    第 4 关：归属权校验
    
    SELF = 自有（可进入本体库）
    EXTERNAL = 外来（必须经过解构-重归属流程才能纳入本体库）
    """
    
    def validate(self, owner: str, is_redefined: bool = False) -> ValidationResult:
        if owner == "SELF":
            return ValidationResult(
                dimension="归属权校验",
                passed=True,
                veto=False,
                score=1.0,
                details="自有编码",
            )
        elif owner == "EXTERNAL":
            if is_redefined:
                return ValidationResult(
                    dimension="归属权校验",
                    passed=True,
                    veto=False,
                    score=0.8,
                    details="外来素材已经过解构-重归属流程",
                )
            else:
                return ValidationResult(
                    dimension="归属权校验",
                    passed=False,
                    veto=True,
                    score=0.1,
                    details="外来素材未经重定义，禁止进入本体库",
                )
        else:
            return ValidationResult(
                dimension="归属权校验",
                passed=False,
                veto=True,
                score=0.0,
                details=f"未知归属权: {owner}",
            )


class DiffractionRuleValidator:
    """
    第 5 关：衍射规则校验
    
    衍射只允许 L1 基元本体之间发生。
    禁止：L1 ↔ L3、L2 ↔ anything 的衍射。
    """
    
    def validate_diffraction(self, source_level: str, target_level: str) -> ValidationResult:
        if source_level != "L1" or target_level != "L1":
            return ValidationResult(
                dimension="衍射规则校验",
                passed=False,
                veto=True,
                score=0.0,
                details=f"非法衍射：{source_level} ↔ {target_level}（只允许 L1↔L1）",
            )
        
        return ValidationResult(
            dimension="衍射规则校验",
            passed=True,
            veto=False,
            score=1.0,
            details="合法衍射（L1 基元↔L1 基元）",
        )


class BlacklistValidator:
    """
    第 6 关：黑名单反面样本检测
    """
    
    def __init__(self):
        self.blacklist: List[Dict] = []
    
    def add_to_blacklist(self, error_pattern: str, reason: str) -> None:
        self.blacklist.append({
            "pattern": error_pattern,
            "reason": reason,
        })
    
    def validate(self, text: str) -> ValidationResult:
        for entry in self.blacklist:
            if entry["pattern"] in text:
                return ValidationResult(
                    dimension="黑名单检测",
                    passed=False,
                    veto=True,
                    score=0.0,
                    details=f"命中黑名单: {entry['reason']}",
                    evidence={"pattern": entry["pattern"]},
                )
        
        return ValidationResult(
            dimension="黑名单检测",
            passed=True,
            veto=False,
            score=1.0,
            details="未命中黑名单",
        )


class GlobalCoordinationScore:
    """
    全局综合协调得分（模拟共振场效果）
    
    不是任何一个维度的判断，而是所有维度拉力的综合涌现。
    
    计算方式：
    - 每个维度输出一个连续数值（不是布尔通过/否决）
    - 维度权重体现"公约数"（顶层目标）的引力分配
    - 最终得分 = 加权综合
    
    关键：某个维度可能"通过"（>0.6），但如果多个维度都略低，
    综合分也会显著下降。这就是"大海病了"。
    """
    
    DIMENSION_WEIGHTS = {
        "意图甄别": 0.8,
        "层级栈校验": 1.5,      # 层级最致命
        "类型身份校验": 1.2,
        "视觉审美校验": 1.0,
        "归属权校验": 1.3,
        "衍射规则校验": 1.4,
        "黑名单检测": 1.0,
    }
    
    def compute(self, results: List[ValidationResult]) -> Tuple[float, str]:
        """
        计算全局协调分
        
        返回：(分数 0~100, 裁决描述)
        """
        if not results:
            return 0.0, "无校验数据"
        
        total_weight = 0
        weighted_sum = 0
        
        for result in results:
            weight = self.DIMENSION_WEIGHTS.get(result.dimension, 1.0)
            
            # 投否决的维度权重加倍（否决的破坏力更大）
            if result.veto:
                weight *= 2.0
            
            weighted_sum += result.score * weight
            total_weight += weight
        
        if total_weight == 0:
            return 0.0, "权重为零"
        
        raw_score = weighted_sum / total_weight
        final_score = raw_score * 100
        
        if final_score >= 80:
            verdict = "健康（各维度谐调）"
        elif final_score >= 60:
            verdict = "亚健康（轻微偏斜）"
        elif final_score >= 40:
            verdict = "病态（系统性偏斜）"
        else:
            verdict = "崩溃（整体失调）"
        
        return final_score, verdict


# ============================================================
# 总流水线
# ============================================================

class GovernancePipeline:
    """
    治理流水线 = 所有校验维度按序执行
    
    执行顺序不可调换。
    任何一个维度投否决 → 整体拒绝（但后续维度继续执行，保证报告完整）。
    """
    
    def __init__(self):
        self.intention_filter = IntentionFilter()
        self.level_validator = LevelStackValidator()
        self.type_validator = TypeIdentityValidator()
        self.aesthetic_validator = AestheticValidator()
        self.ownership_validator = OwnershipValidator()
        self.diffraction_validator = DiffractionRuleValidator()
        self.blacklist_validator = BlacklistValidator()
        self.coordination_scorer = GlobalCoordinationScore()
    
    def validate_pair(self, 
                      node_a: Dict, node_b: Dict,
                      operation: str = "pairing") -> GovernanceReport:
        """
        校验一对节点是否可以配对/衍射/合作
        
        node_a, node_b 是包含以下字段的字典：
        - name: 名称
        - level: L1/L2/L3
        - type: 声母/韵母/声调/修饰零件
        - owner: SELF/EXTERNAL
        - weight: 视觉重量
        - family: 字形家族
        - roundness: 圆度
        """
        report = GovernanceReport(subject=f"{node_a.get('name')} ↔ {node_b.get('name')}")
        
        # 第 1 关：层级
        level_result = self.level_validator.validate_pair(
            node_a.get("level", "L1"),
            node_b.get("level", "L1"),
        )
        report.results.append(level_result)
        if level_result.veto:
            report.vetoed_by.append("层级栈")
        
        # 第 2 关：类型
        type_result_a = self.type_validator.validate(
            node_a.get("type", "unknown"),
            node_a.get("type", "unknown"),
        )
        type_result_b = self.type_validator.validate(
            node_b.get("type", "unknown"),
            node_b.get("type", "unknown"),
        )
        # 取较差的那个
        type_result = type_result_a if type_result_a.score < type_result_b.score else type_result_b
        type_result.details = f"A:{node_a.get('type')} B:{node_b.get('type')} - {type_result.details}"
        report.results.append(type_result)
        if type_result.veto:
            report.vetoed_by.append("类型身份")
        
        # 第 3 关：审美
        aesthetic_result = self.aesthetic_validator.validate_pair(
            node_a.get("weight", 5.0),
            node_b.get("weight", 5.0),
            node_a.get("family", "latin"),
            node_b.get("family", "latin"),
            node_a.get("roundness", 0.5),
            node_b.get("roundness", 0.5),
        )
        report.results.append(aesthetic_result)
        if aesthetic_result.veto:
            report.vetoed_by.append("视觉审美")
        
        # 第 4 关：归属权
        ownership_result = self.ownership_validator.validate(
            node_a.get("owner", "SELF"),
        )
        report.results.append(ownership_result)
        if ownership_result.veto:
            report.vetoed_by.append("归属权")
        
        # 第 5 关：衍射规则
        if operation in ("diffraction", "pairing"):
            diffraction_result = self.diffraction_validator.validate_diffraction(
                node_a.get("level", "L1"),
                node_b.get("level", "L1"),
            )
            report.results.append(diffraction_result)
            if diffraction_result.veto:
                report.vetoed_by.append("衍射规则")
        
        # 第 6 关：黑名单
        blacklist_result = self.blacklist_validator.validate(
            node_a.get("name", "") + node_b.get("name", ""),
        )
        report.results.append(blacklist_result)
        if blacklist_result.veto:
            report.vetoed_by.append("黑名单")
        
        # 全局协调分
        global_score, verdict = self.coordination_scorer.compute(report.results)
        report.global_score = global_score
        
        # 最终裁决
        if report.vetoed_by:
            report.verdict = f"拒绝（被 {', '.join(report.vetoed_by)} 否决）"
        elif global_score >= 60:
            report.verdict = f"通过（全局协调分 {global_score:.1f}）"
        else:
            report.verdict = f"拒绝（全局协调分过低 {global_score:.1f}，系统偏斜）"
        
        return report
