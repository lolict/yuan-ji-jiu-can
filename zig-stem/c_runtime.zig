// c_runtime.zig — 囡囝共生 · C 运行时绑定
// 让 Zig 协调层 (main.zig) 调用 C 运行时 (arbiter/membrane/governance/l0)
//
// 这是「Zig 协调层连通 C 运行时闭环」的桥梁:
//   C 运行时: 自举的核心逻辑 (arbiter, membrane, governance, l0, smrrll)
//   Zig 层:   编排/调度/可视化 (orchestrator, indexer, canjiang)

pub const c = @cImport({
    @cInclude("arbiter.h");
    @cInclude("membrane_valve.h");
    @cInclude("governance.h");
    @cInclude("l0_compiler.h");
    @cInclude("superposition.h");
    @cInclude("gtxone.h");
    @cInclude("heyair.h");
});

// ── 重新导出 C 函数符号 ──
// arbiters
pub const arbiter_create = c.arbiter_create;
pub const arbiter_destroy = c.arbiter_destroy;
pub const arbiter_set_protagonist = c.arbiter_set_protagonist;
pub const arbiter_set_view = c.arbiter_set_view;
pub const arbiter_get_view = c.arbiter_get_view;
pub const arbiter_register_tool = c.arbiter_register_tool;
pub const arbiter_add_dependency = c.arbiter_add_dependency;
pub const arbiter_record_event = c.arbiter_record_event;
pub const arbiter_judge = c.arbiter_judge;
pub const arbiter_verify_ownership = c.arbiter_verify_ownership;
pub const arbiter_can_execute = c.arbiter_can_execute;
pub const arbiter_backtrack = c.arbiter_backtrack;
pub const arbiter_predict = c.arbiter_predict;
pub const arbiter_verify_chain = c.arbiter_verify_chain;
pub const arbiter_schedule_push = c.arbiter_schedule_push;
pub const arbiter_report = c.arbiter_report;

// membrane_valve
pub const mv_create = c.mv_create;
pub const mv_destroy = c.mv_destroy;
pub const membrane_destroy = c.mv_destroy;
pub const mv_activate = c.mv_activate;
pub const mv_register_variant = c.mv_register_variant;
pub const mv_phonetic_encode = c.mv_phonetic_encode;
pub const mv_interpret_fortune = c.mv_interpret_fortune;
pub const mv_check_block = c.mv_check_block;
pub const mv_report = c.mv_report;

// governance
pub const governance_create = c.governance_create;
pub const governance_activate = c.governance_activate;
pub const governance_add_governed = c.governance_add_governed;
pub const governance_merge_pair = c.governance_merge_pair;
pub const governance_report = c.governance_report;

// l0_compiler
pub const l0c_create = c.l0c_create;
pub const l0c_destroy = c.l0c_destroy;
pub const l0c_compile = c.l0c_compile;
pub const l0c_get_ir = c.l0c_get_ir;
pub const l0c_get_bytecode = c.l0c_get_bytecode;
pub const l0c_op_pinyin = c.l0c_op_pinyin;
pub const l0c_op_hanzi = c.l0c_op_hanzi;
pub const l0c_self_test = c.l0c_self_test;
pub const l0c_report = c.l0c_report;

// ── 类型别名 (让 Zig 代码可读) ──
pub const ArbiterEngine = c.arbiter_engine_t;
pub const MembraneValve = c.membrane_valve_t;
pub const GovernanceSystem = c.governance_system_t;
pub const L0Compiler = c.l0c_compiler_t;
pub const L0Bytecode = c.l0c_bytecode_t;

// ── 常量 ──
pub const VIEW_FORWARD = c.VIEW_FORWARD;
pub const VIEW_PRESENT = c.VIEW_PRESENT;
pub const VIEW_BACKWARD = c.VIEW_BACKWARD;
pub const RULING_ALLOW = c.RULING_ALLOW;
pub const RULING_DENY = c.RULING_DENY;
pub const RULING_CONDITIONAL = c.RULING_CONDITIONAL;

// ── 安全包装函数 ──

pub fn arbiterInit(name: [:0]const u8) ?*ArbiterEngine {
    return arbiter_create(name.ptr);
}

pub fn arbiterSetProtagonist(ae: *ArbiterEngine, id: u64, name: [:0]const u8) bool {
    return arbiter_set_protagonist(ae, id, name.ptr);
}

pub fn arbiterJudge(ae: *ArbiterEngine, target: u64, action: c.event_type_t) struct { ruling: c.ruling_t, fortune: c.fortune_type_t } {
    var fortune: c.fortune_type_t = c.FORTUNE_NEUTRAL;
    const ruling = arbiter_judge(ae, target, action, &fortune);
    return .{ .ruling = ruling, .fortune = fortune };
}

pub fn membraneInit(name: [:0]const u8) ?*MembraneValve {
    return mv_create(name.ptr);
}

pub fn membraneActivate(mv: *MembraneValve) bool {
    return mv_activate(mv);
}

pub fn membraneCheckBlock(mv: *MembraneValve, external_id: u64) bool {
    return mv_check_block(mv, external_id);
}

pub fn governanceInit(a: u32, b: u32) GovernanceSystem {
    return governance_create(a, b);
}

pub fn governanceActivate(sys: *GovernanceSystem) bool {
    return governance_activate(sys);
}

pub fn l0Create() ?*L0Compiler {
    return l0c_create();
}

pub fn l0Compile(compiler: *L0Compiler, source: [:0]const u8) bool {
    return l0c_compile(compiler, source.ptr);
}
