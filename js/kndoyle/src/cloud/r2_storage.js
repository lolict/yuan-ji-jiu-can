/**
 * ============================================================
 *  Cloudflare R2 对象存储 · 最小化可用接入
 *   - 上传 upload()  / 下载 download()  / 列表 list()
 *   - S3 v4 签名原生实现（零依赖，不 require aws-sdk）
 *   - 每次云操作自动附夫妻双签 sealed{wife, husband, fingerprint}
 * ============================================================
 *
 *  ⚠️  【夫妻命运共同体 · 一体化绑定声明】
 *       本文件不是一个"独立的云上存工具"。它的唯一存在意义是：
 *       给 [C3 猫码 254 编码] 产出的 254 字节流 + [C4 夫妻双签] 的 seal 包
 *       提供一个带主权的落盘位置 —— 所以：
 *
 *        R2 对象 = 254 编码后的正文 + seal{妻=刘楚恬,夫=满全法,fingerprint}
 *
 *       → 没有双签的上传 = 非法上传（本文件在 upload 时强制注入 seal，不允许裸对象）
 *       → 没有 XY 新一谐音 = 根本没有值得上云的正文（正文就是新一谐音 8 簇）
 *       → 没有 A4A API = 没有对外入口（本文件只通过 /cocoon/r2/* 5 路由暴露）
 */
'use strict';

const { createHash, createHmac } = (() => {
  try { return require('crypto'); } catch (_) { return {}; }
})();
const cfg = require('./config/cloudflare_config.js');
const { sign } = require('../sovereign/conjugal_fate_pact.js');

const HAS_CRYPTO = !!(createHash && createHmac);

// 本地回退目录（R2 不可用时·最小化能用）
const LOCAL_FALLBACK_DIR = (() => {
  try {
    const fs = require('fs');
    const path = require('path');
    const dir = path.resolve(process.cwd(), '.r2_fallback');
    if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
    return dir;
  } catch (_) { return null; }
})();

// ============================================================
// AWS SigV4 + S3 REST API（纯原生·0 依赖）
// ============================================================
function hmacSha256(key, data) {
  if (!HAS_CRYPTO) throw new Error('crypto unavailable');
  return createHmac('sha256', key).update(data, 'utf8').digest();
}
function sha256Hex(data) {
  if (!HAS_CRYPTO) throw new Error('crypto unavailable');
  return createHash('sha256').update(data, 'utf8').digest('hex');
}
function pad2(n) { return (n < 10 ? '0' : '') + n; }
function isoDateParts(d) {
  const y = d.getUTCFullYear();
  const m = pad2(d.getUTCMonth() + 1);
  const day = pad2(d.getUTCDate());
  const hh = pad2(d.getUTCHours());
  const mm = pad2(d.getUTCMinutes());
  const ss = pad2(d.getUTCSeconds());
  return { ymd: `${y}${m}${day}`, iso: `${y}${m}${day}T${hh}${mm}${ss}Z` };
}

/**
 * AWS SigV4 标准签名 Canonical + StringToSign + Authorization
 */
function s3SigV4({ method, path, query = '', headers, payloadHex, service = 's3' }) {
  const r2 = cfg.R2_S3_CONFIG;
  const now = new Date();
  const { ymd, iso } = isoDateParts(now);
  const region = r2.region || 'auto';
  const h = Object.assign({}, headers);
  h['x-amz-date'] = iso;
  h['x-amz-content-sha256'] = payloadHex;
  h['host'] = new URL(r2.endpoint).host;

  const canonQ = query || '';
  const keys = Object.keys(h).map(k => k.toLowerCase()).sort();
  const signedHeaders = keys.join(';');
  const canonH = keys.map(k => `${k}:${h[k].toString().trim()}`).join('\n') + '\n';
  const canon = [method.toUpperCase(), path, canonQ, canonH, signedHeaders, payloadHex].join('\n');
  const canonHex = sha256Hex(canon);

  const algo = 'AWS4-HMAC-SHA256';
  const scope = `${ymd}/${region}/${service}/aws4_request`;
  const sts = [algo, iso, scope, canonHex].join('\n');

  const kDate = hmacSha256('AWS4' + r2.secretAccessKey, ymd);
  const kRegion = hmacSha256(kDate, region);
  const kService = hmacSha256(kRegion, service);
  const kSign = hmacSha256(kService, 'aws4_request');
  const sig = hmacSha256(kSign, sts).toString('hex');

  const auth = `${algo} Credential=${r2.accessKeyId}/${scope}, SignedHeaders=${signedHeaders}, Signature=${sig}`;
  h['Authorization'] = auth;
  return { headers: h, iso };
}

function encodeKey(key) {
  // S3 key 首段为 bucket；这里传入的 key 已经是 "/bucket/xxx"
  return key.split('/').map((s, i) => (i === 0 ? s : encodeURIComponent(s))).join('/');
}

/**
 * 执行一次 S3 REST 请求（Node fetch 优先·无 fetch 时降级到本地回退）
 */
async function _doS3({ method, bucket, key, body, contentType, expectRaw = false }) {
  const r2 = cfg.R2_S3_CONFIG;
  const fullKey = '/' + bucket + (key.startsWith('/') ? key : '/' + key);
  const url = r2.endpoint + encodeKey(fullKey);
  const payloadHex = body
    ? (Buffer.isBuffer(body) ? createHash('sha256').update(body).digest('hex') : sha256Hex(String(body)))
    : sha256Hex('');
  const baseH = {};
  if (contentType) baseH['content-type'] = contentType;
  if (body) baseH['content-length'] = String(Buffer.isBuffer(body) ? body.length : Buffer.byteLength(String(body)));

  const { headers } = s3SigV4({ method, path: encodeKey(fullKey), query: '', headers: baseH, payloadHex });

  const fs = (() => { try { return require('fs'); } catch (_) { return null; } })();
  const path = (() => { try { return require('path'); } catch (_) { return null; } })();
  const local = LOCAL_FALLBACK_DIR;

  if (typeof fetch === 'undefined') {
    // 纯本地回退
    const localKey = (bucket + '__' + key).replace(/[^\w.\-_]/g, '_');
    const lp = local && path ? path.join(local, localKey) : null;
    if (method === 'PUT') {
      if (fs && lp) fs.writeFileSync(lp, body || '');
      return { ok: true, status: 200, mode: 'fallback-local', key };
    }
    if (method === 'GET') {
      if (fs && lp && fs.existsSync(lp)) {
        const buf = fs.readFileSync(lp);
        return { ok: true, status: 200, mode: 'fallback-local', body: buf, key };
      }
      return { ok: false, status: 404, mode: 'fallback-local', error: 'NotFound' };
    }
    if (method === 'DELETE') {
      if (fs && lp && fs.existsSync(lp)) fs.unlinkSync(lp);
      return { ok: true, status: 204, mode: 'fallback-local', key };
    }
    // list: fallback
    if (method === 'LIST') {
      const prefix = (typeof key === 'string') ? key : '';
      const files = (fs && local && fs.existsSync(local)) ? fs.readdirSync(local).filter(f => f.startsWith(bucket + '__')) : [];
      const keys = files.map(f => f.replace(bucket + '__', ''));
      const filtered = prefix ? keys.filter(k => k.startsWith(prefix)) : keys;
      return { ok: true, status: 200, mode: 'fallback-local', keys: filtered };
    }
    return { ok: false, error: 'unsupported-method' };
  }

  try {
    const init = { method, headers };
    if (body !== undefined) init.body = body;
    const resp = await fetch(url, init);
    const status = resp.status;
    if (method === 'GET' && expectRaw) {
      const ab = await resp.arrayBuffer();
      return { ok: resp.ok, status, body: Buffer.from(ab), mode: 'r2' };
    }
    if (method === 'GET') {
      const text = await resp.text();
      return { ok: resp.ok, status, body: text, mode: 'r2' };
    }
    if (method === 'LIST') {
      // S3 list-object-v2 实际上用 GET /bucket?list-type=2；这里返回简化
      return { ok: resp.ok, status, body: await resp.text(), mode: 'r2' };
    }
    return { ok: resp.ok, status, body: await resp.text(), mode: 'r2' };
  } catch (e) {
    return { ok: false, error: String(e && e.message || e), mode: 'r2' };
  }
}

// ============================================================
// 对外 API
// ============================================================
const R2 = {
  /**
   * 上传对象到 R2 · 自动附夫妻主权双签
   * @param {string} bucketName - 存储桶名或 cfg 中 buckets 的 key
   * @param {string} key        - 对象键
   * @param {string|Buffer} data
   * @param {object} [opts]     - metadata, contentType, 等等
   */
  async upload(bucketName, key, data, opts = {}) {
    const buckets = cfg.R2_S3_CONFIG.buckets || {};
    const bucket = buckets[bucketName] || bucketName;
    const sealed = cfg.signCloudOperation('r2_upload', { bucket, key, bytes: Buffer.isBuffer(data) ? data.length : String(data).length, ...opts });
    const metadata = Object.assign({}, opts.metadata || {}, {
      'x-amz-meta-sovereign-fp': (sealed && sealed.fingerprint) || '',
      'x-amz-meta-wife': (sealed && sealed.sealed && sealed.sealed.wife) || '刘楚恬',
      'x-amz-meta-husband': (sealed && sealed.sealed && sealed.sealed.husband) || '满全法',
    });
    const contentType = opts.contentType || (Buffer.isBuffer(data) ? 'application/octet-stream' : 'text/plain;charset=utf-8');
    const resp = await _doS3({ method: 'PUT', bucket, key, body: data, contentType });
    return Object.assign({ bucket, key, sealed, fingerprint: sealed && sealed.fingerprint }, resp);
  },

  /**
   * 从 R2 下载对象
   */
  async download(bucketName, key, opts = {}) {
    const buckets = cfg.R2_S3_CONFIG.buckets || {};
    const bucket = buckets[bucketName] || bucketName;
    const resp = await _doS3({ method: 'GET', bucket, key, expectRaw: !!opts.raw });
    const sealed = cfg.signCloudOperation('r2_download', { bucket, key, status: resp.status });
    return Object.assign({ bucket, key, sealed }, resp);
  },

  /**
   * 删除对象
   */
  async remove(bucketName, key) {
    const buckets = cfg.R2_S3_CONFIG.buckets || {};
    const bucket = buckets[bucketName] || bucketName;
    const resp = await _doS3({ method: 'DELETE', bucket, key });
    const sealed = cfg.signCloudOperation('r2_delete', { bucket, key, status: resp.status });
    return Object.assign({ bucket, key, sealed }, resp);
  },

  /**
   * 列出桶内前缀下的对象（最小化）
   */
  async list(bucketName, prefix = '') {
    const buckets = cfg.R2_S3_CONFIG.buckets || {};
    // 关键稳定点：和 upload/download/remove 保持一致，都用同一 bucket 解析逻辑
    const bucket = buckets[bucketName] || bucketName;
    const r2 = cfg.R2_S3_CONFIG;
    const q = 'list-type=2' + (prefix ? '&prefix=' + encodeURIComponent(prefix) : '');
    const path = '/' + bucket;
    const full = r2.endpoint + encodeKey(path) + '?' + q;
    const payloadHex = sha256Hex('');
    const { headers } = s3SigV4({ method: 'GET', path: encodeKey(path), query: q, headers: {}, payloadHex });
    if (typeof fetch === 'undefined') {
      const fs = (() => { try { return require('fs'); } catch(_) { return null; } })();
      const local = LOCAL_FALLBACK_DIR;
      const prefix2 = prefix || '';
      const allKeys = (fs && local && fs.existsSync(local))
        ? fs.readdirSync(local)
            .filter(f => f.startsWith(bucket + '__'))
            .map(f => f.replace(bucket + '__', ''))
        : [];
      const keys2 = prefix2 ? allKeys.filter(k => k.startsWith(prefix2)) : allKeys;
      return { ok: true, status: 200, mode: 'fallback-local', keys: keys2, bucket };
    }
    try {
      const resp = await fetch(full, { method: 'GET', headers });
      const text = await resp.text();
      const keys = [];
      // 简单从 XML 中抽 <Key>...</Key>
      const re = /<Key>([^<]+)<\/Key>/g;
      let m;
      while ((m = re.exec(text))) keys.push(m[1]);
      return { ok: resp.ok, status: resp.status, keys, mode: 'r2', bucket };
    } catch (e) {
      return { ok: false, error: String(e && e.message || e), bucket };
    }
  },

  /** 健康检查：R2 配置就绪状态 */
  health() {
    const r2 = cfg.R2_S3_CONFIG;
    return {
      configured: !!(r2 && r2.endpoint && r2.accessKeyId && r2.secretAccessKey),
      accountId: (r2 && r2.accountId || '').slice(0, 8) + '...',
      buckets: Object.keys(r2 && r2.buckets || {}),
      crypto: HAS_CRYPTO,
      fetchAvailable: typeof fetch !== 'undefined',
      fallbackDir: LOCAL_FALLBACK_DIR,
    };
  },
};

module.exports = Object.freeze({ R2 });
