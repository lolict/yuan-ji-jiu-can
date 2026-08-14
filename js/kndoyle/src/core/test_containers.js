const { CONTAINERS, ContainerEncoder } = require('./unicore_containers.js');
const { SHENGMU_LOWER, YUNMU_LOWER, SHENGDIAO, NUMBERS } = require('./unicore_symbols.js');

console.log('================================================================================');
console.log('UniCore 集装箱编码系统');
console.log('================================================================================');
console.log();

console.log('--- 集装箱 ---');
console.log('集装箱1 (声母): ' + CONTAINERS[1].count + '个 - ' + SHENGMU_LOWER.join(', '));
console.log('集装箱2 (韵母): ' + CONTAINERS[2].count + '个');
console.log('集装箱3 (声调): ' + CONTAINERS[3].count + '个 - ' + SHENGDIAO.slice(0,10).join(', ') + ' ...');
console.log('集装箱4 (数字): ' + CONTAINERS[4].count + '个 - ' + NUMBERS.join(', '));
console.log();

console.log('--- 测试编码/解码 ---');
const encoder = new ContainerEncoder();

let testIndex = 5;
let symbol = encoder.encode(1, testIndex);
console.log('集装箱1, 索引' + testIndex + ' → ' + symbol);

symbol = encoder.encode(2, testIndex);
console.log('集装箱2, 索引' + testIndex + ' → ' + symbol);

symbol = encoder.encode(3, testIndex);
console.log('集装箱3, 索引' + testIndex + ' → ' + symbol);

symbol = encoder.encode(4, testIndex);
console.log('集装箱4, 索引' + testIndex + ' → ' + symbol);

console.log();

console.log('--- 测试递增进位 ---');
let carrier = [0, 0, 0, 0];
console.log('初始: ' + carrier);
for (let i = 0; i < 10; i++) {
  carrier = encoder.increment(carrier);
  const symbols = encoder.indicesToSymbols(carrier);
  console.log('递增' + (i+1) + ': ' + carrier + ' → ' + symbols);
}
console.log();

console.log('================================================================================');
console.log('集装箱系统完成！');
console.log('================================================================================');
