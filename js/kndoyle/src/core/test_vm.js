const { UniCoreVM, INSTRUCTIONS } = require('./unicore_vm.js');
const { SHENGMU_LOWER, YUNMU_LOWER, SHENGDIAO, NUMBERS } = require('./unicore_symbols.js');

console.log('================================================================================');
console.log('UniCore 完整虚拟机测试');
console.log('================================================================================');
console.log();

const vm = new UniCoreVM();

console.log('--- 测试指令执行 ---');
const program = [
  SHENGDIAO[0],  // aa1 → 堆栈
  YUNMU_LOWER[0],  // ꞟ → 堆栈
  SHENGMU_LOWER[0],  // b → 堆栈
  
  // 此时堆栈: [aa1, ꞟ, b]
  
  INSTRUCTIONS.MOVE,  // b → 格子中心
  INSTRUCTIONS.LOAD,  // ꞟ → 内存
  INSTRUCTIONS.LOAD,  // aa1 → 内存
  INSTRUCTIONS.ZHUI,
  INSTRUCTIONS.JING,
  INSTRUCTIONS.CLUSTER
];

console.log('执行程序...');
vm.execute(program);
console.log();

console.log('--- 堆栈状态 ---');
console.log(vm.getStack());
console.log();

console.log('--- 内存状态 ---');
console.log(vm.getMemory());
console.log();

console.log('--- 格子状态 ---');
const grid = vm.getGrid();
console.log('中心: ' + grid.center);
console.log();

console.log('================================================================================');
console.log('虚拟机测试完成！');
console.log('================================================================================');
