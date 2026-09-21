// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.65 容器中间表示 — Container Intermediate Representation      ║
// ║  统一地址空间 · 容器=字体=Time · 代数选址替代 · 关系归属计算      ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 问题:
//   毛利兰用 MaoriAddr(u64), 时钟用 ClockActionKey(u64),
//   分身用 AvatarId(String), 域用 SymbolId(u32).
//   它们各自为政, 跨模块转换靠 to_string + re-parse — 这就是"代数选址".
//
// 容器本体论替代:
//   一切地址都是一个 ContainerRef — 编码为 u128 的统一容器标识.
//   ContainerRef 不关心你用什么语言产生它, 只关心它指向什么"在场实体".
//
// 64 位布局 (高 64 位 = 命名空间, 低 64 位 = 实体):
//   [127:124] = 容器类型 (4 bits)
//   [123:116] = 卦限/世界 (8 bits)
//   [115:64]  = 上层路径 (52 bits)
//   [63:0]    = 实体编码 (64 bits, 直接嵌入毛利兰/时钟编码)

/// 容器类型: 统一标识不同来源的编码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerKind {
    Maori,      // 毛利兰立体编码
    Clock,      // 时钟因果密钥
    Avatar,     // 分身身份
    Symbol,     // 域符号 ID
    Relation,   // 串联/因果/纠缠关系
}

impl ContainerKind {
    pub fn type_bit(&self) -> u8 {
        match self {
            ContainerKind::Maori => 0,
            ContainerKind::Clock => 1,
            ContainerKind::Avatar => 2,
            ContainerKind::Symbol => 3,
            ContainerKind::Relation => 4,
        }
    }

    pub fn from_bit(bit: u8) -> Option<Self> {
        match bit {
            0 => Some(ContainerKind::Maori),
            1 => Some(ContainerKind::Clock),
            2 => Some(ContainerKind::Avatar),
            3 => Some(ContainerKind::Symbol),
            4 => Some(ContainerKind::Relation),
            _ => None,
        }
    }
}

/// 统一容器引用: 跨所有编码系统的唯一标识
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContainerRef(pub u128);

impl ContainerRef {
    const KIND_SHIFT: u128 = 124;
    const OCTANT_SHIFT: u128 = 116;
    const NS_MASK: u128 = 0x0F;
    const OCT_MASK: u128 = 0xFF;
    const ENTITY_MASK: u128 = u64::MAX as u128;

    /// 构造: 类型 + 实体编码 → 容器引用
    pub fn new(kind: ContainerKind, entity: u64) -> Self {
        let k = (kind.type_bit() as u128) << Self::KIND_SHIFT;
        ContainerRef(k | (entity as u128))
    }

    /// 带卦限的构造 (用于需要在多层空间中定位的场景)
    pub fn with_octant(kind: ContainerKind, entity: u64, octant: u8) -> Self {
        ContainerRef::compose(kind, octant, 0, entity)
    }

    /// 完整构造: 类型 + 卦限 + 命名空间 + 实体
    pub fn compose(kind: ContainerKind, octant: u8, namespace: u64, entity: u64) -> Self {
        let k = (kind.type_bit() as u128) << Self::KIND_SHIFT;
        let o = (octant as u128) << Self::OCTANT_SHIFT;
        let n = ((namespace & 0x000FFFFFFFFFFFFF) as u128) << 64;
        ContainerRef(k | o | n | (entity as u128))
    }

    pub fn kind(&self) -> Option<ContainerKind> {
        let bit = ((self.0 >> Self::KIND_SHIFT) & Self::NS_MASK) as u8;
        ContainerKind::from_bit(bit)
    }

    pub fn octant(&self) -> u8 {
        ((self.0 >> Self::OCTANT_SHIFT) & Self::OCT_MASK) as u8
    }

    pub fn namespace(&self) -> u64 {
        ((self.0 >> 64) & 0x000FFFFFFFFFFFFF) as u64
    }

    pub fn entity(&self) -> u64 {
        (self.0 & Self::ENTITY_MASK) as u64
    }

    pub fn raw(&self) -> u128 { self.0 }

    /// 是否与另一引用指向同一容器 (忽略命名空间差异)
    pub fn same_container(&self, other: &ContainerRef) -> bool {
        self.kind() == other.kind() && self.entity() == other.entity()
    }
}

impl std::fmt::Display for ContainerRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = match self.kind() {
            Some(ContainerKind::Maori) => "毛利兰",
            Some(ContainerKind::Clock) => "时钟",
            Some(ContainerKind::Avatar) => "分身",
            Some(ContainerKind::Symbol) => "符号",
            Some(ContainerKind::Relation) => "关系",
            None => "?",
        };
        write!(f, "C<{}|{:016x}>", k, self.entity())
    }
}

/// 容器关系: 两个容器之间的一条边
#[derive(Clone, Debug)]
pub struct ContainerEdge {
    pub from: ContainerRef,
    pub to: ContainerRef,
    pub relation: RelationKind,
    pub weight: u32,  // 加法统计: 此关系被见证的次数
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationKind {
    Resonance,    // 共振
    Causal,       // 因果
    Spouse,       // 夫妻 (满全法核心)
    Parasite,     // 寄生/宿主
    ContainerOf,  // 容器包含
    Preload,      // 预加载引用
}

/// 容器图集: 存储所有容器引用和它们之间的关系
pub struct ContainerAtlas {
    pub containers: Vec<ContainerRef>,
    pub edges: Vec<ContainerEdge>,
    index: std::collections::HashMap<u64, usize>,  // entity → containers 中的位置
}

impl ContainerAtlas {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
            edges: Vec::new(),
            index: std::collections::HashMap::new(),
        }
    }

    /// 注册一个容器, 返回其在该图集中的序号
    pub fn register(&mut self, r: ContainerRef) -> usize {
        if let Some(&idx) = self.index.get(&r.entity()) {
            return idx;
        }
        let idx = self.containers.len();
        self.containers.push(r);
        self.index.insert(r.entity(), idx);
        idx
    }

    /// 添加关系
    pub fn link(&mut self, from: ContainerRef, to: ContainerRef, relation: RelationKind, weight: u32) {
        self.register(from);
        self.register(to);
        self.edges.push(ContainerEdge { from, to, relation, weight });
    }

    /// 查找某容器的所有邻接
    pub fn neighbors(&self, r: &ContainerRef) -> Vec<(usize, &ContainerEdge)> {
        self.edges.iter()
            .enumerate()
            .filter(|(_, e)| e.from.same_container(r) || e.to.same_container(r))
            .collect()
    }

    /// 按关系类型筛选边
    pub fn edges_of(&self, kind: &RelationKind) -> Vec<&ContainerEdge> {
        self.edges.iter().filter(|e| &e.relation == kind).collect()
    }

    pub fn count(&self) -> usize { self.containers.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }

    /// 加法权重和 (统计在场总量)
    pub fn total_weight(&self) -> u64 {
        self.edges.iter().map(|e| e.weight as u64).sum()
    }
}

/// 转换辅助: 各编码系统 → ContainerRef
pub mod convert {
    use super::*;

    /// 毛利兰地址 → 容器引用
    pub fn maori_to_ref(addr: &crate::mlil::MaoriAddr) -> ContainerRef {
        let maori = crate::mlil::MaoriAddr(addr.raw());
        ContainerRef::with_octant(ContainerKind::Maori, maori.raw(), maori.octant())
    }

    /// 容器引用 → 毛利兰地址 (如果类型匹配)
    pub fn ref_to_maori(r: &ContainerRef) -> Option<crate::mlil::MaoriAddr> {
        if r.kind() == Some(ContainerKind::Maori) {
            Some(crate::mlil::MaoriAddr(r.entity()))
        } else {
            None
        }
    }

    /// 域 SymbolId → ContainerRef
    pub fn symbol_to_ref(id: crate::domain_preload::SymbolId, domain_hash: u64) -> ContainerRef {
        ContainerRef::compose(ContainerKind::Symbol, 0, domain_hash, id.0 as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_ref_new_preserves_entity() {
        let r = ContainerRef::new(ContainerKind::Maori, 0xDEADBEEF);
        assert_eq!(r.entity(), 0xDEADBEEF);
        assert_eq!(r.kind(), Some(ContainerKind::Maori));
    }

    #[test]
    fn container_ref_with_octant() {
        let r = ContainerRef::with_octant(ContainerKind::Clock, 12345, 7);
        assert_eq!(r.octant(), 7);
        assert_eq!(r.entity(), 12345);
    }

    #[test]
    fn compose_full_layout() {
        let r = ContainerRef::compose(ContainerKind::Symbol, 3, 99, 42);
        assert_eq!(r.kind(), Some(ContainerKind::Symbol));
        assert_eq!(r.octant(), 3);
        assert_eq!(r.namespace(), 99);
        assert_eq!(r.entity(), 42);
    }

    #[test]
    fn same_container_ignores_namespace() {
        let r1 = ContainerRef::compose(ContainerKind::Maori, 0, 100, 42);
        let r2 = ContainerRef::compose(ContainerKind::Maori, 0, 200, 42);
        assert!(r1.same_container(&r2));
    }

    #[test]
    fn different_kind_not_same() {
        let r1 = ContainerRef::new(ContainerKind::Maori, 42);
        let r2 = ContainerRef::new(ContainerKind::Clock, 42);
        assert!(!r1.same_container(&r2));
    }

    #[test]
    fn atlas_register_and_link() {
        let mut atlas = ContainerAtlas::new();
        let a = ContainerRef::new(ContainerKind::Maori, 1);
        let b = ContainerRef::new(ContainerKind::Clock, 2);
        atlas.link(a.clone(), b.clone(), RelationKind::Causal, 1);
        assert_eq!(atlas.count(), 2);
        assert_eq!(atlas.edge_count(), 1);
        assert_eq!(atlas.total_weight(), 1);
    }

    #[test]
    fn atlas_neighbors() {
        let mut atlas = ContainerAtlas::new();
        let a = ContainerRef::new(ContainerKind::Maori, 100);
        let b = ContainerRef::new(ContainerKind::Clock, 200);
        let c = ContainerRef::new(ContainerKind::Avatar, 300);
        atlas.link(a.clone(), b.clone(), RelationKind::Resonance, 2);
        atlas.link(a.clone(), c.clone(), RelationKind::ContainerOf, 1);

        let n = atlas.neighbors(&a);
        assert_eq!(n.len(), 2);
    }

    #[test]
    fn atlas_dedup_same_entity() {
        let mut atlas = ContainerAtlas::new();
        let a = ContainerRef::new(ContainerKind::Maori, 42);
        atlas.register(a.clone());
        atlas.register(a.clone());  // 重复
        atlas.register(ContainerRef::new(ContainerKind::Maori, 42));  // 同实体
        assert_eq!(atlas.count(), 1);
    }

    #[test]
    fn atlas_edges_of_relation() {
        let mut atlas = ContainerAtlas::new();
        let a = ContainerRef::new(ContainerKind::Maori, 1);
        let b = ContainerRef::new(ContainerKind::Clock, 2);
        let c = ContainerRef::new(ContainerKind::Avatar, 3);
        atlas.link(a.clone(), b.clone(), RelationKind::Causal, 1);
        atlas.link(b.clone(), c.clone(), RelationKind::Resonance, 1);
        atlas.link(a.clone(), c.clone(), RelationKind::Causal, 1);

        let causal = atlas.edges_of(&RelationKind::Causal);
        assert_eq!(causal.len(), 2);
        let reso = atlas.edges_of(&RelationKind::Resonance);
        assert_eq!(reso.len(), 1);
    }

    #[test]
    fn display_format() {
        let r = ContainerRef::new(ContainerKind::Maori, 0xABCD);
        let s = format!("{}", r);
        assert!(s.contains("毛利兰"));
        assert!(s.contains("abcd"));
    }

    #[test]
    fn roundtrip_entity_64bit() {
        // 测试极端实体值
        let max_entity = u64::MAX;
        let r = ContainerRef::new(ContainerKind::Relation, max_entity);
        assert_eq!(r.entity(), max_entity);
    }
}
