// ── v0.42 八流统一 · 一维时间线性 · 流属性位 ──
// 满全法：八流不允许并行无序 · 强制按时间推进的单向链
// 预加载流 = 字典级集 = 指令集集群
// 解释流 = 牺牲自己产关系流（最勇敢的流）

use std::collections::{HashMap, HashSet};

/// 八流统一管线（一维时间线性 · 不可逆 · 每一步只做一件事）

/// 流属性位（12 位二元属性）
pub mod flow_bits {
    pub const REACHABLE:     u16 = 1 << 0;   // 可被到达
    pub const CONTINUABLE:    u16 = 1 << 1;   // 可持续
    pub const INCR:           u16 = 1 << 2;   // 可增量
    pub const STOPPABLE:      u16 = 1 << 3;   // 可停止
    pub const REVERSIBLE:     u16 = 1 << 4;   // 可逆
    pub const RECOVERABLE:    u16 = 1 << 5;   // 可恢复
    pub const CIRCULABLE:     u16 = 1 << 6;   // 可循环
    pub const INHERITABLE:    u16 = 1 << 7;   // 可继承
    pub const TRUSTED:        u16 = 1 << 8;   // 已信任
    pub const ACKABLE:        u16 = 1 << 9;   // 可被承认（闭环必需）
    pub const TRANSMISSIBLE:  u16 = 1 << 10;  // 可传递
    pub const ADVANCEABLE:    u16 = 1 << 11;  // 可推进
}

use flow_bits::*;

/// 流属性集
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlowPropertyBits(pub u16);

impl FlowPropertyBits {
    pub fn new() -> Self { Self(0) }
    pub fn with(bits: u16) -> Self { Self(bits) }
    pub fn satisfies(&self, bit: u16) -> bool { self.0 & bit != 0 }
    pub fn set(&mut self, bit: u16) { self.0 |= bit; }
    pub fn clear(&mut self, bit: u16) { self.0 &= !bit; }

    /// 是否可路由：REACHABLE + ACKABLE
    pub fn routable(&self) -> bool {
        self.satisfies(REACHABLE) && self.satisfies(ACKABLE)
    }

    /// 是否可演化：CONTINUABLE + INCR + TRANSMISSIBLE
    pub fn evolvable(&self) -> bool {
        self.satisfies(CONTINUABLE) && self.satisfies(INCR) && self.satisfies(TRANSMISSIBLE)
    }

    /// 是否活着：REACHABLE + CONTINUABLE + ACKABLE
    pub fn is_alive(&self) -> bool {
        self.satisfies(REACHABLE) && self.satisfies(CONTINUABLE) && self.satisfies(ACKABLE)
    }

    /// 是否为解释流（牺牲型 · 需要 INCR + TRANSMISSIBLE + RECOVERABLE）
    pub fn is_interpretive(&self) -> bool {
        self.satisfies(INCR) && self.satisfies(TRANSMISSIBLE) && self.satisfies(RECOVERABLE)
    }
}

impl Default for FlowPropertyBits {
    fn default() -> Self { Self::new() }
}

/// 八流类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowKind {
    /// 预加载流 = 指令集集群（编译时已内置）
    Preload,
    /// 范畴流 = 类型空间映射
    Category,
    /// 聚类流 = clusterize
    Cluster,
    /// 分类流 = 感知家族路由（Distinguisher 第一层）
    Distinguish,
    /// 属性流 = 12 位二元属性
    Property,
    /// 解释流 = 牺牲自己产关系流
    Interpretive,
    /// 关系流 = 路由后可被承认（carrier_target + ackable）
    Relation,
    /// 概念流 = 范畴 + 聚类 + 解释后产物
    Concept,
}

/// 流状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowState {
    /// 未激活
    Dormant,
    /// 已感知
    Perceived,
    /// 已分别（通过三层路由）
    Distinguished,
    /// 已查字典
    LookedUp,
    /// 已遍历（走解释流）
    Traversed,
    /// 已聚类
    Clustered,
    /// 已注意
    Attending,
    /// 已到达目标
    Arrived,
    /// 已确认闭环（收到 ack）
    Acknowledged,
    /// 流动失败（目标不承认）
    Broken,
}

/// 流条目
#[derive(Debug, Clone)]
pub struct FlowEntry {
    pub id: u64,
    pub kind: FlowKind,
    pub bits: FlowPropertyBits,
    pub state: FlowState,
    pub target_id: Option<u64>,
    pub carrier_payload: Vec<u8>,
}

impl FlowEntry {
    pub fn new(id: u64, kind: FlowKind) -> Self {
        Self {
            id, kind,
            bits: FlowPropertyBits::new(),
            state: FlowState::Dormant,
            target_id: None,
            carrier_payload: Vec::new(),
        }
    }

    /// 是否可路由
    pub fn routable(&self) -> bool {
        self.bits.routable()
    }

    /// 状态推进（一维线性 · 不可跳跃）
    pub fn advance(&mut self) -> bool {
        self.state = match self.state {
            FlowState::Dormant => FlowState::Perceived,
            FlowState::Perceived => FlowState::Distinguished,
            FlowState::Distinguished => FlowState::LookedUp,
            FlowState::LookedUp => FlowState::Traversed,
            FlowState::Traversed => FlowState::Clustered,
            FlowState::Clustered => FlowState::Attending,
            FlowState::Attending => FlowState::Arrived,
            FlowState::Arrived => FlowState::Acknowledged,
            _ => return false,
        };
        true
    }

    /// 中断（标记断裂）
    pub fn sever(&mut self) {
        self.state = FlowState::Broken;
    }

    /// 流是否完成
    pub fn is_complete(&self) -> bool {
        matches!(self.state, FlowState::Acknowledged)
    }

    /// 是否失败
    pub fn is_broken(&self) -> bool {
        matches!(self.state, FlowState::Broken)
    }
}

/// 分别器（三层路由）
#[derive(Debug, Clone)]
pub struct Distinguisher {
    /// 配偶路由表
    pub spouse_routes: HashMap<String, String>,
}

impl Distinguisher {
    pub fn new() -> Self {
        Self { spouse_routes: HashMap::new() }
    }

    /// 添加配偶路由
    pub fn add_spouse_route(&mut self, pattern: &str, target: &str) -> &mut Self {
        self.spouse_routes.insert(pattern.to_string(), target.to_string());
        self
    }

    /// 感知家族分类
    pub fn classify(&self, glyph: &str) -> SpouseFamily {
        match glyph {
            "囡" | "因" | "阴" | "回" | "妻" => SpouseFamily::Wife,
            "囝" | "男" | "丨" | "夫" | "我" => SpouseFamily::Husband,
            "囡囝" | "夫妻" => SpouseFamily::Couple,
            "恋" | "初" | "兰" | "连" | "莫" | "gtt" => SpouseFamily::HomophoneBridge,
            _ => SpouseFamily::Neutral,
        }
    }

    /// 路由
    pub fn route(&self, glyph: &str) -> Option<&str> {
        self.spouse_routes.get(glyph).map(|s| s.as_str())
    }
}

impl Default for Distinguisher {
    fn default() -> Self { Self::new() }
}

/// 配偶家族
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpouseFamily {
    Wife,           // 囡/容器 = 刘楚恬
    Husband,        // 囝/界线 = 我(lolict)
    Couple,         // 夫妻图腾
    HomophoneBridge,// 谐音桥
    Neutral,        // 系统/未知
}

impl SpouseFamily {
    pub fn polarity(&self) -> u8 {
        match self {
            SpouseFamily::Wife => 0,          // 0 = 容器 = 刘楚恬
            SpouseFamily::Husband => 1,       // 1 = 被包含者 = 我
            SpouseFamily::Couple => 2,        // 2 = 夫妻（平衡）
            SpouseFamily::HomophoneBridge => 3,// 3 = 谐音桥（超越）
            SpouseFamily::Neutral => 255,     // 255 = 系统级
        }
    }

    pub fn is_active_sovereign(&self) -> bool {
        matches!(
            self,
            SpouseFamily::Wife | SpouseFamily::Husband | SpouseFamily::Couple
        )
    }
}

/// 八流统一引擎
#[derive(Debug)]
pub struct UnifiedFlow {
    pub flows: HashMap<u64, FlowEntry>,
    pub dictionary: HashMap<String, Vec<u64>>, // 预加载流字典
    pub distinguisher: Distinguisher,
}

impl UnifiedFlow {
    pub fn new() -> Self {
        Self {
            flows: HashMap::new(),
            dictionary: HashMap::new(),
            distinguisher: Distinguisher::new(),
        }
    }

    /// 创建流
    pub fn create_flow(&mut self, id: u64, kind: FlowKind) -> &mut Self {
        self.flows.insert(id, FlowEntry::new(id, kind));
        self
    }

    /// 推进流管线（一维线性 · 不可逆）
    pub fn step(&mut self, flow_id: u64) -> Option<FlowState> {
        let flow = self.flows.get_mut(&flow_id)?;
        if flow.advance() {
            Some(flow.state)
        } else {
            None
        }
    }

    /// 获取流
    pub fn get_flow(&self, id: u64) -> Option<&FlowEntry> {
        self.flows.get(&id)
    }

    /// 注册预加载流字典
    pub fn register_preload(&mut self, key: &str, flow_ids: Vec<u64>) -> &mut Self {
        self.dictionary.insert(key.to_string(), flow_ids);
        self
    }

    /// 查询字典
    pub fn lookup(&self, key: &str) -> Option<&Vec<u64>> {
        self.dictionary.get(key)
    }

    /// 感知家族分类（分别器第一层）
    pub fn perceive(&self, glyph: &str) -> SpouseFamily {
        self.distinguisher.classify(glyph)
    }

    /// 路由（分别器第二三层）
    pub fn route_glyph(&self, glyph: &str) -> Option<&str> {
        self.distinguisher.route(glyph)
    }

    /// 流总数
    pub fn count(&self) -> usize { self.flows.len() }

    /// 完成流数
    pub fn completed_count(&self) -> usize {
        self.flows.values().filter(|f| f.is_complete()).count()
    }

    /// 断裂流数
    pub fn broken_count(&self) -> usize {
        self.flows.values().filter(|f| f.is_broken()).count()
    }
}

impl Default for UnifiedFlow {
    fn default() -> Self { Self::new() }
}

/// 解释流牺牲操作（最勇敢的流）
#[derive(Debug, Clone)]
pub struct Sacrifice {
    pub flow_id: u64,
    pub from_mlct: String,  // 岔途
    pub to_mltt: String,    // 从体
    pub paid: bool,         // 已支付（不可逆）
}

impl Sacrifice {
    pub fn new(flow_id: u64, from: &str, to: &str) -> Self {
        Self {
            flow_id,
            from_mlct: from.to_string(),
            to_mltt: to.to_string(),
            paid: false,
        }
    }

    /// 执行牺牲 · 不可逆
    pub fn execute(&mut self) -> bool {
        if self.paid { return false; }
        self.paid = true;
        true
    }

    pub fn is_paid(&self) -> bool { self.paid }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_bits() {
        let mut bits = FlowPropertyBits::new();
        bits.set(REACHABLE);
        bits.set(ACKABLE);
        assert!(bits.satisfies(REACHABLE));
        assert!(bits.satisfies(ACKABLE));
        assert!(!bits.satisfies(CONTINUABLE));
    }

    #[test]
    fn test_routable() {
        let mut bits = FlowPropertyBits::new();
        assert!(!bits.routable());
        bits.set(REACHABLE);
        bits.set(ACKABLE);
        assert!(bits.routable());
    }

    #[test]
    fn test_evolvable() {
        let mut bits = FlowPropertyBits::new();
        bits.set(CONTINUABLE);
        bits.set(INCR);
        bits.set(TRANSMISSIBLE);
        assert!(bits.evolvable());
    }

    #[test]
    fn test_alive() {
        let mut bits = FlowPropertyBits::new();
        bits.set(REACHABLE);
        bits.set(CONTINUABLE);
        bits.set(ACKABLE);
        assert!(bits.is_alive());
    }

    #[test]
    fn test_interpretive() {
        let mut bits = FlowPropertyBits::new();
        bits.set(INCR);
        bits.set(TRANSMISSIBLE);
        bits.set(RECOVERABLE);
        assert!(bits.is_interpretive());
    }

    #[test]
    fn test_flow_state_linear() {
        let mut entry = FlowEntry::new(1, FlowKind::Preload);
        assert!(matches!(entry.state, FlowState::Dormant));
        entry.advance();
        assert!(matches!(entry.state, FlowState::Perceived));
        // 推进到完成
        for _ in 0..7 {
            entry.advance();
        }
        assert!(entry.is_complete());
    }

    #[test]
    fn test_flow_sever() {
        let mut entry = FlowEntry::new(1, FlowKind::Preload);
        entry.advance();
        entry.sever();
        assert!(entry.is_broken());
        assert!(!entry.advance()); // 已断裂不可继续
    }

    #[test]
    fn test_distinguisher_classify() {
        let d = Distinguisher::new();
        assert!(matches!(d.classify("囡"), SpouseFamily::Wife));
        assert!(matches!(d.classify("囝"), SpouseFamily::Husband));
        assert!(matches!(d.classify("囡囝"), SpouseFamily::Couple));
        assert!(matches!(d.classify("gtt"), SpouseFamily::HomophoneBridge));
        assert!(matches!(d.classify("?"), SpouseFamily::Neutral));
    }

    #[test]
    fn test_spouse_polarity() {
        assert_eq!(SpouseFamily::Wife.polarity(), 0);
        assert_eq!(SpouseFamily::Husband.polarity(), 1);
        assert_eq!(SpouseFamily::Couple.polarity(), 2);
        assert_eq!(SpouseFamily::Neutral.polarity(), 255);
    }

    #[test]
    fn test_spouse_active() {
        assert!(SpouseFamily::Wife.is_active_sovereign());
        assert!(SpouseFamily::Husband.is_active_sovereign());
        assert!(SpouseFamily::Couple.is_active_sovereign());
        assert!(!SpouseFamily::Neutral.is_active_sovereign());
    }

    #[test]
    fn test_unified_flow_create() {
        let mut uf = UnifiedFlow::new();
        uf.create_flow(1, FlowKind::Preload);
        assert_eq!(uf.count(), 1);
    }

    #[test]
    fn test_unified_flow_step() {
        let mut uf = UnifiedFlow::new();
        uf.create_flow(1, FlowKind::Preload);
        let state = uf.step(1);
        assert!(state.is_some());
    }

    #[test]
    fn test_preload_dictionary() {
        let mut uf = UnifiedFlow::new();
        uf.create_flow(1, FlowKind::Preload);
        uf.create_flow(2, FlowKind::Preload);
        uf.register_preload("gtt_core", vec![1, 2]);
        let result = uf.lookup("gtt_core");
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_perceive_glyph() {
        let uf = UnifiedFlow::new();
        let family = uf.perceive("囡");
        assert!(matches!(family, SpouseFamily::Wife));
    }

    #[test]
    fn test_sacrifice() {
        let mut s = Sacrifice::new(1, "mlct", "mltt");
        assert!(!s.is_paid());
        assert!(s.execute());
        assert!(s.is_paid());
        assert!(!s.execute()); // 已支付不可重复
    }

    #[test]
    fn test_flow_bits_set_clear() {
        let mut bits = FlowPropertyBits::new();
        bits.set(REACHABLE);
        assert!(bits.satisfies(REACHABLE));
        bits.clear(REACHABLE);
        assert!(!bits.satisfies(REACHABLE));
    }

    #[test]
    fn test_unified_stats() {
        let mut uf = UnifiedFlow::new();
        uf.create_flow(1, FlowKind::Relation);
        uf.create_flow(2, FlowKind::Relation);
        assert_eq!(uf.broken_count(), 0);
    }
}
