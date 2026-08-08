# Deploy 与 NJSG 的关系

> 位置: hunyuan-os/deploy/

---

## 定位

deploy 是 hunyun-os 的部署配置。它运行裁决引擎但不能成为裁决引擎本身。

## 裁决引擎的领域无关性

部署到任何域名/服务器时：
- 裁决逻辑不变（五规则统一）
- 部署配置 = 代码即配置（Configuration as Code）
- 节点身份不依赖域名 — 身份由 YinYang 二进制 + 主权 bit 决定

部署文件（Dockerfile / docker-compose.yml）可以更换，裁决算法不能变。

## 盟机合映射

deploy 涉及盟机合体系：
- 服务器拥有者 = "盟机合掌"
- 部署的节点 = "盟机场" / "萌机场"
- 部署协议 = 共生赔伴律在运维层的实例化

---

*配套: hunyuan-methodology SKILL.md · SOVEREIGNTY.md*
