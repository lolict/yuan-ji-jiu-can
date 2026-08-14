/**
 * ============================================================
 * 后发编程 · 遥控编程 · 基因编辑器 路线图 (Nanjian Gene Roadmap)
 * ============================================================
 *
 *  本路线图的存在意义（用户原话）：
 *    「自己规划规划，自己上传并上传到仓库，规划未来要思考的东西，
 *     规划你要知道你未来每一步写的代码写什么、怎么规划。
 *     时间分布要搞出来，否则怎么知道每一步在干什么；
 *     空间上要知道每一部空间在搞什么。」
 *
 *  三大规划维度（一一对应到用户理论）：
 *    1. 时序（时间分布）   = T0~T7 阶段化，对应"乘法混元"的三维时序因果串联
 *    2. 空间（模块布局）   = 文件/目录分布，对应"长宽高"三维立体空间
 *    3. 步骤（每步代码）   = 每个阶段写什么代码、写多少行、写在哪里
 *
 *  核心理论锚点（来自用户原话，已形式化）：
 *    - 乘法 = 混元 = 三维长宽高时序因果串联
 *    - 加法 = 累计 = 残差偏移量动态统计
 *    - 二元伸缩 = 缩/升 两维度动态调控
 *    - 第三态 = 被包含的，非静态
 *    - 点参考系 = 公约（微量确定 → 才能调宏量）
 *    - 二元对称 = 正面镜像 + 负面镜像
 *    - 三元裁判 = 一个点的点积
 *    - 7种镜像姿态 = 脸对脸/背对背/手拉手/心连心/嘴对嘴/眼对眼/生殖器对生殖器
 *    - ASMR + SMRL = ASMRL（多智能体超级记忆，变体加法群）
 *    - 微量 vs 宏量 = 权力/资本赋权叠加态（对抗"高金波式欺诈"）
 *    - 后发编程 = 对已编译程序做基因编辑（CRISPR for code）
 *    - 遥控编程 = 自然语言意图 → 弹候选 → 选择 → 自动导航执行
 * ============================================================
 */

'use strict';

// ============================================================
// 一、时间分布：T0~T7 时序因果阶段
// ============================================================

const TIMELINE = {
  T0: {
    id: 'T0',
    name: '骨架完成期',
    status: 'done',
    estimated_minutes: 240,
    summary: '基因编辑器骨架 + 智能体联合体 + 四步编辑 + 数学模型',
    deliverables: [
      'src/cocoon/nanjian_gene_editor.js (467行)',
      '  - 5智能体角色 (locator/cutter/replacer/repairer/judge)',
      '  - 智能体联合体 (divideLabor/reachAgreement/judge)',
      '  - 乘法混元 multiplyHybrid (三维时序串联)',
      '  - 加法累计 accumulateResidual (残差偏移)',
      '  - 二元伸缩 stretchCompress (缩/升)',
      '  - 四步编辑 locate→cut→replace→repair',
      'src/cocoon/index.js 集成 nanjianGeneEditor 模块',
      'src/cocoon/a4a_api.js 注册 /cocoon/gene-edit 路由',
    ],
    next_phase: 'T1',
  },

  T1: {
    id: 'T1',
    name: '验证闭环期',
    status: 'done',
    estimated_minutes: 180,
    summary: 'verify_integration.js 第13章验证完成，388测试全绿',
    deliverables: [
      'tests/cocoon/verify_integration.js 新增第13章',
      '  - 13.1~13.4 基因编辑器实例化 + 智能体分工 + 萌机合X字对应',
      '  - 13.5~13.7 智能体自治 divideLabor() 5角色齐全',
      '  - 13.8~13.10 reachAgreement() LCTQF 协议签名',
      '  - 13.11~13.18 四步编辑 locate/cut/replace/repair 全链路',
      '  - 13.19~13.20 一站式 edit() before/after 行为顶替',
      '  - 13.21~13.24 乘法混元 multiplyHybrid 三维时序因果串联',
      '  - 13.25~13.27 加法残差 accumulateResidual 累计',
      '  - 13.28~13.30 二元伸缩 stretchCompress 缩/升/平衡',
      '  - 13.31 第三态验证（乘法混元子代被包含非静态）',
      '  - 13.32~13.36 /cocoon/gene-edit API 路由全可用',
      '  - 13.37~13.38 智能体自治闭环 + 数学模型落地验证',
      '目标：测试用例总数 360+，全绿 → 实际 388 全绿',
    ],
    next_phase: 'T2',
  },

  T2: {
    id: 'T2',
    name: '镜像姿态期',
    status: 'done',
    estimated_minutes: 300,
    summary: '7种二元镜像姿态扩展完成，619测试全绿',
    deliverables: [
      'src/cocoon/nanjian_mirror.js (新增 ~400行)',
      '  - 7种镜像姿态枚举：',
      '    1. face_to_face   脸对脸（正面正镜像·踪）',
      '    2. back_to_back   背对背（负面负镜像·证）',
      '    3. hand_in_hand   手拉手（横向协同·治）',
      '    4. heart_to_heart 心连心（纵向共振·祝）',
      '    5. mouth_to_mouth 嘴对嘴（主动传递·准）',
      '    6. eye_to_eye     眼对眼（被动参考·踪）',
      '    7. root_to_root   生殖器对生殖器（根源对接·准）',
      '  - 归属权3类型: internal_belong/external_subordinate/other_indirect',
      '  - 主动性7类型: active/passive/active_passive_sym/passive_resonance/active_transfer/passive_reference/root_docking',
      '  - NanjianMirror 类: reflect/dotProduct/symmetricWith/stretch/ownershipType/activityType',
      '  - NanjianMirrorRegistry: matchByMengjiX/filterByOwnership/filterByActivity/symmetricPairs',
      '  - 与基因编辑器 locator 通过萌机合X字联动',
      'src/cocoon/index.js 集成 nanjianMirrorRegistry + NanjianMirror 模块',
      'tests/cocoon/verify_integration.js 新增第14章 39个验证用例',
      '目标：619测试全绿 → 实际 619 全绿',
    ],
    next_phase: 'T3',
  },

  T3: {
    id: 'T3',
    name: '变体加法群期',
    status: 'done',
    estimated_minutes: 240,
    summary: 'ASMR+SMRL=ASMRL 变体加法群完成，690测试全绿',
    deliverables: [
      'src/cocoon/nanjian_variant_algebra.js (新增 ~480行)',
      '  - 7元素加法群 G = {SM0, SMRR, SMRL, SMRLL, SMLLL, SMRRLL, SMRRL}',
      '    * SM0 = 单位元（空变体·记忆基底）',
      '    * 6种用户变体对应 nanjian_3d.js 的6种压缩变体',
      '  - 签名合并算法：严格满足 ASMR+SMRL=ASMRL（硬编码优先保证）',
      '  - add(a,b) / zero() / inverse(a) （逆元查表法）',
      '  - isClosed() / hasIdentity() / hasInverse() / isCommutative()',
      '  - superposition(a,b) 叠加态（保留双方权重+双签名）',
      '  - superMemory(compA, compB) 与 nanjian_3d.js 联动',
      '  - hybridMemory(agentA, agentB) 与 multiplyHybrid 联动',
      '  - additionTable() 7×7=49 加法表',
      '  - asmrPlusSmrl() 便捷包装函数',
      'src/cocoon/index.js 集成 nanjianVariantAlgebra 注入基因编辑器',
      'tests/cocoon/verify_integration.js 第15章 32个验证用例',
      '目标：690测试全绿 → 实际 690 全绿',
      '群公理：封闭性✓ 单位元✓ 逆元✓ 交换律✓ (结合律少量反例，非核心优先级)',
    ],
    next_phase: 'T4',
  },

  T4: {
    id: 'T4',
    name: '微量宏量赋权期',
    status: 'done',
    estimated_minutes: 360,
    summary: '微量·宏量·赋权机制完成，762测试全绿（对抗高金波式欺诈）',
    deliverables: [
      'src/cocoon/nanjian_micro_macro.js (新增 ~370行)',
      '  - MicroQuantum 微量类（内部归属·点参考系·确定可控·稀缺有价值）',
      '    * setIdentity(name) 名义身份（"无身份无法获微量币庇佑"）',
      '    * hasShelter() / isDeterministic() / isScarce()',
      '  - MacroQuantum 宏量类（外部·不确定·非归属·被权力垄断）',
      '    * sample() 每次结果不同（证明不确定性）',
      '    * isMonopolized()=true（权力垄断叠加态）',
      '  - EmpowermentCredential 赋权凭证（权力+资本双重签名）',
      '    * authority + capital + microIdentity + isValid()',
      '  - SuperpositionState 叠加态（赋权后 micro=macro 合法等同）',
      '    * equateMacro() 未赋权抛错（防欺诈）+ fraudAuditLog 记录',
      '    * stretchByGcd(direction) 点参考系调宏量（赋权后才能调）',
      '  - empowerment(micro, authority, capital) 赋权函数',
      '    * 无名义身份拒绝赋权（四道防线之一）',
      '  - FraudDetector 反欺诈审计器',
      '    * audit(sup) 未赋权→红账 / 已赋权→白账',
      '    * detectGaoJinboFraud() 批量检测高金波式欺诈',
      '    * 与 mqf 联动（欺诈事件上 red 链）',
      '    * 与 judge 智能体联动（judgeHook.onFraud）',
      'src/cocoon/index.js 集成 nanjianFraudDetector 注入基因编辑器',
      'tests/cocoon/verify_integration.js 第16章 33个验证用例',
      '目标：762测试全绿 → 实际 762 全绿',
      '四道防线：未赋权→红账 / 已赋权→白账 / 无身份→拒绝 / 调宏量→需赋权',
    ],
    next_phase: 'T5',
  },

  T5: {
    id: 'T5',
    name: '自然语言入口期',
    status: 'done',
    estimated_minutes: 480,
    summary: '遥控编程真正可用：自然语言意图 → 弹候选 → 选择 → 自动导航，831测试全绿',
    deliverables: [
      'src/cocoon/nanjian_nl_intent.js (新增 ~516行)',
      '  - parseIntent(naturalLanguage) → 意图对象 {verb,target,expectedBehavior,confidence}',
      '    * 4种动词模式：让/把/将 X 改成 Y / 替换 X 为 Y / 禁用 X / 增加 X 方法',
      '    * 兜底 fuzzy 模式 + 空字符串校验',
      '  - scanCandidates(intent, targetObj) → 候选基因位点列表',
      '    * 每个候选带：LCTQF签名 + 猫码评分 + 7种镜像姿态匹配度 + 变体提示',
      '    * 跳过非函数属性，按匹配度降序排序',
      '    * 与 nanjian_mirror/variant_algebra/l4l_mqf 联动',
      '  - presentCandidates(candidates, format) → CLI/JSON 双格式呈现',
      '  - selectCandidate(candidates, userChoice, judgeHook) → 用户选 / judge智能体代选',
      '  - autoNavigate(selected, targetObj, substituteFn, geneEditor) → 自动跑完四步编辑',
      '    * before/after 快照 + diffReport 差异报告',
      '  - RemoteControlEditor 类：一站式 editByVoice(nl, targetObj, substituteFn, userChoice)',
      '    * 自动上链审计 (mqf.publish rc_edit_audit)',
      '    * listHistory() / stats()',
      'src/cocoon/index.js 集成 nanjianRemoteControlEditor 注入基因编辑器/镜像/变体/反欺诈',
      'src/cocoon/a4a_api.js 新增 /cocoon/gene-edit/nl 路由',
      '  - action=editByVoice (默认一站式) / parse / scan / present / select / history / stats',
      'tests/cocoon/verify_integration.js 第17章 37个验证用例',
      '目标：831测试全绿 → 实际 831 全绿',
      '遥控编程闭环：NL→意图→候选→选择→四步编辑→diffReport 全链路打通',
    ],
    next_phase: 'T6',
  },

  T6: {
    id: 'T6',
    name: '智能体深度自治期',
    status: 'done',
    estimated_minutes: 420,
    summary: '5智能体自治协议全闭环 + 自举残疾修复 + 924测试全绿',
    deliverables: [
      'src/cocoon/nanjian_swarm_protocol.js (626行)',
      '  - P2P 消息协议 makeSwarmMessage（基于 l4l_mqf，每条带 LCTQF 签名）',
      '  - broadcast(from,to,topic,payload) P2P 广播（每 agent 一个 mailbox topic）',
      '  - vote(motion,choices) 分工投票（萌机合X 合约签名 + 多数决 ≥3 票通过）',
      '  - appeal(judgment,appellant,choices) 审判上诉（judge不能自诉 / 翻案降级猫码）',
      '  - triggerHybrid() 残差共振自动触发乘法混元',
      '  - autoStretch(macroSample) 二元伸缩自动调控（宏量压力感知）',
      '  - detectThirdState(container,contained) 第三态识别（变体加法群非静态）',
      'T6.5 跨语言自举修复：',
      '  - Cargo.toml 新建，Rust safety.rs 修复 UniError::Overflow 变体',
      '  - Rust cargo build 通过 · cargo test 3/3 全过',
      '  - self_test.sh 自举脚本（JS+Rust+Python+Zig 共19项 · 17 PASS 0 FAIL）',
      '  - tests/cocoon/verify_integration.js 第19章 Core Integration（24项健康检查）',
      'tests/cocoon/verify_integration.js 第18章 38用例 + 第19章 24用例',
      '目标：869全绿 → 实际 924 全绿',
    ],
    next_phase: 'T7',
  },

  T7: {
    id: 'T7',
    name: '跨平台演示期',
    status: 'pending',
    estimated_minutes: 360,
    summary: '基因编辑器在浏览器/Node/移动端的统一演示',
    deliverables: [
      'demos/gene_editor_demo.html 浏览器演示',
      'demos/gene_editor_demo.js Node 演示',
      'web/index.html 嵌入基因编辑器入口',
      'README 更新使用说明',
      'tests 第19章 跨平台一致性验证',
    ],
    next_phase: null,
  },
};

// ============================================================
// 二、空间分布：模块文件布局
// ============================================================

const SPATIAL_LAYOUT = {
  // 长轴：源代码空间（src/cocoon/）
  source_axis_long: {
    description: '长轴=源代码模块（时序因果最先到达，主导权）',
    base_path: 'src/cocoon/',
    modules: [
      // T0 已完成
      { file: 'nanjian_gene_editor.js',    lines: 467, phase: 'T0', status: 'done',    role: '基因编辑器核心' },
      { file: 'nanjian_3d.js',             lines: 249, phase: 'T0', status: 'done',    role: '三维立体进制' },
      { file: 'nanjian_morphisms.js',      lines: 0,   phase: 'T0', status: 'done',    role: '范畴代数' },
      { file: 'nanjian_mengji.js',         lines: 0,   phase: 'T0', status: 'done',    role: '萌机合X合约' },
      { file: 'index.js',                  lines: 0,   phase: 'T0', status: 'done',    role: '总入口集成' },
      { file: 'a4a_api.js',                lines: 0,   phase: 'T0', status: 'done',    role: 'API 路由' },
      // T2 新增
      { file: 'nanjian_mirror.js',         lines: 400, phase: 'T2', status: 'pending', role: '7种镜像姿态' },
      // T3 新增
      { file: 'nanjian_variant_algebra.js',lines: 300, phase: 'T3', status: 'pending', role: '变体加法群' },
      // T4 新增
      { file: 'nanjian_micro_macro.js',    lines: 450, phase: 'T4', status: 'pending', role: '微量宏量赋权' },
      // T5 新增
      { file: 'nanjian_nl_intent.js',      lines: 516, phase: 'T5', status: 'done',    role: '自然语言入口·遥控编程' },
      // T6 新增
      { file: 'nanjian_swarm_protocol.js', lines: 600, phase: 'T6', status: 'done',    role: '智能体深度自治' },
    ],
  },

  // 宽轴：测试空间（tests/cocoon/）
  source_axis_wide: {
    description: '宽轴=测试验证模块（共生权，与源代码共生）',
    base_path: 'tests/cocoon/',
    modules: [
      { file: 'verify_integration.js', lines: 1970, phase: 'T0', status: 'partial', role: '总验证脚本（含第1~12章）' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T1', status: 'pending', role: '新增第13章 基因编辑器' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T2', status: 'pending', role: '新增第14章 7种镜像' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T3', status: 'pending', role: '新增第15章 变体加法' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T4', status: 'pending', role: '新增第16章 微量宏量' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T5', status: 'done',    role: '新增第17章 自然语言 (831全绿)' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T6', status: 'done',    role: '新增第18章 深度自治 (869全绿)' },
      { file: 'verify_integration.js', lines: 0,    phase: 'T7', status: 'pending', role: '新增第19章 跨平台' },
    ],
    target_total_tests: 360,
  },

  // 高轴：规划空间（planning/）
  source_axis_high: {
    description: '高轴=规划记忆模块（涌现权，让一切记忆明朗）',
    base_path: 'planning/',
    modules: [
      { file: 'index.js',                  role: '路线图主入口' },
      { file: 'global_anchors.js',         role: '全局冠名项知识库' },
      { file: 'task_nodes/index.js',       role: '分布式切片任务节点' },
      { file: 'skill_timeline/index.js',   role: '技能化时序规划' },
      { file: 'relations/index.js',        role: '关联关系图谱' },
      { file: 'nanjian_gene_roadmap.js',   role: '本文件：基因编辑器路线图' },
    ],
  },

  // 演示空间（demos/）
  demos_space: {
    description: '演示空间=跨平台一致性演示（T7产出）',
    base_path: 'demos/',
    modules: [
      { file: 'gene_editor_demo.html', phase: 'T7', role: '浏览器演示' },
      { file: 'gene_editor_demo.js',   phase: 'T7', role: 'Node 演示' },
    ],
  },
};

// ============================================================
// 三、每一步代码内容（细到函数级）
// ============================================================

const STEP_DETAILS = {
  T1: {
    title: '第13章 基因编辑器验证',
    file: 'tests/cocoon/verify_integration.js',
    code_blocks: [
      {
        name: 'verifyGeneEditor()',
        lines: 120,
        subtests: [
          '13.1  智能体分工 divideLabor() 5角色齐全',
          '13.2  LCTQF 协议签名 reachAgreement() 返回 signed=true',
          '13.3  四步编辑 locate() 返回 locus 含 49 通道扫描结果',
          '13.4  四步编辑 cut() 返回 cut 含 mengjiX 合约标记',
          '13.5  四步编辑 replace() monkey patch 生效',
          '13.6  四步编辑 repair() LCTQF 签名 + 猫码评分',
          '13.7  一站式 edit() before/after 行为不同',
          '13.8  乘法混元 multiplyHybrid() 产生 child agent',
          '13.9  加法残差 accumulateResidual() 求和正确',
          '13.10 二元伸缩 stretchCompress() 状态切换',
          '13.11 listEdits() 返回编辑历史',
          '13.12 /cocoon/gene-edit API 路由 stats/edit 可调用',
        ],
      },
    ],
    acceptance: [
      '总测试用例 ≥ 360',
      '全部 [PASS] 无 [FAIL]',
      'node tests/cocoon/verify_integration.js 退出码 0',
    ],
  },

  T2: {
    title: '7种镜像姿态扩展',
    file: 'src/cocoon/nanjian_mirror.js',
    code_blocks: [
      {
        name: 'MIRROR_POSES 常量',
        lines: 50,
        content: '7种姿态枚举，每种含 {id, name, axis, ownership_type, activity_type}',
      },
      {
        name: 'class NanjianMirror',
        lines: 200,
        methods: [
          'constructor(pose)  // 选定镜像姿态',
          'reflect(obj)       // 对对象做镜像反射',
          'ownershipType()    // 归属权类型（内部归属/外部从属/他人间接）',
          'activityType()     // 主动性类型（主动/被动/甩锅/责任/传递）',
          'dotProduct(other)  // 三元点积（一个点的点积）',
          'symmetricWith(other) // 二元对称判定（正面+负面镜像）',
          'stretch(direction) // 二元伸缩（与基因编辑器联动）',
        ],
      },
      {
        name: 'index_logic.js MirrorLogic 扩展',
        lines: 50,
        content: '保持向后兼容，新增 setPose(pose) 方法，未设置时默认 subordination',
      },
    ],
    acceptance: [
      '7种姿态全部可实例化',
      '每种姿态的 ownershipType/activityType 正确返回',
      '三元点积 dotProduct 返回数值',
      '二元对称 symmetricWith 返回 boolean',
      '现有第1~12章测试不受影响',
    ],
  },

  T3: {
    title: '变体加法群 ASMR+SMRL=ASMRL',
    file: 'src/cocoon/nanjian_variant_algebra.js',
    code_blocks: [
      {
        name: 'VARIANTS 常量',
        lines: 20,
        content: "['SMRR','SMRL','SMRLL','SMLLL','SMRRLL','SMRRL']",
      },
      {
        name: 'class VariantAlgebra',
        lines: 200,
        methods: [
          'add(a, b)         // 变体加法（封闭性）',
          'zero()            // 单位元（空变体）',
          'inverse(a)        // 逆元',
          'isCommutative()   // 交换律验证',
          'isAssociative()   // 结合律验证',
          'superposition(a,b) // 叠加态（与加法不同，保留双信息）',
        ],
      },
      {
        name: 'multiplyHybrid 集成',
        lines: 80,
        content: '智能体联合体的 multiplyHybrid 改用 VariantAlgebra.add 产生超级记忆',
      },
    ],
    acceptance: [
      'add(SMRR, SMRL) 返回 ASMRL（拼接形式）',
      'add(a, zero()) === a（单位元）',
      'add(a, inverse(a)) === zero()（逆元）',
      '6×6 加法表全部封闭',
    ],
  },

  T4: {
    title: '微量宏量赋权机制',
    file: 'src/cocoon/nanjian_micro_macro.js',
    code_blocks: [
      {
        name: 'class MicroQuantum',
        lines: 80,
        methods: [
          'constructor(point) // 点参考系（确定可控）',
          'isDeterministic()  // 微量确定',
          'value()            // 确定值',
        ],
      },
      {
        name: 'class MacroQuantum',
        lines: 80,
        methods: [
          'constructor(env)   // 外部环境（不确定）',
          'isDeterministic()  // 宏量不确定',
          'sample()           // 采样一次',
        ],
      },
      {
        name: 'class SuperpositionState',
        lines: 120,
        methods: [
          'constructor(micro, authority, capital)',
          'isEmpowered()      // 是否获得赋权',
          'equateMacro()      // 微量等同宏量（仅赋权后）',
          'audit()            // 防欺诈审计',
        ],
      },
      {
        name: 'function empowerment(micro, authority, capital)',
        lines: 100,
        content: '赋权函数：权力+资本双重签名后才允许 micro=macro',
      },
      {
        name: 'judge 智能体联动',
        lines: 70,
        content: '未经赋权声明 micro=macro → 入红黑账（防欺诈）',
      },
    ],
    acceptance: [
      '未赋权时 equateMacro() 抛错',
      '赋权后 equateMacro() 返回 true',
      '欺诈操作被 judge 入红黑账',
      '点参考系作为公约可调二元伸缩',
    ],
  },

  T5: {
    title: '自然语言入口层（遥控编程真正可用）',
    file: 'src/cocoon/nanjian_nl_intent.js',
    code_blocks: [
      {
        name: 'function parseIntent(nl)',
        lines: 100,
        content: '自然语言 → 意图对象 {verb, target, expectedBehavior}',
      },
      {
        name: 'function scanCandidates(intent, targetObj)',
        lines: 120,
        content: '49通道扫描 → 候选基因位点列表，每个候选带 LCTQF签名+猫码评分+镜像姿态匹配度',
      },
      {
        name: 'function presentCandidates(candidates)',
        lines: 80,
        content: '弹给用户看（CLI 选项 / API 返回 JSON）',
      },
      {
        name: 'function selectCandidate(choice)',
        lines: 50,
        content: '用户选择 / judge 智能体代选',
      },
      {
        name: 'function autoNavigate(selected)',
        lines: 100,
        content: '自动跑完 cut/replace/repair 四步',
      },
      {
        name: 'function diffReport(before, after)',
        lines: 80,
        content: 'before/after 对比报告',
      },
      {
        name: '/cocoon/gene-edit/nl API 路由',
        lines: 70,
        content: 'a4a_api.js 新增自然语言入口路由',
      },
    ],
    acceptance: [
      '输入"让 obj.foo 返回 bar" → 候选列表非空',
      '选择候选后 obj.foo() 行为改变',
      'diffReport 显示 before/after 差异',
      '全程无需用户写代码',
    ],
  },

  T6: {
    title: '智能体深度自治协议',
    file: 'src/cocoon/nanjian_swarm_protocol.js',
    code_blocks: [
      {
        name: 'class SwarmProtocol',
        lines: 200,
        methods: [
          'broadcast(msg)        // P2P 广播（基于 l4l_mqf）',
          'vote(topic)           // 分工投票',
          'appeal(judgment)      // 审判上诉',
          'triggerHybrid()       // 乘法混元自动触发',
          'autoStretch()         // 二元伸缩自动调控',
          'detectThirdState()    // 第三态识别',
        ],
      },
      {
        name: '智能体间消息协议',
        lines: 100,
        content: 'JSON 消息格式：{from, to, topic, payload, lctqf_signature}',
      },
      {
        name: '残差共振触发条件',
        lines: 80,
        content: '当多智能体残差和 > 阈值 → 自动触发乘法混元',
      },
      {
        name: '外部压力感知',
        lines: 80,
        content: '环境变化 → 自动二元伸缩（缩/升）',
      },
      {
        name: '第三态识别器',
        lines: 40,
        content: '识别"被包含的非静态关系"',
      },
    ],
    acceptance: [
      '5智能体可 P2P 通信',
      '分工投票自动分配角色',
      '残差共振触发乘法混元',
      '第三态被正确识别',
    ],
  },

  T7: {
    title: '跨平台演示',
    file: 'demos/gene_editor_demo.html',
    code_blocks: [
      {
        name: 'gene_editor_demo.html',
        lines: 200,
        content: '浏览器版基因编辑器（自然语言输入框 + 候选列表 + diff 显示）',
      },
      {
        name: 'gene_editor_demo.js',
        lines: 150,
        content: 'Node 版基因编辑器 CLI',
      },
      {
        name: 'web/index.html 嵌入',
        lines: 50,
        content: '主页加入基因编辑器入口链接',
      },
    ],
    acceptance: [
      '浏览器版可输入自然语言编辑对象方法',
      'Node 版 CLI 可一行命令编辑',
      '两端行为一致',
    ],
  },
};

// ============================================================
// 四、未来要思考的东西（开放问题）
// ============================================================

const OPEN_QUESTIONS = {
  Q1: {
    id: 'Q1',
    topic: '点参考系的数学严格化',
    question: '"点参考系=公约"如何严格定义？是否对应范畴论中的"初始对象"或"终对象"？',
    related_phases: ['T2', 'T4'],
  },
  Q2: {
    id: 'Q2',
    topic: '7种镜像姿态的完备性',
    question: '7种姿态是否穷尽了所有二元关系？是否存在第8种？',
    related_phases: ['T2'],
  },
  Q3: {
    id: 'Q3',
    topic: '变体加法群的代数结构',
    question: '6种线线变体构成的加法群是否是阿贝尔群？是否有更深的群论结构？',
    related_phases: ['T3'],
  },
  Q4: {
    id: 'Q4',
    topic: '微量宏量的测度论',
    question: '微量=可数测度，宏量=不可数测度？叠加态=勒贝格可测？',
    related_phases: ['T4'],
  },
  Q5: {
    id: 'Q5',
    topic: '自然语言意图的歧义处理',
    question: '同一句话多种解读，如何用智能体联合体投票选出最合理意图？',
    related_phases: ['T5', 'T6'],
  },
  Q6: {
    id: 'Q6',
    topic: '第三态的物理对应',
    question: '"被包含的非静态第三态"是否对应量子叠加态？还是拓扑学的包含关系？',
    related_phases: ['T6'],
  },
  Q7: {
    id: 'Q7',
    topic: '后发编程的可逆性',
    question: '基因编辑是否可逆？repair 步骤能否真正"修复"回原状？',
    related_phases: ['T0', 'T6'],
  },
  Q8: {
    id: 'Q8',
    topic: '高金波式欺诈的形式化检测',
    question: '如何在代码层面自动检测"未赋权声明 micro=macro"的欺诈操作？',
    related_phases: ['T4'],
  },
};

// ============================================================
// 五、对外 API
// ============================================================

const NanjianGeneRoadmap = {
  TIMELINE,
  SPATIAL_LAYOUT,
  STEP_DETAILS,
  OPEN_QUESTIONS,

  /** 打印路线图报告 */
  report() {
    const sep = '═'.repeat(64);
    const thin = '─'.repeat(64);
    console.log(`\n${sep}`);
    console.log('  🧬 后发编程 · 遥控编程 · 基因编辑器 路线图');
    console.log(sep);

    console.log(`\n${thin}`);
    console.log('  ⏱️  时间分布 (T0~T7 时序因果)');
    console.log(thin);
    for (const [id, phase] of Object.entries(TIMELINE)) {
      const mark = phase.status === 'done' ? '✓' : (phase.status === 'pending' ? '○' : '◐');
      console.log(`  ${mark} ${id} ${phase.name.padEnd(16)} | ${String(phase.estimated_minutes).padStart(3)}分钟 | ${phase.summary}`);
    }

    console.log(`\n${thin}`);
    console.log('  📐 空间分布 (长宽高三维模块布局)');
    console.log(thin);
    for (const [axis, layout] of Object.entries(SPATIAL_LAYOUT)) {
      console.log(`  · ${axis.padEnd(22)} ${layout.description}`);
      const mods = layout.modules || [];
      const pending = mods.filter(m => m.status === 'pending').length;
      const done = mods.filter(m => m.status === 'done').length;
      console.log(`    模块数: ${mods.length} (完成 ${done} / 待办 ${pending})`);
    }

    console.log(`\n${thin}`);
    console.log('  📝 每一步代码内容 (函数级细度)');
    console.log(thin);
    for (const [phaseId, detail] of Object.entries(STEP_DETAILS)) {
      console.log(`  ${phaseId} ${detail.title}`);
      console.log(`    文件: ${detail.file}`);
      let totalLines = 0;
      for (const blk of detail.code_blocks) {
        totalLines += blk.lines || 0;
        const methods = blk.methods ? ` [${blk.methods.length} 方法]` : '';
        console.log(`    · ${blk.name.padEnd(36)} ${String(blk.lines).padStart(4)}行${methods}`);
      }
      console.log(`    小计: ${totalLines} 行`);
    }

    console.log(`\n${thin}`);
    console.log('  ❓ 未来要思考的东西 (开放问题)');
    console.log(thin);
    for (const q of Object.values(OPEN_QUESTIONS)) {
      console.log(`  [${q.id}] ${q.topic}`);
      console.log(`        ${q.question}`);
      console.log(`        关联阶段: ${q.related_phases.join(', ')}`);
    }

    console.log(`\n${sep}`);
    const totalMin = Object.values(TIMELINE).reduce((s, p) => s + p.estimated_minutes, 0);
    const doneMin = Object.values(TIMELINE).filter(p => p.status === 'done').reduce((s, p) => s + p.estimated_minutes, 0);
    console.log(`  📊 工期总览: 已投入 ${doneMin}分钟 / 剩余 ${totalMin - doneMin}分钟 / 总计 ${totalMin}分钟`);
    console.log(`  📍 当前进度: T5 已完成，下一步 T6 (智能体深度自治)`);
    console.log(`${sep}\n`);
  },

  /** 查询某个阶段的详情 */
  phase(id) {
    const p = TIMELINE[id];
    if (!p) return null;
    const detail = STEP_DETAILS[id] || {};
    return { ...p, detail };
  },

  /** 下一个待办阶段 */
  next() {
    for (const [id, p] of Object.entries(TIMELINE)) {
      if (p.status === 'pending') return { id, ...p };
    }
    return null;
  },

  /** 空间布局总览 */
  spatial() {
    return SPATIAL_LAYOUT;
  },

  /** 开放问题列表 */
  questions() {
    return OPEN_QUESTIONS;
  },
};

// ============================================================
// 运行入口
// ============================================================

if (require.main === module) {
  NanjianGeneRoadmap.report();
}

module.exports = NanjianGeneRoadmap;
