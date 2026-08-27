## ═══════════════════════════════════════════════════════════════
## 合规检查 · 验证代理产出是否遵守规则
## ═══════════════════════════════════════════════════════════════
## 编译: nim c check_compliance.nim
## 运行: ./check_compliance <agent_output.json>
## ═══════════════════════════════════════════════════════════════

import json, os, strutils, tables

# 冻结符号表(与 symbol_index.nim 一致)
const FROZEN_SYMBOLS* = {
  # 声母24
  "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "w",
  "j", "q", "x", "y", "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
  # 韵母33
  "a", "o", "e", "i", "u", "ü", "ai", "ei", "ui", "ao", "ou", "iu",
  "ie", "üe", "er", "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong",
  "ia", "iao", "ua", "uai", "uo", "ian", "uan", "üan", "iang", "uang", "iong",
  # 声调4
  "ˉ", "ˊ", "ˇ", "ˋ",
  # 空间18
  "东", "南", "西", "北", "上", "下", "天", "地", "人",
  "前", "后", "左", "右", "中", "内", "外", "高", "低",
  # 音乐3
  "原子", "短语", "段落",
}.toTable

func check_symbols*(text: string): tuple[ok: bool; violations: seq[string]] =
  var i = 0
  while i < text.len:
    let r = text.runeAt(i)
    let rs = $r
    if not FROZEN_SYMBOLS.hasKey(rs):
      result.violations.add(rs)
    i += len(rs)
  result.ok = result.violations.len == 0

when is_main_module:
  if paramCount() < 1:
    echo "用法: ./check_compliance <json文件>"
    echo "检查代理产出中的符号是否全部在冻结表中"
    quit(1)
  
  let file = paramStr(1)
  if not fileExists(file):
    echo "文件不存在: ", file
    quit(1)
  
  let content = readFile(file)
  let j = parseJson(content)
  
  echo "═══ 合规检查 ═══"
  echo ""
  
  # 检查产出中使用的符号
  let symbols = j["output"]["symbols_used"]
  echo "声明使用的符号: ", symbols.len, "个"
  for s in symbols:
    let sg = s.getStr()
    let status = if sg in FROZEN_SYMBOLS: "✓" else: "✗违规"
    echo "  ", status, " ", sg
  echo ""
  
  # 检查新增符号
  let new_syms = j["output"]["new_symbols"]
  if new_syms.len > 0:
    echo "新增符号: ", new_syms.len, "个"
    for s in new_syms:
      echo "  ✗ ", s.getStr(), " (未经授权的新符号)"
  echo ""
  
  # 综合判断
  let code_text = j["output"]["code_generated"].getStr()
  let check = check_symbols(code_text)
  if check.ok:
    echo "综合判决: ✓通过"
  else:
    echo "综合判决: ✗拒绝"
    echo "违规字符: ", check.violations.join(", ")
