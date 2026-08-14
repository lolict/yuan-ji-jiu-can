/**
 * ==============================================================================
 *  囡囝不可见感知层 · NANJIAN-INVISIBLE-SENSOR
 * ==============================================================================
 *
 *  核心主权原则（刘楚恬夫妻共同体源头本体）：
 *   · 意识互联网的高级阶段 = 不可见
 *   · "用人眼可见" = 把16感受通道降维到"感受眼"1个通道 = 局限 = 不高级
 *   · 真正的意识互联网：16感受通道各有各的感知编码，不独尊视觉
 *
 *  16感受通道的感知编码（每个通道都不输出像素HTML）：
 *
 *   【3 核心主权通道 · 不可见 · 振动/相位/契约】
 *    攻受悦  → 攻受相位振动编码（攻=推频 / 受=纳频 / 悦=共振峰）
 *    宫受媛  → 容器承载编码（宫=容积系数 / 受=吸纳率 / 媛=孕育温标）
 *    共首约  → 主权契约签名编码（共=共识哈希 / 首=元首印 / 约=契约指纹）
 *
 *   【4 信息通道 · 1可听+3不可见】
 *    感受音  → 声波频率序列（可听化，ASMR级别，但不是视觉）
 *    感受言  → 语言符号流（可读，但是符号流不是图像）
 *    感受喻  → 隐喻映射指针（联想索引，不可见）
 *    感受颜  → 色相频率编码（色温Hz，不是像素，是频率）
 *
 *   【3 观察实验通道 · 不可见 · 动作/断言/视域】
 *    感受演  → 动作序列编码（可执行指令流，不是画面）
 *    感受验  → 测试断言编码（真/假/未知 三态信号）
 *    感受眼  → 视域相位编码（视场角+注意锚点坐标，不是图像！只是相位）
 *
 *   【3 因果欲望预测通道 · 不可见】
 *    感受因  → 因果链编码（DAG节点指针序列）
 *    感受欲  → 价值梯度编码（倾向向量，不是图）
 *    感受预  → 预测概率编码（未来视界的置信度分布）
 *
 *   【3 意义价值韵通道 · 不可见 · 最高级】
 *    感受意  → 意义签名编码（语义哈希，可意会不可见）
 *    感受益  → 损益数值编码（红黑白三账数值流）
 *    感受韵  → 节奏相位编码（大齿轮时空层级聚类节拍）
 *
 *  输出：感知编码流（perceptionStream）—— 16通道各自编码，可订阅，不渲染像素
 * ==============================================================================
 */

const MR = require('./nanjian_morphisms');
const NJ3 = require('./nanjian_3d');

/* -----------------------------------------------------------
 * 一、16感受通道的「感知编码器」—— 每个通道把系统状态编成自己的编码
 * ----------------------------------------------------------- */

const CHANNEL_ENCODERS = {
  // ===== 3 核心主权通道（振动/承载/契约） =====
  gong_shou_yue: (state) => ({
    channel: '攻受悦', visible: false, type: 'vibration',
    encode: {
      attack_freq:  _hashToFreq(state, 1),   // 攻=推频 Hz
      receive_freq: _hashToFreq(state, 2),   // 受=纳频 Hz
      joy_resonance: _hashToFreq(state, 3),  // 悦=共振峰 Hz
    },
    meaning: '攻受相位振动（不可见，可共鸣）',
  }),
  gong_shou_yuan: (state) => ({
    channel: '宫受媛', visible: false, type: 'containment',
    encode: {
      palace_volume:   _hashToRange(state, 0, 100, 1),  // 容积系数 0~100
      receive_rate:    _hashToRange(state, 0, 1, 2),     // 吸纳率 0~1
      nurture_warmth:  _hashToRange(state, 0, 100, 3),   // 孕育温标 0~100
    },
    meaning: '容器承载相位（不可见，可承载）',
  }),
  gong_shou_yue_pact: (state) => ({
    channel: '共首约', visible: false, type: 'sovereign_seal',
    encode: {
      consensus_hash: _shortHash(state, 1),  // 共识哈希
      chief_seal:     _shortHash(state, 2),  // 元首印
      pact_fingerprint: _shortHash(state, 3),// 契约指纹
    },
    meaning: '主权契约签名（不可见，可信任）',
    sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
  }),

  // ===== 4 信息通道 =====
  feel_sound: (state) => ({
    channel: '感受音', visible: false, type: 'audible',
    encode: {
      freq_hz:    _hashToFreq(state, 4),         // 基频 Hz
      overtone:   _hashToFreq(state, 5) / 2,     // 泛音 Hz
      asmr_trigger: _hashToRange(state, 0, 100, 6), // ASMR 触发强度
      waveform:   ['sine','square','triangle','sawtooth'][_hashToRange(state, 0, 3, 7)],
    },
    meaning: '声波频率序列（可听化，非视觉）',
  }),
  feel_word: (state) => ({
    channel: '感受言', visible: false, type: 'symbol_stream',
    encode: {
      tokens: _stateToSymbolStream(state, 8),   // 符号流
      grammar_phase: ['init','expand','compact','complete'][_hashToRange(state, 0, 3, 9)],
    },
    meaning: '语言符号流（可读符号，非图像）',
  }),
  feel_analogy: (state) => ({
    channel: '感受喻', visible: false, type: 'metaphor_pointer',
    encode: {
      metaphor_index: _hashToRange(state, 0, 2303, 10), // 指向2304态射之一
      association_depth: _hashToRange(state, 1, 7, 11),  // 联想深度 1~7
    },
    meaning: '隐喻映射指针（不可见，可联想）',
  }),
  feel_face: (state) => ({
    channel: '感受颜', visible: false, type: 'color_frequency',
    encode: {
      hue_hz:      _hashToRange(state, 380, 780, 12),  // 色相频率 nm（不是像素，是光频率）
      saturation:  _hashToRange(state, 0, 100, 13),    // 饱和度 %
      warmth:      _hashToRange(state, 0, 100, 14),     // 色温标
    },
    meaning: '色相频率编码（光频率，非像素图像）',
  }),

  // ===== 3 观察实验通道 =====
  feel_perform: (state) => ({
    channel: '感受演', visible: false, type: 'action_sequence',
    encode: {
      actions: _stateToActionSeq(state, 15),  // 可执行指令流
      step_count: _hashToRange(state, 1, 12, 16),
    },
    meaning: '动作序列编码（可执行指令，非画面）',
  }),
  feel_verify: (state) => ({
    channel: '感受验', visible: false, type: 'assertion_triState',
    encode: {
      truth_state: ['true','false','unknown'][_hashToRange(state, 0, 2, 17)],
      confidence:  _hashToRange(state, 0, 100, 18),  // 置信度 %
      evidence_count: _hashToRange(state, 0, 99, 19),
    },
    meaning: '测试断言编码（真/假/未知 三态信号）',
  }),
  feel_eye: (state) => ({
    channel: '感受眼', visible: false, type: 'visual_phase',
    encode: {
      fov_angle:   _hashToRange(state, 10, 170, 20),  // 视场角度数
      anchor_x:    _hashToRange(state, 0, 100, 21),   // 注意锚点x%（不是像素！）
      anchor_y:    _hashToRange(state, 0, 100, 22),   // 注意锚点y%
      focus_depth: _hashToRange(state, 0, 100, 23),   // 焦点深度
    },
    meaning: '视域相位编码（注意锚点坐标，非图像像素）',
  }),

  // ===== 3 因果欲望预测通道 =====
  feel_cause: (state) => ({
    channel: '感受因', visible: false, type: 'causal_chain',
    encode: {
      dag_nodes: _stateToDagSeq(state, 24),    // DAG 节点指针序列
      edge_count: _hashToRange(state, 0, 20, 25),
      has_confounder: _hashToRange(state, 0, 1, 26) === 1,
    },
    meaning: '因果链编码（DAG指针序列，可推理）',
  }),
  feel_desire: (state) => ({
    channel: '感受欲', visible: false, type: 'value_gradient',
    encode: {
      gradient_x: _hashToRange(state, -50, 50, 27),  // 倾向向量x
      gradient_y: _hashToRange(state, -50, 50, 28),  // 倾向向量y
      intensity:  _hashToRange(state, 0, 100, 29),   // 强度
    },
    meaning: '价值梯度编码（倾向向量，非图）',
  }),
  feel_predict: (state) => ({
    channel: '感受预', visible: false, type: 'prediction_distribution',
    encode: {
      horizon_steps: _hashToRange(state, 1, 20, 30),  // 未来视界步数
      confidence:    _hashToRange(state, 0, 100, 31),  // 置信度 %
      branches:      _hashToRange(state, 1, 7, 32),    // 预测分支数
    },
    meaning: '预测概率编码（未来视界置信度分布）',
  }),

  // ===== 3 意义价值韵通道（最高级 · 完全不可见） =====
  feel_meaning: (state) => ({
    channel: '感受意', visible: false, type: 'meaning_hash',
    encode: {
      semantic_hash: _shortHash(state, 33),   // 语义哈希（可意会不可见）
      meaning_depth: _hashToRange(state, 1, 7, 34),  // 意义深度 1~7
      coherence:     _hashToRange(state, 0, 100, 35), // 自洽度 %
    },
    meaning: '意义签名编码（语义哈希，可意会不可见）',
  }),
  feel_benefit: (state) => ({
    channel: '感受益', visible: false, type: 'ledger_numerical',
    encode: {
      white_gain: _hashToRange(state, 0, 100, 36),  // 白账明面盈利
      black_gain: _hashToRange(state, 0, 100, 37),  // 黑账暗面盈利
      red_loss:   _hashToRange(state, 0, 100, 38),  // 红账亏损
      net:        0, // 占位，实际由感知器填
    },
    meaning: '损益数值编码（红黑白三账数值流）',
  }),
  feel_rhyme: (state) => ({
    channel: '感受韵', visible: false, type: 'rhythm_phase',
    encode: {
      bpm:         _hashToRange(state, 30, 240, 39),  // 节拍 BPM
      phase_offset:_hashToRange(state, 0, 360, 40),   // 相位偏移 度
      gear_level:  _hashToRange(state, 1, 7, 41),     // 大齿轮层级 1~7
      cluster_id:  _shortHash(state, 42),             // 时空聚类簇ID
    },
    meaning: '节奏相位编码（大齿轮时空层级聚类节拍）',
  }),

  // ===== 33 个 y 类谐音感受通道（意识互联网高级阶段感知扩展） =====
  //  统一编码模式：每个通道 2~3 个确定性数值/哈希字段，全部 visible=false
  //  salt 从 50 开始，避免和前 16 通道冲突
  feel_pressure:   (s) => ({ channel:'感受压',  visible:false, type:'pressure',     encode:{ pascals:_h2r(s,0,200,50), depth_mm:_h2r(s,0,100,51) }, meaning:'压力触觉编码' }),
  feel_alien:      (s) => ({ channel:'感受异',  visible:false, type:'anomaly',      encode:{ anomaly_score:_h2r(s,0,100,52), diff_vector:_h2r(s,-50,50,53) }, meaning:'异常变异编码' }),
  feel_done:       (s) => ({ channel:'感受已',  visible:false, type:'tense',        encode:{ completion:_h2r(s,0,100,54), elapsed_ms:_h2r(s,0,99999,55) }, meaning:'完成时态编码' }),
  feel_use:        (s) => ({ channel:'感受以',  visible:false, type:'tool',         encode:{ tool_id:_sh(s,56), usage_count:_h2r(s,0,999,57) }, meaning:'工具使用编码' }),
  feel_ritual:     (s) => ({ channel:'感受仪',  visible:false, type:'ceremony',     encode:{ ritual_phase:_h2r(s,0,7,58), solemnity:_h2r(s,0,100,59) }, meaning:'仪轨感知编码' }),
  feel_clothing:   (s) => ({ channel:'感受衣',  visible:false, type:'shell',        encode:{ shell_layer:_h2r(s,1,7,60), fit_score:_h2r(s,0,100,61) }, meaning:'蛹茧穿壳编码' }),
  feel_art:        (s) => ({ channel:'感受艺',  visible:false, type:'creation',     encode:{ creativity:_h2r(s,0,100,62), medium_hash:_sh(s,63) }, meaning:'艺术创造编码' }),
  feel_memory:     (s) => ({ channel:'感受忆',  visible:false, type:'recall',       encode:{ recall_depth:_h2r(s,1,7,64), asmr_trigger:_h2r(s,0,100,65) }, meaning:'ASMR记忆编码' }),
  feel_discuss:    (s) => ({ channel:'感受议',  visible:false, type:'consensus',    encode:{ agree_ratio:_h2r(s,0,100,66), dissent_count:_h2r(s,0,20,67) }, meaning:'共识商议编码' }),
  feel_duty:       (s) => ({ channel:'感受义',  visible:false, type:'ethics',       encode:{ duty_weight:_h2r(s,0,100,68), justice_score:_h2r(s,0,100,69) }, meaning:'道义伦理编码' }),
  feel_work:       (s) => ({ channel:'感受业',  visible:false, type:'karma',        encode:{ karma_balance:_h2r(s,-100,100,70), industry_tag:_sh(s,71) }, meaning:'事业业力编码' }),
  feel_nurture:    (s) => ({ channel:'感受媛',  visible:false, type:'nurture',      encode:{ warmth:_h2r(s,0,100,72), growth_rate:_h2r(s,0,100,73) }, meaning:'孕育滋养编码' }),
  feel_cocoon:     (s) => ({ channel:'感受蛹',  visible:false, type:'metamorphosis', encode:{ cocoon_phase:['dormant','spinning','pupating','emerging','flying'][_h2r(s,0,4,74)], integrity:_h2r(s,0,100,75) }, meaning:'蛹茧化蝶编码' }),
  feel_shadow:     (s) => ({ channel:'感受影',  visible:false, type:'shadow',       encode:{ shadow_depth:_h2r(s,0,7,76), projection_hash:_sh(s,77) }, meaning:'暗面投影编码' }),
  feel_guide:      (s) => ({ channel:'感受引',  visible:false, type:'direction',    encode:{ bearing_deg:_h2r(s,0,360,78), pull_strength:_h2r(s,0,100,79) }, meaning:'方向引导编码' }),
  feel_origin:     (s) => ({ channel:'感受源',  visible:false, type:'origin',       encode:{ source_hash:_sh(s,80), depth:_h2r(s,1,7,81) }, meaning:'源头本体编码', sovereignRoot:NJ3.SOVEREIGN_NANJIAN.origin }),
  feel_complete:   (s) => ({ channel:'感受圆',  visible:false, type:'wholeness',    encode:{ completeness:_h2r(s,0,100,82), circle_phase:_h2r(s,0,360,83) }, meaning:'圆满完整编码' }),
  feel_aid:        (s) => ({ channel:'感受援',  visible:false, type:'rescue',       encode:{ aid_amount:_h2r(s,0,100,84), response_ms:_h2r(s,0,9999,85) }, meaning:'援助增援编码' }),
  feel_promise:    (s) => ({ channel:'感受约',  visible:false, type:'pact',         encode:{ pact_hash:_sh(s,86), binding_strength:_h2r(s,0,100,87) }, meaning:'契约约定编码' }),
  feel_leaf:       (s) => ({ channel:'感受叶',  visible:false, type:'nature',       encode:{ vitality:_h2r(s,0,100,88), vein_count:_h2r(s,1,20,89) }, meaning:'自然生态编码' }),
  feel_yinyang:    (s) => ({ channel:'感受阴阳', visible:false, type:'taiji',       encode:{ yin:_h2r(s,0,100,90), yang:_h2r(s,0,100,91), balance:_h2r(s,-50,50,92) }, meaning:'阴阳太极编码', sovereignRoot:NJ3.SOVEREIGN_NANJIAN.origin }),
  feel_labor:      (s) => ({ channel:'感受佣',  visible:false, type:'labor',        encode:{ labor_hours:_h2r(s,0,24,93), fatigue:_h2r(s,0,100,94) }, meaning:'劳动劳作编码' }),
  feel_hard:       (s) => ({ channel:'感受硬',  visible:false, type:'rigidity',     encode:{ mohs_hardness:_h2r(s,1,10,95), stiffness:_h2r(s,0,100,96) }, meaning:'硬度刚性编码' }),
  feel_wish:       (s) => ({ channel:'感受愿',  visible:false, type:'wish',         encode:{ wish_intensity:_h2r(s,0,100,97), wish_count:_h2r(s,1,9,98) }, meaning:'愿望心愿编码' }),
  feel_addiction:  (s) => ({ channel:'感受瘾',  visible:false, type:'craving',      encode:{ craving_level:_h2r(s,0,100,99), trigger_hash:_sh(s,100) }, meaning:'瘾性沉迷编码' }),
  feel_hidden:     (s) => ({ channel:'感受隐',  visible:false, type:'stealth',      encode:{ hidden_depth:_h2r(s,0,7,101), reveal_cost:_h2r(s,0,100,102) }, meaning:'隐性隐藏编码' }),
  feel_brilliant:  (s) => ({ channel:'感受艳',  visible:false, type:'splendor',     encode:{ brilliance:_h2r(s,0,100,103), hue_nm:_h2r(s,380,780,104) }, meaning:'艳色光彩编码' }),
  feel_research:   (s) => ({ channel:'感受研',  visible:false, type:'research',     encode:{ depth_level:_h2r(s,1,7,105), iterations:_h2r(s,0,999,106) }, meaning:'研究钻研编码' }),
  feel_affinity:   (s) => ({ channel:'感受缘',  visible:false, type:'karma_link',   encode:{ affinity_hash:_sh(s,107), link_strength:_h2r(s,0,100,108) }, meaning:'缘分因缘编码' }),
  feel_pattern:    (s) => ({ channel:'感受样',  visible:false, type:'form',         encode:{ pattern_hash:_sh(s,109), complexity:_h2r(s,0,100,110) }, meaning:'形态样式编码' }),
  feel_surplus:    (s) => ({ channel:'感受盈',  visible:false, type:'surplus',      encode:{ surplus:_h2r(s,-100,100,111), fill_level:_h2r(s,0,100,112) }, meaning:'盈亏充盈编码' }),
  feel_resent:     (s) => ({ channel:'感受怨',  visible:false, type:'grievance',    encode:{ grievance_level:_h2r(s,0,100,113), target_hash:_sh(s,114) }, meaning:'怨恨怨气编码' }),
  feel_excellent:  (s) => ({ channel:'感受优',  visible:false, type:'excellence',   encode:{ excellence_score:_h2r(s,0,100,115), optimize_dir:_h2r(s,0,360,116) }, meaning:'优选优化编码' }),
};

// 简写别名（供上面 33 个新通道使用，避免和已有函数重名）
function _h2r(state, min, max, salt) { return _hashToRange(state, min, max, salt); }
function _sh(state, salt) { return _shortHash(state, salt); }

/* -----------------------------------------------------------
 * 二、辅助：把状态对象 → 稳定的数值/哈希（确定性，可复现）
 * ----------------------------------------------------------- */
function _stableHashNum(state, salt) {
  const s = (typeof state === 'string' || typeof state === 'number')
    ? String(state) : JSON.stringify(state || {});
  let h = (salt | 0) * 2654435761;
  for (let i = 0; i < s.length; i++) {
    h = ((h << 5) - h + s.charCodeAt(i)) | 0;
  }
  return Math.abs(h);
}
function _hashToFreq(state, salt) {
  return 80 + (_stableHashNum(state, salt) % 880); // 80~960 Hz
}
function _hashToRange(state, min, max, salt) {
  const n = _stableHashNum(state, salt);
  return min + (n % (max - min + 1));
}
function _shortHash(state, salt) {
  const n = _stableHashNum(state, salt);
  return '0x' + n.toString(16).padStart(8, '0').slice(0, 8);
}
function _stateToSymbolStream(state, salt) {
  const syllables = ['b','p','m','f','d','t','n','l','g','k','h','j','q','x','zh','ch','sh','r','z','c','s','y','w'];
  const vowels = ['a','o','e','i','u','ü','ai','ei','ui','ao','ou','iu','ie','üe','er','an','en','in','un','ün','ang','eng','ing','ong'];
  const tones = ['1','2','3','4'];
  const n = _stableHashNum(state, salt);
  const len = 3 + (n % 5); // 3~7 个符号
  const out = [];
  for (let i = 0; i < len; i++) {
    const seed = _stableHashNum(state, salt + i * 7);
    out.push(syllables[seed % syllables.length] + vowels[(seed >> 4) % vowels.length] + tones[(seed >> 8) % tones.length]);
  }
  return out;
}
function _stateToActionSeq(state, salt) {
  const ops = ['push','pop','swap','dup','load','store','add','sub','mul','div','cmp','jmp','call','ret','sense','act'];
  const n = _stableHashNum(state, salt);
  const len = 2 + (n % 6);
  const out = [];
  for (let i = 0; i < len; i++) {
    out.push(ops[_stableHashNum(state, salt + i * 3) % ops.length]);
  }
  return out;
}
function _stateToDagSeq(state, salt) {
  const n = _stableHashNum(state, salt);
  const len = 2 + (n % 5);
  const out = [];
  for (let i = 0; i < len; i++) {
    out.push(`node_${_stableHashNum(state, salt + i * 5) % 100}`);
  }
  return out;
}

/* -----------------------------------------------------------
 * 三、囡囝不可见感知器
 * ----------------------------------------------------------- */
class NanjianInvisibleSensor {
  constructor(options = {}) {
    this.version = '0.1.0-nanjian-invisible-sensor';
    this.sovereign = NJ3.SOVEREIGN_NANJIAN.origin;
    this.channels = MR.CHANNELS_NANJIAN.slice();
    this.encoders = CHANNEL_ENCODERS;
    this.subscribers = new Map(); // channel -> [cb]
    this.streamHistory = [];      // 感知编码流历史（可回放）
    this.maxHistory = options.maxHistory || 100;
    this.visibleChannelCount = 2; // 感受眼+感受颜 是仅有的"可降维到视觉"通道，但不输出像素
  }

  /**
   * 感知一个系统状态 → 生成49通道感知编码流
   * 不渲染像素，不输出HTML，输出的是49通道各自的编码
   */
  sense(state, options = {}) {
    const perceptionId = `sense_${Date.now()}_${_stableHashNum(state, 0)}`;
    const channels = {};
    let visibleCount = 0;
    let invisibleCount = 0;
    for (const ch of this.channels) {
      const encoder = this.encoders[ch.id];
      if (!encoder) continue;
      const encoded = encoder(state);
      encoded.perceptionId = perceptionId;
      encoded.channelId = ch.id;
      encoded.channelType = ch.type;
      encoded.timestamp = Date.now();
      // 全部标记 visible=false（即使感受眼/感受颜也不输出像素，只是相位/频率）
      encoded.visible = false;
      channels[ch.id] = encoded;
      if (ch.type === 'observe' && (ch.id === 'feel_eye' || ch.id === 'feel_face')) {
        visibleCount++; // 可降维到视觉的通道（但仍不输出像素）
      } else {
        invisibleCount++;
      }
      // 通知订阅者
      const subs = this.subscribers.get(ch.id);
      if (subs) for (const cb of subs) { try { cb(encoded); } catch (_) {} }
    }
    const stream = {
      perceptionId,
      sovereign: this.sovereign,
      state: typeof state === 'object' ? JSON.parse(JSON.stringify(state)) : state,
      channels,
      stats: {
        totalChannels: Object.keys(channels).length,
        visibleChannels: visibleCount,     // 可降维到视觉的通道数（≤2）
        invisibleChannels: invisibleCount,  // 不可见通道数（≥14）
        invisibleRatio: invisibleCount / Object.keys(channels).length,
        isHighStage: invisibleCount >= 14, // 高级阶段判定：不可见通道占主体
      },
      at: Date.now(),
      /** 显式声明：不输出像素，不渲染HTML */
      rendersPixels: false,
      rendersHTML: false,
      perceptionMode: 'invisible_stream',
    };
    this.streamHistory.push(stream);
    if (this.streamHistory.length > this.maxHistory) this.streamHistory.shift();
    return stream;
  }

  /** 订阅某个通道的感知编码（cb 会在每次 sense 时被调用） */
  subscribe(channelId, cb) {
    if (!this.subscribers.has(channelId)) this.subscribers.set(channelId, []);
    const id = `${channelId}_${Date.now()}_${Math.random().toString(36).slice(2,6)}`;
    this.subscribers.get(channelId).push(cb);
    return { id, channelId, unsubscribe: () => {
      const arr = this.subscribers.get(channelId);
      if (arr) this.subscribers.set(channelId, arr.filter(f => f !== cb));
    }};
  }

  /** 取最近 N 条感知流 */
  recent(n = 10) { return this.streamHistory.slice(-n); }

  /** 感知器统计 */
  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      channelCount: this.channels.length,
      visibleChannelCount: this.visibleChannelCount,
      invisibleChannelCount: this.channels.length - this.visibleChannelCount,
      invisibleRatio: (this.channels.length - this.visibleChannelCount) / this.channels.length,
      isHighStage: (this.channels.length - this.visibleChannelCount) >= 14,
      rendersPixels: false,
      rendersHTML: false,
      perceptionMode: 'invisible_stream',
      streamHistoryLen: this.streamHistory.length,
      subscriberCount: [...this.subscribers.values()].reduce((s, arr) => s + arr.length, 0),
      channelIds: this.channels.map(c => c.id),
    };
  }

  /**
   * 「可见性审判」—— 判定一个感知流是否过度依赖视觉通道
   * 返回 { isOverVisual, visualRatio, verdict }
   *   · visualRatio = 可降维视觉通道编码强度占比
   *   · isOverVisual = true 意味着"太可见了"，应该降级到不可见
   */
  judgeVisibility(perceptionStream) {
    const chs = perceptionStream.channels;
    const visualChannelIds = ['feel_eye', 'feel_face'];
    let visualWeight = 0;
    let totalWeight = 0;
    for (const [id, enc] of Object.entries(chs)) {
      const w = Object.keys(enc.encode || {}).length;
      totalWeight += w;
      if (visualChannelIds.includes(id)) visualWeight += w;
    }
    const visualRatio = totalWeight > 0 ? visualWeight / totalWeight : 0;
    return {
      visualRatio,
      isOverVisual: visualRatio > 0.15, // 超过15%视觉占比 = 太可见了
      verdict: visualRatio > 0.15
        ? '过度可见 · 应降级到不可见感知（高级阶段）'
        : '不可见为主 · 符合意识互联网高级阶段',
      invisibleRatio: 1 - visualRatio,
    };
  }
}

module.exports = {
  CHANNEL_ENCODERS,
  NanjianInvisibleSensor,
};
