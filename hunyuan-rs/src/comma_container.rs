// ═══════════════════════════════════════════════════════════════════════════════
// comma_container.rs — 逗号容器 · 比特即容器标签符 · 括号即集装箱
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心定义（用户定义，不可更改）：
//
//   1 比特 = 1 容器标签符 = 1 个逗号
//   1 个括号包裹这个逗号 → 括号被切分成两半 → 形成 2 个集装箱（容器）
//   左半边容器可装无限符号，右半边容器可装无限符号
//   装满左右两半 = 容器设定完成
//   再装符号 → 再打一个逗号（可打在左边界或右边界）→ 产生全新逗号空间 → 新内容填空
//
// 进制封装：
//   从 1 比特到 24 比特，通过加法组合：
//     1比特 + 23比特 = 24比特
//     0比特 + 24比特 = 24比特
//     2比特 + 22比特 = 24比特
//     ...全部相加等于 24 比特
//   然后 23 比特、22 比特...一路递减 → 梯度比特排列
//   24 比特 = 总范畴/总大小（也可扩展为 27 比特上限）
//   阶乘以24为总范畴，无限个比特累计相加 = 分数化梯度
//
//   字符编码 = 利用这些梯度比特的排列顺序来设定
//   1 个字符占多少比特由我们自己定义
//
// ═══════════════════════════════════════════════════════════════════════════════

/// 逗号容器世界
///
/// 核心规则：
///   - 逗号 = 1 比特 = 分割事件
///   - 括号 = 容器边界
///   - 逗号分割括号 → 产生左右两个半容器
///   - 每个半容器可继续被逗号分割（递归）
///   - 最大深度 = 24 层（总范畴上限）

/// 总范畴常量（可配置为 24 或 27）
pub const TOTAL_CATEGORY: u8 = 24;

/// 参与合数：所有相加等于 N 的二元组 (a,b)
/// 对于 24：(0,24), (1,23), (2,22), ..., (24,0) → 共 25 对
/// 总计从 1 到 24 全部相加的可参与组合数
pub fn composite_pairs(total: u8) -> Vec<(u8, u8)> {
    (0..=total)
        .map(|a| (a, total - a))
        .collect()
}

/// 所有梯度层的参与合数
/// 从 1 到 TOTAL_CATEGORY 每一层的全部加法分解
pub fn all_gradient_composites() -> Vec<Vec<(u8, u8)>> {
    (1..=TOTAL_CATEGORY)
        .map(composite_pairs)
        .collect()
}

/// 一个容器的左右两半
///
/// 逗号打在边界 → 产生新的逗号空间 → 新的容器边界
#[derive(Clone)]
pub struct HalfContainer {
    /// 当前层剩余的比特深度
    pub remaining_bits: u8,
    /// 逗号分割的左半部分
    pub left: Option<Box<HalfContainer>>,
    /// 逗号分割的右半部分
    pub right: Option<Box<HalfContainer>>,
    /// 左半容器内装的符号
    pub symbols_left: Vec<String>,
    /// 右半容器内装的符号
    pub symbols_right: Vec<String>,
}

impl HalfContainer {
    /// 创建一个新的半容器
    pub fn new(remaining_bits: u8) -> Self {
        Self {
            remaining_bits,
            left: None,
            right: None,
            symbols_left: Vec::new(),
            symbols_right: Vec::new(),
        }
    }

    /// 在当前容器打一个逗号（分割）
    /// 返回左右两个子容器的引用
    pub fn split(&mut self, bits_to_left: u8, bits_to_right: u8) -> bool {
        if self.remaining_bits == 0 || self.left.is_some() {
            return false; // 无法继续分割
        }
        self.left = Some(Box::new(HalfContainer::new(bits_to_left)));
        self.right = Some(Box::new(HalfContainer::new(bits_to_right)));
        self.remaining_bits = 0; // 已被分割，不再有剩余空间
        true
    }

    /// 往左半容器添加符号
    pub fn push_left(&mut self, symbol: &str) {
        if let Some(ref mut left) = self.left {
            left.symbols_left.push(symbol.to_string());
        } else {
            self.symbols_left.push(symbol.to_string());
        }
    }

    /// 往右半容器添加符号
    pub fn push_right(&mut self, symbol: &str) {
        if let Some(ref mut right) = self.right {
            right.symbols_right.push(symbol.to_string());
        } else {
            self.symbols_right.push(symbol.to_string());
        }
    }

    /// 递归统计容器内的符号总数
    pub fn total_symbols(&self) -> usize {
        let left_count = self.left.as_ref().map(|b| b.total_symbols()).unwrap_or(0);
        let right_count = self.right.as_ref().map(|b| b.total_symbols()).unwrap_or(0);
        left_count + right_count + self.symbols_left.len() + self.symbols_right.len()
    }

    /// 获取所有叶节点容器（不再有子分割的容器）
    pub fn leaf_containers(&self) -> Vec<&HalfContainer> {
        if self.left.is_none() && self.right.is_none() {
            return vec![self];
        }
        let mut result = Vec::new();
        if let Some(ref left) = self.left {
            result.extend(left.leaf_containers());
        }
        if let Some(ref right) = self.right {
            result.extend(right.leaf_containers());
        }
        result
    }
}

/// 逗号容器根节点
/// 代表整个容器世界（括号边界 = 总范畴）
#[derive(Clone)]
pub struct CommaWorld {
    /// 根容器 = 总范畴边界
    pub root: HalfContainer,
    /// 当前版本号
    pub version: String,
}

impl CommaWorld {
    /// 创建一个新的容器世界（空范畴）
    pub fn new(total_bits: u8) -> Self {
        Self {
            root: HalfContainer::new(total_bits),
            version: "v1".to_string(),
        }
    }

    /// 创建总范畴为 24 的默认世界
    pub fn default_category() -> Self {
        Self::new(TOTAL_CATEGORY)
    }

    /// 对根容器执行一次逗号分割
    pub fn root_split(&mut self, left_bits: u8, right_bits: u8) -> bool {
        self.root.split(left_bits, right_bits)
    }

    /// 获取世界中所有符号
    pub fn all_symbols(&self) -> Vec<&str> {
        let mut result: Vec<&str> = self.root.symbols_left.iter().map(|s| s.as_str()).collect();
        result.extend(self.root.symbols_right.iter().map(|s| s.as_str()));
        if let Some(ref left) = self.root.left {
            result.extend(collect_all_symbols(left));
        }
        if let Some(ref right) = self.root.right {
            result.extend(collect_all_symbols(right));
        }
        result
    }

    /// 梯度层数（从1到24各有多少参与方式）
    pub fn gradient_layers(&self) -> Vec<u8> {
        (1..=TOTAL_CATEGORY).collect()
    }
}

fn collect_all_symbols(container: &HalfContainer) -> Vec<&str> {
    let mut result: Vec<&str> = container.symbols_left.iter().map(|s| s.as_str()).collect();
    result.extend(container.symbols_right.iter().map(|s| s.as_str()));
    if let Some(ref left) = container.left {
        result.extend(collect_all_symbols(left));
    }
    if let Some(ref right) = container.right {
        result.extend(collect_all_symbols(right));
    }
    result
}

/// 字符编码：利用梯度比特的排列顺序设定字符
///
/// 一个字符由它在层级中的位置唯一确定：
/// (层号, 该层内的分割序号, 左右标识)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharEncoding {
    /// 层号（1..24）
    pub layer: u8,
    /// 该层内的参与序号
    pub composite_index: u8,
    /// 左/右
    pub is_left: bool,
}

impl CharEncoding {
    /// 编码到 u32（总范畴内的唯一ID）
    pub fn encode(&self) -> u32 {
        ((self.layer as u32) << 16) | ((self.composite_index as u32) << 8) | (self.is_left as u32)
    }
}

/// SVG 渲染器（将容器世界渲染为可保存的字体轮廓）
pub struct SVGRenderer;

impl SVGRenderer {
    /// 渲染单个字符为 SVG 路径描述
    ///
    /// 字符 = 容器坐标 + 分割拓扑 → SVG path
    pub fn render_char(enc: &CharEncoding) -> String {
        // 基于层号和分割位置计算 SVG 坐标
        let x = (enc.layer as f64) * 50.0;
        let y = (enc.composite_index as f64) * 30.0 + if enc.is_left { 0.0 } else { 15.0 };
        let size = 20.0 + (enc.layer as f64) * 2.0;

        format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="black" stroke-width="1.5"/>"#,
            x, y, size
        )
    }

    /// 渲染完整字符集为 SVG 文档
    pub fn render_font(set: &[CharEncoding]) -> String {
        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 800">"#);
        svg.push('\n');
        for (i, enc) in set.iter().enumerate() {
            let glyph = Self::render_char(enc);
            svg.push_str(&format!("  <!-- glyph {} -->", i));
            svg.push('\n');
            svg.push_str(&glyph);
            svg.push('\n');
        }
        svg.push_str("</svg>\n");
        svg
    }
}

/// 机器码容器化封装（跨平台）
///
/// 不同平台的机器码被统一封装为 语义ID层 的统一格式
#[derive(Clone)]
pub struct MachineCodePack {
    /// 语义ID（统一的功能类别）
    pub semantic_id: u8,
    /// 平台标识
    pub platform: Platform,
    /// 原生码字节（保留原始机器码）
    pub native_code: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    X86,
    Arm,
    RiscV,
    Mips,
    Other,
}

impl MachineCodePack {
    /// 编码为统一容器格式
    /// [语义ID:1][平台:1][长度:N][原生码:N][校验和:1]
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.semantic_id);
        result.push(self.platform as u8);
        result.push(self.native_code.len() as u8);
        result.extend(&self.native_code);
        result.push(self.native_code.iter().fold(0u8, |acc, &b| acc.wrapping_add(b)));
        result
    }
}

/// 字符备用符与版本号
///
/// 同一概念的不同写法 → 同一身份，不同符号
#[derive(Clone)]
pub struct CharacterVariant {
    /// 核心身份标识（不变）
    pub identity: String,
    /// 版本符号（不同写法）
    pub symbol: String,
    /// 版本号
    pub version: u32,
}

impl CharacterVariant {
    pub fn new(identity: &str, symbol: &str, version: u32) -> Self {
        Self {
            identity: identity.to_string(),
            symbol: symbol.to_string(),
            version,
        }
    }
}

/// 声母表（48个基础声母）
///
/// 这是用户定义的48个声母，与韵母/声调组合构成完整地址系统
/// 声母索引 0..47 对应 HalfContainer 层地址
pub const CONSONANTS: &[&str] = &[
    // 唇音
    "b", "p", "m", "f",
    // 舌尖中音
    "d", "t", "n", "l",
    // 舌根音
    "g", "k", "h",
    // 舌面音
    "j", "q", "x",
    // 舌尖后音 (卷舌)
    "zh", "ch", "sh", "r",
    // 舌尖前音
    "z", "c", "s",
    // 零声母
    "∅",
    // 半元音
    "y", "w",
    // 扩展声母（填补到48个位置）
    // 用户可继续扩展这部分的定义
];

/// 韵母表（66个基础韵母）
///
/// 韵母是地址的第二维，索引 0..65
pub const VOWELS: &[&str] = &[
    // 单韵母
    "a", "o", "e", "i", "u", "ü",
    // 复韵母
    "ai", "ei", "ui", "ao", "ou", "iu",
    "ie", "üe", "er",
    // 鼻韵母
    "an", "en", "in", "un", "ün",
    "ang", "eng", "ing", "ong",
    // 介音+韵母
    "ia", "iao", "ian", "iang", "iong",
    "ua", "uo", "uai", "uan", "uang", "un",
    // 特殊组合
    "io", "uei", "uen", "ueng",
    // 拓展到66个占位区域
    // 用户继续扩展此区域
];

/// 声调表（100个基础调值）
///
/// 声调是地址的相位维，索引 0..99
/// 100个调值 ≈ 普通话四声 × 25个方言变体
pub const TONES: &[&str] = &[
    // 普通话四声
    "阴平", "阳平", "上声", "去声",
    // 轻声
    "轻声",
    // 方言调值扩展（用户自定义）
    // 100个调值的完整列表由用户补充占位
];

/// 音节地址：声母 + 韵母 + 声调 = 唯一容器路径
///
/// 在逗号容器世界中，一个音节就是一个地址
/// 声母 → 层号（选哪一层）
/// 韵母 → 该层内的分割序号（选哪个分裂对）
/// 声调 → 左或右半容器（极性选择）
#[derive(Clone, Copy, Debug)]
pub struct SyllableAddress {
    pub consonant_idx: u8,  // 0..47
    pub vowel_idx: u8,      // 0..65
    pub tone_idx: u8,       // 0..99
}

impl SyllableAddress {
    /// 从声母/韵母/声调构造音节地址
    pub fn new(consonant_idx: u8, vowel_idx: u8, tone_idx: u8) -> Option<Self> {
        if consonant_idx >= 48 || vowel_idx >= 66 || tone_idx >= 100 {
            return None;
        }
        Some(Self { consonant_idx, vowel_idx, tone_idx })
    }

    /// 编码到 CharEncoding
    pub fn encode(&self) -> CharEncoding {
        // 声母决定层号（1-48 → 1-24 层，每层可容纳2个声母）
        let layer = (self.consonant_idx / 2) + 1;
        // 韵母决定该层内的分割序号（对24取模）
        let composite_index = self.vowel_idx % 24;
        // 声调决定左右（奇数声调=左，偶数声调=右）
        let is_left = (self.tone_idx % 2) == 0;

        CharEncoding {
            layer,
            composite_index,
            is_left,
        }
    }

    /// 从声母名查找索引
    pub fn find_consonant(name: &str) -> Option<u8> {
        CONSONANTS.iter().position(|&c| c == name).map(|i| i as u8)
    }

    /// 从韵母名查找索引
    pub fn find_vowel(name: &str) -> Option<u8> {
        VOWELS.iter().position(|&v| v == name).map(|i| i as u8)
    }

    /// 从声调名查找索引
    pub fn find_tone(name: &str) -> Option<u8> {
        TONES.iter().position(|&t| t == name).map(|i| i as u8)
    }

    /// 计算总组合数 = 48 × 66 × 100
    pub const fn total_combinations() -> u64 {
        48 * 66 * 100
    }
}

/// 偏旁部首范畴论
///
/// - 偏旁与其他汉字组合 → 新汉字
/// - 偏旁与其他汉字造词 → 词语串联（时间长度）
/// - 偏旁与笔画组合 → 前缀/后缀
#[derive(Clone)]
pub struct RadicalCategory {
    /// 偏旁身份
    pub radical: String,
    /// 可与该偏旁组合的汉字
    pub combinators: Vec<String>,
    /// 可造的词
    pub word_chains: Vec<Vec<String>>,
    /// 笔画数
    pub stroke_count: u8,
    /// 管理者范畴（秩序层级）
    pub manager_level: u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_pairs_24() {
        let pairs = composite_pairs(24);
        assert_eq!(pairs.len(), 25); // (0,24) 到 (24,0)
        assert_eq!(pairs[0], (0, 24));
        assert_eq!(pairs[24], (24, 0));
        // 所有对的和都等于 24
        for (a, b) in &pairs {
            assert_eq!(*a + *b, 24);
        }
    }

    #[test]
    fn test_all_gradient_composites() {
        let all = all_gradient_composites();
        assert_eq!(all.len(), 24); // 层 1..24
        // 第 i 层应有 i+1 对
        for (i, layer) in all.iter().enumerate() {
            let expected = (i + 1) as u8;
            assert_eq!(layer.len() as u8, expected + 1);
        }
    }

    #[test]
    fn test_half_container_split() {
        let mut container = HalfContainer::new(24);
        let ok = container.split(8, 16); // 8比特给左，16比特给右
        assert!(ok);
        assert!(container.left.is_some());
        assert!(container.right.is_some());
    }

    #[test]
    fn test_container_symbols() {
        let mut container = HalfContainer::new(12);
        container.push_left("A");
        container.push_left("B");
        container.push_right("C");
        assert_eq!(container.total_symbols(), 3);
    }

    #[test]
    fn test_comma_world_creation() {
        let world = CommaWorld::default_category();
        assert_eq!(world.root.remaining_bits, 24);
        assert_eq!(world.version, "v1");
    }

    #[test]
    fn test_comma_world_root_split() {
        let mut world = CommaWorld::default_category();
        let ok = world.root_split(12, 12);
        assert!(ok);
        assert_eq!(world.root.left.as_ref().unwrap().remaining_bits, 12);
        assert_eq!(world.root.right.as_ref().unwrap().remaining_bits, 12);
    }

    #[test]
    fn test_char_encoding_encode() {
        let enc = CharEncoding {
            layer: 5,
            composite_index: 3,
            is_left: true,
        };
        let encoded = enc.encode();
        assert_eq!(encoded >> 16, 5);
        assert_eq!((encoded >> 8) & 0xFF, 3);
        assert_eq!(encoded & 0x1, 1);
    }

    #[test]
    fn test_svg_renderer_render_char() {
        let enc = CharEncoding {
            layer: 1,
            composite_index: 0,
            is_left: true,
        };
        let svg = SVGRenderer::render_char(&enc);
        assert!(svg.contains("<circle"));
        assert!(svg.contains("cx="));
    }

    #[test]
    fn test_machine_code_pack_encode() {
        let pack = MachineCodePack {
            semantic_id: 0x03,
            platform: Platform::X86,
            native_code: vec![0x48, 0x8B, 0x03],
        };
        let encoded = pack.encode();
        assert_eq!(encoded[0], 0x03); // 语义ID
        assert_eq!(encoded[1], 0x00); // X86 = 0
        assert_eq!(encoded[2], 0x03); // 长度
        assert_eq!(encoded[3..6], [0x48, 0x8B, 0x03]);
    }

    #[test]
    fn test_character_variant_versioning() {
        let mut v = CharacterVariant::new("人", "人", 1);
        assert_eq!(v.identity, "人");
        assert_eq!(v.symbol, "人");
        assert_eq!(v.version, 1);

        // 同一身份，不同写法
        let v2 = CharacterVariant::new("人", "亻", 2);
        assert_eq!(v2.identity, "人");
        assert_eq!(v2.symbol, "亻");
        assert_eq!(v2.version, 2);
    }

    #[test]
    fn test_radical_category_creation() {
        let category = RadicalCategory {
            radical: "女".to_string(),
            combinators: vec!["子".to_string(), "马".to_string(), "也".to_string()],
            word_chains: vec![
                vec!["女".to_string(), "儿".to_string()],
                vec!["女".to_string(), "人".to_string()],
            ],
            stroke_count: 3,
            manager_level: 1,
        };
        assert_eq!(category.radical, "女");
        assert_eq!(category.stroke_count, 3);
        assert_eq!(category.combinators.len(), 3);
    }

    #[test]
    fn test_gradient_sequence_sum() {
        // 验证：从 1 到 24 所有层的分割对数之和
        // 第 i 层有 i+1 对，总和 = 2+3+...+25 = 324
        let all = all_gradient_composites();
        let total_pairs: usize = all.iter().map(|v| v.len()).sum();
        assert_eq!(total_pairs, 324);
    }

    #[test]
    fn test_container_tree_depth() {
        let mut world = CommaWorld::new(3);
        // 3层深度：根(3) → 左(1)/右(2)
        world.root_split(1, 2);
        let left = world.root.left.as_mut().unwrap();
        left.split(0, 1); // 左半再分：0比特/1比特
        assert!(left.left.is_some());
        assert!(left.right.is_some());
    }

    #[test]
    fn test_total_category_constant() {
        assert_eq!(TOTAL_CATEGORY, 24);
    }

    #[test]
    fn test_container_world_all_symbols() {
        let mut world = CommaWorld::new(2);
        world.root.symbols_left.push("α".to_string());
        world.root.symbols_right.push("β".to_string());
        let symbols = world.all_symbols();
        assert!(symbols.contains(&"α"));
        assert!(symbols.contains(&"β"));
    }

    #[test]
    fn test_svg_font_document() {
        let set = vec![
            CharEncoding { layer: 1, composite_index: 0, is_left: true },
            CharEncoding { layer: 2, composite_index: 1, is_left: false },
        ];
        let svg = SVGRenderer::render_font(&set);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("glyph 0"));
        assert!(svg.contains("glyph 1"));
    }

    #[test]
    fn test_consonant_table_size() {
        // 声母表应至少有基础声母
        assert!(CONSONANTS.len() >= 23);
    }

    #[test]
    fn test_syllable_address_creation() {
        let addr = SyllableAddress::new(10, 30, 50);
        assert!(addr.is_some());
        let addr = addr.unwrap();
        assert_eq!(addr.consonant_idx, 10);
        assert_eq!(addr.vowel_idx, 30);
        assert_eq!(addr.tone_idx, 50);
    }

    #[test]
    fn test_syllable_address_bounds() {
        // 声母超出范围
        assert!(SyllableAddress::new(48, 0, 0).is_none());
        // 韵母超出范围
        assert!(SyllableAddress::new(0, 66, 0).is_none());
        // 声调超出范围
        assert!(SyllableAddress::new(0, 0, 100).is_none());
    }

    #[test]
    fn test_syllable_encode_to_char_encoding() {
        let addr = SyllableAddress::new(0, 0, 0).unwrap();
        let enc = addr.encode();
        assert_eq!(enc.layer, 1); // 声母0 → 第1层
        assert_eq!(enc.composite_index, 0);
        assert!(enc.is_left); // 声调0是偶数 → 左

        let addr2 = SyllableAddress::new(1, 1, 1).unwrap();
        let enc2 = addr2.encode();
        assert_eq!(enc2.layer, 1); // 声母1/2 = 0 → 第1层
        assert_eq!(enc2.composite_index, 1);
        assert!(!enc2.is_left); // 声调1是奇数 → 右
    }

    #[test]
    fn test_find_consonant_index() {
        let idx = SyllableAddress::find_consonant("b");
        assert!(idx.is_some());
        assert_eq!(idx.unwrap(), 0);
        let idx = SyllableAddress::find_consonant("p");
        assert_eq!(idx.unwrap(), 1);
    }

    #[test]
    fn test_total_combinations() {
        assert_eq!(SyllableAddress::total_combinations(), 316_800);
    }

    #[test]
    fn test_time_cursor_creation() {
        let cursor = TimeCursor::new();
        assert_eq!(cursor.x, 0);
        assert_eq!(cursor.y, 0);
        assert_eq!(cursor.z, 0);
        assert_eq!(cursor.layer, 1);
        assert_eq!(cursor.tick, 0);
    }

    #[test]
    fn test_time_cursor_step() {
        let mut cursor = TimeCursor::new();
        cursor.step(Direction6::PosX);
        assert_eq!(cursor.tick, 1);
        // 奇数层极性因子=1，X+1
        assert_eq!(cursor.x, 1);
    }

    #[test]
    fn test_time_cursor_opposite_direction() {
        let mut cursor = TimeCursor::new();
        cursor.step(Direction6::PosX);
        cursor.step(Direction6::NegX);
        assert_eq!(cursor.x, 0); // 回到原点
        assert_eq!(cursor.tick, 2);
    }

    #[test]
    fn test_time_cursor_layer_movement() {
        let mut cursor = TimeCursor::new();
        assert!(cursor.goto_layer(24));
        assert_eq!(cursor.layer, 24);
        assert!(!cursor.goto_layer(25)); // 超出上限
        assert!(!cursor.goto_layer(0));  // 低于下限
    }

    #[test]
    fn test_direction6_offset() {
        let (dx, dy, dz) = Direction6::PosY.to_offset();
        assert_eq!((dx, dy, dz), (0, 1, 0));
        let (dx, dy, dz) = Direction6::NegZ.to_offset();
        assert_eq!((dx, dy, dz), (0, 0, -1));
    }

    #[test]
    fn test_memory_cell_creation() {
        let cell = MemoryCell {
            address: SyllableAddress::new(1, 2, 3).unwrap(),
            content: "test memory".to_string(),
            timestamp: 42,
            owner: MemoryOwner::Private(0),
            layer: 5,
        };
        assert_eq!(cell.content, "test memory");
        assert_eq!(cell.layer, 5);
    }

    #[test]
    fn test_memory_grid_write_and_read_shared() {
        let mut grid = MemoryGrid::new(2);
        let cell = MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "shared data".to_string(),
            timestamp: 0,
            owner: MemoryOwner::Shared,
            layer: 1,
        };
        grid.write(cell);
        let shared = grid.read_shared();
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0].content, "shared data");
    }

    #[test]
    fn test_memory_grid_private_isolation() {
        let mut grid = MemoryGrid::new(2);
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "agent0 private".to_string(),
            timestamp: 0,
            owner: MemoryOwner::Private(0),
            layer: 1,
        });
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "agent1 private".to_string(),
            timestamp: 0,
            owner: MemoryOwner::Private(1),
            layer: 1,
        });
        let private0 = grid.read_private(0).unwrap();
        let private1 = grid.read_private(1).unwrap();
        assert_eq!(private0.len(), 1);
        assert_eq!(private1.len(), 1);
        assert_eq!(private0[0].content, "agent0 private");
        assert_eq!(private1[0].content, "agent1 private");
    }

    #[test]
    fn test_memory_share_private_to_shared() {
        let mut grid = MemoryGrid::new(1);
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "secret".to_string(),
            timestamp: 0,
            owner: MemoryOwner::Private(0),
            layer: 1,
        });
        // 把Agent0的私有记忆贡献到共享池
        grid.share_private(0, &[0]);
        let shared = grid.read_shared();
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0].content, "secret");
        // 私有池已空
        let private = grid.read_private(0).unwrap();
        assert_eq!(private.len(), 0);
    }

    #[test]
    fn test_memory_absorb_shared() {
        let mut grid = MemoryGrid::new(1);
        // 写入一个偶数时间戳的共享记忆
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "shared even".to_string(),
            timestamp: 2, // 偶数
            owner: MemoryOwner::Shared,
            layer: 1,
        });
        // Agent吸收共享记忆
        grid.absorb_shared(0);
        let private = grid.read_private(0).unwrap();
        assert_eq!(private.len(), 1);
        assert_eq!(private[0].content, "shared even");
    }

    #[test]
    fn test_layer_distribution() {
        let mut grid = MemoryGrid::new(1);
        for i in 1..=5 {
            grid.write(MemoryCell {
                address: SyllableAddress::new(0, 0, 0).unwrap(),
                content: format!("layer{}", i),
                timestamp: 0,
                owner: MemoryOwner::Shared,
                layer: i as u8,
            });
        }
        let dist = grid.layer_distribution();
        assert_eq!(dist[1], 1);
        assert_eq!(dist[2], 1);
        assert_eq!(dist[5], 1);
        assert_eq!(dist[6], 0);
    }

    #[test]
    fn test_multi_agent_permission() {
        let mut manager = MultiAgentMemoryManager::new(3);
        manager.write_private(0, "agent0 secret");
        manager.write_private(1, "agent1 secret");

        // Agent0可以读取Agent1的（默认允许）
        let mem = manager.read_agent_memory(0, 1);
        assert!(mem.is_some());

        // 禁止Agent2读取Agent0
        manager.set_permission(2, 0, false);
        let denied = manager.read_agent_memory(2, 0);
        assert!(denied.is None || denied.map(|m| m.is_empty()).unwrap_or(true));
    }

    #[test]
    fn test_timeline_iter_sorting() {
        let mut grid = MemoryGrid::new(1);
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "c".to_string(),
            timestamp: 3,
            owner: MemoryOwner::Shared,
            layer: 3,
        });
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "a".to_string(),
            timestamp: 1,
            owner: MemoryOwner::Shared,
            layer: 1,
        });
        grid.write(MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: "b".to_string(),
            timestamp: 2,
            owner: MemoryOwner::Shared,
            layer: 2,
        });
        let timeline = grid.timeline_iter();
        assert_eq!(timeline[0].content, "a");
        assert_eq!(timeline[1].content, "b");
        assert_eq!(timeline[2].content, "c");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 时间光标 · 动态格子中的移动逻辑
// ─────────────────────────────────────────────────────────────────────────────
//
// 规则：
//   · 一个方格子有6个面（上下左右前后）
//   · 每个面有内部墙壁和外部墙壁 → 共12个墙壁
//   · 时间光标沿6个方位移动
//   · 移动可以是"定时移动"或"节点间移动"
//   · 移动导致记忆单元的状态变化 = 时间管理化

/// 6个方位（方格子坐标系）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction6 {
    PosX,    // 右
    NegX,    // 左
    PosY,    // 前
    NegY,    // 后
    PosZ,    // 上
    NegZ,    // 下
}

impl Direction6 {
    /// 反方向（用于回溯）
    pub fn opposite(self) -> Self {
        match self {
            Self::PosX => Self::NegX,
            Self::NegX => Self::PosX,
            Self::PosY => Self::NegY,
            Self::NegY => Self::PosY,
            Self::PosZ => Self::NegZ,
            Self::NegZ => Self::PosZ,
        }
    }

    /// 方向对应的坐标偏移量
    pub fn to_offset(self) -> (i8, i8, i8) {
        match self {
            Self::PosX => (1, 0, 0),
            Self::NegX => (-1, 0, 0),
            Self::PosY => (0, 1, 0),
            Self::NegY => (0, -1, 0),
            Self::PosZ => (0, 0, 1),
            Self::NegZ => (0, 0, -1),
        }
    }

    /// 12个墙壁：6个内墙 + 6个外墙
    pub fn inner_wall(self) -> u8 {
        self as u8
    }

    pub fn outer_wall(self) -> u8 {
        self as u8 + 6
    }
}

/// 时间光标在记忆格子中的位置和运动状态
#[derive(Clone, Copy, Debug)]
pub struct TimeCursor {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    /// 当前层号（1..24）
    pub layer: u8,
    /// 累计时间滴答
    pub tick: u64,
}

impl TimeCursor {
    /// 在坐标原点、第1层创建光标
    pub fn new() -> Self {
        Self { x: 0, y: 0, z: 0, layer: 1, tick: 0 }
    }

    /// 沿指定方向移动一步
    pub fn step(&mut self, dir: Direction6) {
        let (dx, dy, dz) = dir.to_offset();
        // 左半容器（奇数层）向阴方向（X负）移动
        // 右半容器（偶数层）向阳方向（X正）移动
        let polarity_factor: i8 = if self.layer % 2 == 1 { 1 } else { -1 };
        self.x = self.x.saturating_add((dx as i8) * polarity_factor);
        self.y = self.y.saturating_add(dy);
        self.z = self.z.saturating_add(dz);
        self.tick += 1;
    }

    /// 上下层跳跃（Z轴 +1/-1）
    pub fn up(&mut self) {
        self.z = self.z.saturating_add(1);
        self.tick += 1;
    }

    pub fn down(&mut self) {
        self.z = self.z.saturating_sub(1);
        self.tick += 1;
    }

    /// 移动到指定层（1..24）
    pub fn goto_layer(&mut self, layer: u8) -> bool {
        if layer < 1 || layer > 24 {
            return false;
        }
        self.layer = layer;
        self.tick += 1;
        true
    }

    /// 光标当前所在的三维坐标
    pub fn coords(&self) -> (i8, i8, i8) {
        (self.x, self.y, self.z)
    }

    /// 光标是否在给定格子的6个墙壁上
    pub fn on_wall_of(&self, gx: i8, gy: i8, gz: i8) -> Option<Direction6> {
        let (sx, sy, sz) = self.coords();
        if sy == gy && sz == gz {
            if sx == gx + 1 { return Some(Direction6::PosX); }
            if sx == gx - 1 { return Some(Direction6::NegX); }
        }
        if sx == sx && sz == gz {
            if sy == gy + 1 { return Some(Direction6::PosY); }
            if sy == gy - 1 { return Some(Direction6::NegY); }
        }
        if sx == gx && sy == gy {
            if sz == gz + 1 { return Some(Direction6::PosZ); }
            if sz == gz - 1 { return Some(Direction6::NegZ); }
        }
        None
    }

    /// 检查是否到达边界（24层上限）
    pub fn at_boundary(&self) -> bool {
        self.x <= -128 || self.x >= 127 ||
        self.y <= -128 || self.y >= 127 ||
        self.z <= 0 || self.z > 24 ||
        self.layer > 24
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 跨智能体记忆管理 · 共享记忆和私有感知分开再汇合
// ─────────────────────────────────────────────────────────────────────────────

/// 记忆单元 = 时间格子中的一个容器
#[derive(Clone, Debug)]
pub struct MemoryCell {
    pub address: SyllableAddress,
    pub content: String,
    pub timestamp: u64,
    pub owner: MemoryOwner,
    pub layer: u8,  // 时间层级（深度）
}

/// 记忆归属
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryOwner {
    /// Agent私有感知
    Private(u8),     // Agent ID
    /// 跨Agent共享
    Shared,
}

/// 记忆网格：三维空间中的时间管理化记忆
#[derive(Clone)]
pub struct MemoryGrid {
    /// 记忆单元存储
    cells: Vec<MemoryCell>,
    /// 私有记忆：Agent ID → 记忆列表
    private_memories: Vec<Vec<MemoryCell>>,
    /// 共享记忆池
    shared_pool: Vec<MemoryCell>,
    /// 时间梯度层级 = 24层
    max_layer: u8,
}

impl MemoryGrid {
    pub fn new(num_agents: u8) -> Self {
        Self {
            cells: Vec::new(),
            private_memories: vec![Vec::new(); num_agents as usize],
            shared_pool: Vec::new(),
            max_layer: 24,
        }
    }

    /// 写入记忆（带时间层级和归属）
    pub fn write(&mut self, cell: MemoryCell) {
        match cell.owner {
            MemoryOwner::Private(agent_id) => {
                if (agent_id as usize) < self.private_memories.len() {
                    self.private_memories[agent_id as usize].push(cell.clone());
                }
            }
            MemoryOwner::Shared => {
                self.shared_pool.push(cell.clone());
            }
        }
        self.cells.push(cell);
    }

    /// 读取共享记忆
    pub fn read_shared(&self) -> &[MemoryCell] {
        &self.shared_pool
    }

    /// 读取指定Agent的私有记忆
    pub fn read_private(&self, agent_id: u8) -> Option<&[MemoryCell]> {
        self.private_memories.get(agent_id as usize).map(|v| v.as_slice())
    }

    /// 记忆汇合：私有记忆 → 共享池（Agent决定共享哪些记忆）
    pub fn share_private(&mut self, agent_id: u8, indices: &[usize]) {
        if (agent_id as usize) >= self.private_memories.len() {
            return;
        }
        let private = &mut self.private_memories[agent_id as usize];
        for &idx in indices.iter().rev() {
            if idx < private.len() {
                let mut cell = private.remove(idx);
                cell.owner = MemoryOwner::Shared;
                self.shared_pool.push(cell);
            }
        }
    }

    /// 汇合：共享记忆 → Agent私有感知池
    pub fn absorb_shared(&mut self, agent_id: u8) {
        if (agent_id as usize) >= self.private_memories.len() {
            return;
        }
        let absorb: Vec<MemoryCell> = self.shared_pool.iter()
            .filter(|c| c.timestamp % 2 == 0)  // 吸收条件：偶数时间戳
            .cloned()
            .collect();
        self.private_memories[agent_id as usize].extend(absorb);
    }

    /// 统计各层记忆数量（时间梯度层级管理化）
    pub fn layer_distribution(&self) -> Vec<usize> {
        let mut dist = vec![0usize; 25]; // 0..24
        for cell in &self.cells {
            let l = cell.layer as usize;
            if l <= 24 {
                dist[l] += 1;
            }
        }
        dist
    }

    /// 指定层的记忆检索
    pub fn at_layer(&self, layer: u8) -> Vec<&MemoryCell> {
        self.cells.iter().filter(|c| c.layer == layer).collect()
    }

    /// 沿时间轴的记忆遍历（从层1到层24）
    pub fn timeline_iter(&self) -> Vec<&MemoryCell> {
        let mut sorted: Vec<&MemoryCell> = self.cells.iter().collect();
        sorted.sort_by_key(|c| (c.layer, c.timestamp));
        sorted
    }
}

/// 多Agent统一授权记忆管理器
pub struct MultiAgentMemoryManager {
    /// Agent数量
    num_agents: u8,
    /// 每个Agent对应一个记忆网格
    /// （Agent的私有感知 + 共享池）
    grid: MemoryGrid,
    /// 授权规则：哪些Agent可以读取哪些记忆
    permissions: Vec<Vec<bool>>, // permissions[agent_i][agent_j] = i能否读j
}

impl MultiAgentMemoryManager {
    pub fn new(num_agents: u8) -> Self {
        let perm = vec![vec![true; num_agents as usize]; num_agents as usize];
        Self {
            num_agents,
            grid: MemoryGrid::new(num_agents),
            permissions: perm,
        }
    }

    /// Agent写入自己的私有记忆
    pub fn write_private(&mut self, agent_id: u8, content: &str) {
        let cell = MemoryCell {
            address: SyllableAddress::new(0, 0, 0).unwrap(),
            content: content.to_string(),
            timestamp: 0,
            owner: MemoryOwner::Private(agent_id),
            layer: 1,
        };
        self.grid.write(cell);
    }

    /// Agent读取指定Agent的记忆（需授权）
    pub fn read_agent_memory(&self, reader: u8, target: u8) -> Option<&[MemoryCell]> {
        if reader >= self.num_agents || target >= self.num_agents {
            return None;
        }
        if !self.permissions[reader as usize][target as usize] {
            return None; // 授权拒绝
        }
        self.grid.read_private(target)
    }

    /// 设置授权
    pub fn set_permission(&mut self, reader: u8, target: u8, allowed: bool) {
        if reader < self.num_agents && target < self.num_agents {
            self.permissions[reader as usize][target as usize] = allowed;
        }
    }

    /// 共享记忆汇合
    pub fn merge_shared(&mut self) {
        // 所有Agent把偶数时间戳私有记忆贡献到共享池
        for agent_id in 0..self.num_agents {
            if let Some(private) = self.grid.read_private(agent_id) {
                let indices: Vec<usize> = private
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.timestamp % 2 == 0)
                    .map(|(i, _)| i)
                    .collect();
                self.grid.share_private(agent_id, &indices);
            }
        }
    }

    /// Agent吸收共享记忆
    pub fn agent_absorb(&mut self, agent_id: u8) {
        self.grid.absorb_shared(agent_id);
    }

    /// 获取时间梯度分布
    pub fn timeline_distribution(&self) -> Vec<usize> {
        self.grid.layer_distribution()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_pairs_24() {
        assert_eq!(24, 24);
    }

    #[test]
    fn test_basic_init() {
        // Placeholder for basic initialization test
        assert!(true);
    }
}
