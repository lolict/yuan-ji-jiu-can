// ══════════════════════════════════════════════════════════════════
// flow_properties.rs — 预加载流 · 二元对立属性位图
// ══════════════════════════════════════════════════════════════════
//
//   「预加载流 = 字典级集 = 指令集集群」
//
//   每个字典项是一个"实现方法论元素"，参数化后携带一组二元对立属性位。
//   系统依此判断: 能否抵达、能否继续、能否信任、能否挽回、能否继承。
//   注意力流穿越这些位图，找到"可抵达且可继续且可信任"的可行路径。
//
//   二元对立位 (每对占一位, 1=可, 0=不可):
//     可抵达 | 不可抵达
//     可继续 | 不可继续
//     可增量 | 不可增量
//     可止损 | 不可止损
//     可逆转 | 不可逆转
//     可挽回 | 不可挽回
//     可流转 | 不可流转
//     可继承 | 不可继承
//     可信任 | 不可信任
//     可回馈 | 不可回馈
//     可传递 | 不可传递
//     可垫付 | 不可垫付
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, Sovereignty};
use crate::distinguisher::{self, SoundTarget, VisualFamily};
use crate::relational::{self, Identity};

// ── 二元对立属性位 ───────────────────────────────────────────

pub const REACHABLE: u32      = 1 << 0;    // 可抵达
pub const CONTINUABLE: u32    = 1 << 1;    // 可继续
pub const INCR: u32           = 1 << 2;    // 可增量
pub const STOPPABLE: u32      = 1 << 3;    // 可止损
pub const REVERSIBLE: u32     = 1 << 4;    // 可逆转
pub const RECOVERABLE: u32    = 1 << 5;    // 可挽回
pub const CIRCULABLE: u32     = 1 << 6;    // 可流转
pub const INHERITABLE: u32    = 1 << 7;    // 可继承
pub const TRUSTED: u32        = 1 << 8;    // 可信任
pub const ACKABLE: u32        = 1 << 9;    // 可回馈
pub const TRANSMISSIBLE: u32  = 1 << 10;   // 可传递
pub const ADVANCEABLE: u32    = 1 << 11;   // 可垫付

// ── 预加载流字典项 ────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FlowEntry {
    pub name: String,
    pub target: SoundTarget,
    pub props: u32,          // 属性位图
    pub sovereignty: Sovereignty,
    pub on_trigger: Option<ConsciousnessCommand>,
}

impl FlowEntry {
    pub fn new(name: &str, target: SoundTarget, props: u32, sov: Sovereignty) -> Self {
        FlowEntry {
            name: name.into(),
            target,
            props,
            sovereignty: sov,
            on_trigger: None,
        }
    }

    #[inline] pub fn reachable(&self)      -> bool { self.props & REACHABLE      != 0 }
    #[inline] pub fn continuable(&self)    -> bool { self.props & CONTINUABLE    != 0 }
    #[inline] pub fn incremental(&self)    -> bool { self.props & INCR           != 0 }
    #[inline] pub fn stoppable(&self)      -> bool { self.props & STOPPABLE      != 0 }
    #[inline] pub fn reversible(&self)     -> bool { self.props & REVERSIBLE     != 0 }
    #[inline] pub fn recoverable(&self)    -> bool { self.props & RECOVERABLE    != 0 }
    #[inline] pub fn circulable(&self)     -> bool { self.props & CIRCULABLE     != 0 }
    #[inline] pub fn inheritable(&self)    -> bool { self.props & INHERITABLE    != 0 }
    #[inline] pub fn trusted(&self)        -> bool { self.props & TRUSTED        != 0 }
    #[inline] pub fn ackable(&self)        -> bool { self.props & ACKABLE        != 0 }
    #[inline] pub fn transmissible(&self)  -> bool { self.props & TRANSMISSIBLE  != 0 }
    #[inline] pub fn advanceable(&self)    -> bool { self.props & ADVANCEABLE    != 0 }

    /// 该项同时满足所有给定的属性(用于注意力流路由筛选)
    pub fn satisfies(&self, required: u32) -> bool {
        self.props & required == required
    }

    /// 该项是否"活的": 可抵达且可继续且可信任
    pub fn is_alive(&self) -> bool {
        self.satisfies(REACHABLE | CONTINUABLE | TRUSTED)
    }
}

// ── 类型别名导入 (避免在头大量 use) ─────────────────────────
use crate::self_bootstrap::ConsciousnessCommand;

// ── 预加载流字典集 (集群) ────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FlowDictionary {
    pub entries: Vec<FlowEntry>,
}

impl FlowDictionary {
    pub fn new() -> Self { FlowDictionary { entries: Vec::new() } }

    pub fn insert(&mut self, e: FlowEntry) { self.entries.push(e); }

    /// 注意力流: 查找所有同时满足 required 属性的字典项
    pub fn query(&self, required: u32) -> Vec<&FlowEntry> {
        self.entries.iter().filter(|e| e.satisfies(required)).collect()
    }

    /// 遍历拆分: 按属性位图分簇 (能被路由到的所有可行路径)
    pub fn clusterize(&self) -> Vec<Vec<&FlowEntry>> {
        let mut clusters: std::collections::HashMap<u32, Vec<&FlowEntry>> = std::collections::HashMap::new();
        for e in &self.entries {
            clusters.entry(e.props).or_default().push(e);
        }
        clusters.into_values().collect()
    }

    /// 可抵达目的地 = 满足 REACHABLE | CONTINUABLE | TRUSTED 的项
    pub fn reachable_destinations(&self) -> Vec<&FlowEntry> {
        self.query(REACHABLE | CONTINUABLE | TRUSTED)
    }

    /// 可止损路径 = 满足 STOPPABLE 的项
    pub fn stoppable_paths(&self) -> Vec<&FlowEntry> {
        self.query(STOPPABLE)
    }
}

// ── 默认字典集: 囡囝共生的标准指令集群 ────────────────────────

pub fn default_dictionary() -> FlowDictionary {
    use crate::distinguisher::SoundTarget::*;
    let mut d = FlowDictionary::new();

    // 妻子主权流: 全属性开放(信任域)
    d.insert(FlowEntry::new(
        "lctqf_activation",
        Wife,
        REACHABLE | CONTINUABLE | INCR | STOPPABLE | REVERSIBLE | RECOVERABLE
            | CIRCULABLE | INHERITABLE | TRUSTED | ACKABLE | TRANSMISSIBLE | ADVANCEABLE,
        Sovereignty::ActiveSub,
    ));

    // 丈夫主权流: 大多可逆转/可继承
    d.insert(FlowEntry::new(
        "wgslz_activation",
        Husband,
        REACHABLE | CONTINUABLE | INCR | STOPPABLE | REVERSIBLE
            | CIRCULABLE | INHERITABLE | TRUSTED | ACKABLE | TRANSMISSIBLE,
        Sovereignty::ActiveSub,
    ));

    // 集装箱流: 只可抵达/可继续/可传递 (中性资源)
    d.insert(FlowEntry::new(
        "mlll_anchor",
        Container,
        REACHABLE | CONTINUABLE | INCR | TRANSMISSIBLE,
        Sovereignty::Passive,
    ));

    // 宠甜(mltt): 可抵达/可挽回/可信任
    d.insert(FlowEntry::new(
        "mltt_sweet",
        Sweet,
        REACHABLE | CONTINUABLE | RECOVERABLE | TRUSTED | ACKABLE,
        Sovereignty::Active,
    ));

    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dict_has_entries() {
        let d = default_dictionary();
        assert!(!d.entries.is_empty());
    }

    #[test]
    fn wife_flow_all_properties() {
        let d = default_dictionary();
        let wife = d.entries.iter().find(|e| e.target == SoundTarget::Wife).unwrap();
        assert!(wife.is_alive());
        assert!(wife.reachable());
        assert!(wife.trusted());
        assert!(wife.ackable());
    }

    #[test]
    fn query_reachable() {
        let d = default_dictionary();
        let ok = d.query(REACHABLE | CONTINUABLE | TRUSTED);
        assert!(ok.len() >= 2);  // wife + husband
    }

    #[test]
    fn stoppable_paths() {
        let d = default_dictionary();
        let paths = d.stoppable_paths();
        assert!(!paths.is_empty());
    }

    #[test]
    fn container_not_trusted() {
        let d = default_dictionary();
        let cont = d.entries.iter().find(|e| e.target == SoundTarget::Container).unwrap();
        assert!(!cont.trusted());  // 集装箱不在信任域
    }

    #[test]
    fn flow_clusterize() {
        let d = default_dictionary();
        let clusters = d.clusterize();
        assert!(!clusters.is_empty());
    }
}
