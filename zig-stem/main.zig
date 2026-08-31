// 主程序 - 运行协调器演示
const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");
const phaser = @import("phaser.zig");
const fqm = @import("fqm.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("\n=== 囡囝共生 操作系统 - FQM 输出 ===\n\n", .{});

    // 创建协调器配置
    var config = orchestrator.OrchestratorConfig.default();
    config.total_steps = 10;
    config.step_mode = .continuous;

    std.debug.print("初始化所有模块...\n", .{});

    // 初始化协调器
    var orch = try orchestrator.Orchestrator.init(allocator, config);
    defer orch.deinit();

    // 添加测试数据
    try orch.indexer.addMenuItem(indexer.MenuItem.init(1, "red", 1, 10.0, 15.0, .positive, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(2, "blue", 2, 8.0, 12.0, .positive, 1));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(3, "ai", 3, 20.0, 18.0, .negative, 2));
    try orch.indexer.addMenuItem(indexer.MenuItem.init(4, "ao", 4, 5.0, 8.0, .neutral, 2));

    // 添加相位状态
    try orch.phaser.addPhaseState(phaser.PhaseState.init(0.0, 0.5));
    try orch.phaser.addPhaseState(phaser.PhaseState.init(0.3, 0.7));

    std.debug.print("所有模块初始化完成！\n\n", .{});

    // 启动
    try orch.start();
    std.debug.print("协调器已启动，开始运行...\n\n", .{});

    // 运行
    var step_count: u64 = 0;
    while (!orch.isComplete()) {
        _ = try orch.step();
        step_count += 1;

        const progress = orch.getProgress();
        const align_status = orch.getAlignmentStatus();

        std.debug.print("步骤 {d}: 进度 {d:.1}%, 对齐置信度 {d:.2}\n", .{
            step_count,
            progress * 100.0,
            align_status.confidence,
        });
    }

    std.debug.print("\n=== 运行完成 ===\n", .{});
    std.debug.print("总步骤: {d}\n", .{step_count});

    // 打印容器统计
    std.debug.print("\n--- 容器统计 ---\n", .{});
    const stats = orch.getContainerStats();
    for (stats) |container| {
        std.debug.print("容器 {s}: 项数={d}, 正因果={d}, 负因果={d}, 净收益={d:.2}\n", .{
            container.name,
            container.item_count,
            container.positive_count,
            container.negative_count,
            container.netBenefit(),
        });
    }

    // 打印标量报告
    std.debug.print("\n--- 标量报告 ---\n", .{});

    var report = try orch.indexer.generateScalarReport();
    defer report.deinit();

    std.debug.print("净收益总和: {d:.2}\n", .{report.getTotalNetBenefit()});

    // ========== FQM 输出（替代 HTML） ==========
    std.debug.print("\n=== 生成 FQM 输出 ===\n", .{});

    // 创建 FQM 写入器
    var writer = fqm.FQMWriter.init(allocator);

    const fqm_output_path = "zig-stem/output/gttx1.fqmll";
    std.fs.cwd().makePath("zig-stem/output") catch {};

    try writer.generateFromOrchestrator(&orch, fqm_output_path);
    std.debug.print("FQM 文件已保存到: {s}\n", .{fqm_output_path});

    // 同时输出 GTTI URI 到控制台
    std.debug.print("\n--- GTTI URI 列表 ---\n", .{});
    var encoder = fqm.FQMEncoder.init();
    for (stats) |container| {
        const uri = try encoder.encodeGTTI(allocator, &container);
        defer allocator.free(uri);
        std.debug.print("{s}\n", .{uri});
    }

    std.debug.print("\n=== 演示结束 ===\n\n", .{});
}
