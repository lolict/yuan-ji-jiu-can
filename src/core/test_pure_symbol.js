const { SHENGMU, YUNMU, SHENGDIAO, NativeOrder, PureGrid, PureHuren, PureJie, PureZhenfa, PureModules, PureCBC } = require('./unicore_pure_symbol.js');

console.log('='.repeat(80));
console.log('UniCore 纯符号底层核心测试 - 历史正确符号版');
console.log('='.repeat(80));
console.log();
console.log('声母数量:', SHENGMU.length, '(应为48)');
console.log('韵母数量:', YUNMU.length, '(应为64)');
console.log('声调数量:', SHENGDIAO.length, '(应为170)');
console.log();
console.log('声母:', SHENGMU.slice(0, 8).join(', '));
console.log('韵母:', YUNMU.slice(0, 8).join(', '));
console.log('声调:', SHENGDIAO.slice(0, 8).join(', '));
console.log('声调最后:', SHENGDIAO[169]);
console.log();

let passCount = 0;
let failCount = 0;

function test(name, passed) {
  if (passed) {
    console.log('✅ ' + name);
    passCount++;
  } else {
    console.log('❌ ' + name);
    failCount++;
  }
}

console.log('--- 测试1：原生顺序判定 ---');
test('声母B在P前面', NativeOrder.shengmuFirst('B', 'P') === true);
test('韵母C在W1前面', NativeOrder.yunmuFirst('C', 'W1') === true);
test('声调DA1在DB1前面', NativeOrder.shengdiaoFirst('DA1', 'DB1') === true);
test('下一个声母', NativeOrder.nextShengmu('B') === 'P');
test('下一个韵母', NativeOrder.nextYunmu('C') === 'W1');
test('下一个声调', NativeOrder.nextShengdiao('DA1') === 'DB1');
console.log();

console.log('--- 测试2：三维格子 ---');
const grid1 = new PureGrid('B', 'C', 'DA1');
test('创建格子', grid1.shengmu === 'B');
test('写入中心', grid1.write('center', '汉字本体'));
test('写入前墙', grid1.write('walls.front', '声母B'));
test('写入内角', grid1.write('inner_corners.ne_fl', '本义'));
grid1.activateCenter();
test('激活中心', grid1.positions.center.active === true);
console.log();

console.log('--- 测试3：阶乘运算 ---');
const jie = new PureJie();
const r1 = jie.jieCheng('B', 'C', 'DA1');
test('正常阶乘进声调', r1.shengdiao === 'DB1');
const r2 = jie.jieCheng('B', 'C', 'DN7');
test('声调用完进韵母', r2.yunmu === 'W1');
console.log();

console.log('--- 测试4：互认系统 ---');
const huren = new PureHuren();
const gA = new PureGrid('B', 'C', 'DA1');
const gB = new PureGrid('P', 'C', 'DA1');
huren.register('A', gA);
huren.register('B', gB);
const hResult = huren.huren('A', 'B');
test('互认判定', hResult.result === true);
test('B权级高于P', hResult.higher === gA);
console.log();

console.log('--- 测试5：模块锚点 ---');
const modules = new PureModules();
modules.activate('mll');
modules.activate('gtt');
test('mll模块激活', modules.modules.mll.active === true);
test('gtt模块激活', modules.modules.gtt.active === true);
console.log();

console.log('--- 测试6：汉字积木编码 ---');
const cbc = new PureCBC();
const charGrid = cbc.storeChar('好', 'P', 'DA3', {
  strokes: '横折折折钩',
  radical: '女',
  component: '子',
  original: '美好',
  evolution: '甲骨文→金文',
  cause: '女子貌美',
  right_wrong: '对',
  yinyang: '阴',
  closeness: '亲',
  boundary: '内',
  synonym: '佳',
  antonym: '坏',
  ancient: '好',
  related: '美',
  derived: '爱好'
});
test('存储汉字', charGrid !== null);
const traced = cbc.traceback('好');
test('回溯汉字', traced !== null);
test('回溯偏旁', traced.radical === '女');
test('回溯本义', traced.original === '美好');
console.log();

console.log('--- 测试7：阵法系统 ---');
const zhenfa = new PureZhenfa();
const zCenter = new PureGrid('B', 'C', 'DA1');
const zResult = zhenfa.zhuiZhen(zCenter, 2);
test('锥阵创建', zResult.length >= 1);
const gX = new PureGrid('B', 'C', 'DA1');
const gY = new PureGrid('P', 'C', 'DB1');
zhenfa.jingxiangZhen(gX, gY);
test('镜像阵', gX.positions.walls.front.content === gY);
console.log();

console.log('='.repeat(80));
console.log('测试总结：');
console.log('  通过：' + passCount);
console.log('  失败：' + failCount);
console.log('='.repeat(80));

if (failCount === 0) {
  console.log('\n所有测试通过！');
  console.log('符号对齐完成：');
  console.log('  - 48声母: B, P, M, F, D, T, N, L, ...');
  console.log('  - 64韵母: C, W1-W6, J1-J8, L1-L6, E1-E48');
  console.log('  - 170声调: DA1, DB1, DC1, ... DN7');
}
