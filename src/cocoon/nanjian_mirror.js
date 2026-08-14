/**
 * ============================================================
 * 囡囝镜像姿态 · NANJIAN-MIRROR (7 种二元镜像)
 * ============================================================
 *
 *  理论来源（用户原话）：
 *    「它是有一个负面镜像和一个正面镜像. 负面镜像和正面镜像才是二元对称的
 *     而三元裁判呢则是一个点的点积。和一个二元的脸对脸镜像和背对背镜像，
 *     手拉手进行，心连心镜像 嘴对嘴进行，眼对眼镜像，生殖器对生殖器镜像，
 *     他要讲的是内部归属微量，可确定计算数量 将来是归属权的可控性、被动性和
 *     主动性 以及外部非归属权从属性，被动从属性和主动从属性，还有他人间接
 *     主导性，他人间接从属性，她人间接，传递性，他人间接责任性 他人间接甩锅性」
 *
 *  设计要点：
 *    1. 二元对称 = 正面镜像 + 负面镜像（face_to_face 与 back_to_back 互为正负）
 *    2. 7种姿态 = 7种二元关系类型，每种对应不同的归属权/主动性组合
 *    3. 三元裁判 = 一个点的点积（dotProduct）
 *    4. 与基因编辑器联动：locator 用 mirror 姿态匹配目标函数的镜像类型
 *
 *  7 种镜像姿态（按归属权/主动性两维度划分）：
 *    ┌──────────────┬─────────────────┬──────────────────┬───────────────────┐
 *    │ 姿态          │ 归属权类型       │ 主动性类型        │ 镜像方向           │
 *    ├──────────────┼─────────────────┼──────────────────┼───────────────────┤
 *    │ face_to_face │ 内部归属（微量） │ 主动+被动对称     │ 正面正镜像         │
 *    │ back_to_back │ 内部归属（微量） │ 主动+被动对称     │ 负面负镜像         │
 *    │ hand_in_hand │ 内部归属共生     │ 主动协同          │ 横向并镜像         │
 *    │ heart_to_heart│内部归属深层     │ 被动共振          │ 纵向同位镜像       │
 *    │ mouth_to_mouth│他人间接传递     │ 主动传递          │ 纵向对位镜像       │
 *    │ eye_to_eye   │ 他人间接注意    │ 被动参考          │ 横向参考镜像       │
 *    │ root_to_root │ 他人间接根源    │ 主动+被动根源对接 │ 根源对接镜像       │
 *    └──────────────┴─────────────────┴──────────────────┴───────────────────┘
 *
 *  归属权维度（来自用户理论）：
 *    - internal_belong:      内部归属权·可控性·微量确定
 *    - external_subordinate: 外部非归属权·从属性
 *    - other_indirect:       他人间接·传递性/责任性/甩锅性
 *
 *  主动性维度（来自用户理论）：
 *    - active:               主动（发起方）
 *    - passive:              被动（接受方）
 *    - active_passive_sym:   主动被动对称（双向平等）
 *    - passive_resonance:    被动共振（同频响应）
 *    - active_transfer:      主动传递（信息流）
 *    - passive_reference:    被动参考（参照系）
 *    - root_docking:         根源对接（双向根源）
 * ============================================================
 */

'use strict';

// ============================================================
// 一、7 种镜像姿态定义
// ============================================================

const MIRROR_POSES = {
  face_to_face: {
    id: 'face_to_face',
    name: '脸对脸',
    nameEn: 'Face to Face',
    axis: 'vertical',           // 纵向轴
    ownershipType: 'internal_belong',
    activityType: 'active_passive_sym',
    mirrorPolarity: 'positive', // 正面镜像
    meaning: '正面正镜像·主动被动对称·内部归属微量确定',
    mengjiX: '踪',              // 对应萌机合X字（locator）
  },
  back_to_back: {
    id: 'back_to_back',
    name: '背对背',
    nameEn: 'Back to Back',
    axis: 'vertical',
    ownershipType: 'internal_belong',
    activityType: 'active_passive_sym',
    mirrorPolarity: 'negative', // 负面镜像
    meaning: '负面负镜像·主动被动对称·内部归属微量确定',
    mengjiX: '证',              // 对应 cutter
  },
  hand_in_hand: {
    id: 'hand_in_hand',
    name: '手拉手',
    nameEn: 'Hand in Hand',
    axis: 'horizontal',         // 横向轴
    ownershipType: 'internal_belong',
    activityType: 'active',     // 主动协同
    mirrorPolarity: 'positive',
    meaning: '横向并镜像·主动协同·内部归属共生',
    mengjiX: '治',              // 对应 replacer
  },
  heart_to_heart: {
    id: 'heart_to_heart',
    name: '心连心',
    nameEn: 'Heart to Heart',
    axis: 'vertical',
    ownershipType: 'internal_belong',
    activityType: 'passive_resonance',
    mirrorPolarity: 'positive',
    meaning: '纵向同位镜像·被动共振·内部归属深层',
    mengjiX: '祝',              // 对应 repairer
  },
  mouth_to_mouth: {
    id: 'mouth_to_mouth',
    name: '嘴对嘴',
    nameEn: 'Mouth to Mouth',
    axis: 'vertical',
    ownershipType: 'other_indirect',
    activityType: 'active_transfer',
    mirrorPolarity: 'positive',
    meaning: '纵向对位镜像·主动传递·他人间接传递性',
    mengjiX: '准',              // 对应 judge
  },
  eye_to_eye: {
    id: 'eye_to_eye',
    name: '眼对眼',
    nameEn: 'Eye to Eye',
    axis: 'horizontal',
    ownershipType: 'other_indirect',
    activityType: 'passive_reference',
    mirrorPolarity: 'positive',
    meaning: '横向参考镜像·被动参考·他人间接注意（点参考系）',
    mengjiX: '踪',              // 注意力参考，回到 locator
  },
  root_to_root: {
    id: 'root_to_root',
    name: '生殖器对生殖器',
    nameEn: 'Root to Root',
    axis: 'vertical',
    ownershipType: 'other_indirect',
    activityType: 'root_docking',
    mirrorPolarity: 'positive',
    meaning: '根源对接镜像·主动被动根源对接·他人间接根源（繁殖/传承）',
    mengjiX: '准',              // 根源对接需要审判确认
  },
};

const POSE_IDS = Object.keys(MIRROR_POSES);

// 归属权类型枚举
const OWNERSHIP_TYPES = {
  internal_belong: '内部归属权·可控性·微量确定',
  external_subordinate: '外部非归属权·从属性·被动从属/主动从属',
  other_indirect: '他人间接·传递性/责任性/甩锅性',
};

// 主动性类型枚举
const ACTIVITY_TYPES = {
  active: '主动（发起方）',
  passive: '被动（接受方）',
  active_passive_sym: '主动被动对称（双向平等）',
  passive_resonance: '被动共振（同频响应）',
  active_transfer: '主动传递（信息流）',
  passive_reference: '被动参考（参照系）',
  root_docking: '根源对接（双向根源）',
};

// ============================================================
// 二、NanjianMirror 类：单个镜像姿态实例
// ============================================================

/**
 * 囡囝镜像姿态实例
 *
 * 一个 NanjianMirror 描述"两个对象之间的某种二元镜像关系"。
 * 核心能力：
 *   1. reflect(obj)         —— 对对象做镜像反射（按姿态规则变换）
 *   2. dotProduct(other)    —— 三元裁判：一个点的点积（公约数参考系）
 *   3. symmetricWith(other) —— 二元对称判定（正面+负面镜像互补）
 *   4. stretch(direction)   —— 二元伸缩（与基因编辑器联动）
 *   5. ownershipType()      —— 归属权类型查询
 *   6. activityType()       —— 主动性类型查询
 */
class NanjianMirror {
  constructor(poseId = 'face_to_face') {
    if (!MIRROR_POSES[poseId]) {
      throw new Error(`未知镜像姿态: ${poseId}，可选: ${POSE_IDS.join(', ')}`);
    }
    this.poseId = poseId;
    this.pose = MIRROR_POSES[poseId];
    this.sovereign = 'liuchutian_hunyuan_origin';
    this.stretchState = 'balanced'; // 二元伸缩状态: up/down/balanced
    this.dotPoint = null;           // 点参考系（公约数锚点）
  }

  /** 归属权类型 */
  ownershipType() {
    return {
      type: this.pose.ownershipType,
      desc: OWNERSHIP_TYPES[this.pose.ownershipType],
      isInternal: this.pose.ownershipType === 'internal_belong',
      isExternal: this.pose.ownershipType === 'external_subordinate',
      isOtherIndirect: this.pose.ownershipType === 'other_indirect',
    };
  }

  /** 主动性类型 */
  activityType() {
    return {
      type: this.pose.activityType,
      desc: ACTIVITY_TYPES[this.pose.activityType],
      isActive: this.pose.activityType === 'active' || this.pose.activityType === 'active_transfer',
      isPassive: this.pose.activityType === 'passive' || this.pose.activityType === 'passive_reference',
      isSymmetric: this.pose.activityType === 'active_passive_sym',
    };
  }

  /**
   * 镜像反射：对对象做镜像变换
   *   - face_to_face: 正面镜像（属性取反+保留id）
   *   - back_to_back: 负面镜像（属性取反+保留id，但极性相反）
   *   - hand_in_hand: 横向并镜像（对象成对，不取反）
   *   - heart_to_heart: 纵向同位镜像（深拷贝+共振标记）
   *   - mouth_to_mouth: 主动传递（信息流从A到B）
   *   - eye_to_eye: 被动参考（B参照A，单向复制）
   *   - root_to_root: 根源对接（双向根源交换）
   */
  reflect(obj) {
    if (obj === null || obj === undefined) return obj;
    const polarity = this.pose.mirrorPolarity;
    switch (this.poseId) {
      case 'face_to_face': {
        // 正面正镜像：数值取反，字符串反转，对象递归镜像
        return reflectWithPolarity(obj, polarity);
      }
      case 'back_to_back': {
        // 负面负镜像：与face_to_face相反极性，但同向
        return reflectWithPolarity(obj, polarity);
      }
      case 'hand_in_hand': {
        // 横向并镜像：成对返回，不取反
        return { original: clone(obj), mirror: clone(obj), paired: true };
      }
      case 'heart_to_heart': {
        // 纵向同位镜像：深拷贝+共振标记
        const reflected = clone(obj);
        if (typeof reflected === 'object' && !Array.isArray(reflected)) {
          reflected._resonance = true;
        }
        return reflected;
      }
      case 'mouth_to_mouth': {
        // 主动传递：信息流标记
        return { from: clone(obj), to: clone(obj), transferred: true };
      }
      case 'eye_to_eye': {
        // 被动参考：单向复制（参照系）
        return { reference: clone(obj), copy: clone(obj), referenced: true };
      }
      case 'root_to_root': {
        // 根源对接：双向根源交换
        return { rootA: clone(obj), rootB: clone(obj), docked: true };
      }
      default:
        return clone(obj);
    }
  }

  /**
   * 三元裁判：一个点的点积
   * 点参考系 = 公约数（gcd），是微量确定的锚点
   * 点积 = 两个对象在点参考系上的投影之积
   */
  dotProduct(other) {
    // 计算两个对象的"点"（数值属性的平均值作为点参考系）
    const pointA = this._extractPoint(this._lastReflected || {});
    const pointB = this._extractPoint(other || {});
    // 公约数作为点参考系（用户理论：点参考系=公约）
    const gcd = computeGcd(Math.round(pointA), Math.round(pointB));
    this.dotPoint = gcd;
    return {
      pointA,
      pointB,
      gcd,                        // 公约数=点参考系
      dot: pointA * pointB,       // 点积
      pose: this.poseId,
      sovereign: this.sovereign,
    };
  }

  /**
   * 二元对称判定：正面镜像 + 负面镜像互补
   * face_to_face 与 back_to_back 互为二元对称
   */
  symmetricWith(otherMirror) {
    if (!(otherMirror instanceof NanjianMirror)) return { symmetric: false, reason: 'not_a_mirror' };
    // face_to_face ↔ back_to_back 是经典的二元对称对
    const complementPairs = [
      ['face_to_face', 'back_to_back'],
      ['hand_in_hand', 'heart_to_heart'],
      ['mouth_to_mouth', 'eye_to_eye'],
    ];
    for (const [a, b] of complementPairs) {
      if ((this.poseId === a && otherMirror.poseId === b)
       || (this.poseId === b && otherMirror.poseId === a)) {
        return {
          symmetric: true,
          pair: [a, b],
          polarity: 'positive+negative',
          meaning: '正面镜像+负面镜像=二元对称',
        };
      }
    }
    // root_to_root 自身对称（根源对接是自对称的）
    if (this.poseId === 'root_to_root' && otherMirror.poseId === 'root_to_root') {
      return { symmetric: true, pair: ['root_to_root','root_to_root'], polarity: 'self', meaning: '根源对接自对称' };
    }
    return { symmetric: false, pair: [this.poseId, otherMirror.poseId], polarity: 'none' };
  }

  /**
   * 二元伸缩：缩/升动态调控（与基因编辑器联动）
   * 用户理论：二元伸缩=可统计上的公约，点参考系作为公约，可随意拉伸二元的伸缩
   */
  stretch(direction) {
    if (!['up', 'down', 'balance'].includes(direction)) {
      throw new Error(`未知伸缩方向: ${direction}，可选: up/down/balance`);
    }
    this.stretchState = direction === 'up' ? 'stretched' : (direction === 'down' ? 'compressed' : 'balanced');
    return {
      direction,
      state: this.stretchState,
      pose: this.poseId,
      dotPoint: this.dotPoint,  // 伸缩以点参考系为锚
      meaning: direction === 'up' ? '升（拉伸公约）'
             : direction === 'down' ? '缩（压缩公约）'
             : '平衡（公约锚定）',
    };
  }

  /** 内部：从对象提取"点"（数值属性的平均值作为点参考系） */
  _extractPoint(obj) {
    if (typeof obj === 'number') return obj;
    if (typeof obj === 'string') return obj.length;
    if (Array.isArray(obj)) {
      if (obj.length === 0) return 0;
      return obj.reduce((s, v) => s + this._extractPoint(v), 0) / obj.length;
    }
    if (obj && typeof obj === 'object') {
      const vals = Object.values(obj).filter(v => typeof v === 'number');
      if (vals.length === 0) return 0;
      return vals.reduce((s, v) => s + v, 0) / vals.length;
    }
    return 0;
  }

  /** 描述信息 */
  describe() {
    return {
      poseId: this.poseId,
      name: this.pose.name,
      nameEn: this.pose.nameEn,
      axis: this.pose.axis,
      polarity: this.pose.mirrorPolarity,
      ownership: this.ownershipType(),
      activity: this.activityType(),
      meaning: this.pose.meaning,
      mengjiX: this.pose.mengjiX,
      stretchState: this.stretchState,
      dotPoint: this.dotPoint,
    };
  }
}

// ============================================================
// 三、镜像姿态注册表：管理7种姿态 + 与基因编辑器联动
// ============================================================

class NanjianMirrorRegistry {
  constructor() {
    this.version = '0.1.0-nanjian-mirror';
    this.sovereign = 'liuchutian_hunyuan_origin';
    this.poses = Object.assign({}, MIRROR_POSES);
    this.instances = new Map(); // poseId → NanjianMirror 实例缓存
  }

  /** 列出所有姿态id */
  listPoses() {
    return POSE_IDS.slice();
  }

  /** 获取姿态定义 */
  getPose(poseId) {
    return this.poses[poseId] ? Object.assign({}, this.poses[poseId]) : null;
  }

  /** 创建/获取镜像实例 */
  create(poseId) {
    if (!this.instances.has(poseId)) {
      this.instances.set(poseId, new NanjianMirror(poseId));
    }
    return this.instances.get(poseId);
  }

  /**
   * 按萌机合X字匹配姿态（与基因编辑器联动）
   * locator=踪 → face_to_face / eye_to_eye
   * cutter=证 → back_to_back
   * replacer=治 → hand_in_hand
   * repairer=祝 → heart_to_heart
   * judge=准 → mouth_to_mouth / root_to_root
   */
  matchByMengjiX(mengjiX) {
    const matched = [];
    for (const pose of Object.values(this.poses)) {
      if (pose.mengjiX === mengjiX) matched.push(pose.id);
    }
    return matched;
  }

  /**
   * 按归属权类型筛选姿态
   * @param {'internal_belong'|'external_subordinate'|'other_indirect'} ownershipType
   */
  filterByOwnership(ownershipType) {
    return Object.values(this.poses)
      .filter(p => p.ownershipType === ownershipType)
      .map(p => p.id);
  }

  /**
   * 按主动性类型筛选姿态
   */
  filterByActivity(activityType) {
    return Object.values(this.poses)
      .filter(p => p.activityType === activityType)
      .map(p => p.id);
  }

  /**
   * 二元对称配对查询
   * 返回所有互为二元对称的姿态对
   */
  symmetricPairs() {
    return [
      { pair: ['face_to_face', 'back_to_back'], polarity: 'positive+negative', meaning: '正面镜像+负面镜像=二元对称' },
      { pair: ['hand_in_hand', 'heart_to_heart'], polarity: 'horizontal+vertical', meaning: '横向协同+纵向共振=二元对称' },
      { pair: ['mouth_to_mouth', 'eye_to_eye'], polarity: 'transfer+reference', meaning: '主动传递+被动参考=二元对称' },
      { pair: ['root_to_root', 'root_to_root'], polarity: 'self', meaning: '根源对接自对称' },
    ];
  }

  /** 统计 */
  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      poseCount: POSE_IDS.length,
      poseIds: POSE_IDS.slice(),
      ownershipTypes: Object.keys(OWNERSHIP_TYPES),
      activityTypes: Object.keys(ACTIVITY_TYPES),
      symmetricPairCount: this.symmetricPairs().length,
    };
  }
}

// ============================================================
// 四、辅助函数
// ============================================================

function clone(obj) {
  if (obj === null || typeof obj !== 'object') return obj;
  if (Array.isArray(obj)) return obj.map(clone);
  const out = {};
  for (const [k, v] of Object.entries(obj)) out[k] = clone(v);
  return out;
}

/** 按极性做镜像反射（正面=取反，负面=取反但标记负极性） */
function reflectWithPolarity(obj, polarity) {
  if (obj === null || obj === undefined) return obj;
  if (typeof obj === 'number') return polarity === 'positive' ? -obj : -obj;
  if (typeof obj === 'string') return obj.split('').reverse().join('');
  if (Array.isArray(obj)) return obj.map(v => reflectWithPolarity(v, polarity));
  if (typeof obj === 'object') {
    const out = {};
    for (const [k, v] of Object.entries(obj)) {
      out[k] = reflectWithPolarity(v, polarity);
    }
    out._polarity = polarity;
    return out;
  }
  return obj;
}

/** 计算公约数（点参考系=公约） */
function computeGcd(a, b) {
  a = Math.abs(Math.round(a));
  b = Math.abs(Math.round(b));
  if (a === 0 && b === 0) return 0;
  if (a === 0) return b;
  if (b === 0) return a;
  while (b !== 0) {
    [a, b] = [b, a % b];
  }
  return a;
}

// ============================================================
// 五、导出
// ============================================================

module.exports = {
  MIRROR_POSES,
  POSE_IDS,
  OWNERSHIP_TYPES,
  ACTIVITY_TYPES,
  NanjianMirror,
  NanjianMirrorRegistry,
};
