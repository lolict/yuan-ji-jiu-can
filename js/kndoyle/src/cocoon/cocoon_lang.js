/**
 * ================================================
 * CocoonLang 自举编程语言 - 核心定义
 * ================================================
 *
 * "自举"含义：本语言的编译器/解释器/翻译器均可用本语言自身书写。
 * 当前为 JavaScript 骨架实现，后续可由 CocoonLang 自身源码逐步替换。
 *
 * 语言特性：
 *   - 声母 = 运算操作符（b=加, p=减, m=乘, f=除, d=赋值, t=比较 ...）
 *   - 韵母 = 流程控制    （ai=条件, ei=循环, ui=函数定义, ao=返回 ...）
 *   - 声调 = 通信/IO    （an1=打印, en1=读取, in1=网络 ...）
 *   - 变量名用汉字或符号（如 「数」「甲」「∞」）
 *   - 支持集装箱操作：直接操作进制本身（不是操作编码后的字符）
 *
 * 模块导出：
 *   - Tokenizer / Lexer
 *   - Parser
 *   - AST 节点类型
 *   - CocoonLang (主类)
 */

'use strict';

// ========================
// 一、关键字 / 符号表
// ========================

// 声母 → 运算操作符（直接复用拼音声母字符）
const SHENGMU_OP = {
  'b': 'ADD',      // 加
  'p': 'SUB',      // 减
  'm': 'MUL',      // 乘
  'f': 'DIV',      // 除
  'd': 'ASSIGN',   // 赋值
  't': 'CMP',      // 比较
  'n': 'AND',      // 与
  'l': 'OR',       // 或
  'g': 'GT',       // 大于
  'k': 'LT',       // 小于
  'h': 'HEX_OP',   // 进制运算（集装箱操作）
  'j': 'JOIN',     // 连接
  'q': 'QUERY',    // 查询
  'x': 'XOR',      // 异或
  'z': 'NEG',      // 取负
  'c': 'CAST',     // 类型转换
  's': 'SHIFT',    // 移位
  'r': 'REM',      // 取余
};

// 韵母 → 流程控制（用拼音韵母字符串识别）
const YUNMU_FLOW = {
  'ai':  'IF',       // 条件
  'ei':  'LOOP',     // 循环
  'ui':  'FUNC',     // 函数定义
  'ao':  'RETURN',   // 返回
  'ou':  'ELSE',     // 否则
  'iu':  'BREAK',    // 跳出
  'ie':  'CONTINUE', // 继续
  'an':  'BLOCK',    // 块开始
  'en':  'END',      // 块结束
  'in':  'CALL',     // 调用
  'un':  'WHILE',    // 当循环
  'vn':  'MATCH',    // 模式匹配
  'ang': 'PAR',      // 并行
  'eng': 'SYNC',     // 同步
  'ing': 'ASYNC',    // 异步
  'ong': 'CHANNEL',  // 通道
};

// 声调（数字后缀） → 通信/IO 操作
// 形如 an1 / en2 / in3 ... 由韵母 + 数字声调组合
const SHENGDIAO_IO = {
  'an1': 'PRINT',     // 打印
  'an2': 'PRINTLN',   // 打印换行
  'en1': 'READ',      // 读取
  'en2': 'READLINE',  // 读一行
  'in1': 'NET_SEND',  // 网络发送
  'in2': 'NET_RECV',  // 网络接收
  'in3': 'NET_OPEN',  // 打开连接
  'un1': 'FILE_OPEN', // 文件打开
  'un2': 'FILE_READ', // 文件读
  'un3': 'FILE_WRITE',// 文件写
  'ong1': 'SHELL',    // 壳命令
  'ong2': 'EXEC',     // 执行
};

// 进制（集装箱）定义 —— 直接操作进制本身
const RADIX_TABLE = {
  BIN: 2,
  OCT: 8,
  DEC: 10,
  HEX: 16,
  // CocoonLang 自有进制：声母韵母联合进制
  COCOON_3X: 3,    // 3 进制（蛹茧三态）
  COCOON_24: 24,   // 声母进制
  COCOON_32: 32,   // 韵母进制
};

// ========================
// 二、AST 节点类型
// ========================

// AST 节点基类
class ASTNode {
  constructor(type, line = 0, col = 0) {
    this.type = type;
    this.line = line;
    this.col = col;
  }
}

// 集装箱引用：直接引用一个进制容器（如 0x1F, 0b101, 或声母韵母容器）
class ContainerRef extends ASTNode {
  constructor(containerId, raw, radix, value, line = 0, col = 0) {
    super('ContainerRef', line, col);
    this.containerId = containerId; // 集装箱编号 1/2/3/4
    this.raw = raw;                 // 原始字面量
    this.radix = radix;             // 进制
    this.value = value;             // 解析后的数值
  }
}

// 变量引用
class VariableRef extends ASTNode {
  constructor(name, line = 0, col = 0) {
    super('VariableRef', line, col);
    this.name = name;
  }
}

// 字面量
class Literal extends ASTNode {
  constructor(value, kind, line = 0, col = 0) {
    super('Literal', line, col);
    this.value = value; // 字符串 / 数字 / 布尔
    this.kind = kind;   // 'string' | 'number' | 'boolean'
  }
}

// 运算节点
class Operation extends ASTNode {
  constructor(op, args = [], line = 0, col = 0) {
    super('Operation', line, col);
    this.op = op;         // SHENGMU_OP 的值
    this.args = args;     // 子节点列表
  }
}

// 流程控制节点
class FlowControl extends ASTNode {
  constructor(kind, condition = null, body = [], branches = [], line = 0, col = 0) {
    super('FlowControl', line, col);
    this.kind = kind;        // YUNMU_FLOW 的值
    this.condition = condition;
    this.body = body;
    this.branches = branches; // else 分支等
  }
}

// 通信/IO 节点
class Communication extends ASTNode {
  constructor(op, args = [], line = 0, col = 0) {
    super('Communication', line, col);
    this.op = op;     // SHENGDIAO_IO 的值
    this.args = args;
  }
}

// 壳命令节点
class ShellCommand extends ASTNode {
  constructor(cmd, args = [], line = 0, col = 0) {
    super('ShellCommand', line, col);
    this.cmd = cmd;
    this.args = args;
  }
}

// 函数定义
class FuncDef extends ASTNode {
  constructor(name, params = [], body = [], line = 0, col = 0) {
    super('FuncDef', line, col);
    this.name = name;
    this.params = params;
    this.body = body;
  }
}

// 程序根节点
class Program extends ASTNode {
  constructor(body = []) {
    super('Program');
    this.body = body;
  }
}

// ========================
// 三、Tokenizer / Lexer 词法分析
// ========================

// Token 类型
const TokenType = {
  SHENGMU:   'SHENGMU',    // 声母关键字
  YUNMU:     'YUNMU',      // 韵母关键字
  SHENGDIAO: 'SHENGDIAO',  // 声调 IO 关键字
  IDENT:     'IDENT',      // 汉字/符号变量名
  NUMBER:    'NUMBER',     // 数字字面量（十进制）
  STRING:    'STRING',     // 字符串字面量
  CONTAINER: 'CONTAINER',  // 集装箱字面量（0x / 0b / 0o 前缀）
  LPAREN:    'LPAREN',     // (
  RPAREN:    'RPAREN',     // )
  LBRACKET:  'LBRACKET',   // [
  RBRACKET:  'RBRACKET',   // ]
  LBRACE:    'LBRACE',     // {
  RBRACE:    'RBRACE',     // }
  COMMA:     'COMMA',      // ,
  DOT:       'DOT',        // .
  NEWLINE:   'NEWLINE',
  EOF:       'EOF',
};

class Token {
  constructor(type, value, line = 0, col = 0) {
    this.type = type;
    this.value = value;
    this.line = line;
    this.col = col;
  }
  toString() {
    return `Token(${this.type}, ${JSON.stringify(this.value)}, ${this.line}:${this.col})`;
  }
}

class Tokenizer {
  constructor(source) {
    this.source = String(source);
    this.pos = 0;
    this.line = 1;
    this.col = 1;
    this.tokens = [];
  }

  // 主入口
  tokenize() {
    while (this.pos < this.source.length) {
      const ch = this.source[this.pos];
      // 空白
      if (ch === ' ' || ch === '\t' || ch === '\r') {
        this.advance();
        continue;
      }
      // 注释：以 ; 开头到行尾
      if (ch === ';') {
        while (this.pos < this.source.length && this.source[this.pos] !== '\n') {
          this.advance();
        }
        continue;
      }
      // 换行
      if (ch === '\n') {
        this.push(TokenType.NEWLINE, '\n');
        this.advance();
        this.line++;
        this.col = 1;
        continue;
      }
      // 字符串
      if (ch === '"') {
        this.readString();
        continue;
      }
      // 数字 / 集装箱 / 负数
      if (ch === '0' && (this.peek(1) === 'x' || this.peek(1) === 'b' || this.peek(1) === 'o')) {
        this.readContainer();
        continue;
      }
      if (ch === '-' && this.isDigit(this.peek(1))) {
        // 负号开头的数字（如 -10 / -3.14）
        this.advance(); // 跳过 '-'
        const signStart = this.line, signCol = this.col - 1;
        let num = '-';
        while (this.pos < this.source.length && (this.isDigit(this.source[this.pos]) || this.source[this.pos] === '.')) {
          num += this.source[this.pos];
          this.advance();
        }
        this.tokens.push(new Token(TokenType.NUMBER, parseFloat(num), signStart, signCol));
        continue;
      }
      if (this.isDigit(ch)) {
        this.readNumber();
        continue;
      }
      // 括号 / 分隔符
      if (ch === '(') { this.push(TokenType.LPAREN, ch); this.advance(); continue; }
      if (ch === ')') { this.push(TokenType.RPAREN, ch); this.advance(); continue; }
      if (ch === '[') { this.push(TokenType.LBRACKET, ch); this.advance(); continue; }
      if (ch === ']') { this.push(TokenType.RBRACKET, ch); this.advance(); continue; }
      if (ch === '{') { this.push(TokenType.LBRACE, ch); this.advance(); continue; }
      if (ch === '}') { this.push(TokenType.RBRACE, ch); this.advance(); continue; }
      if (ch === ',') { this.push(TokenType.COMMA, ch); this.advance(); continue; }
      if (ch === '.') { this.push(TokenType.DOT, ch); this.advance(); continue; }

      // 拼音关键字 / 汉字标识符 / 符号标识符
      if (this.isAlpha(ch)) {
        this.readKeywordOrIdent();
        continue;
      }
      // 汉字 / 其他 Unicode 符号作为标识符
      if (this.isCJKOrSymbol(ch)) {
        this.readCJKIdent();
        continue;
      }

      // 未知字符：跳过并记录
      this.advance();
    }
    this.push(TokenType.EOF, null);
    return this.tokens;
  }

  advance() {
    this.pos++;
    this.col++;
  }
  peek(offset = 0) {
    return this.source[this.pos + offset] || '';
  }
  push(type, value) {
    this.tokens.push(new Token(type, value, this.line, this.col));
  }
  isDigit(c) { return c >= '0' && c <= '9'; }
  isAlpha(c) { return /^[a-zA-Z]$/.test(c); }
  isCJKOrSymbol(c) {
    // 汉字、全角符号、数学符号等
    const code = c.charCodeAt(0);
    return (code >= 0x4e00 && code <= 0x9fff)   // CJK 统一汉字
        || (code >= 0x3400 && code <= 0x4dbf)   // CJK 扩展A
        || (code >= 0x3000 && code <= 0x303f)   // CJK 标点
        || (code >= 0x2e80 && code <= 0x2eff)   // CJK 部首补充
        || (code >= 0x2200 && code <= 0x22ff)   // 数学运算符
        || (code >= 0x2600 && code <= 0x26ff);  // 杂项符号
  }

  readString() {
    const startLine = this.line, startCol = this.col;
    this.advance(); // 跳过开头 "
    let val = '';
    while (this.pos < this.source.length && this.source[this.pos] !== '"') {
      if (this.source[this.pos] === '\\') {
        this.advance();
        const esc = this.source[this.pos];
        switch (esc) {
          case 'n': val += '\n'; break;
          case 't': val += '\t'; break;
          case 'r': val += '\r'; break;
          case '\\': val += '\\'; break;
          case '"': val += '"'; break;
          default:  val += esc;
        }
        this.advance();
      } else {
        val += this.source[this.pos];
        if (val[val.length - 1] === '\n') this.line++;
        this.advance();
      }
    }
    this.advance(); // 跳过结尾 "
    this.tokens.push(new Token(TokenType.STRING, val, startLine, startCol));
  }

  readNumber() {
    const startLine = this.line, startCol = this.col;
    let num = '';
    while (this.pos < this.source.length && (this.isDigit(this.source[this.pos]) || this.source[this.pos] === '.')) {
      num += this.source[this.pos];
      this.advance();
    }
    this.tokens.push(new Token(TokenType.NUMBER, parseFloat(num), startLine, startCol));
  }

  // 集装箱字面量：0x.. / 0b.. / 0o.. —— 直接以指定进制表达数值
  readContainer() {
    const startLine = this.line, startCol = this.col;
    const prefix = this.peek(1);
    this.advance(); this.advance(); // 跳过 0 + x/b/o
    let digits = '';
    while (this.pos < this.source.length && /[0-9a-fA-F]/.test(this.source[this.pos])) {
      digits += this.source[this.pos];
      this.advance();
    }
    let radix, containerId, value;
    if (prefix === 'x') { radix = 16; containerId = 3; value = parseInt(digits, 16); }
    else if (prefix === 'b') { radix = 2; containerId = 1; value = parseInt(digits, 2); }
    else { radix = 8; containerId = 2; value = parseInt(digits, 8); }
    this.tokens.push(new Token(TokenType.CONTAINER, { raw: '0' + prefix + digits, radix, containerId, value }, startLine, startCol));
  }

  // 读取拼音关键字（声母/韵母/声调）或拉丁字母标识符
  readKeywordOrIdent() {
    const startLine = this.line, startCol = this.col;
    let word = '';
    while (this.pos < this.source.length && /^[a-zA-Z0-9]$/.test(this.source[this.pos])) {
      word += this.source[this.pos];
      this.advance();
    }
    // 判断类型：声调 IO > 韵母流程 > 声母运算 > 普通标识符
    if (SHENGDIAO_IO[word]) {
      this.push(TokenType.SHENGDIAO, { raw: word, op: SHENGDIAO_IO[word] });
      return;
    }
    if (YUNMU_FLOW[word]) {
      this.push(TokenType.YUNMU, { raw: word, op: YUNMU_FLOW[word] });
      return;
    }
    if (SHENGMU_OP[word]) {
      this.push(TokenType.SHENGMU, { raw: word, op: SHENGMU_OP[word] });
      return;
    }
    this.push(TokenType.IDENT, word);
  }

  // 读取汉字/符号标识符（连续的 CJK 字符合并为一个标识符）
  readCJKIdent() {
    const startLine = this.line, startCol = this.col;
    let word = '';
    while (this.pos < this.source.length && this.isCJKOrSymbol(this.source[this.pos])) {
      word += this.source[this.pos];
      this.advance();
    }
    this.push(TokenType.IDENT, word);
  }
}

// ========================
// 四、Parser 语法分析
// ========================

class Parser {
  constructor(tokens) {
    this.tokens = tokens;
    this.pos = 0;
  }
  peek(offset = 0) { return this.tokens[this.pos + offset] || this.tokens[this.tokens.length - 1]; }
  next() { return this.tokens[this.pos++]; }
  expect(type) {
    const t = this.next();
    if (t.type !== type) {
      throw new Error(`语法错误: 期望 ${type}, 实际 ${t.type}(${JSON.stringify(t.value)}) @ ${t.line}:${t.col}`);
    }
    return t;
  }
  match(type) { return this.peek().type === type; }
  eat(type) {
    if (this.match(type)) { return this.next(); }
    return null;
  }
  skipNewlines() {
    while (this.match(TokenType.NEWLINE)) this.next();
  }

  parse() {
    const body = [];
    this.skipNewlines();
    while (!this.match(TokenType.EOF)) {
      const stmt = this.parseStatement();
      if (stmt) body.push(stmt);
      this.skipNewlines();
    }
    return new Program(body);
  }

  parseStatement() {
    const tk = this.peek();
    switch (tk.type) {
      case TokenType.YUNMU:      return this.parseFlow();
      case TokenType.SHENGDIAO:  return this.parseCommunication();
      case TokenType.SHENGMU:    return this.parseOperationStmt();
      case TokenType.IDENT:      return this.parseIdentStmt();
      case TokenType.LBRACE:     return this.parseBlock();
      default:
        // 跳过无法识别的 token
        this.next();
        return null;
    }
  }

  // 块：{ ... }
  parseBlock() {
    const t = this.expect(TokenType.LBRACE);
    const stmts = [];
    this.skipNewlines();
    while (!this.match(TokenType.RBRACE) && !this.match(TokenType.EOF)) {
      const s = this.parseStatement();
      if (s) stmts.push(s);
      this.skipNewlines();
    }
    this.expect(TokenType.RBRACE);
    return new FlowControl('BLOCK', null, stmts, [], t.line, t.col);
  }

  // 流程控制：ai/ei/ui/ao/...
  parseFlow() {
    const t = this.next(); // YUNMU token
    const op = t.value.op;
    let condition = null;
    let body = [];
    let branches = [];

    // IF / LOOP / WHILE 等带条件
    // 支持两种条件写法：
    //   1) 圆括号风格: ai (甲>0) { ... }
    //   2) 空格前缀风格: ai 甲 { ... }
    if (['IF', 'LOOP', 'WHILE', 'MATCH', 'PAR', 'SYNC'].includes(op)) {
      this.skipNewlines();
      if (this.match(TokenType.LPAREN)) {
        this.next();
        condition = this.parseExpr();
        this.expect(TokenType.RPAREN);
      } else {
        // 非括号风格：读取一个表达式作为条件，直到遇到 { 或 语句边界
        if (!this.match(TokenType.LBRACE) &&
            !this.match(TokenType.NEWLINE) &&
            !this.match(TokenType.EOF) &&
            !this.match(TokenType.RBRACE)) {
          condition = this.parseExpr();
        }
      }
    }
    // 函数定义 ui 名称(参数...) { body }
    // 支持两种形参风格：
    //   1) ui 名称(甲, 乙, 丙) { ... }   —— 圆括号风格
    //   2) ui 名称 甲 乙 丙 { ... }      —— 空格风格（波兰/前缀风格）
    if (op === 'FUNC') {
      const nameTk = this.eat(TokenType.IDENT);
      const name = nameTk ? nameTk.value : '__anon__';
      const params = [];
      this.skipNewlines();
      if (this.match(TokenType.LPAREN)) {
        // 圆括号风格
        this.next();
        while (!this.match(TokenType.RPAREN)) {
          if (this.match(TokenType.IDENT)) params.push(this.next().value);
          if (!this.match(TokenType.RPAREN)) this.eat(TokenType.COMMA);
        }
        this.expect(TokenType.RPAREN);
      } else {
        // 空格风格：连续读取 IDENT 直到遇到 { 或其他边界
        while (this.match(TokenType.IDENT)) {
          params.push(this.next().value);
          this.skipNewlines();
        }
      }
      this.skipNewlines();
      if (this.match(TokenType.LBRACE)) body = this.parseBlock().body;
      return new FuncDef(name, params, body, t.line, t.col);
    }
    // RETURN ao <expr>
    if (op === 'RETURN' || op === 'BREAK' || op === 'CONTINUE') {
      if (!this.match(TokenType.NEWLINE) && !this.match(TokenType.RBRACE) && !this.match(TokenType.EOF)) {
        condition = this.parseExpr();
      }
      return new FlowControl(op, condition, [], [], t.line, t.col);
    }
    // ELSE / END 不带条件
    if (this.match(TokenType.LBRACE)) {
      body = this.parseBlock().body;
    } else {
      // 单语句体
      const s = this.parseStatement();
      if (s) body = [s];
    }
    const node = new FlowControl(op, condition, body, branches, t.line, t.col);
    // IF：吸进紧随其后的 ELSE / ELSE IF 作为 branches
    if (op === 'IF') {
      while (true) {
        this.skipNewlines();
        const ahead = this.peek();
        if (ahead.type === TokenType.YUNMU &&
            (ahead.value && ahead.value.op === 'ELSE')) {
          this.next(); // consume the ELSE YUNMU token
          let elseBody = [];
          // 如果是 ELSE IF (op=IF 嵌套在 ELSE 中)，就递归解析 parseFlow 作为分支
          this.skipNewlines();
          // 先看 ELSE 后面是不是直接 IF（ELSE IF 链）
          const ahead2 = this.peek();
          if (ahead2.type === TokenType.YUNMU &&
              (ahead2.value && ahead2.value.op === 'IF')) {
            // ELSE IF：把整条 IF 语句作为一个 branch（包装成 FlowControl ELSE 或直接塞入 IF 分支链）
            // 递归调用 parseFlow 来解析 IF
            const innerIf = this.parseFlow();
            node.branches.push(innerIf);
          } else {
            // 普通 ELSE
            if (this.match(TokenType.LBRACE)) {
              elseBody = this.parseBlock().body;
            } else {
              const s2 = this.parseStatement();
              if (s2) elseBody = [s2];
            }
            node.branches.push(new FlowControl('ELSE', null, elseBody, [], ahead.line, ahead.col));
          }
        } else {
          break;
        }
      }
    }
    return node;
  }

  // 通信/IO：an1(参数) / en1 / in1 ...
  parseCommunication() {
    const t = this.next();
    const args = [];
    if (this.match(TokenType.LPAREN)) {
      // 1) 圆括号风格: an1(甲, 乙)
      this.next();
      while (!this.match(TokenType.RPAREN) && !this.match(TokenType.EOF)) {
        args.push(this.parseExpr());
        if (!this.match(TokenType.RPAREN)) this.eat(TokenType.COMMA);
      }
      this.expect(TokenType.RPAREN);
    } else {
      // 2) 空格前缀风格: an1 甲 乙  (直到换行/EOF/逗号/括号)
      while (true) {
        const ahead = this.peek();
        if (ahead.type === TokenType.EOF ||
            ahead.type === TokenType.NEWLINE ||
            ahead.type === TokenType.RPAREN ||
            ahead.type === TokenType.LPAREN ||
            ahead.type === TokenType.RBRACE ||
            ahead.type === TokenType.LBRACE ||
            ahead.type === TokenType.COMMA ||
            args.length >= 8) {
          break;
        }
        args.push(this.parsePrimary());
      }
    }
    return new Communication(t.value.op, args, t.line, t.col);
  }

  // 声母开头的运算语句：前缀表达式，如 b 甲 乙 (甲+乙) / d 甲 10 (赋值)
  parseOperationStmt() {
    return this.parseExpr();
  }

  // 标识符开头的语句：
  //   1) 「函数名(实参列表)」 → CALL（圆括号风格，由 parsePrimary 处理）
  //   2) 「函数名 实参 实参 ...」（本行到换行符的空格分隔表达式）→ CALL（空格/前缀风格）
  //   3) 单纯变量名 → VariableRef
  parseIdentStmt() {
    const start = this.peek();
    if (start.type !== TokenType.IDENT) return this.parseExpr();
    this.next();
    const name = start.value;
    // 圆括号风格：ident(args...)
    if (this.match(TokenType.LPAREN)) {
      this.next();
      const args = [];
      while (!this.match(TokenType.RPAREN) && !this.match(TokenType.EOF)) {
        args.push(this.parseExpr());
        if (!this.match(TokenType.RPAREN)) this.eat(TokenType.COMMA);
      }
      this.expect(TokenType.RPAREN);
      return new FlowControl('CALL', { name, args }, [], [], start.line, start.col);
    }
    // 空格风格：ident 后面的本行内，逐个 parsePrimary，直到换行/EOF/花括号/逗号
    const args = [];
    while (true) {
      const ahead = this.peek();
      if (ahead.type === TokenType.EOF ||
          ahead.type === TokenType.NEWLINE ||
          ahead.type === TokenType.RPAREN ||
          ahead.type === TokenType.RBRACE ||
          ahead.type === TokenType.LBRACE ||
          ahead.type === TokenType.COMMA ||
          args.length >= 8) {
        break;
      }
      args.push(this.parsePrimary());
    }
    if (args.length === 0) {
      return new VariableRef(name, start.line, start.col);
    }
    return new FlowControl('CALL', { name, args }, [], [], start.line, start.col);
  }

  // 表达式：前缀（波兰）记法 —— 运算符在前，操作数在后
  // 例：b 甲 5  => ADD(甲, 5)
  //     m 甲 甲 => MUL(甲, 甲)
  //     b m 甲 甲 5 => ADD(MUL(甲,甲), 5)
  parseExpr() {
    return this.parsePrimary();
  }

  // 运算符元数（参数个数）
  _opArity(op) {
    // 赋值/比较/四则：默认二元
    // 特别：ASSIGN (d 变量 右值) —— 若右值是标识符，则尝试空格风格调用（让 d 净值 净利润 白码损益 黑码损益 红码损益 正常工作）
    // 一元运算
    if (op === 'NEG') return 1;
    return 2;
  }

  parsePrimary() {
    const tk = this.peek();
    switch (tk.type) {
      case TokenType.NUMBER: {
        this.next();
        return new Literal(tk.value, 'number', tk.line, tk.col);
      }
      case TokenType.STRING: {
        this.next();
        return new Literal(tk.value, 'string', tk.line, tk.col);
      }
      case TokenType.CONTAINER: {
        this.next();
        return new ContainerRef(tk.value.containerId, tk.value.raw, tk.value.radix, tk.value.value, tk.line, tk.col);
      }
      case TokenType.IDENT: {
        this.next();
        const identName = tk.value;
        // 调用： ident(args) —— 圆括号风格
        if (this.match(TokenType.LPAREN)) {
          this.next();
          const args = [];
          while (!this.match(TokenType.RPAREN) && !this.match(TokenType.EOF)) {
            args.push(this.parseExpr());
            if (!this.match(TokenType.RPAREN)) this.eat(TokenType.COMMA);
          }
          this.expect(TokenType.RPAREN);
          return new FlowControl('CALL', { name: identName, args }, [], [], tk.line, tk.col);
        }
        // 空格风格调用：仅当 parseIdentStmt() 处于「语句起始位置」上下文时生效，
        // 即 SHENGMU 处理完的 ASSIGN 第二个操作数（右值）位置，由 SHENGMU 专用 parseOperationStmt 调用
        // 此时应当只接受一个表达式作为单纯引用（否则 d 结果 1 中的「结果 1」会被误识别成 CALL(结果, 1)）
        return new VariableRef(identName, tk.line, tk.col);
      }
      case TokenType.SHENGMU: {
        // 前缀运算符：根据元数读取 N 个操作数
        this.next();
        const op = tk.value.op;
        const line = tk.line, col = tk.col;
        const arity = this._opArity(op);
        const args = [];

        // 特别处理 ASSIGN: d 目标变量 函数名 实参1 实参2 ... 实参N
        // 即 ASSIGN 的第二个操作数「可能是以空格分隔的函数调用」
        // 仅当 arity==2 且 op==ASSIGN，并且第二个操作数位置（第一个 parsePrimary 之后）
        //   - 第二个操作数 peek 不是换行/EOF/花括号/逗号
        //   - 且第一个操作数是变量（ VariableRef ）
        //   - 且第二个操作数首 token 是 IDENT （非圆括号 call），并且后面紧跟更多表达式 token，
        //     此时触发 ASSIGN 扩展：先读取 VariableRef(目标)，再用 parseIdentStmt 风格解析空格调用
        if (op === 'ASSIGN' && arity === 2) {
          const target = this.parsePrimary(); // 第一个参数：目标变量
          args.push(target);
          // 判断：下一个 token 是否 IDENT，且再之后（跳过空）仍不是换行/结束？
          const ahead = this.peek();
          const ahead2 = this.peek(1);
          if (ahead.type === TokenType.IDENT &&
              ahead2.type !== TokenType.NEWLINE &&
              ahead2.type !== TokenType.EOF &&
              ahead2.type !== TokenType.RBRACE &&
              ahead2.type !== TokenType.RPAREN &&
              ahead2.type !== TokenType.COMMA) {
            // 以 parseIdentStmt 的方式解析：ident 后接多个空格实参 → CALL
            const callStart = this.peek();
            this.next();
            const fname = callStart.value;
            const callArgs = [];
            while (true) {
              const look = this.peek();
              if (look.type === TokenType.EOF ||
                  look.type === TokenType.NEWLINE ||
                  look.type === TokenType.RBRACE ||
                  look.type === TokenType.LBRACE ||
                  look.type === TokenType.RPAREN ||
                  look.type === TokenType.LPAREN ||
                  look.type === TokenType.COMMA ||
                  callArgs.length >= 8) break;
              callArgs.push(this.parsePrimary());
            }
            const callNode = new FlowControl('CALL',
              { name: fname, args: callArgs }, [], [], callStart.line, callStart.col);
            args.push(callNode);
            return new Operation('ASSIGN', args, line, col);
          }
          // 否则回退到普通二元 ASSIGN
          args.push(this.parsePrimary());
          return new Operation('ASSIGN', args, line, col);
        }

        for (let i = 0; i < arity; i++) {
          args.push(this.parsePrimary());
        }
        return new Operation(op, args, line, col);
      }
      case TokenType.LPAREN: {
        this.next();
        const e = this.parseExpr();
        this.expect(TokenType.RPAREN);
        return e;
      }
      case TokenType.LBRACKET: {
        // 集装箱索引访问 [containerId, index]
        this.next();
        const cid = this.parsePrimary();
        this.eat(TokenType.COMMA);
        const idx = this.parsePrimary();
        this.expect(TokenType.RBRACKET);
        return new Operation('QUERY', [cid, idx], tk.line, tk.col);
      }
      default:
        this.next();
        return new Literal(null, 'null', tk.line, tk.col);
    }
  }
}

// ========================
// 五、字节码定义 (CocoonVM 指令)
// ========================

const OpCode = {
  PUSH:    'PUSH',     // 压栈常量
  LOAD:    'LOAD',     // 载入变量
  STORE:   'STORE',    // 存储变量
  ADD:     'ADD',
  SUB:     'SUB',
  MUL:     'MUL',
  DIV:     'DIV',
  REM:     'REM',
  CMP:     'CMP',
  GT:      'GT',
  LT:      'LT',
  AND:     'AND',
  OR:      'OR',
  XOR:     'XOR',
  NEG:     'NEG',
  CAST:    'CAST',
  JOIN:    'JOIN',
  SHIFT:   'SHIFT',
  JUMP:    'JUMP',
  JUMPF:   'JUMPF',    // 条件假跳转
  CALL:    'CALL',
  RET:     'RET',
  PRINT:   'PRINT',
  PRINTLN: 'PRINTLN',
  READ:    'READ',
  READLINE:'READLINE',
  NET_SEND:'NET_SEND',
  NET_RECV:'NET_RECV',
  FILE_OPEN:'FILE_OPEN',
  FILE_READ:'FILE_READ',
  FILE_WRITE:'FILE_WRITE',
  SHELL:   'SHELL',
  EXEC:    'EXEC',
  HEX_OP:  'HEX_OP',   // 进制运算
  QUERY:   'QUERY',
  HALT:    'HALT',
  POP:     'POP',
  DUP:     'DUP',
};

// ========================
// 六、CocoonLang 主类
// ========================

class CocoonLang {
  constructor() {
    this.version = '0.1.0-bootstrap';
    this.name = 'CocoonLang';
    // 内置函数表
    this.builtins = {
      'len':   (x) => (x && x.length !== undefined) ? x.length : 0,
      'str':   (x) => String(x),
      'num':   (x) => Number(x),
      'bool':  (x) => Boolean(x),
      'hex':   (x) => '0x' + (Number(x) >>> 0).toString(16),
      'bin':   (x) => '0b' + (Number(x) >>> 0).toString(2),
      'oct':   (x) => '0o' + (Number(x) >>> 0).toString(8),
      'radix': (x, r) => (Number(x) >>> 0).toString(Number(r)),
    };
  }

  // 获取 AST
  getAST(source) {
    const tokens = new Tokenizer(source).tokenize();
    return new Parser(tokens).parse();
  }

  // 编译：源代码 → 字节码
  compile(source) {
    const ast = this.getAST(source);
    return this._compileNode(ast, { code: [], consts: [], names: [], jumps: [] }).code;
  }

  // 递归编译 AST → 字节码
  _compileNode(node, ctx) {
    if (!node) return ctx;
    switch (node.type) {
      case 'Program': {
        for (const s of node.body) this._compileNode(s, ctx);
        ctx.code.push({ op: OpCode.HALT });
        return ctx;
      }
      case 'Literal': {
        const idx = ctx.consts.push(node.value) - 1;
        ctx.code.push({ op: OpCode.PUSH, arg: idx });
        return ctx;
      }
      case 'ContainerRef': {
        // 集装箱字面量：直接压值（同时记录进制信息）
        const idx = ctx.consts.push(node.value) - 1;
        ctx.code.push({ op: OpCode.PUSH, arg: idx, radix: node.radix, containerId: node.containerId });
        return ctx;
      }
      case 'VariableRef': {
        const idx = ctx.names.indexOf(node.name);
        const nameIdx = idx >= 0 ? idx : ctx.names.push(node.name) - 1;
        ctx.code.push({ op: OpCode.LOAD, arg: nameIdx });
        return ctx;
      }
      case 'Operation': {
        // 先编译参数，再发运算指令
        for (const a of node.args) this._compileNode(a, ctx);
        // 赋值特殊：左侧必须是变量
        if (node.op === 'ASSIGN') {
          // 最后一个参数应是 VariableRef，已编译为 LOAD；改为 STORE
          // 简化处理：弹出最后一个 LOAD 改为 STORE
          const last = ctx.code[ctx.code.length - 1];
          if (last && last.op === OpCode.LOAD) {
            last.op = OpCode.STORE;
          } else {
            ctx.code.push({ op: OpCode.STORE, arg: 0 });
          }
          return ctx;
        }
        const opMap = {
          ADD: OpCode.ADD, SUB: OpCode.SUB, MUL: OpCode.MUL, DIV: OpCode.DIV,
          CMP: OpCode.CMP, GT: OpCode.GT, LT: OpCode.LT, AND: OpCode.AND,
          OR: OpCode.OR, XOR: OpCode.XOR, NEG: OpCode.NEG, CAST: OpCode.CAST,
          JOIN: OpCode.JOIN, SHIFT: OpCode.SHIFT, REM: OpCode.REM,
          HEX_OP: OpCode.HEX_OP, QUERY: OpCode.QUERY,
        };
        ctx.code.push({ op: opMap[node.op] || OpCode.POP });
        return ctx;
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          // 函数调用
          const cond = node.condition || {};
          for (const a of (cond.args || [])) this._compileNode(a, ctx);
          const nameIdx = ctx.names.push(cond.name) - 1;
          ctx.code.push({ op: OpCode.CALL, arg: nameIdx, argc: (cond.args || []).length });
          return ctx;
        }
        if (node.kind === 'IF') {
          this._compileNode(node.condition, ctx);
          const jmpfIdx = ctx.code.length;
          ctx.code.push({ op: OpCode.JUMPF, arg: -1 }); // 待回填
          for (const s of node.body) this._compileNode(s, ctx);
          ctx.code[jmpfIdx].arg = ctx.code.length;
          return ctx;
        }
        if (node.kind === 'LOOP' || node.kind === 'WHILE') {
          const startIdx = ctx.code.length;
          this._compileNode(node.condition, ctx);
          const jmpfIdx = ctx.code.length;
          ctx.code.push({ op: OpCode.JUMPF, arg: -1 });
          for (const s of node.body) this._compileNode(s, ctx);
          ctx.code.push({ op: OpCode.JUMP, arg: startIdx });
          ctx.code[jmpfIdx].arg = ctx.code.length;
          return ctx;
        }
        if (node.kind === 'RETURN') {
          this._compileNode(node.condition, ctx);
          ctx.code.push({ op: OpCode.RET });
          return ctx;
        }
        if (node.kind === 'BREAK') {
          ctx.code.push({ op: OpCode.JUMP, arg: -1, _break: true });
          return ctx;
        }
        if (node.kind === 'CONTINUE') {
          ctx.code.push({ op: OpCode.JUMP, arg: -1, _continue: true });
          return ctx;
        }
        if (node.kind === 'BLOCK') {
          for (const s of node.body) this._compileNode(s, ctx);
          return ctx;
        }
        // 其他流程控制：编译体即可
        for (const s of node.body) this._compileNode(s, ctx);
        return ctx;
      }
      case 'FuncDef': {
        const nameIdx = ctx.names.push(node.name) - 1;
        const defIdx = ctx.code.length;
        // 跳过函数体，先继续编译外层
        const skipJumpIdx = ctx.code.length;
        ctx.code.push({ op: OpCode.JUMP, arg: -1 });
        const bodyStart = ctx.code.length;
        for (const s of node.body) this._compileNode(s, ctx);
        ctx.code.push({ op: OpCode.RET });
        ctx.code[skipJumpIdx].arg = ctx.code.length;
        // 注册函数入口
        ctx.code.splice(defIdx, 0, { op: 'FUNC_DEF', arg: nameIdx, bodyStart, params: node.params });
        return ctx;
      }
      case 'Communication': {
        for (const a of node.args) this._compileNode(a, ctx);
        const ioMap = {
          PRINT: OpCode.PRINT, PRINTLN: OpCode.PRINTLN,
          READ: OpCode.READ, READLINE: OpCode.READLINE,
          NET_SEND: OpCode.NET_SEND, NET_RECV: OpCode.NET_RECV,
          FILE_OPEN: OpCode.FILE_OPEN, FILE_READ: OpCode.FILE_READ, FILE_WRITE: OpCode.FILE_WRITE,
          SHELL: OpCode.SHELL, EXEC: OpCode.EXEC,
        };
        ctx.code.push({ op: ioMap[node.op] || OpCode.POP, argc: node.args.length });
        return ctx;
      }
      case 'ShellCommand': {
        const nameIdx = ctx.names.push(node.cmd) - 1;
        ctx.code.push({ op: OpCode.SHELL, arg: nameIdx, args: node.args });
        return ctx;
      }
      default:
        return ctx;
    }
  }

  // 直接解释执行源代码
  interpret(source, env = {}) {
    const ast = this.getAST(source);
    return this._interpretAST(ast, env);
  }

  // 树遍历解释器
  _interpretAST(node, env) {
    if (!node) return null;
    switch (node.type) {
      case 'Program': {
        let last = null;
        for (const s of node.body) last = this._interpretAST(s, env);
        return last;
      }
      case 'Literal': return node.value;
      case 'ContainerRef': return node.value;
      case 'VariableRef': return env[node.name];
      case 'Operation': {
        const vals = node.args.map(a => this._interpretAST(a, env));
        switch (node.op) {
          case 'ADD':  return vals[0] + vals[1];
          case 'SUB':  return vals[0] - vals[1];
          case 'MUL':  return vals[0] * vals[1];
          case 'DIV':  return vals[0] / vals[1];
          case 'REM':  return vals[0] % vals[1];
          case 'CMP':  return vals[0] === vals[1];
          case 'GT':   return vals[0] > vals[1];
          case 'LT':   return vals[0] < vals[1];
          case 'AND':  return vals[0] && vals[1];
          case 'OR':   return vals[0] || vals[1];
          case 'XOR':  return vals[0] ^ vals[1];
          case 'NEG':  return -vals[0];
          case 'ASSIGN': {
            // 左侧应是 VariableRef
            const target = node.args[0];
            if (target.type === 'VariableRef') {
              env[target.name] = vals[1];
              return env[target.name];
            }
            return vals[1];
          }
          case 'CAST':  return this._cast(vals[0], vals[1]);
          case 'JOIN':  return String(vals[0]) + String(vals[1]);
          case 'SHIFT': return vals[0] << vals[1];
          case 'HEX_OP': {
            // 集装箱操作：直接对进制进行运算
            // vals[0]=数值, vals[1]=目标进制
            const r = Number(vals[1]) || 10;
            return (Number(vals[0]) >>> 0).toString(r);
          }
          case 'QUERY': {
            // 集装箱查询：返回 vals[0] 在 vals[1] 位置上的"位"
            return (Number(vals[0]) >> Number(vals[1])) & 1;
          }
          default: return null;
        }
      }
      case 'FlowControl': {
        switch (node.kind) {
          case 'IF': {
            if (this._interpretAST(node.condition, env)) {
              return this._interpretBlock(node.body, env);
            }
            for (const br of node.branches) {
              if (this._interpretAST(br, env) !== null) return null;
            }
            return null;
          }
          case 'LOOP': {
            // LOOP 是 do-while 风格：先执行体，再判断
            let last = null;
            do {
              last = this._interpretBlock(node.body, env);
            } while (this._interpretAST(node.condition, env));
            return last;
          }
          case 'WHILE': {
            let last = null;
            while (this._interpretAST(node.condition, env)) {
              last = this._interpretBlock(node.body, env);
            }
            return last;
          }
          case 'BLOCK': return this._interpretBlock(node.body, env);
          case 'CALL': {
            const cond = node.condition || {};
            const args = (cond.args || []).map(a => this._interpretAST(a, env));
            return this._callFunction(cond.name, args, env);
          }
          case 'RETURN':
            return { __return: true, value: this._interpretAST(node.condition, env) };
          case 'BREAK':    return { __break: true };
          case 'CONTINUE': return { __continue: true };
          default:
            return this._interpretBlock(node.body, env);
        }
      }
      case 'FuncDef': {
        env[node.name] = { __func: true, params: node.params, body: node.body, closure: env };
        return null;
      }
      case 'Communication': {
        const vals = node.args.map(a => this._interpretAST(a, env));
        switch (node.op) {
          case 'PRINT':   process.stdout.write(vals.map(v => this._format(v)).join('')); return null;
          case 'PRINTLN': console.log(vals.map(v => this._format(v)).join('')); return null;
          case 'READ':    return null; // 同步读取需要宿主提供
          case 'READLINE':return null;
          case 'NET_SEND':return this._netSend(vals);
          case 'NET_RECV':return this._netRecv(vals);
          case 'SHELL':   return this._shell(vals);
          case 'EXEC':    return this._exec(vals);
          default: return null;
        }
      }
      case 'ShellCommand': {
        return this._shell([node.cmd, ...node.args]);
      }
      default: return null;
    }
  }

  _interpretBlock(stmts, env) {
    let last = null;
    for (const s of stmts) {
      const r = this._interpretAST(s, env);
      if (r && (r.__return || r.__break || r.__continue)) return r;
      last = r;
    }
    return last;
  }

  _callFunction(name, args, env) {
    // 内置函数
    if (this.builtins[name]) return this.builtins[name](...args);
    const fn = env[name];
    if (!fn || !fn.__func) {
      throw new Error(`未定义的函数: ${name}`);
    }
    const localEnv = Object.create(fn.closure);
    fn.params.forEach((p, i) => { localEnv[p] = args[i]; });
    const r = this._interpretBlock(fn.body, localEnv);
    return (r && r.__return) ? r.value : null;
  }

  _cast(value, target) {
    switch (String(target)) {
      case 'num': case 'number': return Number(value);
      case 'str': case 'string': return String(value);
      case 'bool': case 'boolean': return Boolean(value);
      default: return value;
    }
  }

  _format(v) {
    if (v === null || v === undefined) return '';
    if (typeof v === 'object') return JSON.stringify(v);
    return String(v);
  }

  _netSend(vals) {
    // 网络发送骨架（同步环境只返回意图）
    return { sent: true, payload: vals };
  }
  _netRecv(vals) {
    return { received: null, from: vals[0] };
  }
  _shell(vals) {
    // 壳命令骨架：不真正执行，返回结构
    return { shell: true, cmd: vals[0], args: vals.slice(1) };
  }
  _exec(vals) {
    return { exec: true, cmd: vals[0] };
  }

  // 集装箱操作辅助：进制转换
  static toRadix(value, radix) {
    return (Number(value) >>> 0).toString(Number(radix));
  }
  static fromRadix(str, radix) {
    return parseInt(String(str), Number(radix));
  }
}

// ========================
// 七、模块导出
// ========================

module.exports = {
  // 关键字表
  SHENGMU_OP, YUNMU_FLOW, SHENGDIAO_IO, RADIX_TABLE,
  // AST 节点
  ASTNode, ContainerRef, VariableRef, Literal, Operation,
  FlowControl, Communication, ShellCommand, FuncDef, Program,
  // 词法
  Tokenizer, Token, TokenType,
  // 语法
  Parser,
  // 字节码
  OpCode,
  // 主类
  CocoonLang,
};
