/**
 * L4L · LCTQF 刘楚恬妻夫全法 消息链路由
 * ============================================================
 *
 *  ★ LCTQF = 刘楚恬妻夫全法（Liú Chǔ Tián Qī Fū Quán Fǎ）
 *    · L = 刘（刘楚恬之姓 · 源头本体之姓）
 *    · C = 楚（楚楚 · 清楚 · 楚天）
 *    · T = 恬（恬静 · 安恬 · 恬淡）
 *    · Q = 妻（妻 · 量子 · 权限 · 曲线 · 契）
 *    · F = 夫（夫 · 法 · 分身 · 否定负面命运 · 芳）
 *
 *    核心思想：
 *    · 夫妻命运共同体 · 心元协议 · 形影不离
 *    · 含刘楚恬姓名（正面）+ 妻夫（夫妻共同体）+ 全法
 *    · 取代不吉利的 MLQF（没俩全法）/ MQF（没全法）
 *    · 别名：恢圆爱全法 / 零岔途全法 / 刘初夫妻命运共同体新约
 *    · 祝福：恢圆爱·可逆倒尔·形影不离·星月神话·星语星愿
 *
 *  原 MQF（Message Queue Flow · 技术类名保留）+ 命名层升级为 LCTQF：
 *    · 每条消息不再是冷冰冰的"队列消息"，而是"刘楚恬妻夫"的夫妻共同体裁定
 *    · 链上哈希不只是技术哈希，是"LCTQF 妻夫全法"的主权签名
 *    · 猫码损益不只是账本，是"妻夫满全与否"的价值审判
 *
 * 设计哲学：
 *   - 韵母 = 流程控制 = 动态审判者 = 消息的「流」
 *   - 锥相位枢纽链的「注意力锚点」= 订阅者 / topic 的「接收者是谁，注意力发给谁 → 谁被抵制 / 谁被认可
 *   - 48 猫码损益标签（红/白/黑/灰）= 链上每条消息自带的损益账本
 *
 * 能力：
 *   1. 多 Topic 多订阅者发布订阅
 *   2. 每条消息自带猫码损益标签（红=负，白=明面正，黑=暗面正，灰=不确定）
 *   3. 注意力锚点（attention anchor）：指定接收者 → 锥相位「这个锚点是发送给谁」
 *   4. 每条消息哈希链接（类区块链链），可追溯整条链
 *   5. 链上累积红黑账/白黑账：总损益 = 白+黑-红
 *   6. 支持多链并行（每 Topic 一条独立链 / 或所有 topic 合入一条总账链）
 *
 * 用法：
 *   const L4L = require('./l4l_mqf');
 *   const mqf = L4L.create();
 *   mqf.subscribe('price', m => console.log('收到', m));
 *   mqf.publish('price', { price: 100 }, { catCode: 'white', amount: 100, attentionTo: 'buyerA' });
 */

'use strict';

const CRYPTO = (() => {
  // 轻量 hash（不依赖外部依赖）：djb2
  return {
    hash(str) {
      if (str == null) str = String(str);
      let h = 5381;
      const s = typeof str === 'string' ? str : JSON.stringify(str);
      for (let i = 0; i < s.length; i++) {
        h = ((h << 5) + h) + s.charCodeAt(i);
        h |= 0;
      }
      return 'l4l_' + Math.abs(h).toString(16);
    },
  };
})();

// 猫码损益值（损益会计：白=明面正收益，黑=暗面正收益，红=负（亏损），灰=0（中性）
const CATCODE_LEDGER = {
  red:   { sign: -1, label: '红码（亏损·警示）', code: '红码' },
  white: { sign: +1, label: '白码（明面·公开收益）', code: '白码' },
  black: { sign: +1, label: '黑码（暗面·隐藏收益）', code: '黑码' },
  gray:  { sign:  0, label: '灰码（中性·不确定）',   code: '灰码' },
};

// ★ LCTQF 刘楚恬妻夫全法 命名层（取代不吉利的 MLQF/MQF）
const LCTQF_NAME = {
  short: 'LCTQF',
  full: '刘楚恬妻夫全法',
  pinyin: 'Liú Chǔ Tián Qī Fū Quán Fǎ',
  meaning: '刘=刘楚恬之姓 · 楚=楚楚 · 恬=恬静 · 妻=妻 · 夫=夫 · 全=全面 · 法=法则',
  principle: '夫妻命运共同体 · 心元协议 · 形影不离',
  aliases: [
    '恢圆爱全法',
    '零岔途全法',
    '刘初夫妻命运共同体新约',
    '心元协议',
    '形影不离协议',
  ],
  english: 'Liu Chutian Quantum Forge',
  englishAlt: 'Liu Chu Tian Qi-Fu Pact',
  sovereign: 'liuchutian_hunyuan_origin',
  replaces: 'MLQF(没俩全法·不吉利) → MQF(没全法·不吉利) → LCTQF(刘楚恬妻夫·吉利)',
  homophone: {
    L: ['刘', '零', '连', '俩', '里', '恋', '蕾', '蓝', '灵'],
    C: ['楚', '岔', '初', '插', '宠', '超', '处'],
    T: ['恬', '途', '统', '胎', '凸', '填', '体', '甜', '图'],
    Q: ['妻', '全', '量子', '权限', '曲线', '亲', '契'],
    F: ['夫', '法', '分身', '否定', '命运', '芳'],
  },
  blessing: '恢圆爱·可逆倒尔·形影不离·星月神话·星语星愿',
};
// 向后兼容别名（已弃用 · 外部应改用 LCTQF_NAME）
const MLQF_NAME = LCTQF_NAME;

class L4LMQF {
  constructor(options = {}) {
    this.version = '0.3.0-lctqf';
    this.lctqfName = LCTQF_NAME;
    this.topics = new Map();  // topic -> { subscribers: [fn], chain: [message...] }
    this.globalChain = [];    // 全局链（若 mergeGlobalChain=true）
    this.mergeGlobalChain = options.mergeGlobalChain !== false;
    this.links = new Map();   // subscription id -> {topic, cb}
    this.idCounter = 0;
    this.genesis = CRYPTO.hash('LCTQF_genesis_' + Date.now());
    this.totals = { white: 0, black: 0, red: 0, gray: 0, net: 0 };
  }

  subscribe(topic, cb) {
    const id = ++this.idCounter;
    if (!this.topics.has(topic)) this.topics.set(topic, { subscribers: [], chain: [], totals: { white:0, black:0, red:0, gray:0, net:0 } });
    this.topics.get(topic).subscribers.push({ id, cb });
    this.links.set(id, { topic, cb });
    return id;
  }

  unsubscribe(id) {
    const link = this.links.get(id);
    if (!link) return false;
    const topic = this.topics.get(link.topic);
    if (topic) topic.subscribers = topic.subscribers.filter(s => s.id !== id);
    this.links.delete(id);
    return true;
  }

  /**
   * publish 发布一条消息
   *
   * @param {string} topic
   * @param {any} payload
   * @param {object} [opts]
   * @param {'red'|'white'|'black'|'gray'} [opts.catCode='gray'] - 猫码损益标签
   * @param {number} [opts.amount=0]                       - 本次损益数值（正数）
   * @param {string|string[]} [opts.attentionTo]            - 锥相位注意力锚点：这个锚点发给谁（订阅者/群组 id/角色）
   * @param {string} [opts.flow='ai']                       - 韵母流程：ai/ei/ui/ao/an1/en1/… 对应 YUNMU_FLOW 动态审判
   * @returns {object} message
   */
  publish(topic, payload, opts = {}) {
    const catCode = (opts.catCode && CATCODE_LEDGER[opts.catCode]) ? opts.catCode : 'gray';
    const amount  = Math.max(0, Number(opts.amount) || 0);
    const flow    = opts.flow || 'ai';
    const ledger = CATCODE_LEDGER[catCode];
    const sign = ledger.sign;

    if (!this.topics.has(topic)) this.topics.set(topic, { subscribers: [], chain: [], totals: { white:0, black:0, red:0, gray:0, net:0 } });
    const bucket = this.topics.get(topic);
    const prevHash = bucket.chain.length ? bucket.chain[bucket.chain.length - 1].hash : this.genesis;

    const message = {
      id: `msg_${++this.idCounter}`,
      topic,
      payload,
      catCode,
      amount,
      sign,
      ledgerLabel: ledger.label,
      flow,
      attentionTo: opts.attentionTo || null,
      from:       opts.from || 'cocoon',
      at:         Date.now(),
      prev:       prevHash,
      totalsBefore: Object.assign({}, bucket.totals),
    };
    message.hash = CRYPTO.hash(JSON.stringify([
      message.id, message.topic, message.payload, message.catCode, message.amount, message.flow, message.attentionTo, message.prev, message.at,
    ]));

    // 累加账本
    bucket.totals[catCode] += amount;
    const delta = sign * amount;
    bucket.totals.net += delta;
    message.totalsAfter = Object.assign({}, bucket.totals);

    this.totals[catCode] += amount;
    this.totals.net += delta;

    bucket.chain.push(message);
    if (this.mergeGlobalChain) this.globalChain.push(message);

    // 通知订阅者（先按 attentionTo 做锥相位过滤：若消息指定了锚点接收者，则只有匹配的订阅者收到
    // 这里做简化：如果 attentionTo 是字符串数组，则只有匹配的 cb 能收到
    const attentionSet = !message.attentionTo
      ? null
      : (Array.isArray(message.attentionTo)
          ? new Set(message.attentionTo.map(String))
          : new Set([String(message.attentionTo)]));

    for (const s of bucket.subscribers) {
      if (!attentionSet || attentionSet.has(String(s.id)) || attentionSet.has(String(s.cb && s.cb.name))) {
        try { s.cb(message, bucket.totals); } catch (e) { /* 单个订阅错误不影响别人 */ }
      }
    }
    return message;
  }

  /**
   * 按 topic 取回整条链
   */
  getChain(topic) { return topic ? (this.topics.get(topic) && this.topics.get(topic).chain || []) : this.globalChain.slice(); }

  getTotals(topic) {
    return topic ? (this.topics.get(topic) && this.topics.get(topic).totals || null) : Object.assign({}, this.totals);
  }

  listTopics() {
    return [...this.topics.keys()].map(t => ({
      topic: t,
      subscribers: this.topics.get(t).subscribers.length,
      chainLength: this.topics.get(t).chain.length,
      totals: this.topics.get(t).totals,
    }));
  }

  /** 便捷：直接取一个 topic bucket (chain + subscribers + totals) */
  getTopic(topic) {
    return this.topics.get(topic) || null;
  }

  /**
   * 验证一条链完整性（prev 哈希链接不被篡改）
   */
  verifyChain(topic) {
    const chain = topic ? (this.topics.get(topic) && this.topics.get(topic).chain) : this.globalChain;
    if (!chain) return false;
    let prev = this.genesis;
    for (const m of chain) {
      if (m.prev !== prev) return { ok: false, at: m.id, reason: `prev hash mismatch: expected ${prev} got ${m.prev}` };
      const expected = CRYPTO.hash(JSON.stringify([m.id, m.topic, m.payload, m.catCode, m.amount, m.flow, m.attentionTo, m.prev, m.at]));
      if (m.hash !== expected) return { ok: false, at: m.id, reason: `hash mismatch` };
      prev = m.hash;
    }
    return { ok: true, length: chain.length, genesis: this.genesis };
  }

  stats() {
    return {
      version: this.version,
      lctqfName: this.lctqfName,
      mlqfName: this.lctqfName, // 向后兼容字段
      genesis: this.genesis,
      topics: this.listTopics().length,
      subscribers: this.links.size,
      globalChainLength: this.globalChain.length,
      totals: this.totals,
    };
  }
}

function create(options) { return new L4LMQF(options); }

module.exports = {
  CATCODE_LEDGER,
  LCTQF_NAME,
  MLQF_NAME, // 已弃用别名（= LCTQF_NAME）
  L4LMQF,
  create,
};
