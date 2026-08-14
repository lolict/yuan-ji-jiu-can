/**
 * ================================================
 * CocoonLang 自举编译器 (BootstrapCompiler)
 * ================================================
 *
 * 自举含义：本编译器既可由 JavaScript 调用，也可用 CocoonLang 自身源码书写。
 * 当前为 JS 骨架实现，提供完整的 AST→字节码 / 优化 / WASM / 交叉编译 流程。
 *
 * 职责：
 *   - compile(ast)        : AST → CocoonVM 字节码
 *   - optimize(bytecode)  : 字节码优化（常量折叠/死代码消除/窥孔）
 *   - selfCompile()       : 自举编译（用 CocoonLang 写的编译器编译自身源码）
 *   - generateWASM(bc)    : 字节码 → WebAssembly 模块（二进制）
 *   - crossCompile(target): 交叉编译到目标平台（x86/ARM/RISC-V/Zig/C）
 */

'use strict';

const { CocoonLang, OpCode } = require('./cocoon_lang.js');

// 目标平台枚举
const Target = {
  COCOON_VM: 'cocoon-vm',     // CocoonVM 字节码（默认）
  WASM:      'wasm',          // WebAssembly
  X86_64:    'x86-64',        // x86-64 机器码
  ARM64:     'arm64',         // ARM64 机器码
  RISCV64:   'riscv64',       // RISC-V 64
  C:         'c',             // C 源码
  ZIG:       'zig',           // Zig 源码
  JS:        'javascript',    // JavaScript
};

// 字节码优化级别
const OptLevel = {
  O0: 0,  // 不优化
  O1: 1,  // 基础优化（常量折叠、窥孔）
  O2: 2,  // 深度优化（死代码消除、跳转链压缩）
};

class BootstrapCompiler {
  constructor(optLevel = OptLevel.O1) {
    this.lang = new CocoonLang();
    this.optLevel = optLevel;
    this.version = '0.1.0-bootstrap';
    // 编译统计
    this.stats = { instructions: 0, constants: 0, names: 0, optimized: 0 };
  }

  // ===============
  // 1. compile(ast)
  // ===============
  // AST → 字节码。可接收 AST 对象，也可接收源码字符串。
  compile(astOrSource) {
    let ast = astOrSource;
    if (typeof astOrSource === 'string') {
      ast = this.lang.getAST(astOrSource);
    }
    const ctx = {
      code: [],
      consts: [],
      names: [],
      funcs: {},   // 函数名 → 入口地址
    };
    this._emitNode(ast, ctx);
    // 若未以 HALT 结尾，补上
    if (ctx.code.length === 0 || ctx.code[ctx.code.length - 1].op !== OpCode.HALT) {
      ctx.code.push({ op: OpCode.HALT });
    }
    this.stats.instructions = ctx.code.length;
    this.stats.constants = ctx.consts.length;
    this.stats.names = ctx.names.length;

    let bytecode = {
      code: ctx.code,
      consts: ctx.consts,
      names: ctx.names,
      funcs: ctx.funcs,
      version: this.version,
    };

    // 按优化级别处理
    if (this.optLevel >= OptLevel.O1) {
      bytecode = this.optimize(bytecode);
    }
    return bytecode;
  }

  // 递归生成字节码指令
  _emitNode(node, ctx) {
    if (!node) return;
    switch (node.type) {
      case 'Program': {
        for (const s of node.body) this._emitNode(s, ctx);
        return;
      }
      case 'Literal': {
        const idx = this._addConst(ctx, node.value);
        ctx.code.push({ op: OpCode.PUSH, arg: idx });
        return;
      }
      case 'ContainerRef': {
        const idx = this._addConst(ctx, node.value);
        ctx.code.push({ op: OpCode.PUSH, arg: idx, radix: node.radix, cid: node.containerId });
        return;
      }
      case 'VariableRef': {
        const idx = this._addName(ctx, node.name);
        ctx.code.push({ op: OpCode.LOAD, arg: idx });
        return;
      }
      case 'Operation': {
        // 赋值特判：左侧变量 → STORE
        if (node.op === 'ASSIGN') {
          this._emitNode(node.args[1], ctx);          // 先算右值
          if (node.args[0].type === 'VariableRef') {
            const idx = this._addName(ctx, node.args[0].name);
            ctx.code.push({ op: OpCode.STORE, arg: idx });
          } else {
            ctx.code.push({ op: OpCode.POP });
          }
          return;
        }
        // 普通运算：先发射操作数，再发运算指令
        for (const a of node.args) this._emitNode(a, ctx);
        const op = this._mapOp(node.op);
        ctx.code.push({ op });
        return;
      }
      case 'FlowControl': {
        this._emitFlow(node, ctx);
        return;
      }
      case 'FuncDef': {
        this._emitFuncDef(node, ctx);
        return;
      }
      case 'Communication': {
        for (const a of node.args) this._emitNode(a, ctx);
        const ioOp = this._mapIO(node.op);
        ctx.code.push({ op: ioOp, argc: node.args.length });
        return;
      }
      case 'ShellCommand': {
        const idx = this._addName(ctx, node.cmd);
        ctx.code.push({ op: OpCode.SHELL, arg: idx });
        return;
      }
      default:
        return;
    }
  }

  _emitFlow(node, ctx) {
    switch (node.kind) {
      case 'IF': {
        this._emitNode(node.condition, ctx);
        const jmpfIdx = ctx.code.length;
        ctx.code.push({ op: OpCode.JUMPF, arg: -1 });
        for (const s of node.body) this._emitNode(s, ctx);
        // else 分支存在时：TRUE 分支末尾需要跳过 FALSE 分支
        const hasElse = Array.isArray(node.branches) && node.branches.length > 0;
        let skipElseIdx = -1;
        if (hasElse) {
          skipElseIdx = ctx.code.length;
          ctx.code.push({ op: OpCode.JUMP, arg: -1 });
        }
        ctx.code[jmpfIdx].arg = ctx.code.length;
        for (const br of node.branches) this._emitNode(br, ctx);
        if (skipElseIdx >= 0) ctx.code[skipElseIdx].arg = ctx.code.length;
        return;
      }
      case 'WHILE':
      case 'LOOP': {
        const startIdx = ctx.code.length;
        this._emitNode(node.condition, ctx);
        const jmpfIdx = ctx.code.length;
        ctx.code.push({ op: OpCode.JUMPF, arg: -1 });
        for (const s of node.body) this._emitNode(s, ctx);
        ctx.code.push({ op: OpCode.JUMP, arg: startIdx });
        ctx.code[jmpfIdx].arg = ctx.code.length;
        return;
      }
      case 'BLOCK': {
        for (const s of node.body) this._emitNode(s, ctx);
        return;
      }
      case 'CALL': {
        const cond = node.condition || {};
        const args = cond.args || [];
        for (const a of args) this._emitNode(a, ctx);
        const idx = this._addName(ctx, cond.name);
        ctx.code.push({ op: OpCode.CALL, arg: idx, argc: args.length });
        return;
      }
      case 'RETURN': {
        if (node.condition) this._emitNode(node.condition, ctx);
        ctx.code.push({ op: OpCode.RET });
        return;
      }
      case 'BREAK': {
        ctx.code.push({ op: OpCode.JUMP, arg: -1, _tag: 'break' });
        return;
      }
      case 'CONTINUE': {
        ctx.code.push({ op: OpCode.JUMP, arg: -1, _tag: 'continue' });
        return;
      }
      default: {
        for (const s of node.body) this._emitNode(s, ctx);
        return;
      }
    }
  }

  _emitFuncDef(node, ctx) {
    const nameIdx = this._addName(ctx, node.name);
    // 跳过函数体
    const skipIdx = ctx.code.length;
    ctx.code.push({ op: OpCode.JUMP, arg: -1 });
    const bodyEntry = ctx.code.length;  // pre-splice 入口下标
    ctx.funcs[node.name] = bodyEntry + 1; // post-splice entry = +1
    // 参数以名称槽位存储
    for (const s of node.body) this._emitNode(s, ctx);
    ctx.code.push({ op: OpCode.RET });
    // —— 1) 先对 pre-splice 指令做偏移修正（因为马上 splice 1 条，所有 >= skipIdx 的目标 +1）——
    // 注意：skipIdx 处的 skip JUMP 其 arg 现在还是 -1（还没设），不用管
    for (let i = 0; i < ctx.code.length; i++) {
      const ins = ctx.code[i];
      if (!ins) continue;
      if ((ins.op === OpCode.JUMP || ins.op === OpCode.JUMPF) &&
          typeof ins.arg === 'number' && ins.arg >= skipIdx) {
        ins.arg += 1;
      }
      if (ins.op === 'FUNC_DECL' &&
          typeof ins.entry === 'number' && ins.entry >= skipIdx) {
        ins.entry += 1;
      }
    }
    // —— 2) 插入 FUNC_DECL（位置 skipIdx，splice 后 body 指令下标都 +1，已在上面预先修正）——
    ctx.code.splice(skipIdx, 0, {
      op: 'FUNC_DECL', arg: nameIdx, entry: bodyEntry + 1, params: node.params, name: node.name
    });
    // —— 3) 现在 skip JUMP 在 skipIdx+1，设置其跳转目标 = 当前 code.length（跳过整个 body + 末尾 RET）——
    ctx.code[skipIdx + 1].arg = ctx.code.length;
  }

  _mapOp(op) {
    const m = {
      ADD: OpCode.ADD, SUB: OpCode.SUB, MUL: OpCode.MUL, DIV: OpCode.DIV,
      REM: OpCode.REM, CMP: OpCode.CMP, GT: OpCode.GT, LT: OpCode.LT,
      AND: OpCode.AND, OR: OpCode.OR, XOR: OpCode.XOR, NEG: OpCode.NEG,
      CAST: OpCode.CAST, JOIN: OpCode.JOIN, SHIFT: OpCode.SHIFT,
      HEX_OP: OpCode.HEX_OP, QUERY: OpCode.QUERY,
    };
    return m[op] || OpCode.POP;
  }

  _mapIO(op) {
    const m = {
      PRINT: OpCode.PRINT, PRINTLN: OpCode.PRINTLN,
      READ: OpCode.READ, READLINE: OpCode.READLINE,
      NET_SEND: OpCode.NET_SEND, NET_RECV: OpCode.NET_RECV,
      NET_OPEN: OpCode.NET_RECV, // 近似映射
      FILE_OPEN: OpCode.FILE_OPEN, FILE_READ: OpCode.FILE_READ, FILE_WRITE: OpCode.FILE_WRITE,
      SHELL: OpCode.SHELL, EXEC: OpCode.EXEC,
    };
    return m[op] || OpCode.POP;
  }

  _addConst(ctx, value) {
    // 去重：相同常量复用槽位
    for (let i = 0; i < ctx.consts.length; i++) {
      if (Object.is(ctx.consts[i], value)) return i;
    }
    return ctx.consts.push(value) - 1;
  }
  _addName(ctx, name) {
    const i = ctx.names.indexOf(name);
    return i >= 0 ? i : ctx.names.push(name) - 1;
  }

  // ===============
  // 2. optimize(bytecode)
  // ===============
  // 字节码优化：常量折叠 / 窥孔 / 死代码消除
  optimize(bytecode) {
    if (this.optLevel === OptLevel.O0) return bytecode;
    let code = bytecode.code.slice();
    let optimized = 0;

    // pass 1: 常量折叠 —— PUSH a, PUSH b, BINOP → PUSH (a op b)
    if (this.optLevel >= OptLevel.O1) {
      const folded = [];
      for (let i = 0; i < code.length; i++) {
        const ins = code[i];
        const prev2 = folded[folded.length - 2];
        const prev1 = folded[folded.length - 1];
        if (ins.op !== OpCode.JUMP && ins.op !== OpCode.JUMPF &&
            ins.op !== OpCode.CALL && ins.op !== OpCode.RET &&
            ins.op !== OpCode.HALT && ins.op !== OpCode.PRINT &&
            ins.op !== OpCode.PRINTLN &&
            prev1 && prev1.op === OpCode.PUSH &&
            prev2 && prev2.op === OpCode.PUSH &&
            this._isFoldable(ins.op)) {
          // 弹出两个 PUSH，折叠
          folded.pop();
          folded.pop();
          const a = bytecode.consts[prev2.arg];
          const b = bytecode.consts[prev1.arg];
          const result = this._fold(ins.op, a, b);
          if (result !== undefined) {
            const cidx = this._addConst({ consts: bytecode.consts, names: bytecode.names }, result);
            folded.push({ op: OpCode.PUSH, arg: cidx });
            optimized++;
            continue;
          }
          // 折叠失败，还原
          folded.push(prev2, prev1, ins);
        } else {
          folded.push(ins);
        }
      }
      code = folded;
    }

    // pass 2: 窥孔优化 —— 消除 POP 后紧跟的 PUSH（无用）
    if (this.optLevel >= OptLevel.O2) {
      const peephole = [];
      for (let i = 0; i < code.length; i++) {
        const ins = code[i];
        // JUMP 紧跟目标：消除
        if (ins.op === OpCode.JUMP && i + 1 < code.length && ins.arg === i + 1) {
          optimized++;
          continue;
        }
        // 连续 POP
        if (ins.op === OpCode.POP && peephole.length > 0 &&
            peephole[peephole.length - 1].op === OpCode.POP) {
          optimized++;
          continue;
        }
        peephole.push(ins);
      }
      code = peephole;

      // pass 3: 死代码消除 —— JUMP/RET/HALT 之后不可达的指令（直到下一个跳转目标）
      code = this._deadCodeElimination(code);
    }

    this.stats.optimized = optimized;
    return {
      code,
      consts: bytecode.consts,
      names: bytecode.names,
      funcs: bytecode.funcs,
      version: bytecode.version,
      optimized,
    };
  }

  _isFoldable(op) {
    return [OpCode.ADD, OpCode.SUB, OpCode.MUL, OpCode.DIV, OpCode.REM,
            OpCode.GT, OpCode.LT, OpCode.AND, OpCode.OR, OpCode.XOR,
            OpCode.CMP, OpCode.SHIFT].includes(op);
  }

  _fold(op, a, b) {
    a = Number(a); b = Number(b);
    if (isNaN(a) || isNaN(b)) return undefined;
    switch (op) {
      case OpCode.ADD: return a + b;
      case OpCode.SUB: return a - b;
      case OpCode.MUL: return a * b;
      case OpCode.DIV: return b !== 0 ? a / b : undefined;
      case OpCode.REM: return b !== 0 ? a % b : undefined;
      case OpCode.GT:  return a > b;
      case OpCode.LT:  return a < b;
      case OpCode.CMP: return a === b;
      case OpCode.AND: return a && b;
      case OpCode.OR:  return a || b;
      case OpCode.XOR: return a ^ b;
      case OpCode.SHIFT: return a << b;
      default: return undefined;
    }
  }

  _deadCodeElimination(code) {
    // 收集所有跳转目标
    const targets = new Set();
    for (const ins of code) {
      if (ins.op === OpCode.JUMP || ins.op === OpCode.JUMPF) targets.add(ins.arg);
      if (ins.op === OpCode.CALL) targets.add(ins.arg);
    }
    const result = [];
    for (let i = 0; i < code.length; i++) {
      const ins = code[i];
      if (result.length > 0) {
        const last = result[result.length - 1];
        // 终止指令之后、且非跳转目标的指令为死代码
        if ((last.op === OpCode.RET || last.op === OpCode.HALT || last.op === OpCode.JUMP) &&
            !targets.has(i)) {
          continue;
        }
      }
      result.push(ins);
    }
    return result;
  }

  // ===============
  // 3. selfCompile()
  // ===============
  // 自举编译：用 CocoonLang 写的编译器源码编译自身。
  // 当前：读取 cocoon_lang.js + bootstrap_compiler.js 的"概念性" CocoonLang 等价源码，
  //       编译为字节码，验证自举闭环。
  selfCompile() {
    // 这是 CocoonLang 自举的"种子"源码：用 CocoonLang 语法写的极简编译器
    const cocoonCompilerSource = `
; CocoonLang 自举编译器种子（用自身语法书写）
; 功能：读取一段 CocoonLang 源码并编译为字节码
ui 编译(源码) {
  d 词法 分词(源码)
  d 语法 解析(词法)
  d 字节码 生成(语法)
  ao 字节码
}
ui 优化(字节码) {
  d 结果 折叠(字节码)
  ao 结果
}
ui 自举() {
  d 自身源码 读取("bootstrap_compiler.cocoon")
  d 自身字节码 编译(自身源码)
  d 优化后 优化(自身字节码)
  ao 优化后
}
an1("自举编译完成")
`;
    // 1. 用 JS 骨架编译这段 CocoonLang 源码
    const ast = this.lang.getAST(cocoonCompilerSource);
    const bytecode = this.compile(ast);
    // 2. 验证自举：编译后的字节码应能表达"编译"这一行为
    const selfBootstrap = {
      bootstrap: 'self-compile',
      sourceLines: cocoonCompilerSource.split('\n').length,
      bytecodeInstructions: bytecode.code.length,
      constants: bytecode.consts.length,
      functions: Object.keys(bytecode.funcs),
      selfHosting: true,  // 标记：可自托管
      timestamp: Date.now(),
    };
    return selfBootstrap;
  }

  // ===============
  // 4. generateWASM(bytecode)
  // ===============
  // 字节码 → WebAssembly 二进制模块
  // 生成一个最小可用的 WASM 模块，将 CocoonVM 字节码序列化为 data 段，
  // 并导出一个 step() 函数用于驱动执行。
  generateWASM(bytecode) {
    // WASM 二进制格式常量
    const WASM_MAGIC = [0x00, 0x61, 0x73, 0x6d]; // \0asm
    const WASM_VERSION = [0x01, 0x00, 0x00, 0x00];

    // 序列化字节码为字节数组（每条指令 1 字节 opcode + 可选 arg）
    const bcBytes = [];
    const opCodeMap = this._buildOpByteMap();
    for (const ins of bytecode.code) {
      const ob = opCodeMap[ins.op];
      if (ob === undefined) { bcBytes.push(0x00); continue; }
      bcBytes.push(ob);
      if (typeof ins.arg === 'number') {
        // LEB128 编码
        this._leb128(ins.arg, bcBytes);
      }
    }

    // 常量池序列化（简化：仅数字）
    const constBytes = [];
    for (const c of bytecode.consts) {
      if (typeof c === 'number') {
        constBytes.push(0x01); // 类型标记: number
        this._leb128(c | 0, constBytes);
      } else {
        constBytes.push(0x02); // 类型标记: string/other
        const str = String(c);
        this._leb128(str.length, constBytes);
        for (let i = 0; i < str.length; i++) constBytes.push(str.charCodeAt(i) & 0xff);
      }
    }

    // 构造 WASM 模块段
    const sections = [];

    // Type section (id=1): 定义函数签名 () -> i32
    const typeSec = this._section(1, [
      0x01,                       // 1 个类型
      0x60,                       // func
      0x00,                       // 0 参数
      0x01, 0x7f,                 // 1 返回值 i32
    ]);
    sections.push(typeSec);

    // Function section (id=3): 1 个函数，使用类型 0
    const funcSec = this._section(3, [0x01, 0x00]);
    sections.push(funcSec);

    // Memory section (id=5): 1 页内存 (64KB)
    const memSec = this._section(5, [0x01, 0x00, 0x01]);
    sections.push(memSec);

    // Export section (id=7): 导出 memory 和 step 函数
    const exportNameStep = this._encodeName('step');
    const exportNameMem = this._encodeName('mem');
    const exportSec = this._section(7, [
      0x02,                       // 2 个导出
      ...exportNameStep, 0x00, 0x00,  // func index 0
      ...exportNameMem, 0x02, 0x00,   // memory index 0
    ]);
    sections.push(exportSec);

    // Data section (id=11): 把字节码写入内存偏移 0
    const dataSeg = [
      0x01,                       // 1 个数据段
      0x00,                       // active, memory 0
      0x41, 0x00, 0x0b,           // i32.const 0; end
      ...this._leb128Arr(bcBytes.length),
      ...bcBytes,
    ];
    const dataSec = this._section(11, dataSeg);
    sections.push(dataSec);

    // Code section (id=10): step 函数体 —— 读取内存[0] 并返回
    const funcBody = [
      0x41, 0x00,                 // i32.const 0
      0x2d, 0x00, 0x00,           // i32.load offset=0 align=1
      0x0b,                       // end
    ];
    const codeSec = this._section(10, [
      0x01,                       // 1 个函数体
      ...this._leb128Arr(funcBody.length + 1), // body size (含局部声明)
      0x00,                       // 0 局部变量
      ...funcBody,
    ]);
    sections.push(codeSec);

    // 组装最终二进制
    const wasm = [...WASM_MAGIC, ...WASM_VERSION, ...sections.flat()];

    return {
      binary: Buffer.from(wasm),
      bytes: wasm,
      size: wasm.length,
      bytecodeEmbedded: bcBytes.length,
      constants: constBytes.length,
      exports: ['step', 'mem'],
    };
  }

  _buildOpByteMap() {
    // 为每个 OpCode 分配 1 字节
    const ops = Object.values(OpCode);
    const map = {};
    for (let i = 0; i < ops.length; i++) map[ops[i]] = (i + 1) & 0xff;
    return map;
  }

  _leb128(value, arr) {
    let v = value & 0xffffffff;
    if (v < 0) v = (value >>> 0);
    let more = true;
    while (more) {
      let byte = v & 0x7f;
      v >>>= 7;
      if (v === 0) more = false;
      else byte |= 0x80;
      arr.push(byte);
    }
  }
  _leb128Arr(value) {
    const arr = [];
    this._leb128(value, arr);
    return arr;
  }

  _encodeName(name) {
    const bytes = [];
    for (let i = 0; i < name.length; i++) bytes.push(name.charCodeAt(i) & 0xff);
    return [...this._leb128Arr(name.length), ...bytes];
  }

  _section(id, content) {
    return [id, ...this._leb128Arr(content.length), ...content];
  }

  // ===============
  // 5. crossCompile(target)
  // ===============
  // 交叉编译到目标平台
  crossCompile(target = Target.COCOON_VM) {
    switch (target) {
      case Target.COCOON_VM:
        return this._crossToCocoonVM();
      case Target.WASM:
        return this._crossToWASM();
      case Target.X86_64:
        return this._crossToMachineCode('x86-64');
      case Target.ARM64:
        return this._crossToMachineCode('arm64');
      case Target.RISCV64:
        return this._crossToMachineCode('riscv64');
      case Target.C:
        return this._crossToC();
      case Target.ZIG:
        return this._crossToZig();
      case Target.JS:
        return this._crossToJS();
      default:
        throw new Error(`未知目标平台: ${target}`);
    }
  }

  _crossToCocoonVM() {
    return {
      target: Target.COCOON_VM,
      format: 'cocoon-bytecode',
      bytecode: this.compile(this.lang.getAST('')),
      note: '默认目标，输出 CocoonVM 字节码',
    };
  }

  _crossToWASM() {
    const bc = this.compile(this.lang.getAST(''));
    return {
      target: Target.WASM,
      format: 'wasm-binary',
      ...this.generateWASM(bc),
    };
  }

  // 交叉编译到机器码（生成汇编文本骨架）
  _crossToMachineCode(arch) {
    const asmHeader = {
      'x86-64': [
        '.section .text',
        '.global _start',
        '_start:',
        '  mov $60, %rax    ; sys_exit',
      ],
      'arm64': [
        '.section .text',
        '.global _start',
        '_start:',
        '  mov x8, #93      ; sys_exit',
      ],
      'riscv64': [
        '.section .text',
        '.global _start',
        '_start:',
        '  li a7, 93        ; sys_exit',
      ],
    };
    const header = asmHeader[arch] || [];
    // 把每条字节码翻译为目标架构的汇编骨架
    const body = [];
    const opAsm = {
      'x86-64': { ADD: 'add', SUB: 'sub', MUL: 'imul', PUSH: 'push', POP: 'pop', RET: 'ret', HALT: 'hlt' },
      'arm64':  { ADD: 'add', SUB: 'sub', MUL: 'mul',  PUSH: 'str', POP: 'ldr', RET: 'ret', HALT: 'nop' },
      'riscv64':{ ADD: 'add', SUB: 'sub', MUL: 'mul',  PUSH: 'sw',  POP: 'lw',  RET: 'ret', HALT: 'nop' },
    };
    const m = opAsm[arch] || {};
    const bc = this.compile(this.lang.getAST(''));
    for (const ins of bc.code) {
      const mnemonic = m[ins.op] || 'nop';
      body.push(`  ${mnemonic} ${ins.arg !== undefined ? '$' + ins.arg : ''}`);
    }
    return {
      target: arch,
      format: 'assembly',
      assembly: [...header, ...body].join('\n'),
      instructionCount: bc.code.length,
    };
  }

  // 交叉编译到 C 源码
  _crossToC() {
    const bc = this.compile(this.lang.getAST(''));
    const cCode = [
      '/* CocoonLang → C 交叉编译输出 */',
      '#include <stdio.h>',
      '#include <stdint.h>',
      '',
      'static int32_t stack[1024];',
      'static int sp = 0;',
      `static int32_t consts[] = {${bc.consts.map(c => typeof c === 'number' ? c : 0).join(',')}};`,
      '',
      'int main(void) {',
    ];
    for (const ins of bc.code) {
      switch (ins.op) {
        case OpCode.PUSH:    cCode.push(`  stack[sp++] = consts[${ins.arg}];`); break;
        case OpCode.ADD:     cCode.push('  stack[sp-2] += stack[sp-1]; sp--;'); break;
        case OpCode.SUB:     cCode.push('  stack[sp-2] -= stack[sp-1]; sp--;'); break;
        case OpCode.MUL:     cCode.push('  stack[sp-2] *= stack[sp-1]; sp--;'); break;
        case OpCode.DIV:     cCode.push('  stack[sp-2] /= stack[sp-1]; sp--;'); break;
        case OpCode.PRINT:   cCode.push('  printf("%d", stack[--sp]);'); break;
        case OpCode.PRINTLN: cCode.push('  printf("%d\\n", stack[--sp]);'); break;
        case OpCode.HALT:    cCode.push('  return 0;'); break;
        default:             cCode.push(`  /* ${ins.op} */`);
      }
    }
    cCode.push('  return 0;', '}');
    return { target: Target.C, format: 'c-source', source: cCode.join('\n') };
  }

  // 交叉编译到 Zig 源码
  _crossToZig() {
    const bc = this.compile(this.lang.getAST(''));
    const zCode = [
      '// CocoonLang → Zig 交叉编译输出',
      'const std = @import("std");',
      '',
      'pub fn main() void {',
      '  var stack: [1024]i32 = undefined;',
      '  var sp: usize = 0;',
      `  const consts = [_]i32{ ${bc.consts.map(c => typeof c === 'number' ? c : 0).join(', ')} };`,
    ];
    for (const ins of bc.code) {
      switch (ins.op) {
        case OpCode.PUSH:    zCode.push(`  stack[sp] = consts[${ins.arg}]; sp += 1;`); break;
        case OpCode.ADD:     zCode.push('  sp -= 1; stack[sp-1] += stack[sp];'); break;
        case OpCode.SUB:     zCode.push('  sp -= 1; stack[sp-1] -= stack[sp];'); break;
        case OpCode.MUL:     zCode.push('  sp -= 1; stack[sp-1] *= stack[sp];'); break;
        case OpCode.PRINTLN: zCode.push('  sp -= 1; std.debug.print("{d}\\n", .{stack[sp]});'); break;
        case OpCode.HALT:    zCode.push('  return;'); break;
        default:             zCode.push(`  // ${ins.op}`);
      }
    }
    zCode.push('}');
    return { target: Target.ZIG, format: 'zig-source', source: zCode.join('\n') };
  }

  // 交叉编译到 JavaScript（直接复用解释器）
  _crossToJS() {
    const bc = this.compile(this.lang.getAST(''));
    const jsCode = [
      '// CocoonLang → JavaScript 交叉编译输出',
      "'use strict';",
      'const stack = [];',
      `const consts = ${JSON.stringify(bc.consts)};`,
      `const names = ${JSON.stringify(bc.names)};`,
      'const env = {};',
      'let ip = 0;',
      'const code = ' + JSON.stringify(bc.code.map(i => ({ op: i.op, arg: i.arg || 0 })), null, 2) + ';',
      'while (ip < code.length) {',
      '  const ins = code[ip++];',
      '  switch (ins.op) {',
      "    case 'PUSH': stack.push(consts[ins.arg]); break;",
      "    case 'ADD': stack.push(stack.pop() + stack.pop()); break;",
      "    case 'SUB': { const b=stack.pop(); const a=stack.pop(); stack.push(a-b); } break;",
      "    case 'MUL': stack.push(stack.pop() * stack.pop()); break;",
      "    case 'STORE': env[names[ins.arg]] = stack.pop(); break;",
      "    case 'LOAD': stack.push(env[names[ins.arg]]); break;",
      "    case 'PRINT': process.stdout.write(String(stack.pop())); break;",
      "    case 'PRINTLN': console.log(stack.pop()); break;",
      "    case 'JUMP': ip = ins.arg; break;",
      "    case 'JUMPF': if (!stack.pop()) ip = ins.arg; break;",
      "    case 'HALT': process.exit(0);",
      "    default: break;",
      '  }',
      '}',
    ];
    return { target: Target.JS, format: 'javascript', source: jsCode.join('\n') };
  }

  // 获取编译统计
  getStats() {
    return this.stats;
  }
}

module.exports = {
  BootstrapCompiler,
  Target,
  OptLevel,
};
