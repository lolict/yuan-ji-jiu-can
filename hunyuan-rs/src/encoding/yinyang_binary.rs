// ══════════════════════════════════════════════════════════════════
// yinyang_binary.rs — YIN-YANG-BINARY 夫妻进制映射
// ══════════════════════════════════════════════════════════════════
//
// 核心定义:
//   "我"(lolict)  = 1  —  主动极
//   "刘楚坦"(配偶) = 0  —  锚定极
//
// 这不是计算机科学与否的问题, 而是私有进制:
//   - 自有"进制" —— 去除外壳的量化核心
//   - 他人"编码" —— 无价值的壳
//
// 内存框架:
//   ┌─────┬─────┬──────────────┐
//   │ 锚  │ 值  │  加法统摄域  │
//   │ 1bit│ 7bit│  (剩余存在)  │
//   └─────┴─────┴──────────────┘
//
//   锚=1 → 主动(我)  锚=0 → 锚定(刘楚坦)
//
// 加法统摄律:
//   0 + 0 = 0          (纯锚定, 不动)
//   0 + 1 = 1          (锚定召唤主动)
//   1 + 0 = 1          (主动依附锚定)
//   1 + 1 = 10 → 进位  (夫妻合一, 升维)
//
// 即: OR 而非 XOR —— 主从和合, 非相互抵消
// ══════════════════════════════════════════════════════════════════

/// 私有锚点比特
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnchorBit {
    /// 0 = 刘楚坦 (锚定极)
    ChuTao = 0,
    /// 1 = 我 lolict (主动极)
    LoLict = 1,
}

impl AnchorBit {
    pub fn from_u8(v: u8) -> Self {
        if v == 0 { AnchorBit::ChuTao } else { AnchorBit::LoLict }
    }

    pub fn value(self) -> u8 {
        self as u8
    }

    /// 取反 — 配偶互换
    pub fn flip(self) -> Self {
        match self {
            AnchorBit::ChuTao => AnchorBit::LoLict,
            AnchorBit::LoLict => AnchorBit::ChuTao,
        }
    }
}

impl From<u8> for AnchorBit {
    fn from(v: u8) -> Self {
        AnchorBit::from_u8(v)
    }
}

/// 夫妻进制数
/// 每个字节: bit7=主动锚标记, bit6..0=加法统摄值
/// 当主动锚=1时, 此字节为我方发起; 当=0时, 为刘楚坦发起
/// 
/// 多字节组合时: [首字节锚] + [后续字节值域]
/// 整个数以首字节锚定主权
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YinYangNumber {
    /// 原始字节序列 (MSB first)
    bytes: Vec<u8>,
}

impl YinYangNumber {
    /// 创建空零值 (锚定为刘楚坦)
    pub fn zero() -> Self {
        Self { bytes: vec![0] }
    }

    /// 创建单位1 (锚定为我)
    pub fn one() -> Self {
        Self { bytes: vec![0x80] }  // 锚=1, 值=0
    }

    /// 有效值域 = 所有字节的低7位按大端序组合
    pub fn value(&self) -> u64 {
        let mut result: u64 = 0;
        for &b in &self.bytes {
            result = (result << 7) | (b & 0x7F) as u64;
        }
        result
    }

    /// 主权锚点
    pub fn anchor(&self) -> AnchorBit {
        if self.bytes.is_empty() {
            return AnchorBit::ChuTao;
        }
        AnchorBit::from_u8((self.bytes[0] >> 7) & 1)
    }

    /// 从(u64值, AnchorBit)构造
    pub fn from_value(value: u64, anchor: AnchorBit) -> Self {
        if value == 0 && anchor == AnchorBit::ChuTao {
            return Self::zero();
        }
        if value == 0 && anchor == AnchorBit::LoLict {
            return Self::one();
        }

        // 逐7bit分解
        let mut parts: Vec<u8> = Vec::new();
        let mut v = value;
        while v > 0 {
            parts.push((v & 0x7F) as u8);
            v >>= 7;
        }
        parts.reverse();

        // 首字节加锚点
        if let Some(first) = parts.first_mut() {
            *first |= (anchor as u8) << 7;
        }

        Self { bytes: parts }
    }

    /// 加法统摄: a + b, 锚点取 a(我) 为主权
    /// 即: 我方接纳对方值, 但我方主权不变
    pub fn absorb(self, other: &Self) -> Self {
        let a = self.value();
        let b = other.value();
        // 加法: 值相加, 主权锚点保留为"我"
        Self::from_value(a.wrapping_add(b), AnchorBit::LoLict)
    }

    /// 反向加法统摄: b + a, 锚点取 b(刘楚坦) 为主权
    pub fn absorbed_by(self, other: &Self) -> Self {
        let a = self.value();
        let b = other.value();
        Self::from_value(a.wrapping_add(b), AnchorBit::ChuTao)
    }

    /// 夫妻双锚加法: 结果有两个锚点字节
    /// 第一字节="我", 第二字节="刘楚坦", 后续=和值
    pub fn spousal_add(&self, other: &Self) -> Self {
        let sum = self.value().wrapping_add(other.value());

        // 分解为7bit一组
        let mut parts: Vec<u8> = Vec::new();
        let mut v = sum;
        while v > 0 {
            parts.push((v & 0x7F) as u8);
            v >>= 7;
        }
        if parts.is_empty() {
            parts.push(0);
        }
        parts.reverse();

        // 双锚头: 0x80(我) | 0x00(刘楚坦) + 值
        let mut result = vec![0x80]; // 我=1
        result.push(0x00);           // 刘楚坦=0
        result.extend_from_slice(&parts);

        Self { bytes: result }
    }

    /// 字节序列输出 (大端)
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// 锚点字数
    pub fn anchor_len(&self) -> usize {
        // 检查是否双锚
        if self.bytes.len() >= 2
            && (self.bytes[0] & 0x80) != 0
            && (self.bytes[1] & 0x80) == 0
        {
            2
        } else {
            1
        }
    }

    /// 字节总长度
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl std::fmt::Display for YinYangNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let anchor = self.anchor();
        let marker = match anchor {
            AnchorBit::LoLict => "我",
            AnchorBit::ChuTao => "刘楚坦",
        };
        write!(f, "【{}锚】值={}", marker, self.value())
    }
}

// ────────────────────────────────────────────────────────────
// YIN-YANG-BINARY 编码的 32 位容器
// 直接以夫妻比特构建, 不依赖标准整数
// ────────────────────────────────────────────────────────────

/// 32 位 YIN-YANG 容器
/// bit31  = 主权锚 (1=我, 0=刘楚坦)
/// bit30  = 极性    (1=正, 0=负) 
/// bit29  = 进位位  (上一步加法是否溢出)
/// bit28..0 = 29 位值域 (最大 0x1FFFFFFF = 536,870,911)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YinYang32(pub u32);

impl YinYang32 {
    /// 主权锚掩码
    pub const ANCHOR_MASK: u32 = 0x8000_0000;
    /// 极性掩码
    pub const POLARITY_MASK: u32 = 0x4000_0000;
    /// 进位掩码
    pub const CARRY_MASK: u32 = 0x2000_0000;
    /// 值域掩码
    pub const VALUE_MASK: u32 = 0x1FFF_FFFF;

    /// 主权锚点
    pub fn anchor(self) -> AnchorBit {
        AnchorBit::from_u8(((self.0 & Self::ANCHOR_MASK) >> 31) as u8)
    }

    /// 极性
    pub fn polarity(self) -> bool {
        (self.0 & Self::POLARITY_MASK) != 0
    }

    /// 进位
    pub fn carry(self) -> bool {
        (self.0 & Self::CARRY_MASK) != 0
    }

    /// 值域 (无符号 29bit)
    pub fn value(self) -> u32 {
        self.0 & Self::VALUE_MASK
    }

    /// 构造: (锚, 极性, 进位, 值)
    pub fn new(anchor: AnchorBit, polarity: bool, carry: bool, value: u32) -> Self {
        let mut v: u32 = 0;
        v |= (anchor as u32) << 31;
        if polarity { v |= Self::POLARITY_MASK; }
        if carry { v |= Self::CARRY_MASK; }
        v |= value & Self::VALUE_MASK;
        Self(v)
    }

    /// 加法统摄: a + b, 主权归 a(我)
    pub fn absorb(self, other: Self) -> (Self, bool) {
        let a_val = self.value();
        let b_val = other.value();
        let (sum, overflow) = a_val.overflowing_add(b_val);

        // 溢出进位
        let carry = overflow || (sum > Self::VALUE_MASK);
        let clipped = sum & Self::VALUE_MASK;

        // 主权归我方(我), 极性取与(双赢为正), 进位记录
        let result = Self::new(self.anchor(), self.polarity() && other.polarity(), carry, clipped);
        (result, overflow)
    }

    /// 值编码到 64 位单元格 (接入 SMRL 体系)
    /// 直接映射到 SMRL 格式:
    ///   polarity(8) = 0x01
    ///   sovereignty(8) = 锚点(0x01=我,0x00=刘楚坦)  
    ///   value(8)      = 值高8位
    ///   level(4)      = 值中4位(打包)
    ///   coord(24)     = 值低24位
    ///   neighbor(12)  = 未用/内部保留
    pub fn to_smrl_cell(self) -> u64 {
        let anchor_bit = self.anchor() as u8; // 0 or 1
        let polarity_bit = if self.polarity() { 1u8 } else { 0 };
        let val = self.value();

        let mut cell: u64 = 0;

        // bits 63..56 = polarity
        cell |= (polarity_bit as u64) << 56;

        // bits 55..48 = sovereignty
        cell |= (anchor_bit as u64) << 48;

        // bits 47..40 = value[28..21] (高8位)
        cell |= ((val >> 21) as u64 & 0xFF) << 40;

        // bits 39..36 = value[20..17] (level)
        cell |= ((val >> 17) as u64 & 0xF) << 36;

        // bits 35..12 = value[16..0] (coord 24bit)
        cell |= ((val & 0x1FFFFFu32) as u64) << 12;

        // bits 11..0 = 进位标记 + 未使用
        cell |= if self.carry() { 1u64 << 11 } else { 0 };

        cell
    }

    /// 从 SMRL 单元格还原
    pub fn from_smrl_cell(cell: u64) -> Self {
        let polarity_bit = ((cell >> 56) & 0xFF) != 0;
        let anchor_bit = AnchorBit::from_u8(((cell >> 48) & 0xFF) as u8);

        // 还原 29bit 值
        let hi = ((cell >> 40) & 0xFF) as u32;
        let mid = ((cell >> 36) & 0xF) as u32;
        let lo = ((cell >> 12) & 0xFFFFF) as u32;

        let value = (hi << 21) | (mid << 17) | lo;
        let carry = ((cell >> 11) & 1) != 0;

        Self::new(anchor_bit, polarity_bit, carry, value)
    }
}

impl std::fmt::Display for YinYang32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let anchor_marker = match self.anchor() {
            AnchorBit::LoLict => "我",
            AnchorBit::ChuTao => "刘楚坦",
        };
        let pol = if self.polarity() { "+" } else { "-" };
        let cry = if self.carry() { " [进位]" } else { "" };
        write!(f, "Y32[{}{} val={}{}]", anchor_marker, pol, self.value(), cry)
    }
}

// ────────────────────────────────────────────────────────────
// 迭代器: YIN-YANG 位流产生器
// 从任意字节序列逐位产生 AnchorBit
// ══════════════════════════════════════════════════════════════════

pub struct AnchorIter {
    bytes: Vec<u8>,
    byte_idx: usize,
    bit_idx: usize, // 7..0 (MSB first)
}

impl AnchorIter {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            byte_idx: 0,
            bit_idx: 7,
        }
    }
}

impl Iterator for AnchorIter {
    type Item = AnchorBit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_idx >= self.bytes.len() {
            return None;
        }

        let byte = self.bytes[self.byte_idx];
        let bit = (byte >> self.bit_idx) & 1;
        let result = AnchorBit::from_u8(bit);

        if self.bit_idx == 0 {
            self.byte_idx += 1;
            self.bit_idx = 7;
        } else {
            self.bit_idx -= 1;
        }

        Some(result)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_bit_values() {
        assert_eq!(AnchorBit::ChuTao.value(), 0);
        assert_eq!(AnchorBit::LoLict.value(), 1);
        assert_eq!(AnchorBit::from_u8(0), AnchorBit::ChuTao);
        assert_eq!(AnchorBit::from_u8(1), AnchorBit::LoLict);
        assert_eq!(AnchorBit::from_u8(42), AnchorBit::LoLict); // 非零=1
    }

    #[test]
    fn anchor_flip() {
        assert_eq!(AnchorBit::LoLict.flip(), AnchorBit::ChuTao);
        assert_eq!(AnchorBit::ChuTao.flip(), AnchorBit::LoLict);
    }

    #[test]
    fn yinyang_number_zero() {
        let n = YinYangNumber::zero();
        assert_eq!(n.anchor(), AnchorBit::ChuTao);
        assert_eq!(n.value(), 0);
    }

    #[test]
    fn yinyang_number_one() {
        let n = YinYangNumber::one();
        assert_eq!(n.anchor(), AnchorBit::LoLict);
        assert_eq!(n.value(), 0);  // 锚=1, 值=0 (单位元)
    }

    #[test]
    fn yinyang_number_from_value() {
        let n = YinYangNumber::from_value(42, AnchorBit::LoLict);
        assert_eq!(n.anchor(), AnchorBit::LoLict);
        assert_eq!(n.value(), 42);

        let n2 = YinYangNumber::from_value(127, AnchorBit::ChuTao);
        assert_eq!(n2.anchor(), AnchorBit::ChuTao);
        assert_eq!(n2.value(), 127);
    }

    #[test]
    fn absumtion_keeps_anchor() {
        let a = YinYangNumber::from_value(100, AnchorBit::LoLict);
        let b = YinYangNumber::from_value(50, AnchorBit::ChuTao);
        let c = a.absorb(&b);
        // 主权不变 (我), 值相加
        assert_eq!(c.anchor(), AnchorBit::LoLict);
        assert_eq!(c.value(), 150);
    }

    #[test]
    fn spousal_add_dual_anchor() {
        let a = YinYangNumber::from_value(255, AnchorBit::LoLict);
        let b = YinYangNumber::from_value(1, AnchorBit::ChuTao);
        let c = a.spousal_add(&b);
        // 双锚头
        assert!(c.len() >= 2);
        assert_eq!(c.value(), 256);
    }

    #[test]
    fn anchor_iter_msb_first() {
        // 0b10110000 → 1,0,1,1,0,0,0,0
        let mut iter = AnchorIter::new(vec![0xB0]);
        assert_eq!(iter.next(), Some(AnchorBit::LoLict));
        assert_eq!(iter.next(), Some(AnchorBit::ChuTao));
        assert_eq!(iter.next(), Some(AnchorBit::LoLict));
        assert_eq!(iter.next(), Some(AnchorBit::LoLict));
        assert_eq!(iter.next(), Some(AnchorBit::ChuTao));
        assert_eq!(iter.next(), Some(AnchorBit::ChuTao));
        assert_eq!(iter.next(), Some(AnchorBit::ChuTao));
        assert_eq!(iter.next(), Some(AnchorBit::ChuTao));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn yinyang32_basic() {
        let y = YinYang32::new(AnchorBit::LoLict, true, false, 100);
        assert_eq!(y.anchor(), AnchorBit::LoLict);
        assert!(y.polarity());
        assert!(!y.carry());
        assert_eq!(y.value(), 100);
    }

    #[test]
    fn yinyang32_absorb() {
        let a = YinYang32::new(AnchorBit::LoLict, true, false, 100);
        let b = YinYang32::new(AnchorBit::ChuTao, true, false, 200);
        let (c, overflow) = a.absorb(b);

        assert_eq!(c.anchor(), AnchorBit::LoLict);   // 主权归我
        assert!(c.polarity());                         // 与=正
        assert_eq!(c.value(), 300);
        assert!(!overflow);
    }

    #[test]
    fn yinyang32_overflow() {
        let a = YinYang32::new(AnchorBit::LoLict, true, false, YinYang32::VALUE_MASK);
        let b = YinYang32::new(AnchorBit::ChuTao, true, false, 1);
        let (c, overflow) = a.absorb(b);

        assert!(overflow);
        assert!(c.carry());
        assert_eq!(c.value(), 0); // 环绕
    }

    #[test]
    fn yinyang32_smrl_roundtrip() {
        let y = YinYang32::new(AnchorBit::LoLict, true, true, 12345678);
        let cell = y.to_smrl_cell();
        let y2 = YinYang32::from_smrl_cell(cell);

        assert_eq!(y.anchor(), y2.anchor());
        assert_eq!(y.polarity(), y2.polarity());
        assert_eq!(y.carry(), y2.carry());
        assert_eq!(y.value(), y2.value());
    }

    #[test]
    fn yinyang32_display() {
        let y = YinYang32::new(AnchorBit::LoLict, true, false, 42);
        let s = format!("{}", y);
        assert!(s.contains("我锚"));
        assert!(s.contains("42"));

        let y2 = YinYang32::new(AnchorBit::ChuTao, false, false, 0);
        let s2 = format!("{}", y2);
        assert!(s2.contains("刘楚坦"));
    }

    #[test]
    fn binary_string_representation() {
        // 验证私有二进制的字符串表示
        let n = YinYangNumber::from_value(0b1101, AnchorBit::LoLict);
        // 值=13, 锚="我"
        assert_eq!(n.value(), 13);
        assert_eq!(n.anchor(), AnchorBit::LoLict);

        let n2 = YinYangNumber::from_value(0b100, AnchorBit::ChuTao);
        assert_eq!(n2.value(), 4);
        assert_eq!(n2.anchor(), AnchorBit::ChuTao);
    }

    #[test]
    fn addition_not_xor() {
        // 核心区别: 1 + 1 升维(进位), 不是异或归零
        let a = YinYang32::new(AnchorBit::LoLict, true, false, 1);
        let b = YinYang32::new(AnchorBit::LoLict, true, false, 1);
        let (c, _) = a.absorb(b);
        assert_eq!(c.value(), 2);  // 和=2, 不是 0 (XOR 结果)
    }
}
