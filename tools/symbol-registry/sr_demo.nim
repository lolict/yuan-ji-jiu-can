# ============================================================
# 统一符号注册中心 · 完整演示
# 
# 把散落在 21 个工具里的所有符号，汇总到一个统一表。
# ============================================================

import strutils
import options
import symbol_registry/registry

echo "═══════════════════════════════════════════"
echo "  统一符号注册中心 · 符号大汇总"
echo "═══════════════════════════════════════════"
echo ""

# 创建注册中心
var reg = new_registry()

# ══════════════════════════════════════════
# 导入所有工具的符号
# ══════════════════════════════════════════

# ── mlll-254 集装箱 ──
reg.register(UnifiedEntry(code: "MFFF", meaning: "254+2集装箱", category: "根基编码", source: ssMlll254, source_name: "mlll-254", synonyms: @["Mlll254", "CONTAINER_254"], description: "254字符+2锚点，根基隐藏"))
reg.register(UnifiedEntry(code: "ANCHOR_SELF", meaning: "高位持有者=1", category: "锚点", source: ssMlll254, source_name: "mlll-254", synonyms: @["SELF", "YANG"], description: "高位锚点"))
reg.register(UnifiedEntry(code: "ANCHOR_OTHER", meaning: "低位持有者=0", category: "锚点", source: ssMlll254, source_name: "mlll-254", synonyms: @["OTHER", "YIN"], description: "低位锚点"))
reg.register(UnifiedEntry(code: "MIRROR_SLOT", meaning: "镜像槽位", category: "根基编码", source: ssMlll254, source_name: "mlll-254", synonyms: @["MIRROR", "DUICHEN"], description: "槽位 a ↔ 255-a"))

# ── color-base 颜色进制 ──
reg.register(UnifiedEntry(code: "PHASE64", meaning: "64进制相位", category: "颜色编码", source: ssColorBase, source_name: "color-base", synonyms: @["BASE64", "PHASE"], description: "自定义64进制"))
reg.register(UnifiedEntry(code: "PHL1", meaning: "L1基元", category: "层级", source: ssColorBase, source_name: "color-base", synonyms: @["LEVEL1", "BASE"], description: "层级1基元本体"))
reg.register(UnifiedEntry(code: "PHL2", meaning: "L2修饰", category: "层级", source: ssColorBase, source_name: "color-base", synonyms: @["LEVEL2", "MODIFY"], description: "层级2修饰部件"))
reg.register(UnifiedEntry(code: "PHL3", meaning: "L3装配", category: "层级", source: ssColorBase, source_name: "color-base", synonyms: @["LEVEL3", "ASSEMBLE"], description: "层级3装配成品"))

# ── bit-container 分比容器 ──
reg.register(UnifiedEntry(code: "PRESENCE", meaning: "存在态", category: "七阶编码", source: ssBitContainer, source_name: "bit-container", synonyms: @["YOUWU"], description: "第0阶：有/无"))
reg.register(UnifiedEntry(code: "WITNESS", meaning: "见证态", category: "七阶编码", source: ssBitContainer, source_name: "bit-container", synonyms: @["JIANZHENG"], description: "第1阶：已见证"))
reg.register(UnifiedEntry(code: "CHARCODE", meaning: "字符码", category: "七阶编码", source: ssBitContainer, source_name: "bit-container", synonyms: @["TYPE", "FUHAO"], description: "第2阶"))
reg.register(UnifiedEntry(code: "OWNER", meaning: "归属码", category: "七阶编码", source: ssBitContainer, source_name: "bit-container", synonyms: @["GUISHU"], description: "第3阶"))

# ── manquanfa 满全法 ──
reg.register(UnifiedEntry(code: "MQF", meaning: "满全法", category: "核心法则", source: ssManquanfa, source_name: "manquanfa", synonyms: @["MANQUANFA", "FULL_LAW"], description: "核心裁判法则"))
reg.register(UnifiedEntry(code: "VERDICT", meaning: "裁判结果", category: "核心法则", source: ssManquanfa, source_name: "manquanfa", synonyms: @["PANJUE"], description: "vrAccept/vrReject/vrPending"))
reg.register(UnifiedEntry(code: "MERIT", meaning: "功德值", category: "核心法则", source: ssManquanfa, source_name: "manquanfa", synonyms: @["GONGDE"], description: "MERIT_ACCEPT=10.0"))
reg.register(UnifiedEntry(code: "POLARITY", meaning: "极性", category: "核心法则", source: ssManquanfa, source_name: "manquanfa", synonyms: @["JIXING"], description: "Yin/Yang/Balanced"))

# ── canjiang 蚕蛹 ──
reg.register(UnifiedEntry(code: "CANJIANG", meaning: "蚕蛹寄生壳", category: "寄生壳", source: ssCanjiang, source_name: "canjiang", synonyms: @["SILKWORM"], description: "反向寄生"))
reg.register(UnifiedEntry(code: "SHED", meaning: "脱壳", category: "寄生壳", source: ssCanjiang, source_name: "canjiang", synonyms: @["TUOQIAO"], description: "剥离外壳"))
reg.register(UnifiedEntry(code: "DRESS", meaning: "穿衣", category: "寄生壳", source: ssCanjiang, source_name: "canjiang", synonyms: @["CHUANYI"], description: "选壳穿衣"))
reg.register(UnifiedEntry(code: "KERNEL", meaning: "内核", category: "寄生壳", source: ssCanjiang, source_name: "canjiang", synonyms: @["NEIHE"], description: "独立于外壳"))

# ── minlian 民联浏览器 ──
reg.register(UnifiedEntry(code: "HAMMING", meaning: "汉明距离", category: "检索系统", source: ssMinlian, source_name: "minlian", synonyms: @["DISTANCE"], description: "64位指纹差异位"))
reg.register(UnifiedEntry(code: "KMEANS", meaning: "态势化聚类", category: "检索系统", source: ssMinlian, source_name: "minlian", synonyms: @["CLUSTER"], description: "K-Means聚类"))
reg.register(UnifiedEntry(code: "WRAPTREE", meaning: "包裹树", category: "检索系统", source: ssMinlian, source_name: "minlian", synonyms: @["TREE"], description: "连接包裹树"))

# ── njsg-swarm ──
reg.register(UnifiedEntry(code: "COORD", meaning: "协调器", category: "分布式系统", source: ssNjsgSwarm, source_name: "njsg-swarm", synonyms: @["COORDINATOR"], description: "任务分配冲突裁决"))
reg.register(UnifiedEntry(code: "AGENT", meaning: "子代理", category: "分布式系统", source: ssNjsgSwarm, source_name: "njsg-swarm", synonyms: @["SUBAGENT"], description: "id/name/polarity/capacity"))
reg.register(UnifiedEntry(code: "CONFLICT", meaning: "冲突裁决", category: "分布式系统", source: ssNjsgSwarm, source_name: "njsg-swarm", synonyms: @["裁决"], description: "优先级高优先"))

# ── radix-star ──
reg.register(UnifiedEntry(code: "RADIX_STAR", meaning: "部首星图", category: "检索系统", source: ssRadixStar, source_name: "radix-star", synonyms: @["STARMAP"], description: "部首为恒星"))
reg.register(UnifiedEntry(code: "STAR_POS", meaning: "星图位置", category: "检索系统", source: ssRadixStar, source_name: "radix-star", synonyms: @["STARPOS"], description: "轨道/兄弟数"))
reg.register(UnifiedEntry(code: "RADICAL_RESONANCE", meaning: "部首共振", category: "检索系统", source: ssRadixStar, source_name: "radix-star", synonyms: @["GONGZHEN"], description: "公约数>1共振"))

# ── property-judge ──
reg.register(UnifiedEntry(code: "PROPERTY", meaning: "产权流", category: "裁判系统", source: ssPropertyJudge, source_name: "property-judge", synonyms: @["CHANQUAN"], description: "主权流转"))
reg.register(UnifiedEntry(code: "INFRINGE", meaning: "侵权", category: "裁判系统", source: ssPropertyJudge, source_name: "property-judge", synonyms: @["QINQUAN"], description: "7种侵权类型"))
reg.register(UnifiedEntry(code: "AUTHORIZE", meaning: "授权", category: "裁判系统", source: ssPropertyJudge, source_name: "property-judge", synonyms: @["SHOUQUAN"], description: "5种授权级别"))

# ── symbol-engine ──
reg.register(UnifiedEntry(code: "SYNONYM", meaning: "近义词", category: "符号引擎", source: ssSymbolEngine, source_name: "symbol-engine", synonyms: @["JINYICI", "ALIAS"], description: "避免冲突"))
reg.register(UnifiedEntry(code: "DYNAMIC", meaning: "动态加载", category: "符号引擎", source: ssSymbolEngine, source_name: "symbol-engine", synonyms: @["RUNTIME"], description: "不编译也能用"))
reg.register(UnifiedEntry(code: "SYMTABLE", meaning: "符号表", category: "符号引擎", source: ssSymbolEngine, source_name: "symbol-engine", synonyms: @["SYMBOL_TABLE"], description: "纯数据文件"))

# ══════════════════════════════════════════
# 演示
# ══════════════════════════════════════════

echo "【统计报告】"
echo registry_report(reg)
echo ""

echo "【按来源查找: manquanfa】"
let mqf_symbols = reg.lookup_source(ssManquanfa)
for e in mqf_symbols:
  echo "  " & e.code & " → " & e.meaning
echo ""

echo "【按类别查找: 七阶编码】"
let layer7 = reg.lookup_category("七阶编码")
for e in layer7:
  echo "  " & e.code & " → " & e.meaning & " (来自 " & e.source_name & ")"
echo ""

echo "【近义词验证: 解析 SELF】"
let r_self = reg.resolve("SELF")
if r_self.isSome():
  echo "  " & entry_detail(r_self.get())
echo ""

echo "【搜索: 锚点】"
let anchor_results = reg.search("锚点")
for e in anchor_results:
  echo "  " & e.code & " → " & e.meaning & " (" & e.source_name & ")"
echo ""

echo "【搜索: 检索】"
let search_results = reg.search("检索")
for e in search_results:
  echo "  " & e.code & " → " & e.meaning & " (" & e.source_name & ")"
echo ""

echo "【含义反查: 近义词】"
let syn_codes = reg.lookup_meaning("近义词")
echo "  所有编码: " & syn_codes.join(", ")
echo ""

echo "═══════════════════════════════════════════"
echo "  " & $reg.total & " 个符号归入同一张表"
echo "  来自 " & $reg.source_count &  " 个工具"
echo "  近义词贯穿全部"
echo "═══════════════════════════════════════════"
