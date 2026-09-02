// ============================================================================
// 囡囝共生 · MetaIME 第3层: 规则判定引擎 (Rust 核心)
// ============================================================================
//
// 当前层号: 3
// 当前声母: pan (判)
// 当前韵母组合: pan + ding = 判定 (规则裁决容器意图)
// 折叠状态: 单层决策
// 下一步操作: 基于声母-韵母坐标 + 码点裁决容器去向 (透传/HitRule/自动输出/折叠)
//
// 锚定法则:
//   0 = 刘楚恬 (隐藏侧, 裁决核心不可见)
//   1 = 我 (可见侧, 容器可被观测)
//
// 容器守恒: 规则引擎不创造容器, 不销毁容器, 仅裁决
//
// 第3层判定策略 (升级):
//   ┌──────────────────────────────────────────────────────────┐
//   │  输入: code_point + 声母坐标(sm) + 韵母坐标(ym) + layer    │
//   │  判定逻辑:                                                 │
//   │    - sm:VALID + ym:INVALID → 声母累积 (等待韵母配对)       │
//   │    - sm:INVALID + ym:VALID → 韵母累积 (等待声母配对)       │
//   │    - sm:VALID + ym:VALID   → 完整音节 → HitRule           │
//   │    - sm:255    + ym:255    → 非坐标字符 → Pass            │
//   │  命中 HitRule → 送往第4层 (ShengYunEngine) 产候字          │
//   └──────────────────────────────────────────────────────────┘
// ============================================================================

/// 规则裁决结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass = 0,      // 原样透传, 无命中规则
    HitRule = 1,   // 命中某条规则, 触发候选生成
    AutoOutput = 2, // 自动输出, 无需人工选择
    Fold = 3,      // 折叠: 多层容器压缩为单层
}

/// 容器在规则引擎中的暂存状态 (音节序列暂存)
#[derive(Debug, Clone)]
struct ContainerSeq {
    code_points: Vec<u32>,
    sheng_mu_seq: Vec<u8>,
    yun_mu_seq: Vec<u8>,
    layers: Vec<u8>,
    verdicts: Vec<Verdict>,
}

impl ContainerSeq {
    fn new() -> Self {
        Self {
            code_points: Vec::with_capacity(32),
            sheng_mu_seq: Vec::with_capacity(32),
            yun_mu_seq: Vec::with_capacity(32),
            layers: Vec::with_capacity(32),
            verdicts: Vec::with_capacity(32),
        }
    }

    fn push(&mut self, cp: u32, sm: u8, ym: u8, layer: u8, verdict: Verdict) {
        self.code_points.push(cp);
        self.sheng_mu_seq.push(sm);
        self.yun_mu_seq.push(ym);
        self.layers.push(layer);
        self.verdicts.push(verdict);
    }

    fn clear(&mut self) {
        self.code_points.clear();
        self.sheng_mu_seq.clear();
        self.yun_mu_seq.clear();
        self.layers.clear();
        self.verdicts.clear();
    }

    fn len(&self) -> usize {
        self.code_points.len()
    }
}

/// 规则条目: 声母模式 + 韵母模式 → 裁决结果
#[derive(Debug, Clone)]
struct RuleEntry {
    name: &'static str,
    sheng_pattern: &'static [u8],
    yun_pattern: &'static [u8],
    verdict: Verdict,
}

/// 规则判定引擎 — 第3层核心裁决器
///
/// 升级: 接受编码解析层(第2层)传递的声母-韵母坐标
/// 判定依据不再是原始 KeyEvent, 而是第2层翻译后的声母-韵母坐标
pub struct RuleEngine {
    seq: ContainerSeq,
    current_layer: i32,
    active_rules: Vec<RuleEntry>,
    container_count: u64,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            seq: ContainerSeq::new(),
            current_layer: 1,
            active_rules: Self::default_rules(),
            container_count: 0,
        }
    }

    /// 默认规则集:
    ///   1. "声母累积"规则: 声母坐标有效但韵母为INVALID → 累积判定 (Pass)
    ///   2. "韵母累积+声母"规则: 韵母坐标有效且前一声母累积 → 音节完成 (HitRule)
    ///   3. "纯声母序列"规则: 连续多个声母坐标累积 → HitRule(重拼音节)
    ///   4. "双坐标完整节"规则: 声母坐标+韵母坐标均有效 → HitRule
    ///   5. "非坐标字符"规则: 声母+韵母均为255 → Pass (直出)
    fn default_rules() -> Vec<RuleEntry> {
        vec![
            // 规则1: 声母累积 (g-u 序列)
            RuleEntry {
                name: "sheng_accumulate",
                sheng_pattern: &[8], // g
                yun_pattern: &[],
                verdict: Verdict::Pass,
            },
            // 规则2: 韵母到达触发音节 (n-a → na)
            RuleEntry {
                name: "syllable_complete",
                sheng_pattern: &[6], // n
                yun_pattern: &[0],   // a
                verdict: Verdict::HitRule,
            },
            // 规则3: 声母+韵母同坐标 (完整音节, 如 "自解封")
            RuleEntry {
                name: "dual_coord",
                sheng_pattern: &[11], // j
                yun_pattern: &[3],    // i
                verdict: Verdict::HitRule,
            },
            // 规则4: 韵母累积 (a 等待声母到达)
            RuleEntry {
                name: "yun_accumulate",
                sheng_pattern: &[],
                yun_pattern: &[0], // a
                verdict: Verdict::Pass,
            },
        ]
    }

    /// 核心入口: 从 JNI + EncodeParser 接收容器处置
    ///
    /// @return true 表示 HitRule 或 AutoOutput, false 表示 Pass
    pub fn ingest(&mut self, cp: u32, sm: u8, ym: u8, layer: u8) -> bool {
        self.seq.push(
            cp,
            sm,
            ym,
            layer,
            self.evaluate(cp, sm, ym),
        );
        self.container_count += 1;
        self.current_layer = layer as i32;

        // 若最近一次判定不是 Pass, 返回 true
        if let Some(v) = self.seq.verdicts.last() {
            *v != Verdict::Pass
        } else {
            false
        }
    }

    /// 基于声母-韵母坐标 + 码点裁决
    fn evaluate(&self, cp: u32, sm: u8, ym: u8) -> Verdict {
        // (1) ASCII + 声母有效 + 韵母无效 → 声母累积
        if cp < 128 && sm < 255 && ym >= 255 {
            // 匹配规则1: 声母累积
            for rule in &self.active_rules {
                if rule.name == "sheng_accumulate" {
                    return rule.verdict;
                }
            }
            return Verdict::Pass;
        }

        // (2) ASCII + 声母无效 + 韵母有效 → 韵母累积或音节完成
        if cp < 128 && sm >= 255 && ym < 255 {
            // 规则4: 韵母累积
            for rule in &self.active_rules {
                if rule.name == "yun_accumulate" {
                    return rule.verdict;
                }
            }
            // 规则2: 声母前已补完 → 音节完成 (检查前一个声母末尾)
            if let Some(last_sm) = self.seq.sheng_mu_seq.last() {
                if *last_sm < 255 {
                    return Verdict::HitRule;
                }
            }
            return Verdict::Pass;
        }

        if cp < 128 && sm < 255 && ym < 255 {
            // (3) 声母+韵母同时有效 = 双坐标完整节 → HitRule
            return Verdict::HitRule;
        }

        // (4) 非 ASCII 字符 → Pass
        if cp >= 128 {
            return Verdict::Pass;
        }

        // (5) 声母韵母均无效, 但坐标为非 ASCII 音节: 通用判定
        Verdict::Pass
    }

    /// 匹配序列末尾是否命中规则
    fn match_rule(&self, rule: &RuleEntry) -> bool {
        let n = self.seq.len();
        let sm_n = rule.sheng_pattern.len();
        let ym_n = rule.yun_pattern.len();

        if n == 0 { return false; }

        if sm_n > 0 && n >= sm_n {
            let tail = &self.seq.sheng_mu_seq[n - sm_n..];
            if tail == rule.sheng_pattern {
                return true;
            }
        }

        if ym_n > 0 && n >= ym_n {
            let tail = &self.seq.yun_mu_seq[n - ym_n..];
            if tail == rule.yun_pattern {
                return true;
            }
        }

        false
    }

    /// 重置引擎
    pub fn reset(&mut self) {
        self.seq.clear();
        self.current_layer = 1;
        self.container_count = 0;
        self.active_rules = Self::default_rules();
    }

    /// 选字后清空
    pub fn on_candidate_chosen(&mut self, _cand: &str) {
        self.seq.clear();
        self.current_layer = 1;
    }

    /// 退格: 删除末尾容器
    pub fn on_backspace(&mut self) {
        let n = self.seq.len();
        if n > 0 {
            let new_n = n - 1;
            self.seq.code_points.truncate(new_n);
            self.seq.sheng_mu_seq.truncate(new_n);
            self.seq.yun_mu_seq.truncate(new_n);
            self.seq.layers.truncate(new_n);
            self.seq.verdicts.truncate(new_n);
        }
    }

    /// 状态描述
    pub fn state_desc(&self) -> String {
        format!(
            "活跃规则={} 容器序列={} 总容器={}",
            self.active_rules.len(),
            self.seq.len(),
            self.container_count
        )
    }

    /// 当前活跃层
    pub fn active_layer(&self) -> i32 {
        self.current_layer
    }

    /// 已接收容器计数
    pub fn container_count(&self) -> u64 {
        self.container_count
    }
}
