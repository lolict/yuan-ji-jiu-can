const { FourDimNonCarrySystem, ContainerMachineCodeAdapter } = require('./unicore_four_dim.js');

console.log('================================================================================');
console.log('UniCore 四维不进位体系 + 等效维度映射测试');
console.log('================================================================================');
console.log();

console.log('--- 测试四维不进位体系 ---');
const system = new FourDimNonCarrySystem();

console.log('初始状态: ' + system.getState());
console.log();

for (let i = 0; i < 20; i++) {
  const state = system.increment();
  const mapping = system.getEquivalentMapping();
  const machineCode = system.getMachineCodeMapping();
  
  console.log('递增' + (i+1) + ' → ' + state);
  console.log('  等效映射: 数值=' + mapping.value + 
              ', 层级=' + mapping.equivalentLayer + 
              ', 集装箱=' + mapping.equivalentContainer + 
              ', 机器码位=' + mapping.equivalentBit);
  console.log('  机器码映射: ' + machineCode.fullBits);
  console.log();
}

console.log();
console.log('--- 测试机器码适配器 ---');
const adapter = new ContainerMachineCodeAdapter();
for (let i = 0; i < 10; i++) {
  adapter.increment();
  const state = adapter.getState();
  const value = adapter.getEquivalentMachineCodeValue();
  const machineCode = adapter.getMachineCodeMapping();
  
  console.log('递增' + (i+1) + ' → ' + state);
  console.log('  等效数值: 十进制=' + value.decimal + 
              ', 二进制=' + value.binary + 
              ', 集装箱=' + value.containerEquivalent + 
              ', 层级=' + value.layerEquivalent);
  console.log('  机器码: ' + machineCode.fullBits);
  console.log();
}

console.log('================================================================================');
console.log('四维不进位体系 + 等效维度映射测试完成！');
console.log('================================================================================');
