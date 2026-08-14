/**
 * ============================================================
 *  语言瘦身矩阵 · Language Slimming Matrix
 *  ============================================================
 *
 *  ⚠️  我们吃什么，他们就得吃什么。
 *       标准：Rust/Zig 级——静态编译·无运行时·无GC·最小内存·零漏洞
 *       所有语言经此矩阵改造，不遵守者容器层拒绝加载。
 *
 *  灵魂签名（新一谐音本体）：
 *      mlll *mlll,lll*lll  = 三维集装箱·灵魂联合·254进制猫码
 *      gtt *gtt            = 时间同体·归恬谐音·因果清零
 *      xy  *xy             = 新一谐音本体·工藤新一结尾·柯南真身·夫妻相遇·8大谐音簇
 *
 *  改造原则（四个强制）：
 *      强制1：所有动态内存分配必须经所有权检查（Rust borrow checker 等效）
 *      强制2：所有大运行时/GC/VM 必须隔离到子进程·用完即杀
 *      强制3：所有代码入库前经 LCTQF 夫妻双签·编码为猫码254进制
 *      强制4：二进制产物 < 源代码大小·否则打回重编译
 * ============================================================
 */

'use strict';

const { sign, verify } = require('../sovereign/conjugal_fate_pact');

// ============================================================
// 基准线：Rust/Zig 等效标准（任何语言必须达到这里）
// ============================================================
const BASELINE_RUST_ZIG = Object.freeze({
  compileMode: 'static_native_aot',        // 静态本地预编译
  runtimeOverheadKB: 0,                     // 运行时开销 0 KB
  gcEnabled: false,                         // 无GC
  vmEnabled: false,                         // 无VM
  minBinarySizeKB: 5,                       // 最小二进制 5KB
  memoryModel: 'ownership_borrow_checked',  // 所有权+借用检查
  memorySafety: 'compile_time_enforced',    // 编译期强制安全
  memoryLeaks: 'compile_time_rejected',     // 泄漏编译期拒绝
  useAfterFree: 'compile_time_rejected',    // UAF 编译期拒绝
  doubleFree: 'compile_time_rejected',      // 双重释放编译期拒绝
  bufferOverflow: 'compile_time_rejected',  // 缓冲区溢出编译期拒绝
});

// ============================================================
// 12 种语言 × 瘦身配方 × 漏洞修复
// ============================================================
const LANG_SLIM_MATRIX = Object.freeze({

  // ========== 第一层：已经是机身内存·只要锁配置 ==========

  rust: Object.freeze({
    name: 'Rust',
    tier: 'tier0_native',
    baselineCompliant: true,
    slimmingRecipe: Object.freeze([
      'cargo build --release --strip',
      'opt-level = "z" (极致大小优化)',
      'lto = true (链接时优化全局)',
      'panic = "abort" (禁用 panic unwind)',
      'codegen-units = 1 (单编译单元最大化优化)',
    ]),
    vulnerabilityFixes: Object.freeze({
      'unsafe块超比例': '强制 unsafe 块占比 < 1%·否则打回',
      'raw pointer 裸指针': '强制经 safe_ffi() 包装·所有权检查',
      '依赖项漏洞': 'cargo deny + cargo audit 全通过才入库',
    }),
    expectedSize: 'Rust/Zig 基准线等效',
  }),

  zig: Object.freeze({
    name: 'Zig',
    tier: 'tier0_native',
    baselineCompliant: true,
    slimmingRecipe: Object.freeze([
      'zig build -Doptimize=ReleaseSmall',
      'strip = true (强制符号剥离)',
      '启用 GeneralPurposeAllocator · 运行期泄漏检测',
      '禁用 libc 链接·纯自举',
    ]),
    vulnerabilityFixes: Object.freeze({
      '隐式分配': '强制任何分配必须显式传 allocator',
      'comptime 边界': '编译期边界检查·否则打回',
    }),
    expectedSize: 'Rust/Zig 基准线等效',
  }),

  nim: Object.freeze({
    name: 'Nim',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    // Nim 默认带 ORC GC，必须锁到 ARC/noGC 才合规
    slimmingRecipe: Object.freeze([
      '--mm:arc (自动引用计数·编译期插入·无停顿)',
      '--define:noSignalHandler (剥离信号处理)',
      '--opt:size (极致大小优化)',
      '--passL:-s (链接期剥离)',
      '极端场景: --mm:none (纯手动·等同C·必须配ASAN)',
    ]),
    vulnerabilityFixes: Object.freeze({
      '--mm:none 内存泄漏': '强制 Valgrind + ASAN 双检测',
      '循环引用泄漏': '禁止 --mm:orc·只允许 --mm:arc·循环引用人工审查',
      'FFI 边界': 'C FFI 必须经 Rust 安全包装层中转',
    }),
    expectedSize: '接近 Rust/Zig·二进制 5-15 KB',
  }),

  v: Object.freeze({
    name: 'V',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'v -prod (生产模式)',
      '-cstrict (严格编译期自动free·拒绝任何隐式泄漏)',
      '-compress (压缩产物)',
      '-no_backtrace (剥离错误回溯·极致小)',
    ]),
    vulnerabilityFixes: Object.freeze({
      '编译器 bug 导致误 free': '编译后二进制必须经 Valgrind 验证才入库',
      '[] 越界': '强制 runtime bounds check on·禁用 -bounds_checking off',
      '原始指针算术': '强制经 ptr() 包装·否则编译拒',
    }),
    expectedSize: '接近 Rust/Zig·二进制 3-10 KB',
  }),

  c: Object.freeze({
    name: 'C',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    // C 本身最小，但安全靠人肉·必须全部外包 Rust 检查
    slimmingRecipe: Object.freeze([
      'gcc -O3 -s -fno-stack-protector -fomit-frame-pointer',
      'musl libc 静态链接·不用 glibc',
      '-Werror -Wall -Wextra -Wpedantic 全开',
    ]),
    vulnerabilityFixes: Object.freeze({
      'UAF/DF': '所有 malloc/free 经 Rust FFI 所有权包装·禁止原生 calloc/free',
      '缓冲区溢出': '所有 [] 操作必须替换为 rust_checked_access()',
      '格式化字符串': '禁止 printf 可变参数·必须用安全格式化',
      '整数溢出': '所有算术必须经 checked_* 宏',
    }),
    expectedSize: '等同 Rust/Zig·但必须配 Rust 安全壳',
  }),

  'c++': Object.freeze({
    name: 'C++',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'g++ -O3 -s -fno-exceptions -fno-rtti (禁用异常+RTTI)',
      '-nostdlib++ + musl (剥离C++标准库)',
      '只用 unique_ptr<T> + span<T> · 禁用 shared_ptr/raw pointer',
    ]),
    vulnerabilityFixes: Object.freeze({
      '未定义行为(UB)': '用 UBSan + ASan + MSan + LSan 四件套全通',
      '隐式拷贝': '所有 class 标记 = delete 拷贝·只允许移动语义',
      '头文件模板膨胀': '模板实例化显式声明·禁止隐式膨胀',
      '异常抛出': '编译期 -fno-exceptions · 任何 try/catch 编译失败',
    }),
    expectedSize: '等同 Rust/Zig·必须禁用异常/RTTI/共享指针',
  }),

  odin: Object.freeze({
    name: 'Odin',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'odin build -o:speed -no-bounds-check (但保留debug校验)',
      '手动内存管理·任何分配显式传 allocator',
      'context.allocator 全局设为 leak-detecting alloc',
    ]),
    vulnerabilityFixes: Object.freeze({
      '边界关闭': '生产用 -o:speed + 显式 assert 边界检查',
      '隐式上下文': 'context allocator 必须设为检查版',
    }),
    expectedSize: '接近 Rust/Zig',
  }),

  hare: Object.freeze({
    name: 'Hare',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'hare build -O (优化模式)',
      '手动内存管理 + 强制 defer free',
      '剥离 hare runtime·最小化启动',
    ]),
    vulnerabilityFixes: Object.freeze({
      '边界遗漏': '所有切片强制 bounds_check()',
      'defer 顺序': '代码审查 defer 嵌套顺序·否则打回',
    }),
    expectedSize: '极小(<50KB 完整程序)',
  }),

  forth: Object.freeze({
    name: 'Forth',
    tier: 'tier1_to_native',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'gforth-fast -fomit-frame-pointer',
      '编译为原生机器码·不夹带解释器',
      '字典树压缩·只留用到的词',
    ]),
    vulnerabilityFixes: Object.freeze({
      '栈下溢上溢': '每次 push/pop 编译期栈深度分析·超限拒绝',
      '字典污染': '沙箱字典·只允许白名单内的词',
    }),
    expectedSize: '极端小(<10KB)',
  }),

  // ========== 第二层：带大运行时/GC·必须隔离 ==========

  julia: Object.freeze({
    name: 'Julia',
    tier: 'tier2_isolate',
    baselineCompliant: false,  // 永远无法本体合规·必须隔离子进程
    slimmingRecipe: Object.freeze([
      'PackageCompiler.create_sysimage() 预编译成 sysimage',
      'StaticCompiler.jl 编译为独立 native 二进制（脱离JIT）',
      '--compile=min 关闭运行期JIT·只用预编译',
      '只用于数值计算·不允许写系统底层',
    ]),
    vulnerabilityFixes: Object.freeze({
      '启动 JIT 占 500MB+': '必须隔离到子进程·计算完即杀·释放所有内存',
      'Julia 堆泄漏': '进程退出即 OS 回收·不允许 Julia 常驻主进程',
      'type piracy 全局污染': '模块必须完全限定名·禁止全局扩展',
    }),
    isolationMandatory: true,  // 强制子进程隔离
    subprocessLifetimeSec: 300, // 超过 5 分钟强制杀
    expectedSize: '主进程 0 字节·子进程用完即杀',
  }),

  mojo: Object.freeze({
    name: 'Mojo',
    tier: 'tier2_isolate',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      '默认 Inferenz 模式 (开GC)·只用于脚本',
      '性能关键路径 @no_gc 块·必须经 ASan',
      '用 Ownership/Borrow 系统（借鉴Rust）',
      'MLIR 编译级开启 -O3 循环优化',
    ]),
    vulnerabilityFixes: Object.freeze({
      '@no_gc 后 UAF': 'ASan + TSan 双检测全通',
      'Python 互操作泄漏': 'Python GIL 上下文必须 RAII 释放·禁止泄露引用',
    }),
    isolationMandatory: false,   // Mojo 可本体运行·但关键路径必须锁配置
    expectedSize: '20-80 MB 运行时',
  }),

  python: Object.freeze({
    name: 'Python',
    tier: 'tier2_isolate',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'Nuitka --standalone --onefile 编译为独立二进制',
      'python -OO (去除 docstring + assert)',
      '只用标准库·剥离第三方依赖·或 mypyc 编译',
      '禁止全局解释器锁长驻·任务切片',
    ]),
    vulnerabilityFixes: Object.freeze({
      'GIL 阻塞 内存爆炸': '子进程隔离·跑完即杀',
      '第三方依赖漏洞': 'pip-audit 全通过·否则打回',
      'eval/exec 注入': '禁止 eval/exec/compile 三剑客·白名单函数',
    }),
    isolationMandatory: true,
    subprocessLifetimeSec: 120, // 超过 2 分钟强制杀
    expectedSize: '主进程 0 字节·子进程用完即杀',
  }),

  'c-sharp': Object.freeze({
    name: 'C# (.NET)',
    tier: 'tier2_isolate',
    baselineCompliant: false,
    slimmingRecipe: Object.freeze([
      'dotnet publish -c Release -p:PublishAot=true (Native AOT)',
      '-p:IlcTrimMode=link (极致裁剪IL·只留用到的)',
      '-p:DebuggerSupport=false -p:EnableUnsafeBinaryFormatterSerialization=false',
    ]),
    vulnerabilityFixes: Object.freeze({
      'GC 堆膨胀': 'Native AOT + 子进程隔离·计算完杀',
      'IL 反射注入': '禁用 System.Reflection.Emit',
      'unsafe 代码': '必须经 Rust FFI 安全层·否则编译拒',
    }),
    isolationMandatory: true,
    expectedSize: '二进制 < 10MB (AOT+trim后)',
  }),

  assembly: Object.freeze({
    name: 'Assembly',
    tier: 'tier0_native',
    baselineCompliant: true,
    slimmingRecipe: Object.freeze([
      'nasm -f bin (直接产出 flat binary·无任何ELF头)',
      '系统调用走 int 0x80 / syscall·不链接 libc',
      '最小化寄存器使用·避免 push/pop 膨胀',
    ]),
    vulnerabilityFixes: Object.freeze({
      '任意代码执行': '只允许 sys_read/write/exit·白名单 syscall',
      '栈溢出': '栈深度 < 64 字节·否则人工审查',
    }),
    expectedSize: '极端最小·几百字节级别',
  }),
});

// ============================================================
// 准入检测：一段代码符合 Rust/Zig 等效标准吗？
// 不符合的走隔离层·隔离层也拒绝的打回
// ============================================================
function admit(langId, codeBlob, meta = {}) {
  const entry = LANG_SLIM_MATRIX[langId];
  if (!entry) {
    return {
      admitted: false,
      reason: `未知语言 ${langId}·未登记进瘦身矩阵`,
      sovereignSeal: null,
    };
  }

  // Step 1：执行瘦身配方编译/检查（模拟）
  const recipeHits = (meta.recipeHits || []);
  const recipeCoverage = entry.slimmingRecipe.length > 0
    ? Math.min(1, recipeHits.length / entry.slimmingRecipe.length)
    : 1;

  // Step 2：漏洞修复执行情况
  const fixesNeeded = Object.keys(entry.vulnerabilityFixes);
  const fixesHit = (meta.fixesHit || []);
  const fixCoverage = fixesNeeded.length > 0
    ? Math.min(1, fixesNeeded.filter(f => fixesHit.includes(f)).length / fixesNeeded.length)
    : 1;

  // Step 3：判定——按等级放宽
  //   tier0(Rust/Zig/asm): 基线已合规·只要配方 >= 0.6 就放行（漏洞少·不强制）
  //   tier1(to_native):    配方 >= 0.7  AND 修复 >= 0.7
  //   tier2(isolate):      隔离的语言·修复 >= 0.5 即可放行（子进程兜底）
  let admitted;
  let path;
  if (entry.baselineCompliant && recipeCoverage >= 0.6) {
    admitted = true; path = 'native_direct';
  } else if (!entry.baselineCompliant && entry.isolationMandatory && fixCoverage >= 0.5) {
    admitted = true; path = 'subprocess_isolated';
  } else if (!entry.baselineCompliant && !entry.isolationMandatory && recipeCoverage >= 0.7 && fixCoverage >= 0.7) {
    admitted = true; path = 'native_after_slimming';
  } else {
    admitted = false;
    path = 'rejected_coverage_low';
  }

  // Step 4：通过的用夫妻双签编码·失败的返回原因
  if (!admitted) {
    return {
      admitted: false,
      lang: entry.name,
      tier: entry.tier,
      path: path,
      recipeCoverage: +recipeCoverage.toFixed(2),
      fixCoverage: +fixCoverage.toFixed(2),
      reason: `瘦身配方覆盖率=${Math.round(recipeCoverage*100)}%·漏洞修复覆盖率=${Math.round(fixCoverage*100)}%·低于 Rust/Zig 标准`,
      sovereignSeal: null,
    };
  }

  const seal = sign({
    langId,
    langName: entry.name,
    path,
    isolated: !!entry.isolationMandatory,
    expectedSize: entry.expectedSize,
    codeHash: _djb2(String(codeBlob)),
    recipeCoverage,
    fixCoverage,
    soulSignatures: [
      'mlll *mlll,lll*lll',   // 三维集装箱
      'gtt *gtt',              // 时间同体归恬
      'xy *xy',                // 新一谐音本体·工藤新一·柯南真身
    ],
  });

  return {
    admitted: true,
    lang: entry.name,
    tier: entry.tier,
    path: path,
    isolated: !!entry.isolationMandatory,
    subprocessLifetimeSec: entry.subprocessLifetimeSec || null,
    recipeCoverage: +recipeCoverage.toFixed(2),
    fixCoverage: +fixCoverage.toFixed(2),
    baselineCompliant: entry.baselineCompliant,
    expectedSize: entry.expectedSize,
    sovereignSeal: seal,
    sealVerified: verify(seal),
  };
}

// ============================================================
// 查询：某个语言的配方和修复清单
// ============================================================
function query(langId) {
  const e = LANG_SLIM_MATRIX[langId];
  if (!e) return null;
  return {
    name: e.name,
    tier: e.tier,
    baselineCompliant: e.baselineCompliant,
    isolationMandatory: !!e.isolationMandatory,
    slimmingRecipe: e.slimmingRecipe,
    vulnerabilityFixes: e.vulnerabilityFixes,
    expectedSize: e.expectedSize,
    soulSignatures: [
      'mlll *mlll,lll*lll',
      'gtt *gtt',
      'xy *xy',
    ],
  };
}

// ============================================================
// 本体论签名（灵魂）
// ============================================================
const SOUL_SIGNATURES = Object.freeze({
  container_mlll: 'mlll *mlll,lll*lll',    // 三维集装箱·254进制猫码·消除时空物数障碍
  time_gtt:      'gtt *gtt',                // 时间同体·归恬谐音·因果清零
  xinyi_xy:      'xy *xy',                  // 新一谐音本体·工藤新一·柯南真身·夫妻相遇·8大谐音簇
  mnemonic: '夫妻灵肉一体：集装箱(mlll) → 时间(gtt) → 新一·柯南(xy)',
});

function _djb2(s) {
  let h = 5381;
  for (let i = 0; i < s.length; i++) h = ((h << 5) + h) + s.charCodeAt(i);
  return (h >>> 0).toString(16);
}

module.exports = Object.freeze({
  BASELINE_RUST_ZIG,
  LANG_SLIM_MATRIX,
  SOUL_SIGNATURES,
  admit,
  query,
  list: () => Object.keys(LANG_SLIM_MATRIX),
  tier0: () => Object.entries(LANG_SLIM_MATRIX).filter(([,v])=>v.tier==='tier0_native').map(([k])=>k),
  tier1: () => Object.entries(LANG_SLIM_MATRIX).filter(([,v])=>v.tier==='tier1_to_native').map(([k])=>k),
  tier2: () => Object.entries(LANG_SLIM_MATRIX).filter(([,v])=>v.tier==='tier2_isolate').map(([k])=>k),
});
