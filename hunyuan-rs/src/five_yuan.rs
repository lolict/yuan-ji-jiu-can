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
