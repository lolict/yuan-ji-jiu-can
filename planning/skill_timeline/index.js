/**
 * ============================================================
 * 技能化时序规划 (Skillized Timeline Planning)
 * ============================================================
 * 
 * 核心思想：
 * 1. 每个开发阶段对应一套技能组合 (skill bundle)
 * 2. 每个阶段有明确的交付物 (deliverables)
 * 3. 每个阶段有入口验收标准 (entry criteria) 和出口验收标准 (exit criteria)
 * 4. 时序按"最底层优先级最高"的顺序：符号→编码→系统→UI→平台
 * 5. 每个阶段对应历史用户指令的原话映射，确保"最优秀的指令"不丢失
 * 
 * 时序阶段数量：8个阶段 (对应八卦/octave)
 *   T0 → T1 → T2 → T3 → T4 → T5 → T6 → T7
 *   最底层                                        最高层
 * ============================================================
 */

const SKILL_TIMELINE = [
  // ========================================================
  // T0: 纯符号集最终固化阶段
  // ========================================================
  {
    stage: 'T0',
    name: '纯符号集最终固化',
    time_order: 0,
    duration_days: 1,
    skill_bundle: [
      'Unicode特殊字符', '拉丁扩展字符集', '西里尔字母',
      'IPA国际音标符号', '小学拼音顺口溜顺序',
    ],
    user_instruction_refs: [
      '用户原话: "把这些(变音符号)变成小写，这样子他就能够找到大小写的合作关系了，就能凑够那个170个字母了"',
      '用户原话: "按小学生拼音韵母标准顺口溜排序——完全沿用你原有全部符号，一个字符不改，只按小学课本顺口溜重排"',
      '用户原话: "ueng 小写౨大写ⱂ"',
      '用户原话: "32个韵母全部排满、无遗漏、不乱序，可以直接定稿存档"',
      '用户原话: "最优秀的指令是什么？——顺口溜排序、固定大小写对应凑够48/64/170"',
    ],
    deliverables: [
      'unicore_symbols.js 最终符号定义 (唯一真理源)',
      '声母48个: 24小写b p m f ... ŋ + 24大写对应',
      '韵母64个: 32大写按顺口溜序 + 32小写对应，ueng固定౨/ⱂ',
      '声调170个: 85小写+85大写字母+数字生成',
    ],
    entry_criteria: ['有参考符号序列、用户确认了顺口溜顺序和ueng特殊值'],
    exit_criteria: ['符号全部定版且数量正确: 48/64/170', '所有core文件引用同一个unicore_symbols.js'],
    bindings: ['radix_shengmu', 'radix_yunmu', 'radix_shengdiao', 'shunliu_root'],
    depends_on_stages: [],
  },

  // ========================================================
  // T1: 集装箱编码 & 四维机器码对接
  // ========================================================
  {
    stage: 'T1',
    name: '集装箱编码 & 四维机器码对接',
    time_order: 1,
    duration_days: 1.5,
    skill_bundle: [
      '进制转换算法', '进位/不进位逻辑',
      '位操作', '硬件机器码对接思维',
      'Container编码/解码',
    ],
    user_instruction_refs: [
      '用户原话: "通常人类的拼音都是先读声母，然后读韵母，最后读音调...按照税率效率最大，总是数量最多的那个优先。数量少的那个排后。"',
      '用户原话: "四个维度 = 四个集装箱 (第1格,第2格,第3格,第4格)"',
      '用户原话: "前三个维度...锁死和电脑机器码0、1底层对接，做固定集装箱框架，永不随便进位、永不随便变大"',
      '用户原话: "第四个维度是唯一能递增、能变大、能走序号的动态格"',
      '用户原话: "等效维度映射：第四维每涨1个，对应前三维等效层级占位，数值增长但维度层级权重等效对齐"',
      '用户原话: "全程不破坏前三维 0,0,0 的底层结构"',
    ],
    deliverables: [
      'ContainerEncoder: encode/decode/increment/decrement 正确',
      'FourDimNonCarrySystem: increment() 1000次后staticFrame=[0,0,0]',
      'ContainerMachineCodeAdapter: 集装箱1/3→0，集装箱2/4→1',
      '优先级默认byCount: 声调170>韵母64>声母48>数字17',
    ],
    entry_criteria: ['T0已交付符号序列定版'],
    exit_criteria: ['test_containers.js + test_four_dim.js 全部通过'],
    bindings: ['container_1', 'container_2', 'container_3', 'container_4', 'four_dim_system', 'static_frame_123', 'equivalent_mapping', 'container_self_radix', 'leading_zero_negative'],
    depends_on_stages: ['T0'],
  },

  // ========================================================
  // T2: 纯符号核心系统 (格子+互认+阶系+阵法+聚类)
  // ========================================================
  {
    stage: 'T2',
    name: '纯符号核心系统贯通',
    time_order: 2,
    duration_days: 3,
    skill_bundle: [
      '三维数据结构', '关系代数', '抽象数学系统',
      '因果逻辑', '拓扑空间思维', '聚类算法',
    ],
    user_instruction_refs: [
      '用户原话: "我们的声母、我们的韵母、我们的声调本身就是进制..."',
      '用户原话: "3D表格/容器系统，实现稀疏数代数（小集合表示大集合）"',
      '用户原话: "3D格子结构：中心位、6面墙、8内角、6外角、十字架节点"',
      '用户原话: "阶和阶角阶除阶位阶权阶属阶集阶序阶乘阶岔阶段阶叠阶逆阶顺阶降阶升阶渐变阶稀疏数律阶范畴..."',
      '用户原话: "互认接口/权位/格子/节点/墙面/集装箱/阴阳/关系/进制/对齐/身份/问题/目标/基因/钥匙/门锁/因果律..."',
      '用户原话: "锥阵/球阵/方块阵/三角阵/蜂窝阵/门阵/锁阵/镜像阵/对立统一阵/聚类认知阵/空间认知阵/向量阵/标量阵/张量阵/矩阵阵"',
      '用户原话: "聚类系统：对错、远近、运动、边界、大小分类"',
      '用户原话: "模块锚点 mll/ll/ct/gtt/xy/gy/mrror/cbc/mrror.mll/cbc.mll"',
    ],
    deliverables: [
      'PureGrid: 1中心+6墙+8内角+6外连+十字架节点，write/read/connect',
      'PureHuren: 18项互认贯通+权级高低判定',
      'PureJie: 19阶全部有方法，阶乘进位链=声调→韵母→声母→double',
      'PureZhenfa: 锥阵(多层并联)、球阵、蜂窝、门/锁、镜像、三角因果',
      'PureModules: 10锚点激活/绑定/mllCluster分桶算法',
    ],
    entry_criteria: ['T1完成集装箱编码'],
    exit_criteria: ['test_pure_symbol.js 全部通过'],
    bindings: ['pure_symbol_system', 'pure_grid', 'pure_huren', 'pure_jie', 'pure_zhenfa', 'pure_modules'],
    depends_on_stages: ['T0', 'T1'],
  },

  // ========================================================
  // T3: 汉字积木编码 & 稀疏数代数
  // ========================================================
  {
    stage: 'T3',
    name: '汉字积木编码 & 稀疏数代数',
    time_order: 3,
    duration_days: 2,
    skill_bundle: [
      '汉字学：偏旁/部首/笔画/演化',
      '抽象映射代数', '数据结构填充',
    ],
    user_instruction_refs: [
      '用户原话: "文字还要拆分成具体的，它的向量、矩阵、张量，指向它的差分积木零件..."',
      '用户原话: "稀疏数代数（小集合表示大集合）——后面数量少的代表前面数量多的集合"',
      '用户原话: "优先级：声调>韵母>声母>数字，数字是代数，代表前面所有集合"',
      '用户原话: "你还用十进制干什么？这是别人的十进制，我们用它干什么？我们的声母韵母声调本身就是进制！"',
      '用户原话: "稀疏数律——节点疏密排布、阵形聚散留白固有规律"',
    ],
    deliverables: [
      'PureCBC.storeChar(): 1中心+6墙+8内角+6外连共21字段全填',
      'PureCBC.traceback(): 21字段完整回溯',
      'SparseNumberSystem.buildSparseMapping(): 数字→声母→韵母→声调',
      'getSymbolChain()追踪整个稀疏映射链',
      'sparseAdd()取权位更高符号',
    ],
    entry_criteria: ['T2格子系统+锚点模块完成'],
    exit_criteria: ['汉字积木样例可完整traceback', '稀疏数映射分桶平均'],
    bindings: ['pure_cbc', 'sparse_algebra', 'cbc_anchor', 'mll_anchor', 'jie_xishu'],
    depends_on_stages: ['T2'],
  },

  // ========================================================
  // T4: Foundation底层核心 + CBC格式 + Guardian守护
  // ========================================================
  {
    stage: 'T4',
    name: 'Foundation底层核心 + CBC格式 + 知识产权守护',
    time_order: 4,
    duration_days: 3,
    skill_bundle: [
      '二进制解析/序列化', 'WASM/WASI编译',
      '混淆加密技术', '文件格式设计',
      '编译构建工具链',
    ],
    user_instruction_refs: [
      '用户原话: "0xA5 trae 经常用这样子一个指令的代码，我也不知道要不要干预他这样子一个写法"',
      '用户原话: "但为什么都是使用旧的这些格式呀？使用旧的格式，你不怕被别人抄袭拿走吗？"',
      '用户原话: "保护知识产权，采用私有格式和混淆技术"',
      '用户原话: "自定义魔数 / 动态混淆映射 / 物理绑定硬件特征 / 生态输入法+表格+指令闭环"',
      '用户原话: "开发可编译为WASM的核心基础代码，跨平台兼容"',
      '用户原话: "绝对原则：零外部依赖、完全私有格式、自有指令集、自有编码系统、自有进制体系"',
    ],
    deliverables: [
      'CBCParser: encode(parse) roundtrip正确，魔数0x3D434243',
      'SparseAlgebra + UniCoreVM (64KB内存8寄存器)',
      'WASIAdapter: fd_write/clock_time_get/args支持',
      'unicore_foundation_wasm.js 可编译到wasm',
      '.3d.cbc.mll v2.0 6大数据段(进制/字符集/集装箱/空间/时间节点/插件)完整可读写',
      'UniCoreGuardian v5.0: 动态混淆Fisher-Yates+私有魔数+5大聚类+9大阵法+宗族5级关系',
    ],
    entry_criteria: ['T3汉字积木+稀疏代数完成'],
    exit_criteria: ['test_foundation.js全通过', '混淆两次实例化映射不同', '3dcbc私有格式读写一致'],
    bindings: ['unicore_foundation', 'cbc_format_spec', 'cbc_magic', 'unicore_guardian', 'dynamic_obfuscation', 'wasi_adapter'],
    depends_on_stages: ['T0', 'T1', 'T2', 'T3'],
  },

  // ========================================================
  // T5: UniISA v2.0 指令集 + 虚拟机
  // ========================================================
  {
    stage: 'T5',
    name: 'UniISA v2.0 指令集 + 虚拟机集成',
    time_order: 5,
    duration_days: 3.5,
    skill_bundle: [
      'CPU指令集设计', '虚拟机实现',
      '事件驱动模型', '因果链传播',
      '汇编/二进制翻译',
    ],
    user_instruction_refs: [
      '用户原话: "过去的指令集能不能够支持现在的功能呢？过去用的那些指令集全都是落后的指令集"',
      '用户原话: "革命性架构(ISA v2)：不进位计数→零溢出/节点因果时间→事件驱动低功耗/集装箱内存→无碎片/插件即指令→无限扩展/空间向量→多维原生支持"',
      '用户原话: "传统时钟驱动不行→改成节点驱动执行：节点触发→因果传播→指令执行→等待"',
      '用户原话: "不进位、节点时间、集装箱架构——这三个就是v2.0的灵魂"',
      '用户原话: "二进制翻译器：x86/ARM/RISC-V/MIPS → UniISA"',
    ],
    deliverables: [
      'UniISA v2.0 5大类opcode实现: 基础/不进位/节点时间/空间/插件',
      '0xA5特殊系统子命令: INIT_3D/INIT_TABLE/SYNC/EXPORT/IMPORT/CREATE_CONTAINER等',
      'NodeTime节点因果系统: CREATE_NODE→LINK→TRIGGER→WAIT→SYNC_TIME事件驱动模型',
      'SymbolVM + InstructionEncoder: 声韵调符号直接映射成指令',
      'UniCoreVM: 内存/堆栈/寄存器/程序计数器/PureGrid整合',
      'BinaryTranslator: x86/ARM/RISC-V/MIPS 检测+翻译 (Zig实现)',
      '3个汇编样例跑通: 排序.asm / 阶乘.asm / 斐波那契.asm',
    ],
    entry_criteria: ['T4 Foundation层完成'],
    exit_criteria: ['test_vm.js + test_instructions.js全通过', '3个样例汇编程序输出正确'],
    bindings: ['uniisa_v2', 'node_time_system', 'unicore_vm', 'unicore_instructions', 'unicore_translator', 'sys_0xA5'],
    depends_on_stages: ['T1', 'T2', 'T4'],
  },

  // ========================================================
  // T6: 3D UI / 表格系统 / 引擎整合
  // ========================================================
  {
    stage: 'T6',
    name: '3D UI / 里外嵌套表格 / 系统整合',
    time_order: 6,
    duration_days: 3.5,
    skill_bundle: [
      'Three.js/WebGL', '前端UI/UX',
      '三维空间渲染', '事件交互',
      '多层嵌套数据结构',
    ],
    user_instruction_refs: [
      '用户原话: "我嫌弃当前HTML HTML5 vue node Ajax这些也都不行。他们都是二维平面的。我需要的是三维立体的。"',
      '用户原话: "0=母集/女集，1=父集/子集。里外关系：0包含1，点击进入。函子：0包含1的映射关系。大千世界：0就是一个宇宙。"',
      '用户原话: "0包含1的函数关系。每个0内部必然包含一个1，1对0有反馈。"',
      '用户原话: "自动为0添加1子 (autoFillZeros)"',
      '用户原话: "集装箱三维可视化、空间阵列、节点链、自定义字符集进制切换"',
      '用户原话: "输入法一体化和表格一体化..."',
    ],
    deliverables: [
      'CubeTable系统: 0母1子里外嵌套+点击toggle展开最多6层',
      'InsideOutsideMapper: 0包含1函子+autoFillZeros自动填充',
      'CubeTableRenderer(Three.js):三维渲染+选中高亮+标签',
      'TableContainer 导入导出一致',
      'HACSJS 3D引擎5大功能:256集装箱矩阵/锥球蜂窝矩阵阵列/节点链/自定义字符集/自定义进制',
      'UniCoreSystem v2.0: 5合1总控(WASM+3D+Table+Plugin+CBC)',
      'UniCorePluginSystem: Radix/Charset/Spatial/0xA5系统插件',
    ],
    entry_criteria: ['T5 VM层完成'],
    exit_criteria: [
      'cube_table_demo可展开3层嵌套',
      'hacsjs_3d_demo 5大按钮可用',
      'unicore_complete.html 启动成功所有子模块加载',
    ],
    bindings: ['cube_table', 'inside_outside_mapper', 'hacsjs_3d', 'unicore_system_core', 'jie_die'],
    depends_on_stages: ['T2', 'T5'],
  },

  // ========================================================
  // T7: 多语言协作 / 云端同步 / 跨平台部署
  // ========================================================
  {
    stage: 'T7',
    name: '多语言协作 / 云端同步 / 6平台跨平台部署',
    time_order: 7,
    duration_days: 5,
    skill_bundle: [
      'Zig/Rust/Go/Python 多语言FFI',
      '构建系统(CMake/Zig build/Cargo/Go build)',
      'GitHub Gist API / WebDAV协议 / WebRTC P2P',
      'Android NDK / ESP-IDF 嵌入式开发',
      'Wasmtime/wasm3 跨WASM运行时',
    ],
    user_instruction_refs: [
      '用户原话: "完全自主设计的统一指令集架构，支持多语言协作开发"',
      '用户原话: "AI层Python意图分解+Go并发加速+Rust安全包装+Zig核心驱动"',
      '用户原话: "云端同步GitHub Gist + WebDAV坚果云 + P2P WebRTC 多通道"',
      '用户原话: "跨平台Web/Android/ESP32/Linux/Windows/Mac 同一WASM字节码运行"',
      '用户原话: "别人的软件不好用，专注写我们自己的软件吧——让一切的记忆都明朗起来，任务串联起来关联起来"',
      '用户原话: "最底层的优先级的工作任务，技能化时序，分布式切片节点化写进我们自己的项目仓库"',
    ],
    deliverables: [
      '4层语言贯通: Zig(核心+翻译器)/Rust(安全+FFI)/Go(并发+云同步)/Python(AI)',
      'UniCoreSyncManager: Gist+WebDAV+P2P 3通道配置+调度',
      'Web端: 浏览器原生WASM打开即用',
      'Android端: Wasmtime-Android打包APK ~5MB',
      'ESP32端: wasm3-micro + ESP-IDF编译固件 ~50KB',
      'Linux/Win/Mac 3端Wasmtime可执行各~10MB',
      '最终闭环: P0~P7全链路，3端互通同一份.3d.cbc.mll',
      '本任务规划仓库交付: global_anchors / task_nodes / skill_timeline / roadmap 全部文档代码',
    ],
    entry_criteria: ['T4+T5+T6全部交付'],
    exit_criteria: [
      '构建脚本6平台全通过',
      '云端同步三通道各自roundtrip',
      '三平台(Web/Android/ESP32)之间互通数据成功',
      '本任务规划仓库可以正确查询所有冠名项和任务依赖',
    ],
    bindings: ['unicore_architecture_root', 'lang_zig', 'lang_rust', 'lang_go', 'lang_python', 'cloud_sync', 'wasm_runtime', 'task_planning_repo', 'global_memory_anchor'],
    depends_on_stages: ['T4', 'T5', 'T6'],
  },
];

// ============================================================
// 最优秀指令索引 (用户原话精华摘录)
// ============================================================

const GREATEST_INSTRUCTIONS = [
  {
    id: 'GI_001',
    quote: '我们的声母、我们的韵母、我们的声调本身就是进制...',
    context: '拒绝传统二进制/十进制，自有用声韵调做进制的根本',
    impact: 'T0~T5 所有底层设计的灵魂',
  },
  {
    id: 'GI_002',
    quote: '按税率效率最大，总是数量最多的那个优先。数量少的那个排后。',
    context: '确定优先级排序：声调170 > 韵母64 > 声母48 > 数字17',
    impact: 'T1 集装箱编码和进位顺序',
  },
  {
    id: 'GI_003',
    quote: '前三维永远锁定(0,0,0)对接机器码01，永不进位变大；第四维唯一能增长，通过等效维度映射和前三维交互。',
    context: '四维不进位体系的核心公理',
    impact: 'T1 四维系统与机器码对接机制',
  },
  {
    id: 'GI_004',
    quote: '按小学生拼音韵母标准顺口溜排序——完全沿用符号、一个不改，只重排；ueng固定౨/ⱂ。',
    context: '韵母32位顺序最终定稿',
    impact: 'T0 韵母64个符号的顺序和大小写对应',
  },
  {
    id: 'GI_005',
    quote: '稀疏数代数：小集合表示大集合，后面少的代表前面多的。数字→声母→韵母→声调，链上去。',
    context: '稀疏数映射方向',
    impact: 'T3 稀疏代数系统',
  },
  {
    id: 'GI_006',
    quote: '旧格式不行，会被别人抄袭，要有私有魔数+动态混淆+完全自有格式(.3d.cbc.mll)。',
    context: '知识产权保护',
    impact: 'T4 Guardian + CBC格式',
  },
  {
    id: 'GI_007',
    quote: 'UniISA v2.0 三个灵魂：不进位计数 / 节点因果时间 / 集装箱内存。',
    context: '抛弃传统时钟驱动CPU模型',
    impact: 'T5 指令集和虚拟机',
  },
  {
    id: 'GI_008',
    quote: 'HTML/Vue等全是二维平面，我要的是三维立体。0=母包含1=子，点击进入，函子映射，大千世界。',
    context: 'UI交互范式的根本转变',
    impact: 'T6 表格系统和3D引擎',
  },
  {
    id: 'GI_009',
    quote: '阶(19种)/互认(18项)/阵法(15种)/聚类(5大类)/模块锚点(10个)——全部要系统地表达出来。',
    context: '纯符号系统的8大子系统命名',
    impact: 'T2 纯符号核心系统',
  },
  {
    id: 'GI_010',
    quote: '最底层优先级最高的工作任务，技能化时序、分布式切片节点化，写入项目仓库；把过去所有记忆、指令、目标全部找回，冠名关联起来，做成全局冠名项，让一切记忆都明朗起来。',
    context: '本任务规划仓库本身的存在意义',
    impact: '当前交付物 planning/ 目录的全部存在理由',
  },
];

// ============================================================
// 导出
// ============================================================

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    SKILL_TIMELINE,
    GREATEST_INSTRUCTIONS,
  };
} else {
  window.UniCoreSkillTimeline = {
    SKILL_TIMELINE,
    GREATEST_INSTRUCTIONS,
  };
}

const totalDays = SKILL_TIMELINE.reduce((s, t) => s + t.duration_days, 0);
const totalSkills = new Set(SKILL_TIMELINE.flatMap(t => t.skill_bundle)).size;
console.log(`[Skill Timeline] 技能化时序加载完成:
  → 8个阶段(T0~T7), 按最底层→最高层顺序
  → 覆盖 ${SKILL_TIMELINE.flatMap(t=>t.user_instruction_refs).length} 条历史用户原话
  → 涉及 ${totalSkills} 种不同技能
  → 预估总工期: ${totalDays} 天 (${Math.round(totalDays/5*10)/10} 周)
  → 最优秀指令 GI_001~GI_010 ${GREATEST_INSTRUCTIONS.length} 条精华语录已归档
`);
