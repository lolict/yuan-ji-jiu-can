/**
 * 囡囝共生 · 阴阳三进制解释器 · JavaScript 桥
 *
 * 这份 JS 代码是 C 解释器(yinyang_interp.c)的等价 JS 实现,
 * 让浏览器壳(mqf-shell.js)直接调用,无需 WASM。
 *
 * 设计目标:
 *   0 = 容器/包含者/中心 → JS 中表示为 { t:'cell', outer:0, inner:0, flow:0, body:[...] }
 *   1 = 被含物/边界      → JS 中表示为 { t:'cell', outer:1, inner:1, flow:1, body:[...] }
 *   数值 = 微量           → JS 中表示为 { t:'num', value:N, tri:{outer:1,inner:1,flow:1} }
 *   verdict = 裁定结果    → JS 中表示为 { t:'verdict', verdict:TRUSTED|REJECTED, conf:N, tri:{...} }
 *
 * 三进制三轴(3D):
 *   outer = 边界态(0闭锁 / 1开放)
 *   inner = 包含态(0空 / 1有内容)
 *   flow  = 流向(0静止 / 1流出 / 2回流闭环)
 *
 * 与 mqf-shell.js 的对接:
 *   - 外部请求 → 进入容器(0)
 *   - 变(镜像代理) → hook 截断
 *   - 评(共生赔伴) → 六元信任交集
 *   - 判 → 输出 verdict
 */

'use strict';

/** 命名常量 */
const NJ_TRUSTED  = 1;
const NJ_REJECTED = 0;
const NJSG_TRUST_THRESHOLD  = 0.6;
const NJSG_MIRROR_TOLERANCE = 0.2;
const YINYANG_VERSION = '0.1.0';

/* ===== 3D 节点构造 ===== */

function TRL(outer, inner, flow) { return { outer, inner, flow }; }

function mkCell(parent, depth) {
    return {
        t: 'cell',
        trit: TRL(0, 0, 0),     /* 0=容器:闭锁边界+空壳+静止 */
        depth: depth || 0,
        tid: 0,
        parent: parent || null,
        body: []
    };
}

function mkNum(n) {
    return {
        t: 'num',
        value: n,
        trit: TRL(1, 1, 1)      /* 被含物:开放+有内容+流出 */
    };
}

function mkVerdict(verdict, conf) {
    return {
        t: 'verdict',
        verdict: verdict ? NJ_TRUSTED : NJ_REJECTED,
        conf: conf,
        trit: TRL(verdict ? 1 : 0, 0, 2)  /* verdict:outer=verdict,inner=0,flow=2(闭环) */
    };
}

/* ===== 容器操作 ===== */

function cellAppend(cell, val) {
    if (!cell || !cell.body) return;
    cell.body.push(val);
    cell.tid++;
    if (val.trit) { cell.trit.flow = 1; }  /* 有内容流入则 flow=1 */
    if (cell.body.length > 0) { cell.trit.inner = 1; }  /* 有内容则 inner=1 */
}

function cellNth(cell, n) {
    return (cell && cell.body && cell.body[n]) ? cell.body[n] : null;
}

/* ===== 数值辅助 ===== */

function asNum(v) {
    if (!v) return 0;
    if (v.t === 'num') return v.value;
    if (v.t === 'verdict' && v.verdict === NJ_TRUSTED) return v.conf;
    return 0;
}

/* ===== 核心操作 ===== */

/**
 * 共生赔伴裁定(与 mqf_wasm.c / mqf-shell.js 数值算法一致)
 */
function opJudge(env, container) {
    const a = asNum(cellNth(container, 0));
    const b = asNum(cellNth(container, 1));
    const force = asNum(cellNth(container, 2));
    const angle = asNum(cellNth(container, 3));

    /* 六元信任交集 */
    const trust = Math.min(a, b);
    /* 镜像对齐判定 */
    const diff = Math.abs(a - b);
    const mirror_ok = (diff < NJSG_MIRROR_TOLERANCE);
    /* 裁定: 信任足够 + 对齐通过 */
    const verdict = (trust > NJSG_TRUST_THRESHOLD && mirror_ok) ? NJ_TRUSTED : NJ_REJECTED;

    /* 五元:刻度(力* 10) */
    const ticks = Math.floor(force * 10);
    const occupied = (ticks > 100);

    /* 五元:角度 wrap 0-360 */
    let final_angle = angle % 360;
    if (final_angle < 0) final_angle += 360;

    return {
        verdict: mkVerdict(verdict, trust),
        ticks: ticks,
        occupied: occupied ? 'ETERNAL' : 'EPHEMERAL',
        angle: final_angle,
        mirror_ok: mirror_ok,
        trust_intersection: trust
    };
}

/**
 * 镜像代理(公理 1+2+3+5):截断外部调用,重新定义身份
 * 返回被我们"重定义"后的值链
 */
function opMirror(env, container) {
    const mirror_coeff = (container.body.length >= 3 && asNum(cellNth(container, 2)) > 10) ? 0.95 : 0.92;
    const mirrored = mkCell(container.parent, container.depth);
    for (let i = 0; i < container.body.length; i++) {
        const v = container.body[i];
        cellAppend(mirrored, mkNum(asNum(v) * mirror_coeff));
    }
    mirrored.trit.outer = 1;  /* 已被我们接管,开放给外部(看起来正常,但已被我们转写) */
    mirrored.trit.flow  = 2;  /* 闭环 */
    return mirrored;
}

/* ===== 高阶容器语法 ===== */

/**
 * 0(1) 容器含被含物
 * @param {array} body - 被含物数组
 * @param {object} parent - 父容器
 * @param {number} depth - 嵌套深度
 */
function makeContainer01(body, parent, depth) {
    const container = mkCell(parent, depth);
    for (const b of body) {
        if (Array.isArray(b)) {  /* 子容器 */
            cellAppend(container, makeContainer01(b, container, depth + 1));
        } else if (typeof b === 'number') {
            cellAppend(container, mkNum(b));
        } else if (b && typeof b === 'object') {
            cellAppend(container, b);
        }
    }
    return container;
}

/**
 * 3D strcuture dump for debugging
 */
function dumpCell(cell, indent = 0) {
    const pad = '  '.repeat(indent);
    let s = `${pad}[cell depth=${cell.depth} tid=${cell.tid} tri=(${cell.trit.outer},${cell.trit.inner},${cell.trit.flow})]\n`;
    for (const v of cell.body) {
        if (v.t === 'cell') s += dumpCell(v, indent + 1);
        else if (v.t === 'num') s += `${pad}  num=${v.value} tri=(${v.trit.outer},${v.trit.inner},${v.trit.flow})\n`;
        else if (v.t === 'verdict') s += `${pad}  verdict=${v.verdict ? 'TRUSTED':'REJECTED'} conf=${v.conf.toFixed(3)} tri=(${v.trit.outer},${v.trit.inner},${v.trit.flow})\n`;
    }
    return s;
}

/* ===== 公开接口 ===== */

/**
 * 一步裁决入口
 * @param {number} a - compliance_a
 * @param {number} b - compliance_b
 * @param {number} force - 作用力
 * @param {number} angle - 角度
 * @returns {{verdict, ticks, occupied, angle, conf, mirror_ok}}
 */
function judge(a, b, force, angle) {
    const container = makeContainer01([a, b, force + 10, angle], null, 0);
    return opJudge(null, container);
}

/**
 * 镜面代理
 * @returns 镜像后的被含物容器
 */
function mirror(a, b, force, angle) {
    const container = makeContainer01([a, b, force + 10, angle], null, 0);
    return opMirror(null, container);
}

/**
 * 获取版本
 */function version() { return YINYANG_VERSION; }

/* ===== 模块导出 ===== */
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        judge, mirror, version,
        mkCell, mkNum, mkVerdict, cellAppend,
        makeContainer01, dumpCell,
        opJudge, opMirror,
        NJ_TRUSTED, NJ_REJECTED
    };
}

/* 浏览器全局名 */
if (typeof window !== 'undefined') {
    window.YinYang = {
        judge, mirror, version,
        mkCell, mkNum, mkVerdict, cellAppend,
        makeContainer01, dumpCell,
        opJudge, opMirror,
        NJ_TRUSTED, NJ_REJECTED
    };
}
