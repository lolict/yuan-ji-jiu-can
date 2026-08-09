// preload.rs — 预加载流 (PreloadFlow)
// 满全核心运行时机制。一切"流"(规则/范畴/字典/指令/编码/进制/几何/关系镜像)
// 都是预加载的:启动时加载,运行时只用名字引用,不重新声明。
//
// 三种预加载方式:
//   1. 容器预加载: 划定一个边界,边界内的全部能力自动激活
//   2. 定位预加载: 契约分配代数 — 通过签名/合约暴露未来/当下/过去的数据
//   3. 层级预加载: 标题→内容,一个名字统领无限下属,不需要重复声明

use crate::topo::{self, Symbol, Topology, TopologyMapper};
use crate::rel::{Rel, RelGraph, RelKind};

use std::collections::HashMap;

/// 容器边界类型 — 把这个边界内的全部作为预加载内容。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerKind {
    Cube,       // 立方体容器: 自带相位
    Honeycomb,  // 蜂窝容器: 六边形密铺
    Encoded,    // 编码容器: 64 进制符号表 0..63
    Relational, // 关系容器: 二元/多分派生树
    Mirrored,   // 镜像容器: 因果反转
    Flow,       // 时序流容器: 串联层级
}

/// 契约 — 函数签名式的访问点。
#[derive(Debug, Clone)]
pub struct Contract {
    pub name: &'static str,
    pub input: &'static str,
    pub output: &'static str,
}

impl Contract {
    pub const fn new(name: &'static str, input: &'static str, output: &'static str) -> Self {
        Contract { name, input, output }
    }
}

/// 容器 — 自带边界,内部预加载全部能力,外部只用名字引用。
pub struct Container {
    kind: ContainerKind,
    domains: Vec<&'static str>,
    contracts: Vec<Contract>,
}

impl Container {
    pub fn new(kind: ContainerKind) -> Self {
        Container { kind, domains: Vec::new(), contracts: Vec::new() }
    }

    pub fn assign_to(&mut self, domain: &'static str) {
        if !self.domains.contains(&domain) {
            self.domains.push(domain);
        }
    }

    pub fn is_used_by(&self, domain: &str) -> bool {
        self.domains.iter().any(|d| *d == domain)
    }

    pub fn expose(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }

    pub fn find_contract(&self, name: &str) -> Option<&Contract> {
        self.contracts.iter().find(|c| c.name == name)
    }
}

/// 预加载流 — 全局唯一注册表,启动时加载,运行时调用。
pub struct PreloadFlow {
    containers: HashMap<ContainerKind, Container>,
    default_graph: RelGraph,
}

impl PreloadFlow {
    pub fn new() -> Self {
        let mut flow = PreloadFlow {
            containers: HashMap::new(),
            default_graph: Self::build_default_graph(),
        };

        // 立方体容器
        let mut cube = Container::new(ContainerKind::Cube);
        cube.assign_to("spatial");
        cube.assign_to("geometric");
        cube.expose(Contract::new("phase", "symbol", "Coord3"));
        cube.expose(Contract::new("contains", "range", "...symbols"));
        flow.containers.insert(ContainerKind::Cube, cube);

        // 蜂窝容器
        let mut honey = Container::new(ContainerKind::Honeycomb);
        honey.assign_to("adjacency");
        honey.expose(Contract::new("neighbors", "symbol", "...symbols"));
        flow.containers.insert(ContainerKind::Honeycomb, honey);

        // 编码容器
        let mut enc = Container::new(ContainerKind::Encoded);
        enc.assign_to("symbol_table");
        enc.expose(Contract::new("decode", "symbol", "u8"));
        enc.expose(Contract::new("encode", "u8", "symbol"));
        flow.containers.insert(ContainerKind::Encoded, enc);

        // 关系容器 — 真正接入 rel 子系统
        let mut rel = Container::new(ContainerKind::Relational);
        rel.assign_to("algebra");
        rel.assign_to("graph");
        rel.expose(Contract::new("parents", "u8", "...u8"));     // 溯源祖先
        rel.expose(Contract::new("children", "u8", "...u8"));   // 找派生下属
        rel.expose(Contract::new("common", "Two<u8>", "u8"));   // 共同祖先
        rel.expose(Contract::new("is_owned", "Two<u8>", "bool"));
        flow.containers.insert(ContainerKind::Relational, rel);

        // 镜像容器
        let mut mir = Container::new(ContainerKind::Mirrored);
        mir.assign_to("reverse");
        mir.expose(Contract::new("flip", "Rel", "Rel"));
        flow.containers.insert(ContainerKind::Mirrored, mir);

        // 时序流容器
        let mut fl = Container::new(ContainerKind::Flow);
        fl.assign_to("sequence");
        fl.expose(Contract::new("order", "Two<u8>", "Ordering"));
        fl.expose(Contract::new("chain", "u8", "...u8"));
        flow.containers.insert(ContainerKind::Flow, fl);

        flow
    }

    /// 默认关系图: 整除树 p 是 q 的祖先 ⟺ p 整除 q
    fn build_default_graph() -> RelGraph {
        let mut g = RelGraph::new();
        for n in 1u8..64 {
            // 所有真因数 p(p < n 且 n % p == 0) 都是 n 的祖先
            for p in 1..n {
                if n % p == 0 {
                    g.add(Rel::new(p, n, RelKind::PaiSheng).unwrap());
                }
            }
        }
        g
    }

    /// 查询契约签名
    pub fn use_contract(&self, kind: ContainerKind, contract_name: &str) -> Option<&Contract> {
        self.containers.get(&kind)?.find_contract(contract_name)
    }

    /// 查询容器是否开放给某领域
    pub fn container_available(&self, kind: ContainerKind, domain: &str) -> bool {
        self.containers.get(&kind).map_or(false, |c| c.is_used_by(domain))
    }

    /// 真正调用 — 通过盒子+契约名调起实际运算。
    ///
    /// 例:
    ///   pf.call_relational_parents(6)   => [3, 2, 1](排序后)
    ///   pf.call_sym_phase(17)           => Coord3 { x: 1, y: 0, z: 1 }
    ///   pf.call_mirrored_flip(r)        => r 方向反转
    pub fn call_relational_parents(&self, n: u8) -> Vec<u8> {
        let mut result = self.default_graph.trace_origin(n);
        result.sort_unstable();
        result.dedup();
        result
    }

    pub fn call_relational_children(&self, n: u8) -> Vec<u8> {
        let mut result = self.default_graph.reachable(n);
        result.sort_unstable();
        result.dedup();
        result
    }

    pub fn call_relational_common(&self, a: u8, b: u8) -> Option<u8> {
        self.default_graph.common_origin(a, b)
    }

    pub fn call_sym_phase(&self, s: Symbol) -> topo::Coord3 {
        TopologyMapper::map(Topology::Cube4, s)
    }

    pub fn call_mirrored_flip(&self, r: Rel) -> Rel {
        r.mirror()
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preload_decode_contract_signature() {
        let pf = PreloadFlow::new();
        let c = pf.use_contract(ContainerKind::Encoded, "decode").unwrap();
        assert_eq!(c.input, "symbol");
        assert_eq!(c.output, "u8");
    }

    #[test]
    fn preload_spatial_available() {
        let pf = PreloadFlow::new();
        assert!(pf.container_available(ContainerKind::Cube, "spatial"));
        assert!(!pf.container_available(ContainerKind::Encoded, "graph"));
    }

    #[test]
    fn call_parents_integration() {
        let pf = PreloadFlow::new();
        // #6 的因数: 1, 2, 3
        let parents = pf.call_relational_parents(6);
        assert!(parents.contains(&1));
        assert!(parents.contains(&2));
        assert!(parents.contains(&3));
        assert!(!parents.contains(&6)); // 祖先不包括自身
    }

    #[test]
    fn call_children_integration() {
        let pf = PreloadFlow::new();
        // #2 的倍数(≤63): 4,8,10,12,...(凡是 2×质数 的结果)
        let children = pf.call_relational_children(2);
        // #2 × 2 = 4 (因为 2 是质数,2/2=1,count_ones=1,所以 #2→#4)
        assert!(children.contains(&4));
    }

    #[test]
    fn call_common_origin_integration() {
        let pf = PreloadFlow::new();
        // #6 和 #9 的最大公因数 = 3
        assert_eq!(pf.call_relational_common(6, 9), Some(3));
        // #7 和 #11 互质 → 只有共同祖先 #1
        assert_eq!(pf.call_relational_common(7, 11), Some(1));
    }

    #[test]
    fn call_coord_integration() {
        let pf = PreloadFlow::new();
        let s = Symbol::new(0).unwrap();
        let c = pf.call_sym_phase(s);
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 0);
        assert_eq!(c.z, 0);
        let s17 = Symbol::new(17).unwrap();
        let c17 = pf.call_sym_phase(s17);
        // 17 = 4×4 + 1 → x=1,y=0,z=1
        assert_eq!(c17.x, 1);
        assert_eq!(c17.z, 1);
    }

    #[test]
    fn call_mirror_integration() {
        let pf = PreloadFlow::new();
        let r = Rel::new(3, 6, RelKind::PaiSheng).unwrap();
        let m = pf.call_mirrored_flip(r);
        assert_eq!(m.from, 6);
        assert_eq!(m.to, 3);
        // 派生→归属(镜像后的合理映射)
    }
}
