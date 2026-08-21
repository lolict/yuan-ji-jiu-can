#!/bin/bash
# 坚果云同步脚本 · 用于日常同步到私有云
# 用法: bash sync_nutstore.sh

NUT_USER="vxworlk_zcmk@yeah.net"
NUT_PASS="aaevwmwr7k3sw45a"
NUT_BASE="https://dav.jianguoyun.com/dav/openclaw/hunyuan-os"
LOCAL_BASE="$(cd "$(dirname "$0")" && pwd)"

echo "═══ 坚果云同步 · hunyuan-os ═══"

# 目录检查
for dir in encoding sim docs tools; do
  curl -s -o /dev/null -w "  [$dir] mkdir: %{http_code}\n" \
    -X MKCOL -u "$NUT_USER:$NUT_PASS" "$NUT_BASE/$dir/"
done

# encoding
echo "--- encoding ---"
for f in encoding/seeds/fullquan_symbol_seeds.nim \
         encoding/shengmu.h encoding/yunmu.h encoding/fullquan_code.h; do
  [ -f "$LOCAL_BASE/$f" ] && \
    curl -s -o /dev/null -w "  $f: %{http_code}\n" \
      -X PUT -u "$NUT_USER:$NUT_PASS" -T "$LOCAL_BASE/$f" \
      "$NUT_BASE/encoding/$(basename $f)"
done

# sim
echo "--- sim ---"
for f in sim/block_addressing.nim sim/clock_layer.nim sim/dual_keyboard.nim \
         sim/life_society_fq.nim; do
  [ -f "$LOCAL_BASE/$f" ] && \
    curl -s -o /dev/null -w "  $f: %{http_code}\n" \
      -X PUT -u "$NUT_USER:$NUT_PASS" -T "$LOCAL_BASE/$f" \
      "$NUT_BASE/sim/$(basename $f)"
done

# tools/kungguan-container 核心
echo "--- tools ---"
for f in tools/kungguan-container/shengmu48.nim \
         tools/kungguan-container/yunmu66.nim \
         tools/kungguan-container/hunyuan_os.nim \
         tools/kungguan-container/yunmu33.nim \
         tools/kungguan-container/tone60.nim \
         tools/kungguan-container/color42.nim \
         tools/kungguan-container/space_phase.nim \
         tools/kungguan-container/mirror_aligner.nim; do
  [ -f "$LOCAL_BASE/$f" ] && \
    curl -s -o /dev/null -w "  $(basename $f): %{http_code}\n" \
      -X PUT -u "$NUT_USER:$NUT_PASS" -T "$LOCAL_BASE/$f" \
      "$NUT_BASE/tools/$(basename $f)"
done

# daily log
echo "--- daily log ---"
LOG_FILE="/mnt/data/catpaw/home/.meituan-catpaw/9199532794/memory/daily/$(date +%Y-%m-%d)-encoding-sim.md"
if [ -f "$LOG_FILE" ]; then
  curl -s -o /dev/null -w "  log: %{http_code}\n" \
    -X PUT -u "$NUT_USER:$NUT_PASS" -T "$LOG_FILE" \
    "$NUT_BASE/docs/daily-encoding-sim.md"
fi

echo ""
echo "同步完成"
