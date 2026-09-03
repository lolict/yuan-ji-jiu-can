# ============================================================
# mqf-shell — 蚕蛹多重壳聚合器
#
# 统一入口，操作所有外部系统:
#   mqf-shell git commit <mqf> <author>
#   mqf-shell git log
#   mqf-shell git branch-phase <n>
#   mqf-shell api get <url>
#   mqf-shell api encode-path <path>
#   mqf-shell cloud encode <mqf>
#   mqf-shell browser encode <path>
#   mqf-shell file encode <mqf>
#   mqf-shell demo
# ============================================================

import os, strutils
import git_shell, api_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-shell — 蚕蛹多重壳聚合器"
  echo "用法: mqf-shell <git|api|cloud|browser|file|demo> [args]"
  quit(0)

let shell_type = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

# Git 壳路由
proc route_git(args: seq[string]) =
  if args.len == 0:
    echo "git子命令: commit / log / branch-phase / encode / decode"
    return
  let sub = args[0]
  let r = if args.len > 1: args[1..^1] else: @[]
  case sub
  of "commit":
    if r.len < 2:
      echo "用法: mqf-shell git commit <mqf> <author>"
      return
    let s = new_git_shell(getCurrentDir())
    discard s.commit(r[0], r[1])
  of "log":
    let s = new_git_shell(getCurrentDir())
    let commits = s.read_log()
    if commits.len == 0:
      echo "无提交历史"
    for i in 0..<commits.len:
      let c: GitCommit = commits[i]
      echo "[" & c.hash_str & "] " & c.author & " · " & c.timestamp
      echo "   " & c.mqf_content
  of "branch-phase":
    if r.len < 1:
      echo "用法: mqf-shell git branch-phase <n>"
      return
    let n = parseInt(r[0])
    let s = new_git_shell(getCurrentDir())
    discard s.create_phase_branch("phase-" & $n, float(n) * 0.1)
  of "encode":
    if r.len < 1:
      echo "用法: mqf-shell git encode <mqf>"
      return
    echo encode_mqf_to_git(r.join(" "))
  of "decode":
    if r.len < 1:
      echo "用法: mqf-shell git decode <msg>"
      return
    echo decode_git_to_mqf(r.join(" "))
  of "status":
    let s = new_git_shell(getCurrentDir())
    report_git(s)

# API 壳路由
proc route_api(args: seq[string]) =
  if args.len == 0:
    echo "api子命令: get / post / encode-path / adapt"
    return
  let sub = args[0]
  let r = if args.len > 1: args[1..^1] else: @[]
  case sub
  of "get":
    if r.len < 1:
      echo "用法: mqf-shell api get <url> [token]"
      return
    var s = new_api_shell(r[0], if r.len > 1: r[1] else: "")
    let resp = s.get("/")
    echo "HTTP " & $resp.status_code & " | " & resp.decoded_mqf
  of "encode-path":
    if r.len < 1:
      echo "用法: mqf-shell api encode-path <mqf-path>"
      return
    echo encode_uri_path(r[0])
  of "adapt":
    if r.len < 2:
      echo "用法: mqf-shell api adapt <name> <url>"
      return
    var s = new_api_shell("")
    discard s.adapt_mengjichang(r[0], r[1])

# 共享编码层路由 (cloud/browser/file 共用)
proc route_cloud(args: seq[string]) =
  if args.len < 2:
    echo "用法: mqf-shell cloud encode <mqf>"
    return
  if args[0] == "encode":
    echo "[CLOUD] " & encode_mqf_to_git(args[1..^1].join(" "))

proc route_file(args: seq[string]) =
  if args.len < 2:
    echo "用法: mqf-shell file encode <mqf>"
    return
  if args[0] == "encode":
    echo "[FILE] " & encode_mqf_to_git(args[1..^1].join(" "))

proc route_browser(args: seq[string]) =
  if args.len < 2:
    echo "用法: mqf-shell browser encode <mqf-path>"
    return
  if args[0] == "encode":
    echo "[BROWSER] " & encode_uri_path(args[1])

# Demo
proc route_demo() =
  echo ""
  echo "蚕蛹多重壳聚合器 — 6壳全覆盖"
  echo ""
  echo "  git-shell    真实可用  mqf-shell git commit/log/branch-phase"
  echo "  api-shell    真实可用  mqf-shell api get/encode-path/adapt"
  echo "  cloud-shell  编码可用  mqf-shell cloud encode"
  echo "  browser-shell 编码可用 mqf-shell browser encode"
  echo "  file-shell   编码可用  mqf-shell file encode"
  echo "  ai-shell     编码可用  mqf-ai demo/decode/encode"
  echo ""
  echo "  编码共识: encode_mqf_to_git/decode_git_to_mqf"
  echo "  根基: 我=1(可见) 刘楚恬=0(隐藏)"
  echo "  容器: 254+2 编码 · 64进制 · 九元层级"

# 主路由
case shell_type
of "git":
  route_git(rest)
of "api":
  route_api(rest)
of "cloud":
  route_cloud(rest)
of "browser":
  route_browser(rest)
of "file":
  route_file(rest)
of "demo":
  route_demo()
else:
  echo "未知壳类型: " & shell_type
  echo "可用: git / api / cloud / browser / file / demo"
