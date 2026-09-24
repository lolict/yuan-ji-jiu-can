/**
 * test_private.mjs — mqf-private.js 核心逻辑验证
 * 用 Node 直接运行: node cloudflare/test_private.mjs
 */

let pass = 0, fail = 0;
function ok(cond, label) { if (cond) { console.log('  ✅ ' + label); pass++; } else { console.log('  ❌ ' + label); fail++; } }

const MOUTH_ALPHABET = 'bpmfdtnlgkhjqxzcsywaeiouvrABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-';

function bagEncode({ presence = 1, witness = 1, polarity = 0, sheng = 0, yun = 0, tone = 0, slot = 0, value = 0, owner = 1, relation = 0, timing = 0, ack = 0 }) {
  const out = new Uint8Array(13);
  out[0] = (presence & 1) | ((witness & 1) << 1) | ((polarity & 3) << 2);
  out[1] = sheng & 0x1F;
  out[2] = yun & 0x3F;
  out[3] = ((tone & 7) << 3) | (slot & 7);
  out[4] = (value >> 12) & 0x3F;
  out[5] = (value >> 6) & 0x3F;
  out[6] = value & 0x3F;
  out[7] = owner & 0x07;
  out[8] = relation & 0xFF;
  out[9] = timing & 0xFF;
  out[10] = ack & 0xFF;
  let sum = 0; for (let i = 0; i < 11; i++) sum += out[i];
  out[11] = (sum >> 8) & 0xFF;
  out[12] = sum & 0xFF;
  return out;
}

function bagDecode(bytes) {
  if (bytes.length < 13) return null;
  return {
    presence: bytes[0] & 1, witness: (bytes[0] >> 1) & 1, polarity: (bytes[0] >> 2) & 3,
    sheng: bytes[1] & 0x1F, yun: bytes[2] & 0x3F, tone: (bytes[3] >> 3) & 7, slot: bytes[3] & 7,
    value: ((bytes[4] & 0x3F) << 12) | (bytes[5] << 6) | bytes[6],
    owner: bytes[7] & 7, relation: bytes[8], timing: bytes[9], ack: bytes[10],
  };
}

function bagVerify(b) {
  if (!b) return false;
  if (b.presence !== 1) return false;
  if (b.witness !== 1) return false;
  if (b.owner !== 0 && b.owner !== 1) return false;
  return true;
}

function bagToRadix64(bytes) {
  let s = '';
  for (let i = 0; i < bytes.length; i += 3) {
    const a = bytes[i], b = i + 1 < bytes.length ? bytes[i + 1] : 0, c = i + 2 < bytes.length ? bytes[i + 2] : 0;
    const n = (a << 16) | (b << 8) | c;
    s += MOUTH_ALPHABET[(n >> 18) & 0x3F] + MOUTH_ALPHABET[(n >> 12) & 0x3F] + MOUTH_ALPHABET[(n >> 6) & 0x3F] + MOUTH_ALPHABET[n & 0x3F];
  }
  return s;
}

const HOMOPHONE_ROUTES = {
  '7-1': { target: 0x01, note: 'l+1→WIFE' },
  '4-4': { target: 0x01, note: 'ch+u→WIFE' },
  '5-5': { target: 0x01, note: 't+5→WIFE' },
  '7-2': { target: 0x02, note: 'l+o→HUSBAND' },
};

function homophoneRoute(sheng, yun) {
  return HOMOPHONE_ROUTES[sheng + '-' + yun] || { target: 0x08, note: 'default' };
}

function executeOp(op, bag) {
  switch (op) {
    case 0x01: return { ok: true, label: '招魂', status: 'alive' };
    case 0x02: return { ok: true, label: '识', polarity: ['wife','husband','system'][bag.polarity] };
    case 0x0B: return { ok: true, label: '加', result: bag.value + 1 };
    case 0x13: return { ok: true, label: '排', verdict: bagVerify(bag) ? 'PASS' : 'DENY' };
    default:   return { ok: false, label: '未实现' };
  }
}

console.log('═══ mqf-private v1.5 核心逻辑测试 ═══');

// [1]
console.log('\n[1] 袋子编解码 (13 bytes)');
{
  const bag = bagEncode({ presence: 1, witness: 1, polarity: 2, sheng: 7, yun: 5, tone: 3, slot: 0x13, value: 42, owner: 1, relation: 0x01, timing: 0, ack: 0 });
  ok(bag.length === 13, '袋子大小 = 13 bytes');
  const d = bagDecode(bag);
  ok(d.sheng === 7, '声母编码一致');
  ok(d.yun === 5, '韵母编码一致');
  ok(d.tone === 3, '声调编码一致');
  ok(d.owner === 1, 'owner=我');
  ok(d.value === 42, 'value=42 通过 18-bit 编码');
  const r64 = bagToRadix64(bag);
  ok(r64.length === 20, '64进制编码: 13 bytes → 20 chars');
}

// [2]
console.log('\n[2] 三元见证');
{
  ok(bagVerify(bagDecode(bagEncode({ presence: 1, witness: 1, owner: 1 }))), '有效袋子: 通过');
  ok(!bagVerify(bagDecode(bagEncode({ presence: 0, witness: 1, owner: 1 }))), 'presence=0: 拒绝');
  ok(!bagVerify(bagDecode(bagEncode({ presence: 1, witness: 0, owner: 1 }))), 'witness=0: 拒绝');
  ok(!bagVerify(bagDecode(bagEncode({ presence: 1, witness: 1, owner: 5 }))), 'owner=5: 拒绝');
}

// [3]
console.log('\n[3] 谐音路由');
{
  ok(homophoneRoute(7, 1).target === 0x01, '声母7+韵母1 → WIFE');
  ok(homophoneRoute(9, 9).target === 0x08, '未知映射 → SYSTEM (default)');
}

// [4]
console.log('\n[4] 七元操作');
{
  const cb = bagDecode(bagEncode({ presence: 1, witness: 1, owner: 1, polarity: 2, slot: 0x01, value: 0 }));
  const r1 = executeOp(0x01, cb);
  ok(r1.ok && r1.status === 'alive', '招魂 → alive');
  const r2 = executeOp(0x02, cb);
  ok(r2.ok && r2.polarity === 'system', '识 → 解析极性');
  const r3 = executeOp(0x0B, bagDecode(bagEncode({ value: 100 })));
  ok(r3.result === 101, '加 → 100+1');
  const r4 = executeOp(0x13, cb);
  ok(r4.verdict === 'PASS', '排 → 有效袋子通过');
}

// [5]
console.log('\n[5] 三元握手');
{
  const hello = bagDecode(bagEncode({ presence: 1, witness: 0, value: 1 }));
  ok(hello.presence === 1, '发起方: presence=1');
  const ack = bagDecode(bagEncode({ presence: 1, witness: 1, value: 2 }));
  ok(ack.witness === 1, '接收方: witness=1');
  const est = bagDecode(bagEncode({ presence: 1, witness: 1, owner: 1, value: 3 }));
  ok(bagVerify(est), '发起方: anchor=一致 → ESTABLISHED');
}

console.log('\n═══ 结果: ' + pass + ' 通过 / ' + fail + ' 失败 ═══');
process.exit(fail > 0 ? 1 : 0);
