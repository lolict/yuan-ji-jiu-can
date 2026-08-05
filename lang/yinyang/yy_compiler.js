/**
 * 囡囧共生 阴阳三进制 → JS 编译器
 *
 * 把 (0 (expr...) ...) 语法编译为 JavaScript 代码,
 * 在浏览器壳 mqf-shell.js 中 import 使用。
 *
 * 模块模式:输出一段 JS 字符串,可 eval 或写入文件。
 *
 * 用法:
 *   const { compile } = require('./yy_compiler.js');
 *   const jsCode = compile('(0 0.85 0.90 15 45 (评) (判))');
 */

'use strict';

/**
 * 简单 tokenizer
 */
function tokenize(src) {
    const tokens = [];
    let i = 0;
    while (i < src.length) {
        const c = src[i];
        if (c === ';' ) { while (i < src.length && src[i] !== '\n') i++; continue; }
        if (/\s/.test(c)) { i++; continue; }
        if (c === '(' || c === ')') { tokens.push({ type: c }); i++; continue; }
        if (c === '"') { let s = ''; i++; while (i < src.length && src[i] !== '"') s += src[i++]; i++; tokens.push({ type: 'string', value: s }); continue; }
        if (/[0-9.\-]/.test(c)) { let s = ''; while (i < src.length && /[0-9.\-]/.test(src[i])) s += src[i++]; tokens.push({ type: 'number', value: parseFloat(s) }); continue; }
        if (/[a-zA-Z_@\-\+\*\/\?\!]/.test(c)) { let s = ''; while (i < src.length && /[a-zA-Z_@\-\+\*\/\?\!0-9]/.test(src[i])) s += src[i++]; tokens.push({ type: 'symbol', value: s }); continue; }
        i++;
    }
    return tokens;
}

/**
 * AST parser(简化 s-expr)
 */
function parse(tokens) {
    let pos = 0;
    function peek() { return tokens[pos]; }
    function next() { return tokens[pos++]; }

    function parseExpr() {
        const t = peek();
        if (!t) return null;
        if (t.type === '(') {
            next();
            const list = [];
            while (peek() && peek().type !== ')') {
                list.push(parseExpr());
            }
            if (peek() && peek().type === ')') next();
            return list;
        }
        return next();
    }

    const program = [];
    while (pos < tokens.length) program.push(parseExpr());
    return program;
}

/**
 * 编译器:AST → JS 字符串
 */
function compileAST(ast, indent = 0) {
    const pad = '  '.repeat(indent);
    let js = '';
    if (Array.isArray(ast)) {
        if (ast.length === 0) { js += 'YY.mkCell(null, 0)'; return js; }
        const head = ast[0];
        const isTag0 = (head && ((head.type === 'number' && head.value === 0) || (head.type === 'symbol' && head.value === '0')));
        const isTag1 = (head && ((head.type === 'number' && head.value === 1) || (head.type === 'symbol' && head.value === '1')));
        const isSym = (head && head.type === 'symbol');

        if (isTag0 || isTag1) {
            const trit = isTag0 ? 'TRL(0,0,0)' : 'TRL(1,1,1)';
            js += `(()=>{const c=YY.mkCell(null,${indent});c.trit=${trit};`;
            for (let i = 1; i < ast.length; i++) {
                js += `YY.cellAppend(c,${compileAST(ast[i], indent+1)});`;
            }
            js += 'return c;})()';
            return js;
        }
        // 操作符
        if (isSym) {
            switch (head.value) {
                case '评': return `YY.opJudge(null, (${compileAST(ast[1], indent)}))`;
                case '变': return `YY.opMirror(null, (${compileAST(ast[1], indent)}))`;
                case '判': case 'verdict': return `(function(v){console.log("VERDICT:",v);return v;})(${compileAST(ast[1]||['0',0.5,0.5,10,45], indent)})`;
                case '接': case 'link': return `(${compileAST(ast[1], indent)}).concat(${compileAST(ast[2], indent)})`;
                case '闭': case 'loopback': return `((c)=>{c.trit.flow=2;return c;})(${compileAST(ast[1], indent)})`;
                default: break;
            }
            // 默认:函数调用
            let s = head.value + '(';
            for (let i = 1; i < ast.length; i++) {
                if (i > 1) s += ',';
                s += compileAST(ast[i], indent + 1);
            }
            return s + ')';
        }
    }
    // 字面量
    if (ast && ast.type === 'number') return `YY.mkNum(${ast.value})`;
    if (ast && ast.type === 'string') return `JSON.parse(${JSON.stringify(JSON.stringify(ast.value))})`;
    if (ast && ast.type === 'symbol') {
        if (ast.value === '0' || ast.value === '1') return `YY.mkNum(${ast.value === '1' ? 1 : 0})`;
        return `"${ast.value}"`;
    }
    return 'null';
}

/**
 * 顶层编译:源码 → JS
 */
function compile(src) {
    const tokens = tokenize(src);
    const ast = parse(tokens);
    let js = '/** 囡囧共生 阴阳三进制自动编译输出 */\n';
    js += 'const YY = require("./yinyang_bridge.js");\n';
    js += 'const TRL = (o,i,f)=>({outer:o,inner:i,flow:f});\n';
    js += 'const __result = [];\n';
    for (const expr of ast) {
        if (Array.isArray(expr)) {
            js += `__result.push(${compileAST(expr)});\n`;
        }
    }
    js += 'module.exports = { result: __result, YY };\n';
    js += 'if (require.main === module) { console.log("result:", __result); }\n';
    return js;
}

// === 演示 ===
if (require && require.main === module) {
    const test_src = `(0 0.85 0.90 15 45 (评) (判))`;
    console.log('=== 源码 ===');
    console.log(test_src);
    console.log('\n=== 编译输出 ===');
    const jsOut = compile(test_src);
    console.log(jsOut);

    // 执行编译结果(写入同级,require 路径正确)
    console.log('=== 执行测试 ===');
    const path = require('path');
    const temp_file = path.join(__dirname, '_yy_out_temp.js');
    require('fs').writeFileSync(temp_file, jsOut);
    try {
        delete require.cache[require.resolve(temp_file)];
        const { result } = require(temp_file);
        for (const r of result) {
            if (r.t === 'verdict') console.log('verdict:', r.verdict ? 'TRUSTED' : 'REJECTED', 'conf=' + r.conf.toFixed(3));
            else if (r.verdict) console.log('result keys:', Object.keys(r));
        }
    } catch (e) {
        console.log('exec note:', e.message);
    }

    // 清理
    try { require('fs').unlinkSync(temp_file); } catch(e) {}
}

module.exports = { compile, tokenize, parse, compileAST };
