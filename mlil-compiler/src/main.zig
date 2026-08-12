// main.zig — mlil 编译器主入口
// mlil = 毛利兰编程语言 = 表面/界面语言 → 编译为 WASM
//
// 用法: mlil <source.mlil> [-o output.wasm]

const std = @import("std");
const Lexer = @import("lexer.zig").Lexer;
const Parser = @import("parser.zig").Parser;
const WasmGen = @import("wasmgen.zig").WasmGen;
const Ast = @import("ast.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);

    if (args.len < 2) {
        std.debug.print("mlil — 毛利兰编程语言编译器\n", .{});
        std.debug.print("用法: mlil <source.mlil> [-o output.wasm]\n", .{});
        std.debug.print("\n", .{});
        std.debug.print("选项:\n", .{});
        std.debug.print("  -o <path>  输出 WASM 文件路径 (默认: 输入名.wasm)\n", .{});
        return;
    }

    const input_path = args[1];
    var output_path: ?[]const u8 = null;
    var output_path_allocated: bool = false;

    // 解析参数
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        if (std.mem.eql(u8, args[i], "-o")) {
            i += 1;
            if (i >= args.len) {
                std.debug.print("错误: -o 需要路径参数\n", .{});
                return;
            }
            output_path = args[i];
        }
    }

    if (output_path == null) {
        // 默认输出路径: 输入文件名替换扩展名为 .wasm
        const stem = std.fs.path.stem(input_path);
        output_path = try std.fmt.allocPrint(alloc, "{s}.wasm", .{stem});
        output_path_allocated = true;
    }
    defer {
        if (output_path_allocated) {
            if (output_path) |op| alloc.free(op);
        }
    }

    // 读取源文件
    const source = try std.fs.cwd().readFileAlloc(alloc, input_path, 1024 * 1024);
    defer alloc.free(source);

    // 词法分析
    var lexer = try Lexer.init(alloc, source);
    defer lexer.deinit();

    // 语法分析
    var parser = try Parser.init(alloc, &lexer);
    const module_name = std.fs.path.stem(input_path);
    const module = try parser.parseModule(module_name);
    defer {
        module.deinit();
        alloc.destroy(module);
    }

    // 打印解析结果摘要
    std.debug.print("mlil: 解析完成 — 模块 '{s}', {} 个顶层语句\n", .{
        module.name, module.stmts.len,
    });

    // WASM 代码生成
    var gen = WasmGen.init(alloc);
    defer gen.deinit();

    const wasm_bytes = try gen.generate(module);

    // 输出文件
    const out_file = try std.fs.cwd().createFile(output_path.?, .{});
    defer out_file.close();
    try out_file.writeAll(wasm_bytes);
    std.debug.print("mlil: 输出 WASM 到 '{s}' ({} 字节)\n", .{ output_path.?, wasm_bytes.len });

    // 验证 WASM 魔数
    if (wasm_bytes.len >= 4 and 
        wasm_bytes[0] == 0x00 and wasm_bytes[1] == 0x61 and
        wasm_bytes[2] == 0x73 and wasm_bytes[3] == 0x6D) {
        std.debug.print("mlil: WASM 魔数验证通过\n", .{});
    } else {
        std.debug.print("mlil: 警告 — WASM 魔数异常\n", .{});
    }
}
