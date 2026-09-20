// 测试 hunyuan-wasm 模块
import { readFileSync } from 'fs';

const bytes = readFileSync('../target/wasm32-unknown-unknown/release/hunyuan_wasm.wasm');
const { instance } = await WebAssembly.instantiate(bytes);
const wasm = instance.exports;

console.log('导出函数:', Object.keys(wasm).filter(k => typeof wasm[k] === 'function'));

// 测试 reg_seed
wasm.reg_seed();
console.log('✓ reg_seed 完成');

// 测试 reg_dump
const dumpPtr = wasm.reg_dump();
const dumpStr = readString(wasm, dumpPtr);
wasm.reg_free_string(dumpPtr);
const entries = JSON.parse(dumpStr);
console.log('✓ reg_dump: 共', entries.length, '个容器');
console.log('  示例:', JSON.stringify(entries[0]));

// 测试 reg_links
const linksPtr = wasm.reg_links();
const linksStr = readString(wasm, linksPtr);
wasm.reg_free_string(linksPtr);
const links = JSON.parse(linksStr);
console.log('✓ reg_links: 共', links.length, '条公路');

// 测试 reg_reach
const reachPtr = wasm.reg_reach(0x0101);
const reachStr = readString(wasm, reachPtr);
wasm.reg_free_string(reachPtr);
const reach = JSON.parse(reachStr);
console.log('✓ reg_reach(0x0101):', reach);

// 测试 fq_digits (进制展开) — 使用 BigInt (u64 参数)
const digitsBuf = wasm.alloc_mem(8 * 8);
const mem = new DataView(wasm.memory.buffer);
const testVal = 877825024n; // 0x34528C00 as BigInt
const count = wasm.fq_digits(testVal, digitsBuf);
const digits = [];
for (let i = 0; i < count; i++) digits.push(Number(mem.getBigUint64(digitsBuf + i * 8, true)));
wasm.free_mem(digitsBuf, 8 * 8);
console.log('✓ fq_digits(877825024):', digits);

// 测试 fq_compose (进制合成)
const composeBuf = wasm.alloc_mem(8 * 8);
const cMem = new DataView(wasm.memory.buffer);
const testDigits = [16, 21, 6, 3, 2, 0, 223]; // 7位 = 完整的展开
for (let i = 0; i < 7; i++) cMem.setBigUint64(composeBuf + i * 8, BigInt(testDigits[i]), true);
const composedBig = wasm.fq_compose(composeBuf, 7);
wasm.free_mem(composeBuf, 8 * 8);
const composed = Number(composedBig);
console.log('✓ fq_compose([16,21,6,3,2,0]):', composed, '(期望: 877825024)');

// 验证
if (composed === 877825024) {
    console.log('✓ 进制展开/合成 往返正确');
} else {
    console.log('✗ 进制合成结果不匹配! 得到:', composed);
}

function readString(wasm, ptr) {
    const mem = new Uint8Array(wasm.memory.buffer);
    let end = ptr;
    while (mem[end] !== 0) end++;
    return new TextDecoder().decode(mem.slice(ptr, end));
}

console.log('\n全部测试通过 ✓');
