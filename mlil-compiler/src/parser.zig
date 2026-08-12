// parser.zig — mlil 语法分析器 (递归下降)
// 将 token 流转换为 AST

const std = @import("std");
const Lexer = @import("lexer.zig").Lexer;
const TokenMod = @import("token.zig");
const Token = TokenMod.Token;
const TokenType = TokenMod.TokenType;
const Ast = @import("ast.zig");

const Allocator = std.mem.Allocator;
pub const Error = error{
    OutOfMemory,
    UnexpectedToken,
    InvalidCharacter,
    Overflow,
};

pub const Parser = struct {
    lexer: *Lexer,
    current: Token,
    alloc: Allocator,

    pub fn init(alloc: Allocator, lexer: *Lexer) error{OutOfMemory}!Parser {
        var p = Parser{
            .lexer = lexer,
            .current = undefined,
            .alloc = alloc,
        };
        p.current = try lexer.next();
        return p;
    }

    /// 前进到下一个 token
    fn advance(self: *Parser) !void {
        self.current = try self.lexer.next();
    }

    /// 消费特定类型的 token
    fn expect(self: *Parser, typ: TokenType) !void {
        if (self.current.typ != typ) {
            std.log.err("mlil parse error: expected {s}, got {s} at line {}", .{
                @tagName(typ), @tagName(self.current.typ), self.current.line,
            });
            return error.UnexpectedToken;
        }
        try self.advance();
    }

    /// 解析整个模块
    pub fn parseModule(self: *Parser, name: []const u8) Error!*Ast.Module {
        const module = try self.alloc.create(Ast.Module);
        module.* = Ast.Module.init(self.alloc, name);

        // 跳过开头的换行
        while (self.current.typ == .newline) {
            try self.advance();
        }

        // 跳过 import (简化: 不保存, 只跳过)
        while (self.current.typ == .kw_import) {
            try self.advance();
            if (self.current.typ == .string_literal) {
                try self.advance();
            }
            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        // 解析顶层语句
        while (self.current.typ != .eof) {
            const stmt = try self.parseTopLevel();
            try module.append(stmt);

            // 跳过换行
            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        return module;
    }

    /// 解析顶层语句
    fn parseTopLevel(self: *Parser) Error!*Ast.Stmt {
        switch (self.current.typ) {
            .kw_surface, .kw_struct => return self.parseSurface(),
            .kw_scene => return self.parseScene(),
            .kw_export => return self.parseExportFn(),
            .kw_fn => return self.parseFn(),
            .kw_enum => return self.parseEnum(),
            .kw_interface => return self.parseInterface(),
            .kw_impl => return self.parseImpl(),
            .kw_type => return self.parseTypeAlias(),
            else => {
                std.log.err("mlil parse error: unexpected top-level token {s} at line {}", .{
                    @tagName(self.current.typ), self.current.line,
                });
                return error.UnexpectedToken;
            },
        }
    }

    /// 解析 surface 定义
    fn parseSurface(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_surface);
        const name = try self.expectToken(.identifier);
        try self.expect(.lbrace);

        var fields = std.ArrayList(Ast.Field).init(self.alloc);
        defer fields.deinit();

        // 跳过开始的换行
        while (self.current.typ == .newline) {
            try self.advance();
        }

        while (self.current.typ != .rbrace) {
            const fname = try self.expectToken(.identifier);
            try self.expect(.op_colon);
            const ftyp = try self.parseType();
            try fields.append(.{ .name = fname, .typ = ftyp });

            if (self.current.typ == .op_comma) {
                try self.advance();
            }

            // 跳过字段后的换行
            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .surface_def = .{
            .name = name,
            .fields = try fields.toOwnedSlice(),
        } };
        return stmt;
    }

    /// 解析 scene 定义
    fn parseScene(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_scene);
        const name = try self.expectToken(.identifier);
        try self.expect(.lbrace);

        var entries = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer entries.deinit();

        while (self.current.typ != .rbrace) {
            const stmt = try self.parseStatement();
            try entries.append(stmt);
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .scene_def = .{
            .name = name,
            .entries = try entries.toOwnedSlice(),
        } };
        return stmt;
    }

    /// 解析 export 函数
    fn parseExportFn(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_export);
        try self.expect(.kw_fn);
        const name = try self.expectToken(.identifier);
        try self.expect(.lparen);

        var params = std.ArrayList(Ast.Param).init(self.alloc);
        defer params.deinit();

        while (self.current.typ != .rparen) {
            const pname = try self.expectToken(.identifier);
            try self.expect(.op_colon);
            const ptyp = try self.parseType();
            try params.append(.{ .name = pname, .typ = ptyp });

            if (self.current.typ == .op_comma) {
                try self.advance();
            }
        }

        try self.expect(.rparen);

        var return_type: Ast.Type = .void;
        if (self.current.typ == .op_arrow) {
            try self.advance();
            return_type = try self.parseType();
        }

        try self.expect(.lbrace);

        var body = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer body.deinit();

        while (self.current.typ != .rbrace) {
            // 跳过语句间的换行
            while (self.current.typ == .newline) {
                try self.advance();
            }
            if (self.current.typ == .rbrace) break;
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .export_fn = .{
            .name = name,
            .params = try params.toOwnedSlice(),
            .return_type = return_type,
            .body = try body.toOwnedSlice(),
        } };
        return stmt;
    }

    /// 解析普通函数
    fn parseFn(self: *Parser) Error!*Ast.Stmt {
        // 复用 export fn 逻辑 (不加 export)
        try self.expect(.kw_fn);
        const name = try self.expectToken(.identifier);
        try self.expect(.lparen);

        var params = std.ArrayList(Ast.Param).init(self.alloc);
        defer params.deinit();

        while (self.current.typ != .rparen) {
            const pname = try self.expectToken(.identifier);
            try self.expect(.op_colon);
            const ptyp = try self.parseType();
            try params.append(.{ .name = pname, .typ = ptyp });

            if (self.current.typ == .op_comma) {
                try self.advance();
            }
        }

        try self.expect(.rparen);

        var return_type: Ast.Type = .void;
        if (self.current.typ == .op_arrow) {
            try self.advance();
            return_type = try self.parseType();
        }

        try self.expect(.lbrace);
        var body = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer body.deinit();

        while (self.current.typ != .rbrace) {
            // 跳过语句间的换行
            while (self.current.typ == .newline) {
                try self.advance();
            }
            if (self.current.typ == .rbrace) break;
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }

        try self.expect(.rbrace);

        // 普通函数视为不导出
        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .export_fn = .{
            .name = name,
            .params = try params.toOwnedSlice(),
            .return_type = return_type,
            .body = try body.toOwnedSlice(),
        } };
        return stmt;
    }

    /// 解析类型
    fn parseType(self: *Parser) Error!Ast.Type {
        const name = try self.expectToken(.identifier);
        if (std.mem.eql(u8, name, "cell")) return .cell;
        if (std.mem.eql(u8, name, "coord")) return .coord;
        if (std.mem.eql(u8, name, "symbol")) return .symbol;
        if (std.mem.eql(u8, name, "angle")) return .angle;
        if (std.mem.eql(u8, name, "stride")) return .stride;
        if (std.mem.eql(u8, name, "void")) return .void;
        if (std.mem.eql(u8, name, "enum")) return .enumerated;
        if (std.mem.eql(u8, name, "interface")) return .interfaced;
        // 自定义类型（包括枚举和接口的引用）
        return .surface;
    }

    /// 解析语句
    fn parseStatement(self: *Parser) Error!*Ast.Stmt {
        // 跳过语句间的换行
        while (self.current.typ == .newline) {
            try self.advance();
        }

        switch (self.current.typ) {
            .kw_let => return self.parseLet(),
            .kw_return => return self.parseReturn(),
            .kw_if => return self.parseIf(),
            .kw_for => return self.parseFor(),
            else => {
                // 可能是赋值或表达式语句
                const expr = try self.parseExpression();
                if (self.current.typ == .op_assign) {
                    try self.advance();
                    const value = try self.parseExpression();
                    const stmt = try self.alloc.create(Ast.Stmt);
                    stmt.* = .{ .assignment = .{ .target = expr, .value = value } };
                    return stmt;
                }
                const stmt = try self.alloc.create(Ast.Stmt);
                stmt.* = .{ .expr_stmt = .{ .expr = expr } };
                return stmt;
            },
        }
    }

    /// 解析 let 绑定 (支持类型注解: let x: type = expr)
    fn parseLet(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_let);

        var is_mut = false;
        if (self.current.typ == .kw_mut) {
            is_mut = true;
            try self.advance();
        }

        const name = try self.expectToken(.identifier);

        // 可选的类型注解: : type
        if (self.current.typ == .op_colon) {
            try self.advance();
            _ = try self.parseType();
        }

        try self.expect(.op_assign);
        const init_expr = try self.parseExpression();

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .let_binding = .{
            .name = name,
            .is_mut = is_mut,
            .initializer = init_expr,
        } };
        return stmt;
    }

    /// 解析 return
    fn parseReturn(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_return);
        var return_expr: ?*Ast.Expr = null;

        // 只在下一个 token 不是 } 或 newline 时才解析返回值
        if (self.current.typ != .rbrace and self.current.typ != .newline and self.current.typ != .eof) {
            const expr = try self.parseExpression();
            return_expr = expr;
        }

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .return_stmt = .{ .value = return_expr } };
        return stmt;
    }

    /// 解析 if
    fn parseIf(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_if);
        const cond = try self.parseExpression();

        try self.expect(.lbrace);
        var then_body = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer then_body.deinit();

        while (self.current.typ != .rbrace) {
            const stmt = try self.parseStatement();
            try then_body.append(stmt);
        }
        try self.expect(.rbrace);

        var else_body: ?[]*Ast.Stmt = null;
        if (self.current.typ == .kw_else) {
            try self.advance();
            try self.expect(.lbrace);
            var else_list = std.ArrayList(*Ast.Stmt).init(self.alloc);
            defer else_list.deinit();

            while (self.current.typ != .rbrace) {
                const stmt = try self.parseStatement();
                try else_list.append(stmt);
            }
            try self.expect(.rbrace);
            else_body = try else_list.toOwnedSlice();
        }

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .if_stmt = .{
            .condition = cond,
            .then_body = try then_body.toOwnedSlice(),
            .else_body = else_body,
        } };
        return stmt;
    }

    /// 解析 for 循环
    fn parseFor(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_for);
        const var_name = try self.expectToken(.identifier);
        try self.expect(.kw_in);
        const iterable = try self.parseExpression();

        try self.expect(.lbrace);
        var body = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer body.deinit();

        while (self.current.typ != .rbrace) {
            const stmt = try self.parseStatement();
            try body.append(stmt);
        }
        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .for_stmt = .{
            .var_name = var_name,
            .iterable = iterable,
            .body = try body.toOwnedSlice(),
        } };
        return stmt;
    }

    // ── 表达式解析 (优先级 climbing) ──

    fn parseExpression(self: *Parser) Error!*Ast.Expr {
        return self.parseOr();
    }

    fn parseOr(self: *Parser) Error!*Ast.Expr {
        var lhs = try self.parseAnd();
        while (self.current.typ == .op_or) {
            try self.advance();
            const rhs = try self.parseAnd();
            const bin = try self.alloc.create(Ast.Expr);
            bin.* = .{ .binary = .{ .op = .@"or", .lhs = lhs, .rhs = rhs } };
            lhs = bin;
        }
        return lhs;
    }

    fn parseAnd(self: *Parser) Error!*Ast.Expr {
        var lhs = try self.parseComparison();
        while (self.current.typ == .op_and) {
            try self.advance();
            const rhs = try self.parseComparison();
            const bin = try self.alloc.create(Ast.Expr);
            bin.* = .{ .binary = .{ .op = .@"and", .lhs = lhs, .rhs = rhs } };
            lhs = bin;
        }
        return lhs;
    }

    fn parseComparison(self: *Parser) Error!*Ast.Expr {
        var lhs = try self.parseAddition();
        while (true) {
            const op: ?Ast.BinaryOp = switch (self.current.typ) {
                .op_eq => .eq,
                .op_ne => .ne,
                .op_lt => .lt,
                .op_gt => .gt,
                .op_le => .le,
                .op_ge => .ge,
                else => null,
            };
            if (op == null) break;
            try self.advance();
            const rhs = try self.parseAddition();
            const bin = try self.alloc.create(Ast.Expr);
            bin.* = .{ .binary = .{ .op = op.?, .lhs = lhs, .rhs = rhs } };
            lhs = bin;
        }
        return lhs;
    }

    fn parseAddition(self: *Parser) Error!*Ast.Expr {
        var lhs = try self.parseMultiplication();
        while (self.current.typ == .op_plus or self.current.typ == .op_minus) {
            const op: Ast.BinaryOp = if (self.current.typ == .op_plus) .add else .sub;
            try self.advance();
            const rhs = try self.parseMultiplication();
            const bin = try self.alloc.create(Ast.Expr);
            bin.* = .{ .binary = .{ .op = op, .lhs = lhs, .rhs = rhs } };
            lhs = bin;
        }
        return lhs;
    }

    fn parseMultiplication(self: *Parser) Error!*Ast.Expr {
        var lhs = try self.parseUnary();
        while (self.current.typ == .op_star or self.current.typ == .op_slash or self.current.typ == .op_percent) {
            const op: Ast.BinaryOp = switch (self.current.typ) {
                .op_star => .mul,
                .op_slash => .div,
                .op_percent => .mod,
                else => unreachable,
            };
            try self.advance();
            const rhs = try self.parseUnary();
            const bin = try self.alloc.create(Ast.Expr);
            bin.* = .{ .binary = .{ .op = op, .lhs = lhs, .rhs = rhs } };
            lhs = bin;
        }
        return lhs;
    }

    fn parseUnary(self: *Parser) Error!*Ast.Expr {
        if (self.current.typ == .op_minus) {
            try self.advance();
            const operand = try self.parsePrimary();
            const un = try self.alloc.create(Ast.Expr);
            un.* = .{ .unary = .{ .op = .neg, .operand = operand } };
            return un;
        }
        if (self.current.typ == .op_not) {
            try self.advance();
            const operand = try self.parsePrimary();
            const un = try self.alloc.create(Ast.Expr);
            un.* = .{ .unary = .{ .op = .not, .operand = operand } };
            return un;
        }
        return try self.parsePrimary();
    }

    fn parsePrimary(self: *Parser) Error!*Ast.Expr {
        switch (self.current.typ) {
            .int_literal => {
                const val = try std.fmt.parseInt(u64, self.current.lexeme, 10);
                try self.advance();
                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .int_literal = .{ .value = val, .is_stride = false } };
                return expr;
            },
            .angle_literal => {
                // 去掉 ° 后缀 (UTF-8: 2 字节)
                const text = self.current.lexeme[0 .. self.current.lexeme.len - 2];
                const val = try std.fmt.parseInt(u16, text, 10);
                try self.advance();
                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .angle_literal = .{ .value = val } };
                return expr;
            },
            .sym_literal => {
                const val = TokenMod.symbolNameToValue(self.current.lexeme).?;
                try self.advance();
                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .symbol_literal = .{ .value = val } };
                return expr;
            },
            .string_literal => {
                const val = self.current.lexeme;
                try self.advance();
                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .string_literal = .{ .value = val } };
                return expr;
            },
            .identifier => {
                const name = self.current.lexeme;
                try self.advance();

                // 检查是否是函数调用
                if (self.current.typ == .lparen) {
                    try self.advance();
                    var args = std.ArrayList(*Ast.Expr).init(self.alloc);
                    defer args.deinit();

                    while (self.current.typ != .rparen) {
                        const arg = try self.parseExpression();
                        try args.append(arg);

                        if (self.current.typ == .op_comma) {
                            try self.advance();
                        }
                    }
                    try self.expect(.rparen);

                    const expr = try self.alloc.create(Ast.Expr);
                    expr.* = .{ .call = .{
                        .callee = name,
                        .args = try args.toOwnedSlice(),
                    } };
                    return expr;
                }

                // 检查是否是坐标字面量 [x,y,z]
                if (self.current.typ == .lbracket) {
                    try self.advance();
                    // 简化: 期望 [number, number, number]
                    const x = try self.expectToken(.int_literal);
                    try self.expect(.op_comma);
                    const y = try self.expectToken(.int_literal);
                    try self.expect(.op_comma);
                    const z = try self.expectToken(.int_literal);
                    try self.expect(.rbracket);

                    const expr = try self.alloc.create(Ast.Expr);
                    expr.* = .{ .coord_literal = .{
                        .x = @intCast(try std.fmt.parseInt(u8, x, 10)),
                        .y = @intCast(try std.fmt.parseInt(u8, y, 10)),
                        .z = @intCast(try std.fmt.parseInt(u8, z, 10)),
                    } };
                    return expr;
                }

                // 检查是否是字段访问 (obj.field)
                if (self.current.typ == .op_dot) {
                    try self.advance();
                    const field = try self.expectToken(.identifier);
                    const obj = try self.alloc.create(Ast.Expr);
                    obj.* = .{ .ident = .{ .name = name } };
                    const expr = try self.alloc.create(Ast.Expr);
                    expr.* = .{ .field_access = .{ .object = obj, .field = field } };
                    return expr;
                }

                // 普通标识符
                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .ident = .{ .name = name } };
                return expr;
            },
            .kw_match => {
                try self.advance();
                const target = try self.parseExpression();
                try self.expect(.lbrace);

                var arms = std.ArrayList(*Ast.MatchArm).init(self.alloc);
                defer arms.deinit();

                while (self.current.typ == .newline) {
                    try self.advance();
                }

                while (self.current.typ != .rbrace and self.current.typ != .eof) {
                    const pattern = try self.parseExpression();
                    try self.expect(.op_fat_arrow);
                    const body = try self.parseExpression();

                    const arm = try self.alloc.create(Ast.MatchArm);
                    arm.* = .{ .pattern = pattern, .body = body };
                    try arms.append(arm);

                    if (self.current.typ == .op_comma) {
                        try self.advance();
                    }
                    while (self.current.typ == .newline) {
                        try self.advance();
                    }
                }

                try self.expect(.rbrace);

                const expr = try self.alloc.create(Ast.Expr);
                expr.* = .{ .match_expr = .{
                    .target = target,
                    .arms = try arms.toOwnedSlice(),
                } };
                return expr;
            },
            .lparen => {
                try self.advance();
                const expr = try self.parseExpression();
                try self.expect(.rparen);
                return expr;
            },
            else => {
                std.log.err("mlil parse error: unexpected expression token {s} at line {}", .{
                    @tagName(self.current.typ), self.current.line,
                });
                return error.UnexpectedToken;
            },
        }
    }

    /// 解析枚举定义
    fn parseEnum(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_enum);
        const name = try self.expectToken(.identifier);

        // 泛型参数（简化：跳过 <...> ）
        if (self.current.typ == .op_lt) {
            try self.advance();
            while (self.current.typ != .op_gt) {
                try self.advance();
            }
            try self.expect(.op_gt);
        }

        try self.expect(.lbrace);

        var variants = std.ArrayList(Ast.Variant).init(self.alloc);
        defer variants.deinit();

        while (self.current.typ == .newline) {
            try self.advance();
        }

        while (self.current.typ != .rbrace) {
            const vname = try self.expectToken(.identifier);
            var value: ?i64 = null;
            if (self.current.typ == .op_assign) {
                try self.advance();
                const vstr = try self.expectToken(.int_literal);
                value = try std.fmt.parseInt(i64, vstr, 10);
            }
            try variants.append(.{ .name = vname, .value = value });

            if (self.current.typ == .op_comma) {
                try self.advance();
            }
            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .enum_def = .{
            .name = name,
            .variants = try variants.toOwnedSlice(),
            .generics = &[_]Ast.GenericParam{},
        } };
        return stmt;
    }

    /// 解析接口定义
    fn parseInterface(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_interface);
        const name = try self.expectToken(.identifier);
        try self.expect(.lbrace);

        var methods = std.ArrayList(Ast.InterfaceMethod).init(self.alloc);
        defer methods.deinit();

        while (self.current.typ == .newline) {
            try self.advance();
        }

        while (self.current.typ != .rbrace) {
            // interface 方法: fn name(params) -> ret
            if (self.current.typ == .kw_fn) {
                try self.advance(); // 消耗 fn
            }
            const mname = try self.expectToken(.identifier);
            try self.expect(.lparen);

            var params = std.ArrayList(Ast.Param).init(self.alloc);
            defer params.deinit();

            while (self.current.typ != .rparen) {
                const pname = try self.expectToken(.identifier);
                try self.expect(.op_colon);
                const ptyp = try self.parseType();
                try params.append(.{ .name = pname, .typ = ptyp });
                if (self.current.typ == .op_comma) {
                    try self.advance();
                }
            }
            try self.expect(.rparen);

            var ret_type: Ast.Type = .void;
            if (self.current.typ == .op_arrow) {
                try self.advance();
                ret_type = try self.parseType();
            }

            try methods.append(.{
                .name = mname,
                .params = try params.toOwnedSlice(),
                .return_type = ret_type,
            });

            // 接口方法之间可能有逗号
            if (self.current.typ == .op_comma) {
                try self.advance();
            }

            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .interface_def = .{
            .name = name,
            .methods = try methods.toOwnedSlice(),
            .generics = &[_]Ast.GenericParam{},
        } };
        return stmt;
    }

    /// 解析 impl 块
    fn parseImpl(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_impl);
        const type_name = try self.expectToken(.identifier);

        var interface_name: ?[]const u8 = null;
        if (self.current.typ == .kw_for) {
            try self.advance();
            interface_name = try self.expectToken(.identifier);
        }

        try self.expect(.lbrace);

        var methods = std.ArrayList(*Ast.Stmt).init(self.alloc);
        defer methods.deinit();

        while (self.current.typ == .newline) {
            try self.advance();
        }

        while (self.current.typ != .rbrace) {
            // impl 块内是函数定义 (以 fn 或 export 开头)
            const method = try self.parseImplMethod();
            try methods.append(method);
            while (self.current.typ == .newline) {
                try self.advance();
            }
        }

        try self.expect(.rbrace);

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .impl_block = .{
            .type_name = type_name,
            .interface_name = interface_name,
            .methods = try methods.toOwnedSlice(),
        } };
        return stmt;
    }

    /// 解析 impl 块中的方法（fn 或 export fn）
    fn parseImplMethod(self: *Parser) Error!*Ast.Stmt {
        // impl 块内可能以 export 或 fn 开头
        if (self.current.typ == .kw_export) {
            return self.parseExportFn();
        }
        if (self.current.typ == .kw_fn) {
            return self.parseFn();
        }
        // 默认尝试 fn
        return self.parseFn();
    }

    /// 解析类型别名
    fn parseTypeAlias(self: *Parser) Error!*Ast.Stmt {
        try self.expect(.kw_type);
        const name = try self.expectToken(.identifier);
        try self.expect(.op_assign);
        const target = try self.parseType();

        const stmt = try self.alloc.create(Ast.Stmt);
        stmt.* = .{ .type_alias = .{
            .name = name,
            .target_type = target,
        } };
        return stmt;
    }

    /// 期望特定 token 并返回其 lexeme
    fn expectToken(self: *Parser, typ: TokenType) Error![]const u8 {
        if (self.current.typ != typ) {
            return error.UnexpectedToken;
        }
        const lexeme = self.current.lexeme;
        try self.advance();
        return lexeme;
    }
};
