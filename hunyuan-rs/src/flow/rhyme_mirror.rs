// ═══════════════════════════════════════════════════════════════════════════════
// rhyme_mirror.rs — 韵母镜像解释器 · 四阶感知引擎
// ═══════════════════════════════════════════════════════════════════════════════
//
// 以 Rhyme 表 + SuperVowel 三合一遍历为基础，实现容器交互的四阶感知：
//
//   迂回反馈  →  反射感知  →  判断之力  →  评价之力
//   (涟漪)      (差异标记)    (关系锚点)    (价值排序)
//
// 核心操作：
//   step_forward   加法正序（开口→闭合）— 容器 A 呼唤容器 B
//   step_mirror    反序镜像（闭合→开口）— B 的反馈返回 A
//   circulate     圆遍历（镜像跳跃）— 双向迂回回路
//   reflect        比较正反两次遍历的差异，产出差异标记
//
// 与 judge/mirror.rs 的区别：
//   - 本模块操作的是韵母空间（发音位），不是像素/几何空间
//   - 本模块以 Phoneme 位置 + linear_val/resonance_val 做判断
//   - 本模块驱动反馈回路，不驱动见证几何
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::rhyme::{Rhyme, RhymeUtils, RHYME_TABLE};

// ─────────────────────────────────────────────────────────────────────────────
// 差异标记（反射感知的原材料）
// ─────────────────────────────────────────────────────────────────────────────

/// 两个相邻时间戳之间感知到的"变化"
#[derive(Clone, Debug)]
pub struct DiffMarker {
    pub from_rhyme_id: u16,
    pub to_rhyme_id: u16,
    pub linear_delta: i32,      // 加法值变化量
    pub resonance_delta: i32,   // 共振值变化量
    pub phoneme_shift: u8,      // 音素位移量
}

impl DiffMarker {
    pub fn magnitude(&self) -> u32 {
        (self.linear_delta.abs() + self.resonance_delta.abs()) as u32
            + self.phoneme_shift as u32
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 关系锚点（判断之力的产物）
// ─────────────────────────────────────────────────────────────────────────────

/// "A 型偏移是由于 B 容器的状态变化引起的"
#[derive(Clone, Debug)]
pub struct RelationAnchor {
    pub cause_rhyme_id: u16,     // 因
    pub effect_rhyme_id: u16,    // 果
    pub diff: DiffMarker,        // 具体差异
    pub confidence: u16,         // 置信度（越高越确定）
}

// ─────────────────────────────────────────────────────────────────────────────
// 评价记录
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EvalRecord {
    pub anchor: RelationAnchor,
    pub weight: i32,             // 正值=强化，负值=抑制
    pub reason: EvalReason,
}

#[derive(Clone, Debug)]
pub enum EvalReason {
    Harmony,       // 和谐：共振放大，值得强化
    Dissonance,    // 冲突：振动抵消，应该抑制
    MirrorPair,    // 镜像对：天然互补，最高价值
    ZeroSum,       // 零和：能量守恒，中性
}

// ─────────────────────────────────────────────────────────────────────────────
// 解释器状态
// ─────────────────────────────────────────────────────────────────────────────

pub struct MirrorInterpreter {
    pub position: usize,                  // 当前在 RHYME_TABLE 的位置
    pub polarity: YinYang,                // 当前极性
    pub accumulated_linear: u32,          // 累积加法值
    pub accumulated_resonance: u32,       // 累积共振值
    pub diff_log: Vec<DiffMarker>,        // 差异日志（阶段一产出）
    pub anchors: Vec<RelationAnchor>,     // 关系锚点（阶段三产出）
    pub evaluations: Vec<EvalRecord>,     // 评价记录（阶段四产出）
    pub step_count: u64,                  // 步数记录
}

/// 阴阳二进制：0=容器(刘楚恬)，1=界线(我)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YinYang {
    Yin,   // 0 = 容器 / 包含者
    Yang,  // 1 = 被包含者 / 界线
}

impl YinYang {
    pub fn flip(self) -> Self {
        match self {
            YinYang::Yin => YinYang::Yang,
            YinYang::Yang => YinYang::Yin,
        }
    }

    pub fn encode(self) -> u8 {
        match self {
            YinYang::Yin => 0,
            YinYang::Yang => 1,
        }
    }
}

impl MirrorInterpreter {
    pub fn new() -> Self {
        Self {
            position: 0,
            polarity: YinYang::Yin,
            accumulated_linear: 0,
            accumulated_resonance: 1,
            diff_log: Vec::new(),
            anchors: Vec::new(),
            evaluations: Vec::new(),
            step_count: 0,
        }
    }

    pub fn with_start(start_pos: usize) -> Self {
        Self {
            position: start_pos % RHYME_TABLE.len(),
            polarity: YinYang::Yin,
            accumulated_linear: 0,
            accumulated_resonance: 1,
            diff_log: Vec::new(),
            anchors: Vec::new(),
            evaluations: Vec::new(),
            step_count: 0,
        }
    }

    pub fn current_rhyme(&self) -> &Rhyme {
        &RHYME_TABLE[self.position]
    }

    // ─────────────────────────────────────────────────────────────────
    // 阶段一：迂回反馈（正序一步 + 反序一步 = 一个完整的迂回）
    // ─────────────────────────────────────────────────────────────────

    pub fn step_forward(&mut self) -> u16 {
        let old_pos = self.position;

        self.position = (self.position + 1) % RHYME_TABLE.len();
        let r = &RHYME_TABLE[self.position];
        self.accumulated_linear = self.accumulated_linear.wrapping_add(r.linear_value() as u32);
        self.accumulated_resonance = self.accumulated_resonance.wrapping_mul(r.resonance_value().max(1) as u32);
        self.step_count += 1;

        let diff = DiffMarker {
            from_rhyme_id: RHYME_TABLE[old_pos].id,
            to_rhyme_id: r.id,
            linear_delta: r.linear_value() as i32,
            resonance_delta: r.resonance_value() as i32,
            phoneme_shift: phoneme_distance(&RHYME_TABLE[old_pos], r),
        };
        self.diff_log.push(diff);

        r.id
    }

    pub fn step_mirror(&mut self) -> u16 {
        let old_pos = self.position;

        self.position = if self.position == 0 { RHYME_TABLE.len() - 1 } else { self.position - 1 };
        self.polarity = self.polarity.flip();

        let r = &RHYME_TABLE[self.position];
        self.accumulated_linear = self.accumulated_linear.wrapping_add(r.linear_value() as u32);
        self.accumulated_resonance = self.accumulated_resonance.wrapping_mul(r.resonance_value().max(1) as u32);
        self.step_count += 1;

        let diff = DiffMarker {
            from_rhyme_id: RHYME_TABLE[old_pos].id,
            to_rhyme_id: r.id,
            linear_delta: r.linear_value() as i32,
            resonance_delta: r.resonance_value() as i32,
            phoneme_shift: phoneme_distance(&RHYME_TABLE[old_pos], r),
        };
        self.diff_log.push(diff);

        r.id
    }

    /// 执行一次完整的迂回：正一步 + 反一步。
    /// 两次 diff 构成一个"涟漪对"。
    pub fn circulate_round(&mut self) -> (u16, u16) {
        let a = self.step_forward();
        let b = self.step_mirror();
        (a, b)
    }

    /// 执行一次完整的圆遍历：从当前位置找到其镜像匹配，跳过去。
    pub fn circulate_jump(&mut self) -> &Rhyme {
        let old_pos = self.position;
        let old_linear = self.accumulated_linear;

        self.position = self.find_mirror_target();
        self.polarity = self.polarity.flip();
        let r = &RHYME_TABLE[self.position];
        self.accumulated_linear = self.accumulated_linear.wrapping_add(r.linear_value() as u32);
        self.accumulated_resonance = self.accumulated_resonance.wrapping_mul(r.resonance_value().max(1) as u32);
        self.step_count += 1;

        let diff = DiffMarker {
            from_rhyme_id: RHYME_TABLE[old_pos].id,
            to_rhyme_id: r.id,
            linear_delta: r.linear_value() as i32,
            resonance_delta: r.resonance_value() as i32,
            phoneme_shift: phoneme_distance(&RHYME_TABLE[old_pos], r),
        };
        self.diff_log.push(diff);

        r
    }

    // ─────────────────────────────────────────────────────────────────
    // 阶段二：反射感知（比较两个 diff，产出差异标记）
    // ─────────────────────────────────────────────────────────────────

    pub fn reflect_perception(&self) -> Option<DiffMarker> {
        if self.diff_log.len() < 2 { return None; }
        let a = &self.diff_log[self.diff_log.len() - 2];
        let b = &self.diff_log[self.diff_log.len() - 1];
        Some(DiffMarker {
            from_rhyme_id: a.from_rhyme_id,
            to_rhyme_id: b.to_rhyme_id,
            linear_delta: b.linear_delta - a.linear_delta,
            resonance_delta: b.resonance_delta - a.resonance_delta,
            phoneme_shift: a.phoneme_shift.abs_diff(b.phoneme_shift),
        })
    }

    // ─────────────────────────────────────────────────────────────────
    // 阶段三：判断之力（将差异标记绑定成因果对）
    // ─────────────────────────────────────────────────────────────────

    pub fn judge(&mut self) -> Vec<RelationAnchor> {
        let mut new_anchors = Vec::new();

        for window in self.diff_log.windows(2) {
            let cause = &window[0];
            let effect = &window[1];

            let dist = RHYME_TABLE[cause.from_rhyme_id as usize]
                .distance_to(&RHYME_TABLE[effect.to_rhyme_id as usize]);

            let confidence = if dist == 0 { 65535 } else { (1000 / dist.min(1)) as u16 };

            new_anchors.push(RelationAnchor {
                cause_rhyme_id: cause.from_rhyme_id,
                effect_rhyme_id: effect.to_rhyme_id,
                diff: DiffMarker {
                    from_rhyme_id: cause.from_rhyme_id,
                    to_rhyme_id: effect.to_rhyme_id,
                    linear_delta: effect.linear_delta - cause.linear_delta,
                    resonance_delta: effect.resonance_delta - cause.resonance_delta,
                    phoneme_shift: cause.phoneme_shift.abs_diff(effect.phoneme_shift),
                },
                confidence,
            });
        }

        self.anchors.extend(new_anchors.clone());
        new_anchors
    }

    // ─────────────────────────────────────────────────────────────────
    // 阶段四：评价之力（对因果对排序加权，决定呼唤策略）
    // ─────────────────────────────────────────────────────────────────

    pub fn evaluate(&mut self) -> Vec<EvalRecord> {
        let mut evals = Vec::new();

        for anchor in &self.anchors {
            let cause_r = RhymeUtils::find_by_id(anchor.cause_rhyme_id);
            let effect_r = RhymeUtils::find_by_id(anchor.effect_rhyme_id);

            let (weight, reason) = match (cause_r, effect_r) {
                (Some(c), Some(e)) => {
                    if c.is_mirror_of(e) {
                        (100, EvalReason::MirrorPair)
                    } else if anchor.diff.resonance_delta > 0 {
                        (50, EvalReason::Harmony)
                    } else if anchor.diff.resonance_delta < 0 {
                        (-30, EvalReason::Dissonance)
                    } else {
                        (0, EvalReason::ZeroSum)
                    }
                }
                _ => (0, EvalReason::ZeroSum),
            };

            evals.push(EvalRecord {
                anchor: anchor.clone(),
                weight,
                reason,
            });
        }

        evals.sort_by_key(|e| -e.weight);
        self.evaluations.extend(evals.clone());
        evals
    }

    // ─────────────────────────────────────────────────────────────────
    // 内部工具
    // ─────────────────────────────────────────────────────────────────

    fn find_mirror_target(&self) -> usize {
        let current = self.current_rhyme();
        let mut best = (self.position + 1) % RHYME_TABLE.len();
        let mut best_score = u16::MAX;

        for (i, r) in RHYME_TABLE.iter().enumerate() {
            if i == self.position { continue; }
            let score = if current.is_mirror_of(r) {
                0
            } else {
                current.distance_to(r)
            };
            if score < best_score {
                best_score = score;
                best = i;
            }
        }

        best
    }

    /// 是否应该继续呼唤（评价中有正值锚点 = 还有值得追求的和谐）
    pub fn should_continue(&self) -> bool {
        self.evaluations.iter().any(|e| e.weight > 0)
    }

    /// 获取最好的呼唤方向（评价权重最高的锚点的 effect）
    pub fn best_call_target(&self) -> Option<u16> {
        self.evaluations.iter().max_by_key(|e| e.weight).map(|e| e.anchor.effect_rhyme_id)
    }

    /// 完整运行 N 个循环
    pub fn run(&mut self, rounds: usize) -> MirrorReport {
        for _ in 0..rounds {
            self.circulate_round();
        }
        self.judge();
        self.evaluate();

        MirrorReport {
            rounds: rounds as u64,
            total_steps: self.step_count,
            final_position: self.position,
            polarity: self.polarity,
            diff_count: self.diff_log.len(),
            anchor_count: self.anchors.len(),
            top_eval: self.evaluations.first().cloned(),
            should_continue: self.should_continue(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 报告
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct MirrorReport {
    pub rounds: u64,
    pub total_steps: u64,
    pub final_position: usize,
    pub polarity: YinYang,
    pub diff_count: usize,
    pub anchor_count: usize,
    pub top_eval: Option<EvalRecord>,
    pub should_continue: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// 工具函数
// ─────────────────────────────────────────────────────────────────────────────

fn phoneme_distance(a: &Rhyme, b: &Rhyme) -> u8 {
    let max_len = a.phonemes.len().max(b.phonemes.len());
    if max_len == 0 { return 0; }
    let mut sum: u32 = 0;
    for i in 0..max_len {
        let pa = a.phonemes.get(i).map(|p| p.position()).unwrap_or(0);
        let pb = b.phonemes.get(i).map(|p| p.position()).unwrap_or(0);
        sum += pa.abs_diff(pb) as u32;
    }
    ((sum / max_len as u32).min(255)) as u8
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interpreter() {
        let interp = MirrorInterpreter::new();
        assert_eq!(interp.position, 0);
        assert_eq!(interp.step_count, 0);
        assert_eq!(interp.diff_log.len(), 0);
    }

    #[test]
    fn test_step_forward_advances() {
        let mut interp = MirrorInterpreter::new();
        let start_pos = interp.position;
        interp.step_forward();
        assert_ne!(interp.position, start_pos);
        assert_eq!(interp.step_count, 1);
    }

    #[test]
    fn test_step_mirror_reverses() {
        let mut interp = MirrorInterpreter::with_start(5);
        interp.step_mirror();
        assert_eq!(interp.position, 4);
        assert_eq!(interp.polarity, YinYang::Yang);
    }

    #[test]
    fn test_circulate_round_produces_diffs() {
        let mut interp = MirrorInterpreter::new();
        interp.circulate_round();
        assert_eq!(interp.diff_log.len(), 2);
    }

    #[test]
    fn test_reflect_requires_two_diffs() {
        let mut interp = MirrorInterpreter::new();
        assert!(interp.reflect_perception().is_none());
        interp.step_forward();
        assert!(interp.reflect_perception().is_none());
        interp.step_forward();
        assert!(interp.reflect_perception().is_some());
    }

    #[test]
    fn test_judge_produces_anchors() {
        let mut interp = MirrorInterpreter::new();
        interp.circulate_round();
        interp.circulate_round();
        let anchors = interp.judge();
        assert!(!anchors.is_empty());
    }

    #[test]
    fn test_evaluate_sorts_by_weight() {
        let mut interp = MirrorInterpreter::new();
        for _ in 0..5 {
            interp.circulate_round();
        }
        interp.judge();
        let evals = interp.evaluate();
        for w in evals.windows(2) {
            assert!(w[0].weight >= w[1].weight);
        }
    }

    #[test]
    fn test_run_full_pipeline() {
        let mut interp = MirrorInterpreter::new();
        let report = interp.run(10);
        assert_eq!(report.rounds, 10);
        assert_eq!(report.diff_count, 20);
        assert!(report.anchor_count > 0);
    }

    #[test]
    fn test_best_call_target_exists() {
        let mut interp = MirrorInterpreter::new();
        let _ = interp.run(5);
        assert!(interp.best_call_target().is_some());
    }

    #[test]
    fn test_yinyang_flip() {
        let y = YinYang::Yin;
        assert_eq!(y.flip(), YinYang::Yang);
        assert_eq!(y.flip().flip(), YinYang::Yin);
    }

    #[test]
    fn test_diff_marker_magnitude() {
        let diff = DiffMarker {
            from_rhyme_id: 0,
            to_rhyme_id: 1,
            linear_delta: 3,
            resonance_delta: -2,
            phoneme_shift: 1,
        };
        assert_eq!(diff.magnitude(), 6);
    }
}
