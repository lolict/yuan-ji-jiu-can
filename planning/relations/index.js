/**
 * ============================================================
 * 关联关系图谱索引 (Relations Graph Index)
 * ============================================================
 * 
 * 作用：
 * 1. 把全局冠名项 + 任务节点 + 技能时序 三者之间的关联建立起来
 * 2. 提供"关系代数"的查询入口
 * 3. 生成Mermaid可视化关系图
 * 
 * 关系类型 (Relation Types):
 *   - implements: 任务节点 implements(实现) 冠名项
 *   - produces:   阶段A produces 交付物 → 被阶段B consumes
 *   - depends:    任务节点 depends 冠名项前置准备
 *   - skill_map:  阶段 skill_bundle 映射到具体任务节点
 *   - gi_ref:     最优秀指令 GI_xxx 映射到 阶段/冠名项/任务
 * ============================================================
 */

const { GLOBAL_ANCHORS, RELATIONS_ALGEBRA } = require('../global_anchors.js');
const { TASK_NODES, TASK_SCHEDULER } = require('../task_nodes/index.js');
const { SKILL_TIMELINE, GREATEST_INSTRUCTIONS } = require('../skill_timeline/index.js');

// ============================================================
// 关系类型常量
// ============================================================

const RELATION_TYPES = {
  IMPLEMENTS: 'implements',    // 任务节点 → 实现了 → 冠名项
  PRODUCES:    'produces',     // 任务节点 → 产出 → 阶段交付物
  CONSUMES:    'consumes',     // 阶段A → consume → 阶段B的产出
  DEPENDS:     'depends',      // 任务节点 → 依赖 → 冠名项
  SKILL_MAP:   'skill_map',    // 技能 → 可处理 → 任务节点
  GI_REF:      'gi_ref',       // 最优秀指令 → 引用 → 冠名项/阶段/任务
};

// ============================================================
// 构建完整关系图谱
// ============================================================

function buildRelationsGraph() {
  const edges = []; // { from, to, type, meta }
  const nodes = new Map(); // id → { id, label, kind }

  // --- 第一步：注册所有冠名节点 ---
  for (const [id, anchor] of Object.entries(GLOBAL_ANCHORS)) {
    nodes.set(id, {
      id, label: anchor.name || id, kind: 'anchor',
      type: anchor.type || 'unknown',
    });
  }

  // --- 第二步：注册所有任务节点 ---
  for (const [id, task] of Object.entries(TASK_NODES)) {
    nodes.set(id, {
      id, label: task.title || id, kind: 'task',
      phase: task.phase, priority: task.priority,
    });
  }

  // --- 第三步：注册时序阶段 + 最优秀指令 ---
  for (const stage of SKILL_TIMELINE) {
    nodes.set('stage_' + stage.stage, {
      id: 'stage_' + stage.stage,
      label: `${stage.stage} ${stage.name}`,
      kind: 'timeline_stage',
    });
  }
  for (const gi of GREATEST_INSTRUCTIONS) {
    nodes.set(gi.id, {
      id: gi.id, label: `${gi.id} ${gi.quote.substring(0, 30)}...`,
      kind: 'greatest_instruction',
    });
  }

  // --- 边1: 冠名项之间原生bind关系 (从global_anchors继承) ---
  for (const [id, anchor] of Object.entries(GLOBAL_ANCHORS)) {
    for (const bindId of (anchor.bind || [])) {
      if (GLOBAL_ANCHORS[bindId]) {
        edges.push({
          from: id, to: bindId, type: 'anchor_bind',
          meta: { desc: '冠名项原生关联' },
        });
      }
    }
  }

  // --- 边2: 任务节点 → implements → 冠名项 (通过anchor_bindings) ---
  for (const [id, task] of Object.entries(TASK_NODES)) {
    for (const anchorId of (task.anchor_bindings || [])) {
      if (GLOBAL_ANCHORS[anchorId]) {
        edges.push({
          from: id, to: anchorId, type: RELATION_TYPES.IMPLEMENTS,
          meta: { desc: `任务「${task.title.substring(0,20)}」实现冠名项「${GLOBAL_ANCHORS[anchorId].name}」` },
        });
      }
    }
  }

  // --- 边3: 任务节点 → depends_on → 其他任务 (原生) ---
  for (const [id, task] of Object.entries(TASK_NODES)) {
    for (const depId of (task.depends_on || [])) {
      if (TASK_NODES[depId]) {
        edges.push({
          from: id, to: depId, type: 'task_depends',
          meta: { desc: `任务依赖` },
        });
      }
    }
  }

  // --- 边4: 任务节点 → belongs → stage (按phase映射) ---
  for (const [id, task] of Object.entries(TASK_NODES)) {
    if (!task.phase) continue;
    // phase格式: "P0-底层符号集" → "T0"
    const pNum = task.phase.match(/^P(\d+)/);
    if (pNum) {
      const stageId = 'stage_T' + pNum[1];
      if (nodes.has(stageId)) {
        edges.push({
          from: id, to: stageId, type: 'task_in_stage',
          meta: { desc: `任务属于阶段${stageId}` },
        });
      }
    }
  }

  // --- 边5: 阶段 → stage_bindings → 冠名项 ---
  for (const stage of SKILL_TIMELINE) {
    const stageId = 'stage_' + stage.stage;
    for (const anchorId of (stage.bindings || [])) {
      if (GLOBAL_ANCHORS[anchorId]) {
        edges.push({
          from: stageId, to: anchorId, type: 'stage_bind_anchor',
          meta: { desc: `阶段绑定冠名项` },
        });
      }
    }
  }

  // --- 边6: 最优秀指令 GI → 引用 → 阶段 ---
  for (const gi of GREATEST_INSTRUCTIONS) {
    // impact字段如"T0~T5"或"T4"
    const impacts = (gi.impact || '').match(/T\d+/g) || [];
    for (const t of impacts) {
      const stageId = 'stage_' + t;
      if (nodes.has(stageId)) {
        edges.push({
          from: gi.id, to: stageId, type: RELATION_TYPES.GI_REF,
          meta: { desc: `指令${gi.id}影响阶段${t}` },
        });
      }
    }
  }

  return {
    nodes: Array.from(nodes.values()),
    edges,
    stats: {
      node_count: nodes.size,
      edge_count: edges.length,
      anchor_count: Array.from(nodes.values()).filter(n => n.kind === 'anchor').length,
      task_count: Array.from(nodes.values()).filter(n => n.kind === 'task').length,
      stage_count: Array.from(nodes.values()).filter(n => n.kind === 'timeline_stage').length,
      gi_count: Array.from(nodes.values()).filter(n => n.kind === 'greatest_instruction').length,
    },
  };
}

// ============================================================
// "关系代数" 高级查询器
// ============================================================

class SuperRelationsAlgebra {
  constructor() {
    this.graph = buildRelationsGraph();
    this.nodeMap = new Map(this.graph.nodes.map(n => [n.id, n]));
  }

  /**
   * 综合查询：输入任意关键词 → 冠名项/任务/阶段/GI → 输出关联子图
   */
  queryEverything(keyword, maxDepth = 2) {
    // 1. 先模糊搜索冠名项
    const anchorHits = RELATIONS_ALGEBRA.search(keyword).slice(0, 5);
    const results = [];

    for (const hit of anchorHits) {
      results.push({
        kind: 'anchor_hit',
        id: hit.id,
        name: hit.name,
        direct_relations: RELATIONS_ALGEBRA.query(hit.id),
      });
    }

    // 2. 搜索任务节点
    const kw = keyword.toLowerCase();
    const taskHits = Object.values(TASK_NODES).filter(t =>
      t.title.toLowerCase().includes(kw) ||
      t.description.toLowerCase().includes(kw)
    ).slice(0, 5);

    for (const task of taskHits) {
      const relatedAnchors = (task.anchor_bindings || []).map(aid => ({
        id: aid,
        name: GLOBAL_ANCHORS[aid]?.name || aid,
      }));
      results.push({
        kind: 'task_hit',
        id: task.id,
        phase: task.phase,
        title: task.title,
        estimated_minutes: task.estimated_minutes,
        implements_anchors: relatedAnchors,
        depends_on: task.depends_on,
      });
    }

    // 3. 搜索最优秀指令
    const giHits = GREATEST_INSTRUCTIONS.filter(gi =>
      gi.quote.toLowerCase().includes(kw) ||
      gi.context.toLowerCase().includes(kw) ||
      gi.impact.toLowerCase().includes(kw)
    ).slice(0, 3);
    for (const gi of giHits) {
      results.push({
        kind: 'gi_hit',
        id: gi.id,
        quote: gi.quote,
        context: gi.context,
        impact: gi.impact,
      });
    }

    return {
      keyword,
      total_hits: results.length,
      results,
    };
  }

  /**
   * 冠名项→任务链：输入冠名项ID→哪些任务实现了它+它们依赖什么
   */
  anchorToTaskChain(anchorId) {
    const tasks = Object.values(TASK_NODES).filter(t =>
      (t.anchor_bindings || []).includes(anchorId)
    );
    const chains = tasks.map(t => {
      const deps = t.depends_on.map(depId => TASK_NODES[depId]?.title || depId);
      return {
        taskId: t.id,
        taskTitle: t.title,
        phase: t.phase,
        priority: t.priority,
        estimated_minutes: t.estimated_minutes,
        requires: deps,
      };
    });
    return {
      anchor: {
        id: anchorId,
        name: GLOBAL_ANCHORS[anchorId]?.name || anchorId,
        direct_bind: GLOBAL_ANCHORS[anchorId]?.bind || [],
      },
      implementing_tasks: chains,
    };
  }

  /**
   * 完整项目状态总览 (记忆明朗化的核心)
   */
  fullMemoryReport() {
    const progress = TASK_SCHEDULER.getProgress();
    const timeline = TASK_SCHEDULER.getTimelineByPhase();
    return {
      overview: {
        project: 'UniCore - 统一智能核心系统',
        vision: GLOBAL_ANCHORS.unicore_root?.vision || '',
        principles: GLOBAL_ANCHORS.unicore_root?.principles || '',
      },
      global_anchors_stats: {
        layers: 15,
        total_anchors: Object.keys(GLOBAL_ANCHORS).length,
      },
      task_progress: progress,
      timeline_breakdown: timeline.map(t => ({
        phase: t.phase,
        task_count: t.nodes.length,
        total_hours: Math.round(t.total_minutes / 60 * 10) / 10,
        top_3_tasks: t.nodes.slice(0, 3).map(n => ({
          id: n.id, title: n.title, minutes: n.minutes,
        })),
      })),
      greatest_instructions_count: GREATEST_INSTRUCTIONS.length,
      skills_total_covered: new Set(SKILL_TIMELINE.flatMap(s => s.skill_bundle)).size,
      estimated_total_days: SKILL_TIMELINE.reduce((s, t) => s + t.duration_days, 0),
      graph_stats: this.graph.stats,
    };
  }

  /**
   * 生成Mermaid总关系图 (大而全，适合文档渲染)
   */
  generateMermaidOverview() {
    const lines = ['flowchart TB'];
    lines.push('  %% UniCore 项目全局冠名+任务+阶段 关联总览 %%');
    lines.push('');

    // 子图1: 8大阶段
    lines.push('  subgraph TIMELINE[技能化时序 T0~T7 从底层向上]');
    lines.push('    direction LR');
    for (let i = 0; i < SKILL_TIMELINE.length; i++) {
      const s = SKILL_TIMELINE[i];
      const safeName = `stage_${s.stage}`;
      lines.push(`    ${safeName}["${s.stage} ${s.name} (${s.duration_days}天)"]`);
      if (i > 0) {
        const prev = SKILL_TIMELINE[i - 1];
        lines.push(`    stage_${prev.stage} -->|depends on| ${safeName}`);
      }
    }
    lines.push('  end');
    lines.push('');

    // 子图2: 核心冠名项 (挑最重要的30+个)
    const coreAnchors = [
      'unicore_root', 'unicore_architecture_root',
      'radix_shengmu', 'radix_yunmu', 'radix_shengdiao',
      'container_1', 'container_2', 'container_3', 'container_4',
      'container_self_radix', 'leading_zero_negative', 'no_redundant_mapping',
      'four_dim_system', 'static_frame_123', 'equivalent_mapping',
      'pure_symbol_system', 'pure_grid',
      'pure_huren', 'pure_jie', 'pure_zhenfa', 'pure_modules',
      'pure_cbc', 'sparse_algebra',
      'cbc_magic', 'cbc_format_spec', 'cbc_self_container',
      'unicore_foundation', 'uniisa_v2', 'node_time_system',
      'unicore_vm',
      'cube_table', 'hacsjs_3d', 'unicore_system_core',
      'unicore_guardian', 'global_memory_anchor',
    ];
    lines.push('  subgraph ANCHORS[核心全局冠名项 ROOT~底层系统]');
    for (const aid of coreAnchors) {
      const a = GLOBAL_ANCHORS[aid];
      if (a) {
        const safe = aid.replace(/[^a-zA-Z0-9_]/g, '_');
        lines.push(`    ${safe}["${a.name || aid}"]`);
      }
    }
    lines.push('  end');
    lines.push('');

    // 子图3: 最优秀指令
    lines.push('  subgraph GI[最优秀指令 GI_001~GI_010 用户原话精华]');
    lines.push('    direction LR');
    for (const gi of GREATEST_INSTRUCTIONS.slice(0, 5)) {
      lines.push(`    ${gi.id}["${gi.id}: ${gi.quote.substring(0,24).replace(/"/g,'\'')}..."]`);
    }
    lines.push('  end');
    lines.push('');

    // 连接 GI → 阶段
    lines.push('  %% 最优秀指令驱动各阶段');
    for (const gi of GREATEST_INSTRUCTIONS) {
      const ts = (gi.impact || '').match(/T\d+/g) || [];
      for (const t of ts.slice(0, 2)) {
        lines.push(`  ${gi.id} -.-> stage_${t}`);
      }
    }
    lines.push('');

    // 连接 阶段 → 核心冠名 (每条阶段选最核心的绑定)
    for (const s of SKILL_TIMELINE) {
      const sid = `stage_${s.stage}`;
      const topBind = (s.bindings || []).slice(0, 2);
      for (const b of topBind) {
        const safe = b.replace(/[^a-zA-Z0-9_]/g, '_');
        if (coreAnchors.includes(b)) {
          lines.push(`  ${sid} --> ${safe}`);
        }
      }
    }

    // 冠名项之间几个关键链
    lines.push('');
    lines.push('  %% 符号→集装箱→四维→纯符号→CBC→指令→VM 主链路');
    lines.push('  radix_shengmu --> container_1');
    lines.push('  radix_yunmu --> container_2');
    lines.push('  radix_shengdiao --> container_3');
    lines.push('  container_1 --> four_dim_system');
    lines.push('  container_2 --> four_dim_system');
    lines.push('  container_3 --> four_dim_system');
    lines.push('  four_dim_system --> static_frame_123');
    lines.push('  container_self_radix --> four_dim_system');
    lines.push('  leading_zero_negative -.-> container_self_radix');
    lines.push('  four_dim_system --> pure_symbol_system');
    lines.push('  pure_symbol_system --> pure_grid');
    lines.push('  pure_symbol_system --> pure_huren');
    lines.push('  pure_symbol_system --> pure_jie');
    lines.push('  pure_symbol_system --> pure_zhenfa');
    lines.push('  pure_symbol_system --> pure_modules');
    lines.push('  pure_symbol_system --> pure_cbc');
    lines.push('  pure_cbc --> cbc_magic');
    lines.push('  pure_cbc --> cbc_format_spec');
    lines.push('  cbc_format_spec --> unicore_foundation');
    lines.push('  unicore_foundation --> uniisa_v2');
    lines.push('  uniisa_v2 --> node_time_system');
    lines.push('  uniisa_v2 --> unicore_vm');
    lines.push('  unicore_vm --> cube_table');
    lines.push('  unicore_vm --> unicore_system_core');
    lines.push('  unicore_guardian --> cbc_magic');
    lines.push('  unicore_system_core --> unicore_root');
    lines.push('  global_memory_anchor --> unicore_root');

    return lines.join('\n');
  }
}

// ============================================================
// 导出
// ============================================================

const SUPER_ALGEBRA = new SuperRelationsAlgebra();

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    buildRelationsGraph,
    SuperRelationsAlgebra,
    SUPER_ALGEBRA,
    RELATION_TYPES,
  };
} else {
  window.UniCoreRelations = {
    buildRelationsGraph,
    SuperRelationsAlgebra,
    SUPER_ALGEBRA,
    RELATION_TYPES,
  };
}

const rpt = SUPER_ALGEBRA.fullMemoryReport();
console.log(`[Relations Graph] 关系图谱构建完成:
  → 总节点数: ${rpt.graph_stats.node_count}
    · 冠名项: ${rpt.graph_stats.anchor_count}
    · 任务节点: ${rpt.graph_stats.task_count}
    · 时序阶段: ${rpt.graph_stats.stage_count} (T0~T7)
    · 最优秀指令: ${rpt.graph_stats.gi_count} (GI_001~GI_010)
  → 总关联边数: ${rpt.graph_stats.edge_count}
  → 项目总工期: ${rpt.estimated_total_days} 天
  → 全局冠名项: ${rpt.global_anchors_stats.total_anchors} 个
  → 覆盖技能数: ${rpt.skills_total_covered_covered || rpt.skills_total_covered} 种
  → 当前任务完成度: ${rpt.task_progress.percent}%
  → 调用 SUPER_ALGEBRA.queryEverything("关键词") / .anchorToTaskChain("冠名ID") / .generateMermaidOverview() / .fullMemoryReport() 即可让一切记忆明朗起来！
`);
