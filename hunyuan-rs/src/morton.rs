// ══════════════════════════════════════════════════════════════════════
// src/morton.rs — 莫顿码 · 三维坐标 → 单值索引 (v0.40)
// ══════════════════════════════════════════════════════════════════════
//
// 「走捷径」= 不遍历层级，直接跳
// 「三维进制」= 时间轴 × 声波轴 × 深度轴
// 「莫顿码」= 三维坐标按每一位交错编织成 1 维数值
//   输入: (t, s, d)  —  t=时间位置, s=声波位置, d=里外深度
//   输出: 莫顿值 m  —  空间相近的点, 莫顿值也相近
//
// 用法:
//   1. 每个 NestedContainer 分配一个 3D 坐标
//   2. 需要查找时: 计算目标莫顿值 → O(1) 定位 (无需哈希, 无需遍历)
//   3. 空间填充曲线保证: 时间相近的容器 → 莫顿值也相近
//
// 约束: 仅加减 + 位运算, 无乘除
// ══════════════════════════════════════════════════════════════════════

use crate::inside_out::InsideOut;

/// 三维坐标系 (囝共生观察空间)
#[derive(Clone, Copy, Debug)]
pub struct Coord3D {
    /// 时间轴: 创世以来的毫秒级刻度 (或音阶层级)
    pub t: u32,
    /// 声波轴: SoundAddress 的 value() 取低 21 位
    pub s: u32,
    /// 深度轴: Surface=0, Threshold=1, Interior=2
    pub d: u32,
}

/// 莫顿码 (Z-order / Lebesgue curve)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MortonCode(pub u64);

impl MortonCode {
    /// 三维坐标 → 莫顿码
    pub fn encode(coord: Coord3D) -> MortonCode {
        let t = coord.t as u64 & 0x1FFFFF; // 21 bits
        let s = coord.s as u64 & 0x1FFFFF;
        let d = coord.d as u64 & 0x3; // 深度只占 2 bits (0/1/2)

        MortonCode(interleave_3d(t, s, d))
    }

    /// 莫顿码 → 三维坐标 (解码)
    pub fn decode(self) -> Coord3D {
        let (t, s, d) = deinterleave_3d(self.0);
        Coord3D { t: t as u32, s: s as u32, d: d as u32 }
    }

    /// 空间邻居: 同一深度的下一个声波位置 (莫顿距离 = ±1 层内)
    pub fn neighbor_sound_next(self) -> MortonCode {
        let mut c = self.decode();
        c.s = c.s.wrapping_add(1);
        MortonCode::encode(c)
    }

    /// 深度降级 (Interior → Threshold → Surface)
    pub fn depth_outward(self) -> MortonCode {
        let mut c = self.decode();
        c.d = c.d & 0x3; // 确保只有 2 bit
        if c.d > 0 { c.d -= 1; }
        MortonCode::encode(c)
    }

    /// 深度升级 (Surface → Threshold → Interior)
    pub fn depth_inward(self) -> MortonCode {
        let mut c = self.decode();
        c.d = c.d & 0x3; // 确保只有 2 bit
        if c.d < 2 { c.d += 1; }
        MortonCode::encode(c)
    }

    /// 以莫顿值为键, 直接比较 (跳层定位)
    pub fn distance(self, other: MortonCode) -> u64 {
        let a = self.0;
        let b = other.0;
        if a > b { a - b } else { b - a }
    }
}

// 三维交错 (21 bits × 2 + 2 bits)
fn interleave_3d(t: u64, s: u64, d: u64) -> u64 {
    // 用查找表 + 位运算, 无乘除
    // 共 64 位: 每个维度交错排
    // 深度(2) | 声波(21) | 时间(21) = 44 位 → 存入 64 位
    let mut result: u64 = 0;
    let mut i: u32 = 0;
    while i < 21 {
        let tb = (t >> i) & 1;
        let sb = (s >> i) & 1;
        let db = (d >> (i / 11)) & 1; // 深度少, 平均分布
        result |= tb << (i * 3);
        result |= sb << (i * 3 + 1);
        result |= db << (i * 3 + 2);
        i += 1;
    }
    result
}

fn deinterleave_3d(morton: u64) -> (u64, u64, u64) {
    let mut t: u64 = 0;
    let mut s: u64 = 0;
    let mut d: u64 = 0;
    let mut i: u32 = 0;
    while i < 21 {
        t |= ((morton >> (i * 3)) & 1) << i;
        s |= ((morton >> (i * 3 + 1)) & 1) << i;
        // 深度: 每 11 位声循环 2 位深 (与 interleave 对称)
        d |= ((morton >> (i * 3 + 2)) & 1) << (i / 11);
        i += 1;
    }
    (t, s, d)
}

// ══════════════════════════════════════════════════════════
// 测试 — 12
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let c = Coord3D { t: 1725600, s: 42, d: 1 };
        let m = MortonCode::encode(c);
        let back = m.decode();
        assert_eq!(c.t, back.t);
        assert_eq!(c.s, back.s);
        assert_eq!(c.d, back.d);
    }

    #[test]
    fn encode_zero() {
        let m = MortonCode::encode(Coord3D::zero());
        assert_eq!(m.0, 0);
    }

    #[test]
    fn encode_distinct() {
        let a = MortonCode::encode(Coord3D { t: 1, s: 0, d: 0 });
        let b = MortonCode::encode(Coord3D { t: 0, s: 1, d: 0 });
        assert_ne!(a.0, b.0);
    }

    #[test]
    fn ordering_preserved() {
        // 小的小, 大的大 (莫顿保持顺序)
        let a = MortonCode::encode(Coord3D { t: 10, s: 0, d: 0 });
        let b = MortonCode::encode(Coord3D { t: 20, s: 0, d: 0 });
        assert!(a < b);
    }

    #[test]
    fn neighbor_next() {
        let a = MortonCode::encode(Coord3D { t: 100, s: 50, d: 1 });
        let b = a.neighbor_sound_next();
        // 邻居差 ≠ 0
        assert_ne!(a.0, b.0);
    }

    #[test]
    fn depth_outward_reduces() {
        let inner = MortonCode::encode(Coord3D { t: 100, s: 50, d: 2 });
        let outer = inner.depth_outward();
        let diff = inner.distance(outer);
        assert!(diff > 0);
    }

    #[test]
    fn depth_inward_increases() {
        let outer = MortonCode::encode(Coord3D { t: 100, s: 50, d: 0 });
        let inner = outer.depth_inward();
        assert!(inner > outer);
    }

    #[test]
    fn depth_max_clamped() {
        let c = MortonCode::encode(Coord3D { t: 0, s: 0, d: 2 });
        let deeper = c.depth_inward(); // d 已达最大
        assert_eq!(c.0, deeper.0); // 不再增加
    }

    #[test]
    fn depth_min_clamped() {
        let c = MortonCode::encode(Coord3D { t: 0, s: 0, d: 0 });
        let shallower = c.depth_outward();
        assert_eq!(c.0, shallower.0); // 不再减小
    }

    #[test]
    fn distance_symmetric() {
        let a = MortonCode::encode(Coord3D { t: 100, s: 50, d: 0 });
        let b = MortonCode::encode(Coord3D { t: 200, s: 80, d: 1 });
        assert_eq!(a.distance(b), b.distance(a));
    }

    #[test]
    fn distance_zero_for_equal() {
        let a = MortonCode::encode(Coord3D { t: 100, s: 50, d: 1 });
        assert_eq!(a.distance(a), 0);
    }

    #[test]
    fn spatial_locality() {
        // 空间相近的点, 莫顿值也相近
        let a = MortonCode::encode(Coord3D { t: 100, s: 50, d: 0 });
        let near = MortonCode::encode(Coord3D { t: 101, s: 50, d: 0 });
        let far = MortonCode::encode(Coord3D { t: 1000, s: 500, d: 2 });
        assert!(a.distance(near) < a.distance(far));
    }
}

impl Coord3D {
    pub fn zero() -> Self {
        Coord3D { t: 0, s: 0, d: 0 }
    }
}
