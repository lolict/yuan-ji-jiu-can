// preload_dict.zig - Preload Dictionary (resident static / immutable identity)
//
// Core philosophy:
//   The dictionary is the symbol -> color -> position triplet binding.
//   Once loaded, it CANNOT be changed - this is the resident static constraint.
//   The dictionary is the source of truth for all translations.
//   If a symbol changes its position, the whole system's static invariance breaks.
//
//   Zig's comptime makes this dictionary truly immutable:
//     - Computed at compile time
//     - Embedded in binary
//     - Zero runtime cost
//     - Physically impossible to mutate

const std = @import("std");

pub const COLOR_COUNT: u8 = 42;
pub const SYMBOL_COUNT: u8 = 64;
pub const LAYER_COUNT: u8 = 6;
pub const COL_COUNT: u8 = 7;

// === Traditional color names (42 colors, 6 systems) ===
pub const ColorName = struct {
    hanzi: []const u8,    // Chinese character (UTF-8, max 4 bytes)
    pinyin: []const u8,  // Pinyin romanization
    system: u8,             // system 0-5
    layer: u8,              // layer 0-5
    col: u8,                // col 0-6
    quality: []const u8,  // quality (warm/light/cold/etc)
};

// === Symbol binding: 64 symbols -> 42 colors ===
pub const SymbolBinding = struct {
    symbol: u8,
    color_idx: u8,
};

// === Preloaded dictionary: computed at COMPILE TIME ===
// This is the resident static: cannot be modified at runtime
pub const dictionary = blk: {
    var binds: [SYMBOL_COUNT]SymbolBinding = undefined;
    for (0..SYMBOL_COUNT) |i| {
        binds[i] = .{ .symbol = @as(u8, @intCast(i)), .color_idx = @as(u8, @intCast(i % COLOR_COUNT)) };
    }
    break :blk binds;
};

// === Symbol name table (64 symbols from base64_org) ===
pub const SymbolInfo = struct {
    hanzi: []const u8,
    pinyin: []const u8,
    role: []const u8,
};

pub const symbol_names = blk: {
    var names: [SYMBOL_COUNT]SymbolInfo = undefined;
    names[0] = .{ .hanzi = "啊", .pinyin = "a", .role = "妻" };
    names[1] = .{ .hanzi = "哦", .pinyin = "o", .role = "夫" };
    names[2] = .{ .hanzi = "工", .pinyin = "gong", .role = "工藤新一" };
    names[3] = .{ .hanzi = "空", .pinyin = "kong", .role = "柯南" };
    names[4] = .{ .hanzi = "灰", .pinyin = "hui", .role = "灰原哀" };
    names[5] = .{ .hanzi = "开", .pinyin = "kai", .role = "毛利兰" };
    for (6..SYMBOL_COUNT) |i| {
        names[i] = .{ .hanzi = "符", .pinyin = "fu", .role = "扩展层" };
    }
    break :blk names;
};

/// Lookup binding for a symbol (immutable)
pub fn lookup(symbol: u8) ?SymbolBinding {
    if (symbol >= SYMBOL_COUNT) return null;
    return dictionary[symbol];
}

/// Get color index for a symbol
pub fn symbolToColor(symbol: u8) ?u8 {
    if (symbol >= SYMBOL_COUNT) return null;
    return dictionary[symbol].color_idx;
}

/// Verify dictionary integrity (always true for comptime data)
pub fn verify() bool {
    for (dictionary, 0..) |b, i| {
        if (b.symbol != i) return false;
        if (b.color_idx >= COLOR_COUNT) return false;
    }
    return true;
}

/// Get cardinality (total color count - constant)
pub fn cardinality() u32 {
    return COLOR_COUNT;
}

/// Report
pub fn report() void {
    const stdout = std.io.getStdOut().writer();
    stdout.print("  [Preload Dictionary] ", .{}) catch {};
    stdout.print("symbols={} colors={} cardinality={}\n", .{ SYMBOL_COUNT, COLOR_COUNT, cardinality() }) catch {};
    stdout.print("    verify: {}\n", .{verify()}) catch {};
    stdout.print("    couple layer: {s}({s}) -> 妃(R0)\n", .{ symbol_names[0].hanzi, symbol_names[0].pinyin }) catch {};
    stdout.print("    couple layer: {s}({s}) -> 粉(R1)\n", .{ symbol_names[1].hanzi, symbol_names[1].pinyin }) catch {};
    stdout.print("    center: {s}({s}) -> 彤(R2)\n", .{ symbol_names[2].hanzi, symbol_names[2].pinyin }) catch {};
    stdout.print("    resident static: IMMUTABLE (comptime)\n", .{}) catch {};
}

test "dictionary_bounds" {
    try std.testing.expectEqual(@as(u8, 42), COLOR_COUNT);
    try std.testing.expectEqual(@as(u8, 64), SYMBOL_COUNT);
}

test "dictionary_verify" {
    try std.testing.expect(verify());
}

test "symbol_to_color" {
    const c0 = symbolToColor(0);
    try std.testing.expect(c0 != null);
    try std.testing.expect(c0.? < COLOR_COUNT);
    const c63 = symbolToColor(63);
    try std.testing.expect(c63 != null);
    try std.testing.expect(c63.? < COLOR_COUNT);
}

test "cardinality_constant" {
    try std.testing.expectEqual(@as(u32, 42), cardinality());
}
