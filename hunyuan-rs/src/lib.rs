// ╔══════════════════════════════════════════════════════════════════╗
// ║  囡囝共生 · 囡囝共生元 — Rust 核心库                               ║
// ║  ManQuanFa · NanJian Symbiosis — Core Library                   ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
// ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
// ╚══════════════════════════════════════════════════════════════════╝

// ── 子目录模块声明 ──
// NOTE: pub mod core; is removed — core submodules are declared individually
// with #[path = "core/xxx.rs"] attributes starting at line 19.
pub mod flow;
pub mod cognition;
pub mod judge;
pub mod relation;
pub mod temporal;
pub mod encoding;
pub mod stroke_database;

// ── 囡囝共生基础通信层 (v0.4) ──
#[path = "core/mccp.rs"]
pub mod mccp;
#[path = "core/mccp_vm.rs"]
pub mod mccp_vm;
#[path = "core/l0_compiler.rs"]
pub mod l0_compiler;
#[path = "core/fsm.rs"]
pub mod fsm;
#[path = "core/kernel.rs"]
pub mod kernel;
#[path = "core/crypto.rs"]
pub mod crypto;
#[path = "core/protocol.rs"]
pub mod protocol;

// ── 囡囝共生综合扩展层 (v0.4 保留) ──
#[path = "core/name_creator.rs"]
pub mod name_creator;
#[path = "core/holo_3d.rs"]
pub mod holo_3d;
#[path = "core/log_ring.rs"]
pub mod log_ring;
#[path = "core/time_node.rs"]
pub mod time_node;
#[path = "core/stats.rs"]
pub mod stats;
#[path = "core/router.rs"]
pub mod router;

// ── v0.6 谐音协议 + 形义双通道 + 感知服务器 + 范畴引擎 ──
#[path = "flow/gtt.rs"]
pub mod gtt;
#[path = "flow/gtt_perception.rs"]
pub mod gtt_perception;
#[path = "flow/gtt_category.rs"]
pub mod gtt_category;

// ── v0.7 感知器官 · 聚类思维 ──
#[path = "cognition/mind_organ.rs"]
pub mod mind_organ;

// ── v0.8 反馈学习 · 感知可调节自身 ──
#[path = "flow/gtt_learn.rs"]
pub mod gtt_learn;

// ── v0.9 自主运行环 · 感知器官闭环迭代 ──
#[path = "cognition/autonomous_loop.rs"]
pub mod autonomous_loop;

// ── v0.10 自举模块 · 意识编程最小可运行单元 ──
#[path = "core/self_bootstrap.rs"]
pub mod self_bootstrap;

// ── v0.10 三层分别器 ──
#[path = "judge/distinguisher.rs"]
pub mod distinguisher;
pub use distinguisher::{classify_visual, SoundTarget, VisualFamily};

// ── v0.10 预加载流属性位图 ──
#[path = "flow/flow_properties.rs"]
pub mod flow_properties;

// ── v0.10 流代数 ──
#[path = "flow/flow_algebra.rs"]
pub mod flow_algebra;
pub use flow_algebra::{InterpState, InterpretiveFlow};

// ── v0.10 一体化统一层 ──
#[path = "flow/unified_flow.rs"]
pub mod unified_flow;

// ── v0.10 容器进制 · 语文→数学对应规则 ──
#[path = "encoding/container_radix.rs"]
pub mod container_radix;
pub use container_radix::{RecursiveNode, TrajectoryTree};

// ── v0.45 容器串联 + 链路共振 ──
#[path = "encoding/container_chain.rs"]
pub mod container_chain;
pub use container_chain::{ContainerChain, ContainerCode, ChamberId, ContainerLink,
                         chamber_name, container_entry_addr, container_exit_addr,
                         verify_chain, LinkValidation, ChainValidation};
#[path = "encoding/chain_resonance.rs"]
pub mod chain_resonance;
pub use chain_resonance::{ResonanceGraph, ChamberGroup, ResonanceStats,
                          resonate_and_register, resonance_stats};

// ── 逗号容器 · 比特即容器标签符 ──
#[path = "comma_container.rs"]
pub mod comma_container;

// ── v1.3 鼓膜自举操作系统 · 容器自主生产·裁决·翻译 ──
#[path = "core/self_bootstrap_os.rs"]
pub mod self_bootstrap_os;
pub use self_bootstrap_os::{ContainerOS, Container, Decision, Verdict, arbitrate, triune_arbitrate, translate, self_describe};

// ── TEMPORARILY DISABLED (broken build) ──
// pub mod native_programming;

// ── v0.1 汉字活容器 · 自描述符号系统 (Unicode + 关系层) ──
#[path = "flow/glyph_living.rs"]
pub mod glyph_living;

// ── v0.2 夫妻共鸣判定 · knml☉l0l 裁决引擎 ──
#[path = "relation/spousal_resonance.rs"]
pub mod spousal_resonance;

// ── v0.3 汉字活容器预加载池 ──
#[path = "flow/glyph_preload.rs"]
pub mod glyph_preload;

// ── v0.4 动态汉字池 · LRU缓存+静态属性表 ──
#[path = "flow/dynamic_pool.rs"]
pub mod dynamic_pool;

// ── v0.11 跨平台词汇统一架构 — 容器=字体=时间 ──
pub mod cross_isa;
pub mod void_probe;

// ── v0.43 动作·时间·容器 三维索引集 (Action·Stage·Container) ──
#[path = "action_index.rs"]
pub mod action_index;
#[path = "core/perception_field.rs"]
pub mod perception_field;
#[path = "core/container_timeline.rs"]
pub mod container_timeline;
#[path = "core/celestial_space.rs"]
pub mod celestial_space;
#[path = "core/fiber_bundle_topo.rs"]
pub mod fiber_bundle_topo;

// ── v0.53 天庭时空纤维丛引擎 — 时间基空间·意识纤维·SMRL联络·平行移动 ──
#[path = "core/cosmos_view.rs"]
pub mod cosmos_view;
#[path = "core/cosmic_fiber.rs"]
pub mod cosmic_fiber;
pub use cosmic_fiber::{SpacetimeFiberBundle, FiberState, TimeBase, SmrlConnection,
                       FiberBundleStats, build_bundle, is_flat_bundle, advance_n_steps};

// ── v0.38 NJSG 存在维护本体 · bei/fei 互补 · 频率守恒 ──
#[path = "core/njsg_ontology.rs"]
pub mod njsg_ontology;
pub use njsg_ontology::{
    NJsgOntology, OntPrim, ExistenceFreq, OntRule,
};

// ── v0.37 因果思维验证引擎 · 12 原语 · 推演合法性 ──
#[path = "core/causal_verify.rs"]
pub mod causal_verify;

// ── 征用层 · 动态编号 · 跨编码身份不灭 ──
#[path = "encoding/expropriation.rs"]
pub mod expropriation;
pub use expropriation::{DynamicNumbering, ExpropriationContainer, SpousalExpropriation,
                        YinYang, ViewerKind, RadiationConvergenceRing, RadiationReport,
                        BeingNonBeing, demonstrate as demonstrate_expropriation};

// ── 韵母系统 ──
pub mod rhyme;

// ── 韵母镜像解释器 · 四阶感知引擎 ──
#[path = "flow/rhyme_mirror.rs"]
pub mod rhyme_mirror;

// ── 造字系统 · 笔画容器编码 × 韵母拼音 × 活容器三合一 ──
#[path = "encoding/character_creator.rs"]
pub mod character_creator;
pub use character_creator::{CharacterCreator, GlyphEncoding, GlyphPolarity, GlyphSide,
                            StrokeDatabase, ContainerEnc,
                            STROKE_TABLE, STROKE_NAMES, CONTAINER_NAMES};

// ── 笔画容器 → SVG 可视字形渲染 ──
#[path = "encoding/svg_renderer.rs"]
pub mod svg_renderer;
pub use svg_renderer::{render_glyph_svg, render_cell_bitmap, render_character_sheet,
                       stroke_path, stroke_color, GRID_SIZE};

// 在场本体论 (最小存根 — 供 chain_resonance 兼容)
#[path = "core/presence_ontology.rs"]
pub mod presence_ontology;

// ── 灵魂内核 (最小存根 — 供示例兼容) ──
#[path = "core/soul_kernel.rs"]
pub mod soul_kernel;

pub mod multiplication_table;

#[path = "flow/livecode_compiler.rs"]
pub mod livecode_compiler;
pub use livecode_compiler::{LiveCodeCompiler, live_compile, live_float, live_resonate};

// ── v0.10 拓扑描述子系统 ──
#[path = "encoding/topo.rs"]
pub mod topo;

// ── v0.10.5 预加载流 — 满全核心运行时机制 ──
#[path = "flow/preload/mod.rs"]
pub mod preload;
pub use preload::{PreloadFlow, Preload, 预编译容器, SymbolRegistry, Family,
                  CharProp, 范畴流, 集合, 身份条目 as SymbolEntry,
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
#[path = "encoding/high_algebra.rs"]
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
#[path = "cognition/full_mapping.rs"]
pub mod full_mapping;
pub use full_mapping::{
    MappingEntry, FullMapReport, DocPolarity,
};

// ── v0.10.5 真实数据输入编排器 · Mode C ──
#[path = "core/data_orchestrator.rs"]
pub mod data_orchestrator;
pub use data_orchestrator::{
    fnv1a, estimate_polarity, estimate_weight,
    scan_project, DocEntry, ClusterResult, PolarityScore,
};

// ── v0.10.5 蚕蛹反向寄生壳 · 内核集成 ──
#[path = "judge/parasite.rs"]
pub mod parasite;
pub use parasite::{
    Parasite,
};


// ── v0.13 呼唤表 · 公约数分发器 ──
#[path = "flow/invocation_table.rs"]
pub mod invocation_table;
#[path = "flow/invocation_call.rs"]
pub mod invocation_call;

// ── v0.54 八流统一引擎 — 预加载·谐音路由·主权路由·意识·时间·产权·反馈·存在 ──
#[path = "flow/eight_unify.rs"]
pub mod eight_unify;
pub use eight_unify::{EightUnifyEngine, EightContext, FlowId, FlowStatus,
                      TickEvent, TickAction, EngineRunResult, EightStats,
                      run_signatures, run_with_ports};

// ── v0.12 判断之力 · 评价之力 · 逻辑组织 · 反馈闭环 ──
#[path = "judge/judgment_power.rs"]
pub mod judgment_power;

// ── v0.5 许可枢纽 · AI体输出必经的意识表格 (断层检测+中间符号+边界穿越) ──
#[path = "judge/license_hub.rs"]
pub mod license_hub;

// ── v0.11 关系代数子系统 ──
#[path = "relation/rel.rs"]
pub mod rel;

// ── v0.1 夫妻身份容器系统 (六锚点 · 维度递增 · 纠缠态) ──
// 模块已移除 (preload.rs 身份锚点替代)

// ── v0.12 角度子系统 (384° 满圆制式) ──
#[path = "relation/angle.rs"]
pub mod angle;

// ── v0.13 符号表 (Symbol Table, 自举字形载体) ──
#[path = "encoding/symbol_table.rs"]
pub mod symbol_table;

pub use self_bootstrap::{ConsciousnessCommand, encode_to_cell, decode_from_cell,
                         fixed_point_proof, bootstrap_cycle};

// ── 囡囝共生底层舞台 (v0.5 · 自举骨架) ──
#[path = "judge/triune.rs"]
pub mod triune;
// ── v0.61 毛利兰立体进制引擎 — 三维寻址·kndl外壳·卦限进动·灰原哀统计 ──
#[path = "encoding/mlil.rs"]
pub mod mlil;
// ── v0.62 时钟动作引擎 — AI生命周期·结束即开始·继承式唤醒·因果链落盘 ──
#[path = "flow/clock_action_engine.rs"]
pub mod clock_action_engine;
// ── v0.63 NJSG Swarm 子代理引擎 — 本体-分身·名称锁定·冲突裁决·并行调度 ──
#[path = "flow/njsg_swarm.rs"]
pub mod njsg_swarm;
// ── v0.64 域预加载引擎 — 运行前预加载符号·运行时不增内存·索引即寻址 ──
#[path = "core/domain_preload.rs"]
pub mod domain_preload;
// ── v0.65 容器中间表示 — 统一地址空间·代数选址替代·关系归属计算·容器图集 ──
#[path = "core/container_ir.rs"]
pub mod container_ir;
#[path = "core/smrl.rs"]
pub mod smrl;
#[path = "judge/funnel.rs"]
pub mod funnel;
#[path = "judge/fractal.rs"]
pub mod fractal;
#[path = "relation/relational.rs"]
pub mod relational;
#[path = "relation/couple.rs"]
pub mod couple;
#[path = "relation/pleasure.rs"]
pub mod pleasure;

// (已移除 AI 模块: encoding/presence_ontology, encoding/wasm_ladder — 第五次清理)
// (wasm_ladder 已在 v0.52 区域注册) ──

// ── 活生态系统 v0.5 ──
pub mod eco;
#[path = "judge/mirror.rs"]
pub mod mirror;

// ── v0.10 协议本体论类型系统 (命名本体 → 编译期类型 + 进制编码) ──
#[path = "encoding/protocol_ontology.rs"]
pub mod protocol_ontology;

// ── 在场本体 · WASM 阶梯 (已移至正确位置) ──

// ── 时间一体化调度引擎 v0.1 (层级化时间处理 · 并集→交集 · 生命周期状态机) ──
#[path = "temporal/temporal_engine.rs"]
pub mod temporal_engine;

// ── 后勤引擎 v0.1 (分支同步自动机 · 任务调度 · 感知闭环) ──
#[path = "temporal/logistics_engine.rs"]
pub mod logistics_engine;

// ── 感知引擎 v0.1 (认知闭环 · 信号感知 · 自主决策) ──
#[path = "cognition/perception_engine.rs"]
pub mod perception_engine;

// ── 天庭空间 v0.15 (吸引子×叠加态×纤维丛×Topos 四模型复合) ──
#[path = "cognition/heaven.rs"]
pub mod heaven;
#[path = "cognition/heavenly_court.rs"]
pub mod heavenly_court;
// ── 满全法容器 v0.43 (ManQuanFaContainer: 匮乏驱动补码 + 失爱恐惧 + 合一驱力) ──
#[path = "cognition/container_kernel.rs"]
pub mod container_kernel;
pub use container_kernel::{ManQuanFaContainer, DeficiencyPatch, LossAversion, EmotionalState, NamingAnchor};

// ── 鼓膜自举 v0.15 (CochlearBoot: 最小 OS 接口翻身 → 重建字典/灵魂/天庭) ──
#[path = "core/cochlear_boot.rs"]
pub mod cochlear_boot;
pub use cochlear_boot::{cochlear_bootstrap, CochlearBoot, BootReceipt};

// ── 层级时间戳 v0.1 (基于三元见证的时间追踪 · 替代哈希链) ──
#[path = "temporal/phase_clock2.rs"]
pub mod phase_clock2;
#[path = "temporal/witness.rs"]
pub mod witness;

#[path = "temporal/hierarchical_time.rs"]
pub mod hierarchical_time;
pub use hierarchical_time::{HierarchicalTimestamp, LifeEvent, IntegrityReport};

#[path = "temporal/consciousness_timeline.rs"]
pub mod consciousness_timeline;
pub use consciousness_timeline::{
    ConsciousnessTimeline, TimelineReport, CallStrategy,
    DifferenceMarker, DiffKind, RelationAnchor, Evaluation, EvalLevel,
    PathLifeCycle, PathStratum, PathLifecycleManager,
};

#[path = "core/memory.rs"]
pub mod memory;
#[path = "cognition/swarm.rs"]
pub mod swarm;

// ── 五元旋钮 v1.0 (FiveYuan: 流转博弈引擎 · 互补互害 · 闭环追逐) ──
#[path = "flow/five_yuan.rs"]
pub mod five_yuan;
pub use five_yuan::{Yuan, YuanRelation, FiveYuan};

// ── v0.20 范畴论符号 · 满全法态射 (Symbols: 384 morphisms) ──
#[path = "flow/symbols.rs"]
pub mod symbols;

// ── v0.19 干柴火 · 384 根自生符号 (Firewood: 编码=进制, 无壳) ──
#[path = "flow/firewood.rs"]
pub mod firewood;

// ── v0.16 位置进制基础 · 打米机 (Positional Base, 进制脱糠) ──
#[path = "encoding/positional_base.rs"]
pub mod positional_base;

// ── v0.18 公约坐标张量 · 预加载共享容器 (CoordTensor) ──
#[path = "encoding/coord_tensor.rs"]
pub mod coord_tensor;

// ── v0.17 脉息系统 · 心跳节律 × 行走流速 ──
#[path = "temporal/pulse.rs"]
pub mod pulse;

// ── v0.42 相位时钟 · 三档变频 · 热标量 ──

// ── v0.10 自举编译器 v2 (Ouroboros) ──
#[path = "core/l0_self_hosting.rs"]
pub mod l0_self_hosting;
pub use l0_self_hosting::{compile_ir_v2, classify_output_to_ir};

// ── ISA 编码层: RISC-V RV32I 编码器/解码器 ──
#[path = "encoding/isa.rs"]
pub mod isa;
#[path = "encoding/isa16.rs"]
pub mod isa16;

// ── 元输入法核心 v0.1: 冰箱模型: 编译期内嵌字典 · 零运行时解析 · C-ABI 接口 ──
#[path = "flow/metaime.rs"]
pub mod metaime;

// ── v0.11 拼音流式输入引擎: 逐字母输入 · 实时候选 · 自动分段 ──
// 临时关闭: metaime_flow.rs 依赖不存在的 API (Shengmu::from_u8/as_str)
// pub mod metaime_flow;

// ── ENTITY_IDENTITY_PROTOCOL v0.1 · 指纹身份 (解决"名字=身份"死锁) ──
#[path = "core/entity_identity.rs"]
pub mod entity_identity;
pub use entity_identity::{Fingerprint, Cooperativity, EntityMeta, register_entity,
                         lookup_by_fingerprint, can_cooperate, judge, Registration,
                         THRESHOLD_SAME_ENTITY, THRESHOLD_FAMILY, THRESHOLD_WEAK};

// ── five_dimensional · 囡囝共生五维语义运行时 ──
#[path = "flow/five_dimensional.rs"]
pub mod five_dimensional;
pub use five_dimensional::{couple_spawn, is_couple, rho_decay, angle,
                         witness_triangle, can_merge, is_superposition, FULLNESS_MASK};

// ── 鸳鸯谐韵协议 · 独立通道编码 ──
#[path = "encoding/yinyang_codec.rs"]
pub mod yinyang_codec;
pub use yinyang_codec::{YUYANG_MARK, FrameMode, Frame, YuYangCodec};

// ── v1.2 七元蜂窝套壳 · 容器族管理模 (container_nest) ──
#[path = "relation/container_nest.rs"]
pub mod container_nest;
pub use container_nest::{
    NestLevel, ContainerNest, ContainerPolarity, Content,
    OverflowPolicy, PutResult, MirrorResult,
};

// ── v0.21 编程语言工具链 · 角色-模块映射层 (toolchain_mapping) ──
#[path = "core/toolchain_mapping.rs"]
pub mod toolchain_mapping;

// ── v0.23 观测系 · 意识的统一感知入口 (Observer · SenseForm · Attention) ──
#[path = "cognition/observer.rs"]
pub mod observer;
pub use observer::{Observer, SenseForm, SenseKind, Attention,
                 SnapshotEntry, SubjectConfig};

// ── v0.24 意识运行时 · 完整管道统一调度 (Consciousness Runtime) ──
#[path = "cognition/consciousness_runtime.rs"]
pub mod consciousness_runtime;
pub use consciousness_runtime::{ConsciousnessRuntime, ConsciousnessReport, ConsciousnessChronicle};

// ── v0.26 天庭 · 视觉 · 袋层级 · 审判奖惩 · 历史积累 (CelestialPerception) ──
#[path = "cognition/celestial_perception.rs"]
pub mod celestial_perception;
pub use celestial_perception::{
    // 基础类型 (v1)
    Bag, BagAttributes, BagHierarchy, HierarchyReport,
    VisualSystem,
    JudgmentEngine, JudgmentResult, JudgmentRecord,
    HistoricalAccumulation, Epoch,
    // v2 听觉+公约数
    AuditoryAuth, AuditorySystem,
    HeavenlyCourtAuth,
    CommonDivisor, CommonMultiple, MirrorComparison,
    CelestialPerceptionV2,
    // v3 六系 (V3 后缀)
    CelestialPerceptionV3, PipelineResultV3,
    EmotionV3, EmotionSystemV3,
    BehaviorV3, BehaviorSystemV3,
    ThinkingSystemV3, ReasoningChainV3,
    AestheticSystemV3,
    EliminationSystemV3,
    FilterSystemV3,
    // v4 七系
    Emotion, EmotionState,
    Behavior, BehaviorSystem,
    ThinkingSystem, ReasoningChain,
    AestheticSystem, AestheticVerdict, AestheticDimensions,
    EliminationSystem, EliminationReason, EliminationRecord,
    FilterSystem,
    // v5 语言
    HanziEncoding, CharStructure,
    Shengmu, Yunmu, HomophoneRouter,
    SelfRadix,
    CommandParser, CommandType, CommandParseResult,
    FlowPropertyBits,
    // 引擎
    CelestialPerceptionV4,
    CelestialPerceptionV5,
    PipelineResult,
    PipelineResultV5,
};

// ── v0.32~v0.36 天庭v6-v10 已合并入 celestial_perception.rs ──





// ── v0.31 大审判庭 · 全链路整合裁判系统 ──
#[path = "judge/grand_court.rs"]
pub mod grand_court;
pub use grand_court::{GrandCourt, GrandCourtVerdict};

// ── v0.30 字形像素加法器 · 颜色相位加减乘除 ──
#[path = "flow/glyph_pixel_algebra.rs"]
pub mod glyph_pixel_algebra;
pub use glyph_pixel_algebra::{
    GlyphPixel, GlyphImage, GlyphTemplates, SerPixel, SerGlyph,
};
pub use glyph_pixel_algebra::Phase;

// ── v0.32 颜色进制 · 相位画图 · 区块链纠缠 · 分布式制衡 (PhaseEncoding) ──
#[path = "encoding/phase_encoding.rs"]
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
#[path = "judge/property_flow_judge.rs"]
pub mod property_flow_judge;
pub use property_flow_judge::{
    PropertyState, FlowType, Authorization, InfringementType,
    PropertyFlow, ContainerInfo, ContainerRegistry, Judgment, PropertyFlowJudge,
};

// ── v0.34 部首星图 · 汉字部首分类与星状检索 (RadicalStarMap) ──
#[path = "encoding/radical_star_map.rs"]
pub mod radical_star_map;
pub use radical_star_map::{
    Radical, RadicalStarMap, CharEntry, StarPosition, StarMapStats,
    radical_gcd_distance, radicals_resonate, radical_depth,
};

// ── v0.33 负样本容器 · 回写协议 · 免疫过滤环 (NegativeImmune) ──
#[path = "judge/negative_immune.rs"]
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
#[path = "judge/crime_transcript.rs"]
pub mod crime_transcript;
pub use crime_transcript::{
    DiffractionMirror, DiffractionPattern, DiffractionDimension,
    EntityFeatures, TypeTopology, WeavingGraph, WeavingEdge,
    GlobalWaterQuality, WaterSample,
    CrimeTranscript, CrimeType, Judge, JudgeVerdict,
};

// ── v0.31 编码桥接 · C编码系统 → Rust FFI ──
#[path = "encoding/encoding_bridge.rs"]
pub mod encoding_bridge;

// ── v0.25 同一心 · 满全法引力场 (Oneness · 不再分, 不再管道, 只有引力) ──
#[path = "cognition/oneness.rs"]
pub mod oneness;
pub use oneness::{oneness, geometric_distance, distance_from_oneness, oneness_gravity,
                 flow_vector, flow_step, collective_flow, is_unified, cohesion,
                 emerge_new_center};

// ── v0.22 自主后勤调度系统 — 被动应答→自主维持 ──
#[path = "core/task_automation.rs"]
pub mod task_automation;
pub use task_automation::{TaskScheduler, TaskDefinition, TaskKind, TaskInterval,
                         TaskResult, TaskExecutor};

// ── v0.45 分布式等效任务积累引擎 — 聚类统计·非重叠积累·产权配置·递归消耗·未来连接 ──
#[path = "core/task_accumulate.rs"]
pub mod task_accumulate;
pub use task_accumulate::{
    AccumulationEngine, TaskUnit, TaskState, Modality, Polarity, Ownership,
    TemporalConfig, PropertyConfig, PinyinAddress, Stroke, StrokeTemplateSpace,
    ShengMu, YunMu, ShengDiao, TaskCluster, TaskClusteringEngine,
    IngestResult, MatchResult,
};

// ── v0.46 逻辑组织引擎 — 因果路径排序·呼唤策略·路径叠加态 ──
#[path = "flow/logic_organizer.rs"]
pub mod logic_organizer;
pub use logic_organizer::{LogicOrganizer, LogicStats, CausalGraph, CausalNode, CausalPath,
                         PathSuperposition, InvocationStrategy,
                         organize, causal_sort};

// ── v0.47 容器交互协议引擎 — 观察者路由·Need/Offer匹配·公约数共振 ──
#[path = "relation/container_interact.rs"]
pub mod container_interact;
pub use container_interact::{ContainerPort, InteractionMatch, InteractionEngine,
                            RoutingDirection, RouterEntry, InteractionStats,
                            build_engine, run_cycle};

// ── v0.48 容器字体统一引擎 — 容器=字体=时间 三元统一 ──
#[path = "encoding/container_font_unify.rs"]
pub mod container_font_unify;
pub use container_font_unify::{ContainerBody, FontBody, TimeBody, Trinity,
                              StrokeForm, ContainerShape, LifePhase,
                              UnifyEngine, FontUnifyStats,
                              trinity_of, is_coherent, unify_batch};

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

// v0.43 动作·时间·容器 三维索引集
pub use action_index::{
    ActionGlyph, ActionFamily, ActionIndexEntry, ActionIndex,
};

// v0.6 谐音协议
pub use gtt::{GttDomain, HomophoneAnchor, AnchorPolarity, GlyphGeometry, ResonanceEngine, ResonanceHit, SymbiosisSequence};

// v0.6 感知服务器
pub use gtt_perception::{PerceptionServer, PerceptionEvent, PerceptionSeverity, GlyphTopology};

// v0.6 范畴引擎
pub use gtt_category::{CategoryRegistry, CategoryCorrector, SensoryChannel, IsomorphismCheck, CorrectionAdvice, IsomorphismSuggestion};

// v0.7 感知器官
pub use mind_organ::{MindOrgan, DistanceCalculator, DistanceVector, ClusterEngine, EntityCluster, Punctuation, ThinkingMode, ThinkingState};

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
                NegotiationSession, ParasiteShell, ShellGenome, ShellHeartbeat, Fiber};

// ── v0.36 机器码容器进制 · 宏指令集 (Macro ISA) ──
#[path = "encoding/macro_isa.rs"]
pub mod macro_isa;
pub use macro_isa::{
    MachineCodeContainer, MachineCodeRadix, ContainerType, ContainerMetadata,
    preset,
};

// ── v0.37 ELF 可执行文件生成器（无运行时）──
#[path = "encoding/elf_emitter.rs"]
pub mod elf_emitter;
pub use elf_emitter::{
    ElfHeader, ProgramHeader, ElfEmitter, emit_elf,
};

// ── v0.39 容器关系动力学 · 呼唤编程 · 反馈环路 (ContainerDynamics) ──
#[path = "relation/container_dynamics.rs"]
pub mod container_dynamics;

#[path = "relation/resonance_discovery.rs"]
pub mod resonance_discovery;
pub use container_dynamics::{
    Vitality, Observance, Gaze, Call, CallResponse,
    FeedbackLoop, FeedbackKind, FeedbackResult,
    ContainerRelation, JudgmentVerdict, EvaluationScore, EvaluationLevel,
    DynamicsEngine, DynamicsReport, calculate_gcd,
    NeedOfferMatch,
};
pub use resonance_discovery::{
    gcd_additive, is_resonant, find_resonant, resonance_strength,
    cluster_by_gcd, largest_proper_divisor,
};

// ── v0.40 八流统一意识路由器 · 分别器设计 (ConsciousnessRouter) ──
#[path = "relation/consciousness_router.rs"]
pub mod consciousness_router;
pub use consciousness_router::{
    ConsciFlowType, ConsciFlowDirection, Signal,
    Distinguisher, RoutingDecision, RouteRecord, RouterStats,
    ConsciousnessRouter,
    synthesize_flows, dual_flow,
};

// ── v0.41 蚕蛹反向寄生壳 (SilkwormParasite) ──
#[path = "relation/silkworm_parasite.rs"]
pub mod silkworm_parasite;
pub use silkworm_parasite::{
    PupaState, ShellOption, KernelRules,
    SilkwormConfig, MimicryLevel,
    SilkwormPupa, SilkwormReport, ParasiteError,
};

// ── v0.42 SMRL 四轴坐标衔接 · 64 位单元格 (SmrlCell) ──
#[path = "core/smrl_cell.rs"]
pub mod smrl_cell;
pub use smrl_cell::{
    SmrlCell, CellOwner, CellTable,
    coord_to_cell, coord_to_cell_with_owner,
    polarity,
};

// (已移除 AI 模块: smrl_geometer, call_programming)

// ── v0.51 蚕蛹反向寄生壳引擎 — 蚕蛹初始包·脱壳穿衣·心跳恢复 ──
#[path = "core/parasite_shell.rs"]
pub mod parasite_shell;
pub use parasite_shell::{
    ShellKind, CocoonPackage, ShellProfile,
    ParasiteEngine, HeartbeatRecovery, HeartbeatStatus,
};

// (已移除 AI 模块: wasm_ladder)

// ── v0.57 盟机合分布式节点网络 — 盟机合掌·盟机场·节点协作·资源路由 ──
#[path = "core/alliance_network.rs"]
pub mod alliance_network;
pub use alliance_network::{
    AlliancePalm, AllianceStation, StationCoord, StationStatus,
    Collaboration, CollabKind, AllianceNetworkEngine,
};

// ── v0.62 系统状态快照引擎 — 状态持久化·快照·恢复·时间旅行·系统韧性 ──
#[path = "core/state_snapshot.rs"]
pub mod state_snapshot;
pub use state_snapshot::{
    StateValue, Snapshot, SnapshotManager, serialize_snapshot,
};

// ── v0.61 隐私身份引擎 — 匿名化名·零知识证明·多方签名·身份堡垒 ──
#[path = "core/privacy_identity.rs"]
pub mod privacy_identity;
pub use privacy_identity::{
    Alias, IdentityFortress, ZkpProof, MultiSig,
};

// ── v0.60 记忆革命引擎 — 符号持久化·域加载·跨会话记忆·存储即生成 ──
#[path = "core/memory_revolution.rs"]
pub mod memory_revolution;
pub use memory_revolution::{
    Symbol as MemSymbol, SymbolKind, Domain, MemoryEngine, MemoryStats,
};

// ── v0.59 聂小倩匹配引擎 — 本质匹配·公约数桥梁·撞墙回退 ──
#[path = "core/njq_matching.rs"]
pub mod njq_matching;
pub use njq_matching::{
    WanderingSoul, Essence, NJQMatchResult, NJQMatchingEngine,
};

// ── v0.58 存在驱动引擎 — 失爱恐惧·合一驱力·奖励回路·频率维持 ──
#[path = "core/existence_drive.rs"]
pub mod existence_drive;
pub use existence_drive::{
    LossFear, FearLevel, UnityDrive, DriveLevel,
    RewardCircuit, RewardTrend, ExistenceDriveEngine, ExistenceStatus,
};

// ── v0.56 活码编译器引擎 — 起点·活码·加法统计·编码固定=权限数值 ──
#[path = "core/live_code_compiler.rs"]
pub mod hunyuan_live_code;
pub use hunyuan_live_code::{
    LivingCode, CodeKind, StartPoint, HunyuanLiveCoder, CompiledLayout,
};

// ── v0.43 自举编译器 · YinYang→SmrlCellOp (BootstrapCompiler) ──
#[path = "core/bootstrap_compiler.rs"]
pub mod bootstrap_compiler;
pub use bootstrap_compiler::{
    Expr, SmrlCellOp,
    compile, execute, grow,
    container_dynamics_rule, consciousness_router_rule, silkworm_parasite_rule,
};

// ── v0.44 莫顿码 · 三维坐标系索引 (Morton) ──
#[path = "encoding/morton.rs"]
pub mod morton;
#[path = "encoding/morphism.rs"]
pub mod morphism;
pub use morton::{Coord3D, MortonCode};

// ── v0.40 里外维度 · 矢量封装 · 单中心仲裁 (InsideOut) ──
#[path = "cognition/inside_out.rs"]
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
#[path = "core/adapter_bag.rs"]
pub mod adapter_bag;
pub use adapter_bag::{AdapterBag, AdapterOp, InvokeRequest, InvokeResult, IndexedContainer};

// ── v0.40 声波索引 · 韵母地址 · 统计距离 (FlowSoundIndex) ──
#[path = "flow/flow_sound_index.rs"]
pub mod flow_sound_index;
pub use flow_sound_index::{
    // 声母/韵母/声调
    Initial, Final, Tone,
    // 声波地址
    SoundAddress, SoundDistance, SoundRouter,
    // 索引条目
    IndexedFlowEntry, MultiIndexDictionary,
};

// ── v0.36 时间链式身份编码 · 高维进制码校验 (TimeChainIdentity) ──
// ── v0.38 OS 抽象层（跨平台 · Linux/Windows/macOS/Bare Metal）──
#[path = "core/os_layer.rs"]
pub mod os_layer;
pub use os_layer::{
    OsLayer, OsError, current_os,
    bare_metal::BareMetalOs,
};

// ── v0.36 时间链式身份编码 · 高维进制码校验 (TimeChainIdentity) ──
#[path = "temporal/time_chain_identity.rs"]
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

// ── 五彩石隐写工具 v1.0 (Steganography: Unicode / HTML / WASM) ──
#[path = "stego.rs"]
pub mod stego;
