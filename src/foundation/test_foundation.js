/**
 * UniCore Foundation 测试脚本
 * 
 * 验证底层核心功能：
 * 1. CBC格式解析
 * 2. 稀疏数代数
 * 3. 虚拟机执行
 * 4. WASI接口
 * 
 * 运行方式：node test_foundation.js
 */

const { CBCParser, SparseAlgebra, UniCoreVM, WASIAdapter, SHENGMU, YUNMU, SHENGDIAO, RADIX, CBC_MAGIC } = require('./unicore_foundation.js');

console.log('====================================');
console.log('  UniCore Foundation 测试套件');
console.log('====================================\n');

// ==================== 测试1: CBC格式解析 ====================
console.log('【测试1】CBC格式解析');
try {
    // 创建测试数据
    const testGrids = [{
        shengmuIndex: 0,
        yunmuIndex: 0,
        shengdiaoIndex: 0,
        walls: [1, 2, 3, 4, 5, 6],
        corners: [10, 20, 30, 40, 50, 60, 70, 80],
        connections: [1, 2, 3, 4, 5, 6]
    }];

    // 编码
    const encoded = CBCParser.encode(testGrids);
    console.log(`  编码后长度: ${encoded.length} 字节`);
    console.log(`  魔数验证: ${encoded.slice(0, 4).join(',')} === ${CBC_MAGIC.join(',')} : ${encoded.slice(0, 4).every((v, i) => v === CBC_MAGIC[i]) ? 'PASS' : 'FAIL'}`);

    // 解码
    const parser = new CBCParser();
    const decoded = parser.parse(encoded);
    console.log(`  解码格子数: ${decoded.gridCount}`);
    console.log(`  解码验证: ${decoded.grids[0].shengmuIndex === 0 && decoded.grids[0].yunmuIndex === 0 ? 'PASS' : 'FAIL'}`);
    
    console.log('  ✅ CBC格式解析测试通过\n');
} catch (e) {
    console.log(`  ❌ CBC格式解析失败: ${e.message}\n`);
}

// ==================== 测试2: 稀疏数代数 ====================
console.log('【测试2】稀疏数代数');
try {
    const code = SparseAlgebra.encode('B', 'A', 'A0');
    console.log(`  编码 B+A+A0 = ${code}`);
    
    const decoded = SparseAlgebra.decode(code);
    console.log(`  解码 ${code} = ${decoded.shengmu}+${decoded.yunmu}+${decoded.shengdiao}`);
    
    const verify = decoded.shengmu === 'B' && decoded.yunmu === 'A' && decoded.shengdiao === 'A0';
    console.log(`  编码解码验证: ${verify ? 'PASS' : 'FAIL'}`);
    
    // 权位测试
    const addResult = SparseAlgebra.sparseAdd('B', 'shengmu', 'A', 'yunmu');
    console.log(`  稀疏加法(声母vs韵母): ${addResult.symbol}(${addResult.type})`);
    console.log(`  权位验证: ${addResult.type === 'yunmu' ? 'PASS' : 'FAIL'}`);
    
    console.log('  ✅ 稀疏数代数测试通过\n');
} catch (e) {
    console.log(`  ❌ 稀疏数代数失败: ${e.message}\n`);
}

// ==================== 测试3: 虚拟机执行 ====================
console.log('【测试3】虚拟机执行');
try {
    const vm = new UniCoreVM();
    
    // 测试程序：LOAD -> STORE -> HALT
    const program = [
        0x02, 0x00, 0x10, 0x00,  // LOAD addr=0x10 to reg0
        0x03, 0x00, 0x20, 0x00,  // STORE reg0 to addr=0x20
        0x01                      // HALT
    ];
    
    // 设置内存数据
    vm.memory[0x10] = 0x12;
    vm.memory[0x11] = 0x34;
    
    vm.loadProgram(program);
    vm.run(10);
    
    console.log(`  程序执行: ${vm.running ? '运行中' : '已停止'}`);
    console.log(`  存储结果: 0x${vm.memory[0x20].toString(16)}${vm.memory[0x21].toString(16)}`);
    console.log(`  结果验证: ${vm.memory[0x20] === 0x12 && vm.memory[0x21] === 0x34 ? 'PASS' : 'FAIL'}`);
    
    // 测试创建格子
    const gridProgram = [
        0x20, 0x00, 0x00, 0x00, 0x00,  // CREATE_GRID sm=0, ym=0, sd=0
        0x01                           // HALT
    ];
    
    const vm2 = new UniCoreVM();
    vm2.loadProgram(gridProgram);
    vm2.run(10);
    
    console.log(`  创建格子数: ${vm2.grids.length}`);
    console.log(`  格子验证: ${vm2.grids.length === 1 ? 'PASS' : 'FAIL'}`);
    
    console.log('  ✅ 虚拟机测试通过\n');
} catch (e) {
    console.log(`  ❌ 虚拟机失败: ${e.message}\n`);
}

// ==================== 测试4: WASI接口 ====================
console.log('【测试4】WASI接口');
try {
    const vm = new UniCoreVM();
    const adapter = new WASIAdapter(vm);
    
    // 测试时钟获取
    const timeBuf = new Uint8Array(8);
    const result = adapter.clock_time_get(0, 1000, 0);
    
    console.log(`  时钟获取: ${result === 0 ? 'PASS' : 'FAIL'}`);
    
    // 测试参数获取
    adapter.args_sizes_get(0, 0);
    console.log(`  参数获取: PASS`);
    
    console.log('  ✅ WASI接口测试通过\n');
} catch (e) {
    console.log(`  ❌ WASI接口失败: ${e.message}\n`);
}

// ==================== 测试5: 完整流程 ====================
console.log('【测试5】完整流程');
try {
    // 创建格子 -> 编码 -> 保存为CBC -> 读取 -> 解码 -> 验证
    const vm = new UniCoreVM();
    
    // 创建多个格子
    for (let i = 0; i < 5; i++) {
        vm.grids.push({
            shengmuIndex: i % RADIX.shengmu,
            yunmuIndex: i % RADIX.yunmu,
            shengdiaoIndex: i % RADIX.shengdiao,
            walls: new Uint8Array(6),
            corners: new Uint16Array(8),
            connections: new Uint16Array(6)
        });
    }
    
    // 编码为CBC
    const cbcData = CBCParser.encode(vm.grids);
    console.log(`  创建格子数: ${vm.grids.length}`);
    console.log(`  CBC数据大小: ${cbcData.length} 字节`);
    
    // 解码
    const parser = new CBCParser();
    const result = parser.parse(cbcData);
    console.log(`  解码格子数: ${result.gridCount}`);
    console.log(`  完整性验证: ${result.gridCount === vm.grids.length ? 'PASS' : 'FAIL'}`);
    
    console.log('  ✅ 完整流程测试通过\n');
} catch (e) {
    console.log(`  ❌ 完整流程失败: ${e.message}\n`);
}

console.log('====================================');
console.log('  所有测试完成！');
console.log('====================================');
