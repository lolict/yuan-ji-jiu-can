#!/usr/bin/env bash
# ============================================================
# MetaIME Android 构建脚本
#
# 用法: ./build.sh [debug|release] [签名路径]
#
# 前提:
#   - 安装 Android SDK + NDK
#   - 安装 Rust + 三架构 target
#   - export ANDROID_HOME=/path/to/android-sdk
#
# 流程:
#   1. cargo build --target=... (三架构)
#   2. CMake 构建 jni_bridge.so
#   3. 打包 APK
#   4. 可选: 自主签名
# ============================================================

set -euo pipefail

MODE="${1:-debug}"
KEY_PATH="${2:-$HOME/.mqf/mqf.keystore}"

export ANDROID_HOME="${ANDROID_HOME:?请设置 ANDROID_HOME}"
export NDK="$ANDROID_HOME/ndk/25.2.9519653"

#项目根DIR="$(cd "$(dirname "$0")" && pwd)"
CORE_SRC="$DIR/../rust"
JNI_APP="$DIR/app"

echo "=== MetaIME 构建 [$MODE] ==="
echo "  ANDROID_HOME: $ANDROID_HOME"
echo "  NDK:          $NDK"
echo ""

# Step 1: Rust 三架构交叉编译
echo "[1/4] 编译 Rust 核心 (libmetaime_core.so)..."

# 安装 rustup targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android 2>/dev/null || true

export_aarch() {
    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"
    export CARGO_TARGET_ARMV7_LINUX_ANDROID_LINKER="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi34-clang"
    export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android34-clang"
}
export_aarch

cd "$CORE_SRC"
cargo build --release --target aarch64-linux-android
cargo build --release --target armv7-linux-androideabi
cargo build --release --target x86_64-linux-android

# 拷贝到jni目录
cp target/aarch64-linux-android/release/libmetaime_core.so "$JNI_APP/src/main/core/arm64-v8a/" 2>/dev/null || mkdir -p "$JNI_APP/src/main/core/arm64-v8a" && cp target/aarch64-linux-android/release/libmetaime_core.so "$JNI_APP/src/main/core/arm64-v8a/"
cp target/armv7-linux-androideabi/release/libmetaime_core.so    "$JNI_APP/src/main/core/armeabi-v7a/"
cp target/x86_64-linux-android/release/libmetaime_core.so     "$JNI_APP/src/main/core/x86_64/"

echo "  ✓ libmetaime_core.so (arm64-v8a / armeabi-v7a / x86_64)"
echo ""

# Step 2: CMake 构建 jni_bridge.so
echo "[2/4] 构建 jni_bridge.so..."
cd "$JNI_APP/src/main/cpp"
rm -rf build && mkdir build && cd build

cmake .. \
    -DCMAKE_TOOLCHAIN_FILE="$NDK/build/cmake/android.toolchain.cmake" \
    -DANDROID_ABI=arm64-v8a \
    -DANDROID_PLATFORM=android-24

cmake --build .

echo "  ✓ jni_bridge.so"
echo ""

# Step 3: APK 打包
echo "[3/4] 打包 APK..."
cd "$JNI_APP"
./gradlew assemble${MODE^}

APK=$(find app/build/outputs/apk -name "*.apk" | head -1)
echo "  ✓ APK: $APK"
echo ""

# Step 4: 自主签名
echo "[4/4] 签名 APK..."
if [[ ! -f "$KEY_PATH" ]]; then
    echo "  ! 密钥不存在，生成自主签名..."
    mkdir -p "$(dirname "$KEY_PATH")"
    keytool -genkeypair \
        -alias mqf \
        -keyalg RSA -keysize 2048 \
        -validity 3650 \
        -keystore "$KEY_PATH" \
        -storepass mqf-chain \
        -keypass mqf-key \
        -dname "CN=刘楚坦和我, OU=root, O=MQF, C=CN"
fi

SIGNED="${APK%.apk}-signed.apk"
$ANDROID_HOME/build-tools/34.0.0/apksigner sign \
    --ks "$KEY_PATH" \
    --ks-pass pass:mqf-chain \
    --key-pass pass:mqf-key \
    --out "$SIGNED" \
    "$APK"

echo "  ✓ 签名 APK: $SIGNED"
echo ""
echo "=== MetaIME 构建完成 ==="
echo "  APK: $SIGNED"
echo "  安装: adb install $SIGNED"
