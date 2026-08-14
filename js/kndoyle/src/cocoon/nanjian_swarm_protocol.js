/**
 * ============================================================
 * 囡囝智能体深度自治协议 · NANJIAN-SWARM-PROTOCOL
 *    5 智能体 P2P 自治：分工投票 / 审判上诉 / 残差共振 / 自动伸缩 / 第三态识别
 * ============================================================
 *
 *  理论来源（用户原话）：
 *    「智能体自治联合体」—— 不是"我们写命令"，
 *     而是"智能体联合体自己达成一致后顶替先发程序"
 *
 *  T6 六大自治能力（基于 l4l_mqf 消息链做 P2P 通信）：
 *    1. broadcast(msg)        智能体间 P2P 广播（每个 agent 一个 mailbox topic）
 *    2. vote(topic, motion)   分工投票（萌机合X 合约自动分配 + 多数决）
 *    3. appeal(judgment)      审判上诉（judge 判定可被其他智能体质疑 + 翻案）
 *    4. triggerHybrid()       乘法混元自动触发（残差共振 > 阈值 → 自动 multiplyHybrid）
 *    5. autoStretch(pressure) 二元伸缩自动调控（外部压力感知 → 缩/升）
 *    6. detectThirdState()    第三态识别（被包含的非静态关系）
 *
 *  与现有系统联动：
 *    - nanjian_gene_editor.js: 复用 NanjianAgentSwarm 的 5 agent + multiplyHybrid/stretchCompress
 *    - l4l_mqf.js: P2P 消息链（每条带 LCTQF 签名 + 猫码损益）
 *    - nanjian_mengji.js: 分工投票用萌机合X 合约
 *    - nanjian_variant_algebra.js: 第三态识别用变体加法群
 *    - nanjian_micro_macro.js: 自动伸缩的"外部压力" = 宏量采样
 *
 *  P2P 消息格式（JSON）：
 *    { id, from, to, topic, payload, lctqfSignature, at, sovereign }
 *      - to: 'broadcast' | agentRole | 'judge' | 'all'
 *      - topic: 'vote' | 'appeal' | 'hybrid' | 'stretch' | 'third_state' | custom
 * ============================================================
 */

'use strict';

const NJ3 = require('./nanjian_3d');
const AGENT_ROLES_DEF = require('./nanjian_gene_editor').AGENT_ROLES;

// ============================================================
// 一、P2P 消息协议（基于 l4l_mqf 的智能体间通信）
// ============================================================

const SWARM_TOPIC = 'swarm_p2p';       // 公共广播 topic
const VOTE_TOPIC = 'swarm_vote';        // 投票 topic
const APPEAL_TOPIC = 'swarm_appeal';    // 上诉 topic
const HYBRID_TOPIC = 'swarm_hybrid';    // 乘法混元 topic
const STRETCH_TOPIC = 'swarm_stretch';  // 二元伸缩 topic
const THIRD_STATE_TOPIC = 'swarm_third_state'; // 第三态识别 topic

/**
 * 构造一条 P2P 消息（智能体间 JSON 协议）
 *   @param {string} from  发送方 agent 角色
 *   @param {string} to    接收方（'broadcast' | agentRole | 'all' | 'judge'）
 *   @param {string} topic 消息主题
 *   @param {object} payload 消息内容
 *   @param {object} [lctqfSig] LCTQF 签名（可选）
 */
function makeSwarmMessage(from, to, topic, payload, lctqfSig = null) {
  return {
    id: `swarm_${Date.now().toString(36)}_${Math.floor(Math.random() * 1e6).toString(36)}`,
    from,
    to,
    topic,
    payload,
    lctqfSignature: lctqfSig || `LCTQF_${from}_${topic}`,
    at: Date.now(),
    sovereign: NJ3.SOVEREIGN_NANJIAN.origin,
  };
}

// ============================================================
// 二、SwarmProtocol 类：5 智能体 P2P 自治协议
// ============================================================

class SwarmProtocol {
  /**
   * @param {object} options
   *   - swarm:    NanjianAgentSwarm 实例（5 agent）
   *   - mqf:      L4LMQF 实例（消息链）
   *   - mengji:   萌机合X 注册表（分工投票用）
   *   - geneEditor: NanjianGeneEditor 实例（乘法混元/伸缩调用）
   *   - variantAlgebra: VariantAlgebra 实例（第三态识别用）
   *   - fraudDetector: FraudDetector 实例（上诉欺诈检测）
   *   - residualThreshold: 残差共振阈值（默认 5，超过即触发乘法混元）
   *   - pressureThreshold:  外部压力阈值（默认 0.7，超过即触发升伸缩）
   */
  constructor(options = {}) {
    this.version = '0.1.0-nanjian-swarm-protocol';
    this.sovereign = NJ3.SOVEREIGN_NANJIAN.origin;
    this.swarm = options.swarm || null;
    this.mqf = options.mqf || null;
    this.mengji = options.mengji || null;
    this.geneEditor = options.geneEditor || null;
    this.variantAlgebra = options.variantAlgebra || null;
    this.fraudDetector = options.fraudDetector || null;

    // 便捷布尔属性（与 stats() 一致，便于外部直接判断依赖是否注入）
    this.hasSwarm = !!this.swarm;
    this.hasMqf = !!this.mqf;
    this.hasMengji = !!this.mengji;
    this.hasGeneEditor = !!this.geneEditor;
    this.hasVariantAlgebra = !!this.variantAlgebra;
    this.hasFraudDetector = !!this.fraudDetector;

    this.residualThreshold = options.residualThreshold || 5;
    this.pressureThreshold = options.pressureThreshold || 0.7;

    // P2P 邮箱：每个 agent 一个 mailbox topic（agent::role）
    this.mailboxes = new Map();  // role -> subscriberId
    // 全局消息日志（P2P 收发全部留痕）
    this.messageLog = [];
    // 投票记录
    this.votes = [];            // {topic, motion, votes:{role:choice}, result}
    // 上诉记录
    this.appeals = [];          // {judgment, appellant, votes, overturned}
    // 第三态记录
    this.thirdStates = [];      // {container, contained, isStatic, detected}
    // 自动触发历史
    this.autoTriggers = [];     // {type, reason, at}

    this._setupMailboxes();
  }

  /** 为每个 agent 建立 P2P mailbox topic */
  _setupMailboxes() {
    if (!this.swarm || !this.mqf) return;
    for (const role of this.swarm.agents.keys()) {
      const topic = this._mailboxTopic(role);
      // 订阅自己的 mailbox（实际 cb 在 broadcast 时按 to 字段分发）
      const subId = this.mqf.subscribe(topic, () => {});
      this.mailboxes.set(role, subId);
    }
  }

  /** agent 的 mailbox topic 名 */
  _mailboxTopic(role) {
    return `agent::${role}`;
  }

  // ============================================================
  // 能力 1：broadcast 智能体间 P2P 广播
  // ============================================================

  /**
   * P2P 广播：一个 agent 向其他 agent 发消息
   *   @param {string} fromRole  发送方角色
   *   @param {string} to        接收方 ('broadcast' | 'all' | agentRole | 'judge')
   *   @param {string} topic     消息主题
   *   @param {object} payload   消息内容
   *   @param {object} [opts]    { catCode, amount, attentionTo }
   *   @returns {object} 发送结果 + 接收方列表
   */
  broadcast(fromRole, to, topic, payload, opts = {}) {
    if (!this.swarm) throw new Error('SwarmProtocol 需要 swarm 实例');
    if (!this.swarm.agents.has(fromRole)) {
      throw new Error(`未知发送方角色: ${fromRole}`);
    }
    const msg = makeSwarmMessage(fromRole, to, topic, payload);
    this.messageLog.push({ direction: 'out', ...msg });

    // 决定接收方列表
    const recipients = this._resolveRecipients(to);

    // 投递到每个接收方的 mailbox topic（基于 l4l_mqf）
    let delivered = 0;
    if (this.mqf) {
      this.mqf.publish(SWARM_TOPIC, msg, {
        catCode: opts.catCode || 'gray',
        amount: opts.amount || 1,
        attentionTo: recipients.map(r => this._mailboxTopic(r)),
      });
      // 同时投递到每个接收方的 mailbox topic（便于按 agent 检索）
      for (const r of recipients) {
        this.mqf.publish(this._mailboxTopic(r), msg, {
          catCode: opts.catCode || 'gray',
          amount: opts.amount || 1,
        });
        this.messageLog.push({ direction: 'in', to: r, ...msg });
        delivered++;
      }
    } else {
      delivered = recipients.length;
    }

    return {
      sent: true,
      messageId: msg.id,
      from: fromRole,
      to,
      topic,
      recipients,
      delivered,
      sovereign: this.sovereign,
    };
  }

  /** 解析接收方列表 */
  _resolveRecipients(to) {
    if (to === 'broadcast' || to === 'all') {
      return [...this.swarm.agents.keys()];
    }
    if (to === 'judge') return ['judge'];
    if (this.swarm.agents.has(to)) return [to];
    // 未知接收方 → 当广播处理
    return [...this.swarm.agents.keys()];
  }

  /** 取某 agent 收到的所有消息 */
  inbox(role) {
    if (!this.swarm || !this.swarm.agents.has(role)) {
      throw new Error(`未知角色: ${role}`);
    }
    return this.messageLog.filter(m => m.direction === 'in' && m.to === role);
  }

  // ============================================================
  // 能力 2：vote 分工投票（萌机合X 合约自动分配 + 多数决）
  // ============================================================

  /**
   * 分工投票：对一个 motion 发起投票，5 agent 各投一票
   *   - 每个 agent 用自己的萌机合X 合约签名投票
   *   - 多数决（>=3 票通过）
   *   @param {string} motion  议案（如 "谁负责 locate"）
   *   @param {object} [choices] 可选 { role: choice } 预设票
   *   @returns {object} 投票结果 { motion, votes, passed, winner }
   */
  vote(motion, choices = {}) {
    if (!this.swarm) throw new Error('SwarmProtocol 需要 swarm 实例');
    const votes = {};
    // 每个 agent 投票（默认按萌机合X 合约倾向投票，预设 choices 优先）
    for (const [role, agent] of this.swarm.agents) {
      let choice = choices[role];
      if (!choice) {
        // 默认投票策略：judge 投"通过"，其他 agent 投自己倾向
        choice = role === 'judge' ? 'pass' : `assign:${role}`;
      }
      votes[role] = {
        choice,
        mengjiX: agent.spec.mengjiX,
        signedAt: Date.now(),
      };
      // P2P 广播投票消息
      this.broadcast(role, 'judge', 'vote', { motion, choice }, { catCode: 'gray', amount: 1 });
    }

    // 多数决：统计 choice 出现次数
    const tally = {};
    for (const v of Object.values(votes)) {
      tally[v.choice] = (tally[v.choice] || 0) + 1;
    }
    const sortedTally = Object.entries(tally).sort((a, b) => b[1] - a[1]);
    const winner = sortedTally[0] ? sortedTally[0][0] : null;
    const winnerCount = sortedTally[0] ? sortedTally[0][1] : 0;
    const passed = winnerCount >= 3;  // 多数决：5 agent 中 >=3 票通过

    const result = {
      motion,
      votes,
      tally,
      winner,
      winnerCount,
      passed,
      totalVoters: Object.keys(votes).length,
      threshold: 3,
      at: Date.now(),
      sovereign: this.sovereign,
    };
    this.votes.push(result);

    // 上链审计
    if (this.mqf) {
      this.mqf.publish(VOTE_TOPIC, result, {
        catCode: passed ? 'white' : 'red',
        amount: Math.max(1, winnerCount),
      });
    }
    return result;
  }

  /** 分工投票的便捷封装：投票决定谁负责某个步骤 */
  voteForRole(stepName) {
    return this.vote(`谁负责 ${stepName} 步骤`);
  }

  // ============================================================
  // 能力 3：appeal 审判上诉（judge 判定可被其他智能体质疑 + 翻案）
  // ============================================================

  /**
   * 审判上诉：某个 agent 对 judge 的判决提出质疑，其他 agent 投票决定是否翻案
   *   @param {object} judgment     被上诉的判决对象 { catCode, amount, judgedAt }
   *   @param {string} appellant    上诉方角色（非 judge）
   *   @param {string} reason       上诉理由
   *   @returns {object} 上诉结果 { judgment, appellant, votes, overturned, finalCatCode }
   */
  appeal(judgment, appellant, reason) {
    if (!this.swarm) throw new Error('SwarmProtocol 需要 swarm 实例');
    if (appellant === 'judge') {
      throw new Error('judge 不能上诉自己的判决');
    }
    if (!this.swarm.agents.has(appellant)) {
      throw new Error(`未知上诉方角色: ${appellant}`);
    }

    // 上诉方发起 P2P 广播
    this.broadcast(appellant, 'broadcast', 'appeal', {
      judgment,
      reason,
    }, { catCode: 'yellow' || 'gray', amount: 1 });

    // 其他 agent 投票（judge 不能投，上诉方也不投自己）
    const voters = [...this.swarm.agents.keys()].filter(r => r !== appellant && r !== 'judge');
    const votes = {};
    let upholdCount = 0;   // 维持原判
    let overturnCount = 0; // 翻案
    for (const role of voters) {
      // 默认投票策略：replacer/repairer 倾向翻案（执行方对判决敏感），locator 倾向维持
      const choice = (role === 'replacer' || role === 'repairer') ? 'overturn' : 'uphold';
      votes[role] = choice;
      if (choice === 'overturn') overturnCount++;
      else upholdCount++;
    }

    // 翻案需要多数（> 1/2 投票方）
    const overturned = overturnCount > upholdCount;
    // 翻案后 catCode 降级：white→gray, black→gray, red→gray（减轻处罚）
    const finalCatCode = overturned
      ? (judgment && judgment.catCode === 'red' ? 'gray' : 'gray')
      : (judgment && judgment.catCode) || 'gray';
    const finalAmount = overturned
      ? Math.floor((judgment && judgment.amount || 0) / 2)
      : (judgment && judgment.amount || 0);

    const result = {
      judgment,
      appellant,
      reason,
      votes,
      upholdCount,
      overturnCount,
      overturned,
      finalCatCode,
      finalAmount,
      at: Date.now(),
      sovereign: this.sovereign,
    };
    this.appeals.push(result);

    // 上链审计
    if (this.mqf) {
      this.mqf.publish(APPEAL_TOPIC, result, {
        catCode: overturned ? 'white' : 'red',
        amount: Math.max(1, finalAmount),
      });
    }
    return result;
  }

  // ============================================================
  // 能力 4：triggerHybrid 乘法混元自动触发（残差共振阈值）
  // ============================================================

  /**
   * 乘法混元自动触发：当全体 agent 残差和 > 阈值 → 自动 multiplyHybrid
   *   用户理论：残差共振 → 触发乘法混元（Agent × Agent → 新 Agent）
   *   @param {string} [roleA]  默认 'locator'
   *   @param {string} [roleB]  默认 'replacer'
   *   @returns {object} 触发结果 { triggered, residual, threshold, hybrid }
   */
  triggerHybrid(roleA = 'locator', roleB = 'replacer') {
    if (!this.swarm) throw new Error('SwarmProtocol 需要 swarm 实例');
    if (!this.geneEditor) throw new Error('SwarmProtocol 需要 geneEditor 实例');

    // 计算全体残差共振
    const residualResult = this.swarm.accumulateResidual();
    const totalResidual = residualResult.total;
    const triggered = totalResidual > this.residualThreshold;

    let hybrid = null;
    if (triggered) {
      // P2P 广播触发信号
      this.broadcast('judge', 'broadcast', 'hybrid', {
        reason: 'residual_resonance',
        totalResidual,
        threshold: this.residualThreshold,
      }, { catCode: 'white', amount: totalResidual });

      // 调用基因编辑器的乘法混元
      hybrid = this.geneEditor.multiplyHybrid(roleA, roleB);

      // 上链审计
      if (this.mqf) {
        this.mqf.publish(HYBRID_TOPIC, {
          triggered: true,
          totalResidual,
          threshold: this.residualThreshold,
          hybrid,
        }, { catCode: 'white', amount: totalResidual });
      }
    }

    const result = {
      triggered,
      totalResidual,
      threshold: this.residualThreshold,
      perAgent: residualResult.perAgent,
      hybrid,
      roleA, roleB,
      at: Date.now(),
      sovereign: this.sovereign,
    };
    this.autoTriggers.push({ type: 'hybrid', ...result });
    return result;
  }

  // ============================================================
  // 能力 5：autoStretch 二元伸缩自动调控（外部压力感知）
  // ============================================================

  /**
   * 二元伸缩自动调控：感知外部压力 → 自动缩/升
   *   用户理论：外部压力 = 宏量（不确定）；压力大 → 升伸缩（扩容）；压力小 → 缩伸缩（收缩）
   *   @param {number|object} pressure 外部压力值（0~1）或 MacroQuantum 实例
   *   @returns {object} 调控结果 { pressure, direction, stretched, agents }
   */
  autoStretch(pressure) {
    if (!this.swarm) throw new Error('SwarmProtocol 需要 swarm 实例');
    if (!this.geneEditor) throw new Error('SwarmProtocol 需要 geneEditor 实例');

    // 解析压力值：MacroQuantum 实例 → sample()；数值 → 直接用
    let pressureValue;
    if (pressure && typeof pressure.sample === 'function') {
      pressureValue = pressure.sample();
    } else {
      pressureValue = Number(pressure);
    }
    if (isNaN(pressureValue)) pressureValue = 0;

    // 压力 > 阈值 → up（升伸缩扩容）；< 阈值/2 → down（缩伸缩收缩）；中间 → balance
    let direction;
    if (pressureValue > this.pressureThreshold) {
      direction = 'up';
    } else if (pressureValue < this.pressureThreshold / 2) {
      direction = 'down';
    } else {
      direction = 'balance';
    }

    // P2P 广播调控信号
    this.broadcast('judge', 'broadcast', 'stretch', {
      reason: 'external_pressure',
      pressure: pressureValue,
      direction,
    }, { catCode: 'gray', amount: 1 });

    // 调用基因编辑器的二元伸缩
    const stretched = this.geneEditor.stretchCompress(direction);

    const result = {
      pressure: pressureValue,
      threshold: this.pressureThreshold,
      direction,
      stretched,
      at: Date.now(),
      sovereign: this.sovereign,
    };
    this.autoTriggers.push({ type: 'stretch', ...result });

    // 上链审计
    if (this.mqf) {
      this.mqf.publish(STRETCH_TOPIC, result, {
        catCode: direction === 'up' ? 'white' : (direction === 'down' ? 'black' : 'gray'),
        amount: 1,
      });
    }
    return result;
  }

  // ============================================================
  // 能力 6：detectThirdState 第三态识别（被包含的非静态关系）
  // ============================================================

  /**
   * 第三态识别：识别"被包含的非静态关系"
   *   用户理论：第三态 = 被包含的，不是静态
   *     - 不是二元（对称/反对称），不是三元（锚点裁判）
   *     - 是"一个东西被另一个东西包含，但这个包含关系本身在变化"
   *     - 用变体加法群：若 add(A, B) ≠ A 且 ≠ B 且 ≠ SM0 → 第三态存在
   *   @param {string} containerVariant  容器变体 ID（如 'SMRR'）
   *   @param {string} containedVariant  被包含变体 ID（如 'SMRL'）
   *   @returns {object} 识别结果 { container, contained, addResult, isThirdState, reason }
   */
  detectThirdState(containerVariant, containedVariant) {
    if (!this.variantAlgebra) throw new Error('SwarmProtocol 需要 variantAlgebra 实例');

    // 用变体加法群判断：add(A, B) 是否产生既非 A 也非 B 也非单位元的新变体
    let addResult = null;
    let isThirdState = false;
    let reason = '';

    try {
      const r = this.variantAlgebra.add(containerVariant, containedVariant);
      addResult = r.result;
      // 第三态判定：合成结果不是 A、不是 B、不是单位元 SM0
      isThirdState = (r.result !== containerVariant
                   && r.result !== containedVariant
                   && r.result !== 'SM0');
      reason = isThirdState
        ? `add(${containerVariant}, ${containedVariant}) = ${r.result}（既非${containerVariant}也非${containedVariant}也非SM0）→ 第三态存在`
        : `add(${containerVariant}, ${containedVariant}) = ${r.result}（落回已知变体）→ 非第三态`;
    } catch (e) {
      reason = `加法失败: ${e.message}`;
    }

    // 第三态 = 被包含的非静态关系：合成结果代表"包含关系本身在变化"
    const result = {
      container: containerVariant,
      contained: containedVariant,
      addResult,
      isThirdState,
      isStatic: !isThirdState,  // 第三态 = 非静态
      reason,
      theory: '第三态 = 被包含的，不是静态（add(A,B) 产生新变体即第三态）',
      at: Date.now(),
      sovereign: this.sovereign,
    };
    this.thirdStates.push(result);

    // P2P 广播第三态识别结果
    this.broadcast('judge', 'broadcast', 'third_state', result, {
      catCode: isThirdState ? 'white' : 'gray',
      amount: 1,
    });

    // 上链审计
    if (this.mqf) {
      this.mqf.publish(THIRD_STATE_TOPIC, result, {
        catCode: isThirdState ? 'white' : 'gray',
        amount: 1,
      });
    }
    return result;
  }

  // ============================================================
  // 一站式自治循环：跑完所有自动触发条件检查
  // ============================================================

  /**
   * 自治循环：检查所有自动触发条件（残差共振 + 外部压力）
   *   @param {number|object} [pressure] 外部压力（可选）
   *   @returns {object} 自治循环结果 { hybrid, stretch }
   */
  autonomyLoop(pressure = null) {
    const hybrid = this.triggerHybrid();
    const stretch = pressure !== null ? this.autoStretch(pressure) : null;
    return {
      hybrid,
      stretch,
      loopAt: Date.now(),
      sovereign: this.sovereign,
    };
  }

  // ============================================================
  // 统计与查询
  // ============================================================

  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      hasSwarm: !!this.swarm,
      hasMqf: !!this.mqf,
      hasGeneEditor: !!this.geneEditor,
      hasVariantAlgebra: !!this.variantAlgebra,
      hasFraudDetector: !!this.fraudDetector,
      agentCount: this.swarm ? this.swarm.agents.size : 0,
      mailboxCount: this.mailboxes.size,
      messageLogCount: this.messageLog.length,
      voteCount: this.votes.length,
      appealCount: this.appeals.length,
      thirdStateCount: this.thirdStates.length,
      autoTriggerCount: this.autoTriggers.length,
      residualThreshold: this.residualThreshold,
      pressureThreshold: this.pressureThreshold,
    };
  }

  /** 列出所有投票记录 */
  listVotes() { return this.votes.slice(); }

  /** 列出所有上诉记录 */
  listAppeals() { return this.appeals.slice(); }

  /** 列出所有第三态识别记录 */
  listThirdStates() { return this.thirdStates.slice(); }

  /** 列出所有自动触发记录 */
  listAutoTriggers() { return this.autoTriggers.slice(); }

  /** 列出 P2P 消息日志 */
  listMessages() { return this.messageLog.slice(); }
}

// ============================================================
// 三、工厂函数 + 导出
// ============================================================

function create(options) {
  return new SwarmProtocol(options);
}

module.exports = {
  // 常量
  SWARM_TOPIC,
  VOTE_TOPIC,
  APPEAL_TOPIC,
  HYBRID_TOPIC,
  STRETCH_TOPIC,
  THIRD_STATE_TOPIC,
  // 协议
  SwarmProtocol,
  makeSwarmMessage,
  create,
};
