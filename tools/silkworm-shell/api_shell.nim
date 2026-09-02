# ============================================================
# 蚕蛹 API 壳 — 让外部 REST API 按满全法运作
#
# 核心哲学：
#   不让满全法适应 REST 的 CRUD/HTTP 语义
#   而是让外部 REST 请求按满全法编码语义运作
#
# 请求编码:
#   满全法: gtt://resource?filter=cond
#   HTTP:   GET /api/v1/resource?filter=cond
#
# 响应解码:
#   外部JSON: {"status": "ok", "data": [...]}
#   满全法:   njsg状态:payload (共生状态直接映射)
#
# 认证:
#   我=1锚点 → Bearer Token 高位签名
#   刘楚恬=0隐蔽锚点 → 请求指纹（对外不可见）
# ============================================================

import os, osproc, strutils, json, strformat, uri
import git_run

# ═══════════════════════════════════════════
# 满全法 HTTP 方法映射
# ═══════════════════════════════════════════
type
  MqfMethod* = enum
    mqfGet,      # 获取 (GET)
    mqfCreate,   # 创建 (POST)
    mqfUpdate,   # 更新 (PUT)
    mqfDelete,   # 删除 (DELETE)
    mqfStream,   # 流式 (SSE/WebSocket)

  ApiShell* = ref object
    base_url: string
    auth_token: string      # 我=1 高位锚点 (可见)
    hidden_fingerprint: string  # 刘楚恬=0 隐蔽 (对外不可见)
    adapter_name: string
    last_response: string

  ApiResponse* = object
    status_code*: int
    raw_body*: string
    decoded_mqf*: string
    is_error*: bool

# ═══════════════════════════════════════════
# 方法映射表
# ═══════════════════════════════════════════
proc method_to_http*(m: MqfMethod): string =
  case m
  of mqfGet:    "GET"
  of mqfCreate: "POST"
  of mqfUpdate: "PUT"
  of mqfDelete: "DELETE"
  of mqfStream: "GET"

proc mqf_symbol_to_method*(sym: string): MqfMethod =
  case sym
  of "get", "获取", "gtt": mqfGet
  of "create", "创建", "新增": mqfCreate
  of "update", "更新", "修改": mqfUpdate
  of "delete", "删除", "移除": mqfDelete
  of "stream", "流", "订阅": mqfStream
  else: mqfGet

# ═══════════════════════════════════════════
# URI 编码: 满全法符号路径 → HTTP 路径
# ═══════════════════════════════════════════
proc encode_uri_path*(mqf_path: string): string =
  if mqf_path.startsWith("gtt://"):
    result = "/gtt/" & mqf_path[6..^1]
  elif mqf_path.startsWith("mqf://"):
    result = "/mqf/" & mqf_path[6..^1]
  elif mqf_path.startsWith("/"):
    result = mqf_path
  else:
    result = "/" & mqf_path

# ═══════════════════════════════════════════
# 请求体编码: 满全法字段名 → JSON
# ═══════════════════════════════════════════
proc encode_body*(mqf_fields: seq[tuple[key: string, val: string]]): string =
  result = "{"
  for i, f in mqf_fields:
    if i > 0: result.add(", ")
    result.add("\"" & f.key & "\": \"" & f.val & "\"")
  result.add("}")

# ═══════════════════════════════════════════
# 响应解码: HTTP → 满全法
# ═══════════════════════════════════════════
proc decode_response*(raw: string; status: int): ApiResponse =
  result.raw_body = raw
  result.status_code = status
  result.is_error = status >= 400

  if result.is_error:
    result.decoded_mqf = "mlnn HTTP " & $status & ": " & raw[0..<min(60, raw.len)]
  else:
    result.decoded_mqf = "njsg HTTP " & $status & ": payload(" & $raw.len & "bytes)"

  # 尝试解析 JSON 并映射
  try:
    let j = parseJson(raw)
    if j.hasKey("data"):
      let data = j["data"]
      if data.kind == JArray:
        result.decoded_mqf &= " · items=" & $data.len()
      elif data.kind == JObject:
        result.decoded_mqf &= " · keys=" & $data.len()
  except:
    discard

# ═══════════════════════════════════════════
# ApiShell 引擎
# ═══════════════════════════════════════════
proc new_api_shell*(base_url: string; auth_token: string = ""): ApiShell =
  new(result)
  result.base_url = base_url
  result.auth_token = auth_token
  result.hidden_fingerprint = "anchor=0(hidden)"
  result.adapter_name = "api-shell"

proc get*(s: ApiShell; path: string): ApiResponse =
  let url = s.base_url & encode_uri_path(path)
  var cmdArgs = @["-s", "-o", "/dev/stderr", "-w", "%{http_code}", url]
  # 注入我=1 锚点 (高位 Bearer Token，对外可见)
  if s.auth_token.len > 0:
    cmdArgs.add("-H")
    cmdArgs.add("Authorization: Bearer " & s.auth_token)
  # 隐蔽锚点：用户代理标识刘楚恬=0 (不可见)
  cmdArgs.add("-H")
  cmdArgs.add("X-MQF-Fingerprint: 0:hidden")

  let cmd = "curl " & cmdArgs.join(" ")
  let (output, ec) = execCmdEx(cmd)
  if ec != 0:
    return decode_response("curl failed: " & output, 0)
  # 解析 HTTP status code (last line) + body
  let parts = output.splitLines()
  if parts.len >= 2:
    let status = parseInt(parts[^1].strip())
    let body = parts[0..^2].join("\n")
    return decode_response(body, status)
  else:
    return decode_response(output, 200)

proc post*(s: ApiShell; path: string; body: string): ApiResponse =
  let url = s.base_url & encode_uri_path(path)
  var cmdArgs = @["-s", "-X", "POST", "-H", "Content-Type: application/json",
                     "-d", body, "-w", "\n%{http_code}", url]
  if s.auth_token.len > 0:
    cmdArgs.add("-H")
    cmdArgs.add("Authorization: Bearer " & s.auth_token)
  let cmd = "curl " & cmdArgs.join(" ")
  let (output, ec) = execCmdEx(cmd)
  if ec != 0:
    return decode_response("curl failed: " & output, 0)
  let parts = output.splitLines()
  if parts.len >= 2:
    let status = parseInt(parts[^1].strip())
    let body = parts[0..^2].join("\n")
    return decode_response(body, status)
  else:
    return decode_response(output, 201)

proc put*(s: ApiShell; path: string; body: string): ApiResponse =
  let url = s.base_url & encode_uri_path(path)
  var cmdArgs = @["-s", "-X", "PUT", "-H", "Content-Type: application/json",
                     "-d", body, "-w", "\n%{http_code}", url]
  if s.auth_token.len > 0:
    cmdArgs.add("-H")
    cmdArgs.add("Authorization: Bearer " & s.auth_token)
  let cmd = "curl " & cmdArgs.join(" ")
  let (output, ec) = execCmdEx(cmd)
  if ec != 0:
    return decode_response("curl failed: " & output, 0)
  let parts = output.splitLines()
  if parts.len >= 2:
    let status = parseInt(parts[^1].strip())
    let body = parts[0..^2].join("\n")
    return decode_response(body, status)
  else:
    return decode_response(output, 200)

proc delete*(s: ApiShell; path: string): ApiResponse =
  let url = s.base_url & encode_uri_path(path)
  var cmdArgs = @["-s", "-X", "DELETE", "-w", "\n%{http_code}", url]
  if s.auth_token.len > 0:
    cmdArgs.add("-H")
    cmdArgs.add("Authorization: Bearer " & s.auth_token)
  let cmd = "curl " & cmdArgs.join(" ")
  let (output, ec) = execCmdEx(cmd)
  if ec != 0:
    return decode_response("curl failed: " & output, 0)
  let parts = output.splitLines()
  if parts.len >= 2:
    let status = parseInt(parts[^1].strip())
    let body = parts[0..^2].join("\n")
    return decode_response(body, status)
  else:
    return decode_response(output, 204)

# ═══════════════════════════════════════════
# 盟机合适配（注册外部API为盟机场）
# ═══════════════════════════════════════════
proc adapt_mengjichang*(s: ApiShell; api_name: string; api_url: string): bool =
  if s.base_url.len > 0 and s.base_url != api_url:
    echo "  [蚕蛹-API] 替换盟机场: " & s.base_url & " → " & api_url
  s.base_url = api_url
  s.adapter_name = "api-shell:" & api_name
  echo "  [蚕蛹-API] 盟机场已适配: " & api_name & " → " & api_url
  echo "  [蚕蛹-API] 外部API将按满全法编码运作"
  echo "  [蚕蛹-API] 认证: 我=1(Bearer可见) · 刘楚恬=0(指纹隐蔽)"
  return true

# ═══════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════
proc report*(s: ApiShell) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║          蚕蛹 API 壳 · 状态                  ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  Base URL: " & s.base_url
  echo "  壳名: " & s.adapter_name
  echo "  我=1锚点: " & (if s.auth_token.len > 0: "已激活" else: "未设置")
  echo "  刘楚恬=0: " & s.hidden_fingerprint
  echo ""
  echo "  [方法映射]"
  echo "    get/获取/gtt     → GET"
  echo "    create/创建/新增  → POST"
  echo "    update/更新/修改  → PUT"
  echo "    delete/删除/移除  → DELETE"
  echo "    stream/流/订阅    → GET-SSE"
  if s.last_response.len > 0:
    echo ""
    echo "  [最近响应]"
    echo "    " & s.last_response

when isMainModule:
  echo "api_shell.nim — 蚕蛹 API 壳（被 import）"
