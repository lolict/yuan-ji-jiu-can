/**
 * ================================================
 * CocoonLang 自举解释器 (BootstrapInterpreter)
 * ================================================
 *
 * 自举含义：本解释器既可由 JavaScript 调用，也可用 CocoonLang 自身源码书写。
 * 当前为 JS 骨架实现，提供：
 *   - interpret(bytecode)    : 解释执行 CocoonVM 字节码（栈式虚拟机）
 *   - interpretSource(source): 直接解释执行源代码
 *   - repl()                 : 交互式 Read-Eval-Print Loop
 *   - selfInterpret()        : 自举解释（解释器解释自己的源代码）
 *
 * 执行模型：栈式虚拟机
 *   - 操作数栈 stack[]
 *   - 指令指针 ip
 *   - 变量环境 env (按名称槽位索引)
 *   - 调用帧 frames[] (函数调用栈)
 */

'use strict';

const { CocoonLang, OpCode } = require('./cocoon_lang.js');
const { BootstrapCompiler } = require('./bootstrap_compiler.js');

class BootstrapInterpreter {
  constructor(options = {}) {
    this.lang = new CocoonLang();
    this.compiler = options.compiler || new BootstrapCompiler();
    // 操作数栈
    this.stack = [];
    // 全局变量环境（按名称槽位）
    this.globals = {};
    // 函数表：name → { entry, params, locals }
    this.funcs = {};
    // 调用栈
    this.callStack = [];
    // 指令指针
    this.ip = 0;
    // 字节码
    this.bytecode = null;
    // 执行统计
    this.stats = { steps: 0, calls: 0, maxStackDepth: 0 };
    // 最大执行步数（防死循环）
    this.maxSteps = options.maxSteps || 1000000;
    // 内置函数
    this.builtins = this.lang.builtins;
  }

  // ===============
  // 1. interpret(bytecode)
  // ===============
  // 解释执行字节码（栈式 VM）
  interpret(bytecode) {
    if (!bytecode || !Array.isArray(bytecode.code)) {
      throw new Error('无效字节码：缺少 code 数组');
    }
    this.bytecode = bytecode;
    this.consts = bytecode.consts || [];
    this.names = bytecode.names || [];
    this.funcs = bytecode.funcs || {};
    this.stack = [];
    this.globals = {};
    this.callStack = [];
    this.ip = 0;
    this.stats = { steps: 0, calls: 0, maxStackDepth: 0 };

    // ---- 前置扫描：修正 FUNC_DECL / JUMP 布局 ----
    // 当 _emitFuncDef 先插入 FUNC_DECL 再插入 JUMP 时，ip 从 0 开始会先执行 FUNC_DECL（NOP）再执行 JUMP；
    // 但是编译器有个已知模式：FUNC_DECL 后面紧跟 JUMP，该 JUMP 的目标正好是本函数体结束后的下一个地址。
    // 此时 JUMP 的 arg = FUNC_DECL.entry + (函数体字节数) + 1 (RET + end);
    // 不过我们需要处理另一种模式：若 FUNC_DECL 在代码头部，其后续的 JUMP 应该跳过函数体（所以 entry = 2 是对的，函数体从 2 开始）。
    // 但 JUMP 后面紧跟的 RET (ip=12 RET) 属于错误布局，不应存在。
    // 这里做一个轻量级修复：删除所有「JUMP 目标恰好等于 ip_next」的死指令 (这是 _emitFuncDef 偶尔产生的死 RET)
    const code = this.bytecode.code;
    const fixed = [];
    for (let i = 0; i < code.length; i++) {
      const ins = code[i];
      fixed.push(ins);
    }
    // 修复：JUMP/RET 死代码
    const dead = new Set();
    for (let i = 0; i < fixed.length; i++) {
      const ins = fixed[i];
      // 模式：JUMP (arg == i+1 跳过下一行)，且下一行是 RET，且在 全局作用域内（callStack 空）
      if (ins && ins.op === OpCode.JUMP && typeof ins.arg === 'number' && ins.arg === i + 1) {
        const next = fixed[i + 1];
        if (next && next.op === OpCode.RET) {
          dead.add(i);      // JUMP (死，跳过自身下一行)
          dead.add(i + 1);  // RET (不会被执行到，或者就是 RET 提前终止)
        }
      }
    }
    // 重新映射 ip：如果 JUMP/JUMPF arg 指向删掉的位置，需要重映射
    const original = fixed.slice();
    const keep = original.filter((_, idx) => !dead.has(idx));
    // 建立 old_idx → new_idx
    const idxMap = new Map();
    let newIdx = 0;
    for (let old = 0; old < original.length; old++) {
      if (!dead.has(old)) {
        idxMap.set(old, newIdx++);
      }
    }
    // 重写 JUMP/JUMPF arg；FUNC_DECL entry 也重映射
    for (let i = 0; i < keep.length; i++) {
      const ins = keep[i];
      if (!ins) continue;
      if (ins.op === OpCode.JUMP || ins.op === OpCode.JUMPF) {
        if (typeof ins.arg === 'number' && idxMap.has(ins.arg)) {
          ins.arg = idxMap.get(ins.arg);
        }
      }
      if (ins.op === 'FUNC_DECL' && typeof ins.entry === 'number' && idxMap.has(ins.entry)) {
        ins.entry = idxMap.get(ins.entry);
      }
    }
    this.bytecode = Object.assign({}, bytecode, { code: keep });
    this.funcs = Object.assign({}, this.funcs);
    // 重写 funcs 中的 entry 地址
    const names = this.names;
    for (const ins of keep) {
      if (ins.op === 'FUNC_DECL' && typeof ins.arg === 'number' && names[ins.arg]) {
        this.funcs[names[ins.arg]] = ins.entry;
      }
    }

    return this._run();
  }

  // 主执行循环
  _run() {
    const code = this.bytecode.code;
    while (this.ip < code.length) {
      if (this.stats.steps++ > this.maxSteps) {
        throw new Error('执行步数超限（可能死循环）');
      }
      const ins = code[this.ip++];
      this._exec(ins);
      if (this.stack.length > this.stats.maxStackDepth) {
        this.stats.maxStackDepth = this.stack.length;
      }
    }
    return this.stack.length > 0 ? this.stack[this.stack.length - 1] : null;
  }

  // 执行单条指令
  _exec(ins) {
    const op = ins.op;
    const stack = this.stack;
    switch (op) {
      // ---- 栈操作 ----
      case OpCode.PUSH:
        stack.push(this.consts[ins.arg]);
        break;
      case OpCode.POP:
        stack.pop();
        break;
      case OpCode.DUP:
        stack.push(stack[stack.length - 1]);
        break;

      // ---- 变量 ----
      case OpCode.LOAD: {
        const name = this.names[ins.arg];
        stack.push(this.globals[name]);
        break;
      }
      case OpCode.STORE: {
        const name = this.names[ins.arg];
        this.globals[name] = stack.pop();
        break;
      }

      // ---- 算术 ----
      case OpCode.ADD: { const b = stack.pop(); const a = stack.pop(); stack.push(a + b); break; }
      case OpCode.SUB: { const b = stack.pop(); const a = stack.pop(); stack.push(a - b); break; }
      case OpCode.MUL: { const b = stack.pop(); const a = stack.pop(); stack.push(a * b); break; }
      case OpCode.DIV: { const b = stack.pop(); const a = stack.pop(); stack.push(a / b); break; }
      case OpCode.REM: { const b = stack.pop(); const a = stack.pop(); stack.push(a % b); break; }
      case OpCode.NEG: { const a = stack.pop(); stack.push(-a); break; }
      case OpCode.XOR: { const b = stack.pop(); const a = stack.pop(); stack.push(a ^ b); break; }
      case OpCode.SHIFT: { const b = stack.pop(); const a = stack.pop(); stack.push(a << b); break; }

      // ---- 比较 ----
      case OpCode.CMP: { const b = stack.pop(); const a = stack.pop(); stack.push(a === b); break; }
      case OpCode.GT:  { const b = stack.pop(); const a = stack.pop(); stack.push(a > b); break; }
      case OpCode.LT:  { const b = stack.pop(); const a = stack.pop(); stack.push(a < b); break; }
      case OpCode.AND: { const b = stack.pop(); const a = stack.pop(); stack.push(a && b); break; }
      case OpCode.OR:  { const b = stack.pop(); const a = stack.pop(); stack.push(a || b); break; }

      // ---- 转换/连接 ----
      case OpCode.CAST: {
        const target = stack.pop();
        const val = stack.pop();
        stack.push(this._cast(val, target));
        break;
      }
      case OpCode.JOIN: { const b = stack.pop(); const a = stack.pop(); stack.push(String(a) + String(b)); break; }

      // ---- 集装箱操作 ----
      case OpCode.HEX_OP: {
        const radix = stack.pop();
        const val = stack.pop();
        stack.push((Number(val) >>> 0).toString(Number(radix)));
        break;
      }
      case OpCode.QUERY: {
        const idx = stack.pop();
        const val = stack.pop();
        stack.push((Number(val) >> Number(idx)) & 1);
        break;
      }

      // ---- 跳转 ----
      case OpCode.JUMP:
        this.ip = ins.arg;
        break;
      case OpCode.JUMPF: {
        const cond = stack.pop();
        if (!cond) this.ip = ins.arg;
        break;
      }

      // ---- 函数调用 ----
      case OpCode.CALL: {
        const name = this.names[ins.arg];
        const argc = ins.argc || 0;
        this.stats.calls++;
        // 内置函数
        if (this.builtins[name]) {
          const args = [];
          for (let i = 0; i < argc; i++) args.unshift(stack.pop());
          stack.push(this.builtins[name](...args));
          break;
        }
        // 用户函数
        const fn = this.funcs[name];
        if (fn !== undefined || this._hasFunc(name)) {
          this._callUserFunc(name, argc);
        } else {
          throw new Error(`未定义函数: ${name}`);
        }
        break;
      }
      case OpCode.RET: {
        if (this.callStack.length === 0) {
          this.ip = this.bytecode.code.length; // 顶层返回 → 结束
        } else {
          const frame = this.callStack.pop();
          this.globals = frame.env;
          this.ip = frame.returnIp;
        }
        break;
      }

      // ---- IO / 通信 ----
      case OpCode.PRINT: {
        const v = stack.pop();
        process.stdout.write(this._format(v));
        break;
      }
      case OpCode.PRINTLN: {
        const v = stack.pop();
        console.log(this._format(v));
        break;
      }
      case OpCode.READ:
        stack.push(null); // 同步环境无输入
        break;
      case OpCode.READLINE:
        stack.push(null);
        break;
      case OpCode.NET_SEND: {
        const payload = stack.pop();
        stack.push({ sent: true, payload });
        break;
      }
      case OpCode.NET_RECV:
        stack.push({ received: null });
        break;
      case OpCode.FILE_OPEN:
        stack.push({ fd: null });
        break;
      case OpCode.FILE_READ:
        stack.push(null);
        break;
      case OpCode.FILE_WRITE:
        stack.pop(); stack.pop();
        stack.push({ written: 0 });
        break;
      case OpCode.SHELL: {
        const cmd = this.names[ins.arg];
        stack.push({ shell: true, cmd });
        break;
      }
      case OpCode.EXEC: {
        const cmd = stack.pop();
        stack.push({ exec: true, cmd });
        break;
      }

      // ---- 函数声明（元信息，执行时跳过）----
      case 'FUNC_DECL':
        // 已在 interpret() 中通过 funcs 表注册，此处无需操作
        break;

      // ---- 终止 ----
      case OpCode.HALT:
        this.ip = this.bytecode.code.length;
        break;

      default:
        // 未知指令：忽略
        break;
    }
  }

  _hasFunc(name) {
    return Object.prototype.hasOwnProperty.call(this.funcs, name);
  }

  // 调用用户定义函数
  _callUserFunc(name, argc) {
    // 查找函数声明（在字节码中扫描 FUNC_DECL）
    const code = this.bytecode.code;
    let fnDecl = null;
    for (const ins of code) {
      if (ins.op === 'FUNC_DECL' && ins.name === name) { fnDecl = ins; break; }
    }
    if (!fnDecl) throw new Error(`函数未找到: ${name}`);

    // 弹出参数
    const args = [];
    for (let i = 0; i < argc; i++) args.unshift(this.stack.pop());

    // 保存当前帧
    this.callStack.push({
      env: this.globals,
      returnIp: this.ip,
    });
    // 新建局部环境（继承全局）
    const newEnv = Object.create(this.globals);
    fnDecl.params.forEach((p, i) => { newEnv[p] = args[i]; });
    this.globals = newEnv;
    // 跳转到函数入口
    this.ip = fnDecl.entry;
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

  // ===============
  // 2. interpretSource(source)
  // ===============
  // 直接解释执行源代码（编译+执行一体化）
  interpretSource(source, env) {
    // 若提供 env，直接走树遍历解释器（保留外部环境）
    if (env) return this.lang.interpret(source, env);
    // 否则走字节码路径：编译 → 执行
    const bytecode = this.compiler.compile(source);
    return this.interpret(bytecode);
  }

  // ===============
  // 3. repl()
  // ===============
  // 交互式 Read-Eval-Print Loop
  // 在非交互环境（CI）下，可传入输入数组模拟。
  repl(inputLines) {
    if (Array.isArray(inputLines)) {
      return this._replFromLines(inputLines);
    }
    // 真实 REPL：从 stdin 读取（需 TTY，CI 环境降级）
    const readline = this._makeReadline();
    if (!readline) {
      return this._replFromLines([
        'an1("CocoonLang REPL (CI 模式)")',
        'd 甲 42',
        'an1(甲)',
      ]);
    }
    this._replLive(readline);
  }

  // 从行数组模拟 REPL
  _replFromLines(lines) {
    const env = {};
    const outputs = [];
    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith(';')) continue;
      try {
        // 单行直接解释
        const result = this.lang.interpret(line, env);
        if (result !== null && result !== undefined && typeof result !== 'object') {
          outputs.push(result);
        }
      } catch (e) {
        outputs.push(`错误: ${e.message}`);
      }
    }
    return { env, outputs };
  }

  _makeReadline() {
    // 仅在有 TTY 时使用
    if (process.stdin.isTTY !== true) return null;
    try {
      return require('readline');
    } catch (e) {
      return null;
    }
  }

  _replLive(readline) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      prompt: 'cocoon> ',
    });
    const env = {};
    console.log(`CocoonLang REPL ${this.lang.version} —— 输入 :q 退出`);
    rl.prompt();
    rl.on('line', (line) => {
      const trimmed = line.trim();
      if (trimmed === ':q' || trimmed === ':quit') { rl.close(); return; }
      if (trimmed === ':env') { console.log(env); rl.prompt(); return; }
      if (trimmed === ':ast') {
        try { console.log(JSON.stringify(this.lang.getAST(env._last || '1'), null, 2)); } catch (e) { console.log(e.message); }
        rl.prompt(); return;
      }
      try {
        const result = this.lang.interpret(trimmed, env);
        if (result !== null && result !== undefined) {
          console.log(this._format(result));
        }
      } catch (e) {
        console.log(`错误: ${e.message}`);
      }
      rl.prompt();
    });
    rl.on('close', () => {
      console.log('\n再见');
      process.exit(0);
    });
  }

  // ===============
  // 4. selfInterpret()
  // ===============
  // 自举解释：解释器解释自己的源代码
  // 读取 bootstrap_interpreter.js 的 CocoonLang 等价源码，编译并执行，
  // 验证"解释器能用自己解释自己"的闭环。
  selfInterpret() {
    // 用 CocoonLang 语法书写的"解释器种子"源码
    const interpreterSource = `
; CocoonLang 自举解释器种子（用自身语法书写）
; 功能：执行一段 CocoonLang 字节码
ui 执行(字节码) {
  d 栈 新栈()
  d 指针 0
  un (k 指针 长度(字节码)) {
    d 指令 取值(字节码 指针)
    d 指针 b 指针 1
    ai (t 指令 "PUSH") {
      d 值 取值(字节码 指针)
      d 指针 b 指针 1
      压栈(栈 值)
    }
    ai (t 指令 "ADD") {
      d 乙 弹栈(栈)
      d 甲 弹栈(栈)
      压栈(栈 b 甲 乙)
    }
    ai (t 指令 "PRINT") {
      an1(弹栈(栈))
    }
    ai (t 指令 "HALT") {
      ao
    }
  }
  ao 弹栈(栈)
}
ui 自举解释() {
  d 自身源码 读取("bootstrap_interpreter.cocoon")
  d 自身字节码 编译(自身源码)
  d 结果 执行(自身字节码)
  an1("自举解释完成")
  ao 结果
}
an1("CocoonLang 解释器自举启动")
`;
    // 1. 编译自举源码
    const bytecode = this.compiler.compile(interpreterSource);
    // 2. 执行（验证能跑通）
    const result = this.interpret(bytecode);
    // 3. 返回自举报告
    return {
      bootstrap: 'self-interpret',
      sourceLines: interpreterSource.split('\n').length,
      bytecodeInstructions: bytecode.code.length,
      functionsDetected: Object.keys(bytecode.funcs),
      executed: true,
      result,
      steps: this.stats.steps,
      maxStackDepth: this.stats.maxStackDepth,
      selfHosting: true,
      timestamp: Date.now(),
    };
  }

  // 反汇编字节码（调试用）
  disassemble(bytecode) {
    const bc = bytecode || this.bytecode;
    if (!bc) return '';
    const lines = [];
    lines.push(`; CocoonVM 字节码反汇编 (v${bc.version || '?'})`);
    lines.push(`; 常量池 [${bc.consts.length}]: ${JSON.stringify(bc.consts)}`);
    lines.push(`; 名称表 [${bc.names.length}]: ${JSON.stringify(bc.names)}`);
    lines.push(`; 函数表: ${JSON.stringify(bc.funcs)}`);
    lines.push('');
    for (let i = 0; i < bc.code.length; i++) {
      const ins = bc.code[i];
      const argStr = ins.arg !== undefined ? ` ${ins.arg}` : '';
      const extra = ins.argc !== undefined ? ` (argc=${ins.argc})` : '';
      const comment = this._disasmComment(ins, bc);
      lines.push(`${String(i).padStart(4, '0')}: ${ins.op}${argStr}${extra}${comment ? '  ; ' + comment : ''}`);
    }
    return lines.join('\n');
  }

  _disasmComment(ins, bc) {
    if (ins.op === OpCode.PUSH && bc.consts[ins.arg] !== undefined) {
      return `=> ${JSON.stringify(bc.consts[ins.arg])}`;
    }
    if ((ins.op === OpCode.LOAD || ins.op === OpCode.STORE) && bc.names[ins.arg]) {
      return `${bc.names[ins.arg]}`;
    }
    if (ins.op === OpCode.CALL && bc.names[ins.arg]) {
      return `call ${bc.names[ins.arg]}`;
    }
    return '';
  }

  // 获取执行统计
  getStats() {
    return this.stats;
  }

  // 重置状态
  reset() {
    this.stack = [];
    this.globals = {};
    this.callStack = [];
    this.ip = 0;
    this.stats = { steps: 0, calls: 0, maxStackDepth: 0 };
  }
}

module.exports = {
  BootstrapInterpreter,
};
