const { SHENGMU, YUNMU, SHENGDIAO, NUMBERS, RADIX, SHENGMU_LOWER, SHENGMU_UPPER, YUNMU_LOWER, YUNMU_UPPER, PRIORITY } = require('./unicore_symbols.js');

console.log('================================================================================');
console.log('UniCore 顺口溜排序版符号体系');
console.log('================================================================================');
console.log();

console.log('--- 符号数量 ---');
console.log('声母: ' + RADIX.shengmu + ' (48)');
console.log('韵母: ' + RADIX.yunmu + ' (64)');
console.log('声调: ' + RADIX.shengdiao + ' (170)');
console.log('数字: ' + RADIX.number + ' (17)');
console.log();

console.log('--- 32个韵母（顺口溜顺序）---');
console.log('序号 | 韵母 | 小写 | 大写');
const yunmuNames = ['ai','ei','ui','ao','ou','iu','ie','ue','er','an','en','in','un','vn','ang','eng','ing','ong','ia','ua','uo','iao','uai','ian','uan','üan','iang','uang','ueng','iong','iai','oi'];
for (let i = 0; i < 32; i++) {
  console.log((i+1) + ' | ' + yunmuNames[i] + ' | ' + YUNMU_LOWER[i] + ' | ' + YUNMU_UPPER[i]);
}
console.log();

console.log('--- 优先级 ---');
console.log('税率效率优先: ' + PRIORITY.byCount.join(' > '));
console.log('人类认知优先: ' + PRIORITY.byHuman.join(' > '));
console.log('当前使用: ' + PRIORITY.current);
console.log();

console.log('--- 声调示例 ---');
console.log(SHENGDIAO.slice(0,10).join(', ') + ' ... ' + SHENGDIAO.slice(160,170).join(', '));
console.log();

console.log('--- 24个小写声母 ---');
console.log(SHENGMU_LOWER.join(', '));
console.log();

console.log('--- 24个大写声母 ---');
console.log(SHENGMU_UPPER.join(', '));
console.log();

console.log('================================================================================');
console.log('顺口溜排序版符号对齐完成！');
console.log('================================================================================');
