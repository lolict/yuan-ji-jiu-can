# ============================================================
# mqf-shell — 蚕蛹多重壳聚合器
#
# 统一入口，操作所有外部系统:
#   mqf-shell git commit/log/branch-phase
#   mqf-shell api get/put/encode-path/adapt
#   mqf-shell cloud put/get/list/sync/status
#   mqf-shell file write/read/seal/tag/list/metadata
#   mqf-shell browser open/click/type/shot/extract/autofill
#   mqf-shell demo
#
# 编码共识: encode_mqf_to_git/decode_git_to_mqf
# ============================================================

import os, strutils
import git_shell, api_shell, cloud_shell, file_shell, browser_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-shell — 蚕蛹多重壳聚合器"
  echo "用法: mqf-shell <git|api|cloud|file|browser|demo> [args]"
  quit(0)

let shell_type = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

# Git 壳路由
proc route_git(gargs: seq[string]) =
  if gargs.len == 0:
    echo "git子命令: commit / log / branch-phase / encode / decode"
    return
  let sub = gargs[0]
  let r = if gargs.len > 1: gargs[1..^1] else: @[]
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
      echo "用法: mqf-shell git branch-phase <n> (n=0..8)"
      return
    let n = parseInt(r[0])
    if n < 0 or n > 8:
      echo "n 须在 0..8"
      return
    let s = new_git_shell(getCurrentDir())
    discard s.create_phase_branch("mqf-phase-" & $n, float(n) * 0.1)
    echo "分支 mqf-phase-" & $n & " 已创建"
  of "encode":
    if r.len < 1:
      echo "用法: mqf-shell git encode <mqf>"
      return
    echo encode_mqf_to_git(r.join(" "))
  of "decode":
    if r.len < 1:
      echo "用法: mqf-shell git decode <encoded>"
      return
    echo decode_git_to_mqf(r.join(" "))
  else:
    echo "未知git子命令: " & sub

# API 壳路由
proc route_api(aargs: seq[string]) =
  if aargs.len == 0:
    echo "api子命令: get|post / encode-path / adapt / report"
    return
  case aargs[0]
  of "get", "post", "put", "delete":
    if aargs.len < 3:
      echo "用法: mqf-shell api <method> <base-url> <path>"
      return
    let s = new_api_shell(aargs[1])
    let path = aargs[2]
    var resp: ApiResponse
    case aargs[0]
    of "get":    resp = s.get(path)
    of "post":   resp = s.post(path, if aargs.len > 3: aargs[3] else: "{}")
    of "put":    resp = s.put(path, if aargs.len > 3: aargs[3] else: "{}")
    of "delete": resp = s.delete(path)
    else: discard
    echo "HTTP " & $resp.status_code & " · " & resp.decoded_mqf
    echo resp.raw_body[0..<min(200, resp.raw_body.len)]
  of "encode-path":
    if aargs.len < 2:
      echo "用法: mqf-shell api encode-path <mqf-path>"
      return
    echo encode_uri_path(aargs[1])
  of "adapt":
    if aargs.len < 3:
      echo "用法: mqf-shell api adapt <name> <url>"
      return
    let s = new_api_shell("")
    discard s.adapt_mengjichang(aargs[1], aargs[2])
  of "report":
    let s = new_api_shell(getCurrentDir())
    s.report()
  else:
    echo "未知api子命令: " & aargs[0]

# Cloud 壳路由（真实）
proc route_cloud(cargs: seq[string]) =
  if cargs.len == 0:
    echo "cloud子命令: put|get|list|sync|status|encode"
    return
  var cloud = new_cloud_shell()
  case cargs[0]
  of "put":
    if cargs.len < 3:
      echo "用法: mqf-shell cloud put <local> <mqf-remote>"
      return
    discard cloud.put(cargs[1], cargs[2])
    echo cloud.last_result
  of "get":
    if cargs.len < 3:
      echo "用法: mqf-shell cloud get <mqf-remote> <local>"
      return
    discard cloud.get(cargs[1], cargs[2])
    echo cloud.last_result
  of "list":
    let prefix = if cargs.len > 1: cargs[1] else: ""
    let objs = cloud.list(prefix)
    if objs.len == 0:
      echo "(空)"
    for o in objs:
      echo "  " & o.name & " (" & $o.size & "B)"
  of "sync":
    if cargs.len < 2:
      echo "用法: mqf-shell cloud sync <local-dir>"
      return
    let r = cloud.sync(cargs[1])
    echo "sync: ↑" & $r.uploaded & " ↓" & $r.downloaded
  of "status":
    echo cloud.status()
  of "encode":
    if cargs.len < 2:
      echo "用法: mqf-shell cloud encode <mqf-path>"
      return
    echo encode_cloud_path(cargs[1])
  else:
    echo "未知cloud子命令: " & cargs[0]

# File 壳路由（真实）
proc route_file(fargs: seq[string]) =
  if fargs.len == 0:
    echo "file子命令: write|read|seal|tag|list|metadata"
    return
  var fs = new_file_shell()
  case fargs[0]
  of "write":
    if fargs.len < 3:
      echo "用法: mqf-shell file write <mqf-path> <content>"
      return
    discard fs.write(fargs[1], fargs[2..^1].join(" "))
    echo fs.last_result
  of "read":
    if fargs.len < 2:
      echo "用法: mqf-shell file read <mqf-path>"
      return
    let c = fs.read(fargs[1])
    if c.len > 0: echo c
    else: echo fs.last_result
  of "seal":
    if fargs.len < 2:
      echo "用法: mqf-shell file seal <mqf-path>"
      return
    let container = if fargs.len > 2: fargs[2] else: "254+2-libre"
    discard fs.seal(fargs[1], container)
    echo fs.last_result
  of "tag":
    if fargs.len < 3:
      echo "用法: mqf-shell file tag <mqf-path> <tag>"
      return
    discard fs.tag(fargs[1], fargs[2])
    echo fs.last_result
  of "list":
    let files = fs.list(if fargs.len > 1: fargs[1] else: "")
    if files.len == 0: echo "(空)"
    for f in files: echo "  " & f
  of "metadata":
    if fargs.len < 2:
      echo "用法: mqf-shell file metadata <mqf-path>"
      return
    let m = fs.metadata(fargs[1])
    if m.mqf_tag.len > 0:
      echo "tag:   " & m.mqf_tag
      echo "author: " & m.author
      echo "sealed: " & $m.sealed
  else:
    echo "未知file子命令: " & fargs[0]

# Browser 壳路由（真实）
proc route_browser(bargs: seq[string]) =
  if bargs.len == 0:
    echo "browser子命令: open|click|type|shot|extract|autofill"
    return
  var br = new_browser_shell()
  case bargs[0]
  of "open":
    if bargs.len < 2:
      echo "用法: mqf-shell browser open <mqf-url>"
      return
    discard br.open(bargs[1])
    echo "→ " & br.current_url
  of "click":
    if bargs.len < 2:
      echo "用法: mqf-shell browser click <selector>"
      return
    discard br.click(bargs[1])
  of "type":
    if bargs.len < 2:
      echo "用法: mqf-shell browser type <text>"
      return
    discard br.type_text(bargs[1])
  of "shot":
    discard br.screenshot(if bargs.len > 1: bargs[1] else: "")
    echo br.last_result
  of "extract":
    discard br.extract(if bargs.len > 1: bargs[1] else: "")
    echo br.last_result
  of "autofill":
    echo "用法: mqf-shell browser autofill <form#id> (在 CatPaw UI 运行)"
  else:
    echo "未知browser子命令: " & bargs[0]

# Demo
proc route_demo() =
  echo ""
  echo "蚕蛹多重壳聚合器 — 6壳全覆盖 (真实执行)"
  echo ""
  echo "  git-shell     真实可用  mqf-shell git commit/log/branch-phase/encode"
  echo "  api-shell     真实可用  mqf-shell api get/post/put/encode-path/adapt"
  echo "  cloud-shell   真实可用  mqf-shell cloud put/get/list/sync/status"
  echo "  file-shell    真实可用  mqf-shell file write/read/seal/tag/list"
  echo "  browser-shell 真实可用  mqf-shell browser open/click/type/shot/extract"
  echo "  ai-shell      编码可用  mqf-ai encode/decode"
  echo ""
  echo "  [快速测试]"
  # 测试 encode 依然工作
  echo "  git encode 'mlll njsg共生' → " & encode_mqf_to_git("mlll njsg共生")
  echo "  cloud encode cloud://mlll/ROOT → " & encode_cloud_path("cloud://mlll/ROOT")
  echo "  browser encode gtt://test → " & encode_browser_url("gtt://test")
  echo "  file encode mlll:doc → ./.mqfmeta/mlll/doc"
  echo ""
  echo "  根基: 高位持有者=1(可见) 低位持有者=0(隐藏)"
  echo "  容器: 254+2 编码 · 64进制 · 九元层级"

# 主路由
case shell_type
of "git":
  route_git(rest)
of "api":
  route_api(rest)
of "cloud":
  route_cloud(rest)
of "file":
  route_file(rest)
of "browser":
  route_browser(rest)
of "demo":
  route_demo()
else:
  echo "未知壳类型: " & shell_type
  echo "可用: git / api / cloud / file / browser / demo"
