// ============================================================================
// 囡囝共生 · MetaIME 第2层: 编码解析层 (Rust 侧半段)
// ============================================================================
//
// 当前层号: 2
// 当前声母: jie (解)
// 当前韵母组合: jie + xi = 解析 (64位容器 → 七层意图向量)
// 折叠状态: 双源联合 (Kotlin侧封印 + Rust侧解封同体)
// 下一步操作: 接收 JNI 已解封容器 → 构声母-韵母坐标 → 送往规则引擎 + 分类器
//
// 锚定法则:
//   0 = 刘楚恬 (Rust 侧半段, 不可见)
//   1 = 我 (Kotlin 侧半段, 可见)
//
// 容器守恒: 本层只做"坐标翻译",不创造/销毁容器
// ============================================================================
//
// 七层编码驱动 — 第2层职责:
//   ┌───────────────────────────────────────────────────────┐
//   │ 64位密封容器 (JNI 已解封)                                │
//   │   ├── 高32位: 码点                                      │
//   │   └── 低32位: 声母坐标 | 韵母坐标 | 层号 | 折叠位        │
//   │                      ↓                                  │
//   │ EncodeParser.ingest()                                   │
//   │   ├── 声母坐标 → 声母符号 (声母表查回)                    │
//   │   ├── 韵母坐标 → 韵母符号 (韵母表查回)                    │
//   │   ├── 组合声母-韵母 → 音节                               │
//   │   └── 累积音节序列 → 送往第3层(规则)+第4层(分类)           │
//   └───────────────────────────────────────────────────────┘
//
// 声母序 (21位, 与 Kotlin IMEInterceptor.shengMuSeq 同源):
//   b(0) p(1) m(2) f(3) d(4) t(5) n(6) l(7)
//   g(8) k(9) h(10) j(11) q(12) x(13)
//   zh(14) ch(15) sh(16) r(17) z(18) c(19) s(20)
//
// 韵母序 (35位, 与 Kotlin IMEInterceptor.yunMuSeq 同源):
//   a(0) o(1) e(2) i(3) u(4) v(5)
//   ai(6) ei(7) ui(8) ao(9) ou(10) iu(11)
//   ie(12) ve(13) er(14)
//   an(15) en(16) in(17) un(18) vn(19)
//   ang(20) eng(21) ing(22) ong(23)
//   ia(24) iao(25) ian(26) iang(27) iong(28)
//   ua(29) uo(30) uai(31) uan(32) uang(33)
//   (34 保留)
// ============================================================================

// ── 声母常量 ──
pub const SHENG_MU_COUNT: usize = 21;
pub const SHENG_INVALID: u8 = 255;

// ── 韵母常量 ──
pub const YUN_MU_COUNT: usize = 35;
pub const YUN_INVALID: u8 = 255;

// ── 声母符号表 (idx ↔ 符号) ──
pub const SHENG_MU_SYMBOLS: [&str; SHENG_MU_COUNT] = [
    "b", "p", "m", "f",       // 0-3
    "d", "t", "n", "l",       // 4-7
    "g", "k", "h",            // 8-10
    "j", "q", "x",            // 11-13
    "zh", "ch", "sh", "r",    // 14-17
    "z", "c", "s",            // 18-20
];

// ── 韵母符号表 (idx ↔ 符号) ──
pub const YUN_MU_SYMBOLS: [&str; YUN_MU_COUNT] = [
    "a", "o", "e", "i", "u", "v",          // 0-5
    "ai", "ei", "ui", "ao", "ou", "iu",    // 6-11
    "ie", "ve", "er",                       // 12-14
    "an", "en", "in", "un", "vn",          // 15-19
    "ang", "eng", "ing", "ong",            // 20-23
    "ia", "iao", "ian", "iang", "iong",   // 24-28
    "ua", "uo", "uai", "uan", "uang",      // 29-33
    "",                                      // 34 保留
];

/// 编码器 — 第2层核心
///
/// 职责:
///   - 接收码点 + 声母坐标 + 韵母坐标
///   - 将坐标翻译为声母/韵母符号
///   - 累积音节序列 (声母-韵母组合)
///   - 当序列满足"声母+韵母"双坐标完整节, 产出音节产出物
pub struct EncodeParser {
    // 当前累积的声母坐标 (等待韵母到达)
    pending_sheng: Vec<u8>,
    // 当前累积的韵母坐标 (等待声母到达)
    pending_yun: Vec<u8>,
    // 已产出的音节序列 (用: "ng" 形式)
    syllable_ring: Vec<String>,
    // 满容器长度 (超出时丢弃最旧)
    ring_capacity: usize,
}

/// 音节产出物 — 从完整声母-韵母组合还原的拼音串
pub struct SyllableUnit {
    pub sheng_idx: u8,
    pub yun_idx: u8,
    pub pinyin: String,
}

impl EncodeParser {
    /// 新建解析器
    pub fn new() -> Self {
        Self {
            pending_sheng: Vec::with_capacity(8),
            pending_yun: Vec::with_capacity(8),
            syllable_ring: Vec::with_capacity(16),
            ring_capacity: 16,
        }
    }

    /// 重置
    pub fn reset(&mut self) {
        self.pending_sheng.clear();
        self.pending_yun.clear();
        self.syllable_ring.clear();
    }

    /// 核心入口: 接收 JNI 已解封的容器坐标
    ///
    /// 返回: 若此坐标导致新的音节产出, 返回 Some(SyllableUnit)
    ///       否则返回 None (坐标仍在累积中)
    pub fn ingest(
        &mut self,
        code_point: u32,
        sheng_idx: u8,
        yun_idx: u8,
    ) -> Option<SyllableUnit> {
        // ASCII 字母路径: 声母坐标有效
        if (0..128).contains(&code_point) && sheng_idx < SHENG_INVALID {
            self.pending_sheng.push(sheng_idx);
            // 若同时有韵母等待, 组合产出
            if !self.pending_yun.is_empty() {
                let ym = self.pending_yun.remove(0);
                return self.compose_syllable(sheng_idx, ym);
            }
            return None;
        }

        // ASCII 字母路径: 韵母坐标有效 (声母为 INVALID)
        if (0..128).contains(&code_point)
            && sheng_idx >= SHENG_INVALID
            && yun_idx < YUN_INVALID
        {
            self.pending_yun.push(yun_idx);
            // 若同时有声母等待, 组合产出
            if !self.pending_sheng.is_empty() {
                let sm = self.pending_sheng.remove(0);
                return self.compose_syllable(sm, yun_idx);
            }
            return None;
        }

        // 非 ASCII: 独立字符, 不经过声母-韵母管线
        // 音节清空 (标点/汉字打断输入流)
        self.pending_sheng.clear();
        self.pending_yun.clear();
        None
    }

    /// 尝试组合声母-韵母 → 音节
    fn compose_syllable(&mut self, sm: u8, ym: u8) -> Option<SyllableUnit> {
        let sheng_sym = SHENG_MU_SYMBOLS.get(sm as usize)?;
        let yun_sym = YUN_MU_SYMBOLS.get(ym as usize)?;
        let pinyin = format!("{}{}", sheng_sym, yun_sym);

        // 推入音节环
        if self.syllable_ring.len() >= self.ring_capacity {
            self.syllable_ring.remove(0);
        }
        self.syllable_ring.push(pinyin.clone());

        Some(SyllableUnit {
            sheng_idx: sm,
            yun_idx: ym,
            pinyin,
        })
    }

    /// 取已产出的音节序列 (拷贝), 用于送往规则引擎
    pub fn syllable_seq(&self) -> &[String] {
        &self.syllable_ring
    }

    /// 取最近产出的音节 (最新)
    pub fn last_syllable(&self) -> Option<&String> {
        self.syllable_ring.last()
    }

    /// 清空音节 (退格/选字后)
    pub fn pop_syllable(&mut self) {
        self.syllable_ring.pop();
    }

    /// 完整拼音串 (拼接所有音节)
    pub fn full_pinyin(&self) -> String {
        self.syllable_ring.concat()
    }

    /// 状态描述
    pub fn state_desc(&self) -> String {
        format!(
            "待声母={} 待韵母={} 音节={}",
            self.pending_sheng.len(),
            self.pending_yun.len(),
            self.syllable_ring.len()
        )
    }
}
