// ══════════════════════════════════════════════════════════════════════
// cross_isa/morton.rs — 容器莫顿码 · 三维地址 → 线性存储 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 容器坐标系 [X:8][Y:16][Z:8] = 32位地址
//   莫顿码 = 三位交错编织 (Z,Y,X 各 bit 轮流入位)
//   输出: 48位莫顿值 (u64), 空间相近 → 莫顿相近
//
// 用法:
//   let addr = ContainerAddr::new(ContainerId::Add, 0x0A10, 0x5F);
//   let morton = addr.to_morton();   // u64 → 存储索引
//   let recovered = ContainerAddr::from_morton(morton);  // u64 → 地址
//
// 约束: 仅加减 + 位运算, 无乘除
// ══════════════════════════════════════════════════════════════════════

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::vocabulary::ContainerId;

/// 容器地址: ID + 三维坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContainerAddr {
    pub id: ContainerId,
    pub coord: Coord3D,
}

impl ContainerAddr {
    pub fn new(id: ContainerId, coord: Coord3D) -> Self {
        Self { id, coord }
    }

    /// Y操作数 (直接取)
    pub fn operand(&self) -> u16 { self.coord.y }

    /// Z时间窗 (直接取)
    pub fn time_window(&self) -> u8 { self.coord.z }

    /// 32位地址 (与坐标层 to_address 一致)
    pub fn address_32(&self) -> u32 {
        self.coord.to_address()
    }

    /// → 莫顿码 (三维交错)
    pub fn to_morton(self) -> ContainerMorton {
        ContainerMorton::from_coord(self.coord)
    }

    /// ← 莫顿码解码
    pub fn from_morton(morton: ContainerMorton) -> Coord3D {
        morton.to_coord()
    }
}

/// 容器莫顿码 (Z-order / Lebesgue curve)
/// 值越小 = 空间位置越靠近原点 (语义低/操作数低/时间早)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContainerMorton(pub u64);

impl ContainerMorton {
    /// 坐标 → 莫顿码
    /// 交错规则: 每轮 i 取 Z[i], Y[i], X[i]
    /// X/Z 各 8 位, Y 16 位, 共使用 48 位 (u64)
    pub fn from_coord(coord: Coord3D) -> Self {
        let x = coord.x as u64;
        let y = coord.y as u64;
        let z = coord.z as u64;
        Self(interleave_container(x, y, z))
    }

    /// 莫顿码 → 坐标 (解码)
    pub fn to_coord(self) -> Coord3D {
        let (x, y, z) = deinterleave_container(self.0);
        Coord3D::new(x as u8, y as u16, z as u8)
    }

    /// 空间邻居: 同语义下操作数+1
    pub fn next_operand(self) -> Self {
        let mut c = self.to_coord();
        c.y = c.y.wrapping_add(1);
        Self::from_coord(c)
    }

    /// 时间推进: 同一语义+操作数, Z+1
    pub fn next_time(self) -> Self {
        let mut c = self.to_coord();
        c.z = c.z.wrapping_add(1);
        Self::from_coord(c)
    }

    /// 语义前进: X+1 (容器族内移动)
    pub fn next_semantic(self) -> Self {
        let mut c = self.to_coord();
        c.x = c.x.wrapping_add(1);
        Self::from_coord(c)
    }

    /// 莫顿距离 (线性差值, 越小 = 空间越近)
    pub fn distance(self, other: Self) -> u64 {
        let a = self.0;
        let b = other.0;
        if a > b { a - b } else { b - a }
    }

    /// 64位原始值
    pub fn raw(self) -> u64 { self.0 }
}

/// 基于莫顿码的容器存储索引
/// 莫顿键有序排列 → 空间局部性友好的存储布局
#[derive(Debug, Clone, Default)]
pub struct MortonIndex {
    entries: Vec<(ContainerMorton, ContainerId)>,
}

impl MortonIndex {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// 插入容器 (按莫顿值有序)
    pub fn insert(&mut self, coord: Coord3D, id: ContainerId) {
        let morton = ContainerMorton::from_coord(coord);
        // 保持有序: 找到插入位置 (添加循环实现二分插入)
        let pos = self.entries.partition_point(|(m, _)| m.0 <= morton.0);
        self.entries.insert(pos, (morton, id));
    }

    /// 精确查找
    pub fn find(&self, coord: Coord3D) -> Option<ContainerId> {
        let morton = ContainerMorton::from_coord(coord);
        self.entries.iter().find(|(m, _)| m.0 == morton.0).map(|(_, id)| *id)
    }

    /// 范围查询: 莫顿值在 [lo, hi) 范围内的所有容器
    pub fn range(&self, lo: ContainerMorton, hi: ContainerMorton) -> Vec<(ContainerMorton, ContainerId)> {
        let start = self.entries.partition_point(|(m, _)| m.0 < lo.0);
        let end = self.entries.partition_point(|(m, _)| m.0 < hi.0);
        self.entries[start..end].to_vec()
    }

    /// 返回排序后的所有条目
    pub fn entries(&self) -> &[(ContainerMorton, ContainerId)] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ══════════════════════════════════════════════════════════════════════
// 交错 / 解交错 (X:8, Y:16, Z:8)
// ══════════════════════════════════════════════════════════════════════

/// 三维交错: 将 X(8) Y(16) Z(8) 编织到 u64 (48 位有效)
///
/// 位布局 (每轮 i):
///   result 第 (3i)  位 = X 的第 i 位  (i < 8)
///   result 第 (3i+1)位 = Y 的第 i 位  (i < 16)
///   result 第 (3i+2)位 = Z 的第 i 位  (i < 8)
///
/// 完整 16 轮, 共 48 位
pub(crate) fn interleave_container(x: u64, y: u64, z: u64) -> u64 {
    let mut r: u64 = 0;

    // mask 防止越位
    let xm = x & 0xFF;        // 8 bits
    let ym = y & 0xFFFF;      // 16 bits
    let zm = z & 0xFF;        // 8 bits

    // 循环展开核心: 16 轮 (由 Y 的 16 位决定总轮数)
    // X/Z 超出 8 位的轮次自动为 0
    let mut i: u32 = 0;
    while i < 16 {
        let xb = (xm >> i) & 1;
        let yb = (ym >> i) & 1;
        let zb = (zm >> i) & 1;
        r |= xb << (i * 3);
        r |= yb << (i * 3 + 1);
        r |= zb << (i * 3 + 2);
        i += 1;
    }
    r
}

/// 三维解交错: u64 → (X:8, Y:16, Z:8)
pub(crate) fn deinterleave_container(morton: u64) -> (u64, u64, u64) {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let mut z: u64 = 0;

    let mut i: u32 = 0;
    while i < 16 {
        x |= ((morton >> (i * 3)) & 1) << i;
        y |= ((morton >> (i * 3 + 1)) & 1) << i;
        z |= ((morton >> (i * 3 + 2)) & 1) << i;
        i += 1;
    }
    (x & 0xFF, y & 0xFFFF, z & 0xFF)
}

// ══════════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_roundtrip_zero() {
        let c = Coord3D::new(0, 0, 0);
        let m = ContainerMorton::from_coord(c);
        assert_eq!(m.0, 0);
        let back = m.to_coord();
        assert_eq!(c, back);
    }

    #[test]
    fn coord_roundtrip_max() {
        let c = Coord3D::new(0xFF, 0xFFFF, 0xFF);
        let m = ContainerMorton::from_coord(c);
        let back = m.to_coord();
        assert_eq!(c.x, back.x);
        assert_eq!(c.y, back.y);
        assert_eq!(c.z, back.z);
    }

    #[test]
    fn coord_roundtrip_sample() {
        let c = Coord3D::new(0x03, 0x0A10, 0x5F);
        let m = ContainerMorton::from_coord(c);
        let back = m.to_coord();
        assert_eq!(c, back);
    }

    #[test]
    fn coord_all_containers_roundtrip() {
        // 全 64 容器 × 若干 Y/Z 组合的往返
        for x in 0x00u8..=0x3F {
            for y in [0u16, 1, 0x00FF, 0x0A10, 0xFFFF] {
                for z in [0u8, 1, 0x5F, 0xFF] {
                    let c = Coord3D::new(x, y, z);
                    let m = ContainerMorton::from_coord(c);
                    let back = m.to_coord();
                    assert_eq!(c.x, back.x, "X mismatch at x=0x{:02X}", x);
                    assert_eq!(c.y, back.y, "Y mismatch at y=0x{:04X}", y);
                    assert_eq!(c.z, back.z, "Z mismatch at z=0x{:02X}", z);
                }
            }
        }
    }

    #[test]
    fn distinct_coords_yield_distinct_morton() {
        // 不同容器码 → 不同莫顿
        let mut seen = std::collections::HashSet::new();
        for x in 0x00u8..=0x3F {
            let m = ContainerMorton::from_coord(Coord3D::new(x, 0, 0));
            assert!(seen.insert(m.0), "0x{:02X} 产生重复莫顿码", x);
        }
        assert_eq!(seen.len(), 64);
    }

    #[test]
    fn spatial_locality() {
        // 空间相近的容器 → 莫顿差也小
        let a = ContainerMorton::from_coord(Coord3D::new(0x03, 0x0A10, 0x5F));
        let near = ContainerMorton::from_coord(Coord3D::new(0x03, 0x0A11, 0x5F));
        let far = ContainerMorton::from_coord(Coord3D::new(0x3F, 0xFFFF, 0xFF));
        assert!(a.distance(near) < a.distance(far));
    }

    #[test]
    fn next_operand_increments() {
        let a = ContainerMorton::from_coord(Coord3D::new(0x03, 0x0A10, 0x5F));
        let b = a.next_operand();
        let diff = a.distance(b);
        assert!(diff > 0);
    }

    #[test]
    fn next_time_increments() {
        let a = ContainerMorton::from_coord(Coord3D::new(0x03, 0x0A10, 0x5F));
        let b = a.next_time();
        let diff = a.distance(b);
        assert!(diff > 0);
        // 坐标解码验证
        let decoded = b.to_coord();
        assert_eq!(decoded.z, 0x60); // 5F+1 = 60
        assert_eq!(decoded.y, 0x0A10);
        assert_eq!(decoded.x, 0x03);
    }

    #[test]
    fn addr_to_morton_matches_coord() {
        let addr = ContainerAddr::new(
            ContainerId::Add,
            Coord3D::new(0x03, 0x0A10, 0x5F),
        );
        let m_from_addr = addr.to_morton();
        let m_from_coord = ContainerMorton::from_coord(addr.coord);
        assert_eq!(m_from_addr.0, m_from_coord.0);
    }

    // ── MortonIndex 测试 ──

    #[test]
    fn morton_index_insert_and_find() {
        let mut idx = MortonIndex::new();
        idx.insert(Coord3D::new(0x03, 0x0A10, 0x5F), ContainerId::Add);
        idx.insert(Coord3D::new(0x05, 0x0100, 0x10), ContainerId::Sub);

        let found = idx.find(Coord3D::new(0x03, 0x0A10, 0x5F));
        assert_eq!(found, Some(ContainerId::Add));

        let not_found = idx.find(Coord3D::new(0x04, 0x0000, 0x00));
        assert_eq!(not_found, None);
    }

    #[test]
    fn morton_index_ordered_by_morton() {
        let mut idx = MortonIndex::new();
        // 按 X 升序插入
        idx.insert(Coord3D::new(0x05, 0, 0), ContainerId::Sub);
        idx.insert(Coord3D::new(0x03, 0, 0), ContainerId::Add);
        idx.insert(Coord3D::new(0x09, 0, 0), ContainerId::Jmp);

        // 验证排序: 莫顿码递增
        let entries = idx.entries();
        for window in entries.windows(2) {
            assert!(window[0].0 <= window[1].0, "莫顿码未保持有序");
        }
    }

    #[test]
    fn morton_index_range_query() {
        let mut idx = MortonIndex::new();
        for x in 0x01u8..=0x0A {
            idx.insert(Coord3D::new(x, 0, 0), ContainerId::from_u8(x).unwrap());
        }

        // 范围 [0x03, 0x07) 查询
        let lo = ContainerMorton::from_coord(Coord3D::new(0x03, 0, 0));
        let hi = ContainerMorton::from_coord(Coord3D::new(0x07, 0, 0));
        let results = idx.range(lo, hi);
        assert_eq!(results.len(), 4); // 0x03, 0x04, 0x05, 0x06

        // 验证返回的ID正确
        let ids: Vec<u8> = results.iter().map(|(_, id)| *id as u8).collect();
        assert_eq!(ids, vec![0x03, 0x04, 0x05, 0x06]);
    }
}
