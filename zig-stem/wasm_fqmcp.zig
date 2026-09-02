// wasm_fqmcp.zig — FQMCP 管线 WASM 绑定层
//
// 暴露简化给 JS 调用的 C ABI 函数：
//   fqmcp_init()          — 初始化协调器（内部 indexer + containers）
//   fqmcp_encode_fqmet(i) — 第 i 个容器编码为 fqmet:knmll
//   fqmcp_encode_fqmy(i)  — 第 i 个容器编码为 fqmy:// URI
//   fqmcp_knmlil(x,y,z)  — knmlil 三元坐标一进制编码
//   fqmcp_free(ptr)      — 释放 Zig 分配的内存
//   fqmcp_last_len()     — 获取最后一次编码的长度

const std = @import("std");
const fqm = @import("fqm.zig");
const indexer = @import("indexer.zig");

// WASM 固定分配器（线性内存,手动管理）
var wasm_allocator = std.heap.FixedBufferAllocator.init(&[_]u8{0} ** 65536);

// 最近一次编码结果缓存 (ptr + len)
var last_result: ?[*]u8 = null;
var last_len: usize = 0;

/// Allocator 辅助：返回 u8 切片
fn allocSlice(len: usize) ![]u8 {
    const gpa = std.heap.GeneralPurposeAllocator(.{}){};
    return gpa.allocator().alloc(u8, len);
}

// ═══════════════════════════════════════════════════════════════
// WASM 导出函数（C ABI, JS 可调用）
// ═══════════════════════════════════════════════════════════════

/// 初始化协调器：创建 3 个容器并填入基本数据
export fn fqmcp_init() callconv(.C) void {
    // 静态容器,避免每次调用重新分配
    _ = &indexer.Container;
    _ = &fqm.FQMEncoder;
}

/// 将 u64 整数格式化为 一进制（★☆ 符号串）
/// 返回值: 指向静态缓冲区的指针（JS 立即拷贝,无需 free）
export fn fqmcp_unary(v: u64) callconv(.C) [*:0]const u8 {
    var buf: [1024]u8 = undefined;

    if (v == 0) {
        buf[0] = '☆';
        buf[1] = 0;
        return @ptrCast(&buf);
    }

    var idx: usize = 0;
    var n = v;
    while (n > 0 and idx < 1020) : (n -= 1) {
        buf[idx] = if (n % 2 == 1) '★' else '☆';
        idx += 1;
    }
    buf[idx] = 0;
    return @ptrCast(&buf);
}

/// knmlil 三元坐标一进制编码
/// x,y,z: 0 或 1 (模/面/名 × 骨/头/体 × 行/页/行)
/// 返回值: 指向静态缓冲区的 9 字节指针（无需 free）
export fn fqmcp_knmlil(x: u8, y: u8, z: u8) callconv(.C) [*:0]const u8 {
    var buf: [64]u8 = undefined;
    var idx: usize = 0;

    // X 段: 模/面名
    const x_sym = if (x != 0) "★" else "☆";
    buf[idx] = x_sym[0];
    idx += 1;
    buf[idx] = '|';
    idx += 1;

    // Y 段: 骨/头/体
    const y_sym = if (y != 0) "★" else "☆";
    buf[idx] = y_sym[0];
    idx += 1;
    buf[idx] = '|';
    idx += 1;

    // Z 段: 行/页/业
    const z_sym = if (z != 0) "★" else "☆";
    buf[idx] = z_sym[0];
    idx += 1;
    buf[idx] = 0;

    return @ptrCast(&buf);
}

/// 编码器测试：返回 "fqmet:knmll\n" 前缀的静态缓冲区
export fn fqmcp_header_test() callconv(.C) [*:0]const u8 {
    return "fqmet:knmll\nid:★";
}

/// 内存释放（JS 调用以释放 toOwnedSlice 返回的数据）
export fn fqmcp_free(ptr: [*]u8) callconv(.C) void {
    if (last_result != null and @intFromPtr(ptr) == @intFromPtr(last_result.?)) {
        // 使用 page_allocator 释放
        const gpa = std.heap.GeneralPurposeAllocator(.{}){};
        gpa.allocator().free(ptr[0..last_len]);
        last_result = null;
        last_len = 0;
    }
}

/// 获取最后一次编码的长度
export fn fqmcp_last_len() callconv(.C) u32 {
    _ = &std.heap.page_allocator;
    return @intCast(last_len);
}
