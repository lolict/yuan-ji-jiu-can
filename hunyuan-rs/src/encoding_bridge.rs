// ══════════════════════════════════════════════════════════════════
// encoding_bridge.rs — C 编码系统 → Rust 核心桥接
// ══════════════════════════════════════════════════════════════════
//
// 把 encoding/ 目录的 C 编码系统 (1257字 · 五大家族 · 三元裁判)
// 通过 FFI 引入 Rust 核心。供 Rust 各模块调用:
//   - yinyang_codec  → 用极性判向
//   - observer       → 用家族路由
//   - smrl           → 用容器编码做三元观察
//   - mind_organ     → 用流属性做感知聚类

use std::ffi::CString;

// ── 家族枚举 ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Family {
    Container = 0,
    Boundary  = 1,
    Couple    = 2,
    Web       = 3,
    Neutral   = 4,
}

impl Family {
    pub fn from_container_code(code: u32) -> Self {
        match (code >> 29) & 0x7 {
            0 => Family::Container,
            1 => Family::Boundary,
            2 => Family::Couple,
            3 => Family::Web,
            _ => Family::Neutral,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Family::Container => "容器(刘楚恬侧)",
            Family::Boundary  => "边界(lolict侧)",
            Family::Couple    => "夫妻(囡囝合)",
            Family::Web       => "网状",
            Family::Neutral   => "中性",
        }
    }
}

impl std::fmt::Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ── 裁判结果枚举 ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Verdict {
    Pending   = 0,
    Admitted  = 1,
    Rejected  = 2,
    Suspended = 3,
}

// ── 12 位流属性 ──
bitflags::bitflags! {
    pub struct FlowBits: u32 {
        const REACHABLE    = 0x001;
        const CONTINUABLE  = 0x002;
        const INCR         = 0x004;
        const STOPPABLE    = 0x008;
        const REVERSIBLE   = 0x010;
        const RECOVERABLE  = 0x020;
        const CIRCULABLE   = 0x040;
        const INHERITABLE  = 0x080;
        const TRUSTED      = 0x100;
        const ACKABLE      = 0x200;
        const TRANSMISSIBLE = 0x400;
        const ADVANCEABLE  = 0x800;
    }
}

// ── FFI: 调用 _emit 版本的编码桥接函数 ──
extern "C" {
    fn encoding_init_emit();
    fn encoding_utf8_to_container_emit(utf8: *const i8) -> u32;
    fn encoding_classify_family_emit(ucs2: u16) -> u8;
    fn encoding_compute_flow_bits_emit(ucs2: u16, family: u8) -> u32;
    fn encoding_get_polarity_emit(ucs2: u16) -> u8;
    fn encoding_has_hanzi_emit(ucs2: u16) -> i32;
    fn encoding_hanzi_count_emit() -> i32;
}

// ── 初始化 (一次性) ──
fn init_once() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe { encoding_init_emit() });
}

/// UTF-8 字符串 → 32 位容器编码
pub fn utf8_to_container(utf8: &str) -> u32 {
    init_once();
    let c_str = std::ffi::CString::new(utf8).unwrap();
    unsafe { encoding_utf8_to_container_emit(c_str.as_ptr()) }
}

/// UCS-2 → 家族分类
pub fn classify_family(ucs2: u16) -> Family {
    init_once();
    match unsafe { encoding_classify_family_emit(ucs2) } {
        0 => Family::Container,
        1 => Family::Boundary,
        2 => Family::Couple,
        3 => Family::Web,
        _ => Family::Neutral,
    }
}

/// 流属性计算
pub fn compute_flow_bits(ucs2: u16, family: Family) -> FlowBits {
    init_once();
    unsafe { FlowBits::from_bits_truncate(encoding_compute_flow_bits_emit(ucs2, family as u8)) }
}

/// 极性判定 (YinYang): 0=YIN(刘楚恬侧), 1=YANG(lolict侧)
pub fn get_polarity(ucs2: u16) -> u8 {
    init_once();
    unsafe { encoding_get_polarity_emit(ucs2) }
}

/// 汉字是否存在
pub fn has_hanzi(ucs2: u16) -> bool {
    init_once();
    unsafe { encoding_has_hanzi_emit(ucs2) != 0 }
}

/// 表内汉字数
pub fn hanzi_count() -> usize {
    init_once();
    unsafe { encoding_hanzi_count_emit() as usize }
}

/// 综合分析: UTF-8 → (容器码, 家族, 极性, 流属性)
pub fn analyze(utf8: &str) -> Option<(u32, Family, u8, FlowBits)> {
    let code = utf8_to_container(utf8);
    if code == 0 { return None; }
    let fam = Family::from_container_code(code);
    let pol = ((code >> 28) & 1) as u8;
    let fb  = compute_flow_bits(0, fam);
    Some((code, fam, pol, fb))
}

/// 从容器编码提取极性
pub fn polarity_from_code(code: u32) -> u8 {
    ((code >> 28) & 1) as u8
}
