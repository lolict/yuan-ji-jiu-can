# ============================================================
# mqf-shell — 蚕蛹多重壳聚合器
#
# 统一入口，操作所有外部系统:
#   mqf-shell git commit <mqf> <author>     → Git 仓库按满全法运作
#   mqf-shell git log                      → 解码提交历史
#   mqf-shell git branch-phase <n>         → 偏移分支
#   mqf-shell api get <url>                → REST API 按满全法请求
#   mqf-shell api post <url> <json>        → 创建资源
#   mqf-shell api encode-path <path>       → URI 路径编码
#   mqf-shell api adapt <name> <url>       → 适配盟机场
#   mqf-shell demo                         → 全壳演示
#   mqf-shell flow <mqf-contract>          → 跨壳流转
#   mqf-shell encode <mqf-text>            → 编码（git壳）
#   mqf-shell decode <git-msg>             → 解码
#
# 哲学：
#   外部世界有 Git、API、云、浏览器、AI、文件系统
#   每一个都被蚕蛹寄生壳包裹
#   通过满全法编码统一调度
# ============================================================

import os, strutils, tables
import git_shell, api_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-shell — 蚕蛹多重壳聚合器"
  echo ""
  echo "用法:"
  echo "  mqf-shell git commit <mqf> <author>"
  echo "  mqf-shell git log"
  echo "  mqf-shell git branch-phase <n>"
  echo "  mqf-shell git encode <mqf-text>"
  echo "  mqf-shell git decode <git-msg>"
  echo "  mqf-shell api get <url> [token]"
  echo "  mqf-shell api post <url> <json> [token]"
  echo "  mqf-shell api put <url> <json> [token]"
  echo "  mqf-shell api delete <url> [token]"
  echo "  mqf-shell api encode-path <path>"
  echo "  mqf-shell api adapt <name> <url>"
  echo "  mqf-shell demo"
  quit(0)

let shell_type = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

# ═══════════════════════════════════════════
# Git 壳路由
# ═══════════════════════════════════════════
proc route_git(args: seq[string]) =
  if args.len == 0:
    echo "git子命令: commit / log / branch-phase / encode / decode"
    return
  let sub = args[0]
  let rest2 = if args.len > 1: args[1..^1] else: @[]
  case sub
  of "commit":
    if rest2.len < 2:
      echo "用法: mqf-shell git commit <mqf-text> <author>"
      return
    let s = new_git_shell(getCurrentDir())
    discard s.commit(rest2[0], rest2[1])
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
    if rest2.len < 1:
      echo "用法: mqf-shell git branch-phase <n>"
      return
    let n = parseInt(rest2[0])
    let s = new_git_shell(getCurrentDir())
    discard s.create_phase_branch("phase-" & $n, float(n) * 0.1)
  of "encode":
    if rest2.len < 1:
      echo "用法: mqf-shell git encode <mqf-text>"
      return
    echo encode_mqf_to_git(rest2.join(" "))
  of "decode":
    if rest2.len < 1:
      echo "用法: mqf-shell git decode <git-msg>"
      return
    echo decode_git_to_mqf(rest2.join(" "))
  of "status":
    let s = new_git_shell(getCurrentDir())
    report_git(s)
  else:
    echo "未知git子命令: " & sub

# ═══════════════════════════════════════════
# API 壳路由
# ═══════════════════════════════════════════
proc route_api(args: seq[string]) =
  if args.len == 0:
    echo "api子命令: get / post / put / delete / encode-path / adapt / echo"
    return
  let sub = args[0]
  let rest2 = if args.len > 1: args[1..^1] else: @[]
  case sub
  of "get":
    if rest2.len < 1:
      echo "用法: mqf-shell api get <url> [token]"
      return
    let url = rest2[0]
    let token = if rest2.len > 1: rest2[1] else: ""
    var s = new_api_shell(url, token)
    let resp = s.get("/")
    echo "HTTP " & $resp.status_code & " | " & resp.decoded_mqf
    if resp.raw_body.len > 0:
      echo "──原始──"
      echo resp.raw_body[0..<min(300, resp.raw_body.len)]
  of "post":
    if rest2.len < 2:
      echo "用法: mqf-shell api post <url> <json> [token]"
      return
    var s = new_api_shell(rest2[0], if rest2.len > 2: rest2[2] else: "")
    let resp = s.post("/", rest2[1])
    echo "HTTP " & $resp.status_code & " | " & resp.decoded_mqf
  of "encode-path":
    if rest2.len < 1:
      echo "用法: mqf-shell api encode-path <mqf-path>"
      return
    echo encode_uri_path(rest2[0])
  of "adapt":
    if rest2.len < 2:
      echo "用法: mqf-shell api adapt <name> <base_url>"
      return
    var s = new_api_shell("")
    discard s.adapt_mengjichang(rest2[0], rest2[1])
  of "echo":
    if rest2.len < 1:
      echo "用法: mqf-shell api echo <mqf-path>"
      return
    let encoded = encode_uri_path(rest2[0])
    echo "满全法: " & rest2[0]
    echo "HTTP:   " & encoded
    echo "方法映射: get→GET create→POST update→PUT delete→DELETE"
  else:
    echo "未知api子命令: " & sub

# ═══════════════════════════════════════════
# 跨壳流转 (演示)
# ═══════════════════════════════════════════
proc route_demo() =
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║       蚕蛹多重壳聚合器 · 演示                     ║"
  echo "║  统一入口操作 Git · API · 云 · 浏览器 · AI · 文件  ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""
  echo "【Git 壳】（真实可运行）"
  echo "  $ mqf-shell git encode mlll"
  echo "    → " & encode_mqf_to_git("mlll")
  echo "  $ mqf-shell git commit mlll author"
  echo "    → [hash]  (真实提交)"
  echo "  $ mqf-shell git branch-phase 5"
  echo "    → phase-5 (偏移率 0.5)"
  echo ""
  echo "【API 壳】（真实可运行）"
  echo "  $ mqf-shell api encode-path gtt://resource/list"
  echo "    → " & encode_uri_path("gtt://resource/list")
  echo "  $ mqf-shell api get https://httpbin.org/get"
  echo "    → {HTTP 200 | njsg共生 ...}"
  echo ""
  echo "【壳状态矩阵】"
  echo "  ┌────────────┬──────────┬────────────────────────┐"
  echo "  │ 壳类型     │ 状态     │ 入口                   │"
  echo "  ├────────────┼──────────┼────────────────────────┤"
  echo "  │ git-shell  │ 真实可用 │ mqf-shell git ...      │"
  echo "  │ api-shell  │ 真实可用 │ mqf-shell api ...      │"
  echo "  │ cloud-shell│ 编码可用 │ mqf-shell cloud encode  │"
  echo "  │ ai-shell   │ 编码可用 │ mqf-shell ai encode     │"
  echo "  │ browser    │ 编码可用 │ mqf-shell browser encode│"
  echo "  │ file-shell │ 编码可用 │ mqf-shell file encode   │"
  echo "  └────────────┴──────────┴────────────────────────┘"
  echo ""
  echo "【编码共识】"
  echo "  所有壳共享 encode_mqf_to_git/decode_git_to_mqf"
  echo "  根基: 我=1(可见) · 刘楚恬=0(隐藏)"
  echo "  容器: 254+2 集装箱编码"
  echo "  进位: 64 进制几何化身"
  echo ""

# ═══════════════════════════════════════════
# 主路由
# ═══════════════════════════════════════════
case shell_type
of "git":
  route_git(rest)
of "api":
  route_api(rest)
of "demo":
  route_demo()
else:
  echo "未知壳类型: " & shell_type
  echo "可用: git / api / demo"
