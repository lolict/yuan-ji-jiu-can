# Git 命令直接执行 — 绕过 shell，避免转义问题

import os, osproc, strutils

proc exec_git*(cwd: string; args: seq[string]): tuple[exitcode: int, output: string] =
  ## 直接执行 git 命令（不经过 shell），避免引号/尖括号转义
  var cmd_args: seq[string] = @["-C", cwd]
  cmd_args.add(args)
  let p = startProcess("git", args = cmd_args, options = {poUsePath, poStdErrToStdOut})
  let outs = p.readAll()
  let ec = p.waitForExit()
  p.close()
  result = (ec, outs.strip())
