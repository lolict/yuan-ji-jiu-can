#!/usr/bin/env python3
"""
层级治理系统 —— 视觉 · 天庭 · 审判 · 记忆

层级关系（高维）：
  不数米的颗粒，不命名每颗米。
  只看袋子：谁家的、值多少钱、好不好用、
  米脏不脏、有没有虫、晒干了没、能不能吃。

协作流程：
  天庭感知 → 视觉呈现 → 审判判定 → 记忆积累 → 反馈天庭
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional
from enum import Enum
import json
import os
import time


# ============================================================
# 袋子（容器）—— 高维权重对象
# ============================================================

class BagStatus(Enum):
    PRISTINE = "完好"
    USABLE = "可用"
    NEEDS_CLEAN = "需清洗"
    NEEDS_DRY = "需晾晒"
    INFESTED = "生虫"
    CONTAMINATED = "含沙"
    SPOILED = "变质"
    REJECT = "废弃"


@dataclass
class Bag:
    """袋子 —— 层级关系中的核心对象"""
    owner: str           # 谁家的袋子
    value: float         # 值多少钱
    content: str         # 内容物描述
    clean: bool = True       # 脏不脏
    dry: bool = True         # 晒干了没有
    has_worm: bool = False   # 有没有虫
    has_sand: bool = False   # 有没有沙子
    edible: bool = True      # 能不能吃
    washed: bool = False     # 洗了没有

    @property
    def status(self) -> BagStatus:
        if not self.edible:
            return BagStatus.SPOILED
        if self.has_worm:
            return BagStatus.INFESTED
        if self.has_sand:
            return BagStatus.CONTAMINATED
        if not self.dry:
            return BagStatus.NEEDS_DRY
        if not self.clean:
            return BagStatus.NEEDS_CLEAN
        return BagStatus.PRISTINE


# ============================================================
# 记忆层 —— 历史积累
# ============================================================

@dataclass
class JudgmentRecord:
    """审判记录"""
    bag_owner: str
    timestamp: float
    verdict: str
    reason: str
    reward: float = 0.0
    penalty: float = 0.0


class Memory:
    """
    历史的记忆力 —— 积累每次审判的结果
    不是记录米的颗粒，而是记录袋子的历史
    """

    def __init__(self, persist_path: str = None):
        self.records: List[JudgmentRecord] = []
        self.persist_path = persist_path
        if persist_path and os.path.exists(persist_path):
            self._load()

    def record(self, r: JudgmentRecord):
        self.records.append(r)
        if self.persist_path:
            self._save()

    def history_of(self, owner: str) -> List[JudgmentRecord]:
        return [r for r in self.records if r.bag_owner == owner]

    def reputation(self, owner: str) -> float:
        """袋子的声誉 —— 历史审判的净奖惩"""
        hist = self.history_of(owner)
        if not hist:
            return 0.0
        return sum(r.reward - r.penalty for r in hist)

    def trend(self, owner: str) -> str:
        """趋势 —— 袋子在变好还是变差"""
        hist = self.history_of(owner)
        if len(hist) < 2:
            return "未知"
        recent = hist[-3:]
        scores = [1 if r.verdict == "通过" else -1 for r in recent]
        avg = sum(scores) / len(scores)
        if avg > 0.3:
            return "↗ 好转"
        elif avg < -0.3:
            return "↘ 恶化"
        return "→ 平稳"

    def _save(self):
        data = [
            {
                "bag_owner": r.bag_owner,
                "timestamp": r.timestamp,
                "verdict": r.verdict,
                "reason": r.reason,
                "reward": r.reward,
                "penalty": r.penalty,
            }
            for r in self.records
        ]
        with open(self.persist_path, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)

    def _load(self):
        with open(self.persist_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        self.records = [
            JudgmentRecord(
                bag_owner=d["bag_owner"],
                timestamp=d["timestamp"],
                verdict=d["verdict"],
                reason=d["reason"],
                reward=d.get("reward", 0.0),
                penalty=d.get("penalty", 0.0),
            )
            for d in data
        ]


# ============================================================
# 天庭层 —— 感知 + 授权
# ============================================================

class HeavenlyCourt:
    """
    天庭 —— 感知袋子的状态，判定准入授权
    不是感知米的颗粒，而是感知袋子级别的状态
    """

    def __init__(self, memory: Memory):
        self.memory = memory

    def perceive(self, bag: Bag) -> Dict:
        """感知 —— 读取袋子的状态"""
        return {
            "owner": bag.owner,
            "value": bag.value,
            "status": bag.status.value,
            "clean": bag.clean,
            "dry": bag.dry,
            "has_worm": bag.has_worm,
            "has_sand": bag.has_sand,
            "edible": bag.edible,
            "washed": bag.washed,
            "reputation": self.memory.reputation(bag.owner),
            "trend": self.memory.trend(bag.owner),
        }

    def authorize(self, bag: Bag) -> tuple:
        """
        授权 —— 判断袋子是否有资格进入下一层
        返回 (是否通过, 原因)
        """
        # 历史声誉太差 → 自动拒绝
        if self.memory.reputation(bag.owner) < -10:
            return False, "历史声誉过低，天庭拒绝授权"

        # 变质 → 拒绝
        if not bag.edible:
            return False, "内容物已变质，天庭拒绝授权"

        return True, "天庭授权通过"


# ============================================================
# 审判层 —— 判定 + 奖惩
# ============================================================

class Judgment:
    """
    审判 —— 基于天庭感知的结果，执行判定
    审判对了给奖励，审判错了给惩罚
    奖惩对象是袋子（不是米）
    """

    def __init__(self, memory: Memory):
        self.memory = memory

    def judge(self, bag: Bag, perception: Dict) -> JudgmentRecord:
        """审判 —— 针对袋子的整体判定"""
        score = 0.0
        reasons = []

        # === 视觉维度：袋子状态 ===
        if bag.clean:
            score += 2
        else:
            score -= 1
            reasons.append("袋子脏")

        if bag.dry:
            score += 2
        else:
            score -= 1
            reasons.append("没晒干")

        if not bag.has_worm:
            score += 3
        else:
            score -= 5
            reasons.append("生虫")

        if not bag.has_sand:
            score += 2
        else:
            score -= 3
            reasons.append("含沙")

        if bag.edible:
            score += 3
        else:
            score -= 10
            reasons.append("不能吃")

        # === 记忆维度：历史声誉 ===
        rep = perception.get("reputation", 0)
        if rep > 5:
            score += 2
        elif rep < -5:
            score -= 3
            reasons.append("历史声誉差")

        trend = perception.get("trend", "")
        if "好转" in trend:
            score += 1
        elif "恶化" in trend:
            score -= 2
            reasons.append("趋势恶化")

        # === 判定 ===
        if score >= 5:
            verdict = "通过"
            reward = score * 0.5
            penalty = 0.0
            reason = "状态良好" if not reasons else "；".join(reasons)
        elif score >= 0:
            verdict = "有条件通过"
            reward = score * 0.2
            penalty = 0.0
            reason = "；".join(reasons) if reasons else "基本合格"
        else:
            verdict = "不通过"
            reward = 0.0
            penalty = abs(score) * 0.5
            reason = "；".join(reasons) if reasons else "不合格"

        record = JudgmentRecord(
            bag_owner=bag.owner,
            timestamp=time.time(),
            verdict=verdict,
            reason=reason,
            reward=reward,
            penalty=penalty,
        )
        self.memory.record(record)
        return record


# ============================================================
# 视觉层 —— 状态可视化
# ============================================================

class Vision:
    """
    视觉 —— 呈现袋子和米袋子的状态
    让人一眼看到：哪个袋子有问题、哪个没问题
    不是数米，是看袋子级别的状态
    """

    STATUS_ICON = {
        BagStatus.PRISTINE: "🟢",
        BagStatus.USABLE: "🟢",
        BagStatus.NEEDS_CLEAN: "🟡",
        BagStatus.NEEDS_DRY: "🟡",
        BagStatus.INFESTED: "🔴",
        BagStatus.CONTAMINATED: "🔴",
        BagStatus.SPOILED: "⚫",
        BagStatus.REJECT: "⛔",
    }

    VERDICT_ICON = {
        "通过": "✅",
        "有条件通过": "⚠️",
        "不通过": "❌",
    }

    @classmethod
    def show_bag(cls, bag: Bag, perception: Dict = None):
        """展示单个袋子的视觉状态"""
        icon = cls.STATUS_ICON.get(bag.status, "?")
        rep = perception.get("reputation", 0) if perception else 0
        trend = perception.get("trend", "未知") if perception else "未知"

        print(f"\n{'='*50}")
        print(f"{icon} 袋子：{bag.owner}家")
        print(f"   状态：{bag.status.value}")
        print(f"   价值：{bag.value} 元")
        print(f"   内容物：{bag.content}")
        print(f"   ─────────────────────────")
        print(f"   脏不脏：{'✗ 脏' if not bag.clean else '✓ 干净'}")
        print(f"   晒干没：{'✗ 未干' if not bag.dry else '✓ 已干'}")
        print(f"   有虫吗：{'✗ 有虫' if bag.has_worm else '✓ 无虫'}")
        print(f"   有沙吗：{'✗ 含沙' if bag.has_sand else '✓ 无沙'}")
        print(f"   能吃吗：{'✗ 不能吃' if not bag.edible else '✓ 能吃'}")
        print(f"   洗了吗：{'✗ 未洗' if not bag.washed else '✓ 已洗'}")
        print(f"   ─────────────────────────")
        print(f"   历史声誉：{rep:+.1f}")
        print(f"   趋势：{trend}")

    @classmethod
    def show_judgment(cls, record: JudgmentRecord):
        """展示审判结果"""
        icon = cls.VERDICT_ICON.get(record.verdict, "?")
        print(f"\n{'─'*50}")
        print(f"{icon} 审判结果")
        print(f"   袋子：{record.bag_owner}家")
        print(f"   判决：{record.verdict}")
        print(f"   原因：{record.reason}")
        print(f"   奖励：+{record.reward:.1f}")
        print(f"   惩罚：-{record.penalty:.1f}")

    @classmethod
    def show_all_bags(cls, bags: List[Bag], heavenly_court: HeavenlyCourt):
        """展示所有袋子的视觉总览"""
        print(f"\n{'='*60}")
        print(f"  【视觉总览】所有袋子状态")
        print(f"{'='*60}")
        print(f"  {'袋子':<12} {'状态':<10} {'声誉':>6} {'趋势':<8}")
        print(f"  {'─'*44}")
        for bag in bags:
            p = heavenly_court.perceive(bag)
            icon = cls.STATUS_ICON.get(bag.status, "?")
            rep = p["reputation"]
            trend = p["trend"]
            print(f"  {icon} {bag.owner:<10} {bag.status.value:<10} {rep:>+5.1f} {trend}")

    @classmethod
    def show_history(cls, memory: Memory, owner: str):
        """展示袋子的历史审判记录"""
        hist = memory.history_of(owner)
        print(f"\n{'='*50}")
        print(f"  【历史记忆】{owner}家袋子的审判记录")
        print(f"{'='*50}")
        if not hist:
            print("  无历史记录")
            return
        for r in hist:
            icon = cls.VERDICT_ICON.get(r.verdict, "?")
            print(f"  {icon} {r.verdict} | +{r.reward:.1f}/-{r.penalty:.1f} | {r.reason}")


# ============================================================
# 主程序 —— 运行 Demo
# ============================================================

def main():
    # 初始化四层
    memory = Memory()
    heavenly_court = HeavenlyCourt(memory)
    judgment = Judgment(memory)
    vision = Vision()

    # 创建一批袋子（不是米）
    bags = [
        Bag(owner="张", value=100, content="东北大米",
            clean=True, dry=True, has_worm=False, has_sand=False, edible=True),
        Bag(owner="李", value=80, content="江南糯米",
            clean=False, dry=True, has_worm=False, has_sand=True, edible=True),
        Bag(owner="王", value=120, content="泰国香米",
            clean=True, dry=False, has_worm=True, has_sand=False, edible=True),
        Bag(owner="赵", value=60, content="陈年旧米",
            clean=False, dry=False, has_worm=True, has_sand=True, edible=False),
        Bag(owner="刘", value=150, content="有机新米",
            clean=True, dry=True, has_worm=False, has_sand=False, edible=True, washed=True),
    ]

    # 视觉总览
    vision.show_all_bags(bags, heavenly_court)

    # 逐个走流程：天庭感知 → 天庭授权 → 审判 → 视觉呈现
    print(f"\n{'='*60}")
    print(f"  【流程演示】天庭 → 审判 → 视觉")
    print(f"{'='*60}")

    for bag in bags:
        # 天庭感知
        perception = heavenly_court.perceive(bag)

        # 天庭授权
        authorized, auth_reason = heavenly_court.authorize(bag)
        auth_icon = "✓" if authorized else "✗"
        print(f"\n{auth_icon} 天庭授权：{bag.owner}家 → {auth_reason}")

        if not authorized:
            # 未授权也要记录
            record = JudgmentRecord(
                bag_owner=bag.owner,
                timestamp=time.time(),
                verdict="不通过",
                reason=f"天庭未授权：{auth_reason}",
                reward=0.0,
                penalty=1.0,
            )
            memory.record(record)
            continue

        # 审判
        record = judgment.judge(bag, perception)

        # 视觉呈现结果
        vision.show_bag(bag, perception)
        vision.show_judgment(record)

    # 最终视觉总览 + 历史记忆
    vision.show_all_bags(bags, heavenly_court)

    # 展示历史记忆
    print(f"\n{'='*60}")
    print(f"  【历史记忆积累】")
    print(f"{'='*60}")
    for bag in bags:
        vision.show_history(memory, bag.owner)

    # 总结
    print(f"\n{'='*60}")
    print(f"  【总结】")
    print(f"{'='*60}")
    print(f"  层级关系：袋子级别（高维），不数米（低维）")
    print(f"  视觉：看到袋子的状态（脏/湿/虫/沙/变质）")
    print(f"  天庭：感知袋子状态 + 授权准入")
    print(f"  审判：基于感知和历史，判定袋子是否合格")
    print(f"  记忆：积累每次审判，形成声誉和趋势")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
