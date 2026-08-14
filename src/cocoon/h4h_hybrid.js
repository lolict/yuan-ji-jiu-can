/**
 * H4H · 混元杂交槽注册表（Hybrid-4-Hybrid Registry）
 * ============================================================
 *
 * 设计哲学：所有能力 = 男人 × 女人 = 生孩子（做一道乘法题的交集遍历）
 *
 *   - 26 个 x4x 杂交槽 = 26 个交配位点，对应 26 个寄生壳
 *   - 每个槽可登记父本×母本 → produce(parentA, parentB) -> child
 *   - 杂交 = 乘法 × 关系代数的交叉乘积；0 元 = 介于有无之间的混元状态
 *
 * 用法：
 *   const H4H = require('./h4h_hybrid');
 *   const h = H4H.create();
 *   h.registerSeed('h4h', { id:'空间', name:'3D空间', category:'math' });
 *   const child = h.marry({name:'空间'}, {name:'进制'}, 'h4h');
 */

'use strict';

const { SHELL_REGISTRY } = require('./shell_registry');

// ============================================================
// 26 x4x 杂交槽定义（a-z 26 letters = 26 slots）
// ============================================================
const SLOT_DEFS = [
  { id: 'd4d', letter: 'd', name: '道/动力递推杂交槽',       category: 'recursion',   icon: '🔄', meaning: '递归递推·道家·动力·递归关系代数', shellHint: 'pc' },
  { id: 'a4a', letter: 'a', name: '伙码结伙 Agent/API 杂交槽', category: 'api',         icon: '🤝', meaning: '伙码结伙×多智能体Agent×对外API顶层入口', shellHint: 'all' },
  { id: 'g4g', letter: 'g', name: '群论/激光杂交槽',           category: 'math',        icon: '🔦', meaning: '群论/激光/哥德尔/光达LiDAR/概率', shellHint: 'laser_scan' },
  { id: 'r4r', letter: 'r', name: '奖励/检索/仁义杂交槽',     category: 'ai',          icon: '🎁', meaning: 'RL奖励/RAG检索/仁义/荣格原型',   shellHint: 'pc' },
  { id: 't4t', letter: 't', name: '拓扑/投影杂交槽',           category: 'math',        icon: '🗺', meaning: '拓扑/图论/3D投影/痛觉',            shellHint: '3dprojection' },
  { id: 'y4y', letter: 'y', name: '易经/因果/元宇宙杂交槽',   category: 'philosophy',  icon: '🌌', meaning: '易经64卦/因果/博弈/优化/元宇宙',  shellHint: 'metaverse' },
  { id: 'u4u', letter: 'u', name: '无人机蜂群杂交槽',         category: 'robotics',    icon: '🚁', meaning: '无人机蜂群/无人/UAV',              shellHint: 'uav' },
  { id: 'i4i', letter: 'i', name: '意念输入/BCI 杂交槽',      category: 'bci',         icon: '🧠', meaning: '意念输入/Ideas/脑机接口BCI',      shellHint: 'bci' },
  { id: 'o4o', letter: 'o', name: '输出/偶杂交槽',             category: 'io',          icon: '📤', meaning: '输出/对偶/偶配对',                shellHint: 'all' },
  { id: 'l4l', letter: 'l', name: '李群/链/MQF 消息链杂交槽',  category: 'mqf',         icon: '🔗', meaning: '李群/黎曼/链/区块链/MQF消息链',   shellHint: 'pc' },
  { id: 'm4m', letter: 'm', name: '核磁/货币/美感杂交槽',     category: 'society',     icon: '💰', meaning: 'MRI/货码CBDC货币/美感',           shellHint: 'mri/bank' },
  { id: 'n4n', letter: 'n', name: '神经/NeRF 杂交槽',          category: 'ai',          icon: '🧬', meaning: '神经网络/NeRF神经辐射场/数论',    shellHint: 'pc' },
  { id: 'h4h', letter: 'h', name: '★混元杂交本体枢纽槽',      category: 'core',        icon: '⚛', meaning: '混元杂交枢纽=火×混×货×画×幻',     shellHint: 'all' },
  { id: 'j4j', letter: 'j', name: '几何/积分/激光杂交槽',     category: 'math',        icon: '📐', meaning: '几何/积分/激光',                  shellHint: '3dprinter/laser_scan' },
  { id: 'v4v', letter: 'v', name: 'ARVR/视频/向量杂交槽',     category: 'xr',          icon: '🥽', meaning: 'AR/VR/视频生成/向量空间',         shellHint: 'ar_vr' },
  { id: 'f4f', letter: 'f', name: '分形/傅立叶/联邦杂交槽',   category: 'math',        icon: '🌊', meaning: '分形/傅立叶展开/联邦学习',        shellHint: 'pc' },
  { id: 'c4c', letter: 'c', name: '范畴/全景/容器杂交槽',     category: 'math',        icon: '🗃', meaning: '范畴论/360全景/容器/Web容器',     shellHint: '360cam' },
  { id: 'x4x', letter: 'x', name: 'X光/线性/信息杂交槽',       category: 'medical',     icon: '🩻', meaning: 'X光/线性代数/信息论/香农',        shellHint: 'xray' },
  { id: 'z4z', letter: 'z', name: '组合/张量/辨证杂交槽',     category: 'math',        icon: '🧮', meaning: '组合数学/张量/中医辨证',          shellHint: 'all' },
  { id: 's4s', letter: 's', name: '时空/熵/数/卫星杂交槽',     category: 'spacetime',   icon: '⏳', meaning: '大齿轮时空层级/熵/数论/卫星星链',  shellHint: 'satellite' },
  { id: 'e4e', letter: 'e', name: '易石/边缘计算杂交槽',       category: 'iot',         icon: '🪨', meaning: '易石互联/边缘计算/6G太赫兹通信',  shellHint: 'iot' },
  { id: 'w4w', letter: 'w', name: '世界模型/穿戴/微分杂交槽',  category: 'ai',          icon: '🌍', meaning: 'AGI世界模型/可穿戴计算/微分几何', shellHint: 'ar_vr' },
  { id: 'k4k', letter: 'k', name: '空间/康德杂交槽',           category: 'philosophy',  icon: '🧭', meaning: '3D空间感知/康德先验十二范畴',      shellHint: 'all' },
  { id: 'p4p', letter: 'p', name: '3D打印/排列/平衡杂交槽',   category: 'manufacturing', icon: '🖨', meaning: '3D打印/排列群/天平前庭觉平衡',    shellHint: '3dprinter' },
  { id: 'b4b', letter: 'b', name: '绑定/贝叶斯/双足杂交槽',   category: 'robotics',    icon: '🔗', meaning: '绑定/贝叶斯推断/人形双足步态',    shellHint: 'humanoid_robot' },
  { id: 'q4q', letter: 'q', name: '量子/群/潜意识杂交槽',     category: 'quantum',     icon: '⚛️', meaning: '量子计算/群论/佛洛依德潜意识',    shellHint: 'pc/supercomputer' },
];

// ============================================================
// 通用杂交算子：父本 × 母本 → 孩子
// ============================================================
const DEFAULT_HYBRID_OPERATORS = {
  default: (a, b) => {
    const nameA = (a && (a.name || a.cat || a.id)) ? (a.name || a.cat || a.id) : String(a);
    const nameB = (b && (b.name || b.cat || b.id)) ? (b.name || b.cat || b.id) : String(b);
    return {
      type: 'hybrid',
      parents: [a, b],
      name: `${nameA}×${nameB}`,
      meaning: `混元杂交：${nameA} × ${nameB} = 两者交集遍历（乘法）的新概念产物`,
      icon: (a && a.icon) || (b && b.icon) || '🧬',
      born: new Date().toISOString(),
    };
  },
  math_math:   (a, b) => Object.assign({}, DEFAULT_HYBRID_OPERATORS.default(a, b), { kind: 'higher_math', category: 'math×math=高级数学' }),
  ai_ai:       (a, b) => Object.assign({}, DEFAULT_HYBRID_OPERATORS.default(a, b), { kind: 'stronger_ai', category: 'ai×ai=AGI' }),
  cat_cat:     (a, b) => Object.assign({}, DEFAULT_HYBRID_OPERATORS.default(a, b), {
    kind: '损益杂交猫', category: 'catcode×catcode=损益猫',
    color: mixColors(a && a.color, b && b.color),
    attribute: `${(a && a.attribute) || ''} × ${(b && b.attribute) || ''}`,
    meaning:   `红黑白灰账杂交：父=${(a && a.meaning) || ''} 母=${(b && b.meaning) || ''}`,
  }),
  shell_shell: (a, b) => Object.assign({}, DEFAULT_HYBRID_OPERATORS.default(a, b), {
    kind: '跨平台联合壳', category: 'shell×shell=跨平台联合壳',
    permissions: uniq([].concat(a && a.permissions || []).concat(b && b.permissions || [])),
  }),
  spacetime_spacetime: (a, b) => Object.assign({}, DEFAULT_HYBRID_OPERATORS.default(a, b), {
    kind: '大齿轮时空层级', category: 'space×time=时空结构',
  }),
};

function mixColors(c1, c2) {
  if (!c1 || !c2) return '#' + (c1 || c2 || '888888').replace(/^#/, '');
  const pa = parseHex(c1);
  const pb = parseHex(c2);
  return '#'
       + toHex(Math.round((pa[0]+pb[0])/2))
       + toHex(Math.round((pa[1]+pb[1])/2))
       + toHex(Math.round((pa[2]+pb[2])/2));
}
function parseHex(c) {
  c = (c || '#888888').replace('#','');
  if (c.length === 3) c = c.split('').map(x=>x+x).join('');
  return [parseInt(c.slice(0,2),16), parseInt(c.slice(2,4),16), parseInt(c.slice(4,6),16)];
}
function toHex(n) { return n < 16 ? '0'+n.toString(16) : n.toString(16); }
function uniq(arr) { return Array.from(new Set(arr)); }

// ============================================================
// H4HRegistry
// ============================================================
class H4HRegistry {
  constructor(options = {}) {
    this.version = '0.1.0-h4h';
    this.slots = {};
    this.marriages = [];
    this.operators = Object.assign({}, DEFAULT_HYBRID_OPERATORS, options.extraOperators || {});
    this.customOps = options.customOps || {};

    for (const s of SLOT_DEFS) {
      this.slots[s.id] = Object.assign({}, s, {
        registered: [], // 种子库：每个槽内登记的父/母本
        marriages: [],
      });
    }
  }

  size() { return Object.keys(this.slots).length; }
  getSlot(id) { return this.slots[id] || null; }
  listSlots() { return Object.values(this.slots); }
  listSlotIds() { return Object.keys(this.slots); }

  registerSeed(slotId, seed) {
    if (!this.slots[slotId]) throw new Error(`H4H: 不存在的杂交槽 ${slotId}`);
    const payload = (seed && seed.payload) ? seed.payload : seed;
    const entry = {
      id:       (seed && seed.id)       || ('seed_'+Math.random().toString(36).slice(2,8)),
      name:     (seed && seed.name)     || (payload && payload.name) || String(seed),
      category: (seed && seed.category) || (payload && payload.category) || (payload && (payload.permissions ? 'shell' : (payload.color ? 'catcode' : typeof payload))),
      payload,
      at: Date.now(),
    };
    this.slots[slotId].registered.push(entry);
    return entry;
  }

  /**
   * 杂交（结婚生孩子）
   *   marry(parentA, parentB, 'h4h')         // 自动选择算子
   *   marry(parentA, parentB, 't4t', 'math_math') // 指配算子
   */
  marry(a, b, slotId, opKey) {
    if (!this.slots[slotId]) throw new Error(`H4H: 不存在的杂交槽 ${slotId}`);
    const slot = this.slots[slotId];
    const op = opKey
      ? (this.operators[opKey] || this.customOps[opKey] || this.operators.default)
      : this._autoSelectOp(a, b);
    const child = op(a, b);
    child.slot = slotId;
    child.slotName = slot.name;
    child.slotIcon = slot.icon;
    child.slotMeaning = slot.meaning;
    // 合并父本母本的 category/meaning/attribute，便于打分系统看到真实科技关键词
    const catA = a && typeof a.category === 'string' ? a.category : '';
    const catB = b && typeof b.category === 'string' ? b.category : '';
    if (!child.category && (catA || catB)) {
      child.category = [catA, catB].filter(Boolean).join('×');
    }
    const meaningA = a && typeof a.meaning === 'string' ? a.meaning : '';
    const meaningB = b && typeof b.meaning === 'string' ? b.meaning : '';
    if (!child.attribute && (meaningA || meaningB)) {
      child.attribute = [meaningA, meaningB].filter(Boolean).join(' | ');
    }
    // 保存父本母本供 top 排行使用
    child.parentA = a && typeof a === 'object'
      ? { id: a.id, name: a.name || a.cat || a.id, category: a.category, meaning: a.meaning }
      : (typeof a === 'string' ? { name: a } : null);
    child.parentB = b && typeof b === 'object'
      ? { id: b.id, name: b.name || b.cat || b.id, category: b.category, meaning: b.meaning }
      : (typeof b === 'string' ? { name: b } : null);
    const record = { slotId, a, b, child, opName: (op && op.name) || 'hybrid-op', at: Date.now(), id: `m_${(typeof this._mid === 'number' ? ++this._mid : (this._mid=1))}` };
    // 额外暴露 id 便于外部引用
    child.marriageId = record.id;
    this.marriages.push(record);
    slot.marriages.push(record);
    return child;
  }

  /**
   * 指定槽内做前 N×N 全配对乘法
   */
  multiplySlot(slotId, n = 6) {
    if (!this.slots[slotId]) throw new Error(`H4H: 不存在的杂交槽 ${slotId}`);
    const slot = this.slots[slotId];
    const pool = slot.registered.length ? slot.registered : this._defaultSeeds(slot);
    const take = pool.slice(0, Math.max(2, Math.min(pool.length, n)));
    const out = [];
    for (let i = 0; i < take.length; i++)
      for (let j = i + 1; j < take.length; j++)
        out.push(this.marry(take[i].payload, take[j].payload, slotId));
    return out;
  }

  /**
   * 全量遍历 26 槽 × N×N 乘法（默认每个槽做 6×6=15 种组合，26 槽 ≈ 390 孩子）
   */
  multiplyAll(nPerSlot = 6) {
    const out = {};
    for (const id of Object.keys(this.slots)) out[id] = this.multiplySlot(id, nPerSlot);
    return out;
  }

  getMarriagesBySlot(slotId) {
    return slotId ? (this.slots[slotId] && this.slots[slotId].marriages || []) : this.marriages.slice();
  }

  getSlot(slotId) { return this.slots[slotId] || null; }
  listSlotIds() { return Object.keys(this.slots); }

  stats() {
    const perSlot = {};
    for (const [k, v] of Object.entries(this.slots)) perSlot[k] = v.marriages.length;
    return {
      version: this.version,
      total: this.marriages.length,
      slotCount: this.size(),
      perSlot,
      operators: Object.keys(this.operators).length,
    };
  }

  // ---- 内部 ----
  _autoSelectOp(a, b) {
    const ac = (a && a.category) || (a && a.permissions ? 'shell' : (a && a.color ? 'catcode' : ''));
    const bc = (b && b.category) || (b && b.permissions ? 'shell' : (b && b.color ? 'catcode' : ''));
    const key = `${ac}_${bc}`;
    if (this.operators[key])                 return this.operators[key];
    if (ac === 'catcode' && bc === 'catcode') return this.operators.cat_cat;
    if (ac === 'shell'   && bc === 'shell')   return this.operators.shell_shell;
    if ((ac === 'math' || ac === 'spacetime') && (bc === 'math' || bc === 'spacetime'))
      return this.operators[ac + '_' + bc] || this.operators.math_math;
    if (ac === 'ai'      && bc === 'ai')      return this.operators.ai_ai;
    return this.operators.default;
  }

  _defaultSeeds(slot) {
    const out = [];
    const shells = Object.values(SHELL_REGISTRY);
    shells.slice(0, 4).forEach(s => out.push({ id: s.id, name: s.name, category: 'shell', payload: s }));
    const extras = [
      { id: 'math',    name: '数学',    category: 'math' },
      { id: 'ai',      name: '人工智能',category: 'ai' },
      { id: 'space',   name: '空间',    category: 'spacetime' },
      { id: 'time',    name: '时间',    category: 'spacetime' },
      { id: 'logic',   name: '逻辑',    category: 'philosophy' },
      { id: 'matter',  name: '物质',    category: 'quantum' },
    ];
    extras.forEach(e => out.push(Object.assign({}, e, { payload: e })));
    return out;
  }
}

function create(options) { return new H4HRegistry(options); }

module.exports = {
  SLOT_DEFS,
  DEFAULT_HYBRID_OPERATORS,
  H4HRegistry,
  create,
};
