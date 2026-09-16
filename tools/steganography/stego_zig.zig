// @name: stego_zig
// @type: steganography-shell
// @owner: 五彩石
// @status: stable
// @priority: P1
// @description: 五彩石隐写二进制外壳 - Zig实现

const std = @import("std");
const Allocator = std.mem.Allocator;

// === Unicode 隐写核心 ===

const ZWSP: u21 = 0x200B;  // bit 0
const ZWNJ: u21 = 0x200C;  // bit 1
const ZWJ:  u21 = 0x200D;  // 分隔符
const BOM:  u21 = 0xFEFF;  // 结束符

/// 将 payload 嵌入 carrier 文本（零宽字符编码）
pub fn unicodeEncode(allocator: *Allocator, carrier: []const u8, payload: []const u8) ![]u8 {
    // 计算需要的总比特数：4字节长度头 + payload
    const total_bits: usize = (4 + payload.len) * 8;
    // 每个bit用2个码位表示：(bit_char, ZWJ)
    // 再加上可能的 BOM
    const result_len = carrier.len + total_bits * 2 + 1;
    
    var result = try allocator.alloc(u8, result_len);
    errdefer allocator.free(result);
    
    // 写入 carrier
    std.mem.copy(u8, result[0..carrier.len], carrier);
    var pos: usize = carrier.len;
    
    // 写入4字节长度头（大端序）
    const plen = payload.len;
    var header: [4]u8 = undefined;
    header[0] = @truncate(u8, @bitCast(u32, @as(u32, @intCast(u32, plen))) >> 24);
    header[1] = @truncate(u8, @bitCast(u32, @as(u32, @intCast(u32, plen))) >> 16);
    header[2] = @truncate(u8, @bitCast(u32, @as(u32, @intCast(u32, plen))) >> 8);
    header[3] = @truncate(u8, @intCast(u32, plen));
    
    // 编码 header
    for (header) |byte| {
        var bitPos: u8 = 8;
        while (bitPos > 0) {
            bitPos -= 1;
            const bit = (byte >> bitPos) & 0x01;
            const ch = if (bit == 0) ZWSP else ZWNJ;
            // 编码为 UTF-8 (U+200B..U+200D 在 UTF-8 中都是 3 字节)
            const written = std.unicode.utf8Encode(ch, result[pos..]) catch unreachable;
            pos += written;
            // 分隔符 ZWJ
            const sep = std.unicode.utf8Encode(ZWJ, result[pos..]) catch unreachable;
            pos += sep;
        }
    }
    
    // 编码 payload
    for (payload) |c| {
        var bitPos2: u8 = 8;
        while (bitPos2 > 0) {
            bitPos2 -= 1;
            const bit = (@as(u21, c) >> bitPos2) & 0x01;
            const ch3 = if (bit == 0) ZWSP else ZWNJ;
            const w4 = std.unicode.utf8Encode(ch3, result[pos..]) catch unreachable;
            pos += w4;
            const s2 = std.unicode.utf8Encode(ZWJ, result[pos..]) catch unreachable;
            pos += s2;
        }
    }
    
    // 结束符
    const bom = std.unicode.utf8Encode(BOM, result[pos..]) catch unreachable;
    pos += bom;
    
    // 调整为实际大小
    const actual = allocator.realloc(result, pos) catch {
        allocator.free(result);
        return error.OutOfMemory;
    };
    return actual;
}

/// 从 carrier 文本中解码 payload
pub fn unicodeDecode(allocator: *Allocator, data: []const u8) !?[]u8 {
    // 提取零宽字符序列直到 BOM
    var zwSeq: [8192]u21 = undefined;
    var zwLen: usize = 0;
    
    var it = std.unicode.Utf8View.initUnchecked(data);
    while (it.nextCodepoint()) |cp| {
        if (cp == BOM) break;
        if (cp == ZWSP or cp == ZWNJ or cp == ZWJ) {
            if (zwLen < zwSeq.len) {
                zwSeq[zwLen] = cp;
                zwLen += 1;
            }
        }
    }
    
    if (zwLen < 64 return null;  // 至少需要 32 bits（4字节头）
    
    // 解码比特（跳过每个 ZWJ 分隔符）
    var bits: [8]u8 = [_]u8{0} ** 8;
    var bitsCollected: usize = 0;
    var bytesDecoded: usize = 0;
    var byteAcc: u8 = 0;
    var collected: usize = 0;
    
    var extracted = std.ArrayList(u8).init(allocator);
    defer extracted.deinit();
    
    var i: usize = 0;
    while (i < zwLen) : (i += 1) {
        // 取 bit 字符
        const cp = zwSeq[i];
        if (cp != ZWSP and cp != ZWNJ) continue;
        const bit = if (cp == ZWSP) 0 else 1;
        byteAcc = (byteAcc << 1) | bit;
        collected += 1;
        if (collected == 8 {
            try extracted.append(byteAcc);
            byteAcc = 0;
            collected = 0;
        }
        # 跳过分隔符 ZWJ
        if (i + 1 < zwLen and zwSeq[i + 1] == ZWJ) {
            i += 1;
        }
    }
    
    if (extracted.items.len < 4) return null;
    
    const plen = (@as(u32, extracted.items[0]) << 24) |
                 (@as(u32, extracted.items[1]) << 16) |
                 (@as(u32, extracted.items[2]) << 8) |
                 @as(u32, extracted.items[3]);
    
    if (plen == 0 or plen > 1000000 or @as(u32, extracted.items.len) < 4 + plen) {
        return null;
    }
    
    var result = try allocator.alloc(u8, plen);
    std.mem.copy(u8, result, extracted.items[4..4+plen]);
    return result;
}


// === HTML 隐写 ===

pub fn htmlEncode(allocator: *Allocator, htmlContent: []const u8, payload: []const u8) ![]u8 {
    const encoded = std.base64.standard.allocatorEncode(allocator, payload) catch {
        return error.OutOfMemory;
    };
    defer allocator.free(encoded);
    
    const comment = try std.fmt.allocPrint(allocator, "<!-- csl3:{s} -->", .{encoded});
    const hiddenSpan = try std.fmt.allocPrint(allocator,
        "<span style=\"display:none;font-size:0;opacity:0\">" ++
        "<!--csl3-->{s}<!--/csl3--></span>", .{encoded});
    
    // 尝试插入到 前
    const headClose = std.mem.indexOf(u8, htmlContent, "</head>");
    var result: []u8 = undefined;
    
    if (headClose) |idx| {
        result = try std.fmt.allocPrint(allocator,
            "{s}\n{s}\n{s}\n{s}",
            .{ htmlContent[0..idx], comment, hiddenSpan, htmlContent[idx..] });
    } else |_| {
        result = try std.fmt.allocPrint(allocator, "{s}\n{s}\n{s}\n",
            .{ htmlContent, comment, hiddenSpan });
    }
    
    allocator.free(comment);
    allocator.free(hiddenSpan);
    return result;
}

pub fn htmlDecode(allocator: *Allocator, htmlContent: []const u8) !?[]u8 {
    // 从注释中提取
    const patC = "<!-- csl3:";
    const patC2 = " -->";
    
    if (std.mem.indexOf(u8, htmlContent, patC)) |startC| {
        const innerStart = startC + patC.len;
        if (std.mem.indexOfPos(u8, htmlContent, innerStart, patC2)) |innerEnd| {
            const encoded = htmlContent[innerStart..innerEnd];
            const decoded = std.base64.standard.allocatorDecode(allocator, encoded) catch {
                return null;
            };
            return decoded;
        }
    }
    
    // 从隐藏 span 提取
    const patS = "<!--csl3-->";
    const patS2 = "<!--/csl3-->";
    
    if (std.mem.indexOf(u8, htmlContent, patS)) |startS| {
        const innerStart = startS + patS.len;
        if (std.mem.indexOfPos(u8, htmlContent, innerStart, patS2)) |innerEnd| {
            const encoded = htmlContent[innerStart..innerEnd];
            const decoded = std.base64.standard.allocatorDecode(allocator, encoded) catch {
                return null;
            };
            return decoded;
        }
    }
    
    return null;
}


// === CLI ===

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = &gpa.allocator;
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 3) {
        std.debug.print("stego_zig <encode|decode> <mode> <input> [output] [--payload text]\n", .{});
        std.debug.print("  mode = unicode | html | lsb\n", .{});
        return error.InvalidArgs;
    }
    
    const action = args[1];
    const mode = args[2];
    
    if (std.mem.eql(u8, action, "encode")) {
        if (args.len < 5) {
            std.debug.print("需要 input output [--payload text]\n", .{});
            return error.InvalidArgs;
        }
        
        const input = args[3];
        const output = args[4];
        
        var payload: []const u8 = "";
        var i: usize = 5;
        while (i + 1 < args.len) : (i += 1) {
            if (std.mem.eql(u8, args[i], "--payload")) {
                payload = args[i + 1];
                break;
            }
        }
        
        const cin = std.fs.readFile(allocator, input) catch |err| {
            std.debug.print("读取失败: {s}\n", .{@errorName(err)});
            return err;
        };
        defer allocator.free(cin);
        
        if (std.mem.eql(u8, mode, "unicode")) {
            const result = try unicodeEncode(allocator, cin, payload);
            defer allocator.free(result);
            try std.fs.writeFile(output, result);
            std.debug.print("已编码 {d} 字符到 {s}\n", .{ payload.len, output });
        } else if (std.mem.eql(u8, mode, "html")) {
            const result = try htmlEncode(allocator, cin, payload);
            defer allocator.free(result);
            try std.fs.writeFile(output, result);
            std.debug.print("已编码 {d} 字符到 {s}\n", .{ payload.len, output });
        } else if (std.mem.eql(u8, mode, "lsb")) {
            std.debug.print("Zig LSB 需要 PNG 库，暂用 Unicode/HTML\n", .{});
        }
        
    } else if (std.mem.eql(u8, action, "decode")) {
        if (args.len < 4) {
            std.debug.print("需要 input\n", .{});
            return error.InvalidArgs;
        }
        
        const input = args[3];
        const content = std.fs.readFile(allocator, input) catch |err| {
            std.debug.print("读取失败: {s}\n", .{@errorName(err)});
            return err;
        };
        defer allocator.free(content);
        
        if (std.mem.eql(u8, mode, "unicode")) {
            const result = try unicodeDecode(allocator, content) orelse {
                std.debug.print("解码失败\n", .{});
                return error.DecodeFailed;
            };
            defer allocator.free(result);
            std.debug.print("{s}\n", .{result});
        } else if (std.mem.eql(u8, mode, "html")) {
            const result = try htmlDecode(allocator, content) orelse {
                std.debug.print("解码失败\n", .{});
                return error.DecodeFailed;
            };
            defer allocator.free(result);
            std.debug.print("{s}\n", .{result });
        }
    }
}
