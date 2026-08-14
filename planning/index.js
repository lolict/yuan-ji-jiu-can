/**
 * ============================================================
 * UniCore 任务规划仓库总入口 (Roadmap Master Entry)
 * ============================================================
 * 
 * 本目录的使命 (本规划的存在意义，来自用户原话 GI_010)：
 * 
 *   「最底层优先级最高的工作任务，技能化时序、分布式切片节点化，
 *    写入项目仓库；把过去所有记忆、指令、目标全部找回，
 *    冠名关联起来，做成全局冠名项，让一切记忆都明朗起来。」
 * 
 * 子模块一览：
 *   ./global_anchors.js    →  全局冠名关联项知识库 (15层300+冠名项)
 *                             + RelationsAlgebra "关系代数"基础查询
 *   ./task_nodes/index.js  →  分布式切片任务节点系统 (30+节点/P0~P8)
 *                             + TaskNodeScheduler 调度器
 *   ./skill_timeline/index.js → 技能化时序规划 (T0~T7)
 *                             + GI_001~GI_010 最优秀指令索引
 *   ./relations/index.js   →  关联关系图谱索引
 *                             + SuperRelationsAlgebra "超级关系代数"
 * 
 * 使用方法：
 *   node planning/index.js   →  输出完整项目记忆报告
 *   进入node REPL后：
 *     const roadmap = require('./planning/index.js')
 *     roadmap.report()       → 完整记忆总览
 *     roadmap.find("稀疏数") → 跨冠名项/任务/GI搜索
 *     roadmap.path("container_1", "cbc_magic")  → 找关联路径
 *     roadmap.next()         → 下一个可立即执行的就绪任务
 * ============================================================
 */

const {
  GLOBAL_ANCHORS, RELATIONS_ALGEBRA,
  SYMBOL_ANCHORS, RADIX_ANCHORS, CONTAINER_ANCHORS, FOURDIM_ANCHORS,
  PURE_SYMBOL_ANCHORS, SYSTEM_ANCHORS, CBC_ANCHORS, SPARSE_ANCHORS,
  ISA_ANCHORS, TIME_NODE_ANCHORS, VM_ANCHORS, GUARDIAN_ANCHORS,
  UI_ANCHORS, LANG_ANCHORS, ROOT_ANCHORS,
} = require('./global_anchors.js');

const {
  TASK_NODES, TaskNodeScheduler, TASK_SCHEDULER,
} = require('./task_nodes/index.js');

const {
  SKILL_TIMELINE, GREATEST_INSTRUCTIONS,
} = require('./skill_timeline/index.js');

const {
  buildRelationsGraph, SuperRelationsAlgebra, SUPER_ALGEBRA, RELATION_TYPES,
} = require('./relations/index.js');

// 后发编程·遥控编程·基因编辑器 路线图（时序T0~T7 + 空间布局 + 每步代码）
const NanjianGeneRoadmap = require('./nanjian_gene_roadmap.js');

// ============================================================
// 路线图对外 API
// ============================================================

const Roadmap = {
  /** 打印完整记忆报告 (让一切记忆都明朗起来) */
  report() {
    const rpt = SUPER_ALGEBRA.fullMemoryReport();
    const sep = '═'.repeat(64);
    const thinSep = '─'.repeat(64);

    console.log(`\n${sep}`);
    console.log(`  🧠 UniCore 项目完整记忆报告 (Memory Report)`);
    console.log(sep);

    console.log(`\n【项目愿景】${rpt.overview.vision}`);
    console.log(`【核心理念】${rpt.overview.principles}`);

    console.log(`\n${thinSep}`);
    console.log(`  📊 任务进度总览`);
    console.log(thinSep);
    const p = rpt.task_progress;
    console.log(`  总节点数: ${p.total}  |  已完成: ${p.done}  |  失败: ${p.failed}  |  运行中: ${p.running}  |  待执行: ${p.pending}`);
    console.log(`  进度百分比: ${p.percent}%`);
    console.log(`  工时: 已投入 ${p.doneEstimatedHours}h / 总 ${p.totalEstimatedHours}h / 剩余 ${p.remainingEstimatedHours}h`);

    console.log(`\n${thinSep}`);
    console.log(`  🧭 技能化时序分解 (T0~T7 底层→顶层)`);
    console.log(thinSep);
    for (const phase of rpt.timeline_breakdown) {
      console.log(`  ${phase.phase.padEnd(22)} | 任务 ${String(phase.task_count).padStart(2)}个 | ${phase.total_hours}h`);
      for (const t of phase.top_3_tasks) {
        console.log(`    · ${t.id.padEnd(20)} ${String(t.minutes).padStart(3)}分钟  ${t.title}`);
      }
    }

    console.log(`\n${thinSep}`);
    console.log(`  🏷️  全局冠名项分层`);
    console.log(thinSep);
    console.log(`  · 符号层(叶)           ${Object.keys(SYMBOL_ANCHORS).length} 项`);
    console.log(`  · 进制/优先级层        ${Object.keys(RADIX_ANCHORS).length} 项`);
    console.log(`  · 集装箱层             ${Object.keys(CONTAINER_ANCHORS).length} 项`);
    console.log(`  · 四维体系层           ${Object.keys(FOURDIM_ANCHORS).length} 项`);
    console.log(`  · 纯符号/格子层        ${Object.keys(PURE_SYMBOL_ANCHORS).length} 项`);
    console.log(`  · 互认/阶系/阵法/聚类  ${Object.keys(SYSTEM_ANCHORS).length} 项`);
    console.log(`  · 汉字积木/CBC层       ${Object.keys(CBC_ANCHORS).length} 项`);
    console.log(`  · 稀疏数代数层         ${Object.keys(SPARSE_ANCHORS).length} 项`);
    console.log(`  · UniISA指令集层       ${Object.keys(ISA_ANCHORS).length} 项`);
    console.log(`  · 节点时间因果层       ${Object.keys(TIME_NODE_ANCHORS).length} 项`);
    console.log(`  · VM/基础层            ${Object.keys(VM_ANCHORS).length} 项`);
    console.log(`  · 知识产权守护层       ${Object.keys(GUARDIAN_ANCHORS).length} 项`);
    console.log(`  · 表格/3D引擎层        ${Object.keys(UI_ANCHORS).length} 项`);
    console.log(`  · 多语言开发层         ${Object.keys(LANG_ANCHORS).length} 项`);
    console.log(`  · 根节点/总览层(根)    ${Object.keys(ROOT_ANCHORS).length} 项`);
    console.log(`  ────────────────────────────────────────`);
    console.log(`  · 总计:                ${rpt.global_anchors_stats.total_anchors} 项冠名`);

    console.log(`\n${thinSep}`);
    console.log(`  💎 最优秀指令 TOP 10 (用户原话精华)`);
    console.log(thinSep);
    for (const gi of GREATEST_INSTRUCTIONS) {
      console.log(`  [${gi.id}] 影响范围: ${gi.impact}`);
      console.log(`         「${gi.quote}」`);
      console.log(`            ↳ ${gi.context}`);
    }

    console.log(`\n${thinSep}`);
    console.log(`  🕸️  关系图谱规模`);
    console.log(thinSep);
    const gs = rpt.graph_stats;
    console.log(`  总节点 ${gs.node_count} (冠名${gs.anchor_count} + 任务${gs.task_count} + 阶段${gs.stage_count} + GI${gs.gi_count})`);
    console.log(`  总关联边: ${gs.edge_count} 条`);

    console.log(`\n${sep}`);
    console.log(`  📌 工期 & 范围`);
    console.log(sep);
    console.log(`  涉及技能: ${rpt.skills_total_covered} 种  |  最优秀指令: ${rpt.greatest_instructions_count} 条`);
    console.log(`  预估总工期: ${rpt.estimated_total_days} 天 (约 ${Math.round(rpt.estimated_total_days/5*10)/10} 周，按 5天/周计算)`);
    console.log(`\n  💡 如何使用:`);
    console.log(`     roadmap.find("关键词")     → 跨冠名/任务/GI搜索`);
    console.log(`     roadmap.path("A","B")      → 两冠名项间关联路径`);
    console.log(`     roadmap.next()             → 立即可以做的前5个就绪任务`);
    console.log(`     roadmap.task("node_xxx")   → 查看单个任务详情`);
    console.log(`     roadmap.anchor("xxx")      → 查看单个冠名项关联`);
    console.log(`     roadmap.mermaid()          → 输出Mermaid总览图`);
    console.log(`${sep}\n`);
  },

  /** 跨冠名项、任务、最优秀指令的模糊搜索 */
  find(keyword) {
    console.log(`\n🔎 全项目搜索："${keyword}"`);
    const res = SUPER_ALGEBRA.queryEverything(keyword);
    if (res.total_hits === 0) {
      console.log(`  → 没有找到匹配项。`);
      return res;
    }
    console.log(`  → 共找到 ${res.total_hits} 条相关结果\n`);
    for (const r of res.results) {
      if (r.kind === 'anchor_hit') {
        console.log(`  🏷️  [冠名项] ${r.id} — ${r.name}`);
        const dr = r.direct_relations;
        if (dr && dr.direct_relations) {
          console.log(`     直接关联: ${dr.direct_relations.slice(0,4).map(x=>x.name).join('、') || '无'}${dr.direct_relations.length>4?'...':''}`);
        }
      } else if (r.kind === 'task_hit') {
        console.log(`  📋 [任务节点] ${r.id} — ${r.phase} / ${r.priority}优先 / ${r.estimated_minutes}分钟`);
        console.log(`     标题: ${r.title}`);
        if (r.implements_anchors && r.implements_anchors.length) {
          console.log(`     实现冠名项: ${r.implements_anchors.map(a=>a.name).slice(0,3).join('、')}`);
        }
      } else if (r.kind === 'gi_hit') {
        console.log(`  💎 [最优秀指令] ${r.id}`);
        console.log(`     「${r.quote}」`);
        console.log(`     上下文: ${r.context}`);
        console.log(`     影响: ${r.impact}`);
      }
      console.log();
    }
    return res;
  },

  /** 查找两个冠名项之间的关联路径 */
  path(fromId, toId) {
    console.log(`\n🛤️  关联路径: "${fromId}" → "${toId}"`);
    const from = GLOBAL_ANCHORS[fromId] ? GLOBAL_ANCHORS[fromId].name : fromId;
    const to = GLOBAL_ANCHORS[toId] ? GLOBAL_ANCHORS[toId].name : toId;
    console.log(`  起点: ${fromId} (${from})`);
    console.log(`  终点: ${toId} (${to})`);
    const r = RELATIONS_ALGEBRA.findPath(fromId, toId, 8);
    if (!r.found) {
      console.log(`  → 没找到路径(最大深度8)，他们可能分属不同子域。`);
      return r;
    }
    console.log(`  → 共 ${r.path.length - 1} 跳:`);
    r.pathWithNames.forEach((step, i) => {
      const prefix = i === 0 ? '  [' : '   → ';
      const suffix = i === r.pathWithNames.length - 1 ? '] ✓' : '';
      console.log(`${prefix}${step.id.padEnd(28)} ${step.name}${suffix}`);
    });
    return r;
  },

  /** 下一批可立即执行的就绪任务 */
  next(topN = 5) {
    console.log(`\n⏭️  当前可立即执行的就绪任务 TOP ${topN} (依赖全满足):`);
    const ready = TASK_SCHEDULER.getReadyNodes().slice(0, topN);
    if (ready.length === 0) {
      console.log(`  → 暂无可执行任务 (或所有依赖未满足，或全部已完成)`);
      const p = TASK_SCHEDULER.getProgress();
      console.log(`     当前总体进度 ${p.percent}% / 已完成 ${p.done}/${p.total}`);
      return ready;
    }
    for (let i = 0; i < ready.length; i++) {
      const n = ready[i];
      console.log(`\n  ${i + 1}. [${n.id}]  ${n.phase} · 优先级${n.priority} · 预估${n.estimated_minutes}分钟`);
      console.log(`     📌 标题: ${n.title}`);
      console.log(`     👷 技能: ${(n.skills || []).join('、')}`);
      console.log(`     🏷️  关联冠名: ${(n.anchor_bindings || []).slice(0, 4).join('、')}${n.anchor_bindings?.length > 4 ? '...' : ''}`);
      if (n.acceptance_criteria) {
        console.log(`     ✅ 验收要点: ${n.acceptance_criteria.slice(0, 2).join('；')}${n.acceptance_criteria.length > 2 ? '...' : ''}`);
      }
    }
    return ready;
  },

  /** 查看单个任务节点详情 */
  task(nodeId) {
    const n = TASK_NODES[nodeId];
    if (!n) {
      console.log(`\n❌ 未找到任务节点 ${nodeId}`);
      return null;
    }
    console.log(`\n📋 任务节点详情: ${nodeId}`);
    console.log(`  标题: ${n.title}`);
    console.log(`  阶段: ${n.phase}  |  切片: ${n.slice_size}  |  优先级: ${n.priority}  |  状态: ${n.status}`);
    console.log(`  预估: ${n.estimated_minutes} 分钟 (约 ${Math.round(n.estimated_minutes/60*10)/10} 小时)`);
    console.log(`  所需技能: ${(n.skills || []).join('、')}`);
    console.log(`\n📝 说明:`);
    console.log(`  ${n.description || '(无)'}`);
    console.log(`\n🏷️  冠名项绑定:`);
    for (const b of (n.anchor_bindings || [])) {
      const a = GLOBAL_ANCHORS[b];
      console.log(`  · ${b.padEnd(28)} ${a ? a.name : '(未定义)'}`);
    }
    console.log(`\n🔗 依赖节点 (depends_on):`);
    for (const d of (n.depends_on || [])) {
      const dep = TASK_NODES[d];
      const st = dep ? `[${dep.status}]` : '[未定义]';
      console.log(`  · ${st} ${d.padEnd(24)} ${dep ? dep.title : '(缺失)'}`);
    }
    console.log(`\n📤 产出节点 (produces):`);
    for (const p of (n.produces || [])) {
      const pr = TASK_NODES[p];
      console.log(`  · ${p.padEnd(24)} ${pr ? pr.title : ''}`);
    }
    console.log(`\n✅ 验收条件:`);
    (n.acceptance_criteria || []).forEach((c, i) => console.log(`  ${i + 1}. ${c}`));
    if (n.rollback_hint) {
      console.log(`\n⏪ 回滚要点: ${n.rollback_hint}`);
    }
    return n;
  },

  /** 查看单个冠名项详情 + 直接关联 */
  anchor(anchorId) {
    const r = RELATIONS_ALGEBRA.query(anchorId);
    if (!r.found) {
      console.log(`\n❌ 冠名项不存在: ${anchorId}`);
      return null;
    }
    console.log(`\n🏷️  冠名项详情: ${anchorId}`);
    console.log(`  名称: ${r.name}`);
    console.log(`  类型: ${r.type}`);
    if (r.desc) console.log(`  描述: ${r.desc}`);
    console.log(`\n🔗 直接关联 (${r.direct_relations.length} 项):`);
    for (const dr of r.direct_relations) {
      const flag = dr.type === 'missing' ? '⚠️ ' : '   ';
      console.log(`  ${flag}[${dr.type.padEnd(10)}] ${dr.id.padEnd(28)} ${dr.name}`);
    }
    return r;
  },

  /** 生成 Mermaid 总览图 */
  mermaid() {
    const mmd = SUPER_ALGEBRA.generateMermaidOverview();
    console.log(`\n📊 Mermaid 总览图 (复制到 mermaid.live 渲染):\n`);
    console.log('```mermaid');
    console.log(mmd);
    console.log('```');
    return mmd;
  },

  // 后发编程·基因编辑器路线图（独立子模块，含时序/空间/步骤/开放问题）
  geneRoadmap: NanjianGeneRoadmap,

  // 暴露内部子系统供高级调用
  anchors: GLOBAL_ANCHORS,
  tasks: TASK_NODES,
  scheduler: TASK_SCHEDULER,
  timeline: SKILL_TIMELINE,
  greatestInstructions: GREATEST_INSTRUCTIONS,
  relationsAlgebra: RELATIONS_ALGEBRA,
  superAlgebra: SUPER_ALGEBRA,
};

// ============================================================
// 运行入口
// ============================================================

if (require.main === module) {
  Roadmap.report();
}

module.exports = Roadmap;
