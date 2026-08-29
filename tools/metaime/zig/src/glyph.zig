// glyph.zig - 字形几何动力学
//
// 每个字母的字形就是一个力学系统：
//   直线 = 张力（直接传递力）
//   曲线 = 扭力（旋转传递力）
//   交叉 = 分支点（力的一分为二）
//   弯折 = 方向改变（力的反射）
//   封闭 = 容器（内部空间）
//   开口 = 通道（内外交换）
//
// 字母的操作语义由它的几何力学决定

const std = @import("std");

// ============================================================
// 几何基元
// ============================================================

pub const GeoType = enum {
    straight,   // 直线 = 张力
    curve,      // 曲线 = 扭力
    cross,      // 交叉 = 分支
    bend,       // 弯折 = 反射
    closed,     // 封闭 = 容器
    open,       // 开口 = 通道
    spiral,     // 螺旋 = 内外共存
    point,      // 点 = 标记
};

pub const Zone = enum {
    top,        // 上方
    bottom,     // 下方
    middle,     // 中间
    inside,     // 内部
    outside,    // 外部
    junction,   // 交界
    endpoint,   // 端点
    inflection, // 拐点
    spiral_boundary, // 螺旋边界
    fine_transition, // 纤细过渡
};

pub const GeoPrimitive = struct {
    geo_type: GeoType,
    zone: Zone,
    desc: []const u8,
};

// ============================================================
// 字母几何注册表
// ============================================================

pub const GlyphInfo = struct {
    symbol: u8,
    primitives: [4]GeoPrimitive, // 最多4个几何基元
    prim_count: usize,
    operation: []const u8, // 操作语义
    zone_used: Zone, // 哪部分被利用

    pub fn describe(self: GlyphInfo) []const u8 {
        return self.operation;
    }
};

// 辅助函数：创建 GeoPrimitive
fn prim(comptime gt: GeoType, comptime z: Zone, comptime d: []const u8) GeoPrimitive {
    return GeoPrimitive{ .geo_type = gt, .zone = z, .desc = d };
}

// ============================================================
// 声母（小写）几何力学表
// ============================================================

pub const glyph_registry = [_]GlyphInfo{
    // b: 竖+右碗，中间交界处被利用
    GlyphInfo{
        .symbol = 'b',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "直线" },
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "曲线" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "交界" },
            GeoPrimitive{ .geo_type = .closed, .zone = .inside, .desc = "封闭" },
        },
        .prim_count = 4,
        .operation = "装入: 直线支撑曲线，中间交界被利用",
        .zone_used = .middle,
    },
    // p: 竖+下碗，中间交界处被利用
    GlyphInfo{
        .symbol = 'p',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "直线" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "曲线" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "交界" },
            GeoPrimitive{ .geo_type = .closed, .zone = .inside, .desc = "封闭" },
        },
        .prim_count = 4,
        .operation = "取出: 直线悬挂曲线，中间交界被利用",
        .zone_used = .middle,
    },
    // d: 左碗+竖，起点终点共享
    GlyphInfo{
        .symbol = 'd',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "曲线" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "直线" },
            GeoPrimitive{ .geo_type = .point, .zone = .endpoint, .desc = "共同起点" },
            GeoPrimitive{ .geo_type = .point, .zone = .endpoint, .desc = "共同终点" },
        },
        .prim_count = 4,
        .operation = "定位: 曲线直线共享起点终点",
        .zone_used = .endpoint,
    },
    // q: 竖弯+尾，内外进出
    GlyphInfo{
        .symbol = 'q',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "直线" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "曲线" },
            GeoPrimitive{ .geo_type = .bend, .zone = .fine_transition, .desc = "纤细过渡" },
            GeoPrimitive{ .geo_type = .curve, .zone = .inside, .desc = "内部曲线" },
        },
        .prim_count = 4,
        .operation = "追踪: 曲线自我包围，内外进出被利用",
        .zone_used = .inside,
    },
    // g: 圆+弯钩，螺旋边界
    GlyphInfo{
        .symbol = 'g',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .inside, .desc = "内螺旋" },
            GeoPrimitive{ .geo_type = .spiral, .zone = .spiral_boundary, .desc = "螺旋边界" },
            GeoPrimitive{ .geo_type = .curve, .zone = .outside, .desc = "外螺旋" },
            GeoPrimitive{ .geo_type = .bend, .zone = .bottom, .desc = "弯钩" },
        },
        .prim_count = 4,
        .operation = "螺旋: 曲线同时产生向内和向外螺旋，分出一个空间内存的螺旋边界",
        .zone_used = .spiral_boundary,
    },
    // j: 竖弯+点，纤细过渡
    GlyphInfo{
        .symbol = 'j',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "直线" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "曲线" },
            GeoPrimitive{ .geo_type = .point, .zone = .top, .desc = "标记点" },
            GeoPrimitive{ .geo_type = .bend, .zone = .fine_transition, .desc = "纤细过渡" },
        },
        .prim_count = 4,
        .operation = "捕获: 直线略多于曲线，只有一个纤细的直线和曲线",
        .zone_used = .fine_transition,
    },
    // m: 三峰，峰谷交替
    GlyphInfo{
        .symbol = 'm',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "峰1" },
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "谷" },
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "峰2" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "基线" },
        },
        .prim_count = 4,
        .operation = "遍历: 三个容器并排，峰谷交替被利用",
        .zone_used = .middle,
    },
    // n: 两峰，峰谷过渡
    GlyphInfo{
        .symbol = 'n',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "峰" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "谷" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "左支撑" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "右支撑" },
        },
        .prim_count = 4,
        .operation = "比较: 框架结构，峰谷过渡被利用",
        .zone_used = .junction,
    },
    // l: 单竖，纯直线
    GlyphInfo{
        .symbol = 'l',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "全直线" },
            GeoPrimitive{ .geo_type = .open, .zone = .top, .desc = "上开口" },
            GeoPrimitive{ .geo_type = .open, .zone = .bottom, .desc = "下开口" },
            GeoPrimitive{ .geo_type = .point, .zone = .endpoint, .desc = "端点" },
        },
        .prim_count = 4,
        .operation = "连接: 纯直线，两端开口，全部被利用",
        .zone_used = .middle,
    },
    // t: 横+竖钩，交叉点
    GlyphInfo{
        .symbol = 't',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "横" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "竖" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "交叉" },
            GeoPrimitive{ .geo_type = .bend, .zone = .bottom, .desc = "钩" },
        },
        .prim_count = 4,
        .operation = "锚定: 交叉点固定位置，弯钩锁定",
        .zone_used = .junction,
    },
    // f: 横+竖弯
    GlyphInfo{
        .symbol = 'f',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "上横" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "竖" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "下横" },
            GeoPrimitive{ .geo_type = .bend, .zone = .bottom, .desc = "弯钩" },
        },
        .prim_count = 4,
        .operation = "支撑: 三向直线，弯钩固定",
        .zone_used = .bottom,
    },
    // i: 竖+点
    GlyphInfo{
        .symbol = 'i',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "竖线" },
            GeoPrimitive{ .geo_type = .point, .zone = .top, .desc = "标记点" },
            GeoPrimitive{ .geo_type = .open, .zone = .bottom, .desc = "下端" },
            GeoPrimitive{ .geo_type = .point, .zone = .endpoint, .desc = "终点" },
        },
        .prim_count = 4,
        .operation = "标记: 直线+点，精确定位",
        .zone_used = .top,
    },
    // r: 竖+右弯
    GlyphInfo{
        .symbol = 'r',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "上竖" },
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "右弯" },
            GeoPrimitive{ .geo_type = .bend, .zone = .junction, .desc = "弯折" },
            GeoPrimitive{ .geo_type = .open, .zone = .bottom, .desc = "下开口" },
        },
        .prim_count = 4,
        .operation = "分支: 直线+短曲线，弯折处被利用",
        .zone_used = .junction,
    },
    // k: 两斜+竖，分叉点
    GlyphInfo{
        .symbol = 'k',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "左斜" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "右斜" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "竖" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "分叉点" },
        },
        .prim_count = 4,
        .operation = "支撑: 从一点向外分叉，扩散力",
        .zone_used = .junction,
    },
    // v: 两斜下，汇聚点
    GlyphInfo{
        .symbol = 'v',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "左上" },
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "右上" },
            GeoPrimitive{ .geo_type = .open, .zone = .top, .desc = "上开口" },
            GeoPrimitive{ .geo_type = .cross, .zone = .bottom, .desc = "汇聚点" },
        },
        .prim_count = 4,
        .operation = "汇聚: 两点向一点收敛",
        .zone_used = .bottom,
    },
    // w: 双谷，分叉再汇聚
    GlyphInfo{
        .symbol = 'w',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "左上" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "左下谷" },
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "右上" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "右下谷" },
        },
        .prim_count = 4,
        .operation = "合并: 两低一高，分叉再汇聚",
        .zone_used = .bottom,
    },
    // x: 两斜交叉
    GlyphInfo{
        .symbol = 'x',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "左上-右下" },
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "右上-左下" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "交叉点" },
            GeoPrimitive{ .geo_type = .open, .zone = .endpoint, .desc = "四端开口" },
        },
        .prim_count = 4,
        .operation = "比较: 两线交叉，找交集",
        .zone_used = .junction,
    },
    // z: Z形折线
    GlyphInfo{
        .symbol = 'z',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "上横" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "斜" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "下横" },
            GeoPrimitive{ .geo_type = .bend, .zone = .junction, .desc = "折转" },
        },
        .prim_count = 4,
        .operation = "振荡: 锯齿形折转，来回切换",
        .zone_used = .junction,
    },
    // c: 弯月，开口
    GlyphInfo{
        .symbol = 'c',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "弯月" },
            GeoPrimitive{ .geo_type = .open, .zone = .middle, .desc = "右开口" },
            GeoPrimitive{ .geo_type = .closed, .zone = .middle, .desc = "左封闭" },
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "曲率" },
        },
        .prim_count = 4,
        .operation = "包围: 半包围，开口向右",
        .zone_used = .right,
    },
    // o: 闭环
    GlyphInfo{
        .symbol = 'o',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .inside, .desc = "全封闭" },
            GeoPrimitive{ .geo_type = .closed, .zone = .inside, .desc = "内部空间" },
            GeoPrimitive{ .geo_type = .curve, .zone = .outside, .desc = "外部边界" },
            GeoPrimitive{ .geo_type = .spiral, .zone = .middle, .desc = "无始无终" },
        },
        .prim_count = 4,
        .operation = "封闭: 完全包围，内部空间被利用",
        .zone_used = .inside,
    },
    // u: 开口框
    GlyphInfo{
        .symbol = 'u',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "左竖" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "右竖" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "底弯" },
            GeoPrimitive{ .geo_type = .open, .zone = .top, .desc = "上开口" },
        },
        .prim_count = 4,
        .operation = "装入: 向上开口的容器，底部曲线",
        .zone_used = .top,
    },
    // h: 右弯+竖
    GlyphInfo{
        .symbol = 'h',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "左竖" },
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "右弯" },
            GeoPrimitive{ .geo_type = .straight, .zone = .middle, .desc = "右竖" },
            GeoPrimitive{ .geo_type = .bend, .zone = .junction, .desc = "弯折" },
        },
        .prim_count = 4,
        .operation = "承载: 椅子形状，弯折处承载上方",
        .zone_used = .junction,
    },
    // s: S形，中间转折
    GlyphInfo{
        .symbol = 's',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .top, .desc = "上弯" },
            GeoPrimitive{ .geo_type = .curve, .zone = .inflection, .desc = "中转折" },
            GeoPrimitive{ .geo_type = .curve, .zone = .bottom, .desc = "下弯" },
            GeoPrimitive{ .geo_type = .bend, .zone = .inflection, .desc = "拐点" },
        },
        .prim_count = 4,
        .operation = "传递: 纯曲线弯转，中间拐点被利用",
        .zone_used = .middle,
    },
    // a: 圆+横，三声之一韵母
    GlyphInfo{
        .symbol = 'a',
        .primitives = .{
            GeoPrimitive{ .geo_type = .curve, .zone = .middle, .desc = "圆" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "横" },
            GeoPrimitive{ .geo_type = .open, .zone = .middle, .desc = "开口" },
            GeoPrimitive{ .geo_type = .cross, .zone = .junction, .desc = "线圆交叉" },
        },
        .prim_count = 4,
        .operation = "容器+辐射: 曲线容器+直线截断，中间空间被利用",
        .zone_used = .middle,
    },
    // e: 横框+开口
    GlyphInfo{
        .symbol = 'e',
        .primitives = .{
            GeoPrimitive{ .geo_type = .straight, .zone = .top, .desc = "上横" },
            GeoPrimitive{ .geo_type = .straight, .zone = .bottom, .desc = "下横" },
            GeoPrimitive{ .geo_type = .open, .zone = .middle, .desc = "右开口" },
            GeoPrimitive{ .geo_type = .closed, .zone = .middle, .desc = "左封闭" },
        },
        .prim_count = 4,
        .operation = "半开: 三向直线+曲线，内部空间被利用",
        .zone_used = .inside,
    },
};

// ============================================================
// 查找字母的几何信息
// ============================================================

pub fn lookupGlyph(symbol: u8) ?GlyphInfo {
    for (glyph_registry) |info| {
        if (info.symbol == symbol) {
            return info;
        }
    }
    return null;
}

// ============================================================
// 计算字母的"力场强度"
// ============================================================

pub fn calculateForce(glyph: GlyphInfo) u8 {
    var force: u8 = 0;
    for (glyph.primitives[0..glyph.prim_count]) |prim| {
        force += switch (prim.geo_type) {
            .straight => 2,   // 直线 = 张力 = 2
            .curve => 3,      // 曲线 = 扭力 = 3
            .cross => 4,      // 交叉 = 分支 = 4
            .bend => 2,       // 弯折 = 反射 = 2
            .closed => 5,     // 封闭 = 容器 = 5
            .open => 1,       // 开口 = 通道 = 1
            .spiral => 6,     // 螺旋 = 内外共存 = 6
            .point => 1,      // 点 = 标记 = 1
        };
    }
    return force;
}

// ============================================================
// 计算两个字母的"相互作用力"
// ============================================================

pub fn calculateInteraction(a: GlyphInfo, b: GlyphInfo) u8 {
    const force_a = calculateForce(a);
    const force_b = calculateForce(b);
    // 加法统摄：两个力场相互作用 = 力的相加（非相乘）
    const interaction = force_a + force_b;
    return interaction;
}

// ============================================================
// 判断字母的"容器兼容性"
// ============================================================

pub fn isCompatible(a: GlyphInfo, b: GlyphInfo) bool {
    // 如果两个字母的 zone_used 相同，它们兼容（可以组合）
    return a.zone_used == b.zone_used;
}

// ============================================================
// 音节几何分析（声母+韵母 = 力场配置）
// ============================================================

pub const SyllableField = struct
    shengmu: GlyphInfo,
    yunmu: GlyphInfo,
    interaction: u8,

    pub fn describe(self: SyllableField) []const u8 {
        return self.shengmu.operation;
    }
};

pub fn analyzeSyllable(sm: u8, ym: u8) ?SyllableField {
    const sm_info = lookupGlyph(sm) orelse return null;
    const ym_info = lookupGlyph(ym) orelse return null;
    const interaction = calculateInteraction(sm_info, ym_info);

    return SyllableField{
        .shengmu = sm_info,
        .yunmu = ym_info,
        .interaction = interaction,
    };
}

// ============================================================
// 单元测试
// ============================================================

test "glyph_lookup_b" {
    const info = lookupGlyph('b').?;
    try std.testing.expectEqual(@as(u8, 'b'), info.symbol);
    try std.testing.expectEqual(Zone.middle, info.zone_used);
}

test "glyph_force_b" {
    const info = lookupGlyph('b').?;
    const force = calculateForce(info);
    // b = straight(2) + curve(3) + junction(未计算) + closed(5) ≈ 10
    try std.testing.expect(force > 0);
}

test "syllable_ma" {
    const field = analyzeSyllable('m', 'a').?;
    try std.testing.expect(field.interaction > 0);
}

test "glyph_spiral_g" {
    const info = lookupGlyph('g').?;
    try std.testing.expectEqual(Zone.spiral_boundary, info.zone_used);
}

test "glyph_closed_o" {
    const info = lookupGlyph('o').?;
    try std.testing.expectEqual(Zone.inside, info.zone_used);
}

test "glyph_cross_x" {
    const info = lookupGlyph('x').?;
    try std.testing.expectEqual(Zone.junction, info.zone_used);
}

test "compatibility_same_zone" {
    const b = lookupGlyph('b').?;
    const p = lookupGlyph('p').?;
    // b和p都是 zone_used = middle
    try std.testing.expect(isCompatible(b, p));
}
