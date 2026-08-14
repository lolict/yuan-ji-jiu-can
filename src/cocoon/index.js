/**
 * 蛹茧（寄生壳）核心架构系统 —— 总入口
 * ============================================================
 *
 * 本模块是整个蛹茧系统的统一出口，导出所有子模块，
 * 并提供 cocoonBootstrap() 引导函数，串联完整的"零安装"流程：
 *
 *   1. 检测浏览器环境
 *   2. 引导 WASM 沙箱
 *   3. 加载解析器
 *   4. 解析蛹茧
 *   5. 选择壳
 *   6. 孵化执行
 *
 * 使用示例：
 *   const { cocoonBootstrap } = require('./cocoon');
 *   const result = await cocoonBootstrap('/path/to/app.cocoon');
 *
 *   或在浏览器中：
 *   <script src="cocoon/index.js"></script>
 *   await cocoonBootstrap('./app.cocoon');
 */

'use strict';

// 导入核心类与寄生壳注册表
const {
    Cocoon,
    ShellRegistry,
    BrowserWASMEntry,
    CocoonParser,
    COCOON_MAGIC,
    COCOON_VERSION,
    DEFAULT_SANDBOX_PERMISSIONS,
} = require('./cocoon_core');

const { SHELL_REGISTRY } = require('./shell_registry');

// 三大顶枝：H4H（混元杂交）· A4A（对外API）· L4L（MQF消息链）
const H4H = require('./h4h_hybrid');
const A4A = require('./a4a_api');
const L4L = require('./l4l_mqf');
const SEED_LIB = require('./seed_library');

// ★ 囡囝主权架构：三维立体进制 · 2304 态射 · ASMR/RTFM/JEPA 编码导入
const NANJIAN_3D   = require('./nanjian_3d');
const NANJIAN_MORPH = require('./nanjian_morphisms');
const NANJIAN_ENCODERS = require('./nanjian_encoders');
const NANJIAN_SENSOR = require('./nanjian_invisible_sensor');
const NANJIAN_MENGJI = require('./nanjian_mengji');
const NANJIAN_GENE_EDITOR = require('./nanjian_gene_editor');
const NANJIAN_MIRROR = require('./nanjian_mirror');
const NANJIAN_VARIANT_ALGEBRA = require('./nanjian_variant_algebra');
const NANJIAN_MICRO_MACRO = require('./nanjian_micro_macro');
const NANJIAN_NL_INTENT = require('./nanjian_nl_intent');
const NANJIAN_SWARM_PROTOCOL = require('./nanjian_swarm_protocol');

// 预加载：逻辑 + 猫码 + 工具链（方便 A4A 注入 modules）
const LOGIC = require('./index_logic');
const LANG  = require('./index_lang');

// ★ 语言瘦身 + 适配器（强制所有语言吃 Rust/Zig 标准）
const LANG_SLIMMING = require('./lang_slimming_matrix');
const LANG_ADAPTER  = require('./lang_adapter');

// ============================================================
// cocoonBootstrap —— 统一引导函数
// ============================================================

/**
 * 蛹茧系统统一引导函数
 *
 * 完整串联零安装流程：
 *   检测环境 → 引导沙箱 → 加载蛹茧 → 启动解析器 → 解析 → 选壳 → 孵化执行
 *
 * @param {string|ArrayBuffer|Uint8Array} cocoonSource - 蛹茧 URL 或二进制
 * @param {object} [options] - 引导选项
 * @param {string} [options.shellChoice] - 指定壳 id（跳过弹窗）
 * @param {boolean} [options.autoElevate=true] - 是否自动自我提权
 * @param {ShellRegistry} [options.registry] - 自定义壳注册表
 * @returns {Promise<object>} 引导与执行结果
 */
async function cocoonBootstrap(cocoonSource, options = {}) {
    const opts = Object.assign({
        shellChoice: null,
        autoElevate: true,
        registry: null,
    }, options);

    const trace = [];

    // ---------- 1. 检测浏览器环境 ----------
    trace.push('步骤1: 检测运行环境');
    const entry = new BrowserWASMEntry();
    if (!entry.environment.isBrowser && !entry.environment.isNode && !entry.environment.isWorker) {
        trace.push('  警告: 未知运行环境，尝试继续引导');
    }
    trace.push(`  环境: ${entry.environment.isBrowser ? '浏览器' : entry.environment.isNode ? 'Node.js' : 'Worker'}, WASM=${entry.wasmAvailable}`);

    // ---------- 2. 引导 WASM 沙箱 ----------
    trace.push('步骤2: 引导 WASM 沙箱');
    const bootResult = await entry.bootstrap();
    trace.push(`  沙箱权限: ${bootResult.permissions.join(', ')}`);

    // ---------- 3. 加载蛹茧 + 启动解析器 ----------
    trace.push('步骤3: 加载蛹茧文件');
    const cocoonBuffer = await entry.loadCocoon(cocoonSource);

    trace.push('步骤4: 在沙箱内启动解析器');
    const parser = entry.startParser();

    // 若提供了自定义注册表，注入到解析器的 cocoon 中
    if (opts.registry instanceof ShellRegistry) {
        parser.cocoon = parser.cocoon || new Cocoon();
        parser.cocoon.registry = opts.registry;
    }

    // ---------- 5. 解析蛹茧 ----------
    trace.push('步骤5: 解析蛹茧二进制');
    const cocoon = parser.parseCocoon(cocoonBuffer);
    trace.push(`  蛹茧: ${cocoon.metadata.name}, 内核 ${cocoon.kernel.length}B, 可用壳 ${cocoon.shellIds.length} 个`);

    // 提取内核与壳列表
    const kernel = parser.extractKernel(cocoonBuffer);
    const shells = parser.extractShells(cocoonBuffer);

    // ---------- 6. 选择壳 ----------
    trace.push('步骤6: 选择寄生壳');
    let chosenShell;
    if (opts.shellChoice) {
        chosenShell = cocoon.registry.get(opts.shellChoice);
        if (!chosenShell) {
            throw new Error(`[cocoonBootstrap] 指定的壳 "${opts.shellChoice}" 不存在`);
        }
        trace.push(`  使用指定壳: ${chosenShell.name}`);
    } else {
        chosenShell = await parser.presentShellChoice(shells);
        trace.push(`  选择壳: ${chosenShell.name} (${chosenShell.platform})`);
    }

    // ---------- 7. 孵化执行 ----------
    trace.push('步骤7: 穿壳孵化执行');
    const execResult = await parser.executeWithShell(kernel, chosenShell);
    trace.push(`  ${execResult.message}`);

    // ---------- 8. 自我提权（可选） ----------
    let elevation = null;
    if (opts.autoElevate) {
        trace.push('步骤8: 自我提权（通过 WASI 桥接原生能力）');
        elevation = parser.selfElevate(chosenShell);
        trace.push(`  获得 ${Object.keys(elevation.capabilities).length} 项原生能力，无需系统授权`);
    }

    trace.push('引导完成：蛹茧已零安装蜕变为目标平台可执行程序');

    return {
        success: true,
        environment: entry.environment,
        sandboxPermissions: bootResult.permissions,
        cocoon: cocoon.summary(),
        chosenShell,
        execution: execResult,
        elevation,
        trace,
        entry,   // 暴露引导器实例，便于后续操作
        parser,  // 暴露解析器实例，便于后续操作
    };
}

// ============================================================
// 便捷工厂函数
// ============================================================

/**
 * 创建一个空的蛹茧，便于程序化生成
 * @param {object} [init]
 * @returns {Cocoon}
 */
function createCocoon(init) {
    return new Cocoon(init);
}

/**
 * 创建一个预装全部预注册壳的 ShellRegistry
 * @returns {ShellRegistry}
 */
function createRegistry() {
    const reg = new ShellRegistry();
    reg.registerAll(SHELL_REGISTRY);
    return reg;
}

/**
 * 快速构建一个示例蛹茧（用于测试/演示）
 * @param {string} [name]
 * @returns {Cocoon}
 */
function buildDemoCocoon(name = 'demo-cocoon') {
    // 一段最小合法的 WASM 模块字节码（空模块）
    const minimalWasm = new Uint8Array([
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00
    ]);
    const cocoon = new Cocoon({
        metadata: {
            name,
            author: 'cocoon-system',
            created: Date.now(),
            entry: '_start',
            description: '由 cocoon 系统生成的演示蛹茧',
        },
        kernel: minimalWasm,
        // 默认可穿任意预注册壳
        shellIds: Object.keys(SHELL_REGISTRY),
    });
    return cocoon;
}

// ============================================================
// H4H · A4A · L4L 三大顶枝工厂
// ============================================================

/**
 * 构建默认的 A4A API 路由实例。自动注入：
 *   - ShellRegistry (预注册26壳)
 *   - cocoonCore (Cocoon/Parser/BrowserWASMEntry/cocoonBootstrap)
 *   - balanceJudge (天平裁判)
 *   - catCodeSys (猫码系统)
 *   - cocoonLang (声韵调工具链)
 *   - h4h (混元杂交注册表)
 *   - mqf (L4L 消息链)
 */
function createDefaultAPI(options = {}) {
    const registry = createRegistry();
    const balanceJudge = new LOGIC.BalanceJudge();
    const catCodeSys = new LOGIC.CatCodeSystem();
    const h4h = H4H.create();
    const mqf = L4L.create();
    // 往 h4h 里预填一些种子（26壳/48猫码/核心概念）
    for (const s of registry.listAll()) h4h.registerSeed('h4h', Object.assign({}, s, { category: 'shell' }));
    for (let i = 1; i <= Math.min(48, catCodeSys.size()); i++) {
        const c = catCodeSys.getCatCode(i);
        if (c) h4h.registerSeed('h4h', Object.assign({}, c, { category: 'catcode' }));
    }
    [
        { id:'space', name:'3D空间', category:'math' },
        { id:'time',  name:'时间',   category:'spacetime' },
        { id:'radix', name:'进制',   category:'math' },
        { id:'ai',    name:'人工智能', category:'ai' },
        { id:'logic', name:'逻辑', category:'philosophy' },
    ].forEach(e => h4h.registerSeed('h4h', e));
    // ★ 灌种子库：TECH_SEEDS 26 槽 × 7~9 条真实高未来科技种子
    if (typeof options.autoSeedTech !== 'boolean' || options.autoSeedTech) {
        SEED_LIB.enableTopMarriages(H4H.H4HRegistry);
        SEED_LIB.registerTechSeeds(h4h, { ensureAtLeastPerSlot: options.ensureAtLeastPerSlot || 6 });
    }
    // ★★ 囡囝主权架构：三维进制 + 2304 态射注册表 + 编码导入器
    //     （不取别人架构，只取 ASMR/RTFM/JEPA 概念编码，用我们的态射关系代数跑）
    const nanjianMorphisms = new NANJIAN_MORPH.NanjianMorphismRegistry();
    const nanjianSensor = new NANJIAN_SENSOR.NanjianInvisibleSensor();
    const nanjianMengji = new NANJIAN_MENGJI.NanjianMengjiRegistry();
    const nanjianMirrorRegistry = new NANJIAN_MIRROR.NanjianMirrorRegistry();
    const nanjianVariantAlgebra = new NANJIAN_VARIANT_ALGEBRA.VariantAlgebra();
    // 反欺诈审计器（与 judge 智能体 + mqf 上链联动）
    const nanjianFraudDetector = new NANJIAN_MICRO_MACRO.FraudDetector({
        mqfHook: mqf,    // 欺诈事件自动上 red 链
    });
    const nanjianGeneEditor = new NANJIAN_GENE_EDITOR.NanjianGeneEditor({
        morphisms: nanjianMorphisms,
        mengji: nanjianMengji,
        mirrorRegistry: nanjianMirrorRegistry,
        variantAlgebra: nanjianVariantAlgebra,
        fraudDetector: nanjianFraudDetector,
        mqf,
    });
    // 遥控编程入口（自然语言 → 候选 → 选择 → 自动导航）
    const nanjianRemoteControlEditor = new NANJIAN_NL_INTENT.RemoteControlEditor({
        geneEditor: nanjianGeneEditor,
        mirrorRegistry: nanjianMirrorRegistry,
        variantAlgebra: nanjianVariantAlgebra,
        fraudDetector: nanjianFraudDetector,
        mqf,
    });
    // 智能体深度自治协议（P2P消息 + 分工投票 + 审判上诉 + 残差共振 + 自动伸缩 + 第三态识别）
    const nanjianSwarmProtocol = new NANJIAN_SWARM_PROTOCOL.SwarmProtocol({
        swarm: nanjianGeneEditor.swarm,
        mqf,
        mengji: nanjianMengji,
        geneEditor: nanjianGeneEditor,
        variantAlgebra: nanjianVariantAlgebra,
        fraudDetector: nanjianFraudDetector,
    });
    const nanjianModules = {
        // 1. 三维立体进制（时序先后 = 维度命名裁判，非 90 度）
        nanjian3d: NANJIAN_3D,
        nanjian3dTensor: (opts) => new NANJIAN_3D.Nanjian3DTensor(opts),
        // 2. 囡囝 4×6×16×6=2304 态射注册表
        nanjianMorphisms,
        // 3. ASMR/RTFM/JEPA 编码导入器（只取编码，不用原架构）
        nanjianEncoders: NANJIAN_ENCODERS,
        // 4. 囡囝不可见感知层（16通道感知编码流，不渲染像素/HTML）
        nanjianSensor,
        NanjianInvisibleSensor: NANJIAN_SENSOR.NanjianInvisibleSensor,
        // 5. 萌机合X注册表（替代"合伙人"，智能化时代意识互联网专用词汇）
        nanjianMengji,
        NanjianMengjiRegistry: NANJIAN_MENGJI.NanjianMengjiRegistry,
        // 6. 囡囝基因编辑器（后发编程·遥控编程·智能体自治联合体·CRISPR for code）
        nanjianGeneEditor,
        NanjianGeneEditor: NANJIAN_GENE_EDITOR.NanjianGeneEditor,
        // 7. 囡囝镜像姿态（7种二元镜像：脸对脸/背对背/手拉手/心连心/嘴对嘴/眼对眼/生殖器对生殖器）
        nanjianMirror: nanjianMirrorRegistry,
        NanjianMirror: NANJIAN_MIRROR.NanjianMirror,
        NanjianMirrorRegistry: NANJIAN_MIRROR.NanjianMirrorRegistry,
        MIRROR_POSES: NANJIAN_MIRROR.MIRROR_POSES,
        // 8. 囡囝变体加法群（ASMR+SMRL=ASMRL · 多智能体超级记忆）
        nanjianVariantAlgebra,
        VariantAlgebra: NANJIAN_VARIANT_ALGEBRA.VariantAlgebra,
        VARIANTS: NANJIAN_VARIANT_ALGEBRA.VARIANTS,
        // 9. 囡囝微量·宏量·赋权机制（权力+资本赋权，对抗高金波式欺诈）
        nanjianFraudDetector,
        MicroQuantum: NANJIAN_MICRO_MACRO.MicroQuantum,
        MacroQuantum: NANJIAN_MICRO_MACRO.MacroQuantum,
        EmpowermentCredential: NANJIAN_MICRO_MACRO.EmpowermentCredential,
        SuperpositionState: NANJIAN_MICRO_MACRO.SuperpositionState,
        empowerment: NANJIAN_MICRO_MACRO.empowerment,
        FraudDetector: NANJIAN_MICRO_MACRO.FraudDetector,
        // 10. 囡囝自然语言入口层（遥控编程：NL→候选→选择→自动导航）
        nanjianRemoteControlEditor,
        RemoteControlEditor: NANJIAN_NL_INTENT.RemoteControlEditor,
        parseIntent: NANJIAN_NL_INTENT.parseIntent,
        scanCandidates: NANJIAN_NL_INTENT.scanCandidates,
        presentCandidates: NANJIAN_NL_INTENT.presentCandidates,
        selectCandidate: NANJIAN_NL_INTENT.selectCandidate,
        autoNavigate: NANJIAN_NL_INTENT.autoNavigate,
        diffReport: NANJIAN_NL_INTENT.diffReport,
        // 11. 囡囝智能体深度自治协议（P2P消息+分工投票+审判上诉+残差共振+自动伸缩+第三态识别）
        nanjianSwarmProtocol,
        SwarmProtocol: NANJIAN_SWARM_PROTOCOL.SwarmProtocol,
        makeSwarmMessage: NANJIAN_SWARM_PROTOCOL.makeSwarmMessage,
        SWARM_TOPIC: NANJIAN_SWARM_PROTOCOL.SWARM_TOPIC,
        VOTE_TOPIC: NANJIAN_SWARM_PROTOCOL.VOTE_TOPIC,
        APPEAL_TOPIC: NANJIAN_SWARM_PROTOCOL.APPEAL_TOPIC,
        HYBRID_TOPIC: NANJIAN_SWARM_PROTOCOL.HYBRID_TOPIC,
        STRETCH_TOPIC: NANJIAN_SWARM_PROTOCOL.STRETCH_TOPIC,
        THIRD_STATE_TOPIC: NANJIAN_SWARM_PROTOCOL.THIRD_STATE_TOPIC,
    };
    // 如果开了 autoEncodeNanjian 默认 → 把 ASMR/RTFM/JEPA 当种子塞进 H4H 对应槽
    if (typeof options.autoEncodeNanjian !== 'boolean' || options.autoEncodeNanjian) {
        try {
            const asmrResult = NANJIAN_ENCODERS.encodeAsmr({ morphismRegistry: nanjianMorphisms });
            NANJIAN_ENCODERS.toH4HSeeds(asmrResult, h4h, { slot: options.asmrSlot || 'a4a' });
            const rtfmResult = NANJIAN_ENCODERS.encodeRtfm({ morphismRegistry: nanjianMorphisms });
            NANJIAN_ENCODERS.toH4HSeeds(rtfmResult, h4h, { slot: options.rtfmSlot || 'k4k' });
            const jepaResult = NANJIAN_ENCODERS.encodeJepa({ morphismRegistry: nanjianMorphisms });
            NANJIAN_ENCODERS.toH4HSeeds(jepaResult, h4h, { slot: options.jepaSlot || 'j4j' });
            // 导入事件 L4L publish（可选）
            if (mqf && typeof mqf.publish === 'function') {
                [asmrResult, rtfmResult, jepaResult].forEach(r => {
                    NANJIAN_ENCODERS.publishEncode(r, mqf, { topic: 'nanjian_encode' });
                });
            }
        } catch (_nanjianErr) { /* 导入不应该影响主流程 */ }
    }
    const cocoonCore = {
        Cocoon, ShellRegistry, BrowserWASMEntry, CocoonParser,
        registry,
        cocoonBootstrap,
        createCocoon, createRegistry, buildDemoCocoon,
        COCOON_MAGIC, COCOON_VERSION, DEFAULT_SANDBOX_PERMISSIONS,
        SHELL_REGISTRY,
    };
    const modules = Object.assign(
        { registry, cocoonCore, balanceJudge, catCodeSys, cocoonLang: LANG, h4h, mqf },
        nanjianModules,
        options.extraModules || {}
    );
    return A4A.create(Object.assign({}, options, { modules }));
}

function createHybridRegistry(options) { return H4H.create(options); }
function createMQF(options) { return L4L.create(options); }

// ============================================================
// 模块导出
// ============================================================
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        // 核心类
        Cocoon,
        ShellRegistry,
        BrowserWASMEntry,
        CocoonParser,
        // 寄生壳注册表
        SHELL_REGISTRY,
        // 常量
        COCOON_MAGIC,
        COCOON_VERSION,
        DEFAULT_SANDBOX_PERMISSIONS,
        // 引导与工厂函数
        cocoonBootstrap,
        createCocoon,
        createRegistry,
        buildDemoCocoon,
        // ---- 三大顶枝：H4H / A4A / L4L ----
        // H4H 混元杂交槽（26 x4x slots）
        H4H,
        HybridRegistry:   H4H.H4HRegistry,
        createHybridRegistry,
        SLOT_DEFS:        H4H.SLOT_DEFS,
        // L4L MQF 消息链
        L4L,
        MQF:              L4L.L4LMQF,
        createMQF,
        CATCODE_LEDGER:   L4L.CATCODE_LEDGER,
        // A4A 对外 API
        A4A,
        A4ARouter:        A4A.A4ARouter,
        createDefaultAPI,
        // 种子库（真实科技/数学/常识 分类登记到 26 个 x4x 槽）
        SeedLibrary:      SEED_LIB,
        TECH_SEEDS:       SEED_LIB.TECH_SEEDS,
        // ★★ 囡囝主权架构 · 夫妻共同体源头本体 · 不被规则定义的主体
        Nanjian3D:        NANJIAN_3D,
        NanjianMorphisms: NANJIAN_MORPH,
        NanjianEncoders:  NANJIAN_ENCODERS,
        NanjianSensor:    NANJIAN_SENSOR,
        NanjianMengji:    NANJIAN_MENGJI,
        NanjianGeneEditor: NANJIAN_GENE_EDITOR,
        NanjianMirror:    NANJIAN_MIRROR,
        NanjianVariantAlgebra: NANJIAN_VARIANT_ALGEBRA,
        NanjianMicroMacro:    NANJIAN_MICRO_MACRO,
        NanjianNLIntent:      NANJIAN_NL_INTENT,
        NanjianSwarmProtocol: NANJIAN_SWARM_PROTOCOL,
        LCTQF_NAME:       L4L.LCTQF_NAME,
        MLQF_NAME:        L4L.MLQF_NAME, // 已弃用别名
        SOVEREIGN_ROOT:   NANJIAN_3D.SOVEREIGN_NANJIAN,
        // 附带逻辑/猫码/工具链的快捷引用
        Logic:            LOGIC,
        Lang:             LANG,
        // ★★ 语言瘦身矩阵 + 适配器（强制 14 种语言吃 Rust/Zig 标准）
        LangSlimming:     LANG_SLIMMING,
        LangAdapter:      LANG_ADAPTER,
        LANG_SLIM_MATRIX: LANG_SLIMMING.LANG_SLIM_MATRIX,
        SOUL_SIGNATURES:  LANG_SLIMMING.SOUL_SIGNATURES,
    };
}
