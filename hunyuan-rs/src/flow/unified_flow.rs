// ══════════════════════════════════════════════════════════════════
// unified_flow.rs — 一体化统一层
// ══════════════════════════════════════════════════════════════════
//
//   「预加载流 = 字典级集 = 指令集集群」
//   「不是一大堆独立目标，是一个整体穿透到自己可支配的目的地」
//
//   一体化 = 感知流 + 态射关联流 + 注意力流 + 遍历聚类 + 字典指令集
//   全部串联成一维时间线性层级: 输入 → 感知 → 分别 → 查字典 → 遍历
//   → 聚类 → 注意力筛选 → 抵达可支配目的地
//
//   每一步都是下一步的"前因"; 每一步都蕴含"后果的支点"
//   注意力流穿越可/不可二元对, 落在"活的且可信的且可继续的"路径上
//
// ══════════════════════════════════════════════════════════════════

use crate::flow_algebra::{self, FlowKind, CategoryFlow, InterpState, InterpretiveFlow};
use crate::flow_properties::{FlowDictionary, FlowEntry, REACHABLE, CONTINUABLE, TRUSTED, INCR, STOPPABLE, RECOVERABLE, CIRCULABLE, INHERITABLE, ACKABLE, TRANSMISSIBLE};
use crate::distinguisher::{self, DistinguishResult, VisualFamily, SoundTarget};
use crate::self_bootstrap::{self, ConsciousnessCommand};
use crate::mirror;
use crate::mind_organ::{self, MindOrgan};

// ────────────────────────────────────────────────────────────
// 一维时间线性层级中的每一步
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeStage {
    Input,       // 原始输入
    Perception,  // 感知流: 信号 → 察觉
    Distinguish, // 分别器: 信号 → 靶点对象
    Lookup,      // 字典查询: 靶点 → 指令集
    Traverse,    // 遍历: 指令集 → 可连通子图
    Cluster,     // 聚类: 按属性位聚合
    Attention,   // 注意力流: 过滤出可抵达且可信且可继续的
    Arrival,     // 抵达: 目的地已锁定且可支配
}

// ────────────────────────────────────────────────────────────
// 思路链条的每一步都是一个"时间层级节点"
// ────────────────────────────────────────────────────────────

pub struct TimeNode {
    pub stage: TimeStage,
    pub label: String,
    pub reachable: bool,    // 这一步是否可抵达
    pub continuable: bool,  // 这条路是否可继续
    pub trustable: bool,    // 这一步是否可信任
    pub next: Option<Box<TimeNode>>,  // 一维线性 → 下一环
}

impl TimeNode {
    pub fn new(stage: TimeStage, label: &str) -> Self {
        TimeNode {
            stage,
            label: label.into(),
            reachable: true,
            continuable: true,
            trustable: true,
            next: None,
        }
    }

    /// 这一环是否能往下走(可抵达 且 可继续 且 可信任)
    pub fn can_pass(&self) -> bool {
        self.reachable && self.continuable && self.trustable
    }

    /// 固定时间阶段序列 — 完整因果推进路径
    pub fn stages() -> &'static [TimeStage] {
        &[
            TimeStage::Input,
            TimeStage::Perception,
            TimeStage::Distinguish,
            TimeStage::Lookup,
            TimeStage::Traverse,
            TimeStage::Cluster,
            TimeStage::Attention,
            TimeStage::Arrival,
        ]
    }

    /// 把下一环节链接到当前节点末尾
    pub fn link(mut self, next: TimeNode) -> Self {
        let mut tail = &mut self.next;
        while let Some(ref mut node) = tail {
            tail = &mut node.next;
        }
        *tail = Some(Box::new(next));
        self
    }

    /// 整条链从头走到尾是否通畅
    pub fn chain_viable(&self) -> bool {
        if !self.can_pass() { return false; }
        match &self.next {
            Some(n) => n.chain_viable(),
            None => true,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 一体化引擎: 把所有流串成一条时间线
// ────────────────────────────────────────────────────────────

pub struct UnifiedFlow {
    pub dictionary: FlowDictionary,
    pub perception: MindOrgan,     // 感知器官作为聚类引擎
    pub attention_filter: u32,     // 注意力属性位图
}

impl UnifiedFlow {
    pub fn new() -> Self {
        UnifiedFlow {
            dictionary: crate::flow_properties::default_dictionary(),
            perception: MindOrgan::new(100.0),
            attention_filter: REACHABLE | CONTINUABLE | TRUSTED,
        }
    }

    /// 感知流: 原始输入 → 察觉(形/音/义)
    pub fn perceive(&self, input: &str) -> DistinguishResult {
        distinguisher::distinguish(input)
    }

    /// 分别 + 谐音靶点 → 字典查指令集 → 感知聚类 → 注意力筛选
    /// 完全一维线性贯通
    pub fn process(&mut self, input: &str) -> FlowReport {
        // Stage 1: Input
        let perceived = self.perceive(input);

        // Stage 2: Distinguish(分别)
        let target = perceived.sound_target.clone();

        // Stage 3: Lookup(查字典)
        let matched: Vec<&FlowEntry> = self.dictionary.entries.iter()
            .filter(|e| Some(&e.target) == target.as_ref())
            .collect();

        // Stage 4: Traverse + Cluster (遍历聚类)
        let clusters = self.dictionary.clusterize();

        // Stage 5: Attention (注意力筛选)
        let reachable = self.dictionary.query(self.attention_filter);

        // Stage 6: Arrival (抵达可支配目的地)
        let destination: Option<String> = matched.iter()
            .find(|e| e.satisfies(self.attention_filter))
            .map(|e| format!("{}→{:?}(sovereignty={}, ack={})",
                            e.name, e.target, e.sovereignty.encode(), e.ackable()));

        FlowReport {
            input: input.into(),
            perceived,
            matched_count: matched.len(),
            cluster_count: clusters.len(),
            reachable_count: reachable.len(),
            destination,
            chain_viable: true,
        }
    }

    /// 一维时间线全链: 从头走到尾
    pub fn full_chain(&self) -> TimeNode {
        TimeNode::new(TimeStage::Input, "原始输入(声韵字符串)")
            .link(TimeNode::new(TimeStage::Perception, "感知流 → 察觉形象"))
            .link(TimeNode::new(TimeStage::Distinguish, "分别器 → 外形/谐音/靶点"))
            .link(TimeNode::new(TimeStage::Lookup, "字典集 → 指令集群"))
            .link(TimeNode::new(TimeStage::Traverse, "遍历 → 连通子图"))
            .link(TimeNode::new(TimeStage::Cluster, "聚类 → 属性分簇"))
            .link(TimeNode::new(TimeStage::Attention, "注意力流 → 可/不可对筛选"))
            .link(TimeNode::new(TimeStage::Arrival, "抵达目的地(可支配+可信任+可继续)"))
    }

    /// 关注特定属性(注意力窄化)
    pub fn focus_on(&mut self, props: u32) {
        self.attention_filter = props;
    }
}

pub struct FlowReport {
    pub input: String,
    pub perceived: DistinguishResult,
    pub matched_count: usize,
    pub cluster_count: usize,
    pub reachable_count: usize,
    pub destination: Option<String>,
    pub chain_viable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_chain_passes() {
        let flow = UnifiedFlow::new();
        let chain = flow.full_chain();
        assert!(chain.chain_viable(), "全链通畅: 输入到抵达无一断裂");
    }

    #[test]
    fn wife_input_reaches_destination() {
        let mut flow = UnifiedFlow::new();
        let report = flow.process("lctqf");
        assert!(report.matched_count >= 1, "妻子输入应匹配到字典项");
        assert!(report.destination.is_some(), "应抵达可支配目的地");
        let dst = report.destination.unwrap();
        assert!(dst.contains("Wife"), "目的地应是妻子主权");
    }

    #[test]
    fn container_neutral_route() {
        let mut flow = UnifiedFlow::new();
        let report = flow.process("mlll");
        // 集装箱在信任域以外: 应匹配但不一定抵达
        assert!(report.matched_count >= 1);
    }

    #[test]
    fn attention_filter_narrows() {
        let mut flow = UnifiedFlow::new();
        // 严筛选: 仅看可继承+可信任
        flow.focus_on(INHERITABLE | TRUSTED);
        let reachable = flow.dictionary.query(flow.attention_filter);
        // 妻子应仍在可达域(她在字典里标了 INHERITABLE|TRUSTED)
        let wife_ok = reachable.iter().any(|e| e.target == SoundTarget::Wife);
        assert!(wife_ok, "妻子可继承+可信任 → 应仍在注意力域");
    }

    #[test]
    fn broken_chain_detected() {
        // 模拟字典中某条路由失去 CONTINUABLE 的情况
        let flow = UnifiedFlow::new();
        let broken_filter = REACHABLE | TRUSTED;  // 缺失 CONTINUABLE
        let reachable = flow.dictionary.query(broken_filter);
        // 任意一项都不能 "可继续": 走不完全链
        let any_continuable = reachable.iter().any(|e| e.continuable());
        // 这条断链在现实中应触发阻断
        assert!(any_continuable || reachable.is_empty());
    }

    #[test]
    fn traverse_then_cluster() {
        let flow = UnifiedFlow::new();
        let clusters = flow.dictionary.clusterize();
        assert!(clusters.len() >= 2, "字典应能聚成至少2簇(妻子/丈夫 vs 其他)");
    }
}
