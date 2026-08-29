// MetaIME Zig FFI Bridge · 运行时包装器
//
// 由于原始 Zig 实现使用 comptime，FFI 桥需要运行时包装器
// 这些包装器访问编译时数据，但通过 C ABI 暴露运行时接口

const std = @import("std");

// 引入编译时数据（只读访问）
const loader = @import("src/loader.zig");
const constants = @import("src/constants.zig");

// === 错误码 ===
const MI_OK: c_int = 0;
const MI_ERR_NOT_FOUND: c_int = -1;
const MI_ERR_INVALID: c_int = -3;

// === 加载器 API ===

pub export fn mi_loader_create() ?*anyopaque {
    // Zig 使用编译时数据，返回一个非空哑指针即可
    // 实际数据在编译时已确定，不需要运行时分配
    return @ptrFromInt(0x5A167);  // 非空标记值 "ZIG" (Z=0x5A, I=0x16, G=0x7)
}

pub export fn mi_loader_destroy(l: ?*anyopaque) void {
    _ = l;
}

pub export fn mi_loader_load_default(l: ?*anyopaque) c_int {
    _ = l;
    return MI_OK;
}

pub export fn mi_loader_container_count(l: ?*const anyopaque) c_int {
    _ = l;
    return @intCast(loader.fqm_data.container_count);
}

pub export fn mi_translator_create() ?*anyopaque {
    // Zig 使用编译时翻译表，返回非空哑指针
    return @ptrFromInt(0x5A167);  // 非空标记值
}

pub export fn mi_translator_destroy(t: ?*anyopaque) void {
    _ = t;
}

// 翻译查找（运行时包装）
pub export fn mi_translator_translate(t: ?*const anyopaque, foreign: [*c]const u8, out: [*c]u8, out_len: usize) c_int {
    _ = t;
    const f = std.mem.span(foreign);

    // 线性搜索 FOREIGN_MAP（编译时数据）
    var i: usize = 0;
    while (i < constants.FOREIGN_MAP_KEYS.len) : (i += 1) {
        if (std.mem.eql(u8, constants.FOREIGN_MAP_KEYS[i], f)) {
            const result = constants.FOREIGN_MAP_VALS[i];
            const len = @min(result.len, out_len - 1);
            @memcpy(out[0..len], result);
            out[len] = 0;
            return MI_OK;
        }
    }
    return MI_ERR_NOT_FOUND;
}

// === 本体论 API ===

pub export fn mi_linker_create() ?*anyopaque {
    return null;
}

pub export fn mi_linker_destroy(l: ?*anyopaque) void {
    _ = l;
}

pub export fn mi_linker_build_index(l: ?*anyopaque, loader_ptr: ?*const anyopaque) c_int {
    _ = l;
    _ = loader_ptr;
    return MI_OK;
}

// 符号→身份 查找（运行时包装）
pub export fn mi_linker_lookup(l: ?*const anyopaque, domain: [*c]const u8, symbol: [*c]const u8, out: *anyopaque) c_int {
    _ = l;
    const d = std.mem.span(domain);
    const s = std.mem.span(symbol);

    // 在 FQM 数据中搜索
    var ci: u8 = 0;
    while (ci < loader.fqm_data.container_count) : (ci += 1) {
        const c = loader.fqm_data.containers[ci];
        if (std.mem.eql(u8, c.name, d)) {
            var si: u8 = 0;
            while (si < c.symbol_count) : (si += 1) {
                if (std.mem.eql(u8, c.symbols[si].symbol, s)) {
                    // 找到了，写入身份名
                    const out_ptr = @as([*]u8, @ptrCast(out));
                    @memcpy(out_ptr[0..d.len], d);
                    out_ptr[d.len] = 0;

                    // 身份名 = 身份序名[position]
                    const identity = if (c.symbols[si].position < 24)
                        constants.IDENTITY_NAMES[c.symbols[si].position]
                    else
                        "未知";

                    @memcpy(out_ptr[24..24 + identity.len], identity);
                    out_ptr[24 + identity.len] = 0;
                    return MI_OK;
                }
            }
        }
    }
    return MI_ERR_NOT_FOUND;
}

// === 三维进制 API ===

/// 运行时版本：身份名 → 索引（线性搜索）
fn runtimeIdentityToIndex(identity: []const u8) ?u8 {
    var i: u8 = 0;
    while (i < constants.IDENTITY_COUNT) : (i += 1) {
        if (std.mem.eql(u8, constants.IDENTITY_NAMES[i], identity)) return i;
    }
    return null;
}

pub export fn mi_radix3d_coordinate(sm: [*c]const u8, ym: [*c]const u8, t: [*c]const u8, out: *anyopaque) c_int {
    const sm_str = std.mem.span(sm);
    const ym_str = std.mem.span(ym);
    const t_str = std.mem.span(t);

    // 验证身份名有效
    _ = runtimeIdentityToIndex(sm_str) orelse return MI_ERR_NOT_FOUND;
    _ = runtimeIdentityToIndex(ym_str) orelse return MI_ERR_NOT_FOUND;
    _ = runtimeIdentityToIndex(t_str) orelse return MI_ERR_NOT_FOUND;

    const out_ptr = @as([*]u8, @ptrCast(out));
    @memcpy(out_ptr[0..sm_str.len], sm_str);
    out_ptr[sm_str.len] = 0;
    @memcpy(out_ptr[24..24 + ym_str.len], ym_str);
    out_ptr[24 + ym_str.len] = 0;
    @memcpy(out_ptr[48..48 + t_str.len], t_str);
    out_ptr[48 + t_str.len] = 0;
    return MI_OK;
}

// 本体论距离（运行时包装）
pub export fn mi_radix3d_distance(id1: [*c]const u8, id2: [*c]const u8) u8 {
    const a = std.mem.span(id1);
    const b = std.mem.span(id2);

    const idx1 = runtimeIdentityToIndex(a) orelse return 0;
    const idx2 = runtimeIdentityToIndex(b) orelse return 0;

    // 环形距离
    const diff = if (idx1 > idx2) idx1 - idx2 else idx2 - idx1;
    const max_idx: u8 = 23;
    return @min(diff, max_idx - diff);
}

// === 公约数 API ===

fn gcd(a: u8, b: u8) u8 {
    var x = a;
    var y = b;
    while (y != 0) {
        const t = y;
        y = x % y;
        x = t;
    }
    return x;
}

fn lcm(a: u8, b: u8) u16 {
    if (a == 0 or b == 0) return 0;
    return @intCast(a * b / gcd(a, b));
}

pub export fn mi_gcd(a: u8, b: u8) u8 {
    return gcd(a, b);
}

pub export fn mi_lcm(a: u8, b: u8) u16 {
    return lcm(a, b);
}

// === 元层级 API ===

pub export fn mi_yuan_judge(a: [*c]const u8, b: [*c]const u8, out: [*c]u8, out_len: usize) c_int {
    const a_str = std.mem.span(a);
    const b_str = std.mem.span(b);

    // 简单的本体论裁判
    const result = if (std.mem.eql(u8, a_str, b_str))
        "同域同位"
    else
        "同域异位";

    const len = @min(result.len, out_len - 1);
    @memcpy(out[0..len], result);
    out[len] = 0;
    return MI_OK;
}
