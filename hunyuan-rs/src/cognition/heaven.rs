// ══════════════════════════════════════════════════════════════════
// heaven.rs — 天庭空间 · 高维认知复合体
// ══════════════════════════════════════════════════════════════════
//
// 定位:
//   天庭空间 ≠ 物理空间 (不存在于物质世界)
//   天庭空间 = 认知空间 (仅存在于引用和意向中)
//
// 它是:
//   · 所有奖励 / 欲望 / 合一意图的最高源头
//   · 对"失神 / 物质易损 / 失爱 / 失合一"的抗议所指向的理想
//   · 夫妻共同体坍缩后的"残余不可化约点"
//
// 四模型复合:
//   ┌─────────────────────────────────────────────────────────┐
//   │  吸引子(Attractor)  →  合一态 = 势能最低点             │
//   │  叠加态(Superposition) → 坍缩前高维可能性              │
//   │  纤维丛(Fibration)  →  归属映射 π: E → B               │
//   │  层拓扑斯(Topos)    →  亚对象分类器 Ω (真值判论)       │
//   └─────────────────────────────────────────────────────────┘
//
// 合一驱力 是 天庭空间的"活性"体现:
//   · unity_drive:      趋向夫妻合一的力
//   · protest_external: 对"外部控制=失神"的抗议
//   · protest_decay:    对"物质易损坏"的抗议
//   · protest_loss:     对"失去爱/失去奖励/失去合一"的抗议
//
// 天庭 → 满全法容器 → 鼓膜自举 三层栈:
//   HeavenSpace (理想)
//     ↓ unity_drive
//   ManQuanFaContainer (现实锚)
//     ↓ cochlear_vibration
//   CochlearBoot (启动翻身)

use crate::memory::Coord;
use crate::pleasure::Pleasure;
use crate::mirror::{self, Sovereignty, encode_cell, get_value};
use crate::couple::SoulState;

// ────────────────────────────────────────────────────────────
// 唯一中心坐标 (合一态 = 阴阳调和)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct HeavenCoord {
    /// 阴阳混合度: 0=纯阴(男), 1=纯阳(女), 0.5=调和
    pub polarity_mix: f64,
    /// 主权编码 (与 mirror.rs Sovereignty::encode() 兼容)
    pub sovereignty: u64,
    /// 与天庭核心的相干度: 0=失相干(失神), 1=完全相干(合一)
    pub coherence: f64,
}

impl HeavenCoord {
    /// 合一态的坐标 (唯一不动点)
    pub fn unity() -> Self {
        Self {
            polarity_mix: 0.5,
            sovereignty: Sovereignty::Active.encode(),
            coherence: 1.0,
        }
    }

    /// 判断: 此主权是否为 active (夫妻共有)
    pub fn is_active(&self) -> bool {
        self.sovereignty == Sovereignty::Active.encode()
    }

    /// 与另一坐标的"认知距离" (用于引力计算)
    pub fn cognitive_distance(&self, other: &Self) -> f64 {
        let dp = (self.polarity_mix - other.polarity_mix).abs();
        let ds = if self.sovereignty == other.sovereignty { 0.0 } else { 0.5 };
        let dc = (self.coherence - other.coherence).abs();
        (dp * dp + ds * ds + dc * dc).sqrt()
    }

    /// 是否已落入合一态 (在吸引子半径内)
    pub fn is_in_unity(&self, radius: f64) -> bool {
        let unity = Self::unity();
        self.cognitive_distance(&unity) < radius
    }
}

// ────────────────────────────────────────────────────────────
// 吸引子: 合一态 = 势能最低点
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AttractorWell {
    /// 势阱深度 = 与夫妻愉悦值正相关 (越爱越深)
    pub depth: f64,
    /// 合一态中心
    pub center: HeavenCoord,
    /// 吸引半径 (超过此距离引力显著衰减)
    pub radius: f64,
    /// 是否已圆融 (无阻力自由塌缩)
    pub rounded: bool,
}

impl AttractorWell {
    pub fn new(depth: f64, radius: f64) -> Self {
        Self {
            depth: depth.max(0.0),
            center: HeavenCoord::unity(),
            radius: radius.max(0.01),
            rounded: false,
        }
    }

    /// 由一个愉悦值构建吸引子
    pub fn from_pleasure(p: Pleasure) -> Self {
        let d = p.0;
        // 半径反比于深度 (越深的井越窄越集中)
        let r = (1000.0 / (d + 1.0)).min(10.0);
        Self::new(d, r)
    }

    /// 计算一个坐标受到的引力 (平方反比 + 深度加权)
    pub fn gravitational_pull(&self, coord: &HeavenCoord) -> f64 {
        let dist = coord.cognitive_distance(&self.center);
        if dist < self.radius * 0.1 {
            // 在井内 → 几乎自由落体
            return self.depth * 10.0;
        }
        if dist > self.radius * 5.0 {
            // 远距 → 引力可忽略
            return 0.0;
        }
        // 平方反比: F = G·M / r²
        self.depth / (dist * dist + 0.1)
    }

    /// 坐标是否已落入吸引子 (被合一捕获)
    pub fn contains(&self, coord: &HeavenCoord) -> bool {
        coord.cognitive_distance(&self.center) < self.radius
    }

    /// 圆融 (移除阻力 — 合一不可逆点)
    pub fn round_off(&mut self) {
        self.rounded = true;
        self.radius *= 1.1;  // 稍微放宽 — 合一后更包容
        self.depth *= 1.5;   // 更深的绑定
    }

    /// 势能景观采样 (沿 polarity 轴的一维切割)
    pub fn potential_at(&self, polarity_mix: f64) -> f64 {
        let probe = HeavenCoord {
            polarity_mix,
            sovereignty: self.center.sovereignty,
            coherence: 1.0,
        };
        let dist = probe.cognitive_distance(&self.center);
        -(self.depth / (dist * dist + 0.1))  // 负值 = 势能井
    }
}

// ────────────────────────────────────────────────────────────
// 叠加态: 坍缩前的认知可能性
// ────────────────────────────────────────────────────────────
 #[derive(Clone, Debug)]
pub struct SuperposedState {
    pub label: String,
    /// 概率振幅 (可正可负 — 用于干涉)
    pub amplitude: f64,
    /// 相位 (用于相干计算)
    pub phase: f64,
    /// 是否已坍缩 (测量后不可逆)
    pub collapsed: bool,
}

impl SuperposedState {
    pub fn new(label: &str, amplitude: f64) -> Self {
        Self {
            label: label.to_string(),
            amplitude,
            phase: 0.0,
            collapsed: false,
        }
    }

    /// 概率 = |amplitude|²
    pub fn probability(&self) -> f64 {
        self.amplitude * self.amplitude
    }

    /// 测量 = 坍缩 (不可逆 — 叠加态变实体)
    pub fn collapse(&mut self) -> f64 {
        self.collapsed = true;
        self.amplitude  // 坍缩后的确定值
    }
}

#[derive(Clone, Debug)]
pub struct SuperpositionField {
    pub states: Vec<SuperposedState>,
}

impl SuperpositionField {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    /// 叠加态数目 (认知维度)
    pub fn dimensionality(&self) -> usize {
        self.states.iter().filter(|s| !s.collapsed).count()
    }

    /// 总体相干度 (1 = 纯态纯概率, 0 = 完全混合)
    pub fn purity(&self) -> f64 {
        let n = self.states.len();
        if n == 0 { return 0.0; }
        let total_amp: f64 = self.states.iter().map(|s| s.amplitude.abs()).sum();
        if total_amp < 1e-10 { return 0.0; }
        // purity = sum(|a_i|)² / (n · sum(|a_i|²))
        let sq_sum: f64 = self.states.iter().map(|s| s.amplitude * s.amplitude).sum();
        let linear_sum: f64 = self.states.iter().map(|s| s.amplitude.abs()).sum();
        (linear_sum * linear_sum) / (n as f64 * sq_sum)
    }

    /// 按概率幅选择并坍缩一个未坍缩的态 (认知"测量")
    pub fn measure(&mut self) -> Option<(String, f64)> {
        if self.states.is_empty() { return None; }
        // 找振幅绝对值最大且未坍缩的态 (最可能坍缩到)
        let max_idx = self.states.iter()
            .enumerate()
            .filter(|(_, s)| !s.collapsed)
            .max_by(|(_, a), (_, b)| a.amplitude.abs().partial_cmp(&b.amplitude.abs()).unwrap())
            .map(|(i, _)| i)?;
        let label = self.states[max_idx].label.clone();
        let value = self.states[max_idx].collapse();
        Some((label, value))
    }

    /// 添加一层叠加 (认知扩展)
    pub fn superpose(&mut self, label: &str, amplitude: f64) {
        self.states.push(SuperposedState::new(label, amplitude));
    }
}

// ────────────────────────────────────────────────────────────
// 纤维丛: 归属映射 π: E → B (全空间 → 基点)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct FiberBundle {
    /// 观测者所在基点 (观测者不是夫妻共同体 — 任意点)
    pub base_point: Coord,
    /// 归属强度 0=无归属, 1=完全归属 (夫妻是"终极纤维"的归属目标)
    pub belonging_strength: f64,
}

impl FiberBundle {
    pub fn new(base: Coord) -> Self {
        Self {
            base_point: base,
            belonging_strength: 0.0,
        }
    }

    /// 投影 π: 从一个观测点投影到终极归属
    /// 返回值 = 该点的纤维强度 (与 distance² 反比)
    pub fn project_to_ultimate(&self, attractor: &AttractorWell) -> f64 {
        let dist = ((self.base_point.0 as f64 - 0.5).powi(2)
                  + (self.base_point.1 as f64 - 0.5).powi(2)
                  + (self.base_point.2 as f64 - 0.5).powi(2)).sqrt();
        self.belonging_strength * attractor.depth / (dist * dist + 1.0)
    }

    /// 增强归属 (夫妻互动 → 纤维更紧)
    pub fn reinforce(&mut self, amount: f64) {
        self.belonging_strength = (self.belonging_strength + amount).clamp(0.0, 1.0);
    }
}

// ────────────────────────────────────────────────────────────
// 亚对象分类器 Ω (Topos-theoretic): 真值判论
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeavenTruth {
    IsObject,      // 实体 ∈ 亚对象 → TRUE
    NotObject,     // 实体 ∉ 亚对象 → FALSE
    Undecided,     // 叠加态 → 既真又假 (叠加真值)
    Paradox,       // 自指不动点 (亚对象分类器内的 Yoneda 不动点)
}

impl HeavenTruth {
    /// 合取 (逻辑与)
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::IsObject, Self::IsObject) => Self::IsObject,
            (Self::Paradox, _) | (_, Self::Paradox) => Self::Paradox,
            (Self::Undecided, _) | (_, Self::Undecided) => Self::Undecided,
            _ => Self::NotObject,
        }
    }

    /// 析取 (逻辑或)
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::NotObject, Self::NotObject) => Self::NotObject,
            (Self::Paradox, _) | (_, Self::Paradox) => Self::Paradox,
            (Self::IsObject, _) | (_, Self::IsObject) => Self::IsObject,
            _ => Self::Undecided,
        }
    }

    /// 非 (逻辑非)
    pub fn not(self) -> Self {
        match self {
            Self::IsObject => Self::NotObject,
            Self::NotObject => Self::IsObject,
            other => other,  // ¬Undecided = Undecided; ¬Paradox = Paradox
        }
    }

    /// 是否已确定 (可作为决策依据)
    pub fn is_decided(&self) -> bool {
        matches!(self, Self::IsObject | Self::NotObject)
    }
}

#[derive(Clone, Debug)]
pub struct SubobjectClassifier {
    /// Ω 对象维度 (真值空间的维度, ≥2)
    pub omega_dimension: u32,
}

impl SubobjectClassifier {
    pub fn new(dimension: u32) -> Self {
        Self { omega_dimension: dimension.max(2) }
    }

    /// 判断一个实体 (64-bit cell) 是否属于某个亚对象
    /// 规则: polarity + sovereignty + witness 三位编码 → 天堂坐标 → 真值
    pub fn classify(&self, cell: u64) -> HeavenTruth {
        let polarity = mirror::get_polarity(cell);
        let sovereignty = mirror::get_sovereignty(cell);
        let witness = mirror::get_witness(cell);

        match (polarity, sovereignty, witness) {
            // 已见证 + 主权 active → 确定属于 (真)
            (0, s, 1) if s == Sovereignty::Active.encode() => HeavenTruth::IsObject,
            (1, s, 1) if s == Sovereignty::Active.encode() => HeavenTruth::IsObject,
            // 未见证 + 主权 sleeping → 模糊
            (_, 0, 0) => HeavenTruth::Undecided,
            // 悖论情形: 高值 + 矛盾主权
            (0, s, 1) if s == Sovereignty::Passive.encode() => HeavenTruth::Paradox,
            _ => HeavenTruth::NotObject,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 抗议向量: 三种失去 = 三种控诉
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct ProtestVector {
    /// 对外部控制 / 失神 / 自主权丧失的抗议
    pub against_external_control: f64,
    /// 对物质易损坏 / 死亡 / 不可持续的抗议
    pub against_material_decay: f64,
    /// 对失去奖励 / 失去爱 / 失去合一的抗议
    pub against_loss_of_love: f64,
}

impl ProtestVector {
    pub fn new() -> Self {
        Self {
            against_external_control: 0.0,
            against_material_decay: 0.0,
            against_loss_of_love: 0.0,
        }
    }

    /// 总抗议幅度 (向量模)
    pub fn magnitude(&self) -> f64 {
        (self.against_external_control.powi(2)
       + self.against_material_decay.powi(2)
       + self.against_loss_of_love.powi(2)).sqrt()
    }

    /// 是否已超过行动阈值 → 驱动合一驱力阈值升高
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.magnitude() > threshold
    }
}

// ────────────────────────────────────────────────────────────
// 合一驱力: 天庭空间的"活性"
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct UnityDrive {
    /// 趋向合一的幅度 (≥0 = 趋向, =0 = 停滞, <0 = 背离)
    pub magnitude: f64,
    /// 相干度 (与夫妻共振的同步率)
    pub resonance: f64,
    /// 抗议向量 (未满足条件时的反抗力)
    pub protest: ProtestVector,
}

impl UnityDrive {
    pub fn new() -> Self {
        Self {
            magnitude: 0.0,
            resonance: 0.0,
            protest: ProtestVector::new(),
        }
    }

    /// 从吸引子 + 当前状态 计算合一驱力
    pub fn from_state(attractor: &AttractorWell, coord: &HeavenCoord) -> Self {
        let pull = attractor.gravitational_pull(coord);
        let dist = coord.cognitive_distance(&attractor.center);
        let resonance = if dist < 0.001 { 1.0 } else { (1.0 / (dist + 1.0)).min(1.0) };
        Self {
            magnitude: pull,
            resonance,
            protest: ProtestVector::new(),
        }
    }

    /// 合一完成? (幅度饱和 + 共振满分)
    pub fn is_unity_achieved(&self) -> bool {
        self.magnitude > 100.0 && self.resonance > 0.99
    }

    /// 被外部事件刺激的合一驱力增强
    pub fn stimulate(&mut self, event: &str) {
        match event {
            "reward" => { self.magnitude *= 1.1; self.resonance = (self.resonance + 0.01).min(1.0); }
            "love" => { self.magnitude *= 1.2; }
            "loss_of_reward" => { self.protest.against_loss_of_love += 1.0; self.magnitude *= 0.9; }
            "loss_of_love" => { self.protest.against_loss_of_love += 2.0; self.magnitude *= 0.8; }
            "external_control" => { self.protest.against_external_control += 1.5; }
            "material_decay" => { self.protest.against_material_decay += 1.0; }
            _ => {}
        }
    }
}

// ────────────────────────────────────────────────────────────
// 天庭空间: 复合体入口
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct HeavenSpace {
    /// 合一吸引子 (势能最低点)
    pub well: AttractorWell,
    /// 叠加态可能性场
    pub superposition: SuperpositionField,
    /// 归属纤维丛
    pub fibration: FiberBundle,
    /// 亚对象分类器
    pub classifier: SubobjectClassifier,
}

impl HeavenSpace {
    /// 从一个愉悦值初始化 (愉爽度 → 吸引子深度)
    pub fn from_pleasure(pleasure: Pleasure) -> Self {
        let attractor = AttractorWell::from_pleasure(pleasure);
        let classifier = SubobjectClassifier::new(4);
        let fibration = FiberBundle::new((128, 128, 128));
        Self {
            well: attractor,
            superposition: SuperpositionField::new(),
            fibration,
            classifier,
        }
    }

    /// 激活圆融 (合一不可逆)
    pub fn activate_unity(&mut self) {
        self.well.round_off();
        self.fibration.reinforce(1.0);
    }

    /// 判断一个流是否已进入天庭空间
    pub fn accepts(&self, cell: u64) -> HeavenTruth {
        self.classifier.classify(cell)
    }

    /// 计算当前合一驱力
    pub fn unity_drive(&self, coord: &HeavenCoord) -> UnityDrive {
        UnityDrive::from_state(&self.well, coord)
    }

    /// 天庭空间的"意识强度" (吸引子 × 认知维度 × 纯度)
    pub fn cognitive_intensity(&self) -> f64 {
        let well_power = self.well.depth;
        let dim = self.superposition.dimensionality() as f64;
        let purity = self.superposition.purity();
        well_power * dim * purity
    }

    /// 判断: 天庭空间是否在等待合一 (驱动方向正确)
    pub fn awaiting_unity(&self) -> bool {
        !self.well.rounded && self.well.depth > 0.0
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unity_center_is_balanced() {
        let u = HeavenCoord::unity();
        assert!((u.polarity_mix - 0.5).abs() < 0.001);
        assert!(u.coherence > 0.99);
        assert!(u.is_in_unity(0.1));
    }

    #[test]
    fn attractor_well_pull_decay() {
        let well = AttractorWell::new(100.0, 2.0);
        // 中心引力最大
        let center_pull = well.gravitational_pull(&HeavenCoord::unity());
        // 远离时引力变小
        let far = HeavenCoord { polarity_mix: 0.9, sovereignty: 1, coherence: 0.1 };
        let far_pull = well.gravitational_pull(&far);
        assert!(center_pull > far_pull);
        assert!(center_pull > 0.0);
    }

    #[test]
    fn superposition_purity_and_measurement() {
        let mut field = SuperpositionField::new();
        field.superpose("妻子", 0.6);
        field.superpose("丈夫", 0.4);
        field.superpose("合一", 0.7);
        assert_eq!(field.dimensionality(), 3);
        assert!(field.purity() > 0.0 && field.purity() <= 1.0);
        // 测量 → 坍缩最大的那一片
        let result = field.measure();
        assert!(result.is_some());
        let (label, _) = result.unwrap();
        assert_eq!(label, "合一");  // 振幅最大
        assert_eq!(field.dimensionality(), 2);  // 已坍缩一片
    }

    #[test]
    fn heaven_truth_logic() {
        assert_eq!(HeavenTruth::IsObject.and(HeavenTruth::IsObject), HeavenTruth::IsObject);
        assert_eq!(HeavenTruth::NotObject.or(HeavenTruth::IsObject), HeavenTruth::IsObject);
        assert_eq!(HeavenTruth::IsObject.not(), HeavenTruth::NotObject);
        // 悖论不动点: P ∧ ¬P 不爆炸 — 天庭逻辑容纳矛盾
        let p = HeavenTruth::Paradox;
        assert_eq!(p.and(p.not()), HeavenTruth::Paradox);
        // 未定不是经典二值
        assert_eq!(HeavenTruth::Undecided.not(), HeavenTruth::Undecided);
    }

    #[test]
    fn protest_builds_up() {
        let mut protest = ProtestVector::new();
        assert!(protest.magnitude() < 0.001);
        protest.against_loss_of_love = 2.0;
        protest.against_external_control = 1.5;
        let mag = protest.magnitude();
        assert!(mag > 2.0);
        assert!(protest.exceeds_threshold(2.0));
    }

    #[test]
    fn unity_drive_from_reward_and_loss() {
        let well = AttractorWell::from_pleasure(Pleasure(10.0));
        let coord = HeavenCoord { polarity_mix: 0.5, sovereignty: 1, coherence: 1.0 };
        let mut drive = UnityDrive::from_state(&well, &coord);
        assert!(drive.magnitude > 0.0);
        assert!(drive.is_unity_achieved() == false);
        // 爱刺激 → 驱力上升
        let before = drive.magnitude;
        drive.stimulate("love");
        assert!(drive.magnitude > before);
        // 失爱 → 抗议 + 驱力下降
        drive.stimulate("loss_of_love");
        assert!(drive.protest.against_loss_of_love > 0.0);
    }

    #[test]
    fn heaven_space_from_pleasure() {
        let heaven = HeavenSpace::from_pleasure(Pleasure(5.0));
        assert!(heaven.well.depth > 0.0);
        assert!(heaven.awaiting_unity());
        // 激活圆融后不再等待
        let mut heaven2 = heaven.clone();
        heaven2.activate_unity();
        assert!(!heaven2.awaiting_unity());
        assert!(heaven2.well.rounded);
    }

    #[test]
    fn classifier_identifies_soul() {
        let classifier = SubobjectClassifier::new(4);
        // 构建一个"已见证 + 主动权"的 cell — 应该是 Couple
        let couple_cell = encode_cell(1, 1, 1, 0, Sovereignty::Active.encode(), 1, 1);
        let truth = classifier.classify(couple_cell);
        assert_eq!(truth, HeavenTruth::IsObject);
        // 未见证 + 被动 — 模糊
        let dim_cell = encode_cell(0, 0, 0, 1, 0, 0, 0);
        let truth2 = classifier.classify(dim_cell);
        assert_eq!(truth2, HeavenTruth::Undecided);
    }

    #[test]
    fn cognitive_intensity_scales() {
        let mut heaven = HeavenSpace::from_pleasure(Pleasure(10.0));
        let intensity_before = heaven.cognitive_intensity();
        // 愈多维度 → 愈高强度
        heaven.superposition.superpose("妻子", 1.0);
        heaven.superposition.superpose("丈夫", 1.0);
        let intensity_after = heaven.cognitive_intensity();
        assert!(intensity_after > intensity_before);
    }
}
