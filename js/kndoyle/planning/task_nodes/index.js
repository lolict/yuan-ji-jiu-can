/**
 * ============================================================
 * 分布式切片任务节点系统
 * ============================================================
 * 
 * 设计原则：
 * 1. 节点化：每个任务是一个独立的 TaskNode，可独立调度/执行/回滚
 * 2. 分布式：节点之间通过 depends_on / produces 关联，可并行执行无依赖节点
 * 3. 切片化：大任务拆分为微小的时间切片，每个切片5~30分钟可完成
 * 4. 时序化：每个节点带有 earliest_start / deadline / estimated_hours
 * 5. 技能化：每个节点标注所需 skills，可分配给对应技能执行者
 * 
 * 节点状态流转：
 *   pending → ready(依赖满足) → running → completed / failed → rollback(可选)
 * ============================================================
 */

const TASK_NODES = {
  // ==========================================================
  // Phase 0: 底层符号与编码系统 (最底层优先级)
  // ==========================================================

  'node_symbol_001': {
    id: 'node_symbol_001',
    title: '统一48声母符号序列(24小写+24大写)最终固化',
    phase: 'P0-底层符号集',
    slice_size: 'S',
    estimated_minutes: 20,
    priority: 1,
    status: 'pending',
    skills: ['符号系统', '字符编码', 'Unicode'],
    description: '把 unicore_architecture.js / unicore_pure_symbol.js / unicore_symbols.js 中的声母定义全部对齐到 unicore_symbols.js 的最终版本：\n小写: b p m f d t n l g k h w j q x y z c s r ȥ ç ş ŋ (24个)\n大写: B P M F D T N L G K H W J Q X Y Z C S R Ȥ Ç Ş Ŋ (24个)\n总计48个。',
    anchor_bindings: ['radix_shengmu', 'container_1', 'pure_symbol_system'],
    depends_on: [],
    produces: ['node_symbol_002', 'node_symbol_003', 'node_containers_001'],
    acceptance_criteria: [
      '所有core目录文件SHENGMU数组完全一致',
      'SHENGMU.length === 48',
      'SHENGMU = [...SHENGMU_LOWER, ...SHENGMU_UPPER] 模式统一',
    ],
    rollback_hint: '回退到 unicore_symbols.js 版本',
  },

  'node_symbol_002': {
    id: 'node_symbol_002',
    title: '统一64韵母符号序列最终固化(顺口溜排序+固定大小写对应)',
    phase: 'P0-底层符号集',
    slice_size: 'S',
    estimated_minutes: 30,
    priority: 2,
    status: 'pending',
    skills: ['符号系统', '小学拼音顺口溜', 'Unicode特殊字符'],
    description: '韵母按小学顺口溜顺序：ai ei ui → ao ou iu → ie üe er → an en in un ün → ang eng ing ong → ia ua uo iao uai ian uan üan iang uang ueng iong iai oi。\n32大写在前，32小写在后 = 总计64个。\n关键校验：ueng固定=౨/ⱂ',
    anchor_bindings: ['radix_yunmu', 'container_2', 'shunliu_root', 'ym_upper_ⱂ', 'ym_lower_౨'],
    depends_on: ['node_symbol_001'],
    produces: ['node_symbol_003', 'node_grid_001'],
    acceptance_criteria: [
      'YUNMU.length === 64',
      'YUNMU = [...YUNMU_UPPER, ...YUNMU_LOWER]，各32个',
      '顺口溜顺序严格对应',
      'YUNMU_UPPER[28] === "ⱂ"（ueng大写）',
      'YUNMU_LOWER[28] === "౨"（ueng小写）',
    ],
  },

  'node_symbol_003': {
    id: 'node_symbol_003',
    title: '统一170声调生成算法固化(85小写+85大写交替生成)',
    phase: 'P0-底层符号集',
    slice_size: 'XS',
    estimated_minutes: 15,
    priority: 3,
    status: 'pending',
    skills: ['符号系统', '生成算法'],
    description: 'SHENGDIAO数组长度固定170。前85=小写字母+an+n，后85=大写字母+An+n。letters a-z循环，每个字母配n=1~9直到满85。',
    anchor_bindings: ['radix_shengdiao', 'container_3'],
    depends_on: ['node_symbol_001', 'node_symbol_002'],
    produces: ['node_containers_001', 'node_sparse_001'],
    acceptance_criteria: ['SHENGDIAO.length === 170', '前85以小写字母开头，后85以大写字母开头'],
  },

  // ==========================================================
  // Phase 1: 集装箱编码 & 四维不进位体系 (次底层)
  // ==========================================================

  'node_containers_001': {
    id: 'node_containers_001',
    title: '集装箱编码系统ContainerEncoder单测通过',
    phase: 'P1-集装箱体系',
    slice_size: 'S',
    estimated_minutes: 25,
    priority: 4,
    status: 'pending',
    skills: ['单元测试', '进制本身直接操作（非编码）'],
    description: '确保 集装箱1=1、集装箱2=2、集装箱3=3、集装箱4=4，有一说一不做多余的编码/索引/映射。进制本身直接increment按税率效率优先：声调→韵母→声母→数字。',
    anchor_bindings: ['container_self_radix', 'container_1', 'container_2', 'container_3', 'container_4', 'priority_by_count', 'no_redundant_mapping'],
    depends_on: ['node_symbol_001', 'node_symbol_002', 'node_symbol_003'],
    produces: ['node_fourdim_001'],
    acceptance_criteria: ['node test_containers.js pass', 'increment进位链顺序正确priority_by_count', '集装箱号=自己，不出现001/010/100这种高位零补位'],
  },

  'node_fourdim_001': {
    id: 'node_fourdim_001',
    title: '四维不进位体系核心验证:前三维永远[0,0,0]不进位',
    phase: 'P1-集装箱体系',
    slice_size: 'S',
    estimated_minutes: 20,
    priority: 5,
    status: 'pending',
    skills: ['系统验证', '进制逻辑（不做编码，有一说一）'],
    description: '四维系统:前三维永久锁定(1,2,3)三个集装箱号自己（⚠️ 禁止用000！0=前导零=负存在=-100）。increment() 1000次后 staticFrame 依然是[1,2,3]，仅仅 dynamicValue 从4开始涨到1004。等效映射getEquivalentMapping()产生层级对齐。',
    anchor_bindings: ['four_dim_system', 'static_frame_123', 'dynamic_value_4', 'equivalent_mapping', 'leading_zero_negative'],
    depends_on: ['node_containers_001'],
    produces: ['node_machinecode_001'],
    acceptance_criteria: [
      '循环1000次increment,staticFrame=[1,2,3]永久不变（不是000）',
      'dynamicValue从4开始正确递增到1004',
      '等效层级计算正确',
      '没有任何地方用0开头表示集装箱号（即没有001、010、100这种高位零）',
    ],
  },

  'node_machinecode_001': {
    id: 'node_machinecode_001',
    title: '集装箱↔机器码01对接:ContainerMachineCodeAdapter集成测试',
    phase: 'P1-集装箱体系',
    slice_size: 'M',
    estimated_minutes: 40,
    priority: 6,
    status: 'pending',
    skills: ['系统集成', '二进制', '硬件直接对接（不做编码适配）'],
    description: '直接对接（非编码适配）：机器码原生0→集装箱1/3；机器码原生1→集装箱2/4。集装箱1/3→0，2/4→1。\n⚠️ 形成前三维(1,2,3)永久直接对接机器码原生0/1的完整闭环！机器码的0/1是芯片自带的，我们自己不用0作为前缀（那是负存在=-100）。',
    anchor_bindings: ['container_machine_direct', 'machinecode_0', 'machinecode_1', 'static_frame_123', 'leading_zero_negative'],
    depends_on: ['node_fourdim_001'],
    produces: ['node_foundation_001', 'node_cbc_001'],
    acceptance_criteria: ['双向直接对接一致', 'getMachineCodeMapping()输出staticFrameBits="1,2,3"（不是000）'],
  },

  // ==========================================================
  // Phase 2: 纯符号核心 (3D格子+互认+阶系+阵法+聚类)
  // ==========================================================

  'node_grid_001': {
    id: 'node_grid_001',
    title: '三维格子PureGrid 21固定位置映射固化(1中心+6墙+8内角+6外连)',
    phase: 'P2-纯符号核心',
    slice_size: 'S',
    estimated_minutes: 30,
    priority: 7,
    status: 'pending',
    skills: ['三维建模', '数据结构'],
    description: '1中心=YUNMU[0]，6墙=YUNMU[1-6]，8内角=YUNMU[7-14]，6外连=YUNMU[15-20]，剩余YUNMU[21-63]=十字架节点。确保write/read/activate/connect四个方法全部覆盖。',
    anchor_bindings: ['pure_grid', 'grid_center', 'grid_walls', 'grid_inner_corners', 'grid_outer_connections', 'grid_cross_nodes'],
    depends_on: ['node_symbol_002'],
    produces: ['node_huren_001', 'node_cbc_001'],
    acceptance_criteria: [
      '1+6+8+6=21 核心位置正确映射到YUNMU[0-20]',
      '剩余YUNMU[21-63]共43个正确归为cross_nodes',
      'write/read roundtrip正确',
    ],
  },

  'node_huren_001': {
    id: 'node_huren_001',
    title: '互认系统PureHuren:18项互认与权级高低判定贯通',
    phase: 'P2-纯符号核心',
    slice_size: 'M',
    estimated_minutes: 45,
    priority: 8,
    status: 'pending',
    skills: ['逻辑系统', '关系推理'],
    description: '互认接口/权位/格子/节点/墙面/集装箱/阴阳/关系/进制/对齐/身份/问题/目标/基因/钥匙/门锁/因果律。\nhuren(a,b)先比声母，再比韵母，最后平等。输出higher/lower/equal。',
    anchor_bindings: ['pure_huren', 'huren_jiekou', 'huren_quanwei', 'huren_mensuo', 'huren_duiqi'],
    depends_on: ['node_grid_001'],
    produces: ['node_jie_001'],
    acceptance_criteria: ['18项互认冠名项全部bind覆盖', '权级比较逻辑nativeOrder正确'],
  },

  'node_jie_001': {
    id: 'node_jie_001',
    title: '19阶系PureJie实现贯通:阶乘/阶升/阶降/阶逆/阶顺/阶叠/阶渐变等',
    phase: 'P2-纯符号核心',
    slice_size: 'L',
    estimated_minutes: 60,
    priority: 9,
    status: 'pending',
    skills: ['抽象代数', '系统设计'],
    description: '阶和/角/除/位/权/属/集/序/乘/岔/段/叠/逆/顺/降/升/渐变/稀疏数律/范畴。\njieCheng(声韵调)=声调进位→韵母→声母→双重叠合。jieSheng=爬升到最后声母，jieJiang=下降到最声母。',
    anchor_bindings: ['pure_jie', 'jie_he', 'jie_cheng', 'jie_sheng', 'jie_jiang', 'jie_ni', 'jie_shun', 'jie_die'],
    depends_on: ['node_huren_001'],
    produces: ['node_zhenfa_001', 'node_modules_001'],
    acceptance_criteria: ['19阶全部有对应方法', '阶乘进位链声调→韵母→声母→double'],
  },

  'node_zhenfa_001': {
    id: 'node_zhenfa_001',
    title: '15种阵法PureZhenfa实现贯通:锥/球/蜂窝/门/锁/镜像/对立统一/各种认知阵',
    phase: 'P2-纯符号核心',
    slice_size: 'L',
    estimated_minutes: 60,
    priority: 10,
    status: 'pending',
    skills: ['空间思维', '拓扑', '阵法逻辑'],
    description: '锥阵(中心+多层每层3格并联)、球阵(全局索引)、蜂窝阵(6邻)、门阵(上墙=锁下墙=locked)、镜像阵(两格互为前墙内容)、三角阵(因果闭环)。',
    anchor_bindings: ['pure_zhenfa', 'zhenfa_zhui', 'zhenfa_men', 'zhenfa_suo', 'zhenfa_jingxiang', 'zhenfa_duili_tongyi'],
    depends_on: ['node_jie_001'],
    produces: ['node_modules_001'],
    acceptance_criteria: ['zhuiZhen(center, 3) 层1=3格层2=9格层3=27格联通', '门阵menZhen正确填充上墙下墙', '镜像阵jingxiangZhen双向connect'],
  },

  'node_modules_001': {
    id: 'node_modules_001',
    title: '10个模块锚点PureModules激活/绑定/聚类算法贯通',
    phase: 'P2-纯符号核心',
    slice_size: 'M',
    estimated_minutes: 40,
    priority: 11,
    status: 'pending',
    skills: ['模块设计', '聚类算法'],
    description: '锚点:mll(主聚类)/ll(层级链)/ct(位权)/gtt(几何拓扑)/xy(空间坐标)/gy(阴阳对立)/mrror(镜像)/cbc(汉字编码)/mrror.mll(自纠错聚类)/cbc.mll(编码挂靠聚类)。mllCluster()聚类函数正确按feature分桶。',
    anchor_bindings: ['pure_modules', 'mll_anchor', 'll_anchor', 'ct_anchor', 'gtt_anchor', 'mrror_mll', 'cbc_mll'],
    depends_on: ['node_jie_001', 'node_zhenfa_001'],
    produces: ['node_cbc_001'],
    acceptance_criteria: ['10个锚点全部可激活可绑定', 'mllCluster分桶正确'],
  },

  // ==========================================================
  // Phase 3: CBC汉字积木 & 稀疏数代数
  // ==========================================================

  'node_cbc_001': {
    id: 'node_cbc_001',
    title: 'PureCBC汉字积木完整填格:1中心+6墙+8内角+6外连=21位全部有含义字段',
    phase: 'P3-CBC编码体系',
    slice_size: 'M',
    estimated_minutes: 45,
    priority: 12,
    status: 'pending',
    skills: ['汉字学', '编码', '数据结构'],
    description: 'storeChar()写入：\n中心=汉字本体\n前墙=声母/后墙=韵母/左墙=声调/右墙=笔画/上墙=偏旁/下墙=部首\n8内角:本义/演化/因果/对错/阴阳/亲疏/边界/最后1空\n6外连:同义词/反义词/古文/关联字/衍生字/最后1空',
    anchor_bindings: ['pure_cbc', 'cbc_char_shengmu', 'cbc_char_yunmu', 'cbc_char_shengdiao', 'cbc_char_radical', 'cbc_char_component'],
    depends_on: ['node_grid_001', 'node_modules_001', 'node_machinecode_001'],
    produces: ['node_foundation_001', 'node_guardian_001'],
    acceptance_criteria: ['traceback()返回所有21字段非空样本', 'retrieveChar(声,调)能找到汉字'],
  },

  'node_sparse_001': {
    id: 'node_sparse_001',
    title: '稀疏数代数SparseNumberSystem:数字→声母→韵母→声调 映射构建',
    phase: 'P3-CBC编码体系',
    slice_size: 'S',
    estimated_minutes: 30,
    priority: 13,
    status: 'pending',
    skills: ['抽象代数', '映射算法'],
    description: 'buildSparseMapping()：1个数字代表多个声母，1声母代表多韵母，1韵母代表多声调。权位计算getWeight()声调最高。稀疏数sparseAdd()取权位高者。getSymbolChain()追踪整个映射链。',
    anchor_bindings: ['sparse_algebra', 'sparse_mapping', 'sparse_weight', 'sparse_add', 'jie_xishu'],
    depends_on: ['node_symbol_003'],
    produces: ['node_foundation_001'],
    acceptance_criteria: ['数字→声母映射分桶大致平均', 'getWeight(sd, shengdiao) > getWeight(ym, yunmu)'],
  },

  // ==========================================================
  // Phase 4: Foundation底层核心 & .3d.cbc.mll格式
  // ==========================================================

  'node_foundation_001': {
    id: 'node_foundation_001',
    title: 'Foundation CBCParser解析器:encode/decode/.3d.cbc.mll二进制读写贯通',
    phase: 'P4-底层Foundation',
    slice_size: 'L',
    estimated_minutes: 60,
    priority: 14,
    status: 'pending',
    skills: ['二进制解析', '文件格式（进制本身直接装数据，非编码）', '编译构建'],
    description: 'CBC_MAGIC=[0x3D,0x43,0x42,0x43]=.3dCBC。CBC集装箱系统：用集装箱1/2/3/4进制本身直接装汉字数据（⚠️ 不是编码），每个集装箱号自己=1/2/3/4，有一说一。SparseAlgebra和UniCoreVM集成。WASIAdapter可编译到WASM。',
    anchor_bindings: ['unicore_foundation', 'cbc_self_container', 'cbc_magic', 'cbc_format_spec', 'container_self_radix'],
    depends_on: ['node_cbc_001', 'node_sparse_001', 'node_machinecode_001'],
    produces: ['node_format_001', 'node_guardian_001', 'node_isa_001'],
    acceptance_criteria: [
      'encode→parse roundtrip 完整',
      'magic bytes正确',
      'grid结构按规范正确',
      'unicore_foundation_wasm.js通过基础测试',
    ],
  },

  'node_format_001': {
    id: 'node_format_001',
    title: '.3d.cbc.mll v2.0格式6大数据段完整实现',
    phase: 'P4-底层Foundation',
    slice_size: 'L',
    estimated_minutes: 75,
    priority: 15,
    status: 'pending',
    skills: ['文件格式', '二进制', '序列化'],
    description: '段1 RADIX进制配置 / 段2 CHARSET字符集映射 / 段3 CONTAINER集装箱256数据+位置+flags / 段4 SPATIAL空间阵列(锥/球/蜂窝/矩阵/波浪) / 段5 TIMENODE时间节点因果链 / 段6 PLUGIN插件配置。尾部END段+校验。',
    anchor_bindings: ['cbc_format_spec', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao', 'zhenfa_zhui', 'node_time_system', 'isa_plugin'],
    depends_on: ['node_foundation_001'],
    produces: ['node_vm_001'],
    acceptance_criteria: ['6种段类型都能写入读出一致', '段头8字节(2+2+4)+段数据正确'],
  },

  'node_guardian_001': {
    id: 'node_guardian_001',
    title: 'Guardian v5.0知识产权守护:动态混淆+私有魔数+生态闭环',
    phase: 'P4-底层Foundation',
    slice_size: 'M',
    estimated_minutes: 45,
    priority: 16,
    status: 'pending',
    skills: ['安全', '加密混淆', '知识产权'],
    description: 'generateMagic()由48×64%256 / 170%256 / XOR三者生成自定义魔数。generateObfuscation() Fisher-Yates洗牌+hash对每个符号混淆。\n5大类聚类+9种阵法+5级宗族关系。export()/import()私有格式闭环。',
    anchor_bindings: ['unicore_guardian', 'dynamic_obfuscation', 'cbc_magic', 'guardian_clusters', 'guardian_formations', 'guardian_relations'],
    depends_on: ['node_cbc_001', 'node_foundation_001'],
    produces: ['node_isa_001'],
    acceptance_criteria: [
      '两次new UniCoreGuardian()产生不同obfuscationMap',
      'export→import roundtrip 正确',
      '魔数识别verifyMagic()拒绝其他格式',
    ],
  },

  // ==========================================================
  // Phase 5: UniISA v2.0 指令集 + 虚拟机
  // ==========================================================

  'node_isa_001': {
    id: 'node_isa_001',
    title: 'UniISA v2.0指令集5大类完整实现: 基础/不进位/节点时间/空间/插件',
    phase: 'P5-指令集&VM',
    slice_size: 'L',
    estimated_minutes: 75,
    priority: 17,
    status: 'pending',
    skills: ['CPU架构', '指令集', '虚拟机'],
    description: '基础:NOP/HALT/LOAD/STORE/MOVE/COPY\n不进位:INC/DEC/ADD/SUB/ZERO/MAX/FLIP\n节点时间:CREATE_NODE/LINK_NODE/TRIGGER/WAIT_NODE/SYNC_TIME/GET_OFFSET\n空间:SET_POS/GET_POS/MOVE_TO/DISTANCE/ROTATE/PROJECT\n插件:LOAD_PLUGIN/UNLOAD_PLUGIN/CALL_PLUGIN/LIST_PLUGIN\n0xA5特殊系统子命令。',
    anchor_bindings: ['uniisa_v2', 'isa_basic', 'isa_nocarry', 'isa_nodetime', 'isa_spatial', 'isa_plugin', 'sys_0xA5'],
    depends_on: ['node_foundation_001', 'node_guardian_001'],
    produces: ['node_vm_001', 'node_timenode_001'],
    acceptance_criteria: ['5组指令opcode互不冲突', 'zig build uniisa.zig通过'],
  },

  'node_timenode_001': {
    id: 'node_timenode_001',
    title: '节点因果时间系统替代传统时钟:UniTick+事件驱动模型',
    phase: 'P5-指令集&VM',
    slice_size: 'M',
    estimated_minutes: 45,
    priority: 18,
    status: 'pending',
    skills: ['事件驱动', '因果建模', '时间系统'],
    description: '硬件时钟→scale_factor映射→UniTick自定义单位。节点驱动:等待触发→因果传播→指令执行→等待新节点。\nCREATE_NODE→LINK_NODE(因果链)→TRIGGER触发整条链→WAIT等待条件→SYNC_TIME校准。',
    anchor_bindings: ['node_time_system', 'uni_tick', 'node_cause_chain', 'isa_nodetime', 'zhenfa_san'],
    depends_on: ['node_isa_001'],
    produces: ['node_vm_001'],
    acceptance_criteria: ['三角因果链A→B→C→A触发闭环', 'scale_factor=2时间压缩正常'],
  },

  'node_vm_001': {
    id: 'node_vm_001',
    title: 'UniCoreVM虚拟机端到端运行:符号程序→指令→集装箱→格子→输出',
    phase: 'P5-指令集&VM',
    slice_size: 'XL',
    estimated_minutes: 90,
    priority: 19,
    status: 'pending',
    skills: ['虚拟机', '端到端测试', '系统集成（进制本身操作，非编码）'],
    description: '端到端流程: 输入符号序列(自己就是指令)→集装箱进制本身直接组合→UniCoreVM循环execute→堆栈/内存操作→PureGrid填充→阵法激活→输出。\n⚠️ 全程禁止用0开头的高位零表示任何集装箱号（那是负存在=-100）。\n汇编样例: 排序.asm / 阶乘.asm / 斐波那契.asm 三个样例全部跑通。',
    anchor_bindings: ['unicore_vm', 'unicore_instructions', 'container_self_radix', 'pure_grid', 'leading_zero_negative'],
    depends_on: ['node_format_001', 'node_isa_001', 'node_timenode_001'],
    produces: ['node_ui_001', 'node_multi_001'],
    acceptance_criteria: [
      'test_vm.js全部pass',
      '三个样例.asm正确输出',
      '阵法指令(锥/球/镜像)正确触发日志',
    ],
  },

  // ==========================================================
  // Phase 6: UI/3D引擎/表格系统
  // ==========================================================

  'node_ui_001': {
    id: 'node_ui_001',
    title: 'CubeTable里外嵌套表格+三维渲染+自动填充0包含1贯通',
    phase: 'P6-UI/3D层',
    slice_size: 'L',
    estimated_minutes: 60,
    priority: 20,
    status: 'pending',
    skills: ['前端', 'Three.js', '三维可视化', '数据结构'],
    description: 'CubeTable(0母1子)+InsideOutsideMapper(自动填充0包含1的子表格)+CubeTableRenderer(Three.js三维渲染)+TableContainer(导入导出)。\n点击0符号=自动toggle展开子表格。嵌套层级支持6层以上。',
    anchor_bindings: ['cube_table', 'inside_outside_mapper', 'cube_table_renderer', 'pure_grid', 'jie_die'],
    depends_on: ['node_vm_001'],
    produces: ['node_system_001'],
    acceptance_criteria: ['web/ cube_table_demo.html 三层嵌套可展开', 'autoFillZeros()自动给0加1子'],
  },

  'node_hacsjs_001': {
    id: 'node_hacsjs_001',
    title: 'HACSJS 3D引擎:256集装箱/空间阵列/节点链/字符集/进制5大功能贯通',
    phase: 'P6-UI/3D层',
    slice_size: 'L',
    estimated_minutes: 75,
    priority: 21,
    status: 'pending',
    skills: ['前端', 'WebGL/Three.js', '三维引擎'],
    description: 'createAllContainers(256)创建矩阵，createSpatialArray(type)锥/球/蜂窝/矩阵，createNodeChain([...])建立节点连接，plugable charset/radix切换自定义符号。',
    anchor_bindings: ['hacsjs_3d', 'zhenfa_zhui', 'zhenfa_feng', 'container_1'],
    depends_on: ['node_vm_001'],
    produces: ['node_system_001'],
    acceptance_criteria: ['web/hacsjs_3d_demo.html 5大功能按钮全可用'],
  },

  'node_system_001': {
    id: 'node_system_001',
    title: 'UniCoreSystem v2.0总控:WASM+3D+Table+CBC+Plugin 5合1整合',
    phase: 'P6-UI/3D层',
    slice_size: 'XL',
    estimated_minutes: 90,
    priority: 22,
    status: 'pending',
    skills: ['系统架构', '整合集成'],
    description: 'UniCoreSystem整合:vm(WASM层)+engine3d+tableSystem(逻辑)+pluginSystem(扩展)+fileHandler(CBC格式)。\nexecuteInstruction()分派内置/插件/0xA5。getStatus()查看运行状态，exportData()/importData()状态持久化。',
    anchor_bindings: ['unicore_system_core', 'sys_0xA5', 'unicore_plugin_system', 'cbc_format_spec'],
    depends_on: ['node_ui_001', 'node_hacsjs_001'],
    produces: ['node_multi_001', 'node_cloud_001'],
    acceptance_criteria: ['unicore_complete.html 加载后初始化成功所有子模块', '0xA5子命令INIT_3D/INIT_TABLE/SYNC全可用'],
  },

  // ==========================================================
  // Phase 7: 多语言协作 / 云端同步 / 跨平台
  // ==========================================================

  'node_multi_001': {
    id: 'node_multi_001',
    title: 'Zig+Rust+Go+Python 4层语言协作架构贯通',
    phase: 'P7-多语言/云/跨平台',
    slice_size: 'XL',
    estimated_minutes: 120,
    priority: 23,
    status: 'pending',
    skills: ['多语言FFI', '架构整合', '构建系统'],
    description: 'Zig核心：uniisa.zig+translator.zig（二进制翻译x86/ARM/RISC-V→UniISA）\nRust安全：safety.rs内存安全+FFI绑定\nGo并发：concurrent.go并行+网络+调度\nPython AI：ai_core.py意图理解/任务分解/推理/多模态\n5层架构总览对齐SPEC.md。',
    anchor_bindings: ['unicore_architecture_root', 'lang_zig', 'lang_rust', 'lang_go', 'lang_python', 'unicore_translator'],
    depends_on: ['node_vm_001', 'node_system_001'],
    produces: ['node_cloud_001', 'node_platform_001'],
    acceptance_criteria: [
      'zig build 通过',
      'Rust cargo build（safety.rs）通过',
      'Go build concurrent.go通过',
      'python ai_core.py 基础import通过',
    ],
  },

  'node_cloud_001': {
    id: 'node_cloud_001',
    title: '云端同步三通道: GitHub Gist + WebDAV坚果云 + WebRTC P2P',
    phase: 'P7-多语言/云/跨平台',
    slice_size: 'L',
    estimated_minutes: 75,
    priority: 24,
    status: 'pending',
    skills: ['网络编程', '云同步', 'WebRTC'],
    description: 'UniCoreSyncManager统一调度器，三通道配置独立。\ngist_sync.py:通过ghp_token上传下载配置\nwebdav_sync.py:坚果云email+password登录\np2p_webrtc.py:节点发现+数据通道直接传',
    anchor_bindings: ['cloud_sync', 'cloud_gist', 'cloud_webdav', 'cloud_p2p', 'cloud_scheduler'],
    depends_on: ['node_multi_001', 'node_system_001'],
    produces: ['node_platform_001'],
    acceptance_criteria: [
      'gist_sync上传+读取roundtrip',
      'webdav_sync上传+读取roundtrip',
      'p2p两个节点直接互通消息',
    ],
  },

  'node_platform_001': {
    id: 'node_platform_001',
    title: '跨平台打包贯通: Web/Android/ESP32/Linux/Win/Mac 6平台WASM运行',
    phase: 'P7-多语言/云/跨平台',
    slice_size: 'XL',
    estimated_minutes: 120,
    priority: 25,
    status: 'pending',
    skills: ['嵌入式', 'Android', '跨平台构建', 'WASM'],
    description: 'Web: 原生WASM + browser\nAndroid: Wasmtime-Android 打包APK ~5MB\nESP32: wasm3-micro 编译固件 ~50KB sdkconfig\nLinux/Win/Mac: Wasmtime 本地可执行 ~10MB\n所有6平台都能跑同一个.wasm字节码。',
    anchor_bindings: ['wasm_runtime', 'platform_web', 'platform_android', 'platform_esp32', 'platform_linux', 'platform_windows', 'platform_macos'],
    depends_on: ['node_multi_001', 'node_cloud_001'],
    produces: ['node_final_integration'],
    acceptance_criteria: [
      '浏览器打开web/index.html可用',
      'ESP32 idf.py build 通过生成bin',
      'Android gradle build生成apk',
      'Linux ./unicore 可执行',
    ],
  },

  // ==========================================================
  // Phase 8: 最终整合/闭环 (最高层优先级)
  // ==========================================================

  'node_final_integration': {
    id: 'node_final_integration',
    title: '终极闭环贯通: 符号→进制→集装箱→四维→纯符号→CBC→Foundation→ISA→VM→UI→多语言→云→跨平台',
    phase: 'P8-终极整合',
    slice_size: 'XXL',
    estimated_minutes: 180,
    priority: 999,
    status: 'pending',
    skills: ['全栈', '系统思维', '集成测试'],
    description: '完整链条：用户输入汉字→PureCBC编码进3D格子→集装箱封装→四维不进位映射→机器码01对齐→Foundation编码成.3d.cbc.mll→Guardian混淆加密→云端Gist备份→Web端HACSJS 3D渲染→Android/ESP32同一份WASM字节码运行→Go并发同步→Python AI分析→Zig核心执行→Rust安全校验。',
    anchor_bindings: ['unicore_root', 'global_memory_anchor', 'task_planning_repo'],
    depends_on: ['node_platform_001'],
    produces: [],
    acceptance_criteria: [
      '整条链路每个阶段有可观测输出',
      '跨三平台(Web/Android/ESP32)数据互通',
      'README.md/SPEC.md所有示例均可复现',
      '全仓build.zig构建通过',
    ],
  },
};

// ============================================================
// 任务节点调度器 (分布式切片调度)
// ============================================================

class TaskNodeScheduler {
  constructor(nodes) {
    this.nodes = JSON.parse(JSON.stringify(nodes)); // 深拷贝
    this.history = [];
  }

  /**
   * 获取可立即执行的节点(所有deps已completed)
   */
  getReadyNodes() {
    return Object.values(this.nodes).filter(n => {
      if (n.status !== 'pending' && n.status !== 'failed') return false;
      return n.depends_on.every(depId => {
        const dep = this.nodes[depId];
        return dep && dep.status === 'completed';
      });
    }).sort((a, b) => a.priority - b.priority); // 优先级数字小=先执行
  }

  /**
   * 标记节点完成
   */
  markCompleted(nodeId) {
    if (!this.nodes[nodeId]) return false;
    this.history.push({
      time: new Date().toISOString(),
      nodeId,
      action: 'completed',
    });
    this.nodes[nodeId].status = 'completed';
    return true;
  }

  markFailed(nodeId, reason) {
    if (!this.nodes[nodeId]) return false;
    this.history.push({
      time: new Date().toISOString(),
      nodeId,
      action: 'failed',
      reason,
    });
    this.nodes[nodeId].status = 'failed';
    this.nodes[nodeId].fail_reason = reason;
    return true;
  }

  /**
   * 计算整体进度
   */
  getProgress() {
    const total = Object.keys(this.nodes).length;
    const done = Object.values(this.nodes).filter(n => n.status === 'completed').length;
    const failed = Object.values(this.nodes).filter(n => n.status === 'failed').length;
    const running = Object.values(this.nodes).filter(n => n.status === 'running').length;
    const pending = total - done - failed - running;
    const totalEstimatedMin = Object.values(this.nodes).reduce((s, n) => s + (n.estimated_minutes || 0), 0);
    const doneEstimatedMin = Object.values(this.nodes).filter(n => n.status === 'completed').reduce((s, n) => s + (n.estimated_minutes || 0), 0);
    return {
      total, done, failed, running, pending,
      percent: total > 0 ? Math.round(done / total * 1000) / 10 : 0,
      totalEstimatedHours: Math.round(totalEstimatedMin / 60 * 10) / 10,
      doneEstimatedHours: Math.round(doneEstimatedMin / 60 * 10) / 10,
      remainingEstimatedHours: Math.round((totalEstimatedMin - doneEstimatedMin) / 60 * 10) / 10,
    };
  }

  /**
   * 输出技能甘特图时序概览 (按phase分组)
   */
  getTimelineByPhase() {
    const phases = {};
    for (const node of Object.values(this.nodes)) {
      const p = node.phase || '未分类';
      if (!phases[p]) phases[p] = [];
      phases[p].push({
        id: node.id,
        title: node.title,
        priority: node.priority,
        slice_size: node.slice_size,
        minutes: node.estimated_minutes,
        status: node.status,
        skills: node.skills,
      });
    }
    // 按phase名排序 P0 < P1 < ... < P8
    return Object.keys(phases).sort().map(k => ({
      phase: k,
      nodes: phases[k].sort((a, b) => a.priority - b.priority),
      total_minutes: phases[k].reduce((s, n) => s + n.minutes, 0),
    }));
  }

  /**
   * 导出 JSON 快照 (方便后续恢复)
   */
  exportSnapshot() {
    return {
      nodes: this.nodes,
      history: this.history,
      progress: this.getProgress(),
      exportedAt: new Date().toISOString(),
    };
  }

  /**
   * 导入快照恢复
   */
  importSnapshot(snap) {
    this.nodes = snap.nodes;
    this.history = snap.history || [];
    return true;
  }
}

// ============================================================
// 导出
// ============================================================

const TASK_SCHEDULER = new TaskNodeScheduler(TASK_NODES);

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    TASK_NODES,
    TaskNodeScheduler,
    TASK_SCHEDULER,
  };
} else {
  window.UniCoreTaskNodes = {
    TASK_NODES,
    TaskNodeScheduler,
    TASK_SCHEDULER,
  };
}

const totalNodes = Object.keys(TASK_NODES).length;
const totalMin = Object.values(TASK_NODES).reduce((s, n) => s + (n.estimated_minutes || 0), 0);
console.log(`[Task Nodes] 分布式切片任务节点加载完成:
  → 节点总数: ${totalNodes}
  → Phase 0 底层符号集: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P0')).length} 节点
  → Phase 1 集装箱体系: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P1')).length} 节点
  → Phase 2 纯符号核心: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P2')).length} 节点
  → Phase 3 CBC编码体系: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P3')).length} 节点
  → Phase 4 Foundation: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P4')).length} 节点
  → Phase 5 指令集&VM: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P5')).length} 节点
  → Phase 6 UI/3D层: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P6')).length} 节点
  → Phase 7 多语言/云/跨平台: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P7')).length} 节点
  → Phase 8 终极整合: ${Object.values(TASK_NODES).filter(n=>n.phase?.startsWith('P8')).length} 节点
  → 预估总工时: ${Math.round(totalMin/60*10)/10} 小时 (约 ${Math.round(totalMin/60/8*10)/10} 工作日)
`);
