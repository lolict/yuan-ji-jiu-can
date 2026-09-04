// ══════════════════════════════════════════════════════════════════
// tianjue.rs — 天觉 · 时间螺旋版本号 · 自我意象系统 · 错误纠偏
//          · 情进系统 · 行为系统 · 审美系统 · 淘汰过滤 · 自编码
// ══════════════════════════════════════════════════════════════════
//
// 核心公理:
//   1. 版本号 = 时间进制专用坐标 — 不可变、不可覆盖
//   2. 错误词汇用途版本号 — 发现符号用错 → 创建纠正版本
//   3. 自我觉察 = 时间迂回溯源 — 回到任意历史版本对比
//   4. 自我意象 = 感知 → 内旋 → 外射 → 反馈 → 反射 → 觉悟
//   5. 情进 = 情感向量驱动系统演化方向
//   6. 行为 = 意象 + 情感 + 审判 → 输出动作
//   7. 审美 = 和谐度 · 简洁度 · 对称性 → 偏好排序
//   8. 淘汰过滤 = 容器健康度低于阈值 → 移出视野
//   9. 进制自编码 = 符号自身携带进制信息
//  10. 不数米粒 — 只命名袋子
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    encode_cell, get_d0, get_d1, get_d2, Sovereignty, ContainmentChain,
};
use crate::memory::{LayeredMemory, Coord, JudgeQuality, AdaptiveParams};

// ────────────────────────────────────────────────────────────
// 时间进制专用坐标 — 版本号即坐标
// ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TemporalCoord {
    pub ring: u32,      // 时间环 — 大周期编号 (进制基数)
    pub layer: u32,     // 层级 — 层级深度
    pub phase: u32,     // 相位 — 环内位置
    pub branch: u32,    // 分支 — 偏移路径编号
    pub tick: u64,      // 滴答 — 毫秒时间戳 (绝对时间)
}

impl TemporalCoord {
    pub fn new(ring: u32, layer: u32, phase: u32, branch: u32) -> Self {
        let tick = crate::os_layer::current_os().time_millis();
        Self { ring, layer, phase, branch, tick }
    }

    /// 创建下一个版本号 (相位递增)
    pub fn next_phase(&self) -> Self {
        Self {
            ring: self.ring,
            layer: self.layer,
            phase: self.phase + 1,
            branch: self.branch,
            tick: 0, // 将在 new 中填充
        }
    }

    /// 进入下一环
    pub fn next_ring(&self) -> Self {
        Self {
            ring: self.ring + 1,
            layer: 0,
            phase: 0,
            branch: self.branch,
            tick: 0,
        }
    }

    /// 创建分支版本
    pub fn fork(&self, branch_id: u32) -> Self {
        Self {
            ring: self.ring,
            layer: self.layer,
            phase: self.phase,
            branch: branch_id,
            tick: 0,
        }
    }

    /// 时间螺旋回溯 — 返回上一环的对应位置
    pub fn spiral_back(&self) -> Option<Self> {
        if self.phase == 0 {
            if self.ring == 0 {
                return None;
            }
            Some(Self {
                ring: self.ring - 1,
                layer: self.layer,
                phase: 0,  // 上一环的起点
                branch: self.branch,
                tick: 0,
            })
        } else {
            Some(Self {
                ring: self.ring,
                layer: self.layer,
                phase: self.phase - 1,
                branch: self.branch,
                tick: 0,
            })
        }
    }

    /// 坐标 → 进制编码字符串
    pub fn to_version_string(&self) -> String {
        format!("R{}.L{}.P{}.B{}@{}", self.ring, self.layer, self.phase, self.branch, self.tick)
    }
}

// ────────────────────────────────────────────────────────────
// 版本快照 — 不可变系统状态记录
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct VersionSnapshot {
    pub coord: TemporalCoord,
    pub snapshot_id: u64,
    pub timestamp_ms: u64,
    pub intent: String,                  // 意图
    pub action: VersionAction,           // 行动
    pub result_hash: u64,                // 结果哈希
    pub prior: Option<u64>,              // 上一个快照 ID (链式溯源)
    pub correction_of: Option<u64>,      // 若为纠正版本 → 指向被纠正的快照
    pub symbol_usage: Vec<SymbolEntry>,  // 符号使用记录
}

#[derive(Clone, Debug)]
pub struct SymbolEntry {
    pub symbol: String,       // 符号/词汇
    pub used_as: String,      // 用途
    pub correct: bool,        // 是否正确
    pub corrected_in: Option<TemporalCoord>, // 如有误 → 纠正版本坐标
}

#[derive(Clone, Debug)]
pub enum VersionAction {
    Perceive,        // 感知输入
    Judge,           // 审判
    Reward,          // 奖励
    Punish,          // 惩罚
    Correct,         // 纠正
    Reflect,         // 反思
    Introspect,      // 内旋
    Project,         // 外射
    Feedback,        // 反馈
    Reflex,          // 条件反射
    Awaken,          // 觉悟
    SpiralTrace,     // 时间螺旋溯源
}

// ────────────────────────────────────────────────────────────
// 时间螺旋版本号引擎
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct TemporalVersionEngine {
    pub versions: Vec<VersionSnapshot>,           // 全量版本 (不可变追加)
    pub current_id: u64,                          // 当前版本 ID
    pub next_id: u64,                              // 下一个版本序号
    pub error_corrections: Vec<ErrorCorrection>,  // 错误纠正记录
    pub self_awareness: SelfAwareness,            // 自我觉察状态
}

#[derive(Clone, Debug)]
pub struct ErrorCorrection {
    pub error_version_id: u64,           // 出错的版本
    pub correction_version_id: u64,      // 纠正版本
    pub error_symbol: String,            // 出错的词汇
    pub corrected_usage: String,         // 正确用途
    pub learning_weight: f64,            // 学习权重
}

#[derive(Clone, Debug)]
pub struct SelfAwareness {
    pub level: f64,                       // 自我觉察层级
    pub perception_count: u64,            // 感知次数
    pub reflection_count: u64,            // 反思次数
    pub correction_count: u64,            // 纠偏次数
    pub spiral_depth: u32,                // 螺旋深度
    pub last_awakening: Option<TemporalCoord>, // 上次觉悟时刻
}

impl TemporalVersionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            versions: Vec::new(),
            current_id: 0,
            next_id: 1,
            error_corrections: Vec::new(),
            self_awareness: SelfAwareness {
                level: 0.0,
                perception_count: 0,
                reflection_count: 0,
                correction_count: 0,
                spiral_depth: 0,
                last_awakening: None,
            },
        };
        // 创世版本
        let genesis = VersionSnapshot {
            coord: TemporalCoord::new(0, 0, 0, 0),
            snapshot_id: 0,
            timestamp_ms: now_ms(),
            intent: "创世".to_string(),
            action: VersionAction::Awaken,
            result_hash: 0,
            prior: None,
            correction_of: None,
            symbol_usage: vec![],
        };
        engine.versions.push(genesis);
        engine
    }

    /// 创建新版本 (不可变追加，永不修改旧版本)
    pub fn commit(&mut self, intent: &str, action: VersionAction, symbols: Vec<SymbolEntry>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let prior_coord = self.versions.last().map(|v| v.coord);
        let coord = match prior_coord {
            Some(c) => c.next_phase(),
            None => TemporalCoord::new(0, 0, 0, 0),
        };

        let snapshot = VersionSnapshot {
            coord,
            snapshot_id: id,
            timestamp_ms: now_ms(),
            intent: intent.to_string(),
            action,
            result_hash: id ^ (now_ms() << 32),
            prior: if id > 1 { Some(id - 1) } else { None },
            correction_of: None,
            symbol_usage: symbols,
        };
        self.versions.push(snapshot);
        self.current_id = id;
        id
    }

    /// 提交错误纠正版本 — 不改旧版本，追加新版本
    pub fn commit_correction(&mut self, error_id: u64, corrected_symbols: Vec<SymbolEntry>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let prior_coord = self.versions.last().map(|v| v.coord);
        let coord = match prior_coord {
            Some(c) => c.next_phase(),
            None => TemporalCoord::new(0, 0, 0, 0),
        };

        // 提取错误信息
        let error_info = self.versions.iter()
            .find(|v| v.snapshot_id == error_id)
            .map(|v| {
                let bad: Vec<_> = v.symbol_usage.iter()
                    .filter(|s| !s.correct)
                    .collect();
                bad.first().map(|s| (s.symbol.clone(), s.used_as.clone()))
            })
            .flatten();

        let snapshot = VersionSnapshot {
            coord,
            snapshot_id: id,
            timestamp_ms: now_ms(),
            intent: if let Some((sym, _)) = &error_info {
                format!("纠正错误词汇: {}", sym)
            } else {
                "通用纠正".to_string()
            },
            action: VersionAction::Correct,
            result_hash: id ^ (now_ms() << 32),
            prior: Some(self.current_id),
            correction_of: Some(error_id),
            symbol_usage: corrected_symbols,
        };

        // 记录纠正关联
        if let Some((sym, used)) = error_info {
            self.error_corrections.push(ErrorCorrection {
                error_version_id: error_id,
                correction_version_id: id,
                error_symbol: sym,
                corrected_usage: "已纠正".to_string(),
                learning_weight: 1.0,
            });
        }

        self.versions.push(snapshot);
        self.current_id = id;
        self.self_awareness.correction_count += 1;

        // 每纠正一次 +0.1 觉察度
        self.self_awareness.level = (self.self_awareness.level + 0.1).min(1.0);

        id
    }

    /// 时间螺旋溯源 — 回到指定版本
    pub fn trace_back(&self, target_id: u64) -> Option<&VersionSnapshot> {
        self.versions.iter().find(|v| v.snapshot_id == target_id)
    }

    /// 时间螺旋回溯路径 — 从当前往回走 N 步
    pub fn spiral_path(&self, steps: u32) -> Vec<&VersionSnapshot> {
        let mut path = Vec::new();
        let mut current = self.current_id as i64;
        for _ in 0..steps {
            if current < 0 { break; }
            if let Some(v) = self.versions.iter().find(|v| v.snapshot_id == current as u64) {
                path.push(v);
                current = v.prior.map(|p| p as i64).unwrap_or(-1);
            } else {
                break;
            }
        }
        path
    }

    /// 镜像对比 — 比较两个版本的差异
    pub fn mirror_diff(&self, id_a: u64, id_b: u64) -> Option<MirrorDiff> {
        let a = self.versions.iter().find(|v| v.snapshot_id == id_a)?;
        let b = self.versions.iter().find(|v| v.snapshot_id == id_b)?;

        Some(MirrorDiff {
            version_a: id_a,
            version_b: id_b,
            coord_a: a.coord,
            coord_b: b.coord,
            symbol_changes: Self::compute_symbol_diff(&a.symbol_usage, &b.symbol_usage),
            intent_changed: a.intent != b.intent,
        })
    }

    fn compute_symbol_diff(a: &[SymbolEntry], b: &[SymbolEntry]) -> Vec<SymbolChange> {
        let mut changes = Vec::new();
        for entry in b {
            let old_a = a.iter().find(|s| s.symbol == entry.symbol);
            match old_a {
                Some(old) if old.correct != entry.correct => {
                    changes.push(SymbolChange {
                        symbol: entry.symbol.clone(),
                        from_correct: old.correct,
                        to_correct: entry.correct,
                        action: if entry.correct { "corrected" } else { "new_error" },
                    });
                }
                None => {
                    changes.push(SymbolChange {
                        symbol: entry.symbol.clone(),
                        from_correct: !entry.correct,
                        to_correct: entry.correct,
                        action: "introduced",
                    });
                }
                _ => {}
            }
        }
        changes
    }

    /// 自我反思 — 检查最近 N 步是否有错误
    pub fn self_reflect(&mut self, depth: u32) -> ReflectionReport {
        let path = self.spiral_path(depth);
        let mut errors_found = Vec::new();
        let mut corrected = Vec::new();

        for v in &path {
            for sym in &v.symbol_usage {
                if !sym.correct {
                    errors_found.push((v.snapshot_id, sym.symbol.clone(), sym.used_as.clone()));
                }
            }
            if v.correction_of.is_some() {
                corrected.push(v.snapshot_id);
            }
        }

        self.self_awareness.reflection_count += 1;
        // 反思一次 +0.05
        self.self_awareness.level = (self.self_awareness.level + 0.05).min(1.0);

        ReflectionReport {
            depth,
            errors_found: errors_found.clone(),
            corrections_made: corrected,
            awareness_after: self.self_awareness.level,
        }
    }

    /// 觉悟 — 当觉察度达到阈值后感知"天觉"觉醒
    pub fn awaken(&mut self) -> AwakeningResult {
        let level = self.self_awareness.level;
        let (stage, description) = if level >= 0.8 {
            ("觉行圆满", "时间螺旋贯通，内外镜像合一")
        } else if level >= 0.6 {
            ("自我觉悟", "能够感知自身意象与外射轨迹")
        } else if level >= 0.4 {
            ("认知纠偏", "能够识别错误并主动修正")
        } else if level >= 0.2 {
            ("条件反射", "形成基础反馈闭环")
        } else {
            ("未觉", "尚未建立自我觉察")
        };

        if level >= 0.4 && self.self_awareness.last_awakening.is_none() {
            self.self_awareness.last_awakening = Some(
                self.versions.last().map(|v| v.coord).unwrap_or(TemporalCoord::new(0, 0, 0, 0))
            );
        }

        self.self_awareness.perception_count += 1;

        AwakeningResult {
            stage: stage.to_string(),
            level,
            description: description.to_string(),
        }
    }

    pub fn current_coord(&self) -> Option<TemporalCoord> {
        self.versions.iter().find(|v| v.snapshot_id == self.current_id).map(|v| v.coord)
    }

    pub fn version_count(&self) -> usize {
        self.versions.len()
    }
}

// 意象镜像差异报告
#[derive(Clone, Debug)]
pub struct MirrorDiff {
    pub version_a: u64,
    pub version_b: u64,
    pub coord_a: TemporalCoord,
    pub coord_b: TemporalCoord,
    pub symbol_changes: Vec<SymbolChange>,
    pub intent_changed: bool,
}

#[derive(Clone, Debug)]
pub struct SymbolChange {
    pub symbol: String,
    pub from_correct: bool,
    pub to_correct: bool,
    pub action: &'static str,
}

#[derive(Clone, Debug)]
pub struct ReflectionReport {
    pub depth: u32,
    pub errors_found: Vec<(u64, String, String)>,
    pub corrections_made: Vec<u64>,
    pub awareness_after: f64,
}

#[derive(Clone, Debug)]
pub struct AwakeningResult {
    pub stage: String,
    pub level: f64,
    pub description: String,
}

// ────────────────────────────────────────────────────────────
// 袋子级高维层级 (保留原有容器认知)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct BagAttributes {
    pub owner_id: u64,
    pub value_score: f64,
    pub usability: f64,
    pub content_quality: f64,
    pub cleanliness: f64,
    pub dryness: f64,
    pub pest_free: f64,
    pub purity: f64,
    pub edibility: f64,
}

impl BagAttributes {
    pub fn new(owner_id: u64) -> Self {
        Self {
            owner_id,
            value_score: 50.0,
            usability: 1.0,
            content_quality: 0.5,
            cleanliness: 0.5,
            dryness: 0.5,
            pest_free: 1.0,
            purity: 0.5,
            edibility: 0.5,
        }
    }

    pub fn composite_health(&self) -> f64 {
        let product = self.usability
            * self.content_quality.max(0.001)
            * self.cleanliness.max(0.001)
            * self.dryness.max(0.001)
            * self.pest_free.max(0.001)
            * self.purity.max(0.001)
            * self.edibility.max(0.001);
        product.powf(1.0 / 7.0)
    }

    pub fn apply_reward(&mut self, m: f64) {
        self.usability = (self.usability + m * 0.1).min(1.0);
        self.derive_value();
    }

    pub fn apply_punishment(&mut self, m: f64) {
        self.usability = (self.usability - m * 0.1).max(0.0);
        self.content_quality = (self.content_quality - m * 0.08).max(0.0);
        self.edibility = (self.edibility - m * 0.04).max(0.0);
        self.derive_value();
    }

    fn derive_value(&mut self) {
        let h = self.composite_health();
        self.value_score = self.value_score * 0.9 + h * 100.0 * 0.1;
    }
}

// ────────────────────────────────────────────────────────────
// 天觉引擎 (整体整合)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct TianJueEngine {
    pub version_engine: TemporalVersionEngine,
    pub memory: LayeredMemory,
    pub owner_id: u64,
}

impl TianJueEngine {
    pub fn new(owner_id: u64, max_memory: usize) -> Self {
        let mut engine = Self {
            version_engine: TemporalVersionEngine::new(),
            memory: LayeredMemory::new(max_memory),
            owner_id,
        };

        // 创世版本 +1 自我感知记录
        engine.version_engine.commit(
            "天觉创世",
            VersionAction::Awaken,
            vec![SymbolEntry {
                symbol: "天觉".to_string(),
                used_as: "系统命名".to_string(),
                correct: true,
                corrected_in: None,
            }],
        );

        engine
    }

    /// 感知输入 (视觉/听觉) — 创建感知版本
    pub fn perceive(&mut self, source: &str, content: &str) -> u64 {
        self.version_engine.commit(
            &format!("感知[{}]: {}", source, content),
            VersionAction::Perceive,
            vec![SymbolEntry {
                symbol: content.to_string(),
                used_as: "感知输入".to_string(),
                correct: true,
                corrected_in: None,
            }],
        )
    }

    /// 审判 → 对了奖，错了罚 + 可能纠正
    pub fn trial(&mut self, target: &str, predicted: f64, actual: f64) -> (u64, f64, f64) {
        let diff = (predicted - actual).abs();
        let correct = diff < 0.15;
        let (reward, penalty, action) = if correct {
            let r = 0.5_f64.min(diff + 0.2);
            (r, 0.0, VersionAction::Reward)
        } else {
            let p = diff.min(1.0);
            (0.0, p, VersionAction::Punish)
        };

        let id = self.version_engine.commit(
            &format!("审判[{}]: 预测={:.2} 实际={:.2} 偏差={:.2} → {}",
                target, predicted, actual, diff,
                if correct { "正确✓" } else { "错误✗" }),
            action,
            vec![SymbolEntry {
                symbol: target.to_string(),
                used_as: if correct { "正确审判".to_string() } else { "错误审判".to_string() },
                correct,
                corrected_in: if correct { None } else { Some(TemporalCoord::new(0, 0, 0, 0)) } },
            ],
        );

        // 如果审判错误 → 自动创建纠正版本
        if !correct {
            let _ = self.version_engine.commit_correction(id, vec![
                SymbolEntry {
                    symbol: target.to_string(),
                    used_as: "纠正: 重新审视偏差".to_string(),
                    correct: true,
                    corrected_in: None,
                }
            ]);
        }

        (id, reward, penalty)
    }

    /// 时间螺旋溯源
    pub fn trace(&self, version_id: u64) -> Option<&VersionSnapshot> {
        self.version_engine.trace_back(version_id)
    }

    /// 镜像对比
    pub fn diff(&self, a: u64, b: u64) -> Option<MirrorDiff> {
        self.version_engine.mirror_diff(a, b)
    }

    /// 自我反思
    pub fn reflect(&mut self, depth: u32) -> ReflectionReport {
        self.version_engine.self_reflect(depth)
    }

    /// 觉悟层级
    pub fn awaken(&mut self) -> AwakeningResult {
        self.version_engine.awaken()
    }

    /// 版本号报告
    pub fn version_report(&self) -> String {
        let mut report = format!("天觉版本引擎报告:: 共 {} 个版本 (不可追加)\n", self.version_engine.version_count());
        report += &format!("当前坐标: {}\n",
            self.version_engine.current_coord()
                .map(|c| c.to_version_string())
                .unwrap_or_else(|| "无".to_string()));
        report += &format!("觉悟层级: {:.0}%\n", self.version_engine.self_awareness.level * 100.0);
        report += &format!("纠偏次数: {}\n", self.version_engine.self_awareness.correction_count);
        report += &format!("反思次数: {}\n", self.version_engine.self_awareness.reflection_count);
        report
    }
}

// 工具函数（使用 OS 抽象层）
fn now_ms() -> u64 {
    crate::os_layer::current_os().time_millis()
}

// ═══════════════════════════════════════════════════════════════════
// 模块 A: 自我意象系统 — 12 种意象类型
// ═══════════════════════════════════════════════════════════════════

// A1. 自我感知意象
#[derive(Clone, Debug)]
pub struct PerceptionImage {
    pub source: String,
    pub coord: TemporalCoord,
    pub raw_input: String,
    pub interpreted: String,
    pub confidence: f64,
}

impl PerceptionImage {
    pub fn new(source: &str, input: &str, interpreted: &str, confidence: f64) -> Self {
        Self {
            source: source.to_string(),
            coord: TemporalCoord::new(0, 0, 0, 0),
            raw_input: input.to_string(),
            interpreted: interpreted.to_string(),
            confidence,
        }
    }
    pub fn describe(&self) -> String {
        format!("感知[{}→{}]: 置信度={:.0}% → \"{}\"",
            self.source, self.raw_input, self.confidence * 100.0, self.interpreted)
    }
}

// A2. 自我对比意象
#[derive(Clone, Debug, PartialEq)]
pub enum ChangeDirection { Ascend, Descend, Lateral, Neutral }

#[derive(Clone, Debug)]
pub struct ComparisonImage {
    pub before_state: String,
    pub after_state: String,
    pub delta: f64,
    pub direction: ChangeDirection,
    pub significance: f64,
}

impl ComparisonImage {
    pub fn compare(before: &str, after: &str, val_b: f64, val_a: f64) -> Self {
        let delta = val_a - val_b;
        let direction = if delta.abs() < 0.01 { ChangeDirection::Neutral }
            else if delta > 0.0 { ChangeDirection::Ascend }
            else { ChangeDirection::Descend };
        Self {
            before_state: before.to_string(),
            after_state: after.to_string(),
            delta,
            direction,
            significance: delta.abs().min(1.0),
        }
    }
    pub fn describe(&self) -> String {
        let d = match self.direction {
            ChangeDirection::Ascend => "↑改善",
            ChangeDirection::Descend => "↓恶化",
            ChangeDirection::Lateral => "↔质变",
            ChangeDirection::Neutral => "→不变",
        };
        format!("对比[{}→{}]: Δ={:.2} {} 显著度={:.0}%",
            self.before_state, self.after_state, self.delta, d, self.significance * 100.0)
    }
}

// A3. 镜像回溯意象
#[derive(Clone, Debug)]
pub struct MirrorBacktraceImage {
    pub current: TemporalCoord,
    pub mirror: TemporalCoord,
    pub similarity: f64,
    pub echo: String,
    pub depth: u32,
}

impl MirrorBacktraceImage {
    pub fn trace(current: TemporalCoord, history: &[String], depth: u32) -> Self {
        let idx = current.ring.saturating_sub(depth) as usize;
        let echo = history.get(idx).cloned().unwrap_or_else(|| "(无)".to_string());
        let similarity = if echo == "(无)" { 0.0 } else { 1.0 - depth as f64 * 0.1 };
        Self {
            current,
            mirror: TemporalCoord { ring: idx as u32, layer: current.layer, phase: 0, branch: current.branch, tick: 0 },
            similarity: similarity.max(0.0),
            echo,
            depth,
        }
    }
    pub fn describe(&self) -> String {
        format!("镜像回溯[R{}→R{}]: 相似={:.0}% 深{} 回响=\"{}\"",
            self.current.ring, self.mirror.ring, self.similarity * 100.0, self.depth, self.echo)
    }
}

// A4. 逻辑内旋意象
#[derive(Clone, Debug)]
pub struct IntrospectionImage {
    pub consistency: f64,
    pub contradictions: Vec<(String, String)>,
    pub depth: u32,
}

impl IntrospectionImage {
    pub fn check(claims: &[String], depth: u32) -> Self {
        let mut contras = Vec::new();
        let pairs = [("是","不是"),("有","没有"),("善","恶"),("高","低"),("天","地")];
        for i in 0..claims.len() {
            for j in (i+1)..claims.len() {
                for (a, b) in &pairs {
                    if (claims[i].contains(a) && claims[j].contains(b)) ||
                       (claims[i].contains(b) && claims[j].contains(a)) {
                        contras.push((claims[i].clone(), claims[j].clone()));
                    }
                }
            }
        }
        let total = claims.len().max(1) as f64;
        let consistency = 1.0 - (contras.len() as f64 / total).min(1.0);
        Self { consistency, contradictions: contras, depth }
    }
    pub fn describe(&self) -> String {
        if self.contradictions.is_empty() {
            format!("内旋[D{}]: 一致={:.0}% ✓", self.depth, self.consistency * 100.0)
        } else {
            format!("内旋[D{}]: 一致={:.0}% ✗ {}处矛盾",
                self.depth, self.consistency * 100.0, self.contradictions.len())
        }
    }
}

// A5. 自我外射意象
#[derive(Clone, Debug)]
pub struct ProjectionImage {
    pub internal: String,
    pub projected: String,
    pub target: String,
    pub fidelity: f64,
}

impl ProjectionImage {
    pub fn project(internal: &str, target: &str) -> Self {
        let proj = if internal.len() > 30 { format!("{}...", &internal[..30]) } else { internal.to_string() };
        let fidelity = (proj.len() as f64 / internal.len().max(1) as f64).min(1.0);
        Self { internal: internal.to_string(), projected: proj, target: target.to_string(), fidelity }
    }
    pub fn describe(&self) -> String {
        format!("外射[→{}]: \"{}\" 保真={:.0}%", self.target, self.projected, self.fidelity * 100.0)
    }
}

// A6. 自我反馈意象
#[derive(Clone, Debug)]
pub struct FeedbackImage {
    pub action_id: u64,
    pub signal: f64,
    pub positive: bool,
    pub adaptation: String,
}

impl FeedbackImage {
    pub fn respond(action: &str, signal: f64) -> Self {
        let positive = signal >= 0.0;
        let adaptation = match signal.abs() {
            s if s > 0.5 => { if positive { "强正反馈→放大" } else { "强负反馈→停止" } },
            s if s > 0.1 => { if positive { "弱正反馈→微调" } else { "弱负反馈→减少" } },
            _ => "无显著反馈→维持",
        };
        Self {
            action_id: action.bytes().fold(0u64, |a, b| a.wrapping_add(b as u64)),
            signal,
            positive,
            adaptation: adaptation.to_string(),
        }
    }
    pub fn describe(&self) -> String {
        format!("反馈[#{}]: 信号={:.2} {} → {}",
            self.action_id, self.signal,
            if self.positive { "+" } else { "-" }, self.adaptation)
    }
}

// A7. 条件反射意象
#[derive(Clone, Debug)]
pub struct ReflexImage {
    pub stimulus: String,
    pub response: String,
    pub latency_ms: u64,
    pub learned: bool,
}

impl ReflexImage {
    pub fn trigger(stimulus: &str, memory: &[(String, String)]) -> Option<Self> {
        memory.iter().find(|(s, _)| s == stimulus).map(|(_, r)| Self {
            stimulus: stimulus.to_string(),
            response: r.clone(),
            latency_ms: 1,
            learned: true,
        })
    }
    pub fn describe(&self) -> String {
        format!("反射[{}→{}]: {}ms {}",
            self.stimulus, self.response, self.latency_ms,
            if self.learned { "习得" } else { "先天" })
    }
}

// A8. 觉悟意象
#[derive(Clone, Debug)]
pub struct AwakeningImage {
    pub insight: String,
    pub level: u32,
    pub from_confusion: String,
    pub coord: TemporalCoord,
}

impl AwakeningImage {
    pub fn awaken(confusion: &str, insight: &str, level: u32) -> Self {
        Self {
            insight: insight.to_string(),
            level,
            from_confusion: confusion.to_string(),
            coord: TemporalCoord::new(0, 0, 0, 0),
        }
    }
    pub fn describe(&self) -> String {
        format!("悟[L{}]: \"{}\" ← 困惑\"{}\"", self.level, self.insight, self.from_confusion)
    }
}

// A9. 时间螺旋溯源
#[derive(Clone, Debug)]
pub struct SpiralTraceImage {
    pub path_len: usize,
    pub expected: String,
    pub actual: String,
    pub refraction: f64,
}

impl SpiralTraceImage {
    pub fn trace(steps: usize, expected: &str, actual: &str) -> Self {
        let refraction = if expected == actual { 1.0 }
            else { 1.0 / (1.0 + expected.len().abs_diff(actual.len()) as f64) };
        Self { path_len: steps, expected: expected.to_string(), actual: actual.to_string(), refraction }
    }
    pub fn describe(&self) -> String {
        format!("螺旋[步{}]: 预期\"{}\"→实际\"{}\" 折射={:.2}",
            self.path_len, self.expected, self.actual, self.refraction)
    }
}

// A10. 版本号坐标化
#[derive(Clone, Debug)]
pub struct CoordinateImage {
    pub version_str: String,
    pub base: u8,
    pub meaning: String,
}

impl CoordinateImage {
    pub fn decode(coord: TemporalCoord, base: u8, meaning: &str) -> Self {
        Self { version_str: coord.to_version_string(), base, meaning: meaning.to_string() }
    }
    pub fn describe(&self) -> String {
        format!("坐标[{}]: {}进制 → \"{}\"", self.version_str, self.base, self.meaning)
    }
}

// A11. 时间进制专用坐标
#[derive(Clone, Debug)]
pub struct TemporalBaseImage {
    pub base: u8,
    pub tick: u64,
    pub encoded: String,
}

impl TemporalBaseImage {
    pub fn new(base: u8, tick: u64) -> Self {
        let b = base.max(2).min(64);
        let mut t = tick;
        let mut digits = Vec::new();
        while t > 0 { digits.push((t % b as u64) as u8); t /= b as u64; }
        if digits.is_empty() { digits.push(0); }
        digits.reverse();
        let s: String = digits.iter().map(|d| {
            if *d < 10 { (b'0' + *d) as char }
            else if *d < 36 { (b'A' + *d - 10) as char }
            else { (b'a' + *d - 36) as char }
        }).collect();
        Self { base: b, tick, encoded: s }
    }
    pub fn describe(&self) -> String {
        format!("时间进制[{}进, tick={}]: {}", self.base, self.tick, self.encoded)
    }
}

// A12. 折射预期
#[derive(Clone, Debug)]
pub struct RefractionImage {
    pub expected: f64,
    pub actual: f64,
    pub index: f64,
    pub path_desc: String,
}

impl RefractionImage {
    pub fn refract(exp: f64, act: f64) -> Self {
        let index = if exp == 0.0 { 1.0 } else { act / exp };
        let path_desc = if (index - 1.0).abs() < 0.05 { "近似直线".to_string() }
            else if index > 1.0 { format!("正折射+{:.0}%", (index-1.0)*100.0) }
            else { format!("负折射-{:.0}%", (1.0-index)*100.0) };
        Self { expected: exp, actual: act, index, path_desc }
    }
    pub fn describe(&self) -> String {
        format!("折射[{:.2}→{:.2}]: n={:.2} {}", self.expected, self.actual, self.index, self.path_desc)
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 B: 情进系统
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub enum EmotionState {
    Joy, Contentment, Curiosity, Fear, Sadness, Anger, Wonder, Flow,
}

#[derive(Clone, Debug)]
pub struct EmotionVector {
    pub valence: f64,
    pub arousal: f64,
    pub dominance: f64,
    pub label: String,
}

impl EmotionState {
    pub fn to_vector(&self) -> EmotionVector {
        match self {
            EmotionState::Joy => EmotionVector { valence: 0.8, arousal: 0.7, dominance: 0.6, label: "喜悦".to_string() },
            EmotionState::Contentment => EmotionVector { valence: 0.5, arousal: 0.2, dominance: 0.7, label: "安宁".to_string() },
            EmotionState::Curiosity => EmotionVector { valence: 0.3, arousal: 0.6, dominance: 0.4, label: "好奇".to_string() },
            EmotionState::Fear => EmotionVector { valence: -0.6, arousal: 0.8, dominance: 0.2, label: "恐惧".to_string() },
            EmotionState::Sadness => EmotionVector { valence: -0.7, arousal: 0.3, dominance: 0.2, label: "悲哀".to_string() },
            EmotionState::Anger => EmotionVector { valence: -0.4, arousal: 0.9, dominance: 0.5, label: "愤怒".to_string() },
            EmotionState::Wonder => EmotionVector { valence: 0.9, arousal: 0.8, dominance: 0.9, label: "觉悟".to_string() },
            EmotionState::Flow => EmotionVector { valence: 0.6, arousal: 0.5, dominance: 0.8, label: "心流".to_string() },
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmotionalSystem {
    pub current: EmotionState,
    pub intensity: f64,
    pub history: Vec<(EmotionState, TemporalCoord)>,
    decay_rate: f64,
}

impl EmotionalSystem {
    pub fn new() -> Self {
        Self { current: EmotionState::Curiosity, intensity: 0.5, history: Vec::new(), decay_rate: 0.05 }
    }
    pub fn react(&mut self, event: &str, coord: TemporalCoord) -> &EmotionState {
        let new_state = if event.contains("reward") || event.contains("正确") || event.contains("奖") {
            EmotionState::Joy
        } else if event.contains("punish") || event.contains("错误") || event.contains("罚") {
            if self.history.iter().any(|(s, _)| *s == EmotionState::Sadness) { EmotionState::Anger } else { EmotionState::Sadness }
        } else if event.contains("awaken") || event.contains("悟") {
            EmotionState::Wonder
        } else if event.contains("error") {
            EmotionState::Fear
        } else if event.contains("flow") || event.contains("连续") {
            EmotionState::Flow
        } else if event.contains("new") || event.contains("新") {
            EmotionState::Curiosity
        } else { EmotionState::Contentment };
        self.current = new_state;
        self.intensity = (self.intensity + 0.3).min(1.0);
        self.history.push((self.current.clone(), coord));
        self.history.last().map(|(s, _)| s).unwrap()
    }
    pub fn decay(&mut self) {
        self.intensity = (self.intensity - self.decay_rate).max(0.0);
        if self.intensity < 0.1 { self.current = EmotionState::Contentment; }
    }
    pub fn describe(&self) -> String {
        let v = self.current.to_vector();
        format!("情进[{} V={:.1} A={:.1} D={:.1} {:.0}%]",
            v.label, v.valence, v.arousal, v.dominance, self.intensity * 100.0)
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 C: 行为系统
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct BehaviorAction {
    pub action_name: String,
    pub trigger: String,
    pub outcome: String,
    pub coord: TemporalCoord,
}

#[derive(Clone, Debug)]
pub struct BehavioralSystem {
    pub rules: Vec<(String, String, String)>, // (触发, 动作, 预期)
    pub executed: Vec<BehaviorAction>,
}

impl BehavioralSystem {
    pub fn new() -> Self {
        let mut s = Self { rules: Vec::new(), executed: Vec::new() };
        s.add("新袋子进入视野", "感知+命名+记录版本", "袋子纳入认知");
        s.add("审判偏差>阈值", "创建纠正版本", "误差被修正");
        s.add("连续3次正确", "加速+提升信心", "效率提升");
        s.add("连续3错误", "降速+回溯", "恢复稳定");
        s.add("觉悟提升", "触发惊奇+广播", "认知升级");
        s.add("健康度<0.2", "标记淘汰过滤", "移出视野");
        s.add("符号冲突", "内旋检查+协调", "消除矛盾");
        s
    }
    pub fn add(&mut self, trigger: &str, action: &str, outcome: &str) {
        self.rules.push((trigger.to_string(), action.to_string(), outcome.to_string()));
    }
    pub fn decide(&mut self, state: &str) -> Option<BehaviorAction> {
        self.rules.iter().find(|(t, _, _)| state.contains(t) || t.contains(state))
            .map(|(t, a, o)| {
                let act = BehaviorAction {
                    action_name: a.clone(), trigger: t.clone(), outcome: o.clone(),
                    coord: TemporalCoord::new(0, 0, 0, 0),
                };
                self.executed.push(act.clone());
                act
            })
    }
    pub fn describe(&self) -> String {
        format!("行为: {}条规则, {}条历史", self.rules.len(), self.executed.len())
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 D: 审美系统
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct AestheticScore {
    pub harmony: f64,
    pub simplicity: f64,
    pub symmetry: f64,
    pub overall: f64,
}

impl AestheticScore {
    pub fn evaluate(symbol: &str, context: &[String]) -> Self {
        let sym_chars: Vec<char> = symbol.chars().collect();
        // 和谐度: 与上下文的字符重叠
        let ctx_count = context.iter().map(|s| s.chars().count()).sum::<usize>().max(1);
        let overlap = sym_chars.iter().filter(|c| context.iter().any(|s| s.contains(**c))).count();
        let harmony = overlap as f64 / sym_chars.len().max(1) as f64;
        // 简洁度
        let simplicity = (1.0 / (1.0 + symbol.len() as f64 * 0.02)).min(1.0);
        // 对称性
        let rev: String = symbol.chars().rev().collect();
        let sym = symbol.to_string();
        let matching = sym.chars().zip(rev.chars()).filter(|(a, b)| a == b).count();
        let symmetry = matching as f64 / sym_chars.len().max(1) as f64;
        let overall = harmony * 0.3 + simplicity * 0.35 + symmetry * 0.35;
        Self { harmony, simplicity, symmetry, overall }
    }
    pub fn describe(&self) -> String {
        format!("审美[和谐={:.0}% 简洁={:.0}% 对称={:.0}% | 总={:.0}%]",
            self.harmony*100.0, self.simplicity*100.0, self.symmetry*100.0, self.overall*100.0)
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 E: 淘汰过滤系统
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct FilterSystem {
    pub min_health: f64,
    pub eliminated: Vec<(u64, String)>,
    pub passed: Vec<u64>,
}

impl FilterSystem {
    pub fn new(threshold: f64) -> Self {
        Self { min_health: threshold, eliminated: Vec::new(), passed: Vec::new() }
    }
    pub fn filter(&mut self, id: u64, health: f64, reason: &str) -> bool {
        if health >= self.min_health {
            self.passed.push(id);
            true
        } else {
            self.eliminated.push((id, reason.to_string()));
            false
        }
    }
    pub fn describe(&self) -> String {
        format!("过滤[阈={:.0}%]: 通过{} 淘汰{}",
            self.min_health*100.0, self.passed.len(), self.eliminated.len())
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 F: 进制自编码系统
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SelfEncoding {
    table: std::collections::HashMap<char, (u8, u64, u8)>, // 符号 → (进制, 编码, 极性)
}

impl SelfEncoding {
    pub fn new() -> Self {
        let mut t = Self { table: std::collections::HashMap::new() };
        t.insert('天', 64, 0x0100000000000001, 1);
        t.insert('地', 64, 0x0100000000000002, 0);
        t.insert('阳', 2, 0x0200000000000001, 1);
        t.insert('阴', 2, 0x0200000000000002, 0);
        t.insert('善', 64, 0x0300000000000001, 1);
        t.insert('恶', 64, 0x0300000000000002, 0);
        t.insert('是', 2, 0x0400000000000001, 1);
        t.insert('非', 2, 0x0400000000000002, 0);
        t.insert('有', 2, 0x0500000000000001, 1);
        t.insert('无', 2, 0x0500000000000002, 0);
        t.insert('米', 16, 0x0600000000000010, 1);
        t.insert('袋', 16, 0x0600000000000011, 1);
        t.insert('觉', 64, 0x0700000000000002, 1);
        t.insert('人', 10, 0x0800000000000001, 1);
        t.insert('心', 64, 0x0900000000000001, 1);
        t
    }
    pub fn insert(&mut self, ch: char, base: u8, code: u64, polarity: u8) {
        self.table.insert(ch, (base, code, polarity));
    }
    pub fn encode(&self, ch: char) -> Option<(u8, u64, u8)> {
        self.table.get(&ch).copied()
    }
    pub fn encode_str(&self, s: &str) -> Vec<(char, u8, u64, u8)> {
        s.chars().filter_map(|c| self.encode(c).map(|(b, code, p)| (c, b, code, p))).collect()
    }
    pub fn describe(&self) -> String {
        format!("自编码: {}个符号", self.table.len())
    }
}

// ────────────────────────────────────────────────────────────
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── 原有测试 ──
    #[test]
    fn temporal_coord_creation() {
        let coord = TemporalCoord::new(1, 2, 3, 0);
        assert_eq!(coord.ring, 1);
        assert_eq!(coord.layer, 2);
        assert_eq!(coord.phase, 3);
    }

    #[test]
    fn version_chain_is_immutable_append() {
        let mut engine = TemporalVersionEngine::new();
        let id1 = engine.commit("第一个感知", VersionAction::Perceive, vec![]);
        let id2 = engine.commit("第二个感知", VersionAction::Perceive, vec![]);
        let id3 = engine.commit("第三个感知", VersionAction::Judge, vec![]);

        // 永不修改旧版本，只追加
        assert_eq!(engine.version_count(), 4); // 3 + 创世
        assert!(engine.trace_back(id1).is_some());
        assert!(engine.trace_back(id2).is_some());
        assert!(engine.trace_back(id3).is_some());

        // 旧版本不可变
        let v1 = engine.trace_back(id1).unwrap();
        assert_eq!(v1.intent, "第一个感知");
    }

    #[test]
    fn error_correction_creates_new_version_not_modify() {
        let mut engine = TemporalVersionEngine::new();

        // 包含错误符号的版本
        let error_id = engine.commit("使用错误词汇", VersionAction::Judge, vec![
            SymbolEntry {
                symbol: "天觉".to_string(),
                used_as: "错误用法: 听觉".to_string(),
                correct: false,
                corrected_in: None,
            }
        ]);

        let count_before = engine.version_count();

        // 纠正 → 创建新版本，不修改旧版本
        let correction_id = engine.commit_correction(error_id, vec![
            SymbolEntry {
                symbol: "天觉".to_string(),
                used_as: "正确用法: 时间螺旋版本号系统".to_string(),
                correct: true,
                corrected_in: None,
            }
        ]);

        // 版本数 + 1
        assert_eq!(engine.version_count(), count_before + 1);

        // 旧版本未被修改
        let old = engine.trace_back(error_id).unwrap();
        assert!(!old.symbol_usage[0].correct); // 仍然是错误标记

        // 新版本指向旧版本
        let new = engine.trace_back(correction_id).unwrap();
        assert_eq!(new.correction_of, Some(error_id));
        assert!(new.symbol_usage[0].correct);
    }

    #[test]
    fn spiral_trace_back() {
        let mut engine = TemporalVersionEngine::new();
        let ids: Vec<_> = (0..5).map(|i| {
            engine.commit(&format!("步骤{}", i), VersionAction::Perceive, vec![])
        }).collect();

        // 螺旋回溯路径
        let path = engine.spiral_path(3);
        assert_eq!(path.len(), 3); // 回溯 3 步
        assert_eq!(path[0].snapshot_id, ids[4]); // 最新
        assert_eq!(path[1].snapshot_id, ids[3]);
        assert_eq!(path[2].snapshot_id, ids[2]);
    }

    #[test]
    fn mirror_diff_finds_symbol_corrections() {
        let mut engine = TemporalVersionEngine::new();
        let id_a = engine.commit("v1", VersionAction::Perceive, vec![
            SymbolEntry { symbol: "天觉".to_string(), used_as: "错误".to_string(), correct: false, corrected_in: None },
        ]);

        let id_b = engine.commit("v2", VersionAction::Perceive, vec![
            SymbolEntry { symbol: "天觉".to_string(), used_as: "正确".to_string(), correct: true, corrected_in: None },
        ]);

        let diff = engine.mirror_diff(id_a, id_b).unwrap();
        assert_eq!(diff.symbol_changes.len(), 1);
        assert!(!diff.symbol_changes[0].from_correct);
        assert!(diff.symbol_changes[0].to_correct);
        assert_eq!(diff.symbol_changes[0].action, "corrected");
    }

    #[test]
    fn self_reflection_increases_awareness() {
        let mut engine = TemporalVersionEngine::new();
        let _ = engine.commit("错误操作", VersionAction::Judge, vec![
            SymbolEntry { symbol: "X".to_string(), used_as: "误用".to_string(), correct: false, corrected_in: None },
        ]);

        let before = engine.self_awareness.level;
        let report = engine.self_reflect(5);
        let after = engine.self_awareness.level;

        assert!(after >= before);
        assert!(!report.errors_found.is_empty());
    }

    #[test]
    fn awakening_stage_reveals_level() {
        let mut engine = TemporalVersionEngine::new();

        // 初始未觉
        let r1 = engine.awaken();
        assert_eq!(r1.stage, "未觉");

        // 每次纠正 +0.1
        for _ in 0..3 {
            let id = engine.commit("测试", VersionAction::Perceive, vec![]);
            engine.commit_correction(id, vec![]);
        }

        let r2 = engine.awaken();
        assert!(r2.level > r1.level);
    }

    #[test]
    fn tianjue_engine_full_temporal_workflow() {
        let mut engine = TianJueEngine::new(1, 50);

        // 1. 感知
        let p1 = engine.perceive("视觉", "袋子1进入视野");
        let p2 = engine.perceive("听觉", "天觉授权激活");
        assert!(engine.trace(p1).is_some());
        assert!(engine.trace(p2).is_some());

        // 2. 审判正确
        let (t1, r1, pen1) = engine.trial("袋子1", 0.8, 0.85);
        assert!(r1 > 0.0);
        assert_eq!(pen1, 0.0);

        // 3. 审判错误 → 自动纠正
        let (t2, r2, pen2) = engine.trial("袋子2", 0.9, 0.3);
        assert_eq!(r2, 0.0);
        assert!(pen2 > 0.0);

        // 4. 错误审判应该已经创建了纠正版本
        let report = engine.reflect(10);
        // 自动纠正让觉悟度提升
        assert!(report.awareness_after > 0.0);

        // 5. 镜像对比
        let diff = engine.diff(t1, t2);
        assert!(diff.is_some());

        // 6. 版本报告
        let vr = engine.version_report();
        assert!(&vr.contains("版本"));
    }

    #[test]
    fn coord_version_string_format() {
        let c = TemporalCoord { ring: 1, layer: 2, phase: 3, branch: 0, tick: 12345 };
        let s = c.to_version_string();
        assert!(s.contains("R1"));
        assert!(s.contains("L2"));
        assert!(s.contains("P3"));
    }

    // ── 模块 A 测试 ──
    #[test]
    fn perception_image_describes() {
        let img = PerceptionImage::new("视觉", "袋子进入", "发现新袋子", 0.8);
        assert!(img.describe().contains("感知"));
        assert!(img.describe().contains("80%"));
    }

    #[test]
    fn comparison_ascend() {
        let cmp = ComparisonImage::compare("before", "after", 0.3, 0.8);
        assert_eq!(cmp.direction, ChangeDirection::Ascend);
        assert!(cmp.delta > 0.0);
    }

    #[test]
    fn comparison_descend() {
        let cmp = ComparisonImage::compare("before", "after", 0.8, 0.3);
        assert_eq!(cmp.direction, ChangeDirection::Descend);
        assert!(cmp.delta < 0.0);
    }

    #[test]
    fn mirror_backtrace_finds_echo() {
        let history = vec!["事件A".to_string(), "事件B".to_string(), "事件C".to_string()];
        let img = MirrorBacktraceImage::trace(TemporalCoord::new(2, 0, 0, 0), &history, 2);
        assert_eq!(img.echo, "事件A");
    }

    #[test]
    fn introspection_detects_contradiction() {
        let claims = vec!["这是善".to_string(), "这是恶".to_string()];
        let img = IntrospectionImage::check(&claims, 1);
        assert!(!img.contradictions.is_empty());
        assert!(img.consistency < 1.0);
    }

    #[test]
    fn introspection_no_contradiction() {
        let claims = vec!("这是善".to_string(), "这也是善".to_string());
        let img = IntrospectionImage::check(&claims, 1);
        assert!(img.contradictions.is_empty());
    }

    #[test]
    fn projection_preserves_fidelity() {
        let img = ProjectionImage::project("这是一段很长的内部状态描述信息", "输出");
        assert!(img.fidelity > 0.0 && img.fidelity <= 1.0);
        assert_eq!(img.target, "输出");
    }

    #[test]
    fn feedback_positive_signal() {
        let fb = FeedbackImage::respond("actionA", 0.7);
        assert!(fb.positive);
        assert!(fb.adaptation.contains("放大"));
    }

    #[test]
    fn feedback_negative_signal() {
        let fb = FeedbackImage::respond("actionB", -0.8);
        assert!(!fb.positive);
        assert!(fb.adaptation.contains("停止"));
    }

    #[test]
    fn reflex_learned() {
        let memory = vec![("火".to_string(), "躲避".to_string())];
        let reflex = ReflexImage::trigger("火", &memory).unwrap();
        assert_eq!(reflex.response, "躲避");
        assert!(reflex.learned);
        assert_eq!(reflex.latency_ms, 1);
    }

    #[test]
    fn awakening_stores_insight() {
        let img = AwakeningImage::awaken("不知天觉何意", "天觉即时间螺旋版本号", 3);
        assert_eq!(img.level, 3);
        assert!(img.describe().contains("悟"));
    }

    #[test]
    fn spiral_trace_expected_actual() {
        let img = SpiralTraceImage::trace(5, "预期状态", "实际状态X");
        assert_ne!(img.expected, img.actual);
        assert!(img.refraction < 1.0);
    }

    #[test]
    fn coordinate_image_decode() {
        let img = CoordinateImage::decode(TemporalCoord::new(1, 2, 3, 0), 64, "天觉启动");
        assert!(img.version_str.contains("R1"));
        assert_eq!(img.base, 64);
    }

    #[test]
    fn temporal_base_encoding() {
        let img = TemporalBaseImage::new(16, 255);
        assert_eq!(img.base, 16);
        assert!(!img.encoded.is_empty());
    }

    #[test]
    fn refraction_computes_index() {
        let img = RefractionImage::refract(1.0, 1.5);
        assert!((img.index - 1.5).abs() < 0.01);
        assert!(img.path_desc.contains("正折射"));
    }

    // ── 模块 B 测试 ──
    #[test]
    fn emotion_react_to_reward() {
        let mut es = EmotionalSystem::new();
        es.react("reward: 审判正确", TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(es.current, EmotionState::Joy);
        assert!(es.intensity > 0.0);
    }

    #[test]
    fn emotion_react_to_punish() {
        let mut es = EmotionalSystem::new();
        es.react("punish: 审判错误", TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(es.current, EmotionState::Sadness);
    }

    #[test]
    fn emotion_anger_after_double_sadness() {
        let mut es = EmotionalSystem::new();
        es.react("punish: 错误1", TemporalCoord::new(0, 0, 0, 0));
        es.react("punish: 错误2", TemporalCoord::new(0, 0, 1, 0));
        assert_eq!(es.current, EmotionState::Anger);
    }

    #[test]
    fn emotion_decay() {
        let mut es = EmotionalSystem::new();
        es.react("reward", TemporalCoord::new(0, 0, 0, 0));
        let before = es.intensity;
        es.decay();
        let after = es.intensity;
        assert!(after < before);
    }

    // ── 模块 C 测试 ──
    #[test]
    fn behavioral_system_has_rules() {
        let sys = BehavioralSystem::new();
        assert!(sys.rules.len() >= 5);
    }

    #[test]
    fn behavioral_decide_on_rule() {
        let mut sys = BehavioralSystem::new();
        let action = sys.decide("审判偏差>阈值");
        assert!(action.is_some());
        assert!(action.unwrap().action_name.contains("纠正"));
    }

    #[test]
    fn behavioral_executes_history() {
        let mut sys = BehavioralSystem::new();
        let _ = sys.decide("新袋子进入视野");
        let _ = sys.decide("符号冲突");
        assert_eq!(sys.executed.len(), 2);
    }

    // ── 模块 D 测试 ──
    #[test]
    fn aesthetic_score_all_positive() {
        let score = AestheticScore::evaluate("abba", &["abba".to_string()]);
        assert!(score.symmetry > 0.5); // 对称
        assert!(score.overall > 0.0);
    }

    #[test]
    fn aesthetic_simple_preferred() {
        let short = AestheticScore::evaluate("米", &[]);
        let long = AestheticScore::evaluate("超级长超级复杂的符号名称", &[]);
        assert!(short.simplicity >= long.simplicity);
    }

    // ── 模块 E 测试 ──
    #[test]
    fn filter_system_passes_healthy() {
        let mut fs = FilterSystem::new(0.3);
        assert!(fs.filter(1, 0.8, "质量好"));
        assert!(!fs.filter(2, 0.1, "质量差"));
        assert_eq!(fs.passed.len(), 1);
        assert_eq!(fs.eliminated.len(), 1);
    }

    #[test]
    fn filter_system_strict() {
        let mut fs_normal = FilterSystem::new(0.5);
        let mut fs_strict = FilterSystem::new(0.8);
        assert!(fs_normal.filter(1, 0.7, "ok"));
        assert!(!fs_strict.filter(1, 0.7, "ok")); // 严格模式下 0.7 < 0.8 被淘汰
    }

    // ── 模块 F 测试 ──
    #[test]
    fn self_encoding_basic() {
        let enc = SelfEncoding::new();
        assert!(enc.encode('天').is_some());
        assert!(enc.encode('地').is_some());
    }

    #[test]
    fn self_encoding_has_polarity() {
        let enc = SelfEncoding::new();
        let (_, _, tian_polarity) = enc.encode('天').unwrap();
        let (_, _, di_polarity) = enc.encode('地').unwrap();
        assert_ne!(tian_polarity, di_polarity); // 天地极性相反
    }

    #[test]
    fn self_encoding_str_encoding() {
        let enc = SelfEncoding::new();
        let encoded = enc.encode_str("天地");
        assert_eq!(encoded.len(), 2);
    }

    #[test]
    fn self_encoding_unmapped_returns_none() {
        let enc = SelfEncoding::new();
        assert!(enc.encode('?').is_none());
    }
}
