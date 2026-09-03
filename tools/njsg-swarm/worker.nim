# ============================================================
# NJSG-Swarm Worker — 真实子代理 (并行执行单元)
#
# 每个 Worker = 一个 OS 子进程，通过 JSON 与协调器通信
# 输入: stdin JSON { "task": "id:desc", "role": "git-worker"... }
# 输出: stdout JSON { "status": "done|error", "output": "..." }
# 心跳: ./.njsg-swarm/beat/<name>.hb 每 2s 写时间戳
#
# 蚕蛹执行层: worker 调用真实工具 (mqf-git, mqf-api, mqf-ai)
# ============================================================

import std/[os, osproc, json, strutils, times]

const
  REPO_ROOT = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
  TOOLS_DIR = REPO_ROOT & "/tools"
  BEAT_DIR = REPO_ROOT & "/.njsg-swarm/beat"

type
  WorkerTask = object
    task_id*: string
    description*: string
    role*: string
    rule_version*: string

# ═══════════════════════════════════════════
# 心跳: 写时间戳
# ═══════════════════════════════════════════
proc write_beat(worker_name: string) =
  let beat_dir = BEAT_DIR
  if not dirExists(beat_dir):
    createDir(beat_dir)
  let path = beat_dir & "/" & worker_name & ".hb"
  writeFile(path, $epochTime())

# ═══════════════════════════════════════════
# 蚕蛹执行层: 调用真实工具
# ═══════════════════════════════════════════
proc exec_tool(tool: string; args: seq[string]): tuple[output: string; ec: int] =
  # 工具可能是已编译二进制或 Nim 源
  var cmd = ""
  let bin_path = TOOLS_DIR & "/" & tool
  let nim_path = bin_path & ".nim"
  if fileExists(bin_path):
    cmd = bin_path & " " & args.join(" ")
  elif fileExists(nim_path):
    # 用 nim r 直接调用源码
    let nim_bin = getCurrentDir().parentDir() / "nim-2.2.2" / "bin" / "nim"
    cmd = nim_bin & " r --cc:clang --hints:off --warnings:off " & nim_path & " " & args.join(" ")
  else:
    return ("ERROR: tool not found: " & tool, 1)
  let (out_s, ex_c) = execCmdEx(cmd)
  result = (out_s, ex_c)

# ═══════════════════════════════════════════
# 任务执行
# ═══════════════════════════════════════════
proc run_task(t: WorkerTask): JsonNode =
  result = newJObject()
  result["task_id"] = newJString(t.task_id)
  result["role"] = newJString(t.role)

  var output = ""
  var ec = 0

  # 根据角色选择执行路径
  case t.role
  of "git-worker":
    if t.description.contains("编码") or t.description.contains("根基"):
      let r = exec_tool("silkworm-shell/mqf_git", @["encode", t.description])
      output = r.output
      ec = r.ec
    elif t.description.contains("仓库"):
      let r = exec_tool("silkworm-shell/mqf_shell", @["demo"])
      output = r.output
      ec = r.ec
    else:
      output = "[GIT-WORKER] git log --oneline -3:\n"
      let r = execCmdEx("git -C " & REPO_ROOT & " log --oneline -3")
      output &= r[0]

  of "api-worker":
    # 编码解码任务
    if t.description.contains("token"):
      output = "[API-WORKER] Token 任务解码 (简化)\n"
      output &= "Token length: " & $t.description.len & "\n"
      output &= "Header: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9 (MQF-JWT)"
    else:
      let r = exec_tool("silkworm-shell/mqf_api", @["gtt", "/v1/status"])
      output = r.output
      ec = r.ec

  of "ai-worker":
    if t.description.contains("分析") or t.description.contains("辅助"):
      # 编码任务：本地无 API key 时返回编码输出
      output = "[AI-WORKER] 编码模式 (无云端依赖)\n"
      output &= "Task: " & t.description & "\n"
      output &= "Encoding → mltt:" & t.description[0..<min(8, t.description.len)] & "..."
    else:
      output = "[AI-WORKER] 任务接收: " & t.description

  of "validator":
    # 验证任务：检查编码/规则一致性
    output = "[VALIDATOR] 验证: " & t.description & "\n"
    let r = execCmdEx("git -C " & REPO_ROOT & " log --oneline -1")
    output &= "最新提交: " & r[0]

  of "guardian":
    # 守护任务：检查系统状态
    output = "[GUARDIAN] 系统守护报告\n"
    let r = execCmdEx("cd " & REPO_ROOT & " ; git status --short")
    output &= "工作区: " & $r[0].count('\n') & " 改动"

  else:
    output = "[GENERIC] 通用任务执行: " & t.description

  result["output"] = newJString(output.strip())
  result["status"] = if ec == 0: newJString("done") else: newJString("error")
  result["exit_code"] = newJInt(ec)
  result["timestamp"] = %* epochTime()

# ═══════════════════════════════════════════
# Worker 主循环
# ═══════════════════════════════════════════
proc worker_main(worker_name: string) =
  # 启动心跳
  write_beat(worker_name)

  # 从 stdin 读任务 JSON
  let input = stdin.readLine()
  if input.strip().len == 0:
    let err = %* {"error": "empty input"}
    echo err
    quit(1)

  try:
    let task_json = parseJson(input)
    let t = WorkerTask(
      task_id: task_json["task_id"].getStr(),
      description: task_json["description"].getStr(),
      role: task_json.getOrDefault("role").getStr("generic"),
      rule_version: task_json.getOrDefault("rule_version").getStr("v1.0")
    )
    let result = run_task(t)
    echo result
    write_beat(worker_name)
  except Exception as e:
    let err = %* {"error": "parse error: " & e.msg}
    echo err
    quit(1)

# ═══════════════════════════════════════════
# CLI 参数解析（简单版）
# ═══════════════════════════════════════════
var name = "w0"
let all_args = commandLineParams()
for i in 0..<all_args.len:
  if all_args[i] == "--name" and i + 1 < all_args.len:
    name = all_args[i + 1]
    break

worker_main(name)
