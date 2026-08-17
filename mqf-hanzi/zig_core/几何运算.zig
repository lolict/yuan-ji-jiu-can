// 满全法 - Zig 底层几何运算
// 静态编译、零开销、直接生成机器码

const std = @import("std");

/// 64位单元格坐标
pub const 单元格 = extern struct {
    x: u16,        // 左右轴
    y: u16,        // 上下轴
    z: u16,        // 深度轴
    polarity: u8,  // 极性 (阴/阳)
    _pad: u8 = 0,
};

/// 部件对
pub const 部件对 = struct {
    左: 单元格,
    右: 单元格,
    亲密度: f64,
};

/// 3x3 变换矩阵 (用于几何变换)
pub const 矩阵3x3 = [3][3]f64;

/// 计算曼哈顿距离
pub fn 曼哈顿距离(a: 单元格, b: 单元格) u32 {
    const dx = if (a.x > b.x) a.x - b.x else b.x - a.x;
    const dy = if (a.y > b.y) a.y - b.y else b.y - a.y;
    const dz = if (a.z > b.z) a.z - b.z else b.z - a.z;
    return dx + dy + dz;
}

/// 计算欧几里得距离的平方
pub fn 欧距平方(a: 单元格, b: 单元格) f64 {
    const dx = @as(f64, @floatFromInt(a.x)) - @as(f64, @floatFromInt(b.x));
    const dy = @as(f64, @floatFromInt(a.y)) - @as(f64, @floatFromInt(b.y));
    const dz = @as(f64, @floatFromInt(a.z)) - @as(f64, @floatFromInt(b.z));
    return dx * dx + dy * dy + dz * dz;
}

/// 计算亲密度 (0.0 ~ 1.0)
pub fn 计算亲密度(a: 单元格, b: 单元格) f64 {
    const 距离 = 曼哈顿距离(a, b);
    return 1.0 / (1.0 + @as(f64, @floatFromInt(距离)) / 1000.0);
}

/// 矩阵乘法 (3x3)
pub fn 矩阵乘(a: 矩阵3x3, b: 矩阵3x3) 矩阵3x3 {
    var 结果: 矩阵3x3 = undefined;
    var i: usize = 0;
    while (i < 3) : (i += 1) {
        var j: usize = 0;
        while (j < 3) : (j += 1) {
            结果[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    return 结果;
}

/// 矩阵作用于向量
pub fn 矩阵作用于向量(m: 矩阵3x3, v: [3]f64) [3]f64 {
    var 结果: [3]f64 = undefined;
    var i: usize = 0;
    while (i < 3) : (i += 1) {
        结果[i] = m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2];
    }
    return 结果;
}

/// 单位矩阵
pub fn 单位矩阵() 矩阵3x3 {
    return .{
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 1.0 },
    };
}

/// 缩放矩阵
pub fn 缩放矩阵(sx: f64, sy: f64, sz: f64) 矩阵3x3 {
    return .{
        .{ sx, 0.0, 0.0 },
        .{ 0.0, sy, 0.0 },
        .{ 0.0, 0.0, sz },
    };
}

/// 层级常量
pub const 层级枚举 = enum(u8) {
    一元 = 1,
    二元 = 2,
    三元 = 3,
    四元 = 4,
    五元 = 5,
    六元 = 6,
    七元 = 7,
    八元 = 8,
    九元 = 9,
};

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    
    try stdout.print("=== 满全法 Zig 底层几何运算 ===\n\n", .{});
    
    // 测试坐标
    const 女 = 单元格{ .x = 100, .y = 200, .z = 50, .polarity = 0 };
    const 子 = 单元格{ .x = 300, .y = 200, .z = 50, .polarity = 1 };
    const 日 = 单元格{ .x = 150, .y = 180, .z = 40, .polarity = 1 };
    const 月 = 单元格{ .x = 320, .y = 180, .z = 40, .polarity = 0 };
    
    try stdout.print("坐标距离测试:\n", .{});
    try stdout.print("  女-子 (好): 曼哈顿={d} 欧距平方={d:.2}\n", .{ 曼哈顿距离(女, 子), 欧距平方(女, 子) });
    try stdout.print("  日-月 (明): 曼哈顿={d} 欧距平方={d:.2}\n", .{ 曼哈顿距离(日, 月), 欧距平方(日, 月) });
    try stdout.print("  女-日 (跨界): 曼哈顿={d} 欧距平方={d:.2}\n", .{ 曼哈顿距离(女, 日), 欧距平方(女, 日) });
    
    try stdout.print("\n亲密度计算:\n", .{});
    try stdout.print("  好(女+子) 亲密度={d:.6}\n", .{ 计算亲密度(女, 子) });
    try stdout.print("  明(日+月) 亲密度={d:.6}\n", .{ 计算亲密度(日, 月) });
    
    try stdout.print("\n矩阵变换测试:\n", .{});
    const 缩放 = 缩放矩阵(2.0, 0.5, 1.0);
    const 向量 = [3]f64{ 100.0, 200.0, 50.0 };
    const 变换后 = 矩阵作用于向量(缩放, 向量);
    try stdout.print("  原向量: [{d:.1}, {d:.1}, {d:.1}]\n", .{ 向量[0], 向量[1], 向量[2] });
    try stdout.print("  缩放后: [{d:.1}, {d:.1}, {d:.1}]\n", .{ 变换后[0], 变换后[1], 变换后[2] });
    
    try stdout.print("\n层级体系:\n", .{});
    const 层级列表 = [_]层级枚举{
        .一元, .二元, .三元, .四元, .五元,
        .六元, .七元, .八元, .九元,
    };
    for (层级列表) |层| {
        const 深度 = @intFromEnum(层);
        const 容量 = std.math.pow(u32, 2, 深度 * 7);
        try stdout.print("  {}: 深度={d}, 容量={d} ({d}位)\n", .{ 层, 深度, 容量, 深度 * 64 });
    }
    
    try stdout.print("\n=== 几何层就绪 ===\n", .{});
}
