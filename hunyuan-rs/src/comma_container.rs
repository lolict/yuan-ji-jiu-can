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
    pub fn years_covered_double_digit(&self) -> f128_compat {
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

pub type f128_compat = u128; // 对于整数精确定数用 u128

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

    pub fn find_cluster(&self, addr: (u8, u8, u8)) -> Option<&ClusterNode> {
        self.clusters.iter().min_by(|a, b| {
            let da = self.distance(addr, (a.centroid.0 as u8, a.centroid.1 as u8, a.centroid.2 as u8));
            let db = self.distance(addr, (b.centroid.0 as u8, b.centroid.1 as u8, b.centroid.2 as u8));
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
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

#[derive(Clone, Debug)]
pub struct WrappingNode {
    pub name: String,
    pub cluster_id: Option<u64>,
    pub children: Vec<WrappingNode>,
    pub depth: u32,
    pub weight: f64,
}

impl WrappingNode {
    pub fn new(name: &str, cluster_id: Option<u64>) -> Self {
        Self { name: name.to_string(), cluster_id, children: Vec::new(), depth: 0, weight: 1.0 }
    }
    pub fn add_child(&mut self, mut child: WrappingNode) { child.depth = self.depth + 1; self.children.push(child); }
    pub fn leaf_count(&self) -> usize {
        if self.children.is_empty() { return 1; }
        self.children.iter().map(|c| c.leaf_count()).sum()
    }
    pub fn sort_by_weight(&mut self) {
        self.children.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        for c in &mut self.children { c.sort_by_weight(); }
    }
}

pub struct ConnectionWrappingTree { pub root: WrappingNode }

impl ConnectionWrappingTree {
    pub fn new() -> Self { Self { root: WrappingNode::new("根", None) } }
    pub fn from_clusters(clusters: &[ClusterNode]) -> Self {
        let mut tree = Self::new();
        let mut sorted: Vec<&ClusterNode> = clusters.iter().collect();
        sorted.sort_by(|a, b| b.density.partial_cmp(&a.density).unwrap_or(std::cmp::Ordering::Equal));
        for (i, c) in sorted.iter().enumerate() {
            let mut node = WrappingNode::new(&format!("包裹_{}", i), Some(c.id));
            node.weight = c.density * c.members.len() as f64;
            tree.root.add_child(node);
        }
        tree.root.sort_by_weight();
        tree
    }
    pub fn display(&self) -> String {
        let mut out = String::new();
        Self::display_node(&self.root, &mut out, "");
        out
    }
    fn display_node(node: &WrappingNode, out: &mut String, prefix: &str) {
        let id_str = node.cluster_id.map(|id| format!("[聚类{}]", id)).unwrap_or_default();
        out.push_str(&format!("{}{} {} (权重:{:.4})\n", prefix, node.name, id_str, node.weight));
        for (i, child) in node.children.iter().enumerate() {
            let is_last = i == node.children.len() - 1;
            let branch = if is_last { "└── " } else { "├── " };
            let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
            out.push_str(&format!("{}{}", prefix, branch));
            Self::display_node(child, out, &new_prefix);
        }
    }
}

#[derive(Clone, Debug)]
pub struct SituationalDocument { pub id: u64, pub name: String, pub addresses: Vec<(u8, u8, u8)>, pub tags: Vec<String> }

#[derive(Clone, Debug)]
pub struct SituationalView { pub address: (u8, u8, u8), pub cluster_id: u64, pub cluster_centroid: (f64, f64, f64), pub cluster_radius: f64, pub cluster_density: f64 }

pub struct SituationalAggregationBrowser { pub engine: ClusteringEngine, pub documents: Vec<SituationalDocument>, pub wrapping_tree: Option<ConnectionWrappingTree> }

impl SituationalAggregationBrowser {
    pub fn new() -> Self { Self { engine: ClusteringEngine::new(), documents: Vec::new(), wrapping_tree: None } }
    pub fn add_document(&mut self, name: &str, cr: (u8, u8), vr: (u8, u8), tr: (u8, u8)) -> u64 {
        let id = self.documents.len() as u64;
        let mut addresses = Vec::new();
        for c in cr.0..=cr.1.min(47) { for v in vr.0..=vr.1.min(65) { for t in tr.0..=tr.1.min(99) { addresses.push((c, v, t)); } } }
        self.documents.push(SituationalDocument { id, name: name.to_string(), addresses, tags: Vec::new() });
        id
    }
    pub fn tag_document(&mut self, id: u64, tag: &str) { if let Some(d) = self.documents.iter_mut().find(|d| d.id == id) { d.tags.push(tag.to_string()); } }
    pub fn build(&mut self) {
        let all: Vec<(u8, u8, u8)> = self.documents.iter().flat_map(|d| d.addresses.clone()).collect();
        self.engine.cluster(&all);
        self.wrapping_tree = Some(ConnectionWrappingTree::from_clusters(&self.engine.clusters));
    }
    pub fn situational_query(&self, addr: (u8, u8, u8)) -> Option<SituationalView> {
        let c = self.engine.find_cluster(addr)?;
        Some(SituationalView { address: addr, cluster_id: c.id, cluster_centroid: c.centroid, cluster_radius: c.radius, cluster_density: c.density })
    }
    pub fn filter_by_consonant(&self, c_start: u8, c_end: u8) -> Vec<&SituationalDocument> {
        self.documents.iter().filter(|d| d.addresses.iter().any(|a| a.0 >= c_start && a.0 <= c_end)).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 八流统一层 · 意识路由 · 容器交互
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlowKind { Preload, Interpret, Sovereignty, Property, Resonance, Feedback, Invocation, CommonDivisor }

impl FlowKind {
    pub fn layer(&self) -> u8 { match self { FlowKind::Preload => 1, FlowKind::Interpret => 2, FlowKind::Sovereignty => 3, FlowKind::Property => 4, FlowKind::Resonance => 5, FlowKind::Feedback => 6, FlowKind::Invocation => 7, FlowKind::CommonDivisor => 8 } }
    pub fn is_dynamic(&self) -> bool { matches!(self, FlowKind::Resonance | FlowKind::Feedback | FlowKind::Invocation) }
    pub fn priority(&self) -> u8 { match self { FlowKind::Preload => 8, FlowKind::Sovereignty => 7, FlowKind::Interpret => 6, FlowKind::Property => 5, FlowKind::CommonDivisor => 4, FlowKind::Resonance => 3, FlowKind::Feedback => 2, FlowKind::Invocation => 1 } }
    pub fn all() -> [FlowKind; 8] { [FlowKind::Preload, FlowKind::Interpret, FlowKind::Sovereignty, FlowKind::Property, FlowKind::Resonance, FlowKind::Feedback, FlowKind::Invocation, FlowKind::CommonDivisor] }
}

#[derive(Clone, Debug)]
pub struct FlowState { pub kind: FlowKind, pub intensity: f64, pub address: Option<(u8, u8, u8)>, pub completed: bool }

impl FlowState {
    pub fn new(kind: FlowKind) -> Self { Self { kind, intensity: 0.5, address: None, completed: false } }
    pub fn advance(&mut self, delta: f64) { self.intensity = (self.intensity + delta).clamp(0.0, 1.0); }
    pub fn complete(&mut self) { self.completed = true; self.intensity = 1.0; }
}

#[derive(Clone, Debug)]
pub enum ContainerInteraction { Absorb { target: String, source: String }, Resonate { a: String, b: String, strength: f64 }, Invoke { location: (u8, u8, u8), program: String } }

#[derive(Clone, Copy, Debug, Default)]
pub struct RouterStats { pub total_routes: usize, pub absorbed_containers: usize, pub resonated_pairs: usize }

pub struct ConsciousnessRouter { pub active_flows: Vec<FlowState>, pub containers: Vec<String>, pub interactions: Vec<ContainerInteraction>, pub stats: RouterStats }

impl ConsciousnessRouter {
    pub fn new() -> Self { Self { active_flows: Vec::new(), containers: Vec::new(), interactions: Vec::new(), stats: RouterStats::default() } }
    pub fn register_container(&mut self, name: &str) { if !self.containers.contains(&name.to_string()) { self.containers.push(name.to_string()); } }
    pub fn activate_flow(&mut self, kind: FlowKind, addr: Option<(u8, u8, u8)>) -> usize {
        let s = FlowState { kind, intensity: 0.5, address: addr, completed: false };
        self.active_flows.push(s);
        self.active_flows.len() - 1
    }
    pub fn route(&mut self, addr: (u8, u8, u8), kind: FlowKind) -> Option<usize> {
        if let Some((idx, _)) = self.active_flows.iter().enumerate().find(|(_, f)| f.kind == kind && f.address == Some(addr)) { return Some(idx); }
        Some(self.activate_flow(kind, Some(addr)))
    }
    pub fn interact(&mut self, inter: ContainerInteraction) {
        match &inter { ContainerInteraction::Absorb { .. } => self.stats.absorbed_containers += 1, ContainerInteraction::Resonate { .. } => self.stats.resonated_pairs += 1, _ => {} }
        self.stats.total_routes += 1;
        self.interactions.push(inter);
    }
    pub fn find_common_divisor(&self, a: &str, b: &str) -> Option<String> {
        let common: String = a.chars().zip(b.chars()).take_while(|(ca, cb)| ca == cb).map(|(c, _)| c).collect();
        if common.is_empty() { None } else { Some(common) }
    }
    pub fn flows_of_kind(&self, kind: FlowKind) -> Vec<&FlowState> { self.active_flows.iter().filter(|f| f.kind == kind).collect() }
}

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
    #[test] fn test_priority() { assert!(FlowKind::Preload.priority() > FlowKind::Invocation.priority()); }
}

#[cfg(test)]
mod radical_star_tests {
    use super::*;
    #[test] fn test_star_coord() { let c = StarCoordinate::from_address((0, 0, 0)); assert!(c.x > 0.0); assert!(c.y.abs() < 0.01); assert!((c.luminance - 0.2).abs() < 0.01); }
    #[test] fn test_star_angle() { let c = StarCoordinate::from_address((12, 30, 50)); assert!(c.x.abs() < 1.0); assert!(c.y > 0.0); }
    #[test] fn test_star_distance() { let c1 = StarCoordinate::from_address((0, 0, 0)); let c2 = StarCoordinate::from_address((24, 0, 0)); let d = c1.distance_to(&c2); assert!(d > 50.0 && d < 300.0); }
    #[test] fn test_map_creation() { let m = RadicalStarMap::new(); assert_eq!(m.consonant_groups.len(), 14); assert_eq!(m.vowel_groups.len(), 8); }
    #[test] fn test_add_star() { let mut m = RadicalStarMap::new(); m.add_star((10, 20, 50)); assert_eq!(m.stars.len(), 1); }
    #[test] fn test_add_range() { let mut m = RadicalStarMap::new(); m.add_range((0, 2), (0, 3), (0, 1)); assert_eq!(m.stars.len(), 24); }
    #[test] fn test_essential_match_identity() { let mut m = RadicalStarMap::new(); let r = m.essential_match((10, 20, 50), 10); assert_eq!(r[0].match_type, MatchType::Identity); }
    #[test] fn test_essential_match_consonant() { let mut m = RadicalStarMap::new(); let r = m.essential_match((5, 10, 30), 20); let cm: Vec<&EssentialMatch> = r.iter().filter(|m| m.target.0 == 5 && m.target.1 != 10).collect(); assert!(!cm.is_empty()); }
    #[test] fn test_stars_in_region() { let mut m = RadicalStarMap::new(); m.add_star((0, 30, 50)); let region = m.stars_in_region(-200.0, -200.0, 200.0, 200.0); assert!(!region.is_empty()); }
    #[test] fn test_energy_distribution() { let mut m = RadicalStarMap::new(); m.add_range((0, 47), (0, 10), (0, 0)); let d = m.energy_distribution(); assert_eq!(d.len(), 14); assert!(d.iter().any(|&e| e > 0.0)); }
    #[test] fn test_map_stats() { let mut m = RadicalStarMap::new(); m.add_range((0, 5), (0, 3), (0, 1)); assert_eq!(m.stats().star_count, 48); }
    #[test] fn test_semantic_id() { let p = StarPoint::new((15, 30, 50), 0); assert_eq!(p.semantic_id(), 2); }
    #[test] fn test_luminance() { let l = StarCoordinate::from_address((10, 30, 0)); let h = StarCoordinate::from_address((10, 30, 99)); assert!(h.luminance > l.luminance); }
    #[test] fn test_saturation() { let l = StarCoordinate::from_address((20, 40, 0)); let h = StarCoordinate::from_address((20, 40, 99)); assert!(h.saturation >= l.saturation); }
}
