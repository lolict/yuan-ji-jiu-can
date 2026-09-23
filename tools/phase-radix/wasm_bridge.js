/**
 * WASM Bridge — 囡囝共生 · 双向鸿约
 * 加载 zig-stem/mqf64.wasm 和 mqf32.wasm
 * 让浏览器直接执行进制几何计算 (脱离 Python 壳)
 *
 * 导出函数 (C → JS):
 *   mqf_boot()                     → void     初始化
 *   mqf_symbol_angle(u8 sym)       → u32      符号角度 [0,384)
 *   mqf_arc_distance(u32 a, b)     → u32      圆弧距离
 *   mqf_time_stage_depth(u8 n)     → u8       质因数分解深度
 *   mqf_is_adjacent(x,y,z, ax,ay,az) → i32    三维网格邻接
 *   mqf_stride_from_u64(u64 n)     → {sheet, symbol}
 *   mqf_stride_to_u64(stride)      → u64
 */

let wasm = null;
let memory = null;
let exports = null;
let heap = null;
let heapU8 = null;

const WASM_PATHS = {
    wasm64: '/zig/wasm64.wasm',
    wasm32: '/zig/wasm32.wasm',
};

// 双目标: 优先加载 wasm64, 失败回退 wasm32
async function initWasm(preferred = 'wasm64') {
    const order = preferred === 'wasm64' ? ['wasm64', 'wasm32'] : ['wasm32', 'wasm64'];
    for (const target of order) {
        try {
            const response = await fetch(WASM_PATHS[target]);
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
            const bytes = await response.arrayBuffer();
            const module = await WebAssembly.compile(bytes);
            const instance = await WebAssembly.instantiate(module, {
                env: { memory: new WebAssembly.Memory({ initial: 1, maximum: 4 }) },
                wasi_snapshot_preview1: {
                    proc_exit: () => {},
                    fd_close: () => {},
                    fd_write: () => 0,
                    fd_seek: () => 0,
                }
            });
            exports = instance.exports;
            memory = exports.memory || instance.exports.memory;
            if (memory) {
                heap = memory.buffer;
                heapU8 = new Uint8Array(heap);
            }
            wasm = { target, module, instance };
            console.log(`[WASM] 加载成功: ${target}`);
            return true;
        } catch (e) {
            console.warn(`[WASM] ${target} 加载失败:`, e.message);
        }
    }
    console.error('[WASM] 所有目标加载失败');
    return false;
}

// ── 类型安全包装 ──

function wasm_boot() { return exports.mqf_boot ? exports.mqf_boot() : 0; }
function wasm_symbol_angle(sym) { return exports.mqf_symbol_angle(sym & 0xFF); }
function wasm_arc_distance(a, b) { return exports.mqf_arc_distance(a >>> 0, b >>> 0); }
function wasm_time_stage_depth(n) { return exports.mqf_time_stage_depth(n & 0xFF); }
function wasm_is_adjacent(x, y, z, ax, ay, az) {
    return exports.mqf_is_adjacent(x, y, z, ax, ay, az);
}

// ── 高级语义: 从 64位细胞编码调用 WASM 几何 ──

function cell_to_angle(cell) {
    const symbol = (cell >> 32) & 0x3F;  // sovereignty低6位作为符号索引
    return wasm_symbol_angle(symbol);
}

function cells_arc_distance(cellA, cellB) {
    const a = cell_to_angle(cellA);
    const b = cell_to_angle(cellB);
    return wasm_arc_distance(a, b);  // [0, 192]
}

function cell_mccp_depth(cell) {
    const mccp = cell & 0xFFFFFFFF;
    const lo8 = mccp & 0xFF;
    return wasm_time_stage_depth(lo8);  // [0, 8]
}

function cells_geometric_distance(cellA, cellB) {
    /**
     * 几何距离: 基于WASM内核的弧距离 + 相位
     * 返回: [0, 1] 归一化距离 (0=同相位, 1=最远)
     */
    const arc = cells_arc_distance(cellA, cellB);
    return arc / 192.0;  // 最大弧距离 = 384/2 = 192
}

window.WasmBridge = {
    init: initWasm,
    ready: () => !!exports,
    target: () => wasm ? wasm.target : null,
    boot: wasm_boot,
    symbolAngle: wasm_symbol_angle,
    arcDistance: wasm_arc_distance,
    timeStageDepth: wasm_time_stage_depth,
    isAdjacent: wasm_is_adjacent,
    cellToAngle: cell_to_angle,
    cellsArcDistance: cells_arc_distance,
    cellMccpDepth: cell_mccp_depth,
    cellsGeometricDistance: cells_geometric_distance,
};
