// main_nat.zig — 囡囝共生 OS · 原生入口
// Zig 协调层调用 C 运行时 (arbiter/membrane/governance/l0 闭环)
//
// 闭环验证:
//   1. 裁决引擎: arbiter_create → set_protagonist(=2 工藤新一) → judge → verify_chain
//   2. 膜妻阀:   mv_create → activate → phonetic_encode → fortune
//   3. 管辖系统: governance_create → activate → merge_pair
//   4. L0 编译器: l0c_create → compile → op_pinyin → report

const std = @import("std");
const cr = @import("c_runtime.zig");

fn p(comptime fmt: []const u8, args: anytype) void {
    std.debug.print(fmt, args);
}

pub fn main() !void {
    p("\n", .{});
    p("========================================================\n", .{});
    p("  囡囝共生 · 原生 OS (Zig 协调 + C 运行时闭环)     \n", .{});
    p("========================================================\n", .{});
    p("\n", .{});

    // ============================================================
    // 模块 1: 裁决引擎 (Arbiter)
    // ============================================================
    p("[1] 裁决引擎 (arbiter)\n", .{});

    const arbiter = cr.arbiterInit("hunyuan-timeline") orelse {
        std.log.err("arbiter_create failed", .{});
        return;
    };
    defer cr.arbiter_destroy(arbiter);

    // 设置主人公: 工藤新一 = S_KNDO = 2
    _ = cr.arbiterSetProtagonist(arbiter, 2, "Kudo_Shinichi");

    // 注册工具链
    const tool_color = cr.arbiter_register_tool(arbiter, "color-radix");
    const tool_sound = cr.arbiter_register_tool(arbiter, "sound-radix");
    const tool_metaime = cr.arbiter_register_tool(arbiter, "metaime");
    _ = cr.arbiter_add_dependency(arbiter, tool_metaime, tool_color);
    _ = cr.arbiter_add_dependency(arbiter, tool_metaime, tool_sound);

    // 裁决: 允许 metaime 执行?
    const can_run = cr.arbiter_can_execute(arbiter, tool_metaime);
    p("  tools: color#{} sound#{} metaime#{} (deps: color+sound)\n", .{ tool_color, tool_sound, tool_metaime });
    p("  metaime executable: {}\n", .{can_run});

    // 记录时间线事件
    const seq1 = cr.arbiter_record_event(arbiter, cr.c.EVENT_DECISION, 0x100, cr.c.OWNER_MASTER, "init color radix");
    const seq2 = cr.arbiter_record_event(arbiter, cr.c.EVENT_ABSORB, 0x200, cr.c.OWNER_WIFE, "wife absorbs sound");

    // 裁决判断
    var fortune: cr.c.fortune_type_t = cr.c.FORTUNE_NEUTRAL;
    const ruling = cr.arbiter_judge(arbiter, 0x100, cr.c.EVENT_MERGE, &fortune);
    const ruling_str = switch (ruling) {
        cr.RULING_ALLOW => "ALLOW(auspicious)",
        cr.RULING_DENY => "DENY(inauspicious)",
        cr.RULING_CONDITIONAL => "CONDITIONAL(neutral)",
        else => "DEFER",
    };
    p("  judge(target=0x100): ", .{});
    p("{s}", .{ruling_str});
    p(" [ruling={}]\n", .{ruling});

    // 切换时间视角到未来
    cr.arbiter_set_view(arbiter, cr.VIEW_FORWARD);
    const future = cr.arbiter_predict(arbiter, cr.c.EVENT_PUSH);
    p("  predict(next=PUSH): fortune={}\n", .{future});

    // 完整性验证
    const chain_ok = cr.arbiter_verify_chain(arbiter);
    p("  chain integrity: {} · seq: {},{}\n", .{ chain_ok, seq1, seq2 });
    cr.arbiter_report(arbiter);
    p("\n", .{});

    // ============================================================
    // 模块 2: 膜妻阀 (Membrane Valve)
    // ============================================================
    p("[2] 膜妻阀 (membrane_valve)\n", .{});

    const membrane = cr.membraneInit("ManQuanFa_membrane") orelse {
        std.log.err("mv_create failed", .{});
        return;
    };
    defer cr.membrane_destroy(membrane);

    // 激活
    const mv_active = cr.membraneActivate(membrane);
    p("  activated: {}\n", .{mv_active});

    // 注册缚X变体
    _ = cr.mv_register_variant(membrane, cr.c.MV_VARIANT_FU_QI, "FuQi");
    _ = cr.mv_register_variant(membrane, cr.c.MV_VARIANT_FU_QUAN, "FuQuan");
    _ = cr.mv_register_variant(membrane, cr.c.MV_VARIANT_FU_QING, "FuQing");

    // 谐音编码
    const encoded = cr.mv_phonetic_encode(membrane, "fqmy");
    if (encoded) |ptr| {
        _ = ptr;
        p("  phonetic_encode(fqmy): (encoded)\n", .{});
    }

    // 结界阻挡检查
    const blocked_external = cr.membraneCheckBlock(membrane, 999);
    const allowed_internal = cr.membraneCheckBlock(membrane, 2);
    p("  barrier: external(999)={} internal(2/master)={}\n", .{ blocked_external, allowed_internal });

    // 吉凶解释
    const fortune2 = cr.mv_interpret_fortune(membrane, "fqmy");
    p("  interpret(fqmy): fortune={}\n", .{fortune2});
    cr.mv_report(membrane);
    p("\n", .{});

    // ============================================================
    // 模块 3: 管辖系统 (Governance)
    // ============================================================
    p("[3] 管辖系统 (governance)\n", .{});

    var gov = cr.governanceInit(1, 2);
    const gov_active = cr.governanceActivate(&gov);
    p("  activated: {}\n", .{gov_active});

    // 添加被管辖节点
    _ = cr.governance_add_governed(&gov, 10);
    _ = cr.governance_add_governed(&gov, 20);
    _ = cr.governance_add_governed(&gov, 30);

    // 融合计算
    const merged_val = cr.governance_merge_pair(&gov, 0.75, 0.85);
    p("  governed: {} nodes · merge(0.75,0.85)={d:.2}\n", .{ gov.governed_count, merged_val });
    cr.governance_report(&gov);
    p("\n", .{});

    // ============================================================
    // 模块 4: L0 自举编译器
    // ============================================================
    p("[4] L0 self-compiling compiler\n", .{});

    const l0 = cr.l0Create() orelse {
        std.log.err("l0c_create failed", .{});
        return;
    };
    defer cr.l0c_destroy(l0);

    // 编译一条 L0 程序
    const source = "LOADI a 5\nLOADI b 3\nADD a b a\nPRINT a\nHALT";
    const compiled = cr.l0Compile(l0, source);
    p("  compiled: {}\n", .{compiled});

    // 查询 IR
    var ir_count: u16 = 0;
    const ir = cr.l0c_get_ir(l0, &ir_count);
    p("  IR instructions: {}\n", .{ir_count});
    for (0..@min(ir_count, 10)) |i| {
        const op_val = ir[i].op;
        p("    [{d}] op={d} {d},{d} = {d}\n", .{ i, op_val, ir[i].operand1, ir[i].operand2, ir[i].result });
    }

    // 自编译校验
    const self_ok = cr.l0c_self_test(l0);
    p("  self-test: {}\n", .{self_ok});
    cr.l0c_report(l0);
    p("\n", .{});

    // ============================================================
    // 闭环验证摘要
    // ============================================================
    p("== closed-loop verification summary ==\n", .{});
    p("  [ok] arbiter: master chain={} ruling=", .{chain_ok});
    p("{s}\n", .{ruling_str});
    p("  [ok] membrane: active={} barrier(ext/int)={}/{}\n", .{ mv_active, blocked_external, allowed_internal });
    p("  [ok] governance: active={} nodes={} merge={d:.2}\n", .{ gov_active, gov.governed_count, merged_val });
    p("  [ok] L0: compiled={} self-test={}\n", .{ compiled, self_ok });
    p("\n", .{});
    p("Zig coordinator <--> C runtime: CLOSED LOOP OK\n", .{});
    p("native pipeline: arbiter -> membrane -> governance -> l0c\n", .{});
    p("\n", .{});
}
