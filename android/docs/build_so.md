# MetaIME libnjsg_ime.so Build Guide

> NJSG MetaIME · Android IME · Seven-Layer Pipeline

---

## 1. Source Location

```
android/src/com/njsg/parasite/libnjsg_ime/
  Cargo.toml
  src/
    lib.rs          # JNI entry
    encode_parser.rs  # Layer 2: code point + sheng/yun coords -> syllable
    rules.rs        # Layer 3: rule engine
    classifier.rs   # Layer 4: candidate generator
```

Output: `libnjsg_ime.so` (Android shared library, loaded by NativeBridge via JNI)

---

## 2. Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# NDK (via Android Studio or sdkmanager)
sdkmanager --install "ndk;25.2.9519653"
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653

# cargo-ndk
cargo install cargo-ndk

# Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

---

## 3. Build

```bash
cd android/src/com/njsg/parasite/libnjsg_ime/
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 \
  -o ./jniLibs -- build --release
```

Output:
```
libnjsg_ime/jniLibs/
  arm64-v8a/libnjsg_ime.so
  armeabi-v7a/libnjsg_ime.so
  x86_64/libnjsg_ime.so
  x86/libnjsg_ime.so
```

Copy to Android project:
```bash
cp -r libnjsg_ime/jniLibs/* ../../jniLibs/
```

---

## 4. Seven-Layer Pipeline Mapping

| Layer | Name | Location | Role |
|-------|------|----------|------|
| 1 | Key Intercept | IMEInterceptor.kt | Capture + seal 64-bit container |
| 2 | Encode Parse | encode_parser.rs | Coords -> syllable |
| 3 | Rule Engine | rules.rs | Sheng/yun pattern -> verdict |
| 4 | Classifier | classifier.rs | Coords -> candidate hanzi |
| 5 | Auto Output | MqfImeService.kt | Candidate -> commitText |
| 6 | Content Fetch | (reserved) | Background URL fetch |
| 7 | Multi-Upload | (reserved) | Multi-platform sync |

JNI symbols exported:
- nativeInit / nativeSubmitContainer / nativeTakeCandidates
- nativeFullPinyin / nativeSyllableSeq
- nativeEngineStatus / nativeCurrentLayer
- nativeOnCandidateSelected / nativeOnBackspace

---

## 5. Release Profile

```toml
[profile.release]
opt-level = "s"   # optimize for size
strip = true      # strip symbols
panic = "abort"   # no unwinding
crates-type = ["cdylib"]
```

Per-ABI size: approx 150-300 KB (pure Rust, no external C deps).

---

## 6. On-Device Acceptance

```bash
# Deploy APK
adb install -r build/outputs/apk/release/*.apk

# Enable IME in system settings, then:
adb logcat | grep -E "njsg|mqf"
```

Expected:
```
[njsg_bridge] libnjsg_ime.so loaded
[njsg_bridge] Rust core init OK
[njsg_ime] seven-layer pipeline ON
```

Acceptance flow:
1. Type "n" -> sealed -> JNI -> EncodeParser accumulates sheng idx=6
2. Type "i" -> sealed -> JNI -> EncodeParser composes syllable "ni"
      -> RuleEngine HitRule -> ShengYunEngine candidates
      -> Show candidate bar
3. Space / tap -> commitText("...") -> Host app receives glyph

---

## 7. Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| UnsatisfiedLinkError | .so missing or ABI mismatch | Check jniLibs + abiFilters |
| nativeInit != 0 | Rust panic at startup | Check logcat |
| No candidates | Sheng/yun coord mismatch | Verify IMEInterceptor maps align with EncodeParser |
| No commitText | InputConnection dead | Verify IME service lifecycle |

---

## 8. Next Steps

- Layer 6: Background URL fetch
- Layer 7: Multi-platform sync
- Dynamic sheng/yun table hot-reload
- MCCP FSM state integration

---

> Liu Chutian & me (NJSG) · MetaIME Seven-Layer Pipeline
