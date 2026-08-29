// loader.zig - FQM 加载器 · 零运行时内存
//
// 解析 .fqm 容器定义文件，所有解析在编译时完成
// 输出结构化的容器数据供链接器使用

const std = @import("std");
const constants = @import("constants.zig");

// === 数据结构 ===

pub const SymbolEntry = struct {
    symbol: []const u8,
    position: u8,
};

pub const Container = struct {
    name: []const u8,
    container_type: []const u8,
    capacity: u8,
    polarity: u8,
    symbols: [constants.MAX_SYMBOLS]SymbolEntry,
    symbol_count: u8,
};

pub const Link = struct {
    from_domain: []const u8,
    from_symbol: []const u8,
    to_domain: []const u8,
    to_symbol: []const u8,
};

pub const Synonym = struct {
    standard: []const u8,
    variants: [constants.MAX_SYNONYMS][]const u8,
    variant_count: u8,
};

pub const FQMData = struct {
    containers: [constants.MAX_CONTAINERS]Container,
    container_count: u8,
    links: [constants.MAX_LINKS]Link,
    link_count: u8,
    synonyms: [constants.MAX_SYNONYMS]Synonym,
    synonym_count: u8,
    offset: u8,
};

// === 编译时工具函数 ===

fn trim(comptime s: []const u8) []const u8 {
    comptime var start: usize = 0;
    comptime var end: usize = s.len;
    inline while (start < end) : (start += 1) {
        const c = s[start];
        if (c != ' ' and c != '\t' and c != '\r') break;
    }
    inline while (end > start) : (end -= 1) {
        const c = s[end - 1];
        if (c != ' ' and c != '\t' and c != '\r') break;
    }
    return s[start..end];
}

fn parseU8(comptime s: []const u8) u8 {
    if (std.mem.startsWith(u8, s, "0x") or std.mem.startsWith(u8, s, "0X")) {
        comptime var result: u8 = 0;
        inline for (s[2..]) |c| {
            result *%= 16;
            if (c >= '0' and c <= '9') result +%= c - '0'
            else if (c >= 'a' and c <= 'f') result +%= c - 'a' + 10
            else if (c >= 'A' and c <= 'F') result +%= c - 'A' + 10;
        }
        return result;
    }
    comptime var result: u8 = 0;
    inline for (s) |c| {
        if (c >= '0' and c <= '9') {
            result *%= 10;
            result +%= c - '0';
        }
    }
    return result;
}

// Find container index by name, returns container_count if not found
fn findContainerIdx(comptime data: FQMData, comptime name: []const u8) u8 {
    var k: u8 = 0;
    while (k < data.container_count) : (k += 1) {
        if (std.mem.eql(u8, data.containers[k].name, name)) return k;
    }
    return data.container_count; // not found
}

// === 编译时解析 ===

pub fn parseFQM(comptime content: []const u8) FQMData {
    @setEvalBranchQuota(50000);
    comptime var data: FQMData = undefined;
    data.container_count = 0;
    data.link_count = 0;
    data.synonym_count = 0;
    data.offset = 0;

    comptime var pos: usize = 0;
    while (pos < content.len) : ({}) {
        comptime var line_end: usize = pos;
        while (line_end < content.len) : (line_end += 1) {
            if (content[line_end] == '\n') break;
        }
        const line = trim(content[pos..line_end]);
        pos = line_end + 1;

        if (line.len == 0 or line[0] == '#') continue;

        const eq_pos = std.mem.indexOfScalar(u8, line, '=') orelse continue;
        const left = trim(line[0..eq_pos]);
        const right = trim(line[eq_pos + 1 ..]);

        if (std.mem.eql(u8, left, "偏")) {
            data.offset = parseU8(right);
        } else if (std.mem.startsWith(u8, left, "近.")) {
            const standard = left["近.".len..];
            const idx = data.synonym_count;
            if (idx >= constants.MAX_SYNONYMS) continue;
            data.synonyms[idx].standard = standard;
            data.synonyms[idx].variant_count = 0;

            comptime var vpos: usize = 0;
            while (vpos < right.len) : ({}) {
                comptime var item_end: usize = vpos;
                while (item_end < right.len) : (item_end += 1) {
                    if (right[item_end] == ',') break;
                }
                const v = trim(right[vpos..item_end]);
                vpos = item_end + 1;
                if (v.len == 0) continue;
                const vc = data.synonyms[idx].variant_count;
                if (vc >= constants.MAX_SYNONYMS) break;
                data.synonyms[idx].variants[vc] = v;
                data.synonyms[idx].variant_count = vc + 1;
            }
            data.synonym_count = idx + 1;
        } else if (std.mem.startsWith(u8, left, "袋.")) {
            const first_dot = std.mem.indexOfScalar(u8, left, '.') orelse continue;
            const sd = std.mem.indexOfScalar(u8, left[first_dot + 1 ..], '.') orelse continue;
            const second_dot = first_dot + 1 + sd;
            const domain = left[first_dot + 1 .. second_dot];
            const attr = left[second_dot + 1 ..];

            if (std.mem.eql(u8, attr, "容")) {
                // Find or create container (inlined)
                var found_idx: u8 = 0;
                var found: bool = false;
                var k: u8 = 0;
                while (k < data.container_count) : (k += 1) {
                    if (std.mem.eql(u8, data.containers[k].name, domain)) {
                        found_idx = k;
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    found_idx = data.container_count;
                    if (data.container_count < constants.MAX_CONTAINERS) {
                        data.containers[data.container_count] = .{
                            .name = domain, .container_type = domain,
                            .capacity = 0, .polarity = 0,
                            .symbols = undefined, .symbol_count = 0,
                        };
                        data.container_count += 1;
                    }
                }
                // Add symbols
                comptime var spos: usize = 0;
                while (spos < right.len) : ({}) {
                    comptime var item_end: usize = spos;
                    while (item_end < right.len) : (item_end += 1) {
                        if (right[item_end] == ',') break;
                    }
                    const sym = trim(right[spos..item_end]);
                    spos = item_end + 1;
                    if (sym.len == 0) continue;

                    // Check duplicate
                    var dup = false;
                    var m: u8 = 0;
                    while (m < data.containers[found_idx].symbol_count) : (m += 1) {
                        if (std.mem.eql(u8, data.containers[found_idx].symbols[m].symbol, sym)) {
                            dup = true;
                            break;
                        }
                    }
                    if (dup) continue;

                    const sc = data.containers[found_idx].symbol_count;
                    if (sc >= constants.MAX_SYMBOLS) break;
                    data.containers[found_idx].symbols[sc] = .{ .symbol = sym, .position = sc };
                    data.containers[found_idx].symbol_count = sc + 1;
                }
            } else if (std.mem.eql(u8, attr, "进制")) {
                // Find or create container (inlined)
                var found_idx: u8 = 0;
                var found: bool = false;
                var k: u8 = 0;
                while (k < data.container_count) : (k += 1) {
                    if (std.mem.eql(u8, data.containers[k].name, domain)) {
                        found_idx = k;
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    found_idx = data.container_count;
                    if (data.container_count < constants.MAX_CONTAINERS) {
                        data.containers[data.container_count] = .{
                            .name = domain, .container_type = domain,
                            .capacity = 0, .polarity = 0,
                            .symbols = undefined, .symbol_count = 0,
                        };
                        data.container_count += 1;
                    }
                }
                data.containers[found_idx].capacity = parseU8(right);
            } else if (std.mem.eql(u8, attr, "极性")) {
                // Find or create container (inlined)
                var found_idx: u8 = 0;
                var found: bool = false;
                var k: u8 = 0;
                while (k < data.container_count) : (k += 1) {
                    if (std.mem.eql(u8, data.containers[k].name, domain)) {
                        found_idx = k;
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    found_idx = data.container_count;
                    if (data.container_count < constants.MAX_CONTAINERS) {
                        data.containers[data.container_count] = .{
                            .name = domain, .container_type = domain,
                            .capacity = 0, .polarity = 0,
                            .symbols = undefined, .symbol_count = 0,
                        };
                        data.container_count += 1;
                    }
                }
                data.containers[found_idx].polarity = parseU8(right);
            } else if (std.mem.startsWith(u8, attr, "引.")) {
                const target_domain = attr["引.".len..];
                comptime var lpos: usize = 0;
                while (lpos < right.len) : ({}) {
                    comptime var item_end: usize = lpos;
                    while (item_end < right.len) : (item_end += 1) {
                        if (right[item_end] == ',') break;
                    }
                    const pair = trim(right[lpos..item_end]);
                    lpos = item_end + 1;
                    if (pair.len == 0) continue;

                    const arrow = "→";
                    const arrow_pos = std.mem.indexOf(u8, pair, arrow) orelse continue;
                    const from_sym = trim(pair[0..arrow_pos]);
                    const to_sym = trim(pair[arrow_pos + arrow.len ..]);

                    const lc = data.link_count;
                    if (lc >= constants.MAX_LINKS) break;
                    data.links[lc] = .{
                        .from_domain = domain, .from_symbol = from_sym,
                        .to_domain = target_domain, .to_symbol = to_sym,
                    };
                    data.link_count = lc + 1;
                }
            }
        }
    }

    return data;
}

// === 嵌入 FQM 文件并解析 ===

pub const FQM_CONTENT =
    \\# FQM 容器定义文件
    \\# 声母容器（23个）
    \\袋.声母.容=b,p,m,f,d,t,n,l,g,k,h,j,q,x,zh,ch,sh,r,z,c,s,y,w
    \\袋.声母.进制=23
    \\袋.声母.极性=1
    \\# 韵母容器（24个）
    \\袋.韵母.容=a,o,e,i,u,v,ai,ei,ui,ao,ou,iu,ie,ve,er,an,en,in,un,vn,ang,eng,ing,ong
    \\袋.韵母.进制=24
    \\袋.韵母.极性=0
    \\# 声调容器（4个）
    \\袋.声调.容=1,2,3,4
    \\袋.声调.进制=4
    \\袋.声调.极性=1
    \\# 跨容器链接：声母→韵母
    \\袋.声母.引.韵母=b→a,p→o,m→e,f→i,d→u,t→v
    \\# 近义词表
    \\近.分=份,纷,芬
    \\近.家=嘉,佳,加
    \\近.园=元,圆,原
    \\近.召=招,沼,照
    \\# 全局偏移
    \\偏=0x00
;

pub const fqm_data = parseFQM(FQM_CONTENT);

// === 查找函数 ===

pub fn getContainer(comptime name: []const u8) ?Container {
    for (fqm_data.containers[0..fqm_data.container_count]) |c| {
        if (std.mem.eql(u8, c.name, name)) return c;
    }
    return null;
}

pub fn getSymbol(comptime container_name: []const u8, comptime symbol: []const u8) ?SymbolEntry {
    const container = getContainer(container_name) orelse return null;
    for (container.symbols[0..container.symbol_count]) |entry| {
        if (std.mem.eql(u8, entry.symbol, symbol)) return entry;
    }
    return null;
}

pub fn getSymbolByPosition(comptime container_name: []const u8, comptime position: u8) ?SymbolEntry {
    const container = getContainer(container_name) orelse return null;
    for (container.symbols[0..container.symbol_count]) |entry| {
        if (entry.position == position) return entry;
    }
    return null;
}

pub fn getLinks() []const Link {
    return fqm_data.links[0..fqm_data.link_count];
}

pub fn getSynonyms() []const Synonym {
    return fqm_data.synonyms[0..fqm_data.synonym_count];
}

pub fn getOffset() u8 {
    return fqm_data.offset;
}

// === 显示 ===

pub fn displaySummary() void {
    const stdout = std.io.getStdOut().writer();
    printLine(stdout, 60, '=');
    stdout.print("FQM 加载结果\n", .{}) catch {};
    printLine(stdout, 60, '-');
    stdout.print("容器数量: {}\n", .{fqm_data.container_count}) catch {};
    stdout.print("链接数量: {}\n", .{fqm_data.link_count}) catch {};
    stdout.print("近义词数量: {}\n", .{fqm_data.synonym_count}) catch {};
    stdout.print("全局偏移: 0x{X:02}\n", .{fqm_data.offset}) catch {};

    for (fqm_data.containers[0..fqm_data.container_count]) |c| {
        stdout.print("  {s}: {} 个符号\n", .{ c.name, c.symbol_count }) catch {};
    }

    printLine(stdout, 60, '=');
    stdout.print("\n", .{}) catch {};
}

fn printLine(writer: anytype, comptime n: usize, comptime ch: u8) void {
    comptime var i: usize = 0;
    inline while (i < n) : (i += 1) {
        writer.print("{c}", .{ch}) catch {};
    }
    writer.print("\n", .{}) catch {};
}

// === 测试 ===

test "loader_parse_fqm" {
    const data = fqm_data;
    try std.testing.expect(data.container_count >= 3);
    try std.testing.expect(data.link_count >= 6);
    try std.testing.expect(data.synonym_count >= 4);
}

test "loader_get_container" {
    const c = getContainer("声母").?;
    try std.testing.expect(c.symbol_count >= 23);
    try std.testing.expectEqual(@as(u8, 23), c.capacity);
    try std.testing.expectEqual(@as(u8, 1), c.polarity);
}

test "loader_get_symbol" {
    const sym = getSymbol("声母", "b").?;
    try std.testing.expectEqual(@as(u8, 0), sym.position);
    try std.testing.expectEqualStrings("b", sym.symbol);
}

test "loader_get_symbol_by_position" {
    const sym = getSymbolByPosition("声母", 0).?;
    try std.testing.expectEqualStrings("b", sym.symbol);
}

test "loader_links" {
    const links = getLinks();
    try std.testing.expect(links.len >= 6);
    try std.testing.expectEqualStrings("b", links[0].from_symbol);
    try std.testing.expectEqualStrings("a", links[0].to_symbol);
}

test "loader_synonyms" {
    const syns = getSynonyms();
    try std.testing.expect(syns.len >= 4);
    try std.testing.expectEqualStrings("分", syns[0].standard);
}

test "loader_offset" {
    try std.testing.expectEqual(@as(u8, 0x00), getOffset());
}
