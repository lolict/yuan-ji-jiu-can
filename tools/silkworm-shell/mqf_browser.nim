# ============================================================
# mqf-browser CLI — 蚕蛹 BROWSER 壳入口
#
# 注意: paw browser-action 需要 CatPaw 运行时
#       在 headless 环境(如 nim 编译容器)中 dry-run 返回编码指令
#       在有 paw 的环境(如 CatPaw UI)中执行真实浏览器操作
# ============================================================

import os, osproc, strutils, json
import browser_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-browser — 蚕蛹 BROWSER 壳"
  echo "用法: mqf-browser <open|click|type|shot|extract|autofill|demo> [args]"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

var br = new_browser_shell()

# 检查 paw 是否可用（headless 环境返回 dry-run）
let (paw_check, paw_ec) = execCmdEx("which paw 2>/dev/null")
let paw_available = paw_ec == 0

if not paw_available:
  echo "[BROWSER壳] dry-run 模式 (paw 未安装，仅编码路由)"
  echo ""

case cmd
of "open":
  if rest.len < 1:
    echo "用法: mqf-browser open <mqf-url>"
    quit(1)
  let result = br.open(rest[0])
  if paw_available:
    echo result
  else:
    echo "dry-run open: " & rest[0]
    echo "  → url: " & br.current_url

of "click":
  if rest.len < 1:
    echo "用法: mqf-browser click <selector>"
    quit(1)
  let result = br.click(rest[0])
  if paw_available:
    echo result
  else:
    echo "dry-run click: selector=" & rest[0]

of "type":
  if rest.len < 1:
    echo "用法: mqf-browser type <text>"
    quit(1)
  let result = br.type_text(rest[0])
  if paw_available:
    echo result
  else:
    echo "dry-run type: " & rest[0]

of "shot":
  let result = br.screenshot(if rest.len > 0: rest[0] else: "")
  if paw_available:
    echo result
  else:
    echo "dry-run screenshot: output=" & (if rest.len > 0: rest[0] else: "clipboard")

of "extract":
  let sel = if rest.len > 0: rest[0] else: ""
  let result = br.extract(sel)
  if paw_available:
    echo result
  else:
    echo "dry-run extract: selector=" & (if sel.len > 0: sel else: "body")

of "autofill":
  # 简单表单: mqf-browser autofill form#id input@name=value
  if rest.len < 2:
    echo "用法: mqf-browser autofill <form-selector> <input@name=value> ..."
    quit(1)
  let form_sel = rest[0]
  var fields = newJObject()
  for i in 1..<rest.len:
    let parts = rest[i].split("=", maxsplit = 1)
    if parts.len == 2:
      fields[parts[0]] = %* parts[1]
  let form = %* { form_sel: fields }
  let result = br.autofill(form)
  if paw_available:
    echo result
  else:
    echo "dry-run autofill: " & $form

of "demo":
  echo ""
  echo "蚕蛹 BROWSER 壳 — demo (dry-run)"
  echo ""
  echo "  open gtt://example.com:"
  echo "    → " & encode_browser_url("gtt://example.com")
  echo "  open mqf://localhost:8080/api/v1/status:"
  echo "    → " & encode_browser_url("mqf://localhost:8080/api/v1/status")
  echo "  encode-path gtt://mlll/ROOT"
  echo "    → /gtt/mlll/ROOT"
  echo ""
  echo "  [路由规则]"
  echo "    gtt://path         → https://path"
  echo "    mqf://host/path    → http://host/path"
  echo "    :selector          → click"
  echo "    ~text              → type"
  echo "    :shot              → screenshot"
  echo ""
  if not paw_available:
    echo "  (当前 dry-run — 在 CatPaw UI 中执行真实浏览器操作)"

else:
  echo "未知命令: " & cmd
  echo "可用: open / click / type / shot / extract / autofill / demo"
