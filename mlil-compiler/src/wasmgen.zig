// wasmgen.zig — mlil → WASM 代码生成器
// 将 mlil AST 编译为 WebAssembly 二进制 (.wasm)
//
// WASM 模块结构:
//   魔数 + 版本
//   类型段 (Type Section)   — 函数签名
//   函数段 (Function Section)— 函数索引
//   导出段 (Export Section) — 导出给宿主
//   代码段 (Code Section)   — 函数体字节码

const std = @import("std");
const Ast = @import("ast.zig");

const Allocator = std.mem.Allocator;

/// WASM 二进制生成器
pub const WasmGen = struct {
    alloc: Allocator,
    bytes: std.ArrayList(u8),

    pub fn init(alloc: Allocator) WasmGen {
        return .{
            .alloc = alloc,
            .bytes = std.ArrayList(u8).init(alloc),
        };
    }

    pub fn deinit(self: *WasmGen) void {
        self.bytes.deinit();
    }

    /// 生成完整的 WASM 模块
    pub fn generate(self: *WasmGen, module: *const Ast.Module) ![]const u8 {
        // ── 1. 魔数 + 版本 ──
        try self.bytes.appendSlice(&[_]u8{ 0x00, 0x61, 0x73, 0x6D }); // \0asm
        try self.bytes.appendSlice(&[_]u8{ 0x01, 0x00, 0x00, 0x00 }); // 版本 1

        // ── 2. 收集所有函数和导出 ──
        var functions = std.ArrayList(FuncInfo).init(self.alloc);
        defer functions.deinit();
        var exports = std.ArrayList(ExportInfo).init(self.alloc);
        defer exports.deinit();

        for (module.stmts) |stmt| {
            switch (stmt.*) {
                .export_fn => |fn_def| {
                    const idx = @as(u32, @intCast(functions.items.len));
                    try functions.append(.{
                        .name = fn_def.name,
                        .params = fn_def.params,
                        .return_type = fn_def.return_type,
                        .body = fn_def.body,
                    });
                    try exports.append(.{
                        .name = fn_def.name,
                        .func_idx = idx,
                    });
                },
                .surface_def => {
                    // surface 定义暂不生成 WASM 函数
                },
                .scene_def => {
                    // scene 定义: 生成一个初始化函数
                    const idx = @as(u32, @intCast(functions.items.len));
                    try functions.append(.{
                        .name = module.name,
                        .params = &[_]Ast.Param{},
                        .return_type = .void,
                        .body = &[_]*Ast.Stmt{}, // 简化: 空 body
                    });
                    try exports.append(.{
                        .name = module.name,
                        .func_idx = idx,
                    });
                },
                else => {},
            }
        }

        // ── 3. 生成各段 ──
        try self.emitTypeSection(functions.items);
        try self.emitFunctionSection(functions.items);
        try self.emitExportSection(exports.items);
        try self.emitCodeSection(functions.items);

        return self.bytes.items;
    }

    /// 类型段 (section id = 1)
    fn emitTypeSection(self: *WasmGen, functions: []FuncInfo) !void {
        var seg = std.ArrayList(u8).init(self.alloc);
        defer seg.deinit();

        try emitLEB128(&seg, functions.len);
        for (functions) |fn_info| {
            try seg.append(0x60); // 函数类型标记
            try emitLEB128(&seg, fn_info.params.len);
            for (fn_info.params) |param| {
                try seg.append(try typeToWasm(param.typ));
            }
            if (fn_info.return_type == .void) {
                try seg.append(0x00);
            } else {
                try seg.append(0x01);
                try seg.append(try typeToWasm(fn_info.return_type));
            }
        }

        try self.emitSection(1, seg.items);
    }

    /// 函数段 (section id = 3)
    fn emitFunctionSection(self: *WasmGen, functions: []FuncInfo) !void {
        var seg = std.ArrayList(u8).init(self.alloc);
        defer seg.deinit();

        try emitLEB128(&seg, functions.len);
        for (functions, 0..) |_, i| {
            try emitLEB128(&seg, i);
        }

        try self.emitSection(3, seg.items);
    }

    /// 导出段 (section id = 7)
    fn emitExportSection(self: *WasmGen, exports: []ExportInfo) !void {
        var seg = std.ArrayList(u8).init(self.alloc);
        defer seg.deinit();

        try emitLEB128(&seg, exports.len);
        for (exports) |exp| {
            try emitLEB128(&seg, exp.name.len);
            try seg.appendSlice(exp.name);
            try seg.append(0x00); // 导出类型: 函数
            try emitLEB128(&seg, exp.func_idx);
        }

        try self.emitSection(7, seg.items);
    }

    /// 代码段 (section id = 10)
    fn emitCodeSection(self: *WasmGen, functions: []FuncInfo) !void {
        var seg = std.ArrayList(u8).init(self.alloc);
        defer seg.deinit();

        try emitLEB128(&seg, functions.len);
        for (functions) |fn_info| {
            var body = std.ArrayList(u8).init(self.alloc);
            defer body.deinit();

            try self.emitFunctionBody(&body, fn_info);

            try emitLEB128(&seg, body.items.len);
            try seg.appendSlice(body.items);
        }

        try self.emitSection(10, seg.items);
    }

    /// 生成函数体
    fn emitFunctionBody(self: *WasmGen, body: *std.ArrayList(u8), fn_info: FuncInfo) !void {
        // 局部变量声明 (简化: 空列表)
        try body.append(0x00);

        // 生成每条语句
        for (fn_info.body) |stmt| {
            try self.emitStatement(body, stmt);
        }

        try body.append(0x0B); // end opcode
    }

    /// 生成语句的 WASM 字节码
    fn emitStatement(self: *WasmGen, buf: *std.ArrayList(u8), stmt: *Ast.Stmt) !void {
        switch (stmt.*) {
            .return_stmt => |ret| {
                if (ret.value) |val| {
                    try self.emitExpression(buf, val);
                }
                try buf.append(0x0F); // return
            },
            .expr_stmt => |es| {
                try self.emitExpression(buf, es.expr);
                try buf.append(0x1A); // drop
            },
            .let_binding => |lb| {
                if (lb.initializer) |initializer| {
                    try self.emitExpression(buf, initializer);
                }
            },
            .if_stmt => |if_stmt| {
                try self.emitExpression(buf, if_stmt.condition);
                try buf.append(0x04); // if
                try buf.append(0x7F); // block type: i32

                for (if_stmt.then_body) |s| {
                    try self.emitStatement(buf, s);
                }

                if (if_stmt.else_body) |else_body| {
                    try buf.append(0x05); // else
                    for (else_body) |s| {
                        try self.emitStatement(buf, s);
                    }
                }

                try buf.append(0x0B); // end
            },
            else => {},
        }
    }

    /// 生成表达式的 WASM 字节码
    fn emitExpression(self: *WasmGen, buf: *std.ArrayList(u8), expr: *Ast.Expr) !void {
        switch (expr.*) {
            .int_literal => |lit| {
                try buf.append(0x41); // i32.const
                try emitLEB128(buf, lit.value);
            },
            .angle_literal => |ang| {
                try buf.append(0x41); // i32.const
                try emitLEB128(buf, ang.value);
            },
            .symbol_literal => |sym| {
                try buf.append(0x41); // i32.const
                try emitLEB128(buf, sym.value);
            },
            .coord_literal => |coord| {
                const packed_val = (@as(u32, coord.x) << 16) | (@as(u32, coord.y) << 8) | coord.z;
                try buf.append(0x41); // i32.const
                try emitLEB128(buf, packed_val);
            },
            .binary => |bin| {
                try self.emitExpression(buf, bin.lhs);
                try self.emitExpression(buf, bin.rhs);
                if (bin.op.toWasmOpcode()) |op| {
                    try buf.append(op);
                }
            },
            .unary => |unary| {
                try self.emitExpression(buf, unary.operand);
                try buf.append(unary.op.toWasmOpcode());
            },
            .call => |call| {
                for (call.args) |arg| {
                    try self.emitExpression(buf, arg);
                }
                try buf.append(0x10); // call
                try buf.append(0x00); // 函数索引 (占位)
            },
            .ident => |_| {
                try buf.append(0x20); // local.get
                try buf.append(0x00); // 局部变量索引 0 (占位)
            },
            .field_access => |fa| {
                try self.emitExpression(buf, fa.object);
            },
            .string_literal => {
                // 暂不实现
            },
            .index_access => |ia| {
                try self.emitExpression(buf, ia.object);
                try self.emitExpression(buf, ia.index);
            },
        }
    }

    /// 生成段头并追加到输出
    fn emitSection(self: *WasmGen, id: u8, data: []const u8) !void {
        try self.bytes.append(id);
        try emitLEB128(&self.bytes, data.len);
        try self.bytes.appendSlice(data);
    }
};

/// 函数信息 (编译时)
const FuncInfo = struct {
    name: []const u8,
    params: []Ast.Param,
    return_type: Ast.Type,
    body: []*Ast.Stmt,
};

/// 导出信息
const ExportInfo = struct {
    name: []const u8,
    func_idx: u32,
};

/// 将 mlil 类型映射为 WASM 类型
fn typeToWasm(typ: Ast.Type) !u8 {
    return switch (typ) {
        .void => 0x40,
        .cell, .coord, .symbol, .angle, .stride => 0x7F, // i32
        .surface, .scene => 0x7F, // i32 (引用)
        .function => 0x7F,
    };
}

/// LEB128 编码 — 追加到缓冲区
fn emitLEB128(buf: *std.ArrayList(u8), value: anytype) !void {
    var v = @as(u32, @intCast(value));
    while (true) {
        var byte: u8 = @intCast(v & 0x7F);
        v >>= 7;
        if (v != 0) byte |= 0x80;
        try buf.append(byte);
        if (v == 0) break;
    }
}
