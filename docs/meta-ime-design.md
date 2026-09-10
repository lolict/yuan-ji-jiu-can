# MetaIME Android 设计文档

> 基于 hunyuan-rs 容器系统的元输入法 · Kotlin 薄壳 + Rust 核心(JNI)

---

## 一、系统架构图

```
┌─────────────────────────────────────────────────────────┐
│  Android 操作系统                                        │
│  ┌─────────────────────────────────────────────────────┐│
│  │  Kotlin 薄壳 (MetaIMEService + NativeBridge)        ││
│  │  ┌──────────┐  ┌──────────────┐  ┌───────────────┐ ││
│  │  │InputMethod│  │ IMEInterceptor│  │ NativeBridge  │ ││
│  │  │ Service   │──▶   (第1层)    │──▶   (JNI桥)     │ ││
│  │  │(候选显示) │  │键盘拦截→封印  │  │sealed: Long  │ ││
│  │  └──────────┘  └──────────────┘  └───────┬───────┘ ││
│  └──────────────────────────────────────────┼─────────┘│
│  ══════════════════ JNI 边界 ════════════════╪══════════│
│  ┌──────────────────────────────────────────┼─────────┐│
│  │  Rust 核心 (libnjsg_ime.so)              │         ││
│  │  ┌──────────────────────────────────────┐│         ││
│  │  │  EncodeParser(第2层) ─ 声母韵母配对   │◀┘         ││
│  │  │  RuleEngine(第3层) ─ Pass/HitRule    ││          ││
│  │  │  ShengYunEngine(第4层) ─ 候选生成    ││          ││
│  │  │  UploadRegistry(第7层) ─ 多通道上传  ││          ││
│  │  └──────────────┬───────────────────────┘│          ││
│  └─────────────────┼────────────────────────┘          │
│  ══════════════════ 内部 FFI ══════════════════════════│
│  ┌─────────────────┼────────────────────────────────┐  │
│  │  hunyuan-rs ◀───┘                                │  │
│  │  ┌──────────────────────────────────────────┐    │  │
│  │  │ self_bootstrap_os 模块                    │    │  │
│  │  │  ContainerOS.tick() ─ 64容器周期轮转      │    │  │
│  │  │  Container.birth(code) ─ 从码点诞容器     │    │  │
│  │  │  arbitrate(from,to) ─ 容器相容裁决        │    │  │
│  │  │  triune_arbitrate ─ 三元见证裁决           │    │  │
│  │  │  self_describe ─ 容器自我描述             │    │  │
│  │  └──────────────────────────────────────────┘    │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## 二、数据流向(按键 → 容器态转换 → 候选显示)

```
按键事件(n)
   │
   ▼
IMEInterceptor.dispatchChar()
   ├─ 声母归类: n → idx=6
   ├─ 韵母归类: n → 255(非韵母)
   └─ 封印为 Long: UnicodeContainer.seal()
          │
          ▼
   NativeBridge.bridgeContainer(c: UnicodeContainer)
          │
          ▼ (JNI)
   Java_com_njsg_parasite_NativeBridge_nativeSubmitContainer()
          │
          ├─ EncodeParser.ingest()  → 音节产出
          └─ RuleEngine.ingest()    → 裁决 HitRule/Pass
                │
                ▼ (当选字被点击)
   nativeOnCandidateSelected()
          │
          ├─ hunyuan_rs::self_bootstrap_os::ContainerOS.tick()
          │     64容器按邻接关系相容裁决自动流转
          │
          ├─ Container::inject() → capacity 变化
          │
          └─ self_describe() → 候选栏文本回传
                │
                ▼
   nativeTakeCandidates() → Kotlin 候选栏刷新
         显示: "容器32 · 丈夫(01) · 容量 50/343 · 坐标(3,5)"
   (外壳显示: 当前容器码 + 极性 + 几何坐标)
```

候选栏第一行 = hunyuan-rs 容器状态，第二行 = 拼音候选字。容器态每 tick 更新一次，用户每次选字即触发一次 tick，候选栏同步反映容器内压、极性切换和邻接裁决日志。

---

## 三、关键 hunyuan-rs API 调用方案

| 层级 | 调用方 | API | 用途 |
|------|--------|-----|------|
| 初始化 | NativeBridge.nativeInit() | `ContainerOS::new()` | 在 Rust 核心内部创建 64 容器初始状态 |
| 按键输入 | nativeOnCandidateSelected() | `os.inject_to(code, amount)` | 选字后向指定容器注入内容，触发容量变化 |
| 状态流转 | nativeOnCandidateSelected() | `os.tick()` | 推进一个生产周期，所有容器按邻接相容性自动裁决传递 |
| 候选显示 | nativeTakeCandidates() | `self_describe(&container)` | 生成容器自我描述文本 |
| 极性显示 | nativeTakeCandidates() | 直接读 `container.polarity` | 00=独立, 01=夫, 10=妻, 11=合 |
| 容量显示 | nativeTakeCandidates() | 读 `container.capacity / max_capacity` | 妻216, 夫343, 独立1664, 夫妻2223 |
| 相容判定 | tick() 内部 | `arbitrate(from, to)` | 两容器间传输许可: Compatible/Incompatible/Conditional |
| 三元裁决 | Conditional 时 | `triune_arbitrate(primary, w1, w2)` | 见证裁定是否允许跨极性传输 |
| 事件日志 | nativeEngineStatus() | `os.summary()` + `os.log_tail(n)` | 周期数 / 容器数 / 活动记录(RingBuffer, ≤256条) |

---

## 四、候选栏交互映射

用户点击候选字时：

```
 候选栏容器状态行:  [容器32 · 丈夫(01) · 容量 50/343 · 坐标(3,5)]
                      ↓ 点击 ↓
 候选栏拼音候选行:  [你][我][泥][逆] → 选择"你"
                      ↓
 Rust 核心:
   ContainerOS.inject_to(32, 10)  → 容器32容量 50→60
   ContainerOS.tick()             → 容器32邻接传递 时钟+1
   self_describe()                → "容器32 · 丈夫(01) · 容量 45/343 · 坐标(3,5)"
                                   (容量因传递给邻接容器而减少)
```

候选文字符本身仍由第4层 ShengYunEngine 供给，容器态仅作为"元层"展示在候选栏顶部一行，不打断正常拼音输入流程。整个架构使 hunyuan-rs 容器系统在 Android 输入法中以"后台活性层"形式持续运行，每个汉字选择即是一次容器态演化。
