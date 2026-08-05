/**
 * 囡囧共生 · 自举加载器
 * Bootstrap Loader
 *
 * 作用:把 yy_compiler.self.yy(YinYang 语法源)喂给 yy_compiler.js 处理,
 * 产出 _yy_compiler.auto.js,即一个"能编译 YinYang 语法的 JS 编译器"。
 *
 * 验证自举成功:
 *   1) node yy_bootstrap.js             → 产出 _yy_compiler.auto.js
 *   2) 手工检查 _yy_compiler.auto.js    → 它应该是合法的可执行 JS
 *   3) node _yy_compiler.auto.js < yy_compiler.self.yy → 应该产生相同输出
 *
 * 若步骤 3 的输出 == 步骤 2 的文件内容,则自举闭合。
 */

'use strict';
const fs = require('fs');
const path = require('path');
const { compile, tokenize, parse } = require('./yy_compiler.js');

const SELF_SRC = path.join(__dirname, 'yy_compiler.self.yy');
const AUTO_OUT = path.join(__dirname, '_yy_compiler.auto.js');

console.log('=== 囡囧共生 自举加载器 ===\n');

// 1. 读 YinYang 源
const src = fs.readFileSync(SELF_SRC, 'utf8');
console.log(`已读 ${SELF_SRC}(${src.length} bytes)`);

// 2. tokenize → parse
const tokens = tokenize(src);
const ast = parse(tokens);
console.log(`tokens: ${tokens.length}, ast nodes: ${ast.filter(x=>x!==null).length}`);

// 3. 编译为 JS
const jsOut = compile(src);
console.log(`编译输出: ${jsOut.length} chars`);

// 4. 写入
fs.writeFileSync(AUTO_OUT, jsOut);
console.log(`✅ 自举产出: ${AUTO_OUT}`);

console.log('\n=== 自举步骤完成 ===');
console.log('下一步: 让 _yy_compiler.auto.js 再吃自己一次,验证固定点');
