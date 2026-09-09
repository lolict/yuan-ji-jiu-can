// baflow.zig — 八流统一 (Eight-Flow Unification)
//
// 八流统一是意识路由的核心层，将八种意识流合并到同一接口：
// ★ 预加载流 (preload) — 意识初始化注入
// ★★ 解释流 (interpret) — 对输入的语义解码
// ★★★ 谐音路由 (homophone) — 用谐音映射实现路由（gttx://）
// ★★★★ 主权路由 (sovereign) — 产权流判定，谁拥有什么
// ★★★★★ 感知流 (perception) — 感官输入的极性判断
// ★★★★★★ 评价流 (evaluate) — 裁判器调用，merit 计算
// ★★★★★★★ 反馈流 (feedback) — 迂回反馈，反射感知
// ★★★★★★★★ 时间流 (temporal) — tick/cycle/相位推进

const std = @import("std");

// ============================================================
// 1. 流类型 (Flow Type)
// ============================================================
pub const FlowType = enum(u8) {
    preload = 1,
    interpret = 2,
    homophone = 3,
    sovereign = 4,
    perception = 5,
    evaluate = 6,
    feedback = 7,
    temporal = 8,

    pub fn star(self: FlowType) []const u8 {
        return switch (self) {
            .preload => "★",
            .interpret => "★★",
            .homophone => "★★★",
            .sovereign => "★★★★",
            .perception => "★★★★★",
            .evaluate => "★★★★★★",
            .feedback => "★★★★★★★",
            .temporal => "★★★★★★★★",
        };
    }

    pub fn name(self: FlowType) []const u8 {
        return switch (self) {
            .preload => "preload",
            .interpret => "interpret",
            .homophone => "homophone",
            .sovereign => "sovereign",
            .perception => "perception",
            .evaluate => "evaluate",
            .feedback => "feedback",
            .temporal => "temporal",
        };
    }
};

// 所有流枚举值常量数组
pub const ALL_FLOWS = [_]FlowType{
    .preload, .interpret, .homophone, .sovereign,
    .perception, .evaluate, .feedback, .temporal,
};

// ============================================================
// 2. 意识包 (Consciousness Packet)
// ============================================================
pub const CPacket = struct {
    src: u32,
    dst: u32,
    flow: FlowType,
    polarity: Polarity,
    payload: []const u8,
    tick: u64,
    witnessed: bool,

    pub fn init(src: u32, dst: u32, flow: FlowType, payload: []const u8) CPacket {
        return CPacket{
            .src = src,
            .dst = dst,
            .flow = flow,
            .polarity = .neutral,
            .payload = payload,
            .tick = 0,
            .witnessed = false,
        };
    }
};

pub const Polarity = enum {
    yin,
    yang,
    balanced,
    neutral,
};

// ============================================================
// 3. 流处理器 (Flow Handler)
// ============================================================
pub const FlowResult = struct {
    accepted: bool,
    transformed_payload: ?[]const u8,
    next_flow: ?FlowType,
    merit: i64,
    log: []const u8,
};

pub const FlowHandler = struct {
    self_flow: FlowType,

    pub fn init(flow: FlowType) FlowHandler {
        return FlowHandler{ .self_flow = flow };
    }

    pub fn handle(self: FlowHandler, packet: *const CPacket, allocator: std.mem.Allocator) !FlowResult {
        _ = allocator;

        switch (self.self_flow) {
            .preload => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = .interpret,
                    .merit = 5,
                    .log = "preload→interpret",
                };
            },
            .interpret => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = .homophone,
                    .merit = 8,
                    .log = "interpret→homophone",
                };
            },
            .homophone => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = "gttx://1",
                    .next_flow = .sovereign,
                    .merit = 10,
                    .log = "homophone→sovereign",
                };
            },
            .sovereign => {
                const owned = (packet.src == 1 or packet.src == 2);
                return FlowResult{
                    .accepted = owned,
                    .transformed_payload = if (owned) packet.payload else null,
                    .next_flow = if (owned) .perception else null,
                    .merit = if (owned) 12 else -15,
                    .log = if (owned) "sovereign→perception" else "sovereign:reject",
                };
            },
            .perception => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = .evaluate,
                    .merit = 7,
                    .log = "perception→evaluate",
                };
            },
            .evaluate => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = .feedback,
                    .merit = 15,
                    .log = "evaluate→feedback",
                };
            },
            .feedback => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = .temporal,
                    .merit = 6,
                    .log = "feedback→temporal",
                };
            },
            .temporal => {
                return FlowResult{
                    .accepted = true,
                    .transformed_payload = packet.payload,
                    .next_flow = null,
                    .merit = 3,
                    .log = "temporal:complete",
                };
            },
        }
    }
};

// ============================================================
// 4. 八流路由器 (BaFlow Router)
// ============================================================
pub const BaFlow = struct {
    allocator: std.mem.Allocator,
    handlers: [8]FlowHandler,
    log_entries: std.ArrayList([]const u8),
    total_merit: i64,
    tick: u64,

    pub fn init(allocator: std.mem.Allocator) BaFlow {
        var handlers: [8]FlowHandler = undefined;
        for (ALL_FLOWS, 0..) |ft, i| {
            handlers[i] = FlowHandler.init(ft);
        }
        return BaFlow{
            .allocator = allocator,
            .handlers = handlers,
            .log_entries = std.ArrayList([]const u8).init(allocator),
            .total_merit = 0,
            .tick = 0,
        };
    }

    pub fn deinit(self: *BaFlow) void {
        for (self.log_entries.items) |entry| {
            self.allocator.free(entry);
        }
        self.log_entries.deinit();
    }

    pub fn route(self: *BaFlow, packet: *const CPacket) !void {
        const current = packet;
        var current_flow = packet.flow;

        while (true) {
            const idx = @intFromEnum(current_flow) - 1;
            const result = try self.handlers[idx].handle(current, self.allocator);
            self.total_merit += result.merit;
            self.tick += 1;

            const entry = try std.fmt.allocPrint(self.allocator, "[{s}] {s} (merit={d})", .{
                current_flow.star(), result.log, result.merit,
            });
            try self.log_entries.append(entry);

            if (!result.accepted or result.next_flow == null) break;
            current_flow = result.next_flow.?;
        }
    }

    pub fn formatReport(self: *const BaFlow, allocator: std.mem.Allocator) ![]const u8 {
        var out = std.ArrayList(u8).init(allocator);
        defer out.deinit();

        try out.appendSlice("# 八流统一 路由报告\n");
        try out.appendSlice("flows: ★★★★★★★★★★ (8 types)\n");
        try out.appendSlice("tick: ");
        try formatUnary(&out, self.tick);
        try out.appendSlice("\ntotal_merit: ");
        try formatSigned(&out, self.total_merit);
        try out.appendSlice("\nroute_log: ");
        try formatUnary(&out, self.log_entries.items.len);
        try out.appendSlice("\n");

        for (self.log_entries.items, 0..) |entry, i| {
            try out.appendSlice("  ");
            try formatUnary(&out, i + 1);
            try out.appendSlice(". ");
            try out.appendSlice(entry);
            try out.appendSlice("\n");
        }

        try out.appendSlice("\neight_flow_bitmap:\n");
        for (ALL_FLOWS) |ft| {
            try out.appendSlice("  ");
            try out.appendSlice(ft.star());
            try out.appendSlice(" ");
            try out.appendSlice(ft.name());
            try out.appendSlice("\n");
        }

        return out.toOwnedSlice();
    }
};

// ============================================================
// 辅助
// ============================================================
fn formatUnary(writer: *std.ArrayList(u8), value: u64) !void {
    if (value == 0) {
        try writer.appendSlice("☆");
        return;
    }
    var r = value;
    while (r > 0) : (r -= 1) {
        try writer.appendSlice("★");
    }
}

fn formatSigned(writer: *std.ArrayList(u8), value: i64) !void {
    if (value < 0) {
        try writer.appendSlice("☆");
        var r: u64 = @intCast(-value);
        while (r > 0) : (r -= 1) {
            try writer.appendSlice("★");
        }
    } else if (value == 0) {
        try writer.appendSlice("☆");
    } else {
        var r: u64 = @intCast(value);
        while (r > 0) : (r -= 1) {
            try writer.appendSlice("★");
        }
    }
}

// ============================================================
// 测试
// ============================================================
test "baflow route full chain" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var baflow = BaFlow.init(allocator);
    defer baflow.deinit();

    var packet = CPacket.init(1, 2, .preload, "test_consciousness");

    try baflow.route(&packet);

    // 完整链路：preload→interpret→homophone→sovereign→perception→evaluate→feedback→temporal
    try std.testing.expectEqual(@as(usize, 8), baflow.log_entries.items.len);
    try std.testing.expectEqual(@as(i64, 66), baflow.total_merit); // 5+8+10+12+7+15+6+3
}

test "baflow report format" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var baflow = BaFlow.init(allocator);
    defer baflow.deinit();

    var packet = CPacket.init(1, 2, .homophone, "gttx://1");
    try baflow.route(&packet);

    const report = try baflow.formatReport(allocator);
    defer allocator.free(report);

    try std.testing.expect(std.mem.indexOf(u8, report, "八流统一") != null);
    try std.testing.expect(std.mem.indexOf(u8, report, "route_log:") != null);
}

test "baflow reject unauthorized" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var baflow = BaFlow.init(allocator);
    defer baflow.deinit();

    var packet = CPacket.init(99, 2, .sovereign, "unauthorized");
    try baflow.route(&packet);

    try std.testing.expectEqual(@as(usize, 1), baflow.log_entries.items.len);
    try std.testing.expectEqual(@as(i64, -15), baflow.total_merit);
}
