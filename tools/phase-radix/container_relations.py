#!/usr/bin/env python3
# ============================================================
# Container Relation Calculus — 容器关系演算
# 囡囝共生 OS 的几何基础引擎
#
# 核心哲学实现:
#   进制 = 地址    → 主权值本身就是相空间位置
#   公约数 = 距离  → GCD(a,b) 自然涌现接近程度
#   折叠 = 包含    → a→b iff b.sov % a.sov == 0
#   极性 = 几何类型 → 极性对确定交互的几何形态
#   见证 = 裁判    → 见证字段确定谁做裁定
#
# 不需要查表 — 所有关系由数学自然生成
# ============================================================

import math
from typing import Optional

# ============================================================
# 常量
# ============================================================
POL_NAMES = ['妻', '夫', '系统', '观察', '裁决', '秘']
FLOW_NAMES = ['预加载', '分类', '聚类', '辨别', '产权', '解释', '关系', '概念']
STATE_NAMES = ['NN', 'NJSG', 'JJ', 'JN']

# 囡囝四态 → 裁决倾向权重
STATE_JUDGE_BIAS = {
    0: {'MERGE': 0.7, 'REPEL': 0.1, 'WITNESS': 0.6, 'TRANSFORM': 0.4, 'DEFER': 0.3, 'ESCALATE': 0.2},  # NN
    1: {'MERGE': 0.9, 'REPEL': 0.15, 'WITNESS': 0.5, 'TRANSFORM': 0.3, 'DEFER': 0.2, 'ESCALATE': 0.15},  # NJSG
    2: {'MERGE': 0.2, 'REPEL': 0.85, 'WITNESS': 0.4, 'TRANSFORM': 0.6, 'DEFER': 0.3, 'ESCALATE': 0.25},  # JJ
    3: {'MERGE': 0.5, 'REPEL': 0.5, 'WITNESS': 0.7, 'TRANSFORM': 0.5, 'DEFER': 0.5, 'ESCALATE': 0.3},  # JN
}

# 极性交互矩阵: (pol_a, pol_b) → 基础几何类型
#      妻    夫   系统  观察  裁决   秘
POL_MATRIX = [
    # 妻
    ['self', 'couple', 'host', 'mirror', 'shield', 'hidden'],
    # 夫
    ['couple', 'self', 'host', 'mirror', 'shield', 'hidden'],
    # 系统
    ['host', 'host', 'merge', 'contain', 'rule', 'core'],
    # 观察
    ['mirror', 'mirror', 'contain', 'observe', 'record', 'peek'],
    # 裁决
    ['shield', 'shield', 'rule', 'record', 'judge', 'cut'],
    # 秘
    ['hidden', 'hidden', 'core', 'peek', 'cut', 'mystery'],
]

# 几何类型 → 裁决议倾向
GEOMETRY_VERDICT = {
    'self':      {'MERGE': 1.0, 'WITNESS': 0.3, 'DEFER': 0.1},
    'couple':    {'MERGE': 0.9, 'REPEL': 0.15, 'TRANSFORM': 0.4},
    'host':      {'MERGE': 0.6, 'WITNESS': 0.4, 'TRANSFORM': 0.3},
    'mirror':    {'WITNESS': 0.7, 'MERGE': 0.3, 'TRANSFORM': 0.5},
    'shield':    {'REPEL': 0.5, 'WITNESS': 0.5, 'DEFER': 0.4},
    'hidden':    {'DEFER': 0.4, 'ESCALATE': 0.4, 'WITNESS': 0.3},
    'merge':     {'MERGE': 0.95, 'TRANSFORM': 0.2},
    'contain':   {'MERGE': 0.5, 'WITNESS': 0.6, 'TRANSFORM': 0.3},
    'rule':      {'WITNESS': 0.7, 'REPEL': 0.3, 'TRANSFORM': 0.4},
    'record':    {'WITNESS': 0.8, 'DEFER': 0.3},
    'observe':   {'WITNESS': 0.6, 'DEFER': 0.5},
    'peek':      {'DEFER': 0.4, 'ESCALATE': 0.3},
    'core':      {'ESCALATE': 0.4, 'MERGE': 0.3},
    'judge':     {'WITNESS': 0.9, 'REPEL': 0.3},
    'cut':       {'REPEL': 0.6, 'TRANSFORM': 0.4},
    'mystery':   {'ESCALATE': 0.3, 'DEFER': 0.4},
}

# 折叠等级
FOLD_NONE = 0    # 无折叠
FOLD_A_IN_B = 1  # a 折叠入 b
FOLD_B_IN_A = 2  # b 折叠入 a
FOLD_MUTUAL = 3  # 互折叠 (a=b)


# ============================================================
# 64位细胞编码解析
# ============================================================
def parse_cell(cell: int) -> dict:
    """解析64位细胞编码为字段"""
    return {
        'raw': cell,
        'polarity': (cell >> 60) & 0xF,
        'witness': (cell >> 56) & 0xF,
        'flow':    (cell >> 52) & 0xF,
        'sovereignty': (cell >> 32) & 0x1FFFFF,
        'mccp':    cell & 0xFFFFFFFF,
    }

def build_cell(mccp: int, sov: int, pol: int, wit: int, flow: int = 0) -> int:
    """构造64位细胞编码"""
    return ((pol & 0xF) << 60) | ((wit & 0xF) << 56) | ((flow & 0xF) << 52) | \
           ((sov & 0x1FFFFF) << 32) | (mccp & 0xFFFFFFFF)


# ============================================================
# 容器关系演算引擎
# ============================================================
class ContainerRelationEngine:
    """
    容器关系演算引擎
    
    所有计算基于数学自然涌现:
    - 距离 = GCD / min (不需要存储空间)
    - 包含 = 模运算 (不需要标记)
    - 几何 = 极性自然属性 (不需要声明)
    """

    def __init__(self, state: int = 1):
        self.state = state  # 囡囝四态

    # ── 基本几何原语 ──

    def gcd_distance(self, a_sov: int, b_sov: int) -> float:
        """
        GCD距离: gcd(a,b) / min(a,b)
        =1.0 → 完全同构 (同一位置)
        →0   → 互质 (最远距离)
        """
        if a_sov <= 0 or b_sov <= 0:
            return 0.0
        mn = min(a_sov, b_sov)
        return math.gcd(a_sov, b_sov) / mn

    def lcm_span(self, a_sov: int, b_sov: int) -> int:
        """最小公倍数 = 两细胞的公共周期"""
        if a_sov <= 0 or b_sov <= 0:
            return 0
        return a_sov * b_sov // math.gcd(a_sov, b_sov)

    def fold_relation(self, a_sov: int, b_sov: int) -> int:
        """
        折叠关系:
        - A→B: a折叠入b iff b % a == 0 (a是b的容器)
        - B→A: b折叠入a iff a % b == 0
        折叠 = 包含 = 地址空间的子集关系 (进制即地址)
        """
        if a_sov <= 0 or b_sov <= 0:
            return FOLD_NONE
        a_in_b = (a_sov <= b_sov) and (b_sov % a_sov == 0)
        b_in_a = (b_sov <= a_sov) and (a_sov % b_sov == 0)
        if a_in_b and b_in_a:
            return FOLD_MUTUAL
        if a_in_b:
            return FOLD_A_IN_B
        if b_in_a:
            return FOLD_B_IN_A
        return FOLD_NONE

    def phase_distance(self, a_sov: int, b_sov: int) -> float:
        """
        相位距离: 主权值之比的对数距离
        d = |ln(a) - ln(b)| / ln(max(a,b))
        同阶细胞 → 小距离
        跨阶细胞 → 大距离
        """
        if a_sov <= 0 or b_sov <= 0:
            return 1.0
        mx = max(a_sov, b_sov)
        if mx <= 1:
            return 0.0
        return abs(math.log2(a_sov) - math.log2(b_sov)) / math.log2(mx)

    def polarity_geometry(self, pol_a: int, pol_b: int) -> str:
        """极性对 → 交互几何类型"""
        pa = min(pol_a, len(POL_MATRIX) - 1)
        pb = min(pol_b, len(POL_MATRIX[0]) - 1)
        return POL_MATRIX[pa][pb]

    def witness_strength(self, wit_a: int, wit_b: int) -> float:
        """
        见证强度: 两细胞见证字段之和决定裁判清晰度
        0 → 无见证 (裁决弱)
        8+ → 强见证 (裁决强)
        """
        total = wit_a + wit_b
        return min(total / 8.0, 1.0)

    def sovereignty_distance(self, a_sov: int, b_sov: int) -> float:
        """
        主权距离: 基于GCD的加权距离公式
        d = 1 - gcd(a,b)^2 / (a*b)
        = 0 → 完全共享 (gcd=a=b)
        → 1 → 完全互质
        """
        if a_sov <= 0 or b_sov <= 0:
            return 1.0
        g = math.gcd(a_sov, b_sov)
        return 1.0 - (g * g) / (a_sov * b_sov)

    # ── 关系裁决 ──

    def compute_relationship(self, cell_a: int, cell_b: int, state: Optional[int] = None) -> dict:
        """
        计算两细胞之间的完整关系裁决
        
        融合全部:
        1. GCD距离 (进制几何)
        2. 折叠包含 (进制地址)
        3. 极性几何 (自然类型)
        4. 见证强度 (裁判资质)
        5. 囡囝态权重 (状态共振)
        → 输出: 六裁决定 + 置信度
        """
        if state is None:
            state = self.state

        pa = parse_cell(cell_a)
        pb = parse_cell(cell_b)

        sov_a = pa['sovereignty']
        sov_b = pb['sovereignty']
        pol_a = pa['polarity']
        pol_b = pb['polarity']
        wit_a = pa['witness']
        wit_b = pb['witness']

        # 步骤 1: GCD距离
        gcd_dist = self.gcd_distance(sov_a, sov_b)
        sov_dist = self.sovereignty_distance(sov_a, sov_b)
        phase_dist = self.phase_distance(sov_a, sov_b)

        # 步骤 2: 折叠关系
        fold = self.fold_relation(sov_a, sov_b)

        # 步骤 3: 极性几何
        geom = self.polarity_geometry(pol_a, pol_b)

        # 步骤 4: 见证强度
        wit_str = self.witness_strength(wit_a, wit_b)

        # 步骤 5: 囡囝态权重
        state_bias = STATE_JUDGE_BIAS.get(state, STATE_JUDGE_BIAS[1])

        # 步骤 6: 几何类型裁决议倾向
        geom_verdict = GEOMETRY_VERDICT.get(geom, {'WITNESS': 0.5, 'DEFER': 0.5})

        # ── 融合六裁决置信度 ──
        verdict_scores = {'MERGE': 0, 'REPEL': 0, 'WITNESS': 0, 'TRANSFORM': 0, 'DEFER': 0, 'ESCALATE': 0}

        # 基于GCD距离贡献
        verdict_scores['MERGE'] += gcd_dist * 0.3
        verdict_scores['REPEL'] += sov_dist * 0.3

        # 基于折叠关系
        if fold == FOLD_A_IN_B:
            verdict_scores['MERGE'] += 0.25
            verdict_scores['TRANSFORM'] += 0.1
        elif fold == FOLD_B_IN_A:
            verdict_scores['MERGE'] += 0.25
            verdict_scores['TRANSFORM'] += 0.1
        elif fold == FOLD_MUTUAL:
            verdict_scores['MERGE'] += 0.35
        else:
            verdict_scores['REPEL'] += 0.15
            verdict_scores['WITNESS'] += 0.1

        # 基于极性几何
        for v, w in geom_verdict.items():
            verdict_scores[v] += w * 0.25

        # 基于见证强度
        verdict_scores['WITNESS'] += wit_str * 0.2
        if wit_str < 0.3:
            verdict_scores['DEFER'] += 0.15  # 弱见证 → 倾向延迟

        # 基于相位距离
        verdict_scores['ESCALATE'] += phase_dist * 0.15
        if phase_dist < 0.2:
            verdict_scores['TRANSFORM'] += 0.1

        # 应用囡囝态权重
        for v in verdict_scores:
            bias = state_bias.get(v, 0.5)
            verdict_scores[v] *= bias

        # 选最高分裁决
        best_verdict = max(verdict_scores, key=verdict_scores.get)
        confidence = min(verdict_scores[best_verdict], 1.0)

        # 确定输出细胞 (融合时)
        output_cell = None
        if best_verdict == 'MERGE':
            new_sov = self.lcm_span(sov_a, sov_b)  # LCM = 融合周期
            new_mccp = pa['mccp'] ^ pb['mccp']     # XOR = 信息混合
            new_pol = pol_a  # 取第一个极性
            new_wit = min(wit_a + wit_b, 15)
            output_cell = build_cell(new_mccp, new_sov, new_pol, new_wit)

        return {
            'verdict': best_verdict,
            'verdict_str': best_verdict,
            'confidence': confidence,
            'output_cell': output_cell,
            'metrics': {
                'gcd_dist': round(gcd_dist, 4),
                'sov_dist': round(sov_dist, 4),
                'phase_dist': round(phase_dist, 4),
                'fold': fold,
                'fold_str': ['无', 'a→b', 'b→a', '互'][fold],
                'geometry': geom,
                'witness_str': round(wit_str, 2),
                'state': state,
            },
            'scores': {k: round(v, 3) for k, v in verdict_scores.items()},
        }

    def batch_relationships(self, cells: list, state: Optional[int] = None) -> list:
        """计算细胞列表中所有两两关系"""
        results = []
        n = len(cells)
        for i in range(n):
            for j in range(i + 1, n):
                r = self.compute_relationship(cells[i], cells[j], state)
                r['pair'] = (i, j)
                results.append(r)
        return results

    def find_containers(self, cells: list, target_idx: int) -> list:
        """找到能容纳目标细胞的所有细胞 (基于折叠关系)"""
        target = parse_cell(cells[target_idx])
        result = []
        for i, c in enumerate(cells):
            if i == target_idx:
                continue
            pc = parse_cell(c)
            fold = self.fold_relation(pc['sovereignty'], target['sovereignty'])
            if fold == FOLD_A_IN_B:  # cells[i] 折叠入 target → cells[i] 是 target 的子
                result.append({'idx': i, 'fold': 'sub', 'cell': c})
            elif fold == FOLD_B_IN_A:  # target 折叠入 cells[i] → cells[i] 是 target 的超
                result.append({'idx': i, 'fold': 'super', 'cell': c})
        return result

    def find_nearest(self, cells: list, target_idx: int, top_k: int = 3) -> list:
        """找到距目标最近的 top_k 个细胞 (GCD距离最小)"""
        target = parse_cell(cells[target_idx])
        distances = []
        for i, c in enumerate(cells):
            if i == target_idx:
                continue
            pc = parse_cell(c)
            d = self.sovereignty_distance(target['sovereignty'], pc['sovereignty'])
            distances.append((i, d))
        distances.sort(key=lambda x: x[1])
        return distances[:top_k]


# ============================================================
# CLI 自测
# ============================================================
if __name__ == '__main__':
    engine = ContainerRelationEngine(state=1)  # NJSG

    print("=== 容器关系演算 · 自测 ===\n")

    # 构造测试细胞
    c1 = build_cell(0x1000, 2520, 0, 2)   # 妻 · 主权2520 (LCM锚点)
    c2 = build_cell(0x2000, 60, 1, 2)     # 夫 · 主权60
    c3 = build_cell(0x3000, 720, 2, 3)    # 系统 · 主权720
    c4 = build_cell(0x4000, 7, 0, 1)      # 妻 · 主权7 (质数)
    c5 = build_cell(0x5000, 840, 1, 4)    # 夫 · 主权840

    cells = [c1, c2, c3, c4, c5]
    labels = ['2520(妻)', '60(夫)', '720(系统)', '7(妻)', '840(夫)']

    # 测试 1: 2520 vs 60 (60 整除 2520 → 折叠)
    print("[Test 1] 2520 (妻) vs 60 (夫)")
    r = engine.compute_relationship(c1, c2)
    m = r['metrics']
    print(f"  裁决: {r['verdict']} ({r['confidence']:.2f})")
    print(f"  GCD距离: {m['gcd_dist']}  折叠: {m['fold_str']}  几何: {m['geometry']}")
    print(f"  预期: 60|2520 → 折叠 → MERGE")
    print()

    # 测试 2: 2520 vs 7 (质数，GCD=1 → 互质)
    print("[Test 2] 2520 (妻) vs 7 (妻)")
    r = engine.compute_relationship(c1, c4)
    m = r['metrics']
    print(f"  裁决: {r['verdict']} ({r['confidence']:.2f})")
    print(f"  GCD距离: {m['gcd_dist']}  主权距离: {m['sov_dist']}  折叠: {m['fold_str']}")
    print(f"  预期: GCD=1 → 互质 → REPEL 或 WITNESS")
    print()

    # 测试 3: 全对关系矩阵
    print("[Test 3] 全对关系矩阵 (5细胞)")
    batch = engine.batch_relationships(cells)
    for r in batch:
        i, j = r['pair']
        m = r['metrics']
        print(f"  {labels[i]:10} × {labels[j]:10} → {r['verdict']:8} (conf={r['confidence']:.2f}) "
              f"gcd={m['gcd_dist']:.3f} fold={m['fold_str']}")
    print()

    # 测试 4: 找最近邻
    print("[Test 4] 距 7(妻) 最近的细胞")
    nearest = engine.find_nearest(cells, 3)
    for idx, d in nearest:
        print(f"  → {labels[idx]} (sov_dist={d:.3f})")
    print()

    # 测试 5: 找容器
    print("[Test 5] 60(夫) 的容器层级")
    containers = engine.find_containers(cells, 1)
    for c in containers:
        print(f"  {labels[c['idx']]:10} → {c['fold']}")
    print()

    # 测试 6: 囡囝态对比
    print("[Test 6] 同一对细胞在不同囡囝态下的裁决")
    for state in range(4):
        engine_s = ContainerRelationEngine(state=state)
        r = engine_s.compute_relationship(c2, c5)  # 60 vs 840
        print(f"  {STATE_NAMES[state]:4} → {r['verdict']:8} ({r['confidence']:.2f})")
    print()

    print("=== 自测完成 ===")
