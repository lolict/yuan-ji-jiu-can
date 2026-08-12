// test_main.zig — mlil 编译器单元测试
const std = @import("std");
const testing = std.testing;
const Lexer = @import("lexer.zig").Lexer;
const Parser = @import("parser.zig").Parser;
const WasmGen = @import("wasmgen.zig").WasmGen;
const TokenMod = @import("token.zig");
const Token = TokenMod.Token;
const Ast = @import("ast.zig");

test "token type keyword detection" {
    try testing.expect(TokenMod.TokenType.kw_module.isKeyword());
    try testing.expect(TokenMod.TokenType.kw_surface.isKeyword());
    try testing.expect(TokenMod.TokenType.kw_fn.isKeyword());
    try testing.expect(!TokenMod.TokenType.int_literal.isKeyword());
    try testing.expect(!TokenMod.TokenType.identifier.isKeyword());
}

test "symbol name mapping" {
    try testing.expectEqual(@as(u6, 0), TokenMod.symbolNameToValue("ymba").?);
    try testing.expectEqual(@as(u32, 63), TokenMod.symbolNameToValue("lyrm").?);
    try testing.expectEqual(@as(u32, 4), TokenMod.symbolNameToValue("sita").?);
    try testing.expect(TokenMod.symbolNameToValue("notasymbol") == null);
}

test "lexer basic tokens" {
    const source = "fn hello() { return 42 }";
    const alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    const t0 = try lexer.next();
    try testing.expectEqual(TokenMod.TokenType.kw_fn, t0.typ);

    const t1 = try lexer.next();
    try testing.expectEqual(TokenMod.TokenType.identifier, t1.typ);
    try testing.expectEqualStrings("hello", t1.lexeme);

    const t2 = try lexer.next();
    try testing.expectEqual(TokenMod.TokenType.lparen, t2.typ);

    const t3 = try lexer.next();
    try testing.expectEqual(TokenMod.TokenType.rparen, t3.typ);
}

test "parser simple module" {
    const source =
        \\export fn test() -> angle {
        \\    return 120°
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expectEqualStrings("test", module.name);
}

test "wasm magic number generation" {
    const source =
        \\export fn noop() {}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("noop");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    var gen = WasmGen.init(alloc);
    defer gen.deinit();

    const bytes = try gen.generate(module);

    // 验证 WASM 魔数: 0x00 0x61 0x73 0x6D = \0asm
    try testing.expectEqual(@as(u8, 0x00), bytes[0]);
    try testing.expectEqual(@as(u8, 0x61), bytes[1]);
    try testing.expectEqual(@as(u8, 0x73), bytes[2]);
    try testing.expectEqual(@as(u8, 0x6D), bytes[3]);

    // 验证版本号: 0x01 0x00 0x00 0x00 = v1
    try testing.expectEqual(@as(u8, 0x01), bytes[4]);
    try testing.expectEqual(@as(u8, 0x00), bytes[5]);
    try testing.expectEqual(@as(u8, 0x00), bytes[6]);
    try testing.expectEqual(@as(u8, 0x00), bytes[7]);
}

test "wasm export section present" {
    const source =
        \\export fn my_func() -> angle {
        \\    return 90°
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("my");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    var gen = WasmGen.init(alloc);
    defer gen.deinit();

    const bytes = try gen.generate(module);

    // 查找导出段 ID (0x07)
    var found_export = false;
    var pos: usize = 8; // 跳过魔数+版本
    while (pos < bytes.len) {
        const section_id = bytes[pos];
        pos += 1;
        if (section_id == 0x07) {
            found_export = true;
            break;
        }
        // 跳过段内容 (简化: 假设段长度 < 128)
        const len = bytes[pos];
        pos += 1 + @as(usize, len);
    }

    try testing.expect(found_export);
}

test "end-to-end compile angle function" {
    const source =
        \\export fn double(a: angle) -> angle {
        \\    return a + a
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("double");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    var gen = WasmGen.init(alloc);
    defer gen.deinit();

    const bytes = try gen.generate(module);

    // 最低限度检查: 魔数 + 至少包含一个段
    try testing.expect(bytes.len > 8);
    try testing.expectEqual(@as(u8, 0x6D), bytes[3]); // 'm' in \0asm
}

// ── 新特性测试 ──

test "parse enum definition" {
    const source =
        \\enum Color {
        \\    Red = 0,
        \\    Green = 1,
        \\    Blue = 2,
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test_enum");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expectEqualStrings("Color", module.stmts[0].enum_def.name);
    try testing.expectEqual(@as(usize, 3), module.stmts[0].enum_def.variants.len);
}

test "parse interface definition" {
    const source =
        \\interface Printable {
        \\    fn print() -> void
        \\    fn to_string() -> cell
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test_iface");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expectEqualStrings("Printable", module.stmts[0].interface_def.name);
    try testing.expectEqual(@as(usize, 2), module.stmts[0].interface_def.methods.len);
}

test "parse impl block" {
    const source =
        \\impl Pixel for Printable {
        \\    fn print() -> void {
        \\        return
        \\    }
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test_impl");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expectEqualStrings("Pixel", module.stmts[0].impl_block.type_name);
}

test "parse type alias" {
    const source =
        \\type Int = cell
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test_type");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expectEqualStrings("Int", module.stmts[0].type_alias.name);
}

test "parse match expression" {
    const source =
        \\export fn classify(x: cell) -> cell {
        \\    return match x {
        \\        0 => 100,
        \\        1 => 200,
        \\    }
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("test_match");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    try testing.expectEqual(@as(usize, 1), module.stmts.len);
    try testing.expect(module.stmts[0].export_fn.body.len > 0);
}

test "advanced module with enum and impl" {
    const source =
        \\enum Color {
        \\    Red,
        \\    Green,
        \\    Blue,
        \\}
        \\type Int = cell
        \\export fn main() -> cell {
        \\    let x: cell = 42
        \\    return x
        \\}
    ;
    var alloc = testing.allocator;

    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    var parser = try Parser.init(alloc, &lexer);
    const module = try parser.parseModule("advanced");
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    var gen = WasmGen.init(alloc);
    defer gen.deinit();

    const bytes = try gen.generate(module);

    // 验证 WASM 魔数
    try testing.expectEqual(@as(u8, 0x00), bytes[0]);
    try testing.expectEqual(@as(u8, 0x61), bytes[1]);
    try testing.expectEqual(@as(u8, 0x73), bytes[2]);
    try testing.expectEqual(@as(u8, 0x6D), bytes[3]);

    // 3 个顶层语句: enum + type + export fn
    try testing.expectEqual(@as(usize, 3), module.stmts.len);
}
