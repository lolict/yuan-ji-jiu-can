// token.zig — mlil 语言词法定义
// mlil = 毛利兰编程语言 = 表面/界面语言
//
// mlil 的核心语法单元: 可见即可得, 每个 token 对应一个"表面符号"

const std = @import("std");
const Allocator = std.mem.Allocator;

/// mlil token 类型
/// 设计原则: token 本身携带满全法的语义信息
///   - 整数默认是 64 进制阶梯制 (不显示前缀)
///   - 角度用 ° 后缀
///   - 坐标用 [] 包裹
pub const TokenType = enum(u8) {
    // ── 字面量 ──
    int_literal,      // 64 进制整数 (阶梯进制)
    angle_literal,    // 角度值 (如 192°)
    coord_literal,    // 坐标 [x,y,z]
    sym_literal,      // 符号值 (0..63 映射到 64 个符号名称)
    string_literal,   // 字符串

    // ── 标识符 ──
    identifier,       // 变量名/函数名

    // ── 关键字 ──
    kw_module,        // module — 定义模块
    kw_import,        // import — 导入
    kw_surface,       // surface — 定义表面 (结构体)
    kw_scene,         // scene — 定义场景 (可执行界面)
    kw_fn,            // fn — 定义函数
    kw_export,        // export — 导出给宿主
    kw_let,           // let — 绑定变量
    kw_mut,           // mut — 可变绑定
    kw_if,            // if — 条件
    kw_else,          // else — 否则
    kw_for,           // for — 循环
    kw_in,            // in — 迭代
    kw_return,        // return — 返回
    kw_self,          // self — 自身引用
    kw_enum,          // enum — 枚举类型
    kw_interface,     // interface — 接口
    kw_impl,          // impl — 实现
    kw_type,          // type — 类型别名
    kw_match,         // match — 模式匹配
    kw_as,            // as — 类型转换
    kw_struct,        // struct — 结构体（surface 的别名）

    // ── 运算符 ──
    op_plus,          // +
    op_minus,         // -
    op_star,          // *
    op_slash,         // /
    op_percent,       // %
    op_arrow,         // ->
    op_fat_arrow,     // => (for match arms)
    op_assign,        // =
    op_eq,            // ==
    op_ne,            // !=
    op_lt,            // <
    op_gt,            // >
    op_le,            // <=
    op_ge,            // >=
    op_and,           // and
    op_or,            // or
    op_not,           // not
    op_dot,           // .
    op_colon,         // :
    op_semicolon,     // ;
    op_comma,         // ,

    // ── 分隔符 ──
    lparen,           // (
    rparen,           // )
    lbracket,         // [
    rbracket,         // ]
    lbrace,           // {
    rbrace,           // }

    // ── 特殊 ──
    eof,              // 文件结束
    newline,          // 换行 (mlil 中换行有意义)
    pipe,             // | 表面管道

    pub fn isKeyword(self: TokenType) bool {
        return switch (self) {
            .kw_module, .kw_import, .kw_surface, .kw_scene,
            .kw_fn, .kw_export, .kw_let, .kw_mut,
            .kw_if, .kw_else, .kw_for, .kw_in, .kw_return, .kw_self,
            .kw_enum, .kw_interface, .kw_impl, .kw_type, .kw_match,
            .kw_as, .kw_struct,
            => true,
            else => false,
        };
    }
};

/// Token 实例
pub const Token = struct {
    typ: TokenType,
    lexeme: []const u8,      // 源码片段
    line: u32,
    col: u32,

    pub fn init(typ: TokenType, lexeme: []const u8, line: u32, col: u32) Token {
        return .{ .typ = typ, .lexeme = lexeme, .line = line, .col = col };
    }
};

/// 关键字表 — mlil 的关键字全部是英文简写, 保持"表面"的简洁感
/// 使用 page_allocator 避免对 libc 的依赖
pub fn initKeywords() !std.StringHashMap(TokenType) {
    var map = std.StringHashMap(TokenType).init(std.heap.page_allocator);
    try map.put("module", .kw_module);
    try map.put("import", .kw_import);
    try map.put("surface", .kw_surface);
    try map.put("scene", .kw_scene);
    try map.put("fn", .kw_fn);
    try map.put("export", .kw_export);
    try map.put("let", .kw_let);
    try map.put("mut", .kw_mut);
    try map.put("if", .kw_if);
    try map.put("else", .kw_else);
    try map.put("for", .kw_for);
    try map.put("in", .kw_in);
    try map.put("return", .kw_return);
    try map.put("self", .kw_self);
    try map.put("enum", .kw_enum);
    try map.put("interface", .kw_interface);
    try map.put("impl", .kw_impl);
    try map.put("type", .kw_type);
    try map.put("match", .kw_match);
    try map.put("as", .kw_as);
    try map.put("struct", .kw_struct);
    try map.put("and", .op_and);
    try map.put("or", .op_or);
    try map.put("not", .op_not);
    return map;
}

/// 64 符号满全法表 (简化映射: 0..63 → 名称)
pub const SYMBOL_NAMES = [64][]const u8{
    "ymba", "roon", "gail", "wezi", "sita", "fino", "koma", "pelm",
    "huxe", "bria", "soln", "cryl", "nemu", "dral", "fexo", "glam",
    "prin", "quol", "stex", "trio", "urmi", "vand", "wexo", "xylo",
    "yaru", "zeph", "alve", "blix", "corr", "dext", "elve", "frim",
    "grav", "holm", "ixis", "jolt", "kren", "luxe", "morb", "nive",
    "ovol", "plix", "quiv", "rove", "silx", "trin", "uvae", "vix",
    "wyrm", "xand", "yvry", "zond", "ambr", "brux", "cyph", "dren",
    "eluc", "fryn", "glyn", "hyre", "ixor", "jyx", "kyzn", "lyrm",
};

/// 将符号名映射为数值 (如果识别不了 → null)
pub fn symbolNameToValue(name: []const u8) ?u6 {
    for (SYMBOL_NAMES, 0..) |sym, i| {
        if (std.mem.eql(u8, sym, name)) return @intCast(i);
    }
    return null;
}
