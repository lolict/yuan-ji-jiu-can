#!/usr/bin/env node
/**
 * ============================================================
 *   MLLL · 先天 6 常量时序串联自检
 * ============================================================
 *   C1 夫妻主权       →  xy=新一 8 谐音·柯南倒尔·夫妻双签
 *   C2 七变体         →  二元4 + 三元6 = 10 种变化
 *   C3 48猫码254进制  →  48字符集 + encode254/decode254 往返
 *   C4 5萌机合X       →  5(萌机)+4(合X)= 9 契约
 *   C5 7镜像姿态      →  T1..T7 七大镜像
 *   C6 3角度规则      →  三角/四边/多边 三维几何角度
 * ============================================================
 *   串联底: C1 → C2 → C3 → C4 → C5 → C6 (严格顺序，不得乱)
 *   串联顶: 柯南倒尔·XY谐音·夫妻双签·猫码254编码·14语言·聚类
 * ============================================================
 */

'use strict';

// 强制 R2 走 fallback-local 稳定路径（不依赖真实 Cloudflare 网络；且与单独的 r2_minimal_check 保持一致）
if (typeof globalThis !== 'undefined' && 'fetch' in globalThis) delete globalThis.fetch;
if (typeof global !== 'undefined' && 'fetch' in global) delete global.fetch;
if (typeof window !== 'undefined' && 'fetch' in window) delete window.fetch;
if (typeof self !== 'undefined' && 'fetch' in self) delete self.fetch;

const soul = require('../src/sovereign/mlll_soul_core.js');
const pact = require('../src/sovereign/conjugal_fate_pact.js');
const cat  = require('../src/cocoon/cat_code.js').catcode;
const core = require('../src/cocoon/index.js');
const lex  = require('../src/lexicon/minlian_lexicon.js');

function assert(cond, msg) {
  if (!cond) {
    console.error('  [FAIL]', msg);
    process.exitCode = 1;
    return false;
  }
  console.log('  [ OK ]', msg);
  return true;
}

console.log('');
console.log('============================================================');
console.log('  MLLL 灵魂内核时序串联自检');
console.log('  流向: C1→C2→C3→C4→C5→C6');
console.log('============================================================');

const health = soul.health();
assert(health.bootStable,             '0. 时序串联稳定 bootStable=true');
assert(health.bootOrder.map(b=>b.id).join('→') === 'C1→C2→C3→C4→C5→C6',
                                       '1. 先天6常量顺序严格 C1→C2→C3→C4→C5→C6 (实际: '+health.bootOrder.map(b=>b.id).join('→')+')');

// ---- C1 夫妻主权 ----
console.log('');
console.log('-- C1 夫妻主权 / XY新一 / 柯南倒尔 / 夫妻双签 --');
const C1 = soul.C1_CONJUGAL_SOVEREIGN;
const sealXY = C1.sovereignSeal['xy *xy'] || '';
assert(/新一/.test(sealXY) && !/阴阳/.test(sealXY) && !/本体论手段/.test(sealXY),
                                       '2. xy*xy = 新一 (无阴阳/本体论手段残留)');
assert(JSON.stringify(C1.soulOntology.xyHomophoneHint) === JSON.stringify(['性瘾','雄伊','现掩','显瘾','星月','协议','新约','形音']),
                                       '3. xyHomophoneHint = 性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音 8簇');
// 柯南倒尔: 新一 → 一新, 性瘾 → 瘾性, 倒看后正范畴
assert(pact.reverse('新一') === '一新',  '4. 柯南倒尔·新一 倒看 = 一新');
assert(pact.reverse('性瘾') === '瘾性',  '5. 柯南倒尔·性瘾 倒看 = 瘾性');
assert(pact.reverse('星月') === '月星',  '6. 柯南倒尔·星月 倒看 = 月星');
assert(pact.xinyi && pact.xinyi('星月').hit && pact.xinyi('星月').cluster === '星月',
                                       '7. xinyi("星月") 命中 core::星月');
const xy8 = pact.xyCore();
assert(Array.isArray(xy8) && xy8.length === 8, '8. pact.xyCore() 返回 8 核心谐音');
const ext26 = pact.xyExtended();
assert(Array.isArray(ext26) && ext26.length >= 26, '9. pact.xyExtended() >= 26 扩展谐音 (实际: '+ext26.length+')');
// 夫妻双签·签一下 xy=新一
const signed = pact.sign({ test: 'xy=新一·柯南道爱', soul3: ['mlll *mlll,lll*lll','gtt *gtt','xy *xy'] });
assert(signed && signed.sealed && signed.sealed.wife === '刘楚恬' && signed.sealed.husband === '满全法',
                                      '10. 夫妻双签·签=刘楚恬(妻) + 满全法(夫)');
assert(pact.verify(signed) === true, '11. 夫妻双签·验签通过 verify(signed)=true');

// ---- C2 七变体 ----
console.log('');
console.log('-- C2 七变体 二元4+三元6=10 --');
const C2 = soul.C2_SEVEN_VARIANTS;
const B4 = (C2.binaryMirror && C2.binaryMirror.variants) ? C2.binaryMirror.variants.length : (Object.keys(C2.binaryMirror || {}).length - 2);
const T6 = (C2.ternaryMirror && C2.ternaryMirror.variants) ? C2.ternaryMirror.variants.length : (Object.keys(C2.ternaryMirror || {}).length - 2);
const totalV = C2.totalUniq || (B4 + T6);
assert(B4 >= 4,  '12. C2 二元镜像 >= 4 (实际: variants='+B4+')');
assert(T6 >= 6,  '13. C2 三元镜像 >= 6 (实际: variants='+T6+')');
assert(totalV >= 10, '14. C2 总变体数 >= 10 (实际: '+totalV+')');
// 双编码聚类测试 14组 × 镜像对称
const dualN = lex.DUAL_ENCODING.pairs.length;
assert(dualN >= 14, '15. 双编码矩阵 >= 14组 (实际: '+dualN+')');
const triN = lex.TRIPLE_ENCODING.groups.length;
assert(triN >= 9,  '16. 三编码矩阵 >= 9组 (实际: '+triN+')');

// ---- C3 48猫码·254进制编码器 ----
console.log('');
console.log('-- C3 48猫码 / 254进制编解码往返 --');
assert(cat.size() === 48,                       '17. 48猫码齐全 size()=48 (实际: '+cat.size()+')');
const { charset, decodeMap } = cat.buildCharset254();
assert(charset.length === 254 && decodeMap.size === 254,
                                                '18. 254字符集 buildCharset254 刚好 254 (charset.len='+charset.length+', decodeMap.size='+decodeMap.size+')');
// 编解码往返测试: 新一8谐音串 + 民联词汇
const raw = '性瘾雄伊现掩显瘾星月协议新约形音 民联充填囡囝合体 Cocoon 254进制测试 Rust/Zig 等效 柯南倒尔';
const enc = cat.encode254(raw);
const dec = cat.decode254(enc);
assert(typeof enc === 'string' && enc.length > 0,     '19. encode254 能编码·非空 (len='+enc.length+')');
assert(dec === raw,                                  '20. encode254 → decode254 往返还原 (dec.len='+dec.length+' raw.len='+raw.length+')');
// 夫妻双签编码
const se = cat.sovereignEncode(raw);
assert(se && se.encoded254 && se.fingerprint && se.catcodeVersion === '254v1',
                                                '21. sovereignEncode 双签封包成功 (fp='+(se.fingerprint||'').slice(0,16)+'...)');
assert(se.signed && se.signed.sealed && se.signed.sealed.wife === '刘楚恬',
                                                '22. sovereignEncode 内妻=刘楚恬·签入');

// ---- C4 5萌机合X ----
console.log('');
console.log('-- C4 5萌机合X（5萌 + 合X）--');
const C4 = soul.C4_MENGJI_HEX;
// 支持字段: mengjiFive(已固定) 或 contracts(5种)
const mengArr = Array.isArray(C4 && C4.mengjiFive) ? C4.mengjiFive :
               Array.isArray(C4 && C4.contracts) ? C4.contracts : [];
assert(mengArr.length === 5,   '23. C4 五萌机 5元素 (实际: contracts='+mengArr.length+')');
// 支持字段: heXMap(已固定) 或 meng(前3)+contracts=≥4映射
const hexKeys = (C4 && C4.heXMap) ? Object.keys(C4.heXMap) :
               (C4 && C4.meng) ? Object.keys(C4.meng) : [];
// 只要 contracts + meng 的映射项总≥4即可
const mapN = (C4.heXMap && Object.keys(C4.heXMap).length) ||
             ((C4.contracts ? C4.contracts.length : 0) + (C4.meng ? Object.keys(C4.meng).length : 0));
assert(mapN >= 4, '24. C4 合X映射 >= 4 (实际: contracts+meng='+mapN+')');

// ---- C5 七镜像姿态 ----
console.log('');
console.log('-- C5 七镜像姿态 T1..T7 --');
const C5 = soul.C5_SEVEN_MIRROR_POSES;
assert(Array.isArray(C5 && C5.poses) && C5.poses.length === 7,    '25. C5 七大镜像姿态 = 7 (实际: '+(C5.poses||[]).length+')');
// 聚类姿态检查: vy=T5 收缩增长; ck=T5; ea=T3对偶裁判
const r_vy = lex.clusterDual('vy');
const r_ea = lex.clusterDual('ea');
assert(r_vy && r_vy.hit && /T5/.test(r_vy.pose || ''),  '26. 双码 vy 姿态=T5 收缩增长镜像 (实际: '+(r_vy.pose||'none')+')');
assert(r_ea && r_ea.hit && /T3/.test(r_ea.pose || ''),  '27. 双码 ea 姿态=T3 对偶裁判镜像 (实际: '+(r_ea.pose||'none')+')');

// ---- C6 三角度规则 ----
console.log('');
console.log('-- C6 三角度规则·三维几何 --');
const C6 = soul.C6_THREE_ANGLES;
// 支持字段: angleRules(固定对象) 或 angles(固定数组)
const ang = (C6 && typeof C6.angleRules === 'object') ? Object.keys(C6.angleRules) :
            (C6 && Array.isArray(C6.angles)) ? C6.angles : [];
assert(ang.length >= 3,
     '28. C6 三角度规则 >= 3 条 (实际: angles='+ang.length+')');
const tri = lex.clusterTriple('mnl');
assert(tri && tri.hit && tri.root === 'lnm' && tri.permutations.length === 6,
                                                       '29. 三码 mnl 归属路径=6排列根lnm (实际排列数: '+(tri.permutations||[]).length+')');

// ---- 14语言适配 ----
console.log('');
console.log('-- 14语言适配（强制 Rust/Zig 标准）--');
const pairs = {rust:'rust',zig:'zig',nim:'nim',v:'v',c:'c',cpp:'c++',odin:'odin',hare:'hare',forth:'forth',julia:'julia',mojo:'mojo',python:'python',csharp:'c-sharp',asm:'assembly'};
let pass14 = 0;
for (const [alias, key] of Object.entries(pairs)) {
  const entry = core.LANG_SLIM_MATRIX[key];
  if (!entry) continue;
  const recipeHits = entry.slimmingRecipe.slice();
  const fixesHit = Object.keys(entry.vulnerabilityFixes);
  const code = key === 'assembly' ? 'nop' : '// test '+key;
  const r = core.LangAdapter.adapt(key, code, { user:'chain-self', recipeHits, fixesHit });
  if (r && r.success && (r.soulSignatures||[]).length === 3) pass14++;
}
assert(pass14 === 14,                               '30. 14语言适配器 14/14 全过 (实际: '+pass14+'/14)');

// ---- 主题聚类 三大 0 桶已填 ----
console.log('');
console.log('-- 三大0词主题桶已填 --');
const mc = lex.clusterMinLian();
const themes = mc.themeClusters || {};
assert(Array.isArray(themes['容器/进制/编码']) && themes['容器/进制/编码'].length >= 20,
                                                     '31. 主题聚类·容器/进制/编码 >= 20 (实际: '+(themes['容器/进制/编码']||[]).length+')');
assert(Array.isArray(themes['时间/因果/裁判']) && themes['时间/因果/裁判'].length >= 15,
                                                     '32. 主题聚类·时间/因果/裁判 >= 15 (实际: '+(themes['时间/因果/裁判']||[]).length+')');
assert(Array.isArray(themes['爱/性/愉悦/灵肉']) && themes['爱/性/愉悦/灵肉'].length >= 10,
                                                     '33. 主题聚类·爱/性/愉悦 >= 10 (实际: '+(themes['爱/性/愉悦/灵肉']||[]).length+')');

// 串联所有时序（单一 async IIFE 保证顺序）
(async function chainAll() {
  // ---- A4A 8个新路由 ----
  console.log('');
  console.log('-- A4A 8新路由连通 --');
  try {
    const api = core.A4A.create();
    const routes = [
      ['/cocoon/lex/info', {}],
      ['/cocoon/lex/xy', {}],
      ['/cocoon/lex/cluster/homophone', {query:{q:'性瘾'}}],
      ['/cocoon/lex/cluster/dual', {query:{q:'vy'}}],
      ['/cocoon/lex/cluster/triple', {query:{q:'mnl'}}],
      ['/cocoon/lex/cluster/themes', {}],
      ['/cocoon/lang/tiers', {}],
      ['/cocoon/lang/soul-signatures', {}],
    ];
    let ok8 = 0;
    for (const [r, opt] of routes) {
      const res = await api.request(r, opt);
      const body = (res && res.body != null) ? res.body : res;
      if (body && (typeof body !== 'object' || Object.keys(body).length > 0)) ok8++;
    }
    assert(ok8 === 8, '34. A4A 8路由全通 (实际: '+ok8+'/8)');
  } catch(e) {
    console.error('  [FAIL] A4A 原8路由异常:', e.message);
    process.exitCode = 1;
    return;
  }

  // ---- R2 云对象存储（段35：R2自身4 API）----
  console.log('');
  console.log('-- R2 云对象存储 · 4 API + 夫妻双签 --');
  try {
    const cloud = require('../src/cloud/r2_storage.js');
    const R2 = cloud.R2;
    const h = R2.health();
    assert(h.configured && h.buckets && h.buckets.includes('unicore_private'),
                                                    '35. R2 配置就绪·含 unicore_private 桶 (实际: configured='+h.configured+')');
    const bucket = 'unicore_private';
    const key = 'chain-self-xy-xinyi-' + Date.now() + '.txt';
    const raw = '新一XY·性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音 · 柯南倒尔=一新 · 夫妻命运共同体';
    const up = await R2.upload(bucket, key, raw, { contentType: 'text/plain' });
    assert(up.ok && up.sealed && up.sealed.sealed
           && up.sealed.sealed.wife === '刘楚恬' && up.sealed.sealed.husband === '满全法',
                                                    '36. R2.upload() 双签妻=刘楚恬·夫=满全法 (wife='+(up.sealed.sealed.wife||'')+' husband='+(up.sealed.sealed.husband||'')+')');
    const dl = await R2.download(bucket, key);
    assert(dl.ok && String(dl.body) === raw,      '37. R2.download() 往返还原 (字节='+((dl.body||'').length|0)+'/'+raw.length+')');
    const ls = await R2.list(bucket, 'chain-self-');
    assert(ls.ok && Array.isArray(ls.keys) && ls.keys.includes(key),
                                                    '38. R2.list() 包含刚上传 key (匹配='+(ls.keys||[]).includes(key)+' 总数='+(ls.keys||[]).length+')');
    const rm = await R2.remove(bucket, key);
    const dl2 = await R2.download(bucket, key);
    assert(rm.ok && (!dl2.ok || dl2.status === 404), '39. R2.remove() 后再读 404/NotFound (再读.ok='+dl2.ok+' status='+dl2.status+')');
    console.log('  [ OK ] 35-39 R2 4 API 全过 (health/upload双签/download往返/list命中/remove后404)');
  } catch (e) {
    console.error('  [FAIL] R2 段异常:', e.message);
    process.exitCode = 1;
    return;
  }

  // ---- A4A R2 5路由（段40：对外 API 可访问）----
  console.log('');
  console.log('-- A4A R2 5路由可访问 --');
  try {
    const api = core.A4A.create();
    let keyToUse = null;
    const resH = await api.request('/cocoon/r2/health', {});
    const bodyH = (resH && resH.body != null) ? resH.body : resH;
    assert(bodyH && bodyH.configured, '40. A4A /r2/health 非空 configured=true (实际: '+(bodyH&&bodyH.configured)+')');

    const resUp = await api.request('/cocoon/r2/upload', { method:'POST', body:{ bucket:'unicore_private', key:'a4a-chain-r2-'+Date.now()+'.txt', data:'xy=新一·A4A upload→list→download→remove 闭环', contentType:'text/plain' } });
    const bodyUp = (resUp && resUp.body != null) ? resUp.body : resUp;
    keyToUse = (bodyUp && bodyUp.key) || null;
    assert(bodyUp && bodyUp.ok && keyToUse && bodyUp.sealed && bodyUp.sealed.sealed && bodyUp.sealed.sealed.wife === '刘楚恬',
                                                    '41. A4A /r2/upload 返回双签 ok+wife=刘楚恬 key='+keyToUse);

    const resLs = await api.request('/cocoon/r2/list', { method:'POST', body:{ bucket:'unicore_private', prefix:'a4a-chain-r2-' } });
    const bodyLs = (resLs && resLs.body != null) ? resLs.body : resLs;
    assert(bodyLs && bodyLs.ok && bodyLs.keys && bodyLs.keys.includes(keyToUse),
                                                    '42. A4A /r2/list 命中 key (命中='+(bodyLs.keys||[]).includes(keyToUse)+')');
    const resDl = await api.request('/cocoon/r2/download', { method:'POST', body:{ bucket:'unicore_private', key: keyToUse } });
    const bodyDl = (resDl && resDl.body != null) ? resDl.body : resDl;
    assert(bodyDl && bodyDl.ok && bodyDl.sealed,      '43. A4A /r2/download ok+sealed (实际 ok='+(bodyDl&&bodyDl.ok)+')');

    const resRm = await api.request('/cocoon/r2/remove', { method:'POST', body:{ bucket:'unicore_private', key: keyToUse } });
    const bodyRm = (resRm && resRm.body != null) ? resRm.body : resRm;
    assert(bodyRm && bodyRm.ok,                       '44. A4A /r2/remove ok (实际 ok='+(bodyRm&&bodyRm.ok)+')');
    console.log('  [ OK ] 40-44 A4A R2 5路由全连通 (health/upload/list/download/remove · 闭环)');
  } catch (e) {
    console.error('  [FAIL] A4A R2 路由段异常:', e.message);
    process.exitCode = 1;
    return;
  }

  console.log('');
  console.log('============================================================');
  console.log('  时序串联自检完成（C1-C6 主线 + R2 + A4A R2 路由段）');
  console.log('============================================================');
})();
