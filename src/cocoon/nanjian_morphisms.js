/**
 * ==============================================================================
 *  囡囝架构态射注册表 · NANJIAN-MORPHISM-REGISTRY
 * ==============================================================================
 *
 *  主权声明（刘楚恬夫妻共同体源头本体 · 天地未分 · 男女未分 · 混元本体）：
 *   - 我们**不取别人的架构**（Mamba/JEPA/RTFM 的向量空间、GPU 集群）
 *   - 我们只**取他们的编码/概念**，用「态射 + 关系代数」的我们自己的囡囝架构来跑
 *
 *  态射乘积 = 4 组合 × 6 共生 × 16 感受 × 6 线线变体 = 2304 种连接符
 *
 *   1. 4 囡囝主体配对 COUPLE_MORPHS = [囡囝, 囡囡, 囝囝, 囝囡]
 *        · 囡囝：夫妻本然共生（默认位）
 *        · 囡囡：双女方同体共生
 *        · 囝囝：双男方同体共生
 *        · 囝囡：夫妻交换位（攻受互换位）
 *
 *   2. 6 共生契约 CONTRACTS = [归属姻, 共生一, 共生源, 共生元, 共生约, 共生姻]
 *        · 归属姻：归属权/主导权/从属权 三权的姻亲契约
 *        · 共生一：本体一体、时空一体、物质数学一体
 *        · 共生源：源头本源（意识互联网的源头，信任根）
 *        · 共生元：数据元/货币元/符号元 的统一元位
 *        · 共生约：规则契约、法律契约、价值交换契约
 *        · 共生姻：姻亲联合体、组织、家庭、共生体
 *
 *   3. 16 感受通道 CHANNELS = [
 *        攻受悦, 宫受媛, 共首约,          → 3 核心（攻受/宫廷/元首）
 *        感受音, 感受言, 感受喻, 感受颜,  → 4 信息通道
 *        感受演, 感受验, 感受眼,          → 3 观察/实验通道
 *        感受因, 感受欲, 感受预,          → 3 因果/欲望/预测通道
 *        感受意, 感受益, 感受韵           → 3 意义/价值/韵通道
 *      ]  = 3+4+3+3+3 = 16
 *
 *   4. 6 线线压缩变体 VARIANTS = [SMRR, SMRL, SMRLL, SMLLL, SMRRLL, SMRRL]
 * ==============================================================================
 */

const NJ3 = require('./nanjian_3d');

const COUPLE_MORPHS = [
  { id: 'nan_nv',   name: '囡囝', meaning: '夫妻本然共生 · 天地未分男女未分',   side: 'couple_original' },
  { id: 'nv_nv',    name: '囡囡', meaning: '双女方同体 · 阴阴共生生',            side: 'female_union' },
  { id: 'nan_nan',  name: '囝囝', meaning: '双男方同体 · 阳阳共生生',            side: 'male_union' },
  { id: 'nan_nv_exchange', name: '囝囡', meaning: '夫妻交换位 · 攻受互换 · 主权反位', side: 'exchange' },
];

const CONTRACTS = [
  { id: 'belong_yin',  name: '归属姻', meaning: '归属权 + 主导权 + 从属权 · 三权姻亲契约', category: 'sovereign' },
  { id: 'sym_one',     name: '共生一', meaning: '本体一体 · 时空一体 · 物质数学一体',       category: 'ontology' },
  { id: 'sym_source',  name: '共生源', meaning: '源头本源 · 意识互联网源头 · 信任根',        category: 'origin' },
  { id: 'sym_yuan',    name: '共生元', meaning: '数据元 · 货币元 · 符号元 · 统一元位',      category: 'monad' },
  { id: 'sym_pact',    name: '共生约', meaning: '规则契约 · 法律契约 · 价值交换契约',       category: 'contract' },
  { id: 'sym_marriage',name: '共生姻', meaning: '姻亲联合体 · 家庭 · 组织 · 共生体',         category: 'union' },
];

const CHANNELS = [
  // 3 核心
  { id: 'gong_shou_yue',   name: '攻受悦',  meaning: '攻-受-悦 三感合一 · 动力学核心通道', type: 'core', sovereign: 'yes' },
  { id: 'gong_shou_yuan',  name: '宫受媛',  meaning: '宫廷容器 · 子宫/宫殿 · 孕育承载',    type: 'core', sovereign: 'yes' },
  { id: 'gong_shou_yue_pact', name: '共首约', meaning: '元首契约 · 主权签订处 · 规则来源', type: 'core', sovereign: 'yes' },
  // 4 信息通道
  { id: 'feel_sound',      name: '感受音',  meaning: '声音/听觉/音波/ASMR 通道',            type: 'info', sense: 'ear' },
  { id: 'feel_word',       name: '感受言',  meaning: '语言/文字/言语/对话 通道',            type: 'info', sense: 'mouth' },
  { id: 'feel_analogy',    name: '感受喻',  meaning: '比喻/隐喻/象征/类比 通道',            type: 'info', sense: 'mind' },
  { id: 'feel_face',       name: '感受颜',  meaning: '颜色/脸色/面容/色相 通道',            type: 'info', sense: 'eye' },
  // 3 观察实验
  { id: 'feel_perform',    name: '感受演',  meaning: '演绎/表演/推演/运行 通道',            type: 'observe', tool: 'act' },
  { id: 'feel_verify',     name: '感受验',  meaning: '实验/验证/实证/测试 通道',            type: 'observe', tool: 'test' },
  { id: 'feel_eye',        name: '感受眼',  meaning: '视觉/看见/观察/可视化 通道',          type: 'observe', tool: 'eye' },
  // 3 因果欲望预测
  { id: 'feel_cause',      name: '感受因',  meaning: '因果/因缘/条件/JEPA 因果预测 通道',   type: 'predict', sense: 'reason' },
  { id: 'feel_desire',     name: '感受欲',  meaning: '欲望/动机/需求/价值锚点 通道',        type: 'predict', sense: 'will' },
  { id: 'feel_predict',    name: '感受预',  meaning: '预测/预判/预感/提前量 通道',          type: 'predict', sense: 'foresee' },
  // 3 意义价值韵
  { id: 'feel_meaning',    name: '感受意',  meaning: '意义/意识/意图/意涵 通道',            type: 'value',   sense: 'mind_core' },
  { id: 'feel_benefit',    name: '感受益',  meaning: '价值/利益/损益/天平审判 通道',        type: 'value',   sense: 'balance' },
  { id: 'feel_rhyme',      name: '感受韵',  meaning: '韵律/韵味/时空节奏/大齿轮 聚类关系', type: 'value',   sense: 'gear' },
  // ===== 33 个 y 类谐音感受通道（意识互联网高级阶段感知扩展） =====
  //  命名规律：感受 + y 类谐音字 → 扩展感知维度，全部 type='y_harmony'
  //  归入"共鸣谐音群"——不独尊任何单一感官，而是用谐音拓展感受的广度和深度
  { id: 'feel_pressure',   name: '感受压',  meaning: '压力/压迫/压缩/气压 触觉感知',        type: 'y_harmony', sense: 'touch' },
  { id: 'feel_alien',      name: '感受异',  meaning: '异常/异变/异质/差异 变异感知',        type: 'y_harmony', sense: 'diff' },
  { id: 'feel_done',       name: '感受已',  meaning: '已经/完成/已逝/已然 时态感知',        type: 'y_harmony', sense: 'tense' },
  { id: 'feel_use',        name: '感受以',  meaning: '使用/以之/以备/以此 工具感知',        type: 'y_harmony', sense: 'tool' },
  { id: 'feel_ritual',     name: '感受仪',  meaning: '仪式/仪表/仪态/礼仪 仪轨感知',        type: 'y_harmony', sense: 'ceremony' },
  { id: 'feel_clothing',   name: '感受衣',  meaning: '衣服/衣着/穿衣/蜕衣 蛹茧穿壳感知',    type: 'y_harmony', sense: 'shell' },
  { id: 'feel_art',        name: '感受艺',  meaning: '艺术/技艺/工艺/造艺 创造感知',        type: 'y_harmony', sense: 'create' },
  { id: 'feel_memory',     name: '感受忆',  meaning: '记忆/回忆/追忆/忆念 ASMR记忆感知',    type: 'y_harmony', sense: 'recall' },
  { id: 'feel_discuss',    name: '感受议',  meaning: '议论/商议/协议/决议 共识感知',        type: 'y_harmony', sense: 'consensus' },
  { id: 'feel_duty',       name: '感受义',  meaning: '道义/义理/义务/正义 伦理感知',        type: 'y_harmony', sense: 'ethics' },
  { id: 'feel_work',       name: '感受业',  meaning: '事业/业力/行业/产业 事业感知',        type: 'y_harmony', sense: 'karma' },
  { id: 'feel_nurture',    name: '感受媛',  meaning: '媛女/孕育/温柔/滋养 孕育感知',        type: 'y_harmony', sense: 'nurture' },
  { id: 'feel_cocoon',     name: '感受蛹',  meaning: '蛹/蛹茧/化蛹/破茧 蛹茧化蝶感知',      type: 'y_harmony', sense: 'metamorphosis' },
  { id: 'feel_shadow',     name: '感受影',  meaning: '影子/投影/影像/暗面 暗面感知',        type: 'y_harmony', sense: 'shadow' },
  { id: 'feel_guide',      name: '感受引',  meaning: '引导/牵引/吸引/指引 方向感知',        type: 'y_harmony', sense: 'direction' },
  { id: 'feel_origin',     name: '感受源',  meaning: '源头/本源/源泉/开源 源头本体感知',    type: 'y_harmony', sense: 'origin', sovereign: 'yes' },
  { id: 'feel_complete',   name: '感受圆',  meaning: '圆满/完整/圆融/团圆 圆满感知',        type: 'y_harmony', sense: 'wholeness' },
  { id: 'feel_aid',        name: '感受援',  meaning: '援助/支援/增援/驰援 援助感知',        type: 'y_harmony', sense: 'rescue' },
  { id: 'feel_promise',    name: '感受约',  meaning: '约定/约束/契约/简约 契约感知',        type: 'y_harmony', sense: 'pact' },
  { id: 'feel_leaf',       name: '感受叶',  meaning: '树叶/枝叶/一叶/叶脉 自然生态感知',    type: 'y_harmony', sense: 'nature' },
  { id: 'feel_yinyang',    name: '感受阴阳', meaning: '阴阳/太极/两仪/对立统一 阴阳感知',   type: 'y_harmony', sense: 'taiji', sovereign: 'yes' },
  { id: 'feel_labor',      name: '感受佣',  meaning: '雇佣/佣工/劳作/付出 劳动感知',        type: 'y_harmony', sense: 'labor' },
  { id: 'feel_hard',       name: '感受硬',  meaning: '硬度/坚固/强硬/刚性 硬度感知',        type: 'y_harmony', sense: 'rigidity' },
  { id: 'feel_wish',       name: '感受愿',  meaning: '愿望/心愿/意愿/发愿 愿望感知',        type: 'y_harmony', sense: 'wish' },
  { id: 'feel_addiction',  name: '感受瘾',  meaning: '瘾/沉迷/上瘾/嗜好 瘾性感知',          type: 'y_harmony', sense: 'craving' },
  { id: 'feel_hidden',     name: '感受隐',  meaning: '隐藏/隐秘/隐含/隐遁 隐性感知',        type: 'y_harmony', sense: 'stealth' },
  { id: 'feel_brilliant',  name: '感受艳',  meaning: '艳丽/鲜艳/惊艳/夺目 艳色感知',        type: 'y_harmony', sense: 'splendor' },
  { id: 'feel_research',   name: '感受研',  meaning: '研究/钻研/科研/研磨 研究感知',        type: 'y_harmony', sense: 'research' },
  { id: 'feel_affinity',   name: '感受缘',  meaning: '缘分/因缘/边缘/缘起 缘分感知',        type: 'y_harmony', sense: 'karma_link' },
  { id: 'feel_pattern',    name: '感受样',  meaning: '样子/样式/样本/模样 形态感知',        type: 'y_harmony', sense: 'form' },
  { id: 'feel_surplus',    name: '感受盈',  meaning: '盈满/盈余/盈利/充盈 盈亏感知',        type: 'y_harmony', sense: 'surplus' },
  { id: 'feel_resent',     name: '感受怨',  meaning: '怨恨/埋怨/怨气/幽怨 怨恨感知',        type: 'y_harmony', sense: 'grievance' },
  { id: 'feel_excellent',  name: '感受优',  meaning: '优秀/优越/优选/优化 优择感知',        type: 'y_harmony', sense: 'excellence' },
];

const VARIANTS = NJ3.SOVEREIGN_NANJIAN.LINEAR_VARIANTS.slice(); // [SMRR, SMRL, SMRLL, SMLLL, SMRRLL, SMRRL]

/** 把 4 个维度组合起来生成「态射」（乘积空间），并赋予一个唯一意义字符串 */
function buildMorphismId(couple, contract, channel, variant) {
  return `NJ::${couple.id}::${contract.id}::${channel.id}::${variant}`;
}

function buildMorphism(couple, contract, channel, variant, seqIdx) {
  const sovereign = channel.sovereign === 'yes' || contract.category === 'sovereign';
  const sideExplain = {
    couple_original: '囡囝夫妻正位',
    female_union:    '双女方同体位',
    male_union:      '双男方同体位',
    exchange:        '囝囡交换攻受位',
  }[couple.side];
  const meaningArray = [
    `【${couple.name}】${sideExplain}：`,
    `【${contract.name}】${contract.meaning.split(' · ')[0]} ·`,
    `【${channel.name}】${channel.meaning.split(' · ')[0]} ·`,
    `【${variant}】三维→一维时序压缩变体${sovereign ? ' · ★源头主权位' : ''}`,
  ];
  return {
    id: buildMorphismId(couple, contract, channel, variant),
    seq: seqIdx,
    category: 'nanjian_morphism',
    couple:   { id: couple.id,    name: couple.name,    side: couple.side,    meaning: couple.meaning },
    contract: { id: contract.id,  name: contract.name,  category: contract.category, meaning: contract.meaning },
    channel:  { id: channel.id,   name: channel.name,   type: channel.type,   meaning: channel.meaning },
    variant,
    kind: `${couple.name}×${contract.name}×${channel.name}×${variant}`,
    name: `${couple.name}-${contract.name}-${channel.name}-${variant}`,
    meaning: meaningArray.join(' '),
    attribute: `${contract.category} | ${channel.type} | ${couple.side}`,
    isSovereign: !!sovereign,
    /** 夫妻源头主权签名（信任/规则/权力/契约源头） */
    sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
  };
}

class NanjianMorphismRegistry {
  constructor() {
    this.version = '0.1.0-nanjian-morphisms';
    this.sovereign = NJ3.SOVEREIGN_NANJIAN.origin;
    this.couples  = COUPLE_MORPHS.slice();
    this.contracts = CONTRACTS.slice();
    this.channels = CHANNELS.slice();
    this.variants = VARIANTS.slice();
    this.items = new Map(); // id -> morphism
    this.byCouple   = new Map();
    this.byContract = new Map();
    this.byChannel  = new Map();
    this.byVariant  = new Map();
    this.byType     = new Map(); // core/info/observe/predict/value
    let idx = 0;
    for (const c of COUPLE_MORPHS) {
      for (const ct of CONTRACTS) {
        for (const ch of CHANNELS) {
          for (const v of VARIANTS) {
            const m = buildMorphism(c, ct, ch, v, ++idx);
            this.items.set(m.id, m);
            if (!this.byCouple.has(c.id)) this.byCouple.set(c.id, []);
            this.byCouple.get(c.id).push(m);
            if (!this.byContract.has(ct.id)) this.byContract.set(ct.id, []);
            this.byContract.get(ct.id).push(m);
            if (!this.byChannel.has(ch.id)) this.byChannel.set(ch.id, []);
            this.byChannel.get(ch.id).push(m);
            if (!this.byVariant.has(v)) this.byVariant.set(v, []);
            this.byVariant.get(v).push(m);
            if (!this.byType.has(ch.type)) this.byType.set(ch.type, []);
            this.byType.get(ch.type).push(m);
          }
        }
      }
    }
    this.total = this.items.size;
  }

  get(id)          { return this.items.get(id) || null; }
  list(options = {}) {
    const { offset = 0, limit = this.total, couple, contract, channel, variant, type, isSovereign } = options;
    let arr = [...this.items.values()];
    if (couple)     arr = arr.filter(m => m.couple.id === couple);
    if (contract)   arr = arr.filter(m => m.contract.id === contract);
    if (channel)    arr = arr.filter(m => m.channel.id === channel);
    if (variant)    arr = arr.filter(m => m.variant === variant);
    if (type)       arr = arr.filter(m => m.channel.type === type);
    if (typeof isSovereign === 'boolean') arr = arr.filter(m => m.isSovereign === isSovereign);
    arr = arr.slice(offset, offset + limit);
    return arr;
  }
  /** 三元规则裁判：范畴管理者（按 type × contract 交叉分类返回） */
  refereeCategories() {
    const out = {};
    for (const [type, list] of this.byType) {
      out[type] = {};
      for (const ct of CONTRACTS) {
        const matched = list.filter(m => m.contract.id === ct.id);
        out[type][ct.name] = { count: matched.length, sample: matched.slice(0, 3).map(m => m.name) };
      }
    }
    return out;
  }
  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      total: this.total,
      byCoupleCount:    Object.fromEntries([...this.byCouple.entries()].map(([k,v]) => [k, v.length])),
      byContractCount:  Object.fromEntries([...this.byContract.entries()].map(([k,v]) => [k, v.length])),
      byChannelCount:   Object.fromEntries([...this.byChannel.entries()].map(([k,v]) => [k, v.length])),
      byVariantCount:   Object.fromEntries([...this.byVariant.entries()].map(([k,v]) => [k, v.length])),
      byTypeCount:      Object.fromEntries([...this.byType.entries()].map(([k,v]) => [k, v.length])),
      expectedProduct: COUPLE_MORPHS.length * CONTRACTS.length * CHANNELS.length * VARIANTS.length,
      categories_types: [...this.byType.keys()],
      sovereignMorphisms: this.list({ isSovereign: true }).length,
    };
  }
}

module.exports = {
  COUPLE_MORPHS,
  CONTRACTS,
  CHANNELS_NANJIAN: CHANNELS,
  VARIANTS_NANJIAN: VARIANTS,
  NanjianMorphismRegistry,
  buildMorphismId,
};
