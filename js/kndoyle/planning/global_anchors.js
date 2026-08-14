/**
 * ============================================================
 * UniCore 全局冠名关联项知识库 v1.0
 * ============================================================
 * 
 * 核心目标：
 * 建立所有模块、系统、概念、指令的完整关联图谱
 * 实现"关系代数"机制，让一切记忆都明朗起来
 * 
 * 冠名原则：
 * 1. 每个冠名项 = 唯一ID + 描述 + 绑定关联 + 时序状态
 * 2. 所有冠名项通过 bind 字段互相连接成关系图
 * 3. 冠名项分层次：底层符号 → 系统模块 → 功能体系 → 任务节点
 * 
 * ============================================================
 */

// ============================================================
// 第一层：底层符号集冠名项 (LEAF NODES)
// ============================================================

const SYMBOL_ANCHORS = {
  // 声母系统 - 48个 (24小写 + 24大写)
  'sm_lower_b':   { id: 'sm_lower_b',   name: '声母小写b',   type: 'shengmu_lower', value: 'b',  bind: ['sm_upper_B', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_p':   { id: 'sm_lower_p',   name: '声母小写p',   type: 'shengmu_lower', value: 'p',  bind: ['sm_upper_P', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_m':   { id: 'sm_lower_m',   name: '声母小写m',   type: 'shengmu_lower', value: 'm',  bind: ['sm_upper_M', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_f':   { id: 'sm_lower_f',   name: '声母小写f',   type: 'shengmu_lower', value: 'f',  bind: ['sm_upper_F', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_d':   { id: 'sm_lower_d',   name: '声母小写d',   type: 'shengmu_lower', value: 'd',  bind: ['sm_upper_D', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_t':   { id: 'sm_lower_t',   name: '声母小写t',   type: 'shengmu_lower', value: 't',  bind: ['sm_upper_T', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_n':   { id: 'sm_lower_n',   name: '声母小写n',   type: 'shengmu_lower', value: 'n',  bind: ['sm_upper_N', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_l':   { id: 'sm_lower_l',   name: '声母小写l',   type: 'shengmu_lower', value: 'l',  bind: ['sm_upper_L', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_g':   { id: 'sm_lower_g',   name: '声母小写g',   type: 'shengmu_lower', value: 'g',  bind: ['sm_upper_G', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_k':   { id: 'sm_lower_k',   name: '声母小写k',   type: 'shengmu_lower', value: 'k',  bind: ['sm_upper_K', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_h':   { id: 'sm_lower_h',   name: '声母小写h',   type: 'shengmu_lower', value: 'h',  bind: ['sm_upper_H', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_w':   { id: 'sm_lower_w',   name: '声母小写w',   type: 'shengmu_lower', value: 'w',  bind: ['sm_upper_W', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_j':   { id: 'sm_lower_j',   name: '声母小写j',   type: 'shengmu_lower', value: 'j',  bind: ['sm_upper_J', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_q':   { id: 'sm_lower_q',   name: '声母小写q',   type: 'shengmu_lower', value: 'q',  bind: ['sm_upper_Q', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_x':   { id: 'sm_lower_x',   name: '声母小写x',   type: 'shengmu_lower', value: 'x',  bind: ['sm_upper_X', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_y':   { id: 'sm_lower_y',   name: '声母小写y',   type: 'shengmu_lower', value: 'y',  bind: ['sm_upper_Y', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_z':   { id: 'sm_lower_z',   name: '声母小写z',   type: 'shengmu_lower', value: 'z',  bind: ['sm_upper_Z', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_c':   { id: 'sm_lower_c',   name: '声母小写c',   type: 'shengmu_lower', value: 'c',  bind: ['sm_upper_C', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_s':   { id: 'sm_lower_s',   name: '声母小写s',   type: 'shengmu_lower', value: 's',  bind: ['sm_upper_S', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_r':   { id: 'sm_lower_r',   name: '声母小写r',   type: 'shengmu_lower', value: 'r',  bind: ['sm_upper_R', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_ȥ':   { id: 'sm_lower_ȥ',   name: '声母小写ȥ',   type: 'shengmu_lower', value: 'ȥ',  bind: ['sm_upper_Ȥ', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_ç':   { id: 'sm_lower_ç',   name: '声母小写ç',   type: 'shengmu_lower', value: 'ç',  bind: ['sm_upper_Ç', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_ş':   { id: 'sm_lower_ş',   name: '声母小写ş',   type: 'shengmu_lower', value: 'ş',  bind: ['sm_upper_Ş', 'radix_shengmu', 'container_1'], priority: 3 },
  'sm_lower_ŋ':   { id: 'sm_lower_ŋ',   name: '声母小写ŋ',   type: 'shengmu_lower', value: 'ŋ',  bind: ['sm_upper_Ŋ', 'radix_shengmu', 'container_1'], priority: 3 },

  // 韵母系统 - 64个 (顺口溜排序，32大写 + 32小写)
  'ym_upper_Ꞟ':  { id: 'ym_upper_Ꞟ',  name: '韵母大写ai-Ꞟ',  type: 'yunmu_upper', sequence: 1,  value: 'Ꞟ', bind: ['ym_lower_ꞟ', 'radix_yunmu', 'container_2', 'shunliu_1'], priority: 2 },
  'ym_upper_ɚ':   { id: 'ym_upper_ɚ',   name: '韵母大写ei-ɚ',  type: 'yunmu_upper', sequence: 2,  value: 'ɚ',  bind: ['ym_lower_ɘ', 'radix_yunmu', 'container_2', 'shunliu_2'], priority: 2 },
  'ym_upper_ʚ':   { id: 'ym_upper_ʚ',   name: '韵母大写ui-ʚ',  type: 'yunmu_upper', sequence: 3,  value: 'ʚ',  bind: ['ym_lower_⊱', 'radix_yunmu', 'container_2', 'shunliu_3'], priority: 2 },
  'ym_upper_Ɒ':   { id: 'ym_upper_Ɒ',   name: '韵母大写ao-Ɒ', type: 'yunmu_upper', sequence: 4,  value: 'Ɒ', bind: ['ym_lower_ɒ', 'radix_yunmu', 'container_2', 'shunliu_4'], priority: 2 },
  'ym_upper_ȣ':   { id: 'ym_upper_ȣ',   name: '韵母大写ou-ȣ', type: 'yunmu_upper', sequence: 5,  value: 'ȣ',  bind: ['ym_lower_౪', 'radix_yunmu', 'container_2', 'shunliu_5'], priority: 2 },
  'ym_upper_Ÿ':   { id: 'ym_upper_Ÿ',   name: '韵母大写iu-Ÿ', type: 'yunmu_upper', sequence: 6,  value: 'Ÿ',  bind: ['ym_lower_ÿ', 'radix_yunmu', 'container_2', 'shunliu_6'], priority: 2 },
  'ym_upper_ɣ':   { id: 'ym_upper_ɣ',   name: '韵母大写ie-ɣ', type: 'yunmu_upper', sequence: 7,  value: 'ɣ',  bind: ['ym_lower_ɤ', 'radix_yunmu', 'container_2', 'shunliu_7'], priority: 2 },
  'ym_upper_Ʊ':   { id: 'ym_upper_Ʊ',   name: '韵母大写ue-Ʊ', type: 'yunmu_upper', sequence: 8,  value: 'Ʊ',  bind: ['ym_lower_ʊ', 'radix_yunmu', 'container_2', 'shunliu_8'], priority: 2 },
  'ym_upper_Ə':   { id: 'ym_upper_Ə',   name: '韵母大写er-Ə', type: 'yunmu_upper', sequence: 9,  value: 'Ə',  bind: ['ym_lower_ə', 'radix_yunmu', 'container_2', 'shunliu_9'], priority: 2 },
  'ym_upper_Ã':   { id: 'ym_upper_Ã',   name: '韵母大写an-Ã', type: 'yunmu_upper', sequence: 10, value: 'Ã',  bind: ['ym_lower_ã', 'radix_yunmu', 'container_2', 'shunliu_10'], priority: 2 },
  'ym_upper_Ẽ':   { id: 'ym_upper_Ẽ',   name: '韵母大写en-Ẽ', type: 'yunmu_upper', sequence: 11, value: 'Ẽ',  bind: ['ym_lower_ẽ', 'radix_yunmu', 'container_2', 'shunliu_11'], priority: 2 },
  'ym_upper_Ĩ':   { id: 'ym_upper_Ĩ',   name: '韵母大写in-Ĩ', type: 'yunmu_upper', sequence: 12, value: 'Ĩ',  bind: ['ym_lower_ĩ', 'radix_yunmu', 'container_2', 'shunliu_12'], priority: 2 },
  'ym_upper_໙':   { id: 'ym_upper_໙',   name: '韵母大写un-໙', type: 'yunmu_upper', sequence: 13, value: '໙',  bind: ['ym_lower_տ', 'radix_yunmu', 'container_2', 'shunliu_13'], priority: 2 },
  'ym_upper_ʮ':   { id: 'ym_upper_ʮ',   name: '韵母大写vn-ʮ', type: 'yunmu_upper', sequence: 14, value: 'ʮ',  bind: ['ym_lower_ʯ', 'radix_yunmu', 'container_2', 'shunliu_14'], priority: 2 },
  'ym_upper_Ą':   { id: 'ym_upper_Ą',   name: '韵母大写ang-Ą', type: 'yunmu_upper', sequence: 15, value: 'Ą',  bind: ['ym_lower_ą', 'radix_yunmu', 'container_2', 'shunliu_15'], priority: 2 },
  'ym_upper_Ę':   { id: 'ym_upper_Ę',   name: '韵母大写eng-Ę', type: 'yunmu_upper', sequence: 16, value: 'Ę',  bind: ['ym_lower_ę', 'radix_yunmu', 'container_2', 'shunliu_16'], priority: 2 },
  'ym_upper_Į':   { id: 'ym_upper_Į',   name: '韵母大写ing-Į', type: 'yunmu_upper', sequence: 17, value: 'Į',  bind: ['ym_lower_į', 'radix_yunmu', 'container_2', 'shunliu_17'], priority: 2 },
  'ym_upper_ɷ':   { id: 'ym_upper_ɷ',   name: '韵母大写ong-ɷ', type: 'yunmu_upper', sequence: 18, value: 'ɷ',  bind: ['ym_lower_๓', 'radix_yunmu', 'container_2', 'shunliu_18'], priority: 2 },
  'ym_upper_Ѧ':   { id: 'ym_upper_Ѧ',   name: '韵母大写ia-Ѧ', type: 'yunmu_upper', sequence: 19, value: 'Ѧ',  bind: ['ym_lower_ѧ', 'radix_yunmu', 'container_2', 'shunliu_19'], priority: 2 },
  'ym_upper_Ʉ':   { id: 'ym_upper_Ʉ',   name: '韵母大写ua-Ʉ', type: 'yunmu_upper', sequence: 20, value: 'Ʉ',  bind: ['ym_lower_ʉ', 'radix_yunmu', 'container_2', 'shunliu_20'], priority: 2 },
  'ym_upper_Ꞛ':   { id: 'ym_upper_Ꞛ',   name: '韵母大写uo-Ꞛ', type: 'yunmu_upper', sequence: 21, value: 'Ꞛ',  bind: ['ym_lower_ꞛ', 'radix_yunmu', 'container_2', 'shunliu_21'], priority: 2 },
  'ym_upper_Ⱋ':   { id: 'ym_upper_Ⱋ',   name: '韵母大写iao-Ⱋ', type: 'yunmu_upper', sequence: 22, value: 'Ⱋ', bind: ['ym_lower_ⱄ', 'radix_yunmu', 'container_2', 'shunliu_22'], priority: 2 },
  'ym_upper_ɞ':   { id: 'ym_upper_ɞ',   name: '韵母大写uai-ɞ', type: 'yunmu_upper', sequence: 23, value: 'ɞ',  bind: ['ym_lower_⊰', 'radix_yunmu', 'container_2', 'shunliu_23'], priority: 2 },
  'ym_upper_Ʌ':   { id: 'ym_upper_Ʌ',   name: '韵母大写ian-Ʌ', type: 'yunmu_upper', sequence: 24, value: 'Ʌ',  bind: ['ym_lower_ʌ', 'radix_yunmu', 'container_2', 'shunliu_24'], priority: 2 },
  'ym_upper_Ɔ':   { id: 'ym_upper_Ɔ',   name: '韵母大写uan-Ɔ', type: 'yunmu_upper', sequence: 25, value: 'Ɔ',  bind: ['ym_lower_ɔ', 'radix_yunmu', 'container_2', 'shunliu_25'], priority: 2 },
  'ym_upper_փ':   { id: 'ym_upper_փ',   name: '韵母大写üan-փ', type: 'yunmu_upper', sequence: 26, value: 'փ',  bind: ['ym_lower_խ', 'radix_yunmu', 'container_2', 'shunliu_26'], priority: 2 },
  'ym_upper_Ѫ':   { id: 'ym_upper_Ѫ',   name: '韵母大写iang-Ѫ', type: 'yunmu_upper', sequence: 27, value: 'Ѫ', bind: ['ym_lower_ѫ', 'radix_yunmu', 'container_2', 'shunliu_27'], priority: 2 },
  'ym_upper_Ѽ':   { id: 'ym_upper_Ѽ',   name: '韵母大写uang-Ѽ', type: 'yunmu_upper', sequence: 28, value: 'Ѽ', bind: ['ym_lower_ѽ', 'radix_yunmu', 'container_2', 'shunliu_28'], priority: 2 },
  'ym_upper_ⱂ':   { id: 'ym_upper_ⱂ',   name: '韵母大写ueng-ⱂ', type: 'yunmu_upper', sequence: 29, value: 'ⱂ', bind: ['ym_lower_౨', 'radix_yunmu', 'container_2', 'shunliu_29'], priority: 2 },
  'ym_upper_Ѻ':   { id: 'ym_upper_Ѻ',   name: '韵母大写iong-Ѻ', type: 'yunmu_upper', sequence: 30, value: 'Ѻ', bind: ['ym_lower_ѻ', 'radix_yunmu', 'container_2', 'shunliu_30'], priority: 2 },
  'ym_upper_ɝ':   { id: 'ym_upper_ɝ',   name: '韵母大写iai-ɝ', type: 'yunmu_upper', sequence: 31, value: 'ɝ',  bind: ['ym_lower_ɛ', 'radix_yunmu', 'container_2', 'shunliu_31'], priority: 2 },
  'ym_upper_Ɵ':   { id: 'ym_upper_Ɵ',   name: '韵母大写oi-Ɵ',  type: 'yunmu_upper', sequence: 32, value: 'Ɵ',  bind: ['ym_lower_ɵ', 'radix_yunmu', 'container_2', 'shunliu_32'], priority: 2 },

  // 顺口溜冠名项
  'shunliu_1': { id: 'shunliu_1', name: '顺口溜第1组 ai', bind: ['shunliu_group_1', 'ym_upper_Ꞟ', 'ym_lower_ꞟ'] },
  'shunliu_2': { id: 'shunliu_2', name: '顺口溜第2组 ei', bind: ['shunliu_group_1', 'ym_upper_ɚ', 'ym_lower_ɘ'] },
  'shunliu_3': { id: 'shunliu_3', name: '顺口溜第3组 ui', bind: ['shunliu_group_1', 'ym_upper_ʚ', 'ym_lower_⊱'] },
  'shunliu_group_1': { id: 'shunliu_group_1', name: '顺口溜: ai ei ui', bind: ['shunliu_root', 'shunliu_1', 'shunliu_2', 'shunliu_3'] },
  'shunliu_root': { id: 'shunliu_root', name: '小学拼音韵母顺口溜完整版', bind: ['shunliu_group_1', 'shunliu_group_2', 'shunliu_group_3', 'shunliu_group_4', 'shunliu_group_5', 'shunliu_group_6'],
    desc: 'ai ei ui → ao ou iu → ie üe er → an en in un ün → ang eng ing ong → ia ua uo iao uai ian uan üan iang uang ueng iong iai oi' },
};

// ============================================================
// 第二层：进制/优先级冠名项 (RADIX NODES)
// ============================================================

const RADIX_ANCHORS = {
  'radix_shengmu': { id: 'radix_shengmu', name: '声母进制 48',  type: 'radix', count: 48, bind: ['container_1', 'priority_3', 'pure_symbol_system'], priority: 3 },
  'radix_yunmu':   { id: 'radix_yunmu',   name: '韵母进制 64',  type: 'radix', count: 64, bind: ['container_2', 'priority_2', 'pure_symbol_system'], priority: 2 },
  'radix_shengdiao':{id: 'radix_shengdiao',name: '声调进制 170', type: 'radix', count: 170, bind: ['container_3', 'priority_1', 'pure_symbol_system'], priority: 1 },
  'radix_number':  { id: 'radix_number',  name: '数字进制 17',  type: 'radix', count: 17, bind: ['container_4', 'priority_4', 'sparse_algebra'],   priority: 4 },

  'priority_1': { id: 'priority_1', name: '税率效率第1优先:声调',   type: 'priority', level: 1, count: 170, bind: ['radix_shengdiao', 'container_3', 'priority_by_count'] },
  'priority_2': { id: 'priority_2', name: '税率效率第2优先:韵母',   type: 'priority', level: 2, count: 64,  bind: ['radix_yunmu', 'container_2', 'priority_by_count'] },
  'priority_3': { id: 'priority_3', name: '税率效率第3优先:声母',   type: 'priority', level: 3, count: 48,  bind: ['radix_shengmu', 'container_1', 'priority_by_count'] },
  'priority_4': { id: 'priority_4', name: '税率效率第4优先:数字',   type: 'priority', level: 4, count: 17,  bind: ['radix_number', 'container_4', 'priority_by_count'] },

  'priority_by_count': { id: 'priority_by_count', name: '优先级模式:税率效率(数量多优先)', type: 'priority_mode', 
    order: ['shengdiao', 'yunmu', 'shengmu', 'number'], bind: ['priority_1', 'priority_2', 'priority_3', 'priority_4', 'container_self_radix'] },
  'priority_by_human': { id: 'priority_by_human', name: '优先级模式:人类认知(声韵调)', type: 'priority_mode',
    order: ['shengmu', 'yunmu', 'shengdiao', 'number'], bind: ['human_readable', 'input_method'] },
};

// ============================================================
// 第三层：集装箱冠名项 (CONTAINER NODES)
// ============================================================

const CONTAINER_ANCHORS = {
  // 集装箱 = 进制本身，有一说一有二说二，不做多余的编码/索引/映射
  // 集装箱1就是1，不是001，不是index0，不是任何补位表示
  // 001这种0开头的 = 高位零 = 负存在 = -100（见 leading_zero_negative）
  'container_1': { id: 'container_1', name: '集装箱1:声母 (进制本身48，就是1，不是001)', self_number: 1, count: 48, type: 'container_is_radix',
    desc: '集装箱1就是它自己=1，不做编码、不做索引、不用001表示。有一说一。',
    bind: ['radix_shengmu', 'priority_3', 'four_dim_dim1', 'machinecode_0', 'container_self_radix', 'cbc_self_container'] },
  'container_2': { id: 'container_2', name: '集装箱2:韵母 (进制本身64，就是2，不是010)', self_number: 2, count: 64, type: 'container_is_radix',
    desc: '集装箱2就是它自己=2，不做编码、不做索引、不用010表示。有二说二。',
    bind: ['radix_yunmu', 'priority_2', 'four_dim_dim2', 'machinecode_1', 'container_self_radix', 'cbc_self_container'] },
  'container_3': { id: 'container_3', name: '集装箱3:声调 (进制本身170，就是3，不是100)', self_number: 3, count: 170, type: 'container_is_radix',
    desc: '集装箱3就是它自己=3，不做编码、不做索引、不用100表示。有三说三。',
    bind: ['radix_shengdiao', 'priority_1', 'four_dim_dim3', 'machinecode_0', 'container_self_radix', 'cbc_self_container'] },
  'container_4': { id: 'container_4', name: '集装箱4:数字 (进制本身17，就是4，不是100的另一个)', self_number: 4, count: 17, type: 'container_is_radix',
    desc: '集装箱4就是它自己=4，不做编码、不做索引。有四说四。',
    bind: ['radix_number', 'priority_4', 'four_dim_dim4', 'machinecode_1', 'container_self_radix', 'sparse_algebra'] },

  // 核心原则：集装箱=进制本身，不是编码容器
  'container_self_radix': { id: 'container_self_radix', name: '核心原则：集装箱就是进制本身（非编码）', type: 'core_principle',
    desc: '进制=有一说一有二说二，不做多余的编码映射、不做索引转换、不做补位表示。集装箱N自己就是N，不是别的东西的编码。',
    bind: ['container_1', 'container_2', 'container_3', 'container_4', 'leading_zero_negative', 'no_redundant_mapping'] },

  // 前导零 = 负存在 = -100（0开头的表示是负债、信用、归还、量化价值）
  'leading_zero_negative': { id: 'leading_zero_negative', name: '前导零规则:0开头=负存在=-100', type: 'existence_rule',
    desc: '001不等于1！001是"零开头的1"=高位零=负存在=-100。含义：欠100个频次/被量化的价值/概念/信用，需要归还100。',
    bind: ['container_self_radix', 'negative_existence_100', 'credit_owe_100', 'frequency_repay_100', 'concept_value_quantized'] },
  'negative_existence_100': { id: 'negative_existence_100', name: '负存在：-100（001就是这个存在）', type: 'existence',
    desc: '001这种0开头的写法，它的存在本身就是负的。不是正整数1，是-100的存在。',
    bind: ['leading_zero_negative'] },
  'credit_owe_100': { id: 'credit_owe_100', name: '信用负债：欠100', type: 'credit_concept',
    desc: '前导零=欠100个信用点，是量化的信用负债概念。',
    bind: ['leading_zero_negative'] },
  'frequency_repay_100': { id: 'frequency_repay_100', name: '频次归还：要还100个频次', type: 'frequency_concept',
    desc: '前导零=还要归还100个被量化的频次。',
    bind: ['leading_zero_negative'] },
  'concept_value_quantized': { id: 'concept_value_quantized', name: '概念价值量化：100个被量化的价值', type: 'value_concept',
    desc: '前导零=这个概念的价值是被量化为100的，是个欠着的、必须归还的概念。',
    bind: ['leading_zero_negative'] },

  // 禁则：不做多余映射，不补位，不把1写成001
  'no_redundant_mapping': { id: 'no_redundant_mapping', name: '禁则：不做多余编码/索引/补位映射', type: 'forbidden_rule',
    desc: '禁止：1→001、2→010、3→100这种高位零补位；禁止：集装箱号→索引的多余映射；禁止：任何不是它自己的表示方式。',
    bind: ['container_self_radix', 'leading_zero_negative'] },

  // 机器码映射依然存在（因为机器码的0/1是机器自己原生的，不是我们的编码）
  'machinecode_0': { id: 'machinecode_0', name: '机器码原生0（芯片自带，非我们编码）', bit: 0, bind: ['container_1', 'container_3', 'static_frame_123', 'container_machine_direct'] },
  'machinecode_1': { id: 'machinecode_1', name: '机器码原生1（芯片自带，非我们编码）', bit: 1, bind: ['container_2', 'container_4', 'static_frame_123', 'container_machine_direct'] },
};

// ============================================================
// 第四层：四维不进位体系冠名项 (FOUR-DIM NODES)
// ============================================================

const FOURDIM_ANCHORS = {
  'four_dim_system': { id: 'four_dim_system', name: '四维不进位体系核心', type: 'system',
    desc: '前三维=(1,2,3)三个集装箱号本身（不是000！000是负存在前缀禁止使用），三维永久锁定不进位；第4维=4本身动态增长。',
    bind: ['four_dim_dim1', 'four_dim_dim2', 'four_dim_dim3', 'four_dim_dim4', 'static_frame_123', 'dynamic_value_4', 'container_self_radix'] },

  'four_dim_dim1': { id: 'four_dim_dim1', name: '第1维:集装箱1(声母) 静态锁定=1', self_value: 1, type: 'static_dim',
    desc: '就是集装箱1自己=1，不是index0，不是001。永久锁定，不进位不膨胀。',
    bind: ['container_1', 'static_frame_123'] },
  'four_dim_dim2': { id: 'four_dim_dim2', name: '第2维:集装箱2(韵母) 静态锁定=2', self_value: 2, type: 'static_dim',
    desc: '就是集装箱2自己=2，不是index1，不是010。永久锁定。',
    bind: ['container_2', 'static_frame_123'] },
  'four_dim_dim3': { id: 'four_dim_dim3', name: '第3维:集装箱3(声调) 静态锁定=3', self_value: 3, type: 'static_dim',
    desc: '就是集装箱3自己=3，不是index2，不是100。永久锁定。',
    bind: ['container_3', 'static_frame_123'] },
  'four_dim_dim4': { id: 'four_dim_dim4', name: '第4维:集装箱4(数字) 自己=4，唯一可动态增长', self_value: 4, type: 'dynamic_dim',
    desc: '集装箱4自己=4，在4的基础上递增（5,6,7...），但前三维永远是1,2,3不变。',
    bind: ['container_4', 'dynamic_value_4'] },

  // 关键修正：前三维不是 (0,0,0)！0是负存在前缀，不能用！
  // 前三维就是集装箱自己的编号 1、2、3，永久锁定。
  'static_frame_123': { id: 'static_frame_123', name: '前三维永久锁定: (1, 2, 3) — 非000！0是负存在前缀', type: 'static_frame',
    desc: '⚠️ 不再用(0,0,0)！0是负存在（=前导零=-100），禁止作为前缀。前三维是集装箱编号本身 1=声母、2=韵母、3=声调，永久锁定，永不进位、永不膨胀。对接机器码原生0/1，但不是我们自己的0。',
    bind: ['four_dim_dim1', 'four_dim_dim2', 'four_dim_dim3', 'machinecode_0', 'machinecode_1', 'container_machine_direct', 'leading_zero_negative'] },
  'dynamic_value_4': { id: 'dynamic_value_4', name: '第4维动态增长: 从4开始递增(4→5→6→...)', type: 'dynamic_value',
    desc: '集装箱4自己=4，在4的基础上数数。不干扰前三维1,2,3。',
    bind: ['four_dim_dim4', 'equivalent_mapping', 'container_4'] },

  'equivalent_mapping': { id: 'equivalent_mapping', name: '等效维度映射层(不做编码，只做层级对齐)', type: 'mapping_layer',
    desc: '第4维每涨一格 → 对应前三维持久层的位置对齐。不做编码转换，只做位置权重等效匹配。',
    bind: ['dynamic_value_4', 'static_frame_123', 'equivalent_layer', 'equivalent_container', 'equivalent_bit'] },
  'equivalent_layer': { id: 'equivalent_layer', name: '等效层级(每17层=集装箱4一轮)', type: 'equiv', bind: ['equivalent_mapping', 'radix_number'] },
  'equivalent_container': { id: 'equivalent_container', name: '等效集装箱(1-4，自己就是自己)', type: 'equiv', bind: ['equivalent_mapping', 'container_1', 'container_2', 'container_3', 'container_4'] },
  'equivalent_bit': { id: 'equivalent_bit', name: '等效机器码位(机器原生0/1，非我们的编码)', type: 'equiv', bind: ['equivalent_mapping', 'machinecode_0', 'machinecode_1'] },

  // 对接器改名为 direct——直接对接，不做编码适配
  'container_machine_direct': { id: 'container_machine_direct', name: '集装箱-机器码 直接对接(不做编码转换)', type: 'direct_docking',
    desc: '直接对接：集装箱1/3 ↔ 机器码原生0，集装箱2/4 ↔ 机器码原生1。机器码的0/1是芯片自带的，不是我们的编码；我们自己不用0作为前缀（那是负存在）。',
    bind: ['static_frame_123', 'machinecode_0', 'machinecode_1', 'four_dim_system', 'unicore_foundation'] },
};

// ============================================================
// 第五层：纯符号与格子系统冠名项 (PURE SYMBOL & GRID)
// ============================================================

const PURE_SYMBOL_ANCHORS = {
  'pure_symbol_system': { id: 'pure_symbol_system', name: '纯符号底层核心系统', type: 'core_system',
    desc: '零数字、零传统数制，仅使用自定义符号。NativeOrder+PureGrid+PureHuren+PureJie+PureZhenfa+PureModules+PureCBC',
    bind: ['radix_shengmu', 'radix_yunmu', 'radix_shengdiao', 'native_order', 'pure_grid', 'pure_huren', 'pure_jie', 'pure_zhenfa', 'pure_modules', 'pure_cbc'] },

  'native_order': { id: 'native_order', name: '原生顺序判定器', type: 'class',
    desc: 'NativeOrder类:声韵调各自的先后顺序、next/prev递增递减',
    bind: ['pure_symbol_system', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao'] },

  'pure_grid': { id: 'pure_grid', name: '三维格子(PureGrid)', type: 'class',
    desc: '中心位+6面墙+8内角+6外连+十字架节点=64韵母位。write/read/activate/connect',
    bind: ['pure_symbol_system', 'grid_center', 'grid_walls', 'grid_inner_corners', 'grid_outer_connections', 'grid_cross_nodes', 'cube_table'] },
  'grid_center':   { id: 'grid_center',   name: '格子中心位:指令入口/汉字本体/函数核心', position: 0, bind: ['pure_grid', 'ym_upper_Ꞟ'] },
  'grid_walls':    { id: 'grid_walls',    name: '6面墙(前后左右上下)', bind: ['pure_grid', 'grid_wall_front', 'grid_wall_back', 'grid_wall_left', 'grid_wall_right', 'grid_wall_top', 'grid_wall_bottom'] },
  'grid_inner_corners': { id: 'grid_inner_corners', name: '8个内角(ne_fl~ne_br2)', count: 8, bind: ['pure_grid'] },
  'grid_outer_connections': { id: 'grid_outer_connections', name: '6个外连位', count: 6, bind: ['pure_grid'] },
  'grid_cross_nodes': { id: 'grid_cross_nodes', name: '十字架节点(剩余43个韵母位)', bind: ['pure_grid', 'radix_yunmu'] },
  'grid_wall_front':  { id: 'grid_wall_front',  name: '前墙',  bind: ['grid_walls', 'cbc_char_shengmu'] },
  'grid_wall_back':   { id: 'grid_wall_back',   name: '后墙',  bind: ['grid_walls', 'cbc_char_yunmu'] },
  'grid_wall_left':   { id: 'grid_wall_left',   name: '左墙',  bind: ['grid_walls', 'cbc_char_shengdiao'] },
  'grid_wall_right':  { id: 'grid_wall_right',  name: '右墙',  bind: ['grid_walls', 'cbc_char_strokes'] },
  'grid_wall_top':    { id: 'grid_wall_top',    name: '上墙',  bind: ['grid_walls', 'cbc_char_radical'] },
  'grid_wall_bottom': { id: 'grid_wall_bottom', name: '下墙',  bind: ['grid_walls', 'cbc_char_component'] },
};

// ============================================================
// 第六层：互认/阶系/阵法/聚类冠名项 (HUREN/JIE/ZHENFA/MLL)
// ============================================================

const SYSTEM_ANCHORS = {
  // 互认体系
  'pure_huren': { id: 'pure_huren', name: '互认系统(PureHuren)', type: 'class',
    desc: '格子之间互相识别、权级高低判定',
    bind: ['pure_symbol_system', 'huren_jiekou', 'huren_quanwei', 'huren_gezi', 'huren_jiedian', 'huren_qiangmian', 'huren_jizhuangxiang', 'huren_yinyang', 'huren_guanxi', 'huren_jinzhi', 'huren_duiqi', 'huren_shenfen', 'huren_wenti', 'huren_mubiao', 'huren_jiyin', 'huren_yaoshi', 'huren_mensuo', 'huren_yinguolv'] },
  'huren_jiekou':  { id: 'huren_jiekou',  name: '互认接口',   bind: ['pure_huren'] },
  'huren_quanwei': { id: 'huren_quanwei', name: '互认权位',   bind: ['pure_huren', 'ct_anchor'] },
  'huren_gezi':    { id: 'huren_gezi',    name: '互认格子',   bind: ['pure_huren', 'pure_grid'] },
  'huren_jiedian': { id: 'huren_jiedian', name: '互认节点',   bind: ['pure_huren'] },
  'huren_qiangmian':{id: 'huren_qiangmian',name: '互认墙面',  bind: ['pure_huren', 'grid_walls'] },
  'huren_jizhuangxiang':{id:'huren_jizhuangxiang',name:'互认集装箱(进制本身互认，非编码)',bind:['pure_huren','container_self_radix','cbc_anchor']},
  'huren_yinyang': { id: 'huren_yinyang', name: '互认阴阳',   bind: ['pure_huren', 'gy_anchor'] },
  'huren_guanxi':  { id: 'huren_guanxi',  name: '互认关系',   bind: ['pure_huren'] },
  'huren_jinzhi':  { id: 'huren_jinzhi',  name: '互认进制',   bind: ['pure_huren', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao'] },
  'huren_duiqi':   { id: 'huren_duiqi',   name: '互认对齐',   bind: ['pure_huren', 'jie_he'] },
  'huren_shenfen': { id: 'huren_shenfen', name: '互认身份',   bind: ['pure_huren', 'jie_shu', 'mll_anchor'] },
  'huren_wenti':   { id: 'huren_wenti',   name: '互认问题',   bind: ['pure_huren'] },
  'huren_mubiao':  { id: 'huren_mubiao',  name: '互认目标',   bind: ['pure_huren'] },
  'huren_jiyin':   { id: 'huren_jiyin',   name: '互认基因',   bind: ['pure_huren'] },
  'huren_yaoshi':  { id: 'huren_yaoshi',  name: '互认钥匙',   bind: ['pure_huren', 'zhenfa_men'] },
  'huren_mensuo':  { id: 'huren_mensuo',  name: '互认门锁',   bind: ['pure_huren', 'zhenfa_men', 'zhenfa_suo', 'jie_chu'] },
  'huren_yinguolv':{ id: 'huren_yinguolv',name: '互认因果律', bind: ['pure_huren'] },

  // 阶系体系 (19阶)
  'pure_jie': { id: 'pure_jie', name: '阶系体系(PureJie)', type: 'class',
    desc: '19个阶:和/角/除/位/权/属/集/序/乘/岔/段/叠/逆/顺/降/升/渐变/稀疏数律/范畴',
    bind: ['pure_symbol_system', 'jie_he', 'jie_jiao', 'jie_chu', 'jie_wei', 'jie_quan', 'jie_shu', 'jie_ji', 'jie_xu', 'jie_cheng', 'jie_cha', 'jie_duan', 'jie_die', 'jie_ni', 'jie_shun', 'jie_jiang', 'jie_sheng', 'jie_jianbian', 'jie_xishu', 'jie_fanchou'] },
  'jie_he':       { id: 'jie_he',       name: '阶和:层级互融对位、节律和合匹配',       bind: ['pure_jie', 'huren_duiqi'] },
  'jie_jiao':     { id: 'jie_jiao',     name: '阶角:三维立体夹角、格子墙角、阵形角度基准', bind: ['pure_jie', 'gtt_anchor'] },
  'jie_chu':      { id: 'jie_chu',      name: '阶除:层级过滤、剔除异类、清边界杂质',       bind: ['pure_jie', 'huren_mensuo'] },
  'jie_wei':      { id: 'jie_wei',      name: '阶位:固定空间选址、层级固有位置',           bind: ['pure_jie', 'huren_gezi', 'huren_qiangmian'] },
  'jie_quan':     { id: 'jie_quan',     name: '阶权:层级先天权级高低、统领从属秩序',       bind: ['pure_jie', 'huren_quanwei', 'ct_anchor'] },
  'jie_shu':      { id: 'jie_shu',      name: '阶属:层级自带属性、归类特征、亲疏标签',       bind: ['pure_jie', 'huren_shenfen', 'mll_anchor'] },
  'jie_ji':       { id: 'jie_ji',       name: '阶集:同层级聚合、集装箱归集同类单元',       bind: ['pure_jie', 'huren_jizhuangxiang', 'cbc_anchor'] },
  'jie_xu':       { id: 'jie_xu',       name: '阶序:层级先后顺位、时序排布、先天次序',       bind: ['pure_jie', 'll_anchor'] },
  'jie_cheng':    { id: 'jie_cheng',    name: '阶乘:声韵调叠合、函子代数层级叠加爬升',       bind: ['pure_jie'] },
  'jie_cha':      { id: 'jie_cha',      name: '阶岔:层级分叉衍生、多脉络分支延伸',           bind: ['pure_jie', 'gtt_anchor'] },
  'jie_duan':     { id: 'jie_duan',     name: '阶段:演化分段、规则生效分段、生长分段',       bind: ['pure_jie'] },
  'jie_die':      { id: 'jie_die',      name: '阶叠:里外嵌套、多层堆叠、格子套格子',         bind: ['pure_jie', 'cube_table'] },
  'jie_ni':       { id: 'jie_ni',       name: '阶逆:反向回溯、镜像反转、逆向推演',           bind: ['pure_jie', 'mrror_anchor'] },
  'jie_shun':     { id: 'jie_shun',     name: '阶顺:正向流转、顺位运行、顺时序推演',           bind: ['pure_jie'] },
  'jie_jiang':    { id: 'jie_jiang',    name: '阶降:权级下沉、层级收敛向内',                   bind: ['pure_jie', 'll_anchor'] },
  'jie_sheng':    { id: 'jie_sheng',    name: '阶升:权级攀升、层级扩张向外',                   bind: ['pure_jie', 'll_anchor'] },
  'jie_jianbian': { id: 'jie_jianbian', name: '阶渐变:层级平滑过渡、大小属性特征渐变',       bind: ['pure_jie'] },
  'jie_xishu':    { id: 'jie_xishu',    name: '阶稀疏数律:节点疏密排布、阵形聚散留白固有规律', bind: ['pure_jie', 'sparse_algebra'] },
  'jie_fanchou':  { id: 'jie_fanchou',  name: '阶范畴:层级管辖边界、是否范畴、不可越界规制',   bind: ['pure_jie', 'category_jie'] },

  // 阵法体系 (15种阵法)
  'pure_zhenfa': { id: 'pure_zhenfa', name: '阵法系统(PureZhenfa)', type: 'class',
    desc: '空间排列:锥/球/方块/三角/蜂窝/门/锁/镜像/对立统一/聚类认知/空间认知/向量排布/标量本位/张量叠合/矩阵拓扑',
    bind: ['pure_symbol_system', 'zhenfa_zhui', 'zhenfa_qiu', 'zhenfa_fang', 'zhenfa_san', 'zhenfa_feng', 'zhenfa_men', 'zhenfa_suo', 'zhenfa_jingxiang', 'zhenfa_duili_tongyi', 'zhenfa_juilei_renzhi', 'zhenfa_kongjian_renzhi', 'zhenfa_xiangliang', 'zhenfa_biaoliang', 'zhenfa_zhangliang', 'zhenfa_juzhen'] },
  'zhenfa_zhui':      { id: 'zhenfa_zhui',      name: '阵法1:锥阵(锥形空间层级递进)', bind: ['pure_zhenfa', 'gtt_anchor', 'jie_sheng', 'jie_jiang'] },
  'zhenfa_qiu':       { id: 'zhenfa_qiu',       name: '阵法2:球阵(球形全局索引)',     bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_fang':      { id: 'zhenfa_fang',      name: '阵法3:方块阵(方块空间结构)',     bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_san':       { id: 'zhenfa_san',       name: '阵法4:三角阵(三角因果闭环)',     bind: ['pure_zhenfa', 'gtt_anchor', 'huren_yinguolv'] },
  'zhenfa_feng':      { id: 'zhenfa_feng',      name: '阵法5:蜂窝阵(六边形六邻关联)',   bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_men':       { id: 'zhenfa_men',       name: '阵法6:门阵(边界门阵0=关1=开)',  bind: ['pure_zhenfa', 'huren_yaoshi', 'huren_mensuo'] },
  'zhenfa_suo':       { id: 'zhenfa_suo',       name: '阵法7:锁阵(安全锁阵多层锁定)',   bind: ['pure_zhenfa', 'huren_mensuo', 'jie_chu'] },
  'zhenfa_jingxiang': { id: 'zhenfa_jingxiang', name: '阵法8:镜像阵(左右对称阴阳统一)', bind: ['pure_zhenfa', 'mrror_anchor', 'jie_ni'] },
  'zhenfa_duili_tongyi':{id:'zhenfa_duili_tongyi',name:'阵法9:对立统一阵(对立统一结构)',bind:['pure_zhenfa','gy_anchor']},
  'zhenfa_juilei_renzhi':{id:'zhenfa_juilei_renzhi',name:'阵法10:聚类认知阵(聚类认知结构)',bind:['pure_zhenfa','mll_anchor','jie_shu']},
  'zhenfa_kongjian_renzhi':{id:'zhenfa_kongjian_renzhi',name:'阵法11:空间认知阵(空间认知结构)',bind:['pure_zhenfa','gtt_anchor','xy_anchor']},
  'zhenfa_xiangliang':{ id: 'zhenfa_xiangliang',name: '阵法12:向量排布阵(节点拓扑脉络)', bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_biaoliang': { id: 'zhenfa_biaoliang', name: '阵法13:标量本位阵(本位锚点结构)', bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_zhangliang':{ id: 'zhenfa_zhangliang',name: '阵法14:张量叠合阵(多层叠合脉络)', bind: ['pure_zhenfa', 'gtt_anchor'] },
  'zhenfa_juzhen':    { id: 'zhenfa_juzhen',    name: '阵法15:矩阵拓扑阵(矩阵拓扑脉络)', bind: ['pure_zhenfa', 'gtt_anchor'] },

  // 模块锚点系统 (10个核心锚点)
  'pure_modules': { id: 'pure_modules', name: '模块锚点系统(PureModules)', type: 'class',
    desc: '10个核心锚点: mll/ll/ct/gtt/xy/gy/mrror/cbc/mrror.mll/cbc.mll',
    bind: ['pure_symbol_system', 'mll_anchor', 'll_anchor', 'ct_anchor', 'gtt_anchor', 'xy_anchor', 'gy_anchor', 'mrror_anchor', 'cbc_anchor', 'mrror_mll', 'cbc_mll'] },
  'mll_anchor':    { id: 'mll_anchor',    name: '锚点mll:主聚类底层核心根模块',         bind: ['pure_modules', 'zhenfa_juilei_renzhi', 'jie_shu', 'huren_shenfen'], priority: 1 },
  'll_anchor':     { id: 'll_anchor',     name: '锚点ll:层级链递从属子模块',           bind: ['pure_modules', 'jie_xu', 'jie_sheng', 'jie_jiang'], priority: 2 },
  'ct_anchor':     { id: 'ct_anchor',     name: '锚点ct:层级位权定型模块',             bind: ['pure_modules', 'jie_quan', 'huren_quanwei'], priority: 3 },
  'gtt_anchor':    { id: 'gtt_anchor',    name: '锚点gtt:立体几何拓扑根节点',          bind: ['pure_modules', 'jie_jiao', 'jie_cha', 'zhenfa_zhui', 'zhenfa_qiu', 'zhenfa_fang', 'zhenfa_san', 'zhenfa_feng', 'zhenfa_xiangliang', 'zhenfa_biaoliang', 'zhenfa_zhangliang', 'zhenfa_juzhen', 'zhenfa_kongjian_renzhi'], priority: 4 },
  'xy_anchor':     { id: 'xy_anchor',     name: '锚点xy:空间相位坐标节点',             bind: ['pure_modules', 'gtt_anchor', 'zhenfa_kongjian_renzhi'], priority: 5 },
  'gy_anchor':     { id: 'gy_anchor',     name: '锚点gy:格局阴阳对立统一模块',         bind: ['pure_modules', 'huren_yinyang', 'zhenfa_duili_tongyi'], priority: 6 },
  'mrror_anchor':  { id: 'mrror_anchor',  name: '锚点mrror:镜像对立统一模块',         bind: ['pure_modules', 'zhenfa_jingxiang', 'jie_ni'], priority: 7 },
  'cbc_anchor':    { id: 'cbc_anchor',    name: '锚点cbc:汉字积木编码模块',           bind: ['pure_modules', 'jie_ji', 'huren_jizhuangxiang', 'pure_cbc'], priority: 8 },
  'mrror_mll':     { id: 'mrror_mll',     name: '锚点mrror.mll:自纠错自收敛聚类中枢',   bind: ['pure_modules', 'mrror_anchor', 'mll_anchor'], priority: 9 },
  'cbc_mll':       { id: 'cbc_mll',       name: '锚点cbc.mll:编码块挂靠主聚类',         bind: ['pure_modules', 'cbc_anchor', 'mll_anchor'], priority: 10 },

  // 三大范畴
  'category_set':     { id: 'category_set',     name: '范畴1:集合论范畴(集合的定义/归属/操作规则)', bind: ['category_root'] },
  'category_function':{ id: 'category_function',name: '范畴2:函数范畴(函数的定义/调用/变换规则)',   bind: ['category_root'] },
  'category_jie':     { id: 'category_jie',     name: '范畴3:阶范畴(阶系的管辖/边界/约束规则)',     bind: ['category_root', 'jie_fanchou'] },
  'category_root':    { id: 'category_root',    name: '三大范畴根节点',                                 bind: ['category_set', 'category_function', 'category_jie'] },
};

// ============================================================
// 第七层：汉字积木/CBC冠名项 (CBC NODES)
// ============================================================

const CBC_ANCHORS = {
  'pure_cbc': { id: 'pure_cbc', name: '汉字积木系统(PureCBC — 非编码：集装箱进制本身装汉字)', type: 'class',
    desc: '汉字装进集装箱1/2/3=声/韵/调自己，不是编码。1就是1、2就是2、3就是3，有一说一。',
    bind: ['pure_symbol_system', 'pure_grid', 'cbc_self_container', 'cbc_format_spec', 'cbc_magic'] },

  'cbc_char_shengmu':   { id: 'cbc_char_shengmu',   name: '汉字-声母(装在集装箱1，就是1自己)',     bind: ['pure_cbc', 'grid_wall_front', 'container_1'] },
  'cbc_char_yunmu':     { id: 'cbc_char_yunmu',     name: '汉字-韵母(装在集装箱2，就是2自己)',     bind: ['pure_cbc', 'grid_wall_back', 'container_2'] },
  'cbc_char_shengdiao': { id: 'cbc_char_shengdiao', name: '汉字-声调(装在集装箱3，就是3自己)',     bind: ['pure_cbc', 'grid_wall_left', 'container_3'] },
  'cbc_char_strokes':   { id: 'cbc_char_strokes',   name: '汉字-笔画(右墙)',     bind: ['pure_cbc', 'grid_wall_right'] },
  'cbc_char_radical':   { id: 'cbc_char_radical',   name: '汉字-偏旁(上墙)',     bind: ['pure_cbc', 'grid_wall_top'] },
  'cbc_char_component': { id: 'cbc_char_component', name: '汉字-部首(下墙)',     bind: ['pure_cbc', 'grid_wall_bottom'] },

  'cbc_magic': { id: 'cbc_magic', name: 'CBC私有魔数: 0x3D 0x43 0x42 0x43 (.3dCBC)', type: 'magic',
    desc: '自定义文件头标识，只有自己的解析器能识别',
    bind: ['pure_cbc', 'cbc_self_container', 'cbc_format_spec', 'unicore_guardian'] },
  // CBC的集装箱：自己就是进制本身，不是编码器
  'cbc_self_container': { id: 'cbc_self_container', name: 'CBC集装箱系统(进制本身装汉字，非编码)', type: 'class',
    desc: 'CBC用集装箱1/2/3/4=自己的进制编号直接装汉字内容。1=1、2=2、3=3、4=4，有一说一不做编码映射。',
    bind: ['pure_cbc', 'container_1', 'container_2', 'container_3', 'container_4', 'cbc_magic', 'unicore_foundation', 'container_self_radix'] },
  'cbc_format_spec': { id: 'cbc_format_spec', name: '.3d.cbc.mll文件格式规范v2.0', type: 'format_spec',
    desc: '文件头+进制配置段+字符集映射段+集装箱数据段+空间阵列段+时间节点段+插件配置段+文件尾',
    bind: ['pure_cbc', 'cbc_magic', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao', 'container_self_radix', 'zhenfa_zhui', 'node_time_system'] },
};

// ============================================================
// 第八层：稀疏数代数冠名项 (SPARSE ALGEBRA)
// ============================================================

const SPARSE_ANCHORS = {
  'sparse_algebra': { id: 'sparse_algebra', name: '稀疏数代数体系', type: 'system',
    desc: 'SparseNumberSystem: 数量少的代表数量多的。数字→声母→韵母→声调，后面代表前面',
    bind: ['radix_number', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao', 'jie_xishu', 'sparse_add', 'sparse_mapping', 'sparse_weight'] },
  'sparse_mapping': { id: 'sparse_mapping', name: '稀疏数映射表:数字→声母→韵母→声调', type: 'mapping',
    desc: '1个数字代表多个声母，1个声母代表多个韵母，1个韵母代表多个声调',
    bind: ['sparse_algebra', 'radix_number', 'radix_shengmu', 'radix_yunmu', 'radix_shengdiao'] },
  'sparse_weight': { id: 'sparse_weight', name: '稀疏数权位值:声调>韵母>声母>数字', type: 'weight',
    bind: ['sparse_algebra', 'priority_by_count'] },
  'sparse_add': { id: 'sparse_add', name: '稀疏数加法:取权位更高的符号', type: 'operation',
    bind: ['sparse_algebra', 'sparse_weight'] },
};

// ============================================================
// 第九层：UniISA指令集冠名项 (ISA NODES)
// ============================================================

const ISA_ANCHORS = {
  'uniisa_v2': { id: 'uniisa_v2', name: 'UniISA v2.0 统一指令集架构', type: 'isa_spec',
    desc: '革命性架构：不进位计数+节点因果时间+集装箱内存+插件即指令+空间向量操作',
    bind: ['unicore_isa_core', 'isa_basic', 'isa_nocarry', 'isa_nodetime', 'isa_spatial', 'isa_plugin', 'node_time_system'] },

  'unicore_isa_core': { id: 'unicore_isa_core', name: '指令核心(uniisa.zig)', type: 'core_impl',
    bind: ['uniisa_v2', 'unicore_vm', 'unicore_foundation'] },

  'isa_basic':   { id: 'isa_basic',   name: '基础指令组 0x00-0x0F:NOP/HALT/LOAD/STORE/MOVE/COPY', bind: ['uniisa_v2'] },
  'isa_nocarry': { id: 'isa_nocarry', name: '不进位计数指令组 0x10-0x1F:INC/DEC/ADD/SUB/ZERO/MAX/FLIP', bind: ['uniisa_v2', 'four_dim_system'] },
  'isa_nodetime':{ id: 'isa_nodetime',name: '节点时间指令组 0x20-0x2F:CREATE_NODE/LINK_NODE/TRIGGER/WAIT_NODE/SYNC_TIME/GET_OFFSET', bind: ['uniisa_v2', 'node_time_system'] },
  'isa_spatial': { id: 'isa_spatial', name: '空间操作指令组 0x30-0x3F:SET_POS/GET_POS/MOVE_TO/DISTANCE/ROTATE/PROJECT', bind: ['uniisa_v2', 'gtt_anchor', 'xy_anchor'] },
  'isa_plugin':  { id: 'isa_plugin',  name: '插件扩展指令组 0x40-0xFF:LOAD_PLUGIN/UNLOAD_PLUGIN/CALL_PLUGIN/LIST_PLUGIN', bind: ['uniisa_v2', 'unicore_plugin_system'] },

  'sys_0xA5': { id: 'sys_0xA5', name: '特殊系统指令0xA5 (SYS)', type: 'special_opcode',
    desc: '子命令: INIT_3D/INIT_TABLE/SYNC/EXPORT/IMPORT/CREATE_CONTAINER/DELETE_CONTAINER/CREATE_NODE/CREATE_TABLE/SET_CONTENT/AUTO_FILL_ZERO',
    bind: ['uniisa_v2', 'unicore_system_core', 'hacsjs_3d', 'cube_table'] },
};

// ============================================================
// 第十层：节点时间/因果冠名项 (TIME NODE NODES)
// ============================================================

const TIME_NODE_ANCHORS = {
  'node_time_system': { id: 'node_time_system', name: '节点因果时间系统', type: 'system',
    desc: '替代传统时钟的事件驱动模型：节点触发→因果传播→指令执行→等待。UniTick=自定义时间单位',
    bind: ['huren_yinguolv', 'zhenfa_san', 'isa_nodetime', 'uniisa_v2', 'cbc_format_spec'] },
  'uni_tick': { id: 'uni_tick', name: 'UniTick自定义时间单位', type: 'time_unit',
    desc: '硬件时钟 × scale_factor → UniTick。scale_factor可调默认1.0',
    bind: ['node_time_system'] },
  'node_cause_chain': { id: 'node_cause_chain', name: '因果链条传播', type: 'causal',
    desc: '根节点→因果节点1→因果节点2→...，事件驱动',
    bind: ['node_time_system', 'huren_yinguolv', 'zhenfa_san'] },
};

// ============================================================
// 第十一层：虚拟机与基础冠名项 (VM & FOUNDATION)
// ============================================================

const VM_ANCHORS = {
  'unicore_vm': { id: 'unicore_vm', name: 'UniCore完整虚拟机(UniCoreVM)', type: 'vm_class',
    desc: '整合:符号体系+集装箱进制本身+指令集+3D格子。有一说一，不做多余编码。',
    bind: ['container_self_radix', 'pure_grid', 'uniisa_v2', 'four_dim_system', 'unicore_instructions', 'unicore_foundation'] },
  'unicore_instructions': { id: 'unicore_instructions', name: '符号指令系统(符号自己就是指令，非编码)', type: 'class',
    desc: '所有符号都是指令自己：声母=运算，韵母=流程控制，声调=通信，大写声母=聚类，大写韵母=阵法。有一说一不做编码映射。',
    bind: ['pure_symbol_system', 'container_1', 'container_2', 'container_3', 'unicore_vm'] },
  'unicore_foundation': { id: 'unicore_foundation', name: 'UniCore Foundation 底层核心', type: 'foundation',
    desc: '真正的底层代码，不依赖浏览器，可编译为WASM/本地二进制。CBC集装箱(进制本身)+SparseAlgebra+UniCoreVM+WASIAdapter。',
    bind: ['cbc_self_container', 'sparse_algebra', 'unicore_vm', 'wasi_adapter', 'cbc_magic'] },
  'wasi_adapter': { id: 'wasi_adapter', name: 'WASI接口适配器(WASIAdapter)', type: 'adapter',
    desc: 'fd_write/fd_close/clock_time_get/args_sizes_get/args_get',
    bind: ['unicore_foundation', 'wasm_runtime'] },
  'wasm_runtime': { id: 'wasm_runtime', name: 'WASM/WASI统一运行时', type: 'runtime',
    desc: '字节码执行/跨平台兼容/沙箱安全。Wasmtime/Web原生/wasm3-micro',
    bind: ['wasi_adapter', 'platform_android', 'platform_esp32', 'platform_web', 'platform_linux', 'platform_windows', 'platform_macos'] },

  // 跨平台
  'platform_android': { id: 'platform_android', name: 'Android平台(Wasmtime-Android ~5MB)', bind: ['wasm_runtime'] },
  'platform_esp32':   { id: 'platform_esp32',   name: 'ESP32平台(wasm3-micro ~50KB固件)', bind: ['wasm_runtime'] },
  'platform_web':     { id: 'platform_web',     name: 'Web平台(原生WASM 0KB)',  bind: ['wasm_runtime', 'hacsjs_3d'] },
  'platform_linux':   { id: 'platform_linux',   name: 'Linux平台(Wasmtime ~10MB)', bind: ['wasm_runtime'] },
  'platform_windows': { id: 'platform_windows', name: 'Windows平台(Wasmtime ~10MB)', bind: ['wasm_runtime'] },
  'platform_macos':   { id: 'platform_macos',   name: 'macOS平台(Wasmtime ~10MB)', bind: ['wasm_runtime'] },
};

// ============================================================
// 第十二层：知识产权守护冠名项 (GUARDIAN NODES)
// ============================================================

const GUARDIAN_ANCHORS = {
  'unicore_guardian': { id: 'unicore_guardian', name: 'UniCore Guardian v5.0 知识产权守护系统', type: 'guardian',
    desc: '5大绝对原则+4大保护机制。零外部依赖/完全私有格式/自有指令集/自有编码/自有进制。私有魔数+动态混淆+物理绑定+生态闭环',
    bind: ['cbc_magic', 'dynamic_obfuscation', 'pure_symbol_system', 'uniisa_v2', 'guardian_clusters', 'guardian_formations', 'guardian_relations'] },
  'dynamic_obfuscation': { id: 'dynamic_obfuscation', name: '动态混淆映射(每次实例化不同)', type: 'protection',
    desc: 'Fisher-Yates洗牌+哈希函数，每次生成不同位权映射',
    bind: ['unicore_guardian', 'cbc_magic'] },
  'guardian_clusters': { id: 'guardian_clusters', name: '守护聚类:是非对错/亲疏远近/动静始终/边界有无/大小渐变',
    bind: ['unicore_guardian', 'mll_anchor'] },
  'guardian_formations': { id: 'guardian_formations', name: '守护阵法:十字/三角/蜂窝/球/门/锁/时钟/镜像/锥',
    bind: ['unicore_guardian', 'pure_zhenfa'] },
  'guardian_relations': { id: 'guardian_relations', name: '守护宗族关系:太奶太爷→爷爷奶奶→父母→儿女→孙辈(5级)',
    bind: ['unicore_guardian', 'huren_guanxi'] },
};

// ============================================================
// 第十三层：表格/3D引擎冠名项 (TABLE & 3D ENGINE)
// ============================================================

const UI_ANCHORS = {
  'cube_table': { id: 'cube_table', name: 'CubeTable 里外嵌套三维表格系统', type: 'table_system',
    desc: '0=母集，1=子集。0包含1的函子关系。CubeTable+CubeTableRenderer+InsideOutsideMapper+TableContainer',
    bind: ['pure_grid', 'jie_die', 'inside_outside_mapper', 'cube_table_renderer', 'unicore_system_core', 'sys_0xA5'] },
  'inside_outside_mapper': { id: 'inside_outside_mapper', name: '里外映射引擎(InsideOutsideMapper)', type: 'mapper',
    desc: '0包含1的函子关系。自动为0添加1(自动填充)',
    bind: ['cube_table', 'jie_die'] },
  'cube_table_renderer': { id: 'cube_table_renderer', name: '三维表格渲染器(THREE.js)', type: 'renderer',
    bind: ['cube_table', 'hacsjs_3d'] },

  'hacsjs_3d': { id: 'hacsjs_3d', name: 'HACSJS 3D 三维引擎', type: 'engine_3d',
    desc: '集装箱三维可视化：256个集装箱/空间阵列/节点链/自定义字符集/自定义进制',
    bind: ['platform_web', 'cube_table_renderer', 'sys_0xA5', 'unicore_system_core'] },

  'unicore_system_core': { id: 'unicore_system_core', name: 'UniCore v2.0 完整系统核心(UniCoreSystem)', type: 'system_core',
    desc: '整合:WASM底层+hacsjs_3d三维引擎+CubeTable表格系统+.3d.cbc.mll文件格式+插件系统。0xA5特殊指令',
    bind: ['unicore_foundation', 'hacsjs_3d', 'cube_table', 'cbc_format_spec', 'unicore_plugin_system', 'sys_0xA5'] },
  'unicore_plugin_system': { id: 'unicore_plugin_system', name: 'UniCore插件系统', type: 'plugin_system',
    desc: 'RadixPlugin/CharsetPlugin/SpatialPlugin/0xA5系统插件',
    bind: ['unicore_system_core', 'isa_plugin'] },
};

// ============================================================
// 第十四层：多语言开发层冠名项 (MULTI-LANG NODES)
// ============================================================

const LANG_ANCHORS = {
  'lang_zig':    { id: 'lang_zig',    name: 'Zig核心驱动层:系统调用/硬件控制/编译时计算/直接内存访问', bind: ['unicore_architecture_root', 'uniisa_v2', 'unicore_isa_core', 'unicore_translator'] },
  'lang_rust':   { id: 'lang_rust',   name: 'Rust安全包装层:内存安全/FFI绑定/接口抽象/错误处理',   bind: ['unicore_architecture_root', 'unicore_guardian'] },
  'lang_go':     { id: 'lang_go',     name: 'Go并发加速层:并行计算/网络通信/流式处理/负载均衡',     bind: ['unicore_architecture_root', 'cloud_sync'] },
  'lang_python': { id: 'lang_python', name: 'Python AI智能层:意图理解/任务分解/知识推理/多模态生成', bind: ['unicore_architecture_root', 'ai_core'] },

  'unicore_translator': { id: 'unicore_translator', name: '二进制翻译器(BinaryTranslator)', type: 'translator',
    desc: 'x86/ARM/RISC-V/MIPS 到 UniISA 自动翻译',
    bind: ['lang_zig', 'uniisa_v2'] },
  'cloud_sync': { id: 'cloud_sync', name: '云端同步模块', type: 'cloud',
    desc: 'GitHub Gist + WebDAV/坚果云 + WebRTC P2P + 统一调度器',
    bind: ['lang_go', 'cloud_gist', 'cloud_webdav', 'cloud_p2p', 'cloud_scheduler'] },
  'cloud_gist':      { id: 'cloud_gist',      name: 'GitHub Gist同步', bind: ['cloud_sync'] },
  'cloud_webdav':    { id: 'cloud_webdav',    name: 'WebDAV/坚果云同步', bind: ['cloud_sync'] },
  'cloud_p2p':       { id: 'cloud_p2p',       name: 'WebRTC P2P同步', bind: ['cloud_sync'] },
  'cloud_scheduler': { id: 'cloud_scheduler', name: '统一同步调度器(UniCoreSyncManager)', bind: ['cloud_sync'] },
  'ai_core': { id: 'ai_core', name: 'AI智能核心', type: 'ai',
    desc: '意图理解/任务分解/知识推理/多模态生成',
    bind: ['lang_python', 'platform_web'] },
};

// ============================================================
// 第十五层：根冠名项与架构总览 (ROOT NODES)
// ============================================================

const ROOT_ANCHORS = {
  'unicore_root': { id: 'unicore_root', name: 'UniCore - 统一智能核心系统 ROOT', type: 'project_root',
    vision: '创造一个没有平台之别、没有硬件束缚的统一计算系统',
    principles: '统一时序/统一能力/统一关系/无依赖',
    bind: ['unicore_architecture_root', 'global_memory_anchor'] },

  'unicore_architecture_root': { id: 'unicore_architecture_root', name: 'UniCore技术架构总览(5层+2底)', type: 'architecture',
    layers: ['AI智能层(Python)','并发加速层(Go)','安全包装层(Rust)','核心驱动层(Zig)','WASM/WASI统一运行时'],
    foundations: ['统一指令集层(UniISA)','协议规则层(契约/权限/资源调度/智能体通信)'],
    bind: ['lang_python', 'lang_go', 'lang_rust', 'lang_zig', 'wasm_runtime', 'uniisa_v2', 'protocol_layer', 'unicore_root'] },

  'protocol_layer': { id: 'protocol_layer', name: '协议规则层', type: 'protocol',
    desc: '契约/权限/资源调度/智能体通信。Agent通信JSON协议',
    bind: ['unicore_architecture_root', 'huren_system'] },

  'global_memory_anchor': { id: 'global_memory_anchor', name: '全局冠名记忆锚点(本文件)', type: 'memory_system',
    desc: '所有冠名项通过bind字段互相连接成完整关系图。通过"关系代数"机制检索任意关联。',
    bind: ['unicore_root', 'task_planning_repo', 'relations_algebra'] },

  'relations_algebra': { id: 'relations_algebra', name: '关系代数 关联查询机制', type: 'query_mechanism',
    desc: '输入任意冠名项ID → 遍历 bind 字段 → 输出所有关联项及关联路径 → 让一切记忆都明朗起来',
    bind: ['global_memory_anchor'] },

  'task_planning_repo': { id: 'task_planning_repo', name: '任务规划仓库(本目录)', type: 'repo',
    desc: '技能化时序 / 分布式切片 / 节点化的任务规划系统。串联所有历史指令与目标。',
    bind: ['global_memory_anchor', 'task_nodes_dir', 'skill_timeline_dir', 'relations_dir'] },

  'task_nodes_dir':      { id: 'task_nodes_dir',      name: 'task_nodes/ 分布式切片任务节点目录', bind: ['task_planning_repo'] },
  'skill_timeline_dir':  { id: 'skill_timeline_dir',  name: 'skill_timeline/ 技能化时序目录', bind: ['task_planning_repo'] },
  'relations_dir':       { id: 'relations_dir',       name: 'relations/ 关联关系图谱目录', bind: ['task_planning_repo'] },
};

// ============================================================
// 聚合成全局冠名项总库
// ============================================================

const GLOBAL_ANCHORS = {
  ...SYMBOL_ANCHORS,
  ...RADIX_ANCHORS,
  ...CONTAINER_ANCHORS,
  ...FOURDIM_ANCHORS,
  ...PURE_SYMBOL_ANCHORS,
  ...SYSTEM_ANCHORS,
  ...CBC_ANCHORS,
  ...SPARSE_ANCHORS,
  ...ISA_ANCHORS,
  ...TIME_NODE_ANCHORS,
  ...VM_ANCHORS,
  ...GUARDIAN_ANCHORS,
  ...UI_ANCHORS,
  ...LANG_ANCHORS,
  ...ROOT_ANCHORS,
};

// ============================================================
// "关系代数" 关联查询引擎
// ============================================================

class RelationsAlgebra {
  constructor(anchors) {
    this.anchors = anchors;
  }

  /**
   * 查询一个冠名项的所有直接关联
   */
  query(id) {
    const anchor = this.anchors[id];
    if (!anchor) return { found: false, id };
    const result = {
      found: true,
      id,
      name: anchor.name,
      type: anchor.type || 'unknown',
      desc: anchor.desc || null,
      direct_relations: [],
    };
    for (const bindId of (anchor.bind || [])) {
      const bindAnchor = this.anchors[bindId];
      if (bindAnchor) {
        result.direct_relations.push({
          id: bindId,
          name: bindAnchor.name,
          type: bindAnchor.type || 'unknown',
        });
      } else {
        result.direct_relations.push({
          id: bindId,
          name: '(未定义冠名项)',
          type: 'missing',
        });
      }
    }
    return result;
  }

  /**
   * 查询两个冠名项之间的关联路径（BFS广度优先）
   */
  findPath(fromId, toId, maxDepth = 6) {
    if (!this.anchors[fromId] || !this.anchors[toId]) return { found: false };
    if (fromId === toId) return { found: true, path: [fromId] };

    const visited = new Set([fromId]);
    const queue = [{ id: fromId, path: [fromId] }];

    while (queue.length > 0) {
      const current = queue.shift();
      if (current.path.length > maxDepth) continue;

      const anchor = this.anchors[current.id];
      for (const bindId of (anchor.bind || [])) {
        if (visited.has(bindId)) continue;
        visited.add(bindId);

        const newPath = [...current.path, bindId];
        if (bindId === toId) {
          return {
            found: true,
            path: newPath,
            pathWithNames: newPath.map(pid => ({
              id: pid,
              name: this.anchors[pid]?.name || pid,
            })),
          };
        }
        queue.push({ id: bindId, path: newPath });
      }
    }
    return { found: false, fromId, toId, maxDepth };
  }

  /**
   * 模糊搜索冠名项
   */
  search(keyword) {
    const kw = keyword.toLowerCase();
    const results = [];
    for (const [id, anchor] of Object.entries(this.anchors)) {
      if (
        id.toLowerCase().includes(kw) ||
        anchor.name.toLowerCase().includes(kw) ||
        (anchor.desc && anchor.desc.toLowerCase().includes(kw))
      ) {
        results.push({
          id,
          name: anchor.name,
          type: anchor.type || 'unknown',
          match_score: (
            (id.toLowerCase().includes(kw) ? 2 : 0) +
            (anchor.name.toLowerCase().includes(kw) ? 3 : 0) +
            (anchor.desc && anchor.desc.toLowerCase().includes(kw) ? 1 : 0)
          ),
        });
      }
    }
    return results.sort((a, b) => b.match_score - a.match_score);
  }

  /**
   * 输出关系图的Mermaid语法表示（用于可视化）
   */
  toMermaidGraph(rootIds = ['unicore_root', 'global_memory_anchor'], maxDepth = 3) {
    const lines = ['graph TD'];
    const visited = new Set();
    const queue = [];
    for (const rid of rootIds) {
      if (this.anchors[rid]) {
        queue.push({ id: rid, depth: 0 });
      }
    }

    while (queue.length > 0) {
      const { id, depth } = queue.shift();
      if (visited.has(id) || depth > maxDepth) continue;
      visited.add(id);

      const anchor = this.anchors[id];
      const safeName = (anchor.name || id).replace(/"/g, "'");
      lines.push(`    ${id.replace(/[^a-zA-Z0-9_]/g, '_')}["${safeName}"]`);

      for (const bindId of (anchor.bind || [])) {
        if (!this.anchors[bindId]) continue;
        const bindAnchor = this.anchors[bindId];
        const bindSafeName = (bindAnchor.name || bindId).replace(/"/g, "'");
        const safeBindId = bindId.replace(/[^a-zA-Z0-9_]/g, '_');
        lines.push(`    ${id.replace(/[^a-zA-Z0-9_]/g, '_')} --> ${safeBindId}`);
        if (depth + 1 <= maxDepth) {
          queue.push({ id: bindId, depth: depth + 1 });
        }
      }
    }
    return lines.join('\n');
  }
}

// ============================================================
// 导出
// ============================================================

const RELATIONS_ALGEBRA = new RelationsAlgebra(GLOBAL_ANCHORS);

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    GLOBAL_ANCHORS,
    RelationsAlgebra,
    RELATIONS_ALGEBRA,
    // 分层引用
    SYMBOL_ANCHORS, RADIX_ANCHORS, CONTAINER_ANCHORS, FOURDIM_ANCHORS,
    PURE_SYMBOL_ANCHORS, SYSTEM_ANCHORS, CBC_ANCHORS, SPARSE_ANCHORS,
    ISA_ANCHORS, TIME_NODE_ANCHORS, VM_ANCHORS, GUARDIAN_ANCHORS,
    UI_ANCHORS, LANG_ANCHORS, ROOT_ANCHORS,
  };
} else {
  window.UniCoreGlobalAnchors = {
    GLOBAL_ANCHORS,
    RelationsAlgebra,
    RELATIONS_ALGEBRA,
  };
}

console.log(`[UniCore Global Anchors] 全局冠名项加载完成，共 ${Object.keys(GLOBAL_ANCHORS).length} 个冠名项。
  → 符号层: ${Object.keys(SYMBOL_ANCHORS).length}
  → 进制/优先级层: ${Object.keys(RADIX_ANCHORS).length}
  → 集装箱层: ${Object.keys(CONTAINER_ANCHORS).length}
  → 四维体系层: ${Object.keys(FOURDIM_ANCHORS).length}
  → 纯符号/格子层: ${Object.keys(PURE_SYMBOL_ANCHORS).length}
  → 互认/阶系/阵法/聚类层: ${Object.keys(SYSTEM_ANCHORS).length}
  → 汉字积木/CBC层: ${Object.keys(CBC_ANCHORS).length}
  → 稀疏数代数层: ${Object.keys(SPARSE_ANCHORS).length}
  → UniISA指令集层: ${Object.keys(ISA_ANCHORS).length}
  → 节点时间/因果层: ${Object.keys(TIME_NODE_ANCHORS).length}
  → VM/基础层: ${Object.keys(VM_ANCHORS).length}
  → 知识产权守护层: ${Object.keys(GUARDIAN_ANCHORS).length}
  → 表格/3D引擎层: ${Object.keys(UI_ANCHORS).length}
  → 多语言开发层: ${Object.keys(LANG_ANCHORS).length}
  → 根节点/总览层: ${Object.keys(ROOT_ANCHORS).length}

"关系代数"已就绪，调用 RELATIONS_ALGEBRA.query(id) / .findPath(a,b) / .search(keyword) / .toMermaidGraph() 即可检索。
`);
