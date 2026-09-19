// ────────────────────────────────────────────────────────────
// 身份条目 — 自描述 + 全局唯一
// ────────────────────────────────────────────────────────────

/// 身份条目 — 万物之中独一无二的存在
///
/// 全局唯一性由 (家族码, 时间线位置) 二元组保证
/// 即便名字相同, 家族码不同即不同存在
#[derive(Clone, Copy)]
pub struct 身份条目 {
    pub 名字: &'static str,
    pub 时间线: u16,        // D1: 家族内部位置
    pub 偏移: u32,          // 定点偏移率 (×10000)
    pub 组码: i16,          // 家族内分组码
    pub 附加: &'static str, // 额外描述
}

impl 身份条目 {
    pub const fn 新(名字: &'static str, 时间线: u16, 偏移: u32, 组码: i16, 附加: &'static str) -> Self {
        Self { 名字, 时间线, 偏移, 组码, 附加 }
    }

    /// 全局唯一编码 (接 mirror.rs 六十四位坐标系)
    ///
    /// D0 = 家族码 (哪个集装箱)
    /// D1 = 时间线位置 (集装箱内第几个)
    /// D2 = 组码绝对值 mod 64 (组级层级)
    /// 极性 = 组码正负
    pub const fn 编六十四(&self, 家族码: u8) -> u64 {
        let d0 = (家族码 & 0x3F) as u64;
        let d1 = (self.时间线 & 0x3F) as u64;
        let d2 = (self.组码.abs() as u64) & 0x3F;
        let polarity = if self.组码 >= 0 { 1u64 } else { 0u64 };
        let value = (self.偏移 / 100) as u64;
        d0 | (d1 << 10) | (d2 << 20) | (polarity << 30) | (value << 40)
    }

    /// 命名空间名 — 全局唯一的名字标识
    pub const fn 全路径(&self, 家族名: &'static str) -> [u8; 128] {
        // 拼接 "家族名::身份名"
        let mut result = [0u8; 128];
        let fam = 家族名.as_bytes();
        let id = self.名字.as_bytes();
        let mut i = 0;
        while i < fam.len() {
            result[i] = fam[i];
            i += 1;
        }
        result[i] = b':'; i += 1;
        result[i] = b':'; i += 1;
        let mut j = 0;
        while j < id.len() {
            result[i] = id[j];
            i += 1;
            j += 1;
        }
        result
    }
}

// ────────────────────────────────────────────────────────────
// 家族 — 身份容器 + 层级关系
// ────────────────────────────────────────────────────────────

/// 身份家族 — 同类型身份的层级集合
///
/// 层级结构: 家族 > 组 > 身份
/// 每个家族有唯一码, 每个身份在家族内有时间线位置
/// 全局唯一 = 家族偏移 + 时间线位置
pub struct 身份家族<const N: usize> {
    pub 名字: &'static str,
    pub 家族码: u8,
    pub 成员: [身份条目; N],
}

impl<const N: usize> 身份家族<N> {
    pub const fn 新(名字: &'static str, 家族码: u8, 成员: [身份条目; N]) -> Self {
        Self { 名字, 家族码, 成员 }
    }

    /// 家族偏移 — 此家族在全局时间线上的起始位置
    pub fn 全局偏移(&self) -> u32 {
        self.家族码 as u32 * 家族跨距
    }

    /// 通过名字查找身份
    pub fn 取(&self, 名字: &str) -> Option<&身份条目> {
        let mut i = 0;
        while i < N {
            if self.成员[i].名字.len() == 名字.len() {
                let mut same = true;
                let mut j = 0;
                while j < 名字.len() {
                    if self.成员[i].名字.as_bytes()[j] != 名字.as_bytes()[j] {
                        same = false;
                        break;
                    }
                    j += 1;
                }
                if same { return Some(&self.成员[i]); }
            }
            i += 1;
        }
        None
    }

    /// 通过时间线位置取
    pub fn 时间线取(&self, pos: u16) -> Option<&身份条目> {
        let mut i = 0;
        while i < N {
            if self.成员[i].时间线 == pos { return Some(&self.成员[i]); }
            i += 1;
        }
        None
    }

    /// 按组码取群
    pub fn 组(&self, 组码: i16) -> Vec<&身份条目> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < N {
            if self.成员[i].组码 == 组码 { result.push(&self.成员[i]); }
            i += 1;
        }
        result
    }

    /// 近缘 — 同族偏移最接近
    pub fn 近缘(&self, 名字: &str) -> Option<(&身份条目, u32)> {
        let target = self.取(名字)?;
        let mut best: Option<(&身份条目, u32)> = None;
        let mut i = 0;
        while i < N {
            let m = &self.成员[i];
            if m.名字 != target.名字 {
                let d = if m.偏移 > target.偏移 { m.偏移 - target.偏移 } else { target.偏移 - m.偏移 };
                if best.is_none() || d < best.unwrap().1 { best = Some((m, d)); }
            }
            i += 1;
        }
        best
    }

    /// 远缘 — 同族偏移最远 (夫妻对偶)
    pub fn 远缘(&self, 名字: &str) -> Option<(&身份条目, u32)> {
        let target = self.取(名字)?;
        let mut best: Option<(&身份条目, u32)> = None;
        let mut i = 0;
        while i < N {
            let m = &self.成员[i];
            if m.名字 != target.名字 {
                let d = if m.偏移 > target.偏移 { m.偏移 - target.偏移 } else { target.偏移 - m.偏移 };
                if best.is_none() || d > best.unwrap().1 { best = Some((m, d)); }
            }
            i += 1;
        }
        best
    }

    /// 总数
    pub const fn 总数(&self) -> usize { N }

    /// 全列
    pub fn 全列(&self) -> &[身份条目; N] { &self.成员 }

    /// 身份表 (向后兼容)
    pub fn 身份(&self, 名字: &str) -> Option<&身份条目> { self.取(名字) }

    /// 向后兼容: new 方法
    pub fn new() -> Self where [身份条目; N]: Default {
        Self { 名字: "", 家族码: 0, 成员: [身份条目::新("", 0, 0, 0, ""); N] }
    }
}
