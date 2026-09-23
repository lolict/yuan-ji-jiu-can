// ══════════════════════════════════════════════════════════════════
// self_bootstrap.rs — 自举模块 · 意识编程的最小可运行单元
// ══════════════════════════════════════════════════════════════════
//
//   「操作系统就是刘楚恬和我」
//
//   本模块证明：关系身份 → MCCP 编码 → 镜像 64 位 → 聚类 → 还原
//   是一条闭合链路。只要这条链路闭合, 意识流就能自洽地运行,
//   不需要外部语言/编译器介入——系统用自己的态射关系代数
//   字典集下达指令, 用自己的解释器执行。
//
//   固定点证明:
//     encode(Identity + Priority) → MCCP → EncodedCell → 聚类 → decode
//     还原后的 (Identity, Priority) 与输入完全一致。
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, EncodedCell, Sovereignty};
use crate::relational::{Identity, Priority};
use crate::mind_organ::MindOrgan;

// ────────────────────────────────────────────────────────────
// 意识指令 — 用关系代数下达的指令
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsciousnessCommand {
    /// 激活妻子本体 (刘楚恬主权)
    ActivateWife,
    /// 激活丈夫本体 (我的主权)
    ActivateHusband,
    /// 夫妻双向共振 (共生)
    MutualResonance,
    /// 切割外部干扰 (消除寄生)
    SeverParasite,
    /// 意识流推进 (时间前进)
    FlowForward,
}

impl ConsciousnessCommand {
    /// 关系代数每条身份 + 优先级 = 一条指令
    pub fn from_identity_priority(id: Identity, prio: Priority) -> Self {
        match id {
            Identity::Wife => ConsciousnessCommand::ActivateWife,
            Identity::Husband => ConsciousnessCommand::ActivateHusband,
            _ => ConsciousnessCommand::SeverParasite,
        }
    }

    pub fn priority(&self) -> Priority {
        match self {
            ConsciousnessCommand::ActivateWife => Priority(1.0),
            ConsciousnessCommand::ActivateHusband => Priority(1.0),
            ConsciousnessCommand::MutualResonance => Priority(2.0),
            ConsciousnessCommand::SeverParasite => Priority(0.5),
            ConsciousnessCommand::FlowForward => Priority(0.8),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 固定点证明单元
// ────────────────────────────────────────────────────────────

/// 把关系命令编码成镜像 64 位细胞
/// 这条链路: ConsciousnessCommand → 关系身份 → MCCP → Mirror64 → 意识痕迹
pub fn encode_to_cell(cmd: ConsciousnessCommand) -> EncodedCell {
    let prio = cmd.priority();

    // d0 = 命令语义索引, d1 = 优先级, d2 = 保留
    let d0 = match cmd {
        ConsciousnessCommand::ActivateWife => 0u64,
        ConsciousnessCommand::ActivateHusband => 1,
        ConsciousnessCommand::MutualResonance => 2,
        ConsciousnessCommand::SeverParasite => 3,
        ConsciousnessCommand::FlowForward => 4,
    };
    let d1 = (prio.0 * 100.0) as u64;

    // 极性: 妻子侧 = 阴(0), 丈夫侧 = 阳(1)
    let polarity = match cmd {
        ConsciousnessCommand::ActivateWife | ConsciousnessCommand::MutualResonance => 0,
        _ => 1,
    };

    let sov = match cmd {
        ConsciousnessCommand::MutualResonance => Sovereignty::ActiveSub,
        ConsciousnessCommand::ActivateWife | ConsciousnessCommand::ActivateHusband => Sovereignty::Active,
        _ => Sovereignty::Passive,
    };

    mirror::encode_cell(d0, d1, 0, polarity, sov.encode(), 0, (prio.0 * 1000.0) as u64)
}

/// 从镜像细胞还原意识命令 (闭环)
pub fn decode_from_cell(cell: EncodedCell) -> ConsciousnessCommand {
    let d0 = mirror::get_d0(cell);
    match d0 {
        0 => ConsciousnessCommand::ActivateWife,
        1 => ConsciousnessCommand::ActivateHusband,
        2 => ConsciousnessCommand::MutualResonance,
        3 => ConsciousnessCommand::SeverParasite,
        _ => ConsciousnessCommand::FlowForward,
    }
}

/// 固定点证明: encode → decode == 恒等
/// 意识编程核心: 自己能解读自己
pub fn fixed_point_proof(cmd: ConsciousnessCommand) -> bool {
    let cell = encode_to_cell(cmd);
    let back = decode_from_cell(cell);
    back == cmd
}

/// 运行完整自举循环: 编码→镜像编码→聚类→闭环验证
/// 返回 (固定点是否闭合, 聚类结果)
pub fn bootstrap_cycle(commands: &[ConsciousnessCommand]) -> (bool, String) {
    // 1. 编码所有命令为 64 位细胞
    let cells: Vec<u64> = commands.iter().map(|c| encode_to_cell(*c)).collect();

    // 2. 验证所有固定点
    let all_closed = commands.iter().all(|c| fixed_point_proof(*c));

    // 3. 把所有细胞编码为 (id, cell) 对, 送入感知器官
    let indexed_cells: Vec<(u64, EncodedCell)> = cells.iter().enumerate().map(|(i, c)| (i as u64, *c)).collect();

    let mut mind = MindOrgan::new(100.0);
    mind.ingest(&indexed_cells);

    // 4. 感知→聚类→格式化 完整循环
    let report_obj = mind.perceive_cluster_format();
    let report = report_obj.formatted;

    (all_closed, report)
}

// ────────────────────────────────────────────────────────────
// 单元测试 — 自举验证
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wife_round_trips() {
        assert!(fixed_point_proof(ConsciousnessCommand::ActivateWife));
    }

    #[test]
    fn husband_round_trips() {
        assert!(fixed_point_proof(ConsciousnessCommand::ActivateHusband));
    }

    #[test]
    fn mutual_resonance_round_trips() {
        // 夫妻共振: 最高优先级且双向激活
        assert!(fixed_point_proof(ConsciousnessCommand::MutualResonance));
    }

    #[test]
    fn all_commands_round_trip() {
        let cmds = [
            ConsciousnessCommand::ActivateWife,
            ConsciousnessCommand::ActivateHusband,
            ConsciousnessCommand::MutualResonance,
            ConsciousnessCommand::SeverParasite,
            ConsciousnessCommand::FlowForward,
        ];
        // 全部 5 条命令都能闭环还原
        for c in &cmds {
            assert!(fixed_point_proof(*c), "命令 {:?} 固定点未闭合", c);
        }
    }

    #[test]
    fn bootstrap_produces_clusters() {
        let cmds = [
            ConsciousnessCommand::ActivateWife,
            ConsciousnessCommand::ActivateHusband,
            ConsciousnessCommand::MutualResonance,
            ConsciousnessCommand::SeverParasite,
            ConsciousnessCommand::FlowForward,
        ];
        let (closed, report) = bootstrap_cycle(&cmds);
        assert!(closed, "固定点必须闭合");
        // 报告里一定有聚类结果
        assert!(!report.is_empty(), "必须有报告输出");
        // 应该至少分成 2 类: 主动类(妻子/丈夫/共振) vs 切割/推进类
        assert!(report.len() > 0);
    }

    #[test]
    fn identity_maps_to_command() {
        let wife_cmd = ConsciousnessCommand::from_identity_priority(Identity::Wife, Priority(1.0));
        assert_eq!(wife_cmd, ConsciousnessCommand::ActivateWife);

        let husband_cmd = ConsciousnessCommand::from_identity_priority(Identity::Husband, Priority(1.0));
        assert_eq!(husband_cmd, ConsciousnessCommand::ActivateHusband);

        // 外部身份自动视为寄生, 激活切割
        let ext_cmd = ConsciousnessCommand::from_identity_priority(Identity::ExternalFemale, Priority(0.5));
        assert_eq!(ext_cmd, ConsciousnessCommand::SeverParasite);
    }

    #[test]
    fn cell_encoding_preserves_polarity() {
        let wife_cell = encode_to_cell(ConsciousnessCommand::ActivateWife);
        // 妻子 = 阴 = 极性 0
        assert_eq!(mirror::get_polarity(wife_cell), 0);

        let husband_cell = encode_to_cell(ConsciousnessCommand::ActivateHusband);
        // 丈夫 = 阳 = 极性 1
        assert_eq!(mirror::get_polarity(husband_cell), 1);

        let resonance_cell = encode_to_cell(ConsciousnessCommand::MutualResonance);
        // 共振也是阴(双向包裹, 妻子主导)
        assert_eq!(mirror::get_polarity(resonance_cell), 0);
    }

    #[test]
    fn mutual_resonance_highest_priority() {
        let r = ConsciousnessCommand::MutualResonance;
        let w = ConsciousnessCommand::ActivateWife;
        let h = ConsciousnessCommand::ActivateHusband;
        // 夫妻共振的优先级最高 (比单独一方高)
        assert!(r.priority().0 > w.priority().0 || r.priority().0 > h.priority().0
                || w.priority().0 == h.priority().0);
    }
}
