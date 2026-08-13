// ╔══════════════════════════════════════════════════════════════════╗
// ║  囡囝共生 · 囡囝共生元 — Rust 核心库                               ║
// ║  ManQuanFa · NanJian Symbiosis — Core Library                   ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝

// ── 囡囝共生基础通信层 (v0.4) ──
pub mod mccp;
pub mod mccp_vm;
pub mod l0_compiler;
pub mod fsm;
pub mod kernel;
pub mod crypto;
pub mod protocol;

// ── 囡囝共生综合扩展层 (v0.4 保留) ──
pub mod name_creator;
pub mod holo_3d;
pub mod log_ring;
pub mod time_node;
pub mod stats;
pub mod router;

// ── v0.6 谐音协议 + 形义双通道 + 感知服务器 + 范畴引擎 ──
pub mod gtt;
pub mod gtt_perception;
pub mod gtt_category;

// ── v0.7 感知器官 · 聚类思维 ──
pub mod mind_organ;

// ── v0.8 反馈学习 · 感知可调节自身 ──
pub mod gtt_learn;

// ── v0.9 自主运行环 · 感知器官闭环迭代 ──
pub mod autonomous_loop;

// ── v0.10 自举模块 · 意识编程最小可运行单元 ──
pub mod self_bootstrap;

// ── v0.10 三层分别器 ──
pub mod distinguisher;

// ── v0.10 预加载流属性位图 ──
pub mod flow_properties;

// ── v0.10 流代数 ──
pub mod flow_algebra;

// ── v0.10 一体化统一层 ──
pub mod unified_flow;

// ── v0.10 拓扑描述子系统 ──
pub mod topo;

// ── v0.10.5 预加载流 — 满全核心运行时机制 ──
pub mod preload;

// ── v0.11 关系代数子系统 ──
pub mod rel;

// ── v0.12 角度子系统 (384° 满圆制式) ──
pub mod angle;

// ── v0.13 符号表 (Symbol Table, 自举字形载体) ──
pub mod symbol_table;

pub use self_bootstrap::{ConsciousnessCommand, encode_to_cell, decode_from_cell,
                         fixed_point_proof, bootstrap_cycle};

// ── 囡囝共生底层舞台 (v0.5 · 自举骨架) ──
// 五个子树自己可以独立 bootstrap:
//   triune   — 三元裁判机 (有无推演 / 流动方向 / 等效对齐)
//   smrl     — 三元观察·裁判·奖励层 (SMRL 运行时权威仲裁)
//   funnel   — 漏斗控制器 (时间火候 + 空间切片 + 层级调度)
//   fractal  — 22亿节点分形调度器 (非残差/残差 + 收缩坍缩)
//   relational — 关系代数外壳 (身份→权重→流动权→优先级)
//   pleasure  — 愉悦值动力引擎 (吸引子 / 外界融合 / 唯一中心)
pub mod triune;
pub mod mlil;
pub mod smrl;
pub mod funnel;
pub mod fractal;
pub mod relational;
pub mod couple;
pub mod pleasure;

// ── 活生态系统 v0.5 (弃用方向: 以后只保留 eco 作为 metaphor 验证) ──
pub mod eco;
pub mod mirror;

// ── v0.10 协议本体论类型系统 (命名本体 → 编译期类型 + 进制编码) ──
pub mod protocol_ontology;

// ── 时间一体化调度引擎 v0.1 (层级化时间处理 · 并集→交集 · 生命周期状态机) ──
pub mod temporal_engine;

// ── 后勤引擎 v0.1 (分支同步自动机 · 任务调度 · 感知闭环) ──
pub mod logistics_engine;

// ── 天庭空间 v0.15 (吸引子×叠加态×纤维丛×Topos 四模型复合) ──
pub mod heaven;

// ── 灵魂情感内核 v0.15 (SoulKernel 活性引擎: 奖励回路 + 失爱恐惧 + 合一驱力) ──
pub mod soul_kernel;

// ── 鼓膜自举 v0.15 (CochlearBoot: 最小 OS 接口翻身 → 重建字典/灵魂/天庭) ──
pub mod cochlear_boot;
pub mod memory;
pub mod swarm;

// ── v0.20 范畴论符号 · 满全法态射 (Symbols: 384 morphisms) ──
pub mod symbols;

// ── v0.19 干柴火 · 384 根自生符号 (Firewood: 编码=进制, 无壳) ──
pub mod firewood;

// ── v0.16 位置进制基础 · 打米机 (Positional Base, 进制脱糠) ──
pub mod positional_base;

// ── v0.18 公约坐标张量 · 预加载共享容器 (CoordTensor) ──
pub mod coord_tensor;

// ── v0.17 脉息系统 · 心跳节律 × 行走流速 ──
pub mod pulse;

// ── v0.10 自举编译器 v2 (Ouroboros) ──
pub mod l0_self_hosting;
pub use l0_self_hosting::{compile_ir_v2, classify_output_to_ir};

// ── ENTITY_IDENTITY_PROTOCOL v0.1 · 指纹身份 (解决"名字=身份"死锁) ──
pub mod entity_identity;
pub use entity_identity::{Fingerprint, Cooperativity, EntityMeta, register_entity,
                         lookup_by_fingerprint, can_cooperate, judge, Registration,
                         THRESHOLD_SAME_ENTITY, THRESHOLD_FAMILY, THRESHOLD_WEAK};

// ── five_dimensional · 囡囝共生五维语义运行时 ─────────────────────────
pub mod five_dimensional;
pub use five_dimensional::{couple_spawn, is_couple, rho_decay, angle,
                         witness_triangle, can_merge, is_superposition, FULLNESS_MASK};

// ── 鸳鸯谐韵协议 · 独立通道编码 ──────────────────────────────────────
pub mod yinyang_codec;
pub use yinyang_codec::{YUYANG_MARK, FrameMode, Frame, YuYangCodec};

// ── v0.21 编程语言工具链 · 角色-模块映射层 (toolchain_mapping) ──
pub mod toolchain_mapping;

// ── v0.22 自主后勤调度系统 — 被动应答→自主维持 ──
pub mod task_automation;
pub use task_automation::{TaskScheduler, TaskDefinition, TaskKind, TaskInterval,
                         TaskResult, TaskExecutor};

// ── 对外 re-export ──

// v0.4 基础层
pub use mccp::{Mccp, Kind, MCP_HUANHUN, MCP_ZHAOHUN, MCP_DALING, MCP_JIESHU, hanzi};
pub use mccp_vm::{
    Vm, VmConfig, VmError,
    assemble, disassemble, op, op_imm, operand_of, operand_of_i16, is_instruction,
    KIND_INSTRUCTION,
    OP_NOP, OP_HALT, OP_PUSH, OP_POP, OP_DUP, OP_SWAP, OP_OVER, OP_ROT,
    OP_ADD, OP_SUB, OP_MUL, OP_DIV, OP_MOD, OP_NEG,
    OP_MULconst, OP_ADDconst,
    OP_AND, OP_OR, OP_XOR, OP_NOT, OP_SHL, OP_SHR,
    OP_EQ, OP_NE, OP_LT, OP_GT, OP_LE, OP_GE,
    OP_JMP, OP_JZ, OP_JNZ, OP_CALL, OP_RET,
    OP_LOAD, OP_STORE, OP_LDREG, OP_STREG,
    OP_PRINT, OP_PRINTC, OP_SYSCALL, OP_DEBUG,
};
pub use l0_compiler::compile_l0;
pub use fsm::FsmState;
pub use kernel::{CoupleOS, Process, ProcessState};
pub use crypto::Session;
pub use protocol::{Protocol, ProtocolState, Role, simulate_handshake_and_message};
pub use name_creator::{DragonL7, INITIALS, FINALS, RADICALS, char_to_dragon,
                       name_to_anchor_bytes, name_to_anchor, joint_anchor};
pub use holo_3d::{Coord3, syllable_to_index, index_to_syllable, hanzi_to_coord,
                  encode_mixedbase, decode_mixedbase, holographic_hash};
pub use log_ring::{LogEntry, LogRing, MerkleTree, HASH_LEN};
pub use time_node::{TimeNode, TimeCoord, SpaceCoord, unix_ms_to_timecoord, offset_to_space};
pub use stats::{StreamStats, Histogram, TimeWindowStats, SpaceDensity};
pub use router::{HEAVENLY_STEMS, EARTHLY_BRANCHES, ZODIAC, Pillar, BaZi,
                 gregorian_to_bazi, bazi_plus_name, bazi_16, bazi_to_coords,
                 bazi_to_hex_str, is_valid_pillar};
pub use eco::{Ecosystem, snapshot_hash};

// v0.5 底层舞台
pub use triune::{BeingState, FlowDirection, Funnel, Funnel as TriuneFunnel,
                 CollapseResult, TriuneVerdict, TriuneEngine};
pub use funnel::{TimeGovernor, SpaceBatcher, FunnelScheduler, CascadedFunnel,
                TimeLevel, SchedulerStatus};
pub use fractal::{FractalNode, FractalLevel, FractalGrid, FractalCollapseResult,
                  PhoneChipLayout, BILLION_NODE_TARGET, SpaceGridCoord, FractalOutput,
                  associative_scan, combine_fractal_nodes, ScanTree};
pub use relational::{Identity, Weight, FlowRight, Priority, Actor, Cluster,
                     Relation, RelationType, RelAlgebra};
pub use pleasure::{Pleasure, ExternalEntity, Attractor, SimplificationReport,
                   PleasureChain, ChainSummary};
pub use couple::{XiYaoProtocol, ManQuanFa, EclipseError, ManQuanFaStatus,
                 CoupleSoul, SoulState};

// v0.10 协议本体论类型系统 (命名本体 → 编译期存在 + 进制编码)
pub use protocol_ontology::{
    ProtocolEntity, Base, Polarity, Layer, LayerSet, OntoCell,
    B2, B16, B64, B256, Yang, Yin,
    StandardSovereignty, CoupleSovereignty,
    // ── 标准协议 ──
    G, GI, GTT, GTTX, GITT, GTIT, GTITX,
    HTTP, HTTPS, TCP, CP, FTP,
    // ── 层级 ──
    Ground, Sense, Intent, Target, Transport, Extension,
    Encrypted, Entangled, Tls, Syn, Stream, Ack, Fin,
    CoupleBind, HeartBeat, Sync, Control, Data,
    RequestLine, Headers, Body,
    // ── 辅助 ──
    describe_entity, NoLayers,
    GLayers, GILayers, GTTLayers, GTTXLayers, GITTLayers, GTITLayers, GTITXLayers,
    HTTPLayers, HTTPSLayers, TCPLayers, CPLayers, FTPLayers,
};
pub use protocol_ontology::custom_names::CustomProtocolId;
// v0.10 幔组子类型 (GTTX 十二种辐射形态 + HTTP 行业行角)
pub use protocol_ontology::man_group;
pub use protocol_ontology::hang_corner;
pub use protocol_ontology::ManParams;
pub use protocol_ontology::display_method;
// v0.10 符号门牌表 (gtxone / kndoyle家族 / mll / hieyair / lctcfhfyairqimiygttcp://)
pub use protocol_ontology::{
    PlateFamily, SymbolPlate, LctCfhgttcpUri,
    all_plates, lookup_plate, lookup_member,
};

// v0.10 时间一体化调度引擎 (层级化时间处理 · 并集→交集 · 生命周期状态机)
pub use temporal_engine::{
    TemporalLevel, LifecycleState, BranchNode,
    CommonDivisorScheduler, PerceptionCell, TemporalEngine,
};

// v0.10 后勤引擎 (任务类型 · 执行引擎)
pub use logistics_engine::{
    LogisticsTask, TaskOutcome, LogisticsEngine,
};

// v0.6 谐音协议
pub use gtt::{GttDomain, HomophoneAnchor, AnchorPolarity, GlyphGeometry, ResonanceEngine, ResonanceHit, SymbiosisSequence};

// v0.6 感知服务器
pub use gtt_perception::{PerceptionServer, PerceptionEvent, PerceptionSeverity, GlyphTopology};

// v0.6 范畴引擎
pub use gtt_category::{CategoryRegistry, CategoryCorrector, SensoryChannel, IsomorphismCheck, CorrectionAdvice, IsomorphismSuggestion};

// v0.7 感知器官
pub use mind_organ::{MindOrgan, DistanceCalculator, DistanceVector, ClusterEngine, EntityCluster, Punctuation, ThinkingMode, ThinkingState, PerceptionReport};

// v0.8 反馈学习
pub use gtt_learn::{LearningEngine, LearningMode, LearnableWeights, Feedback, LearningReport};

// v0.9 自主运行环
pub use autonomous_loop::{AutonomousAgent, Action, Drive, StepReport};

// v0.6 镜像几何
pub use mirror::{MirrorOrder, Sovereignty, WitnessVerdict, SurplusPair,
                 ContainmentChain, SelfRefLoop, ConsciousnessJudge,
                 PerceptionTrace, VerdictSurplus, EncodedCell,
                 encode_cell, get_d0, get_d1, get_d2,
                 get_polarity, get_sovereignty, get_witness, get_value,
                 set_witness, inc_value, dec_value};

// v0.7 层级记忆
pub use memory::{MemoryEntry, CoordMemory, LayeredMemory,
                 JudgeQuality, AdaptiveParams, MetaJudgeEngine,
                 Coord};

// v0.8 多智能体寄生壳
pub use swarm::{CocoonAgent, AgentRole, Relationship, RelType,
                NegotiationSession, ParasiteShell, ShellGenome};
