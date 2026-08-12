// lexer.zig — mlil 词法分析器
// 将 mlil 源码转换为 token 流

const std = @import("std");
const TokenMod = @import("token.zig");
const Token = TokenMod.Token;
const TokenType = TokenMod.TokenType;

const Allocator = std.mem.Allocator;

pub const Lexer = struct {
    source: []const u8,
    pos: usize,
    line: u32,
    col: u32,
    alloc: Allocator,
    keywords: std.StringHashMap(TokenType),

    pub fn init(alloc: Allocator, source: []const u8) !Lexer {
        return .{
            .source = source,
            .pos = 0,
            .line = 1,
            .col = 1,
            .alloc = alloc,
            .keywords = try TokenMod.initKeywords(),
        };
    }

    pub fn deinit(self: *Lexer) void {
        self.keywords.deinit();
    }

    /// 获取下一个 token
    pub fn next(self: *Lexer) !Token {
        self.skipWhitespace();

        if (self.pos >= self.source.len) {
            return Token.init(.eof, "", self.line, self.col);
        }

        const c = self.source[self.pos];

        // ── 换行 ──
        if (c == '\n') {
            self.pos += 1;
            self.line += 1;
            self.col = 1;
            return Token.init(.newline, "\\n", self.line - 1, self.col);
        }

        // ── 注释 (// 到行尾) ──
        if (c == '/' and self.pos + 1 < self.source.len and self.source[self.pos + 1] == '/') {
            const start = self.pos;
            while (self.pos < self.source.len and self.source[self.pos] != '\n') {
                self.pos += 1;
            }
            return Token.init(.newline, self.source[start..self.pos], self.line, self.col);
        }

        // ── 数字 (64 进制整数或角度) ──
        if (isDigit(c)) {
            return self.readNumber();
        }

        // ── 字符串 ──
        if (c == '"') {
            return self.readString();
        }

        // ── 标识符/关键字/符号名 ──
        if (isAlpha(c)) {
            return self.readIdentifier();
        }

        // ── 单字符 token ──
        return self.readSingleChar(c);
    }

    /// 读取数字 (支持 64 进制和角度)
    fn readNumber(self: *Lexer) !Token {
        const start = self.pos;
        const start_col = self.col;

        while (self.pos < self.source.len and isDigit(self.source[self.pos])) {
            self.pos += 1;
            self.col += 1;
        }

        // 检查是否是角度 (° 后缀, UTF-8: 0xC2 0xB0)
        if (self.pos + 1 < self.source.len and
            self.source[self.pos] == 0xC2 and
            self.source[self.pos + 1] == 0xB0)
        {
            self.pos += 2;
            self.col += 1;
            return Token.init(.angle_literal, self.source[start..self.pos], self.line, start_col);
        }

        return Token.init(.int_literal, self.source[start..self.pos], self.line, start_col);
    }

    /// 读取字符串
    fn readString(self: *Lexer) !Token {
        const start_col = self.col;
        self.pos += 1; // 跳过开头 "
        self.col += 1;

        const start = self.pos;
        while (self.pos < self.source.len and self.source[self.pos] != '"') {
            if (self.source[self.pos] == '\n') {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }

        if (self.pos >= self.source.len) {
            @panic("mlil: 未闭合的字符串");
        }

        const str = self.source[start..self.pos];
        self.pos += 1; // 跳过结尾 "
        self.col += 1;

        return Token.init(.string_literal, str, self.line, start_col);
    }

    /// 读取标识符或关键字
    fn readIdentifier(self: *Lexer) !Token {
        const start = self.pos;
        const start_col = self.col;

        while (self.pos < self.source.len and isAlnum(self.source[self.pos])) {
            self.pos += 1;
            self.col += 1;
        }

        const text = self.source[start..self.pos];

        // 检查是否是关键字
        if (self.keywords.get(text)) |kw| {
            return Token.init(kw, text, self.line, start_col);
        }

        // 检查是否是 64 符号名
        if (TokenMod.symbolNameToValue(text) != null) {
            return Token.init(.sym_literal, text, self.line, start_col);
        }

        return Token.init(.identifier, text, self.line, start_col);
    }

    /// 读取单字符 token
    fn readSingleChar(self: *Lexer, c: u8) !Token {
        const start_col = self.col;
        self.pos += 1;
        self.col += 1;

        const typ: TokenType = switch (c) {
            '+' => .op_plus,
            '-' => blk: {
                // 检查是否是 ->
                if (self.pos < self.source.len and self.source[self.pos] == '>') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_arrow;
                }
                break :blk .op_minus;
            },
            '*' => .op_star,
            '/' => .op_slash,
            '%' => .op_percent,
            '=' => blk: {
                if (self.pos < self.source.len and self.source[self.pos] == '=') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_eq;
                }
                if (self.pos < self.source.len and self.source[self.pos] == '>') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_fat_arrow;
                }
                break :blk .op_assign;
            },
            '!' => blk: {
                if (self.pos < self.source.len and self.source[self.pos] == '=') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_ne;
                }
                break :blk .op_not;
            },
            '<' => blk: {
                if (self.pos < self.source.len and self.source[self.pos] == '=') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_le;
                }
                break :blk .op_lt;
            },
            '>' => blk: {
                if (self.pos < self.source.len and self.source[self.pos] == '=') {
                    self.pos += 1;
                    self.col += 1;
                    break :blk .op_ge;
                }
                break :blk .op_gt;
            },
            '.' => .op_dot,
            ':' => .op_colon,
            ';' => .op_semicolon,
            ',' => .op_comma,
            '(' => .lparen,
            ')' => .rparen,
            '[' => .lbracket,
            ']' => .rbracket,
            '{' => .lbrace,
            '}' => .rbrace,
            '|' => .pipe,
            else => @panic("mlil: 未知字符"),
        };

        return Token.init(typ, self.source[self.pos - 1 .. self.pos], self.line, start_col);
    }

    /// 跳过空白 (不含换行)
    fn skipWhitespace(self: *Lexer) void {
        while (self.pos < self.source.len) {
            const c = self.source[self.pos];
            if (c == ' ' or c == '\t' or c == '\r') {
                self.pos += 1;
                self.col += 1;
            } else {
                break;
            }
        }
    }
};

/// 判断是否是数字
fn isDigit(c: u8) bool {
    return (c >= '0' and c <= '9');
}

/// 判断是否是字母
fn isAlpha(c: u8) bool {
    return (c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or c == '_';
}

/// 判断是否是字母或数字
fn isAlnum(c: u8) bool {
    return isAlpha(c) or isDigit(c);
}
