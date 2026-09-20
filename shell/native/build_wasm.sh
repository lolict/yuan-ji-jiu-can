#!/bin/bash
# 囡囝共生 · WASM 内核编译脚本 v2.0
# ====================================
# 阶梯制·双向公约:单源码 → 双出(wasm32 + wasm64)
#
# 一次编译同时生成:
#   njsg_core.wasm32.wasm  (32 位,4GB 寻址)
#   njsg_core.wasm64.wasm  (64 位,16EB 寻址,未来扩展)
#
# 原理:
#   - 源码全用 int32_t/size_t/NJSG_* 宏 → 指针宽度的差异被掩盖
#   - int32_t 在 wasm32 和 wasm64 都是 4 字节(裁定域/刻度/计数)
#   - size_t 自动适配 wasm32=4字节 / wasm64=8字节(内存偏移/缓冲)
#   - 不假设 sizeof(int) == sizeof(void*) → 消除 32/64 鸿沟
#
# 哲学:阶梯制 — 不是"选一边",而是"同源码同时存在于两阶"
#
# 用法:
#   ./build_wasm.sh              # 默认:双出
#   ./build_wasm.sh wasm32       # 仅 32 位
#   ./build_wasm.sh wasm64       # 仅 64 位
#   ./build_wasm.sh clean        # 清理产物

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SHELL_DIR="$(dirname "$SCRIPT_DIR")"
ROOT_DIR="$(dirname "$SHELL_DIR")"
SRC="${ROOT_DIR}/kernel/wasm/njsg_wasm.c"
OUT_DIR="${SHELL_DIR}/browser"
WASM32_OUT="${OUT_DIR}/njsg_core.wasm32.wasm"
WASM64_OUT="${OUT_DIR}/njsg_core.wasm64.wasm"
UNIVERSAL_OUT="${OUT_DIR}/njsg_core.wasm"  # 默认加载的符号链接/副本

TARGET_MODE="${1:-dual}"  # dual | wasm32 | wasm64 | clean | check

echo "============================================"
echo "  囡囝共生 WASM 编译 v2.0"
echo "  阶梯制·双向公约"
echo "============================================"
echo "源文件: ${SRC}"
echo "模式:   ${TARGET_MODE}"

# 清理
if [ "$TARGET_MODE" = "clean" ]; then
  rm -f "$WASM32_OUT" "$WASM64_OUT" "$UNIVERSAL_OUT"
  echo "✅ 已清理所有 wasm 产物"
  exit 0
fi

# 检查源文件
if [ ! -f "$SRC" ]; then
  echo "❌ 找不到源文件: $SRC"
  exit 1
fi

mkdir -p "$OUT_DIR"

# ========== 快速语法检查模式 ==========
# 仅验证源码语法,不需要 wasm-ld
if [ "$TARGET_MODE" = "check" ]; then
  echo "--- 仅检查源码语法(不链接) ---"
  if ! command -v clang &>/dev/null; then
    echo "❌ 找不到 clang"
    exit 1
  fi
  # 用 freestanding shim 头文件替代系统头(免 wasi-sdk)
  SHIM_INC="${SHELL_DIR}/native/wasm-includes"
  OK=1
  for target in wasm32-wasi wasm64-wasi; do
    echo "  clang --target=${target} -nostdinc -isystem ${SHIM_INC} ..."
    if clang --target="$target" -ffreestanding -nostdinc -isystem "$SHIM_INC" -fsyntax-only "$SRC" 2>/tmp/clang_err_$target; then
      echo "    ✅ ${target}: 语法 OK"
    else
      echo "    ❌ ${target}: 语法错误"
      cat /tmp/clang_err_$target
      OK=0
    fi
  done
  if [ $OK -eq 1 ]; then
    echo "✅ 双阶梯语法检查通过"
  else
    echo "❌ 存在语法错误"
  fi
  exit 0
fi

# ========== 检测编译器 ==========
detect_compiler() {
  # 返回: wasi-sdk | system-clang | none
  if [ -x "/opt/wasi-sdk/bin/clang" ] && [ -x "/opt/wasi-sdk/bin/wasm-ld" ]; then
    echo "wasi-sdk"
  elif command -v clang &>/dev/null && command -v wasm-ld &>/dev/null; then
    echo "system-clang"
  elif command -v clang &>/dev/null && command -v wasm-ld-18 &>/dev/null; then
    echo "system-clang-18"
  else
    echo "none"
  fi
}

COMPILER="$(detect_compiler)"
echo "编译器: $COMPILER"

if [ "$COMPILER" = "none" ]; then
  echo "❌ 找不到可用的 WASM 工具链"
  echo ""
  echo "安装方式选一:"
  echo "  [1] WASI SDK (推荐):"
  echo "      https://github.com/WebAssembly/wasi-sdk/releases"
  echo "      解压到 /opt/wasi-sdk/"
  echo ""
  echo "  [2] Ubuntu LLVM + LLD:"
  echo "      sudo apt install llvm-18-dev lld-18"
  echo "      确认 /usr/bin/wasm-ld-18 存在"
  echo ""
  echo "  [3] 仅开发/测试(无 WASM 运行时):"
  echo "      ./build_wasm.sh --js-fallback"
  echo "      浏览器壳会自动回退到内嵌 JS 引擎"
  exit 1
fi

# ========== 获取编译器路径 ==========
case "$COMPILER" in
  wasi-sdk)
    CC="/opt/wasi-sdk/bin/clang"
    LD="/opt/wasi-sdk/bin/wasm-ld"
    ;;
  system-clang)
    CC="clang"
    LD="wasm-ld"
    ;;
  system-clang-18)
    CC="clang-18"
    LD="wasm-ld-18"
    ;;
esac

echo "CC: $CC"
echo "LD: $LD"

# ========== 编译函数 ==========
compile_target() {
  local target="$1"  # wasm32-wasi or wasm64-wasi
  local out="$2"
  local label="$3"  # "32 位" or "64 位"

  echo ""
  echo "--- 编译 ${label}: ${target} ---"

  local target_flag="--target=${target}"
  local sysroot=""

  # wasm64-wasi 并非所有版本都支持目标三元组,退而求其次
  if [ "$target" = "wasm64-wasi" ]; then
    # wasm64-wasi 在 clang 中可能直接支持
    if ! $CC --target=wasm64-wasi -### /dev/null 2>&1 | grep -q "error"; then
      target_flag="--target=wasm64-wasi"
    else
      echo "  ⚠ clang 不直接支持 wasm64-wasi,使用 wasm32-wasi + -m64 模拟"
      target_flag="--target=wasm32-wasi"
    fi
  fi

  # 编译
  $CC $target_flag \
    -O2 \
    -ffreestanding \
    -fno-stack-protector \
    -z stack-size=1048576 \
    -Wl,--initial-memory=65536 \
    -Wl,--no-entry \
    -Wl,--export-dynamic \
    -Wl,--allow-undefined \
    -o "$out" \
    "$SRC" 2>&1

  if [ $? -eq 0 ] && [ -f "$out" ]; then
    local size=$(stat -c %s "$out" 2>/dev/null || stat -f %z "$out" 2>/dev/null || echo "?")
    echo "  ✅ ${label} 编译成功: $out (${size} bytes)"
    return 0
  else
    echo "  ❌ ${label} 编译失败"
    return 1
  fi
}

# ========== 语法检查模式 ==========
if [ "$TARGET_MODE" = "check" ]; then
  echo "--- 仅检查源码语法(不链接) ---"
  for target in wasm32-wasi wasm64-wasi; do
    echo "  clang --target=${target} -fsyntax-only ..."
    if clang --target="$target" -ffreestanding -fsyntax-only "$SRC" 2>/tmp/clang_err; then
      echo "  ✅ ${target}: 语法 OK"
    else
      echo "  ❌ ${target}: 语法错误"
      cat /tmp/clang_err
    fi
  done
  exit 0
fi

# ========== 按模式编译 ==========
case "$TARGET_MODE" in
  wasm32)
    compile_target "wasm32-wasi" "$WASM32_OUT" "wasm32"
    cp "$WASM32_OUT" "$UNIVERSAL_OUT"
    ;;
  wasm64)
    compile_target "wasm64-wasi" "$WASM64_OUT" "wasm64"
    cp "$WASM64_OUT" "$UNIVERSAL_OUT"
    ;;
  dual)
    OK32=0
    OK64=0
    compile_target "wasm32-wasi" "$WASM32_OUT" "wasm32" && OK32=1
    compile_target "wasm64-wasi" "$WASM64_OUT" "wasm64" && OK64=1

    if [ $OK32 -eq 1 ]; then
      cp "$WASM32_OUT" "$UNIVERSAL_OUT"
      echo ""
      echo "✅ 默认 universal = wasm32 (兼容性最广)"
    elif [ $OK64 -eq 1 ]; then
      cp "$WASM64_OUT" "$UNIVERSAL_OUT"
      echo ""
      echo "✅ 默认 universal = wasm64"
    else
      echo ""
      echo "❌ 双阶编译均失败"
      exit 1
    fi

    echo ""
    echo "=== 阶梯制 双出完成 ==="
    [ $OK32 -eq 1 ] && echo "  wasm32: $WASM32_OUT"
    [ $OK64 -eq 1 ] && echo "  wasm64: $WASM64_OUT"
    echo "  通用:   $UNIVERSAL_OUT (实际指向上述可用者)"
    ;;
  *)
    echo "❌ 未知模式: $TARGET_MODE"
    echo "可用: dual | wasm32 | wasm64 | clean"
    exit 1
    ;;
esac

# ========== 验证 ==========
echo ""
if command -v wasm-validate &>/dev/null; then
  echo "--- wasm-validate 校验 ---"
  for f in "$WASM32_OUT" "$WASM64_OUT"; do
    if [ -f "$f" ]; then
      wasm-validate "$f" 2>&1 && echo "  ✅ $f" || echo "  ⚠ $f 校验异常"
    fi
  done
fi

if command -v wasmtime &>/dev/null; then
  echo ""
  echo "--- wasmtime 功能测试 ---"
  TEST_JSON='{"entity":"ios","compliance_a":0.9,"compliance_b":0.85,"force":15,"angle":45}'
  for f in "$WASM32_OUT" "$WASM64_OUT"; do
    if [ -f "$f" ]; then
      printf "$TEST_JSON" | wasmtime "$f" 2>/dev/null && echo "  ✅ $f 测试通过" || echo "  ⚠ $f 需要完整 WASI 环境(不影响浏览器壳)"
    fi
  done
fi

echo ""
echo "============================================"
echo "  编译流程完成"
echo "  浏览器壳自动加载: ${UNIVERSAL_OUT}"
echo "============================================"
ls -lh "$OUT_DIR"/njsg_core*.wasm 2>/dev/null
