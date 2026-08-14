/**
 * UniCore Foundation WASM Module
 * 
 * 可编译为WASM的核心代码
 * 使用 wasm-pack 或 emscripten 编译
 * 
 * 编译命令：
 * wasm-pack build --target web
 * 或
 * emcc -o unicore.wasm unicore_foundation_wasm.c
 */

// ==================== WASM导出函数 ====================

// 初始化内存
function initMemory(size) {
    return new Uint8Array(size);
}

// CBC格式解析
function parseCBC(dataPtr, dataLen) {
    const data = new Uint8Array(memory.buffer, dataPtr, dataLen);
    const parser = new CBCParser();
    
    try {
        const result = parser.parse(data);
        return result.gridCount;
    } catch (e) {
        return -1;
    }
}

// 稀疏数编码
function sparseEncode(smIdx, ymIdx, sdIdx) {
    return (smIdx * RADIX.yunmu + ymIdx) * RADIX.shengdiao + sdIdx;
}

// 稀疏数解码
function sparseDecode(code) {
    const sdIdx = code % RADIX.shengdiao;
    const ymIdx = Math.floor(code / RADIX.shengdiao) % RADIX.yunmu;
    const smIdx = Math.floor(code / (RADIX.yunmu * RADIX.shengdiao));
    
    // 打包成32位：smIdx(8位) | ymIdx(8位) | sdIdx(16位)
    return (smIdx << 24) | (ymIdx << 16) | sdIdx;
}

// VM执行
function vmExecute(programPtr, programLen, cycles) {
    const vm = new UniCoreVM();
    const program = new Uint8Array(memory.buffer, programPtr, programLen);
    vm.loadProgram(program);
    vm.run(cycles);
    return vm.registers[0];
}

// 创建格子
function createGrid(smIdx, ymIdx, sdIdx) {
    const vm = new UniCoreVM();
    vm.grids.push({
        shengmuIndex: smIdx,
        yunmuIndex: ymIdx,
        shengdiaoIndex: sdIdx,
        walls: new Uint8Array(6),
        corners: new Uint16Array(8),
        connections: new Uint16Array(6)
    });
    return vm.grids.length - 1;
}

// ==================== WASI调用 ====================

function wasi_fd_write(fd, iovs, iovs_len) {
    const adapter = new WASIAdapter(new UniCoreVM());
    return adapter.fd_write(fd, iovs, iovs_len);
}

function wasi_clock_time_get(clock_id, precision, time) {
    const adapter = new WASIAdapter(new UniCoreVM());
    return adapter.clock_time_get(clock_id, precision, time);
}

// ==================== 导出供WASM使用 ====================

if (typeof wasm !== 'undefined') {
    wasm.exports = {
        initMemory,
        parseCBC,
        sparseEncode,
        sparseDecode,
        vmExecute,
        createGrid,
        wasi_fd_write,
        wasi_clock_time_get
    };
}

// ==================== 测试脚本 ====================
function runTests() {
    console.log('=== UniCore Foundation Tests ===');
    
    // 测试CBC解析
    const cbcData = CBCParser.encode([{
        shengmuIndex: 0,
        yunmuIndex: 0,
        shengdiaoIndex: 0,
        walls: [0, 0, 0, 0, 0, 0],
        corners: [0, 0, 0, 0, 0, 0, 0, 0],
        connections: [0, 0, 0, 0, 0, 0]
    }]);
    const parser = new CBCParser();
    const result = parser.parse(cbcData);
    console.log('CBC Parse:', result.gridCount === 1 ? 'PASS' : 'FAIL');
    
    // 测试稀疏数编码解码
    const code = SparseAlgebra.encode('B', 'A', 'A0');
    const decoded = SparseAlgebra.decode(code);
    console.log('Sparse Encode/Decode:', decoded.shengmu === 'B' && decoded.yunmu === 'A' ? 'PASS' : 'FAIL');
    
    // 测试VM
    const vm = new UniCoreVM();
    const program = [0x00, 0x01]; // NOP, HALT
    vm.loadProgram(program);
    vm.run();
    console.log('VM Execution:', !vm.running ? 'PASS' : 'FAIL');
    
    console.log('=== Tests Complete ===');
}

// 在Node.js环境运行测试
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        initMemory,
        parseCBC,
        sparseEncode,
        sparseDecode,
        vmExecute,
        createGrid,
        runTests
    };
    
    // 如果直接运行，执行测试
    if (require.main === module) {
        runTests();
    }
} else {
    window.UniCoreWASM = {
        initMemory,
        parseCBC,
        sparseEncode,
        sparseDecode,
        vmExecute,
        createGrid,
        runTests
    };
}
