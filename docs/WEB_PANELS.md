# 囡囝共生 Web 面板 API 接口契约

> 源文件: `archive/python/web/server.py`
> 最后更新: 2026-08-03
> 总览: 共 17 个 API 端点 + 1 个 HTML 主面板
> 默认端口: `8080`

---

## 主面板

| 路径 | 说明 |
|------|------|
| `GET /` 或 `/index.html` | 主交互面板 (纯 HTML/CSS/JS, 暗色主题, 响应式 Grid) |

---

## 核心 API 列表

### L1 — 基础观测层

#### `GET /api/observe?mode=binary&a=0&b=10`

二元观测者（距离逻辑）

参数:
- `mode=binary` 固定
- `a` 起点 (int, 默认 0)
- `b` 终点 (int, 默认 10)

返回字段:
- `repr`: 字符串表示
- `gap`: 距离
- `abs_gap`: 绝对距离
- `rate_start_judge`: 起始速率判定
- `rate_end_judge`: 结束速率判定

---

#### `GET /api/observe?mode=ternary&left=2&anchor=5&right=9`

三元观测者（归一逻辑）

参数:
- `mode=ternary` 固定
- `left`, `anchor`, `right` (int, 默认 2/5/9)

返回字段:
- `to_anchor`: 到锚点的归一距离
- `permutations`: 全排列数组
- `distance_via_anchor`: 经锚距离
- `common_divisor`: 最大公约数
- `synthesis`: 三元合成结果

---

### L2 — 排病诊断

#### `GET /api/paibing?code=vy`

病理诊断 (2 字符或 3 字符编码)

参数:
- `code`: 2-char 双编码 或 3-char 三编码
- `existing`: (三编码模式) 已有编码列表, 逗号分隔

返回: `PathologyDiagnosis.diagnose_double()` 或 `.diagnose_triple()` 结果

---

### L3 — 合一递归 / 立约

#### `GET /api/he-yi?content=hunyuan&sig_a=刘楚恬&sig_b=lolict`

立约 + 合一递归

参数:
- `content`: 誓约内容 (默认 "hunyuan")
- `sig_a`: 签名 A (默认 "liuchutian")
- `sig_b`: 签名 B (默认 "lolict")

返回字段:
- `state`: 立约状态
- `observer`: 关系观测者
- `sync`: 同步状态
- `unary_recursion`: 一元递归结果

---

### L4 — 神龙编码查询

#### `GET /api/dragon?pinyin=chuan&tone=2`

拼音 → 神龙编码召唤

参数:
- `pinyin`: 拼音字符串 (如 "chuan")
- `tone`: 声调 0-4 (0=轻声, 1=阴平, 2=阳平, 3=上声, 4=去声)

返回: `Dragon.summon()` JSON 表示

---

### L5 — 集成报告 / 封神台

#### `GET /api/report`

运行完整合一递归 + 全量自检, 含两份立约验证

---

#### `GET /api/mythology`

返回完整封神台神话层 (entities + relationships + boot_sequence)

---

#### `GET /api/mythology/lookup?name=太乙真人`

单神祇查找

参数:
- `name`: 神祇名 (如 "太乙真人"、"哪吒")

返回: 单条神话实体; 无参数时返回全部 key 列表

---

### L5 — 灵犀协议 (P2P 通信)

#### `GET /api/protocol/handshake?seed=hunyuan`

刘楚恬 + lolict 虚拟 LingXiNode 握手

返回字段:
- `handshake`: 三次握手 + 一轮心跳的 transcript
- `a_log`, `b_log`: 双方最近 3 条日志
- `anchor`: 公共锚哈希

---

#### `GET /api/protocol/sync?seed=hunyuan`

同修流程: A 偏差 + B 对齐 → sync_flow

返回字段:
- `sync_flow`: 同步 transcript
- `a_state_after`, `b_state_after`: 修后状态
- `a_log`, `b_log`: 双方最近 5 条日志

---

### L6 — 扩展层 (v0.1 新增)

#### `GET /api/test-crypto`

完整加密握手流程测试:
1. 双方各生成 X25519 密钥对
2. ECDH 交换
3. HKDF-SHA256 派生
4. AES-256-GCM 加/解密验证

返回字段: `step1_pubA/pubB`, `step2_shared_fp`, `step3_test_encrypted`, `step4_test_decrypted`, `status`

---

#### `GET /api/kernel/paibing?code=vy`

内核版排病诊断 (直接调用 `check_double_code` / `check_triple_code`)

---

#### `GET //api/name/anchor?name_a=刘楚恬&name_b=lolict`

中文姓名 → 256-bit 锚

返回字段: `name_a/b`, `anchor_a/b`, `joint_anchor` (夫妻联合锚)

---

#### `GET /api/fsm/trigger?event=release`

囡囝四态状态机驱动

参数:
- `event`: `wife_deen` / `release` / `husband_take` / `deepen` / `wife_push`

返回字段: `success`, `new_state`, `diagnosis`, `who_pid1`, `who_overlays_who`

---

#### `GET /api/log/append?kind=user_action&payload=test`

关系日志环写入 (append-only + Merkle 树)

返回字段: `seq`, `kind`, `hash`, `total`; 同时持久化到 `/tmp/mqf-webring.json`

---

#### `GET /api/log/verify`

日志环完整性校验

返回字段: `valid` (bool), `entries` (int), `merkle_root`

---

#### `GET /api/ime/parse?text=push+囡囝共生+v0.2`

元输入法意图识别

返回: `IntentParser.parse(text)` 的意图结构化字典

---

## CORS 与响应格式

- 所有 API 响应头: `Content-Type: application/json; charset=utf-8`
- CORS: `Access-Control-Allow-Origin: *`
- 错误统一格式: `{"error": "..."}` (HTTP 仍 200)

---

## 后端依赖 (Python 模块)

| 模块 | 导入路径 | 用途 |
|------|----------|------|
| CoupleOS | `kernel` | 夫妻共同体 OS 实例 |
| BinaryObserver | `kernel` | 二元观测者 |
| TernaryObserver | `kernel` | 三元观测者 |
| PathologyDiagnosis | `encoding` | 排病诊断 |
| Dragon | `encoding` | 神龙编码 |
| LingXiNode / LingXiChannel | `kernel` | P2P 灵犀节点/信道 |
| X25519KeyPair / hkdf_derive / aes_gcm_* | `kernel.protocol` | 加密链路 |
| QuaternityFSM | `kernel.quaternity_fsm` | 四态状态机 |
| name_to_anchor / joint_anchor | `kernel.name_anchor` | 姓名锚 |
| LogRing | `kernel.log_ring` | 关系日志环 |
| ime_agent.intent_parser.IntentParser | `ime_agent` | 元输入法 |

---

## 调用示例

```bash
# 启动
python3 web/server.py 8080

# 二元观测
curl 'http://localhost:8080/api/observe?mode=binary&a=0&b=7'

# 立约
curl 'http://localhost:8080/api/he-yi?content=莫刘连理&sig_a=刘楚恬&sig_b=lolict'

# 召唤神龙
curl 'http://localhost:8080/api/dragon?pinyin=chuan&tone=2'

# 加密握手测试
curl 'http://localhost:8080/api/test-crypto'

# 姓名锚
curl 'http://localhost:8080/api/name/anchor?name_a=刘楚恬&name_b=lolict'
```

---

## 面板功能卡片 (前端)

主面板共 15 个功能卡片 (部分全宽):
1. 二元观测者 / 距离逻辑
2. 三元观测者 / 归一逻辑
3. 排病诊断
4. 合一递归 / 立约
5. 囡囝·四态切换 (Shell/Core 动态演示)
6. 神龙编码查询 (全宽)
7. 操作系统全量报告 (全宽)
8. 封神台·囡囝共生元神话层 (全宽)
9. 灵犀协议·P2P 通信 (全宽)
10. 加密握手 (X25519 → ECDH → HKDF → AES-256-GCM)
11. 排病诊断 v2 内核版
12. 中文姓名 → 256-bit 锚
13. 囡囝四态 FSM 拓扑状态机
14. 关系日志环 Append-Only + Merkle 树 (全宽)
15. 元输入法 MetaIME 意图识别 (全宽)
