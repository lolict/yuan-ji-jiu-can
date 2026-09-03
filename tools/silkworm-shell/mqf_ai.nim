# ============================================================
# mqf-ai — 蚕蛹 AI 壳 CLI
#
# 用法:
#   mqf-ai demo                            — 自演示
#   mqf-ai decode <llm-response>           — 解码LLM输出
#   mqf-ai encode <mqf-input>               — 编码提示
#   mqf-ai call <api_url> <model> <prompt>  — 调用LLM API
#   mqf-ai adapt <name> <url> <model> [key] — 适配盟机场
#   mqf-ai phase <level>                    — 设置九元层级 (1-9)
#   mqf-ai status                           — 茧状态报告
# ============================================================

import os, strutils
import ai_shell, ai_decode

let args = commandLineParams()
if args.len == 0:
  echo "mqf-ai — 蚕蛹 AI 壳 CLI"
  echo "用法:"
  echo "  mqf-ai demo"
  echo "  mqf-ai decode <llm-response>"
  echo "  mqf-ai encode <mqf-input>"
  echo "  mqf-ai call <api_url> <model> <prompt>"
  echo "  mqf-ai adapt <name> <url> <model> [key]"
  echo "  mqf-ai phase <level>"
  echo "  mqf-ai status"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

let shell = new_ai_shell()

case cmd
of "demo":
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║         蚕蛹 AI 壳 · 自演示                       ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""
  echo "  [九元层级→推理参数映射]"
  for lvl in 1..9:
    echo "    " & align($lvl, 1) & "元 → " &
         "temp=" & align($level_to_temperature(lvl), 3) &
         " · max_tokens=" & $level_to_max_tokens(lvl)
  echo ""
  echo "  [提示编码验证]"
  let test_prompts = @[
    "定义mlll根基符号",
    "创建gtt路由规则",
    "njsg共生关系如何运作"
  ]
  for p in test_prompts:
    echo "    输入: " & p
    echo "    编码: " & encode_prompt(p)
  echo ""
  echo "  [响应解码验证]"
  let test_responses = @[
    "ROOT是CONTAINER的根基，用于MUTUAL共生",
    "需要创建NAME和ROUTE规则",
    "SPACE天庭通过ENGINE引擎驱动UNIT袋子"
  ]
  for r in test_responses:
    echo "    原始: " & r
    echo "    解码: " & decode_llm_response(r)
  echo ""
  echo "  [System Prompt 注入]"
  echo "    " & shell.build_system_prompt()
  echo ""
  echo "  [盟机场适配演示]"
  discard shell.adapt_mengjichang("demo-local", "http://localhost:11434", "llama3")
  echo ""
  echo "  注: 真实 API 调用需要 LLM 服务在线。"
  echo "  配置环境变量 OLLAMA_HOST 或 OPENAI_API_KEY 可供调用。"

of "decode":
  if rest.len < 1:
    echo "用法: mqf-ai decode <llm-response>"
    quit(1)
  echo decode_llm_response(rest.join(" "))

of "encode":
  if rest.len < 1:
    echo "用法: mqf-ai encode <mqf-input>"
    quit(1)
  echo encode_prompt(rest.join(" "))

of "call":
  if rest.len < 3:
    echo "用法: mqf-ai call <api_url> <model> <prompt>"
    quit(1)
  let api_url = rest[0]
  let model = rest[1]
  let prompt = rest[2..^1].join(" ")
  let s = new_ai_shell(api_url, "", model)
  let resp = s.call_local(prompt)
  echo "Model: " & resp.model
  echo "Tokens: " & $resp.tokens_used
  echo "──"
  if resp.decoded_mqf.len > 0:
    echo resp.decoded_mqf
  else:
    echo resp.raw_text

of "adapt":
  if rest.len < 3:
    echo "用法: mqf-ai adapt <name> <url> <model> [key]"
    quit(1)
  let name = rest[0]
  let url = rest[1]
  let model = rest[2]
  let key = if rest.len > 3: rest[3] else: ""
  var s = new_ai_shell(url, key, model)
  discard s.adapt_mengjichang(name, url, model, key)

of "phase":
  if rest.len < 1:
    echo "用法: mqf-ai phase <level> (1-9)"
    quit(1)
  let lvl = parseInt(rest[0])
  if lvl < 1 or lvl > 9:
    echo "层级须在 1-9"
    quit(1)
  shell.phase_level = lvl
  echo "层级设置: " & $lvl & "元"
  echo "推理参数: temp=" & $level_to_temperature(lvl) &
       " · max_tokens=" & $level_to_max_tokens(lvl)

of "status":
  report(shell)

else:
  echo "未知命令: " & cmd
  echo "可用: demo / decode / encode / call / adapt / phase / status"
  quit(1)
