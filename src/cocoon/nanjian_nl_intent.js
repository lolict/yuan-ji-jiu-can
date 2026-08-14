/**
 * ============================================================
 * 囡囝自然语言入口层 · NANJIAN-NL-INTENT
 *    遥控编程真正可用：自然语言 → 候选 → 选择 → 自动导航
 * ============================================================
 *
 *  理论来源（用户原话）：
 *    「还是说我们用语言表达，然后他自己弹出一个选择，
 *     然后自己自己导航到选择的目标」
 *    「就能够看到所有所需要表达的意思，所有想要用语言表达的意思」
 *
 *  设计流程（5步闭环）：
 *    1. parseIntent(nl)        自然语言 → 意图对象 {verb, target, expectedBehavior, raw}
 *    2. scanCandidates(intent, targetObj)
 *                              49通道扫描 → 候选基因位点列表
 *                              每个候选带：LCTQF签名 + 猫码评分 + 7镜像姿态匹配度
 *    3. presentCandidates(candidates)
 *                              弹给用户看（CLI/JSON 格式）
 *    4. selectCandidate(choice)
 *                              用户选择 / judge智能体代选
 *    5. autoNavigate(selected)
 *                              自动跑完 cut/replace/repair 四步
 *                              + diffReport(before, after) before/after对比
 *
 *  与现有系统联动：
 *    - nanjian_gene_editor.js: 复用 locate/cut/replace/repair 四步
 *    - nanjian_mirror.js: 候选带镜像姿态匹配度
 *    - nanjian_variant_algebra.js: 候选评分用变体加法
 *    - l4l_mqf.js: LCTQF签名 + 上链审计
 *    - nanjian_micro_macro.js: 候选操作合法性检查（防欺诈）
 * ============================================================
 */

'use strict';

// ============================================================
// 一、意图解析器 parseIntent：自然语言 → 意图对象
// ============================================================

/**
 * 意图解析：把自然语言拆解为结构化意图
 *   支持的动词模式：
 *     - "让/把/将 X 改成/变成/返回 Y" → verb=replace, target=X, expectedBehavior=Y
 *     - "替换 X 为 Y" / "修改 X 为 Y" → verb=replace
 *     - "禁用 X" / "删除 X" → verb=disable
 *     - "增加 X 方法 返回 Y" → verb=add
 *   通用兜底：把整句当作 expectedBehavior，target 由 scanCandidates 模糊匹配
 */
function parseIntent(naturalLanguage) {
  if (typeof naturalLanguage !== 'string' || naturalLanguage.trim().length === 0) {
    throw new Error('parseIntent 必须接收非空自然语言字符串');
  }
  const nl = naturalLanguage.trim();
  const intent = {
    raw: nl,
    verb: 'unknown',
    target: null,
    expectedBehavior: null,
    confidence: 0,
    parsedAt: Date.now(),
    sovereign: 'liuchutian_hunyuan_origin',
  };

  // 模式1：让/把/将 X 改成/变成/返回 Y
  let m = nl.match(/^(?:让|把|将)\s*(.+?)\s*(?:改成|变成|返回|改为|替换为|修改为)\s*(.+)$/);
  if (m) {
    intent.verb = 'replace';
    intent.target = m[1].trim();
    intent.expectedBehavior = m[2].trim();
    intent.confidence = 0.9;
    return intent;
  }
  // 模式2：替换 X 为 Y / 修改 X 为 Y
  m = nl.match(/^(?:替换|修改)\s+(.+?)\s+为\s+(.+)$/);
  if (m) {
    intent.verb = 'replace';
    intent.target = m[1].trim();
    intent.expectedBehavior = m[2].trim();
    intent.confidence = 0.85;
    return intent;
  }
  // 模式3：禁用 X / 删除 X
  m = nl.match(/^(?:禁用|删除|移除)\s+(.+)$/);
  if (m) {
    intent.verb = 'disable';
    intent.target = m[1].trim();
    intent.expectedBehavior = 'noop';
    intent.confidence = 0.8;
    return intent;
  }
  // 模式4：增加 X 方法 返回 Y
  m = nl.match(/^(?:增加|添加|新增)\s+(.+?)\s+(?:方法|函数)\s*(?:返回|返回值)?\s*(.*)$/);
  if (m) {
    intent.verb = 'add';
    intent.target = m[1].trim();
    intent.expectedBehavior = m[2].trim() || 'undefined';
    intent.confidence = 0.75;
    return intent;
  }
  // 兜底：整句当 expectedBehavior，target=null（让 scanCandidates 模糊匹配）
  intent.verb = 'fuzzy';
  intent.expectedBehavior = nl;
  intent.confidence = 0.5;
  return intent;
}

// ============================================================
// 二、候选扫描器 scanCandidates：49通道扫描目标对象
// ============================================================

/**
 * 候选扫描：扫描目标对象的所有可编辑函数，按意图匹配度排序
 *   - 每个候选含：functionName / fn / LCTQF签名 / 猫码评分 / 镜像姿态匹配度
 *   - 模糊匹配：意图.target 模糊匹配函数名（包含/相似度）
 *
 * @param {object} intent    parseIntent 返回的意图对象
 * @param {object} targetObj 被编辑的目标对象
 * @param {object} [hooks]   钩子 { mirrorRegistry, variantAlgebra, mqf }
 * @returns {Array<Candidate>} 候选列表
 */
function scanCandidates(intent, targetObj, hooks = {}) {
  if (!intent || !targetObj || typeof targetObj !== 'object') {
    throw new Error('scanCandidates 需要意图对象和目标对象');
  }
  const candidates = [];
  const allKeys = Object.keys(targetObj);
  for (const key of allKeys) {
    const fn = targetObj[key];
    if (typeof fn !== 'function') continue;

    // 匹配度计算：意图.target 与函数名的相似度
    const matchScore = computeMatchScore(intent.target, key);

    // 镜像姿态匹配（与 nanjian_mirror 联动）
    let mirrorPose = null;
    if (hooks.mirrorRegistry) {
      // 默认匹配 face_to_face（正面镜像·最适合"替换"意图）
      const poses = hooks.mirrorRegistry.matchByMengjiX(intent.verb === 'replace' ? '治' : '踪');
      mirrorPose = poses && poses.length > 0 ? poses[0] : 'face_to_face';
    } else {
      mirrorPose = 'face_to_face';
    }

    // 猫码评分（0-100）：匹配度×60 + 镜像姿态匹配×20 + 基础分20
    const catScore = Math.min(100, Math.round(matchScore * 60 + 20 + (mirrorPose ? 20 : 0)));

    // LCTQF 签名（与 l4l_mqf 联动，这里简化为哈希）
    const lctqfSig = makeLctqfSignature(key, intent.raw);

    // 变体加法评分（与 nanjian_variant_algebra 联动）
    let variantHint = 'SMRR';
    if (hooks.variantAlgebra) {
      // 默认 SMRR+SMRL=SMRRL 表示"混合编辑"
      try {
        const r = hooks.variantAlgebra.add('SMRR', 'SMRL');
        variantHint = r.result;
      } catch (_) {}
    }

    candidates.push({
      functionName: key,
      fn,
      matchScore,                  // 0~1 模糊匹配度
      catScore,                    // 0~100 猫码评分
      mirrorPose,                  // 7种镜像姿态之一
      variantHint,                 // 变体加法提示
      lctqfSig,                    // LCTQF签名
      intent: {
        verb: intent.verb,
        target: intent.target,
        expectedBehavior: intent.expectedBehavior,
      },
      sovereign: 'liuchutian_hunyuan_origin',
      candidateId: `cand_${candidates.length}_${key}`,
    });
  }
  // 按匹配度降序排序
  candidates.sort((a, b) => b.matchScore - a.matchScore);
  return candidates;
}

/** 模糊匹配度计算（包含关系+编辑距离） */
function computeMatchScore(target, key) {
  if (!target) return 0.3;  // 无明确目标 → 默认低分让用户自己选
  const t = String(target).toLowerCase();
  const k = String(key).toLowerCase();
  if (k === t) return 1.0;
  if (k.includes(t) || t.includes(k)) return 0.8;
  // 编辑距离相似度
  const dist = editDistance(t, k);
  const maxLen = Math.max(t.length, k.length);
  return Math.max(0, 1 - dist / maxLen);
}

function editDistance(a, b) {
  const m = a.length, n = b.length;
  if (m === 0) return n;
  if (n === 0) return m;
  const dp = Array.from({length: m+1}, () => new Array(n+1).fill(0));
  for (let i = 0; i <= m; i++) dp[i][0] = i;
  for (let j = 0; j <= n; j++) dp[0][j] = j;
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      dp[i][j] = a[i-1] === b[j-1]
        ? dp[i-1][j-1]
        : 1 + Math.min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]);
    }
  }
  return dp[m][n];
}

function makeLctqfSignature(functionName, intentRaw) {
  // 简化 LCTQF 签名（刘楚恬妻夫全法）：函数名+意图的简单哈希
  const str = `${functionName}|${intentRaw}|LCTQF`;
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash + str.charCodeAt(i)) | 0;
  }
  return `LCTQF_${(hash >>> 0).toString(16)}`;
}

// ============================================================
// 三、候选呈现器 presentCandidates：弹给用户看
// ============================================================

/**
 * 候选呈现：把候选列表格式化为用户可读的形式
 *   @param {Array<Candidate>} candidates
 *   @param {string} [format='cli']  'cli' | 'json'
 *   @returns {string|object} 格式化后的候选列表
 */
function presentCandidates(candidates, format = 'cli') {
  if (!Array.isArray(candidates)) throw new Error('presentCandidates 需要候选数组');
  if (candidates.length === 0) {
    return format === 'json' ? { count: 0, candidates: [] } : '（无候选基因位点）';
  }
  if (format === 'json') {
    return {
      count: candidates.length,
      candidates: candidates.map((c, i) => ({
        index: i,
        candidateId: c.candidateId,
        functionName: c.functionName,
        matchScore: c.matchScore,
        catScore: c.catScore,
        mirrorPose: c.mirrorPose,
        variantHint: c.variantHint,
        lctqfSig: c.lctqfSig,
      })),
    };
  }
  // CLI 格式
  const lines = [`共找到 ${candidates.length} 个候选基因位点：`, ''];
  candidates.forEach((c, i) => {
    lines.push(`  [${i}] ${c.functionName}`);
    lines.push(`      匹配度: ${(c.matchScore * 100).toFixed(0)}%  猫码评分: ${c.catScore}  镜像姿态: ${c.mirrorPose}  变体: ${c.variantHint}`);
    lines.push(`      LCTQF签名: ${c.lctqfSig}`);
    lines.push(`      候选ID: ${c.candidateId}`);
    lines.push('');
  });
  return lines.join('\n');
}

// ============================================================
// 四、候选选择器 selectCandidate：用户选 / judge智能体代选
// ============================================================

/**
 * 候选选择：用户指定 index，或 judge智能体自动选最高分
 *   @param {Array<Candidate>} candidates
 *   @param {number|null} [userChoice]  用户选择的索引（null=judge自动选）
 *   @param {object} [judgeHook]        judge智能体钩子 { autoSelect(candidates) }
 *   @returns {Candidate} 选中的候选
 */
function selectCandidate(candidates, userChoice = null, judgeHook = null) {
  if (!Array.isArray(candidates) || candidates.length === 0) {
    throw new Error('selectCandidate 候选列表为空');
  }
  // 用户明确选择
  if (typeof userChoice === 'number' && userChoice >= 0 && userChoice < candidates.length) {
    return Object.assign({}, candidates[userChoice], { selectedBy: 'user', selectedIndex: userChoice });
  }
  // judge智能体自动选（默认选猫码评分最高，已排序则取第一个）
  if (judgeHook && typeof judgeHook.autoSelect === 'function') {
    const idx = judgeHook.autoSelect(candidates);
    if (typeof idx === 'number' && idx >= 0 && idx < candidates.length) {
      return Object.assign({}, candidates[idx], { selectedBy: 'judge', selectedIndex: idx });
    }
  }
  // 默认选第一个（已按匹配度排序）
  return Object.assign({}, candidates[0], { selectedBy: 'auto_top', selectedIndex: 0 });
}

// ============================================================
// 五、自动导航器 autoNavigate：自动跑完 cut/replace/repair 四步
// ============================================================

/**
 * 自动导航：选好候选后，自动跑完四步编辑
 *   @param {Candidate} selected       选中的候选
 *   @param {object} targetObj         被编辑的目标对象
 *   @param {function} substituteFn    替换函数
 *   @param {object} geneEditor        NanjianGeneEditor 实例
 *   @param {string} [intent]          原始意图（用于 LCTQF 签名）
 *   @returns {object} 编辑结果 + diffReport
 */
function autoNavigate(selected, targetObj, substituteFn, geneEditor, intent = '') {
  if (!selected || !targetObj || typeof substituteFn !== 'function') {
    throw new Error('autoNavigate 需要选中的候选 + 目标对象 + 替换函数');
  }
  if (!geneEditor || typeof geneEditor.edit !== 'function') {
    throw new Error('autoNavigate 需要 NanjianGeneEditor 实例');
  }

  // before 快照
  const before = captureSnapshot(targetObj, selected.functionName);

  // 跑四步编辑
  const editResult = geneEditor.edit(
    targetObj,
    selected.functionName,
    substituteFn,
    intent || `遥控编程自动导航: ${selected.functionName}`
  );

  // after 快照
  const after = captureSnapshot(targetObj, selected.functionName);

  // diffReport
  const diff = diffReport(before, after, selected);

  return {
    selected,
    editResult,
    diffReport: diff,
    before,
    after,
    sovereign: 'liuchutian_hunyuan_origin',
    navigatedAt: Date.now(),
  };
}

/** 捕获函数快照（执行结果） */
function captureSnapshot(targetObj, functionName) {
  const fn = targetObj[functionName];
  if (typeof fn !== 'function') return { value: undefined, exists: false };
  try {
    // 尝试无参调用，失败则记录函数源码
    const value = fn();
    return { value, exists: true, type: typeof value };
  } catch (e) {
    return { value: null, exists: true, error: e.message, source: fn.toString().slice(0, 200) };
  }
}

// ============================================================
// 六、差异报告 diffReport：before/after 对比
// ============================================================

/**
 * 差异报告：对比 before/after 快照
 */
function diffReport(before, after, selected) {
  const changed = JSON.stringify(before.value) !== JSON.stringify(after.value);
  return {
    functionName: selected.functionName,
    mirrorPose: selected.mirrorPose,
    before: before.value,
    after: after.value,
    beforeType: before.type,
    afterType: after.type,
    changed,
    catScore: selected.catScore,
    lctqfSig: selected.lctqfSig,
    meaning: changed
      ? `后发编程生效: ${selected.functionName} 已被顶替 (${before.value} → ${after.value})`
      : `未变化: ${selected.functionName} 行为相同`,
    sovereign: 'liuchutian_hunyuan_origin',
  };
}

// ============================================================
// 七、遥控编程主入口 RemoteControlEditor：一站式封装
// ============================================================

/**
 * 遥控编程主入口：把上述5步封装成一个可链式调用的编辑器
 *   用法：
 *     const rc = new RemoteControlEditor({ geneEditor, mirrorRegistry, variantAlgebra });
 *     const result = await rc.editByVoice('让 greet 改成返回 hi', targetObj, () => 'hi');
 */
class RemoteControlEditor {
  constructor(options = {}) {
    this.version = '0.1.0-nanjian-nl-intent';
    this.sovereign = 'liuchutian_hunyuan_origin';
    this.geneEditor = options.geneEditor || null;
    this.mirrorRegistry = options.mirrorRegistry || null;
    this.variantAlgebra = options.variantAlgebra || null;
    this.mqf = options.mqf || null;
    this.judgeHook = options.judgeHook || null;
    this.fraudDetector = options.fraudDetector || null;
    this.history = [];  // 遥控编辑历史
  }

  /** 步骤1：解析意图 */
  parseIntent(nl) {
    return parseIntent(nl);
  }

  /** 步骤2：扫描候选 */
  scanCandidates(intent, targetObj) {
    return scanCandidates(intent, targetObj, {
      mirrorRegistry: this.mirrorRegistry,
      variantAlgebra: this.variantAlgebra,
      mqf: this.mqf,
    });
  }

  /** 步骤3：呈现候选 */
  presentCandidates(candidates, format = 'cli') {
    return presentCandidates(candidates, format);
  }

  /** 步骤4：选择候选 */
  selectCandidate(candidates, userChoice = null) {
    return selectCandidate(candidates, userChoice, this.judgeHook);
  }

  /** 步骤5：自动导航 */
  autoNavigate(selected, targetObj, substituteFn, intent = '') {
    const result = autoNavigate(selected, targetObj, substituteFn, this.geneEditor, intent);
    this.history.push(result);
    // 上链审计
    if (this.mqf && typeof this.mqf.publish === 'function') {
      try {
        this.mqf.publish('rc_edit_audit', {
          functionName: selected.functionName,
          changed: result.diffReport.changed,
          catScore: selected.catScore,
          lctqfSig: selected.lctqfSig,
        }, { catCode: result.diffReport.changed ? 'white' : 'gray', amount: 10 });
      } catch (_) {}
    }
    return result;
  }

  /**
   * 一站式：自然语言 → 自动跑完全流程
   *   @param {string} naturalLanguage  自然语言意图
   *   @param {object} targetObj        被编辑对象
   *   @param {function} substituteFn   替换函数
   *   @param {number|null} [userChoice] 用户选择索引（null=自动选）
   *   @returns {object} 完整流程结果
   */
  editByVoice(naturalLanguage, targetObj, substituteFn, userChoice = null) {
    // 1. 解析意图
    const intent = this.parseIntent(naturalLanguage);
    // 2. 扫描候选
    const candidates = this.scanCandidates(intent, targetObj);
    if (candidates.length === 0) {
      return { ok: false, reason: 'no_candidates', intent };
    }
    // 3. 呈现候选（保留 JSON 格式供程序读取）
    const presentation = this.presentCandidates(candidates, 'json');
    // 4. 选择候选
    const selected = this.selectCandidate(candidates, userChoice);
    // 5. 自动导航
    const navigation = this.autoNavigate(selected, targetObj, substituteFn, intent.raw);

    return {
      ok: true,
      intent,
      candidates: presentation,
      selected,
      navigation,
      sovereign: this.sovereign,
    };
  }

  /** 历史记录 */
  listHistory() {
    return this.history.slice();
  }

  /** 统计 */
  stats() {
    return {
      version: this.version,
      sovereign: this.sovereign,
      historyCount: this.history.length,
      hasGeneEditor: !!this.geneEditor,
      hasMirrorRegistry: !!this.mirrorRegistry,
      hasVariantAlgebra: !!this.variantAlgebra,
      hasFraudDetector: !!this.fraudDetector,
    };
  }
}

// ============================================================
// 八、导出
// ============================================================

module.exports = {
  parseIntent,
  scanCandidates,
  presentCandidates,
  selectCandidate,
  autoNavigate,
  diffReport,
  RemoteControlEditor,
  // 辅助函数（供测试）
  computeMatchScore,
  editDistance,
  makeLctqfSignature,
  captureSnapshot,
};
