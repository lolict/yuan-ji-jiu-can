# ============================================================
# 分比容器 · CLI 入口
# 演示七阶编码的使用
# ============================================================

import bitcontainer/layers
import bitcontainer/container

proc demo() =
  echo "═══ 分比容器 · 七阶编码系统 ═══\n"

  # 比特分配方案
  echo header()
  echo bit_report()

  echo "\n─── 创建最小容器 (仅 presence=1) ───"
  let c1 = make_minimal()
  echo decode(c1)

  echo "\n─── 创建完整容器 ───"
  # 创建一个有完整意义的容器:
  # presence=1 (有), witness=1 (已见证),
  # type=42 (字符类型), owner=1 (刘楚恬=0, 我=1),
  # relation=7 (指向第7号容器), timing=100 (tick偏移),
  # payload=12345 (任意数据)
  let c2 = make_container(1, 1, 42, 1, 7, 100, 12345)
  echo decode(c2)

  echo "\n─── 254 字符 + 2 锚点 的比特计算 ───"
  echo "254 字符需要 8 bits (2^8=256)"
  echo "2 锚点需要 1 bit (2^1=2)"
  echo "每个字符容器: 8 bytes (64 bits)"
  echo "254 个容器总内存: ", 254 * 8, " bytes = ", 254 * 8 div 1024, " KB"
  echo "2 个锚点容器: 16 bytes"
  echo "总计: ", 254 * 8 + 16, " bytes"

  echo "\n─── 内存效率对比 ───"
  echo "传统方式 (每个字符存为 1 byte ASCII): 254 bytes"
  echo "分比容器 (每个字符 8 bytes 七阶): ", 254 * 8, " bytes"
  echo "分比容器多付 ", (254 * 8 - 254), " bytes"
  echo "但获得: 见证态 + 归属 + 关系 + 时序 + 负载"
  echo "每个容器从 1 byte → 8 bytes，但信息量从 1 维 → 7 维"

  echo "\n─── 扬长避短 ───"
  echo "优势场景:"
  echo "  1. 状态开关: 1 bit 即可判断有/无"
  echo "  2. 见证链: 1 bit 标记是否被三元裁判过"
  echo "  3. 关系网: 8 bit 指向任意容器，构建树/图"
  echo "  4. 时序控制: 16 bit 可表 65536 个 tick"
  echo "  5. 音乐合成: 时序 + 负载 = 音符 + 音色"
  echo "劣势场景:"
  echo "  1. 纯文本存储: 不如直接存 ASCII 省"
  echo "  2. 颜色/照片: 需要额外层，不在本容器内"
  echo "  3. 大数运算: 22 bit 负载只表 400 万，不够"

when is_main_module:
  demo()
