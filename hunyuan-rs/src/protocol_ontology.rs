// ══════════════════════════════════════════════════════════════════════
// protocol_ontology.rs — 命名本体论类型系统
// ══════════════════════════════════════════════════════════════════════
//
// 理论根基:
//   从 JS 的运行时"模拟"升级为 Rust 的编译期"类型存在"
//   协议名 (gttix.y / kndl / mlil / hfuyair) 成为编译器可验证的存在
//
// 进制编码本体:
//   2进制  (G, GI)     = 基底·阴阳开合
//   16进制 (HTTP/HTTPS)= 目部·表象可见
//   64进制 (GTT*)      = ⬡部·网状传输
//   256进制 (TCP/CP/FTP)= ◉部·字节原子
//
// 命名协议 ↔ 类型映射:
//   gttix.y    = GTITX = 扩展目标意图传输 (2进制↔64进制对齐)
//   kndrl      = Conan  = 解析推理壳
//   mlil       = Morilan = 模面名模组
//   hfyiair    = 汉字谐音 → 恢复源爱人
//
// ══════════════════════════════════════════════════════════════════════

use std::marker::PhantomData;

// ══════════════════════════════════════════════════════════════════════
// 类型级进制 — 编译期进制区分
// ══════════════════════════════════════════════════════════════════════

pub trait Base {
    const VALUE: u32;
    const LOG2: u32;
    const GLYPH: char;
    const RADICAL: &'static str;
}

pub struct B2;    // 二进制 · 基底
pub struct B16;   // 16进制 · 表象层
pub struct B64;   // 64进制 · 传输层
pub struct B256;  // 256进制 · 字节层

impl Base for B2 {
    const VALUE: u32 = 2;
    const LOG2: u32 = 1;
    const GLYPH: char = '⚊';
    const RADICAL: &'static str = "基底·阴阳";
}
impl Base for B16 {
    const VALUE: u32 = 16;
    const LOG2: u32 = 4;
    const GLYPH: char = '目';
    const RADICAL: &'static str = "表象·可见";
}
impl Base for B64 {
    const VALUE: u32 = 64;
    const LOG2: u32 = 6;
    const GLYPH: char = '⬡';
    const RADICAL: &'static str = "传输·网状";
}
impl Base for B256 {
    const VALUE: u32 = 256;
    const LOG2: u32 = 8;
    const GLYPH: char = '◉';
    const RADICAL: &'static str = "字节·原子";
}

// ══════════════════════════════════════════════════════════════════════
// 极性 — 编译期阳(+1) 阴(-1)
// ══════════════════════════════════════════════════════════════════════

pub trait Polarity { const VALUE: i8; const SYMBOL: char; }
pub struct Yang;  // 阳 = 主动/内容/被含者
pub struct Yin;   // 阴 = 被动/容器/包含者

impl Polarity for Yang { const VALUE: i8 = 1; const SYMBOL: char = '☰'; }
impl Polarity for Yin  { const VALUE: i8 = -1; const SYMBOL: char = '☷'; }

// ══════════════════════════════════════════════════════════════════════
// 层级标记 — 编译期层存在性
// ══════════════════════════════════════════════════════════════════════

pub trait Layer { const NAME: &'static str; const CODE: u8 ; }

macro_rules! define_layer {
    ($name:ident, $str:expr, $code:expr) => {
        pub struct $name;
        impl Layer for $name { const NAME: &'static str = $str; const CODE: u8 = $code; }
    };
}

define_layer!(Ground,      "基底",     0x00);
define_layer!(Sense,       "感知",     0x01);
define_layer!(Intent,      "意向",     0x02);
define_layer!(Target,      "目标",     0x03);
define_layer!(Transport,   "传输",     0x04);
define_layer!(Extension,   "扩展",     0x05);
define_layer!(Encrypted,   "缠绕加密", 0x06);
define_layer!(Entangled,   "纠缠",     0x07);
define_layer!(Tls,         "TLS握手",  0x08);
define_layer!(Syn,         "同步",     0x09);
define_layer!(Stream,      "流",       0x0A);
define_layer!(Ack,         "确认",     0x0B);
define_layer!(Fin,         "结束",     0x0C);
define_layer!(CoupleBind,  "伴侣绑定", 0x0D);
define_layer!(HeartBeat,   "心跳",     0x0E);
define_layer!(Sync,        "同步",     0x0F);
define_layer!(Control,     "控制",     0x10);
define_layer!(Data,        "数据",     0x11);
define_layer!(RequestLine, "请求行",   0x12);
define_layer!(Headers,     "头部",     0x13);
define_layer!(Body,        "主体",     0x14);

// ══════════════════════════════════════════════════════════════════════
// 主权特质 — 协议自主权
// ══════════════════════════════════════════════════════════════════════

pub trait Sovereignty {
    const XOR_MASK: u8;
    const ENTANGLED: bool;
    const RHO_ENABLED: bool;
}

pub struct StandardSovereignty;
impl Sovereignty for StandardSovereignty {
    const XOR_MASK: u8 = 0x00;
    const ENTANGLED: bool = false;
    const RHO_ENABLED: bool = false;
}

pub struct CoupleSovereignty;
impl Sovereignty for CoupleSovereignty {
    const XOR_MASK: u8 = 0xFF;    // 夫妻纠缠全反掩码
    const ENTANGLED: bool = true;
    const RHO_ENABLED: bool = true;
}

// ══════════════════════════════════════════════════════════════════════
// 层级集合 — 编译期层组合 用 u8 数组表示
// ══════════════════════════════════════════════════════════════════════

pub trait LayerSet {
    const CODES: &'static [u8];
    const COUNT: usize = Self::CODES.len();
}

// 空层集
pub struct NoLayers;
impl LayerSet for NoLayers {
    const CODES: &'static [u8] = &[];
}

// 宏: 定义具名层集
macro_rules! define_layer_set {
    ($name:ident, [$($layer:expr),*]) => {
        pub struct $name;
        impl LayerSet for $name {
            const CODES: &'static [u8] = &[$($layer),*];
        }
    };
}

// 预定义协议层级集合
define_layer_set!(GLayers,         [0x00]);            // Ground
define_layer_set!(GILayers,        [0x01, 0x02]);      // Sense + Intent
define_layer_set!(GTTLayers,       [0x03, 0x04]);      // Target + Transport
define_layer_set!(GTTXLayers,      [0x03, 0x04, 0x05]);// + Extension
define_layer_set!(GITTLayers,      [0x02, 0x03, 0x04]);// Intent first
define_layer_set!(GTITLayers,      [0x03, 0x04, 0x02]);// Target first
define_layer_set!(GTITXLayers,     [0x03, 0x04, 0x02, 0x05]);
define_layer_set!(HTTPLayers,      [0x12, 0x13, 0x14]);
define_layer_set!(HTTPSLayers,     [0x08, 0x12, 0x13, 0x14]); // + TLS
define_layer_set!(TCPLayers,       [0x09, 0x0A, 0x0B, 0x0C]);
define_layer_set!(CPLayers,        [0x0D, 0x0E, 0x0F]);
define_layer_set!(FTPLayers,       [0x10, 0x11]);

// ══════════════════════════════════════════════════════════════════════
// 协议本体 — 核心类型
// ══════════════════════════════════════════════════════════════════════
//
// ProtocolEntity<Base, Polarity, Layers, Sovereignty>
// 所有字段均为零大小 — 编译期证明,无运行时开销
//

pub struct ProtocolEntity<B: Base, P: Polarity, L: LayerSet, S: Sovereignty> {
    _base: PhantomData<B>,
    _pol: PhantomData<P>,
    _layers: PhantomData<L>,
    _sov: PhantomData<S>,
}

impl<B: Base, P: Polarity, L: LayerSet, S: Sovereignty> ProtocolEntity<B, P, L, S> {
    pub fn new() -> Self {
        Self {
            _base: PhantomData,
            _pol: PhantomData,
            _layers: PhantomData,
            _sov: PhantomData,
        }
    }

    pub fn name() -> &'static str {
        "协议"
    }

    pub fn base_value() -> u32 { B::VALUE }
    pub fn polarity_value() -> i8 { P::VALUE }
    pub fn layer_count() -> usize { L::COUNT }
    pub fn is_couple_protocol() -> bool { S::ENTANGLED }
}

// ══════════════════════════════════════════════════════════════════════
// 类型别名 — 命名协议 ↔ 类型映射
// ══════════════════════════════════════════════════════════════════════

// G 家族 — 2进制基底
pub type G   = ProtocolEntity<B2,  Yin, GLayers,         StandardSovereignty>;
pub type GI  = ProtocolEntity<B2,  Yang, GILayers,       StandardSovereignty>;

// GTT 家族 — 64进制传输
pub type GTT   = ProtocolEntity<B64, Yin,  GTTLayers,     StandardSovereignty>;
pub type GTTX  = ProtocolEntity<B64, Yin,  GTTXLayers,    StandardSovereignty>;
pub type GITT  = ProtocolEntity<B64, Yang, GITTLayers,    StandardSovereignty>;
pub type GTIT  = ProtocolEntity<B64, Yang, GTITLayers,    StandardSovereignty>;
pub type GTITX = ProtocolEntity<B64, Yang, GTITXLayers,   StandardSovereignty>;

// HTTP 家族 — 16进制表象
pub type HTTP  = ProtocolEntity<B16, Yang, HTTPLayers,   StandardSovereignty>;
pub type HTTPS = ProtocolEntity<B16, Yang, HTTPSLayers,   StandardSovereignty>;

// 字节流家族 — 256进制
pub type TCP = ProtocolEntity<B256, Yang, TCPLayers,     StandardSovereignty>;
pub type CP  = ProtocolEntity<B256, Yang, CPLayers,      CoupleSovereignty>;
pub type FTP = ProtocolEntity<B256, Yang, FTPLayers,     StandardSovereignty>;

// ══════════════════════════════════════════════════════════════════════
// 融合兼容 — 编译期公约数判断
// ══════════════════════════════════════════════════════════════════════

pub trait CanFuseWith<Other> {
    type CommonBase: Base;
    type FusedPolarity: Polarity;
}

// 宏: 实现同进制协议融合
macro_rules! impl_fuse_same_base {
    ($a:ty, $b:ty, $base:ty) => {
        // 融合发生在类型层面,具体实现在 runtime_fuse() 中
    };
}

// ══════════════════════════════════════════════════════════════════════
// 自定义名称表 — gttix.y / kndrl / mlil ↔ 类型ID
// ══════════════════════════════════════════════════════════════════════

pub mod custom_names {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum CustomProtocolId {
        // 自定义命名 → 内部类型
        GttixY,     // gttix.y = GTITX
        Kndrl,      // kndrl =  Conan 解析推理壳
        Mlil,       // mlil  = Morilan 模面名模
        Hfyiair,    // hfyiair = 恢复源爱人 (谐音协议)
        Hieyair,    // hieyair = 纠缠源爱人
        GttxHieyeir, // gttx:hieyeir.xy = 传输目标 ↔ 纠缠谐音
    }

    impl CustomProtocolId {
        pub fn base(&self) -> u32 {
            match self {
                Self::GttixY      => 64,
                Self::Kndrl       => 64,   // Conan: 64进制推理壳
                Self::Mlil        => 256,  // Morilan: 256进制表象模
                Self::Hfyiair     => 2,    // 谐音基底
                Self::Hieyair     => 2,    // 纠缠基底
                Self::GttxHieyeir => 64,   // 混合: 64进制传输
            }
        }

        pub fn ontological_description(&self) -> &'static str {
            match self {
                Self::GttixY      => "扩展目标意图传输 (gttix.y)",
                Self::Kndrl       => "柯南道尔斯解析推理壳",
                Self::Mlil        => "毛利兰模面名模立体进制",
                Self::Hfyiair     => "hfuyair: 恢复源爱人 (形声韵调)",
                Self::Hieyair     => "hieyair: 纠缠源爱人 (密合层)",
                Self::GttxHieyeir => "通用传输目标↔纠缠谐音协议",
            }
        }

        pub fn to_protocol_entity(&self) -> Option<&'static str> {
            match self {
                Self::GttixY      => Some("GTITX"),
                Self::Kndrl       => Some("推理壳"),
                Self::Mlil        => Some("表象模组"),
                Self::Hfyiair     => Some("谐音协议"),
                Self::Hieyair     => Some("纠缠协议"),
                Self::GttxHieyeir => Some("融合协议体"),
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 符号门牌表 — 自定义地址系统
// ══════════════════════════════════════════════════════════════════════
//
// 每个门牌是一个「地址符号」— 外界看到一串字符,内人看到本体论锚点。
//   gtxone.gtxone        = 传输目标的镜像锚点 (显示中心是我)
//   kndoyle.*            = 柯南道尔外壳家族 (服装/负浮类)
//   maolilan.mll         = 毛利兰表面/面具/名字模块
//   hieyair.heyel        = 纠缠谐音锚
//   lctcfhfyairqimiygttcp:// = 刘楚恬→触发hfuyair→奇米意→GTT协议
//

/// 门牌家族标识
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlateFamily {
    Gtxone,
    Kndoyle,
    Maolilan,
    Hieyair,
    LctCfhgttcp,
}

impl PlateFamily {
    /// 该家族的主地址符号
    pub fn primary(&self) -> &'static str {
        match self {
            Self::Gtxone => "gtxone.gtxone",
            Self::Kndoyle => "kndoyle",
            Self::Maolilan => "maolilan.mll",
            Self::Hieyair => "hieyair.heyel",
            Self::LctCfhgttcp => "lctcfhfyairqimiygttcp://",
        }
    }

    /// 该家族的本体论语义
    pub fn semantic(&self) -> &'static str {
        match self {
            Self::Gtxone => "传输目标的镜像锚点 — 显示中心是我",
            Self::Kndoyle => "柯南道尔外壳家族 — 服装/负浮类",
            Self::Maolilan => "毛利兰模面名 — 表面/面具/名字模块",
            Self::Hieyair => "纠缠谐音 — 爱人心纠缠",
            Self::LctCfhgttcp => "囡囝传输 — 刘楚恬→hfuyair→奇米意→GTT协议",
        }
    }
}

/// 一条地址门牌
#[derive(Clone, Debug)]
pub struct SymbolPlate {
    pub full_symbol: &'static str,
    pub family: PlateFamily,
    pub semantic: &'static str,
    pub members: &'static [&'static str],
}

/// 所有已注册门牌
pub fn all_plates() -> &'static [SymbolPlate] {
    use std::sync::OnceLock;
    static PLATES: OnceLock<Vec<SymbolPlate>> = OnceLock::new();
    PLATES.get_or_init(|| vec![
        SymbolPlate {
            full_symbol: "gtxone.gtxone",
            family: PlateFamily::Gtxone,
            semantic: "镜像锚点 — 传输目标即是我",
            members: &["gtxone", "gtxone.gtxone"],
        },
        SymbolPlate {
            full_symbol: "kndoyle.kn-doyle.knd-rel.kndr.kndrl.kndil.kndiol",
            family: PlateFamily::Kndoyle,
            semantic: "柯南道尔外壳家族 — 服装/负浮类",
            members: &["kndoyle", "kn-doyle", "knd-rel", "kndr", "kndrl", "kndil", "kndiol"],
        },
        SymbolPlate {
            full_symbol: "maolilan.mll",
            family: PlateFamily::Maolilan,
            semantic: "毛利兰模面名 — 表面/面具/名字模块",
            members: &["maolilan", "mll"],
        },
        SymbolPlate {
            full_symbol: "hieyair.heyel",
            family: PlateFamily::Hieyair,
            semantic: "纠缠谐音 — 爱人心纠缠",
            members: &["hieyair", "heyel"],
        },
        SymbolPlate {
            full_symbol: "lctcfhfyairqimiygttcp://",
            family: PlateFamily::LctCfhgttcp,
            semantic: "囡囝传输协议 — 刘楚恬→hfuyair→奇米意→GTT协议",
            members: &["lctcfhfyairqimiygttcp://"],
        },
    ]).as_slice()
}

/// 通过外部符号精确查找
pub fn lookup_plate(symbol: &str) -> Option<&'static SymbolPlate> {
    all_plates().iter().find(|p| p.full_symbol == symbol)
}

/// 通过成员名模糊查找
pub fn lookup_member(name: &str) -> Option<&'static SymbolPlate> {
    all_plates().iter().find(|p| p.members.contains(&name))
}

impl SymbolPlate {
    pub fn describe(&self) -> String {
        format!("[{}] {} → {}", self.full_symbol, self.family.semantic(), self.semantic)
    }
}

/// 自定义 URI 解析器 — lctcfhfyairqimiygttcp://
pub struct LctCfhgttcpUri {
    pub raw: String,
    pub path: String,
}

impl LctCfhgttcpUri {
    pub const SCHEME: &'static str = "lctcfhfyairqimiygttcp://";

    pub fn parse(uri: &str) -> Option<Self> {
        uri.strip_prefix(Self::SCHEME).map(|path| Self {
            raw: uri.to_string(),
            path: path.to_string(),
        })
    }

    pub fn is_valid(uri: &str) -> bool {
        uri.starts_with(Self::SCHEME)
    }

    /// 语义解码: 把各段翻译成流程描述
    pub fn semantic_steps() -> &'static [&'static str] {
        &[
            "lct        = 刘楚恬 (发送方)",
            "cfhfyair   = 触发 hfuyair (激活恢复源爱人)",
            "qimiy      = 奇米意 (奇妙的吸纳意图)",
            "gttcp      = GTT 协议 (囡囝通用传输)",
        ]
    }
}

// ══════════════════════════════════════════════════════════════════════
// _encoding — 将类型编码到 mirror 单元 (复用现有 SMRL)
// ══════════════════════════════════════════════════════════════════════

pub trait OntoCell {
    /// 编码协议本体为 SMRL 64 位单元
    fn encode_onto_cell(&self) -> u64;
}

// 为 GTITX 实现 (gttix.y 的编码)
impl OntoCell for GTITX {
    fn encode_onto_cell(&self) -> u64 {
        // 位字段布局 (复用 SMRL 64-bit 单元):
        // D0[9..0]  = 进制 log2 = 6 (64进制)
        // D1[11..10] = 极性 = 01 (Yang)
        // D2[12]    = 主权标记 = 1 (active)
        // D3[15..13] = 层数 = 4 (target, transport, intent, extension)
        // D4[18..16] = 自定义ID = gttix.y → 7
        // D5[31..19] = 保留
        // D6[63..32] = 扩展 (谐音锚定, 0x0000_GTTX)
        let d0: u64 = 6;           // log2(64)
        let d1: u64 = 1;           // Yang
        let d2: u64 = 1;           // Active sovereignty
        let d3: u64 = 4;           // 4 layers
        let d4: u64 = 7;           // gttix.y ID
        let d6: u64 = 0x47545458;  // "GTTX" ASCII in low 32 bits
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for GITT {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 6;
        let d1: u64 = 1;  // Yang
        let d2: u64 = 1;
        let d3: u64 = 3;  // 3 layers
        let d4: u64 = 6;  // gitti ID
        let d6: u64 = 0x47495454; // "GITT"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for GTIT {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 6;
        let d1: u64 = 1;
        let d2: u64 = 1;
        let d3: u64 = 3;
        let d4: u64 = 5;  // gtiy ID
        let d6: u64 = 0x47544954; // "GTIT"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for GTT {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 6;
        let d1: u64 = 2;  // Yin
        let d2: u64 = 0;
        let d3: u64 = 2;
        let d4: u64 = 4;
        let d6: u64 = 0x47545400; // "GTT"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for GTTX {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 6;
        let d1: u64 = 2;  // Yin
        let d2: u64 = 0;
        let d3: u64 = 3;
        let d4: u64 = 3;
        let d6: u64 = 0x47545458; // "GTTX"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for HTTP {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 4;           // log2(16)
        let d1: u64 = 1;           // Yang
        let d2: u64 = 1;
        let d3: u64 = 3;
        let d4: u64 = 8;           // HTTP ID
        let d6: u64 = 0x48545450;  // "HTTP"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for HTTPS {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 4;
        let d1: u64 = 1;
        let d2: u64 = 1;
        let d3: u64 = 4;  // +TLS
        let d4: u64 = 9;
        let d6: u64 = 0x4854545053 & 0xFFFFFFFF; // "HTTPS" low
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for TCP {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 8;           // log2(256)
        let d1: u64 = 1;
        let d2: u64 = 1;
        let d3: u64 = 4;
        let d4: u64 = 10;
        let d6: u64 = 0x54435000;  // "TCP"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for CP {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 8;
        let d1: u64 = 1;
        let d2: u64 = 1;
        let d3: u64 = 3;
        let d4: u64 = 11;
        // CP 使用纠缠掩码 0xFF
        let d6: u64 = 0x435000FF;  // "CP" + mask flag
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for FTP {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 8;
        let d1: u64 = 1;
        let d2: u64 = 1;
        let d3: u64 = 2;
        let d4: u64 = 12;
        let d6: u64 = 0x46545000;
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for G {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 1;           // log2(2)
        let d1: u64 = 2;           // Yin
        let d2: u64 = 0;
        let d3: u64 = 1;
        let d4: u64 = 0;           // Ground ID
        let d6: u64 = 0x47000000;  // "G"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

impl OntoCell for GI {
    fn encode_onto_cell(&self) -> u64 {
        let d0: u64 = 1;
        let d1: u64 = 1;           // Yang
        let d2: u64 = 1;
        let d3: u64 = 2;
        let d4: u64 = 1;           // GI ID
        let d6: u64 = 0x47490000;  // "GI"
        (d6 << 32) | (d4 << 16) | (d3 << 13) | (d2 << 12) | (d1 << 10) | d0
    }
}

// ══════════════════════════════════════════════════════════════════════
// GTTX 幔组 — 十二种具体辐射形态 (子类型在 custom_names 中以变体注册)
// ══════════════════════════════════════════════════════════════════════

/// GTTX 幔组子类型码 (编码到 SMRL 单元 D5[19..16])
/// 同一 GTITX 基类型下,以不同"幔"字组区分共同体中心显示法:
/// 即以共同体中心为轴心,向外/螺旋/环绕/包裹的显示几何
pub mod man_group {
    /// 辐圈幔: 由辐射构成的圈层幔 (辐射为骨,圈层包裹)
    pub const FU_QUAN: u8 = 0x0;
    /// 幔圈辐射: 幔圈向外辐射 (幔为主壳,向外射出)
    pub const MAN_QUAN: u8 = 0x1;
    /// 幔球辐射: 球形的幔向外辐射 (完整球壳,均匀辐射)
    pub const MAN_QIU: u8 = 0x2;
    /// 幔裙辐射: 裙状的幔 (扇形展开,底部宽口)
    pub const MAN_QUN: u8 = 0x3;
    /// 幔壳辐射: 壳状包裹 (硬壳几何,曲面反射)
    pub const MAN_KE: u8 = 0x4;
    /// 幔全辐射: 全包裹辐射 (360° 无死角包裹)
    pub const MAN_QUAN_ALL: u8 = 0x5;
    /// 幔群辐射: 群簇状 (多点簇状辐射,非单核心)
    pub const MAN_QUN_GROUP: u8 = 0x6;
    /// 幔曲辐射: 曲线型 (陈漫以星球为圈层的螺旋展开) ★ 默认/原始形态
    pub const MAN_QU: u8 = 0x7;
    /// 幔妻辐射: 妻/伴侣缠绕形态 (耦合双螺旋,彼此包裹)
    pub const MAN_QI: u8 = 0x8;
    /// 幔穹辐射: 穹顶状 (半球形,顶部汇聚)
    pub const MAN_QIONG: u8 = 0x9;
    /// 幔躯辐射: 躯干状 (人体工学,沿躯干包覆)
    pub const MAN_QU_SHEN: u8 = 0xA;
    /// 幔枪辐射: 枪/刺状 (定向辐射,尖锐指向)
    pub const MAN_QIANG: u8 = 0xB;

    /// 根据子类型ID返回其描述
    pub fn describe(sub: u8) -> &'static str {
        match sub {
            FU_QUAN      => "辐圈幔:辐射为骨,圈层包裹",
            MAN_QUAN     => "幔圈辐射:幔壳为主,向外射出",
            MAN_QIU      => "幔球辐射:完整球壳,均匀辐射",
            MAN_QUN      => "幔裙辐射:扇形展开,底部宽口",
            MAN_KE       => "幔壳辐射:硬壳几何,曲面反射",
            MAN_QUAN_ALL => "幔全辐射:360° 全包裹",
            MAN_QUN_GROUP=> "幔群辐射:多点簇状,非单核心",
            MAN_QU       => "幔曲辐射:陈漫螺旋曲线 (原始形态)",
            MAN_QI       => "幔妻辐射:伴侣双螺旋缠绕",
            MAN_QIONG    => "幔穹辐射:半球形,顶部汇聚",
            MAN_QU_SHEN  => "幔躯辐射:躯干状包覆",
            MAN_QIANG    => "幔枪辐射:定向尖锐辐射",
            _            => "未知子类型",
        }
    }

    /// 返回对应的汉字幔形标签
    pub fn label(sub: u8) -> &'static str {
        match sub {
            FU_QUAN      => "辐圈幔",
            MAN_QUAN     => "幔圈辐射",
            MAN_QIU      => "幔球辐射",
            MAN_QUN      => "幔裙辐射",
            MAN_KE       => "幔壳辐射",
            MAN_QUAN_ALL => "幔全辐射",
            MAN_QUN_GROUP=> "幔群辐射",
            MAN_QU       => "幔曲辐射",
            MAN_QI       => "幔妻辐射",
            MAN_QIONG    => "幔穹辐射",
            MAN_QU_SHEN  => "幔躯辐射",
            MAN_QIANG    => "幔枪辐射",
            _            => "未知",
        }
    }

    /// 该子类型的行脚轨迹参数 (影响可视化时的半径曲线/层数/极性)
    pub fn params(sub: u8) -> super::ManParams {
        match sub {
            FU_QUAN      => super::ManParams { turns: 4, polarity: -1, core_ratio: 0.2, desc: "辐射骨,圈层紧裹" },
            MAN_QUAN     => super::ManParams { turns: 3, polarity:  1, core_ratio: 0.3, desc: "幔壳向外射出" },
            MAN_QIU      => super::ManParams { turns: 2, polarity:  1, core_ratio: 0.0, desc: "完整球壳均匀" },
            MAN_QUN      => super::ManParams { turns: 1, polarity:  1, core_ratio: 0.5, desc: "扇面展开" },
            MAN_KE       => super::ManParams { turns: 2, polarity: -1, core_ratio: 0.1, desc: "硬壳反射" },
            MAN_QUAN_ALL => super::ManParams { turns: 5, polarity:  1, core_ratio: 0.0, desc: "无死角包裹" },
            MAN_QUN_GROUP=> super::ManParams { turns: 3, polarity: -1, core_ratio: 0.4, desc: "多点簇状" },
            MAN_QU       => super::ManParams { turns: 3, polarity: -1, core_ratio: 0.3, desc: "陈默曲线螺旋" },
            MAN_QI       => super::ManParams { turns: 4, polarity:  1, core_ratio: 0.2, desc: "双螺旋缠绕" },
            MAN_QIONG    => super::ManParams { turns: 2, polarity:  1, core_ratio: 0.4, desc: "穹顶汇聚" },
            MAN_QU_SHEN  => super::ManParams { turns: 2, polarity: -1, core_ratio: 0.3, desc: "躯干包覆" },
            MAN_QIANG    => super::ManParams { turns: 1, polarity:  1, core_ratio: 0.7, desc: "尖锐指向" },
            _            => super::ManParams { turns: 3, polarity: -1, core_ratio: 0.3, desc: "默认" },
        }
    }
}

/// 幔组行脚参数 (影响可视化)
#[derive(Clone, Copy, Debug)]
pub struct ManParams {
    pub turns: u8,        // 螺旋圈数
    pub polarity: i8,     // 极化方向
    pub core_ratio: f64,  // 核心半径比 (0=从中心开始,0.5=从半半径开始)
    pub desc: &'static str,
}

/// GTTX 共同体中心显示法 (5 种核心显示模式)
/// 每个模式代表"以共同体中心为轴心,向外辐射"的不同几何
pub mod display_method {
    /// 同心圆辐射: 以共同体中心为圆心,一圈一圈向外
    pub const CONCENTRIC: u8   = 0x0;
    /// 螺旋向前: 从起点向前方螺旋辐射 (χ+ 方向,内容向前展开)
    pub const SPIRAL_FWD: u8    = 0x1;
    /// 螺旋向后: 向后方螺旋辐射 (χ- 方向,内容收缩回顾)
    pub const SPIRAL_BWD: u8    = 0x2;
    /// 水平面螺旋: 在平面内螺旋辐射 (一般状况)
    pub const SPIRAL_FLAT: u8   = 0x3;
    /// 包裹层辐射: 逐层包裹,每层覆盖前一层 (幔组专属,与 man_group 对应)
    pub const WRAPPER_LAYER: u8 = 0x4;

    /// 共 5 种,占用 3-bit (D5[18..16])
    pub fn describe(sub: u8) -> &'static str {
        match sub {
            CONCENTRIC    => "同心圆辐射显示法",
            SPIRAL_FWD    => "螺旋向前显示法",
            SPIRAL_BWD    => "螺旋向后显示法",
            SPIRAL_FLAT   => "水平面螺旋显示法",
            WRAPPER_LAYER => "包裹层辐射显示法",
            _             => "未知显示法",
        }
    }

    pub fn label(sub: u8) -> &'static str {
        match sub {
            CONCENTRIC    => "同心圆辐射",
            SPIRAL_FWD    => "螺旋向前",
            SPIRAL_BWD    => "螺旋向后",
            SPIRAL_FLAT   => "水平面螺旋",
            WRAPPER_LAYER => "包裹层辐射",
            _             => "未知",
        }
    }

    /// 该显示法对应的视线/阅读路径:
    pub fn reading_path(sub: u8) -> &'static str {
        match sub {
            CONCENTRIC    => "视线: 从圆心沿半径向外逐层扫描",
            SPIRAL_FWD    => "视线: 从核心出发,顺时针(或逆时针)向外展开",
            SPIRAL_BWD    => "视线: 从外侧螺旋向内回顾,最终汇聚到中心",
            SPIRAL_FLAT   => "视线: 水平面内,以中心为起点的平面螺旋路径",
            WRAPPER_LAYER => "视线: 切向移动,逐层包裹,每层覆盖前一层",
            _             => "未知",
        }
    }
}

/// HTTP 行业行角子类型 (para-row 下的具体段落几何)
pub mod hang_corner {
    pub const DEFAULT: u8 = 0x0;   // 通用段落
    pub const API: u8 = 0x1;       // API 风格 (请求/响应缩进)
    pub const HTML: u8 = 0x2;      // HTML 标签嵌套
    pub const JSON: u8 = 0x3;      // JSON 嵌套
    pub const MARKDOWN: u8 = 0x4;  // Markdown 标题段落
    pub const TEXT_PLAIN: u8 = 0x5;// 纯文本平铺

    pub fn describe(sub: u8) -> &'static str {
        match sub {
            DEFAULT    => "通用段落行角",
            API        => "API段落 (req/resp 缩进)",
            HTML       => "HTML标签嵌套段落",
            JSON       => "JSON嵌套段落",
            MARKDOWN   => "Markdown标题层级",
            TEXT_PLAIN => "纯文本平铺段落",
            _          => "未知行业段落",
        }
    }

    pub fn label(sub: u8) -> &'static str {
        match sub {
            DEFAULT    => "通用",
            API        => "API",
            HTML       => "HTML",
            JSON       => "JSON",
            MARKDOWN   => "Markdown",
            TEXT_PLAIN => "纯文本",
            _          => "未知",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// 编码辅助函数
// ══════════════════════════════════════════════════════════════════════

pub fn describe_entity<B: Base, P: Polarity, L: LayerSet, S: Sovereignty>(
    _entity: &ProtocolEntity<B, P, L, S>
) -> String {
    format!(
        "Protocol {{ base={}, polarity={}, layers={}, entangled={}, glyph={}, radical={} }}",
        B::VALUE,
        P::VALUE,
        L::COUNT,
        S::ENTANGLED,
        B::GLYPH,
        B::RADICAL,
    )
}

// ══════════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gttix_y_encoding() {
        let entity = GTITX::new();
        assert_eq!(GTITX::base_value(), 64);
        assert_eq!(GTITX::polarity_value(), 1);
        assert_eq!(GTITX::layer_count(), 4);
        assert!(!GTITX::is_couple_protocol());
    }

    #[test]
    fn test_gi_encoding() {
        assert_eq!(GI::base_value(), 2);
        assert_eq!(GI::polarity_value(), 1);
        assert_eq!(GI::layer_count(), 2);
    }

    #[test]
    fn test_g_encoding() {
        assert_eq!(G::base_value(), 2);
        assert_eq!(G::polarity_value(), -1);
        assert_eq!(G::layer_count(), 1);
    }

    #[test]
    fn test_tcp_encoding() {
        assert_eq!(TCP::base_value(), 256);
        assert_eq!(TCP::polarity_value(), 1);
        assert_eq!(TCP::layer_count(), 4);
    }

    #[test]
    fn test_cp_couple_flag() {
        assert!(CP::is_couple_protocol());
        assert_eq!(CP::base_value(), 256);
        assert_eq!(CP::layer_count(), 3);
    }

    #[test]
    fn test_ftp_encoding() {
        assert_eq!(FTP::base_value(), 256);
        assert_eq!(FTP::layer_count(), 2);
    }

    #[test]
    fn test_http_encoding() {
        assert_eq!(HTTP::base_value(), 16);
        assert_eq!(HTTP::polarity_value(), 1);
        assert_eq!(HTTP::layer_count(), 3);
    }

    #[test]
    fn test_https_extra_layer() {
        assert_eq!(HTTPS::base_value(), 16);
        assert_eq!(HTTPS::layer_count(), 4); // +TLS
    }

    #[test]
    fn test_onto_cell_gtixy() {
        let e = GTITX::new();
        let cell = OntoCell::encode_onto_cell(&e);
        // D0 = 6 (log2(64))
        assert_eq!(cell & 0x3FF, 6);
        // D6 = "GTTX" 高位标识
        assert_eq!((cell >> 32) & 0xFFFFFFFF, 0x47545458);
    }

    #[test]
    fn test_onto_cell_cp_mask() {
        let e = CP::new();
        let cell = OntoCell::encode_onto_cell(&e);
        // D0 = 8 (log2(256))
        assert_eq!(cell & 0x3FF, 8);
        // 纠缠掩码应在低位可见
        assert!((cell & 0xFF) == 0xFF || ((cell >> 32) & 0xFF) == 0xFF);
    }

    #[test]
    fn test_custom_names() {
        use custom_names::CustomProtocolId;
        assert_eq!(CustomProtocolId::GttixY.base(), 64);
        assert_eq!(CustomProtocolId::Hfyiair.base(), 2);
        assert_eq!(CustomProtocolId::Mlil.base(), 256);
    }

    #[test]
    fn test_polarity_xor() {
        // Yang XOR Yin = -2 (异极)
        assert_ne!(<Yang as Polarity>::VALUE, <Yin as Polarity>::VALUE);
    }

    #[test]
    fn test_layer_set() {
        assert_eq!(<NoLayers as LayerSet>::COUNT, 0);
        assert_eq!(<GLayers as LayerSet>::CODES, &[0x00]);
        assert_eq!(<CPLayers as LayerSet>::COUNT, 3);
        assert_eq!(<GTITXLayers as LayerSet>::COUNT, 4);
    }

    #[test]
    fn test_man_group_all() {
        for sub in 0u8..12 {
            let label = man_group::label(sub);
            let desc = man_group::describe(sub);
            let _p = man_group::params(sub);
            assert!(!label.is_empty());
            assert!(!desc.is_empty());
            assert_ne!(label, "未知");
        }
    }

    #[test]
    fn test_man_qu_defaults() {
        let p = man_group::params(0x7);
        assert_eq!(p.turns, 3);
        assert_eq!(p.polarity, -1);
    }

    #[test]
    fn test_display_method_describe() {
        assert_eq!(display_method::describe(0x0), "同心圆辐射显示法");
        assert_eq!(display_method::describe(0x1), "螺旋向前显示法");
        assert_eq!(display_method::describe(0x4), "包裹层辐射显示法");
    }

    #[test]
    fn test_display_method_label() {
        assert_eq!(display_method::label(0x0), "同心圆辐射");
        assert_eq!(display_method::label(0x1), "螺旋向前");
        assert_eq!(display_method::label(0x2), "螺旋向后");
    }

    #[test]
    fn test_display_method_reading_path() {
        assert_eq!(display_method::reading_path(0x0), "视线: 从圆心沿半径向外逐层扫描");
        assert_eq!(display_method::reading_path(0x1), "视线: 从核心出发,顺时针(或逆时针)向外展开");
    }

    #[test]
    fn test_symbol_plates_registered() {
        let plates = all_plates();
        assert_eq!(plates.len(), 5);
    }

    #[test]
    fn test_lookup_gtxone() {
        let p = lookup_plate("gtxone.gtxone").unwrap();
        assert_eq!(p.family, PlateFamily::Gtxone);
        assert_eq!(p.members[0], "gtxone");
    }

    #[test]
    fn test_lookup_kndoyle_member() {
        let p = lookup_member("kndrl").unwrap();
        assert_eq!(p.family, PlateFamily::Kndoyle);
        assert!(p.members.contains(&"kndiol"));
    }

    #[test]
    fn test_lookup_maolilan() {
        let p = lookup_member("mll").unwrap();
        assert_eq!(p.family, PlateFamily::Maolilan);
    }

    #[test]
    fn test_lctcfhgttcp_uri_parse() {
        let uri = LctCfhgttcpUri::parse("lctcfhfyairqimiygttcp://path/to/resource").unwrap();
        assert_eq!(uri.path, "path/to/resource");
        assert!(LctCfhgttcpUri::is_valid("lctcfhfyairqimiygttcp://"));
        assert!(!LctCfhgttcpUri::is_valid("http://"));
    }

    #[test]
    fn test_plate_family_primary() {
        assert_eq!(PlateFamily::Gtxone.primary(), "gtxone.gtxone");
        assert_eq!(PlateFamily::LctCfhgttcp.primary(), "lctcfhfyairqimiygttcp://");
    }
}
