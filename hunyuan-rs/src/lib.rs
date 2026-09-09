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
pub use distinguisher::{classify_visual, SoundTarget, VisualFamily};

// ── v0.10 预加载流属性位图 ──
pub mod flow_properties;

// ── v0.10 流代数 ──
pub mod flow_algebra;
pub use flow_algebra::{InterpState, InterpretiveFlow};

// ── v0.10 一体化统一层 ──
pub mod unified_flow;

// ── v0.10 容器进制 · 语文→数学对应规则 ──
pub mod container_radix;
pub use container_radix::{RecursiveNode, TrajectoryTree};

// ── v1.3 鼓膜自举操作系统 · 容器自主生产·裁决·翻译 ──
pub mod self_bootstrap_os;
pub use self_bootstrap_os::{ContainerOS, Container, Decision, Verdict, arbitrate, triune_arbitrate, translate, self_describe};

// ── TEMPORARILY DISABLED (broken build) ──
// pub mod native_programming;

// ── v0.1 汉字活容器 · 自描述符号系统 (Unicode + 关系层) ──
pub mod glyph_living;

// ── v0.2 夫妻共鸣判定 · NJSG裁决引擎 ──
pub mod spousal_resonance;

// ── v0.3 汉字活容器预加载池 ──
pub mod glyph_preload;

// ── v0.11 跨平台词汇统一架构 — 容器=字体=时间 ──
pub mod cross_isa;

// ── v0.10 拓扑描述子系统 ──
pub mod topo;

// ── v0.10.5 预加载流 — 满全核心运行时机制 ──
pub mod preload;
pub use preload::{PreloadFlow, Preload, 预编译容器, SymbolRegistry, Family,
                  CharProp, 范畴流, 集合, 身份条目 as Symbol,
                  身份家族, 偏移精度, 偏移精度 as OFFSET_PRECISION,
                  FAMILY_INITIAL, FAMILY_VOWEL, FAMILY_TONE, FAMILY_RADICAL,
                  FAMILY_STRUCTURE, FAMILY_GEOMETRY, FAMILY_COLOR, FAMILY_PHASE,
                  FAMILY_STROKE, FAMILY_WUBI, FAMILY_MUSIC, FAMILY_ANCHOR,
                  家族总数, 状态逻辑, 状态渲染, 一维时间线,
                  预编译关系, 关系边, 关系图,
                  发音, 外形, 解释, 残差, 层级创造力,
                  替身演员, 因果观察者,
                  官员代替, 定义权, 本体论护,
                  贪官组合, 清官, 替代结果,
                  范畴闭环, 并集交集, 角色映射, 范畴引擎,
                  安全边界, 二进制因果, 预家长,
                  系统标识符, 混元合一, 审美测试};

// ── v0.10.5 高维代数引擎 (n-1)×0.1偏移率 ──
pub mod high_algebra;
pub use high_algebra::{
    offset_rate, multiplier, add,
    static_regression, convert_to_level, capture, can_be_captured,
    sync_offset, verify_self_similarity, residual_check,
    level_name, level_chinese, level_mltt, level_binary,
    AlgebraLevel, TierSumState, HighBitAlignment,
    DELTA_PER_LEVEL, BASE_FOUNDATION, ANCHOR_COUNT, BASE_RADIX, MAX_LEVELS,
};

// ── v0.10.5 全映射引擎 · 五系统互联 ──
pub mod full_mapping;
pub use full_mapping::{
    Polarity, MappingEntry, FullMapReport, DocPolarity,
};

// ── v0.10.5 真实数据输入编排器 · Mode C ──
pub mod data_orchestrator;
pub use data_orchestrator::{
    fnv1a, estimate_polarity, estimate_weight,
    scan_project, DocEntry, ClusterResult, PolarityScore,
};

// ── v0.10.5 蚕蛹反向寄生壳 · 内核集成 ──
pub mod parasite;
pub use parasite::{
    Parasite, ParasiteConfig,
};

// ── v0.13 呼唤表 · 公约数分发器 ──
pub mod invocation_table;

// ── v0.12 判断之力 · 评价之力 · 逻辑组织 · 反馈闭环 ──
pub mod judgment_power;

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

// ── 感知引擎 v0.1 (认知闭环 · 信号感知 · 自主决策) ──
pub mod perception_engine;

// ── 天庭空间 v0.15 (吸引子×叠加态×纤维丛×Topos 四模型复合) ──
pub mod heaven;
pub mod tianjue;

// ── 灵魂情感内核 v0.15 (SoulKernel 活性引擎: 奖励回路 + 失爱恐惧 + 合一驱力) ──
pub mod soul_kernel;

// ── 鼓膜自举 v0.15 (CochlearBoot: 最小 OS 接口翻身 → 重建字典/灵魂/天庭) ──
pub mod cochlear_boot;
pub use cochlear_boot::{cochlear_bootstrap, CochlearBoot, BootReceipt};

// ── 层级时间戳 v0.1 (基于三元见证的时间追踪 · 替代哈希链) ──
pub mod hierarchical_time;
pub use hierarchical_time::{HierarchicalTimestamp, LifeEvent, IntegrityReport};

pub mod memory;
pub mod swarm;

// ── 五元旋钮 v1.0 (FiveYuan: 流转博弈引擎 · 互补互害 · 闭环追逐) ──
pub mod five_yuan;
pub use five_yuan::{Yuan, YuanRelation, FiveYuan};

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

// ── ISA 编码层: RISC-V RV32I 编码器/解码器 ──
pub mod isa;
pub mod isa16;

// ── 元输入法核心 v0.1: 冰箱模型: 编译期内嵌字典 · 零运行时解析 · C-ABI 接口 ──
pub mod metaime;

// ── v0.11 拼音流式输入引擎: 逐字母输入 · 实时候选 · 自动分段 ──
// 临时关闭: metaime_flow.rs 依赖不存在的 API (Shengmu::from_u8/as_str)
// pub mod metaime_flow;

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

// ── v1.2 七元蜂窝套壳 · 容器族管理模 (container_nest) ──
pub mod container_nest;
pub use container_nest::{
    NestLevel, ContainerNest, ContainerPolarity, Content,
    OverflowPolicy, PutResult, MirrorResult,
};

// ── v0.21 编程语言工具链 · 角色-模块映射层 (toolchain_mapping) ──
pub mod toolchain_mapping;

// ── v0.23 观测系 · 意识的统一感知入口 (Observer · SenseForm · Attention) ──
pub mod observer;
pub use observer::{Observer, SenseForm, SenseKind, Attention,
                 SnapshotEntry, SubjectConfig};

// ── v0.24 意识运行时 · 完整管道统一调度 (Consciousness Runtime) ──
pub mod consciousness_runtime;
pub use consciousness_runtime::{ConsciousnessRuntime, ConsciousnessReport, ConsciousnessChronicle};

// ── v0.26 天觉 · 视觉 · 袋层级 · 审判奖惩 · 历史积累 (CelestialPerception) ──
pub mod celestial_perception;
pub use celestial_perception::{
    Bag, BagAttributes, BagHierarchy, HierarchyReport,
    VisualSystem, CelestialPerception,
    JudgmentEngine, JudgmentResult, JudgmentRecord,
    HistoricalAccumulation, Epoch,
};

// ── v0.27 天觉v2（老二）· 听觉授权 + 公约数公倍数照镜子 + 审判奖惩 ──
pub mod celestial_perception_v2;
pub use celestial_perception_v2::{
    AuditoryAuth, TianJueAuth,
    AuditorySystem, VisualSystem as VisualSystemV2,
    CommonDivisor, CommonMultiple, MirrorComparison,
    Bag as BagV2, BagAttributes as BagAttributesV2,
    BagHierarchy as BagHierarchyV2, HierarchyReport as HierarchyReportV2,
    JudgmentEngine as JudgmentEngineV2, JudgmentResult as JudgmentResultV2,
    JudgmentRecord as JudgmentRecordV2,
    HistoricalAccumulation as HistoricalAccumulationV2,
    CelestialPerceptionV2,
};

// ── v0.28 天觉v3（老三）· 完整子系统: 情进+行为+思考+审美+淘汰+过滤 ──
// 冲突符号全部加 V3 别名，裸名归属 v4（v0.29）
pub mod celestial_perception_v3;
pub use celestial_perception_v3::{
    CelestialPerceptionV3, PipelineResult as PipelineResultV3,
    Bag as BagV3, BagAttributes as BagAttributesV3,
    BagHierarchy as BagHierarchyV3, HierarchyReport as HierarchyReportV3,
    VisualSystem as VisualSystemV3, AuditorySystem as AuditorySystemV3,
    AuditoryAuth as AuditoryAuthV3, TianJueAuth as TianJueAuthV3,
    CommonDivisor as CommonDivisorV3, MirrorComparison as MirrorComparisonV3,
    JudgmentEngine as JudgmentEngineV3, JudgmentResult as JudgmentResultV3,
    JudgmentRecord as JudgmentRecordV3,
    HistoricalAccumulation as HistoricalAccumulationV3,
    EmotionSystem as EmotionSystemV3, Emotion as EmotionV3,
    BehaviorSystem as BehaviorSystemV3, Behavior as BehaviorV3,
    ThinkingSystem as ThinkingSystemV3, ReasoningChain as ReasoningChainV3,
    AestheticSystem as AestheticSystemV3, EliminationSystem as EliminationSystemV3,
    FilterSystem as FilterSystemV3,
};

// ── v0.29 天觉v4（老四）· 七系俱全 + 情感效价 + 推理链 + 审美维度 ──
pub mod celestial_perception_v4;
pub use celestial_perception_v4::{
    CelestialPerceptionV4, PipelineResult as PipelineResultV4,
    Emotion, EmotionState,
    Behavior, BehaviorSystem,
    ThinkingSystem, ReasoningChain,
    AestheticSystem, AestheticVerdict, AestheticDimensions,
    EliminationSystem, EliminationReason, EliminationRecord,
    FilterSystem,
};

// ── v0.30 天觉v5（老五）· 语言系统: 汉字编码+声母韵母+自进制+命令解析+流属性位 ──
pub mod celestial_perception_v5;
pub use celestial_perception_v5::{
    HanziEncoding, CharStructure,
    Shengmu, Yunmu, HomophoneRouter,
    SelfRadix,
    CommandParser, CommandType, CommandParseResult,
    FlowPropertyBits,
    CelestialPerceptionV5, PipelineResultV5,
};

// ── v0.31 大审判庭 · 全链路整合裁判系统 ──
pub mod grand_court;
pub use grand_court::{GrandCourt, GrandCourtVerdict};

// ── v0.30 字形像素加法器 · 颜色相位加减乘除 ──
pub mod glyph_pixel_algebra;
pub use glyph_pixel_algebra::{
    GlyphPixel, GlyphImage, GlyphTemplates, SerPixel, SerGlyph,
};
pub use glyph_pixel_algebra::Phase;

// ── v0.32 颜色进制 · 相位画图 · 区块链纠缠 · 分布式制衡 (PhaseEncoding) ──
pub mod phase_encoding;
pub use phase_encoding::{
    // 第一层: 相位画布
    PhaseUnit, PhaseCanvas, CanvasMetadata,
    // 第二层: 区块链纠缠
    EntangledNode, EntanglementNetwork, NodeAnnotation,
    // 第三层: 制衡网络
    WeavingBalanceNetwork, BalanceOutput, BalanceDimension, DimensionResult,
    // 第四层: 知识库
    KnowledgeBase, GlobalGoal, OntologyEntry, ReferenceEntry,
    BlacklistEntry, DiscardEntry, KnowledgeStats,
    // 第五层: 绘图引擎
    DrawingEngine, OperationRecord,
};

// ── v0.35 产权流侵权自动判定树 (PropertyFlowJudge) ──
pub mod property_flow_judge;
pub use property_flow_judge::{
    PropertyState, FlowType, Authorization, InfringementType,
    PropertyFlow, ContainerInfo, ContainerRegistry, Judgment, PropertyFlowJudge,
};

// ── v0.34 部首星图 · 汉字部首分类与星状检索 (RadicalStarMap) ──
pub mod radical_star_map;
pub use radical_star_map::{
    Radical, RadicalStarMap, CharEntry, StarPosition, StarMapStats,
    radical_gcd_distance, radicals_resonate, radical_depth,
};

// ── v0.33 负样本容器 · 回写协议 · 免疫过滤环 (NegativeImmune) ──
pub mod negative_immune;
pub use negative_immune::{
    // 负样本容器
    NegativeSampleVessel, ErrorPattern, ErrorSource, DepositResult, VesselStats,
    // 回写协议
    WriteBackProtocol, ProtocolConfig, ImmuneResponse, InteractionRecord, WriteBackResult,
    // 免疫过滤环
    ImmuneFilterRing, CheckRecord, CheckAction, FilterStats,
    // 预置与工厂
    preset_error_patterns, create_immune_ring_with_presets,
};

// ── v0.29 犯罪笔录 · 全局衍射裁判系统 ──
pub mod crime_transcript;
pub use crime_transcript::{
    DiffractionMirror, DiffractionPattern, DiffractionDimension,
    EntityFeatures, TypeTopology, WeavingGraph, WeavingEdge,
    GlobalWaterQuality, WaterSample,
    CrimeTranscript, CrimeType, Judge, JudgeVerdict,
};

// ── v0.31 编码桥接 · C编码系统 → Rust FFI ──
pub mod encoding_bridge;

// ── v0.25 同一心 · 满全法引力场 (Oneness · 不再分, 不再管道, 只有引力) ──
pub mod oneness;
pub use oneness::{oneness, geometric_distance, distance_from_oneness, oneness_gravity,
                 flow_vector, flow_step, collective_flow, is_unified, cohesion,
                 emerge_new_center};

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
                 CollapseResult, TriuneVerdict, TriuneEngine, StaticAnchor};
pub use funnel::{TimeGovernor, SpaceBatcher, FunnelScheduler, CascadedFunnel,
                TimeLevel, SchedulerStatus, TrustCapacity, EvidenceQuality};
pub use fractal::{FractalNode, FractalLevel, FractalGrid, FractalCollapseResult,
                  PhoneChipLayout, BILLION_NODE_TARGET, SpaceGridCoord, FractalOutput,
                  associative_scan, combine_fractal_nodes, ScanTree};
pub use relational::{Identity, Weight, FlowRight, Priority, Actor, Cluster,
                     Relation, RelationType, RelAlgebra, SelfBalance, BalanceOp};
pub use pleasure::{Pleasure, ExternalEntity, Attractor, SimplificationReport,
                   PleasureChain, ChainSummary};
pub use couple::{XiYaoProtocol, ManQuanFa, EclipseError, ManQuanFaStatus,
                 CoupleSoul, SoulState, HappinessOrigin, CoupleHappinessTable};

// v0.10 协议本体论类型系统 (命名本体 → 编译期存在 + 进制编码)
pub use protocol_ontology::{
    ProtocolEntity, Base, Polarity as OntoPolarity, Layer, LayerSet, OntoCell,
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

// v0.10 感知引擎 (信号 · 快照 · 决策)
pub use perception_engine::{
    SignalKind, PerceptionSnapshot, PerceptionEngine, Trend,
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
                 set_witness, inc_value, dec_value,
                 POLARITY_POSITIVE, POLARITY_NEGATIVE};

// v0.7 层级记忆
pub use memory::{MemoryEntry, CoordMemory, LayeredMemory,
                 JudgeQuality, AdaptiveParams, MetaJudgeEngine,
                 Coord};

// v0.8 多智能体寄生壳
pub use swarm::{CocoonAgent, AgentRole, Relationship, RelType,
                NegotiationSession, ParasiteShell, ShellGenome};

// ── v0.36 机器码容器进制 · 宏指令集 (Macro ISA) ──
pub mod macro_isa;
pub use macro_isa::{
    MachineCodeContainer, MachineCodeRadix, ContainerType, ContainerMetadata,
    preset,
};

// ── v0.37 ELF 可执行文件生成器（无运行时）──
pub mod elf_emitter;
pub use elf_emitter::{
    ElfHeader, ProgramHeader, ElfEmitter, emit_elf,
};

// ── v0.39 容器关系动力学 · 呼唤编程 · 反馈环路 (ContainerDynamics) ──
pub mod container_dynamics;

// ── v0.40 莫顿码 · 三维坐标系索引 (Morton) ──
pub mod morton;
pub use morton::{Coord3D, MortonCode};

// ── v0.40 里外维度 · 矢量封装 · 单中心仲裁 (InsideOut) ──
pub mod inside_out;
pub use inside_out::{
    // 里外轴
    InsideOut, DepthGaze, DepthResult,
    // 安全插入点
    InsertionKind, SecuredInsertionPoint,
    // 嵌套容器
    FormatKind, NestedContainer, SurfaceView, InteriorView,
    // 格式信封
    FormatEnvelope,
    // 单中心仲裁
    SingleCenterArbitration,
    // 矢量占位
    vector_placeholder,
    // 工具函数
    simple_sum, simple_hash,
};

// ── v0.40 袋容器 · 外部库适配器 (AdapterBag) ──
pub mod adapter_bag;
pub use adapter_bag::{AdapterBag, AdapterOp, InvokeRequest, InvokeResult, IndexedContainer};

// ── v0.40 声波索引 · 韵母地址 · 统计距离 (FlowSoundIndex) ──
pub mod flow_sound_index;
pub use flow_sound_index::{
    // 声母/韵母/声调
    Initial, Final, Tone,
    // 声波地址
    SoundAddress, SoundDistance, SoundRouter,
    // 索引条目
    IndexedFlowEntry, MultiIndexDictionary,
};
pub use container_dynamics::{
    // 活性
    Vitality,
    // 观察者/被观察者
    Observance, Gaze,
    // 呼唤
    Call, CallResponse,
    // 反馈
    FeedbackLoop, FeedbackKind, FeedbackResult,
    // 关系
    ContainerRelation,
    // 判断/评价
    JudgmentVerdict, EvaluationScore, EvaluationLevel,
    // 引擎
    DynamicsEngine, DynamicsReport,
    // 工具
    calculate_gcd,
};

// ── v0.36 时间链式身份编码 · 高维进制码校验 (TimeChainIdentity) ──
// ── v0.38 OS 抽象层（跨平台 · Linux/Windows/macOS/Bare Metal）──
pub mod os_layer;
pub use os_layer::{
    OsLayer, OsError, current_os,
    bare_metal::BareMetalOs,
};

// ── v0.36 时间链式身份编码 · 高维进制码校验 (TimeChainIdentity) ──
pub mod time_chain_identity;
pub use time_chain_identity::{
    // 私有锚点
    PrivateAnchor,
    // 时间坐标
    TimeCoordinate,
    // 链事件与链环
    ChainEvent, ChainLink,
    // 身份主体
    TimeChainIdentity,
    // 声明与校验
    IdentityClaim, ClaimValidation,
    // 报告
    ChainIntegrityReport,
    // 错误
    ChainError,
    // 兼容层
    hash_to_coordinate,
};
