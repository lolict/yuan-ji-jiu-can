/**
 * 民联浏览器壳 · MQF Shell v1.7-v1.10
 * ==================================
 * 民联浏览器 · WASM 裁定内核加载器
 * 
 * v1.10 新增: 移动端原生持久化
 *   - Android WebView mqfNative 桥
 *   - iOS WKWebView mqfNative 桥
 *   - localStorage 自动回退
 *   - window.mqfNative 统一接口 (save/load/clear/getPlatform)
 * 
 * 功能:
 *   1. 优先加载 mqf_core.wasm (WASI 裁定内核)
 *   2. 如 WASM 不可用,自动回退到 JS 实现
 *   3. 提供裁定台 UI 控制器 / 旋钮可视化 / 蜂窝嵌套视图
 *   4. 移动端原生持久化 (优先) / localStorage (回退)
 *   5. 域无关: 任何地方运行输出一致
 *
 * 裁定引擎与 kernel/wasm/mqf_wasm.c 保持算法一致:
 *   五元: step + angle wrap 360° + 占存 (>100 → ETERNAL)
 *   六元: 信任交集(min) + 镜像对齐(|a-b|<0.2)
 *   七元: 蜂窝套壳递归 (6 child per shell)
 */

'use strict';

// ============== 裁定核心 ==============
// 与 mqf_wasm.c 算法严格一致

const MQF_SHELL_VERSION = '1.10.0';
const FIVE_YUAN_STEPS = 5;
const FIVE_YUAN_OCCUPY_THRESHOLD = 100;
const SIX_YUAN_MIRROR_THRESHOLD = 0.2;
const SIX_YUAN_VERDICT_THRESHOLD = 0.6;
const MAX_CHILD = 6;

const VerdictState = {
  REJECTED: 0,
  TRUSTED: 1
};

/**
 * 五元旋钮: 角度累加 wrap 360°, ticks 累加, 占存判定
 */
function fiveYuanPush(state, angleInc, forceInc) {
  if (state.step >= FIVE_YUAN_STEPS) return state;
  state.step++;
  state.angle = ((state.angle + angleInc) % 360 + 360) % 360;
  state.force += forceInc;
  state.ticks += Math.round(forceInc * 10);
  if (state.ticks > FIVE_YUAN_OCCUPY_THRESHOLD) {
    state.occupied = true;
  }
  return state;
}

/**
 * 六元裁定: 信任交集 + 镜像对齐
 */
function sixYuanVerdict(complianceA, complianceB) {
  const intersection = Math.min(complianceA, complianceB);
  const diff = Math.abs(complianceA - complianceB);
  const mirrorAligned = diff < SIX_YUAN_MIRROR_THRESHOLD;
  const verdict = (intersection > SIX_YUAN_VERDICT_THRESHOLD && mirrorAligned)
    ? VerdictState.TRUSTED
    : VerdictState.REJECTED;
  return { intersection, mirrorAligned, verdict };
}

/**
 * 裁定主引擎 (对应 mqf_verdict_engine in mqf_wasm.c)
 */
function mqfVerdictEngine(request) {
  const { entity, complianceA, complianceB, force, angle } = request;

  // 五元: 角度 wrap
  const finalAngle = ((angle % 360) + 360) % 360;
  const ticks = Math.round(force * 10);
  const occupied = ticks > FIVE_YUAN_OCCUPY_THRESHOLD;

  // 六元: 信任交集 + 镜像对齐
  const { intersection, mirrorAligned, verdict } = sixYuanVerdict(complianceA, complianceB);

  const philosophy = verdict === VerdictState.TRUSTED
    ? '已被铭记 — 双重背书,可信任'
    : '仍在观察 — 单方或双方否决';

  return {
    ok: true,
    version: MQF_SHELL_VERSION,
    entity: entity || 'unknown',
    mqf_verdict: {
      trust: verdict === VerdictState.TRUSTED ? 'TRUSTED' : 'REJECTED',
      confidence: Math.round(intersection * 1000) / 1000,
      mirror_alignment: mirrorAligned ? 'ALIGNED' : 'OFFSET',
      five_yuan: {
        ticks,
        occupied: occupied ? 'ETERNAL' : 'EPHEMERAL',
        angle: Math.round(finalAngle * 10) / 10
      },
      six_yuan: {
        intersection: Math.round(intersection * 1000) / 1000,
        verdict
      }
    },
    philosophy
  };
}

// ============== WASM 加载器 ==============

/**
 * 尝试加载 mqf_core.wasm
 * 如可用,使用 WASI 接口; 如不可用,回退到 JS 实现
 */
const MqfCore = {
  wasmModule: null,
  wasmReady: false,
  mode: 'js', // 'wasm' or 'js'

  async init() {
    // 尝试加载 wasm
    try {
      if (typeof WebAssembly !== 'undefined') {
        const response = await fetch('./mqf_core.wasm', { method: 'GET' });
        if (response.ok) {
          const buf = await response.arrayBuffer();
          this.wasmModule = await WebAssembly.instantiate(buf, {
            wasi_snapshot_preview1: this._wasiShim()
          });
          this.wasmReady = true;
          this.mode = 'wasm';
          return 'wasm';
        }
      }
    } catch (e) {
      console.warn('[MQF Shell] WASM load failed, falling back to JS:', e.message);
    }
    this.mode = 'js';
    return 'js';
  },

  /**
   * 极简 WASI shim (仅支持 fd_read / fd_write / proc_exit)
   */
  _wasiShim() {
    const state = { inputBuf: null, inputPos: 0, outputBuf: [] };
    return {
      fd_read: (fd, iovs, iovsLen, nread) => {
        if (fd !== 0) return 8; // EBADF
        // 不实际读取,返回空
        const mem = this.wasmModule.exports.memory;
        const view = new DataView(mem.buffer);
        view.setInt32(nread, 0, true);
        return 0;
      },
      fd_write: (fd, iovs, iovsLen, nwritten) => {
        if (fd !== 1) return 8;
        const mem = this.wasmModule.exports.memory;
        const u8 = new Uint8Array(mem.buffer);
        const view = new DataView(mem.buffer);
        let total = 0;
        for (let i = 0; i < iovsLen; i++) {
          const ptr = view.getInt32(iovs + i * 8, true);
          const len = view.getInt32(iovs + i * 8 + 4, true);
          state.outputBuf.push(...u8.subarray(ptr, ptr + len));
          total += len;
        }
        view.setInt32(nwritten, total, true);
        return 0;
      },
      proc_exit: (code) => { throw new Error('WASI exit: ' + code); }
    };
  },

  /**
   * 调用裁定 (自动选择 wasm 或 js)
   */
  verdict(request) {
    if (this.mode === 'wasm' && this.wasmReady) {
      return this._verdictWasm(request);
    }
    return mqfVerdictEngine(request);
  },

  _verdictWasm(request) {
    // 调用 wasm 导出的 mqf_verdict
    // TODO: 实现 WASI stdin 写入
    return mqfVerdictEngine(request); // fallback for now
  }
};

// ============== 蜂窝套壳 ==============

class ShellNest {
  constructor() {
    this.nodes = [];
    this.rootId = -1;
  }

  wrap(name) {
    const id = this.nodes.length;
    this.nodes.push({ name, level: 0, parentId: -1, childIds: [], spaceRatio: 1.0 });
    if (this.rootId === -1) this.rootId = id;
    return id;
  }

  insert(parentId, childName) {
    if (parentId < 0 || parentId >= this.nodes.length) return -1;
    const parent = this.nodes[parentId];
    if (parent.childIds.length >= MAX_CHILD) return -1;
    const id = this.nodes.length;
    const node = {
      name: childName,
      level: parent.level + 1,
      parentId,
      childIds: [],
      spaceRatio: parent.spaceRatio / MAX_CHILD
    };
    this.nodes.push(node);
    parent.childIds.push(id);
    return id;
  }

  honeycomb(level) {
    if (this.nodes.length === 0) this.wrap('root');
    if (level <= 0) return;
    // 从 root 开始每层填 6 个 child
    const fill = (parentId, remaining) => {
      if (remaining <= 0) return;
      const parent = this.nodes[parentId];
      while (parent.childIds.length < MAX_CHILD && this.nodes.length < 500) {
        const childName = `${parent.name}.${parent.childIds.length}`;
        const childId = this.insert(parentId, childName);
        fill(childId, remaining - 1);
      }
    };
    fill(this.rootId, level);
  }
}

// ============== UI 控制器 ==============

class MqfShellController {
  constructor() {
    this.history = [];
    this.shellNest = new ShellNest();
    this.shellNest.honeycomb(2);
  }

  async init() {
    const mode = await MqfCore.init();
    this._log(`mode=${mode} (${mode === 'wasm' ? 'WASI 内核' : 'JS 回退'})`);

    this._bindUI();
    this._renderShell();
    this._loadPersistedHistory();
    this._runDefaultVerdict();
  }

  _bindUI() {
    const $ = (sel) => document.querySelector(sel);
    this.el = {
      entity: $('#input-entity'),
      complianceA: $('#input-compliance-a'),
      complianceB: $('#input-compliance-b'),
      force: $('#input-force'),
      angle: $('#input-angle'),
      forceVal: $('#val-force'),
      angleVal: $('#val-angle'),
      verdictIcon: $('#verdict-icon'),
      verdictLabel: $('#verdict-label'),
      verdictSub: $('#verdict-sub'),
      verdictTicks: $('#verdict-ticks'),
      mirrorA: $('#mirror-a'),
      mirrorB: $('#mirror-b'),
      mirrorGap: $('#mirror-gap'),
      shellNodes: $('#shell-nodes'),
      history: $('#history-list'),
      terminal: $('#terminal-output'),
      wasmBadge: $('#wasm-mode-badge'),
      runBtn: $('#btn-run'),
      presetIos: $('#btn-preset-ios'),
      presetAndroid: $('#btn-preset-android'),
      presetBank: $('#btn-preset-bank'),
      honeycombBtn: $('#btn-honeycomb')
    };

    // 滑块实时显示
    this.el.force.addEventListener('input', () => {
      this.el.forceVal.textContent = this.el.force.value;
    });
    this.el.angle.addEventListener('input', () => {
      this.el.angleVal.textContent = this.el.angle.value;
    });

    // 按钮
    this.el.runBtn.addEventListener('click', () => this.runVerdict());
    this.el.entity.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') this.runVerdict();
    });

    // 预设按钮
    this.el.presetIos.addEventListener('click', () => this._loadPreset('ios', 0.9, 0.85, 15, 45));
    this.el.presetAndroid.addEventListener('click', () => this._loadPreset('android', 0.3, 0.7, 2, 15));
    this.el.presetBank.addEventListener('click', () => this._loadPreset('bank', 0.85, 0.80, 12, 60));

    // 蜂窝填充
    this.el.honeycombBtn.addEventListener('click', () => {
      this.shellNest.honeycomb(1);
      this._renderShell();
      this._log('sys: 蜂窝填充 +1 层');
    });

    // WASM badge
    if (MqfCore.mode === 'wasm') {
      this.el.wasmBadge.textContent = 'WASM';
      this.el.wasmBadge.classList.remove('fallback');
    } else {
      this.el.wasmBadge.textContent = 'JS 回退';
      this.el.wasmBadge.classList.add('fallback');
    }
  }

  _loadPreset(entity, a, b, force, angle) {
    this.el.entity.value = entity;
    this.el.complianceA.value = a;
    this.el.complianceB.value = b;
    this.el.force.value = force;
    this.el.angle.value = angle;
    this.el.forceVal.textContent = force;
    this.el.angleVal.textContent = angle;
    this.runVerdict();
  }

  runVerdict() {
    const entity = this.el.entity.value.trim() || 'unknown';
    const complianceA = parseFloat(this.el.complianceA.value) || 0.5;
    const complianceB = parseFloat(this.el.complianceB.value) || 0.5;
    const force = parseFloat(this.el.force.value) || 10;
    const angle = parseFloat(this.el.angle.value) || 45;

    const result = MqfCore.verdict({ entity, complianceA, complianceB, force, angle });

    this._displayVerdict(result);
    this._addToHistory(result);
    this._log(`out: ${JSON.stringify(result.mqf_verdict)}`);
  }

  _displayVerdict(result) {
    const v = result.mqf_verdict;
    const trusted = v.trust === 'TRUSTED';

    // 图标 + 标签
    this.el.verdictIcon.textContent = trusted ? '★' : '○';
    this.el.verdictIcon.className = 'mqf-verdict-icon ' + (trusted ? 'trusted' : 'rejected');
    this.el.verdictLabel.textContent = trusted ? 'TRUSTED (信赖)' : 'REJECTED (否决)';
    this.el.verdictLabel.style.color = trusted ? 'var(--mqf-ok)' : 'var(--mqf-danger)';
    this.el.verdictSub.textContent = result.philosophy;
    this.el.verdictTicks.textContent =
      `ticks=${v.five_yuan.ticks} · ${v.five_yuan.occupied} · 角度=${v.five_yuan.angle}° · 交集=${v.six_yuan.intersection}`;

    // 六元可视化
    if (this.el.mirrorA) this.el.mirrorA.textContent = `A: ${result.mqf_verdict.confidence.toFixed(2)}`;
    if (this.el.mirrorB) this.el.mirrorB.textContent = `B: ${result.mqf_verdict.confidence.toFixed(2)}`;
    if (this.el.mirrorGap) {
      this.el.mirrorGap.textContent = v.mirror_alignment === 'ALIGNED' ? '≈ 对齐' : '≠ 偏移';
      this.el.mirrorGap.className = v.mirror_alignment === 'ALIGNED' ? 'mqf-mirror-gap aligned' : 'mqf-mirror-gap offset';
    }
  }

  _addToHistory(result) {
    this.history.unshift(result);
    if (this.history.length > 50) this.history.pop();
    this._renderHistory();
    this._persistHistory();
  }

  /**
   * 持久化历史 — v1.10+ 支持 mqfNative (Android/iOS), 回退 localStorage
   */
  _persistHistory() {
    const data = JSON.stringify(this.history.slice(0, 20));
    if (window.mqfNative && typeof window.mqfNative.save === 'function') {
      try { window.mqfNative.save('verdict_history', data); } catch(e) {}
    } else {
      try { localStorage.setItem('mqf_verdict_history', data); } catch(e) {}
    }
  }

  _loadPersistedHistory() {
    let data = null;
    if (window.mqfNative && typeof window.mqfNative.load === 'function') {
      try { data = window.mqfNative.load('verdict_history'); } catch(e) {}
    }
    if (!data) {
      try { data = localStorage.getItem('mqf_verdict_history'); } catch(e) {}
    }
    if (data) {
      try {
        this.history = JSON.parse(data);
        this._renderHistory();
      } catch(e) {}
    }
  }

  _renderHistory() {
    const html = this.history.slice(0, 20).map(r => {
      const trusted = r.mqf_verdict.trust === 'TRUSTED';
      return `<div class="history-item ${trusted ? 'trusted' : 'rejected'}">
        <span class="h-entity">${r.entity}</span>
        <span class="h-meta">${r.mqf_verdict.trust} · ticks=${r.mqf_verdict.five_yuan.ticks} · ${r.mqf_verdict.five_yuan.occupied}</span>
      </div>`;
    }).join('');
    this.el.history.innerHTML = html || '<div style="color:#6b7a99;font-size:0.7rem">暂无裁定记录</div>';
  }

  _renderShell() {
    const nodes = this.shellNest.nodes.slice(0, 50);
    const html = nodes.map(n => {
      const indent = '&nbsp;'.repeat(n.level * 2);
      return `<div class="mqf-shell-node" title="level=${n.level} ratio=${n.spaceRatio.toFixed(3)}">${indent}${n.name} (l${n.level})</div>`;
    }).join('');
    this.el.shellNodes.innerHTML = html;
  }

  _log(msg) {
    const time = new Date().toLocaleTimeString();
    const line = document.createElement('div');
    line.className = 'line out';
    line.textContent = `[${time}] ${msg}`;
    this.el.terminal.appendChild(line);
    this.el.terminal.scrollTop = this.el.terminal.scrollHeight;
  }

  _runDefaultVerdict() {
    this._loadPreset('ios', 0.9, 0.85, 15, 45);
    this._log('sys: 民联浏览器壳 v1.7 已就绪');
    this._log('sys: 加载 WASI 裁定内核...');
  }
}

// ============== 启动 ==============
document.addEventListener('DOMContentLoaded', () => {
  window.mqfShell = new MqfShellController();
  window.mqfShell.init();
});

// 导出供外部调用 (Node / WebView / 测试环境)
if (typeof module !== 'undefined') {
  module.exports = { mqfVerdictEngine, MqfCore, ShellNest, MqfShellController, MQF_SHELL_VERSION };
}
