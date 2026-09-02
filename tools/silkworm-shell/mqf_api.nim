# ============================================================
# mqf-api — 蚕蛹 API 壳 CLI
#
# 用法:
#   mqf-api get <url> [auth_token]
#   mqf-api post <url> <json-body> [auth_token]
#   mqf-api put <url> <json-body> [auth_token]
#   mqf-api delete <url> [auth_token]
#   mqf-api adapt <name> <base_url>
#   mqf-api encode-path <mqf-path>
#   mqf-api status
#   mqf-api demo
# ============================================================

import os, strutils
import api_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-api — 蚕蛹 API 壳 CLI"
  echo "用法: mqf-api <get|post|put|delete|adapt|encode-path|status|demo>"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

case cmd
of "get":
  if rest.len < 1:
    echo "用法: mqf-api get <url> [token]"
    quit(1)
  let url = rest[0]
  let token = if rest.len > 1: rest[1] else: ""
  var shell = new_api_shell(url, token)
  let resp = shell.get("/")
  echo "状态: " & $resp.status_code
  echo "满全法: " & resp.decoded_mqf
  echo "原始: " & resp.raw_body[0..<min(200, resp.raw_body.len)]
of "post":
  if rest.len < 2:
    echo "用法: mqf-api post <url> <json> [token]"
    quit(1)
  let url = rest[0]
  let body = rest[1]
  let token = if rest.len > 2: rest[2] else: ""
  var shell = new_api_shell(url, token)
  let resp = shell.post("/", body)
  echo "状态: " & $resp.status_code
  echo "满全法: " & resp.decoded_mqf
of "encode-path":
  if rest.len < 1:
    echo "用法: mqf-api encode-path <mqf-path>"
    quit(1)
  echo encode_uri_path(rest[0])
of "adapt":
  if rest.len < 2:
    echo "用法: mqf-api adapt <name> <base_url>"
    quit(1)
  var shell = new_api_shell("")
  discard shell.adapt_mengjichang(rest[0], rest[1])
of "echo":
  # 本地 echo 测试，不需要外部 API
  if rest.len < 1:
    echo "用法: mqf-api echo <mqf-path>"
    quit(1)
  let encoded = encode_uri_path(rest[0])
  echo "满全法路径: " & rest[0]
  echo "HTTP路径:  " & encoded
  echo "方法映射表已就绪"
of "demo":
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║         蚕蛹 API 壳 · 自演示                     ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""
  echo "  [方法映射验证]"
  echo "    get    → " & method_to_http(mqf_symbol_to_method("get"))
  echo "    获取   → " & method_to_http(mqf_symbol_to_method("获取"))
  echo "    gtt    → " & method_to_http(mqf_symbol_to_method("gtt"))
  echo "    create → " & method_to_http(mqf_symbol_to_method("create"))
  echo ""
  echo "  [URI编码验证]"
  let test_paths = @["gtt://container/list", "mqf://user/profile", "simple/path"]
  for p in test_paths:
    echo "    " & p & " → " & encode_uri_path(p)
  echo ""
  echo "  [请求体编码]"
  let fields: seq[tuple[key: string, val: string]] = @[
    ("mlll", "根基值"), ("mltt", "名称值")
  ]
  echo "    " & encode_body(fields)
  echo ""
  echo "  [盟机场适配]"
  var s = new_api_shell("")
  discard s.adapt_mengjichang("demo-api", "https://httpbin.org")
  echo ""
  echo "  [认证结构]"
  echo "    我=1 (高位锚点, Bearer可见): ACTIVE"
  echo "    刘楚恬=0 (隐蔽锚点, X-MQF-Fingerprint): HIDDEN"
  echo ""
  echo "  注: 真实HTTP请求需要外部API在线。自演示仅验证编码层。"
of "status":
  echo "API 壳就绪。使用 mqf-api <cmd> 操作。"
else:
  echo "未知命令: " & cmd
  quit(1)
