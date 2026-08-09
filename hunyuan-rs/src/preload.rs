// preload.rs — 预加载流 (PreloadFlow)
// 满全核心运行时机制。一切"流"(规则/范畴/字典/指令/编码/进制/几何/关系镜像)
// 都是预加载的:启动时加载,运行时只用名字引用,不重新声明。
//
// 三种预加载方式:
//   1. 容器预加载: 划定一个边界,边界内的全部能力自动激活
//   2. 定位预加载: 契约分配代数 — 通过签名/合约暴露未来/当下/过去的数据
//   3. 层级预加载: 标题→内容,一个名字统领无限下属,不需要重复声明

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

/// 容器 — 自带边界,内部预加载全部能力,外部只用名字引用。
pub struct Container {
    kind: ContainerKind,
    domains: Vec<&'static str>,  // 这个容器被分配给哪些领域
    contracts: Vec<Contract>,    // 它向外暴露的契约
}

impl Container {
    pub fn new(kind: ContainerKind) -> Self {
        Container { kind, domains: Vec::new(), contracts: Vec::new() }
    }

    /// 把当前容器的能力分配给某领域使用
    pub fn assign_to(&mut self, domain: &'static str) {
        if !self.domains.contains(&domain) {
            self.domains.push(domain);
        }
    }

    /// 检查容器是否被某领域使用
    pub fn is_used_by(&self, domain: &str) -> bool {
        self.domains.iter().any(|d| *d == domain)
    }

    /// 暴露一个契约(函数签名)
    pub fn expose(&mut self, name: &'static str, input: &'static str, output: &'static str) {
        self.contracts.push(Contract { name, input, output });
    }

    /// 查找契约
    pub fn find_contract(&self, name: &str) -> Option<&Contract> {
        self.contracts.iter().find(|c| c.name == name)
    }
}

/// 契约 — 函数签名式的访问点。:
///   名字 = 调用名
///   输入 = 参数类型描述
///   输出 = 返回类型描述
#[derive(Debug, Clone)]
pub struct Contract {
    pub name: &'static str,
    pub input: &'static str,
    pub output: &'static str,
}

/// 预加载流 — 已经是全局唯一注册表,启动时一次性加载。
pub struct PreloadFlow {
    containers: HashMap<ContainerKind, Container>,
}

impl PreloadFlow {
    /// 默认初始化: 加载全部标准容器。启动时调用一次,之后只读。
    pub fn new() -> Self {
        let mut flow = PreloadFlow { containers: HashMap::new() };

        // 立方体:空间相位容器
        let mut cube = Container::new(ContainerKind::Cube);
        cube.assign_to("spatial");
        cube.assign_to("geometric");
        cube.expose("phase", "symbol", "Coord3");
        cube.expose("contains", "range", "Vec<Symbol>");
        flow.containers.insert(ContainerKind::Cube, cube);

        // 蜂窝:邻接关系容器
        let mut honey = Container::new(ContainerKind::Honeycomb);
        honey.assign_to("adjacency");
        honey.expose("neighbors", "symbol", "Vec<Symbol>");
        flow.containers.insert(ContainerKind::Honeycomb, honey);

        // 编码容器
        let mut enc = Container::new(ContainerKind::Encoded);
        enc.assign_to("symbol_table");
        enc.expose("decode", "Symbol", "u8");
        enc.expose("encode", "u8", "Symbol");
        flow.containers.insert(ContainerKind::Encoded, enc);

        // 关系容器
        let mut rel = Container::new(ContainerKind::Relational);
        rel.assign_to("algebra");
        rel.assign_to("graph");
        rel.expose("parents", "u8", "Vec<u8>");     // 溯源:找到祖先
        rel.expose("children", "u8", "Vec<u8>");   // 派生:找到下属
        rel.expose("common", "Two<u8>", "u8");     // 同源:找共同祖先
        flow.containers.insert(ContainerKind::Relational, rel);

        // 镜像容器
        let mut mir = Container::new(ContainerKind::Mirrored);
        mir.assign_to("reverse");
        mir.expose("flip", "Rel", "Rel");   // 关系镜像:反转因果
        flow.containers.insert(ContainerKind::Mirrored, mir);

        // 时序流容器
        let mut fl = Container::new(ContainerKind::Flow);
        fl.assign_to("sequence");
        fl.expose("order", "Two<u8>", "Ordering");  // 判定先后
        fl.expose("chain", "Symbol", "Vec<Symbol>"); // 取时序链
        flow.containers.insert(ContainerKind::Flow, fl);

        flow
    }

    /// 通过容器类型和契约名调用能力。这是唯一允许的访问方式:
    ///   不暴露内部实现,只用 (容器, 契约) 二元组引用。
    pub fn use_contract(&self, kind: ContainerKind, contract_name: &str) -> Option<&Contract> {
        self.containers.get(&kind)?.find_contract(contract_name)
    }

    /// 查询某容器是否开放给某领域
    pub fn container_available(&self, kind: ContainerKind, domain: &str) -> bool {
        self.containers.get(&kind).map_or(false, |c| c.is_used_by(domain))
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preload_invention() {
        let pf = PreloadFlow::new();
        // 编码容器暴露 decode
        let c = pf.use_contract(ContainerKind::Encoded, "decode").unwrap();
        assert_eq!(c.input, "Symbol");
        assert_eq!(c.output, "u8");
    }

    #[test]
    fn preload_spatial_preloaded() {
        let pf = PreloadFlow::new();
        assert!(pf.container_available(ContainerKind::Cube, "spatial"));
        assert!(pf.container_available(ContainerKind::Cube, "geometric"));
        // 编码容器没有分配给 relation 领域
        assert!(!pf.container_available(ContainerKind::Encoded, "graph"));
    }

    #[test]
    fn preload_contract_lookup() {
        let pf = PreloadFlow::new();
        // 关系容器可以直接溯源祖先
        let parents = pf.use_contract(ContainerKind::Relational, "parents").unwrap();
        assert_eq!(parents.name, "parents");
        // 镜像容器提供因果翻转
        let flip = pf.use_contract(ContainerKind::Mirrored, "flip").unwrap();
        assert_eq!(flip.input, "Rel");
    }
}
