#!/bin/bash
# 民联浏览器壳 · 移动端构建脚本 v1.10
# ======================================
# 把 shell/browser/ 内容打包到 Android / iOS 项目
#
# 用法:
#   ./build_mobile.sh          # 复制 assets
#   ./build_mobile.sh --apk    # 构建 APK (需要 Android SDK)
#   ./build_mobile.sh --ios    # 构建 iOS (需要 Xcode)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
BROWSER_DIR="${PROJECT_DIR}/browser"
ANDROID_ASSETS="${SCRIPT_DIR}/android/app/src/main/assets/mqf_shell"
IOS_RESOURCES="${SCRIPT_DIR}/ios/MqfShell/mqf_shell"

echo "=== 民联浏览器壳 v1.10 移动端构建 ==="
echo "源: ${BROWSER_DIR}"

# ─── 1. 复制到 Android assets ──────────────────────────────────
echo ">> 复制到 Android assets/"
rm -rf "$ANDROID_ASSETS"
mkdir -p "$ANDROID_ASSETS"
cp "${BROWSER_DIR}/index.html" "$ANDROID_ASSETS/"
cp "${BROWSER_DIR}/mqf-shell.js" "$ANDROID_ASSETS/"
cp "${BROWSER_DIR}/shell.css" "$ANDROID_ASSETS/"
cp "${SCRIPT_DIR}/mobile-bridge.js" "$ANDROID_ASSETS/"

# 如果存在 mqf_core.wasm,也复制过去
if [ -f "${BROWSER_DIR}/mqf_core.wasm" ]; then
  cp "${BROWSER_DIR}/mqf_core.wasm" "$ANDROID_ASSETS/"
  echo "  + mqf_core.wasm 已复制"
fi

echo "✅ Android assets: ${ANDROID_ASSETS}"
ls -lh "$ANDROID_ASSETS"

# ─── 2. 复制到 iOS Bundle 资源 ──────────────────────────────
echo ">> 复制到 iOS Bundle Resources/"
rm -rf "$IOS_RESOURCES"
mkdir -p "$IOS_RESOURCES"
cp "${BROWSER_DIR}/index.html" "$IOS_RESOURCES/"
cp "${BROWSER_DIR}/mqf-shell.js" "$IOS_RESOURCES/"
cp "${BROWSER_DIR}/shell.css" "$IOS_RESOURCES/"
cp "${SCRIPT_DIR}/mobile-bridge.js" "$IOS_RESOURCES/"

if [ -f "${BROWSER_DIR}/mqf_core.wasm" ]; then
  cp "${BROWSER_DIR}/mqf_core.wasm" "$IOS_RESOURCES/"
  echo "  + mqf_core.wasm 已复制"
fi

echo "✅ iOS Resources: ${IOS_RESOURCES}"
ls -lh "$IOS_RESOURCES"

# ─── 3. 获取平台信息注入 ────────────────────────────────────
BROWSER_VERSION=$(grep 'MQF_SHELL_VERSION' "${BROWSER_DIR}/mqf-shell.js" | head -1 | sed 's/.*= *["'"'"'"]\([^"'"'"'"]*\).*/\1/')
echo ""
echo "=== 构建完成 ==="
echo "浏览器壳版本: ${BROWSER_VERSION}"
echo ""
echo "Android 集成:"
echo "  cd android && ./gradlew assembleDebug"
echo "  输出: android/app/build/outputs/apk/debug/app-debug.apk"
echo ""
echo "iOS 集成:"
echo "  打开 ios/MqfShell.xcodeproj 在 Xcode 中"
echo "  Product → Archive → Distribute App"
echo ""
echo "均支持离线运行,无需任何网络请求。"
