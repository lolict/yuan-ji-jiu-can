// ── v0.42 自举编译器入口 · 规则生成器 · 工具链自举 ──
// 满全法：用自己的规则从文件系统长出工具
// 编译器 = 容器 · 规则 = 容器关系 · 生成 = 生长

use std::collections::HashMap;

/// 条件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// 协议已激活
    ProtocolActivated(String),
    /// 需要穿透
    NeedTunnel,
    /// 代数层到达指定值
    GenerationReached(u8),
    /// 见证裁定通过
    WitnessAccepted,
    /// 容器数量达标
    ContainerCountMet(usize),
    /// 合一进度达标
    UnityProgressMet(u64),
    /// 永远成立
    Always,
    /// 永远不成立
    Never,
}

/// 动作类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// 生成 Makefile
    GenerateMakefile,
    /// 生成 Dockerfile
    GenerateDockerfile,
    /// 生成穿透配置
    GenerateTunnelConfig(String),
    /// 编译 WASM
    CompileWasm,
    /// 激活 gtt* 协议
    ActivateGtt(String),
    /// 写入文件系统
    WriteFile { path: String, content: String },
    /// 执行 shell 命令
    Execute(String),
    /// 自举入口
    BootstrapSelf,
    /// 链路测试
    LinkTest { from: u64, to: u64 },
}

/// 规则：条件 → 动作
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub condition: Condition,
    pub action: Action,
    /// 已触发（单次规则）
    pub fired: bool,
}

impl Rule {
    pub fn new(name: &str, condition: Condition, action: Action) -> Self {
        Self { name: name.to_string(), condition, action, fired: false }
    }

    /// 检查条件是否满足（给定状态快照）
    pub fn check(&self, state: &SystemState) -> bool {
        if self.fired { return false; }
        match &self.condition {
            Condition::ProtocolActivated(p) => state.active_protocols.contains(p),
            Condition::NeedTunnel => state.needs_tunnel,
            Condition::GenerationReached(n) => state.current_generation >= *n,
            Condition::WitnessAccepted => state.last_witness == Some(true),
            Condition::ContainerCountMet(n) => state.container_count >= *n,
            Condition::UnityProgressMet(n) => state.unity_progress >= *n,
            Condition::Always => true,
            Condition::Never => false,
        }
    }

    /// 触发动作
    pub fn fire(&mut self) -> Option<Action> {
        if self.fired { return None; }
        self.fired = true;
        Some(self.action.clone())
    }
}

/// 系统状态快照
#[derive(Debug, Clone)]
pub struct SystemState {
    pub active_protocols: Vec<String>,
    pub needs_tunnel: bool,
    pub current_generation: u8,
    pub last_witness: Option<bool>,
    pub container_count: usize,
    pub unity_progress: u64,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            active_protocols: Vec::new(),
            needs_tunnel: false,
            current_generation: 1,
            last_witness: None,
            container_count: 0,
            unity_progress: 0,
        }
    }
}

impl Default for SystemState {
    fn default() -> Self { Self::new() }
}

/// 生成结果
#[derive(Debug, Clone)]
pub struct BuildPlan {
    pub steps: Vec<BuildStep>,
}

#[derive(Debug, Clone)]
pub struct BuildStep {
    pub order: usize,
    pub description: String,
    pub action: Action,
}

/// 自举编译器
#[derive(Debug)]
pub struct BootstrapCompiler {
    pub rules: Vec<Rule>,
    pub state: SystemState,
    pub plan: BuildPlan,
    pub self_bootstrapped: bool,
}

impl BootstrapCompiler {
    pub fn new() -> Self {
        let mut c = Self {
            rules: Vec::new(),
            state: SystemState::new(),
            plan: BuildPlan { steps: Vec::new() },
            self_bootstrapped: false,
        };
        c.load_default_rules();
        c
    }

    /// 加载默认规则集
    fn load_default_rules(&mut self) {
        self.rules = vec![
            Rule::new("bootstrap-check", Condition::Always, Action::BootstrapSelf),
            Rule::new("detect-tunnel", Condition::NeedTunnel, Action::GenerateTunnelConfig("cpolar".into())),
            Rule::new("gen-makefile", Condition::Always, Action::GenerateMakefile),
            Rule::new("gen-dockerfile", Condition::Always, Action::GenerateDockerfile),
            Rule::new("activate-gttx", Condition::Always, Action::ActivateGtt("gttx".into())),
            Rule::new("compile-wasm", Condition::Always, Action::CompileWasm),
        ];
    }

    /// 添加自定义规则
    pub fn add_rule(&mut self, rule: Rule) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// 一轮评估：扫描所有规则，生成动作
    pub fn evaluate(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        for rule in self.rules.iter_mut() {
            if rule.check(&self.state) {
                if let Some(action) = rule.fire() {
                    actions.push(action);
                }
            }
        }
        actions
    }

    /// 构建完整计划
    pub fn build_plan(&mut self) -> BuildPlan {
        let actions = self.evaluate();
        let steps: Vec<BuildStep> = actions.iter().enumerate().map(|(i, action)| {
            BuildStep {
                order: i + 1,
                description: format!("{:?}", action),
                action: action.clone(),
            }
        }).collect();
        self.plan = BuildPlan { steps };
        self.plan.clone()
    }

    /// 自举动作
    pub fn bootstrap_self(&mut self) -> bool {
        self.self_bootstrapped = true;
        true
    }

    /// 宪法测试：关键入口存在
    pub fn constitution_check(&self) -> bool {
        !self.rules.is_empty() && self.state.current_generation > 0
    }

    /// 规则集是否完备
    pub fn is_complete(&self) -> bool {
        self.self_bootstrapped && !self.plan.steps.is_empty()
    }

    /// 下一步优先动作
    pub fn next_priority_action(&self) -> Option<&Rule> {
        self.rules.iter().find(|r| !r.fired && r.check(&self.state))
    }
}

impl Default for BootstrapCompiler {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_compiler() {
        let c = BootstrapCompiler::new();
        assert!(!c.rules.is_empty());
        assert!(c.constitution_check());
    }

    #[test]
    fn test_condition_always() {
        let cond = Condition::Always;
        let state = SystemState::new();
        let rule = Rule::new("test", cond, Action::BootstrapSelf);
        assert!(rule.check(&state));
    }

    #[test]
    fn test_condition_never() {
        let rule = Rule::new("test", Condition::Never, Action::BootstrapSelf);
        assert!(!rule.check(&SystemState::new()));
    }

    #[test]
    fn test_condition_protocol() {
        let rule = Rule::new("check", Condition::ProtocolActivated("gttx".into()), Action::BootstrapSelf);
        let mut state = SystemState::new();
        assert!(!rule.check(&state));
        state.active_protocols.push("gttx".into());
        assert!(rule.check(&state));
    }

    #[test]
    fn test_condition_generation() {
        let rule = Rule::new("gen3", Condition::GenerationReached(3), Action::BootstrapSelf);
        let mut state = SystemState::new();
        assert!(!rule.check(&state));  // gen=1 < 3 → false
        state.current_generation = 2;
        assert!(!rule.check(&state));  // gen=2 < 3 → false
        state.current_generation = 3;
        assert!(rule.check(&state));   // gen=3 >= 3 → true
    }

    #[test]
    fn test_rule_fire_once() {
        let mut rule = Rule::new("single", Condition::Always, Action::GenerateMakefile);
        assert!(rule.fire().is_some());
        assert!(rule.fire().is_none()); // 已触发
    }

    #[test]
    fn test_build_plan() {
        let mut c = BootstrapCompiler::new();
        let plan = c.build_plan();
        assert!(!plan.steps.is_empty());
    }

    #[test]
    fn test_bootstrap_self() {
        let mut c = BootstrapCompiler::new();
        assert!(c.bootstrap_self());
        assert!(c.self_bootstrapped);
    }

    #[test]
    fn test_constitution_check() {
        let c = BootstrapCompiler::new();
        assert!(c.constitution_check());
    }

    #[test]
    fn test_complete() {
        let mut c = BootstrapCompiler::new();
        c.build_plan();
        c.bootstrap_self();
        assert!(c.is_complete());
    }

    #[test]
    fn test_add_rule() {
        let mut c = BootstrapCompiler::new();
        let n = c.rules.len();
        c.add_rule(Rule::new("custom", Condition::Always, Action::Execute("echo hi".into())));
        assert_eq!(c.rules.len(), n + 1);
    }

    #[test]
    fn test_next_priority() {
        let c = BootstrapCompiler::new();
        let next = c.next_priority_action();
        assert!(next.is_some());
    }

    #[test]
    fn test_system_state_default() {
        let s = SystemState::new();
        assert_eq!(s.current_generation, 1);
        assert_eq!(s.container_count, 0);
    }

    #[test]
    fn test_condition_unity_progress() {
        let rule = Rule::new("unity", Condition::UnityProgressMet(80), Action::BootstrapSelf);
        let mut state = SystemState::new();
        assert!(!rule.check(&state));
        state.unity_progress = 80;
        assert!(rule.check(&state));
    }

    #[test]
    fn test_condition_container_count() {
        let rule = Rule::new("cc", Condition::ContainerCountMet(5), Action::BootstrapSelf);
        let mut state = SystemState::new();
        assert!(!rule.check(&state));
        state.container_count = 5;
        assert!(rule.check(&state));
    }

    #[test]
    fn test_condition_witness() {
        let rule = Rule::new("w", Condition::WitnessAccepted, Action::BootstrapSelf);
        let mut state = SystemState::new();
        assert!(!rule.check(&state));
        state.last_witness = Some(true);
        assert!(rule.check(&state));
    }

    #[test]
    fn test_build_step_ordering() {
        let mut c = BootstrapCompiler::new();
        let plan = c.build_plan();
        for (i, step) in plan.steps.iter().enumerate() {
            assert_eq!(step.order, i + 1);
        }
    }

    #[test]
    fn test_generate_tunnel_config() {
        let mut c = BootstrapCompiler::new();
        c.add_rule(Rule::new("tun", Condition::NeedTunnel, Action::GenerateTunnelConfig("frp".into())));
        c.state.needs_tunnel = true;
        let actions = c.evaluate();
        let has_tunnel = actions.iter().any(|a| matches!(a, Action::GenerateTunnelConfig(_)));
        assert!(has_tunnel);
    }
}
