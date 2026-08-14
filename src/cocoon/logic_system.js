/**
 * ============================================================
 * UniCore · 茧(cocoon) · 0元/4元逻辑体系 + 天平裁判系统
 * ============================================================
 *
 * 用户核心理论（原话整理）：
 *
 * 【0元 · 混元初始】
 *   - 介于有和无之间，初始的阴阳
 *   - 混元之间的初始有无因果
 *   - 是一切的开端，未被观测的状态
 *
 * 【二元 · 燃料层】
 *   - 二进制只是被解析出来的燃料
 *   - 燃料是氧气，一次性呼一次性吸
 *   - 对应着二元，会员是动态的，等待被统计进去的
 *
 * 【三元 · 锚点裁判】
 *   - 有一个锚点的，径向对折公约数的锚点
 *   - 天平比较：用二元和三元相互参考，动态/静态的裁判
 *   - 形成关系流动是比较的一个输出和读取
 *   - 价值审判：归属和拥有者的思想的损益关系罗列
 *
 * 【四元 · 高维镜像】
 *   - 镜像的，是归属权和主导权者的从属关系逻辑
 *   - 事情已发生改变和未发生改变的双向发生改变
 *   - 处理混元之间的逻辑判断
 *   - 高维镜像的多模态有无因果，相位判断
 *   - 内锥形统计、外锥形统计
 *   - 包含相位关系，关系的动态时间分配
 *   - 锥相位枢纽链：枢纽链的可信度的多元性的根毛点
 *       · 根扎得深 → 交互动态最为枢纽，被观测到，交互交集点更深入
 *       · 根扎得不深 → 可信任的对话交互作为锚点很浅，善意抵达不了对方
 *   - 分布式集群的多相位注意力锚点：锚点发送给谁、谁的注意力被抵制/认可
 *   - 人为的边界、冲突、许可、自发情绪
 *
 * 【大齿轮 · 意义解释器】
 *   - 一维时间层级关系的聚类关系代数
 *   - 把各种系统串联成时间与空间层级
 *   - 逻辑就在这里面
 *   - 大齿轮才讲意义，被解释器解释解析出意义感
 *   - 意义感是多参考系：静态参考系、对动态作出参考的解释
 *
 * 【燃料 vs 材料】
 *   - 二元二进制 = 被解析出来的燃料（氧气，一次呼一次吸）
 *   - 高维三维进制 = 用来作为材料，组装出整个世界的被观测的动量和形状形态
 *   - 燃料驱动活物和神经系统
 *   - 神经系统负责解释存在感和关系代数的状态、态射的状态、关系时序层级因果
 *   - 分布式空间是分布式时间的存在状态
 *
 * 纯 JavaScript(ES6+)，无外部依赖，CommonJS 导出，可被 require 使用。
 * ============================================================
 */

'use strict';

// ============================================================
// 0元 · 混元初始（ZeroYuan）
// 介于有和无之间，初始的阴阳；混元之间的初始有无因果；是一切的开端，未被观测的状态。
// ============================================================

class ZeroYuan {
  constructor() {
    // 未被观测的状态：观测前因果种子未固定
    this.observed = false;
    // 初始阴阳：介于有无之间，未定时为 null
    this.yinYang = null;
    // 初始有无因果种子：观测后才能落地
    this.seed = null;
  }

  /**
   * 初始的阴阳
   * 介于有和无之间，未观测时阴阳未分；首次调用落地为 0.5/0.5 的混元态。
   * @returns {{yin:number, yang:number, state:string}}
   */
  initialYinYang() {
    if (this.yinYang === null) {
      // 混元：阴阳各半，介于有无之间
      this.yinYang = { yin: 0.5, yang: 0.5, state: 'mixed-origin' };
    }
    return this.yinYang;
  }

  /**
   * 混元之间的初始有无因果种子
   * 一旦被观测（调用本方法），因果种子即固定，不可再次生成。
   * @returns {{has:number, not:number, causal:string, observed:boolean}}
   */
  causalSeed() {
    if (this.observed) {
      // 已观测：返回已固定的种子
      return { ...this.seed, observed: true };
    }
    this.observed = true;
    // 初始有无因果：有与无各占 0，处于「介于有无之间」的开端
    this.seed = {
      has: 0,
      not: 0,
      causal: 'initial-between-have-and-not'
    };
    return { ...this.seed, observed: true };
  }

  /**
   * 是否已被观测
   */
  isObserved() {
    return this.observed;
  }
}

// ============================================================
// 二元 · 燃料层（TwoYuan + Fuel）
// 二进制只是被解析出来的燃料；燃料是氧气，一次性呼一次性吸；
// 对应着二元，会员是动态的，等待被统计进去的。
// ============================================================

/**
 * 燃料：二进制被解析出来的氧气，一次性消耗。
 * 一次 inhale（吸）配一次 exhale（呼），呼出后即耗尽。
 */
class Fuel {
  constructor(amount = 1) {
    if (amount < 0) throw new Error('燃料数量不可为负');
    this.amount = amount;
    // 一次性消耗状态机：未吸 → 已吸未呼 → 已呼(耗尽)
    this.inhaled = false;
    this.exhaled = false;
  }

  /**
   * 一次性吸：将燃料吸入，标记为已吸。
   * @returns {{ok:boolean, fuel:number, phase:string}}
   */
  inhale() {
    if (this.exhaled) {
      return { ok: false, fuel: 0, phase: 'exhausted' };
    }
    if (this.inhaled) {
      // 已吸过，不可重复吸
      return { ok: false, fuel: 0, phase: 'already-inhaled' };
    }
    this.inhaled = true;
    return { ok: true, fuel: this.amount, phase: 'inhaled' };
  }

  /**
   * 一次性呼：呼出已吸入的燃料，呼出即耗尽。
   * @returns {{ok:boolean, fuel:number, phase:string}}
   */
  exhale() {
    if (this.exhaled) {
      return { ok: false, fuel: 0, phase: 'exhausted' };
    }
    if (!this.inhaled) {
      // 未吸入，无法呼出
      return { ok: false, fuel: 0, phase: 'not-inhaled' };
    }
    const released = this.amount;
    this.amount = 0;
    this.exhaled = true;
    return { ok: true, fuel: released, phase: 'exhaled' };
  }

  /**
   * 燃料是否已耗尽（一次性）
   */
  isConsumed() {
    return this.exhaled;
  }
}

/**
 * 二元燃料层：会员是动态的，等待被统计进去的燃料集合。
 */
class TwoYuan {
  constructor() {
    // 动态会员燃料池：等待被统计进去
    this.members = [];
  }

  /**
   * 加入一个燃料会员（动态）
   * @param {Fuel} fuel
   */
  addFuel(fuel) {
    if (!(fuel instanceof Fuel)) {
      throw new Error('必须是 Fuel 实例');
    }
    this.members.push(fuel);
    return this;
  }

  /**
   * 二元燃料统计：把所有动态会员燃料一次性吸入并汇总。
   * 返回本次统计得到的燃料总量。
   * @returns {number}
   */
  consume() {
    let total = 0;
    for (const f of this.members) {
      const r = f.inhale();
      if (r.ok) total += r.fuel;
    }
    return total;
  }

  /**
   * 全部呼出（一次性耗尽），返回呼出总量。
   * @returns {number}
   */
  release() {
    let total = 0;
    for (const f of this.members) {
      const r = f.exhale();
      if (r.ok) total += r.fuel;
    }
    return total;
  }

  /**
   * 当前未耗尽的会员数量（动态等待统计的部分）
   */
  pendingCount() {
    return this.members.filter(f => !f.isConsumed()).length;
  }
}

// ============================================================
// 三元 · 锚点裁判（ThreeYuan + Anchor）
// 有一个锚点的，径向对折公约数的锚点；
// 天平比较：用二元和三元相互参考，动态/静态的裁判；
// 形成关系流动是比较的一个输出和读取；
// 价值审判：归属和拥有者的思想的损益关系罗列。
// ============================================================

/**
 * 锚点：径向对折 + 公约数。
 */
class Anchor {
  constructor(value, label = '') {
    this.value = value;
    this.label = label;
  }

  /**
   * 径向对折：将锚点值径向对折到中心。
   * @returns {{center:number, folded:boolean, origin:number}}
   */
  radialFold() {
    return {
      center: this.value / 2,
      folded: true,
      origin: this.value
    };
  }

  /**
   * 公约数：与另一个锚点求最大公约数。
   * @param {Anchor} other
   * @returns {number}
   */
  commonDivisor(other) {
    const a = Math.abs(Math.floor(this.value));
    const b = Math.abs(Math.floor(other.value));
    const gcd = (x, y) => (y === 0 ? x : gcd(y, x % y));
    return gcd(a, b);
  }
}

/**
 * 三元锚点层：管理多个锚点，提供径向对折、多锚点公约数。
 */
class ThreeYuan {
  constructor() {
    this.anchors = [];
  }

  /**
   * 添加锚点
   * @param {Anchor} anchor
   */
  addAnchor(anchor) {
    if (!(anchor instanceof Anchor)) {
      throw new Error('必须是 Anchor 实例');
    }
    this.anchors.push(anchor);
    return this;
  }

  /**
   * 全部锚点径向对折
   * @returns {Array}
   */
  radialFold() {
    return this.anchors.map(a => a.radialFold());
  }

  /**
   * 多锚点公约数：所有锚点值的最大公约数。
   * 静态参考量，用于天平裁判。
   * @returns {number|null}
   */
  commonDivisor() {
    if (this.anchors.length === 0) return null;
    if (this.anchors.length === 1) return Math.abs(Math.floor(this.anchors[0].value));
    let g = Math.abs(Math.floor(this.anchors[0].value));
    for (let i = 1; i < this.anchors.length; i++) {
      const tmp = new Anchor(g);
      g = tmp.commonDivisor(this.anchors[i]);
    }
    return g;
  }

  /**
   * 锚点数量
   */
  size() {
    return this.anchors.length;
  }
}

// ============================================================
// 锥相位枢纽链（ConePhaseHubChain）
// 枢纽链的可信度的多元性的根毛点：
//   根扎得深 → 交互动态最为枢纽，被观测到，交互交集点更深入；
//   根扎得不深 → 可信任的对话交互作为锚点很浅，善意抵达不了对方。
// 注意力锚点分发：锚点发送给谁、谁的注意力被抵制/认可。
// ============================================================

class ConePhaseHubChain {
  /**
   * @param {number} rootDepth 根毛深度（越深越枢纽、越被观测）
   */
  constructor(rootDepth = 0) {
    this.rootDepth = rootDepth;
    // 可信度：由根毛深度与多元性共同决定
    this.trustLevel = 0;
    // 多元性的根毛点：每一条根毛代表一个可被观测的交互通道
    this.rootHairs = [];
    // 注意力锚点分发列表：{target, action('approve'|'resist'), depth}
    this.attentionAnchors = [];
    // 交互交集点：根扎得深时更深入
    this.intersectionDepth = 0;
    this._recompute();
  }

  /**
   * 添加一条根毛（多元性的根毛点之一）
   * @param {{label?:string, weight?:number}} hair
   */
  addRootHair(hair = {}) {
    this.rootHairs.push({
      label: hair.label || `root-${this.rootHairs.length + 1}`,
      weight: hair.weight !== undefined ? hair.weight : 1
    });
    this._recompute();
    return this;
  }

  /**
   * 根据根毛深度 + 多元性，重算可信度与交互交集深度。
   * 根扎得深 → 可信度高、交集点深入；根扎得不深 → 锚点很浅，善意抵达不了对方。
   */
  _recompute() {
    const diversity = this.rootHairs.reduce((s, h) => s + h.weight, 0); // 多元性总量
    // 可信度 = 深度归一 × 多元性归一（两者缺一不可）
    const depthFactor = Math.min(1, this.rootDepth / 10);
    const diversityFactor = Math.min(1, diversity / 10);
    this.trustLevel = +(depthFactor * diversityFactor).toFixed(4);
    // 交互交集点深度：根越深越深入
    this.intersectionDepth = +(this.rootDepth * (1 + diversityFactor)).toFixed(4);
  }

  /**
   * 注意力锚点分发：把锚点发送给某个目标，标记为认可或抵制。
   * @param {string} target 锚点发送给谁
   * @param {'approve'|'resist'} action 认可 / 抵制
   */
  dispatchAnchor(target, action) {
    if (action !== 'approve' && action !== 'resist') {
      throw new Error("action 必须是 'approve' 或 'resist'");
    }
    const anchor = {
      target,
      action,
      depth: this.rootDepth,
      trust: this.trustLevel
    };
    this.attentionAnchors.push(anchor);
    return anchor;
  }

  /**
   * 某目标的注意力是否被认可
   */
  isApproved(target) {
    return this.attentionAnchors.some(a => a.target === target && a.action === 'approve');
  }

  /**
   * 某目标的注意力是否被抵制
   */
  isResisted(target) {
    return this.attentionAnchors.some(a => a.target === target && a.action === 'resist');
  }

  /**
   * 善意是否能抵达对方：
   * 根扎得不深 → 锚点很浅，善意抵达不了对方。
   * 这里以根毛深度阈值 3 作为「能抵达」的门槛。
   */
  canReach() {
    return this.rootDepth >= 3 && this.trustLevel > 0;
  }

  /**
   * 当前是否为枢纽节点：根扎得深 + 多元性足 + 被观测（有注意力分发）
   */
  isHub() {
    return this.rootDepth >= 3 && this.rootHairs.length >= 2 && this.attentionAnchors.length > 0;
  }
}

// ============================================================
// 四元 · 高维镜像（FourYuan + MirrorLogic）
// 镜像的，归属权和主导权者的从属关系逻辑；
// 已发生改变 / 未发生改变的双向发生改变；
// 高维镜像的多模态有无因果、相位判断；
// 内锥形统计 / 外锥形统计；
// 锥相位枢纽链；分布式集群的多相位注意力锚点；
// 人为的边界、冲突、许可、自发情绪。
// ============================================================

/**
 * 镜像逻辑：归属权 + 主导权 + 双向改变 + 多模态有无因果 + 相位。
 */
class MirrorLogic {
  constructor() {
    // 归属权：从属关系逻辑的归属方
    this.ownership = null;
    // 主导权：主导方
    this.dominance = null;
    // 已发生改变 / 未发生改变（双向同时成立）
    this.changed = false;
    this.unchanged = true;
    // 相位
    this.phase = 0;
  }

  /**
   * 设置归属权
   */
  setOwnership(owner) {
    this.ownership = owner;
    return this;
  }

  /**
   * 设置主导权
   */
  setDominance(master) {
    this.dominance = master;
    return this;
  }

  /**
   * 从属关系：归属权隶属于主导权
   * @returns {{owner:*, master:*, relation:string}}
   */
  subordination() {
    return {
      owner: this.ownership,
      master: this.dominance,
      relation: this.ownership && this.dominance ? 'subordinate' : 'unbound'
    };
  }

  /**
   * 已改变 / 未改变的双向发生改变
   * 事情已发生改变和未发生改变，双向同时发生改变。
   * @param {boolean} changeInput
   * @returns {{changed:boolean, unchanged:boolean, bidirectional:boolean}}
   */
  bidirectionalChange(changeInput) {
    this.changed = !!changeInput;
    // 双向发生改变：changed 与 unchanged 互为镜像，同时翻转
    this.unchanged = !this.changed;
    return {
      changed: this.changed,
      unchanged: this.unchanged,
      bidirectional: true
    };
  }

  /**
   * 多模态有无因果：相位判断
   * 高维镜像的多模态有无因果，包含相位关系。
   * @param {number} [phase]
   * @returns {{has:number, not:number, phase:number, multimodal:boolean}}
   */
  multiModalCausal(phase) {
    if (typeof phase === 'number') this.phase = phase;
    return {
      has: this.changed ? 1 : 0,
      not: this.unchanged ? 1 : 0,
      phase: this.phase,
      multimodal: true
    };
  }

  /**
   * 内锥形统计：向内收敛（平均/聚拢）
   * @param {number[]} items
   * @returns {number}
   */
  innerConeStats(items) {
    if (!items || items.length === 0) return 0;
    return items.reduce((s, x) => s + x, 0) / items.length;
  }

  /**
   * 外锥形统计：向外发散（累加/扩散）
   * @param {number[]} items
   * @returns {number}
   */
  outerConeStats(items) {
    if (!items || items.length === 0) return 0;
    return items.reduce((s, x) => s + x, 0);
  }

  /**
   * 关系的动态时间分配：按相位分配时间权重
   * @param {number} timeSlice 总时间片
   * @returns {{phase:number, weight:number, timeAllocated:number}}
   */
  dynamicTimeAllocation(timeSlice) {
    // 相位越大，分配到的动态时间越多
    const weight = Math.abs(this.phase) + 1;
    return {
      phase: this.phase,
      weight,
      timeAllocated: timeSlice * (weight / (weight + 1))
    };
  }

  /**
   * 锥相位枢纽链：挂接一条枢纽链
   * @param {ConePhaseHubChain} chain
   */
  conePhaseHub(chain) {
    if (!(chain instanceof ConePhaseHubChain)) {
      throw new Error('必须是 ConePhaseHubChain 实例');
    }
    this.hubChain = chain;
    return chain;
  }
}

/**
 * 四元高维镜像层：
 * 管理镜像逻辑、锥相位枢纽链、分布式集群的多相位注意力锚点，
 * 以及人为的边界、冲突、许可、自发情绪。
 */
class FourYuan {
  constructor() {
    this.mirrors = [];
    this.hubChains = [];
    // 分布式集群的多相位注意力锚点：{phase, target, action}
    this.attentionAnchors = [];
    // 人为的边界 / 冲突 / 许可 / 自发情绪
    this.boundaries = [];
    this.conflicts = [];
    this.permissions = [];
    this.spontaneousEmotions = [];
  }

  /**
   * 添加镜像逻辑
   */
  addMirror(mirror) {
    if (!(mirror instanceof MirrorLogic)) {
      throw new Error('必须是 MirrorLogic 实例');
    }
    this.mirrors.push(mirror);
    return this;
  }

  /**
   * 添加锥相位枢纽链
   */
  addHubChain(chain) {
    if (!(chain instanceof ConePhaseHubChain)) {
      throw new Error('必须是 ConePhaseHubChain 实例');
    }
    this.hubChains.push(chain);
    return this;
  }

  /**
   * 分布式集群的多相位注意力锚点
   * 锚点发送给谁、谁的注意力被抵制 / 认可。
   * @param {number} phase 相位
   * @param {string} target 锚点发送给谁
   * @param {'approve'|'resist'} action 认可 / 抵制
   */
  dispatchAttention(phase, target, action) {
    const anchor = { phase, target, action };
    this.attentionAnchors.push(anchor);
    return anchor;
  }

  /**
   * 在指定相位上被认可的目标
   */
  approvedAt(phase) {
    return this.attentionAnchors
      .filter(a => a.phase === phase && a.action === 'approve')
      .map(a => a.target);
  }

  /**
   * 在指定相位上被抵制的目标
   */
  resistedAt(phase) {
    return this.attentionAnchors
      .filter(a => a.phase === phase && a.action === 'resist')
      .map(a => a.target);
  }

  /**
   * 添加人为边界
   */
  addBoundary(label) {
    this.boundaries.push({ label });
    return this;
  }

  /**
   * 添加冲突
   */
  addConflict(label) {
    this.conflicts.push({ label });
    return this;
  }

  /**
   * 添加许可
   */
  addPermission(label) {
    this.permissions.push({ label, granted: true });
    return this;
  }

  /**
   * 添加自发情绪
   */
  addSpontaneousEmotion(label) {
    this.spontaneousEmotions.push({ label });
    return this;
  }
}

// ============================================================
// 大齿轮 · 意义解释器（BigGear）
// 一维时间层级关系的聚类关系代数；
// 把各种系统串联成时间与空间层级；逻辑就在这里面；
// 大齿轮才讲意义，被解释器解释解析出意义感；
// 意义感是多参考系：静态参考系、对动态作出参考的解释。
// ============================================================

class BigGear {
  constructor() {
    // 串联进来的各系统（0元/二元/三元/四元/猫码等）
    this.systems = [];
    // 时间层级（一维时间层级关系）
    this.timeHierarchy = [];
    // 空间层级
    this.spaceHierarchy = [];
    // 聚类关系代数记录
    this.clusters = [];
  }

  /**
   * 把一个系统串联进大齿轮
   * @param {{name:string, [key:string]:any}} system
   */
  attach(system) {
    if (!system || typeof system.name !== 'string') {
      throw new Error('系统必须包含 name 字段');
    }
    this.systems.push(system);
    return this;
  }

  /**
   * 添加时间层级
   * @param {string} layer
   */
  addTimeLayer(layer) {
    this.timeHierarchy.push(layer);
    return this;
  }

  /**
   * 添加空间层级
   * @param {string} layer
   */
  addSpaceLayer(layer) {
    this.spaceHierarchy.push(layer);
    return this;
  }

  /**
   * 时空层级：把各种系统串联成时间与空间层级
   * @returns {{time:string[], space:string[], systems:number}}
   */
  timeSpaceHierarchy() {
    return {
      time: this.timeHierarchy.slice(),
      space: this.spaceHierarchy.slice(),
      systems: this.systems.length
    };
  }

  /**
   * 聚类关系代数：一维时间层级关系的聚类
   * @param {Array} items 待聚类项
   * @param {Function} keyFn 提取聚类键的函数
   * @returns {Map<any, Array>}
   */
  clusterAlgebra(items, keyFn) {
    const map = new Map();
    for (const it of items) {
      const k = keyFn(it);
      if (!map.has(k)) map.set(k, []);
      map.get(k).push(it);
    }
    this.clusters.push({ size: map.size, keys: Array.from(map.keys()) });
    return map;
  }

  /**
   * 意义解释器：被解释器解析出意义感
   * 大齿轮才讲意义。
   * @param {*} signal 待解释的信号
   * @returns {{meaning:string|null, interpreted:boolean, signal:*}}
   */
  meaningInterpreter(signal) {
    if (signal === null || signal === undefined || signal === '') {
      return { meaning: null, interpreted: false, signal };
    }
    return {
      meaning: `meaning-of:${String(signal)}`,
      interpreted: true,
      signal
    };
  }

  /**
   * 多参考系：静态参考系 + 对动态作出参考的解释
   * 意义感是多参考系。
   * @param {*} staticRef 静态参考系
   * @param {*} dynamicRef 对动态作出参考的解释
   * @returns {{static:*, dynamic:*, combined:string}}
   */
  multiReference(staticRef, dynamicRef) {
    return {
      static: staticRef,
      dynamic: dynamicRef,
      combined: `${String(staticRef)} ∩ ${String(dynamicRef)}`
    };
  }
}

// ============================================================
// 天平裁判（BalanceJudge）
// 用二元和三元相互参考，动态/静态的裁判；
// 形成关系流动是比较的一个输出和读取；
// 价值审判：归属和拥有者的思想的损益关系罗列；
// 损益账：归属和拥有者的损益关系。
// ============================================================

class BalanceJudge {
  constructor() {
    // 比较记录：每一次比较的输出与读取
    this.records = [];
    // 价值审判记录
    this.judgments = [];
  }

  /**
   * 天平比较：用二元和三元相互参考，动态/静态的裁判。
   * 二元（燃料）= 动态；三元（锚点公约数）= 静态。
   * 形成关系流动是比较的一个输出和读取。
   * @param {TwoYuan} twoYuan
   * @param {ThreeYuan} threeYuan
   * @returns {{dynamic:number, staticVal:number, flow:number, read:number, output:number}}
   */
  compare(twoYuan, threeYuan) {
    // 动态：二元燃料被统计进去的量
    const dynamic = (twoYuan && typeof twoYuan.consume === 'function')
      ? twoYuan.consume()
      : 0;
    // 静态：三元锚点公约数
    const staticVal = (threeYuan && typeof threeYuan.commonDivisor === 'function')
      ? (threeYuan.commonDivisor() || 0)
      : 0;
    // 关系流动：比较的输出 = 动态 - 静态
    const flow = dynamic - staticVal;
    const record = {
      dynamic,
      static: staticVal,
      flow,
      read: flow,    // 读取
      output: flow,  // 输出
      timestamp: Date.now()
    };
    this.records.push(record);
    return record;
  }

  /**
   * 价值审判：归属和拥有者的思想的损益关系罗列。
   * @param {string} owner 归属和拥有者
   * @param {Array<{label:string, value:number}>} items 思想的损益项（正=益，负=损）
   * @returns {{owner:string, profit:number, loss:number, net:number, items:Array}}
   */
  valueJudgment(owner, items) {
    const safe = Array.isArray(items) ? items : [];
    const profitItems = safe.filter(i => i.value > 0);
    const lossItems = safe.filter(i => i.value < 0);
    const profit = profitItems.reduce((s, i) => s + i.value, 0);
    const loss = lossItems.reduce((s, i) => s + i.value, 0);
    const judgment = {
      owner,
      profit,
      loss,
      net: profit + loss,
      items: safe.slice()
    };
    this.judgments.push(judgment);
    return judgment;
  }

  /**
   * 损益账：归属和拥有者的损益关系罗列。
   * @param {Array} judgments 价值审判结果列表（缺省时使用已记录的审判）
   * @returns {Array}
   */
  profitLossAccount(judgments) {
    const list = Array.isArray(judgments) ? judgments : this.judgments;
    return list.map(j => ({
      owner: j.owner,
      profit: j.profit,
      loss: j.loss,
      net: j.net
    }));
  }

  /**
   * 读取最近一次比较的关系流动
   */
  lastFlow() {
    if (this.records.length === 0) return null;
    return this.records[this.records.length - 1];
  }

  /**
   * 红黑账（红=亏损，黑=暗面盈利）
   *   红 = 显式亏损（成本/失败），黑 = 隐式获利（缓存/复用/暗面收益）
   *   净 = 黑 - 红（>0 暗面盈利，<0 账面亏损）
   * @param {number} redLoss 红色亏损金额
   * @param {number} darkGain 黑色暗面盈利金额
   * @returns {{red:number, black:number, net:number, verdict:'dark_profit'|'loss'|'balance'}}
   */
  judgeRedBlack(redLoss, darkGain) {
    const red = Number(redLoss) || 0;
    const black = Number(darkGain) || 0;
    const net = black - red;
    const verdict = net > 0 ? 'dark_profit' : (net < 0 ? 'loss' : 'balance');
    const rec = { type: 'red_black', red, black, net, verdict, at: Date.now() };
    this.records.push(rec);
    return rec;
  }

  /**
   * 白黑账（白=明面盈利，黑=暗面盈利）
   *   白 = 公开可说明的盈利，黑 = 隐式/未公布的盈利
   *   净 = 白 + 黑（总盈利）
   * @param {number} whiteGain 白色明面盈利
   * @param {number} darkGain 黑色暗面盈利
   * @returns {{white:number, black:number, total:number, level:'weak'|'normal'|'strong'|'dominant'}}
   */
  judgeWhiteBlack(whiteGain, darkGain) {
    const white = Number(whiteGain) || 0;
    const black = Number(darkGain) || 0;
    const total = white + black;
    let level;
    if (total <= 0) level = 'weak';
    else if (total < 50) level = 'normal';
    else if (total < 100) level = 'strong';
    else level = 'dominant';
    const rec = { type: 'white_black', white, black, total, level, at: Date.now() };
    this.records.push(rec);
    return rec;
  }

  /**
   * 快捷双判：我 vs 他（或阈值）→ diff/winner + 红黑账 + 白黑账
   * @param {number} myScore
   * @param {number} theirScore
   * @returns {{my:number, their:number, diff:number, winner:'self'|'other'|'tie', redBlack:any, whiteBlack:any}}
   */
  judgeCompare(myScore, theirScore) {
    const my = Number(myScore) || 0;
    const their = Number(theirScore) || 0;
    const diff = my - their;
    const winner = diff > 0 ? 'self' : (diff < 0 ? 'other' : 'tie');
    // 红黑账：红=对方获利(我损)，黑=我获利的暗面部分(diff>0取diff)
    const lossByOther = Math.max(0, their - my);
    const darkGain   = Math.max(0, Math.round((my - their) * 0.5));
    // 白黑账：白=我公开得分的一半，黑=暗面同上
    const whiteGain  = Math.max(0, Math.round(my * 0.5));
    const redBlack   = this.judgeRedBlack(lossByOther, darkGain);
    const whiteBlack = this.judgeWhiteBlack(whiteGain, darkGain);
    const rec = { type: 'compare', my, their, diff, winner, redBlack, whiteBlack, at: Date.now() };
    this.records.push(rec);
    return rec;
  }
}

// ============================================================
// 导出
// ============================================================

module.exports = {
  // 0元 混元初始
  ZeroYuan,
  // 二元 燃料层
  Fuel,
  TwoYuan,
  // 三元 锚点裁判
  Anchor,
  ThreeYuan,
  // 锥相位枢纽链
  ConePhaseHubChain,
  // 四元 高维镜像
  MirrorLogic,
  FourYuan,
  // 大齿轮 意义解释器
  BigGear,
  // 天平裁判
  BalanceJudge
};
