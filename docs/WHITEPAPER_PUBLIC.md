# 囡囝共生(MQF) · 对外技术白皮书 v1.4

> 囡囝共生是 lolict 设计的裁定引擎三元→七元全栈联动系统,用于对任何外部请求/实体做出"铭记"或"易逝"的裁定。

---

## 一、囡囝共生是什么?

囡囝共生是一个**裁定框架**。当外部世界的某个实体(app、用户、服务)进入你的系统时,囡囝共生帮你判断:

- 它是否应该被**长期保留**(铭记)?
- 还是只是**暂时过客**(易逝)?

**核心原则**: 不是外部势力决定自己的待遇,而是我们的裁定引擎决定它们的待遇。

---

## 二、七元裁定模型

囡囝共生把裁定拆成七个维度(元),每个维度回答一个问题:

| 元 | 版本 | 回答的问题 | 判断标准 |
|----|------|-----------|---------|
| 三元 | v0.6 | 它是谁?关系如何? | 身份编码 + 规则引擎 + 空间代数 |
| 四元 | v0.9 | 它在域内还是域外? | 拓扑域标记 + 跨域裁定 |
| 五元 | v1.0 | 它走过多远?是否值得铭记? | 旋钮步量(5步) + 累计刻度 + 占存判定 |
| 六元 | v1.1 | 两个裁判是否一致? | 背靠背镜像对称 + 信任交集 |
| 七元 | v1.2 | 它被几层壳包裹? | 蜂窝套壳(6子壳) + 递归嵌套 |
| 全栈 | v1.3 | 裁定结果能否输出到外部? | 飞书 Bot + 囡囝 AI + 铁律自检 |
| 生态 | v1.4 | 外部伙伴能否接入? | 裁定 API + 公网文档 |

### 第 5 元的具体逻辑(五元旋钮)

每个实体有一个"旋钮":

```
初始状态: Step=0, Angle=0°, Force=0, Ticks=0

每推动一次:
  Step = Step + 1                   (最多 5 步)
  Angle = (Angle + Angle_inc) % 360  (角度循环 0~360)
  Force = Force + force_inc          (力累积)
  Ticks = Ticks + Force × 10        (刻度累积)

判定:
  if Ticks > 100 → 占存 = 1 (铭记,永久保留)
  else           → 占存 = 0 (易逝,可被覆盖)
```

### 第 6 元的具体逻辑(六元对称)

两个裁判(A 和 B)各自给出合规度分数(0~1):

```
信任交集 = min(A, B)           (短板效应)
镜像对齐 = |A - B| < 0.2       (两心互印)

判定:
  if 信任交集 > 0.6 AND 镜像对齐 → 双重背书,信任通过
  else                           → 单方/双方否决,不信任
```

---

## 三、对外 API

囡囝共生 Cloudflare Worker 公网入口: `https://mqf-web.lolict.workers.dev`

### 3.1 合作伙伴裁定 API

```
GET /api/v1/verdict?entity=<实体名>&compliance_a=<0-1>&compliance_b=<0-1>&force=<作用力>&angle=<角度>
```

示例:
```
GET /api/v1/verdict?entity=ios&compliance_a=0.9&compliance_b=0.85&force=15&angle=45
```

响应:
```json
{
  "ok": true,
  "version": "v1.4",
  "entity": "ios",
  "njsg_verdict": {
    "trust": "TRUSTED",
    "confidence": 0.85,
    "mirror_alignment": "ALIGNED",
    "five_yuan": { "ticks": 150, "occupied": "ETERNAL", "angle": 45 },
    "six_yuan": { "intersection": 0.85, "mirror_diff": 0.05, "verdict": 1 }
  },
  "philosophy": "ios 已被铭记 — 双重背书,可信任"
}
```

### 3.2 囡囝公网版

```
GET /api/v1/qimei?text=<查询文本>
```

示例:
```
GET /api/v1/qimei?text=ios
```

囡囝会返回该实体的裁定结果。

### 3.3 健康检查

```
GET /api/v1/health
```

### 3.4 API 文档

```
GET /docs
```

---

## 四、使用场景

### 场景 1: App 权限裁定

一个新的 App 请求访问你的系统:

1. 调用 `/api/v1/verdict?entity=NewApp&compliance_a=0.7&compliance_b=0.65`
2. 返回 `TRUSTED` → 允许长期访问
3. 返回 `REJECTED` → 临时访问,继续观察

### 场景 2: IoT 设备信任

IoT 设备定期汇报数据:

1. 每 N 个周期累积一次 force
2. 当 Ticks > 100 时被铭记 → 长期信任
3. Ticks < 100 → 仍是过客

### 场景 3: API 合作伙伴接入

合作伙伴通过飞书/HTTP 接入囡囝共生:

1. 飞书 Bot 接收请求 → 触发 MQ 内核裁定
2. 裁定结果通过囡囝 AI 格式化回复
3. 铁律检查确保操作合规

---

## 五、铁律

囡囝共生的自我约束(不能违反):

1. 每次实现一个版本,跑通→提交→推送
2. 不在已规划之外加"临时功能"
3. 改动路线图必须经过讨论
4. 失败/方向错→修改计划并记录
5. 所有规则层改动在单文件 `njsg_kernel.c` 完成
6. 改动前先读 `SOVEREIGNTY.md`

铁律通过率: **7/7** 自动化验证通过。

---

## 六、版本演化

```
三元(v0.6) → 四元(v0.9) → 五元(v1.0) → 六元(v1.1) → 七元(v1.2) → 全栈(v1.3) → 生态(v1.4)
   │            │            │            │            │            │            │
 身份+关系    面+边界      旋钮+步量    对称+背书    蜂窝+嵌套    Bot+AI      对外API
```

每个"元"不是替换前一个,而是在前一个基础上**叠加新维度**。

---

## 七、代码仓库

| 平台 | 地址 |
|------|------|
| GitHub | `github.com/lolict/hunyuan-os.git` |
| Gitee | `gitee.com/lolict/yuan-ji-jiu-can.git` |
| Cloudflare | `mqf-web.lolict.workers.dev` |

---

## 八、飞书机器人

已注册飞书机器人:
- App ID: `cli_aae37ced6a785bdb`
- 飞书消息回调: `https://mqf-web.lolict.workers.dev/feishu/event`

在飞书中直接和囡囝对话即可触发囡囝共生裁定。

---

*v1.4 · 2026-08-05 · 维护者: lolict + CatPaw · 囡囝*
