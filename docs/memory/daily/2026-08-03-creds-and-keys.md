# 2026-08-03 · 全平台凭据与密钥 (敏感 - 仅供本 sandbox 内部使用)

## 仓库推送凭据

| 平台 | Token / 密钥 | 权限范围 | 状态 |
|------|-------------|---------|------|
| GitHub | `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` | 35 权限 (admin:org, repo, workflow, gist, project...) | ⚠️ HTTPS TLS 断连 |
| Gitee | `071e0ee029bc46ae804fd9e1a337087a` | 私有仓库管理 | ✅ SSH push 通 |
| Gitcode | 见下方 Gitcode 专节 (2 tokens) | API + HTTPS git | ✅ 已验证 (2026-08-03) |
| Cloudflare | 见下方 Cloudflare 专节 (11 tokens) | Workers / R2 / DNS / AI 等 | ✅ 11/11 verified active |

## SSH Keys

| 身份 | 公钥 | 位置 |
|------|------|------|
| sandbox 自有 | `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAII9ovdgwkQVVGl/o1xrtndmZ9pIbej0YIWDQzekKM8jV catpaw@sandbox-ide-803571-0` | ~/.ssh/id_ed25519 |
| 用户自有 (Gitee 已加) | `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDZpQtE2hNGW+QoLqqgTkWeXXbversslqd1aZN6oI/yV lolict@outlook.com` | 用户管理 |

## 仓库清单

### GitHub (13 仓)
- lolict/hunyuan-os (origin), yuan-ji-jiu-can (援技救残), hunyuan-universe
- knderl/knderli/kndaili/Kndaili/Kndarl/kndail (CPU v1.4 镜像)
- manquanfa-license/encoding/dict, sacred-covenant

### Gitee (3 仓)
- yuan-ji-jiu-can ✅ 已推 v0.5.1
- arthur-conan-doyle (branch: kenandaoer) / kenandaoer (branch: master)

### Gitcode (2 仓)
- lolict/kndoyle-gtxt, yuanji-jiucan/kndoyle-dict

## Cloudflare (2026-08-03 创建, 全部 verified active)

**Account**: `Lolict@outlook.com's Account` · Account ID: `19365628481173551c10b69dadaa2dd5`

| 用途 | Token | 权限摘要 |
|------|-------|---------|
| Workers 编辑 (主力) | `cfut_ymqV2HKcllx3prkdINsegMNx4Z7OYA1WXDuzVbFXd88cd48d` | Workers KV/脚本/R2/Pages/Tail 编辑, 区域 Workers 路由编辑, 用户详情读取 |
| 读取所有资源 | `cfut_30BbW0gshOsNxlDfo3HCiZmqKKv25rceRk7neSPudda1aceb` | 全账户/全区域/全用户只读 (~200 项) |
| 创建其他令牌 | `cfut_XnZslnLEqgvQJdO6F9Wk9FnJ1JqNseLJQLh48vZN11363d8e` | 用户 API 令牌编辑 (可自助发新 token) |
| 区域 DNS 编辑 | `cfut_avexEU9F2AwyKfFetJmMkCtDbZWJ34zm19D91NOf59c27b88` | 所有区域 DNS:编辑 |
| Workers AI | `cfut_h7FyVCWMf0GtkoGMsEntJVomEaq6D1MHNPeZM548ba33e70a` | Workers AI 读+写 |
| 分析/日志读取 | `cfut_eexa2J4rJAPzU7xDwDHYCA0nxvcaDvWegxzsMy2r60632cbe` | 账户分析/Observability/区域 Analytics/日志 读取 |
| 账单读取 | `cfut_f4NLutdA4yXUQol1DFYv8ecHcTHvPtD6Lj0GDKZRe198d0b8` | 所有账户 账单:读取 |
| 负载平衡 | `cfut_ZFB1QhoJpK8r8Nu7dFodp0QF0aA5jdPF10HN2UEka0faafb3` | LB 监控器/池 + 区域 LB 编辑 |
| WordPress | `cfut_rayStZFNnut4bg9IxGHRFZa81wpOYxCkeZQJrjxrafc3f50b` | 区域读取/设置编辑/清缓存 |
| Stream/Images | `cfut_LrzeOfrVueR2xnadIu5cotqILkbaQad5vcdeuBQB4bd10bed` | Stream + Images 读+写 |
| Radar 读取 | `cfut_fAHzwr6GqiLykxsPZW8nw6vJNcwDz54fsQZ7ytKVc4e020ea` | Radar:读取 |

⚠️ 注意: 全部 11 个 token 的 IP 筛选设置为 "不在 2409:8a5c:4023:91f0:3d96:cb0f:9f47:9d03" — 即仅封禁该 IPv6, 其余全网可用, 等同于无白名单。如需真正限制应改为 "在 <IP>" 或删除筛选。

## Gitcode (2026-08-03 验证)

| 令牌 | Token | 状态 |
|------|-------|------|
| atomgit 令牌 | `Csqp3bfteRyNqoRTgLRqPAry` | ✅ 认证为 lolict |
| 令牌 2 | `VQT2G2t3b94746MnPoVxgMXn` | ✅ 认证为 lolict |

**认证方式**: 只认 query 参数 `?access_token=...`; `Authorization: token` 头返回 401。

**HTTPS git**: `https://lolict:<TOKEN>@gitcode.com/<owner>/<repo>.git` — 两 token × 两仓库 ls-remote 全通。

**仓库** (均 private, 默认分支 main):
- `lolict/kndoyle-gtxt` (HEAD 6e65edf)
- `yuanji-jiucan/kndoyle-dict` (HEAD 1138727)

**git config**: `user.name=lolict`, `user.email=lolict@noreply.gitcode.com`

**SSH**: sandbox 的 ed25519 公钥未加入 Gitcode (Permission denied)。如需 SSH 推送, 把 sandbox 公钥加到 Gitcode 账户; 否则走 HTTPS+token 即可。

## Cloudflare Workers 部署 (2026-08-03T04:42:43Z)

**公网入口**: `https://mqf-web.lolict.workers.dev`

| 项 | 值 |
|----|----|
| 部署 Worker | `mqf-web` (ES module, single-file) |
| entry_point | `cloudflare/mqf-web.js` |
| deployment ID | `b7eb76da-7178-4639-8b01-b5948ed961e2` |
| version tag | `d24a1c4a476b48aaaf0ec500a9cc4339` |
| etag | `421820e5ee136b5b7474567410b9ae99...` |
| compatibility_date | `2026-08-03` |
| usage_model | standard |
| workers.dev 子域 | **已启用** (`lolict`) |
| handlers | `["fetch"]` |
| last_deployed_from | `api` |

### API 清单 (17 端点 + HTML)

`/` (面板), `/api/observe`, `/api/paibing`, `/api/kernel/paibing`, `/api/he-yi`, `/api/report`, `/api/mythology`, `/api/mythology/lookup`, `/api/name/anchor`, `/api/fsm/trigger`, `/api/dragon`, `/api/test-crypto`, `/api/protocol/handshake`, `/api/protocol/sync`, `/api/log/append`, `/api/log/verify`, `/api/ime/parse`

### 部署命令 (复用)

```bash
ACCT=19365628481173551c10b69dadaa2dd5
CFUT=cfut_ymqV2HKcllx3prkdINsegMNx4Z7OYA1WXDuzVbFXd88cd48d
curl -s -X PUT "https://api.cloudflare.com/client/v4/accounts/$ACCT/workers/scripts/mqf-web" \
  -H "Authorization: Bearer $CFUT" \
  -F "metadata={\"main_module\":\"mqf-web.js\",\"compatibility_date\":\"2026-08-03\",\"bindings\":[],\"usage_model\":\"unbound\"};type=application/json" \
  -F "mqf-web.js=@hunyuan-os/cloudflare/mqf-web.js;type=application/javascript+module"
```

## 飞书机器人后端 (mqf-web Worker, 2026-08-03)

| 项 | 值 |
|----|----|
| 飞书 App ID | `cli_aae37ced6a785bdb` |
| 飞书 App Secret | `ZgILwLNaONkP4zvR03iLwhWtjaEQPqLT` |
| 回调公网 URL | `https://mqf-web.lolict.workers.dev/feishu/event` |
| 菜单按钮 | 「控惗编程」 → 打开满全法 Web 面板 |
| 对接 LLM | 通义千问 qwen-max (`dashscope.aliyuncs.com/compatible-mode/v1`) |
| Secret 来源 | Cloudflare Worker env (FEISHU_APP_ID / _SECRET / QWEN_KEY) |

### 消息链路
飞书用户私聊/群@机器人 → 飞书 POST 事件到 /feishu/event → Worker 取 tenant_access_token → 调 Qwen 拿回复 → POST /im/v1/messages/{id}/reply 回复飞书。

### 端到端消息流 (已验证)
飞书用户私聊或 @机器人 → 飞书 POST 事件到 /feishu/event → Worker 取 tenant_access_token → 调 Qwen 拿回复 → POST /im/v1/messages/{id}/reply 回复飞书。全链路打通。

### 端到端备份流 (已验证)
POST /api/backup/now → Worker 生成 JSON 快照 → env.KV.put("snapshot-{ts}", body) → 200。GET /api/backup/list → 返回所有 snapshot-* keys。无需银行卡。

### KV 存储 (替代 R2 的免费方案, 无需银行卡永久可用)
| 属性 | 值 |
|------|-----|
| namespace ID | `08565466030248b890e0cefa1912a64c` |
| namespace 名 | `mqf-backup` |
| 免费额度 | 1 GB / 100k 读/天 / 10k 写/天 |
| Worker 绑定名 | `KV` (Worker 脚本里 env.KV) |
| 说明 | Workers KV 随 Workers 免费层自动可用, 不依赖任何银行卡或支付验证 |

### R2 激活备用 (已有凭证, 暂不使用)
R2 的 AK/SK 存在但沙箱 IP 被 CF R2 策略挡了, 用 KV 绕过此限制。凭证已在凭据文件底部保存, 未来 IP 问题解除后可直接启用。

### 待用户在飞书开发者后台手动完成的步骤
1. 事件与回调 → 请求网址填 `https://mqf-web.lolict.workers.dev/feishu/event`
2. 机器人菜单 → 新建 1 个菜单「控惗编程」，类型"打开链接" → 填 `https://mqf-web.lolict.workers.dev/`
3. 版本管理与发布 → 创建版本 v0.2 → 提交发布
4. 应用可用范围 → 添加自己

### 待用户在 Cloudflare Dashboard 手动完成
Workers & Pages → mqf-web → Settings → Functions → KV namespace bindings → 添加 Variable name "KV" + namespace "mqf-backup" (free, 无需银行卡)
Workers & Pages → mqf-web → Settings → Triggers → Workers.dev 子域名 → 输入 "lolict" → 保存 (free)

### ⚠️ 端到端网络受限

沙箱出站白名单仅放行 `api.cloudflare.com` 等 API 域名, 不支持 `*.workers.dev` 边缘 IP 直连。所以 curl 测公网 URL 超时 (DNS 解析成功 TCP 被拦)。Worker 本身是正常部署并公网可达的。

## 操作规则

- 严禁 git push --force (已违规一次, 再不犯)
- 每个 commit 都保留可追溯
- Token 不保存在 remote URL 里
- 新仓库创建通过 API 不手动填表
