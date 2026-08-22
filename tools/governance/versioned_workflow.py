#!/usr/bin/env python3
"""
版本化层级工作流系统

解决什么问题：
  AI 蠢，做错了不知道怎么办，只会改旧文件。
   sekarang: 每次出新版本（老大、老二、老三...）
  所有历史版本都留着，对比挑对的。
  错了不改旧的，直接出新的。

系统：
  视觉 → 听觉(授权) → 审判(奖惩) → 记忆(历史积累)
  层级关系：袋子级别（高维），不数米（低维）
"""

import os
import json
import shutil
import hashlib
import time
from dataclasses import dataclass, field, asdict
from typing import List, Dict, Optional, Callable
from enum import Enum
from pathlib import Path


# ============================================================
# 袋子（容器）—— 层级对象
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
    """袋子 —— 只看袋子级别，不数米"""
    owner: str
    value: float
    content: str
    clean: bool = True
    dry: bool = True
    has_worm: bool = False
    has_sand: bool = False
    edible: bool = True
    washed: bool = False

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
# 版本号 —— 老大、老二、老三...
# ============================================================

class VersionCounter:
    """版本计数 —— 报数：老大、老二、老三..."""

    NAMES = [
        "老大", "老二", "老三", "老四", "老五",
        "老六", "老七", "老八", "老九", "老十",
        "老十一", "老十二", "老十三", "老十四", "老十五",
    ]

    def __init__(self):
        self.count = 0

    def next(self) -> str:
        self.count += 1
        if self.count <= len(self.NAMES):
            return self.NAMES[self.count - 1]
        return f"老{self.count}"

    @property
    def current(self) -> str:
        if self.count == 0:
            return "无"
        if self.count <= len(self.NAMES):
            return self.NAMES[self.count - 1]
        return f"老{self.count}"


# ============================================================
# 视觉层
# ============================================================

class Vision:
    """视觉 —— 看袋子状态（脏/湿/虫/沙/变质）"""

    STATUS_ICON = {
        BagStatus.PRISTINE: "PRISTINE",
        BagStatus.USABLE: "USABLE",
        BagStatus.NEEDS_CLEAN: "NEEDS_CLEAN",
        BagStatus.NEEDS_DRY: "NEEDS_DRY",
        BagStatus.INFESTED: "INFESTED",
        BagStatus.CONTAMINATED: "CONTAMINATED",
        BagStatus.SPOILED: "SPOILED",
        BagStatus.REJECT: "REJECT",
    }

    @classmethod
    def see(cls, bag: Bag) -> Dict:
        """感知袋子状态"""
        return {
            "owner": bag.owner,
            "status": bag.status.value,
            "clean": bag.clean,
            "dry": bag.dry,
            "has_worm": bag.has_worm,
            "has_sand": bag.has_sand,
            "edible": bag.edible,
            "washed": bag.washed,
            "value": bag.value,
            "content": bag.content,
        }

    @classmethod
    def render(cls, perception: Dict) -> str:
        """视觉呈现"""
        lines = [
            f"袋子：{perception['owner']}家",
            f"  状态：{perception['status']}",
            f"  价值：{perception['value']} 元",
            f"  内容物：{perception['content']}",
            f"  脏不脏：{'✗ 脏' if not perception['clean'] else '✓ 干净'}",
            f"  晒干没：{'✗ 未干' if not perception['dry'] else '✓ 已干'}",
            f"  有虫吗：{'✗ 有虫' if perception['has_worm'] else '✓ 无虫'}",
            f"  有沙吗：{'✗ 含沙' if perception['has_sand'] else '✓ 无沙'}",
            f"  能吃吗：{'✗ 不能吃' if not perception['edible'] else '✓ 能吃'}",
            f"  洗了吗：{'✗ 未洗' if not perception['washed'] else '✓ 已洗'}",
        ]
        return "\n".join(lines)


# ============================================================
# 听觉层 —— 授权
# ============================================================

class Hearing:
    """
    听觉 —— 听袋子有没有问题
    问题：脏、湿、虫、沙、变质、没洗
    授权通过 = 没有任何问题
    """

    @staticmethod
    def listen(bag: Bag) -> Dict:
        """听袋子的问题"""
        issues = []
        if not bag.clean:
            issues.append("脏")
        if not bag.dry:
            issues.append("没晒干")
        if bag.has_worm:
            issues.append("生虫")
        if bag.has_sand:
            issues.append("含沙")
        if not bag.edible:
            issues.append("变质不能吃")
        if not bag.washed:
            issues.append("没洗")
        return {
            "owner": bag.owner,
            "issues": issues,
            "issue_count": len(issues),
            "clear": len(issues) == 0,
        }

    @staticmethod
    def authorize(hearing: Dict) -> tuple:
        """授权 —— 有问题就拒绝"""
        if hearing["clear"]:
            return True, "听觉授权通过：无任何问题"
        issues_str = "、".join(hearing["issues"])
        return False, f"听觉拒绝授权：发现 {hearing['issue_count']} 个问题 [{issues_str}]"


# ============================================================
# 审判层 —— 奖惩
# ============================================================

class Judgment:
    """
    审判 —— 判定袋子是否合格
    对了奖励，错了惩罚
    """

    def judge(self, vision: Dict, hearing: Dict) -> Dict:
        """审判"""
        score = 0.0
        reasons_good = []
        reasons_bad = []

        # === 视觉维度 ===
        if vision["clean"]:
            score += 2
            reasons_good.append("干净")
        else:
            score -= 1
            reasons_bad.append("脏")

        if vision["dry"]:
            score += 2
            reasons_good.append("晒干")
        else:
            score -= 1
            reasons_bad.append("没晒干")

        if not vision["has_worm"]:
            score += 3
            reasons_good.append("无虫")
        else:
            score -= 5
            reasons_bad.append("生虫")

        if not vision["has_sand"]:
            score += 2
            reasons_good.append("无沙")
        else:
            score -= 3
            reasons_bad.append("含沙")

        if vision["edible"]:
            score += 3
            reasons_good.append("能吃")
        else:
            score -= 10
            reasons_bad.append("变质不能吃")

        if vision["washed"]:
            score += 1
            reasons_good.append("已洗")

        # === 听觉维度 ===
        issue_count = hearing["issue_count"]
        score -= issue_count * 0.5

        # === 判定 ===
        if score >= 5:
            verdict = "通过"
            reward = score * 0.5
            penalty = 0.0
        elif score >= 0:
            verdict = "有条件通过"
            reward = score * 0.2
            penalty = 0.0
        else:
            verdict = "不通过"
            reward = 0.0
            penalty = abs(score) * 0.5

        return {
            "owner": vision["owner"],
            "verdict": verdict,
            "score": score,
            "reward": reward,
            "penalty": penalty,
            "reasons_good": reasons_good,
            "reasons_bad": reasons_bad,
            "timestamp": time.time(),
        }


# ============================================================
# 记忆层 —— 历史积累 + 版本管理
# ============================================================

@dataclass
class VersionRecord:
    """版本记录"""
    version_name: str           # 老大、老二、老三...
    version_number: int
    action: str                 # 做了什么
    judgment: Dict              # 审判结果
    perception: Dict            # 视觉感知
    hearing: Dict               # 听觉感知
    timestamp: float
    is_valid: bool = True       # 是否有效
    is_selected: bool = False   # 是否被用户选中


class Memory:
    """
    历史的记忆力 —— 积累所有版本
    每个版本独立保存，不修改旧版本
    """

    def __init__(self, storage_dir: str):
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.versions: List[VersionRecord] = []
        self.counter = VersionCounter()
        self._load()

    def _load(self):
        """加载历史"""
        for subdir in sorted(self.storage_dir.iterdir()):
            if subdir.is_dir() and subdir.name.startswith("v_"):
                record_file = subdir / "record.json"
                if record_file.exists():
                    with open(record_file, "r", encoding="utf-8") as f:
                        data = json.load(f)
                    self.versions.append(VersionRecord(**data))
                    self.counter.count = max(self.counter.count, data["version_number"])
        self.versions.sort(key=lambda v: v.version_number)

    def record(self, action: str, judgment: Dict, perception: Dict, hearing: Dict) -> VersionRecord:
        """记录新版本 —— 每次都是新版本，不修改旧的"""
        version_name = self.counter.next()
        version_number = self.counter.count

        record = VersionRecord(
            version_name=version_name,
            version_number=version_number,
            action=action,
            judgment=judgment,
            perception=perception,
            hearing=hearing,
            timestamp=time.time(),
        )

        # 创建版本目录
        version_dir = self.storage_dir / f"v_{version_number:04d}_{version_name}"
        version_dir.mkdir(exist_ok=True)

        # 保存记录
        record_file = version_dir / "record.json"
        with open(record_file, "w", encoding="utf-8") as f:
            json.dump(asdict(record), f, ensure_ascii=False, indent=2, default=str)

        # 保存视觉呈现
        vision_file = version_dir / "vision.txt"
        with open(vision_file, "w", encoding="utf-8") as f:
            f.write(Vision.render(perception))
            f.write(f"\n\n审判结果：{judgment['verdict']}\n")
            f.write(f"分数：{judgment['score']:+.1f}\n")
            f.write(f"奖励：+{judgment['reward']:.1f}\n")
            f.write(f"惩罚：-{judgment['penalty']:.1f}\n")
            if judgment['reasons_good']:
                f.write(f"优点：{'、'.join(judgment['reasons_good'])}\n")
            if judgment['reasons_bad']:
                f.write(f"缺点：{'、'.join(judgment['reasons_bad'])}\n")

        self.versions.append(record)
        return record

    def get_version(self, version_number: int) -> Optional[VersionRecord]:
        for v in self.versions:
            if v.version_number == version_number:
                return v
        return None

    def list_versions(self) -> List[Dict]:
        """列出所有版本"""
        result = []
        for v in self.versions:
            result.append({
                "version": v.version_name,
                "number": v.version_number,
                "verdict": v.judgment["verdict"],
                "score": v.judgment["score"],
                "reward": v.judgment["reward"],
                "penalty": v.judgment["penalty"],
                "action": v.action,
            })
        return result

    def compare_versions(self, a_num: int, b_num: int) -> str:
        """对比两个版本"""
        a = self.get_version(a_num)
        b = self.get_version(b_num)
        if not a or not b:
            return "版本不存在"

        lines = [
            f"版本对比：{a.version_name} vs {b.version_name}",
            f"{'─'*50}",
            f"  {'':<15} {a.version_name:<10} {b.version_name:<10}",
            f"  审判结果：{a.judgment['verdict']:<10} {b.judgment['verdict']:<10}",
            f"  分数：{a.judgment['score']:>+6.1f}{'':<6} {b.judgment['score']:>+6.1f}",
            f"  奖励：{a.judgment['reward']:>+6.1f}{'':<6} {b.judgment['reward']:>+6.1f}",
            f"  惩罚：{a.judgment['penalty']:>+6.1f}{'':<6} {b.judgment['penalty']:>+6.1f}",
        ]
        if a.judgment['reasons_bad']:
            lines.append(f"  缺点(a)：{'、'.join(a.judgment['reasons_bad'])}")
        if b.judgment['reasons_bad']:
            lines.append(f"  缺点(b)：{'、'.join(b.judgment['reasons_bad'])}")
        if a.judgment['reasons_good']:
            lines.append(f"  优点(a)：{'、'.join(a.judgment['reasons_good'])}")
        if b.judgment['reasons_good']:
            lines.append(f"  优点(b)：{'、'.join(b.judgment['reasons_good'])}")
        return "\n".join(lines)

    def select_version(self, version_number: int) -> str:
        """挑选版本"""
        v = self.get_version(version_number)
        if not v:
            return "版本不存在"
        for vv in self.versions:
            vv.is_selected = False
        v.is_selected = True
        return f"已选中版本：{v.version_name}（第 {v.version_number} 号）"

    def summary(self) -> str:
        """总结所有版本"""
        lines = [
            f"{'='*60}",
            f"  【记忆总结】共 {len(self.versions)} 个版本",
            f"{'='*60}",
            f"  {'版本':<8} {'结果':<10} {'分数':>6} {'奖':>6} {'惩':>6} {'动作':<15}",
            f"  {'─'*55}",
        ]
        for v in self.versions:
            sel = " ★" if v.is_selected else ""
            lines.append(
                f"  {v.version_name:<6}{sel} {v.judgment['verdict']:<10} "
                f"{v.judgment['score']:>+5.1f} "
                f"{v.judgment['reward']:>+5.1f} "
                f"{v.judgment['penalty']:>+5.1f} "
                f"{v.action:<15}"
            )
        return "\n".join(lines)


# ============================================================
# 工作流引擎
# ============================================================

class Workflow:
    """
    版本化工作流
    每次执行：视觉 → 听觉 → 审判 → 记忆（新版本）
    """

    def __init__(self, storage_dir: str):
        self.memory = Memory(storage_dir)
        self.vision = Vision()
        self.hearing = Hearing()
        self.judgment = Judgment()

    def process(self, bag: Bag, action: str = "检验") -> VersionRecord:
        """处理一个袋子 —— 输出一个新版本"""
        # 视觉
        p = self.vision.see(bag)
        # 听觉
        h = self.hearing.listen(bag)
        # 审判
        j = self.judgment.judge(p, h)
        # 记忆（新版本）
        record = self.memory.record(action, j, p, h)
        return record

    def process_with_fix(self, bag: Bag) -> tuple:
        """
        处理 + 如果审判不通过，自动修正并出新版本
        返回 (原版本, 修正版本或None)
        """
        original = self.process(bag, "原版")
        if original.judgment["verdict"] != "通过":
            # 修正袋子
            fixed_bag = Bag(
                owner=bag.owner,
                value=bag.value,
                content=bag.content,
                clean=True,
                dry=True,
                has_worm=False,
                has_sand=False,
                edible=bag.edible,  # 变质无法修正
                washed=True,
            )
            fix = self.process(fixed_bag, "修正版")
            return original, fix
        return original, None


# ============================================================
# 主程序
# ============================================================

def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions"
    wf = Workflow(storage)

    # 创建袋子
    bags = [
        Bag(owner="张", value=100, content="东北大米",
            clean=True, dry=True, has_worm=False, has_sand=False, edible=True),
        Bag(owner="李", value=80, content="江南糯米",
            clean=False, dry=True, has_worm=False, has_sand=True, edible=True),
        Bag(owner="王", value=120, content="泰国香米",
            clean=True, dry=False, has_worm=True, has_sand=False, edible=True),
        Bag(owner="赵", value=60, content="陈年旧米",
            clean=False, dry=False, has_worm=True, has_sand=True, edible=False),
    ]

    print("=" * 60)
    print("  版本化层级工作流系统")
    print("  层级：袋子（高维） → 不数米（低维）")
    print("=" * 60)

    # 逐个袋子处理
    for bag in bags:
        original, fix = wf.process_with_fix(bag)
        print(f"\n{'─'*50}")
        print(f"  袋子：{bag.owner}家 - 原版 [{original.version_name}]")
        print(f"  审判：{original.judgment['verdict']} | 分数：{original.judgment['score']:+.1f}")
        print(f"  奖/惩：+{original.judgment['reward']:.1f} / -{original.judgment['penalty']:.1f}")
        if original.judgment['reasons_bad']:
            print(f"  缺点：{'、'.join(original.judgment['reasons_bad'])}")

        if fix:
            print(f"  ↳ 修正版 [{fix.version_name}]")
            print(f"    审判：{fix.judgment['verdict']} | 分数：{fix.judgment['score']:+.1f}")
            print(f"    奖/惩：+{fix.judgment['reward']:.1f} / -{fix.judgment['penalty']:.1f}")

    # 列出所有版本
    print(wf.memory.summary())

    # 对比版本
    if len(wf.memory.versions) >= 2:
        print(f"\n{'='*60}")
        print("  版本对比示例")
        print(f"{'='*60}")
        print(wf.memory.compare_versions(1, 2))

    # 选中版本
    best_version = max(wf.memory.versions, key=lambda v: v.judgment["score"])
    print(f"\n{wf.memory.select_version(best_version.version_number)}")

    print(wf.memory.summary())

    print(f"\n  所有版本存储在：{storage}")
    print(f"  每个版本一个目录，不修改旧版本")


if __name__ == "__main__":
    main()
