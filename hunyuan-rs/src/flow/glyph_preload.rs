// glyph_preload — 汉字活容器预加载池
// ⚠️ 扩冲存根：补全 dynamic_pool 依赖的 GlyphPool 接口

use crate::glyph_living::{LivingGlyph, GlyphFamily, Shengmu, Yunmu, Tone};

#[derive(Clone, Debug)]
pub struct GlyphPool;

impl GlyphPool {
    pub fn new() -> Self { Self }

    pub fn get(&self, c: char) -> Option<&LivingGlyph> {
        static NONE: Option<LivingGlyph> = None;
        if c == '囡' {
            // This is a stub; we cannot return a reference to temporary
            return None;
        }
        None
    }
}

impl Default for GlyphPool {
    fn default() -> Self { Self::new() }
}
