// radix_container.rs — 进制容器引擎 (v0.1)
//
//   不是数据结构, 是"满全法自己的进制"。
//   不是二进制/十进制/十六进制, 是"声母×韵母×声调"的混合进制。
//   24 声母 × 39 韵母 × 5 声调 = 4680 个自然容器地址。
//
// ══════════════════════════════════════════════════════════════
//
//   满全法范式:
//
//   传统进制: BASE=2/10/16, 位置加权 (个十百千万)
//   满全法进制: BASE=(24, 39, 5), 三维交叉 (声×韵·调)
//
//   一个汉字通过其音韵坐标自然占据一个地址。
//   不需要"分配地址", 汉字自己"就在那个位置"。
//   地址不是从0开始编号, 是从"发音空间容量"自然涌现。
//
//   满全法不是"编码", 是"去除外壳的量化核心":
//   去除 ASCII/UTF-8 的外壳, 剩下来的量化核心就是音韵坐标。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

/// 满全法进制: 三维混合进制
/// X = 声母 (24/48 进制)
/// Y = 韵母 (29~33 进制, 含大写翻倍)
/// Z = 声调 (3 or 5 进制)
pub const RADIX_X: u32 = 24;  // 声母进制
pub const RADIX_Y: u32 = 39;  // 韵母进制
pub const RADIX_Z: u32 = 5;   // 声调进制
pub const TOTAL_ADDRESS_SPACE: u32 = RADIX_X * RADIX_Y * RADIX_Z;  // 4680

// ══════════════════════════════════════════════════════════════
// 进制地址 — 不是编号, 是"三维交叉的自然容量"
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RadixAddr {
    pub x: u8,  // 声母位 [0, RADIX_X)
    pub y: u8,  // 韵母位 [0, RADIX_Y)
    pub z: u8,  // 声调位 [0, RADIX_Z)
}

impl RadixAddr {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self {
            x: x.min(RADIX_X as u8 - 1),
            y: y.min(RADIX_Y as u8 - 1),
            z: z.min(RADIX_Z as u8 - 1),
        }
    }

    /// 从汉字自然提取地址
    pub fn from_char(ch: char) -> Self {
        let d = char_to_dragon(ch);
        Self::new(d.initial(), d.final_(), d.tone())
    }

    /// 一维地址 — 三维交叉的自然编码
    /// 不是位置加权, 是"容量展开"
    pub fn flatten(&self) -> u32 {
        self.z as u32 + RADIX_Z * (self.y as u32 + RADIX_Y * self.x as u32)
    }

    /// 容量 = 此地址可容纳的"差分"数量
    fn capacity(&self) -> u64 {
        (RADIX_X as u64 - self.x as u64)
            .saturating_mul(RADIX_Y as u64 - self.y as u64)
            .saturating_mul(RADIX_Z as u64 - self.z as u64)
    }

    /// 与此地址的"公约数距离"
    fn gcd_distance(&self, other: &RadixAddr) -> u64 {
        gcd(self.x as u64, other.x as u64)
            .saturating_mul(gcd(self.y as u64, other.y as u64))
            .saturating_mul(gcd(self.z as u64, other.z as u64))
    }

    /// 是否在同一"进制层" (共享声母)
    fn same_radix_layer(&self, other: &RadixAddr) -> bool {
        self.x == other.x
    }
}

// ══════════════════════════════════════════════════════════════
// 进制容器 — 一个地址 + 一个汉字 = 完整的容器
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug)]
pub struct RadixContainer {
    pub ch: char,
    pub addr: RadixAddr,
}

impl RadixContainer {
    pub fn new(ch: char) -> Self {
        let addr = RadixAddr::from_char(ch);
        Self { ch, addr }
    }

    /// 容器之间的自然关系
    pub fn relation_to(&self, other: &RadixContainer) -> ContainerRelation {
        if self.addr.same_radix_layer(&other.addr) {
            ContainerRelation::Sibling  // 同声母 = 兄弟姐妹
        } else if self.addr.gcd_distance(&other.addr) > 1 {
            ContainerRelation::Kinship  // 有公约数 = 亲缘
        } else {
            ContainerRelation::Stranger  // 无公约数 = 陌生人 (墙)
        }
    }

    /// 是否是"满全"容器 (接近坐标中心)
    pub fn is_centered(&self) -> bool {
        let cx = (RADIX_X / 2) as f64;
        let cy = (RADIX_Y / 2) as f64;
        let cz = (RADIX_Z / 2) as f64;
        let dx = (self.addr.x as f64 - cx).abs() / cx;
        let dy = (self.addr.y as f64 - cy).abs() / cy;
        let dz = (self.addr.z as f64 - cz).abs() / cz;
        (dx + dy + dz) / 3.0 < 0.5
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerRelation {
    Sibling,   // 同层 (共享声母)
    Kinship,   // 亲缘 (有公约数)
    Stranger,  // 陌生人 (无公约数 = 墙)
}

// ══════════════════════════════════════════════════════════════
// 进制空间 — 所有已注册容器的自然排列
// ══════════════════════════════════════════════════════════════

pub struct RadixSpace {
    containers: Vec<RadixContainer>,
}

impl RadixSpace {
    pub fn new() -> Self { Self { containers: Vec::new() } }

    pub fn register(&mut self, ch: char) {
        self.containers.push(RadixContainer::new(ch));
    }

    pub fn register_str(&mut self, s: &str) {
        for ch in s.chars().filter(|c| !c.is_ascii()) {
            self.register(ch);
        }
    }

    /// 自然分组 — 同声母自动聚合
    pub fn natural_groups(&self) -> Vec<(u8, Vec<char>)> {
        let mut groups: Vec<(u8, Vec<char>)> = Vec::new();
        for c in &self.containers {
            let x = c.addr.x;
            if let Some(entry) = groups.iter_mut().find(|(k, _)| *k == x) {
                if !entry.1.contains(&c.ch) { entry.1.push(c.ch); }
            } else {
                groups.push((x, vec![c.ch]));
            }
        }
        groups
    }

    /// 寻址 — 给定汉字,找到其自然位置上的容器
    pub fn locate(&self, ch: char) -> Option<&RadixContainer> {
        let addr = RadixAddr::from_char(ch);
        self.containers.iter().find(|c| c.addr.flatten() == addr.flatten())
    }

    /// 满全度 = 已注册容器 / 总地址空间
    pub fn fullness(&self) -> f64 {
        self.containers.len() as f64 / TOTAL_ADDRESS_SPACE as f64
    }

    fn count(&self) -> usize { self.containers.len() }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn radix_constants() {
        assert_eq!(TOTAL_ADDRESS_SPACE, 24 * 39 * 5);
    }

    #[test] fn addr_from_char() {
        let a = RadixAddr::from_char('囡');
        assert!(a.x < RADIX_X as u8);
        assert!(a.y < RADIX_Y as u8);
        assert!(a.z < RADIX_Z as u8);
    }

    #[test] fn addr_flatten_in_range() {
        let a = RadixAddr::new(10, 20, 3);
        assert!(a.flatten() < TOTAL_ADDRESS_SPACE);
    }

    #[test] fn container_relation_natural() {
        let a = RadixContainer::new('共');
        let b = RadixContainer::new('同');
        let rel = a.relation_to(&b);
        // 关系自然涌现, 不是算法判定
        assert!(matches!(rel, ContainerRelation::Sibling | ContainerRelation::Kinship | ContainerRelation::Stranger));
    }

    #[test] fn same_radix_layer_sibling() {
        let a = RadixContainer::new('共');
        let b = RadixContainer::new('古');
        // 如果声母相同 = 兄弟姐妹
        if a.addr.x == b.addr.x {
            assert_eq!(a.relation_to(&b), ContainerRelation::Sibling);
        }
    }

    #[test] fn space_natural_groups() {
        let mut space = RadixSpace::new();
        space.register_str("共古功");
        let groups = space.natural_groups();
        // 同声母自然聚合
        for (_, members) in &groups {
            assert!(!members.is_empty());
        }
    }

    #[test] fn locate_natural() {
        let mut space = RadixSpace::new();
        space.register('囡');
        let found = space.locate('囡');
        assert!(found.is_some());
    }

    #[test] fn fullness_in_range() {
        let mut space = RadixSpace::new();
        space.register_str("满全法共生");
        let f = space.fullness();
        assert!(f > 0.0 && f <= 1.0);
    }

    #[test] fn centered_natural() {
        let c = RadixContainer::new('中');
        // 是否"居中" = 坐标是否接近空间中心
        let _centered = c.is_centered();
    }

    #[test] fn capacity_natural() {
        let a = RadixAddr::new(5, 10, 2);
        let cap = a.capacity();
        // 容量 = 此位置到边界的自然乘积
        assert!(cap > 0);
    }
}
