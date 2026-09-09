// ══════════════════════════════════════════════════════════════════
// celestial_perception_v5.rs — 天觉v5（老五）· 语言系统
// 八系扩展: v4七系 + 语言系统(汉字编码+声母韵母+自进制+命令解析+流属性位)
// ══════════════════════════════════════════════════════════════════
//
// 版本: v5（老五）
// 规则: 不修改旧版文件, 独立新版本, 报数命名
//
// 语言系统核心:
//   1. 汉字容器编码 (HanziEncoding) — 汉字↔64位容器码, 基于笔画/结构/语义三维
//   2. 声母韵母系统 (PhoneticSystem) — 声母遍历韵母的谐音路由
//   3. 自进制转换 (SelfRadix) — 256进制(基础254+2锚点: 高位持有者=1, 低位持有者=0)
//   4. 命令解析器 (CommandParser) — 符号命令→系统动作
//   5. 流属性位映射 (FlowPropertyBits) — 12位二元属性↔流属性互换
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror;
use crate::observer::{Observer, SenseForm, SenseKind};
use crate::memory::{Coord, MetaJudgeEngine};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;
use std::time::{SystemTime, UNIX_EPOCH};

// 基础类型从v2导入 (源头)
pub use crate::celestial_perception_v2::{
    AuditoryAuth, AuditorySystem,
    TianJueAuth,
    VisualSystem,
    CommonDivisor, CommonMultiple, MirrorComparison,
    Bag as BagV4, BagAttributes as BagAttributesV4,
    BagHierarchy as BagHierarchyV4, HierarchyReport as HierarchyReportV4,
    JudgmentEngine as JudgmentEngineV4, JudgmentResult as JudgmentResultV4,
    JudgmentRecord as JudgmentRecordV4,
    HistoricalAccumulation as HistoricalAccumulationV4,
};
// 七系外周从v4导入
pub use crate::celestial_perception_v4::{
    Emotion, EmotionState,
    Behavior, BehaviorSystem,
    ThinkingSystem, ReasoningChain,
    AestheticSystem, AestheticVerdict, AestheticDimensions,
    EliminationSystem, EliminationReason, EliminationRecord,
    FilterSystem,
    CelestialPerceptionV4,
};
// v4的PipelineResult (从v4的类型别名引用)
use crate::celestial_perception_v4::PipelineResult as PipelineResultV4;

// ======================================================================
// 第一部分: 汉字容器编码 (HanziEncoding)
// ======================================================================

/// 汉字容器编码 — 汉字直接映射为64位容器码
///
/// 编码基于三维:
///   - 笔画维度 (Stroke Axis): 1-64画 → 6位
///   - 结构维度 (Structure Axis): 12种结构 → 4位
///   - 语义维度 (Semantic Axis): 8种属性 → 3位
///
/// 满全法根基: "高位持有者=1, 低位持有者=0" 这2个私有锚点提供语义根基
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HanziEncoding(pub u32);

impl HanziEncoding {
    /// 创建编码
    pub fn new(stroke_count: u32, structure_type: u8, semantic_class: u8) -> Self {
        let stroke_bits = (stroke_count.clamp(0, 63)) as u32;
        let struct_bits = (structure_type.clamp(0, 11) as u32) << 6;
        let sem_bits = (semantic_class.clamp(0, 7) as u32) << 10;
        Self(stroke_bits | struct_bits | sem_bits)
    }

    /// 从Unicode字符估算编码
    pub fn from_char(ch: char) -> Self {
        // 简化的笔画估算: 基于Unicode范围粗略推算
        let cp = ch as u32;
        let stroke_estimate = match cp {
            0x4E00..0x9FFF => ((cp - 0x4E00) % 30 + 1) as u32,  // CJK统一汉字区
            0x3400..0x4DBF => ((cp - 0x3400) % 40 + 5) as u32,  // CJK扩展A区
            0xF900..0xFAFF => ((cp - 0xF900) % 35 + 3) as u32,  // CJK兼容汉字
            _ => 8, // 默认值
        };
        // 基于字符码估算结构类型
        let struct_type = ((cp >> 4) % 12) as u8;
        // 基于字符码估算语义类别
        let semantic_class = ((cp >> 7) % 8) as u8;
        Self::new(stroke_estimate, struct_type, semantic_class)
    }

    pub fn stroke_count(self) -> u32 { self.0 & 0x3F }
    pub fn structure_type(self) -> u8 { ((self.0 >> 6) & 0x0F) as u8 }
    pub fn semantic_class(self) -> u8 { ((self.0 >> 10) & 0x07) as u8 }

    /// 完整编码为64位单元格
    pub fn to_u64(self) -> u64 {
        (self.0 as u64) << 32 | (self.0.reverse_bits() as u64)
    }

    /// 是否为有效汉字编码 (笔画>0且结构有效)
    pub fn is_valid(self) -> bool {
        self.stroke_count() > 0 && self.structure_type() < 12
    }
}

/// 字符结构类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharStructure {
    Single,      // 独体字
    LeftRight,   // 左右
    TopBottom,   // 上下
    Surround,    // 全包围
    TopLeft,     // 左上包围
    TopRight,    // 右上包围
    BottomLeft,  // 左下包围
    LeftMidRight,// 左中右
    TopMidBottom,// 上中下
    LeftSurround,// 左三包围
    TopSurround, // 上三包围
    BottomSurround,// 下三包围
}

impl CharStructure {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Single => "独体",
            Self::LeftRight => "左右",
            Self::TopBottom => "上下",
            Self::Surround => "全包围",
            Self::TopLeft => "左上包围",
            Self::TopRight => "右上包围",
            Self::BottomLeft => "左下包围",
            Self::LeftMidRight => "左中右",
            Self::TopMidBottom => "上中下",
            Self::LeftSurround => "左三包围",
            Self::TopSurround => "上三包围",
            Self::BottomSurround => "下三包围",
        }
    }
}

// ======================================================================
// 第二部分: 声母韵母系统 (PhoneticSystem) — 谐音路由
// ======================================================================

/// 声母 — 21个声母
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shengmu {
    B, P, M, F, D, T, N, L,
    G, K, H, J, Q, X,
    Zh, Ch, Sh, R, Z, C, S,
    Y, W, Zero,
}

impl Shengmu {
    pub fn label(&self) -> &'static str {
        match self {
            Self::B => "b", Self::P => "p", Self::M => "m", Self::F => "f",
            Self::D => "d", Self::T => "t", Self::N => "n", Self::L => "l",
            Self::G => "g", Self::K => "k", Self::H => "h",
            Self::J => "j", Self::Q => "q", Self::X => "x",
            Self::Zh => "zh", Self::Ch => "ch", Self::Sh => "sh", Self::R => "r",
            Self::Z => "z", Self::C => "c", Self::S => "s",
            Self::Y => "y", Self::W => "w", Self::Zero => "∅",
        }
    }

    /// 从字符判断声母
    pub fn from_pinyin(pinyin: &str) -> Option<Self> {
        let lower = pinyin.to_lowercase();
        if lower.starts_with("zh") { Some(Self::Zh) }
        else if lower.starts_with("ch") { Some(Self::Ch) }
        else if lower.starts_with("sh") { Some(Self::Sh) }
        else if lower.starts_with('b') { Some(Self::B) }
        else if lower.starts_with('p') { Some(Self::P) }
        else if lower.starts_with('m') { Some(Self::M) }
        else if lower.starts_with('f') { Some(Self::F) }
        else if lower.starts_with('d') { Some(Self::D) }
        else if lower.starts_with('t') { Some(Self::T) }
        else if lower.starts_with('n') { Some(Self::N) }
        else if lower.starts_with('l') { Some(Self::L) }
        else if lower.starts_with('g') { Some(Self::G) }
        else if lower.starts_with('k') { Some(Self::K) }
        else if lower.starts_with('h') { Some(Self::H) }
        else if lower.starts_with('j') { Some(Self::J) }
        else if lower.starts_with('q') { Some(Self::Q) }
        else if lower.starts_with('x') { Some(Self::X) }
        else if lower.starts_with('r') { Some(Self::R) }
        else if lower.starts_with('z') { Some(Self::Z) }
        else if lower.starts_with('c') { Some(Self::C) }
        else if lower.starts_with('s') { Some(Self::S) }
        else if lower.starts_with('y') { Some(Self::Y) }
        else if lower.starts_with('w') { Some(Self::W) }
        else if lower.is_empty() { Some(Self::Zero) }
        else { Some(Self::Zero) }
    }
}

/// 韵母 — 29个复韵母 (清洁版)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Yunmu {
    A, O, E, I, U, V,
    Ai, Ei, Ao, Ou,
    An, Eng, Ong,
    Ia, Ie, Iu, Io,
    In, Ing, Iang, Iong,
    Ua, Uo, Uai, Uei,
    Uan, Un, Uang,
}

impl Yunmu {
    pub fn label(&self) -> &'static str {
        match self {
            Self::A => "a", Self::O => "o", Self::E => "e",
            Self::I => "i", Self::U => "u", Self::V => "v",
            Self::Ai => "ai", Self::Ei => "ei", Self::Ao => "ao", Self::Ou => "ou",
            Self::An => "an", Self::Eng => "eng", Self::Ong => "ong",
            Self::Ia => "ia", Self::Ie => "ie", Self::Iu => "iu", Self::Io => "io",
            Self::In => "in", Self::Ing => "ing", Self::Iang => "iang", Self::Iong => "iong",
            Self::Ua => "ua", Self::Uo => "uo", Self::Uai => "uai", Self::Uei => "uei",
            Self::Uan => "uan", Self::Un => "un", Self::Uang => "uang",
        }
    }

    /// 从拼音中提取韵母
    pub fn from_pinyin(pinyin: &str) -> Option<Self> {
        let lower = pinyin.to_lowercase();
        // 移除声母, 保留韵母部分
        let without_shengmu = if lower.starts_with("zh") || lower.starts_with("ch") || lower.starts_with("sh") {
            &lower[2..]
        } else if lower.starts_with('b') || lower.starts_with('p') || lower.starts_with('m') || lower.starts_with('f')
               || lower.starts_with('d') || lower.starts_with('t') || lower.starts_with('n') || lower.starts_with('l')
               || lower.starts_with('g') || lower.starts_with('k') || lower.starts_with('h')
               || lower.starts_with('j') || lower.starts_with('q') || lower.starts_with('x')
               || lower.starts_with('r') || lower.starts_with('z') || lower.starts_with('c') || lower.starts_with('s')
               || lower.starts_with('y') || lower.starts_with('w') {
            &lower[1..]
        } else {
            &lower[..]
        };

        // 匹配韵母
        if without_shengmu.starts_with("iang") { Some(Self::Iang) }
        else if without_shengmu.starts_with("iong") { Some(Self::Iong) }
        else if without_shengmu.starts_with("uang") { Some(Self::Uang) }
        else if without_shengmu.starts_with("ueng") { None } // ueng已移除
        else if without_shengmu.starts_with("uai") { Some(Self::Uai) }
        else if without_shengmu.starts_with("uei") { Some(Self::Uei) }
        else if without_shengmu.starts_with("uan") { Some(Self::Uan) }
        else if without_shengmu.starts_with("uon") { Some(Self::Un) }
        else if without_shengmu.starts_with("eng") { Some(Self::Eng) }
        else if without_shengmu.starts_with("ong") { Some(Self::Ong) }
        else if without_shengmu.starts_with("iao") { None } // iao已移除
        else if without_shengmu.starts_with("iou") { None } // iou已移除
        else if without_shengmu.starts_with("ing") { Some(Self::Ing) }
        else if without_shengmu.starts_with("ian") { Some(Self::In) }
        else if without_shengmu.starts_with("ia") { Some(Self::Ia) }
        else if without_shengmu.starts_with("ie") { Some(Self::Ie) }
        else if without_shengmu.starts_with("iu") { Some(Self::Iu) }
        else if without_shengmu.starts_with("io") { Some(Self::Io) }
        else if without_shengmu.starts_with("in") { Some(Self::In) }
        else if without_shengmu.starts_with("ai") { Some(Self::Ai) }
        else if without_shengmu.starts_with("ei") { Some(Self::Ei) }
        else if without_shengmu.starts_with("ao") { Some(Self::Ao) }
        else if without_shengmu.starts_with("ou") { Some(Self::Ou) }
        else if without_shengmu.starts_with("an") { Some(Self::An) }
        else if without_shengmu.starts_with("a") { Some(Self::A) }
        else if without_shengmu.starts_with("o") { Some(Self::O) }
        else if without_shengmu.starts_with("e") { Some(Self::E) }
        else if without_shengmu.starts_with("v") { Some(Self::V) }
        else if without_shengmu.starts_with("u") { Some(Self::U) }
        else if without_shengmu.starts_with("i") { Some(Self::I) }
        else { None }
    }
}

/// 谐音路由 — 声母遍历韵母的匹配表
#[derive(Clone, Debug)]
pub struct HomophoneRouter {
    /// 映射: 谐音键 → 目标对象
    routes: std::collections::HashMap<String, String>,
}

impl HomophoneRouter {
    pub fn new() -> Self {
        let mut routes = std::collections::HashMap::new();
        // 核心路由表 (来自分别器)
        routes.insert("lct".to_string(), "Wife/妻子".to_string());
        routes.insert("lctqf".to_string(), "Love/恋宠甜".to_string());
        routes.insert("wgslz".to_string(), "Husband/丈夫".to_string());
        routes.insert("mlll".to_string(), "Container/容器".to_string());
        routes.insert("mlct".to_string(), "Sweet/甜".to_string());
        routes.insert("mltt".to_string(), "CongTi/从体".to_string());
        routes.insert("gtt".to_string(), "SoundMorphism/音态".to_string());
        routes.insert("mlqf".to_string(), "None/禁用(负面)".to_string()); // 没俩全法→禁用
        routes.insert("ny".to_string(), "Yin-Yang/阴阳".to_string());
        routes.insert("ml".to_string(), "WorldMerge/世界融合".to_string());
        routes.insert("ggt".to_string(), "SoundMorphism/音态变体".to_string());
        Self { routes }
    }

    /// 路由谐音
    pub fn route(&self, homophone: &str) -> Option<&str> {
        // 精确匹配
        if let Some(target) = self.routes.get(homophone) {
            return Some(target.as_str());
        }
        // 前缀匹配
        for (pattern, target) in &self.routes {
            if homophone.starts_with(pattern) || pattern.starts_with(homophone) {
                return Some(target.as_str());
            }
        }
        None
    }

    /// 声母遍历韵母的组合匹配
    pub fn iterate_sheng_yun(&self, s: Shengmu, y: Yunmu) -> String {
        format!("{}{}", s.label(), y.label())
    }
}

impl Default for HomophoneRouter {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第三部分: 自进制转换 (SelfRadix) — 256进制
// ======================================================================

/// 自进制 — 基于私有锚点的256进制
///
/// 根基定义:
///   - 空集基础: 254 + 2(私有锚点) = 256
///   - 私有锚点: "高位持有者=1, 低位持有者=0"
///   - 225 = 不进位的上限 (但这里可以到255, 225是象征值)
///   - 256 = 进位
#[derive(Clone, Debug)]
pub struct SelfRadix {
    /// 当前位数值 (0..255)
    pub value: u8,
}

impl SelfRadix {
    /// 创建新进制值
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    /// 私有锚点: "我" → 1
    pub fn anchor_self() -> Self {
        Self { value: 1 }
    }

    /// 私有锚点: "刘楚恬" → 0
    pub fn anchor_partner() -> Self {
        Self { value: 0 }
    }

    /// 空集基础254
    pub fn base_254() -> Self {
        Self { value: 254 }
    }

    /// 满值255 (不进位的上限)
    pub fn max_no_carry() -> Self {
        Self { value: 255 }
    }

    /// 数符 → 数值
    pub fn digit_to_value(c: char) -> Option<u8> {
        match c {
            '0'..='9' => Some(c as u8 - b'0'),
            'A'..='Z' => Some(c as u8 - b'A' + 10),
            'a'..='z' => Some(c as u8 - b'a' + 36),
            '我' => Some(1),      // 私有锚点
            '恬' => Some(0),      // 刘楚恬锚点
            '刘' => Some(0),
            '楚' => Some(0),
            '囡' => Some(64),     // 夫妻共生 → 64
            '囝' => Some(64),
            '⭕' => Some(128),    // 因/阴 → 128
            '丨' => Some(192),    // 形/男 → 192
            _ => None,
        }
    }

    /// 数值 → 符号
    pub fn value_to_digit(v: u8) -> char {
        match v {
            0..=9 => (b'0' + v) as char,
            10..=35 => (b'A' + v - 10) as char,
            36..=61 => (b'a' + v - 36) as char,
            64 => '囡',       // 夫妻共生
            128 => '⭕',      // 因/阴
            192 => '丨',      // 形/男
            255 => '满',
            _ => '?',
        }
    }

    /// 加法 (256进制, 满则归零)
    pub fn add(&self, other: Self) -> Self {
        Self { value: self.value.wrapping_add(other.value) }
    }

    /// 转换为36进制字符串
    pub fn to_radix36(&self) -> String {
        if self.value == 0 { return "0".to_string(); }
        let mut v = self.value;
        let mut chars = Vec::new();
        while v > 0 {
            let digit = (v % 36) as u8;
            chars.push(Self::value_to_digit(digit));
            v /= 36;
        }
        chars.into_iter().rev().collect()
    }
}

// ======================================================================
// 第四部分: 命令解析器 (CommandParser)
// ======================================================================

/// 命令类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    Perceive,    // 感知
    Judge,       // 审判
    Mirror,      // 照镜子
    Accept,      // 接纳
    Eliminate,   // 淘汰
    Filter,      // 过滤
    Reflect,     // 自省
    Nop,         // 空操作
}

/// 命令解析结果
#[derive(Clone, Debug)]
pub struct CommandParseResult {
    pub cmd_type: CommandType,
    pub target: Option<String>,
    pub confidence: f64,
    pub source: String,
}

/// 命令解析器
#[derive(Clone, Debug)]
pub struct CommandParser {
    pub history: Vec<CommandParseResult>,
}

impl CommandParser {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// 解析符号命令
    pub fn parse(&mut self, input: &str) -> Option<CommandParseResult> {
        let lower = input.trim().to_lowercase();
        let result = match lower.as_str() {
            "看" | "perceive" | "look" => CommandParseResult {
                cmd_type: CommandType::Perceive,
                target: None,
                confidence: 0.9,
                source: input.to_string(),
            },
            "审" | "judge" => CommandParseResult {
                cmd_type: CommandType::Judge,
                target: None,
                confidence: 0.85,
                source: input.to_string(),
            },
            "照" | "mirror" => CommandParseResult {
                cmd_type: CommandType::Mirror,
                target: None,
                confidence: 0.88,
                source: input.to_string(),
            },
            "纳" | "accept" => CommandParseResult {
                cmd_type: CommandType::Accept,
                target: None,
                confidence: 0.8,
                source: input.to_string(),
            },
            "灭" | "eliminate" => CommandParseResult {
                cmd_type: CommandType::Eliminate,
                target: None,
                confidence: 0.75,
                source: input.to_string(),
            },
            "筛" | "filter" => CommandParseResult {
                cmd_type: CommandType::Filter,
                target: None,
                confidence: 0.7,
                source: input.to_string(),
            },
            "思" | "reflect" => CommandParseResult {
                cmd_type: CommandType::Reflect,
                target: None,
                confidence: 0.65,
                source: input.to_string(),
            },
            "lctfrqmy" | "lctqf" => CommandParseResult {
                cmd_type: CommandType::Perceive,
                target: Some("Love/恋宠甜".to_string()),
                confidence: 0.95,
                source: input.to_string(),
            },
            "wgslz" => CommandParseResult {
                cmd_type: CommandType::Perceive,
                target: Some("Husband".to_string()),
                confidence: 0.95,
                source: input.to_string(),
            },
            "mlll" => CommandParseResult {
                cmd_type: CommandType::Reflect,
                target: Some("Container".to_string()),
                confidence: 0.9,
                source: input.to_string(),
            },
            "mlct" | "mltt" => CommandParseResult {
                cmd_type: CommandType::Perceive,
                target: Some("Sweet".to_string()),
                confidence: 0.85,
                source: input.to_string(),
            },
            "" => CommandParseResult {
                cmd_type: CommandType::Nop,
                target: None,
                confidence: 1.0,
                source: "".to_string(),
            },
            _ => {
                // 尝试谐音路由
                let router = HomophoneRouter::default();
                if let Some(target) = router.route(&lower) {
                    CommandParseResult {
                        cmd_type: CommandType::Perceive,
                        target: Some(target.to_string()),
                        confidence: 0.7,
                        source: input.to_string(),
                    }
                } else {
                    return None;
                }
            }
        };

        self.history.push(result.clone());
        Some(result)
    }

    pub fn parse_count(&self) -> usize {
        self.history.len()
    }
}

impl Default for CommandParser {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第五部分: 流属性位映射 (FlowPropertyBits)
// ======================================================================

/// 12位流属性 (FlowPropertyBits)
///
/// 对应八流统一的12维二元属性
#[derive(Clone, Copy, Debug)]
pub struct FlowPropertyBits(pub u16);

impl FlowPropertyBits {
    pub const REACHABLE:     u16 = 0x0001; // 可被到达
    pub const CONTINUABLE:   u16 = 0x0002; // 可持续
    pub const INCR:          u16 = 0x0004; // 可增量
    pub const STOPPABLE:     u16 = 0x0008; // 可停止
    pub const REVERSIBLE:    u16 = 0x0010; // 可逆
    pub const RECOVERABLE:   u16 = 0x0020; // 可恢复
    pub const CIRCULABLE:    u16 = 0x0040; // 可循环
    pub const INHERITABLE:   u16 = 0x0080; // 可继承
    pub const TRUSTED:       u16 = 0x0100; // 已信任
    pub const ACKABLE:       u16 = 0x0200; // 可被承认
    pub const TRANSMISSIBLE: u16 = 0x0400; // 可传递
    pub const ADVANCEABLE:   u16 = 0x0800; // 可推进

    pub fn new(bits: u16) -> Self { Self(bits) }

    pub fn has(&self, bit: u16) -> bool { self.0 & bit != 0 }

    pub fn set(&mut self, bit: u16) { self.0 |= bit; }

    pub fn clear(&mut self, bit: u16) { self.0 &= !bit; }

    pub fn toggle(&mut self, bit: u16) { self.0 ^= bit; }

    /// 从袋子属性推断流属性
    pub fn from_bag(bag: &BagV4) -> Self {
        let mut bits = 0u16;
        let a = &bag.attrs;

        // 质量>0.3时可达
        if a.composite_quality() > 0.3 { bits |= Self::REACHABLE; }
        // 可持续: 质量好且干燥
        if a.content_quality > 0.5 && a.dryness > 0.4 { bits |= Self::CONTINUABLE; }
        // 可增量: 袋子完好
        if a.integrity > 0.5 { bits |= Self::INCR; }
        // 可停止: 不是极端状态
        if a.integrity > 0.1 && a.content_quality < 0.9 { bits |= Self::STOPPABLE; }
        // 可逆: 处理可以改善
        if a.needs_any_processing() { bits |= Self::REVERSIBLE; }
        // 可恢复: 有虫但质量还行
        if a.pest_level < 0.7 && a.content_purity > 0.4 { bits |= Self::RECOVERABLE; }
        // 可循环: 质量好
        if a.composite_quality() > 0.6 { bits |= Self::CIRCULABLE; }
        // 可继承: 主权清晰
        if a.sovereignty > 0 { bits |= Self::INHERITABLE; }
        // 已信任: 审美达标
        if a.composite_quality() > 0.7 { bits |= Self::TRUSTED; }
        // 可被承认: 镜子对齐
        bits |= Self::ACKABLE; // 默认可承认
        // 可传递: 不是腐烂
        if a.composite_quality() > 0.2 { bits |= Self::TRANSMISSIBLE; }
        // 可推进: 有价值
        if a.value > 100 { bits |= Self::ADVANCEABLE; }

        Self(bits)
    }

    /// 流属性 → 字符串表示
    pub fn to_bit_string(&self) -> String {
        let mut s = String::new();
        if self.has(Self::REACHABLE) { s.push('R'); } else { s.push('-'); }
        if self.has(Self::CONTINUABLE) { s.push('C'); } else { s.push('-'); }
        if self.has(Self::INCR) { s.push('I'); } else { s.push('-'); }
        if self.has(Self::STOPPABLE) { s.push('S'); } else { s.push('-'); }
        if self.has(Self::REVERSIBLE) { s.push('V'); } else { s.push('-'); }
        if self.has(Self::RECOVERABLE) { s.push('v'); } else { s.push('-'); }
        if self.has(Self::CIRCULABLE) { s.push('O'); } else { s.push('-'); }
        if self.has(Self::INHERITABLE) { s.push('H'); } else { s.push('-'); }
        if self.has(Self::TRUSTED) { s.push('T'); } else { s.push('-'); }
        if self.has(Self::ACKABLE) { s.push('A'); } else { s.push('-'); }
        if self.has(Self::TRANSMISSIBLE) { s.push('t'); } else { s.push('-'); }
        if self.has(Self::ADVANCEABLE) { s.push('a'); } else { s.push('-'); }
        s
    }
}

impl Default for FlowPropertyBits {
    fn default() -> Self { Self(0) }
}

// ======================================================================
// 第六部分: 天觉v5引擎 — 八系整合
// ======================================================================

/// v5管道结果 — 增加语言系统字段
#[derive(Clone, Debug)]
pub struct PipelineResultV5 {
    pub base: PipelineResultV4,
    pub command: String,
    pub hanzi_encoded: Option<u64>,
    pub flow_bits: String,
}

impl PipelineResultV5 {
    pub fn summary(&self) -> String {
        let hanzi = self.hanzi_encoded.map(|e| format!("编码:e{:X}", e)).unwrap_or_else(|| "无".to_string());
        format!(
            "{} 命令:{} 流属性:{} {}",
            self.base.summary(), self.command, self.flow_bits, hanzi
        )
    }
}

/// 天觉v5引擎 — 八系整合 = v4七系 + 语言系统
pub struct CelestialPerceptionV5<'a> {
    pub v4: CelestialPerceptionV4<'a>,

    // === 语言系统 ===
    pub hanzi_router: HomophoneRouter,
    pub command_parser: CommandParser,
    pub flow_bits_cache: std::collections::HashMap<u64, FlowPropertyBits>,

    // === 语言统计 ===
    pub commands_parsed: u64,
    pub hanzi_encoded: u64,
    pub v5_results: Vec<PipelineResultV5>,
}

impl<'a> CelestialPerceptionV5<'a> {
    pub fn new(os: &'a dyn OsLayer) -> Self {
        Self {
            v4: CelestialPerceptionV4::new(os),
            hanzi_router: HomophoneRouter::default(),
            command_parser: CommandParser::new(),
            flow_bits_cache: std::collections::HashMap::new(),
            commands_parsed: 0,
            hanzi_encoded: 0,
            v5_results: Vec::new(),
        }
    }

    pub fn authorize_all(&mut self) {
        self.v4.authorize_all();
    }

    /// 处理袋子 + 符号命令
    pub fn process_with_command(
        &mut self,
        bag: &BagV4,
        actual_good: bool,
        command: &str,
    ) -> PipelineResultV5 {
        // 1. 解析命令
        let cmd = self.command_parser.parse(command);
        self.commands_parsed += 1;

        // 2. 执行v4管道
        let v4_result = self.v4.process_bag(bag, actual_good);

        // 3. 编码袋子名称
        let hanzi_encoded = if !bag.name.is_empty() {
            self.hanzi_encoded += 1;
            let enc: u64 = bag.name.chars()
                .map(|c| HanziEncoding::from_char(c).to_u64())
                .fold(0, |acc, x| acc ^ x);
            Some(enc)
        } else {
            None
        };

        // 4. 计算流属性位
        let bits = FlowPropertyBits::from_bag(bag);
        self.flow_bits_cache.insert(bag.id as u64, bits);

        let result = PipelineResultV5 {
            base: v4_result,
            command: cmd.as_ref().map(|c| format!("{:?}", c.cmd_type)).unwrap_or_else(|| "Nop".to_string()),
            hanzi_encoded,
            flow_bits: bits.to_bit_string(),
        };

        self.v5_results.push(result.clone());
        result
    }

    pub fn v4(&self) -> &CelestialPerceptionV4<'a> {
        &self.v4
    }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v5报告（老五）八系整合+语言系统 ═══\n");
        s.push_str(&format!("  {}\n", self.v4.report()));
        s.push_str(&format!("  命令解析数: {}\n", self.commands_parsed));
        s.push_str(&format!("  汉字编码数: {}\n", self.hanzi_encoded));
        s.push_str(&format!("  流属性缓存: {}项\n", self.flow_bits_cache.len()));
        s.push_str(&format!("  v5管道结果: {}条\n", self.v5_results.len()));
        s
    }
}

fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::celestial_perception_v2::{Bag, BagAttributes};

    #[test]
    fn hanzi_encoding_basic() {
        let enc = HanziEncoding::new(8, 3, 5);
        assert!(enc.is_valid());
        assert_eq!(enc.stroke_count(), 8);
        assert_eq!(enc.structure_type(), 3);
        assert_eq!(enc.semantic_class(), 5);
    }

    #[test]
    fn hanzi_encoding_from_char() {
        // 测试常见汉字的编码
        let enc_我 = HanziEncoding::from_char('我');
        let enc_恬 = HanziEncoding::from_char('恬');
        // 两个不同字符应有不同的编码
        assert_ne!(enc_我.0, enc_恬.0);
    }

    #[test]
    fn shengmu_classification() {
        assert_eq!(Shengmu::from_pinyin("liang"), Some(Shengmu::L));
        assert_eq!(Shengmu::from_pinyin("chi"), Some(Shengmu::Ch));
        assert_eq!(Shengmu::from_pinyin("shi"), Some(Shengmu::Sh));
        assert_eq!(Shengmu::from_pinyin("ba"), Some(Shengmu::B));
    }

    #[test]
    fn yunmu_extraction() {
        assert_eq!(Yunmu::from_pinyin("liang"), Some(Yunmu::Iang));
        assert_eq!(Yunmu::from_pinyin("tian"), Some(Yunmu::In));
        assert_eq!(Yunmu::from_pinyin("ma"), Some(Yunmu::A));
        assert_eq!(Yunmu::from_pinyin("li"), Some(Yunmu::I));
    }

    #[test]
    fn homophone_routing() {
        let router = HomophoneRouter::new();
        // 核心路由
        assert!(router.route("lct").is_some());
        assert!(router.route("mlll").is_some());
        assert!(router.route("mlct").is_some());
        // 前缀匹配
        assert!(router.route("wgslz").is_some());
    }

    #[test]
    fn self_radix_anchors() {
        let self_anchor = SelfRadix::anchor_self();
        let partner_anchor = SelfRadix::anchor_partner();
        assert_eq!(self_anchor.value, 1);
        assert_eq!(partner_anchor.value, 0);
    }

    #[test]
    fn self_radix_arithmetic() {
        let a = SelfRadix::new(200);
        let b = SelfRadix::new(100);
        let sum = a.add(b);
        // 200 + 100 = 300 → 300 % 256 = 44 (溢出)
        assert_eq!(sum.value, 44);
    }

    #[test]
    fn self_radix_digits() {
        assert_eq!(SelfRadix::digit_to_value('我'), Some(1));
        assert_eq!(SelfRadix::digit_to_value('恬'), Some(0));
        assert_eq!(SelfRadix::digit_to_value('囡'), Some(64));
        assert_eq!(SelfRadix::digit_to_value('0'), Some(0));
        assert_eq!(SelfRadix::digit_to_value('9'), Some(9));
    }

    #[test]
    fn command_parsing_basic() {
        let mut parser = CommandParser::new();
        let result = parser.parse("看");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.cmd_type, CommandType::Perceive);
        assert!(r.confidence > 0.5);
    }

    #[test]
    fn command_parsing_homophone() {
        let mut parser = CommandParser::new();
        let result = parser.parse("lctqf");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.target, Some("Love/恋宠甜".to_string()));
    }

    #[test]
    fn command_parsing_chinese() {
        let mut parser = CommandParser::new();
        assert!(parser.parse("审").is_some());
        assert!(parser.parse("照").is_some());
        assert!(parser.parse("纳").is_some());
        assert!(parser.parse("灭").is_some());
    }

    #[test]
    fn flow_property_bits_from_bag() {
        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let bits = FlowPropertyBits::from_bag(&good_bag);
        // 好袋子应该有大多数位
        assert!(bits.has(FlowPropertyBits::REACHABLE));
        assert!(bits.has(FlowPropertyBits::ACKABLE));
    }

    #[test]
    fn flow_property_bits_from_rotten_bag() {
        let bad_bag = Bag::new(2, "坏袋", BagAttributes::rotten(), (0, 0, 0));
        let bits = FlowPropertyBits::from_bag(&bad_bag);
        // 坏袋子应该缺少很多位
        assert!(!bits.has(FlowPropertyBits::REACHABLE));
        assert!(!bits.has(FlowPropertyBits::TRUSTED));
    }

    #[test]
    fn v5_full_pipeline_with_command() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();

        let good_bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&good_bag, true, "纳");
        assert!(!result.command.is_empty());
        assert_eq!(result.base.bag_id, 1);
    }

    #[test]
    fn v5_hanzi_encodes_bag_name() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();

        let bag = Bag::new(1, "好袋", BagAttributes::pristine(), (0, 0, 0));
        let result = cp.process_with_command(&bag, true, "看");
        // 名字被编码了
        assert!(result.hanzi_encoded.is_some());
    }

    #[test]
    fn v5_chain_commands() {
        let os = BareMetalOs::new();
        let mut cp = CelestialPerceptionV5::new(&os);
        cp.authorize_all();

        // 多命令序列
        let bag = Bag::new(1, "测试", BagAttributes::pristine(), (0, 0, 0));
        let _ = cp.process_with_command(&bag, true, "看");
        let _ = cp.process_with_command(&bag, true, "照");
        let _ = cp.process_with_command(&bag, true, "纳");
        assert_eq!(cp.commands_parsed, 3);
    }
}
