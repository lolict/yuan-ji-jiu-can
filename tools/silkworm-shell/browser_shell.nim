# ============================================================
# 蚕蛹 BROWSER 壳 — 浏览器按满全法运作
#
# 核心哲学:
#   不让满全法适应 DOM/HTML 语义
#   而是让浏览器行为按满全法编码表运作
#
# 执行层: 调用 `paw browser-action` (CatPaw 内置浏览器)
#   paw browser-action '<json>'
#   JSON: { "action": "open|click|type|screenshot|extract", ... }
#
# 满全法路由:
#   导航:  gtt://path → open {url}
#   点击:  :<selector>    → click {sel}
#   输入:  ~<text>         → type {text}
#   截屏:  :shot           → screenshot
#   提取:  :extract        → extract
#
# 命令:
#   mqf-browser open <mqf-url>
#   mqf-browser click <selector>
#   mqf-browser type <text>
#   mqf-browser shot [output]
#   mqf-browser extract
#   mqf-browser autofill <mqf-form>
#   mqf-browser demo
# ============================================================

import os, osproc, strutils, json, uri

const
  REPO_ROOT = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"

type
  BrowserShell* = ref object
    current_url*: string
    last_action*: string
    last_result*: string
    op_count*: int

# ═══════════════════════════════════════════
# URL 编码
# ═══════════════════════════════════════════
proc encode_browser_url*(mqf_url: string): string =
  if mqf_url.startsWith("gtt://"):
    # 满全法路由 → 转目标 URL
    let path = mqf_url[6..^1]
    if path.startsWith("mqf://"):
      # mqf://host:port/path → http://host:port/path
      result = "http://" & path[6..^1]
    else:
      result = "https://" & path
  elif mqf_url.startsWith("mqf://"):
    # mqf://host:port/path → http://host:port/path
    result = "http://" & mqf_url[6..^1]
  elif not mqf_url.startsWith("http"):
    result = "https://" & mqf_url
  else:
    result = mqf_url

# ═══════════════════════════════════════════
# paw browser-action 直接调用
# ═══════════════════════════════════════════
proc run_browser_action*(j: JsonNode): string =
  let cmd = "paw browser-action '" & $j & "'"
  let (output, ec) = execCmdEx(cmd)
  if ec == 0:
    result = output
  else:
    result = "paw-error(ec=" & $ec & "): " & output

# ═══════════════════════════════════════════
# BrowserShell 引擎
# ═══════════════════════════════════════════
proc new_browser_shell*(): BrowserShell =
  result = BrowserShell()

proc open*(s: BrowserShell; mqf_url: string): string =
  let url = encode_browser_url(mqf_url)
  let j = %* {"action": "open", "url": url}
  result = run_browser_action(j)
  s.current_url = url
  s.last_action = "open"
  s.last_result = result
  s.op_count += 1

proc click*(s: BrowserShell; selector: string): string =
  let j = %* {"action": "click", "selector": selector}
  result = run_browser_action(j)
  s.last_action = "click"
  s.last_result = result
  s.op_count += 1

proc type_text*(s: BrowserShell; text: string): string =
  let j = %* {"action": "type", "text": text}
  result = run_browser_action(j)
  s.last_action = "type"
  s.last_result = result
  s.op_count += 1

proc screenshot*(s: BrowserShell; output_path: string = ""): string =
  var j = %* {"action": "screenshot"}
  if output_path.len > 0:
    j["output"] = %* output_path
  result = run_browser_action(j)
  s.last_action = "screenshot"
  s.last_result = result
  s.op_count += 1

proc extract*(s: BrowserShell; selector: string = ""): string =
  var j = %* {"action": "extract"}
  if selector.len > 0:
    j["selector"] = %* selector
  result = run_browser_action(j)
  s.last_action = "extract"
  s.last_result = result
  s.op_count += 1

proc autofill*(s: BrowserShell; mqf_form: JsonNode): string =
  ## 按满全法表单定义自动填写
  ## mqf_form: { "form#id": [ {"input@name": "value"}, ... ] }
  var actions = newJArray()
  for selector, fields in mqf_form.pairs:
    for field in fields.items:
      for field_name, field_val in field.pairs:
        let action = %* {
          "action": "fill",
          "selector": selector & " " & field_name,
          "value": field_val.getStr()
        }
        actions.add(action)
  var all_outputs: seq[string] = @[]
  for act in actions.items:
    all_outputs.add(run_browser_action(act))
  result = all_outputs.join(" | ")
  s.last_action = "autofill"
  s.last_result = result
  s.op_count += actions.len

proc status*(s: BrowserShell): string =
  result = "BROWSER壳 url=" & s.current_url & " ops=" & $s.op_count
  s.last_result = result

proc report*(s: BrowserShell) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║           蚕蛹 BROWSER 壳 · 状态             ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  当前 URL: " & s.current_url
  echo "  ops: " & $s.op_count
  echo "  最近操作: " & s.last_action
  echo "  最近结果: " & s.last_result
  echo ""
  echo "  [路由规则]"
  echo "    gtt://path      → https://path"
  echo "    mqf://host/path → http://host/path"
  echo "    :sel            → click sel"
  echo "    ~text           → type text"
  echo "    :shot           → screenshot"
  echo "    :extract        → extract DOM"

when isMainModule:
  echo "browser_shell.nim — 被 mqf-browser 路由调用"
