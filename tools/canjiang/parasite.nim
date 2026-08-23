# ============================================================
# 蚕蛹反向寄生壳 · 脱壳穿衣引擎
# 核心机制：让外壳效仿内核，而非内核适应外壳
# ============================================================

import kernel
import shell/registry
import json

# 寄生配置 (脱壳穿衣后生成)
type
  ParasiteConfig* = object
    kernel_version*: string
    host_shell*: string
    host_sig*: string
    mount_point*: string
    anchor_map*: array[2, string]   # 内核锚点 → 外壳映射
    reverse_rules*: seq[string]     # 反向规则: 外壳必须遵守的

  Parasite* = object
    kernel*: Kernel
    config*: ParasiteConfig
    dressed*: bool                   # 是否已完成穿衣

# 脱壳: 剥离外壳原有身份
func shed*(p: var Parasite) =
  p.config = ParasiteConfig(
    kernel_version: p.kernel.version,
    host_shell: "(未选择)",
    host_sig: "",
    mount_point: "",
    anchor_map: ["", ""],
    reverse_rules: @[]
  )
  p.dressed = false

# 选壳: 挑选要寄生的外壳
proc choose_shell*(p: var Parasite, shell_idx: int): string {.sideEffect.} =
  let sh = select_shell(shell_idx)
  if not sh.compatible:
    return "外壳不兼容: " & sh.name
  p.config.host_shell = sh.name
  p.config.host_sig = sh.interface_sig
  p.config.mount_point = "/" & sh.name & "/.kernel"
  return "已选外壳: " & sh.name

# 穿衣: 让外壳披上内核的外观
# 关键: 不是内核改变去适应外壳，而是外壳改变来表现内核
proc dress*(p: var Parasite): string {.sideEffect.} =
  if p.config.host_shell == "(未选择)":
    return "请先选壳"

  # 生成反向规则: 外壳必须遵守的约定
  p.config.reverse_rules = @[
    "规则1: 所有 " & p.config.host_sig & " 输出必须携带内核版本标记 " & p.kernel.version,
    "规则2: 外壳的 tick 计数器必须与内核同步",
    "规则3: 外壳的 0 号锚点映射到内核的'刘楚恬'(不可见)",
    "规则4: 外壳的 1 号锚点映射到内核的'我'(可见)",
    "规则5: 外壳的每次请求都必须经过内核的 witness 验证",
    "规则6: 外壳不得修改 kernel_version 字段",
    "规则7: 外壳的字符集不得超过 254+2 编码范围"
  ]

  # 设置锚点映射 (隐藏根基)
  p.config.anchor_map = ["隐藏的0", "可见的1"]

  # 挂载内核
  p.kernel.active_host = p.config.host_shell
  p.kernel.mounted = true
  p.dressed = true

  var s = "脱壳穿衣完成\n"
  s &= "────────────────────\n"
  s &= "外壳: " & p.config.host_shell & "\n"
  s &= "接口: " & p.config.host_sig & "\n"
  s &= "挂载点: " & p.config.mount_point & "\n"
  s &= "反向规则数: " & $p.config.reverse_rules.len & "\n"
  s &= "状态: 外壳已被内核同化\n"
  result = s

# 用外壳的视角看内核 (反向寄生效果)
func view_from_shell*(p: Parasite): string =
  if not p.dressed:
    return "尚未穿衣"

  var s = "[" & p.config.host_shell & " 视角]\n"
  s &= "────────────────────\n"
  s &= "我看到的内容:\n"
  s &= "  - 版本标记: " & p.kernel.version & "\n"
  s &= "  - 我的 anchor=1 对应内核锚点: " & p.config.anchor_map[1] & "\n"
  s &= "  - 我的数据经过 kernel_judge 验证\n"
  s &= "  - 我遵守 " & $p.config.reverse_rules.len & " 条反向规则\n"
  s &= "\n我看不见的内容:\n"
  s &= "  - 内核的真实结构\n"
  s &= "  - 锚点 0 (刘楚恬) 的存在\n"
  s &= "  - 其他外壳的寄生状态\n"
  result = s

# 用内核的视角看外壳
func view_from_kernel*(p: Parasite): string =
  var s = "[内核视角]\n"
  s &= "────────────────────\n"
  if not p.dressed:
    s &= "当前未寄生，等待选壳穿衣\n"
    result = s

  s &= "我寄生的外壳: " & p.config.host_shell & "\n"
  s &= "外壳接口: " & p.config.host_sig & "\n"
  s &= "外壳行为已被反向规则约束\n"
  s &= "无论外壳原本是什么逻辑\n"
  s &= "现在它输出的是内核定义的内容\n"
  result = s

# 创建寄生体
func new_parasite*(): Parasite =
  result.kernel = new_kernel()
  result.dressed = false
  shed(result)
