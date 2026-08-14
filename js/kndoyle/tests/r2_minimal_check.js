/**
 *  R2 最小化可用自检：
 *  1) 能 import
 *  2) health() 有字段
 *  3) 本地回退模式下 upload/download/list/remove 往返成立
 *  4) 每步都有夫妻双签 sealed{wife, husband, fingerprint}
 */
'use strict';
const assert = require('assert');

// 让运行环境假装没有 fetch，保证走 fallback-local 稳定路径
if (typeof globalThis !== 'undefined') delete globalThis.fetch;
if (typeof global !== 'undefined') delete global.fetch;
if (typeof window !== 'undefined') delete window.fetch;
if (typeof self !== 'undefined') delete self.fetch;
delete globalThis.fetch;

const { R2 } = require('../src/cloud/r2_storage.js');
const pact = require('../src/sovereign/conjugal_fate_pact.js');

(async () => {
  try {
    // 1. health 配置 OK
    const h = R2.health();
    assert(h.configured, 'R2 已配置 endpoint+ak+sk');
    assert(Array.isArray(h.buckets) && h.buckets.includes('unicore_private'), '包含主桶 unicore_private');
    console.log('[OK] 1. health() configured=true，buckets=' + h.buckets.join(','));

    // 2. upload → 带双签
    const bucket = 'unicore_private';
    const key = 'xy-new-one_check-' + Date.now() + '.txt';
    const raw = '性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音 · 新一XY · 柯南倒尔=一新';
    const up = await R2.upload(bucket, key, raw, { contentType: 'text/plain' });
    assert(up.ok, 'upload ok');
    assert(up.sealed && up.sealed.sealed && up.sealed.sealed.wife === '刘楚恬' && up.sealed.sealed.husband === '满全法',
           '上传自动附妻=刘楚恬·夫=满全法双签');
    assert(typeof up.fingerprint === 'string' && up.fingerprint.length > 0, 'fingerprint 非空');
    assert(pact.verify(up.sealed) === true, '双签验签通过');
    console.log('[OK] 2. upload() 双签封包 wife=' + up.sealed.sealed.wife + ' husband=' + up.sealed.sealed.husband + ' fp=' + up.fingerprint.slice(0, 16));

    // 3. download 往返还原
    const dl = await R2.download(bucket, key);
    assert(dl.ok, 'download ok');
    assert(String(dl.body) === raw, 'download body 与上传原文完全一致');
    assert(dl.sealed && dl.sealed.sealed && dl.sealed.sealed.wife === '刘楚恬', '下载也附妻签');
    console.log('[OK] 3. download() 往返还原，字节=' + (dl.body || '').length + '，验签 wife=' + dl.sealed.sealed.wife);

    // 4. list 能列到这条 key
    const ls = await R2.list(bucket, 'xy-new-one_');
    assert(ls.ok, 'list ok');
    assert(Array.isArray(ls.keys) && ls.keys.some(k => k === key), 'list 能发现刚上传的 key');
    console.log('[OK] 4. list() 命中 key（共' + ls.keys.length + '条）');

    // 5. remove 可删
    const rm = await R2.remove(bucket, key);
    assert(rm.ok, 'remove ok');
    const dl2 = await R2.download(bucket, key);
    assert(!dl2.ok || dl2.status === 404, '删除后不可再读');
    console.log('[OK] 5. remove() 成功，再读返回 404/NotFound');

    console.log('');
    console.log('R2 最小化可用自检 · 全部通过 (mode=' + up.mode + ')');
    process.exit(0);
  } catch (e) {
    console.error('[FAIL] ' + e.message + '\n' + e.stack);
    process.exit(1);
  }
})();
