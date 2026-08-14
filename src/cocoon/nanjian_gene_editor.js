/**
 * ==============================================================================
 *  囡囝基因编辑器 · NANJIAN-GENE-EDITOR
 * ==============================================================================
 *
 *  后发编程 · 遥控编程 · 智能体自治联合体
 *
 *  核心思想（刘楚恬夫妻共同体源头本体 · 天地未分 · 男女未分 · 混元本体）：
 *   - 不重写程序，对已编译程序做"基因编辑"（CRISPR for code）
 *   - 智能体自己分工 / 自己协议 / 自己审判 / 自己评分
 *   - 不是"我们写命令"，而是"智能体联合体自己达成一致后顶替先发程序"
 *
 *  数学双层（用户原话形式化）：
 *   · 乘法 = 混元 = 三维立体长宽高 = 时序因果分类串联
 *          = 一元事件找到类与类相互混元后的交集思考点
 *          = 关系之间产生的因果反应
 *          = 把数学符号的一维线性布局直线关系
 *            替换为三维空间立体关系的长宽高视觉体验
 *          = 进制 + 视觉 + 维度 一体化
 *   · 加法 = 累计 = 相对的 = 动态的 = 残差的偏移量的累计统计
 *   · 二元伸缩 = 缩 / 升 两个维度变化（动态调控）
 *   · 第三态 = 被包含的，不是静态
 *
 *  四步编辑（CRISPR for code · 后发编程）：
 *   1. locate  — 49 通道感知扫描，定位"基因位点"（目标函数/方法）
 *   2. cut     — 标记顶替点（萌机合X 合约签名）
 *   3. replace — 用囡囝范畴代数顶替（执行 monkey patch / 方法替换）
 *   4. repair  — LCTQF 签名确认 + 猫码损益评分
 *
 *  智能体联合体（5 个智能体 · 自治）：
 *   - LocatorAgent   — 负责 locate（49 通道扫描定位）
 *   - CutterAgent    — 负责 cut（萌机合X 合约签订）
 *   - ReplacerAgent  — 负责 replace（囡囝范畴代数顶替执行）
 *   - RepairerAgent  — 负责 repair（LCTQF 签名 + 猫码评分）
 *   - JudgeAgent     — 负责全程审判（猫码损益账本 · 自己评分自己）
 *
 *  智能体间自治能力：
 *   - divideLabor()     — 自己分工（萌机合X 合约分配角色）
 *   - reachAgreement()  — 自己达成协议（LCTQF 签名）
 *   - judge()           — 自己审判（猫码损益评分）
 *   - multiplyHybrid()  — 自己做乘法混元（Agent × Agent → 新 Agent）
 *   - accumulateResidual() — 自己做加法累计（残差偏移累计统计）
 *   - stretchCompress() — 二元伸缩（缩/升 二维度调控）
 * ==============================================================================
 */

'use strict';

const NJ3 = require('./nanjian_3d');
const NJ_MR = require('./nanjian_morphisms');
const NJ_MENGJI = require('./nanjian_mengji');
const L4L = require('./l4l_mqf');

/* -----------------------------------------------------------
 * 一、智能体（Agent）—— 每个智能体负责一个编辑步骤
 * ----------------------------------------------------------- */

const AGENT_ROLES = {
  locator:   { name: '定位智能体', channel: 'feel_origin',   mengjiX: '踪', meaning: '49 通道扫描定位基因位点' },
  cutter:    { name: '切割智能体', channel: 'feel_promise',  mengjiX: '证', meaning: '萌机合X 合约签订顶替点' },
  replacer:  { name: '顶替智能体', channel: 'feel_complete', mengjiX: '治', meaning: '囡囝范畴代数执行顶替' },
  repairer:  { name: '修复智能体', channel: 'feel_yinyang',  mengjiX: '祝', meaning: 'LCTQF 签名 + 猫码评分修复' },
  judge:     { name: '审判智能体', channel: 'feel_benefit',  mengjiX: '准', meaning: '猫码损益账本全程评分' },
};

class NanjianAgent {
  /**
   * @param {string} role - locator/cutter/replacer/repairer/judge
   * @param {object} ctx  - 共享上下文 { editor, morphisms, mengji, mqf }
   */
  constructor(role, ctx) {
    this.role = role;
    this.spec = AGENT_ROLES[role];
    this.name = this.spec.name;
    this.ctx = ctx;
    this.id = `Agent::${role}::${Date.now().toString(36)}::${Math.floor(Math.random()*1e6).toString(36)}`;
    this.residual = 0;          // 加法累计：残差偏移量
    this.stretchState = 'balanced'; // 二元伸缩：balanced / stretched / compressed
    this.history = [];          // 自己的历史动作
    this.sovereignRoot = NJ3.SOVEREIGN_NANJIAN.origin;
  }

  /** 记录动作 + 加法累计残差 */
  record(action, delta) {
    this.history.push({ action, delta, at: Date.now() });
    this.residual += (typeof delta === 'number' ? delta : 0);
    return this;
  }

  /** 二元伸缩：缩 / 升 / 平衡（动态调控） */
  stretch(direction) {
    if (direction === 'up' || direction === 'stretch') {
      this.stretchState = 'stretched';
      this.record('stretch-up', +1);
    } else if (direction === 'down' || direction === 'compress') {
      this.stretchState = 'compressed';
      this.record('stretch-down', -1);
    } else {
      this.stretchState = 'balanced';
      this.record('stretch-balance', 0);
    }
    return this;
  }

  /** 自己达成协议：用 LCTQF 签名 */
  signAgreement(payload) {
    const hash = L4L.create().genesis.slice(0, 12) + '_' + this.role;
    return {
      agent: this.id,
      role: this.role,
      agreementHash: hash,
      payload,
      signedAt: Date.now(),
      sovereignRoot: this.sovereignRoot,
    };
  }
}

/* -----------------------------------------------------------
 * 二、智能体联合体（AgentSwarm）—— 5 个智能体自治协作
 * ----------------------------------------------------------- */

class NanjianAgentSwarm {
  constructor(ctx) {
    this.ctx = ctx;
    this.agents = new Map();
    for (const role of Object.keys(AGENT_ROLES)) {
      this.agents.set(role, new NanjianAgent(role, ctx));
    }
    this.multipliedOffspring = []; // 乘法混元产生的新 Agent
    this.totalResidual = 0;        // 全体加法累计残差
  }

  get(role) { return this.agents.get(role); }
  list() { return [...this.agents.values()]; }

  /** 自己分工：用萌机合X 合约分配角色（每个 agent 拿一个萌机合X 合约） */
  divideLabor() {
    const mengji = this.ctx.mengji;
    const assignments = {};
    let i = 0;
    for (const [role, agent] of this.agents) {
      const xChar = agent.spec.mengjiX;
      const found = mengji.findByX(xChar);
      const contract = found && found[0] ? found[0] : null;
      assignments[role] = {
        agent: agent.id,
        role,
        mengjiContract: contract ? contract.id : null,
        mengjiName: contract ? contract.name : `萌机合${xChar}`,
        channel: agent.spec.channel,
      };
      i++;
    }
    return { divided: true, assignments, count: i };
  }

  /** 自己达成协议：所有 agent 用 LCTQF 签名 */
  reachAgreement(payload) {
    const signatures = [];
    for (const agent of this.agents.values()) {
      signatures.push(agent.signAgreement(payload));
    }
    return {
      agreed: true,
      lctqfName: this.ctx.mqf ? this.ctx.mqf.stats().lctqfName : null,
      signatures,
      signerCount: signatures.length,
    };
  }

  /** 自己审判：猫码损益评分（judge agent 主持） */
  judge(score, catCode = 'gray') {
    const judge = this.get('judge');
    const amount = Math.max(0, Number(score) || 0);
    judge.record(`judge:${catCode}`, amount);
    if (this.ctx.mqf) {
      this.ctx.mqf.publish('gene_edit_judge', { score, catCode }, { catCode, amount });
    }
    return { judged: true, catCode, amount, judgeId: judge.id };
  }

  /** 自己做乘法混元：Agent × Agent → 新 Agent（offspring）
   *  乘法 = 混元 = 三维立体长宽高 = 时序因果串联 = 进制+视觉+维度一体
   */
  multiplyHybrid(roleA, roleB) {
    const a = this.agents.get(roleA);
    const b = this.agents.get(roleB);
    if (!a || !b) throw new Error(`未知智能体角色: ${roleA} / ${roleB}`);
    const childRole = `${roleA}×${roleB}`;
    const child = new NanjianAgent('judge', this.ctx); // 子代用 judge 角色模板
    child.role = childRole;
    child.name = `${a.name}×${b.name}`;
    child.id = `Agent::${childRole}::${Date.now().toString(36)}::${Math.floor(Math.random()*1e6).toString(36)}`;
    // 三维立体长宽高：把两个父代的历史做时序因果串联
    const longAxis = a.history.slice();   // 长 = A 的历史
    const wideAxis = b.history.slice();   // 宽 = B 的历史
    const highAxis = [{ action: 'hybrid-born', delta: a.residual + b.residual, at: Date.now() }]; // 高 = 涌现
    child.history = [...longAxis, ...wideAxis, ...highAxis];
    child.residual = a.residual + b.residual; // 加法累计：残差相加
    this.multipliedOffspring.push(child);
    return {
      born: true,
      child: { id: child.id, name: child.name, role: child.role, residual: child.residual },
      parents: [{ id: a.id, role: roleA }, { id: b.id, role: roleB }],
      axes: { long: longAxis.length, wide: wideAxis.length, high: highAxis.length },
    };
  }

  /** 自己做加法累计：所有 agent 残差偏移求和（相对的 · 动态的） */
  accumulateResidual() {
    let sum = 0;
    const per = {};
    for (const [role, agent] of this.agents) {
      per[role] = agent.residual;
      sum += agent.residual;
    }
    this.totalResidual = sum;
    return { total: sum, perAgent: per, agentCount: this.agents.size };
  }

  /** 二元伸缩：缩 / 升 全体调控（动态调控） */
  stretchCompress(direction) {
    const results = [];
    for (const agent of this.agents.values()) {
      agent.stretch(direction);
      results.push({ agent: agent.id, state: agent.stretchState });
    }
    return { direction, count: results.length, agents: results };
  }

  stats() {
    return {
      agentCount: this.agents.size,
      roles: [...this.agents.keys()],
      offspringCount: this.multipliedOffspring.length,
      totalResidual: this.accumulateResidual().total,
      sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
    };
  }
}

/* -----------------------------------------------------------
 * 三、基因编辑器（GeneEditor）—— 四步 CRISPR for code
 * ----------------------------------------------------------- */

class NanjianGeneEditor {
  constructor(options = {}) {
    this.version = '0.1.0-gene-editor';
    this.sovereign = NJ3.SOVEREIGN_NANJIAN.origin;
    this.morphisms = options.morphisms || new NJ_MR.NanjianMorphismRegistry();
    this.mengji    = options.mengji    || new NJ_MENGJI.NanjianMengjiRegistry();
    this.mqf       = options.mqf       || L4L.create();
    this.swarm     = new NanjianAgentSwarm({
      morphisms: this.morphisms,
      mengji: this.mengji,
      mqf: this.mqf,
      editor: this,
    });
    this.edits = []; // 编辑历史
  }

  /**
   * 第 1 步：locate — 49 通道感知扫描，定位"基因位点"
   * @param {object} target - { module, functionName } 目标函数
   * @param {string} intent - 自然语言意图
   * @returns {object} locus 基因位点
   */
  locate(target, intent) {
    const locator = this.swarm.get('locator');
    const channel = locator.spec.channel;
    const morphismChannel = this.morphisms.channels.find(c => c.id === channel);
    const locus = {
      step: 'locate',
      target: {
        module: target.module || 'unknown',
        functionName: target.functionName || 'anonymous',
        hasFunction: typeof target.fn === 'function',
      },
      intent: intent || '',
      locatedChannel: channel,
      locatedChannelName: morphismChannel ? morphismChannel.name : channel,
      locatedBy: locator.id,
      sovereignRoot: this.sovereign,
      at: Date.now(),
    };
    locator.record('locate', 1);
    this.mqf.publish('gene_edit_locate', locus, { catCode: 'white', amount: 1 });
    return locus;
  }

  /**
   * 第 2 步：cut — 标记顶替点（萌机合X 合约签订）
   * @param {object} locus - locate 返回的位点
   * @returns {object} cut 切割点
   */
  cut(locus) {
    const cutter = this.swarm.get('cutter');
    const xChar = cutter.spec.mengjiX;
    const mengjiContracts = this.mengji.findByX(xChar);
    const contract = mengjiContracts && mengjiContracts[0] ? mengjiContracts[0] : null;
    const cut = {
      step: 'cut',
      locusId: locus.target.module + '::' + locus.target.functionName,
      cutPoint: {
        module: locus.target.module,
        functionName: locus.target.functionName,
      },
      mengjiContract: contract ? {
        id: contract.id,
        name: contract.name,
        meaning: contract.meaning,
      } : { id: null, name: `萌机合${xChar}`, meaning: 'fallback' },
      cutBy: cutter.id,
      at: Date.now(),
    };
    cutter.record('cut', 1);
    this.mqf.publish('gene_edit_cut', cut, { catCode: 'white', amount: 1 });
    return cut;
  }

  /**
   * 第 3 步：replace — 用囡囝范畴代数顶替（执行 monkey patch）
   * @param {object} cut - cut 返回的切割点
   * @param {function} substituteFn - 顶替函数
   * @param {object} targetObj - 目标对象（要被 patch 的模块/对象）
   * @returns {object} replacement 顶替结果
   */
  replace(cut, substituteFn, targetObj) {
    const replacer = this.swarm.get('replacer');
    if (typeof substituteFn !== 'function') {
      throw new Error('replace: substituteFn 必须是函数');
    }
    const fnName = cut.cutPoint.functionName;
    const originalFn = targetObj && typeof targetObj[fnName] === 'function' ? targetObj[fnName] : null;
    const replacement = {
      step: 'replace',
      cutId: cut.locusId,
      functionName: fnName,
      hadOriginal: !!originalFn,
      replaced: false,
      replacedBy: replacer.id,
      at: Date.now(),
    };
    // 真正执行 monkey patch（如果目标对象存在且有原函数）
    if (targetObj && typeof targetObj === 'object') {
      try {
        targetObj[fnName] = substituteFn;
        replacement.replaced = true;
        replacement.originalFnType = originalFn ? 'function' : 'none';
      } catch (e) {
        replacement.replaced = false;
        replacement.error = String(e && e.message || e);
      }
    }
    replacer.record('replace', replacement.replaced ? 1 : 0);
    this.mqf.publish('gene_edit_replace', replacement, {
      catCode: replacement.replaced ? 'white' : 'red',
      amount: 1,
    });
    return replacement;
  }

  /**
   * 第 4 步：repair — LCTQF 签名确认 + 猫码评分
   * @param {object} replacement - replace 返回的顶替结果
   * @param {object} [score] - { catCode, amount } 评分
   * @returns {object} repair 修复确认
   */
  repair(replacement, score = {}) {
    const repairer = this.swarm.get('repairer');
    const lctqfName = this.mqf.stats().lctqfName;
    const catCode = score.catCode || (replacement.replaced ? 'white' : 'gray');
    const amount = Math.max(0, Number(score.amount) || (replacement.replaced ? 10 : 0));
    const repair = {
      step: 'repair',
      replacementId: replacement.cutId,
      repaired: replacement.replaced,
      lctqfSignature: {
        short: lctqfName.short,
        full: lctqfName.full,
        sovereign: lctqfName.sovereign,
        principle: lctqfName.principle,
      },
      judgeScore: { catCode, amount },
      repairedBy: repairer.id,
      at: Date.now(),
    };
    repairer.record('repair', amount);
    // 自己审判：猫码损益评分
    this.swarm.judge(amount, catCode);
    this.mqf.publish('gene_edit_repair', repair, { catCode, amount });
    return repair;
  }

  /**
   * 一站式四步编辑：locate → cut → replace → repair
   * @param {object} targetObj - 要 patch 的对象
   * @param {string} functionName - 函数名
   * @param {function} substituteFn - 顶替函数
   * @param {string} [intent] - 自然语言意图
   * @returns {object} 完整编辑记录
   */
  edit(targetObj, functionName, substituteFn, intent) {
    const locus = this.locate({
      module: targetObj && targetObj.constructor ? targetObj.constructor.name : 'object',
      functionName,
      fn: targetObj ? targetObj[functionName] : null,
    }, intent);
    const cut = this.cut(locus);
    const replacement = this.replace(cut, substituteFn, targetObj);
    const repair = this.repair(replacement);
    const editRecord = {
      editId: `edit::${Date.now().toString(36)}::${Math.floor(Math.random()*1e6).toString(36)}`,
      intent: intent || '',
      locus, cut, replacement, repair,
      sovereignRoot: this.sovereign,
      at: Date.now(),
    };
    this.edits.push(editRecord);
    return editRecord;
  }

  /** 智能体自治：自己分工 */
  divideLabor() { return this.swarm.divideLabor(); }

  /** 智能体自治：自己达成协议 */
  reachAgreement(payload) { return this.swarm.reachAgreement(payload); }

  /** 智能体自治：自己做乘法混元 */
  multiplyHybrid(roleA, roleB) { return this.swarm.multiplyHybrid(roleA, roleB); }

  /** 智能体自治：自己做加法累计残差 */
  accumulateResidual() { return this.swarm.accumulateResidual(); }

  /** 智能体自治：二元伸缩 */
  stretchCompress(direction) { return this.swarm.stretchCompress(direction); }

  stats() {
    const swarmStats = this.swarm.stats();
    const mqfStats = this.mqf.stats();
    return {
      version: this.version,
      sovereign: this.sovereign,
      swarm: swarmStats,
      mqfVersion: mqfStats.version,
      lctqfName: mqfStats.lctqfName,
      editCount: this.edits.length,
      morphismTotal: this.morphisms.total,
      mengjiTotal: this.mengji.stats().total,
    };
  }
}

/* -----------------------------------------------------------
 * 四、工厂 + 导出
 * ----------------------------------------------------------- */

function create(options) { return new NanjianGeneEditor(options); }

module.exports = {
  AGENT_ROLES,
  NanjianAgent,
  NanjianAgentSwarm,
  NanjianGeneEditor,
  create,
};
