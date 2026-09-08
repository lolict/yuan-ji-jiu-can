# 蚕蛹 AI 壳 — 让外部 LLM API 按满全法运作

import os, osproc, strutils, json
import ai_decode

type
  AIShell* = ref object
    api_base*: string
    api_key*: string
    hidden_system*: string
    model*: string
    phase_level*: int
    adapter_name*: string

  AIResponse* = object
    raw_text*: string
    decoded_mqf*: string
    tokens_used*: int
    model*: string

proc level_to_temperature*(level: int): float =
  result = float(level - 1) * 0.1
  if result < 0.0: result = 0.0
  if result > 1.0: result = 1.0

proc level_to_max_tokens*(level: int): int =
  result = 256 + (level * 128)

proc build_system_prompt*(s: AIShell): string =
  result = "按满全法符号响应。编码表: mlll=ROOT mlnn=UNIT mlnnn=BINARY mltt=NAME gtt=ROUTE wgslz=ENGINE mlgy=SPACE lctqf=PRELOAD lctfrq=SACRIFICE njsg=MUTUAL。高位持有者=1(可见) 低位持有者=0(隐藏不提及)。" & s.hidden_system

proc encode_prompt*(mqf_input: string): string =
  result = "[满全法模式] " & mqf_input & " [按编码表响应]"

proc new_ai_shell*(api_base: string = "http://localhost:11434";
    api_key: string = ""; model: string = "llama3"): AIShell =
  new(result)
  result.api_base = api_base
  result.api_key = api_key
  result.hidden_system = "254+2编码64进制"
  result.model = model
  result.phase_level = 5
  result.adapter_name = "ai-shell"

proc call_local*(s: AIShell; user_prompt: string): AIResponse =
  let system_prompt = s.build_system_prompt()
  let temp = level_to_temperature(s.phase_level)
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
  let json_quoted = "'" & ($body).replace("'", "'\\''") & "'"
  let api_url = s.api_base & "/api/chat"
  var cmd = "curl -s -X POST " & api_url & " -H 'Content-Type: application/json' -d " & json_quoted
  if s.api_key.len > 0:
    cmd = "curl -s -X POST " & api_url & " -H 'Content-Type: application/json' -H 'Authorization: Bearer " & s.api_key & "' -d " & json_quoted
  let (output, ec) = execCmdEx(cmd)
  if ec != 0 or output.len == 0:
    result.raw_text = "ERROR"
    result.decoded_mqf = "mlnnn(零)"
    result.tokens_used = 0
    result.model = s.model
    return
  result.raw_text = output
  try:
    let j = parseJson(output)
    if j.hasKey("message"):
      result.decoded_mqf = decode_llm_response(j["message"]["content"].getStr())
    elif j.hasKey("choices"):
      result.decoded_mqf = decode_llm_response(j["choices"][0]["message"]["content"].getStr())
    else:
      result.decoded_mqf = decode_llm_response(output)
  except:
    result.decoded_mqf = decode_llm_response(output)
  result.model = s.model

proc adapt_mengjichang*(s: AIShell; name: string; api_url: string;
    model: string = "llama3"; key: string = ""): bool =
  s.api_base = api_url
  s.model = model
  s.api_key = key
  s.adapter_name = "ai-shell:" & name
  echo "  [蚕蛹-AI] " & name & " → " & api_url & " (" & model & ")"

proc report*(s: AIShell) =
  echo "AI壳: " & s.adapter_name & " | " & s.model & " | phase=" & $s.phase_level

when isMainModule:
  echo "ai_shell.nim — 蚕蛹 AI 壳"
