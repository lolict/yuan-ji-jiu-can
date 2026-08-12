// ast.zig — mlil 抽象语法树定义
// mlil 的 AST 是"表面描述"的树形编码

const std = @import("std");
const Allocator = std.mem.Allocator;

/// mlil 支持的类型
/// 每种类型都映射到满全法的某个"表面属性"
pub const Type = enum(u8) {
    void,           // 无返回
    cell,           // 64 位单元格
    coord,          // 坐标
    symbol,         // 64 符号
    angle,          // 角度
    stride,         // 阶梯进制数
    surface,        // 表面 (结构体实例)
    scene,          // 场景
    function,       // 函数类型
    enumerated,     // 枚举类型
    interfaced,     // 接口
    generic,        // 泛型参数

    pub fn name(self: Type) []const u8 {
        return switch (self) {
            .void => "void",
            .cell => "cell",
            .coord => "coord",
            .symbol => "symbol",
            .angle => "angle",
            .stride => "stride",
            .surface => "surface",
            .scene => "scene",
            .function => "fn",
            .enumerated => "enum",
            .interfaced => "interface",
            .generic => "generic",
        };
    }
};

/// 一元运算符
pub const UnaryOp = enum(u8) {
    neg,    // 取负 (角度反向)
    not,    // 逻辑非

    pub fn toWasmOpcode(self: UnaryOp) u8 {
        return switch (self) {
            .neg => 0x7C, // i32.neg (占位, 角度需要特殊处理)
            .not => 0x45, // i32.eqz (逻辑非)
        };
    }
};

/// 二元运算符
pub const BinaryOp = enum(u8) {
    add, sub, mul, div, mod,     // 算术
    eq, ne, lt, gt, le, ge,      // 比较
    @"and", @"or",               // 逻辑 (保留字需用 @"" 转义)

    /// 转为对应 WASM opcode (简化: 仅映射部分)
    pub fn toWasmOpcode(self: BinaryOp) ?u8 {
        return switch (self) {
            .add => 0x6A, // i32.add
            .sub => 0x6B, // i32.sub
            .mul => 0x6C, // i32.mul
            .div => 0x6D, // i32.div_s
            .mod => 0x6F, // i32.rem_s (近似)
            .eq => 0x46,  // i32.eq
            .ne => 0x47,  // i32.ne
            .lt => 0x48,  // i32.lt_s
            .gt => 0x4A,  // i32.gt_s
            .le => 0x4E,  // i32.le_s
            .ge => 0x50,  // i32.ge_s
            .@"and", .@"or" => null, // 需要特殊处理
        };
    }
};

/// 表达式节点 — 满全法"表面描述"的叶节点
pub const Expr = union(enum) {
    int_literal: struct {
        value: u64,     // 64 进制阶梯制
        is_stride: bool, // 是否是阶梯进制 (sheet × 64 + symbol)
    },
    angle_literal: struct {
        value: u16,     // 0..378 (每 6° 一档, ×6 = 度数)
    },
    coord_literal: struct {
        x: u8, y: u8, z: u8,
    },
    symbol_literal: struct {
        value: u6,      // 0..63
    },
    string_literal: struct {
        value: []const u8,
    },
    ident: struct {
        name: []const u8,
    },
    binary: struct {
        op: BinaryOp,
        lhs: *Expr,
        rhs: *Expr,
    },
    unary: struct {
        op: UnaryOp,
        operand: *Expr,
    },
    call: struct {
        callee: []const u8,
        args: []*Expr,
    },
    field_access: struct {
        object: *Expr,
        field: []const u8,
    },
    index_access: struct {
        object: *Expr,
        index: *Expr,
    },
    match_expr: struct {
        target: *Expr,
        arms: []*MatchArm,
    },
    cast_expr: struct {
        operand: *Expr,
        target_type: Type,
    },
    variant_access: struct {
        enum_name: []const u8,
        variant: []const u8,
    },

    /// 释放表达式内存
    pub fn deinit(self: *Expr, alloc: Allocator) void {
        switch (self.*) {
            .binary => |bin| {
                bin.lhs.deinit(alloc);
                alloc.destroy(bin.lhs);
                bin.rhs.deinit(alloc);
                alloc.destroy(bin.rhs);
            },
            .unary => |u| {
                u.operand.deinit(alloc);
                alloc.destroy(u.operand);
            },
            .field_access => |fa| {
                fa.object.deinit(alloc);
                alloc.destroy(fa.object);
            },
            .call => |c| {
                for (c.args) |arg| {
                    arg.deinit(alloc);
                    alloc.destroy(arg);
                }
                alloc.free(c.args);
            },
            .index_access => |a| {
                a.object.deinit(alloc);
                alloc.destroy(a.object);
                a.index.deinit(alloc);
                alloc.destroy(a.index);
            },
            .match_expr => |m| {
                m.target.deinit(alloc);
                alloc.destroy(m.target);
                for (m.arms) |arm| {
                    arm.pattern.deinit(alloc);
                    alloc.destroy(arm.pattern);
                    arm.body.deinit(alloc);
                    alloc.destroy(arm.body);
                    alloc.destroy(arm);
                }
                alloc.free(m.arms);
            },
            .cast_expr => |c| {
                c.operand.deinit(alloc);
                alloc.destroy(c.operand);
            },
            else => {},
        }
    }
};

/// 语句节点 — 满全法"表面操作"的内部节点
pub const Stmt = union(enum) {
    let_binding: struct {
        name: []const u8,
        is_mut: bool,
        initializer: ?*Expr,
    },
    assignment: struct {
        target: *Expr,
        value: *Expr,
    },
    expr_stmt: struct {
        expr: *Expr,
    },
    if_stmt: struct {
        condition: *Expr,
        then_body: []*Stmt,
        else_body: ?[]*Stmt,
    },
    for_stmt: struct {
        var_name: []const u8,
        iterable: *Expr,
        body: []*Stmt,
    },
    return_stmt: struct {
        value: ?*Expr,
    },
    export_fn: struct {
        name: []const u8,
        params: []Param,
        return_type: Type,
        body: []*Stmt,
    },
    surface_def: struct {
        name: []const u8,
        fields: []Field,
    },
    scene_def: struct {
        name: []const u8,
        entries: []*Stmt,
    },
    enum_def: struct {
        name: []const u8,
        variants: []Variant,
        generics: []GenericParam,
    },
    interface_def: struct {
        name: []const u8,
        methods: []InterfaceMethod,
        generics: []GenericParam,
    },
    impl_block: struct {
        type_name: []const u8,
        interface_name: ?[]const u8,
        methods: []*Stmt,  // export_fn 语句
    },
    type_alias: struct {
        name: []const u8,
        target_type: Type,
    },

    /// 释放语句内存
    pub fn deinit(self: *Stmt, alloc: Allocator) void {
        switch (self.*) {
            .let_binding => |lb| {
                if (lb.initializer) |init| {
                    init.deinit(alloc);
                    alloc.destroy(init);
                }
            },
            .assignment => |a| {
                a.target.deinit(alloc);
                alloc.destroy(a.target);
                a.value.deinit(alloc);
                alloc.destroy(a.value);
            },
            .expr_stmt => |e| {
                e.expr.deinit(alloc);
                alloc.destroy(e.expr);
            },
            .if_stmt => |if_| {
                if_.condition.deinit(alloc);
                alloc.destroy(if_.condition);
                for (if_.then_body) |s| {
                    s.deinit(alloc);
                    alloc.destroy(s);
                }
                alloc.free(if_.then_body);
                if (if_.else_body) |else_body| {
                    for (else_body) |s| {
                        s.deinit(alloc);
                        alloc.destroy(s);
                    }
                    alloc.free(else_body);
                }
            },
            .for_stmt => |f| {
                f.iterable.deinit(alloc);
                alloc.destroy(f.iterable);
                for (f.body) |s| {
                    s.deinit(alloc);
                    alloc.destroy(s);
                }
                alloc.free(f.body);
            },
            .return_stmt => |r| {
                if (r.value) |v| {
                    v.deinit(alloc);
                    alloc.destroy(v);
                }
            },
            .export_fn => |fn_| {
                alloc.free(fn_.params);
                for (fn_.body) |s| {
                    s.deinit(alloc);
                    alloc.destroy(s);
                }
                alloc.free(fn_.body);
            },
            .surface_def => |s| {
                alloc.free(s.fields);
            },
            .scene_def => |sc| {
                for (sc.entries) |e| {
                    e.deinit(alloc);
                    alloc.destroy(e);
                }
                alloc.free(sc.entries);
            },
            .enum_def => |e| {
                alloc.free(e.variants);
                alloc.free(e.generics);
            },
            .interface_def => |i| {
                for (i.methods) |m| {
                    alloc.free(m.params);
                }
                alloc.free(i.methods);
                alloc.free(i.generics);
            },
            .impl_block => |imp| {
                for (imp.methods) |s| {
                    s.deinit(alloc);
                    alloc.destroy(s);
                }
                alloc.free(imp.methods);
            },
            .type_alias => {
                // 无堆分配字段
            },
        }
    }
};

/// 函数参数
pub const Param = struct {
    name: []const u8,
    typ: Type,
};

/// 表面字段
pub const Field = struct {
    name: []const u8,
    typ: Type,
};

/// 枚举变体
pub const Variant = struct {
    name: []const u8,
    value: ?i64,  // 可选显式值
};

/// 接口方法签名
pub const InterfaceMethod = struct {
    name: []const u8,
    params: []Param,
    return_type: Type,
};

/// 泛型参数
pub const GenericParam = struct {
    name: []const u8,
    constraint: ?[]const u8,  // 可选接口约束
};

/// 匹配臂 (for match expressions)
pub const MatchArm = struct {
    pattern: *Expr,      // 匹配模式
    body: *Expr,         // 执行体
};

/// 完整的 mlil 模块 (顶层 AST)
pub const Module = struct {
    name: []const u8,
    imports: [][]const u8,
    stmts: []*Stmt,
    alloc: Allocator,

    pub fn init(alloc: Allocator, name: []const u8) Module {
        return .{
            .name = name,
            .imports = &[_][]const u8{},
            .stmts = &[_]*Stmt{},
            .alloc = alloc,
        };
    }

    /// 添加语句
    pub fn append(self: *Module, stmt: *Stmt) !void {
        self.stmts = try self.alloc.realloc(self.stmts, self.stmts.len + 1);
        self.stmts[self.stmts.len - 1] = stmt;
    }

    /// 释放模块
    pub fn deinit(self: *Module) void {
        for (self.stmts) |s| {
            s.deinit(self.alloc);
            self.alloc.destroy(s);
        }
        self.alloc.free(self.stmts);
        self.alloc.free(self.imports);
    }
};
