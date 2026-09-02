# ============================================================
# mqf-git — 蚕蛹 Git 壳 CLI
#
# 用法:
#   mqf-git commit <mqf-text> <author>
#   mqf-git encode <mqf-text>
#   mqf-git decode <git-message>
#   mqf-git log
#   mqf-git branch-phase <n>
#   mqf-git adapt-remote <url> [name]
#   mqf-git status
# ============================================================

import os, strutils
import git_shell, git_run

const VERSION = "1.0.0"

proc print_usage() =
  echo "mqf-git v" & VERSION & " — 蚕蛹 Git 壳 CLI"
  echo ""
  echo "用法:"
  echo "  mqf-git commit <mqf-text> <author>"
  echo "  mqf-git encode <mqf-text>"
  echo "  mqf-git decode <git-message>"
  echo "  mqf-git log"
  echo "  mqf-git branch-phase <n>          (n=0..8)"
  echo "  mqf-git adapt-remote <url> [name]"
  echo "  mqf-git status"
  echo "  mqf-git help"

let args = commandLineParams()
if args.len == 0:
  print_usage()
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

case cmd
of "commit":
  if rest.len < 2:
    echo "用法: mqf-git commit <mqf-text> <author>"
    quit(1)
  let shell = new_git_shell(getCurrentDir())
  let h = shell.commit(rest[0], rest[1])
  if h.len > 0:
    echo "提交成功: " & h
of "encode":
  if rest.len < 1:
    echo "用法: mqf-git encode <mqf-text>"
    quit(1)
  echo encode_mqf_to_git(rest.join(" "))
of "decode":
  if rest.len < 1:
    echo "用法: mqf-git decode <git-message>"
    quit(1)
  echo decode_git_to_mqf(rest.join(" "))
of "log":
  let shell = new_git_shell(getCurrentDir())
  let commits = shell.read_log()
  if commits.len == 0:
    echo "无提交历史"
  for i in 0..<commits.len:
    let c: GitCommit = commits[i]
    echo "[" & c.hash_str & "] " & c.author & " · " & c.timestamp
    echo "   " & c.mqf_content
of "branch-phase":
  if rest.len < 1:
    echo "用法: mqf-git branch-phase <n> (n=0..8)"
    quit(1)
  let n = parseInt(rest[0])
  if n < 0 or n > 8:
    echo "n 须在 0..8"
    quit(1)
  let shell = new_git_shell(getCurrentDir())
  discard shell.create_phase_branch("phase-" & $n, float(n) * 0.1)
of "adapt-remote":
  if rest.len < 1:
    echo "用法: mqf-git adapt-remote <url> [name]"
    quit(1)
  let url = rest[0]
  let name = if rest.len > 1: rest[1] else: "mqf-mengjichang"
  let shell = new_git_shell(getCurrentDir())
  discard shell.adapt_mengjichang(url, name)
of "status":
  let shell = new_git_shell(getCurrentDir())
  report_git(shell)
of "help", "-h", "--help":
  print_usage()
else:
  echo "未知命令: " & cmd
  quit(1)
