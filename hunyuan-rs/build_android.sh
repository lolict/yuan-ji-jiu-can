#!/bin/bash
# 元输入法 MetaIME · Android 交叉编译脚本
#
# 用法:
#   ./build_android.sh          # 编译 ARM64 静态库
#   ./build_android.sh --all    # 编译 ARM64 + ARMv7 + x86_64
#
# 产物:
#   target/aarch64-linux-android/release/libmqf.a
#   target/armv7-linux-androideabi/release/libmqf.a
#   target/x86_64-linux-android/release/libmqf.a

set -e

# 工具链路径 (按个人环境修改)
ARM_GCC="/tmp/arm-gnu-toolchain-13.3.rel1-x86_64-aarch64-none-linux-gnu/bin"
NDK="${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk}"

# 添加 rustup 到 PATH
export PATH="$HOME/.cargo/bin:$PATH"

# 添加 arm64 工具链到 PATH
export PATH="$ARM_GCC:$PATH"

# 设置交叉编译环境变量
export CC_aarch64_unknown_linux_gnu="$ARM_GCC/aarch64-none-linux-gnu-gcc"
export AR_aarch64_unknown_linux_gnu="$ARM_GCC/aarch64-none-linux-gnu-ar"

# 确保目标已安装
rustup target add aarch64-unknown-linux-gnu 2>/dev/null || true

echo "=== 编译 ARM64 (aarch64-unknown-linux-gnu) ==="
cargo build --lib --target aarch64-unknown-linux-gnu --release

# 将 rlib 转为静态库 (用于 Android NDK 链接)
AARCH64_RLIB="../../target/aarch64-unknown-linux-gnu/release/libmqf.rlib"
AARCH64_A="../../target/aarch64-unknown-linux-gnu/release/libmqf.a"

if [ -f "$AARCH64_RLIB" ]; then
    # 用 arm ar 从 rlib 提取目标文件并打包为 .a
    cd "$(dirname "$AARCH64_RLIB")"
    tmpdir=$(mktemp -d)
    cd "$tmpdir"
    "$ARM_GCC/aarch64-none-linux-gnu-ar" x "$(dirname "$AARCH64_RLIB")/libmqf.rlib"
    "$ARM_GCC/aarch64-none-linux-gnu-ar" rcs "$AARCH64_A" *.o
    rm -rf "$tmpdir"
    echo "✅ ARM64 静态库: $AARCH64_A"
    ls -lh "$AARCH64_A"
    cd /mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/hunyuan-rs
fi

if [ "$1" = "--all" ]; then
    echo ""
    echo "=== 编译 ARMv7 ==="
    rustup target add armv7-unknown-linux-gnueabihf 2>/dev/null || true
    export CC_armv7_unknown_linux_gnueabihf=arm-none-linux-gnueabihf-gcc
    export AR_armv7_unknown_linux_gnueabihf=arm-none-linux-gnueabihf-ar
    cargo build --lib --target armv7-unknown-linux-gnueabihf --release 2>&1 || echo "⏭️  ARMv7 工具链未配置,跳过"

    echo ""
    echo "=== 编译 x86_64 ==="
    rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true
    cargo build --lib --target x86_64-unknown-linux-gnu --release
fi

echo ""
echo "=== 完成 ==="
