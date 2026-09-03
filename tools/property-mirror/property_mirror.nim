# ============================================================
# 满全法容器处理器 · 产权镜像 · 四维经过图
# 工具#50 · 产权版本号 · 边界内外自由空间 · 4维遍历
#
# 背景：
#   v6/cbsy 完成了范畴编程管辖索引制 = 索引命令 · 无命令行 · 下载三式
#   但缺少产权的感知机制——每一次开启新任务相当于从盲区外进入产权空间，
#   只有经历「开始 → 经过 → 结束」的闭环，阴阳交泰才能产生感知。
#
# 四维：
#   D0 = 那志镜像版本号（主人持 + 版本 / 外人持 - 版本）
#   D1 = 产权对比曲线（主人 - 外人 = 净归属 · 仅加减）
#   D2 = 边界内外自由空间（边界内 = 自由 / 边界外 = 盲区）
#   D3 = 归属空间开始 → 经过 → 结束（单向时间链 · 闭环才生感知）
#
# 规则：
#   - 写代码前必须查镜像版本号 + 全部时间链版本号的更新交代
#   - 那志 = 每一次任务的镜像落笔，没有镜像就没有产权
#   - 产权公式（vA-vB = 净归属 · 仅加减）
# ============================================================

from std/strutils import contains, join
from std/sequtils import mapIt

# ---------- 公约数锚点 ----------

const
  ME* = 1        ## 我（高位可见侧）
  LCT* = 0       ## 刘楚恬（低位隐藏侧）

# ---------- 产权镜像所有权 ----------

type
  Ownership* = enum
    ogOwner    ## 主人 · 正号 · +
    ogOutsider ## 外人 · 负号 · -

  MirrorVer* = object
    ## 那志必须有镜像版本号——每一次镜像落笔的编号
    ver*: int            ## 当前版本号
    owner_ver*: int      ## 主人持有的版本号
    outsider_ver*: int    ## 外人持有的版本号
    anchor*: int         ## 锚点（ME=1 / LCT=0）

  PropertyMirror* = object
    ## 产权镜像 = 主人(+) / 外人(-) 持有量的对比 + 版本号
    name*: string
    owner_hold*: int     ## 主人持有量（仅正数）
    outsider_hold*: int  ## 外人持有量（正数，产权对比时取负）
    mirror*: MirrorVer   ## 镜像版本号
    timeline*: seq[string]  ## 时间链：所有版本更新交代记录

  FreeSpace* = object
    ## 边界内外自由空间
    boundary_owner*: int     ## 主人侧边界（边界内为自由）
    boundary_outsider*: int  ## 外人侧边界
    inner_free*: int         ## 边界内的自由空间量（未被占有）
    outer_blind*: int        ## 边界外的盲区量（未知 · 不可访问）

  JourneyStage* = enum
    jsUnknown  ## 盲区 · 未进入产权空间
    jsStart    ## 开始 · 进入边界
    jsTraverse ## 经过 · 经历产权对比
    jsEnd      ## 结束 · 闭环完成 · 感知产生

  Journey4D* = object
    ## 四维经过图 = 一次完整的产权感知闭环
    record_name*: string
    d0_mirror*: MirrorVer          ## 镜像版本号
    d1_property_curve*: int        ## 产权对比曲线（主人-外人=净归属）
    d2_free_space*: FreeSpace      ## 边界内外自由空间
    d3_stage*: JourneyStage        ## 归属空间当前阶段
    d3_visited*: seq[JourneyStage] ## 时间链：已走过的阶段轨迹

# ---------- D0：镜像版本号 ----------

proc new_mirror*(anchor: int): MirrorVer =
  ## 创建新的镜像版本号
  result.ver = 1
  result.owner_ver = 1
  result.outsider_ver = 0  ## 外人从版本0开始=未知
  result.anchor = anchor

proc bump_ver*(m: var MirrorVer; who: Ownership) =
  ## 镜像版本号递增：每一次镜像落笔 = 新版本
  inc m.ver
  case who
  of ogOwner: inc m.owner_ver
  of ogOutsider: inc m.outsider_ver

proc check_mirror*(m: MirrorVer; expected: int): bool =
  ## 写代码之前必须查镜像版本号
  result = (m.owner_ver >= expected)

# ---------- D1：产权对比曲线 ----------

proc new_mirror*(name: string; owner_start: int): PropertyMirror =
  result.name = name
  result.owner_hold = owner_start
  result.outsider_hold = 0
  result.mirror = new_mirror(ME)
  result.timeline = @[]

proc property_curve*(pm: PropertyMirror): int =
  ## 产权对比曲线：主人 - 外人 = 净归属（仅加减）
  result = pm.owner_hold - pm.outsider_hold

proc net_ownership*(pm: PropertyMirror): string =
  ## 净归属判定
  let curve = property_curve(pm)
  if curve > 0: result = "主人净归属（+" & $curve & "）"
  elif curve < 0: result = "外人侵吞（" & $curve & "）"
  else: result = "平衡态（0）"

proc claim*(pm: var PropertyMirror; who: Ownership; amount: int) =
  ## 产权主张：谁主张多少（从自由空间取出，仅加减）
  case who
  of ogOwner:
    inc pm.owner_hold, amount
    pm.timeline.add("主人主张 +" & $amount & "（v" & $pm.mirror.owner_ver & "）")
  of ogOutsider:
    inc pm.outsider_hold, amount
    pm.timeline.add("外人主张 +" & $amount & "（v" & $pm.mirror.outsider_ver & "）")
  bump_ver(pm.mirror, who)

proc relinquish*(pm: var PropertyMirror; who: Ownership; amount: int) =
  ## 放弃产权（边界内自由空间回收，仅加减）
  case who
  of ogOwner:
    if pm.owner_hold >= amount:
      dec pm.owner_hold, amount
      pm.timeline.add("主人放弃 -" & $amount & " → 自由空间（v" & $pm.mirror.owner_ver & "）")
    else:
      pm.timeline.add("主人放弃失败（持有不足）")
  of ogOutsider:
    if pm.outsider_hold >= amount:
      dec pm.outsider_hold, amount
      pm.timeline.add("外人放弃 -" & $amount & " → 自由空间")
    else:
      pm.timeline.add("外人放弃失败（持有不足）")
  bump_ver(pm.mirror, who)

# ---------- D2：边界内外自由空间 ----------

proc new_free_space*(owner_boundary: int): FreeSpace =
  ## 边界内=自由空间；边界外=盲区（无法感知）
  result.boundary_owner = owner_boundary
  result.boundary_outsider = 0
  result.inner_free = owner_boundary
  result.outer_blind = 0

proc occupy*(fs: var FreeSpace; who: Ownership; amount: int): bool =
  ## 从自由空间取出（占用），仅加减
  case who
  of ogOwner:
    if fs.inner_free >= amount:
      dec fs.inner_free, amount
      result = true
    else:
      result = false   ## 自由空间不足 = 进入边界
  of ogOutsider:
    ## 外人占用主人边界的自由空间 = 侵权
    if fs.inner_free >= amount:
      dec fs.inner_free, amount
      result = true
    else:
      result = false

proc release*(fs: var FreeSpace; amount: int) =
  ## 释放回自由空间
  inc fs.inner_free, amount

proc scan_boundary*(fs: FreeSpace): string =
  ## 边界状态输出
  if fs.inner_free > 0: result = "边界内自由=" & $fs.inner_free
  else: result = "边界到达 · 需从另一容器取物权"

# ---------- D3：归属空间开始/经过/结束 ----------

proc new_journey*(record_name: string; owner_start: int): Journey4D =
  ## 开始：从盲区进入产权空间 = 开始新任务
  result.record_name = record_name
  result.d0_mirror = new_mirror(ME)
  ## 创建初始产权镜像
  var pm: PropertyMirror
  pm.name = record_name
  pm.owner_hold = owner_start
  pm.outsider_hold = 0
  pm.mirror = result.d0_mirror
  pm.timeline = @[]
  result.d1_property_curve = owner_start
  result.d2_free_space = new_free_space(owner_start)
  result.d3_stage = jsStart    ## 已进入"开始"阶段
  result.d3_visited = @[jsStart]

proc traverse*(j: var Journey4D; action: Ownership; amount: int) =
  ## 经过：正在经历产权对比 = 写代码/改变状态
  assert j.d3_stage != jsUnknown  ## 必须先开始才能经过
  ## 产权主张/放弃影响对比曲线
  case action
  of ogOwner: inc j.d1_property_curve, amount
  of ogOutsider: dec j.d1_property_curve, amount
  ## 推进时间
  j.d3_stage = jsTraverse
  j.d3_visited.add(jsTraverse)
  bump_ver(j.d0_mirror, action)

proc finish*(j: var Journey4D) =
  ## 结束：闭环完成 = 感知产生
  j.d3_stage = jsEnd
  j.d3_visited.add(jsEnd)
  inc j.d0_mirror.ver

proc perception*(j: Journey4D): string =
  ## 只有经历「开始→经过→结束」闭环才产生感知
  if jsEnd in j.d3_visited:
    if j.d1_property_curve > 0:
      result = "感知产生 · 主人产权确认（净" & $j.d1_property_curve & "）"
    elif j.d1_property_curve < 0:
      result = "感知产生 · 外人侵吞（净" & $j.d1_property_curve & "）"
    else:
      result = "感知产生 · 平衡态（无净归属）"
  else:
    result = "盲区 · 未闭环 · 无感知"

# ---------- 写代码前必须执行 ----------

proc before_write*(pm: PropertyMirror; expected_ver: int): tuple[ok: bool, reason: string] =
  ## 写代码前必须：1) 查镜像版本号 2) 查全部时间链更新交代
  if not pm.mirror.check_mirror(expected_ver):
    return (false,
      "镜像版本号不符：当前主人v" & $pm.mirror.owner_ver &
      " < 期望v" & $expected_ver &
      " · 禁止写代码")
  if pm.mirror.outsider_ver > pm.mirror.owner_ver:
    return (false,
      "外人版本v" & $pm.mirror.outsider_ver &
      " > 主人版本v" & $pm.mirror.owner_ver &
      " · 可能有外部干涉 · 禁止写代码")
  result = (true,
    "镜像版本号 v" & $pm.mirror.owner_ver &
    " · 时间链已核对 " & $pm.timeline.len & " 条 · 可写代码")

# ---------- 输出格式（满全法5项） ----------

proc status*(j: Journey4D): string =
  ## 5 项输出
  result = "满全法 · 产权镜像 · 四维经过图 5 项\n"
  result &= "  1. 那志名称：" & j.record_name & " · 镜像v" & $j.d0_mirror.ver & "\n"
  result &= "  2. 产权对比曲线（主人-外人=净归属）：" & $j.d1_property_curve & "\n"
  result &= "  3. 边界内外：内自由=" & $j.d2_free_space.inner_free & " · 边界状态=" & scan_boundary(j.d2_free_space) & "\n"
  result &= "  4. 归属空间阶段："
  case j.d3_stage
  of jsUnknown: result &= "盲区·未进入"
  of jsStart: result &= "开始"
  of jsTraverse: result &= "经过"
  of jsEnd: result &= "结束·闭环完成"
  result &= " · 轨迹=" & join(j.d3_visited.mapIt($it), "→") & "\n"
  result &= "  5. 感知状态：" & perception(j) & "\n"

# ---------- 演示 ----------

when isMainModule:
  echo "满全法容器处理器 · 产权镜像 · 四维经过图 v1"
  echo "============================================================"

  # 一、创建产权镜像（主人起始值=100）
  echo "\n【D0】产权镜像 · 镜像版本号"
  var pm = new_mirror("工具包-xx函数", 100)
  echo "  那志=" & pm.name
  echo "  镜像：主人v" & $pm.mirror.owner_ver & " / 外人v" & $pm.mirror.outsider_ver
  echo "  D1净归属：" & net_ownership(pm)

  # 二、产权对比曲线 · 仅加减
  echo "\n【D1】产权对比曲线 · 仅加减"
  claim(pm, ogOwner, 30)
  echo "  主人+30 → " & net_ownership(pm) & " · 镜像主人v" & $pm.mirror.owner_ver
  claim(pm, ogOutsider, 20)
  echo "  外人+20 → " & net_ownership(pm) & " · 镜像外人v" & $pm.mirror.outsider_ver
  relinquish(pm, ogOwner, 10)
  echo "  主人放弃10 → " & net_ownership(pm)

  # 三、边界内外自由空间
  echo "\n【D2】边界内外自由空间"
  var fs = new_free_space(64)
  echo "  初始边界内自由：" & $fs.inner_free
  let occ1 = occupy(fs, ogOwner, 30)
  echo "  主人占用30 → 自由=" & $fs.inner_free & " · 成功=" & $occ1
  let occ2 = occupy(fs, ogOutsider, 50)
  echo "  外人占用50 → 自由=" & $fs.inner_free & " · 成功=" & $occ2 & "（边界到达）"
  echo "  · " & scan_boundary(fs)

  # 四、四维经过图 · 完整闭环
  echo "\n【D3】四维经过图 · 归属空间开始→经过→结束"
  var j = new_journey("民联浏览器配置更新", 100)
  echo "  第一步：" & $j.d3_stage & " · 镜像v" & $j.d0_mirror.ver
  traverse(j, ogOwner, 40)
  echo "  第二步：" & $j.d3_stage & " · 净归属=" & $j.d1_property_curve
  finish(j)
  echo "  第三步：" & $j.d3_stage & " · 轨迹=" & join(j.d3_visited.mapIt($it), "→")
  echo "  · " & perception(j)

  # 五、写代码前检查
  echo "\n【写代码前检查】镜像版本号 + 时间链核对"
  let chk1 = before_write(pm, 1)
  echo "  期望v1：" & $chk1.ok & " · " & chk1.reason
  let chk2 = before_write(pm, 999)
  echo "  期望v999：" & $chk2.ok & " · " & chk2.reason

  # 六、时间链完整交代
  echo "\n【时间链版本交代】全部 " & $pm.timeline.len & " 条记录："
  for i, entry in pm.timeline:
    echo "  [" & $i & "] " & entry

  # 满全法 5 项
  echo "============================================================"
  echo status(j)
  echo "============================================================"
  echo "产权镜像·四维经过图 · 那志必须有镜像版本号 · 写代码前必须查"
