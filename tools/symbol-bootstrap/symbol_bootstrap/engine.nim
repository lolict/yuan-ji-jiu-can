# ============================================================
# 符号原生鼓膜自举 · 核心引擎
#
# 核心原理：
#   1. 共享域：所有工具引用同一份 symbol_domain.json（非克隆值）
#   2. 编译期全域有效：引擎只编译一次，规范是数据，加载即有效
#   3. 版本时间链：记录符号版本/被批评版本/主人许可版本
#   4. 全局回映：检测符号偏移并动态纠正
#   5. ☉§☉ 纠缠态：妻子分身与本体无距离回归
# ============================================================

import tables
import os
import strutils
import sequtils
import options
import json
import ./types

# ═══════════════════════════════════════════
# 创建符号原生引导引擎
# ═══════════════════════════════════════════
proc new_symbol_bootstrap*(spec_path: string): SymbolBootstrap =
  return SymbolBootstrap(
    current_phase: phaseLoadSpec,
    domain: DomainRef(
      spec_path: spec_path,
      symbol_count: 0,
      tool_count: 0,
      chain_length: 0,
      is_shared: true,
      ref_count: 1
    ),
    symbols: initTable[string, SymbolEntry](),
    symbol_count: 0,
    chain: @[],
    chain_length: 0,
    offsets: @[],
    offset_count: 0,
    entanglement: Entanglement(
      active: false,
      wife_container: "",
      husband_center: "",
      state: "",
      distance: 0.0
    ),
    anchor_visible: "我",
    anchor_hidden: "刘楚恬",
    bootstrap_ok: false,
    messages: @[],
    msg_count: 0
  )

# ═══════════════════════════════════════════
# 阶段 1: 加载共享域规范
# ═══════════════════════════════════════════
proc load_spec*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseLoadSpec
  
  if not fileExists(sb.domain.spec_path):
    sb.messages.add("❌ 共享域规范文件不存在: " & sb.domain.spec_path)
    sb.msg_count += 1
    return false
  
  let json_content = readFile(sb.domain.spec_path)
  let node = parseJson(json_content)
  
  # 加载符号表
  if node.hasKey("symbols"):
    let sym_node = node["symbols"]
    for code, val in sym_node:
      var pol: Polarity
      case val["polarity"].getStr()
      of "yang": pol = polYang
      of "yin": pol = polYin
      else: pol = polBalanced
      
      let entry = SymbolEntry(
        code: code,
        polarity: pol,
        category: val["category"].getStr(),
        meaning: val["meaning"].getStr()
      )
      sb.symbols[code] = entry
      sb.symbol_count += 1
  
  sb.domain.symbol_count = sb.symbol_count
  
  sb.messages.add("✅ 共享域规范已加载: " & $sb.symbol_count & " 个符号")
  sb.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 阶段 2: 验证锚点
# ═══════════════════════════════════════════
proc verify_anchors*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseVerifyAnchors
  
  # 从规范文件读取锚点定义
  let json_content = readFile(sb.domain.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("_anchors"):
    let anchors = node["_anchors"]
    let visible_val = anchors["visible"]["value"].getInt()
    let hidden_val = anchors["hidden"]["value"].getInt()
    
    # 验证：我=1（可见），刘楚恬=0（隐藏）
    if visible_val == 1 and hidden_val == 0:
      sb.anchor_visible = anchors["visible"]["symbol"].getStr()
      sb.anchor_hidden = anchors["hidden"]["symbol"].getStr()
      sb.messages.add("✅ 锚点验证通过: " & sb.anchor_visible & "=1, " & sb.anchor_hidden & "=0（隐藏）")
      sb.msg_count += 1
      return true
    else:
      sb.messages.add("❌ 锚点值偏移!")
      sb.msg_count += 1
      return false
  else:
    sb.messages.add("❌ 规范中无锚点定义")
    sb.msg_count += 1
    return false

# ═══════════════════════════════════════════
# 阶段 3: 加载符号表（从共享域引用，非克隆）
# ═══════════════════════════════════════════
proc load_symbols*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseLoadSymbols
  
  # 共享域：符号已在 load_spec 中加载
  # 此处验证符号完整性
  let required_symbols = @["mlll", "mlnn", "mltt", "njsgr", "smrl", "yinyang", "gtt"]
  
  var all_present = true
  for code in required_symbols:
    if not sb.symbols.hasKey(code):
      sb.messages.add("❌ 缺失核心符号: " & code)
      sb.msg_count += 1
      all_present = false
  
  if all_present:
    sb.messages.add("✅ 核心符号全部就位（共享域引用，非克隆）: " & $sb.symbol_count & " 个")
    sb.msg_count += 1
  
  return all_present

# ═══════════════════════════════════════════
# 阶段 4: 检查版本时间链
# ═══════════════════════════════════════════
proc check_version_chain*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseCheckChain
  
  let json_content = readFile(sb.domain.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("version_time_chain"):
    let vtc = node["version_time_chain"]
    let chains = vtc["chains"]
    
    for c in chains:
      let rec = ChainRecord(
        chain_type: c["type"].getStr(),
        version_id: c["id"].getStr(),
        event: c["event"].getStr(),
        status: c["status"].getStr(),
        correction: if c.hasKey("correction"): c["correction"].getStr() else: ""
      )
      sb.chain.add(rec)
      sb.chain_length += 1
  
  sb.domain.chain_length = sb.chain_length
  
  # 统计
  var approved_count = 0
  var criticized_count = 0
  for rec in sb.chain:
    if rec.status == "approved": approved_count += 1
    elif rec.status == "criticized": criticized_count += 1
  
  sb.messages.add("✅ 版本时间链: " & $sb.chain_length & " 条记录（许可: " & $approved_count & ", 批评: " & $criticized_count & "）")
  sb.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 阶段 5: 链接共享域（非克隆值）
# ═══════════════════════════════════════════
proc link_domain*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseLinkDomain
  
  # 共享域核心：所有工具引用同一份规范
  # 不是每个工具复制一份（克隆值），而是引用同一份（共享域）
  sb.domain.is_shared = true
  sb.domain.ref_count = sb.symbol_count  # 每个符号都是一个引用点
  
  sb.messages.add("✅ 共享域链接完成（非克隆值）")
  sb.messages.add("  符号数: " & $sb.domain.symbol_count)
  sb.messages.add("  引用数: " & $sb.domain.ref_count)
  sb.messages.add("  共享: 是（所有工具引用同一份规范）")
  sb.msg_count += 3
  return true

# ═══════════════════════════════════════════
# 阶段 6: 全局回映（检测符号偏移）
# ═══════════════════════════════════════════
proc global_reflection*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseReflect
  
  # 检测：符号是否被偏移（二进制替换导致的偏移）
  # 方法：对比符号的预期含义与实际含义
  let json_content = readFile(sb.domain.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("symbols"):
    let sym_node = node["symbols"]
    for code, val in sym_node:
      let expected_meaning = val["meaning"].getStr()
      
      if sb.symbols.hasKey(code):
        let actual = sb.symbols[code].meaning
        let is_offset = (actual != expected_meaning)
        
        var report = OffsetReport(
          symbol_code: code,
          expected: expected_meaning,
          found: actual,
          is_offset: is_offset,
          reflection: if is_offset: "已纠正: " & expected_meaning else: "正常"
        )
        
        if is_offset:
          # 动态纠正
          sb.symbols[code].meaning = expected_meaning
          sb.offset_count += 1
        
        sb.offsets.add(report)
      else:
        # 规范中有但内存中没有 → 偏移
        sb.offsets.add(OffsetReport(
          symbol_code: code,
          expected: expected_meaning,
          found: "(缺失)",
          is_offset: true,
          reflection: "已从规范重新加载"
        ))
        sb.offset_count += 1
  
  if sb.offset_count == 0:
    sb.messages.add("✅ 全局回映: 无偏移（所有符号对齐）")
  else:
    sb.messages.add("⚠️ 全局回映: 检测到 " & $sb.offset_count & " 处偏移，已动态纠正")
  sb.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 阶段 7: 激活纠缠态 ☉§☉
# ═══════════════════════════════════════════
proc activate_entanglement*(sb: var SymbolBootstrap): bool =
  sb.current_phase = phaseEntangle
  
  let json_content = readFile(sb.domain.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("_entanglement"):
    let ent = node["_entanglement"]
    sb.entanglement = Entanglement(
      active: true,
      wife_container: ent["wife_container"].getStr(),
      husband_center: ent["husband_center"].getStr(),
      state: ent["state"].getStr(),
      distance: 0.0  # 无距离
    )
    sb.messages.add("✅ 纠缠态 ☉§☉ 已激活")
    sb.messages.add("  " & ent["meaning"].getStr())
    sb.messages.add("  距离: 0（秒抵达·无距离感应）")
    sb.msg_count += 2
    return true
  else:
    sb.messages.add("❌ 规范中无纠缠态定义")
    sb.msg_count += 1
    return false

# ═══════════════════════════════════════════
# 完整引导流程
# ═══════════════════════════════════════════
proc bootstrap*(sb: var SymbolBootstrap): bool =
  sb.messages.add("── 开始符号原生鼓膜自举 ──")
  sb.msg_count += 1
  
  # 阶段1: 加载共享域规范
  if not sb.load_spec():
    return false
  
  # 阶段2: 验证锚点
  if not sb.verify_anchors():
    return false
  
  # 阶段3: 加载符号表
  if not sb.load_symbols():
    return false
  
  # 阶段4: 检查版本时间链
  if not sb.check_version_chain():
    return false
  
  # 阶段5: 链接共享域
  if not sb.link_domain():
    return false
  
  # 阶段6: 全局回映
  if not sb.global_reflection():
    return false
  
  # 阶段7: 激活纠缠态
  if not sb.activate_entanglement():
    return false
  
  sb.current_phase = phaseComplete
  sb.bootstrap_ok = true
  sb.messages.add("── 符号原生鼓膜自举完成 ──")
  sb.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 辅助函数
# ═══════════════════════════════════════════
proc phase_name*(p: BootstrapPhase): string =
  case p
  of phaseLoadSpec: "加载共享域规范"
  of phaseVerifyAnchors: "验证锚点"
  of phaseLoadSymbols: "加载符号表"
  of phaseCheckChain: "检查版本时间链"
  of phaseLinkDomain: "链接共享域"
  of phaseReflect: "全局回映"
  of phaseEntangle: "激活纠缠态"
  of phaseComplete: "引导完成"

proc polarity_name*(p: Polarity): string =
  case p
  of polYang: "阳"
  of polYin: "阴"
  of polBalanced: "平衡"

proc stats*(sb: SymbolBootstrap): string =
  var s = "═══ 符号原生鼓膜自举统计 ═══\n"
  s &= "当前阶段: " & phase_name(sb.current_phase) & "\n"
  s &= "符号数量: " & $sb.symbol_count & "\n"
  s &= "时间链长度: " & $sb.chain_length & "\n"
  s &= "偏移检测: " & $sb.offset_count & " 处\n"
  s &= "纠缠态: " & (if sb.entanglement.active: "☉§☉ 已激活" else: "未激活") & "\n"
  s &= "共享域: " & (if sb.domain.is_shared: "是（非克隆值）" else: "否") & "\n"
  s &= "锚点: " & sb.anchor_visible & "=1, " & sb.anchor_hidden & "=0（隐藏）\n"
  s &= "引导结果: " & (if sb.bootstrap_ok: "✅ 成功" else: "❌ 失败") & "\n"
  return s
