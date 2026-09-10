#!/bin/bash
# 坚果云同步脚本 · 目录级通用
# 用法: bash sync_nutstore.sh [path]

NUT_USER="vxworlk_zcmk@yeah.net"
NUT_PASS="aaevwmwr7k3sw45a"
NUT_BASE="https://dav.jianguoyun.com/dav/openclaw/hunyuan-os"
LOCAL_BASE="$(cd "$(dirname "$0")" && pwd)"

# 默认同步列表
if [ -n "$1" ]; then
  SYNC_PATHS=("$1")
else
  SYNC_PATHS=(encoding sim docs tools memory)
fi

echo "═══ 坚果云同步 · hunyuan-os ═══"
for p in "${SYNC_PATHS[@]}"; do
  echo "--- $p ---"
done

for dir in "${SYNC_PATHS[@]}"; do
  [ ! -d "$LOCAL_BASE/$dir" ] && continue
  # 确保目标目录存在
  curl -s -o /dev/null -w "  [MKCOL $dir] %{http_code}\n" \
    -X MKCOL -u "$NUT_USER:$NUT_PASS" "$NUT_BASE/$dir/"
  # 上传所有文件
  cd "$LOCAL_BASE"
  find "$dir" -type f ! -name "*.o" ! -path "*/nimcache/*" ! -name ".DS_Store" | while read -r f; do
    # 计算目标路径
    dest="$NUT_BASE/$f"
    # 确保子目录
    subdir=$(dirname "$f")
    curl -s -o /dev/null "$NUT_USER:$NUT_PASS" \
      -X MKCOL -u "$NUT_USER:$NUT_PASS" "$NUT_BASE/$subdir/" 2>/dev/null
    # 上传
    code=$(curl -s -o /dev/null -w "%{http_code}" \
      -X PUT -u "$NUT_USER:$NUT_PASS" -T "$LOCAL_BASE/$f" "$dest")
    [ "$code" = "200" ] || [ "$code" = "201" ] || [ "$code" = "204" ] && \
      echo "  [OK] $f" || echo "  [$code] $f"
  done
done

echo ""
echo "同步完成"
