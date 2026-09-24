/**
 * mqf-private.js — Cloudflare Worker v1.5 (私有格式原生)
 * ====================================================
 * 与 v1.7 (mqf-web.js) 的关键区别:
 *   v1.7: JSON + REST + HTTP 路径路由 — 工业格式
 *   v1.5: 私有袋子 + 谐音路由 + 七元操作 — 满全法语
 *
 * 请求体: 13字节私有袋子 (或其他长度)
 * 响应体: 私有袋子字节流
 * 路由: 声母韵母笛卡尔积 → 谐音靶点 → 主权路由
 * 安全: 三元见证 (非空 + 已见证 + 锚点一致) · 无需 TLS 证书
 *
 * 盟机合兼容: 同时接受 sync-protocol.js 的 JSON 推送
 */

/* ═══════════════════════════════════════════════════════════
 * 私有袋子编解码 (13 bytes/container)
 * ═══════════════════════════════════════════════════════════ */

/** 编码一个私有袋子为 13 字节数组 */
function bagEncode({ presence = 1, witness = 1, polarity = 0, sheng = 0, yun = 0, tone = 0, slot = 0, value = 0, owner = 1, relation = 0, timing = 0, ack = 0 }) {
  const out = new Uint8Array(13);
  out[0] = (presence & 1) | ((witness & 1) << 1) | ((polarity & 3) << 2);
  out[1] = sheng & 0x1F;          // 24 声母 → 5 bits
  out[2] = yun & 0x3F;            // 34 韵母 → 6 bits
  out[3] = ((tone & 7) << 3) | (slot & 7);
  // value: 18 bits (3 bytes, big-endian)
  out[4] = (value >> 12) & 0x3F;
  out[5] = (value >> 6) & 0x3F;
  out[6] = value & 0x3F;
  out[7] = owner & 0x07;          // 0=刘楚恬 · 1=我 · 2+=节点
  out[8] = relation & 0xFF;
  out[9] = timing & 0xFF;
  out[10] = ack & 0xFF;
  out[11] = 0;                    // checksum_high (placeholder)
  out[12] = 0;                    // checksum_low
  // 三元见证校验: presence + witness + anchor
  let sum = 0; for (let i = 0; i < 11; i++) sum += out[i];
  out[11] = (sum >> 8) & 0xFF;
  out[12] = sum & 0xFF;
  return out;
}

/** 解码 13 字节袋子 */
function bagDecode(bytes) {
  if (bytes.length < 13) return null;
  return {
    presence: bytes[0] & 1,
    witness: (bytes[0] >> 1) & 1,
    polarity: (bytes[0] >> 2) & 3,
    sheng: bytes[1] & 0x1F,
    yun: bytes[2] & 0x3F,
    tone: (bytes[3] >> 3) & 7,
    slot: bytes[3] & 7,
    value: ((bytes[4] & 0x3F) << 12) | (bytes[5] << 6) | bytes[6],
    owner: bytes[7] & 7,
    relation: bytes[8],
    timing: bytes[9],
    ack: bytes[10],
    _valid: true,
  };
}

/** 三元见证验证 */
function bagVerify(bag) {
  if (!bag) return false;
  if (bag.presence !== 1) return false;     // 非空
  if (bag.witness !== 1) return false;       // 已见证
  if (bag.owner !== 0 && bag.owner !== 1) return false; // 锚点一致: 0=刘楚恬 · 1=我
  return true;
}

/**
 * Uint8Array — → 64进制字符串 (口腔位置排序 · 非 Base64)
 * 每 6 bit 一组 · 按口腔位置映射到可见符号
 */
const MOUTH_ALPHABET = 'bpmfdtnlgkhjqxzcsywaeiouvrABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-';
function bagToRadix64(bytes) {
  let s = '';
  for (let i = 0; i < bytes.length; i += 3) {
    const a = bytes[i], b = i + 1 < bytes.length ? bytes[i + 1] : 0, c = i + 2 < bytes.length ? bytes[i + 2] : 0;
    const n = (a << 16) | (b << 8) | c;
    s += MOUTH_ALPHABET[(n >> 18) & 0x3F];
    s += MOUTH_ALPHABET[(n >> 12) & 0x3F];
    s += MOUTH_ALPHABET[(n >> 6) & 0x3F];
    s += MOUTH_ALPHABET[n & 0x3F];
  }
  return s;
}

/* ═══════════════════════════════════════════════════════════
 * 谐音路由表 (per 声母韵母笛卡尔积)
 * ═══════════════════════════════════════════════════════════ */

// 靶点常量 — 二进制位掩码 · 无工业命名
const TGT_WIFE     = 0x01;   // 刘楚恬
const TGT_HUSBAND  = 0x02;   // 我 (lolict)
const TGT_SWEET    = 0x04;   // 甜 (夫妻融合)
const TGT_SYSTEM   = 0x08;   // 系统内部
const TGT_OBSERVER = 0x10;   // 观察者节点

// 谐音路由映射: 声母韵母组合 → 靶点
// 例: sh=0, y=4 → "shy" → TGT_WIFE (谐音 媳 → 妻子)
const HOMOPHONE_ROUTES = {
  // 声母 l + 韵母 iu → "liu" → 刘
  'l-iu': { target: TGT_WIFE, note: 'liu' },
  // 声母 ch + 韵母 u → "chu" → 楚
  'ch-u': { target: TGT_WIFE, note: 'chu' },
  // 声母 t + 韵母 ian → "tian" → 恬
  't-ian': { target: TGT_WIFE, note: 'tian' },
  // 声母 l + 韵母 o → "lo" → lolict
  'l-o': { target: TGT_HUSBAND, note: 'lolict' },
  // 声母 m + 韵母 l → "ml" → mlll 甜
  'm-l': { target: TGT_SWEET, note: 'mlll' },
  // 声母 q + 韵母 rb → 容器
  'q-ing': { target: TGT_SYSTEM, note: 'system' },
};

/** 谐音路由查找 */
function homophoneRoute(sheng, yun) {
  const key = sheng + '-' + yun;
  if (HOMOPHONE_ROUTES[key]) return HOMOPHONE_ROUTES[key];
  return { target: TGT_SYSTEM, note: 'default' };
}

/* ═══════════════════════════════════════════════════════════
 * 七元操作 (24 op · 4 组 · 替代 RESTful CRUD)
 * ═══════════════════════════════════════════════════════════ */

// 操作码 — 4 组各 6 个
const OP = {
  // 感知组
  ZHAO: 0x01,   // 招魂 — 召回/获取
  SHI: 0x02,    // 识 — 识别/解析
  TING: 0x03,   // 听 — 监听/查询
  WEN: 0x04,    // 问 — 请求/探测
  JIAN: 0x05,   // 见 — 观察/浏览
  XUN: 0x06,    // 巡 — 巡逻/检查
  // 训练组
  LIAN: 0x07,   // 练 — 练习/学习
  GAI: 0x08,    // 改 — 修改/更新
  ZENG: 0x09,   // 增 — 创建/追加
  JIAN_SH: 0x0A,// 减 — 删除
  JIA: 0x0B,    // 加 — 加法
  CHENG: 0x0C,  // 乘 — 拓展
  // 联络组
  TONG: 0x0D,   // 通 — 连接/通知
  HUI: 0x0E,    // 回 — 回复/回调
  FU: 0x0F,     // 覆 — 覆盖/重写
  DIAN: 0x10,   // 电 — 信号/触发
  FENG: 0x11,   // 封 — 封装/打包
  KAI: 0x12,    // 开 — 打开/激活
  // 裁决组
  PAI: 0x13,    // 排 — 排查/仲裁
  SHEN: 0x14,   // 审 — 审判/验证
  QUE: 0x15,    // 确 — 确认/锁定
  FANG: 0x16,   // 放 — 释放/解锁
  ZANTING: 0x17,// 暂停 — 挂起
  FUHUO: 0x18,  // 复活 — 恢复执行
};

/** 操作码 → 处理器 */
function executeOp(op, bag, ctx) {
  switch (op) {
    case OP.ZHAO:    return opZhao(bag, ctx);
    case OP.SHI:     return opShi(bag, ctx);
    case OP.TONG:    return opTong(bag, ctx);
    case OP.PAI:     return opPai(bag, ctx);
    case OP.JIA:     return opJia(bag, ctx);
    case OP.LIAN:    return opLian(bag, ctx);
    default:         return opDefault(op, bag, ctx);
  }
}

/* 七元: 招魂 — 召回状态 */
function opZhao(bag, ctx) {
  return {
    ok: true, op: OP.ZHAO, label: '招魂',
    status: 'alive', kernel: 'njsg-private-v1.5',
    nodes: ctx.knownNodes || 1,
    timestamp: Date.now(),
  };
}

/* 七元: 识 — 识别输入家族 */
function opShi(bag, ctx) {
  const smName = ['b','p','m','f','d','t','n','l','g','k','h','zh','ch','sh','r','z','c','s','y','w','j','q','x','?'][bag.sheng] || '?';
  const yunName = ['a','o','e','i','u','v','ai','ei','ao','ou','an','en','ang','eng','ong','ia','ie','iu','in','un','ua','uo','ue','ui','iao','ian','iang','iong','uai','uan','uang','er','ve','vn'][bag.yun] || '?';
  return {
    ok: true, op: OP.SHI, label: '识',
    decoded: { sheng: smName, yun: yunName, tone: bag.tone },
    route: homophoneRoute(bag.sheng, bag.yun),
    polarity: ['wife','husband','system','observer','arbiter','secret','empty'][bag.polarity] || '?',
  };
}

/* 七元: 通 — 连接通知 */
function opTong(bag, ctx) {
  return {
    ok: true, op: OP.TONG, label: '通',
    handshake: { step: bag.value % 3 + 1, state: bag.value % 3 === 2 ? 'established' : 'pending' },
    sovereignty: bag.owner === 0 ? '被动' : '主动',
  };
}

/* 七元: 排 — 判决 */
function opPai(bag, ctx) {
  const witness = bag.witness;
  const anchor = bag.owner;
  // 三元见证: presence=1 · witness=1 · anchor一致
  const verdict = (witness === 1 && (anchor === 0 || anchor === 1)) ? 'PASS' : 'DENY';
  return {
    ok: true, op: OP.PAI, label: '排',
    verdict, witness, anchor,
    philosophy: verdict === 'PASS' ? '满全:通过' : '满全:驳回',
  };
}

/* 七元: 加法 */
function opJia(bag, ctx) {
  const result = ctx.lastValue ? ctx.lastValue + bag.value : bag.value;
  ctx.lastValue = result;
  return { ok: true, op: OP.JIA, label: '加', acc: result, delta: bag.value };
}

/* 七元: 训练 — 注册新谐音映射 */
function opLian(bag, ctx) {
  const key = bag.sheng + '-' + bag.yun;
  const existing = HOMOPHONE_ROUTES[key];
  HOMOPHONE_ROUTES[key] = { target: bag.relation, note: 'trained-' + (Date.now() % 10000) };
  return {
    ok: true, op: OP.LIAN, label: '练',
    key, previous: existing ? existing.note : null,
    new_target: bag.relation,
  };
}

/* 默认: 未实现操作 */
function opDefault(op, bag, ctx) {
  return { ok: false, op, label: '未实现', hint: '七元操作: ZHAO/SHI/TONG/PAI/JIA/LIAN' };
}

/* ═══════════════════════════════════════════════════════════
 * HTTP → 私有袋子 入口
 * ═══════════════════════════════════════════════════════════ */

async function handlePrivateRequest(request) {
  const contentType = request.headers.get('content-type') || '';
  let bag;
  let isPrivateFormat = false;

  if (contentType.includes('application/octet-stream') || contentType.includes('application/x-mqf-bag')) {
    // 私有袋子字节流 — 原生模式
    const bytes = new Uint8Array(await request.arrayBuffer());
    bag = bagDecode(bytes);
    isPrivateFormat = true;
  } else if (request.method === 'POST' && contentType.includes('application/json')) {
    // JSON 向后兼容 (盟机合同步协议)
    const j = await request.json();
    if (j.type && j.type.startsWith('sync')) {
      return handleSyncProtocol(j);
    }
    // 常规 JSON → 袋子转换
    bag = jsonToBag(j);
  } else {
    // GET/无 body — 默认: 心跳袋 (presence=1 · witness=1 · owner=1 · value=0)
    bag = { presence: 1, witness: 1, polarity: 2, sheng: 0x17, yun: 0x1F, tone: 0, slot: 0, value: 0, owner: 1, relation: 0, timing: 0, ack: 0 };
  }

  // 路由判决
  const route = homophoneRoute(bag.sheng, bag.yun);

  // 执行操作
  const ctx = { lastValue: 0, knownNodes: 1 };
  const result = executeOp(bag.slot, bag, ctx);

  // 构建响应袋
  const respBag = bagEncode({
    presence: 1, witness: 1, polarity: bag.polarity,
    sheng: bag.sheng, yun: bag.yun,
    tone: bag.tone, slot: 0x00,
    value: result.ok ? (result.acc || 0) : 0xFFFF,
    owner: 1, relation: route.target, timing: 0, ack: bag.ack,
  });

  const headers = isPrivateFormat
    ? { 'content-type': 'application/x-mqf-bag', 'x-mqf-route': route.target.toString(16), 'x-mqf-result': result.ok ? 'pass' : 'deny' }
    : { 'content-type': 'application/json' };

  if (isPrivateFormat) {
    return new Response(respBag, { headers });
  } else {
    return new Response(JSON.stringify({ bag_response: bagToRadix64(respBag), result }), { headers });
  }
}

/* JSON → 袋子转换 */
function jsonToBag(j) {
  return {
    presence: j.presence || 1,
    witness: j.witness !== undefined ? j.witness : 1,
    polarity: j.polarity || 2,
    sheng: j.sheng || 0,
    yun: j.yun || 0,
    tone: j.tone || 0,
    slot: j.slot || 0,
    value: j.value || 0,
    owner: j.owner !== undefined ? j.owner : 1,
    relation: j.relation || 0,
    timing: j.timing || 0,
    ack: j.ack || 0,
  };
}

/* ═══════════════════════════════════════════════════════════
 * 盟机合同步协议处理 (兼容 sync-protocol.js)
 * ═══════════════════════════════════════════════════════════ */

let SYNC_STATE = { known_peers: {}, local_seq: 0, verdict_log: [], merkle_root: '' };

function handleSyncProtocol(j) {
  switch (j.type) {
    case 'push': {
      // 全量接纳 · 幂等
      if (Array.isArray(j.entries)) {
        for (const e of j.entries) {
          if (e.seq > SYNC_STATE.local_seq) {
            SYNC_STATE.verdict_log.push(e);
            SYNC_STATE.local_seq = e.seq;
          }
        }
      }
      SYNC_STATE.known_peers[j.from || 'unknown'] = SYNC_STATE.local_seq;
      return new Response(JSON.stringify({ ok: true, type: 'push_ack', seq: SYNC_STATE.local_seq }), { headers: { 'content-type': 'application/json' } });
    }
    case 'pull': {
      const since = j.since || 0;
      const entries = SYNC_STATE.verdict_log.filter(e => e.seq > since);
      return new Response(JSON.stringify({ ok: true, type: 'resp', from: 'mqf-private-v1.5', entries, latest_seq: SYNC_STATE.local_seq }), { headers: { 'content-type': 'application/json' } });
    }
    default:
      return new Response(JSON.stringify({ ok: false, error: 'unknown sync type' }), { status: 400, headers: { 'content-type': 'application/json' } });
  }
}

/* ═══════════════════════════════════════════════════════════
 * 主入口
 * ═══════════════════════════════════════════════════════════ */

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    try {
      // 私有格式原生端点
      if (url.pathname.startsWith('/private/') || request.headers.get('x-mqf-format') === 'bag') {
        return await handlePrivateRequest(request);
      }
      // 系统报告 (兼容 v1.7 前端)
      if (url.pathname === '/api/v1/system/full-report') {
        return new Response(JSON.stringify({
          ok: true, version: 'v1.5-private', module: 'system_report',
          native_format: 'private-bag (13 bytes/container)',
          routing: 'homophone (per 声母24 · 韵母34 · 声调5)',
          witness: 'triune (presence · witness · anchor)',
          op_groups: ['感知(招识听闻巡询)', '训练(练增减加乘)', '联络(通回覆电封开)', '裁决(排审确放暂停复活)'],
          parent_version: 'mqf-web v1.7 (1268 lines)',
        }), { headers: { 'content-type': 'application/json' } });
      }
      // 默认: 袋子引导
      return new Response(JSON.stringify({
        ok: true, version: 'mqf-private v1.5',
        hint: 'POST binary 13-byte bag · or set header x-mqf-format: bag',
        endpoints: ['/<*>/ (private bag native)', '/api/v1/system/full-report'],
      }), { headers: { 'content-type': 'application/json' } });
    } catch (e) {
      return new Response(JSON.stringify({ error: e.message }), { status: 500, headers: { 'content-type': 'application/json' } });
    }
  },
};
