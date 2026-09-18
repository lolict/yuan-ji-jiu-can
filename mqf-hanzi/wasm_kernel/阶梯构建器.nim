## 阶梯制 · 双向公约 构建器
##
##  读取同一份 mqf_container.c · 按目标平台各编译一次
##
##  目标               命令
##  ──────────────    ─────────────────────────────────────────
##  kernel_64         clang -m64 -DMQF_BITWIDTH=64             ✓ 已验证
##  kernel_32         clang -m32 -DMQF_BITWIDTH=32             (需 32 位头文件)
##  wasm32            clang --target=wasm32 -nostdlib -c       (需 wasm-ld)
##  wasm64            clang --target=wasm64 -nostdlib -c       (需 wasm-ld)
##
##  运行：nim c --cc:clang -r 阶梯构建器.nim

import std/os
import std/strutils
import std/tables

type
  编译目标* = object
    名称*: string
    标志*: seq[string]
    输出*: string
    状态*: string     # "pass" / "skip" / "fail"

  构建报告* = object
    目标列表*: seq[编译目标]
    gcc路径*: string

const
  源码文件 = "mqf_container.c"
  头文件   = "mqf_wasm.h"
  编译器   = "/usr/bin/clang"

proc 执行(cmd: string): tuple[输出: string, 码: int] =
  let tmp = ".__build_tmp"
  let fullCmd = cmd & " > " & tmp & " 2>&1; echo $? > " & tmp & ".code"
  discard execShellCmd(fullCmd)
  result.输出 = readFile(tmp).strip
  let codeStr = readFile(tmp & ".code").strip
  result.码 = if codeStr.len > 0: codeStr.parseInt else: -1
  discard tryRemoveFile(tmp)
  discard tryRemoveFile(tmp & ".code")

proc 收集目标(): seq[编译目标] =
  result = @[
    编译目标(名称: "kernel_64",
             标志: @["-m64", "-DMQF_BITWIDTH=64", "-o", "kernel_64"],
             输出: "kernel_64"),
    编译目标(名称: "kernel_32",
             标志: @["-m32", "-DMQF_BITWIDTH=32", "-o", "kernel_32"],
             输出: "kernel_32"),
    编译目标(名称: "wasm32_obj",
             标志: @["--target=wasm32", "-DMQF_BITWIDTH=32", "-nostdlib",
                       "-c", "-o", "kernel_wasm32.o"],
             输出: "kernel_wasm32.o"),
    编译目标(名称: "wasm64_obj",
             标志: @["--target=wasm64", "-DMQF_BITWIDTH=64", "-nostdlib",
                       "-c", "-o", "kernel_wasm64.o"],
             输出: "kernel_wasm64.o")
  ]

when isMainModule:
  echo "========================================"
  echo "  阶梯制 · 双向公约  构建器"
  echo "========================================"

  if not 源码文件.fileExists or not 头文件.fileExists:
    echo "[失败] mqf_container.c 或 mqf_wasm.h 不存在"
    quit(1)

  let cwd = getCurrentDir()
  var targets = 收集目标()

  echo ""
  echo "▸ 源码文件: " & 源码文件
  echo "▸ 编译标志（统一宏）: -DMQF_BITWIDTH=32 / -DMQF_BITWIDTH=64"
  echo "▸ 输出目录: " & cwd
  echo ""

  var passCount = 0
  var skipCount = 0
  var failCount = 0

  for t in targets.mitems:
    echo "──────── " & t.名称 & " ────────"
    var cmd = 编译器
    for f in t.标志:
      cmd.add(" " & f)
    cmd.add(" " & 源码文件)

    echo "  $ " & cmd
    let (outp, code) = 执行(cmd)
    if outp.len > 0:
      for line in outp.splitLines:
        echo "    " & line

    if code == 0:
      t.状态 = "PASS"
      inc passCount
      echo "  [" & t.输出 & "] ✓"
    elif code == 127 or outp.contains("file not found"):
      t.状态 = "SKIP (tool missing)"
      inc skipCount
      echo "  [跳过] 工具链缺失"
    else:
      t.状态 = "FAIL"
      inc failCount
      echo "  [失败] 码=" & $code

  echo ""
  echo "========================================"
  echo "  构建报告"
  echo "========================================"
  echo "  PASS: " & $passCount
  echo "  SKIP: " & $skipCount
  echo "  FAIL: " & $failCount

  echo ""
  echo "  目标矩阵:"
  echo "  ┌────────────┬──────────┬──────────┐"
  echo "  │ 目标       │ 32 位    │ 64 位    │"
  echo "  ├────────────┼──────────┼──────────┤"
  echo "  │ 本地       │ " & (if targets[1].状态.contains("PASS"): "  ✓  " else: "  ⏸  ") & " │ " & (if targets[0].状态.contains("PASS"): "  ✓  " else: "  ✗  ") & " │"
  echo "  │ WASM       │ " & (if targets[2].状态.contains("PASS"): "  ✓  " else: "  ⏸  ") & " │ " & (if targets[3].状态.contains("PASS"): "  ✓  " else: "  ⏸  ") & " │"
  echo "  └────────────┴──────────┴──────────┘"
  echo ""
  echo "  阶梯制：一份源码 × 两种位宽 × 两个平台 = 同一套规则"
  echo "========================================"

  # 清理中间产物（只保留 pass 的二进制）
  for t in targets:
    if t.状态 != "PASS" and t.输出.fileExists:
      discard tryRemoveFile(t.输出)
