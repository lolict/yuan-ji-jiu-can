#!/usr/bin/env python3
"""
公约数-镜子 层级治理系统

数学框架：
  公约数 = 锚定标准（袋子应该是干净的、晒干的...）
  公倍数 = 绕锚旋转的实例（张家的袋子、李家的袋子...）
  旋转 = 每次生成新实例
  照镜子 = 用公约数检验公倍数
  对不对 = 公倍数映出公约数的样子

工作流：
  公约数锚定 → 公倍数生成(旋转) → 照镜审判(奖惩) → 版本积累
"""

import os
import json
import time
from dataclasses import dataclass, asdict
from typing import List, Dict, Optional, Tuple
from pathlib import Path


# ============================================================
# 公约数 —— 锚定标准
# ============================================================

@dataclass
class CommonDivisor:
    """
    公约数 = 袋子应该是的样子
    这是所有袋子实例绕着旋转的锚点
    """
    # 袋子级别的规则（不数米，只看袋子）
    must_be_clean: bool = True        # 必须干净
    must_be_dry: bool = True          # 必须晒干
    must_have_no_worm: bool = True    # 必须无虫
    must_have_no_sand: bool = True    # 必须无沙
    must_be_edible: bool = True       # 必须能吃
    should_be_washed: bool = False    # 最好洗过

    def mirror_check(self, bag_state: Dict) -> Dict:
        """
        照镜子 —— 用公约数检验袋子
        看公倍数是否映出公约数的样子
        """
        reflections = []

        if self.must_be_clean:
            reflections.append(("干净", bag_state.get("clean", False), True))

        if self.must_be_dry:
            reflections.append(("晒干", bag_state.get("dry", False), True))

        if self.must_have_no_worm:
            reflections.append(("无虫", not bag_state.get("has_worm", True), True))

        if self.must_have_no_sand:
            reflections.append(("无沙", not bag_state.get("has_sand", True), True))

        if self.must_be_edible:
            reflections.append(("能吃", bag_state.get("edible", False), True))

        if self.should_be_washed:
            reflections.append(("已洗", bag_state.get("washed", False), False))

        return {
            "reflections": reflections,
            "passed": sum(1 for _, actual, expected in reflections if actual == expected),
            "total": len(reflections),
            "score": sum(2.0 if actual == expected else -3.0 for _, actual, expected in reflections),
        }


# ============================================================
# 公倍数 —— 绕锚旋转的实例
# ============================================================

@dataclass
class CommonMultiple:
    """
    公倍数 = 具体的袋子实例
    绕着公约数旋转出来的实际对象
    """
    owner: str
    value: float
    content: str
    clean: bool = True
    dry: bool = True
    has_worm: bool = False
    has_sand: bool = False
    edible: bool = True
    washed: bool = False

    def to_state(self) -> Dict:
        return {
            "owner": self.owner,
            "value": self.value,
            "content": self.content,
            "clean": self.clean,
            "dry": self.dry,
            "has_worm": self.has_worm,
            "has_sand": self.has_sand,
            "edible": self.edible,
            "washed": self.washed,
        }


# ============================================================
# 旋转器 —— 围绕公约数生成公倍数
# ============================================================

class Rotator:
    """
    旋转器 —— 围绕公约数生成不同的公倍数
    每次旋转产生一个新实例（老大、老二、老三...）
    """

    NAMES = ["老大", "老二", "老三", "老四", "老五", "老六", "老七", "老八", "老九", "老十"]

    def __init__(self):
        self.count = 0

    def next_name(self) -> str:
        self.count += 1
        if self.count <= len(self.NAMES):
            return self.NAMES[self.count - 1]
        return f"老{self.count}"

    def generate(self, owner: str, base_value: float, content: str,
                 clean: bool = True, dry: bool = True,
                 has_worm: bool = False, has_sand: bool = False,
                 edible: bool = True, washed: bool = False) -> CommonMultiple:
        """生成一个公倍数实例"""
        return CommonMultiple(
            owner=owner,
            value=base_value,
            content=content,
            clean=clean,
            dry=dry,
            has_worm=has_worm,
            has_sand=has_sand,
            edible=edible,
            washed=washed,
        )


# ============================================================
# 视觉层 —— 看公倍数
# ============================================================

class Vision:
    """视觉 —— 看到公倍数的状态"""

    STATUS_MAP = {
        "clean": ("脏不脏", True, "✓ 干净", "✗ 脏"),
        "dry": ("晒干没", True, "✓ 已干", "✗ 未干"),
        "has_worm": ("有虫吗", False, "✓ 无虫", "✗ 有虫"),
        "has_sand": ("有沙吗", False, "✓ 无沙", "✗ 含沙"),
        "edible": ("能吃吗", True, "✓ 能吃", "✗ 不能吃"),
        "washed": ("洗了吗", True, "✓ 已洗", "✗ 未洗"),
    }

    @classmethod
    def see(cls, multiple: CommonMultiple) -> Dict:
        return multiple.to_state()

    @classmethod
    def render(cls, state: Dict) -> str:
        lines = [
            f"袋子：{state['owner']}家",
            f"  价值：{state['value']} 元",
            f"  内容物：{state['content']}",
        ]
        for key, (label, good_val, good_str, bad_str) in cls.STATUS_MAP.items():
            actual = state.get(key, not good_val)
            icon = good_str if actual == good_val else bad_str
            lines.append(f"  {label}：{icon}")
        return "\n".join(lines)


# ============================================================
# 听觉层 —— 听授权
# ============================================================

class Hearing:
    """听觉 —— 听公倍数有没有问题"""

    @staticmethod
    def listen(multiple: CommonMultiple) -> Dict:
        state = multiple.to_state()
        issues = []

        if not state.get("clean", True):
            issues.append("脏")
        if not state.get("dry", True):
            issues.append("没晒干")
        if state.get("has_worm", False):
            issues.append("生虫")
        if state.get("has_sand", False):
            issues.append("含沙")
        if not state.get("edible", True):
            issues.append("变质不能吃")
        if not state.get("washed", False):
            issues.append("没洗")

        return {
            "owner": state["owner"],
            "issues": issues,
            "clear": len(issues) == 0,
        }

    @staticmethod
    def authorize(hearing: Dict) -> Tuple[bool, str]:
        if hearing["clear"]:
            return True, "听觉授权通过"
        return False, f"听觉拒绝：发现 {len(hearing['issues'])} 个问题 [{'、'.join(hearing['issues'])}]"


# ============================================================
# 审判层 —— 照镜子奖惩
# ============================================================

class Judgment:
    """
    审判 —— 用公约数照公倍数
    映出公约数样子 → 奖励
    映不出 → 惩罚
    """

    def judge(self, multiple: CommonMultiple, divisor: CommonDivisor) -> Dict:
        """用公约数检验公倍数"""
        state = multiple.to_state()
        reflection = divisor.mirror_check(state)

        passed = reflection["passed"]
        total = reflection["total"]
        score = reflection["score"]

        if passed == total:
            verdict = "通过"
            reward = score * 0.5
            penalty = 0.0
        elif passed >= total * 0.6:
            verdict = "有条件通过"
            reward = max(score * 0.2, 0)
            penalty = 0.0
        else:
            verdict = "不通过"
            reward = 0.0
            penalty = abs(score) * 0.5

        details = []
        for label, actual, expected in reflection["reflections"]:
            if actual == expected:
                details.append(f"  ✓ {label}：映出公约数")
            else:
                details.append(f"  ✗ {label}：未映出公约数")

        return {
            "owner": state["owner"],
            "verdict": verdict,
            "score": score,
            "passed": passed,
            "total": total,
            "reward": reward,
            "penalty": penalty,
            "details": details,
            "timestamp": time.time(),
        }


# ============================================================
# 记忆层 —— 版本积累
# ============================================================

@dataclass
class VersionRecord:
    version_name: str
    version_number: int
    action: str
    judgment: Dict
    perception: Dict
    mirror_details: List[str]
    timestamp: float
    is_selected: bool = False


class Memory:
    """历史的记忆力 —— 所有版本独立保存，不改旧的"""

    def __init__(self, storage_dir: str):
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.versions: List[VersionRecord] = []
        self.rotator = Rotator()
        self._load()

    def _load(self):
        for subdir in sorted(self.storage_dir.iterdir()):
            if subdir.is_dir() and subdir.name.startswith("v_"):
                rf = subdir / "record.json"
                if rf.exists():
                    with open(rf, "r", encoding="utf-8") as f:
                        data = json.load(f)
                    self.versions.append(VersionRecord(**data))
                    self.rotator.count = max(self.rotator.count, data["version_number"])
        self.versions.sort(key=lambda v: v.version_number)

    def record(self, action: str, judgment: Dict, perception: Dict, mirror_details: List[str]) -> VersionRecord:
        name = self.rotator.next_name()
        num = self.rotator.count

        record = VersionRecord(
            version_name=name,
            version_number=num,
            action=action,
            judgment=judgment,
            perception=perception,
            mirror_details=mirror_details,
            timestamp=time.time(),
        )

        vdir = self.storage_dir / f"v_{num:04d}_{name}"
        vdir.mkdir(exist_ok=True)

        with open(vdir / "record.json", "w", encoding="utf-8") as f:
            json.dump(asdict(record), f, ensure_ascii=False, indent=2, default=str)

        self.versions.append(record)
        return record

    def compare(self, a_num: int, b_num: int) -> str:
        a = next((v for v in self.versions if v.version_number == a_num), None)
        b = next((v for v in self.versions if v.version_number == b_num), None)
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
        return "\n".join(lines)

    def select(self, num: int) -> str:
        v = next((v for v in self.versions if v.version_number == num), None)
        if not v:
            return "版本不存在"
        for vv in self.versions:
            vv.is_selected = False
        v.is_selected = True
        return f"已选中：{v.version_name}（第 {num} 号）"

    def summary(self) -> str:
        lines = [
            f"{'='*60}",
            f"  记忆总结：共 {len(self.versions)} 个版本",
            f"{'='*60}",
            f"  {'版本':<6} {'结果':<12} {'分数':>6} {'奖':>6} {'惩':>6}",
            f"  {'─'*45}",
        ]
        for v in self.versions:
            sel = " ★" if v.is_selected else ""
            lines.append(
                f"  {v.version_name:<4}{sel} {v.judgment['verdict']:<12} "
                f"{v.judgment['score']:>+5.1f} "
                f"{v.judgment['reward']:>+5.1f} "
                f"{v.judgment['penalty']:>+5.1f}"
            )
        return "\n".join(lines)


# ============================================================
# 主系统 —— 公约数-镜子-旋转
# ============================================================

class DivisorMirrorSystem:
    """
    公约数-镜子 层级治理系统

    公约数锚定 → 公倍数旋转 → 照镜审判 → 版本积累
    """

    def __init__(self, storage_dir: str):
        # 公约数 —— 锚定标准
        self.divisor = CommonDivisor()
        # 旋转器 —— 生成公倍数
        self.rotator = Rotator()
        # 视觉/听觉/审判/记忆
        self.vision = Vision()
        self.hearing = Hearing()
        self.judgment = Judgment()
        self.memory = Memory(storage_dir)

    def process(self, multiple: CommonMultiple, action: str = "检验") -> VersionRecord:
        """处理一个公倍数 —— 出新版本"""
        # 视觉
        p = self.vision.see(multiple)
        # 听觉
        h = self.hearing.listen(multiple)
        authorized, auth_msg = self.hearing.authorize(h)
        if not authorized:
            j = {
                "owner": multiple.owner,
                "verdict": "不通过",
                "score": -5.0,
                "passed": 0,
                "total": 5,
                "reward": 0.0,
                "penalty": 2.5,
                "details": [auth_msg],
            }
            return self.memory.record(action, j, p, [auth_msg])
        # 照镜审判
        j = self.judgment.judge(multiple, self.divisor)
        return self.memory.record(action, j, p, j.get("details", []))

    def process_with_fix(self, multiple: CommonMultiple) -> Tuple[VersionRecord, Optional[VersionRecord]]:
        """处理 + 不通过则修正出新版"""
        original = self.process(multiple, "原版")
        if original.judgment["verdict"] != "通过":
            fixed = CommonMultiple(
                owner=multiple.owner,
                value=multiple.value,
                content=multiple.content,
                clean=True, dry=True,
                has_worm=False, has_sand=False,
                edible=multiple.edible,
                washed=True,
            )
            fix = self.process(fixed, "修正版")
            return original, fix
        return original, None


# ============================================================
# 主程序
# ============================================================

def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions_mirror"
    system = DivisorMirrorSystem(storage)

    print("=" * 60)
    print("  公约数-镜子 层级治理系统")
    print("  ─────────────────────────────")
    print("  公约数 = 锚定标准（袋子应该是干净的、晒干的、无虫...）")
    print("  公倍数 = 绕锚旋转的实例（各家的袋子）")
    print("  照镜子 = 用公约数检验公倍数")
    print("  层级：袋子（高维） → 不数米（低维）")
    print("=" * 60)

    # 用旋转器生成公倍数
    multiples = [
        system.rotator.generate("张", 100, "东北大米",
                                clean=True, dry=True, has_worm=False, has_sand=False, edible=True),
        system.rotator.generate("李", 80, "江南糯米",
                                clean=False, dry=True, has_worm=False, has_sand=True, edible=True),
        system.rotator.generate("王", 120, "泰国香米",
                                clean=True, dry=False, has_worm=True, has_sand=False, edible=True),
        system.rotator.generate("赵", 60, "陈年旧米",
                                clean=False, dry=False, has_worm=True, has_sand=True, edible=False),
        system.rotator.generate("刘", 150, "有机新米",
                                clean=True, dry=True, has_worm=False, has_sand=False, edible=True, washed=True),
    ]

    for m in multiples:
        original, fix = system.process_with_fix(m)
        print(f"\n{'─'*50}")
        print(f"  {m.owner}家 [{original.version_name}]")
        print(f"  视觉：{system.vision.render(system.vision.see(m))}")
        print(f"  照镜：{original.judgment['verdict']} | 分数：{original.judgment['score']:+.1f}")
        for d in original.mirror_details:
            print(f"  {d}")
        if fix:
            print(f"  ↳ 修正 [{fix.version_name}]：{fix.judgment['verdict']} | 分数：{fix.judgment['score']:+.1f}")

    print(system.memory.summary())

    print(f"\n  所有版本存储在：{storage}")
    print(f"  公约数不动，每次旋转出新版本")


if __name__ == "__main__":
    main()
