#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════════════
# bootstrap.sh — 在本地安装 Zig + 编译双 wasm
# ══════════════════════════════════════════════════════════════════
#
# 用法:
#   cd hunyuan-os/zig-stem
#   ./bootstrap.sh            → 自动下载 zig + 产出双 wasm
#   ZIG=/path/to/zig ./bootstrap.sh  → 使用已有 zig
#
# 产出:
#   zig-out/wasm32/release/mqf32.wasm
#   zig-out/wasm64/release/mqf64.wasm

set -euo pipefail

ZIG_DIR="$(cd "$(dirname "$0")" && pwd)/.zig"
ZIG_BIN="$ZIG_DIR/bin/zig"

# ── Step 1: 若无 zig 则下载 (zig 0.13.0) ──
if [[ ! -x "$ZIG_BIN" ]]; then
    ZIG_VERSION="0.13.0"
    case "$(uname -m)" in
        x86_64)  ZIG_ARCH="x86_64" ;;
        aarch64) ZIG_ARCH="aarch64" ;;
        *) echo "[zig-stem] 不支持的架构 $(uname -m)"; exit 1 ;;
    esac
    case "$(uname -s)" in
        Linux)  ZIG_OS="linux" ;;
        Darwin) ZIG_OS="macos" ;;
        *) echo "[zig-stem] 不支持的系统 $(uname -s)"; exit 1 ;;
    esac
    ZIG_TARBALL="zig-${ZIG_OS}-${ZIG_ARCH}-${ZIG_VERSION}.tar.xz"
    ZIG_URL="https://ziglang.org/download/${ZIG_VERSION}/${ZIG_TARBALL}"

    echo "[zig-stem] 下载 zig ${ZIG_VERSION}..."
    mkdir -p "$ZIG_DIR"
    if command -v curl >/dev/null 2>&1; then
        curl -fL "$ZIG_URL" -o "/tmp/$ZIG_TARBALL"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$ZIG_URL" -O "/tmp/$ZIG_TARBALL"
    else
        echo "[zig-stem] 需要 curl 或 wget"; exit 1
    fi
    tar -xf "/tmp/$ZIG_TARBALL" -C "$ZIG_DIR" --strip-components=1
    rm -f "/tmp/$ZIG_TARBALL"
    echo "[zig-stem] zig 安装到 $ZIG_DIR"
else
    echo "[zig-stem] 使用现有 zig: $ZIG_BIN"
fi

# ── Step 2: 用 build.sh 编译 ──
export PATH="$ZIG_DIR/bin:$PATH"
echo "[zig-stem] zig 版本: $(zig version)"
bash "$(dirname "$0")/build.sh"

# ── Step 3: 验证 ──
echo ""
echo "════════════════════════════════════════"
WASM32="zig-out/wasm32/release/mqf32.wasm"
WASM64="zig-out/wasm64/release/mqf64.wasm"
ls -lh "$WASM32" "$WASM64" 2>/dev/null
echo ""
if [[ -f "$WASM32" && -f "$WASM64" ]]; then
    echo "[zig-stem] ✓ 阶梯制·双向鸿约产出完成"
    echo "  $(file "$WASM32" 2>/dev/null | cut -d: -f2)"
    echo "  $(file "$WASM64" 2>/dev/null | cut -d: -f2)"
else
    echo "[zig-stem] ✗ 产出不完整,请检查 build.sh"
    exit 1
fi
echo "════════════════════════════════════════"
