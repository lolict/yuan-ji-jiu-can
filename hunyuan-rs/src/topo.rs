// topo.rs — 拓扑描述子系统
// 把身体部位秩序 + 几何形容词 + 64 符号排布统一在一个位编码体系里。
//
// 设计原则：
//   - 没有字符串关键字，一切是位。
//   - 时间先后 = 空间串联：符号枚举顺序就是排布顺序。
//   - 几何形容词 = 拓扑属性标签，可组合（位标志）。
//   - 多拓扑可切换，符号层不变，坐标映射函数替换。

// ═══════════════════════════════════════════════════════════════
// 第一层：64 进制符号表（固定 6 位，0~63）
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub u8);   // 只取低 6 位 (0..64)

impl Symbol {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 63;
    pub const COUNT: u8 = 64;

    pub fn new(v: u8) -> Option<Self> {
        if v < 64 { Some(Symbol(v)) } else { None }
    }

    pub fn as_u8(self) -> u8 { self.0 }

    /// 时间先后顺序即 u8 大小顺序。
    pub fn time_index(self) -> u8 { self.0 }
}

// ═══════════════════════════════════════════════════════════════
// 第二层：身体即坐标（用部位做空间轴，不含字符标签）
// ═══════════════════════════════════════════════════════════════

/// 身体纵轴 — 从肩到脚（6 档，用 3 位表示，最多支持 8 档）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyAxis {
    Head = 0,     // 面/头顶
    Neck = 1,     // 脖侧
    Shoulder = 2, // 肩 / 肩侧
    Chest = 3,    // 胸侧
    Waist = 4,    // 腰 / 腰侧
    Navel = 5,    // 脐
    Foot = 6,     // 脚 / 脚侧
    Ground = 7,   // 地
}

impl BodyAxis {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(BodyAxis::Head),
            1 => Some(BodyAxis::Neck),
            2 => Some(BodyAxis::Shoulder),
            3 => Some(BodyAxis::Chest),
            4 => Some(BodyAxis::Waist),
            5 => Some(BodyAxis::Navel),
            6 => Some(BodyAxis::Foot),
            7 => Some(BodyAxis::Ground),
            _ => None,
        }
    }
}

/// 八方 — 东西南北 + 四维（4 位编码，可扩展至 16 方向）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction8 {
    East  = 0,  // 东
    West  = 1,  // 西
    South = 2,  // 南
    North = 3,  // 北
    Up    = 4,  // 天
    Down  = 5,  // 地
    Center = 6, // 中
    Reserved = 7,
}

// ═══════════════════════════════════════════════════════════════
// 第三层：几何形容词 = 拓扑属性位标志（每位一个布尔属性，可组合）
// ═══════════════════════════════════════════════════════════════

bitflags::bitflags! {
    /// 拓扑标签：描述一个单元格外在几何属性。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TopoLabel: u32 {
        // ── 位置关系 ──────────────────────────────
        const INNER          = 1 << 0;   // 内   - 内部/里面
        const OUTER          = 1 << 1;   // 外   - 外部/表面
        const OPPOSITE       = 1 << 2;   // 对   - 对面的
        const ORTHOGONAL     = 1 << 3;   // 正交 - 垂直相交
        const DIAGONAL       = 1 << 4;   // 斜对角
        const CROSS          = 1 << 5;   // 十字节点

        // ── 形态 ─────────────────────────────────
        const CORNER         = 1 << 6;   // 角
        const EDGE           = 1 << 7;   // 边
        const FACE           = 1 << 8;   // 面
        const CELL           = 1 << 9;   // 内格子
        const FRAME          = 1 << 10;  // 外矩框
        const NODE           = 1 << 11;  // 节点
        const PERSPECTIVE    = 1 << 12;  // 透视图

        // ── 数量度量 ─────────────────────────────
        const COUNT_2        = 1 << 13;  // 内2 / 对面 2
        const COUNT_6        = 1 << 14;  // 六面 / 六对
        const THIN           = 1 << 15;  // 面薄
        const THICK          = 1 << 16;  // 面厚

        // ── 结构状态 ─────────────────────────────
        const VALLEY         = 1 << 17;  // 内谷边
        const PEAK           = 1 << 18;  // 外峰边
        const BISECTOR       = 1 << 19;  // 对角线 / 斜对角正交线
        const AB_PAIR        = 1 << 20;  // 内六对 ab 面

        // ── 一致性 ───────────────────────────────
        const EQUIVALENT     = 1 << 21;  // 几何等效
        const MULTICENTER    = 1 << 22;  // 容器中心数量 > 1
    }
}

// ═══════════════════════════════════════════════════════════════
// 第四层：单元格 = 符号 + 拓扑标签 + 坐标（由拓扑决定含义）
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub struct TopoCell {
    pub symbol: Symbol,
    pub label: TopoLabel,
}

impl TopoCell {
    pub fn new(symbol: Symbol) -> Self {
        TopoCell { symbol, label: TopoLabel::empty() }
    }

    pub fn with_label(symbol: Symbol, label: TopoLabel) -> Self {
        TopoCell { symbol, label }
    }

    pub fn is_equivalent_to(self, other: Self) -> bool {
        self.label.contains(TopoLabel::EQUIVALENT) &&
        other.label.contains(TopoLabel::EQUIVALENT) &&
        self.label.iter().count() == other.label.iter().count()
    }
}

// ═══════════════════════════════════════════════════════════════
// 第五层：拓扑坐标映射（坐标类型 + 映射函数，拓扑无关）
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Topology {
    Cube4,      // 4×4×4 立方
    Honeycomb,  // 蜂窝形（六边形密铺）
    Star,       // 星形（1 中心 + 辐射）
    Sphere,     // 球形（环形 + 极）
    Plane,      // 无边平面
    MultiCenter,// 多中心型
}

/// 三维坐标（由 Topology 解释含义，支持负值）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub struct TopologyMapper;

impl TopologyMapper {
    /// 把符号（0~63）映射到给定拓扑下的坐标。
    pub fn map(topo: Topology, s: Symbol) -> Coord3 {
        let i = s.as_u8();
        match topo {
            Topology::Cube4 => {
                Coord3 {
                    x: (i % 4) as i8,
                    y: ((i / 4) % 4) as i8,
                    z: (i / 16) as i8,
                }
            }
            // 蜂窝映射：6 环 × ~10 格（简化为环 + 扇区）
            Topology::Honeycomb => {
                let ring = i / 10;
                let sector = i % 10;
                Coord3 { x: ring as i8, y: sector as i8, z: 0 }
            }
            // 星形：中心(0,0,0)是 #0，其余按 8 方向辐射
            Topology::Star => {
                if i == 0 {
                    Coord3 { x: 0, y: 0, z: 0 }
                } else {
                    let dir = (i - 1) % 8;  // 8 个方向
                    // 用简单坐标：每个方向在 x 或 y 上偏移 1
                    let (dx, dy) = match dir {
                        0 => (1, 0),   // 东
                        1 => (-1, 0),  // 西
                        2 => (0, -1),  // 南
                        3 => (0, 1),   // 北
                        4 => (1, 1),   // 天/东北
                        5 => (-1, -1), // 地/西南
                        _ => (1, 0),
                    };
                    Coord3 { x: dx, y: dy, z: 0 }
                }
            }
            // 球面：经度 + 极
            Topology::Sphere => {
                let lon = i % 8;
                let lat = i / 8;
                Coord3 { x: lon as i8, y: lat as i8, z: 0 }
            }
            // 平面：无边矩阵（8×8）
            Topology::Plane => {
                Coord3 {
                    x: (i % 8) as i8,
                    y: (i / 8) as i8,
                    z: 0,
                }
            }
            // 多中心：每个中心 8 格
            Topology::MultiCenter => {
                let center = i / 8;
                let pos = i % 8;
                Coord3 { x: center as i8, y: pos as i8, z: 0 }
            }
        }
    }

    /// 同一拓扑中，两个符号的曼哈顿距离（邻接度量）。
    pub fn manhattan(topo: Topology, a: Symbol, b: Symbol) -> u8 {
        let ca = Self::map(topo, a);
        let cb = Self::map(topo, b);
        ((ca.x - cb.x).abs() + (ca.y - cb.y).abs() + (ca.z - cb.z).abs()) as u8
    }

    /// 是否邻接（距离 == 1）。
    pub fn are_adjacent(topo: Topology, a: Symbol, b: Symbol) -> bool {
        Self::manhattan(topo, a, b) == 1
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_range() {
        assert_eq!(Symbol::new(0).unwrap().as_u8(), 0);
        assert_eq!(Symbol::new(63).unwrap().as_u8(), 63);
        assert!(Symbol::new(64).is_none());
    }

    #[test]
    fn cube_mapping_range() {
        let s0 = Symbol::new(0).unwrap();
        let s63 = Symbol::new(63).unwrap();
        let c0 = TopologyMapper::map(Topology::Cube4, s0);
        let c63 = TopologyMapper::map(Topology::Cube4, s63);
        assert_eq!(c0, Coord3 { x: 0, y: 0, z: 0 });
        assert_eq!(c63, Coord3 { x: 3, y: 3, z: 3 });
    }

    #[test]
    fn cube_adjacency() {
        let s0 = Symbol::new(0).unwrap();
        let s1 = Symbol::new(1).unwrap();
        let s4 = Symbol::new(4).unwrap();
        let s5 = Symbol::new(5).unwrap();
        assert!(TopologyMapper::are_adjacent(Topology::Cube4, s0, s1));
        assert!(TopologyMapper::are_adjacent(Topology::Cube4, s0, s4));
        assert!(!TopologyMapper::are_adjacent(Topology::Cube4, s0, s5));
    }

    #[test]
    fn star_center_adjacency() {
        let center = Symbol::new(0).unwrap();
        let arm1 = Symbol::new(1).unwrap();
        assert!(TopologyMapper::are_adjacent(Topology::Star, center, arm1));
    }

    #[test]
    fn multi_topology_same_symbol() {
        let s32 = Symbol::new(32).unwrap();
        let cube = TopologyMapper::map(Topology::Cube4, s32);
        let star = TopologyMapper::map(Topology::Star, s32);
        let plane = TopologyMapper::map(Topology::Plane, s32);
        assert_ne!(cube, star);
        assert_ne!(cube, plane);
    }

    #[test]
    fn topo_label_combinations() {
        let l = TopoLabel::INNER | TopoLabel::CORNER | TopoLabel::EQUIVALENT;
        assert!(l.contains(TopoLabel::INNER));
        assert!(l.contains(TopoLabel::CORNER));
        assert!(!l.contains(TopoLabel::OUTER));
    }

    #[test]
    fn equivalence_check() {
        let c1 = TopoCell::with_label(
            Symbol::new(0).unwrap(),
            TopoLabel::INNER | TopoLabel::CORNER | TopoLabel::EQUIVALENT,
        );
        let c2 = TopoCell::with_label(
            Symbol::new(1).unwrap(),
            TopoLabel::INNER | TopoLabel::CORNER | TopoLabel::EQUIVALENT,
        );
        assert!(c1.is_equivalent_to(c2));
    }

    #[test]
    fn time_order_equals_symbol_order() {
        let prev = Symbol::new(10).unwrap();
        let next = Symbol::new(11).unwrap();
        assert!(prev.time_index() < next.time_index());
    }
}
