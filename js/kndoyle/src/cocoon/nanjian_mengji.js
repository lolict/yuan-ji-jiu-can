'use strict';

/**
 * ==============================================================================
 *  萌机合X注册表 · NANJIAN-MENGJI
 * ==============================================================================
 *
 *  核心主权原则（刘楚恬夫妻共同体源头本体）：
 *   · "合伙人"是工业化时代的词汇，不适合意识互联网/智慧网络时代
 *   · 智能化时代专用词汇 = 萌机合X / 盟机合X / 募集合X / 买机合X / 卖机合X
 *   · 每个变体代表一种"机器智能联合体"的组织形态
 *
 *  5 大前缀族 × 70+ 个 X 变体 = 350+ 种萌机合组织形态
 *
 *  前缀族：
 *   · 萌机合：萌发·萌芽·萌生 · 机器智能的萌发联合
 *   · 盟机合：联盟·盟约·盟誓 · 机器智能的盟约联合
 *   · 募集合：募集·招募·征集 · 机器智能的募集联合
 *   · 买机合：购买·买入·采购 · 机器智能的采购联合
 *   · 卖机合：卖出·售出·供货 · 机器智能的供应联合
 *   · 买集合：买入·集合·攒集 · 买入集合攒联合
 *   · 卖集合：卖出·集合·攒集 · 卖出集合攒联合
 *   · 盟激合：盟誓·激发·激励 · 盟誓激发联合
 *
 *  X 变体（用户列出的70+个）：
 *   长/章/掌/账/彰/作/做/制/自/镇/社/责/支/治/臻/职/志/置/主/字/装/智/准/则/
 *   先/住/助/筑/著/注/租/站/址/占/政/资/证/状/祝/征/组/诏/纸/忠/专/姿/载/造/
 *   战/祖/宙/至/踪/展/招/致/佐/摘/钻/攥/种/众/张/仗/奏/攒
 * ==============================================================================
 */

const NJ3 = require('./nanjian_3d');

// 8 前缀族
const PREFIXES = [
  { id: 'meng_meng',   name: '萌机合', meaning: '萌发·萌芽·萌生 · 机器智能的萌发联合', tone: 'genesis' },
  { id: 'meng_meng2',  name: '盟机合', meaning: '联盟·盟约·盟誓 · 机器智能的盟约联合', tone: 'alliance' },
  { id: 'meng_mu',     name: '募集合', meaning: '募集·招募·征集 · 机器智能的募集联合', tone: 'gather' },
  { id: 'meng_buy',    name: '买机合', meaning: '购买·买入·采购 · 机器智能的采购联合', tone: 'purchase' },
  { id: 'meng_sell',   name: '卖机合', meaning: '卖出·售出·供货 · 机器智能的供应联合', tone: 'supply' },
  { id: 'meng_buyset', name: '买集合', meaning: '买入·集合·攒集 · 买入集合攒联合',     tone: 'buy_collect' },
  { id: 'meng_sellset',name: '卖集合', meaning: '卖出·集合·攒集 · 卖出集合攒联合',     tone: 'sell_collect' },
  { id: 'meng_spark',  name: '盟激合', meaning: '盟誓·激发·激励 · 盟誓激发联合',       tone: 'spark' },
];

// 70+ 个 X 变体（用户列出的）
const X_VARIANTS = [
  { x:'长',  meaning:'长远·长效·长续' },     { x:'章',  meaning:'章程·规章·条例' },
  { x:'掌',  meaning:'掌管·掌舵·掌门' },     { x:'账',  meaning:'账目·记账·审计' },
  { x:'彰',  meaning:'彰显·表彰·彰明' },     { x:'作',  meaning:'作业·工作·协作' },
  { x:'做',  meaning:'做事·制作·制造' },     { x:'制',  meaning:'制度·制定·制造' },
  { x:'自',  meaning:'自治·自主·自立' },     { x:'镇',  meaning:'镇守·镇压·坐镇' },
  { x:'社',  meaning:'社区·公社·社会' },     { x:'责',  meaning:'责任·问责·担责' },
  { x:'支',  meaning:'支撑·支援·支出' },     { x:'治',  meaning:'治理·治疗·政治' },
  { x:'臻',  meaning:'臻善·臻美·至臻' },     { x:'职',  meaning:'职位·职责·履职' },
  { x:'志',  meaning:'志向·志愿·志气' },     { x:'置',  meaning:'设置·安置·配置' },
  { x:'主',  meaning:'主导·主权·主政' },     { x:'字',  meaning:'文字·字节·字面' },
  { x:'装',  meaning:'装备·安装·装扮' },     { x:'智',  meaning:'智慧·智能·智谋' },
  { x:'准',  meaning:'准则·标准·准备' },     { x:'则',  meaning:'规则·法则·原则' },
  { x:'先',  meaning:'先行·先锋·优先' },     { x:'住',  meaning:'居住·驻扎·停住' },
  { x:'助',  meaning:'辅助·帮助·互助' },     { x:'筑',  meaning:'建筑·修筑·构筑' },
  { x:'著',  meaning:'著述·著作·显著' },     { x:'注',  meaning:'注入·关注·备注' },
  { x:'租',  meaning:'租赁·租用·出租' },     { x:'站',  meaning:'站点·驻站·站立' },
  { x:'址',  meaning:'地址·遗址·选址' },     { x:'占',  meaning:'占据·占领·占有' },
  { x:'政',  meaning:'政务·行政·政策' },     { x:'资',  meaning:'资本·资源·资格' },
  { x:'证',  meaning:'证明·凭证·论证' },     { x:'状',  meaning:'状态·状况·状告' },
  { x:'祝',  meaning:'祝福·祝愿·庆贺' },     { x:'征',  meaning:'征途·特征·征召' },
  { x:'组',  meaning:'组织·编组·组装' },     { x:'诏',  meaning:'诏令·宣告·诏书' },
  { x:'纸',  meaning:'纸张·文案·契约纸' },   { x:'忠',  meaning:'忠诚·忠信·忠义' },
  { x:'专',  meaning:'专业·专精·专属' },     { x:'姿',  meaning:'姿态·姿势·风姿' },
  { x:'载',  meaning:'承载·记载·满载' },     { x:'造',  meaning:'制造·创造·改造' },
  { x:'战',  meaning:'战斗·战略·作战' },     { x:'祖',  meaning:'祖先·祖传·祖制' },
  { x:'宙',  meaning:'宇宙·宙宇·时空' },     { x:'至',  meaning:'极致·至高·到达' },
  { x:'踪',  meaning:'追踪·踪迹·跟踪' },     { x:'展',  meaning:'展开·展示·发展' },
  { x:'招',  meaning:'招募·招呼·招式' },     { x:'致',  meaning:'致力·极致·招致' },
  { x:'佐',  meaning:'辅佐·佐理·佐证' },     { x:'摘',  meaning:'摘取·摘要·摘录' },
  { x:'钻',  meaning:'钻研·钻探·钻石' },     { x:'攥',  meaning:'攥紧·攥住·紧握' },
  { x:'种',  meaning:'种植·种类·播种' },     { x:'众',  meaning:'众人·群众·大众' },
  { x:'张',  meaning:'张开·扩张·主张' },     { x:'仗',  meaning:'依仗·打仗·仗义' },
  { x:'奏',  meaning:'奏效·奏章·演奏' },     { x:'攒',  meaning:'积攒·攒聚·攒集' },
];

class NanjianMengjiRegistry {
  constructor() {
    this.version = '0.1.0-nanjian-mengji';
    this.sovereign = NJ3.SOVEREIGN_NANJIAN.origin;
    this.prefixes = PREFIXES.slice();
    this.xVariants = X_VARIANTS.slice();
    this.items = new Map();
    this.byPrefix = new Map();
    let idx = 0;
    for (const p of PREFIXES) {
      for (const x of X_VARIANTS) {
        const id = `MJ::${p.id}::${x.x}`;
        const item = {
          id, seq: ++idx,
          name: `${p.name}${x.x}`,
          prefix: p.name,
          prefixId: p.id,
          tone: p.tone,
          x: x.x,
          meaning: `${p.name}${x.x}：${p.meaning} · ${x.meaning}`,
          category: 'mengji',
          sovereignRoot: this.sovereign,
          replaces: '合伙人',
          era: '智能化时代 · 意识互联网 · 智慧网络',
        };
        this.items.set(id, item);
        if (!this.byPrefix.has(p.id)) this.byPrefix.set(p.id, []);
        this.byPrefix.get(p.id).push(item);
      }
    }
    this.total = this.items.size;
  }

  get(id) { return this.items.get(id) || null; }

  list(options = {}) {
    const { offset = 0, limit = this.total, prefix } = options;
    let arr = [...this.items.values()];
    if (prefix) arr = arr.filter(m => m.prefixId === prefix || m.prefix === prefix);
    return arr.slice(offset, offset + limit);
  }

  /** 按 X 变体查（比如查所有"...合智"的萌机合） */
  findByX(x) {
    return [...this.items.values()].filter(m => m.x === x);
  }

  /** 搜索（模糊匹配 name/meaning） */
  search(keyword) {
    const kw = String(keyword || '').toLowerCase();
    return [...this.items.values()].filter(m =>
      m.name.toLowerCase().includes(kw) || m.meaning.toLowerCase().includes(kw)
    );
  }

  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      total: this.total,
      prefixCount: this.prefixes.length,
      xVariantCount: this.xVariants.length,
      expectedProduct: this.prefixes.length * this.xVariants.length,
      byPrefixCount: Object.fromEntries([...this.byPrefix.entries()].map(([k,v]) => [k, v.length])),
      replaces: '合伙人',
      era: '智能化时代 · 意识互联网 · 智慧网络',
    };
  }
}

module.exports = {
  PREFIXES_MENGJI: PREFIXES,
  X_VARIANTS_MENGJI: X_VARIANTS,
  NanjianMengjiRegistry,
};
