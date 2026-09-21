#!/usr/bin/env bash
# deploy.sh — Cloudflare Worker 部署脚本
# 用法: CF_API_TOKEN=xxx ./cloudflare/deploy.sh [worker名] [脚本文件]
#
# 前置条件:
#   1. CF_API_TOKEN 环境变量 (需 Workers Scripts:Edit 权限)
#   2. Account ID: 19365628481173551c10b69dadaa2dd5
#   3. 子域名: lolict (已在 Dashboard 注册)

set -euo pipefail

ACCT="19365628481173551c10b69dadaa2dd5"
WORKER_NAME="${1:-mqf-web}"
SCRIPT_FILE="${2:-cloudflare/mqf-web.js}"
SUBDOMAIN="lolict"

if [ -z "${CF_API_TOKEN:-}" ]; then
  echo "❌ 缺少 CF_API_TOKEN 环境变量"
  echo "   获取方式: Cloudflare Dashboard → My Profile → API Tokens → Create Token"
  echo "   模板: 'Edit Cloudflare Workers' (内置模板)"
  echo ""
  echo "   使用方法:"
  echo "   CF_API_TOKEN=your_token_here ./cloudflare/deploy.sh"
  exit 1
fi

if [ ! -f "$SCRIPT_FILE" ]; then
  echo "❌ 脚本文件不存在: $SCRIPT_FILE"
  exit 1
fi

SIZE=$(stat -c%s "$SCRIPT_FILE" 2>/dev/null || stat -f%z "$SCRIPT_FILE" 2>/dev/null)
echo "═══ Cloudflare Worker 部署 ═══"
echo "Worker: $WORKER_NAME"
echo "脚本: $SCRIPT_FILE ($SIZE bytes)"
echo "Account: $ACCT"
echo ""

# ── 1. PUT Worker 脚本 ──
echo "[1/3] 上传脚本..."
METADATA=$(printf '{"main_module":"%s","compatibility_date":"2026-09-21","compatibility_flags":["nodejs_compat"],"bindings":[],"usage_model":"unbound"}' "$WORKER_NAME")

RESP=$(curl -s -X PUT "https://api.cloudflare.com/client/v4/accounts/$ACCT/workers/scripts/$WORKER_NAME" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -F "metadata=$METADATA;type=application/json" \
  -F "$WORKER_NAME=@$SCRIPT_FILE;type=application/javascript+module")

SUCCESS=$(echo "$RESP" | python3 -c "import sys,json;print(json.load(sys.stdin)['success'])" 2>/dev/null || echo "false")
if [ "$SUCCESS" != "true" ]; then
  echo "❌ 上传失败: $RESP"
  exit 1
fi
echo "  ✅ 脚本上传成功"

# ── 2. 启用 workers.dev 子域 ──
echo "[2/3] 检查 workers.dev 子域..."
RESP=$(curl -s -X GET "https://api.cloudflare.com/client/v4/accounts/$ACCT/workers/subdomain" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json")
SUB_ENABLED=$(echo "$RESP" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d.get('result',{}).get('subdomain',''))" 2>/dev/null || echo "")

if [ -n "$SUB_ENABLED" ]; then
  echo "  ✅ 子域已注册: ${SUB_ENABLED}.workers.dev"
else
  echo "  ⚠️  子域未注册 — 需在 Dashboard 手动注册一次:"
  echo "     https://dash.cloudflare.com/$ACCT/workers/subdomain"
  echo "     (API 不支持注册子域 · 仅 Dashboard UI 可操作)"
fi

# ── 3. 验证部署 ──
echo "[3/3] 验证部署..."
RESP=$(curl -s "https://api.cloudflare.com/client/v4/accounts/$ACCT/workers/scripts/$WORKER_NAME/deployments?per_page=1" \
  -H "Authorization: Bearer $CF_API_TOKEN")
DEPLOYED=$(echo "$RESP" | python3 -c "import sys,json;d=json.load(sys.stdin);v=d['result'][0] if d.get('result') else {};print(v.get('version_id','?'), v.get('created_on','?'))" 2>/dev/null || echo "unknown")
echo "  部署版本: $DEPLOYED"

echo ""
echo "═══ 部署 URL ═══"
echo "  https://${WORKER_NAME}.${SUBDOMAIN}.workers.dev"
echo ""
echo "═══ 验证命令 ═══"
echo "  curl https://${WORKER_NAME}.${SUBDOMAIN}.workers.dev/api/report"
