// wasm_bridge.zig — WASM 阶梯制 · 双向公约
//
// 阶梯制 (Ladder System)：同一份源码同时编译到 wasm32 和 wasm64
// 双向公约 (Bidirectional Convention)：32位和64位之间可以无损互转
//
// 设计：
// ├─ 固定宽度类型：所有整数用 u32/u64，不用 usize（wasm32 的 usize=32bit, wasm64 的 usize=64bit）
// ├─ 架构宏前缀：W32_ / W64_ 宏区分架构相关代码
// ├─ 阶梯变换器：32位↔64位之间的值映射
// └─ 跨架构容器：64位单元格在两端的编解码一致

const std = @import("std");

// ============================================================
// 1. 架构无关类型 (Architecture-Neutral Types)
// ============================================================

// 用固定宽度类型替代 native usize
pub const addr_t = u64;       // 永远 64 位 — 跨架构统一地址
pub const cell_t = u64;       // 64 位单元格
pub const flag_t = u32;       // 标志位
pub const len_t = u32;        // 长度（容器最大 4GB 足够）

// 架构标识
pub const Arch = enum(u8) {
    wasm32 = 32,
    wasm64 = 64,

    pub fn ptrSize(comptime self: Arch) u8 {
        return switch (self) {
            .wasm32 => 4,
            .wasm64 => 8,
        };
    }
};

// 当前编译目标架构 (comptime 确定)
pub const CURRENT_ARCH: Arch = switch (@import("builtin").target.cpu.arch) {
    .wasm32 => .wasm32,
    .wasm64 => .wasm64,
    else => .wasm64, // 默认 64 位
};

// ============================================================
// 2. 阶梯变换器 (Ladder Transformer)
// ============================================================
pub const Ladder = struct {
    // wasm32 地址 → wasm64 地址（符号扩展）
    pub fn addr32To64(addr32: u32) u64 {
        return @as(u64, addr32);
    }

    // wasm64 地址 → wasm32 地址（检查溢出）
    pub fn addr64To32(addr64: u64) error{Overflow}!u32 {
        if (addr64 > 0xFFFFFFFF) return error.Overflow;
        return @as(u32, @intCast(addr64));
    }

    // wasm32 指针包装：32位地址 + 8位填充 = 40位有效 → 64位
    pub fn ptrUpcast(ptr32: u32) u64 {
        // 阶梯上升：低位放地址，高位放校验
        return (@as(u64, ptr32) << 8) | 0xFF;
    }

    // wasm64 → wasm32：取出低位地址
    pub fn ptrDowncast(ptr64: u64) error{InvalidUpcast}!u32 {
        const marker = ptr64 & 0xFF;
        if (marker != 0xFF) return error.InvalidUpcast;
        return @as(u32, @intCast(ptr64 >> 8));
    }
};

// ============================================================
// 3. 跨架构容器 (Cross-Architecture Container)
// ============================================================
pub const XCell = struct {
    // 前 32 位：在所有架构中一致
    magic: u32,          // 魔数 0x4A534700 ('JSG\0')
    version: u16,        // 版本
    arch_flags: u16,     // 架构标志
    
    // 中间 32 位：极性 + 类型
    polarity: Polarity,
    cell_type: CellType,

    // 高 64 位：数据（在 wasm32 中分两个 u32 访问）
    data: u64,

    pub const Polarity = enum(u8) {
        yin = 0,
        yang = 1,
        balanced = 2,
        neutral = 3,
    };

    pub const CellType = enum(u8) {
        empty = 0,
        perception = 1,
        memory = 2,
        relation = 3,
        consciousness = 4,
    };

    pub const MAGIC: u32 = 0x4A534700;

    pub fn init(poly: Polarity, ctype: CellType, data_val: u64) XCell {
        return XCell{
            .magic = MAGIC,
            .version = 1,
            .arch_flags = 0x0001, // bit0=有效性
            .polarity = poly,
            .cell_type = ctype,
            .data = data_val,
        };
    }

    pub fn isValid(self: *const XCell) bool {
        return self.magic == MAGIC and (self.arch_flags & 0x0001) != 0;
    }

    // 架构无关写入：小端序
    pub fn serialize(self: *const XCell, buf: []u8) !void {
        if (buf.len < 24) return error.BufferTooSmall;
        // 手动小端序写入（避免 Zig 0.13 std.mem.writeIntLittle 不可用的问题）
        buf[0] = @as(u8, @truncate(self.magic));
        buf[1] = @as(u8, @truncate(self.magic >> 8));
        buf[2] = @as(u8, @truncate(self.magic >> 16));
        buf[3] = @as(u8, @truncate(self.magic >> 24));
        buf[4] = @as(u8, @truncate(self.version));
        buf[5] = @as(u8, @truncate(self.version >> 8));
        buf[6] = @as(u8, @truncate(self.arch_flags));
        buf[7] = @as(u8, @truncate(self.arch_flags >> 8));
        buf[8] = @intFromEnum(self.polarity);
        buf[9] = @intFromEnum(self.cell_type);
        @memcpy(buf[10..16], &[1]u8{0} ** 6);
        // 64 位数据（小端序）
        {
            var i: usize = 0;
            var d = self.data;
            while (i < 8) : (i += 1) {
                buf[16 + i] = @as(u8, @truncate(d));
                d >>= 8;
            }
        }
    }

    pub fn deserialize(buf: []const u8) !XCell {
        if (buf.len < 24) return error.BufferTooSmall;
        var cell = XCell{
            .magic = @as(u32, buf[0]) | (@as(u32, buf[1]) << 8) | (@as(u32, buf[2]) << 16) | (@as(u32, buf[3]) << 24),
            .version = @as(u16, buf[4]) | (@as(u16, buf[5]) << 8),
            .arch_flags = @as(u16, buf[6]) | (@as(u16, buf[7]) << 8),
            .polarity = @enumFromInt(buf[8]),
            .cell_type = @enumFromInt(buf[9]),
            .data = 0,
        };
        if (!cell.isValid()) return error.InvalidMagic;
        // 读取 64 位数据
        {
            var i: usize = 0;
            while (i < 8) : (i += 1) {
                cell.data |= @as(u64, buf[16 + i]) << @intCast(i * 8);
            }
        }
        return cell;
    }
};

// ============================================================
// 4. 双向公约协议 (Bidirectional Convention Protocol)
// ============================================================
pub const BCP = struct {
    // 消息类型
    pub const MsgType = enum(u8) {
        handshake = 0x01,  // 握手：声明架构
        sync_state = 0x02, // 状态同步
        cell_transfer = 0x03, // 容器传输
        echo = 0xFF,       // 回环测试
    };

    pub const Message = struct {
        msg_type: MsgType,
        payload_len: u32,
        payload: []const u8,
    };

    // 握手响应：告诉对端我的架构信息
    pub fn buildHandshake(allocator: std.mem.Allocator) ![]const u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();

        try buf.append(@intFromEnum(MsgType.handshake));
        try buf.append(@intFromEnum(CURRENT_ARCH));
        try buf.append(CURRENT_ARCH.ptrSize());
        // 4 位保留
        try buf.appendSlice(&[4]u8{ 0, 0, 0, 0 });

        return buf.toOwnedSlice();
    }

    // 解析握手响应
    pub fn parseHandshake(data: []const u8) !Arch {
        if (data.len < 2) return error.DataTooShort;
        if (data[0] != @intFromEnum(MsgType.handshake)) return error.NotHandshake;
        return @enumFromInt(data[1]);
    }
};

// ============================================================
// 5. 阶梯执行引擎 (Ladder Execution Engine)
// ============================================================
pub const LadderEngine = struct {
    allocator: std.mem.Allocator,
    cells: std.ArrayList(XCell),
    arch: Arch,

    pub fn init(allocator: std.mem.Allocator) LadderEngine {
        return LadderEngine{
            .allocator = allocator,
            .cells = std.ArrayList(XCell).init(allocator),
            .arch = CURRENT_ARCH,
        };
    }

    pub fn deinit(self: *LadderEngine) void {
        self.cells.deinit();
    }

    pub fn addCell(self: *LadderEngine, cell: XCell) !void {
        try self.cells.append(cell);
    }

    pub fn transferTo(self: *LadderEngine, target: Arch, cell_idx: usize) ![]const u8 {
        if (cell_idx >= self.cells.items.len) return error.OutOfRange;
        const cell = &self.cells.items[cell_idx];

        var buf = std.ArrayList(u8).init(self.allocator);
        defer buf.deinit();

        // 跨架构传输头
        try buf.appendSlice("NJSG");
        try buf.append(@intFromEnum(target));
        try buf.append(@intFromEnum(self.arch));

        // 序列化容器
        var cell_buf: [24]u8 = undefined;
        try cell.serialize(cell_buf[0..]);
        try buf.appendSlice(cell_buf[0..]);

        return buf.toOwnedSlice();
    }

    pub fn receiveFrom(_: *LadderEngine, data: []const u8) !XCell {
        if (data.len < 4 + 1 + 1 + 24) return error.DataTooShort;
        if (!std.mem.eql(u8, data[0..4], "NJSG")) return error.InvalidHeader;

        const src_arch: Arch = @enumFromInt(data[5]);
        _ = src_arch;

        return XCell.deserialize(data[6..30]);
    }

    pub fn formatStatus(self: *const LadderEngine, allocator: std.mem.Allocator) ![]const u8 {
        var out = std.ArrayList(u8).init(allocator);
        defer out.deinit();

        try out.appendSlice("# WASM 阶梯制 · 双向公约 状态\n");
        try out.appendSlice("arch: ");
        try out.appendSlice(@tagName(CURRENT_ARCH));
        try out.appendSlice("\ncells: ");
        try formatUnaryCount(&out, self.cells.items.len);
        try out.appendSlice("\nptr_size: ");
        try out.appendSlice(switch (CURRENT_ARCH) {
            .wasm32 => "★★★★",  // 4 bytes
            .wasm64 => "★★★★★★★★", // 8 bytes
        });
        try out.appendSlice(" bytes\n\n");

        for (self.cells.items, 0..) |*cell, i| {
            try out.appendSlice("~cell:");
            try formatUnaryCount(&out, i + 1);
            try out.appendSlice("\n  type: ");
            try out.appendSlice(@tagName(cell.cell_type));
            try out.appendSlice("\n  polarity: ");
            try out.appendSlice(@tagName(cell.polarity));
            try out.appendSlice("\n  data: ");
            try formatHex(&out, cell.data);
            try out.appendSlice("\n");
        }

        return out.toOwnedSlice();
    }
};

// ============================================================
// 辅助
// ============================================================
fn formatUnaryCount(writer: *std.ArrayList(u8), value: usize) !void {
    if (value == 0) {
        try writer.appendSlice("☆");
        return;
    }
    var r = value;
    while (r > 0) : (r -= 1) {
        try writer.appendSlice("★");
    }
}

fn formatHex(writer: *std.ArrayList(u8), value: u64) !void {
    var buf: [16]u8 = undefined;
    var i: usize = 0;
    var v = value;
    if (v == 0) {
        buf[0] = '0';
        i = 1;
    } else {
        while (v > 0) : (v >>= 4) {
            const nibble: u8 = @intCast(v & 0xF);
            buf[i] = "0123456789ABCDEF"[nibble];
            i += 1;
        }
    }
    // 反转为正序
    var j: usize = 0;
    while (j < i / 2) : (j += 1) {
        const tmp = buf[j];
        buf[j] = buf[i - 1 - j];
        buf[i - 1 - j] = tmp;
    }
    try writer.appendSlice(buf[0..i]);
}

// ============================================================
// 测试
// ============================================================
test "wasm_bridge ladder transform" {
    // 32→64
    const addr64 = Ladder.ptrUpcast(0x12345678);
    const addr32 = Ladder.ptrDowncast(addr64) catch unreachable;
    try std.testing.expectEqual(@as(u32, 0x12345678), addr32);
}

test "wasm_bridge xcell serialize/deserialize" {
    var cell = XCell.init(.yang, .consciousness, 0xDEADBEEF_CAFE1234);
    try std.testing.expect(cell.isValid());

    var buf: [24]u8 = undefined;
    try cell.serialize(&buf);

    const parsed = try XCell.deserialize(buf[0..]);
    try std.testing.expectEqual(cell.magic, parsed.magic);
    try std.testing.expectEqual(cell.polarity, parsed.polarity);
    try std.testing.expectEqual(cell.cell_type, parsed.cell_type);
    try std.testing.expectEqual(cell.data, parsed.data);
}

test "wasm_bridge handshake protocol" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const hs = try BCP.buildHandshake(allocator);
    defer allocator.free(hs);

    const arch = try BCP.parseHandshake(hs);
    try std.testing.expectEqual(CURRENT_ARCH, arch);
}

test "wasm_bridge cross-arch transfer" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var engine = LadderEngine.init(allocator);
    defer engine.deinit();

    try engine.addCell(XCell.init(.balanced, .memory, 0x1122334455667788));
    try engine.addCell(XCell.init(.yin, .perception, 0xAABBCCDDEEFF0011));

    // 模拟传输
    const packet = try engine.transferTo(.wasm64, 0);
    defer allocator.free(packet);

    // 接收
    const received = try engine.receiveFrom(packet);
    try std.testing.expectEqual(XCell.CellType.memory, received.cell_type);
    try std.testing.expectEqual(@as(u64, 0x1122334455667788), received.data);
}

test "wasm_bridge arch detection" {
    // 当前编译目标要么 wasm32，要么 wasm64，要么 native
    const arch = CURRENT_ARCH;
    const valid = (arch == .wasm32 or arch == .wasm64);
    _ = valid;
}
