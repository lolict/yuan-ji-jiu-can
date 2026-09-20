# ============================================================
# 满全法 — 全栈统一调度器 (Unified Pipeline Orchestrator)
# 单入口串联: 编译 → 容器注册 → 时钟驱动 → 分身协作 → 裁决输出
# 跨模块: 整合 njsg-swarm + 时钟动作 + 容器本体论
# ============================================================

import strutils
import tables
import algorithm

# ============================================================
# 跨模块统一类型
# ============================================================

type
  极性* = enum
    阴 = 0
    阳 = 1

  容器* = object
    名字*: string
    相位*: int
    层*: int
    极性*: 极性
    物质*: string
    时间戳*: int
    活性*: bool

  生命周期阶段* = enum
    虚无, 诞生, 生长, 沉淀, 蜕生

  动作* = object
    名字*: string
    极性*: 极性
    阶段*: 生命周期阶段
    龄*: int
    寿*: int
    意志*: int
    记忆*: string
    产出*: string

  分身* = object
    名字*: string
    极性*: 极性
    主权*: uint64
    已完成*: seq[string]
    活性*: bool

  全栈调度器* = object
    轮次*: int
    容器图谱*: seq[容器]
    动作时钟*: seq[动作]
    分身群*: seq[分身]
    记忆*: seq[string]

proc 极性标签*(p: 极性): string =
  if p == 阳: "阳" else: "阴"

proc 阶段标签*(s: 生命周期阶段): string =
  case s
  of 虚无: "虚"
  of 诞生: "诞"
  of 生长: "长"
  of 沉淀: "沉"
  of 蜕生: "蜕"

# ============================================================
# 阶段1: 编译层
# ============================================================

proc 编译层运行*(pipe: var 全栈调度器): string =
  var lines: seq[string]
  lines.add "[编译] 域规则加载 → 域索引"
  lines.add "[编译] 字根-声母-极性映射激活"
  lines.add "[编译] MQF.24.7指纹生成"
  pipe.容器图谱 = @[
    容器(名字: "天元", 相位: 0, 层: 0, 极性: 阳, 物质: "元起", 时间戳: 0, 活性: true),
    容器(名字: "水元", 相位: 1, 层: 1, 极性: 阴, 物质: "水润", 时间戳: 1, 活性: true),
    容器(名字: "火元", 相位: 2, 层: 1, 极性: 阳, 物质: "火燃", 时间戳: 2, 活性: true),
    容器(名字: "山元", 相位: 4, 层: 2, 极性: 阴, 物质: "山稳", 时间戳: 3, 活性: true),
    容器(名字: "泽元", 相位: 5, 层: 2, 极性: 阳, 物质: "泽聚", 时间戳: 4, 活性: true),
    容器(名字: "人元", 相位: 8, 层: 3, 极性: 阴, 物质: "人和", 时间戳: 5, 活性: true),
  ]
  lines.add "[编译] 生成 " & $len(pipe.容器图谱) & " 个基础容器"
  return lines.join("\n")

# ============================================================
# 阶段2: 容器层
# ============================================================

proc 容器层运行*(pipe: var 全栈调度器): string =
  var lines: seq[string]
  lines.add "[容器] 容器=物质=时间 本体编码"
  for c in pipe.容器图谱:
    lines.add "  " & c.名字 & "=(" & $c.相位 & "相/" & $c.层 & "层/" & 极性标签(c.极性) & ")"
  lines.add "[容器] 纤维丛: 层0底点(天元) 层1底点(水元,火元) 层2底点(山元)"
  return lines.join("\n")

# ============================================================
# 阶段3: 时钟层
# ============================================================

proc 阶段寿命*(a: 动作): int =
  case a.阶段
  of 诞生: 2
  of 生长: 3
  of 沉淀: 2
  of 蜕生: 1
  of 虚无: 0

proc 下一阶段计算*(a: 动作): 生命周期阶段 =
  case a.阶段
  of 虚无: 诞生
  of 诞生: 生长
  of 生长: 沉淀
  of 沉淀: 蜕生
  of 蜕生: 诞生

proc 诞生动作*(名字: string; 极性: 极性; 意志: int; 记忆: string): 动作 =
  动作(名字:名字, 极性:极性, 阶段:诞生, 龄:0, 寿:2, 意志:意志, 记忆:记忆, 产出:"")

proc 时钟层运行*(pipe: var 全栈调度器; 滴答: int): string =
  var lines: seq[string]

  if len(pipe.动作时钟) == 0:
    for c in pipe.容器图谱:
      let 意志 = 80 - c.层 * 15 + (if c.极性 == 阳: 5 else: 0)
      pipe.动作时钟.add(诞生动作(c.名字, c.极性, 意志, c.物质))
    lines.add "[时钟] 初始化 " & $len(pipe.动作时钟) & " 个动作"

  var 蜕生事件: seq[string]
  for i in 0 ..< len(pipe.动作时钟):
    inc pipe.动作时钟[i].龄
    let a = pipe.动作时钟[i]

    if a.龄 >= a.寿:
      if a.阶段 == 蜕生:
        let 旧记忆 = a.记忆
        pipe.动作时钟[i].阶段 = 诞生
        pipe.动作时钟[i].龄 = 0
        pipe.动作时钟[i].寿 = 阶段寿命(pipe.动作时钟[i])
        pipe.动作时钟[i].记忆 = 旧记忆
        蜕生事件.add(a.名字 & ".◁")
      else:
        pipe.动作时钟[i].阶段 = 下一阶段计算(a)
        pipe.动作时钟[i].龄 = 0
        pipe.动作时钟[i].寿 = 阶段寿命(pipe.动作时钟[i])

  lines.add "[时钟] 滴答" & $滴答 & " | 活跃: " & $len(pipe.动作时钟)
  if 蜕生事件.len > 0:
    lines.add "[时钟] 蜕生: " & 蜕生事件.join(" ")
    pipe.记忆.add("T" & $滴答 & ":蜕生=" & 蜕生事件.join(","))

  proc cmp(x, y: 动作): int =
    if x.意志 > y.意志: -1 elif x.意志 < y.意志: 1 else: 0
  pipe.动作时钟.sort(cmp)

  for a in pipe.动作时钟:
    let 尾 = if a.记忆.len > 6: a.记忆[^6..^1] else: a.记忆
    lines.add "  " & a.名字 & "|" & 极性标签(a.极性) & "|" & 阶段标签(a.阶段) & "|意" & $a.意志 & "|" & 尾

  return lines.join("\n")

# ============================================================
# 阶段4: Swarm层
# ============================================================

proc 创建分身*(c: 容器): 分身 =
  let 主权 = (uint64(c.相位) * 72057594037927936'u64) + (uint64(c.层) * 281474976710656'u64) + (uint64(ord(c.极性)) * 2199023255552'u64)
  result = 分身(名字: c.名字, 极性: c.极性, 主权: 主权, 已完成: @["BOOT." & c.名字], 活性: true)

proc Swarm层运行*(pipe: var 全栈调度器; 轮次: int): string =
  var lines: seq[string]

  if len(pipe.分身群) == 0:
    for c in pipe.容器图谱:
      pipe.分身群.add(创建分身(c))
    lines.add "[Swarm] 生成 " & $len(pipe.分身群) & " 分身"

  let taskNames = @["NS_SCAN", "POL_JUDGE", "SYM_CLEAN", "HASH_VER", "MEM_AUDIT"]
  let targetPol = if 轮次 mod 2 == 0: 阳 else: 阴

  var 分配数 = 0
  for i in 0 ..< len(pipe.分身群):
    let gap = abs(ord(pipe.分身群[i].极性) - ord(targetPol))
    if gap == 1:
      let task = taskNames[(轮次 + i) mod len(taskNames)]
      pipe.分身群[i].已完成.add(task)
      inc 分配数

  lines.add "[Swarm] 轮" & $轮次 & " 极性=" & 极性标签(targetPol) & " 分配=" & $分配数

  proc cmp(x, y: 分身): int =
    if x.主权 > y.主权: -1 elif x.主权 < y.主权: 1 else: 0
  pipe.分身群.sort(cmp)

  for s in pipe.分身群:
    lines.add "  " & s.名字 & "|" & 极性标签(s.极性) & "|主=" & $s.主权 & "|" & $len(s.已完成) & "任"

  return lines.join("\n")

# ============================================================
# 阶段5: 裁决层
# ============================================================

proc 裁决层运行*(pipe: var 全栈调度器): string =
  var lines: seq[string]
  lines.add "[裁决] 病毒扫描 + 极性冲突"

  var 符号表 = initTable[string, seq[int]]()
  for i, s in pipe.分身群:
    for t in s.已完成:
      if not 符号表.hasKey(t):
        符号表[t] = @[]
      符号表[t].add(i)

  var 病毒数 = 0
  for 符号, 来源 in 符号表.pairs:
    if 来源.len > 1:
      inc 病毒数
      var 感染源: seq[string]
      for idx in 来源: 感染源.add(pipe.分身群[idx].名字)
      lines.add "  病毒: '" & 符号 & "' 来自[" & 感染源.join(",") & "]"
      # 清洗: 保留主权最高
      var 保留 = 0
      var 最高主权: uint64 = 0'u64
      for idx in 来源:
        if pipe.分身群[idx].主权 > 最高主权:
          最高主权 = pipe.分身群[idx].主权
          保留 = idx
      for idx in 来源:
        if idx != 保留:
          var newDone: seq[string]
          for t in pipe.分身群[idx].已完成:
            if t != 符号: newDone.add(t)
          pipe.分身群[idx].已完成 = newDone
      lines.add "    → 保留" & pipe.分身群[保留].名字

  if 病毒数 == 0:
    lines.add "  无病毒"

  var 冲突数 = 0
  for i in 0 ..< len(pipe.分身群):
    for j in i+1 ..< len(pipe.分身群):
      if pipe.分身群[i].极性 == pipe.分身群[j].极性:
        for t in pipe.分身群[i].已完成:
          if t in pipe.分身群[j].已完成:
            let isBootI = t == "BOOT." & pipe.分身群[i].名字
            let isBootJ = t == "BOOT." & pipe.分身群[j].名字
            if not isBootI and not isBootJ:
              lines.add "  冲突: " & pipe.分身群[i].名字 & " vs " & pipe.分身群[j].名字 & " 竞'" & t & "'"
              if pipe.分身群[i].主权 >= pipe.分身群[j].主权:
                var nd: seq[string]
                for tt in pipe.分身群[j].已完成:
                  if tt != t: nd.add(tt)
                pipe.分身群[j].已完成 = nd
                lines.add "    → 驱逐" & pipe.分身群[j].名字
              else:
                var nd: seq[string]
                for tt in pipe.分身群[i].已完成:
                  if tt != t: nd.add(tt)
                pipe.分身群[i].已完成 = nd
                lines.add "    → 驱逐" & pipe.分身群[i].名字
              inc 冲突数

  if 冲突数 == 0:
    lines.add "  无冲突"

  return lines.join("\n")

# ============================================================
# 全栈编排
# ============================================================

proc 创建调度器*(): 全栈调度器 =
  全栈调度器(轮次: 0, 容器图谱: @[], 动作时钟: @[], 分身群: @[], 记忆: @[])

proc 运行一轮*(pipe: var 全栈调度器): string =
  inc pipe.轮次
  let 前一记忆数 = len(pipe.记忆)

  var report: seq[string]
  report.add "══ 全栈轮次" & $pipe.轮次 & " ══"

  if pipe.轮次 == 1:
    report.add 编译层运行(pipe)
    report.add ""
    report.add 容器层运行(pipe)
    report.add ""

  report.add 时钟层运行(pipe, pipe.轮次)
  report.add ""
  report.add Swarm层运行(pipe, pipe.轮次)
  report.add ""
  report.add 裁决层运行(pipe)
  report.add ""

  if len(pipe.记忆) > 前一记忆数:
    report.add "[记忆] " & pipe.记忆[^1]

  return report.join("\n")

proc 最终报告*(pipe: 全栈调度器): string =
  var lines: seq[string]
  lines.add ""
  lines.add "══════════════════════════════════════"
  lines.add "全栈调度完毕 | 轮次=" & $pipe.轮次
  lines.add "容器:" & $len(pipe.容器图谱) & " 动作:" & $len(pipe.动作时钟) & " 分身:" & $len(pipe.分身群)
  lines.add ""
  lines.add "── 容器 ──"
  for c in pipe.容器图谱:
    lines.add "  " & c.名字 & "=(" & $c.相位 & "相/" & $c.层 & "层/" & 极性标签(c.极性) & ")"
  lines.add ""
  lines.add "── 动作 ──"
  for a in pipe.动作时钟:
    lines.add "  " & a.名字 & "|" & 阶段标签(a.阶段) & "|意" & $a.意志
  lines.add ""
  lines.add "── 分身 ──"
  for s in pipe.分身群:
    lines.add "  " & s.名字 & "|" & 极性标签(s.极性) & "|" & $len(s.已完成) & "任"
  lines.add ""
  if len(pipe.记忆) > 0:
    lines.add "── 记忆 ──"
    for m in pipe.记忆: lines.add "  " & m
  lines.add "══════════════════════════════════════"
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════════"
  echo "  满全法 全栈统一调度器"
  echo "  编译 → 容器 → 时钟 → Swarm → 裁决"
  echo "═══════════════════════════════════════════"
  echo ""

  var pipe = 创建调度器()
  for i in 1..3:
    echo pipe.运行一轮()
    echo ""

  echo pipe.最终报告()
