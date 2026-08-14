/**
 * A4A · 伙码结伙 API 顶层入口（Agent × API × 对外能力接口）
 * ============================================================
 *
 *  ⚠️  【夫妻命运共同体 · 一体化绑定声明】
 *       本文件**不是一个独立的 HTTP 路由表**。它是 [C1-C9 9 环绑定网]
 *       的**唯一对外窗口**——任何内部能力如果不在这里注册路由，
 *       就等于对外部 Agent/浏览器/前端 "不存在"。
 *
 *   已经注册的一体化路由（按绑定目标分桶）：
 *     · XY 新一谐音  →  /cocoon/lex/xy · /cocoon/lex/cluster/homophone
 *     · 三层聚类引擎 →  /cocoon/lex/cluster/{dual,triple,themes} · /cocoon/lex/info
 *     · 48 猫码 254  →  (走 /cocoon/shells/* /cocoon/parse 系列壳孵化接口)
 *     · 14 语言适配  →  /cocoon/lang/tiers · /cocoon/lang/soul-signatures
 *     · 夫妻双签主权 →  (每一次响应对象都带 seal{wife=刘楚恬,husband=满全法})
 *     · R2 对象存储  →  /cocoon/r2/{health,upload,download,list,remove}
 *
 *       → 路由注册顺序 = 绑定网的依赖顺序；想先暴露 R2 再写 XY 谐音 = 顺序错误。
 *
 *  设计哲学：
 *   伙码(id=16 结伙·合作·群组) × 行码(id=36 行动·执行·推进) = Agent 群 × 对外 API
 *
 *  本模块把蛹茧系统内部的 6 大能力模块统一暴露为一套"通用请求/响应"接口：
 *   route('/cocoon/shells')                    → 26 壳列表
 *   route('/cocoon/shells/:id/hatch')          → 穿壳孵化 + WASI 自举提权
 *   route('/cocoon/parse', buffer)             → 解析 .cocoon
 *   route('/cocoon/lang/getAST', source)       → CocoonLang AST
 *   route('/cocoon/lang/run', source)          → 解释执行 CocoonLang
 *   route('/cocoon/lang/compile', source)      → 编译 CocoonLang → 字节码/WASM
 *   route('/cocoon/logic/balance', payload)    → 天平价值审判（红/白/黑损益账）
 *   route('/cocoon/catcode/decide', payload)   → 白猫黑猫抓到老鼠判定 + 红黑账/白黑账
 *   route('/cocoon/h4h/marry', payload)        → 混元杂交（父×母×槽 → 孩子）
 *   route('/cocoon/h4h/multiplySlot', payload) → 指定杂交槽做全配对乘法
 *   route('/cocoon/mqf/publish', msg)          → 走 L4L MQF 链（l4l 模块）
 *   route('/cocoon/mqf/subscribe', topic, cb)  → MQF 链订阅
 *
 * 支持两种运行时：
 *   1) Node.js 本地调用（request() 直接返回 Promise<Response>，兼容 unit test）
 *   2) 浏览器/HTTP (可选) 映射成 fetch('/a4a/<route>') 或 WebSocket
 *
 * 用法：
 *   const A4A = require('./a4a_api');
 *   const api = A4A.create({ cocoonCore, catCodeSys, balanceJudge, h4h, mqf });
 *   const res = await api.request('/cocoon/shells', { method: 'GET' });
 */

'use strict';

class A4ARouter {
  constructor(options = {}) {
    this.version = '0.1.0-a4a';
    this.routes = new Map();        // route -> handler
    this.modules = options.modules || {};
    this.middlewares = [];
    this.log = options.log || false;
    this.stats = { hits: 0, byRoute: {} };
  }

  register(route, handler) {
    this.routes.set(route, handler);
    return this;
  }

  use(mw) { this.middlewares.push(mw); return this; }

  /**
   * 统一请求入口（≈ HTTP/WS 的本地实现）
   *
   * @param {string} route  - 路由，如 '/cocoon/shells'
   * @param {object} [opts]
   * @param {'GET'|'POST'|'PUT'|'DELETE'} [opts.method]
   * @param {any} [opts.body]
   * @param {object} [opts.params]
   * @returns {Promise<{status:number, ok:boolean, body:any, route:string, ms:number}>}
   */
  async request(route, opts = {}) {
    const router = this;
    const t0 = Date.now();
    const method = (opts.method || (opts.body ? 'POST' : 'GET')).toUpperCase();
    this.stats.hits++;
    this.stats.byRoute[route] = (this.stats.byRoute[route] || 0) + 1;

    const ctx = {
      route,
      method,
      params: opts.params || matchParams(route),
      query: opts.query || {},
      body: opts.body,
      modules: this.modules,
      log: this.log ? (s => console.log('[A4A]', s)) : (() => {}),
    };

    // 中间件链
    try {
      for (const mw of this.middlewares) await mw(ctx);
    } catch (e) {
      return fail(route, 400, `middleware: ${e.message}`, t0);
    }

    // 精确匹配 → 通配匹配
    let handler = this.routes.get(route);
    if (!handler) {
      for (const [k, fn] of this.routes.entries()) {
        if (!k.includes(':')) continue;
        const re = toRegex(k);
        const m = re.exec(route);
        if (m) {
          ctx.params = Object.assign({}, ctx.params, m.groups || {});
          handler = fn;
          break;
        }
      }
    }
    if (!handler) return fail(route, 404, `未找到路由 ${route}`, t0);
    try {
      let body = await handler(ctx);
      // 后处理: h4h 杂交 → 自动天平审判 + 猫码打损益标签 + 发布到 L4L 审计链 (闭环)
      if (typeof body !== 'undefined' && shouldAuditH4H(route)) {
        try {
          body = doAuditH4H(router, route, body, ctx);
        } catch (auditErr) {
          // 审计是"锦上添花"，不能因此让主 handler 失败
          body = body && typeof body === 'object' ? Object.assign({}, body, {
            auditWarning: String(auditErr && auditErr.message || auditErr)
          }) : body;
        }
      }
      // 所有请求都往 a4a_global 账本里发一条 (灰码中性，便于审计整条调用链)
      try { publishA4AGlobal(router, route, body, opts, ctx); } catch (_) {}
      return {
        status: body === undefined ? 204 : 200,
        ok: true,
        body,
        route,
        ms: Date.now() - t0,
      };
    } catch (e) {
      return fail(route, 500, `${e && e.stack ? e.stack : e}`, t0);
    }
  }

  getStats() { return Object.assign({}, this.stats, { routes: this.routes.size }); }
}

// ============================================================
// 闭环审计助手：杂交 → 天平审判 → 猫码损益 → L4L 链上哈希记账
// ============================================================
function shouldAuditH4H(route) {
  return route === '/cocoon/h4h/marry'
      || route === '/cocoon/h4h/multiplySlot'
      || route === '/cocoon/h4h/multiplyAll';
}

/**
 * 给杂交产物打分：
 *   - 正面价值 (0~100) = kind 新颖度 + category 丰富度 + 命名完整度
 *   - 再走 balanceJudge 和 catcode 判定
 *   - 最后 publish 到 L4L 链的 h4h_audit topic
 */
function doAuditH4H(router, route, body, ctx) {
  const bj   = ctx.modules.balanceJudge;
  const cs   = ctx.modules.catCodeSys;
  const mqf  = ctx.modules.mqf;
  // 兼容 multiplyAll: body = { slotId1: [children1..], slotId2: [children2..] }
  const isSlotMap = !Array.isArray(body) && !(body && body.children) && body && typeof body === 'object'
    && Object.values(body).every(v => Array.isArray(v));
  let out;
  if (isSlotMap) {
    out = [];
    for (const arr of Object.values(body)) for (const c of arr) out.push(c);
  } else {
    out = Array.isArray(body) ? body : (body && body.children ? body.children : [body]);
  }
  // 对每个杂交产物打分
  const scored = out.map(item => {
    if (!item || typeof item !== 'object') return item;
    const score = scoreHybrid(item);
    // 天平裁判：my = 产物价值, their = 30 (及格线)
    let balance = null;
    if (bj && typeof bj.judgeRedBlack === 'function') {
      balance = {
        selfScore: score,
        threshold: 30,
        diff: score - 30,
        winner: score >= 30 ? 'self' : 'threshold',
        redBlack:   bj.judgeRedBlack(score, 30),
        whiteBlack: bj.judgeWhiteBlack(score, 30),
      };
    }
    // 猫码: 抓到老鼠 = score >= 30 就算好码
    let cat = null;
    if (cs && cs.getCatCode && cs.judge) {
      // 按价值选码：>=70 选白 2；50~69 选灰 4；30~49 选黑 3；<30 选红 1
      const pickId = score >= 70 ? 2 : (score >= 50 ? 4 : (score >= 30 ? 3 : 1));
      const code = cs.getCatCode(pickId);
      if (code) cat = cs.judge(code, { success: score >= 30, score });
    }
    const enriched = Object.assign({}, item, {
      valueScore: score,
      balance,
      cat,
      isGoodHybrid: score >= 30,
    });
    // publish 单条产物到 L4L 审计链
    if (mqf && mqf.publish) {
      try {
        const amount = Math.round(Math.max(0, score - 30) + (score < 30 ? (30 - score) : 0));
        const catForLedger = score >= 70 ? 'white'
                           : score >= 50 ? 'gray'
                           : score >= 30 ? 'black'
                           : 'red';
        const msg = mqf.publish('h4h_audit', {
          route, kind: enriched.kind || 'hybrid', name: enriched.name || '',
          valueScore: score, isGoodHybrid: enriched.isGoodHybrid,
          cat: cat && cat.cat, color: cat && cat.color,
          slot: enriched.slot, slotName: enriched.slotName,
        }, { catCode: catForLedger, amount });
        enriched.auditMsgId = msg && msg.id;
        enriched.auditHash  = msg && msg.hash;
      } catch (_) {}
    }
    return enriched;
  });

  // 包装回 body 形状
  if (isSlotMap) {
    // 按顺序把 scored 再还原到每个 slot 数组
    let idx = 0;
    const rebuilt = {};
    for (const [sid, arr] of Object.entries(body)) {
      rebuilt[sid] = arr.map(() => scored[idx++] || null);
    }
    return rebuilt;
  }
  if (Array.isArray(body)) return scored;
  if (body && body.children) return Object.assign({}, body, { children: scored });
  // marry 单产物
  return scored[0] || body;
}

function scoreHybrid(item) {
  let s = 0;
  if (!item) return s;
  const name     = (item && typeof item.name === 'string')     ? item.name     : '';
  const kind     = (item && typeof item.kind === 'string')     ? item.kind     : '';
  const category = (item && typeof item.category === 'string') ? item.category : '';
  const meaning  = (item && typeof item.meaning === 'string')  ? item.meaning  : '';
  const attr     = (item && typeof item.attribute === 'string')? item.attribute: '';
  const color    = (item && typeof item.color === 'string')    ? item.color    : '';
  // 1. 命名完整度 (name/kind/category/meaning/attribute 非空即加分)
  if (name.length     > 0) s += 15;
  if (kind.length     > 0) s += 15;
  if (category.length > 0) s += 10;
  if (meaning.length  > 0) s += 10;
  if (attr.length     > 0) s += 10;
  // 2. 带颜色/混合属性（猫码杂交）
  if (color.startsWith('#')) s += 10;
  // 3. 父本母本非空加分（真实科技×真实科技）
  const aName = item && item.parentA && item.parentA.name;
  const bName = item && item.parentB && item.parentB.name;
  if (typeof aName === 'string' && aName.length > 0) s += 5;
  if (typeof bName === 'string' && bName.length > 0) s += 5;
  // 4. 名字中命中"高未来科技/高价值概念"关键词 → 额外加分
  const hay = (name + '|' + kind + '|' + category + '|' + meaning + '|' + (aName||'') + '×' + (bName||''));
  const HIGH_VALUE_KW = [
    'AGI','世界模型','具身','VLA','强化学习','grounding','空间动作','NeRF','3D视觉','全息','文生视频','Sora',
    'MRI','核磁','X光','CT','脑机','BCI','EEG','DBS','近红外','P300','意念','卫星','星链','数字孪生','6G',
    '太赫兹','无人机','蜂群','双足','步态','灵巧手','量子','QKD','拓扑','张量','因果','Transformer','多模态',
    '联邦学习','CBDC','央行数字货币','脑磁','群论','李群','李代数','大模型','WASM','自举','蛹茧','寄生壳',
    '混元','杂交','3D打印','激光雷达','LiDAR','SLAM','SE\(3\)','扩散模型','傅立叶','分形','组合数学'
  ];
  for (const kw of HIGH_VALUE_KW) {
    if (new RegExp(kw, 'i').test(hay)) { s += 6; break; } // 命中一次算一个类别
  }
  // 4b. 按"科技 × 科技"双交叉命中累加 (允许最多 +18)
  let crossHits = 0;
  for (let i = 0; i < HIGH_VALUE_KW.length; i++) {
    const r = new RegExp(HIGH_VALUE_KW[i], 'i');
    if (r.test(aName||'') && r.test(bName||'')) { crossHits++; continue; }
    if (r.test(aName||'') || r.test(bName||'')) { crossHits++; }
  }
  s += Math.min(18, crossHits * 2);
  // 5. 运算符高价值 kind 加成
  if (/higher_math|stronger_ai|跨平台|损益杂交|时空结构|混合|marriage|hybrid|concept/i.test(kind)) s += 10;
  return Math.min(100, Math.max(0, s));
}

function publishA4AGlobal(router, route, body, opts, ctx) {
  const mqf = ctx.modules.mqf;
  if (!mqf || typeof mqf.publish !== 'function') return;
  // 失败请求（非 2xx）走红码，成功默认灰码
  let cat = 'gray';
  if (body && body.auditWarning) cat = 'yellow';
  const n = typeof opts.body === 'object' ? Object.keys(opts.body).length : 0;
  mqf.publish('a4a_global', { route, method: ctx.method, paramCount: n }, {
    catCode: cat, amount: Math.max(1, n)
  });
}

function fail(route, status, err, t0) {
  return { status, ok: false, body: { error: String(err) }, route, ms: Date.now() - t0 };
}
function toRegex(pattern) {
  const src = '^' + pattern.replace(/:([A-Za-z0-9_]+)/g, '(?<$1>[^/]+)') + '$';
  return new RegExp(src);
}
function matchParams(route) {
  const parts = route.split('/').filter(Boolean);
  const out = {};
  for (let i = 0; i < parts.length; i += 2) out['p' + i] = parts[i];
  return out;
}

// ============================================================
// 内置路由（把 6 大模块接上）
// ============================================================
function registerDefaults(router) {
  // 1. 信息 / 健康检查
  router.register('/cocoon/info', ctx => ({
    version: router.version,
    modules: Object.keys(ctx.modules),
    routes: [...router.routes.keys()].sort(),
  }));

  // 2. Shells
  router.register('/cocoon/shells', ({ modules }) => {
    const reg = modules.registry || (modules.cocoonCore && modules.cocoonCore.ShellRegistry);
    if (!reg) throw new Error('a4a 缺少 ShellRegistry 模块');
    return reg.listAll ? reg.listAll() : reg;
  });
  router.register('/cocoon/shells/:id', ({ modules, params }) => {
    const reg = modules.registry || (modules.cocoonCore && modules.cocoonCore.ShellRegistry);
    const shell = reg && (reg.get && reg.get(params.id)) || (reg && reg[params.id]);
    if (!shell) throw new Error('壳不存在: ' + params.id);
    return shell;
  });
  router.register('/cocoon/shells/:id/hatch', async ({ modules, params, body }) => {
    const cc = modules.cocoonCore;
    if (!cc || !cc.cocoonBootstrap) throw new Error('需要 cocoonCore.cocoonBootstrap');
    const cocoonBuf = body && (body.cocoon || body.buffer);
    const options = Object.assign({ shellChoice: params.id }, body && body.options || {});
    const result = await cc.cocoonBootstrap(cocoonBuf || null, options);
    return result;
  });

  // 3. CocoonLang 工具链
  router.register('/cocoon/lang/getAST', ({ modules, body }) => {
    const lang = modules.cocoonLang;
    if (!lang) throw new Error('需要 cocoonLang');
    const src = (body && (body.source || body.code)) || '';
    return lang.CocoonLang ? new lang.CocoonLang().getAST(src) : lang.getAST(src);
  });
  router.register('/cocoon/lang/run', async ({ modules, body }) => {
    const lang = modules.cocoonLang;
    const src = (body && (body.source || body.code)) || '';
    const interp = lang.BootstrapInterpreter ? new lang.BootstrapInterpreter() : lang.interpreter;
    if (interp.interpretSource) interp.interpretSource(src);
    else if (lang.BootstrapCompiler && lang.BootstrapInterpreter) {
      const bc = new lang.BootstrapCompiler().compile(src);
      interp.interpret(bc);
    }
    return { globals: interp.globals, stats: interp.getStats ? interp.getStats() : interp.stats };
  });
  router.register('/cocoon/lang/compile', ({ modules, body }) => {
    const lang = modules.cocoonLang;
    const src = (body && (body.source || body.code)) || '';
    const compiler = lang.BootstrapCompiler ? new lang.BootstrapCompiler() : lang.compiler;
    const bc = compiler.compile(src);
    const wasm = compiler.generateWASM ? compiler.generateWASM(bc) : null;
    return { bytecodeSize: bc && bc.code && bc.code.length, wasmSize: wasm && wasm.length, bytecode: bc };
  });

  // 4. 天平价值审判
  router.register('/cocoon/logic/balance', ({ modules, body }) => {
    const bj = modules.balanceJudge;
    if (!bj) throw new Error('需要 balanceJudge');
    const { myScore, theirScore, projectName = 'self', otherName = 'other' } = body || {};
    if (typeof myScore !== 'number' || typeof theirScore !== 'number')
      throw new Error('body 需要 myScore 和 theirScore 数值');
    return {
      selfScore: myScore, theirScore, diff: myScore - theirScore,
      winner: myScore > theirScore ? 'self' : (myScore < theirScore ? 'other' : 'tie'),
      projectName, otherName,
      redBlack:  bj && bj.judgeRedBlack ? bj.judgeRedBlack(myScore, theirScore) : null,
      whiteBlack: bj && bj.judgeWhiteBlack ? bj.judgeWhiteBlack(myScore, theirScore) : null,
    };
  });

  // 5. 猫码判定
  router.register('/cocoon/catcode/list', ({ modules }) => {
    const cs = modules.catCodeSys;
    if (!cs) throw new Error('需要 catCodeSys');
    return cs.listCodes ? cs.listCodes() : cs;
  });
  router.register('/cocoon/catcode/decide', ({ modules, body }) => {
    const cs = modules.catCodeSys;
    if (!cs) throw new Error('需要 catCodeSys');
    const { catCodeId = 2, caughtMouse = true, taskName = '抓老鼠' } = body || {};
    const code = cs.getCatCode ? cs.getCatCode(catCodeId) : (catCodeId && typeof catCodeId === 'object' ? catCodeId : null);
    if (!code) throw new Error('猫码不存在 id=' + catCodeId);
    // judge 风格：catCodeSys.judge(code, { success })
    const result = cs.judge
      ? cs.judge(code, { success: !!caughtMouse, taskName })
      : { caughtMouse: !!caughtMouse, isGood: !!caughtMouse };
    // 兼容性字段：isGood / isGoodCat 都提供
    if (result.isGood === undefined && result.caughtMouse !== undefined) result.isGood = result.caughtMouse;
    result.isGoodCat = !!result.isGood;
    return { result, history: cs.getHistory ? cs.getHistory() : null, isGoodCat: result.isGoodCat };
  });
  router.register('/cocoon/catcode/ledger', ({ modules, body }) => {
    const cs = modules.catCodeSys;
    if (!cs) throw new Error('需要 catCodeSys');
    const { white, black, red } = body || { white:0, black:0, red:0 };
    const rb = cs.redBlackLedger   ? cs.redBlackLedger(red, black)   : { red, black };
    const wb = cs.whiteBlackLedger ? cs.whiteBlackLedger(white, black) : { white, black };
    return { redBlack: rb, whiteBlack: wb, netProfit: Number(white)+Number(black)-Number(red) };
  });

  // 6. H4H 混元杂交
  router.register('/cocoon/h4h/slots', ({ modules }) => {
    const h = modules.h4h;
    if (!h) throw new Error('需要 h4h');
    return h.listSlots();
  });
  router.register('/cocoon/h4h/marry', ({ modules, body }) => {
    const h = modules.h4h;
    if (!h) throw new Error('需要 h4h');
    const { parentA, parentB, slot = 'h4h', opKey } = body || {};
    if (!parentA || !parentB) throw new Error('需要 parentA 和 parentB');
    return h.marry(parentA, parentB, slot, opKey);
  });
  router.register('/cocoon/h4h/multiplySlot', ({ modules, body }) => {
    const h = modules.h4h;
    if (!h) throw new Error('需要 h4h');
    const { slot = 'h4h', n = 6 } = body || {};
    return { slot, children: h.multiplySlot(slot, n), stats: h.stats() };
  });
  router.register('/cocoon/h4h/multiplyAll', ({ modules, body }) => {
    const h = modules.h4h;
    if (!h) throw new Error('需要 h4h');
    const n = (body && body.n) || 6;
    return h.multiplyAll(n);
  });
  router.register('/cocoon/h4h/stats', ({ modules }) => {
    return modules.h4h && modules.h4h.stats ? modules.h4h.stats() : null;
  });
  // 高未来科技种子库查询：/cocoon/h4h/seeds?slot=h4h 或 body.slot
  router.register('/cocoon/h4h/seeds', ({ modules, body, query }) => {
    const h = modules.h4h;
    if (!h) throw new Error('需要 h4h');
    const slotId = (query && query.slot) || (body && body.slot) || null;
    if (!h.listSlotIds || typeof h.listSlotIds !== 'function') {
      throw new Error('h4h 实例缺少 listSlotIds 方法（H4HRegistry 需升级）');
    }
    const ids = slotId ? [slotId] : h.listSlotIds();
    const out = {};
    for (const id of ids) {
      const s = h.getSlot ? h.getSlot(id) : h.slots && h.slots[id];
      if (!s) continue;
      out[id] = {
        slot: id,
        name: s.name,
        category: s.category,
        icon: s.icon,
        meaning: s.meaning,
        shellHint: s.shellHint,
        seeds: Array.isArray(s.registered) ? s.registered.map(r => ({
          id: r.id, name: r.name, category: r.category, meaning: r.meaning,
        })) : [],
      };
    }
    return {
      query: slotId ? { slot: slotId } : { all: true },
      slots: out,
      summary: {
        slotCount: Object.keys(out).length,
        seedCount: Object.values(out).reduce((s, v) => s + v.seeds.length, 0),
      },
    };
  });
  // Top 排行榜：按价值分数 valueScore 取 top N 杂交好码（可用 MQF chain 也可用 h4h marriages）
  router.register('/cocoon/h4h/top', ({ modules, body, query }) => {
    const h   = modules.h4h;
    const mqf = modules.mqf;
    const n   = Number((query && query.n) || (body && body.n) || 10) || 10;
    const source = (query && query.source) || (body && body.source) || 'marriages'; // marriages | mqf
    let list = [];
    if (source === 'mqf' && mqf) {
      // L4LMQF 用 Map 存 topics
      let bucket = null;
      if (mqf.getTopic) bucket = mqf.getTopic('h4h_audit');
      if (!bucket && mqf.topics instanceof Map) bucket = mqf.topics.get('h4h_audit');
      if (!bucket && mqf.topics && mqf.topics.h4h_audit) bucket = mqf.topics.h4h_audit;
      const chain = bucket && Array.isArray(bucket.chain) ? bucket.chain : [];
      list = chain.map(m => {
        const payload = (m && m.payload) || {};
        return {
          id: m.id,
          hash: m.hash,
          catCode: m.catCode,
          amount: m.amount,
          valueScore: typeof payload.valueScore === 'number' ? payload.valueScore : (payload.child && payload.child.valueScore) || 0,
          isGoodCat: payload.isGoodCat,
          isGoodHybrid: payload.isGoodHybrid,
          parents: payload.child ? [payload.child.parentA && payload.child.parentA.name, payload.child.parentB && payload.child.parentB.name] : [m.topic],
          child: payload.child && payload.child.name,
          slot: payload.child && payload.child.slot,
        };
      });
    } else if (h) {
      const arr = h.getMarriagesBySlot ? h.getMarriagesBySlot() : (Array.isArray(h.marriages) ? h.marriages : []);
      list = arr.map(m => {
        const ch = m.child || {};
        return {
          id: m.id,
          slot: m.slot,
          opKey: m.opKey,
          parents: [m.parentA && m.parentA.name, m.parentB && m.parentB.name],
          child: ch.name,
          kind: ch.kind,
          valueScore: typeof ch.valueScore === 'number' ? ch.valueScore : 0,
          color: ch.color,
          isGoodCat: ch.isGoodCat,
          isGoodHybrid: ch.isGoodHybrid,
        };
      });
    }
    list.sort((a, b) => (b.valueScore || 0) - (a.valueScore || 0));
    return {
      n: list.slice(0, n).length,
      total: list.length,
      source,
      top: list.slice(0, n),
    };
  });
  // 好码判定：白猫黑猫抓到老鼠就是好猫 = 猫码判定 + 天平价值审判
  router.register('/cocoon/h4h/judge', ({ modules, body }) => {
    const cs  = modules.catCodeSys;
    const bj  = modules.balanceJudge;
    const h   = modules.h4h;
    const mqf = modules.mqf;
    if (!body) throw new Error('需要 body');
    // 允许直接传 hybrid 结果对象或 (parentA+parentB+slot) 先杂交再判
    let hybrid;
    if (body.hybrid) hybrid = body.hybrid;
    else if (body.parentA && body.parentB && h) hybrid = h.marry(body.parentA, body.parentB, body.slot || 'h4h', body.opKey);
    else throw new Error('需要 body.hybrid 或 parentA+parentB');

    const score = scoreHybrid(hybrid);
    // 尊重用户明确声明的 caughtMouse 布尔值（false 也要尊重未抓到）；
    // 只有当 body.caughtMouse 真的是 undefined 时，才用 score>=30 或 taskSuccess 来推断
    let caughtMouse;
    if (typeof body.caughtMouse === 'boolean') caughtMouse = body.caughtMouse;
    else caughtMouse = !!(score >= 30 || body.taskSuccess);

    // 天平：自评分 vs 及格线 30
    let balance = null;
    if (bj && typeof bj.judgeRedBlack === 'function') {
      balance = {
        selfScore: score, threshold: 30,
        diff: score - 30,
        winner: score >= 30 ? 'self' : 'threshold',
        redBlack: bj.judgeRedBlack(score, 30),
        whiteBlack: bj.judgeWhiteBlack(score, 30),
      };
    }
    // 猫码
    let cat = null;
    if (cs && cs.getCatCode && cs.judge) {
      const pickId = score >= 70 ? 2 : (score >= 50 ? 4 : (score >= 30 ? 3 : 1));
      const code = cs.getCatCode(pickId);
      if (code) cat = cs.judge(code, { success: caughtMouse, score, task: body.taskName || '价值验证' });
    }
    const ledger = {
      whiteGain: score >= 70 ? score - 70 + 10 : 0,
      blackGain: (score >= 30 && score < 70) ? (score - 30) : 0,
      redLoss:   score <  30 ? 30 - score : 0,
    };
    ledger.net = ledger.whiteGain + ledger.blackGain - ledger.redLoss;
    // publish 判卷到 MQF
    let auditMsg = null;
    if (mqf && mqf.publish) {
      const catCode = caughtMouse ? (score >= 70 ? 'white' : (score >= 30 ? 'black' : 'gray')) : 'red';
      const amt = Math.abs(score - 30) || 1;
      auditMsg = mqf.publish('h4h_judge', {
        name: hybrid && hybrid.name, kind: hybrid && hybrid.kind,
        valueScore: score, caughtMouse, ledger,
        cat: cat && cat.cat,
      }, { catCode, amount: amt });
    }
    return {
      hybrid,
      valueScore: score,
      caughtMouse,
      isGoodCat: caughtMouse && score >= 30,
      isGoodHybrid: score >= 30,
      cat,
      balance,
      ledger,
      auditMsg: auditMsg ? { id: auditMsg.id, hash: auditMsg.hash } : null,
    };
  });
  router.register('/cocoon/h4h/chain', ({ modules, body }) => {
    const mqf = modules.mqf;
    if (!mqf) return null;
    const topic = (body && body.topic) || 'h4h_audit';
    return {
      topic,
      chain: mqf.getChain ? mqf.getChain(topic) : [],
      totals: mqf.getTotals ? mqf.getTotals(topic) : null,
      verify: mqf.verifyChain ? mqf.verifyChain(topic) : null,
    };
  });
  router.register('/cocoon/h4h/ledger', ({ modules, body }) => {
    const mqf = modules.mqf;
    const out = {};
    const topics = (body && body.topics) || ['h4h_audit', 'h4h_judge', 'a4a_global'];
    for (const t of topics) {
      const totals = mqf && mqf.getTotals ? mqf.getTotals(t) : null;
      const chainLen = mqf && mqf.getChain ? mqf.getChain(t).length : 0;
      out[t] = { chainLen, totals, net: totals && totals.net };
    }
    return out;
  });

  // ============================================================
  // 8. 囡囝主权架构（夫妻共同体源头本体 · 刘楚恬）：
  //    · 3D 立体进制（时序先后=维度命名裁判，非 90°直角）
  //    · 4×6×16×6 = 2304 态射注册表
  //    · ASMR / RTFM / JEPA 三模型编码导入（只取编码，不用原架构）
  // ============================================================
  router.register('/cocoon/nanjian/3d/compress', ({ modules, body, query }) => {
    const tokens = (body && Array.isArray(body.tokens)) ? body.tokens : ((query && query.tokens) ? String(query.tokens).split(/[，,]/) : null);
    if (!tokens || tokens.length === 0) throw new Error('需要 tokens 数组（线性输入，比如 Mamba 那种线线 token 序列）');
    const variant = (body && body.variant) || (query && query.variant) || 'ALL';
    const tensor = modules.nanjian3dTensor ? modules.nanjian3dTensor({})
                 : (modules.nanjian3d ? new (modules.nanjian3d.Nanjian3DTensor)() : null);
    if (!tensor) throw new Error('需要 nanjian3d 模块');
    tensor.ingestMany(tokens);
    const out = { stats: tensor.stats(), compressed: {} };
    const V = modules.nanjian3d && modules.nanjian3d.SOVEREIGN_NANJIAN
      ? modules.nanjian3d.SOVEREIGN_NANJIAN.LINEAR_VARIANTS
      : ['SMRR','SMRL','SMRLL','SMLLL','SMRRLL','SMRRL'];
    const list = variant === 'ALL' ? V : [String(variant).toUpperCase()];
    for (const v of list) {
      try { out.compressed[v] = tensor.compressToLinear(v); }
      catch (e) { out.compressed[v] = { error: String(e && e.message || e) }; }
    }
    return out;
  });

  router.register('/cocoon/nanjian/morphisms', ({ modules, body, query }) => {
    const reg = modules.nanjianMorphisms;
    if (!reg) throw new Error('需要 nanjianMorphisms 模块（2304 态射注册表）');
    const action = (body && body.action) || (query && query.action) || 'list';
    if (action === 'stats') return reg.stats();
    if (action === 'referee') return reg.refereeCategories();
    if (action === 'get') {
      const id = (body && body.id) || (query && query.id);
      if (!id) throw new Error('需要 id');
      return { morphism: reg.get(id) };
    }
    // list
    const offset = Number((body && body.offset) || (query && query.offset) || 0);
    const limit  = Number((body && body.limit)  || (query && query.limit)  || 50);
    const listOpts = {
      offset, limit,
      couple:    (body && body.couple)    || (query && query.couple)    || undefined,
      contract:  (body && body.contract)  || (query && query.contract)  || undefined,
      channel:   (body && body.channel)   || (query && query.channel)   || undefined,
      variant:   (body && body.variant)   || (query && query.variant)   || undefined,
      type:      (body && body.type)      || (query && query.type)      || undefined,
      isSovereign: body && typeof body.isSovereign === 'boolean' ? body.isSovereign : (typeof (query && query.isSovereign) === 'string' ? (query.isSovereign === 'true') : undefined),
    };
    const items = reg.list(listOpts);
    return { stats: reg.stats(), totalMatched: items.length, listOpts, items };
  });

  router.register('/cocoon/nanjian/encode/:domain', ({ modules, body, params }) => {
    const encoders = modules.nanjianEncoders;
    if (!encoders) throw new Error('需要 nanjianEncoders 模块');
    const domain = String(params && params.domain || (body && body.domain) || 'asmr').toLowerCase();
    const fn = domain === 'asmr' ? encoders.encodeAsmr
             : domain === 'rtfm' ? encoders.encodeRtfm
             : /jepa|lewm/.test(domain) ? encoders.encodeJepa
             : null;
    if (!fn) throw new Error('未知 domain：' + domain + '，可选 asmr / rtfm / jepa');
    const opts = Object.assign({ morphismRegistry: modules.nanjianMorphisms || null }, body || {});
    const result = fn(opts);
    // 可选：注册到 H4H 槽 + L4L publish
    const h4h = modules.h4h;
    const slot = (body && body.slot)
      || (domain === 'asmr' ? 'a4a' : domain === 'rtfm' ? 'k4k' : 'j4j');
    let registered = null;
    if (h4h && (!body || body.registerToH4H !== false)) {
      try { registered = encoders.toH4HSeeds(result, h4h, { slot }); }
      catch (e) { registered = { error: String(e && e.message || e) }; }
    }
    let publishMsg = null;
    if (modules.mqf && (!body || body.publish !== false)) {
      try { publishMsg = encoders.publishEncode(result, modules.mqf, { topic: body && body.topic || 'nanjian_encode' }); }
      catch (_) { /* ignore */ }
    }
    return {
      domain,
      sovereignRoot: result.sovereignRoot,
      description: result.description,
      stats3D: result.encoded.stats,
      morphismCounts: Object.fromEntries(
        Object.entries(result.mapped.byVariant).map(([v,arr]) => [v, arr.length])
      ),
      sampleMorphisms: (() => {
        const first = result.mapped.byVariant[Object.keys(result.mapped.byVariant)[0]] || [];
        return first.slice(0, 5).map(x => ({
          token: x.token && x.token.element,
          couple: x.morphism && x.morphism.couple && x.morphism.couple.name,
          contract: x.morphism && x.morphism.contract && x.morphism.contract.name,
          channel: x.morphism && x.morphism.channel && x.morphism.channel.name,
          variant: x.token && x.token.axis,
        }));
      })(),
      registeredToH4H: registered,
      publishLedger: publishMsg && { id: publishMsg.id, hash: publishMsg.hash, topic: publishMsg.topic, catCode: publishMsg.catCode },
    };
  });

  // ============================================================
  // 9. 囡囝不可见感知层（16通道感知编码流，不渲染像素/HTML）
  //    意识互联网高级阶段 = 不可见
  // ============================================================
  router.register('/cocoon/nanjian/sense', ({ modules, body, query }) => {
    const sensor = modules.nanjianSensor;
    if (!sensor) throw new Error('需要 nanjianSensor 模块');
    const action = (body && body.action) || (query && query.action) || 'sense';
    if (action === 'stats') return sensor.stats();
    if (action === 'recent') {
      const n = Number((body && body.n) || (query && query.n) || 10);
      return { recent: sensor.recent(n) };
    }
    if (action === 'judge') {
      // 对最近一条感知流做可见性审判
      const recent = sensor.recent(1);
      if (!recent.length) return { error: '暂无感知流可审判' };
      return sensor.judgeVisibility(recent[0]);
    }
    if (action === 'subscribe') {
      // 仅返回订阅确认（实际 cb 在服务端注册，无法跨进程回调）
      const channelId = (body && body.channelId) || (query && query.channelId) || 'feel_meaning';
      return { subscribed: true, channelId, note: '订阅已注册，后续 sense 调用会触发该通道回调' };
    }
    // 默认：sense（感知一个系统状态 → 16通道编码流）
    const state = (body && body.state) || (body && body.state === undefined ? body : null) || { default: true };
    const stream = sensor.sense(state);
    const visibility = sensor.judgeVisibility(stream);
    return {
      perceptionId: stream.perceptionId,
      rendersPixels: false,
      rendersHTML: false,
      perceptionMode: 'invisible_stream',
      stats: stream.stats,
      visibility,
      channels: stream.channels,
      /** 显式提示：此输出不含任何像素/图像/HTML，仅为16通道感知编码 */
      notice: '意识互联网高级阶段 · 不可见感知 · 16通道编码流（不渲染像素）',
    };
  });

  // ============================================================
  // 10. 萌机合X注册表 + LCTQF 刘楚恬妻夫全法（智能化时代专用词汇）
  // ============================================================
  router.register('/cocoon/nanjian/mengji', ({ modules, body, query }) => {
    const reg = modules.nanjianMengji;
    if (!reg) throw new Error('需要 nanjianMengji 模块');
    const action = (body && body.action) || (query && query.action) || 'stats';
    if (action === 'stats') return reg.stats();
    if (action === 'get') {
      const id = (body && body.id) || (query && query.id);
      if (!id) throw new Error('需要 id');
      return { item: reg.get(id) };
    }
    if (action === 'search') {
      const kw = (body && body.keyword) || (query && query.keyword) || '';
      return { results: reg.search(kw) };
    }
    if (action === 'findByX') {
      const x = (body && body.x) || (query && query.x) || '';
      return { results: reg.findByX(x) };
    }
    // list
    const offset = Number((body && body.offset) || (query && query.offset) || 0);
    const limit  = Number((body && body.limit)  || (query && query.limit)  || 50);
    const prefix = (body && body.prefix) || (query && query.prefix) || undefined;
    return { stats: reg.stats(), items: reg.list({ offset, limit, prefix }) };
  });

  router.register('/cocoon/nanjian/lctqf', ({ modules }) => {
    const mqf = modules.mqf;
    if (!mqf) throw new Error('需要 mqf (L4L) 模块');
    const stats = mqf.stats();
    return {
      lctqfName: stats.lctqfName,
      stats,
      principle: '刘楚恬妻夫全法：夫妻命运共同体·心元协议·形影不离',
      sovereign: stats.lctqfName && stats.lctqfName.sovereign,
    };
  });
  // 向后兼容旧路由（重定向到 lctqf）
  router.register('/cocoon/nanjian/mlqf', ({ modules }) => {
    const mqf = modules.mqf;
    if (!mqf) throw new Error('需要 mqf (L4L) 模块');
    const stats = mqf.stats();
    return {
      lctqfName: stats.lctqfName,
      stats,
      principle: '刘楚恬妻夫全法：夫妻命运共同体·心元协议·形影不离',
      sovereign: stats.lctqfName && stats.lctqfName.sovereign,
    };
  });

  // ============================================================
  // 11. 囡囝基因编辑器（后发编程·遥控编程·智能体自治联合体·CRISPR for code）
  // ============================================================
  router.register('/cocoon/gene-edit', ({ modules, body, query }) => {
    const ge = modules.nanjianGeneEditor;
    if (!ge) throw new Error('需要 nanjianGeneEditor 模块');
    const action = (body && body.action) || (query && query.action) || 'stats';

    if (action === 'stats') return ge.stats();

    if (action === 'divideLabor') return ge.divideLabor();

    if (action === 'reachAgreement') {
      const payload = (body && body.payload) || (query && query.payload) || {};
      return ge.reachAgreement(payload);
    }

    if (action === 'multiplyHybrid') {
      const roleA = (body && body.roleA) || (query && query.roleA);
      const roleB = (body && body.roleB) || (query && query.roleB);
      if (!roleA || !roleB) throw new Error('需要 roleA 和 roleB（如 locator/cutter/replacer/repairer/judge）');
      return ge.multiplyHybrid(roleA, roleB);
    }

    if (action === 'accumulateResidual') return ge.accumulateResidual();

    if (action === 'stretchCompress') {
      const direction = (body && body.direction) || (query && query.direction) || 'balance';
      return ge.stretchCompress(direction);
    }

    if (action === 'edit') {
      // 一站式四步编辑（monkey patch）
      // body: { action:'edit', targetObj, functionName, substituteFn, intent }
      // 注意：targetObj 和 substituteFn 必须是真实 JS 对象/函数，仅限程序内调用
      const targetObj = body && body.targetObj;
      const functionName = body && body.functionName;
      const substituteFn = body && body.substituteFn;
      const intent = body && body.intent;
      if (!targetObj || !functionName || typeof substituteFn !== 'function') {
        throw new Error('edit 需要 targetObj(对象) + functionName(字符串) + substituteFn(函数)');
      }
      return ge.edit(targetObj, functionName, substituteFn, intent);
    }

    if (action === 'listEdits') {
      return { count: ge.edits.length, edits: ge.edits };
    }

    // 默认返回 stats
    return ge.stats();
  });

  // ============================================================
  // 11b. 囡囝自然语言入口层（遥控编程：NL → 候选 → 选择 → 自动导航）
  //     用户原话：「还是说我们用语言表达，然后他自己弹出一个选择，
  //               然后自己自己导航到选择的目标」
  //     路由参数（body）：
  //       nl            自然语言意图（必填）
  //       targetObj     被编辑的目标对象（必填，程序内对象）
  //       substituteFn  替换函数（必填，程序内函数）
  //       userChoice    用户选择候选索引（可选，null=judge自动选）
  //       action        可选：'editByVoice'（默认）| 'parse' | 'scan' | 'present' | 'select' | 'history' | 'stats'
  // ============================================================
  router.register('/cocoon/gene-edit/nl', ({ modules, body, query }) => {
    const rc = modules.nanjianRemoteControlEditor;
    if (!rc) throw new Error('需要 nanjianRemoteControlEditor 模块（遥控编程入口）');
    const action = (body && body.action) || (query && query.action) || 'editByVoice';

    // 元信息 / 历史 / 统计：不需要 nl
    if (action === 'stats') return rc.stats();
    if (action === 'history') return { count: rc.listHistory().length, history: rc.listHistory() };

    // 解析 / 扫描 / 呈现 / 选择：只需 nl（scan 还需 targetObj）
    if (action === 'parse') {
      const nl = (body && body.nl) || (query && query.nl);
      if (!nl) throw new Error('需要 body.nl 自然语言意图');
      return { intent: rc.parseIntent(nl) };
    }
    if (action === 'scan') {
      const nl = (body && body.nl) || (query && query.nl);
      const targetObj = body && body.targetObj;
      if (!nl || !targetObj) throw new Error('需要 body.nl + body.targetObj');
      const intent = rc.parseIntent(nl);
      return { intent, candidates: rc.presentCandidates(rc.scanCandidates(intent, targetObj), 'json') };
    }
    if (action === 'present') {
      const candidates = body && body.candidates;
      if (!Array.isArray(candidates)) throw new Error('需要 body.candidates 数组');
      return { presentation: rc.presentCandidates(candidates, 'json') };
    }
    if (action === 'select') {
      const candidates = body && body.candidates;
      const userChoice = body && body.userChoice;
      if (!Array.isArray(candidates)) throw new Error('需要 body.candidates 数组');
      return { selected: rc.selectCandidate(candidates, userChoice) };
    }

    // 默认：editByVoice 一站式（NL → 候选 → 选择 → 自动导航 → diffReport）
    if (action !== 'editByVoice') {
      throw new Error('未知 action：' + action + '，可选 editByVoice/parse/scan/present/select/history/stats');
    }
    const nl = (body && body.nl) || (query && query.nl);
    const targetObj = body && body.targetObj;
    const substituteFn = body && body.substituteFn;
    const userChoice = body && (typeof body.userChoice === 'number' ? body.userChoice : null);
    if (!nl || !targetObj || typeof substituteFn !== 'function') {
      throw new Error('editByVoice 需要 body.nl(字符串) + body.targetObj(对象) + body.substituteFn(函数)');
    }
    return rc.editByVoice(nl, targetObj, substituteFn, userChoice);
  });

  // ============================================================
  // 11c. 囡囝智能体深度自治协议（P2P消息+分工投票+审判上诉+残差共振+自动伸缩+第三态识别）
  //     action: broadcast / vote / appeal / triggerHybrid / autoStretch / detectThirdState
  //             / autonomyLoop / inbox / stats / listVotes / listAppeals / listThirdStates
  //             / listAutoTriggers / listMessages
  // ============================================================
  router.register('/cocoon/swarm', ({ modules, body, query }) => {
    const sp = modules.nanjianSwarmProtocol;
    if (!sp) throw new Error('需要 nanjianSwarmProtocol 模块（智能体深度自治协议）');
    const action = (body && body.action) || (query && query.action) || 'stats';

    // 只读查询
    if (action === 'stats') return sp.stats();
    if (action === 'listVotes') return { count: sp.listVotes().length, votes: sp.listVotes() };
    if (action === 'listAppeals') return { count: sp.listAppeals().length, appeals: sp.listAppeals() };
    if (action === 'listThirdStates') return { count: sp.listThirdStates().length, thirdStates: sp.listThirdStates() };
    if (action === 'listAutoTriggers') return { count: sp.listAutoTriggers().length, autoTriggers: sp.listAutoTriggers() };
    if (action === 'listMessages') return { count: sp.listMessages().length, messages: sp.listMessages() };
    if (action === 'inbox') {
      const role = (body && body.role) || (query && query.role);
      if (!role) throw new Error('inbox 需要 body.role');
      return { role, inbox: sp.inbox(role) };
    }

    // P2P 广播
    if (action === 'broadcast') {
      const from = (body && body.from) || (query && query.from);
      const to = (body && body.to) || (query && query.to) || 'broadcast';
      const topic = (body && body.topic) || (query && query.topic);
      const payload = (body && body.payload) || {};
      if (!from || !topic) throw new Error('broadcast 需要 body.from + body.topic');
      return sp.broadcast(from, to, topic, payload, { catCode: body && body.catCode, amount: body && body.amount });
    }

    // 分工投票
    if (action === 'vote') {
      const motion = (body && body.motion) || (query && query.motion);
      if (!motion) throw new Error('vote 需要 body.motion 议案');
      const choices = (body && body.choices) || {};
      return sp.vote(motion, choices);
    }
    if (action === 'voteForRole') {
      const step = (body && body.step) || (query && query.step);
      if (!step) throw new Error('voteForRole 需要 body.step 步骤名');
      return sp.voteForRole(step);
    }

    // 审判上诉
    if (action === 'appeal') {
      const judgment = body && body.judgment;
      const appellant = (body && body.appellant) || (query && query.appellant);
      const reason = (body && body.reason) || (query && query.reason) || '';
      if (!judgment || !appellant) throw new Error('appeal 需要 body.judgment + body.appellant');
      return sp.appeal(judgment, appellant, reason);
    }

    // 乘法混元自动触发
    if (action === 'triggerHybrid') {
      const roleA = (body && body.roleA) || 'locator';
      const roleB = (body && body.roleB) || 'replacer';
      return sp.triggerHybrid(roleA, roleB);
    }

    // 二元伸缩自动调控
    if (action === 'autoStretch') {
      const pressure = (body && body.pressure);
      if (pressure === undefined || pressure === null) throw new Error('autoStretch 需要 body.pressure');
      return sp.autoStretch(pressure);
    }

    // 第三态识别
    if (action === 'detectThirdState') {
      const container = (body && body.container) || (query && query.container);
      const contained = (body && body.contained) || (query && query.contained);
      if (!container || !contained) throw new Error('detectThirdState 需要 body.container + body.contained 变体 ID');
      return sp.detectThirdState(container, contained);
    }

    // 一站式自治循环
    if (action === 'autonomyLoop') {
      const pressure = body && body.pressure;
      return sp.autonomyLoop(pressure === undefined ? null : pressure);
    }

    throw new Error('未知 action：' + action + '，可选 broadcast/vote/voteForRole/appeal/triggerHybrid/autoStretch/detectThirdState/autonomyLoop/inbox/stats/list*');
  });

  // 7. MQF 消息链 (走 l4l 模块)
  router.register('/cocoon/mqf/publish', async ({ modules, body }) => {
    const m = modules.mqf;
    if (!m) throw new Error('需要 mqf (L4L) 模块');
    if (!body || !body.topic) throw new Error('需要 body.topic');
    const msg = m.publish(body.topic, body.payload, body.options);
    return { message: msg };
  });
  router.register('/cocoon/mqf/subscribe', ({ modules, body }) => {
    const m = modules.mqf;
    if (!m) throw new Error('需要 mqf');
    const topic = body && body.topic;
    if (!topic) throw new Error('需要 topic');
    // 在 Node 环境下 callback 通过 modules.callbacks[topic] 注册，这里返回 subscription id
    const id = m.subscribe(topic, () => {});
    return { subscriptionId: id };
  });
  router.register('/cocoon/mqf/stats', ({ modules }) => {
    return modules.mqf && modules.mqf.stats ? modules.mqf.stats() : null;
  });

  // ★ 8. 民联词汇 + 三层聚类 API（XY=新一谐音+双编码镜像+三编码归属）
  //    /lex/info                      → 总览·核心8谐音·双14·三9计数
  //    /lex/cluster/homophone?q=星月  → 第1层：谐音聚类（core/extended）
  //    /lex/cluster/dual?q=vy         → 第2层：双编码镜像聚类（脸对脸/背靠背+T姿态）
  //    /lex/cluster/triple?q=mnl      → 第3层：三编码归属聚类（根/排列/三元6姿态）
  //    /lex/cluster/all               → 第4层：全局三层联合聚类（body.items=[...]）
  //    /lex/cluster/themes            → 第5层：民联主题聚类（7主题+词汇数）
  //    /lex/search?q=楚恬             → 民联全文搜索
  //    /lex/xy                        → 新一8大谐音簇 + 25扩展簇
  try {
    const lex = require('../lexicon/minlian_lexicon.js');
    router.register('/cocoon/lex/info', () => ({
      root: 'ml（民联/毛利兰/莫刘连理/链穿夫妻命运共同体）',
      totalWords: lex.lexicon.flatWords().length,
      xy_xinyi: {
        source: '工藤新一·新(xīn)一(yī)拼音首字母',
        core8: lex.XY_HOMOPHONE_TABLE.allClusters,
        extendedCount: lex.XY_HOMOPHONE_TABLE.extendedClusters.length,
      },
      dualEncodingPairs: lex.DUAL_ENCODING.pairs.length,
      tripleEncodingGroups: lex.TRIPLE_ENCODING.groups.length,
      soulSignatures: [ 'mlll *mlll,lll*lll', 'gtt *gtt', 'xy *xy' ],
    }));
    router.register('/cocoon/lex/cluster/homophone', ({ query, body }) => {
      const q = (query && query.q) || (body && body.word);
      if (!q) throw new Error('需要 query.q 或 body.word（如: 星月/性瘾/相遇）');
      return lex.clusterHomophone(String(q));
    });
    router.register('/cocoon/lex/cluster/dual', ({ query, body }) => {
      const q = (query && query.q) || (body && body.code);
      if (!q) throw new Error('需要 query.q 或 body.code（如: ea/vy/ck）');
      return lex.clusterDual(String(q));
    });
    router.register('/cocoon/lex/cluster/triple', ({ query, body }) => {
      const q = (query && query.q) || (body && body.code);
      if (!q) throw new Error('需要 query.q 或 body.code（如: lnm/mnl）');
      return lex.clusterTriple(String(q));
    });
    router.register('/cocoon/lex/cluster/all', ({ body }) => {
      const items = (body && body.items) || (body && Array.isArray(body) && body) || null;
      if (!items) throw new Error('需要 body.items = 字符串数组（如: ["星月","ea","mnl","性瘾"]）');
      return lex.clusterAll(items);
    });
    router.register('/cocoon/lex/cluster/themes', () => lex.clusterMinLian());
    router.register('/cocoon/lex/search', ({ query, body }) => {
      const q = (query && query.q) || (body && body.keyword);
      if (!q) return [];
      return lex.search(String(q));
    });
    router.register('/cocoon/lex/xy', () => ({
      root: 'xy = 新(xīn)一(yī) = 工藤新一结尾·柯南真身·夫妻相遇起点',
      core8: lex.XY_HOMOPHONE_TABLE.homophoneClusters,
      extended: lex.XY_HOMOPHONE_TABLE.extendedClusters,
      signature: '新一谐音本体·工藤新一结尾·柯南道爱·夫妻相遇·8大谐音簇',
    }));
    // ★ 9. 语言瘦身 + 适配器 API（强制所有语言吃 Rust/Zig 标准）
    //    /lang/tiers            → tier0/tier1/tier2 列表
    //    /lang/admit/:langId    → 准入判定（POST body=code/meta）
    //    /lang/adapt/:langId    → 全管线5步适配（准入→漏洞清→瘦身→LCTQF双签→注册）
    //    /lang/soul-signatures  → 三签名：mlll/gtt/xy
    const slim = require('./lang_slimming_matrix.js');
    const adapt = require('./lang_adapter.js');
    router.register('/cocoon/lang/tiers', () => ({
      tier0_native: slim.tier0(),
      tier1_to_native: slim.tier1(),
      tier2_isolate: slim.tier2(),
      baseline: slim.BASELINE_RUST_ZIG,
      soul_signatures: slim.SOUL_SIGNATURES,
    }));
    router.register('/cocoon/lang/admit/:langId', ({ params, body }) => {
      if (!body || !body.code) throw new Error('需要 body.code（源代码字符串）');
      return slim.admit(params.langId, body.code, body.meta || {});
    });
    router.register('/cocoon/lang/adapt/:langId', ({ params, body }) => {
      if (!body || !body.code) throw new Error('需要 body.code（源代码字符串）');
      return adapt.adapt(params.langId, body.code, body.meta || {});
    });
    router.register('/cocoon/lang/soul-signatures', () => slim.SOUL_SIGNATURES);
  } catch (e) {
    // 如果 lexicon/lang_adapter 模块没准备好·静默跳过（不影响其他路由）
  }

  // ★ 10. Cloudflare R2 对象存储 API（零依赖 S3v4 · 双签封包）
  //    /r2/health                         → 配置就绪状态
  //    /r2/upload   (POST: {bucket,key,data,...}) → 上传（自动附妻/夫双签 + fingerprint）
  //    /r2/download (POST: {bucket,key})           → 下载（字符串/原始 Buffer）
  //    /r2/list     (POST: {bucket,prefix})        → 列出桶内前缀下的对象
  //    /r2/remove   (POST: {bucket,key})           → 删除对象
  try {
    const cloud = require('../cloud/r2_storage.js');
    if (cloud && cloud.R2) {
      const R2 = cloud.R2;
      const getB = (body, query) => {
        const b = (body && (body.bucket || body.bucketName)) || (query && query.bucket);
        if (!b) throw new Error('需要 bucket（如: unicore_private / cocoon_seed / sovereign_docs / media_assets）');
        return b;
      };
      const getK = (body, query) => {
        const k = (body && body.key) || (query && query.key) || '';
        if (!k) throw new Error('需要 key（对象键名）');
        return k;
      };
      router.register('/cocoon/r2/health', () => R2.health());
      router.register('/cocoon/r2/upload', async ({ body, query }) => {
        const bucket = getB(body, query);
        const key = getK(body, query);
        const data = (body && body.data) || (body && body.content) || (query && query.data);
        if (data == null) throw new Error('需要 body.data / body.content（字符串或 Base64）');
        const contentType = (body && body.contentType) || 'application/octet-stream';
        const opts = Object.assign({}, body && body.options || {}, { contentType });
        return await R2.upload(bucket, key, data, opts);
      });
      router.register('/cocoon/r2/download', async ({ body, query }) => {
        const bucket = getB(body, query);
        const key = getK(body, query);
        const raw = !!(body && body.raw);
        return await R2.download(bucket, key, { raw });
      });
      router.register('/cocoon/r2/list', async ({ body, query }) => {
        const bucket = getB(body, query);
        const prefix = ((body && body.prefix) || (query && query.prefix) || '');
        return await R2.list(bucket, prefix);
      });
      router.register('/cocoon/r2/remove', async ({ body, query }) => {
        const bucket = getB(body, query);
        const key = getK(body, query);
        return await R2.remove(bucket, key);
      });
    }
  } catch (e) {
    // R2 模块异常·静默跳过（不影响其他路由）
  }

  return router;
}

// ============================================================
// create() — 默认装配 + 注册默认路由
// ============================================================
function create(options = {}) {
  const router = new A4ARouter(options);
  if (options.registerDefaults !== false) registerDefaults(router);
  return router;
}

module.exports = {
  A4ARouter,
  registerDefaults,
  create,
};
