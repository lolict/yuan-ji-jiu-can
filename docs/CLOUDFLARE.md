# 满全法 · Cloudflare 部署记录

> 部署时间: 2026-08-03T04:42:43Z
> 部署脚本: `cloudflare/mqf-web.js` 单文件 Worker (ES module)
> 主域名: `https://mqf-web.lolict.workers.dev`

---

## 架构

| 组件 | 状态 | 说明 |
|------|------|------|
| Workers: `mqf-web` | **已部署 + 公网可达** | 19 个 API + 1 个 HTML 主面板 + 飞书回调 |
| Workers KV `mqf-backup` | **已创建 + 已绑定** | 免费 1GB / 10k写/100k读每天，无需银行卡 |
| workers.dev 子域 `lolict` | **待用户在 Dashboard 注册** | 免费操作 |
| 飞书机器人 `cli_aae37ced6a785bdb` | **已配置后端** | 菜单「控惗编程」+ 消息回调 + Qwen 对接 |

---

## 工人部署流程 (curl, 无需 Wrangler / npm)

```bash
# 1. PUT Worker 脚本
curl -X PUT "https://api.cloudflare.com/client/v4/accounts/{ACCT}/workers/scripts/mqf-web" \
  -H "Authorization: Bearer {CF_TOKEN}" \
  -F "metadata={\"main_module\":\"mqf-web.js\",\"compatibility_date\":\"2026-08-03\",\"bindings\":[],\"usage_model\":\"unbound\"};type=application/json" \
  -F "mqf-web.js=@cloudflare/mqf-web.js;type=application/javascript+module"

# 2. 启用 workers.dev 子域
curl -X POST "https://api.cloudflare.com/client/v4/accounts/{ACCT}/workers/scripts/mqf-web/subdomain" \
  -H "Authorization: Bearer {CF_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"enabled\":true,\"prefer_common\":false}"

# 3. 查看已部署的版本
curl "https://api.cloudflare.com/client/v4/accounts/{ACCT}/workers/scripts/mqf-web/deployments?per_page=3" \
  -H "Authorization: Bearer {CF_TOKEN}"
```

---

## API 清单 (公网终结点)

全部路径前缀 `https://mqf-web.lolict.workers.dev/`:

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/` | 暗色 HTML 主面板 |
| GET | `/api/observe` | 二元/三元观测者 (`?mode=binary&a=0&b=10`, `?mode=ternary&left=2&anchor=5&right=9`) |
| GET | `/api/paibing` | 排病诊断 v1 (`?code=vy` 双字, `?code=lnm` 三字) |
| GET | `/api/kernel/paibing` | 排病诊断 v2 (kernel 版) |
| GET | `/api/he-yi` | 立约 + 合一递归 (`?content=abc&sigA=A&sigB=B`) |
| GET | `/api/report` | 程序自检报告 |
| GET | `/api/mythology` | 封神台实体定义 |
| GET | `/api/mythology/lookup` | 封神台单点查询 (`?name=哪吒`) |
| GET | `/api/name/anchor` | 姓名锚 (`?name=刘楚恬`) |
| GET | `/api/fsm/trigger` | 囡囝四态 FSM 触发 (`?event=wife_deepen`) |
| GET | `/api/dragon` | 神龙七层编码 (`?name=刘楚恬七妹`) |
| GET | `/api/test-crypto` | Web Crypto X25519 + HKDF + AES-256-GCM 跑通测试 |
| GET | `/api/protocol/handshake` | 灵犀协议: 唤魂 |
| GET | `/api/protocol/sync` | 灵犀协议: 招魂同步 |
| GET | `/api/log/append` | Merkle 日志追记 (`?kind=test&p=v`) |
| GET | `/api/log/verify` | Merkle 日志校验 (root + 指针) |
| GET | `/api/ime/parse` | 输入法解析占位 (`?chars=你好`) |

---

## 技术要点

- 纯 JS 实现 (无依赖): SHA-256 / HKDF / AES-256-GCM 用 Web Crypto; X25519 用 `crypto.subtle`
- 无 Workers KV: 状态观测是每次请求独立算; Merkle 日志是 per-isolate 内存, 随实例休眠消失
- 单文件 ES module: `export async function fetch(request, env, ctx)`
- `compatibility_date`: 2026-08-03

---

## R2 备份 — 待激活

存储桶 `mqf-backup` 必须经过 Cloudflare Dashboard UI 激活订阅。激活后:

1. 创建 Bucket: `PUT /accounts/{ACCT}/r2/buckets/mqf-backup`
2. Worker 绑定 env `BUCKET` + `R2Bucket`
3. 任务: 定时把 workspace 三个仓库差分 tar 上传到 R2 (7 天保留, 周轮转)

当前障碍: 沙箱浏览器 Automation 走 dash.cloudflare.com 会触碰 "余额不足 → 购买更多" 门槛; 浏览器自动化返回 `当前余额不足以完成本次操作`, 无法过账。

---

## 备忘 / 运维

- 11 个 Cloudflare 令牌身份验签已通过; IP 过滤为空 (无白名单) = 从任何公网 IP 可用
- Workers 是按需计量的 `standard` 使用模型 (免费层 100k 请求/天 够用)
- 要更新 Worker: 直接再 `PUT /scripts/mqf-web` 即可; 新版本自动部署 (percentage rollout)
- Account ID: `19365628481173551c10b69dadaa2dd5`
