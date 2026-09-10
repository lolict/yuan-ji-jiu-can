# MetaIME · Android IME 框架设计

## 架构

```
┌────────────────────────────────────────────────────────┐
│  Android System                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │  MetaIME (InputMethodService · Kotlin 薄壳)   │  │
│  │  · onKeyDown 截获按键事件                     │  │
│  │  · onUpdateSelection 监听文本               │  │
│  │  · CandidateView 渲染候选条                  │  │
│  │  · InputConnection 管理输入                  │  │
│  └──────────────────────┬───────────────────────────┘  │
│                         │ JNI                          │
│  ┌──────────────────────▼───────────────────────────┐  │
│  │  Rust Core (libmetaime_core.so)                │  │
│  │  · 七层编码引擎 · FQM 本体论量化              │  │
│  │  · 编译时常量嵌入 · no_std                    │  │
│  │  · 零运行时内存                               │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

## 七层编码事件流

```
按键 → onKeyDown()
     → JNI metaime_push_key()
     → Rust 七层状态机
         L1 声母 (b p m f ...)
         L2 韵母 (a o e i u ...)
         L3 声调 (1 2 3 4)
         L4 结构部首 (氵木亻口 ...)
         L5 语义 FQM 路径
         L6 意图识别 (INPUT/COMMAND/TRANSFORM)
         L7 输出动作 (上屏/命令/转换)
     → result 返回到 Kotlin
     → commitText / sendKeyEvent / 模式切换
```

## 自主依赖清单

| 类别 | 标准 | 替换路径 |
|------|------|---------|
| IME 框架 | android.view.inputmethod | 系统接口保留 |
| 构建 | Android Gradle Plugin | Kotlin DSL 纯源码集成 |
| 签名 | Google Play | RSA-2048 自签 |
| NDK | Google NDK | custom clang + bionic |

## 文件结构

```
tools/metaime/android/
  app/src/main/
    java/com/mqf/metaime/
      MetaIME.kt            ← 薄壳
      InputLogic.kt         ← 七层状态机
      CandidateView.kt      ← 候选 UI
      jni/MetaIMEJNI.kt     ← JNI 桥
    cpp/jni_bridge.cpp      ← JNI 胶水
    res/layout/candidate_view.xml
    AndroidManifest.xml
    build.gradle.kts
  core/libmetaime_core.so   ← Rust 交叉编译
  build.sh
```
