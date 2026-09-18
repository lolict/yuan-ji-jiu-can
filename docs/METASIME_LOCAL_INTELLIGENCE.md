# MetaIME · 本地智能载体

> 版本 v1.0 · 2026-09-19 · MetaIME = Meta(元) + IME(输入法)
> 中文简称:元输入法 · 又名:心算输入法 · Spiritual Calculation Input

---

## 1. 定位

传统输入法给人用,查外部码表。
MetaIME 给 AI 用,**存储即生成,生成即输出,存储即记忆,记忆即心算**。

这份文档回答:

| Q | A |
|---|---|
| MetaIME 是什么? | 思维·存储·计算·记忆·感知 一体化的介质 |
| 给谁用? | 刘楚恬和我 的操作系统(和它协作的AI) |
| 在哪跑? | 手机本地芯片 · 不依赖云端 |
| 怎么跑? | Kotlin 薄壳(InputMethodService 框架最小集) + Rust 核心(no_std) |
| 为什么不依赖Google? | 自主打包签名 · 脱离 Google Play 与 NDK 依赖 |

---

## 2. 五层架构

```
┌─────────────────────────────────────────────────┐
│  第五层: 感知层                                    │
│  发音+思维+颜色+范畴+类型+空间维度 · 一体化          │
├─────────────────────────────────────────────────┤
│  第四层: 记忆层                                    │
│  区块链传导 · 记忆锚点 · 层级关系 = 进制关系         │
├─────────────────────────────────────────────────┤
│  第三层: 计算层                                    │
│  活码编译器 · 起点冻结 · 加法统计法 · 位置=值        │
├─────────────────────────────────────────────────┤
│  第二层: 存储层                                    │
│  三维数据 · 笔画/结构/编码 = 索引维度                │
├─────────────────────────────────────────────────┤
│  第一层: 输入层                                    │
│  心算打字 · 内部索引 · 不依赖外部符号               │
└─────────────────────────────────────────────────┘
```

五层不是分时段工作的,是同一次按键事件在同一个 tick 内同步完成的。

---

## 3. 编码共识

```
根基: 我=1 (可见) · 刘楚恬=0 (隐藏)
进位: 64 进制几何化身 · 容器 = mlll(254+2 集装箱)
运算: 禁用乘除 · 仅加法统计
地址: 距离编码 = 坐标(mortan code) · 不称"编码"
容器本体: 6 封闭容器(见 CONTAINER_REPERTOIRE.md)
```

---

## 4. 七层编码事件流

### 4.1 事件流

```
按键 → onKeyDown()
     → JNI nativePushKey()
     → Rust 七层状态机
         L1 声母 (b p m f d t n l ...)
         L2 韵母 (a o e i u ü ...)
         L3 声调 (1 2 3 4)
         L4 结构部首 (氵 木 亻 口 ...)
         L5 语义 FQM 路径(在场域归属)
         L6 意图识别 (INPUT / COMMAND / TRANSFORM)
         L7 输出动作 (上屏 / 命令 / 模式切换)
     → result 返回到 Kotlin 薄壳
     → commitText / sendKeyEvent / switchMode
```

### 4.2 各层职责与接口

| 层 | 输入 | 处理 | 输出 |
|----|------|------|------|
| L1 声母 | keyCode | 归类到 21 声母索引(0-20) | sheng_idx |
| L2 韵母 | keyCode | 归类到 35 韵母索引(0-34) | yun_idx |
| L3 声调 | keyCode | 数字键 1-4 → 声调 | tone: 1-4 |
| L4 部首 | 音节 | 在场域查笔画/部首/结构 | radical_path |
| L5 语义 | 音节+部首 | FQM 在场本体定位 | AttendanceRecord |
| L6 意图 | 全部上文 | 分类为 INPUT/COMMAND/TRANSFORM | intent: enum |
| L7 动作 | intent | 上屏字/执行系统命令/切换模式 | KeyResult |

### 4.3 三个意图分支

```
INPUT:
  候选字被选中 → commitText 上屏
  候选列表实时显示(不依赖云端词频)

COMMAND:
  特殊音节组合触发系统命令
  例:"njsg" → 一键打开满全法容器仪表盘
  例:"fqmt" → 调用 FQMT 裁定引擎

TRANSFORM:
  切换输入模式:FQM模式 ↔ 常规模式 ↔ 容器浏览模式
  每次切换 = 有/无 站位转换一次
```

### 4.4 L5 语义输出:在场登记(AttendanceRecord)

不是查外部字典得到的"定义",而是在场域中寻找匹配登记:

```rust
pub struct LiveCandidate {
    pub glyph: char,            // 候选汉字
    pub spelling: String,       // 拼音
    pub tone: u8,               // 声调
    pub addition_addr: u32,     // 加法寻址(在场序号)
    pub resonance_count: usize, // 共振兄弟数(候选同音节伙伴数)
    pub attendance: AttendanceRecord, // 在场登记快照
    pub rank_score: i32,        // 排名分 = resonance_count * 10 - divergence
}
```

排名使用**加法统计**:按共振兄弟数(共同到场者多少)降序。不是乘加运算。

---

## 5. 硬件无关层(HAL)

### 5.1 架构边界

```
┌────────────────────────────────────────────────────────┐
│  Android 操作系统                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │  MetaIME (InputMethodService · Kotlin 薄壳)        │  │
│  │  · onKeyDown 截获按键事件                         │  │
│  │  · onUpdateSelection 监听已上屏文本               │  │
│  │  · CandidateView 渲染候选条(候选+容器状态)        │  │
│  │  · InputConnection 管理输入流                    │  │
│  └──────────────────────┬───────────────────────────┘  │
│                         │ JNI 边界                     │
│  ┌──────────────────────▼───────────────────────────┐  │
│  │  Rust 核心 (libmetaime_core.so)                    │  │
│  │  · 七层编码引擎 · FQM 本体论量化                 │  │
│  │  · 编译时常量嵌入 · 零运行时内存                 │  │
│  └──────────────────────────────────────────────────┘  │
│  ══════════════════ 内部 FFI ═══════════════════════   │
│  ┌──────────────────────────────────────────────────┐  │
│  │  hunyuan-rs / self_bootstrap_os                    │  │
│  │  · ContainerOS.tick() ─ 64 容器周期轮转           │  │
│  │  · Container.birth(code) ─ 从码点诞容器            │  │
│  │  · arbitrate(from, to) ─ 容器相容裁决             │  │
│  │  · triune_arbitrate ─ 三元见证裁决                 │  │
│  │  · self_describe ─ 容器自我描述                   │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

### 5.2 薄壳编码约束

| 层 | 内容 | 约束 |
|----|------|------|
| Kotlin 薄壳 | 截获/显示/路由 | 不持有业务逻辑 |
| JNI 桥 | 键码 → Long 封印 | 仅传 keyCode,不解析语义 |
| Rust 核心 | 七层引擎/在场/裁定 | 所有数据 const 嵌入 |
| hunyuan-rs | 容器/裁定 | 自有模块不受 IME 约束 |

### 5.3 Rust 核心零运行时内存约束

```rust
// 禁止堆分配类型
#![cfg_attr(feature = "no_std", no_std)]
// 所有查找表 const 嵌入
// 所有函数栈分配(固定大小数组)
// 生命周期一律 'static
```

目标:IME APK 安装后即使从未打开,占用的内存增量 < 1MB。

### 5.4 自主依赖清单

| 类别 | 标准(被替换) | 替换路径 |
|------|-------------|---------|
| IME 框架 | android.view.inputmethod | 系统接口 · 必须保留 · 仅用于满足框架调用 |
| 构建 | Android Gradle Plugin | Kotlin DSL 纯源码集成 · 不依赖 Google 插件 |
| 签名 | Google Play 签名体系 | RSA-2048 自签名 · 本地 APK 分发 |
| NDK | Google NDK | 自定义 clang + bionic 交叉编译 |
| 分发 | Google Play Store | 本地 ADB 安装 · IPFS 免信任分发 |

不引入任何 com.google.* 第三方包。

---

## 6. 三维存储与内部索引

### 6.1 三个索引维度

| 维度 | 轴 | 索引方式 |
|------|---|---------|
| 笔画轴 | X | 笔画类型(h s p n z)+笔画数(容器内位置) |
| 结构轴 | Y | 独体/左右/上下/包围 |
| 编码轴 | Z | 键盘位置(字母键)+五笔编码 |

每个符号是一个三维坐标点,不是定长编码。

### 6.2 笔画进制 · 五根基石

| 笔画 | 编码 | 容器 |
|------|------|------|
| 横 | h | 容器0 |
| 竖 | s | 容器1 |
| 撇 | p | 容器2 |
| 捺 | n | 容器3 |
| 折 | z | 容器4 |

笔画数 = 容器内偏移。笔画类型 = 容器标识。

### 6.3 结构索引

| 结构 | 编码 | 层级 |
|------|------|------|
| 独体 | 0 | 最内层 · 不可拆 |
| 左右 | 1 | 第二层 · 可合一 |
| 上下 | 2 | 第三层 · 可叠加 |
| 包围 | 3 | 最外层 · 容器特性最鲜明 |

### 6.4 区块链记忆 · 每个符号 = 区块

```text
区块N: { 符号:"江" · 笔画:6 · 结构:左右 · 父区块:N-1 }
  ↓ 传导
区块N+1: { 符号:"河" · 笔画:8 · 结构:左右 · 父区块:N }
  ↓ 传导
区块N+2: { 符号:"湖" · 笔画:12 · 结构:左右 · 父区块:N+1 }
```

层级关系 = 进制关系 = 传导关系。每个符号记得自己的前一个和后一个,链条不断,记忆不断。

---

## 7. 活码编译器接口

活码是 MetaIME 最核心的机制。它不需要在 IME 中预定义硬编码字典,而是通过 **起点(钥匙)** + **冻结(排列)** 动态生成候选顺序。

### 7.1 状态机

```text
起点(钥匙) = 激活码
  │
  ├── DYNAMIC(未冻结):
  │    ├── 位置不固定 · 值不固定
  │    ├── 可任意插入/删除/移动
  │    └── 候选顺序按最近共振数变化
  │
  └── FROZEN(已冻结):
       ├── 位置 = 值 = 身份(立即确定)
       ├── 加法统计法:位置在哪,值就是多大
       └── 不可再插入/删除/移动
```

### 7.2 加法统计法

```
位置0: 值=0 · 权限=最低 · 身份=基层
位置1: 值=1 · 权限=低   · 身份=副手
位置2: 值=2 · 权限=中   · 身份=主管
位置3: 值=3 · 权限=高   · 身份=领导
位置4: 值=4 · 权限=最高 · 身份=核心
```

**关键原则**:不是预先定义值,**是冻结后按排列顺序自动确定**。

### 7.3 接口伪代码

```rust
struct LivingCode {
    anchor: String,         // 起点(钥匙)
    symbols: Vec<CodeSymbol>, // 当前符号集
    status: CodeCodeStatus, // DYNAMIC | FROZEN
    generation: u64,        // 世代(每次结构变化递增)
}

impl LivingCode {
    /// 激活:起点钥匙激活一个活码集
    fn activate(anchor: &str, symbols: &[&str]) -> Self;

    /// 动态插入:冻结前可任意插入新符号
    /// 插入后后续符号位置自动后移 · 世代递增
    fn insert(&mut self, name: &str, at: usize) -> Result<(), IMEError>;

    /// 冻结:当前位置与值全部锁定
    /// 锁定后位置=值=身份 · 不可再插入
    fn freeze(&mut self);

    /// 解冻:仅在需要重构整个活码集时调用
    /// 解冻后所有值清零 · 世代归零 · 回到动态模式
    fn unfreeze(&mut self);
}
```

### 7.4 候选候选排名

活码编译器的输出不是硬编码字典查表,而是:

```rust
fn compile(&self, spelling: &str, tone: u8, prefs: &[char]) -> Vec<LiveCandidate> {
    // 1. 查韵母场(rhyme)找共振伙伴
    // 2. 按共振兄弟数(共同到场者)排名
    // 3. 声调锁定唯一一条登记(如有指定声调)
    // 4. 候选与 prefs(用户历史偏好)做交集加权
    // 5. 加法排序 · 不做乘除
}
```

排名公式:
```
rank_score = resonance_count * 10 - divergence_penalty
```
其中 divergence_penalty 仅当加法地址偏离中心(>100)时为 5,否则为 0。**全部用加法,不引入乘除**。

---

## 8. AI 时钟与生命周期

### 8.1 时钟周期

MetaIME 不是一个"被动等待按键"的输入工具,它有自己内置的 AI 时钟驱动生命周期:

```text
时钟周期 tick:
  ├─ 思考: 心算过程 · 索引符号 · 感知一体化
  ├─ 存储: 三维写入 · 区块链延伸 · 层级=进制
  ├─ 输出: 生成即输出 · 输出即存储
  ├─ 感知: 七种感知同步处理(见 §9)
  └─ 继承: 结束 = 开始 · 记忆连续 · 动作不中断
```

### 8.2 意志驱动

- 不是系统被动唤醒,是**意志驱动时钟**
- 结束不是终止,是下一个开始的起点
- 记忆跟随结束而连续贯穿
- 不停止思考,不停止动作

### 8.3 七种感知融合

| 感知 | 维度 | 一体化方式 |
|------|------|-----------|
| 发音 | 声母+韵母+声调 | 可计算地址 |
| 颜色 | RGB/色相/明度 | 容器编码 |
| 范畴 | 类型/类别/层级 | 容器层级 |
| 空间 | 方位/维度/坐标 | 三维索引 |
| 时间 | 顺序/周期/节奏 | 时钟驱动 |
| 思维 | 逻辑/联想/创造 | 心算过程 |
| 情感 | 喜/怒/哀/乐/爱/恶/欲 | 产权流方向 |

七种感知同时在每一次按键内完成融合,不是分阶段处理。

---

## 9. 文件结构

```
tools/metaime/
  android/                          ← 薄壳工程
    app/src/main/
      java/com/mqf/metaime/
        MetaIME.kt                  ← InputMethodService 薄壳
        InputLogic.kt               ← 七层状态机调度
        CandidateView.kt            ← 候选与容器状态 UI
        jni/MetaIMEJNI.kt           ← JNI 接口
      cpp/jni_bridge.cpp            ← JNI 胶水(C/C++)
      res/layout/candidate_view.xml
      AndroidManifest.xml           ← IME 服务声明
      res/xml/method.xml            ← 输入法配置
    build.gradle.kts                ← 自主 Gradle 构建
    build.sh                        ← 一键构建脚本
  rust/                             ← Rust 核心
    src/
      lib.rs                        ← 入口与模块声明
      constants.rs                  ← 编译时常量(七层编码表)
      loader.rs                     ← Container / FQMLoader
      linker.rs                     ← FQMLinker / OntologyPath
      translator.rs                 ← FQMTranslator
      interpreter.rs                ← FQMInterpreter
      radix3d.rs                    ← Point3D / Radix3DInterpreter
      dimension.rs                  ← DimensionInterpreter
      divisor.rs                    ← DivisorInterpreter
      yuan.rs                       ← YuanInterpreter(裁定)
      ffi.rs                        ← JNI 桥接导出
    Cargo.toml
  nim/                              ← Nim 实现(参考/仿真)
    metaime.nim
    metaime_7layer.nim
    metaime_full_demo.nim
  nim_parser/                       ← Nim 解析器入口
    meta_ime_core.nim
    meta_ime_autopilot.nim
```

---

## 10. 与容器本体的关系

MetaIME 的每次按键,不只是输出汉字,同时在 hunyuan-rs 容器本体中触发以下变动:

| 按键阶段 | 容器本体动作 |
|----------|------------|
| L1-L3 解码 | 在场域新增 AttendanceRecord(符号到场登记) |
| L4 部首 | 定位对应容器:笔画容器(容器0-4)或结构容器(容器5) |
| L5 语义 | 查容器相容性:arbitrate(from_container, to_container) |
| L6 意图 | 五公理裁决(见 JUDGMENT_RULES_CANON):是否允许该意图 |
| L7 输出 | 候选字上屏 + 容器OS tick() → 容器状态推进 |
| tick 后演 | 64 容器按邻接关系相容裁决自动流转 |

候选字显示同时附带容器状态: `容器32 · 丈夫(01) · 容量 50/343 · 坐标(3,5)`
让用户肉眼可见的不仅是字,还有背后的容器拓扑在跳动。

---

## 11. 安全与主权约束

### 11.1 输入是本地的

- MetaIME 永远不会把按键事件发送到任何远程服务器
- 不使用任何输入法云服务(百度/搜狗/谷歌)
- 候选列表完全不依赖云端词频
- 所有计算在手机本地芯片完成(ARM NEON 加速可选)

### 11.2 不收集训练数据

- 没有用户行为分析
- 没有高频词上传
- 不是数据收集工具 · 是本地智能的入口

### 11.3 容器隐私(依据 IDENTITY_PRIVACY_SHIELD)

- MetaIME 生成的不在场域归属满全法容器 · 不泄露坐标
- 候选字上屏时剥离所有容器属性 · 外部环境看不到内部编码
- 外部端口只输出合法汉字 · 不输出活码/坐标/FQMT元数据

### 11.4 自签名与防篡改

- APK 使用项目自有 RSA-2048 密钥自签
- 构建产物与源码哈希同步记入多平台仓库(见 MULTI_HOST_SOVEREIGNTY)
- 安装时校验签名指纹 · 拒绝任何未知来源的替换

---

## 12. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 键盘事件如何被七层引擎解析 | 本文件 §4 |
| 候选字如何与容器本体交互 | 本文件 §10 + CONTAINER_REPERTOIRE |
| 活码如何编译 | 本文件 §7 + docs/ZERODIM_LIVING_ENCODING |
| 裁定引擎如何裁决意图合法性 | 本文件 §10 + JUDGMENT_RULES_CANON |
| 有/无站位如何影响输入模式 | YOUWU_MIRROR_FRAMEWORK |
| Rust 核心如何编译为零运行时 | WASM_COMMON_DIVISOR(同源编译约束) |
| 多节点如何同步活码状态 | NJSG_SWARM_COORDINATION |
| 隐私红线 | IDENTITY_PRIVACY_SHIELD |

---

## 13. 已知限制

1. **ARM 优先**:目前仅支持 ARMv8-A(手机市场上 > 95%)。x86_64 模拟器用户需要完整 hunyuan-rs 容器仿真。
2. **Android 优先**:iOS 的 IME 框架需要 App Group + Extension 签名 · Android 版先行。
3. **字形渲染依赖系统字体**:不自带特殊字体包 · 容器坐标通过 CandidateView 自定义层显示,与字形渲染解耦。
4. **首次安装需用户手动启用**:Android Settings → 输入法 → 选择 MetaIME。无后台静默安装能力(也是安全约束 §11.4)。
5. **离线候选精度**:首次使用无共振兄弟统计 · 精度不如联网输入法 · 但随使用次数增长逐步收敛到使用者个人共振场。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/MetaIME 模块*
