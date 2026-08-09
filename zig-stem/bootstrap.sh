#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════════════
# bootstrap.sh — 在本地安装 Zig (稳定版 0.13+) 并编译鸿蒙双 wasm
# ══════════════════════════════════════════════════════════════════
#
# 用法:
#   cd hunyuan-os/zig-stem
#   ./bootstrap.sh           → 自动下载 zig + 产出双 wasm
#   ./bootstrap.sh --local   → 使用已有的 $ZIG 环境变量
#
# 产出:
#   zig-out/wasm32/release/mqf.wasm
#   zig-out/wasm64/release/mqf.wasm
#
# 注意: Zig 二进制放在 zig-stem/.zig/ 下,不污染系统目录。
#       用户可自行携带 .zig/ 目录跨机器迁移 — 自寄生策略的一部分。

set -euo pipefail

ZIG_DIR="$(cd "$(dirname "$0")" && pwd)/.zig"
ZIG_BIN="$ZIG_DIR/bin/zig"

# ── 选择版本 (0.13.0 稳定版) ──
ZIG_VERSION="0.13.0"
case "$(uname -m)" in
    x86_64)  ZIG_ARCH="x86_64" ;;
    aarch64) ZIG_ARCH="aarch64" ;;
    *) echo "不支持的架构 $(uname -m)"; exit 1 ;;
esac
case "$(uname -s)" in
    Linux)  ZIG_OS="linux" ;;
    Darwin) ZIG_OS="macos" ;;
    *) echo "不支持的系统 $(uname -s)"; exit 1 ;;
esac
ZIG_TARBALL="zig-${ZIG_OS}-${ZIG_ARCH}-${ZIG_VERSION}.tar.xz"
ZIG_URL="https://ziglang.org/download/${ZIG_VERSION}/${ZIG_TARBALL}"

# ── Step 1: 若无 zig 则下载 ──
if [[ ! -x "$ZIG_BIN" ]]; then
    echo "[zig-stem] 下载 zig ${ZIG_VERSION} (${ZIG_OS}-${ZIG_ARCH})..."
    mkdir -p "$ZIG_DIR"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$ZIG_URL" -o "/tmp/$ZIG_TARBALL"
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

# ── Step 2: 编译 ──
export PATH="$ZIG_DIR/bin:$PATH"
echo "[zig-stem] zig 版本: $(zig version)"
echo "[zig-stem] 编译双 wasm (wasm32 + wasm64)..."
zig build all -Dtarget=wasm32-freestanding 2>&1 || true  # 降级:32位产出
zig build wasm64 2>&1 || echo "[zig-stem] wasm64 在 0.13 仍为实验,可跳过"

# ── Step 3: 验证 ──
echo "[zig-stem] 检查产出:"
find zig-out -name "*.wasm" -exec ls -la {} \; 2>/dev/null

WASM32=$(find zig-out -name "mqf.wasm" -path "*wasm32*" 2>/dev/null | head -1)
WASM64=$(find zig-out -name "mqf.wasm" -path "*wasm64*" 2>/dev/null | head -1)

echo ""
echo "════════════════════════════════════════"
echo "  wasm32: ${WASM32:-未生成}"
echo "  wasm64: ${WASM64:-未生成}"
echo "════════════════════════════════════════"

if [[ -n "$WASM32" ]]; then
    echo "[zig-stem] ✓ 阶梯制鸿约 wasm32 产出成功"
fi
