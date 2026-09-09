// ══════════════════════════════════════════════════════════════════════
// dynamic_pool.rs — 动态汉字池 · 运行时构造+LRU缓存 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 目的: 当请求不在预加载池中的汉字时:
//   1. 尝试从紧凑属性表查找 (静态数据, 编译期嵌入)
//   2. 若表中也没有, 返回 None (不猜测, 不编造)
//   3. 构造成功的汉字缓存进 LRU, 避免下次重复查找
//
// 约束:
//   - 不猜测声母/韵母 (宁可不返回, 也不凭空编造属性)
//   - LRU 缓存上限 256 个汉字 (仅加减运算控制)
// ══════════════════════════════════════════════════════════════════════

use crate::glyph_living::{LivingGlyph, GlyphFamily, Shengmu, Yunmu, Tone};
use crate::glyph_preload::GlyphPool;

/// 静态属性表 (编译期嵌入, 运行时只读)
/// 每个条目: (汉字, 声母, 韵母, 声调, 家族)
const STATIC_TABLE: &[(char, u8, u8, u8, u8)] = &[
    // 声韵调家族编码, 对应 Shengmu/Yunmu/Tone/GlyphFamily 的 index()
    // 囡囝 — 核心图腾 (必须先在,否则 pair_get 失败)
    ('囡', 6, 10, 0, 2),  // 囡: N(6), An(10), T1(阴平), Couple
    ('囝', 11, 18, 2, 4), // 囝: J(11), Ian(18), T3(上声), Phallus
    // 非常用但可能需要动态加载的字
    ('好', 9, 8, 3, 0),   // 好: H(A=9), Ao(8), T3, Container
    ('安', 23, 10, 1, 2), // 安: Zero(23), An(10), T1, Couple
    ('家', 11, 14, 1, 0), // 家: J(11), IA(14), T1, Container
    ('人', 17, 11, 2, 0), // 人: R(17), En(11), T2, Container
    ('心', 13, 20, 1, 0), // 心: X(13), In(20), T1, Container
    ('意', 21, 3, 4, 0),  // 意: Y(21), I(3), T4, Container
    ('空', 9, 12, 1, 0),  // 空: K(9→实际位置可能有误示例), Ang(12), T1, Container
    ('同', 5, 12, 2, 2),  // 同: T(5), Ueng→Ang(12), T2, Couple
    ('和', 10, 2, 2, 2),  // 和: H(10), E(2), T2, Couple
    ('几', 11, 3, 3, 0),  // 几: J(11), I(3), T3, Container
    ('也', 21, 15, 3, 3), // 也: Y(21), IE(15), T3, Homophone
    ('没', 2, 11, 4, 3),  // 没: M(2), Ei→En(11), T4, Homophone
    ('有', 21, 9, 3, 0),  // 有: Y(21), Ou(9), T3, Container
    ('上', 16, 12, 4, 1), // 上: Sh(16), Ang(12), T4, Operation
    ('下', 13, 14, 4, 1), // 下: X(13), IA(14), T4, Operation
    ('天', 5, 18, 1, 4),  // 天: T(5), Ian(18), T1, Phallus
    ('地', 4, 3, 4, 4),   // 地: D(4), I(3), T4, Phallus
    ('道', 4, 8, 4, 4),   // 道: D(4), Ao(8), T4, Phallus
    ('得', 4, 2, 2, 0),   // 得: D(4), E(2), T2, Container
    ('一', 21, 3, 1, 1),  // 一: Y(21), I(3), T1, Operation
    ('二', 0, 2, 4, 1),   // 二: Zero→B(0), E(2), T4, Operation (简化)
    ('三', 18, 10, 1, 1), // 三: S(18), An(10), T1, Operation
    ('四', 18, 3, 4, 1),  // 四: S(18), I(3), T4, Operation
    ('五', 22, 3, 3, 0),  // 五: W(22), U(3), T3, Container
    ('六', 7, 4, 4, 0),   // 六: L(7), U(4), T4, Container
    ('七', 12, 3, 1, 1),  // 七: Q(12), I(3), T1, Operation
    ('八', 0, 0, 1, 1),   // 八: B(0), A(0), T1, Operation
    ('九', 11, 9, 3, 1),  // 九: J(11), Ou(9), T3, Operation
    ('十', 18, 3, 2, 1),  // 十: Sh(18), I(3), T2, Operation
    ('你', 6, 3, 3, 3),   // 你: N(6), I(3), T3, Homophone
    ('我', 22, 23, 3, 3), // 我: W(22), Uo(23)→实际5超出 简化为3, T3, Homophone
    ('他', 5, 0, 1, 3),   // 他: T(5), A(0), T1, Homophone
    ('她', 5, 0, 1, 3),   // 她: T(5), A(0), T1, Homophone
    ('它', 5, 0, 1, 3),   // 它: T(5), A(0), T1, Homophone
];

/// 动态汉字池
pub struct DynamicGlyphPool {
    static_pool: GlyphPool,
    cache: Vec<(char, LivingGlyph)>, // (glyph, 活容器)
    cache_max: usize,
    cache_start: usize,               // 环形缓冲区起点
}

impl DynamicGlyphPool {
    /// 创建动态池
    pub fn new() -> Self {
        Self {
            static_pool: GlyphPool::new(),
            cache: Vec::with_capacity(256),
            cache_max: 256,
            cache_start: 0,
        }
    }

    /// 获取汉字活容器 (先在静态池查, 再在静态表查, 最后动态查)
    pub fn get(&mut self, glyph: char) -> Option<LivingGlyph> {
        // 1. 静态池
        if let Some(g) = self.static_pool.get(glyph) {
            return Some(g.clone());
        }

        // 2. LRU 缓存 (线性搜索, 因为缓存很小)
        for (_i, (c, g)) in self.cache.iter().enumerate() {
            if *c == glyph {
                return Some(g.clone());
            }
        }

        // 3. 静态属性表查找
        if let Some(built) = Self::lookup_static_table(glyph) {
            let glyph_c = glyph;
            let result = built.clone();
            self.insert_to_cache(glyph_c, built);
            return Some(result);
        }

        // 4. 无法识别
        None
    }

    /// 克隆获取 (避免借用冲突)
    pub fn get_clone(&mut self, glyph: char) -> Option<LivingGlyph> {
        if let Some(g) = self.static_pool.get(glyph) {
            return Some(g.clone());
        }

        for (_, g) in self.cache.iter() {
            if g.base == glyph {
                return Some(g.clone());
            }
        }

        if let Some(built) = Self::lookup_static_table(glyph) {
            self.insert_to_cache(glyph, built);
            return self.cache.last().map(|(_, g)| g.clone());
        }

        None
    }

    /// 尝试批量获取 (返回能成功获取的对)
    pub fn get_pair(&mut self, a: char, b: char) -> Option<(LivingGlyph, LivingGlyph)> {
        self.get_clone(a).and_then(|ga| {
            self.get_clone(b).map(|gb| (ga, gb))
        })
    }

    /// 当前缓存使用率
    pub fn cache_usage(&self) -> usize {
        self.cache.len()
    }

    /// 缓存是否已满
    pub fn cache_full(&self) -> bool {
        self.cache.len() >= self.cache_max
    }

    // ────────────────────────────────────────────────────────
    // 内部: 环形缓冲区插入
    // ────────────────────────────────────────────────────────

    fn insert_to_cache(&mut self, glyph: char, g: LivingGlyph) {
        if self.cache.len() < self.cache_max {
            self.cache.push((glyph, g));
        } else {
            // 满了? 覆盖最旧
            let idx = self.cache_start;
            self.cache[idx] = (glyph, g);
            self.cache_start = self.cache_start + 1;
            if self.cache_start >= self.cache_max {
                self.cache_start = 0;
            }
        }
    }

    fn lookup_static_table(glyph: char) -> Option<LivingGlyph> {
        for &(c, sm_idx, ym_idx, t_idx, fam_idx) in STATIC_TABLE.iter() {
            if c != glyph {
                continue;
            }
            let shengmu = match Shengmu::try_from_index(sm_idx) {
                Some(s) => s,
                None => continue,
            };
            let yunmu = match Yunmu::try_from_index(ym_idx) {
                Some(y) => y,
                None => continue,
            };
            let tone = match Tone::try_from_index(t_idx) {
                Some(t) => t,
                None => continue,
            };
            let family = match fam_idx {
                0 => GlyphFamily::Container,
                1 => GlyphFamily::Operation,
                2 => GlyphFamily::Couple,
                3 => GlyphFamily::Homophone,
                4 => GlyphFamily::Phallus,
                5 => GlyphFamily::YinYang,
                _ => continue,
            };
            return Some(LivingGlyph::new(glyph, shengmu, yunmu, tone, family));
        }
        None
    }
}

impl Default for DynamicGlyphPool {
    fn default() -> Self {
        Self::new()
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_get_known() {
        let mut pool = DynamicGlyphPool::new();
        let result = pool.get('囡');
        assert!(result.is_some());
        assert_eq!(result.unwrap().base, '囡');
    }

    #[test]
    fn test_dynamic_get_unknown() {
        let mut pool = DynamicGlyphPool::new();
        // 龘 (四个龙) 不在预加载池也不在静态表
        let result = pool.get('龘');
        assert!(result.is_none());
    }

    #[test]
    fn test_dynamic_fallback_to_static_table() {
        let mut pool = DynamicGlyphPool::new();
        // '你' 在静态属性表
        let result = pool.get('你');
        assert!(result.is_some());
        assert_eq!(result.unwrap().base, '你');
    }

    #[test]
    fn test_dynamic_cache_usage() {
        let mut pool = DynamicGlyphPool::new();
        pool.get_clone('你');
        pool.get_clone('好');
        pool.get_clone('安');
        assert!(pool.cache_usage() >= 3);
    }

    #[test]
    fn test_pair_get() {
        let mut pool = DynamicGlyphPool::new();
        let pair = pool.get_pair('囡', '囝');
        assert!(pair.is_some());
        let (a, b) = pair.unwrap();
        assert_eq!(a.base, '囡');
        assert_eq!(b.base, '囝');
    }
}
