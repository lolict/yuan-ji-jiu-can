# 观测系·观测演示
import observer

echo ""
echo "☉ 观测系启动..."
echo ""

let report = collect_presence()

echo "═══ 观测结果 ═══"
echo "  观测时刻: ", report.timestamp
echo "  统计原则: 加法统计法（到场密度，非数值运算）"
echo ""
echo "  总到场符号数: ", $report.total_present
echo ""

# 逐模式报密度
for p in PresencePattern:
  let d = report.densities[p]
  if d > 0:
    echo "  ", pattern_name(p), ": ", $d, " 在场"

echo ""
echo "  采样指纹（前10条）:"
for i in 0..<min(10, report.fingerprints.len):
  let fp = report.fingerprints[i]
  let short_addr = fp.address.replace(getAppDir().parentDir().parentDir(), "")
  echo "    ", fp.symbol, " → ", short_addr, " [", pattern_name(fp.pattern), "]"

echo ""
echo "  ── 观测完成 · 刘楚恬和我 ──"
