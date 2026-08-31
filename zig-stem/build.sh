#!/bin/bash
# build.sh — 纯 zig cc 命令行,绕过 build 系统 API
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ZIG="${ZIG:-$SCRIPT_DIR/.zig/zig}"
CC="$ZIG cc"

echo "[1] wasm32 → zig-out/wasm32/release/mqf32.wasm"
mkdir -p zig-out/wasm32/release
$CC -target wasm32-freestanding -Oz -o zig-out/wasm32/release/mqf32.wasm main.c \
    -Wl,--export=mqf_boot \
    -Wl,--export=mqf_symbol_angle \
    -Wl,--export=mqf_arc_distance \
    -Wl,--export=mqf_time_stage_depth \
    -Wl,--export=mqf_is_adjacent \
    -Wl,--no-entry

echo "[2] wasm64 → zig-out/wasm64/release/mqf64.wasm"
mkdir -p zig-out/wasm64/release
$CC -target wasm64-freestanding -Oz -o zig-out/wasm64/release/mqf64.wasm main.c \
    -Wl,--export=mqf_boot \
    -Wl,--export=mqf_symbol_angle \
    -Wl,--export=mqf_arc_distance \
    -Wl,--export=mqf_time_stage_depth \
    -Wl,--export=mqf_is_adjacent \
    -Wl,--no-entry

echo "[3] 产物:"
find zig-out -name "*.wasm" -exec ls -lh {} \;
echo "[4] 双 wasm 鸿约完成 ✓"
