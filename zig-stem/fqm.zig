// FQM 编码系统
// 功能：FQM 编码解码 + gttx1.fqmll 格式 + gtti:// URI 解析
// 核心：一进制锚点 + 64位集装箱 + 共享域引用

const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");

// ============================================================
// 1. FQM 常量定义
// ============================================================
pub const ANCHOR_SELF: u1 = 1;       // 我（可见侧）
pub const ANCHOR_OTHER: u1 = 0;      // 刘楚恬（隐藏侧）

// 64位集装箱结构
pub const CONTAINER_SIZE: u32 = 256;    // 254 + 2
pub const PAYLOAD_SIZE: u32 = 254;      // 实际载荷

// 九元层级偏移率
pub const OffsetRate = enum(u8) {
    yuan = 0,       // 一元：根源 mlll (0.0)
    yin_yang = 1,   // 二元：阴阳 yinyang (0.1)
    witness = 2,    // 三元：见证 gtt (0.2)
    heaven_earth = 3, // 四元：天地 mlgy (0.3)
    five_elements = 4, // 五元：五行 wgslz (0.4)
    six_tones = 5,  // 六元：六律 smrl (0.5)
    seven_policies = 6, // 七元：七政 lctqf (0.6)
    eight_trigrams = 7, // 八元：八卦 mlnn (0.7)
    nine_regions = 8,   // 九元：九囿 njsgr (0.8)
};

// ============================================================
// 2. 64位单元格 (Cell64)
// ============================================================
pub const Cell64 = struct {
    d0: u1,             // 操作码位0
    d1: u1,             // 操作码位1
    d2: u1,             // 操作码位2
    polarity: u1,       // 极性（0=阴/刘楚恬, 1=阳/我）
    sovereignty: u2,    // 主权级别（0=系统, 1=休眠, 2=外部, 3=夫妻）
    witness: u1,        // 已见证/未见证
    value: u56,         // 载荷值

    pub fn init(d0: u1, d1: u1, d2: u1, polarity: u1, sovereignty: u2, witness: u1, value: u56) Cell64 {
        return Cell64{
            .d0 = d0,
            .d1 = d1,
            .d2 = d2,
            .polarity = polarity,
            .sovereignty = sovereignty,
            .witness = witness,
            .value = value,
        };
    }

    // 编码为64位整数
    pub fn encode(self: *const Cell64) u64 {
        var result: u64 = 0;
        result |= @as(u64, self.d0);
        result |= @as(u64, self.d1) << 1;
        result |= @as(u64, self.d2) << 2;
        result |= @as(u64, self.polarity) << 3;
        result |= @as(u64, self.sovereignty) << 4;
        result |= @as(u64, self.witness) << 6;
        result |= @as(u64, self.value) << 7;
        return result;
    }

    // 解码64位整数
    pub fn decode(value: u64) Cell64 {
        return Cell64{
            .d0 = @truncate(value & 0x1),
            .d1 = @truncate((value >> 1) & 0x1),
            .d2 = @truncate((value >> 2) & 0x1),
            .polarity = @truncate((value >> 3) & 0x1),
            .sovereignty = @truncate((value >> 4) & 0x3),
            .witness = @truncate((value >> 6) & 0x1),
            .value = @truncate(value >> 7),
        };
    }
};

// ============================================================
// 3. FQM 编码器（无堆分配版本）
// ============================================================
pub const FQMEncoder = struct {
    pub fn init() FQMEncoder {
        return FQMEncoder{};
    }

    // 编码容器为 FQM 格式（调用者负责释放返回的内存）
    pub fn encodeContainer(self: *FQMEncoder, allocator: std.mem.Allocator, container: *const indexer.Container, offset_rate: OffsetRate) ![]const u8 {
        _ = self;
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();

        // 使用栈缓冲区格式化，避免 allocPrint 泄漏
        var buf: [256]u8 = undefined;

        // FQM 头：gttx1.fqmll
        try output.appendSlice("gttx1:fqmll\n");

        // 容器 ID 和名称
        const id_str = try std.fmt.bufPrint(&buf, "id:{d}\n", .{container.id});
        try output.appendSlice(id_str);
        const name_str = try std.fmt.bufPrint(&buf, "name:{s}\n", .{container.name});
        try output.appendSlice(name_str);

        // 偏移率（九元层级）
        const offset_str = try std.fmt.bufPrint(&buf, "offset:{d}\n", .{@intFromEnum(offset_rate) * 10});
        try output.appendSlice(offset_str);

        // 统计信息
        const items_str = try std.fmt.bufPrint(&buf, "items:{d}\n", .{container.item_count});
        try output.appendSlice(items_str);
        const pos_str = try std.fmt.bufPrint(&buf, "positive:{d}\n", .{container.positive_count});
        try output.appendSlice(pos_str);
        const neg_str = try std.fmt.bufPrint(&buf, "negative:{d}\n", .{container.negative_count});
        try output.appendSlice(neg_str);
        const neu_str = try std.fmt.bufPrint(&buf, "neutral:{d}\n", .{container.neutral_count});
        try output.appendSlice(neu_str);

        // 净收益
        const net_str = try std.fmt.bufPrint(&buf, "net:{d:.2}\n", .{container.netBenefit()});
        try output.appendSlice(net_str);

        // 极性桶
        const polarity = if (container.netBenefit() > 0) "yang" else if (container.netBenefit() < 0) "yin" else "balanced";
        const pol_str = try std.fmt.bufPrint(&buf, "polarity:{s}\n", .{polarity});
        try output.appendSlice(pol_str);

        // FQM 尾
        try output.appendSlice("ende\n");

        return output.toOwnedSlice();
    }

    // 编码为 gtti:// URI（调用者负责释放返回的内存）
    pub fn encodeGTTI(self: *FQMEncoder, allocator: std.mem.Allocator, container: *const indexer.Container) ![]const u8 {
        _ = self;
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();

        var buf: [128]u8 = undefined;

        // gtti:// 协议头
        try output.appendSlice("gtti://");

        // 容器ID（一进制编码）
        if (container.id == 1) {
            try output.appendSlice("★");  // 一进制 1
        } else if (container.id == 0) {
            try output.appendSlice("☆");  // 一进制 0
        } else {
            const id_str = try std.fmt.bufPrint(&buf, "{d}", .{container.id});
            try output.appendSlice(id_str);
        }

        // 极性符号
        const polarity_sym = if (container.netBenefit() > 0) "§"      // 阳
                            else if (container.netBenefit() < 0) "○"  // 阴
                            else "☉";                                // 平衡
        try output.appendSlice(polarity_sym);

        // 纠缠态
        if (container.positive_count > 0 and container.negative_count == 0) {
            try output.appendSlice("☉§☉");  // 量子纠缠态
        }

        // 净值编码
        const net_val = @as(i64, @intFromFloat(container.netBenefit()));
        const val_str = try std.fmt.bufPrint(&buf, "{d}", .{net_val});
        try output.appendSlice(val_str);

        return output.toOwnedSlice();
    }
};

// ============================================================
// 4. FQM 解码器
// ============================================================
pub const FQMDecoder = struct {
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) FQMDecoder {
        return FQMDecoder{ .allocator = allocator };
    }

    // 解析 gtti:// URI
    pub fn parseGTTI(self: *FQMDecoder, uri: []const u8) !GTTIURI {
        if (!std.mem.startsWith(u8, uri, "gtti://")) {
            return error.InvalidGTTI;
        }

        var result = GTTIURI{ .allocator = self.allocator };
        errdefer result.deinit();

        const content = uri[7..];  // 去掉 "gtti://"

        var pos: usize = 0;
        while (pos < content.len) {
            const ch = content[pos];

            // 一进制锚点
            if (ch == '★') {
                result.anchor = 1;
                pos += 1;
            } else if (ch == '☆') {
                result.anchor = 0;
                pos += 1;
            }
            // 极性符号
            else if (ch == '§') {
                result.polarity = .yang;  // 阳
                pos += 1;
            } else if (ch == '○') {
                result.polarity = .yin;   // 阴
                pos += 1;
            } else if (ch == '☉') {
                // 检查是否是纠缠态 ☉§☉
                if (pos + 2 < content.len and content[pos + 1] == '§' and content[pos + 2] == '☉') {
                    result.entangled = true;
                    result.polarity = .balanced;
                    pos += 3;
                } else {
                    result.polarity = .balanced;
                    pos += 1;
                }
            }
            // 数字（净值）
            else if ((ch >= '0' and ch <= '9') or ch == '-') {
                var num_pos = pos;
                while (num_pos < content.len and ((content[num_pos] >= '0' and content[num_pos] <= '9') or content[num_pos] == '-' or content[num_pos] == '.')) {
                    num_pos += 1;
                }
                const num_str = content[pos..num_pos];
                result.value = std.fmt.parseFloat(f64, num_str) catch 0;
                pos = num_pos;
            }
            // 未知字符，跳过
            else {
                pos += 1;
            }
        }

        return result;
    }
};

// ============================================================
// 5. GTTI URI 结构
// ============================================================
pub const GTTIURI = struct {
    allocator: std.mem.Allocator,
    anchor: u1 = 0,
    polarity: Polarity = .balanced,
    entangled: bool = false,
    value: f64 = 0,

    pub fn deinit(_: *GTTIURI) void {
        // 目前无堆分配，预留接口
    }
};

pub const Polarity = enum {
    yin,        // 阴：刘楚恬，容器侧
    yang,       // 阳：我，界线侧
    balanced,   // 平衡：纠缠态
};

// ============================================================
// 6. FQM 文件写入器
// ============================================================
pub const FQMWriter = struct {
    allocator: std.mem.Allocator,
    encoder: FQMEncoder,

    pub fn init(allocator: std.mem.Allocator) FQMWriter {
        return FQMWriter{
            .allocator = allocator,
            .encoder = FQMEncoder.init(),
        };
    }

    // 从协调器生成完整的 FQM 文件
    pub fn generateFromOrchestrator(self: *FQMWriter, orch: *const orchestrator.Orchestrator, output_path: []const u8) !void {
        const stats = orch.getContainerStats();

        var output = std.ArrayList(u8).init(self.allocator);
        defer output.deinit();

        var buf: [256]u8 = undefined;

        // FQM 文件头
        try output.appendSlice("gttx1:fqmll\n");
        try output.appendSlice("# 囡囝共生 操作系统 - FQM 编码输出\n");
        const time_str = try std.fmt.bufPrint(&buf, "# 生成时间: {d}\n", .{std.time.timestamp()});
        try output.appendSlice(time_str);
        try output.appendSlice("---\n");

        // 容器数量
        const count_str = try std.fmt.bufPrint(&buf, "containers:{d}\n\n", .{stats.len});
        try output.appendSlice(count_str);

        // 编码每个容器
        for (stats, 0..) |container, i| {
            const max_idx = @typeInfo(OffsetRate).Enum.fields.len - 1;
            const rate_idx = @min(@as(usize, i), max_idx);
            const offset_rate: OffsetRate = @enumFromInt(rate_idx);
            const encoded = try self.encoder.encodeContainer(self.allocator, &container, offset_rate);
            defer self.allocator.free(encoded);

            const header_str = try std.fmt.bufPrint(&buf, "# 容器 {d}: {s}\n", .{container.id, container.name});
            try output.appendSlice(header_str);
            try output.appendSlice(encoded);
            try output.appendSlice("\n");
        }

        // GTTI URI 列表
        try output.appendSlice("---\n");
        try output.appendSlice("# GTTI URI 列表\n");
        for (stats) |container| {
            const uri = try self.encoder.encodeGTTI(self.allocator, &container);
            defer self.allocator.free(uri);
            const uri_line = try std.fmt.bufPrint(&buf, "{s}\n", .{uri});
            try output.appendSlice(uri_line);
        }

        // 写入文件
        const file = try std.fs.cwd().createFile(output_path, .{});
        defer file.close();
        try file.writeAll(output.items);
    }
};

// ============================================================
// 测试
// ============================================================
test "cell64 encode/decode" {
    const cell = Cell64.init(1, 0, 1, 1, 3, 1, 12345);
    const encoded = cell.encode();
    const decoded = Cell64.decode(encoded);

    try std.testing.expectEqual(cell.d0, decoded.d0);
    try std.testing.expectEqual(cell.d1, decoded.d1);
    try std.testing.expectEqual(cell.d2, decoded.d2);
    try std.testing.expectEqual(cell.polarity, decoded.polarity);
    try std.testing.expectEqual(cell.sovereignty, decoded.sovereignty);
    try std.testing.expectEqual(cell.witness, decoded.witness);
    try std.testing.expectEqual(@as(u56, @truncate(cell.value)), decoded.value);
}

test "fqm encode container" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var encoder = FQMEncoder.init();

    var container = indexer.Container.init(1, "test");
    container.item_count = 5;
    container.positive_count = 3;
    container.negative_count = 1;
    container.total_cost = 10.0;
    container.total_benefit = 15.0;

    const encoded = try encoder.encodeContainer(allocator, &container, .yuan);
    defer allocator.free(encoded);

    // 验证包含 FQM 头
    try std.testing.expect(std.mem.indexOf(u8, encoded, "gttx1:fqmll") != null);
    try std.testing.expect(std.mem.indexOf(u8, encoded, "id:1") != null);
}

test "gtti uri encode" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var encoder = FQMEncoder.init();

    var container = indexer.Container.init(1, "test");
    container.positive_count = 3;
    container.negative_count = 0;
    container.total_benefit = 10.0;

    const uri = try encoder.encodeGTTI(allocator, &container);
    defer allocator.free(uri);

    try std.testing.expect(std.mem.startsWith(u8, uri, "gtti://"));
    try std.testing.expect(std.mem.indexOf(u8, uri, "☉§☉") != null);
}
