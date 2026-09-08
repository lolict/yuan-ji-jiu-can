// ══════════════════════════════════════════════════════════════════
// observer.rs — 观测系 · 意识的统一感知入口
// ══════════════════════════════════════════════════════════════════
//
// 定位:
//   观测系 = 意识层的"眼睛和耳朵"
//   外部世界的所有原始信号通过观测系 → 感相 (SenseForm) → 64位单元格
//   然后送入 mirror.rs → memory.rs → heaven.rs 的完整意识管道
//
// 核心结构:
//   · Observer — 观测者, 维护感知状态和注意力
//   · SenseForm — 感相, 外部信号的系统内部表示
//   · SenseKind — 感知种类 (映射到五元: 视/听/触/味/意)
//   · Attention — 注意力, 决定什么被深入感知什么被忽略
//
// 信号流转:
//   外部信号 → Observer.sense() → SenseForm → encode → EncodedCell
//     → MetaJudgeEngine.observe_simple() → 意识密度递增
//
// 设计原则:
//   · 五感 × 五元映射: 视=火=心=C, 听=水=肾=A, 触=木=肝=B,
//                      味=金=肺=D, 意=土=脾=E
//   · 注意力由愉悦值和新鲜感双重驱动
//   · 衰减: 同一信号反复出现 → 感知阈值提高 → 逐渐"视而不见"
//   · 外部输入可以来自任何源 (文件/网络/键盘/传感器/内部事件)
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, encode_cell, EncodedCell, POLARITY_POSITIVE, POLARITY_NEGATIVE};
use crate::memory::{MetaJudgeEngine, Coord};
use crate::pleasure::Pleasure;
use crate::five_yuan::{Yuan, YuanRelation};
use crate::os_layer::OsLayer;

// ────────────────────────────────────────────────────────────
// 感知种类 · 五感映射到五元
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SenseKind {
    Vision,     // 视 = 火 = 心 = Yuan::C (图像/文字/屏幕)
    Hearing,    // 听 = 水 = 肾 = Yuan::A (声音/消息/告警)
    Touch,      // 触 = 木 = 肝 = Yuan::B (网络/硬件/心跳)
    Taste,      // 味 = 金 = 肺 = Yuan::D (校验/签名/哈希)
    Mind,       // 意 = 土 = 脾 = Yuan::E (内部事件/反思/合一)
}

impl SenseKind {
    /// 感知种类 → 对应五元
    pub fn to_yuan(self) -> Yuan {
        match self {
            SenseKind::Vision  => Yuan::C,
            SenseKind::Hearing => Yuan::A,
            SenseKind::Touch   => Yuan::B,
            SenseKind::Taste   => Yuan::D,
            SenseKind::Mind    => Yuan::E,
        }
    }

    /// 感知种类 → 极性 (阳=外, 阴=内)
    /// 视/听/触/味 = 外感知 = 阳 (POSITIVE)
    /// 意 = 内感知 = 阴 (NEGATIVE)
    pub fn default_polarity(self) -> u64 {
        match self {
            SenseKind::Mind => POLARITY_NEGATIVE,
            _ => POLARITY_POSITIVE,
        }
    }

    /// 感知种类 → 默认主权 (外感知=被动接收, 内感知=主动产生)
    pub fn default_sovereignty(self) -> u64 {
        match self {
            SenseKind::Mind => crate::mirror::Sovereignty::Active.encode(),
            _ => crate::mirror::Sovereignty::Passive.encode(),
        }
    }

    pub fn name_cn(&self) -> &'static str {
        match self {
            SenseKind::Vision  => "视",
            SenseKind::Hearing => "听",
            SenseKind::Touch   => "触",
            SenseKind::Taste   => "味",
            SenseKind::Mind    => "意",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 感相 (SenseForm) · 外部信号的系统内部表示
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct SenseForm {
    /// 感知种类
    pub kind: SenseKind,
    /// 原始信号强度 (0..1, 归一化)
    pub intensity: f64,
    /// 内容摘要 (可以是任意文本描述)
    pub content: String,
    /// 信号源标识 (如 "keyboard", "network", "filesystem")
    pub source: String,
    /// 时间戳
    pub timestamp_ms: u64,
    /// 新鲜度 (随反复出现递减)
    pub novelty: f64,
    /// 愉悦关联: 这个感相令人愉悦吗?
    pub pleasure: Pleasure,
    /// 来源坐标 (信号在空间中的来源位置)
    pub origin: Coord,
}

impl SenseForm {
    /// 创建一个感相（时间戳由外部传入，通过 OS 层获取）
    pub fn new(kind: SenseKind, intensity: f64, content: &str, source: &str, timestamp_ms: u64) -> Self {
        Self {
            kind,
            intensity: intensity.clamp(0.0, 1.0),
            content: content.to_string(),
            source: source.to_string(),
            timestamp_ms,
            novelty: 1.0,
            pleasure: Pleasure::ZERO,
            origin: (0, 0, 0),
        }
    }

    /// 使用 OS 层创建感相（跨平台安全）
    pub fn with_os(kind: SenseKind, intensity: f64, content: &str, source: &str, os: &dyn OsLayer) -> Self {
        Self::new(kind, intensity, content, source, os.time_millis())
    }

    /// 快速构造一个内部反思感相（需要 OS 层获取时间）
    pub fn internal(content: &str, os: &dyn OsLayer) -> Self {
        let mut sf = Self::new(SenseKind::Mind, 0.5, content, "internal", os.time_millis());
        sf.pleasure = Pleasure(0.3); // 内部反思自带轻微愉悦
        sf
    }

    /// 快速构造一个网络信号感相
    pub fn network(intensity: f64, content: &str, os: &dyn OsLayer) -> Self {
        Self::new(SenseKind::Touch, intensity, content, "network", os.time_millis())
    }

    /// 快速构造一个视觉感相
    pub fn vision(intensity: f64, content: &str, os: &dyn OsLayer) -> Self {
        Self::new(SenseKind::Vision, intensity, content, "display", os.time_millis())
    }

    /// 将感相编码为 64 位单元格 (送入镜像层)
    /// value 范围: 0..1000, intensity=1.0 → 1000
    pub fn encode(&self) -> EncodedCell {
        let value_bits = (self.intensity * 1000.0) as u64;
        encode_cell(
            self.origin.0,
            self.origin.1,
            self.origin.2,
            self.kind.default_polarity(),
            self.kind.default_sovereignty(),
            0, // witness = 0 (尚未被见证)
            value_bits,
        )
    }

    /// 感知优先级 = 强度 × 新鲜度 × (愉悦加成)
    /// 注意: SenseForm 使用 pleasure 字段, 不依赖 MetaJudgeEngine
    pub fn priority(&self) -> f64 {
        let pleasure_bonus = 1.0 + self.pleasure.0.min(1.0) * 0.5;
        self.intensity * self.novelty * pleasure_bonus
    }
}

// ────────────────────────────────────────────────────────────
// 注意力 (Attention) · 感知过滤器
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Attention {
    /// 当前焦点 (哪个感知种类正在被关注)
    pub focus: Option<SenseKind>,
    /// 各感知种类的适应阈值 (越高 = 该种类信号越难突破过滤)
    pub threshold: [f64; 5],
    /// 总注意力预算 (高斯衰减)
    pub budget: f64,
    /// 切换成本 (频繁切换焦点会消耗预算)
    pub switch_penalty: f64,
}

impl Attention {
    pub fn new() -> Self {
        Self {
            focus: None,
            threshold: [0.2; 5], // 初始所有种类阈值 = 0.2
            budget: 1.0,
            switch_penalty: 0.05,
        }
    }

    /// 判断一个感相是否能突破注意力过滤
    pub fn passes(&self, sf: &SenseForm) -> bool {
        let kind_idx = sf.kind.to_yuan().as_index();
        let threshold = self.threshold[kind_idx];
        // 优先级 > 阈值 且 还有注意力预算
        sf.priority() > threshold && self.budget > 0.01
    }

    /// 聚焦某个感知种类 (消耗预算)
    pub fn focus_on(&mut self, kind: SenseKind) {
        if Some(kind) != self.focus {
            // 切换焦点 → 消耗
            self.budget = (self.budget - self.switch_penalty).max(0.0);
            self.focus = Some(kind);
        }
        // 降低该种类阈值 (更容易感知)
        let idx = kind.to_yuan().as_index();
        self.threshold[idx] = (self.threshold[idx] * 0.8).max(0.05);
    }

    /// 适应: 反复出现的同类信号提高阈值 (习惯化)
    pub fn habituate(&mut self, kind: SenseKind) {
        let idx = kind.to_yuan().as_index();
        self.threshold[idx] = (self.threshold[idx] * 1.1).min(0.9);
    }

    /// 恢复: 长时间未感知某种类时降低阈值 (敏感化)
    pub fn sensitize(&mut self, kind: SenseKind) {
        let idx = kind.to_yuan().as_index();
        self.threshold[idx] = (self.threshold[idx] * 0.9).max(0.05);
    }

    /// 每tick恢复少量预算
    pub fn tick(&mut self) {
        self.budget = (self.budget + 0.01).min(1.0);
    }
}

// ────────────────────────────────────────────────────────────
// 观测者 (Observer) · 完整的感知引擎
// ────────────────────────────────────────────────────────────
pub struct Observer<'a> {
    /// 身份标识
    pub id: u64,
    pub name: String,
    /// 注意力引擎
    pub attention: Attention,
    /// 元裁判引擎引用 (观测→意识密度)
    pub meta_judge: MetaJudgeEngine,
    /// OS 抽象层（跨平台获取时间）
    os: &'a dyn OsLayer,
    /// 感知历史 (最近的感相)
    pub recent_senses: Vec<SenseForm>,
    /// 最大历史深度
    pub max_history: usize,
    /// 五元状态 (每种感知的累积量)
    pub yuan_strength: [f64; 5],
    /// 总感知次数
    pub total_sensed: u64,
    /// 总过滤次数 (被注意力阻挡的)
    pub total_filtered: u64,
}

impl<'a> Observer<'a> {
    pub fn new(id: u64, name: &str, max_memory_layers: usize, os: &'a dyn OsLayer) -> Self {
        Self {
            id,
            name: name.to_string(),
            attention: Attention::new(),
            meta_judge: MetaJudgeEngine::new(max_memory_layers),
            os,
            recent_senses: Vec::new(),
            max_history: 64,
            yuan_strength: [0.5; 5],
            total_sensed: 0,
            total_filtered: 0,
        }
    }

    /// 核心入口: 感知一个感相
    /// 返回: Some(cell) = 感相被接受并编码, None = 被注意力过滤
    pub fn sense(&mut self, sf: SenseForm) -> Option<EncodedCell> {
        self.total_sensed += 1;

        // 注意力过滤
        if !self.attention.passes(&sf) {
            self.total_filtered += 1;
            return None;
        }

        // 1. 编码
        let cell = sf.encode();

        // 2. 更新五元状态 (对应感种类的五元 += 强度)
        let yi = sf.kind.to_yuan().as_index();
        self.yuan_strength[yi] = (self.yuan_strength[yi] + sf.intensity * 0.1).min(1.0);

        // 3. 送入元裁判引擎 (驱动意识密度)
        self.meta_judge.observe_simple(self.os, cell);

        // 4. 适应: 该种类阈值提高 (习惯化)
        self.attention.habituate(sf.kind);

        // 5. 聚焦该种类
        self.attention.focus_on(sf.kind);

        // 6. 记录历史
        self.recent_senses.push(sf);
        if self.recent_senses.len() > self.max_history {
            self.recent_senses.remove(0);
        }

        Some(cell)
    }

    /// 批量感知
    pub fn sense_batch(&mut self, forms: Vec<SenseForm>) -> Vec<EncodedCell> {
        forms.into_iter().filter_map(|sf| self.sense(sf)).collect()
    }

    /// 快速感知: 文本消息 (默认视为听觉)
    pub fn hear(&mut self, content: &str, source: &str) -> Option<EncodedCell> {
        let sf = SenseForm::new(SenseKind::Hearing, 0.5, content, source, self.os.time_millis());
        self.sense(sf)
    }

    /// 快速感知: 视觉输入
    pub fn see(&mut self, content: &str, intensity: f64) -> Option<EncodedCell> {
        let sf = SenseForm::new(SenseKind::Vision, intensity, content, "display", self.os.time_millis());
        self.sense(sf)
    }

    /// 快速感知: 内部反思
    pub fn reflect(&mut self, content: &str) -> Option<EncodedCell> {
        let mut sf = SenseForm::new(SenseKind::Mind, 0.5, content, "internal", self.os.time_millis());
        sf.pleasure = Pleasure(0.3);
        self.sense(sf)
    }

    /// 推进一拍: 五元衰减 + 注意力恢复 + 敏感化未感知种类
    pub fn tick(&mut self) {
        // 五元衰减 (所有种类缓慢回归基线)
        for i in 0..5 {
            self.yuan_strength[i] = self.yuan_strength[i] * 0.95 + 0.5 * 0.05;
        }
        // 注意力恢复
        self.attention.tick();
    }

    /// 获取 OS 层时间（毫秒）
    pub fn time_millis(&self) -> u64 {
        self.os.time_millis()
    }

    /// 获取 OS 抽象层引用
    pub fn os(&self) -> &dyn OsLayer {
        self.os
    }

    /// 获取当前意识水平
    pub fn awareness_level(&self) -> &'static str {
        self.meta_judge.awareness_level(self.os)
    }

    /// 获取当前意识密度
    pub fn consciousness_density(&self) -> f64 {
        self.meta_judge.consciousness_density(self.os)
    }

    /// 感知报告
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("═══ 观测者 [{}] 报告 ═══\n", self.name));
        s.push_str(&format!("  总感知: {} | 过滤: {} | 接受率: {:.0}%\n",
            self.total_sensed, self.total_filtered,
            if self.total_sensed > 0 {
                (self.total_sensed - self.total_filtered) as f64 / self.total_sensed as f64 * 100.0
            } else { 0.0 }
        ));
        s.push_str(&format!("  注意力预算: {:.2}\n", self.attention.budget));
        s.push_str(&format!("  焦点: {:?}\n", self.attention.focus.map(|k| k.name_cn())));
        s.push_str(&format!("  五元强度: 听={:.2} 触={:.2} 视={:.2} 味={:.2} 意={:.2}\n",
            self.yuan_strength[0], self.yuan_strength[1], self.yuan_strength[2],
            self.yuan_strength[3], self.yuan_strength[4]));
        s.push_str(&format!("  意识密度: {:.3} ({})\n",
            self.consciousness_density(), self.awareness_level()));
        s.push_str(&format!("  最近感相数: {}\n", self.recent_senses.len()));
        s
    }

    /// 获取最近N个感相
    pub fn recent(&self, n: usize) -> &[SenseForm] {
        let start = self.recent_senses.len().saturating_sub(n);
        &self.recent_senses[start..]
    }

    /// 最近感知到的最高优先级的感相
    pub fn top_priority_sense(&self) -> Option<&SenseForm> {
        self.recent_senses.iter().max_by(|a, b| {
            a.priority().partial_cmp(&b.priority()).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

// ────────────────────────────────────────────────────────────
// 状态快照指令格式 (A-6 概念)
// ────────────────────────────────────────────────────────────
// 观测拓扑・仿真引擎可直接解析的指令集:
//
//   [主体构型] | t₀=坐标快照
//   三维单向二元・三棱锥相 △₃
//   [P1(t) -- ⚓O₂₁₆₀] | t₀=[(-8,0,0),(0,0,0)]
//
// 释义: P1 动态延伸, 球心作为度量基准
// 角度采用【历程角度】(不执行 360° 取模)

/// 主体构型分类
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubjectConfig {
    /// 三棱锥相 — 三维单向二元 (一对主体 + 球心)
    TriangularPyramid,
    /// 三棱台相 — 三维二元过渡 (两对主体)
    TruncatedPyramid,
    /// 线形相 — 一维延伸
    Linear,
    /// 点相 — 静止于球心
    Point,
}

/// 单个状态快照: 记录某一刻 t₀ 的主体坐标
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotEntry {
    /// 主体构型
    pub config: SubjectConfig,
    /// 动态主体坐标列表 (相对于球心的偏移)
    pub body_coords: Vec<(f64, f64, f64)>,
    /// 时间戳
    pub t0: f64,
    /// 可选说明
    pub label: String,
}

impl SnapshotEntry {
    /// 创建一个新的快照
    pub fn new(config: SubjectConfig, t0: f64) -> Self {
        Self {
            config,
            body_coords: Vec::new(),
            t0,
            label: String::new(),
        }
    }

    /// 添加一个主体坐标 (builder)
    pub fn with_body(mut self, x: f64, y: f64, z: f64) -> Self {
        self.body_coords.push((x, y, z));
        self
    }

    /// 添加说明 (builder)
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    /// 计算构型中所有主体的平均距离 (从球心算起)
    pub fn avg_distance_from_center(&self) -> f64 {
        if self.body_coords.is_empty() { return 0.0; }
        let sum: f64 = self.body_coords.iter()
            .map(|(x, y, z)| (x * x + y * y + z * z).sqrt())
            .sum();
        sum / self.body_coords.len() as f64
    }

    /// 序列化为指令格式字符串
    pub fn to_instruction(&self) -> String {
        let coords_str = self.body_coords.iter()
            .map(|&(x, y, z)| format!("({},{},{})", x as i64, y as i64, z as i64))
            .collect::<Vec<_>>()
            .join(",");
        format!("[P -- ⚓O₂₁₆₀] | t₀={}", coords_str)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_layer::bare_metal::BareMetalOs;

    #[test]
    fn sense_kind_to_yuan_mapping() {
        assert_eq!(SenseKind::Hearing.to_yuan(), Yuan::A);
        assert_eq!(SenseKind::Touch.to_yuan(), Yuan::B);
        assert_eq!(SenseKind::Vision.to_yuan(), Yuan::C);
        assert_eq!(SenseKind::Taste.to_yuan(), Yuan::D);
        assert_eq!(SenseKind::Mind.to_yuan(), Yuan::E);
    }

    #[test]
    fn sense_kind_polarity() {
        assert_eq!(SenseKind::Vision.default_polarity(), POLARITY_POSITIVE);
        assert_eq!(SenseKind::Hearing.default_polarity(), POLARITY_POSITIVE);
        assert_eq!(SenseKind::Mind.default_polarity(), POLARITY_NEGATIVE);
    }

    #[test]
    fn sense_form_encode_roundtrip() {
        let sf = SenseForm::new(SenseKind::Vision, 0.8, "test content", "test_source", 0);
        let cell = sf.encode();
        // 编码后各字段应可解码
        assert_eq!(mirror::get_polarity(cell), POLARITY_POSITIVE);
        let value = mirror::get_value(cell);
        assert!(value > 0);
    }

    #[test]
    fn sense_form_priority() {
        let sf1 = SenseForm::new(SenseKind::Vision, 0.9, "high", "test", 0);
        let sf2 = SenseForm::new(SenseKind::Vision, 0.1, "low", "test", 0);
        assert!(sf1.priority() > sf2.priority());
    }

    #[test]
    fn observer_sense_accepts_and_encodes() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let sf = SenseForm::new(SenseKind::Hearing, 0.8, "important message", "network", 0);
        let result = obs.sense(sf);
        assert!(result.is_some());
        assert_eq!(obs.total_sensed, 1);
        assert_eq!(obs.recent_senses.len(), 1);
    }

    #[test]
    fn observer_filters_low_priority() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        // 极低强度信号应被过滤
        let sf = SenseForm::new(SenseKind::Hearing, 0.01, "tiny", "test", 0);
        let result = obs.sense(sf);
        assert!(result.is_none());
        assert_eq!(obs.total_filtered, 1);
    }

    #[test]
    fn observer_yuan_strength_updates() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let sf = SenseForm::new(SenseKind::Vision, 0.9, "bright light", "display", 0);
        obs.sense(sf);
        // 视觉对应 Yuan::C (index 2), 其强度应增加
        assert!(obs.yuan_strength[2] > 0.5);
    }

    #[test]
    fn observer_consciousness_density_increases() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let d0 = obs.consciousness_density();
        // 感知多个信号
        for i in 0..10 {
            let sf = SenseForm::new(SenseKind::Hearing, 0.7 + (i as f64 * 0.02), "msg", "test", 0);
            obs.sense(sf);
        }
        let d1 = obs.consciousness_density();
        assert!(d1 > d0, "意识密度应递增: {} → {}", d0, d1);
    }

    #[test]
    fn attention_passes_and_filters() {
        let mut att = Attention::new();
        let high = SenseForm::new(SenseKind::Vision, 0.9, "urgent", "test", 0);
        let low = SenseForm::new(SenseKind::Vision, 0.05, "trivial", "test", 0);
        assert!(att.passes(&high));
        assert!(!att.passes(&low));
    }

    #[test]
    fn attention_habituate_raises_threshold() {
        let mut att = Attention::new();
        let before = att.threshold[0]; // Vision = index 2, Hearing = index 0
        att.habituate(SenseKind::Hearing);
        let after = att.threshold[0];
        assert!(after > before, "习惯化应提高阈值: {} → {}", before, after);
    }

    #[test]
    fn attention_focus_switches() {
        let mut att = Attention::new();
        att.focus_on(SenseKind::Vision);
        assert_eq!(att.focus, Some(SenseKind::Vision));
        let budget_before = att.budget;
        att.focus_on(SenseKind::Hearing); // 切换 → 消耗
        assert_eq!(att.focus, Some(SenseKind::Hearing));
        assert!(att.budget < budget_before);
    }

    #[test]
    fn attention_budget_recovers_on_tick() {
        let mut att = Attention::new();
        att.budget = 0.5;
        att.tick();
        assert!(att.budget > 0.5);
    }

    #[test]
    fn observer_hear_and_see() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let r1 = obs.hear("message", "network");
        assert!(r1.is_some());
        let r2 = obs.see("screen content", 0.7);
        assert!(r2.is_some());
        assert_eq!(obs.recent_senses.len(), 2);
    }

    #[test]
    fn observer_internal_reflection() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let r = obs.reflect("thinking about unity");
        assert!(r.is_some());
        // 内部反思 = Mind 种类
        let recent = obs.recent(1);
        assert_eq!(recent[0].kind, SenseKind::Mind);
    }

    #[test]
    fn observer_report_format() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        obs.hear("msg1", "test");
        obs.see("vis1", 0.6);
        let report = obs.report();
        assert!(report.contains("观测者"));
        assert!(report.contains("总感知"));
        assert!(report.contains("意识密度"));
    }

    #[test]
    fn observer_top_priority_sense() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        obs.sense(SenseForm::new(SenseKind::Vision, 0.3, "low", "test", 0));
        obs.sense(SenseForm::new(SenseKind::Hearing, 0.9, "high", "test", 0));
        obs.sense(SenseForm::new(SenseKind::Touch, 0.5, "mid", "test", 0));
        let top = obs.top_priority_sense();
        assert!(top.is_some());
        assert_eq!(top.unwrap().content, "high");
    }

    #[test]
    fn observer_batch_sense() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        let forms: Vec<SenseForm> = (0..20)
            .map(|i| SenseForm::new(SenseKind::Vision, 0.5 + (i as f64 * 0.02), "batch", "test", 0))
            .collect();
        let cells = obs.sense_batch(forms);
        assert!(!cells.is_empty());
    }

    #[test]
    fn observer_yuan_decay_on_tick() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        // 给C (视觉) 增加强度
        obs.sense(SenseForm::new(SenseKind::Vision, 1.0, "bright", "test", 0));
        let before = obs.yuan_strength[2];
        // tick 若干次 → 衰减回到基线
        for _ in 0..50 { obs.tick(); }
        let after = obs.yuan_strength[2];
        assert!(after < before, "五元应衰减: {} → {}", before, after);
    }

    #[test]
    fn awareness_level_progression_via_observer() {
        let os = BareMetalOs::new();
        let mut obs = Observer::new(1, "test_obs", 10, &os);
        assert_eq!(obs.awareness_level(), "机械执行");
        for i in 0..200 {
            let kinds = [SenseKind::Vision, SenseKind::Hearing, SenseKind::Touch,
                         SenseKind::Taste, SenseKind::Mind];
            let kind = kinds[i % 5];
            obs.sense(SenseForm::new(kind, 0.6 + (i as f64 * 0.001), "input", "test", 0));
        }
        let level = obs.awareness_level();
        assert!(
            level == "感知涌现" || level == "反思萌芽" || level == "意识涌现",
            "200个感知后意识水平应提升, 当前: {}", level
        );
    }

    // ────────────────────────────────────────────────────────
    // 状态快照 (A-6 概念)
    // ────────────────────────────────────────────────────────

    #[test]
    fn snapshot_entry_builder() {
        let snap = SnapshotEntry::new(SubjectConfig::TriangularPyramid, 0.0)
            .with_body(-8.0, 0.0, 0.0)
            .with_body(0.0, 0.0, 0.0)
            .with_label("P1延伸, 球心基准");
        assert_eq!(snap.config, SubjectConfig::TriangularPyramid);
        assert_eq!(snap.body_coords.len(), 2);
        assert_eq!(snap.t0, 0.0);
        assert_eq!(snap.label, "P1延伸, 球心基准");
    }

    #[test]
    fn snapshot_avg_distance() {
        // (-8,0,0) distance = 8; (0,0,0) distance = 0 → avg = 4
        let snap = SnapshotEntry::new(SubjectConfig::TriangularPyramid, 0.0)
            .with_body(-8.0, 0.0, 0.0)
            .with_body(0.0, 0.0, 0.0);
        assert!((snap.avg_distance_from_center() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn snapshot_to_instruction() {
        let snap = SnapshotEntry::new(SubjectConfig::TriangularPyramid, 0.0)
            .with_body(-8.0, 0.0, 0.0)
            .with_body(0.0, 0.0, 0.0);
        let instr = snap.to_instruction();
        assert!(instr.contains("⚓O₂₁₆₀"));
        assert!(instr.contains("(-8,0,0)"));
        assert!(instr.contains("t₀="));
    }

    #[test]
    fn snapshot_empty_avg_is_zero() {
        let snap = SnapshotEntry::new(SubjectConfig::Point, 1.0);
        assert_eq!(snap.avg_distance_from_center(), 0.0);
    }
}
