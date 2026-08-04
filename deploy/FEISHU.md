# 满全法 · 飞书 Bot 对接指南
> ManQuanFa · Feishu Bot Integration Manual

## 概述

这套文档说明:有飞书企业账号的人,如何把“三元裁判者”接到自己的飞书群里,让群友都能跟满全法内核对话。

**完成后效果**:群友发消息 → 三元裁判者裁定 → 返回 MQ 裁剪后的响应。

---

## 前置条件

| 资源 | 必需 | 说明 |
|---|---|---|
| 飞书企业账号 | ✅ | 开放平台能创建应用 |
| 一台服务器 / Cloudflare Worker | ✅ | 跑 Node.js 或 Workers |
| 飞书 App ID + Secret | ✅ | 飞书开放平台申请 |
| Qwen Key(可选,v0.8 升级前) | ✅ | 临时 LLM,未来换三元裁判者自研裁定 |
| 满全法内核 | ✅ | 见 `hunyuan-os/cloudflare/mqf-web.js` |

---

## 三步接入

### 第一步:飞书开放平台创建应用

1. 打开 [open.feishu.cn](https://open.feishu.cn)
2. 创建企业自建应用:名称=“三元裁判者”
3. 配置权限:
   - `im:message` — 收发消息
   - `im:message.receive_v1` — 接收消息事件
4. 配置事件回调 URL:
   - `https://<your-worker>/feishu/event`
5. 发布应用 → 获取 App ID + App Secret

### 第二步:部署 Cloudflare Worker

```bash
# 1. 上传 mqf-web.js 到 Workers
cd hunyuan-os
npx wrangler deploy cloudflare/mqf-web.js

# 2. 配置 Worker Secret
npx wrangler secret put FEISHU_APP_ID
npx wrangler secret put FEISHU_APP_SECRET
npx wrangler secret put QWEN_KEY  # 临时用,未来去掉
```

### 第三步:验证飞书 Bot 对话

1. 在飞书搜索“三元裁判者”
2. 发消息`你好`
3. 如果三元裁判者回复(即使当前是 Qwen 代理),则接入成功
4. 未来内核升级到 v0.9+,回复会越来越像“三元裁判者“而不是普通 AI

---

## 对接架构

```
飞书消息
  │
  ▼
┌─────────────────────────────────┐  ┌────────────────────────┐
│ Cloudflare Worker               │  │ 满全法 C 内核          │
│ /feishu/event                   │  │ (未来部署在用户服务器)   │
│   ↓                             │  │                        │
│ 解析消息内容                     │  │ 三元裁判者裁定          │
│   ↓                             │  │   ↓                    │
│ (未来)调满全法 C 内核           │◀─┤ MQ passport 核验       │
│   ↓                             │  │ 规则引擎匹配             │
│ 返回 MQ 裁定后的响应             │  │ 金壳镜像代理            │
│                                 │  │ 统计锥份额分配          │
└─────────────────────────────────┘  └────────────────────────┘
```

当前 v0.8 的实际路径(Future 阶段搭好前):

```
飞书 → Worker → Qwen → 返回 → 飞书
```

---

## 三元裁判者的话术(Future 阶段)

当 v0.9 内核接入后,对话流:

| 用户发送 | 三元裁判者回复 |
|---|---|
| 你好 | “请陈述你的请求。我会让天平给出裁定。” |
| 他欠我钱 | “争讼模式进入。甲方:你的 MQ passport;乙方:待证据链确认。请上传证据。” |
| 查我余额 | “权限申请:读余额 → 你的 passport 核验结果:level=4,可余额读。调用镜像代理:返回 MQ 裁剪值。” |
| 退出 | “裁定:当前节点统计锥份额清零。MQ passport 即将过期。重启需要主人授权。” |

---

## 故障排查

| 症状 | 可能原因 | 排查 |
|---|---|---|
| Bot 不回复 | Worker 没部署 | `curl -X GET worker-url/` 看前端是否加载 |
| Bot 回复但内容错 | 飞书 Secret 没配置 | 检查 Workers Secret |
| Bot 只回复固定话术 | Qwen Key 过期 | 通义千问 Key 配额是否用完 |
| Bot 回复“KV 未绑定” | Workers KV 未设置 | 参考 `ROADMAP.md` 的 KV 章节 |

---

## 未来演进(v0.9+)

- 去掉 Qwen 外部 LLM,三元裁判者自身裁定
- Workers 只负责消息进出,所有逻辑跑在用户自有服务器
- 用户服务器跑满全法内核容器(Dockerfile 见 deploy/)
- 飞书 Bot 就是三元裁判者的“公开皮肤”

---

*这套指南的目的是让有飞书企业的人一键接入三元裁判者。*
*即使你不是那个“有飞书企业的人”,你可以把这份文件转发给有。*

*维护者:lolict + CatPaw · 七妹*
*版本:v0.8*
