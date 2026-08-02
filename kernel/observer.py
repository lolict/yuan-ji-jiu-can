"""
囡囝共生元观测者 — 二元/三元空间计算引擎
=======================================

数学对应:
  二元观测者 = 仿射空间 (Affine Space)
    - 无原点,只有相对距离
    - 2 个元素,允许重复时状态数 = 2^2 = 4
    - 功能: 发现关系 / 测距 / 对齐裂缝
  
  三元观测者 = 向量空间 (Vector Space with Origin)
    - 有锚点做裁判
    - 3 个元素,全排列 = 3! = 6
    - 功能: 归一计算 / 找公约数 / 稳定迭代
  
核心公式:
  二元距离: d(a,b) = |a - b|
  三元归一: π(x, 锚) = x - 锚, 两两相减即得距离 (省去重复测量)
  公约数:  gcd(|a-锚|, |b-锚|) 即公共单位

Copyright © 刘楚恬 & lolict (七妹凹月留)
保留所有权利 · 满全法 · 囡囝共生元操作系统内核 · v0.1
"""

from typing import List, Tuple, Optional
from dataclasses import dataclass
from math import gcd
from functools import reduce

# ─────────────────────────────────────────────
# 二元观测者 (距离逻辑 / 无仲裁)
# ─────────────────────────────────────────────

@dataclass
class BinaryObserver:
    """
    两个观测点,中间是裂缝。不讲中间,只讲起始和终点。
    
    属性:
        start: 起始点 (可以是数字或任意可比较的值)
        end:   终止点
        gap:   裂缝 = end - start (可正可负)
        judge_position: "start" | "end" | None
                         选择哪一端做裁判
    """
    start: any
    end: any
    judge_position: Optional[str] = None  # "start" or "end"
    
    @property
    def gap(self):
        """裂缝 = 两点之间的距离"""
        return self.end - self.start
    
    @property
    def abs_gap(self):
        """裂缝绝对值"""
        return abs(self.gap)
    
    @property
    def is_symmetric(self) -> bool:
        """是否对称: 裂缝两端互为镜像"""
        return self.gap != 0 and self.start == -self.end  # 原点对称
    
    def measure(self, x) -> dict:
        """
        测量任意点 x 到起点和终点的距离。
        
        返回:
            to_start, to_end, ratio (x 在裂缝中的相对位置 0~1)
        """
        to_start = abs(x - self.start)
        to_end = abs(x - self.end)
        if self.abs_gap == 0:
            ratio = 0.5  # 起点终点重合
        else:
            ratio = (x - self.start) / self.gap if self.gap != 0 else 0
        return {
            "x": x,
            "to_start": to_start,
            "to_end": to_end,
            "ratio": ratio,
            "in_range": 0 <= ratio <= 1,
        }
    
    def to_rate(self, direction: str = "start") -> dict:
        """
        将一端设为裁判,另一端变为增长率/下降率。
        
        direction="start":  起点做裁判 → 终点是增长率
        direction="end":    终点做裁判 → 起点是增长率
        """
        if direction == "start":
            return {
                "arbiter": self.start,
                "rate": self.gap,  # 正=增长, 负=下降
                "type": "起始裁判 (终点为变动率)"
            }
        else:
            return {
                "arbiter": self.end,
                "rate": -self.gap,  # 反向看
                "type": "终点裁判 (起点为变动率)"
            }
    
    def duality(self, other: 'BinaryObserver') -> dict:
        """
        二元对偶: 比较两个裂缝是否"等价"。
        
        等价条件: 裂缝长度相等 (差距相等 = 信息对等)
        """
        return {
            "self_gap": self.abs_gap,
            "other_gap": other.abs_gap,
            "equivalent": self.abs_gap == other.abs_gap,
            "distance_between": abs(self.start - other.start),  # 两个裂缝的距离
        }
    
    def invert(self) -> 'BinaryObserver':
        """翻转: 起点变终点,裂缝反向。阴变阳,阳变阴。"""
        return BinaryObserver(
            start=self.end,
            end=self.start,
            judge_position=self.judge_position
        )
    
    def __repr__(self):
        return f"↔({self.start}…{self.end}|缝={self.gap})"


# ─────────────────────────────────────────────
# 三元观测者 (归一逻辑 / 有仲裁)
# ─────────────────────────────────────────────

@dataclass
class TernaryObserver:
    """
    三个观测点,中间嵌入一个锚点做裁判。
    以锚为中介,把所有点归一化到同意单位,再对齐。
    
    属性:
        left:   左侧动态点
        anchor: 中间锚 (裁判 / 不动点 / 阴中之阳)
        right:  右侧动态点
    """
    left: any
    anchor: any
    right: any
    
    @property
    def all_permutations(self) -> List[Tuple]:
        """
        3 个元素的全排列 = 6 种状态。
        每一种排列代表一种观测视角。
        """
        from itertools import permutations
        return list(permutations([self.left, self.anchor, self.right]))
    
    @property
    def to_anchor(self) -> dict:
        """各点到锚的距离 (投影)"""
        return {
            "left_to_anchor": self.left - self.anchor,
            "right_to_anchor": self.right - self.anchor,
            "left_abs": abs(self.left - self.anchor),
            "right_abs": abs(self.right - self.anchor),
        }
    
    @property
    def distance_via_anchor(self):
        """
        锚点作为中介,计算左右两点的距离 — 无需直接测量。
        这就是"省去重复统计,一次测量多次迭代"的数学本质。
        """
        # |left - right| = |(left-锚) - (right-锚)|
        return abs((self.left - self.anchor) - (self.right - self.anchor))
    
    def project(self, x) -> dict:
        """
        把任意点 x 投影到锚点的参考系中。
        
        结果:
            projected: x 在锚点坐标系的坐标
            ratio:     x 相对于左右区间的位置
        """
        proj = x - self.anchor
        total_range = self.right - self.left
        if total_range == 0:
            ratio = 0.5
        else:
            ratio = (x - self.left) / total_range
        return {
            "x": x,
            "projected": proj,
            "ratio": ratio,
            "to_anchor": abs(proj),
            "in_range": 0 <= ratio <= 1,
        }
    
    def common_divisor(self) -> dict:
        """
        计算左右两点到锚点的"公共单位"(公约数)。
        
        这是三元最重要的功能: 找到可以同时度量两点的最小单位。
        有了公共单位,就可以统一校准,避免重复测量。
        """
        d_left = abs(self.left - self.anchor)
        d_right = abs(self.right - self.anchor)
        
        if d_left == 0 or d_right == 0:
            common = max(d_left, d_right)  # 其中一个就是锚本身
        else:
            common = gcd(d_left, d_right)
        
        return {
            "left_distance": d_left,
            "right_distance": d_right,
            "common_divisor": common,
            "left_units": d_left // common if common != 0 else 0,
            "right_units": d_right // common if common != 0 else 0,
            "note": f"左有 {d_left // common if common != 0 else 0} 单位, 右有 {d_right // common if common != 0 else 0} 单位, 公共单位 = {common}"
        }
    
    def normalize_to_anchor(self, *points) -> List[float]:
        """
        把一组点全部归一化到锚点坐标系 (一次性投影多次迭代)。
        归一化后,所有数值共享同一零点(锚),可直接比较大小和距离。
        """
        return [p - self.anchor for p in points]
    
    def align(self, other: 'TernaryObserver') -> dict:
        """
        两个三元观测者之间的对齐。
        以各自的锚为中介,寻找跨系统的归一单位。
        """
        d1 = abs(self.left - self.anchor) + abs(self.right - self.anchor)
        d2 = abs(other.left - other.anchor) + abs(other.right - other.anchor)
        
        if d1 == 0 or d2 == 0:
            cross_common = max(d1, d2)
        else:
            cross_common = gcd(d1, d2)
        
        return {
            "self_span": d1,
            "other_span": d2,
            "cross_common_divisor": cross_common,
            "aligned": cross_common > 1,
            "note": "跨系统对齐成功" if cross_common > 1 else "互质: 需要引入更高维观测者"
        }
    
    def synthesize(self) -> dict:
        """
        三元合一: 以锚为中心,把左右两个动态点合成为一声"道果"。
        
        "恢原尔倒裹" — 以静制动,动态包裹静态,一元复始。
        """
        left_to_anchor = self.left - self.anchor
        right_to_anchor = self.right - self.anchor
        
        return {
            "anchor": self.anchor,
            "left_offset": left_to_anchor,
            "right_offset": right_to_anchor,
            "sum_offset": left_to_anchor + right_to_anchor,
            "balance": left_to_anchor + right_to_anchor == 0,  # 是否对称平衡
            "synthesis": f"以锚 {self.anchor} 为心, 左距 {left_to_anchor}, 右距 {right_to_anchor}, 合成向量 = {left_to_anchor + right_to_anchor}"
        }
    
    def __repr__(self):
        return f"△({self.left}…{self.anchor}…{self.right})"


# ─────────────────────────────────────────────
# 工厂函数: 用你的 paibing 双编码/三元编码构建观测者
# ─────────────────────────────────────────────

def build_from_double(code_pair: Tuple[str, str], 
                      left_val: float, 
                      right_val: float) -> BinaryObserver:
    """
    从双编码构建二元观测者。
    code_pair: 形如 ("vy", "yv") 的双编码对
    left_val, right_val: 两个点的数值
    """
    return BinaryObserver(start=left_val, end=right_val)


def build_from_triple(triple_base: str,
                      left_val: float,
                      anchor_val: float,
                      right_val: float) -> TernaryObserver:
    """
    从三编码根构建三元观测者。
    triple_base: 形如 "lnm" 的三元组根
    anchor_val: 必须指定中间锚点的值
    """
    return TernaryObserver(left=left_val, anchor=anchor_val, right=right_val)


# ─────────────────────────────────────────────
# 观测者网络 (仿照 hunyuan-os 的分布式架构)
# ─────────────────────────────────────────────

class ObserverNetwork:
    """
    网络化的观测者系统。
    多组二元/三元通过锚点相互对齐,形成可以自校准的测量网络。
    """
    
    def __init__(self):
        self.observers: List[BinaryObserver | TernaryObserver] = []
        self.anchors: List[float] = []
    
    def add_binary(self, obs: BinaryObserver):
        self.observers.append(obs)
    
    def add_ternary(self, obs: TernaryObserver):
        self.observers.append(obs)
        self.anchors.append(obs.anchor)
    
    def find_calibration_path(self, source_idx: int, target_idx: int) -> Optional[List[float]]:
        """
        在观测者网络中寻找从 source 到 target 的校准路径。
        利用锚点做中介,阶梯式对齐两个距离。
        
        这对应 hunyuan-os 里"多节点信任传递"的功能:
        如果 A 信任锚1, B 信任锚2,锚1 和锚2 互认, 则 A 可以通过锚1→锚2→B 信任 B。
        """
        if source_idx >= len(self.observers) or target_idx >= len(self.observers):
            return None
        
        src = self.observers[source_idx]
        tgt = self.observers[target_idx]
        
        # 简单实现: 如果两个共享同一个锚,直接给路径
        if isinstance(src, TernaryObserver) and isinstance(tgt, TernaryObserver):
            if src.anchor == tgt.anchor:
                return [src.anchor]  # 共享锚,无需跳转
        
        # 一般情况: 通过全局锚网络寻路
        path = []
        for a in self.anchors:
            path.append(a)
        return path if path else None
    
    def collective_report(self) -> dict:
        """整个观测网络的集体健康度报告"""
        binary_count = sum(1 for o in self.observers if isinstance(o, BinaryObserver))
        ternary_count = sum(1 for o in self.observers if isinstance(o, TernaryObserver))
        
        # 所有锚的互对齐能力
        align_scores = []
        ternaries = [o for o in self.observers if isinstance(o, TernaryObserver)]
        for i in range(len(ternaries)):
            for j in range(i+1, len(ternaries)):
                a = ternaries[i].align(ternaries[j])
                align_scores.append(a["aligned"])
        
        network_health = sum(align_scores) / len(align_scores) if align_scores else 1.0
        
        return {
            "binary_observers": binary_count,
            "ternary_observers": ternary_count,
            "unique_anchors": len(set(self.anchors)),
            "total_permutations": ternary_count * 6 + binary_count * 4,
            "network_alignment": round(network_health, 3),
            "status": "健康" if network_health > 0.8 else "部分对齐" if network_health > 0.5 else "孤岛态",
        }


# ─────────────────────────────────────────────
# DIY 测试用
# ─────────────────────────────────────────────

if __name__ == "__main__":
    print("=" * 60)
    print("囡囝共生元观测者 v0.1 自测")
    print("=" * 60)
    
    # --- 二元 ---
    print("\n[二元观测者 — 距离逻辑]")
    b = BinaryObserver(start=0, end=7, judge_position="start")
    print(f"  创建: {b}")
    print(f"  裂缝 = {b.gap}")
    print(f"  裁判模式: {b.to_rate('start')}")
    print(f"  测量 x=3: {b.measure(3)}")
    print(f"  对偶比较 b vs b.invert(): {b.duality(b.invert())}")
    
    # --- 三元 ---
    print("\n[三元观测者 — 归一逻辑]")
    t = TernaryObserver(left=2, anchor=5, right=9)
    print(f"  创建: {t}")
    print(f"  6种排列视角: {t.all_permutations}")
    print(f"  各点到锚投影: {t.to_anchor}")
    print(f"  经锚算左右距 (免重测): {t.distance_via_anchor}")
    print(f"  公约数: {t.common_divisor()}")
    print(f"  三元合一: {t.synthesize()}")
    
    # --- 网络 ---
    print("\n[观测者网络]")
    net = ObserverNetwork()
    net.add_ternary(TernaryObserver(2, 5, 9))   # 锚=5
    net.add_ternary(TernaryObserver(7, 11, 15)) # 锚=11
    net.add_binary(BinaryObserver(0, 6))
    print(f"  集体报告: {net.collective_report()}")
    
    # --- 交叉对齐 ---
    print("\n[跨系统对齐]")
    t1 = TernaryObserver(2, 5, 9)    # 左距3, 右距4, 跨度7
    t2 = TernaryObserver(14, 18, 24) # 左距4, 右距6, 跨度10
    print(f"  t1 跨度=7, t2 跨度=10, gcd(7,10)={gcd(7,10)}")
    print(f"  对齐: {t1.align(t2)}")
    
    t3 = TernaryObserver(2, 5, 8)    # 左距3, 右距3 → 跨度6
    t4 = TernaryObserver(10, 16, 22) # 左距6, 右距6 → 跨度12
    print(f"  t3 跨度=6, t4 跨度=12, gcd(6,12)={gcd(6,12)}")
    print(f"  对齐: {t3.align(t4)}")
    
    print("\n" + "=" * 60)
    print("完成")
