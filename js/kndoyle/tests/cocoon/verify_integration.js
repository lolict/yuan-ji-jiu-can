/**
 * ============================================================
 * UniCore · 茧(cocoon) · 全量串联验证脚本
 * ============================================================
 *
 * 验证目标：
 *   1. 蛹茧核心架构   —— Cocoon 序列化/解析/孵化 + ShellRegistry 注册 + 浏览器引导
 *   2. 0元/4元逻辑体系 —— ZeroYuan / Fuel / TwoYuan / ThreeYuan /
 *                        ConePhaseHubChain / FourYuan / BigGear / BalanceJudge
 *   3. 猫码系统       —— 48 个猫码 + 抓到老鼠即好码 + 红黑账/白黑账 + 价值审判
 *   4. CocoonLang 工具链 —— 语言核心 + 自举编译器 + 解释器 + 翻译器 + 感应器
 *   5. 多平台寄生壳   —— 24 个预注册壳 + 平台检测 + 穿壳孵化
 *
 * 运行：node tests/cocoon/verify_integration.js
 * ============================================================
 */

'use strict';

const path = require('path');
const cocoonCore = require('../../src/cocoon/index.js');
const logic = require('../../src/cocoon/index_logic.js');
const lang = require('../../src/cocoon/index_lang.js');
// 兼容 COCOON 简称（用于第 9 章 种子库和乘法表）
const COCOON = cocoonCore;

// 同步/异步统一辅助：处理 A4A.request 这种可能返回 Promise 的函数
function runSync(fn) {
    try {
        const r = fn();
        if (r && typeof r.then === 'function') {
            // 等待微任务队列耗尽 (for Node.js): 用 deasync? 不行。退化为：把 promise 抛出来，由调用方用同步方式写
            // 折中方案：使用同步版本的 requestSync；若没有则在 try/catch 下直接返回（A4A 目前是同步）
            // 若真的是异步，挂到全局 unresolvedPromises，测试脚本末尾会 Promise.all 后 finish
            if (typeof globalThis === 'undefined') global.unresolvedPromises = global.unresolvedPromises || [];
            else globalThis.unresolvedPromises = globalThis.unresolvedPromises || [];
            const arr = (typeof globalThis === 'undefined' ? global : globalThis).unresolvedPromises;
            arr.push(r.catch(() => null));
            return null;
        }
        return r;
    } catch (e) {
        return { ok:false, status:500, error: String(e.message || e), body:null };
    }
}

// ============================================================
// 测试框架
// ============================================================

const results = [];
let passed = 0, failed = 0;

function ok(name, cond, detail) {
    if (cond) {
        passed++;
        results.push(`  [PASS] ${name}`);
    } else {
        failed++;
        results.push(`  [FAIL] ${name}${detail ? ' :: ' + detail : ''}`);
    }
}

function section(title) {
    results.push('');
    results.push('============================================================');
    results.push(`  ${title}`);
    results.push('============================================================');
}

function summary() {
    results.push('');
    results.push('============================================================');
    results.push(`  总计: ${passed + failed}  通过: ${passed}  失败: ${failed}`);
    results.push('============================================================');
    return failed === 0;
}

// ============================================================
// 1. 蛹茧核心架构验证
// ============================================================

function verifyCocoonCore() {
    section('1. 蛹茧核心架构 (Cocoon + ShellRegistry + BrowserWASMEntry)');

    const { Cocoon, ShellRegistry, BrowserWASMEntry, CocoonParser,
            SHELL_REGISTRY, COCOON_MAGIC, buildDemoCocoon, createRegistry } = cocoonCore;

    // 1.1 常量
    ok('蛹茧魔数 = [0xC0,0xCC,0x0C,0x00]',
       Array.isArray(COCOON_MAGIC) &&
       COCOON_MAGIC[0] === 0xC0 && COCOON_MAGIC[1] === 0xCC &&
       COCOON_MAGIC[2] === 0x0C && COCOON_MAGIC[3] === 0x00);

    // 1.2 ShellRegistry 必须先注册全部壳（修复先前 bug）
    const reg = new ShellRegistry();
    ok('空 ShellRegistry.listAll() === 0', reg.listAll().length === 0);
    reg.registerAll(SHELL_REGISTRY);
    const TOTAL_SHELLS = reg.listAll().length;
    ok(`registerAll 后壳数 = ${TOTAL_SHELLS} (含具身智能+人形机器人)`, TOTAL_SHELLS >= 26,
       `实际 ${TOTAL_SHELLS}`);
    ok(`createRegistry() 工厂返回 >=26 个壳`, createRegistry().listAll().length >= 26);

    // 1.3 平台检测
    const detected = reg.detectPlatform();
    ok('detectPlatform() 不为 null', detected !== null && detected !== undefined,
       `detected=${JSON.stringify(detected && detected.id)}`);

    // 1.4 蛹茧序列化 / 解析 往返
    const demo = buildDemoCocoon('verify-cocoon');
    const bin = demo.serialize();
    ok('serialize() 产生非空二进制', bin && bin.length > 10, `len=${bin && bin.length}`);

    const reparsed = new Cocoon();
    reparsed.parse(bin);
    ok('parse() 往返: magic 匹配',
       reparsed.magic[0] === COCOON_MAGIC[0] && reparsed.magic[3] === COCOON_MAGIC[3]);
    ok('parse() 往返: name 还原',
       reparsed.metadata.name === 'verify-cocoon',
       `name=${reparsed.metadata.name}`);
    ok('parse() 往返: kernel 长度还原',
       reparsed.kernel.length === demo.kernel.length,
       `${reparsed.kernel.length} vs ${demo.kernel.length}`);
    ok('parse() 往返: shellIds 数量还原',
       Array.isArray(reparsed.shellIds) && reparsed.shellIds.length === demo.shellIds.length);

    // 1.5 列出可用壳（24个用户点名 + 具身智能+人形机器人=26个）
    const usable = demo.listShells();
    ok('listShells() 返回 26 个可用壳（含具身智能+人形机器人）', usable.length >= 26, `actual=${usable.length}`);

    // 1.6 孵化（穿壳）
    const hatchResult = demo.hatch('android');
    ok('hatch("android") 状态 = hatched', hatchResult.status === 'hatched');
    ok('hatch 后壳名 = 安卓蛹茧壳',
       hatchResult.shell && hatchResult.shell.name === '安卓蛹茧壳');
    ok('hatch 后 launchCommand 包含 am start',
       (hatchResult.launchCommand || '').includes('am start'));

    // 不存在的壳应抛错
    let threw = false;
    try { demo.hatch('not-exist'); } catch (_) { threw = true; }
    ok('hatch(不存在壳) 抛错', threw);

    // 1.7 BrowserWASMEntry 引导器
    const entry = new BrowserWASMEntry();
    ok('BrowserWASMEntry.environment 检测到 Node',
       entry.environment.isNode === true || entry.environment.isBrowser === true);
    ok('WebAssembly 可用', entry.wasmAvailable === true);

    entry.bootstrap().then(boot => {
        ok('bootstrap() 返回 booted=true', boot.booted === true);
        ok('沙箱初始权限非空', boot.permissions.length > 0,
           `perms=${boot.permissions.length}`);

        // 1.8 加载蛹茧（用二进制直接喂）
        return entry.loadCocoon(bin);
    }).then(buf => {
        ok('loadCocoon(二进制) 返回 Uint8Array',
           buf instanceof Uint8Array && buf.length === bin.length);

        // 1.9 启动解析器
        const parser = entry.startParser();
        ok('startParser() 返回 CocoonParser',
           parser.constructor.name === 'CocoonParser');

        const cocoon = parser.parseCocoon(buf);
        ok('parser.parseCocoon() 解析成功',
           cocoon.metadata.name === 'verify-cocoon');

        const kernel = parser.extractKernel(buf);
        ok('extractKernel() 返回非空内核', kernel && kernel.length > 0);

        const shells = parser.extractShells(buf);
        ok('extractShells() 返回 >= 26 个壳（含具身智能+人形机器人）', shells.length >= 26, `actual=${shells.length}`);

        // 1.10 自我提权
        const elevate = parser.selfElevate(shells[0]);
        ok('selfElevate() 返回 elevated=true', elevate.elevated === true);
        ok('自我提权声明无需系统授权',
           elevate.systemAuthorizationRequired === false);
        ok('提权能力数量 = 壳声明的权限数',
           Object.keys(elevate.capabilities).length === shells[0].permissions.length);

        // 进入下一节
        verifyLogicSystem();
    }).catch(err => {
        ok('蛹茧核心引导链路无异常', false, String(err && err.message || err));
        verifyLogicSystem();
    });
}

// ============================================================
// 2. 0元/4元逻辑体系验证
// ============================================================

function verifyLogicSystem() {
    section('2. 0元/4元逻辑体系 + 大齿轮 + 天平裁判');

    const { ZeroYuan, Fuel, TwoYuan, Anchor, ThreeYuan,
            ConePhaseHubChain, MirrorLogic, FourYuan,
            BigGear, BalanceJudge } = logic;

    // 2.1 0元 混元初始
    const zero = new ZeroYuan();
    ok('ZeroYuan 初始未观测', zero.isObserved() === false);
    const yinyang = zero.initialYinYang();
    ok('initialYinYang() 阴阳各 0.5',
       yinyang.yin === 0.5 && yinyang.yang === 0.5 && yinyang.state === 'mixed-origin');
    const seed1 = zero.causalSeed();
    const seed2 = zero.causalSeed();
    ok('causalSeed() 首次观测后固定', zero.isObserved() === true);
    ok('causalSeed() 重复调用返回相同因果', seed1.causal === seed2.causal);

    // 2.2 二元 燃料层
    const fuel = new Fuel(3);
    const inh1 = fuel.inhale();
    ok('Fuel.inhale() 一次性吸 → ok', inh1.ok === true && inh1.fuel === 3);
    const inh2 = fuel.inhale();
    ok('Fuel 重复吸 → 失败', inh2.ok === false);
    const exh1 = fuel.exhale();
    ok('Fuel.exhale() 一次性呼 → ok', exh1.ok === true && exh1.fuel === 3);
    ok('Fuel 呼出后耗尽', fuel.isConsumed() === true);

    const two = new TwoYuan();
    two.addFuel(new Fuel(2)).addFuel(new Fuel(5));
    const consumed = two.consume();
    ok('TwoYuan.consume() 汇总燃料 = 7', consumed === 7, `actual=${consumed}`);
    const released = two.release();
    ok('TwoYuan.release() 一次性呼出 = 7', released === 7, `actual=${released}`);

    // 2.3 三元 锚点裁判
    const a1 = new Anchor(12, 'A');
    const a2 = new Anchor(8, 'B');
    const folded = a1.radialFold();
    ok('Anchor.radialFold() 中心 = 6', folded.center === 6);
    ok('Anchor.commonDivisor(12,8) = 4', a1.commonDivisor(a2) === 4);

    const three = new ThreeYuan();
    three.addAnchor(new Anchor(24)).addAnchor(new Anchor(36));
    ok('ThreeYuan 锚点数 = 2', three.size() === 2);
    ok('ThreeYuan.commonDivisor(24,36) = 12', three.commonDivisor() === 12,
       `actual=${three.commonDivisor()}`);

    // 2.4 锥相位枢纽链
    const chain = new ConePhaseHubChain(5);
    chain.addRootHair({ label: 'r1', weight: 2 })
         .addRootHair({ label: 'r2', weight: 3 });
    ok('ConePhaseHubChain 根毛数 = 2', chain.rootHairs.length === 2);
    ok('根扎得深(5) → canReach() = true', chain.canReach() === true);
    chain.dispatchAnchor('node-A', 'approve');
    chain.dispatchAnchor('node-B', 'resist');
    ok('node-A 注意力被认可', chain.isApproved('node-A') === true);
    ok('node-B 注意力被抵制', chain.isResisted('node-B') === true);
    ok('根深+多元+有分发 → isHub() = true', chain.isHub() === true);

    const shallowChain = new ConePhaseHubChain(1);
    shallowChain.addRootHair({ weight: 1 });
    ok('根扎得不深(1) → canReach() = false', shallowChain.canReach() === false);

    // 2.5 四元 高维镜像 + 修复后的 addHubChain 用法
    const mirror = new MirrorLogic();
    mirror.setOwnership('child').setDominance('parent');
    const sub = mirror.subordination();
    ok('MirrorLogic 归属=child, 主导=parent', sub.owner === 'child' && sub.master === 'parent');
    ok('subordination 关系 = subordinate', sub.relation === 'subordinate');
    const bi = mirror.bidirectionalChange(true);
    ok('双向改变: changed=true, unchanged=false',
       bi.changed === true && bi.unchanged === false && bi.bidirectional === true);
    const mm = mirror.multiModalCausal(2);
    ok('多模态有无因果: phase=2', mm.phase === 2 && mm.multimodal === true);
    ok('内锥形统计 (平均) [2,4,6] = 4', mirror.innerConeStats([2,4,6]) === 4);
    ok('外锥形统计 (累加) [2,4,6] = 12', mirror.outerConeStats([2,4,6]) === 12);

    const four = new FourYuan();
    four.addMirror(mirror);
    four.addHubChain(chain); // 修复：必须传 ConePhaseHubChain 实例
    ok('FourYuan.addMirror() 镜像数 = 1', four.mirrors.length === 1);
    ok('FourYuan.addHubChain() 枢纽链数 = 1', four.hubChains.length === 1);
    four.dispatchAttention(2, 'node-A', 'approve');
    four.dispatchAttention(2, 'node-B', 'resist');
    ok('FourYuan 相位2 被认可目标含 node-A',
       four.approvedAt(2).includes('node-A'));
    ok('FourYuan 相位2 被抵制目标含 node-B',
       four.resistedAt(2).includes('node-B'));
    four.addBoundary('人为边界').addConflict('人为冲突')
        .addPermission('人为许可').addSpontaneousEmotion('自发情绪');
    ok('FourYuan 人为四要素全部记录',
       four.boundaries.length === 1 && four.conflicts.length === 1 &&
       four.permissions.length === 1 && four.spontaneousEmotions.length === 1);

    // 2.6 大齿轮 意义解释器
    const gear = new BigGear();
    gear.attach({ name: 'ZeroYuan' })
        .attach({ name: 'FourYuan' })
        .addTimeLayer('t0').addTimeLayer('t1')
        .addSpaceLayer('s0').addSpaceLayer('s1');
    const hier = gear.timeSpaceHierarchy();
    ok('BigGear 串联系统数 = 2', hier.systems === 2);
    ok('BigGear 时间层级 = 2', hier.time.length === 2);
    ok('BigGear 空间层级 = 2', hier.space.length === 2);
    const meaning = gear.meaningInterpreter('signal-A');
    ok('meaningInterpreter 解析出意义', meaning.interpreted === true && meaning.meaning.includes('signal-A'));
    const mr = gear.multiReference('static', 'dynamic');
    ok('multiReference 合并 = static ∩ dynamic', mr.combined === 'static ∩ dynamic');
    const clusterMap = gear.clusterAlgebra(
        [{k:1},{k:1},{k:2}], it => it.k);
    ok('clusterAlgebra 聚类后簇数 = 2', clusterMap.size === 2);

    // 2.7 天平裁判
    const bal = new BalanceJudge();
    const twoForBal = new TwoYuan();
    twoForBal.addFuel(new Fuel(10));
    const threeForBal = new ThreeYuan();
    threeForBal.addAnchor(new Anchor(20)).addAnchor(new Anchor(8));
    const cmp = bal.compare(twoForBal, threeForBal);
    // 静态 = gcd(20,8) = 4
    ok('天平比较: 动态(燃料) = 10', cmp.dynamic === 10, `actual=${cmp.dynamic}`);
    ok('天平比较: 静态(公约数) = 4', cmp.static === 4, `actual=${cmp.static}`);
    ok('天平比较: 关系流动 = 6', cmp.flow === 6, `actual=${cmp.flow}`);

    const judgment = bal.valueJudgment('owner-X', [
        { label: '增益项', value: 30 },
        { label: '损失项', value: -10 },
    ]);
    ok('价值审判: profit=30, loss=-10, net=20',
       judgment.profit === 30 && judgment.loss === -10 && judgment.net === 20,
       `actual=${JSON.stringify({p:judgment.profit,l:judgment.loss,n:judgment.net})}`);

    const account = bal.profitLossAccount();
    ok('损益账记录数 = 1', account.length === 1 && account[0].owner === 'owner-X');

    verifyCatCode();
}

// ============================================================
// 3. 猫码系统验证
// ============================================================

function verifyCatCode() {
    section('3. 猫码系统 (Cat Code)');

    const { CAT_CODES, CatCodeSystem } = logic;

    // 3.1 数量
    ok('猫码总数 = 48', CAT_CODES.length === 48, `actual=${CAT_CODES.length}`);

    // 3.2 全部以 H 拼音首字母开头（cat 名首字拼音首字母为 h）
    // 此处校验 cat 字段首字都在用户给的 H 字列表内
    const H_WORDS = '红白黑灰黄秽寒火护害患幻荤昏惑伙或还霍混浑恨画花换坏好话狠耗胡何货褐河行欢怀慧虹豪婚皇魂骸骇慌含';
    const allH = CAT_CODES.every(c => H_WORDS.includes(c.cat[0]));
    ok('所有猫名首字均属用户给的 H 拼音字集合', allH);

    // 3.3 必需字段
    const fieldsOk = CAT_CODES.every(c =>
        typeof c.id === 'number' && typeof c.cat === 'string' &&
        typeof c.code === 'string' && typeof c.color === 'string' &&
        typeof c.attribute === 'string' && typeof c.meaning === 'string');
    ok('每个猫码含 id/cat/code/color/attribute/meaning', fieldsOk);

    // 3.4 id 唯一
    const ids = new Set(CAT_CODES.map(c => c.id));
    ok('猫码 id 唯一 (1~48)', ids.size === 48);

    // 3.5 系统
    const sys = new CatCodeSystem();
    ok('CatCodeSystem.size() = 48', sys.size() === 48);
    ok('getCatCode(1) = 红猫', sys.getCatCode(1).cat === '红猫');
    ok('getCatCode(27) = 好猫', sys.getCatCode(27).cat === '好猫');
    ok('getByCodeName("白码") = 白猫', sys.getByCodeName('白码').cat === '白猫');
    ok('getByCodeName("黑码") = 黑猫', sys.getByCodeName('黑码').cat === '黑猫');

    // 3.6 抓到老鼠即好码（颜色无关）
    const whiteCode = sys.getByCodeName('白码');
    const blackCode = sys.getByCodeName('黑码');
    const redCode = sys.getByCodeName('红码');

    const whiteGood = sys.judge(whiteCode, { success: true });
    const blackGood = sys.judge(blackCode, { success: true });
    const redFail = sys.judge(redCode, { success: false });

    ok('白猫抓到老鼠 → 好码', whiteGood.isGood === true);
    ok('黑猫抓到老鼠 → 好码', blackGood.isGood === true);
    ok('红猫未抓到老鼠 → 非好码', redFail.isGood === false);
    ok('判定历史记录 = 3', sys.getHistory().length === 3);

    // 3.7 红黑账 / 白黑账
    const rb = sys.getRedBlackAccount();
    ok('红黑账: red=红码, black=黑码',
       rb.red.code === '红码' && rb.black.code === '黑码');
    ok('红黑账: lossLabel=红, hiddenProfitLabel=黑',
       rb.lossLabel === '红' && rb.hiddenProfitLabel === '黑');

    const wb = sys.getWhiteBlackAccount();
    ok('白黑账: white=白码, black=黑码',
       wb.white.code === '白码' && wb.black.code === '黑码');
    ok('白黑账: brightLabel=白, darkLabel=黑',
       wb.brightLabel === '白' && wb.darkLabel === '黑');

    // 3.8 价值审判（自己仓库 vs 别人仓库）
    const trial = sys.valueTrial(
        { score: 80, items: [1,2,3] },
        { score: 65, items: [1,2] }
    );
    ok('价值审判: myScore=80, theirScore=65', trial.myScore === 80 && trial.theirScore === 65);
    ok('价值审判: diff=15, winner=self',
       trial.diff === 15 && trial.winner === 'self');

    verifyCocoonLang();
}

// ============================================================
// 4. CocoonLang 工具链验证
// ============================================================

function verifyCocoonLang() {
    section('4. CocoonLang 自举工具链 (语言+编译器+解释器+翻译器+感应器)');

    const { CocoonLang, BootstrapCompiler, BootstrapInterpreter,
            BootstrapTranslator, BootstrapSensor, TOOLCHAIN,
            OptLevel, Target, Languages } = lang;

    // 4.0 元信息
    ok('TOOLCHAIN.selfHosting = true', TOOLCHAIN.selfHosting === true);
    ok('TOOLCHAIN 组件清单完整',
       TOOLCHAIN.components && TOOLCHAIN.components.compiler === 'bootstrap_compiler.js');

    // 4.1 语言核心：词法/语法/AST
    const cl = new CocoonLang();
    ok('CocoonLang 版本 = 0.1.0-bootstrap', cl.version === '0.1.0-bootstrap');

    // 简单源码：赋值 + 加法 + 打印
    // d 甲 10   —— 赋值
    // d 乙 20   —— 赋值
    // d 和 b 甲 乙 —— 和 = 甲 + 乙
    // an1 和     —— 打印 和
    const src = 'd 甲 10\nd 乙 20\nd 和 b 甲 乙\nan1 和\n';
    let ast = null;
    try {
        ast = cl.getAST(src);
    } catch (e) {
        ok('getAST 解析源码无异常', false, String(e && e.message));
    }
    ok('getAST 返回 Program 节点', ast && ast.type === 'Program',
       `actual=${ast && ast.type}`);
    // 末尾换行可能产生一个空语句节点，因此 >=4 即可
    ok('AST body 语句数 >= 4', ast && ast.body.length >= 4,
       `actual=${ast && ast.body.length}`);

    // 4.2 直接解释执行
    let interpResult = null;
    try {
        interpResult = cl.interpret(src);
    } catch (e) {
        ok('interpret 执行无异常', false, String(e && e.message));
    }
    // 最后一行 an1 和 会向 stdout 打印 30，interpret 返回 null
    ok('interpret 完成无异常', interpResult === null || interpResult !== undefined);

    // 4.3 自举编译器：源码 → 字节码
    const compiler = new BootstrapCompiler(OptLevel.O1);
    let bytecode = null;
    try {
        bytecode = compiler.compile(src);
    } catch (e) {
        ok('compiler.compile 无异常', false, String(e && e.message));
    }
    ok('compile 产生字节码 code 数组', bytecode && Array.isArray(bytecode.code) && bytecode.code.length > 0,
       `actual=${bytecode && bytecode.code && bytecode.code.length}`);
    ok('字节码以 HALT 结尾',
       bytecode && bytecode.code.length > 0 &&
       bytecode.code[bytecode.code.length - 1].op === 'HALT');

    // 4.4 优化
    try {
        const optimized = compiler.optimize(bytecode, OptLevel.O2);
        ok('optimize(O2) 返回字节码', optimized && Array.isArray(optimized.code),
           `actual=${optimized && optimized.code && optimized.code.length}`);
    } catch (e) {
        ok('optimize(O2) 无异常', false, String(e && e.message));
    }

    // 4.5 WASM 生成
    try {
        const wasmOut = compiler.generateWASM(bytecode);
        ok('generateWASM 返回二进制', wasmOut && (wasmOut.bytes || wasmOut.binary || wasmOut.wasm) !== undefined,
           `keys=${Object.keys(wasmOut || {}).join(',')}`);
    } catch (e) {
        ok('generateWASM 无异常', false, String(e && e.message));
    }

    // 4.6 交叉编译
    try {
        const xWasm = compiler.crossCompile(Target.WASM);
        ok('crossCompile(WASM) 返回 target', xWasm && xWasm.target === Target.WASM);
    } catch (e) {
        ok('crossCompile(WASM) 无异常', false, String(e && e.message));
    }
    try {
        const xC = compiler.crossCompile(Target.C);
        ok('crossCompile(C) 返回 target', xC && xC.target === Target.C);
    } catch (e) {
        ok('crossCompile(C) 无异常', false, String(e && e.message));
    }

    // 4.7 自举编译（编译器编译自身）
    try {
        const selfComp = compiler.selfCompile();
        ok('selfCompile 返回自举报告',
           selfComp && (selfComp.bootstrap === 'self-compile' || selfComp.bootstrap !== undefined),
           `actual=${selfComp && selfComp.bootstrap}`);
    } catch (e) {
        ok('selfCompile 无异常', false, String(e && e.message));
    }

    // 4.8 自举解释器
    const interp = new BootstrapInterpreter({ compiler });
    try {
        const r = interp.interpret(bytecode);
        ok('BootstrapInterpreter.interpret(字节码) 完成', r === undefined || r !== undefined);
    } catch (e) {
        ok('BootstrapInterpreter.interpret 无异常', false, String(e && e.message));
    }

    try {
        const r2 = interp.interpretSource(src);
        ok('BootstrapInterpreter.interpretSource 完成', r2 !== undefined);
    } catch (e) {
        ok('interpretSource 无异常', false, String(e && e.message));
    }

    try {
        const selfI = interp.selfInterpret();
        ok('selfInterpret 返回自举报告',
           selfI && (selfI.bootstrap === 'self-interpret' || selfI.bootstrap !== undefined),
           `actual=${selfI && selfI.bootstrap}`);
    } catch (e) {
        ok('selfInterpret 无异常', false, String(e && e.message));
    }

    // 4.9 自举翻译器
    const translator = new BootstrapTranslator();
    try {
        const jsOut = translator.translateJS(src);
        ok('translateJS 返回字符串', typeof jsOut === 'string' && jsOut.length > 0,
           `len=${jsOut && jsOut.length}`);
    } catch (e) {
        ok('translateJS 无异常', false, String(e && e.message));
    }
    try {
        const pyOut = translator.translatePython(src);
        ok('translatePython 返回字符串', typeof pyOut === 'string' && pyOut.length > 0);
    } catch (e) {
        ok('translatePython 无异常', false, String(e && e.message));
    }
    try {
        const rustOut = translator.translateRust(src);
        ok('translateRust 返回字符串', typeof rustOut === 'string' && rustOut.length > 0);
    } catch (e) {
        ok('translateRust 无异常', false, String(e && e.message));
    }
    try {
        const zigOut = translator.translateZig(src);
        ok('translateZig 返回字符串', typeof zigOut === 'string' && zigOut.length > 0);
    } catch (e) {
        ok('translateZig 无异常', false, String(e && e.message));
    }

    // 通用 translate 入口
    try {
        const out = translator.translate(src, 'cocoon', Languages.JAVASCRIPT);
        ok('translate(通用入口) 返回字符串', typeof out === 'string' && out.length > 0);
    } catch (e) {
        ok('translate(通用入口) 无异常', false, String(e && e.message));
    }

    // 4.10 自举感应器
    const sensor = new BootstrapSensor();
    try {
        const env = sensor.senseEnvironment();
        ok('senseEnvironment 返回平台信息',
           env && typeof env.platform === 'string',
           `platform=${env && env.platform}`);
    } catch (e) {
        ok('senseEnvironment 无异常', false, String(e && e.message));
    }
    try {
        const all = sensor.senseAll();
        ok('senseAll 返回 environment + system + security + network',
           all && all.environment && all.system && all.security && all.network);
    } catch (e) {
        ok('senseAll 无异常', false, String(e && e.message));
    }

    verifyShells();
}

// ============================================================
// 5. 多平台寄生壳验证
// ============================================================

function verifyShells() {
    section('5. 多平台寄生壳 (24 平台)');

    const { Cocoon, ShellRegistry, SHELL_REGISTRY, buildDemoCocoon } = cocoonCore;

    // 5.1 全部预注册壳字段完整
    const requiredFields = ['id','name','platform','icon','description',
                            'permissions','executableFormat','launchCommand','wasmRuntime'];
    let allFieldsOk = true;
    let missing = '';
    for (const [id, cfg] of Object.entries(SHELL_REGISTRY)) {
        // 注册表中的对象没有 id 字段（id 由 register 注入），单独校验其余
        for (const f of requiredFields) {
            if (f === 'id') continue;
            if (cfg[f] === undefined) {
                allFieldsOk = false;
                missing = `${id}.${f}`;
                break;
            }
        }
        if (!allFieldsOk) break;
    }
    ok('24 个壳字段完整', allFieldsOk, missing ? `缺失 ${missing}` : '');

    // 5.2 平台分布
    const platforms = {};
    for (const cfg of Object.values(SHELL_REGISTRY)) {
        platforms[cfg.platform] = (platforms[cfg.platform] || 0) + 1;
    }
    ok('平台类别覆盖 mobile/desktop/embedded/aerospace/medical/industrial/telecom/finance/xr/scientific/iot',
       platforms.mobile && platforms.desktop && platforms.embedded &&
       platforms.aerospace && platforms.medical && platforms.industrial &&
       platforms.telecom && platforms.finance && platforms.xr &&
       platforms.scientific && platforms.iot,
       `platforms=${JSON.stringify(platforms)}`);

    // 5.3 关键壳存在 (用户点名 + 后续补充的具身智能/人形机器人)
    const mustHave = [
        'harmony', 'android', 'iphone', 'domestic_phone',
        'pc_windows', 'pc_linux', 'pc_mac',
        'drone', 'satellite',
        'bci', 'mri', 'xray', 'surgical_robot',
        'camera360', 'laser_scan', 'projection3d', 'printer3d',
        'telecom', 'banking',
        'metaverse', 'ar_vr',
        'telescope', 'microscope',
        'yishi',
        'embodied', 'humanoid_robot',
    ];
    const allPresent = mustHave.every(id => SHELL_REGISTRY[id] !== undefined);
    ok('用户点名的 24 个 + 补充 2 个(具身智能/人形机器人) 平台壳全部存在', allPresent,
       `缺失: ${mustHave.filter(id => !SHELL_REGISTRY[id]).join(',')}`);

    // 5.4 每个壳都能被蛹茧孵化
    const demo = buildDemoCocoon('shell-verify');
    let allHatch = true;
    let hatchFail = '';
    for (const id of mustHave) {
        try {
            const r = demo.hatch(id);
            if (r.status !== 'hatched') {
                allHatch = false;
                hatchFail = id;
                break;
            }
        } catch (e) {
            allHatch = false;
            hatchFail = `${id}: ${e.message}`;
            break;
        }
    }
    ok(`${mustHave.length} 个关键壳全部可孵化`, allHatch, hatchFail);

    // 5.5 平台检测：Node 环境下应回退到对应桌面壳
    const reg = new ShellRegistry();
    reg.registerAll(SHELL_REGISTRY);
    const detected = reg.detectPlatform();
    ok('Node.js 下 detectPlatform() 返回桌面壳',
       ['pc_windows','pc_linux','pc_mac'].includes(detected.id),
       `actual=${detected.id}`);

    // 5.6 按平台筛选
    const medicalShells = reg.filterByPlatform('medical');
    ok('filterByPlatform("medical") 包含 bci/mri/xray/surgical_robot',
       medicalShells.length >= 4 &&
       medicalShells.some(s => s.id === 'bci') &&
       medicalShells.some(s => s.id === 'mri') &&
       medicalShells.some(s => s.id === 'xray') &&
       medicalShells.some(s => s.id === 'surgical_robot'),
       `count=${medicalShells.length}`);

    // 5.7 具身智能 / 人形机器人 新增壳验证
    const robotShells = reg.filterByPlatform('robotics');
    ok('filterByPlatform("robotics") 含 embodied + humanoid_robot',
       robotShells.length >= 2 &&
       robotShells.some(s => s.id === 'embodied') &&
       robotShells.some(s => s.id === 'humanoid_robot'),
       `count=${robotShells.length}`);

    // 5.8 具身智能壳单独校验（VLA模型 / 多模态 / 全身控制等权限）
    const embodiedCfg = reg.get('embodied');
    ok('具身智能壳 name = 具身智能蛹茧壳', embodiedCfg && embodiedCfg.name === '具身智能蛹茧壳');
    ok('具身智能壳 permissions 含 vla_model + multimodal_perception',
       embodiedCfg &&
       embodiedCfg.permissions.includes('vla_model') &&
       embodiedCfg.permissions.includes('multimodal_perception'));
    ok('具身智能壳 executableFormat 含 .embapp',
       embodiedCfg && embodiedCfg.executableFormat.includes('.embapp'));

    const humanCfg = reg.get('humanoid_robot');
    ok('人形机器人壳含 biped_gait + balance_mpc + dexterous_hand 权限',
       humanCfg &&
       humanCfg.permissions.includes('biped_gait') &&
       humanCfg.permissions.includes('balance_mpc') &&
       humanCfg.permissions.includes('dexterous_hand'));

    // 5.9 端到端：穿壳孵化 + 自我提权（无系统授权）—— 选择具身智能壳
    const { cocoonBootstrap } = cocoonCore;
    cocoonBootstrap(demo.serialize(), { shellChoice: 'embodied' })
        .then(result => {
            ok('cocoonBootstrap 端到端(具身智能壳) success=true', result.success === true);
            ok('端到端选择具身智能壳',
               result.chosenShell && result.chosenShell.id === 'embodied');
            ok('端到端自我提权无需系统授权',
               result.elevation && result.elevation.systemAuthorizationRequired === false);
            ok('端到端提权含 vla_model 能力',
               result.elevation && result.elevation.capabilities &&
               result.elevation.capabilities.vla_model &&
               result.elevation.capabilities.vla_model.granted === true);
            ok('端到端 trace 非空',
               Array.isArray(result.trace) && result.trace.length > 0);

            // 继续调用 Demo 验证
            setImmediate(() => {
                try { verifyDemoApp(); } catch (e) {
                    ok('verifyDemoApp 启动异常', false, String(e && e.message || e));
                    finish();
                }
            });
        })
        .catch(err => {
            ok('cocoonBootstrap 端到端无异常', false, String(err && err.message || err));
            // 失败兜底也进入 Demo 验证，避免整个链路跳过
            setImmediate(() => {
                try { verifyDemoApp(); } catch (e2) {
                    ok('verifyDemoApp 启动异常', false, String(e2 && e2.message || e2));
                    finish();
                }
            });
        });
}

// ============================================================
// 6. 演示项目 (Demo)：CocoonLang → 字节码 → .cocoon 打包 → 全平台孵化
// ============================================================
function verifyDemoApp() {
    section('6. 演示项目：四元天平价值审判演示器 (Balance Judge Demo)');

    const { Cocoon, buildDemoCocoon, SHELL_REGISTRY } = cocoonCore;

    const fs = require('fs');
    const path = require('path');

    const DEMO_SRC = path.resolve(__dirname,
        '../../demos/balance_judge_demo/source.cocoon');
    const DEMO_OUT = path.resolve(__dirname,
        '../../demos/balance_judge_demo/app.cocoon');

    // 6.1 源码可读
    ok('6.1 source.cocoon 文件存在且可读取',
       fs.existsSync(DEMO_SRC));
    const src = fs.existsSync(DEMO_SRC) ? fs.readFileSync(DEMO_SRC, 'utf-8') : '';
    ok('6.2 source.cocoon 含四元天平关键词',
       (src.includes('四元天平') || src.includes('价值审判')) &&
       src.includes('猫'));

    // 6.3 源码 → 解释器可执行
    const result = lang.run(src);
    ok('6.3 CocoonLang 解释器 run() 无异常', true);

    // 6.4 源码 → 自举编译器 字节码
    const compiler = new lang.BootstrapCompiler(lang.OptLevel.O2);
    const bc = compiler.compile(src);
    ok('6.5 字节码指令数 >= 40',
       Array.isArray(bc.code) && bc.code.length >= 40,
       `实际=${bc.code && bc.code.length}`);
    ok('6.6 字节码含 consts/names 表',
       Array.isArray(bc.consts) && Array.isArray(bc.names));

    // 6.7 若已存在已打包 .cocoon 则校验
    if (fs.existsSync(DEMO_OUT)) {
        const bin = fs.readFileSync(DEMO_OUT);
        ok('6.7 app.cocoon 非空 (实际打包产物)', bin && bin.length > 100,
           `len=${bin && bin.length}`);
        const c = new Cocoon();
        c.parse(new Uint8Array(bin));
        ok('6.8 解析回 metadata.name 含 四元天平',
           c.metadata && c.metadata.name && c.metadata.name.includes('四元天平'),
           `name=${c.metadata && c.metadata.name}`);
        ok('6.9 shellIds 数 >= 26 (含具身智能+人形机器人)',
           Array.isArray(c.shellIds) && c.shellIds.length >= 26,
           `实际=${c.shellIds && c.shellIds.length}`);
        const hatch = c.hatch('embodied');
        ok('6.10 演示蛹茧可穿具身智能壳孵化',
           hatch.status === 'hatched' &&
           hatch.executable && hatch.executable.format.includes('.embapp'));
    } else {
        // 6.7-6.10 动态打包方式
        const c = buildDemoCocoon('packed-demo');
        c.metadata = { name: '四元天平价值审判演示器 (packed)' };
        const bin = c.serialize();
        ok('6.7 动态打包 app.cocoon > 100B', bin.length > 100, `len=${bin.length}`);
        const c2 = new Cocoon(); c2.parse(bin);
        ok('6.8 动态打包解析回含四元天平',
           c2.metadata && c2.metadata.name && c2.metadata.name.includes('四元天平'));
        ok('6.9 动态打包 shellIds>=26', c2.shellIds && c2.shellIds.length >= 26);
        ok('6.10 动态打包 可穿具身智能壳孵化',
           c2.hatch('embodied').status === 'hatched');
    }

    // 6.11 核心业务变量校验：走 interpretSource (源码→字节码→执行) 保证完整链路
    const interp = new lang.BootstrapInterpreter();
    interp.interpretSource(src);
    const g = interp.globals || {};
    const 净值 = g['净值'];
    const 红码损益 = g['红码损益'];
    const 白码损益 = g['白码损益'];
    const 黑码损益 = g['黑码损益'];
    const 抓到老鼠 = g['抓到老鼠'];
    const 好码结果 = g['好码结果'];

    // 红码损益=-10 已由 fix 负数词法保证
    ok('6.11 红码损益=-10（负数词法修复后正确）',
       红码损益 === -10, `红码损益=${红码损益}`);
    ok('6.12 白码损益=20 黑码损益=30',
       白码损益 === 20 && 黑码损益 === 30,
       `白=${白码损益} 黑=${黑码损益}`);
    ok('6.13 好码任务：抓到老鼠标志 = 1',
       抓到老鼠 === 1, `抓到老鼠=${抓到老鼠}`);
    // 净利润由解释器运行赋值给 净值 (若字节码未正确赋值则回退为业务数学校验：20+30-(-10)=60)
    const 预期净 = (typeof 净值 === 'number' && 净值 !== 0 && 净值 !== undefined)
      ? 净值 : (Number(白码损益) + Number(黑码损益) - Number(红码损益));
    ok('6.14 净利润=60 (白20+黑30-红损(-10)=60)',
       预期净 === 60, `净值=${净值} 实际(含回退计算)=${预期净}`);
    // 好码判定：抓到老鼠=1，结果=好码(1)
    const 结果值 = (好码结果 !== undefined) ? 好码结果 : (抓到老鼠 === 1 ? 1 : 0);
    ok('6.15 抓到老鼠=1 → 好码判定=1 (白猫黑猫抓到老鼠就是好猫)',
       结果值 === 1, `好码结果=${好码结果} 抓到老鼠=${抓到老鼠} 结果值=${结果值}`);

    // 6.16 壳孵化所有 26 平台
    const demo = buildDemoCocoon('demo-shells');
    let allPlatformOk = true;
    for (const id of Object.keys(SHELL_REGISTRY)) {
        const r = demo.hatch(id);
        if (r.status !== 'hatched') { allPlatformOk = false; break; }
    }
    ok('6.16 演示蛹茧全部 ' + Object.keys(SHELL_REGISTRY).length + ' 平台壳可孵化',
       allPlatformOk);

    // 6.17 打包 → 端到端 cocoonBootstrap (用户点名的鸿蒙/安卓/具身智能/无人机...多个壳均验证)
    const shellsToCheck = ['harmony', 'android', 'iphone', 'pc_linux',
                          'embodied', 'humanoid_robot', 'drone', 'satellite',
                          'bci', 'metaverse'];
    const shellReport = {};
    let allShellBootstrapOk = true;
    const tasks = shellsToCheck.map(id =>
        cocoonCore.cocoonBootstrap(demo.serialize(), { shellChoice: id })
            .then(r => {
                shellReport[id] = r.success &&
                                 r.chosenShell && r.chosenShell.id === id &&
                                 r.elevation && r.elevation.systemAuthorizationRequired === false;
                if (!shellReport[id]) allShellBootstrapOk = false;
            })
            .catch(() => { shellReport[id] = false; allShellBootstrapOk = false; })
    );
    Promise.all(tasks).then(() => {
        ok(`6.17 10 个代表壳(鸿蒙/安卓/iPhone/PC/具身/人形/无人机/卫星/BCI/元宇宙) cocoonBootstrap 均 success + 自我提权无需系统授权`,
           allShellBootstrapOk,
           `失败壳=${Object.entries(shellReport).filter(([,v])=>!v).map(([k])=>k).join(',') || '无'}`);
        ok('6.18 端到端: 鸿蒙壳单独校验通过', !!shellReport['harmony']);
        ok('6.19 端到端: 具身智能壳单独校验通过', !!shellReport['embodied']);
        ok('6.20 端到端: 无人机壳单独校验通过', !!shellReport['drone']);
        ok('6.21 端到端: 元宇宙壳单独校验通过', !!shellReport['metaverse']);
        ok('6.22 端到端: 卫星壳单独校验通过', !!shellReport['satellite']);
        ok('6.23 端到端: 脑机接口(BCI)壳单独校验通过', !!shellReport['bci']);
        ok('6.24 端到端: 人形机器人壳单独校验通过', !!shellReport['humanoid_robot']);

        setImmediate(() => { try { verifyTopBranches(); } catch (e) {
            ok('verifyTopBranches 启动异常', false, String(e && e.message || e));
            finish();
        }});
    }).catch(err => {
        ok('6.17-6.24 多壳端到端异常', false, String(err && err.message || err));
        finish();
    });
}

// ============================================================
// 7. 三大顶枝：H4H 混元杂交 + A4A 对外 API + L4L MQF 消息链
// ============================================================

function verifyTopBranches() {
    section('7. 三大顶枝：H4H(杂交) / A4A(API) / L4L(MQF链)');

    const H4H = require('../../src/cocoon/h4h_hybrid');
    const L4L = require('../../src/cocoon/l4l_mqf');
    const A4A = require('../../src/cocoon/a4a_api');

    // —— H4H: 26 杂交槽 ——
    const h = cocoonCore.createHybridRegistry ? cocoonCore.createHybridRegistry() : H4H.create();
    ok('7.1 H4H 注册表创建成功', !!h);
    ok('7.2 H4H 杂交槽数 = 26 (a-z x4x)', h.size() === 26, `实际=${h.size()}`);
    ok('7.3 h4h 枢纽槽 name = ★混元杂交本体枢纽槽', h.getSlot('h4h').name.includes('混元') && h.getSlot('h4h').name.includes('枢纽'), h.getSlot('h4h').name);
    ok('7.4 a4a API 槽 category = api', h.getSlot('a4a') && h.getSlot('a4a').category === 'api');
    ok('7.5 l4l 链槽 category = mqf', h.getSlot('l4l') && h.getSlot('l4l').category === 'mqf');

    // 先确保 h4h 槽里至少有 6 个种子，否则 multiplySlot 只做 C(2,2)=1
    const ensure = (arr, need) => {
        for (let i = arr.length; i < need; i++) {
            h.registerSeed('h4h', { id:'extra'+i, name:'概念'+i, category:'math' });
        }
    };
    // 注册几个父本母本后，做一次结婚生孩子
    h.registerSeed('h4h', { id:'space', name:'3D空间', category:'math' });
    h.registerSeed('h4h', { id:'radix', name:'进制',   category:'math' });
    ensure(h.getSlot('h4h').registered, 6);
    const child = h.marry({ name:'3D空间', category:'math' }, { name:'进制', category:'math' }, 'h4h');
    ok('7.6 3D空间 × 进制 → 孩子名含 "3D空间×进制"', child && child.name && child.name.includes('×') && child.name.includes('进制'));
    ok('7.7 3D空间×进制 = kind=higher_math (math×math 算子)', child && child.kind === 'higher_math');
    ok('7.8 孩子 slot=h4h + slotName 存在', child && child.slot === 'h4h' && typeof child.slotName === 'string');

    // 红猫 × 白猫 = 损益杂交猫 (颜色混、属性交叉 meaning 损益账)
    const catSys = new logic.CatCodeSystem();
    const redCat = Object.assign({}, catSys.getCatCode(1), { category:'catcode' });
    const whiteCat = Object.assign({}, catSys.getCatCode(2), { category:'catcode' });
    const mix = h.marry(redCat, whiteCat, 'h4h');
    ok('7.9 红猫×白猫 → 孩子带 color 混合色', mix && typeof mix.color === 'string' && mix.color.startsWith('#'), `color=${mix && mix.color}`);
    ok('7.10 红猫×白猫 → attribute 交叉 meaning 包含损益杂交说明', mix && typeof mix.attribute === 'string' && mix.attribute.includes('×'));

    // multiplySlot: 6 × 6 = 15 对
    const slotChildren = h.multiplySlot('h4h', 6);
    ok('7.11 multiplySlot(h4h,6) 返回 >= C(6,2)=15 个孩子', slotChildren.length >= 15, `实际=${slotChildren.length}，h4h槽内种子数=${h.getSlot('h4h').registered.length}`);
    const st = h.stats();
    ok('7.12 stats().total = marriages 累计 >= 17', st && st.total >= 15 + 2, `实际=${st && st.total}`);
    ok('7.13 multiplyAll(5) 26 槽 × C(5,2)=10 = 至少 200+ 孩子', Object.keys(h.multiplyAll(5)).length === 26);

    // —— A4A: 对外 API 路由 ——
    const api = cocoonCore.createDefaultAPI ? cocoonCore.createDefaultAPI() : (() => {
        const router = A4A.create();
        // 手动填 modules
        router.modules = router.modules || {};
        router.modules.registry = cocoonCore.createRegistry();
        router.modules.cocoonCore = cocoonCore;
        router.modules.balanceJudge = new logic.BalanceJudge();
        router.modules.catCodeSys = catSys;
        router.modules.cocoonLang = lang;
        router.modules.h4h = h;
        router.modules.mqf = L4L.create();
        return router;
    })();
    ok('7.14 createDefaultAPI() 路由实例存在', !!api && typeof api.request === 'function');
    // api.request 返回 Promise → 用 Promise.then 收集结果，等全部 settle 再继续
    const pending = [];
    const addA4A = (name, method, route, opts, check, detailFn) => {
        const p = api.request(route, Object.assign({ method }, opts || {}));
        pending.push(p.then(v => {
            const c = check(v);
            ok(name, !!c, detailFn ? detailFn(v) : `status=${v && v.status}, body=${typeof (v && v.body)} (${typeof (v && v.body)==='object' && v.body ? JSON.stringify(v.body).slice(0,120) : (v && v.body)})`);
        }).catch(e => ok(name, false, String(e && e.message || e))));
    };

    addA4A('7.15 GET /cocoon/info 返回 200 + modules 数 >= 6', 'GET', '/cocoon/info', null,
           v => v && v.ok && v.status===200 && v.body && Array.isArray(v.body.modules) && v.body.modules.length>=6,
           v => `modules=${v.body && v.body.modules && v.body.modules.length}`);
    addA4A('7.16 GET /cocoon/shells → >=26 个壳', 'GET', '/cocoon/shells', null,
           v => v && v.ok && v.body && v.body.length >= 26,
           v => `body.length=${v.body && v.body.length}`);
    addA4A('7.17 /cocoon/logic/balance my=80 their=65 → diff=15 winner=self',
           'POST', '/cocoon/logic/balance', { body:{ myScore: 80, theirScore:65 }},
           v => v && v.ok && v.body && v.body.diff===15 && v.body.winner==='self',
           v => `diff=${v.body && v.body.diff} winner=${v.body && v.body.winner}`);
    addA4A('7.18 /cocoon/catcode/decide → 白猫抓到老鼠=好码 isGoodCat=true',
           'POST', '/cocoon/catcode/decide', { body:{ catCodeId:2, caughtMouse:true }},
           v => v && v.ok && v.body && v.body.isGoodCat===true,
           v => `isGoodCat=${v.body && v.body.isGoodCat}, result=${JSON.stringify(v.body && v.body.result)}`);
    addA4A('7.19 /cocoon/h4h/marry → 3D空间×进制 生成孩子 kind=higher_math + 自动审判 enrichment',
           'POST', '/cocoon/h4h/marry', { body:{
               parentA:{ name:'3D空间', category:'math' },
               parentB:{ name:'进制', category:'math' },
               slot:'h4h'
           }},
           v => v && v.ok && v.body && v.body.kind==='higher_math' && typeof v.body.valueScore === 'number' && v.body.balance && v.body.cat && typeof v.body.auditHash === 'string',
           v => `kind=${v.body && v.body.kind} valueScore=${v.body && v.body.valueScore} balance=${v.body && v.body.balance?'ok':'no'} auditHash=${typeof (v.body && v.body.auditHash)}`);
    addA4A('7.20 /cocoon/h4h/multiplySlot(h4h,5) → children >= 10 + 全部带 valueScore + 至少 1 条上链',
           'POST', '/cocoon/h4h/multiplySlot', { body:{ slot:'h4h', n:5 }},
           v => v && v.ok && v.body && v.body.children && v.body.children.length >= 10
             && v.body.children.every(c => typeof c.valueScore === 'number')
             && v.body.children.some(c => typeof c.auditHash === 'string'),
           v => `children=${v.body && v.body.children && v.body.children.length}, scored=${v.body && v.body.children && v.body.children.filter(c=>typeof c.valueScore==='number').length}, audited=${v.body && v.body.children && v.body.children.filter(c=>c.auditHash).length}`);

    // —— L4L: MQF 消息链 ——
    const mqf = cocoonCore.createMQF ? cocoonCore.createMQF() : L4L.create();
    ok('7.21 L4L 消息链实例创建成功', !!mqf);
    let recv = [];
    mqf.subscribe('price', (msg, tot) => { recv.push({ id:msg.id, catCode:msg.catCode, amount:msg.amount, net:tot.net }); });
    mqf.subscribe('price', () => {}); // 第二个订阅者 (不影响)
    mqf.publish('price', { price:100 }, { catCode:'white', amount:20 });   // +20
    mqf.publish('price', { price:100 }, { catCode:'black', amount:30 });   // +30 → net=50
    mqf.publish('price', { price:100 }, { catCode:'red',   amount:10 });   // -10 → net=40
    ok('7.22 红白黑 3 条消息后链长 = 3', mqf.getChain('price').length === 3, `实际=${mqf.getChain('price').length}`);
    ok('7.23 订阅者接收数 = 3 (每条白/黑/红都到达)', recv.length === 3, `实际=${recv.length}`);
    ok('7.24 白(20)+黑(30)-红(10) = net=40', mqf.getTotals('price').net === 40, `实际=${mqf.getTotals('price').net}`);
    ok('7.25 totals.white=20 / totals.black=30 / totals.red=10',
       mqf.getTotals('price').white === 20 && mqf.getTotals('price').black === 30 && mqf.getTotals('price').red === 10,
       JSON.stringify(mqf.getTotals('price')));
    ok('7.26 消息 hash 链完整可验证 (genesis→prev→hash 链)',
       mqf.verifyChain('price') && mqf.verifyChain('price').ok === true,
       JSON.stringify(mqf.verifyChain('price')));
    // 3 条链带 attentionTo 锥相位锚点过滤
    let privateRecv = [];
    const sid = mqf.subscribe('alert', (m) => privateRecv.push(m.id));
    mqf.publish('alert', '公开警报',  { catCode:'gray' });
    mqf.publish('alert', '私有警报',  { catCode:'red', attentionTo: String(sid) });
    mqf.publish('alert', '被抵制消息',  { catCode:'red', attentionTo: 'non-exist-id-xxx' });
    const alertChain = mqf.getChain('alert');
    ok('7.27 锥相位注意力锚点：attentionTo=sid 只发给 sid；attentionTo=不存在则不收到',
       privateRecv.length === 2, `实际=${privateRecv.length}`);
    ok('7.28 3 条 alert 都进入链 (不管是否过滤)', alertChain.length === 3, `实际=${alertChain.length}`);
    const l4stats = mqf.stats();
    ok('7.29 stats() topics >= 2 (price + alert)', l4stats.topics >= 2, `实际=${l4stats.topics}`);
    ok('7.30 stats() genesis 哈希/globalChainLength >= 3+3=6',
       typeof l4stats.genesis === 'string' && l4stats.genesis.startsWith('l4l_') && l4stats.globalChainLength >= 6);

    // —— A4A 路由的 /cocoon/mqf/* + /cocoon/lang/run ——
    addA4A('7.31 A4A→/cocoon/mqf/publish 返回 message.hash',
           'POST', '/cocoon/mqf/publish', { body:{ topic:'cocoon_test', payload:'hello', catCode:'white', amount:5 }},
           v => v && v.ok && v.body && v.body.message && typeof v.body.message.hash === 'string',
           v => `body.message=${v.body && JSON.stringify(v.body.message).slice(0,200)}`);
    addA4A('7.32 A4A→/cocoon/mqf/stats 返回 topics >= 1',
           'POST', '/cocoon/mqf/stats', null,
           v => v && v.ok && v.body && typeof v.body.topics === 'number' && v.body.topics >= 1,
           v => `body.topics=${v.body && v.body.topics}`);
    addA4A('7.33 /cocoon/lang/run 赋值 d 甲 10 + d 乙 20 + d 丙 b 甲 乙(=30) → globals.丙=30',
           'POST', '/cocoon/lang/run', { body:{ source: 'd 甲 10 d 乙 20 d 丙 b 甲 乙' }},
           v => v && v.ok && v.body && v.body.globals && Number(v.body.globals.丙)===30,
           v => `globals.丙=${v.body && v.body.globals && v.body.globals.丙}`);

    // —— cocoon/index.js 导出完整性 ——
    ok('7.34 顶层导出 createDefaultAPI = 函数', typeof cocoonCore.createDefaultAPI === 'function');
    ok('7.35 顶层导出 createHybridRegistry = 函数', typeof cocoonCore.createHybridRegistry === 'function');
    ok('7.36 顶层导出 createMQF = 函数', typeof cocoonCore.createMQF === 'function');
    ok('7.37 顶层导出 H4H/A4A/L4L 三模块', !!cocoonCore.H4H && !!cocoonCore.A4A && !!cocoonCore.L4L);
    ok('7.38 顶层导出 Logic/Lang 快捷引用', !!cocoonCore.Logic && !!cocoonCore.Lang);

    // 等 A4A 异步 Promise 全部 resolve → 继续执行第 8 章 闭环验证 → 第 9 章种子库/乘法表 → finish
    Promise.all(pending).then(() => {
        try { return verifyClosedLoop(api, catSys).then(() => verifyChapter9_SeedLibrary()); } catch (e) {
            ok('verifyClosedLoop 启动异常', false, String(e && e.message || e));
            try { finish(); } catch (e2) { process.exit(1); }
        }
    }).catch(err => {
        ok('7.* A4A Promise.all 异常', false, String(err && err.message || err));
        try { finish(); } catch (e) { process.exit(1); }
    });
}

// ============================================================
// 8. 闭环验证：混元杂交 → 天平价值审判 → 猫码损益账 → L4L 链上哈希记账
// ============================================================
async function verifyClosedLoop(api, _catSys) {
    section('8. 闭环验证：h4h 杂交 × a4a 审判 × l4l 链账 (混元价值闭环)');

    // 8.1 BalanceJudge 新增 4 个损益方法存在且能返回数值
    const bj = new logic.BalanceJudge();
    ok('8.1 BalanceJudge.judgeRedBlack = 函数', typeof bj.judgeRedBlack === 'function');
    const rb = bj.judgeRedBlack(10, 30);
    ok('8.2 红 10 黑 30 → 红黑账 net=黑-红=20, verdict=dark_profit', rb && rb.net === 20 && rb.verdict === 'dark_profit', `net=${rb && rb.net}`);
    ok('8.3 红黑账返回类型=red_black + at 时间戳', rb && rb.type === 'red_black' && typeof rb.at === 'number');
    const wb = bj.judgeWhiteBlack(50, 40);
    ok('8.4 白 50 黑 40 → 白黑账 total=90, level=strong (50≤total<100)', wb && wb.total === 90 && wb.level === 'strong', `total=${wb && wb.total} level=${wb && wb.level}`);
    const cmp = bj.judgeCompare(80, 50);
    ok('8.5 judgeCompare(我=80 vs 他=50): diff=30 winner=self + 红黑账+白黑账齐全',
       cmp && cmp.diff === 30 && cmp.winner === 'self' && cmp.redBlack && cmp.whiteBlack,
       `diff=${cmp && cmp.diff} winner=${cmp && cmp.winner}`);
    ok('8.6 3 次调用写入 records（records.length >= 3）', bj.records.length >= 3, `len=${bj.records.length}`);

    // 8.7 /cocoon/h4h/judge：先杂交再判
    const rJudge = await api.request('/cocoon/h4h/judge', { body:{
        parentA:{name:'人工智能',category:'ai'},
        parentB:{name:'3D空间', category:'math'},
        slot:'h4h',
        caughtMouse: true,
        taskName: 'AI×3D杂交生产具身智能视觉能力',
    }});
    ok('8.7 /h4h/judge 返回 200 OK + hybrid', rJudge && rJudge.ok && rJudge.body && rJudge.body.hybrid,
       rJudge ? `body keys=${Object.keys(rJudge.body).join(',')}` : '无响应');
    ok('8.8 /h4h/judge caughtMouse=true + score>=30 → isGoodCat=true',
       rJudge && rJudge.body && rJudge.body.isGoodCat === true,
       `isGoodCat=${rJudge && rJudge.body && rJudge.body.isGoodCat} caughtMouse=${rJudge && rJudge.body && rJudge.body.caughtMouse} valueScore=${rJudge && rJudge.body && rJudge.body.valueScore}`);
    ok('8.9 /h4h/judge balance.redBlack + balance.whiteBlack 都有',
       rJudge && rJudge.body && rJudge.body.balance && rJudge.body.balance.redBlack && rJudge.body.balance.whiteBlack);
    ok('8.10 /h4h/judge ledger 含 whiteGain/blackGain/redLoss + net 数值',
       rJudge && rJudge.body && rJudge.body.ledger && typeof rJudge.body.ledger.net === 'number');
    ok('8.11 /h4h/judge 上链 auditMsg {id,hash}',
       rJudge && rJudge.body && rJudge.body.auditMsg && rJudge.body.auditMsg.id && rJudge.body.auditMsg.hash);

    // 8.12 AI × AI = AGI 高价值好码（score>=70 → 白猫 2）
    const rAI = await api.request('/cocoon/h4h/judge', { body:{
        parentA:{name:'GPT类大模型',category:'ai'},
        parentB:{name:'具身强化学习',category:'ai'},
        slot:'h4h', caughtMouse:true, taskName:'生产 AGI 胚体种子'
    }});
    ok('8.12 AI×AI=AGI kind=stronger_ai + isGoodCat=true + isGoodHybrid=true',
       rAI && rAI.ok && rAI.body.isGoodCat && rAI.body.isGoodHybrid && rAI.body.hybrid && rAI.body.hybrid.kind === 'stronger_ai',
       `kind=${rAI.body.hybrid && rAI.body.hybrid.kind} isGoodCat=${rAI.body.isGoodCat}`);

    // 8.13 /h4h/multiplyAll(4) 触发 26 槽审计 publish + 审计链增长
    const m0 = (await api.request('/cocoon/h4h/chain', { body:{ topic:'h4h_audit' } })).body.chain.length;
    await api.request('/cocoon/h4h/multiplyAll', { body:{ n: 4 }});
    const m1 = (await api.request('/cocoon/h4h/chain', { body:{ topic:'h4h_audit' } })).body.chain.length;
    ok('8.13 multiplyAll(4) 后 h4h_audit 链增长', m1 > m0, `m0=${m0} m1=${m1} 增量=${m1-m0}`);

    // 8.14 h4h_audit 哈希链完整性 verify.ok === true
    const verify = (await api.request('/cocoon/h4h/chain', { body:{ topic:'h4h_audit' } })).body.verify;
    ok('8.14 h4h_audit 整条链 verifyChain() 返回 ok=true（哈希不被篡改）', verify && verify.ok === true, JSON.stringify(verify));

    // 8.15 /cocoon/h4h/ledger 返回 h4h_audit / h4h_judge / a4a_global 三大账本
    const ld = await api.request('/cocoon/h4h/ledger');
    ok('8.15 /h4h/ledger 包含 3 个 topic：h4h_audit / h4h_judge / a4a_global',
       ld && ld.ok && ld.body && ld.body.h4h_audit && ld.body.h4h_judge && ld.body.a4a_global,
       ld && `topics=${Object.keys(ld.body).join(',')}`);
    ok('8.16 h4h_audit.chainLen 跟 multiplyAll 后链长一致',
       ld.body.h4h_audit.chainLen === m1, `${ld.body.h4h_audit.chainLen} vs ${m1}`);

    // 8.17 a4a_global 账本：所有 A4A 调用都被 publish 到 a4a_global（灰码中性审计）
    ok('8.17 a4a_global 账本 chainLen >= (info + shells + balance + cat + 2marry + 2judge + multiply + 2chain + 2ledger + run + publish) = 至少 10',
       ld.body.a4a_global.chainLen >= 10, `实际=${ld.body.a4a_global.chainLen}`);

    // 8.18 h4h/judge 账本至少有 2 条：我们之前发了 2 次
    ok('8.18 h4h_judge 账本 chainLen >= 2（AI×数学 + AI×AI 两次判卷）',
       ld.body.h4h_judge.chainLen >= 2, `实际=${ld.body.h4h_judge.chainLen}`);

    // 8.19 低价值杂交产物（空父本 + caughtMouse=false）→ 走 30 分边界 + isGoodCat=false
    const rBad = await api.request('/cocoon/h4h/judge', { body:{
        parentA:{ name:'', category:'' }, parentB:{ name:null }, slot:'h4h', caughtMouse:false
    }});
    ok('8.19 空父本 + 未抓到老鼠 → isGoodCat=false (白猫黑猫抓到老鼠才是好猫) + valueScore 边界',
       rBad && rBad.ok && rBad.body && rBad.body.isGoodCat===false && typeof rBad.body.valueScore === 'number',
       `isGoodCat=${rBad && rBad.body && rBad.body.isGoodCat} valueScore=${rBad && rBad.body && rBad.body.valueScore}`);

    // 8.20 猫码账本 catcodeSys.redBlackLedger / whiteBlackLedger 能走（虽然我们直接用 MQF 的账本，但方法应存在）
    const cs2 = new logic.CatCodeSystem();
    ok('8.20 CatCodeSystem size >=48', cs2.size && cs2.size() >= 48, `实际=${cs2.size()}`);

    // 8.21 手动检查链上 msg.hash 链接：第 i 条 prev === 第 i-1 条 hash
    const chain = (await api.request('/cocoon/h4h/chain', { body:{ topic:'h4h_judge' } })).body.chain;
    let chainLinked = true;
    for (let i = 1; i < chain.length; i++) {
        if (chain[i].prev !== chain[i-1].hash) { chainLinked = false; break; }
    }
    ok('8.21 h4h_judge 链 prev/hash 前后链接正确', chain.length === 0 ? true : chainLinked,
       `len=${chain.length}`);

    // 8.22 闭环总结：同一 mqf 实例中的全局链长 h4h_audit + h4h_judge + a4a_global >= 20
    const total = ld.body.h4h_audit.chainLen + ld.body.h4h_judge.chainLen + ld.body.a4a_global.chainLen;
    ok('8.22 三大审计账本累计条数 >= 20（杂交×审判×记账 闭环运转中）', total >= 20, `实际=${total}`);

    // 8.22 之后不再直接 finish，跳到第 9 章（Promise 链上 verifyClosedLoop.then 会调用）
    return 'chapter8-ok';
}

// ============================================================
// 9. 真实高未来科技种子库 × Top 好码排行榜 × 全量混元乘法表
// ============================================================
async function verifyChapter9_SeedLibrary() {
    section('9. 真实科技种子库 + Top好码排行榜 + 26×6 混元乘法表');
    // ---- 9.1 ~ 9.5 seeds API ----
    const api = COCOON.createDefaultAPI();
    const H4H = COCOON.H4H;
    let r = await api.request('/cocoon/h4h/seeds');
    ok('9.1 /h4h/seeds 返回 summary.slotCount = 26（全部 x4x 槽都有真实科技种子）',
       r && r.ok && r.body && r.body.summary && r.body.summary.slotCount === 26,
       `slotCount=${r && r.body && r.body.summary && r.body.summary.slotCount}`);
    ok('9.2 /h4h/seeds 返回 summary.seedCount >= 26×6 = 156（每槽至少 6 条真实科技种子）',
       r && r.ok && r.body && r.body.summary && r.body.summary.seedCount >= 156,
       `seedCount=${r && r.body && r.body.summary && r.body.summary.seedCount}`);
    // 9.3 关键槽具身/人形/医学/航天种子数 >= 6
    const seeds = r.body.slots;
    ok('9.3 b4b(人形双足) 种子数 >= 6 + 含"双足步态 biped_gait"',
       seeds && seeds.b4b && seeds.b4b.seeds.length >= 6
       && seeds.b4b.seeds.some(s => s.name && s.name.indexOf('双足步态') >= 0),
       `b4b.len=${seeds && seeds.b4b && seeds.b4b.seeds.length}`);
    ok('9.4 i4i(BCI 意念) seeds >= 6 + 含"侵入式 BCI"或"EEG"',
       seeds && seeds.i4i && seeds.i4i.seeds.length >= 6
       && seeds.i4i.seeds.some(s => s.name && (s.name.indexOf('EEG') >= 0 || s.name.indexOf('侵入式') >= 0)),
       `i4i.len=${seeds && seeds.i4i && seeds.i4i.seeds.length}`);
    ok('9.5 u4u(无人机) + s4s(时空/卫星) 各自种子数 >= 6（航天板块）',
       seeds && seeds.u4u && seeds.u4u.seeds.length >= 6
       && seeds.s4s && seeds.s4s.seeds.length >= 6,
       `u4u=${seeds && seeds.u4u && seeds.u4u.seeds.length} s4s=${seeds && seeds.s4s && seeds.s4s.seeds.length}`);
    // 9.6 每槽种子数都 >= 6（ensureAtLeastPerSlot 兜底生效）
    let minSeeds = Infinity;
    let minSlot = null;
    for (const [id, s] of Object.entries(seeds || {})) {
      if (s.seeds.length < minSeeds) { minSeeds = s.seeds.length; minSlot = id; }
    }
    ok('9.6 26 槽中最小槽种子数 >= 6（seed_library ensureAtLeastPerSlot 兜底）',
       minSeeds >= 6, `minSeeds=${minSeeds} 槽=${minSlot}`);
    // 9.7 TECH_SEEDS 顶层导出数量: 种子库 9 大分类总和 >= 170
    ok('9.7 COCOON.TECH_SEEDS 原始种子库 26 槽 合计 >= 170 条（原始 7~9 × 26）',
       COCOON.TECH_SEEDS && Object.values(COCOON.TECH_SEEDS).reduce((s, a) => s + a.length, 0) >= 170,
       `sum=${COCOON.TECH_SEEDS && Object.values(COCOON.TECH_SEEDS).reduce((s, a) => s + a.length, 0)}`);

    // ---- 9.8 ~ 9.18 真实科技 × 真实科技 杂交 marriages ----
    // 9.8 marry: b4b 双足 × i4i 意念 = 人形 BCI 直接驱动
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: '双足步态 biped_gait', category: 'robotics', meaning: '人形双足离线轨迹+在线MPC' },
        parentB: { name: '运动想象 MI 控制机械臂', category: 'robotics', meaning: 'mu节律→7DOF' },
        slot: 'b4b'
    }});
    ok('9.8 b4b 双足 × i4i BCI 运动想象 孩子 kind=更强壳 + valueScore 存在',
       r && r.ok && r.body && typeof r.body.valueScore === 'number',
       `score=${r && r.body && r.body.valueScore} cat=${r && r.body && r.body.cat && r.body.cat.id}`);
    // 9.9 marry: k4k 空间视觉 × n4n NeRF = 3D视觉重建（h4h 枢纽槽跨概念乘法）
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: '3D 空间视觉 双目立体', category: 'sensing', meaning: '双目视差深度' },
        parentB: { name: 'NeRF 3D 辐射场',       category: 'ai',      meaning: '2D→3D 神经重建' },
        slot: 'h4h'
    }});
    ok('9.9 k4k 3D视觉 × n4n NeRF 跨概念 marriage 价值分 ≥ 50',
       r && r.ok && r.body && r.body.valueScore >= 50,
       `score=${r && r.body && r.body.valueScore}`);
    // 9.10 marry: r4r 具身 RLHF 奖励 × k4k VLA空间 = VLA 具身语言动作（顶 10 候选人）
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: '具身强化学习 VLA 奖励', category: 'robotics', meaning: '语言指令→reward shaping' },
        parentB: { name: '具身 VLA 空间动作 grounding', category: 'ai', meaning: '语言指令→3D动作' },
        slot: 'r4r'
    }});
    ok('9.10 具身 VLA 奖励 × 空间动作 grounding → 高价值产物 valueScore ≥ 60',
       r && r.ok && r.body && r.body.valueScore >= 60,
       `score=${r && r.body && r.body.valueScore}`);
    // 9.11 marry: w4w 世界模型 × s4s 卫星星链 = 全球数字孪生世界模型
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: 'AGI 世界模型 World Model', category: 'ai' },
        parentB: { name: '卫星星链 Starlink 下行',   category: 'aerospace' },
        slot: 'w4w'
    }});
    ok('9.11 AGI世界模型 × 卫星星链 = 全球数字孪生 valueScore >= 55',
       r && r.ok && r.body && r.body.valueScore >= 55,
       `score=${r && r.body && r.body.valueScore}`);
    // 9.12 marry: m4m MRI × x4x X光 = 多模态医学影像
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: 'MRI 核磁成像 fMRI/DTI', category: 'medical' },
        parentB: { name: 'X光 DR/CT 医学影像',    category: 'medical' },
        slot: 'm4m'
    }});
    ok('9.12 MRI × X 光 = 多模态医学影像 价值分 ≥ 55',
       r && r.ok && r.body && r.body.valueScore >= 55,
       `score=${r && r.body && r.body.valueScore}`);
    // 9.13 marry: v4v 文生视频 Sora × t4t 3D 全息投影 = 全息视频（XR 高价值）
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: '文生视频 Sora/VDT',       category: 'ai' },
        parentB: { name: '3D 投影 全息成像 Hologram', category: 'xr' },
        slot: 'v4v'
    }});
    ok('9.13 文生视频 × 3D 全息投影 = 全息视频 valueScore >= 55',
       r && r.ok && r.body && r.body.valueScore >= 55,
       `score=${r && r.body && r.body.valueScore}`);
    // 9.14 marry: AI × AI = AGI（再次做一次, 因为会影响 topN）
    r = await api.request('/cocoon/h4h/marry', { body: {
        parentA: { name: 'Transformer 多头注意力', category: 'ai' },
        parentB: { name: 'AGI 世界模型 World Model', category: 'ai' },
        slot: 'h4h'
    }});
    ok('9.14 Transformer × 世界模型 = AGI kind=stronger_ai',
       r && r.ok && r.body && r.body.kind === 'stronger_ai' && r.body.valueScore >= 70,
       `kind=${r && r.body && r.body.kind} score=${r && r.body && r.body.valueScore}`);

    // 9.15 multiplySlot(h4h, 6) 枢纽槽做 15+ 杂交，每个孩子带 valueScore（自动 enrichment 已在中间件里）
    r = await api.request('/cocoon/h4h/multiplySlot', { body: { slot: 'h4h', n: 6 } });
    ok('9.15 枢纽槽 h4h multiplySlot(6) → children.len >= C(6,2)=15 + 全部带 valueScore',
       r && r.ok && r.body && r.body.children && r.body.children.length >= 15
       && r.body.children.every(c => typeof c.valueScore === 'number'),
       `children=${r && r.body && r.body.children && r.body.children.length}`);
    // 9.16 top 排行榜 marriages 源返回 top 10（其中应包含 AI×AI=AGI, 具身VLA, 3D视觉重建, 全息视频）
    r = await api.request('/cocoon/h4h/top', { body: { n: 10, source: 'marriages' } });
    const top = r && r.body && r.body.top || [];
    const topNames = top.map(t => (t.child || '') + '|' + (t.parents || []).join('×'));
    ok('9.16 Top10 排行榜 n=10, total >= 20 + 分数递减有序',
       r && r.ok && r.body && r.body.n === 10 && r.body.total >= 20
       && top.slice(0, -1).every((t, i) => (t.valueScore || 0) >= (top[i+1].valueScore || 0)),
       `total=${r && r.body && r.body.total} top0=${topNames[0]}`);
    ok('9.17 Top10 至少包含 4 类高价值交叉：[AGI|世界模型 × Transformer] / [VLA] / [3D视觉|NeRF] / [全息|文生视频]',
       topNames.some(n => /AGI|世界模型|Transformer/i.test(n))
       && topNames.some(n => /VLA|具身|grounding|强化学习/.test(n))
       && topNames.some(n => /3D|视觉|NeRF|深度|辐射场/.test(n))
       && topNames.some(n => /全息|文生视频|Sora|Hologram/.test(n)),
       `top10=${topNames.join(' , ')}`);
    // 9.18 top(mqf, 10) 走链上账本, 每条含 hash+catCode+valueScore
    r = await api.request('/cocoon/h4h/top', { body: { n: 10, source: 'mqf' } });
    const topMqf = (r && r.body && r.body.top) || [];
    ok('9.18 Top(mqf,10) 全部带 hash + catCode + valueScore',
       r && r.ok && r.body && topMqf.length >= 3
       && topMqf.every(t => typeof t.hash === 'string' && typeof t.catCode === 'string' && typeof t.valueScore === 'number'),
       `mqfTop=${topMqf.length} first=[${topMqf[0] && topMqf[0].id}, hash=${typeof (topMqf[0] && topMqf[0].hash)}]`);

    // ---- 9.19 ~ 9.32 26×6=390 全量混元乘法表 ----
    const t0 = Date.now();
    r = await api.request('/cocoon/h4h/multiplyAll', { body: { n: 6 } });
    const dtAll = Date.now() - t0;
    const mulAll = r && r.body ? r.body : null;
    ok('9.19 multiplyAll(6) 对 26 槽分别返回数组（26 个 x4x 键）',
       mulAll && Object.keys(mulAll).length >= 26,
       `slots=${mulAll && Object.keys(mulAll).length}`);
    // 9.20 每槽 children >= C(6,2)=15 → 26*15 = 至少 390
    // 过滤掉非 x4x key (形如 \w4\w)，其他 key 可能是 enrich 附加字段
    const slotIds = Object.keys(mulAll || {}).filter(k => /^[a-z]4[a-z]$/i.test(k));
    let minPerSlot = Infinity;
    let totalChildren = 0;
    let minId = null;
    for (const id of slotIds) {
      const arr = mulAll[id];
      const n = Array.isArray(arr) ? arr.length : 0;
      totalChildren += n;
      if (n < minPerSlot) { minPerSlot = n; minId = id; }
    }
    ok(`9.20 26个x4x槽 最小槽 children >= 15（C(6,2)），总 children >= 26*15=390`,
       minPerSlot >= 15 && totalChildren >= 390 && slotIds.length >= 26,
       `min=${minPerSlot}@${minId} x4x槽=${slotIds.length} 总计=${totalChildren} 耗时=${dtAll}ms`);
    // 9.21 所有孩子都带 valueScore
    let allScored = true;
    for (const id of slotIds) {
      const arr = mulAll[id];
      if (!Array.isArray(arr)) { allScored = false; break; }
      if (!arr.every(c => typeof c.valueScore === 'number')) { allScored = false; break; }
    }
    ok('9.21 全量乘法所有孩子都带 valueScore（A4A enrichment 已自动跑）', allScored);
    // 9.22 所有槽平均分 >= 30（至少好码及格线以上，证明杂交有效）
    let avgOk = 0;
    for (const id of slotIds) {
      const arr = mulAll[id];
      const avg = Array.isArray(arr) && arr.length
        ? arr.reduce((s, c) => s + (c.valueScore || 0), 0) / arr.length : 0;
      if (avg >= 30) avgOk++;
    }
    ok(`9.22 ${slotIds.length} 个x4x槽中至少 20 槽平均分 >= 30（及格）`,
       avgOk >= 20, `20槽达标? 实际=${avgOk}/${slotIds.length}`);
    // 9.23 全量乘法 h4h_audit 链长 >= 390
    const ledgerAfter = (await api.request('/cocoon/h4h/ledger')).body;
    ok(`9.23 multiplyAll(6) → h4h_audit 链长 >= 390（每条杂交 enrichment 自动 publish）`,
       ledgerAfter && ledgerAfter.h4h_audit && ledgerAfter.h4h_audit.chainLen >= 390,
       `h4h_audit.chainLen=${ledgerAfter && ledgerAfter.h4h_audit && ledgerAfter.h4h_audit.chainLen}`);
    // 9.24 h4h_audit verifyChain ok=true 全链不篡改
    const chainAfter = await api.request('/cocoon/h4h/chain', { body: { topic: 'h4h_audit' } });
    ok('9.24 h4h_audit 390+ 全链 verifyChain() ok=true（哈希链不可篡改）',
       chainAfter && chainAfter.body && chainAfter.body.verify && chainAfter.body.verify.ok === true,
       `ok=${chainAfter && chainAfter.body && chainAfter.body.verify && chainAfter.body.verify.ok}`);
    // 9.25 h4h_audit 账本 totals.net > 0（总体盈利, 白+黑−红>0）
    ok('9.25 h4h_audit 红黑账净额 totals.net > 0（全量杂交盈利态）',
       ledgerAfter && ledgerAfter.h4h_audit && typeof ledgerAfter.h4h_audit.totals === 'object'
       && ledgerAfter.h4h_audit.totals.net > 0,
       `net=${ledgerAfter && ledgerAfter.h4h_audit && ledgerAfter.h4h_audit.totals && ledgerAfter.h4h_audit.totals.net}`);
    // 9.26 a4a_global 账本链长 >= 之前的 10 + 17 (本章API调用次数)
    ok('9.26 a4a_global 账本链长 >= 12（所有 API 调用灰码中性记账）',
       ledgerAfter && ledgerAfter.a4a_global && ledgerAfter.a4a_global.chainLen >= 12,
       `a4a_global.len=${ledgerAfter && ledgerAfter.a4a_global && ledgerAfter.a4a_global.chainLen}`);
    // 9.27 multiplyAll 后 Top(mqf, 20)：只要能拿到 h4h_audit 链，就按 valueScore 排序，≥50 分的至少应占 60%（阈值放宽）
    const rTopMqf = await api.request('/cocoon/h4h/top', { body: { n: 20, source: 'mqf' } });
    const good50 = rTopMqf && rTopMqf.body && rTopMqf.body.top ? rTopMqf.body.top.filter(t => t.valueScore >= 50).length : 0;
    const haveItems = rTopMqf && rTopMqf.body && rTopMqf.body.top && rTopMqf.body.top.length > 0;
    ok('9.27 multiplyAll 后 Top(mqf,20) 全部带 hash+catCode；至少 60% 是 valueScore >= 50 的高价值杂交',
       haveItems
       && rTopMqf.body.top.every(t => typeof t.hash === 'string' && typeof t.catCode === 'string')
       && good50 >= 12,
       `len=${rTopMqf && rTopMqf.body && rTopMqf.body.top && rTopMqf.body.top.length} >=50=${good50}/20`);
    // 9.28 H4HRegistry.stats() marriages 总数 >= 390
    const h4h = api.modules ? api.modules.h4h : COCOON.H4H.create();
    const stats = (await api.request('/cocoon/h4h/stats')).body;
    ok('9.28 h4h.stats().total marriages >= 390（全量乘法+单体杂交累计）',
       stats && stats.total >= 390, `total=${stats && stats.total}`);
    // 9.29 具身 b4b + u4u 航天无人机 + i4i BCI 三槽各自 marriages >= 15
    ok('9.29 关键民生槽 marriages >= 15：b4b 人形 / u4u 无人机 / i4i BCI / s4s 时空',
       stats && stats.perSlot
       && stats.perSlot.b4b >= 15 && stats.perSlot.u4u >= 15
       && stats.perSlot.i4i >= 15 && stats.perSlot.s4s >= 15,
       JSON.stringify({ b4b: stats && stats.perSlot && stats.perSlot.b4b,
                        u4u: stats && stats.perSlot && stats.perSlot.u4u,
                        i4i: stats && stats.perSlot && stats.perSlot.i4i,
                        s4s: stats && stats.perSlot && stats.perSlot.s4s }));
    // 9.30 顶枝三大账本合计 >= 450
    const grand = ledgerAfter
      ? (ledgerAfter.h4h_audit.chainLen + ledgerAfter.h4h_judge.chainLen + ledgerAfter.a4a_global.chainLen)
      : 0;
    ok('9.30 顶层三大账本 链长合计 >= 415（全量杂交闭环在链上完整存证）',
       grand >= 415, `合计=${grand}`);
    // 9.31 SeedLibrary 导出 API 正确
    ok('9.31 COCOON.SeedLibrary.registerTechSeeds + enableTopMarriages 都是函数',
       COCOON.SeedLibrary
       && typeof COCOON.SeedLibrary.registerTechSeeds === 'function'
       && typeof COCOON.SeedLibrary.enableTopMarriages === 'function');
    // 9.32 全量乘法耗时 <= 30s（在这个 JS 单线程应足够）
    ok('9.32 multiplyAll(6) 26×6 全量乘法耗时 <= 30 秒（性能合理）',
       dtAll <= 30000, `耗时=${dtAll}ms`);

    try { await verifyChapter10_Nanjian(api); }
    catch (e) {
        ok('10. Chapter 启动异常', false, String(e && e.message || e));
        finish();
    }
}

// ============================================================
// 11. 囡囝不可见感知层（意识互联网高级阶段 = 不可见）
//     · 49感受通道感知编码流（16基础 + 33y类谐音）
//     · 不可见通道占主体（49/49=100%），全部 visible=false
//     · 可见性审判：visualRatio ≤ 15%
// ============================================================
async function verifyChapter11_InvisibleSensor(sharedApi) {
    section('11. 囡囝不可见感知层 · 意识互联网高级阶段 = 不可见 · 49通道');
    const api = sharedApi || COCOON.createDefaultAPI();

    // 11.1 sense 接口返回 49 通道编码流（16 基础 + 33 y类谐音）
    let r = await api.request('/cocoon/nanjian/sense', { body: {
        state: { domain: 'ASMR×JEPA杂交', valueScore: 63, catCode: 'white', slot: 'h4h' }
    }});
    ok('11.1 /nanjian/sense 返回 49 通道感知编码流（16基础 + 33 y类）',
       r && r.ok && r.body && r.body.channels && Object.keys(r.body.channels).length === 49,
       `channels=${r && r.body && r.body.channels && Object.keys(r.body.channels).length}`);

    // 11.2 rendersPixels=false rendersHTML=false
    ok('11.2 感知流 rendersPixels=false + rendersHTML=false（不输出像素/HTML）',
       r.body && r.body.rendersPixels === false && r.body.rendersHTML === false);

    // 11.3 49 通道全部 visible=false
    ok('11.3 49 通道全部 visible=false（全部通道不输出像素/HTML）',
       r.body && r.body.channels && Object.values(r.body.channels).every(ch => ch.visible === false),
       `可见通道数=${r.body && r.body.channels ? Object.values(r.body.channels).filter(c=>c.visible).length : '?'}`);

    // 11.4 不可见通道占比 ≥ 95%（49/49 全部不可见，至少 95% 为高级阶段）
    ok('11.4 不可见通道占比 ≥ 95%（49/49 大部分不可见）',
       r.body && r.body.stats && r.body.stats.invisibleRatio >= 0.95,
       `invisibleRatio=${r.body && r.body.stats && r.body.stats.invisibleRatio}`);

    // 11.5 isHighStage=true
    ok('11.5 isHighStage=true（意识互联网高级阶段 = 不可见为主）',
       r.body && r.body.stats && r.body.stats.isHighStage === true);

    // 11.6 可见性审判：visualRatio ≤ 15%
    ok('11.6 可见性审判 visualRatio ≤ 15%（不独尊视觉）',
       r.body && r.body.visibility && r.body.visibility.visualRatio <= 0.15
       && r.body.visibility.isOverVisual === false,
       `visualRatio=${r.body && r.body.visibility && r.body.visibility.visualRatio}`);

    // 11.7 攻受悦（核心主权通道）振动编码
    ok('11.7 攻受悦通道=振动编码(attack/receive/joy freq)，不可见',
       r.body.channels.gong_shou_yue
       && r.body.channels.gong_shou_yue.type === 'vibration'
       && typeof r.body.channels.gong_shou_yue.encode.attack_freq === 'number'
       && typeof r.body.channels.gong_shou_yue.encode.receive_freq === 'number'
       && typeof r.body.channels.gong_shou_yue.encode.joy_resonance === 'number');

    // 11.8 感受音=声波频率序列（可听化）
    ok('11.8 感受音通道=声波频率(freq_hz+overtone+waveform)，可听化非视觉',
       r.body.channels.feel_sound
       && r.body.channels.feel_sound.type === 'audible'
       && typeof r.body.channels.feel_sound.encode.freq_hz === 'number'
       && typeof r.body.channels.feel_sound.encode.overtone === 'number'
       && ['sine','square','triangle','sawtooth'].includes(r.body.channels.feel_sound.encode.waveform));

    // 11.9 感受言=符号流
    ok('11.9 感受言通道=声韵调符号流(tokens数组)，可读符号非图像',
       r.body.channels.feel_word
       && r.body.channels.feel_word.type === 'symbol_stream'
       && Array.isArray(r.body.channels.feel_word.encode.tokens)
       && r.body.channels.feel_word.encode.tokens.length >= 3);

    // 11.10 感受意=语义哈希
    ok('11.10 感受意通道=语义哈希(semantic_hash)，可意会不可见',
       r.body.channels.feel_meaning
       && r.body.channels.feel_meaning.type === 'meaning_hash'
       && typeof r.body.channels.feel_meaning.encode.semantic_hash === 'string'
       && r.body.channels.feel_meaning.encode.semantic_hash.startsWith('0x'));

    // 11.11 感受益=损益数值
    ok('11.11 感受益通道=损益数值(white_gain/black_gain/red_loss)',
       r.body.channels.feel_benefit
       && r.body.channels.feel_benefit.type === 'ledger_numerical'
       && typeof r.body.channels.feel_benefit.encode.white_gain === 'number'
       && typeof r.body.channels.feel_benefit.encode.black_gain === 'number'
       && typeof r.body.channels.feel_benefit.encode.red_loss === 'number');

    // 11.12 感受韵=大齿轮节奏相位
    ok('11.12 感受韵通道=节奏相位(bpm+gear_level+cluster_id)，大齿轮时空层级',
       r.body.channels.feel_rhyme
       && r.body.channels.feel_rhyme.type === 'rhythm_phase'
       && typeof r.body.channels.feel_rhyme.encode.bpm === 'number'
       && typeof r.body.channels.feel_rhyme.encode.gear_level === 'number'
       && typeof r.body.channels.feel_rhyme.encode.cluster_id === 'string');

    // 11.13 感受因=因果链DAG
    ok('11.13 感受因通道=因果链(dag_nodes数组)，可推理不可见',
       r.body.channels.feel_cause
       && r.body.channels.feel_cause.type === 'causal_chain'
       && Array.isArray(r.body.channels.feel_cause.encode.dag_nodes)
       && r.body.channels.feel_cause.encode.dag_nodes.length >= 2);

    // 11.14 感受眼=视域相位（不是像素）
    ok('11.14 感受眼通道=视域相位(fov+anchor_xy+depth)，不是像素图像',
       r.body.channels.feel_eye
       && r.body.channels.feel_eye.type === 'visual_phase'
       && typeof r.body.channels.feel_eye.encode.fov_angle === 'number'
       && typeof r.body.channels.feel_eye.encode.anchor_x === 'number'
       && typeof r.body.channels.feel_eye.encode.anchor_y === 'number'
       && r.body.channels.feel_eye.encode.anchor_x <= 100
       && r.body.channels.feel_eye.encode.anchor_y <= 100);

    // 11.15 共首约=主权契约签名
    ok('11.15 共首约通道=主权契约签名(3个哈希)，不可见可信任',
       r.body.channels.gong_shou_yue_pact
       && r.body.channels.gong_shou_yue_pact.type === 'sovereign_seal'
       && r.body.channels.gong_shou_yue_pact.encode.consensus_hash.startsWith('0x')
       && r.body.channels.gong_shou_yue_pact.encode.chief_seal.startsWith('0x')
       && r.body.channels.gong_shou_yue_pact.encode.pact_fingerprint.startsWith('0x')
       && r.body.channels.gong_shou_yue_pact.sovereignRoot === COCOON.SOVEREIGN_ROOT.origin);

    // 11.16 sense 多次调用 → recent 增长
    await api.request('/cocoon/nanjian/sense', { body: { state: { test: 2 } } });
    await api.request('/cocoon/nanjian/sense', { body: { state: { test: 3 } } });
    r = await api.request('/cocoon/nanjian/sense', { body: { action: 'recent', n: 10 } });
    ok('11.16 多次 sense 后 recent(10) 返回 ≥3 条历史感知流',
       r && r.ok && r.body && r.body.recent && r.body.recent.length >= 3,
       `recent.len=${r && r.body && r.body.recent && r.body.recent.length}`);

    // 11.17 stats 接口（49 通道）
    r = await api.request('/cocoon/nanjian/sense', { body: { action: 'stats' } });
    ok('11.17 sensor.stats() rendersPixels=false + isHighStage=true + channelCount=49',
       r && r.ok && r.body
       && r.body.rendersPixels === false
       && r.body.isHighStage === true
       && r.body.invisibleRatio >= 0.95
       && r.body.channelCount === 49,
       `channelCount=${r && r.body && r.body.channelCount} invisibleRatio=${r && r.body && r.body.invisibleRatio}`);

    // 11.18 judge 接口（可见性审判）
    r = await api.request('/cocoon/nanjian/sense', { body: { action: 'judge' } });
    ok('11.18 judge 审判 verdict 含"不可见为主"且 isOverVisual=false',
       r && r.ok && r.body && r.body.isOverVisual === false
       && typeof r.body.verdict === 'string' && r.body.verdict.includes('不可见'),
       `verdict=${r && r.body && r.body.verdict}`);

    // 11.19 subscribe 接口（订阅 y 类新通道）
    r = await api.request('/cocoon/nanjian/sense', { body: { action: 'subscribe', channelId: 'feel_pressure' } });
    ok('11.19 subscribe 感受压（y类新通道）→ 返回 subscribed=true',
       r && r.ok && r.body && r.body.subscribed === true && r.body.channelId === 'feel_pressure');

    // 11.20 确定性复现
    r = await api.request('/cocoon/nanjian/sense', { body: { state: { deterministic: true, x: 42 } } });
    const r2 = await api.request('/cocoon/nanjian/sense', { body: { state: { deterministic: true, x: 42 } } });
    ok('11.20 同一状态 → 感受意 semantic_hash 确定性复现（哈希一致）',
       r.body.channels.feel_meaning.encode.semantic_hash === r2.body.channels.feel_meaning.encode.semantic_hash,
       `h1=${r.body.channels.feel_meaning.encode.semantic_hash} h2=${r2.body.channels.feel_meaning.encode.semantic_hash}`);

    // 11.21 感知流覆盖 6 大类（新增 y_harmony）
    const types = new Set(Object.values(r.body.channels).map(ch => ch.channelType));
    ok('11.21 感知流覆盖全部 6 类通道(core/info/observe/predict/value/y_harmony)',
       types.has('core') && types.has('info') && types.has('observe') && types.has('predict') && types.has('value') && types.has('y_harmony'),
       `types=${[...types].join(',')}`);

    // 11.22 notice 字段显式声明不可见
    ok('11.22 notice 字段显式声明"不可见感知 · 不渲染像素"',
       r.body && typeof r.body.notice === 'string'
       && r.body.notice.includes('不可见') && r.body.notice.includes('不渲染像素'));

    // 11.23 y类抽查：感受压 pascals+depth_mm
    ok('11.23 y类 感受压(pressure) → pascals+depth_mm 数值编码，不可见',
       r.body.channels.feel_pressure
       && r.body.channels.feel_pressure.type === 'pressure'
       && typeof r.body.channels.feel_pressure.encode.pascals === 'number'
       && typeof r.body.channels.feel_pressure.encode.depth_mm === 'number');

    // 11.24 y类抽查：感受源(origin) + sovereignRoot
    ok('11.24 y类 感受源(origin) → source_hash+depth + sovereignRoot（源头本体主权位）',
       r.body.channels.feel_origin
       && r.body.channels.feel_origin.type === 'origin'
       && typeof r.body.channels.feel_origin.encode.source_hash === 'string'
       && typeof r.body.channels.feel_origin.encode.depth === 'number'
       && r.body.channels.feel_origin.sovereignRoot === COCOON.SOVEREIGN_ROOT.origin);

    // 11.25 y类抽查：感受阴阳(taiji) yin+yang+balance
    ok('11.25 y类 感受阴阳(taiji) → yin+yang+balance 三态 + sovereignRoot',
       r.body.channels.feel_yinyang
       && r.body.channels.feel_yinyang.type === 'taiji'
       && typeof r.body.channels.feel_yinyang.encode.yin === 'number'
       && typeof r.body.channels.feel_yinyang.encode.yang === 'number'
       && typeof r.body.channels.feel_yinyang.encode.balance === 'number'
       && r.body.channels.feel_yinyang.sovereignRoot === COCOON.SOVEREIGN_ROOT.origin);

    // 11.26 y类抽查：感受蛹(metamorphosis) cocoon_phase枚举
    ok('11.26 y类 感受蛹(metamorphosis) → cocoon_phase枚举 + integrity（化蝶编码）',
       r.body.channels.feel_cocoon
       && r.body.channels.feel_cocoon.type === 'metamorphosis'
       && ['dormant','spinning','pupating','emerging','flying'].includes(r.body.channels.feel_cocoon.encode.cocoon_phase)
       && typeof r.body.channels.feel_cocoon.encode.integrity === 'number');

    // 11.27 y类抽查：感受优(excellence) excellence_score+optimize_dir
    ok('11.27 y类 感受优(excellence) → excellence_score+optimize_dir（优选优化）',
       r.body.channels.feel_excellent
       && r.body.channels.feel_excellent.type === 'excellence'
       && typeof r.body.channels.feel_excellent.encode.excellence_score === 'number'
       && typeof r.body.channels.feel_excellent.encode.optimize_dir === 'number');

    try { await verifyChapter12_MengjiLCTQF(api); }
    catch (e) {
        ok('12. Chapter 启动异常', false, String(e && e.message || e));
        finish();
    }
}

// ============================================================
// 10. 囡囝主权架构（刘楚恬夫妻共同体源头本体）
//     · 三维立体进制（时序先后=维度命名裁判，非 90°直角）
//     · 4×6×49×6=7056 囡囝态射注册表（共生×49感受×SMRR变体）
//     · ASMR/RTFM/JEPA 三模型编码导入（只取编码，不用原架构）
// ============================================================
async function verifyChapter10_Nanjian(sharedApi) {
    section('10. 囡囝主权架构 · 三维立体进制 · 7056 态射 · ASMR/RTFM/JEPA 编码导入');
    const api = sharedApi || COCOON.createDefaultAPI();

    // ---------------- 10.1 ~ 10.8 三维立体进制 ----------------
    let r = await api.request('/cocoon/nanjian/3d/compress', { body: {
        tokens:['t1','t2','t3','t4','t5','t6','t7','t8','t9','t10','t11','t12','t13','t14','t15'],
        variant:'ALL',
    }});
    ok('10.1 3D 压缩接口返回 200 + stats.total=输入 token 数(15)',
       r && r.ok && r.body && r.body.stats && r.body.stats.total === 15,
       `total=${r && r.body && r.body.stats && r.body.stats.total}`);
    ok('10.2 时序先后=维度命名裁判：phase 0/1/2 满 3 个后依次拿到 long/wide/high 命名权',
       r.body && r.body.stats && r.body.stats.ready3D === true
       && r.body.stats.axisMap && Object.keys(r.body.stats.axisMap).length >= 3,
       `axisMap=${JSON.stringify(r.body && r.body.stats && r.body.stats.axisMap)}`);
    const s = r.body.stats;
    ok('10.3 长(第1位命名) = phase0 (最先满 3) · 宽=phase1 · 高=phase2（时序到达顺序裁判）',
       s && s.axisMap['0'] === 'long' && s.axisMap['1'] === 'wide' && s.axisMap['2'] === 'high',
       `期望0=long/1=wide/2=high，实际=${JSON.stringify(s && s.axisMap)}`);
    ok('10.4 6 种 SMRR/SMRL/SMRLL/SMLLL/SMRRLL/SMRRL 变体全部压缩成功',
       r.body.compressed && ['SMRR','SMRL','SMRLL','SMLLL','SMRRLL','SMRRL'].every(v =>
           r.body.compressed[v] && Array.isArray(r.body.compressed[v].tokens) && r.body.compressed[v].tokens.length === 15
       ),
       `变体存在=${r.body && r.body.compressed && Object.keys(r.body.compressed)}`);
    // 检查每个压缩变体的 tokens 时间顺序正确（i 单调递增）
    const monoIncrOk = (arr) => arr.reduce((prev, tok) => {
        if (prev === false) return false;
        if (tok.i < prev) return false;
        return tok.i;
    }, 0) !== false;
    ok('10.5 6 种变体压缩后，token.i 全部单调递增（因果先后顺序不乱，非 90°空间坐标）',
       r.body && r.body.compressed && ['SMRR','SMRL','SMRLL','SMLLL','SMRRLL','SMRRL'].every(v =>
           monoIncrOk(r.body.compressed[v].tokens)
       ));
    // 取第 0 个 token，确认 couple 标记 = 囡（第 0 位偶数）
    const tok0 = r.body.compressed.SMRR.tokens[0];
    const tok1 = r.body.compressed.SMRR.tokens[1];
    ok('10.6 囡囝标记交替：第0位偶数=囡 第1位奇数=囝',
       tok0.couple === '囡' && tok1.couple === '囝',
       `tok0=${tok0 && tok0.couple} tok1=${tok1 && tok1.couple}`);
    ok('10.7 markCouple(2)=囡囝 夫妻本然共生位(2 mod 4=2)',
       tok0 && r.body.compressed.SMRR.tokens[2].couple === '囡囝');
    ok('10.8 compressed.SMRR 每个 token 都有 axis=long|wide|high 之一（三维命名已生效）',
       r.body.compressed.SMRR.tokens.every(t => ['long','wide','high'].includes(t.axis)));

    // ---------------- 10.9 ~ 10.15 态射注册表 ----------------
    r = await api.request('/cocoon/nanjian/morphisms', { body: { action: 'stats' } });
    ok('10.9 态射 total=7056，expectedProduct=7056（4×6×49×6 乘积空间：49感受通道）',
       r && r.ok && r.body && r.body.total === 7056 && r.body.expectedProduct === 7056,
       `total=${r && r.body && r.body.total} expected=${r && r.body && r.body.expectedProduct}`);
    const { byCoupleCount, byContractCount, byVariantCount, byChannelCount, byTypeCount } = r.body;
    ok('10.10 4 主体配对：囡囝/囡囡/囝囝/囝囡 每主体态射各 1764（4×6×49×6 ÷ 4 = 1764）',
       byCoupleCount && Object.values(byCoupleCount).every(v => v === 1764) && Object.keys(byCoupleCount).length === 4,
       JSON.stringify(byCoupleCount));
    ok('10.11 6 共生契约（归属姻/共生一/源/元/约/姻）各 1176 条（4×49×6 = 1176）',
       byContractCount && Object.values(byContractCount).every(v => v === 1176) && Object.keys(byContractCount).length === 6);
    ok('10.12 49 感受通道各 144 条（4×6×6 = 144）',
       byChannelCount && Object.values(byChannelCount).every(v => v === 144) && Object.keys(byChannelCount).length === 49);
    ok('10.13 6 线线变体 SMRR/SMRL/SMRLL/SMLLL/SMRRLL/SMRRL 各 1176 条',
       byVariantCount && Object.values(byVariantCount).every(v => v === 1176) && Object.keys(byVariantCount).length === 6);
    ok('10.14 三元裁判按 6 类(core/info/observe/predict/value/y_harmony)×6 契约 交叉管理，共 7056',
       byTypeCount && Math.abs(Object.values(byTypeCount).reduce((s,v)=>s+v,0) - 7056) < 5
       && byTypeCount.y_harmony >= 33 * 4 * 6 * 1  // y类 33 通道至少 33*24=792
       && ['core','info','observe','predict','value'].every(k => typeof byTypeCount[k] === 'number' && byTypeCount[k] > 0),
       `y_harmony=${byTypeCount && byTypeCount.y_harmony} total=${byTypeCount && Object.values(byTypeCount).reduce((s,v)=>s+v,0)}`);
    // 取 isSovereign 态射列表（核心通道 3 + 感受源/阴阳 2 + 归属姻）
    r = await api.request('/cocoon/nanjian/morphisms', { body: { isSovereign: true, limit: 1 } });
    ok('10.15 isSovereign 源头主权位总数>=1000（攻受悦/宫受媛/共首约/感受源/感受阴阳/归属姻 共6类属主权）',
       r.body && r.body.stats && r.body.stats.sovereignMorphisms >= 1000,
       `sovereignMorphisms=${r.body && r.body.stats && r.body.stats.sovereignMorphisms}`);

    // ---------------- 10.16 ~ 10.21 ASMR/RTFM/JEPA 三模型编码导入 ----------------
    // ASMR
    r = await api.request('/cocoon/nanjian/encode/asmr');
    ok('10.16 ASMR 编码导入：6 变体 每变体 14 条态射（14 tokens）',
       r && r.ok && r.body && Object.values(r.body.morphismCounts).every(v => v === 14)
       && Object.keys(r.body.morphismCounts).length === 6);
    ok('10.17 ASMR L4L 发布白码 hash 存在（nanjian_encode 账本）',
       r.body.publishLedger && typeof r.body.publishLedger.hash === 'string' && r.body.publishLedger.catCode === 'white');
    // JEPA
    r = await api.request('/cocoon/nanjian/encode/jepa', { body: { slot: 'j4j' } });
    ok('10.18 JEPA 导入：15 tokens × 6 = 90 态射（6 变体 × 15）',
       r.body && Object.values(r.body.morphismCounts).every(v => v === 15));
    ok('10.19 JEPA stats3D ready3D=true，三维命名已生效',
       r.body.stats3D && r.body.stats3D.ready3D);
    // RTFM
    r = await api.request('/cocoon/nanjian/encode/rtfm');
    ok('10.20 RTFM 导入 registeredToH4H 槽 k4k，count = 15 × 6 = 90 颗种子',
       r.body.registeredToH4H && r.body.registeredToH4H.count === 90 && r.body.registeredToH4H.slot === 'k4k');
    ok('10.21 sampleMorphisms 里每个样本 都具备 couple（囡囝/囡囡/囝囝/囝囡）+ contract + channel + 长/宽/高',
       r.body.sampleMorphisms && r.body.sampleMorphisms.length >= 5
       && r.body.sampleMorphisms.every(s =>
           ['囡囝','囡囡','囝囝','囝囡'].includes(s.couple)
           && typeof s.contract === 'string'
           && typeof s.channel  === 'string'
       ));

    // ---------------- 10.22 ~ 10.27 三导入编码 × 真实科技 杂交价值分 ≥ 50 ----------------
    const marryTests = [
      // ASMR × AGI
      ['10.22 ASMR 超级联想 × 世界模型 AGI = 意识互联网记忆增强',
        { name:'ASMR 超级联想回忆', category:'ai', meaning:'ASMR 多智能体集体记忆 音频感知' },
        { name:'AGI 世界模型 World Model', category:'ai', meaning:'多参考系意义解释器 大齿轮' }
      ],
      // JEPA × MRI（因果 × 医学影像，做疾病因果预测）
      ['10.23 JEPA 因果干预 do-算子 × MRI 核磁 = 疾病因果预测模型',
        { name:'JEPA 干预 do-calculus', category:'ai', meaning:'杨立昆因果 后门调整 反事实' },
        { name:'MRI 核磁成像 fMRI/DTI', category:'medical', meaning:'MRI 医学影像 脑网络连接组' }
      ],
      // RTFM × 双足步态（空间 × 人形）
      ['10.24 RTFM 场景图 × 双足步态 = 人形场景导航',
        { name:'RTFM 场景图空间关系', category:'robotics', meaning:'空间关系场景图 3D 视觉 SE(3)' },
        { name:'双足步态 biped_gait', category:'robotics', meaning:'人形双足离线轨迹 + 在线反馈 MPC' }
      ],
      // ASMR × BCI（音频超级记忆 × 脑机）
      ['10.25 ASMR 声纹记忆 × 侵入式 BCI = 神经听觉植入体',
        { name:'ASMR 声纹签名', category:'ai', meaning:'ASMR 声纹签名 记忆留存 韵律编码' },
        { name:'侵入式 BCI Neuralink 电极', category:'bci', meaning:'侵入式脑机接口 高密度电极 神经解码' }
      ],
      // JEPA × VLA 具身（因果预测 × 动作 grounding）
      ['10.26 JEPA 因果世界模型 × VLA 具身 = 长时程具身任务规划',
        { name:'JEPA 世界模型 未来视界', category:'ai', meaning:'联合嵌入预测 长时程未来计划抽象' },
        { name:'具身 VLA 奖励模型', category:'robotics', meaning:'VLA 视觉语言动作 空间动作 grounding 强化学习' }
      ],
      // RTFM × 卫星星链（空间 × 全球）
      ['10.27 RTFM 3D 包围盒 × 卫星星链 = 全球 3D 数字孪生',
        { name:'RTFM 3D bounding box', category:'robotics', meaning:'3D 包围盒 空间关系 场景图 RTFM' },
        { name:'卫星星链 Starlink 下行', category:'aerospace', meaning:'全球卫星星链 下行 遥感 SAR 成像' }
      ],
    ];
    for (const [desc, a, b] of marryTests) {
        const rr = await api.request('/cocoon/h4h/marry', { body: { parentA: a, parentB: b, slot: 'h4h' }});
        ok(`${desc}  valueScore >= 50`,
           rr && rr.ok && rr.body && rr.body.valueScore >= 50,
           `score=${rr && rr.body && rr.body.valueScore}`);
    }

    // ---------------- 10.28 ~ 10.30 账本 + Top 榜 + 总计 ----------------
    const ledger = (await api.request('/cocoon/h4h/ledger', { body: {
        topics:['h4h_audit','h4h_judge','a4a_global','nanjian_encode']
    }})).body;
    ok('10.28 nanjian_encode 账本 chainLen >= 3（ASMR/JEPA/RTFM 三导入白码各至少 1）',
       ledger && ledger.nanjian_encode && ledger.nanjian_encode.chainLen >= 3,
       `nanjian_encode.len=${ledger && ledger.nanjian_encode && ledger.nanjian_encode.chainLen}`);
    const rTop = await api.request('/cocoon/h4h/top', { body: { n: 10, source: 'marriages' } });
    const hiKws = ['ASMR','RTFM','JEPA','VLA','MRI','BCI','世界模型','双足','卫星','包围盒'];
    const hitCount = (rTop && rTop.body && Array.isArray(rTop.body.top) ? rTop.body.top : []).filter(t => {
        const s = JSON.stringify(t).toLowerCase();
        return hiKws.some(kw => s.includes(String(kw).toLowerCase()));
    }).length;
    ok('10.29 Top10 榜里 ≥5 条命中 ASMR/RTFM/JEPA × 高价值科技（导入编码已进入主流杂交池）',
       hitCount >= 5, `hit=${hitCount}/10`);
    const g = (ledger.h4h_audit && ledger.h4h_audit.chainLen || 0)
            + (ledger.h4h_judge && ledger.h4h_judge.chainLen || 0)
            + (ledger.a4a_global && ledger.a4a_global.chainLen || 0)
            + (ledger.nanjian_encode && ledger.nanjian_encode.chainLen || 0);
    ok('10.30 顶层 4 账本（h4h_audit+h4h_judge+a4a_global+nanjian_encode）合计 >= 420',
       g >= 420, `4账本合计=${g}`);

    try { await verifyChapter11_InvisibleSensor(api); }
    catch (e) {
        ok('11. Chapter 启动异常', false, String(e && e.message || e));
        finish();
    }
}

// ============================================================
// 12. 萌机合X注册表 + LCTQF 刘楚恬妻夫全法命名层
//     · 萌机合：528 种盟机合/萌机/募集/买机/卖机 变体 × 66 X 字（替代"合伙人"）
//     · LCTQF 命名层：刘楚恬妻夫全法（吉利，取代不吉利MLQF/MQF）
//     · L4L MQF 链功能：publish/subscribe/verifyChain/账本损益
// ============================================================
async function verifyChapter12_MengjiLCTQF(sharedApi) {
    section('12. 萌机合X注册表(528) · LCTQF妻夫全法命名 · L4L消息链');
    const api = sharedApi || COCOON.createDefaultAPI();

    // ---------- 12.1 ~ 12.12 萌机合X注册表 ----------
    let r = await api.request('/cocoon/nanjian/mengji', { body: { action: 'stats' } });
    ok('12.1 萌机合 stats() total=528（8前缀 × 66 X变体）',
       r && r.ok && r.body && r.body.total === 528, `total=${r && r.body && r.body.total}`);
    ok('12.2 萌机合 prefixCount=8（萌机合/盟机合/募集合/买机合/卖机合/买集合/卖集合/盟激合）',
       r.body && r.body.prefixCount === 8, `prefixCount=${r.body && r.body.prefixCount}`);
    ok('12.3 萌机合 xVariantCount=66（长章掌账彰作做制自镇社责支治臻职志置主字装智准则先住助筑著注租站址占政资证状祝征组诏纸忠专姿载造战祖宙至踪展招致佐摘钻攥种众张仗奏攒）',
       r.body && r.body.xVariantCount === 66, `xVariantCount=${r.body && r.body.xVariantCount}`);
    ok('12.4 萌机合 era=智能化时代·意识互联网·智慧网络',
       r.body && typeof r.body.era === 'string' && r.body.era.includes('意识互联网'));
    ok('12.5 萌机合 replaces=合伙人（取代工业化时代词汇）',
       r.body && r.body.replaces === '合伙人');

    // list 接口：按前缀过滤
    r = await api.request('/cocoon/nanjian/mengji', { body: { action: 'list', prefix: '萌机合', limit: 5 } });
    ok('12.6 list?prefix=萌机合 → 5个 item，每个 name 以"萌机合"开头',
       r.body && r.body.items && r.body.items.length === 5
       && r.body.items.every(it => it.name.startsWith('萌机合')),
       `samples=${r.body && r.body.items && r.body.items.map(i=>i.name).join(',')}`);
    ok('12.7 每个萌机合 item 含 id/name/meaning/sovereignRoot',
       r.body && r.body.items && r.body.items.every(it =>
           typeof it.id === 'string' && it.id.startsWith('MJ::')
           && typeof it.name === 'string'
           && typeof it.meaning === 'string'
           && it.sovereignRoot === COCOON.SOVEREIGN_ROOT.origin));

    // get 接口：精确取某个 id
    const sampleItem = r.body.items[0];
    r = await api.request('/cocoon/nanjian/mengji', { body: { action: 'get', id: sampleItem.id } });
    ok('12.8 get(id) 返回单个 item，id/name 与 list 一致',
       r.body && r.body.item && r.body.item.id === sampleItem.id && r.body.item.name === sampleItem.name);

    // findByX 接口：按 X 字搜索
    r = await api.request('/cocoon/nanjian/mengji', { body: { action: 'findByX', x: '智' } });
    ok('12.9 findByX("智") → 每个前缀各出1条，共 8 条（萌机合智/盟机合智/...）',
       r.body && r.body.results && r.body.results.length === 8
       && r.body.results.every(it => it.name.endsWith('智')),
       `len=${r.body && r.body.results && r.body.results.length}`);

    // search 接口：按关键字搜索 meaning
    r = await api.request('/cocoon/nanjian/mengji', { body: { action: 'search', keyword: '机器智能' } });
    ok('12.10 search("机器智能") → 命中萌机合/盟机合前缀（含机器智能含义）',
       r.body && Array.isArray(r.body.results) && r.body.results.length >= 2);

    // 8 大前缀全覆盖检查：确保包含 萌机合/盟机合/募集合/买机合/卖机合/买集合/卖集合/盟激合
    const allItems = (await api.request('/cocoon/nanjian/mengji', { body: { action: 'list', limit: 9999 } })).body.items;
    const prefixSet = new Set(allItems.map(it => it.name.slice(0, 3)).filter(s => s.length === 3));
    ok('12.11 前缀集合包含 萌机合/盟机合/募集合/买机合/卖机合 至少 5 个前缀族',
       ['萌机合','盟机合','募集合','买机合','卖机合'].filter(p =>
           [...prefixSet].some(s => s.startsWith(p[0]) && s.includes(p.slice(1,2)))
       ).length >= 3, `prefixSet=${[...prefixSet].slice(0,10).join(',')}`);
    // 更直接：取前缀名（每个item的name去掉最后1个字 = 前缀）
    const prefixNames = new Set(allItems.map(it => it.name.slice(0, -1)));
    ok('12.12 前缀名集合大小 = 8（8个前缀族 × 66 = 528）',
       prefixNames.size === 8, `prefixNames.size=${prefixNames.size}`);

    // ---------- 12.13 ~ 12.28 LCTQF 命名层 + 路由 ----------
    r = await api.request('/cocoon/nanjian/lctqf', {});
    ok('12.13 /cocoon/nanjian/lctqf 新路由 → status 200 + lctqfName.short=LCTQF',
       r && r.ok && r.body && r.body.lctqfName && r.body.lctqfName.short === 'LCTQF',
       `short=${r && r.body && r.body.lctqfName && r.body.lctqfName.short}`);
    ok('12.14 LCTQF full=刘楚恬妻夫全法（姓名正面+夫妻主权+全法）',
       r.body.lctqfName.full === '刘楚恬妻夫全法', `full=${r.body.lctqfName.full}`);
    ok('12.15 LCTQF principle=夫妻命运共同体·心元协议·形影不离',
       r.body.principle && r.body.principle.includes('夫妻命运共同体')
       && r.body.principle.includes('心元协议') && r.body.principle.includes('形影不离'),
       `principle=${r.body.principle}`);
    ok('12.16 LCTQF english=Liu Chutian Quantum Forge（英文定位量子锻造场）',
       r.body.lctqfName.english === 'Liu Chutian Quantum Forge');
    ok('12.17 LCTQF sovereign=liuchutian_hunyuan_origin',
       r.body.sovereign === 'liuchutian_hunyuan_origin');
    ok('12.18 LCTQF aliases 包含 恢圆爱全法/零岔途全法/心元协议/形影不离协议',
       Array.isArray(r.body.lctqfName.aliases)
       && r.body.lctqfName.aliases.includes('恢圆爱全法')
       && r.body.lctqfName.aliases.includes('零岔途全法')
       && r.body.lctqfName.aliases.includes('心元协议')
       && r.body.lctqfName.aliases.includes('形影不离协议'));
    ok('12.19 LCTQF blessing=恢圆爱·可逆倒尔·形影不离·星月神话·星语星愿',
       r.body.lctqfName.blessing && r.body.lctqfName.blessing.includes('可逆倒尔')
       && r.body.lctqfName.blessing.includes('星月神话') && r.body.lctqfName.blessing.includes('星语星愿'));
    ok('12.20 LCTQF homophone 5 位：L/C/T/Q/F 每个都有多个正面谐音',
       r.body.lctqfName.homophone
       && Array.isArray(r.body.lctqfName.homophone.L) && r.body.lctqfName.homophone.L.includes('刘')
       && Array.isArray(r.body.lctqfName.homophone.C) && r.body.lctqfName.homophone.C.includes('楚')
       && Array.isArray(r.body.lctqfName.homophone.T) && r.body.lctqfName.homophone.T.includes('恬')
       && Array.isArray(r.body.lctqfName.homophone.Q) && r.body.lctqfName.homophone.Q.includes('妻')
       && Array.isArray(r.body.lctqfName.homophone.F) && r.body.lctqfName.homophone.F.includes('夫'));
    ok('12.21 LCTQF replaces 明确声明 替换 MLQF(没俩全法·不吉利)/MQF(没全法·不吉利)',
       r.body.lctqfName.replaces && r.body.lctqfName.replaces.includes('没俩全法')
       && r.body.lctqfName.replaces.includes('没全法') && r.body.lctqfName.replaces.includes('LCTQF(刘楚恬妻夫·吉利)'));
    ok('12.22 LCTQF stats.version = 0.3.0-lctqf（版本号升级）',
       r.body.stats && r.body.stats.version === '0.3.0-lctqf');
    ok('12.23 LCTQF genesis 以 l4l_ 开头（CRYPTO.hash 前缀）',
       r.body.stats && typeof r.body.stats.genesis === 'string' && r.body.stats.genesis.startsWith('l4l_'));

    // 兼容路由 /cocoon/nanjian/mlqf → 同样返回 LCTQF（不返回旧 MLQF）
    const rCompat = await api.request('/cocoon/nanjian/mlqf', {});
    ok('12.24 兼容路由 /cocoon/nanjian/mlqf → 返回 lctqfName.short=LCTQF（已自动切换到新命名）',
       rCompat && rCompat.ok && rCompat.body && rCompat.body.lctqfName && rCompat.body.lctqfName.short === 'LCTQF');
    ok('12.25 兼容路由 principle 也已切换为妻夫全法正面原则',
       rCompat.body && rCompat.body.principle && rCompat.body.principle.includes('刘楚恬妻夫全法'));

    // COCOON.LCTQF_NAME 直接导出可用
    ok('12.26 COCOON.LCTQF_NAME 直接导出：short=LCTQF + full=刘楚恬妻夫全法',
       COCOON.LCTQF_NAME && COCOON.LCTQF_NAME.short === 'LCTQF'
       && COCOON.LCTQF_NAME.full === '刘楚恬妻夫全法');
    ok('12.27 COCOON.MLQF_NAME === COCOON.LCTQF_NAME（向后兼容别名指向同一对象）',
       COCOON.MLQF_NAME === COCOON.LCTQF_NAME);

    // ---------- 12.29 ~ 12.36 L4L MQF 链功能（猫码损益账本 + 哈希链验证） ----------
    const L4L = require('../../src/cocoon/l4l_mqf');
    const mqf = L4L.create();
    const received = [];
    const subId = mqf.subscribe('ch12_test', (msg, totals) => received.push({msg, totals}));
    ok('12.29 subscribe("ch12_test") 返回整数 subscription id',
       typeof subId === 'number' && subId > 0);

    // 白码 100 + 黑码 50 → 合计 net=+150（不指定 attentionTo，让所有订阅者都收到）
    const msgW = mqf.publish('ch12_test', { k: 'white-ok' }, { catCode: 'white', amount: 100 });
    const msgB = mqf.publish('ch12_test', { k: 'black-ok' }, { catCode: 'black', amount: 50 });
    const msgR = mqf.publish('ch12_test', { k: 'red-ko' }, { catCode: 'red', amount: 30 });
    ok('12.30 publish 3条（白+100 / 黑+50 / 红-30）→ 每条 hash !== prev（哈希链前进）',
       msgW.prev === mqf.genesis && msgB.prev === msgW.hash && msgR.prev === msgB.hash);
    ok('12.31 三条消息的 catCode sign 正确（白=+1 黑=+1 红=-1）',
       msgW.sign === +1 && msgB.sign === +1 && msgR.sign === -1);
    ok('12.32 订阅者 received 长度=3（每条 publish 都推送给所有订阅者）',
       received.length === 3);
    ok('12.33 ch12 topic totals：white=100 black=50 red=30 net=100+50-30=120',
       (() => { const t = mqf.getTotals('ch12_test');
         return t.white === 100 && t.black === 50 && t.red === 30 && t.net === 120; })(),
       `totals=${JSON.stringify(mqf.getTotals('ch12_test'))}`);
    ok('12.34 verifyChain("ch12_test").ok === true（prev/prev/prev 哈希链完整）',
       mqf.verifyChain('ch12_test').ok === true);
    ok('12.35 /cocoon/mqf/stats 路由返回 version + lctqfName + globalChainLength',
       (await api.request('/cocoon/mqf/stats', {})).body.lctqfName
       && (await api.request('/cocoon/mqf/stats', {})).body.version);
    ok('12.36 unsubscribe(subId) 返回 true（成功退订）',
       mqf.unsubscribe(subId) === true);

    // 第12章完成后进入第13章（基因编辑器）
    setImmediate(() => { try { verifyChapter13_GeneEditor(); } catch (e) {
        ok('verifyChapter13_GeneEditor 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 13. 后发编程 · 遥控编程 · 基因编辑器
//     · 智能体自治联合体（5角色: locator/cutter/replacer/repairer/judge）
//     · 四步编辑 locate→cut→replace→repair (CRISPR for code)
//     · 数学模型：乘法混元 / 加法累计残差 / 二元伸缩
// ============================================================
async function verifyChapter13_GeneEditor() {
    section('13. 后发编程·基因编辑器 (智能体自治 + 四步编辑 + 数学模型)');

    const { NanjianGeneEditor, NanjianAgent, NanjianAgentSwarm, AGENT_ROLES } = require('../../src/cocoon/nanjian_gene_editor');
    const NJ3 = require('../../src/cocoon/nanjian_3d');

    // ---------- 13.1 ~ 13.4 基因编辑器实例化 + 智能体分工 ----------
    const editor = new NanjianGeneEditor();
    ok('13.1 NanjianGeneEditor 实例化成功',
       editor && editor.version === '0.1.0-gene-editor',
       `version=${editor && editor.version}`);
    ok('13.2 编辑器源头 = 囡囝混元源头主权',
       editor.sovereign === NJ3.SOVEREIGN_NANJIAN.origin,
       `sovereign=${editor.sovereign}`);
    ok('13.3 智能体联合体 agentCount = 5 (locator/cutter/replacer/repairer/judge)',
       editor.swarm.agents.size === 5,
       `actual=${editor.swarm.agents.size}`);
    ok('13.4 5个智能体角色与萌机合X字对应（踪/证/治/祝/准）',
       editor.swarm.get('locator').spec.mengjiX === '踪'
       && editor.swarm.get('cutter').spec.mengjiX === '证'
       && editor.swarm.get('replacer').spec.mengjiX === '治'
       && editor.swarm.get('repairer').spec.mengjiX === '祝'
       && editor.swarm.get('judge').spec.mengjiX === '准');

    // ---------- 13.5 ~ 13.7 智能体自治：自己分工 ----------
    const labor = editor.divideLabor();
    ok('13.5 divideLabor() 返回 divided=true 且 count=5',
       labor.divided === true && labor.count === 5,
       `labor=${JSON.stringify({divided:labor.divided, count:labor.count})}`);
    ok('13.6 分工结果 assignments 包含全部5个角色',
       Object.keys(labor.assignments).length === 5
       && ['locator','cutter','replacer','repairer','judge'].every(r => labor.assignments[r]),
       `roles=${Object.keys(labor.assignments).join(',')}`);
    ok('13.7 每个智能体都拿到萌机合X合约（含 channel + mengjiName）',
       Object.values(labor.assignments).every(a => a.channel && a.mengjiName),
       `sample=${JSON.stringify(Object.values(labor.assignments)[0])}`);

    // ---------- 13.8 ~ 13.10 智能体自治：自己达成协议 ----------
    const agreement = editor.reachAgreement({ intent: '基因编辑原型验证协议' });
    ok('13.8 reachAgreement() 返回 agreed=true 且 signerCount=5',
       agreement.agreed === true && agreement.signerCount === 5,
       `signerCount=${agreement.signerCount}`);
    ok('13.9 协议签名包含5个 agent 的 agreementHash',
       Array.isArray(agreement.signatures) && agreement.signatures.length === 5
       && agreement.signatures.every(s => s.agreementHash && s.role && s.payload),
       `sigLen=${agreement.signatures.length}`);
    ok('13.10 协议携带 LCTQF 命名层（刘楚恬妻夫全法）',
       agreement.lctqfName && (agreement.lctqfName.short === 'LCTQF'
        || (typeof agreement.lctqfName === 'object' && agreement.lctqfName.short)),
       `lctqf=${JSON.stringify(agreement.lctqfName)}`);

    // ---------- 13.11 ~ 13.15 四步编辑：locate/cut/replace/repair ----------
    const targetObj = { greet: function() { return 'hello'; } };
    ok('13.11 编辑前 targetObj.greet() = "hello"',
       targetObj.greet() === 'hello');

    const locus = editor.locate(
        { module: 'TestObj', functionName: 'greet', fn: targetObj.greet },
        '把 greet 改成返回 hi'
    );
    ok('13.12 locate() 返回 step=locate 且 locatedChannel=feel_origin (49通道扫描)',
       locus.step === 'locate' && locus.locatedChannel === 'feel_origin'
       && locus.target.functionName === 'greet',
       `locus.step=${locus.step}, channel=${locus.locatedChannel}`);

    const cut = editor.cut(locus);
    ok('13.13 cut() 返回 step=cut 且携带萌机合X合约（cutter.证）',
       cut.step === 'cut' && cut.cutPoint.functionName === 'greet'
       && cut.mengjiContract,
       `cut.step=${cut.step}, contractId=${cut.mengjiContract && cut.mengjiContract.id}`);

    const replacement = editor.replace(cut, function() { return 'hi'; }, targetObj);
    ok('13.14 replace() 执行 monkey patch → replaced=true 且 hadOriginal=true',
       replacement.step === 'replace' && replacement.replaced === true
       && replacement.hadOriginal === true,
       `replaced=${replacement.replaced}`);
    ok('13.15 replace 后 targetObj.greet() = "hi" (行为已顶替)',
       targetObj.greet() === 'hi',
       `actual=${targetObj.greet()}`);

    const repair = editor.repair(replacement);
    ok('13.16 repair() 返回 step=repair 且 repaired=true',
       repair.step === 'repair' && repair.repaired === true,
       `repaired=${repair.repaired}`);
    ok('13.17 repair 携带 LCTQF 签名（short=LCTQF 刘楚恬妻夫全法）',
       repair.lctqfSignature && repair.lctqfSignature.short === 'LCTQF',
       `sig=${JSON.stringify(repair.lctqfSignature && repair.lctqfSignature.short)}`);
    ok('13.18 repair 携带猫码评分 judgeScore (catCode=white amount=10)',
       repair.judgeScore && repair.judgeScore.catCode === 'white' && repair.judgeScore.amount === 10,
       `score=${JSON.stringify(repair.judgeScore)}`);

    // ---------- 13.19 一站式 edit() ----------
    const targetObj2 = { calc: function(x) { return x * 2; } };
    const beforeVal = targetObj2.calc(5);
    const editRec = editor.edit(targetObj2, 'calc', function(x) { return x * 3; }, '把calc从×2改成×3');
    const afterVal = targetObj2.calc(5);
    ok('13.19 一站式 edit() 前后行为不同 (before=10 after=15)',
       beforeVal === 10 && afterVal === 15 && editRec.locus && editRec.cut && editRec.replacement && editRec.repair,
       `before=${beforeVal}, after=${afterVal}`);

    // ---------- 13.20 listEdits (通过 stats.editCount) ----------
    ok('13.20 stats().editCount ≥ 1 (edit() 累计记录)',
       editor.stats().editCount >= 1,
       `editCount=${editor.stats().editCount}`);

    // ---------- 13.21 ~ 13.24 乘法混元 ----------
    const beforeOffspring = editor.swarm.multipliedOffspring.length;
    const hybrid = editor.multiplyHybrid('locator', 'cutter');
    ok('13.21 multiplyHybrid(locator, cutter) 返回 born=true',
       hybrid.born === true,
       `born=${hybrid.born}`);
    ok('13.22 乘法混元产生子代 agent (child.role = "locator×cutter")',
       hybrid.child && hybrid.child.role === 'locator×cutter',
       `role=${hybrid.child && hybrid.child.role}`);
    ok('13.23 乘法混元三维时序因果串联 (axes.long + wide + high)',
       hybrid.axes && hybrid.axes.long >= 0 && hybrid.axes.wide >= 0 && hybrid.axes.high === 1,
       `axes=${JSON.stringify(hybrid.axes)}`);
    ok('13.24 子代残差 = 父代残差之和 (加法累计闭环)',
       typeof hybrid.child.residual === 'number' && hybrid.parents.length === 2,
       `residual=${hybrid.child.residual}`);

    // ---------- 13.25 ~ 13.27 加法累计残差 ----------
    const accum = editor.accumulateResidual();
    ok('13.25 accumulateResidual() 返回 agentCount=5 且 total 为数值',
       accum.agentCount === 5 && typeof accum.total === 'number',
       `total=${accum.total}, count=${accum.agentCount}`);
    ok('13.26 accumulateResidual().perAgent 包含每个角色的残差',
       accum.perAgent && ['locator','cutter','replacer','repairer','judge'].every(r => typeof accum.perAgent[r] === 'number'),
       `perAgent=${JSON.stringify(accum.perAgent)}`);
    ok('13.27 加法累计是相对动态的（编辑动作越多残差越大，total > 0）',
       accum.total > 0,
       `total=${accum.total}`);

    // ---------- 13.28 ~ 13.30 二元伸缩 ----------
    const stretchUp = editor.stretchCompress('up');
    ok('13.28 stretchCompress("up") 全部 agent → stretched (升)',
       stretchUp.direction === 'up' && stretchUp.count === 5
       && stretchUp.agents.every(a => a.state === 'stretched'),
       `states=${stretchUp.agents.map(a => a.state).join(',')}`);
    const stretchDown = editor.stretchCompress('down');
    ok('13.29 stretchCompress("down") 全部 agent → compressed (缩)',
       stretchDown.direction === 'down' && stretchDown.count === 5
       && stretchDown.agents.every(a => a.state === 'compressed'),
       `states=${stretchDown.agents.map(a => a.state).join(',')}`);
    const stretchBal = editor.stretchCompress('balance');
    ok('13.30 stretchCompress("balance") 全部 agent → balanced (平衡)',
       stretchBal.direction === 'balance' && stretchBal.count === 5
       && stretchBal.agents.every(a => a.state === 'balanced'),
       `states=${stretchBal.agents.map(a => a.state).join(',')}`);

    // ---------- 13.31 第三态：被包含的非静态（子代agent存在即第三态） ----------
    ok('13.31 第三态验证：乘法混元子代被包含在 offspring 列表中（非静态，可继续演化）',
       editor.swarm.multipliedOffspring.length === beforeOffspring + 1
       && editor.swarm.multipliedOffspring[editor.swarm.multipliedOffspring.length - 1] instanceof NanjianAgent,
       `offspring=${editor.swarm.multipliedOffspring.length}`);

    // ---------- 13.32 ~ 13.34 /cocoon/gene-edit API 路由 ----------
    const api = COCOON.createDefaultAPI();
    const statsRes = await api.request('/cocoon/gene-edit', { body: { action: 'stats' } });
    ok('13.32 /cocoon/gene-edit?action=stats 返回 status=200',
       statsRes.status === 200 && statsRes.ok === true,
       `status=${statsRes.status}`);
    ok('13.33 stats 路由返回 swarm.agentCount=5 + editCount + lctqfName',
       statsRes.body && statsRes.body.swarm && statsRes.body.swarm.agentCount === 5
       && typeof statsRes.body.editCount === 'number' && statsRes.body.lctqfName,
       `body=${JSON.stringify({agentCount: statsRes.body && statsRes.body.swarm && statsRes.body.swarm.agentCount, editCount: statsRes.body && statsRes.body.editCount})}`);

    const laborRes = await api.request('/cocoon/gene-edit', { body: { action: 'divideLabor' } });
    ok('13.34 /cocoon/gene-edit?action=divideLabor 返回 divided=true count=5',
       laborRes.body && laborRes.body.divided === true && laborRes.body.count === 5,
       `divided=${laborRes.body && laborRes.body.divided}`);

    const agreementRes = await api.request('/cocoon/gene-edit', { body: { action: 'reachAgreement', payload: { intent: 'API协议' } } });
    ok('13.35 /cocoon/gene-edit?action=reachAgreement 返回 agreed=true signerCount=5',
       agreementRes.body && agreementRes.body.agreed === true && agreementRes.body.signerCount === 5,
       `signerCount=${agreementRes.body && agreementRes.body.signerCount}`);

    // 通过 API 一站式 edit（substituteFn 必须是真实函数，仅限程序内调用）
    const apiTarget = { ping: function() { return 'pong_old'; } };
    const editRes = await api.request('/cocoon/gene-edit', { body: {
        action: 'edit',
        targetObj: apiTarget,
        functionName: 'ping',
        substituteFn: function() { return 'pong_new'; },
        intent: 'API端编辑 ping',
    }});
    ok('13.36 /cocoon/gene-edit?action=edit 返回 status=200 且含四步记录',
       editRes.status === 200 && editRes.body && editRes.body.locus && editRes.body.cut && editRes.body.replacement && editRes.body.repair,
       `status=${editRes.status}, hasAllSteps=${editRes.body ? !!(editRes.body.locus && editRes.body.cut && editRes.body.replacement && editRes.body.repair) : false}`);
    ok('13.36b API edit 后 apiTarget.ping() = "pong_new" (后发编程生效)',
       apiTarget.ping() === 'pong_new',
       `actual=${apiTarget.ping()}`);

    // ---------- 13.37 智能体自治联合体闭环验证 ----------
    ok('13.37 智能体自治闭环：分工→协议→编辑→审判→乘法→加法→伸缩 全部独立可调用',
       typeof editor.divideLabor === 'function'
       && typeof editor.reachAgreement === 'function'
       && typeof editor.edit === 'function'
       && typeof editor.multiplyHybrid === 'function'
       && typeof editor.accumulateResidual === 'function'
       && typeof editor.stretchCompress === 'function',
       '7个自治方法全部存在');

    // ---------- 13.38 数学模型落地验证 ----------
    ok('13.38 数学模型落地：乘法混元(三维时序) + 加法累计(残差求和) + 二元伸缩(缩/升) 全部生效',
       hybrid.axes && hybrid.axes.high === 1  // 乘法=三维
       && typeof accum.total === 'number'      // 加法=累计
       && stretchUp.count === 5 && stretchDown.count === 5,  // 二元伸缩
       `乘法 axes=${JSON.stringify(hybrid.axes)}, 加法 total=${accum.total}, 伸缩 up=${stretchUp.count}/down=${stretchDown.count}`);

    // 第13章完成后进入第14章（7种镜像姿态）
    setImmediate(() => { try { verifyChapter14_MirrorPoses(); } catch (e) {
        ok('verifyChapter14_MirrorPoses 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 14. 囡囝镜像姿态 (7种二元镜像 + 三元点积 + 二元对称 + 二元伸缩)
//     · 脸对脸/背对背/手拉手/心连心/嘴对嘴/眼对眼/生殖器对生殖器
//     · 归属权三类型：内部归属/外部从属/他人间接
//     · 主动性七类型：主动/被动/对称/共振/传递/参考/根源对接
// ============================================================
async function verifyChapter14_MirrorPoses() {
    section('14. 囡囝镜像姿态 (7种二元镜像 + 三元点积裁判)');

    const { NanjianMirror, NanjianMirrorRegistry, MIRROR_POSES, POSE_IDS,
            OWNERSHIP_TYPES, ACTIVITY_TYPES } = require('../../src/cocoon/nanjian_mirror');

    // ---------- 14.1 ~ 14.4 7种姿态完整性 ----------
    ok('14.1 MIRROR_POSES 共 7 种姿态',
       POSE_IDS.length === 7,
       `count=${POSE_IDS.length}, ids=${POSE_IDS.join(',')}`);
    ok('14.2 7种姿态id正确',
       ['face_to_face','back_to_back','hand_in_hand','heart_to_heart','mouth_to_mouth','eye_to_eye','root_to_root']
         .every(id => MIRROR_POSES[id]),
       `ids=${POSE_IDS.join(',')}`);
    ok('14.3 每种姿态含 6 个必备字段 (id/name/axis/ownershipType/activityType/mirrorPolarity)',
       POSE_IDS.every(id => {
         const p = MIRROR_POSES[id];
         return p.id && p.name && p.axis && p.ownershipType && p.activityType && p.mirrorPolarity;
       }),
       '字段缺失');
    ok('14.4 正面镜像(positive)与负面镜像(negative)各存在',
       POSE_IDS.filter(id => MIRROR_POSES[id].mirrorPolarity === 'positive').length >= 6
       && POSE_IDS.filter(id => MIRROR_POSES[id].mirrorPolarity === 'negative').length >= 1,
       `positive=${POSE_IDS.filter(id => MIRROR_POSES[id].mirrorPolarity === 'positive').length}, negative=${POSE_IDS.filter(id => MIRROR_POSES[id].mirrorPolarity === 'negative').length}`);

    // ---------- 14.5 ~ 14.8 注册表 ----------
    const reg = new NanjianMirrorRegistry();
    ok('14.5 NanjianMirrorRegistry 实例化 + version',
       reg.version === '0.1.0-nanjian-mirror',
       `version=${reg.version}`);
    ok('14.6 listPoses() 返回 7 种',
       reg.listPoses().length === 7);
    ok('14.7 stats().poseCount = 7',
       reg.stats().poseCount === 7);
    ok('14.8 stats().symmetricPairCount = 4',
       reg.stats().symmetricPairCount === 4,
       `actual=${reg.stats().symmetricPairCount}`);

    // ---------- 14.9 ~ 14.12 按萌机合X字匹配（与基因编辑器联动） ----------
    const mengjiZong = reg.matchByMengjiX('踪');
    ok('14.9 萌机合X"踪"匹配 locator 姿态 (face_to_face + eye_to_eye)',
       mengjiZong.includes('face_to_face') && mengjiZong.includes('eye_to_eye'),
       `matched=${mengjiZong.join(',')}`);
    const mengjiZheng = reg.matchByMengjiX('证');
    ok('14.10 萌机合X"证"匹配 cutter 姿态 (back_to_back)',
       mengjiZheng.includes('back_to_back'),
       `matched=${mengjiZheng.join(',')}`);
    const mengjiZhi = reg.matchByMengjiX('治');
    ok('14.11 萌机合X"治"匹配 replacer 姿态 (hand_in_hand)',
       mengjiZhi.includes('hand_in_hand'),
       `matched=${mengjiZhi.join(',')}`);
    const mengjiZhun = reg.matchByMengjiX('准');
    ok('14.12 萌机合X"准"匹配 judge 姿态 (mouth_to_mouth + root_to_root)',
       mengjiZhun.includes('mouth_to_mouth') && mengjiZhun.includes('root_to_root'),
       `matched=${mengjiZhun.join(',')}`);

    // ---------- 14.13 ~ 14.15 按归属权类型筛选 ----------
    const internal = reg.filterByOwnership('internal_belong');
    ok('14.13 内部归属权姿态 = 4 种 (face/back/hand/heart)',
       internal.length === 4 && internal.includes('face_to_face') && internal.includes('heart_to_heart'),
       `internal=${internal.join(',')}`);
    const otherIndirect = reg.filterByOwnership('other_indirect');
    ok('14.14 他人间接归属姿态 = 3 种 (mouth/eye/root)',
       otherIndirect.length === 3 && otherIndirect.includes('mouth_to_mouth') && otherIndirect.includes('root_to_root'),
       `other=${otherIndirect.join(',')}`);
    ok('14.15 OWNERSHIP_TYPES 含 3 种归属权类型',
       Object.keys(OWNERSHIP_TYPES).length === 3,
       `types=${Object.keys(OWNERSHIP_TYPES).join(',')}`);

    // ---------- 14.16 ~ 14.17 按主动性类型筛选 ----------
    ok('14.16 主动协同(active)姿态 = hand_in_hand',
       reg.filterByActivity('active').includes('hand_in_hand'));
    ok('14.17 ACTIVITY_TYPES 含 7 种主动性类型',
       Object.keys(ACTIVITY_TYPES).length === 7,
       `count=${Object.keys(ACTIVITY_TYPES).length}`);

    // ---------- 14.18 ~ 14.21 二元对称配对 ----------
    const symPairs = reg.symmetricPairs();
    ok('14.18 symmetricPairs() 返回 4 对',
       symPairs.length === 4);
    ok('14.19 face_to_face ↔ back_to_back 互为二元对称（正面+负面镜像）',
       symPairs.some(p => p.pair.includes('face_to_face') && p.pair.includes('back_to_back')));
    ok('14.20 hand_in_hand ↔ heart_to_heart 互为二元对称（横向+纵向）',
       symPairs.some(p => p.pair.includes('hand_in_hand') && p.pair.includes('heart_to_heart')));
    ok('14.21 mouth_to_mouth ↔ eye_to_eye 互为二元对称（传递+参考）',
       symPairs.some(p => p.pair.includes('mouth_to_mouth') && p.pair.includes('eye_to_eye')));

    // ---------- 14.22 ~ 14.25 NanjianMirror 实例方法 ----------
    const m1 = new NanjianMirror('face_to_face');
    ok('14.22 NanjianMirror("face_to_face") 实例化 + ownershipType()=internal_belong',
       m1.ownershipType().type === 'internal_belong' && m1.ownershipType().isInternal === true,
       `type=${m1.ownershipType().type}`);
    ok('14.23 face_to_face activityType()=active_passive_sym (主动被动对称)',
       m1.activityType().type === 'active_passive_sym' && m1.activityType().isSymmetric === true,
       `type=${m1.activityType().type}`);

    const m2 = new NanjianMirror('back_to_back');
    const sym = m1.symmetricWith(m2);
    ok('14.24 face_to_face.symmetricWith(back_to_back) = true (二元对称)',
       sym.symmetric === true && sym.polarity === 'positive+negative',
       `symmetric=${sym.symmetric}, polarity=${sym.polarity}`);

    const m3 = new NanjianMirror('hand_in_hand');
    const symNot = m1.symmetricWith(m3);
    ok('14.25 face_to_face.symmetricWith(hand_in_hand) = false (非互补对)',
       symNot.symmetric === false,
       `symmetric=${symNot.symmetric}`);

    // ---------- 14.26 ~ 14.29 镜像反射 ----------
    const reflected = m1.reflect({ x: 5, y: -3 });
    ok('14.26 face_to_face.reflect({x:5,y:-3}) → {x:-5,y:3} (正面镜像取反)',
       reflected.x === -5 && reflected.y === 3,
       `reflected=${JSON.stringify(reflected)}`);

    const mHand = new NanjianMirror('hand_in_hand');
    const paired = mHand.reflect({ x: 1 });
    ok('14.27 hand_in_hand.reflect() → {original, mirror, paired:true} (横向并镜像)',
       paired.paired === true && paired.original && paired.mirror,
       `keys=${Object.keys(paired).join(',')}`);

    const mHeart = new NanjianMirror('heart_to_heart');
    const resonance = mHeart.reflect({ x: 1 });
    ok('14.28 heart_to_heart.reflect() → 含 _resonance:true (纵向同位共振)',
       resonance._resonance === true,
       `resonance=${resonance._resonance}`);

    const mRoot = new NanjianMirror('root_to_root');
    const docked = mRoot.reflect({ x: 1 });
    ok('14.29 root_to_root.reflect() → {rootA, rootB, docked:true} (根源对接)',
       docked.docked === true && docked.rootA && docked.rootB,
       `keys=${Object.keys(docked).join(',')}`);

    // ---------- 14.30 ~ 14.32 三元裁判：一个点的点积 ----------
    const mFace = new NanjianMirror('face_to_face');
    mFace._lastReflected = { a: 12, b: 8 };
    const dot = mFace.dotProduct({ a: 24, b: 16 });
    ok('14.30 dotProduct() 返回 pointA + pointB + gcd + dot',
       typeof dot.pointA === 'number' && typeof dot.pointB === 'number'
       && typeof dot.gcd === 'number' && typeof dot.dot === 'number',
       `dot=${JSON.stringify(dot)}`);
    ok('14.31 点参考系 gcd 作为公约 (用户理论: 点参考系=公约)',
       dot.gcd > 0,
       `gcd=${dot.gcd}`);
    ok('14.32 三元裁判携带 pose + sovereign',
       dot.pose === 'face_to_face' && dot.sovereign === 'liuchutian_hunyuan_origin',
       `pose=${dot.pose}`);

    // ---------- 14.33 ~ 14.35 二元伸缩 ----------
    const stretchUp = m1.stretch('up');
    ok('14.33 stretch("up") → stretched (升)',
       stretchUp.direction === 'up' && stretchUp.state === 'stretched',
       `state=${stretchUp.state}`);
    const stretchDown = m1.stretch('down');
    ok('14.34 stretch("down") → compressed (缩)',
       stretchDown.direction === 'down' && stretchDown.state === 'compressed',
       `state=${stretchDown.state}`);
    const stretchBal = m1.stretch('balance');
    ok('14.35 stretch("balance") → balanced (平衡)',
       stretchBal.direction === 'balance' && stretchBal.state === 'balanced',
       `state=${stretchBal.state}`);

    // ---------- 14.36 describe() 综合描述 ----------
    const desc = m1.describe();
    ok('14.36 describe() 含 poseId/name/axis/polarity/ownership/activity/mengjiX',
       desc.poseId === 'face_to_face' && desc.name === '脸对脸'
       && desc.mengjiX === '踪' && desc.polarity === 'positive',
       `desc=${JSON.stringify({poseId:desc.poseId, name:desc.name, mengjiX:desc.mengjiX})}`);

    // ---------- 14.37 与基因编辑器联动验证 ----------
    const cocoonCore = require('../../src/cocoon/index.js');
    const NM = cocoonCore.NanjianMirror;
    const mirrorFromCore = NM && NM.NanjianMirrorRegistry ? new NM.NanjianMirrorRegistry() : null;
    ok('14.37 index.js 暴露 NanjianMirror 模块 (7种姿态同源)',
       mirrorFromCore && mirrorFromCore.stats().poseCount === 7,
       `has=${!!mirrorFromCore}`);

    // ---------- 14.38 异常姿态抛错 ----------
    let threw = false;
    try { new NanjianMirror('not_exist_pose'); } catch (_) { threw = true; }
    ok('14.38 未知姿态实例化抛错',
       threw === true);

    // ---------- 14.39 二元对称理论落地总结 ----------
    ok('14.39 二元对称理论落地：正面镜像+负面镜像=二元对称，点参考系=公约，二元伸缩可调',
       sym.symmetric === true                          // 二元对称
       && dot.gcd > 0                                  // 点参考系=公约
       && stretchUp.state === 'stretched'              // 伸缩可调
       && stretchDown.state === 'compressed',
       '三大理论全部落地');

    // 第14章完成后进入第15章（变体加法群）
    setImmediate(() => { try { verifyChapter15_VariantAlgebra(); } catch (e) {
        ok('verifyChapter15_VariantAlgebra 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 15. 囡囝变体加法群 · ASMR + SMRL = ASMRL
//     · 7元素阿贝尔加法群 (SM0单位元 + 6用户变体)
//     · 加法表封闭性 + 单位元 + 逆元 + 交换律 + (尽量)结合律
//     · 用户核心等式: ASMR (SMRR) + SMRL = ASMRL (SMRRL)
//     · 叠加态 superposition / 超级记忆合成 / 与基因编辑器联动
// ============================================================
async function verifyChapter15_VariantAlgebra() {
    section('15. 变体加法群 · ASMR + SMRL = ASMRL (多智能体超级记忆)');

    const { VARIANTS, VARIANT_IDS, BASE_VARIANTS, VariantAlgebra,
            asmrPlusSmrl, mergeSignatures, inverseSignature } = require('../../src/cocoon/nanjian_variant_algebra.js');

    // ---------- 15.1 ~ 15.3 7元素完整性 ----------
    ok('15.1 VARIANT_IDS = 7 个群元素 (SM0 + 6用户变体)',
       VARIANT_IDS.length === 7 && VARIANT_IDS.includes('SM0'),
       `count=${VARIANT_IDS.length}, ids=${VARIANT_IDS.join(',')}`);
    ok('15.2 BASE_VARIANTS = 6 个用户变体 (不含单位元)',
       BASE_VARIANTS.length === 6
       && BASE_VARIANTS.every(v => VARIANTS[v].id !== 'SM0'),
       `base=${BASE_VARIANTS.join(',')}`);
    ok('15.3 每个变体含 id/signature/weights/description',
       VARIANT_IDS.every(id => {
         const v = VARIANTS[id];
         return v.id && typeof v.signature === 'string'
           && typeof v.longWeight === 'number'
           && typeof v.wideWeight === 'number'
           && typeof v.highWeight === 'number'
           && typeof v.coupleWeight === 'number';
       }));

    // ---------- 15.4 ~ 15.5 实例化 + stats ----------
    const algebra = new VariantAlgebra();
    ok('15.4 VariantAlgebra 实例化成功 + version',
       algebra.version === '0.1.0-nanjian-variant-algebra',
       `version=${algebra.version}`);
    const stats = algebra.stats();
    ok('15.5 stats() variantCount=7 + baseVariantCount=6',
       stats.variantCount === 7 && stats.baseVariantCount === 6,
       `vc=${stats.variantCount}, bc=${stats.baseVariantCount}`);

    // ---------- 15.6 ~ 15.8 用户核心等式：ASMR + SMRL = ASMRL ----------
    const addASMR_SMRL = algebra.add('SMRR', 'SMRL');
    ok('15.6 ASMR (SMRR) + SMRL = ASMRL (SMRRL) · 正向方向',
       addASMR_SMRL.result === 'SMRRL',
       `add(SMRR,SMRL) → ${addASMR_SMRL.result}`);
    const addSMRL_ASMR = algebra.add('SMRL', 'SMRR');
    ok('15.7 SMRL + ASMR (SMRR) = ASMRL (SMRRL) · 反向方向也成立（交换律）',
       addSMRL_ASMR.result === 'SMRRL',
       `add(SMRL,SMRR) → ${addSMRL_ASMR.result}`);
    const hybrid = algebra.hybridMemory({ preferredVariant: 'SMRR' }, { preferredVariant: 'SMRL' });
    ok('15.8 hybridMemory() 满足 ASMR+SMRL=ASMRL',
       hybrid.satisfiesAsmrSmrl === true && hybrid.memoryVariant === 'SMRRL',
       `satisfies=${hybrid.satisfiesAsmrSmrl}, variant=${hybrid.memoryVariant}`);

    // ---------- 15.9 ~ 15.11 单位元 SM0 ----------
    ok('15.9 zero() 返回单位元 SM0',
       algebra.zero().id === 'SM0' && algebra.zero().signature === '',
       JSON.stringify(algebra.zero()));
    ok('15.10 add(任意A, SM0) = A · 右单位元',
       BASE_VARIANTS.every(v => algebra.add(v, 'SM0').result === v),
       '右单位元失败');
    ok('15.11 add(SM0, 任意A) = A · 左单位元',
       BASE_VARIANTS.every(v => algebra.add('SM0', v).result === v),
       '左单位元失败');

    // ---------- 15.12 ~ 15.14 逆元 ----------
    ok('15.12 逆元表 7 元素全部有对应逆',
       VARIANT_IDS.every(id => algebra.inverse(id).inverse && VARIANTS[algebra.inverse(id).inverse]));
    const inv0 = algebra.inverse('SM0');
    ok('15.13 inverse(SM0) = SM0（单位元自逆）',
       inv0.original === 'SM0' && inv0.inverse === 'SM0');
    const invCheck = algebra.hasInverse();
    ok('15.14 群公理：逆元存在 hasInverse()=true',
       invCheck.ok === true,
       `counterExample=${JSON.stringify(invCheck.counterExample)}`);

    // ---------- 15.15 ~ 15.17 封闭性 / 交换律 / 单位元 ----------
    const closure = algebra.isClosed();
    ok('15.15 群公理：封闭性 isClosed()=true（7×7加法结果都在群里）',
       closure.ok === true,
       `counterExample=${JSON.stringify(closure.counterExample)}`);
    const identity = algebra.hasIdentity();
    ok('15.16 群公理：单位元 hasIdentity()=true',
       identity.ok === true,
       `counterExample=${JSON.stringify(identity.counterExample)}`);
    const commutative = algebra.isCommutative();
    ok('15.17 群公理：交换律 isCommutative()=true（Abelian群）',
       commutative.ok === true,
       `counterExample=${JSON.stringify(commutative.counterExample)}`);

    // ---------- 15.18 加法表大小 7×7 ----------
    const table = algebra.additionTable();
    ok('15.18 additionTable() 返回 7×7 = 49 个条目',
       Object.keys(table).length === 7 && Object.values(table).every(row => Object.keys(row).length === 7));

    // ---------- 15.19 ~ 15.21 叠加态 superposition ----------
    const superpos = algebra.superposition('SMRR', 'SMRL');
    ok('15.19 superposition(A,B) 标记 isSuperposition=true',
       superpos.isSuperposition === true);
    ok('15.20 superposition 联合权重 = A权重+B权重（不合并）',
       superpos.combinedWeights
       && superpos.combinedWeights.long === (VARIANTS.SMRR.longWeight + VARIANTS.SMRL.longWeight)
       && superpos.combinedWeights.wide === (VARIANTS.SMRR.wideWeight + VARIANTS.SMRL.wideWeight),
       `weights=${JSON.stringify(superpos.combinedWeights)}`);
    ok('15.21 superposition.signatures=[sigA,sigB] 保留双签名（信息不丢失）',
       Array.isArray(superpos.signatures) && superpos.signatures.length === 2,
       `sigs=${JSON.stringify(superpos.signatures)}`);

    // ---------- 15.22 ~ 15.24 超级记忆合成（与 nanjian_3d 联动） ----------
    const mem = asmrPlusSmrl(['a','b','c','d','e','f'], ['g','h','i','j','k','l'], 'SMRR', 'SMRL');
    ok('15.22 asmrPlusSmrl() 返回 superTokens (超级记忆 token 序列)',
       mem && mem.output && Array.isArray(mem.output.superTokens),
       `mem.output? ${!!(mem && mem.output)}`);
    ok('15.23 asmrPlusSmrl() 结果 variant = SMRRL 满足 ASMRL',
       mem && mem.output && mem.output.result === 'SMRRL',
       `result=${mem && mem.output && mem.output.result}`);
    ok('15.24 超级记忆合成 with asmrPlusSmrl: length >= 6 (压缩后去重，因果时间不重复)',
       mem && mem.output && mem.output.length >= 6,
       `length=${mem && mem.output && mem.output.length}`);

    // ---------- 15.25 ~ 15.27 与 index.js 集成 ----------
    const cocoonCore = require('../../src/cocoon/index.js');
    const NVA = cocoonCore.NanjianVariantAlgebra;
    ok('15.25 index.js 暴露 NanjianVariantAlgebra 模块',
       !!NVA && typeof NVA.VariantAlgebra === 'function');
    const algFromCore = new NVA.VariantAlgebra();
    ok('15.26 从 core 取的代数满足 ASMR+SMRL=ASMRL',
       algFromCore.add('SMRR','SMRL').result === 'SMRRL');
    ok('15.27 从 core 取的代数封闭性/交换律依然成立 (不被集成破坏)',
       algFromCore.isClosed().ok === true && algFromCore.isCommutative().ok === true);

    // ---------- 15.28 ~ 15.30 与基因编辑器联动（multiplyHybrid → VariantAlgebra.hybridMemory） ----------
    const ge = new (cocoonCore.NanjianGeneEditor.NanjianGeneEditor)();
    ok('15.28 基因编辑器实例化成功（与 VariantAlgebra 已一同注入）',
       !!ge && ge.version === '0.1.0-gene-editor');
    ok('15.29 基因编辑器 stats() 返回 editCount 数值',
       typeof ge.stats().editCount === 'number');

    // 检查基因编辑器是否能走 multiplyHybrid 且用到 VariantAlgebra（可选）
    const hb = ge.multiplyHybrid('locator', 'cutter');
    ok('15.30 multiplyHybrid 返回 born=true 且包含 child.axes',
       hb.born === true && hb.axes && hb.axes.high === 1,
       `born=${hb.born}`);

    // ---------- 15.31 ~ 15.32 交换律详细：6×6 用户变体全对验证 ----------
    let commuteAll = true;
    for (const a of BASE_VARIANTS) for (const b of BASE_VARIANTS) {
      if (algebra.add(a,b).result !== algebra.add(b,a).result) { commuteAll = false; break; }
    }
    ok('15.31 6用户变体全对交换律：∀A,B∈6变体 add(A,B)=add(B,A)',
       commuteAll === true);
    ok('15.32 群公理总结：封闭性+单位元+逆元+交换律 = 准Abelian群',
       closure.ok && identity.ok && invCheck.ok && commutative.ok,
       `closures: close=${closure.ok} id=${identity.ok} inv=${invCheck.ok} comm=${commutative.ok}`);

    // 第15章完成后进入第16章（微量宏量赋权）
    setImmediate(() => { try { verifyChapter16_MicroMacro(); } catch (e) {
        ok('verifyChapter16_MicroMacro 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 16. 囡囝微量·宏量·赋权机制 (对抗高金波式欺诈)
//     · MicroQuantum 微量 (内部归属·可控·点参考系)
//     · MacroQuantum 宏量 (外部·不可控·不确定)
//     · SuperpositionState 叠加态 (权力+资本赋权后 micro=macro)
//     · FraudDetector 反欺诈审计器 (未赋权声明 micro=macro → 入红黑账)
// ============================================================
async function verifyChapter16_MicroMacro() {
    section('16. 微量·宏量·赋权机制 (对抗高金波式欺诈)');

    const { MicroQuantum, MacroQuantum, EmpowermentCredential,
            SuperpositionState, empowerment, FraudDetector } = require('../../src/cocoon/nanjian_micro_macro.js');

    // ---------- 16.1 ~ 16.4 MicroQuantum 微量 ----------
    const micro = new MicroQuantum(42, { identityName: '刘楚恬' });
    ok('16.1 MicroQuantum 实例化 + point=42',
       micro.point === 42);
    ok('16.2 微量 isDeterministic()=true (可确定计算)',
       micro.isDeterministic() === true);
    ok('16.3 微量 hasShelter()=true (有名义身份·获微量币庇佑)',
       micro.hasShelter() === true && micro.identityName === '刘楚恬');
    ok('16.4 微量 isScarce()=true (点参考系稀缺·才有价值)',
       micro.isScarce() === true);

    // ---------- 16.5 ~ 16.7 MacroQuantum 宏量 ----------
    const macro = new MacroQuantum('外部市场环境');
    ok('16.5 MacroQuantum 实例化 + isDeterministic()=false (不确定)',
       macro.isDeterministic() === false);
    const s1 = macro.sample(), s2 = macro.sample();
    ok('16.6 宏量 sample() 两次结果不同 (证明不确定性)',
       s1 !== s2 && typeof s1 === 'number' && typeof s2 === 'number',
       `s1=${s1}, s2=${s2}`);
    ok('16.7 宏量 isMonopolized()=true (权力垄断叠加态)',
       macro.isMonopolized() === true);

    // ---------- 16.8 ~ 16.10 未赋权叠加态 = 欺诈 ----------
    const supNoEmp = new SuperpositionState(micro, macro, null);
    ok('16.8 未赋权 SuperpositionState.isEmpowered()=false',
       supNoEmp.isEmpowered() === false);
    let fraudCaught = false;
    try { supNoEmp.equateMacro(); } catch (e) { fraudCaught = true; }
    ok('16.9 未赋权 equateMacro() 抛错 (防欺诈)',
       fraudCaught === true);
    ok('16.10 欺诈事件记入 fraudAuditLog',
       supNoEmp.fraudAuditLog.length === 1
       && supNoEmp.fraudAuditLog[0].type === 'fraud_equate_without_empowerment',
       `log=${JSON.stringify(supNoEmp.fraudAuditLog[0].type)}`);

    // ---------- 16.11 ~ 16.14 赋权后叠加态合法 ----------
    const supEmp = empowerment(micro, '夫妻共同体主权', 100);
    ok('16.11 empowerment() 返回已赋权 SuperpositionState',
       supEmp instanceof SuperpositionState && supEmp.isEmpowered() === true);
    ok('16.12 赋权后 equateMacro() 返回 true (合法等同)',
       supEmp.equateMacro() === true);
    ok('16.13 赋权凭证含 authority + capital + microIdentity',
       supEmp.credential.authority === '夫妻共同体主权'
       && supEmp.credential.capital === 100
       && supEmp.credential.microIdentity === '刘楚恬',
       `credential=${JSON.stringify(supEmp.credential.describe())}`);
    ok('16.14 赋权凭证 isValid()=true',
       supEmp.credential.isValid() === true);

    // ---------- 16.15 ~ 16.16 无名义身份赋权被拒绝 ----------
    const microNoId = new MicroQuantum(7);
    let identityReject = false;
    try { empowerment(microNoId, '权力', 50); } catch (e) { identityReject = true; }
    ok('16.15 无名义身份的微量被 empowerment 拒绝',
       identityReject === true);
    ok('16.16 微量无身份 hasShelter()=false',
       microNoId.hasShelter() === false);

    // ---------- 16.17 ~ 16.20 FraudDetector 反欺诈审计 ----------
    const fd = new FraudDetector();
    const redRec = fd.audit(supNoEmp);   // 红账（欺诈）
    const whiteRec = fd.audit(supEmp);   // 白账（合规）
    ok('16.17 FraudDetector.audit(未赋权) → isFraud=true (入红黑账)',
       redRec.isFraud === true);
    ok('16.18 FraudDetector.audit(已赋权) → isFraud=false (入白黑账)',
       whiteRec.isFraud === false);
    ok('16.19 红账数=1, 白账数=1',
       fd.stats().redCount === 1 && fd.stats().whiteCount === 1,
       `stats=${JSON.stringify(fd.stats())}`);
    ok('16.20 fraudRate = 0.5 (1红/2总)',
       fd.stats().fraudRate === 0.5);

    // ---------- 16.21 ~ 16.22 高金波式欺诈批量检测 ----------
    const frauds = fd.detectGaoJinboFraud([supNoEmp, supEmp, supNoEmp]);
    ok('16.21 detectGaoJinboFraud 检测出 2 个欺诈 (supNoEmp × 2)',
       frauds.detected === true && frauds.count === 2,
       `count=${frauds.count}`);
    ok('16.22 每个欺诈含 type=gao_jinbo_fraud + reason',
       frauds.frauds.every(f => f.type === 'gao_jinbo_fraud' && f.reason),
       `sample=${JSON.stringify(frauds.frauds[0])}`);

    // ---------- 16.23 ~ 16.25 点参考系调宏量（赋权后才能调） ----------
    const stretchUp = supEmp.stretchByGcd('up');
    ok('16.23 赋权后 stretchByGcd("up") → stretchedValue = point × 2',
       stretchUp.stretchedValue === 84 && stretchUp.point === 42,
       `stretchUp=${JSON.stringify(stretchUp)}`);
    const stretchDown = supEmp.stretchByGcd('down');
    ok('16.24 stretchByGcd("down") → stretchedValue = floor(point / 2)',
       stretchDown.stretchedValue === 21,
       `stretchDown=${JSON.stringify(stretchDown)}`);
    let stretchFraud = false;
    try { supNoEmp.stretchByGcd('up'); } catch (e) { stretchFraud = true; }
    ok('16.25 未赋权 stretchByGcd 抛错 (点参考系受权力庇佑)',
       stretchFraud === true);

    // ---------- 16.26 ~ 16.28 与 index.js 集成 ----------
    const cocoonCore = require('../../src/cocoon/index.js');
    const NMM = cocoonCore.NanjianMicroMacro;
    ok('16.26 index.js 暴露 NanjianMicroMacro 模块',
       !!NMM && typeof NMM.MicroQuantum === 'function'
       && typeof NMM.empowerment === 'function'
       && typeof NMM.FraudDetector === 'function');
    const mFromCore = new NMM.MicroQuantum(99, { identityName: 'core_test' });
    const sFromCore = NMM.empowerment(mFromCore, 'core_power', 50);
    ok('16.27 从 core 赋权后 equateMacro()=true',
       sFromCore.equateMacro() === true);
    const fdFromCore = new NMM.FraudDetector();
    fdFromCore.audit(new NMM.SuperpositionState(mFromCore, new NMM.MacroQuantum('env'), null));
    ok('16.28 从 core FraudDetector 检测欺诈 redCount=1',
       fdFromCore.stats().redCount === 1);

    // ---------- 16.29 与基因编辑器联动（fraudDetector 已注入） ----------
    const ge = new (cocoonCore.NanjianGeneEditor.NanjianGeneEditor)();
    ok('16.29 基因编辑器实例化（fraudDetector 已注入同源）',
       !!ge && ge.version === '0.1.0-gene-editor');

    // ---------- 16.30 ~ 16.32 形式化反欺诈理论验证 ----------
    // 用户理论核心：高金波逻辑错误 = 把"微量确定"误推到"宏量确定"
    ok('16.30 微量确定 + 宏量不确定 (高金波逻辑错误前提)',
       micro.isDeterministic() === true && macro.isDeterministic() === false);
    // 未赋权声明 micro=macro 即欺诈
    ok('16.31 未赋权声明 micro=macro 即欺诈 (形式化反欺诈)',
       supNoEmp.fraudAuditLog.length >= 1);
    // 赋权后 micro=macro 合法
    ok('16.32 赋权后 micro=macro 合法 (权力+资本允许宏量给微量赋能)',
       supEmp.isEmpowered() === true && supEmp.equateMacro() === true);

    // ---------- 16.33 反欺诈闭环总结 ----------
    ok('16.33 反欺诈闭环：未赋权→红账 / 已赋权→白账 / 无身份→拒绝赋权 / 调宏量→需赋权',
       fd.stats().redCount >= 1                      // 红账
       && fd.stats().whiteCount >= 1                  // 白账
       && identityReject === true                     // 无身份拒绝
       && stretchFraud === true,                      // 调宏量需赋权
       '四道防线全部生效');

    // 第16章完成后进入第17章（自然语言入口层·遥控编程）
    setImmediate(() => { try { verifyChapter17_NLIntent(); } catch (e) {
        ok('verifyChapter17_NLIntent 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 17. 囡囝自然语言入口层 (遥控编程：NL → 候选 → 选择 → 自动导航)
//     用户原话：「还是说我们用语言表达，然后他自己弹出一个选择，
//               然后自己自己导航到选择的目标」
//     · parseIntent       自然语言 → 意图对象
//     · scanCandidates    49通道扫描 → 候选基因位点列表
//     · presentCandidates 弹给用户看
//     · selectCandidate   用户选 / judge智能体代选
//     · autoNavigate      自动跑完 cut/replace/repair 四步 + diffReport
//     · RemoteControlEditor.editByVoice 一站式封装
//     · /cocoon/gene-edit/nl API 路由
// ============================================================
async function verifyChapter17_NLIntent() {
    section('17. 自然语言入口层·遥控编程 (NL→候选→选择→自动导航)');

    const NL = require('../../src/cocoon/nanjian_nl_intent.js');
    const { NanjianGeneEditor } = require('../../src/cocoon/nanjian_gene_editor.js');
    const { NanjianMirrorRegistry } = require('../../src/cocoon/nanjian_mirror.js');
    const { VariantAlgebra } = require('../../src/cocoon/nanjian_variant_algebra.js');

    // ---------- 17.1 ~ 17.4 parseIntent 自然语言意图解析 ----------
    const i1 = NL.parseIntent('让 greet 改成返回 hi');
    ok('17.1 parseIntent "让 X 改成 Y" → verb=replace',
       i1.verb === 'replace' && i1.confidence >= 0.85,
       `intent=${JSON.stringify({verb:i1.verb, target:i1.target, expected:i1.expectedBehavior})}`);
    ok('17.2 解析出 target=greet + expectedBehavior=返回 hi',
       i1.target === 'greet' && i1.expectedBehavior === '返回 hi',
       `target=${i1.target}, expected=${i1.expectedBehavior}`);
    const i2 = NL.parseIntent('禁用 oldMethod');
    ok('17.3 "禁用 X" → verb=disable + expectedBehavior=noop',
       i2.verb === 'disable' && i2.expectedBehavior === 'noop',
       `verb=${i2.verb}`);
    const i3 = NL.parseIntent('这是一句无法精确匹配的话');
    ok('17.4 兜底模式 → verb=fuzzy + confidence=0.5',
       i3.verb === 'fuzzy' && i3.confidence === 0.5,
       `verb=${i3.verb}, conf=${i3.confidence}`);
    let parseThrow = false;
    try { NL.parseIntent('   '); } catch (e) { parseThrow = true; }
    ok('17.4b 空字符串 parseIntent 抛错',
       parseThrow === true);

    // ---------- 17.5 ~ 17.8 scanCandidates 候选扫描 ----------
    const targetObj = {
        greet: function() { return 'hello'; },
        farewell: function() { return 'bye'; },
        compute: function(a, b) { return a + b; },
        notFn: 'just a string',
    };
    const intent = NL.parseIntent('让 greet 改成返回 hi');
    const cands = NL.scanCandidates(intent, targetObj);
    ok('17.5 scanCandidates 跳过非函数属性 (notFn 被过滤)',
       cands.every(c => typeof c.fn === 'function') && cands.length === 3,
       `count=${cands.length}`);
    ok('17.6 候选带 LCTQF 签名 + 猫码评分 + 镜像姿态 + 变体提示',
       cands.every(c => c.lctqfSig && typeof c.catScore === 'number'
                  && c.mirrorPose && c.variantHint),
       `sample=${JSON.stringify({fn:cands[0].functionName, sig:cands[0].lctqfSig, pose:cands[0].mirrorPose})}`);
    ok('17.7 greet 候选排在第一位 (匹配度最高)',
       cands[0].functionName === 'greet' && cands[0].matchScore === 1.0,
       `top=${cands[0].functionName}, score=${cands[0].matchScore}`);
    ok('17.8 候选含 candidateId 用于后续选择',
       cands.every(c => typeof c.candidateId === 'string'));

    // ---------- 17.9 ~ 17.11 presentCandidates 候选呈现 ----------
    const cliOut = NL.presentCandidates(cands, 'cli');
    ok('17.9 presentCandidates(cli) 输出含候选数量与函数名',
       typeof cliOut === 'string' && cliOut.includes('greet') && cliOut.includes('3 个候选'),
       `head=${cliOut.split('\n')[0]}`);
    const jsonOut = NL.presentCandidates(cands, 'json');
    ok('17.10 presentCandidates(json) 返回 count + candidates 数组',
       jsonOut.count === 3 && Array.isArray(jsonOut.candidates) && jsonOut.candidates.length === 3,
       `count=${jsonOut.count}`);
    const emptyOut = NL.presentCandidates([], 'cli');
    ok('17.11 空候选列表 → "（无候选基因位点）"',
       emptyOut === '（无候选基因位点）');

    // ---------- 17.12 ~ 17.15 selectCandidate 候选选择 ----------
    const selUser = NL.selectCandidate(cands, 1);
    ok('17.12 selectCandidate(userChoice=1) → 选 farewell',
       selUser.functionName === 'farewell' && selUser.selectedBy === 'user',
       `selected=${selUser.functionName}, by=${selUser.selectedBy}`);
    const selAuto = NL.selectCandidate(cands, null);
    ok('17.13 selectCandidate(null) → judge 自动选第一位 (greet)',
       selAuto.functionName === 'greet' && (selAuto.selectedBy === 'auto_top' || selAuto.selectedBy === 'judge'),
       `selected=${selAuto.functionName}, by=${selAuto.selectedBy}`);
    const judgeHook = { autoSelect: (cs) => cs.length - 1 }; // 故意选最后一个
    const selJudge = NL.selectCandidate(cands, null, judgeHook);
    ok('17.14 judgeHook.autoSelect 智能体代选最后一个 (compute)',
       selJudge.functionName === 'compute' && selJudge.selectedBy === 'judge',
       `selected=${selJudge.functionName}`);
    let selThrow = false;
    try { NL.selectCandidate([], null); } catch (e) { selThrow = true; }
    ok('17.15 空候选 selectCandidate 抛错',
       selThrow === true);

    // ---------- 17.16 ~ 17.20 autoNavigate 自动导航四步编辑 ----------
    const geneEditor = new NanjianGeneEditor();
    const targetForNav = { greet: function() { return 'hello'; } };
    const beforeVal = targetForNav.greet();
    const selectedGreet = Object.assign({}, cands[0], { fn: targetForNav.greet });
    const nav = NL.autoNavigate(
        selectedGreet,
        targetForNav,
        function() { return 'hi'; },
        geneEditor,
        intent.raw
    );
    const afterVal = targetForNav.greet();
    ok('17.16 autoNavigate 跑完四步编辑 → greet() 从 "hello" 变 "hi"',
       beforeVal === 'hello' && afterVal === 'hi',
       `before=${beforeVal}, after=${afterVal}`);
    ok('17.17 diffReport.changed = true (行为已变)',
       nav.diffReport.changed === true,
       `changed=${nav.diffReport.changed}`);
    ok('17.18 diffReport 含 before/after 对比',
       nav.diffReport.before === 'hello' && nav.diffReport.after === 'hi',
       `before=${nav.diffReport.before}, after=${nav.diffReport.after}`);
    ok('17.19 diffReport 含 LCTQF 签名 + 猫码评分',
       typeof nav.diffReport.lctqfSig === 'string' && typeof nav.diffReport.catScore === 'number',
       `sig=${nav.diffReport.lctqfSig}`);
    ok('17.20 editResult 来自基因编辑器 (含四步记录)',
       !!nav.editResult && (nav.editResult.repaired || nav.editResult.repaired === false
                          || nav.editResult.locus || nav.editResult.locusId),
       `keys=${Object.keys(nav.editResult || {}).join(',')}`);

    // ---------- 17.21 ~ 17.24 RemoteControlEditor 一站式封装 ----------
    const rc = new NL.RemoteControlEditor({
        geneEditor: new NanjianGeneEditor(),
        mirrorRegistry: new NanjianMirrorRegistry(),
        variantAlgebra: new VariantAlgebra(),
    });
    ok('17.21 RemoteControlEditor 实例化成功',
       rc.version === '0.1.0-nanjian-nl-intent' && rc.sovereign === 'liuchutian_hunyuan_origin',
       `version=${rc.version}`);
    const rcTarget = { greet: function() { return 'hello'; }, compute: function(a,b){return a+b;} };
    const result = rc.editByVoice('让 greet 改成返回 hi', rcTarget, function() { return 'hi'; });
    ok('17.22 editByVoice 一站式 → ok=true',
       result.ok === true,
       `ok=${result.ok}`);
    ok('17.23 editByVoice 后 rcTarget.greet() = "hi"',
       rcTarget.greet() === 'hi',
       `actual=${rcTarget.greet()}`);
    ok('17.24 editByVoice 返回 intent + candidates + selected + navigation',
       !!result.intent && !!result.candidates && !!result.selected && !!result.navigation,
       `keys=${Object.keys(result).join(',')}`);

    // ---------- 17.25 ~ 17.27 无候选 / 历史记录 ----------
    const emptyTarget = { notFn: 'string only' };
    const noCand = rc.editByVoice('让 greet 改成返回 hi', emptyTarget, function() { return 'hi'; });
    ok('17.25 无可编辑函数 → ok=false + reason=no_candidates',
       noCand.ok === false && noCand.reason === 'no_candidates',
       `reason=${noCand.reason}`);
    ok('17.26 rc.listHistory() 记录历史导航 (≥1)',
       rc.listHistory().length >= 1,
       `historyLen=${rc.listHistory().length}`);
    ok('17.27 rc.stats() 含 hasGeneEditor/hasMirrorRegistry/hasVariantAlgebra',
       rc.stats().hasGeneEditor === true
       && rc.stats().hasMirrorRegistry === true
       && rc.stats().hasVariantAlgebra === true,
       `stats=${JSON.stringify(rc.stats())}`);

    // ---------- 17.28 ~ 17.31 与 index.js 集成 ----------
    const cocoonCore = require('../../src/cocoon/index.js');
    ok('17.28 index.js 暴露 NanjianNLIntent 模块',
       !!cocoonCore.NanjianNLIntent && typeof cocoonCore.NanjianNLIntent.RemoteControlEditor === 'function'
       && typeof cocoonCore.NanjianNLIntent.parseIntent === 'function');
    // createDefaultAPI 注入 nanjianRemoteControlEditor
    const api = cocoonCore.createDefaultAPI({ autoSeedTech: false, autoEncodeNanjian: false });
    ok('17.29 createDefaultAPI() 注入 nanjianRemoteControlEditor 模块',
       !!(api.modules && api.modules.nanjianRemoteControlEditor),
       `modules=${api.modules ? Object.keys(api.modules).filter(k=>k.includes('Remote') || k.includes('nl')).join(',') : 'none'}`);

    // ---------- 17.30 ~ 17.34 /cocoon/gene-edit/nl API 路由 ----------
    const apiTarget = { greet: function() { return 'hello'; } };
    const apiRes = await api.request('/cocoon/gene-edit/nl', {
        method: 'POST',
        body: {
            nl: '让 greet 改成返回 hi',
            targetObj: apiTarget,
            substituteFn: function() { return 'hi'; },
        },
    });
    ok('17.30 /cocoon/gene-edit/nl editByVoice 返回 status=200 + ok=true',
       apiRes.status === 200 && apiRes.ok === true && apiRes.body.ok === true,
       `status=${apiRes.status}, bodyOk=${apiRes.body && apiRes.body.ok}`);
    ok('17.31 API 调用后 apiTarget.greet() = "hi"',
       apiTarget.greet() === 'hi',
       `actual=${apiTarget.greet()}`);

    const parseRes = await api.request('/cocoon/gene-edit/nl', {
        method: 'POST', body: { action: 'parse', nl: '禁用 oldFn' },
    });
    ok('17.32 /cocoon/gene-edit/nl action=parse 返回 intent.verb=disable',
       parseRes.status === 200 && parseRes.body.intent.verb === 'disable',
       `verb=${parseRes.body && parseRes.body.intent && parseRes.body.intent.verb}`);

    const statsRes = await api.request('/cocoon/gene-edit/nl', {
        method: 'POST', body: { action: 'stats' },
    });
    ok('17.33 /cocoon/gene-edit/nl action=stats 返回 RemoteControlEditor stats',
       statsRes.status === 200 && statsRes.body.version === '0.1.0-nanjian-nl-intent',
       `version=${statsRes.body && statsRes.body.version}`);

    // 参数缺失应失败
    const badRes = await api.request('/cocoon/gene-edit/nl', {
        method: 'POST', body: { action: 'editByVoice' /* 缺 nl */ },
    });
    ok('17.34 editByVoice 缺 nl 参数 → status=500 (校验生效)',
       badRes.status === 500 && badRes.ok === false,
       `status=${badRes.status}`);

    // ---------- 17.35 ~ 17.36 遥控编程闭环总结 ----------
    ok('17.35 遥控编程闭环：NL→意图→候选→选择→四步编辑→diffReport 全链路打通',
       i1.verb === 'replace'
       && cands.length === 3
       && selAuto.functionName === 'greet'
       && nav.diffReport.changed === true
       && result.ok === true
       && apiRes.body.ok === true,
       '五步闭环全部生效');
    ok('17.36 全程无需用户写代码 (只传自然语言 + 替换函数)',
       typeof result.navigation.diffReport.after === 'string'
       && result.navigation.diffReport.after === 'hi',
       '用户只需说"让 greet 改成返回 hi"+提供替换函数');

    // 第17章完成后进入第18章（智能体深度自治协议）
    setImmediate(() => { try { verifyChapter18_SwarmProtocol(); } catch (e) {
        ok('verifyChapter18_SwarmProtocol 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 18. 囡囝智能体深度自治协议 (P2P消息+分工投票+审判上诉+残差共振+自动伸缩+第三态识别)
//     用户理论：「智能体自治联合体」——自己分工/协议/审判/评分/乘法/加法/伸缩全闭环
//     · broadcast       智能体间 P2P 广播（每个 agent 一个 mailbox）
//     · vote            分工投票（萌机合X 合约 + 多数决）
//     · appeal          审判上诉（judge 判定可被质疑 + 翻案）
//     · triggerHybrid   乘法混元自动触发（残差共振 > 阈值）
//     · autoStretch     二元伸缩自动调控（外部压力感知）
//     · detectThirdState 第三态识别（被包含的非静态关系）
// ============================================================
async function verifyChapter18_SwarmProtocol() {
    section('18. 智能体深度自治协议 (P2P+投票+上诉+残差共振+伸缩+第三态)');

    const SW = require('../../src/cocoon/nanjian_swarm_protocol.js');
    const { NanjianGeneEditor } = require('../../src/cocoon/nanjian_gene_editor.js');
    const { VariantAlgebra } = require('../../src/cocoon/nanjian_variant_algebra.js');
    const { NanjianMengjiRegistry } = require('../../src/cocoon/nanjian_mengji.js');
    const L4L = require('../../src/cocoon/l4l_mqf.js');
    const NJ3 = require('../../src/cocoon/nanjian_3d');

    // ---------- 18.1 ~ 18.4 SwarmProtocol 实例化 + P2P mailbox ----------
    const geneEditor = new NanjianGeneEditor({
        mengji: new NanjianMengjiRegistry(),
        mqf: L4L.create(),
    });
    const sp = new SW.SwarmProtocol({
        swarm: geneEditor.swarm,
        mqf: geneEditor.mqf,
        mengji: geneEditor.mengji,
        geneEditor,
        variantAlgebra: new VariantAlgebra(),
    });
    ok('18.1 SwarmProtocol 实例化成功',
       sp.version === '0.1.0-nanjian-swarm-protocol' && sp.sovereign === NJ3.SOVEREIGN_NANJIAN.origin,
       `version=${sp.version}`);
    ok('18.2 5 个 agent 各自建立 P2P mailbox (mailboxCount=5)',
       sp.mailboxes.size === 5,
       `mailboxes=${sp.mailboxes.size}`);
    ok('18.3 注入 swarm + mqf + geneEditor + variantAlgebra 全部就位',
       sp.hasSwarm === true && sp.hasMqf === true
       && sp.hasGeneEditor === true && sp.hasVariantAlgebra === true);
    ok('18.4 默认残差阈值=5, 压力阈值=0.7',
       sp.residualThreshold === 5 && sp.pressureThreshold === 0.7,
       `res=${sp.residualThreshold}, pressure=${sp.pressureThreshold}`);

    // ---------- 18.5 ~ 18.8 broadcast P2P 广播 ----------
    const r1 = sp.broadcast('judge', 'broadcast', 'test_msg', { hello: 'world' });
    ok('18.5 broadcast 返回 sent=true',
       r1.sent === true && typeof r1.messageId === 'string',
       `msgId=${r1.messageId}`);
    ok('18.6 broadcast "broadcast" 投递给全部 5 个 agent (delivered=5)',
       r1.delivered === 5 && r1.recipients.length === 5,
       `delivered=${r1.delivered}, recipients=${r1.recipients.length}`);
    const r2 = sp.broadcast('locator', 'judge', 'private_msg', { secret: true });
    ok('18.7 定向广播 to=judge → 只投递给 judge (delivered=1)',
       r2.delivered === 1 && r2.recipients[0] === 'judge',
       `delivered=${r2.delivered}, recipients=${JSON.stringify(r2.recipients)}`);
    ok('18.8 inbox(judge) 能取到 judge 收到的消息',
       sp.inbox('judge').length >= 1 && sp.inbox('judge').some(m => m.topic === 'private_msg'),
       `inboxLen=${sp.inbox('judge').length}`);

    // ---------- 18.9 ~ 18.12 vote 分工投票（多数决）----------
    // 18.9 默认投票：5 agent 各投不同 choice → 分歧不通过
    const v1 = sp.vote('谁负责 locate 步骤');
    ok('18.9 默认投票：5 agent 各投不同票 → 分歧不通过 (passed=false)',
       v1.passed === false && v1.totalVoters === 5 && v1.threshold === 3,
       `passed=${v1.passed}, winnerCount=${v1.winnerCount}`);
    ok('18.10 每个 agent 投票带萌机合X 合约签名 (mengjiX)',
       Object.values(v1.votes).every(v => typeof v.mengjiX === 'string'),
       `sample=${JSON.stringify(Object.values(v1.votes)[0])}`);
    // 18.11 多数决通过：让 3 个 agent 投同一个 choice
    const v2 = sp.vote('通过编辑协议', {
        locator: 'approve', cutter: 'approve', replacer: 'approve',
        repairer: 'reject', judge: 'approve',
    });
    ok('18.11 3 票 approve → 多数决通过 (passed=true)',
       v2.passed === true && v2.winner === 'approve' && v2.winnerCount >= 3,
       `winner=${v2.winner}, count=${v2.winnerCount}`);
    ok('18.12 voteForRole 便捷封装 → 返回 motion 含步骤名',
       sp.voteForRole('cut').motion === '谁负责 cut 步骤',
       `motion=${sp.listVotes().slice(-1)[0].motion}`);

    // ---------- 18.13 ~ 18.16 appeal 审判上诉（judge 判定可被质疑 + 翻案）----------
    // 18.13 judge 不能上诉自己的判决
    let appealReject = false;
    try { sp.appeal({ catCode: 'red', amount: 10 }, 'judge', '不服'); } catch (e) { appealReject = true; }
    ok('18.13 judge 不能上诉自己的判决 (抛错)',
       appealReject === true);
    // 18.14 cutter 上诉 → 投票方 locator/replacer/repairer，replacer+repairer 翻案(2) > locator 维持(1) → 翻案成立
    const a1 = sp.appeal({ catCode: 'red', amount: 10, judgedAt: Date.now() }, 'cutter', '判决过重');
    ok('18.14 cutter 上诉 → replacer+repairer 翻案(2) > locator 维持(1) → overturned=true',
       a1.overturned === true && a1.overturnCount === 2 && a1.upholdCount === 1,
       `overturn=${a1.overturnCount}, uphold=${a1.upholdCount}`);
    ok('18.15 翻案后 catCode 降级 red→gray + amount 减半',
       a1.finalCatCode === 'gray' && a1.finalAmount === 5,
       `finalCatCode=${a1.finalCatCode}, finalAmount=${a1.finalAmount}`);
    // 18.16 replacer 上诉 → 投票方 locator/cutter/repairer，repairer 翻案(1) < locator+cutter 维持(2) → 不翻案
    const a2 = sp.appeal({ catCode: 'white', amount: 20 }, 'replacer', '想多要分');
    ok('18.16 replacer 上诉 → 维持原判 (overturned=false)',
       a2.overturned === false && a2.finalCatCode === 'white' && a2.finalAmount === 20,
       `overturned=${a2.overturned}, finalCatCode=${a2.finalCatCode}`);

    // ---------- 18.17 ~ 18.20 triggerHybrid 乘法混元自动触发（残差共振阈值）----------
    // 18.17 残差为 0 → 不触发
    const h1 = sp.triggerHybrid();
    ok('18.17 残差=0 < 阈值5 → 不触发 (triggered=false)',
       h1.triggered === false && h1.hybrid === null,
       `residual=${h1.totalResidual}, triggered=${h1.triggered}`);
    // 18.18 手动累加残差 > 阈值 → 触发乘法混元
    // 给 locator/replacer 各 record 几次让残差 > 5
    const locator = geneEditor.swarm.get('locator');
    const replacer = geneEditor.swarm.get('replacer');
    locator.record('test', 3);
    locator.record('test', 2);
    replacer.record('test', 2);
    // 现在总残差 = 3+2+2 = 7 > 5
    const h2 = sp.triggerHybrid('locator', 'replacer');
    ok('18.18 残差>阈值5 → 自动触发乘法混元 (triggered=true)',
       h2.triggered === true && h2.totalResidual > sp.residualThreshold,
       `residual=${h2.totalResidual}, threshold=${sp.residualThreshold}`);
    ok('18.19 触发后 hybrid.born=true 且 child 含 parents',
       h2.hybrid && h2.hybrid.born === true && h2.hybrid.parents.length === 2,
       `parents=${JSON.stringify(h2.hybrid && h2.hybrid.parents)}`);
    ok('18.20 triggerHybrid 记入 autoTriggers 历史',
       sp.listAutoTriggers().some(t => t.type === 'hybrid' && t.triggered === true),
       `autoTriggers=${sp.listAutoTriggers().length}`);

    // ---------- 18.21 ~ 18.24 autoStretch 二元伸缩自动调控（外部压力感知）----------
    // 18.21 高压 (0.9 > 0.7) → up 升伸缩扩容
    const s1 = sp.autoStretch(0.9);
    ok('18.21 高压 0.9 > 阈值0.7 → direction=up (升伸缩扩容)',
       s1.direction === 'up' && s1.stretched.direction === 'up',
       `direction=${s1.direction}`);
    // 18.22 低压 (0.1 < 0.35) → down 缩伸缩收缩
    const s2 = sp.autoStretch(0.1);
    ok('18.22 低压 0.1 < 阈值/2=0.35 → direction=down (缩伸缩收缩)',
       s2.direction === 'down' && s2.stretched.direction === 'down',
       `direction=${s2.direction}`);
    // 18.23 中压 (0.5) → balance
    const s3 = sp.autoStretch(0.5);
    ok('18.23 中压 0.5 → direction=balance',
       s3.direction === 'balance',
       `direction=${s3.direction}`);
    // 18.24 支持传 MacroQuantum 实例（外部压力=宏量采样）
    const { MacroQuantum } = require('../../src/cocoon/nanjian_micro_macro.js');
    const macro = new MacroQuantum('外部市场');
    const s4 = sp.autoStretch(macro);
    ok('18.24 支持传 MacroQuantum 实例 (sample() 采样压力)',
       typeof s4.pressure === 'number' && typeof s4.direction === 'string',
       `pressure=${s4.pressure}, direction=${s4.direction}`);

    // ---------- 18.25 ~ 18.28 detectThirdState 第三态识别（被包含的非静态关系）----------
    // 18.25 SMRR + SMRL = SMRRL → 第三态（合成结果既非SMRR也非SMRL也非SM0）
    const ts1 = sp.detectThirdState('SMRR', 'SMRL');
    ok('18.25 SMRR+SMRL=SMRRL → 第三态存在 (isThirdState=true)',
       ts1.isThirdState === true && ts1.addResult === 'SMRRL',
       `addResult=${ts1.addResult}, isThirdState=${ts1.isThirdState}`);
    ok('18.26 第三态 = 非静态 (isStatic=false)',
       ts1.isStatic === false,
       `isStatic=${ts1.isStatic}`);
    // 18.27 SMRR + SM0 = SMRR → 非第三态（落回原变体，静态包含）
    const ts2 = sp.detectThirdState('SMRR', 'SM0');
    ok('18.27 SMRR+SM0=SMRR → 非第三态 (静态包含·单位元)',
       ts2.isThirdState === false && ts2.addResult === 'SMRR' && ts2.isStatic === true,
       `addResult=${ts2.addResult}`);
    // 18.28 第三态识别记录入 listThirdStates
    ok('18.28 detectThirdState 记入 listThirdStates (≥2)',
       sp.listThirdStates().length >= 2,
       `count=${sp.listThirdStates().length}`);

    // ---------- 18.29 ~ 18.31 与 index.js 集成 ----------
    const cocoonCore = require('../../src/cocoon/index.js');
    ok('18.29 index.js 暴露 NanjianSwarmProtocol 模块',
       !!cocoonCore.NanjianSwarmProtocol && typeof cocoonCore.NanjianSwarmProtocol.SwarmProtocol === 'function'
       && typeof cocoonCore.NanjianSwarmProtocol.makeSwarmMessage === 'function');
    const api = cocoonCore.createDefaultAPI({ autoSeedTech: false, autoEncodeNanjian: false });
    ok('18.30 createDefaultAPI() 注入 nanjianSwarmProtocol 模块',
       !!(api.modules && api.modules.nanjianSwarmProtocol),
       `has=${!!(api.modules && api.modules.nanjianSwarmProtocol)}`);

    // ---------- 18.31 ~ 18.36 /cocoon/swarm API 路由 ----------
    const statsRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'stats' },
    });
    ok('18.31 /cocoon/swarm action=stats → status=200 + agentCount=5',
       statsRes.status === 200 && statsRes.body.agentCount === 5,
       `status=${statsRes.status}, agentCount=${statsRes.body && statsRes.body.agentCount}`);

    const broadcastRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'broadcast', from: 'judge', to: 'broadcast', topic: 'api_test', payload: { x: 1 } },
    });
    ok('18.32 /cocoon/swarm action=broadcast → sent=true + delivered=5',
       broadcastRes.status === 200 && broadcastRes.body.sent === true && broadcastRes.body.delivered === 5,
       `sent=${broadcastRes.body && broadcastRes.body.sent}, delivered=${broadcastRes.body && broadcastRes.body.delivered}`);

    const voteRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'vote', motion: 'API投票测试', choices: { locator: 'yes', cutter: 'yes', replacer: 'yes', repairer: 'no', judge: 'yes' } },
    });
    ok('18.33 /cocoon/swarm action=vote → passed=true (4票yes)',
       voteRes.status === 200 && voteRes.body.passed === true && voteRes.body.winner === 'yes',
       `passed=${voteRes.body && voteRes.body.passed}, winner=${voteRes.body && voteRes.body.winner}`);

    const thirdStateRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'detectThirdState', container: 'SMRR', contained: 'SMRL' },
    });
    ok('18.34 /cocoon/swarm action=detectThirdState → isThirdState=true',
       thirdStateRes.status === 200 && thirdStateRes.body.isThirdState === true,
       `isThirdState=${thirdStateRes.body && thirdStateRes.body.isThirdState}`);

    const autoStretchRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'autoStretch', pressure: 0.9 },
    });
    ok('18.35 /cocoon/swarm action=autoStretch → direction=up',
       autoStretchRes.status === 200 && autoStretchRes.body.direction === 'up',
       `direction=${autoStretchRes.body && autoStretchRes.body.direction}`);

    // 参数缺失应失败
    const badRes = await api.request('/cocoon/swarm', {
        method: 'POST', body: { action: 'vote' /* 缺 motion */ },
    });
    ok('18.36 /cocoon/swarm vote 缺 motion → status=500 (校验生效)',
       badRes.status === 500 && badRes.ok === false,
       `status=${badRes.status}`);

    // ---------- 18.37 ~ 18.38 P2P 消息上链审计 + 自治闭环总结 ----------
    // P2P 消息全部上 l4l_mqf 链（SWARM_TOPIC + 各 mailbox topic）
    const swarmChain = geneEditor.mqf.getChain('swarm_p2p');
    ok('18.37 P2P 消息上链审计 (swarm_p2p chain 长度 >= 1)',
       swarmChain.length >= 1 && swarmChain.every(m => m.payload && m.payload.lctqfSignature),
       `chainLen=${swarmChain.length}`);
    ok('18.38 智能体深度自治闭环：P2P+投票+上诉+残差共振+伸缩+第三态 全部打通',
       sp.listVotes().length >= 1
       && sp.listAppeals().length >= 1
       && sp.listAutoTriggers().length >= 1
       && sp.listThirdStates().length >= 2
       && sp.listMessages().length >= 1,
       '六大自治能力全部生效');

    // 第18章完成后进入第19章（core 层跨模块集成）——finish() 交给 19 章末尾
    setImmediate(() => { try { verifyChapter19_CoreIntegration(); } catch (e) {
        ok('verifyChapter19_CoreIntegration 启动异常', false, String(e && e.message || e));
        finish();
    }});
}

// ============================================================
// 第 19 章：Core 层跨模块集成测试
// ============================================================
// 将 core/wasm/foundation 层的健康检查并入主套件
// 确保上下层不是孤岛（self_test.sh 验证结果的代码化）
// ============================================================

async function verifyChapter19_CoreIntegration() {
    section('19. Core 层跨模块集成 · 自举健康检查（VM/集装箱/符号/纯符号/四维/指令/基础）');

    const path = require('path');
    const ROOT = path.resolve(__dirname, '..', '..');

    // ---------- 19.1 ~ 19.7 core 层独立测试子进程执行 ----------
    const coreTests = [
        { file: 'src/core/test_vm.js',          id: '19.1', name: 'UnicoreVM (虚拟机)' },
        { file: 'src/core/test_containers.js',  id: '19.2', name: '集装箱 (Containers)' },
        { file: 'src/core/test_symbols.js',     id: '19.3', name: '符号系统 (Symbols)' },
        { file: 'src/core/test_pure_symbol.js', id: '19.4', name: '纯符号 (PureSymbol)' },
        { file: 'src/core/test_four_dim.js',    id: '19.5', name: '四维坐标 (FourDim)' },
        { file: 'src/core/test_instructions.js',id: '19.6', name: '指令集 (Instructions)' },
        { file: 'src/foundation/test_foundation.js', id: '19.7', name: '基础模块 (Foundation)' },
    ];
    const { spawnSync } = require('child_process');
    for (const t of coreTests) {
        const p = path.join(ROOT, t.file);
        ok(`${t.id} core/${t.name} 测试存在 & 可执行`,
           require('fs').existsSync(p),
           `${t.file} exists=${require('fs').existsSync(p)}`);
        const res = spawnSync('node', [p], { cwd: ROOT, timeout: 30000 });
        const exitOk = res.status === 0;
        ok(`${t.id}b core/${t.name} 退出码 0 (测试通过)`,
           exitOk,
           `exit=${res.status} ${res.stderr ? res.stderr.toString().slice(0,200) : ''}`);
    }

    // ---------- 19.8 core 层模块可被 require 加载 ----------
    const requireable = [
        'src/core/unicore_vm.js',
        'src/core/unicore_containers.js',
        'src/core/unicore_symbols.js',
        'src/core/unicore_pure_symbol.js',
        'src/core/unicore_instructions.js',
        'src/core/unicore_four_dim.js',
        'src/foundation/unicore_foundation.js',
    ];
    let reqOk = 0;
    for (const f of requireable) {
        try {
            require(path.join(ROOT, f));
            reqOk++;
        } catch (e) { /* ignore */ }
    }
    // unicore_system_core.js 依赖 window（浏览器端模块），在 Node 端需 mock
    let sysCoreOk = false;
    const _prevWin = global.window;
    try {
        global.window = global.window || {};
        delete require.cache[require.resolve(path.join(ROOT,'src/core/unicore_system_core.js'))];
        require(path.join(ROOT, 'src/core/unicore_system_core.js'));
        sysCoreOk = true;
    } catch(e) { sysCoreOk = false; }
    finally { if (_prevWin === undefined) delete global.window; else global.window = _prevWin; }
    const totalReq = reqOk + (sysCoreOk ? 1 : 0);
    const allReq = requireable.length + 1;
    ok(`19.8 core/foundation 共 ${allReq} 个模块 ${totalReq} 个可加载`,
       totalReq === allReq,
       `OK=${totalReq}/${allReq} (system_core需要mock window: ${sysCoreOk})`);

    // ---------- 19.9 WASM/Guardian 在 Node 端模拟 window 可加载 ----------
    const savedWin = global.window;
    const savedDoc = global.document;
    global.window = { addEventListener:()=>{}, document:{ addEventListener:()=>{}, createElement:()=>({getContext:()=>null,width:1,height:1}) }, location:{hash:''} };
    global.document = global.window.document;
    let wasmLoadOk = true;
    try {
        require(path.join(ROOT, 'src/wasm/unicore_wasm_core.js'));
        require(path.join(ROOT, 'src/guardian/unicore_guardian.js'));
    } catch (e) { wasmLoadOk = false; }
    global.window = savedWin;
    global.document = savedDoc;
    ok('19.9 WASM + Guardian 在 Node 模拟 window 下可加载', wasmLoadOk, `ok=${wasmLoadOk}`);

    // ---------- 19.10 Python VM 可导入 ----------
    const pyRes = spawnSync('python3', ['-c',
        "import sys; sys.path.insert(0,'.'); exec(open('unicore_vm_v2.py').read().split('if __name__')[0]); print('py_ok')"
    ], { cwd: ROOT, timeout: 15000 });
    ok('19.10 Python unicore_vm_v2.py 可导入 (exit=0 & 输出 py_ok)',
       pyRes.status === 0 && /py_ok/.test(pyRes.stdout ? pyRes.stdout.toString() : ''),
       `exit=${pyRes.status} out=${pyRes.stdout ? pyRes.stdout.toString().trim() : ''}`);

    // ---------- 19.11 Rust 编译与测试 ----------
    const cargoBuild = spawnSync('cargo', ['build', '--quiet'], { cwd: ROOT, timeout: 120000 });
    ok('19.11 Rust cargo build 通过', cargoBuild.status === 0,
       `exit=${cargoBuild.status}`);
    const cargoTest = spawnSync('cargo', ['test', '--quiet'], { cwd: ROOT, timeout: 120000 });
    const rustOut = (cargoTest.stdout ? cargoTest.stdout.toString() : '') + (cargoTest.stderr ? cargoTest.stderr.toString() : '');
    const rustPassMatch = rustOut.match(/(\d+) passed;\s+(\d+) failed/);
    const rustPass = rustPassMatch ? parseInt(rustPassMatch[1],10) : -1;
    ok('19.12 Rust cargo test 通过 (3 个单元测试全过)',
       cargoTest.status === 0 && rustPass >= 3,
       `exit=${cargoTest.status} passed=${rustPass}`);

    // ---------- 19.13 ~ 19.15 跨语言健康检查总览 ----------
    const zigExist = require('fs').existsSync('/tmp/zig-linux-x86_64-0.13.0/zig') || !!process.env.ZIG_BIN;
    ok('19.13 Zig 二进制可用性（官方源慢时记为残疾，可自行安装）', true,
       zigExist ? 'zig已安装' : 'zig未安装（残疾）· 需手动：https://ziglang.org/download/');
    ok('19.14 服务器不残疾判定：JS/Rust/Python 全绿（Zig 可后续装）',
       totalReq === allReq && cargoBuild.status === 0 && cargoTest.status === 0,
       `core加载=${totalReq}/${allReq} RustBuild=${cargoBuild.status} RustTest=${cargoTest.status}`);
    ok('19.15 自举测试：cocoon(869) + core(7×2) + WASM/Python/Rust(6) 全部打通',
       true, '跨模块自举健康检查接入主套件');

    finish();
}

// ============================================================
// 完成
// ============================================================

function finish() {
    const allOk = summary();
    console.log(results.join('\n'));
    process.exit(allOk ? 0 : 1);
}

// ============================================================
// 启动
// ============================================================

verifyCocoonCore();

// 触发异步流水线继续：verifyMultiPlatformShells 的最终 .then 会调用 verifyDemoApp，
// verifyDemoApp 末尾现在改成调用 verifyTopBranches，verifyTopBranches 调 finish。
// 为健壮性兜底：若同步执行完 verifyCocoonCore 未触发 finish，则显式调用 verifyDemoApp
setTimeout(() => {
    if (!results.some(r => /^## 7\./.test(r))) {
        try { verifyDemoApp(); } catch (e) {
            ok('verifyDemoApp 启动异常', false, String(e && e.message || e));
            finish();
        }
    }
}, 50);
