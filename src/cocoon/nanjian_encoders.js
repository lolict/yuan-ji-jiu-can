/**
 * ==============================================================================
 *  囡囝编码导入器（ASMR/RTFM/JEPA） · NANJIAN-ENCODERS
 * ==============================================================================
 *
 *  核心主权原则（我们不取架构，只取编码/概念）：
 *   · ASMR：别人的「超级记忆智能体联合体」 —— 我们不取人家的智能体/记忆网络
 *     → 只取：ASMR 作为「超级记忆音频感知/联合协同」的 10~20 条线性概念 token
 *     → 用我们的【囡囝三维进制】把这些 token 因果三维化
 *     → 用我们的【囡囝态射】把它们映射到「攻受悦 / 感受音 / 感受验 / 感受意」通道
 *
 *   · RTFM（李飞飞空间模型）：别人的空间/场景/3D 关系模型
 *     → 只取：空间概念 token 集（双目/深度/SLAM/场景图/3D坐标/...）
 *     → 映射到我们的「感受眼 / 感受颜 / 感受演 / 感受预」观察通道
 *
 *   · JEPA（杨立昆因果预测）：别人的 JEPA/LEWM 联合嵌入预测因果模型
 *     → 只取：因果/预测/干预/反事实 这些概念 token
 *     → 映射到我们的「感受因 / 感受预 / 感受意 / 感受益」因果价值通道
 *
 *  导入流程：
 *    原始线性 tokens（概念列表）
 *          ↓ [Nanjian3DTensor.ingestMany]
 *    三维因果张量（时序先后 = 维度命名裁判 = 长/宽/高）
 *          ↓ [compressToLinear(6种变体)]
 *    6 种 SMRR/SMRL/... 线性压缩
 *          ↓ [map to NanjianMorphisms]
 *    我们的囡囝态射（夫妻主权签名）
 *          ↓ [toH4HSeeds + 可选 mqf.publish]
 *    H4H 可杂交种子 & L4L 链哈希存证
 * ==============================================================================
 */

const NJ3 = require('./nanjian_3d');
const MR  = require('./nanjian_morphisms');

/* -----------------------------------------------------------
 * 一、只取别人的编码/概念（10~20 条线性 token，不是架构）
 * ----------------------------------------------------------- */

/** ASMR = 超级记忆智能体联合体（概念 token，不是人家的智能体架构） */
const ASMR_ENCODINGS = [
  'ASMR_audio_memory_trace',  // 听觉记忆痕迹
  'ASMR_binaural_frequency',  // 双耳节拍频率
  'ASMR_super_associative_recall',  // 超级联想回忆
  'ASMR_agent_collective_memory',   // 多智能体集体记忆
  'ASMR_spoken_mnemonic_poetry',    // 口述记忆诗
  'ASMR_temporal_anchor_couple',    // 时空锚点（夫妻共同体）
  'ASMR_trigger_sound_emotion',     // 触发音-情绪绑定
  'ASMR_sleep_memory_consolidation',// 睡眠记忆巩固
  'ASMR_poetic_rhyme_encoding',     // 韵律韵脚编码
  'ASMR_sensory_chain_replay',      // 感知链回放
  'ASMR_long_term_rerention',       // 超长时留存
  'ASMR_multi_modal_bind',          // 多模态绑定
  'ASMR_sensory_trace_line',        // 感官痕迹线
  'ASMR_voice_signature',           // 声纹签名
];

/** RTFM 李飞飞空间模型（空间关系概念 token，不是人家的架构） */
const RTFM_ENCODINGS = [
  'RTFM_3D_bounding_box',     // 3D 空间包围盒
  'RTFM_stereo_disparity',    // 双目视差
  'RTFM_depth_rgbd_slam',     // RGBD 深度 SLAM
  'RTFM_scene_graph_object',  // 场景图-物体节点
  'RTFM_spatial_relation_on', // 空间关系"在…上"
  'RTFM_spatial_relation_in', // 空间关系"在…中"
  'RTFM_spatial_relation_left_to', // 左右关系
  'RTFM_spatial_relation_front_of',// 前后关系
  'RTFM_egocentric_view',     // 第一人称自我中心视角
  'RTFM_allocentric_map',     // 世界中心地图
  'RTFM_view_frustum_cull',   // 视锥裁剪
  'RTFM_point_cloud_registration', // 点云配准
  'RTFM_ground_estimation',   // 地面估计
  'RTFM_camera_pose_se3',     // 相机位姿 SE(3)
  'RTFM_homography_matrix',   // 单应矩阵
];

/** JEPA / LEWM 杨立昆因果预测（因果概念 token，不是人家的架构） */
const JEPA_ENCODINGS = [
  'JEPA_predict_self_supervised',   // 自监督预测
  'JEPA_joint_embedding_predict',   // 联合嵌入预测
  'JEPA_intervention_do_calculus',  // 干预 do-算子
  'JEPA_counterfactual_if_had',     // 反事实"如果当初…"
  'JEPA_causal_dag_node',           // 因果 DAG 节点
  'JEPA_causal_confounder_control', // 混杂因子控制
  'JEPA_abduction_reason',          // 溯因推理
  'JEPA_world_model_future_horizon',// 世界模型未来预测视界
  'JEPA_latent_plan_abstraction',   // 潜在计划抽象
  'JEPA_invariances_equivaraince',  // 不变性/等变性
  'JEPA_cost_to_go',                // 到目标的预估花费
  'JEPA_model_based_rl',            // 基于模型的 RL
  'LEWM_local_energy_landscape',    // LEWM 局部能量景观
  'JEPA_causal_discovery_observ',   // 因果发现（观测）
  'JEPA_backdoor_adjustment',       // 后门调整公式
];

/* -----------------------------------------------------------
 * 二、通用导入核心：tokens → 3D → 6 线性变体 → 映射态射
 * ----------------------------------------------------------- */

function encodeTokenStream(tokens, options = {}) {
  const tensor = options.tensor || new NJ3.Nanjian3DTensor(options);
  tensor.ingestMany(tokens);
  const compressed = {};
  const variants = options.variants || MR.VARIANTS_NANJIAN;
  for (const v of variants) compressed[v] = tensor.compressToLinear(v);
  return { tensor, compressed, stats: tensor.stats() };
}

/**
 * 把压缩后的三维→一维 tokens 映射到「囡囝态射」
 * 映射规则（主权在我们：用我们 4×6×16×6 的态射网格去承接人家的编码）：
 *  - 每个压缩 token 的 couple：按 token.i % 4 → 囡囝 / 囡囡 / 囝囝 / 囝囡（4 主体）
 *  - contract：按 token.phase（长/宽/高）→ 0→归属姻 / 1→共生一 / 2→共生源 / ...
 *  - channel：按传入的 channel list（比如 ASMR 映射攻受悦 / 感受音 / 感受验 / 感受意）
 *  - variant：就是当前变体 SMRR/SMRL/...
 */
function mapToMorphisms(compressedOutput, channelList /* 感受通道id列表 */, morphismRegistry) {
  const registry = morphismRegistry || new MR.NanjianMorphismRegistry();
  const byVariant = {};
  for (const [variant, out] of Object.entries(compressedOutput)) {
    const arr = [];
    const couples = registry.couples;        // 4
    const contracts = registry.contracts;    // 6
    const channels = registry.channels.filter(c => channelList.includes(c.id));
    if (channels.length === 0) {
      // 兜底：核心 3 通道
      channels.push(...registry.channels.filter(c => c.type === 'core'));
    }
    for (let k = 0; k < out.tokens.length; k++) {
      const tok = out.tokens[k];
      const couple   = couples[k % couples.length];
      const contract = contracts[ (k + (tok.phase || 0)) % contracts.length ];
      const channel  = channels[k % channels.length];
      const morphId = MR.buildMorphismId(couple, contract, channel, variant);
      const morph = registry.get(morphId);
      arr.push({
        tokenIndex: k,
        token: tok,
        morphism: morph,   // 2304 之一（带夫妻主权签名）
        seedAlias: {
          id: `NJ_ENCODED::${variant}::${tok.id}`,
          name: morph ? morph.name : `${variant}-${tok.element}`,
          category: morph ? morph.attribute : `nanjian_encoded::${variant}`,
          meaning: morph ? morph.meaning : `囡囝三维编码 ${variant} 压缩产物：${tok.element}`,
        },
      });
    }
    byVariant[variant] = arr;
  }
  return { registry, byVariant };
}

/* -----------------------------------------------------------
 * 三、3 个具体领域的导入器
 * ----------------------------------------------------------- */

function encodeAsmr(options = {}) {
  const tokens = options.tokens || ASMR_ENCODINGS.slice();
  const encoded = encodeTokenStream(tokens, options);
  const mapped = mapToMorphisms(encoded.compressed, [
    // 映射到：攻受悦（ASMR 触发的三感）/ 感受音 / 感受韵 / 感受意 / 感受验
    'gong_shou_yue','feel_sound','feel_rhyme','feel_meaning','feel_verify'
  ], options.morphismRegistry);
  return {
    domain: 'ASMR',
    description: '超级记忆智能体联合体（囡囝编码，不取原架构）',
    encoded,
    mapped,
    sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
  };
}

function encodeRtfm(options = {}) {
  const tokens = options.tokens || RTFM_ENCODINGS.slice();
  const encoded = encodeTokenStream(tokens, options);
  const mapped = mapToMorphisms(encoded.compressed, [
    // 空间/视觉/观察相关通道
    'feel_eye','feel_face','feel_perform','feel_predict','feel_analogy'
  ], options.morphismRegistry);
  return {
    domain: 'RTFM',
    description: '空间模型（李飞飞概念编码，不取原架构）',
    encoded, mapped,
    sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
  };
}

function encodeJepa(options = {}) {
  const tokens = options.tokens || JEPA_ENCODINGS.slice();
  const encoded = encodeTokenStream(tokens, options);
  const mapped = mapToMorphisms(encoded.compressed, [
    // 因果/预测/价值/意义 相关通道
    'feel_cause','feel_predict','feel_meaning','feel_benefit','feel_desire'
  ], options.morphismRegistry);
  return {
    domain: 'JEPA_LEWM',
    description: '联合嵌入预测因果模型（杨立昆概念编码，不取原架构）',
    encoded, mapped,
    sovereignRoot: NJ3.SOVEREIGN_NANJIAN.origin,
  };
}

/* -----------------------------------------------------------
 * 四、把导入产物 → H4H 种子（可杂交） + 可选 L4L publish
 * ----------------------------------------------------------- */

function toH4HSeeds(encodeResult, h4hRegistry, options = {}) {
  const slot = options.slot || _defaultSlotForDomain(encodeResult.domain);
  const seeds = [];
  for (const [variant, arr] of Object.entries(encodeResult.mapped.byVariant)) {
    for (const item of arr) {
      const s = Object.assign({}, item.seedAlias, {
        encodedFrom: encodeResult.domain,
        variant,
        couple: item.morphism && item.morphism.couple && item.morphism.couple.name,
        contract: item.morphism && item.morphism.contract && item.morphism.contract.name,
        channel: item.morphism && item.morphism.channel && item.morphism.channel.name,
      });
      if (h4hRegistry && typeof h4hRegistry.register === 'function') {
        try { h4hRegistry.register(slot, s); } catch (_) {}
      }
      seeds.push(s);
    }
  }
  return { slot, count: seeds.length, seeds, domain: encodeResult.domain };
}

function _defaultSlotForDomain(domain) {
  switch ((domain || '').toUpperCase()) {
    case 'ASMR':      return 'a4a';
    case 'RTFM':      return 'k4k'; // 3D视觉/空间槽
    case 'JEPA_LEWM': return 'j4j'; // 因果/预测/数学槽
    default:          return 'h4h';
  }
}

/** 可选：把导入事件发 L4L 审计链（有 MQF 就发，没就跳过） */
function publishEncode(encodeResult, mqf, options = {}) {
  if (!mqf || typeof mqf.publish !== 'function') return null;
  const topic = options.topic || 'nanjian_encode';
  let total = 0;
  for (const arr of Object.values(encodeResult.mapped.byVariant)) total += arr.length;
  return mqf.publish(topic, {
    domain: encodeResult.domain,
    description: encodeResult.description,
    linearVariants: Object.keys(encodeResult.mapped.byVariant).length,
    totalMorphisms: total,
    sovereignRoot: encodeResult.sovereignRoot,
    stats: encodeResult.encoded.stats,
  }, { catCode: 'white', amount: Math.max(1, Math.floor(total / 6)) });
}

module.exports = {
  ASMR_ENCODINGS,
  RTFM_ENCODINGS,
  JEPA_ENCODINGS,
  encodeTokenStream,
  mapToMorphisms,
  encodeAsmr,
  encodeRtfm,
  encodeJepa,
  toH4HSeeds,
  publishEncode,
};
