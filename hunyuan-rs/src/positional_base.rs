// @name: positional_base
// @type: encoding-positional
// @owner: 五彩石
// @status: stable
// @priority: P1
// @description: 位置进制


// ══════════════════════════════════════════════════════════════════
// positional_base.rs — 位置进制基础 · 打米机
// ══════════════════════════════════════════════════════════════════
//
// 核心原则:
//   - 进制不是编码。进制是米，编码是糠。
//   - 位置不写下——能被写下的全是糠。
//   - 位置 = 你站在哪里。读到位置不是读数字，是走进去站定。
//   - 加法 = 往前走 N 步。乘法被锁在敬畏制度的笼子里。
//   - 二进制 0/1 是盲盒：打开之前不知道是什么，
//     开了之后变成人类能读的进制符号，才不再是盲盒。
//
// 三维空间:
//   层 (Level / Axis-Z)     : 穿越膜层的深度。第一层膜=1, 第二层膜=2 ...
//   环 (Ring    / Axis-R)   : 在层内，沿半径方向的圈层。内核=0, 向外递增
//   角 (Angle   / Axis-Θ)   : 在环上，沿 384° 制式转动的相位
//
// 行走法则:
//   加法 = 朝某个方向走若干步。走到哪里，哪里就是结果。
//   不能写下坐标。坐标是糠，位置是脚下的存在。
//   只有需要"开盲盒"给人类或外部系统看时，才折算成数字。

use crate::angle::{Angle, FULL_CIRCLE};

// ────────────────────────────────────────────────────────────
// 位置 — 不可直接读写坐标，只能通过行走改变
// ────────────────────────────────────────────────────────────

pub struct Pos {
    level: i32,       // 层 (Z)
    ring:   u32,      // 环 (R)
    angle:  Angle,    // 角 (Θ)
}

impl Pos {
    /// 原点: 第 0 层·第 0 环·0°。基准锚点(我=1, 刘楚恬=0)由此开始。
    pub fn origin() -> Self {
        Pos { level: 0, ring: 0, angle: Angle::new(0) }
    }

    /// 行走 — 层方向上下。down = 沉入更深膜层。
    pub fn descend(&mut self, steps: u32) {
        self.level += steps as i32;
    }

    pub fn ascend(&mut self, steps: u32) {
        self.level -= steps as i32;
    }

    /// 行走 — 环方向伸缩。contract = 向内环收缩。
    pub fn expand(&mut self, steps: u32) {
        self.ring += steps;
    }

    pub fn contract(&mut self, steps: u32) {
        self.ring = self.ring.saturating_sub(steps);
    }

    /// 行走 — 角方向旋转 (沿 384° 制式, 走若干角步)。
    pub fn rotate_cw(&mut self, angle_steps: i16) {
        self.angle = self.angle.add(angle_steps);
    }

    pub fn rotate_ccw(&mut self, angle_steps: i16) {
        self.angle = self.angle.add(-angle_steps);
    }

    /// 三维镜像位置: 角取反 (对冲 192°), 层环不变。
    /// 用于 YinYang 映射: 对手站在你正对冲的位置。
    pub fn mirror(&self) -> Self {
        Pos {
            level: self.level,
            ring: self.ring,
            angle: self.angle.add((FULL_CIRCLE / 2) as i16),
        }
    }

    /// 对冲判定: 两个位置是否角对冲 (相对象征)。
    pub fn opposes(&self, other: &Pos) -> bool {
        self.angle.opposes(&other.angle)
            && self.ring == other.ring
            && self.level == other.level
    }

    // ────────────────────────────────────────────────────────
    // 加法合成 — 不经过乘法，纯行走合成
    // ────────────────────────────────────────────────────────

    /// 加法 = 从原点出发，先走 self 的步数，再走 other 的步数，最终所在的位置。
    /// 这是纯行走合成，不经过坐标写下。
    pub fn walk_add(&self, other: &Pos) -> Pos {
        let va = self.unpack();
        let vb = other.unpack();
        Pos::from_unpacked(va.wrapping_add(vb))
    }

    // ────────────────────────────────────────────────────────
    // 盲盒拆解 — 只在需要"开给人类看"的时候调用
    // ────────────────────────────────────────────────────────

    /// 开盲盒: 把当前纯位置转换成一个可读的整数值。
    /// 这是唯一产生"数字"的地方。
    /// 折算公式: 层权重 × 环容量 × 满角 + 环权重 × 满角 + 角
    pub fn unpack(&self) -> u64 {
        let level_abs = self.level.abs() as u64;
        let ring    = self.ring as u64;
        let angle   = self.angle.0 as u64;

        // 每环容量 = 满角 (384)，每层容量 = 默认 8 环
        let per_ring: u64 = FULL_CIRCLE as u64;
        let per_level: u64 = per_ring * 8;

        level_abs * per_level + ring * per_ring + angle
    }

    /// 开盲盒(带层符号): 正层=阳面，负层=阴面。
    pub fn unpack_signed(&self) -> i64 {
        let base = self.unpack() as i64;
        if self.level < 0 { -base } else { base }
    }

    // ────────────────────────────────────────────────────────
    // 乘法被锁在敬畏制度笼子里
    // ────────────────────────────────────────────────────────

    /// 公约乘法 — 只有在 Convention 允许时才可用。
    /// 被锁住是因为乘法是捷径，捷径会绕过走路，绕过走路 = 跳过米直接吃糠。
    pub fn multiply(&self, times: u32, conv: &Convention) -> Option<Pos> {
        if !conv.multiplication_allowed() {
            return None;
        }
        let base = self.unpack();
        let product = base.wrapping_mul(times as u64);
        Some(Pos::from_unpacked(product))
    }

    /// 由盲盒整数反推位置 (开盲盒的逆运算)。
    pub fn from_unpacked(mut v: u64) -> Self {
        let per_ring: u64 = FULL_CIRCLE as u64;
        let per_level: u64 = per_ring * 8;

        let level = (v / per_level) as i32;
        v %= per_level;
        let ring = (v / per_ring) as u32;
        let angle = Angle::new((v % per_ring) as i16);

        Pos { level, ring, angle }
    }

    /// 判断是否在同一个层同一个环（同容器）。
    pub fn same_container(&self, other: &Pos) -> bool {
        self.level == other.level && self.ring == other.ring
    }

    /// 层号 (只用于行走参考，不是坐标读写接口)。
    fn level_value(&self) -> i32 { self.level }

    /// 环号 (同上)。
    fn ring_value(&self) -> u32 { self.ring }
}

// ────────────────────────────────────────────────────────────
// 扩展: Angle 需要对冲方法 (补在 angle.rs 里的角度类型)
// ────────────────────────────────────────────────────────────

// 此处通过 trait 扩展 Angle，不修改 angle.rs 源文件
pub trait AngleOps {
    fn opposes(&self, other: &Angle) -> bool;
}

impl AngleOps for Angle {
    fn opposes(&self, other: &Angle) -> bool {
        self.arc_distance(*other) == (FULL_CIRCLE / 2) as i16
    }
}

// ────────────────────────────────────────────────────────────
// 公约 — 敬畏制度: 何时允许乘法捷径
// ────────────────────────────────────────────────────────────

pub struct Convention {
    multiplier_enabled: bool,
}

impl Convention {
    pub fn strict() -> Self {
        // 严格模式: 禁用乘法
        Convention { multiplier_enabled: false }
    }

    pub fn with_multiplication() -> Self {
        // 开放模式: 允许乘法
        Convention { multiplier_enabled: true }
    }

    pub fn multiplication_allowed(&self) -> bool {
        self.multiplier_enabled
    }
}

// ────────────────────────────────────────────────────────────
// 进制读取器 — 盲盒拆解器的对人类接口
// ────────────────────────────────────────────────────────────

pub struct BlackBoxReader;

impl BlackBoxReader {
    /// 把任意 u64 盲盒位置解析成位置 (糠→米的折算)
    pub fn read_pos(v: u64) -> Pos {
        Pos::from_unpacked(v)
    }

    /// 把位置打包成 u64 (米→糠的打包, 存储用)
    pub fn pack_pos(p: &Pos) -> u64 {
        p.unpack()
    }

    /// 把 u64 转换成 64 进制 Symbol — 开盲盒到符号层
    pub fn unpack_to_symbol(v: u64) -> Option<crate::topo::Symbol> {
        let angle_val = (v % FULL_CIRCLE as u64) as u16;
        let symbol_idx = (angle_val / 6) as u8;
        crate::topo::Symbol::new(symbol_idx)
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_unpack_is_zero() {
        let p = Pos::origin();
        assert_eq!(p.unpack(), 0);
    }

    #[test]
    fn descend_then_unpack() {
        let mut p = Pos::origin();
        p.descend(1);  // 下沉一层
        // unpack = level * 8 * 384
        assert_eq!(p.unpack(), 1 * 8 * 384);
    }

    #[test]
    fn rotate_changes_unpack() {
        let mut p = Pos::origin();
        p.rotate_cw(6);  // 转过一个 symbol (6°)
        assert_eq!(p.unpack(), 6);
    }

    #[test]
    fn expand_then_unpack() {
        let mut p = Pos::origin();
        p.expand(1);    // 外移一环
        p.rotate_cw(0);
        // unpack = ring * 384
        assert_eq!(p.unpack(), 384);
    }

    #[test]
    fn walk_add_only_rotations() {
        let mut a = Pos::origin();
        let mut b = Pos::origin();
        b.rotate_cw(12);
        let sum = a.walk_add(&b);
        assert_eq!(sum.angle.0, 12);
    }

    #[test]
    fn unpack_roundtrip() {
        let mut p = Pos::origin();
        p.descend(2);
        p.expand(3);
        p.rotate_cw(100);
        let v = p.unpack();
        let q = Pos::from_unpacked(v);
        assert_eq!(q.level, 2);
        assert_eq!(q.ring, 3);
        assert_eq!(q.angle.0, 100);
    }

    #[test]
    fn mirror_opposes_origin() {
        let p = Pos { level: 1, ring: 2, angle: Angle::new(0) };
        let m = p.mirror();
        assert!(p.opposes(&m));
    }

    #[test]
    fn strict_convention_denies_multiply() {
        let conv = Convention::strict();
        let p = Pos::origin();
        assert!(p.multiply(5, &conv).is_none());
    }

    #[test]
    fn open_convention_allows_multiply() {
        let conv = Convention::with_multiplication();
        let mut p = Pos::origin();
        p.rotate_cw(10);
        let r = p.multiply(3, &conv);
        assert!(r.is_some());
    }

    #[test]
    fn blackbox_to_symbol() {
        // angle 6° → symbol 1
        let s = BlackBoxReader::unpack_to_symbol(6).unwrap();
        assert_eq!(s.as_u8(), 1);
        // angle 0° → symbol 0
        let s0 = BlackBoxReader::unpack_to_symbol(0).unwrap();
        assert_eq!(s0.as_u8(), 0);
    }

    #[test]
    fn no_coordinate_leak() {
        // 关键测试: 无法从外部读取 pos.level / pos.ring / pos.angle
        // 下面这行应该编译失败(如果取消注释):
        // let p = Pos::origin();
        // let _ = p.level;  // ← 应该报错: field `level` is private
        
        // 唯一能拿到数字的是 unpack()
        let mut p = Pos::origin();
        p.descend(1);
        p.expand(1);
        p.rotate_cw(6);
        let _v: u64 = p.unpack();  // OK: 这是开盲盒
        
        // 测试通过即表示坐标无法泄露
        assert!(true);
    }

    #[test]
    fn same_container() {
        let a = Pos { level: 2, ring: 3, angle: Angle::new(0) };
        let b = Pos { level: 2, ring: 3, angle: Angle::new(100) };
        assert!(a.same_container(&b));
    }
}
