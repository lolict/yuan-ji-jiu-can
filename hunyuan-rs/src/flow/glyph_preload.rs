// ══════════════════════════════════════════════════════════════════════
// glyph_preload.rs — 汉字活容器预加载池 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 目的: 把常用汉字构造为 LivingGlyph, 放入预加载池
// 这样运行时每次直接取用, 不需每次重新构造声母/韵母等属性
//
// 预加载范围:
//   - 囡囝共生核心: 囡/囝/阴/阳/夫/妻/我/你
//   - 容器族基础: 袋/桶/罐/缸/盆
//   - 操作族基础: 照/见/开/关/分/合
//   - 审判族基础: 裁/判/审/决/断
//
// 约束:
//   - 仅预加载确定性属性 (声母/韵母/声调/家族)
//   - 不预加载坐标/见证计数 — 这些是运行时状态
// ══════════════════════════════════════════════════════════════════════

use crate::glyph_living::{LivingGlyph, GlyphFamily, Shengmu, Yunmu, Tone};
use std::collections::HashMap;

/// 预加载池: 汉字 → 活容器
pub struct GlyphPool {
    pool: HashMap<char, LivingGlyph>,
}

impl GlyphPool {
    /// 创建并填充预加载池
    pub fn new() -> Self {
        let mut pool = HashMap::new();

        // ── 囡囝共生核心 ──
        pool.insert('囡', Self::mk('囡', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::Couple));
        pool.insert('囝', Self::mk('囝', Shengmu::J, Yunmu::Iou, Tone::T1, GlyphFamily::Couple));
        pool.insert('阴', Self::mk('阴', Shengmu::Zero, Yunmu::In, Tone::T1, GlyphFamily::YinYang));
        pool.insert('阳', Self::mk('阳', Shengmu::Zero, Yunmu::Iang, Tone::T2, GlyphFamily::YinYang));
        pool.insert('夫', Self::mk('夫', Shengmu::F, Yunmu::U, Tone::T1, GlyphFamily::Couple));
        pool.insert('妻', Self::mk('妻', Shengmu::Q, Yunmu::I, Tone::T1, GlyphFamily::Couple));
        pool.insert('我', Self::mk('我', Shengmu::Zero, Yunmu::Uo, Tone::T3, GlyphFamily::Homophone));
        pool.insert('你', Self::mk('你', Shengmu::N, Yunmu::I, Tone::T3, GlyphFamily::Homophone));
        pool.insert('她', Self::mk('她', Shengmu::T, Yunmu::A, Tone::T1, GlyphFamily::Homophone));

        // ── 容器族基础 ──
        pool.insert('袋', Self::mk('袋', Shengmu::D, Yunmu::Ai, Tone::T4, GlyphFamily::Container));
        pool.insert('桶', Self::mk('桶', Shengmu::T, Yunmu::Ueng, Tone::T3, GlyphFamily::Container));
        pool.insert('罐', Self::mk('罐', Shengmu::G, Yunmu::Uan, Tone::T4, GlyphFamily::Container));
        pool.insert('缸', Self::mk('缸', Shengmu::G, Yunmu::Ang, Tone::T1, GlyphFamily::Container));
        pool.insert('盆', Self::mk('盆', Shengmu::P, Yunmu::En, Tone::T2, GlyphFamily::Container));
        pool.insert('盂', Self::mk('盂', Shengmu::Zero, Yunmu::U, Tone::T2, GlyphFamily::Container));
        pool.insert('钵', Self::mk('钵', Shengmu::B, Yunmu::O, Tone::T1, GlyphFamily::Container));
        pool.insert('盏', Self::mk('盏', Shengmu::Zh, Yunmu::An, Tone::T3, GlyphFamily::Container));
        pool.insert('盒', Self::mk('盒', Shengmu::H, Yunmu::E, Tone::T2, GlyphFamily::Container));
        pool.insert('杯', Self::mk('杯', Shengmu::B, Yunmu::Ei, Tone::T1, GlyphFamily::Container));
        pool.insert('壶', Self::mk('壶', Shengmu::H, Yunmu::U, Tone::T2, GlyphFamily::Container));
        pool.insert('瓶', Self::mk('瓶', Shengmu::P, Yunmu::Ing, Tone::T2, GlyphFamily::Container));
        pool.insert('篓', Self::mk('篓', Shengmu::L, Yunmu::Ou, Tone::T3, GlyphFamily::Container));
        pool.insert('筐', Self::mk('筐', Shengmu::K, Yunmu::Uang, Tone::T1, GlyphFamily::Container));
        pool.insert('篮', Self::mk('篮', Shengmu::L, Yunmu::An, Tone::T2, GlyphFamily::Container));
        pool.insert('箩', Self::mk('箩', Shengmu::L, Yunmu::Uo, Tone::T2, GlyphFamily::Container));
        pool.insert('筲', Self::mk('筲', Shengmu::Sh, Yunmu::Ao, Tone::T1, GlyphFamily::Container));
        pool.insert('箕', Self::mk('箕', Shengmu::J, Yunmu::I, Tone::T1, GlyphFamily::Container));
        pool.insert('笼', Self::mk('笼', Shengmu::L, Yunmu::Ueng, Tone::T2, GlyphFamily::Container));
        pool.insert('橱', Self::mk('橱', Shengmu::Ch, Yunmu::U, Tone::T2, GlyphFamily::Container));
        pool.insert('柜', Self::mk('柜', Shengmu::G, Yunmu::Uei, Tone::T4, GlyphFamily::Container));
        pool.insert('箱', Self::mk('箱', Shengmu::X, Yunmu::Iang, Tone::T1, GlyphFamily::Container));
        pool.insert('匣', Self::mk('匣', Shengmu::X, Yunmu::IA, Tone::T2, GlyphFamily::Container));
        pool.insert('奁', Self::mk('奁', Shengmu::L, Yunmu::Ian, Tone::T2, GlyphFamily::Container));
        pool.insert('瓮', Self::mk('瓮', Shengmu::Zero, Yunmu::Ueng, Tone::T4, GlyphFamily::Container));
        pool.insert('罂', Self::mk('罂', Shengmu::Zero, Yunmu::Ing, Tone::T1, GlyphFamily::Container));
        pool.insert('坛', Self::mk('坛', Shengmu::T, Yunmu::An, Tone::T2, GlyphFamily::Container));
        pool.insert('瓯', Self::mk('瓯', Shengmu::Zero, Yunmu::Ou, Tone::T1, GlyphFamily::Container));
        pool.insert('盌', Self::mk('盌', Shengmu::Zero, Yunmu::Uan, Tone::T3, GlyphFamily::Container));
        pool.insert('盅', Self::mk('盅', Shengmu::Zh, Yunmu::Ueng, Tone::T1, GlyphFamily::Container));

        // ── 操作族 ──
        pool.insert('照', Self::mk('照', Shengmu::Zh, Yunmu::Ao, Tone::T4, GlyphFamily::Operation));
        pool.insert('见', Self::mk('见', Shengmu::J, Yunmu::Ian, Tone::T4, GlyphFamily::Operation));
        pool.insert('开', Self::mk('开', Shengmu::K, Yunmu::Ai, Tone::T1, GlyphFamily::Operation));
        pool.insert('关', Self::mk('关', Shengmu::G, Yunmu::Uan, Tone::T1, GlyphFamily::Operation));
        pool.insert('分', Self::mk('分', Shengmu::F, Yunmu::En, Tone::T1, GlyphFamily::Operation));
        pool.insert('合', Self::mk('合', Shengmu::H, Yunmu::E, Tone::T2, GlyphFamily::Operation));
        pool.insert('聚', Self::mk('聚', Shengmu::J, Yunmu::U, Tone::T4, GlyphFamily::Operation));
        pool.insert('散', Self::mk('散', Shengmu::S, Yunmu::An, Tone::T4, GlyphFamily::Operation));
        pool.insert('折', Self::mk('折', Shengmu::Zh, Yunmu::E, Tone::T2, GlyphFamily::Operation));
        pool.insert('叠', Self::mk('叠', Shengmu::D, Yunmu::IE, Tone::T2, GlyphFamily::Operation));
        pool.insert('加', Self::mk('加', Shengmu::J, Yunmu::IA, Tone::T1, GlyphFamily::Operation));
        pool.insert('减', Self::mk('减', Shengmu::J, Yunmu::Ian, Tone::T3, GlyphFamily::Operation));
        pool.insert('进', Self::mk('进', Shengmu::J, Yunmu::In, Tone::T4, GlyphFamily::Operation));
        pool.insert('退', Self::mk('退', Shengmu::T, Yunmu::Uei, Tone::T4, GlyphFamily::Operation));
        pool.insert('通', Self::mk('通', Shengmu::T, Yunmu::Ueng, Tone::T1, GlyphFamily::Operation));
        pool.insert('断', Self::mk('断', Shengmu::D, Yunmu::Uan, Tone::T4, GlyphFamily::Operation));

        // ── 审判族 ──
        pool.insert('裁', Self::mk('裁', Shengmu::C, Yunmu::Ai, Tone::T2, GlyphFamily::Operation));
        pool.insert('判', Self::mk('判', Shengmu::P, Yunmu::An, Tone::T4, GlyphFamily::Operation));
        pool.insert('审', Self::mk('审', Shengmu::Sh, Yunmu::En, Tone::T3, GlyphFamily::Operation));
        pool.insert('决', Self::mk('决', Shengmu::J, Yunmu::Ue, Tone::T2, GlyphFamily::Operation));
        pool.insert('断', Self::mk('断', Shengmu::D, Yunmu::Uan, Tone::T4, GlyphFamily::Operation));
        pool.insert('定', Self::mk('定', Shengmu::D, Yunmu::Ing, Tone::T4, GlyphFamily::Operation));
        pool.insert('评', Self::mk('评', Shengmu::P, Yunmu::Ing, Tone::T2, GlyphFamily::Operation));
        pool.insert('核', Self::mk('核', Shengmu::H, Yunmu::E, Tone::T2, GlyphFamily::Operation));
        pool.insert('功', Self::mk('功', Shengmu::G, Yunmu::Ueng, Tone::T1, GlyphFamily::Operation));
        pool.insert('过', Self::mk('过', Shengmu::G, Yunmu::Uo, Tone::T4, GlyphFamily::Operation));
        pool.insert('赏', Self::mk('赏', Shengmu::Sh, Yunmu::Ang, Tone::T3, GlyphFamily::Operation));
        pool.insert('罚', Self::mk('罚', Shengmu::F, Yunmu::A, Tone::T2, GlyphFamily::Operation));
        pool.insert('褒', Self::mk('褒', Shengmu::B, Yunmu::Ao, Tone::T1, GlyphFamily::Operation));
        pool.insert('贬', Self::mk('贬', Shengmu::B, Yunmu::Ian, Tone::T3, GlyphFamily::Operation));
        pool.insert('陟', Self::mk('陟', Shengmu::Zh, Yunmu::I, Tone::T4, GlyphFamily::Operation));
        pool.insert('黜', Self::mk('黜', Shengmu::Ch, Yunmu::U, Tone::T4, GlyphFamily::Operation));

        // ── 极性族 ──
        pool.insert('盈', Self::mk('盈', Shengmu::Zero, Yunmu::Ing, Tone::T2, GlyphFamily::YinYang));
        pool.insert('亏', Self::mk('亏', Shengmu::K, Yunmu::Uei, Tone::T1, GlyphFamily::YinYang));
        pool.insert('虚', Self::mk('虚', Shengmu::X, Yunmu::U, Tone::T1, GlyphFamily::YinYang));
        pool.insert('实', Self::mk('实', Shengmu::Sh, Yunmu::I, Tone::T2, GlyphFamily::YinYang));
        pool.insert('表', Self::mk('表', Shengmu::B, Yunmu::Iao, Tone::T3, GlyphFamily::YinYang));
        pool.insert('里', Self::mk('里', Shengmu::L, Yunmu::I, Tone::T3, GlyphFamily::YinYang));
        pool.insert('升', Self::mk('升', Shengmu::Sh, Yunmu::Eng, Tone::T1, GlyphFamily::YinYang));
        pool.insert('降', Self::mk('降', Shengmu::J, Yunmu::Iang, Tone::T4, GlyphFamily::YinYang));
        pool.insert('清', Self::mk('清', Shengmu::Q, Yunmu::Ing, Tone::T1, GlyphFamily::YinYang));
        pool.insert('浊', Self::mk('浊', Shengmu::Zh, Yunmu::Uo, Tone::T2, GlyphFamily::YinYang));
        pool.insert('寒', Self::mk('寒', Shengmu::H, Yunmu::An, Tone::T2, GlyphFamily::YinYang));
        pool.insert('热', Self::mk('热', Shengmu::R, Yunmu::E, Tone::T4, GlyphFamily::YinYang));
        pool.insert('动', Self::mk('动', Shengmu::D, Yunmu::Ueng, Tone::T4, GlyphFamily::YinYang));
        pool.insert('静', Self::mk('静', Shengmu::J, Yunmu::Ing, Tone::T4, GlyphFamily::YinYang));
        pool.insert('出', Self::mk('出', Shengmu::Ch, Yunmu::U, Tone::T1, GlyphFamily::YinYang));
        pool.insert('入', Self::mk('入', Shengmu::R, Yunmu::U, Tone::T4, GlyphFamily::YinYang));

        // ── 谐音桥（莫连毛兰系列）──
        pool.insert('莫', Self::mk('莫', Shengmu::M, Yunmu::O, Tone::T4, GlyphFamily::Homophone));
        pool.insert('连', Self::mk('连', Shengmu::L, Yunmu::Ian, Tone::T2, GlyphFamily::Homophone));
        pool.insert('毛', Self::mk('毛', Shengmu::M, Yunmu::Ao, Tone::T2, GlyphFamily::Homophone));
        pool.insert('兰', Self::mk('兰', Shengmu::L, Yunmu::An, Tone::T2, GlyphFamily::Homophone));
        pool.insert('恋', Self::mk('恋', Shengmu::L, Yunmu::Ian, Tone::T4, GlyphFamily::Homophone));
        pool.insert('初', Self::mk('初', Shengmu::Ch, Yunmu::U, Tone::T1, GlyphFamily::Homophone));

        Self { pool }
    }

    /// 取出一个活容器 (返回引用)
    pub fn get(&self, glyph: char) -> Option<&LivingGlyph> {
        self.pool.get(&glyph)
    }

    /// 取出一个活容器 (克隆返回)
    pub fn get_clone(&self, glyph: char) -> Option<LivingGlyph> {
        self.pool.get(&glyph).cloned()
    }

    /// 池子里有多少汉字
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    /// 列出所有已预加载的汉字
    pub fn keys(&self) -> Vec<char> {
        self.pool.keys().copied().collect()
    }

    /// 快速构造一个活容器 (不暴露到外面)
    fn mk(glyph: char, sm: Shengmu, ym: Yunmu, t: Tone, fam: GlyphFamily) -> LivingGlyph {
        LivingGlyph::new(glyph, sm, ym, t, fam)
    }
}

impl Default for GlyphPool {
    fn default() -> Self {
        Self::new()
    }
}

// ══════════════════════════════════════════════════════════════
// 测试: 验证预加载正确性
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_contains_core() {
        let pool = GlyphPool::new();
        assert!(pool.get('囡').is_some());
        assert!(pool.get('囝').is_some());
        assert!(pool.get('阴').is_some());
        assert!(pool.get('阳').is_some());
    }

    #[test]
    fn test_pool_has_family() {
        let pool = GlyphPool::new();
        let nan = pool.get('囡').unwrap();
        assert_eq!(nan.family, GlyphFamily::Couple);

        let dai = pool.get('袋').unwrap();
        assert_eq!(dai.family, GlyphFamily::Container);
    }

    #[test]
    fn test_pool_has_shengmu() {
        let pool = GlyphPool::new();
        let nan = pool.get('囡').unwrap();
        assert_eq!(nan.shengmu, Shengmu::N);

        let zhao = pool.get('照').unwrap();
        assert_eq!(zhao.shengmu, Shengmu::Zh);
    }

    #[test]
    fn test_pool_size() {
        let pool = GlyphPool::new();
        // 预加载池至少有 30 个汉字
        assert!(pool.len() >= 30);
    }

    #[test]
    fn test_pool_no_unknown() {
        let pool = GlyphPool::new();
        assert!(pool.get(' ').is_none());
        // 未预加载的字返回 None
    }
}
