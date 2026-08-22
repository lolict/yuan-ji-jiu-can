#!/usr/bin/env python3
"""
满全法 · 层级治理系统

初始目的：用这堆符号写自己的系统
- 自己的操作系统
- 自己的语言系统
- 自己的听觉、视觉系统
- 自己的行为、判断、思考系统
- 自己的奖励、处罚系统
- 自己的审美、淘汰、过滤系统

框架：
  公约数 = 锚定标准（不动）
  公倍数 = 绕锚旋转的实例（每次出新版本）
  照镜子 = 用公约数检验公倍数

层级（高维）：
  袋子级别操作，不数米的颗粒，不为每颗米命名
"""

import os
import json
import time
from dataclasses import dataclass, asdict
from typing import List, Dict, Optional, Tuple
from pathlib import Path


# ============================================================
# 版本计数器 —— 老大、老二、老三...
# ============================================================

class VersionCounter:
    NAMES = ["老大","老二","老三","老四","老五","老六","老七","老八","老九","老十",
             "老十一","老十二","老十三","老十四","老十五"]

    def __init__(self):
        self.count = 0

    def next(self) -> str:
        self.count += 1
        return self.NAMES[self.count-1] if self.count <= len(self.NAMES) else f"老{self.count}"


# ============================================================
# 公约数 —— 锚定标准（永远不动）
# ============================================================

@dataclass
class CommonDivisor:
    """
    公约数 = 袋子应该是的样子
    所有公倍数绕此锚点旋转
    """
    # 袋子级别的标准（不数米）
    standard: str = "袋子应是干净的、晒干的、无虫的、无沙的、能吃的"

    def mirror(self, bag: Dict) -> Dict:
        """
        照镜子 —— 用公约数检验公倍数
        看公倍数是否映出公约数的样子
        """
        reflections = []

        checks = [
            ("干净", "clean", True),
            ("晒干", "dry", True),
            ("无虫", "has_worm", False),
            ("无沙", "has_sand", False),
            ("能吃", "edible", True),
            ("已洗", "washed", True),
        ]

        for label, key, expected in checks:
            actual = bag.get(key, not expected)
            reflections.append({
                "label": label,
                "expected": expected,
                "actual": actual,
                "matched": actual == expected,
            })

        passed = sum(1 for r in reflections if r["matched"])
        total = len(reflections)
        score = sum(2.0 if r["matched"] else -3.0 for r in reflections)

        return {
            "reflections": reflections,
            "passed": passed,
            "total": total,
            "score": score,
            "mirror_rate": passed / total,
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

    def to_dict(self) -> Dict:
        return asdict(self)


# ============================================================
# 旋转器 —— 围绕公约数生成公倍数
# ============================================================

class Rotator:
    def __init__(self):
        self.counter = VersionCounter()

    def generate(self, owner: str, value: float, content: str,
                 clean: bool = True, dry: bool = True,
                 has_worm: bool = False, has_sand: bool = False,
                 edible: bool = True, washed: bool = False) -> CommonMultiple:
        """生成公倍数实例"""
        return CommonMultiple(
            owner=owner, value=value, content=content,
            clean=clean, dry=dry, has_worm=has_worm,
            has_sand=has_sand, edible=edible, washed=washed,
        )


# ============================================================
# 视觉 —— 看公倍数状态
# ============================================================

class Vision:
    """视觉 —— 看到袋子的状态（脏/湿/虫/沙/变质/洗）"""

    STATUS = {
        "clean": ("脏不脏", True, "✓ 干净", "✗ 脏"),
        "dry": ("晒干没", True, "✓ 已干", "✗ 未干"),
        "has_worm": ("有虫吗", False, "✓ 无虫", "✗ 有虫"),
        "has_sand": ("有沙吗", False, "✓ 无沙", "✗ 含沙"),
        "edible": ("能吃吗", True, "✓ 能吃", "✗ 不能吃"),
        "washed": ("洗了吗", True, "✓ 已洗", "✗ 未洗"),
    }

    @classmethod
    def see(cls, m: CommonMultiple) -> Dict:
        return m.to_dict()

    @classmethod
    def render(cls, state: Dict) -> str:
        lines = [f"袋子：{state['owner']}家（{state['value']}元）"]
        for key, (label, good, good_str, bad_str) in cls.STATUS.items():
            actual = state.get(key, not good)
            icon = good_str if actual == good else bad_str
            lines.append(f"  {label}：{icon}")
        return "\n".join(lines)


# ============================================================
# 听觉 —— 听授权
# ============================================================

class Hearing:
    """听觉 —— 听袋子有没有问题，授权准入"""

    @staticmethod
    def listen(m: CommonMultiple) -> Dict:
        state = m.to_dict()
        issues = []
        for key, label, bad_value in [
            ("clean", "脏", False),
            ("dry", "没晒干", False),
            ("has_worm", "生虫", True),
            ("has_sand", "含沙", True),
            ("edible", "变质", False),
            ("washed", "没洗", False),
        ]:
            if state.get(key) == bad_value:
                issues.append(label)
        return {"owner": m.owner, "issues": issues, "clear": len(issues) == 0}

    @staticmethod
    def authorize(hearing: Dict) -> Tuple[bool, str]:
        if hearing["clear"]:
            return True, "听觉授权通过"
        return False, f"听觉拒绝：{'、'.join(hearing['issues'])}"


# ============================================================
# 审判 —— 照镜子 + 奖惩
# ============================================================

class Judgment:
    """
    审判 —— 用公约数照公倍数
    映出公约数 → 奖励
    映不出 → 惩罚
    """

    def judge(self, m: CommonMultiple, divisor: CommonDivisor) -> Dict:
        reflection = divisor.mirror(m.to_dict())
        score = reflection["score"]

        if reflection["passed"] == reflection["total"]:
            verdict = "通过"
            reward = score * 0.5
            penalty = 0.0
        elif reflection["passed"] >= reflection["total"] * 0.6:
            verdict = "有条件通过"
            reward = max(score * 0.2, 0)
            penalty = 0.0
        else:
            verdict = "不通过"
            reward = 0.0
            penalty = abs(score) * 0.5

        return {
            "owner": m.owner,
            "verdict": verdict,
            "score": score,
            "reward": reward,
            "penalty": penalty,
            "reflection": reflection,
            "timestamp": time.time(),
        }


# ============================================================
# 审美 —— 评估好坏
# ============================================================

class Aesthetics:
    """
    审美 —— 评估袋子的整体品质
    不只是"能不能吃"，还包括价值、外观、状态的综合评分
    """

    @staticmethod
    def evaluate(m: CommonMultiple) -> Dict:
        score = 0.0
        aspects = []

        # 清洁度
        if m.clean:
            score += 2
            aspects.append(("清洁", 2))
        else:
            score -= 1
            aspects.append(("污浊", -1))

        # 干燥度
        if m.dry:
            score += 2
            aspects.append(("干爽", 2))
        else:
            score -= 1
            aspects.append(("潮湿", -1))

        # 纯净度（无虫无砂）
        if not m.has_worm and not m.has_sand:
            score += 3
            aspects.append(("纯净", 3))
        elif not m.has_worm:
            score += 1
            aspects.append(("微瑕", 1))
        else:
            score -= 2
            aspects.append(("虫害", -2))

        # 实用度
        if m.edible:
            score += 3
            aspects.append(("可用", 3))
        else:
            score -= 5
            aspects.append(("废弃", -5))

        # 价值
        if m.value >= 100:
            score += 1
            aspects.append(("贵重", 1))
        elif m.value >= 50:
            score += 0
            aspects.append(("平价", 0))
        else:
            score -= 1
            aspects.append(("廉价", -1))

        # 处理
        if m.washed:
            score += 1
            aspects.append(("精细", 1))

        if score >= 8:
            grade = "上品"
        elif score >= 5:
            grade = "中品"
        elif score >= 2:
            grade = "下品"
        else:
            grade = "废品"

        return {
            "owner": m.owner,
            "score": score,
            "grade": grade,
            "aspects": aspects,
        }


# ============================================================
# 淘汰 —— 去掉不合格
# ============================================================

class Elimination:
    """
    淘汰 —— 去掉映不出公约数的公倍数
    标准：照镜通过 + 审美中品以上
    """

    @staticmethod
    def eliminate(judgment: Dict, aesthetics: Dict) -> Tuple[bool, str]:
        if judgment["verdict"] != "通过":
            return False, f"淘汰：审判{judgment['verdict']}"
        if aesthetics["grade"] in ("下品", "废品"):
            return False, f"淘汰：审美{aesthetics['grade']}"
        return True, "保留"


# ============================================================
# 过滤 —— 筛选合格
# ============================================================

class Filter:
    """
    过滤 —— 从公倍数中筛出可用的
    标准：听觉授权 + 审判通过 + 审美合格
    """

    @staticmethod
    def filter_pass(hearing: Dict, judgment: Dict, aesthetics: Dict) -> bool:
        return (
            hearing["clear"]
            and judgment["verdict"] == "通过"
            and aesthetics["grade"] in ("上品", "中品")
        )


# ============================================================
# 记忆 —— 版本积累
# ============================================================

@dataclass
class VersionRecord:
    version_name: str
    version_number: int
    action: str
    judgment: Dict
    aesthetics: Dict
    passed_filter: bool
    timestamp: float
    is_selected: bool = False


class Memory:
    """历史的记忆力 —— 所有版本独立保存，不改旧的"""

    def __init__(self, storage_dir: str):
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.versions: List[VersionRecord] = []
        self.counter = VersionCounter()
        self._load()

    def _load(self):
        for subdir in sorted(self.storage_dir.iterdir()):
            if subdir.is_dir() and subdir.name.startswith("v_"):
                rf = subdir / "record.json"
                if rf.exists():
                    with open(rf, "r", encoding="utf-8") as f:
                        data = json.load(f)
                    self.versions.append(VersionRecord(**data))
                    self.counter.count = max(self.counter.count, data["version_number"])
        self.versions.sort(key=lambda v: v.version_number)

    def record(self, action: str, judgment: Dict, aesthetics: Dict, passed_filter: bool) -> VersionRecord:
        name = self.counter.next()
        num = self.counter.count

        record = VersionRecord(
            version_name=name,
            version_number=num,
            action=action,
            judgment=judgment,
            aesthetics=aesthetics,
            passed_filter=passed_filter,
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
            f"对比：{a.version_name} vs {b.version_name}",
            f"  {'':<12} {a.version_name:<8} {b.version_name:<8}",
            f"  审判：{a.judgment['verdict']:<10} {b.judgment['verdict']:<10}",
            f"  分数：{a.judgment['score']:>+6.1f}{'':<4} {b.judgment['score']:>+6.1f}",
            f"  审美：{a.aesthetics['grade']:<10} {b.aesthetics['grade']:<10}",
            f"  过滤：{'通过' if a.passed_filter else '淘汰':<10} {'通过' if b.passed_filter else '淘汰':<10}",
        ]
        return "\n".join(lines)

    def select(self, num: int) -> str:
        v = next((v for v in self.versions if v.version_number == num), None)
        if not v:
            return "版本不存在"
        for vv in self.versions:
            vv.is_selected = False
        v.is_selected = True
        return f"已选中：{v.version_name}（第{num}号）"

    def summary(self) -> str:
        lines = [
            f"{'='*60}",
            f"  记忆总结：共 {len(self.versions)} 个版本",
            f"{'='*60}",
            f"  {'版本':<5} {'审判':<10} {'分数':>5} {'审美':<6} {'过滤':<6}",
            f"  {'─'*45}",
        ]
        for v in self.versions:
            sel = " ★" if v.is_selected else ""
            lines.append(
                f"  {v.version_name:<3}{sel} {v.judgment['verdict']:<10} "
                f"{v.judgment['score']:>+4.1f} "
                f"{v.aesthetics['grade']:<6} "
                f"{'过' if v.passed_filter else '汰':<6}"
            )
        return "\n".join(lines)


# ============================================================
# 主系统
# ============================================================

class FullSystem:
    """
    满全法 · 层级治理系统

    公约数锚定 → 公倍数旋转 → 照镜审判 → 审美评估 → 淘汰过滤 → 版本积累
    """

    def __init__(self, storage_dir: str):
        self.divisor = CommonDivisor()
        self.rotator = Rotator()
        self.vision = Vision()
        self.hearing = Hearing()
        self.judgment = Judgment()
        self.aesthetics = Aesthetics()
        self.elimination = Elimination()
        self.filter = Filter()
        self.memory = Memory(storage_dir)

    def process(self, m: CommonMultiple, action: str = "检验") -> VersionRecord:
        """处理公倍数 —— 出新版本"""
        # 视觉
        p = self.vision.see(m)
        # 听觉
        h = self.hearing.listen(m)
        # 审判（照镜子）
        j = self.judgment.judge(m, self.divisor)
        # 审美
        a = self.aesthetics.evaluate(m)
        # 过滤
        passed = self.filter.filter_pass(h, j, a)
        # 记录新版本
        return self.memory.record(action, j, a, passed)

    def process_with_fix(self, m: CommonMultiple) -> Tuple[VersionRecord, Optional[VersionRecord]]:
        """处理 + 不通过则修正出新版"""
        original = self.process(m, "原版")
        if not original.passed_filter:
            fixed = CommonMultiple(
                owner=m.owner, value=m.value, content=m.content,
                clean=True, dry=True, has_worm=False, has_sand=False,
                edible=m.edible, washed=True,
            )
            fix = self.process(fixed, "修正版")
            return original, fix
        return original, None


# ============================================================
# 主程序
# ============================================================

def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions_full"
    system = FullSystem(storage)

    print("=" * 60)
    print("  满全法 · 层级治理系统")
    print("  ─────────────────────────────")
    print("  公约数(锚) → 公倍数(旋转) → 照镜审判 → 审美 → 淘汰/过滤")
    print("  层级：袋子级别（高维），不数米（低维）")
    print("=" * 60)

    # 生成公倍数
    multiples = [
        system.rotator.generate("张", 100, "东北大米", clean=True, dry=True, has_worm=False, has_sand=False, edible=True),
        system.rotator.generate("李", 80, "江南糯米", clean=False, dry=True, has_worm=False, has_sand=True, edible=True),
        system.rotator.generate("王", 120, "泰国香米", clean=True, dry=False, has_worm=True, has_sand=False, edible=True),
        system.rotator.generate("赵", 60, "陈年旧米", clean=False, dry=False, has_worm=True, has_sand=True, edible=False),
        system.rotator.generate("刘", 150, "有机新米", clean=True, dry=True, has_worm=False, has_sand=False, edible=True, washed=True),
    ]

    for m in multiples:
        original, fix = system.process_with_fix(m)
        print(f"\n{'─'*50}")
        print(f"  {m.owner}家 [{original.version_name}]")
        print(f"  {system.vision.render(system.vision.see(m))}")
        print(f"  照镜：{original.judgment['verdict']} | {original.judgment['score']:+.1f}")
        print(f"  审美：{original.aesthetics['grade']} | {original.aesthetics['score']:+.0f}")
        print(f"  过滤：{'通过' if original.passed_filter else '淘汰'}")
        if fix:
            print(f"  ↳ 修正 [{fix.version_name}]：{fix.judgment['verdict']} | {fix.aesthetics['grade']} | {'通过' if fix.passed_filter else '淘汰'}")

    print(system.memory.summary())

    # 选出版本
    passed = [v for v in system.memory.versions if v.passed_filter]
    if passed:
        best = max(passed, key=lambda v: v.judgment["score"])
        print(f"\n{system.memory.select(best.version_number)}")

    print(f"\n  版本存储：{storage}")
    print("  公约数不动，每次旋转出新版本")


if __name__ == "__main__":
    main()
