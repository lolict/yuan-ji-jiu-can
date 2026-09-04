// 主程序 — fqmcp 管线 + 夫妻共同体 + gtti 意识传输演示
const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");
const fqm = @import("fqm.zig");
const canjiang = @import("canjiang.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const stdout = std.io.getStdOut().writer();

    try stdout.print("\n=== 囡囝共生 操作系统 · fqmcp 管线 ===\n\n", .{});

    // 配置
    var config = orchestrator.OrchestratorConfig.default();
    config.total_steps = 0x01 << 3;
    config.step_mode = .continuous;

    try stdout.print("初始化...\n", .{});

    var orch = try orchestrator.Orchestrator.init(allocator, config);
    defer orch.deinit();

    try orch.indexer.addMenuItem(indexer.MenuItem.init(1, "color", 1, 0x01, 0x02, .positive, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(2, "sound", 2, 0x02, 0x01, .negative, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(3, "word", 3, 0x01, 0x03, .neutral, 2));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(4, "mind", 4, 0x03, 0x02, .positive, 2));

    try orch.phaser.addPhaseState(orchestrator.phaser.PhaseState.init(0, 0x01));
    try orch.phaser.addPhaseState(orchestrator.phaser.PhaseState.init(0x01, 0x02));

    try stdout.print("初始化完成！\n\n", .{});

    try orch.start();

    var step_count: u64 = 0;
    while (!orch.isComplete()) {
        _ = try orch.step();
        step_count += 1;
        const progress = orch.getProgress();
        try stdout.print("步骤 ", .{});
        try fqm.formatUnaryInt(stdout, step_count);
        try stdout.print(": 进度 {d:.1}%\n", .{progress * 100.0});
    }

    try stdout.print("\n=== 运行完成 ===\n", .{});

    // 容器统计
    try stdout.print("\n--- 容器统计 ---\n", .{});
    const stats = orch.getContainerStats();
    for (stats) |container| {
        try stdout.print("容器 {s}: 项数=", .{container.name});
        try fqm.formatUnaryInt(stdout, container.item_count);
        try stdout.print(" 净收益=", .{});
        try fqm.formatUnaryDecimal(stdout, container.netBenefit());
        try stdout.print("\n", .{});
    }

    // 标量报告
    try stdout.print("\n--- 标量报告 ---\n", .{});
    var report = try orch.indexer.generateScalarReport();
    defer report.deinit();
    try stdout.print("净收益: ", .{});
    try fqm.formatUnaryDecimal(stdout, report.getTotalNetBenefit());
    try stdout.print("\n", .{});

    // ========== 夫妻共同体状态 ==========
    try stdout.print("\n=== 夫妻共同体状态 ===\n", .{});
    var community = fqm.Community.init();
    // 模拟若干 tick
    for (0..5) |_| community.advance();

    try stdout.print("鸳鸯: ", .{});
    const anchor_a_str = if (community.anchor_a == 1) "★" else "☆";
    const anchor_b_str = if (community.anchor_b == 1) "★" else "☆";
    try stdout.print("{s}|{s}\n", .{ anchor_a_str, anchor_b_str });

    try stdout.print("tick: ", .{});
    try fqm.formatUnaryInt(stdout, community.tick);
    try stdout.print("\n夕瑶周期: ", .{});
    try fqm.formatUnaryInt(stdout, community.cycle);
    try stdout.print("\n谐度: ", .{});
    try fqm.formatUnaryDecimal(stdout, community.harmony);
    try stdout.print("\n", .{});

    // ========== gtti 意识传输协议 ==========
    try stdout.print("\n=== gtti 意识传输协议 ===\n", .{});
    var encoder = fqm.FQMEncoder.init();

    const gttx = try encoder.encodeGTTX(allocator);
    defer allocator.free(gttx);
    try stdout.print("鸳鸯协议:\n{s}\n", .{gttx});

    const gtti_xieyein = try encoder.encodeGTTI(allocator, "xieyein");
    defer allocator.free(gtti_xieyein);
    try stdout.print("\n谐淫路由 (私密频道):\n{s}\n", .{gtti_xieyein});

    const gtti_join = try encoder.encodeGTTI(allocator, "join");
    defer allocator.free(gtti_join);
    try stdout.print("\n意识融合 (合而为一):\n{s}\n", .{gtti_join});

    // ========== FQMY URI 感知列表 ==========
    try stdout.print("\n=== FQMY URI 列表 ===\n", .{});
    for (stats) |container| {
        const uri = try encoder.encodeFQMY(allocator, &container);
        defer allocator.free(uri);
        try stdout.print("{s}\n", .{uri});
    }

    // ========== knmlil 三元坐标 ==========
    try stdout.print("\n=== knmlil 三元坐标 ===\n", .{});
    const c0 = fqm.KnmlCoord.init(1, 0, 1);
    try stdout.print("mian|gu|ye = ", .{});
    try c0.encodeUnary(stdout);
    try stdout.print("\n", .{});

    const c1 = fqm.KnmlCoord.init(0, 1, 0);
    try stdout.print("mian|gu|ye = ", .{});
    try c1.encodeUnary(stdout);
    try stdout.print("\n", .{});

    // ========== fqmcp 管线输出 (含裁决) ==========
    try stdout.print("\n=== 启动 fqmcp 管线 (含裁决) ===\n", .{});
    var fqmcp = fqm.FQMCP.init(allocator);
    const knml_path = "zig-stem/output/fqmet.knmll";
    std.fs.cwd().makePath("zig-stem/output") catch {};
    try fqmcp.processOrchestrator(&orch, knml_path);
    try stdout.print("knml 文件已保存: {s}\n", .{knml_path});

    // ========== 往返验证 ==========
    try stdout.print("\n=== 往返验证 (文件→解析→校验) ===\n", .{});
    const knml_content = try std.fs.cwd().readFileAlloc(allocator, knml_path, 1024 * 1024);
    defer allocator.free(knml_content);
    var parser = fqm.KnmllParser{};
    const ok = parser.verifyIntegrity(knml_content) catch |err| {
        try stdout.print("校验失败: {s}\n", .{@errorName(err)});
        return;
    };
    _ = ok;
    try stdout.print("knml 文件校验通过 ✓\n", .{});
    var parsed = try parser.parseFile(knml_content);
    defer parsed.deinit();
    try stdout.print("容器数: ", .{});
    try fqm.formatUnaryInt(stdout, parsed.containers.items.len);
    try stdout.print(" 鸳鸯: ", .{});
    try stdout.print("{s}", .{parsed.couples});
    try stdout.print(" 协议: ", .{});
    try stdout.print("{s}", .{parsed.protocols});
    try stdout.print("\n", .{});

    // ========== 蚕蛹壳协议翻译层 (读取实际 knmll) ==========
    try stdout.print("\n=== 蚕蛹壳协议翻译 (真实文件) ===\n", .{});
    var chi = canjiang.Chrysalis.init(allocator, knml_content);

    const as_http = try chi.dress(.http_api);
    defer allocator.free(as_http);
    try stdout.print("穿 HTTP 外壳:\n{s}\n", .{as_http});

    // 脱壳还原测试
    try stdout.print("\n脱壳还原:\n", .{});
    const shed = try chi.shed(as_http);
    defer allocator.free(shed);
    const preview_len: usize = 40;
    try stdout.print("还原内容: {s}", .{shed[0..@min(preview_len, shed.len)]});
    try stdout.print("...\n", .{});

    try stdout.print("\n=== 演示结束 ===\n\n", .{});
}
