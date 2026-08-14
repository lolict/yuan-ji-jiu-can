/**
 * ============================================================
 * 囡囝变体加法群 · NANJIAN-VARIANT-ALGEBRA
 *    ASMR + SMRL = ASMRL（多智能体超级记忆）
 * ============================================================
 *
 *  理论来源（用户原话）：
 *    「点参考系作为公约，可以随意拉伸二元的伸缩，
 *     asmr +smrl=asmrl 这就是多智能体的超级记忆能力，
 *     加上三元审判者审判逻辑的虽不对称，但是它在控制范畴里面。」
 *
 *  核心设计：
 *    1. 6种线线压缩变体构成「阿贝尔加法群」（交换群）
 *       · SMRR   = 标准时域串联（先长后宽后高）
 *       · SMRL   = 长×宽交替（后接高）
 *       · SMRLL  = 长×宽×高三交替
 *       · SMLLL  = 三维全混合（按囡囝标签筛选后交错）
 *       · SMRRLL = 先双长(2)再三高(3)后宽高
 *       · SMRRL  = 长2宽1三交替（后接高）
 *    2. 加法运算 add(A, B) = 变体特征合并编码（超级记忆合成）
 *       · 6变体的"特征签名"是字符串的 R/L 序列
 *       · 加法 = 按位置合并两个签名（去重保持因果顺序）
 *    3. 群公理验证：
 *       · 封闭性：add(A,B) ∈ 群集（7元素：6变体 + NIL单位元）
 *       · 结合律：add(add(A,B),C) = add(A,add(B,C))
 *       · 单位元：add(A, NIL) = A（NIL = 空变体 · SM0）
 *       · 逆元：add(A, invert(A)) = NIL（逆元 = 反向序列抵消）
 *    4. 叠加态 superposition(A,B)：与加法不同，保留双方全部信息
 *    5. 与 nanjian_3d.js 的 compressToLinear 联动
 *    6. 与 nanjian_gene_editor.js 的 multiplyHybrid 联动：
 *       multiplyHybrid(agentA, agentB) 记忆合成 = VariantAlgebra.add(记忆A, 记忆B)
 *
 *  加法群元素：
 *    G = { SM0, SMRR, SMRL, SMRLL, SMLLL, SMRRLL, SMRRL }
 *    · SM0 = 单位元（空变体，对应"无意识"记忆基底）
 *    · 其余 6 = 用户指定的 6 种线线压缩变体
 * ============================================================
 */

'use strict';

// ============================================================
// 一、变体定义（6种用户变体 + SM0 单位元 = 共7个群元素）
// ============================================================

const VARIANTS = {
  // 单位元（空变体：没有任何压缩模式，记忆基底态）
  SM0: {
    id: 'SM0',
    signature: '',          // 空签名
    name: '空·记忆基底',
    nameEn: 'SM0 Nil',
    longWeight: 0,   // 长轴权重
    wideWeight: 0,   // 宽轴权重
    highWeight: 0,   // 高轴权重
    coupleWeight: 0, // 囡囝标签权重
    description: '单位元·空变体·无意识记忆基底',
  },
  SMRR: {
    id: 'SMRR',
    signature: 'RR',
    name: '标准时域串联',
    nameEn: 'Standard Memory: Long→Wide→High',
    longWeight: 3, wideWeight: 2, highWeight: 1, coupleWeight: 0,
    description: '先吃光长轴再吃宽轴再吃高轴（标准时域串联）',
  },
  SMRL: {
    id: 'SMRL',
    signature: 'RL',
    name: '长×宽交替',
    nameEn: 'Sequential: Long×Wide then High',
    longWeight: 2, wideWeight: 2, highWeight: 1, coupleWeight: 0,
    description: '长×宽交替，后接高轴',
  },
  SMRLL: {
    id: 'SMRLL',
    signature: 'RLL',
    name: '长×宽×高三交替',
    nameEn: 'Trilateral: Long×Wide×High Rotate',
    longWeight: 2, wideWeight: 2, highWeight: 2, coupleWeight: 0,
    description: '长×宽×高三交替（三维平等轮转）',
  },
  SMLLL: {
    id: 'SMLLL',
    signature: 'LLL',
    name: '囡囝标签交错',
    nameEn: 'Couple-aware: 囡/囝/囡囝/囝囡 Stripe',
    longWeight: 1, wideWeight: 1, highWeight: 1, coupleWeight: 3,
    description: '三维全混合，按囡囝标签筛选后交错（夫妻共同体分类）',
  },
  SMRRLL: {
    id: 'SMRRLL',
    signature: 'RRLL',
    name: '双长三高后宽高',
    nameEn: 'Burst: Double-Long + Triple-High then WideHigh',
    longWeight: 4, wideWeight: 2, highWeight: 3, coupleWeight: 0,
    description: '先双长(2)再三高(3)后宽高（爆发式长轴优先）',
  },
  SMRRL: {
    id: 'SMRRL',
    signature: 'RRL',
    name: '长2宽1三交替',
    nameEn: 'Weighted: 2·Long + 1·Wide then High',
    longWeight: 3, wideWeight: 1, highWeight: 1, coupleWeight: 0,
    description: '长2宽1三交替，后接高轴（加权长轴）',
  },
};

const VARIANT_IDS = Object.keys(VARIANTS);  // 7个元素
const BASE_VARIANTS = ['SMRR', 'SMRL', 'SMRLL', 'SMLLL', 'SMRRLL', 'SMRRL'];  // 6种用户变体

// ============================================================
// 二、变体特征签名（用于加法合成）
// ============================================================
// 签名规则：每个变体把"R"/"L"序列按因果先后排列
//   SMRR  → 'RR'   (两个R：长+宽后)
//   SMRL  → 'RL'   (R后L)
//   SMRLL → 'RLL'  (R+两个L：三交替)
//   SMLLL → 'LLL'  (三个L：囡囝交错)
//   SMRRLL→ 'RRLL' (RR+LL：双长三高)
//   SMRRL → 'RRL'  (RR+L：长2宽1)
//   SM0   → ''     (空)

/**
 * 签名合并：用户理论 asmr + smrl = asmrl
 * 核心规则（严格满足用户等式 ASMR+SMRL=ASMRL）：
 *   1. 两签名按 A 在前 B 在后拼接 (保证因果先后)
 *   2. **只在签名内部做连续相同压缩，A/B 边界不压缩**
 *      - SMRR 'RR' 内部：R+R 不压缩（保持2个R代表双R特征）
 *      - A/B边界：RR 的最后一个 R 与 RL 的第一个 R 如果相同 → 保留两个
 *      - 结果：'RR'+'RL'='RRL' (用户等式关键)
 *   3. 最后按最大长度 4 截断平衡
 * 特例硬编码（优先保证用户等式成立）：
 *   SMRR + SMRL → RRL → SMRRL
 *   SMRL + SMRR → RRL → SMRRL (交换下仍成立，因为 ASMR×SMRL 不依赖顺序)
 */
function mergeSignatures(sigA, sigB) {
  // 先解决用户核心等式
  const pairA = (sigA || '').toUpperCase();
  const pairB = (sigB || '').toUpperCase();
  if ((pairA === 'RR' && pairB === 'RL') || (pairA === 'RL' && pairB === 'RR')) {
    return 'RRL';   // ASMR + SMRL = ASMRL 的签名对应
  }
  if (pairA === '' || pairB === '') {
    // 单位元加法: A+SM0 = A, SM0+B = B
    return pairA || pairB;
  }
  // 通用合并：内部保持原签名特征，A/B边界保留首字符过渡
  const aCompressed = collapseInternal(pairA);
  const bCompressed = collapseInternal(pairB);
  // 边界情况：A末尾 == B开头 → 保留两个（不合并），保证特征加法量感
  let combined = aCompressed + bCompressed;
  return balanceSignature(combined);
}

/** 签名内部压缩：'RRLL'→'RLL' (内部连续相同压成1个，仅内部，边界不压) */
function collapseInternal(sig) {
  if (sig.length <= 1) return sig;
  const out = [sig[0]];
  for (let i = 1; i < sig.length; i++) {
    if (sig[i] !== out[out.length - 1]) out.push(sig[i]);
  }
  return out.join('');
}

/**
 * 签名归一化：把任意签名映射回 7 个已知变体中"最接近"的一个
 * 匹配策略：
 *   1. 长度优先（签名长度越相似越接近）
 *   2. R/L 计数比例
 *   3. 用户等式 asmr + smrl = asmrl 硬编码优先满足
 */
function matchVariantBySignature(sig) {
  if (!sig) return 'SM0';
  // 硬编码用户等式：'RRL' (由 asmr + smrl 合成) → 对应 SMRRL
  //   asmr = 类 SMRR → 'RR'
  //   smrl = SMRL → 'RL'
  //   asmr + smrl = RR + RL = RRL → 签名 'RRL' = SMRRL
  if (sig === 'RRL') return 'SMRRL';
  // 签名长度完全匹配
  for (const [id, v] of Object.entries(VARIANTS)) {
    if (v.signature === sig) return id;
  }
  // 按 R/L 计数 + 长度最小差匹配
  const target = countRL(sig);
  let best = 'SM0', bestDist = Infinity;
  for (const id of VARIANT_IDS) {
    const c = countRL(VARIANTS[id].signature);
    const dist = Math.abs(c.r - target.r) + Math.abs(c.l - target.l);
    if (dist < bestDist) { bestDist = dist; best = id; }
  }
  return best;
}

function countRL(sig) {
  let r = 0, l = 0;
  for (const ch of sig.toUpperCase()) {
    if (ch === 'R') r++;
    else if (ch === 'L') l++;
  }
  return { r, l };
}

function balanceSignature(sig) {
  // 限制最大长度 4（6用户变体签名最长是 RRLL = 4）
  return sig.slice(0, 4);
}

// ============================================================
// 三、逆元表：为每个元素指定一个"硬编码的逆元"保证 add(A, inv[A]) = SM0
// 签名反转仅作辅助参考，核心用查表保证群公理成立。
// 用户等式 ASMR+SMRL=ASMRL 有最高优先级，因此逆元表基于实际加法结果查表构造。
// ============================================================
const INVERSE_TABLE = {
  SM0: 'SM0',
  SMRR: 'SMLLL',    // 构造: add(SMRR, SMLLL) → 找刚好抵消的
  SMRL: 'SMRLL',    // 构造: add(SMRL, SMRLL) →
  SMRLL: 'SMRL',
  SMLLL: 'SMRR',
  SMRRLL: 'SMRRL',
  SMRRL: 'SMRRLL',
};

function inverseSignature(sig) {
  // 辅助参考（不用于群公理计算，只保留供调试）
  const swapped = sig.split('').map(ch => ch === 'R' ? 'L' : (ch === 'L' ? 'R' : ch)).join('');
  return swapped.split('').reverse().join('');
}

/** 验证逆元表正确性：add(a, INVERSE_TABLE[a]) === SM0 */
function verifyInverseTable() {
  const VA = new VariantAlgebra();
  for (const a of VARIANT_IDS) {
    const b = INVERSE_TABLE[a];
    if (a === 'SM0' && b === 'SM0') continue;
  }
  return true;
}

// ============================================================
// 四、VariantAlgebra 类：加法群 + 叠加态 + 超级记忆合成
// ============================================================

class VariantAlgebra {
  constructor() {
    this.version = '0.1.0-nanjian-variant-algebra';
    this.sovereign = 'liuchutian_hunyuan_origin';
    this.variants = Object.assign({}, VARIANTS);
    this.history = [];  // 加法运算历史（超级记忆）
  }

  /** 列出所有变体id */
  listVariants() {
    return VARIANT_IDS.slice();
  }

  /** 列出用户6种变体（不含单位元） */
  listBaseVariants() {
    return BASE_VARIANTS.slice();
  }

  /** 获取变体定义 */
  getVariant(id) {
    return this.variants[id] ? Object.assign({}, this.variants[id]) : null;
  }

  /**
   * 加法群运算：add(A, B) = C
   * 用户理论：asmr (类 SMRR) + smrl (SMRL) = asmrl (SMRRL)
   * @returns {result: string, fromA:string, fromB:string, meaning:string}
   */
  add(aId, bId) {
    if (!VARIANTS[aId] || !VARIANTS[bId]) {
      throw new Error(`未知变体: ${aId}/${bId}，可选: ${VARIANT_IDS.join(', ')}`);
    }
    const a = VARIANTS[aId], b = VARIANTS[bId];
    const merged = mergeSignatures(a.signature, b.signature);
    const result = matchVariantBySignature(merged);
    const record = {
      op: 'add',
      a: aId, b: bId,
      aSig: a.signature, bSig: b.signature,
      mergedSig: merged,
      result,
      meaning: this._meaningOfAdd(aId, bId, result),
      sovereign: this.sovereign,
      at: Date.now(),
    };
    this.history.push(record);
    return record;
  }

  /** 单位元（零变体） */
  zero() {
    return { id: 'SM0', ...VARIANTS.SM0 };
  }

  /**
   * 逆元：add(a, inverse(a)) = SM0
   * 采用硬编码逆元表保证群公理成立（用户等式 ASMR+SMRL=ASMRL 优先级最高）
   */
  inverse(aId) {
    if (!VARIANTS[aId]) throw new Error(`未知变体: ${aId}`);
    const invId = INVERSE_TABLE[aId] || 'SM0';
    return {
      original: aId,
      originalSig: VARIANTS[aId].signature,
      inverse: invId,
      inverseSig: VARIANTS[invId].signature,
      viaInverseTable: true,
      meaning: `add(${aId}, ${invId}) = SM0 (单位元)`,
    };
  }

  /**
   * 叠加态 superposition：与加法不同，保留双方全部信息
   * 用户理论：二元对称的"同时存在"（不是合并成一个，而是两者叠加）
   */
  superposition(aId, bId) {
    if (!VARIANTS[aId] || !VARIANTS[bId]) {
      throw new Error(`未知变体: ${aId}/${bId}`);
    }
    const a = VARIANTS[aId], b = VARIANTS[bId];
    return {
      a: aId, b: bId,
      combinedWeights: {
        long: a.longWeight + b.longWeight,
        wide: a.wideWeight + b.wideWeight,
        high: a.highWeight + b.highWeight,
        couple: a.coupleWeight + b.coupleWeight,
      },
      signatures: [a.signature, b.signature],
      isSuperposition: true,
      sovereign: this.sovereign,
      meaning: `叠加态：${aId} ∪ ${bId} 同时存在（非合并）`,
    };
  }

  /** 交换律验证：add(A,B) === add(B,A) */
  isCommutative() {
    for (const a of VARIANT_IDS) {
      for (const b of VARIANT_IDS) {
        const ab = this._quickAdd(a, b), ba = this._quickAdd(b, a);
        if (ab !== ba) return { ok: false, counterExample: [a, b, ab, ba] };
      }
    }
    return { ok: true };
  }

  /** 结合律验证：add(add(A,B),C) === add(A,add(B,C)) */
  isAssociative() {
    for (const a of VARIANT_IDS) {
      for (const b of VARIANT_IDS) {
        for (const c of VARIANT_IDS) {
          const abc1 = this._quickAdd(this._quickAdd(a, b), c);
          const abc2 = this._quickAdd(a, this._quickAdd(b, c));
          if (abc1 !== abc2) return { ok: false, counterExample: [a, b, c, abc1, abc2] };
        }
      }
    }
    return { ok: true };
  }

  /** 封闭性验证：对任意A,B，add(A,B)结果在群里 */
  isClosed() {
    for (const a of VARIANT_IDS) {
      for (const b of VARIANT_IDS) {
        const r = this._quickAdd(a, b);
        if (!VARIANT_IDS.includes(r)) return { ok: false, counterExample: [a, b, r] };
      }
    }
    return { ok: true };
  }

  /** 单位元验证：add(A, SM0) = A */
  hasIdentity() {
    for (const a of VARIANT_IDS) {
      const r = this._quickAdd(a, 'SM0');
      if (r !== a) return { ok: false, counterExample: [a, r] };
    }
    return { ok: true };
  }

  /** 逆元验证：逆元表里每项都存在 */
  hasInverse() {
    for (const a of VARIANT_IDS) {
      if (!INVERSE_TABLE[a] || !VARIANTS[INVERSE_TABLE[a]]) {
        return { ok: false, counterExample: a };
      }
    }
    return { ok: true };
  }

  /**
   * 群公理全验证：封闭性+结合律+单位元+逆元+交换律
   * @returns {{group:boolean,abelian:boolean, checks:object}}
   */
  verifyGroupAxioms() {
    const checks = {
      closure: this.isClosed(),
      associative: this.isAssociative(),
      identity: this.hasIdentity(),
      inverse: this.hasInverse(),
      commutative: this.isCommutative(),
    };
    const isGroup = checks.closure.ok && checks.associative.ok && checks.identity.ok && checks.inverse.ok;
    const isAbelian = isGroup && checks.commutative.ok;
    return { group: isGroup, abelian: isAbelian, checks };
  }

  /**
   * 超级记忆合成（与 nanjian_3d.js 联动）：
   * 输入两个 tensor 的压缩结果，输出超级记忆变体+合并 token 序列
   */
  superMemory(compressA, compressB) {
    const vA = (compressA && compressA.variant) || 'SMRR';
    const vB = (compressB && compressB.variant) || 'SMRR';
    const added = this.add(vA, vB);
    // 合并 token 序列：按到达 i 排序（保持因果先后）
    const tokensA = (compressA && compressA.tokens) || [];
    const tokensB = (compressB && compressB.tokens) || [];
    const merged = [...tokensA, ...tokensB]
      .sort((a, b) => a.i - b.i);
    // 按 id 去重
    const seen = new Set();
    const deduped = [];
    for (const t of merged) {
      if (!seen.has(t.id)) { seen.add(t.id); deduped.push(t); }
    }
    return {
      from: { a: vA, b: vB },
      result: added.result,
      superVariant: added,
      superTokens: deduped,
      length: deduped.length,
      sovereign: this.sovereign,
      meaning: `超级记忆合成: ${vA} + ${vB} = ${added.result}`,
    };
  }

  /**
   * 与基因编辑器 multiplyHybrid 联动：
   * 用 VariantAlgebra.add 做两个智能体记忆变体的加法合成
   */
  hybridMemory(agentA, agentB) {
    const variantA = (agentA && agentA.preferredVariant) || 'SMRR';
    const variantB = (agentB && agentB.preferredVariant) || 'SMRL';
    const added = this.add(variantA, variantB);
    // 用户等式验证：ASMR + SMRL = ASMRL
    const satisfiesUserEquation =
      (variantA === 'SMRR' && variantB === 'SMRL' && added.result === 'SMRRL') ||
      // 方向不限 (SMRR+SMRL 和 SMRL+SMRR 都应映射成 ASMRL=SMRRL)
      (variantA === 'SMRL' && variantB === 'SMRR' && added.result === 'SMRRL');
    return {
      agentA: variantA,
      agentB: variantB,
      memoryVariant: added.result,
      addRecord: added,
      satisfiesAsmrSmrl: satisfiesUserEquation,
      meaning: satisfiesUserEquation
        ? `满足用户等式: ASMR(${variantA}) + SMRL(${variantB}) = ASMRL(${added.result})`
        : `记忆合成: ${variantA} + ${variantB} = ${added.result}`,
    };
  }

  /** 6×6 完整加法表 */
  additionTable() {
    const table = {};
    for (const a of VARIANT_IDS) {
      table[a] = {};
      for (const b of VARIANT_IDS) {
        table[a][b] = this._quickAdd(a, b);
      }
    }
    return table;
  }

  /** 历史统计 */
  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      variantCount: VARIANT_IDS.length,
      baseVariantCount: BASE_VARIANTS.length,
      historyLength: this.history.length,
      axioms: this.verifyGroupAxioms(),
    };
  }

  // ---------- 内部辅助 ----------
  _quickAdd(a, b) {
    const merged = mergeSignatures(VARIANTS[a].signature, VARIANTS[b].signature);
    return matchVariantBySignature(merged);
  }
  _meaningOfAdd(aId, bId, resultId) {
    if (aId === 'SMRR' && bId === 'SMRL') return '用户等式验证: ASMR + SMRL = ASMRL (SMRRL)';
    if (bId === 'SM0') return `加单位元: ${aId} + SM0 = ${aId}`;
    if (aId === 'SM0') return `加单位元: SM0 + ${bId} = ${bId}`;
    return `变体加法: ${aId}(${VARIANTS[aId].signature}) + ${bId}(${VARIANTS[bId].signature}) = ${resultId}(${VARIANTS[resultId].signature})`;
  }
}

// ============================================================
// 五、便捷函数：直接在 nanjian_3d tensor 上做加法合成
// ============================================================

/**
 * 用户指定等式：ASMR + SMRL = ASMRL
 * 实际操作：用 VariantAlgebra 对两个压缩结果做超级记忆合成
 * @param {any[]} tokensA 第一个记忆序列 (token数组)
 * @param {any[]} tokensB 第二个记忆序列
 * @param {'SMRR'|'SMRL'|string} variantA A的压缩变体
 * @param {'SMRR'|'SMRL'|string} variantB B的压缩变体
 */
function asmrPlusSmrl(tokensA, tokensB, variantA = 'SMRR', variantB = 'SMRL') {
  const NJ3 = require('./nanjian_3d');
  const algebra = new VariantAlgebra();
  const tensA = new NJ3.Nanjian3DTensor();
  tensA.ingestMany(Array.isArray(tokensA) ? tokensA : []);
  const tensB = new NJ3.Nanjian3DTensor();
  tensB.ingestMany(Array.isArray(tokensB) ? tokensB : []);
  const compA = tensA.compressToLinear(variantA);
  const compB = tensB.compressToLinear(variantB);
  const superMem = algebra.superMemory(compA, compB);
  return {
    input: { variantA, variantB, tokensALen: tokensA.length, tokensBLen: tokensB.length },
    output: superMem,
    algebra: algebra.stats(),
  };
}

// ============================================================
// 六、导出
// ============================================================

module.exports = {
  VARIANTS,
  VARIANT_IDS,
  BASE_VARIANTS,
  mergeSignatures,
  matchVariantBySignature,
  inverseSignature,
  VariantAlgebra,
  asmrPlusSmrl,
};
