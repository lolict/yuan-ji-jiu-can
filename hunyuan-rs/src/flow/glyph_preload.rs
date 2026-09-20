// glyph_preload — 汉字活容器预加载池
// ⚠️ 扩冲存根：补全 dynamic_pool 依赖的 GlyphPool 接口

use crate::glyph_living::{LivingGlyph, GlyphFamily, Shengmu, Yunmu, Tone};

#[derive(Clone, Debug)]
pub struct GlyphPool;

impl GlyphPool {
    pub fn new() -> Self { Self }

    pub fn len(&self) -> usize { 0 }

    pub fn get(&self, _c: char) -> Option<&LivingGlyph> {
        None
    }
}

impl Default for GlyphPool {
    fn default() -> Self { Self::new() }
}
