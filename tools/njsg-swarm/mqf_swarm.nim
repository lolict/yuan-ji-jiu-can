# ============================================================
# mqf-swarm — NJSG-Swarm 协调器 CLI
#
# 用法:
#   mqf-swarm demo                               — 完整演示
#   mqf-swarm spawn <name> <role>                 — 派生分身
#   mqf-swarm task <desc> [priority]              — 提交任务
#   mqf-swarm assign                              — 分配任务
#   mqf-swarm report <clone> <result>             — 分身报告
#   mqf-swarm heartbeat <clone>                   — 心跳重置
#   mqf-swarm resolve <conflict-id>               — 裁决冲突
#   mqf-swarm check                               — 心跳检测
#   mqf-swarm status                              — 协调器状态
#
# 分身角色: git-worker / api-worker / ai-worker / validator / guardian
# ============================================================

import os, strutils
import swarm_engine

var swarm = new_swarm()

let args = commandLineParams()
if args.len == 0:
  echo "mqf-swarm — NJSG-Swarm 协调器"
  echo "用法: mqf-swarm <demo|spawn|task|assign|parallel|report|heartbeat|resolve|check|beat|status>"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

case cmd
of "demo":
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║        NJSG-Swarm · 协调器演示                    ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""

  echo "── 1. 派生分身 ──"
  echo "  " & swarm.spawn_clone("clf-1", "git-worker")
  echo "  " & swarm.spawn_clone("clf-2", "api-worker")
  echo "  " & swarm.spawn_clone("clf-3", "ai-worker")
  echo "  " & swarm.spawn_clone("guard-1", "guardian")

  echo ""
  echo "── 2. 提交任务 ──"
  echo "  " & swarm.submit_task("编码:mlll根基流转", 7)
  echo "  " & swarm.submit_task("验证:gtt路由检查", 5)
  echo "  " & swarm.submit_task("API:外部数据获取", 6)
  echo "  " & swarm.submit_task("AI:辅助编码分析", 8)
  echo "  " & swarm.submit_task("验证:根编码一致性", 4)

  echo ""
  echo "── 3. 分配任务 ──"
  let assigns = swarm.assign_tasks()
  for a in assigns:
    echo "  " & a

  echo ""
  echo "── 4. 真实并行执行 (fork worker 子进程) ──"
  # 让未完成的保持 working 状态，由 spawn_parallel 处理
  let par = swarm.spawn_parallel()
  for r in par:
    echo "  " & r
  echo ""
  echo "── 5. 磁盘心跳检测 ──"
  let beats = swarm.check_disk_heartbeats()
  for b in beats:
    echo "  " & b

  echo ""
  echo "── 6. 冲突检测 ──"
  # 制造冲突
  discard swarm.submit_task("token:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJtZXNzYWdlIjoiSGVsbG8gZnJvbSBDYXRQYXcifQ", 9)
  discard swarm.submit_task("token:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJtZXNzYWdlIjoiSGVsbG8gZnJvbSBDYXRQYXcifQ", 9)
  let assigns2 = swarm.assign_tasks()
  for a in assigns2:
    echo "  " & a
  let conflicts = swarm.detect_conflicts()
  if conflicts.len > 0:
    echo "  检测到 " & $conflicts.len & " 个冲突"
    for c in conflicts:
      echo "  " & swarm.resolve_conflict(c.id)

  echo ""
  echo "── 7. 协调器报告 ──"
  swarm.report()

of "spawn":
  if rest.len < 2:
    echo "用法: mqf-swarm spawn <name> <role>"
    quit(1)
  echo swarm.spawn_clone(rest[0], rest[1])

of "task":
  if rest.len < 1:
    echo "用法: mqf-swarm task <desc> [priority]"
    quit(1)
  let prio = if rest.len > 1: parseInt(rest[1]) else: 5
  echo swarm.submit_task(rest[0], prio)

of "assign":
  let assigns = swarm.assign_tasks()
  if assigns.len == 0:
    echo "无可分配任务"
  for a in assigns:
    echo a

of "parallel":
  # 真实并行执行: fork worker 子进程
  let res = swarm.spawn_parallel()
  if res.len == 0:
    echo "无工作中的分身可执行"
  for r in res:
    echo r

of "beat":
  # 磁盘心跳检测
  let res = swarm.check_disk_heartbeats()
  if res.len == 0:
    echo "无心跳文件，未检测到活动分身"
  for r in res:
    echo r

of "report":
  if rest.len < 2:
    echo "用法: mqf-swarm report <clone> <result>"
    quit(1)
  echo swarm.report_result(rest[0], rest[1])
  swarm.complete_clone_task(rest[0])

of "heartbeat":
  if rest.len < 1:
    echo "用法: mqf-swarm heartbeat <clone>"
    quit(1)
  swarm.reset_heartbeat(rest[0])
  echo "[" & rest[0] & "] 心跳重置"

of "resolve":
  if rest.len < 1:
    echo "用法: mqf-swarm resolve <conflict-id>"
    quit(1)
  echo swarm.resolve_conflict(rest[0])

of "check":
  let res = swarm.check_heartbeats()
  if res.len == 0:
    echo "所有分身心跳正常"
  for r in res:
    echo r

of "status":
  swarm.report()

else:
  echo "未知命令: " & cmd
  quit(1)
