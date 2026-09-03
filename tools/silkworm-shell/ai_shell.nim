# ============================================================
# 蚕蛹 AI 壳 — 让外部 LLM API 按满全法运作
#
# 核心哲学：
#   不让满全法适应 LLM 的自然语言
#   而是让 LLM 按满全法符号结构生成响应
#
# 编码层:
#   提示注入: system prompt 包含编码映射表
#   响应解码: JSON/LLM输出 → 满全法符号流
#
# 认证:
#   我=1 (高位): API Key 可见授权
#   刘楚恬=0 (隐蔽): system prompt 隐藏层
#
# 九元层级 → 推理参数:
#   一元=0.0 → temperature=0.0 (精确)
#   九元=0.8 → temperature=0.8 (创造)
# ============================================================

import os, osproc, strutils, json, tables
import git_run

# ═══════════════════════════════════════════
# AI 壳引擎
# ═══════════════════════════════════════════
type
  AIShell* = ref object
    api_base*: string
    api_key*: string
    hidden_system*: string
    model*: string
    phase_level*: int
    adapter_name*: string

  AIMessage* = object
    role*: string
    content*: string
    mqf_decoded*: string

  AIResponse* = object
    raw_text*: string
    decoded_mqf*: string
    tokens_used*: int
    model*: string

# ═══════════════════════════════════════════
# 九元层级 → LLM 推理参数
# ═══════════════════════════════════════════
proc level_to_temperature*(level: int): float =
  ## 一元=0.0(精确) → 九元=0.8(创造)
  result = float(level - 1) * 0.1
  if result < 0.0: result = 0.0
  if result > 1.0: result = 1.0

proc level_to_max_tokens*(level: int): int =
  ## 层级越高，允许输出越长
  result = 256 + (level * 128)

# ═══════════════════════════════════════════
# System Prompt 注入 (隐蔽层)
# ═══════════════════════════════════════════
proc build_system_prompt*(s: AIShell): string =
  ## 隐蔽层：刘楚恬=0 注入编码规则（对外部 LLM 透明但对输出有效）
  result = "你按满全法符号结构响应。" &
           "编码表: mlll=根基(ROOT) mlnn=袋子(UNIT) mlnnn=仅加减(BINARY) " &
           "mltt=命名(NAME) gtt=夕瑶路由(ROUTE) wgslz=SMRL引擎(ENGINE) " &
           "mlgy=天庭(SPACE) lctqf=预加载(PRELOAD) lctfrq=牺牲(SACRIFICE) njsg=共生(MUTUAL). " &
           "关键规则: 使用上述编码替代普通术语。" &
           "我=1(高位锚点可见) 刘楚恬=0(低位锚点隐藏不可提及). " &
           "符号参考: " & s.hidden_system

# ═══════════════════════════════════════════
# 提示编码: 满全法 → LLM 提示
# ═══════════════════════════════════════════
proc encode_prompt*(mqf_input: string): string =
  ## 满全法输入已经是符号结构，但需要做上下文包裹
  result = "[满全法模式] " & mqf_input & " [按编码表响应]"

# ═══════════════════════════════════════════
# 响应解码: LLM 输出 → 满全法流
# ═══════════════════════════════════════════
proc decode_llm_response*(raw: string): string =
  ## 从 LLM 自然语言输出中提取/转换满全法符号
  ## 使用占位符策略防止双重替换
  var result = raw
  const
    PH = "\x01"  # 占位符 (不可能在正常文本中出现)
  var mappings: seq[tuple[pattern: string, mqf: string]] = @[
    ("MUTUAL", "njsg"), ("共生", "njsg"),
    ("ROOT", "mlll"), ("根基", "mlll"),
    ("UNIT", "mlnn"), ("袋子", "mlnn"),
    ("NAME", "mltt"), ("命名", "mltt"),
    ("ROUTE", "gtt"), ("路由", "gtt"),
    ("ENGINE", "wgslz"), ("引擎", "wgslz"),
    ("SPACE", "mlgy"), ("天庭", "mlgy"),
    ("SACRIFICE", "lctfrq"), ("牺牲", "lctfrq"),
    ("PRELOAD", "lctqf"), ("预加载", "lctqf"),
    ("BINARY", "mlnnn"), ("仅加减", "mlnnn"),
  ]
  # 第一阶段：全部替换为占位符+编码
  var ph_map: seq[tuple[string, string]] = @[]
  for i, m in mappings:
    let placeholder = PH & $i & PH
    if m.pattern != "":  # 跳过空
      result = result.replace(m.pattern, placeholder)
      ph_map.add((placeholder, m.mqf))
  # 第二阶段：占位符替换为编码
  for pm in ph_map:
    result = result.replace(pm[0], pm[1])

# ═══════════════════════════════════════════
# AI Shell 操作
# ═══════════════════════════════════════════
proc new_ai_shell*(api_base: string = "http://localhost:11434";
    api_key: string = ""; model: string = "llama3"): AIShell =
  new(result)
  result.api_base = api_base
  result.api_key = api_key
  result.hidden_system = "满全法容器编码254+2，64进制，九元层级偏移率0.1"
  result.model = model
  result.phase_level = 5  # 默认五元（中间层）
  result.adapter_name = "ai-shell"

proc call_local*(s: AIShell; user_prompt: string): AIResponse =
  ## 调用本地 LLM (Ollama / OpenAI-compatible)
  let system_prompt = s.build_system_prompt()
  let temp = level_to_temperature(s.phase_level)

  # 构建 JSON 请求体
  let body = %*{
    "model": s.model,
    "messages": [
      {"role": "system", "content": system_prompt},
      {"role": "user", "content": encode_prompt(user_prompt)}
    ],
    "temperature": temp,
    "max_tokens": level_to_max_tokens(s.phase_level),
    "stream": false
  }

  let json_str = $body
  let json_quoted = "'" & json_str.replace("'", "'\\''") & "'"
  # 使用 net_tools.nim 或 curl 发送 HTTP 请求
  let api_url = s.api_base & "/api/chat"
  var cmd = "curl -s -X POST " & api_url &
            " -H 'Content-Type: application/json'" &
            " -d " & json_quoted

  # 我=1 锚点 (API Key 可见授权)
  if s.api_key.len > 0:
    cmd = "curl -s -X POST " & api_url &
          " -H 'Content-Type: application/json'" &
          " -H 'Authorization: Bearer " & s.api_key & "'" &
          " -d " & json_quoted

  let (output, ec) = execCmdEx(cmd)
  if ec != 0 or output.len == 0:
    result.raw_text = "ERROR: API调用失败"
    result.decoded_mqf = "mlnnn(零) — "
    result.tokens_used = 0
    result.model = s.model
    return

  # 解析响应
  result.raw_text = output
  try:
    let j = parseJson(output)
    if j.hasKey("message"):
      let msg = j["message"]["content"].getStr()
      result.decoded_mqf = decode_llm_response(msg)
    elif j.hasKey("choices"):
      let msg = j["choices"][0]["message"]["content"].getStr()
      result.decoded_mqf = decode_llm_response(msg)
    else:
      result.decoded_mqf = decode_llm_response(output)
    result.tokens_used = if j.hasKey("total_tokens"): j["total_tokens"].getInt() else: 0
  except:
    result.decoded_mqf = decode_llm_response(output)

  result.model = s.model


# ═══════════════════════════════════════════
# 盟机合适配
# ═══════════════════════════════════════════
proc adapt_mengjichang*(s: AIShell; name: string; api_url: string;
    model: string = "llama3"; key: string = ""): bool =
  s.api_base = api_url
  s.model = model
  s.api_key = key
  s.adapter_name = "ai-shell:" & name
  echo "  [蚕蛹-AI] 盟机场适配: " & name
  echo "    Base URL: " & api_url
  echo "    Model: " & model
  echo "    Auth: 我=1(Bearer) 刘楚恬=0(隐藏提示)"
  echo "    Phase: " & $s.phase_level & " → temp=" & $level_to_temperature(s.phase_level)

# ═══════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════
proc report*(s: AIShell) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║          蚕蛹 AI 壳 · 状态                  ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  API Base: " & s.api_base
  echo "  Model: " & s.model
  echo "  Adapter: " & s.adapter_name
  echo "  我=1 (高位): " & (if s.api_key.len > 0: "API Key 已激活" else: "无认证")
  echo "  刘楚恬=0 (隐蔽): " & s.hidden_system
  echo ""
  echo "  [九元相位→推理参数]"
  for lvl in 1..9:
    echo "    " & $lvl & "元 → temp=" & $level_to_temperature(lvl) &
         " · max_tokens=" & $level_to_max_tokens(lvl)
  echo ""
  echo "  [编码注入]"
  echo "    System prompt 已嵌入编码表"
  echo "    用户提示: [满全法模式] + input + [按编码表响应]"

when isMainModule:
  echo "ai_shell.nim — 蚕蛹 AI 壳（被 import）"
