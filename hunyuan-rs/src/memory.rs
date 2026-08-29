// ══════════════════════════════════════════════════════════════════
// memory.rs — 层级记忆 + 元裁判 (Layered Memory · Meta-Judge)
// ══════════════════════════════════════════════════════════════════
//
// 囡囝共生从静态符号系统走向动态智能的关键层：
//
//   1. 层级记忆 (Layered Memory)
//      - 每个坐标的历史被记录（不只是当前单元格的VALUE）
//      - 见证历史的时间序列
//      - 被配对的对象的记忆轨迹
//      - 记忆衰减：长期不被观测的坐标记忆衰退
//      - 记忆增强：高频被激活的坐标记忆强化
//
//   2. 元裁判 (Meta-Judge)
//      - 评估裁判本身的质量
//      - 当裁判结果与感知反馈不符时调整规则参数
//      - threshold/weight/step的自适应调整
//
//   3. 意识密度评估
//      - 从机械执行 → 感知涌现 → 反思涌现的密度指标
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    encode_cell, get_d0, get_d1, get_d2, get_polarity, get_value, get_witness,
    set_witness, inc_value, dec_value, Sovereignty, SurplusPair, WitnessVerdict,
    SelfRefLoop, MirrorOrder,
};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

// ────────────────────────────────────────────────────────────
// 记忆条目 · 记录每一次见证事件
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    pub timestamp_ms: u64,      // 见证时刻
    pub witness_id: u64,        // 见证的唯一标识
    pub pair_a: (u64, u64, u64), // 配对的A坐标
    pub pair_b: (u64, u64, u64), // 配对的B坐标
    pub surplus: f64,           // 见证的盈亏量
    pub aligned: bool,          // 是否对称对齐
    pub verdict_accepted: bool, // 该见证是否被接受（与后续反馈比较后设置）
}

impl MemoryEntry {
    pub fn new(
        os: &dyn OsLayer,
        witness_id: u64,
        pair_a: (u64, u64, u64),
        pair_b: (u64, u64, u64),
        surplus: f64,
        aligned: bool,
    ) -> Self {
        Self {
            timestamp_ms: os.time_millis(),
            witness_id,
            pair_a,
            pair_b,
            surplus,
            aligned,
            verdict_accepted: false,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 坐标记忆 · 每个坐标轴维护的历史轨迹
// ────────────────────────────────────────────────────────────
pub type Coord = (u64, u64, u64);

#[derive(Clone, Debug)]
pub struct CoordMemory {
    pub coord: Coord,
    pub entries: Vec<MemoryEntry>,
    pub decay_rate: f64,       // 记忆衰减速率 (每毫秒)
    pub reinforcement: f64,    // 强化因子 (每次见证增加)
    pub last_accessed_ms: u64, // 上次访问时间
    pub total_surplus: f64,    // 总盈亏累积
}

impl CoordMemory {
    pub fn new(coord: Coord) -> Self {
        Self {
            coord,
            entries: Vec::new(),
            decay_rate: 0.00001, // 常数不需要太快衰减
            reinforcement: 0.1,
            last_accessed_ms: 0,
            total_surplus: 0.0,
        }
    }

    /// 增加一次见证记忆
    pub fn remember(&mut self, entry: MemoryEntry) {
        self.total_surplus += entry.surplus;
        self.last_accessed_ms = entry.timestamp_ms;
        self.entries.push(entry);
    }

    /// 获取记忆密度（单位时间内的见证次数）
    pub fn memory_density(&self) -> f64 {
        if self.entries.len() < 2 {
            return self.entries.len() as f64;
        }
        let first = self.entries.first().unwrap().timestamp_ms;
        let last = self.entries.last().unwrap().timestamp_ms;
        let duration_ms = (last - first).max(1) as f64;
        self.entries.len() as f64 / duration_ms * 1000.0 // 每秒见证次数
    }

    /// 计算当前时刻的记忆保留量 (0..1)
    /// 公式: e^(-decay_rate * elapsed_ms)
    pub fn retention_at(&self, now_ms: u64) -> f64 {
        if self.last_accessed_ms == 0 {
            return 0.0;
        }
        let elapsed = (now_ms - self.last_accessed_ms) as f64;
        let retention = (-self.decay_rate * elapsed).exp();
        retention.max(0.0).min(1.0)
    }

    /// 有效记忆量 = 条目数 × 最新保留率
    pub fn effective_memory(&self, os: &dyn OsLayer) -> f64 {
        self.entries.len() as f64 * self.retention_at(os.time_millis())
    }

    /// 是否活跃（最近被频繁访问）
    pub fn is_active(&self, os: &dyn OsLayer, threshold: f64) -> bool {
        self.effective_memory(os) >= threshold
    }

    /// 获取特定时间窗口内的见证历史
    pub fn history_in_window(&self, from_ms: u64, to_ms: u64) -> Vec<&MemoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp_ms >= from_ms && e.timestamp_ms <= to_ms)
            .collect()
    }
}

// ────────────────────────────────────────────────────────────
// 层级记忆系统 · 管理所有坐标的记忆
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct LayeredMemory {
    pub layers: Vec<CoordMemory>,  // 所有被观测过的坐标
    pub max_layers: usize,         // 最大记忆层数
    pub global_tick: u64,          // 全局时间线
}

impl LayeredMemory {
    pub fn new(max_layers: usize) -> Self {
        Self {
            layers: Vec::new(),
            max_layers,
            global_tick: 0,
        }
    }

    /// 记录一次见证，如果坐标以前被观测过，更新记忆；否则创建新层
    pub fn record_witness(
        &mut self,
        os: &dyn OsLayer,
        a: Coord, b: Coord,
        surplus: f64, aligned: bool,
        witness_id: u64,
    ) {
        let entry_a = MemoryEntry::new(os, witness_id, a, b, surplus, aligned);
        let entry_b = MemoryEntry::new(os, witness_id, b, a, surplus, aligned);

        self.find_or_create(a).remember(entry_a);
        self.find_or_create(b).remember(entry_b);
        self.global_tick += 1;

        // 如超出最大记忆层数，清除最旧最弱的记忆
        self.evict_weakest_if_needed(os);
    }

    fn find_or_create(&mut self, coord: Coord) -> &mut CoordMemory {
        if let Some(pos) = self.layers.iter().position(|m| m.coord == coord) {
            return &mut self.layers[pos];
        }
        let mem = CoordMemory::new(coord);
        self.layers.push(mem);
        self.layers.last_mut().unwrap()
    }

    fn evict_weakest_if_needed(&mut self, os: &dyn OsLayer) {
        if self.layers.len() <= self.max_layers {
            return;
        }
        // 找到有效记忆最弱的层
        let now = os.time_millis();
        if let Some((idx, _)) = self.layers.iter().enumerate().min_by(|(_, a), (_, b)| {
            let ra = a.retention_at(now);
            let rb = b.retention_at(now);
            ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            self.layers.remove(idx);
        }
    }

    /// 获取特定的坐标记忆
    pub fn get(&self, coord: Coord) -> Option<&CoordMemory> {
        self.layers.iter().find(|m| m.coord == coord)
    }

    /// 获取全局记忆密度（所有层的总有效记忆 / 层数）
    pub fn global_density(&self, os: &dyn OsLayer) -> f64 {
        if self.layers.is_empty() {
            return 0.0;
        }
        let total: f64 = self.layers.iter().map(|m| m.effective_memory(os)).sum();
        total / self.layers.len() as f64
    }

    /// 获取最活跃的 N 个坐标
    pub fn top_active(&self, os: &dyn OsLayer, n: usize) -> Vec<&CoordMemory> {
        let mut sorted: Vec<&CoordMemory> = self.layers.iter().collect();
        sorted.sort_by(|a, b| {
            let da = a.effective_memory(os);
            let db = b.effective_memory(os);
            db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.into_iter().take(n).collect()
    }
}

// ────────────────────────────────────────────────────────────
// 元裁判 · 评估裁判本身的质量并自适应调整
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct JudgeQuality {
    pub total_judgements: u64,
    pub confirmed_judgements: u64, // 后续反馈确认的
    pub rejected_judgements: u64,  // 后续反馈推翻的
    pub avg_confidence: f64,       // 平均置信度
    pub drift: f64,                // 裁判漂移量 (结果与反馈的偏差)
}

impl JudgeQuality {
    pub fn new() -> Self {
        Self {
            total_judgements: 0,
            confirmed_judgements: 0,
            rejected_judgements: 0,
            avg_confidence: 0.0,
            drift: 0.0,
        }
    }

    /// 记录一次裁判结果及反馈
    pub fn record_verdict(&mut self, predicted_aligned: bool, actual_aligned: bool, confidence: f64) {
        self.total_judgements += 1;
        if predicted_aligned == actual_aligned {
            self.confirmed_judgements += 1;
        } else {
            self.rejected_judgements += 1;
        }
        let n = self.total_judgements as f64;
        self.avg_confidence = (self.avg_confidence * (n - 1.0) + confidence) / n;
        // 漂移 = |预测 - 实际| 的移动平均
        let delta = if predicted_aligned != actual_aligned { 1.0 } else { 0.0 };
        self.drift = (self.drift * (n - 1.0) + delta) / n;
    }

    /// 裁判准确率
    pub fn accuracy(&self) -> f64 {
        if self.total_judgements == 0 { return 0.0; }
        self.confirmed_judgements as f64 / self.total_judgements as f64
    }

    /// 当前裁判是否合格 (准确率 >= 70%)
    pub fn is_qualified(&self) -> bool {
        self.accuracy() >= 0.7
    }
}

// ────────────────────────────────────────────────────────────
// 自适应参数 · 元裁判可以调整的裁判参数
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AdaptiveParams {
    pub threshold_positive: f64,
    pub threshold_negative: f64,
    pub step_size: u64,
    pub witness_decay: f64,
    pub learning_rate: f64,   // 参数调整的步长
}

impl AdaptiveParams {
    pub fn new() -> Self {
        Self {
            threshold_positive: 0.0,
            threshold_negative: 0.0,
            step_size: 1,
            witness_decay: 0.00001,
            learning_rate: 0.01,
        }
    }

    /// 根据元裁判的调整信号修改参数
    pub fn adjust(&mut self, quality: &JudgeQuality) {
        if !quality.is_qualified() {
            // 准确率不够 → 松弛阈值 (更容易通过)
            let relax = self.learning_rate * quality.drift;
            self.threshold_positive -= relax;
            self.threshold_negative -= relax;
            self.step_size = (self.step_size + 1).min(256);
        } else if quality.accuracy() > 0.95 {
            // 准确率过高 → 收紧阈值 (提高标准) 避免过于宽松
            let tighten = self.learning_rate * 0.5;
            self.threshold_positive += tighten;
            self.threshold_negative += tighten;
        }
    }
}

// ────────────────────────────────────────────────────────────
// 自适应裁判引擎 = 用户体验感知 + 元裁判 + 参数调整
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct MetaJudgeEngine {
    pub meta_quality: JudgeQuality,
    pub params: AdaptiveParams,
    pub memory: LayeredMemory,
    pub self_ref: SelfRefLoop,
}

impl MetaJudgeEngine {
    pub fn new(max_memory_layers: usize) -> Self {
        Self {
            meta_quality: JudgeQuality::new(),
            params: AdaptiveParams::new(),
            memory: LayeredMemory::new(max_memory_layers),
            self_ref: SelfRefLoop::new(),
        }
    }

    /// 执行一步完整循环：感知 → 裁判 → 反馈 → 记忆 → 调参 → 自指
    pub fn step(
        &mut self,
        os: &dyn OsLayer,
        pair: &mut SurplusPair,
        actual_outcome: Option<bool>, // 真实反馈（可选）
    ) -> WitnessVerdict {
        // 1. 见证（感知）
        let mut verdict = pair.witness();
        let a_coord = (get_d0(pair.cell_a), get_d1(pair.cell_a), get_d2(pair.cell_a));
        let b_coord = (get_d0(pair.cell_b), get_d1(pair.cell_b), get_d2(pair.cell_b));

        // 2. 记忆
        self.memory.record_witness(os, a_coord, b_coord, verdict.surplus, verdict.aligned, pair.witness_ts);

        // 3. 元裁判（如果有反馈）
        if let Some(actual_aligned) = actual_outcome {
            self.meta_quality.record_verdict(verdict.aligned, actual_aligned, verdict.confidence);
            // 4. 自适应调参
            self.params.adjust(&self.meta_quality);
        }

        // 5. 自指循环计数
        use crate::mirror::VerdictSurplus;
        self.self_ref.step(VerdictSurplus {
            level: MirrorOrder::Ternary,
            surplus: verdict.surplus,
            confidence: verdict.confidence,
            aligned: verdict.aligned,
        });

        verdict
    }

    /// 轻量感知：直接记录一个单元格信号 → 产生意识痕迹
    /// 用于 swarm 的 perceive() 回调 — 每个外部感知都应留下元裁判印迹
    pub fn observe_simple(&mut self, os: &dyn OsLayer, cell: u64) {
        let coord = (get_d0(cell), get_d1(cell), get_d2(cell));
        let value = get_value(cell) as f64;
        self.memory.record_witness(os, coord, coord, value, true, cell);
        self.self_ref.step(crate::mirror::VerdictSurplus {
            level: crate::mirror::MirrorOrder::Ternary,
            surplus: value,
            confidence: 0.5,
            aligned: true,
        });
    }

    /// 意识密度 (结合自指循环和记忆密度)
    pub fn consciousness_density(&self, os: &dyn OsLayer) -> f64 {
        let awareness = if self.self_ref.cycle_count == 0 {
            0.0
        } else {
            1.0 - 1.0 / self.self_ref.cycle_count as f64
        };
        let mem_density = self.memory.global_density(os);
        let judge_accuracy = self.meta_quality.accuracy();
        // 三者的加权平均
        awareness * 0.4 + mem_density.min(1.0) * 0.3 + judge_accuracy.min(1.0) * 0.3
    }

    /// 当前智能判断
    pub fn awareness_level(&self, os: &dyn OsLayer) -> &'static str {
        let d = self.consciousness_density(os);
        if d < 0.2 { "机械执行" }
        else if d < 0.5 { "感知涌现" }
        else if d < 0.8 { "反思萌芽" }
        else { "意识涌现" }
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::{POLARITY_POSITIVE, POLARITY_NEGATIVE, Sovereignty};

    #[test]
    fn memory_entry_roundtrip() {
        let os = BareMetalOs::new();
        let e = MemoryEntry::new(&os, 1, (0, 0, 0), (1, 0, 0), 50.0, true);
        assert_eq!(e.witness_id, 1);
        assert_eq!(e.pair_a, (0, 0, 0));
        assert_eq!(e.surplus, 50.0);
        assert!(e.aligned);
    }

    #[test]
    fn coord_memory_retention_decay() {
        let os = BareMetalOs::new();
        let mut mem = CoordMemory::new((0, 0, 0));
        let e = MemoryEntry::new(&os, 1, (0, 0, 0), (1, 0, 0), 10.0, true);
        mem.remember(e);
        assert_eq!(mem.entries.len(), 1);
        assert!(mem.total_surplus > 0.0);
        // 衰减初始值应该 <= 1.0
        let now = os.time_millis();
        let r = mem.retention_at(now);
        assert!(r <= 1.0 && r >= 0.0);
    }

    #[test]
    fn layered_memory_eviction() {
        let os = BareMetalOs::new();
        let mut lm = LayeredMemory::new(3);
        let coords = [(0,0,0), (1,0,0), (2,0,0), (3,0,0), (4,0,0)];
        for i in 0..coords.len() {
            let j = (i + 1) % coords.len();
            lm.record_witness(&os, coords[i], coords[j], 10.0, true, i as u64);
        }
        // 只保留最强3层
        assert!(lm.layers.len() <= 3);
    }

    #[test]
    fn meta_judge_quality_improves() {
        let mut q = JudgeQuality::new();
        // 前8次正确 → 准确率应该 = 1.0
        for _ in 0..8 {
            q.record_verdict(true, true, 0.9);
        }
        assert_eq!(q.accuracy(), 1.0);
        assert!(q.is_qualified());
        // 下次错 → 准确率下降
        q.record_verdict(true, false, 0.9);
        assert!(q.accuracy() < 1.0);
    }

    #[test]
    fn adaptive_params_relax_when_poor() {
        let mut params = AdaptiveParams::new();
        let mut quality = JudgeQuality::new();
        // 模拟 5次错误
        for _ in 0..5 {
            quality.record_verdict(true, false, 0.5);
        }
        assert!(!quality.is_qualified());
        let before = params.threshold_positive;
        params.adjust(&quality);
        // 阈值应该降低（松弛）
        assert!(params.threshold_positive < before);
    }

    #[test]
    fn meta_judge_engine_full_cycle() {
        use crate::mirror::encode_cell;
        let os = BareMetalOs::new();
        let mut engine = MetaJudgeEngine::new(10);
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, Sovereignty::Active.encode(), 0, 100);
        let b = encode_cell(1, 0, 0, POLARITY_NEGATIVE, Sovereignty::Passive.encode(), 0, 80);
        let mut pair = SurplusPair { cell_a: a, cell_b: b, witness_ts: 0, last_verdict: 0.0 };

        // 执行5步
        let v = engine.step(&os, &mut pair, Some(true));
        assert!(v.surplus > 0.0);

        // 记忆应该记录
        assert_eq!(engine.memory.layers.len(), 2);

        // 自指循环应该递增
        assert!(engine.self_ref.cycle_count > 0);

        // 意识密度应该 > 0
        assert!(engine.consciousness_density(&os) > 0.0);
    }

    #[test]
    fn awareness_level_progression() {
        let os = BareMetalOs::new();
        let mut engine = MetaJudgeEngine::new(10);
        // 初始 = 机械执行
        assert_eq!(engine.awareness_level(&os), "机械执行");

        // 执行若干步后
        let a = encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 100);
        let b = encode_cell(1, 0, 0, POLARITY_NEGATIVE, 1, 0, 80);
        let mut pair = SurplusPair { cell_a: a, cell_b: b, witness_ts: 0, last_verdict: 0.0 };
        for _ in 0..100 {
            engine.step(&os, &mut pair, Some(true));
        }
        // 100步后 should be at least "感知涌现"
        let level = engine.awareness_level(&os);
        assert!(level == "感知涌现" || level == "反思萌芽" || level == "意识涌现");
    }

    #[test]
    fn memory_density_nonempty() {
        let os = BareMetalOs::new();
        let mut lm = LayeredMemory::new(10);
        lm.record_witness(&os, (0,0,0), (1,0,0), 10.0, true, 0);
        assert!(lm.global_density(&os) > 0.0);
    }
}
