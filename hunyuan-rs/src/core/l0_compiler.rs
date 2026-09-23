// ══════════════════════════════════════════════════════════════════
// l0_compiler.rs — L0 高层语言 → MCCP MIR 编译器
// ══════════════════════════════════════════════════════════════════
//
// L0 = 极简 ALGOL-like 源语言, 编译为我们已有的 MCCP Vm.
// 是 MCCP 自举编译器 v1 的核心.
//
// 语法:
//   program   ::= stmt*
//   stmt      ::= assign | if | while | print | printc | label | jmp | call | ret
//   assign    ::= VAR '=' expr
//   if        ::= 'if' expr 'then' stmt* ('else' stmt*)? 'end'
//   while     ::= 'while' expr 'do' stmt* 'end'
//   print     ::= 'print' expr
//   printc    ::= 'printc' expr
//   label     ::= IDENT ':'
//   jmp       ::= 'jmp' IDENT
//   jz        ::= 'jz' IDENT expr
//   jnz       ::= 'jnz' IDENT expr
//   call      ::= 'call' IDENT
//   ret       ::= 'ret'
//   expr      ::= term (('+'|'-'|'<>'|'<'|'>'|'<='|'>='|'&'|'|'|'^') term)*
//   term      <- factor (('*' | '/' | '%') factor)*
//   factor    ::= INT | VAR | '(' expr ')' | '-' factor
// 注释: ';' 到行尾
//
// 变量命名: 单字母 a-z 映射到内存槽位 0..25

use std::collections::HashMap;
use std::fmt;

use crate::mccp::Mccp;
use crate::mccp_vm::{
    self, op, op_imm, OP_ADD, OP_SUB, OP_MUL, OP_MUL_CONST, OP_ADD_CONST, OP_DIV, OP_MOD,
    OP_AND, OP_OR, OP_XOR, OP_SHL,
    OP_EQ, OP_NE, OP_LT, OP_GT, OP_LE, OP_GE,
    OP_PUSH, OP_JMP, OP_JZ, OP_JNZ, OP_CALL, OP_RET,
    OP_LOAD, OP_STORE, OP_LOAD_IND, OP_STORE_IND,
    OP_PRINT, OP_PRINTC, OP_WRITERAW, OP_SYSCALL, OP_HALT,
};

// ────────────────────────────────────────────────────────────
// 错误
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    LexError(String),
    ParseError(String, usize),
    UndefinedVar(char, usize),
    UndefinedLabel(String, usize),
    DupLabel(String),
}
impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexError(s) => write!(f, "词法错误: {}", s),
            Self::ParseError(s, ln) => write!(f, "语法错误 (第{}行): {}", ln, s),
            Self::UndefinedVar(v, ln) => write!(f, "未定义变量 '{}' (第{}行)", v, ln),
            Self::UndefinedLabel(l, ln) => write!(f, "未定义标签 '{}' (第{}行)", l, ln),
            Self::DupLabel(l) => write!(f, "重复标签 '{}'", l),
        }
    }
}
impl std::error::Error for CompileError {}

// ────────────────────────────────────────────────────────────
// 词法分析
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Int(i32),
    Var(char),
    Ident(String),
    Op(Symbol),
    If, Then, Else, End,
    While, Do,
    Print, Printc,
    WriteRaw,         // 写原始字节到 output_raw (逃逸 UTF-8 膨胀)
    Jmp, Jz, JzN, Call, Ret, Halt,
    Input,           // 读一个字符
    Peek,            // 间接寻址读: peek var → var = memory[var_value]
Poke,            // 间接寻址写: poke addr_var val_var → memory[addr] = val
    Semicolon,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
enum Symbol {
    Plus, Minus, Star, Slash, Percent,
    LParen, RParen,
    Assign,
    Lt, Gt, Le, Ge, Eq, Ne,
    Amp, Bar, Caret, Tilde, Not,
    Colon,
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.input.get(self.pos).copied();
        if c == Some('\n') {
            self.line += 1;
        }
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == ';' {
                while let Some(c) = self.peek() {
                    if c == '\n' { break; }
                    self.advance();
                }
            } else if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn lex(&mut self) -> Result<Vec<(Tok, usize)>, CompileError> {
        let mut toks = Vec::new();
        loop {
            self.skip_ws();
            let line = self.line;
            match self.peek() {
                None => { toks.push((Tok::Eof, line)); break; }
                Some('\n') => {
                    self.advance();
                    toks.push((Tok::Newline, line));
                }
                Some(c) if c.is_ascii_digit() => {
                    let mut s = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() { s.push(c); self.advance(); }
                        else { break; }
                    }
                    match s.parse::<i32>() {
                        Ok(n) => toks.push((Tok::Int(n), line)),
                        Err(_) => return Err(CompileError::LexError(format!("数字解析失败: {}", s))),
                    }
                }
                Some(c) if c.is_ascii_lowercase() => {
                    let mut s = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_lowercase() || c == '_' {
                            s.push(c); self.advance();
                        } else { break; }
                    }
                    // Check if this is a reserved word (case-insensitive)
                    let upper = s.to_uppercase();
                    let tok = match upper.as_str() {
                        "PRINT" => Tok::Print,
                        "PRINTC" => Tok::Printc,
                        "WRITERAW" => Tok::WriteRaw,
                        "INPUT" => Tok::Input,
                        "PEEK" => Tok::Peek,
                        "POKE" => Tok::Poke,
                        "HALT" => Tok::Halt,
                        "IF" => Tok::If,
                        "THEN" => Tok::Then,
                        "ELSE" => Tok::Else,
                        "END" => Tok::End,
                        "WHILE" => Tok::While,
                        "DO" => Tok::Do,
                        "JMP" => Tok::Jmp,
                        "JZ" => Tok::Jz,
                        "JNZ" => Tok::JzN,
                        "CALL" => Tok::Call,
                        "RET" => Tok::Ret,
                        _ => {
                            if s.len() == 1 {
                                Tok::Var(s.chars().next().unwrap())
                            } else {
                                // Multi-char identifier = label name
                                Tok::Ident(s)
                            }
                        }
                    };
                    toks.push((tok, line));
                }
                Some(c) if c.is_ascii_uppercase() || c == '_' => {
                    let mut s = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            s.push(c); self.advance();
                        } else { break; }
                    }
                    let upper = s.to_uppercase();
                    let tok = match upper.as_str() {
                        "THEN" => Tok::Then,
                        "IF" => Tok::If,
                        "ELSE" => Tok::Else,
                        "END" => Tok::End,
                        "WHILE" => Tok::While,
                        "DO" => Tok::Do,
                        "PRINT" => Tok::Print,
                        "PRINTC" => Tok::Printc,
                        "WRITERAW" => Tok::WriteRaw,
                        "INPUT" => Tok::Input,
                        "PEEK" => Tok::Peek,
                        "POKE" => Tok::Poke,
                        "HALT" => Tok::Halt,
                        "JMP" => Tok::Jmp,
                        "JZ" => Tok::Jz,
                        "JNZ" => Tok::JzN,
                        "CALL" => Tok::Call,
                        "RET" => Tok::Ret,
                        _ => Tok::Ident(s),
                    };
                    toks.push((tok, line));
                }
                Some('+') => { self.advance(); toks.push((Tok::Op(Symbol::Plus), line)); }
                Some('-') => { self.advance(); toks.push((Tok::Op(Symbol::Minus), line)); }
                Some('*') => { self.advance(); toks.push((Tok::Op(Symbol::Star), line)); }
                Some('/') => { self.advance(); toks.push((Tok::Op(Symbol::Slash), line)); }
                Some('%') => { self.advance(); toks.push((Tok::Op(Symbol::Percent), line)); }
                Some('(') => { self.advance(); toks.push((Tok::Op(Symbol::LParen), line)); }
                Some(')') => { self.advance(); toks.push((Tok::Op(Symbol::RParen), line)); }
                Some('<') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance(); toks.push((Tok::Op(Symbol::Le), line));
                    } else if self.peek() == Some('>') {
                        self.advance(); toks.push((Tok::Op(Symbol::Ne), line));
                    } else {
                        toks.push((Tok::Op(Symbol::Lt), line));
                    }
                }
                Some('>') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance(); toks.push((Tok::Op(Symbol::Ge), line));
                    } else {
                        toks.push((Tok::Op(Symbol::Gt), line));
                    }
                }
                Some('=') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        toks.push((Tok::Op(Symbol::Eq), line));
                    } else {
                        toks.push((Tok::Op(Symbol::Assign), line));
                    }
                }
                Some('&') => { self.advance(); toks.push((Tok::Op(Symbol::Amp), line)); }
                Some('|') => { self.advance(); toks.push((Tok::Op(Symbol::Bar), line)); }
                Some('^') => { self.advance(); toks.push((Tok::Op(Symbol::Caret), line)); }
                Some(':') => { self.advance(); toks.push((Tok::Op(Symbol::Colon), line)); }
                Some(c) => return Err(CompileError::LexError(format!("非法字符 '{}' (第{}行)", c, line))),
            }
        }
        Ok(toks)
    }
}

// ────────────────────────────────────────────────────────────
// AST
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Expr {
    Int(i32),
    Var(String),
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
enum UnOp { Neg, Not }

#[derive(Debug, Clone, Copy)]
enum BinOp { Add, Sub, Mul, Div, Mod, Lt, Gt, Le, Ge, Eq, Ne, And, Or, Xor }

#[derive(Debug, Clone)]
enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),    // if cond then-body else-body
    While(Expr, Vec<Stmt>),
    Nop,
    Print(Expr),
    PrintC(Expr),
    WriteRaw(Expr),                   // writeraw expr - 写原始字节到 output_raw
    Input(String),                    // input var - 从输入缓冲读一个字符到变量
    Peek(String),                     // peek var → var = memory[var_value]
Poke(String, String),              // poke addr_var val_var → memory[addr] = val
    Label(String),
    Jmp(String),
    Jz(String, Expr),
    Jnz(String, Expr),
    Call(String),
    Ret,
    Halt,
}

// ────────────────────────────────────────────────────────────
// 解析器 (递归下降)
// ────────────────────────────────────────────────────────────

struct Parser {
    toks: Vec<(Tok, usize)>,
    pos: usize,
}

impl Parser {
    fn new(toks: Vec<(Tok, usize)>) -> Self {
        Self { toks, pos: 0 }
    }

    fn peek_tok(&self) -> &Tok {
        self.toks.get(self.pos).map(|(t, _)| t).unwrap_or(&Tok::Eof)
    }

    fn peek_line(&self) -> usize {
        self.toks.get(self.pos).map(|(_, l)| *l).unwrap_or(0)
    }

    fn advance(&mut self) -> (Tok, usize) {
        let t = self.toks.get(self.pos).cloned().unwrap_or((Tok::Eof, 0));
        if self.pos < self.toks.len() - 1 {
            self.pos += 1;
        } else {
            self.pos = self.toks.len();
        }
        t
    }

    fn expect(&mut self, expected: Tok) -> Result<(), CompileError> {
        let (tok, line) = self.advance();
        if tok == expected {
            Ok(())
        } else {
            Err(CompileError::ParseError(
                format!("期望 {:?}, 得到 {:?}", expected, tok),
                line,
            ))
        }
    }

    fn parse(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = Vec::new();
        while *self.peek_tok() != Tok::Eof {
            if *self.peek_tok() == Tok::Newline {
                self.advance();
                continue;
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, CompileError> {
        match self.peek_tok().clone() {
            Tok::Var(name) => {
                let line = self.peek_line();
                self.advance();
                let name_str = name.to_string();
                match self.peek_tok() {
                    Tok::Op(Symbol::Colon) => {
                        self.advance();
                        Ok(Stmt::Label(name_str))
                    }
                    Tok::Op(Symbol::Assign) => {
                        self.advance();
                        let expr = self.parse_expr()?;
                        Ok(Stmt::Assign(name_str, expr))
                    }
                    _ => Err(CompileError::ParseError(
                        format!("'{}' 后期待 '=' 或 ':'", name),
                        line,
                    )),
                }
            }
            Tok::Ident(name) => {
                let line = self.peek_line();
                self.advance();
                match self.peek_tok() {
                    Tok::Op(Symbol::Colon) => {
                        self.advance();
                        Ok(Stmt::Label(name))
                    }
                    Tok::Op(Symbol::Assign) => {
                        self.advance();
                        let expr = self.parse_expr()?;
                        Ok(Stmt::Assign(name, expr))
                    }
                    _ => Err(CompileError::ParseError(
                        format!("标识符 '{}' 后期待 '=' 或 ':'", name),
                        line,
                    )),
                }
            }
            Tok::Print => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::Print(expr))
            }
            Tok::Printc => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::PrintC(expr))
            }
            Tok::WriteRaw => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::WriteRaw(expr))
            }
            Tok::Input => {
                self.advance();
                let var = self.parse_input_var()?;
                Ok(Stmt::Input(var))
            }
            Tok::Peek => {
                // peek var → var = memory[var_value]
                self.advance();
                let var = self.parse_input_var()?;
                Ok(Stmt::Peek(var))
            }
            Tok::Poke => {
                // poke addr_var val_var → memory[addr] = val
                let line = self.peek_line();
                self.advance();
                let addr_var = self.parse_input_var()?;
                let val_var = self.parse_input_var()?;
                if addr_var.len() != 1 || val_var.len() != 1 {
                    return Err(CompileError::ParseError(
                        format!("POKE 要求单字符变量 (got {}, {})", addr_var, val_var),
                        line,
                    ));
                }
                Ok(Stmt::Poke(addr_var, val_var))
            }
            Tok::Jmp => {
                let line = self.peek_line();
                self.advance();
                let label = self.parse_jump_label()?;
                Ok(Stmt::Jmp(label))
            }
            Tok::Jz => {
                let line = self.peek_line();
                self.advance();
                let label = self.parse_jump_label().map_err(|_| {
                    CompileError::ParseError("JZ 后期待标签".into(), line)
                })?;
                let expr = self.parse_expr()?;
                Ok(Stmt::Jz(label, expr))
            }
            Tok::JzN => {
                let line = self.peek_line();
                self.advance();
                let label = self.parse_jump_label().map_err(|_| {
                    CompileError::ParseError("JNZ 后期待标签".into(), line)
                })?;
                let expr = self.parse_expr()?;
                Ok(Stmt::Jnz(label, expr))
            }
            Tok::Call => {
                let line = self.peek_line();
                self.advance();
                let label = self.parse_jump_label().map_err(|_| {
                    CompileError::ParseError("CALL 后期待标签".into(), line)
                })?;
                Ok(Stmt::Call(label))
            }
            Tok::Ret => {
                self.advance();
                Ok(Stmt::Ret)
            }
            Tok::Halt => {
                self.advance();
                Ok(Stmt::Halt)
            }
            Tok::While => {
                self.advance();
                let cond = self.parse_expr()?;
                self.expect(Tok::Do)?;
                let mut body = Vec::new();
                while !self.is_block_end() {
                    if *self.peek_tok() == Tok::Newline { self.advance(); continue; }
                    body.push(self.parse_stmt()?);
                }
                self.expect(Tok::End)?;
                Ok(Stmt::While(cond, body))
            }
            Tok::If => {
                self.advance();
                let cond = self.parse_expr()?;
                self.expect(Tok::Then)?;
                let mut then_body = Vec::new();
                while !self.is_block_end_or_else() {
                    if *self.peek_tok() == Tok::Newline { self.advance(); continue; }
                    then_body.push(self.parse_stmt()?);
                }
                let else_body = if *self.peek_tok() == Tok::Else {
                    self.advance();
                    let mut body = Vec::new();
                    while !self.is_block_end() {
                        if *self.peek_tok() == Tok::Newline { self.advance(); continue; }
                        body.push(self.parse_stmt()?);
                    }
                    body
                } else {
                    Vec::new()
                };
                self.expect(Tok::End)?;
                Ok(Stmt::If(cond, then_body, else_body))
            }
            Tok::Then | Tok::Else | Tok::End | Tok::Do => {
                // 可能是保留字作标签: "end:" / "else:" 等
                let line = self.peek_line();
                let name = match self.peek_tok() {
                    Tok::End => "end".to_string(),
                    Tok::Else => "else".to_string(),
                    Tok::Then => "then".to_string(),
                    Tok::Do => "do".to_string(),
                    _ => unreachable!(),
                };
                self.advance();
                if *self.peek_tok() == Tok::Op(Symbol::Colon) {
                    self.advance();
                    Ok(Stmt::Label(name))
                } else {
                    Err(CompileError::ParseError(format!("意外的关键字 {} (开头应为变量/print/if/while)", name), line))
                }
            }
            _ => {
                let line = self.peek_line();
                let tok = self.peek_tok().clone();
                self.advance();
                Err(CompileError::ParseError(format!("意外的 tok {:?}", tok), line))
            }
        }
    }

    /// 检查当前是否为真正的块结束 (Tok::End 不跟冒号)
    fn is_block_end(&self) -> bool {
        if *self.peek_tok() == Tok::Eof { return true; }
        if *self.peek_tok() == Tok::End {
            // 检查下一个 token 是否是冒号 → end: 是标签，不是块结束
            self.toks.get(self.pos + 1)
                .map(|(t, _)| *t != Tok::Op(Symbol::Colon))
                .unwrap_or(true)
        } else {
            false
        }
    }

    /// 检查是否为 else 分支或块结束
    fn is_block_end_or_else(&self) -> bool {
        *self.peek_tok() == Tok::Else || self.is_block_end()
    }

    /// 解析跳转/调用目标标签 — 接受 Ident、Var 及部分保留字作标签名
    fn parse_jump_label(&mut self) -> Result<String, CompileError> {
        let line = self.peek_line();
        match self.peek_tok().clone() {
            Tok::Ident(s) => { self.advance(); Ok(s) }
            Tok::Var(c) => { self.advance(); Ok(c.to_string()) }
            Tok::End => { self.advance(); Ok("end".into()) }
            Tok::Else => { self.advance(); Ok("else".into()) }
            Tok::Then => { self.advance(); Ok("then".into()) }
            Tok::Do => { self.advance(); Ok("do".into()) }
            Tok::While => { self.advance(); Ok("while".into()) }
            Tok::If => { self.advance(); Ok("if".into()) }
            Tok::Print => { self.advance(); Ok("print".into()) }
            Tok::Printc => { self.advance(); Ok("printc".into()) }
            Tok::WriteRaw => { self.advance(); Ok("writeraw".into()) }
            Tok::Ret => { self.advance(); Ok("ret".into()) }
            Tok::Jmp => { self.advance(); Ok("jmp".into()) }
            Tok::Jz => { self.advance(); Ok("jz".into()) }
            Tok::JzN => { self.advance(); Ok("jnz".into()) }
            Tok::Call => { self.advance(); Ok("call".into()) }
            other => Err(CompileError::ParseError(
                format!("期望标签名, 得到 {:?}", other), line)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, CompileError> {
        self.parse_comparison()
    }

    /// 解析 input 后的变量名
    fn parse_input_var(&mut self) -> Result<String, CompileError> {
        let line = self.peek_line();
        match self.peek_tok().clone() {
            Tok::Var(c) => { self.advance(); Ok(c.to_string()) }
            Tok::Ident(name) => { self.advance(); Ok(name) }
            _ => Err(CompileError::ParseError(
                format!("input 后期望变量名"), line)),
        }
    }

    fn parse_comparison(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_add()?;
        loop {
            let op = match self.peek_tok() {
                Tok::Op(Symbol::Lt) => BinOp::Lt,
                Tok::Op(Symbol::Gt) => BinOp::Gt,
                Tok::Op(Symbol::Le) => BinOp::Le,
                Tok::Op(Symbol::Ge) => BinOp::Ge,
                Tok::Op(Symbol::Eq) => BinOp::Eq,
                Tok::Op(Symbol::Ne) => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_add()?;
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek_tok() {
                Tok::Op(Symbol::Plus) => BinOp::Add,
                Tok::Op(Symbol::Minus) => BinOp::Sub,
                Tok::Op(Symbol::Amp) => BinOp::And,
                Tok::Op(Symbol::Bar) => BinOp::Or,
                Tok::Op(Symbol::Caret) => BinOp::Xor,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.peek_tok() {
                Tok::Op(Symbol::Star) => BinOp::Mul,
                Tok::Op(Symbol::Slash) => BinOp::Div,
                Tok::Op(Symbol::Percent) => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, CompileError> {
        match self.peek_tok().clone() {
            Tok::Int(n) => { self.advance(); Ok(Expr::Int(n)) }
            Tok::Var(c) => { self.advance(); Ok(Expr::Var(c.to_string())) }
            Tok::Ident(name) => { self.advance(); Ok(Expr::Var(name)) }
            Tok::Op(Symbol::Minus) => {
                self.advance();
                let inner = self.parse_factor()?;
                Ok(Expr::UnOp(UnOp::Neg, Box::new(inner)))
            }
            Tok::Op(Symbol::Tilde) => {
                self.advance();
                let inner = self.parse_factor()?;
                Ok(Expr::UnOp(UnOp::Not, Box::new(inner)))
            }
            Tok::Op(Symbol::LParen) => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(Tok::Op(Symbol::RParen))?;
                Ok(e)
            }
            _ => {
                let line = self.peek_line();
                let tok = self.peek_tok().clone();
                self.advance();
                Err(CompileError::ParseError(format!("意外的 tok {:?}", tok), line))
            }
        }
    }
}

// ────────────────────────────────────────────────────────────
// 代码生成
// ────────────────────────────────────────────────────────────

const VAR_BASE: usize = 0;       // a..z → memory[VAR_BASE..VAR_BASE+26]
const LABEL_PATCH: i16 = -9999;  // 占位地址

struct CodeGen {
    code: Vec<Mccp>,
    labels: HashMap<String, usize>,
    patches: Vec<(usize, String)>,  // (code_idx, label_name)
    vars: HashMap<String, u16>,     // var_name → memory address
    next_addr: u16,                 // next free address for new var
}

impl CodeGen {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            patches: Vec::new(),
            vars: HashMap::new(),
            next_addr: VAR_BASE as u16,
        }
    }

    fn emit(&mut self, m: Mccp) {
        self.code.push(m);
    }

    fn emit_op(&mut self, opcode: u16) {
        self.emit(op(opcode));
    }

    fn emit_imm(&mut self, opcode: u16, imm: i16) {
        self.emit(op_imm(opcode, imm));
    }

    /// 推送任意 i32 常数到栈。若超出 i16 范围,拆分为
    ///   PUSH(hi_adj) PUSH(16) SHL PUSH(lo_signed) ADD
    /// 其中 hi_adj = n>>16 (+1 若 lo 半字为负),lo_signed = 低 16 位有符号解释。
    /// 适用于 i16::MAX+1 .. 0x7FFF_7FFF(约 21 亿)范围内的正数及对应的负数。
    fn emit_push_i32(&mut self, n: i32) {
        const I16_MIN: i32 = i16::MIN as i32;
        const I16_MAX: i32 = i16::MAX as i32;
        if n >= I16_MIN && n <= I16_MAX {
            self.emit_imm(OP_PUSH, n as i16);
            return;
        }
        let hi_raw = n >> 16;
        let lo_unsigned = (n as u32) & 0xFFFF;
        let lo_signed = lo_unsigned as i16;
        let hi_adj = if lo_signed < 0 {
            hi_raw.wrapping_add(1)
        } else {
            hi_raw
        };
        self.emit_imm(OP_PUSH, hi_adj as i16);
        self.emit_imm(OP_PUSH, 16);
        self.emit_op(OP_SHL);
        self.emit_imm(OP_PUSH, lo_signed);
        self.emit_op(OP_ADD);
    }

    fn define_label(&mut self, name: &str) -> Result<(), CompileError> {
        if self.labels.contains_key(name) {
            return Err(CompileError::DupLabel(name.to_string()));
        }
        self.labels.insert(name.to_string(), self.code.len());
        Ok(())
    }

    fn reference_label(&mut self, name: &str, opcode: u16) {
        self.patches.push((self.code.len(), name.to_string()));
        self.emit_imm(opcode, LABEL_PATCH);
    }

    fn patch(&mut self) -> Result<(), CompileError> {
        for (idx, label) in &self.patches {
            match self.labels.get(label) {
                Some(&addr) => {
                    self.code[*idx] = op_imm(self.code[*idx].index(), addr as i16);
                }
                None => return Err(CompileError::UndefinedLabel(label.clone(), 0)),
            }
        }
        Ok(())
    }

    fn var_addr(&mut self, name: &str) -> u16 {
        if let Some(&addr) = self.vars.get(name) {
            return addr;
        }
        let addr = self.next_addr;
        self.next_addr += 1;
        self.vars.insert(name.to_string(), addr);
        addr
    }

    fn gen_expr(&mut self, e: &Expr) -> Result<(), CompileError> {
        match e {
            Expr::Int(n) => self.emit_push_i32(*n),
            Expr::Var(name) => {
                let addr = self.var_addr(name);
                self.emit_imm(OP_LOAD, addr as i16);
            }
            Expr::UnOp(op, inner) => {
                self.gen_expr(inner)?;
                match op {
                    UnOp::Neg => self.emit_op(mccp_vm::OP_NEG),
                    UnOp::Not => self.emit_op(mccp_vm::OP_NOT),
                }
            }
            Expr::BinOp(op, l, r) => {
                // 优化: "表达式 OP 常数"(c 在 i16 范围内)→ 使用 MULconst/ADDconst 立即数指令
                // 语义: l 已过栈顶,r 是指令内嵌立即数 → 少一次 PUSH
                match (op, r.as_ref()) {
                    (BinOp::Add, Expr::Int(n))
                        if *n >= i16::MIN as i32 && *n <= i16::MAX as i32 =>
                    {
                        self.gen_expr(l)?;
                        self.emit_imm(OP_ADD_CONST, *n as i16);
                    }
                    (BinOp::Mul, Expr::Int(n))
                        if *n >= i16::MIN as i32 && *n <= i16::MAX as i32 =>
                    {
                        self.gen_expr(l)?;
                        self.emit_imm(OP_MUL_CONST, *n as i16);
                    }
                    _ => {
                        // 常规路径: 左值入栈,右值入栈,二元运算
                        self.gen_expr(l)?;
                        self.gen_expr(r)?;
                        let opcode = match op {
                            BinOp::Add => OP_ADD,
                            BinOp::Sub => OP_SUB,
                            BinOp::Mul => OP_MUL,
                            BinOp::Div => OP_DIV,
                            BinOp::Mod => OP_MOD,
                            BinOp::Lt => OP_LT,
                            BinOp::Gt => OP_GT,
                            BinOp::Le => OP_LE,
                            BinOp::Ge => OP_GE,
                            BinOp::Eq => OP_EQ,
                            BinOp::Ne => OP_NE,
                            BinOp::And => OP_AND,
                            BinOp::Or => OP_OR,
                            BinOp::Xor => OP_XOR,
                        };
                        self.emit_op(opcode);
                    }
                }
            }
        }
        Ok(())
    }

    fn gen_stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        match s {
            Stmt::Nop => Ok(()),
            Stmt::Label(name) => self.define_label(name),
            Stmt::Jmp(label) => {
                self.reference_label(label, OP_JMP);
                Ok(())
            }
            Stmt::Jz(label, cond) => {
                self.gen_expr(cond)?;
                self.reference_label(label, OP_JZ);
                Ok(())
            }
            Stmt::Jnz(label, cond) => {
                self.gen_expr(cond)?;
                self.reference_label(label, OP_JNZ);
                Ok(())
            }
            Stmt::Call(label) => {
                self.reference_label(label, OP_CALL);
                Ok(())
            }
            Stmt::Ret => {
                self.emit_op(OP_RET);
                Ok(())
            }
            Stmt::Peek(addr_var) => {
                // peek x → x = memory[x]
                // 汇编: LOAD x, LOAD_IND, STORE x
                let slot = self.var_addr(addr_var);
                self.emit_imm(OP_LOAD, slot as i16);
                self.emit_op(OP_LOAD_IND);
                self.emit_imm(OP_STORE, slot as i16);
                Ok(())
            }
            Stmt::Poke(addr_var, val_var) => {
                // poke A B → memory[A] = B
                // 汇编: LOAD B, LOAD A, STORE_IND (addr value STORE_IND)
                let a_slot = self.var_addr(addr_var);
                let v_slot = self.var_addr(val_var);
                self.emit_imm(OP_LOAD, v_slot as i16);
                self.emit_imm(OP_LOAD, a_slot as i16);
                self.emit_op(OP_STORE_IND);
                Ok(())
            }
            Stmt::Halt => {
                self.emit_op(OP_HALT);
                Ok(())
            }
            Stmt::Assign(name, expr) => {
                self.gen_expr(expr)?;
                let addr = self.var_addr(name);
                self.emit_imm(OP_STORE, addr as i16);
                Ok(())
            }
            Stmt::Print(expr) => {
                self.gen_expr(expr)?;
                self.emit_op(OP_PRINT);
                Ok(())
            }
            Stmt::PrintC(expr) => {
                self.gen_expr(expr)?;
                self.emit_op(OP_PRINTC);
                Ok(())
            }
            Stmt::WriteRaw(expr) => {
                self.gen_expr(expr)?;
                self.emit_op(OP_WRITERAW);
                Ok(())
            }
            Stmt::Input(name) => {
                // syscall 2 = READ_CHAR
                self.emit_imm(OP_SYSCALL, 2);
                let addr = self.var_addr(name);
                self.emit_imm(OP_STORE, addr as i16);
                Ok(())
            }
            Stmt::If(cond, then_body, else_body) => {
                self.gen_expr(cond)?;
                let jz_idx = self.code.len();
                self.emit_imm(OP_JZ, LABEL_PATCH); // jump past then-body if cond==0

                for stmt in then_body {
                    self.gen_stmt(stmt)?;
                }

                if !else_body.is_empty() {
                    let jmp_idx = self.code.len();
                    self.emit_imm(OP_JMP, LABEL_PATCH); // jump past else-body

                    // Patch jz to here (start of else)
                    let else_start = self.code.len() as i16;
                    self.code[jz_idx] = op_imm(OP_JZ, else_start);

                    for stmt in else_body {
                        self.gen_stmt(stmt)?;
                    }

                    // Patch jmp to here (end of else)
                    let end = self.code.len() as i16;
                    self.code[jmp_idx] = op_imm(OP_JMP, end);
                } else {
                    // Patch jz to here
                    let after_then = self.code.len() as i16;
                    self.code[jz_idx] = op_imm(OP_JZ, after_then);
                }
                Ok(())
            }
            Stmt::While(cond, body) => {
                let loop_top = self.code.len();
                self.gen_expr(cond)?;
                let jz_idx = self.code.len();
                self.emit_imm(OP_JZ, LABEL_PATCH); // exit loop when cond==0
                for stmt in body {
                    self.gen_stmt(stmt)?;
                }
                self.emit_imm(OP_JMP, loop_top as i16); // jump back to top
                let after = self.code.len() as i16;
                self.code[jz_idx] = op_imm(OP_JZ, after);
                Ok(())
            }
        }
    }

    fn gen_program(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        for stmt in stmts {
            self.gen_stmt(stmt)?;
        }
        self.emit_op(OP_HALT);
        self.patch()?;
        Ok(())
    }
}

// ────────────────────────────────────────────────────────────
// 公共 API
// ────────────────────────────────────────────────────────────

pub fn compile_l0(source: &str) -> Result<Vec<Mccp>, CompileError> {
    let mut lexer = Lexer::new(source);
    let toks = lexer.lex()?;
    let mut parser = Parser::new(toks);
    let stmts = parser.parse()?;
    let mut gen = CodeGen::new();
    gen.gen_program(&stmts)?;
    Ok(gen.code)
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mccp_vm::{Vm, VmConfig};

    fn run_l0(source: &str) -> Result<(String, u64), CompileError> {
        let code = compile_l0(source)?;
        let mut vm = Vm::new(code);
        vm.run().map_err(|e| CompileError::ParseError(format!("VM error: {:?}", e), 0))?;
        Ok((vm.output.clone(), vm.steps))
    }

    fn run_l0_with_input(source: &str, input: &str) -> Result<(String, u64), CompileError> {
        let code = compile_l0(source)?;
        let mut vm = Vm::with_input(code, VmConfig::default(), input.into());
        vm.run().map_err(|e| CompileError::ParseError(format!("VM error: {:?}", e), 0))?;
        Ok((vm.output.clone(), vm.steps))
    }

    #[test]
    fn l0_assign_and_add() {
        let src = "a = 3\nb = 4\nc = a + b\nprint c";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "7");
    }

    #[test]
    fn l0_sub_order() {
        let src = "a = 10\nb = 3\nc = a - b\nprint c";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "7");
    }

    #[test]
    fn l0_mul() {
        let src = "a = 6\nb = 7\nprint a * b";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "42");
    }

    #[test]
    fn l0_div() {
        let src = "a = 20\nb = 4\nprint a / b";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "5");
    }

    #[test]
    fn l0_mod() {
        let src = "a = 17\nb = 5\nprint a % b";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "2");
    }

    #[test]
    fn l0_negate() {
        let src = "a = -5\nprint a";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "-5");
    }

    #[test]
    fn l0_parens() {
        let src = "print (2 + 3) * 4";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "20");
    }

    #[test]
    fn l0_precedence() {
        let src = "print 2 + 3 * 4";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "14");
    }

    #[test]
    fn l0_comparison_gt() {
        let src = "print 5 > 3";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "1");
    }

    #[test]
    fn l0_comparison_lt_false() {
        let src = "print 3 > 5";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "0");
    }

    #[test]
    fn l0_eq() {
        let src = "print 42 == 42";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "1");
    }

    #[test]
    fn l0_ne() {
        let src = "print 1 <> 2";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "1");
    }

    #[test]
    fn l0_if_then() {
        let src = "a = 5\nif a > 3 then\n  print 1\nend";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "1");
    }

    #[test]
    fn l0_if_else() {
        let src = "a = 2\nif a > 3 then\n  print 1\nelse\n  print 0\nend";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "0");
    }

    #[test]
    fn l0_if_eq_then() {
        let src = "if 5 == 5 then\nprint 42\nend";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "42");
    }

    #[test]
    fn l0_while_loop() {
        let src = "
a = 0
while a < 10 do
  a = a + 1
end
print a";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "10");
    }

    #[test]
    fn l0_while_sum() {
        let src = "
sum = 0
i = 1
while i <= 10 do
  sum = sum + i
  i = i + 1
end
print sum";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "55");
    }

    #[test]
    fn l0_printc() {
        let src = "printc 72";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "H");
    }

    #[test]
    fn l0_factorial() {
        let src = "
n = 5
result = 1
while n > 0 do
  result = result * n
  n = n - 1
end
print result";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "120");
    }

    #[test]
    fn l0_fib() {
        let src = "
n = 10
a = 0
b = 1
i = 0
while i < n do
  t = b
  b = a + b
  a = t
  i = i + 1
end
print a";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "55");
    }

    #[test]
    fn l0_comment() {
        let src = "; full-line comment\na = 99\nprint a ; inline";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "99");
    }

    #[test]
    fn l0_label_jmp() {
        let src = "
a = 0
loop:
a = a + 1
if a == 5 then
  jmp exit
end
jmp loop
exit:
print a";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "5");
    }

    #[test]
    fn l0_jz_jnz() {
        let src = "
a = 0
loop:
jz exit a - 5
a = a + 1
jmp loop
exit:
print a";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "5");
    }

    #[test]
    fn l0_call_ret() {
        let src = "
call double_it
print b
jmp end

double_it:
a = 21
b = a + a
ret

end:
print b";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "4242");
    }

    #[test]
    fn l0_bitwise() {
        let src = "print 12 & 10";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "8");
    }

    #[test]
    fn l0_or() {
        let src = "print 12 | 3";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "15");
    }

    #[test]
    fn l0_xor() {
        let src = "print 15 ^ 5";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "10");
    }

    #[test]
    fn l0_ge_le() {
        let src = "print 5 >= 5\nprint 3 <= 2";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "10");
    }

    #[test]
    fn l0_complex_expr() {
        let src = "print 1 + 2 * 3 + 4";
        let (out, _) = run_l0(src).unwrap();
        assert_eq!(out, "11");
    }

    #[test]
    fn l0_error_dup_label() {
        let src = "x:\nx:\nprint 1";
        assert!(compile_l0(src).is_err());
    }

    #[test]
    fn l0_error_undef_label() {
        let src = "jmp nowhere\nprint 1";
        assert!(compile_l0(src).is_err());
    }

    #[test]
    fn l0_error_syntax() {
        let src = "if print 1";
        assert!(compile_l0(src).is_err());
    }

    // ── 输入/自举: 用 L0 读取输入 ──

    #[test]
    fn l0_input_read_chars() {
        // L0 程序从输入缓冲读三个字符
        let src = "input a\ninput b\ninput c\nprintc a\nprintc b\nprintc c";
        let (out, _) = run_l0_with_input(src, "ABC").unwrap();
        assert_eq!(out, "ABC");
    }

    #[test]
    fn l0_input_eof_is_minus1() {
        // 读完所有字符后再 input 会得到 -1
        let src = "input a\ninput b\ninput c\nprint a\nprint b\nprint c";
        let (out, _) = run_l0_with_input(src, "AB").unwrap();
        assert_eq!(out, "6566-1"); // A=65 B=66 EOF=-1
    }

    #[test]
    fn l0_input_echo_until_eof() {
        // L0 cat 程序: 逐字符回显直到 EOF (-1)
        // Simplified: just read 1 char and echo
        let simple_src = "input c\nif c == -1 then\n  print 0\nelse\n  printc c\nend";
        let (out, _) = run_l0_with_input(simple_src, "X").unwrap();
        assert_eq!(out, "X");
    }

    // ── 自举雏形: 用 L0 处理 L0 ──

    #[test]
    fn l0_self_hosting_char_counter() {
        // L0 字符计数程序: 逐字符读取直到 EOF, 计数并打印
        // 这是编译器雏形——读取源码字符流, 统计 (编译器的第一件事就是 read source)
        let counter_src = r#"
count = 0
loop:
input c
if c == -1 then
  jmp done
end
count = count + 1
jmp loop
done:
print count"#;
        // 输入 5 字符 "hello" → 计数 = 5
        let (out, _) = run_l0_with_input(counter_src, "hello").unwrap();
        assert_eq!(out, "5");
    }

    #[test]
    fn l0_self_hosting_char_counter_empty() {
        // 空输入 → 计数 = 0
        let counter_src = r#"
count = 0
loop:
input c
if c == -1 then
  jmp done
end
count = count + 1
jmp loop
done:
print count"#;
        let (out, _) = run_l0_with_input(counter_src, "").unwrap();
        assert_eq!(out, "0");
    }

    #[test]
    fn l0_self_hosting_echo_program() {
        // L0 echo 直到 EOF: 读取并回显所有字符
        // 演示 L0 能处理和转发任意输入流
        let echo_src = r#"
loop:
input c
if c == -1 then
  jmp done
end
printc c
jmp loop
done:
printc 33"#; // '!' = 33 结束标记

    let (out, _) = run_l0_with_input(echo_src, "knml").unwrap();
    assert_eq!(out, "knml!");
    }

    #[test]
    fn l0_self_hosting_read_and_compile_markers() {
        // 证明: L0 程序能吞吃 L0 源码并解析关键标记
        // 读入一串 "print" 关键字并计数出现多少次 "print" 的 p 字符
        // 这只是 token counting, 但已经是 "compiler frontend" 的雏形

        let tokenizer_demo = r#"
p_count = 0
loop:
input c
if c == -1 then
  jmp done
end
if c == 112 then
  p_count = p_count + 1
end
jmp loop
done:
print p_count"#;

        // "print print" 有 2 个 'p' (实际上 print 有 1 个 p, "printprint" 有 2 个)
        let (out, _) = run_l0_with_input(tokenizer_demo, "printprint").unwrap();
        assert_eq!(out, "2");
    }

    #[test]
    fn l0_self_hosting_can_read_own_source() {
        // 关键: L0 程序读取自己的源码, 验证编译和自读的闭环
        let own_source = r#"count = 0
loop:
input c
if c == -1 then
  jmp done
end
count = count + 1
jmp loop
done:
print count"#;

        // 编译 own_source (确认它是合法 L0)
        let _ = compile_l0(own_source).expect("own_source 应被编译");

        // 运行 own_source, 把 own_source 自身作为输入
        let (out, _) = run_l0_with_input(own_source, own_source).unwrap();
        let count: i32 = out.parse().unwrap();

        // 验证: count == own_source.len()
        assert_eq!(count, own_source.len() as i32);
    }

    // ── L0 Lexer 雏形: 用 L0 程序识别关键字 "print" ──

    #[test]
    fn l0_lexer_recognizes_print_keyword() {
        // 状态机识别 "print" 五个字母 (p-r-i-n-t)
        // 识别成功输出 1, 失败输出 0
        //
        // 这就是编译器的"词法分析器"(Lexer)用 L0 自身写成!
        // Lexer 的本质是一个有限状态机器:
        //   状态0 读'p'→状态1, 读其他→失败
        //   状态1 读'r'→状态2, 读其他→失败
        //   ...直到读完 5 个字母到达接受状态

        let lexer = r#"
; 读第一个字符, 必须是 'p' (ASCII 112)
input c
if c == 112 then
  ; 读第二个字符, 必须是 'r' (ASCII 114)
  input c
  if c == 114 then
    ; 读第三个字符, 必须是 'i' (ASCII 105)
    input c
    if c == 105 then
      ; 读第四个字符, 必须是 'n' (ASCII 110)
      input c
      if c == 110 then
        ; 读第五个字符, 必须是 't' (ASCII 116)
        input c
        if c == 116 then
          print 1
          jmp done
        end
      end
    end
  end
end
; 任何一步不匹配, 走到这里
print 0
done:"#;

        // 输入 "print" → 应识别成功
        let (out, _) = run_l0_with_input(lexer, "print").unwrap();
        assert_eq!(out, "1");

        // 输入 "prin" (缺末字母) → 失败
        let (out, _) = run_l0_with_input(lexer, "prin").unwrap();
        assert_eq!(out, "0");

        // 输入 "pront" (错字母) → 失败
        let (out, _) = run_l0_with_input(lexer, "pront").unwrap();
        assert_eq!(out, "0");
    }

    #[test]
    fn l0_large_i32_constant() {
        // 验证 emit_push_i32 把 >i16::MAX 的常数拆分为 hi:16,lo:16
        // 然后用 PUSH(hi) PUSH(16) SHL PUSH(lo) ADD 装箱回 i32。
        // 注:L0 的 ; 是注释,语句必须用 \n 分隔。

        // 40000 = 0x0000_9C40
        let (out, _) = run_l0("n = 40000\nprint n").unwrap();
        assert_eq!(out, "40000");

        // 100_000 = 0x0001_86A0(hi_adj=2,lo_signed=-31072,SHL+ADD 重组)
        let (out, _) = run_l0("n = 100000\nprint n").unwrap();
        assert_eq!(out, "100000");

        // 32768 = 0x0000_8000(刚好超出 i16::MAX 一字节,hi=0,lo 溢出为负)
        let (out, _) = run_l0("n = 32768\nprint n").unwrap();
        assert_eq!(out, "32768");

        // 32767 = 0x0000_7FFF(i16::MAX 仍走 fast path)
        let (out, _) = run_l0("n = 32767\nprint n").unwrap();
        assert_eq!(out, "32767");

        // 0x0001_0000 = 65536(边界,hi=1,lo=0)
        let (out, _) = run_l0("n = 65536\nprint n").unwrap();
        assert_eq!(out, "65536");

        // 2_000_000 = 0x001E_8480(hi>0,lo>0)
        let (out, _) = run_l0("n = 2000000\nprint n").unwrap();
        assert_eq!(out, "2000000");
    }

    #[test]
    fn l0_lexer_two_keywords_dispatcher() {
        // 调度器: 根据首字符分发到不同的关键字识别
        // 'p' → 尝试 print, 'i' → 尝试 if/input
        //
        // 这是真实编译器的 Lexer 入口:
        //   next_token() {
        //     c = peek_char()
        //     if c == 'p': return try_match_print()
        //     if c == 'i': return try_match_if_or_input()
        //     ...
        //   }

        let dispatcher = r#"
 ; peek first char
input a

 ; a == 112 ('p') === check print?
if a == 112 then
  input b
  if b == 114 then
    print 1
    jmp done
  end
  print 0
  jmp done
end

 ; a == 105 ('i') === check if or input
if a == 105 then
  input b
  if b == 102 then
    print 2
    jmp done
  end
  if b == 110 then
    print 3
    jmp done
  end
  print 0
  jmp done
end

; 不认识的首字符
print 0
done:"#;

        // 'p' 'r' → print (token 1)
        let (out, _) = run_l0_with_input(dispatcher, "pr").unwrap();
        assert_eq!(out, "1");

        // 'i' 'f' → if (token 2)
        let (out, _) = run_l0_with_input(dispatcher, "if").unwrap();
        assert_eq!(out, "2");

        // 'i' 'n' → input (token 3)
        let (out, _) = run_l0_with_input(dispatcher, "in").unwrap();
        assert_eq!(out, "3");

        // 不认识的关键字
        let (out, _) = run_l0_with_input(dispatcher, "xy").unwrap();
        assert_eq!(out, "0");
    }

    /// peek/poke 间接寻址 — 通过 compile_l0 跑通整条管线
    #[test]
    fn l0_peek_compiles_and_runs() {
        // slot 分配按遇到顺序:a=slot0, b=slot1
        // a=99 → mem[0]=99; b=0 → mem[1]=0 (用作 addr 指向 slot a)
        // peek b: LOAD b(slot1)=0, LOAD_IND addr=0 → push mem[0]=99, STORE slot1 → slot1=99
        let src = r#"
a = 99
b = 0
peek b
print b
"#;
        let code = compile_l0(src).unwrap();
        assert!(code.len() > 4, "bytecode 应含 LOAD_IND 指令");
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert_eq!(vm.output, "99", "peek 应间接读回 99,实际 output='{}'", vm.output);
    }

    #[test]
    fn l0_mulconst_immediate() {
        let src = r#"
a = 14
b = a * 3
print b
"#;
        let code = compile_l0(src).unwrap();
        // 先反汇编(code 会被 Vm::new 移动)
        let text = crate::mccp_vm::disassemble(&code);
        assert!(text.contains("MULconst"), "a*3 应编译为 MULconst,disasm={}", text);
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert_eq!(vm.output, "42", "a * 3 应输出 42,实际 output='{}'", vm.output);
    }

    #[test]
    fn l0_addconst_immediate() {
        let src = r#"
a = 100
b = a + 5
print b
"#;
        let code = compile_l0(src).unwrap();
        let text = crate::mccp_vm::disassemble(&code);
        assert!(text.contains("ADDconst"), "a+5 应编译为 ADDconst,disasm={}", text);
        let mut vm = Vm::new(code);
        vm.run().unwrap();
        assert_eq!(vm.output, "105", "a + 5 应输出 105,实际 output='{}'", vm.output);
    }

    #[test]
    fn l0_mulconst_ouroboros_e2e() {
        // 端到端:相同语义源码走完整 Ouroboros 管线
        let src = r#"
a = 3
b = a * 10
print b
"#;
        // 1. compile_l0(Rust 端)
        let bc = compile_l0(src).unwrap();
        let mut vm = Vm::new(bc);
        vm.run().unwrap();
        assert_eq!(vm.output, "30", "Ouroboros 管线应输出 30,实际 output='{}'", vm.output);
    }
}
