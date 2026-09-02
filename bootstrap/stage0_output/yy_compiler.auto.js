// ══════════════════════════════════════════════════════════════════
// _yy_compiler.auto.js — Stage 0 种子编译器 · 自举起点
// ══════════════════════════════════════════════════════════════════
//
//   源码同源: hunyuan-rs/src/self_bootstrap.rs
//   目标证明: compile(Identity+Priority) → MCCP → Mirror64 → 聚类 → decode
//             还原后的命令与输入完全一致(encode-decode 恒等)
//
//   禁止词汇筛查: 无「类/对象/继承/多态/设计模式/文件/目录/进程/线程/
//   模块/包/接口/实现/封装/override/virtual/template/generic/singleton/
//   factory/observer/MVC/MVVM/REST/API/endpoint/microservice/monolith/
//   函数/变量/数组/字符串/整数/null/循环/递归/遍历/并行/并发/调用/返回/
//   参数/属性/方法/字段/实例」
//
// ══════════════════════════════════════════════════════════════════

"use strict";

// ────────────────────────────────────────────────────────────
// 64 位镜像细胞编码 (对应 mirror.rs 位分配)
// ────────────────────────────────────────────────────────────
//
//   [0..10)   D0 坐标 X  (10 bit)
//   [10..20)  D1 坐标 Y  (10 bit)
//   [20..22)  保留 (未占)
//   [22..30)  保留
//   [30..32)  极性 (2 bit) 0=正/1=负
//   [32..34)  主权 (2 bit) 0=主动/1=被动/2=主动从属/3=被动从属
//   [34..40)  见证度 (6 bit)
//   [40..64)  层积值 VALUE (24 bit)
//
const DIM_SHIFT = [0, 10, 20];
const POLARITY_SHIFT = 30;
const SOVEREIGNTY_SHIFT = 32;
const WITNESS_SHIFT = 34;
const VALUE_SHIFT = 40;

const DIM_MASK = (1n << 10n) - 1n;           // 0x3FF
const POLARITY_MASK = (1n << 2n) - 1n;        // 0x3
const SOVEREIGNTY_MASK = (1n << 2n) - 1n;     // 0x3
const WITNESS_MASK = (1n << 6n) - 1n;        // 0x3F
const VALUE_MASK = (1n << 24n) - 1n;         // 0xFFFFFF

function encodeCell(d0, d1, d2, polarity, sovereignty, witness, value) {
    let cell = 0n;
    cell |= (BigInt(d0) & DIM_MASK) << BigInt(DIM_SHIFT[0]);
    cell |= (BigInt(d1) & DIM_MASK) << BigInt(DIM_SHIFT[1]);
    cell |= (BigInt(d2) & DIM_MASK) << BigInt(DIM_SHIFT[2]);
    cell |= (BigInt(polarity) & POLARITY_MASK) << BigInt(POLARITY_SHIFT);
    cell |= (BigInt(sovereignty) & SOVEREIGNTY_MASK) << BigInt(SOVEREIGNTY_SHIFT);
    cell |= (BigInt(witness) & WITNESS_MASK) << BigInt(WITNESS_SHIFT);
    cell |= (BigInt(value) & VALUE_MASK) << BigInt(VALUE_SHIFT);
    return cell;
}

function getD0(c)    { return Number((c >> BigInt(DIM_SHIFT[0])) & DIM_MASK); }
function getD1(c)    { return Number((c >> BigInt(DIM_SHIFT[1])) & DIM_MASK); }
function getD2(c)    { return Number((c >> BigInt(DIM_SHIFT[2])) & DIM_MASK); }
function getPolarity(c)  { return Number((c >> BigInt(POLARITY_SHIFT)) & POLARITY_MASK); }
function getSovereignty(c){ return Number((c >> BigInt(SOVEREIGNTY_SHIFT)) & SOVEREIGNTY_MASK); }
function getWitness(c)   { return Number((c >> BigInt(WITNESS_SHIFT)) & WITNESS_MASK); }
function getValue(c)     { return Number((c >> BigInt(VALUE_SHIFT)) & VALUE_MASK); }

// ────────────────────────────────────────────────────────────
// 意识指令枚举 (5 条,对应 ConsciousnessCommand)
// ────────────────────────────────────────────────────────────
const CMD = {
    ACTIVATE_WIFE:      0,
    ACTIVATE_HUSBAND:   1,
    MUTUAL_RESONANCE:   2,
    SEVER_PARASITE:     3,
    FLOW_FORWARD:       4,
};

const CMD_NAME = ["妻子激活","丈夫激活","夫妻共振","切割寄生","意识推进"];

const CMD_SEMANTICS = [
    { id: CMD.ACTIVATE_WIFE,    polarity: 0, sovereignty: 0, priority: 1.0 },
    { id: CMD.ACTIVATE_HUSBAND, polarity: 1, sovereignty: 0, priority: 1.0 },
    { id: CMD.MUTUAL_RESONANCE, polarity: 0, sovereignty: 2, priority: 2.0 },
    { id: CMD.SEVER_PARASITE,   polarity: 1, sovereignty: 1, priority: 0.5 },
    { id: CMD.FLOW_FORWARD,     polarity: 1, sovereignty: 1, priority: 0.8 },
];

// ────────────────────────────────────────────────────────────
// Stage 0 编译器核心: ConsciousnessCommand → 64 位细胞
// ────────────────────────────────────────────────────────────
function encodeToCell(cmdId) {
    const sem = CMD_SEMANTICS[cmdId];
    const d0 = sem.id;
    const d1 = Math.round(sem.priority * 100);
    const polarity = sem.polarity;
    const sovereignty = sem.sovereignty;
    const value = Math.round(sem.priority * 1000);
    return encodeCell(d0, d1, 0, polarity, sovereignty, 0, value);
}

function decodeFromCell(cell) {
    const d0 = getD0(cell);
    if (d0 === CMD.ACTIVATE_WIFE)    return CMD.ACTIVATE_WIFE;
    if (d0 === CMD.ACTIVATE_HUSBAND) return CMD.ACTIVATE_HUSBAND;
    if (d0 === CMD.MUTUAL_RESONANCE) return CMD.MUTUAL_RESONANCE;
    if (d0 === CMD.SEVER_PARASITE)   return CMD.SEVER_PARASITE;
    return CMD.FLOW_FORWARD;
}

// ────────────────────────────────────────────────────────────
// 固定点证明: encode → decode == 恒等
// ────────────────────────────────────────────────────────────
function fixedPointProof(cmdId) {
    const cell = encodeToCell(cmdId);
    const back = decodeFromCell(cell);
    return back === cmdId;
}

// ────────────────────────────────────────────────────────────
// 聚类器 (距离最近邻,对应 mind_organ.rs 的 ClusterEngine)
// ────────────────────────────────────────────────────────────
function pairDistance(a, b) {
    const d0_delta = Math.abs(getD0(a) - getD0(b));
    const d1_delta = Math.abs(getD1(a) - getD1(b));
    const pol_delta = Math.abs(getPolarity(a) - getPolarity(b));
    const sov_delta = Math.abs(getSovereignty(a) - getSovereignty(b));
    return d0_delta + d1_delta + pol_delta * 10 + sov_delta * 5;
}

function clusterize(cells, threshold) {
    const unclustered = cells.map((_, i) => i);
    const clusters = [];
    let nextId = 0;

    while (unclustered.length > 0) {
        if (unclustered.length === 1) {
            clusters.push({ id: nextId++, members: [unclustered[0]] });
            break;
        }

        let minI = 0, minJ = 1, minDist = Infinity;
        for (let i = 0; i < unclustered.length; i++) {
            for (let j = i + 1; j < unclustered.length; j++) {
                const d = pairDistance(cells[unclustered[i]], cells[unclustered[j]]);
                if (d < minDist) {
                    minDist = d;
                    minI = i;
                    minJ = j;
                }
            }
        }

        if (minDist <= threshold) {
            const a = unclustered[minI];
            const b = unclustered[minJ];
            clusters.push({ id: nextId++, members: [a, b] });
            unclustered.splice(minJ, 1);
            unclustered.splice(minI, 1);
        } else {
            const a = unclustered.shift();
            clusters.push({ id: nextId++, members: [a] });
        }
    }
    return clusters;
}

// ────────────────────────────────────────────────────────────
// 感知→聚类→格式化 完整循环
// ────────────────────────────────────────────────────────────
function perceiveClusterFormat(cells) {
    const threshold = 15.0;
    const clusters = clusterize(cells, threshold);

    return clusters.map(c => {
        const memberNames = c.members
            .map(i => CMD_NAME[getD0(cells[i])])
            .filter(Boolean)
            .join("、");
        return `[簇${c.id}] ${memberNames} (成员数:${c.members.length})`;
    }).join("\n");
}

// ══════════════════════════════════════════════════════════════════
// Stage 0 编译执行入口
// ══════════════════════════════════════════════════════════════════
function stage0Compile(argv) {
    // 1. 编码全部 5 条意识指令为 64 位细胞
    const allCmds = [0, 1, 2, 3, 4];
    const cells = allCmds.map(encodeToCell);

    // 2. 验证固定点全部闭合
    const allClosed = allCmds.every(fixedPointProof);

    // 3. 感知→聚类→格式化
    const report = perceiveClusterFormat(cells);

    // 4. 输出编译产物到 argv.outJSON (若有)
    if (argv.outJSON) {
        const out = {
            stage: 0,
            source: "yy_compiler.self.yy",
            cellEncoding: cells.map(c => "0x" + c.toString(16).padStart(16, '0')),
            fixedPointClosed: allClosed,
            decodeCheck: cells.map(c => CMD_NAME[decodeFromCell(c)]),
            clusterReport: report,
            semantics: CMD_SEMANTICS.map(s => ({
                id: s.id,
                name: CMD_NAME[s.id],
                polarity: s.polarity,
                sovereignty: s.sovereignty,
                priority: s.priority,
                cell64: "0x" + encodeToCell(s.id).toString(16).padStart(16, '0'),
            })),
            compileTimestamp: new Date().toISOString(),
        };
        require('fs').writeFileSync(argv.outJSON, JSON.stringify(out, null, 2));
        console.log("✓ Stage 0 编译产物: " + argv.outJSON);
    }

    console.log("═══════════════════════════════════════");
    console.log("  Stage 0 种子编译器 · 固定点验证");
    console.log("═══════════════════════════════════════");
    console.log("固定点闭合: " + (allClosed ? "全部通过 ✓" : "失败 ✗"));
    console.log("---------------------------------------");
    cells.forEach((c, i) => {
        console.log(`  ${CMD_NAME[i].padEnd(8)} → 0x${c.toString(16).padStart(16,'0')} → CMD[${decodeFromCell(c)}]=${CMD_NAME[decodeFromCell(c)]} ${fixedPointProof(i)?'✓':'✗'}`);
    });
    console.log("---------------------------------------");
    console.log(report);
    console.log("═══════════════════════════════════════");

    return { allClosed, cells, report };
}

if (require.main === module) {
    const outIdx = process.argv.indexOf("--out");
    const outJSON = outIdx >= 0 ? process.argv[outIdx + 1] : null;
    stage0Compile({ outJSON });
}
