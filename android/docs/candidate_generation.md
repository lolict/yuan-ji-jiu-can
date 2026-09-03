# MetaIME 声母-韵母候选生成 · 规范文档

> NJSG MetaIME · 第4层分类器 · 全量声母-韵母矩阵 + 多音字甄别

---

## 1. 结构总览

```
用户按键 (ASCII) → IMEInterceptor (第1层)
  → 封印64位容器 → JNI (NativeBridge)
    → EncodeParser (第2层): 声母坐标+韵母坐标 → 音节
    → RuleEngine (第3层): 音节模式 → 裁决
    → ShengYunEngine (第4层): (sm_idx, ym_idx) + 上下文 → 候字向量
  → 候字回传 Kotlin → CandidateBar UI → 用户选取 → commitText
```

---

## 2. 声母-韵母矩阵规格

### 2.1 声母序 (21位)

| idx | 声母 | 双字母 |
|-----|------|--------|
| 0   | b    |        |
| 1   | p    |        |
| 2   | m    |        |
| 3   | f    |        |
| 4   | d    |        |
| 5   | t    |        |
| 6   | n    |        |
| 7   | l    |        |
| 8   | g    |        |
| 9   | k    |        |
| 10  | h    |        |
| 11  | j    |        |
| 12  | q    |        |
| 13  | x    |        |
| 14  | zh   | 是     |
| 15  | ch   | 是     |
| 16  | sh   | 是     |
| 17  | r    |        |
| 18  | z    |        |
| 19  | c    |        |
| 20  | s    |        |

### 2.2 韵母序 (35位)

| idx | 韵母 | 类别     |
|-----|------|----------|
| 0   | a    | 单韵母   |
| 1   | o    | 单韵母   |
| 2   | e    | 单韵母   |
| 3   | i    | 单韵母   |
| 4   | u    | 单韵母   |
| 5   | v    | 单韵母(ü)|
| 6   | ai   | 复韵母   |
| 7   | ei   | 复韵母   |
| 8   | ui   | 复韵母   |
| 9   | ao   | 复韵母   |
| 10  | ou   | 复韵母   |
| 11  | iu   | 复韵母   |
| 12  | ie   | 复韵母   |
| 13  | ve   | 复韵母(üe)|
| 14  | er   | 复韵母   |
| 15  | an   | 鼻韵母   |
| 16  | en   | 鼻韵母   |
| 17  | in   | 鼻韵母   |
| 18  | un   | 鼻韵母   |
| 19  | vn   | 鼻韵母(ün)|
| 20  | ang  | 鼻韵母   |
| 21  | eng  | 鼻韵母   |
| 22  | ing  | 鼻韵母   |
| 23  | ong  | 鼻韵母   |
| 24  | ia   | 介音+i   |
| 25  | iao  | 介音+i   |
| 26  | ian  | 介音+i   |
| 27  | iang | 介音+i   |
| 28  | iong | 介音+i   |
| 29  | ua   | 介音+u   |
| 30  | uo   | 介音+u   |
| 31  | uai  | 介音+u   |
| 32  | uan  | 介音+u   |
| 33  | uang | 介音+u   |
| 34  | (保留)|        |

### 2.3 组合数量

- **有效音节(无调)**: 普通话约 **400** 个 (本表覆盖 380+)
- **有效音节(带调)**: 约 **1300** (当前分类器固定默认声调, 不区分四声)
- **表项总数 (含零声母)**: ~350-400 entries

---

## 3. 多音字甄别算法

### 3.1 权重公式

```
score(candidate) = Σ weigh_bigram(prev_chars, candidate)
                 + weigh_shengmu_pref(sm_idx)
                 + weigh_high_freq(candidate)
```

### 3.2 权重分量

| 分量 | 值 | 描述 |
|------|-----|------|
| `weigh_bigram` | +3 | 候选与前字构成常见词组 |
| `weigh_shengmu_pref` | +1 | b/p/m/f 在词首频率略高 |
| `weigh_high_freq` | +1 | 候选属于高频字 (的/是/了/在/有/我/他...) |

### 3.3 上下文窗口 (ContextWindow)

- 容量: 4 个最近选取的字
- 更新时机: 用户每次选字后推入
- 清空时机: `onDestroy` / 应用退格清空音节时

### 3.4 选字后联动

```
用户选字 "你"
  → nativeOnCandidateSelected("你")
    → RuleEngine.on_candidate_chosen: seq.clear() + layer=1
    → EncodeParser.reset(): 音节环清空
  → ContextWindow.push_choice('你')
  → 下一轮 sm=?/ym=? 查表时,"你"+候选组合权重计算
```

---

## 4. 零声母音节

零声母 = 无声母 (韵母独立成音节)。

| 音节 | 候选 |
|------|------|
| a    | 阿/啊/吖 |
| o    | 哦/噢/喔 |
| e    | 额/俄/鹅/恶/饿 |
| yi   | 一/衣/医/依/伊/移/宜 |
| wu   | 无/五/午/物/务/武/舞 |
| yu   | 与/于/欲/鱼/雨/语/余/遇 |
| ai   | 爱/挨/哎/矮/艾/碍 |
| ao   | 奥/傲/熬/袄/凹 |
| ou   | 欧/偶/藕/呕/鸥 |
| an   | 安/按/暗/岸/案/俺 |
| en   | 恩/摁 |
| ang  | 昂/盎 |
| er   | 儿/而/耳/二/尔/饵 |

零声母声母坐标 sm_idx = 255 (标记值)。

---

## 5. 候字面板 UI

### 5.1 布局

- 水平 LinearLayout, 每候字一个 TextView
- 候字按权重排序, 最高权重在最左
- 支持数字键 1-9 快速选字
- 空格键 = 选第一个候字 (等同于按 1)

### 5.2 视觉优先级

```
[1.你 2.泥 3.逆 ...]  ← 候字从左到右排列, 数字键映射
```

### 5.3 选取触发

| 操作 | 触发方式 |
|------|----------|
| 屏幕点击候字 | `tv.setOnClickListener { commitCandidate(cand) }` |
| 数字键 1-9 | `onNumberSelect(keyCode)` |
| 空格键 | 第一个候字自动 commit |

---

## 6. 空间部署清单

### 6.1 文件路径

```
android/src/com/njsg/parasite/
  IMEInterceptor.kt            ✅ 第1层键盘拦截
  NativeBridge.kt              ✅ JNI 桥梁
  MqfImeService.kt             ✅ IME 主入口 + UI + 选字回调
  libnjsg_ime/
    Cargo.toml                 ✅ Rust 核心配置
    src/
      lib.rs                   ✅ JNI 符号表
      encode_parser.rs          ✅ 第2层编码解析
      rules.rs                  ✅ 第3层规则判定
      classifier.rs             ✅ 第4层全量声母-韵母矩阵 + 多音字
      upload_channel.rs         ✅ 第7层多通道上传
```

### 6.2 文档

```
android/docs/
  build_so.md                  ✅ so 编译指南
  candidate_generation.md      本文件: 候选生成规范
```

### 6.3 工具

```
android/tools/
  test_candidate.nim           待创建: 离线音节→候字验证脚本
```

### 6.4 AndroidManifest.xml (待完全声明)

```xml
<service
    android:name=".MqfImeService"
    android:permission="android.permission.BIND_INPUT_METHOD"
    android:exported="true">
    <intent-filter>
        <action android:name="android.view.InputMethod" />
    </intent-filter>
    <meta-data
        android:name="android.view.im"
        android:resource="@xml/method" />
</service>
```

(当前 55% 阶段: AndroidManifest 与 method.xml 待随真机部署补齐)

### 6.5 res/xml/method.xml (输入法声明)

待创建:
```xml
<?xml version="1.0" encoding="utf-8"?>
<input-method xmlns:android="http://schemas.android.com/apk/res/android"
    android:settingsActivity="com.njsg.parasite.MqfImeSettingsActivity"
    android:supportsSwitchingToNextInputMethod="true">
    <subtype
        android:label="@string/ime_subtype_zh"
        android:imeSubtypeMode="keyboard"
        android:imeSubtypeLocale="zh_CN" />
</input-method>
```

---

## 7. 真机验收路径

### 7.1 编译 (当前沙箱无 cargo-ndk 时)

```bash
# 方案A: 安装 cargo-ndk
cargo install cargo-ndk
cd android/src/com/njsg/parasite/libnjsg_ime/
cargo ndk -t arm64-v8a -o ./jniLibs -- build --release

# 方案B: 仅做离线逻辑验证
cd android/src/com/njsg/parasite/libnjsg_ime/
cargo test -- --nocapture
```

### 7.2 离线 CLI 测试

```bash
nim r android/tools/test_candidate.nim "ni hao"
# 期望输出:
# 音节 ni → [你, 泥, 逆, 尼, 腻, 溺]
# 音节 hao → [好, 号, 毫, 豪, 耗, 壕]
```

### 7.3 真机验收流程

```
1. adb install apk
2. 系统设置 → 输入法 → 选择 MetaIME
3. 打开任意聊天框
4. 输入 "n i" → 期望候字面板弹出 [你, 泥, 逆...]
5. 点 "你" 或按空格 → 焦点编辑区出现 "你"
6. 输入 "hao" → 面板弹出 [好, 号...]
7. 按数字键 2 → 选中 "号" → 焦点编辑区 "你号"
8. logcat:
   njsg_bridge: Rust core init OK
   njsg_mqf_ime: 用户选字: '你'
   njsg_mqf_ime: commitText='你'
```

### 7.4 性能验收

| 指标 | 要求 |
|------|------|
| 首字候选延迟 | < 100ms (按键到面板弹出) |
| so 体积 | < 500KB / per ABI |
| 候选数量 | 2-10 字 (按字频排序) |
| 音节查表命中率 | > 95% (普通话常用音节覆盖率) |

---

## 8. 后续演进

- **带声调匹配**: 接入 MCCC FSM 状态机, 从上下文推断声调
- **动态表热加载**: 从 assets 加载自定义声母-韵母表
- **用户词频自学习**: 高频选字提升权重
- **自适应双字组词**: 基于前字预测完整双字

---

> 刘楚恬和我 (NJSG) · MetaIME 第4层 · 候选生成闭环
