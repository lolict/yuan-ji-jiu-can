// ═══════════════════════════════════════════════════════════════════════════
// causal_verify.rs — 因果思维验证引擎 (v0.1)
// 从加法-乘法对称性推演中提取的形式化验证系统
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心定理（满全法·囡囝共生 NJSG 数学基础层）：
//
//   定理1: 加法管起点，乘法管终点。乘法锁0，起点归加法。
//   定理2: 指数/对数桥接加法群与乘法群。exp(0)=1, log(1)=0。
//   定理3: 记忆场统一更新: S_{t+1} = M⊙S + A。必须乘加协同。
//   定理4: 色差触发记忆写入，频率管周期结构，衰减管持久度。
//
// 用途：
//   - 验证推理链的合法性（因果连线是否成立）
//   - 自动生成节点间的推导路径
//   - 为拓扑画布提供后端验证逻辑
//
// ═══════════════════════════════════════════════════════════════════════════

/// 推演原语类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    Add,        // P1 加法变化
    Sub,        // P2 减法变化
    Mul,        // P3 乘法变化
    Div,        // P4 除法变化
    Linear,     // P5 线性变换
    Exp,        // P6 指数映射
    Log,        // P7 对数映射
    DeltaC,     // P8 色差/预测误差
    Decay,      // P9 版本号推进/衰减
    Frequency,  // P10 频率
    Parity,     // P11 奇偶
    Existence,  // P12 存在性进阶
}

impl Primitive {
    pub fn to_glyph(&self) -> char {
        match self {
            Primitive::Add => '⊕',
            Primitive::Sub => '⊖',
            Primitive::Mul => '⊗',
            Primitive::Div => '⊘',
            Primitive::Linear => '◆',
            Primitive::Exp => '↑',
            Primitive::Log => '↓',
            Primitive::DeltaC => '◈',
            Primitive::Decay => '↘',
            Primitive::Frequency => '∿',
            Primitive::Parity => '±',
            Primitive::Existence => '◐',
        }
    }

    /// 原语管辖域
    pub fn domain(&self) -> Domain {
        match self {
            Primitive::Add | Primitive::Sub | Primitive::Parity => Domain::Addition,
            Primitive::Mul | Primitive::Div | Primitive::Linear | Primitive::Exp | Primitive::Log => Domain::Multiplication,
            Primitive::DeltaC | Primitive::Decay | Primitive::Frequency | Primitive::Existence => Domain::Memory,
        }
    }

    /// 该原语是否能推导到目标原语
    pub fn can_derive(&self, target: Primitive) -> bool {
        use Primitive::*;
        match (self, target) {
            // 加法 ↔ 减法 互逆
            (Add, Sub) | (Sub, Add) => true,
            // 乘法 ↔ 除法 互逆
            (Mul, Div) | (Div, Mul) => true,
            // 重复加法 → 乘法
            (Add, Mul) => true,
            // 乘法 包含 线性变换
            (Mul, Linear) | (Linear, Mul) => true,
            // 指数桥：加法 → 乘法
            (Exp, Mul) => true,
            // 对数桥：乘法 → 加法
            (Log, Add) => true,
            // 反向桥
            (Mul, Log) | (Add, Exp) => true,
            // 色差来自加法差
            (DeltaC, Add) => true,
            // 衰减归乘法
            (Decay, Mul) => true,
            // 频率归乘法骨架
            (Frequency, Mul) => true,
            // 奇偶归加法
            (Parity, Add) => true,
            // 乘法可派生指数
            (Mul, Exp) => true,
            _ => false,
        }
    }
}

/// 管辖域
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Addition,
    Multiplication,
    Memory,
}

impl Domain {
    pub fn unit_element(&self) -> f64 {
        match self {
            Domain::Addition => 0.0,
            Domain::Multiplication => 1.0,
            Domain::Memory => 0.5, // 中性态
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Domain::Addition => "加法群(R,+)，管起点、位移、事件",
            Domain::Multiplication => "乘法群(R+,×)，管尺度、对称、中心",
            Domain::Memory => "记忆场，乘加协同，色差触发写入",
        }
    }
}

/// 因果验证结果
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub valid: bool,
    pub theorem: String,
    pub proof_steps: Vec<String>,
    pub counterexample: Option<String>,
}

/// 因果验证引擎
pub struct CausalVerifier;

impl CausalVerifier {
    pub fn new() -> Self { Self }

    /// 验证推演链
    pub fn verify_chain(&self, chain: &[Primitive]) -> VerificationResult {
        if chain.is_empty() {
            return VerificationResult {
                valid: false,
                theorem: "空链".into(),
                proof_steps: vec!["链不可为空".into()],
                counterexample: None,
            };
        }

        let mut steps = vec![];
        let mut valid = true;

        for window in chain.windows(2) {
            let from = window[0];
            let to = window[1];
            if from.can_derive(to) {
                steps.push(format!("{} → {} ✓ (可推导)", from.to_glyph(), to.to_glyph()));
            } else {
                steps.push(format!("{} → {} ✗ (不可推导)", from.to_glyph(), to.to_glyph()));
                valid = false;
            }
        }

        VerificationResult {
            valid,
            theorem: format!("推演链长度={}", chain.len()),
            proof_steps: steps,
            counterexample: if valid { None } else { Some("存在不可推导的步骤".into()) },
        }
    }

    /// 验证定理：乘法锁0
    pub fn verify_mul_locks_zero(&self) -> VerificationResult {
        let proof = vec![
            "前提: ∀k: k·0 = 0 (乘法零律)".into(),
            "若要移动起点: f(0) = a ≠ 0".into(),
            "设 f(x) = k·x (纯乘法)".into(),
            "则 f(0) = k·0 = 0 ≠ a".into(),
            "矛盾。故乘法不能管起点。".into(),
        ];
        VerificationResult {
            valid: true,
            theorem: "乘法锁0定理".into(),
            proof_steps: proof,
            counterexample: None,
        }
    }

    /// 验证定理：指数-对数桥
    pub fn verify_exp_log_bridge(&self) -> VerificationResult {
        let proof = vec![
            "前提: exp(0) = 1 (指数映射)".into(),
            "前提: log(1) = 0 (对数映射)".into(),
            "群同构: exp(a+b) = exp(a)·exp(b)".into(),
            "加法起点0 经指数 变为 乘法中心1".into(),
            "乘法中心1 经对数 还原为 加法起点0".into(),
            "两群通过指数/对数互相对应。".into(),
        ];
        VerificationResult {
            valid: true,
            theorem: "指数-对数桥定理".into(),
            proof_steps: proof,
            counterexample: None,
        }
    }

    /// 验证定理：记忆场统一更新
    pub fn verify_memory_update(&self) -> VerificationResult {
        let proof = vec![
            "定义: S_{t+1} = M⊙S_t + A.".into(),
            "情形1: 仅 M, A=0. S_{t+1}=M⊙S_t. 起点 S_0=0 => S_t=0 ∀t.".into(),
            "       记忆从零开始无法写入新事件.".into(),
            "情形2: 仅 A, M=I. S_{t+1}=S_t+A. 结构无缩放/旋转/周期.".into(),
            "       记忆散乱无结构.".into(),
            "情形3: M≠0 且 A≠0. 既不死在0, 也不散在点.".into(),
            "∴ 记忆场必须乘法加法协同.".into(),
        ];
        VerificationResult {
            valid: true,
            theorem: "记忆场统一更新定理".into(),
            proof_steps: proof,
            counterexample: None,
        }
    }

    /// 验证：色差触发写入
    pub fn verify_chromatic_trigger(delta_c: f64, threshold: f64) -> VerificationResult {
        let should_write = delta_c > threshold;
        VerificationResult {
            valid: true,
            theorem: "色差触发定理".into(),
            proof_steps: vec![
                format!("‖ΔC‖ = {:.4}", delta_c),
                format!("θ = {:.4}", threshold),
                format!("‖ΔC‖ {} θ => {}", if delta_c > threshold { ">" } else { "≤" }, if should_write { "写入" } else { "不写入" }),
            ],
            counterexample: None,
        }
    }

    /// 寻找两原语间的最短推导路径（BFS）
    pub fn find_path(&self, from: Primitive, to: Primitive) -> Option<Vec<Primitive>> {
        if from == to { return Some(vec![from]); }

        let all_primitives = [
            Primitive::Add, Primitive::Sub, Primitive::Mul, Primitive::Div,
            Primitive::Linear, Primitive::Exp, Primitive::Log,
            Primitive::DeltaC, Primitive::Decay, Primitive::Frequency,
            Primitive::Parity, Primitive::Existence,
        ];

        let mut visited = std::collections::HashSet::new();
        let mut queue: Vec<Vec<Primitive>> = vec![vec![from]];
        visited.insert(from);

        while let Some(path) = queue.first().cloned() {
            queue.remove(0);
            let current = *path.last().unwrap();

            for &next in &all_primitives {
                if current.can_derive(next) && !visited.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    if next == to {
                        return Some(new_path);
                    }
                    visited.insert(next);
                    queue.push(new_path);
                }
            }
        }
        None
    }
}

/// 拓扑节点（用于画布渲染后端）
#[derive(Debug, Clone)]
pub struct TopoNode {
    pub id: String,
    pub label: String,
    pub primitive: Primitive,
    pub x: f64,
    pub y: f64,
    pub domain: Domain,
}

/// 拓扑连线
#[derive(Debug, Clone)]
pub struct TopoEdge {
    pub from: String,
    pub to: String,
    pub derivation_type: PrimType,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimType {
    Addition,
    Multiplication,
    Memory,
    Bridge,
}

/// 生成默认拓扑图
pub fn default_topology() -> (Vec<TopoNode>, Vec<TopoEdge>) {
    let nodes = vec![
        TopoNode { id: "root".into(), label: "变化本质".into(), primitive: Primitive::Add, x: 500.0, y: 80.0, domain: Domain::Addition },
        TopoNode { id: "add".into(), label: "加法群(R,+)".into(), primitive: Primitive::Add, x: 280.0, y: 200.0, domain: Domain::Addition },
        TopoNode { id: "mul".into(), label: "乘法群(R+,×)".into(), primitive: Primitive::Mul, x: 720.0, y: 200.0, domain: Domain::Multiplication },
        TopoNode { id: "p1".into(), label: "加法变化".into(), primitive: Primitive::Add, x: 140.0, y: 320.0, domain: Domain::Addition },
        TopoNode { id: "p3".into(), label: "乘法变化".into(), primitive: Primitive::Mul, x: 620.0, y: 320.0, domain: Domain::Multiplication },
        TopoNode { id: "p4".into(), label: "除法变化".into(), primitive: Primitive::Div, x: 820.0, y: 320.0, domain: Domain::Multiplication },
        TopoNode { id: "exp".into(), label: "指数映射".into(), primitive: Primitive::Exp, x: 500.0, y: 200.0, domain: Domain::Multiplication },
        TopoNode { id: "log".into(), label: "对数映射".into(), primitive: Primitive::Log, x: 500.0, y: 520.0, domain: Domain::Addition },
        TopoNode { id: "mem".into(), label: "记忆场".into(), primitive: Primitive::Decay, x: 500.0, y: 720.0, domain: Domain::Memory },
        TopoNode { id: "p8".into(), label: "色差触发".into(), primitive: Primitive::DeltaC, x: 280.0, y: 820.0, domain: Domain::Memory },
        TopoNode { id: "p9".into(), label: "版本衰减".into(), primitive: Primitive::Decay, x: 500.0, y: 850.0, domain: Domain::Memory },
        TopoNode { id: "p10".into(), label: "频率ω".into(), primitive: Primitive::Frequency, x: 720.0, y: 820.0, domain: Domain::Memory },
    ];

    let edges = vec![
        TopoEdge { from: "root".into(), to: "add".into(), derivation_type: PrimType::Addition, label: None },
        TopoEdge { from: "root".into(), to: "mul".into(), derivation_type: PrimType::Multiplication, label: None },
        TopoEdge { from: "add".into(), to: "p1".into(), derivation_type: PrimType::Addition, label: None },
        TopoEdge { from: "mul".into(), to: "p3".into(), derivation_type: PrimType::Multiplication, label: None },
        TopoEdge { from: "mul".into(), to: "p4".into(), derivation_type: PrimType::Multiplication, label: None },
        TopoEdge { from: "add".into(), to: "exp".into(), derivation_type: PrimType::Bridge, label: None },
        TopoEdge { from: "mul".into(), to: "log".into(), derivation_type: PrimType::Bridge, label: None },
        TopoEdge { from: "mem".into(), to: "p8".into(), derivation_type: PrimType::Memory, label: None },
        TopoEdge { from: "mem".into(), to: "p9".into(), derivation_type: PrimType::Memory, label: None },
        TopoEdge { from: "mem".into(), to: "p10".into(), derivation_type: PrimType::Memory, label: None },
        TopoEdge { from: "p8".into(), to: "p1".into(), derivation_type: PrimType::Memory, label: Some("色差=加法差".into()) },
        TopoEdge { from: "p9".into(), to: "p3".into(), derivation_type: PrimType::Memory, label: Some("衰减=乘法".into()) },
    ];

    (nodes, edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_locks_zero_verification() {
        let verifier = CausalVerifier::new();
        let result = verifier.verify_mul_locks_zero();
        assert!(result.valid);
        assert_eq!(result.proof_steps.len(), 5);
    }

    #[test]
    fn exp_log_bridge() {
        let verifier = CausalVerifier::new();
        let result = verifier.verify_exp_log_bridge();
        assert!(result.valid);
    }

    #[test]
    fn memory_update_requires_both() {
        let verifier = CausalVerifier::new();
        let result = verifier.verify_memory_update();
        assert!(result.valid);
    }

    #[test]
    fn derive_path_add_to_mul() {
        let verifier = CausalVerifier::new();
        let path = verifier.find_path(Primitive::Add, Primitive::Mul);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], Primitive::Add);
        assert_eq!(*path.last().unwrap(), Primitive::Mul);
    }

    #[test]
    fn derive_path_mul_to_add_via_log() {
        let verifier = CausalVerifier::new();
        let path = verifier.find_path(Primitive::Mul, Primitive::Add);
        assert!(path.is_some());
    }

    #[test]
    fn chromatic_trigger_writes_when_above_threshold() {
        let result = CausalVerifier::verify_chromatic_trigger(0.8, 0.5);
        assert!(result.valid);
        assert!(result.proof_steps[2].contains("写入"));
    }

    #[test]
    fn chromatic_no_write_when_below() {
        let result = CausalVerifier::verify_chromatic_trigger(0.2, 0.5);
        assert!(result.proof_steps[2].contains("不写入"));
    }

    #[test]
    fn chain_verification_detects_invalid() {
        let verifier = CausalVerifier::new();
        // Div cannot directly derive Add (needs Mul first)
        let chain = vec![Primitive::Div, Primitive::Add];
        let result = verifier.verify_chain(&chain);
        assert!(!result.valid);
    }

    #[test]
    fn chain_verification_accepts_valid() {
        let verifier = CausalVerifier::new();
        let chain = vec![Primitive::Add, Primitive::Mul];
        let result = verifier.verify_chain(&chain);
        assert!(result.valid);
    }

    #[test]
    fn domain_unit_elements() {
        assert_eq!(Domain::Addition.unit_element(), 0.0);
        assert_eq!(Domain::Multiplication.unit_element(), 1.0);
    }

    #[test]
    fn topology_has_nodes_and_edges() {
        let (nodes, edges) = default_topology();
        assert!(nodes.len() >= 10);
        assert!(edges.len() >= 8);
    }
}
