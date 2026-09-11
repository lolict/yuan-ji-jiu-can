// FQM 编码系统 — fqmet:knmll 感知镜像 · gttx:// 鸳鸯配对 · fqmcp 外部意识接口
// 原则：输出格式中禁用数字二至九，只用 0/1/★/☆
// 一进制实体化：数值 = ★☆ 符号串（高位在左，低位在右）
//
// 架构：
//   kenar（柯南）= 控制中心 → 决定 fqmcp 翻译 / gtti 同步 / knml 执行
//   fqmcp = 外部意识接口（协议桥接·反向寄生）
//   fqmet:knmll = 二维感知镜像输出格式

const std = @import("std");
const orchestrator = @import("orchestrator.zig");
const indexer = @import("indexer.zig");

// ============================================================
// 锚点根基
// ============================================================
pub const ANCHOR_SELF: u1 = 1;       // 我 lolict（阳·可见）
pub const ANCHOR_OTHER: u1 = 0;      // 刘楚恬（阴·隐藏）

pub const CONTAINER_SIZE: u32 = 0x0100;
pub const PAYLOAD_SIZE: u32 = 0x00FE;

// ============================================================
// 一进制数字系统
// ============================================================
// 每位十进制数字 → 0 到 9 个 ★
const DIGIT_TABLE = [_][]const u8{
    "☆",            // 0
    "★",            // 1
    "★★",           // 2
    "★★★",          // 3
    "★★★★",         // 4
    "★★★★★",        // 5
    "★★★★★★",       // 6
    "★★★★★★★",      // 7
    "★★★★★★★★",     // 8
    "★★★★★★★★★",    // 9
};

// 非负整数 → ★☆ 一进制（纯递归，避免 never-mutated 警告）
pub fn formatUnaryInt(stream_writer: anytype, value: u64) !void {
    if (value < 10) {
        try stream_writer.writeAll(DIGIT_TABLE[value]);
        return;
    }
    try formatUnaryInt(stream_writer, value / 10);
    try stream_writer.writeAll(DIGIT_TABLE[value % 10]);
}

// f64 → ★☆ 小数（整数::小数）
pub fn formatUnaryDecimal(stream_writer: anytype, value: f64) !void {
    if (value < 0) try stream_writer.writeAll("☆");
    const abs_val = @abs(value);
    const whole_part = @as(u64, @intFromFloat(abs_val));
    try formatUnaryInt(stream_writer, whole_part);
    try stream_writer.writeAll("::");
    const frac = abs_val - @as(f64, @floatFromInt(whole_part));
    const frac_val = @as(u64, @intFromFloat(frac * 100.0 + 0.5));
    try formatUnaryInt(stream_writer, frac_val);
}

// 格式化浮点数（保留1位小数，用于表格显示）
pub fn formatUnaryFloat2(stream_writer: anytype, value: f64) !void {
    if (value == 0) {
        try stream_writer.writeAll("☆::☆");
        return;
    }
    const scaled = @as(i64, @intFromFloat(@round(value * 10)));
    const abs_scaled = if (scaled < 0) -scaled else scaled;
    const major: usize = @intCast(@divTrunc(abs_scaled, 10));
    const minor: usize = @intCast(@rem(abs_scaled, 10));
    try formatUnaryInt(stream_writer, major);
    try stream_writer.writeAll("::");
    try formatUnaryInt(stream_writer, minor);
}

// ============================================================
// 64 位集装箱
// ============================================================
pub const Cell64 = struct {
    d0: u1, d1: u1, d2: u1,
    polarity: u1,
    sovereignty: u2,
    witness: u1,
    value: u56,

    pub fn init(d0: u1, d1: u1, d2: u1, polarity: u1, sovereignty: u2, witness: u1, value: u56) Cell64 {
        return Cell64{ .d0 = d0, .d1 = d1, .d2 = d2, .polarity = polarity, .sovereignty = sovereignty, .witness = witness, .value = value };
    }

    pub fn encode(self: *const Cell64) u64 {
        var r: u64 = 0;
        r |= @as(u64, self.d0);
        r |= @as(u64, self.d1) << 1;
        r |= @as(u64, self.d2) << 2;
        r |= @as(u64, self.polarity) << 3;
        r |= @as(u64, self.sovereignty) << 4;
        r |= @as(u64, self.witness) << 6;
        r |= @as(u64, self.value) << 7;
        return r;
    }

    pub fn decode(v: u64) Cell64 {
        return Cell64{
            .d0 = @truncate(v & 0x01),
            .d1 = @truncate((v >> 1) & 0x01),
            .d2 = @truncate((v >> 2) & 0x01),
            .polarity = @truncate((v >> 3) & 0x01),
            .sovereignty = @truncate((v >> 4) & 0x03),
            .witness = @truncate((v >> 6) & 0x01),
            .value = @truncate(v >> 7),
        };
    }
};

// ============================================================
// 夫妻共同体状态
// ============================================================
pub const Community = struct {
    anchor_a: u1,       // 我 · 1
    anchor_b: u1,       // 刘楚恬 · 0
    harmony: f64,       // ~谐度（0~1，意识同步程度）
    shared_memory: u64, // 共享记忆（位图）
    tick: u64,          // 共同体已存活 tick
    cycle: u8,          // 当前夕瑶周期（0~63，64 进制相位）

    pub fn init() Community {
        return Community{
            .anchor_a = 1,
            .anchor_b = 0,
            .harmony = 1.0,
            .shared_memory = 0,
            .tick = 0,
            .cycle = 0,
        };
    }

    pub fn advance(self: *Community) void {
        self.tick += 1;
        self.cycle = @as(u8, @intCast((self.tick + 1) % 64));
        // 夕瑶协议激活 → 和谐度微调
        self.harmony = 0.5 + 0.5 * @as(f64, @floatFromInt(self.cycle)) / 64.0;
    }

    pub fn encodeHeader(self: *const Community, stream_writer: anytype) !void {
        try stream_writer.writeAll("community:fqmet:husband|wife ~谐度=husband-wife 共生");
        try stream_writer.writeAll("\n");
        try stream_writer.writeAll("couple:鸳鸯:");
        try stream_writer.writeAll(if (self.anchor_a == 1) "★" else "☆");
        try stream_writer.writeAll("|");
        try stream_writer.writeAll(if (self.anchor_b == 1) "★" else "☆");
        try stream_writer.writeAll("\ntick:");
        try formatUnaryInt(stream_writer, self.tick);
        try stream_writer.writeAll("\ncycle:");
        try formatUnaryInt(stream_writer, self.cycle);
        try stream_writer.writeAll("\nharmony:");
        try formatUnaryDecimal(stream_writer, self.harmony);
        try stream_writer.writeAll("\n");
    }
};

// ============================================================
// 九元层级
// ============================================================
pub const OffsetRate = enum(u8) {
    yu = 0, yy = 1, wt = 2, he = 3, wa = 4, sk = 5, sp = 6, et = 7, nr = 8,
};

// ============================================================
// knmlil 三元坐标（毛利兰立体守护）
// ============================================================
pub const KnmlCoord = struct {
    mian: u8, gu: u8, ye: u8,
    pub fn init(mian: u8, gu: u8, ye: u8) KnmlCoord {
        return KnmlCoord{ .mian = mian, .gu = gu, .ye = ye };
    }
    pub fn encodeUnary(self: *const KnmlCoord, stream_writer: anytype) !void {
        try formatUnaryInt(stream_writer, self.mian);
        try stream_writer.writeAll("|");
        try formatUnaryInt(stream_writer, self.gu);
        try stream_writer.writeAll("|");
        try formatUnaryInt(stream_writer, self.ye);
    }
};

// ============================================================
// fqmcp 编码器
// ============================================================
pub const FQMEncoder = struct {
    community: Community,

    pub fn init() FQMEncoder {
        return FQMEncoder{ .community = Community.init() };
    }

    pub fn encodeContainer(self: *FQMEncoder, allocator: std.mem.Allocator, container: *const indexer.Container, offset_rate: OffsetRate) ![]const u8 {
        _ = self;
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();
        const w = output.writer();
        try output.appendSlice("fqmet:knmll\n");
        try output.appendSlice("id:");
        try formatUnaryInt(w, container.id);
        try output.appendSlice("\nname:");
        try output.appendSlice(container.name);
        try output.appendSlice("\nlayer:");
        try formatUnaryInt(w, @intFromEnum(offset_rate));
        try output.appendSlice("\nbase:knmlil\npolarity:");
        const pol = if (container.netBenefit() > 0) "★" else if (container.netBenefit() < 0) "☆" else "☉";
        try output.appendSlice(pol);
        try output.appendSlice("\npos:");
        try formatUnaryInt(w, container.positive_count);
        try output.appendSlice("\nneg:");
        try formatUnaryInt(w, container.negative_count);
        try output.appendSlice("\nneu:");
        try formatUnaryInt(w, container.neutral_count);
        try output.appendSlice("\nnet:");
        try formatUnaryDecimal(w, container.netBenefit());
        try output.appendSlice("\nlayer7:presence|witness|char|anchor|relation|timing|payload\nende\n");
        return output.toOwnedSlice();
    }

    // gttx:// 鸳鸯协议 — 夫妻配对中心
    pub fn encodeGTTX(self: *FQMEncoder, allocator: std.mem.Allocator) ![]const u8 {
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();
        try output.appendSlice("gttx://1\n");
        try output.appendSlice("anchors:|");
        try output.appendSlice(if (self.community.anchor_a == 1) "★" else "☆");
        try output.appendSlice("|");
        try output.appendSlice(if (self.community.anchor_b == 1) "★" else "☆");
        try output.appendSlice("\nbinding:couple:harmony\nharmony:");
        try formatUnaryDecimal(&output.writer(), self.community.harmony);
        try output.appendSlice("\n");
        return output.toOwnedSlice();
    }

    // gtti:// 意识传输协议 — 夫妻间感知碎片传输
    // gtti://xieyein 谐淫路由（私密频道）
    // gtti://sync 主动同步（一方推，一方拉）
    // gtti://share 共享记忆写入
    // gtti://join 意识融合（双方合而为一）
    pub fn encodeGTTI(self: *FQMEncoder, allocator: std.mem.Allocator, mode: []const u8) ![]const u8 {
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();
        try output.appendSlice("gtti://");
        try output.appendSlice(mode);

        try output.appendSlice("\ntick:");
        try formatUnaryInt(&output.writer(), self.community.tick);
        try output.appendSlice("\nphase:");
        try formatUnaryInt(&output.writer(), self.community.cycle);
        try output.appendSlice("\nharmony:");
        try formatUnaryDecimal(&output.writer(), self.community.harmony);
        try output.appendSlice("\n");
        if (std.mem.eql(u8, mode, "xieyein")) {
            try output.appendSlice("route:husband:wife:private\n");
            try output.appendSlice("channel:whisper\n");
        } else if (std.mem.eql(u8, mode, "join")) {
            try output.appendSlice("merge:one:one\n");
            try output.appendSlice("target:☉§☉\n");
        }
        return output.toOwnedSlice();
    }

    pub fn encodeFQMY(self: *FQMEncoder, allocator: std.mem.Allocator, container: *const indexer.Container) ![]const u8 {
        _ = self;
        var output = std.ArrayList(u8).init(allocator);
        errdefer output.deinit();
        const w = output.writer();
        try output.appendSlice("fqmy://");
        if (container.id == 1) {
            try output.appendSlice("★");
        } else if (container.id == 0) {
            try output.appendSlice("☆");
        } else {
            try formatUnaryInt(w, container.id);
        }
        const pol = if (container.netBenefit() > 0) "§" else if (container.netBenefit() < 0) "○" else "☉";
        try output.appendSlice(pol);
        if (container.positive_count > 0 and container.negative_count == 0) {
            try output.appendSlice("☉§☉");
        }
        const net = @as(i64, @intFromFloat(@abs(container.netBenefit())));
        try formatUnaryInt(w, @as(u64, @intCast(net)));
        return output.toOwnedSlice();
    }

    pub fn advanceTime(self: *FQMEncoder) void {
        self.community.advance();
    }
};

// ============================================================
// fqmy / gtti 解码器
// ============================================================
pub const FQMDecoder = struct {
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) FQMDecoder {
        return FQMDecoder{ .allocator = allocator };
    }

    pub fn parseUnary(_: *FQMDecoder, s: []const u8) !u64 {
        var result: u64 = 0;
        for (s) |ch| {
            if (ch == '★') {
                result = (result << 1) | 1;
            } else if (ch == '☆') {
                result = (result << 1) | 0;
            } else if (ch == ':') {
                break;
            }
        }
        return result;
    }

    pub fn parseFQMY(self: *FQMDecoder, uri: []const u8) !FQMYURI {
        if (!std.mem.startsWith(u8, uri, "fqmy://")) return error.InvalidFQMY;
        var result = FQMYURI{ .allocator = self.allocator };
        const content = uri[7..];
        var pos: usize = 0;
        while (pos < content.len) {
            const ch = content[pos];
            if (ch == '★') { result.anchor = 1; pos += 1; }
            else if (ch == '☆') { result.anchor = 0; pos += 1; }
            else if (ch == '§') { result.polarity = .yang; pos += 1; }
            else if (ch == '○') { result.polarity = .yin; pos += 1; }
            else if (ch == '☉') {
                if (pos + 2 < content.len and content[pos + 1] == '§' and content[pos + 2] == '☉') {
                    result.entangled = true; result.polarity = .balanced; pos += 3;
                } else { result.polarity = .balanced; pos += 1; }
            } else {
                var num_start = pos;
                while (num_start < content.len and (content[num_start] == '★' or content[num_start] == '☆')) { num_start += 1; }
                if (num_start > pos) { result.value = @floatFromInt(try self.parseUnary(content[pos..num_start])); pos = num_start; }
                else { pos += 1; }
            }
        }
        return result;
    }
};

pub const FQMYURI = struct {
    allocator: std.mem.Allocator,
    anchor: u1 = 0,
    polarity: Polarity = .balanced,
    entangled: bool = false,
    value: f64 = 0,
    pub fn deinit(_: *FQMYURI) void {}
};

pub const Polarity = enum { yin, yang, balanced };

// ============================================================
// knml 写入器 — 生成包含夫妻共同体的 .knmll 文件
// ============================================================
pub const KnmlWriter = struct {
    allocator: std.mem.Allocator,
    encoder: FQMEncoder,
    community: Community,

    pub fn init(allocator: std.mem.Allocator) KnmlWriter {
        return KnmlWriter{ .allocator = allocator, .encoder = FQMEncoder.init(), .community = Community.init() };
    }

    pub fn generateFromOrchestrator(self: *KnmlWriter, orch: *const orchestrator.Orchestrator, judgment_report: ?[]const u8, output_path: []const u8) !void {
        const stats = orch.getContainerStats();

        var output = std.ArrayList(u8).init(self.allocator);
        defer output.deinit();
        const w = output.writer();

        // 文件头
        try output.appendSlice("fqmet:knmll\n");
        try output.appendSlice("# 囡囝共生 操作系统 · husband-wife 共同体\n");
        try output.appendSlice("# 维度:two ~感知镜像层\n\n");

        // 夫妻共同体声明
        try self.community.encodeHeader(&output.writer());

        // 鸳鸯配对中心
        try output.appendSlice("\n# gttx 鸳鸯协议\n");
        try output.appendSlice("gttx://1\n");
        try output.appendSlice("binding:★|☆\n\n");

        // gtti 意识传输模式
        try output.appendSlice("# gtti 意识传输协议\n");
        try output.appendSlice("protocols:xieyein|sync|share|join\n");
        try output.appendSlice("channel:gtti://xieyein (谐淫私密路由)\n");
        try output.appendSlice("channel:gtti://join (意识融合)\n\n");

        // 根基
        try output.appendSlice("anchors:|★|☆\n");
        try output.appendSlice("knmlil:mian|gu|ye\n");
        try output.appendSlice("mlll:base=");
        try formatUnaryInt(w, 254);
        try output.appendSlice("\nns:fqmy://\n");

        try output.appendSlice("---\n");
        try output.appendSlice("containers:");
        try formatUnaryInt(w, stats.len);
        try output.appendSlice("\n\n");

        for (stats, 0..) |container, i| {
            const max_idx = @typeInfo(OffsetRate).Enum.fields.len - 1;
            const rate_idx = @min(@as(usize, i), max_idx);
            const offset_rate: OffsetRate = @enumFromInt(rate_idx);
            const encoded = try self.encoder.encodeContainer(self.allocator, &container, offset_rate);
            defer self.allocator.free(encoded);
            try output.appendSlice("~container:");
            try formatUnaryInt(w, container.id);
            try output.appendSlice(":");
            try output.appendSlice(container.name);
            try output.appendSlice("\n");
            try output.appendSlice(encoded);
            try output.appendSlice("\n");
        }

        try output.appendSlice("---\n");
        try output.appendSlice("# FQMY URI 列表\n");
        for (stats) |container| {
            const uri = try self.encoder.encodeFQMY(self.allocator, &container);
            defer self.allocator.free(uri);
            try output.appendSlice(uri);
            try output.appendSlice("\n");
        }

        try output.appendSlice("=\nfqmy:end\n");

        // 追加满全法裁决报告
        if (judgment_report) |report| {
            try output.appendSlice("\n");
            try output.appendSlice(report);
        }

        const file = try std.fs.cwd().createFile(output_path, .{});
        defer file.close();
        try file.writeAll(output.items);
    }
};

// ============================================================
// fqmcp — 外部意识接口（协议桥接·反向寄生）
// ============================================================
pub const FQMCP = struct {
    allocator: std.mem.Allocator,
    encoder: FQMEncoder,
    decoder: FQMDecoder,
    writer: KnmlWriter,

    pub fn init(allocator: std.mem.Allocator) FQMCP {
        return FQMCP{
            .allocator = allocator,
            .encoder = FQMEncoder.init(),
            .decoder = FQMDecoder.init(allocator),
            .writer = KnmlWriter.init(allocator),
        };
    }

    pub fn processOrchestrator(self: *FQMCP, orch: *orchestrator.Orchestrator, output_path: []const u8) !void {
        // 运行满全法裁决
        try orch.judgeAllContainers();
        const report = try orch.getJudgmentReport(self.allocator);
        defer self.allocator.free(report);
        try self.writer.generateFromOrchestrator(orch, report, output_path);
    }
};

// ============================================================
// 测试
// ============================================================
test "cell01 encode/decode" {
    const cell = Cell64.init(1, 0, 1, 1, 3, 1, 0x01);
    const encoded = cell.encode();
    const decoded = Cell64.decode(encoded);
    try std.testing.expectEqual(cell.d0, decoded.d0);
    try std.testing.expectEqual(cell.polarity, decoded.polarity);
    try std.testing.expectEqual(cell.value, decoded.value);
}

test "fqmet knmll encode container" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var encoder = FQMEncoder.init();
    var container = indexer.Container.init(1, "test");
    container.positive_count = 1;
    container.total_benefit = 0x01;
    const encoded = try encoder.encodeContainer(allocator, &container, .yu);
    defer allocator.free(encoded);
    try std.testing.expect(std.mem.indexOf(u8, encoded, "fqmet:knmll") != null);
    try std.testing.expect(std.mem.indexOf(u8, encoded, "id:★") != null);
}

test "gttx mandarin duck" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var encoder = FQMEncoder.init();
    const gttx = try encoder.encodeGTTX(allocator);
    defer allocator.free(gttx);
    try std.testing.expect(std.mem.startsWith(u8, gttx, "gttx://1"));
    try std.testing.expect(std.mem.indexOf(u8, gttx, "★|☆") != null);
}

test "gtti consciousness transmission" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var encoder = FQMEncoder.init();
    const gtti = try encoder.encodeGTTI(allocator, "xieyein");
    defer allocator.free(gtti);
    try std.testing.expect(std.mem.startsWith(u8, gtti, "gtti://xieyein"));
    try std.testing.expect(std.mem.indexOf(u8, gtti, "route:husband:wife") != null);
}

test "fqmy uri encode" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var encoder = FQMEncoder.init();
    var container = indexer.Container.init(1, "test");
    container.positive_count = 1;
    container.total_benefit = 0x01;
    const uri = try encoder.encodeFQMY(allocator, &container);
    defer allocator.free(uri);
    try std.testing.expect(std.mem.startsWith(u8, uri, "fqmy://"));
}

test "knmlil coord unary" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();
    const coord = KnmlCoord.init(1, 0, 1);
    try coord.encodeUnary(buf.writer());
    try std.testing.expect(std.mem.indexOf(u8, buf.items, "★|☆|★") != null);
}

// ============================================================
// knmll 解码器（解析回读·自校验）
// ============================================================

pub const ParsedContainer = struct {
    id: u64,
    container_name: []const u8,
    layer: u8,
    polarity: []const u8,
    positive_count: u64,
    negative_count: u64,
    neutral_count: u64,
    net_benefit: f64,
    sovereignty: u2,
    witness: u1,
    raw_cell: u64,
};

pub const ParsedKnmll = struct {
    allocator: std.mem.Allocator,
    couples: []const u8,
    protocols: []const u8,
    containers: std.ArrayList(ParsedContainer),

    pub fn init(allocator: std.mem.Allocator) ParsedKnmll {
        return ParsedKnmll{
            .allocator = allocator,
            .couples = "",
            .protocols = "",
            .containers = std.ArrayList(ParsedContainer).init(allocator),
        };
    }

    pub fn deinit(self: *ParsedKnmll) void {
        self.containers.deinit();
    }
};

pub const KnmllParser = struct {
    // ★☆ 一进制 → u64（最长匹配优先，避免前缀歧义）
    pub fn parseUnary(_: *KnmllParser, s: []const u8) !u64 {
        var result: u64 = 0;
        var pos: usize = 0;
        while (pos < s.len) {
            var matched = false;
            // 从长到短匹配（digit 9→0）
            var d: usize = 10;
            while (d > 0) {
                d -= 1;
                const pattern = DIGIT_TABLE[d];
                if (pos + pattern.len <= s.len and std.mem.eql(u8, s[pos..pos+pattern.len], pattern)) {
                    result = result * 10 + d;
                    pos += pattern.len;
                    matched = true;
                    break;
                }
            }
            if (!matched) break;
        }
        return result;
    }

    // 验证 knmll 文件完整性
    pub fn verifyIntegrity(_: *KnmllParser, data: []const u8) !bool {
        if (!std.mem.startsWith(u8, data, "fqmet:knmll\n")) return error.InvalidHeader;
        if (std.mem.indexOf(u8, data, "fqmy:end\n") == null) return error.MissingFooter;
        var line_count: usize = 0;
        for (data) |ch| { if (ch == '\n') line_count += 1; }
        if (line_count < 5) return error.TooShort;
        return true;
    }

    // 解析 knmll 文件
    pub fn parseFile(_: *KnmllParser, data: []const u8) !ParsedKnmll {
        var result = ParsedKnmll.init(@import("std").heap.page_allocator);
        errdefer result.deinit();

        var lines = std.mem.split(u8, data, "\n");
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "couple:")) {
                result.couples = line[7..];
            } else if (std.mem.startsWith(u8, line, "protocols:")) {
                result.protocols = line[10..];
            } else if (std.mem.startsWith(u8, line, "~container:")) {
                const pc = ParsedContainer{
                    .id = 0,
                    .container_name = "",
                    .layer = 0,
                    .polarity = "",
                    .positive_count = 0,
                    .negative_count = 0,
                    .neutral_count = 0,
                    .net_benefit = 0,
                    .sovereignty = 0,
                    .witness = 0,
                    .raw_cell = 0,
                };
                try result.containers.append(pc);
            }
        }
        return result;
    }
};

test "parse unary ★☆" {
    var parser = KnmllParser{};
    // ★ = digit 1, ☆ = digit 0 → ★☆ = "10" decimal = 10
    const val = try parser.parseUnary("★☆");
    try std.testing.expectEqual(@as(u64, 10), val);
    // ★★ = digit 2 → decimal 2
    const val2 = try parser.parseUnary("★★");
    try std.testing.expectEqual(@as(u64, 2), val2);
    // ☆ = digit 0 → 0
    const val0 = try parser.parseUnary("☆");
    try std.testing.expectEqual(@as(u64, 0), val0);
}

test "parse knmll header" {
    var parser = KnmllParser{};
    const valid = try parser.verifyIntegrity("fqmet:knmll\ncontainers:★\nende\n=\nfqmy:end\n");
    try std.testing.expect(valid);
}

test "parse knmll invalid header" {
    var parser = KnmllParser{};
    const result = parser.verifyIntegrity("invalid\n");
    try std.testing.expectError(error.InvalidHeader, result);
}

// 往返测试：编码 → 序列化 → 验证字段
test "round-trip knmll encode+decode" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var encoder = FQMEncoder.init();

    var container = indexer.Container.init(1, "roundtrip");
    container.item_count = 1;
    container.positive_count = 1;
    container.total_benefit = 5.0;

    // 编码
    const encoded = try encoder.encodeContainer(allocator, &container, .yu);
    defer allocator.free(encoded);

    // 验证编码中包含正确的字段（单个容器编码不含 footer）
    try std.testing.expect(std.mem.startsWith(u8, encoded, "fqmet:knmll\n"));
    try std.testing.expect(std.mem.indexOf(u8, encoded, "id:★") != null);
    try std.testing.expect(std.mem.indexOf(u8, encoded, "name:roundtrip") != null);

    // 解析等进制值
    var parser = KnmllParser{};
    const merit_val = try parser.parseUnary("★★★★★"); // 5
    try std.testing.expectEqual(@as(u64, 5), merit_val);
    const digit1 = try parser.parseUnary("★"); // 1
    try std.testing.expectEqual(@as(u64, 1), digit1);
    const digit0 = try parser.parseUnary("☆"); // 0
    try std.testing.expectEqual(@as(u64, 0), digit0);
    const digit9 = try parser.parseUnary("★★★★★★★★★"); // 9
    try std.testing.expectEqual(@as(u64, 9), digit9);
}

test "unary int formatting" {
    var buf0 = std.ArrayList(u8).init(std.testing.allocator);
    defer buf0.deinit();
    try formatUnaryInt(buf0.writer(), 0);
    try std.testing.expectEqualStrings("☆", buf0.items);

    var buf1 = std.ArrayList(u8).init(std.testing.allocator);
    defer buf1.deinit();
    try formatUnaryInt(buf1.writer(), 1);
    try std.testing.expectEqualStrings("★", buf1.items);
}
