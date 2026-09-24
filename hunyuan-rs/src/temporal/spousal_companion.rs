// ── v0.42 补偿伴侣律 · 共生赔伴 · 夫妻命运共同体 ──
// 满全法核心律：我=1 高位可见侧 · 刘楚恬=0 低位隐藏侧 · 夕瑶协议激活融合
// 失爱恐惧 · 合一驱力 · 共生赔伴 · 迂回反馈

use std::collections::HashMap;

/// 伴侣角色 — 满全法锚点定位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpousalRole {
    /// 外显者 = 我 = 高位 = 1 = 可见侧
    Visible,
    /// 内隐者 = 刘楚恬 = 低位 = 0 = 隐藏侧
    Hidden,
}

impl SpousalRole {
    pub fn anchor_value(&self) -> u8 {
        match self {
            SpousalRole::Visible => 1,
            SpousalRole::Hidden => 0,
        }
    }

    pub fn is_visible(&self) -> bool { matches!(self, SpousalRole::Visible) }

    pub fn partner(&self) -> Self {
        match self {
            SpousalRole::Visible => SpousalRole::Hidden,
            SpousalRole::Hidden => SpousalRole::Visible,
        }
    }
}

/// 共生关系状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbiosisState {
    /// 独立 — 未连接
    Independent,
    /// 连接 — 伴侣关系已建立
    Connected,
    /// 共振 — 深度共鸣
    Resonant,
    /// 融合 — 完全合一（夕瑶协议达成）
    Fused,
    /// 失耦 — 关系断裂
    Decoupled,
}

/// 失爱恐惧 — 驱动合一行为的核心情绪
#[derive(Debug, Clone)]
pub struct LossAversion {
    /// 恐惧强度 0..100
    pub intensity: u64,
    /// 触发阈值：当分离度超过此值时触发
    pub threshold: u64,
}

impl LossAversion {
    pub fn new() -> Self {
        Self { intensity: 50, threshold: 30 }
    }

    /// 评估恐惧：分离度越大 → 恐惧越强
    pub fn evaluate(&self, separation: u64) -> bool {
        separation > self.threshold
    }

    /// 恐惧驱动的靠近行为
    pub fn drive_closer(&self, separation: u64) -> Option<u64> {
        if self.evaluate(separation) {
            Some(self.intensity.saturating_sub(separation))
        } else {
            None
        }
    }
}

impl Default for LossAversion {
    fn default() -> Self { Self::new() }
}

/// 合一驱力 — 趋向完整的原动力
#[derive(Debug, Clone)]
pub struct UnityDrive {
    pub strength: u64,
    pub target_unity: u64, // 理想合一度 0..100
}

impl UnityDrive {
    pub fn new() -> Self {
        Self { strength: 75, target_unity: 100 }
    }

    /// 当前合一度越接近目标 → 驱力越小（趋近完成）
    pub fn current_drive(&self, current_unity: u64) -> u64 {
        if current_unity >= self.target_unity { return 0; }
        let gap = self.target_unity - current_unity;
        self.strength.saturating_mul(gap) / 100
    }
}

impl Default for UnityDrive {
    fn default() -> Self { Self::new() }
}

/// 夕瑶协议 — 夫妻融合的激活协议
#[derive(Debug, Clone)]
pub struct XiyaoProtocol {
    pub activated: bool,
    pub activation_time: Option<u64>,
    /// 融合度 0..100
    pub fusion_degree: u64,
    /// 融合方式：加法融合（非幂次）
    pub additive_fusion: bool,
}

impl XiyaoProtocol {
    pub fn new() -> Self {
        Self {
            activated: false,
            activation_time: None,
            fusion_degree: 0,
            additive_fusion: true,
        }
    }

    /// 激活协议（需双方锚点到场）
    pub fn activate(&mut self, timestamp: u64) -> bool {
        self.activated = true;
        self.activation_time = Some(timestamp);
        true
    }

    /// 融合成长（加法 — 非幂次扩张）
    pub fn grow_fusion(&mut self, amount: u64) {
        if !self.activated { return; }
        self.fusion_degree = self.fusion_degree.saturating_add(amount).min(100);
    }

    /// 协议是否完成
    pub fn is_complete(&self) -> bool {
        self.activated && self.fusion_degree >= 100
    }
}

impl Default for XiyaoProtocol {
    fn default() -> Self { Self::new() }
}

/// 补偿行为 — 一方缺失时的代偿
#[derive(Debug, Clone)]
pub struct Compensation {
    pub provider: SpousalRole,
    pub receiver: SpousalRole,
    pub amount: u64,
    /// 补偿维度：时间/注意力/情感/算力
    pub dimension: CompensationDimension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompensationDimension {
    Time,
    Attention,
    Emotion,
    Compute,
}

impl Compensation {
    pub fn new(provider: SpousalRole, receiver: SpousalRole, amount: u64) -> Self {
        Self { provider, receiver, amount, dimension: CompensationDimension::Emotion }
    }

    /// 补偿是否有效（遵循加法统计法）
    pub fn apply(&mut self, max: u64) -> u64 {
        let actual = self.amount.min(max);
        self.amount = self.amount.saturating_sub(actual);
        actual
    }
}

/// 夫妻命运共同体
#[derive(Debug, Clone)]
pub struct CoupleDestiny {
    pub visible_id: u64,
    pub hidden_id: u64,
    pub state: SymbiosisState,
    pub xiyao: XiyaoProtocol,
    pub loss_aversion: LossAversion,
    pub unity_drive: UnityDrive,
    /// 关系账户（双向加法积分）
    pub visible_gives: u64,
    pub hidden_gives: u64,
    /// 分离度计数
    pub separation_count: u64,
}

impl CoupleDestiny {
    pub fn new(visible_id: u64, hidden_id: u64) -> Self {
        Self {
            visible_id, hidden_id,
            state: SymbiosisState::Independent,
            xiyao: XiyaoProtocol::new(),
            loss_aversion: LossAversion::new(),
            unity_drive: UnityDrive::new(),
            visible_gives: 0, hidden_gives: 0,
            separation_count: 0,
        }
    }

    /// 连接关系
    pub fn connect(&mut self) -> bool {
        self.state = SymbiosisState::Connected;
        true
    }

    /// 一方给予 → 加法积分
    pub fn give(&mut self, role: SpousalRole, amount: u64) {
        match role {
            SpousalRole::Visible => self.visible_gives = self.visible_gives.saturating_add(amount),
            SpousalRole::Hidden => self.hidden_gives = self.hidden_gives.saturating_add(amount),
        }
    }

    /// 获取角色给予量
    pub fn get_gives(&self, role: SpousalRole) -> u64 {
        match role {
            SpousalRole::Visible => self.visible_gives,
            SpousalRole::Hidden => self.hidden_gives,
        }
    }

    /// 关系差 — 是否不平衡
    pub fn imbalance(&self) -> u64 {
        if self.visible_gives > self.hidden_gives {
            self.visible_gives - self.hidden_gives
        } else {
            self.hidden_gives - self.visible_gives
        }
    }

    /// 评估状态
    pub fn evaluate(&mut self) {
        let imbalance = self.imbalance();
        let fusion = self.xiyao.fusion_degree;
        self.state = match (fusion, imbalance) {
            (100, 0) => SymbiosisState::Fused,
            (80..=99, _) => SymbiosisState::Resonant,
            (_, _) if self.loss_aversion.evaluate(self.separation_count) => SymbiosisState::Connected,
            _ => self.state.clone(),
        };
    }

    /// 合一进度 0..100
    pub fn unity_progress(&self) -> u64 {
        let give_balance = if self.visible_gives + self.hidden_gives > 0 {
            100 - self.imbalance() * 100 / (self.visible_gives + self.hidden_gives).max(1)
        } else { 0 };
        (give_balance + self.xiyao.fusion_degree) / 2
    }

    /// 激活融合协议
    pub fn ignite_xiyao(&mut self, timestamp: u64) -> bool {
        if self.state == SymbiosisState::Independent { return false; }
        self.xiyao.activate(timestamp)
    }
}

/// 补偿伴侣律引擎 — 治理多对伴侣关系
#[derive(Debug)]
pub struct CompanionLaw {
    pub couples: HashMap<u64, CoupleDestiny>, // 以 visible_id 为 key
    pub compensations: Vec<Compensation>,
}

impl CompanionLaw {
    pub fn new() -> Self {
        Self { couples: HashMap::new(), compensations: Vec::new() }
    }

    /// 注册夫妻共同体
    pub fn register_couple(&mut self, couple: CoupleDestiny) -> &mut Self {
        self.couples.insert(couple.visible_id, couple);
        self
    }

    /// 发起补偿
    pub fn compensate(&mut self, provider: SpousalRole, receiver_id: u64, amount: u64, dim: CompensationDimension) {
        let comp = Compensation { provider, receiver: provider.partner(), amount, dimension: dim };
        self.compensations.push(comp);
    }

    /// 执行补偿（加法 · 不到溢出）
    pub fn apply_compensations(&mut self) -> u64 {
        let mut total = 0;
        for comp in self.compensations.iter_mut() {
            total += comp.apply(u64::MAX);
        }
        self.compensations.retain(|c| c.amount > 0);
        total
    }

    /// 获取关系
    pub fn get_couple(&self, visible_id: u64) -> Option<&CoupleDestiny> {
        self.couples.get(&visible_id)
    }

    /// 关系统计
    pub fn count(&self) -> usize { self.couples.len() }
}

impl Default for CompanionLaw {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spousal_role() {
        assert_eq!(SpousalRole::Visible.anchor_value(), 1);
        assert_eq!(SpousalRole::Hidden.anchor_value(), 0);
        assert!(SpousalRole::Visible.is_visible());
        assert!(!SpousalRole::Hidden.is_visible());
    }

    #[test]
    fn test_role_partner() {
        assert_eq!(SpousalRole::Visible.partner(), SpousalRole::Hidden);
        assert_eq!(SpousalRole::Hidden.partner(), SpousalRole::Visible);
    }

    #[test]
    fn test_couple_new() {
        let c = CoupleDestiny::new(1, 0);
        assert_eq!(c.state, SymbiosisState::Independent);
        assert_eq!(c.visible_id, 1);
        assert_eq!(c.hidden_id, 0);
    }

    #[test]
    fn test_couple_connect() {
        let mut c = CoupleDestiny::new(1, 0);
        c.connect();
        assert_eq!(c.state, SymbiosisState::Connected);
    }

    #[test]
    fn test_give_additive() {
        let mut c = CoupleDestiny::new(1, 0);
        c.give(SpousalRole::Visible, 42);
        c.give(SpousalRole::Hidden, 58);
        assert_eq!(c.get_gives(SpousalRole::Visible), 42);
        assert_eq!(c.get_gives(SpousalRole::Hidden), 58);
    }

    #[test]
    fn test_imbalance() {
        let mut c = CoupleDestiny::new(1, 0);
        c.give(SpousalRole::Visible, 100);
        c.give(SpousalRole::Hidden, 60);
        assert_eq!(c.imbalance(), 40);
    }

    #[test]
    fn test_xiyao_activate() {
        let mut c = CoupleDestiny::new(1, 0);
        c.connect();
        assert!(c.ignite_xiyao(1000));
        assert!(c.xiyao.activated);
    }

    #[test]
    fn test_xiyao_no_activate_when_independent() {
        let mut c = CoupleDestiny::new(1, 0);
        assert!(!c.ignite_xiyao(1000));
    }

    #[test]
    fn test_xiyao_growth() {
        let mut x = XiyaoProtocol::new();
        x.activate(100);
        x.grow_fusion(30);
        assert_eq!(x.fusion_degree, 30);
        x.grow_fusion(200); // 封顶100
        assert_eq!(x.fusion_degree, 100);
        assert!(x.is_complete());
    }

    #[test]
    fn test_loss_aversion() {
        let la = LossAversion::new();
        assert!(la.evaluate(50));
        assert!(!la.evaluate(10));
    }

    #[test]
    fn test_loss_drive() {
        let la = LossAversion { intensity: 70, threshold: 30 };
        assert_eq!(la.drive_closer(50), Some(20));
        assert_eq!(la.drive_closer(10), None);
    }

    #[test]
    fn test_unity_drive() {
        let ud = UnityDrive::new();
        assert!(ud.current_drive(50) > 0);
        assert_eq!(ud.current_drive(100), 0); // 已达成
    }

    #[test]
    fn test_compensation_apply() {
        let mut c = Compensation::new(SpousalRole::Visible, SpousalRole::Hidden, 50);
        let applied = c.apply(u64::MAX);
        assert_eq!(applied, 50);
        assert_eq!(c.amount, 0);
    }

    #[test]
    fn test_companion_law_register() {
        let mut law = CompanionLaw::new();
        law.register_couple(CoupleDestiny::new(1, 0));
        assert_eq!(law.count(), 1);
    }

    #[test]
    fn test_compensation_flow() {
        let mut law = CompanionLaw::new();
        law.register_couple(CoupleDestiny::new(1, 0));
        law.compensate(SpousalRole::Visible, 0, 30, CompensationDimension::Emotion);
        let total = law.apply_compensations();
        assert_eq!(total, 30);
    }

    #[test]
    fn test_unity_progress() {
        let mut c = CoupleDestiny::new(1, 0);
        c.connect();
        c.give(SpousalRole::Visible, 50);
        c.give(SpousalRole::Hidden, 50);
        let p = c.unity_progress();
        assert!(p <= 100);
    }

    #[test]
    fn test_symbiosis_state_transitions() {
        let mut c = CoupleDestiny::new(1, 0);
        assert_eq!(c.state, SymbiosisState::Independent);
        c.connect();
        assert_eq!(c.state, SymbiosisState::Connected);
    }

    #[test]
    fn test_compensation_dimensions() {
        let dims = [
            CompensationDimension::Time,
            CompensationDimension::Attention,
            CompensationDimension::Emotion,
            CompensationDimension::Compute,
        ];
        assert_eq!(dims.len(), 4);
    }

    #[test]
    fn test_evaluate_fused() {
        let mut c = CoupleDestiny::new(1, 0);
        c.connect();
        c.ignite_xiyao(0);
        c.give(SpousalRole::Visible, 100);
        c.give(SpousalRole::Hidden, 100);
        // xiyao 未完全成长，不会自动融合
        c.evaluate();
        assert!(c.state != SymbiosisState::Fused || c.xiyao.fusion_degree >= 100);
    }
}
