# ============================================================
# 蚕蛹反向寄生壳 · 内核
# 不随外壳变化而变化的核心逻辑
# ============================================================


# 内核状态 - 与外壳无关
type
  KernelState* = object
    version*: string
    anchors*: array[2, string]    # [0]=刘楚恬, [1]=我
    active_host*: string          # 当前寄生的外壳名称
    mounted*: bool
    tick*: int
    merit*: float
    demerit*: float

  Kernel* = ref KernelState

# 创建内核
func new_kernel*(): Kernel =
  result = Kernel(
    version: "0.1.0",
    anchors: ["刘楚恬", "lolict"],
    active_host: "(未寄生)",
    mounted: false,
    tick: 0,
    merit: 0.0,
    demerit: 0.0
  )

# 内核自检 (不依赖外壳)
func kernel_selfcheck*(k: Kernel): string =
  var s = "═══ 内核自检 ═══\n"
  s &= "版本: " & k.version & "\n"
  s &= "锚点: [" & k.anchors[0] & ", " & k.anchors[1] & "]\n"
  s &= "外壳: " & k.active_host & "\n"
  s &= "挂载: " & (if k.mounted: "是" else: "否") & "\n"
  s &= "Tick: " & $k.tick & "\n"
  s &= "功德: " & $k.merit & "\n"
  s &= "业报: " & $k.demerit & "\n"
  s &= "净余额: " & $(k.merit - k.demerit) & "\n"
  result = s

# 内核脉冲 (tick 推进)
func kernel_tick*(k: Kernel) =
  k.tick += 1
  # 每个 tick 自然产生微量功德
  k.merit += 0.1

# 内核核心判断 - 无论寄生到哪都是这个逻辑
func kernel_judge*(k: Kernel, input_code: uint64): string =
  # 提取 anchor bit (bit 10)
  let a = (input_code shr 10) and 1
  # 提取 witness bit (bit 1)
  let w = (input_code shr 1) and 1
  # 提取 presence bit (bit 0)
  let p = input_code and 1

  if p == 0:
    k.demerit += 0.5
    return "空袋(拒)"
  if w == 0:
    return "待定"
  if a == 1:
    k.merit += 1.0
    return "准入(我)"
  else:
    k.merit += 1.0
    return "准入(刘楚恬)"

# 内核报告 - 无论外壳怎么看，内核如实报告
func kernel_report*(k: Kernel): string =
  var s = "蚕蛹内核报告\n"
  s &= "────────────────────\n"
  s &= "当前寄生: " & k.active_host & "\n"
  s &= "已运行: " & $k.tick & " ticks\n"
  s &= "累计功德: " & $k.merit & "\n"
  s &= "累计业报: " & $k.demerit & "\n"
  let bal = k.merit - k.demerit
  s &= "净余额: " & $bal & " "
  if bal > 0: s &= "(功德占优)\n"
  elif bal < 0: s &= "(业报占优)\n"
  else: s &= "(平衡)\n"
  result = s
