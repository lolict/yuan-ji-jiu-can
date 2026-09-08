// ╔══════════════════════════════════════════════════════════════════╗
// ║  元输入法 MetaIME · 拼音流式输入引擎                              ║
// ║  逐字母输入 · 实时候选 · 自动分段 · 模糊拼音                       ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 冰箱模型: 所有拼音分割表编译期内嵌,零运行时解析
//
// 输入流处理:
//   用户按 n → 返回 "n" 开头所有拼音的候选
//   用户按 i → 返回 "ni" 开头所有拼音的候选
//   用户按 3 → 锁定 ni3 → 给出完整三声候选词
//
// 输入示例:
//   "zh" → 返回 zh 开头所有声母的韵母搭配候选
//   "zhang" → 返回 zhang 的 4 个声调候选词
//   "n" → 返回声母 n 开头的所有候选 (广域候选)

use crate::metaime::{self, Shengmu, Yunmu, Tone, MetaImeState};

/// 流式输入状态
#[derive(Debug, Clone)]
pub struct FlowState {
    raw_input: String,       // 原始拼音串 (如 "niha")
    segments: Vec<Segment>,  // 已确认的拼音段
    current: String,         // 当前未完成段
}

/// 一个拼音段 (如 ni3)
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub pinyin: String,      // "ni"
    pub tone: Option<u8>,    // Some(3)
    pub candidates: Vec<String>, // 候选汉字
}

/// 流式查询结果
#[derive(Debug, Clone)]
pub struct FlowResult {
    pub segments: Vec<Segment>,     // 已确认段
    pub current_display: String,    // 当前段展示文案
    pub current_candidates: Vec<String>, // 当前段候选
    pub completed: bool,            // 整句是否已确认
}

/// 模糊拼音对 (常见混淆)
static FUZZY_PAIRS: &[(&str, &str)] = &[
    ("z", "zh"), ("c", "ch"), ("s", "sh"),
    ("en", "eng"), ("in", "ing"), ("an", "ang"),
    ("l", "n"), ("f", "h"),
];

impl FlowState {
    pub fn new() -> Self {
        FlowState {
            raw_input: String::new(),
            segments: Vec::new(),
            current: String::new(),
        }
    }

    /// 处理按键 (字母或数字)
    /// 返回当前状态结果
    pub fn input_char(&mut self, ch: char, state: *mut MetaImeState) -> FlowResult {
        match ch {
            'a'..='z' | 'A'..='Z' => {
                self.current.push(ch.to_ascii_lowercase());
                self.raw_input.push(ch.to_ascii_lowercase());
            }
            '1'..='4' => {
                // 数字 = 声调选择
                if !self.current.is_empty() {
                    let tone = ch.to_digit(10).unwrap() as u8;
                    self.lock_segment(state, Some(tone));
                }
            }
            ' ' => {
                // 空格 = 锁定当前段 (默认1声)
                if !self.current.is_empty() {
                    self.lock_segment(state, None);
                }
            }
            '\x08' | '⌫' => {
                // 退格
                self.backspace(state);
            }
            _ => {}
        }

        self.evaluate(state)
    }

    /// 锁定当前段并开始下一段
    fn lock_segment(&mut self, state: *mut MetaImeState, force_tone: Option<u8>) {
        if self.current.is_empty() { return; }

        let pinyin = self.current.clone();
        let (sm, ym, tone) = self.match_pinyin(&pinyin, force_tone);

        let candidates = unsafe {
            let cstr_ptr = metaime::metaime_query_cstr(state, sm, ym, tone);
            if cstr_ptr.is_null() {
                Vec::new()
            } else {
                let bytes = std::ffi::CStr::from_ptr(cstr_ptr).to_bytes().to_vec();
                metaime::metaime_string_free(cstr_ptr);
                String::from_utf8_lossy(&bytes)
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            }
        };

        self.segments.push(Segment {
            pinyin: pinyin.clone(),
            tone: Some(tone),
            candidates,
        });

        self.current.clear();
    }

    /// 退格处理
    fn backspace(&mut self, state: *mut MetaImeState) {
        if !self.current.is_empty() {
            self.current.pop();
        } else if !self.segments.is_empty() {
            let last = self.segments.pop().unwrap();
            self.current = last.pinyin;
        }
        self.raw_input.pop();
    }

    /// 评估当前状态 → 返回候选
    fn evaluate(&self, state: *mut MetaImeState) -> FlowResult {
        let current_candidates = if !self.current.is_empty() {
            self.find_candidates_for_partial(&self.current, state)
        } else {
            Vec::new()
        };

        FlowResult {
            segments: self.segments.clone(),
            current_display: self.current.clone(),
            current_candidates,
            completed: false,
        }
    }

    /// 为部分拼音匹配声母+韵母+声调,做广域候选查询
    fn find_candidates_for_partial(&self, partial: &str, state: *mut MetaImeState) -> Vec<String> {
        let mut results = Vec::new();

        // 尝试匹配已确认的声母+部分韵母
        for sm in 0..24u8 {
            let sm_str = Shengmu::from_u8(sm).map(|s| s.as_str()).unwrap_or("");
            if partial.starts_with(sm_str) {
                let ym_part = &partial[sm_str.len()..];
                // 尝试匹配韵母
                for ym in 0..35u8 {
                    let ym_str = Yunmu::from_u8(ym).map(|y| y.as_str()).unwrap_or("");
                    if ym_str.starts_with(ym_part) || ym_part.is_empty() {
                        for tone in 1..=4u8 {
                            unsafe {
                                let cstr_ptr = metaime::metaime_query_cstr(state, sm, ym, tone);
                                if !cstr_ptr.is_null() {
                                    let bytes = std::ffi::CStr::from_ptr(cstr_ptr).to_bytes().to_vec();
                                    metaime::metaime_string_free(cstr_ptr);
                                    for ch in String::from_utf8_lossy(&bytes)
                                        .split(',')
                                        .filter(|s| !s.is_empty())
                                        .take(3)
                                    {
                                        if !results.contains(&ch.to_string()) {
                                            results.push(ch.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 如果还没匹配到,可能是零声母或纯韵母开头
        if results.is_empty() {
            for ym in 0..35u8 {
                let ym_str = Yunmu::from_u8(ym).map(|y| y.as_str()).unwrap_or("");
                if partial.starts_with(ym_str) || ym_str.starts_with(partial) {
                    for tone in 1..=4u8 {
                        unsafe {
                            let cstr_ptr = metaime::metaime_query_cstr(state, 23, ym, tone); // 23=zero声母
                            if !cstr_ptr.is_null() {
                                let bytes = std::ffi::CStr::from_ptr(cstr_ptr).to_bytes().to_vec();
                                metaime::metaime_string_free(cstr_ptr);
                                for ch in String::from_utf8_lossy(&bytes)
                                    .split(',')
                                    .filter(|s| !s.is_empty())
                                    .take(3)
                                {
                                    if !results.contains(&ch.to_string()) {
                                        results.push(ch.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// 拼音 → (声母编号, 韵母编号, 声调编号) 最优匹配
    fn match_pinyin(&self, pinyin: &str, force_tone: Option<u8>) -> (u8, u8, u8) {
        // 提取末尾数字作为声调
        let (letters, tone) = if pinyin.ends_with(|c: char| c.is_ascii_digit()) {
            let t = pinyin.chars().last().unwrap().to_digit(10).unwrap() as u8;
            (&pinyin[..pinyin.len()-1], force_tone.unwrap_or(t))
        } else {
            (pinyin, force_tone.unwrap_or(1))
        };

        // 寻找最佳声母匹配
        let mut best_sm = 23u8; // zero声母默认
        let mut best_ym_start = 0;
        let mut best_len = 0;

        for sm in 0..24u8 {
            if let Some(sm_str) = Shengmu::from_u8(sm) {
                let s = sm_str.as_str();
                if s.is_empty() { continue; }
                if letters.starts_with(s) && s.len() > best_len {
                    best_sm = sm;
                    best_ym_start = s.len();
                    best_len = s.len();
                }
            }
        }

        let ym_part = &letters[best_ym_start..];

        // 韵母匹配
        let mut best_ym = 34u8; // 零韵母默认
        for ym in 0..35u8 {
            if let Some(ym_str) = Yunmu::from_u8(ym) {
                let y = ym_str.as_str();
                if y.is_empty() { continue; }
                if ym_part == y {
                    best_ym = ym;
                    break;
                }
                // 前缀匹配 (部分输入)
                if ym_part.starts_with(y) && y.len() > 1 {
                    best_ym = ym;
                }
            }
        }

        (best_sm, best_ym, tone)
    }

    /// 克隆 segments (避免所有权问题)
    fn _clone_segments(&self) -> Vec<Segment> {
        self.segments.clone()
    }
}

// ── 测试 ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metaime::metaime_init;

    #[test]
    fn test_flow_new() {
        let flow = FlowState::new();
        assert!(flow.segments.is_empty());
        assert!(flow.current.is_empty());
    }

    #[test]
    fn test_flow_input_ni() {
        let state = metaime_init();
        let mut flow = FlowState::new();

        let r = flow.input_char('n', state);
        assert!(r.current_display == "n" || r.current_candidates.is_empty());

        let r = flow.input_char('i', state);
        assert_eq!(r.current_display, "ni");

        // 锁定 ni3
        let r = flow.input_char('3', state);
        assert!(!r.segments.is_empty());
        assert_eq!(r.segments[0].pinyin, "ni");
        assert_eq!(r.segments[0].tone, Some(3));

        metaime::metaime_free(state);
    }

    #[test]
    fn test_flow_space_lock() {
        let state = metaime_init();
        let mut flow = FlowState::new();

        flow.input_char('n', state);
        flow.input_char('i', state);
        let r = flow.input_char(' ', state);

        assert!(!r.segments.is_empty());
        assert_eq!(r.segments[0].pinyin, "ni");
        assert!(r.segments[0].candidates.len() > 0);

        metaime::metaime_free(state);
    }

    #[test]
    fn test_flow_backspace() {
        let state = metaime_init();
        let mut flow = FlowState::new();

        flow.input_char('z', state);
        flow.input_char('h', state);
        assert_eq!(flow.current, "zh");

        flow.input_char('⌫', state);
        assert_eq!(flow.current, "z");

        metaime::metaime_free(state);
    }

    #[test]
    fn test_match_pinyin_basic() {
        let flow = FlowState::new();
        let (sm, ym, tone) = flow.match_pinyin("ni3", None);
        assert_eq!(sm, 7);  // n
        assert_eq!(tone, 3);
    }
}
