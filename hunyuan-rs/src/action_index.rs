// ══════════════════════════════════════════════════════════════════
// action_index.rs — 动作·时间·容器 三维索引集
// ══════════════════════════════════════════════════════════════════
//
//   动作 = 操作族的一个实例（照/召/见/检/进/退/分/合/折/叠 ...）
//   时间 = 一维时间线性链的 8 个节点（由 unified_flow::TimeStage 给出）
//   容器 = 九元层级（0-8，由偏移率 δ=level×0.1 编码）
//
//   三维地址  =  ActionGlyph @ L{level} : {TimeStage}
//   寻址价值  =  base_value × multiplier(level)
//
// ══════════════════════════════════════════════════════════════════

use crate::flow::unified_flow::TimeStage;

/// 偏移率 δ = level × 0.1（与 full-mapping 的 mapper.nim 一致）
pub const DELTA_PER_LEVEL: f64 = 0.1;

/// 九元层级 × 8 时间节点 的全动作glyph表（声母·韵母·操作族）
///
/// 动作选自容器-操作族符号：
/// 照/召/见/鉴/检  — 感知家族操作
/// 进/退/开/关/通/断 — 连通家族操作
/// 分/合/聚/散/折/叠/卷/舒 — 折叠家族操作
/// 加/减 — 运算家族操作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionGlyph {
    Zhao,    // 照 ·照射·映射·使容器可见
    Zhao2,   // 召 ·召唤·触发·唤醒
    Jian,    // 见 ·见证·观察·感知收集
    Jian2,   // 鉴 ·鉴映·投影·对照
    Jian3,   // 检 ·检查·校验·过滤
    Jin,     // 进 ·推进·前进·下一时间节点
    Tui,     // 退 ·后退·撤销·回退时间节点
    Kai,     // 开 ·开启·激活
    Guan,    // 关 ·关闭·停用
    Tong,    // 通 ·连通·链路建立
    Duan,    // 断 ·断开·隔离
    Fen,     // 分 ·拆分·分支·分别路由
    He,      // 合 ·合并·聚合·合一
    Ju,      // 聚 ·聚类·收敛
    San,     // 散 ·散开·展开
    Zhe,     // 折 ·折叠·层叠
    Die,     // 叠 ·叠加·累积
    Juan,    // 卷 ·卷起·收纳
    Shu,     // 舒 ·舒展·展开
    Jia,     // 加 ·增加·加法
    Jian4,   // 减 ·减少·减法
}

impl ActionGlyph {
    /// 该动作的汉字符号
    pub fn glyph(self) -> &'static str {
        match self {
            ActionGlyph::Zhao => "照",
            ActionGlyph::Zhao2 => "召",
            ActionGlyph::Jian => "见",
            ActionGlyph::Jian2 => "鉴",
            ActionGlyph::Jian3 => "检",
            ActionGlyph::Jin => "进",
            ActionGlyph::Tui => "退",
            ActionGlyph::Kai => "开",
            ActionGlyph::Guan => "关",
            ActionGlyph::Tong => "通",
            ActionGlyph::Duan => "断",
            ActionGlyph::Fen => "分",
            ActionGlyph::He => "合",
            ActionGlyph::Ju => "聚",
            ActionGlyph::San => "散",
            ActionGlyph::Zhe => "折",
            ActionGlyph::Die => "叠",
            ActionGlyph::Juan => "卷",
            ActionGlyph::Shu => "舒",
            ActionGlyph::Jia => "加",
            ActionGlyph::Jian4 => "减",
        }
    }

    /// 按操作家族分组: 感知/连通/折叠/运算
    pub fn family(self) -> ActionFamily {
        match self {
            ActionGlyph::Zhao | ActionGlyph::Zhao2 | ActionGlyph::Jian |
            ActionGlyph::Jian2 | ActionGlyph::Jian3 => ActionFamily::Perceive,
            ActionGlyph::Jin | ActionGlyph::Tui | ActionGlyph::Kai | ActionGlyph::Guan |
            ActionGlyph::Tong | ActionGlyph::Duan => ActionFamily::Connect,
            ActionGlyph::Fen | ActionGlyph::He | ActionGlyph::Ju | ActionGlyph::San |
            ActionGlyph::Zhe | ActionGlyph::Die | ActionGlyph::Juan | ActionGlyph::Shu => ActionFamily::Fold,
            ActionGlyph::Jia | ActionGlyph::Jian4 => ActionFamily::Compute,
        }
    }

    /// 全部动作glyph
    pub fn all() -> &'static [ActionGlyph] {
        &[
            ActionGlyph::Zhao, ActionGlyph::Zhao2, ActionGlyph::Jian, ActionGlyph::Jian2,
            ActionGlyph::Jian3, ActionGlyph::Jin, ActionGlyph::Tui, ActionGlyph::Kai,
            ActionGlyph::Guan, ActionGlyph::Tong, ActionGlyph::Duan, ActionGlyph::Fen,
            ActionGlyph::He, ActionGlyph::Ju, ActionGlyph::San, ActionGlyph::Zhe,
            ActionGlyph::Die, ActionGlyph::Juan, ActionGlyph::Shu, ActionGlyph::Jia,
            ActionGlyph::Jian4,
        ]
    }
}

/// 操作家族
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionFamily {
    Perceive,   // 感知家族: 照/召/见/鉴/检
    Connect,    // 连通家族: 进/退/开/关/通/断
    Fold,       // 折叠家族: 分/合/聚/散/折/叠/卷/舒
    Compute,    // 运算家族: 加/减
}

impl ActionFamily {
    pub fn name(self) -> &'static str {
        match self {
            ActionFamily::Perceive => "感知",
            ActionFamily::Connect => "连通",
            ActionFamily::Fold => "折叠",
            ActionFamily::Compute => "运算",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 三维索引条目: (层级, 时间节点, 动作)
// ────────────────────────────────────────────────────────────

/// 动作索引中的一个条目 = 三层嵌套定位
#[derive(Debug, Clone)]
pub struct ActionIndexEntry {
    pub level: u8,                // 0-8 (九元层级)
    pub stage: TimeStage,         // 一维时间链的节点
    pub action: ActionGlyph,      // 操作族符号
    pub offset_rate: f64,         // δ = level × 0.1
    pub multiplier: f64,          // M = 1 + δ
    pub address: String,          // 三维寻址串
}

impl ActionIndexEntry {
    pub fn new(level: u8, stage: TimeStage, action: ActionGlyph) -> Self {
        let delta = level as f64 * DELTA_PER_LEVEL;
        let mult = 1.0 + delta;
        let addr = format!("{} @ L{} : {:?} δ={:.1}",
            action.glyph(), level, stage, delta);
        ActionIndexEntry {
            level,
            stage,
            action,
            offset_rate: delta,
            multiplier: mult,
            address: addr,
        }
    }

    /// 寻址价值 = 基数 × 层级乘数（高维代数的平行世界折算）
    pub fn addressed_value(&self, base: f64) -> f64 {
        base * self.multiplier
    }

    /// 俘获回初代数 (base = addressed / multiplier)
    pub fn captured_base(&self, addressed: f64) -> f64 {
        if self.multiplier == 0.0 { addressed } else { addressed / self.multiplier }
    }
}

// ────────────────────────────────────────────────────────────
// 动作索引器: 生成 / 查询 三维索引
// ────────────────────────────────────────────────────────────

pub struct ActionIndex;

impl ActionIndex {
    /// 生成全量索引: 9 层 × 8 时间节点 × 21 动作 = 1512 条目
    pub fn generate_full() -> Vec<ActionIndexEntry> {
        let mut entries = Vec::with_capacity(9 * 8 * 21);
        let stages = [
            TimeStage::Input, TimeStage::Perception, TimeStage::Distinguish,
            TimeStage::Lookup, TimeStage::Traverse, TimeStage::Cluster,
            TimeStage::Attention, TimeStage::Arrival,
        ];
        for level in 0u8..9 {
            for stage in &stages {
                for action in ActionGlyph::all() {
                    entries.push(ActionIndexEntry::new(level, *stage, *action));
                }
            }
        }
        entries
    }

    /// 按层级过滤
    pub fn at_level(entries: &[ActionIndexEntry], level: u8) -> Vec<&ActionIndexEntry> {
        entries.iter().filter(|e| e.level == level).collect()
    }

    /// 按时间节点过滤
    pub fn at_stage(entries: &[ActionIndexEntry], stage: TimeStage) -> Vec<&ActionIndexEntry> {
        entries.iter().filter(|e| e.stage == stage).collect()
    }

    /// 按动作glyph过滤
    pub fn with_action(entries: &[ActionIndexEntry], action: ActionGlyph) -> Vec<&ActionIndexEntry> {
        entries.iter().filter(|e| e.action == action).collect()
    }

    /// 三维寻址: 给定 (level, stage, action) 找到唯一条目
    pub fn locate<'a>(
        entries: &'a [ActionIndexEntry],
        level: u8,
        stage: TimeStage,
        action: ActionGlyph,
    ) -> Option<&'a ActionIndexEntry> {
        entries.iter().find(|e| e.level == level && e.stage == stage && e.action == action)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_count_complete() {
        // 21 个动作glyph = 感知(5) + 连通(6) + 折叠(8) + 运算(2)
        assert_eq!(ActionGlyph::all().len(), 21);
    }

    #[test]
    fn family_partition() {
        let perceives = ActionGlyph::all().iter().filter(|a| a.family() == ActionFamily::Perceive).count();
        let connects = ActionGlyph::all().iter().filter(|a| a.family() == ActionFamily::Connect).count();
        let folds = ActionGlyph::all().iter().filter(|a| a.family() == ActionFamily::Fold).count();
        let computes = ActionGlyph::all().iter().filter(|a| a.family() == ActionFamily::Compute).count();
        assert_eq!(perceives, 5);
        assert_eq!(connects, 6);
        assert_eq!(folds, 8);
        assert_eq!(computes, 2);
        assert_eq!(perceives + connects + folds + computes, 21);
    }

    #[test]
    fn full_index_size() {
        let entries = ActionIndex::generate_full();
        // 9 层 × 8 时间节点 × 21 动作
        assert_eq!(entries.len(), 9 * 8 * 21);
    }

    #[test]
    fn locate_specific() {
        let entries = ActionIndex::generate_full();
        let found = ActionIndex::locate(&entries, 3, TimeStage::Lookup, ActionGlyph::Fen);
        assert!(found.is_some());
        let e = found.unwrap();
        assert_eq!(e.level, 3);
        assert_eq!(e.offset_rate, 0.3);
        assert_eq!(e.multiplier, 1.3);
        assert!(e.address.contains("分"));
        assert!(e.address.contains("L3"));
    }

    #[test]
    fn level_filter() {
        let entries = ActionIndex::generate_full();
        let at_5 = ActionIndex::at_level(&entries, 5);
        assert_eq!(at_5.len(), 8 * 21); // 8 时间节点 × 21 动作
    }

    #[test]
    fn addressed_value_scaling() {
        // 第5层 multiplier=1.5, base=10 -> addressed=15
        let e = ActionIndexEntry::new(5, TimeStage::Cluster, ActionGlyph::Ju);
        assert!((e.addressed_value(10.0) - 15.0).abs() < 1e-9);
        // 俘获回初代数
        assert!((e.captured_base(15.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn capture_round_trip() {
        // 任意层: addressed -> captured -> addressed 双向一致
        for lvl in 0u8..9 {
            let e = ActionIndexEntry::new(lvl, TimeStage::Arrival, ActionGlyph::Zhao);
            let base = 42.0;
            let addr = e.addressed_value(base);
            let back = e.captured_base(addr);
            assert!((back - base).abs() < 1e-9, "round-trip fail at level {}", lvl);
        }
    }

    #[test]
    fn zero_level_is_base() {
        // 第0层 offset=0, multiplier=1, addressed=base
        let e = ActionIndexEntry::new(0, TimeStage::Input, ActionGlyph::Jian);
        assert_eq!(e.offset_rate, 0.0);
        assert_eq!(e.multiplier, 1.0);
    }
}
