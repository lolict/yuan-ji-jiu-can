// ontology.zig — 容器本体论 (Container Ontology)
//
// 聶小聶匹配：纯粹匹配 — 容器之间本质相吸的强度计算
// ├─ 声部首解：将名称按声母/韵母/声调拆分
// ├─ 公约数折叠：两个名称的最大公约数为匹配锚点
// ├─ 聶小聶匹配值：本质匹配度（非表面相似度）
// └─ 部首星图：容器在关系空间中的星座结构
//
// 核心思想：容器不是因为相似而匹配，而是因为有共同的根（公约数）而凝聚

const std = @import("std");

// ============================================================
// 1. 声母/韵母分解 (Sound Topology)
// ============================================================
pub const SoundUnit = struct {
    initial: []const u8,   // 声母
    final: []const u8,     // 韵母
    tone: u8,              // 声调 (0=轻声, 1-4)

    pub fn formatKey(self: *const SoundUnit, writer: anytype) !void {
        try writer.print("{s}{s}{d}", .{ self.initial, self.final, self.tone });
    }
};

// 声母表 (按满全法声母调入规则简化)
pub const INITIALS = [_][]const u8{
    "m",  // ☆ 木面模
    "k",  // ★ 空坎科
    "f",  // ☆ 风分方
    "g",  // ☆ 光过古
    "x",  // ★ 谐泻夕
    "n",  // ☆ 囡奶能
    "h",  // ☆ 合横化
    "t",  // ☆ 同通天
    "q",  // ☆ 前亲清
    "b",  // ☆ 本比白
    "g_", // ☆ 公谷工
};

// ============================================================
// 2. 部首分解 (Radical Decomposition)
// ============================================================
pub const Radical = struct {
    name: []const u8,
    weight: f64,        // 部首权重
    category: Category,

    pub const Category = enum {
        surface,     // mian  面 — 外在形态
        structure,   // gu    骨 — 结构关系
        motion,      // ye    行 — 动态行为
        essence,     // ben   本 — 本质属性
    };
};

// 部首星图：常用部首及其权重映射
pub const RADICAL_MAP = struct {
    pub fn get(name: []const u8) ?Radical {
        if (std.mem.eql(u8, name, "面")) return Radical{ .name = "surface", .weight = 0.8, .category = .surface };
        if (std.mem.eql(u8, name, "骨")) return Radical{ .name = "structure", .weight = 1.2, .category = .structure };
        if (std.mem.eql(u8, name, "行")) return Radical{ .name = "motion", .weight = 1.0, .category = .motion };
        if (std.mem.eql(u8, name, "本")) return Radical{ .name = "essence", .weight = 1.5, .category = .essence };
        return null;
    }
};

// ============================================================
// 3. 聶小聶匹配引擎 (Nie-Xiao-Qian Match Engine)
// ============================================================
pub const MatchResult = struct {
    a_name: []const u8,
    b_name: []const u8,
    gcd_score: f64,        // 公约数匹配度
    sound_affinity: f64,   // 声韵亲和度
    radical_resonance: f64,// 部首谐振度
    total: f64,            // 综合匹配度
    verdict: Verdict,

    pub const Verdict = enum {
        unite,        // 合一 — 强匹配（公约数 > 0.7）
        resonate,     // 共鸣 — 中匹配（0.4 ~ 0.7）
        neutral,      // 中性 — 弱匹配（0.2 ~ 0.4）
        separate,     // 分离 — 不匹配（< 0.2）
    };
};

pub const OntologyEngine = struct {
    allocator: std.mem.Allocator,
    results: std.ArrayList(MatchResult),

    pub fn init(allocator: std.mem.Allocator) OntologyEngine {
        return OntologyEngine{
            .allocator = allocator,
            .results = std.ArrayList(MatchResult).init(allocator),
        };
    }

    pub fn deinit(self: *OntologyEngine) void {
        self.results.deinit();
    }

    // 计算公约数 (GCD of two u64 values)
    pub fn gcd(a: u64, b: u64) u64 {
        var x = a;
        var y = b;
        while (y != 0) {
            const t = y;
            y = x % y;
            x = t;
        }
        return x;
    }

    // 声韵亲和度：两个音韵单元的声母匹配
    pub fn soundAffinity(initial_a: []const u8, initial_b: []const u8) f64 {
        if (std.mem.eql(u8, initial_a, initial_b)) return 1.0;
        // 声母按距离衰减
        return 0.3;
    }

    // 部首谐振度：两个名的部首匹配
    pub fn radicalResonance(name_a: []const u8, name_b: []const u8) f64 {
        const ra = RADICAL_MAP.get(name_a);
        const rb = RADICAL_MAP.get(name_b);
        if (ra == null or rb == null) return 0.0;
        if (ra.?.category == rb.?.category) return 1.0;
        return 0.5;
    }

    // 核心：聶小聶匹配 — 计算两个名称的本质匹配度
    pub fn match(self: *OntologyEngine, id_a: u32, id_b: u32, name_a: []const u8, name_b: []const u8) !void {
        // 1. 公约数匹配 — ID数值的最大公约数
        const g = gcd(id_a, id_b);
        const gcd_score: f64 = if (g > 1)
            @as(f64, @floatFromInt(g)) / @as(f64, @floatFromInt(@max(id_a, id_b)))
        else 0.1;

        // 2. 声韵亲和 — 首字声母（简化）
        const sound_aff = soundAffinity(name_a[0..@min(1, name_a.len)], name_b[0..@min(1, name_b.len)]);

        // 3. 部首谐振 — 名称尾部隐含的部首
        const radical_res = radicalResonance(name_a, name_b);

        // 4. 综合 = 公约数(r=0.5) + 声韵(r=0.3) + 部首(r=0.2)
        const total = 0.5 * gcd_score + 0.3 * sound_aff + 0.2 * radical_res;

        const verdict: MatchResult.Verdict = if (total > 0.7) .unite
            else if (total > 0.4) .resonate
            else if (total > 0.2) .neutral
            else .separate;

        try self.results.append(MatchResult{
            .a_name = name_a,
            .b_name = name_b,
            .gcd_score = gcd_score,
            .sound_affinity = sound_aff,
            .radical_resonance = radical_res,
            .total = total,
            .verdict = verdict,
        });
    }

    // 名称"公约数编码" — 提取名称中隐藏的公约数
    pub fn extractCommonRoot(name: []const u8) []const u8 {
        // 返回名称的核心部首/声母作为公约数根
        if (name.len == 0) return "☆";

        // 如果名称中有知道的部首，返回它
        if (std.mem.indexOf(u8, name, "面") != null) return "面";
        if (std.mem.indexOf(u8, name, "骨") != null) return "骨";
        if (std.mem.indexOf(u8, name, "行") != null) return "行";
        if (std.mem.indexOf(u8, name, "本") != null) return "本";

        // 默认：第一个字的声母作为公约数根
        return name[0..1];
    }

    // 格式化匹配报告 (FQM 格式)
    pub fn formatReport(self: *OntologyEngine, allocator: std.mem.Allocator) ![]const u8 {
        var out = std.ArrayList(u8).init(allocator);
        defer out.deinit();

        try out.appendSlice("# 容器本体论 · 聶小聶匹配报告\n");
        try out.appendSlice("matches: ");
        try formatUnary(&out, self.results.items.len);
        try out.appendSlice("\n\n");

        for (self.results.items, 0..) |*r, i| {
            try out.appendSlice("~match:");
            try formatUnary(&out, i + 1);
            try out.appendSlice("\n");
            try out.appendSlice("  pair: ");
            try out.appendSlice(r.a_name);
            try out.appendSlice(" · ");
            try out.appendSlice(r.b_name);
            try out.appendSlice("\n");
            try out.appendSlice("  verdict: ");
            try out.appendSlice(@tagName(r.verdict));
            try out.appendSlice("\n");
            try out.appendSlice("  total: ");
            try formatDecimal(&out, r.total);
            try out.appendSlice("\n");
            try out.appendSlice("  factors: gcd=");
            try formatDecimal(&out, r.gcd_score);
            try out.appendSlice("|sound=");
            try formatDecimal(&out, r.sound_affinity);
            try out.appendSlice("|radical=");
            try formatDecimal(&out, r.radical_resonance);
            try out.appendSlice("\n\n");
        }

        // 部首星图汇总
        try out.appendSlice("#\n");
        try out.appendSlice("\n# 部首星图 (Radical Constellation)\n");
        var unity_count: usize = 0;
        var resonate_count: usize = 0;
        var neutral_count: usize = 0;
        var separate_count: usize = 0;
        for (self.results.items) |r| {
            switch (r.verdict) {
                .unite => unity_count += 1,
                .resonate => resonate_count += 1,
                .neutral => neutral_count += 1,
                .separate => separate_count += 1,
            }
        }
        try out.appendSlice("  unite: ");
        try formatUnary(&out, unity_count);
        try out.appendSlice("  resonate: ");
        try formatUnary(&out, resonate_count);
        try out.appendSlice("  neutral: ");
        try formatUnary(&out, neutral_count);
        try out.appendSlice("  separate: ");
        try formatUnary(&out, separate_count);
        try out.appendSlice("\n");

        return out.toOwnedSlice();
    }
};

// ============================================================
// 辅助函数
// ============================================================
fn formatUnary(writer: *std.ArrayList(u8), value: usize) !void {
    if (value == 0) {
        try writer.appendSlice("☆");
        return;
    }
    var r = value;
    while (r > 0) : (r -= 1) {
        try writer.appendSlice("★");
    }
}

fn formatDecimal(writer: *std.ArrayList(u8), value: f64) !void {
    const percent = @as(i64, @intFromFloat(@round(value * 100)));
    const major = @as(usize, @intCast(@divTrunc(percent, 100)));
    const minor = @as(usize, @intCast(@as(usize, @intCast(percent)) % 100));
    const tens = minor / 10;
    const ones = minor % 10;
    const formatted = try std.fmt.allocPrint(std.heap.page_allocator, "{d}.{d}{d}%", .{ major, tens, ones });
    defer std.heap.page_allocator.free(formatted);
    try writer.appendSlice(formatted);
}

// ============================================================
// 测试
// ============================================================
test "ontology gcd calculation" {
    try std.testing.expectEqual(@as(u64, 6), OntologyEngine.gcd(12, 18));
    try std.testing.expectEqual(@as(u64, 1), OntologyEngine.gcd(7, 13));
    try std.testing.expectEqual(@as(u64, 4), OntologyEngine.gcd(8, 12));
}

test "ontology match engine" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var engine = OntologyEngine.init(allocator);
    defer engine.deinit();

    // ID=2 和 ID=4 的公约数为 2
    try engine.match(2, 4, "面_base", "骨_base");

    try std.testing.expectEqual(@as(usize, 1), engine.results.items.len);
    try std.testing.expect(engine.results.items[0].gcd_score > 0.3); // gcd(2,4)/max(2,4) = 2/4 = 0.5
}

test "ontology verdict unite" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var engine = OntologyEngine.init(allocator);
    defer engine.deinit();

    // ID=6 和 ID=12，gcd=6，gcd_score = 6/12 = 0.5
    // total = 0.5*0.5 + 0.3*0.3 + 0.2*0.5 = 0.25+0.09+0.1 = 0.44 → resonate
    try engine.match(6, 12, "面_structure", "面_motion");

    // Same name prefix (first char match)
    const result = engine.results.items[0];
    try std.testing.expect(result.verdict == .resonate or result.verdict == .unite);
}

test "ontology common root extraction" {
    try std.testing.expectEqualStrings("面", OntologyEngine.extractCommonRoot("面_facade"));
    try std.testing.expectEqualStrings("骨", OntologyEngine.extractCommonRoot("骨_core"));
    try std.testing.expectEqualStrings("☆", OntologyEngine.extractCommonRoot(""));
}
