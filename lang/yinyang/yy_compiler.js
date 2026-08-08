/**
 * 囡囝共生 阴阳三进制 → JS 编译器 (Stage 0)
 *
 * YinYang 源是 s-expr 列表:
 *   (= NAME EXPR)        → JS const 定义
 *   (λ (PARAMS) BODY)    → JS 箭头函数
 *   (FUNC ARGS...)       → JS 函数调用
 *   NUMBER / STRING / SYMBOL → 字面量
 *   (0 ARGS...)          → 容器 0 (outer/inner/flow 三元器)
 *   (1 ARGS...)          → 容器 1
 *
 * 特殊语法:
 *   (if C T E)              → 三元/条件
 *   (select K (a CASEA) (b CASEB) ...)
 *   (let NAME VAL BODY)     → 内联 VAL 到 BODY
 *   (++ A B ...)            → 字符串拼接
 *   (head L) / (tail L) / (nth I L) / (cons X L) / (join L SEP) / (map F L)
 *   (num? X) / (str? X) / (nil? X) / (mem? X L) / (== A B)
 *
 * 目标:能编译 self.yy(编译器以 YinYang 自身写成),产出等价的 JS 编译器。
 */

'use strict';

function tokenize(src) {
    const tokens = [];
    let i = 0;
    while (i < src.length) {
        const c = src[i];
        if (c === ';') { while (i < src.length && src[i] !== '\n') i++; continue; }
        if (/\s/.test(c)) { i++; continue; }
        if (c === '(' || c === ')') { tokens.push({ type: c }); i++; continue; }
        if (c === '"') {
            let s = ''; i++;
            while (i < src.length && src[i] !== '"') {
                if (src[i] === '\\' && i + 1 < src.length) {
                    const nxt = src[i+1];
                    if (nxt === 'n') s += '\n';
                    else if (nxt === 't') s += '\t';
                    else if (nxt === 'r') s += '\r';
                    else if (nxt === '\\') s += '\\';
                    else if (nxt === '"') s += '"';
                    else s += nxt;
                    i += 2; continue;
                }
                s += src[i++];
            }
            i++;
            tokens.push({ type: 'string', value: s });
            continue;
        }
        if (/[0-9.\-]/.test(c)) {
            let s = '';
            while (i < src.length && /[0-9.\-]/.test(src[i])) s += src[i++];
            tokens.push({ type: 'number', value: parseFloat(s) });
            continue;
        }
        if (/[\p{L}_@%\-+*=<>!?^&\/\u0370-\u03FF\u0400-\u04FF]/u.test(c)) {
            let s = '';
            while (i < src.length && /[\p{L}_@%\-+*=<>!?^&\/\u0370-\u03FF\u0400-\u04FF0-9]/u.test(src[i])) s += src[i++];
            tokens.push({ type: 'symbol', value: s });
            continue;
        }
        i++;
    }
    return tokens;
}

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
            while (peek() && peek().type !== ')') list.push(parseExpr());
            if (peek() && peek().type === ')') next();
            return list;
        }
        return next();
    }
    const program = [];
    while (pos < tokens.length) program.push(parseExpr());
    return program;
}

/* === JS 安全标识符化:
 *   1. 关键字后加 _ (避免保留字冲突)
 *   2. 含 - 等非标识符字符 → 替换为 _
 *   3. 非字母/数字/首字符补 _
 */
const JS_KW = new Set('break case catch continue debugger default delete do else finally for function if in instanceof new return switch this throw try typeof var void while with class const enum export extends import super implements interface let package private protected public static yield'.split(' '));
function safeId(s) {
    if (s == null) return '_';
    let out = String(s).replace(/[^a-zA-Z0-9_$]/g, '_');
    if (!/^[a-zA-Z_$]/.test(out[0] || '')) out = '_' + out;
    if (JS_KW.has(out)) out += '_';
    return out;
}

/* === 编译表达式(返回 JS 表达式字符串) */
function compileExpr(ast) {
    if (ast === null || ast === undefined) return 'null';
    if (Array.isArray(ast)) {
        if (ast.length === 0) return 'null';
        const head = ast[0];
        const isSym = head && head.type === 'symbol';
        const isNumHead = head && head.type === 'number';

        // 容器字面量: (0 ...) 或 (1 ...)
        if (isNumHead && (head.value === 0 || head.value === 1)) {
            const trit = head.value === 0 ? 'TRL(0,0,0)' : 'TRL(1,1,1)';
            let s = `(()=>{const c=YY.mkCell(null,0);c.trit=${trit};`;
            for (let i = 1; i < ast.length; i++) {
                s += `YY.cellAppend(c,${compileExpr(ast[i])});`;
            }
            s += 'return c;})()';
            return s;
        }

        // 容器外表标记: (0 (expr)) / (1 (expr))
        if (isNumHead) {
            // (N EXPR) where N is other number → tagged container
            let s = `(()=>{const c=YY.mkCell(null,0);`;
            s += `YY.cellAppend(c,YY.mkNum(${head.value}));`;
            for (let i = 1; i < ast.length; i++) {
                s += `YY.cellAppend(c,${compileExpr(ast[i])});`;
            }
            s += 'return c;})()';
            return s;
        }

        if (isSym) {
            const origName = head.value;
            const name = safeId(origName);
            const tail = ast.slice(1);

            // 特殊形式(用原名匹配,输出用 safeId)
            switch (origName) {
                case '=':   // (= NAME EXPR)
                    if (ast.length === 3) {
                        return `const ${safeId(ast[1].value)} = ${compileExpr(ast[2])}`;
                    }
                    break;
                case 'λ':   // lambda / closure
                case 'lambda': {
                    const params = (Array.isArray(ast[1]) ? ast[1] : [ast[1]]).map(p => p && p.value != null ? safeId(p.value) : '_').join(',');
                    if (ast.length === 3) {
                        const body = compileExpr(ast[2]);
                        return `((${params}) => (${body}))`;
                    }
                    // 多表达式主体 (顺序执行,返回最后一个)
                    const stmts = ast.slice(2);
                    const lastExpr = stmts[stmts.length - 1];
                    const seq = stmts.slice(0, -1).map(e => {
                        let c = compileExpr(e);
                        // 如果 c 是 const X = ... 定义,直接保留
                        if (c.startsWith('const ')) return c + ';';
                        // 否则作为表达式语句
                        return `(${c});`;
                    }).join('\n');
                    const ret = compileExpr(lastExpr);
                    return `((${params}) => {\n${seq}\nreturn ${ret};\n})`;
                }
                case 'if':  // (if cond then else?)
                    if (ast.length === 4) {
                        return `(${compileExpr(ast[1])} ? ${compileExpr(ast[2])} : ${compileExpr(ast[3])})`;
                    }
                    if (ast.length === 3) {
                        return `(${compileExpr(ast[1])} && ${compileExpr(ast[2])})`;
                    }
                    break;
                case 'when':
                    return compileExpr(['if', ast[1], ast[2], null]);
                case 'let': {
                    const vname = safeId(ast[1] && ast[1].value);
                    if (ast.length === 3) return `const ${vname} = ${compileExpr(ast[2])}`;
                    if (ast.length === 4) return `((${vname}) => (${compile_expr_to_str(ast[3])}))(${compileExpr(ast[2])})`;
                    break;
                }
                case 'select': { // (select VAL (KEY EXPR) ...)
                    const keyExpr = compileExpr(ast[1]);
                    let chain = 'null';
                    for (let i = ast.length - 1; i >= 2; i--) {
                        const clause = ast[i];
                        if (Array.isArray(clause) && clause.length >= 2) {
                            if (clause[0].value === 'default') { chain = compileExpr(clause[1]); }
                            else { chain = `(${keyExpr} === ${compileExpr(clause[0])} ? ${compileExpr(clause[1])} : ${chain})`; }
                        }
                    }
                    return chain;
                }
                case 'list':    // (list a b c)
                    return `[${tail.map(compileExpr).join(', ')}]`;
                case 'begin': { // (begin EXPR EXPR ...) = IIFE
                    const body = tail.map((e, i) => i < tail.length - 1 ? `(${compileExpr(e)});` : `return ${compileExpr(e)}`).join('\n');
                    return `(() => { ${body} })()`;
                }
                case 'comment':
                    return '/* skipped */';
                /* ---- type/val/predicate 内联展开 ---- */
                case 'type':   return YY_PRIMITIVES.type(compileExpr(ast[1]));
                case 'val':    return YY_PRIMITIVES.val(compileExpr(ast[1]));
                case 'num?':   return YY_PRIMITIVES['num?'](compileExpr(ast[1]));
                case 'str?':   return YY_PRIMITIVES['str?'](compileExpr(ast[1]));
                case 'list?':  return YY_PRIMITIVES['list?'](compileExpr(ast[1]));
                case 'nil?':   return YY_PRIMITIVES['nil?'](compileExpr(ast[1]));
                case '==':     return '(' + compileExpr(ast[1]) + ' === ' + compileExpr(ast[2]) + ')';
                case '!=':     return '(' + compileExpr(ast[1]) + ' !== ' + compileExpr(ast[2]) + ')';
                case '<':      return YY_PRIMITIVES ['<'] ? YY_PRIMITIVES['<'](compileExpr(ast[1]), compileExpr(ast[2])) : '(' + compileExpr(ast[1]) + ' < ' + compileExpr(ast[2]) + ')';
                case '>':      return '(' + compileExpr(ast[1]) + ' > ' + compileExpr(ast[2]) + ')';
                case 'and':    return '(' + compileExpr(ast[1]) + ' && ' + compileExpr(ast[2]) + ')';
                case 'or':     return '(' + compileExpr(ast[1]) + ' || ' + compileExpr(ast[2]) + ')';
                case 'not':    return '(!' + compileExpr(ast[1]) + ')';
                case 'cons':   return YY_PRIMITIVES.cons(compileExpr(ast[1]), compileExpr(ast[2]));
                case 'nth':    return YY_PRIMITIVES.nth(compileExpr(ast[1]), compileExpr(ast[2]));
                case 'head':   return YY_PRIMITIVES.head(compileExpr(ast[1]));
                case 'tail':   return YY_PRIMITIVES.tail(compileExpr(ast[1]));
                case 'join':   return YY_PRIMITIVES.join(compileExpr(ast[1]), compileExpr(ast[2]));
                case 'map':    return YY_PRIMITIVES.map(compileExpr(ast[1]), compileExpr(ast[2]));
                case 'stringify': return YY_PRIMITIVES.stringify(compileExpr(ast[1]));
                case 'print':  return YY_PRIMITIVES.print(...tail.map(e => compileExpr(e)));
                // 内建助手
                case '++':      // 字符串拼接
                    return `('' + ${tail.map(compileExpr).join(' + ')})`;
                case 'head':
                    return `((${compileExpr(ast[1])})[0])`;
                case 'tail':
                    return `((${compileExpr(ast[1])}).slice(1))`;
                case 'cons':
                    return `[${compileExpr(ast[1])}, ...${compileExpr(ast[2])}]`;
                case 'nth':
                    return `((${compileExpr(ast[2])})[${compileExpr(ast[1])}])`;
                case 'join':
                    return `((${compileExpr(ast[1])}).join(${compileExpr(ast[2])}))`;
                case 'map':
                    return `((${compileExpr(ast[2])}).map(${compileExpr(ast[1])}))`;
                case 'length':
                    return `((${compileExpr(ast[1])}).length)`;
                case 'stringify':
                    return `JSON.stringify(${compileExpr(ast[1])})`;
                case 'num?':
                    return `(typeof ${compileExpr(ast[1])} === 'number')`;
                case 'str?':
                    return `(typeof ${compileExpr(ast[1])} === 'string')`;
                case 'list?':
                    return `Array.isArray(${compileExpr(ast[1])})`;
                case 'nil?':
                    return `(${compileExpr(ast[1])} == null)`;
                case 'mem?':
                    return `(${compileExpr(ast[2])}.includes(${compileExpr(ast[1])}))`;
                case '==':
                    return '(' + compileExpr(ast[1]) + ' === ' + compileExpr(ast[2]) + ')';
                case '!=':
                    return '(' + compileExpr(ast[1]) + ' !== ' + compileExpr(ast[2]) + ')';
                case '<':
                    return '(' + compileExpr(ast[1]) + ' < ' + compileExpr(ast[2]) + ')';
                case '>':
                    return '(' + compileExpr(ast[1]) + ' > ' + compileExpr(ast[2]) + ')';
                case 'and':
                    return '(' + compileExpr(ast[1]) + ' && ' + compileExpr(ast[2]) + ')';
                case 'or':
                    return '(' + compileExpr(ast[1]) + ' || ' + compileExpr(ast[2]) + ')';
                case 'not':
                    return '(!' + compileExpr(ast[1]) + ')';

                default:
                    // YinYang 核心运行时操作(保留)
                    if (name === '评') return `YY.opJudge(null, ${compileExpr(['0', ast[1]])} )`;
                    if (name === '判') return `(function(v){return v;})(${compileExpr(ast[1]||['0', 0.5, 0.5, 10, 45])})`;
                    if (name === '变') return `YY.opMirror(null, ${compileExpr(['0', ast[1]])})`;
                    // 默认:普通函数调用
                    return `(${name}(${tail.map(e => compileExpr(e)).join(', ')}))`;
            }
        }
        // 头部非符号,默认视为容器
        return `null`;
    }

    // 字面量
    if (typeof ast === 'number') return `${ast}`;
    if (typeof ast === 'string') return JSON.stringify(ast);
    if (ast.type === 'number') return `${ast.value}`;
    if (ast.type === 'string') return JSON.stringify(ast.value);
    if (ast.type === 'symbol') return safeId(ast.value);
    return 'null';
}

/* === YinYang 标准库内建(直接产出对应 JS) */
const YY_PRIMITIVES = {
    'type':       v => `((${v}) && (${v}).t)`,
    'val':        v => `((${v}) && (${v}).value)`,
    'str?':        v => `(typeof ${v} === 'string')`,
    'num?':        v => `(typeof ${v} === 'number')`,
    'list?':       v => `Array.isArray(${v})`,
    'nil?':        v => `(${v} == null)`,
    'symbol?':     v => `(typeof ${v} === 'symbol' || (typeof ${v} === 'object' && (${v}).t === 'symbol'))`,
    head:          a => `((typeof ${a} === 'string') ? (${a})[0] : (${a})[0])`,
    tail:          a => `((typeof ${a} === 'string') ? (${a}).slice(1) : (${a}).slice(1))`,
    nth:         (i,a) => `(${a})[${i}]`,
    cons:        (x,a) => `[${x}, ...(${a})]`,
    list:              xs => `[${xs.join(', ')}]`,
    join:        (a,s) => `(${a}).join(${s})`,
    map:         (f,a) => `(${a}).map(${f})`,
    filter:      (f,a) => `(${a}).filter(${f})`,
    length:        a => `(${a}).length`,
    stringify:     v => `JSON.stringify(${v})`,
    print:        (...xs) => `console.log(${xs.join(', ')})`,
    cmd_args_raw:    () => `(process.argv.slice(2))`,
};

/* === 编译顶层程序:把 AST 转换为一组 JS 语句 */
function compileProgram(ast) {
    const stmts = [];
    const exportsList = [];
    stmts.push(`/** 囡囝共生 阴阳三进制 · 自动编译输出 (Stage 1) */`);
    stmts.push(`'use strict';`);
    stmts.push(`const YY = require('./yinyang_bridge.js');`);
    stmts.push(`const TRL = (o,i,f) => ({outer:o, inner:i, flow:f});`);
    stmts.push(`/* ---- 编译器运行时内嵌(parse/tokenize 共享)---- */`);
    stmts.push(`const __run = require('./yy_runtime.js');`);
    stmts.push(`const tokenize = __run.tokenize;`);
    stmts.push(`const parse = __run.parse;`);

    const topExprs = [];
    for (const expr of ast) {
        if (!Array.isArray(expr)) continue;
        if (expr.length === 0) continue;
        const head = expr[0];
        // 顶层定义
        if (head && head.type === 'symbol' && head.value === '=' && expr.length === 3) {
            const nm = expr[1] && expr[1].value;
            if (!nm) continue;
            const name = safeId(nm);
            const valueJS = compileExpr(expr[2]);
            stmts.push(`const ${name} = ${valueJS};`);
            exportsList.push(name);
        } else if (head && head.type === 'symbol' && head.value === 'comment') {
            continue;
        } else {
            // 顶层语句:延后执行,以防引用未就绪的 helper
            topExprs.push(compileExpr(expr));
        }
    }

    // 所有顶层语句包裹在 "启动器" 里,免得 require 时立刻调到未 ready 的 helper
    if (topExprs.length) {
        exportsList.push('__auto_main');
        stmts.push(`const __auto_main = () => {`);
        for (const e of topExprs) {
            stmts.push('  (' + e + ');');
        }
        stmts.push(`};`);
    }

    stmts.push(`module.exports = {`);
    if (exportsList.length) {
        stmts.push('  ' + exportsList.join(', '));
    }
    stmts.push(`};`);
    stmts.push(`if (require.main === module && typeof __auto_main === 'function') { __auto_main(); }`);
    return stmts.join('\n') + '\n';
}

function compile(src) {
    return compileProgram(parse(tokenize(src)));
}

// === 演示/直运行 ===
if (require && require.main === module) {
    const fs = require('fs');
    const path = require('path');
    let testSrc = null;
    if (process.argv[2]) {
        testSrc = fs.readFileSync(path.resolve(process.argv[2]), 'utf8');
    } else {
        testSrc = `(0 0.85 0.90 15 45 (评) (判))`;
    }
    console.log(compile(testSrc));
}

module.exports = { compile, tokenize, parse, compileExpr, compileProgram };
