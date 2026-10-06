// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.83 进制寻址引擎                                              ║
// ║  YinYang 二进制 · 六十四位编码 · 固定点 · 公约数编码             ║
// ║  去外壳量化核心 · 三维进制容器                                   ║
// ║                                                                  ║
// ║  核心: 自有进制, 量化核心即容器, 地址即量值                       ║
// ║  · YinYang: 0/1 非数值, 是阴阳                                  ║
// ║  · 六十四位: 坐标(20)+极性(4)+主权(20)+价值(20)                   ║
// ║  · 固定点: 整数缩放代替浮点                                      ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};

/// YinYang 比特 — 不是 0/1 数值, 是阴阳
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum YinYang {
    /// 阴 = 0 = 静态 = 被动 = 接收
    Yin,
    /// 阳 = 1 = 动态 = 主动 = 发射
    Yang,
}

impl YinYang {
    /// 从 u8 转换 (0=Yin, 非0=Yang)
    pub fn from_bits(b: u8) -> Self {
        if b == 0 { YinYang::Yin } else { YinYang::Yang }
    }

    /// 转为 u8
    pub fn to_bit(self) -> u8 {
        match self {
            YinYang::Yin => 0,
            YinYang::Yang => 1,
        }
    }

    /// 翻转
    pub fn flip(self) -> Self {
        match self {
            YinYang::Yin => YinYang::Yang,
            YinYang::Yang => YinYang::Yin,
        }
    }

    /// 是否相反
    pub fn is_opposite(self, other: Self) -> bool {
        self != other
    }
}

impl Default for YinYang {
    fn default() -> Self {
        YinYang::Yin
    }
}

/// YinYang 二进制 — 阴阳位串
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct YinYangBinary {
    /// 位串 (LSB 在前)
    pub bits: Vec<YinYang>,
}

impl YinYangBinary {
    pub fn new(bits: Vec<YinYang>) -> Self {
        YinYangBinary { bits }
    }

    /// 从 u64 构造
    pub fn from_u64(mut val: u64, width: usize) -> Self {
        let mut bits = Vec::with_capacity(width);
        for _ in 0..width {
            bits.push(YinYang::from_bits((val & 1) as u8));
            val >>= 1;
        }
        YinYangBinary { bits }
    }

    /// 转为 u64
    pub fn to_u64(&self) -> u64 {
        let mut val = 0u64;
        for (i, bit) in self.bits.iter().enumerate() {
            if i >= 64 { break; }
            if bit == &YinYang::Yang {
                val |= 1 << i;
            }
        }
        val
    }

    /// 长度
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// 按位阴阳相加 (不进位, 用于 YinYang AND)
    pub fn yinyang_or(&self, other: &Self) -> Self {
        let max_len = self.bits.len().max(other.bits.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.bits.get(i).unwrap_or(&YinYang::Yin);
            let b = other.bits.get(i).unwrap_or(&YinYang::Yin);
            // OR: 任一为 Yang 则为 Yang
            result.push(if *a == YinYang::Yang || *b == YinYang::Yang {
                YinYang::Yang
            } else {
                YinYang::Yin
            });
        }
        YinYangBinary { bits: result }
    }

    /// YinYang XOR — 位异或 (阴+阴=阴, 阳+阳=阴, 阴+阳=阳)
    pub fn yinyang_xor(&self, other: &Self) -> Self {
        let max_len = self.bits.len().max(other.bits.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.bits.get(i).unwrap_or(&YinYang::Yin);
            let b = other.bits.get(i).unwrap_or(&YinYang::Yin);
            result.push(if a == b { YinYang::Yin } else { YinYang::Yang });
        }
        YinYangBinary { bits: result }
    }

    /// 翻转所有位
    pub fn flip_all(&self) -> Self {
        let bits: Vec<YinYang> = self.bits.iter().map(|b| b.flip()).collect();
        YinYangBinary { bits }
    }

    /// 阳位计数
    pub fn yang_count(&self) -> usize {
        self.bits.iter().filter(|b| **b == YinYang::Yang).count()
    }
}

impl Default for YinYangBinary {
    fn default() -> Self {
        YinYangBinary { bits: vec![YinYang::Yin; 8] }
    }
}

/// 六十四位编码: 坐标(20)+极性(4)+主权(20)+价值(20)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Code64(pub u64);

impl Code64 {
    /// 坐标位宽
    const COORD_BITS: u32 = 20;
    /// 极性位宽
    const POLARITY_BITS: u32 = 4;
    /// 主权位宽
    const SOVEREIGNTY_BITS: u32 = 20;
    /// 价值位宽
    const VALUE_BITS: u32 = 20;

    /// 从字段构造
    pub fn new(coord: u32, polarity: u8, sovereignty: u32, value: u32) -> Self {
        let c = (coord as u64) & 0xFFFFF;        // 20 bits
        let p = (polarity as u64) & 0xF;         // 4 bits
        let s = (sovereignty as u64) & 0xFFFFF;   // 20 bits
        let v = (value as u64) & 0xFFFFF;         // 20 bits
        Code64(c | (p << 20) | (s << 24) | (v << 44))
    }

    /// 提取坐标 (低 20 位)
    pub fn coord(&self) -> u32 {
        (self.0 & 0xFFFFF) as u32
    }

    /// 提取极性 (bit 20..24)
    pub fn polarity(&self) -> u8 {
        ((self.0 >> 20) & 0xF) as u8
    }

    /// 提取主权 (bit 24..44)
    pub fn sovereignty(&self) -> u32 {
        ((self.0 >> 24) & 0xFFFFF) as u32
    }

    /// 提取价值 (bit 44..64)
    pub fn value(&self) -> u32 {
        ((self.0 >> 44) & 0xFFFFF) as u32
    }

    /// 原始 u64
    pub fn raw(&self) -> u64 {
        self.0
    }

    /// 汉明重量 (阳位数 = 激活程度)
    pub fn activity(&self) -> u32 {
        self.0.count_ones()
    }
}

impl Default for Code64 {
    fn default() -> Self {
        Code64(0)
    }
}

/// 固定点 — 整数缩放替代浮点
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedPoint {
    /// 内部值 = real * 2^scale
    pub raw: i64,
    /// 缩放指数 (位数)
    pub scale: u32,
}

impl FixedPoint {
    pub fn new(raw: i64, scale: u32) -> Self {
        FixedPoint { raw, scale }
    }

    /// 从整数构造
    pub fn from_int(val: i64, scale: u32) -> Self {
        FixedPoint { raw: val << scale, scale }
    }

    /// 转为 f64
    pub fn to_f64(&self) -> f64 {
        self.raw as f64 / (1u64 << self.scale) as f64
    }

    /// 加法 (scale 必须相同)
    pub fn add(self, other: Self) -> Option<Self> {
        if self.scale != other.scale { return None; }
        Some(FixedPoint { raw: self.raw + other.raw, scale: self.scale })
    }

    /// 乘法
    pub fn mul(self, other: Self) -> Option<Self> {
        if self.scale != other.scale { return None; }
        let product = (self.raw as i128 * other.raw as i128) >> self.scale;
        Some(FixedPoint { raw: product as i64, scale: self.scale })
    }

    /// 是否为零
    pub fn is_zero(&self) -> bool {
        self.raw == 0
    }
}

impl Default for FixedPoint {
    fn default() -> Self {
        FixedPoint { raw: 0, scale: 8 }
    }
}

/// 公约数地址 — 基于 GCD 的容器寻址
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GcdAddress(pub u64);

impl GcdAddress {
    /// 构造: address = GCD(dimensions)
    pub fn from_dims(dims: &[u64]) -> Self {
        if dims.is_empty() {
            return GcdAddress(1);
        }
        let mut result = dims[0];
        for &d in &dims[1..] {
            result = gcd64(result, d);
            if result == 1 { break; }
        }
        GcdAddress(if result == 0 { 1 } else { result })
    }

    /// 是否与另一地址同源 (有共同因子)
    pub fn is_cousin(&self, other: &Self) -> bool {
        gcd64(self.0, other.0) > 1
    }

    /// 寻址强度 = 对数维度
    pub fn strength(&self) -> u32 {
        if self.0 == 0 { 0 } else { 63 - self.0.leading_zeros() }
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// 量化核心 — 去除编码外壳后的纯量
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuantizeCore(pub u64);

impl QuantizeCore {
    /// 从容器编码去外壳 (去除极性与命名空间)
    pub fn strip(kind: ContainerKind, entity: u64) -> Self {
        // 量化核心 = 实体部分的量化表示 (去掉 kind 和命名空间)
        QuantizeCore(entity)
    }

    /// 两个量化核心的共振 = GCD
    pub fn resonance(&self, other: &Self) -> u64 {
        gcd64(self.0, other.0)
    }

    /// 量化即地址
    pub fn as_address(&self) -> u64 {
        self.0
    }
}

/// 进制寻址引擎
pub struct RadixAddressingEngine {
    /// YinYang 键值存储
    pub yinyang_store: HashMap<YinYangBinary, ContainerRef>,
    /// 公约数地址表
    pub gcd_addrs: HashMap<u64, Vec<ContainerRef>>,
    /// 固定点寄存器
    pub fixed_regs: HashMap<u64, FixedPoint>,
    /// 量化核心表
    pub quantize_cores: HashMap<u64, QuantizeCore>,
}

impl RadixAddressingEngine {
    pub fn new() -> Self {
        RadixAddressingEngine {
            yinyang_store: HashMap::new(),
            gcd_addrs: HashMap::new(),
            fixed_regs: HashMap::new(),
            quantize_cores: HashMap::new(),
        }
    }

    /// YinYang 寻址
    pub fn store_yinyang(&mut self, key: YinYangBinary, val: ContainerRef) {
        self.yinyang_store.insert(key, val);
    }

    /// YinYang 查询
    pub fn lookup_yinyang(&self, key: &YinYangBinary) -> Option<&ContainerRef> {
        self.yinyang_store.get(key)
    }

    /// 公约数注册
    pub fn register_gcd(&mut self, gcd_val: u64, container: ContainerRef) {
        self.gcd_addrs.entry(gcd_val).or_default().push(container);
    }

    /// 查询同族容器 (有共同因子)
    pub fn find_kin(&self, addr: GcdAddress) -> Vec<(&u64, &Vec<ContainerRef>)> {
        self.gcd_addrs
            .iter()
            .filter(|(k, _)| addr.is_cousin(&GcdAddress(**k)))
            .collect()
    }

    /// 固定点设置
    pub fn set_fixed(&mut self, id: u64, val: FixedPoint) {
        self.fixed_regs.insert(id, val);
    }

    /// 固定点获取
    pub fn get_fixed(&self, id: u64) -> Option<&FixedPoint> {
        self.fixed_regs.get(&id)
    }

    /// 量化核心设置
    pub fn set_core(&mut self, addr: u64, core: QuantizeCore) {
        self.quantize_cores.insert(addr, core);
    }

    /// 量化共振查询
    pub fn resonance_with(&self, addr: u64, other: u64) -> Option<u64> {
        let core_a = self.quantize_cores.get(&addr)?;
        let core_b = self.quantize_cores.get(&other)?;
        Some(core_a.resonance(core_b))
    }
}

impl Default for RadixAddressingEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn yinyang_from_bits() {
        assert_eq!(YinYang::from_bits(0), YinYang::Yin);
        assert_eq!(YinYang::from_bits(1), YinYang::Yang);
        assert_eq!(YinYang::from_bits(42), YinYang::Yang);
    }

    #[test]
    fn yinyang_to_bit() {
        assert_eq!(YinYang::Yin.to_bit(), 0);
        assert_eq!(YinYang::Yang.to_bit(), 1);
    }

    #[test]
    fn yinyang_flip() {
        assert_eq!(YinYang::Yin.flip(), YinYang::Yang);
        assert_eq!(YinYang::Yang.flip(), YinYang::Yin);
    }

    #[test]
    fn yinyang_opposite() {
        assert!(YinYang::Yin.is_opposite(YinYang::Yang));
        assert!(!YinYang::Yin.is_opposite(YinYang::Yin));
    }

    #[test]
    fn yinyang_binary_to_from_u64() {
        let val: u64 = 0b1010;
        let bin = YinYangBinary::from_u64(val, 8);
        assert_eq!(bin.to_u64(), val);
    }

    #[test]
    fn yinyang_binary_or() {
        let a = YinYangBinary::from_u64(0b0011, 4);
        let b = YinYangBinary::from_u64(0b0101, 4);
        let result = a.yinyang_or(&b);
        assert_eq!(result.to_u64(), 0b0111);
    }

    #[test]
    fn yinyang_binary_xor() {
        let a = YinYangBinary::from_u64(0b0011, 4);
        let b = YinYangBinary::from_u64(0b0101, 4);
        let result = a.yinyang_xor(&b);
        assert_eq!(result.to_u64(), 0b0110);
    }

    #[test]
    fn yinyang_flip_all() {
        let bin = YinYangBinary::from_u64(0b1010, 4);
        let flipped = bin.flip_all();
        assert_eq!(flipped.to_u64(), 0b0101);
    }

    #[test]
    fn yinyang_yang_count() {
        let bin = YinYangBinary::from_u64(0b1011, 4);
        assert_eq!(bin.yang_count(), 3);
    }

    #[test]
    fn code64_fields() {
        let code = Code64::new(42, 3, 100, 200);
        assert_eq!(code.coord(), 42);
        assert_eq!(code.polarity(), 3);
        assert_eq!(code.sovereignty(), 100);
        assert_eq!(code.value(), 200);
    }

    #[test]
    fn code64_activity() {
        let code = Code64(0xFFFF_FFFF_FFFF_FFFF);
        assert!(code.activity() > 0);
    }

    #[test]
    fn fixed_point_add() {
        let a = FixedPoint::from_int(10, 8);
        let b = FixedPoint::from_int(20, 8);
        let sum = a.add(b).unwrap();
        assert!((sum.to_f64() - 30.0).abs() < 0.01);
    }

    #[test]
    fn fixed_point_mul() {
        let a = FixedPoint::from_int(4, 8);
        let b = FixedPoint::from_int(3, 8);
        let prod = a.mul(b).unwrap();
        assert!((prod.to_f64() - 12.0).abs() < 0.1);
    }

    #[test]
    fn fixed_point_scale_mismatch() {
        let a = FixedPoint::new(10, 8);
        let b = FixedPoint::new(20, 16);
        assert!(a.add(b).is_none());
    }

    #[test]
    fn gcd_address_from_dims() {
        let addr = GcdAddress::from_dims(&[12, 18, 24]);
        assert_eq!(addr.raw(), 6);
    }

    #[test]
    fn gcd_address_cousin() {
        let a = GcdAddress(6);
        let b = GcdAddress(12);
        assert!(a.is_cousin(&b));
    }

    #[test]
    fn gcd_address_not_cousin() {
        let a = GcdAddress(7);
        let b = GcdAddress(11);
        assert!(!a.is_cousin(&b));
    }

    #[test]
    fn gcd_address_strength() {
        let addr = GcdAddress(256); // 2^8
        assert!(addr.strength() >= 8);
    }

    #[test]
    fn quantize_core_resonance() {
        let a = QuantizeCore(12);
        let b = QuantizeCore(18);
        assert_eq!(a.resonance(&b), 6);
    }

    #[test]
    fn engine_yinyang_store() {
        let mut engine = RadixAddressingEngine::new();
        let key = YinYangBinary::from_u64(0xAB, 8);
        engine.store_yinyang(key.clone(), mk_ref(100));

        let found = engine.lookup_yinyang(&key);
        assert!(found.is_some());
        assert_eq!(found.unwrap().entity(), 100);
    }

    #[test]
    fn engine_gcd_register_and_find() {
        let mut engine = RadixAddressingEngine::new();
        engine.register_gcd(6, mk_ref(100));
        engine.register_gcd(12, mk_ref(200));

        let addr = GcdAddress(6);
        let kin = engine.find_kin(addr);
        assert!(!kin.is_empty());
    }

    #[test]
    fn engine_fixed_regs() {
        let mut engine = RadixAddressingEngine::new();
        engine.set_fixed(1, FixedPoint::from_int(42, 8));

        let val = engine.get_fixed(1).unwrap();
        assert!((val.to_f64() - 42.0).abs() < 0.01);
    }

    #[test]
    fn engine_resonance_with() {
        let mut engine = RadixAddressingEngine::new();
        engine.set_core(1, QuantizeCore(12));
        engine.set_core(2, QuantizeCore(18));

        let r = engine.resonance_with(1, 2).unwrap();
        assert_eq!(r, 6);
    }

    #[test]
    fn engine_new() {
        let engine = RadixAddressingEngine::new();
        assert_eq!(engine.yinyang_store.len(), 0);
        assert_eq!(engine.fixed_regs.len(), 0);
    }
}
