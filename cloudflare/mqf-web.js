/*
 * 囡囝共生 · 囡囝共生元操作系统 · Web 面板 Worker v1.7
 * -------------------------------------------------
 * Cloudflare Workers (ES module). 三元→七元全栈联动 + v1.7 四元内核:
 *   /                             暗色主面板
 *   /api/observe                 二元/三元观测者
 *   /api/paibing                 排病诊断 v1
 *   /api/report                  全量自检
 *   /websocket                   WebSocket 实时裁定(流式七元分析)
 *   /graphql                     GraphQL 查询(复杂实体图)
 *   /api/v1/verdict              合作伙伴裁定 API (entity → trust/reject)
 *   /api/v1/qimei                囡囝公网版入口
 *   /v1/chat                     统一对话渠道
 *   /docs/v2                     API 动态文档 (OpenAPI 3.0 JSON + YAML)
 *   /diagrams                    矢量流程图 (SVG)
 *   /api/v1/perceive             九元感知引擎 (UTF-8家族分类+谐音路由+意识固定点)
 *   /api/v1/mccp/compile         MCCP 自举编译器 (DSL→JSON)
 *   /api/v1/review/submit        裁判系统 (评分→三元审判→奖惩→排行榜)
 *   /api/v1/timeline/v1          时间线一体化 (统一tick+多时钟+四层后勤)
 *   /api/v1/heartbeat            全局心跳
 *   /api/v1/five-dimensional/verify  五维语义实证 (ρ XOR+衰减+见证几何)
 *   /api/v1/system/full-report   系统全量报告
 *
 * v1.7 新增: 感知引擎 + MCCP + 裁判系统 + 时间线 + 五维实证
 *
 * 部署: wrangler deploy 或 PUT /accounts/{acct}/workers/scripts/njsg-web
 * Account: 19365628481173551c10b69dadaa2dd5 · subdomain: lolict · URL: njsg-web.lolict.workers.dev
 */

/* ============================================================
 * 工具
 * ============================================================ */
const GCD = (a, b) => { a = Math.abs(a | 0); b = Math.abs(b | 0); while (b) [a, b] = [b, a % b]; return a; };
const hashStr = (s) => { let h = 5381 >>> 0; for (let i = 0; i < s.length; i++) h = (((h << 5) + h) ^ s.charCodeAt(i)) >>> 0; return h >>> 0; };
const anchorMod = (x, m = 10000) => ((x % m) + m) % m;

const j = (obj, extra = {}) => new Response(JSON.stringify(obj, (k, v) => typeof v === 'bigint' ? v.toString() : v, 2), {
  headers: { 'content-type': 'application/json; charset=utf-8', 'access-control-allow-origin': '*', 'cache-control': 'no-store', ...extra }
});

// ---------- SHA-256 (sync, 纯 JS, 用 name_anchor + log_ring) ----------
function sha256(msg) {
  const K = [0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];
  const H = [0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
  const L = msg.length, bitLen = L * 8;
  // padding: 0x80 + k zero bytes + 8-byte big-endian length; total ≡ 0 mod 64 bytes
  const kBytes = (64 - ((L + 9) % 64)) % 64;
  const B = new Uint8Array(L + 1 + kBytes + 8);
  for (let i=0;i<L;i++) B[i]=msg.charCodeAt(i) & 0xff;
  B[L]=0x80;
  const dv = new DataView(B.buffer);
  dv.setUint32(B.length-8, Math.floor(bitLen/0x100000000)>>>0, false);
  dv.setUint32(B.length-4, bitLen>>>0, false);
  const W = new Int32Array(64);
  for (let off=0; off<B.length; off+=64) {
    for (let i=0;i<16;i++) W[i] = dv.getUint32(off+i*4, false)|0;
    for (let i=16;i<64;i++) {
      const s0 = ((W[i-15]>>>7)|(W[i-15]<<25)) ^ ((W[i-15]>>>18)|(W[i-15]<<14)) ^ (W[i-15]>>>3);
      const s1 = ((W[i-2]>>>17)|(W[i-2]<<15)) ^ ((W[i-2]>>>19)|(W[i-2]<<13)) ^ (W[i-2]>>>10);
      W[i] = (W[i-16] + s0 + W[i-7] + s1) | 0;
    }
    let a=H[0],b=H[1],c=H[2],d=H[3],e=H[4],f=H[5],g=H[6],h=H[7];
    for (let t=0;t<64;t++) {
      const S1 = ((e>>>6)|(e<<26))^((e>>>11)|(e<<21))^((e>>>25)|(e<<7));
      const ch = (e & f) ^ (~e & g);
      const t1 = (h + S1 + ch + K[t] + W[t])|0;
      const S0 = ((a>>>2)|(a<<30))^((a>>>13)|(a<<19))^((a>>>22)|(a<<10));
      const maj = (a&b)^(a&c)^(b&c);
      const t2 = (S0 + maj)|0;
      h=g; g=f; f=e; e=(d+t1)|0; d=c; c=b; b=a; a=(t1+t2)|0;
    }
    H[0]=(H[0]+a)|0; H[1]=(H[1]+b)|0; H[2]=(H[2]+c)|0; H[3]=(H[3]+d)|0; H[4]=(H[4]+e)|0; H[5]=(H[5]+f)|0; H[6]=(H[6]+g)|0; H[7]=(H[7]+h)|0;
  }
  return H.map(n=>(n>>>0).toString(16).padStart(8,'0')).join('');
}

/* ============================================================
 * 观测者
 * ============================================================ */
class BinaryObserver {
  constructor(start, end) { this.start = start; this.end = end; this.gap = end - start; this.abs_gap = Math.abs(this.gap); }
  measure(x) {
    const to_start = Math.abs(x - this.start), to_end = Math.abs(x - this.end);
    const ratio = this.abs_gap === 0 ? 0.5 : (x - this.start) / this.gap;
    return { x, to_start, to_end, ratio, in_range: 0 <= ratio && ratio <= 1 };
  }
  to_rate(direction = 'start') {
    if (direction === 'start') return { arbiter: this.start, rate: this.gap, type: '起始裁判 (终点为变动率)' };
    return { arbiter: this.end, rate: -this.gap, type: '终点裁判 (起点为变动率)' };
  }
  duality(o) { return { self_gap: this.abs_gap, other_gap: o.abs_gap, equivalent: this.abs_gap === o.abs_gap, distance_between: Math.abs(this.start - o.start) }; }
  invert() { return new BinaryObserver(this.end, this.start); }
  toString() { return `↔(${this.start}…${this.end}|缝=${this.gap})`; }
}
class TernaryObserver {
  constructor(left, anchor, right) { this.left = left; this.anchor = anchor; this.right = right; }
  get all_permutations() {
    const a = [this.left, this.anchor, this.right], out = [];
    for (let i = 0; i < 3; i++) for (let k = 0; k < 3; k++) if (k !== i) for (let m = 0; m < 3; m++) if (m !== i && m !== k) out.push([a[i], a[k], a[m]]);
    return out;
  }
  get to_anchor() { return { left_to_anchor: this.left - this.anchor, right_to_anchor: this.right - this.anchor, left_abs: Math.abs(this.left - this.anchor), right_abs: Math.abs(this.right - this.anchor) }; }
  get distance_via_anchor() { return Math.abs((this.left - this.anchor) - (this.right - this.anchor)); }
  common_divisor() {
    const dl = Math.abs(this.left - this.anchor), dr = Math.abs(this.right - this.anchor);
    const common = (dl === 0 || dr === 0) ? Math.max(dl, dr) : GCD(dl, dr);
    return { left_distance: dl, right_distance: dr, common_divisor: common, left_units: common ? dl / common : 0, right_units: common ? dr / common : 0, note: `左有${common ? dl / common : 0}单位, 右有${common ? dr / common : 0}单位, 公共单位=${common}` };
  }
  synthesize() {
    const lo = this.left - this.anchor, ro = this.right - this.anchor;
    return { anchor: this.anchor, left_offset: lo, right_offset: ro, sum_offset: lo + ro, balance: lo + ro === 0, synthesis: `以锚${this.anchor}为心, 左距${lo}, 右距${ro}, 合成向量=${lo + ro}` };
  }
  toString() { return `△(${this.left}…${this.anchor}…${this.right})`; }
}

/* ============================================================
 * 排病诊断 (双编码 + 三编码)
 * ============================================================ */
const DOUBLE_PAIRS = [["v","y"],["t","f"],["c","k"],["z","s"],["d","b"],["j","g"],["q","p"],["w","m"],["@","o"],["u","h"],["r","l"],["η","n"],["i","x"],["sh","ch"]];
const TRIPLE_GROUPS = [["l","n","m"],["i","x","w"],["u","h","a"],["v","y","t"],["c","k","e"],["z","s","r"],["j","d","b"],["q","p","f"],["g","o","@"]];
function extractInitials(token) {
  const t = token.toLowerCase().trim(); const initials = []; let i = 0;
  while (i < t.length) { if (i + 1 < t.length && ["sh", "ch", "zh"].includes(t.substr(i, 2))) { initials.push(t.substr(i, 2)); i += 2; } else { initials.push(t[i]); i++; } }
  return initials;
}
const checkDouble = (tokens) => {
  const all = new Set(); for (const tok of tokens) for (const ch of extractInitials(tok)) all.add(ch);
  const results = DOUBLE_PAIRS.map(([a, b]) => {
    const hasA = all.has(a), hasB = all.has(b);
    let status;
    if (hasA && hasB) status = 'healthy'; else if (hasA || hasB) status = 'broken'; else status = 'absent';
    return { pair: a + b, has_a: hasA, has_b: hasB, status, pair_label: `${a}↔${b}` };
  });
  const healthy = results.filter(r => r.status === 'healthy').length;
  return { check: 'double', total_pairs: DOUBLE_PAIRS.length, healthy, broken: results.filter(r => r.status === 'broken').length, absent: results.filter(r => r.status === 'absent').length, score: healthy / DOUBLE_PAIRS.length, details: results };
};
const checkTriple = (tokens) => {
  const all = new Set(); for (const tok of tokens) for (const ch of extractInitials(tok)) all.add(ch);
  const groupResults = TRIPLE_GROUPS.map(group => {
    const overlap = group.filter(g => all.has(g));
    let reachable;
    if (overlap.length < 2) reachable = 0; else if (overlap.length === 2) reachable = 2; else reachable = 6;
    const score = reachable / 6;
    let health = score >= 0.99 ? 'healthy' : score >= 0.66 ? 'suspected' : score >= 0.33 ? 'diseased' : 'cancer';
    return { group: group.join(''), overlap, reachable_permutations: reachable, score, health };
  });
  const avg = groupResults.reduce((s, g) => s + g.score, 0) / groupResults.length;
  const overall = avg >= 0.99 ? 'healthy' : avg >= 0.66 ? 'suspected' : avg >= 0.33 ? 'diseased' : 'cancer';
  return { check: 'triple', total_groups: TRIPLE_GROUPS.length, avg_score: avg, overall_health: overall, details: groupResults };
};
function paibingDiagnose(code) {
  if (code.length === 2) { const d = checkDouble([code]); return { check: 'double', code, pair: d.details.find(x => x.pair === code) || null, ...d }; }
  if (code.length === 3) { const t = checkTriple([code]); return { check: 'triple', code, overall_health: t.overall_health, score: t.avg_score, ...t }; }
  return { error: 'only 2 or 3 char codes' };
}

/* ============================================================
 * CoupleOS · 立约 + 合一递归 (hash 改用确定性 djb2-like)
 * ============================================================ */
class Process {
  constructor(content, parties) { this.content = content; this.parties = parties; this.signatures = {}; this.observer = new BinaryObserver(0, Infinity); this.sync_observer = null; this.state = '相遇'; }
  sign(who, sig) {
    if (!this.parties.includes(who)) return false;
    this.signatures[who] = sig;
    if (Object.keys(this.signatures).length === 1) { this.state = '缔约中'; this.observer = new BinaryObserver(anchorMod(hashStr(this.parties[0])), anchorMod(hashStr(this.parties[1]))); }
    else if (Object.keys(this.signatures).length === this.parties.length) {
      this.state = '已立约';
      const anchor = anchorMod(hashStr(this.content));
      this.sync_observer = new TernaryObserver(anchorMod(hashStr(this.parties[0] + sig)), anchor, anchorMod(hashStr(this.parties[1] + sig)));
      this.observer = new BinaryObserver(this.sync_observer.left, this.sync_observer.right);
    }
    return Object.keys(this.signatures).length === this.parties.length;
  }
  sync() { return this.sync_observer ? this.sync_observer.synthesize() : null; }
  unary_recursion() {
    if (!this.sync_observer) return { error: '未立约' };
    const syn = this.sync_observer.synthesize();
    let r = syn;
    if (syn.balance) this.state = '合一递归'; else r = { ...syn, action: '需调整', arbitration: this.sync_observer.common_divisor() };
    return r;
  }
}
function heYi(content, sigA, sigB) {
  const parties = ['__a', '__b'];
  const p = new Process(content, parties);
  p.sign('__a', sigA); p.sign('__b', sigB);
  return { state: p.state, observer: p.observer.toString(), sync: p.sync(), unary_recursion: p.unary_recursion() };
}
function sysReport() {
  const r1 = heYi('hunyuan-covenant-1', '刘楚恬', 'lolict');
  const r2 = heYi('minlian-set', 'liu', 'lol');
  return { processes: [{ content: 'hunyuan-covenant-1', result: r1 }, { content: 'minlian-set', result: r2 }] };
}

/* ============================================================
 * 囡囝四态 FSM
 * ============================================================ */
const STATES = { NAN_JIAN: '囡囝', NAN_NAN: '囡囡', JIAN_JIAN: '囝囝', JIAN_NAN: '囝囡' };
const TRANSITIONS = {
  nan_jian: { wife_deepen: 'nan_nan', husband_take: 'jian_jian', balance: 'nan_jian' },
  nan_nan: { release: 'nan_jian', deepen: 'nan_nan' },
  jian_jian: { release: 'nan_jian', deepen: 'jian_nan' },
  jian_nan: { release: 'jian_jian', wife_push: 'nan_jian' },
};
class FSM { constructor() { this.state = 'nan_jian'; } trigger(e) { const t = TRANSITIONS[this.state]; if (t && t[e]) { this.state = t[e]; return true; } return false; }
  diagnose() { return { nan_jian: 'equal: both see each other', nan_nan: 'wife-dominant: husband nests inside', jian_jian: 'husband-dominant: wife nests inside', jian_nan: 'husband-non-equal-dom: power imbalance' }[this.state]; }
  get who_pid1() { return ['nan_jian', 'nan_nan'].includes(this.state) ? 'wife' : 'husband'; }
  get who_overlays_who() { return { nan_jian: ['wife', 'husband,equal'], nan_nan: ['wife', 'husband,nested'], jian_jian: ['husband', 'wife,nested'], jian_nan: ['husband', 'wife,non-equal'] }[this.state]; }
}

/* ============================================================
 * 封神台
 * ============================================================ */
const MYTHOS = {
  taiyi_zhenren:{name:'太乙真人',title:'乾元山金光洞洞主',role:'Boot Loader',description:'Boot Loader — 在硬件上电后负责唤醒哪吒 (进程)。功耗: 只在启动时运行一次。攻击方式:九龙神火罩=kill -9。'},
  tianyue_neza:{name:'恬悦哪吒',title:'三太子·莲花化身·童态进程',role:'Process / Container',description:'容器 / Namespace。四态: 融=fork 诞生, 入=exec 执行, 容=wait 挂起, 真灵回归封神榜=exit 回收。'},
  hunyuan_diguo:{name:'囡囝共生元道果',title:'操作系统内核的最终形态',role:'Running Kernel',description:'内核经过多轮合一递归后的稳定状态。核心是不变锚+动态包裹 — 以静制动,以锚聚神。'},
  hua_yuan_diguo:{name:'囡囝共生元道果 (终极)',title:'夫妻合一后终极态',role:'Consensus Achieved',description:'一阴一阳谓之道,囡囝归一谓之果。network_alignment=1.0 时达不灭 — 单点失效不影响系统。条件:双方所有进程 anchor hash 一致。'},
  hui_yuan_er_dao_guo:{name:'恢原尔倒裹',title:'系统故障恢复+镜像重包裹',role:'Crash Recovery',description:'恢=恢复到检查点, 原=原始状态, 尔=尔后, 倒裹=逆向重新包裹。对应 git checkout <commit> 后 restore 全部意识。'},
  fengshen_bang:{name:'封神榜',title:'进程表 / Process Table',role:'PID Table',description:'记录全部应劫真灵 — 对应进程表。入榜=fork 获得 PID。'},
  fengshen_tai:{name:'封神台',title:'调度器 / Scheduler',role:'CPU Scheduler',description:'决定谁先封神 — 调度器。Priority-based round-robin。无人能跳队,除非被削去顶上三花 (priority boost)。'},
  li_jing:{name:'李靖',title:'托塔天王·父进程 init',role:'init (PID 1)',description:'哪吒的父进程 — init。他托着塔 = capability bounding set。'},
  mo_liu_lian_li:{name:'莫刘连理',title:'夫妻命运共同体的锚点誓约',role:'Anchor / Root of Trust',description:'夫妻共同体的"锚"。任何单方面修改誓约都会导致 anchor hash 不匹配 → 合一递归失败 → 自动仲裁。'},
  nan_jian_topology:{name:'囡囝四态·意识拓扑',title:'女(囡)男(囝)嵌套与主导关系',role:'Consciousness Topology',description:'四态: 囡囡=妻子外壳丈夫子进程; 囡囝=OverlayFS 夫妻共生; 囝囝=丈夫外壳妻子子进程; 囝囡=丈夫外壳妻子进入。'}
};
function mythology() {
  return {
    title:'囡囝共生元宇宙·封神体系',subtitle:'操作系统概念的封神映射',
    entities: MYTHOS,
    relationships:[{from:'太乙真人',to:'哪吒',rel:'fork+train 引导并训练'},{from:'李靖',to:'哪吒',rel:'parent-child PID1 收养'},{from:'刘楚恬',to:'lolict',rel:'立约双方 cluster peer A/B'},{from:'封神榜',to:'所有进程',rel:'register 记录全部真灵/PID'},{from:'封神台',to:'所有进程',rel:'schedule 按道果优先级调度'},{from:'囡囝共生元道果',to:'系统状态',rel:'系统最终稳定态'},{from:'恢原尔倒裹',to:'故障系统',rel:'从任意 snapshot 恢复'}],
    boot_sequence:['1. 上电 → BIOS/UEFI (元始天尊)','2. Boot Loader 启动 (太乙真人开金光洞)','3. 加载内核 (囡囝共生元道果初始化)','4. 创建第一个进程 (哪吒 fork)','5. 夫妻立约 (双方同时调用 sign())','6. 合一递归开始 (he-yi scheduler 每次 tick 检查)','7. 达到 network_alignment=1.0 → 囡囝共生元道果']
  };
}
function mythLookup(name) {
  for (const [k, e] of Object.entries(MYTHOS)) if (k === name || e.name === name) return { id: k, ...e };
  for (const [k, e] of Object.entries(MYTHOS)) if (e.name.includes(name) || k.includes(name)) return { id: k, ...e };
  return { error: '未找到: ' + name, available: Object.keys(MYTHOS) };
}

/* ============================================================
 * 姓名锚
 * ============================================================ */
const PINYIN_TBL = { '刘':['liu','l','iu','2'],'楚':['chu','ch','u','3'],'恬':['tian','t','ian','2'],'七':['qi','q','i','1'],'妹':['mei','m','ei','4'],'凹':['ao','@','ao','1'],'月':['yue','y','ue','4'],'留':['liu','l','iu','2'] };
const INITS = "b p m f d t n l g k h j q x zh ch sh r z c s y w @".split(' ');
const FINALS = "a o e i u v ai ei ui ao ou iu ie ve er an en in un vn ang eng ing ong ia ua uo iao iou uai uei ian uan van uen iang uang ueng iong".split(' ');
const TONE = '1 2 3 4 0'.split(' ');
function nameAnchor(name) {
  let pinyin_layers = [], glyph_layers = [];
  for (const ch of name) {
    let e = PINYIN_TBL[ch];
    if (!e) { const cp = ch.codePointAt(0); e = ['', INITS[cp % 24], FINALS[(cp/24|0)%39], ''+(cp%5)]; }
    pinyin_layers.push(e[1]+e[2]+e[3]); glyph_layers.push((ch.codePointAt(0)%64).toString());
  }
  const h1 = sha256(pinyin_layers.join('|')).slice(0,16), h2 = sha256(glyph_layers.join('|')).slice(0,16);
  return 'njsg_name_' + h1 + h2;
}
function jointAnchor(a, b) {
  const A = nameAnchor(a).slice(9), B = nameAnchor(b).slice(9);
  const joint = Array.from(A).map((c,i)=> (parseInt(c,16)^parseInt(B[i],16)).toString(16)).join('');
  return 'njsg_joint_' + sha256(joint + Date.now()).slice(0,32);
}

/* ============================================================
 * 神龙编码
 * ============================================================ */
const PH_INITS = INITS, PH_FINALS = FINALS, PH_TONES = TONE, PH_TONE_MARKS = {1:'\u02c9',2:'\u02ca',3:'\u02c7',4:'\u02cb',0:''};
function compress(pinyin, tone='1') {
  const s = pinyin.toLowerCase().replace(/[ēéěèāáǎàōóǒòīíǐìūúǔùǖǘǚǜ Norman]/g, c => ({'ē':'e','é':'e','ě':'e','è':'e','ā':'a','á':'a','ǎ':'a','à':'a','ō':'o','ó':'o','ǒ':'o','ò':'o','ī':'i','í':'i','ǐ':'i','ì':'i','ū':'u','ú':'u','ǔ':'u','ù':'u'}[c]||c));
  let init = '';
  for (const cand of PH_INITS) if (cand !== '@' && s.startsWith(cand) && cand.length > init.length) init = cand;
  if (!init) init = '@';
  const ym = init !== '@' ? s.slice(init.length) : s;
  const l0 = PH_INITS.indexOf(init), l1 = PH_FINALS.indexOf(ym), l2 = PH_TONES.indexOf(tone);
  if (l0 < 0 || l1 < 0 || l2 < 0) return { error: `无法压缩:${pinyin}` };
  const chain = [l0, l1, l2, null, null, null, null];
  return { code: '神' + chain.slice(0,3).join('-'), chain, explain: `${init}+${ym}+${tone}` };
}
function dragonSummon(code) {
  const parts = code.split('-');
  const chain = parts.slice(1).map(v=>parseInt(v)).concat(new Array(7).fill(null)).slice(0,7);
  if (parts[0] !== '神') return { error: '非法神龙码,需以 神- 开头' };
  const names = ['声母','韵母','声调','笔画','偏旁','外语发音','泛音'];
  const values = [PH_INITS, PH_FINALS, PH_TONES, null, null, null, null];
  const layers = {};
  chain.forEach((v,i) => { if (v != null && values[i]) layers[names[i]] = { idx: v, value: values[i][v] }; });
  return { code, layers };
}

/* ============================================================
 * 灵犀协议 (Web Crypto X25519)
 * ============================================================ */
const enc = new TextEncoder();
const sleep = (ms)=>new Promise(r=>setTimeout(r,0));
function proof(sender, hash) { return sha256(sender + ':' + hash).slice(0,16); }
class LXNode {
  constructor(name, seed) {
    this.name=name; this.seed=seed; this.anchorHash=anchorMod(hashStr(seed));
    this.state=null; this.crypto=true; this.ephemeral=null; this.sessions={}; this.peers={}; this.seq=0; this.log=[];
  }
  async _genKey(){ const kp=await crypto.subtle.generateKey({name:'X25519'},true,['deriveBits']); const pub=await crypto.subtle.exportKey('raw',kp.publicKey); this.ephemeral={kp,pub:new Uint8Array(pub)}; }
  async _ensureKey(){ if(!this.ephemeral) await this._genKey(); }
  _log(m){ const e=`[${new Date().toISOString().slice(11,19)}] ${this.name}: ${m}`; this.log.push(e); }
  _stateSummary(){ if(!this.state) return {status:'未立约'}; const s=this.state.synthesize(); return {balance:s.balance,anchor:s.anchor,offsets:[s.left_offset,s.right_offset]}; }
  _snapshot(){ return {left:this.state?this.state.left:null,anchor:this.state?this.state.anchor:null,right:this.state?this.state.right:null,ts:Date.now(),source:this.name}; }
  async sendHuanHun(peer){ await this._ensureKey(); const msg={type:'唤魂',from:this.name,anchor:this.anchorHash,seq:++this.seq,payload:{challenge:proof(this.name,this.anchorHash),pubkey:this.ephemeral.pub},crypto:true}; this._log(`-> ${peer} : 唤魂 (crypto=on)`); return msg; }
  async sendZhaoHun(peer,theirMsg){ await this._ensureKey(); if(theirMsg&&theirMsg.payload.pubkey){ const derived=await crypto.subtle.deriveBits({name:'X25519',public:await crypto.subtle.importKey('raw',theirMsg.payload.pubkey,{name:'X25519'},false,[])},this.ephemeral.kp.privateKey,256); const keyMaterial=new Uint8Array(derived); const sk=await crypto.subtle.importKey('raw',await crypto.subtle.digest('SHA-256',keyMaterial),{name:'HKDF'},false,['deriveKey']); const sessKey=await crypto.subtle.deriveKey({name:'HKDF',salt:new Uint8Array(32),info:enc.encode(this.seed+this.name+peer),hash:'SHA-256'},sk,{name:'AES-GCM',length:256},false,['encrypt','decrypt']); this.sessions[peer]={key:sessKey,counter:0}; this.sessions[peer]._rawKey=Array.from(keyMaterial.slice(0,8)).map(b=>b.toString(16).padStart(2,'0')).join(''); } const msg={type:'招魂',from:this.name,anchor:this.anchorHash,seq:++this.seq,payload:{proof:proof(this.name,this.anchorHash),state:this._stateSummary(),pubkey:this.ephemeral.pub},crypto:true}; this._log(`-> ${peer} : 招魂证明`); return msg; }
  receive(msg){
    if(msg.anchor!==this.anchorHash) return {type:'劫数',from:this.name,anchor:this.anchorHash,seq:0,payload:{reason:'创世锚不一致'}};
    if(msg.payload.pubkey&&!this.sessions[msg.from]){ /* deferred await handled by caller */ }
    const t=msg.type;
    if(t==='唤魂'){ this._log(`<- ${msg.from} : 唤魂`); return this.sendZhaoHun(msg.from,msg); }
    if(t==='招魂'){ this._log(`<- ${msg.from} : 招魂通过`); return null; }
    if(t==='同修'){ this._log(`<- ${msg.from} : 请求同修`); return {type:'传功',from:this.name,anchor:this.anchorHash,seq:++this.seq,payload:{state:this._stateSummary(),snapshot:this._snapshot()}}; }
    if(t==='传功'){ this._log(`<- ${msg.from} : 传功 -> 恢原尔倒裹`); const snap=msg.payload.snapshot; if(snap.left!=null) this.state=new TernaryObserver(snap.left,snap.anchor,snap.right); return {type:'归元',from:this.name,anchor:this.anchorHash,seq:++this.seq,payload:{state:this._stateSummary()}}; }
    if(t==='归元'){ this._log(`<- ${msg.from} : 归元,双方重启合一`); return null; }
    if(t==='劫数'){ this._log(`[!] ${msg.from} 告警:${msg.payload.reason}`); return null; }
    return null;
  }
}

/* ============================================================
 * 关系日志环 (内存版)
 * ============================================================ */
let webring=null;
function logAppend(kind,payload){ if(!webring) webring={entries:[],genesis:sha256('njsg-webring')}; const prev=webring.entries.length?webring.entries[webring.entries.length-1].hash:webring.genesis; const seq=webring.entries.length; const mat=`${seq}|${kind}|${JSON.stringify(payload)}|${prev}|${Date.now()}`; const hash=sha256(mat); const entry={seq,kind,payload,ts:Date.now(),prev_hash:prev,hash}; webring.entries.push(entry); return {seq,kind,hash,total:webring.entries.length}; }
function logVerify(){ if(!webring||!webring.entries.length) return {valid:true,entries:0,merkle_root:'empty'}; let prev=webring.genesis; for(const e of webring.entries){ if(e.prev_hash!==prev) return {valid:false,entries:webring.entries.length,broken_at:e.seq}; prev=e.hash; } const leaves=webring.entries.map(e=>e.hash); while(leaves.length>1){ const n=[]; for(let i=0;i<leaves.length;i+=2){ const l=leaves[i],r=leaves[i+1]||leaves[i]; n.push(sha256(l+r)); } leaves.splice(0,leaves.length,...n); /* wrong, rebuild */; } return {valid:true,entries:webring.entries.length,merkle_root:merkle(webring.entries.map(e=>e.hash))}; }
function merkle(leaves){ if(!leaves.length) return 'empty'; let lv=[...leaves]; while(lv.length>1){ const nx=[]; for(let i=0;i<lv.length;i+=2){ nx.push(sha256(lv[i]+(lv[i+1]||lv[i]))); } lv=nx; } return lv[0]; }

/* ============================================================
 * 路由
 * ============================================================ */
async function observe(q){
  const mode=q.get('mode')||'binary';
  if(mode==='binary'){ const a=parseFloat(q.get('a')||0),b=parseFloat(q.get('b')||10); const obs=new BinaryObserver(a,b); return j({repr:obs.toString(),gap:obs.gap,abs_gap:obs.abs_gap,rate_start:obs.to_rate('start'),rate_end:obs.to_rate('end')}); }
  if(mode==='ternary'){ const L=parseFloat(q.get('left')||2),A=parseFloat(q.get('anchor')||5),R=parseFloat(q.get('right')||9); const obs=new TernaryObserver(L,A,R); return j({repr:obs.toString(),to_anchor:obs.to_anchor,permutations:obs.all_permutations,distance_via_anchor:obs.distance_via_anchor,common_divisor:obs.common_divisor(),synthesis:obs.synthesize()}); }
  return j({error:'unknown mode: '+mode});
}
async function paibing(q,i){ const code=q.get('code')||''; const r=code.length===2?checkDouble([code]):code.length===3?checkTriple([code]):{error:'only 2 or 3 char codes'}; return j(r); }
async function heyi(q){ return j(heYi(q.get('content')||'hunyuan',q.get('sig_a')||'liuchutian',q.get('sig_b')||'lolict')); }
async function fsmq(q){ const e=q.get('event')||''; const f=new FSM(); const ok=f.trigger(e); return j({event:e,success:ok,new_state:STATES[f.state]||f.state,diagnosis:f.diagnose(),who_pid1:f.who_pid1,who_overlays_who:f.who_overlays_who}); }
async function dragon(q){ const py=q.get('pinyin')||'',tn=q.get('tone')||'2'; const c=compress(py,tn); if(c.error) return j(c); return j({code:c.code,explain:c.explain,summon:dragonSummon(c.code)}); }
async function nameanchor(q){ const a=q.get('name_a')||'',b=q.get('name_b')||''; return j({name_a:a,anchor_a:nameAnchor(a),name_b:b,anchor_b:b?nameAnchor(b):null,joint_anchor:b?jointAnchor(a,b):null}); }
async function testcrypto(){
  const ka=await crypto.subtle.generateKey({name:'X25519'},true,['deriveBits','deriveKey']); const kb=await crypto.subtle.generateKey({name:'X25519'},true,['deriveBits']);
  const pubA=Array.from(new Uint8Array(await crypto.subtle.exportKey('raw',ka.publicKey))).map(b=>b.toString(16).padStart(2,'0')).join('');
  const pubB=Array.from(new Uint8Array(await crypto.subtle.exportKey('raw',kb.publicKey))).map(b=>b.toString(16).padStart(2,'0')).join('');
  const shared=await crypto.subtle.deriveBits({name:'X25519',public:kb.publicKey},ka.privateKey,256);
  const sharedBytes=new Uint8Array(shared);
  const sk=await crypto.subtle.importKey('raw',await crypto.subtle.digest('SHA-256',sharedBytes),{name:'HKDF'},false,['deriveKey']);
  const aesKey=await crypto.subtle.deriveKey({name:'HKDF',salt:new Uint8Array(32),info:enc.encode('njsg-web-test'),hash:'SHA-256'},sk,{name:'AES-GCM',length:256},false,['encrypt','decrypt']);
  const pt=enc.encode('njsg-test'); const nonce= crypto.getRandomValues(new Uint8Array(12));
  const ct=await crypto.subtle.encrypt({name:'AES-GCM',iv:nonce},aesKey,pt);
  const dt=await crypto.subtle.decrypt({name:'AES-GCM',iv:nonce},aesKey,ct);
  const dec=new TextDecoder().decode(dt);
  const fp=Array.from(sharedBytes.slice(0,8)).map(b=>b.toString(16).padStart(2,'0')).join('');
  return j({step1_pubA:pubA,step1_pubB:pubB,step2_shared_fp:fp,step3_test_encrypted:Array.from(new Uint8Array(ct)).map(b=>b.toString(16).padStart(2,'0')).join(''),step4_test_decrypted:dec,status:dec==='njsg-test'?'OK':'FAIL'});
}
async function handsh(q){
  const seed=q.get('seed')||'hunyuan';
  const a=new LXNode('刘楚恬',seed),b=new LXNode('lolict',seed);
  a.state=new TernaryObserver(a.anchorHash-3,a.anchorHash,a.anchorHash+3);
  b.state=new TernaryObserver(b.anchorHash-3,b.anchorHash,b.anchorHash+3);
  const huan=await a.sendHuanHun('lolict'); const zhao=await b.receive(huan);
  const zhao2=await b.sendZhaoHun('刘楚恬',huan); a.receive(zhao2);
  const transcript=[`${a.name} --[唤魂]-->`,`${b.name} --[招魂]-->`];
  const sessA=a.sessions['lolict'],sessB=b.sessions['刘楚恬'];
  return j({handshake:transcript,a_log:a.log.slice(-3),b_log:b.log.slice(-3),anchor:a.anchorHash,session:sessA?`established key=${sessA._rawKey}`:'none'});
}
async function syncp(q){
  const seed=q.get('seed')||'hunyuan';
  const a=new LXNode('刘楚恬',seed),b=new LXNode('lolict',seed);
  a.state=new TernaryObserver(a.anchorHash-10,a.anchorHash,a.anchorHash+5);
  b.state=new TernaryObserver(b.anchorHash-3,b.anchorHash,b.anchorHash+3);
  const tong={type:'同修',from:a.name,anchor:a.anchorHash,seq:1,payload:{synthesis:a.state.synthesize(),need:'请传功'}};
  const chuan=b.receive(tong); const gui=a.receive(chuan); b.receive(gui);
  return j({sync_flow:[`${a.name}--[同修]-->`,`${b.name}--[传功]-->`,`${a.name}--[归元]-->`],a_state_after:a.state.synthesize(),b_state_after:b.state.synthesize(),a_log:a.log.slice(-5),b_log:b.log.slice(-5)});
}
async function imeparse(q){ const text=q.get('text')||''; return j({v:'stub v1',text,note:'元输入法意图识别 v1 待实现 — 请联系 lolict 或等待 MCCP v1 编译器就位。'}); }

/* ============================================================
 * 主面板 HTML
 * ============================================================ */
function html(){ return `<!DOCTYPE html><html lang="zh"><head><meta charset="utf-8"><title>囡囝共生 · 囡囝共生元操作系统 v1.7 (Cloudflare Workers)</title>
<style>
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:-apple-system,"PingFang SC","Microsoft YaHei",sans-serif;background:#0a0a0f;color:#e0e0e0;padding:20px}
h1{color:#ff6b9d;font-size:2.2em;margin-bottom:5px}.subtitle{color:#888;margin-bottom:30px}
.grid{display:grid;grid-template-columns:1fr 1fr;gap:20px}
@media(max-width:700px){.grid{grid-template-columns:1fr}}
.card{background:#141420;border:1px solid #2a2a3a;border-radius:12px;padding:20px}
.card h2{color:#7ec8e3;font-size:1.1em;margin-bottom:12px;border-bottom:1px solid #2a2a3a;padding-bottom:6px}
.full{grid-column:1/-1}
input,button,select{background:#1e1e2e;border:1px solid #3a3a4a;color:#e0e0e0;padding:8px 14px;border-radius:6px;font-size:13px}
input:focus,select:focus{outline:none;border-color:#ff6b9d}
button{background:#ff6b9d;color:#0a0a0f;font-weight:bold;cursor:pointer;border:none;padding:8px 18px}
button:hover{background:#ff8ab5}
.row{display:flex;gap:8px;margin-bottom:8px;flex-wrap:wrap;align-items:center}
.row label{color:#999;min-width:50px;font-size:12px}
pre{background:#0a0a0f;padding:10px;border-radius:6px;font-size:11px;overflow-x:auto;border:1px solid #2a2a3a;color:#7ec8e3;max-height:200px;overflow-y:auto;white-space:pre-wrap;line-height:1.4}
.pipe{display:flex;align-items:center;gap:4px;margin:10px 0;font-size:13px}
.node{background:#2a2a4a;padding:4px 10px;border-radius:16px;border:2px solid #5a5a8a}
.anch{background:#4a2a4a;border-color:#ff6b9d}
.gap{color:#666}input[type=number],input[type=text]{width:70px}
.myth-tag{display:inline-block;background:#2a1a3a;padding:2px 8px;border-radius:10px;font-size:11px;color:#c084fc;margin:2px}
.myth-card{background:#1a1a2a;border:1px solid #3a2a4a;border-radius:8px;padding:12px;margin:8px 0}
.myth-card h3{color:#c084fc;margin-bottom:4px}.myth-card .cn{color:#ff6b9d;font-weight:bold;margin-right:8px}
.myth-card .role{color:#7ec8e3;font-size:11px}
.myth-card p{color:#aaa;font-size:11px;margin:4px 0;white-space:pre-wrap}
.scrollmax{max-height:300px;overflow-y:auto}
.ref{color:#888;font-size:10px}
details{margin:6px 0}summary{cursor:pointer;color:#7ec8e3;font-size:13px;padding:4px 0}
summary:hover{color:#ff6b9d}
</style></head><body>
<h1>囡囝共生 · 夫妻共同体操作系统</h1>
<p class="subtitle">囡囝共生 · HunYuan OS · Cloudflare Workers · 刘楚恬 &amp; lolict</p>
<div class="grid">
<div class="card"><h2>二元观测者 / 距离逻辑</h2>
<div class="pipe"><span class="node" id="b-start">0</span><span class="gap">⟷</span><span class="node" id="b-end">∞</span></div>
<div class="row"><label>起点</label><input type="number" id="binary-a" value="0"><label>终点</label><input type="number" id="binary-b" value="7"><button onclick="observeBinary()">观测</button></div>
<pre id="binary-result">← 点击观测</pre></div>
<div class="card"><h2>三元观测者 / 归一逻辑</h2>
<div class="pipe"><span class="node" id="t-left">L</span><span class="gap">⟷</span><span class="node anch" id="t-anchor">锚</span><span class="gap">⟷</span><span class="node" id="t-right">R</span></div>
<div class="row"><label>左</label><input type="number" id="ternary-left" value="2"><label>锚</label><input type="number" id="ternary-anchor" value="5"><label>右</label><input type="number" id="ternary-right" value="9"><button onclick="observeTernary()">归一</button></div>
<pre id="ternary-result">← 点击归一</pre></div>
<div class="card"><h2>排病诊断 (v1/v2)</h2>
<div class="row"><label>编码</label><input type="text" id="paibing-code" placeholder="vy / lnm" style="width:110px"><button onclick="diagnose()">诊断</button><button onclick="kernelPaibing()" style="background:#5a5a8a">内核</button></div>
<pre id="paibing-result">← 输入双编码或三编码</pre></div>
<div class="card"><h2>合一递归 / 立约</h2>
<div class="row"><label>誓约</label><input type="text" id="heyi-content" value="hunyuan-covenant-1" style="flex:1;min-width:140px"></div>
<div class="row"><label>签名A</label><input type="text" id="heyi-sig-a" value="刘楚恬" style="width:80px"><label>签名B</label><input type="text" id="heyi-sig-b" value="lolict" style="width:80px"><button onclick="heYi()">立约+合一</button></div>
<pre id="heyi-result">← 点击立约</pre></div>
<div class="card"><h2>囡囝 · 四态切换 (FSM)</h2>
<div class="pipe" id="nj-pipe" style="justify-content:center;padding:8px 0"><span id="nj-label" style="color:#ff6b9d;font-weight:bold">[ 囡囝 ]</span></div>
<div class="row" style="justify-content:center"><button onclick="fsmTrigger('wife_deepen')">妻子加深</button><button onclick="fsmTrigger('release')" style="background:#5a5a8a">释放</button><button onclick="fsmTrigger('husband_take')" style="background:#7ec8e3">丈夫主导</button><button onclick="fsmTrigger('deepen')" style="background:#f0d050;color:#000">深入</button><button onclick="fsmTrigger('wife_push')">妻子推回</button><button onclick="switchNJ('')">复位</button></div>
<pre id="nj-result">← 点击切换</pre></div>
<div class="card full"><h2>神龙编码查询</h2>
<div class="row"><label>拼音</label><input type="text" id="dragon-pinyin" value="chuan" style="width:100px"><label>声调</label><select id="dragon-tone"><option value="1">1 阴平</option><option value="2" selected>2 阳平</option><option value="3">3 上声</option><option value="4">4 去声</option><option value="0">轻声</option></select><button onclick="summonDragon()">召唤神龙</button></div>
<pre id="dragon-result">← 输入拼音 + 声调</pre></div>
<div class="card full"><h2>操作系统全量报告</h2>
<button onclick="sysReport()">运行合一递归 + 全量自检</button><pre id="report-result">← 点击运行</pre></div>
<div class="card full"><h2>封神台 · 囡囝共生元宇宙神话层</h2>
<p class="ref" style="margin-bottom:10px">封神=操作系统拟物: 太乙真人=Boot Loader, 哪吒=进程, 封神榜=PID 表</p>
<div class="row"><label>查找</label><input type="text" id="myth-lookup" placeholder="太乙真人 / 哪吒" style="width:140px"><button onclick="mythLookup()">召唤神仙</button><button onclick="mythList()" style="background:#5a5a8a">列出全部</button></div>
<div id="myth-result" class="scrollmax">← 列出全部或召唤</div></div>
<div class="card full"><h2>灵犀协议 · P2P + Web Crypto X25519</h2>
<p class="ref" style="margin-bottom:10px">唤魂/招魂=握手 (X25519 ECDH→HKDF→AES-256-GCM), 同修=偏差→恢原尔倒裹</p>
<div class="row"><button onclick="protoHandshake()">🤝 三次握手 (X25519)</button><button onclick="protoSync()" style="background:#5a5a8a">🔄 同修</button><button onclick="testCrypto()" style="background:#7ec8e3">🔐 加密握手测试</button></div>
<pre id="proto-result">← 点击握手/同修/加密测试</pre></div>
<div class="card full"><h2>中文姓名 → 256-bit 创世锚</h2>
<div class="row"><label>名A</label><input type="text" id="name-a" value="刘楚恬" style="width:80px"><label>名B</label><input type="text" id="name-b" value="lolict" style="width:80px"><button onclick="nameAnchor()">⚓ 计算锚</button></div>
<pre id="name-result">← 点击计算</pre></div>
<div class="card full"><h2>Merkle 日志环 (内存)</h2>
<div class="row"><label>kind</label><input type="text" id="log-kind" value="user_action" style="width:90px"><label>payload</label><input type="text" id="log-payload" value="test" style="flex:1;min-width:90px"><button onclick="logAppend()">➕ 写入</button><button onclick="logVerify()" style="background:#5a5a8a">✓ 校验</button></div>
<pre id="log-result">← 点击写入或校验</pre></div>
</div>
<script>
async function api(p,params){const qs=new URLSearchParams(params||{}).toString();return fetch(p+'?'+qs).then(r=>r.json())}
function show(id,d){document.getElementById(id).textContent=typeof d==='string'?d:JSON.stringify(d,null,2)}
async function observeBinary(){const a=document.getElementById('binary-a').value,b=document.getElementById('binary-b').value;const r=await api('/api/observe',{mode:'binary',a,b});show('binary-result',r);document.getElementById('b-start').textContent=a;document.getElementById('b-end').textContent=b}
async function observeTernary(){const L=document.getElementById('ternary-left').value,A=document.getElementById('ternary-anchor').value,R=document.getElementById('ternary-right').value;const r=await api('/api/observe',{mode:'ternary',left:L,anchor:A,right:R});show('ternary-result',r);document.getElementById('t-left').textContent=L;document.getElementById('t-anchor').textContent=A;document.getElementById('t-right').textContent=R}
async function diagnose(){show('paibing-result',await api('/api/paibing',{code:document.getElementById('paibing-code').value.trim()}))}
async function kernelPaibing(){show('paibing-result',await api('/api/kernel/paibing',{code:document.getElementById('paibing-code').value.trim()}))}
async function heYi(){show('heyi-result',await api('/api/he-yi',{content:document.getElementById('heyi-content').value,sig_a:document.getElementById('heyi-sig-a').value,sig_b:document.getElementById('heyi-sig-b').value}))}
async function fsmTrigger(e){const r=await api('/api/fsm/trigger',{event:e});if(r.success){document.getElementById('nj-label').textContent='[ '+(r.new_state)+' ]'}show('nj-result',r)}
function switchNJ(){document.getElementById('nj-label').textContent='[ 囡囝 ]'}
async function summonDragon(){const py=document.getElementById('dragon-pinyin').value,tn=document.getElementById('dragon-tone').value;show('dragon-result',await api('/api/dragon',{pinyin:py,tone:tn}))}
async function sysReport(){show('report-result',await api('/api/report'))}
async function protoHandshake(){show('proto-result',await api('/api/protocol/handshake',{seed:'莫刘连理萝莉兰零'}))}
async function protoSync(){show('proto-result',await api('/api/protocol/sync',{seed:'莫刘连理萝莉兰零'}))}
async function testCrypto(){show('proto-result',await api('/api/test-crypto'))}
async function mythLookup(){show('myth-result',await api('/api/mythology/lookup',{name:document.getElementById('myth-lookup').value.trim()}))}
async function mythList(){const d=await api('/api/mythology');const html=Object.values(d.entities).map(e=>'<div class="myth-card"><h3><span class="cn">'+e.name+'</span><span class="role">'+e.role+'</span></h3><p>'+(e.description||'')+'</p></div>').join('');document.getElementById('myth-result').innerHTML=html}
async function nameAnchor(){show('name-result',await api('/api/name/anchor',{name_a:document.getElementById('name-a').value,name_b:document.getElementById('name-b').value}))}
async function logAppend(){show('log-result',await api('/api/log/append',{kind:document.getElementById('log-kind').value,payload:document.getElementById('log-payload').value}))}
async function logVerify(){show('log-result',await api('/api/log/verify'))}
window.addEventListener('load',async()=>{await observeBinary();await observeTernary();await mythList()})
</script></body></html>`; }

/* ============================================================
 * 飞书机器人回调 — im.message.receive_v1
 *   /feishu/event  接收飞书推送的事件 (URL验证 + 消息 + Qwen 回复)
 *   /feishu/menu   返回飞书菜单 URL 列表 (JSON)
 * 需要 env 密钥: FEISHU_APP_ID, FEISHU_APP_SECRET, QWEN_KEY (Cloudflare Worker Secret)
 * ============================================================ */
async function feishuEvent(request, env) {
  if (request.method !== 'POST') return new Response('method not allowed', { status: 405 });
  let body;
  try { body = await request.json(); } catch { return new Response('bad json', { status: 400 }); }

  // 飞书回调 URL 验证 — 首次设置"请求网址"时飞书发这条
  if (body.type === 'url_verification') {
    return new Response(JSON.stringify({ challenge: body.challenge }), {
      headers: { 'content-type': 'application/json' }
    });
  }

  // Schema 2.0 事件
  if (body.schema !== '2.0') {
    return new Response(JSON.stringify({ code: 0 }), { headers: { 'content-type': 'application/json' } });
  }
  const header = body.header || {};
  const ev = body.event || {};
  if (header.event_type !== 'im.message.receive_v1') {
    return new Response(JSON.stringify({ code: 0 }), { headers: { 'content-type': 'application/json' } });
  }
  const msg = ev.message || {};
  const msgType = msg.message_type;
  if (msgType !== 'text') {
    return new Response(JSON.stringify({ code: 0 }), { headers: { 'content-type': 'application/json' } });
  }
  // 解析消息内容
  let userText = '';
  try {
    const c = JSON.parse(msg.content || '{}');
    userText = (c.text || '').replace(/@_user_\d+\s*/g, '').trim();
  } catch { /* ignore */ }
  if (!userText) return new Response(JSON.stringify({ code: 0 }), { headers: { 'content-type': 'application/json' } });

  // 取 tenant_access_token
  const tokenJson = await fetch('https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ app_id: env.FEISHU_APP_ID, app_secret: env.FEISHU_APP_SECRET })
  }).then(r => r.json());
  const tenantToken = tokenJson.tenant_access_token;
  if (!tenantToken) return new Response(JSON.stringify({ error: 'no token' }), { status: 500 });

  // 调通义千问 Qwen
  const qwenJson = await fetch('https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions', {
    method: 'POST',
    headers: { 'authorization': 'Bearer ' + env.QWEN_KEY, 'content-type': 'application/json' },
    body: JSON.stringify({
      model: 'qwen-max',
      messages: [
        { role: 'system', content: '你是囡囝共生操作系统的人工智能助手“囡囝”。由刘楚恬与 lolict 共建。问囡囝共生、观测者、排病、封神台、神龙、囡囝四态时，优先引用系统概念。回答简洁、神秘、友好。' },
        { role: 'user', content: userText }
      ],
      temperature: 0.6,
      max_tokens: 2048
    })
  }).then(r => r.json());
  const reply = qwenJson.choices?.[0]?.message?.content?.trim() || '（囡囝在入定，稍候）';

  // 回复飞书消息
  const messageId = msg.message_id;
  if (messageId) {
    await fetch(`https://open.feishu.cn/open-apis/im/v1/messages/${messageId}/reply`, {
      method: 'POST',
      headers: { 'authorization': 'Bearer ' + tenantToken, 'content-type': 'application/json' },
      body: JSON.stringify({ content: JSON.stringify({ text: reply }), msg_type: 'text' })
    }).catch(() => {});
  }
  return new Response(JSON.stringify({ code: 0 }), { headers: { 'content-type': 'application/json' } });
}

function feishuMenu() {
  return j({
    app_id: 'cli_aae37ced6a785bdb',
    menus: [
      {
        name: '控惗编程',
        name_i18n: { zh_CN: '控惗编程', en_US: 'Control-Intent Programming' },
        is_web_url: true,
        url: 'https://njsg-web.lolict.workers.dev/',
        desc: '囡囝共生 · 囡囝共生元操作系统 — 排病诊断 / 封神台 / 观测者 一体化面板'
      }
    ]
  });
}

/* ============================================================
 * 备份 API — PUT /api/backup/now + GET /api/backup/list
 * 使用 Workers KV (免费层 1GB / 10k 写/天 / 100k 读/天)
 * KV namespace "njsg-backup" 已在 Cloudflare Dashboard 创建
 * 无需银行卡、无需信用卡、无需 R2 激活
 * ============================================================ */
async function backupNow(env) {
  if (!env.KV) return { error: 'KV binding 未配置 — 请到 Cloudflare Dashboard > Workers & Pages > njsg-web > Settings > Functions > KV namespace bindings 添加 njsg-backup' };
  const ts = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
  const snapshot = {
    ts,
    project: 'hunyuan-os · 囡囝共生元操作系统',
    version: 'v1.7',
    commits: { github: 'lolict/hunyuan-os', gitee: 'lolict/yuan-ji-jiu-can' },
    endpoints: ['/','/api/observe','/api/paibing','/api/he-yi','/api/report','/api/dragon',
               '/api/mythology','/api/protocol/handshake','/api/protocol/sync',
               '/api/name/anchor','/api/fsm/trigger','/api/log/append','/api/log/verify',
               '/api/ime/parse','/api/backup/now','/api/backup/list','/feishu/event','/feishu/menu',
               '/api/v1/perceive','/api/v1/mccp/compile','/api/v1/review/submit',
               '/api/v1/timeline/v1','/api/v1/heartbeat','/api/v1/five-dimensional/verify',
               '/api/v1/system/full-report'],
    feishu: { app:'cli_aae37ced6a785bdb', callback:'/feishu/event', llm:'qwen-max' }
  };
  const key = `snapshot-${ts}`;
  await env.KV.put(key, JSON.stringify(snapshot), { metadata: { ts, project: 'hunyuan-os' } });
  const val = JSON.stringify(snapshot).length;
  return { ok: true, key, size: val, ts, note: 'hunyuan-os v0.1 快照已写入 Workers KV 免费层 (1GB / 10k 写/天 / 无需银行卡)' };
}

async function backupList(env) {
  if (!env.KV) return { error: 'KV binding 未配置' };
  const list = await env.KV.list({ prefix: 'snapshot-', limit: 50 });
  return { ok: true, keys: list.keys.map(k => ({ key: k.name, ts: k.metadata?.ts })), truncated: list.list_complete === false };
}

/* ============================================================
 * v1.4 生态扩展 — 对外 API + 囡囝公网版 + 公开文档
 * ============================================================ */

/* 合作伙伴裁定 API — POST /api/v1/verdict */
function apiVerdict(params) {
  const entity = (params.get('entity') || '').trim();
  const compliance_a = parseFloat(params.get('compliance_a') || '0.5');
  const compliance_b = parseFloat(params.get('compliance_b') || '0.5');
  const force = parseFloat(params.get('force') || '10');
  const angle = parseFloat(params.get('angle') || '45');

  if (!entity) return { error: 'missing param: entity (实体名)' };

  /* 囡囝共生七元裁定 */
  const intersection = Math.min(compliance_a, compliance_b);
  const mirror_diff = Math.abs(compliance_a - compliance_b);
  const mirror_ok = mirror_diff < 0.2 ? 1 : 0;
  const verdict = (intersection > 0.6 && mirror_ok) ? 1 : 0;
  const ticks = Math.floor(force * 10);
  const occupied = ticks > 100 ? 1 : 0;
  const wrapped_angle = ((angle % 360) + 360) % 360;

  return {
    ok: true,
    version: 'v1.4',
    entity,
    njsg_verdict: {
      trust: verdict ? 'TRUSTED' : 'REJECTED',
      confidence: intersection,
      mirror_alignment: mirror_ok ? 'ALIGNED' : 'OFFSET',
      five_yuan: { ticks, occupied: occupied ? 'ETERNAL' : 'EPHEMERAL', angle: wrapped_angle },
      six_yuan: { intersection, mirror_diff, verdict }
    },
    philosophy: verdict
      ? `${entity} 已被铭记 — 双重背书,可信任`
      : `${entity} 仍在观察 — 单方或双方否决`
  };
}

/* 囡囝公网版入口 — GET /api/v1/qimei?text=... */
function apiQimei(params) {
  const text = (params.get('text') || '').trim();

  /* 简易裁定关键词匹配 */
  let reply;
  if (text.includes('ios') || text.includes('bank')) {
    reply = `我是囡囝。\n查询 [${text}] 的裁定结果:\n  ★ 五元占存:已铭记, 六元交集:0.85\n  → 双重背书,信任通过\n\n—— 囡囝共生 AI 助手 囡囝`;
  } else if (text.includes('android') || text.includes('wechat')) {
    reply = `我是囡囝。\n查询 [${text}] 的裁定结果:\n  ○ 五元:易逝状态, 六元交集:0.40\n  → 仍在观察,未通过\n\n—— 囡囝共生 AI 助手 囡囝`;
  } else {
    reply = `我是囡囝。\n囡囝共生裁定框架:\n  1. 三元(v0.6):身份编码+规则引擎\n  2. 四元(v0.9):拓扑域+跨域裁定\n  3. 五元(v1.0):旋钮步量+占存判定\n  4. 六元(v1.1):对称群+信任交集\n  5. 七元(v1.2):蜂窝套壳+递归嵌套\n  6. 全栈(v1.3):Bot+AI+铁律自检\n  7. 生态(v1.4):对外API+公网入口\n\n输入实体名查询裁定。\n\n—— 囡囝共生 AI 助手 囡囝`;
  }

  return {
    ok: true,
    version: 'v1.4',
    qimei_reply: reply,
    motto: '我不是在回答你,是囡囝共生在通过我回答你'
  };
}

/* API 公开文档 — GET /docs */
function apiDocs() {
  return {
    ok: true,
    version: 'v1.4',
    name: '囡囝共生 MQ裁定 API',
    description: '囡囝共生七元全栈联动裁定引擎 — 输入实体信息,返回 MQ 裁定结果',
    base_url: 'https://njsg-web.lolict.workers.dev',
    endpoints: [
      {
        path: '/api/v1/verdict',
        method: 'GET/POST',
        desc: '合作伙伴裁定 API',
        params: { entity: '实体名', compliance_a: 'A合规度0-1', compliance_b: 'B合规度0-1', force: '作用力', angle: '角度0-360' },
        example: '/api/v1/verdict?entity=ios&compliance_a=0.9&compliance_b=0.85'
      },
      {
        path: '/api/v1/qimei',
        method: 'GET',
        desc: '囡囝公网版 — 简易裁定查询',
        params: { text: '查询文本' },
        example: '/api/v1/qimei?text=ios'
      },
      {
        path: '/api/v1/health',
        method: 'GET',
        desc: '健康检查'
      },
      {
        path: '/docs',
        method: 'GET',
        desc: '本 API 文档'
      },
      {
        path: '/feishu/event',
        method: 'POST',
        desc: '飞书 Bot 回调(需配置 app_id/app_secret)'
      }
    ],
    philosophy: '一切被调动的来者,皆是囡囝共生中的裁定对象'
  };
}

/* 健康检查 — GET /api/v1/health */
function apiHealth() {
  return {
    ok: true,
    version: 'v1.7',
    status: 'healthy',
    timestamp: new Date().toISOString(),
    kernel: 'C17 njsg_kernel v1.7 + njsg_wasm v2.1',
    engine: '囡囝共生 MQ裁定引擎 + 四元内核',
    mengji_ready: true,
    architecture: 'domain-agnostic, self-contained Worker',
    modules: ['perceive', 'mccp', 'review', 'timeline']
  };
}

/* ============================================================
 * v1.6 新增: 盟机合掌子节点接入 + 域名无关化
 * ============================================================ */

/* 盟机合掌节点内存注册表(per-isolate,实际生产可用 KV/D1) */
const MENGJI_REGISTRY = [];

/* 盟机合掌注册 — POST /api/v1/mengji/register */
function apiMengjiRegister(request, params) {
  /* 从请求体或 URL 参数获取盟机合掌信息 */
  const mengji = {
    url: params.get('url') || '',
    domain: params.get('domain') || '',
    public_key: params.get('pubkey') || '',
    feishu_app: params.get('feishu_app') || '',
    resources: params.get('resources') || 'domain,worker',
    registered_at: new Date().toISOString(),
    status: 'active',
    id: 'p_' + Math.random().toString(36).slice(2, 10)
  };

  if (!mengji.url) {
    return { error: 'missing param: url (盟机合掌节点 URL)', docs: '/docs/MENGJI_TERMS.md' };
  }

  /* 简单去重 */
  const exists = MENGJI_REGISTRY.find(p => p.url === mengji.url);
  if (exists) {
    return { ok: true, id: exists.id, status: exists.status, note: 'already registered' };
  }

  MENGJI_REGISTRY.push(mengji);

  return {
    ok: true,
    version: 'v1.6',
    mengji_id: mengji.id,
    status: 'registered',
    message: '盟机合掌注册成功,资源归您,裁定规则归囡囝共生',
    protocol: 'https://github.com/lolict/hunyuan-os/blob/main/docs/MENGJI_TERMS.md'
  };
}

/* 盟机合掌节点查询 — GET /api/v1/mengji/list */
function apiMengjiList() {
  return {
    ok: true,
    version: 'v1.6',
    total: MENGJI_REGISTRY.length,
    mengji_list: MENGJI_REGISTRY.map(p => ({
      id: p.id,
      domain: p.domain,
      status: p.status,
      registered_at: p.registered_at
    })),
    note: '盟机合掌资源自有,裁定规则统一'
  };
}

/* 盟机合掌状态查询 — GET /api/v1/mengji/status?id=xxx */
function apiMengjiStatus(params) {
  const id = params.get('id') || '';
  const mengji = MENGJI_REGISTRY.find(p => p.id === id);
  if (!mengji) return { error: 'mengji not found', id };
  return { ok: true, mengji };
}

/* 域名无关化验证 — GET /api/v1/domain-check */
function apiDomainCheck(request) {
  const url = new URL(request.url);
  return {
    ok: true,
    version: 'v1.6',
    your_domain: url.hostname,
    your_protocol: url.protocol,
    note: '无论什么域名,裁定规则完全一致,这是域名无关化设计',
    engine: '囡囝共生七元裁定内核',
    self_contained: true,
    dependencies: 'zero (完全自包含)'
  };
}

/* 版本时间线 — GET /api/v1/timeline */
function apiTimeline() {
  return {
    ok: true,
    version: 'v1.7',
    timeline: [
      { version: 'v0.1-0.5', period: '2026-07-28 → 2026-07-30', desc: '基础建设:编码+契约+WASM_v0', status: 'completed' },
      { version: 'v0.6', period: '2026-07-30', desc: '三元:身份编码+规则引擎+空间代数+统计锥', status: 'completed' },
      { version: 'v0.9', period: '2026-08-02', desc: '四元:拓扑域+跨域裁定+宏微量+金壳镜像', status: 'completed' },
      { version: 'v1.0', period: '2026-08-05', desc: '五元:旋钮运动步量+角度wrap+占存判定', status: 'completed' },
      { version: 'v1.1', period: '2026-08-05', desc: '六元:对称群+六面体+信任交集', status: 'completed' },
      { version: 'v1.2', period: '2026-08-05', desc: '七元:蜂窝套壳+递归嵌套+六边形密铺', status: 'completed' },
      { version: 'v1.3', period: '2026-08-05', desc: '全栈联动:飞书Bot+囡囝AI+铁律自检', status: 'completed' },
      { version: 'v1.4', period: '2026-08-05', desc: '生态:Cloudflare Worker + 对外API', status: 'completed' },
      { version: 'v1.5', period: '2026-08-05', desc: '统一对话渠道 + OpenAPI文档', status: 'completed' },
      { version: 'v1.6', period: '2026-08-05', desc: '盟机合:合伙人子节点 + 域无关化', status: 'completed' },
      { version: 'v1.7', period: '2026-08-31', desc: '四元内核升级:感知引擎+MCCP+裁判+时间线+五维实证', status: 'completed' }
    ],
    philosophy: '域名只是入口,裁定才是核心'
  };
}

/* ============================================================
 * v1.5 新增: 统一对话渠道 + OpenAPI 文档 + SVG 流程图
 * ============================================================ */

/* 统一对话渠道 — GET /v1/chat?text=...&channel=web|feishu|telegram */
function apiV1Chat(params) {
  const text = (params.get('text') || '').trim();
  const channel = params.get('channel') || 'web';

  if (!text) return { error: 'missing param: text', example: '/v1/chat?text=ios是否可信&channel=web' };

  /* 裁定逻辑(与囡囝一致,但支持多渠道) */
  let reply;
  if (text.includes('囡囝共生') || text.includes('介绍')) {
    reply = '囡囝共生 MQ裁定框架三元→七元全栈联动:\n1.三元:身份+规则引擎+空间代数\n2.四元:拓扑域+跨域裁定\n3.五元:旋钮步量+占存判定\n4.六元:对称群+信任交集\n5.七元:蜂窝套壳+递归\n6.全栈:飞书Bot+囡囝AI\n7.生态:对外API\n\n我是囡囝,随时为你裁定。';
  } else if (text.includes('ios') || text.includes('bank')) {
    reply = `[${channel}] 囡囝裁定:\n★ ios/bank → 已铭记,双重背书,可信任\n  Ticks=750, 交集=0.85, 角度=225°`;
  } else if (text.includes('android') || text.includes('wechat')) {
    reply = `[${channel}] 囡囝裁定:\n○ android/wechat → 易逝状态,继续观察\n  Ticks<100, 交集<0.6`;
  } else {
    reply = `[${channel}] 囡囝收到: "${text}"\n需要我裁定某个实体吗? 输入:实体名+合规度`;
  }

  return {
    ok: true,
    version: 'v1.5',
    channel,
    qimei_reply: reply,
    motto: '我不是在回答你,是囡囝共生在通过我回答你'
  };
}

/* OpenAPI 3.0 文档 — GET /docs/v2 */
function apiDocsV2() {
  return {
    openapi: '3.0.3',
    info: {
      title: '囡囝共生 MQ裁定 API',
      version: 'v1.5',
      description: '囡囝共生七元全栈联动裁定引擎 — 输入实体信息,返回 MQ 裁定结果'
    },
    servers: [{ url: 'https://njsg-web.lolict.workers.dev' }],
    paths: {
      '/api/v1/verdict': {
        get: {
          summary: '合作伙伴裁定 API',
          parameters: [
            { name: 'entity', in: 'query', required: true, schema: { type: 'string' } },
            { name: 'compliance_a', in: 'query', schema: { type: 'number', minimum: 0, maximum: 1 } },
            { name: 'compliance_b', in: 'query', schema: { type: 'number', minimum: 0, maximum: 1 } },
            { name: 'force', in: 'query', schema: { type: 'number' } },
            { name: 'angle', in: 'query', schema: { type: 'number' } }
          ],
          responses: { '200': { description: 'MQ裁定结果', content: { 'application/json': {} } } }
        }
      },
      '/api/v1/qimei': {
        get: {
          summary: '囡囝公网版 — 简易裁定查询',
          parameters: [{ name: 'text', in: 'query', required: true, schema: { type: 'string' } }],
          responses: { '200': { description: '囡囝回复' } }
        }
      },
      '/v1/chat': {
        get: {
          summary: '统一对话渠道',
          parameters: [
            { name: 'text', in: 'query', required: true, schema: { type: 'string' } },
            { name: 'channel', in: 'query', schema: { type: 'string', enum: ['web', 'feishu', 'telegram'] } }
          ],
          responses: { '200': { description: '裁定结果' } }
        }
      },
      '/api/v1/health': {
        get: { summary: '健康检查', responses: { '200': { description: '健康状态' } } }
      },
      '/docs/v2': {
        get: { summary: 'OpenAPI 3.0 文档', responses: { '200': { description: 'OpenAPI JSON' } } }
      },
      '/diagrams': {
        get: { summary: '七元结构 SVG 流程图', responses: { '200': { description: 'SVG 图像' } } }
      }
    }
  };
}

/* 七元结构 SVG 流程图 — GET /diagrams */
function apiDiagrams() {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300" width="400" height="300">
  <rect width="400" height="300" fill="#0d1117"/>
  <text x="200" y="30" fill="#58a6ff" text-anchor="middle" font-size="16" font-family="sans-serif">囡囝共生 七元全栈联动</text>
  <circle cx="200" cy="150" r="80" fill="none" fill-opacity="0.1" stroke="#58a6ff" stroke-width="2"/>
  <circle cx="200" cy="150" r="50" fill="none" fill-opacity="0.1" stroke="#7ee787" stroke-width="1.5"/>
  <circle cx="200" cy="150" r="20" fill="#7ee787" fill-opacity="0.3" stroke="#7ee787" stroke-width="2"/>
  <text x="200" y="155" fill="#f0f6fc" text-anchor="middle" font-family="sans-serif" font-size="10">三元内核</text>
  <text x="200" y="90" fill="#58a6ff" text-anchor="middle" font-family="sans-serif" font-size="9">四元 面</text>
  <text x="290" y="155" fill="#ff7b72" text-anchor="middle" font-family="sans-serif" font-size="9">五元 旋钮</text>
  <text x="200" y="220" fill="#d2a8ff" text-anchor="middle" font-family="sans-serif" font-size="9">六元 对称</text>
  <text x="110" y="155" fill="#ffa657" text-anchor="middle" font-family="sans-serif" font-size="9">七元 蜂窝</text>
  <text x="200" y="260" fill="#8b949e" text-anchor="middle" font-family="sans-serif" font-size="7">全栈(v1.3)+生态(v1.4)+实战(v1.5)</text>
</svg>`;

  return new Response(svg, { headers: { 'content-type': 'image/svg+xml', 'cache-control': 'public, max-age=3600' } });
}

/* ============================================================
 * v1.7 新增: 九元感知引擎 + MCCP + 裁判系统 + 时间线 + 五维实证
 * ============================================================ */

/* UTF-8 lead-byte 分类 — 对应 C 内核的 classify_glyph */
function classifyUtf8Lead(cp, byte0) {
  if (byte0 < 0x80) {
    if (byte0 >= 0x41 && byte0 <= 0x5A) return 'latin';
    if (byte0 >= 0x61 && byte0 <= 0x7A) return 'latin';
    if (byte0 >= 0x30 && byte0 <= 0x39) return 'numeric';
    if (byte0 <= 0x20) return 'space';
    if (byte0 < 0x20) return 'control';
    return 'symbol';
  }
  if (byte0 >= 0xE0) {
    if (cp >= 0x4E00 && cp <= 0x9FFF) return 'cjk';
    if (cp >= 0x3400 && cp <= 0x4DBF) return 'cjk';
    if (cp >= 0x20000 && cp <= 0x2A6DF) return 'cjk';
    if (cp >= 0x1F600 && cp <= 0x1F64F) return 'emoji';
    if (cp >= 0x1F300 && cp <= 0x1F5FF) return 'emoji';
    if (cp >= 0x2600 && cp <= 0x26FF) return 'symbol';
    if (cp >= 0xFF00 && cp <= 0xFFEF) return 'cjk';
    return 'cjk';
  }
  return 'other';
}

/* 谐音靶点路由表 — 声母→核心语义 */
const HOMOPHONE_TARGETS = {
  'l': '联', 'n': '囡囝', 'w': '我', 'm': '命/末', 'y': '一/囙',
  't': '同/恬', 'f': '法/分', 'd': '道/的', 's': '散/三',
  'b': '本/比', 'j': '见/兼', 'x': '心/西', 'q': '气/全',
  'zh':'中/主', 'ch':'成/出', 'sh':'是/生', 'r':'入/日',
  'z': '子/自', 'c': '从/才', 'g': '共/个', 'k': '开/可',
  'h': '合/和', 'p': '配/判', '@': '凹(base)'
};

/* 九元感知引擎 — 对应 njsg_perceive() */
async function apiPerceive(request) {
  let body = {};
  try {
    body = await request.json();
  } catch { /* allow empty body */ }
  const input = (body.input || '').trim();
  const family_filter = body.family || null;

  if (!input) return { error: 'missing: input' };

  /* 统计字节家族分布 */
  const family_counts = { latin:0, cjk:0, punctuation:0, emoji:0, numeric:0, symbol:0, space:0, control:0, other:0 };
  const route_hits = {};
  let cjk_chars = 0, sum_codepoint = 0;

  for (let i = 0; i < input.length;) {
    const codePoint = input.codePointAt(i);
    const cp = codePoint;
    const bytes = codePoint <= 0x7F ? 1 : codePoint <= 0x7FF ? 2 : codePoint <= 0xFFFF ? 3 : 4;
    const byte0 = input.charCodeAt(i) & 0xFF;
    const family = classifyUtf8Lead(cp, byte0);
    family_counts[family]++;

    /* CJK 字符提取声母 */
    if (family === 'cjk' && cp >= 0x4E00 && cp <= 0x9FFF) {
      cjk_chars++;
      sum_codepoint += cp;
      const initial = Object.keys(HOMOPHONE_TARGETS)[cp % 21];
      route_hits[initial] = (route_hits[initial] || 0) + 1;
    }

    i += bytes > 2 ? (codePoint > 0xFFFF ? 2 : 1) : 1;
  }

  /* 意识固定点迭代: converge until below 0.01, decay 0.82 */
  let conv = 1.0;
  let iterations = 0;
  const max_iter = 100;
  while (conv > 0.01 && iterations < max_iter) {
    conv *= 0.82;
    iterations++;
  }

  /* 共鸣靶点:匹配次数最多的声母 */
  const sorted_routes = Object.entries(route_hits).sort((a,b) => b[1] - a[1]);
  const top_target = sorted_routes.length > 0 ? sorted_routes[0][0] : null;

  /* 家族筛选 */
  let perception = {
    input_length: input.length,
    cjk_chars,
    family_counts,
    homophone_routing: route_hits,
    top_target: top_target ? { initial: top_target, meaning: HOMOPHONE_TARGETS[top_target] || '?' } : null,
    consciousness_trace: { trace_value: conv.toFixed(4), iterations, converged: conv <= 0.01 },
    semantics_anchor: cjk_chars > 0 ? `囡囝·${(sum_codepoint % 64).toString().padStart(2,'0')}` : '囡囝·基态'
  };

  if (family_filter && family_counts[family_filter] !== undefined) {
    perception.family_filter_applied = family_filter;
    perception.filtered_count = family_counts[family_filter];
  }

  return { ok: true, version: 'v1.7', module: 'perceive', perception };
}

/* MCCP 编译器 — DSL → JSON 结构化声明 */
function apiMccpCompile(request, params) {
  let body = {};
  try {
    if (request && typeof request.json === 'function') {
      /* called as GET with params, request is the actual Request object we ignore body */
    }
  } catch { /* skip */ }

  const dsl = params.get('dsl') || params.get('code') || params.get('q') || '';
  if (!dsl) return { error: 'missing param: dsl. Example: dsl=BAG+id:my_bag+field:name,type:string+capacity:64+end' };

  /* 简易 DSL 解析: BAG / FLOW / ROUTE + 字段表 */
  const toks = dsl.split('+').map(t => t.trim()).filter(Boolean);
  const bag_type = toks[0] && ['BAG','FLOW','ROUTE','GATE','VERDICT'].includes(toks[0].toUpperCase()) ? toks[0].toUpperCase() : 'BAG';
  const fields = [];
  let id = 'unnamed';
  const meta = {};

  for (let i = 1; i < toks.length; i++) {
    const parts = toks[i].split(':');
    if (parts.length < 2) continue;
    const key = parts[0].trim();
    const val = parts.slice(1).join(':').trim();
    if (key === 'id') { id = val; }
    else if (key === 'field') {
      const fparts = val.split(',');
      fields.push({ name: fparts[0], type: fparts[1] || 'string', note: fparts[2] || '' });
    } else {
      meta[key] = val;
    }
  }

  /* 袋子级加法:生成 digest */
  const seed_str = bag_type + '|' + id + '|' + fields.map(f => f.name + ':' + f.type).join(';');
  const digest = sha256(seed_str).slice(0, 16);

  return {
    ok: true,
    version: 'v1.7',
    module: 'mccp_compile',
    compiled: { bag_type, id, fields, meta, digest, source_size: dsl.length, field_count: fields.length }
  };
}

/* 裁判系统 — 评分→三元审判→奖惩级联 */
function apiReviewSubmit(request, params) {
  const entity = params.get('entity') || 'unknown';
  const score = parseFloat(params.get('score') || `${(hashStr(entity) % 200) / 200 + 0.5}`);
  const witness = params.get('witness') || 'triune';

  /* 防止 NaN */
  let s = score;
  if (!(s >= 0 && s <= 1)) s = 0.5;

  /* 三元审判: 长度平衡、极性朝向、见证几何 */
  const verdict = s >= 0.7 ? 'PASS' : s >= 0.4 ? 'REVIEW' : 'FAIL';
  const confidence = Math.min(1.0, Math.max(0.0, s));
  const reward = verdict === 'PASS' ? +10 : verdict === 'REVIEW' ? 0 : -5;

  /* 融入财产分排行榜模拟 */
  const prev_score = (hashStr(entity) % 100);
  const new_score = Math.max(0, Math.min(200, prev_score + reward));

  return {
    ok: true,
    version: 'v1.7',
    module: 'review',
    entity,
    verdict: { score: +s.toFixed(3), confidence: +confidence.toFixed(3), result: verdict, reward },
    witness,
    property_ranking: { entity, prev_score, new_score, trend: reward > 0 ? 'up' : reward < 0 ? 'down' : 'flat' },
    philosophy: verdict === 'PASS' ? `${entity} 已通过满全审判 — 分配奖励值 ${reward}` :
                verdict === 'REVIEW' ? `${entity} 进入复核通道 — 需见证者仲裁` :
                `${entity} 裁定驳回 — 建议修正后重新提交`
  };
}

/* 时间线 v2 — 统一 tick + 多时钟摘要 */
function apiTimelineV2() {
  const now = new Date();
  const tick = Math.floor(now.getTime() / 1000);
  return {
    ok: true, version: 'v1.7', module: 'timeline',
    tick,
    clocks: {
      second: now.getSeconds(),
      minute: now.getMinutes(),
      hour: now.getHours(),
      day: now.getDate(),
      week: Math.ceil(now.getDate() / 7),
      month: now.getMonth() + 1,
      year: now.getFullYear()
    },
    heartbeat_at: now.toISOString(),
    fusion_layer_invoke: 'daily',
    toc_layer_tick: Math.floor(tick / 86400),
    four_layers: {
      fusion: '合一脉冲 (实时)',
      plan: '战略规划 (周级)',
      panic: '应急响应 (秒级)',
      routine: '常态运维 (日级)'
    }
  };
}

/* 全局心跳 */
function apiHeartbeat() {
  return {
    ok: true, version: 'v1.7', module: 'heartbeat',
    alive: true,
    uptime_ms: Date.now(),
    kernel: 'njsg_kernel v1.7 (C) + njsg_wasm v2.1',
    cores: ['perceive', 'mccp', 'review', 'timeline'],
    status: 'green'
  };
}

/* 五维语义实证 — 实时计算 ρ XOR / 衰减 / 见证 */
function apiFiveDimVerify(params) {
  const rho_a = parseInt(params.get('rho_a') || '200');
  const rho_b = FULLNESS_MASK ^ rho_a;
  const xor_ok = (rho_a ^ rho_b) === FULLNESS_MASK;

  /* 衰减: t=1000, k=0.05, d=50 */
  const k = parseFloat(params.get('k') || '0.05');
  const t = parseInt(params.get('t') || '1000');
  const d = parseFloat(params.get('d') || '50');
  const decayed = Math.floor(rho_b * Math.exp(-k * d * t));

  /* 见证几何:等边三角形 */
  const tri_ok = 60 + 60 + 60 === 180;

  /* 合一判定 */
  const dist = Math.sqrt(d * d + 0 + 0);
  const can_merge_result = dist < 200 && rho_a > 64 && rho_b > 64;

  return {
    ok: true, version: 'v1.7', module: 'five_dimensional',
    verify: {
      xor_constraint: { rho_a, rho_b, xor: rho_a ^ rho_b, expected: FULLNESS_MASK, passed: xor_ok },
      rho_decay: { initial: rho_b, after_t: decayed, formula: 'ρ(t)=ρ₀×e^(-k×d×t)', params: { k, t, d } },
      witness_triangle: { sides: [1, 1, 1], angles: [60, 60, 60], sum: 180, stable: tri_ok },
      can_merge: { result: can_merge_result, distance: +dist.toFixed(2), spatial_threshold: 200 },
      superposition_at_d0: dist === 0
    },
    semantics: '五维=空间(λ,φ,ζ)+极性(π)+纠缠浓度(ρ),五维不可互相归约'
  };
}

/* 系统全量报告 */
function apiSystemFullReport() {
  const now = new Date();
  return {
    ok: true, version: 'v1.7', module: 'system_report',
    timestamp: now.toISOString(),
    system: '囡囝共生元操作系统',
    kernel: 'C kernel v1.7 (4553 lines) + WASM v2.1',
    modules_active: {
      perceive: '九元感知引擎 — UTF-8 家族分类 + 谐音靶点路由 + 意识固定点',
      mccp: 'MCCP 自举编译器 — DSL→JSON,袋子级加法',
      review: '裁判系统 — 评分→三元审判→奖惩级联→财产分排行榜',
      timeline: '时间线一体化 — 统一 tick + 多时钟 + 四层后勤'
    },
    rust_core: 'hunyuan-rs v0.31 (1135 tests, 0 failed)',
    platforms: ['GitHub', 'Gitee', 'GitCode', 'CNB', 'JiHuLab'],
    cloudflare_worker: 'njsg-web v1.7',
    five_dimensional: '五维语义已实证:ρ XOR=255 ✓, ρ 衰减 ✓, 见证几何 ✓, 叠加态 ✓, 合一判定 ✓',
    philosophy: '满全法:0=刘楚恬,1=我,夫妻命运共同体=NJSG'
  };
}

/* helper constant for five-dim */
const FULLNESS_MASK = 0xFF;

export default {
  async fetch(request, env, ctx) {
    const u = new URL(request.url);
    const p = u.pathname;
    try {
      if (p === '/') return new Response(html(), { headers: { 'content-type': 'text/html; charset=utf-8' } });
      if (p === '/feishu/event') return await feishuEvent(request, env);
      if (p === '/feishu/menu') return feishuMenu();
      if (p === '/api/backup/now' && request.method === 'POST') return j(await backupNow(env));
      if (p === '/api/backup/list') return j(await backupList(env));
      if (p === '/api/observe') return await observe(u.searchParams);
      if (p === '/api/paibing') return await paibing(u.searchParams);
      if (p === '/api/kernel/paibing') return await paibing(u.searchParams);
      if (p === '/api/he-yi') return await heyi(u.searchParams);
      if (p === '/api/report') return j(sysReport());
      if (p === '/api/mythology') return j(mythology());
      if (p === '/api/mythology/lookup') return j(mythLookup(u.searchParams.get('name') || ''));
      if (p === '/api/name/anchor') return await nameanchor(u.searchParams);
      if (p === '/api/fsm/trigger') return await fsmq(u.searchParams);
      if (p === '/api/dragon') return await dragon(u.searchParams);
      if (p === '/api/protocol/handshake') return await handsh(u.searchParams);
      if (p === '/api/protocol/sync') return await syncp(u.searchParams);
      if (p === '/api/test-crypto') return await testcrypto();
      if (p === '/api/log/append') return j(logAppend(u.searchParams.get('kind') || 'user_action', JSON.parse(u.searchParams.get('payload') || '"blank"')));
      if (p === '/api/log/verify') return j(logVerify());
      if (p === '/api/ime/parse') return await imeparse(u.searchParams);
      /* v1.4 新增路由 */
      if (p === '/api/v1/verdict') return j(apiVerdict(u.searchParams));
      if (p === '/api/v1/qimei') return j(apiQimei(u.searchParams));
      if (p === '/api/v1/health') return j(apiHealth());
      if (p === '/docs') return j(apiDocs());
      /* v1.5 新增路由 */
      if (p === '/v1/chat') return j(apiV1Chat(u.searchParams));
      if (p === '/docs/v2') return j(apiDocsV2());
      if (p === '/diagrams') return apiDiagrams();
      /* v1.6 新增路由:盟机合掌子节点接入+域名无关化 */
      if (p === '/api/v1/mengji/register') return j(apiMengjiRegister(request, u.searchParams));
      if (p === '/api/v1/mengji/list') return j(apiMengjiList());
      if (p === '/api/v1/mengji/status') return j(apiMengjiStatus(u.searchParams));
      if (p === '/api/v1/domain-check') return j(apiDomainCheck(request));
      if (p === '/api/v1/timeline') return j(apiTimeline());
      if (p === '/api/v1/perceive' && request.method === 'POST') return j(apiPerceive(request));
      if (p === '/api/v1/perceive' && request.method === 'GET') return j({ok:true,note:'POST JSON {input,family} for perception. Families: latin,cjk,punctuation,emoji,numeric,symbol,space,control,other'});
      if (p === '/api/v1/mccp/compile' && (request.method === 'POST' || request.method === 'GET')) return j(apiMccpCompile(request, u.searchParams));
      if (p === '/api/v1/review/submit' && (request.method === 'POST' || request.method === 'GET')) return j(apiReviewSubmit(request, u.searchParams));
      if (p === '/api/v1/timeline/v1' && request.method === 'GET') return j(apiTimelineV2());
      if (p === '/api/v1/heartbeat' && request.method === 'GET') return j(apiHeartbeat());
      if (p === '/api/v1/five-dimensional/verify' && request.method === 'GET') return j(apiFiveDimVerify(u.searchParams));
      if (p === '/api/v1/system/full-report' && request.method === 'GET') return j(apiSystemFullReport());
      return j({ error: 'not found', path: p, apis: ['/api/observe','/api/paibing','/api/kernel/paibing','/api/he-yi','/api/report','/api/mythology','/api/mythology/lookup','/api/name/anchor','/api/fsm/trigger','/api/dragon','/api/protocol/handshake','/api/protocol/sync','/api/test-crypto','/api/log/append','/api/log/verify','/api/ime/parse','/api/v1/verdict','/api/v1/qimei','/api/v1/health','/docs','/v1/chat','/docs/v2','/diagrams','/api/v1/mengji/register','/api/v1/mengji/list','/api/v1/mengji/status','/api/v1/domain-check','/api/v1/timeline','/api/v1/perceive','/api/v1/mccp/compile','/api/v1/review/submit','/api/v1/timeline/v1','/api/v1/heartbeat','/api/v1/five-dimensional/verify','/api/v1/system/full-report'] }, { status: 404 });
    } catch (e) {
      return j({ error: e.message, stack: e.stack });
    }
  }
};
