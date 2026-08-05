/**
 * 盟机合 · 多节点同步协议 v1.11
 * ============================
 * 实现盟机合掌节点间的裁定结果同步
 *
 * 原则:
 *   - 去中心化 Gossip 风格,无主节点协调
 *   - 最终一致性 (eventual consistency)
 *   - 轻量 JSON 消息,HTTP POST
 *   - 幂等 + 增量同步 (seq 序号)
 *
 * 消息格式:
 *   SYNC_PUSH  { type: 'push', from, seq, entries[], sig }
 *   SYNC_PULL  { type: 'pull', from, since }
 *   SYNC_RESP  { type: 'resp', from, entries[], latest_seq }
 */

// ============== 节点同步状态 (存储在 KV / memory) ==============

/**
 * SYNC_STATE  — 每个节点的同步状态
 * {
 *   known_peers:   { peer_url: last_seq }    // 已知的对等节点及其最新序号
 *   local_seq:     0                          // 本地全局序号
 *   verdict_log:    []                         // 有序裁定日志
 *   merkle_root:   ""                         // 当前默克尔根
 * }
 * 
 * 对于 Cloudflare Worker,存储在 KV (NJSG_SYNC_KV)
 * 对于民联浏览器,存储在 localStorage (njsg_sync_state)
 */

// ============== 裁定日志条目 ==============

function makeVerdictEntry(verdictResult, sender) {
  const entry = {
    seq: null,            // 由 node 分配
    ts: Date.now(),
    sender: sender || 'self',
    entity: verdictResult.entity || 'unknown',
    verdict: verdictResult.njsg_verdict?.trust || 'UNKNOWN',
    confidence: verdictResult.njsg_verdict?.confidence || 0,
    ticks: verdictResult.njsg_verdict?.five_yuan?.ticks || 0,
    hash: null             // SHA-256(JSON.stringify(entry_without_hash))
  };
  return entry;
}

// ============== 默克尔树 (简化版) ==============

function simpleHash(data) {
  // 简易 hash,生产环境用 SubtleCrypto 或 WebCrypto
  const str = typeof data === 'string' ? data : JSON.stringify(data);
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash + str.charCodeAt(i)) | 0;
  }
  return Math.abs(hash).toString(16).padStart(8, '0');
}

function computeMerkleRoot(entries) {
  if (!entries || entries.length === 0) return '0'.repeat(8);
  
  let hashes = entries.map(e => simpleHash(JSON.stringify({
    seq: e.seq, ts: e.ts, entity: e.entity, verdict: e.verdict
  })));
  
  while (hashes.length > 1) {
    const next = [];
    for (let i = 0; i < hashes.length; i += 2) {
      const left = hashes[i];
      const right = hashes[i + 1] || left;
      next.push(simpleHash(left + right));
    }
    hashes = next;
  }
  return hashes[0];
}

// ============== 同步引擎 (框架,可被 Worker 和浏览器共用) ==============

class MqfSyncEngine {
  constructor(opts = {}) {
    this.nodeId = opts.nodeId || ('node_' + Math.random().toString(36).slice(2, 10));
    this.storage = opts.storage || new MemoryStorage();
    this.peers = opts.peers || [];
    this.maxLogSize = opts.maxLogSize || 1000;
    this.onSync = opts.onSync || (() => {});
  }

  // 获取/初始化状态
  async getState() {
    const state = await this.storage.get('njsg_sync_state');
    if (state) return state;
    return { known_peers: {}, local_seq: 0, verdict_log: [], merkle_root: '00000000' };
  }

  // 保存状态
  async saveState(state) {
    await this.storage.set('njsg_sync_state', state);
  }

  // 添加本地裁定
  async addVerdict(verdictResult) {
    const state = await this.getState();
    state.local_seq++;
    const entry = makeVerdictEntry(verdictResult, this.nodeId);
    entry.seq = state.local_seq;
    entry.hash = simpleHash(JSON.stringify({
      seq: entry.seq, ts: entry.ts, entity: entry.entity, verdict: entry.verdict
    }));
    state.verdict_log.push(entry);
    
    // 裁剪日志
    if (state.verdict_log.length > this.maxLogSize) {
      state.verdict_log = state.verdict_log.slice(-this.maxLogSize);
    }
    
    state.merkle_root = computeMerkleRoot(state.verdict_log);
    await this.saveState(state);
    return entry;
  }

  // SYNC_PUSH: 接收来自 peer 的裁定
  async handlePush(msg) {
    if (!msg || msg.type !== 'push') return { error: 'invalid msg' };
    const state = await this.getState();
    const newEntries = msg.entries || [];
    
    let added = 0;
    for (const entry of newEntities(newEntries, state.verdict_log)) {
      state.local_seq++;
      entry.seq = state.local_seq;
      entry.sender = msg.from || 'peer';
      state.verdict_log.push(entry);
      added++;
    }
    
    if (added > 0) {
      if (state.verdict_log.length > this.maxLogSize) {
        state.verdict_log = state.verdict_log.slice(-this.maxLogSize);
      }
      state.merkle_root = computeMerkleRoot(state.verdict_log);
    
      // 记录 peer 状态
      if (msg.from) {
        const latestEntry = newEntries[newEntries.length - 1];
        if (latestEntry) {
          state.known_peers[msg.from] = Math.max(
            state.known_peers[msg.from] || 0,
            latestEntry.seq || 0
          );
        }
      }
      
      await this.saveState(state);
    }
    
    this.onSync({ type: 'push', added, from: msg.from });
    return { ok: true, added, merkle_root: state.merkle_root };
  }

  // 生成 SYNC_PUSH 消息
  makePushMessage() {
    return (async () => {
      const state = await this.getState();
      return {
        type: 'push',
        from: this.nodeId,
        seq: state.local_seq,
        merkle_root: state.merkle_root,
        entries: state.verdict_log.slice(-50),  // 最多推 50 条
        ts: Date.now()
      };
    })();
  }

  // 生成 SYNC_PULL 消息
  makePullMessage(since) {
    return {
      type: 'pull',
      from: this.nodeId,
      since: since || 0,
      ts: Date.now()
    };
  }

  // SYNC_PULL: 响应 peer 的拉取请求
  async handlePull(msg) {
    if (!msg || msg.type !== 'pull') return { error: 'invalid pull' };
    const state = await this.getState();
    const since = msg.since || 0;
    
    const entries = state.verdict_log.filter(e => e.seq > since);
    return {
      type: 'resp',
      from: this.nodeId,
      entries,
      latest_seq: state.local_seq,
      merkle_root: state.merkle_root,
      total: state.verdict_log.length
    };
  }

  // SYNC_RESP: 处理 peer 的响应 (合并进本地)
  async handleResp(msg) {
    if (!msg || msg.type !== 'resp') return { error: 'invalid resp' };
    const state = await this.getState();
    
    let added = 0;
    for (const entry of newEntities(msg.entries || [], state.verdict_log)) {
      state.local_seq++;
      entry.seq = state.local_seq;
      entry.sender = msg.from || 'peer';
      state.verdict_log.push(entry);
      added++;
    }
    
    if (added > 0) {
      if (state.verdict_log.length > this.maxLogSize) {
        state.verdict_log = state.verdict_log.slice(-this.maxLogSize);
      }
      state.merkle_root = computeMerkleRoot(state.verdict_log);
      
      if (msg.from) {
        state.known_peers[msg.from] = msg.latest_seq || 0;
      }
      
      await this.saveState(state);
    }
    
    this.onSync({ type: 'pull_response', added, from: msg.from });
    return { ok: true, added, merkle_root: state.merkle_root };
  }

  // 主动同步到某 peer
  async syncTo(peerUrl) {
    // 1. 先 PULL: 获取 peer 已知最新序号
    const state = await this.getState();
    const lastKnown = state.known_peers[peerUrl] || 0;
    const pullMsg = this.makePullMessage(lastKnown);
    
    try {
      const resp = await fetch(peerUrl + '/api/v1/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(pullMsg)
      });
      if (!resp.ok) return { error: 'pull failed', status: resp.status };
      const pullData = await resp.json();
      
      // 2. 合并 peer 返回的 entries
      const pullResult = await this.handleResp(pullData);
      
      // 3. 再 PUSH: 把本地最新推给 peer
      const pushMsg = await this.makePushMessage();
      const pushResp = await fetch(peerUrl + '/api/v1/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(pushMsg)
      });
      const pushData = pushResp.ok ? await pushResp.json() : { skipped: true };
      
      return {
        ok: true,
        peerUrl,
        pulled: pullResult.added || 0,
        pushed: typeof pushData.added === 'number' ? pushData.added : 0,
        local_seq: state.local_seq,
        merkle_root: state.merkle_root
      };
    } catch (e) {
      return { error: e.message, peerUrl };
    }
  }

  // 广播同步到所有 peers
  async syncAll() {
    const results = [];
    for (const peer of this.peers) {
      const r = await this.syncTo(peer);
      results.push(r);
    }
    return { ok: true, results };
  }

  // 获取同步摘要 (用于 UI 显示)
  async summary() {
    const state = await this.getState();
    const peers = Object.entries(state.known_peers);
    return {
      nodeId: this.nodeId,
      local_seq: state.local_seq,
      total_verdicts: state.verdict_log.length,
      merkle_root: state.merkle_root,
      peer_count: peers.length,
      peers: peers.map(([url, seq]) => ({ url, last_seq: seq })),
      recent_verdicts: state.verdict_log.slice(-5).reverse()
    };
  }
}

// ============== 存储适配器 ==============

/** 内存存储 (用于测试) */
class MemoryStorage {
  constructor() { this.data = {}; }
  async get(key) { return this.data[key] || null; }
  async set(key, val) { this.data[key] = val; }
  async delete(key) { delete this.data[key]; }
}

/** localStorage 存储 (浏览器端) */
class LocalStorageAdapter {
  async get(key) { return localStorage.getItem(key); }
  async set(key, val) { localStorage.setItem(key, typeof val === 'string' ? val : JSON.stringify(val)); }
  async delete(key) { localStorage.removeItem(key); }
}

/** 解析 JSON 时自动还原 */
async function getStateFromStorage(storage) {
  const raw = await storage.get('njsg_sync_state');
  if (!raw) return null;
  try { return typeof raw === 'string' ? JSON.parse(raw) : raw; }
  catch(e) { return null; }
}

/** 去重: 过滤掉 already 已存在的 entries */
function newEntities(newEntries, existingEntries) {
  const existingKeys = new Set(
    existingEntries.map(e => `${e.sender}:${e.seq}:${e.entity}:${e.ts}`)
  );
  return newEntries.filter(e => {
    const key = `${e.sender}:${e.seq}:${e.entity}:${e.ts}`;
    if (existingKeys.has(key)) return false;
    existingKeys.add(key);
    return true;
  });
}

// ============== Cloudflare Worker 端适配器 ==============

/**
 * 在 njsg-web.js 中调用:
 * 
 *   import { WorkerSyncHandler } from './sync-protocol.js';
 *   
 *   // 在 Worker 事件处理中:
 *   if (pathname === '/api/v1/sync') {
 *     const result = await handleWorkerSync(request, env);
 *     return new Response(JSON.stringify(result), { ... });
 *   }
 */
async function handleWorkerSync(request, env) {
  const SYNC_STORAGE_KEY = 'njsg_sync_state_v1';
  
  if (request.method !== 'POST') {
    return { error: 'POST only' };
  }
  
  let msg;
  try { msg = await request.json(); } catch(e) { return { error: 'invalid json' }; }
  
  // 读取 state
  let state = await env?.NJSG_SYNC_KV?.get(SYNC_STORAGE_KEY, 'json');
  if (!state) {
    state = { known_peers: {}, local_seq: 0, verdict_log: [], merkle_root: '00000000' };
  }
  
  // 简单处理 (KV 场景),完整版用 MqfSyncEngine
  if (msg.type === 'push') {
    const newEntries = msg.entries || [];
    let added = 0;
    for (const entry of newEntities(newEntries, state.verdict_log)) {
      state.local_seq++;
      entry.seq = state.local_seq;
      state.verdict_log.push(entry);
      added++;
    }
    if (added > 0) {
      if (state.verdict_log.length > 1000) {
        state.verdict_log = state.verdict_log.slice(-1000);
      }
      state.merkle_root = computeMerkleRoot(state.verdict_log);
      if (msg.from) {
        state.known_peers[msg.from] = msg.seq || 0;
      }
      await env?.NJSG_SYNC_KV?.put(SYNC_STORAGE_KEY, JSON.stringify(state));
    }
    return { ok: true, added, merkle_root: state.merkle_root };
  }
  
  if (msg.type === 'pull') {
    const since = msg.since || 0;
    const entries = state.verdict_log.filter(e => e.seq > since);
    return {
      type: 'resp',
      from: 'njsg-worker',
      entries,
      latest_seq: state.local_seq,
      merkle_root: state.merkle_root,
      total: state.verdict_log.length
    };
  }
  
  return { error: 'unknown msg type: ' + msg.type };
}

// ============== .exports ==============

if (typeof module !== 'undefined') {
  module.exports = {
    MqfSyncEngine, MemoryStorage, LocalStorageAdapter,
    handleWorkerSync, makeVerdictEntry, computeMerkleRoot, simpleHash,
    newEntities
  };
}
