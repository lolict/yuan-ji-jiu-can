// ══════════════════════════════════════════════════════════════════
// five_yuan.rs — 五元旋钮 · 流转博弈引擎
// ══════════════════════════════════════════════════════════════════
//
// 定位:
//   不科学但有用 — 像五行一样，是思维工具，不是科学理论。
//   五个节点、两种关系（互补/互害）、封闭式流转。
//
// 核心结构:
//   · 五个元（Yuan），每个有一个名字和当前强度 (0..1)
//   · 关系矩阵 5×5: +1 相生(互补增益), -1 相克(互害损耗), 0 无关
//   · 流转: tick() 推动关系矩阵旋转，元与元之间按关系交换能量
//
// 命名:
//   "旋钮" — 可以手动旋转到任意一个元作为当前观察点
//   "五元" — 不是二元的正反对立，是五个追逐彼此的闭环
//   "抓尾巴" — 元A 作用于 元B 也接受 元E 的作用，循环无端

// ────────────────────────────────────────────────────────────
// 五元定义 — 默认五元是抽象的 A/B/C/D/E
// 用户在外部调用时映射为具体语义（我/她/系统/满全法/囡囝）
// ────────────────────────────────────────────────────────────

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Yuan {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
}

impl Yuan {
    const ALL: [Yuan; 5] = [Yuan::A, Yuan::B, Yuan::C, Yuan::D, Yuan::E];

    pub fn as_index(self) -> usize {
        self as usize
    }

    pub fn from_index(i: usize) -> Self {
        match i % 5 {
            0 => Yuan::A,
            1 => Yuan::B,
            2 => Yuan::C,
            3 => Yuan::D,
            4 => Yuan::E,
            _ => unreachable!(),
        }
    }

    pub fn next(self) -> Self {
        Yuan::from_index(self.as_index() + 1)
    }

    pub fn prev(self) -> Self {
        Yuan::from_index(self.as_index() + 4)
    }
}

impl fmt::Display for Yuan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Yuan::A => "A",
            Yuan::B => "B",
            Yuan::C => "C",
            Yuan::D => "D",
            Yuan::E => "E",
        };
        write!(f, "{}", name)
    }
}

// ────────────────────────────────────────────────────────────
// 关系类型: 互补(相生) / 互害(相克)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YuanRelation {
    Synergize,  // 相生: +1, 互补增益
    Conflict,   // 相克: -1, 互害损耗
}

impl YuanRelation {
    pub fn sign(self) -> f64 {
        match self {
            YuanRelation::Synergize => 1.0,
            YuanRelation::Conflict => -1.0,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            YuanRelation::Synergize => YuanRelation::Conflict,
            YuanRelation::Conflict => YuanRelation::Synergize,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 五元旋钮引擎
// ────────────────────────────────────────────────────────────

pub struct FiveYuan {
    /// 五个元的当前强度 (0.0 = 空, 1.0 = 满)
    pub strength: [f64; 5],
    /// 关系矩阵: matrix[i][j] 表示 i 对 j 的关系
    ///   +1.0 = i 增益 j (i 生 j)
    ///   -1.0 = i 损耗 j (i 克 j)
    ///    0.0 = 无关
    pub matrix: [[f64; 5]; 5],
    /// 当前旋钮位置 (你正在观察哪个元)
    pub knob: Yuan,
    /// 时间步计数
    pub tick_count: u64,
}

impl FiveYuan {
    /// 创建五元旋钮，默认所有元强度 = 0.5，关系 = 五行相生相克
    pub fn new() -> Self {
        // 默认关系矩阵 (五行简化模型):
        // A: 生B, 克C
        // B: 生C, 克D
        // C: 生D, 克E
        // D: 生E, 克A
        // E: 生A, 克B
        //
        // 即每个元: 对下一个元 = 生 (+), 对下下个元 = 克 (-)
        let mut matrix = [[0.0f64; 5]; 5];
        for i in 0..5 {
            // 对邻居(+1) = 生
            matrix[i][(i + 1) % 5] = 1.0;
            // 对隔一个(+2) = 克
            matrix[i][(i + 2) % 5] = -1.0;
        }

        FiveYuan {
            strength: [0.5; 5],
            matrix,
            knob: Yuan::A,
            tick_count: 0,
        }
    }

    /// 创建完全空白的五元（无默认关系）
    pub fn empty() -> Self {
        FiveYuan {
            strength: [0.5; 5],
            matrix: [[0.0; 5]; 5],
            knob: Yuan::A,
            tick_count: 0,
        }
    }

    /// 设置元与元之间的单向关系
    pub fn set_relation(&mut self, from: Yuan, to: Yuan, rel: YuanRelation) {
        self.matrix[from.as_index()][to.as_index()] = rel.sign();
    }

    /// 设置元与元之间的双向关系（对称）
    pub fn set_mutual(&mut self, a: Yuan, b: Yuan, rel: YuanRelation) {
        let s = rel.sign();
        self.matrix[a.as_index()][b.as_index()] = s;
        self.matrix[b.as_index()][a.as_index()] = s;
    }

    /// 设置单个元的强度
    pub fn set_strength(&mut self, yuan: Yuan, value: f64) {
        self.strength[yuan.as_index()] = value.clamp(0.0, 1.0);
    }

    /// 旋转旋钮到指定元
    pub fn turn_knob(&mut self, yuan: Yuan) {
        self.knob = yuan;
    }

    /// 顺时针旋转旋钮一步
    pub fn rotate_clockwise(&mut self) {
        self.knob = self.knob.next();
    }

    /// 逆时针旋转旋钮一步
    pub fn rotate_counter(&mut self) {
        self.knob = self.knob.prev();
    }

    /// 读取旋钮当前指向的元的强度
    pub fn read_knob(&self) -> f64 {
        self.strength[self.knob.as_index()]
    }

    /// 获取旋钮当前指向的元的名字
    pub fn knob_name(&self) -> String {
        self.knob.to_string()
    }

    /// 获取所有元的名字和强度
    pub fn all_strength(&self) -> Vec<(String, f64)> {
        Yuan::ALL
            .iter()
            .map(|&y| (y.to_string(), self.strength[y.as_index()]))
            .collect()
    }

    /// 推进一拍: 按关系矩阵，每个元对其它元施加影响
    ///
    /// 能量规则:
    ///   · 基础再生: 每拍每个元 +0.02 (元气自然流动)
    ///   · 相生(+1): 给予方 -0.02, 接收方 +0.02 (能量守恒转移)
    ///   · 相克(-1): 被克方 -0.01, 克人方 +0.01 (夺取而非毁灭)
    ///
    /// 这样设计:
    ///   · 相生 = 给予 (giver 少一点, receiver 多一点)
    ///   · 相克 = 夺取 (defender 损失一部分, attacker 夺取一部分)
    ///   · 基础再生保证系统不会消亡
    ///   · 整体能量守恒 + 再生, 不会无限增长也不会崩溃
    pub fn tick(&mut self) {
        let mut delta = [0.0f64; 5];
        let base_regen = 0.02;       // 基础再生 (自然元气)
        let synergize_transfer = 0.02; // 相生: 转移量
        let conflict_transfer = 0.01;  // 相克: 夺取量

        // 基础再生
        for i in 0..5 {
            delta[i] += base_regen;
        }

        for i in 0..5 {
            for j in 0..5 {
                if i == j {
                    continue;
                }
                let rel = self.matrix[i][j];
                if rel > 0.0 {
                    // i 生 j: i 给 j 能量 (转移, 守恒)
                    delta[i] -= synergize_transfer;
                    delta[j] += synergize_transfer;
                } else if rel < 0.0 {
                    // i 克 j: i 从 j 夺取能量 (不毁灭, 只转移)
                    delta[j] -= conflict_transfer;
                    delta[i] += conflict_transfer;
                }
            }
        }

        for i in 0..5 {
            self.strength[i] = (self.strength[i] + delta[i]).clamp(0.0, 1.0);
        }
        self.tick_count += 1;
    }

    /// 推进 n 拍
    pub fn tick_n(&mut self, n: u64) {
        for _ in 0..n {
            self.tick();
        }
    }

    /// 读取当前关系网络中，与旋钮位的元的关系情况
    pub fn knob_relations(&self) -> Vec<(String, String, f64)> {
        let k = self.knob.as_index();
        let mut result = Vec::new();
        for j in 0..5 {
            if k == j {
                continue;
            }
            let rel = self.matrix[k][j];
            if rel != 0.0 {
                let rel_name = if rel > 0.0 { "生" } else { "克" };
                result.push((
                    self.knob.to_string(),
                    Yuan::from_index(j).to_string(),
                    rel,
                ));
            }
        }
        result
    }

    /// 获取旋钮位的元的"环境评分"
    /// 正值 = 当前环境对旋钮元有利 (被生多)
    /// 负值 = 当前环境对旋钮元有害 (被克多)
    pub fn environment_score(&self) -> f64 {
        let k = self.knob.as_index();
        let mut score = 0.0;
        for i in 0..5 {
            if i == k {
                continue;
            }
            // i 对 k 的关系: 正值 = i 生 k (有益), 负值 = i 克 k (有害)
            score += self.matrix[i][k] * self.strength[i];
        }
        score
    }

    /// 判断当前五元是否处于平衡状态（所有元强度差异 < 阈值）
    pub fn is_balanced(&self, threshold: f64) -> bool {
        let max = self.strength.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = self.strength.iter().cloned().fold(f64::INFINITY, f64::min);
        (max - min).abs() < threshold
    }

    /// 总结当前状态
    pub fn summary(&self) -> String {
        let mut lines = vec![];
        lines.push(format!("=== 五元旋钮 · 第 {} 拍 ===", self.tick_count));
        lines.push(format!("旋钮指向: {}", self.knob));
        lines.push("各元强度:".to_string());
        for i in 0..5 {
            let y = Yuan::from_index(i);
            let s = self.strength[i];
            let bar = "█".repeat((s * 10.0) as usize);
            let empty = "░".repeat(10 - (s * 10.0) as usize);
            let marker = if y == self.knob { " ◀ 旋钮" } else { "" };
            lines.push(format!("  {}: {}{} {:.2}{}", y, bar, empty, s, marker));
        }
        let env = self.environment_score();
        let env_desc = if env > 0.2 {
            "有利"
        } else if env < -0.2 {
            "不利"
        } else {
            "中性"
        };
        lines.push(format!("环境评分: {:.2} ({})", env, env_desc));
        if self.is_balanced(0.15) {
            lines.push("✅ 系统平衡".to_string());
        } else {
            lines.push("⚠️ 系统失衡".to_string());
        }
        lines.join("\n")
    }
}

impl Default for FiveYuan {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_five_yuan() {
        let fy = FiveYuan::new();
        assert_eq!(fy.strength[0], 0.5);
        assert_eq!(fy.knob, Yuan::A);
        assert!(fy.matrix[0][1] > 0.0); // A 生 B
        assert!(fy.matrix[0][2] < 0.0); // A 克 C
    }

    #[test]
    fn test_set_strength() {
        let mut fy = FiveYuan::new();
        fy.set_strength(Yuan::A, 0.9);
        assert_eq!(fy.strength[0], 0.9);
        fy.set_strength(Yuan::A, 1.5); // 超过 1.0 会被 clamp
        assert_eq!(fy.strength[0], 1.0);
    }

    #[test]
    fn test_knob_rotation() {
        let mut fy = FiveYuan::new();
        assert_eq!(fy.knob, Yuan::A);
        fy.rotate_clockwise();
        assert_eq!(fy.knob, Yuan::B);
        fy.rotate_clockwise();
        assert_eq!(fy.knob, Yuan::C);
        fy.rotate_counter();
        assert_eq!(fy.knob, Yuan::B);
    }

    #[test]
    fn test_knob_wrap() {
        let mut fy = FiveYuan::new();
        fy.turn_knob(Yuan::E);
        fy.rotate_clockwise(); // E -> A (wrap)
        assert_eq!(fy.knob, Yuan::A);
        fy.rotate_counter(); // A -> E (wrap)
        assert_eq!(fy.knob, Yuan::E);
    }

    #[test]
    fn test_tick_changes_strength() {
        let mut fy = FiveYuan::new();
        fy.set_strength(Yuan::A, 0.8);
        fy.set_strength(Yuan::C, 0.2);
        fy.tick_n(5);
        // 强度应该发生变化
        assert_ne!(fy.strength[0], 0.8);
        assert_ne!(fy.strength[2], 0.2);
    }

    #[test]
    fn test_environment_score() {
        let mut fy = FiveYuan::new();
        fy.turn_knob(Yuan::A);
        // A 被 E 生 (+), 被 D 克 (-)
        let score = fy.environment_score();
        // 默认所有强度都是 0.5，E->A = +1, D->A = -1
        // score = 0.5*1.0 + 0.5*(-1.0) = 0
        assert!(score.abs() < 0.001);
    }

    #[test]
    fn test_is_balanced() {
        let fy = FiveYuan::new(); // 所有元初始 0.5
        assert!(fy.is_balanced(0.15));
        let mut fy2 = FiveYuan::new();
        fy2.set_strength(Yuan::A, 1.0);
        fy2.set_strength(Yuan::B, 0.0);
        assert!(!fy2.is_balanced(0.15));
    }

    #[test]
    fn test_set_relation() {
        let mut fy = FiveYuan::empty();
        fy.set_relation(Yuan::A, Yuan::B, YuanRelation::Synergize);
        assert!(fy.matrix[0][1] > 0.0);
        assert_eq!(fy.matrix[1][0], 0.0); // 单向
    }

    #[test]
    fn test_set_mutual() {
        let mut fy = FiveYuan::empty();
        fy.set_mutual(Yuan::A, Yuan::B, YuanRelation::Synergize);
        assert!(fy.matrix[0][1] > 0.0);
        assert!(fy.matrix[1][0] > 0.0); // 双向对称
    }

    #[test]
    fn test_summary_format() {
        let fy = FiveYuan::new();
        let s = fy.summary();
        assert!(s.contains("五元旋钮"));
        assert!(s.contains("旋钮指向"));
        assert!(s.contains("各元强度"));
    }

    #[test]
    fn test_tick_n() {
        let mut fy = FiveYuan::new();
        fy.tick_n(10);
        assert_eq!(fy.tick_count, 10);
    }

    #[test]
    fn test_all_strength() {
        let fy = FiveYuan::new();
        let all = fy.all_strength();
        assert_eq!(all.len(), 5);
    }
}

// ══════════════════════════════════════════════════════════════════
// v1.0 五元旋钮运动步量 + 注意力分配
// ══════════════════════════════════════════════════════════════════
//
// step(0~4): 离散档位,对应五个元
// angle(0~360): 连续角度,每元占 72°
//   A: 0°   ~ 72°   (中心 36°)
//   B: 72°  ~ 144°  (中心 108°)
//   C: 144° ~ 216°  (中心 180°)
//   D: 216° ~ 288°  (中心 252°)
//   E: 288° ~ 360°  (中心 324°)

/// 旋钮位置:离散档位 + 连续角度
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KnobPosition {
    pub step: u8,     /* 0~4,对应 Yuan A~E */
    pub angle: f64,   /* 0.0 ~ 360.0 */
}

impl KnobPosition {
    pub const DEG_PER_YUAN: f64 = 72.0; /* 360 / 5 = 72 */

    /// 从档位创建 (自动对齐到该档位的中心角度)
    pub fn from_step(step: u8) -> Self {
        let s = (step % 5) as f64;
        KnobPosition {
            step: step % 5,
            angle: s * Self::DEG_PER_YUAN + Self::DEG_PER_YUAN / 2.0,
        }
    }

    /// 从角度创建 (自动归入最近档位)
    pub fn from_angle(angle: f64) -> Self {
        let normalized = angle.rem_euclid(360.0);
        let step = (normalized / Self::DEG_PER_YUAN).floor() as u8 % 5;
        KnobPosition {
            step,
            angle: normalized,
        }
    }

    /// 获取对应元
    pub fn yuan(&self) -> Yuan {
        Yuan::from_index(self.step as usize)
    }

    /// 顺时针转一步
    pub fn step_forward(&mut self) {
        self.step = (self.step + 1) % 5;
        self.angle = self.step as f64 * Self::DEG_PER_YUAN + Self::DEG_PER_YUAN / 2.0;
    }

    /// 逆时针转一步
    pub fn step_back(&mut self) {
        self.step = (self.step + 4) % 5;
        self.angle = self.step as f64 * Self::DEG_PER_YUAN + Self::DEG_PER_YUAN / 2.0;
    }

    /// 角度微调 (连续运动)
    pub fn nudge(&mut self, delta_deg: f64) {
        self.angle = (self.angle + delta_deg).rem_euclid(360.0);
        self.step = (self.angle / Self::DEG_PER_YUAN).floor() as u8 % 5;
    }

    /// 离最近档位中心的偏移 (-36° ~ +36°)
    pub fn offset_from_center(&self) -> f64 {
        let center = self.step as f64 * Self::DEG_PER_YUAN + Self::DEG_PER_YUAN / 2.0;
        (self.angle - center).rem_euclid(360.0).min(360.0 - (self.angle - center).rem_euclid(360.0))
    }
}

impl Default for KnobPosition {
    fn default() -> Self {
        KnobPosition::from_step(0)
    }
}

// ──────────────────────────────────────────────────────────────
// 注意力分配
// ──────────────────────────────────────────────────────────────

/// 五元注意力分布:每个元占据多少认知资源
#[derive(Clone, Debug)]
pub struct AttentionDist {
    /// 五个元的注意力权重 (sum = 1.0)
    pub weights: [f64; 5],
}

impl AttentionDist {
    pub fn weight(&self, y: Yuan) -> f64 {
        self.weights[y.as_index()]
    }

    /// 注意力最大的元
    pub fn dominant(&self) -> Yuan {
        let mut max_i = 0;
        let mut max_v = 0.0;
        for i in 0..5 {
            if self.weights[i] > max_v {
                max_v = self.weights[i];
                max_i = i;
            }
        }
        Yuan::from_index(max_i)
    }

    /// 注意力熵 (越高越分散,越低越集中)
    pub fn entropy(&self) -> f64 {
        let mut h = 0.0;
        for &w in &self.weights {
            if w > 1e-10 {
                h -= w * w.ln();
            }
        }
        h
    }

    /// 注意力集中度 (1.0 = 完全集中, 0.0 = 均匀分散)
    pub fn concentration(&self) -> f64 {
        let max = self.weights.iter().cloned().fold(0.0f64, f64::max);
        max * 5.0 - 1.0 / 4.0 /* normalize: max=1 → conc=1, max=0.2 → conc=0 */
    }
}

impl AttentionDist {
    /// 五元注意力分配算法
    ///
    /// 输入:
    ///   · fy: 五元旋钮状态
    ///   · knob_pos: 当前旋钮位置 (step+angle)
    ///   · focus: 聚焦系数 (0.0 = 完全分散注意, 1.0 = 完全聚焦旋钮位)
    ///
    /// 算法步骤:
    ///   1. 基础权重 = 元强度 × (1 - focus) / 5  (均匀分量)
    ///   2. 聚焦分量: knob 位额外获得 focus × (1 + synergy_sum - conflict_sum)
    ///   3. 互补分量: 与 knob 位相生的元获得 focus × 0.3 × 关系强度
    ///   4. 互害分量: 与 knob 位相克的元失去 focus × 0.2 × 关系强度
    ///   5. 角度影响: 偏移旋钮中心越远,聚焦效果越弱
    ///   6. 归一化到 sum = 1.0
    pub fn compute(fy: &FiveYuan, knob_pos: &KnobPosition, focus: f64) -> Self {
        let focus = focus.clamp(0.0, 1.0);
        let knob_idx = knob_pos.yuan().as_index();
        let mut raw = [0.0f64; 5];

        // 1. 基础权重 (均匀分量,所有元平等)
        let base_uniform = (1.0 - focus) / 5.0;
        for i in 0..5 {
            raw[i] = base_uniform;
        }

        // 2. 聚焦分量 (旋钮位元获得聚焦加成)
        let knob_strength = fy.strength[knob_idx];
        let env = fy.environment_score(); /* -4 ~ +4 */
        let env_factor = 1.0 + env.tanh() * 0.5; /* 有利环境提升聚焦,不利环境削弱 */
        let angle_atten = 1.0 - knob_pos.offset_from_center() / 72.0; /* 偏移中心越远越弱 */
        let focus_bonus = focus * knob_strength * env_factor * angle_atten;
        raw[knob_idx] += focus_bonus;

        // 3. 互补分量 (与 knob 位相生的元获得溢出注意力)
        for j in 0..5 {
            if j == knob_idx { continue; }
            let relation = fy.matrix[knob_idx][j];
            if relation > 0.0 {
                raw[j] += focus * 0.3 * relation * fy.strength[j];
            }
        }

        // 4. 互害分量 (与 knob 位相克的元被抑制)
        for j in 0..5 {
            if j == knob_idx { continue; }
            let relation = fy.matrix[knob_idx][j];
            if relation < 0.0 {
                raw[j] = (raw[j] - focus * 0.2 * relation.abs()).max(0.0);
            }
        }

        // 5. 归一化
        let sum: f64 = raw.iter().sum();
        let weights = if sum > 1e-10 {
            let mut w = [0.0f64; 5];
            for i in 0..5 {
                w[i] = raw[i] / sum;
            }
            w
        } else {
            [0.2; 5]
        };

        AttentionDist { weights }
    }

    /// 均匀注意力 (所有元相等)
    pub fn uniform() -> Self {
        AttentionDist {
            weights: [0.2; 5],
        }
    }

    /// 全聚焦 (100% 注意力在 knob 位)
    pub fn full_focus(knob: Yuan) -> Self {
        let mut w = [0.0; 5];
        w[knob.as_index()] = 1.0;
        AttentionDist { weights: w }
    }
}

impl fmt::Display for AttentionDist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        for i in 0..5 {
            let y = Yuan::from_index(i);
            parts.push(format!("{}:{:.1}%%", y, self.weights[i] * 100.0));
        }
        write!(f, "注意力[{}]", parts.join(" "))
    }
}

// ──────────────────────────────────────────────────────────────
// FiveYuan v1.0 扩展方法
// ──────────────────────────────────────────────────────────────

impl FiveYuan {
    /// 带步量位置的五元 (v1.0)
    pub fn new_v1() -> (Self, KnobPosition) {
        (Self::new(), KnobPosition::default())
    }

    /// 计算当前状态下的注意力分布
    pub fn attention(&self, knob_pos: &KnobPosition, focus: f64) -> AttentionDist {
        AttentionDist::compute(self, knob_pos, focus)
    }

    /// 带步量推进 (推进后自动调整角度)
    pub fn tick_with_knob(&mut self, knob: &mut KnobPosition, focus: f64) {
        self.tick();
        // 根据环境评分微调角度 (有利 → 向中心靠拢; 不利 → 外移)
        let env = self.environment_score();
        let env_angle_drift = env * 2.0; /* 每拍最多漂移 ±2° */
        knob.nudge(env_angle_drift * focus);
    }

    /// 采样 n 拍并返回每一步的注意力轨迹
    pub fn simulate_attention(
        &mut self,
        knob: &mut KnobPosition,
        focus: f64,
        steps: u64,
    ) -> Vec<AttentionDist> {
        let mut trace = Vec::with_capacity(steps as usize);
        for _ in 0..steps {
            self.tick_with_knob(knob, focus);
            trace.push(self.attention(knob, focus));
        }
        trace
    }
}

// ──────────────────────────────────────────────────────────────
// v1.0 测试
// ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod v1_tests {
    use super::*;

    #[test]
    fn test_knob_position_from_step() {
        let kp = KnobPosition::from_step(0);
        assert_eq!(kp.step, 0);
        assert!((kp.angle - 36.0).abs() < 0.01);
        assert_eq!(kp.yuan(), Yuan::A);

        let kp = KnobPosition::from_step(2);
        assert_eq!(kp.step, 2);
        assert!((kp.angle - 180.0).abs() < 0.01);
        assert_eq!(kp.yuan(), Yuan::C);
    }

    #[test]
    fn test_knob_position_from_angle() {
        let kp = KnobPosition::from_angle(0.0);
        assert_eq!(kp.step, 0); /* 0° → A档 */

        let kp = KnobPosition::from_angle(100.0);
        assert_eq!(kp.step, 1); /* 72~144° → B档 */

        let kp = KnobPosition::from_angle(350.0);
        assert_eq!(kp.step, 4); /* 288~360° → E档 */
    }

    #[test]
    fn test_knob_angle_wrap() {
        let mut kp = KnobPosition::from_angle(350.0);
        kp.nudge(20.0); /* 350 + 20 = 370 → 10° */
        assert!(kp.angle < 15.0);
        assert_eq!(kp.step, 0);
    }

    #[test]
    fn test_knob_step_forward() {
        let mut kp = KnobPosition::from_step(0);
        kp.step_forward();
        assert_eq!(kp.step, 1);
        assert!((kp.angle - 108.0).abs() < 0.01);
        kp.step_forward();
        assert_eq!(kp.step, 2);
    }

    #[test]
    fn test_knob_step_wrap() {
        let mut kp = KnobPosition::from_step(4);
        kp.step_forward(); /* E → A */
        assert_eq!(kp.step, 0);
        kp.step_back(); /* A → E */
        assert_eq!(kp.step, 4);
    }

    #[test]
    fn test_knob_offset() {
        let mut kp = KnobPosition::from_step(0);
        assert!(kp.offset_from_center() < 0.01); /* 在中心 */
        kp.nudge(30.0); /* 偏移 30° */
        assert!((kp.offset_from_center() - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_attention_uniform() {
        let ad = AttentionDist::uniform();
        for w in &ad.weights {
            assert!((w - 0.2).abs() < 0.001);
        }
        assert_eq!(ad.dominant(), Yuan::A); /* 全相等,取第一个 */
    }

    #[test]
    fn test_attention_full_focus() {
        let ad = AttentionDist::full_focus(Yuan::C);
        assert!((ad.weight(Yuan::C) - 1.0).abs() < 0.001);
        assert_eq!(ad.dominant(), Yuan::C);
    }

    #[test]
    fn test_attention_compute() {
        let fy = FiveYuan::new();
        let knob = KnobPosition::from_step(0); /* 指向 A */
        let ad = AttentionDist::compute(&fy, &knob, 0.5);

        // 权重和 = 1.0
        let sum: f64 = ad.weights.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);

        // 旋钮位的权重应高于平均值 (0.2)
        assert!(ad.weight(Yuan::A) > 0.2);
    }

    #[test]
    fn test_attention_focus_influence() {
        let fy = FiveYuan::new();
        let knob = KnobPosition::from_step(2); /* 指向 C */

        let ad_low = AttentionDist::compute(&fy, &knob, 0.0);
        let ad_high = AttentionDist::compute(&fy, &knob, 1.0);

        // 高聚焦时旋钮位权重应更高
        assert!(ad_high.weight(Yuan::C) > ad_low.weight(Yuan::C));
    }

    #[test]
    fn test_attention_synergy_boost() {
        /* A 生 B,聚焦 A 时 B 应该获得溢出注意力 */
        let fy = FiveYuan::new();
        let knob = KnobPosition::from_step(0); /* A */
        let ad = AttentionDist::compute(&fy, &knob, 0.8);

        assert!(ad.weight(Yuan::B) > 0.2); /* B 应高于均匀值 */
    }

    #[test]
    fn test_attention_conflict_suppression() {
        /* A 克 C,聚焦 A 时 C 应该被抑制 */
        let fy = FiveYuan::new();
        let knob = KnobPosition::from_step(0); /* A */
        let ad = AttentionDist::compute(&fy, &knob, 0.8);

        assert!(ad.weight(Yuan::C) < 0.2); /* C 应低于均匀值 */
    }

    #[test]
    fn test_attention_dominant() {
        let ad = AttentionDist::full_focus(Yuan::D);
        assert_eq!(ad.dominant(), Yuan::D);
    }

    #[test]
    fn test_attention_entropy() {
        let ad_uniform = AttentionDist::uniform();
        let ad_focused = AttentionDist::full_focus(Yuan::A);

        // 均匀分布的熵应高于完全聚焦
        assert!(ad_uniform.entropy() > ad_focused.entropy());
    }

    #[test]
    fn test_tick_with_knob() {
        let mut fy = FiveYuan::new();
        let mut knob = KnobPosition::from_step(0);
        fy.tick_with_knob(&mut knob, 0.5);
        assert_eq!(fy.tick_count, 1);
    }

    #[test]
    fn test_simulate_attention() {
        let mut fy = FiveYuan::new();
        let mut knob = KnobPosition::from_step(0);
        let trace = fy.simulate_attention(&mut knob, 0.6, 20);

        assert_eq!(trace.len(), 20);
        // 最终节拍数 = 20
        assert_eq!(fy.tick_count, 20);

        // 每一步注意力权重和都应为 1.0
        for ad in &trace {
            let sum: f64 = ad.weights.iter().sum();
            assert!((sum - 1.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_new_v1() {
        let (fy, knob) = FiveYuan::new_v1();
        assert_eq!(knob.step, 0);
        assert_eq!(fy.knob, Yuan::A);
    }

    #[test]
    fn test_attention_display() {
        let ad = AttentionDist::uniform();
        let s = format!("{}", ad);
        assert!(s.contains("注意力"));
        assert!(s.contains("A:"));
    }

    #[test]
    fn test_knob_nudge_continuous() {
        let mut kp = KnobPosition::from_step(0);
        kp.nudge(10.0);
        assert!((kp.angle - 46.0).abs() < 0.01);
        kp.nudge(30.0);
        assert!((kp.angle - 76.0).abs() < 0.01);
    }
}
