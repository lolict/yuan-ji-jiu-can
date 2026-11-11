# 表格编程引擎 · 第五部分 · CLI 命令行接口
# 框架来源：TABLE_PROGRAMMING_SPEC.md + RUNTIME_LOOP_ENGINE.md

import os
import strutils
import times
import 地址管理器
import 表格引擎
import 表格引擎_二
import 表格引擎_三
import 表格引擎_四

# =============================================================================
# CLI 命令定义
# =============================================================================

type
  CliCommand* = enum
    CmdHelp
    CmdInit
    CmdRun
    CmdInject
    CmdShow
    CmdSave
    CmdLoad
    CmdEval
    CmdFork
    CmdExport
    CmdClean

  CliConfig* = object
    command*: CliCommand
    args*: seq[string]
    flags*: seq[string]
    workDir*: string
    chainFile*: string
    metaFile*: string
    verbose*: bool
    maxTicks*: int
    frequency*: float

# =============================================================================
# 命令行解析
# =============================================================================

const HelpText = """
表格编程引擎 CLI v1.0

用法：table-cli <command> [options] [args]

命令:
  init              初始化新项目 (创建空表格和地址空间)
  run [file]        运行 Tick 循环 (默认加载当前目录的 .chain 文件)
  inject <question> 向表格注入新问题
  show              显示当前表格状态
  save [file]       保存表格到文件
  load <file>       从文件加载表格
  eval              执行 Surplus 评估
  fork              检查信念分叉
  export <format>   导出为指定格式 (chain/json/markdown)
  clean             清理临时文件和日志
  help              显示此帮助信息

选项:
  -v, --verbose     详细输出模式
  -t, --ticks <n>   设置 Tick 次数 (默认：10)
  -f, --freq <hz>   设置 Tick 频率 (默认：1.0Hz)
  -o, --out <file>  输出文件路径
  -w, --work <dir>  工作目录 (默认：当前目录)
  -h, --help        显示帮助

示例:
  table-cli init
  table-cli run -t 100 -f 2.0
  table-cli inject "我当前站入有还是无的状态？"
  table-cli show -v
  table-cli export json -o output.json
"""

proc parseArgs*(args: seq[string]): CliConfig =
  result.command = CmdHelp
  result.args = @[]
  result.flags = @[]
  result.workDir = getCurrentDir()
  result.chainFile = ""
  result.metaFile = ""
  result.verbose = false
  result.maxTicks = 10
  result.frequency = 1.0
  
  if args.len == 0:
    return
  
  # 解析命令
  let cmdStr = args[0].toLower()
  case cmdStr
  of "init": result.command = CmdInit
  of "run": result.command = CmdRun
  of "inject": result.command = CmdInject
  of "show": result.command = CmdShow
  of "save": result.command = CmdSave
  of "load": result.command = CmdLoad
  of "eval": result.command = CmdEval
  of "fork": result.command = CmdFork
  of "export": result.command = CmdExport
  of "clean": result.command = CmdClean
  of "help", "-h", "--help": result.command = CmdHelp
  else:
    echo "[ERROR] 未知命令：", cmdStr
    echo "使用 'table-cli help' 查看可用命令"
    result.command = CmdHelp
    return
  
  # 解析选项和参数
  var i = 1
  while i < args.len:
    let arg = args[i]
    
    if arg.startsWith("-"):
      case arg
      of "-v", "--verbose": result.verbose = true
      of "-t", "--ticks":
        if i + 1 < args.len:
          inc i
          result.maxTicks = parseInt(args[i])
      of "-f", "--freq":
        if i + 1 < args.len:
          inc i
          result.frequency = parseFloat(args[i])
      of "-o", "--out":
        if i + 1 < args.len:
          inc i
          result.chainFile = args[i]
      of "-w", "--work":
        if i + 1 < args.len:
          inc i
          result.workDir = args[i]
      of "-h", "--help": result.command = CmdHelp
      else:
        result.flags.add(arg)
    else:
      result.args.add(arg)
    
    inc i

# =============================================================================
# CLI 命令实现
# =============================================================================

proc cmdHelp*() =
  echo HelpText

proc cmdInit*(config: CliConfig) =
  echo "[INIT] 初始化新项目..."
  
  # 创建空表格
  var table = newTable(254)
  echo "创建表格：254 行"
  
  # 创建地址空间
  var space = newAddressSpace()
  echo "创建地址空间：1024 槽"
  
  # 保存文件
  let chainFile = config.workDir / "default.chain"
  table.saveChain(chainFile)
  
  let addrFile = config.workDir / "address_space.txt"
  space.saveAddressSpace(addrFile)
  
  # 保存元数据
  var f = open(config.workDir / "meta.txt", fmWrite)
  defer: f.close()
  f.writeLine("# PROJECT_METADATA v1.0")
  f.writeLine("created=", now().format("yyyy-MM-dd HH:mm:ss"))
  f.writeLine("version=1.0")
  
  echo "[INIT] 项目初始化完成于：", config.workDir

proc cmdRun*(config: CliConfig) =
  echo "[RUN] 启动 Tick 循环..."
  echo "  工作目录：", config.workDir
  echo "  Tick 次数：", config.maxTicks
  echo "  频率：", config.frequency, "Hz"
  
  # 加载运行时
  let prefix = config.workDir / "default"
  var engine = loadRuntime(prefix)
  engine.tickFrequency = config.frequency
  engine.maxTicks = uint64(config.maxTicks)
  
  # 注册回调
  var tickCount = 0
  proc onTick(e: var RuntimeEngine) {.gcsafe.} =
    inc tickCount
    if config.verbose and tickCount mod 5 == 0:
      echo "  [TICK ", tickCount, "] surplus=", e.surplus
  
  engine.registerTick(onTick)
  
  # 执行循环
  while engine.tickCount < engine.maxTicks:
    engine.executeTick()
    if engine.paused:
      echo "[PAUSE] 引擎已暂停 (危机模式)"
      break
  
  # 保存状态
  engine.saveRuntime(prefix)
  
  echo "[RUN] 循环完成，执行 ", engine.tickCount, " 个 Tick"
  echo "  最终 surplus=", engine.surplus
  echo "  模式=", engine.tickMode

proc cmdInject*(config: CliConfig) =
  if config.args.len == 0:
    echo "[ERROR] 请提供要注入的问题"
    echo "用法：table-cli inject <问题描述>"
    return
  
  let question = config.args.join(" ")
  echo "[INJECT] 注入问题：", question
  
  # 加载表格
  let prefix = config.workDir / "default"
  var engine = loadRuntime(prefix)
  
  # 注入数据
  let rowIdx = engine.onInject(Symbol, uint64(engine.tickCount), question)
  if rowIdx >= 0:
    echo "[OK] 已注入到行 ", rowIdx
    
    # 自动执行自问答
    engine.autoSelfQA()
    
    # 保存
    engine.saveRuntime(prefix)
    echo "[SAVE] 状态已保存"
  else:
    echo "[ERROR] 注入失败 (表格已满或地址空间不足)"

proc cmdShow*(config: CliConfig) =
  echo "[SHOW] 表格状态"
  echo "=" . repeat(60)
  
  # 加载表格
  let chainFile = config.workDir / "default.chain"
  let table = loadChain(chainFile)
  
  echo "表格大小：", table.maxRows, " 行"
  
  # 显示非空行
  var activeRows = 0
  for i in 0..<table.maxRows:
    if table.rows[i].question.len > 0:
      inc activeRows
      
      if config.verbose:
        echo "\n--- 行 ", i + 1, " ---"
        echo "C2 (提问): ", table.rows[i].question
        echo "C3 (视角): ", table.rows[i].viewpoint
        echo "C4 (所在态): ", table.rows[i].state
        echo "C7 (管辖): ", table.rows[i].jurisdiction
        echo "C13 (博弈): ", table.rows[i].gameIdentity
        echo "C14 (回答): ", table.rows[i].answer
        echo "C15 (信念): ", table.rows[i].belief
  
  echo "\n", "=" . repeat(60)
  echo "活跃行数：", activeRows, "/", table.maxRows

proc cmdEval*(config: CliConfig) =
  echo "[EVAL] 执行 Surplus 评估..."
  
  # 加载表格
  let prefix = config.workDir / "default"
  var engine = loadRuntime(prefix)
  
  # 执行评估
  let obs = engine.injection.table.observe()
  let report = obs.evaluateSurplus()
  
  echo "\n=== Surplus 评估报告 ==="
  echo "总 Surplus: ", report.surplus
  echo "\n规则详情:"
  for i, rule in report.rules:
    echo "  ", i+1, ". ", rule.rule, ": ", rule.score
  echo "\n危机状态：", if report.crisis: "是" else: "否"
  echo "建议操作：", surplusStatus(report.surplus)
  echo "========================"

proc cmdFork*(config: CliConfig) =
  echo "[FORK] 检查信念分叉..."
  
  # 加载表格
  let prefix = config.workDir / "default"
  var engine = loadRuntime(prefix)
  
  var forkCount = 0
  for i in 0..<engine.injection.table.maxRows:
    if engine.injection.table.rows[i].question.len == 0:
      continue
    
    let fork = engine.injection.table.checkFork(i)
    if fork.hasFork:
      inc forkCount
      echo "\n--- 行 ", i + 1, " 发现分叉 ---"
      echo "路径 A 信念：", fork.pathA.belief
      echo "路径 B 信念：", fork.pathB.belief
      echo "胜出路径：", if fork.winner == 0: "A" elif fork.winner == 1: "B" else: "无"
  
  if forkCount == 0:
    echo "未发现信念分叉"
  else:
    echo "\n总计：", forkCount, " 处分叉"

proc cmdExport*(config: CliConfig) =
  if config.args.len == 0:
    echo "[ERROR] 请指定导出格式 (chain/json/markdown)"
    return
  
  let format = config.args[0].toLower()
  echo "[EXPORT] 导出为 ", format, " 格式..."
  
  # 加载表格
  let prefix = config.workDir / "default"
  let engine = loadRuntime(prefix)
  let table = engine.injection.table
  
  let outFile = if config.chainFile.len > 0: config.chainFile
                else: config.workDir / "export." & format
  
  case format
  of "chain":
    table.saveChain(outFile)
    echo "[OK] 已导出到：", outFile
  
  of "json":
    var f = open(outFile, fmWrite)
    defer: f.close()
    
    f.writeLine("{")
    f.writeLine("  \"version\": \"1.0\",")
    f.writeLine("  \"table\": {")
    f.writeLine("    \"maxRows\": ", table.maxRows, ",")
    f.writeLine("    \"rows\": [")
    
    var first = true
    for i in 0..<table.maxRows:
      if table.rows[i].question.len == 0:
        continue
      
      if not first:
        f.writeLine(",")
      first = false
      
      f.write("      {")
      f.write("\"row\": ", i + 1, ", ")
      f.write("\"question\": \"", table.rows[i].question.replace("\"", "\\\""), "\", ")
      f.write("\"viewpoint\": \"", $table.rows[i].viewpoint, "\", ")
      f.write("\"state\": \"", $table.rows[i].state, "\", ")
      f.write("\"jurisdiction\": \"", $table.rows[i].jurisdiction, "\", ")
      f.write("\"gameIdentity\": \"", $table.rows[i].gameIdentity, "\", ")
      f.write("\"answer\": \"", table.rows[i].answer.replace("\"", "\\\""), "\", ")
      f.write("\"belief\": \"", table.rows[i].belief.replace("\"", "\\\""), "\"")
      f.write("}")
    
    f.writeLine("\n    ]")
    f.writeLine("  }")
    f.writeLine("}")
    
    echo "[OK] 已导出到：", outFile
  
  of "markdown":
    var f = open(outFile, fmWrite)
    defer: f.close()
    
    f.writeLine("# 表格引擎状态导出\n")
    f.writeLine("## 基本信息\n")
    f.writeLine("- 表格大小：", table.maxRows, " 行")
    f.writeLine("- 导出时间：", now().format("yyyy-MM-dd HH:mm:ss"))
    f.writeLine("\n## 活跃行\n")
    f.writeLine("| 行号 | 提问 | 视角 | 所在态 | 管辖 | 博弈身份 | 回答 | 信念 |")
    f.writeLine("|------|------|------|--------|------|----------|------|------|")
    
    for i in 0..<table.maxRows:
      if table.rows[i].question.len == 0:
        continue
      
      f.writeLine("| ", i + 1, 
                  " | ", table.rows[i].question,
                  " | ", $table.rows[i].viewpoint,
                  " | ", $table.rows[i].state,
                  " | ", $table.rows[i].jurisdiction,
                  " | ", $table.rows[i].gameIdentity,
                  " | ", table.rows[i].answer,
                  " | ", table.rows[i].belief, " |")
    
    echo "[OK] 已导出到：", outFile
  
  else:
    echo "[ERROR] 未知格式：", format
    echo "支持：chain, json, markdown"

proc cmdClean*(config: CliConfig) =
  echo "[CLEAN] 清理临时文件..."
  
  let files = [
    config.workDir / "*.tmp",
    config.workDir / "*.log",
    config.workDir / "*.bak"
  ]
  
  var cleaned = 0
  for pattern in files:
    # 简单实现：只清理已知临时文件
    let tmpFile = config.workDir / "default_meta.tmp"
    if fileExists(tmpFile):
      removeFile(tmpFile)
      inc cleaned
  
  echo "[CLEAN] 清理完成，删除 ", cleaned, " 个文件"

# =============================================================================
# 主入口
# =============================================================================

proc runCli*(args: seq[string]) =
  let config = parseArgs(args)
  
  case config.command
  of CmdHelp: cmdHelp()
  of CmdInit: cmdInit(config)
  of CmdRun: cmdRun(config)
  of CmdInject: cmdInject(config)
  of CmdShow: cmdShow(config)
  of CmdSave: discard  # 由 run 命令自动保存
  of CmdLoad: discard  # 由各命令自动加载
  of CmdEval: cmdEval(config)
  of CmdFork: cmdFork(config)
  of CmdExport: cmdExport(config)
  of CmdClean: cmdClean(config)

when isMainModule:
  let args = commandLineParams()
  if args.len == 0:
    cmdHelp()
  else:
    runCli(args)
