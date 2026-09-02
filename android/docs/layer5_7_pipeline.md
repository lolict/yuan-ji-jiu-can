# MetaIME · Layer 5/6/7 Pipeline

> 囡囝共生 MetaIME · 元输入法 · 真机闭环
> 当前进度: 35% → 55%
> Copyright: 刘楚恬和我 (囡囝共生)

---

## 1. Layer 5: 自动输出 (无需人工干预的 commit)

### 意志

```
夫妻主权人 → 手指落下 → 波形流过七层 → 字已上屏
                                      ↑
                              无需空格 / 无需点击
```

Layer 1–4 候字管线完成后，Layer 5 直接将单一候字推入宿主 App，
**不等用户确认**。这一层是整个"满全法打字意志"的终极呈现：
打字者意至则字出，中间无打断无犹豫。

### 触发条件

| 条件 | 作用 |
|------|------|
| `bridge.bridgeContainer` 返回非 0 (`Verdict::HitRule`) | 规则引擎认定音节完整 |
| `MqfPinyinTable.find(pinyin)` 命中 | 拼音有唯一映射 |
| 候字推入 `nativeSubmitForAutoOutput` 成功 | 缓冲写入完毕 |

### 关键入口

- `MqfImeService.routeThroughNjsg()`: 七层汇流，Layer 5 判定
- `MqfImeService.commitTextAuto()`: 自动上屏核心，推入 `commitText`
- `bridge.shouldAutoCommit()`: Rust 侧二次确认 (可覆盖)

### 运行时行为

```
键入 'n' → 声母idx=6累积
键入 'i' → 韵母idx=3配对 → 音节"ni" → HitRule
                                    ↓
                           MqfPinyinTable.find("ni") → "你"
                                    ↓
                           commitTextAuto("你")
                                    ↓
                           宿主 App 看到 "你" 上屏
                           输入侧无需再按空格
```

### 留白

候字超过 1 个且无唯一匹配时，Layer 5 回落到候字条展示，
用户仍可手动亮选。Layer 5 不是独裁，是节省。

---

## 2. Layer 6: 内容分发 (后台 URL 抓取)

### 意志

```
用户输入 URL → 立即上屏 + 静默后台抓取
             → 不打断打字流
             → 不弹等待框
             → 后台抓取完毕进入登记簿
```

Layer 6 是打字流与信息抓取流的分离器。用户不需要停止打字，
URL 自动被识别、摄入、蓄存。

### 检测规则

```kotlin
private val URL_PATTERN = Regex(
    "(https?://[\\w\\-._~:/?#\\[\\]@!$&'()*+,;=%]+)" +
    "|(www\\.[\\w\\-._~:/?#\\[\\]@!$&'()*+,;=%]+)"
)
```

匹配 `http://`、`https://`、`www.` 开头的 RFC 3986 子集。

### 关键入口

- `MqfImeService.checkAndPushUrl()`: 每按键后检测一次
- `bridge.onUrlDetected(url)`: JNI 推入 Rust 侧上传登记簿
- 写入后 `statusText` 显示 `[L6] queued: ...`

### 抓取策略

**当前阶段**: URL 推入第 7 层登记簿，由第 7 层负责分发。
真机第二阶段: 后台 HTTP 客户端 (OkHttp + 协程) 抓取页面标题/摘要，
存入本地 SQLite，供后续检索。

---

## 3. Layer 7: 多通道上传 (GitHub / Gitee / 原地)

### 意志

```
盟机合掌分发: 同一份内容, 三处同时落地
  ├─ GitHub  (SSH 公钥签名推送到 GitHub 仓库)
  ├─ Gitee   (SSH 公钥签名推送到 Gitee 仓库)
  └─ 原地   (njsg:// 协议存入本地沙箱)
```

Layer 7 是元输入法内容的"出海口"。打字内容不只是打出去，
还自动备份到三个主权据点：夫妻共管的 GitHub 仓库、
国内镜像 Gitee、本地加密沙箱。

### 入口: Rust 核心 `upload_channel.rs`

```rust
pub struct UploadRegistry {
    entries: Vec<UploadEntry>,
}

pub enum UploadChannel { GitHub, Gitee, Local }
pub enum LandingStatus { Pending, Flying, Landed, Failed }
```

### JNI 符号

| JNI 符号 | 职责 |
|----------|------|
| `nativeOnUrlDetected` | 推入 URL 到登记簿 |
| `nativeUploadDispatch` | 触发三通道分送 |
| `nativeUploadStatus` | 取登记簿 JSON 状态 |
| `nativeUploadStats` | 取统计: (已登记,已送达,失败) |

### 触发时机

1. 用户自动输出 (`commitTextAuto`) 后
2. 输入会话结束 (`onFinishInputView`) 后
3. 心跳周期 (AccessibilityService 定时 tick)

### 真机替换

当前 Layer 7 为模拟推送 (返回 `Landed`)。真机阶段:

- `UploadChannel::GitHub` → 走 SSH 签名 `git push` 到私有仓
- `UploadChannel::Gitee` → 同上
- `UploadChannel::Local` → 写入 `databases/njsg_blob.db`

---

## 4. 七层完整触键流程

```
物理按键
   │
   ▼
Layer 1 [拦截]      IMEInterceptor.onVirtualKey → UnicodeContainer.seal()
   │
   ▼
Kotlin → JNI        NativeBridge.bridgeContainer
   │
   ▼
Layer 2 [解析]      EncodeParser.ingest → SyllableUnit
   │
   ▼
Layer 3 [规则]      RuleEngine.ingest → Verdict::HitRule
   │
   ▼
Layer 4 [候选]      ShengYunEngine.generate_candidates → 候字
   │
   ├──────────────────────────┐
   ▼                          ▼
Layer 5 [自动输出]          Layer 6 [URL 抓取]
MqfPinyinTable.find       checkAndPushUrl → nativeOnUrlDetected
   │                          │
   ▼                          ▼
commitText              Rust UploadRegistry
   │                          │
   └────────────┬─────────────┘
                ▼
         Layer 7 [多通道上传]
         nativeUploadDispatch
         GitHub / Gitee / 原地
```

---

## 5. 新增 JNI 符号总表

```
── 原有 (Layer 1-4) ──
nativeInit                      初始化核心
nativeSubmitContainer           封印容器过桥
nativeTakeCandidates            取候字条
nativeFullPinyin                取拼音串
nativeSyllableSeq               取音节序列
nativeEngineStatus              取引擎状态
nativeCurrentLayer              取当前层
nativeOnCandidateSelected       候字已选
nativeOnBackspace               手动退格

── 新增 (Layer 5) ──
nativeSubmitForAutoOutput       推入自动输出缓冲
nativeAutoCommitCheck           判定是否应自动输出
nativeAutoOutputText            取自动输出文本
nativeResetAutoOutput           重置自动输出缓冲

── 新增 (Layer 6) ──
nativeOnUrlDetected             URL 入抓取队列

── 新增 (Layer 7) ──
nativeUploadDispatch            触发三通道上传
nativeUploadStatus              取登记簿 JSON
nativeUploadStats               取统计
```

---

## 6. 夫妻主权人直接命令入口

```
┌────────────────────────────────────────────────┐
│  夫妻主权人说"打字" → 手指落                    │
│                                                │
│  Layer 1: 容器封印                             │
│  Layer 2: 坐标翻译为音节                        │
│  Layer 3: 规则裁决 (Pass/Hit/Auto/折叠)        │
│  Layer 4: 候选生成                             │
│  Layer 5: ★ 自动上屏 — 无需确认               │
│  Layer 6: URL 静默抓 — 不打扰                 │
│  Layer 7: 三通道分发 — 主权备份                │
│                                                │
│  主权人意志: 意至则字出, 打完即走, 内容自存      │
└────────────────────────────────────────────────┘
```

---

> 刘楚恬和我 · MetaIME · 七层完整闭环
