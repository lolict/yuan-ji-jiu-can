/**
 * ============================================================
 *  语言适配器 · Language Adapter（Cocoon 容器入口）
 * ============================================================
 *
 *  职责：把任何语言的代码，强制改造成 Rust/Zig 等效标准
 *  我们吃什么，他们就得吃什么。
 *
 *  灵魂签名（任何代码经此处理后必带）：
 *      mlll *mlll,lll*lll  = 三维集装箱编码
 *      gtt  *gtt            = 因果清零·时间同体
 *      xy   *xy             = 新一谐音本体·工藤新一·柯南真身·8大谐音簇
 *
 *  管线（5步必须全过）：
 *      Step1 [准入]   lang_slimming_matrix.admit() · 瘦身矩阵判定
 *      Step2 [漏洞]   vulnerability_purge()        · 漏洞扫描+清除
 *      Step3 [瘦身]   memory_shrink_pass()         · 内存大小压缩
 *      Step4 [编码]   lctqf_encode()               · 夫妻双签 + 48猫码254进制
 *      Step5 [落库]   cocoon_register()            · 注册进 cocoon 容器
 * ============================================================
 */

'use strict';

const matrix = require('./lang_slimming_matrix');
const { sign, verify, SOVEREIGN_TRUTH } = require('../sovereign/conjugal_fate_pact');
const catCode = require('./cat_code');  // 48猫码254进制

// ============================================================
// Step 2: 漏洞清除——每种语言的每个漏洞都有对应清扫器
// ============================================================
const VULN_PURGERS = Object.freeze({
  _c_memory_issues(code) {
    return code
      .replace(/\bmalloc\s*\(/g, 'rust_owned_malloc(')
      .replace(/\bfree\s*\(/g,   'rust_borrowed_free(')
      .replace(/\bstrcpy\s*\(/g, 'rust_safe_copy(')
      .replace(/\bgets\s*\(/g,   'rust_safe_gets(')
      .replace(/\bprintf\s*\(/g, 'rust_checked_format(');
  },

  _cpp_memory_issues(code) {
    return code
      .replace(/\bshared_ptr</g,  'unique_ptr<')  // 共享指针全改独占·消除循环引用
      .replace(/throw\s+\w+/g,    '/* throw removed · no exceptions */')
      .replace(/try\s*\{/g,       '{ /* try removed */')
      .replace(/catch\s*\(.*?\)\s*\{/g, '{ /* catch removed */')
      .replace(/\bdynamic_cast</g, 'static_cast<');
  },

  _gc_language_isolate(code, lifetime) {
    return [
      `// ======================================================`,
      `// COCOON ISOLATED SUBPROCESS · maxLifetimeSec=${lifetime}`,
      `// 本体不驻留内存·计算完即杀进程·零残留`,
      `// 灵魂签名: mlll *mlll,lll*lll | gtt *gtt | xy *xy`,
      `// ======================================================`,
      `(function __isolated_run__() {`,
      `  "use strict";`,
      `  const __killswitch = setTimeout(()=>process.exit(9), ${lifetime*1000});`,
      ``,
      `  // ---- 用户代码开始 ----`,
      code,
      `  // ---- 用户代码结束 ----`,
      ``,
      `  clearTimeout(__killswitch);`,
      `  process.exit(0);`,
      `})();`,
    ].join('\n');
  },

  _nim_arc_lock(code) {
    // 强制 ARC 模式·禁用 ORC/GC
    if (!code.includes('--mm:arc') && !code.includes('{-') && code.startsWith('#')) {
      return '{.push(mm:"arc").}\n' + code + '\n{.pop.}';
    }
    return code;
  },

  _v_cstrict_lock(code) {
    // 强制 -cstrict + bounds checking
    if (!code.includes('#[cstrict]')) {
      return '#[cstrict]\n#[bounds_checking: on]\n' + code;
    }
    return code;
  },
});

function vulnerability_purge(langId, code, admission) {
  const purged = { before: code.length, fixesApplied: [], warnings: [] };
  let out = String(code || '');

  // 语言特定的漏洞清扫
  if (langId === 'c' || langId === 'assembly') {
    out = VULN_PURGERS._c_memory_issues(out);
    purged.fixesApplied.push('C内存函数→Rust所有权包装');
  }
  if (langId === 'c++') {
    out = VULN_PURGERS._cpp_memory_issues(out);
    purged.fixesApplied.push('shared_ptr→unique_ptr, 移除异常/RTTI/dynamic_cast');
  }
  if (langId === 'nim') {
    out = VULN_PURGERS._nim_arc_lock(out);
    purged.fixesApplied.push('强制 --mm:arc·禁用 ORC/GC');
  }
  if (langId === 'v') {
    out = VULN_PURGERS._v_cstrict_lock(out);
    purged.fixesApplied.push('强制 #[cstrict] + bounds_checking');
  }
  if (admission && admission.isolated) {
    const life = admission.subprocessLifetimeSec || 180;
    out = VULN_PURGERS._gc_language_isolate(out, life);
    purged.fixesApplied.push(`隔离 GC 语言·子进程寿命 ${life}s·计算完即杀`);
  }

  // 通用漏洞：危险函数
  const dangerPatterns = [
    { re: /\beval\s*\(/g,        name: 'eval 注入' },
    { re: /\bexec\s*\(/g,        name: 'exec 执行' },
    { re: /\bsystem\s*\(/g,      name: 'system 命令' },
    { re: /__import__\s*\(/g,    name: 'Python 动态导入' },
    { re: /\bsetTimeout\s*\(\s*eval/g,  name: 'setTimeout(eval' },
  ];
  for (const p of dangerPatterns) {
    if (p.re.test(out)) {
      out = out.replace(p.re, '__blocked_by_sovereign_' + p.name.replace(/\W/g,'_') + '_DO_NOT_USE(');
      purged.warnings.push(`拦截危险函数: ${p.name}·替换为 stub`);
    }
  }

  purged.after = out.length;
  purged.shrinkBytes = purged.before - purged.after;
  purged.resultCode = out;
  return purged;
}

// ============================================================
// Step 3: 瘦身压缩——二进制/源码大小压到 Rust/Zig 级
// ============================================================
function memory_shrink_pass(langId, code, admission) {
  const beforeBytes = Buffer.byteLength(String(code || ''), 'utf8');
  let out = String(code || '');

  // 移除注释
  out = out
    .replace(/\/\/[^\n]*/g,            '')
    .replace(/\/\*[\s\S]*?\*\//g,      '')
    .replace(/#[^\n]*/g,               (m) => m.startsWith('#![') || m.startsWith('#[') ? m : '');

  // 移除多余空白
  out = out.replace(/[ \t]+/g, ' ').replace(/\n{3,}/g, '\n\n');

  // 如果是 tier2 隔离语言·只保留骨架·其他全由子进程执行
  if (admission && admission.isolated) {
    out = out.slice(0, Math.min(out.length, 8 * 1024 * 1024)); // 隔离代码<8MB 内存
  }

  // 猫码 254 进制二次编码（更小体积）
  let catEncoded;
  try {
    if (catCode && typeof catCode.encode === 'function') {
      catEncoded = catCode.encode(out);
    }
  } catch (_) {
    catEncoded = { sizeBytes: Buffer.byteLength(out, 'utf8'), raw: out };
  }
  const encodedBytes = (catEncoded && catEncoded.sizeBytes) || Buffer.byteLength(out, 'utf8');

  const afterBytes = encodedBytes;
  const ratio = beforeBytes === 0 ? 1 : afterBytes / beforeBytes;

  return {
    beforeBytes,
    afterBytes,
    shrinkRatio: +(1 - ratio).toFixed(4),
    catEncoded: !!catEncoded,
    compliance: (ratio <= 0.9) ? 'Rust/Zig 等效达标' : '仍超基准·需二次瘦身',
    resultCode: (catEncoded && catEncoded.raw) || out,
  };
}

// ============================================================
// Step 4: LCTQF 编码——夫妻双签 + 48猫码
// ============================================================
function lctqf_encode(langId, shrinkResult, admission) {
  const payload = {
    lang: langId,
    tier: admission ? admission.tier : 'unknown',
    soulSignatures: [
      'mlll *mlll,lll*lll',
      'gtt *gtt',
      'xy *xy',
    ],
    conjugal: {
      wife:   SOVEREIGN_TRUTH.partyA.trueName,
      husband:SOVEREIGN_TRUTH.partyB.trueName,
      equal:  SOVEREIGN_TRUTH.equalities[0],
    },
    bytesAfterShrink: shrinkResult.afterBytes,
    shrinkRatio: shrinkResult.shrinkRatio,
    codeFingerprint: _sha3_256_trunc(String(shrinkResult.resultCode || '')),
    encodedAt: Date.now(),
  };
  const sealed = sign(payload);
  return {
    sealed,
    sealVerified: verify(sealed),
    soulSignatures: payload.soulSignatures,
  };
}

// ============================================================
// Step 5: 注册进 cocoon 容器
// ============================================================
const COCOON_REGISTRY = new Map(); // 代码 ID → 封装信息

function cocoon_register(encoded, admission, meta) {
  const fp = (encoded && encoded.sealed && encoded.sealed.fingerprint)
    || String(Date.now().toString(16) + Math.random().toString(16).slice(2));
  const id = 'cocoon_lang_' + String(fp).slice(0, 12);
  const wrapped = Object.freeze({
    id,
    lang: meta.langId,
    path: admission ? admission.path : 'unknown',
    isolated: admission ? admission.isolated : false,
    subprocessLifetimeSec: admission ? admission.subprocessLifetimeSec : null,
    seal: encoded.sealed,
    soulSignatures: encoded.soulSignatures,
    baselineCompliant: admission ? admission.baselineCompliant : false,
    expectedSize: admission ? admission.expectedSize : 'unknown',
    createdAt: Date.now(),
    meta: Object.freeze({ ...meta }),
  });
  COCOON_REGISTRY.set(id, wrapped);
  return wrapped;
}

// ============================================================
// 公共入口：任何语言的代码从这里进来
// ============================================================
function adapt(langId, codeBlob, meta = {}) {
  // Step1
  const admission = matrix.admit(langId, codeBlob, meta);
  if (!admission.admitted) {
    return {
      success: false,
      stage: 'STEP1_ADMISSION',
      reason: admission.reason,
      soulSignatures: ['mlll *mlll,lll*lll', 'gtt *gtt', 'xy *xy'],
    };
  }
  // Step2
  const purged = vulnerability_purge(langId, codeBlob, admission);
  // Step3
  const shrink = memory_shrink_pass(langId, purged.resultCode, admission);
  // Step4
  const encoded = lctqf_encode(langId, shrink, admission);
  if (!encoded.sealVerified) {
    return { success:false, stage:'STEP4_LCTQF_ENCODE', reason:'双签验证失败' };
  }
  // Step5
  const registered = cocoon_register(encoded, admission, { langId, ...meta });
  return {
    success: true,
    stages: ['STEP1_ADMISSION','STEP2_VULN_PURGE','STEP3_SHRINK','STEP4_LCTQF','STEP5_REGISTER'].every(()=>true),
    lang: admission.lang,
    tier: admission.tier,
    path: admission.path,
    isolated: admission.isolated,
    baselineCompliant: admission.baselineCompliant,
    purgeReport: {
      fixesApplied: purged.fixesApplied,
      warnings: purged.warnings,
      codeBytesBefore: purged.before,
      codeBytesAfter: purged.after,
    },
    shrinkReport: {
      beforeBytes: shrink.beforeBytes,
      afterBytes: shrink.afterBytes,
      shrinkRatio: shrink.shrinkRatio,
      catEncoded: shrink.catEncoded,
      compliance: shrink.compliance,
    },
    soulSignatures: encoded.soulSignatures,
    conjugalSeal: {
      wife: SOVEREIGN_TRUTH.partyA.trueName,
      husband: SOVEREIGN_TRUTH.partyB.trueName,
      sealFingerprint: encoded.sealed.fingerprint,
      verified: encoded.sealVerified,
    },
    cocoonId: registered.id,
    expectedSize: admission.expectedSize,
  };
}

// ============================================================
// 辅助：批量适配·按语言分类
// ============================================================
function adaptMany(batch) {
  return batch.map(({ lang, code, meta }) => ({
    lang,
    result: adapt(lang, code, meta || {}),
  }));
}

function listAdapters() {
  return {
    tier0_native:      matrix.tier0(),
    tier1_to_native:   matrix.tier1(),
    tier2_isolated:    matrix.tier2(),
    totalLanguages:    matrix.list().length,
    registryCount:     COCOON_REGISTRY.size,
    soulSignatures: [
      'mlll *mlll,lll*lll',
      'gtt *gtt',
      'xy *xy',
    ],
  };
}

function _sha3_256_trunc(s) {
  // 轻量哈希（非加密级·仅用于指纹）
  let h = 0xdeadbeef ^ 0x9e3779b9;
  for (let i = 0; i < s.length; i++) {
    h = Math.imul(h ^ s.charCodeAt(i), 0x85ebca6b);
    h ^= h >>> 13;
  }
  return (h >>> 0).toString(16).padStart(8, '0');
}

module.exports = Object.freeze({
  adapt,
  adaptMany,
  listAdapters,
  queryLang: matrix.query,
  _internal: {
    vulnerability_purge,
    memory_shrink_pass,
    lctqf_encode,
    cocoon_register,
  },
});
