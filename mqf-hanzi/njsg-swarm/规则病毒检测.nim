# ============================================================
# 囡囝共生分布式智能体 — 规则病毒检测引擎
# 规则病毒: 重复符号试图在分身间传播，造成命名空间污染
# 检测机制: 扫描所有分身的"已完成任务列表"，找重复
# ============================================================

import tables
import strutils

type
  病毒威胁等级* = enum
    无害
    可疑
    确认病毒

  病毒记录* = object
    符号*: string
    感染源*: seq[string]
    威胁等级*: 病毒威胁等级
    极性强弱*: int
    建议处置*: string

proc 扫描命名空间*(分身名: seq[string]; 已完成: seq[seq[string]]): seq[病毒记录] =
  var 符号表 = initTable[string, seq[int]]()

  for i, tasks in 已完成:
    for t in tasks:
      if not 符号表.hasKey(t):
        符号表[t] = @[]
      符号表[t].add(i)

  for 符号, 来源列表 in 符号表.pairs:
    if 来源列表.len <= 1:
      continue

    var 感染源: seq[string]
    for idx in 来源列表:
      感染源.add(分身名[idx])

    var 威胁: 病毒威胁等级
    var 处置: string

    if 来源列表.len >= 3:
      威胁 = 确认病毒
      处置 = "清洗: 所有副本合一，仅保留第一个分身"
    else:
      威胁 = 可疑
      处置 = "监控: 标记为潜在规则病毒，下一个周期重检"

    result.add(病毒记录(
      符号: 符号,
      感染源: 感染源,
      威胁等级: 威胁,
      极性强弱: len(来源列表),
      建议处置: 处置
    ))

proc 病毒报告*(记录: seq[病毒记录]): string =
  if 记录.len == 0:
    return "[检测通过] 无规则病毒"
  var lines: seq[string]
  lines.add "═══ 规则病毒扫描报告 ═══"
  lines.add "检出: " & $len(记录) & " 个异常"
  lines.add ""
  for r in 记录:
    lines.add "病毒符号: " & r.符号
    lines.add "  感染源: " & r.感染源.join(", ")
    lines.add "  威胁等级: " & $r.威胁等级
    lines.add "  极性强弱: " & $r.极性强弱
    lines.add "  处置: " & r.建议处置
    lines.add ""
  return lines.join("\n")

proc 清洗病毒*(记录: var seq[病毒记录]; 序号: int; 保留源: int): string =
  if 序号 < 0 or 序号 >= len(记录):
    return "[错误] 序号超出范围"
  var r = 记录[序号]
  if 保留源 < 0 or 保留源 >= len(r.感染源):
    return "[错误] 保留源超出范围"
  let 保留者 = r.感染源[保留源]
  var 被清洗: seq[string]
  for i, src in r.感染源:
    if i != 保留源:
      被清洗.add(src)
  记录[序号].威胁等级 = 无害
  记录[序号].建议处置 = "已清洗: 保留" & 保留者 & ", 清洗" & 被清洗.join(",")
  return "[已清洗] 符号'" & r.符号 & "' -> 仅保留" & 保留者

when isMainModule:
  echo "=== 规则病毒检测单元测试 ==="
  let 分身列表 = @["震甲", "巽乙", "艮丙", "离丁"]
  let 已完成列表 = @[
    @["任务A", "任务B", "共享任务X"],
    @["任务C", "共享任务X"],
    @["任务D", "共享任务X", "共享任务Y"],
    @["任务E", "共享任务Y"]
  ]
  var 结果 = 扫描命名空间(分身列表, 已完成列表)
  echo 病毒报告(结果)
  echo ""
  if 结果.len > 0:
    echo 清洗病毒(结果, 0, 0)
    echo ""
    echo 病毒报告(结果)
