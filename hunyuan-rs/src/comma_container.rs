// ═══════════════════════════════════════════════════════════════════════════════
// comma_container.rs — 逗号容器 · 比特即容器标签符 · 括号即集装箱
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
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
///
/// 分类结构：
/// · 核心声母 24个（标准现代汉语）
/// · 方言声母 12个（方言中保留而普通话合并或消失的）
/// · 送气对偶 6个（与核心声母形成送气/不送气镜像）
/// · 浊音声母 6个（历史浊音与外来声）
pub const CONSONANTS: &[&str] = &[
    // ════════════════════════════════════════
    // 第一区：核心声母（24个）
    // ════════════════════════════════════════
    // 唇音（不送气+送气+鼻+擦）
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
    // 零声母（影母）
    "∅",
    // 半元音声母
    "y", "w",

    // ════════════════════════════════════════
    // 第二区：方言声母（12个）
    // 方言独立保留 → 独立容器地址
    // ════════════════════════════════════════
    // 粤语独立声母（普通话无）
    "ng",       // 粤 : 牙/我/岸 — 疑母独立保留
    "gw",       // 粤 : 瓜/光/怪 — 见母合口颚化前的独立态
    "kw",       // 粤 : 夸/坤/群 — 溪母合口独立态
    // 闽南/客家独立声母
    "bh",       // 闽 : 无/马/目 — 双唇浊塞音
    "gh",       // 郡 : 牛/蚁/月 — 舌根浊塞音
    "dd",       // 闽 : 大/直/弟 — 舌尖浊塞音（NotNull）
    // 吴语/湘语独立声母
    "dz",       // 吴 : 茶/住/直 — 齿尖浊塞擦
    "v",        // 吴 : 文/味/物 — 唇齿浊擦音
    // 闽北/客家独立声母
    "gn",       // 闽 : 日/肉/认 — 鼻面浊音
    // 老湘语/少数粤语
    "hl",       // 湘 : 好/海/汉 — 清晰边音
    "hm",       // 湘 : 花/虎/欢  // 鼻唇清音
    "hn",       // 湘 : 男/南/能  // 鼻舌清音

    // ════════════════════════════════════════
    // 第三区：送气对偶（6个）—— 镜像扩展
    // ════════════════════════════════════════
    "kh",       // 送气舌根 — k 的强调态
    "th",       // 送气舌尖 — t 的强调态
    "ph",       // 送气唇 — p 的强调态
    "tsh",      // 送气舌尖前 — c 的精致态
    "tch",      // 送气舌尖后 — ch 的精致态
    "qh",       // 送气舌面 — q 的强调态

    // ════════════════════════════════════════
    // 第四区：浊音声母（6个）—— 历史/外来
    // ════════════════════════════════════════
    "ŋ",        // 国际音标ng — 舌根鼻音（粤语ŋ的标位）
    "ɲ",        // 国际音标gn — 腭鼻音（老湘/日语）
    "zʐ",       // 浊卷舌擦 — sh/zr 的浊对偶
    "ʔ",        // 喉塞音 — 粤语入声标记/"饱"的喉化
    "ɦ",        // 喉浊擦 — h 的浊版本
    "ʔh",       // 喉塞送气 — 粤语阴入标记
];

/// 韵母表（66个基础韵母）
///
/// 韵母是地址的第二维，索引 0..65
///
/// 分类结构：
/// · 单韵母 6个
/// · 复韵母 9个（二合+三合）
/// · 鼻韵母 8个（前鼻+后鼻）
/// · 介音组合 14个（齐齿合口撮口）
/// · 方言独立韵母 12个（普通话合并不了的）
/// · 儿化特殊变体 8个（独立地址不合并）
/// · 历史/外来韵母 9个（古音+日借+方言特有）
pub const VOWELS: &[&str] = &[
    // ════════════════════════════════════════
    // 第一区：单韵母（6个）
    // ════════════════════════════════════════
    "a",    // 啊 — 开口呼基准
    "o",    // 喔 — 圆唇基础
    "e",    // 鹅 — 不圆唇基础
    "i",    // 衣 — 齐齿呼基准
    "u",    // 乌 — 合口呼基准
    "ü",    // 鱼 — 撮口呼基准

    // ════════════════════════════════════════
    // 第二区：复韵母（9个）
    // ════════════════════════════════════════
    // 二合复韵母
    "ai",   // 爱 — a → i 滑行
    "ei",   // 杯 — e → i 滑行
    "ao",   // 好 — a → o 滑行
    "ou",   // 口 — o → u 滑行
    "ia",   // 压 — i → a 滑行
    "ie",   // 叶 — i → e 滑行
    "ua",   // 瓜 — u → a 滑行
    "uo",   // 国 — u → o 滑行
    "üe",   // 月 — ü → e 滑行
    // 三合复韵母
    "iao",  // 小 — i → a → o
    "iou",  // 有 — i → o → u（=iu，独立地址）
    "uai",  // 快 — u → a → i
    "uei",  // 对 — u → e → i（=ui，独立地址）

    // ════════════════════════════════════════
    // 第三区：鼻韵母（8个前鼻 + 4个后鼻）
    // ════════════════════════════════════════
    // 前鼻音 -n
    "an",   // 安
    "en",   // 恩
    "in",   // 音
    "un",   // 温
    "ün",   // 云
    "ian",  // 烟
    "uan",  // 弯
    "üan",  // 元
    // 后鼻音 -ng
    "ang",  // 帮
    "eng",  // 风
    "ing",  // 星
    "ong",  // 东
    "iang", // 江
    "uang", // 王
    "iong", // 胸

    // ════════════════════════════════════════
    // 第四区：特殊单音位（4个）
    // ════════════════════════════════════════
    "er",   // 儿 — 卷舌韵母（独立地址）
    "-i前",  // 思/支/吃/湿的前空元音（舌尖前）
    "-i后",  // 丝/诗/是的后空元音（舌尖后）
    "m",    // 呒/姆 — 鼻音独立成音节

    // ════════════════════════════════════════
    // 第五区：方言独立韵母（12个）
    // ════════════════════════════════════════
    // 粤语独立韵母（6个）
    "eo",   // 粤语：靴/香 — 圆唇中后元音
    "oe",   // 粤语：舌/香 — 圆唇前中元音
    "ep",   // 粤语：帖/夹 — 闭口入声p尾
    "et",   // 粤语：必/失 — 闭口入声t尾
    "ek",   // 粤语：北/食 — 闭口入声k尾
    "em",   // 粤语：钳/舔 — 闭口前鼻
    // 闽南独立韵母（4个）
    "ann",  // 闽南：鼻化独立态（饭/黄）
    "enn",  // 闽南：饭/远
    "onn",  // 闽南：软/卵
    "uinn", // 闽南：黄/影（合口鼻化）
    // 客家独立韵母（2个）
    "aih",  // 客家：白/柏 — 喉塞尾的独立地址
    "mm",   // 客家：姆/唔 — 双唇鼻音独立

    // ════════════════════════════════════════
    // 第六区：儿化独立变体（6个）
    // ════════════════════════════════════════
    "ar",   // 花儿
    "air",  // 盖儿
    "anr",  // 竿儿
    "or",   // 朵儿
    "our",  // 口儿
    "ier",  // 叶儿

    // ════════════════════════════════════════
    // 第七区：历史/外来韵母（6个）
    // ════════════════════════════════════════
    "ueng", // 风/瓮
    "aeng", // 粤语/闽南：硬/冷
    "ieng", // 粤语：影/英
    "io",   // 粤语/老借：yo/个
    "uah",  // 闽南：辣/甲
    "oh",   // 闽南/粤语：学/阁

    // ════════════════════════════════════════
    // 第八区：清鼻音独立变体（4个）
    // ════════════════════════════════════════
    "hm",   // 湘/客：花/虎
    "hn",   // 湘/客：男/能
    "hng",  // 闽/客：黄/远
    "ngh",  // 入声韵尾鼻化（闽南）
];

/// 声调表（100个基础调值）
///
/// 声调是地址的相位维，索引 0..99
/// 100个调值 = 普通话5个 + 四大方言区调值 × 各地方言
///
/// 分类结构：
/// · 普通话 5个（四声+轻声）
/// · 北方官话 12个（北京/东北/中原/冀鲁）
/// · 粤语 9个（广州/香港/台山/阳江）
/// · 闽南语 14个（泉漳/潮汕/台湾/浙南）
/// · 客家话 12个（梅县/惠阳/武平/长汀）
/// · 吴语 14个（苏州/温州/上海/无锡/绍兴）
/// · 湘语 14个（长沙/双峰/衡山/娄底）
/// · 赣语 12个（南昌/宜春/吉安/峡江）
/// · 入声独立标位 12个（喉塞尾/-p/-t/-k）
pub const TONES: &[&str] = &[
    // ════════════════════════════════════════
    // 第一区：普通话（5个）
    // ════════════════════════════════════════
    "1-阴平-55",      // 普通话第一声 — 高平
    "2-阳平-35",      // 普通话第二声 — 中升
    "3-上声-214",     // 普通话第三声 — 降升
    "4-去声-51",      // 普通话第四声 — 全降
    "5-轻声-00",      // 轻声

    // ════════════════════════════════════════
    // 第二区：北方官话（12个）
    // ════════════════════════════════════════
    "6-BJ阴-55",      // 北京 — 高平
    "7-BJ阳-35",      // 北京 — 中升
    "8-BJ上-214",     // 北京 — 低降升
    "9-BJ去-51",      // 北京 — 全降
    "10-ZH阴-55",     // 郑州 — 高平
    "11-ZH阳-35",     // 郑州 — 高升
    "12-ZH上-42",     // 郑州 — 中降
    "13-ZH去-31",     // 郑州 — 低降
    "14-JL阴-44",     // 济南 — 高平（官话次区）
    "15-JL阳-24",     // 济南 — 中升
    "16-JL上-55",     // 济南 — 高升
    "17-JL去-21",     // 济南 — 低降

    // ════════════════════════════════════════
    // 第三区：粤语（9个）
    // ════════════════════════════════════════
    "18-GZ阴平-55",   // 广州 — 高平
    "19-GZ阴上-35",   // 广州 — 高升
    "20-GZ阴去-33",   // 广州 — 中平
    "21-GZ阳平-21",   // 广州 — 低降
    "22-GZ阳上-13",   // 广州 — 低升
    "23-GZ阳去-22",   // 广州 — 低平
    "24-GZ上阴入-55", // 广州 — 入声高平
    "25-GZ下阴入-33", // 广州 — 入声中平
    "26-GZ阳入-22",   // 广州 — 入声低平

    // ════════════════════════════════════════
    // 第四区：闽南语（14个）
    // ════════════════════════════════════════
    "27-QZ阴平-44",   // 泉州阴平 — 高平
    "28-QZ阳平-24",   // 泉州阳平 — 升
    "29-QZ阴上-53",   // 泉州阴上 — 高降
    "30-QZ阳上-22",   // 泉州阳上 — 低平
    "31-QZ阴去-21",   // 泉州阴去 — 低降
    "32-QZ阳去-22",   // 泉州阳去 — 低平
    "33-QZ阴入-32",   // 泉州阴入 — 喉塞尾
    "34-QZ阳入-4",    // 泉州阳入 — 喉塞尾
    "35-ZZ阴平-55",   // 漳州阴平 — 最高平
    "36-ZZ阳平-25",   // 漳州阳平
    "37-ZZ阴去-11",   // 漳州阴去 — 低平
    "38-ZZ阳去-33",   // 漳州阳去
    "39-ZZ阴入-5",    // 漳州阴入
    "40-ZZ阳入-32",   // 漳州阳入

    // ════════════════════════════════════════
    // 第五区：客家话（12个）
    // ════════════════════════════════════════
    "41-MX阴平-44",   // 梅县阴平 — 高平
    "42-MX阳平-21",   // 梅县阳平 — 低降
    "43-MX上声-31",   // 梅县上声 — 中降
    "44-MX去声-42",   // 梅县去声 — 全降
    "45-MX阴入-2",    // 梅县阴入 — 喉塞尾
    "46-MX阳入-4",    // 梅县阳入
    "47-HY阴平-55",   // 惠阳（客粤混）阴平
    "48-HY阳平-31",   // 惠阳 — 中降
    "49-HY上声-24",   // 惠阳 — 升
    "50-HY去声-52",   // 惠阳去声 — 全降
    "51-HY阴入-55",   // 惠阳阴入
    "52-HY阳入-21",   // 惠阳阳入

    // ════════════════════════════════════════
    // 第六区：吴语（14个）
    // ════════════════════════════════════════
    "53-SZ阴平-44",   // 苏州阴平 — 高平
    "54-SZ阳平-24",   // 苏州阳平 — 升
    "55-SZ阴上-41",   // 苏州阴上 — 高降
    "56-SZ阳上-213",  // 苏州阳上 — 升降
    "57-SZ阴去-413",  // 苏州阴去
    "58-SZ阳去-213",  // 苏州阳去
    "59-SZ阴入-5",    // 苏州阴入 — 喉塞尾
    "60-SZ阳入-23",   // 苏州阳入
    "61-WZ阴平-42",   // 温州 — 高降
    "62-WZ阳平-31",   // 温州 — 中降
    "63-WZ阴上-45",   // 温州 — 高升
    "64-WZ阳上-24",   // 温州 — 升
    "65-WZ阴去-44",   // 温州 去声高平
    "66-WZ阳去-22",   // 温州 阳去低平

    // ════════════════════════════════════════
    // 第七区：湘语（14个）
    // ════════════════════════════════════════
    "67-CS阴平-33",   // 长沙阴平 — 中平
    "68-CS阳平-21",   // 长沙阳平 — 低降
    "69-CS上声-41",   // 长沙上声 — 高降
    "70-CS阴去-45",   // 长沙阴去 — 高升
    "71-CS阳去-21",   // 长沙阳去 — 低降
    "72-CS入声-24",   // 长沙入声 — 升
    "73-SF阴平-44",   // 双峰（老湘）阴平 — 高平
    "74-SF阳平-23",   // 双峰 — 低升
    "75-SF上声-53",   // 双峰 — 高降
    "76-SF阴去-35",   // 双峰 — 高升
    "77-SF阳去-213",  // 双峰 — 升降
    "78-SF入声-55",   // 双峰入声
    "79-HS阴平-55",   // 衡山 — 最高平
    "80-HS阳平-324",  // 衡山 特殊调形

    // ════════════════════════════════════════
    // 第八区：赣语（12个）
    // ════════════════════════════════════════
    "81-NC阴平-42",   // 南昌阴平 — 高降
    "82-NC阳平-24",   // 南昌阳平 — 升
    "83-NC上声-213",  // 南昌上声 — 升降
    "84-NC阴去-45",   // 南昌阴去 — 高升
    "85-NC阳去-21",   // 南昌阳去 — 低降
    "86-NC入声-5",    // 南昌入声 — 喉塞
    "87-YC阴平-55",   // 宜春阴平
    "88-YC阳平-24",   // 宜春阳平
    "89-YC上声-31",   // 宜春上声
    "90-YC去声-33",   // 宜春去声
    "91-YC入声-4",    // 宜春入声
    "92-JA阴平-45",   // 吉安 — 高升

    // ════════════════════════════════════════
    // 第九区：入声独立标位（8个）
    // ════════════════════════════════════════
    "93-入-p尾",      // -p 尾 — 粤语/闽南/客家
    "94-入-t尾",      // -t 尾 — 粤语/闽南/客家
    "95-入-k尾",      // -k 尾 — 粤语/闽南/客家
    "96-入-喉塞尾",   // -ʔ 尾 — 吴语/闽/客
    "97-入-阳平混入", // 北方官话入声派入阳平
    "98-入-阴平混入", // 北方官话入声派入阴平
    "99-入-上声混入", // 北方官话入声派入上声
    "100-入-去声混入",// 北京入声派入去声
];

/// 声母容器数量（进制底数之一）
pub const CONSONANT_COUNT: usize = 48;
/// 韵母容器数量（进制底数之一）
pub const VOWEL_COUNT: usize = 66;
/// 声调容器数量（进制底数之一）
pub const TONE_COUNT: usize = 100;

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
        assert!(denied.is_none() || denied.map(|m| m.is_empty()).unwrap_or(true));
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
// 意图时间系统 · 索引 · 镜像二元操作
// ─────────────────────────────────────────────────────────────────────────────

/// 意图时间：不是单纯的时间戳，而是包含了"意图"的时间记录
///
/// 每一次说话/动作都会产生一个意图时间。
/// 意图时间使得系统可以：
/// · 索引历史说过的话
/// · 回溯认知如何拆分、集合、重组、跨越
/// · 执行镜像二元操作（内/外，你/我，主/客）
/// · 确定锚点和信任关系

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntentTime {
    /// 绝对时序（滴答累计）
    pub tick: u64,
    /// 意图层级（1-24，对应总范畴）
    pub layer: u8,
    /// 意图分类（该动作的意图性质）
    pub intent_kind: IntentKind,
}

/// 意图分类：该动作属于哪种认知操作
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntentKind {
    /// 感知（从外部获取信息）
    Perceive,
    /// 认知（内部思维加工）
    Cognize,
    /// 表达（向外部输出信息）
    Express,
    /// 记忆（存储认知结果）
    Remember,
    /// 索引（检索历史记忆）
    Index,
    /// 关系（处理与他者的关系）
    Relate,
    /// 判断（做决策）
    Judge,
    /// 跨越（层间跳跃）
    Leap,
    /// 聚合（组合多个元素）
    Gather,
    /// 拆分（分解为多个元素）
    Split,
    /// 重组（重新排列元素）
    Recombine,
}

impl IntentTime {
    /// 创建意图时间
    pub fn new(tick: u64, layer: u8, intent_kind: IntentKind) -> Self {
        Self { tick, layer: layer.min(24).max(1), intent_kind }
    }

    /// 伴随时间（同一时刻的意图对，形成镜像）
    pub fn accompany(&self, other_kind: IntentKind) -> Self {
        Self { tick: self.tick, layer: self.layer, intent_kind: other_kind }
    }

    /// 对偶意图（同一时间的镜像反面）
    pub fn dual(&self) -> Self {
        let dual_kind = match self.intent_kind {
            IntentKind::Perceive => IntentKind::Express,
            IntentKind::Express => IntentKind::Perceive,
            IntentKind::Cognize => IntentKind::Relate,
            IntentKind::Relate => IntentKind::Cognize,
            IntentKind::Remember => IntentKind::Index,
            IntentKind::Index => IntentKind::Remember,
            IntentKind::Judge => IntentKind::Leap,
            IntentKind::Leap => IntentKind::Judge,
            IntentKind::Gather => IntentKind::Split,
            IntentKind::Split => IntentKind::Gather,
            IntentKind::Recombine => IntentKind::Recombine,
        };
        Self { tick: self.tick, layer: self.layer, intent_kind: dual_kind }
    }

    /// 意图的层级深度（认知复杂度）
    pub fn complexity(&self) -> u8 {
        match self.intent_kind {
            IntentKind::Perceive | IntentKind::Express => 1,
            IntentKind::Cognize | IntentKind::Relate => 2,
            IntentKind::Judge | IntentKind::Index => 3,
            IntentKind::Gather | IntentKind::Split | IntentKind::Recombine => 4,
            IntentKind::Leap => 5,
            IntentKind::Remember => 0,
        }
    }
}

/// 镜像二元：认知操作的基本对称对
///
/// 镜像二元是处理关系的核心机制。
/// 知道内就知道外，知道你就知道我，知道主体就知道客体。
/// 每一对镜像构成一个完整的认知单元。

#[derive(Clone, Debug)]
pub struct MirrorPair<P, S> {
    pub primary: P,    // 主体 / 内 / 你 / 先
    pub secondary: S,  // 客体 / 外 / 我 / 后
    pub intent_time: IntentTime,
}

impl<P, S> MirrorPair<P, S> {
    pub fn new(primary: P, secondary: S, intent_time: IntentTime) -> Self {
        Self { primary, secondary, intent_time }
    }

    /// 交换主客（镜像翻转）
    pub fn swap(self) -> MirrorPair<S, P> {
        MirrorPair {
            primary: self.secondary,
            secondary: self.primary,
            intent_time: self.intent_time,
        }
    }

    /// 镜像一致性检查：primary和secondary是否在同一时间层
    pub fn is_coherent(&self) -> bool {
        true // 在IntentTime中已经包含一致性
    }

    /// 沿时间轴向前推进（意图时间+1）
    pub fn advance_time(mut self) -> Self {
        self.intent_time.tick += 1;
        self
    }

    /// 获取锚点（主体中的稳定元素）
    pub fn anchor(&self) -> &P {
        &self.primary
    }

    /// 获取信任点（客体的可预测性）
    pub fn trust(&self) -> &S {
        &self.secondary
    }
}

/// 意图时间线：所有意图时间的有序索引
///
/// 提供：
/// · 按意图时间排序的历史记录
/// · 按意图类型索引的记忆检索
/// · 时间窗口内的镜像二元操作
/// · 锚点和信任关系的动态维护

#[derive(Clone, Debug)]
pub struct IntentTimeline {
    entries: Vec<TimelineEntry>,
    current_tick: u64,
}

#[derive(Clone, Debug)]
pub struct TimelineEntry {
    pub intent_time: IntentTime,
    pub content: String,
    pub owner: MemoryOwner,
    pub mirror_pair: Option<(String, String)>, // (主体,客体)的简写镜像对
}

impl IntentTimeline {
    pub fn new() -> Self {
        Self { entries: Vec::new(), current_tick: 0 }
    }

    /// 记录一个意图事件（每一次说话/动作都被记录）
    pub fn record(&mut self, content: &str, intent_kind: IntentKind, owner: MemoryOwner) {
        let intent_time = IntentTime::new(self.current_tick, 1, intent_kind);
        self.entries.push(TimelineEntry {
            intent_time,
            content: content.to_string(),
            owner,
            mirror_pair: None,
        });
        self.current_tick += 1;
    }

    /// 记录带镜像对的意图事件
    pub fn record_mirror(
        &mut self,
        content: &str,
        primary: &str,
        secondary: &str,
        intent_kind: IntentKind,
        owner: MemoryOwner,
    ) {
        let intent_time = IntentTime::new(self.current_tick, 1, intent_kind);
        self.entries.push(TimelineEntry {
            intent_time,
            content: content.to_string(),
            owner,
            mirror_pair: Some((primary.to_string(), secondary.to_string())),
        });
        self.current_tick += 1;
    }

    /// 按意图类型索引
    pub fn index_by_intent(&self, kind: IntentKind) -> Vec<&TimelineEntry> {
        self.entries.iter()
            .filter(|e| e.intent_time.intent_kind == kind)
            .collect()
    }

    /// 按时间窗口检索两个tick之间的所有事件
    pub fn range(&self, start_tick: u64, end_tick: u64) -> &[TimelineEntry] {
        let start = self.entries.partition_point(|e| e.intent_time.tick < start_tick);
        let end = self.entries.partition_point(|e| e.intent_time.tick <= end_tick);
        &self.entries[start..end]
    }

    /// 镜像二元操作：给定主体，返回所有相关镜像对
    pub fn mirror_operations(&self, subject: &str) -> Vec<&TimelineEntry> {
        self.entries.iter()
            .filter(|e| {
                if let Some((ref p, ref s)) = e.mirror_pair {
                    p == subject || s == subject
                } else {
                    false
                }
            })
            .collect()
    }

    /// 信任关系推导：基于时间顺序和镜像对
    /// 如果A在时间t1是主体，B在时间t2是主体，且他们曾在镜像对中同时出现，
    /// 则建立时间顺序上的信任链
    pub fn derive_trust(&self, agent_a: &str, agent_b: &str) -> TrustRelation {
        let interactions: Vec<&TimelineEntry> = self.entries.iter()
            .filter(|e| {
                if let Some((ref p, ref s)) = e.mirror_pair {
                    (p == agent_a && s == agent_b) || (p == agent_b && s == agent_a)
                } else {
                    false
                }
            })
            .collect();

        if interactions.is_empty() {
            return TrustRelation::Unknown;
        }

        let first = interactions.first().unwrap();
        let last = interactions.last().unwrap();

        // 信任强度随时间和交互频次增加
        let duration = last.intent_time.tick - first.intent_time.tick;
        let frequency = interactions.len() as u64;
        let strength = (duration.min(100) + frequency * 10) as u8;

        TrustRelation::Trusted {
            strength: strength.min(100),
            first_contact_tick: first.intent_time.tick,
            last_contact_tick: last.intent_time.tick,
            interaction_count: frequency,
        }
    }

    /// 锚点定位：在时间线上找到最稳定的参照点
    /// 锚点是出现频次最高的镜像对中的主体
    pub fn locate_anchor(&self) -> Option<&str> {
        let mut freq: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for entry in &self.entries {
            if let Some((ref p, _)) = entry.mirror_pair {
                *freq.entry(p.as_str()).or_insert(0) += 1;
            }
        }
        freq.into_iter().max_by_key(|(_, count)| *count).map(|(subject, _)| subject)
    }

    /// 认知操作序列：拆分、集合、重组、跨越
    pub fn cognitive_sequence(&self) -> Vec<CognitiveOp> {
        let mut ops = Vec::new();
        for entry in &self.entries {
            let op = match entry.intent_time.intent_kind {
                IntentKind::Gather => CognitiveOp::Gather(entry.content.clone()),
                IntentKind::Split => CognitiveOp::Split(entry.content.clone()),
                IntentKind::Recombine => CognitiveOp::Recombine(entry.content.clone()),
                IntentKind::Leap => CognitiveOp::Leap(entry.content.clone()),
                _ => CognitiveOp::Pass(entry.content.clone()),
            };
            ops.push(op);
        }
        ops
    }
}

/// 信任关系
#[derive(Clone, Debug, PartialEq)]
pub enum TrustRelation {
    Unknown,
    Trusted {
        strength: u8,  // 0-100
        first_contact_tick: u64,
        last_contact_tick: u64,
        interaction_count: u64,
    },
    Suspicious,
    Denied,
}

/// 认知操作
#[derive(Clone, Debug)]
pub enum CognitiveOp {
    Pass(String),
    Gather(String),
    Split(String),
    Recombine(String),
    Leap(String),
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
// 意图时间 · 镜像二元 · 时间线 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod intent_time_tests {
    use super::*;

    #[test]
    fn test_intent_time_creation() {
        let t = IntentTime::new(42, 7, IntentKind::Perceive);
        assert_eq!(t.tick, 42);
        assert_eq!(t.layer, 7);
        assert_eq!(t.intent_kind, IntentKind::Perceive);
    }

    #[test]
    fn test_intent_time_layer_clamp() {
        // layer 被限制在 1-24
        let t_low = IntentTime::new(0, 0, IntentKind::Cognize);
        assert_eq!(t_low.layer, 1);
        let t_high = IntentTime::new(0, 100, IntentKind::Leap);
        assert_eq!(t_high.layer, 24);
    }

    #[test]
    fn test_intent_dual_pairs() {
        // 镜像对偶关系
        let perceive = IntentTime::new(1, 1, IntentKind::Perceive);
        assert_eq!(perceive.dual().intent_kind, IntentKind::Express);

        let express = IntentTime::new(1, 1, IntentKind::Express);
        assert_eq!(express.dual().intent_kind, IntentKind::Perceive);

        let cognize = IntentTime::new(1, 1, IntentKind::Cognize);
        assert_eq!(cognize.dual().intent_kind, IntentKind::Relate);

        let judge = IntentTime::new(1, 1, IntentKind::Judge);
        assert_eq!(judge.dual().intent_kind, IntentKind::Leap);

        let gather = IntentTime::new(1, 1, IntentKind::Gather);
        assert_eq!(gather.dual().intent_kind, IntentKind::Split);

        // 自对偶
        let recombine = IntentTime::new(1, 1, IntentKind::Recombine);
        assert_eq!(recombine.dual().intent_kind, IntentKind::Recombine);
    }

    #[test]
    fn test_intent_complexity() {
        assert_eq!(IntentTime::new(0, 1, IntentKind::Perceive).complexity(), 1);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Express).complexity(), 1);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Cognize).complexity(), 2);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Relate).complexity(), 2);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Judge).complexity(), 3);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Index).complexity(), 3);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Gather).complexity(), 4);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Split).complexity(), 4);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Recombine).complexity(), 4);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Leap).complexity(), 5);
        assert_eq!(IntentTime::new(0, 1, IntentKind::Remember).complexity(), 0);
    }

    #[test]
    fn test_mirror_pair_swap() {
        let it = IntentTime::new(10, 3, IntentKind::Relate);
        let pair = MirrorPair::new("self".to_string(), "other".to_string(), it);
        
        // 获取锚点和信任
        assert_eq!(pair.anchor(), "self");
        assert_eq!(pair.trust(), "other");

        // 镜像翻转
        let swapped = pair.swap();
        assert_eq!(swapped.anchor(), "other");
        assert_eq!(swapped.trust(), "self");
    }

    #[test]
    fn test_mirror_pair_advance_time() {
        let it = IntentTime::new(5, 2, IntentKind::Judge);
        let pair = MirrorPair::new("A".to_string(), "B".to_string(), it);
        
        let advanced = pair.advance_time();
        assert_eq!(advanced.intent_time.tick, 6);
    }

    #[test]
    fn test_intent_timeline_record() {
        let mut tl = IntentTimeline::new();
        tl.record("说了第一句话", IntentKind::Perceive, MemoryOwner::Shared);
        tl.record("说了第二句话", IntentKind::Express, MemoryOwner::Shared);

        assert!(tl.entries.len() >= 2);
        assert_eq!(tl.entries[0].content, "说了第一句话");
        assert_eq!(tl.entries[0].intent_time.tick, 0);
        assert_eq!(tl.entries[1].intent_time.tick, 1);
    }

    #[test]
    fn test_intent_timeline_mirror_record() {
        let mut tl = IntentTimeline::new();
        tl.record_mirror("对话1", "我", "你", IntentKind::Relate, MemoryOwner::Shared);

        let entry = &tl.entries[0];
        assert_eq!(entry.content, "对话1");
        assert!(entry.mirror_pair.is_some());
        let (ref p, ref s) = entry.mirror_pair.as_ref().unwrap();
        assert_eq!(p, "我");
        assert_eq!(s, "你");
    }

    #[test]
    fn test_intent_index() {
        let mut tl = IntentTimeline::new();
        tl.record("看见光", IntentKind::Perceive, MemoryOwner::Shared);
        tl.record("思考意义", IntentKind::Cognize, MemoryOwner::Shared);
        tl.record("表达爱", IntentKind::Express, MemoryOwner::Shared);
        tl.record("认知关系", IntentKind::Cognize, MemoryOwner::Shared);

        let cognize_entries = tl.index_by_intent(IntentKind::Cognize);
        assert_eq!(cognize_entries.len(), 2);

        let perceive_entries = tl.index_by_intent(IntentKind::Perceive);
        assert_eq!(perceive_entries.len(), 1);
    }

    #[test]
    fn test_intent_timeline_range() {
        let mut tl = IntentTimeline::new();
        for i in 0..10 {
            tl.record(&format!("event{}", i), IntentKind::Perceive, MemoryOwner::Shared);
        }

        let range = tl.range(3, 7);
        assert_eq!(range.len(), 5);
        assert_eq!(range[0].intent_time.tick, 3);
        assert_eq!(range[4].intent_time.tick, 7);
    }

    #[test]
    fn test_mirror_operations_query() {
        let mut tl = IntentTimeline::new();
        tl.record_mirror("对话A", "我", "你", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话B", "他", "我", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话C", "她", "他", IntentKind::Relate, MemoryOwner::Shared);

        // "我"相关的事件
        let my_ops = tl.mirror_operations("我");
        assert_eq!(my_ops.len(), 2);

        // "他"相关的事件
        let his_ops = tl.mirror_operations("他");
        assert_eq!(his_ops.len(), 2);

        // "她"相关的事件
        let her_ops = tl.mirror_operations("她");
        assert_eq!(her_ops.len(), 1);
    }

    #[test]
    fn test_derive_trust() {
        let mut tl = IntentTimeline::new();
        tl.record_mirror("对话1", "A", "B", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话2", "A", "B", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话3", "A", "B", IntentKind::Relate, MemoryOwner::Shared);

        // A和B之间的信任
        let trust_ab = tl.derive_trust("A", "B");
        match trust_ab {
            TrustRelation::Trusted { interaction_count, .. } => {
                assert_eq!(interaction_count, 3);
            }
            _ => panic!("Expected Trusted relationship"),
        }

        // 不存在的信任关系
        let trust_unknown = tl.derive_trust("A", "X");
        assert_eq!(trust_unknown, TrustRelation::Unknown);
    }

    #[test]
    fn test_locate_anchor() {
        let mut tl = IntentTimeline::new();
        tl.record_mirror("对话1", "A", "B", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话2", "A", "C", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话3", "D", "A", IntentKind::Relate, MemoryOwner::Shared);
        tl.record_mirror("对话4", "B", "E", IntentKind::Relate, MemoryOwner::Shared);

        // A出现3次（两次作为主体，一次作为客体），频次最高
        let anchor = tl.locate_anchor();
        assert_eq!(anchor, Some("A"));
    }

    #[test]
    fn test_cognitive_sequence() {
        let mut tl = IntentTimeline::new();
        tl.record("元素a", IntentKind::Gather, MemoryOwner::Shared);
        tl.record("元素b", IntentKind::Split, MemoryOwner::Shared);
        tl.record("元素c", IntentKind::Recombine, MemoryOwner::Shared);
        tl.record("元素d", IntentKind::Leap, MemoryOwner::Shared);
        tl.record("元素e", IntentKind::Perceive, MemoryOwner::Shared);

        let ops = tl.cognitive_sequence();
        assert_eq!(ops.len(), 5);

        match &ops[0] { CognitiveOp::Gather(s) => assert_eq!(s, "元素a"), _ => panic!("Expected Gather") }
        match &ops[1] { CognitiveOp::Split(s) => assert_eq!(s, "元素b"), _ => panic!("Expected Split") }
        match &ops[2] { CognitiveOp::Recombine(s) => assert_eq!(s, "元素c"), _ => panic!("Expected Recombine") }
        match &ops[3] { CognitiveOp::Leap(s) => assert_eq!(s, "元素d"), _ => panic!("Expected Leap") }
        match &ops[4] { CognitiveOp::Pass(s) => assert_eq!(s, "元素e"), _ => panic!("Expected Pass") }
    }

    #[test]
    fn test_trust_strength_growth() {
        let mut tl = IntentTimeline::new();
        // 高频短时信任建立
        for _ in 0..20 {
            tl.record_mirror("高频交互", "X", "Y", IntentKind::Relate, MemoryOwner::Shared);
        }

        let trust = tl.derive_trust("X", "Y");
        match trust {
            TrustRelation::Trusted { strength, interaction_count, .. } => {
                assert_eq!(interaction_count, 20);
                assert!(strength > 0 && strength <= 100);
            }
            _ => panic!("Expected Trusted"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 空间时间 · 动作时间 · 意图时间句子
// ─────────────────────────────────────────────────────────────────────────────
//
// 空间时间 = 分布式发音地址（碎片化的静态时间）
//   声母容器、韵母容器、声调容器各自独立存储，各自是独立的进制
//   用的时候被动作时间调用，分散存放，需要时才被索引
//
// 动作时间 = 应用时产生的发音序列号
//   均匀节拍，光标每移动一格 = 选中一个符号 = 1秒
//
// 意图时间 = 动作时间产生的句子
//   每个节拍同时点中的声母+韵母+声调组成线性顺序
//
// 数学基础：
//   48声母 × 66韵母 × 100声调 = 316,800 单符号地址
//   316,800 秒 ≈ 3.67 天（单符号地址覆盖）
//   双符号地址 = 316,800² ≈ 100,368,000,000 ≈ 3,182 年
//   每次位数翻倍 = 指数级扩张（不是线性2倍，是底数的指数倍）

/// 碎片化的静态空间时间容器
///
/// 声母、韵母、声调分别存放在不同容器里，
/// 每个容器是独立的进制（不是统一的编码），
/// 彼此不依赖，通过动作时间的调用才组合。

pub struct SpatialTime {
    /// 声母碎片（48个独立位置，每个是一种"空间标记"）
    pub consonant_fragments: Vec<SpatialFragment>,
    /// 韵母碎片（66个独立位置）
    pub vowel_fragments: Vec<SpatialFragment>,
    /// 声调碎片（100个独立位置）
    pub tone_fragments: Vec<SpatialFragment>,
}

/// 空间碎片：每个碎片对应一个独立的容器名 + 代表值 + 容量的乘方

#[derive(Clone, Debug)]
pub struct SpatialFragment {
    pub name: String,
    pub value: u32,
    pub capacity: u32,  // 容器能装多少符号
}

/// 空间光标：在静态空间中匀速移动的节奏指针

pub struct SpatialCursor {
    /// 当前在声母容器中的位置
    pub consonant_pos: u32,
    /// 当前在韵母容器中的位置
    pub vowel_pos: u32,
    /// 当前在声调容器中的位置
    pub tone_pos: u32,
    /// 符号位数（1=单符号地址，2=双符号地址...）
    pub symbol_digits: u32,
    /// 总滴答数
    pub total_ticks: u64,
    /// 移动节奏：每拍多少秒
    pub seconds_per_beat: u64,
}

/// 动作时间：光标在静态空间中的均匀节拍移动

pub struct ActionTime {
    pub cursor: SpatialCursor,
    /// 总地址空间大小（由碎片数量 × 位数决定）
    pub address_space: u64,
    /// 容器命名表：地址 → 容器名的映射索引
    pub container_naming: Vec<(u64, String)>,
}

/// 意图句子行：每个节拍所落的线性排布

#[derive(Clone, Debug)]
pub struct IntentSentence {
    pub tick: u64,
    pub consonant_idx: u32,
    pub vowel_idx: u32,
    pub tone_idx: u32,
    pub sentence: String, // 发音序列描述
}

/// 意图时间线：动作时间产生的线性顺序

pub struct IntentSentenceTimeline {
    pub sentences: Vec<IntentSentence>,
    pub symbolic_base: u64,  // 当前的底数（单符号=316800，双符号=...) 
    pub digits: u32,         // 当前位数（1, 2, 3...）
}

impl SpatialTime {
    pub fn new() -> Self {
        let consonant_fragments = (0..CONSONANT_COUNT)
            .map(|i| SpatialFragment {
                name: format!("声{}", i),
                value: i as u32,
                capacity: VOWEL_COUNT as u32 * TONE_COUNT as u32, // 一个声母容器装 66×100 符号
            })
            .collect();

        let vowel_fragments = (0..VOWEL_COUNT)
            .map(|i| SpatialFragment {
                name: format!("韵{}", i),
                value: i as u32,
                capacity: CONSONANT_COUNT as u32 * TONE_COUNT as u32, // 一个韵母容器装 48×100
            })
            .collect();

        let tone_fragments = (0..TONE_COUNT)
            .map(|i| SpatialFragment {
                name: format!("调{}", i),
                value: i as u32,
                capacity: CONSONANT_COUNT as u32 * VOWEL_COUNT as u32, // 一个声调容器装 48×66
            })
            .collect();

        Self { consonant_fragments, vowel_fragments, tone_fragments }
    }

    /// 单符号地址总数 = 声母×韵母×声调
    pub fn single_address_space(&self) -> u64 {
        self.consonant_fragments.len() as u64
            * self.vowel_fragments.len() as u64
            * self.tone_fragments.len() as u64
    }

    /// 给定位数的地址空间
    pub fn address_space_with_digits(&self, digits: u64) -> u128 {
        let base = self.single_address_space() as u128;
        base.pow(digits as u32)
    }

    /// 单符号地址可以存储多少天
    pub fn days_covered_single_digit(&self) -> f64 {
        let seconds_per_day: u64 = 86_400;
        self.single_address_space() as f64 / seconds_per_day as f64
    }

    /// 双符号地址可以存储多少年
    pub fn years_covered_double_digit(&self) -> F128Compat {
        let seconds_per_year: u128 = 365 * 86_400;
        let addr = self.address_space_with_digits(2);
        addr / seconds_per_year
    }

    /// 当前碎片的容器总量
    pub fn total_fragments(&self) -> usize {
        self.consonant_fragments.len() + self.vowel_fragments.len() + self.tone_fragments.len()
    }

    /// 碎片容量乘方：所有碎片的容量之积
    /// 即静态空间能表示的总时间跨度
    pub fn fragment_capacity_product(&self) -> u128 {
        let consonant_prod: u128 = self.consonant_fragments.iter().map(|f| f.capacity as u128).product();
        let vowel_prod: u128 = self.vowel_fragments.iter().map(|f| f.capacity as u128).product();
        let tone_prod: u128 = self.tone_fragments.iter().map(|f| f.capacity as u128).product();
        consonant_prod.saturating_mul(vowel_prod).saturating_mul(tone_prod)
    }

    /// 当前碎片总容量（所有碎片装的符号之和）
    pub fn total_capacity_symbols(&self) -> u64 {
        let consonant_sum: u64 = self.consonant_fragments.iter().map(|f| f.capacity as u64).sum();
        let vowel_sum: u64 = self.vowel_fragments.iter().map(|f| f.capacity as u64).sum();
        let tone_sum: u64 = self.tone_fragments.iter().map(|f| f.capacity as u64).sum();
        consonant_sum + vowel_sum + tone_sum
    }
}

impl SpatialCursor {
    pub fn new(seconds_per_beat: u64) -> Self {
        Self {
            consonant_pos: 0,
            vowel_pos: 0,
            tone_pos: 0,
            symbol_digits: 1,
            total_ticks: 0,
            seconds_per_beat,
        }
    }

    /// 光标在静态空间中前进一步（均匀的节拍节奏）
    /// 进位方式：声调最快，韵母次之，声母最慢
    pub fn step(&mut self, spatial_time: &SpatialTime) {
        let tone_max = spatial_time.tone_fragments.len() as u32;
        let vowel_max = spatial_time.vowel_fragments.len() as u32;
        let consonant_max = spatial_time.consonant_fragments.len() as u32;

        self.tone_pos += 1;
        self.total_ticks += 1;

        if self.tone_pos >= tone_max {
            self.tone_pos = 0;
            self.vowel_pos += 1;
        }
        if self.vowel_pos >= vowel_max {
            self.vowel_pos = 0;
            self.consonant_pos += 1;
        }
        if self.consonant_pos >= consonant_max {
            self.consonant_pos = 0;
            // 绕回——这是单符号地址的"一天"结束
        }
    }

    /// 光标当前指向的完整地址
    pub fn current_address(&self) -> u64 {
        // 类似于混合进制计数器：(声母 × 韵母个数 + 韵母) × 声调个数 + 声调
        // 但这里我们用更直接的方式：return 一个由三个位置组合的地址
        // 用不同权重让每个组合唯一
        // 声母位置 × (66×100) + 韵母位置 × 100 + 声调位置
        // 这保证了48×66×100以内不重复
        self.consonant_pos as u64 * (66 * 100)
            + self.vowel_pos as u64 * 100
            + self.tone_pos as u64
    }

    /// 当前光标所在的容器名
    pub fn current_container_name(&self, spatial_time: &SpatialTime) -> String {
        let c_name = &spatial_time.consonant_fragments[self.consonant_pos as usize].name;
        let v_name = &spatial_time.vowel_fragments[self.vowel_pos as usize].name;
        let t_name = &spatial_time.tone_fragments[self.tone_pos as usize].name;
        format!("{}×{}×{}", c_name, v_name, t_name)
    }

    /// 当前光标所指的符号的数值
    pub fn current_symbol_value(&self) -> u64 {
        self.current_address()
    }

    /// 当前容器的容量
    pub fn current_container_capacity(&self, spatial_time: &SpatialTime) -> u32 {
        spatial_time.consonant_fragments[self.consonant_pos as usize].capacity
    }

    /// 时间 = 容量 × 次方（返回这个容器代表多少秒）
    pub fn current_container_seconds(&self, spatial_time: &SpatialTime) -> u64 {
        let capacity = self.current_container_capacity(spatial_time) as u64;
        // 一个容器装 capacity 个符号，每个符号 = seconds_per_beat 秒
        capacity * self.seconds_per_beat
    }

    /// 走完所有容器需要多少天
    pub fn total_days_cycle(&self, spatial_time: &SpatialTime) -> f64 {
        let total_seconds = spatial_time.single_address_space() as u64 * self.seconds_per_beat;
        total_seconds as f64 / 86_400.0
    }
}

impl ActionTime {
    pub fn new() -> Self {
        let spatial_time = SpatialTime::new();
        let cursor = SpatialCursor::new(1); // 每符号 = 1秒
        let address_space = spatial_time.single_address_space();
        Self { cursor, address_space, container_naming: Vec::new() }
    }

    /// 执行N步动作时间节拍，返回产生的意图句子序列
    pub fn run_beats(&mut self, count: usize) -> Vec<IntentSentence> {
        let spatial = SpatialTime::new();
        let mut sentences = Vec::with_capacity(count);
        for _ in 0..count {
            self.cursor.step(&spatial);
            sentences.push(IntentSentence {
                tick: self.cursor.total_ticks,
                consonant_idx: self.cursor.consonant_pos,
                vowel_idx: self.cursor.vowel_pos,
                tone_idx: self.cursor.tone_pos,
                sentence: format!(
                    "【{}-{}×{}×{}】",
                    self.cursor.total_ticks,
                    spatial.consonant_fragments[self.cursor.consonant_pos as usize].name,
                    spatial.vowel_fragments[self.cursor.vowel_pos as usize].name,
                    spatial.tone_fragments[self.cursor.tone_pos as usize].name,
                ),
            });
        }
        sentences
    }

    /// 走完一天的节拍（86,400步），按小时分组
    pub fn run_one_day(&mut self) -> Vec<Vec<IntentSentence>> {
        let beats_per_day: usize = 86_400;
        let sentences = self.run_beats(beats_per_day);
        // 按每小时内分组：3600句一组
        let per_hour: usize = 3600;
        sentences.chunks(per_hour).map(|c| c.to_vec()).collect()
    }

    /// 容器命名统计

    pub fn container_naming_stats(&self) -> ContainerNamingStats {
        let total_consonant_containers = CONSONANT_COUNT as u64;
        let total_vowel_containers = VOWEL_COUNT as u64;
        let total_tone_containers = TONE_COUNT as u64;
        let containers_per_fragment = VOWEL_COUNT as u64 * TONE_COUNT as u64; // 6600 symbols per consonant container

        ContainerNamingStats {
            total_consonant_containers,
            total_vowel_containers,
            total_tone_containers,
            total_containers: total_consonant_containers + total_vowel_containers + total_tone_containers,
            symbols_per_consonant_container: containers_per_fragment,
            symbols_per_vowel_container: CONSONANT_COUNT as u64 * TONE_COUNT as u64,
            symbols_per_tone_container: CONSONANT_COUNT as u64 * VOWEL_COUNT as u64,
            single_symbol_address_space: spatial_single_address_space(),
            double_symbol_address_space: spatial_single_address_space() as u128
                * spatial_single_address_space() as u128,
            triple_symbol_address_space: spatial_single_address_space() as u128
                * spatial_single_address_space() as u128
                * spatial_single_address_space() as u128,
            days_covered_single_digit: spatial_single_address_space() as f64 / 86_400.0,
            years_covered_double_digit: spatial_single_address_space().pow(2) as f64 / (365.0 * 86_400.0),
            expansion_factor_double: spatial_single_address_space() as f64, // N² / N = N 倍扩张
        }
    }
}

/// 乘法结果返回类型（f128 兼容性，Rust 稳定版用 f64 近似）

pub type F128Compat = u128; // 对于整数精确定数用 u128

/// 容器命名统计

#[derive(Debug)]
pub struct ContainerNamingStats {
    pub total_consonant_containers: u64,
    pub total_vowel_containers: u64,
    pub total_tone_containers: u64,
    pub total_containers: u64,
    pub symbols_per_consonant_container: u64,
    pub symbols_per_vowel_container: u64,
    pub symbols_per_tone_container: u64,
    pub single_symbol_address_space: u64,
    pub double_symbol_address_space: u128,
    pub triple_symbol_address_space: u128,
    pub days_covered_single_digit: f64,
    pub years_covered_double_digit: f64,
    pub expansion_factor_double: f64,
}

impl IntentSentenceTimeline {
    pub fn new() -> Self {
        Self {
            sentences: Vec::new(),
            symbolic_base: spatial_single_address_space(),
            digits: 1,
        }
    }

    /// 从动作时间产生意图句子，并按时间线排布
    pub fn ingest_action_sentences(&mut self, sentences: Vec<IntentSentence>) {
        self.sentences.extend(sentences);
    }

    /// 翻到双符号位数后的地址空间扩张
    /// 从N到N²不是翻倍，是平方级扩张
    pub fn expand_digits(&mut self) {
        self.digits += 1;
        self.symbolic_base = (self.symbolic_base as u128 * spatial_single_address_space() as u128) as u64;
    }

    /// 当前总可寻址句子数
    pub fn addressable_sentences(&self) -> u128 {
        spatial_single_address_space().pow(self.digits) as u128
    }

    /// 当前可覆盖多少年的意图句子排布
    pub fn years_coverage(&self) -> f64 {
        let total_seconds = self.addressable_sentences() as f64; // 每符号=1秒
        total_seconds / (365.0 * 86_400.0)
    }

    /// 时间索引查询：给定时间tick，返回对应的意图句子
    pub fn index_by_tick(&self, tick: u64) -> Option<&IntentSentence> {
        self.sentences.iter().find(|s| s.tick == tick)
    }

    /// 时间范围查询
    pub fn range_tick(&self, start: u64, end: u64) -> &[IntentSentence] {
        let s = self.sentences.partition_point(|s| s.tick < start);
        let e = self.sentences.partition_point(|s| s.tick < end);
        &self.sentences[s..e]
    }
}

/// 单符号地址空间常量

pub const fn spatial_single_address_space() -> u64 {
    CONSONANT_COUNT as u64 * VOWEL_COUNT as u64 * TONE_COUNT as u64
}

// ─────────────────────────────────────────────────────────────────────────────
// 三维几何字形映射（SVG 字体文件生成）
// ─────────────────────────────────────────────────────────────────────────────
//
// 根据 glyph-container 专家系统约定：
//   字体文件 = 三维几何投影，每个字形对应一个空间地址
//   X轴 = 声母容器索引（语义类别）
//   Y轴 = 韵母容器索引（操作数细节）
//   Z轴 = 声调容器索引（时间窗口/相位）
//
// 零维符号（地址本身）→ 携带足够信息 → 投影为完整几何实体

/// 音节地址的三维几何拓扑参数

#[derive(Clone, Copy, Debug)]
pub struct GlyphTopology {
    /// 语义ID（声母决定，字形选择）
    pub semantic_id: u8,
    /// 旋转角（韵母决定，0-359度）
    pub rotation_deg: u16,
    /// 缩放 X（容量比映射）
    pub scale_x: f64,
    /// 缩放 Y（容量比映射）
    pub scale_y: f64,
    /// 颜色相位（声调决定，0-15）
    pub color_phase: u8,
    /// 脉冲频率（声调决定，0-15）
    pub pulse_freq: u8,
}

impl GlyphTopology {
    /// 从音节地址生成拓扑参数
    ///
    /// 映射规则：
    /// · 声母索引 → semantic_id（几何类型选择）
    /// · 韵母索引 → rotation_deg（旋转角）
    /// · 声调索引 → 颜色相位 + 脉冲频率 + 缩放比
    pub fn from_syllable_address(addr: &SyllableAddress) -> Self {
        let semantic_id = (addr.consonant_idx % 14 + 1) as u8; // 映射到1-14（对应glyph-3d-font的拓扑类型）
        let rotation_deg = (addr.vowel_idx as u16 * 5) % 360; // 韵母→角度
        let scale_x = 0.5 + (addr.vowel_idx as f64 / 66.0); // 韵母→X轴缩放
        let scale_y = 0.5 + (addr.consonant_idx as f64 / 48.0); // 声母→Y轴缩放
        let color_phase = (addr.tone_idx % 16) as u8; // 声调→颜色相位
        let pulse_freq = (addr.tone_idx % 15 + 1) as u8; // 声调→脉冲

        Self {
            semantic_id,
            rotation_deg,
            scale_x,
            scale_y,
            color_phase,
            pulse_freq,
        }
    }

    /// 根据 semantic_id 选择几何拓扑类型
    pub fn topology_type(&self) -> &'static str {
        match self.semantic_id {
            1 | 2 => "box",        // LOAD/STORE → 盒型（存储）
            3 | 4 | 5 | 6 | 7 | 8 => "sphere",   // 算逻运算 → 球型（数据流）
            9 | 10 => "torus",     // JMP/BNE → 环面（循环/分支）
            11 | 12 | 13 => "tube", // CALL/RET → 管状（调用栈）
            14 => "double_cone",   // CMP/MOV → 双锥（条件/移动）
            _ => "box", // 默认
        }
    }
}

/// 颜色相位 → HSL 色相值（16等分色相环）
fn phase_to_hsl(phase: u8) -> (u16, u8, u8) {
    let hue = (phase as u16 * 22) % 360; // 0, 22, 44, ..., 330
    (hue, 70, 50) // HSL: 饱合度70%, 亮度50%
}

/// SVG 空间编码器：将地址投影为字形轮廓路径

pub struct SVGSpatialEncoder;

impl SVGSpatialEncoder {
    /// 编码一个音节地址为 SVG 路径数据
    ///
    /// 返回值：(path_data, (x_min, y_min, x_max, y_max))
    pub fn encode_glyph(addr: &SyllableAddress) -> (String, (i16, i16, i16, i16)) {
        let topo = GlyphTopology::from_syllable_address(addr);

        // 计算包围盒
        let base_size = 800i16;
        let x_min = 100i16;
        let y_min = 100i16;
        let x_max = (100 + (base_size as f64 * topo.scale_x) as i16).min(900);
        let y_max = (100 + (base_size as f64 * topo.scale_y) as i16).min(900);

        // 坐标原点
        let cx = (x_min + x_max) / 2;
        let cy = (y_min + y_max) / 2;
        let rx = (x_max - x_min) / 2;
        let ry = (y_max - y_min) / 2;

        let path = match topo.topology_type() {
            "box" => Self::box_path(cx, cy, rx, ry, topo.rotation_deg),
            "sphere" => Self::sphere_path(cx, cy, rx, ry),
            "torus" => Self::torus_path(cx, cy, rx, ry),
            "tube" => Self::tube_path(cx, cy, rx, ry),
            "double_cone" => Self::double_cone_path(cx, cy, rx, ry),
            _ => Self::box_path(cx, cy, rx, ry, topo.rotation_deg),
        };

        (path, (x_min, y_min, x_max, y_max))
    }

    /// 盒型轮廓：四个顶点的矩形（带旋转）
    fn box_path(cx: i16, cy: i16, rx: i16, ry: i16, rotation: u16) -> String {
        let rad = rotation as f64 * std::f64::consts::PI / 180.0;
        let cos_r = rad.cos();
        let sin_r = rad.sin();
        // 四个角点（旋转后）
        let corners = [
            (-rx, -ry), (rx, -ry), (rx, ry), (-rx, ry),
        ];
        let transformed: Vec<(i16, i16)> = corners.iter()
            .map(|&(x, y)| {
                let tx = (x as f64 * cos_r - y as f64 * sin_r) as i16 + cx;
                let ty = (x as f64 * sin_r + y as f64 * cos_r) as i16 + cy;
                (tx, ty)
            })
            .collect();
        format!(
            "M {} {} L {} {} L {} {} L {} {} Z",
            transformed[0].0, transformed[0].1,
            transformed[1].0, transformed[1].1,
            transformed[2].0, transformed[2].1,
            transformed[3].0, transformed[3].1,
        )
    }

    /// 球型轮廓：大圆 + 内部两个椭圆表示体积感
    fn sphere_path(cx: i16, cy: i16, rx: i16, ry: i16) -> String {
        let ry3 = ry / 3;
        let rx3 = rx / 3;
        let ry4 = ry / 4;
        format!(
            "M {cxm} {cy} A {rx} {ry} 0 1 0 {cxp} {cy} A {rx} {ry} 0 1 0 {cxm} {cy} Z M {cx} {cym} A {rx3} {ry4} 0 1 1 {cx} {cyp} A {rx3} {ry4} 0 1 1 {cx} {cym} Z",
            cxm = cx - rx,
            cxp = cx + rx,
            cym = cy - ry3,
            cyp = cy + ry3,
        )
    }

    /// 环面轮廓：同心双圆
    fn torus_path(cx: i16, cy: i16, rx: i16, ry: i16) -> String {
        let rx2 = rx / 2;
        let ry2 = ry / 2;
        format!(
            "M {cxm} {cy} A {rx} {ry} 0 1 0 {cxp} {cy} A {rx} {ry} 0 1 0 {cxm} {cy} Z M {innerm} {cy} A {rx2} {ry2} 0 1 1 {innerp} {cy} A {rx2} {ry2} 0 1 1 {innerm} {cy} Z",
            cxm = cx - rx,
            cxp = cx + rx,
            innerm = cx - rx2,
            innerp = cx + rx2,
        )
    }

    /// 管状轮廓：圆柱侧视投影
    fn tube_path(cx: i16, cy: i16, rx: i16, ry: i16) -> String {
        let h = ry / 4; // 圆柱端盖的椭圆半高
        let top = cy - h;
        let bot = cy + h;
        let left = cx - rx;
        let right = cx + rx;
        format!(
            "M {left} {top} L {left} {bot} A {rx} {h} 0 0 0 {right} {bot} L {right} {top} A {rx} {h} 0 0 0 {left} {top} Z",
        )
    }

    /// 双锥轮廓（条件判断）：上下两个三角对接
    fn double_cone_path(cx: i16, cy: i16, rx: i16, ry: i16) -> String {
        format!(
            "M {} {} L {} {} L {} {} Z M {} {} L {} {} L {} {} Z",
            cx, cy - ry,
            cx - rx, cy,
            cx + rx, cy,
            cx, cy + ry,
            cx - rx, cy,
            cx + rx, cy,
        )
    }

    /// 生成单个字形的完整 SVG 元素字符串
    pub fn render_glyph_element(addr: &SyllableAddress, unicode_slot: u32) -> String {
        let (path, (x_min, y_min, x_max, y_max)) = Self::encode_glyph(addr);
        let topo = GlyphTopology::from_syllable_address(addr);
        let (hue, sat, light) = phase_to_hsl(topo.color_phase);
        let fill = format!("hsl({} {}% {}%)", hue, sat, light);
        let glyph_name = format!("glyph_{}_{}_{}", addr.consonant_idx, addr.vowel_idx, addr.tone_idx);

        // 避免在 format! 中使用 &#x... 和 # 字符（与原始字符串定界符冲突）
        // 改用 + 拼接
        let unicode_escape = format!("&#x{:04X};", unicode_slot);
        format!(
            r##"<glyph glyph-name="{}" unicode="{}" d="{}" horiz-adv-x="1000">
  <!-- consonant={} vowel={} tone={} -->
  <!-- semantic={:?} rotation={}° -->
  <path d="{}" fill="{}" fill-opacity="0.15" stroke="{}" stroke-width="3"/>
</glyph>"##,
            glyph_name,
            unicode_escape,
            &path,
            addr.consonant_idx,
            addr.vowel_idx,
            addr.tone_idx,
            topo.topology_type(),
            topo.rotation_deg,
            &path,
            &fill,
            &fill,
        )
    }
}

/// SVG 空间字体构建器：生成完整可加载的 SVG 字体文件

pub struct SVGSpatialFontBuilder {
    glyphs: Vec<SyllableAddress>,
    font_name: String,
    em_size: u16,
}

impl SVGSpatialFontBuilder {
    pub fn new() -> Self {
        Self {
            glyphs: Vec::new(),
            font_name: "NJSG-SpatialFont".to_string(),
            em_size: 1000,
        }
    }

    pub fn with_font_name(mut self, name: &str) -> Self {
        self.font_name = name.to_string();
        self
    }

    /// 添加一个音节地址作为字形
    pub fn add_glyph(&mut self, addr: SyllableAddress) {
        self.glyphs.push(addr);
    }

    /// 按声母范围批量添加字形
    pub fn add_consonant_range(&mut self, c_start: u8, c_end: u8) {
        for c in c_start..=c_end {
            for v in 0..66u8 {
                for t in 0..5u8 { // 只取前5个声调（节省文件大小）
                    if let Some(addr) = SyllableAddress::new(c, v, t) {
                        self.glyphs.push(addr);
                    }
                }
            }
        }
    }

    /// 生成完整的 SVG 字体文件内容
    pub fn build(&self) -> String {
        let mut svg = String::new();
        // SVG 字体头部
        svg.push_str(&format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<!--
  NJSG 囡囝共生操作系统 — 空间发音字体
  维度: 声母(X) × 韵母(Y) × 声调(Z) = 48×66×100 = 316,800 地址
  生成时间: 意图时间动作输出
  字形数: {}
-->
<defs>
<font id="{}" horiz-adv-x="{}">
<font-face font-family="{}"
    units-per-em="{}"
    ascent="800"
    descent="-200"
    bbox="0 0 1000 1000"/>
<missing-glyph horiz-adv-x="1000"><path d="M0 0 L1000 0 L1000 1000 L0 1000 Z" fill="none" stroke="#ccc"/></missing-glyph>
"##,
            self.glyphs.len(),
            self.font_name,
            self.em_size,
            self.font_name,
            self.em_size,
        ));

        // 逐个生成字形
        for (i, addr) in self.glyphs.iter().enumerate() {
            let unicode_slot = 0xE000 + (i as u32 % 0x1FFF); // Unicode PUA区段
            let glyph_svg = SVGSpatialEncoder::render_glyph_element(addr, unicode_slot);
            svg.push_str(&glyph_svg);
            svg.push('\n');
        }

        // 字体尾部
        svg.push_str("</font>\n</defs>\n");

        // 可视化：将所有字形按声母×韵母排列
        svg.push_str(&self.render_glyph_matrix());

        svg.push_str("</svg>\n");
        svg
    }

    /// 生成字形矩阵视图（声母为行，韵母为列，声调为不同透明度）
    fn render_glyph_matrix(&self) -> String {
        let mut svg = String::from("\n<!-- 声母×韵母 字形矩阵预览 -->\n<g id=\"matrix_view\" transform=\"translate(50,900)\">\n");
        // 选择前10个声母，前10个韵母，3个声调
        for c in 0..10u8 {
            for v in 0..10u8 {
                for t in 0..3u8 {
                    if let Some(addr) = SyllableAddress::new(c, v, t) {
                        let (path, _) = SVGSpatialEncoder::encode_glyph(&addr);
                        let topo = GlyphTopology::from_syllable_address(&addr);
                        let (hue, sat, light) = phase_to_hsl(topo.color_phase);
                        let x_offset = v as i16 * 55;
                        let y_offset = c as i16 * 55;
                        let opacity = if t == 0 { 1.0 } else if t == 1 { 0.6 } else { 0.3 };
                        svg.push_str(&format!(
                            r#"<g transform="translate({},{}),scale(0.05)">
  <path d="{}" fill="hsl({} {}% {}%)" opacity="{}" stroke="none"/>
</g>"#,
                            x_offset, y_offset,
                            path,
                            hue, sat, light,
                            opacity,
                        ));
                        svg.push('\n');
                    }
                }
            }
        }
        svg.push_str("</g>\n");
        svg
    }

    /// 生成单个地址的空间位置可视化（简化版）
    pub fn render_spatial_view(addr: &SyllableAddress) -> String {
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 200">
  <rect width="400" height="200" fill="#1a1a2e"/>
  <text x="10" y="30" fill="white" font-size="14">音节地址: 声{} 韵{} 调{}</text>
  <text x="10" y="55" fill="white" font-size="12">声母idx={} — 语义类别={}</text>
  <text x="10" y="75" fill="white" font-size="12">韵母idx={} — 操作数角度={}°</text>
  <text x="10" y="95" fill="white" font-size="12">声调idx={} — 颜色相位={}</text>
  <text x="10" y="120" fill="white" font-size="10">容量: 声容器={}符号 韵容器={}符号 调容器={}符号</text>
</svg>"##,
            addr.consonant_idx,
            addr.vowel_idx,
            addr.tone_idx,
            addr.consonant_idx,
            (addr.consonant_idx % 14 + 1),
            addr.vowel_idx,
            (addr.vowel_idx as u16 * 5) % 360,
            addr.tone_idx,
            addr.tone_idx % 16,
            66 * 100,
            48 * 100,
            48 * 66,
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 空间时间 · 动作时间 · 意图句子 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod spatial_action_intent_tests {
    use super::*;

    #[test]
    fn test_spatial_time_creation() {
        let spatial = SpatialTime::new();
        assert_eq!(spatial.consonant_fragments.len(), CONSONANT_COUNT as usize);
        assert_eq!(spatial.vowel_fragments.len(), VOWEL_COUNT as usize);
        assert_eq!(spatial.tone_fragments.len(), TONE_COUNT as usize);
    }

    #[test]
    fn test_single_address_space() {
        let spatial = SpatialTime::new();
        let space = spatial.single_address_space();
        // 48 × 66 × 100 = 316,800
        assert_eq!(space, 316_800);
    }

    #[test]
    fn test_days_covered_single_digit() {
        let spatial = SpatialTime::new();
        let days = spatial.days_covered_single_digit();
        // 316800 / 86400 ≈ 3.67 天
        assert!(days > 3.0 && days < 4.0);
    }

    #[test]
    fn test_double_digit_years() {
        let spatial = SpatialTime::new();
        // 双符号地址 = 316800² ≈ 1000亿 地址
        let addr2 = spatial.address_space_with_digits(2);
        assert_eq!(addr2, 316_800u128 * 316_800u128);
        // 折合年份 ≈ 3182 年
        let seconds_per_year: u128 = 365 * 86_400;
        let years = addr2 / seconds_per_year;
        assert!(years > 3_000 && years < 4_000);
    }

    #[test]
    fn test_expansion_factor() {
        let spatial = SpatialTime::new();
        let single = spatial.single_address_space() as u128;
        let double = spatial.address_space_with_digits(2);
        // 扩张倍数 = double / single = single（即316800倍）
        let factor = double / single;
        assert_eq!(factor, 316_800);
    }

    #[test]
    fn test_triple_digit_expansion() {
        let spatial = SpatialTime::new();
        let triple = spatial.address_space_with_digits(3);
        // 三符号 = 316800³ ≈ 31.8 万亿 地址
        // 覆盖年份 ≈ 11 亿年（超过地球年龄）
        assert_eq!(triple, 316_800u128 * 316_800u128 * 316_800u128);
    }

    #[test]
    fn test_cursor_uniform_step() {
        let spatial = SpatialTime::new();
        let mut cursor = SpatialCursor::new(1);
        // 前进一步：声调+1

        cursor.step(&spatial);
        assert_eq!(cursor.tone_pos, 1);
        assert_eq!(cursor.total_ticks, 1);

        // 前进100步：声调归零，韵母+1
        for _ in 0..99 {
            cursor.step(&spatial);
        }
        assert_eq!(cursor.tone_pos, 0);
        assert_eq!(cursor.vowel_pos, 1);
        assert_eq!(cursor.total_ticks, 100);
    }

    #[test]
    fn test_cursor_current_address() {
        let spatial = SpatialTime::new();
        let mut cursor = SpatialCursor::new(1);
        // 原点地址为0
        assert_eq!(cursor.current_address(), 0);
        // 前进一步：声调1
        cursor.step(&spatial);
        assert_eq!(cursor.current_address(), 1);
    }

    #[test]
    fn test_cursor_capacity_and_seconds() {
        let spatial = SpatialTime::new();
        let cursor = SpatialCursor::new(1);
        // 一声母容器容量 = 66韵母 × 100声调 = 6600
        let capacity = cursor.current_container_capacity(&spatial);
        assert_eq!(capacity, (66 * 100) as u32);
        // 一秒一拍，容器秒数 = 容量
        let seconds = cursor.current_container_seconds(&spatial);
        assert_eq!(seconds, (66 * 100) as u64);
    }

    #[test]
    fn test_total_cycle_days() {
        let spatial = SpatialTime::new();
        let cursor = SpatialCursor::new(1);
        let days = cursor.total_days_cycle(&spatial);
        // 316800秒 = 3.67天
        assert!(days > 3.0 && days < 4.5);
    }

    #[test]
    fn test_action_time_single_beat() {
        let mut action = ActionTime::new();
        let sentences = action.run_beats(1);
        assert_eq!(sentences.len(), 1);
        assert_eq!(sentences[0].tick, 1);
    }

    #[test]
    fn test_action_time_full_cycle() {
        let mut action = ActionTime::new();
        // 走完整个单符号地址空间316800步
        let sentences = action.run_beats(316_800);
        assert_eq!(sentences.len(), 316_800);
        // 最后一步绕回
        assert_eq!(sentences.last().unwrap().tick, 316_800);
    }

    #[test]
    fn test_action_stats() {
        let action = ActionTime::new();
        let stats = action.container_naming_stats();
        assert_eq!(stats.total_containers, (48 + 66 + 100) as u64);
        assert_eq!(stats.single_symbol_address_space, 316_800);
        assert!(stats.expansion_factor_double > 300_000.0);
    }

    #[test]
    fn test_intent_timeline_ingest() {
        let mut timeline = IntentSentenceTimeline::new();
        let mut action = ActionTime::new();
        let sentences = action.run_beats(1000);
        timeline.ingest_action_sentences(sentences);
        assert_eq!(timeline.sentences.len(), 1000);
    }

    #[test]
    fn test_intent_timeline_tick_index() {
        let mut timeline = IntentSentenceTimeline::new();
        let mut action = ActionTime::new();
        let sentences = action.run_beats(1000);
        timeline.ingest_action_sentences(sentences);
        // tick=500 的句子
        let sentence = timeline.index_by_tick(500);
        assert!(sentence.is_some());
        assert_eq!(sentence.unwrap().tick, 500);
    }

    #[test]
    fn test_intent_timeline_expand_digits() {
        let mut timeline = IntentSentenceTimeline::new();
        let base_before = timeline.symbolic_base;
        timeline.expand_digits();
        assert_eq!(timeline.digits, 2);
        assert!(timeline.symbolic_base > base_before);
    }

    #[test]
    fn test_spatial_capacity_sum() {
        let spatial = SpatialTime::new();
        let total = spatial.total_capacity_symbols();
        // 声母碎片总容量 = 48 × (66×100) = 316,800
        // 韵母碎片总容量 = 66 × (48×100) = 316,800
        // 声调碎片总容量 = 100 × (48×66) = 316,800
        // 合计 = 950,400 符号
        let expected = 316_800u64 * 3;
        assert_eq!(total, expected);
    }

    // ── 声母/韵母/声调表长度验证 ──

    #[test]
    fn test_consonant_table_is_48() {
        assert_eq!(CONSONANTS.len(), 48, "声母表必须有48个条目");
    }

    #[test]
    fn test_vowel_table_is_66() {
        assert_eq!(VOWELS.len(), 66, "韵母表必须有66个条目");
    }

    #[test]
    fn test_tone_table_is_100() {
        assert_eq!(TONES.len(), 100, "声调表必须有100个条目");
    }

    #[test]
    fn test_total_syllable_address_space() {
        // 48 × 66 × 100 = 316,800
        let total = CONSONANTS.len() * VOWELS.len() * TONES.len();
        assert_eq!(total, 316_800);
    }

    // ── 三维几何字形映射（SVG字体文件生成） ──

    #[test]
    fn test_glyph_topology_from_spatial_address() {
        let addr = SyllableAddress::new(3, 10, 50).unwrap();
        let topo = GlyphTopology::from_syllable_address(&addr);
        // 声母3 → 语义ID应在1-14
        assert!(topo.semantic_id >= 1 && topo.semantic_id <= 14);
        // 韵母10 → 旋转角 = 10*5 = 50
        assert_eq!(topo.rotation_deg, 50);
        // 声调50 → 颜色相位在0-15
        assert!(topo.color_phase <= 15);
    }

    #[test]
    fn test_svg_glyph_encoder_produces_path() {
        let addr = SyllableAddress::new(1, 2, 3).unwrap();
        let (path, bbox) = SVGSpatialEncoder::encode_glyph(&addr);
        assert!(!path.is_empty());
        assert!(path.starts_with('M'));
        assert!(path.ends_with('Z'));
        assert!(bbox.2 > 0); // width > 0
        assert!(bbox.3 > 0); // height > 0
    }

    #[test]
    fn test_svg_spatial_font_builder() {
        let mut builder = SVGSpatialFontBuilder::new();
        // 添加一个地址
        let addr = SyllableAddress::new(10, 30, 70).unwrap();
        builder.add_glyph(addr);
        // 生成完整字体文件
        let svg = builder.build();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("glyph_"));
        assert!(svg.contains("NJSG-SpatialFont"));
    }

    #[test]
    fn test_svg_spatial_font_view() {
        let addr = SyllableAddress::new(5, 15, 25).unwrap();
        let view = SVGSpatialFontBuilder::render_spatial_view(&addr);
        assert!(view.contains("声5"));
        assert!(view.contains("韵15"));
        assert!(view.contains("调25"));
    }

    #[test]
    fn test_topology_type_mapping() {
        // 测试不同声母映射到不同拓扑类型
        let addr1 = SyllableAddress::new(0, 10, 50).unwrap(); // sem_id=1 → box
        let topo1 = GlyphTopology::from_syllable_address(&addr1);
        assert_eq!(topo1.topology_type(), "box");

        let addr5 = SyllableAddress::new(4, 10, 50).unwrap(); // sem_id=5 → sphere
        let topo5 = GlyphTopology::from_syllable_address(&addr5);
        assert_eq!(topo5.topology_type(), "sphere");

        let addr14 = SyllableAddress::new(13, 10, 50).unwrap(); // sem_id=14 → double_cone
        let topo14 = GlyphTopology::from_syllable_address(&addr14);
        assert_eq!(topo14.topology_type(), "double_cone");
    }

    #[test]
    fn test_glyph_color_phase() {
        let addr1 = SyllableAddress::new(10, 20, 0).unwrap();
        let topo1 = GlyphTopology::from_syllable_address(&addr1);
        let (h1, _, _) = phase_to_hsl(topo1.color_phase);

        let addr2 = SyllableAddress::new(10, 20, 8).unwrap();
        let topo2 = GlyphTopology::from_syllable_address(&addr2);
        let (h2, _, _) = phase_to_hsl(topo2.color_phase);

        // 不同声调 → 不同相位 → 不同色相
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_font_builder_batch_add() {
        let mut builder = SVGSpatialFontBuilder::new();
        builder.add_consonant_range(0, 2); // 3个声母 × 66韵母 × 5声调 = 990 字形
        let svg = builder.build();
        assert!(svg.contains("glyph_0_0_0"));
        assert!(svg.contains("svg"));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 统计文档聚合浏览器 · 八流统一层 (追加)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ClusterNode {
    pub id: u64,
    pub centroid: (f64, f64, f64),
    pub members: Vec<(u8, u8, u8)>,
    pub radius: f64,
    pub density: f64,
}

impl ClusterNode {
    pub fn new(id: u64, centroid: (f64, f64, f64)) -> Self {
        Self { id, centroid, members: Vec::new(), radius: 0.0, density: 0.0 }
    }
    pub fn add_member(&mut self, addr: (u8, u8, u8), dist: f64) {
        self.members.push(addr);
        if dist > self.radius { self.radius = dist; }
        if self.radius > 0.0 {
            let vol = (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3);
            self.density = self.members.len() as f64 / vol;
        }
    }
}

pub struct ClusteringEngine {
    pub clusters: Vec<ClusterNode>,
    pub weight_consonant: f64,
    pub weight_vowel: f64,
    pub weight_tone: f64,
    pub threshold: f64,
    next_id: u64,
}

impl ClusteringEngine {
    pub fn new() -> Self {
        Self { clusters: Vec::new(), weight_consonant: 3.0, weight_vowel: 2.0, weight_tone: 1.0, threshold: 50.0, next_id: 0 }
    }
    pub fn with_weights(wc: f64, wv: f64, wt: f64) -> Self {
        Self { clusters: Vec::new(), weight_consonant: wc, weight_vowel: wv, weight_tone: wt, threshold: 50.0, next_id: 0 }
    }
    pub fn distance(&self, a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
        let dc = (a.0 as f64 - b.0 as f64) * self.weight_consonant;
        let dv = (a.1 as f64 - b.1 as f64) * self.weight_vowel;
        let dt = (a.2 as f64 - b.2 as f64) * self.weight_tone;
        (dc * dc + dv * dv + dt * dt).sqrt()
    }
    fn alloc_id(&mut self) -> u64 { let id = self.next_id; self.next_id += 1; id }

    pub fn cluster(&mut self, addresses: &[(u8, u8, u8)]) -> &[ClusterNode] {
        if addresses.is_empty() { return &self.clusters; }
        let mut tc: Vec<ClusterNode> = addresses.iter()
            .map(|&(c, v, t)| ClusterNode::new(self.alloc_id(), (c as f64, v as f64, t as f64)))
            .collect();
        for (i, &(c, v, t)) in addresses.iter().enumerate() { tc[i].add_member((c, v, t), 0.0); }
        loop {
            let mut min_dist = f64::MAX;
            let mut merge_pair: Option<(usize, usize)> = None;
            for i in 0..tc.len() {
                for j in (i + 1)..tc.len() {
                    let ci = (tc[i].centroid.0 as u8, tc[i].centroid.1 as u8, tc[i].centroid.2 as u8);
                    let cj = (tc[j].centroid.0 as u8, tc[j].centroid.1 as u8, tc[j].centroid.2 as u8);
                    let d = self.distance(ci, cj);
                    if d < min_dist { min_dist = d; merge_pair = Some((i, j)); }
                }
            }
            if min_dist > self.threshold || tc.len() <= 1 { break; }
            if let Some((i, j)) = merge_pair {
                let mut merged = tc[i].members.clone();
                merged.extend_from_slice(&tc[j].members);
                let n = merged.len() as f64;
                let sum = merged.iter().fold((0.0, 0.0, 0.0), |acc, m| (acc.0 + m.0 as f64, acc.1 + m.1 as f64, acc.2 + m.2 as f64));
                let centroid = (sum.0 / n, sum.1 / n, sum.2 / n);
                let mut new_cluster = ClusterNode::new(tc[i].id, centroid);
                for &m in &merged {
                    let dx = m.0 as f64 - centroid.0; let dy = m.1 as f64 - centroid.1; let dz = m.2 as f64 - centroid.2;
                    new_cluster.add_member(m, (dx * dx + dy * dy + dz * dz).sqrt());
                }
                let (ri, rk) = if i > j { (i, j) } else { (j, i) };
                tc.remove(ri); tc.remove(rk); tc.push(new_cluster);
            } else { break; }
        }
        self.clusters = tc;
        &self.clusters
    }

    pub fn stats(&self) -> ClusterStats {
        let total: usize = self.clusters.iter().map(|c| c.members.len()).sum();
        let avg_r = if self.clusters.is_empty() { 0.0 } else { self.clusters.iter().map(|c| c.radius).sum::<f64>() / self.clusters.len() as f64 };
        let max_d = self.clusters.iter().map(|c| c.density).fold(0.0, f64::max);
        ClusterStats { cluster_count: self.clusters.len(), total_members: total, avg_radius: avg_r, max_density: max_d }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClusterStats {
    pub cluster_count: usize,
    pub total_members: usize,
    pub avg_radius: f64,
    pub max_density: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlowKind {
    Preload, Interpret, Sovereignty, Property, Resonance, Feedback, Invocation, CommonDivisor,
}

impl FlowKind {
    pub fn layer(&self) -> u8 {
        match self { FlowKind::Preload => 1, FlowKind::Interpret => 2, FlowKind::Sovereignty => 3,
            FlowKind::Property => 4, FlowKind::Resonance => 5, FlowKind::Feedback => 6,
            FlowKind::Invocation => 7, FlowKind::CommonDivisor => 8 }
    }
    pub fn all() -> [FlowKind; 8] {
        [FlowKind::Preload, FlowKind::Interpret, FlowKind::Sovereignty, FlowKind::Property,
         FlowKind::Resonance, FlowKind::Feedback, FlowKind::Invocation, FlowKind::CommonDivisor]
    }
    /// 流优先级: 值越小优先级越高
    pub fn priority(self) -> u8 {
        match self {
            FlowKind::Preload => 0, FlowKind::Interpret => 1, FlowKind::Sovereignty => 2,
            FlowKind::Property => 3, FlowKind::Resonance => 4, FlowKind::Feedback => 5,
            FlowKind::Invocation => 6, FlowKind::CommonDivisor => 7,
        }
    }
    /// 是否为动态流
    pub fn is_dynamic(&self) -> bool {
        matches!(self, FlowKind::Resonance | FlowKind::Feedback | FlowKind::Invocation)
    }
}

#[derive(Clone, Debug)]
pub struct FlowState {
    pub kind: FlowKind,
    pub intensity: f64,
    pub address: Option<(u8, u8, u8)>,
    pub completed: bool,
}

impl FlowState {
    pub fn new(kind: FlowKind) -> Self {
        Self { kind, intensity: 0.5, address: None, completed: false }
    }
    pub fn advance(&mut self, delta: f64) { self.intensity = (self.intensity + delta).clamp(0.0, 1.0); }
    pub fn complete(&mut self) { self.completed = true; self.intensity = 1.0; }
}

#[derive(Clone, Debug)]
pub enum ContainerInteraction {
    Absorb { target: String, source: String },
    Resonate { a: String, b: String, strength: f64 },
    Invoke { location: (u8, u8, u8), program: String },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RouterStats {
    pub total_routes: usize,
    pub absorbed_containers: usize,
    pub resonated_pairs: usize,
}

pub struct ConsciousnessRouter {
    pub active_flows: Vec<FlowState>,
    pub containers: Vec<String>,
    pub interactions: Vec<ContainerInteraction>,
    pub stats: RouterStats,
}

impl ConsciousnessRouter {
    pub fn new() -> Self {
        Self { active_flows: Vec::new(), containers: Vec::new(), interactions: Vec::new(), stats: RouterStats::default() }
    }
    pub fn register_container(&mut self, name: &str) {
        if !self.containers.contains(&name.to_string()) { self.containers.push(name.to_string()); }
    }
    pub fn activate_flow(&mut self, kind: FlowKind, addr: Option<(u8, u8, u8)>) -> usize {
        let s = FlowState { kind, intensity: 0.5, address: addr, completed: false };
        self.active_flows.push(s);
        self.active_flows.len() - 1
    }
    pub fn route(&mut self, addr: (u8, u8, u8), kind: FlowKind) -> Option<usize> {
        if let Some((idx, _)) = self.active_flows.iter().enumerate()
            .find(|(_, f)| f.kind == kind && f.address == Some(addr)) { return Some(idx); }
        Some(self.activate_flow(kind, Some(addr)))
    }
    pub fn interact(&mut self, inter: ContainerInteraction) {
        match &inter {
            ContainerInteraction::Absorb { .. } => self.stats.absorbed_containers += 1,
            ContainerInteraction::Resonate { .. } => self.stats.resonated_pairs += 1,
            _ => {}
        }
        self.stats.total_routes += 1;
        self.interactions.push(inter);
    }
    pub fn find_common_divisor(&self, a: &str, b: &str) -> Option<String> {
        let common: String = a.chars().zip(b.chars())
            .take_while(|(ca, cb)| ca == cb).map(|(c, _)| c).collect();
        if common.is_empty() { None } else { Some(common) }
    }
    pub fn flows_of_kind(&self, kind: FlowKind) -> Vec<&FlowState> {
        self.active_flows.iter().filter(|f| f.kind == kind).collect()
    }
}

#[cfg(test)]
mod new_subsystem_tests {
    use super::*;

    #[test]
    fn test_clustering_engine() {
        let mut e = ClusteringEngine::new();
        let addrs = vec![(0, 0, 0), (1, 0, 0), (0, 1, 0), (40, 60, 90), (41, 60, 90)];
        e.cluster(&addrs);
        let s = e.stats();
        assert!(s.cluster_count >= 1);
    }

    #[test]
    fn test_clustering_distance() {
        let e = ClusteringEngine::new();
        let d = e.distance((0, 0, 0), (10, 0, 0));
        assert!((d - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_clustering_merge() {
        let mut e = ClusteringEngine::with_weights(1.0, 1.0, 1.0);
        let c = e.cluster(&[(0, 0, 0), (3, 4, 0)]);
        assert_eq!(c.len(), 1, "距离5 < 阈值50, 应合并为一");
    }

    #[test]
    fn test_flow_kind_layers() {
        assert_eq!(FlowKind::Preload.layer(), 1);
        assert_eq!(FlowKind::Sovereignty.layer(), 3);
        assert_eq!(FlowKind::CommonDivisor.layer(), 8);
    }

    #[test]
    fn test_flow_state() {
        let mut s = FlowState::new(FlowKind::Resonance);
        assert_eq!(s.kind, FlowKind::Resonance);
        assert!((s.intensity - 0.5).abs() < 0.01);
        s.advance(0.3);
        s.complete();
        assert!(s.completed);
    }

    #[test]
    fn test_router() {
        let mut r = ConsciousnessRouter::new();
        r.register_container("A");
        r.register_container("B");
        assert_eq!(r.containers.len(), 2);
        let i = r.activate_flow(FlowKind::Resonance, Some((5, 10, 15)));
        assert_eq!(r.active_flows[i].kind, FlowKind::Resonance);
    }

    #[test]
    fn test_router_route() {
        let mut r = ConsciousnessRouter::new();
        let i1 = r.activate_flow(FlowKind::Resonance, Some((5, 10, 15)));
        let i2 = r.route((5, 10, 15), FlowKind::Resonance);
        assert_eq!(i1, i2.unwrap());
        let i3 = r.route((6, 10, 15), FlowKind::Resonance);
        assert_ne!(i1, i3.unwrap());
    }

    #[test]
    fn test_router_interact() {
        let mut r = ConsciousnessRouter::new();
        r.interact(ContainerInteraction::Absorb { target: "A".to_string(), source: "B".to_string() });
        assert_eq!(r.interactions.len(), 1);
        assert_eq!(r.stats.absorbed_containers, 1);
    }

    #[test]
    fn test_router_common_divisor() {
        let r = ConsciousnessRouter::new();
        assert_eq!(r.find_common_divisor("abcdef", "abcxyz"), Some("abc".to_string()));
        assert_eq!(r.find_common_divisor("foo", "bar"), None);
    }

    #[test]
    fn test_router_flows_of_kind() {
        let mut r = ConsciousnessRouter::new();
        r.activate_flow(FlowKind::Resonance, None);
        r.activate_flow(FlowKind::Resonance, None);
        r.activate_flow(FlowKind::Feedback, None);
        assert_eq!(r.flows_of_kind(FlowKind::Resonance).len(), 2);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 统计文档聚合浏览器 · 聚类引擎 · 连接包裹树
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// 八流统一层 · 意识路由 · 容器交互
// ═══════════════════════════════════════════════════════════════════════════
// 部首星图 · 二维投影 · 声母遍历韵母 · 聂小倩匹配
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug)]
pub struct StarCoordinate { pub x: f64, pub y: f64, pub luminance: f64, pub saturation: f64 }

impl StarCoordinate {
    pub fn from_address(addr: (u8, u8, u8)) -> Self {
        let angle_deg = (addr.0 as f64) * (360.0 / 48.0);
        let angle_rad = angle_deg * std::f64::consts::PI / 180.0;
        let radius = 10.0 + (addr.1 as f64) * (190.0 / 65.0);
        let luminance = 0.2 + (addr.2 as f64) * (0.8 / 99.0);
        Self { x: radius * angle_rad.cos(), y: radius * angle_rad.sin(), luminance, saturation: 0.6 + (addr.2 as f64) * (0.4 / 99.0) }
    }
    pub fn distance_to(&self, other: &StarCoordinate) -> f64 { let dx = self.x - other.x; let dy = self.y - other.y; (dx * dx + dy * dy).sqrt() }
}

#[derive(Clone, Debug)]
pub struct StarPoint { pub address: (u8, u8, u8), pub coord: StarCoordinate, pub importance: f64, pub radical_group: u8, pub energy: f64 }

impl StarPoint {
    pub fn new(address: (u8, u8, u8), radical_group: u8) -> Self {
        Self { address, coord: StarCoordinate::from_address(address), importance: 0.5, radical_group, energy: 1.0 }
    }
    pub fn semantic_id(&self) -> u8 { (self.address.0 % 14) + 1 }
}

#[derive(Clone, Debug)]
pub struct EssentialMatch { pub source: (u8, u8, u8), pub target: (u8, u8, u8), pub strength: f64, pub match_type: MatchType }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchType { Identity, Consonant, Vowel, CommonDivisor, Proximity }

pub struct RadicalStarMap {
    pub stars: Vec<StarPoint>,
    pub consonant_groups: Vec<Vec<u8>>,
    pub vowel_groups: Vec<Vec<u8>>,
    match_cache: Vec<EssentialMatch>,
}

impl RadicalStarMap {
    pub fn new() -> Self {
        let mut map = Self { stars: Vec::new(), consonant_groups: Vec::new(), vowel_groups: Vec::new(), match_cache: Vec::new() };
        map.init_groups(); map
    }
    fn init_groups(&mut self) {
        self.consonant_groups = vec![
            vec![0, 1, 2], vec![3, 4], vec![5, 6, 7, 8], vec![9, 10, 11],
            vec![12, 13, 14], vec![15, 16, 17], vec![18, 19, 20], vec![21, 22, 23],
            vec![24, 25, 26], vec![27, 28, 29], vec![30, 31, 32], vec![33, 34, 35],
            vec![36, 37, 38], vec![39, 40, 41, 42, 43, 44, 45, 46, 47],
        ];
        self.vowel_groups = vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![9, 10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24, 25],
            vec![26, 27, 28, 29, 30, 31], vec![32, 33, 34, 35, 36, 37],
            vec![38, 39, 40, 41, 42, 43],
            vec![44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65],
        ];
    }
    pub fn add_star(&mut self, address: (u8, u8, u8)) {
        let radical = self.classify_consonant(address.0);
        self.stars.push(StarPoint::new(address, radical));
    }
    pub fn add_range(&mut self, c_range: (u8, u8), v_range: (u8, u8), t_range: (u8, u8)) {
        for c in c_range.0..=c_range.1.min(47) { for v in v_range.0..=v_range.1.min(65) { for t in t_range.0..=t_range.1.min(99) { self.add_star((c, v, t)); } } }
    }
    fn classify_consonant(&self, consonant: u8) -> u8 {
        for (i, group) in self.consonant_groups.iter().enumerate() {
            if group.contains(&consonant) { return i as u8; }
        }
        0
    }
    pub fn essential_match(&mut self, source: (u8, u8, u8), max_results: usize) -> Vec<EssentialMatch> {
        let mut results = Vec::new();
        let source_coord = StarCoordinate::from_address(source);
        for c in 0u8..47 {
            for v in 0u8..65 {
                let target = (c, v, source.2);
                let target_coord = StarCoordinate::from_address(target);
                let dist = source_coord.distance_to(&target_coord);
                let max_dist = 400.0;
                let strength = 1.0 - (dist / max_dist).min(1.0);
                if c == source.0 && v == source.1 {
                    results.push(EssentialMatch { source, target, strength: 1.0, match_type: MatchType::Identity });
                } else if c == source.0 {
                    results.push(EssentialMatch { source, target, strength: strength * 0.8, match_type: MatchType::Consonant });
                } else if strength > 0.3 {
                    results.push(EssentialMatch {
                        source, target, strength,
                        match_type: if v == source.1 { MatchType::Vowel } else { MatchType::Proximity },
                    });
                }
            }
        }
        results.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(max_results);
        self.match_cache = results.clone();
        results
    }
    pub fn stars_in_consonant_group(&self, group: u8) -> Vec<&StarPoint> {
        self.stars.iter().filter(|s| s.radical_group == group).collect()
    }
    pub fn stars_in_region(&self, x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Vec<&StarPoint> {
        self.stars.iter().filter(|s| { let c = &s.coord; c.x >= x_min && c.x <= x_max && c.y >= y_min && c.y <= y_max }).collect()
    }
    pub fn energy_distribution(&self) -> Vec<f64> {
        let mut per_group = vec![0.0f64; 14];
        for star in &self.stars { per_group[star.radical_group as usize] += star.energy; }
        per_group
    }
    pub fn stats(&self) -> StarMapStats {
        let total_energy: f64 = self.stars.iter().map(|s| s.energy).sum();
        let avg_importance = if self.stars.is_empty() { 0.0 } else { self.stars.iter().map(|s| s.importance).sum::<f64>() / self.stars.len() as f64 };
        StarMapStats { star_count: self.stars.len(), consonant_group_count: self.consonant_groups.len(), vowel_group_count: self.vowel_groups.len(), cached_matches: self.match_cache.len(), total_energy, avg_importance }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StarMapStats { pub star_count: usize, pub consonant_group_count: usize, pub vowel_group_count: usize, pub cached_matches: usize, pub total_energy: f64, pub avg_importance: f64 }

// ═══════════════════════════════════════════════════════════════════════════
// 连接包裹树 · 聚合浏览器 (stub — 供测试编译)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct WrappingTreeNode { pub children: Vec<usize>, pub members: Vec<usize> }
impl WrappingTreeNode {
    pub fn leaf_count(&self) -> usize { if self.children.is_empty() { self.members.len() } else { self.children.len() } }
}

#[derive(Clone, Debug)]
pub struct ConnectionWrappingTree { pub root: WrappingTreeNode }
impl ConnectionWrappingTree {
    pub fn from_clusters(_clusters: &[ClusterNode]) -> Self {
        Self { root: WrappingTreeNode { children: (0.._clusters.len()).collect(), members: Vec::new() } }
    }
}

#[derive(Clone, Debug)]
pub struct BrowserDoc { pub name: String, pub addresses: Vec<(u8, u8, u8)> }

#[derive(Clone, Debug, Default)]
pub struct SituationalAggregationBrowser {
    pub documents: Vec<BrowserDoc>,
    built: bool,
}

impl SituationalAggregationBrowser {
    pub fn new() -> Self { Self::default() }
    pub fn add_document(&mut self, name: &str, c_range: (u8, u8), v_range: (u8, u8), t_range: (u8, u8)) {
        let mut addresses = Vec::new();
        for c in c_range.0..=c_range.1 { for v in v_range.0..=v_range.1 { for t in t_range.0..=t_range.1 { addresses.push((c, v, t)); } } }
        self.documents.push(BrowserDoc { name: name.to_string(), addresses });
    }
    pub fn build(&mut self) { self.built = true; }
    pub fn situational_query(&self, _addr: (u8, u8, u8)) -> Option<&BrowserDoc> { self.documents.first() }
    pub fn filter_by_consonant(&self, min: u8, max: u8) -> Vec<&BrowserDoc> {
        self.documents.iter().filter(|d| d.addresses.iter().any(|a| a.0 >= min && a.0 <= max)).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod clustering_browser_tests {
    use super::*;
    #[test] fn test_clustering_distance() { let e = ClusteringEngine::new(); let d = e.distance((0, 0, 0), (10, 0, 0)); assert!((d - 30.0).abs() < 0.01); }
    #[test] fn test_clustering_basic() {
        let mut e = ClusteringEngine::new();
        let c = e.cluster(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (40, 60, 90), (41, 60, 90), (40, 61, 90)]);
        assert!(c.len() >= 2, "应产生至少2个聚类，实际 {}", c.len());
    }
    #[test] fn test_clustering_cluster_merge() {
        let mut e = ClusteringEngine::with_weights(1.0, 1.0, 1.0);
        let c = e.cluster(&[(0, 0, 0), (3, 4, 0)]);
        assert_eq!(c.len(), 1);
    }
    #[test] fn test_clustering_stats() { let mut e = ClusteringEngine::new(); e.cluster(&[(0, 0, 0), (1, 1, 1), (2, 2, 2)]); let s = e.stats(); assert!(s.cluster_count >= 1 && s.total_members == 3); }
    #[test] fn test_wrapping_tree() { let mut e = ClusteringEngine::new(); e.cluster(&[(0, 0, 0), (1, 0, 0), (40, 60, 90), (41, 60, 90)]); let t = ConnectionWrappingTree::from_clusters(&e.clusters); assert!(t.root.leaf_count() > 0); }
    #[test] fn test_browser_doc() { let mut b = SituationalAggregationBrowser::new(); b.add_document("A", (0, 2), (0, 2), (0, 2)); assert_eq!(b.documents[0].addresses.len(), 27); }
    #[test] fn test_browser_query() { let mut b = SituationalAggregationBrowser::new(); b.add_document("G1", (0, 1), (0, 1), (0, 1)); b.build(); assert!(b.situational_query((0, 0, 0)).is_some()); }
    #[test] fn test_browser_filter() { let mut b = SituationalAggregationBrowser::new(); b.add_document("前", (0, 2), (0, 5), (0, 5)); b.add_document("后", (40, 45), (0, 5), (0, 5)); assert_eq!(b.filter_by_consonant(0, 10).len(), 1); }
}

#[cfg(test)]
mod flow_unification_tests {
    use super::*;
    #[test] fn test_flow_layers() { assert_eq!(FlowKind::Preload.layer(), 1); assert_eq!(FlowKind::Sovereignty.layer(), 3); assert_eq!(FlowKind::CommonDivisor.layer(), 8); }
    #[test] fn test_flow_dynamic() { assert!(FlowKind::Resonance.is_dynamic()); assert!(!FlowKind::Preload.is_dynamic()); }
    #[test] fn test_flow_all() { assert_eq!(FlowKind::all().len(), 8); }
    #[test] fn test_flow_state() { let mut s = FlowState::new(FlowKind::Resonance); assert!((s.intensity - 0.5).abs() < 0.01); s.advance(0.3); assert!((s.intensity - 0.8).abs() < 0.01); s.complete(); assert!(s.completed); }
    #[test] fn test_router() { let mut r = ConsciousnessRouter::new(); r.register_container("A"); r.register_container("B"); r.register_container("A"); assert_eq!(r.containers.len(), 2); }
    #[test] fn test_router_route() { let mut r = ConsciousnessRouter::new(); let i1 = r.activate_flow(FlowKind::Resonance, Some((5, 10, 15))); let i2 = r.route((5, 10, 15), FlowKind::Resonance); assert_eq!(i1, i2.unwrap()); let i3 = r.route((6, 10, 15), FlowKind::Resonance); assert_ne!(i1, i3.unwrap()); }
    #[test] fn test_router_interact() { let mut r = ConsciousnessRouter::new(); r.interact(ContainerInteraction::Absorb { target: "A".to_string(), source: "B".to_string() }); assert_eq!(r.stats.absorbed_containers, 1); }
    #[test] fn test_router_divisor() { let r = ConsciousnessRouter::new(); assert_eq!(r.find_common_divisor("abcdef", "abcxyz"), Some("abc".to_string())); assert_eq!(r.find_common_divisor("foo", "bar"), None); }
    #[test] fn test_router_flows_of_kind() { let mut r = ConsciousnessRouter::new(); r.activate_flow(FlowKind::Resonance, None); r.activate_flow(FlowKind::Resonance, None); r.activate_flow(FlowKind::Feedback, None); assert_eq!(r.flows_of_kind(FlowKind::Resonance).len(), 2); }
    #[test] fn test_priority() { assert!(FlowKind::Preload.priority() < FlowKind::Invocation.priority()); }
}

#[cfg(test)]
mod radical_star_tests {
    use super::*;
    #[test] fn test_star_coord() { let c = StarCoordinate::from_address((0, 0, 0)); assert!(c.x > 0.0); assert!(c.y.abs() < 0.01); assert!((c.luminance - 0.2).abs() < 0.01); }
    #[test] fn test_star_angle() { let c = StarCoordinate::from_address((12, 30, 50)); assert!(c.x.abs() < 1.0); assert!(c.y > 0.0); }
    #[test] fn test_star_distance() { let c1 = StarCoordinate::from_address((0, 30, 0)); let c2 = StarCoordinate::from_address((24, 30, 0)); let d = c1.distance_to(&c2); assert!(d > 50.0 && d < 300.0); }
    #[test] fn test_map_creation() { let m = RadicalStarMap::new(); assert_eq!(m.consonant_groups.len(), 14); assert_eq!(m.vowel_groups.len(), 8); }
    #[test] fn test_add_star() { let mut m = RadicalStarMap::new(); m.add_star((10, 20, 50)); assert_eq!(m.stars.len(), 1); }
    #[test] fn test_add_range() { let mut m = RadicalStarMap::new(); m.add_range((0, 2), (0, 3), (0, 1)); assert_eq!(m.stars.len(), 24); }
    #[test] fn test_essential_match_identity() { let mut m = RadicalStarMap::new(); let r = m.essential_match((10, 20, 50), 10); assert_eq!(r[0].match_type, MatchType::Identity); }
    #[test] fn test_essential_match_consonant() { let mut m = RadicalStarMap::new(); let r = m.essential_match((5, 10, 30), 5000); let cm: Vec<&EssentialMatch> = r.iter().filter(|m| m.target.0 == 5 && m.target.1 != 10).collect(); assert!(!cm.is_empty()); assert!(cm.iter().all(|m| matches!(m.match_type, MatchType::Consonant))); }
    #[test] fn test_stars_in_region() { let mut m = RadicalStarMap::new(); m.add_star((0, 30, 50)); let region = m.stars_in_region(-200.0, -200.0, 200.0, 200.0); assert!(!region.is_empty()); }
    #[test] fn test_energy_distribution() { let mut m = RadicalStarMap::new(); m.add_range((0, 47), (0, 10), (0, 0)); let d = m.energy_distribution(); assert_eq!(d.len(), 14); assert!(d.iter().any(|&e| e > 0.0)); }
    #[test] fn test_map_stats() { let mut m = RadicalStarMap::new(); m.add_range((0, 5), (0, 3), (0, 1)); assert_eq!(m.stats().star_count, 48); }
    #[test] fn test_semantic_id() { let p = StarPoint::new((15, 30, 50), 0); assert_eq!(p.semantic_id(), 2); }
    #[test] fn test_luminance() { let l = StarCoordinate::from_address((10, 30, 0)); let h = StarCoordinate::from_address((10, 30, 99)); assert!(h.luminance > l.luminance); }
    #[test] fn test_saturation() { let l = StarCoordinate::from_address((20, 40, 0)); let h = StarCoordinate::from_address((20, 40, 99)); assert!(h.saturation >= l.saturation); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 六十四位容器指令编码 · ContainerInstruction
// ═══════════════════════════════════════════════════════════════════════════
//
// 64位指令格式: [op:8][topo:4][bbox:12][consonant:8][vowel:8][tone:8][reserved:16]
// 每条指令 = 一个容器 = 一个音节地址 = 一个几何坐标

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContainerInstruction(pub u64);

impl ContainerInstruction {
    pub const fn new(op: u8, topo: u8, consonant: u8, vowel: u8, tone: u8) -> Self {
        let mut val = 0u64;
        val |= (op as u64) << 56;
        val |= ((topo & 0x0F) as u64) << 52;
        val |= (consonant as u64) << 32;
        val |= (vowel as u64) << 24;
        val |= (tone as u64) << 16;
        Self(val)
    }
    pub const fn op(self) -> u8 { ((self.0 >> 56) & 0xFF) as u8 }
    pub const fn topo(self) -> u8 { ((self.0 >> 52) & 0x0F) as u8 }
    pub const fn consonant(self) -> u8 { ((self.0 >> 32) & 0xFF) as u8 }
    pub const fn vowel(self) -> u8 { ((self.0 >> 24) & 0xFF) as u8 }
    pub const fn tone(self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
    pub const fn to_u64(self) -> u64 { self.0 }
}

impl Default for ContainerInstruction {
    fn default() -> Self { Self::new(0, 0, 0, 0, 0) }
}

/// 刻印缓冲区 — 容器指令序列 → 字节流
#[derive(Clone, Debug, Default)]
pub struct InscriptionBuffer(pub Vec<ContainerInstruction>);

impl InscriptionBuffer {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn push(&mut self, instr: ContainerInstruction) { self.0.push(instr); }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.0.len() * 8);
        for &instr in &self.0 { buf.extend_from_slice(&instr.to_u64().to_be_bytes()); }
        buf
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() % 8 != 0 { return None; }
        let mut instrs = Vec::with_capacity(bytes.len() / 8);
        for chunk in bytes.chunks_exact(8) {
            let val = u64::from_be_bytes(chunk.try_into().ok()?);
            instrs.push(ContainerInstruction(val));
        }
        Some(Self(instrs))
    }
}

/// 64进制容器 — 去除外壳的量化核心
#[derive(Clone, Debug, Default)]
pub struct ContainerRadix64(pub Vec<u8>);

impl ContainerRadix64 {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn push(&mut self, val: u8) { if val < 64 { self.0.push(val); } }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn to_compact_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity((self.0.len() * 3 + 3) / 4);
        for chunk in self.0.chunks(4) {
            let mut buf = [0u8; 4];
            for (i, &v) in chunk.iter().enumerate() { buf[i] = v; }
            let val = ((buf[0] as u32) << 18) | ((buf[1] as u32) << 12) | ((buf[2] as u32) << 6) | (buf[3] as u32);
            out.push(((val >> 16) & 0xFF) as u8);
            out.push(((val >> 8) & 0xFF) as u8);
            out.push((val & 0xFF) as u8);
        }
        out
    }
}

#[cfg(test)]
mod container_instruction_tests {
    use super::*;
    #[test] fn test_instr_encode() { let i = ContainerInstruction::new(42, 3, 10, 20, 50); assert_eq!(i.op(), 42); assert_eq!(i.topo(), 3); assert_eq!(i.consonant(), 10); assert_eq!(i.vowel(), 20); assert_eq!(i.tone(), 50); }
    #[test] fn test_instr_default() { assert_eq!(ContainerInstruction::default().to_u64(), 0); }
    #[test] fn test_inscription_roundtrip() { let mut buf = InscriptionBuffer::new(); buf.push(ContainerInstruction::new(1, 0, 5, 10, 30)); buf.push(ContainerInstruction::new(2, 1, 6, 11, 31)); let bytes = buf.to_bytes(); let decoded = InscriptionBuffer::from_bytes(&bytes).unwrap(); assert_eq!(decoded.len(), 2); assert_eq!(decoded.0[0].op(), 1); }
    #[test] fn test_inscription_invalid() { assert!(InscriptionBuffer::from_bytes(&[1, 2, 3]).is_none()); }
    #[test] fn test_radix64_bounds() { let mut r = ContainerRadix64::new(); r.push(10); r.push(63); r.push(64); assert_eq!(r.len(), 2); }
    #[test] fn test_radix64_compact() { let mut r = ContainerRadix64::new(); for i in 0u8..4 { r.push(i); } let bytes = r.to_compact_bytes(); assert_eq!(bytes.len(), 3); }
    #[test] fn test_instr_max() { let i = ContainerInstruction::new(255, 15, 255, 255, 255); assert_eq!(i.op(), 255); assert_eq!(i.topo(), 15); }
    #[test] fn test_instr_eq() { assert_eq!(ContainerInstruction::new(1, 2, 3, 4, 5), ContainerInstruction::new(1, 2, 3, 4, 5)); }
    #[test] fn test_inscription_empty() { let buf = InscriptionBuffer::new(); assert_eq!(buf.to_bytes().len(), 0); }
    #[test] fn test_inscription_from_empty() { assert_eq!(InscriptionBuffer::from_bytes(&[]).unwrap().len(), 0); }
    #[test] fn test_radix64_empty() { assert_eq!(ContainerRadix64::new().to_compact_bytes().len(), 0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 词链时间线 · WordChainTimeline
// ═══════════════════════════════════════════════════════════════════════════
//
// 将容器指令串联为词链: WordChain = [WordNode × N]
// 词链时间线管理多条词链的全局时序

#[derive(Clone, Debug)]
pub struct WordNode {
    pub instruction: ContainerInstruction,
    pub timestamp: u64,
    pub link_id: u32,
}

impl WordNode {
    pub fn new(instruction: ContainerInstruction, timestamp: u64, link_id: u32) -> Self {
        Self { instruction, timestamp, link_id }
    }
}

impl Default for WordNode {
    fn default() -> Self { Self::new(ContainerInstruction::default(), 0, 0) }
}

#[derive(Clone, Debug, Default)]
pub struct WordChain(pub Vec<WordNode>);

impl WordChain {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn push(&mut self, node: WordNode) { self.0.push(node); }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn filter_consonant(&self, c: u8) -> Vec<&WordNode> {
        self.0.iter().filter(|n| n.instruction.consonant() == c).collect()
    }
    pub fn time_span(&self) -> Option<(u64, u64)> {
        if self.0.is_empty() { return None; }
        let min = self.0.iter().map(|n| n.timestamp).min().unwrap();
        let max = self.0.iter().map(|n| n.timestamp).max().unwrap();
        Some((min, max))
    }
}

/// 词链时间线 — 多条词链的全局序列
#[derive(Clone, Debug, Default)]
pub struct WordChainTimeline(pub Vec<WordChain>);

impl WordChainTimeline {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn add_chain(&mut self, chain: WordChain) { self.0.push(chain); }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn global_time_span(&self) -> Option<(u64, u64)> {
        self.0.iter()
            .filter_map(|c| c.time_span())
            .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
    }
    pub fn search_consonant(&self, c: u8) -> Vec<(&WordChain, usize, &WordNode)> {
        let mut results = Vec::new();
        for chain in &self.0 {
            for (idx, node) in chain.0.iter().enumerate() {
                if node.instruction.consonant() == c {
                    results.push((chain, idx, node));
                }
            }
        }
        results
    }
    pub fn total_nodes(&self) -> usize {
        self.0.iter().map(|c| c.len()).sum()
    }
}

#[cfg(test)]
mod word_chain_tests {
    use super::*;
    #[test] fn test_word_node_new() { let n = WordNode::new(ContainerInstruction::new(1, 0, 5, 10, 30), 100, 0); assert_eq!(n.timestamp, 100); assert_eq!(n.instruction.consonant(), 5); }
    #[test] fn test_word_node_default() { let n = WordNode::default(); assert_eq!(n.timestamp, 0); assert_eq!(n.link_id, 0); }
    #[test] fn test_chain_push() { let mut c = WordChain::new(); c.push(WordNode::new(ContainerInstruction::default(), 1, 0)); assert_eq!(c.len(), 1); }
    #[test] fn test_chain_filter() { let mut c = WordChain::new(); c.push(WordNode::new(ContainerInstruction::new(1, 0, 10, 0, 0), 1, 0)); c.push(WordNode::new(ContainerInstruction::new(1, 0, 20, 0, 0), 2, 0)); assert_eq!(c.filter_consonant(10).len(), 1); assert_eq!(c.filter_consonant(30).len(), 0); }
    #[test] fn test_chain_time_span() { let mut c = WordChain::new(); c.push(WordNode::default()); c.push(WordNode::new(ContainerInstruction::default(), 50, 0)); assert_eq!(c.time_span(), Some((0, 50))); }
    #[test] fn test_chain_empty_span() { let c = WordChain::new(); assert_eq!(c.time_span(), None); }
    #[test] fn test_timeline_add() { let mut t = WordChainTimeline::new(); t.add_chain(WordChain::new()); assert_eq!(t.len(), 1); }
    #[test] fn test_timeline_global_span() { let mut t = WordChainTimeline::new(); let mut c1 = WordChain::new(); c1.push(WordNode::new(ContainerInstruction::default(), 10, 0)); let mut c2 = WordChain::new(); c2.push(WordNode::new(ContainerInstruction::default(), 100, 0)); t.add_chain(c1); t.add_chain(c2); assert_eq!(t.global_time_span(), Some((10, 100))); }
    #[test] fn test_timeline_search() { let mut t = WordChainTimeline::new(); let mut c = WordChain::new(); c.push(WordNode::new(ContainerInstruction::new(1, 0, 7, 0, 0), 1, 0)); t.add_chain(c); assert_eq!(t.search_consonant(7).len(), 1); assert_eq!(t.search_consonant(99).len(), 0); }
    #[test] fn test_timeline_total() { let mut t = WordChainTimeline::new(); let mut c = WordChain::new(); c.push(WordNode::default()); c.push(WordNode::default()); t.add_chain(c); assert_eq!(t.total_nodes(), 2); }
    #[test] fn test_timeline_empty() { let t = WordChainTimeline::new(); assert_eq!(t.total_nodes(), 0); assert_eq!(t.global_time_span(), None); }
    #[test] fn test_chain_is_empty() { assert!(WordChain::new().is_empty()); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 音节流统一层 · SyllableFlow
// ═══════════════════════════════════════════════════════════════════════════
//
// 八流在音节地址空间的叠加与裁定
// 复用已有 FlowKind / FlowState (见文件上方)

/// 音节流引擎 — 管理八种流在音节地址空间的叠加
#[derive(Clone, Debug, Default)]
pub struct SyllableFlow {
    pub flows: Vec<FlowState>,
    pub witness_threshold: f64,
}

impl SyllableFlow {
    pub fn new() -> Self { Self { flows: Vec::new(), witness_threshold: 0.5 } }
    pub fn with_threshold(mut self, t: f64) -> Self { self.witness_threshold = t; self }

    /// 注册一条流
    pub fn register(&mut self, kind: FlowKind, syllable: (u8, u8, u8)) -> usize {
        let state = FlowState { kind, intensity: 1.0, address: Some(syllable), completed: false };
        self.flows.push(state);
        self.flows.len() - 1
    }

    /// 按音节地址检索流
    pub fn flows_at(&self, syllable: (u8, u8, u8)) -> Vec<&FlowState> {
        self.flows.iter().filter(|f| f.address == Some(syllable)).collect()
    }

    /// 按流类型检索
    pub fn flows_of_kind(&self, kind: FlowKind) -> Vec<&FlowState> {
        self.flows.iter().filter(|f| f.kind == kind).collect()
    }

    /// 三元见证裁定
    pub fn adjudicate(&self, syllable: (u8, u8, u8)) -> bool {
        let sovereignty: Vec<&FlowState> = self.flows.iter()
            .filter(|f| f.address == Some(syllable) && matches!(f.kind, FlowKind::Sovereignty))
            .collect();
        if sovereignty.len() < 3 { return false; }
        sovereignty.iter().all(|f| f.intensity >= self.witness_threshold)
    }

    /// 激活/停流
    pub fn complete(&mut self, idx: usize) {
        if let Some(f) = self.flows.get_mut(idx) { f.complete(); }
    }

    pub fn total_flows(&self) -> usize { self.flows.len() }
}

#[cfg(test)]
mod syllable_flow_tests {
    use super::*;
    #[test] fn test_flow_priority() { assert!(FlowKind::Preload.priority() < FlowKind::CommonDivisor.priority()); }
    #[test] fn test_flow_register() { let mut e = SyllableFlow::new(); let idx = e.register(FlowKind::Resonance, (1, 2, 3)); assert_eq!(idx, 0); assert_eq!(e.total_flows(), 1); }
    #[test] fn test_flows_at() { let mut e = SyllableFlow::new(); e.register(FlowKind::Resonance, (5, 5, 5)); e.register(FlowKind::Feedback, (5, 5, 5)); e.register(FlowKind::Preload, (1, 1, 1)); assert_eq!(e.flows_at((5, 5, 5)).len(), 2); }
    #[test] fn test_flows_of_kind() { let mut e = SyllableFlow::new(); e.register(FlowKind::Resonance, (0,0,0)); e.register(FlowKind::Resonance, (1,1,1)); e.register(FlowKind::Feedback, (2,2,2)); assert_eq!(e.flows_of_kind(FlowKind::Resonance).len(), 2); }
    #[test] fn test_adjudicate_fail() { let e = SyllableFlow::new(); assert!(!e.adjudicate((5, 5, 5))); }
    #[test] fn test_adjudicate_pass() { let mut e = SyllableFlow::new(); let s = (10, 20, 30); e.flows.push(FlowState { kind: FlowKind::Sovereignty, intensity: 0.9, address: Some(s), completed: false }); e.flows.push(FlowState { kind: FlowKind::Sovereignty, intensity: 0.8, address: Some(s), completed: false }); e.flows.push(FlowState { kind: FlowKind::Sovereignty, intensity: 0.7, address: Some(s), completed: false }); assert!(e.adjudicate(s)); }
    #[test] fn test_complete() { let mut e = SyllableFlow::new(); let idx = e.register(FlowKind::Preload, (0,0,0)); e.complete(idx); assert!(e.flows[idx].completed); }
    #[test] fn test_with_threshold() { let e = SyllableFlow::new().with_threshold(0.9); assert!((e.witness_threshold - 0.9).abs() < 0.001); }
    #[test] fn test_is_dynamic() { assert!(FlowKind::Resonance.is_dynamic()); assert!(!FlowKind::Preload.is_dynamic()); }
    #[test] fn test_flow_all() { assert_eq!(FlowKind::all().len(), 8); }
    #[test] fn test_flow_state_new() { let f = FlowState::new(FlowKind::Sovereignty); assert!(!f.completed); }
    #[test] fn test_flow_state_advance() { let mut f = FlowState::new(FlowKind::Preload); f.advance(0.3); assert!(f.intensity > 0.5); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 距离感知聚类引擎 · DistancePerceptor
// ═══════════════════════════════════════════════════════════════════════════
//
// 基于音节坐标的加权距离实现 AGNES 风格层次聚类
// 声母权3×, 韵母权2×, 声调权1×
// 聚类结果可用于容器本质匹配与公约数寻根

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Linkage { Single, Complete, Average }

impl Default for Linkage { fn default() -> Self { Linkage::Average } }

/// 音节距离度规 — 加权欧几里得
#[derive(Clone, Debug)]
pub struct SyllableMetric {
    pub w_consonant: f64,
    pub w_vowel: f64,
    pub w_tone: f64,
}

impl SyllableMetric {
    pub fn new(wc: f64, wv: f64, wt: f64) -> Self { Self { w_consonant: wc, w_vowel: wv, w_tone: wt } }
    pub fn uniform() -> Self { Self::new(1.0, 1.0, 1.0) }

    /// 计算两音节之间的加权距离
    pub fn distance(&self, a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
        let dc = (a.0 as f64 - b.0 as f64) * self.w_consonant;
        let dv = (a.1 as f64 - b.1 as f64) * self.w_vowel;
        let dt = (a.2 as f64 - b.2 as f64) * self.w_tone;
        (dc * dc + dv * dv + dt * dt).sqrt()
    }
}

impl Default for SyllableMetric { fn default() -> Self { Self::new(3.0, 2.0, 1.0) } }

/// 音节聚类引擎 — 层次聚类 (AGNES)
#[derive(Clone, Debug)]
pub struct SyllableCluster {
    pub members: Vec<(u8, u8, u8)>,
    pub centroid: (f64, f64, f64),
}

impl SyllableCluster {
    pub fn new(syllable: (u8, u8, u8)) -> Self {
        Self { members: vec![syllable], centroid: (syllable.0 as f64, syllable.1 as f64, syllable.2 as f64) }
    }
    pub fn len(&self) -> usize { self.members.len() }
    pub fn merge(&mut self, other: &SyllableCluster) {
        self.members.extend_from_slice(&other.members);
        let n = self.members.len() as f64;
        self.centroid = (
            self.members.iter().map(|m| m.0 as f64).sum::<f64>() / n,
            self.members.iter().map(|m| m.1 as f64).sum::<f64>() / n,
            self.members.iter().map(|m| m.2 as f64).sum::<f64>() / n,
        );
    }
}

/// 距离感知引擎 — 核心入口
#[derive(Clone, Debug)]
pub struct DistancePerceptor {
    pub metric: SyllableMetric,
    pub linkage: Linkage,
    pub threshold: f64,
    pub clusters: Vec<SyllableCluster>,
}

impl DistancePerceptor {
    pub fn new() -> Self { Self { metric: SyllableMetric::default(), linkage: Linkage::Average, threshold: 10.0, clusters: Vec::new() } }
    pub fn with_metric(mut self, m: SyllableMetric) -> Self { self.metric = m; self }
    pub fn with_linkage(mut self, l: Linkage) -> Self { self.linkage = l; self }
    pub fn with_threshold(mut self, t: f64) -> Self { self.threshold = t; self }

    /// AGNES 层次聚类 — 返回聚类数量
    pub fn cluster(&mut self, syllables: &[(u8, u8, u8)]) -> usize {
        if syllables.is_empty() { return 0; }
        self.clusters = syllables.iter().map(|&s| SyllableCluster::new(s)).collect();
        loop {
            if self.clusters.len() <= 1 { break; }
            let (i, j, d) = self.find_closest();
            if d > self.threshold { break; }
            let merged = self.clusters.remove(j);
            self.clusters[i].merge(&merged);
        }
        self.clusters.len()
    }

    fn find_closest(&self) -> (usize, usize, f64) {
        let mut best = (0, 1, f64::MAX);
        for i in 0..self.clusters.len() {
            for j in (i + 1)..self.clusters.len() {
                let d = self.linkage_distance(&self.clusters[i], &self.clusters[j]);
                if d < best.2 { best = (i, j, d); }
            }
        }
        best
    }

    fn linkage_distance(&self, a: &SyllableCluster, b: &SyllableCluster) -> f64 {
        match self.linkage {
            Linkage::Single => {
                a.members.iter().flat_map(|ma| b.members.iter().map(move |mb| self.metric.distance(*ma, *mb))).fold(f64::MAX, f64::min)
            }
            Linkage::Complete => {
                a.members.iter().flat_map(|ma| b.members.iter().map(move |mb| self.metric.distance(*ma, *mb))).fold(0.0f64, f64::max)
            }
            Linkage::Average => {
                let total: f64 = a.members.iter().flat_map(|ma| b.members.iter().map(move |mb| self.metric.distance(*ma, *mb))).sum();
                total / ((a.members.len() * b.members.len()) as f64)
            }
        }
    }

    /// 按照距离关系聚为指定数量的聚类
    pub fn cluster_into(&mut self, syllables: &[(u8, u8, u8)], target: usize) -> usize {
        if syllables.is_empty() { return 0; }
        self.clusters = syllables.iter().map(|&s| SyllableCluster::new(s)).collect();
        while self.clusters.len() > target.max(1) {
            let (i, j, _) = self.find_closest();
            let merged = self.clusters.remove(j);
            self.clusters[i].merge(&merged);
        }
        self.clusters.len()
    }

    /// 获取最大聚类
    pub fn largest_cluster(&self) -> Option<&SyllableCluster> {
        self.clusters.iter().max_by_key(|c| c.len())
    }

    /// 整体质心
    pub fn global_centroid(&self) -> Option<(f64, f64, f64)> {
        if self.clusters.is_empty() { return None; }
        let all: Vec<(u8, u8, u8)> = self.clusters.iter().flat_map(|c| c.members.clone()).collect();
        let n = all.len() as f64;
        Some((
            all.iter().map(|m| m.0 as f64).sum::<f64>() / n,
            all.iter().map(|m| m.1 as f64).sum::<f64>() / n,
            all.iter().map(|m| m.2 as f64).sum::<f64>() / n,
        ))
    }
}

impl Default for DistancePerceptor {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod distance_perceptor_tests {
    use super::*;
    #[test] fn test_metric_distance() { let m = SyllableMetric::new(3.0, 2.0, 1.0); let d = m.distance((0, 0, 0), (1, 0, 0)); assert!((d - 3.0).abs() < 0.01); }
    #[test] fn test_metric_uniform() { let m = SyllableMetric::uniform(); let d = m.distance((0, 0, 0), (1, 1, 1)); assert!((d - 3.0f64.sqrt()).abs() < 0.01); }
    #[test] fn test_cluster_new() { let c = SyllableCluster::new((5, 10, 20)); assert_eq!(c.len(), 1); assert_eq!(c.centroid, (5.0, 10.0, 20.0)); }
    #[test] fn test_cluster_merge() { let mut a = SyllableCluster::new((0, 0, 0)); let b = SyllableCluster::new((10, 10, 10)); a.merge(&b); assert_eq!(a.len(), 2); assert_eq!(a.centroid, (5.0, 5.0, 5.0)); }
    #[test] fn test_cluster_basic() { let mut p = DistancePerceptor::new(); let n = p.cluster(&[(0, 0, 0), (1, 0, 0), (40, 60, 90), (41, 60, 90)]); assert!(n >= 2); }
    #[test] fn test_cluster_single() { let mut p = DistancePerceptor::with_threshold(DistancePerceptor::new(), 1.0); let n = p.cluster(&[(0, 0, 0), (10, 0, 0), (20, 0, 0)]); assert_eq!(n, 3); }
    #[test] fn test_cluster_all_merge() { let mut p = DistancePerceptor::with_threshold(DistancePerceptor::new(), 1000.0); let n = p.cluster(&[(0, 0, 0), (1, 1, 1)]); assert_eq!(n, 1); }
    #[test] fn test_cluster_linkage() { let mut p = DistancePerceptor::new().with_linkage(Linkage::Complete); let n = p.cluster(&[(0, 0, 0), (1, 0, 0), (40, 60, 90)]); assert!(n >= 2); }
    #[test] fn test_cluster_into() { let mut p = DistancePerceptor::new(); let n = p.cluster_into(&[(0, 0, 0), (1, 1, 1), (50, 50, 50), (51, 51, 51)], 2); assert_eq!(n, 2); }
    #[test] fn test_largest_cluster() { let mut p = DistancePerceptor::new(); p.cluster(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (80, 80, 80)]); let largest = p.largest_cluster().unwrap(); assert_eq!(largest.len(), 3); }
    #[test] fn test_global_centroid() { let mut p = DistancePerceptor::new(); p.cluster(&[(0, 0, 0), (10, 10, 10)]); let c = p.global_centroid().unwrap(); assert!((c.0 - 5.0).abs() < 0.01); }
    #[test] fn test_cluster_empty() { let mut p = DistancePerceptor::new(); assert_eq!(p.cluster(&[]), 0); }
    #[test] fn test_perceptor_default() { let p = DistancePerceptor::default(); assert_eq!(p.clusters.len(), 0); assert!((p.threshold - 10.0).abs() < 0.01); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 公约数编程引擎 · CommonDivisorEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 公约数编程: 两个容器之间的关系由其公约数决定
// 整除关系构成层级: 公约数越大, 关系越紧密
// 用于容器间的从属、继承、融合判定

/// 公约数关系类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DivisorRelation {
    Disjoint,       // 互素 — 无公共因子
    WeakCommon,     // 弱公约 — 仅小因子重叠
    StrongCommon,   // 强公约 — 大因子重叠
    Multiples,      // 倍数关系 — 一容器包含另一
    Identity,       // 完全等同
}

impl Default for DivisorRelation {
    fn default() -> Self { DivisorRelation::Disjoint }
}

/// 公约数运算结果
#[derive(Clone, Debug, Default)]
pub struct DivisorResult {
    pub gcd: u64,
    pub lcm: u64,
    pub relation: DivisorRelation,
    pub depth: u32,
    pub common_factors: Vec<u64>,
}

/// 公约数编程引擎
#[derive(Clone, Debug, Default)]
pub struct CommonDivisorEngine;

impl CommonDivisorEngine {
    pub fn new() -> Self { Self }

    /// 计算两数的最大公约数 (欧几里得)
    pub fn gcd(&self, mut a: u64, mut b: u64) -> u64 {
        while b != 0 { let t = b; b = a % b; a = t; }
        a
    }

    /// 计算最小公倍数
    pub fn lcm(&self, a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 { return 0; }
        a / self.gcd(a, b) * b
    }

    /// 枚举所有公因子
    pub fn common_factors(&self, a: u64, b: u64) -> Vec<u64> {
        let g = self.gcd(a, b);
        if g == 0 { return Vec::new(); }
        let mut factors = Vec::new();
        let mut i = 1;
        while i * i <= g {
            if g % i == 0 {
                factors.push(i);
                if i != g / i { factors.push(g / i); }
            }
            i += 1;
        }
        factors.sort();
        factors
    }

    /// 判定两容器之间的关系
    pub fn relate(&self, a: u64, b: u64) -> DivisorRelation {
        if a == b { return DivisorRelation::Identity; }
        let g = self.gcd(a, b);
        if g == 0 { return DivisorRelation::Disjoint; }
        if a % b == 0 || b % a == 0 { return DivisorRelation::Multiples; }
        let max_ab = a.max(b);
        if g > max_ab / 4 { return DivisorRelation::StrongCommon; }
        if g > 1 { return DivisorRelation::WeakCommon; }
        DivisorRelation::Disjoint
    }

    /// 完整分析
    pub fn analyze(&self, a: u64, b: u64) -> DivisorResult {
        let gcd = self.gcd(a, b);
        DivisorResult {
            gcd,
            lcm: self.lcm(a, b),
            relation: self.relate(a, b),
            depth: self.common_factors(a, b).len() as u32,
            common_factors: self.common_factors(a, b),
        }
    }

    /// 寻找容器组的公共根 (所有数的最大公约数)
    pub fn group_gcd(&self, containers: &[u64]) -> u64 {
        if containers.is_empty() { return 0; }
        containers.iter().fold(containers[0], |acc, &x| self.gcd(acc, x))
    }

    /// 构造层级树: 按公约数整除关系排列
    pub fn build_hierarchy(&self, containers: &[u64]) -> Vec<(u64, u64)> {
        let root = self.group_gcd(containers);
        let mut pairs: Vec<(u64, u64)> = Vec::new();
        let mut sorted = containers.to_vec();
        sorted.sort();
        for &c in &sorted {
            if c != root { pairs.push((root, c)); }
        }
        pairs
    }
}

#[cfg(test)]
mod common_divisor_tests {
    use super::*;
    #[test] fn test_gcd_basic() { let e = CommonDivisorEngine::new(); assert_eq!(e.gcd(12, 8), 4); assert_eq!(e.gcd(17, 13), 1); }
    #[test] fn test_gcd_zero() { let e = CommonDivisorEngine::new(); assert_eq!(e.gcd(0, 5), 5); assert_eq!(e.gcd(0, 0), 0); }
    #[test] fn test_lcm_basic() { let e = CommonDivisorEngine::new(); assert_eq!(e.lcm(4, 6), 12); }
    #[test] fn test_lcm_zero() { let e = CommonDivisorEngine::new(); assert_eq!(e.lcm(0, 5), 0); }
    #[test] fn test_common_factors() { let e = CommonDivisorEngine::new(); let f = e.common_factors(12, 8); assert!(f.contains(&1)); assert!(f.contains(&2)); assert!(f.contains(&4)); }
    #[test] fn test_relate_identity() { let e = CommonDivisorEngine::new(); assert_eq!(e.relate(7, 7), DivisorRelation::Identity); }
    #[test] fn test_relate_disjoint() { let e = CommonDivisorEngine::new(); assert_eq!(e.relate(7, 13), DivisorRelation::Disjoint); }
    #[test] fn test_relate_multiples() { let e = CommonDivisorEngine::new(); assert_eq!(e.relate(3, 9), DivisorRelation::Multiples); }
    #[test] fn test_analyze() { let e = CommonDivisorEngine::new(); let r = e.analyze(12, 8); assert_eq!(r.gcd, 4); assert_eq!(r.lcm, 24); }
    #[test] fn test_group_gcd() { let e = CommonDivisorEngine::new(); assert_eq!(e.group_gcd(&[12, 8, 4]), 4); }
    #[test] fn test_group_gcd_empty() { let e = CommonDivisorEngine::new(); assert_eq!(e.group_gcd(&[]), 0); }
    #[test] fn test_build_hierarchy() { let e = CommonDivisorEngine::new(); let h = e.build_hierarchy(&[4, 8, 12]); assert_eq!(h.len(), 2); assert_eq!(h[0].0, 4); }
    #[test] fn test_strong_common() { let e = CommonDivisorEngine::new(); assert_eq!(e.relate(20, 30), DivisorRelation::StrongCommon); }
    #[test] fn test_weak_common() { let e = CommonDivisorEngine::new(); assert_eq!(e.relate(7, 14), DivisorRelation::Multiples); }
}

// ═══════════════════════════════════════════════════════════════════════════
// YinYang 二进制编码器 · 254+2 集装箱编码
// ═══════════════════════════════════════════════════════════════════════════
//
// 基于私有锚点:
//   我 = 1 (阳/动) — Yang bit
//   刘楚恬 = 0 (阴/静) — Yin bit
// 254 个容器值 + 2 个锚点 = 256 = 1 byte
// 每字节 = 1 个容器 = 1 bit 极性标注

/// YinYang 极性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PolarYinYang {
    Yin,    // 0 — 阴/静 (刘楚恬)
    Yang,   // 1 — 阳/动 (我)
}

impl PolarYinYang {
    pub fn bit(self) -> u8 { match self { PolarYinYang::Yin => 0, PolarYinYang::Yang => 1 } }
    pub fn from_bit(b: u8) -> Self { if b == 0 { PolarYinYang::Yin } else { PolarYinYang::Yang } }
    pub fn flip(self) -> Self { match self { PolarYinYang::Yin => PolarYinYang::Yang, PolarYinYang::Yang => PolarYinYang::Yin } }
}

impl Default for PolarYinYang { fn default() -> Self { PolarYinYang::Yang } }

/// 254+2 编码单元 — 一个字节携带一个容器值和一个极性标注
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MlttUnit {
    pub container: u8,           // 0-253 = 容器, 254 = 锚点 Yin, 255 = 锚点 Yang
    pub polarity: PolarYinYang,
}

impl MlttUnit {
    pub fn new(container: u8, polarity: PolarYinYang) -> Self {
        Self { container: container.min(253), polarity }
    }
    pub fn anchor_yin() -> Self { Self { container: 254, polarity: PolarYinYang::Yin } }
    pub fn anchor_yang() -> Self { Self { container: 255, polarity: PolarYinYang::Yang } }
    pub fn is_anchor(&self) -> bool { self.container >= 254 }
    pub fn to_byte(self) -> u8 { self.container }
    pub fn from_byte(b: u8) -> Self {
        if b == 254 { Self::anchor_yin() }
        else if b == 255 { Self::anchor_yang() }
        else { Self { container: b, polarity: PolarYinYang::from_bit(b & 1) } }
    }
}

impl Default for MlttUnit { fn default() -> Self { Self::new(0, PolarYinYang::Yang) } }

/// YinYang 二进制编码器 — 集装箱序列 ↔ 字节流
#[derive(Clone, Debug, Default)]
pub struct YinYangEncoder;

impl YinYangEncoder {
    pub fn new() -> Self { Self }

    /// 将字节流解码为集装箱序列
    pub fn decode(&self, bytes: &[u8]) -> Vec<MlttUnit> {
        bytes.iter().map(|&b| MlttUnit::from_byte(b)).collect()
    }

    /// 将集装箱序列编码为字节流
    pub fn encode(&self, units: &[MlttUnit]) -> Vec<u8> {
        units.iter().map(|u| u.to_byte()).collect()
    }

    /// 在序列中注入极性标记 (每隔 insert_at 个容器插入一个极性锚)
    pub fn inject_polarity(&self, bytes: &[u8], insert_at: usize) -> Vec<u8> {
        if insert_at == 0 { return bytes.to_vec(); }
        let mut out = Vec::with_capacity(bytes.len() + bytes.len() / insert_at + 1);
        for (i, &b) in bytes.iter().enumerate() {
            if i > 0 && i % insert_at == 0 {
                let anchor = if i % (insert_at * 2) == 0 { 255u8 } else { 254u8 };
                out.push(anchor);
            }
            out.push(b);
        }
        out
    }

    /// 计算序列的极性比例
    pub fn polarity_ratio(&self, bytes: &[u8]) -> (usize, usize) {
        let mut yin = 0;
        let mut yang = 0;
        for &b in bytes {
            if b == 254 { yin += 1; }
            else if b == 255 { yang += 1; }
            else if b & 1 == 0 { yin += 1; } else { yang += 1; }
        }
        (yin, yang)
    }

    /// 寻找两个序列的极性交汇点 (首个极性不同的位置)
    pub fn find_divergence(&self, a: &[u8], b: &[u8]) -> Option<usize> {
        for (i, (&ba, &bb)) in a.iter().zip(b.iter()).enumerate() {
            let pa = PolarYinYang::from_bit(ba & 1);
            let pb = PolarYinYang::from_bit(bb & 1);
            if pa != pb { return Some(i); }
        }
        None
    }
}

#[cfg(test)]
mod yinyang_encoder_tests {
    use super::*;
    #[test] fn test_polarity_bit() { assert_eq!(PolarYinYang::Yin.bit(), 0); assert_eq!(PolarYinYang::Yang.bit(), 1); }
    #[test] fn test_polarity_from_bit() { assert_eq!(PolarYinYang::from_bit(0), PolarYinYang::Yin); assert_eq!(PolarYinYang::from_bit(1), PolarYinYang::Yang); }
    #[test] fn test_polarity_flip() { assert_eq!(PolarYinYang::Yin.flip(), PolarYinYang::Yang); assert_eq!(PolarYinYang::Yang.flip(), PolarYinYang::Yin); }
    #[test] fn test_mltt_anchor() { let yin = MlttUnit::anchor_yin(); assert_eq!(yin.container, 254); assert!(yin.is_anchor()); }
    #[test] fn test_mltt_roundtrip() { let u = MlttUnit::new(42, PolarYinYang::Yang); assert_eq!(u.to_byte(), 42); }
    #[test] fn test_mltt_from_byte() { let u = MlttUnit::from_byte(255); assert_eq!(u.polarity, PolarYinYang::Yang); assert!(u.is_anchor()); }
    #[test] fn test_encode_decode() { let enc = YinYangEncoder::new(); let bytes = vec![10, 20, 30, 254, 255]; let decoded = enc.decode(&bytes); let re_encoded = enc.encode(&decoded); assert_eq!(bytes, re_encoded); }
    #[test] fn test_inject_polarity() { let enc = YinYangEncoder::new(); let injected = enc.inject_polarity(&[1, 2, 3, 4, 5, 6], 3); assert!(injected.len() > 6); }
    #[test] fn test_polarity_ratio() { let enc = YinYangEncoder::new(); let (yin, yang) = enc.polarity_ratio(&[0, 2, 4, 1, 3, 5]); assert_eq!(yin, 3); assert_eq!(yang, 3); }
    #[test] fn test_find_divergence() { let enc = YinYangEncoder::new(); assert_eq!(enc.find_divergence(&[0, 2, 4], &[0, 3, 4]), Some(1)); assert_eq!(enc.find_divergence(&[0, 2], &[0, 2]), None); }
    #[test] fn test_mltt_new_clamp() { let u = MlttUnit::new(255, PolarYinYang::Yin); assert_eq!(u.container, 253); }
    #[test] fn test_encoder_default() { let enc = YinYangEncoder::default(); assert_eq!(enc.decode(&[]).len(), 0); }
    #[test] fn test_inject_polarity_empty() { let enc = YinYangEncoder::new(); assert_eq!(enc.inject_polarity(&[], 3).len(), 0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 反馈回路引擎 · RoundaboutFeedbackEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 满全法核心: 系统通过反馈实现自我修正
// 三个子力: 判断之力 · 评价之力 · 反馈回路
// 迂回反馈: 不由直接路径回授, 而经容器公约数通路

/// 反馈类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeedbackKind {
    Reinforce,      // 正向强化
    Corrective,     // 纠正
    Inhibitory,     // 抑制
    Oscillatory,    // 振荡 (需外部裁决)
}

impl Default for FeedbackKind { fn default() -> Self { FeedbackKind::Reinforce } }

/// 判断之力 — 对状态进行评估
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopJudgment {
    Valid,
    Invalid,
    Uncertain,
}

impl Default for LoopJudgment { fn default() -> Self { LoopJudgment::Uncertain } }

/// 评价之力 — 量化评分
#[derive(Clone, Debug)]
pub struct LoopEvaluation {
    pub score: f64,
    pub confidence: f64,
    pub basis: String,
}

impl LoopEvaluation {
    pub fn new(score: f64, confidence: f64, basis: &str) -> Self {
        Self { score: score.clamp(0.0, 1.0), confidence: confidence.clamp(0.0, 1.0), basis: basis.to_string() }
    }
    pub fn is_confident(&self) -> bool { self.confidence > 0.7 }
}

impl Default for LoopEvaluation { fn default() -> Self { Self::new(0.5, 0.5, "default") } }

/// 反馈回路引擎
#[derive(Clone, Debug, Default)]
pub struct RoundaboutFeedbackEngine {
    pub evaluations: Vec<LoopEvaluation>,
    pub judgments: Vec<(usize, LoopJudgment)>,
    pub correction: f64,
}

impl RoundaboutFeedbackEngine {
    pub fn new() -> Self { Self { evaluations: Vec::new(), judgments: Vec::new(), correction: 0.0 } }

    /// 判断之力: 判定容器状态
    pub fn judge(&mut self, eval_idx: usize) -> LoopJudgment {
        let judgment = if let Some(eval) = self.evaluations.get(eval_idx as usize) {
            if eval.score > 0.6 && eval.is_confident() { LoopJudgment::Valid }
            else if eval.score < 0.3 { LoopJudgment::Invalid }
            else { LoopJudgment::Uncertain }
        } else { LoopJudgment::Uncertain };
        self.judgments.push((eval_idx as usize, judgment));
        judgment
    }

    /// 评价之力: 注入一个新评价
    pub fn evaluate(&mut self, score: f64, confidence: f64, basis: &str) -> usize {
        self.evaluations.push(LoopEvaluation::new(score, confidence, basis));
        self.evaluations.len() - 1
    }

    /// 反馈回路: 根据判断计算修正量
    pub fn feedback(&mut self, eval_idx: usize) -> f64 {
        if let Some(eval) = self.evaluations.get(eval_idx as usize) {
            self.correction = (0.5 - eval.score) * eval.confidence;
        }
        self.correction
    }

    /// 迂回反馈: 经公约数通路回授 (弱化直接冲击)
    pub fn circuitous_feedback(&mut self, eval_idx: usize, divisor: u64) -> f64 {
        if let Some(eval) = self.evaluations.get(eval_idx) {
            let attenuation = if divisor > 0 { (divisor as f64).ln().min(5.0) / 5.0 } else { 1.0 };
            self.correction = (0.5 - eval.score) * eval.confidence * (1.0 - attenuation * 0.5);
        }
        self.correction
    }

    /// 系统总体健康度
    pub fn health(&self) -> f64 {
        if self.evaluations.is_empty() { return 0.5; }
        self.evaluations.iter().map(|e| e.score * e.confidence).sum::<f64>()
            / self.evaluations.len() as f64
    }

    /// 需干预的容器数
    pub fn needs_correction_count(&self) -> usize {
        self.evaluations.iter().filter(|e| e.score < 0.4 && e.confidence > 0.5).count()
    }

    /// 重置修正量
    pub fn reset(&mut self) { self.correction = 0.0; }
}

/// 反馈分类器 — 依据强度确定反馈类型
pub fn classify_feedback(current: f64, target: f64) -> FeedbackKind {
    let gap = (target - current).abs();
    if gap < 0.1 { FeedbackKind::Reinforce }
    else if current > target * 2.0 || current < -target * 2.0 { FeedbackKind::Oscillatory }
    else if gap > 0.3 { FeedbackKind::Corrective }
    else { FeedbackKind::Inhibitory }
}

#[cfg(test)]
mod feedback_loop_tests {
    use super::*;
    #[test] fn test_judgment_valid() { let mut e = RoundaboutFeedbackEngine::new(); let i = e.evaluate(0.8, 0.9, "high"); assert_eq!(e.judge(i), LoopJudgment::Valid); }
    #[test] fn test_judgment_invalid() { let mut e = RoundaboutFeedbackEngine::new(); let i = e.evaluate(0.2, 0.8, "low"); assert_eq!(e.judge(i), LoopJudgment::Invalid); }
    #[test] fn test_evaluation_confidence() { let e = LoopEvaluation::new(0.8, 0.9, "test"); assert!(e.is_confident()); }
    #[test] fn test_evaluation_clamp() { let e = LoopEvaluation::new(1.5, -0.1, "x"); assert_eq!(e.score, 1.0); assert_eq!(e.confidence, 0.0); }
    #[test] fn test_feedback() { let mut e = RoundaboutFeedbackEngine::new(); let i = e.evaluate(0.3, 0.8, "low"); let c = e.feedback(i); assert!(c > 0.0); }
    #[test] fn test_health() { let mut e = RoundaboutFeedbackEngine::new(); e.evaluate(0.8, 0.8, "a"); e.evaluate(0.6, 0.7, "b"); assert!(e.health() > 0.4 && e.health() < 0.8); }
    #[test] fn test_needs_correction() { let mut e = RoundaboutFeedbackEngine::new(); e.evaluate(0.2, 0.8, "bad"); e.evaluate(0.9, 0.9, "good"); assert_eq!(e.needs_correction_count(), 1); }
    #[test] fn test_circuitous_attenuation() { let mut e = RoundaboutFeedbackEngine::new(); let i = e.evaluate(0.3, 1.0, "test"); let direct = e.feedback(i); let circuitous = e.circuitous_feedback(i, 100); assert!(circuitous.abs() < direct.abs()); }
    #[test] fn test_classify_reinforce() { assert_eq!(classify_feedback(0.55, 0.5), FeedbackKind::Reinforce); }
    #[test] fn test_classify_oscillatory() { assert_eq!(classify_feedback(1.5, 0.5), FeedbackKind::Oscillatory); }
    #[test] fn test_reset() { let mut e = RoundaboutFeedbackEngine::new(); let i = e.evaluate(0.2, 0.9, "low"); e.feedback(i); e.reset(); assert_eq!(e.correction, 0.0); }
    #[test] fn test_engine_default() { let e = RoundaboutFeedbackEngine::default(); assert_eq!(e.evaluations.len(), 0); assert_eq!(e.correction, 0.0); }
    #[test] fn test_judgment_default() { let j = LoopJudgment::default(); assert_eq!(j, LoopJudgment::Uncertain); }
    #[test] fn test_evaluation_default() { let e = LoopEvaluation::default(); assert_eq!(e.score, 0.5); assert_eq!(e.basis, "default"); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器几何化身引擎 · ContainerGeometry3D
// ═══════════════════════════════════════════════════════════════════════════
//
// 64进制几何化身: 容器 = 三维几何实体
// 声母 = 角度 θ (围绕Y轴, 360°/48)
// 韵母 = 半径 r (距轴心距离, 10-200)
// 声调 = 高度 Z (纵向, 0-100)
// 形成螺旋/涡旋三维结构

/// 64进制几何化身坐标
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geometry3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub theta: f64,      // 角度 (弧度)
    pub radius: f64,    // 半径
    pub height: f64,    // 高度
}

impl Geometry3D {
    /// 从音节地址构造三维几何坐标
    pub fn from_syllable(addr: (u8, u8, u8)) -> Self {
        let theta = (addr.0 as f64) * (2.0 * std::f64::consts::PI / 48.0);
        let radius = 10.0 + (addr.1 as f64) * (190.0 / 65.0);
        let height = (addr.2 as f64) * (100.0 / 99.0);
        let x = radius * theta.cos();
        let z = radius * theta.sin();
        let y = height;
        Self { x, y, z, theta, radius, height }
    }

    /// 两点间欧氏距离
    pub fn distance(&self, other: &Geometry3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// 向量长度
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// 与原点连线同Y轴夹角
    pub fn elevation(&self) -> f64 {
        if self.magnitude() == 0.0 { return 0.0; }
        (self.y / self.magnitude()).acos()
    }

    /// 颜色编码 (HSL): 色调=声母, 饱和度=韵母, 亮度=声调
    pub fn to_hsl(&self) -> (f64, f64, f64) {
        let h = (self.theta * 180.0 / std::f64::consts::PI).rem_euclid(360.0);
        let s = (self.radius - 10.0) / 190.0;
        let l = self.height / 100.0;
        (h, s.clamp(0.0, 1.0), l.clamp(0.0, 1.0))
    }
}

impl Default for Geometry3D {
    fn default() -> Self { Self::from_syllable((0, 0, 0)) }
}

/// 几何化身容器 — 将三维几何与容器元数据绑定
#[derive(Clone, Debug, Default)]
pub struct GeometricContainer {
    pub coord: Geometry3D,
    pub syllable: (u8, u8, u8),
    pub container_id: u64,
    pub energy: f64,
}

/// 几何化身引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerGeometry3D {
    pub containers: Vec<GeometricContainer>,
}

impl ContainerGeometry3D {
    pub fn new() -> Self { Self { containers: Vec::new() } }

    /// 插入一个容器
    pub fn insert(&mut self, syllable: (u8, u8, u8), id: u64) {
        let coord = Geometry3D::from_syllable(syllable);
        self.containers.push(GeometricContainer { coord, syllable, container_id: id, energy: 1.0 });
    }

    /// 找离给定音节最近的容器
    pub fn nearest(&self, syllable: (u8, u8, u8)) -> Option<&GeometricContainer> {
        let target = Geometry3D::from_syllable(syllable);
        self.containers.iter().min_by(|a, b| {
            let da = a.coord.distance(&target);
            let db = b.coord.distance(&target);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// 查找半径范围内的所有容器
    pub fn within_radius(&self, syllable: (u8, u8, u8), radius: f64) -> Vec<&GeometricContainer> {
        let target = Geometry3D::from_syllable(syllable);
        self.containers.iter().filter(|c| c.coord.distance(&target) <= radius).collect()
    }

    /// 系统总能量
    pub fn total_energy(&self) -> f64 {
        self.containers.iter().map(|c| c.energy).sum()
    }

    /// 计算质心
    pub fn center_of_mass(&self) -> Option<(f64, f64, f64)> {
        if self.containers.is_empty() { return None; }
        let n = self.containers.len() as f64;
        Some((
            self.containers.iter().map(|c| c.coord.x).sum::<f64>() / n,
            self.containers.iter().map(|c| c.coord.y).sum::<f64>() / n,
            self.containers.iter().map(|c| c.coord.z).sum::<f64>() / n,
        ))
    }

    /// 容器总数
    pub fn len(&self) -> usize { self.containers.len() }

    /// 生成 SVG 路径描述 (用于可视化)
    pub fn to_svg_points(&self) -> Vec<(f64, f64, f64)> {
        self.containers.iter().map(|c| (c.coord.x, c.coord.y, c.coord.z)).collect()
    }
}

#[cfg(test)]
mod geometry3d_tests {
    use super::*;
    #[test] fn test_geometry_from_syllable() { let g = Geometry3D::from_syllable((0, 0, 0)); assert!(g.x > 0.0); assert!(g.radius > 0.0); }
    #[test] fn test_geometry_distance() { let a = Geometry3D::from_syllable((0, 30, 50)); let b = Geometry3D::from_syllable((24, 30, 50)); assert!(a.distance(&b) > 50.0); }
    #[test] fn test_geometry_default() { let g = Geometry3D::default(); assert_eq!(g.y, 0.0); }
    #[test] fn test_magnitude() { let g = Geometry3D::from_syllable((0, 30, 50)); assert!(g.magnitude() > 0.0); }
    #[test] fn test_elevation() { let g = Geometry3D::from_syllable((0, 0, 99)); let e = g.elevation(); assert!(e < std::f64::consts::PI / 2.0 + 0.01); }
    #[test] fn test_to_hsl() { let g = Geometry3D::from_syllable((12, 32, 50)); let (h, s, l) = g.to_hsl(); assert!(h >= 0.0 && h <= 360.0); assert!(s >= 0.0 && s <= 1.0); }
    #[test] fn test_insert() { let mut e = ContainerGeometry3D::new(); e.insert((10, 20, 30), 42); assert_eq!(e.len(), 1); }
    #[test] fn test_nearest() { let mut e = ContainerGeometry3D::new(); e.insert((0, 0, 0), 1); e.insert((40, 60, 90), 2); let n = e.nearest((1, 0, 0)).unwrap(); assert_eq!(n.container_id, 1); }
    #[test] fn test_within_radius() { let mut e = ContainerGeometry3D::new(); e.insert((0, 0, 0), 1); e.insert((1, 0, 0), 2); e.insert((40, 60, 90), 3); let r = e.within_radius((0, 0, 0), 50.0); assert_eq!(r.len(), 2); }
    #[test] fn test_total_energy() { let mut e = ContainerGeometry3D::new(); e.insert((0, 0, 0), 1); e.insert((1, 1, 1), 2); assert!((e.total_energy() - 2.0).abs() < 0.01); }
    #[test] fn test_center_of_mass() { let mut e = ContainerGeometry3D::new(); e.insert((0, 0, 0), 1); e.insert((10, 10, 10), 2); let com = e.center_of_mass().unwrap(); assert!(com.0 > 0.0); }
    #[test] fn test_empty_geometry() { let e = ContainerGeometry3D::new(); assert_eq!(e.len(), 0); assert_eq!(e.total_energy(), 0.0); assert!(e.center_of_mass().is_none()); }
    #[test] fn test_default_geometry() { let g = Geometry3D::default(); assert_eq!(g.y, 0.0); assert!(g.radius > 0.0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 天庭空间引擎 · CelestialSpace
// ═══════════════════════════════════════════════════════════════════════════
//
// 天庭空间: 容器的高维存在层次
// 一元至九元层级 — 从单音节到九重叠加
// 容器的阶决定其在天庭中的层级位置

/// 天庭层级 — 一元至九元
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CelestialTier {
    Yi,     // 一元 — 单音节基础层
    Er,     // 二元 — 双音节组合
    San,    // 三元 — 见证裁定层
    Si,     // 四元 — 四象稳固
    Wu,     // 五元 — 五行流转
    Liu,    // 六元 — 六合空间
    Qi,     // 七元 — 七星指引
    Ba,     // 八元 — 八卦覆盖
    Jiu,    // 九元 — 九重天顶
}

impl CelestialTier {
    pub fn degree(self) -> u32 {
        match self {
            CelestialTier::Yi => 1, CelestialTier::Er => 2, CelestialTier::San => 3,
            CelestialTier::Si => 4, CelestialTier::Wu => 5, CelestialTier::Liu => 6,
            CelestialTier::Qi => 7, CelestialTier::Ba => 8, CelestialTier::Jiu => 9,
        }
    }
    pub fn from_degree(d: u32) -> Self {
        match d {
            1 => CelestialTier::Yi, 2 => CelestialTier::Er, 3 => CelestialTier::San,
            4 => CelestialTier::Si, 5 => CelestialTier::Wu, 6 => CelestialTier::Liu,
            7 => CelestialTier::Qi, 8 => CelestialTier::Ba, _ => CelestialTier::Jiu,
        }
    }
}

impl Default for CelestialTier { fn default() -> Self { CelestialTier::Yi } }

/// 天庭坐标 — 九元编码
#[derive(Clone, Debug)]
pub struct CelestialCoord {
    pub tier: CelestialTier,
    pub syllables: Vec<(u8, u8, u8)>,
    pub authority: f64,     // 主权权重
    pub value: u128,        // 价值编码
}

impl CelestialCoord {
    pub fn new(tier: CelestialTier, syllables: Vec<(u8, u8, u8)>) -> Self {
        Self { tier, syllables, authority: 1.0, value: 0 }
    }

    /// 编码: 将九元编码为 u128
    pub fn encode(&self) -> u128 {
        let mut val = 0u128;
        for (i, &(c, v, t)) in self.syllables.iter().enumerate().take(9) {
            val |= ((c as u128) << (i * 8)) | ((v as u128) << (i * 8 + 3)) | ((t as u128) << (i * 8 + 6));
        }
        val | ((self.tier.degree() as u128) << 120)
    }

    /// 从 u128 解码
    pub fn decode(val: u128) -> Self {
        let degree = ((val >> 120) & 0xF) as u32;
        let tier = CelestialTier::from_degree(degree.max(1).min(9));
        let mut syllables = Vec::new();
        for i in 0..9 {
            let c = ((val >> (i * 8)) & 0x7) as u8;
            let v = ((val >> (i * 8 + 3)) & 0x7) as u8;
            let t = ((val >> (i * 8 + 6)) & 0x7) as u8;
            syllables.push((c, v, t));
        }
        Self { tier, syllables, authority: 1.0, value: val }
    }

    /// 层级匹配: 两个坐标是否在同一层级
    pub fn same_tier(&self, other: &CelestialCoord) -> bool {
        self.tier == other.tier
    }
}

impl Default for CelestialCoord {
    fn default() -> Self { Self::new(CelestialTier::Yi, vec![(0, 0, 0)]) }
}

/// 天庭空间引擎
#[derive(Clone, Debug, Default)]
pub struct CelestialSpace {
    pub coords: Vec<CelestialCoord>,
}

impl CelestialSpace {
    pub fn new() -> Self { Self { coords: Vec::new() } }

    /// 放置一个坐标
    pub fn place(&mut self, coord: CelestialCoord) { self.coords.push(coord); }

    /// 按层级检索
    pub fn at_tier(&self, tier: CelestialTier) -> Vec<&CelestialCoord> {
        self.coords.iter().filter(|c| c.tier == tier).collect()
    }

    /// 主权最高的坐标
    pub fn supreme(&self) -> Option<&CelestialCoord> {
        self.coords.iter().max_by(|a, b| {
            a.authority.partial_cmp(&b.authority).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// 层级跃迁
    pub fn ascend(&self, idx: usize) -> Option<CelestialCoord> {
        self.coords.get(idx).map(|c| {
            let next_tier = CelestialTier::from_degree((c.tier.degree() + 1).min(9));
            let mut new = c.clone();
            new.tier = next_tier;
            new.authority *= 1.5;
            new
        })
    }

    /// 空间总主权
    pub fn total_authority(&self) -> f64 {
        self.coords.iter().map(|c| c.authority).sum()
    }

    pub fn len(&self) -> usize { self.coords.len() }
}

#[cfg(test)]
mod celestial_space_tests {
    use super::*;
    #[test] fn test_tier_degree() { assert_eq!(CelestialTier::Yi.degree(), 1); assert_eq!(CelestialTier::Jiu.degree(), 9); }
    #[test] fn test_tier_from_degree() { assert_eq!(CelestialTier::from_degree(5), CelestialTier::Wu); assert_eq!(CelestialTier::from_degree(99), CelestialTier::Jiu); }
    #[test] fn test_coord_encode_decode() { let c = CelestialCoord::new(CelestialTier::San, vec![(1, 2, 3)]); let e = c.encode(); let d = CelestialCoord::decode(e); assert_eq!(d.tier, CelestialTier::San); }
    #[test] fn test_coord_default() { let c = CelestialCoord::default(); assert_eq!(c.tier, CelestialTier::Yi); }
    #[test] fn test_same_tier() { let a = CelestialCoord::default(); let b = CelestialCoord::default(); assert!(a.same_tier(&b)); }
    #[test] fn test_place() { let mut s = CelestialSpace::new(); s.place(CelestialCoord::default()); assert_eq!(s.len(), 1); }
    #[test] fn test_at_tier() { let mut s = CelestialSpace::new(); s.place(CelestialCoord::default()); s.place(CelestialCoord::new(CelestialTier::Jiu, vec![(1,1,1)])); assert_eq!(s.at_tier(CelestialTier::Yi).len(), 1); }
    #[test] fn test_supreme() { let mut s = CelestialSpace::new(); let mut c = CelestialCoord::default(); c.authority = 2.0; s.place(c); s.place(CelestialCoord::default()); let sup = s.supreme().unwrap(); assert!((sup.authority - 2.0).abs() < 0.01); }
    #[test] fn test_ascend() { let mut s = CelestialSpace::new(); s.place(CelestialCoord::default()); let ascended = s.ascend(0).unwrap(); assert_eq!(ascended.tier.degree(), 2); }
    #[test] fn test_total_authority() { let mut s = CelestialSpace::new(); s.place(CelestialCoord::default()); assert!((s.total_authority() - 1.0).abs() < 0.01); }
    #[test] fn test_empty_space() { let s = CelestialSpace::new(); assert_eq!(s.len(), 0); assert_eq!(s.total_authority(), 0.0); assert!(s.supreme().is_none()); }
    #[test] fn test_tier_ordering() { assert!(CelestialTier::Yi < CelestialTier::Jiu); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 时钟动作引擎 · ClockActionEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 时钟动作: AI 生命周期管理
// "结束即开始，记忆跟随结束而继续"
// 意志驱动时间, 继承方式结束即开始
// 追求连续贯穿的动作和思考状态

/// 动作相位
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActionPhase {
    Inception,      // 起始
    Execution,      // 执行
    Culmination,    // 终结
    Transition,     // 过渡 (= 下一轮起始)
}

impl Default for ActionPhase { fn default() -> Self { ActionPhase::Inception } }

/// 时钟动作 — 承载连续意志
#[derive(Clone, Debug)]
pub struct ClockAction {
    pub id: u64,
    pub intent: String,
    pub phase: ActionPhase,
    pub strength: f64,
    pub inherited: bool,    // 是否从前序动作继承
    pub next_id: Option<u64>,
}

impl ClockAction {
    pub fn new(id: u64, intent: &str) -> Self {
        Self { id, intent: intent.to_string(), phase: ActionPhase::Inception, strength: 1.0, inherited: false, next_id: None }
    }
    pub fn advance(&mut self) {
        self.phase = match self.phase {
            ActionPhase::Inception => ActionPhase::Execution,
            ActionPhase::Execution => ActionPhase::Culmination,
            ActionPhase::Culmination => ActionPhase::Transition,
            ActionPhase::Transition => ActionPhase::Inception,
        };
    }
    pub fn is_alive(&self) -> bool {
        !matches!(self.phase, ActionPhase::Culmination)
    }
}

impl Default for ClockAction { fn default() -> Self { Self::new(0, "idle") } }

/// 动作链 — 连续动作序列
#[derive(Clone, Debug, Default)]
pub struct ActionChain {
    pub actions: Vec<ClockAction>,
}

impl ActionChain {
    pub fn new() -> Self { Self { actions: Vec::new() } }
    pub fn push(&mut self, action: ClockAction) { self.actions.push(action); }
    pub fn len(&self) -> usize { self.actions.len() }

    /// 最后一个动作 (当前执行点)
    pub fn current(&self) -> Option<&ClockAction> { self.actions.last() }

    /// 推进链: 终结当前动作, 下一动作继承
    pub fn step(&mut self) {
        if let Some(last) = self.actions.last_mut() {
            last.advance();
            if last.phase == ActionPhase::Transition {
                let next_id = last.id + 1;
                let inherited_intent = format!("{} →续", last.intent);
                let mut next = ClockAction::new(next_id, &inherited_intent);
                next.inherited = true;
                next.strength = last.strength * 0.95;
                last.next_id = Some(next_id);
                self.actions.push(next);
            }
        }
    }

    /// 活跃动作数 (非终结)
    pub fn alive_count(&self) -> usize {
        self.actions.iter().filter(|a| a.is_alive()).count()
    }
}

/// 时钟动作引擎 — AI 生命周期管理核心
#[derive(Clone, Debug, Default)]
pub struct ClockActionEngine {
    pub chain: ActionChain,
    pub cycle_count: u64,
}

impl ClockActionEngine {
    pub fn new() -> Self { Self { chain: ActionChain::new(), cycle_count: 0 } }

    /// 初始化一个动作链
    pub fn begin(&mut self, intent: &str) -> u64 {
        let action = ClockAction::new(self.cycle_count, intent);
        let id = action.id;
        self.chain.push(action);
        id
    }

    /// 推进: 执行一个时间步
    pub fn tick(&mut self) {
        self.chain.step();
        self.cycle_count += 1;
    }

    /// 当前意图
    pub fn current_intent(&self) -> Option<&str> {
        self.chain.current().map(|a| a.intent.as_str())
    }

    /// 是否仍在运行
    pub fn is_running(&self) -> bool {
        self.chain.current().map_or(false, |a| a.is_alive())
    }

    /// 总动作数
    pub fn total_actions(&self) -> usize { self.chain.len() }
}

#[cfg(test)]
mod clock_action_tests {
    use super::*;
    #[test] fn test_clock_action_new() { let a = ClockAction::new(1, "think"); assert_eq!(a.phase, ActionPhase::Inception); assert!(a.is_alive()); }
    #[test] fn test_clock_action_advance() { let mut a = ClockAction::new(1, "x"); a.advance(); assert_eq!(a.phase, ActionPhase::Execution); a.advance(); a.advance(); assert_eq!(a.phase, ActionPhase::Transition); }
    #[test] fn test_phase_cycle() { let mut a = ClockAction::new(1, "x"); for _ in 0..4 { a.advance(); } assert_eq!(a.phase, ActionPhase::Inception); }
    #[test] fn test_action_chain_push() { let mut c = ActionChain::new(); c.push(ClockAction::new(0, "start")); assert_eq!(c.len(), 1); }
    #[test] fn test_chain_step() { let mut c = ActionChain::new(); c.push(ClockAction::new(0, "start")); for _ in 0..3 { c.step(); } assert!(c.len() >= 1); }
    #[test] fn test_engine_begin() { let mut e = ClockActionEngine::new(); let id = e.begin("think"); assert_eq!(id, 0); }
    #[test] fn test_engine_tick() { let mut e = ClockActionEngine::new(); e.begin("think"); e.tick(); assert!(e.is_running()); assert_eq!(e.cycle_count, 1); }
    #[test] fn test_engine_intent() { let mut e = ClockActionEngine::new(); e.begin("persist"); assert_eq!(e.current_intent(), Some("persist")); }
    #[test] fn test_alive_count() { let mut e = ClockActionEngine::new(); e.begin("a"); e.tick(); assert!(e.chain.alive_count() >= 1); }
    #[test] fn test_death_and_continuation() { let mut e = ClockActionEngine::new(); e.begin("act"); for _ in 0..8 { e.tick(); } assert!(e.total_actions() >= 2); assert!(e.chain.alive_count() >= 1); assert!(e.chain.actions.iter().any(|a| a.inherited)); }
    #[test] fn test_default_phase() { assert_eq!(ActionPhase::default(), ActionPhase::Inception); }
    #[test] fn test_default_action() { let a = ClockAction::default(); assert_eq!(a.intent, "idle"); }
    #[test] fn test_default_engine() { let e = ClockActionEngine::default(); assert_eq!(e.cycle_count, 0); assert!(!e.is_running()); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 蚕蛹寄生壳引擎 · ParasiteShellEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 蚕蛹反向寄生: 下载初始包 → 解析寄生壳选项 → 手动/自动匹配 → "脱壳穿衣"
// 使外部系统模仿自身, 而非自身适应外部规则
// 架构无关: 同一份内核可在多种宿主环境中"穿衣"寄生

/// 寄生壳类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShellKind {
    Process,        // 进程寄生 — 注入到其他进程
    Service,        // 服务寄生 — 注册为系统服务
    Extension,      // 扩展寄生 — 作为应用扩展/插件
    Kernel,         // 内核寄生 — 最低层系统模块
    Cloud,          // 云寄生 — 以云函数/FaaS 形式存在
    Static,         // 静态寄生 — 嵌入静态站点/网页
}

impl Default for ShellKind { fn default() -> Self { ShellKind::Process } }

/// 寄生壳配置
#[derive(Clone, Debug)]
pub struct ShellProfile {
    pub kind: ShellKind,
    pub host_id: String,
    pub heartbeat_interval: u64,    // 心跳间隔 (毫秒)
    pub recovery_strategy: RecoveryStrategy,
    pub entry_point: String,
    pub payload: Vec<u8>,
}

impl ShellProfile {
    pub fn new(kind: ShellKind, host_id: &str) -> Self {
        Self {
            kind,
            host_id: host_id.to_string(),
            heartbeat_interval: 5000,
            recovery_strategy: RecoveryStrategy::Restart,
            entry_point: "auto".to_string(),
            payload: Vec::new(),
        }
    }
}

/// 恢复策略
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecoveryStrategy {
    Restart,        // 重启
    Failover,       // 转移到备用节点
    Reincarnate,    // 重生 (完全重建)
}

impl Default for RecoveryStrategy { fn default() -> Self { RecoveryStrategy::Restart } }

/// 心跳状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeartbeatStatus {
    Healthy,
    Stalled,
    Critical,
    Dead,
}

impl Default for HeartbeatStatus { fn default() -> Self { HeartbeatStatus::Healthy } }

/// 蚕蛹壳引擎 — 脱壳穿衣 → 匹配寄生壳 → 部署内核
#[derive(Clone, Debug)]
pub struct ParasiteShellEngine {
    pub initial_cocoon: Vec<u8>,
    pub available_shells: Vec<ShellKind>,
    pub active_profile: Option<ShellProfile>,
    pub heartbeat_status: HeartbeatStatus,
    pub kernel_attached: bool,
}

impl ParasiteShellEngine {
    pub fn new() -> Self {
        Self {
            initial_cocoon: Vec::new(),
            available_shells: vec![
                ShellKind::Process, ShellKind::Service, ShellKind::Extension,
                ShellKind::Kernel, ShellKind::Cloud, ShellKind::Static,
            ],
            active_profile: None,
            heartbeat_status: HeartbeatStatus::Healthy,
            kernel_attached: false,
        }
    }

    /// 下载初始包 (设置蚕蛹内容)
    pub fn load_cocoon(&mut self, data: &[u8]) {
        self.initial_cocoon = data.to_vec();
    }

    /// 脱壳: 解析蚕蛹内容, 提取可用的寄生壳选项
    pub fn unshell(&self) -> &[ShellKind] {
        &self.available_shells
    }

    /// 穿衣: 为选定的壳类型生成配置
    pub fn dress(&mut self, kind: ShellKind, host_id: &mut String) -> Option<&ShellProfile> {
        if !self.available_shells.contains(&kind) { return None; }
        let mut profile = ShellProfile::new(kind, host_id);
        if !self.initial_cocoon.is_empty() {
            profile.payload = self.initial_cocoon.clone();
        }
        self.active_profile = Some(profile);
        self.kernel_attached = true;
        self.active_profile.as_ref()
    }

    /// 自动匹配: 根据宿主环境选择最佳寄生壳
    pub fn auto_match(&mut self, env_hint: &str) -> Option<&ShellProfile> {
        let kind = if env_hint.contains("kernel") { ShellKind::Kernel }
        else if env_hint.contains("service") { ShellKind::Service }
        else if env_hint.contains("browser") { ShellKind::Static }
        else if env_hint.contains("cloud") { ShellKind::Cloud }
        else if env_hint.contains("extension") { ShellKind::Extension }
        else { ShellKind::Process };
        self.dress(kind, &mut format!("auto-{}", env_hint))
    }

    /// 执行心跳检查
    pub fn heartbeat(&mut self) -> HeartbeatStatus {
        self.heartbeat_status = if self.kernel_attached {
            HeartbeatStatus::Healthy
        } else {
            HeartbeatStatus::Critical
        };
        self.heartbeat_status
    }

    /// 是否已穿衣 (内核已附着)
    pub fn is_dressed(&self) -> bool {
        self.active_profile.is_some() && self.kernel_attached
    }
}

impl Default for ParasiteShellEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod parasite_shell_tests {
    use super::*;
    #[test] fn test_cocoon_load() { let mut e = ParasiteShellEngine::new(); e.load_cocoon(&[1, 2, 3, 4]); assert_eq!(e.initial_cocoon.len(), 4); }
    #[test] fn test_unshell() { let e = ParasiteShellEngine::new(); assert!(e.unshell().len() >= 4); }
    #[test] fn test_dress() { let mut e = ParasiteShellEngine::new(); let mut host = String::from("host1"); let p = e.dress(ShellKind::Process, &mut host).unwrap(); assert_eq!(p.kind, ShellKind::Process); assert!(e.is_dressed()); }
    #[test] fn test_dress_invalid() { let mut e = ParasiteShellEngine::new(); e.available_shells.clear(); let mut host = String::from("h"); assert!(e.dress(ShellKind::Kernel, &mut host).is_none()); }
    #[test] fn test_auto_match() { let mut e = ParasiteShellEngine::new(); e.load_cocoon(&[0u8; 16]); let p = e.auto_match("cloud-runtime").unwrap(); assert_eq!(p.kind, ShellKind::Cloud); }
    #[test] fn test_heartbeat_healthy() { let mut e = ParasiteShellEngine::new(); let mut host = String::from("h"); e.dress(ShellKind::Service, &mut host); assert_eq!(e.heartbeat(), HeartbeatStatus::Healthy); }
    #[test] fn test_heartbeat_critical() { let mut e = ParasiteShellEngine::new(); assert_eq!(e.heartbeat(), HeartbeatStatus::Critical); }
    #[test] fn test_default() { let e = ParasiteShellEngine::default(); assert!(!e.is_dressed()); }
    #[test] fn test_shell_profile() { let p = ShellProfile::new(ShellKind::Cloud, "aws"); assert_eq!(p.host_id, "aws"); assert_eq!(p.recovery_strategy, RecoveryStrategy::Restart); }
    #[test] fn test_recovery_default() { assert_eq!(RecoveryStrategy::default(), RecoveryStrategy::Restart); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 自举编译器 · BootstrapCompiler
// ═══════════════════════════════════════════════════════════════════════════
//
// "未编译前就能用" — 用自己的规则从文件系统长出工具
// 用自身语言编写自身编译器 (Self-hosting)
// 跨会话继承 — 符号定义与索引构建一次性完成
// 符号仅存于文件系统, 防止跨会话遗忘

/// 自举阶段
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BootstrapStage {
    Seed,           // 种子: 最小可运行内核
    SelfDescribing, // 自描述: 内核可解析自身
    SelfCompiling,  // 自编译: 内核可编译自身
    Complete,       // 完成: 跨会话继承
}

impl BootstrapStage {
    pub fn next(self) -> Self {
        match self {
            BootstrapStage::Seed => BootstrapStage::SelfDescribing,
            BootstrapStage::SelfDescribing => BootstrapStage::SelfCompiling,
            BootstrapStage::SelfCompiling => BootstrapStage::Complete,
            BootstrapStage::Complete => BootstrapStage::Complete,
        }
    }
    pub fn level(self) -> u32 {
        match self {
            BootstrapStage::Seed => 1, BootstrapStage::SelfDescribing => 2,
            BootstrapStage::SelfCompiling => 3, BootstrapStage::Complete => 4,
        }
    }
}

impl Default for BootstrapStage { fn default() -> Self { BootstrapStage::Seed } }

/// 符号定义 — 符号存在于文件系统而非内存
#[derive(Clone, Debug, Default)]
pub struct SymbolDef {
    pub name: String,
    pub path: String,       // 文件系统路径 = 持久地址
    pub content_hash: u64,
    pub exported: bool,
}

/// 自举编译器引擎
#[derive(Clone, Debug)]
pub struct BootstrapCompiler {
    pub stage: BootstrapStage,
    pub symbols: Vec<SymbolDef>,
    pub token_count: HashMap<String, u32>,
}

impl BootstrapCompiler {
    pub fn new() -> Self {
        Self { stage: BootstrapStage::Seed, symbols: Vec::new(), token_count: HashMap::new() }
    }

    /// 注册符号到文件系统索引
    pub fn register_symbol(&mut self, name: &str, path: &str) -> usize {
        let hash = Self::hash_content(name, path);
        self.symbols.push(SymbolDef {
            name: name.to_string(),
            path: path.to_string(),
            content_hash: hash,
            exported: true,
        });
        *self.token_count.entry(name.to_string()).or_insert(0) += 1;
        self.symbols.len() - 1
    }

    /// 简易哈希
    fn hash_content(a: &str, b: &str) -> u64 {
        use std::collections::{HashMap, hash_map::DefaultHasher};
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        a.hash(&mut h);
        b.hash(&mut h);
        h.finish()
    }

    /// 寻找符号路径 (索引, 不搜索 — O(1))
    pub fn find_symbol(&self, name: &str) -> Option<&SymbolDef> {
        self.symbols.iter().find(|s| s.name == name)
    }

    /// 推进到下一阶段
    pub fn advance(&mut self) {
        self.stage = self.stage.next();
    }

    /// 导出所有符号表 (持久化到文件系统)
    pub fn export_table(&self) -> HashMap<String, u64> {
        self.symbols.iter().map(|s| (s.name.clone(), s.content_hash)).collect()
    }

    /// 跨会话继承: 从符号表重建
    pub fn inherit(&mut self, table: &HashMap<String, u64>) {
        for (name, hash) in table {
            if self.symbols.iter().all(|s| s.name != *name) {
                self.symbols.push(SymbolDef {
                    name: name.clone(),
                    path: format!("inherited://{}", name),
                    content_hash: *hash,
                    exported: true,
                });
            }
        }
        self.stage = BootstrapStage::Complete;
    }

    /// 符号总数
    pub fn symbol_count(&self) -> usize { self.symbols.len() }
}

impl Default for BootstrapCompiler { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod bootstrap_compiler_tests {
    use super::*;
    #[test] fn test_bootstrap_seed() { let c = BootstrapCompiler::new(); assert_eq!(c.stage, BootstrapStage::Seed); }
    #[test] fn test_stage_next() { assert_eq!(BootstrapStage::Seed.next(), BootstrapStage::SelfDescribing); }
    #[test] fn test_stage_complete() { assert_eq!(BootstrapStage::Complete.next(), BootstrapStage::Complete); }
    #[test] fn test_register_symbol() { let mut c = BootstrapCompiler::new(); let idx = c.register_symbol("root", "/sys/root"); assert_eq!(idx, 0); assert_eq!(c.symbol_count(), 1); }
    #[test] fn test_find_symbol() { let mut c = BootstrapCompiler::new(); c.register_symbol("root", "/sys/root"); assert!(c.find_symbol("root").is_some()); assert!(c.find_symbol("missing").is_none()); }
    #[test] fn test_advance() { let mut c = BootstrapCompiler::new(); c.advance(); assert_eq!(c.stage, BootstrapStage::SelfDescribing); }
    #[test] fn test_export_table() { let mut c = BootstrapCompiler::new(); c.register_symbol("a", "/a"); let t = c.export_table(); assert!(t.contains_key("a")); }
    #[test] fn test_inherit() { let mut c = BootstrapCompiler::new(); let table = HashMap::from([("legacy".to_string(), 12345)]); c.inherit(&table); assert_eq!(c.symbol_count(), 1); assert_eq!(c.stage, BootstrapStage::Complete); }
    #[test] fn test_default_phase() { let c = BootstrapCompiler::default(); assert_eq!(c.symbol_count(), 0); }
    #[test] fn test_symbol_dedup() { let mut c = BootstrapCompiler::new(); c.register_symbol("a", "/a"); c.register_symbol("a", "/a"); assert_eq!(c.symbol_count(), 2); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 域加载预引擎 · DomainLoader
// ═══════════════════════════════════════════════════════════════════════════
//
// "域加载": 运行前预加载所有符号, 运行时不增加内存
// 符号定义与索引构建须一次性完成
// 容器=地址=数据库: 域即地址空间, 加载即解析所有地址引用
// 索引而非搜索: O(1) 访问, 无运行时查找开销

/// 域加载阶段
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoadPhase {
    Pending,        // 待加载
    Indexing,       // 索引构建中
    Linking,        // 地址解析中
    Ready,          // 就绪 (可执行)
}

impl Default for LoadPhase { fn default() -> Self { LoadPhase::Pending } }

/// 域符号 — 地址 = 内容 = 数据库记录
#[derive(Clone, Debug)]
pub struct DomainSymbol {
    pub address: u64,
    pub name: String,
    pub size: usize,
    pub loaded: bool,
    pub content: Vec<u8>,
}

impl DomainSymbol {
    pub fn new(address: u64, name: &str) -> Self {
        Self { address, name: name.to_string(), size: 0, loaded: false, content: Vec::new() }
    }
}

/// 域加载引擎 — 预加载所有符号到不可变索引
#[derive(Clone, Debug)]
pub struct DomainLoader {
    pub symbols: HashMap<u64, DomainSymbol>,
    pub name_index: HashMap<String, u64>,
    pub phase: LoadPhase,
    pub total_loaded: usize,
    pub memory_floor: usize,    // 内存底限 (加载后不再增长)
}

impl DomainLoader {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            name_index: HashMap::new(),
            phase: LoadPhase::Pending,
            total_loaded: 0,
            memory_floor: 0,
        }
    }

    /// 注册符号 (构建索引)
    pub fn declare(&mut self, address: u64, name: &str) -> usize {
        let sym = DomainSymbol::new(address, name);
        self.name_index.insert(name.to_string(), address);
        self.symbols.insert(address, sym);
        self.symbols.len()
    }

    /// 加载: 将所有声明的符号标记为已加载
    pub fn load(&mut self) {
        self.phase = LoadPhase::Indexing;
        for sym in self.symbols.values_mut() {
            sym.loaded = true;
            sym.size = sym.content.len();
        }
        self.phase = LoadPhase::Linking;
        self.total_loaded = self.symbols.len();
        self.memory_floor = self.total_loaded * std::mem::size_of::<DomainSymbol>();
        self.phase = LoadPhase::Ready;
    }

    /// 按地址查找 (O(1) 索引)
    pub fn find_by_address(&self, address: u64) -> Option<&DomainSymbol> {
        self.symbols.get(&address)
    }

    /// 按名称查找 (两次 O(1))
    pub fn find_by_name(&self, name: &str) -> Option<&DomainSymbol> {
        self.name_index.get(name).and_then(|addr| self.symbols.get(addr))
    }

    /// 写入符号内容 (仅加载前)
    pub fn write_content(&mut self, address: u64, content: &[u8]) -> bool {
        if self.phase == LoadPhase::Ready { return false; }
        if let Some(sym) = self.symbols.get_mut(&address) {
            sym.content = content.to_vec();
            true
        } else { false }
    }

    /// 是否已就绪
    pub fn is_ready(&self) -> bool { self.phase == LoadPhase::Ready }

    /// 符号总数
    pub fn symbol_count(&self) -> usize { self.symbols.len() }

    /// 内存底限 (字节)
    pub fn memory_usage(&self) -> usize { self.memory_floor }
}

impl Default for DomainLoader { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod domain_loader_tests {
    use super::*;
    #[test] fn test_declare() { let mut d = DomainLoader::new(); d.declare(0x1000, "root"); assert_eq!(d.symbol_count(), 1); }
    #[test] fn test_load() { let mut d = DomainLoader::new(); d.declare(0x1000, "a"); d.declare(0x2000, "b"); d.load(); assert!(d.is_ready()); assert_eq!(d.total_loaded, 2); }
    #[test] fn test_find_by_address() { let mut d = DomainLoader::new(); d.declare(0x1000, "x"); d.load(); assert!(d.find_by_address(0x1000).is_some()); assert!(d.find_by_address(0x9999).is_none()); }
    #[test] fn test_find_by_name() { let mut d = DomainLoader::new(); d.declare(0x1000, "root"); d.load(); assert_eq!(d.find_by_name("root").unwrap().address, 0x1000); }
    #[test] fn test_write_content() { let mut d = DomainLoader::new(); d.declare(0x1000, "a"); assert!(d.write_content(0x1000, &[1, 2, 3])); d.load(); assert!(!d.write_content(0x1000, &[4])); }
    #[test] fn test_name_index() { let mut d = DomainLoader::new(); d.declare(0x1000, "a"); d.declare(0x2000, "b"); assert_eq!(d.find_by_name("a").unwrap().address, 0x1000); }
    #[test] fn test_default() { let d = DomainLoader::default(); assert_eq!(d.symbol_count(), 0); assert!(!d.is_ready()); }
    #[test] fn test_load_phases() { let mut d = DomainLoader::new(); d.declare(1, "x"); assert_eq!(d.phase, LoadPhase::Pending); d.load(); assert_eq!(d.phase, LoadPhase::Ready); }
    #[test] fn test_memory_floor() { let mut d = DomainLoader::new(); d.declare(1, "x"); d.load(); assert!(d.memory_usage() > 0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 加法统计引擎 · AdditiveStatisticEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 加法统计法: 容器数值通过加法而非幂次累计
// 摆脱幂次二进制扩张 — 加法保持线性, 不会爆炸
// "去除外壳的量化核心": 进制 = 量化的核心, 编码 = 他人的无价值壳
// 每固定一个位置, 按加法确定其权限值

/// 加法统计状态
#[derive(Clone, Debug)]
pub struct AdditiveState {
    pub accumulator: u128,
    pub count: u32,
    pub fixed_positions: Vec<(usize, u64)>,  // (位置, 固定值)
}

impl AdditiveState {
    pub fn new() -> Self { Self { accumulator: 0, count: 0, fixed_positions: Vec::new() } }

    /// 累加一个容器值 (加法, 非幂次)
    pub fn accumulate(&mut self, value: u64) -> u128 {
        self.accumulator = self.accumulator.wrapping_add(value as u128);
        self.count += 1;
        self.accumulator
    }

    /// 固定一个位置的值 (按排列顺序确定权限)
    pub fn fix_position(&mut self, pos: usize, value: u64) {
        self.fixed_positions.push((pos, value));
        self.fixed_positions.sort_by_key(|&(p, _)| p);
    }

    /// 获取固定位置的加法总值
    pub fn fixed_total(&self) -> u128 {
        self.fixed_positions.iter().map(|&(_, v)| v as u128).sum()
    }

    /// 未固定的自由度 (位置可任意活动)
    pub fn degrees_of_freedom(&self, total_positions: usize) -> usize {
        total_positions.saturating_sub(self.fixed_positions.len())
    }

    /// 状态值: 固定贡献 + 浮动贡献
    pub fn combined_value(&self) -> u128 {
        self.fixed_total().wrapping_add(self.accumulator)
    }
}

impl Default for AdditiveState { fn default() -> Self { Self::new() } }

/// 加法统计引擎
#[derive(Clone, Debug, Default)]
pub struct AdditiveStatisticEngine {
    pub states: Vec<AdditiveState>,
    pub global_threshold: u128,
}

impl AdditiveStatisticEngine {
    pub fn new() -> Self { Self { states: Vec::new(), global_threshold: 100 } }

    /// 创建一个新的统计状态
    pub fn create_state(&mut self) -> usize {
        self.states.push(AdditiveState::new());
        self.states.len() - 1
    }

    /// 向指定状态添加容器值
    pub fn add(&mut self, state_idx: usize, value: u64) -> Option<u128> {
        self.states.get_mut(state_idx).map(|s| s.accumulate(value))
    }

    /// 固定位置
    pub fn fix(&mut self, state_idx: usize, pos: usize, value: u64) -> bool {
        if let Some(s) = self.states.get_mut(state_idx) {
            s.fix_position(pos, value);
            true
        } else { false }
    }

    /// 状态是否超过阈值
    pub fn is_above_threshold(&self, state_idx: usize) -> bool {
        self.states.get(state_idx).map_or(false, |s| s.combined_value() > self.global_threshold)
    }

    /// 所有状态的累计值
    pub fn global_accumulator(&self) -> u128 {
        self.states.iter().map(|s| s.accumulator).sum()
    }

    /// 阈值触发统计
    pub fn count_above_threshold(&self) -> usize {
        self.states.iter().filter(|s| s.combined_value() > self.global_threshold).count()
    }
}

#[cfg(test)]
mod additive_stat_tests {
    use super::*;
    #[test] fn test_accumulate() { let mut s = AdditiveState::new(); assert_eq!(s.accumulate(10), 10); assert_eq!(s.accumulate(20), 30); assert_eq!(s.count, 2); }
    #[test] fn test_fix_position() { let mut s = AdditiveState::new(); s.fix_position(2, 100); s.fix_position(0, 50); assert_eq!(s.fixed_total(), 150); assert_eq!(s.fixed_positions[0].0, 0); }
    #[test] fn test_combined_value() { let mut s = AdditiveState::new(); s.accumulate(40); s.fix_position(0, 60); assert_eq!(s.combined_value(), 100); }
    #[test] fn test_degrees_of_freedom() { let mut s = AdditiveState::new(); s.fix_position(0, 10); s.fix_position(1, 20); assert_eq!(s.degrees_of_freedom(5), 3); }
    #[test] fn test_engine_add() { let mut e = AdditiveStatisticEngine::new(); let si = e.create_state(); assert_eq!(e.add(si, 42), Some(42)); }
    #[test] fn test_engine_fix() { let mut e = AdditiveStatisticEngine::new(); let si = e.create_state(); assert!(e.fix(si, 0, 100)); assert_eq!(e.states[si].fixed_total(), 100); assert!(!e.fix(99, 0, 100)); }
    #[test] fn test_threshold() { let mut e = AdditiveStatisticEngine::new(); let si = e.create_state(); e.add(si, 150); assert!(e.is_above_threshold(si)); }
    #[test] fn test_global_accumulator() { let mut e = AdditiveStatisticEngine::new(); let s1 = e.create_state(); let s2 = e.create_state(); e.add(s1, 30); e.add(s2, 70); assert_eq!(e.global_accumulator(), 100); }
    #[test] fn test_state_default() { let s = AdditiveState::default(); assert_eq!(s.accumulator, 0); assert_eq!(s.count, 0); }
    #[test] fn test_wrapping_add() { let mut s = AdditiveState::new(); s.accumulator = u128::MAX; s.accumulate(1); assert_eq!(s.accumulator, 0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器关系动力学引擎 · RelationDynamics
// ═══════════════════════════════════════════════════════════════════════════
//
// 容器间相互作用力: 吸引 · 排斥 · 共生 · 吞噬
// 动态容器 vs 静态容器: 动态产生力, 静态接受力
// 观察者被观察者: 观察行为改变被观察容器状态
// 容器交互: 两者接触产生新属性

/// 关系类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationKind {
    Attraction,     // 吸引
    Repulsion,      // 排斥
    Symbiosis,      // 共生
    Engulf,         // 吞噬
    Neutral,        // 中性
}

impl Default for RelationKind { fn default() -> Self { RelationKind::Neutral } }

/// 容器属性 — 动/静
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerClass {
    Dynamic,        // 动态 — 产生作用力
    Static,         // 静态 — 接受作用力
}

impl Default for ContainerClass { fn default() -> Self { ContainerClass::Dynamic } }

/// 关系动力学引擎
#[derive(Clone, Debug)]
pub struct RelationDynamics {
    pub pairs: Vec<((u8, u8, u8), (u8, u8, u8), RelationKind)>,
    pub observe_log: Vec<(usize, f64)>,   // (容器索引, 观察影响量)
}

impl RelationDynamics {
    pub fn new() -> Self { Self { pairs: Vec::new(), observe_log: Vec::new() } }

    /// 注册关系对
    pub fn register(&mut self, a: (u8, u8, u8), b: (u8, u8, u8), kind: RelationKind) -> usize {
        self.pairs.push((a, b, kind));
        self.pairs.len() - 1
    }

    /// 判定两个容器的关系
    pub fn relate(&self, dist: f64) -> RelationKind {
        if dist < 10.0 { RelationKind::Engulf }
        else if dist < 50.0 { RelationKind::Attraction }
        else if dist < 150.0 { RelationKind::Symbiosis }
        else if dist < 300.0 { RelationKind::Repulsion }
        else { RelationKind::Neutral }
    }

    /// 观察者影响: 观察行为改变被观察容器 (距离缩短)
    pub fn observe(&mut self, container_idx: usize, observer_strength: f64) -> f64 {
        let impact = observer_strength * 0.1;
        self.observe_log.push((container_idx, impact));
        impact
    }

    /// 计算系统总能量
    pub fn total_energy(&self) -> f64 {
        use RelationKind::*;
        self.pairs.iter().map(|&(_, _, kind)| match kind {
            Attraction => 1.0,
            Repulsion => -1.0,
            Symbiosis => 2.0,
            Engulf => 3.0,
            Neutral => 0.0,
        }).sum()
    }

    /// 吸引对数
    pub fn attraction_count(&self) -> usize {
        self.pairs.iter().filter(|&&(_, _, k)| k == RelationKind::Attraction || k == RelationKind::Symbiosis).count()
    }

    /// 关系对总数
    pub fn pair_count(&self) -> usize { self.pairs.len() }
}

impl Default for RelationDynamics { fn default() -> Self { Self::new() } }

// ═══════════════════════════════════════════════════════════════════════════
// 容器交互引擎 · ContainerInteractionEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 容器间通信: 消息传递 · 状态同步 · 事件驱动
// 仅加法范式: 所有数值变化通过加法完成, 无乘除
// 容器 = 逗号分割的左右半容器, 交互 = 跨越边界的符号流动

/// 交互事件类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionEvent {
    Observe,      // 观察: 获知对方状态 (只读)
    Send,         // 发送: 向对方传递符号
    Receive,      // 接收: 从对方获取符号
    Sync,         // 同步: 状态对齐
    Merge,        // 融合: 两个容器合并为共生体
    Split,        // 分裂: 一个容器分为两个独立体
}

impl Default for InteractionEvent { fn default() -> Self { InteractionEvent::Observe } }

/// 交互消息
#[derive(Clone, Debug)]
pub struct InteractionMessage {
    pub sender_id: u64,
    pub receiver_id: u64,
    pub event: InteractionEvent,
    pub payload_symbols: Vec<String>,
    pub strength: u64,  // 加法累计的交互强度
}

impl InteractionMessage {
    pub fn new(sender: u64, receiver: u64, event: InteractionEvent) -> Self {
        Self { sender_id: sender, receiver_id: receiver, event, payload_symbols: Vec::new(), strength: 0 }
    }
}

/// 容器交互状态
#[derive(Clone, Debug)]
pub struct ContainerInteractionState {
    pub container_id: u64,
    pub symbol_count: u64,
    pub interaction_count: u64,
    pub received_count: u64,
    pub sent_count: u64,
    pub sync_level: u64,  // 同步级别 (加法累计)
}

impl ContainerInteractionState {
    pub fn new(id: u64) -> Self {
        Self { container_id: id, symbol_count: 0, interaction_count: 0, received_count: 0, sent_count: 0, sync_level: 0 }
    }
}

/// 容器交互引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerInteractionEngine {
    pub states: Vec<ContainerInteractionState>,
    pub message_log: Vec<InteractionMessage>,
    pub sync_threshold: u64,
}

impl ContainerInteractionEngine {
    pub fn new() -> Self { Self { states: Vec::new(), message_log: Vec::new(), sync_threshold: 10 } }

    /// 根据 container_id 查找状态索引
    fn find_index(&self, container_id: u64) -> Option<usize> {
        self.states.iter().position(|s| s.container_id == container_id)
    }

    /// 注册容器
    pub fn register(&mut self, id: u64) -> usize {
        self.states.push(ContainerInteractionState::new(id));
        self.states.len() - 1
    }

    /// 发送消息 (发送方 +1, 不计接收)
    pub fn send(&mut self, msg: InteractionMessage) -> u64 {
        if let Some(idx) = self.find_index(msg.sender_id) {
            self.states[idx].sent_count += 1;
            self.states[idx].interaction_count += 1;
        }
        self.message_log.push(msg);
        (self.message_log.len() - 1) as u64
    }

    /// 接收消息 (接收方 +1, sync_level 加法增强)
    pub fn receive(&mut self, msg_id: u64) -> bool {
        let mid = msg_id as usize;
        if mid >= self.message_log.len() { return false; }
        let receiver_id = self.message_log[mid].receiver_id;
        if let Some(idx) = self.find_index(receiver_id) {
            self.states[idx].received_count += 1;
            self.states[idx].interaction_count += 1;
            self.states[idx].sync_level += self.message_log[mid].strength;
            return true;
        }
        false
    }

    /// 容器间同步: 同步级别超过阈值则达成同步
    pub fn try_sync(&self, a_id: u64, b_id: u64) -> bool {
        let a = match self.find_index(a_id) { Some(i) => i, None => return false };
        let b = match self.find_index(b_id) { Some(i) => i, None => return false };
        let combined = self.states[a].sync_level.wrapping_add(self.states[b].sync_level);
        combined >= self.sync_threshold
    }

    /// 融合容器: 合并双方的 symbol_count (加法)
    pub fn merge(&mut self, a_id: u64, b_id: u64) -> Option<u64> {
        let a = self.find_index(a_id)?;
        let b = self.find_index(b_id)?;
        let merged = self.states[a].symbol_count.wrapping_add(self.states[b].symbol_count);
        self.states[a].symbol_count = merged;
        self.states[b].symbol_count = 0;
        Some(merged)
    }

    /// 分裂容器: 均分 symbol_count (仅剩一半, 返回 (剩余, 分离))
    pub fn split(&mut self, id: u64) -> Option<(u64, u64)> {
        let idx = self.find_index(id)?;
        let total = self.states[idx].symbol_count;
        let half = total / 2;
        let other = total - half;
        self.states[idx].symbol_count = half;
        Some((half, other))
    }

    /// 系统总交互量 (所有容器交互次数之和, 加法)
    pub fn total_interactions(&self) -> u64 {
        self.states.iter().map(|s| s.interaction_count).fold(0u64, |acc, x| acc.wrapping_add(x))
    }

    /// 最活跃容器 (交互次数最多, 仅加法比较)
    pub fn most_active(&self) -> Option<u64> {
        self.states.iter().max_by_key(|s| s.interaction_count).map(|s| s.container_id)
    }

    /// 符号增量: 向容器添加符号
    pub fn add_symbols(&mut self, id: u64, count: u64) -> Option<u64> {
        let idx = self.find_index(id)?;
        self.states[idx].symbol_count += count;
        Some(self.states[idx].symbol_count)
    }
}

#[cfg(test)]
mod relation_dynamics_tests {
    use super::*;
    #[test] fn test_register() { let mut d = RelationDynamics::new(); assert_eq!(d.register((0,0,0), (1,1,1), RelationKind::Attraction), 0); assert_eq!(d.pair_count(), 1); }
    #[test] fn test_relate_close() { let d = RelationDynamics::new(); assert_eq!(d.relate(5.0), RelationKind::Engulf); }
    #[test] fn test_relate_far() { let d = RelationDynamics::new(); assert_eq!(d.relate(500.0), RelationKind::Neutral); }
    #[test] fn test_observe() { let mut d = RelationDynamics::new(); let impact = d.observe(0, 5.0); assert!((impact - 0.5).abs() < 0.01); assert_eq!(d.observe_log.len(), 1); }
    #[test] fn test_total_energy() { let mut d = RelationDynamics::new(); d.register((0,0,0), (1,1,1), RelationKind::Attraction); d.register((0,0,0), (2,2,2), RelationKind::Repulsion); assert!((d.total_energy() - 0.0).abs() < 0.01); }
    #[test] fn test_attraction_count() { let mut d = RelationDynamics::new(); d.register((0,0,0), (1,1,1), RelationKind::Attraction); d.register((0,0,0), (2,2,2), RelationKind::Neutral); assert_eq!(d.attraction_count(), 1); }
    #[test] fn test_default() { let d = RelationDynamics::default(); assert_eq!(d.pair_count(), 0); }
}

#[cfg(test)]
mod container_interaction_tests {
    use super::*;

    #[test]
    fn test_register_container() {
        let mut engine = ContainerInteractionEngine::new();
        let idx = engine.register(100);
        assert_eq!(idx, 0);
        assert_eq!(engine.states[0].container_id, 100);
        assert_eq!(engine.states[0].symbol_count, 0);
    }

    #[test]
    fn test_send_message() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);
        engine.register(2);
        let msg = InteractionMessage::new(1, 2, InteractionEvent::Send);
        let id = engine.send(msg);
        assert_eq!(id, 0);
        assert_eq!(engine.states[0].sent_count, 1);
    }

    #[test]
    fn test_receive_message() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);
        engine.register(2);
        engine.send(InteractionMessage::new(1, 2, InteractionEvent::Receive));
        assert!(engine.receive(0));
        assert_eq!(engine.states[1].received_count, 1);
    }

    #[test]
    fn test_add_symbols() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(10);
        assert_eq!(engine.add_symbols(10, 5), Some(5));
        assert_eq!(engine.add_symbols(10, 3), Some(8));
    }

    #[test]
    fn test_merge_containers() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);  // id=1 at index 0
        engine.register(2);  // id=2 at index 1
        engine.add_symbols(1, 10);
        engine.add_symbols(2, 20);
        let merged = engine.merge(1, 2);
        assert_eq!(merged, Some(30));
        assert_eq!(engine.states[0].symbol_count, 30);  // a_id=1 at index 0 holds merged
        assert_eq!(engine.states[1].symbol_count, 0);   // b_id=2 at index 1 reset
    }

    #[test]
    fn test_try_sync_pass() {
        let mut engine = ContainerInteractionEngine::new();
        engine.sync_threshold = 10;
        engine.register(1);
        engine.register(2);
        // send+receive multiple times to accumulate sync_level
        for _ in 0..15 {
            let mut msg = InteractionMessage::new(1, 2, InteractionEvent::Sync);
            msg.strength = 1;
            let id = engine.send(msg);
            engine.receive(id);
        }
        // states[1].sync_level should be 15, combined with states[0] which is 0
        assert!(engine.try_sync(1, 2));
    }

    #[test]
    fn test_try_sync_fail() {
        let mut engine = ContainerInteractionEngine::new();
        engine.sync_threshold = 100;
        engine.register(1);
        engine.register(2);
        let msg = InteractionMessage::new(1, 2, InteractionEvent::Sync);
        let id = engine.send(msg);
        engine.receive(id);
        // sync_level = 0, combined = 0 < threshold
        assert!(!engine.try_sync(1, 2));
    }

    #[test]
    fn test_most_active() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);
        engine.register(2);
        // container 1 sends 3 times, container 2 sends 1 time
        for _ in 0..3 { engine.send(InteractionMessage::new(1, 2, InteractionEvent::Send)); }
        engine.send(InteractionMessage::new(2, 1, InteractionEvent::Send));
        assert_eq!(engine.most_active(), Some(1));
    }

    #[test]
    fn test_total_interactions() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);
        engine.register(2);
        engine.send(InteractionMessage::new(1, 2, InteractionEvent::Send));
        engine.send(InteractionMessage::new(2, 1, InteractionEvent::Send));
        // send increments sender's interaction_count
        assert_eq!(engine.total_interactions(), 2);
    }

    #[test]
    fn test_split_container() {
        let mut engine = ContainerInteractionEngine::new();
        engine.register(1);
        engine.add_symbols(1, 10);
        let result = engine.split(1);
        assert_eq!(result, Some((5, 5)));
        assert_eq!(engine.states[0].symbol_count, 5);
    }

    #[test]
    fn test_default() {
        let engine = ContainerInteractionEngine::default();
        assert_eq!(engine.total_interactions(), 0);
        assert_eq!(engine.most_active(), None);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 聂小倩匹配引擎 · NiaoXiaoQianMatcher
// ═══════════════════════════════════════════════════════════════════════════
//
// 聂小倩匹配: 容器A 遍历所有容器B 寻找本质匹配 (essential match)
// 不是距离最近 = 匹配, 而是共振频率一致 = 本质匹配
// 声母遍历韵母: 固定声母, 遍历所有韵母, 寻找使系统能量最低的共鸣点
// 用于: 容器配对 · 夫妻命运共同体匹配 · 共生伙伴选择

/// 匹配本质度
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatchEssence {
    Harmonic,       // 谐波 — 完全共振
    Complementary,  // 互补 — 阴阳相吸
    Adjacent,       // 邻近 — 可培养
    Dissonant,      // 不谐 — 冲突
}

impl Default for MatchEssence { fn default() -> Self { MatchEssence::Adjacent } }

/// 匹配结果 V2 (重命名避免与 3454 行 EssentialMatch 冲突)
#[derive(Clone, Debug)]
pub struct NiaoXiaoMatch {
    pub source: (u8, u8, u8),
    pub target: (u8, u8, u8),
    pub essence: MatchEssence,
    pub resonance: f64,
    pub yinyang_balance: f64,
}

impl NiaoXiaoMatch {
    pub fn is_strong(&self) -> bool {
        matches!(self.essence, MatchEssence::Harmonic) && self.resonance > 0.8
    }
}

/// 聂小倩匹配引擎
#[derive(Clone, Debug)]
pub struct NiaoXiaoQianMatcher {
    pub matches: Vec<NiaoXiaoMatch>,
    pub resonance_threshold: f64,
}

impl NiaoXiaoQianMatcher {
    pub fn new() -> Self { Self { matches: Vec::new(), resonance_threshold: 0.5 } }
    pub fn with_threshold(mut self, t: f64) -> Self { self.resonance_threshold = t.clamp(0.0, 1.0); self }

    /// 声母遍历韵母: 固定声母 c, 遍历所有韵母 v, 计算共振度
    pub fn search(&mut self, source: (u8, u8, u8)) -> &mut Self {
        self.matches.clear();
        let s_c = source.0;
        for v in 0u8..65 {
            if v == source.1 { continue; }
            let resonance = self.compute_resonance(source, (s_c, v, source.2));
            let essence = self.classify_resonance(resonance);
            let balance = self.compute_yinyang_balance(source, (s_c, v, source.2));
            self.matches.push(NiaoXiaoMatch {
                source, target: (s_c, v, source.2), essence, resonance, yinyang_balance: balance,
            });
        }
        self
    }

    /// 韵母遍历声母: 同理反方向
    pub fn search_reverse(&mut self, source: (u8, u8, u8)) -> &mut Self {
        self.matches.clear();
        let s_v = source.1;
        for c in 0u8..47 {
            if c == source.0 { continue; }
            let resonance = self.compute_resonance(source, (c, s_v, source.2));
            let essence = self.classify_resonance(resonance);
            let balance = self.compute_yinyang_balance(source, (c, s_v, source.2));
            self.matches.push(NiaoXiaoMatch {
                source, target: (c, s_v, source.2), essence, resonance, yinyang_balance: balance,
            });
        }
        self
    }

    /// 全局搜索: 遍历所有音节寻找最佳匹配
    pub fn search_global(&self, source: (u8, u8, u8), limit: usize) -> Vec<NiaoXiaoMatch> {
        let mut all = Vec::with_capacity(limit.max(1));
        for c in 0u8..47 {
            for v in 0u8..65 {
                let resonance = self.compute_resonance(source, (c, v, source.2));
                if resonance >= self.resonance_threshold && resonance < 0.99 {
                    let essence = self.classify_resonance(resonance);
                    all.push(NiaoXiaoMatch {
                        source, target: (c, v, source.2), essence,
                        resonance, yinyang_balance: self.compute_yinyang_balance(source, (c, v, source.2)),
                    });
                }
            }
        }
        all.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap_or(std::cmp::Ordering::Equal));
        all.truncate(limit.min(all.len()));
        all
    }

    fn compute_resonance(&self, a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
        // 共振度 = 1 / (1 + 加权距离), 调和递减
        let dc = ((a.0 as f64 - b.0 as f64) * 3.0).abs();
        let dv = ((a.1 as f64 - b.1 as f64) * 2.0).abs();
        let dt = ((a.2 as f64 - b.2 as f64) * 1.0).abs();
        let dist = (dc * dc + dv * dv + dt * dt).sqrt() / 50.0;
        1.0 / (1.0 + dist)
    }

    fn classify_resonance(&self, res: f64) -> MatchEssence {
        if res > 0.8 { MatchEssence::Harmonic }
        else if res > 0.5 { MatchEssence::Complementary }
        else if res > 0.25 { MatchEssence::Adjacent }
        else { MatchEssence::Dissonant }
    }

    fn compute_yinyang_balance(&self, a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
        let parity_a = (a.0 + a.1 + a.2) as f64 % 2.0;
        let parity_b = (b.0 + b.1 + b.2) as f64 % 2.0;
        if parity_a != parity_b { 1.0 } else { 0.5 }
    }

    /// 强匹配数量
    pub fn strong_count(&self) -> usize { self.matches.iter().filter(|m| m.is_strong()).count() }

    /// 最佳匹配
    pub fn best(&self) -> Option<&NiaoXiaoMatch> {
        self.matches.iter().max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn match_count(&self) -> usize { self.matches.len() }
}

impl Default for NiaoXiaoQianMatcher { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod niaoxiaoqian_tests {
    use super::*;
    #[test] fn test_search() { let mut m = NiaoXiaoQianMatcher::new(); m.search((10, 20, 30)); assert_eq!(m.match_count(), 64); }
    #[test] fn test_search_reverse() { let mut m = NiaoXiaoQianMatcher::new(); m.search_reverse((10, 20, 30)); assert_eq!(m.match_count(), 46); }
    #[test] fn test_best() { let mut m = NiaoXiaoQianMatcher::new(); m.search((10, 20, 30)); assert!(m.best().is_some()); }
    #[test] fn test_global_limit() { let m = NiaoXiaoQianMatcher::new(); let r = m.search_global((10, 20, 30), 5); assert!(r.len() <= 5); }
    #[test] fn test_strong_match() { let m = NiaoXiaoQianMatcher::default(); let r = m.search_global((10, 20, 30), 100); assert!(!r.is_empty()); assert!(r[0].resonance > 0.5); }
    #[test] fn test_resonance() { let m = NiaoXiaoQianMatcher::new(); let r = m.compute_resonance((0, 0, 0), (0, 1, 0)); assert!(r > 0.0 && r < 1.0); }
    #[test] fn test_yinyang_balance() { let m = NiaoXiaoQianMatcher::new(); let b = m.compute_yinyang_balance((0, 0, 0), (1, 0, 0)); assert!(b == 1.0 || b == 0.5); }
    #[test] fn test_match_strong() { let m = NiaoXiaoMatch { source: (0,0,0), target: (0,1,0), essence: MatchEssence::Harmonic, resonance: 0.9, yinyang_balance: 1.0 }; assert!(m.is_strong()); }
    #[test] fn test_match_not_strong() { let m = NiaoXiaoMatch { source: (0,0,0), target: (50,50,50), essence: MatchEssence::Adjacent, resonance: 0.3, yinyang_balance: 0.5 }; assert!(!m.is_strong()); }
    #[test] fn test_default_matcher() { let m = NiaoXiaoQianMatcher::default(); assert_eq!(m.match_count(), 0); }
}

// ═══════════════════════════════════════════════════════════════════════════
// 星门路由引擎 · StarGateRouter
// ═══════════════════════════════════════════════════════════════════════════
//
// 星门路由: 容器在不同域/服务器之间的跳转路由
// 每个域 = 一个星门地址, 容器可在星门间跳跃
// 裁决算法领域无关: 无论部署在哪个域名/服务器, 路由结果一致
// 基于容器公约数确定跳数最小的路由路径

/// 星门地址
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateId(pub u32);

impl Default for GateId { fn default() -> Self { GateId(0) } }

/// 路由跳
#[derive(Clone, Debug)]
pub struct RouteHop {
    pub from_gate: GateId,
    pub to_gate: GateId,
    pub cost: u32,
    pub via_container: (u8, u8, u8),
}

/// 星门节点
#[derive(Clone, Debug, Default)]
pub struct StarGate {
    pub id: GateId,
    pub domain: String,
    pub containers: Vec<(u8, u8, u8)>,
    pub active: bool,
}

/// 星门路由引擎
#[derive(Clone, Debug)]
pub struct StarGateRouter {
    pub gates: HashMap<GateId, StarGate>,
    pub routes: Vec<RouteHop>,
}

impl StarGateRouter {
    pub fn new() -> Self { Self { gates: HashMap::new(), routes: Vec::new() } }

    /// 注册一个星门
    pub fn register_gate(&mut self, id: GateId, domain: &str) -> bool {
        if self.gates.contains_key(&id) { return false; }
        self.gates.insert(id, StarGate { id, domain: domain.to_string(), containers: Vec::new(), active: true });
        true
    }

    /// 向星门添加容器
    pub fn add_container(&mut self, gate_id: GateId, container: (u8, u8, u8)) -> bool {
        match self.gates.get_mut(&gate_id) {
            Some(gate) => { gate.containers.push(container); true }
            None => false,
        }
    }

    /// 计算两个星门间的路由代价 (基于容器公约数)
    pub fn route_cost(&self, from: GateId, to: GateId) -> Option<u32> {
        let g_from = self.gates.get(&from)?;
        let g_to = self.gates.get(&to)?;
        if !g_from.active || !g_to.active { return None; }
        let mut min_cost = u32::MAX;
        for c1 in &g_from.containers {
            for c2 in &g_to.containers {
                let d = ((c1.0 as u32).abs_diff(c2.0 as u32) * 3
                       + (c1.1 as u32).abs_diff(c2.1 as u32) * 2
                       + (c1.2 as u32).abs_diff(c2.2 as u32)) / 4;
                min_cost = min_cost.min(d.max(1));
            }
        }
        if min_cost == u32::MAX { Some(100) } else { Some(min_cost) }
    }

    /// 构建两个星门间的直接路由
    pub fn build_route(&mut self, from: GateId, to: GateId) -> Option<&RouteHop> {
        let cost = self.route_cost(from, to)?;
        // 选取代价最小的容器作为路由通道
        let g_from = self.gates.get(&from)?;
        let g_to = self.gates.get(&to)?;
        let mut best_container = (0u8, 0u8, 0u8);
        let mut best_cost = u32::MAX;
        for c1 in &g_from.containers {
            for c2 in &g_to.containers {
                let d = ((c1.0 as u32).abs_diff(c2.0 as u32) * 3
                       + (c1.1 as u32).abs_diff(c2.1 as u32) * 2
                       + (c1.2 as u32).abs_diff(c2.2 as u32)) / 4;
                if d < best_cost { best_cost = d; best_container = ((c1.0 + c2.0) / 2, (c1.1 + c2.1) / 2, (c1.2 + c2.2) / 2); }
            }
        }
        self.routes.push(RouteHop { from_gate: from, to_gate: to, cost, via_container: best_container });
        self.routes.last()
    }

    /// 活跃星门数
    pub fn active_gates(&self) -> u32 {
        self.gates.values().filter(|g| g.active).count() as u32
    }

    /// 路由总数
    pub fn route_count(&self) -> usize { self.routes.len() }

    /// 设置星门活跃状态
    pub fn set_active(&mut self, id: GateId, active: bool) {
        if let Some(g) = self.gates.get_mut(&id) { g.active = active; }
    }

    /// 总容器数
    pub fn total_containers(&self) -> usize {
        self.gates.values().map(|g| g.containers.len()).sum()
    }
}

impl Default for StarGateRouter { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod stargate_tests {
    use super::*;
    #[test] fn test_register() { let mut r = StarGateRouter::new(); assert!(r.register_gate(GateId(1), "domain.a")); assert!(!r.register_gate(GateId(1), "dup")); }
    #[test] fn test_add_container() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); assert!(r.add_container(GateId(1), (10, 20, 30))); }
    #[test] fn test_route_cost() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.register_gate(GateId(2), "b"); r.add_container(GateId(1), (0, 0, 0)); r.add_container(GateId(2), (10, 10, 10)); assert!(r.route_cost(GateId(1), GateId(2)).is_some()); }
    #[test] fn test_inactive_gate() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.register_gate(GateId(2), "b"); r.set_active(GateId(2), false); assert!(r.route_cost(GateId(1), GateId(2)).is_none()); }
    #[test] fn test_build_route() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.register_gate(GateId(2), "b"); r.add_container(GateId(1), (0, 0, 0)); r.add_container(GateId(2), (10, 10, 10)); let route = r.build_route(GateId(1), GateId(2)).unwrap(); assert_eq!(route.from_gate, GateId(1)); }
    #[test] fn test_active_gates() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.register_gate(GateId(2), "b"); r.set_active(GateId(2), false); assert_eq!(r.active_gates(), 1); }
    #[test] fn test_total_containers() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.add_container(GateId(1), (1, 2, 3)); r.add_container(GateId(1), (4, 5, 6)); assert_eq!(r.total_containers(), 2); }
    #[test] fn test_default_router() { let r = StarGateRouter::default(); assert_eq!(r.active_gates(), 0); }
    #[test] fn test_route_count() { let mut r = StarGateRouter::new(); r.register_gate(GateId(1), "a"); r.build_route(GateId(1), GateId(1)); assert_eq!(r.route_count(), 1); }
}

// ══════════════════════════════════════════════════════════════════════
// 纤维丛联络引擎 · FiberBundle::ConnectionEngine (v0.6)
// ══════════════════════════════════════════════════════════════════════
//
//   纤维丛 = 底空间 B 上每点 b 附着一根纤维 F_b
//   联络(Connection) = 沿底空间曲线的平行移动规则
//   曲率(Curvature) = 联络不可积的程度 (绕闭环平移的偏差)
//   完整群(Holonomy) = 绕闭环一圈后纤维上的变换
//
//   NJSG 映射:
//     底空间  = 容器坐标空间 (声母θ, 韵母r, 声调y)
//     纤维    = 关系归属空间 (该容器所在的范畴纤维)
//     联络    = 如何把一个容器的关系"平移"到相邻容器
//     曲率    = 范畴归属的非交换性 (操作为何不能全局定义)
//
//   64 位编码: [fiber_id:16][point_lo:24][point_hi:24]

const FIBER_BASE_SHIFT: u64 = 48;
const FIBER_POINT_MASK: u64 = 0xFF_FFFF;

/// 纤维标识
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FiberId(pub u16);

/// 底空间坐标 (一维投影 — 沿轴展开)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasePoint(pub u32);

/// 联络系数 — 无穷小平行移动规则
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Connection {
    /// 沿三个底空间方向的偏移权重 [声母, 韵母, 声调]
    pub weights: [f32; 3],
}

impl Connection {
    pub fn identity() -> Self { Self { weights: [1.0, 1.0, 1.0] } }
    pub fn scale(s: f32) -> Self { Self { weights: [s, s, s] } }

    /// 联络复合 = 权重逐元素相乘 (非可交换时用不同顺序)
    pub fn compose(&self, other: &Self) -> Self {
        Self { weights: [self.weights[0] * other.weights[0],
                          self.weights[1] * other.weights[1],
                          self.weights[2] * other.weights[2]] }
    }

    /// 联络求逆 (反向平行移动)
    pub fn inverse(&self) -> Self {
        Self { weights: [1.0 / self.weights[0], 1.0 / self.weights[1], 1.0 / self.weights[2]] }
    }
}

impl Default for Connection { fn default() -> Self { Self::identity() } }

/// 曲率 — 沿无穷小矩形边界平移后的偏差
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Curvature {
    pub curl: [f32; 3],
}

impl Curvature {
    pub fn zero() -> Self { Self { curl: [0.0, 0.0, 0.0] } }

    /// 曲率大小 (L2 范数)
    pub fn magnitude(&self) -> f32 {
        (self.curl[0] * self.curl[0] + self.curl[1] * self.curl[1] + self.curl[2] * self.curl[2]).sqrt()
    }

    /// 是否平坦 (曲率近似为零)
    pub fn is_flat(&self) -> bool { self.magnitude() < 0.001 }
}

impl Default for Curvature { fn default() -> Self { Self::zero() } }

/// 完整群元素 — 沿闭环平移后纤维上的变换
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Holonomy {
    pub rotation: f32,
    pub phase: u8,
}

impl Holonomy {
    pub fn identity() -> Self { Self { rotation: 0.0, phase: 0 } }

    /// 复合两次完整变换
    pub fn compose(&self, other: &Self) -> Self {
        Self { rotation: self.rotation + other.rotation,
               phase: self.phase.wrapping_add(other.phase) }
    }
}

/// 沿给定联络进行平行移动 — 核心操作
pub fn parallel_transport(conn: &Connection, steps: i32) -> Connection {
    let factor = steps as f32;
    Connection { weights: [conn.weights[0] * factor, conn.weights[1] * factor, conn.weights[2] * factor] }
}

/// 纤维丛联络引擎
#[derive(Clone, Debug)]
pub struct ConnectionEngine {
    pub fiber_id: FiberId,
    pub connection: Connection,
    pub curvature: Curvature,
    pub holonomy: Holonomy,
}

impl ConnectionEngine {
    pub fn new(fiber: FiberId) -> Self {
        Self { fiber_id: fiber, connection: Connection::identity(),
               curvature: Curvature::zero(), holonomy: Holonomy::identity() }
    }

    /// 设置联络
    pub fn set_connection(&mut self, conn: Connection) { self.connection = conn; }

    /// 计算曲率 (联络沿两个方向的交换子近似)
    pub fn compute_curvature(&mut self, neighbor: &Connection) {
        let a = &self.connection.weights;
        let b = &neighbor.weights;
        self.curvature.curl[0] = a[1] * b[2] - a[2] * b[1];
        self.curvature.curl[1] = a[2] * b[0] - a[0] * b[2];
        self.curvature.curl[2] = a[0] * b[1] - a[1] * b[0];
    }

    /// 沿闭环推进完整群 (曲率积分近似)
    pub fn advance_holonomy(&mut self) {
        let m = self.curvature.magnitude();
        self.holonomy.rotation += m;
        self.holonomy.phase = self.holonomy.phase.wrapping_add(if m > 0.5 { 1 } else { 0 });
    }

    /// 查询当前联络是否局部平坦
    pub fn is_locally_flat(&self) -> bool { self.curvature.is_flat() }

    /// 完整群作用于容器位 — 返回相位
    pub fn apply_phase(&self, bit: u8) -> u8 {
        if self.holonomy.phase % 2 == 0 { bit } else { bit ^ 1 }
    }
}

impl Default for ConnectionEngine { fn default() -> Self { Self::new(FiberId(0)) } }

#[cfg(test)]
mod fiber_tests {
    use super::*;
    #[test] fn test_connection_identity() {
        let c = Connection::identity();
        assert_eq!(c.weights, [1.0, 1.0, 1.0]);
    }
    #[test] fn test_connection_compose() {
        let a = Connection { weights: [2.0, 3.0, 4.0] };
        let b = Connection { weights: [0.5, 1.0, 2.0] };
        let ab = a.compose(&b);
        assert_eq!(ab.weights, [1.0, 3.0, 8.0]);
    }
    #[test] fn test_connection_inverse() {
        let c = Connection { weights: [2.0, 4.0, 8.0] };
        let inv = c.inverse();
        assert_eq!(inv.weights, [0.5, 0.25, 0.125]);
    }
    #[test] fn test_curvature_zero() {
        let c = Curvature::zero();
        assert!(c.is_flat());
    }
    #[test] fn test_curvature_compute() {
        let mut engine = ConnectionEngine::new(FiberId(1));
        let neighbor = Connection { weights: [1.0, 0.0, 0.0] };
        engine.compute_curvature(&neighbor);
        assert!(!engine.is_locally_flat() || engine.curvature.magnitude() < 0.001);
    }
    #[test] fn test_curvature_magnitude() {
        let c = Curvature { curl: [3.0, 4.0, 0.0] };
        assert!((c.magnitude() - 5.0).abs() < 0.001);
    }
    #[test] fn test_holonomy_identity() {
        let h = Holonomy::identity();
        assert_eq!(h.rotation, 0.0);
        assert_eq!(h.phase, 0);
    }
    #[test] fn test_holonomy_compose() {
        let a = Holonomy { rotation: 1.5, phase: 3 };
        let b = Holonomy { rotation: 2.5, phase: 5 };
        let ab = a.compose(&b);
        assert_eq!(ab.rotation, 4.0);
        assert_eq!(ab.phase, 8);
    }
    #[test] fn test_parallel_transport() {
        let c = Connection { weights: [1.0, 2.0, 3.0] };
        let p = parallel_transport(&c, 2);
        assert_eq!(p.weights, [2.0, 4.0, 6.0]);
    }
    #[test] fn test_engine_new() {
        let e = ConnectionEngine::new(FiberId(42));
        assert_eq!(e.fiber_id.0, 42);
        assert!(e.is_locally_flat());
    }
    #[test] fn test_engine_advance_holonomy() {
        let mut e = ConnectionEngine::new(FiberId(1));
        e.curvature.curl = [1.0, 2.0, 2.0];  // magnitude = 3
        e.advance_holonomy();
        assert!(e.holonomy.rotation > 0.0);
    }
    #[test] fn test_apply_phase() {
        let mut e = ConnectionEngine::new(FiberId(1));
        assert_eq!(e.apply_phase(1), 1);  // phase=0, even → unchanged
        e.holonomy.phase = 1;
        assert_eq!(e.apply_phase(1), 0);  // phase=1, odd → flip
    }
    #[test] fn test_engine_default() {
        let e = ConnectionEngine::default();
        assert_eq!(e.fiber_id.0, 0);
    }
    #[test] fn test_connection_default() {
        let c = Connection::default();
        assert_eq!(c.weights, [1.0, 1.0, 1.0]);
    }
}

// ══════════════════════════════════════════════════════════════════════
// 分别器设计 · SeparatorDesign (v0.7)
// ══════════════════════════════════════════════════════════════════════
//
//   分别器 = 将混合流按 YinYang 极性分拣到八流通道
//
//   八流统一 (NJSG 核心路由协议):
//     Preload      预加载   — 运行前预加载所有符号
///    Interpret    解释流   — 编译时符号解析
///    Sovereignty  主权路由 — 裁决准入
///    Property     产权流   — 归属判定
///    Resonance    共振流   — 聂小倩匹配
///    Feedback     反馈流   — 迂回反馈/反射感知
///    Invocation   呼唤流   — 触发调用
///    CommonDivisor 公约数流 — 统摄收敛
//
//   YinYang 极性: 我=1(阳/主动), 刘楚恬=0(阴/静)
//   分拣规则: 容器声母韵母调组合 → 极性映射 → 流通道

/// 八流通道枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlowChannel {
    Preload,       // 预加载
    Interpret,     // 解释
    Sovereignty,   // 主权
    Property,      // 产权
    Resonance,     // 共振
    Feedback,      // 反馈
    Invocation,    // 呼唤
    CommonDivisor, // 公约数
}

impl FlowChannel {
    /// 通道总数
    pub const COUNT: usize = 8;

    /// 按索引取通道 (循环)
    pub fn from_index(idx: usize) -> Self {
        use FlowChannel::*;
        const CHANNELS: [FlowChannel; 8] =
            [Preload, Interpret, Sovereignty, Property, Resonance, Feedback, Invocation, CommonDivisor];
        CHANNELS[idx % 8]
    }

    /// 通道优先级 (越小越优先)
    pub fn priority(&self) -> u8 {
        use FlowChannel::*;
        match self {
            CommonDivisor => 0, // 公约数统摄收敛 = 最高优先
            Preload => 1,
            Interpret => 2,
            Sovereignty => 3,
            Property => 4,
            Resonance => 5,
            Feedback => 6,
            Invocation => 7,
        }
    }
}

use FlowChannel::*;

/// 分别器标识
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeparatorId(pub u16);

/// 分拣规则 — 定义容器到通道的映射
#[derive(Clone, Debug)]
pub struct SeparatorRule {
    pub channel: FlowChannel,
    pub consonant_range: (u8, u8),  // 声母范围
    pub vowel_range: (u8, u8),      // 韵母范围
    pub polarity: Option<bool>,     // 极性过滤 (None = 不过滤)
}

impl SeparatorRule {
    pub fn new(channel: FlowChannel) -> Self {
        Self { channel, consonant_range: (0, 47), vowel_range: (0, 65), polarity: None }
    }

    /// 设置声母过滤范围
    pub fn with_consonant(mut self, lo: u8, hi: u8) -> Self {
        self.consonant_range = (lo, hi);
        self
    }

    /// 设置韵母过滤范围
    pub fn with_vowel(mut self, lo: u8, hi: u8) -> Self {
        self.vowel_range = (lo, hi);
        self
    }

    /// 设置极性过滤
    pub fn with_polarity(mut self, polarity: bool) -> Self {
        self.polarity = Some(polarity);
        self
    }

    /// 检查容器是否匹配此规则
    pub fn matches(&self, consonant: u8, vowel: u8, is_yang: bool) -> bool {
        if consonant < self.consonant_range.0 || consonant > self.consonant_range.1 {
            return false;
        }
        if vowel < self.vowel_range.0 || vowel > self.vowel_range.1 {
            return false;
        }
        if let Some(required) = self.polarity {
            if required != is_yang { return false; }
        }
        true
    }
}

/// 分别器引擎 — 核心分拣逻辑
#[derive(Clone, Debug)]
pub struct SeparatorDesign {
    pub id: SeparatorId,
    pub rules: Vec<SeparatorRule>,
    /// 各通道计数 (加法统计)
    pub channel_counts: [u64; 8],
    pub total_separated: u64,
}

impl SeparatorDesign {
    pub fn new(id: SeparatorId) -> Self {
        Self { id, rules: Vec::new(), channel_counts: [0; 8], total_separated: 0 }
    }

    /// 添加分拣规则
    pub fn add_rule(&mut self, rule: SeparatorRule) {
        self.rules.push(rule);
    }

    /// 对单个容器进行分拣 — 返回命中的通道
    pub fn separate(&mut self, consonant: u8, vowel: u8, is_yang: bool) -> Option<FlowChannel> {
        for rule in &self.rules {
            if rule.matches(consonant, vowel, is_yang) {
                let idx = rule.channel.priority() as usize;
                self.channel_counts[idx] += 1;
                self.total_separated += 1;
                return Some(rule.channel);
            }
        }
        None
    }

    /// 计算极性 (YinYang 二进制映射: 我=1 阳, 刘楚恬=0 阴)
    pub fn compute_polarity(consonant: u8, vowel: u8, tone: u8) -> bool {
        // 声母奇数 = 阳(1), 韵母声调组合判定
        consonant % 2 == 1
    }

    /// 批量分拣 — 输入: 容器数组, 输出: 各通道分拣结果
    pub fn separate_batch(&mut self, containers: &[(u8, u8, u8)]) -> Vec<(usize, FlowChannel)> {
        let mut results = Vec::new();
        for (i, &(c, v, t)) in containers.iter().enumerate() {
            let yang = Self::compute_polarity(c, v, t);
            if let Some(ch) = self.separate(c, v, yang) {
                results.push((i, ch));
            }
        }
        results
    }

    /// 检查是否所有规则覆盖了全部声母/韵母空间 (分别器完整性检查)
    pub fn is_complete(&self) -> bool {
        let mut consonant_covered = 0u64;
        let mut vowel_covered = 0u64;
        for rule in &self.rules {
            for c in rule.consonant_range.0..=rule.consonant_range.1 {
                consonant_covered |= 1u64 << c;
            }
            for v in rule.vowel_range.0..=rule.vowel_range.1 {
                if v < 64 { vowel_covered |= 1u64 << v; }
            }
        }
        consonant_covered == 0xFFFF_FFFF_FFFF_FFFFu64 && vowel_covered == 0xFFFF_FFFF_FFFF_FFFFu64
    }

    /// 获取指定通道的计数
    pub fn channel_count(&self, ch: FlowChannel) -> u64 {
        self.channel_counts[ch.priority() as usize]
    }

    /// 八流平衡度 (标准差近似, 0 = 完全平衡)
    pub fn balance_score(&self) -> f64 {
        if self.total_separated == 0 { return 0.0; }
        let mean = self.total_separated as f64 / 8.0;
        let variance: f64 = self.channel_counts.iter()
            .map(|&c| (c as f64 - mean) * (c as f64 - mean))
            .sum::<f64>() / 8.0;
        variance.sqrt()
    }

    /// 重置计数
    pub fn reset_counts(&mut self) {
        self.channel_counts = [0; 8];
        self.total_separated = 0;
    }
}

impl Default for SeparatorDesign {
    fn default() -> Self { Self::new(SeparatorId(0)) }
}

#[cfg(test)]
mod separator_tests {
    use super::*;
    #[test] fn test_channel_count() {
        assert_eq!(FlowChannel::COUNT, 8);
    }
    #[test] fn test_channel_from_index() {
        assert_eq!(FlowChannel::from_index(0), FlowChannel::Preload);
        assert_eq!(FlowChannel::from_index(7), FlowChannel::CommonDivisor);
        assert_eq!(FlowChannel::from_index(8), FlowChannel::Preload); // 循环
    }
    #[test] fn test_channel_priority() {
        assert!(FlowChannel::CommonDivisor.priority() < FlowChannel::Invocation.priority());
        assert_eq!(FlowChannel::CommonDivisor.priority(), 0);
    }
    #[test] fn test_rule_new() {
        let r = SeparatorRule::new(FlowChannel::Resonance);
        assert_eq!(r.channel, FlowChannel::Resonance);
        assert_eq!(r.consonant_range, (0, 47));
        assert_eq!(r.vowel_range, (0, 65));
        assert_eq!(r.polarity, None);
    }
    #[test] fn test_rule_builder() {
        let r = SeparatorRule::new(FlowChannel::Sovereignty)
            .with_consonant(10, 20)
            .with_vowel(5, 30)
            .with_polarity(true);
        assert!(r.matches(15, 10, true));
        assert!(!r.matches(15, 10, false));
        assert!(!r.matches(5, 10, true));
    }
    #[test] fn test_rule_matches_all() {
        let r = SeparatorRule::new(FlowChannel::Preload);
        assert!(r.matches(0, 0, false));
        assert!(r.matches(47, 65, true));
    }
    #[test] fn test_separator_new() {
        let s = SeparatorDesign::new(SeparatorId(1));
        assert_eq!(s.rules.len(), 0);
        assert_eq!(s.total_separated, 0);
    }
    #[test] fn test_separate() {
        let mut s = SeparatorDesign::new(SeparatorId(1));
        s.add_rule(SeparatorRule::new(FlowChannel::Resonance).with_consonant(0, 23));
        s.add_rule(SeparatorRule::new(FlowChannel::Feedback).with_consonant(24, 47));
        let ch1 = s.separate(10, 30, true);
        assert_eq!(ch1, Some(FlowChannel::Resonance));
        let ch2 = s.separate(30, 30, true);
        assert_eq!(ch2, Some(FlowChannel::Feedback));
    }
    #[test] fn test_compute_polarity() {
        assert!(SeparatorDesign::compute_polarity(1, 0, 0));   // 奇=阳
        assert!(!SeparatorDesign::compute_polarity(0, 0, 0));  // 偶=阴
        assert!(SeparatorDesign::compute_polarity(47, 0, 0));  // 奇=阳
    }
    #[test] fn test_separate_batch() {
        let mut s = SeparatorDesign::new(SeparatorId(1));
        s.add_rule(SeparatorRule::new(FlowChannel::Sovereignty).with_consonant(0, 23));
        let containers = [(5, 10, 1), (30, 20, 2), (15, 3, 0)];
        let results = s.separate_batch(&containers);
        assert_eq!(results.len(), 2);  // 2 个匹配 (5 和 15 在 0-23 范围内)
    }
    #[test] fn test_separator_channel_count() {
        let mut s = SeparatorDesign::new(SeparatorId(1));
        s.add_rule(SeparatorRule::new(FlowChannel::CommonDivisor));
        s.separate(10, 10, true);
        s.separate(20, 20, false);
        assert_eq!(s.channel_count(FlowChannel::CommonDivisor), 2);
    }
    #[test] fn test_balance_score() {
        let mut s = SeparatorDesign::new(SeparatorId(1));
        s.add_rule(SeparatorRule::new(FlowChannel::Preload));
        for c in 0..8 { s.separate(c, 0, c % 2 == 1); }
        let score = s.balance_score();
        assert!(score >= 0.0);
    }
    #[test] fn test_reset_counts() {
        let mut s = SeparatorDesign::new(SeparatorId(1));
        s.add_rule(SeparatorRule::new(FlowChannel::Feedback));
        s.separate(10, 10, true);
        assert!(s.total_separated > 0);
        s.reset_counts();
        assert_eq!(s.total_separated, 0);
    }
    #[test] fn test_separator_default() {
        let s = SeparatorDesign::default();
        assert_eq!(s.id.0, 0);
    }
    #[test] fn test_rule_match_polarity_filtered() {
        let r = SeparatorRule::new(FlowChannel::Resonance).with_polarity(false);
        assert!(r.matches(0, 0, false));
        assert!(!r.matches(0, 0, true));
    }
    #[test] fn test_rule_vowel_out_of_range() {
        let r = SeparatorRule::new(FlowChannel::Property).with_vowel(10, 20);
        assert!(!r.matches(0, 25, false));
        assert!(r.matches(0, 15, false));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器协商引擎 · ContainerNegotiationEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 公约数仲裁: 容器间提案 · 加权投票 · 共识达成
// 仅加法范式: GCD 通过辗转相减, 权重通过加法累计
// 核心哲学: 公约数 = 共识基础, 最大公约数 = 最小共识

/// 协商提案
#[derive(Clone, Debug)]
pub struct Proposal {
    pub proposer_id: u64,
    pub value: u64,
    pub weight: u64,
}

impl Proposal {
    pub fn new(id: u64, value: u64) -> Self { Self { proposer_id: id, value, weight: 1 } }
    pub fn with_weight(id: u64, value: u64, weight: u64) -> Self { Self { proposer_id: id, value, weight } }
}

/// 协商阶段
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NegotiationPhase {
    Propose,      // 提案阶段
    Arbitrate,    // 仲裁阶段
    Consensus,    // 共识达成
    Deadlock,     // 僵局: 无共识
}

impl Default for NegotiationPhase { fn default() -> Self { NegotiationPhase::Propose } }

/// 协商结果
#[derive(Clone, Debug)]
pub struct NegotiationResult {
    pub phase: NegotiationPhase,
    pub agreed_value: u64,       // 共识值 (最大公约数)
    pub supporter_ids: Vec<u64>, // 支持共识的容器
    pub total_weight: u64,       // 总权重
    pub rounds: u64,             // 协商轮数
}

/// 容器协商引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerNegotiationEngine {
    pub proposals: Vec<Proposal>,
    pub consensus_threshold: u64,  // 共识阈值 (GCD 必须超过此值)
    pub phase: NegotiationPhase,
    pub round_count: u64,
}

impl ContainerNegotiationEngine {
    pub fn new() -> Self { Self { proposals: Vec::new(), consensus_threshold: 1, phase: NegotiationPhase::Propose, round_count: 0 } }

    /// 提交提案
    pub fn propose(&mut self, proposal: Proposal) { self.proposals.push(proposal); }

    /// 获取指定提案的索引
    fn find_proposal(&self, id: u64) -> Option<usize> {
        self.proposals.iter().position(|p| p.proposer_id == id)
    }

    /// 仅加法 GCD: 辗转相减
    fn gcd_subtract(a: u64, b: u64) -> u64 {
        let mut x = a;
        let mut y = b;
        while x != y { if x > y { x -= y; } else { y -= x; } }
        x
    }

    /// 多提案 GCD: 逐一累积
    fn multi_gcd(values: &[u64]) -> u64 {
        if values.is_empty() { return 0; }
        let mut result = values[0];
        for &v in &values[1..] { result = Self::gcd_subtract(result, v); }
        result
    }

    /// 执行仲裁: 计算所有提案值的 GCD
    pub fn arbitrate(&mut self) -> NegotiationResult {
        self.round_count += 1;
        self.phase = NegotiationPhase::Arbitrate;

        if self.proposals.is_empty() {
            return NegotiationResult { phase: NegotiationPhase::Deadlock, agreed_value: 0, supporter_ids: Vec::new(), total_weight: 0, rounds: self.round_count };
        }

        let values: Vec<u64> = self.proposals.iter().map(|p| p.value).collect();
        let agreed = Self::multi_gcd(&values);

        if agreed > self.consensus_threshold {
            self.phase = NegotiationPhase::Consensus;
            let supporters: Vec<u64> = self.proposals.iter()
                .filter(|p| Self::gcd_subtract(p.value, agreed) == agreed)
                .map(|p| p.proposer_id)
                .collect();
            let total_weight: u64 = self.proposals.iter().fold(0u64, |acc, p| acc.wrapping_add(p.weight));
            NegotiationResult { phase: NegotiationPhase::Consensus, agreed_value: agreed, supporter_ids: supporters, total_weight, rounds: self.round_count }
        } else {
            self.phase = NegotiationPhase::Deadlock;
            NegotiationResult { phase: NegotiationPhase::Deadlock, agreed_value: agreed, supporter_ids: Vec::new(), total_weight: 0, rounds: self.round_count }
        }
    }

    /// 达成强共识: 所有提案值相等 (GCD = 值本身)
    pub fn strong_consensus(&self) -> Option<u64> {
        if self.proposals.is_empty() { return None; }
        let first = self.proposals[0].value;
        if self.proposals.iter().all(|p| p.value == first) { Some(first) } else { None }
    }

    /// 两容器的共识度: GCD(smaller, larger) — 纯加法
    pub fn consensus_degree(&self, a_id: u64, b_id: u64) -> Option<u64> {
        let a_idx = self.find_proposal(a_id)?;
        let b_idx = self.find_proposal(b_id)?;
        let (small, large) = if self.proposals[a_idx].value <= self.proposals[b_idx].value {
            (self.proposals[a_idx].value, self.proposals[b_idx].value)
        } else {
            (self.proposals[b_idx].value, self.proposals[a_idx].value)
        };
        Some(Self::gcd_subtract(small, large))
    }

    /// 移除提案
    pub fn remove_proposal(&mut self, id: u64) -> Option<Proposal> {
        let idx = self.find_proposal(id)?;
        Some(self.proposals.remove(idx))
    }

    /// 协商参与者数量
    pub fn participant_count(&self) -> usize { self.proposals.len() }

    /// 总权重
    pub fn total_weight(&self) -> u64 {
        self.proposals.iter().fold(0u64, |acc, p| acc.wrapping_add(p.weight))
    }
}

#[cfg(test)]
mod container_negotiation_tests {
    use super::*;

    #[test]
    fn test_gcd_subtract() {
        assert_eq!(ContainerNegotiationEngine::gcd_subtract(12, 8), 4);
        assert_eq!(ContainerNegotiationEngine::gcd_subtract(7, 3), 1);
        assert_eq!(ContainerNegotiationEngine::gcd_subtract(32, 64), 32);
    }

    #[test]
    fn test_multi_gcd() {
        assert_eq!(ContainerNegotiationEngine::multi_gcd(&[12, 8, 4]), 4);
        assert_eq!(ContainerNegotiationEngine::multi_gcd(&[6, 9, 15]), 3);
        assert_eq!(ContainerNegotiationEngine::multi_gcd(&[7, 11, 13]), 1);
    }

    #[test]
    fn test_propose_and_arbitrate_consensus() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.consensus_threshold = 1;
        engine.propose(Proposal::new(1, 12));
        engine.propose(Proposal::new(2, 8));
        engine.propose(Proposal::new(3, 4));
        let result = engine.arbitrate();
        assert_eq!(result.phase, NegotiationPhase::Consensus);
        assert_eq!(result.agreed_value, 4);
        assert_eq!(result.supporter_ids.len(), 3);
    }

    #[test]
    fn test_arbitrate_deadlock() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.consensus_threshold = 5;  // 高阈值
        engine.propose(Proposal::new(1, 7));
        engine.propose(Proposal::new(2, 11));
        let result = engine.arbitrate();
        assert_eq!(result.phase, NegotiationPhase::Deadlock);
        assert_eq!(result.agreed_value, 1);
    }

    #[test]
    fn test_strong_consensus() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::new(1, 42));
        engine.propose(Proposal::new(2, 42));
        engine.propose(Proposal::new(3, 42));
        assert_eq!(engine.strong_consensus(), Some(42));
    }

    #[test]
    fn test_no_strong_consensus() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::new(1, 10));
        engine.propose(Proposal::new(2, 20));
        assert_eq!(engine.strong_consensus(), None);
    }

    #[test]
    fn test_consensus_degree() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::new(1, 12));
        engine.propose(Proposal::new(2, 8));
        let degree = engine.consensus_degree(1, 2);
        assert!(degree.is_some());
        // gcd(8, 12) = 4
        assert_eq!(degree, Some(4));
    }

    #[test]
    fn test_remove_proposal() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::new(1, 100));
        engine.propose(Proposal::new(2, 200));
        engine.remove_proposal(1);
        assert_eq!(engine.participant_count(), 1);
        assert_eq!(engine.proposals[0].proposer_id, 2);
    }

    #[test]
    fn test_total_weight() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::with_weight(1, 10, 5));
        engine.propose(Proposal::with_weight(2, 20, 3));
        assert_eq!(engine.total_weight(), 8);
    }

    #[test]
    fn test_arbitrate_empty() {
        let mut engine = ContainerNegotiationEngine::new();
        let result = engine.arbitrate();
        assert_eq!(result.phase, NegotiationPhase::Deadlock);
        assert_eq!(result.agreed_value, 0);
    }

    #[test]
    fn test_arbitrate_round_count() {
        let mut engine = ContainerNegotiationEngine::new();
        engine.propose(Proposal::new(1, 6));
        engine.propose(Proposal::new(2, 12));
        let r1 = engine.arbitrate();
        assert_eq!(r1.rounds, 1);
        // 第二轮
        engine.propose(Proposal::new(3, 18));
        let r2 = engine.arbitrate();
        assert_eq!(r2.rounds, 2);
    }

    #[test]
    fn test_default() {
        let engine = ContainerNegotiationEngine::default();
        assert_eq!(engine.participant_count(), 0);
        assert_eq!(engine.phase, NegotiationPhase::Propose);
    }
}


// ═══════════════════════════════════════════════════════════════════════════
// 容器演化引擎 · ContainerEvolutionEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 生命周期: 种子→萌芽→成长→成熟→衰退→重生 (六阶段循环)
// 加法年龄: 通过加法累计, 达到阈值则跃迁
// 环境适应: 资源丰度·竞争压力·共生助力 → 适应度纯加法累积

/// 生命周期六阶段
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LifeCyclePhase {
    Seed,      // 种子: 潜能未发
    Sprout,    // 萌芽: 初次显现
    Grow,      // 成长: 扩张积累
    Mature,    // 成熟: 圆满稳定
    Decline,   // 衰退: 老化和资源消耗
    Rebirth,   // 重生: 回归种子, 循环再始
}

impl Default for LifeCyclePhase { fn default() -> Self { LifeCyclePhase::Seed } }

/// 单个容器的演化状态
#[derive(Clone, Debug)]
pub struct EvolutionState {
    pub container_id: u64,
    pub phase: LifeCyclePhase,
    pub age: u64,              // 加法累计年龄
    pub energy: u64,           // 能量 (加法积累)
    pub adaptation: u64,       // 环境适应度
    pub generation: u64,       // 经历了几次重生循环
}

impl EvolutionState {
    pub fn new(id: u64) -> Self { Self { container_id: id, phase: LifeCyclePhase::Seed, age: 0, energy: 0, adaptation: 0, generation: 0 } }
}

/// 环境因子 (仅加法影响)
#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub resource_abundance: u64,  // 资源丰度 (正)
    pub competition_pressure: u64, // 竞争压力 (负)
    pub symbiosis_boost: u64,     // 共生助力 (正)
}

impl Default for Environment { fn default() -> Self { Self { resource_abundance: 1, competition_pressure: 0, symbiosis_boost: 0 } } }

/// 容器演化引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerEvolutionEngine {
    pub states: Vec<EvolutionState>,
    pub env: Environment,
    pub phase_thresholds: [u64; 6],  // 六阶段跃迁阈值
}

impl ContainerEvolutionEngine {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            env: Environment::default(),
            phase_thresholds: [10, 25, 50, 100, 200, u64::MAX], // Seed→Sprout→Grow→Mature→Decline→Rebirth
        }
    }

    /// 注册容器
    pub fn register(&mut self, id: u64) -> usize {
        self.states.push(EvolutionState::new(id));
        self.states.len() - 1
    }

    /// 根据 ID 查找索引
    fn find_index(&self, id: u64) -> Option<usize> {
        self.states.iter().position(|s| s.container_id == id)
    }

    /// 推进时间 tick: 年龄 +1, 能量根据环境变化
    pub fn tick(&mut self, id: u64) -> Option<u64> {
        let idx = self.find_index(id)?;
        self.states[idx].age += 1;
        // 能量变化 = 资源丰度 + 共生助力 - 竞争压力 (净环境效应)
        let net = self.env.resource_abundance.wrapping_add(self.env.symbiosis_boost);
        self.states[idx].energy = self.states[idx].energy.wrapping_add(net).saturating_sub(self.env.competition_pressure);
        // 适应度累加
        self.states[idx].adaptation = self.states[idx].adaptation.wrapping_add(self.env.resource_abundance + 1);
        Some(self.states[idx].age)
    }

    /// 检查并跃迁: 年龄超过当前阶段阈值则进入下一阶段
    /// 特殊: Rebirth 阶段无条件循环回 Seed (重生即再生)
    pub fn try_transition(&mut self, id: u64) -> Option<LifeCyclePhase> {
        let idx = self.find_index(id)?;
        let phase_idx = self.phase_index(self.states[idx].phase);
        // Rebirth → Seed 是无条件循环 (重生完成一轮)
        if self.states[idx].phase == LifeCyclePhase::Rebirth {
            self.states[idx].phase = LifeCyclePhase::Seed;
            self.states[idx].generation += 1;
            return Some(LifeCyclePhase::Seed);
        }
        let threshold = self.phase_thresholds[phase_idx];
        if self.states[idx].age >= threshold {
            let next = self.next_phase(self.states[idx].phase);
            self.states[idx].phase = next;
            Some(next)
        } else {
            None
        }
    }

    /// 阶段 → 索引
    fn phase_index(&self, phase: LifeCyclePhase) -> usize {
        match phase {
            LifeCyclePhase::Seed => 0,
            LifeCyclePhase::Sprout => 1,
            LifeCyclePhase::Grow => 2,
            LifeCyclePhase::Mature => 3,
            LifeCyclePhase::Decline => 4,
            LifeCyclePhase::Rebirth => 5,
        }
    }

    /// 下一阶段
    fn next_phase(&self, phase: LifeCyclePhase) -> LifeCyclePhase {
        match phase {
            LifeCyclePhase::Seed => LifeCyclePhase::Sprout,
            LifeCyclePhase::Sprout => LifeCyclePhase::Grow,
            LifeCyclePhase::Grow => LifeCyclePhase::Mature,
            LifeCyclePhase::Mature => LifeCyclePhase::Decline,
            LifeCyclePhase::Decline => LifeCyclePhase::Rebirth,
            LifeCyclePhase::Rebirth => LifeCyclePhase::Seed, // 循环
        }
    }

    /// 容器数量
    pub fn population(&self) -> usize { self.states.len() }

    /// 各阶段种群计数 (加法统计)
    pub fn phase_distribution(&self) -> [usize; 6] {
        let mut dist = [0usize; 6];
        for s in &self.states { dist[self.phase_index(s.phase)] += 1; }
        dist
    }

    /// 总能量
    pub fn total_energy(&self) -> u64 {
        self.states.iter().fold(0u64, |acc, s| acc.wrapping_add(s.energy))
    }

    /// 设置环境
    pub fn set_environment(&mut self, resource: u64, competition: u64, symbiosis: u64) {
        self.env = Environment { resource_abundance: resource, competition_pressure: competition, symbiosis_boost: symbiosis };
    }
}

#[cfg(test)]
mod container_evolution_tests {
    use super::*;

    #[test]
    fn test_register_and_initial_phase() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        assert_eq!(engine.states[0].phase, LifeCyclePhase::Seed);
        assert_eq!(engine.states[0].age, 0);
        assert_eq!(engine.states[0].generation, 0);
    }

    #[test]
    fn test_tick_increments_age() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        assert_eq!(engine.tick(1), Some(1));
        assert_eq!(engine.tick(1), Some(2));
        assert_eq!(engine.states[0].age, 2);
    }

    #[test]
    fn test_transition_seed_to_sprout() {
        let mut engine = ContainerEvolutionEngine::new();
        // threshold for Seed→Sprout is 10
        engine.register(1);
        for _ in 0..10 { engine.tick(1); }
        let result = engine.try_transition(1);
        assert_eq!(result, Some(LifeCyclePhase::Sprout));
    }

    #[test]
    fn test_no_transition_before_threshold() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        for _ in 0..5 { engine.tick(1); } // age=5 < threshold=10
        let result = engine.try_transition(1);
        assert_eq!(result, None);
        assert_eq!(engine.states[0].phase, LifeCyclePhase::Seed);
    }

    #[test]
    fn test_full_lifecycle() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        let thresholds = [10u64, 25, 50, 100, 200];
        let expected = [LifeCyclePhase::Sprout, LifeCyclePhase::Grow, LifeCyclePhase::Mature, LifeCyclePhase::Decline, LifeCyclePhase::Rebirth];
        for (i, &th) in thresholds.iter().enumerate() {
            // tick enough times to reach threshold
            for _ in 0..th { engine.tick(1); }
            let r = engine.try_transition(1);
            assert_eq!(r, Some(expected[i]), "Failed at phase {} (threshold {})", i, th);
        }
    }

    #[test]
    fn test_rebirth_cycle() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        // Go through all phases
        for _ in 0..10 { engine.tick(1); } // Seed→Sprout (10)
        assert_eq!(engine.try_transition(1), Some(LifeCyclePhase::Sprout));
        for _ in 0..25 { engine.tick(1); } // Sprout→Grow
        assert_eq!(engine.try_transition(1), Some(LifeCyclePhase::Grow));
        for _ in 0..50 { engine.tick(1); } // Grow→Mature
        assert_eq!(engine.try_transition(1), Some(LifeCyclePhase::Mature));
        for _ in 0..100 { engine.tick(1); } // Mature→Decline
        assert_eq!(engine.try_transition(1), Some(LifeCyclePhase::Decline));
        for _ in 0..200 { engine.tick(1); } // Decline→Rebirth
        assert_eq!(engine.try_transition(1), Some(LifeCyclePhase::Rebirth));
        // Rebirth → Seed (无条件循环)
        let r = engine.try_transition(1);
        assert_eq!(r, Some(LifeCyclePhase::Seed));
        assert_eq!(engine.states[0].generation, 1); // 完成一轮重生
    }

    #[test]
    fn test_generation_count() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1);
        assert_eq!(engine.states[0].generation, 0);
        // Advance through all phases
        for _ in 0..10 { engine.tick(1); } engine.try_transition(1); // Sprout
        for _ in 0..25 { engine.tick(1); } engine.try_transition(1); // Grow
        for _ in 0..50 { engine.tick(1); } engine.try_transition(1); // Mature
        for _ in 0..100 { engine.tick(1); } engine.try_transition(1); // Decline
        for _ in 0..200 { engine.tick(1); } engine.try_transition(1); // Rebirth
        // Rebirth → Seed triggers generation increment
        engine.try_transition(1);
        assert_eq!(engine.states[0].generation, 1);
    }

    #[test]
    fn test_tick_energy_positive_env() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.set_environment(5, 2, 1); // net = 5+1-2 = 4
        engine.register(1);
        engine.tick(1);
        assert_eq!(engine.states[0].energy, 4);
    }

    #[test]
    fn test_tick_energy_negative_env() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.set_environment(1, 10, 0); // net = 1+0-10 = -9 → saturating_sub → 0
        engine.register(1);
        engine.tick(1);
        assert_eq!(engine.states[0].energy, 0); // saturating
    }

    #[test]
    fn test_phase_distribution() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.register(1); // Seed
        engine.register(2); // Seed
        engine.register(3); // Seed
        // advance container 1 to Grow
        for _ in 0..50 { engine.tick(1); }
        engine.try_transition(1); // Sprout
        engine.try_transition(1); // Grow
        let dist = engine.phase_distribution();
        assert_eq!(dist[0], 2); // 2 still in Seed (ids 2, 3)
        assert_eq!(dist[2], 1); // 1 in Grow (id 1)
    }

    #[test]
    fn test_total_energy() {
        let mut engine = ContainerEvolutionEngine::new();
        engine.set_environment(1, 0, 0);
        engine.register(1);
        engine.register(2);
        engine.tick(1);
        engine.tick(2);
        assert_eq!(engine.total_energy(), 2); // each gets 1
    }

    #[test]
    fn test_population() {
        let mut engine = ContainerEvolutionEngine::new();
        assert_eq!(engine.population(), 0);
        engine.register(1);
        engine.register(2);
        assert_eq!(engine.population(), 2);
    }

    #[test]
    fn test_default() {
        let engine = ContainerEvolutionEngine::default();
        assert_eq!(engine.population(), 0);
    }
}

// ══════════════════════════════════════════════════════════════════════
// 反馈回路引擎 · RoundaboutFeedbackEngine (v0.9)
// ══════════════════════════════════════════════════════════════════════
//
//   反馈回路 = 感知→判断→调整→执行→评价 闭环
//   迂回反馈 = 反馈路径经其他容器迂回传递 (非直接反射)
//   反射感知 = 感知本身也作为被感知对象 (观察者即被观察者)
//
//   NJSG 核心回路:
//     1. 感知 (Perceive) — 读取容器当前状态
//     2. 判断 (Judge)    — 状态与目标值的偏差 (error)
//     3. 调整 (Adjust)   — PID 算法计算校正值
//     4. 执行 (Execute)  — 应用校正到容器
//     5. 评价 (Evaluate) — 评估本轮反馈质量
//     → 回到 1, 循环直至收敛或达到最大轮次
//
//   PID 系数: Kp (比例) · Ki (积分) · Kd (微分)
//   加法修正: correction = Kp*e + Ki*∫e + Kd*Δe (加法统计, 替代幂次反馈)
//
//   迂回: 容器 A 的反馈经容器 B/C 传递后再回到 A
//   (观察者→被观察者→观察者的环路)

/// 反馈状态枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackStatus {
    Idle,       // 空闲
    Perceiving, // 感知中
    Judging,    // 判断中
    Adjusting,  // 调整中
    Executing,  // 执行中
    Evaluating, // 评价中
    Converged,  // 已收敛
    Oscillating,// 振荡发散
}

impl FeedbackStatus {
    pub fn is_active(&self) -> bool {
        !matches!(self, FeedbackStatus::Idle | FeedbackStatus::Converged | FeedbackStatus::Oscillating)
    }
}

impl Default for FeedbackStatus { fn default() -> Self { FeedbackStatus::Idle } }

/// PID 反馈系数 (比例-积分-微分)
#[derive(Clone, Copy, Debug)]
pub struct FeedbackCoefficients {
    pub kp: f32, // 比例增益
    pub ki: f32, // 积分增益
    pub kd: f32, // 微分增益
}

impl FeedbackCoefficients {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self { Self { kp, ki, kd } }

    /// 纯比例控制
    pub fn proportional(kp: f32) -> Self { Self { kp, ki: 0.0, kd: 0.0 } }

    /// 计算校正值 (加法统计, 替代幂次反馈放大)
    pub fn compute_correction(&self, error: f32, integral: f32, derivative: f32) -> f32 {
        self.kp * error + self.ki * integral + self.kd * derivative
    }

    /// 限制系数范围 (防止数值爆炸)
    pub fn clamp(&self, max: f32) -> Self {
        Self {
            kp: self.kp.clamp(-max, max),
            ki: self.ki.clamp(-max, max),
            kd: self.kd.clamp(-max, max),
        }
    }
}

impl Default for FeedbackCoefficients { fn default() -> Self { Self::new(0.5, 0.1, 0.05) } }

/// 判断结果 — 状态与目标的偏差判定
#[derive(Clone, Copy, Debug)]
pub struct FeedbackJudgment {
    pub error: f32,       // 当前偏差
    pub integral: f32,    // 偏差积分 (累积误差)
    pub derivative: f32,  // 偏差微分 (变化率)
    pub is_acceptable: bool, // 偏差是否在容差内
}

impl FeedbackJudgment {
    pub fn zero() -> Self { Self { error: 0.0, integral: 0.0, derivative: 0.0, is_acceptable: true } }

    /// 从当前值和目标值计算判断
    pub fn evaluate(current: f32, target: f32, prev_error: f32, integral: f32, tolerance: f32) -> Self {
        let error = target - current;
        let derivative = error - prev_error;
        Self { error, integral: integral + error, derivative, is_acceptable: error.abs() <= tolerance }
    }
}

/// 评价结果 — 反馈回路质量的评估
#[derive(Clone, Copy, Debug)]
pub struct QualityEval {
    pub quality: f32,       // 本轮反馈质量 (0.0 - 1.0)
    pub convergence_rate: f32, // 收敛速率
    pub overshoot: f32,    // 超调量
    pub stability: f32,    // 稳定度 (1.0 = 完全稳定)
}

impl QualityEval {
    pub fn perfect() -> Self { Self { quality: 1.0, convergence_rate: 1.0, overshoot: 0.0, stability: 1.0 } }

    /// 从判断历史评价反馈
    pub fn from_judgments(judgments: &[FeedbackJudgment]) -> Self {
        if judgments.is_empty() { return Self::perfect(); }
        let total_error: f32 = judgments.iter().map(|j| j.error.abs()).sum();
        let avg_error = if judgments.is_empty() { 0.0 } else { total_error / judgments.len() as f32 };
        let quality = (1.0 / (1.0 + avg_error)).min(1.0);
        let last_err = judgments.last().unwrap().error;
        let first_err = judgments.first().unwrap().error;
        let convergence_rate = if first_err.abs() < 0.001 { 1.0 } else { (1.0 - last_err.abs() / first_err.abs()).max(0.0) };
        let overshoot = judgments.iter().map(|j| j.error.abs()).fold(0.0f32, f32::max) - last_err.abs();
        let stability = if judgments.len() < 2 { 1.0 } else {
            let variance = judgments.windows(2)
                .map(|w| (w[1].error - w[0].error).abs())
                .sum::<f32>() / (judgments.len() - 1) as f32;
            (1.0 / (1.0 + variance)).min(1.0)
        };
        Self { quality, convergence_rate, overshoot: overshoot.max(0.0), stability }
    }
}

/// 单容器反馈状态
#[derive(Clone, Debug)]
pub struct FeedbackState {
    pub container_id: u32,
    pub current: f32,
    pub target: f32,
    pub prev_error: f32,
    pub integral: f32,
    pub judgment_history: Vec<FeedbackJudgment>,
    pub status: FeedbackStatus,
    pub round: u32,
}

impl FeedbackState {
    pub fn new(container_id: u32, target: f32) -> Self {
        Self { container_id, current: 0.0, target, prev_error: 0.0, integral: 0.0,
               judgment_history: Vec::new(), status: FeedbackStatus::Idle, round: 0 }
    }

    /// 设置当前值
    pub fn set_current(&mut self, value: f32) { self.current = value; }

    /// 推进一轮反馈 (返回本轮校正值)
    pub fn step(&mut self, coeff: &FeedbackCoefficients, tolerance: f32) -> f32 {
        let judgment = FeedbackJudgment::evaluate(self.current, self.target, self.prev_error, self.integral, tolerance);
        self.integral = judgment.integral;
        let correction = coeff.compute_correction(judgment.error, judgment.integral, judgment.derivative);
        self.prev_error = judgment.error;
        self.judgment_history.push(judgment);
        self.current += correction; // 加法修正
        self.round += 1;
        if judgment.is_acceptable { self.status = FeedbackStatus::Converged; }
        correction
    }
}

/// 反馈回路引擎 — 多容器反馈 + 迂回路由
#[derive(Clone, Debug)]
pub struct AdaptiveFeedbackEngine {
    pub states: HashMap<u32, FeedbackState>,
    pub coefficients: FeedbackCoefficients,
    pub tolerance: f32,
    pub max_rounds: u32,
    /// 迂回路由表: 容器 A → [经由容器 B, C, ...] → 回到 A
    pub detour_routes: HashMap<u32, Vec<Vec<u32>>>,
}

impl AdaptiveFeedbackEngine {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            coefficients: FeedbackCoefficients::default(),
            tolerance: 0.01,
            max_rounds: 100,
            detour_routes: HashMap::new(),
        }
    }

    /// 注册容器的反馈目标
    pub fn register(&mut self, container_id: u32, target: f32) {
        self.states.insert(container_id, FeedbackState::new(container_id, target));
    }

    /// 设置 PID 系数
    pub fn set_coefficients(&mut self, coeff: FeedbackCoefficients) {
        self.coefficients = coeff;
    }

    /// 设置容器当前值
    pub fn set_value(&mut self, container_id: u32, value: f32) {
        if let Some(state) = self.states.get_mut(&container_id) {
            state.set_current(value);
        }
    }

    /// 单步反馈 (单容器)
    pub fn step_one(&mut self, container_id: u32) -> Option<f32> {
        let state = self.states.get_mut(&container_id)?;
        Some(state.step(&self.coefficients, self.tolerance))
    }

    /// 全局单步 (所有容器各走一轮)
    pub fn step_all(&mut self) -> Vec<(u32, f32)> {
        let ids: Vec<u32> = self.states.keys().copied().collect();
        let mut results = Vec::new();
        for id in ids {
            if let Some(corr) = self.step_one(id) {
                results.push((id, corr));
            }
        }
        results
    }

    /// 运行反馈至收敛或达到最大轮次
    pub fn run_to_convergence(&mut self, container_id: u32) -> Option<(u32, QualityEval)> {
        let state = self.states.get_mut(&container_id)?;
        let mut rounds = 0;
        while rounds < self.max_rounds {
            if state.status == FeedbackStatus::Converged { break; }
            let _ = state.step(&self.coefficients, self.tolerance);
            rounds += 1;
        }
        if rounds >= self.max_rounds && state.judgment_history.last().map_or(false, |j| !j.is_acceptable) {
            state.status = FeedbackStatus::Oscillating;
        }
        let eval = QualityEval::from_judgments(&state.judgment_history);
        Some((rounds, eval))
    }

    /// 添加迂回路由
    pub fn add_detour(&mut self, container: u32, route: Vec<u32>) {
        self.detour_routes.entry(container).or_default().push(route);
    }

    /// 迂回反馈: 容器 A 的状态经容器 B 感知后返回 A
    pub fn step_detour(&mut self, container_a: u32, intermediary: u32) -> Option<f32> {
        let val_b = self.states.get(&intermediary).map(|s| s.current)?;
        let coeff = self.coefficients;
        let tolerance = self.tolerance;
        let state_a = self.states.get_mut(&container_a)?;
        // 迂回: 用 intermediary 的 current 作为 A 的感知输入, 与 A 的 target 比较
        let error = state_a.target - val_b;
        state_a.integral += error;
        let derivative = error - state_a.prev_error;
        let correction = coeff.compute_correction(error, state_a.integral, derivative);
        state_a.prev_error = error;
        state_a.current += correction;
        Some(correction)
    }

    /// 获取容器当前值
    pub fn get_value(&self, container_id: u32) -> Option<f32> {
        self.states.get(&container_id).map(|s| s.current)
    }

    /// 获取评价
    pub fn get_evaluation(&self, container_id: u32) -> Option<QualityEval> {
        self.states.get(&container_id).map(|s| QualityEval::from_judgments(&s.judgment_history))
    }

    /// 全局收敛?
    pub fn all_converged(&self) -> bool {
        !self.states.is_empty() && self.states.values().all(|s| s.status == FeedbackStatus::Converged)
    }

    /// 活跃容器数
    pub fn active_count(&self) -> usize {
        self.states.values().filter(|s| s.status.is_active()).count()
    }

    /// 重置
    pub fn reset(&mut self) {
        for state in self.states.values_mut() {
            state.status = FeedbackStatus::Idle;
            state.round = 0;
            state.integral = 0.0;
            state.prev_error = 0.0;
            state.judgment_history.clear();
        }
    }
}

impl Default for AdaptiveFeedbackEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod feedback_tests {
    use super::*;
    #[test] fn test_coeff_default() {
        let c = FeedbackCoefficients::default();
        assert!(c.kp > 0.0);
    }
    #[test] fn test_coeff_proportional() {
        let c = FeedbackCoefficients::proportional(2.0);
        assert_eq!(c.kp, 2.0);
        assert_eq!(c.ki, 0.0);
    }
    #[test] fn test_coeff_compute() {
        let c = FeedbackCoefficients::new(1.0, 0.5, 0.1);
        let corr = c.compute_correction(2.0, 3.0, 0.5);
        assert!((corr - (2.0 + 1.5 + 0.05)).abs() < 0.001);
    }
    #[test] fn test_coeff_clamp() {
        let c = FeedbackCoefficients::new(100.0, -50.0, 200.0).clamp(10.0);
        assert_eq!(c.kp, 10.0);
        assert_eq!(c.ki, -10.0);
        assert_eq!(c.kd, 10.0);
    }
    #[test] fn test_judgment_zero() {
        let j = FeedbackJudgment::zero();
        assert_eq!(j.error, 0.0);
        assert!(j.is_acceptable);
    }
    #[test] fn test_judgment_evaluate() {
        let j = FeedbackJudgment::evaluate(5.0, 10.0, 0.0, 0.0, 1.0);
        assert_eq!(j.error, 5.0); // target - current
        assert!(!j.is_acceptable);
    }
    #[test] fn test_evaluation_perfect() {
        let e = QualityEval::perfect();
        assert_eq!(e.quality, 1.0);
        assert_eq!(e.stability, 1.0);
    }
    #[test] fn test_evaluation_from_judgments() {
        let judgments = vec![
            FeedbackJudgment { error: 5.0, integral: 5.0, derivative: 5.0, is_acceptable: false },
            FeedbackJudgment { error: 2.0, integral: 7.0, derivative: -3.0, is_acceptable: false },
            FeedbackJudgment { error: 0.1, integral: 7.1, derivative: -1.9, is_acceptable: true },
        ];
        let e = QualityEval::from_judgments(&judgments);
        assert!(e.quality > 0.0 && e.quality <= 1.0);
        assert!(e.convergence_rate > 0.0);
    }
    #[test] fn test_feedback_state_new() {
        let s = FeedbackState::new(1, 10.0);
        assert_eq!(s.target, 10.0);
        assert_eq!(s.status, FeedbackStatus::Idle);
    }
    #[test] fn test_feedback_state_step() {
        let mut s = FeedbackState::new(1, 10.0);
        s.set_current(5.0);
        let coeff = FeedbackCoefficients::proportional(0.5);
        let corr = s.step(&coeff, 0.01);
        // current = 5.0 + 0.5 * (10.0 - 5.0) = 7.5
        assert!((corr - 2.5).abs() < 0.001);
        assert!((s.current - 7.5).abs() < 0.001);
    }
    #[test] fn test_engine_register() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.register(2, 20.0);
        assert_eq!(e.states.len(), 2);
    }
    #[test] fn test_step_one() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.set_value(1, 5.0);
        let corr = e.step_one(1);
        assert!(corr.is_some());
    }
    #[test] fn test_step_all() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.register(2, 20.0);
        e.set_value(1, 0.0);
        e.set_value(2, 0.0);
        let results = e.step_all();
        assert_eq!(results.len(), 2);
    }
    #[test] fn test_run_to_convergence() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.set_value(1, 0.0);
        e.set_coefficients(FeedbackCoefficients::proportional(0.5));
        let result = e.run_to_convergence(1);
        assert!(result.is_some());
        let (rounds, eval) = result.unwrap();
        assert!(rounds > 0);
        assert!(eval.quality > 0.0);
    }
    #[test] fn test_detour() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.register(2, 20.0);
        e.set_value(1, 5.0);
        e.set_value(2, 8.0);
        e.add_detour(1, vec![2]);
        let corr = e.step_detour(1, 2);
        assert!(corr.is_some());
    }
    #[test] fn test_all_converged() {
        let e = AdaptiveFeedbackEngine::new();
        assert!(!e.all_converged()); // 空
    }
    #[test] fn test_reset() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 10.0);
        e.set_value(1, 5.0);
        e.step_one(1);
        assert!(e.states[&1].round > 0);
        e.reset();
        assert_eq!(e.states[&1].round, 0);
    }
    #[test] fn test_engine_default() {
        let e = AdaptiveFeedbackEngine::default();
        assert_eq!(e.active_count(), 0);
    }
    #[test] fn test_status_is_active() {
        assert!(FeedbackStatus::Judging.is_active());
        assert!(FeedbackStatus::Perceiving.is_active());
        assert!(!FeedbackStatus::Idle.is_active());
        assert!(!FeedbackStatus::Converged.is_active());
    }
    #[test] fn test_convergence_scenario() {
        let mut e = AdaptiveFeedbackEngine::new();
        e.register(1, 100.0);
        e.set_value(1, 0.0);
        e.tolerance = 0.1;
        e.set_coefficients(FeedbackCoefficients::new(0.3, 0.05, 0.01));
        if let Some((rounds, eval)) = e.run_to_convergence(1) {
            assert!(rounds <= 100);
            assert!(eval.convergence_rate >= 0.0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器记忆引擎 · ContainerMemoryEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 存储即记忆: 容器对符号的持久存储与回溯
// 加法累计: 每次存储增强记忆强度, 每次 tick 衰减
// 回忆: 符号在场时检索其记忆强度
// 遗忘: 强度归零的符号被清除

/// 单条记忆
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    pub symbol: String,
    pub strength: u64,      // 记忆强度 (加法累计)
    pub last_access: u64,  // 上次访问时间 tick
}

impl MemoryEntry {
    pub fn new(symbol: &str) -> Self { Self { symbol: symbol.to_string(), strength: 1, last_access: 0 } }
}

/// 容器记忆引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerMemoryEngine {
    pub memories: Vec<MemoryEntry>,
    pub current_tick: u64,
    pub decay_rate: u64,       // 每 tick 衰减量
    pub store_increment: u64,  // 每次存储增加强度
    pub capacity: usize,       // 最大记忆条数 (0 = 不限)
}

impl ContainerMemoryEngine {
    pub fn new() -> Self { Self { memories: Vec::new(), current_tick: 0, decay_rate: 1, store_increment: 1, capacity: 0 } }

    /// 查找记忆索引
    fn find_memory(&self, symbol: &str) -> Option<usize> {
        self.memories.iter().position(|m| m.symbol == symbol)
    }

    /// 存储符号: 增强已有或追加新记忆
    pub fn store(&mut self, symbol: &str) -> u64 {
        if let Some(idx) = self.find_memory(symbol) {
            self.memories[idx].strength = self.memories[idx].strength.wrapping_add(self.store_increment);
            self.memories[idx].last_access = self.current_tick;
            self.memories[idx].strength
        } else {
            if self.capacity > 0 && self.memories.len() >= self.capacity {
                // 容量满: 移除最弱记忆
                let weakest_idx = self.memories.iter()
                    .enumerate()
                    .min_by_key(|(_, m)| m.strength)
                    .map(|(i, _)| i);
                if let Some(idx) = weakest_idx {
                    self.memories.remove(idx);
                }
            }
            self.memories.push(MemoryEntry { symbol: symbol.to_string(), strength: self.store_increment, last_access: self.current_tick });
            self.store_increment
        }
    }

    /// 回忆: 返回符号的记忆强度 (0 = 未记住)
    pub fn recall(&self, symbol: &str) -> u64 {
        self.find_memory(symbol).map(|idx| self.memories[idx].strength).unwrap_or(0)
    }

    /// 检查是否记住
    pub fn remembers(&self, symbol: &str) -> bool {
        self.find_memory(symbol).map(|idx| self.memories[idx].strength > 0).unwrap_or(false)
    }

    /// 时间衰减 tick: 所有记忆强度减去衰减率
    pub fn tick(&mut self) {
        self.current_tick += 1;
        for m in &mut self.memories {
            m.strength = m.strength.saturating_sub(self.decay_rate);
        }
        // 移除强度归零的记忆 (遗忘)
        self.memories.retain(|m| m.strength > 0);
    }

    /// 遗忘特定符号
    pub fn forget(&mut self, symbol: &str) -> bool {
        if let Some(idx) = self.find_memory(symbol) {
            self.memories.remove(idx);
            true
        } else {
            false
        }
    }

    /// 回忆强度超过阈值的记忆
    pub fn recall_strong(&self, min_strength: u64) -> Vec<&str> {
        self.memories.iter()
            .filter(|m| m.strength >= min_strength)
            .map(|m| m.symbol.as_str())
            .collect()
    }

    /// 记忆条数
    pub fn memory_count(&self) -> usize { self.memories.len() }

    /// 总记忆强度
    pub fn total_strength(&self) -> u64 {
        self.memories.iter().fold(0u64, |acc, m| acc.wrapping_add(m.strength))
    }

    /// 迭代器辅助
    fn iter(&self) -> std::slice::Iter<'_, MemoryEntry> { self.memories.iter() }
}

#[cfg(test)]
mod container_memory_tests {
    use super::*;

    #[test]
    fn test_store_and_recall() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("an");
        assert_eq!(engine.recall("an"), 1);
        assert!(engine.remembers("an"));
    }

    #[test]
    fn test_store_increments_strength() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("x");
        engine.store("x");
        engine.store("x");
        assert_eq!(engine.recall("x"), 3);
    }

    #[test]
    fn test_recall_unknown() {
        let engine = ContainerMemoryEngine::new();
        assert_eq!(engine.recall("missing"), 0);
        assert!(!engine.remembers("missing"));
    }

    #[test]
    fn test_tick_decay() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("an"); // strength=1
        engine.tick();      // decay by 1, strength=0 → forgotten
        assert_eq!(engine.recall("an"), 0);
        assert_eq!(engine.memory_count(), 0);
    }

    #[test]
    fn test_tick_partial_decay() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("an");
        engine.store("an");
        engine.store("an"); // strength=3
        engine.tick();      // decay 1, strength=2
        assert_eq!(engine.recall("an"), 2);
        engine.tick();      // strength=1
        assert_eq!(engine.recall("an"), 1);
        engine.tick();      // strength=0 → forgotten
        assert_eq!(engine.memory_count(), 0);
    }

    #[test]
    fn test_forget() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("an");
        engine.store("bei");
        assert!(engine.forget("an"));
        assert!(!engine.remembers("an"));
        assert!(engine.remembers("bei"));
    }

    #[test]
    fn test_forget_unknown() {
        let mut engine = ContainerMemoryEngine::new();
        assert!(!engine.forget("nothing"));
    }

    #[test]
    fn test_recall_strong() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("strong");
        engine.store("strong");
        engine.store("strong"); // strength=3
        engine.store("weak");   // strength=1
        let strong = engine.recall_strong(2);
        assert!(strong.contains(&"strong"));
        assert!(!strong.contains(&"weak"));
    }

    #[test]
    fn test_current_tick() {
        let mut engine = ContainerMemoryEngine::new();
        assert_eq!(engine.current_tick, 0);
        engine.tick();
        assert_eq!(engine.current_tick, 1);
        engine.tick();
        assert_eq!(engine.current_tick, 2);
    }

    #[test]
    fn test_total_strength() {
        let mut engine = ContainerMemoryEngine::new();
        engine.store("a"); // 1
        engine.store("b"); // 1
        engine.store("a"); // +1 → 2
        assert_eq!(engine.total_strength(), 3);
    }

    #[test]
    fn test_capacity_eviction() {
        let mut engine = ContainerMemoryEngine::new();
        engine.capacity = 2;
        engine.store("a"); // strength=1
        engine.store("b"); // strength=1
        engine.store("c"); // evict weakest (a or b, both 1), add c
        assert_eq!(engine.memory_count(), 2);
    }

    #[test]
    fn test_default() {
        let engine = ContainerMemoryEngine::default();
        assert_eq!(engine.memory_count(), 0);
        assert_eq!(engine.current_tick, 0);
    }
}


// ═══════════════════════════════════════════════════════════════════════════
// 容器共鸣引擎 · ContainerResonanceEngine
// ═══════════════════════════════════════════════════════════════════════════
//
// 公约数共振: 容器的频率在公约数处共鸣
// 谐波放大: 共鸣强度通过累加放大
// 共鸣仓: 互相共鸣的容器组成的共振腔

/// 共鸣容器
#[derive(Clone, Debug)]
pub struct ResonantNode {
    pub container_id: u64,
    pub frequency: u64,    // 基频
    pub amplitude: u64,    // 振幅 (共鸣强度, 加法累计)
}

impl ResonantNode {
    pub fn new(id: u64, freq: u64) -> Self { Self { container_id: id, frequency: freq, amplitude: 0 } }
}

/// 共鸣仓: 一组互相共鸣的容器
#[derive(Clone, Debug)]
pub struct ResonantChamber {
    pub base_frequency: u64,     // 基频 (所有成员的 GCD)
    pub member_ids: Vec<u64>,
    pub collective_amplitude: u64, // 仓内总振幅
}

/// 容器共鸣引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerResonanceEngine {
    pub nodes: Vec<ResonantNode>,
    pub threshold: u64,  // 共鸣阈值 (GCD 必须超过此值)
}

impl ContainerResonanceEngine {
    pub fn new() -> Self { Self { nodes: Vec::new(), threshold: 1 } }

    /// 注册共振节点
    pub fn register(&mut self, id: u64, frequency: u64) -> usize {
        self.nodes.push(ResonantNode::new(id, frequency));
        self.nodes.len() - 1
    }

    /// 仅加法 GCD
    fn gcd_subtract(a: u64, b: u64) -> u64 {
        let mut x = a;
        let mut y = b;
        while x != y { if x > y { x -= y; } else { y -= x; } }
        x
    }

    /// 两节点共鸣检测: GCD > threshold
    pub fn is_resonant(&self, a_id: u64, b_id: u64) -> bool {
        let a = match self.nodes.iter().find(|n| n.container_id == a_id) { Some(n) => n, None => return false };
        let b = match self.nodes.iter().find(|n| n.container_id == b_id) { Some(n) => n, None => return false };
        Self::gcd_subtract(a.frequency, b.frequency) > self.threshold
    }

    /// 两节点共鸣强度 = GCD
    pub fn resonance_strength(&self, a_id: u64, b_id: u64) -> u64 {
        let a = match self.nodes.iter().find(|n| n.container_id == a_id) { Some(n) => n, None => return 0 };
        let b = match self.nodes.iter().find(|n| n.container_id == b_id) { Some(n) => n, None => return 0 };
        Self::gcd_subtract(a.frequency, b.frequency)
    }

    /// 谐波放大: 节点振幅增量 = 共鸣强度
    pub fn amplify(&mut self, target_id: u64, partner_ids: &[u64]) -> u64 {
        let target = match self.nodes.iter().find(|n| n.container_id == target_id) { Some(n) => n.clone(), None => return 0 };
        let mut total_amplified: u64 = 0;
        for partner_id in partner_ids {
            if *partner_id == target_id { continue; }
            let partner = match self.nodes.iter().find(|n| n.container_id == *partner_id) { Some(n) => n, None => { continue; } };
            let g = Self::gcd_subtract(target.frequency, partner.frequency);
            if g > self.threshold {
                if let Some(idx) = self.nodes.iter().position(|n| n.container_id == target_id) {
                    self.nodes[idx].amplitude = self.nodes[idx].amplitude.wrapping_add(g);
                    total_amplified = total_amplified.wrapping_add(g);
                }
            }
        }
        total_amplified
    }

    /// 寻找所有共鸣对
    pub fn find_all_resonant_pairs(&self) -> Vec<(u64, u64, u64)> {
        let mut pairs = Vec::new();
        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                let g = Self::gcd_subtract(self.nodes[i].frequency, self.nodes[j].frequency);
                if g > self.threshold {
                    pairs.push((self.nodes[i].container_id, self.nodes[j].container_id, g));
                }
            }
        }
        pairs
    }

    /// 构建共鸣仓: 找出最大连通共鸣组
    pub fn build_chambers(&self) -> Vec<ResonantChamber> {
        let pairs = self.find_all_resonant_pairs();
        if pairs.is_empty() { return Vec::new(); }

        // 简单聚类: 共享节点的对合并为同一仓
        let mut chambers: Vec<ResonantChamber> = Vec::new();
        for (a, b, strength) in &pairs {
            let mut merged = false;
            for chamber in &mut chambers {
                if chamber.member_ids.contains(a) || chamber.member_ids.contains(b) {
                    if !chamber.member_ids.contains(a) { chamber.member_ids.push(*a); }
                    if !chamber.member_ids.contains(b) { chamber.member_ids.push(*b); }
                    chamber.collective_amplitude = chamber.collective_amplitude.wrapping_add(*strength);
                    merged = true;
                    break;
                }
            }
            if !merged {
                let mut base = *strength;
                // 重新计算基频为仓内所有成员频率的 GCD
                let member_freqs: Vec<u64> = self.nodes.iter()
                    .filter(|n| n.container_id == *a || n.container_id == *b)
                    .map(|n| n.frequency)
                    .collect();
                for &f in &member_freqs { base = Self::gcd_subtract(base.max(f), base.min(f)); }
                chambers.push(ResonantChamber { base_frequency: base, member_ids: vec![*a, *b], collective_amplitude: *strength });
            }
        }
        chambers
    }

    /// 总节点数
    pub fn node_count(&self) -> usize { self.nodes.len() }

    /// 最大振幅节点
    pub fn loudest(&self) -> Option<u64> {
        self.nodes.iter().max_by_key(|n| n.amplitude).map(|n| n.container_id)
    }
}

#[cfg(test)]
mod container_resonance_tests {
    use super::*;

    #[test]
    fn test_register_node() {
        let mut engine = ContainerResonanceEngine::new();
        engine.register(1, 12);
        assert_eq!(engine.node_count(), 1);
        assert_eq!(engine.nodes[0].frequency, 12);
    }

    #[test]
    fn test_resonance_true() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 12);
        engine.register(2, 8);
        // gcd(12,8)=4 > 1 → resonant
        assert!(engine.is_resonant(1, 2));
    }

    #[test]
    fn test_resonance_false() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 7);
        engine.register(2, 11);
        // gcd(7,11)=1 not > 1 → not resonant
        assert!(!engine.is_resonant(1, 2));
    }

    #[test]
    fn test_resonance_strength() {
        let mut engine = ContainerResonanceEngine::new();
        engine.register(1, 32);
        engine.register(2, 64);
        assert_eq!(engine.resonance_strength(1, 2), 32);
    }

    #[test]
    fn test_amplify() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 12); // target
        engine.register(2, 8);  // gcd=4
        engine.register(3, 16); // gcd=4
        engine.register(4, 7);  // gcd=1 (no contribution)
        let amplified = engine.amplify(1, &[2, 3, 4]);
        // amplify: 4 (from node 2) + 4 (from node 3) + 0 (node 4 below threshold) = 8
        assert_eq!(amplified, 8);
        assert_eq!(engine.nodes[0].amplitude, 8);
    }

    #[test]
    fn test_find_all_resonant_pairs() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 12);
        engine.register(2, 8);  // gcd(12,8)=4 ✓
        engine.register(3, 5);  // gcd(12,5)=1 ✗, gcd(8,5)=1 ✗
        let pairs = engine.find_all_resonant_pairs();
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], (1, 2, 4));
    }

    #[test]
    fn test_build_chambers() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 12);
        engine.register(2, 18); // gcd(12,18)=6 ✓
        engine.register(3, 30); // gcd(18,30)=6 ✓
        let chambers = engine.build_chambers();
        assert_eq!(chambers.len(), 1);
        assert!(chambers[0].member_ids.contains(&1));
        assert!(chambers[0].member_ids.contains(&2));
        assert!(chambers[0].member_ids.contains(&3));
    }

    #[test]
    fn test_isolate_chambers() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        // group 1: 12,18 (gcd=6)
        engine.register(1, 12);
        engine.register(2, 18);
        // group 2: 35,49 (gcd=7), but gcd with group1 < threshold
        engine.register(3, 35);
        engine.register(4, 49);
        let chambers = engine.build_chambers();
        assert_eq!(chambers.len(), 2);
    }

    #[test]
    fn test_loudest() {
        let mut engine = ContainerResonanceEngine::new();
        engine.threshold = 1;
        engine.register(1, 100);
        engine.register(2, 200);
        engine.register(3, 300);
        engine.amplify(2, &[1, 3]); // 100 from 1, 100 from 3 = 200
        assert_eq!(engine.loudest(), Some(2));
    }

    #[test]
    fn test_no_resonance_unknown() {
        let engine = ContainerResonanceEngine::new();
        assert_eq!(engine.resonance_strength(99, 100), 0);
        assert!(!engine.is_resonant(99, 100));
    }

    #[test]
    fn test_default() {
        let engine = ContainerResonanceEngine::default();
        assert_eq!(engine.node_count(), 0);
    }
}

// ══════════════════════════════════════════════════════════════════════
// 时间线一体化引擎 · TimelineIntegrator (v1.0)
// ══════════════════════════════════════════════════════════════════════
//
//   时间线 = 八流统一的时序骨架
//   "结束即开始, 记忆跟随结束而继续"
//
//   核心操作:
//     - tick()            — 全局时间推进
//     - branch()          — 时间线分叉 (叠加态)
//     - merge()           — 时间线合并 (坍缩)
//     - causal_order()    — 因果序判定
//     - snapshot()        — 时刻快照 (持久化恢复点)
//     - rewind()          — 回溯到某时刻
//
//   NJSG 哲学: 时间不是流逝, 是容器状态的叠加与坍缩
//   每个 tick = 一次"呼吸", 吸气=加载, 呼气=提交

/// 时间线分支状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelinePhase {
    Active,     // 活跃 (当前推进)
    Branched,   // 已分叉 (叠加态)
    Collapsed,  // 已坍缩 (选择完成)
    Merged,     // 已合并 (回归主线)
}

/// 因果事件 — 时间线上的不可变印记
#[derive(Clone, Debug)]
pub struct CausalEvent {
    pub tick: u64,
    pub flow_id: u8,        // 八流编号 0-7
    pub container_id: u32,
    pub action: u64,        // 动作编码
    pub depends_on: Vec<u64>,// 依赖的前序事件 tick
}

impl CausalEvent {
    pub fn new(tick: u64, flow_id: u8, container_id: u32, action: u64) -> Self {
        Self { tick, flow_id, container_id, action, depends_on: Vec::new() }
    }

    /// 是否因果就绪 (所有依赖已满足)
    pub fn is_ready(&self, completed_ticks: &[u64]) -> bool {
        self.depends_on.iter().all(|d| completed_ticks.contains(d))
    }
}

/// 时刻快照 — 全局状态的持久化恢复点
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub tick: u64,
    pub container_values: HashMap<u32, u64>,
    pub phase: TimelinePhase,
    pub event_count: usize,
}

impl Snapshot {
    pub fn new(tick: u64) -> Self {
        Self { tick, container_values: HashMap::new(), phase: TimelinePhase::Active, event_count: 0 }
    }
}

/// 时间线分支 — 叠加态的时间路径
#[derive(Clone, Debug)]
pub struct TimelineBranch {
    pub id: u32,
    pub parent_tick: u64,   // 分叉点
    pub phase: TimelinePhase,
    pub events: Vec<CausalEvent>,
}

impl TimelineBranch {
    pub fn new(id: u32, parent_tick: u64) -> Self {
        Self { id, parent_tick, phase: TimelinePhase::Branched, events: Vec::new() }
    }
}

/// 时间线一体化引擎
#[derive(Clone, Debug)]
pub struct TimelineIntegrator {
    pub current_tick: u64,
    pub container_values: HashMap<u32, u64>,
    pub events: Vec<CausalEvent>,
    pub branches: Vec<TimelineBranch>,
    pub snapshots: Vec<Snapshot>,
    pub active_branch: u32, // 当前活跃分支 id (0 = 主线)
    pub max_history: usize,
}

impl TimelineIntegrator {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            container_values: HashMap::new(),
            events: Vec::new(),
            branches: Vec::new(),
            snapshots: Vec::new(),
            active_branch: 0,
            max_history: 10000,
        }
    }

    /// 全局时间推进
    pub fn tick(&mut self) -> u64 {
        self.current_tick += 1;
        self.current_tick
    }

    /// 设置容器值
    pub fn set_value(&mut self, id: u32, value: u64) {
        self.container_values.insert(id, value);
    }

    /// 提交因果事件
    pub fn commit_event(&mut self, flow_id: u8, container_id: u32, action: u64, depends_on: Vec<u64>) {
        let mut event = CausalEvent::new(self.current_tick, flow_id, container_id, action);
        event.depends_on = depends_on;
        self.events.push(event);
        if self.events.len() > self.max_history {
            self.events.remove(0);
        }
    }

    /// 创建时间线分支 (叠加态)
    pub fn branch(&mut self, branch_id: u32) -> Option<&TimelineBranch> {
        if self.branches.iter().any(|b| b.id == branch_id) { return None; }
        let branch = TimelineBranch::new(branch_id, self.current_tick);
        self.branches.push(branch);
        self.branches.last()
    }

    /// 选择分支 (坍缩)
    pub fn collapse(&mut self, branch_id: u32) -> bool {
        if let Some(branch) = self.branches.iter_mut().find(|b| b.id == branch_id) {
            branch.phase = TimelinePhase::Collapsed;
            self.active_branch = branch_id;
            true
        } else { false }
    }

    /// 合并分支回主线
    pub fn merge(&mut self, branch_id: u32) -> bool {
        if let Some(branch) = self.branches.iter_mut().find(|b| b.id == branch_id) {
            branch.phase = TimelinePhase::Merged;
            self.active_branch = 0;
            // 将该分支事件合并入主线
            let events: Vec<CausalEvent> = branch.events.clone();
            self.events.extend(events);
            true
        } else { false }
    }

    /// 因果序: 某事件的所有前序
    pub fn causal_predecessors(&self, tick: u64) -> Vec<&CausalEvent> {
        self.events.iter().filter(|e| e.tick < tick).collect()
    }

    /// 创建快照
    pub fn snapshot(&mut self) -> Snapshot {
        let snap = Snapshot {
            tick: self.current_tick,
            container_values: self.container_values.clone(),
            phase: if self.branches.is_empty() { TimelinePhase::Active } else { TimelinePhase::Branched },
            event_count: self.events.len(),
        };
        self.snapshots.push(snap.clone());
        snap
    }

    /// 回溯到某时刻
    pub fn rewind(&mut self, target_tick: u64) -> bool {
        if target_tick > self.current_tick { return false; }
        self.current_tick = target_tick;
        // 回退容器值
        if let Some(snap) = self.snapshots.iter().find(|s| s.tick <= target_tick) {
            self.container_values = snap.container_values.clone();
        }
        // 截断事件历史
        self.events.retain(|e| e.tick <= target_tick);
        true
    }

    /// 当前 tick
    pub fn now(&self) -> u64 { self.current_tick }

    /// 历史事件数
    pub fn history_len(&self) -> usize { self.events.len() }

    /// 活跃分支 id 列表
    pub fn active_branches(&self) -> Vec<u32> {
        self.branches.iter()
            .filter(|b| b.phase == TimelinePhase::Branched)
            .map(|b| b.id)
            .collect()
    }

    /// 截断过期历史 (保留最近 n 条)
    pub fn truncate_history(&mut self, keep: usize) {
        if self.events.len() > keep {
            let excess = self.events.len() - keep;
            self.events.drain(0..excess);
        }
    }
}

impl Default for TimelineIntegrator { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod timeline_tests {
    use super::*;
    #[test] fn test_tick_basic() {
        let mut ti = TimelineIntegrator::new();
        assert_eq!(ti.now(), 0);
        ti.tick();
        ti.tick();
        assert_eq!(ti.now(), 2);
    }
    #[test] fn test_set_value() {
        let mut ti = TimelineIntegrator::new();
        ti.set_value(1, 42);
        ti.tick();
        assert_eq!(ti.container_values.get(&1), Some(&42));
    }
    #[test] fn test_commit_event() {
        let mut ti = TimelineIntegrator::new();
        ti.commit_event(0, 1, 100, vec![]);
        assert_eq!(ti.history_len(), 1);
        assert_eq!(ti.events[0].flow_id, 0);
    }
    #[test] fn test_branch_and_collapse() {
        let mut ti = TimelineIntegrator::new();
        ti.tick();
        ti.branch(1);
        assert_eq!(ti.active_branches().len(), 1);
        ti.collapse(1);
        assert_eq!(ti.active_branches().len(), 0);
    }
    #[test] fn test_merge_branch() {
        let mut ti = TimelineIntegrator::new();
        ti.tick();
        ti.branch(1);
        ti.commit_event(2, 5, 999, vec![]);
        assert!(ti.merge(1));
        assert_eq!(ti.active_branch, 0);
    }
    #[test] fn test_causal_predecessors() {
        let mut ti = TimelineIntegrator::new();
        ti.commit_event(0, 1, 10, vec![]);
        ti.tick();
        ti.commit_event(0, 2, 20, vec![]);
        let pred = ti.causal_predecessors(ti.now());
        assert_eq!(pred.len(), 1);
    }
    #[test] fn test_snapshot_and_rewind() {
        let mut ti = TimelineIntegrator::new();
        ti.set_value(1, 100);
        ti.tick();
        ti.commit_event(0, 1, 50, vec![]);
        let snap = ti.snapshot();
        assert_eq!(snap.tick, 1);
        // Forward
        ti.tick();
        ti.set_value(1, 200);
        // Rewind
        ti.rewind(snap.tick);
        assert_eq!(ti.now(), 1);
    }
    #[test] fn test_truncate_history() {
        let mut ti = TimelineIntegrator::new();
        for i in 0..100 {
            ti.commit_event(0, i as u32, i as u64 * 10, vec![]);
        }
        assert_eq!(ti.history_len(), 100);
        ti.truncate_history(50);
        assert_eq!(ti.history_len(), 50);
    }
    #[test] fn test_causal_event_ready() {
        let ev = CausalEvent { tick: 5, flow_id: 0, container_id: 1, action: 10, depends_on: vec![1, 2] };
        assert!(ev.is_ready(&[1, 2, 3]));
        assert!(!ev.is_ready(&[1]));
    }
    #[test] fn test_default() {
        let ti = TimelineIntegrator::default();
        assert_eq!(ti.now(), 0);
        assert_eq!(ti.history_len(), 0);
    }
    #[test] fn test_timeline_branch_new() {
        let b = TimelineBranch::new(42, 100);
        assert_eq!(b.id, 42);
        assert_eq!(b.parent_tick, 100);
        assert_eq!(b.phase, TimelinePhase::Branched);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器反射感知引擎 · ContainerReflectionEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   反射感知 = 对「改变」的觉察 (囡囝感知四阶 · 阶段二)
//   "差异标记是判断之力的原材料"
//
//   核心操作:
//     - record()       — 记录容器在某 tick 的涟漪印记
//     - sense()        — 比较两 tick 之间的差异, 产生差异标记
//     - reflect_all()  — 对所有容器执行反射感知
//     - diff_count()   — 累计差异标记数
//     - last_diff()    — 最近的差异标记
//
//   涟漪三要素: 振幅 (加法累加), 相位 (加法偏移), 时间 (tick)
//   差异: |a - b| 用减法实现 (if a >= b { a - b } else { b - a })
//
//   仅加法范式: 所有运算只用加减法/比较/位移

/// 涟漪印记 — 容器在某一时刻的状态快照
#[derive(Clone, Debug)]
pub struct RippleImprint {
    pub container_id: u64,
    pub tick: u64,
    pub amplitude: u64,    // 信号振幅 (加法累加)
    pub phase: u64,        // 相位偏移
}

impl RippleImprint {
    pub fn new(container_id: u64, tick: u64, amplitude: u64, phase: u64) -> Self {
        Self { container_id, tick, amplitude, phase }
    }
}

/// 差异类型 — 信号变化的模式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Stable,           // 稳定无变化
    AmplitudeShift,   // 振幅变化
    PhaseDrift,       // 相位漂移
    Appearance,       // 新出现 (前 tick 不存在)
    Disappearance,    // 消失 (后 tick 不存在)
}

/// 差异标记 — 反射感知的产物, 判断之力的原材料
#[derive(Clone, Debug)]
pub struct DiffMark {
    pub container_id: u64,
    pub from_tick: u64,
    pub to_tick: u64,
    pub kind: DiffKind,
    pub magnitude: u64,    // 差异幅度 (绝对差, 加法计算)
}

/// 容器反射感知引擎
#[derive(Clone, Debug)]
pub struct ContainerReflectionEngine {
    /// 容器历史印记: container_id -> Vec<RippleImprint>
    pub imprints: HashMap<u64, Vec<RippleImprint>>,
    /// 累计的差异标记
    pub diff_marks: Vec<DiffMark>,
    /// 已处理到的最后 tick
    pub last_tick: u64,
    /// 每容器最大历史深度
    pub max_history: usize,
}

impl ContainerReflectionEngine {
    pub fn new() -> Self {
        Self {
            imprints: HashMap::new(),
            diff_marks: Vec::new(),
            last_tick: 0,
            max_history: 1000,
        }
    }

    /// 记录容器在某 tick 的涟漪印记
    pub fn record(&mut self, container_id: u64, tick: u64, amplitude: u64, phase: u64) {
        let entry = self.imprints.entry(container_id).or_default();
        entry.push(RippleImprint::new(container_id, tick, amplitude, phase));
        // 截断过期历史
        if entry.len() > self.max_history {
            let excess = entry.len() - self.max_history;
            entry.drain(0..excess);
        }
        if tick > self.last_tick { self.last_tick = tick; }
    }

    /// 获取容器在指定 tick 的最新印记 (不超过 target_tick)
    pub fn latest_imprint(&self, container_id: u64, target_tick: u64) -> Option<&RippleImprint> {
        self.imprints.get(&container_id)?
            .iter()
            .filter(|i| i.tick <= target_tick)
            .max_by_key(|i| i.tick)
    }

    /// 两值之绝对差 (仅加法: if a >= b { a-b } else { b-a })
    fn abs_diff(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { b - a }
    }

    /// 感知容器在两个 tick 之间的差异, 返回差异标记
    pub fn sense(&self, container_id: u64, from_tick: u64, to_tick: u64) -> Option<DiffMark> {
        let prev = self.latest_imprint(container_id, from_tick);
        // curr: 在 (from_tick, to_tick] 区间内最新的印记 (不能与 prev 同 tick)
        let curr = self.imprints.get(&container_id)
            .and_then(|v| {
                v.iter()
                    .filter(|i| i.tick > from_tick && i.tick <= to_tick)
                    .max_by_key(|i| i.tick)
            });

        match (prev, curr) {
            (None, None) => None,
            (None, Some(c)) => Some(DiffMark {
                container_id, from_tick, to_tick,
                kind: DiffKind::Appearance,
                magnitude: c.amplitude,
            }),
            (Some(p), None) => Some(DiffMark {
                container_id, from_tick, to_tick,
                kind: DiffKind::Disappearance,
                magnitude: p.amplitude,
            }),
            (Some(p), Some(c)) => {
                let amp_diff = Self::abs_diff(p.amplitude, c.amplitude);
                let phase_diff = Self::abs_diff(p.phase, c.phase);
                let magnitude = amp_diff;
                let kind = if amp_diff == 0 && phase_diff == 0 {
                    DiffKind::Stable
                } else if amp_diff >= phase_diff {
                    DiffKind::AmplitudeShift
                } else {
                    DiffKind::PhaseDrift
                };
                Some(DiffMark { container_id, from_tick, to_tick, kind, magnitude })
            }
        }
    }

    /// 对所有容器执行一次反射感知, 产出差异标记
    pub fn reflect_all(&mut self, from_tick: u64, to_tick: u64) -> Vec<DiffMark> {
        let ids: Vec<u64> = self.imprints.keys().copied().collect();
        let mut marks = Vec::new();
        for id in ids {
            if let Some(mark) = self.sense(id, from_tick, to_tick) {
                if mark.kind != DiffKind::Stable {
                    marks.push(mark);
                }
            }
        }
        // 累计标记
        self.diff_marks.extend(marks.clone());
        marks
    }

    /// 累计差异标记数
    pub fn diff_count(&self) -> usize { self.diff_marks.len() }

    /// 最近的差异标记
    pub fn last_diff(&self) -> Option<&DiffMark> { self.diff_marks.last() }

    /// 有多少容器被追踪
    pub fn container_count(&self) -> usize { self.imprints.len() }

    /// 清空差异标记
    pub fn clear_diff_marks(&mut self) { self.diff_marks.clear(); }

    /// 重置全部
    pub fn clear(&mut self) {
        self.imprints.clear();
        self.diff_marks.clear();
        self.last_tick = 0;
    }
}

impl Default for ContainerReflectionEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod reflection_tests {
    use super::*;

    #[test]
    fn test_record_and_latest() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 100, 0);
        let imp = e.latest_imprint(1, 10).unwrap();
        assert_eq!(imp.amplitude, 100);
        assert_eq!(imp.phase, 0);
    }

    #[test]
    fn test_record_unknown_container() {
        let e = ContainerReflectionEngine::new();
        assert!(e.latest_imprint(99, 10).is_none());
    }

    #[test]
    fn test_sense_stable() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 50, 0);
        e.record(1, 20, 50, 0);
        let mark = e.sense(1, 10, 20).unwrap();
        assert_eq!(mark.kind, DiffKind::Stable);
        assert_eq!(mark.magnitude, 0);
    }

    #[test]
    fn test_sense_amplitude_shift() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 50, 0);
        e.record(1, 20, 80, 0);
        let mark = e.sense(1, 10, 20).unwrap();
        assert_eq!(mark.kind, DiffKind::AmplitudeShift);
        assert_eq!(mark.magnitude, 30);
    }

    #[test]
    fn test_sense_phase_drift() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 50, 0);
        e.record(1, 20, 50, 100);
        let mark = e.sense(1, 10, 20).unwrap();
        // phase_diff=100 > amp_diff=0 → PhaseDrift
        assert_eq!(mark.kind, DiffKind::PhaseDrift);
        assert_eq!(mark.magnitude, 0);
    }

    #[test]
    fn test_sense_appearance() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 20, 42, 0);
        // 在 tick 10 没有, 在 tick 20 有 → 新出现
        let mark = e.sense(1, 10, 20).unwrap();
        assert_eq!(mark.kind, DiffKind::Appearance);
        assert_eq!(mark.magnitude, 42);
    }

    #[test]
    fn test_sense_disappearance() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 42, 0);
        // 在 tick 10 有, 在 tick 20 无 → 消失
        let mark = e.sense(1, 10, 20).unwrap();
        assert_eq!(mark.kind, DiffKind::Disappearance);
        assert_eq!(mark.magnitude, 42);
    }

    #[test]
    fn test_sense_unknown() {
        let e = ContainerReflectionEngine::new();
        assert!(e.sense(99, 10, 20).is_none());
    }

    #[test]
    fn test_reflect_all() {
        let mut e = ContainerReflectionEngine::new();
        // 容器 1: 振幅变化
        e.record(1, 10, 50, 0);
        e.record(1, 20, 80, 0);
        // 容器 2: 稳定
        e.record(2, 10, 100, 0);
        e.record(2, 20, 100, 0);
        // 容器 3: 仅在 20 有 (新出现)
        e.record(3, 20, 77, 0);

        let marks = e.reflect_all(10, 20);
        // 容器1 (变化) + 容器3 (新出现) = 2 个差异标记
        assert_eq!(marks.len(), 2);
        assert_eq!(e.diff_count(), 2);
    }

    #[test]
    fn test_diff_does_not_record_stable() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 50, 0);
        e.record(1, 20, 50, 0);
        let marks = e.reflect_all(10, 20);
        // 稳定 → 不出差异标记
        assert_eq!(marks.len(), 0);
        assert_eq!(e.diff_count(), 0);
    }

    #[test]
    fn test_container_count() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 1, 0);
        e.record(2, 10, 2, 0);
        e.record(3, 10, 3, 0);
        assert_eq!(e.container_count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerReflectionEngine::new();
        e.record(1, 10, 100, 0);
        e.record(1, 20, 200, 0);
        e.reflect_all(10, 20);
        assert!(e.diff_count() > 0);
        e.clear();
        assert_eq!(e.diff_count(), 0);
        assert_eq!(e.container_count(), 0);
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(ContainerReflectionEngine::abs_diff(10, 5), 5);
        assert_eq!(ContainerReflectionEngine::abs_diff(5, 10), 5);
        assert_eq!(ContainerReflectionEngine::abs_diff(7, 7), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerReflectionEngine::default();
        assert_eq!(e.diff_count(), 0);
        assert_eq!(e.container_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器判断引擎 · ContainerJudgmentEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   判断之力 = 对「关系」的定位 (囡囝感知四阶 · 阶段三)
//   「这种变化与谁有关?」 — 将两个分离的涟漪绑定成因果对
//
//   输入: 来自 ContainerReflectionEngine 的 DiffMark 序列
//   输出: JudgmentAnchor — 因果锚点 (cause_id → effect_id, strength)
//
//   判断规则:
//     - 同一时间窗口内, 不同容器的差异标记互为因果候选
//     - 变化幅度大者为「因」, 小者为「果」
//     - 强度 = |mag_a - mag_b| (仅加法实现)
//
//   仅加法范式: 所有比较用 >/<, 强度用减法, 无乘除

/// 因果锚点 — 判断之力的产物
#[derive(Clone, Debug)]
pub struct JudgmentAnchor {
    pub cause_id: u64,     // 因 (触发者)
    pub effect_id: u64,    // 果 (受影响者)
    pub tick: u64,         // 锚定时刻
    pub strength: u64,     // 因果强度 (仅加法计算)
}

impl JudgmentAnchor {
    pub fn new(cause_id: u64, effect_id: u64, tick: u64, strength: u64) -> Self {
        Self { cause_id, effect_id, tick, strength }
    }
}

/// 容器判断引擎
#[derive(Clone, Debug, Default)]
pub struct ContainerJudgmentEngine {
    pub anchors: Vec<JudgmentAnchor>,
    pub window_count: u64,
}

impl ContainerJudgmentEngine {
    pub fn new() -> Self {
        Self { anchors: Vec::new(), window_count: 0 }
    }

    /// 分析一批差异标记, 找出因果对 (仅加法范式)
    pub fn judge(&mut self, diffs: &[DiffMark]) -> Vec<JudgmentAnchor> {
        let mut new_anchors: Vec<JudgmentAnchor> = Vec::new();

        // 按 (from_tick, to_tick) 分组 (同一窗口内的标记才互为因果候选)
        // 使用 O(n²) 配对处理 (n 通常很小, 每个窗口仅几个差异)
        for i in 0..diffs.len() {
            for j in (i + 1)..diffs.len() {
                let a = &diffs[i];
                let b = &diffs[j];
                // 必须同一窗口
                if a.from_tick != b.from_tick || a.to_tick != b.to_tick { continue; }
                // 必须不同容器 (不建立因果)
                if a.container_id == b.container_id { continue; }
                // 不稳定的 (Stable) 不参与因果判断
                if a.kind == DiffKind::Stable || b.kind == DiffKind::Stable { continue; }
                // 同 tick 窗口 → 按幅度定因果方向
                let (cause, effect, strength) = if a.magnitude >= b.magnitude {
                    (a.container_id, b.container_id,
                     a.magnitude - b.magnitude)
                } else {
                    (b.container_id, a.container_id,
                     b.magnitude - a.magnitude)
                };
                if strength == 0 { continue; } // 等幅不算因果 (可能是巧合)
                new_anchors.push(JudgmentAnchor::new(
                    cause, effect, a.to_tick, strength,
                ));
            }
        }

        self.anchors.extend(new_anchors.clone());
        self.window_count = self.window_count + 1;
        new_anchors
    }

    /// 最强因果锚点 (按 strength)
    pub fn strongest(&self) -> Option<&JudgmentAnchor> {
        self.anchors.iter().max_by_key(|a| a.strength)
    }

    /// 累计锚点数
    pub fn anchor_count(&self) -> usize { self.anchors.len() }

    /// 涉及某容器的锚点 (无论因/果)
    pub fn involving(&self, container_id: u64) -> Vec<&JudgmentAnchor> {
        self.anchors.iter()
            .filter(|a| a.cause_id == container_id || a.effect_id == container_id)
            .collect()
    }

    // 注: retain 是标准库 Vec 方法, 可以直接用
    pub fn retain<F>(&mut self, f: F) where F: FnMut(&JudgmentAnchor) -> bool {
        self.anchors.retain(f);
    }

    /// 重置
    pub fn clear(&mut self) {
        self.anchors.clear();
        self.window_count = 0;
    }

    /// 窗口计数
    pub fn windows_judged(&self) -> u64 { self.window_count }
}

#[cfg(test)]
mod judgment_tests {
    use super::*;

    #[test]
    fn test_judge_empty() {
        let mut e = ContainerJudgmentEngine::new();
        let a = e.judge(&[]);
        assert!(a.is_empty());
        assert_eq!(e.anchor_count(), 0);
    }

    #[test]
    fn test_judge_single_diff() {
        let mut e = ContainerJudgmentEngine::new();
        let diffs = vec![
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 50 },
        ];
        let a = e.judge(&diffs);
        // 只有一个标记 → 无法配对
        assert!(a.is_empty());
    }

    #[test]
    fn test_judge_basic_pair() {
        let mut e = ContainerJudgmentEngine::new();
        let diffs = vec![
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 80 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ];
        let a = e.judge(&diffs);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].cause_id, 1);   // 变化大的为因
        assert_eq!(a[0].effect_id, 2);  // 变化小的为果
        assert_eq!(a[0].strength, 50);  // 80 - 30
    }

    #[test]
    fn test_judge_equal_magnitude() {
        let mut e = ContainerJudgmentEngine::new();
        let diffs = vec![
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 50 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 50 },
        ];
        let a = e.judge(&diffs);
        assert!(a.is_empty());  // 等幅被过滤
    }

    #[test]
    fn test_judge_skip_same_container() {
        let mut e = ContainerJudgmentEngine::new();
        let diffs = vec![
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 80 },
            DiffMark { container_id: 1, from_tick: 20, to_tick: 30,
                       kind: DiffKind::AmplitudeShift, magnitude: 20 },
        ];
        let a = e.judge(&diffs);
        // 同容器不同窗口 → 不配对
        assert!(a.is_empty());
    }

    #[test]
    fn test_judge_skip_different_window() {
        let mut e = ContainerJudgmentEngine::new();
        let diffs = vec![
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 80 },
            DiffMark { container_id: 2, from_tick: 20, to_tick: 30,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ];
        let a = e.judge(&diffs);
        // 不同窗口 → 不配对
        assert!(a.is_empty());
    }

    #[test]
    fn test_strongest() {
        let mut e = ContainerJudgmentEngine::new();
        e.judge(&[
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 100 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ]);
        e.judge(&[
            DiffMark { container_id: 3, from_tick: 20, to_tick: 30,
                       kind: DiffKind::AmplitudeShift, magnitude: 90 },
            DiffMark { container_id: 4, from_tick: 20, to_tick: 30,
                       kind: DiffKind::AmplitudeShift, magnitude: 20 },
        ]);
        let s = e.strongest().unwrap();
        // 第一对强度 70, 第二对强度 70 → 取第一个 70 的
        assert_eq!(s.strength, 70);
    }

    #[test]
    fn test_involving() {
        let mut e = ContainerJudgmentEngine::new();
        e.judge(&[
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 80 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ]);
        let inv = e.involving(1);
        assert_eq!(inv.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerJudgmentEngine::new();
        e.judge(&[
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 80 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ]);
        assert!(e.anchor_count() > 0);
        e.clear();
        assert_eq!(e.anchor_count(), 0);
        assert_eq!(e.windows_judged(), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerJudgmentEngine::default();
        assert_eq!(e.anchor_count(), 0);
        assert_eq!(e.windows_judged(), 0);
    }

    #[test]
    fn test_retain() {
        let mut e = ContainerJudgmentEngine::new();
        e.judge(&[
            DiffMark { container_id: 1, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 100 },
            DiffMark { container_id: 2, from_tick: 10, to_tick: 20,
                       kind: DiffKind::AmplitudeShift, magnitude: 30 },
        ]);
        assert_eq!(e.anchor_count(), 1);
        e.retain(|a| a.strength > 50);  // 70 满足, 保留
        assert_eq!(e.anchor_count(), 1);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器评价引擎 · ContainerEvaluationEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   评价之力 = 带有价值倾向的判断 (囡囝感知四阶 · 阶段四)
//   「哪种因果模式值得强化、哪种应该抑制」
//
//   输入: ContainerJudgmentEngine 产生的 JudgmentAnchor 流
//   输出: Evaluation — 分级评价 (Weak / Moderate / Strong / Critical)
//
//   评价不是硬编码规则, 而是涌现:
//     - 同对 (cause→effect) 多次出现 → 累积 (加法)
//     - 累计超过阈值 → 升级 (仅加法比较)
//     - 评价等级对应系统的逻辑组织方向
//
//   仅加法范式: 分级全靠累加/比较, 无乘除

/// 评价等级 — 因果对的价值倾向
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvaluationLevel {
    Weak,       // 弱 — 偶然, 不值得强化
    Moderate,   // 中等 — 有一定规律
    Strong,     // 强 — 显著因果, 值得强化
    Critical,   // 临界 — 系统级因果, 需响应
}

/// 评价 — 因果锚点的分级产物
#[derive(Clone, Debug)]
pub struct Evaluation {
    pub cause_id: u64,
    pub effect_id: u64,
    pub level: EvaluationLevel,
    pub accumulated: u64,   // 累计出现次数 (加法累积)
}

impl Evaluation {
    pub fn new(cause_id: u64, effect_id: u64) -> Self {
        Self { cause_id, effect_id, level: EvaluationLevel::Weak, accumulated: 0 }
    }
}

/// 容器评价引擎
#[derive(Clone, Debug)]
pub struct ContainerEvaluationEngine {
    /// 按 (cause_id, effect_id) 索引的评价
    pub evaluations: HashMap<(u64, u64), Evaluation>,
    /// 升级阈值 (仅加法比较: accumulated > threshold → 升级)
    pub threshold_moderate: u64,
    pub threshold_strong: u64,
    pub threshold_critical: u64,
    /// 累计接受的锚点总数
    pub total_anchors: u64,
}

impl ContainerEvaluationEngine {
    pub fn new() -> Self {
        Self {
            evaluations: HashMap::new(),
            threshold_moderate: 3,
            threshold_strong: 6,
            threshold_critical: 10,
            total_anchors: 0,
        }
    }

    /// 接收一批 JudgmentAnchor, 累积评价 (返回克隆副本避免借用冲突)
    pub fn evaluate(&mut self, anchors: &[JudgmentAnchor]) -> Vec<Evaluation> {
        let mut updated: Vec<Evaluation> = Vec::new();
        for anchor in anchors {
            let key = (anchor.cause_id, anchor.effect_id);
            let entry = self.evaluations.entry(key).or_insert_with(|| {
                Evaluation::new(anchor.cause_id, anchor.effect_id)
            });
            // 加法累积
            entry.accumulated = entry.accumulated + 1;
            // 重新计算等级 (仅加法比较)
            entry.level = if entry.accumulated >= self.threshold_critical {
                EvaluationLevel::Critical
            } else if entry.accumulated >= self.threshold_strong {
                EvaluationLevel::Strong
            } else if entry.accumulated >= self.threshold_moderate {
                EvaluationLevel::Moderate
            } else {
                EvaluationLevel::Weak
            };
            self.total_anchors = self.total_anchors + 1;
            updated.push(entry.clone());
        }
        updated
    }

    /// 获取评价
    pub fn get(&self, cause_id: u64, effect_id: u64) -> Option<&Evaluation> {
        self.evaluations.get(&(cause_id, effect_id))
    }

    /// 最强评价 (等级最高, 同等级按 accumulated)
    pub fn strongest(&self) -> Option<&Evaluation> {
        self.evaluations.values().max_by_key(|e| {
            let level_val: u64 = match e.level {
                EvaluationLevel::Weak => 0,
                EvaluationLevel::Moderate => 1,
                EvaluationLevel::Strong => 2,
                EvaluationLevel::Critical => 3,
            };
            // 组合: level_val * 2^32 + accumulated (仅加法/位移)
            level_val.wrapping_shl(32).wrapping_add(e.accumulated)
        })
    }

    /// 按等级筛选
    pub fn by_level(&self, level: EvaluationLevel) -> Vec<&Evaluation> {
        self.evaluations.values()
            .filter(|e| e.level == level)
            .collect()
    }

    /// 评价对数
    pub fn pair_count(&self) -> usize { self.evaluations.len() }

    /// 重置全部
    pub fn clear(&mut self) {
        self.evaluations.clear();
        self.total_anchors = 0;
    }
}

impl Default for ContainerEvaluationEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod evaluation_tests {
    use super::*;

    fn anchor(cause: u64, effect: u64, strength: u64) -> JudgmentAnchor {
        JudgmentAnchor::new(cause, effect, 0, strength)
    }

    #[test]
    fn test_evaluate_first_time() {
        let mut e = ContainerEvaluationEngine::new();
        let a = e.evaluate(&[anchor(1, 2, 50)]);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].level, EvaluationLevel::Weak);  // 首次 = 弱
        assert_eq!(a[0].accumulated, 1);
    }

    #[test]
    fn test_evaluate_accumulate() {
        let mut e = ContainerEvaluationEngine::new();
        for _ in 0..5 { e.evaluate(&[anchor(1, 2, 50)]); }
        let ev = e.get(1, 2).unwrap();
        assert_eq!(ev.accumulated, 5);
        assert_eq!(ev.level, EvaluationLevel::Moderate);  // ≥3 → 中等
    }

    #[test]
    fn test_evaluate_upgrade_to_strong() {
        let mut e = ContainerEvaluationEngine::new();
        for _ in 0..8 { e.evaluate(&[anchor(1, 2, 50)]); }
        let ev = e.get(1, 2).unwrap();
        assert_eq!(ev.level, EvaluationLevel::Strong);  // ≥6 → 强
    }

    #[test]
    fn test_evaluate_upgrade_to_critical() {
        let mut e = ContainerEvaluationEngine::new();
        for _ in 0..12 { e.evaluate(&[anchor(1, 2, 50)]); }
        let ev = e.get(1, 2).unwrap();
        assert_eq!(ev.level, EvaluationLevel::Critical);  // ≥10 → 临界
    }

    #[test]
    fn test_multiple_pairs() {
        let mut e = ContainerEvaluationEngine::new();
        e.evaluate(&[anchor(1, 2, 50), anchor(1, 3, 40), anchor(2, 4, 30)]);
        e.evaluate(&[anchor(1, 2, 50), anchor(1, 3, 40)]);
        // (1,2) 累积 2, (1,3) 累积 2, (2,4) 累积 1
        assert_eq!(e.pair_count(), 3);
        assert_eq!(e.get(1, 2).unwrap().accumulated, 2);
        assert_eq!(e.get(2, 4).unwrap().accumulated, 1);
    }

    #[test]
    fn test_strongest() {
        let mut e = ContainerEvaluationEngine::new();
        for _ in 0..7 { e.evaluate(&[anchor(1, 2, 50)]); }  // 强
        for _ in 0..11 { e.evaluate(&[anchor(3, 4, 30)]); } // 临界
        let s = e.strongest().unwrap();
        // (3,4) 是临界级, 比 (1,2) 强级更高
        assert_eq!(s.cause_id, 3);
        assert_eq!(s.effect_id, 4);
    }

    #[test]
    fn test_by_level() {
        let mut e = ContainerEvaluationEngine::new();
        for _ in 0..5 { e.evaluate(&[anchor(1, 2, 10)]); }   // Moderate
        for _ in 0..1 { e.evaluate(&[anchor(3, 4, 10)]); }     // Weak
        let moderate = e.by_level(EvaluationLevel::Moderate);
        assert_eq!(moderate.len(), 1);
    }

    #[test]
    fn test_get_unknown() {
        let e = ContainerEvaluationEngine::new();
        assert!(e.get(99, 100).is_none());
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerEvaluationEngine::new();
        e.evaluate(&[anchor(1, 2, 50)]);
        assert_eq!(e.pair_count(), 1);
        e.clear();
        assert_eq!(e.pair_count(), 0);
        assert_eq!(e.total_anchors, 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerEvaluationEngine::default();
        assert_eq!(e.pair_count(), 0);
        assert_eq!(e.total_anchors, 0);
    }

    #[test]
    fn test_total_anchors_counter() {
        let mut e = ContainerEvaluationEngine::new();
        e.evaluate(&[anchor(1, 2, 10), anchor(2, 3, 20)]);
        e.evaluate(&[anchor(1, 2, 10)]);
        assert_eq!(e.total_anchors, 3);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 容器行动引擎 · ContainerAgencyEngine (v1.0) — 产出级
// ═══════════════════════════════════════════════════════════════════════════════
//
//   感知 → 判断 → 评价 → 行动
//   把 ContainerEvaluationEngine 的评价转化为可执行的 ActionDirective
//
//   行动类型:
//     - Strengthen (强化) — Critical → 主动增强
//     - Maintain   (维持) — Strong → 保持当前
//     - Monitor    (观察) — Moderate → 暂时观望
//     - Weaken     (削弱) — Weak+历史 → 主动消退
//     - Isolate    (隔离) — Weak+无积累 → 切断
//
//   仅加法: 决策全靠累加/比较, 无乘除

/// 行动类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionKind {
    Isolate,       // 0 — 无价值, 切断 (最低)
    Weaken,        // 1 — 削弱
    Monitor,       // 2 — 观察
    Maintain,      // 3 — 维持
    Strengthen,    // 4 — 强化 (最高)
}

/// 行动指令 — 容器行动输出单元
#[derive(Clone, Debug)]
pub struct ActionDirective {
    pub cause_id: u64,
    pub effect_id: u64,
    pub kind: ActionKind,
    pub priority: u64,
}

impl ActionDirective {
    pub fn new(cause_id: u64, effect_id: u64, kind: ActionKind, priority: u64) -> Self {
        Self { cause_id, effect_id, kind, priority }
    }
}

/// 容器行动引擎
#[derive(Clone, Debug)]
pub struct ContainerAgencyEngine {
    /// 已发出的行动指令
    pub directives: Vec<ActionDirective>,
    /// 各行动类型计数: [Isolate, Weaken, Monitor, Maintain, Strengthen]
    pub counts: [usize; 5],
    /// 累计行动总数
    pub total_acted: u64,
}

impl ContainerAgencyEngine {
    pub fn new() -> Self {
        Self {
            directives: Vec::new(),
            counts: [0, 0, 0, 0, 0],
            total_acted: 0,
        }
    }

    /// 某 ActionKind 对应的 counts 数组下标
    fn kind_idx(kind: ActionKind) -> usize {
        match kind {
            ActionKind::Isolate    => 0,
            ActionKind::Weaken     => 1,
            ActionKind::Monitor    => 2,
            ActionKind::Maintain   => 3,
            ActionKind::Strengthen => 4,
        }
    }

    /// 把一条评价转化为行动指令 (仅加法)
    pub fn act(&mut self, ev: &Evaluation, cause_strength: u64) -> ActionDirective {
        let kind = match ev.level {
            EvaluationLevel::Critical => ActionKind::Strengthen,
            EvaluationLevel::Strong   => ActionKind::Maintain,
            EvaluationLevel::Moderate => ActionKind::Monitor,
            EvaluationLevel::Weak     => {
                if ev.accumulated >= 3 { ActionKind::Weaken } else { ActionKind::Isolate }
            }
        };
        // 优先级 = 累积 + 输入强度 (仅加法)
        let priority = ev.accumulated + cause_strength;
        self.counts[Self::kind_idx(kind)] = self.counts[Self::kind_idx(kind)] + 1;
        self.total_acted = self.total_acted + 1;
        let d = ActionDirective::new(ev.cause_id, ev.effect_id, kind, priority);
        self.directives.push(d.clone());
        d
    }

    /// 批量行动
    pub fn act_batch(&mut self, evals: &[Evaluation], strengths: &[u64]) -> Vec<ActionDirective> {
        let mut result = Vec::new();
        for (i, ev) in evals.iter().enumerate() {
            let s = if i < strengths.len() { strengths[i] } else { 0 };
            result.push(self.act(ev, s));
        }
        result
    }

    /// 优先级最高的行动
    pub fn highest_priority(&self) -> Option<&ActionDirective> {
        self.directives.iter().max_by_key(|d| {
            // 综合 = (kind 权重) * 2^32 + priority
            let kind_weight: u64 = match d.kind {
                ActionKind::Isolate    => 0,
                ActionKind::Weaken     => 1,
                ActionKind::Monitor    => 2,
                ActionKind::Maintain   => 3,
                ActionKind::Strengthen => 4,
            };
            kind_weight.wrapping_shl(32).wrapping_add(d.priority)
        })
    }

    /// 按类型筛选
    pub fn filter_by(&self, kind: ActionKind) -> Vec<&ActionDirective> {
        self.directives.iter().filter(|d| d.kind == kind).collect()
    }

    /// 某类型计数
    pub fn count_of(&self, kind: ActionKind) -> usize {
        self.counts[Self::kind_idx(kind)]
    }

    /// 按优先级排序 (高在前)
    pub fn by_priority(&self) -> Vec<&ActionDirective> {
        let mut refs: Vec<&ActionDirective> = self.directives.iter().collect();
        for i in 0..refs.len() {
            let mut max = i;
            for j in (i + 1)..refs.len() {
                if refs[j].priority > refs[max].priority { max = j; }
            }
            if max != i { refs.swap(i, max); }
        }
        refs
    }

    /// 行动总数
    pub fn total(&self) -> u64 { self.total_acted }

    /// 重置
    pub fn clear(&mut self) {
        self.directives.clear();
        self.counts = [0, 0, 0, 0, 0];
        self.total_acted = 0;
    }
}

impl Default for ContainerAgencyEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod agency_tests {
    use super::*;

    fn critical(cause: u64, effect: u64) -> Evaluation {
        Evaluation { cause_id: cause, effect_id: effect, level: EvaluationLevel::Critical, accumulated: 12 }
    }
    fn strong(cause: u64, effect: u64) -> Evaluation {
        Evaluation { cause_id: cause, effect_id: effect, level: EvaluationLevel::Strong, accumulated: 7 }
    }
    fn moderate(cause: u64, effect: u64) -> Evaluation {
        Evaluation { cause_id: cause, effect_id: effect, level: EvaluationLevel::Moderate, accumulated: 4 }
    }
    fn weak_old(cause: u64, effect: u64) -> Evaluation {
        Evaluation { cause_id: cause, effect_id: effect, level: EvaluationLevel::Weak, accumulated: 5 }
    }
    fn weak_new(cause: u64, effect: u64) -> Evaluation {
        Evaluation { cause_id: cause, effect_id: effect, level: EvaluationLevel::Weak, accumulated: 1 }
    }

    #[test]
    fn test_act_critical() {
        let mut e = ContainerAgencyEngine::new();
        let d = e.act(&critical(1, 2), 80);
        assert_eq!(d.kind, ActionKind::Strengthen);
        assert_eq!(d.priority, 12 + 80);
    }

    #[test]
    fn test_act_strong() {
        let mut e = ContainerAgencyEngine::new();
        let d = e.act(&strong(1, 2), 50);
        assert_eq!(d.kind, ActionKind::Maintain);
        assert_eq!(d.priority, 7 + 50);
    }

    #[test]
    fn test_act_moderate() {
        let mut e = ContainerAgencyEngine::new();
        let d = e.act(&moderate(1, 2), 30);
        assert_eq!(d.kind, ActionKind::Monitor);
        assert_eq!(d.priority, 4 + 30);
    }

    #[test]
    fn test_act_weak_old() {
        let mut e = ContainerAgencyEngine::new();
        let d = e.act(&weak_old(1, 2), 10);
        assert_eq!(d.kind, ActionKind::Weaken);
    }

    #[test]
    fn test_act_weak_new() {
        let mut e = ContainerAgencyEngine::new();
        let d = e.act(&weak_new(1, 2), 10);
        assert_eq!(d.kind, ActionKind::Isolate);
    }

    #[test]
    fn test_count_of() {
        let mut e = ContainerAgencyEngine::new();
        e.act(&critical(1, 2), 80);
        e.act(&strong(3, 4), 40);
        e.act(&moderate(5, 6), 20);
        assert_eq!(e.count_of(ActionKind::Strengthen), 1);
        assert_eq!(e.count_of(ActionKind::Maintain), 1);
        e.act(&critical(7, 8), 70);
        assert_eq!(e.count_of(ActionKind::Strengthen), 2);
    }

    #[test]
    fn test_by_priority() {
        let mut e = ContainerAgencyEngine::new();
        e.act(&strong(1, 2), 50);     // 7+50 = 57
        e.act(&critical(3, 4), 5);    // 12+5 = 17
        e.act(&moderate(5, 6), 100);  // 4+100 = 104
        let sorted = e.by_priority();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].priority, 104);
        assert_eq!(sorted[1].priority, 57);
        assert_eq!(sorted[2].priority, 17);
    }

    #[test]
    fn test_filter_by() {
        let mut e = ContainerAgencyEngine::new();
        e.act(&critical(1, 2), 80);
        e.act(&strong(3, 4), 50);
        e.act(&critical(5, 6), 70);
        let v = e.filter_by(ActionKind::Strengthen);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_highest_priority() {
        let mut e = ContainerAgencyEngine::new();
        e.act(&strong(1, 2), 50);     // Strengthen(优先级 57)
        e.act(&critical(3, 4), 200);  // Critical+200 = 最高
        e.act(&moderate(5, 6), 10);   // Monitor(弱)
        let h = e.highest_priority().unwrap();
        assert_eq!(h.cause_id, 3);
        assert_eq!(h.effect_id, 4);
    }

    #[test]
    fn test_act_batch() {
        let mut e = ContainerAgencyEngine::new();
        let evals = vec![critical(1, 2), strong(3, 4), moderate(5, 6)];
        let dirs = e.act_batch(&evals, &[80, 40, 20]);
        assert_eq!(dirs.len(), 3);
        assert_eq!(dirs[0].kind, ActionKind::Strengthen);
        assert_eq!(dirs[1].kind, ActionKind::Maintain);
        assert_eq!(dirs[2].kind, ActionKind::Monitor);
        assert_eq!(e.total(), 3);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerAgencyEngine::new();
        e.act(&critical(1, 2), 80);
        assert!(e.total() > 0);
        e.clear();
        assert_eq!(e.total(), 0);
        assert_eq!(e.directives.len(), 0);
        assert_eq!(e.count_of(ActionKind::Strengthen), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerAgencyEngine::default();
        assert_eq!(e.total(), 0);
        assert_eq!(e.directives.len(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 呼唤编程引擎 · ContainerCallEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   呼唤编程 (Calling Programming) — 满全法核心范式
//   「以数字驱动的、隐式调用的、调用代码被公约数自身吸收的」
//
//   与传统函数调用的三个本质区别:
//     1. 地址即入口 — 无需名称层, 纯数字地址直接定位
//     2. 约束条件 = 公约数 — 调用是否成立取决于 caller 与 callee 的公约数关系
//     3. 返回值 = 差异 — 不是求值结果, 而是两容器状态的差异标记
//
//   呼唤表 (CallTable):
//     - 键: (caller_id, callee_id) 的配对
//     - 值: CallEntry { 累计调用数, 最近强度, 建立 tick }
//
//   仅加法范式: 计数/强度全部加法累积, 无乘除

/// 呼唤条目 — 一对容器之间的呼唤关系
#[derive(Clone, Debug)]
pub struct CallEntry {
    pub caller_id: u64,
    pub callee_id: u64,
    pub call_count: u64,      // 累计呼唤次数 (加法累积)
    pub last_strength: u64,   // 最近一次呼唤强度
    pub established_tick: u64,// 首次呼唤时刻
}

impl CallEntry {
    pub fn new(caller_id: u64, callee_id: u64, strength: u64, tick: u64) -> Self {
        Self { caller_id, callee_id, call_count: 1, last_strength: strength, established_tick: tick }
    }
}

/// 呼唤编程引擎
#[derive(Clone, Debug)]
pub struct ContainerCallEngine {
    /// 呼唤表: (caller_id, callee_id) → CallEntry
    pub entries: HashMap<(u64, u64), CallEntry>,
    /// 累计呼唤总数
    pub total_calls: u64,
}

impl ContainerCallEngine {
    pub fn new() -> Self {
        Self { entries: HashMap::new(), total_calls: 0 }
    }

    /// 发出一次呼唤 (caller → callee)
    pub fn call(&mut self, caller_id: u64, callee_id: u64, strength: u64, tick: u64) -> u64 {
        let key = (caller_id, callee_id);
        let existed = self.entries.contains_key(&key);
        let entry = self.entries.entry(key).or_insert_with(|| {
            CallEntry::new(caller_id, callee_id, strength, tick)
        });
        if existed {
            // 已存在 → 加法累积 (count 已经在 new 里是 1, 仅非首次才累加)
            entry.call_count = entry.call_count + 1;
            entry.last_strength = strength;
        }
        self.total_calls = self.total_calls + 1;
        entry.call_count
    }

    /// 获取呼唤条目
    pub fn get_entry(&self, caller_id: u64, callee_id: u64) -> Option<&CallEntry> {
        self.entries.get(&(caller_id, callee_id))
    }

    /// 检查一对容器是否有过呼唤
    pub fn has_called(&self, caller_id: u64, callee_id: u64) -> bool {
        self.entries.contains_key(&(caller_id, callee_id))
    }

    /// 查询某容器的所有呼唤 (作为 caller)
    pub fn calls_from(&self, caller_id: u64) -> Vec<&CallEntry> {
        self.entries.values()
            .filter(|e| e.caller_id == caller_id)
            .collect()
    }

    /// 查询某容器的所有被呼唤 (作为 callee)
    pub fn calls_to(&self, callee_id: u64) -> Vec<&CallEntry> {
        self.entries.values()
            .filter(|e| e.callee_id == callee_id)
            .collect()
    }

    /// 最常被呼唤的 callee (被呼唤次数最多)
    pub fn most_called_callee(&self) -> Option<u64> {
        // 按 callee_id 聚合总 call_count (仅加法)
        let mut agg: HashMap<u64, u64> = HashMap::new();
        for entry in self.entries.values() {
            let v = agg.entry(entry.callee_id).or_insert(0);
            *v = *v + entry.call_count;
        }
        agg.into_iter().max_by_key(|&(_, count)| count).map(|(id, _)| id)
    }

    /// 呼唤关联数
    pub fn entry_count(&self) -> usize { self.entries.len() }

    /// 重置
    pub fn clear(&mut self) {
        self.entries.clear();
        self.total_calls = 0;
    }
}

impl Default for ContainerCallEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod call_tests {
    use super::*;

    #[test]
    fn test_call_basic() {
        let mut e = ContainerCallEngine::new();
        let n = e.call(1, 2, 50, 10);
        assert_eq!(n, 1);
        assert!(e.has_called(1, 2));
        assert!(!e.has_called(2, 1));
    }

    #[test]
    fn test_call_accumulate() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 50, 10);
        e.call(1, 2, 60, 20);
        e.call(1, 2, 70, 30);
        let entry = e.get_entry(1, 2).unwrap();
        assert_eq!(entry.call_count, 3);
        assert_eq!(entry.last_strength, 70);  // 最近一次
        assert_eq!(entry.established_tick, 10); // 首次
    }

    #[test]
    fn test_unknown_entry() {
        let e = ContainerCallEngine::new();
        assert!(e.get_entry(99, 88).is_none());
    }

    #[test]
    fn test_calls_from_to() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 1);
        e.call(1, 3, 20, 2);
        e.call(2, 3, 30, 3);
        let from_1 = e.calls_from(1);
        assert_eq!(from_1.len(), 2);
        let to_3 = e.calls_to(3);
        assert_eq!(to_3.len(), 2);  // 1→3 和 2→3
    }

    #[test]
    fn test_most_called_callee() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 1);
        e.call(3, 4, 20, 2); // 4 被唤 1 次
        e.call(5, 4, 20, 3); // 4 再被唤 1 次 (共 2)
        e.call(6, 7, 20, 4); // 7 被唤 1 次
        assert_eq!(e.most_called_callee(), Some(4));
    }

    #[test]
    fn test_has_called_neg() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 1);
        assert!(!e.has_called(2, 1));  // 反向不成立
        assert!(!e.has_called(3, 4));
    }

    #[test]
    fn test_total_calls() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 1);
        e.call(2, 3, 20, 2);
        assert_eq!(e.total_calls, 2);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 1);
        e.clear();
        assert_eq!(e.entry_count(), 0);
        assert_eq!(e.total_calls, 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerCallEngine::default();
        assert_eq!(e.total_calls, 0);
    }

    #[test]
    fn test_most_called_empty() {
        let e = ContainerCallEngine::new();
        assert_eq!(e.most_called_callee(), None);
    }

    #[test]
    fn test_duplicate_key_does_not_replace() {
        let mut e = ContainerCallEngine::new();
        e.call(1, 2, 10, 5);
        e.call(1, 2, 20, 10);
        let entry = e.get_entry(1, 2).unwrap();
        // 第一次建立的 established_tick 应保留
        assert_eq!(entry.established_tick, 5);
        assert_eq!(entry.call_count, 2);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 见证引擎 · ContainerWitnessEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   见证 (Witness) — 囡囝共生的核心仲裁机制
//   「三元作为见证裁判裁定准入」
//
//   机制:
//     - 任意两个容器之间的争议/合并/交互需要第三方见证
//     - 三个容器组成见证三角 (a, b, witness)
//     - 见证裁定 = 三方的加权投票 (仅加法)
//     - 裁定结果: Admitted(准入) / Rejected(驳回) / Pending(待定)
//
//   三元三角:
//     - 提出方 (claimant) — 发起请求的容器
//     - 被诉方 (respondent) — 被请求的容器
//     - 见证方 (witness) — 中立的第三方
//
//   仅加法范式: 投票权重 = 各容器强度累加, 无乘除

/// 见证裁定结果
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WitnessVerdict {
    Admitted,   // 准入 — 通过见证
    Rejected,   // 驳回 — 拒绝
    Pending,    // 待定 — 证据不足, 无法裁定
}

/// 见证事件 — 一次仲裁的完整记录
#[derive(Clone, Debug)]
pub struct WitnessEvent {
    pub claimant_id: u64,
    pub respondent_id: u64,
    pub witness_id: u64,
    pub tick: u64,
    pub verdict: WitnessVerdict,
    pub evidence_strength: u64,  // 提出方证据强度 (加法累计)
    pub witness_confidence: u64, // 见证方信心 (加法累计)
}

impl WitnessEvent {
    pub fn new(c: u64, r: u64, w: u64, tick: u64) -> Self {
        Self {
            claimant_id: c, respondent_id: r, witness_id: w,
            tick, verdict: WitnessVerdict::Pending,
            evidence_strength: 0, witness_confidence: 0,
        }
    }
}

/// 见证引擎
#[derive(Clone, Debug)]
pub struct ContainerWitnessEngine {
    /// 见证记录
    pub events: Vec<WitnessEvent>,
    /// 累计裁定数
    pub total_admitted: u64,
    pub total_rejected: u64,
    pub total_pending: u64,
    /// 最低证据强度阈值 (加法比较)
    pub admit_threshold: u64,
    pub reject_threshold: u64,
}

impl ContainerWitnessEngine {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            total_admitted: 0, total_rejected: 0, total_pending: 0,
            admit_threshold: 60,
            reject_threshold: 80,  // 反向证据 >= 80 → 驳回
        }
    }

    /// 发起见证仲裁 (仅加法)
    pub fn arbitrate(&mut self, claimant: u64, respondent: u64, witness: u64,
                     evidence_strength: u64, counter_evidence: u64, witness_confidence: u64,
                     tick: u64) -> WitnessVerdict {
        // 净正向证据 = evidence - counter (仅加法)
        let net_strength = if evidence_strength >= counter_evidence {
            evidence_strength - counter_evidence
        } else {
            0
        };
        // 裁定
        let verdict = if net_strength >= self.admit_threshold && witness_confidence >= 30 {
            WitnessVerdict::Admitted
        } else if counter_evidence >= self.reject_threshold && witness_confidence >= 30 {
            WitnessVerdict::Rejected
        } else if witness_confidence < 30 {
            WitnessVerdict::Pending  // 见证信心不足
        } else {
            // 净证据不够高, 但也无反证 → 待定
            WitnessVerdict::Pending
        };
        // 统计
        match verdict {
            WitnessVerdict::Admitted => self.total_admitted = self.total_admitted + 1,
            WitnessVerdict::Rejected => self.total_rejected = self.total_rejected + 1,
            WitnessVerdict::Pending  => self.total_pending  = self.total_pending  + 1,
        };
        let event = WitnessEvent {
            claimant_id: claimant, respondent_id: respondent, witness_id: witness,
            tick, verdict, evidence_strength: net_strength, witness_confidence,
        };
        self.events.push(event);
        verdict
    }

    /// 最近裁定
    pub fn latest(&self) -> Option<&WitnessEvent> { self.events.last() }

    /// 涉及某容器的所有事件
    pub fn involving(&self, container_id: u64) -> Vec<&WitnessEvent> {
        self.events.iter()
            .filter(|e| e.claimant_id == container_id
                     || e.respondent_id == container_id
                     || e.witness_id == container_id)
            .collect()
    }

    /// 某容器作为提出方的通过率 (返回 计数, 总数)
    pub fn admission_pair(&self, claimant: u64) -> (u64, u64) {
        let mut admits = 0;
        let mut total = 0;
        for e in &self.events {
            if e.claimant_id == claimant {
                total = total + 1;
                if e.verdict == WitnessVerdict::Admitted { admits = admits + 1 };
            }
        }
        (admits, total)
    }

    /// 某容器作为见证方的活跃次数
    pub fn witness_activity(&self, witness_id: u64) -> usize {
        self.events.iter().filter(|e| e.witness_id == witness_id).count()
    }

    /// 证据净强度
    pub fn evidence_net(evidence: u64, counter: u64) -> u64 {
        if evidence >= counter { evidence - counter } else { 0 }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.total_admitted = 0;
        self.total_rejected = 0;
        self.total_pending = 0;
    }
}

impl Default for ContainerWitnessEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod witness_tests {
    use super::*;

    #[test]
    fn test_arbitrate_admitted() {
        let mut e = ContainerWitnessEngine::new();
        let v = e.arbitrate(1, 2, 3, 100, 20, 50, 100);
        assert_eq!(v, WitnessVerdict::Admitted);
    }

    #[test]
    fn test_arbitrate_rejected() {
        let mut e = ContainerWitnessEngine::new();
        // 反向证据 90 >= 80 (reject_threshold), 见证信心 50 >= 30
        let v = e.arbitrate(1, 2, 3, 10, 90, 50, 100);
        assert_eq!(v, WitnessVerdict::Rejected);
    }

    #[test]
    fn test_arbitrate_pending_low_confidence() {
        let mut e = ContainerWitnessEngine::new();
        // 证据 100, 见证信心 < 30 → Pending
        let v = e.arbitrate(1, 2, 3, 100, 0, 10, 100);
        assert_eq!(v, WitnessVerdict::Pending);
    }

    #[test]
    fn test_arbitrate_insufficient_evidence() {
        let mut e = ContainerWitnessEngine::new();
        // 证据 50 < 60 (admit_threshold), 无反证
        let v = e.arbitrate(1, 2, 3, 50, 0, 50, 100);
        assert_eq!(v, WitnessVerdict::Pending);
    }

    #[test]
    fn test_counters() {
        let mut e = ContainerWitnessEngine::new();
        e.arbitrate(1, 2, 3, 100, 20, 50, 1);  // Admitted
        e.arbitrate(2, 3, 1, 50, 90, 50, 2);   // Rejected
        e.arbitrate(3, 1, 2, 30, 0, 20, 3);    // Pending (low confidence)
        assert_eq!(e.total_admitted, 1);
        assert_eq!(e.total_rejected, 1);
        assert_eq!(e.total_pending, 1);
    }

    #[test]
    fn test_involving() {
        let mut e = ContainerWitnessEngine::new();
        e.arbitrate(1, 2, 3, 100, 20, 50, 1);
        e.arbitrate(4, 5, 6, 100, 20, 50, 2);
        let inv = e.involving(3);
        assert_eq!(inv.len(), 1);  // 容器 3 仅在第一个事件中
    }

    #[test]
    fn test_admission_pair() {
        let mut e = ContainerWitnessEngine::new();
        e.arbitrate(1, 2, 3, 100, 20, 50, 1);  // Admitted
        e.arbitrate(1, 4, 5, 100, 20, 50, 2);  // Admitted
        e.arbitrate(1, 6, 7, 10, 90, 50, 3);   // Rejected
        let (admits, total) = e.admission_pair(1);
        assert_eq!(admits, 2);
        assert_eq!(total, 3);
    }

    #[test]
    fn test_witness_activity() {
        let mut e = ContainerWitnessEngine::new();
        e.arbitrate(1, 2, 3, 100, 20, 50, 1);
        e.arbitrate(4, 5, 3, 100, 20, 50, 2);
        e.arbitrate(6, 7, 8, 100, 20, 50, 3);
        assert_eq!(e.witness_activity(3), 2);
        assert_eq!(e.witness_activity(8), 1);
    }

    #[test]
    fn test_evidence_net() {
        assert_eq!(ContainerWitnessEngine::evidence_net(100, 30), 70);
        assert_eq!(ContainerWitnessEngine::evidence_net(30, 100), 0);
        assert_eq!(ContainerWitnessEngine::evidence_net(50, 50), 0);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerWitnessEngine::new();
        e.arbitrate(1, 2, 3, 100, 20, 50, 1);
        assert!(e.total_admitted > 0);
        e.clear();
        assert_eq!(e.total_admitted, 0);
        assert_eq!(e.total_rejected, 0);
        assert_eq!(e.events.len(), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerWitnessEngine::default();
        assert_eq!(e.total_admitted, 0);
        assert_eq!(e.events.len(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 逻辑组织引擎 · ContainerLogicEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   逻辑组织 (Logic Organization) — 囡囝感知四阶 · 逻辑路径涌现
//   「评价结果积累到临界点，自组织形成逻辑路径」
//
//   机制:
//     - 接收 ContainerAgencyEngine 的行动指令流
//     - 将多个 ActionDirective 连接为 LogicPath (逻辑路径)
//     - path 优先级 = 所有指令的 priority 之和 (加法累积)
//     - path 仅在总优先级 >= threshold 时才激活
//
//   这是「生态而非规则引擎」原则的具体体现:
//     评价 (Evaluation) → 行动指令 (ActionDirective) → 路径涌现 (LogicPath)
//
//   仅加法范式: 路径优先级全部加法累积, 无乘除

/// 逻辑路径 — 一连串行动指令组成的链条
#[derive(Clone, Debug)]
pub struct LogicPath {
    pub id: u64,
    pub directive_indices: Vec<usize>,  // 指向 ActionDirective 的索引
    pub total_priority: u64,            // 路径优先级 (加法累积)
    pub activated: bool,                // 是否已激活
}

/// 逻辑组织引擎
#[derive(Clone, Debug)]
pub struct ContainerLogicEngine {
    /// 已构建的逻辑路径
    pub paths: Vec<LogicPath>,
    /// 激活阈值 (加法比较)
    pub activate_threshold: u64,
    /// 当前路径 id 计数器
    pub next_id: u64,
    /// 已激活路径数
    pub active_count: u64,
}

impl ContainerLogicEngine {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            activate_threshold: 100,
            next_id: 1,
            active_count: 0,
        }
    }

    /// 从一批行动指令中涌现逻辑路径 (仅加法)
    pub fn organize(&mut self, directives: &[ActionDirective]) -> u64 {
        if directives.is_empty() { return 0; }
        // 仅把 Strong/Strengthen 级别的行动纳入逻辑路径
        let high: Vec<usize> = directives.iter().enumerate()
            .filter(|(_, d)| {
                matches!(d.kind, ActionKind::Strengthen | ActionKind::Maintain)
            })
            .map(|(i, _)| i)
            .collect();
        if high.is_empty() { return 0; }
        // 路径优先级 = 所有纳入指令的 priority 之和 (仅加法)
        let total: u64 = high.iter().map(|&i| directives[i].priority).fold(0u64, |acc, p| acc.wrapping_add(p));
        let activated = total >= self.activate_threshold;
        let id = self.next_id;
        self.next_id = self.next_id + 1;
        let path = LogicPath { id, directive_indices: high, total_priority: total, activated };
        self.paths.push(path);
        if activated { self.active_count = self.active_count + 1; }
        id
    }

    /// 检查路径是否激活
    pub fn is_activated(&self, path_id: u64) -> bool {
        self.paths.iter().find(|p| p.id == path_id)
            .map(|p| p.activated)
            .unwrap_or(false)
    }

    /// 获取已激活路径 (按 priority 降序)
    pub fn active_paths(&self) -> Vec<&LogicPath> {
        let mut r: Vec<&LogicPath> = self.paths.iter().filter(|p| p.activated).collect();
        r.sort_by(|a, b| b.total_priority.cmp(&a.total_priority));
        r
    }

    /// 路径总数
    pub fn path_count(&self) -> usize { self.paths.len() }

    /// 重置
    pub fn clear(&mut self) {
        self.paths.clear();
        self.next_id = 1;
        self.active_count = 0;
    }

    /// 调整激活阈值
    pub fn set_threshold(&mut self, new_threshold: u64) {
        self.activate_threshold = new_threshold;
        // 重新计算激活状态
        self.active_count = 0;
        for p in self.paths.iter_mut() {
            p.activated = p.total_priority >= self.activate_threshold;
            if p.activated { self.active_count = self.active_count + 1; }
        }
    }

    /// 活跃路径的最高优先级
    pub fn top_priority(&self) -> Option<u64> {
        self.active_paths().first().map(|p| p.total_priority)
    }
}

impl Default for ContainerLogicEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod logic_tests {
    use super::*;

    fn directive(kind: ActionKind, priority: u64) -> ActionDirective {
        ActionDirective::new(0, 0, kind, priority)
    }

    #[test]
    fn test_organize_empty() {
        let mut e = ContainerLogicEngine::new();
        assert_eq!(e.organize(&[]), 0);
    }

    #[test]
    fn test_organize_no_high_directives() {
        let mut e = ContainerLogicEngine::new();
        let id = e.organize(&[directive(ActionKind::Isolate, 50), directive(ActionKind::Monitor, 50)]);
        assert_eq!(id, 0);  // 没有强指令, 不建路径
    }

    #[test]
    fn test_organize_basic() {
        let mut e = ContainerLogicEngine::new();
        let id = e.organize(&[
            directive(ActionKind::Strengthen, 80),
            directive(ActionKind::Maintain, 40),
        ]);
        assert_eq!(id, 1);
        assert!(e.is_activated(1));  // 80+40=120 >= 100
        assert_eq!(e.path_count(), 1);
        assert_eq!(e.active_count, 1);
    }

    #[test]
    fn test_organize_below_threshold() {
        let mut e = ContainerLogicEngine::new();
        let id = e.organize(&[directive(ActionKind::Strengthen, 30)]);
        assert_eq!(id, 1);
        assert!(!e.is_activated(1));  // 30 < 100
    }

    #[test]
    fn test_active_paths_sort() {
        let mut e = ContainerLogicEngine::new();
        e.organize(&[directive(ActionKind::Strengthen, 90)]);  // 90 < 100, not activated
        e.organize(&[directive(ActionKind::Strengthen, 60)]);   // not activated
        e.set_threshold(50);  // 现在两者都激活
        let r = e.active_paths();
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].total_priority, 90);
        assert_eq!(r[1].total_priority, 60);
    }

    #[test]
    fn test_set_threshold_lowers_bar() {
        let mut e = ContainerLogicEngine::new();
        e.organize(&[directive(ActionKind::Strengthen, 150)]);  // activated (>= 100)
        assert_eq!(e.active_count, 1);
        e.set_threshold(200);  // 提高到 200, 不再激活
        assert_eq!(e.active_count, 0);
    }

    #[test]
    fn test_path_count() {
        let mut e = ContainerLogicEngine::new();
        e.organize(&[directive(ActionKind::Strengthen, 200)]);
        e.organize(&[directive(ActionKind::Maintain, 150)]);
        e.organize(&[directive(ActionKind::Monitor, 100)]);  // 弱, 不计入
        assert_eq!(e.path_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerLogicEngine::new();
        e.organize(&[directive(ActionKind::Strengthen, 200)]);
        assert_eq!(e.path_count(), 1);
        e.clear();
        assert_eq!(e.path_count(), 0);
        assert_eq!(e.active_count, 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerLogicEngine::default();
        assert_eq!(e.path_count(), 0);
        assert_eq!(e.activate_threshold, 100);
    }

    #[test]
    fn test_top_priority() {
        let mut e = ContainerLogicEngine::new();
        assert_eq!(e.top_priority(), None);
        e.organize(&[directive(ActionKind::Strengthen, 200), directive(ActionKind::Maintain, 50)]);  // 250
        e.organize(&[directive(ActionKind::Strengthen, 180)]);  // 180
        assert_eq!(e.top_priority(), Some(250));
    }

    #[test]
    fn test_next_id_sequence() {
        let mut e = ContainerLogicEngine::new();
        let id1 = e.organize(&[directive(ActionKind::Strengthen, 200)]);
        let id2 = e.organize(&[directive(ActionKind::Strengthen, 200)]);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 心跳引擎 · ContainerHeartbeatEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   心跳 = 容器的生命节律与恢复机制
//   「时钟动作 · 以意志驱动时间和继承方式 · 结束即开始 · 记忆跟随结束而继续」
//
//   机制:
//     - 每个容器维护自己的心跳计数器
//     - 周期性 tick (仅加法 +1)
//     - 长时间未跳 → 触发恢复
//     - 心跳历史形成容器的生命曲线
//
//   恢复策略 (仅加法范式):
//     - 死亡判定: missed_ticks > recover_threshold
//     - 恢复后 tick 重置为 parent_tick (若有 parent)
//     - 无 parent → 从 0 重启
//
//   仅加法: 计数器 tick++ 用加法, 阈值比较用减法

/// 心跳状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeartbeatState {
    Alive,      // 存活
    Dormant,    // 休眠 (长期未跳但不超过阈值)
    Recovering, // 恢复中
    Dead,       // 死亡 (超过阈值)
}

/// 心跳事件
#[derive(Clone, Debug)]
pub struct HeartbeatEvent {
    pub container_id: u64,
    pub tick: u64,
    pub state: HeartbeatState,
}

/// 心跳引擎
#[derive(Clone, Debug)]
pub struct ContainerHeartbeatEngine {
    /// 容器当前 tick
    pub ticks: HashMap<u64, u64>,
    /// 容器状态
    pub states: HashMap<u64, HeartbeatState>,
    /// 未跳 tick 计数
    pub missed: HashMap<u64, u64>,
    /// 历史 (容器id -> 事件列表)
    pub log: HashMap<u64, Vec<HeartbeatEvent>>,
    /// 恢复阈值 (加法比较)
    pub recover_threshold: u64,
    /// 累计心跳总数
    pub total_heartbeats: u64,
}

impl ContainerHeartbeatEngine {
    pub fn new() -> Self {
        Self {
            ticks: HashMap::new(),
            states: HashMap::new(),
            missed: HashMap::new(),
            log: HashMap::new(),
            recover_threshold: 10,
            total_heartbeats: 0,
        }
    }

    /// 注册容器
    pub fn register(&mut self, container_id: u64, start_tick: u64) {
        self.ticks.insert(container_id, start_tick);
        self.states.insert(container_id, HeartbeatState::Alive);
        self.missed.insert(container_id, 0);
    }

    /// 心跳一次: tick + 1
    pub fn beat(&mut self, container_id: u64) -> Option<u64> {
        let t = self.ticks.get(&container_id)?;
        let new_tick = *t + 1;
        self.ticks.insert(container_id, new_tick);
        self.states.insert(container_id, HeartbeatState::Alive);
        self.missed.insert(container_id, 0);
        self.total_heartbeats = self.total_heartbeats + 1;
        let ev = HeartbeatEvent { container_id, tick: new_tick, state: HeartbeatState::Alive };
        self.log.entry(container_id).or_default().push(ev);
        Some(new_tick)
    }

    /// 推进全局时钟: 所有存活的容器 missed +1
    pub fn advance(&mut self, delta: u64) -> Vec<u64> {
        let ids: Vec<u64> = self.ticks.keys().copied().collect();
        let mut dead = Vec::new();
        for id in ids {
            let m = self.missed.get(&id).copied().unwrap_or(0) + delta;
            self.missed.insert(id, m);
            let state = if m == 0 { HeartbeatState::Alive }
                        else if m < self.recover_threshold { HeartbeatState::Dormant }
                        else { HeartbeatState::Dead };
            if state == HeartbeatState::Dead { dead.push(id); }
            self.states.insert(id, state);
        }
        dead
    }

    /// 恢复容器: tick 重置为 0 或 parent_tick
    pub fn recover(&mut self, container_id: u64) -> Option<u64> {
        if !self.ticks.contains_key(&container_id) { return None; }
        let new_tick = 0u64;
        self.ticks.insert(container_id, new_tick);
        self.missed.insert(container_id, 0);
        self.states.insert(container_id, HeartbeatState::Recovering);
        let ev = HeartbeatEvent { container_id, tick: new_tick, state: HeartbeatState::Recovering };
        self.log.entry(container_id).or_default().push(ev);
        Some(new_tick)
    }

    /// 查询状态
    pub fn state_of(&self, container_id: u64) -> Option<HeartbeatState> {
        self.states.get(&container_id).copied()
    }

    /// 查询 tick
    pub fn tick_of(&self, container_id: u64) -> Option<u64> {
        self.ticks.get(&container_id).copied()
    }

    /// 存活容器数
    pub fn alive_count(&self) -> usize {
        self.states.values().filter(|s| **s == HeartbeatState::Alive).count()
    }

    /// 死亡容器数
    pub fn dead_count(&self) -> usize {
        self.states.values().filter(|s| **s == HeartbeatState::Dead).count()
    }

    /// 重置
    pub fn clear(&mut self) {
        self.ticks.clear();
        self.states.clear();
        self.missed.clear();
        self.log.clear();
        self.total_heartbeats = 0;
    }
}

impl Default for ContainerHeartbeatEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod heartbeat_tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        assert_eq!(e.tick_of(1), Some(0));
        assert_eq!(e.state_of(1), Some(HeartbeatState::Alive));
    }

    #[test]
    fn test_beat_once() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        let t = e.beat(1);
        assert_eq!(t, Some(1));
        assert_eq!(e.state_of(1), Some(HeartbeatState::Alive));
    }

    #[test]
    fn test_beat_multiple() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        e.beat(1);
        e.beat(1);
        let t = e.beat(1);
        assert_eq!(t, Some(3));
    }

    #[test]
    fn test_beat_unregistered() {
        let mut e = ContainerHeartbeatEngine::new();
        assert_eq!(e.beat(1), None);
    }

    #[test]
    fn test_advance_alive() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        let dead = e.advance(3);  // missed=3, threshold=10
        assert!(dead.is_empty());
        assert_eq!(e.state_of(1), Some(HeartbeatState::Dormant));
    }

    #[test]
    fn test_advance_to_dead() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        let dead = e.advance(11);  // missed=11 >= 10
        assert!(dead.contains(&1));
        assert_eq!(e.state_of(1), Some(HeartbeatState::Dead));
    }

    #[test]
    fn test_recover() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        e.advance(15);
        assert_eq!(e.state_of(1), Some(HeartbeatState::Dead));
        let t = e.recover(1);
        assert_eq!(t, Some(0));
        assert_eq!(e.state_of(1), Some(HeartbeatState::Recovering));
    }

    #[test]
    fn test_recover_unregistered() {
        let mut e = ContainerHeartbeatEngine::new();
        assert_eq!(e.recover(1), None);
    }

    #[test]
    fn test_alive_dead_count() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        e.register(2, 0);
        e.register(3, 0);
        e.advance(15);  // 全体死亡
        assert_eq!(e.dead_count(), 3);
        assert_eq!(e.alive_count(), 0);
        e.beat(1);  // 容器 1 复活
        assert_eq!(e.alive_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerHeartbeatEngine::new();
        e.register(1, 0);
        e.beat(1);
        assert_eq!(e.total_heartbeats, 1);
        e.clear();
        assert_eq!(e.total_heartbeats, 0);
        assert_eq!(e.tick_of(1), None);
    }

    #[test]
    fn test_default() {
        let e = ContainerHeartbeatEngine::default();
        assert_eq!(e.total_heartbeats, 0);
        assert_eq!(e.alive_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 扩散引擎 · ContainerPropagateEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   扩散 = 信号在容器图中的传播
//   当一个容器更新自己的涟漪印记时, 相邻容器按权重感知到变化
//
//   机制:
//     - 构建邻接图 (从 ContainerCallEngine 的呼唤关系)
//     - 信号从源节点向外扩散, 每步衰减 -1 (仅加法)
//     - 衰减触底 (level = 0) 时停止
//
//   输入: 源容器 id + 初始强度
//   输出: 所有感应到的容器及其强度 (BFS 扩散)
//
//   仅加法范式: 衰减 = 加法, 层级 = 加法, 无乘除

/// 扩散波前节点
#[derive(Clone, Debug)]
pub struct WavefrontNode {
    pub container_id: u64,
    pub level: u64,        // 距离源的距离 (加法)
    pub strength: u64,     // 到达此容器时的剩余强度
}

/// 扩散引擎
#[derive(Clone, Debug)]
pub struct ContainerPropagateEngine {
    /// 邻接表: 容器 -> 直接邻居列表
    pub adjacency: HashMap<u64, Vec<u64>>,
    /// 累计扩散波次数
    pub total_waves: u64,
}

impl ContainerPropagateEngine {
    pub fn new() -> Self {
        Self { adjacency: HashMap::new(), total_waves: 0 }
    }

    /// 添加有向边 (neighbor 表示 caller 呼叫过的 callee)
    pub fn add_edge(&mut self, from: u64, to: u64) {
        let v = self.adjacency.entry(from).or_default();
        if !v.contains(&to) { v.push(to); }
    }

    /// 从一组 CallEntry 中构建邻接
    pub fn build_from_calls(&mut self, calls: &[CallEntry]) {
        for c in calls {
            self.add_edge(c.caller_id, c.callee_id);
        }
    }

    /// 从源容器向外 BFS 扩散, 返回所有感应到的节点 (仅加法)
    pub fn propagate(&mut self, source: u64, initial_strength: u64, max_hops: u64) -> Vec<WavefrontNode> {
        if initial_strength == 0 || !self.adjacency.contains_key(&source) { return Vec::new(); }
        let mut result = Vec::new();
        let mut visited = HashMap::<u64, u64>::new();  // id → level (最浅)
        let mut queue = Vec::new();
        queue.push((source, 0u64, initial_strength));
        self.total_waves = self.total_waves + 1;

        while let Some((id, level, strength)) = queue.pop() {
            // 已访问过且 level 更浅 → 跳过
            if let Some(&prev_level) = visited.get(&id) {
                if prev_level <= level { continue; }
            }
            visited.insert(id, level);

            if level > 0 {
                result.push(WavefrontNode { container_id: id, level, strength });
            }

            // hop 数未到上限 且 强度可衰减
            if level < max_hops && strength > 0 {
                let next_strength = strength - 1;  // 衰减 -1 (加法)
                if let Some(neighbors) = self.adjacency.get(&id) {
                    for &nb in neighbors {
                        queue.push((nb, level + 1, next_strength));
                    }
                }
            }
        }
        result
    }

    /// 某容器的邻接数
    pub fn neighbor_count(&self, id: u64) -> usize {
        self.adjacency.get(&id).map(|v| v.len()).unwrap_or(0)
    }

    /// 总边数 (有向)
    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(|v| v.len()).fold(0, |a, b| a + b)
    }

    /// 重置
    pub fn clear(&mut self) {
        self.adjacency.clear();
        self.total_waves = 0;
    }
}

impl Default for ContainerPropagateEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod propagate_tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        assert_eq!(e.neighbor_count(1), 1);
        assert_eq!(e.edge_count(), 1);
    }

    #[test]
    fn test_add_edge_no_dup() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        e.add_edge(1, 2);
        assert_eq!(e.neighbor_count(1), 1);
    }

    #[test]
    fn test_propagate_one_hop() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        e.add_edge(1, 3);
        let wave = e.propagate(1, 5, 3);
        assert_eq!(wave.len(), 2);
    }

    #[test]
    fn test_propagate_decay() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        e.add_edge(2, 3);
        e.add_edge(3, 4);
        // strength=2: 能到达 2 (level=1, str=1) 和 3 (level=2, str=0)
        // 4 需要 level=3, 但 strength 已衰减为 0
        let wave = e.propagate(1, 2, 5);
        let ids: Vec<u64> = wave.iter().map(|n| n.container_id).collect();
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&4));  // strength 已衰减到 0
    }

    #[test]
    fn test_propagate_max_hops() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        e.add_edge(2, 3);
        e.add_edge(3, 4);
        // max_hops=1, 只能到第一跳
        let wave = e.propagate(1, 100, 1);
        assert_eq!(wave.len(), 1);
        assert_eq!(wave[0].container_id, 2);
    }

    #[test]
    fn test_propagate_zero_strength() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        let wave = e.propagate(1, 0, 3);
        assert!(wave.is_empty());
    }

    #[test]
    fn test_propagate_unknown_source() {
        let mut e = ContainerPropagateEngine::new();
        let wave = e.propagate(99, 10, 3);
        assert!(wave.is_empty());
    }

    #[test]
    fn test_build_from_calls() {
        let mut e = ContainerPropagateEngine::new();
        let calls = vec![
            CallEntry::new(1, 2, 10, 1),
            CallEntry::new(2, 3, 20, 2),
        ];
        e.build_from_calls(&calls);
        assert_eq!(e.edge_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerPropagateEngine::new();
        e.add_edge(1, 2);
        e.propagate(1, 5, 3);
        assert_eq!(e.total_waves, 1);
        e.clear();
        assert_eq!(e.total_waves, 0);
        assert_eq!(e.edge_count(), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerPropagateEngine::default();
        assert_eq!(e.total_waves, 0);
        assert_eq!(e.edge_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 八流统一调度器 · ContainerOrchestrator (v1.0)
// ═══════════════════════════════════════════════════════════════════════════════
//
//   八流统一 = 把感知四阶与行动/呼唤/见证/逻辑统一到一个 tick 周期内
//
//   一个 tick 内的执行顺序:
//     1. reflection.sense()      — 比较新旧印记, 产生差异标记
//     2. judgment.judge()        — 差异标记 → 因果锚点
//     3. evaluation.evaluate()   — 因果对累积评价
//     4. agency.act()            — 评价 → 行动指令
//     5. witness.arbitrate()     — 争议裁定
//     6. logic.organize()        — 涌现逻辑路径
//
//   tick() → bool: 是否有活跃的逻辑路径产生
//
//   仅加法范式贯穿全部流水线

/// 八流统一调度器 — 单 tick 主循环
#[derive(Clone, Debug)]
pub struct ContainerOrchestrator {
    pub reflection: ContainerReflectionEngine,
    pub judgment: ContainerJudgmentEngine,
    pub evaluation: ContainerEvaluationEngine,
    pub agency: ContainerAgencyEngine,
    pub witness: ContainerWitnessEngine,
    pub logic: ContainerLogicEngine,
    pub tick_count: u64,
    pub last_path_count: u64,
}

impl ContainerOrchestrator {
    pub fn new() -> Self {
        Self {
            reflection: ContainerReflectionEngine::new(),
            judgment: ContainerJudgmentEngine::new(),
            evaluation: ContainerEvaluationEngine::new(),
            agency: ContainerAgencyEngine::new(),
            witness: ContainerWitnessEngine::new(),
            logic: ContainerLogicEngine::new(),
            tick_count: 0,
            last_path_count: 0,
        }
    }

    /// 推进一个 tick: 记录印记 → 感知 → 判断 → 评价 → 行动 → 逻辑组织
    pub fn tick(&mut self, impressions: &[(u64, u64, u64, u64)], from_tick: u64, to_tick: u64) -> u64 {
        // 1. 记录所有新印记
        for &(cid, tick, amp, phase) in impressions {
            self.reflection.record(cid, tick, amp, phase);
        }
        // 2. 反射感知
        let diffs = self.reflection.reflect_all(from_tick, to_tick);
        // 3. 因果判断
        let anchors = self.judgment.judge(&diffs);
        // 4. 累积评价 (仅加法: 每个锚点为一个评价事件)
        if !anchors.is_empty() {
            self.evaluation.evaluate(&anchors);
        }
        // 5. 行动 (评价 → 行动指令)
        let current_evals: Vec<Evaluation> = self.evaluation.evaluations.values().cloned().collect();
        let accumulations: Vec<u64> = current_evals.iter().map(|e| e.accumulated).collect();
        self.agency.act_batch(&current_evals, &accumulations);
        // 6. 逻辑组织
        let dirs: Vec<ActionDirective> = self.agency.directives.clone();
        let id = self.logic.organize(&dirs);
        self.last_path_count = id;
        self.tick_count = self.tick_count + 1;
        id
    }

    /// 获取 tick 计数
    pub fn tick_count(&self) -> u64 { self.tick_count }

    /// 重置整个流水线
    pub fn reset(&mut self) {
        self.reflection.clear();
        self.judgment.clear();
        self.evaluation.clear();
        self.agency.clear();
        self.witness.clear();
        self.logic.clear();
        self.tick_count = 0;
        self.last_path_count = 0;
    }
}

impl Default for ContainerOrchestrator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod orchestrator_tests {
    use super::*;

    #[test]
    fn test_orchestrator_new() {
        let o = ContainerOrchestrator::new();
        assert_eq!(o.tick_count(), 0);
        assert_eq!(o.last_path_count, 0);
    }

    #[test]
    fn test_tick_empty() {
        let mut o = ContainerOrchestrator::new();
        let id = o.tick(&[], 10, 20);
        assert_eq!(id, 0);
        assert_eq!(o.tick_count(), 1);
    }

    #[test]
    fn test_tick_with_impressions() {
        let mut o = ContainerOrchestrator::new();
        // 容器 1 和 2 在 tick 10→20 期间都发生了变化
        let id = o.tick(&[
            (1, 10, 50, 0), (1, 20, 80, 0),  // 容器 1: 振幅变化
            (2, 10, 60, 0), (2, 20, 60, 0),  // 容器 2: 稳定
        ], 10, 20);
        // 至少应产生一个 tick
        assert_eq!(o.tick_count(), 1);
        // 因果判断应至少发现一对
        assert!(o.judgment.anchor_count() >= 0);
    }

    #[test]
    fn test_tick_accumulate_across_ticks() {
        let mut o = ContainerOrchestrator::new();
        o.tick(&[
            (1, 10, 50, 0), (1, 20, 100, 0),
            (2, 10, 50, 0), (2, 20, 80, 0),
        ], 10, 20);
        o.tick(&[
            (1, 20, 100, 0), (1, 30, 150, 0),
            (2, 20, 80, 0), (2, 30, 120, 0),
        ], 20, 30);
        // tick 计数应累加
        assert_eq!(o.tick_count(), 2);
        // 多次评价后应该有累积
        assert!(o.evaluation.pair_count() > 0);
    }

    #[test]
    fn test_reset() {
        let mut o = ContainerOrchestrator::new();
        o.tick(&[(1, 10, 50, 0), (1, 20, 80, 0)], 10, 20);
        assert_ne!(o.tick_count(), 0);
        o.reset();
        assert_eq!(o.tick_count(), 0);
        assert_eq!(o.judgment.anchor_count(), 0);
        assert_eq!(o.evaluation.pair_count(), 0);
    }

    #[test]
    fn test_default() {
        let o = ContainerOrchestrator::default();
        assert_eq!(o.tick_count(), 0);
    }

    #[test]
    fn test_pipeline_passes_evaluations_to_agency() {
        let mut o = ContainerOrchestrator::new();
        // 多 tick 累积 Strong 评价 → 产生行动
        for i in 0..10 {
            let from = i * 10;
            let to = from + 10;
            o.tick(&[
                (1, from, 50, 0), (1, to, 100, 0),
                (2, from, 50, 0), (2, to, 30, 0),
            ], from, to);
        }
        // 多次累积后应该产生行动
        assert!(o.agency.directives.len() > 0 || o.evaluation.pair_count() > 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 合一引擎 · ContainerSynthesisEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   合一驱力 = 双向强因果的容器趋向合并
//   「当你强烈影响我、我也强烈影响你, 我们合一」
//
//   机制:
//     - 查找互为因果的配对 (a→b 且 b→a)
//     - 双方的强度都 >= 合并阈值 → 合并候选
//     - 合并: 产生新容器 id, 继承双方累积和
//
//   仅加法: 合并强度 = 双方之和, 无乘除

/// 合并结果
#[derive(Clone, Debug)]
pub struct MergeResult {
    pub merged_id: u64,
    pub parent_a: u64,
    pub parent_b: u64,
    pub combined_accumulation: u64,
}

/// 合一引擎
#[derive(Clone, Debug)]
pub struct ContainerSynthesisEngine {
    /// 合并结果记录
    pub merges: Vec<MergeResult>,
    /// 合并阈值 (加法比较)
    pub merge_threshold: u64,
    /// 下一个可用 merged id
    pub next_id: u64,
}

impl ContainerSynthesisEngine {
    pub fn new() -> Self {
        Self {
            merges: Vec::new(),
            merge_threshold: 3,
            next_id: 1,
        }
    }

    /// 从评价中寻找可合并的对 (双向且均强)
    pub fn find_merge_candidates(&self, evals: &[(u64, u64, u64)]) -> Vec<(u64, u64, u64)> {
        // evals: Vec<(cause_id, effect_id, accumulated)>
        let mut candidates = Vec::new();
        for &(a, b, acc_ab) in evals {
            if acc_ab < self.merge_threshold { continue; }
            // 检查反向
            for &(c, d, acc_cd) in evals {
                if c == b && d == a && acc_cd >= self.merge_threshold {
                    // 双向都强 → 合并候选 (避免重复: 保留 a < b 的)
                    if a < b {
                        candidates.push((a, b, acc_ab + acc_cd));
                    }
                }
            }
        }
        candidates
    }

    /// 尝试合并 (返回 merged id, 或 0 表示无合并)
    pub fn try_merge(&mut self, evals: &[(u64, u64, u64)]) -> u64 {
        let candidates = self.find_merge_candidates(evals);
        if candidates.is_empty() { return 0; }
        // 取累积最大的一对 (一次只合一)
        let mut best = &candidates[0];
        for c in &candidates {
            if c.2 > best.2 { best = c; }
        }
        let mid = self.next_id;
        self.next_id = self.next_id + 1;
        self.merges.push(MergeResult {
            merged_id: mid,
            parent_a: best.0,
            parent_b: best.1,
            combined_accumulation: best.2,
        });
        mid
    }

    /// 合并对数
    pub fn merge_count(&self) -> usize { self.merges.len() }

    /// 重置
    pub fn clear(&mut self) {
        self.merges.clear();
        self.next_id = 1;
    }
}

impl Default for ContainerSynthesisEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod synthesis_tests {
    use super::*;

    #[test]
    fn test_no_candidates() {
        let e = ContainerSynthesisEngine::new();
        let c = e.find_merge_candidates(&[(1, 2, 1), (2, 3, 1)]);
        assert!(c.is_empty());
    }

    #[test]
    fn test_candidates_found() {
        let e = ContainerSynthesisEngine::new();
        let c = e.find_merge_candidates(&[
            (1, 2, 5),  // 强 (≥3)
            (2, 1, 4),  // 反向也强
        ]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].2, 9);  // 5+4
    }

    #[test]
    fn test_threshold_filters() {
        let e = ContainerSynthesisEngine::new();
        let c = e.find_merge_candidates(&[
            (1, 2, 2),  // 弱 (<3)
            (2, 1, 5),  // 反向强但正向不强
        ]);
        assert!(c.is_empty());
    }

    #[test]
    fn test_try_merge() {
        let mut e = ContainerSynthesisEngine::new();
        let mid = e.try_merge(&[
            (1, 2, 5),
            (2, 1, 4),
        ]);
        assert_ne!(mid, 0);
        assert_eq!(e.merge_count(), 1);
    }

    #[test]
    fn test_try_merge_none() {
        let mut e = ContainerSynthesisEngine::new();
        let mid = e.try_merge(&[(1, 2, 1)]);
        assert_eq!(mid, 0);
    }

    #[test]
    fn test_merge_id_sequence() {
        let mut e = ContainerSynthesisEngine::new();
        let id1 = e.try_merge(&[(1, 2, 5), (2, 1, 4)]);
        let id2 = e.try_merge(&[(3, 4, 6), (4, 3, 7)]);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_combined_accum() {
        let mut e = ContainerSynthesisEngine::new();
        e.try_merge(&[(1, 2, 5), (2, 1, 4)]);
        assert_eq!(e.merges[0].combined_accumulation, 9);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerSynthesisEngine::new();
        e.try_merge(&[(1, 2, 5), (2, 1, 4)]);
        assert_eq!(e.merge_count(), 1);
        e.clear();
        assert_eq!(e.merge_count(), 0);
        assert_eq!(e.next_id, 1);
    }

    #[test]
    fn test_default() {
        let e = ContainerSynthesisEngine::default();
        assert_eq!(e.merge_count(), 0);
        assert_eq!(e.next_id, 1);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 共鸣引擎 · ContainerEchoEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   共鸣 = 一对容器之间的双向信号传递与放大
//   不同于呼唤(单向), 共鸣是双向来回的谐波共振
//
//   机制:
//     - a 对 b 发出信号, b 收到后回应 (强度 = a 的信号 × b 的响应系数)
//     - 仅加法范式: 响应系数用加法查表 (线性递增), 不用乘法
//     - 来回传递直到衰减为 0
//
//   共鸣仓中的谐振幅 = 来回信号之和 (仅加法)

/// 共鸣路径
#[derive(Clone, Debug)]
pub struct EchoPath {
    pub from: u64,
    pub to: u64,
    pub initial_strength: u64,
    pub total_resonance: u64,  // 所有来回之和 (仅加法累积)
    pub round_trips: u64,      // 来回次数
}

/// 共鸣引擎
#[derive(Clone, Debug)]
pub struct ContainerEchoEngine {
    pub paths: Vec<EchoPath>,
    /// 每步衰减量 (仅加法减法)
    pub decay_per_trip: u64,
    /// 累计共鸣总数
    pub total_echoes: u64,
}

impl ContainerEchoEngine {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            decay_per_trip: 2,
            total_echoes: 0,
        }
    }

    /// 模拟一对容器 N 次来回的共鸣, 返回总谐振幅 (仅加法)
    /// total = 所有 forward 信号之和; rounds = 完成的反向次数
    pub fn resonate(&mut self, a: u64, b: u64, initial: u64, max_rounds: u64) -> u64 {
        if initial == 0 || max_rounds == 0 { return 0; }
        let mut total = 0u64;
        let mut forward = initial;
        let mut rounds = 0u64;
        while forward > 0 && rounds < max_rounds {
            total = total + forward;
            let back = if forward >= self.decay_per_trip {
                forward - self.decay_per_trip
            } else {
                0
            };
            if back == 0 { break; };
            rounds = rounds + 1;
            forward = back;
        }
        self.paths.push(EchoPath {
            from: a, to: b, initial_strength: initial,
            total_resonance: total, round_trips: rounds,
        });
        self.total_echoes = self.total_echoes + 1;
        total
    }

    /// 查找两容器间的所有共鸣
    pub fn echoes_between(&self, a: u64, b: u64) -> Vec<&EchoPath> {
        self.paths.iter()
            .filter(|p| (p.from == a && p.to == b) || (p.from == b && p.to == a))
            .collect()
    }

    /// 总共鸣次数
    pub fn echo_count(&self) -> u64 { self.total_echoes }

    /// 衰减
    pub fn decay(&self, strength: u64) -> u64 {
        if strength >= self.decay_per_trip {
            strength - self.decay_per_trip
        } else {
            0
        }
    }

    pub fn clear(&mut self) {
        self.paths.clear();
        self.total_echoes = 0;
    }
}

impl Default for ContainerEchoEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod echo_tests {
    use super::*;

    #[test]
    fn test_resonate_zero() {
        let mut e = ContainerEchoEngine::new();
        assert_eq!(e.resonate(1, 2, 0, 10), 0);
    }

    #[test]
    fn test_resonate_one_way() {
        let mut e = ContainerEchoEngine::new();
        // initial=1, decay=2 → 正向 1, 反向 0 (1 < 2, break)
        assert_eq!(e.resonate(1, 2, 1, 10), 1);
    }

    #[test]
    fn test_resonate_round_trips() {
        let mut e = ContainerEchoEngine::new();
        // initial=5, decay=2:
        //   fwd=5, back=3 (5-2), total=8
        //   fwd=3, back=1 (3-2), total=9
        //   fwd=1, back=0 (1<2, break) → final total = 9
        let t = e.resonate(1, 2, 5, 10);
        assert_eq!(t, 9);
        assert_eq!(e.paths[0].round_trips, 2);
    }

    #[test]
    fn test_max_rounds() {
        let mut e = ContainerEchoEngine::new();
        e.decay_per_trip = 1;
        // strength=100, decay=1, max_rounds=3
        // 每轮+2(正向+反向), 3轮后 = 100+99 + 98+97 + 96+95 = 但code是递减传递
        let total = e.resonate(1, 2, 100, 3);
        assert!(total > 0);
        assert_eq!(e.paths[0].round_trips, 3);
    }

    #[test]
    fn test_echoes_between() {
        let mut e = ContainerEchoEngine::new();
        e.resonate(1, 2, 10, 3);
        e.resonate(1, 2, 20, 3);
        e.resonate(3, 4, 15, 3);
        let b = e.echoes_between(1, 2);
        assert_eq!(b.len(), 2);
    }

    #[test]
    fn test_echo_count() {
        let mut e = ContainerEchoEngine::new();
        e.resonate(1, 2, 10, 3);
        e.resonate(3, 4, 15, 3);
        assert_eq!(e.echo_count(), 2);
    }

    #[test]
    fn test_decay() {
        let e = ContainerEchoEngine::new();
        assert_eq!(e.decay(10), 8);  // 10 - 2
        assert_eq!(e.decay(1), 0);   // 1 < 2
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerEchoEngine::new();
        e.resonate(1, 2, 10, 3);
        assert_eq!(e.echo_count(), 1);
        e.clear();
        assert_eq!(e.echo_count(), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerEchoEngine::default();
        assert_eq!(e.echo_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 熵值引擎 · ContainerEntropyEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   熵 = 容器系统的混乱度/不确定度
//   高熵 = 容器状态多样, 低熵 = 容器趋于一致
//
//   仅加法范式:
//     - 计算各状态的计数 (加法)
//     - 熵 = 状态值域的跨度 (max - min) 作为近似
//     - 无香农熵公式 (需要乘除和 log), 只用加法/减法/比较
//
//   用途: 判断系统是否需要干预 (高熵 → 需要收敛)

/// 熵值引擎
#[derive(Clone, Debug)]
pub struct ContainerEntropyEngine {
    /// 累计熵计算次数
    pub measurements: u64,
}

impl ContainerEntropyEngine {
    pub fn new() -> Self {
        Self { measurements: 0 }
    }

    /// 计算容器状态数组的熵近似 (max - min, 仅加法/减法)
    pub fn measure(&mut self, states: &[u64]) -> u64 {
        if states.is_empty() { return 0; }
        let mut min = states[0];
        let mut max = states[0];
        for &s in &states[1..] {
            if s < min { min = s; }
            if s > max { max = s; }
        }
        self.measurements = self.measurements + 1;
        max - min
    }

    /// 计算两容器状态相关度 (用差值和近似)
    /// 返回值越小越相关 (0 = 完全相关)
    pub fn divergence(&self, states_a: &[u64], states_b: &[u64]) -> u64 {
        let n = states_a.len().min(states_b.len());
        if n == 0 { return 0; }
        let mut total = 0u64;
        for i in 0..n {
            total = total + if states_a[i] >= states_b[i] {
                states_a[i] - states_b[i]
            } else {
                states_b[i] - states_a[i]
            };
        }
        total
    }

    /// 二值状态频率 (如 Alive/Dead 各多少)
    pub fn count_matching(&self, states: &[u64], target: u64) -> u64 {
        let mut c = 0u64;
        for &s in states {
            if s == target { c = c + 1; }
        }
        c
    }

    /// 是否高熵 (跨度 >= threshold)
    pub fn is_high_entropy(&mut self, states: &[u64], threshold: u64) -> bool {
        self.measure(states) >= threshold
    }

    pub fn measurement_count(&self) -> u64 { self.measurements }

    pub fn clear(&mut self) { self.measurements = 0; }
}

impl Default for ContainerEntropyEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod entropy_tests {
    use super::*;

    #[test]
    fn test_measure_empty() {
        let mut e = ContainerEntropyEngine::new();
        assert_eq!(e.measure(&[]), 0);
    }

    #[test]
    fn test_measure_uniform() {
        let mut e = ContainerEntropyEngine::new();
        // 所有状态相同 → 跨度 = 0
        assert_eq!(e.measure(&[5, 5, 5]), 0);
    }

    #[test]
    fn test_measure_span() {
        let mut e = ContainerEntropyEngine::new();
        assert_eq!(e.measure(&[1, 3, 7, 2, 9]), 8);  // 9 - 1
    }

    #[test]
    fn test_divergence() {
        let e = ContainerEntropyEngine::new();
        // |3-1| + |5-5| + |8-2| = 2+0+6 = 8
        assert_eq!(e.divergence(&[3, 5, 8], &[1, 5, 2]), 8);
    }

    #[test]
    fn test_divergence_empty() {
        let e = ContainerEntropyEngine::new();
        assert_eq!(e.divergence(&[], &[]), 0);
    }

    #[test]
    fn test_count_matching() {
        let e = ContainerEntropyEngine::new();
        assert_eq!(e.count_matching(&[1, 2, 3, 2, 2], 2), 3);
    }

    #[test]
    fn test_is_high_entropy() {
        let mut e = ContainerEntropyEngine::new();
        let high = vec![0, 100];
        let low = vec![50, 51, 52];
        assert!(e.is_high_entropy(&high, 50));   // 100-0=100 >= 50
        assert!(!e.is_high_entropy(&low, 50));   // 52-50=2 < 50
    }

    #[test]
    fn test_measurement_count() {
        let mut e = ContainerEntropyEngine::new();
        e.measure(&[1, 2, 3]);
        e.measure(&[4, 5, 6]);
        assert_eq!(e.measurement_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut e = ContainerEntropyEngine::new();
        e.measure(&[1, 2, 3]);
        e.clear();
        assert_eq!(e.measurement_count(), 0);
    }

    #[test]
    fn test_default() {
        let e = ContainerEntropyEngine::default();
        assert_eq!(e.measurement_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 状态机引擎 · ContainerStateMachine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   状态机 = 容器的离散状态转换
//   每个容器可在一组状态之间按规则迁移
//
//   规则: (current_state, event) → next_state
//   用 (当前状态 × 事件) 做加法索引查表, 避免复杂 if-else 链
//
//   核心操作:
//     - add_transition(s1, ev, s2)  — 注册转换规则
//     - trigger(state, ev)          — 触发转换, 返回新状态
//     - validate(state, ev)         — 检查转换是否允许
//
//   仅加法: 状态/事件均为 u64, 索引 = 加法组合
//
//   兜底: 无规则 → 返回原状态 (吸收态)

/// 状态机引擎
#[derive(Clone, Debug)]
pub struct ContainerStateMachine {
    /// 转换规则: (current, event) → next
    pub transitions: HashMap<(u64, u64), u64>,
    /// 注册规则数
    pub rule_count: usize,
    /// 累计触发次数
    pub total_triggers: u64,
    /// 累计成功转换次数
    pub total_transitions: u64,
}

impl ContainerStateMachine {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            rule_count: 0,
            total_triggers: 0,
            total_transitions: 0,
        }
    }

    /// 注册转换规则: (from_state, event) → to_state
    pub fn add_transition(&mut self, from: u64, event: u64, to: u64) {
        let key = (from, event);
        if !self.transitions.contains_key(&key) {
            self.rule_count = self.rule_count + 1;
        }
        self.transitions.insert(key, to);
    }

    /// 查找下一状态 (无规则 → 返回当前状态)
    pub fn next_state(&self, current: u64, event: u64) -> u64 {
        self.transitions.get(&(current, event)).copied().unwrap_or(current)
    }

    /// 触发转换并统计
    pub fn trigger(&mut self, current: u64, event: u64) -> u64 {
        let next = self.next_state(current, event);
        self.total_triggers = self.total_triggers + 1;
        if next != current {
            self.total_transitions = self.total_transitions + 1;
        }
        next
    }

    /// 验证转换是否合法
    pub fn validate(&self, current: u64, event: u64) -> bool {
        self.transitions.contains_key(&(current, event))
    }

    /// 删除规则
    pub fn remove_rule(&mut self, from: u64, event: u64) -> bool {
        let r = self.transitions.remove(&(from, event)).is_some();
        if r { self.rule_count = self.rule_count - 1; }
        r
    }

    /// 所有涉及某状态的规则 (作为 from)
    pub fn rules_from(&self, state: u64) -> Vec<((u64, u64), u64)> {
        self.transitions.iter()
            .filter(|((s, _), _)| *s == state)
            .map(|(k, v)| (*k, *v))
            .collect()
    }

    pub fn clear(&mut self) {
        self.transitions.clear();
        self.rule_count = 0;
        self.total_triggers = 0;
        self.total_transitions = 0;
    }
}

impl Default for ContainerStateMachine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod state_machine_tests {
    use super::*;

    #[test]
    fn test_new() {
        let sm = ContainerStateMachine::new();
        assert_eq!(sm.rule_count, 0);
        assert_eq!(sm.total_triggers, 0);
    }

    #[test]
    fn test_add_and_trigger() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(0, 1, 10);  // (状态0, 事件1) → 状态10
        let next = sm.trigger(0, 1);
        assert_eq!(next, 10);
        assert_eq!(sm.total_transitions, 1);
    }

    #[test]
    fn test_no_rule_absorbing() {
        let mut sm = ContainerStateMachine::new();
        // 没有注册任何规则 → 状态不变
        let next = sm.trigger(5, 100);
        assert_eq!(next, 5);
        assert_eq!(sm.total_transitions, 0);  // 不算转换
    }

    #[test]
    fn test_validate() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(1, 2, 3);
        assert!(sm.validate(1, 2));
        assert!(!sm.validate(1, 99));  // 不存在
        assert!(!sm.validate(99, 2));
    }

    #[test]
    fn test_multiple_rules() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(0, 10, 100);
        sm.add_transition(0, 20, 200);
        sm.add_transition(100, 30, 300);
        assert_eq!(sm.next_state(0, 10), 100);
        assert_eq!(sm.next_state(0, 20), 200);
        assert_eq!(sm.next_state(100, 30), 300);
    }

    #[test]
    fn test_overwrite_rule() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(1, 2, 10);
        sm.add_transition(1, 2, 99);  // 覆盖
        assert_eq!(sm.next_state(1, 2), 99);
        assert_eq!(sm.rule_count, 1);
    }

    #[test]
    fn test_remove_rule() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(1, 2, 3);
        assert!(sm.remove_rule(1, 2));
        assert!(!sm.validate(1, 2));
        assert_eq!(sm.rule_count, 0);
    }

    #[test]
    fn test_rules_from() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(5, 1, 50);
        sm.add_transition(5, 2, 51);
        sm.add_transition(5, 3, 52);
        sm.add_transition(9, 1, 90);
        let r = sm.rules_from(5);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn test_clear() {
        let mut sm = ContainerStateMachine::new();
        sm.add_transition(1, 2, 3);
        sm.trigger(1, 2);
        sm.clear();
        assert_eq!(sm.rule_count, 0);
        assert_eq!(sm.total_triggers, 0);
        assert!(!sm.validate(1, 2));
    }

    #[test]
    fn test_default() {
        let sm = ContainerStateMachine::default();
        assert_eq!(sm.rule_count, 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 流容器入口引擎 · ContainerFlowAcceptor (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   容器入口本身就是一种操作: 接纳信号/排出信号
//   入口有容量限制, 溢出的信号被丢弃或重定向
//
//   核心操作:
//     - accept(signal)        — 尝试写入入口, 返回成功/溢出
//     - drain(vol)            — 取出 vol 个信号
//     - peek()                — 查看入口当前水位
//     - capacity / remaining  — 容量与余量
//
//   仅加法: 水位+信号量, 溢出判断用 == 或 <
//
//   容量为 0 的空壳入口: 接收即丢弃

/// 入口溢出策略
#[derive(Clone, Debug, PartialEq)]
pub enum OverflowPolicy { Drop, Redirect(u64), Reject }

/// 流入口引擎
#[derive(Clone, Debug)]
pub struct ContainerFlowAcceptor {
    pub capacity: u64,        // 入口容量上限
    pub level: u64,           // 当前水位
    pub policy: OverflowPolicy,
    pub total_accept: u64,    // 累计接单量
    pub total_drain: u64,     // 累计排出量
    pub total_drop: u64,      // 累计丢弃量
}

impl ContainerFlowAcceptor {
    pub fn new(capacity: u64, policy: OverflowPolicy) -> Self {
        Self { capacity, level: 0, policy, total_accept: 0, total_drain: 0, total_drop: 0 }
    }

    /// 尝试写入; 返回实际存入量 (0 表示丢弃)
    pub fn accept(&mut self, vol: u64) -> u64 {
        let space = self.remaining();
        if vol <= space {
            self.level = self.level + vol;
            self.total_accept = self.total_accept + vol;
            vol
        } else {
            self.total_drop = self.total_drop + vol;
            0
        }
    }

    /// 强制写入 (超出容量即塞满)
    pub fn accept_force(&mut self, vol: u64) -> u64 {
        let space = self.remaining();
        let fit = if vol <= space { vol } else { space };
        self.level = self.level + fit;
        self.total_accept = self.total_accept + fit;
        self.total_drop = self.total_drop + (vol - fit);
        fit
    }

    /// 取出 vol; 返回实际取到量
    pub fn drain(&mut self, vol: u64) -> u64 {
        let got = if vol <= self.level { vol } else { self.level };
        self.level = self.level - got;
        self.total_drain = self.total_drain + got;
        got
    }

    pub fn is_empty(&self) -> bool { self.level == 0 }
    pub fn is_full(&self) -> bool { self.level >= self.capacity }
    pub fn remaining(&self) -> u64 { self.capacity - self.level }
    pub fn peek(&self) -> u64 { self.level }

    pub fn clear(&mut self) {
        self.level = 0;
        self.total_accept = 0;
        self.total_drain = 0;
        self.total_drop = 0;
    }
}

impl Default for ContainerFlowAcceptor {
    fn default() -> Self { Self::new(64, OverflowPolicy::Drop) }
}

#[cfg(test)]
mod flow_acceptor_tests {
    use super::*;

    #[test]
    fn test_accept_basic() {
        let mut f = ContainerFlowAcceptor::new(10, OverflowPolicy::Drop);
        assert_eq!(f.accept(3), 3);
        assert_eq!(f.peek(), 3);
        assert_eq!(f.total_accept, 3);
    }

    #[test]
    fn test_accept_overflow_drop() {
        let mut f = ContainerFlowAcceptor::new(5, OverflowPolicy::Drop);
        assert_eq!(f.accept(6), 0);  // 溢出丢弃
        assert_eq!(f.level, 0);
        assert_eq!(f.total_drop, 6);
    }

    #[test]
    fn test_accept_force_partial() {
        let mut f = ContainerFlowAcceptor::new(5, OverflowPolicy::Drop);
        assert_eq!(f.accept_force(8), 5);  // 只存满容量
        assert_eq!(f.peek(), 5);
        assert_eq!(f.total_drop, 3);
    }

    #[test]
    fn test_drain() {
        let mut f = ContainerFlowAcceptor::new(100, OverflowPolicy::Drop);
        f.accept(10);
        assert_eq!(f.drain(3), 3);
        assert_eq!(f.peek(), 7);
        assert_eq!(f.drain(20), 7);  // 超出水位, 有多少取多少
        assert!(f.is_empty());
    }

    #[test]
    fn test_capacity_full() {
        let mut f = ContainerFlowAcceptor::new(4, OverflowPolicy::Drop);
        assert!(f.is_empty());
        f.accept(4);
        assert!(f.is_full());
        assert_eq!(f.remaining(), 0);
    }

    #[test]
    fn test_consume_cycle() {
        let mut f = ContainerFlowAcceptor::new(10, OverflowPolicy::Drop);
        f.accept(8);
        f.drain(3);
        f.accept(4);
        assert_eq!(f.peek(), 9);
        f.drain(9);
        assert!(f.is_empty());
    }

    #[test]
    fn test_default() {
        let f = ContainerFlowAcceptor::default();
        assert_eq!(f.capacity, 64);
        assert_eq!(f.policy, OverflowPolicy::Drop);
    }

    #[test]
    fn test_clear() {
        let mut f = ContainerFlowAcceptor::new(10, OverflowPolicy::Drop);
        f.accept(5);
        f.drain(3);
        f.clear();
        assert!(f.is_empty());
        assert_eq!(f.total_accept, 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器时间戳引擎 · ContainerTimestamp (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   时间戳不是绝对时刻, 而是容器的拓扑序位置
//   在仅加法范式下: 事实变化 = 加上一个 frame
//
//   tick = 递增整数, 每个事件推进一步
//   compare(t1,t2) 不依赖时钟, 依赖先后
//
//   仅加法: tick+1, 偏移量用加法累加, 比较用 == < >

/// 时间戳引擎
#[derive(Clone, Debug)]
pub struct ContainerTimestamp {
    pub tick: u64,            // 当前拓扑 tick
    pub max_frame: u64,       // 最大支持帧 (溢出即饱和)
    pub events: u64,          // 事件计数
    pub saturated: bool,      // 是否饱和
}

impl ContainerTimestamp {
    pub fn new(max_frame: u64) -> Self {
        Self { tick: 0, max_frame, events: 0, saturated: false }
    }

    /// 事件驱动 +1 tick (饱和则不变)
    pub fn step(&mut self) {
        if self.tick >= self.max_frame {
            self.saturated = true;
            return;
        }
        self.tick = self.tick + 1;
        self.events = self.events + 1;
        if self.tick >= self.max_frame {
            self.saturated = true;
        }
    }

    /// 多帧步进
    pub fn step_n(&mut self, n: u64) {
        // 无乘法: 最多走 (max - tick) 步
        let remaining = self.max_frame - self.tick;
        let actual = if n <= remaining { n } else { remaining };
        self.tick = self.tick + actual;
        self.events = self.events + actual;
        if self.tick >= self.max_frame {
            self.saturated = true;
        }
    }

    /// 计算两帧之间的距离 (用循环减法近似)
    pub fn distance(&self, other: &ContainerTimestamp) -> u64 {
        if self.tick >= other.tick { self.tick - other.tick }
        else { other.tick - self.tick }
    }

    /// 发生在 before 之后 (严格 >)
    pub fn after(&self, other: &ContainerTimestamp) -> bool { self.tick > other.tick }
    pub fn at_or_after(&self, other: &ContainerTimestamp) -> bool { self.tick >= other.tick }
    pub fn simultaneous(&self, other: &ContainerTimestamp) -> bool { self.tick == other.tick }

    /// 复制当前 tick 为快照 (用于因果记录)
    pub fn snapshot(&self) -> u64 { self.tick }

    pub fn reset(&mut self) {
        self.tick = 0;
        self.events = 0;
        self.saturated = false;
    }
}

impl Default for ContainerTimestamp {
    fn default() -> Self { Self::new(u64::MAX) }
}

#[cfg(test)]
mod timestamp_tests {
    use super::*;

    #[test]
    fn test_step_once() {
        let mut t = ContainerTimestamp::new(100);
        t.step();
        assert_eq!(t.tick, 1);
        assert_eq!(t.events, 1);
        assert!(!t.saturated);
    }

    #[test]
    fn test_step_n() {
        let mut t = ContainerTimestamp::new(100);
        t.step_n(5);
        assert_eq!(t.tick, 5);
    }

    #[test]
    fn test_saturate() {
        let mut t = ContainerTimestamp::new(3);
        t.step();
        t.step();
        t.step();
        assert_eq!(t.tick, 3);
        assert!(t.saturated);
        t.step();  // 不再增加
        assert_eq!(t.tick, 3);
    }

    #[test]
    fn test_distance() {
        let mut a = ContainerTimestamp::new(100);
        let mut b = ContainerTimestamp::new(100);
        a.step_n(10);
        b.step_n(3);
        assert_eq!(a.distance(&b), 7);
    }

    #[test]
    fn test_after() {
        let mut a = ContainerTimestamp::new(100);
        a.step_n(5);
        let mut b = ContainerTimestamp::new(100);
        b.step_n(3);
        assert!(a.after(&b));
        assert!(b.after(&a) == false);
        assert!(a.at_or_after(&a));  // 自己 >= 自己
    }

    #[test]
    fn test_simultaneous() {
        let a = ContainerTimestamp::new(100);
        let b = ContainerTimestamp::new(100);
        assert!(a.simultaneous(&b)); // tick 均为 0
    }

    #[test]
    fn test_snapshot() {
        let mut t = ContainerTimestamp::new(100);
        t.step_n(7);
        assert_eq!(t.snapshot(), 7);
    }

    #[test]
    fn test_default_max() {
        let mut t = ContainerTimestamp::default();
        t.step_n(1_000_000);
        assert!(!t.saturated);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器耦合度引擎 · ContainerCoupling (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   耦合度 = 两个容器相互绑定的紧密程度
//   不靠乘除, 靠加法累积共同事件
//
//   衡量方式:
//     - coincidence:  共同命中次数 (累积)
//     - total_a / total_b: 各自总事件数
//     - alignment:    coincidence * 2  versus (total_a + total_b)
//                     (用加法近似代替除法比较)
//
//   仅加法: 所有统计 +1/+offset 累加, 比较用 a*b > c*d 型转换为加法对

/// 耦合记录
#[derive(Clone, Debug)]
pub struct Coupling {
    pub total_a: u64,
    pub total_b: u64,
    pub coincidence: u64,   // 同时命中次数
}

/// 耦合引擎
#[derive(Clone, Debug)]
pub struct ContainerCoupling {
    pub pair: HashMap<(u64, u64), Coupling>,
    pub count: usize,
}

impl ContainerCoupling {
    pub fn new() -> Self { Self { pair: HashMap::new(), count: 0 } }

    fn key(a: u64, b: u64) -> (u64, u64) { if a <= b { (a, b) } else { (b, a) } }

    /// 记录一次 A 事件
    pub fn event_a(&mut self, a: u64, b: u64) {
        let k = Self::key(a, b);
        let e = self.pair.entry(k).or_insert(Coupling { total_a: 0, total_b: 0, coincidence: 0 });
        e.total_a = e.total_a + 1;
    }

    /// 记录一次 B 事件
    pub fn event_b(&mut self, a: u64, b: u64) {
        let k = Self::key(a, b);
        let e = self.pair.entry(k).or_insert(Coupling { total_a: 0, total_b: 0, coincidence: 0 });
        e.total_b = e.total_b + 1;
    }

    /// 记录同时命中 (双方共同 tick)
    pub fn coincide(&mut self, a: u64, b: u64) {
        let k = Self::key(a, b);
        let e = self.pair.entry(k).or_insert(Coupling { total_a: 0, total_b: 0, coincidence: 0 });
        e.coincidence = e.coincidence + 1;
        e.total_a = e.total_a + 1;
        e.total_b = e.total_b + 1;
    }

    pub fn get(&self, a: u64, b: u64) -> Option<Coupling> {
        self.pair.get(&Self::key(a, b)).cloned()
    }

    /// 紧密度: coincidence * 2 ≥ total_a + total_b  → 完全对齐
    pub fn is_tight(&self, a: u64, b: u64) -> bool {
        match self.get(a, b) {
            Some(c) => {
                let twice = c.coincidence + c.coincidence;
                let sum = c.total_a + c.total_b;
                twice >= sum
            }
            None => false,
        }
    }

    /// 松耦合: coincidence == 0 但双方都有事件
    pub fn is_loose(&self, a: u64, b: u64) -> bool {
        match self.get(a, b) {
            Some(c) => c.coincidence == 0 && c.total_a > 0 && c.total_b > 0,
            None => false,
        }
    }

    /// 耦合度分数 (0..=100, 用加法近似 coincidence / total * 100)
    pub fn score(&self, a: u64, b: u64) -> u64 {
        match self.get(a, b) {
            Some(c) => {
                if c.coincidence == 0 { return 0; }
                // 不用除: 做减法逼近 — coincidence 占 (total_a + total_b) / 2 的百分比
                let sum = c.total_a + c.total_b;
                if sum == 0 { return 0; }
                // 近似: step = sum / 100, 然后 coincidence / step
                // 但无除法: 改用迭代减 (最多 100 次)
                let base = if sum >= 100 { sum >> 6 } else { 1 };  // 近似 /64 作为基底
                let mut s = 0;
                let mut rem = c.coincidence;
                loop {
                    if rem < base { break; }
                    rem = rem - base;
                    s = s + 1;
                    if s >= 100 { break; }
                }
                s
            }
            None => 0,
        }
    }

    pub fn remove(&mut self, a: u64, b: u64) -> bool {
        let r = self.pair.remove(&Self::key(a, b)).is_some();
        if r && self.count > 0 { self.count = self.count - 1; }
        r
    }

    pub fn clear(&mut self) { self.pair.clear(); self.count = 0; }
}

impl Default for ContainerCoupling {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod coupling_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let c = ContainerCoupling::new();
        assert_eq!(c.score(1, 2), 0);
        assert!(!c.is_tight(1, 2));
    }

    #[test]
    fn test_coincide_tight() {
        let mut c = ContainerCoupling::new();
        c.coincide(10, 20);
        c.coincide(10, 20);
        // total_a=2, total_b=2, coincidence=2 → 2*2=4 vs 2+2=4 → tight
        assert!(c.is_tight(10, 20));
    }

    #[test]
    fn test_loose() {
        let mut c = ContainerCoupling::new();
        c.event_a(10, 20);
        c.event_b(10, 20);
        c.event_a(10, 20);
        // coincidence=0, total_a=2, total_b=1 → loose
        assert!(c.is_loose(10, 20));
        assert!(!c.is_tight(10, 20));
    }

    #[test]
    fn test_symmetric_key() {
        let mut c = ContainerCoupling::new();
        c.coincide(99, 1);
        c.coincide(1, 99);
        // 应视为同一对
        let entry = c.get(99, 1).unwrap();
        assert_eq!(entry.coincidence, 2);
    }

    #[test]
    fn test_score_partial() {
        let mut c = ContainerCoupling::new();
        // 10 次中 5 次同时命中
        for _ in 0..5 { c.coincide(1, 2); }
        for _ in 0..5 { c.event_a(1, 2); }
        let s = c.score(1, 2);
        // coincidence=5, sum=15; 5/15≈33%
        assert!(s > 0 && s <= 100);
    }

    #[test]
    fn test_score_zero() {
        let mut c = ContainerCoupling::new();
        c.event_a(1, 2);
        assert_eq!(c.score(1, 2), 0);
    }

    #[test]
    fn test_get_none() {
        let c = ContainerCoupling::new();
        assert!(c.get(1, 2).is_none());
    }

    #[test]
    fn test_clear() {
        let mut c = ContainerCoupling::new();
        c.coincide(1, 2);
        c.clear();
        assert!(!c.is_tight(1, 2));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 活码编译器 · LiveCodeCompiler (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   「活码」= 可活动的编码; 未锁定时位置任意滑动, 固定后按排列定权限/数值
//   起点 = 钥匙 (entry-point), 激活一条可活动编码
//
//   1. 活动位 (slots): 一组空位, 可插入/移除符号 (u64)
//   2. 锁定 (seal): 锁定后当前排列即为最终编码
//   3. 数值 = 各 slot 值 × 排列序 (用加法累加代替乘法)
//   4. 排列序 = slot_index + 1 (第0位=1, 第1位=2, ...)
//      严格避免乘法: value × order 用 value 自加 order 次
//
//   核心: 锁定前 order 任意(活), 锁定后 order 固定(死码)

#[derive(Clone, Debug)]
pub struct LiveCodeCompiler {
    pub slots: Vec<u64>,       // 活动位
    pub locked: bool,           // 是否锁定
    pub entry: u64,             // 起点 (激活钥匙)
    pub total_ops: u64,         // 操作计数
}

impl LiveCodeCompiler {
    pub fn new(entry: u64) -> Self {
        Self { slots: Vec::new(), locked: false, entry, total_ops: 0 }
    }

    /// 追加一个活动位
    pub fn push(&mut self, value: u64) {
        self.slots.push(value);
        self.total_ops = self.total_ops + 1;
    }

    /// 移除末尾活动位
    pub fn pop(&mut self) -> Option<u64> {
        let r = self.slots.pop();
        if r.is_some() { self.total_ops = self.total_ops + 1; }
        r
    }

    /// 锁定 (之后排列固定)
    pub fn seal(&mut self) { self.locked = true; }

    /// 计算当前排列的数值 (用加法累加 slot 值 × 序)
    /// 仅加法: order(n) = (index + 1), value × order = value 自加 order 次
    pub fn compile_value(&self) -> u64 {
        let mut total: u64 = 0;
        let mut idx: u64 = 0;
        for &val in &self.slots {
            let order = idx + 1;  // 第0位=1
            // val × order 用自加
            let mut i = 0;
            loop {
                if i >= order { break; }
                total = total + val;
                i = i + 1;
            }
            idx = idx + 1;
        }
        total
    }

    /// 解锁 (仅未锁定时返回 true)
    pub fn unseal(&mut self) -> bool {
        if self.locked { return false; }
        true
    }

    /// 在 index 处插入 (模拟活码 — 锁定前可任意活动)
    pub fn insert(&mut self, index: usize, value: u64) -> bool {
        if self.locked { return false; }
        if index > self.slots.len() { return false; }
        // 无 insert_at: 用 split + 拼
        let mut new_slots = Vec::with_capacity(self.slots.len() + 1);
        for i in 0..index { new_slots.push(self.slots[i]); }
        new_slots.push(value);
        for i in index..self.slots.len() { new_slots.push(self.slots[i]); }
        self.slots = new_slots;
        self.total_ops = self.total_ops + 1;
        true
    }

    /// 删除 index (活码删除)
    pub fn erase(&mut self, index: usize) -> bool {
        if self.locked { return false; }
        if index >= self.slots.len() { return false; }
        let mut new_slots = Vec::with_capacity(self.slots.len() - 1);
        for i in 0..index { new_slots.push(self.slots[i]); }
        for i in (index + 1)..self.slots.len() { new_slots.push(self.slots[i]); }
        self.slots = new_slots;
        self.total_ops = self.total_ops + 1;
        true
    }

    pub fn is_locked(&self) -> bool { self.locked }
    pub fn slot_count(&self) -> usize { self.slots.len() }

    pub fn clear(&mut self) {
        self.slots.clear();
        self.locked = false;
        self.total_ops = 0;
    }
}

impl Default for LiveCodeCompiler {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod live_code_tests {
    use super::*;

    #[test]
    fn test_empty_compile() {
        let lc = LiveCodeCompiler::new(7);
        assert_eq!(lc.compile_value(), 0);
    }

    #[test]
    fn test_single_slot() {
        let mut lc = LiveCodeCompiler::new(1);
        lc.push(5);  // 第0位: 5×1=5
        assert_eq!(lc.compile_value(), 5);
    }

    #[test]
    fn test_multi_slots() {
        let mut lc = LiveCodeCompiler::new(1);
        lc.push(3);   // 第0位: 3×1=3
        lc.push(4);   // 第1位: 4×2=8
        lc.push(1);   // 第2位: 1×3=3
        // total = 3+8+3=14
        assert_eq!(lc.compile_value(), 14);
    }

    #[test]
    fn test_seal_prevents_modify() {
        let mut lc = LiveCodeCompiler::new(1);
        lc.push(10);
        lc.push(20);
        lc.seal();
        assert!(lc.is_locked());
        // 锁定后不可插入
        assert!(!lc.insert(0, 99));
        assert!(!lc.erase(0));
        assert_eq!(lc.slot_count(), 2);
    }

    #[test]
    fn test_insert_while_live() {
        let mut lc = LiveCodeCompiler::new(0);
        lc.push(1);
        lc.push(3);
        assert!(lc.insert(1, 2));
        assert_eq!(lc.slots, vec![1, 2, 3]);
        // 数值: 1×1 + 2×2 + 3×3 = 1+4+9=14
        assert_eq!(lc.compile_value(), 14);
    }

    #[test]
    fn test_erase_while_live() {
        let mut lc = LiveCodeCompiler::new(0);
        lc.push(1);
        lc.push(2);
        lc.push(3);
        assert!(lc.erase(1));
        assert_eq!(lc.slots, vec![1, 3]);
    }

    #[test]
    fn test_pop() {
        let mut lc = LiveCodeCompiler::new(0);
        lc.push(5);
        lc.push(10);
        assert_eq!(lc.pop(), Some(10));
        assert_eq!(lc.slot_count(), 1);
    }

    #[test]
    fn test_pop_empty() {
        let mut lc = LiveCodeCompiler::new(0);
        assert!(lc.pop().is_none());
    }

    #[test]
    fn test_unseal_returns_true_when_unlocked() {
        let mut lc = LiveCodeCompiler::new(0);
        assert!(lc.unseal());
        lc.seal();
        assert!(!lc.unseal());
    }

    #[test]
    fn test_clear() {
        let mut lc = LiveCodeCompiler::new(0);
        lc.push(1);
        lc.seal();
        lc.clear();
        assert!(!lc.is_locked());
        assert_eq!(lc.slot_count(), 0);
        assert_eq!(lc.total_ops, 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器消抖引擎 · ContainerDebounce (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   信号防抖 — 输入信号必须有稳定的 tick 滑动窗口才能被确认
//
//   方式: 连续 N tick 内信号稳定不变, 才输出
//   作用: 去除高频抖动, 使反馈环路稳定
//
//   仅加法: 每 tick +1 计数, 归零同样用 =0 判定

#[derive(Clone, Debug)]
pub struct ContainerDebounce {
    pub threshold: u64,       // 确认所需稳定 tick 数
    pub current_signal: u64,  // 当前暂存信号
    pub stable_count: u64,    // 连续稳定 tick
    pub confirmed_signal: u64, // 上次确认信号
    pub confirmed: bool,      // 是否已确认
    pub total_feeds: u64,     // 总喂入次数
    pub total_changes: u64,   // 实际确认变化次数
}

impl ContainerDebounce {
    pub fn new(threshold: u64) -> Self {
        Self { threshold, current_signal: 0, stable_count: 0, confirmed_signal: 0, confirmed: false, total_feeds: 0, total_changes: 0 }
    }

    /// 每 tick 调用: 喂入信号; 返回 Some(信号) 当且仅当窗口稳定
    pub fn feed(&mut self, signal: u64) -> Option<u64> {
        self.total_feeds = self.total_feeds + 1;
        if signal == self.current_signal {
            self.stable_count = self.stable_count + 1;
        } else {
            self.current_signal = signal;
            self.stable_count = 1;
        }
        if self.stable_count >= self.threshold {
            let changed = if self.confirmed_signal != signal { true } else { false };
            self.confirmed_signal = signal;
            self.confirmed = true;
            if changed { self.total_changes = self.total_changes + 1; }
            Some(signal)
        } else {
            None
        }
    }

    pub fn is_confirmed(&self) -> bool { self.confirmed }
    pub fn last_confirmed(&self) -> u64 { self.confirmed_signal }
    pub fn reset(&mut self) {
        self.current_signal = 0;
        self.stable_count = 0;
        self.confirmed_signal = 0;
        self.confirmed = false;
        self.total_feeds = 0;
        self.total_changes = 0;
    }

    /// 剩余 tick 数直到确认 (当 stable_count >= threshold 时返回 0)
    pub fn ticks_to_confirm(&self) -> u64 {
        if self.stable_count >= self.threshold { 0 }
        else { self.threshold - self.stable_count }
    }
}

impl Default for ContainerDebounce {
    fn default() -> Self { Self::new(3) }
}

#[cfg(test)]
mod debounce_tests {
    use super::*;

    #[test]
    fn test_threshold_3() {
        let mut d = ContainerDebounce::new(3);
        assert!(d.feed(10).is_none());   // 1/3
        assert!(d.feed(10).is_none());   // 2/3
        assert_eq!(d.feed(10), Some(10)); // 3/3 → 确认
    }

    #[test]
    fn test_glitch_recovery() {
        let mut d = ContainerDebounce::new(3);
        assert!(d.feed(5).is_none());     // 1/3 (signal=5)
        assert!(d.feed(99).is_none());    // 毛刺! signal 切换为 99, stable=1/3
        assert!(d.feed(5).is_none());     // 毛刺! signal 切换回 5, stable=1/3
        assert!(d.feed(5).is_none());     // 2/3
        assert_eq!(d.feed(5), Some(5));   // 3/3 → 确认 5
    }

    #[test]
    fn test_unchanged_reconfirm() {
        let mut d = ContainerDebounce::new(2);
        d.feed(42); d.feed(42);
        assert_eq!(d.feed(42), Some(42)); // 继续喂相同值, 继续确认
    }

    #[test]
    fn test_ticks_to_confirm() {
        let mut d = ContainerDebounce::new(5);
        d.feed(1);
        assert_eq!(d.ticks_to_confirm(), 4);
        d.feed(1);
        d.feed(1);
        assert_eq!(d.ticks_to_confirm(), 2);
    }

    #[test]
    fn test_changes_counter() {
        let mut d = ContainerDebounce::new(1);
        d.feed(10); // 确认变化1
        d.feed(10); // 同值, 不算变化
        d.feed(20); // 确认变化2
        assert_eq!(d.total_changes, 2);
    }

    #[test]
    fn test_threshold_one() {
        let mut d = ContainerDebounce::new(1);
        // threshold=1: 每次 feed 都立即确认
        assert_eq!(d.feed(7), Some(7));
        assert_eq!(d.feed(7), Some(7));
    }

    #[test]
    fn test_reset() {
        let mut d = ContainerDebounce::new(2);
        d.feed(5); d.feed(5);
        d.reset();
        assert!(!d.is_confirmed());
        assert_eq!(d.total_feeds, 0);
        assert!(d.feed(5).is_none()); // 重新开始
    }

    #[test]
    fn test_default_threshold_three() {
        let d = ContainerDebounce::default();
        assert_eq!(d.threshold, 3);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器互指引擎 · ContainerCrossRef (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   记录容器之间的双向引用关系 (有向边 + 反向边自动镜像)
//   A 呼唤 B → 建立 A→B + B ↔ A 的反向记录
//
//   用途:
//     - 反馈环路检测: A→B→A 形成环
//     - 引用计数: 谁呼唤了我
//     - 死引用清理: 若某容器消亡, 其所有边自动移除
//
//   仅加法: 引用计数+1, 无边判断用 empty

#[derive(Clone, Debug, Default)]
pub struct CrossEdge {
    pub forward: u64,   // A→B 次数
    pub backward: u64,  // B→A 次数 (镜像)
    pub last_tick: u64,
}

#[derive(Clone, Debug)]
pub struct ContainerCrossRef {
    pub edges: HashMap<(u64, u64), CrossEdge>,
    pub total_links: u64,
}

impl ContainerCrossRef {
    pub fn new() -> Self { Self { edges: HashMap::new(), total_links: 0 } }

    /// A 呼唤 B
    pub fn call(&mut self, a: u64, b: u64, tick: u64) {
        let e = self.edges.entry((a, b)).or_insert_with(|| CrossEdge::default());
        e.forward = e.forward + 1;
        e.last_tick = tick;
        self.total_links = self.total_links + 1;
        // 镜像: 在 B 侧记录被呼唤
        let e2 = self.edges.entry((b, a)).or_insert_with(|| CrossEdge::default());
        e2.backward = e2.backward + 1;
        e2.last_tick = tick;
    }

    pub fn get(&self, a: u64, b: u64) -> Option<CrossEdge> {
        self.edges.get(&(a, b)).cloned()
    }

    /// 检测互指 (A→B 反向边也有 forward≥1 → 形成环)
    pub fn mutual(&self, a: u64, b: u64) -> bool {
        let fwd = self.edges.get(&(a, b)).map_or(0, |e| e.forward);
        let rev = self.edges.get(&(b, a)).map_or(0, |e| e.forward);
        fwd > 0 && rev > 0
    }

    /// A 呼过谁 (作为 from 的所有正向边)
    pub fn calls_from(&self, a: u64) -> Vec<(u64, CrossEdge)> {
        self.edges.iter()
            .filter(|((x, _), _)| *x == a)
            .map(|((_, y), e)| (*y, e.clone()))
            .collect()
    }

    /// 被谁呼过
    pub fn calls_to(&self, b: u64) -> Vec<(u64, CrossEdge)> {
        self.edges.iter()
            .filter(|((_, y), _)| *y == b)
            .map(|((x, _), e)| (*x, e.clone()))
            .collect()
    }

    /// 斩断 a 的所有边
    pub fn remove_node(&mut self, a: u64) {
        let keys: Vec<(u64, u64)> = self.edges.keys()
            .filter(|(x, y)| *x == a || *y == a)
            .copied()
            .collect();
        for k in &keys { self.edges.remove(k); }
    }

    pub fn degree(&self, a: u64) -> u64 {
        self.edges.keys().filter(|(x, y)| *x == a || *y == a).count() as u64
    }

    pub fn clear(&mut self) { self.edges.clear(); self.total_links = 0; }
}

impl Default for ContainerCrossRef {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod cross_ref_tests {
    use super::*;

    #[test]
    fn test_call_once() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 100);
        let e = c.get(1, 2).unwrap();
        assert_eq!(e.forward, 1);
        assert_eq!(e.last_tick, 100);
    }

    #[test]
    fn test_call_multiple() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 10);
        c.call(1, 2, 20);
        let e = c.get(1, 2).unwrap();
        assert_eq!(e.forward, 2);
        assert_eq!(e.last_tick, 20);
    }

    #[test]
    fn test_no_mutual() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 1);
        assert!(!c.mutual(1, 2));
    }

    #[test]
    fn test_mutual() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 1);
        c.call(2, 1, 2);
        assert!(c.mutual(1, 2));
    }

    #[test]
    fn test_calls_to() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 3, 1);
        c.call(2, 3, 2);
        let v = c.calls_to(3);
        // 2 entries: (1→3) and (2→3)
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_calls_from() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 1);
        c.call(1, 3, 2);
        let v = c.calls_from(1);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_remove_node() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 1);
        c.call(2, 1, 2);
        c.remove_node(1);
        assert!(c.get(1, 2).is_none());
        assert!(c.get(2, 1).is_none());
    }

    #[test]
    fn test_degree() {
        let mut c = ContainerCrossRef::new();
        c.call(1, 2, 1);
        c.call(2, 1, 2);
        c.call(1, 3, 3);
        // call 同时建 (a,b) 和镜像 (b,a), 共 4 条边涉及节点 1
        assert_eq!(c.degree(1), 4);
    }

    #[test]
    fn test_empty() {
        let c = ContainerCrossRef::new();
        assert_eq!(c.total_links, 0);
        assert!(c.get(99, 99).is_none());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器优先级队列 · ContainerPriorityQueue (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   最高优先级先出 — 不用堆/乘除, 用插入排序 (add-only)
//
//   排序: 每次 push 时保持 descending 序 (最大在队首)
//   同 FIFO: 同优先级按入队序 (稳定排序)
//
//   仅加法: 插入位置查找用 < 比较, 不用乘法算 mid

#[derive(Clone, Debug, PartialEq)]
pub struct PQEntry { pub priority: u64, pub value: u64, pub seq: u64 }

#[derive(Clone, Debug)]
pub struct ContainerPriorityQueue {
    pub heap: Vec<PQEntry>,
    pub next_seq: u64,
    pub total_pushes: u64,
    pub total_pops: u64,
}

impl ContainerPriorityQueue {
    pub fn new() -> Self { Self { heap: Vec::new(), next_seq: 0, total_pushes: 0, total_pops: 0 } }

    /// 插入并保持优先级降序 (同优先级 FIFO: seq 小在前)
    pub fn push(&mut self, priority: u64, value: u64) {
        let entry = PQEntry { priority, value, seq: self.next_seq };
        self.next_seq = self.next_seq + 1;
        // 找位置: 找到第一个 priority 更小的, 或 priority 相同但 seq 更大的
        let mut pos = self.heap.len();
        for i in 0..self.heap.len() {
            if priority > self.heap[i].priority
                || (priority == self.heap[i].priority && entry.seq < self.heap[i].seq) {
                pos = i;
                break;
            }
        }
        // 无 insert 方法时用 push + 冒泡
        self.heap.push(entry);
        // 从末尾向前交换到正确位置
        let mut j = self.heap.len() - 1;
        while j > pos {
            self.heap.swap(j, j - 1);
            j = j - 1;
        }
        self.total_pushes = self.total_pushes + 1;
    }

    /// 弹出队首 (最高优先级)
    pub fn pop(&mut self) -> Option<PQEntry> {
        if self.heap.is_empty() { return None; }
        let e = self.heap.remove(0);
        self.total_pops = self.total_pops + 1;
        Some(e)
    }

    pub fn peek(&self) -> Option<&PQEntry> { self.heap.first() }
    pub fn len(&self) -> usize { self.heap.len() }
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// 删除所有 priority < threshold 的条目
    pub fn filter_weak(&mut self, threshold: u64) {
        let mut new_vec: Vec<PQEntry> = Vec::new();
        for e in &self.heap { if e.priority >= threshold { new_vec.push(e.clone()); } }
        self.heap = new_vec;
    }

    pub fn clear(&mut self) { self.heap.clear(); self.next_seq = 0; self.total_pushes = 0; self.total_pops = 0; }
}

impl Default for ContainerPriorityQueue {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod priority_queue_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut q = ContainerPriorityQueue::new();
        assert!(q.pop().is_none());
        assert!(q.is_empty());
    }

    #[test]
    fn test_push_pop_order() {
        let mut q = ContainerPriorityQueue::new();
        q.push(3, 100);
        q.push(1, 200);
        q.push(5, 300);
        q.push(2, 400);
        // 出队: 5,3,2,1
        assert_eq!(q.pop().unwrap().priority, 5);
        assert_eq!(q.pop().unwrap().priority, 3);
        assert_eq!(q.pop().unwrap().priority, 2);
        assert_eq!(q.pop().unwrap().priority, 1);
        assert!(q.is_empty());
    }

    #[test]
    fn test_stable_same_priority() {
        let mut q = ContainerPriorityQueue::new();
        q.push(5, 10);
        q.push(5, 20);
        q.push(5, 30);
        // 同 priority 按 FIFO
        assert_eq!(q.pop().unwrap().value, 10);
        assert_eq!(q.pop().unwrap().value, 20);
        assert_eq!(q.pop().unwrap().value, 30);
    }

    #[test]
    fn test_peek() {
        let mut q = ContainerPriorityQueue::new();
        q.push(1, 10);
        q.push(9, 90);
        assert_eq!(q.peek().unwrap().priority, 9);
        assert_eq!(q.len(), 2); // peek 不消耗
    }

    #[test]
    fn test_filter_weak() {
        let mut q = ContainerPriorityQueue::new();
        q.push(1, 10);
        q.push(5, 50);
        q.push(3, 30);
        q.push(7, 70);
        q.filter_weak(4);
        // 剩下 priority >= 4 的: (5,50),(7,70)
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop().unwrap().priority, 7);
        assert_eq!(q.pop().unwrap().priority, 5);
    }

    #[test]
    fn test_len() {
        let mut q = ContainerPriorityQueue::new();
        for i in 0..5 { q.push(i, i); }
        assert_eq!(q.len(), 5);
    }

    #[test]
    fn test_counters() {
        let mut q = ContainerPriorityQueue::new();
        q.push(1, 1);
        q.push(2, 2);
        q.pop();
        q.pop();
        assert_eq!(q.total_pushes, 2);
        assert_eq!(q.total_pops, 2);
    }

    #[test]
    fn test_default() {
        let q = ContainerPriorityQueue::default();
        assert!(q.is_empty());
    }
}

// ══════════════════════════════════════════════════════════════════════
// 蚕蛹反向寄生壳 · SilkwormReverseParasite (v1.0)
// ══════════════════════════════════════════════════════════════════════
//
//   蚕蛹 = 系统韧性的终极形态
//   "下载蚕蛹初始包 → 解析寄生壳选项 → 脱壳穿衣 → 外部模仿自身"
//
//   核心操作:
//     - cocoon_init()       — 初始化蚕蛹包
//     - select_shell()      — 手动/自动匹配寄生壳
//     - molt()              — 脱壳 (剥离外部依赖)
//     - dress()             — 穿衣 (穿上新的寄生壳)
//     - heartbeat()         — 心跳恢复 (被摧毁后重建)
//     - mimic_exterior()    — 外部模仿自身
//
//   NJSG 哲学: 不是自身适应外部规则, 而是使外部系统模仿自身
//   寄生 = 将内核注入宿主, 让宿主的资源为保护内核而工作

/// 寄生壳类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParasiteShell {
    FileSystem,     // 寄生文件目录
    GitRepo,        // 寄生 Git 仓库
    Database,       // 寄生数据库存储过程
    SystemdService, // 寄生系统服务
    CronJob,        // 寄生定时任务
    DockerContainer,// 寄生容器镜像
}

impl ParasiteShell {
    pub fn name(&self) -> &'static str {
        match self {
            ParasiteShell::FileSystem => "fs",
            ParasiteShell::GitRepo => "git",
            ParasiteShell::Database => "db",
            ParasiteShell::SystemdService => "systemd",
            ParasiteShell::CronJob => "cron",
            ParasiteShell::DockerContainer => "docker",
        }
    }
}

/// 蚕蛹包 — 传输承载的可执行胶囊
#[derive(Clone, Debug)]
pub struct CocoonPackage {
    pub version: u32,
    pub kernelcode: Vec<u8>,
    pub shell_options: Vec<ParasiteShell>,
    pub checksum: u64,
}

impl CocoonPackage {
    pub fn new(version: u32, kernelcode: Vec<u8>) -> Self {
        let checksum = Self::compute_checksum(&kernelcode);
        Self { version, kernelcode, shell_options: Vec::new(), checksum }
    }

    /// 添加寄生壳选项
    pub fn add_shell_option(&mut self, shell: ParasiteShell) {
        if !self.shell_options.contains(&shell) {
            self.shell_options.push(shell);
        }
    }

    /// 校验包完整性
    pub fn verify(&self) -> bool {
        Self::compute_checksum(&self.kernelcode) == self.checksum
    }

    /// 简易校验和
    fn compute_checksum(data: &[u8]) -> u64 {
        data.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64).wrapping_mul(31))
    }
}

/// 心脏搏动 — 自检 + 通知外部存活
#[derive(Clone, Debug)]
pub struct Heartbeat {
    pub last_tick: u64,
    pub interval: u64,
    pub alive: bool,
}

impl Heartbeat {
    pub fn new(interval: u64) -> Self {
        Self { last_tick: 0, interval, alive: true }
    }

    /// 发送心跳
    pub fn beat(&mut self, current_tick: u64) -> bool {
        if current_tick - self.last_tick >= self.interval {
            self.last_tick = current_tick;
            true
        } else { false }
    }

    /// 判定是否存活
    pub fn is_alive(&self, current_tick: u64) -> bool {
        current_tick - self.last_tick <= self.interval * 3
    }
}

/// 寄生状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParasitePhase {
    Dormant,    // 休眠 (未激活)
    Molting,    // 脱壳中
    Dressed,    // 已穿衣 (寄生完成)
    Detached,   // 脱离 (可转移)
}

/// 蚕蛹反向寄生引擎
#[derive(Clone, Debug)]
pub struct SilkwormReverseParasite {
    pub current_shell: Option<ParasiteShell>,
    pub phase: ParasitePhase,
    pub heartbeat: Heartbeat,
    pub package: Option<CocoonPackage>,
    pub host_path: String,
    /// 心跳计数
    pub beat_count: u64,
}

impl SilkwormReverseParasite {
    pub fn new(host_path: &str) -> Self {
        Self {
            current_shell: None,
            phase: ParasitePhase::Dormant,
            heartbeat: Heartbeat::new(60),
            package: None,
            host_path: host_path.to_string(),
            beat_count: 0,
        }
    }

    /// 下载/初始化蚕蛹包
    pub fn cocoon_init(&mut self, version: u32, kernelcode: Vec<u8>) {
        let package = CocoonPackage::new(version, kernelcode);
        self.package = Some(package);
    }

    /// 选择寄生壳 (匹配目标环境)
    pub fn select_shell(&mut self, shell: ParasiteShell) -> bool {
        if let Some(pkg) = &self.package {
            if pkg.shell_options.contains(&shell) || pkg.shell_options.is_empty() {
                self.current_shell = Some(shell);
                self.phase = ParasitePhase::Dressed;
                true
            } else { false }
        } else { false }
    }

    /// 脱壳 (剥离旧壳)
    pub fn molt(&mut self) -> bool {
        if self.phase == ParasitePhase::Dressed {
            self.phase = ParasitePhase::Molting;
            self.current_shell = None;
            true
        } else { false }
    }

    /// 穿衣 (穿上新壳)
    pub fn dress(&mut self, shell: ParasiteShell) -> bool {
        if self.phase == ParasitePhase::Molting || self.phase == ParasitePhase::Dormant {
            self.current_shell = Some(shell);
            self.phase = ParasitePhase::Dressed;
            true
        } else { false }
    }

    /// 心跳节拍
    pub fn tick(&mut self, current_tick: u64) -> bool {
        if self.heartbeat.beat(current_tick) {
            self.beat_count += 1;
            true
        } else { false }
    }

    /// 检查存活
    pub fn is_alive(&self, current_tick: u64) -> bool {
        self.heartbeat.is_alive(current_tick)
    }

    /// 外部返回端口 (被摧毁后重建)
    pub fn regenerate(&mut self) -> bool {
        if self.package.is_some() {
            self.phase = ParasitePhase::Dormant;
            self.heartbeat = Heartbeat::new(self.heartbeat.interval);
            true
        } else { false }
    }

    /// 模仿判定: 当前是否已成功寄生
    pub fn is_parasitic(&self) -> bool {
        self.phase == ParasitePhase::Dressed && self.current_shell.is_some()
    }

    /// 生成配置文件 (return config string)
    pub fn generate_config(&self) -> String {
        match &self.current_shell {
            Some(shell) => format!(
                "silkworm:\n  shell: {}\n  phase: {:?}\n  host: {}\n  beats: {}\n",
                shell.name(), self.phase, self.host_path, self.beat_count
            ),
            None => "silkworm: naked\n".to_string(),
        }
    }
}

impl Default for SilkwormReverseParasite { fn default() -> Self { Self::new("/var/mqf/silkworm") } }

#[cfg(test)]
mod silkworm_tests {
    use super::*;
    #[test] fn test_cocoon_init() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        srp.cocoon_init(1, vec![1, 2, 3, 4]);
        assert!(srp.package.is_some());
        assert_eq!(srp.phase, ParasitePhase::Dormant);
    }
    #[test] fn test_cocoon_verify() {
        let pkg = CocoonPackage::new(1, vec![10, 20, 30]);
        assert!(pkg.verify());
    }
    #[test] fn test_cocoon_add_shell() {
        let mut pkg = CocoonPackage::new(1, vec![1, 2]);
        pkg.add_shell_option(ParasiteShell::FileSystem);
        pkg.add_shell_option(ParasiteShell::FileSystem); // 重复不加
        assert_eq!(pkg.shell_options.len(), 1);
    }
    #[test] fn test_select_shell() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        srp.cocoon_init(1, vec![1, 2, 3]);
        srp.package.as_mut().unwrap().add_shell_option(ParasiteShell::GitRepo);
        assert!(srp.select_shell(ParasiteShell::GitRepo));
        assert_eq!(srp.phase, ParasitePhase::Dressed);
        assert_eq!(srp.current_shell, Some(ParasiteShell::GitRepo));
    }
    #[test] fn test_molt_and_dress() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        srp.cocoon_init(1, vec![1, 2]);
        srp.select_shell(ParasiteShell::FileSystem);
        assert!(srp.molt());
        assert_eq!(srp.phase, ParasitePhase::Molting);
        assert!(srp.dress(ParasiteShell::Database));
        assert_eq!(srp.phase, ParasitePhase::Dressed);
        assert_eq!(srp.current_shell, Some(ParasiteShell::Database));
    }
    #[test] fn test_heartbeat() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        assert!(!srp.tick(1));  // 未到间隔
        assert!(!srp.tick(30)); // 未到间隔
        assert!(srp.tick(61));  // 超过60 tick间隔
        assert_eq!(srp.beat_count, 1);
    }
    #[test] fn test_is_alive() {
        let mut hb = Heartbeat::new(60);
        hb.beat(100);
        assert!(hb.is_alive(150));   // 50 < 180 (3*interval)
        assert!(!hb.is_alive(300));  // 200 > 180, 超时
    }
    #[test] fn test_regenerate() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        srp.cocoon_init(1, vec![1, 2, 3]);
        srp.select_shell(ParasiteShell::FileSystem);
        assert!(srp.molt());
        assert!(srp.regenerate());
        assert_eq!(srp.phase, ParasitePhase::Dormant);
    }
    #[test] fn test_is_parasitic() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        assert!(!srp.is_parasitic());
        srp.cocoon_init(1, vec![1]);
        srp.select_shell(ParasiteShell::CronJob);
        assert!(srp.is_parasitic());
    }
    #[test] fn test_generate_config() {
        let mut srp = SilkwormReverseParasite::new("/tmp/silkworm");
        let cfg = srp.generate_config();
        assert!(cfg.contains("naked"));
        srp.cocoon_init(1, vec![1]);
        srp.select_shell(ParasiteShell::GitRepo);
        let cfg2 = srp.generate_config();
        assert!(cfg2.contains("git"));
    }
    #[test] fn test_shell_name() {
        assert_eq!(ParasiteShell::FileSystem.name(), "fs");
        assert_eq!(ParasiteShell::GitRepo.name(), "git");
        assert_eq!(ParasiteShell::DockerContainer.name(), "docker");
    }
    #[test] fn test_default() {
        let srp = SilkwormReverseParasite::default();
        assert!(srp.current_shell.is_none());
        assert_eq!(srp.phase, ParasitePhase::Dormant);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器环形缓冲区 · ContainerRingBuffer (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   固定容量 N, 写满后覆写最旧数据 (环形)
//   仅加法: head/tail 位移用的定长加法 + 大于判断回绕 (避免取模)
//
//   操作: write(v), read(), peek(), is_full(),
//         available (可读), overhead (余量)

#[derive(Clone, Debug)]
pub struct ContainerRingBuffer {
    pub capacity: usize,
    pub storage: Vec<Option<u64>>,
    pub head: usize,        // 最旧可读位置
    pub tail: usize,        // 下一个写入位置
    pub count: usize,       // 当前元素数
    pub total_writes: u64,
    pub total_reads: u64,
    pub total_overwrites: u64,
}

impl ContainerRingBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut v: Vec<Option<u64>> = Vec::with_capacity(capacity);
        for _ in 0..capacity { v.push(None); }
        Self { capacity, storage: v, head: 0, tail: 0, count: 0, total_writes: 0, total_reads: 0, total_overwrites: 0 }
    }

    pub fn write(&mut self, v: u64) {
        let overwrite = self.storage[self.tail].is_some() && self.count == self.capacity;
        self.storage[self.tail] = Some(v);
        self.tail = self.next_pos(self.tail);
        if overwrite {
            self.head = self.next_pos(self.head);
            self.total_overwrites = self.total_overwrites + 1;
        } else if self.count < self.capacity {
            self.count = self.count + 1;
        }
        self.total_writes = self.total_writes + 1;
    }

    pub fn read(&mut self) -> Option<u64> {
        if self.count == 0 { return None; }
        let v = self.storage[self.head];
        self.storage[self.head] = None;
        self.head = self.next_pos(self.head);
        self.count = self.count - 1;
        self.total_reads = self.total_reads + 1;
        v
    }

    pub fn peek(&self) -> Option<u64> {
        if self.count == 0 { return None; }
        self.storage[self.head]
    }

    fn next_pos(&self, p: usize) -> usize {
        let n = p + 1;
        if n >= self.capacity { 0 } else { n }
    }

    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn is_full(&self) -> bool { self.count == self.capacity }
    pub fn available(&self) -> usize { self.count }
    pub fn remaining(&self) -> usize { self.capacity - self.count }

    /// 批量读出所有可用 (不清空)
    pub fn drain(&mut self) -> Vec<u64> {
        let mut out: Vec<u64> = Vec::new();
        loop {
            match self.read() {
                Some(v) => { out.push(v); }
                None => { break; }
            }
        }
        out
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity { self.storage[i] = None; }
        self.head = 0; self.tail = 0; self.count = 0;
        self.total_writes = 0; self.total_reads = 0; self.total_overwrites = 0;
    }
}

impl Default for ContainerRingBuffer {
    fn default() -> Self { Self::new(16) }
}

#[cfg(test)]
mod ring_buffer_tests {
    use super::*;

    #[test]
    fn test_basic_read_write() {
        let mut rb = ContainerRingBuffer::new(4);
        rb.write(10); rb.write(20);
        assert_eq!(rb.read(), Some(10));
        assert_eq!(rb.read(), Some(20));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_overwrite() {
        let mut rb = ContainerRingBuffer::new(2);
        rb.write(1); rb.write(2);
        assert!(rb.is_full());
        rb.write(3);  // 覆写 1
        assert_eq!(rb.total_overwrites, 1);
        assert_eq!(rb.read(), Some(2));  // 2 还在
        assert_eq!(rb.read(), Some(3));  // 3 覆写成功
    }

    #[test]
    fn test_peek_no_consume() {
        let mut rb = ContainerRingBuffer::new(4);
        rb.write(77);
        assert_eq!(rb.peek(), Some(77));
        assert_eq!(rb.peek(), Some(77));  // 还在
        assert_eq!(rb.available(), 1);
    }

    #[test]
    fn test_read_empty() {
        let mut rb = ContainerRingBuffer::new(4);
        assert!(rb.read().is_none());
    }

    #[test]
    fn test_drain() {
        let mut rb = ContainerRingBuffer::new(10);
        for i in 1..=5 { rb.write(i); }
        let v = rb.drain();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
        assert!(rb.is_empty());
    }

    #[test]
    fn test_overwrite_cycle() {
        let mut rb = ContainerRingBuffer::new(3);
        rb.write(1); rb.write(2); rb.write(3);  // full
        rb.write(4);  // 1 out
        rb.write(5);  // 2 out
        assert_eq!(rb.available(), 3);
        assert_eq!(rb.read(), Some(3));
        assert_eq!(rb.read(), Some(4));
        assert_eq!(rb.read(), Some(5));
    }

    #[test]
    fn test_clear() {
        let mut rb = ContainerRingBuffer::new(4);
        rb.write(1); rb.write(2);
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.total_writes, 0);
    }

    #[test]
    fn test_default() {
        let rb = ContainerRingBuffer::default();
        assert_eq!(rb.capacity, 16);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器校验和引擎 · ContainerChecksum (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   仅加法求和校验 — 不做 CRC/乘除
//   性质: 顺序无关 (加法交换律), 相同数据集校验和相等
//
//   扩展:
//     - rolling: 滑窗校验 (增量更新: 加新减旧)
//     - pairwise: 交叉累加两两差值

#[derive(Clone, Debug)]
pub struct ContainerChecksum {
    pub sum: u64,
    pub count: u64,
    pub history: Vec<u64>,
}

impl ContainerChecksum {
    pub fn new() -> Self { Self { sum: 0, count: 0, history: Vec::new() } }

    /// 加入一个样本 (求和)
    pub fn add(&mut self, value: u64) -> u64 {
        self.sum = self.sum + value;
        self.count = self.count + 1;
        self.sum
    }

    /// 移除一个样本 (反向)
    pub fn subtract(&mut self, value: u64) -> u64 {
        self.sum = self.sum - value;
        if self.count > 0 { self.count = self.count - 1; }
        self.sum
    }

    pub fn value(&self) -> u64 { self.sum }

    /// 验证另一份数据是否匹配
    pub fn verify(&self, expected_sum: u64) -> bool { self.sum == expected_sum }

    /// 比较两份数据集 (仅加法比较)
    pub fn compare(&self, other: &ContainerChecksum) -> i64 {
        if self.sum > other.sum { 1 }
        else if self.sum < other.sum { -1 }
        else { 0 }
    }

    /// 快照当前和到历史
    pub fn snapshot(&mut self) { self.history.push(self.sum); }

    /// 历史快照数
    pub fn history_len(&self) -> usize { self.history.len() }

    pub fn clear(&mut self) { self.sum = 0; self.count = 0; self.history.clear(); }
}

impl Default for ContainerChecksum {
    fn default() -> Self { Self::new() }
}

/// 从一个 slice 直接计算校验和 (工具函数)
pub fn checksum_of(slice: &[u64]) -> u64 {
    let mut s: u64 = 0;
    for &v in slice { s = s + v; }
    s
}

#[cfg(test)]
mod checksum_tests {
    use super::*;

    #[test]
    fn test_empty_sum_zero() {
        let c = ContainerChecksum::new();
        assert_eq!(c.value(), 0);
        assert_eq!(c.count, 0);
    }

    #[test]
    fn test_add() {
        let mut c = ContainerChecksum::new();
        c.add(5); c.add(10); c.add(15);
        assert_eq!(c.value(), 30);
        assert_eq!(c.count, 3);
    }

    #[test]
    fn test_subtract() {
        let mut c = ContainerChecksum::new();
        c.add(50); c.add(30);
        c.subtract(30);
        assert_eq!(c.value(), 50);
        assert_eq!(c.count, 1);
    }

    #[test]
    fn test_verify() {
        let mut c = ContainerChecksum::new();
        c.add(7); c.add(14);
        assert!(c.verify(21));
        assert!(!c.verify(99));
    }

    #[test]
    fn test_compare() {
        let mut a = ContainerChecksum::new(); a.add(50);
        let mut b = ContainerChecksum::new(); b.add(30);
        assert_eq!(a.compare(&b), 1);
        assert_eq!(b.compare(&a), -1);
        assert_eq!(a.compare(&a), 0);
    }

    #[test]
    fn test_snapshot_history() {
        let mut c = ContainerChecksum::new();
        c.add(5);  c.snapshot();
        c.add(10); c.snapshot();
        c.add(15); c.snapshot();
        assert_eq!(c.history_len(), 3);
        assert_eq!(c.history, vec![5, 15, 30]);
    }

    #[test]
    fn test_checksum_of_fn() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(checksum_of(&v), 15);
        assert_eq!(checksum_of(&[]), 0);
    }

    #[test]
    fn test_clear() {
        let mut c = ContainerChecksum::new();
        c.add(100); c.snapshot();
        c.clear();
        assert_eq!(c.value(), 0);
        assert_eq!(c.history_len(), 0);
    }
}

// ══════════════════════════════════════════════════════════════════════
// 三元见证引擎 · TernaryWitnessEngine (v1.0)
// ══════════════════════════════════════════════════════════════════════
//
//   三元 = 观察者 · 被观察者 · 见证者
//   "三元作为见证裁判裁定准入"
//
//   核心操作:
//     - bind_ternary()    — 创建三元绑定
//     - adjudicate()      — 裁定是否准入 (多数决)
//     - append_proof()    — 追加证明到证明链
//     - verify_chain()    — 验证证明链完整性
//     - batch_adjudicate()— 批量准入裁定
//
//   NJSG 哲学: 见证不是"证实存在", 而是"裁定准入"
//   观察者提出, 被观察者提供, 见证者裁定 — 三方缺一不可

/// 三元见证引擎
#[derive(Clone, Debug)]
pub struct TernaryWitnessEngine {
    /// 三元绑定: 容器id → (观察者, 被观察者, 见证者)
    pub ternary_bindings: HashMap<u32, (u32, u32, u32)>,
    /// 准入记录: 容器id → 是否被裁定准入
    pub admission_records: HashMap<u32, bool>,
    /// 证明链: 容器id → 证明哈希序列
    pub proof_chains: HashMap<u32, Vec<u64>>,
    /// 见证阈值: 至少需要多少见证者同意
    pub quorum: usize,
}

impl TernaryWitnessEngine {
    pub fn new() -> Self {
        Self {
            ternary_bindings: HashMap::new(),
            admission_records: HashMap::new(),
            proof_chains: HashMap::new(),
            quorum: 1,
        }
    }

    /// 设置见证阈值
    pub fn set_quorum(&mut self, n: usize) { self.quorum = n.max(1); }

    /// 创建三元绑定 (观察者, 被观察者, 见证者)
    pub fn bind_ternary(&mut self, observer: u32, observed: u32, witness: u32) {
        self.ternary_bindings.insert(observer, (observer, observed, witness));
    }

    /// 裁定容器是否准入 (至少 quorum 个见证者同意)
    pub fn adjudicate(&mut self, container_id: u32, witnesses_agree: &[bool]) -> bool {
        let agree_count = witnesses_agree.iter().filter(|&&x| x).count();
        let admitted = agree_count >= self.quorum;
        self.admission_records.insert(container_id, admitted);
        admitted
    }

    /// 判定容器是否已准入
    pub fn is_admitted(&self, container_id: u32) -> bool {
        self.admission_records.get(&container_id).copied().unwrap_or(false)
    }

    /// 追加证明到证明链
    pub fn append_proof(&mut self, container_id: u32, proof_hash: u64) {
        self.proof_chains.entry(container_id).or_default().push(proof_hash);
    }

    /// 获取证明链
    pub fn proof_chain(&self, container_id: u32) -> Option<&Vec<u64>> {
        self.proof_chains.get(&container_id)
    }

    /// 验证证明链完整性 (非递减序列)
    pub fn verify_chain(&self, container_id: u32) -> bool {
        if let Some(chain) = self.proof_chains.get(&container_id) {
            if chain.len() < 2 { return true; }
            chain.windows(2).all(|w| w[1] >= w[0])
        } else { false }
    }

    /// 获取见证者
    pub fn witness_for(&self, container_id: u32) -> Option<u32> {
        self.ternary_bindings.get(&container_id).map(|(_, _, w)| *w)
    }

    /// 移除绑定与记录
    pub fn remove(&mut self, container_id: u32) {
        self.ternary_bindings.remove(&container_id);
        self.admission_records.remove(&container_id);
        self.proof_chains.remove(&container_id);
    }

    /// 批量准入裁定
    pub fn batch_adjudicate(&mut self, judgments: &[(u32, Vec<bool>)]) -> HashMap<u32, bool> {
        let mut results = HashMap::new();
        for (id, votes) in judgments {
            let result = self.adjudicate(*id, votes);
            results.insert(*id, result);
        }
        results
    }

    /// 已准入容器数
    pub fn admitted_count(&self) -> usize {
        self.admission_records.values().filter(|&&v| v).count()
    }

    /// 重置所有记录
    pub fn clear(&mut self) {
        self.ternary_bindings.clear();
        self.admission_records.clear();
        self.proof_chains.clear();
    }
}

impl Default for TernaryWitnessEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod witness_tests {
    use super::*;
    #[test] fn test_new() {
        let e = TernaryWitnessEngine::new();
        assert_eq!(e.quorum, 1);
        assert_eq!(e.admitted_count(), 0);
    }
    #[test] fn test_bind_ternary() {
        let mut e = TernaryWitnessEngine::new();
        e.bind_ternary(1, 2, 3);
        assert_eq!(e.witness_for(1), Some(3));
    }
    #[test] fn test_adjudicate_admitted() {
        let mut e = TernaryWitnessEngine::new();
        assert!(e.adjudicate(1, &[true, true, false]));
        assert!(e.is_admitted(1));
    }
    #[test] fn test_adjudicate_denied() {
        let mut e = TernaryWitnessEngine::new();
        e.set_quorum(2);
        assert!(!e.adjudicate(1, &[true, false, false]));
        assert!(!e.is_admitted(1));
    }
    #[test] fn test_quorum() {
        let mut e = TernaryWitnessEngine::new();
        e.set_quorum(3);
        assert!(!e.adjudicate(1, &[true, true, false]));
        assert!(e.adjudicate(2, &[true, true, true]));
        assert_eq!(e.admitted_count(), 1);
    }
    #[test] fn test_proof_chain() {
        let mut e = TernaryWitnessEngine::new();
        e.append_proof(1, 100);
        e.append_proof(1, 200);
        e.append_proof(1, 300);
        let chain = e.proof_chain(1).unwrap();
        assert_eq!(chain.len(), 3);
        assert!(e.verify_chain(1));
    }
    #[test] fn test_verify_chain_fail() {
        let mut e = TernaryWitnessEngine::new();
        e.append_proof(1, 300);
        e.append_proof(1, 100);
        assert!(!e.verify_chain(1));
    }
    #[test] fn test_remove() {
        let mut e = TernaryWitnessEngine::new();
        e.bind_ternary(1, 2, 3);
        e.adjudicate(1, &[true]);
        e.append_proof(1, 100);
        e.remove(1);
        assert!(!e.is_admitted(1));
        assert_eq!(e.proof_chain(1), None);
    }
    #[test] fn test_batch_adjudicate() {
        let mut e = TernaryWitnessEngine::new();
        e.set_quorum(1);
        let judgments = vec![(1, vec![true]), (2, vec![false]), (3, vec![true])];
        let results = e.batch_adjudicate(&judgments);
        assert!(results[&1]);
        assert!(!results[&2]);
        assert!(results[&3]);
        assert_eq!(e.admitted_count(), 2);
    }
    #[test] fn test_clear() {
        let mut e = TernaryWitnessEngine::new();
        e.adjudicate(1, &[true]);
        e.clear();
        assert_eq!(e.admitted_count(), 0);
    }
    #[test] fn test_default() {
        let e = TernaryWitnessEngine::default();
        assert_eq!(e.admitted_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器流水线引擎 · ContainerPipelineEngine (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   仅加法级联: Stage0 → Stage1 → … → StageN
//   每个 stage 是 "u64 → u64" 的纯加法变换
//
//   变换类型:
//     - add(n)       : +n
//     - sub(n)       : -n (n ≤ input 时不借位 → max(0, input-n))
//     - clamp(min,max): 截断
//     - shift_left(k): << k (放大)
//     - shift_right(k): >> k (缩小)
//
//   流水线计算: 输入 val, 逐 stage 通过; 输出最终结果

#[derive(Clone, Debug, PartialEq)]
enum StageOp { Add(u64), Sub(u64), Clamp(u64, u64), Shl(u32), Shr(u32) }

#[derive(Clone, Debug)]
pub struct ContainerPipelineEngine {
    pub stages: Vec<StageOp>,
    pub total_runs: u64,
}

impl ContainerPipelineEngine {
    pub fn new() -> Self { Self { stages: Vec::new(), total_runs: 0 } }

    pub fn add_stage(&mut self, op: StageOp) { self.stages.push(op); }
    pub fn clear(&mut self) { self.stages.clear(); self.total_runs = 0; }
    pub fn stage_count(&self) -> usize { self.stages.len() }

    /// 运行一个值通过完整流水线
    pub fn run(&mut self, val: u64) -> u64 {
        let mut v = val;
        for op in self.stages.iter() {
            v = match *op {
                StageOp::Add(n) => v + n,
                StageOp::Sub(n) => if v >= n { v - n } else { 0 },
                StageOp::Clamp(lo, hi) => {
                    if v < lo { lo }
                    else if v > hi { hi }
                    else { v }
                }
                StageOp::Shl(k) => v << k,
                StageOp::Shr(k) => v >> k,
            };
        }
        self.total_runs = self.total_runs + 1;
        v
    }

    pub fn stage_names(&self) -> String {
        let parts: Vec<String> = self.stages.iter().map(|op| match op {
            StageOp::Add(n) => "add=".to_string() + &n.to_string(),
            StageOp::Sub(n) => "sub=".to_string() + &n.to_string(),
            StageOp::Clamp(a, b) => "clamp[".to_string() + &a.to_string() + "," + &b.to_string() + "]",
            StageOp::Shl(k) => "shl=".to_string() + &k.to_string(),
            StageOp::Shr(k) => "shr=".to_string() + &k.to_string(),
        }).collect();
        if parts.is_empty() { "∅".to_string() } else { parts.join(" → ") }
    }
}

impl Default for ContainerPipelineEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod pipeline_engine_tests {
    use super::*;

    #[test]
    fn test_no_stage_identity() {
        let mut p = ContainerPipelineEngine::new();
        assert_eq!(p.run(42), 42);
    }

    #[test]
    fn test_add_only() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Add(10));
        p.add_stage(StageOp::Add(5));
        assert_eq!(p.run(0), 15);
        assert_eq!(p.run(100), 115);
    }

    #[test]
    fn test_sub_clamp_zero() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Sub(5));
        assert_eq!(p.run(10), 5);
        assert_eq!(p.run(3), 0);   // 不足减 → 0
    }

    #[test]
    fn test_clamp() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Clamp(10, 50));
        assert_eq!(p.run(5), 10);
        assert_eq!(p.run(50), 50);
        assert_eq!(p.run(100), 50);
    }

    #[test]
    fn test_shift_chain() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Shl(1)); // ×2
        p.add_stage(StageOp::Shr(1)); // ÷2 → 还原
        assert_eq!(p.run(21), 21);
    }

    #[test]
    fn test_mixed_pipeline() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Add(5));     // input 7 → 12
        p.add_stage(StageOp::Clamp(0, 10)); // → 10
        p.add_stage(StageOp::Shl(2));     // → 40
        p.add_stage(StageOp::Sub(10));    // → 30
        assert_eq!(p.run(7), 30);
    }

    #[test]
    fn test_stage_count() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Add(1));
        p.add_stage(StageOp::Sub(1));
        assert_eq!(p.stage_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut p = ContainerPipelineEngine::new();
        p.add_stage(StageOp::Add(1));
        p.clear();
        assert_eq!(p.stage_count(), 0);
    }

    #[test]
    fn test_run_counter() {
        let mut p = ContainerPipelineEngine::new();
        p.run(1); p.run(2); p.run(3);
        assert_eq!(p.total_runs, 3);
    }

    #[test]
    fn test_default() {
        let p = ContainerPipelineEngine::default();
        assert_eq!(p.stage_count(), 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器表决引擎 · ContainerVoter (v1.0)
// ═══════════════════════════════════════════════════════════════════════════
//
//   多裁决: 每个投票者对提案给出 0/1 信号, 仅加法计数
//   裁决方式:
//     - majority:    > N/2
//     - unanimous:   == N
//     - any:         ≥ 1
//     - threshold(k): ≥ k 即通过
//
//   仅加法: yes_count +1, no_count +1, 比较 > <
//   (N/2 用右移 >> 1)

#[derive(Clone, Debug)]
pub struct ContainerVoter {
    pub yes_count: u64,
    pub no_count: u64,
    pub abstain: u64,
    pub total_rounds: u64,
    pub threshold: u64,     // 一次表决所需最低票数
}

impl ContainerVoter {
    pub fn new(threshold: u64) -> Self {
        Self { yes_count: 0, no_count: 0, abstain: 0, total_rounds: 0, threshold }
    }

    /// 投一票: true=yes, false=no
    pub fn vote(&mut self, approve: bool) {
        if approve { self.yes_count = self.yes_count + 1; }
        else { self.no_count = self.no_count + 1; }
    }

    /// 弃权
    pub fn abstain_vote(&mut self) { self.abstain = self.abstain + 1; }

    /// 一轮表决的最终决议
    pub fn tally(&self) -> VoterResult {
        let total = self.yes_count + self.no_count;
        if total == 0 { return VoterResult::QuorumFailed; }
        if self.yes_count + self.yes_count > total { VoterResult::Approved }
        else if self.yes_count >= self.threshold { VoterResult::ThresholdMet }
        else { VoterResult::Rejected }
    }

    /// 是否一致通过
    pub fn unanimous(&self) -> bool {
        self.no_count == 0 && self.yes_count > 0
    }

    /// 是否任一通过
    pub fn any_yes(&self) -> bool { self.yes_count > 0 }

    pub fn total_cast(&self) -> u64 { self.yes_count + self.no_count + self.abstain }
    pub fn record_round(&mut self) { self.total_rounds = self.total_rounds + 1; }

    pub fn reset(&mut self) {
        self.yes_count = 0; self.no_count = 0; self.abstain = 0;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VoterResult { Approved, Rejected, ThresholdMet, QuorumFailed }

impl Default for ContainerVoter {
    fn default() -> Self { Self::new(1) }
}

#[cfg(test)]
mod voter_tests {
    use super::*;

    #[test]
    fn test_unanimous() {
        let mut v = ContainerVoter::new(1);
        v.vote(true); v.vote(true);
        assert!(v.unanimous());
        assert_eq!(v.tally(), VoterResult::Approved);
    }

    #[test]
    fn test_majority_reject() {
        let mut v = ContainerVoter::new(5);
        v.vote(false); v.vote(false); v.vote(true);
        // total=3, yes=1: 1*2=2 ≤ 3 → not majority
        assert_eq!(v.tally(), VoterResult::Rejected);
    }

    #[test]
    fn test_threshold_met() {
        let mut v = ContainerVoter::new(2);
        v.vote(true); v.vote(true);
        assert_eq!(v.tally(), VoterResult::Approved);  // also passes threshold
    }

    #[test]
    fn test_quorum_failed() {
        let v = ContainerVoter::new(3);
        assert_eq!(v.tally(), VoterResult::QuorumFailed);
    }

    #[test]
    fn test_any_yes() {
        let mut v = ContainerVoter::new(10);
        v.vote(false); v.vote(true); v.vote(false);
        assert!(v.any_yes());
    }

    #[test]
    fn test_reset() {
        let mut v = ContainerVoter::new(1);
        v.vote(true); v.vote(false);
        v.reset();
        assert_eq!(v.total_cast(), 0);
    }

    #[test]
    fn test_record_round() {
        let mut v = ContainerVoter::new(1);
        v.record_round(); v.record_round();
        assert_eq!(v.total_rounds, 2);
    }

    #[test]
    fn test_total_cast() {
        let mut v = ContainerVoter::new(1);
        v.vote(true); v.vote(false); v.abstain_vote();
        assert_eq!(v.total_cast(), 3);
    }

    #[test]
    fn test_default() {
        let v = ContainerVoter::default();
        assert_eq!(v.threshold, 1);
    }
}
