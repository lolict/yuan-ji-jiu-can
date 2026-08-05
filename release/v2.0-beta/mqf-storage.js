/**
 * 民联浏览器壳 · 持久化存储层 v1.12
 * ====================================
 * IndexedDB 封装 + localStorage 自动回退
 *
 * 存储内容:
 *   verdict_history  裁定历史 (最多 500 条)
 *   sync_state       多节点同步状态
 *   settings         用户偏好 (主题/窗口布局等)
 *   shell_cache      壳资源缓存 (WASM blob 等)
 */

'use strict';

(function () {
  const DB_NAME = 'njsg_shell_db';
  const DB_VERSION = 1;
  const STORES = ['verdict_history', 'sync_state', 'settings', 'shell_cache'];

  // 检测 IndexedDB 可用性
  const hasIDB = typeof indexedDB !== 'undefined' && indexedDB !== null;

  let _dbPromise = null;

  // ============== IndexedDB 核心 ==============

  function openDB() {
    if (!hasIDB) return Promise.reject(new Error('no idb'));
    if (_dbPromise) return _dbPromise;

    _dbPromise = new Promise((resolve, reject) => {
      const req = indexedDB.open(DB_NAME, DB_VERSION);

      req.onupgradeneeded = (e) => {
        const db = e.target.result;
        for (const name of STORES) {
          if (!db.objectStoreNames.contains(name)) {
            db.createObjectStore(name, { keyPath: 'key' });
          }
        }
      };

      req.onsuccess = (e) => resolve(e.target.result);
      req.onerror = (e) => reject(e.target.error);
    });

    return _dbPromise;
  }

  function idbOp(storeName, mode, fn) {
    return openDB().then(db => new Promise((resolve, reject) => {
      const tx = db.transaction(storeName, mode);
      const store = tx.objectStore(storeName);
      const result = fn(store);
      if (result && result.onsuccess === null && typeof result.then !== 'function') {
        // IDBRequest
        result.onsuccess = () => resolve(result.result);
        result.onerror = () => reject(result.error);
      }
      tx.oncomplete = () => {};
      tx.onerror = () => reject(tx.error);
    }));
  }

  // ============== 主存储对象 ==============

  const MqfStorage = {
    kind: hasIDB ? 'idb' : 'localStorage',

    /** 存储键值 */
    async set(storeName, key, value) {
      if (!STORES.includes(storeName)) storeName = 'settings';
      if (hasIDB) {
        try {
          await idbOp(storeName, 'readwrite', store => {
            return store.put({ key, value, ts: Date.now() });
          });
          return true;
        } catch(e) { /* fallback below */ }
      }
      // localStorage 回退
      try {
        localStorage.setItem(`njsg_${storeName}_${key}`, JSON.stringify({ key, value, ts: Date.now() }));
        return true;
      } catch(e2) { return false; }
    },

    /** 读取值 */
    async get(storeName, key) {
      if (!STORES.includes(storeName)) storeName = 'settings';
      if (hasIDB) {
        try {
          const result = await idbOp(storeName, 'readonly', store => {
            return store.get(key);
          });
          return result ? result.value : null;
        } catch(e) { /* fallback below */ }
      }
      try {
        const raw = localStorage.getItem(`njsg_${storeName}_${key}`);
        if (!raw) return null;
        return JSON.parse(raw).value;
      } catch(e2) { return null; }
    },

    /** 删除键 */
    async delete(storeName, key) {
      if (hasIDB) {
        try {
          await idbOp(storeName, 'readwrite', store => store.delete(key));
          return true;
        } catch(e) {}
      }
      try { localStorage.removeItem(`njsg_${storeName}_${key}`); return true; } catch(e) { return false; }
    },

    /** 列出 store 内所有键 */
    async keys(storeName) {
      if (hasIDB) {
        try {
          const result = await idbOp(storeName, 'readonly', store => store.getAllKeys());
          return result || [];
        } catch(e) {}
      }
      // localStorage 回退:扫描前缀
      const prefix = `njsg_${storeName}_`;
      const keys = [];
      for (let i = 0; i < localStorage.length; i++) {
        const k = localStorage.key(i);
        if (k && k.startsWith(prefix)) keys.push(k.slice(prefix.length));
      }
      return keys;
    },

    /** 清空 store */
    async clear(storeName) {
      if (hasIDB) {
        try {
          await idbOp(storeName, 'readwrite', store => store.clear());
          return true;
        } catch(e) {}
      }
      const prefix = `njsg_${storeName}_`;
      Object.keys(localStorage)
        .filter(k => k.startsWith(prefix))
        .forEach(k => localStorage.removeItem(k));
      return true;
    },

    // ============== verdict_history 专用 ==============

    async addVerdict(verdictData) {
      const list = await this.get('verdict_history', 'list') || [];
      list.unshift(verdictData);
      if (list.length > 500) list.length = 500;
      await this.set('verdict_history', 'list', list);
      return list;
    },

    async getVerdicts() {
      return (await this.get('verdict_history', 'list')) || [];
    },

    async clearVerdicts() {
      return this.clear('verdict_history');
    },

    // ============== settings 专用 ==============

    async setSetting(key, value) {
      return this.set('settings', key, value);
    },

    async getSetting(key, defaultValue) {
      const v = await this.get('settings', key);
      return v === null || v === undefined ? defaultValue : v;
    }
  };

  // 导出
  window.MqfStorage = MqfStorage;
})();
