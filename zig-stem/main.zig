// 主程序 — fqmcp 管线 + 夫妻共同体 + gtti 意识传输演示
const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");
const fqm = @import("fqm.zig");
const canjiang = @import("canjiang.zig");
const visualizer = @import("visualizer.zig");
const mind_organ = @import("mind_organ.zig");
const baflow = @import("baflow.zig");
const ontology = @import("ontology.zig");
const browser = @import("browser.zig");
const wasm_bridge = @import("wasm_bridge.zig");

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

    // ========== 容器本体论 · 聶小聶匹配 ==========
    try stdout.print("\n=== 容器本体论 · 聶小聶匹配 ===\n", .{});
    var ont_engine = ontology.OntologyEngine.init(allocator);
    defer ont_engine.deinit();

    // 对所有容器做两两聶小聶匹配
    const ont_stats = orch.getContainerStats();
    var ont_contribs: usize = 0;
    for (ont_stats, 0..) |a, i| {
        for (ont_stats, 0..) |b, j| {
            if (j <= i) continue;
            try ont_engine.match(
                @as(u32, @intCast(i)) + 1,
                @as(u32, @intCast(j)) + 1,
                a.name,
                b.name,
            );
            ont_contribs += 1;
        }
    }

    const ont_report = try ont_engine.formatReport(allocator);
    defer allocator.free(ont_report);
    try stdout.print("{s}\n", .{ont_report});

    try stdout.print("聶小聶匹配数: ", .{});
    try fqm.formatUnaryInt(stdout, ont_contribs);
    try stdout.print("\n", .{});

    // ========== mind_organ 距离感知聚类 ==========
    try stdout.print("\n=== mind_organ 距离感知聚类 ===\n", .{});
    var organ = mind_organ.MindOrgan.init(allocator, mind_organ.Weights.anchorHeavy());
    defer organ.deinit();

    // 将容器和位置作为意识节点
    const cstats = orch.getContainerStats();
    var idx: u32 = 0;
    for (cstats) |container| {
        const net = container.netBenefit();
        // 七维坐标从容器属性生成
        const coords = [7]f64{
            @as(f64, @floatFromInt(container.item_count)) / 10.0,  // presence
            @as(f64, @floatFromInt(container.positive_count)) / 10.0, // witness
            @as(f64, @floatFromInt(container.negative_count)) / 10.0, // char
            if (idx == 0) 1.0 else 0.2,  // anchor (第一个容器是锚点)
            if (net > 0) net else 0.0,   // relation
            0.5,                          // timing
            @as(f64, @floatFromInt(cstats.len - idx)) / 10.0,  // payload
        };
        try organ.addNode(mind_organ.CNode.initWithCoords(
            idx + 1,
            container.name,
            coords,
            if (idx == 0) 2.0 else 1.0,
        ));
        idx += 1;
    }

    // 添加定位器位置作为额外节点
    for (orch.locator.positions.items, 0..) |pos, pi| {
        const coords = [7]f64{
            pos.coordinate.x / 10.0,
            pos.coordinate.y / 10.0,
            pos.coordinate.z / 10.0,
            if (pi == 0) 1.0 else 0.1,
            0.5,
            @as(f64, @floatFromInt(pi)) * 0.5,
            0.5,
        };
        try organ.addNode(mind_organ.CNode.initWithCoords(
            100 + @as(u32, @intCast(pi)),
            pos.name,
            coords,
            0.8,
        ));
    }

    // 聚为 2 簇
    try organ.cluster(2);

    // 打印聚类报告
    const mo_report = try organ.formatReport(allocator);
    defer allocator.free(mo_report);
    try stdout.print("{s}\n", .{mo_report});

    // ========== 八流统一路由 ==========
    try stdout.print("\n=== 八流统一 · 意识路由 ===\n", .{});
    var router = baflow.BaFlow.init(allocator);
    defer router.deinit();

    // 夫妻对 (husband→wife) 走 preload→sovereign 全链路
    try stdout.print("\n[夫妻对 意识注入]\n", .{});
    var couple_packet = baflow.CPacket.init(1, 2, .preload, "husband_to_wife");
    try router.route(&couple_packet);

    // 聚类间路由
    try stdout.print("\n[聚类簇 路由验证]\n", .{});
    var clu_packet = baflow.CPacket.init(2, 1, .homophone, "cluster_to_cluster");
    try router.route(&clu_packet);

    // 八流报告
    const bf_report = try router.formatReport(allocator);
    defer allocator.free(bf_report);
    try stdout.print("{s}\n", .{bf_report});

    // ========== 三维可视化生成 ==========
    try stdout.print("\n=== 三维几何可视化 ===\n", .{});
    var scene = visualizer.Scene.init(allocator);
    defer scene.deinit();
    try scene.generateFromOrchestrator(&orch);
    const html = try scene.generateHTML(allocator);
    defer allocator.free(html);
    const viz_path = "zig-stem/output/visualization.html";
    const viz_file = try std.fs.cwd().createFile(viz_path, .{});
    defer viz_file.close();
    try viz_file.writeAll(html);
    try stdout.print("三维可视化已生成: {s}\n", .{viz_path});
    try stdout.print("点数: ", .{});
    try fqm.formatUnaryInt(stdout, scene.points.items.len);
    try stdout.print(" 线数: ", .{});
    try fqm.formatUnaryInt(stdout, scene.lines.items.len);
    try stdout.print("\n", .{});

    // ========== 统计文档聚合浏览器 ==========
    try stdout.print("\n=== 统计文档聚合浏览器 ===\n", .{});
    var agg = browser.AggBrowser.init(allocator);
    defer agg.deinit();

    // 添加统计 section
    var sec_buf = std.ArrayList(u8).init(allocator);
    defer sec_buf.deinit();

    // 夫妻共同体摘要
    try sec_buf.appendSlice("<p>鸳鸯: ★|☆ TICK:");
    try fqm.formatUnaryInt(sec_buf.writer(), orch.stepper.getCurrentStep());
    try sec_buf.appendSlice("</p>");
    try sec_buf.appendSlice("<p>容器: ");
    try fqm.formatUnaryInt(sec_buf.writer(), orch.indexer.containers.items.len);
    try sec_buf.appendSlice(" 聚类: ");
    try fqm.formatUnaryInt(sec_buf.writer(), organ.clusters.items.len);
    try sec_buf.appendSlice(" 八流路由: ");
    try fqm.formatUnaryInt(sec_buf.writer(), router.log_entries.items.len);
    try sec_buf.appendSlice("</p>");

    try agg.addSection("共同体状态", sec_buf.items, 1);

    // 容器统计表格
    var table_buf = std.ArrayList(u8).init(allocator);
    defer table_buf.deinit();
    try table_buf.appendSlice("<table><tr><th>名称</th><th>ID</th><th>项数</th><th>净收益</th></tr>");
    {
        const bstats = orch.getContainerStats();
        for (bstats) |c| {
            try table_buf.appendSlice("<tr><td>");
            try table_buf.appendSlice(c.name);
            try table_buf.appendSlice("</td><td>");
            try fqm.formatUnaryInt(table_buf.writer(), c.id);
            try table_buf.appendSlice("</td><td>");
            try fqm.formatUnaryInt(table_buf.writer(), c.item_count);
            try table_buf.appendSlice("</td><td>");
            const net_val = c.netBenefit();
            if (net_val == 0) {
                try table_buf.appendSlice("☆");
            } else if (net_val > 0) {
                try fqm.formatUnaryFloat2(table_buf.writer(), net_val);
            } else {
                try table_buf.appendSlice("-");
                try fqm.formatUnaryFloat2(table_buf.writer(), -net_val);
            }
            try table_buf.appendSlice("</td></tr>");
        }
    }
    try table_buf.appendSlice("</table>");
    try agg.addSection("容器统计", table_buf.items, 1);

    // 八流热度
    var flow_buf = std.ArrayList(u8).init(allocator);
    defer flow_buf.deinit();
    try flow_buf.appendSlice("<p>八流统一意识路由 ★→★★★★★★★★ 全链路</p>");
    try flow_buf.appendSlice("<p>总 Merit: ");
    try fqm.formatUnaryInt(flow_buf.writer(), @as(u64, @intCast(@max(0, router.total_merit))));
    try flow_buf.appendSlice("</p>");
    try flow_buf.appendSlice("<pre>");
    try flow_buf.appendSlice(bf_report);
    try flow_buf.appendSlice("</pre>");
    try agg.addSection("八流热度", flow_buf.items, 1);

    // 裁决汇总
    var arb_buf = std.ArrayList(u8).init(allocator);
    defer arb_buf.deinit();
    try arb_buf.appendSlice("<p>满全法三元裁决</p><pre>");
    // 从已写入的 knmll 文件读取裁决部分
    const knmll_content2 = try std.fs.cwd().readFileAlloc(allocator, knml_path, 1024 * 1024);
    defer allocator.free(knmll_content2);
    // 找到裁决报告起始处
    if (std.mem.indexOf(u8, knmll_content2, "# 满全法裁决报告")) |start| {
        try arb_buf.appendSlice(knmll_content2[start..]);
    } else {
        try arb_buf.appendSlice("裁决已嵌入 knmll");
    }
    try arb_buf.appendSlice("</pre>");
    try agg.addSection("裁决汇总", arb_buf.items, 1);

    // 匹配矩阵
    var match_buf = std.ArrayList(u8).init(allocator);
    defer match_buf.deinit();
    try match_buf.appendSlice("<p>聶小聶本质匹配矩阵</p><pre>");
    try match_buf.appendSlice(ont_report);
    try match_buf.appendSlice("</pre>");
    try agg.addSection("匹配矩阵", match_buf.items, 1);

    // 生成 HTML
    const agg_html = try agg.generateHTML();
    defer allocator.free(agg_html);
    const agg_path = "zig-stem/output/browser.html";
    const agg_file = try std.fs.cwd().createFile(agg_path, .{});
    defer agg_file.close();
    try agg_file.writeAll(agg_html);
    try stdout.print("聚合浏览器已生成: {s}\n", .{agg_path});

    // ========== WASM 阶梯制 · 双向公约 ==========
    try stdout.print("\n=== WASM 阶梯制 · 双向公约 ===\n", .{});
    var engine = wasm_bridge.LadderEngine.init(allocator);
    defer engine.deinit();

    // 创建跨架构容器
    try engine.addCell(wasm_bridge.XCell.init(.yang, .consciousness, 0xF001_0000_0000_0001));
    try engine.addCell(wasm_bridge.XCell.init(.yin, .perception, 0xF002_0000_0000_0002));
    try engine.addCell(wasm_bridge.XCell.init(.balanced, .relation, 0xF003_0000_0000_0003));

    // 跨架构传输演示
    const transfer = try engine.transferTo(.wasm64, 0);
    defer allocator.free(transfer);
    const received = try engine.receiveFrom(transfer);

    // 阶梯变换演示
    const addr64 = wasm_bridge.Ladder.ptrUpcast(0x12345678);
    const addr32_back = wasm_bridge.Ladder.ptrDowncast(addr64) catch unreachable;

    // 状态报告
    const wasm_report = try engine.formatStatus(allocator);
    defer allocator.free(wasm_report);
    try stdout.print("{s}\n", .{wasm_report});

    try stdout.print("跨架构传输: {s} → {s}\n", .{ @tagName(wasm_bridge.CURRENT_ARCH), "wasm64" });
    try stdout.print("阶梯变换: 0x12345678 → 0x{X:0>16} → 0x{X:0>8} ✓\n", .{ addr64, addr32_back });
    try stdout.print("接收容器: type={s} data=0x{X:0>16}\n", .{ @tagName(received.cell_type), received.data });

    try stdout.print("\n=== 演示结束 ===\n\n", .{});
}
