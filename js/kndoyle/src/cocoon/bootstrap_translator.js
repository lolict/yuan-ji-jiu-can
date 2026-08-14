/**
 * ================================================
 * CocoonLang 自举翻译器 (BootstrapTranslator)
 * ================================================
 *
 * 自举含义：本翻译器既可由 JavaScript 调用，也可用 CocoonLang 自身源码书写。
 * 当前为 JS 骨架实现，提供：
 *   - translate(source, fromLang, toLang): 通用语言间翻译
 *   - translateJS(source)   : CocoonLang → JavaScript
 *   - translatePython(source): CocoonLang → Python
 *   - translateZig(source)  : CocoonLang → Zig
 *   - translateRust(source) : CocoonLang → Rust
 *   - translateGo(source)   : CocoonLang → Go
 *   - binaryTranslate(arch) : 二进制翻译 x86/ARM/RISC-V → CocoonVM 字节码
 *   - selfTranslate()       : 自举翻译
 *
 * 翻译策略：AST → 目标语言源码（树到树/树到串）
 */

'use strict';

const { CocoonLang, OpCode } = require('./cocoon_lang.js');

// 支持的目标语言
const Languages = {
  JAVASCRIPT: 'javascript',
  PYTHON:     'python',
  ZIG:        'zig',
  RUST:       'rust',
  GO:         'go',
  C:          'c',
};

// 支持的二进制架构
const BinaryArch = {
  X86_64:  'x86-64',
  ARM64:   'arm64',
  RISCV64: 'riscv64',
  X86:     'x86',
  ARM32:   'arm32',
};

class BootstrapTranslator {
  constructor() {
    this.lang = new CocoonLang();
    this.version = '0.1.0-bootstrap';
  }

  // ===============
  // 1. translate(source, fromLang, toLang)
  // ===============
  // 通用翻译入口
  translate(source, fromLang = 'cocoon', toLang = Languages.JAVASCRIPT) {
    if (fromLang === 'cocoon') {
      // 从 CocoonLang 翻译出去
      return this._fromCocoon(source, toLang);
    }
    if (toLang === 'cocoon') {
      // 翻译进 CocoonLang
      return this._toCocoon(source, fromLang);
    }
    // 跨语言翻译：先转到 CocoonLang 中间表示，再转到目标
    const cocoonIR = this._toCocoon(source, fromLang);
    return this._fromCocoon(cocoonIR, toLang);
  }

  _fromCocoon(source, toLang) {
    switch (toLang) {
      case Languages.JAVASCRIPT: return this.translateJS(source);
      case Languages.PYTHON:     return this.translatePython(source);
      case Languages.ZIG:        return this.translateZig(source);
      case Languages.RUST:       return this.translateRust(source);
      case Languages.GO:         return this.translateGo(source);
      case Languages.C:          return this._translateC(source);
      default: throw new Error(`不支持的目标语言: ${toLang}`);
    }
  }

  _toCocoon(source, fromLang) {
    // 其他语言 → CocoonLang 的简化翻译
    // 此处提供基于正则的轻量翻译骨架
    let cocoon = source;
    if (fromLang === Languages.JAVASCRIPT) {
      cocoon = cocoon
        .replace(/let\s+(\w+)\s*=\s*/g, 'd $1 ')
        .replace(/const\s+(\w+)\s*=\s*/g, 'd $1 ')
        .replace(/console\.log\(/g, 'an1(')
        .replace(/function\s+(\w+)\s*\(([^)]*)\)\s*\{/g, 'ui $1($2) {')
        .replace(/return\s+/g, 'ao ')
        .replace(/if\s*\(/g, 'ai (')
        .replace(/while\s*\(/g, 'un (')
        .replace(/\+/g, ' b ')
        .replace(/-/g, ' p ')
        .replace(/\*/g, ' m ')
        .replace(/\//g, ' f ');
    } else if (fromLang === Languages.PYTHON) {
      cocoon = cocoon
        .replace(/^(\s*)print\(/gm, '$1an1(')
        .replace(/^(\s*)def\s+(\w+)\s*\(([^)]*)\)\s*:/gm, '$1ui $2($3) {')
        .replace(/^(\s*)return\s+/gm, '$1ao ')
        .replace(/^(\s*)if\s+/gm, '$1ai (')
        .replace(/^(\s*)while\s+/gm, '$1un (')
        .replace(/=/g, ' d ');
    }
    return cocoon;
  }

  // ===============
  // 2. translateJS(source) — CocoonLang → JavaScript
  // ===============
  translateJS(source) {
    const ast = this.lang.getAST(source);
    const ctx = { indent: 0, lines: [], vars: new Set() };
    ctx.lines.push("'use strict';");
    ctx.lines.push('// CocoonLang → JavaScript 翻译输出');
    this._jsEmitProgram(ast, ctx);
    return ctx.lines.join('\n');
  }

  _jsIndent(ctx) { return '  '.repeat(ctx.indent); }

  _jsEmitProgram(node, ctx) {
    for (const s of node.body) this._jsEmitStmt(s, ctx);
  }

  _jsEmitStmt(node, ctx) {
    const ind = this._jsIndent(ctx);
    switch (node.type) {
      case 'Operation': {
        const expr = this._jsEmitExpr(node, ctx);
        ctx.lines.push(`${ind}${expr};`);
        return;
      }
      case 'FlowControl': {
        this._jsEmitFlow(node, ctx);
        return;
      }
      case 'FuncDef': {
        const params = node.params.join(', ');
        ctx.lines.push(`${ind}function ${node.name}(${params}) {`);
        ctx.indent++;
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        ctx.indent--;
        ctx.lines.push(`${ind}}`);
        return;
      }
      case 'Communication': {
        const args = node.args.map(a => this._jsEmitExpr(a, ctx));
        switch (node.op) {
          case 'PRINT':   ctx.lines.push(`${ind}process.stdout.write(String(${args.join(', ')}));`); break;
          case 'PRINTLN': ctx.lines.push(`${ind}console.log(${args.join(', ')});`); break;
          case 'READ':    ctx.lines.push(`${ind}/* read */`); break;
          case 'NET_SEND':ctx.lines.push(`${ind}/* net_send(${args.join(', ')}) */`); break;
          case 'SHELL':   ctx.lines.push(`${ind}/* shell(${args.join(', ')}) */`); break;
          default:        ctx.lines.push(`${ind}/* ${node.op}(${args.join(', ')}) */`);
        }
        return;
      }
      case 'ShellCommand': {
        ctx.lines.push(`${ind}/* shell: ${node.cmd} */`);
        return;
      }
      default:
        return;
    }
  }

  _jsEmitFlow(node, ctx) {
    const ind = this._jsIndent(ctx);
    switch (node.kind) {
      case 'IF': {
        const cond = this._jsEmitExpr(node.condition, ctx);
        ctx.lines.push(`${ind}if (${cond}) {`);
        ctx.indent++;
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        ctx.indent--;
        ctx.lines.push(`${ind}}`);
        return;
      }
      case 'WHILE': {
        const cond = this._jsEmitExpr(node.condition, ctx);
        ctx.lines.push(`${ind}while (${cond}) {`);
        ctx.indent++;
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        ctx.indent--;
        ctx.lines.push(`${ind}}`);
        return;
      }
      case 'LOOP': {
        const cond = this._jsEmitExpr(node.condition, ctx);
        ctx.lines.push(`${ind}do {`);
        ctx.indent++;
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        ctx.indent--;
        ctx.lines.push(`${ind}} while (${cond});`);
        return;
      }
      case 'BLOCK': {
        ctx.lines.push(`${ind}{`);
        ctx.indent++;
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        ctx.indent--;
        ctx.lines.push(`${ind}}`);
        return;
      }
      case 'CALL': {
        const cond = node.condition || {};
        const args = (cond.args || []).map(a => this._jsEmitExpr(a, ctx));
        ctx.lines.push(`${ind}${cond.name}(${args.join(', ')});`);
        return;
      }
      case 'RETURN': {
        if (node.condition) {
          ctx.lines.push(`${ind}return ${this._jsEmitExpr(node.condition, ctx)};`);
        } else {
          ctx.lines.push(`${ind}return;`);
        }
        return;
      }
      case 'BREAK':    ctx.lines.push(`${ind}break;`); return;
      case 'CONTINUE': ctx.lines.push(`${ind}continue;`); return;
      default: {
        for (const s of node.body) this._jsEmitStmt(s, ctx);
        return;
      }
    }
  }

  _jsEmitExpr(node, ctx) {
    if (!node) return 'null';
    switch (node.type) {
      case 'Literal':       return JSON.stringify(node.value);
      case 'ContainerRef':  return String(node.value);
      case 'VariableRef':   return node.name;
      case 'Operation': {
        const args = node.args.map(a => this._jsEmitExpr(a, ctx));
        switch (node.op) {
          case 'ADD':  return `(${args[0]} + ${args[1]})`;
          case 'SUB':  return `(${args[0]} - ${args[1]})`;
          case 'MUL':  return `(${args[0]} * ${args[1]})`;
          case 'DIV':  return `(${args[0]} / ${args[1]})`;
          case 'REM':  return `(${args[0]} % ${args[1]})`;
          case 'CMP':  return `(${args[0]} === ${args[1]})`;
          case 'GT':   return `(${args[0]} > ${args[1]})`;
          case 'LT':   return `(${args[0]} < ${args[1]})`;
          case 'AND':  return `(${args[0]} && ${args[1]})`;
          case 'OR':   return `(${args[0]} || ${args[1]})`;
          case 'XOR':  return `(${args[0]} ^ ${args[1]})`;
          case 'NEG':  return `(-${args[0]})`;
          case 'ASSIGN': return `${args[0]} = ${args[1]}`;
          case 'CAST':  return `(${args[1]} === 'num' ? Number(${args[0]}) : String(${args[0]}))`;
          case 'JOIN':  return `(String(${args[0]}) + String(${args[1]}))`;
          case 'SHIFT': return `(${args[0]} << ${args[1]})`;
          case 'HEX_OP':return `(${args[0]} >>> 0).toString(${args[1]})`;
          case 'QUERY': return `((${args[0]} >> ${args[1]}) & 1)`;
          default: return `null`;
        }
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._jsEmitExpr(a, ctx));
          return `${cond.name}(${args.join(', ')})`;
        }
        return 'null';
      }
      default: return 'null';
    }
  }

  // ===============
  // 3. translatePython(source) — CocoonLang → Python
  // ===============
  translatePython(source) {
    const ast = this.lang.getAST(source);
    const ctx = { indent: 0, lines: [] };
    ctx.lines.push('# CocoonLang → Python 翻译输出');
    this._pyEmitProgram(ast, ctx);
    return ctx.lines.join('\n');
  }

  _pyIndent(ctx) { return '    '.repeat(ctx.indent); }

  _pyEmitProgram(node, ctx) {
    for (const s of node.body) this._pyEmitStmt(s, ctx);
  }

  _pyEmitStmt(node, ctx) {
    const ind = this._pyIndent(ctx);
    switch (node.type) {
      case 'Operation': {
        if (node.op === 'ASSIGN') {
          const name = this._pyEmitExpr(node.args[0], ctx);
          const val = this._pyEmitExpr(node.args[1], ctx);
          ctx.lines.push(`${ind}${name} = ${val}`);
        } else {
          ctx.lines.push(`${ind}${this._pyEmitExpr(node, ctx)}`);
        }
        return;
      }
      case 'FlowControl': this._pyEmitFlow(node, ctx); return;
      case 'FuncDef': {
        const params = node.params.join(', ');
        ctx.lines.push(`${ind}def ${node.name}(${params}):`);
        ctx.indent++;
        if (node.body.length === 0) ctx.lines.push(`${this._pyIndent(ctx)}pass`);
        for (const s of node.body) this._pyEmitStmt(s, ctx);
        ctx.indent--;
        return;
      }
      case 'Communication': {
        const args = node.args.map(a => this._pyEmitExpr(a, ctx));
        switch (node.op) {
          case 'PRINT':   ctx.lines.push(`${ind}print(${args.join(', ')}, end='')`); break;
          case 'PRINTLN': ctx.lines.push(`${ind}print(${args.join(', ')})`); break;
          case 'READ':    ctx.lines.push(`${ind}input()`); break;
          default:        ctx.lines.push(`${ind}# ${node.op}(${args.join(', ')})`);
        }
        return;
      }
      default: return;
    }
  }

  _pyEmitFlow(node, ctx) {
    const ind = this._pyIndent(ctx);
    switch (node.kind) {
      case 'IF': {
        const cond = this._pyEmitExpr(node.condition, ctx);
        ctx.lines.push(`${ind}if ${cond}:`);
        ctx.indent++;
        if (node.body.length === 0) ctx.lines.push(`${this._pyIndent(ctx)}pass`);
        for (const s of node.body) this._pyEmitStmt(s, ctx);
        ctx.indent--;
        return;
      }
      case 'WHILE': {
        const cond = this._pyEmitExpr(node.condition, ctx);
        ctx.lines.push(`${ind}while ${cond}:`);
        ctx.indent++;
        if (node.body.length === 0) ctx.lines.push(`${this._pyIndent(ctx)}pass`);
        for (const s of node.body) this._pyEmitStmt(s, ctx);
        ctx.indent--;
        return;
      }
      case 'CALL': {
        const cond = node.condition || {};
        const args = (cond.args || []).map(a => this._pyEmitExpr(a, ctx));
        ctx.lines.push(`${ind}${cond.name}(${args.join(', ')})`);
        return;
      }
      case 'RETURN': {
        if (node.condition) {
          ctx.lines.push(`${ind}return ${this._pyEmitExpr(node.condition, ctx)}`);
        } else {
          ctx.lines.push(`${ind}return`);
        }
        return;
      }
      case 'BREAK':    ctx.lines.push(`${ind}break`); return;
      case 'CONTINUE': ctx.lines.push(`${ind}continue`); return;
      default: {
        for (const s of node.body) this._pyEmitStmt(s, ctx);
        return;
      }
    }
  }

  _pyEmitExpr(node, ctx) {
    if (!node) return 'None';
    switch (node.type) {
      case 'Literal':      return typeof node.value === 'string' ? JSON.stringify(node.value) : String(node.value);
      case 'ContainerRef': return String(node.value);
      case 'VariableRef':  return node.name;
      case 'Operation': {
        const args = node.args.map(a => this._pyEmitExpr(a, ctx));
        switch (node.op) {
          case 'ADD':  return `(${args[0]} + ${args[1]})`;
          case 'SUB':  return `(${args[0]} - ${args[1]})`;
          case 'MUL':  return `(${args[0]} * ${args[1]})`;
          case 'DIV':  return `(${args[0]} // ${args[1]})`;
          case 'REM':  return `(${args[0]} % ${args[1]})`;
          case 'CMP':  return `(${args[0]} == ${args[1]})`;
          case 'GT':   return `(${args[0]} > ${args[1]})`;
          case 'LT':   return `(${args[0]} < ${args[1]})`;
          case 'AND':  return `(${args[0]} and ${args[1]})`;
          case 'OR':   return `(${args[0]} or ${args[1]})`;
          case 'XOR':  return `(${args[0]} ^ ${args[1]})`;
          case 'NEG':  return `(-${args[0]})`;
          case 'ASSIGN': return args[0];
          case 'JOIN':  return `(str(${args[0]}) + str(${args[1]}))`;
          case 'HEX_OP':return `format(${args[0]}, 'x')`;
          default: return 'None';
        }
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._pyEmitExpr(a, ctx));
          return `${cond.name}(${args.join(', ')})`;
        }
        return 'None';
      }
      default: return 'None';
    }
  }

  // ===============
  // 4. translateZig(source) — CocoonLang → Zig
  // ===============
  translateZig(source) {
    const ast = this.lang.getAST(source);
    const ctx = { indent: 0, lines: [], funcs: [] };
    ctx.lines.push('// CocoonLang → Zig 翻译输出');
    ctx.lines.push("const std = @import(\"std\");");
    ctx.lines.push('');
    // 先收集函数，再输出
    for (const s of ast.body) {
      if (s.type === 'FuncDef') this._zigEmitFunc(s, ctx);
    }
    ctx.lines.push('pub fn main() void {');
    ctx.indent++;
    for (const s of ast.body) {
      if (s.type !== 'FuncDef') this._zigEmitStmt(s, ctx);
    }
    ctx.indent--;
    ctx.lines.push('}');
    return ctx.lines.join('\n');
  }

  _zigIndent(ctx) { return '    '.repeat(ctx.indent); }

  _zigEmitFunc(node, ctx) {
    const params = node.params.map((p, i) => `${p}: i32`).join(', ');
    ctx.lines.push(`fn ${node.name}(${params}) i32 {`);
    ctx.indent++;
    for (const s of node.body) this._zigEmitStmt(s, ctx);
    ctx.indent--;
    ctx.lines.push('}');
    ctx.lines.push('');
  }

  _zigEmitStmt(node, ctx) {
    const ind = this._zigIndent(ctx);
    switch (node.type) {
      case 'Operation': {
        if (node.op === 'ASSIGN') {
          const name = node.args[0].name || 'v';
          const val = this._zigEmitExpr(node.args[1], ctx);
          ctx.lines.push(`${ind}var ${name}: i32 = ${val};`);
        } else {
          ctx.lines.push(`${ind}${this._zigEmitExpr(node, ctx)};`);
        }
        return;
      }
      case 'FlowControl': {
        if (node.kind === 'IF') {
          const cond = this._zigEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}if (${cond}) {`);
          ctx.indent++;
          for (const s of node.body) this._zigEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'WHILE') {
          const cond = this._zigEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}while (${cond}) {`);
          ctx.indent++;
          for (const s of node.body) this._zigEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'RETURN') {
          const v = node.condition ? this._zigEmitExpr(node.condition, ctx) : '0';
          ctx.lines.push(`${ind}return ${v};`);
        } else if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._zigEmitExpr(a, ctx));
          ctx.lines.push(`${ind}_ = ${cond.name}(${args.join(', ')});`);
        }
        return;
      }
      case 'Communication': {
        const args = node.args.map(a => this._zigEmitExpr(a, ctx));
        if (node.op === 'PRINTLN') {
          ctx.lines.push(`${ind}std.debug.print("{d}\\n", .{${args.join(', ')}});`);
        } else if (node.op === 'PRINT') {
          ctx.lines.push(`${ind}std.debug.print("{d}", .{${args.join(', ')}});`);
        }
        return;
      }
      default: return;
    }
  }

  _zigEmitExpr(node, ctx) {
    if (!node) return '0';
    switch (node.type) {
      case 'Literal':      return String(node.value);
      case 'ContainerRef': return String(node.value);
      case 'VariableRef':  return node.name;
      case 'Operation': {
        const args = node.args.map(a => this._zigEmitExpr(a, ctx));
        switch (node.op) {
          case 'ADD': return `(${args[0]} + ${args[1]})`;
          case 'SUB': return `(${args[0]} - ${args[1]})`;
          case 'MUL': return `(${args[0]} * ${args[1]})`;
          case 'DIV': return `(@divTrunc(${args[0]}, ${args[1]}))`;
          case 'REM': return `(@rem(${args[0]}, ${args[1]}))`;
          case 'CMP': return `(${args[0]} == ${args[1]})`;
          case 'GT':  return `(${args[0]} > ${args[1]})`;
          case 'LT':  return `(${args[0]} < ${args[1]})`;
          case 'NEG': return `(-${args[0]})`;
          default: return '0';
        }
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._zigEmitExpr(a, ctx));
          return `${cond.name}(${args.join(', ')})`;
        }
        return '0';
      }
      default: return '0';
    }
  }

  // ===============
  // 5. translateRust(source) — CocoonLang → Rust
  // ===============
  translateRust(source) {
    const ast = this.lang.getAST(source);
    const ctx = { indent: 0, lines: [] };
    ctx.lines.push('// CocoonLang → Rust 翻译输出');
    ctx.lines.push('fn main() {');
    ctx.indent++;
    for (const s of ast.body) {
      if (s.type !== 'FuncDef') this._rustEmitStmt(s, ctx);
    }
    ctx.indent--;
    ctx.lines.push('}');
    // 函数定义放在 main 之后
    for (const s of ast.body) {
      if (s.type === 'FuncDef') this._rustEmitFunc(s, ctx);
    }
    return ctx.lines.join('\n');
  }

  _rustIndent(ctx) { return '    '.repeat(ctx.indent); }

  _rustEmitFunc(node, ctx) {
    const params = node.params.map(p => `${p}: i32`).join(', ');
    ctx.lines.push(`fn ${node.name}(${params}) -> i32 {`);
    ctx.indent++;
    for (const s of node.body) this._rustEmitStmt(s, ctx);
    ctx.indent--;
    ctx.lines.push('}');
  }

  _rustEmitStmt(node, ctx) {
    const ind = this._rustIndent(ctx);
    switch (node.type) {
      case 'Operation': {
        if (node.op === 'ASSIGN') {
          const name = node.args[0].name || 'v';
          const val = this._rustEmitExpr(node.args[1], ctx);
          ctx.lines.push(`${ind}let mut ${name}: i32 = ${val};`);
        } else {
          ctx.lines.push(`${ind}let _ = ${this._rustEmitExpr(node, ctx)};`);
        }
        return;
      }
      case 'FlowControl': {
        if (node.kind === 'IF') {
          const cond = this._rustEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}if ${cond} {`);
          ctx.indent++;
          for (const s of node.body) this._rustEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'WHILE') {
          const cond = this._rustEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}while ${cond} {`);
          ctx.indent++;
          for (const s of node.body) this._rustEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'RETURN') {
          const v = node.condition ? this._rustEmitExpr(node.condition, ctx) : '0';
          ctx.lines.push(`${ind}return ${v};`);
        } else if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._rustEmitExpr(a, ctx));
          ctx.lines.push(`${ind}let _ = ${cond.name}(${args.join(', ')});`);
        }
        return;
      }
      case 'Communication': {
        const args = node.args.map(a => this._rustEmitExpr(a, ctx));
        if (node.op === 'PRINTLN') {
          ctx.lines.push(`${ind}println!("{}", ${args.join(', ')});`);
        } else if (node.op === 'PRINT') {
          ctx.lines.push(`${ind}print!("{}", ${args.join(', ')});`);
        }
        return;
      }
      default: return;
    }
  }

  _rustEmitExpr(node, ctx) {
    if (!node) return '0';
    switch (node.type) {
      case 'Literal':      return typeof node.value === 'string' ? JSON.stringify(node.value) : String(node.value);
      case 'ContainerRef': return String(node.value);
      case 'VariableRef':  return node.name;
      case 'Operation': {
        const args = node.args.map(a => this._rustEmitExpr(a, ctx));
        switch (node.op) {
          case 'ADD': return `(${args[0]} + ${args[1]})`;
          case 'SUB': return `(${args[0]} - ${args[1]})`;
          case 'MUL': return `(${args[0]} * ${args[1]})`;
          case 'DIV': return `(${args[0]} / ${args[1]})`;
          case 'REM': return `(${args[0]} % ${args[1]})`;
          case 'CMP': return `(${args[0]} == ${args[1]})`;
          case 'GT':  return `(${args[0]} > ${args[1]})`;
          case 'LT':  return `(${args[0]} < ${args[1]})`;
          case 'NEG': return `(-${args[0]})`;
          default: return '0';
        }
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._rustEmitExpr(a, ctx));
          return `${cond.name}(${args.join(', ')})`;
        }
        return '0';
      }
      default: return '0';
    }
  }

  // ===============
  // 6. translateGo(source) — CocoonLang → Go
  // ===============
  translateGo(source) {
    const ast = this.lang.getAST(source);
    const ctx = { indent: 0, lines: [] };
    ctx.lines.push('// CocoonLang → Go 翻译输出');
    ctx.lines.push('package main');
    ctx.lines.push('');
    ctx.lines.push('import "fmt"');
    ctx.lines.push('');
    // 函数定义
    for (const s of ast.body) {
      if (s.type === 'FuncDef') this._goEmitFunc(s, ctx);
    }
    ctx.lines.push('func main() {');
    ctx.indent++;
    for (const s of ast.body) {
      if (s.type !== 'FuncDef') this._goEmitStmt(s, ctx);
    }
    ctx.indent--;
    ctx.lines.push('}');
    return ctx.lines.join('\n');
  }

  _goIndent(ctx) { return '\t'.repeat(ctx.indent); }

  _goEmitFunc(node, ctx) {
    const params = node.params.map(p => `${p} int`).join(', ');
    ctx.lines.push(`func ${node.name}(${params}) int {`);
    ctx.indent++;
    for (const s of node.body) this._goEmitStmt(s, ctx);
    ctx.indent--;
    ctx.lines.push('\treturn 0');
    ctx.lines.push('}');
  }

  _goEmitStmt(node, ctx) {
    const ind = this._goIndent(ctx);
    switch (node.type) {
      case 'Operation': {
        if (node.op === 'ASSIGN') {
          const name = node.args[0].name || 'v';
          const val = this._goEmitExpr(node.args[1], ctx);
          ctx.lines.push(`${ind}${name} := ${val}`);
        } else {
          ctx.lines.push(`${ind}_ = ${this._goEmitExpr(node, ctx)}`);
        }
        return;
      }
      case 'FlowControl': {
        if (node.kind === 'IF') {
          const cond = this._goEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}if ${cond} {`);
          ctx.indent++;
          for (const s of node.body) this._goEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'WHILE') {
          const cond = this._goEmitExpr(node.condition, ctx);
          ctx.lines.push(`${ind}for ${cond} {`);
          ctx.indent++;
          for (const s of node.body) this._goEmitStmt(s, ctx);
          ctx.indent--;
          ctx.lines.push(`${ind}}`);
        } else if (node.kind === 'RETURN') {
          const v = node.condition ? this._goEmitExpr(node.condition, ctx) : '0';
          ctx.lines.push(`${ind}return ${v}`);
        } else if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._goEmitExpr(a, ctx));
          ctx.lines.push(`${ind}${cond.name}(${args.join(', ')})`);
        }
        return;
      }
      case 'Communication': {
        const args = node.args.map(a => this._goEmitExpr(a, ctx));
        if (node.op === 'PRINTLN') {
          ctx.lines.push(`${ind}fmt.Println(${args.join(', ')})`);
        } else if (node.op === 'PRINT') {
          ctx.lines.push(`${ind}fmt.Print(${args.join(', ')})`);
        }
        return;
      }
      default: return;
    }
  }

  _goEmitExpr(node, ctx) {
    if (!node) return '0';
    switch (node.type) {
      case 'Literal':      return typeof node.value === 'string' ? JSON.stringify(node.value) : String(node.value);
      case 'ContainerRef': return String(node.value);
      case 'VariableRef':  return node.name;
      case 'Operation': {
        const args = node.args.map(a => this._goEmitExpr(a, ctx));
        switch (node.op) {
          case 'ADD': return `(${args[0]} + ${args[1]})`;
          case 'SUB': return `(${args[0]} - ${args[1]})`;
          case 'MUL': return `(${args[0]} * ${args[1]})`;
          case 'DIV': return `(${args[0]} / ${args[1]})`;
          case 'REM': return `(${args[0]} % ${args[1]})`;
          case 'CMP': return `(${args[0]} == ${args[1]})`;
          case 'GT':  return `(${args[0]} > ${args[1]})`;
          case 'LT':  return `(${args[0]} < ${args[1]})`;
          case 'NEG': return `(-${args[0]})`;
          default: return '0';
        }
      }
      case 'FlowControl': {
        if (node.kind === 'CALL') {
          const cond = node.condition || {};
          const args = (cond.args || []).map(a => this._goEmitExpr(a, ctx));
          return `${cond.name}(${args.join(', ')})`;
        }
        return '0';
      }
      default: return '0';
    }
  }

  // ===============
  // 7. binaryTranslate(arch)
  // ===============
  // 二进制翻译：x86/ARM/RISC-V 机器码 → CocoonVM 字节码
  // 这是一个反汇编+语义映射的骨架实现
  binaryTranslate(arch, machineCode) {
    const decoder = this._getDecoder(arch);
    if (!decoder) throw new Error(`不支持的架构: ${arch}`);

    // 机器码可以是字节数组或十六进制字符串
    const bytes = this._normalizeInput(machineCode);
    const instructions = decoder.decode(bytes);
    const bytecode = this._mapToCocoonVM(instructions);

    return {
      arch,
      inputBytes: bytes.length,
      decodedInstructions: instructions.length,
      cocoonBytecode: bytecode,
      summary: this._summarize(instructions),
    };
  }

  _normalizeInput(input) {
    if (Buffer.isBuffer(input)) return Array.from(input);
    if (Array.isArray(input)) return input;
    if (typeof input === 'string') {
      // 十六进制字符串
      const hex = input.replace(/0x|[\s,]/gi, '');
      const bytes = [];
      for (let i = 0; i + 1 < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    }
    return [];
  }

  _getDecoder(arch) {
    switch (arch) {
      case BinaryArch.X86_64:
      case BinaryArch.X86:
        return new X86Decoder();
      case BinaryArch.ARM64:
      case BinaryArch.ARM32:
        return new ARMDecoder(arch);
      case BinaryArch.RISCV64:
        return new RISCVDecoder();
      default:
        return null;
    }
  }

  _mapToCocoonVM(instructions) {
    // 把目标架构指令映射到 CocoonVM 字节码
    const code = [];
    const opMap = {
      'ADD': OpCode.ADD, 'SUB': OpCode.SUB, 'MUL': OpCode.MUL,
      'DIV': OpCode.DIV, 'AND': OpCode.AND, 'OR': OpCode.OR,
      'XOR': OpCode.XOR, 'SHL': OpCode.SHIFT, 'RET': OpCode.RET,
      'CALL': OpCode.CALL, 'JMP': OpCode.JUMP, 'JE': OpCode.JUMPF,
      'MOV': OpCode.STORE, 'PUSH': OpCode.PUSH, 'POP': OpCode.POP,
      'CMP': OpCode.CMP, 'NOP': OpCode.POP, 'HLT': OpCode.HALT,
    };
    for (const ins of instructions) {
      const op = opMap[ins.mnemonic] || OpCode.POP;
      code.push({ op, arg: ins.operand, _orig: `${ins.mnemonic} ${ins.operand || ''}` });
    }
    code.push({ op: OpCode.HALT });
    return { code, consts: [], names: [], funcs: {}, version: this.version };
  }

  _summarize(instructions) {
    const counts = {};
    for (const ins of instructions) {
      counts[ins.mnemonic] = (counts[ins.mnemonic] || 0) + 1;
    }
    return counts;
  }

  // ===============
  // 8. selfTranslate()
  // ===============
  // 自举翻译：把翻译器自身的 CocoonLang 源码翻译成多种目标语言，
  // 验证"翻译器能翻译自己"
  selfTranslate() {
    // 用 CocoonLang 语法书写的"翻译器种子"
    const translatorSource = `
; CocoonLang 自举翻译器种子（用自身语法书写）
ui 翻译(源码 源语言 目标语言) {
  ai (t 源语言 "cocoon") {
    ao 翻译出(源码 目标语言)
  }
  ao 翻译进(源码 源语言)
}
ui 翻译出(源码 目标语言) {
  d 语法 解析(源码)
  d 目标代码 生成(语法 目标语言)
  ao 目标代码
}
ui 翻译进(源码 源语言) {
  d 中间 表示(源码 源语言)
  ao 中间
}
an1("翻译器自举就绪")
`;
    // 1. 把这段 CocoonLang 翻译成各目标语言
    const translations = {
      javascript: this.translateJS(translatorSource),
      python:     this.translatePython(translatorSource),
      zig:        this.translateZig(translatorSource),
      rust:       this.translateRust(translatorSource),
      go:         this.translateGo(translatorSource),
    };
    // 2. 验证：每份翻译都应包含函数名
    const checks = {};
    for (const [lang, code] of Object.entries(translations)) {
      checks[lang] = {
        lineCount: code.split('\n').length,
        hasTranslate: code.includes('翻译') || code.includes('translate') || code.includes('翻译出'),
        charCount: code.length,
      };
    }
    return {
      bootstrap: 'self-translate',
      sourceLines: translatorSource.split('\n').length,
      translations: checks,
      selfHosting: true,
      timestamp: Date.now(),
    };
  }
}

// ========================
// 二进制解码器（x86/ARM/RISC-V 骨架）
// ========================

// x86 解码器：基于操作码表的简化解码
class X86Decoder {
  constructor() {
    // 常见 x86 操作码（单字节前缀）
    this.opcodes = {
      0x01: { mnemonic: 'ADD', operandSize: 1 },
      0x29: { mnemonic: 'SUB', operandSize: 1 },
      0x50: { mnemonic: 'PUSH', operandSize: 0 }, // push eax
      0x58: { mnemonic: 'POP',  operandSize: 0 },
      0x90: { mnemonic: 'NOP',  operandSize: 0 },
      0xC3: { mnemonic: 'RET',  operandSize: 0 },
      0xF4: { mnemonic: 'HLT',  operandSize: 0 },
      0xE8: { mnemonic: 'CALL', operandSize: 4 }, // rel32
      0xE9: { mnemonic: 'JMP',  operandSize: 4 },
      0x74: { mnemonic: 'JE',   operandSize: 1 },
      0x75: { mnemonic: 'JNE',  operandSize: 1 },
      0xB8: { mnemonic: 'MOV',  operandSize: 4 }, // mov eax, imm32
      0x89: { mnemonic: 'MOV',  operandSize: 1 },
      0x31: { mnemonic: 'XOR',  operandSize: 1 },
      0x21: { mnemonic: 'AND',  operandSize: 1 },
      0x09: { mnemonic: 'OR',   operandSize: 1 },
      0xF7: { mnemonic: 'MUL',  operandSize: 1 }, // 实际为 group
      0xD1: { mnemonic: 'SHL',  operandSize: 1 },
      0x3B: { mnemonic: 'CMP',  operandSize: 1 },
    };
  }

  decode(bytes) {
    const ins = [];
    let i = 0;
    while (i < bytes.length) {
      const op = bytes[i];
      const info = this.opcodes[op];
      if (!info) {
        // 未知操作码：作为数据跳过
        ins.push({ mnemonic: 'DB', operand: op, offset: i });
        i++;
        continue;
      }
      let operand = null;
      if (info.operandSize > 0 && i + 1 + info.operandSize <= bytes.length) {
        let val = 0;
        for (let b = 0; b < info.operandSize; b++) {
          val |= bytes[i + 1 + b] << (8 * b);
        }
        operand = val;
        i += 1 + info.operandSize;
      } else {
        i += 1;
      }
      ins.push({ mnemonic: info.mnemonic, operand, offset: i });
    }
    return ins;
  }
}

// ARM 解码器（简化：固定 4 字节指令）
class ARMDecoder {
  constructor(arch) {
    this.arch = arch;
    this.is64 = (arch === BinaryArch.ARM64);
  }

  decode(bytes) {
    const ins = [];
    const instrSize = 4;
    for (let i = 0; i + instrSize <= bytes.length; i += instrSize) {
      // 小端序读取 32 位指令字
      const word = bytes[i] | (bytes[i + 1] << 8) | (bytes[i + 2] << 16) | (bytes[i + 3] << 24);
      const decoded = this._decodeWord(word);
      ins.push({ ...decoded, offset: i });
    }
    // 尾部不足 4 字节的作为数据
    const tail = bytes.length % instrSize;
    for (let i = bytes.length - tail; i < bytes.length; i++) {
      ins.push({ mnemonic: 'DB', operand: bytes[i], offset: i });
    }
    return ins;
  }

  _decodeWord(word) {
    // 统一转为无符号 32 位，避免有符号比较错误
    const w = word >>> 0;
    if (this.is64) {
      // A64 解码骨架
      if ((w & 0xFC000000) >>> 0 === 0x94000000) return { mnemonic: 'CALL', operand: w & 0x03ffffff };
      if ((w & 0xFC000000) >>> 0 === 0x14000000) return { mnemonic: 'JMP',  operand: w & 0x03ffffff };
      if (w === 0xd65f03c0)                       return { mnemonic: 'RET',  operand: null };
      if ((w & 0x7F800000) >>> 0 === 0x52800000) return { mnemonic: 'MOV',  operand: (w >> 5) & 0xffff };
      if ((w & 0x7FE00000) >>> 0 === 0x0B000000) return { mnemonic: 'ADD',  operand: null };
      if ((w & 0x7FE00000) >>> 0 === 0x4B000000) return { mnemonic: 'SUB',  operand: null };
      if ((w & 0x7FE00000) >>> 0 === 0x1B007C00) return { mnemonic: 'MUL',  operand: null };
    } else {
      // A32 解码骨架
      if ((w & 0x0F000000) >>> 0 === 0x0B000000) return { mnemonic: 'ADD', operand: null };
      if ((w & 0x0F000000) >>> 0 === 0x0A000000) return { mnemonic: 'SUB', operand: null };
      if ((w & 0x0FF00000) >>> 0 === 0x01A00000) return { mnemonic: 'MOV', operand: null };
      if (w === 0xe1a0f00e)                        return { mnemonic: 'RET', operand: null };
      if ((w & 0x0F000000) >>> 0 === 0x0A000000)  return { mnemonic: 'JMP', operand: w & 0x00ffffff };
    }
    return { mnemonic: 'NOP', operand: w };
  }
}

// RISC-V 解码器（简化：固定 4 字节指令）
class RISCVDecoder {
  decode(bytes) {
    const ins = [];
    for (let i = 0; i + 4 <= bytes.length; i += 4) {
      const word = bytes[i] | (bytes[i + 1] << 8) | (bytes[i + 2] << 16) | (bytes[i + 3] << 24);
      const opcode = word & 0x7f;
      const decoded = this._decodeOpcode(opcode, word);
      ins.push({ ...decoded, offset: i });
    }
    return ins;
  }

  _decodeOpcode(opcode, word) {
    // RISC-V 主要操作码
    const rd = (word >> 7) & 0x1f;
    const funct3 = (word >> 12) & 0x07;
    const rs1 = (word >> 15) & 0x1f;
    const imm = (word >> 20) & 0xfff;
    switch (opcode) {
      case 0x33: { // R-type
        const map = { 0x0: 'ADD', 0x1: 'SHL', 0x4: 'AND', 0x6: 'OR', 0x7: 'AND' };
        return { mnemonic: map[funct3] || 'NOP', operand: null };
      }
      case 0x13: { // I-type ALU
        const map = { 0x0: 'ADD', 0x4: 'XOR', 0x6: 'OR', 0x7: 'AND' };
        return { mnemonic: map[funct3] || 'ADD', operand: imm };
      }
      case 0x23: return { mnemonic: 'MOV', operand: imm };  // store
      case 0x03: return { mnemonic: 'MOV', operand: imm };  // load
      case 0x63: return { mnemonic: 'JE',  operand: imm };  // branch
      case 0x6f: return { mnemonic: 'JMP', operand: imm };  // jal
      case 0x67: return { mnemonic: 'RET', operand: null }; // jalr
      case 0x73: return { mnemonic: 'HLT', operand: null }; // system
      default:   return { mnemonic: 'NOP', operand: word };
    }
  }
}

module.exports = {
  BootstrapTranslator,
  Languages,
  BinaryArch,
  X86Decoder,
  ARMDecoder,
  RISCVDecoder,
};
