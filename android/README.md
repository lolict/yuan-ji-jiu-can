# 元输入法 MetaIME · Android 壳工程

满全法 / 囡囝共生 — 意识互联网输入法入口

## 架构

```
┌─────────────────────────────────────────────────────┐
│  Kotlin 薄壳 (本目录)                                │
│  MetaIMEService  → InputMethodService               │
│  MetaIMENative   → JNI 封装                         │
├─────────────────────────────────────────────────────┤
│  Rust 核心 (hunyuan-rs/)                             │
│  src/bin/metaime_jni.rs → JNI 桥接 + C-ABI FFI       │
│  src/metaime.rs         → 字典编译期内嵌 + 查询        │
└─────────────────────────────────────────────────────┘
```

## 构建步骤

### 1. 编译 Rust → .so 动态库

前置: cargo-ndk + Android NDK

```bash
# 安装 cargo-ndk
cargo install cargo-ndk

# 设置 NDK 路径
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653

# 交叉编译到三种目标架构
cd ../hunyuan-rs
cargo ndk -t arm64-v8a -o ../android/app/src/main/jniLibs build --release --bin metaime_jni
cargo ndk -t armeabi-v7a -o ../android/app/src/main/jniLibs build --release --bin metaime_jni
cargo ndk -t x86_64 -o ../android/app/src/main/jniLibs build --release --bin metaime_jni
```

### 2. 构建 APK

```bash
cd ../android
./gradlew assembleDebug
# 产物: app/build/outputs/apk/debug/app-debug.apk
```

### 3. 安装到手机

```bash
adb install -r app/build/outputs/apk/debug/app-debug.apk
adb shell ime enable com.manquanca.metaime/.MetaIMEService
adb shell ime set com.manquanca.metaime/.MetaIMEService
```

## 项目管理

| 文件 | 协议 |
|------|------|
| 囡囝共生 hunyuan-rs/ | Proprietary (私有) |
| android/ | Proprietary (私有) |

所有权保留 — 刘楚恬 & lolict
