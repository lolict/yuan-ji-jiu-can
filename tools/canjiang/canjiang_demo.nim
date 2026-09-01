# ============================================================
# 蚕蛹反向寄生壳 · 端到端测试 + 真实 Linux 外壳接入
# TASKS.md P2 — 脱壳穿衣流程端到端 + Android/iOS/Windows/macOS/Linux
# 真实环境：Linux 容器 = cli-shell (STDIN/STDOUT) + fs-shell (PATH/INODE)
# ============================================================

import parasite
import kernel
import shell/registry
import json
import strutils
import std/os
import std/times

# 单次完整生命周期的端到端 trace
type
  E2ERecord* = object
    step*: int
    shell_name*: string
    action*: string
    result*: string
    duration_ms*: float

var trace*: seq[E2ERecord] = @[]
var step_counter = 0

proc rec(action, result: string, shell = "", dur = 0.0) =
  inc step_counter
  trace.add(E2ERecord(
    step: step_counter,
    shell_name: shell,
    action: action,
    result: result,
    duration_ms: dur
  ))

# ============ 真实 Linux 外壳适配 ============

# cli-shell: 真实输出到 STDOUT
proc do_real_cli(par: var Parasite, input_code: uint64): string =
  let k = par.kernel
  let verdict = k.kernel_judge(input_code)
  # 外壳被迫遵守反向规则: 输出必须携带内核版本
  result = "[v" & k.version & "|" & par.config.host_shell & "] " & verdict

# fs-shell: 写真实文件到磁盘
proc do_real_fs(par: var Parasite, data: string): string =
  let mount = par.config.mount_point
  if mount == "": return "挂载点未设置"
  # PATH/INODE 外壳: 映射内核数据到 /tmp/canjiang-mount/...
  let realpath = "/tmp/canjiang-fs-shell" & mount
  try:
    createDir(realpath.parentDir)
    let f = open(realpath, fmWrite)
    f.writeLine("# 蚕蛹外壳-内核绑定")
    f.writeLine("version: " & par.kernel.version)
    f.writeLine("anchors: " & par.kernel.anchors[0] & "," & par.kernel.anchors[1])
    f.writeLine("host: " & par.config.host_shell)
    f.writeLine("data: " & data)
    f.close()
    result = "已寄生到文件系统: " & realpath & " (真实文件已写入)"
  except:
    result = "外壳写入失败: " & getCurrentExceptionMsg()

# ============ 端到端测试 ============

proc e2e_single_shell(par: var Parasite, idx: int) =
  let t0 = cpuTime()

  # 1. 脱壳
  par.shed()
  rec("shed", "身份已剥离")

  # 2. 选壳
  let r = par.choose_shell(idx)
  rec("choose_shell", r)
  if r.startsWith("外壳不兼容"):
    rec("skip", "外壳不兼容")
    return

  # 3. 穿衣
  let dr = par.dress()
  rec("dress", "反向规则数=" & $par.config.reverse_rules.len)

  # 4. 内核真实判断 (anchor 见证)
  let k = par.kernel
  k.kernel_tick()
  # 构造 64 位代码: presence=1, witness=1, anchor=1 → 准入(我)
  let code: uint64 = (1 shl 10) or (1 shl 1) or 1
  let verdict = k.kernel_judge(code)
  rec("kernel_judge", verdict)

  # 5. 外壳视角
  let sv = par.view_from_shell()
  rec("view_from_shell", sv.split("\n")[0])
  # 6. 内核视角
  let kv = par.view_from_kernel()
  rec("view_from_kernel", kv.split("\n")[0])

  # 7. 真实外壳动作 (按类型区分)
  if par.config.host_shell == "cli-shell":
    let rOut = par.do_real_cli(code)
    rec("real_cli", rOut)
  elif par.config.host_shell == "fs-shell":
    let rOut = par.do_real_fs("kernel_state_snapshot")
    rec("real_fs", rOut)
  else:
    rec("mock_action", "外壳已虚拟化就绪")

  let t1 = cpuTime()
  rec("e2e_complete", "耗时=" & $( (t1 - t0) * 1000 ) & "ms",
      par.config.host_shell, (t1 - t0) * 1000)

# ============ 入口 ============

when isMainModule:
  echo "蚕蛹反向寄生壳 · 端到端 + 真实外壳接入"
  echo "============================================================"

  # 新建内核寄生体
  var par = new_parasite()
  echo par.kernel.kernel_selfcheck()

  # 遍历所有外壳
  let shell_cnt = shell_count()
  for i in 0 ..< shell_cnt:
    echo "\n--- 测试外壳 #" & $i & " ---"
    par.e2e_single_shell(i)

  # 打印完整 trace (JSON)
  echo "\n============================================================"
  echo "端到端 Trace (JSON)"
  echo "============================================================"
  var jarr = newJArray()
  for r in trace:
    let o = newJObject()
    o["step"] = newJInt(r.step)
    o["shell"] = newJString(r.shell_name)
    o["action"] = newJString(r.action)
    o["result"] = newJString(r.result)
    o["ms"] = newJFloat(r.duration_ms)
    jarr.add(o)
  echo pretty(jarr, 2)

  # 验证真实 fs 外壳写入
  echo "\n============================================================"
  echo "真实 FS 外壳验证"
  echo "============================================================"
  let fpath = "/tmp/canjiang-fs-shell/fs-shell/.kernel"
  if fileExists(fpath):
    echo "✓ 文件已存在: ", fpath
    for line in lines(fpath):
      echo "  ", line
  else:
    echo "✗ 文件不存在: ", fpath

  # 验证反向规则 (隐藏根基)
  echo "\n隐藏根基验证:"
  echo "anchor_map[0] = ", par.config.anchor_map[0]
  echo "anchor_map[1] = ", par.config.anchor_map[1]
  echo "内核真实锚点: [", par.kernel.anchors[0], ", ", par.kernel.anchors[1], "]"
  echo "外壳视角看到的是反向映射, 真实锚点已隐藏"
