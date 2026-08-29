#!/bin/bash
# MetaIME FFI 构建脚本
# 编译三语言实现并链接 FFI 测试程序

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
METAIME_DIR="$(dirname "$SCRIPT_DIR")"

echo "============================================================"
echo "MetaIME FFI Bridge · 构建脚本"
echo "============================================================"

# 确保编译器在 PATH 中
export PATH="$HOME/.local/bin:$HOME/zig/zig-linux-x86_64-0.13.0:$HOME/.nimble/bin:$HOME/.cargo/bin:$PATH"

cd "$SCRIPT_DIR"

# === 1. 编译 Zig FFI 桥 ===
echo ""
echo "[1/4] 编译 Zig FFI 桥..."
cd "$METAIME_DIR/zig"
zig build-lib "$SCRIPT_DIR/metaime_zig_bridge.zig" \
    -static \
    -OReleaseFast \
    -lc \
    -femit-bin="$SCRIPT_DIR/libmetaime_zig_bridge.a"
echo "  → libmetaime_zig_bridge.a"

# === 2. 编译 Rust FFI 桥 ===
echo ""
echo "[2/4] 编译 Rust FFI 桥..."
cd "$METAIME_DIR/rust"
cargo build --release --lib --manifest-path Cargo.toml 2>&1 | tail -3
# 复制静态库
cp "$METAIME_DIR/rust/target/release/libmetaime_rust_bridge.a" "$SCRIPT_DIR/" 2>/dev/null || \
cp "$METAIME_DIR/rust/target/release/libmetaime.a" "$SCRIPT_DIR/libmetaime_rust_bridge.a" 2>/dev/null || \
echo "  (Rust 静态库位置需手动确认)"
echo "  → libmetaime_rust_bridge.a"

# === 3. 编译 Nim FFI 桥 ===
echo ""
echo "[3/4] 编译 Nim FFI 桥..."
cd "$METAIME_DIR/nim"
nim c --app:lib --staticLib --noMain \
    -o:"$SCRIPT_DIR/libmetaime_nim_bridge.a" \
    "$SCRIPT_DIR/metaime_nim_bridge.nim" 2>&1 | tail -5
echo "  → libmetaime_nim_bridge.a"

# === 4. 编译 C 测试程序 ===
echo ""
echo "[4/4] 编译 FFI 测试程序..."
cd "$SCRIPT_DIR"
gcc -o test_ffi test_ffi.c \
    -I"$SCRIPT_DIR" \
    -L"$SCRIPT_DIR" \
    -lmetaime_zig_bridge \
    -lmetaime_rust_bridge \
    -lmetaime_nim_bridge \
    -lm \
    -lpthread
echo "  → test_ffi"

echo ""
echo "============================================================"
echo "构建完成！运行测试: $SCRIPT_DIR/test_ffi"
echo "============================================================"
