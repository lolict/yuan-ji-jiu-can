/**
 * 囡囝共生 · YinYang 运行时 · 共享库(Stage 0 / Stage 2 共用)
 *
 * 提供 tokenize + parse + select-dispatch 等编译器内建原语。
 * Stage 0 编译器产出 auto.js 时,这些原语既嵌入 auto.js,
 * 也被 Stage 2 自举编译器 require 以自我运行。
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

/* ===== 编译期 select 分派(用于 Stage 2 自举运行时) ===== */
/* 把 (select VAL (K V) ...) 化为等价 dispatch。这里不展开,
   Stage 1 编译器已把 select 内联为三元链,下方纯属备用。 */

module.exports = { tokenize, parse };
