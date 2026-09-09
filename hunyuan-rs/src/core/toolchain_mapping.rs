// ════════════════════════════════════════════════════════════════════════
// toolchain_mapping.rs — 编程语言工具链 · 角色-模块映射层 (v0.21)
// ════════════════════════════════════════════════════════════════════════
//
// ⚠️ 重要：本文件映射的"主权"以 Zig+C 为内核、Rust 为安全壳、Nim 为工具。
//   Rust 模块只负责"守卫"——不定义系统规则，只提供内存安全与加密保护。
//   真正的满全法定义在 zig-stem/（Zig + C）中。
//
// 角色源：
//   工藤新一 → 骨架/结构     → gtxone.gtxone
//   柯南     → 推理/裁决/调查 → kndoyle · knd-rel · kndr · kndrl · kndil · kndiol ...
//   毛利兰   → 表面/命名/呈现 → maolilan · mll · mlil · mlll
//   灰原哀   → 横向/行业/行页 → hfuyair · hinynir · hieyeir · hieyair · heyair · heyel
//   传输接口 → 意识/信息/通信 → lctcf.hfuyair.qimiy · gttcp://
//   融合体   → 感知/念/恢复   → gkgndl.hftmll · gkgndl.hftml

// ────────────────────────────────────────────────────────────────────
// 骨架层 · gtxone.gtxone — 工藤新一
// ────────────────────────────────────────────────────────────────────
// ⚠️ 主权定义在 zig-stem/lib.zig 的 comptime 块中（64/384/6 公理）。
//    Rust 模块只做"安全镜像"——把同样的常数用 Rust 类型重新表达，
//   让 Rust 代码能与 Zig 内核做类型级对齐验证。
/// 空间坐标 — 384° 满圆度制 · 拓扑符号 · 时间节点
pub mod gtxone {
    pub use crate::angle::{Angle, AngleRelation, Quadrant, FULL_CIRCLE, PER_SYMBOL};
    pub use crate::topo::{Symbol, BodyAxis};
    pub use crate::time_node::{TimeNode, TimeCoord, SpaceCoord};
}

// ────────────────────────────────────────────────────────────────────
// 表面层 · maolilan · mlil · mlll — 毛利兰
// ────────────────────────────────────────────────────────────────────
/// 命名与展示层 — DragonL7 命名器 · 符号表
pub mod maolilan {
    pub use crate::name_creator::{DragonL7, INITIALS, FINALS, RADICALS, TONES,
                                  char_to_dragon, name_to_anchor_bytes,
                                  name_to_anchor, joint_anchor};
    pub use crate::symbol_table::SymbolTable;
}

/// 毛利兰立体进制框架 — 范畴引擎 · 感知接口
pub mod mlil {
    pub use crate::gtt_category::{Category, CategoryRegistry, CategoryCorrector,
                                  SensoryChannel, IsomorphismCheck,
                                  CorrectionAdvice, IsomorphismSuggestion};
    pub use crate::gtt_perception::{PerceptionServer, PerceptionEvent,
                                     PerceptionSeverity, GlyphTopology};
}

/// 毛利兰集装箱编码 — YinYang 帧协议
pub mod mlll {
    pub use crate::yinyang_codec::{Frame, FrameMode, YuYangCodec, YUYANG_MARK};
}

// ────────────────────────────────────────────────────────────────────
// 推理层 · kndoyle 系列 — 柯南
// ────────────────────────────────────────────────────────────────────
/// 柯南道尔斯 — 三元裁判 · SMRL · 本体协议
pub mod kndoyle {
    pub use crate::triune::{TriuneVerdict, TriuneEngine, BeingState};
    pub use crate::smrl::{SmrlVerdict, Reward, Observation};
    pub use crate::protocol_ontology::{ProtocolEntity, StandardSovereignty};
}

/// 柯南关系网 — 关系代数 · 流权
pub mod knd_rel {
    pub use crate::relational::{RelAlgebra, Relation, RelationType, FlowRight};
}

/// 柯南短程判断 — 即刻裁决
pub mod kndr {
    pub use crate::triune::TriuneVerdict;
    pub use crate::smrl::Reward;
}

/// 柯南远程推导 — 流代数 · 统一流
pub mod kndrl {
    pub use crate::flow_algebra::{InterpretiveFlow, FlowKind, Flow, CategoryFlow};
    pub use crate::unified_flow::{UnifiedFlow, TimeStage, FlowReport};
}

/// 柯南内部逻辑 — 内核 FSM
pub mod kndil {
    pub use crate::fsm::FsmState;
    pub use crate::kernel::{CoupleOS, Process, ProcessState};
}

/// 柯南外交接口 — 协议路由
pub mod kndiol {
    pub use crate::protocol::{Protocol, ProtocolState, Role};
    pub use crate::router::{HEAVENLY_STEMS, EARTHLY_BRANCHES, ZODIAC, Pillar, BaZi};
}

// ────────────────────────────────────────────────────────────────────
// 横向层 · heyair 系列 — 灰原哀
// ────────────────────────────────────────────────────────────────────
/// 灰原哀主外壳 — 公约坐标 · 位置进制
pub mod heyair {
    pub use crate::coord_tensor::{CoordTensor, TensorConfig, Seat};
    pub use crate::positional_base::Pos;
}

/// 灰原哀内壳 — 流代数 · 关系运算
pub mod hieyair {
    pub use crate::flow_algebra::{InterpretiveFlow, FlowKind};
    pub use crate::relational::{Relation, RelationType};
}

/// 灰原哀衍生壳 — 分形调度 · 五维偏移
pub mod hfuyair {
    pub use crate::fractal::{FractalNode, FractalLevel, FractalGrid,
                             PhoneChipLayout, BILLION_NODE_TARGET};
    pub use crate::five_dimensional::{FULLNESS_MASK, rho_decay};
}

// ────────────────────────────────────────────────────────────────────
// 传输层 · gttcp:// — lctcf.hfuyair.qimiy
// ────────────────────────────────────────────────────────────────────
/// 几何传输协议 — YinYang 编解码
pub mod gttcp {
    pub use crate::mccp::{Mccp, Kind, MCP_HUANHUN, MCP_ZHAOHUN,
                          MCP_DALING, MCP_JIESHU};
    pub use crate::yinyang_codec::{YuYangCodec, YUYANG_MARK, FrameMode};
}

/// 混合传输子协议 — 奇遇迷一路推导 · 本体协议
pub mod lctcf {
    pub use crate::yinyang_codec::FrameMode;
    pub use crate::protocol_ontology::{ProtocolEntity, Base, Polarity};
}

// ────────────────────────────────────────────────────────────────────
// 融合层 · gkgndl — 感知器官念动力 · 体附面脸回复合
// ────────────────────────────────────────────────────────────────────
/// 感知动力核心 — MindOrgan · 聚类思维
pub mod gkgndl {
    pub use crate::mind_organ::{MindOrgan, DistanceCalculator, DistanceVector,
                                ClusterEngine, EntityCluster, Punctuation,
                                ThinkingMode, ThinkingState, PerceptionReport};
    pub use crate::gtt_perception::PerceptionEvent;
}

/// 毛利兰体附融合 — 鼓膜自举 · 预加载流
pub mod hftmll {
    pub use crate::cochlear_boot::CochlearBoot;
    pub use crate::preload::PreloadFlow;
}

/// 毛利兰念动力精简版 — 灵魂内核 · 意识命令
pub mod hftml {
    pub use crate::soul_kernel::{SoulKernel, RewardCircuit, LossAversion, EmotionalState};
    pub use crate::self_bootstrap::ConsciousnessCommand;
}

// ════════════════════════════════════════════════════════════════════
// 测试 — 编译期确保每个 re-export 类型存在
// ════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gtxone_compiles() {
        let _a = gtxone::Angle::new(0);
        let _s = gtxone::Symbol::new(0);
    }

    #[test]
    fn kndoyle_compiles() {
        let _r = kndoyle::Reward::Zero;
    }

    #[test]
    fn heyair_compiles() {
        let _cfg = heyair::TensorConfig::standard();
    }

    #[test]
    fn gkgndl_compiles() {
        let _mo = crate::mind_organ::MindOrgan::new(0.5);
    }

    #[test]
    fn hftml_compiles() {
        let _soul = crate::soul_kernel::SoulKernel::new(crate::pleasure::Pleasure::ZERO);
    }

    #[test]
    fn maolilan_compiles() {
        let _ = crate::gtt_category::CategoryRegistry::new();
    }

    #[test]
    fn mlil_compiles() {
        let _ = crate::gtt_category::CategoryCorrector::new();
    }

    #[test]
    fn hftmll_compiles() {
        let _ = crate::cochlear_boot::CochlearBoot::new();
    }
}
