// ═══════════════════════════════════════════════════════════════════════════════
// rhyme.rs — 韵母进制 · 镜像遍历 · 超韵母选址
// ═══════════════════════════════════════════════════════════════════════════════
//
// 设计：
//   1. 将汉语普通话全部韵母及其镜像变体，统一合并为一棵遍历树
//   2. 每个韵母 = 一个地址（进制位），不是编码 — 它的"值"就是它的发音位置
//   3. 超韵母（SuperVowel）= 所有韵母遍历后的合一
//   4. 遍历时使用加法/乘法叠加：加法=韵尾连接，乘法=韵头共鸣放大
//   5. 不依赖系统编码（不调 unicode/utf-8），内存中直接用数组索引
//
// 三种遍历模式：
//   正韵  a → ai → an → ang → ao        （开口到闭合）
//   反韵  ang → an → ai → a             （镜像闭合回开口）
//   圆遍历  ei ↔ ie ↔ oi ↔ io ↔ ai ↔ ia  （镜像对折，用户式遍历）
//
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// 基础音素（不可再分的最小发音单位）
// ─────────────────────────────────────────────────────────────────────────────

/// 10 个基础音素 = 音空间的"原子"
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phoneme {
    A,  // 开口 a
    O,  // 半合 o
    E,  // 半开 e
    I,  // 齐齿 i
    U,  // 合口 u
    V,  // 撮口 ü
    N,  // 鼻音前后 ng/n
}

impl Phoneme {
    /// 音素 → 口腔位置（数值 = 前后高低位置，0=最前/最低，7=最厚/最高）
    pub const fn position(self) -> u8 {
        match self {
            Phoneme::A => 0,
            Phoneme::O => 3,
            Phoneme::E => 2,
            Phoneme::I => 1,
            Phoneme::U => 5,
            Phoneme::V => 4,
            Phoneme::N => 7,
        }
    }

    /// 从口腔位置反推音素
    pub fn from_position(p: u8) -> Option<Self> {
        match p {
            0 => Some(Phoneme::A),
            1 => Some(Phoneme::I),
            2 => Some(Phoneme::E),
            3 => Some(Phoneme::O),
            4 => Some(Phoneme::V),
            5 => Some(Phoneme::U),
            7 => Some(Phoneme::N),
            _ => None,
        }
    }

    /// 两个音素叠加（加法：位置相加取模）
    pub fn add(self, other: Self) -> u8 {
        (self.position() + other.position()) % 8
    }

    /// 两个音素共振（乘法：位置相乘取模，增强差异）
    pub fn mul(self, other: Self) -> u8 {
        ((self.position() as u16 * other.position() as u16) % 8) as u8
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 韵母分类
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RhymeCategory {
    Simple,      // 单韵母：a o e i u ü  (6)
    Compound,    // 复韵母：ai ei ao ou ia ie io iu ui uo ue  (约20)
    Nasal,       // 鼻韵母：an en in un ün ang eng ing ong  (约15)
    Retroflex,   // 儿化：er
    Mirror,      // 镜像三合：iao uai uei uen iou iang uang ... (约10)
    Final26,     // 用户26个超韵母合成变体
}

// ─────────────────────────────────────────────────────────────────────────────
// 韵母本身（不是编码，是发音坐标）
// ─────────────────────────────────────────────────────────────────────────────

/// 一个韵母 = 发音位置序列（固定大小数组），这个序列本身就是地址
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rhyme {
    pub id: u16,                        // 内部ID（仅用于数组索引，不参与数值运算）
    pub spelling: &'static str,         // 拼写形式（如 "ai", "iao", "uang"）
    pub phonemes: &'static [Phoneme],   // 音素序列 = 数值序列（静态）
    pub category: RhymeCategory,        // 分类标签
    pub linear_val: u16,                // 缓存的加法自值
    pub resonance_val: u16,             // 缓存的乘法共振值
}

impl Rhyme {
    /// 创建新韵母（运行时）
    pub fn new(id: u16, spelling: &'static str, phonemes: &'static [Phoneme], category: RhymeCategory) -> Self {
        let linear_val = Self::calc_linear(phonemes);
        let resonance_val = Self::calc_resonance(phonemes);
        Self { id, spelling, phonemes, category, linear_val, resonance_val }
    }

    /// 线性加法值
    fn calc_linear(phonemes: &[Phoneme]) -> u16 {
        phonemes.iter().map(|p| p.position() as u16).fold(0u16, |acc, x| acc.wrapping_add(x))
    }

    /// 乘法共振值
    fn calc_resonance(phonemes: &[Phoneme]) -> u16 {
        if phonemes.is_empty() { return 0; }
        phonemes.iter().map(|p| (p.position() as u16).max(1)).fold(1u16, |acc, x| acc.wrapping_mul(x))
    }

    /// 韵母的"加法自值"（live计算，用作备用）
    pub fn linear_value(&self) -> u16 {
        self.linear_val
    }

    /// 韵母的"乘法共振值"
    pub fn resonance_value(&self) -> u16 {
        self.resonance_val
    }

    /// 综合值：加法 × 乘法叠加（既有线性结构又有共振放大）
    pub fn combined_value(&self) -> u32 {
        (self.linear_val as u32).wrapping_mul(self.resonance_val.max(1) as u32)
    }

    /// 与另一个韵母的"距离" = 绝对值差（度量）
    pub fn distance_to(&self, other: &Self) -> u16 {
        let a = self.combined_value();
        let b = other.combined_value();
        if a > b { (a - b) as u16 } else { (b - a) as u16 }
    }

    /// 是否镜像韵母（结构反转后相等）
    pub fn is_mirror_of(&self, other: &Self) -> bool {
        if self.phonemes.len() != other.phonemes.len() { return false; }
        self.phonemes.iter().eq(other.phonemes.iter().rev())
    }

    /// 韵母音素数量
    pub fn len(&self) -> usize { self.phonemes.len() }
    pub fn is_empty(&self) -> bool { self.phonemes.is_empty() }
}

// ─────────────────────────────────────────────────────────────────────────────
// 静态韵母表
// ─────────────────────────────────────────────────────────────────────────────

// 单韵母序列常量（编译期确定，不占运行时栈）
static PH_A:  [Phoneme; 1] = [Phoneme::A];
static PH_O:  [Phoneme; 1] = [Phoneme::O];
static PH_E:  [Phoneme; 1] = [Phoneme::E];
static PH_I:  [Phoneme; 1] = [Phoneme::I];
static PH_U:  [Phoneme; 1] = [Phoneme::U];
static PH_V:  [Phoneme; 1] = [Phoneme::V];

// 二合复韵母
static PH_AI:  [Phoneme; 2] = [Phoneme::A, Phoneme::I];
static PH_EI:  [Phoneme; 2] = [Phoneme::E, Phoneme::I];
static PH_AO:  [Phoneme; 2] = [Phoneme::A, Phoneme::O];
static PH_OU:  [Phoneme; 2] = [Phoneme::O, Phoneme::U];
static PH_IA:  [Phoneme; 2] = [Phoneme::I, Phoneme::A];
static PH_IE:  [Phoneme; 2] = [Phoneme::I, Phoneme::E];
static PH_IO:  [Phoneme; 2] = [Phoneme::I, Phoneme::O];
static PH_IU:  [Phoneme; 2] = [Phoneme::I, Phoneme::U];
static PH_UA:  [Phoneme; 2] = [Phoneme::U, Phoneme::A];
static PH_UO:  [Phoneme; 2] = [Phoneme::U, Phoneme::O];
static PH_UE:  [Phoneme; 2] = [Phoneme::U, Phoneme::E];
static PH_UI:  [Phoneme; 2] = [Phoneme::U, Phoneme::I];
static PH_VE:  [Phoneme; 2] = [Phoneme::V, Phoneme::E];

// 三合复韵母
static PH_IAO: [Phoneme; 3] = [Phoneme::I, Phoneme::A, Phoneme::O];
static PH_IOU: [Phoneme; 3] = [Phoneme::I, Phoneme::O, Phoneme::U];
static PH_UAI: [Phoneme; 3] = [Phoneme::U, Phoneme::A, Phoneme::I];
static PH_UEI: [Phoneme; 3] = [Phoneme::U, Phoneme::E, Phoneme::I];

// 前鼻韵母
static PH_AN:   [Phoneme; 2] = [Phoneme::A, Phoneme::N];
static PH_EN:   [Phoneme; 2] = [Phoneme::E, Phoneme::N];
static PH_IN:   [Phoneme; 2] = [Phoneme::I, Phoneme::N];
static PH_UN:   [Phoneme; 2] = [Phoneme::U, Phoneme::N];
static PH_VN:   [Phoneme; 2] = [Phoneme::V, Phoneme::N];
static PH_IAN:  [Phoneme; 3] = [Phoneme::I, Phoneme::A, Phoneme::N];
static PH_UAN:  [Phoneme; 3] = [Phoneme::U, Phoneme::A, Phoneme::N];
static PH_VAN:  [Phoneme; 3] = [Phoneme::V, Phoneme::A, Phoneme::N];

// 后鼻韵母
static PH_ANG:  [Phoneme; 3] = [Phoneme::A, Phoneme::N, Phoneme::N];
static PH_ENG:  [Phoneme; 3] = [Phoneme::E, Phoneme::N, Phoneme::N];
static PH_ING:  [Phoneme; 3] = [Phoneme::I, Phoneme::N, Phoneme::N];
static PH_ONG:  [Phoneme; 3] = [Phoneme::O, Phoneme::N, Phoneme::N];
static PH_IANG: [Phoneme; 4] = [Phoneme::I, Phoneme::A, Phoneme::N, Phoneme::N];
static PH_UANG: [Phoneme; 4] = [Phoneme::U, Phoneme::A, Phoneme::N, Phoneme::N];
static PH_IONG: [Phoneme; 4] = [Phoneme::I, Phoneme::O, Phoneme::N, Phoneme::N];

// 儿化韵母
static PH_ER:   [Phoneme; 2] = [Phoneme::E, Phoneme::N];

// ═══ 用户自创镜像遍历变体 ═══
static PH_EI_IE:  [Phoneme; 4] = [Phoneme::E, Phoneme::I, Phoneme::I, Phoneme::E];
static PH_AI_IA:  [Phoneme; 4] = [Phoneme::A, Phoneme::I, Phoneme::I, Phoneme::A];
static PH_OI_IO:  [Phoneme; 4] = [Phoneme::O, Phoneme::I, Phoneme::I, Phoneme::O];
static PH_UV_VU:  [Phoneme; 4] = [Phoneme::U, Phoneme::V, Phoneme::V, Phoneme::U];
static PH_AA:     [Phoneme; 2] = [Phoneme::A, Phoneme::A];
static PH_EE:     [Phoneme; 2] = [Phoneme::E, Phoneme::E];
static PH_II:     [Phoneme; 2] = [Phoneme::I, Phoneme::I];
static PH_OO:     [Phoneme; 2] = [Phoneme::O, Phoneme::O];
static PH_UU:     [Phoneme; 2] = [Phoneme::U, Phoneme::U];
static PH_VV:     [Phoneme; 2] = [Phoneme::V, Phoneme::V];
static PH_NN:     [Phoneme; 2] = [Phoneme::N, Phoneme::N];
static PH_IAU:    [Phoneme; 3] = [Phoneme::I, Phoneme::A, Phoneme::U];
static PH_UAE:    [Phoneme; 3] = [Phoneme::U, Phoneme::A, Phoneme::E];
static PH_EAE:    [Phoneme; 3] = [Phoneme::E, Phoneme::A, Phoneme::E];
static PH_IAI:    [Phoneme; 3] = [Phoneme::I, Phoneme::A, Phoneme::I];
static PH_UAU:    [Phoneme; 3] = [Phoneme::U, Phoneme::A, Phoneme::U];
static PH_IAOI:   [Phoneme; 4] = [Phoneme::I, Phoneme::A, Phoneme::O, Phoneme::I];
static PH_UEIU:   [Phoneme; 4] = [Phoneme::U, Phoneme::E, Phoneme::I, Phoneme::U];
// 超韵母候选
static PH_SUPER:  [Phoneme; 7] = [Phoneme::A, Phoneme::O, Phoneme::E, Phoneme::I, Phoneme::V, Phoneme::U, Phoneme::N];
static PH_SUPER2: [Phoneme; 8] = [Phoneme::N, Phoneme::V, Phoneme::U, Phoneme::I, Phoneme::E, Phoneme::A, Phoneme::O, Phoneme::A];

/// 韵母全景表 — 全局唯一
pub static RHYME_TABLE: [Rhyme; 59] = {
    // 显式构造每一条（coercion through binding）
    let ph_a:  &[Phoneme] = &PH_A;
    let ph_o:  &[Phoneme] = &PH_O;
    let ph_e:  &[Phoneme] = &PH_E;
    let ph_i:  &[Phoneme] = &PH_I;
    let ph_u:  &[Phoneme] = &PH_U;
    let ph_v:  &[Phoneme] = &PH_V;
    let ph_ai:  &[Phoneme] = &PH_AI;
    let ph_ei:  &[Phoneme] = &PH_EI;
    let ph_ao:  &[Phoneme] = &PH_AO;
    let ph_ou:  &[Phoneme] = &PH_OU;
    let ph_ia:  &[Phoneme] = &PH_IA;
    let ph_ie:  &[Phoneme] = &PH_IE;
    let ph_io:  &[Phoneme] = &PH_IO;
    let ph_iu:  &[Phoneme] = &PH_IU;
    let ph_ua:  &[Phoneme] = &PH_UA;
    let ph_uo:  &[Phoneme] = &PH_UO;
    let ph_ue:  &[Phoneme] = &PH_UE;
    let ph_ui:  &[Phoneme] = &PH_UI;
    let ph_ve:  &[Phoneme] = &PH_VE;
    let ph_iao: &[Phoneme] = &PH_IAO;
    let ph_iou: &[Phoneme] = &PH_IOU;
    let ph_uai: &[Phoneme] = &PH_UAI;
    let ph_uei: &[Phoneme] = &PH_UEI;
    let ph_an:   &[Phoneme] = &PH_AN;
    let ph_en:   &[Phoneme] = &PH_EN;
    let ph_in:   &[Phoneme] = &PH_IN;
    let ph_un:   &[Phoneme] = &PH_UN;
    let ph_vn:   &[Phoneme] = &PH_VN;
    let ph_ian:  &[Phoneme] = &PH_IAN;
    let ph_uan:  &[Phoneme] = &PH_UAN;
    let ph_van:  &[Phoneme] = &PH_VAN;
    let ph_ang:  &[Phoneme] = &PH_ANG;
    let ph_eng:  &[Phoneme] = &PH_ENG;
    let ph_ing:  &[Phoneme] = &PH_ING;
    let ph_ong:  &[Phoneme] = &PH_ONG;
    let ph_iang: &[Phoneme] = &PH_IANG;
    let ph_uang: &[Phoneme] = &PH_UANG;
    let ph_iong: &[Phoneme] = &PH_IONG;
    let ph_er:   &[Phoneme] = &PH_ER;
    let ph_ei_ie:  &[Phoneme] = &PH_EI_IE;
    let ph_ai_ia:  &[Phoneme] = &PH_AI_IA;
    let ph_oi_io:  &[Phoneme] = &PH_OI_IO;
    let ph_uv_vu:  &[Phoneme] = &PH_UV_VU;
    let ph_aa:     &[Phoneme] = &PH_AA;
    let ph_ee:     &[Phoneme] = &PH_EE;
    let ph_ii:     &[Phoneme] = &PH_II;
    let ph_oo:     &[Phoneme] = &PH_OO;
    let ph_uu:     &[Phoneme] = &PH_UU;
    let ph_vv:     &[Phoneme] = &PH_VV;
    let ph_nn:     &[Phoneme] = &PH_NN;
    let ph_iau:    &[Phoneme] = &PH_IAU;
    let ph_uae:    &[Phoneme] = &PH_UAE;
    let ph_eae:    &[Phoneme] = &PH_EAE;
    let ph_iai:    &[Phoneme] = &PH_IAI;
    let ph_uau:    &[Phoneme] = &PH_UAU;
    let ph_iaoi:   &[Phoneme] = &PH_IAOI;
    let ph_ueiu:   &[Phoneme] = &PH_UEIU;
    let ph_super:  &[Phoneme] = &PH_SUPER;
    let ph_super2: &[Phoneme] = &PH_SUPER2;

    [
        // 单韵母 (6)
        Rhyme { id: 0,  spelling: "a",     phonemes: ph_a,   category: RhymeCategory::Simple,    linear_val: 1,  resonance_val: 1 },
    Rhyme { id: 1,  spelling: "o",     phonemes: ph_o,   category: RhymeCategory::Simple,    linear_val: 3,  resonance_val: 3 },
    Rhyme { id: 2,  spelling: "e",     phonemes: ph_e,   category: RhymeCategory::Simple,    linear_val: 2,  resonance_val: 2 },
    Rhyme { id: 3,  spelling: "i",     phonemes: ph_i,   category: RhymeCategory::Simple,    linear_val: 1,  resonance_val: 1 },
    Rhyme { id: 4,  spelling: "u",     phonemes: ph_u,   category: RhymeCategory::Simple,    linear_val: 5,  resonance_val: 5 },
    Rhyme { id: 5,  spelling: "v",     phonemes: ph_v,   category: RhymeCategory::Simple,    linear_val: 4,  resonance_val: 4 },
    // 复韵母 (13)
    Rhyme { id: 6,  spelling: "ai",    phonemes: ph_ai,  category: RhymeCategory::Compound,  linear_val: 1,  resonance_val: 0 },
    Rhyme { id: 7,  spelling: "ei",    phonemes: ph_ei,  category: RhymeCategory::Compound,  linear_val: 3,  resonance_val: 2 },
    Rhyme { id: 8,  spelling: "ao",    phonemes: ph_ao,  category: RhymeCategory::Compound,  linear_val: 3,  resonance_val: 0 },
    Rhyme { id: 9,  spelling: "ou",    phonemes: ph_ou,  category: RhymeCategory::Compound,  linear_val: 8,  resonance_val: 15 },
    Rhyme { id: 10, spelling: "ia",    phonemes: ph_ia,  category: RhymeCategory::Compound,  linear_val: 1,  resonance_val: 0 },
    Rhyme { id: 11, spelling: "ie",    phonemes: ph_ie,  category: RhymeCategory::Compound,  linear_val: 3,  resonance_val: 2 },
    Rhyme { id: 12, spelling: "io",    phonemes: ph_io,  category: RhymeCategory::Compound,  linear_val: 4,  resonance_val: 3 },
    Rhyme { id: 13, spelling: "iu",    phonemes: ph_iu,  category: RhymeCategory::Compound,  linear_val: 6,  resonance_val: 5 },
    Rhyme { id: 14, spelling: "ua",    phonemes: ph_ua,  category: RhymeCategory::Compound,  linear_val: 5,  resonance_val: 0 },
    Rhyme { id: 15, spelling: "uo",    phonemes: ph_uo,  category: RhymeCategory::Compound,  linear_val: 8,  resonance_val: 15 },
    Rhyme { id: 16, spelling: "ue",    phonemes: ph_ue,  category: RhymeCategory::Compound,  linear_val: 7,  resonance_val: 10 },
    Rhyme { id: 17, spelling: "ui",    phonemes: ph_ui,  category: RhymeCategory::Compound,  linear_val: 6,  resonance_val: 5 },
    Rhyme { id: 18, spelling: "ve",    phonemes: ph_ve,  category: RhymeCategory::Compound,  linear_val: 6,  resonance_val: 8 },
    // 三合 (4)
    Rhyme { id: 19, spelling: "iao",   phonemes: ph_iao, category: RhymeCategory::Compound,  linear_val: 4,  resonance_val: 0 },
    Rhyme { id: 20, spelling: "iou",   phonemes: ph_iou, category: RhymeCategory::Compound,  linear_val: 9,  resonance_val: 15 },
    Rhyme { id: 21, spelling: "uai",   phonemes: ph_uai, category: RhymeCategory::Compound,  linear_val: 6,  resonance_val: 0 },
    Rhyme { id: 22, spelling: "uei",   phonemes: ph_uei, category: RhymeCategory::Compound,  linear_val: 8,  resonance_val: 10 },
    // 前鼻 (8)
    Rhyme { id: 23, spelling: "an",    phonemes: ph_an,   category: RhymeCategory::Nasal,     linear_val: 7,  resonance_val: 0 },
    Rhyme { id: 24, spelling: "en",    phonemes: ph_en,   category: RhymeCategory::Nasal,     linear_val: 9,  resonance_val: 14 },
    Rhyme { id: 25, spelling: "in",    phonemes: ph_in,   category: RhymeCategory::Nasal,     linear_val: 8,  resonance_val: 7 },
    Rhyme { id: 26, spelling: "un",    phonemes: ph_un,   category: RhymeCategory::Nasal,     linear_val: 12, resonance_val: 35 },
    Rhyme { id: 27, spelling: "vn",    phonemes: ph_vn,   category: RhymeCategory::Nasal,     linear_val: 11, resonance_val: 28 },
    Rhyme { id: 28, spelling: "ian",   phonemes: ph_ian,  category: RhymeCategory::Nasal,     linear_val: 8,  resonance_val: 0 },
    Rhyme { id: 29, spelling: "uan",   phonemes: ph_uan,  category: RhymeCategory::Nasal,     linear_val: 12, resonance_val: 0 },
    Rhyme { id: 30, spelling: "van",   phonemes: ph_van,  category: RhymeCategory::Nasal,     linear_val: 11, resonance_val: 0 },
    // 后鼻 (8)
    Rhyme { id: 31, spelling: "ang",   phonemes: ph_ang,  category: RhymeCategory::Nasal,     linear_val: 14, resonance_val: 0 },
    Rhyme { id: 32, spelling: "eng",   phonemes: ph_eng,  category: RhymeCategory::Nasal,     linear_val: 16, resonance_val: 98 },
    Rhyme { id: 33, spelling: "ing",   phonemes: ph_ing,  category: RhymeCategory::Nasal,     linear_val: 15, resonance_val: 49 },
    Rhyme { id: 34, spelling: "ong",   phonemes: ph_ong,  category: RhymeCategory::Nasal,     linear_val: 17, resonance_val: 147 },
    Rhyme { id: 35, spelling: "iang",  phonemes: ph_iang, category: RhymeCategory::Nasal,     linear_val: 15, resonance_val: 0 },
    Rhyme { id: 36, spelling: "uang",  phonemes: ph_uang, category: RhymeCategory::Nasal,     linear_val: 19, resonance_val: 0 },
    Rhyme { id: 37, spelling: "iong",  phonemes: ph_iong, category: RhymeCategory::Nasal,     linear_val: 18, resonance_val: 147 },
    // 儿化 (1)
    Rhyme { id: 38, spelling: "er",    phonemes: ph_er,   category: RhymeCategory::Retroflex, linear_val: 9,  resonance_val: 14 },
    // ═══ 用户自创镜像变体 (20) ═══
    Rhyme { id: 39, spelling: "ei_ie", phonemes: ph_ei_ie, category: RhymeCategory::Mirror, linear_val: 6,  resonance_val: 4 },
    Rhyme { id: 40, spelling: "ai_ia", phonemes: ph_ai_ia, category: RhymeCategory::Mirror, linear_val: 2,  resonance_val: 0 },
    Rhyme { id: 41, spelling: "oi_io", phonemes: ph_oi_io, category: RhymeCategory::Mirror, linear_val: 8,  resonance_val: 9 },
    Rhyme { id: 42, spelling: "uv_vu", phonemes: ph_uv_vu, category: RhymeCategory::Mirror, linear_val: 18, resonance_val: 400 },
    Rhyme { id: 43, spelling: "aa",    phonemes: ph_aa,   category: RhymeCategory::Final26,   linear_val: 0,  resonance_val: 0 },
    Rhyme { id: 44, spelling: "ee",    phonemes: ph_ee,   category: RhymeCategory::Final26,   linear_val: 4,  resonance_val: 4 },
    Rhyme { id: 45, spelling: "ii",    phonemes: ph_ii,   category: RhymeCategory::Final26,   linear_val: 2,  resonance_val: 1 },
    Rhyme { id: 46, spelling: "oo",    phonemes: ph_oo,   category: RhymeCategory::Final26,   linear_val: 6,  resonance_val: 9 },
    Rhyme { id: 47, spelling: "uu",    phonemes: ph_uu,   category: RhymeCategory::Final26,   linear_val: 10, resonance_val: 25 },
    Rhyme { id: 48, spelling: "vv",    phonemes: ph_vv,   category: RhymeCategory::Final26,   linear_val: 8,  resonance_val: 16 },
    Rhyme { id: 49, spelling: "nn",    phonemes: ph_nn,   category: RhymeCategory::Final26,   linear_val: 14, resonance_val: 49 },
    Rhyme { id: 50, spelling: "iau",   phonemes: ph_iau,  category: RhymeCategory::Final26,   linear_val: 6,  resonance_val: 0 },
    Rhyme { id: 51, spelling: "uae",   phonemes: ph_uae,  category: RhymeCategory::Final26,   linear_val: 7,  resonance_val: 0 },
    Rhyme { id: 52, spelling: "eae",   phonemes: ph_eae,  category: RhymeCategory::Final26,   linear_val: 4,  resonance_val: 0 },
    Rhyme { id: 53, spelling: "iai",   phonemes: ph_iai,  category: RhymeCategory::Final26,   linear_val: 2,  resonance_val: 0 },
    Rhyme { id: 54, spelling: "uau",   phonemes: ph_uau,  category: RhymeCategory::Final26,   linear_val: 5,  resonance_val: 0 },
    Rhyme { id: 55, spelling: "iaoi",  phonemes: ph_iaoi, category: RhymeCategory::Final26,   linear_val: 5,  resonance_val: 0 },
    Rhyme { id: 56, spelling: "ueiu",  phonemes: ph_ueiu, category: RhymeCategory::Final26,   linear_val: 11, resonance_val: 10 },
    // 超韵母 (2)
    Rhyme { id: 57, spelling: "sup7",  phonemes: ph_super,  category: RhymeCategory::Final26, linear_val: 22, resonance_val: 0 },
    Rhyme { id: 58, spelling: "sup8a", phonemes: ph_super2, category: RhymeCategory::Final26, linear_val: 31, resonance_val: 0 },
    ]
};

// ─────────────────────────────────────────────────────────────────────────────
// 超韵母（所有韵母的合一）— 操作入口
// ─────────────────────────────────────────────────────────────────────────────

pub struct SuperVowel;

impl SuperVowel {
    /// 加法合一值
    pub fn addition_value() -> u32 {
        let mut sum: u32 = 0;
        for r in RHYME_TABLE.iter() {
            sum = sum.wrapping_add(r.linear_value() as u32);
        }
        sum
    }

    /// 乘法共振值
    pub fn resonance_value() -> u32 {
        let mut prod: u32 = 1;
        for r in RHYME_TABLE.iter() {
            let v = r.resonance_value().max(1) as u32;
            prod = prod.wrapping_mul(v);
        }
        prod
    }

    /// 正序遍历
    pub fn traverse<'a>() -> &'a [Rhyme] {
        &RHYME_TABLE
    }

    /// 反序遍历
    pub fn traverse_mirror<'a>() -> Vec<&'a Rhyme> {
        let mut v: Vec<_> = RHYME_TABLE.iter().collect();
        v.reverse();
        v
    }

    /// 圆遍历（镜像跳跃）
    pub fn traverse_circular<'a>() -> Vec<&'a Rhyme> {
        let mut result = Vec::new();
        let len = RHYME_TABLE.len();
        if len == 0 { return result; }

        let mut visited = vec![false; len];
        let mut idx = 0;

        for _ in 0..len {
            if visited[idx] { break; }
            visited[idx] = true;
            result.push(&RHYME_TABLE[idx]);
            idx = Self::find_mirror_match(idx, &visited);
        }

        for i in 0..len {
            if !visited[i] {
                result.push(&RHYME_TABLE[i]);
            }
        }

        result
    }

    fn find_mirror_match(current: usize, visited: &[bool]) -> usize {
        let current_r = &RHYME_TABLE[current];
        let rev: Vec<Phoneme> = current_r.phonemes.iter().cloned().rev().collect();

        let mut best = None;
        let mut best_dist = u16::MAX;

        for (i, r) in RHYME_TABLE.iter().enumerate() {
            if visited[i] { continue; }
            if r.phonemes.len() != rev.len() { continue; }

            let dist: u16 = r.phonemes.iter().zip(rev.iter())
                .map(|(a, b)| {
                    let pa = a.position();
                    let pb = b.position();
                    pa.abs_diff(pb) as u16
                })
                .fold(0u16, |acc, x| acc.wrapping_add(x));

            if dist < best_dist {
                best_dist = dist;
                best = Some(i);
            }
        }

        best.unwrap_or_else(|| {
            visited.iter().position(|v| !v).unwrap_or(0)
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 韵母进制工具函数
// ─────────────────────────────────────────────────────────────────────────────

pub struct RhymeUtils;

impl RhymeUtils {
    /// 查找韵母（按拼写）
    pub fn find(spelling: &str) -> Option<&'static Rhyme> {
        RHYME_TABLE.iter().find(|r| r.spelling == spelling)
    }

    /// 查找韵母（按 ID）
    pub fn find_by_id(id: u16) -> Option<&'static Rhyme> {
        RHYME_TABLE.iter().find(|r| r.id == id)
    }

    /// 韵母总数
    pub fn count() -> usize {
        RHYME_TABLE.len()
    }

    /// 按加法值排序
    pub fn sorted_by_linear() -> Vec<&'static Rhyme> {
        let mut v: Vec<_> = RHYME_TABLE.iter().collect();
        v.sort_by_key(|r| r.linear_value());
        v
    }

    /// 距离矩阵：计算两类韵母之间的远近
    pub fn distance_matrix() -> Vec<Vec<u16>> {
        let n = RHYME_TABLE.len();
        let mut m = vec![vec![0u16; n]; n];
        for i in 0..n {
            for j in 0..n {
                m[i][j] = RHYME_TABLE[i].distance_to(&RHYME_TABLE[j]);
            }
        }
        m
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoneme_positions() {
        assert_eq!(Phoneme::A.position(), 0);
        assert_eq!(Phoneme::I.position(), 1);
        assert_eq!(Phoneme::U.position(), 5);
        assert_eq!(Phoneme::N.position(), 7);
    }

    #[test]
    fn test_phoneme_add_mul() {
        assert_eq!(Phoneme::A.add(Phoneme::I), 1);
        assert_eq!(Phoneme::A.mul(Phoneme::I), 0);
        assert_eq!(Phoneme::I.mul(Phoneme::U), 5);
    }

    #[test]
    fn test_rhyme_table_complete() {
        // 韵母表有59个条目
        assert_eq!(RHYME_TABLE.len(), 59);
    }

    #[test]
    fn test_all_linear_values_nonzero() {
        for r in RHYME_TABLE.iter() {
            // AA(0,0) = 0 是允许的（零韵母）
            // 其他都应该 > 0
            if r.spelling != "aa" && r.spelling != "sup7" {
                assert!(r.linear_value() > 0 || r.resonance_value() > 0,
                    "韵母 {} 所有值为 0", r.spelling);
            }
        }
    }

    #[test]
    fn test_super_vowel_addition_nonzero() {
        assert!(SuperVowel::addition_value() > 0);
    }

    #[test]
    fn test_traverse_circular_full() {
        let v = SuperVowel::traverse_circular();
        assert_eq!(v.len(), RHYME_TABLE.len());
    }

    #[test]
    fn test_find_by_spelling() {
        assert!(RhymeUtils::find("a").is_some());
        assert!(RhymeUtils::find("ai").is_some());
        assert!(RhymeUtils::find("ei_ie").is_some());  // 用户变体
        assert!(RhymeUtils::find("xyz").is_none());
    }

    #[test]
    fn test_distance_symmetry() {
        let a = RhymeUtils::find("a").unwrap();
        let i = RhymeUtils::find("i").unwrap();
        assert_eq!(a.distance_to(i), i.distance_to(a));
    }

    #[test]
    fn test_zero_distance_self() {
        let a = RhymeUtils::find("a").unwrap();
        assert_eq!(a.distance_to(a), 0);
    }

    #[test]
    fn test_mirror_detection() {
        let ai = RhymeUtils::find("ai").unwrap();
        let ia = RhymeUtils::find("ia").unwrap();
        assert!(ai.is_mirror_of(ia));
    }

    #[test]
    fn test_traverse_mirror_order() {
        let fwd: Vec<_> = SuperVowel::traverse().iter().map(|r| r.id).collect();
        let mir: Vec<_> = SuperVowel::traverse_mirror().iter().map(|r| r.id).collect();
        assert_eq!(fwd.len(), mir.len());
        assert_eq!(fwd[0], mir[mir.len() - 1]);
    }
}
