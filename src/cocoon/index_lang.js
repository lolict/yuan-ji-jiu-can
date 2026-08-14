/**
 * ================================================
 * CocoonLang 自举编程语言工具链 - 总入口
 * ================================================
 *
 * 统一导出 CocoonLang 语言工具链的全部模块：
 *   - cocoon_lang.js          : 语言核心（词法/语法/AST/字节码/主类）
 *   - bootstrap_compiler.js   : 自举编译器（AST→字节码/优化/WASM/交叉编译）
 *   - bootstrap_interpreter.js: 自举解释器（栈式VM/REPL/自举解释）
 *   - bootstrap_translator.js : 自举翻译器（多语言互译/二进制翻译）
 *   - bootstrap_sensor.js     : 自举感应器（环境/蛹茧/壳/系统/网络/安全感知）
 *
 * 用法：
 *   const Cocoon = require('./index_lang.js');
 *   const lang = new Cocoon.CocoonLang();
 *   lang.interpret('an1("Hello CocoonLang")');
 *
 *   const compiler = new Cocoon.BootstrapCompiler();
 *   const bytecode = compiler.compile(source);
 *
 *   const interp = new Cocoon.BootstrapInterpreter();
 *   interp.interpret(bytecode);
 *
 *   const translator = new Cocoon.BootstrapTranslator();
 *   translator.translateJS(source);
 *
 *   const sensor = new Cocoon.BootstrapSensor();
 *   sensor.senseAll();
 */

'use strict';

// ========================
// 一、语言核心
// ========================
const cocoonLang = require('./cocoon_lang.js');

// ========================
// 二、自举编译器
// ========================
const bootstrapCompiler = require('./bootstrap_compiler.js');

// ========================
// 三、自举解释器
// ========================
const bootstrapInterpreter = require('./bootstrap_interpreter.js');

// ========================
// 四、自举翻译器
// ========================
const bootstrapTranslator = require('./bootstrap_translator.js');

// ========================
// 五、自举感应器
// ========================
const bootstrapSensor = require('./bootstrap_sensor.js');

// ========================
// 六、统一导出
// ========================

// 语言核心
const {
  CocoonLang,
  Tokenizer, Token, TokenType,
  Parser,
  OpCode,
  SHENGMU_OP, YUNMU_FLOW, SHENGDIAO_IO, RADIX_TABLE,
  ASTNode, ContainerRef, VariableRef, Literal, Operation,
  FlowControl, Communication, ShellCommand, FuncDef, Program,
} = cocoonLang;

// 编译器
const { BootstrapCompiler, Target, OptLevel } = bootstrapCompiler;

// 解释器
const { BootstrapInterpreter } = bootstrapInterpreter;

// 翻译器
const { BootstrapTranslator, Languages, BinaryArch, X86Decoder, ARMDecoder, RISCVDecoder } = bootstrapTranslator;

// 感应器
const { BootstrapSensor } = bootstrapSensor;

// ========================
// 七、工具链版本与元信息
// ========================
const TOOLCHAIN = {
  name: 'CocoonLang',
  version: '0.1.0-bootstrap',
  description: '自举编程语言 CocoonLang 及其工具链',
  // 自举状态：所有组件均可用 CocoonLang 自身源码书写
  selfHosting: true,
  // 组件清单
  components: {
    core: 'cocoon_lang.js',
    compiler: 'bootstrap_compiler.js',
    interpreter: 'bootstrap_interpreter.js',
    translator: 'bootstrap_translator.js',
    sensor: 'bootstrap_sensor.js',
    index: 'index_lang.js',
  },
  // 语言特性
  features: {
    syntax: '声母(运算) + 韵母(流程) + 声调(IO)',
    variableNames: '汉字/符号',
    containerOps: '直接操作进制本身',
    bootstrap: '编译器/解释器/翻译器均可自举',
  },
};

// ========================
// 八、便捷工厂函数
// ========================

// 创建完整工具链实例
function createToolchain(options = {}) {
  const lang = new CocoonLang();
  const compiler = new BootstrapCompiler(options.optLevel || OptLevel.O1);
  const interpreter = new BootstrapInterpreter({ compiler });
  const translator = new BootstrapTranslator();
  const sensor = new BootstrapSensor();
  return { lang, compiler, interpreter, translator, sensor };
}

// 一行执行 CocoonLang 源码
function run(source, env) {
  const lang = new CocoonLang();
  return lang.interpret(source, env);
}

// 编译并执行
function compileAndRun(source) {
  const compiler = new BootstrapCompiler();
  const interpreter = new BootstrapInterpreter({ compiler });
  const bytecode = compiler.compile(source);
  return interpreter.interpret(bytecode);
}

// ========================
// 九、模块导出
// ========================

module.exports = {
  // 版本与元信息
  TOOLCHAIN,

  // 语言核心类与组件
  CocoonLang,
  Tokenizer, Token, TokenType,
  Parser,
  OpCode,
  // 关键字表
  SHENGMU_OP, YUNMU_FLOW, SHENGDIAO_IO, RADIX_TABLE,
  // AST 节点类型
  ASTNode, ContainerRef, VariableRef, Literal, Operation,
  FlowControl, Communication, ShellCommand, FuncDef, Program,

  // 自举编译器
  BootstrapCompiler,
  Target,        // 交叉编译目标平台枚举
  OptLevel,      // 优化级别枚举

  // 自举解释器
  BootstrapInterpreter,

  // 自举翻译器
  BootstrapTranslator,
  Languages,     // 支持的目标语言枚举
  BinaryArch,    // 二进制架构枚举
  X86Decoder, ARMDecoder, RISCVDecoder,

  // 自举感应器
  BootstrapSensor,

  // 便捷工厂
  createToolchain,
  run,
  compileAndRun,
};
