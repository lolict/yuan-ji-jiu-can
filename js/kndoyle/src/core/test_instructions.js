const { INSTRUCTIONS, InstructionEncoder, SymbolVM } = require('./unicore_instructions.js');

console.log('================================================================================');
console.log('UniCore 指令集测试 - 用户定义符号');
console.log('================================================================================');
console.log();

console.log('--- 指令集示例 ---');
const sampleInsts = ['b', 'p', 'm', 'B', 'Ə', 'ꙮ', 'aa1', 'aa2'];
for (const sym of sampleInsts) {
  const inst = INSTRUCTIONS[sym];
  console.log(sym + ' -> ' + inst.op + ' (' + inst.desc + ')');
}
console.log();

console.log('--- 编码器测试 ---');
const encoder = new InstructionEncoder();
const symbols = ['b', 'm', 'B', 'Ə', 'ꙮ'];
const insts = encoder.compose(symbols);
console.log('符号序列:', symbols.join(' '));
console.log('指令序列:', insts.map(i => i.op).join(' -> '));
console.log();

console.log('--- 虚拟机测试 ---');
const vm = new SymbolVM();
vm.execute(['b', 'm', 'B', 'Ə', 'ꙮ']);
console.log('执行后寄存器:', JSON.stringify(vm.status().registers));
console.log();

console.log('--- 完整符号覆盖 ---');
console.log('声母指令:');
for (const sm of ['b', 'p', 'm', 'f', 'd', 't', 'n', 'l']) {
  console.log('  ' + sm + ' = ' + INSTRUCTIONS[sm].op);
}
console.log();

console.log('韵母指令:');
for (const ym of ['ə', 'ä', 'ɘ', 'ö', 'ѧ', 'ʉ']) {
  console.log('  ' + ym + ' = ' + INSTRUCTIONS[ym].op);
}
console.log();

console.log('大写韵母阵法:');
for (const ym of ['Ə', 'Ä', 'Ǝ', 'Ɯ', 'Ö', 'Я', 'Ж', 'ꙮ', 'М']) {
  console.log('  ' + ym + ' = ' + INSTRUCTIONS[ym].op);
}
console.log();

console.log('================================================================================');
console.log('指令集测试完成！');
console.log('================================================================================');
