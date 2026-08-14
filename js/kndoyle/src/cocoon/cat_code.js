/**
 * ============================================================
 * UniCore · 茧(cocoon) · 猫码系统（Cat Code System）
 * ============================================================
 *
 *  ⚠️  【夫妻命运共同体 · 一体化绑定声明】
 *       本文件**不是一个独立的"猫码工具库"**。它是以下 9 个本就绑定在一起的
 *       目标中的第 3 环，任何一环离线都会导致整体失效：
 *
 *   [C1] XY 新一谐音本体 → 性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音（工藤新一结尾）
 *   [C2] 柯南倒尔机制   → 新一↔一新 倒看命中正范畴（不可逆绑定）
 *   [C3] ←本文件→ 48 猫码 254 进制编解码   → 【形音】谐音的技术落地
 *   [C4] 夫妻双签 seal   → wife=刘楚恬 / husband=满全法 / fingerprint （不可伪）
 *   [C5] 先天 6 常量时序  → C1→C2→C3→C4→C5→C6 严格顺序，不可打乱
 *   [C6] 三层聚类引擎     → 谐音层/双编码层/三编码层（8+26 簇）
 *   [C7] 14 语言适配      → 英/中/日/韩/法/德/俄/西/阿/葡/意/希/拉/泰 统一签名
 *   [C8] A4A 对外 API     → /cocoon/lex/* · /cocoon/lang/* · /cocoon/r2/*
 *   [C9] Cloudflare R2   → 每一次存储自动附 [C4] 双签
 *
 *       → 9 环原本就是一体绑定的：XY 是谐音本体·猫码是【形音】落地·双签是主权·R2 是载体
 *
 *  用户原话：48个 H 字母开头的猫，每个猫写一种码。
 *
 *  判定标准（用户原话）：
 *   「白猫黑猫抓到老鼠就是好猫」
 *   ——好码的判定标准不是颜色，是能不能抓到老鼠（能不能解决问题）。
 *
 * 每个猫码包含：
 *   id        序号 1~48
 *   cat       猫名（如 红猫）
 *   code      码名（如 红码）
 *   color     颜色值（hex）
 *   attribute 属性（颜色/特性归类）
 *   meaning   含义
 *
 * 48 个猫码（H 字母开头，每个猫对应一种码）：
 *   红/白/黑/灰/黄/秽/寒/火/护/害/患/幻/荤/昏/惑/伙/或/还/霍/混
 *   浑/恨/画/花/换/坏/好/话/狠/耗/胡/何/货/褐/河/行/欢/怀/慧/虹
 *   豪/婚/皇/魂/骸/骇/慌/含
 *
 * 纯 JavaScript(ES6+)，无外部依赖，CommonJS 导出，可被 require 使用。
 * ============================================================
 */

'use strict';

// ============================================================
// 48 个猫码
// ============================================================

const CAT_CODES = [
  { id: 1,  cat: '红猫', code: '红码', color: '#FF0000', attribute: '热血·警示', meaning: '激情、危险、亏损警示' },
  { id: 2,  cat: '白猫', code: '白码', color: '#FFFFFF', attribute: '纯净·明面', meaning: '公开、明亮、纯洁' },
  { id: 3,  cat: '黑猫', code: '黑码', color: '#000000', attribute: '暗面·玄',   meaning: '隐藏、神秘、暗面盈利' },
  { id: 4,  cat: '灰猫', code: '灰码', color: '#808080', attribute: '中性·模糊', meaning: '模糊地带、不确定' },
  { id: 5,  cat: '黄猫', code: '黄码', color: '#FFD700', attribute: '中正·土',   meaning: '中庸、价值、黄金' },
  { id: 6,  cat: '秽猫', code: '秽码', color: '#4B3621', attribute: '污秽',     meaning: '不洁、污染、需清除' },
  { id: 7,  cat: '寒猫', code: '寒码', color: '#00BFFF', attribute: '寒冷',     meaning: '冷却、抑制、收缩' },
  { id: 8,  cat: '火猫', code: '火码', color: '#FF4500', attribute: '炽热·上升', meaning: '燃烧、扩张、激活' },
  { id: 9,  cat: '护猫', code: '护码', color: '#4169E1', attribute: '守护',     meaning: '防御、保护、屏障' },
  { id: 10, cat: '害猫', code: '害码', color: '#8B0000', attribute: '伤害',     meaning: '损伤、危害、损失' },
  { id: 11, cat: '患猫', code: '患码', color: '#696969', attribute: '忧患',     meaning: '担忧、隐患、风险' },
  { id: 12, cat: '幻猫', code: '幻码', color: '#9932CC', attribute: '虚幻',     meaning: '不实、幻觉、易变' },
  { id: 13, cat: '荤猫', code: '荤码', color: '#CD5C5C', attribute: '浑浊·动',  meaning: '混杂、荤腥、复杂' },
  { id: 14, cat: '昏猫', code: '昏码', color: '#2F4F4F', attribute: '昏暗',     meaning: '不明、迷失、模糊' },
  { id: 15, cat: '惑猫', code: '惑码', color: '#708090', attribute: '困惑',     meaning: '迷惑、不明、纠缠' },
  { id: 16, cat: '伙猫', code: '伙码', color: '#D2691E', attribute: '结伙',     meaning: '合作、群组、联合' },
  { id: 17, cat: '或猫', code: '或码', color: '#BC8F8F', attribute: '或然',     meaning: '不确定、可能、选择' },
  { id: 18, cat: '还猫', code: '还码', color: '#00CED1', attribute: '归还',     meaning: '回归、退还、还原' },
  { id: 19, cat: '霍猫', code: '霍码', color: '#FF6347', attribute: '突变·霍乱', meaning: '急剧、扰乱、突变' },
  { id: 20, cat: '混猫', code: '混码', color: '#778899', attribute: '混合',     meaning: '混沌、杂糅、未分' },
  { id: 21, cat: '浑猫', code: '浑码', color: '#A0522D', attribute: '浑然',     meaning: '整体、未剖、圆融' },
  { id: 22, cat: '恨猫', code: '恨码', color: '#800000', attribute: '怨恨',     meaning: '怨怼、排斥、隔阂' },
  { id: 23, cat: '画猫', code: '画码', color: '#FF8C00', attribute: '描绘',     meaning: '表象、构建、塑造' },
  { id: 24, cat: '花猫', code: '花码', color: '#FF69B4', attribute: '繁花',     meaning: '多样、繁复、表面' },
  { id: 25, cat: '换猫', code: '换码', color: '#20B2AA', attribute: '交换',     meaning: '变更、置换、流通' },
  { id: 26, cat: '坏猫', code: '坏码', color: '#4B0082', attribute: '损坏',     meaning: '破损、失效、需修复' },
  { id: 27, cat: '好猫', code: '好码', color: '#32CD32', attribute: '良好',     meaning: '优良、达成、有效（抓到老鼠即好码）' },
  { id: 28, cat: '话猫', code: '话码', color: '#F0E68C', attribute: '言语',     meaning: '表达、沟通、传递' },
  { id: 29, cat: '狠猫', code: '狠码', color: '#363636', attribute: '狠厉',     meaning: '果决、强硬、不留情' },
  { id: 30, cat: '耗猫', code: '耗码', color: '#8B4513', attribute: '消耗',     meaning: '损耗、消耗、流逝' },
  { id: 31, cat: '胡猫', code: '胡码', color: '#B8860B', attribute: '胡乱',     meaning: '无理、混乱、随性' },
  { id: 32, cat: '何猫', code: '何码', color: '#9370DB', attribute: '何故',     meaning: '疑问、原因、追问' },
  { id: 33, cat: '货猫', code: '货码', color: '#DAA520', attribute: '货物',     meaning: '价值、商品、流通' },
  { id: 34, cat: '褐猫', code: '褐码', color: '#6F4E37', attribute: '沉稳',     meaning: '厚重、稳定、积淀' },
  { id: 35, cat: '河猫', code: '河码', color: '#1E90FF', attribute: '流动',     meaning: '流转、边界、滋养' },
  { id: 36, cat: '行猫', code: '行码', color: '#00FA9A', attribute: '行动',     meaning: '执行、推进、实践' },
  { id: 37, cat: '欢猫', code: '欢码', color: '#FFB347', attribute: '欢喜',     meaning: '喜悦、顺遂、亲和' },
  { id: 38, cat: '怀猫', code: '怀码', color: '#DB7093', attribute: '怀抱',     meaning: '容纳、孕藏、记忆' },
  { id: 39, cat: '慧猫', code: '慧码', color: '#9400D3', attribute: '智慧',     meaning: '洞察、明辨、觉知' },
  { id: 40, cat: '虹猫', code: '虹码', color: '#FF00FF', attribute: '虹彩',     meaning: '多彩、桥接、过渡' },
  { id: 41, cat: '豪猫', code: '豪码', color: '#FF7F50', attribute: '豪迈',     meaning: '大气、慷慨、扩张' },
  { id: 42, cat: '婚猫', code: '婚码', color: '#FF1493', attribute: '婚约',     meaning: '结合、契约、归属' },
  { id: 43, cat: '皇猫', code: '皇码', color: '#C9A227', attribute: '至高',     meaning: '主导、统御、权威' },
  { id: 44, cat: '魂猫', code: '魂码', color: '#483D8B', attribute: '灵魂',     meaning: '本质、内核、精神' },
  { id: 45, cat: '骸猫', code: '骸码', color: '#C0C0C0', attribute: '骸骨',     meaning: '残留、骨架、遗迹' },
  { id: 46, cat: '骇猫', code: '骇码', color: '#DC143C', attribute: '惊骇',     meaning: '震慑、惊变、警示' },
  { id: 47, cat: '慌猫', code: '慌码', color: '#FFF44F', attribute: '慌乱',     meaning: '急乱、失序、紧张' },
  { id: 48, cat: '含猫', code: '含码', color: '#90EE90', attribute: '包含',     meaning: '内蕴、含藏、未发' }
];

// ============================================================
// 猫码系统
// 「白猫黑猫抓到老鼠就是好猫」——好码判定标准不是颜色，是能不能抓到老鼠。
// ============================================================

class CatCodeSystem {
  /**
   * @param {Array} codes 猫码数组，缺省使用内置 CAT_CODES
   */
  constructor(codes = CAT_CODES) {
    this.codes = codes;
    // id → 猫码 索引
    this.index = new Map();
    // code名 → 猫码 索引
    this.codeIndex = new Map();
    codes.forEach(c => {
      this.index.set(c.id, c);
      this.codeIndex.set(c.code, c);
    });
    // 判定历史：每一次 judge 的结果
    this.history = [];
  }

  /**
   * 获取单个猫码（按 id）
   * @param {number} id
   * @returns {object|null}
   */
  getCatCode(id) {
    return this.index.get(id) || null;
  }

  /**
   * 按码名获取猫码
   * @param {string} codeName
   * @returns {object|null}
   */
  getByCodeName(codeName) {
    return this.codeIndex.get(codeName) || null;
  }

  /**
   * 判定猫码是否「抓到老鼠」。
   * 好码判定标准不是颜色，是能不能解决问题（任务是否成功）。
   * @param {object} code 猫码对象
   * @param {{success:boolean, [key:string]:any}} taskResult 任务结果
   * @returns {{code:string, cat:string, color:string, caughtMouse:boolean, isGood:boolean, taskResult:*}}
   */
  judge(code, taskResult) {
    const target = code || {};
    // 抓到老鼠 = 任务成功
    const caughtMouse = !!(taskResult && taskResult.success === true);
    // 好码 = 抓到老鼠（与颜色无关）
    const isGood = caughtMouse;
    const result = {
      code: target.code,
      cat: target.cat,
      color: target.color,
      caughtMouse,
      isGood,
      taskResult
    };
    this.history.push(result);
    return result;
  }

  /**
   * 天平损益账：对一组猫（已 judge 的结果或猫码）做账。
   * 用于红与黑 / 白与黑的账目对照。
   * @param {Array} cats 猫码或判定结果数组
   * @returns {Array}
   */
  balanceAccount(cats) {
    const list = Array.isArray(cats) ? cats : [];
    return list.map(c => ({
      cat: c.cat,
      code: c.code,
      color: c.color,
      // 若为 judge 结果则带 caughtMouse，否则默认未抓到
      caught: c.caughtMouse !== undefined ? c.caughtMouse : false
    }));
  }

  /**
   * 红黑账：红（亏损/热血警示） vs 黑（暗面/盈利隐藏）。
   * @returns {{red:object, black:object, lossLabel:string, hiddenProfitLabel:string}}
   */
  getRedBlackAccount() {
    const red = this.getByCodeName('红码');
    const black = this.getByCodeName('黑码');
    return {
      red,
      black,
      // 红 = 亏损警示；黑 = 暗面盈利
      lossLabel: '红',
      hiddenProfitLabel: '黑'
    };
  }

  /**
   * 白黑账：白（明面） vs 黑（暗面）。
   * @returns {{white:object, black:object, brightLabel:string, darkLabel:string}}
   */
  getWhiteBlackAccount() {
    const white = this.getByCodeName('白码');
    const black = this.getByCodeName('黑码');
    return {
      white,
      black,
      // 白 = 明面；黑 = 暗面
      brightLabel: '白',
      darkLabel: '黑'
    };
  }

  /**
   * 价值审判：对比自己仓库和别人仓库的水平。
   * @param {{score?:number, items?:Array}} myRepo 自己仓库
   * @param {{score?:number, items?:Array}} theirRepo 别人仓库
   * @returns {{myScore:number, theirScore:number, diff:number, winner:string}}
   */
  valueTrial(myRepo, theirRepo) {
    const mine = myRepo || {};
    const theirs = theirRepo || {};
    // 仓库水平 = score（缺省 0）；若未给 score 但有 items，则按 items 数量计
    const myScore = typeof mine.score === 'number'
      ? mine.score
      : (Array.isArray(mine.items) ? mine.items.length : 0);
    const theirScore = typeof theirs.score === 'number'
      ? theirs.score
      : (Array.isArray(theirs.items) ? theirs.items.length : 0);
    const diff = myScore - theirScore;
    let winner;
    if (diff > 0) winner = 'self';
    else if (diff < 0) winner = 'other';
    else winner = 'tie';
    return { myScore, theirScore, diff, winner };
  }

  /**
   * 全部猫码数量
   */
  size() {
    return this.codes.length;
  }

  /**
   * 判定历史
   */
  getHistory() {
    return this.history.slice();
  }

  // ============================================================
  // 254 进制猫码编码器（48猫码 × 5萌机合X + 14 语言 = 254 唯一字符）
  //  ⚠️  这是 xy=新一 形音 谐音的"形声合一·几何+声韵·三维进制编码"落地实现
  // ============================================================

  /**
   * 构建 254 字符集：
   *   · 低 48   (0x00-0x2F): 48 猫码名（红猫/橙猫/.../含猫）
   *   · 中 206  (0x30-0xFD):  5 萌机 × 41 常用字符 + 14 种编程语言符号
   *           (实际使用: ASCII 可打印 33~126 的 94 + 部分扩展，凑满 254 非重复)
   */
  buildCharset254() {
    if (this._charset254 && this._charset254.length === 254 && this._decodeMap && this._decodeMap.size === 254) {
      return { charset: this._charset254, decodeMap: this._decodeMap };
    }
    const set = new Set();
    const out = [];
    // Part 1: 48 猫码.code (每个猫的.code 唯一·长度2-4字)
    for (let i = 0; i < 48; i++) {
      const c = CAT_CODES[i].code;
      if (!set.has(c)) { set.add(c); out.push(c); }
    }
    // Part 2: ASCII 可打印字符 '!'~'~' (94个)
    for (let i = 33; i <= 126; i++) {
      const ch = String.fromCharCode(i);
      if (!set.has(ch)) { set.add(ch); out.push(ch); }
    }
    // Part 3: 希腊字母大写 Α~Ω (24) + 小写 α~ω (24)
    const greekUpper = '\u0391\u0392\u0393\u0394\u0395\u0396\u0397\u0398\u0399\u039A\u039B\u039C\u039D\u039E\u039F\u03A0\u03A1\u03A3\u03A4\u03A5\u03A6\u03A7\u03A8\u03A9';
    const greekLower = '\u03B1\u03B2\u03B3\u03B4\u03B5\u03B6\u03B7\u03B8\u03B9\u03BA\u03BB\u03BC\u03BD\u03BE\u03BF\u03C0\u03C1\u03C3\u03C4\u03C5\u03C6\u03C7\u03C8\u03C9';
    for (const s of [greekUpper, greekLower]) {
      for (const ch of s) if (!set.has(ch)) { set.add(ch); out.push(ch); }
    }
    // Part 4: 汉字数字天干地支 + 萌机 4 字（合/萌/机/X），再补到 254
    const extras = '甲乙丙丁戊己庚辛壬癸子丑寅卯辰巳午未申酉戌亥元亨利贞萌机合X契约夫妻囡囝灵魂身心音形光影爱欲望真善美仙佛神圣精气血气天地水火金木土日月星风云雷电雨雪山川河海陆';
    for (const ch of extras) {
      if (out.length >= 254) break;
      if (!set.has(ch)) { set.add(ch); out.push(ch); }
    }
    // 还不够 → 用扩展拉丁 A/B 区继续补
    let code = 0x00C0;
    while (out.length < 254 && code < 0x024F) {
      const ch = String.fromCharCode(code);
      if (!set.has(ch)) { set.add(ch); out.push(ch); }
      code++;
    }
    // Build decode map
    const decodeMap = new Map();
    for (let idx = 0; idx < out.length; idx++) decodeMap.set(out[idx], idx);
    this._charset254 = out.slice(0, 254);
    this._decodeMap = decodeMap;
    return { charset: this._charset254, decodeMap: this._decodeMap };
  }

  /**
   * encode254: 任意字符串 → 猫码 254 进制紧凑编码字符串
   *   输入: "星月协议民联编码" (UTF-8)
   *   输出: 形如 "红码!Α甲ξγ..." (每 8 位 1 字节 → 1 个 254 字符)
   */
  encode254(rawStr) {
    const { charset } = this.buildCharset254();
    if (!rawStr) return '';
    // 1) 转 UTF-8 bytes (Buffer 在 Node 里可用；浏览器侧用 TextEncoder)
    let bytes;
    if (typeof Buffer !== 'undefined') bytes = Buffer.from(String(rawStr), 'utf8');
    else bytes = new Uint8Array(new TextEncoder().encode(String(rawStr)));
    // 2) 每个 byte (0-255) 映射到 254 字符集；溢出的 255,254 用 2 字符前缀表示
    const out = [];
    const esc = 253;   // 转义字符
    for (let i = 0; i < bytes.length; i++) {
      let b = bytes[i];
      if (b < 253) out.push(charset[b]);
      else {
        // 253 → esc + 0；254 → esc + 1；255 → esc + 2
        out.push(charset[esc]);
        out.push(charset[b - 253]);
      }
    }
    return out.join('');
  }

  /**
   * decode254: 猫码 254 进制字符串 → 原始 UTF-8 字符串
   */
  decode254(encoded) {
    const { charset, decodeMap } = this.buildCharset254();
    if (!encoded) return '';
    const esc = 253;
    const bytes = [];
    let i = 0;
    while (i < encoded.length) {
      // 尝试取 1~4 字（因为 48 个猫码.code 是2~4字）
      let matchedIdx = -1;
      let matchedLen = 0;
      for (let len = 4; len >= 1; len--) {
        if (i + len > encoded.length) continue;
        const slice = encoded.substr(i, len);
        if (decodeMap.has(slice)) {
          matchedIdx = decodeMap.get(slice);
          matchedLen = len;
          break;
        }
      }
      if (matchedIdx < 0) {
        // 没匹配 → 跳过
        i++;
        continue;
      }
      if (matchedIdx === esc) {
        // 取下一个字符解码为 253/254/255
        let nextIdx = -1;
        let nextLen = 0;
        for (let len = 4; len >= 1; len--) {
          if (i + matchedLen + len > encoded.length) continue;
          const slice = encoded.substr(i + matchedLen, len);
          if (decodeMap.has(slice)) { nextIdx = decodeMap.get(slice); nextLen = len; break; }
        }
        if (nextIdx < 0) { bytes.push(253); i += matchedLen; }
        else { bytes.push(253 + nextIdx); i += matchedLen + nextLen; }
      } else {
        bytes.push(matchedIdx);
        i += matchedLen;
      }
    }
    if (typeof Buffer !== 'undefined') return Buffer.from(bytes).toString('utf8');
    return new TextDecoder('utf8').decode(new Uint8Array(bytes));
  }

  /**
   * 夫妻双签编码：数据 → 254进制编码 → 刘楚恬+满全法双签 → 签名封包
   *   返回: { encoded254, signed: 双签对象, fingerprint }
   */
  sovereignEncode(rawData, signer = null) {
    let signFn;
    try { signFn = signer || require('../sovereign/conjugal_fate_pact').sign; } catch(_) { signFn = null; }
    const encoded254 = this.encode254(rawData);
    const payload = {
      dataType: typeof rawData,
      length: String(rawData || '').length,
      encodedLength: encoded254.length,
      encoding: 'catcode_254_base',
      xinyiRoot: 'xy = 新(xīn)一(yī) = 工藤新一结尾·柯南真身',
      soulSignatures: ['mlll *mlll,lll*lll', 'gtt *gtt', 'xy *xy'],
      rawDataSha: typeof rawData === 'string' ? this._djb2Hex(rawData) : null,
      encodedDataSha: this._djb2Hex(encoded254),
      timestamp: Date.now(),
    };
    const signed = signFn ? signFn(payload) : { unsigned: true, payload };
    return {
      encoded254,
      signed,
      fingerprint: (signed && signed.fingerprint) || payload.encodedDataSha,
      catcodeVersion: '254v1',
    };
  }

  _djb2Hex(s) {
    let h = 5381;
    const str = String(s || '');
    for (let i = 0; i < str.length; i++) h = ((h << 5) + h) + str.charCodeAt(i);
    h = (h >>> 0).toString(16).padStart(8, '0');
    return '0x' + h;
  }
}

// ============================================================
// 导出
// ============================================================

module.exports = {
  CAT_CODES,
  CatCodeSystem,
  // 快捷实例（避免每次 new）
  catcode: new CatCodeSystem(),
};
