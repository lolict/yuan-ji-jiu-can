// MetaIME Zig FFI Bridge · 导出 Zig 实现为 C ABI
//
// 编译: zig build-lib metaime_zig_bridge.zig -dynamic -lc
// 生成: libmetaime_zig_bridge.so

const std = @import("std");
const c = @cImport({
    @cInclude("metaime_ffi.h");
});

// 引入 Zig 实现模块（使用绝对路径）
const loader = @import("../zig/src/loader.zig");
const linker = @import("../zig/src/linker.zig");
const translator = @import("../zig/src/translator.zig");
const radix3d = @import("../zig/src/radix3d.zig");
const divisor = @import("../zig/src/divisor.zig");
const yuan = @import("../zig/src/yuan.zig");

export fn mi_loader_create() ?*c.mi_loader {
    // 使用 comptime 数据，无需运行时分配
    return null; // Zig 用编译时数据，不需要实例
}

export fn mi_loader_destroy(l: ?*c.mi_loader) void {
    _ = l;
}

export fn mi_loader_load_default(l: ?*c.mi_loader) c.mi_err_t {
    _ = l;
    return c.MI_OK; // Zig 数据已在编译时嵌入
}

export fn mi_loader_container_count(l: ?*const c.mi_loader) c.int {
    _ = l;
    return @intCast(loader.fqm_data.container_count);
}

export fn mi_loader_get_container(l: ?*const c.mi_loader, name: [*c]const u8, out: *c.mi_container_t) c.mi_err_t {
    _ = l;
    const name_str = std.mem.span(name);
    const container = loader.getContainer(name_str) orelse return c.MI_ERR_NOT_FOUND;

    @memcpy(out.name[0..container.name.len], container.name);
    out.name[container.name.len] = 0;
    out.symbol_count = container.symbol_count;
    out.capacity = container.capacity;

    for (container.symbols[0..container.symbol_count], 0..) |sym, i| {
        @memcpy(out.symbols[i].symbol[0..sym.symbol.len], sym.symbol);
        out.symbols[i].symbol[sym.symbol.len] = 0;
        out.symbols[i].position = sym.position;
    }

    return c.MI_OK;
}

export fn mi_linker_create() ?*c.mi_linker {
    return null;
}

export fn mi_linker_destroy(l: ?*c.mi_linker) void {
    _ = l;
}

export fn mi_linker_build_index(l: ?*c.mi_linker, loader_ptr: ?*const c.mi_loader) c.mi_err_t {
    _ = l;
    _ = loader_ptr;
    return c.MI_OK;
}

export fn mi_linker_lookup(l: ?*const c.mi_linker, domain: [*c]const u8, symbol: [*c]const u8, out: *c.mi_path_t) c.int {
    _ = l;
    const d = std.mem.span(domain);
    const s = std.mem.span(symbol);
    const entry = linker.lookup(d, s) orelse return c.MI_ERR_NOT_FOUND;

    @memcpy(out.domain[0..d.len], d);
    out.domain[d.len] = 0;
    @memcpy(out.identity[0..entry.identity.len], entry.identity);
    out.identity[entry.identity.len] = 0;
    return c.MI_OK;
}

export fn mi_translator_create() ?*c.mi_translator {
    return null;
}

export fn mi_translator_destroy(t: ?*c.mi_translator) void {
    _ = t;
}

export fn mi_translator_translate(t: ?*const c.mi_translator, foreign: [*c]const u8, out: [*c]u8, out_len: usize) c.mi_err_t {
    _ = t;
    const f = std.mem.span(foreign);
    const result = translator.translate(f) orelse return c.MI_ERR_NOT_FOUND;
    const len = @min(result.len, out_len - 1);
    @memcpy(out[0..len], result);
    out[len] = 0;
    return c.MI_OK;
}

export fn mi_radix3d_coordinate(sm: [*c]const u8, ym: [*c]const u8, t: [*c]const u8, out: *c.mi_point3d_t) c.mi_err_t {
    const sm_str = std.mem.span(sm);
    const ym_str = std.mem.span(ym);
    const t_str = std.mem.span(t);

    _ = linker.identityToIndex(sm_str) orelse return c.MI_ERR_NOT_FOUND;
    _ = linker.identityToIndex(ym_str) orelse return c.MI_ERR_NOT_FOUND;
    _ = linker.identityToIndex(t_str) orelse return c.MI_ERR_NOT_FOUND;

    @memcpy(out.shengmu[0..sm_str.len], sm_str);
    out.shengmu[sm_str.len] = 0;
    @memcpy(out.yunmu[0..ym_str.len], ym_str);
    out.yunmu[ym_str.len] = 0;
    @memcpy(out.tone[0..t_str.len], t_str);
    out.tone[t_str.len] = 0;
    return c.MI_OK;
}

export fn mi_radix3d_distance(id1: [*c]const u8, id2: [*c]const u8) u8 {
    const a = std.mem.span(id1);
    const b = std.mem.span(id2);
    return radix3d.ontologyDistance(a, b);
}

export fn mi_gcd(a: u8, b: u8) u8 {
    return divisor.gcd(a, b);
}

export fn mi_lcm(a: u8, b: u8) u16 {
    return divisor.lcm(a, b);
}

export fn mi_yuan_judge(a: [*c]const u8, b: [*c]const u8, out: [*c]u8, out_len: usize) c.mi_err_t {
    const a_str = std.mem.span(a);
    const b_str = std.mem.span(b);
    const result = yuan.judge(a_str, b_str);
    const len = @min(result.message.len, out_len - 1);
    @memcpy(out[0..len], result.message);
    out[len] = 0;
    return c.MI_OK;
}
