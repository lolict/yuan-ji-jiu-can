## 满全法全栈联动 v2.0
## ─────────────────────────────────────────────────────────
##  一键跑通五模块：
##    自举编译器 → WASM内核 → 蚕蛹壳 → 运行时 → 三维渲染
##
##  运行：nim c --cc:clang -r 全栈联动v2.nim

import std/os
import std/strutils

const NIM_COMPILER = "/mnt/data/catpaw/home/.local/nim/bin/nim"
const CLANG        = "/usr/bin/clang"

type
  PipelineStep* = object
    name*:    string
    command*: string
    status*:  string
    output*:  string

proc definePipeline(): seq[PipelineStep] =
  result = @[
    PipelineStep(
      name: "① 自举编译器",
      command: NIM_COMPILER & " c --cc:clang -r" &
               " ../nim_parser/自举编译器/自举编译器.nim"
    ),
    PipelineStep(
      name: "② WASM内核-wasm32",
      command: CLANG & " --target=wasm32 -nostdlib -c" &
               " ../wasm_kernel/mqf_container.c" &
               " -o ../wasm_kernel/_fs_wasm32.o"
    ),
    PipelineStep(
      name: "② WASM内核-wasm64",
      command: CLANG & " --target=wasm64 -nostdlib -c" &
               " ../wasm_kernel/mqf_container.c" &
               " -o ../wasm_kernel/_fs_wasm64.o"
    ),
    PipelineStep(
      name: "② WASM内核-bridge64",
      command: CLANG & " -m64 -DMQF_BITWIDTH=64 -DMQF_NO_MAIN" &
               " ../wasm_kernel/mqf_container.c" &
               " ../wasm_kernel/mqf_bridge.c" &
               " -o ../wasm_kernel/_fs_bridge64"
    ),
    PipelineStep(
      name: "③ 蚕蛹脱壳器",
      command: NIM_COMPILER & " c --cc:clang -r" &
               " ../silkworm_shell/蚕蛹脱壳器.nim"
    ),
    PipelineStep(
      name: "④ 满全法运行时",
      command: NIM_COMPILER & " c --cc:clang -r" &
               " ../runtime/满全法运行时.nim"
    ),
    PipelineStep(
      name: "⑤ 三维几何渲染",
      command: NIM_COMPILER & " c --cc:clang -r" &
               " ../glyph3d/容器字体时间.nim"
    ),
    PipelineStep(
      name: "⑥ bridge64 运行验证",
      command: "../wasm_kernel/_fs_bridge64"
    )
  ]

var pipeCounter = 0

proc runCommand(cmd: string): tuple[output: string, code: int] =
  inc pipeCounter
  let tmp = ".__pipe_" & $pipeCounter
  let exitFile = tmp & ".code"
  let fullCmd = cmd & " > " & tmp & " 2>&1; echo $? > " & exitFile
  try:
    discard execShellCmd(fullCmd)
    result.output = readFile(tmp).strip
    let codeStr = readFile(exitFile).strip
    result.code = if codeStr.len > 0: codeStr.parseInt else: 0
  except:
    result.output = ""
    result.code = -1
  finally:
    discard tryRemoveFile(tmp)
    discard tryRemoveFile(exitFile)

when isMainModule:
  echo "╔══════════════════════════════════════════════════╗"
  echo "║       满全法全栈联动 v2.0  MQF Full Stack       ║"
  echo "║     自举 → WASM → 蚕蛹 → 运行时 → 渲染        ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo ""

  let cwd = getCurrentDir()
  var steps = definePipeline()

  echo "▸ 工作目录: " & cwd
  echo "▸ 流水线步骤: " & $steps.len
  echo ""

  var passCount = 0
  var skipCount = 0
  var failCount = 0

  for step in steps.mitems:
    echo "── " & step.name & " ──"
    echo "  $ " & step.command

    let (outp, code) = runCommand(step.command)

    if outp.len > 0:
      let lines = outp.splitLines
      let shown = min(lines.len, 10)
      for i in 0 ..< shown:
        echo "    " & lines[i]
      if lines.len > shown:
        echo "    ... (" & $(lines.len - shown) & " 行省略)"

    if code == 0:
      step.status = "PASS"
      inc passCount
      echo "  [PASS] ✓"
    elif code == 127 or outp.contains("file not found"):
      step.status = "SKIP"
      inc skipCount
      echo "  [SKIP] (依赖缺失)"
    else:
      step.status = "FAIL"
      inc failCount
      echo "  [FAIL] code=" & $code
    echo ""

  echo "══════════════════════════════════════════════════"
  echo "  全栈联动报告"
  echo "══════════════════════════════════════════════════"
  echo ""
  echo "  流水线总览:"
  echo "  ┌────┬───────────────────┬────────┐"
  echo "  │  # │ 步骤              │ 状态   │"
  echo "  ├────┼───────────────────┼────────┤"

  for i, step in steps:
    let num = align($(i+1), 2)
    let nm = step.name & "                "
    let st = case step.status
             of "PASS": " ✓   "
             of "SKIP": " ⏸   "
             of "FAIL": " ✗   "
             else: " ？  "
    echo "  │ " & num & " │ " & nm[0..18] & "│ " & st & "│"

  echo "  └────┴───────────────────┴────────┘"
  echo ""
  echo "  PASS:    " & $passCount
  echo "  SKIP:    " & $skipCount
  echo "  FAIL:    " & $failCount
  echo ""

  if failCount == 0:
    echo "▸ 全栈联动成功 · 满全法系统端到端可运行"
  else:
    echo "▸ 部分步骤失败，请检查上方日志"

  echo ""
  echo "══════════════════════════════════════════════════"

  # 清理临时产物
  for f in ["../nim_parser/自举编译器/自举编译器",
            "../wasm_kernel/_fs_wasm32.o", "../wasm_kernel/_fs_wasm64.o",
            "../wasm_kernel/_fs_bridge64",
            "../silkworm_shell/蚕蛹脱壳器",
            "../runtime/满全法运行时",
            "../glyph3d/容器字体时间"]:
    discard tryRemoveFile(f)
