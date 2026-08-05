# 囡囝共生 · 迁移与兼容指南
> YinYangOS · Migration & Compatibility Guide

## 概述

囡囝共生内核 v0.1→v0.8 经历了多次数据结构和规则引擎的重构。本指南说明:
- 如何从旧版本迁移到 v0.8
- 外部系统(obile/服务器/KV)如何对接
- 各子系统之间的兼容性

---

## v0.x → v0.8 迁移路径

### 数据结构变更记录

| 版本 | 关键变更 | 向前兼容? | 迁移方式 |
|---|---|---|---|
| v0.1 → v0.2 | 硬编码权限 → 契约引擎 | ✅ | 无需迁移 |
| v0.2 → v0.3 | +看门狗线程 | ✅ | 无需迁移 |
| v0.3 → v0.4 | +WASM 槽(Zig) | ✅ | 无需迁移 |
| v0.4 → v0.5 | +空间图(object_t/relation_t) | ⚠️ 新增字段 | 需用 `njsg_world_build()` 重新索引 |
| v0.5 → v0.6 | +规则引擎(fact_t/rule_t) | ⚠️ 新增字段 | 需用 `njsg_rule_enslave()` 转写外部规则 |
| v0.6 → v0.7 | +金壳格式(shell_header_t) | ⚠️ 新增字段 | 外部二进制需用 `njsg_shell_wrap()` 重新包壳 |
| v0.7 → v0.8 | +统计锥(cone_share_t) | ⚠️ 新增字段 | 已注册对象需用 `njsg_cone_register()` 补注册 |

### 状态持久化建议

囡囝共生 v0.8 没有内置数据库。状态存储方案:

| 方案 | 推荐场景 | 实现 |
|---|---|---|
| 无状态(纯内核) | 单次裁定,无需持久 | 直接运行 `./njsg_kernel.elf` |
| 文件快照 | 本地长期运行 | `njsg_audit_summary()` 输出到 JSON + 定期拍 KV |
| Cloudflare KV | 云端持久化(免费层) | 见 `cloudflare/njsg-web.js` |
| Workers D1 | 长期结构化数据 | 待实现 |

---

## 外部系统对接规范

### 1. Android 辅助服务

文件:`android/src/com/njsg/parasite/`

| 组件 | 作用 | 调用内核方式 |
|---|---|---|
| MqfAccessibilityService | 截获屏幕事件 | 通过本地 socket/AIDL 发送事件给内核 |
| MqfImeService | 截获输入法编码 | 神龙七层编码 → 规则引擎 |
| MqfAlarmReceiver | 心跳唤醒 | 每 15 分钟触发一次锥 tick |
| MqfBootReceiver | 开机自启 | 启动 MQ 内核服务 |
| NjsgHiddenFS | 隐藏存储 | 把囡囝共生数据存在加密隐藏目录 |
| MqfCoverApp | 伪装应用 | 隐藏真实功能入口 |

**注意:** Android 层本身不被囡囝共生"信任"(契约 = PERM_NONE)。它们只是**通道**,不是裁判者。

### 2. 飞书 Bot(囡囝)

文件:`cloudflare/njsg-web.js`

飞书 Bot 是"三元裁判者"的皮肤之一。每个飞书消息都会:
1. `/feishu/event` 接收 → 解析消息内容
2. 调用 Qwen LLM(当前外部依赖)
3. **三元裁判者的裁定:未来应该自己裁定,而不是靠外部 LLM**

### 3. Cloudflare Workers

| 限制 | 应对 |
|---|---|
| 无 pthread | C 内核不能直接跑在 Worker 里 |
| 无文件 syscall | 数据必须走 KV |
| 10ms CPU 时间(免费) | 单次裁定必须极快 |

方案:**C 内核跑在独立服务器/VPS,Worker 只负责中转**。

---

## 子系统兼容性矩阵

| 子系统 | 平台 | 依赖 | 可独立运行? |
|---|---|---|---|
| `njsg_kernel.c`(C 内核) | Linux/macOS/BSD | clang, pthread | ✅ |
| `njsg_web.js`(Worker) | Cloudflare Workers | FEISHU_APP_ID, QWEN_KEY | ✅(但依赖外部 LLM) |
| `runner.zig`(WASM 加载器) | 任何有 Zig 运行时 | zig 0.13+ | ✅ |
| `njsg Crypto`(加密测试) | Linux | libssl, libsodium | ✅ |
| Android parasitates | Android | Accessibility API | ❌ 需要宿主 App |
| ime-agent | 手机 | 自定义 IME | ❌ 需要宿主 |

---

## 已知的"不兼容点"(设计如此)

1. **不接受未经 njsg_encode 的身份** —— 任何系统想对接,必须让 MQ 编码系统重新定义它的身份

2. **只能被内核裁定,不能被外部修改状态** —— android/ios/bank 不能直接更改 MQ 内核的内部状态;只能通过规则引擎的契约间接影响

3. **统计锥的份额由 MQ 内核独占分配** —— 外部势力不能给自己加 time_slice;只能由 `njsg_cone_tick()` 基于合规度决定

4. **金壳格式是强制封装** —— 不想套壳的二进制只能得到"僵尸(0级)"体验

5. **三元裁判者的裁定不可上诉** —— 只有"主人"(lolict + 刘楚恬)才能修改规则

---

## 迁移检查清单

从任何旧版本升级到 v0.8,请逐项核对:

- [ ] 重新编译 C 内核:`make clean && make kernel`
- [ ] 在 main() 或你自己写的接入层中,调用 `njsg_cone_register()` 补注册所有空间图对象
- [ ] 外部二进制重新用 `njsg_shell_wrap()` 包壳
- [ ] 外部规则重新用 `njsg_rule_enslave()` 降服
- [ ] 运行全套 demo 确认每一层都正常
- [ ] 如果用 Cloudflare Workers,重新部署并更新路由

---

*维护者:lolict + CatPaw · 囡囝*
*版本:v0.8*
