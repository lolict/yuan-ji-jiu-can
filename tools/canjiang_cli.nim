# ============================================================
# 蚕蛹反向寄生壳 · CLI 演示
# 脱壳 → 选壳 → 穿衣 → 反向寄生
# ============================================================

import canjiang/kernel
import canjiang/shell/registry
import canjiang/parasite

proc demo() =
  echo "═══ 蚕蛹反向寄生壳 ═══\n"

  # 初始状态: 内核未寄生
  var p = new_parasite()
  echo kernel_selfcheck(p.kernel)
  echo ""

  # 列出可选外壳
  echo list_shells()

  # 选壳: 选择 web-shell (索引 0)
  echo "─── 选择外壳 ───"
  echo choose_shell(p, 0)
  echo ""

  # 脱壳穿衣
  echo "─── 脱壳穿衣 ───"
  echo dress(p)
  echo ""

  # 穿衣后内核状态
  echo "─── 穿衣后内核 ───"
  echo kernel_selfcheck(p.kernel)
  echo ""

  # 双视角对比
  echo view_from_kernel(p)
  echo ""
  echo view_from_shell(p)
  echo ""

  # 内核脉冲 (模拟运行)
  echo "─── 运行 5 个 tick ───"
  for i in 1..5:
    kernel_tick(p.kernel)
  echo "运行后: tick=" & $p.kernel.tick & " 功德=" & $p.kernel.merit & "\n"

  # 模拟输入验证
  echo "─── 内核判断测试 ───"
  # 有效输入: presence=1, witness=1, anchor=1 → 准入(我)
  let input1 = 0b00000000000000000000010000000011'u64  # bit0=1, bit1=1, bit10=1
  echo "输入 A (presence=1, witness=1, anchor=1): ", kernel_judge(p.kernel, input1)
  # 无效输入: presence=0
  let input2 = 0b00000000000000000000000000000000'u64
  echo "输入 B (presence=0): ", kernel_judge(p.kernel, input2)
  echo ""

  # 最终报告
  echo kernel_report(p.kernel)

  echo "\n─── 核心思想 ───"
  echo "外部系统以为自己在使用 web-server"
  echo "实际上它在运行蚕蛹内核"
  echo "外壳被反向规则约束，表现出内核的行为"
  echo "这就是: 让外部效仿自身，而非自身适应外部"

when is_main_module:
  demo()
