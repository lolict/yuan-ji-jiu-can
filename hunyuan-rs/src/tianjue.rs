// ══════════════════════════════════════════════════════════════════
// tianjue.rs — 天觉 · 时间螺旋版本号 · 自我意象系统 · 错误纠偏
// ══════════════════════════════════════════════════════════════════
//
// 核心公理:
//   1. 版本号 = 时间进制专用坐标 — 不可变、不可覆盖
//   2. 错误词汇用途版本号 — 发现符号用错 → 创建纠正版本
//   3. 自我觉察 = 时间迂回溯源 — 回到任意历史版本对比
//   4. 自我意象 = 感知 → 内旋 → 外射 → 反馈 → 反射 → 觉悟
//   5. 不数米粒 — 只命名袋子
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

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

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
}
