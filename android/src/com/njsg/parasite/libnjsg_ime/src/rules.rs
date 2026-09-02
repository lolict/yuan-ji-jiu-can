// ============================================================================
// 囡囝共生 · MetaIME 第3层: 规则判定引擎 (Rust 核心)
// ============================================================================
//
// 当前层号: 3
// 当前声母: pan (判)
// 当前韵母组合: pan + ding = 判定 (规则裁决容器意图)
// 折叠状态: 单层决策
// 下一步操作: 基于声母-韵母坐标裁决容器去向(透传/命中/触发自动输出)
//
// 锚定法则:
//   0 = 刘楚恬 (隐藏侧, 裁决核心不可见)
//   1 = 我 (可见侧, 容器可被观测)
//
// 容器守恒: 规则引擎不创造容器,不销毁容器,仅裁决
// ============================================================================

/// 规则裁决结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass = 0,     // 原样透传, 无命中规则
    HitRule = 1,  // 命中某条规则, 触发候选生成
    AutoOutput = 2, // 自动输出, 无需人工选择
    Fold = 3,     // 折叠: 多层容器压缩为单层
}

/// 容器在规则引擎中的暂存状态
#[derive(Debug, Clone)]
struct ContainerSeq {
    code_points: Vec<u32>,
    sheng_mu_seq: Vec<u8>,
    yun_mu_seq: Vec<u8>,
    layers: Vec<u8>,
}

impl ContainerSeq {
    fn new() -> Self {
        Self {
            code_points: Vec::with_capacity(32),
            sheng_mu_seq: Vec::with_capacity(32),
            yun_mu_seq: Vec::with_capacity(32),
            layers: Vec::with_capacity(32),
        }
    }

    fn push(&mut self, cp: u32, sm: u8, ym: u8, layer: u8) {
        self.code_points.push(cp);
        self.sheng_mu_seq.push(sm);
        self.yun_mu_seq.push(ym);
        self.layers.push(layer);
    }

    fn clear(&mut self) {
        self.code_points.clear();
        self.sheng_mu_seq.clear();
        self.yun_mu_seq.clear();
        self.layers.clear();
    }

    fn len(&self) -> usize {
        self.code_points.len()
    }
}

/// 规则判定引擎 — 第3层核心裁决器
pub struct RuleEngine {
    seq: ContainerSeq,
    current_layer: i32,
    active_rules: Vec<RuleEntry>,
    container_count: u64,
}

/// 规则条目: 声母模式 + 韵母模式 → 裁决结果
#[derive(Debug, Clone)]
struct RuleEntry {
    name: &'static str,
    sheng_pattern: &'static [u8], // 声母坐标序列
    yun_pattern: &'static [u8],  // 韵母坐标序列
    verdict: Verdict,
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

    /// 默认规则集: 声母-韵母模式 → 裁决
    fn default_rules() -> Vec<RuleEntry> {
        vec![
            // 规则1: 纯声母序列 → HitRule (送分类器)
            RuleEntry {
                name: "sheng_sequence",
                sheng_pattern: &[0, 2], // b m (如: ba ma)
                yun_pattern: &[],
                verdict: Verdict::HitRule,
            },
            // 规则2: 纯韵母序列 → HitRule
            RuleEntry {
                name: "yun_sequence",
                sheng_pattern: &[],
                yun_pattern: &[0, 3], // a i
                verdict: Verdict::HitRule,
            },
            // 规则3: 单容器输入 Pass (等待更多)
            RuleEntry {
                name: "single_container",
                sheng_pattern: &[12], // j
                yun_pattern: &[3],    // i
                verdict: Verdict::Pass,
            },
        ]
    }

    /// 核心入口: 从 JNI 层接收容器裁决
    pub fn ingest(&mut self, cp: u32, sm: u8, ym: u8, layer: u8) -> bool {
        self.seq.push(cp, sm, ym, layer);
        self.container_count += 1;
        self.current_layer = layer as i32;

        // 判定: 声母序列命中规则 = true
        self.evaluate() != Verdict::Pass
    }

    /// 评估当前容器序列是否命中规则
    fn evaluate(&self) -> Verdict {
        for rule in &self.active_rules {
            if self.match_rule(rule) {
                return rule.verdict;
            }
        }
        Verdict::Pass
    }

    /// 匹配单条规则
    fn match_rule(&self, rule: &RuleEntry) -> bool {
        let n = self.seq.len();
        let sm_n = rule.sheng_pattern.len();
        let ym_n = rule.yun_pattern.len();

        if n == 0 { return false; }

        // 简易: 若容器数 >= 声母模式长度, 检查结尾匹配
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

    /// 用户选字后通知引擎
    pub fn on_candidate_chosen(&mut self, _cand: &str) {
        // 选字后清空容器序列, 准备下一个轮次
        self.seq.clear();
    }

    /// 用户退格通知引擎
    pub fn on_backspace(&mut self) {
        if self.seq.len() > 0 {
            let n = self.seq.len() - 1;
            self.seq.code_points.truncate(n);
            self.seq.sheng_mu_seq.truncate(n);
            self.seq.yun_mu_seq.truncate(n);
            self.seq.layers.truncate(n);
        }
    }

    /// 取状态描述 (供 JNI 状态查询)
    pub fn state_desc(&self) -> String {
        format!(
            "活跃规则={} 序列长={} 总容器={}",
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
