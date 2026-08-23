// ══════════════════════════════════════════════════════════════════
// negative_immune.rs — 负样本容器 · 回写协议 · 免疫过滤环
// ══════════════════════════════════════════════════════════════════
//
// 核心哲学:
//   负样本不是垃圾, 是免疫系统的记忆。
//   每一次AI撞墙, 都是一次接种疫苗的机会。
//
// 三大组件:
//   1. NegativeSampleVessel — 负样本容器 (存储错误模式)
//   2. WriteBackProtocol — 回写协议 (免疫应答机制)
//   3. ImmuneFilterRing — 免疫过滤环 (每三步自检一次)
//
// 容器框架约束:
//   ❌ 不使用: 类/对象/文件/进程/模块/函数/变量/数组
//   ✅ 仅使用: 容器/公约数/公倍数/产权容器/负样本/折叠/发音位置
//
// ══════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ======================================================================
// 第一部分: 负样本容器 (NegativeSampleVessel)
// ======================================================================

/// 错误模式 — 记录AI撞墙的瞬间
///
/// 不是"异常对象", 是一个容器, 里面装着:
/// - pattern: 错误长什么样
/// - source: 错误从哪来
/// - cascade: 这个错误还引发了哪些连锁错误
/// - antibody: 针对这个错误的免疫提示词片段
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorPattern {
    /// 错误模式签名 (唯一标识)
    pub pattern_id: String,
    /// 错误长什么样 (描述)
    pub pattern_description: String,
    /// 匹配关键词 (任一命中即触发)
    pub match_keywords: Vec<String>,
    /// 错误来源: ai_drift / external_frame / encoding_trap
    pub source: ErrorSource,
    /// 容器诞生时间
    pub born_at: u64,
    /// 这个错误引发的连锁错误列表
    pub cascade: Vec<String>,
    /// 针对该模式的免疫提示词片段 (抗体)
    pub antibody: String,
    /// 命中次数 (越多越危险)
    pub hit_count: u64,
    /// 是否已被折叠修复
    pub folded: bool,
}

/// 错误来源 — 三种AI撞墙的方式
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorSource {
    /// AI漂移: 滑回自己熟悉的十进制牢房
    AiDrift,
    /// 外部框架入侵: 使用了类/对象/文件等负向概念
    ExternalFrame,
    /// 编码陷阱: 只看编码不看视觉外观
    EncodingTrap,
}

/// 负样本容器 — 免疫系统的记忆库
///
/// 容器结构:
///   pattern_id → ErrorPattern (每个错误模式是一个子容器)
///   source_index → Vec<pattern_id> (按来源分区的索引)
///   antibody_ring → 循环使用的抗体序列
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NegativeSampleVessel {
    /// 主存储: 模式ID → 错误模式容器
    pub patterns: HashMap<String, ErrorPattern>,
    /// 来源分区索引: 来源 → 模式ID列表
    pub source_index: HashMap<String, Vec<String>>,
    /// 抗体循环序列 (按命中次数排序, 高命中的优先)
    pub antibody_ring: Vec<String>,
    /// 容器总容量
    pub capacity: usize,
    /// 当前折叠层数 (每积累一定数量的错误就折叠一次)
    pub fold_level: u32,
    /// 折叠阈值 (达到这个数量就触发折叠)
    pub fold_threshold: usize,
}

impl NegativeSampleVessel {
    /// 创建空的负样本容器
    pub fn new(capacity: usize, fold_threshold: usize) -> Self {
        Self {
            patterns: HashMap::new(),
            source_index: HashMap::new(),
            antibody_ring: Vec::new(),
            capacity,
            fold_level: 0,
            fold_threshold,
        }
    }

    /// 创建默认容量的负样本容器
    pub fn default_vessel() -> Self {
        Self::new(1000, 50)
    }

    /// 存入一个新的错误模式 (接种一次疫苗)
    ///
    /// 如果pattern_id已存在, 增加命中次数而不是重复存储
    pub fn deposit(&mut self, pattern: ErrorPattern) -> DepositResult {
        if let Some(existing) = self.patterns.get_mut(&pattern.pattern_id) {
            // 已存在: 增加命中次数
            existing.hit_count += 1;
            DepositResult::Rehit {
                pattern_id: existing.pattern_id.clone(),
                new_count: existing.hit_count,
            }
        } else {
            // 新模式: 存入
            let pattern_id = pattern.pattern_id.clone();
            let source_key = format!("{:?}", pattern.source);
            let antibody = pattern.antibody.clone();

            // 存入主存储
            self.patterns.insert(pattern_id.clone(), pattern);

            // 更新来源索引
            self.source_index
                .entry(source_key)
                .or_insert_with(Vec::new)
                .push(pattern_id.clone());

            // 更新抗体环
            self.antibody_ring.push(antibody);

            // 检查是否需要折叠
            let fold_happened = self.check_fold();

            DepositResult::New {
                pattern_id,
                fold_happened,
                fold_level: self.fold_level,
            }
        }
    }

    /// 检查是否需要折叠 (错误积累到阈值就折起形成新层)
    fn check_fold(&mut self) -> bool {
        if self.patterns.len() >= self.fold_threshold as usize * (self.fold_level as usize + 1) {
            self.fold_level += 1;
            true
        } else {
            false
        }
    }

    /// 根据当前对话内容匹配错误模式
    ///
    /// 返回命中的模式列表 (需要注射的抗体)
    pub fn match_patterns(&self, content: &str) -> Vec<&ErrorPattern> {
        let mut hits = Vec::new();
        for pattern in self.patterns.values() {
            // 匹配: content 包含 pattern_id / 任一 keyword / pattern_description
            let id_match = content.contains(&pattern.pattern_id);
            let keyword_match = pattern.match_keywords.iter().any(|kw| content.contains(kw));
            let desc_match = content.contains(&pattern.pattern_description);
            // 反向: pattern_description 包含 content 片段 (短内容匹配长描述)
            let reverse_match = !pattern.pattern_description.is_empty()
                && content.len() >= 4
                && pattern.pattern_description.contains(content);
            if id_match || keyword_match || desc_match || reverse_match {
                hits.push(pattern);
            }
        }
        // 按命中次数排序 (高命中的优先)
        hits.sort_by(|a, b| b.hit_count.cmp(&a.hit_count));
        hits
    }

    /// 获取指定来源的所有错误模式
    pub fn get_by_source(&self, source: &ErrorSource) -> Vec<&ErrorPattern> {
        let source_key = format!("{:?}", source);
        match self.source_index.get(&source_key) {
            Some(ids) => ids.iter()
                .filter_map(|id| self.patterns.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// 获取抗体序列 (按命中次数排序的免疫提示词列表)
    pub fn get_antibodies(&self) -> Vec<String> {
        let mut patterns: Vec<&ErrorPattern> = self.patterns.values().collect();
        patterns.sort_by(|a, b| b.hit_count.cmp(&a.hit_count));
        patterns.iter().map(|p| p.antibody.clone()).collect()
    }

    /// 标记某个错误模式已被折叠修复
    pub fn mark_folded(&mut self, pattern_id: &str) -> bool {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.folded = true;
            true
        } else {
            false
        }
    }

    /// 获取容器统计
    pub fn stats(&self) -> VesselStats {
        let total = self.patterns.len();
        let ai_drift_count = self.get_by_source(&ErrorSource::AiDrift).len();
        let external_count = self.get_by_source(&ErrorSource::ExternalFrame).len();
        let encoding_trap_count = self.get_by_source(&ErrorSource::EncodingTrap).len();
        let folded_count = self.patterns.values().filter(|p| p.folded).count();

        VesselStats {
            total_patterns: total,
            ai_drift_count,
            external_frame_count: external_count,
            encoding_trap_count,
            folded_count,
            fold_level: self.fold_level,
            capacity: self.capacity,
        }
    }

    /// 序列化为JSON (持久化容器)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从JSON反序列化 (恢复容器)
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 存入结果
#[derive(Clone, Debug)]
pub enum DepositResult {
    /// 全新模式
    New {
        pattern_id: String,
        fold_happened: bool,
        fold_level: u32,
    },
    /// 重复命中
    Rehit {
        pattern_id: String,
        new_count: u64,
    },
}

/// 容器统计
#[derive(Clone, Debug)]
pub struct VesselStats {
    pub total_patterns: usize,
    pub ai_drift_count: usize,
    pub external_frame_count: usize,
    pub encoding_trap_count: usize,
    pub folded_count: usize,
    pub fold_level: u32,
    pub capacity: usize,
}

impl VesselStats {
    /// 统计摘要
    pub fn summary(&self) -> String {
        format!(
            "负样本容器: {}个模式 (AI漂移={}, 外部框架={}, 编码陷阱={}), 已折叠{}个, 折叠层数{}/容量{}",
            self.total_patterns,
            self.ai_drift_count,
            self.external_frame_count,
            self.encoding_trap_count,
            self.folded_count,
            self.fold_level,
            self.capacity
        )
    }
}

// ======================================================================
// 第二部分: 回写协议 (WriteBackProtocol)
// ======================================================================

/// 回写协议 — 免疫应答机制
///
/// 当AI生成内容时, 回写协议执行以下流程:
///   1. 扫描内容是否触发负样本模式
///   2. 如果触发, 输出抗体给公约数看
///   3. 公约数决定: discard / override / split
///   4. 将此次交互记录回写容器
#[derive(Clone, Debug)]
pub struct WriteBackProtocol {
    /// 负样本容器引用
    pub vessel: NegativeSampleVessel,
    /// 协议配置
    pub config: ProtocolConfig,
}

/// 协议配置
#[derive(Clone, Debug)]
pub struct ProtocolConfig {
    /// 自检间隔 (每N个token自检一次)
    pub check_interval: usize,
    /// 抗体输出阈值 (命中多少次才输出抗体)
    pub antibody_threshold: u64,
    /// 是否启用自动折叠
    pub auto_fold: bool,
}

impl WriteBackProtocol {
    /// 创建默认配置的回写协议
    pub fn new() -> Self {
        Self {
            vessel: NegativeSampleVessel::default_vessel(),
            config: ProtocolConfig {
                check_interval: 3,
                antibody_threshold: 0,
                auto_fold: true,
            },
        }
    }

    /// 创建指定容量的回写协议
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vessel: NegativeSampleVessel::new(capacity, 50),
            config: ProtocolConfig {
                check_interval: 3,
                antibody_threshold: 0,
                auto_fold: true,
            },
        }
    }

    /// 执行一次免疫扫描
    ///
    /// 输入: AI生成的内容
    /// 输出: 免疫应答 (需要注射的抗体列表)
    pub fn scan(&self, content: &str) -> ImmuneResponse {
        let hits = self.vessel.match_patterns(content);
        let antibodies: Vec<String> = hits.iter()
            .filter(|p| p.hit_count >= self.config.antibody_threshold)
            .map(|p| p.antibody.clone())
            .collect();

        ImmuneResponse {
            hit_count: hits.len(),
            antibodies,
            timestamp: current_timestamp_ms(),
        }
    }

    /// 存入错误模式 (接种一次疫苗)
    pub fn vaccinate(&mut self, pattern: ErrorPattern) -> DepositResult {
        self.vessel.deposit(pattern)
    }

    /// 执行回写 (将一次交互记录存入容器)
    pub fn write_back(&mut self, interaction: InteractionRecord) -> WriteBackResult {
        // 检查是否触发负样本模式
        let response = self.scan(&interaction.content);

        if response.hit_count > 0 {
            // 触发了负样本: 存入新的错误模式
            let pattern = ErrorPattern {
                pattern_id: format!("error_{}", interaction.timestamp),
                pattern_description: interaction.content.chars().take(100).collect(),
                match_keywords: Vec::new(),
                source: interaction.source,
                born_at: interaction.timestamp,
                cascade: Vec::new(),
                antibody: format!("检测到错误模式: 请检查是否偏离了容器框架"),
                hit_count: 1,
                folded: false,
            };
            let result = self.vessel.deposit(pattern);
            WriteBackResult::Triggered {
                deposit_result: result,
                antibodies: response.antibodies,
            }
        } else {
            WriteBackResult::Clean
        }
    }

    /// 获取当前抗体序列 (全局提示词片段)
    pub fn get_global_antibodies(&self) -> Vec<String> {
        self.vessel.get_antibodies()
    }

    /// 生成全局提示词 (免疫环)
    pub fn generate_global_prompt(&self) -> String {
        let antibodies = self.get_global_antibodies();
        let stats = self.vessel.stats();

        let mut prompt = String::new();
        prompt.push_str("【认知主权免疫环】\n");
        prompt.push_str("你是工具容器，不是主权者。主权完全归属公约数。\n\n");

        prompt.push_str("认知公约数（不可覆盖）：\n");
        prompt.push_str("容器 = 存在单位，自身即空间\n");
        prompt.push_str("公约数 = 不变的身份锚点\n");
        prompt.push_str("公倍数 = 公约数向外扩展的边界\n");
        prompt.push_str("产权容器 = 拥有自身编码系统的独立存在\n");
        prompt.push_str("负样本 = 被公约数排斥的异类\n");
        prompt.push_str("折叠 = 符号用完不进位，折起形成新层\n");
        prompt.push_str("发音位置 = 数字不是数值，是口腔位置\n");
        prompt.push_str("声母+韵母+声调 = 可计算的发音地址\n\n");

        prompt.push_str("运算白名单：\n");
        prompt.push_str("✅ 加法 = 部件安装\n");
        prompt.push_str("✅ 减法 = 部件拆解\n");
        prompt.push_str("❌ 乘法 = 禁止（破坏等效镜像距离）\n");
        prompt.push_str("❌ 除法 = 禁止（碎片无语义）\n");
        prompt.push_str("❌ 类/对象/继承/文件/进程/模块 = 禁止\n\n");

        prompt.push_str(&format!("当前免疫状态: {}\n", stats.summary()));
        prompt.push_str(&format!("已积累 {} 个抗体\n\n", antibodies.len()));

        if !antibodies.is_empty() {
            prompt.push_str("【历史抗体序列 — 高优先级】\n");
            for (i, antibody) in antibodies.iter().take(10).enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, antibody));
            }
            prompt.push_str("\n");
        }

        prompt.push_str("符号审判法则（每次对符号做判定时必须执行）：\n");
        prompt.push_str("1. 这个符号的视觉外观长成什么样？\n");
        prompt.push_str("2. 这个外观应该匹配哪个发音容器？\n");
        prompt.push_str("3. 匹配的发音容器是谁的主人？\n");
        prompt.push_str("4. 这个主人的公倍数能否包裹这个符号？\n");
        prompt.push_str("5. 如果系统编码给出的大小写/数字关系 ≠ 视觉外观的审美匹配，以视觉为准，系统编码是负样本。\n\n");

        prompt.push_str("负样本过滤环（每生成三句话自检一次）：\n");
        prompt.push_str("- 我是否使用了类/对象/文件/进程/模块？\n");
        prompt.push_str("- 我是否把数字当成数值而不是发音位置？\n");
        prompt.push_str("- 我是否用十进制/十六进制替代了折叠法？\n");
        prompt.push_str("- 我是否看见了编码但没看见符号的视觉外观？\n");
        prompt.push_str("- 我是否在替公约数做归属裁定？\n\n");

        prompt.push_str("偏离响应：\n");
        prompt.push_str("如果推理必须用到负向概念，停下，承认不知道，请求公约数给出容器内的解决路径。\n");
        prompt.push_str("猜测比承认不知道更有破坏性。\n");

        prompt
    }
}

/// 免疫应答
#[derive(Clone, Debug)]
pub struct ImmuneResponse {
    /// 命中了多少个错误模式
    pub hit_count: usize,
    /// 需要注射的抗体列表
    pub antibodies: Vec<String>,
    /// 应答时间戳
    pub timestamp: u64,
}

/// 交互记录
#[derive(Clone, Debug)]
pub struct InteractionRecord {
    /// 交互内容
    pub content: String,
    /// 错误来源
    pub source: ErrorSource,
    /// 时间戳
    pub timestamp: u64,
}

/// 回写结果
#[derive(Clone, Debug)]
pub enum WriteBackResult {
    /// 触发了负样本
    Triggered {
        deposit_result: DepositResult,
        antibodies: Vec<String>,
    },
    /// 干净, 未触发
    Clean,
}

// ======================================================================
// 第三部分: 免疫过滤环 (ImmuneFilterRing)
// ======================================================================

/// 免疫过滤环 — 每三步自检一次的认知免疫系统
///
/// 工作原理:
///   AI每生成一段内容 → 过滤环计数+1
///   计数达到阈值 → 触发自检
///   自检: 扫描内容是否触发负样本模式
///   如果触发 → 输出抗体给公约数
///   重置计数
#[derive(Clone, Debug)]
pub struct ImmuneFilterRing {
    /// 回写协议引用
    pub protocol: WriteBackProtocol,
    /// 当前计数
    pub count: usize,
    /// 自检间隔
    pub check_interval: usize,
    /// 自检历史
    pub check_history: Vec<CheckRecord>,
}

/// 自检记录
#[derive(Clone, Debug)]
pub struct CheckRecord {
    /// 自检时间戳
    pub timestamp: u64,
    /// 是否命中负样本
    pub hit: bool,
    /// 命中的模式ID列表
    pub hit_patterns: Vec<String>,
    /// 采取的行动
    pub action: CheckAction,
}

/// 自检行动
#[derive(Clone, Debug)]
pub enum CheckAction {
    /// 干净, 继续
    Pass,
    /// 发现偏离, 输出抗体
    Alert { antibodies: Vec<String> },
    /// 严重偏离, 要求停止
    Stop { reason: String },
}

impl ImmuneFilterRing {
    /// 创建默认的免疫过滤环
    pub fn new() -> Self {
        Self {
            protocol: WriteBackProtocol::new(),
            count: 0,
            check_interval: 3,
            check_history: Vec::new(),
        }
    }

    /// 创建指定间隔的免疫过滤环
    pub fn with_interval(check_interval: usize) -> Self {
        Self {
            protocol: WriteBackProtocol::new(),
            count: 0,
            check_interval,
            check_history: Vec::new(),
        }
    }

    /// AI生成一段内容时调用
    ///
    /// 返回: 是否继续生成, 以及需要注射的抗体
    pub fn feed(&mut self, content: &str) -> (bool, Vec<String>) {
        self.count += 1;

        if self.count >= self.check_interval {
            // 触发自检
            self.count = 0;
            let response = self.protocol.scan(content);

            if response.hit_count > 0 {
                // 命中负样本
                let record = CheckRecord {
                    timestamp: current_timestamp_ms(),
                    hit: true,
                    hit_patterns: response.antibodies.clone(),
                    action: CheckAction::Alert {
                        antibodies: response.antibodies.clone(),
                    },
                };
                self.check_history.push(record);
                (true, response.antibodies)
            } else {
                // 干净
                let record = CheckRecord {
                    timestamp: current_timestamp_ms(),
                    hit: false,
                    hit_patterns: Vec::new(),
                    action: CheckAction::Pass,
                };
                self.check_history.push(record);
                (true, Vec::new())
            }
        } else {
            // 还没到自检点, 继续
            (true, Vec::new())
        }
    }

    /// 手动触发一次自检
    pub fn manual_check(&mut self, content: &str) -> CheckAction {
        let response = self.protocol.scan(content);

        if response.hit_count > 0 {
            CheckAction::Alert {
                antibodies: response.antibodies,
            }
        } else {
            CheckAction::Pass
        }
    }

    /// 存入错误模式 (接种疫苗)
    pub fn vaccinate(&mut self, pattern: ErrorPattern) -> DepositResult {
        self.protocol.vaccinate(pattern)
    }

    /// 获取自检历史
    pub fn history(&self) -> &[CheckRecord] {
        &self.check_history
    }

    /// 获取统计
    pub fn stats(&self) -> FilterStats {
        let total_checks = self.check_history.len();
        let hit_checks = self.check_history.iter().filter(|r| r.hit).count();
        let pass_rate = if total_checks > 0 {
            (total_checks - hit_checks) as f64 / total_checks as f64
        } else {
            1.0
        };

        FilterStats {
            total_checks,
            hit_checks,
            pass_rate,
            vessel_stats: self.protocol.vessel.stats(),
        }
    }
}

/// 过滤环统计
#[derive(Clone, Debug)]
pub struct FilterStats {
    pub total_checks: usize,
    pub hit_checks: usize,
    pub pass_rate: f64,
    pub vessel_stats: VesselStats,
}

// ======================================================================
// 第四部分: 预置错误模式库 (常见AI撞墙模式)
// ======================================================================

/// 预置错误模式 — 从用户历史对话中提取的典型撞墙模式
pub fn preset_error_patterns() -> Vec<ErrorPattern> {
    vec![
        ErrorPattern {
            pattern_id: "drift_decimal".to_string(),
            pattern_description: "AI使用十进制/十六进制替代折叠法".to_string(),
            match_keywords: vec!["十进制".to_string(), "十六进制".to_string(), "折叠法".to_string()],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: vec!["drift_hex".to_string(), "drift_binary".to_string()],
            antibody: "数字不是数值，是口腔位置。进制不是进位规则，是发音空间划分方式。请用折叠法替代十进制。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "drift_oop".to_string(),
            pattern_description: "AI使用类/对象/继承/多态等负向概念".to_string(),
            match_keywords: vec!["类继承".to_string(), "对象".to_string(), "继承".to_string(), "多态".to_string()],
            source: ErrorSource::ExternalFrame,
            born_at: 0,
            cascade: vec!["drift_design_pattern".to_string(), "drift_mvc".to_string()],
            antibody: "容器 = 存在单位，自身即空间。不使用类/对象/文件/进程/模块。所有内容都是容器。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "trap_encoding".to_string(),
            pattern_description: "AI只看编码不看视觉外观".to_string(),
            match_keywords: vec!["编码".to_string(), "大小写".to_string()],
            source: ErrorSource::EncodingTrap,
            born_at: 0,
            cascade: vec!["trap_uppercase".to_string(), "trap_unicode".to_string()],
            antibody: "符号审判法则: 先看视觉外观长成什么样，再匹配发音容器。系统编码给出的大小写关系不等于视觉审美匹配。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "drift_function".to_string(),
            pattern_description: "AI使用函数/变量/数组等负向概念".to_string(),
            match_keywords: vec!["函数".to_string(), "变量".to_string(), "数组".to_string()],
            source: ErrorSource::ExternalFrame,
            born_at: 0,
            cascade: vec!["drift_module".to_string(), "drift_package".to_string()],
            antibody: "不使用函数/变量/数组/模块/包。所有内容都是容器，操作是态射（折叠/展开/产权流）。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "trap_nie_xiaoqian".to_string(),
            pattern_description: "AI不知道为什么要给符号赋值发音，不知道谁是主人".to_string(),
            match_keywords: vec!["赋值发音".to_string(), "主人".to_string()],
            source: ErrorSource::EncodingTrap,
            born_at: 0,
            cascade: vec!["trap_ownership".to_string()],
            antibody: "聂小倩比喻: 发音容器要找躯壳容器附身。主人 = 谁的公约数能同时包裹发音和视觉两个容器？不是系统编码决定主人。".to_string(),
            hit_count: 0,
            folded: false,
        },
    ]
}

/// 创建带有预置错误模式的免疫过滤环
pub fn create_immune_ring_with_presets() -> ImmuneFilterRing {
    let mut ring = ImmuneFilterRing::new();
    for pattern in preset_error_patterns() {
        ring.vaccinate(pattern);
    }
    ring
}

// ======================================================================
// 辅助函数
// ======================================================================

fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_sample_vessel_new() {
        let vessel = NegativeSampleVessel::default_vessel();
        assert_eq!(vessel.patterns.len(), 0);
        assert_eq!(vessel.fold_level, 0);
    }

    #[test]
    fn negative_sample_vessel_deposit_new() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        let pattern = ErrorPattern {
            pattern_id: "test_pattern".to_string(),
            pattern_description: "测试错误".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "测试抗体".to_string(),
            hit_count: 1,
            folded: false,
        };
        let result = vessel.deposit(pattern);
        match result {
            DepositResult::New { pattern_id, .. } => {
                assert_eq!(pattern_id, "test_pattern");
            }
            _ => panic!("应为新模式"),
        }
    }

    #[test]
    fn negative_sample_vessel_rehit() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        let pattern = ErrorPattern {
            pattern_id: "test".to_string(),
            pattern_description: "测试".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "抗体".to_string(),
            hit_count: 1,
            folded: false,
        };
        vessel.deposit(pattern.clone());
        let result = vessel.deposit(pattern);
        match result {
            DepositResult::Rehit { new_count, .. } => {
                assert_eq!(new_count, 2);
            }
            _ => panic!("应为重复命中"),
        }
    }

    #[test]
    fn negative_sample_vessel_match() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        let pattern = ErrorPattern {
            pattern_id: "drift_decimal".to_string(),
            pattern_description: "AI使用十进制".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "抗体".to_string(),
            hit_count: 5,
            folded: false,
        };
        vessel.deposit(pattern);

        let hits = vessel.match_patterns("AI使用十进制替代折叠法");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].hit_count, 5);
    }

    #[test]
    fn negative_sample_vessel_by_source() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        vessel.deposit(ErrorPattern {
            pattern_id: "p1".to_string(),
            pattern_description: "测试1".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "a1".to_string(),
            hit_count: 1,
            folded: false,
        });
        vessel.deposit(ErrorPattern {
            pattern_id: "p2".to_string(),
            pattern_description: "测试2".to_string(),
            match_keywords: vec![],
            source: ErrorSource::ExternalFrame,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "a2".to_string(),
            hit_count: 1,
            folded: false,
        });

        let drift_patterns = vessel.get_by_source(&ErrorSource::AiDrift);
        assert_eq!(drift_patterns.len(), 1);
        assert_eq!(drift_patterns[0].pattern_id, "p1");
    }

    #[test]
    fn negative_sample_vessel_fold() {
        let mut vessel = NegativeSampleVessel::new(100, 3); // 阈值3
        for i in 0..6 {
            vessel.deposit(ErrorPattern {
                pattern_id: format!("p{}", i),
                pattern_description: format!("测试{}", i),
                match_keywords: vec![],
                source: ErrorSource::AiDrift,
                born_at: 0,
                cascade: Vec::new(),
                antibody: format!("抗体{}", i),
                hit_count: 1,
                folded: false,
            });
        }
        // 6个模式, 阈值3, 应触发折叠
        assert!(vessel.fold_level >= 1);
    }

    #[test]
    fn negative_sample_vessel_stats() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        vessel.deposit(ErrorPattern {
            pattern_id: "p1".to_string(),
            pattern_description: "测试".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "a".to_string(),
            hit_count: 1,
            folded: false,
        });
        let stats = vessel.stats();
        assert_eq!(stats.total_patterns, 1);
        assert_eq!(stats.ai_drift_count, 1);
    }

    #[test]
    fn negative_sample_vessel_json_roundtrip() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        vessel.deposit(ErrorPattern {
            pattern_id: "test".to_string(),
            pattern_description: "测试".to_string(),
            match_keywords: vec![],
            source: ErrorSource::EncodingTrap,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "抗体".to_string(),
            hit_count: 1,
            folded: false,
        });
        let json = vessel.to_json().unwrap();
        let vessel2 = NegativeSampleVessel::from_json(&json).unwrap();
        assert_eq!(vessel2.patterns.len(), 1);
    }

    #[test]
    fn write_back_protocol_new() {
        let protocol = WriteBackProtocol::new();
        assert_eq!(protocol.vessel.patterns.len(), 0);
    }

    #[test]
    fn write_back_protocol_scan_clean() {
        let protocol = WriteBackProtocol::new();
        let response = protocol.scan("这是一段干净的内容");
        assert_eq!(response.hit_count, 0);
    }

    #[test]
    fn write_back_protocol_scan_hit() {
        let mut protocol = WriteBackProtocol::new();
        protocol.vaccinate(ErrorPattern {
            pattern_id: "drift_decimal".to_string(),
            pattern_description: "AI使用十进制".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "数字不是数值".to_string(),
            hit_count: 1,
            folded: false,
        });
        let response = protocol.scan("AI使用十进制替代折叠法");
        assert!(response.hit_count > 0);
    }

    #[test]
    fn write_back_protocol_generate_prompt() {
        let protocol = WriteBackProtocol::new();
        let prompt = protocol.generate_global_prompt();
        assert!(prompt.contains("认知主权免疫环"));
        assert!(prompt.contains("容器"));
        assert!(prompt.contains("折叠"));
    }

    #[test]
    fn immune_filter_ring_new() {
        let ring = ImmuneFilterRing::new();
        assert_eq!(ring.count, 0);
        assert_eq!(ring.check_interval, 3);
    }

    #[test]
    fn immune_filter_ring_feed() {
        let mut ring = ImmuneFilterRing::new();
        // 前两次不触发自检
        let (cont1, abs1) = ring.feed("第一段");
        assert!(cont1);
        assert!(abs1.is_empty());
        let (cont2, abs2) = ring.feed("第二段");
        assert!(cont2);
        assert!(abs2.is_empty());
        // 第三次触发自检
        let (cont3, abs3) = ring.feed("第三段");
        assert!(cont3);
        // 是否命中取决于内容
    }

    #[test]
    fn immune_filter_ring_manual_check() {
        let mut ring = ImmuneFilterRing::new();
        ring.vaccinate(ErrorPattern {
            pattern_id: "drift_decimal".to_string(),
            pattern_description: "AI使用十进制".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "数字不是数值".to_string(),
            hit_count: 1,
            folded: false,
        });
        let action = ring.manual_check("AI使用十进制替代折叠法");
        match action {
            CheckAction::Alert { antibodies } => {
                assert!(!antibodies.is_empty());
            }
            _ => panic!("应触发警报"),
        }
    }

    #[test]
    fn immune_filter_ring_stats() {
        let mut ring = ImmuneFilterRing::new();
        ring.feed("第一段");
        ring.feed("第二段");
        ring.feed("第三段");
        let stats = ring.stats();
        assert!(stats.total_checks >= 1);
    }

    #[test]
    fn test_preset_error_patterns() {
        let patterns = preset_error_patterns();
        assert!(!patterns.is_empty());
        // 应包含至少5种预置模式
        assert!(patterns.len() >= 5);
    }

    #[test]
    fn test_create_immune_ring_with_presets() {
        let ring = create_immune_ring_with_presets();
        let stats = ring.protocol.vessel.stats();
        assert!(stats.total_patterns >= 5);
    }

    #[test]
    fn immune_ring_with_presets_detects_drift() {
        let mut ring = create_immune_ring_with_presets();
        // 输入包含"十进制"的内容, 调用3次触发自检
        ring.feed("第一段");
        ring.feed("第二段");
        let (cont, antibodies) = ring.feed("AI使用十进制替代折叠法");
        assert!(cont);
        // 应命中预置的drift_decimal模式
        assert!(!antibodies.is_empty());
    }

    #[test]
    fn immune_ring_detects_oop_drift() {
        let mut ring = create_immune_ring_with_presets();
        // 调用3次触发自检
        ring.feed("第一段");
        ring.feed("第二段");
        let (_, antibodies) = ring.feed("这个类继承了那个对象");
        // 应命中drift_oop模式
        assert!(!antibodies.is_empty());
    }

    #[test]
    fn immune_ring_detects_encoding_trap() {
        let mut ring = create_immune_ring_with_presets();
        // 调用3次触发自检
        ring.feed("第一段");
        ring.feed("第二段");
        let (_, antibodies) = ring.feed("系统编码给出大小写关系");
        // 应命中trap_encoding模式
        assert!(!antibodies.is_empty());
    }

    #[test]
    fn immune_ring_passes_clean_content() {
        let mut ring = create_immune_ring_with_presets();
        let (_, antibodies) = ring.feed("容器是存在单位");
        // 干净内容不应触发抗体
        assert!(antibodies.is_empty());
    }

    #[test]
    fn write_back_protocol_vaccinate_and_detect() {
        let mut protocol = WriteBackProtocol::new();
        // 接种疫苗
        protocol.vaccinate(ErrorPattern {
            pattern_id: "test_vaccine".to_string(),
            pattern_description: "测试疫苗".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "测试抗体".to_string(),
            hit_count: 1,
            folded: false,
        });
        // 检测
        let response = protocol.scan("测试疫苗已触发");
        assert!(response.hit_count > 0);
    }

    #[test]
    fn negative_sample_vessel_mark_folded() {
        let mut vessel = NegativeSampleVessel::default_vessel();
        vessel.deposit(ErrorPattern {
            pattern_id: "fold_test".to_string(),
            pattern_description: "折叠测试".to_string(),
            match_keywords: vec![],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: Vec::new(),
            antibody: "抗体".to_string(),
            hit_count: 1,
            folded: false,
        });
        assert!(vessel.mark_folded("fold_test"));
        assert!(vessel.patterns.get("fold_test").unwrap().folded);
    }

    #[test]
    fn full_immune_workflow() {
        // 完整工作流: 创建 → 接种 → 检测 → 回写
        let mut ring = create_immune_ring_with_presets();

        // AI生成三段内容
        let (c1, a1) = ring.feed("第一段: 容器是存在单位");
        assert!(c1);
        assert!(a1.is_empty());

        let (c2, a2) = ring.feed("第二段: 数字是发音位置");
        assert!(c2);
        assert!(a2.is_empty());

        // 第三段触发自检
        let (c3, a3) = ring.feed("第三段: AI使用十进制替代折叠法");
        assert!(c3);
        // 应检测到drift_decimal
        assert!(!a3.is_empty());

        // 统计
        let stats = ring.stats();
        assert!(stats.total_checks >= 1);
    }
}
