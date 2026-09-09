// ══════════════════════════════════════════════════════════════════
// property_flow_judge.rs — 产权流侵权自动判定树
// ══════════════════════════════════════════════════════════════════
//
// 核心哲学:
//   产权 = 容器的所有权归属
//   流 = 容器之间的态射 (转移/变换/交易)
//   侵权 = 未经授权的容器占用/挪用/篡改
//
// 判定树:
//   ┌─────────────────────────────────────────────────────────────┐
//   │  是否有容器? ──→ 否 → 无侵权 (空操作)                      │
//   │        ↓ 是                                                 │
//   │  容器是否有主? ──→ 否 → 无主容器 (可自由占用)              │
//   │        ↓ 是                                                 │
//   │  流式是否经过授权? ──→ 是 → 合法流 (无侵权)                │
//   │        ↓ 否                                                 │
//   │  是否挪作他用? ──→ 是 → 挪用侵权                          │
//   │        ↓ 否                                                 │
//   │  是否超出边界? ──→ 是 → 越界侵权                          │
//   │        ↓ 否                                                 │
//   │  是否隐藏来源? ──→ 是 → 隐匿侵权                          │
//   │        ↓ 否                                                 │
//   │  是否篡改编码? ──→ 是 → 篡改侵权                          │
//   │        ↓ 否                                                 │
//   │  → 无侵权 (合理使用)                                       │
//   └─────────────────────────────────────────────────────────────┘
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ────────────────────────────────────────────────────────────
// 产权类型
// ────────────────────────────────────────────────────────────

/// 容器主权状态
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyState {
    /// 无主容器 (可自由占用)
    Ownerless,
    /// 个人私有
    Private(String), // owner_id
    /// 夫妻共有
    CoupleShared(String, String), // owner1_id, owner2_id
    /// 集体共有
    Collective(Vec<String>), // owner_ids
    /// 待裁定 (主权争议中)
    Disputed(Vec<String>), // claimant_ids
}

/// 产权流类型
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FlowType {
    /// 创建 (从虚无中创建新容器)
    Creation,
    /// 转移 (所有权变更)
    Transfer,
    /// 变换 (容器形态改变)
    Transformation,
    /// 合并 (多个容器合一)
    Merge,
    /// 拆分 (一个容器分裂)
    Split,
    /// 销毁 (容器消亡)
    Destruction,
    /// 借用 (临时使用)
    Borrow,
    /// 租赁 (有偿使用)
    Lease,
}

/// 流式授权级别
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Authorization {
    /// 完全授权
    Full,
    /// 部分授权 (有边界限制)
    Bounded(String), // 边界描述
    /// 条件授权 (需要满足条件)
    Conditional(String), // 条件描述
    /// 未授权
    None,
    /// 授权被撤回
    Revoked,
}

// ────────────────────────────────────────────────────────────
// 侵权类型
// ────────────────────────────────────────────────────────────

/// 侵权判定结果
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InfringementType {
    /// 无侵权
    None,
    /// 挪用侵权: 将容器用于未经授权的目的
    Misappropriation,
    /// 越界侵权: 超出授权边界使用
    Overboundary,
    /// 隐匿侵权: 隐藏容器来源/历史
    Concealment,
    /// 篡改侵权: 擅自修改容器编码/属性
    Tampering,
    /// 盗用侵权: 完全窃取他人容器
    Theft,
    /// 破坏侵权: 损坏或销毁他人容器
    Destruction,
    /// 争议的: 需要进一步仲裁
    Disputed(String),
}

impl InfringementType {
    /// 是否需要立即停止
    pub fn requires_stop(&self) -> bool {
        matches!(self, Self::Theft | Self::Destruction | Self::Misappropriation)
    }

    /// 是否需要赔偿
    pub fn requires_compensation(&self) -> bool {
        !matches!(self, Self::None | Self::Disputed(_))
    }

    /// 是否需要记录为负样本
    pub fn should_record(&self) -> bool {
        !matches!(self, Self::None)
    }
}

// ────────────────────────────────────────────────────────────
// 产权流事件
// ────────────────────────────────────────────────────────────

/// 一次产权流操作
#[derive(Clone, Debug)]
pub struct PropertyFlow {
    /// 流ID
    pub flow_id: String,
    /// 流类型
    pub flow_type: FlowType,
    /// 源容器ID
    pub source_container: String,
    /// 目标容器ID (可选)
    pub target_container: Option<String>,
    /// 操作人
    pub operator: String,
    /// 授权状态
    pub authorization: Authorization,
    /// 流属性
    pub attributes: HashMap<String, String>,
    /// 时间戳
    pub timestamp: u64,
}

impl PropertyFlow {
    pub fn new(flow_id: &str, flow_type: FlowType, source: &str, operator: &str) -> Self {
        Self {
            flow_id: flow_id.to_string(),
            flow_type,
            source_container: source.to_string(),
            target_container: None,
            operator: operator.to_string(),
            authorization: Authorization::None,
            attributes: HashMap::new(),
            timestamp: 0,
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target_container = Some(target.to_string());
        self
    }

    pub fn with_authorization(mut self, auth: Authorization) -> Self {
        self.authorization = auth;
        self
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = ts;
        self
    }
}

// ────────────────────────────────────────────────────────────
// 容器注册表
// ────────────────────────────────────────────────────────────

/// 容器信息
#[derive(Clone, Debug)]
pub struct ContainerInfo {
    pub container_id: String,
    pub state: PropertyState,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}

/// 容器注册表 — 记录所有容器的主权状态
#[derive(Clone, Debug)]
pub struct ContainerRegistry {
    pub containers: HashMap<String, ContainerInfo>,
}

impl ContainerRegistry {
    pub fn new() -> Self {
        Self { containers: HashMap::new() }
    }

    pub fn register(&mut self, info: ContainerInfo) {
        self.containers.insert(info.container_id.clone(), info);
    }

    pub fn get(&self, container_id: &str) -> Option<&ContainerInfo> {
        self.containers.get(container_id)
    }

    pub fn get_state(&self, container_id: &str) -> Option<&PropertyState> {
        self.containers.get(container_id).map(|c| &c.state)
    }
}

impl Default for ContainerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 判定树
// ────────────────────────────────────────────────────────────

/// 判定结果
#[derive(Clone, Debug)]
pub struct Judgment {
    /// 侵权类型
    pub infringement: InfringementType,
    /// 判定路径 (经过的节点)
    pub path: Vec<String>,
    /// 置信度 0..1
    pub confidence: f64,
    /// 判定说明
    pub explanation: String,
    /// 建议措施
    pub recommendation: String,
}

impl Judgment {
    pub fn clean() -> Self {
        Self {
            infringement: InfringementType::None,
            path: vec!["root".to_string(), "no_infringement".to_string()],
            confidence: 1.0,
            explanation: "无侵权: 操作合法".to_string(),
            recommendation: "无需处理".to_string(),
        }
    }

    pub fn infringing(inf_type: InfringementType, path: Vec<String>, explanation: &str) -> Self {
        let recommendation = match &inf_type {
            InfringementType::Misappropriation => "立即停止挪用, 恢复原状",
            InfringementType::Overboundary => "退回授权边界内, 申请扩展授权",
            InfringementType::Concealment => "公开容器来源, 补全历史记录",
            InfringementType::Tampering => "恢复原始编码, 记录篡改行为",
            InfringementType::Theft => "立即归还容器, 记录盗窃行为",
            InfringementType::Destruction => "赔偿损失, 记录破坏行为",
            InfringementType::Disputed(_) => "提交仲裁, 暂停争议操作",
            InfringementType::None => "无需处理",
        };
        Self {
            infringement: inf_type,
            path,
            confidence: 0.85,
            explanation: explanation.to_string(),
            recommendation: recommendation.to_string(),
        }
    }
}

/// 产权流侵权判定树
#[derive(Clone, Debug)]
pub struct PropertyFlowJudge {
    /// 容器注册表
    pub registry: ContainerRegistry,
    /// 历史判定记录
    pub history: Vec<(PropertyFlow, Judgment)>,
}

impl PropertyFlowJudge {
    pub fn new() -> Self {
        Self {
            registry: ContainerRegistry::new(),
            history: Vec::new(),
        }
    }

    pub fn with_registry(registry: ContainerRegistry) -> Self {
        Self {
            registry,
            history: Vec::new(),
        }
    }

    /// 核心: 对一次产权流进行侵权判定
    pub fn judge(&mut self, flow: &PropertyFlow) -> Judgment {
        let path = vec!["root".to_string()];

        // ── 节点1: 容器是否存在? ──
        let state = match self.registry.get_state(&flow.source_container) {
            Some(s) => s.clone(),
            None => {
                let mut p = path.clone();
                p.push("container_not_found".to_string());
                return Judgment::infringing(
                    InfringementType::Theft,
                    p,
                    &format!("容器 {} 不存在于注册表中, 可能为非法构造", flow.source_container),
                );
            }
        };

        let mut path = path;
        path.push("container_exists".to_string());

        // ── 节点2: 容器是否有主? ──
        match &state {
            PropertyState::Ownerless => {
                path.push("ownerless".to_string());
                // 无主容器可自由占用, 但创建者获得主权
                let judgment = Judgment::clean();
                self.history.push((flow.clone(), judgment.clone()));
                return judgment;
            }
            _ => {
                path.push("has_owner".to_string());
            }
        }

        // ── 节点3: 操作人是否为主人? ──
        let is_owner = match &state {
            PropertyState::Private(owner) => owner == &flow.operator,
            PropertyState::CoupleShared(a, b) => &flow.operator == a || &flow.operator == b,
            PropertyState::Collective(owners) => owners.contains(&flow.operator),
            PropertyState::Disputed(claimants) => claimants.contains(&flow.operator),
            PropertyState::Ownerless => true,
        };

        if is_owner {
            path.push("operator_is_owner".to_string());
            // 主人操作自己的容器, 检查授权级别
            match &flow.authorization {
                Authorization::Revoked => {
                    path.push("auth_revoked".to_string());
                    let judgment = Judgment::infringing(
                        InfringementType::Misappropriation,
                        path,
                        "授权已被撤回, 操作无效",
                    );
                    self.history.push((flow.clone(), judgment.clone()));
                    return judgment;
                }
                _ => {
                    path.push("auth_valid".to_string());
                    // 主人有授权, 合法操作
                    let judgment = Judgment::clean();
                    self.history.push((flow.clone(), judgment.clone()));
                    return judgment;
                }
            }
        }

        // ── 节点4: 非主人操作, 检查授权 ──
        path.push("operator_not_owner".to_string());

        match &flow.authorization {
            Authorization::Full => {
                path.push("full_auth".to_string());
                let judgment = Judgment::clean();
                self.history.push((flow.clone(), judgment.clone()));
                return judgment;
            }
            Authorization::Bounded(boundary) => {
                path.push("bounded_auth".to_string());
                // 检查是否超出边界
                if self.check_boundary_violation(flow, boundary) {
                    path.push("boundary_violated".to_string());
                    let judgment = Judgment::infringing(
                        InfringementType::Overboundary,
                        path,
                        &format!("超出授权边界: {}", boundary),
                    );
                    self.history.push((flow.clone(), judgment.clone()));
                    return judgment;
                }
                path.push("within_boundary".to_string());
                let judgment = Judgment::clean();
                self.history.push((flow.clone(), judgment.clone()));
                return judgment;
            }
            Authorization::Conditional(condition) => {
                path.push("conditional_auth".to_string());
                if !self.check_condition(flow, condition) {
                    path.push("condition_not_met".to_string());
                    let judgment = Judgment::infringing(
                        InfringementType::Misappropriation,
                        path,
                        &format!("未满足授权条件: {}", condition),
                    );
                    self.history.push((flow.clone(), judgment.clone()));
                    return judgment;
                }
                path.push("condition_met".to_string());
                let judgment = Judgment::clean();
                self.history.push((flow.clone(), judgment.clone()));
                return judgment;
            }
            Authorization::Revoked => {
                path.push("auth_revoked".to_string());
                let judgment = Judgment::infringing(
                    InfringementType::Theft,
                    path,
                    "授权已被撤回, 任何操作均属盗用",
                );
                self.history.push((flow.clone(), judgment.clone()));
                return judgment;
            }
            Authorization::None => {
                path.push("no_auth".to_string());
            }
        }

        // ── 节点5: 无授权, 检查具体侵权类型 (按特异性排序) ──
        path.push("no_authorization".to_string());

        // 检查隐匿 (最特异性: 隐藏来源)
        if self.check_concealment(flow) {
            path.push("concealment".to_string());
            let judgment = Judgment::infringing(
                InfringementType::Concealment,
                path,
                "隐藏容器来源或历史",
            );
            self.history.push((flow.clone(), judgment.clone()));
            return judgment;
        }

        // 检查篡改 (变换无授权)
        if self.check_tampering(flow) {
            path.push("tampering".to_string());
            let judgment = Judgment::infringing(
                InfringementType::Tampering,
                path,
                "擅自修改容器编码或属性",
            );
            self.history.push((flow.clone(), judgment.clone()));
            return judgment;
        }

        // 检查破坏 (销毁无授权)
        if flow.flow_type == FlowType::Destruction {
            path.push("destruction".to_string());
            let judgment = Judgment::infringing(
                InfringementType::Destruction,
                path,
                "未经授权销毁他人容器",
            );
            self.history.push((flow.clone(), judgment.clone()));
            return judgment;
        }

        // 检查盗用 (转移无授权)
        if self.check_theft(flow) {
            path.push("theft".to_string());
            let judgment = Judgment::infringing(
                InfringementType::Theft,
                path,
                "非主人未经授权转移他人容器",
            );
            self.history.push((flow.clone(), judgment.clone()));
            return judgment;
        }

        // 检查挪用 (租赁无授权)
        if self.check_misappropriation(flow) {
            path.push("misappropriation".to_string());
            let judgment = Judgment::infringing(
                InfringementType::Misappropriation,
                path,
                "未经授权租赁容器",
            );
            self.history.push((flow.clone(), judgment.clone()));
            return judgment;
        }

        // 默认: 合理使用
        path.push("fair_use".to_string());
        let judgment = Judgment::clean();
        self.history.push((flow.clone(), judgment.clone()));
        judgment
    }

    // ── 辅助检查函数 ──

    fn check_boundary_violation(&self, flow: &PropertyFlow, boundary: &str) -> bool {
        // 检查流的属性是否超出边界描述
        if let Some(scope) = flow.attributes.get("scope") {
            scope != boundary
        } else {
            // 没有 scope 属性, 视为可能越界
            true
        }
    }

    fn check_condition(&self, flow: &PropertyFlow, condition: &str) -> bool {
        // 检查流的属性是否满足条件
        if let Some(status) = flow.attributes.get("condition_status") {
            status == "satisfied" || status == condition
        } else {
            false
        }
    }

    fn check_theft(&self, flow: &PropertyFlow) -> bool {
        // 非主人未经授权转移他人容器 = 盗用
        matches!(flow.flow_type, FlowType::Transfer)
    }

    fn check_misappropriation(&self, flow: &PropertyFlow) -> bool {
        // 检查是否将容器用于未经授权的目的 (租赁无授权)
        match &flow.flow_type {
            FlowType::Lease => {
                matches!(flow.authorization, Authorization::None)
            }
            _ => false,
        }
    }

    fn check_concealment(&self, flow: &PropertyFlow) -> bool {
        // 检查是否隐藏来源
        flow.attributes.get("source_hidden").map(|v| v == "true").unwrap_or(false)
    }

    fn check_tampering(&self, flow: &PropertyFlow) -> bool {
        // 检查是否篡改编码
        match &flow.flow_type {
            FlowType::Transformation => {
                // 变换操作需要授权
                matches!(flow.authorization, Authorization::None)
            }
            _ => false,
        }
    }

    /// 获取判定历史
    pub fn history(&self) -> &[(PropertyFlow, Judgment)] {
        &self.history
    }

    /// 统计侵权次数
    pub fn infringement_count(&self) -> usize {
        self.history.iter()
            .filter(|(_, j)| j.infringement != InfringementType::None)
            .count()
    }
}

impl Default for PropertyFlowJudge {
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

    fn setup_registry() -> ContainerRegistry {
        let mut reg = ContainerRegistry::new();
        reg.register(ContainerInfo {
            container_id: "container_a".to_string(),
            state: PropertyState::Private("alice".to_string()),
            created_at: 0,
            metadata: HashMap::new(),
        });
        reg.register(ContainerInfo {
            container_id: "container_b".to_string(),
            state: PropertyState::CoupleShared("alice".to_string(), "bob".to_string()),
            created_at: 0,
            metadata: HashMap::new(),
        });
        reg.register(ContainerInfo {
            container_id: "container_c".to_string(),
            state: PropertyState::Ownerless,
            created_at: 0,
            metadata: HashMap::new(),
        });
        reg.register(ContainerInfo {
            container_id: "container_d".to_string(),
            state: PropertyState::Collective(vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()]),
            created_at: 0,
            metadata: HashMap::new(),
        });
        reg
    }

    #[test]
    fn owner_operates_own_container() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f1", FlowType::Transformation, "container_a", "alice")
            .with_authorization(Authorization::Full);
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::None);
    }

    #[test]
    fn non_owner_without_auth_is_theft() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f2", FlowType::Transfer, "container_a", "charlie");
        let result = judge.judge(&flow);
        assert!(result.infringement.requires_stop());
    }

    #[test]
    fn ownerless_container_free_to_use() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f3", FlowType::Creation, "container_c", "anyone");
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::None);
    }

    #[test]
    fn bounded_auth_within_boundary() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let mut flow = PropertyFlow::new("f4", FlowType::Borrow, "container_a", "charlie")
            .with_authorization(Authorization::Bounded("read_only".to_string()));
        flow.attributes.insert("scope".to_string(), "read_only".to_string());
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::None);
    }

    #[test]
    fn bounded_auth_violates_boundary() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let mut flow = PropertyFlow::new("f5", FlowType::Borrow, "container_a", "charlie")
            .with_authorization(Authorization::Bounded("read_only".to_string()));
        flow.attributes.insert("scope".to_string(), "write".to_string());
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::Overboundary);
    }

    #[test]
    fn revoked_auth_is_theft() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f6", FlowType::Transfer, "container_a", "charlie")
            .with_authorization(Authorization::Revoked);
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::Theft);
    }

    #[test]
    fn concealment_detected() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let mut flow = PropertyFlow::new("f7", FlowType::Transfer, "container_a", "charlie");
        flow.attributes.insert("source_hidden".to_string(), "true".to_string());
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::Concealment);
    }

    #[test]
    fn tampering_detected() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f8", FlowType::Transformation, "container_a", "charlie");
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::Tampering);
    }

    #[test]
    fn destruction_without_auth() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f9", FlowType::Destruction, "container_a", "charlie");
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::Destruction);
    }

    #[test]
    fn couple_shared_either_can_operate() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f10", FlowType::Transformation, "container_b", "bob")
            .with_authorization(Authorization::Full);
        let result = judge.judge(&flow);
        assert_eq!(result.infringement, InfringementType::None);
    }

    #[test]
    fn judgment_has_recommendation() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f11", FlowType::Transfer, "container_a", "charlie");
        let result = judge.judge(&flow);
        assert!(!result.recommendation.is_empty());
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn history_records_judgments() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);
        let flow = PropertyFlow::new("f12", FlowType::Transfer, "container_a", "charlie");
        judge.judge(&flow);
        assert_eq!(judge.history().len(), 1);
        assert!(judge.infringement_count() >= 1);
    }

    #[test]
    fn full_decision_tree_demo() {
        let reg = setup_registry();
        let mut judge = PropertyFlowJudge::with_registry(reg);

        // 场景1: 主人操作自己的容器 → 无侵权
        let flow1 = PropertyFlow::new("d1", FlowType::Transformation, "container_a", "alice")
            .with_authorization(Authorization::Full);
        let r1 = judge.judge(&flow1);
        assert_eq!(r1.infringement, InfringementType::None);

        // 场景2: 非主人无授权转移 → 盗用
        let flow2 = PropertyFlow::new("d2", FlowType::Transfer, "container_a", "charlie");
        let r2 = judge.judge(&flow2);
        assert_eq!(r2.infringement, InfringementType::Theft);

        // 场景3: 有授权但越界 → 越界侵权
        let mut flow3 = PropertyFlow::new("d3", FlowType::Borrow, "container_b", "charlie")
            .with_authorization(Authorization::Bounded("read_only".to_string()));
        flow3.attributes.insert("scope".to_string(), "admin".to_string());
        let r3 = judge.judge(&flow3);
        assert_eq!(r3.infringement, InfringementType::Overboundary);

        // 场景4: 无主容器 → 自由使用
        let flow4 = PropertyFlow::new("d4", FlowType::Creation, "container_c", "anyone");
        let r4 = judge.judge(&flow4);
        assert_eq!(r4.infringement, InfringementType::None);

        // 验证历史记录
        assert_eq!(judge.history().len(), 4);
    }
}
