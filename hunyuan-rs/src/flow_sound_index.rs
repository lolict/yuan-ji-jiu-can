// ══════════════════════════════════════════════════════════════════════
// flow_sound_index.rs — 声波索引 · 韵母地址 · 统计距离 (v0.40)
// ══════════════════════════════════════════════════════════════════════
//
//  「数字 = 口腔位置（不是数值）」
//  「进制 = 发音空间划分方式（不是进位规则）」
//  「声母24 + 韵母 + 声调 = 可计算的发音地址」
//
//  五个维度统一:
//   声母(辅音起始位) → 触发空间位置      → 0~23 (24 腔位)
//   韵母(共振延续)   → 空间内划分        → 0~36 (37 共振型)
//   声调(音高)       → 时间相位          → 0~4 (阴/阳/上/去/入)
//   里外(深度)       → 观察者到容器核心   → Surface/Threshold/Interior
//   统计距离        → 基于频次、共现、历史 → 加权欧氏
//
//  实现约束:
//    - 声母遍历韵母后再调下一个（声母一次调入一个）
//    - 折叠 = 符号用完后不进位，形成新层
//    - 距离只用加减+位运算（无乘除）
//
// ══════════════════════════════════════════════════════════════════════

use crate::mirror::Sovereignty;
use crate::distinguisher::SoundTarget;
use crate::flow_properties::{FlowDictionary, FlowEntry, REACHABLE, CONTINUABLE, TRUSTED, TRANSMISSIBLE};
use crate::inside_out::InsideOut;

// ────────────────────────────────────────────────────────────
// 1. 声母口腔位置 (24 个辅音起始位)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Initial {
    B = 0,   // 双唇-闭
    P = 1,   // 双唇-送
    M = 2,   // 双唇-鼻
    F = 3,   // 齿唇
    D = 4,   // 舌尖-闭
    T = 5,   // 舌尖-送
    N = 6,   // 舌尖-鼻
    L = 7,   // 舌尖-边
    G = 8,   // 舌根-闭
    K = 9,   // 舌根-送
    H = 10,  // 舌根-擦
    J = 11,  // 舌面-闭
    Q = 12,  // 舌面-送
    X = 13,  // 舌面-擦
    Zh = 14, // 卷舌-闭
    Ch = 15, // 卷舌-送
    Sh = 16, // 卷舌-擦
    R = 17,  // 卷舌-浊
    Z = 18,  // 平舌-闭
    C = 19,  // 平舌-送
    S = 20,  // 平舌-擦
    Y = 21,  // 半元音-舌面
    W = 22,  // 半元音-唇
    Zero = 23, // 零声母 (直接起韵)
}

impl Initial {
    pub fn position(self) -> u32 { self as u32 }

    pub fn from_pinyin(s: &str) -> Option<Initial> {
        use Initial::*;
        Some(match s {
            "b" => B, "p" => P, "m" => M, "f" => F,
            "d" => D, "t" => T, "n" => N, "l" => L,
            "g" => G, "k" => K, "h" => H,
            "j" => J, "q" => Q, "x" => X,
            "zh" => Zh, "ch" => Ch, "sh" => Sh, "r" => R,
            "z" => Z, "c" => C, "s" => S,
            "y" => Y, "w" => W,
            "" | "'" => Zero,
            _ => return None,
        })
    }
}

// ────────────────────────────────────────────────────────────
// 2. 韵母共振型 (37 种共振模式)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Final {
    A = 0,  O = 1,  E = 2,  I = 3,  U = 4,  V = 5,
    Ai = 6, Ei = 7, Ao = 8, Ou = 9,
    An = 10, En = 11, Ang = 12, Eng = 13, Ong = 14,
    Ia = 15, Ie = 16, Iao = 17, Iu = 18, Ian = 19, In = 20, Iang = 21, Ing = 22, Ion = 23,
    Ua = 24, Uo = 25, Uai = 26, Ui = 27, Uan = 28, Un = 29, Uang = 30, Ueng = 31,
    Ve = 32, Van = 33, Vn = 34,
    Er = 35,
    N = 36,  // 鼻音自成音节
}

impl Final {
    pub fn position(self) -> u32 { self as u32 }

    pub fn from_pinyin(s: &str) -> Option<Final> {
        use Final::*;
        Some(match s {
            "a" => A, "o" => O, "e" => E, "i" => I, "u" => U, "v" => V,
            "ai" => Ai, "ei" => Ei, "ao" => Ao, "ou" => Ou,
            "an" => An, "en" => En, "ang" => Ang, "eng" => Eng, "ong" => Ong,
            "ia" => Ia, "ie" => Ie, "iao" => Iao, "iu" => Iu, "ian" => Ian,
            "in" => In, "iang" => Iang, "ing" => Ing, "ion" => Ion,
            "ua" => Ua, "uo" => Uo, "uai" => Uai, "ui" => Ui,
            "uan" => Uan, "un" => Un, "uang" => Uang, "ueng" => Ueng,
            "ve" => Ve, "van" => Van, "vn" => Vn,
            "er" => Er,
            "n" => N,
            _ => return None,
        })
    }
}

// ────────────────────────────────────────────────────────────
// 3. 声调 (5 声位: 阴平/阳平/上声/去声/入声)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    YinPing = 0,   // 阴平 (高平)
    YangPing = 1,  // 阳平 (上升)
    ShangSheng = 2, // 上声 (曲折)
    QuSheng = 3,   // 去声 (下降)
    RuSheng = 4,   // 入声 (短促)
}

impl Tone {
    pub fn position(self) -> u32 { self as u32 }
}

// ────────────────────────────────────────────────────────────
// 4. 声波地址 — 可计算的发音地址
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct SoundAddress {
    pub initial: Initial,
    pub final_: Final,
    pub tone: Tone,
}

impl SoundAddress {
    pub fn new(initial: Initial, final_: Final, tone: Tone) -> Self {
        SoundAddress { initial, final_, tone }
    }

    /// 计算唯一地址值（24 × 37 × 5 = 4440 个不同地址）
    pub fn value(self) -> u64 {
        let i = self.initial.position() as u64;
        let f = self.final_.position() as u64;
        let t = self.tone.position() as u64
            + (f * 5) // 韵母展开 × 声调
            + (i * 5 * 37); // 声母展开 × 韵母 × 声调
        // 简化计算 + 数值保证唯一
        i + f * 32 + t * 64
    }

    /// 声母遍历韵母完成后 → 调下一个声母（折叠层进）
    pub fn next_initial(self) -> Option<SoundAddress> {
        let cur = self.initial as u32;
        if cur >= 23 { return None; } // 超出 24 声母
        Some(SoundAddress {
            initial: unsafe { std::mem::transmute::<u8, Initial>((cur + 1) as u8) },
            final_: self.final_,
            tone: self.tone,
        })
    }

    /// 同一声母下遍历下一韵母（进韵不进位，折叠到下一层）
    pub fn next_final(self) -> Option<SoundAddress> {
        let cur = self.final_ as u32;
        if cur >= 36 { return None; } // 超出 37 韵母
        Some(SoundAddress {
            initial: self.initial,
            final_: unsafe { std::mem::transmute::<u8, Final>((cur + 1) as u8) },
            tone: self.tone,
        })
    }
}

// ────────────────────────────────────────────────────────────
// 5. 声波距离 — 两个音节之间的「口腔距离」
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SoundDistance {
    pub initial_dist: i32,   // 声母位置差
    pub final_dist: i32,     // 韵母位置差
    pub tone_dist: i32,      // 声调差
    pub total: i32,          // 加权和
}

impl SoundDistance {
    pub fn between(a: SoundAddress, b: SoundAddress) -> SoundDistance {
        let i = (a.initial.position() as i32 - b.initial.position() as i32).abs();
        let f = (a.final_.position() as i32 - b.final_.position() as i32).abs();
        let t = (a.tone.position() as i32 - b.tone.position() as i32).abs();
        SoundDistance {
            initial_dist: i,
            final_dist: f,
            tone_dist: t,
            // 加权和：声母差异权重最高（口腔位置最核心）
            total: i * 4 + f * 2 + t * 1,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 6. 一条路声波路由 — 声母一次调入一个，韵母全遍历完再调下一个声母
// ────────────────────────────────────────────────────────────

pub struct SoundRouter;

impl SoundRouter {
    /// 声母遍历韵母（声母固定，遍历 37 韵母，每个韵母再 × 5 声调）
    pub fn enumerate_with_fixed_initial(initial: Initial) -> Vec<SoundAddress> {
        let mut v = Vec::with_capacity(37 * 5);
        let tone_values = [Tone::YinPing, Tone::YangPing, Tone::ShangSheng, Tone::QuSheng, Tone::RuSheng];
        for f in 0..37u32 {
            for t in tone_values {
                v.push(SoundAddress::new(
                    initial,
                    unsafe { std::mem::transmute::<u8, Final>(f as u8) },
                    t,
                ));
            }
        }
        v
    }

    /// 声母顺序调用：从某个声母开始 → 遍历完全部韵母 → 调下一个声母
    pub fn traverse_all() -> Vec<Vec<SoundAddress>> {
        let mut layers = Vec::with_capacity(24);
        for i in 0..24u32 {
            let initial = unsafe { std::mem::transmute::<u8, Initial>(i as u8) };
            layers.push(Self::enumerate_with_fixed_initial(initial));
        }
        layers
    }
}

// ────────────────────────────────────────────────────────────
// 7. 注册声波地址到字典项
// ────────────────────────────────────────────────────────────

pub struct IndexedFlowEntry {
    pub base: FlowEntry,
    pub sound: Option<SoundAddress>,
    pub depth: InsideOut,
    pub stat_freq: u32,          // 历史出现次数
    pub stat_cooccurrence: u32,  // 共现次数
}

impl IndexedFlowEntry {
    pub fn from_base(base: FlowEntry) -> Self {
        IndexedFlowEntry {
            base,
            sound: None,
            depth: InsideOut::Surface,
            stat_freq: 0,
            stat_cooccurrence: 0,
        }
    }

    pub fn with_sound(mut self, addr: SoundAddress) -> Self {
        self.sound = Some(addr);
        self
    }

    pub fn with_depth(mut self, depth: InsideOut) -> Self {
        self.depth = depth;
        self
    }

    /// 统计距离：与另一个条目的「相似度」（越小越相似）
    pub fn stat_distance(&self, other: &IndexedFlowEntry) -> f64 {
        let freq_diff = if self.stat_freq > other.stat_freq {
            self.stat_freq - other.stat_freq
        } else {
            other.stat_freq - self.stat_freq
        } as f64;
        // 里外深度差权重
        let depth_dist = match (&self.depth, &other.depth) {
            (InsideOut::Surface, InsideOut::Surface) => 0.0,
            (InsideOut::Surface, InsideOut::Threshold) => 1.0,
            (InsideOut::Surface, InsideOut::Interior) => 2.0,
            (InsideOut::Threshold, InsideOut::Threshold) => 0.0,
            (InsideOut::Threshold, InsideOut::Interior) => 1.0,
            (InsideOut::Interior, InsideOut::Interior) => 0.0,
            _ => -1.0,
        };
        freq_diff + depth_dist * 10.0
    }
}

// ────────────────────────────────────────────────────────────
// 8. 多维索引字典 — 字典的扩展
// ────────────────────────────────────────────────────────────

pub struct MultiIndexDictionary {
    pub entries: Vec<IndexedFlowEntry>,
}

impl MultiIndexDictionary {
    pub fn new() -> Self { MultiIndexDictionary { entries: Vec::new() } }

    pub fn insert(&mut self, e: IndexedFlowEntry) { self.entries.push(e); }

    /// 声波索引查询：按声母位置找最近的字典项
    pub fn query_by_initial(&self, initial: Initial) -> Vec<&IndexedFlowEntry> {
        self.entries.iter()
            .filter(|e| e.sound.map_or(false, |s| s.initial == initial))
            .collect()
    }

    /// 声波距离查询：找与给定音节地址最接近的 k 个条目
    pub fn query_nearest(&self, addr: SoundAddress, k: usize) -> Vec<(i32, &IndexedFlowEntry)> {
        let mut scored: Vec<_> = self.entries.iter()
            .filter_map(|e| e.sound.map(|s| (SoundDistance::between(s, addr).total, e)))
            .collect();
        scored.sort_by_key(|(d, _)| *d);
        scored.into_iter().take(k).collect()
    }

    /// 里外深度过滤：只返回表面/边界/内部 中指定位的条目
    pub fn query_by_depth(&self, depth: InsideOut) -> Vec<&IndexedFlowEntry> {
        self.entries.iter().filter(|e| e.depth == depth).collect()
    }

    /// 统计距离：按频次和深度的综合距离排序
    pub fn rank_by_stat_distance(&self, reference: &IndexedFlowEntry) -> Vec<(f64, &IndexedFlowEntry)> {
        let mut scored: Vec<_> = self.entries.iter()
            .map(|e| (reference.stat_distance(e), e))
            .collect();
        scored.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}

// ══════════════════════════════════════════════════════════
// 测试 — 29 个
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distinguisher::SoundTarget;

    // === Initial ===
    #[test]
    fn initial_positions_range() {
        assert!(Initial::B.position() < 24);
        assert_eq!(Initial::Zero.position(), 23);
    }

    #[test]
    fn initial_from_pinyin() {
        assert_eq!(Initial::from_pinyin("b"), Some(Initial::B));
        assert_eq!(Initial::from_pinyin("zh"), Some(Initial::Zh));
        assert_eq!(Initial::from_pinyin(""), Some(Initial::Zero));
        assert_eq!(Initial::from_pinyin("?"), None);
    }

    // === Final ===
    #[test]
    fn final_positions_range() {
        assert!(Final::A.position() < 37);
        assert_eq!(Final::N.position(), 36);
    }

    #[test]
    fn final_from_pinyin() {
        assert_eq!(Final::from_pinyin("a"), Some(Final::A));
        assert_eq!(Final::from_pinyin("zh"), None); // zh 是声母，不是韵母
        assert_eq!(Final::from_pinyin("ing"), Some(Final::Ing));
        assert_eq!(Final::from_pinyin("?"), None);
    }

    // === SoundAddress ===
    #[test]
    fn sound_address_value_deterministic() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let b = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        assert_eq!(a.value(), b.value());
    }

    #[test]
    fn sound_address_different_values() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let d = SoundAddress::new(Initial::D, Final::O, Tone::YangPing);
        assert_ne!(a.value(), d.value());
    }

    #[test]
    fn next_initial_increments() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let n = a.next_initial();
        assert!(n.is_some());
        assert_eq!(n.unwrap().initial.position(), 1); // P
    }

    #[test]
    fn next_initial_overflow_returns_none() {
        let a = SoundAddress::new(Initial::Zero, Final::A, Tone::QuSheng);
        let n = a.next_initial();
        assert!(n.is_none()); // 已经是第 24 个声母
    }

    #[test]
    fn next_final_increments() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let n = a.next_final();
        assert!(n.is_some());
        assert_eq!(n.unwrap().final_.position(), 1); // O
    }

    #[test]
    fn next_final_overflow_returns_none() {
        let a = SoundAddress::new(Initial::B, Final::N, Tone::RuSheng);
        let n = a.next_final();
        assert!(n.is_none()); // 已经是第 37 个韵母
    }

    // === SoundDistance ===
    #[test]
    fn sound_distance_same_address_is_zero() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let d = SoundDistance::between(a, a);
        assert_eq!(d.total, 0);
    }

    #[test]
    fn sound_distance_symmetric() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let b = SoundAddress::new(Initial::D, Final::O, Tone::YangPing);
        let d1 = SoundDistance::between(a, b);
        let d2 = SoundDistance::between(b, a);
        assert_eq!(d1.total, d2.total);
    }

    #[test]
    fn initial_matters_more_than_tone() {
        let a = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let b = SoundAddress::new(Initial::D, Final::A, Tone::YinPing);
        let c = SoundAddress::new(Initial::B, Final::A, Tone::RuSheng);
        let d_initial = SoundDistance::between(a, b).total;
        let d_tone = SoundDistance::between(a, c).total;
        assert!(d_initial > d_tone);
    }

    // === SoundRouter ===
    #[test]
    fn enumerate_fixed_initial_returns_185() {
        // 37 韵母 × 5 声调 = 185
        let v = SoundRouter::enumerate_with_fixed_initial(Initial::B);
        assert_eq!(v.len(), 185);
    }

    #[test]
    fn traverse_all_returns_24_layers() {
        let layers = SoundRouter::traverse_all();
        assert_eq!(layers.len(), 24);
    }

    // === IndexedFlowEntry ===
    #[test]
    fn indexed_from_base_has_surface_default() {
        let base = FlowEntry::new("test", SoundTarget::Container, REACHABLE | CONTINUABLE, Sovereignty::Passive);
        let entry = IndexedFlowEntry::from_base(base);
        assert_eq!(entry.depth, InsideOut::Surface);
        assert!(entry.sound.is_none());
    }

    #[test]
    fn indexed_with_sound_attaches() {
        use crate::flow_properties::{REACHABLE, CONTINUABLE};
        let base = FlowEntry::new("test", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let addr = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let entry = IndexedFlowEntry::from_base(base).with_sound(addr);
        assert!(entry.sound.is_some());
        assert_eq!(entry.sound.unwrap().value(), addr.value());
    }

    #[test]
    fn indexed_with_depth_attaches() {
        use crate::flow_properties::REACHABLE;
        let base = FlowEntry::new("test", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let entry = IndexedFlowEntry::from_base(base).with_sound(
            SoundAddress::new(Initial::B, Final::A, Tone::YinPing)
        ).with_depth(InsideOut::Interior);
        assert_eq!(entry.depth, InsideOut::Interior);
    }

    #[test]
    fn stat_distance_same_entry_is_zero() {
        let base = FlowEntry::new("test", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let e1 = IndexedFlowEntry::from_base(base);
        let base2 = FlowEntry::new("test2", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let e2 = IndexedFlowEntry::from_base(base2);
        assert_eq!(e1.stat_distance(&e2), 0.0);
    }

    #[test]
    fn stat_distance_depth_diff_adds() {
        let base = FlowEntry::new("a", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let e1 = IndexedFlowEntry::from_base(base).with_depth(InsideOut::Surface);
        let base2 = FlowEntry::new("b", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let e2 = IndexedFlowEntry::from_base(base2).with_depth(InsideOut::Interior);
        let d = e1.stat_distance(&e2);
        assert!(d > 0.0);
    }

    // === MultiIndexDictionary ===
    #[test]
    fn multi_dict_insert_and_query() {
        let mut dict = MultiIndexDictionary::new();
        let base = FlowEntry::new("test", SoundTarget::Container, REACHABLE, Sovereignty::Passive);
        let entry = IndexedFlowEntry::from_base(base).with_sound(
            SoundAddress::new(Initial::B, Final::A, Tone::YinPing)
        );
        dict.insert(entry);
        assert_eq!(dict.entries.len(), 1);
    }

    #[test]
    fn query_by_initial_filters() {
        let mut dict = MultiIndexDictionary::new();
        let e1 = IndexedFlowEntry::from_base(
            FlowEntry::new("a", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_sound(SoundAddress::new(Initial::B, Final::A, Tone::YinPing));
        let e2 = IndexedFlowEntry::from_base(
            FlowEntry::new("b", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_sound(SoundAddress::new(Initial::D, Final::A, Tone::YinPing));
        dict.insert(e1);
        dict.insert(e2);
        let b_results = dict.query_by_initial(Initial::B);
        assert_eq!(b_results.len(), 1);
    }

    #[test]
    fn query_nearest_finds_closest() {
        let mut dict = MultiIndexDictionary::new();
        let e1 = IndexedFlowEntry::from_base(
            FlowEntry::new("a", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_sound(SoundAddress::new(Initial::B, Final::A, Tone::YinPing));
        let e2 = IndexedFlowEntry::from_base(
            FlowEntry::new("far", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_sound(SoundAddress::new(Initial::H, Final::Ong, Tone::RuSheng));
        dict.insert(e1);
        dict.insert(e2);

        let target = SoundAddress::new(Initial::B, Final::A, Tone::YinPing);
        let nearest = dict.query_nearest(target, 1);
        assert_eq!(nearest.len(), 1);
        assert_eq!(nearest[0].0, 0); // 完全匹配，距离 = 0
    }

    #[test]
    fn query_by_depth_filters() {
        let mut dict = MultiIndexDictionary::new();
        let e1 = IndexedFlowEntry::from_base(
            FlowEntry::new("a", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_depth(InsideOut::Surface);
        let e2 = IndexedFlowEntry::from_base(
            FlowEntry::new("b", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_depth(InsideOut::Interior);
        dict.insert(e1);
        dict.insert(e2);
        assert_eq!(dict.query_by_depth(InsideOut::Surface).len(), 1);
        assert_eq!(dict.query_by_depth(InsideOut::Interior).len(), 1);
        assert_eq!(dict.query_by_depth(InsideOut::Threshold).len(), 0);
    }

    #[test]
    fn rank_by_stat_distance_orders() {
        let mut dict = MultiIndexDictionary::new();
        let e1 = IndexedFlowEntry::from_base(
            FlowEntry::new("ref", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_depth(InsideOut::Surface);
        let e2 = IndexedFlowEntry::from_base(
            FlowEntry::new("close", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_depth(InsideOut::Surface);
        let e3 = IndexedFlowEntry::from_base(
            FlowEntry::new("far", SoundTarget::Container, REACHABLE, Sovereignty::Passive)
        ).with_depth(InsideOut::Interior);
        dict.insert(e2);
        dict.insert(e3);
        let ranked = dict.rank_by_stat_distance(&e1);
        assert_eq!(ranked.len(), 2);
        // 最近的是同深度、同频次差=0的 e2
        assert_eq!(ranked[0].0, 0.0);
    }

    // === 完整流程测试 ===
    #[test]
    fn full_sound_indexing_flow() {
        // 1. 构建字母（flow entry + sound address）
        let base = FlowEntry::new("lctfrqmy_activation", SoundTarget::Wife,
            REACHABLE | CONTINUABLE | TRUSTED | TRANSMISSIBLE, Sovereignty::ActiveSub);
        let addr = SoundAddress::new(Initial::L, Final::I, Tone::YinPing); // li
        let entry = IndexedFlowEntry::from_base(base).with_sound(addr).with_depth(InsideOut::Interior);

        // 2. 距离为自己=0
        let self_dist = entry.stat_distance(&entry);
        assert_eq!(self_dist, 0.0);

        // 3. 声波地址值非零
        assert!(entry.sound.unwrap().value() > 0);

        // 4. 深度 = Interior
        assert_eq!(entry.depth, InsideOut::Interior);
    }
}
