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
    self, op, op_imm, OP_ADD, OP_SUB, OP_MUL, OP_DIV, OP_MOD,
    OP_AND, OP_OR, OP_XOR, OP_NOT, OP_SHL, OP_SHR,
    OP_EQ, OP_NE, OP_LT, OP_GT, OP_LE, OP_GE,
    OP_PUSH, OP_POP, OP_JMP, OP_JZ, OP_JNZ, OP_CALL, OP_RET,
    OP_LOAD, OP_STORE, OP_PRINT, OP_PRINTC, OP_SYSCALL, OP_HALT,
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
    Jmp, Jz, JzN, Call, Ret,
    Input,           // 读一个字符
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
                        "INPUT" => Tok::Input,
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
                        "INPUT" => Tok::Input,
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
    Print(Expr),
    PrintC(Expr),
    Input(String),                    // input var - 从输入缓冲读一个字符到变量
    Label(String),
    Jmp(String),
    Jz(String, Expr),
    Jnz(String, Expr),
    Call(String),
    Ret,
    Nop,
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
            Tok::Input => {
                self.advance();
                let var = self.parse_input_var()?;
                Ok(Stmt::Input(var))
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
            Expr::Int(n) => self.emit_imm(OP_PUSH, *n as i16),
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
}
