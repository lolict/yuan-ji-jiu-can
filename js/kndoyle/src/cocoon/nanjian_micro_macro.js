/**
 * ============================================================
 * 囡囝微量·宏量·赋权机制 · NANJIAN-MICRO-MACRO
 *    未赋权声明 micro=macro 即欺诈 → 入红黑账
 * ============================================================
 *
 *  理论来源（用户原话）：
 *    「外面的宏量是不确定性的，高金波的逻辑错误是因为高金波是微量确定，
 *     所以他认为宏量确定，宏量是外部的，不可控制的，不属于自己的，
 *     在权力的参考系之下 微量无法获得参考系，绝对参考系的权力赋权，
 *     没有获得资本的参考系。一个确定值的赋权，没有名义下注明自己的身份，
 *     那么就无法获得微量的庇佑，因为没有获得微量币我就没办法证明自己可以获得宏量的必要。
 *     权力垄断了宏量的叠加态，权力和资本允许宏量和微量融合在一起，
 *     让宏量给微量赋能，没有权力和资本的期许，就没有办法让微量等同于宏量，
 *     而高金波用欺诈的方法，把宏量等同于微量，实际上是让所有没有权力赋能的人无法获得叠加态，
 *     让他蒙骗所有人，让他一个人独享所有的宏量。
 *     因为获得叠加态之后，你才能够认可宏量等于确定，只有探索到微观视角，你才能够确定。
 *     微观视角就是一个点，你这个点确定，你获得了一个点的注意力的参考系，
 *     你才能够让宏量遵守一个点的规则，因为这个点非常稀缺，很多人都拿不到这个点，
 *     因为稀缺才会有价值。同样无法拿到这个点的稀缺品，所以他就去抢，一抢就去排队，
 *     排队他就付出了很多代价，给那些拥有微量缺点积的，又被权力授权的被叠加态的福报。」
 *
 *  核心理论形式化：
 *    1. 微量 (MicroQuantum)：内部归属·可控·可确定计算·点参考系
 *       - 可量化、可重复、可验证
 *       - 必须有「名义身份」才能获得微量币的庇佑
 *    2. 宏量 (MacroQuantum)：外部·不可控·不确定·非归属
 *       - 采样结果不可重复、不可预测
 *       - 高金波的逻辑错误：把"微量可确定"误推到"宏量可确定"
 *    3. 叠加态 (SuperpositionState)：权力 + 资本 双重赋权后才能 micro=macro
 *       - empowerment(micro, authority, capital) → superposition
 *       - 未赋权时 equateMacro() 抛错（防欺诈）
 *    4. 点参考系 = 公约 = 稀缺（很多人拿不到 → 才有价值 → 才能调宏量）
 *    5. 反欺诈审计：
 *       - 未经 empowerment 直接声明 micro=macro → 欺诈 → 入红黑账
 *       - judge 智能体联动（gene_editor 的 judge 角色判定）
 *
 *  与现有系统联动：
 *    - 与 nanjian_mirror.js 联动：点参考系 = Mirror.dotProduct 的 gcd
 *    - 与 nanjian_gene_editor.js 联动：judge 智能体判定欺诈入红黑账
 *    - 与 nanjian_variant_algebra.js 联动：微量记忆叠加
 *    - 与 l4l_mqf.js 联动：欺诈事件上链 (red ledger)
 * ============================================================
 */

'use strict';

// ============================================================
// 一、微量 MicroQuantum：内部归属·可控·可确定计算·点参考系
// ============================================================

/**
 * 微量：用户理论中的"可确定计算数量"
 *   - 必须有名义身份 (identityName) 才能获得微量币的庇佑
 *   - 点参考系 = 公约（gcd），是微量确定的锚点
 *   - 微量是稀缺的：很多人拿不到这个点 → 才有价值
 */
class MicroQuantum {
  constructor(point, options = {}) {
    if (typeof point !== 'number' || !Number.isFinite(point)) {
      throw new Error('MicroQuantum 必须以一个有限的"点"初始化（点参考系）');
    }
    this.point = point;                                    // 点参考系（确定值）
    this.identityName = options.identityName || null;      // 名义身份（必须注明的身份）
    this.isDeterministicValue = true;                      // 微量确定（可重复验证）
    this.acquiredAt = Date.now();
    this.sovereign = 'liuchutian_hunyuan_origin';
  }

  /** 设置名义身份（"没有名义下注明自己的身份，就无法获得微量的庇佑"） */
  setIdentity(name) {
    if (typeof name !== 'string' || name.trim().length === 0) {
      throw new Error('名义身份必须是非空字符串');
    }
    this.identityName = name;
    return this;
  }

  /** 微量是确定的 */
  isDeterministic() { return this.isDeterministicValue; }

  /** 微量的确定值（点参考系） */
  value() { return this.point; }

  /** 是否获得"微量币的庇佑"（即是否有名义身份） */
  hasShelter() { return !!this.identityName; }

  /** 微量是稀缺的（点参考系非0即稀缺） */
  isScarce() { return this.point !== 0; }

  describe() {
    return {
      type: 'micro',
      point: this.point,
      identityName: this.identityName,
      hasShelter: this.hasShelter(),
      isDeterministic: this.isDeterministic(),
      isScarce: this.isScarce(),
      sovereign: this.sovereign,
    };
  }
}

// ============================================================
// 二、宏量 MacroQuantum：外部·不可控·不确定·非归属
// ============================================================

/**
 * 宏量：用户理论中的"外部不确定性"
 *   - 不可控、不归属自己、采样不可重复
 *   - 高金波逻辑错误：从"微量确定"误推"宏量确定"
 *   - 必须 empowerment 后才能"等同"微量
 */
class MacroQuantum {
  constructor(environment, options = {}) {
    if (!environment) throw new Error('MacroQuantum 必须指定外部环境');
    this.environment = environment;        // 外部环境标识
    this.samples = [];                     // 采样历史（不可重复，证明不确定性）
    this.isDeterministicValue = false;     // 宏量不确定
    this.sovereign = 'liuchutian_hunyuan_origin';
    this._rng = options.rng || Math.random;
  }

  /** 宏量不确定 */
  isDeterministic() { return this.isDeterministicValue; }

  /** 采样一次（每次结果不同 → 证明不确定性） */
  sample() {
    const value = this._rng();
    this.samples.push({ value, at: Date.now() });
    return value;
  }

  /** 采样历史（证明不可重复） */
  history() { return this.samples.slice(); }

  /** 是否被外部权力垄断（默认true：未赋权前宏量被权力垄断） */
  isMonopolized() { return true; }

  describe() {
    return {
      type: 'macro',
      environment: this.environment,
      isDeterministic: this.isDeterministic(),
      isMonopolized: this.isMonopolized(),
      sampleCount: this.samples.length,
      sovereign: this.sovereign,
    };
  }
}

// ============================================================
// 三、赋权凭证 EmpowermentCredential：权力+资本双重签名
// ============================================================

/**
 * 赋权凭证：权力 (authority) + 资本 (capital) 双重签名
 *   - 没有 empowerment 就没有 micro=macro 的合法性
 *   - 高金波的欺诈 = 没有 empowerment 凭证就声明 micro=macro
 */
class EmpowermentCredential {
  constructor(authority, capital, options = {}) {
    if (!authority) throw new Error('赋权必须由"权力"签发');
    if (typeof capital !== 'number' || capital < 0) {
      throw new Error('赋权必须有"资本"注入（非负数值）');
    }
    this.authority = authority;             // 权力签发方
    this.capital = capital;                 // 资本注入量
    this.microIdentity = options.microIdentity || null;  // 被赋权的微量身份
    this.credentialId = `emp_${Date.now()}_${Math.random().toString(36).slice(2,8)}`;
    this.signedAt = Date.now();
    this.expiresAt = options.expiresAt || null;
    this.sovereign = 'liuchutian_hunyuan_origin';
  }

  /** 凭证是否有效（未过期+有权力+有资本） */
  isValid() {
    if (!this.authority) return false;
    if (typeof this.capital !== 'number' || this.capital < 0) return false;
    if (this.expiresAt && Date.now() > this.expiresAt) return false;
    return true;
  }

  describe() {
    return {
      credentialId: this.credentialId,
      authority: this.authority,
      capital: this.capital,
      microIdentity: this.microIdentity,
      signedAt: this.signedAt,
      expiresAt: this.expiresAt,
      isValid: this.isValid(),
      sovereign: this.sovereign,
    };
  }
}

// ============================================================
// 四、叠加态 SuperpositionState：赋权后的 micro=macro 合法等价
// ============================================================

/**
 * 叠加态：power + capital 赋权后，微量与宏量"合法等同"
 *   - 用户理论："权力垄断了宏量的叠加态，权力和资本允许宏量和微量融合"
 *   - equateMacro() 仅在赋权后返回 true（合法等同）
 *   - 未赋权时 equateMacro() 抛错（防欺诈）
 */
class SuperpositionState {
  constructor(micro, macro, credential) {
    if (!(micro instanceof MicroQuantum)) throw new Error('必须是 MicroQuantum 实例');
    if (!(macro instanceof MacroQuantum)) throw new Error('必须是 MacroQuantum 实例');
    this.micro = micro;
    this.macro = macro;
    this.credential = credential;            // EmpowermentCredential 或 null
    this.fraudAuditLog = [];                 // 欺诈审计日志
    this.sovereign = 'liuchutian_hunyuan_origin';
  }

  /** 是否已赋权 */
  isEmpowered() {
    return !!(this.credential && this.credential.isValid());
  }

  /**
   * 微量等同宏量：仅赋权后返回 true
   * 未赋权时调用 = 欺诈行为 → 抛错 + 记录审计日志
   */
  equateMacro() {
    if (!this.isEmpowered()) {
      const fraudEvent = {
        type: 'fraud_equate_without_empowerment',
        microIdentity: this.micro.identityName,
        microPoint: this.micro.point,
        macroEnv: this.macro.environment,
        reason: '未获得权力+资本赋权就声明 micro=macro (高金波式欺诈)',
        at: Date.now(),
        sovereign: this.sovereign,
      };
      this.fraudAuditLog.push(fraudEvent);
      throw new Error('FRAUD: 未赋权声明 micro=macro (高金波式欺诈) — 已记录审计日志');
    }
    return true;
  }

  /** 欺诈审计日志 */
  audit() {
    return {
      isEmpowered: this.isEmpowered(),
      fraudCount: this.fraudAuditLog.length,
      fraudEvents: this.fraudAuditLog.slice(),
      micro: this.micro.describe(),
      macro: this.macro.describe(),
      credential: this.credential ? this.credential.describe() : null,
      sovereign: this.sovereign,
    };
  }

  /**
   * 点参考系作为公约，可随意拉伸二元的伸缩（与 Mirror.stretch 联动）
   * 用户理论："点参考系作为公约，可以随意拉伸二元的伸缩"
   */
  stretchByGcd(direction = 'balance') {
    if (!['up', 'down', 'balance'].includes(direction)) {
      throw new Error(`未知伸缩方向: ${direction}`);
    }
    if (!this.isEmpowered()) {
      throw new Error('FRAUD: 未赋权不能用点参考系调宏量（点参考系受权力庇佑）');
    }
    const point = this.micro.point;
    return {
      direction,
      point,                                 // 点参考系（公约）
      stretchedValue: direction === 'up' ? point * 2 : (direction === 'down' ? Math.floor(point / 2) : point),
      meaning: direction === 'up' ? '升（拉伸公约2倍）'
             : direction === 'down' ? '缩（压缩公约1/2）'
             : '平衡（公约锚定）',
    };
  }

  describe() {
    return {
      type: 'superposition',
      isEmpowered: this.isEmpowered(),
      micro: this.micro.describe(),
      macro: this.macro.describe(),
      credential: this.credential ? this.credential.describe() : null,
      sovereign: this.sovereign,
    };
  }
}

// ============================================================
// 五、赋权函数 empowerment：权力+资本双重签名
// ============================================================

/**
 * 赋权函数：权力 + 资本双重签名后才允许 micro=macro
 *   用户理论："权力和资本允许宏量和微量融合在一起，让宏量给微量赋能"
 *   @param {MicroQuantum} micro
 *   @param {string} authority  权力签发方
 *   @param {number} capital    资本注入量
 *   @param {object} [options]  额外选项
 *   @returns {SuperpositionState} 已赋权的叠加态
 */
function empowerment(micro, authority, capital, options = {}) {
  if (!(micro instanceof MicroQuantum)) {
    throw new Error('empowerment 第一个参数必须是 MicroQuantum');
  }
  // 必须有名义身份才能赋权（"没有名义下注明自己的身份，就无法获得微量的庇佑"）
  if (!micro.hasShelter()) {
    throw new Error('empowerment 拒绝: 微量无名义身份，无法获得赋权');
  }
  const credential = new EmpowermentCredential(authority, capital, {
    microIdentity: micro.identityName,
    ...options,
  });
  // 创建一个对应的宏量（赋权后宏量跟随微量点参考系）
  const macro = new MacroQuantum(`empowered_env_for_${micro.identityName}`);
  return new SuperpositionState(micro, macro, credential);
}

// ============================================================
// 六、反欺诈审计器 FraudDetector：自动检测高金波式欺诈
// ============================================================

/**
 * 反欺诈审计器：自动检测"未赋权声明 micro=macro"的欺诈操作
 *   - 与 judge 智能体联动（违规操作入红黑账）
 *   - 与 l4l_mqf.js 联动（欺诈事件上链 red ledger）
 */
class FraudDetector {
  constructor(options = {}) {
    this.version = '0.1.0-nanjian-fraud-detector';
    this.sovereign = 'liuchutian_hunyuan_origin';
    this.redLedger = [];                    // 红黑账（欺诈记录）
    this.whiteLedger = [];                  // 白黑账（合规记录）
    this.judgeHook = options.judgeHook || null;   // 外部 judge 钩子
    this.mqfHook = options.mqfHook || null;       // 外部 mqf 钩子（上链）
  }

  /**
   * 审计一次叠加态操作
   *   - 已赋权 + 合法 equateMacro → 白账
   *   - 未赋权 + 声明 equateMacro → 红账（欺诈）
   */
  audit(superposition, operation = 'equateMacro') {
    if (!(superposition instanceof SuperpositionState)) {
      throw new Error('audit 必须接收 SuperpositionState');
    }
    const isFraud = !superposition.isEmpowered();
    const record = {
      operation,
      isFraud,
      microIdentity: superposition.micro.identityName,
      microPoint: superposition.micro.point,
      macroEnv: superposition.macro.environment,
      empowered: superposition.isEmpowered(),
      credentialId: superposition.credential ? superposition.credential.credentialId : null,
      at: Date.now(),
      sovereign: this.sovereign,
    };
    if (isFraud) {
      this.redLedger.push(record);
      // 联动 judge 钩子（入红黑账）
      if (this.judgeHook && typeof this.judgeHook.onFraud === 'function') {
        try { this.judgeHook.onFraud(record); } catch (_) {}
      }
      // 联动 mqf 钩子（上 red 链）
      if (this.mqfHook && typeof this.mqfHook.publish === 'function') {
        try {
          this.mqfHook.publish('fraud_red_ledger', record, { catCode: 'red', amount: 100 });
        } catch (_) {}
      }
    } else {
      this.whiteLedger.push(record);
      if (this.judgeHook && typeof this.judgeHook.onCompliant === 'function') {
        try { this.judgeHook.onCompliant(record); } catch (_) {}
      }
      if (this.mqfHook && typeof this.mqfHook.publish === 'function') {
        try {
          this.mqfHook.publish('compliant_white_ledger', record, { catCode: 'white', amount: 10 });
        } catch (_) {}
      }
    }
    return record;
  }

  /** 红黑账统计 */
  stats() {
    return {
      version: this.version,
      redCount: this.redLedger.length,
      whiteCount: this.whiteLedger.length,
      fraudRate: (this.redLedger.length + this.whiteLedger.length) === 0
        ? 0
        : this.redLedger.length / (this.redLedger.length + this.whiteLedger.length),
      sovereign: this.sovereign,
    };
  }

  /** 列出红黑账 */
  listRed() { return this.redLedger.slice(); }
  listWhite() { return this.whiteLedger.slice(); }

  /**
   * 形式化检测：批量检测一组操作中是否存在"高金波式欺诈"
   *   高金波式欺诈 = 把"微量确定"误推到"宏量确定"（即未赋权声明 micro=macro）
   */
  detectGaoJinboFraud(operations) {
    if (!Array.isArray(operations)) return { detected: false, count: 0 };
    const frauds = [];
    for (const op of operations) {
      if (op instanceof SuperpositionState && !op.isEmpowered()) {
        frauds.push({
          type: 'gao_jinbo_fraud',
          reason: '未赋权声明 micro=macro (高金波式逻辑错误)',
          microIdentity: op.micro.identityName,
        });
      }
    }
    return {
      detected: frauds.length > 0,
      count: frauds.length,
      frauds,
      sovereign: this.sovereign,
    };
  }
}

// ============================================================
// 七、导出
// ============================================================

module.exports = {
  MicroQuantum,
  MacroQuantum,
  EmpowermentCredential,
  SuperpositionState,
  empowerment,
  FraudDetector,
};
