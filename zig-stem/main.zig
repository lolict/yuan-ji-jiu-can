// 主程序 — fqmcp 管线演示
const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");
const fqm = @import("fqm.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("\n=== 囡囝共生 操作系统 · fqmcp 管线 ===\n\n", .{});

    // 配置
    var config = orchestrator.OrchestratorConfig.default();
    config.total_steps = 0x01 << 3; // 8 步
    config.step_mode = .continuous;

    std.debug.print("初始化所有模块...\n", .{});

    var orch = try orchestrator.Orchestrator.init(allocator, config);
    defer orch.deinit();

    // 测试数据
    try orch.indexer.addMenuItem(indexer.MenuItem.init(1, "color", 1, 0x01, 0x02, .positive, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(2, "sound", 2, 0x02, 0x01, .negative, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(3, "word", 3, 0x01, 0x03, .neutral, 2));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(4, "mind", 4, 0x03, 0x02, .positive, 2));

    try orch.phaser.addPhaseState(orchestrator.phaser.PhaseState.init(0, 0x01));
    try orch.phaser.addPhaseState(orchestrator.phaser.PhaseState.init(0x01, 0x02));

    std.debug.print("所有模块初始化完成！\n\n", .{});

    try orch.start();
    std.debug.print("协调器已启动...\n\n", .{});

    const stdout = std.io.getStdOut().writer();

    var step_count: u64 = 0;
    while (!orch.isComplete()) {
        _ = try orch.step();
        step_count += 1;

        const progress = orch.getProgress();
        const align_status = orch.getAlignmentStatus();

        std.debug.print("步骤 ", .{});
        try fqm.formatUnaryInt(stdout, step_count);
        std.debug.print(": 进度 {d:.1}%, 对齐 {d:.2}\n", .{ progress * 100.0, align_status.confidence });
    }

    std.debug.print("\n=== 运行完成 ===\n", .{});

    // 容器统计
    std.debug.print("\n--- 容器统计 ---\n", .{});
    const stats = orch.getContainerStats();
    for (stats) |container| {
        std.debug.print("容器 {s}: 项数=", .{container.name});
        try fqm.formatUnaryInt(stdout, container.item_count);
        std.debug.print(", 正因果=", .{});
        try fqm.formatUnaryInt(stdout, container.positive_count);
        std.debug.print(", 净收益=", .{});
        try fqm.formatUnaryDecimal(stdout, container.netBenefit());
        std.debug.print("\n", .{});
    }

    // 标量报告
    std.debug.print("\n--- 标量报告 ---\n", .{});
    var report = try orch.indexer.generateScalarReport();
    defer report.deinit();
    std.debug.print("净收益总和: ", .{});
    try fqm.formatUnaryDecimal(stdout, report.getTotalNetBenefit());
    std.debug.print("\n", .{});

    // ========== FQMCP 管线输出 ==========
    std.debug.print("\n=== 启动 fqmcp 管线 ===\n", .{});

    var fqmcp = fqm.FQMCP.init(allocator);

    const knml_path = "zig-stem/output/fqmet.knmll";
    std.fs.cwd().makePath("zig-stem/output") catch {};

    try fqmcp.processOrchestrator(&orch, knml_path);
    std.debug.print("knml 文件已保存: {s}\n", .{knml_path});

    // FQMY URI 列表
    std.debug.print("\n--- FQMY URI 列表 ---\n", .{});
    var encoder = fqm.FQMEncoder.init();
    for (stats) |container| {
        const uri = try encoder.encodeFQMY(allocator, &container);
        defer allocator.free(uri);
        std.debug.print("{s}\n", .{uri});
    }

    // knmlil 坐标演示
    std.debug.print("\n--- knmlil 三元坐标 ---\n", .{});

    const c0 = fqm.KnmlCoord.init(1, 0, 1);
    std.debug.print("mian|gu|ye = ", .{});
    try c0.encodeUnary(stdout);
    std.debug.print("\n", .{});

    const c1 = fqm.KnmlCoord.init(0, 1, 0);
    std.debug.print("mian|gu|ye = ", .{});
    try c1.encodeUnary(stdout);
    std.debug.print("\n", .{});

    std.debug.print("\n=== 演示结束 ===\n\n", .{});
}
