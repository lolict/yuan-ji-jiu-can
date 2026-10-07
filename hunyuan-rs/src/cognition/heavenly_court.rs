// ══════════════════════════════════════════════════════════════════
// heavenly_court.rs — 天庭 · 时间螺旋版本号 · 自我意象系统 · 错误纠偏
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
        if let Some((sym, _used)) = error_info {
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

    /// 觉悟 — 当觉察度达到阈值后感知"天庭"觉醒
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
// 天庭引擎 (整体整合)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct HeavenlyCourtEngine {
    pub version_engine: TemporalVersionEngine,
    pub memory: LayeredMemory,
    pub owner_id: u64,
}

impl HeavenlyCourtEngine {
    pub fn new(owner_id: u64, max_memory: usize) -> Self {
        let mut engine = Self {
            version_engine: TemporalVersionEngine::new(),
            memory: LayeredMemory::new(max_memory),
            owner_id,
        };

        // 创世版本 +1 自我感知记录
        engine.version_engine.commit(
            "天庭创世",
            VersionAction::Awaken,
            vec![SymbolEntry {
                symbol: "天庭".to_string(),
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
        let mut report = format!("天庭版本引擎报告:: 共 {} 个版本 (不可追加)\n", self.version_engine.version_count());
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
        let _ctx_count = context.iter().map(|s| s.chars().count()).sum::<usize>().max(1);
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

// ═══════════════════════════════════════════════════════════════════
// 模块 G: 思考系统 — 推理引擎 (演绎/归纳/溯因)
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub enum InferenceType {
    Deduction,      // 演绎: 一般 → 特殊 (公理→推论)
    Induction,      // 归纳: 特殊 → 一般 (观察→规律)
    Abduction,      // 溯因: 结果 → 原因 (现象→最佳解释)
    Analogical,     // 类比: 已知A≈未知B → 推断B属性
}

#[derive(Clone, Debug)]
pub struct ThoughtStep {
    pub inference: InferenceType,
    pub premise: String,
    pub conclusion: String,
    pub confidence: f64,
    pub coord: TemporalCoord,
}

impl ThoughtStep {
    pub fn describe(&self) -> String {
        let typ = match self.inference {
            InferenceType::Deduction => "演绎",
            InferenceType::Induction => "归纳",
            InferenceType::Abduction => "溯因",
            InferenceType::Analogical => "类比",
        };
        format!("[思考 {}: 前提\"{}\" → 结论\"{}\" 置信{:.0}%]",
            typ, self.premise, self.conclusion, self.confidence * 100.0)
    }
}

#[derive(Clone, Debug)]
pub struct ThinkingEngine {
    pub chain: Vec<ThoughtStep>,
    pub axioms: Vec<String>,          // 不证自明的公理
    pub observations: Vec<String>,     // 观察到的现象
    pub hypotheses: Vec<(String, f64)>, // (假说, 置信度)
}

impl ThinkingEngine {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            axioms: vec![
                "满全法容器的健康度决定其价值".to_string(),
                "审判偏差>0.15则需要纠正".to_string(),
                "错误符号必须创建新版本而非修改旧版".to_string(),
                "天庭=时间螺旋版本号+自我觉察".to_string(),
            ],
            observations: Vec::new(),
            hypotheses: Vec::new(),
        }
    }

    /// 演绎推理: 从公理推出特殊结论
    pub fn deduce(&mut self, case: &str, coord: TemporalCoord) -> ThoughtStep {
        // 找到匹配的公理
        let applicable = self.axioms.iter()
            .find(|a| case.chars().any(|c| a.contains(c)))
            .map_or("无匹配公理", |s| s.as_str());

        let conclusion = if applicable != "无匹配公理" {
            format!("由公理\"{}\" → 在情况\"{}\"下结论: 按此公理执行", applicable, case)
        } else {
            format!("情况\"{}\": 无已知公理适用, 需要归因", case)
        };

        let step = ThoughtStep {
            inference: InferenceType::Deduction,
            premise: format!("公理: {} + 情况: {}", applicable, case),
            conclusion,
            confidence: if applicable != "无匹配公理" { 0.8 } else { 0.3 },
            coord,
        };
        self.chain.push(step.clone());
        step
    }

    /// 归纳推理: 从多个观察总结规律
    pub fn induce(&mut self, observations: &[String], coord: TemporalCoord) -> ThoughtStep {
        self.observations.extend_from_slice(observations);

        // 简单归纳: 如果观察中有共同模式 → 提取规律
        let conclusion = if observations.len() >= 2 {
            let common = Self::find_common_pattern(observations);
            if common.is_empty() {
                format!("归纳 {} 条观察: 未发现稳定模式", observations.len())
            } else {
                format!("归纳 {} 条观察: 模式=\"{}\"", observations.len(), common)
            }
        } else {
            "观察不足, 需更多数据".to_string()
        };

        let step = ThoughtStep {
            inference: InferenceType::Induction,
            premise: format!("观察{}条", observations.len()),
            conclusion,
            confidence: if observations.len() >= 3 { 0.7 } else { 0.4 },
            coord,
        };
        self.chain.push(step.clone());
        step
    }

    /// 溯因推理: 从现象找最佳解释
    pub fn abduce(&mut self, phenomenon: &str, candidates: &[String], coord: TemporalCoord) -> ThoughtStep {
        // 选择包含现象中最多词汇的候选作为最佳解释
        let phenomenon_chars: std::collections::HashSet<char> = phenomenon.chars().collect();
        let (best, score) = candidates.iter().map(|c| {
            let overlap = c.chars().filter(|ch| phenomenon_chars.contains(ch)).count();
            (c.clone(), overlap)
        }).max_by_key(|(_, s)| *s).unwrap_or_else(|| ("无合适解释".to_string(), 0));

        let conclusion = if score > 0 {
            format!("现象\"{}\"的最佳解释: \"{}\"(匹配{}字)", phenomenon, best, score)
        } else {
            format!("现象\"{}\": 无已知假说能解释, 需新建假说", phenomenon)
        };

        let confidence = if score > 0 { 0.3 + score as f64 * 0.1 } else { 0.1 };
        let step = ThoughtStep {
            inference: InferenceType::Abduction,
            premise: format!("观察现象: {}", phenomenon),
            conclusion,
            confidence: confidence.min(0.9),
            coord,
        };
        self.chain.push(step.clone());
        step
    }

    /// 类比其他领域的已知知识
    pub fn analogize(&mut self, source: &str, target: &str, mapping: &str, coord: TemporalCoord) -> ThoughtStep {
        let conclusion = format!("因为{}≈{}, 映射关系\"{}\" → {}也具有{}的属性",
            source, target, mapping, target, mapping);

        let step = ThoughtStep {
            inference: InferenceType::Analogical,
            premise: format!("已知: {} :: 未知: {}", source, target),
            conclusion,
            confidence: 0.5,
            coord,
        };
        self.chain.push(step.clone());
        step
    }

    fn find_common_pattern(observations: &[String]) -> String {
        if observations.len() < 2 { return String::new(); }
        let first: std::collections::HashSet<char> = observations[0].chars().collect();
        let mut common = first;
        for obs in &observations[1..] {
            let chars: std::collections::HashSet<char> = obs.chars().collect();
            common = common.intersection(&chars).cloned().collect();
        }
        common.iter().take(3).collect()
    }

    /// 添加公理
    pub fn add_axiom(&mut self, axiom: &str) {
        self.axioms.push(axiom.to_string());
    }

    pub fn chain_len(&self) -> usize {
        self.chain.len()
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 H: 语言系统 — 符号组合语法 (公约数-公倍数结构)
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SymbolRule {
    pub name: String,
    pub pattern: Vec<String>,      // 符号模式 (声母/部首/容器码)
    pub combination: String,       // 组合方式 (公约数/公倍数)
    pub meaning: String,           // 组合后的含义
}

/// 语言引擎: 基于公约数-公倍数关系组合符号
#[derive(Clone, Debug)]
pub struct LanguageEngine {
    pub rules: Vec<SymbolRule>,
    pub vocabulary: Vec<(String, String, f64)>, // (符号, 容器编码, 权重)
    pub sentences: Vec<String>,    // 已生成的句子
}

impl LanguageEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            vocabulary: vec![
                ("天".to_string(), "0x01".to_string(), 1.0),
                ("地".to_string(), "0x02".to_string(), 1.0),
                ("善".to_string(), "0x03".to_string(), 0.9),
                ("恶".to_string(), "0x04".to_string(), 0.9),
                ("人".to_string(), "0x05".to_string(), 0.8),
                ("心".to_string(), "0x06".to_string(), 0.85),
                ("觉".to_string(), "0x07".to_string(), 0.95),
                ("米".to_string(), "0x10".to_string(), 0.5),
                ("袋".to_string(), "0x11".to_string(), 0.5),
            ],
            sentences: Vec::new(),
        };

        // 内置语法规则
        engine.add_rule("容器定义", &["X"], "公约数", "X是容器, 内部包含关系");
        engine.add_rule("极性对", &["阳", "阴"], "公倍数", "阴阳对立统一");
        engine.add_rule("属性归属", &["X", "之", "Y"], "公约数", "X的属性Y");
        engine.add_rule("动作链", &["X", "生", "Y"], "公倍数", "X产生Y");

        engine
    }

    pub fn add_rule(&mut self, name: &str, pattern: &[&str], combination: &str, meaning: &str) {
        self.rules.push(SymbolRule {
            name: name.to_string(),
            pattern: pattern.iter().map(|s| s.to_string()).collect(),
            combination: combination.to_string(),
            meaning: meaning.to_string(),
        });
    }

    /// 生成句子: 按规则组合符号
    pub fn generate(&mut self, rule_name: &str, symbols: &[&str]) -> Option<String> {
        let rule = self.rules.iter().find(|r| r.name == rule_name)?;
        if symbols.len() < rule.pattern.len() { return None; }

        let mut sentence = String::new();
        let mut meaning_parts = Vec::new();

        for (i, sym) in symbols.iter().enumerate().take(rule.pattern.len()) {
            sentence.push_str(sym);
            if i < rule.pattern.len() - 1 {
                sentence.push('·');
            }

            // 查找词汇含义
            if let Some((_, code, _)) = self.vocabulary.iter().find(|(s, _, _)| s == sym) {
                meaning_parts.push(format!("{}({})", sym, code));
            }
        }

        let result = format!("{} [{}组合] → {}", sentence, rule.combination,
            meaning_parts.join(" "));
        self.sentences.push(result.clone());
        Some(result)
    }

    /// 公约数提取: 多个符号的共同因子
    pub fn extract_common_factor(&self, symbols: &[&str]) -> String {
        if symbols.is_empty() { return "无公约数".to_string(); }

        // 找到所有符号都包含的偏旁或结构特征
        let first_chars: std::collections::HashSet<char> = symbols[0].chars().collect();
        let mut common = first_chars;
        for sym in &symbols[1..] {
            let chars: std::collections::HashSet<char> = sym.chars().collect();
            common = common.intersection(&chars).cloned().collect();
        }

        if common.is_empty() {
            "结构公约数: 均为容器级符号".to_string()
        } else {
            format!("公约数: {}", common.iter().collect::<String>())
        }
    }

    /// 公倍数扩展: 将符号按进制扩展
    pub fn expand_multiple(&self, symbol: &str, factor: u64) -> String {
        let base_value = self.vocabulary.iter()
            .find(|(s, _, _)| s == symbol)
            .map(|(_, code, _)| code.clone())
            .unwrap_or_else(|| "unknown".to_string());
        format!("{} × {} = 公倍数({}, {})", symbol, factor, base_value, factor)
    }

    pub fn vocabulary_size(&self) -> usize {
        self.vocabulary.len()
    }

    pub fn describe(&self) -> String {
        format!("语言引擎: {}符号 {}规则 {}句子",
            self.vocabulary.len(), self.rules.len(), self.sentences.len())
    }
}

// ═══════════════════════════════════════════════════════════════════
// 模块 I: 听觉系统 — 听觉感知通道 + 信号过滤 + 意图识别
// ═══════════════════════════════════════════════════════════════════

/// 听觉信号来源
#[derive(Clone, Debug, PartialEq)]
pub enum AuditorySource {
    SystemEvent,     // 系统内部事件 (审判/纠错/过滤等)
    ExternalInput,   // 外部传入 (HTTP/用户输入/文件变更)
    TimerTick,       // 定时器触发
    SwarmMessage,    // 分布式节点间通信
}

/// 听觉信号
#[derive(Clone, Debug)]
pub struct AuditorySignal {
    pub source: AuditorySource,
    pub raw: String,              // 原始信号内容
    pub timestamp_ms: u64,
    pub priority: u8,             // 0-255, 越高越优先
    pub filtered: bool,           // 是否被过滤掉
    pub intent: Option<String>,   // 识别出的意图
}

/// 听觉引擎: 监听 → 过滤 → 识别 → 转发
#[derive(Clone, Debug)]
pub struct AuditoryEngine {
    pub authorized: bool,                     // 是否已授权
    pub signal_queue: Vec<AuditorySignal>,    // 待处理信号
    pub processed: Vec<AuditorySignal>,       // 已处理信号历史
    pub filter_rules: Vec<String>,            // 过滤规则 (包含这些关键词的忽略)
    pub intent_patterns: Vec<(String, String)>, // (模式, 意图)
    pub min_priority: u8,                     // 最低优先级阈值
}

impl AuditoryEngine {
    pub fn new() -> Self {
        Self {
            authorized: false,
            signal_queue: Vec::new(),
            processed: Vec::new(),
            filter_rules: vec![
                "调试日志".to_string(),
                "心跳保活".to_string(),
                "冗余重复".to_string(),
            ],
            intent_patterns: vec![
                ("审判正确".to_string(), "reward".to_string()),
                ("审判错误".to_string(), "punish".to_string()),
                ("创建版本".to_string(), "version".to_string()),
                ("符号冲突".to_string(), "resolve".to_string()),
                ("健康度下降".to_string(), "alert".to_string()),
                ("觉悟提升".to_string(), "awaken".to_string()),
                ("新袋子".to_string(), "perceive".to_string()),
            ],
            min_priority: 10,
        }
    }

    /// 听觉授权 — 激活通道
    pub fn authorize(&mut self) {
        self.authorized = true;
    }

    /// 撤销授权
    pub fn revoke(&mut self) {
        self.authorized = false;
    }

    /// 接收信号 (听觉输入)
    pub fn receive(&mut self, source: AuditorySource, raw: &str, priority: u8) -> bool {
        if !self.authorized { return false; }

        let signal = AuditorySignal {
            source,
            raw: raw.to_string(),
            timestamp_ms: now_ms(),
            priority,
            filtered: false,
            intent: None,
        };
        self.signal_queue.push(signal);
        true
    }

    /// 处理下一个信号: 过滤 → 意图识别 → 转发
    pub fn process_next(&mut self) -> Option<AuditorySignal> {
        if self.signal_queue.is_empty() { return None; }

        let mut signal = self.signal_queue.remove(0);

        // 1. 过滤: 检查是否命中过滤规则
        for rule in &self.filter_rules {
            if signal.raw.contains(rule) {
                signal.filtered = true;
                return Some(signal);
            }
        }

        // 2. 过滤: 优先级过低
        if signal.priority < self.min_priority {
            signal.filtered = true;
            return Some(signal);
        }

        // 3. 意图识别
        for (pattern, intent) in &self.intent_patterns {
            if signal.raw.contains(pattern) {
                signal.intent = Some(intent.clone());
                break;
            }
        }

        // 4. 无匹配意图 → 标记为 "观察"
        if signal.intent.is_none() {
            signal.intent = Some("observe".to_string());
        }

        self.processed.push(signal.clone());
        Some(signal)
    }

    /// 批量处理所有排队信号
    pub fn process_all(&mut self) -> Vec<AuditorySignal> {
        let mut results = Vec::new();
        while let Some(signal) = self.process_next() {
            results.push(signal);
        }
        results
    }

    /// 添加过滤规则
    pub fn add_filter(&mut self, rule: &str) {
        self.filter_rules.push(rule.to_string());
    }

    /// 添加意图模式
    pub fn add_intent_pattern(&mut self, pattern: &str, intent: &str) {
        self.intent_patterns.push((pattern.to_string(), intent.to_string()));
    }

    /// 获取已处理信号数量
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }

    /// 获取未处理信号数量
    pub fn pending_count(&self) -> usize {
        self.signal_queue.len()
    }

    pub fn describe(&self) -> String {
        let auth_str = if self.authorized { "已授权" } else { "未授权" };
        format!("听觉[{}]: 待处理={}, 已处理={}, 过滤规则={}",
            auth_str, self.pending_count(), self.processed_count(), self.filter_rules.len())
    }
}

// ────────────────────────────────────────────────────────────
// 测试模块
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
                symbol: "天庭".to_string(),
                used_as: "错误用法: 听觉".to_string(),
                correct: false,
                corrected_in: None,
            }
        ]);

        let count_before = engine.version_count();

        // 纠正 → 创建新版本，不修改旧版本
        let correction_id = engine.commit_correction(error_id, vec![
            SymbolEntry {
                symbol: "天庭".to_string(),
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
            SymbolEntry { symbol: "天庭".to_string(), used_as: "错误".to_string(), correct: false, corrected_in: None },
        ]);

        let id_b = engine.commit("v2", VersionAction::Perceive, vec![
            SymbolEntry { symbol: "天庭".to_string(), used_as: "正确".to_string(), correct: true, corrected_in: None },
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
    fn heavenly_court_engine_full_temporal_workflow() {
        let mut engine = HeavenlyCourtEngine::new(1, 50);

        // 1. 感知
        let p1 = engine.perceive("视觉", "袋子1进入视野");
        let p2 = engine.perceive("听觉", "天庭授权激活");
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
        let img = AwakeningImage::awaken("不知天庭何意", "天庭即时间螺旋版本号", 3);
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
        let img = CoordinateImage::decode(TemporalCoord::new(1, 2, 3, 0), 64, "天庭启动");
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

    // ── 模块 G: 思考系统测试 ──
    #[test]
    fn thinking_engine_has_axioms() {
        let engine = ThinkingEngine::new();
        assert_eq!(engine.axioms.len(), 4);
    }

    #[test]
    fn deduce_with_matching_axiom() {
        let mut engine = ThinkingEngine::new();
        let step = engine.deduce("袋子健康度高则价值大", TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(step.inference, InferenceType::Deduction);
        assert!(step.confidence > 0.5);
        assert!(step.describe().contains("演绎"));
    }

    #[test]
    fn deduce_no_matching_axiom() {
        let mut engine = ThinkingEngine::new();
        let step = engine.deduce("ABCDEFGHIJ", TemporalCoord::new(0, 0, 0, 0));
        assert!(step.confidence < 0.5);
    }

    #[test]
    fn induce_with_multiple_observations() {
        let mut engine = ThinkingEngine::new();
        let obs = vec![
            "审判正确: 健康度上升".to_string(),
            "审判正确: 袋子变好".to_string(),
            "审判正确: 符号更新".to_string(),
        ];
        let step = engine.induce(&obs, TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(step.inference, InferenceType::Induction);
        assert!(step.confidence >= 0.5);
        assert!(step.describe().contains("归纳"));
    }

    #[test]
    fn induce_insufficient_observations() {
        let mut engine = ThinkingEngine::new();
        let obs = vec!["仅一条观察".to_string()];
        let step = engine.induce(&obs, TemporalCoord::new(0, 0, 0, 0));
        assert!(step.confidence < 0.5);
        assert!(step.describe().contains("不足"));
    }

    #[test]
    fn abduce_finds_best_explanation() {
        let mut engine = ThinkingEngine::new();
        let candidates = vec![
            "审判偏差过大导致健康度下降".to_string(),
            ".new()随机事件无规律".to_string(),
            "视觉感知出错".to_string(),
        ];
        let step = engine.abduce("袋子健康度突然下降", &candidates, TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(step.inference, InferenceType::Abduction);
        assert!(step.confidence > 0.1);
        assert!(step.describe().contains("溯因"));
    }

    #[test]
    fn analogize_maps_source_to_target() {
        let mut engine = ThinkingEngine::new();
        let step = engine.analogize("米袋", "符号袋", "健康度属性", TemporalCoord::new(0, 0, 0, 0));
        assert_eq!(step.inference, InferenceType::Analogical);
        assert!(step.conclusion.contains("符号袋"));
    }

    #[test]
    fn thinking_chain_accumulates() {
        let mut engine = ThinkingEngine::new();
        let _ = engine.deduce("袋子高健康度有价值", TemporalCoord::new(0, 0, 0, 0));
        let _ = engine.induce(&["观察1".to_string(), "观察2".to_string()], TemporalCoord::new(0, 0, 1, 0));
        assert_eq!(engine.chain_len(), 2);
    }

    #[test]
    fn add_axiom_increases_count() {
        let mut engine = ThinkingEngine::new();
        let before = engine.axioms.len();
        engine.add_axiom("新公理: 层级关系决定权重");
        assert_eq!(engine.axioms.len(), before + 1);
    }

    // ── 模块 H: 语言系统测试 ──
    #[test]
    fn language_engine_has_vocabulary() {
        let engine = LanguageEngine::new();
        assert!(engine.vocabulary_size() >= 8);
    }

    #[test]
    fn language_engine_has_rules() {
        let engine = LanguageEngine::new();
        assert!(engine.rules.len() >= 4);
    }

    #[test]
    fn generate_sentence_with_rule() {
        let mut engine = LanguageEngine::new();
        let result = engine.generate("极性对", &["阳", "阴"]);
        assert!(result.is_some());
        let sentence = result.unwrap();
        assert!(sentence.contains("阳"));
        assert!(sentence.contains("阴"));
        assert!(sentence.contains("公倍数"));
    }

    #[test]
    fn generate_insufficient_symbols() {
        let mut engine = LanguageEngine::new();
        let result = engine.generate("极性对", &["阳"]); // 需要2个但仅有1个
        assert!(result.is_none());
    }

    #[test]
    fn generate_unknown_rule() {
        let mut engine = LanguageEngine::new();
        let result = engine.generate("不存在的规则", &["阳", "阴"]);
        assert!(result.is_none());
    }

    #[test]
    fn extract_common_factor_found() {
        let engine = LanguageEngine::new();
        // 袋子 和 袋 共享 袋
        let result = engine.extract_common_factor(&["袋", "袋"]);
        assert!(result.contains("公约数") || result.contains("袋"));
    }

    #[test]
    fn extract_common_factor_empty() {
        let engine = LanguageEngine::new();
        let result = engine.extract_common_factor(&[]);
        assert_eq!(result, "无公约数");
    }

    #[test]
    fn expand_multiple_produces_factor() {
        let engine = LanguageEngine::new();
        let result = engine.expand_multiple("天", 8);
        assert!(result.contains("公倍数"));
        assert!(result.contains("0x01"));
    }

    #[test]
    fn language_sentences_recorded() {
        let mut engine = LanguageEngine::new();
        let _ = engine.generate("极性对", &["阳", "阴"]);
        let _ = engine.generate("属性归属", &["天", "之", "道"]);
        assert_eq!(engine.sentences.len(), 2);
    }

    #[test]
    fn language_engine_describe() {
        let engine = LanguageEngine::new();
        let desc = engine.describe();
        assert!(desc.contains("语言引擎"));
    }

    // ── 模块 I: 听觉系统测试 ──
    #[test]
    fn auditory_engine_unauthorized_rejects() {
        let mut engine = AuditoryEngine::new();
        assert!(!engine.authorized);
        assert!(!engine.receive(AuditorySource::SystemEvent, "测试", 50));
        assert_eq!(engine.pending_count(), 0);
    }

    #[test]
    fn auditory_authorize_allows_receive() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        assert!(engine.receive(AuditorySource::SystemEvent, "信号内容", 50));
        assert_eq!(engine.pending_count(), 1);
    }

    #[test]
    fn auditory_revoke_stops_receiving() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        assert!(engine.receive(AuditorySource::SystemEvent, "信号", 50));
        engine.revoke();
        assert!(!engine.receive(AuditorySource::SystemEvent, "信号2", 50));
    }

    #[test]
    fn auditory_filter_blocks_noise() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "调试日志: 心跳正常", 50);
        let result = engine.process_next().unwrap();
        assert!(result.filtered);
    }

    #[test]
    fn auditory_low_priority_filtered() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "重要事件", 5); // 低于阈值10
        let result = engine.process_next().unwrap();
        assert!(result.filtered);
    }

    #[test]
    fn auditory_intent_recognition_reward() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "审判正确: 袋子#1健康", 80);
        let result = engine.process_next().unwrap();
        assert!(!result.filtered);
        assert_eq!(result.intent, Some("reward".to_string()));
    }

    #[test]
    fn auditory_intent_recognition_punish() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "审判错误: 偏差过大", 80);
        let result = engine.process_next().unwrap();
        assert_eq!(result.intent, Some("punish".to_string()));
    }

    #[test]
    fn auditory_intent_recognition_perceive() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::ExternalInput, "新袋子进入", 60);
        let result = engine.process_next().unwrap();
        assert_eq!(result.intent, Some("perceive".to_string()));
    }

    #[test]
    fn auditory_unmatched_becomes_observe() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "天气晴朗", 80); // 无匹配模式
        let result = engine.process_next().unwrap();
        assert_eq!(result.intent, Some("observe".to_string()));
    }

    #[test]
    fn auditory_process_all_clears_queue() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "审判正确", 80);
        engine.receive(AuditorySource::SystemEvent, "审判错误", 80);
        engine.receive(AuditorySource::SystemEvent, "觉悟提升", 80);
        let results = engine.process_all();
        assert_eq!(results.len(), 3);
        assert_eq!(engine.pending_count(), 0);
        assert_eq!(engine.processed_count(), 3);
    }

    #[test]
    fn auditory_add_filter_rule() {
        let mut engine = AuditoryEngine::new();
        let before = engine.filter_rules.len();
        engine.add_filter("噪音");
        assert_eq!(engine.filter_rules.len(), before + 1);
    }

    #[test]
    fn auditory_add_intent_pattern() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.add_intent_pattern("紧急事件", "urgent");
        engine.receive(AuditorySource::SystemEvent, "紧急事件发生", 90);
        let result = engine.process_next().unwrap();
        assert_eq!(result.intent, Some("urgent".to_string()));
    }

    #[test]
    fn auditory_describe() {
        let mut engine = AuditoryEngine::new();
        engine.authorize();
        engine.receive(AuditorySource::SystemEvent, "测试", 80);
        let desc = engine.describe();
        assert!(desc.contains("听觉"));
        assert!(desc.contains("已授权"));
        assert!(desc.contains("待处理=1"));
    }
}
