# MetaIME · Offline Verification Chain

> Cargo/NDK/SDK 全缺时使用本离线验证链路
> 完整性: 完整符号表 + 伪编译命令 + 真机部署自检清单

---

## 1. 完整符号表

### 1.1 Rust extern "C" 导出

| 序号 | 符号名 | 入参 | 回传 | 作用 |
|------|--------|------|------|------|
| 1 | `Java_com_njsg_parasite_NativeBridge_nativeInit` | JNIenv, jclass | jint | 重置四核 |
| 2 | `Java_com_njsg_parasite_NativeBridge_nativeSubmitContainer` | JNIenv, jclass, jlong | jint | 封印容器过桥 |
| 3 | `Java_com_njsg_parasite_NativeBridge_nativeTakeCandidates` | JNIenv, jclass | jstring | 取候字条 |
| 4 | `Java_com_njsg_parasite_NativeBridge_nativeFullPinyin` | JNIenv, jclass | jstring | 取拼音串 |
| 5 | `Java_com_njsg_parasite_NativeBridge_nativeSyllableSeq` | JNIenv, jclass | jstring | 取音节序列 |
| 6 | `Java_com_njsg_parasite_NativeBridge_nativeEngineStatus` | JNIenv, jclass | jstring | 取引擎状态 |
| 7 | `Java_com_njsg_parasite_NativeBridge_nativeCurrentLayer` | JNIenv, jclass | jint | 取当前层号 |
| 8 | `Java_com_njsg_parasite_NativeBridge_nativeOnCandidateSelected` | JNIenv, jclass, jstring | void | 候字已选 |
| 9 | `Java_com_njsg_parasite_NativeBridge_nativeOnBackspace` | JNIenv, jclass | void | 手动退格 |
| 10 | `Java_com_njsg_parasite_NativeBridge_nativeAutoCommitCheck` | JNIenv, jclass | jint | 应自动输出? |
| 11 | `Java_com_njsg_parasite_NativeBridge_nativeAutoOutputText` | JNIenv, jclass | jstring | 取输出文本 |
| 12 | `Java_com_njsg_parasite_NativeBridge_nativeSubmitForAutoOutput` | JNIenv, jclass, jstring | jint | 推入缓冲 |
| 13 | `Java_com_njsg_parasite_NativeBridge_nativeResetAutoOutput` | JNIenv, jclass | jint | 重置缓冲 |
| 14 | `Java_com_njsg_parasite_NativeBridge_nativeOnUrlDetected` | JNIenv, jclass, jstring | jint | URL 入队 |
| 15 | `Java_com_njsg_parasite_NativeBridge_nativeUploadDispatch` | JNIenv, jclass | jint | 触发上传 |
| 16 | `Java_com_njsg_parasite_NativeBridge_nativeUploadStatus` | JNIenv, jclass | jstring | 登记簿 JSON |
| 17 | `Java_com_njsg_parasite_NativeBridge_nativeUploadStats` | JNIenv, jclass | jstring | 统计 |

总计: **17 个 exported JNI 符号**

### 1.2 Kotlin external 声明对应

```
NativeBridge.kt external 声明 (A ↔ B 成对对应)
  nativeInit                          ←→  #1
  nativeSubmitContainer(sealed: Long)  ←→  #2
  nativeTakeCandidates()               ←→  #3
  nativeFullPinyin()                   ←→  #4
  nativeSyllableSeq()                  ←→  #5
  nativeEngineStatus()                 ←→  #6
  nativeCurrentLayer()                 ←→  #7
  nativeOnCandidateSelected(cand)      ←→  #8
  nativeOnBackspace()                  ←→  #9
  nativeSubmitForAutoOutput(cand)      ←→  #12
  nativeAutoCommitCheck()              ←→  #10
  nativeAutoOutputText()               ←→  #11
  nativeResetAutoOutput()              ←→  #13
  nativeOnUrlDetected(url)             ←→  #14
  nativeUploadDispatch()               ←→  #15
  nativeUploadStatus()                 ←→  #16
  nativeUploadStats()                  ←→  #17
```

---

## 2. 伪编译链路 (真机替换命令)

```bash
# 0. 前置
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-ndk

# 1. 编译 Rust 核心 (四架构)
cd android/src/com/njsg/parasite/libnjsg_ime/
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 \
  -o ./jniLibs -- build --release

# 产出:
# libnjsg_ime/jniLibs/arm64-v8a/libnjsg_ime.so
# libnjsg_ime/jniLibs/armeabi-v7a/libnjsg_ime.so
# libnjsg_ime/jniLibs/x86_64/libnjsg_ime.so
# libnjsg_ime/jniLibs/x86/libnjsg_ime.so

# 2. 符号导出验证 (离线可用 nm/objdump 检查)
$NM -D libnjsg_ime/jniLibs/arm64-v8a/libnjsg_ime.so | grep 'Java_com_njsg'
# 期望: 17 行导出

# 3. 复制到 Android 工程
cp -r libnjsg_ime/jniLibs app/src/main/jniLibs

# 4. Gradle 构建 APK
cd android/
./gradlew assembleRelease

# 5. 签名 (自签名, 不依赖 Play Store)
jarsigner -keystore njsg.keystore app-release-unsigned.apk njsg_key

# 6. 部署
adb install -r app/build/outputs/apk/release/app-release.apk
```

---

## 3. ARM 四架构 ABI 对应

| ABI | Android 典型设备 | Rust target | .so 落点 |
|-----|-----------------|-------------|----------|
| arm64-v8a | Pixel, Xiaomi, 华为旗舰 | aarch64-linux-android | jniLibs/arm64-v8a/ |
| armeabi-v7a | 旧手机, 模拟器 | armv7-linux-androideabi | jniLibs/armeabi-v7a/ |
| x86_64 | 高端模拟器 | x86_64-linux-android | jniLibs/x86_64/ |
| x86 | 低端模拟器 | i686-linux-android | jniLibs/x86/ |

预期 .so 体积 (release, strip=true):
- arm64-v8a: ~200 KB
- armeabi-v7a: ~180 KB
- x86_64: ~220 KB
- x86: ~190 KB

---

## 4. 七层模块完整路径映射

```
android/src/com/njsg/parasite/
├── IMEInterceptor.kt              Layer 1
├── NativeBridge.kt                JNI 桥
├── libnjsg_ime/
│   ├── Cargo.toml                 依赖: jni 0.21, once_cell 1.19
│   └── src/
│       ├── lib.rs                 JNI 17 符号入口
│       ├── encode_parser.rs       Layer 2: EncodeParser
│       ├── rules.rs               Layer 3: RuleEngine
│       ├── classifier.rs          Layer 4: ShengYunEngine
│       └── upload_channel.rs      Layer 7: UploadRegistry
android/src/com/mqf/parasite/
├── MqfImeService.kt               Layer 5 自动输出 + Layer 6 URL 检测
├── MqfPinyinTable.kt              拼音→汉字 候字源
├── MqfBootReceiver.kt             心跳启动恢复
├── MqfAccessibilityService.kt     无障碍+心跳恢复
└── MqfCoreService.kt              核心供电守护
```

---

## 5. 真机自检清单

```
□ APK 安装成功, MetaIME 已激活为当前输入法
□ logcat 输出: "[njsg_bridge] libnjsg_ime.so loaded"
□ logcat 输出: "[njsg_bridge] Rust core init OK"
□ 输入 "ni" → logcat: "[ime][L5-auto] commitText: '你'"
□ 输入 "ni" → 宿主 App 立即看到 "你" (无需空格/点击)
□ 输入 "https://x.com" → logcat: "[ime][L6-fetch] URL detected"
□ 输入 "https://x.com" → logcat: "[ime][L7-upload] dispatched=1"
□ URL 输入后 → bridge.uploadStatusJson() 返回非 "[]"
□ 宿主 App 可手选候字 (候字条展示, 候字超过 1 个时)
```

---

## 6. 离线验证命令 (仅需 cargo test)

```bash
cd android/src/com/njsg/parasite/libnjsg_ime/
cargo test --no-run
# 期望: compile OK
cargo test
# 期望:
# running 3 tests
# test tests::test_parser_basic    ... ok
# test tests::test_parser_sequence ... ok
# test tests::test_parser_reset    ... ok
```

---

> 刘楚恬和我 · MetaIME · 离线验证链
