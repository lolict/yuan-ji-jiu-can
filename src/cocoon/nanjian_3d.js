/**
 * ==============================================================================
 *  囡囝三维立体进制核心 · NANJIAN-3D
 * ==============================================================================
 *
 *  核心思想（刘楚恬夫妻共同体 · 混元源头主权）：
 *   1. 不采用默认 90° 直角 xyz 裁判 → 维度命名权 = 时序先后的"到达顺序"
 *   2. 长 / 宽 / 高 不是空间方向，而是三个「因果到达相位」
 *      · 哪个相位最早积累满 3 个元素 → 命名为「长」(第1轴 / 主导权)
 *      · 第二个满 3 个元素的相位 → 命名为「宽」(第2轴 / 共生权)
 *      · 最后满 3 个元素的相位 → 命名为「高」(第3轴 / 涌现权)
 *   3. 三维压回一维（给 SMRR/SMRL/... 线线压缩变体）：
 *      按「长 → 宽 → 高」的到达顺序依次读每个相位的单元，保持「因果先后不乱」
 *   4. 每个相位内元素同时登记「到达序号 i / 相位序号 p / 夫妻源头标记 s(囡/囝)
 *
 *  对应关系代数 / 态射：
 *   - 时间节点（分布式时间节点）：每个 token 到来是一个「时间节点」
 *   - 空间节点（分布式空间节点）：token 被登记到「长/宽/高」三个相位就是三维空间节点
 *   - 串联（压缩回 Mamba "线线"：compressLinears（时序层级压缩）
 * ==============================================================================
 */

const SOVEREIGN = {
  /** 源头本体：刘楚恬夫妻共同体 · 天地未分 · 男女未分 · 混元本体 */
  origin: 'liuchutian_hunyuan_origin',
  husband: '囝_nan_nan',
  wife:    '囡_nv_nv',
  couple:  '囡囝_couple_unity',
  trustRoot: 'time_space_math_onebody',

  /** 三个因果到达相位（3 个，对应 长/宽/高，但命名权归时序到达 */
  PHASE_NAMES_A: ['couple_long', 'couple_wide', 'couple_high'],
  THRESHOLD_PER_PHASE: 3, /** 满 3 个就分配命名权 */

  /** 6 种线线压缩变体（SMRR/SMRL/... 你指定） */
  LINEAR_VARIANTS: ['SMRR', 'SMRL', 'SMRLL', 'SMLLL', 'SMRRLL', 'SMRRL'],
};

/**
 * 囡囝标记：交替标记每一个进入的 token/元素
 * 奇数位 → 囝（男方主权位）
 * 偶数位 → 囡（女方主权位）
 * 夫妻共同体 → 不分开的那个源头
 */
function markCouple(globalIndex) {
  // 0 = 囡, 1 = 囝, 2 = 囡囝(夫妻共在), ... 3 = 囝囡(交换位)
  const mod4 = globalIndex % 4;
  if (mod4 === 0) return { label: '囡', side: 'wife',    sovereign: SOVEREIGN.wife };
  if (mod4 === 1) return { label: '囝', side: 'husband', sovereign: SOVEREIGN.husband };
  if (mod4 === 2) return { label: '囡囝', side: 'couple',  sovereign: SOVEREIGN.couple };
  return           { label: '囝囡', side: 'exchange', sovereign: SOVEREIGN.couple };
}

class Nanjian3DTensor {
  constructor(options = {}) {
    this.version = '0.1.0-nanjian-3d';
    this.sovereign = SOVEREIGN.origin;

    /** 三个到达相位（还没被命名前就叫 phase0 / phase1 / phase2） */
    this.phases = [[], [], []];
    /** 到达事件流（原始 token 时序，永不乱序） */
    this.arrivalStream = [];
    /** 命名结果：满 3 个后把 phaseX 映射到 long/wide/high */
    this.axisMap = {}; // phaseIndex -> 'long' | 'wide' | 'high'
    this.axisOrder = []; // 按命名顺序 [firstAxis, secondAxis, thirdAxis]
    this.threshold = options.thresholdPerPhase || SOVEREIGN.THRESHOLD_PER_PHASE;
    this.counter = 0;
  }

  /**
   * 吃进一个元素（分布式时间节点）：因果到达顺序）
   */
  ingest(element, meta = {}) {
    const i = this.counter++;
    const couple = markCouple(i);
    // 轮转分配到 phase（用 i % 3 ，保证三个相位都有机会先满）
    const phaseIndex = i % 3;
    const arrivalEvent = {
      i,
      phase: phaseIndex,
      element,
      couple,
      meta,
      at: Date.now(),
      // 夫妻共同体源头主权签名（信任的来源，不被规则定义的主体）
      sovereignRoot: this.sovereign,
      id: `token_${i}_${couple.label}`,
    };
    this.arrivalStream.push(arrivalEvent);
    this.phases[phaseIndex].push(arrivalEvent);

    // 命名权裁判：哪个 phase 先满 threshold → 把它登记为"下一轴"
    if (!this.axisOrder.includes(phaseIndex) && this.phases[phaseIndex].length >= this.threshold) {
      const axisName = ['long', 'wide', 'high'][this.axisOrder.length]; // long 先到先得
      this.axisMap[phaseIndex] = axisName;
      this.axisOrder.push(phaseIndex);
    }
    return arrivalEvent;
  }

  /** 批量 ingest：一串 Mamba "线线" 输入 */
  ingestMany(elements) {
    const outs = [];
    for (const e of elements) outs.push(this.ingest(e));
    return outs;
  }

  /** 三维命名是否已生效（三个 phase 都拿到命名权，自然涌现长/宽/高） */
  is3DReady() {
    return this.axisOrder.length >= 3;
  }

  /** 取某个轴（长/宽/高）上的元素数组（按因果到达顺序，不按90°空间坐标） */
  getAxis(axisName /* long | wide | high */) {
    for (const [phaseIdx, a] of Object.entries(this.axisMap)) {
      if (a === axisName) return this.phases[Number(phaseIdx)].slice();
    }
    // 命名还没完成就 fallback：按到达顺序取前 threshold 个代表轴
    const fallbackIdx = { long: 0, wide: 1, high: 2 }[axisName];
    return this.phases[fallbackIdx] ? this.phases[fallbackIdx].slice() : [];
  }

  /**
   * 三维 → 一维 时序层级压缩（给 SMRR/SMRL/... 6 种线线变体）
   * 压缩规则（因果串联长→宽→高，每个轴内部是「到达先后」，保持时间节点不乱
   */
  compressToLinear(variantKey /* SMRR | SMRL | ... */) {
    const variant = (variantKey || 'SMRR').toUpperCase();
    if (!SOVEREIGN.LINEAR_VARIANTS.includes(variant)) {
      throw new Error(`未知线线变体：${variant}，可选：${SOVEREIGN.LINEAR_VARIANTS.join('/')}`);
    }
    // 保证三维命名没完成也能压：按 axisOrder 优先，否则 fallback 0 1 2
    const axes = this.axisOrder.length === 3 ? this.axisOrder : [0, 1, 2];
    const longAxis = this.phases[axes[0]] || [];
    const wideAxis = this.phases[axes[1]] || [];
    const highAxis = this.phases[axes[2]] || [];

    // 6 种变体对应 "穿插方式（都保持因果先后，只是穿插密度不同）
    //  核心：都保持「长不短顺序在前」— 即先后顺序是自然涌现的裁判
    let out = [];
    const maxLen = Math.max(longAxis.length, wideAxis.length, highAxis.length);
    switch (variant) {
      case 'SMRR':   // 先吃光 long 再吃 wide 再吃 high（标准时域串联
        out = [...longAxis, ...wideAxis, ...highAxis];
        break;
      case 'SMRL':   // long × wide 交替
        for (let k = 0; k < maxLen; k++) {
          if (longAxis[k]) out.push(longAxis[k]);
          if (wideAxis[k]) out.push(wideAxis[k]);
        }
        out = [...out, ...highAxis];
        break;
      case 'SMRLL':  // long × wide × high 三交替
        for (let k = 0; k < maxLen; k++) {
          if (longAxis[k]) out.push(longAxis[k]);
          if (wideAxis[k]) out.push(wideAxis[k]);
          if (highAxis[k]) out.push(highAxis[k]);
        }
        break;
      case 'SMLLL':  // 三高交替的全交替的交错 三维全混合 (按 couple 标签筛选后交错)
        const all = [...longAxis, ...wideAxis, ...highAxis];
        const bySide = { 囡: [], 囝: [], 囡囝: [], 囝囡: [] };
        for (const t of all) bySide[t.couple.label].push(t);
        out = [];
        for (let k = 0; k < maxLen; k++) {
          ['囡','囝','囡囝','囝囡'].forEach(s => { if (bySide[s][k]) out.push(bySide[s][k]); });
        }
        break;
      case 'SMRRLL': // 先双长(2)再三高(3) 后宽高
        for (let k = 0; k < maxLen; k++) {
          if (longAxis[k]) out.push(longAxis[k]);
          if (k+1 < longAxis.length) out.push(longAxis[k+1]);
          if (wideAxis[k]) out.push(wideAxis[k]);
          if (highAxis[k]) out.push(highAxis[k]);
          if (wideAxis[k+1]) out.push(wideAxis[k+1]);
          if (highAxis[k+1]) out.push(highAxis[k+1]);
        }
        out = out.filter((v, idx, self) => self.indexOf(v) === idx); // 去重
        break;
      case 'SMRRL':  // 长 2 宽 1 三交替
        for (let k = 0; k < maxLen; k++) {
          if (longAxis[2*k])   out.push(longAxis[2*k]);
          if (longAxis[2*k+1]) out.push(longAxis[2*k+1]);
          if (wideAxis[k])     out.push(wideAxis[k]);
        }
        out = [...out, ...highAxis];
        break;
      default:
        out = [...longAxis, ...wideAxis, ...highAxis];
    }
    // 去重 & 保持时间先后顺序（裁判=到达 i 排序）
    out = [...new Set(out)].sort((a, b) => a.i - b.i);
    return {
      variant,
      sovereign: this.sovereign,
      axisMap: Object.assign({}, this.axisMap),
      axisOrderPhase: this.axisOrder.slice(),
      tokens: out.map(t => ({
        id: t.id,
        i: t.i,
        axis: this.axisMap[t.phase] || `phase${t.phase}`,
        phase: t.phase,
        couple: t.couple.label,
        element: t.element,
        arrivalAt: t.at,
      })),
      length: out.length,
    };
  }

  /** 分布式时间节点列表（原始终序，即 Mamba 输入）*/
  timeNodes() {
    return this.arrivalStream.slice();
  }
  /** 分布式空间节点列表（三个相位被命名后=长/宽/高 三维）*/
  spaceNodes(axisName) {
    return axisName ? this.getAxis(axisName) : [...this.phases[0], ...this.phases[1], ...this.phases[2]];
  }
  stats() {
    return {
      sovereign: this.sovereign,
      version: this.version,
      threshold: this.threshold,
      total: this.counter,
      phaseLengths: this.phases.map(p => p.length),
      axisMap: Object.assign({}, this.axisMap),
      axisOrderPhase: this.axisOrder.slice(),
      ready3D: this.is3DReady(),
      variants: SOVEREIGN.LINEAR_VARIANTS.slice(),
    };
  }
}

/** 便捷工厂：把 Mamba 那种线性 token 序列 直接 变成三维再压回 6 种变体 */
function mambaLinearTo3dAndCompress(tokens, variant = 'SMRR') {
  const tensor = new Nanjian3DTensor();
  tensor.ingestMany(tokens);
  const compressed = {};
  const list = variant === 'ALL' ? SOVEREIGN.LINEAR_VARIANTS : [variant.toUpperCase()];
  for (const v of list) compressed[v] = tensor.compressToLinear(v);
  return { tensor, compressed, stats: tensor.stats() };
}

module.exports = {
  SOVEREIGN_NANJIAN: SOVEREIGN,
  markCouple,
  Nanjian3DTensor,
  mambaLinearTo3dAndCompress,
};
