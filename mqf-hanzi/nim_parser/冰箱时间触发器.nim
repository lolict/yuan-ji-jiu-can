## 满全法 - 冰箱时间触发器
## 全局时钟 + 触发器调度 + 容器加载/卸载/执行
##
## 冰箱模型 = 时间触发容器加载
## 压缩机运转 = 执行容器碰撞程序
## 温度升降 = 全局能量波动
## 制冷周期 = 加载→执行→卸载→恢复
##
## 触发类型：
##   ttTime      — 到达指定时钟周期
##   ttCollision — 两个容器距离归0时
##   ttOverflow  — 容器能量超限
##   ttCool      — 温度下降到阈值

import tables
import sequtils
import strutils

type
  容器态 = enum csEmpty, csLoaded, csLocked, csOverflow

  容器 = ref object
    地址* : int
    态* : 容器态
    值* : int
    名称* : string
    能量* : int
    激活时间* : int
    过期时间* : int
    已执行* : bool

  集群 = ref object
    池* : Table[int, 容器]
    下一个* : int

  触发类型* = enum ttTime, ttCollision, ttOverflow, ttCool
  触发阶段* = enum tsPending, tsLoaded, tsDone

  触发器* = object
    类型* : 触发类型
    触发值* : int
    目标地址* : int
    阶段* : 触发阶段
    已触发* : bool

  冰箱* = ref object
    时钟* : int
    温度* : float
    触发器表* : seq[触发器]
    集群* : 集群
    日志* : seq[string]
    历史* : seq[int]        # 历史能量记录

# === 基础 ===

proc 创建集群(): 集群 =
  new(result); result.池 = initTable[int, 容器](); result.下一个 = 0

proc 创建冰箱(): 冰箱 =
  new(result)
  result.时钟 = 0; result.温度 = 20.0
  result.触发器表 = @[]; result.日志 = @[]; result.历史 = @[]
  result.集群 = 创建集群()

proc 放入*(冰: 冰箱, 名称: string, 值: int): int =
  var c: 容器; new(c)
  c.地址 = 冰.集群.下一个; c.名称 = 名称; c.值 = 值
  c.态 = csEmpty; c.能量 = 0; c.激活时间 = -1; c.过期时间 = -1
  冰.集群.池[c.地址] = c; 冰.集群.下一个 += 1
  result = c.地址

# === 触发器注册 ===

proc 注册*(冰: 冰箱, 类型: 触发类型, 触发值: int, 目标地址: int) =
  var t: 触发器
  t.类型 = 类型; t.触发值 = 触发值; t.目标地址 = 目标地址
  t.阶段 = tsPending; t.已触发 = false
  冰.触发器表.add(t)
  冰.日志.add("注册触发[" & $类型 & "] 时钟=" & $触发值 & " 目标=#" & $目标地址)

# === 温度（能量 / 活跃容器数） ===

proc 更新温度*(冰: 冰箱) =
  var e = 0; var n = 0
  for _, c in 冰.集群.池:
    if c.态 == csLoaded or c.态 == csOverflow: e += c.能量; n += 1
  冰.温度 = if n > 0: e.float / n.float * 0.1 else: 0.0
  冰.历史.add(e)

# === 触发分发 ===

proc 检查触发*(冰: 冰箱): seq[触发器] =
  result = @[]
  for i in 0..<冰.触发器表.len:
    var t = 冰.触发器表[i]
    if t.已触发: continue
    var fire = false
    case t.类型:
    of ttTime:      fire = (冰.时钟 >= t.触发值)
    of ttOverflow:
      if 冰.集群.池.hasKey(t.目标地址):
        fire = 冰.集群.池[t.目标地址].态 == csOverflow
    of ttCollision: fire = false
    of ttCool:      fire = (冰.温度 < t.触发值.float and 冰.时钟 > 0)
    if fire:
      冰.触发器表[i].已触发 = true; result.add(t); 冰.日志.add("触发@时钟" & $冰.时钟 & " → #" & $t.目标地址)

# === 容器生命周期 ===

proc 加载*(冰: 冰箱, 地址: int) =
  if not 冰.集群.池.hasKey(地址): return
  var c = 冰.集群.池[地址]
  c.态 = csLoaded; c.能量 = 100 + 地址 * 10; c.激活时间 = 冰.时钟; c.过期时间 = 冰.时钟 + 50
  冰.日志.add("  加载↑ #" & $地址 & " " & c.名称 & " 能量=" & $c.能量)

proc 卸载*(冰: 冰箱, 地址: int) =
  if not 冰.集群.池.hasKey(地址): return
  var c = 冰.集群.池[地址]
  c.态 = csLocked; c.能量 = 0
  冰.日志.add("  卸载↓ #" & $地址 & " " & c.名称 & " 能量归0")

proc 执行*(冰: 冰箱, 地址: int) =
  if not 冰.集群.池.hasKey(地址): return
  var c = 冰.集群.池[地址]
  c.已执行 = true
  c.能量 -= 10  # 执行消耗能量
  if c.能量 > 200: c.态 = csOverflow
  冰.日志.add("  执行★ #" & $地址 & " " & c.名称 & " 值=" & $c.值 & " 能量→" & $c.能量)

proc 推进*(冰: 冰箱) =
  冰.时钟 += 1
  冰.更新温度()
  # 过期自动卸载
  for _, c in 冰.集群.池:
    if c.态 == csLoaded and c.过期时间 == 冰.时钟:
      冰.卸载(c.地址)
      冰.日志.add("  过期→卸载 #" & $c.地址 & " " & c.名称)

# === 主程序 ===

when isMainModule:
  echo "================================================"
  echo " MQFA Refrigerator Time Trigger"
  echo " Time-line Container Load/Unload/Execute"
  echo "================================================"
  echo ""

  var 冰 = 创建冰箱()

  # -- 创建待加载容器 --
  echo "--- [初始化] 等待时间触发的容器 ---"
  let a = 冰.放入("物质A:bit_2",     2);    echo "  #" & $a & " 物质A (2bit)"
  let b = 冰.放入("物质B:mlll_64",    64);   echo "  #" & $b & " 物质B (64进制)"
  let c = 冰.放入("物质C:tone_100",   100);  echo "  #" & $c & " 物质C (100声调)"
  let d = 冰.放入("物质D:phase_9",    9);    echo "  #" & $d & " 物质D (相位)"
  let e = 冰.放入("物质E:yun_32",     32);   echo "  #" & $e & " 物质E (韵母)"
  echo ""

  # -- 注册触发器 --
  echo "--- [触发器注册] ---"
  冰.注册(ttTime, 1, a)       # 时钟=1: 加载物质A
  冰.注册(ttTime, 3, b)       # 时钟=3: 加载物质B
  冰.注册(ttTime, 5, c)       # 时钟=5: 加载物质C
  冰.注册(ttTime, 8, d)       # 时钟=8: 加载物质D
  冰.注册(ttTime, 12, e)      # 时钟=12: 加载物质E
  echo ""

  # -- 执行时间线 --
  echo "--- [运行] 时间线推进 ---"
  for tick in 1..30:
    let fired = 冰.检查触发()
    for t in fired:
      case t.类型:
      of ttTime:
        冰.加载(t.目标地址)
        冰.执行(t.目标地址)
      of ttOverflow:
        冰.执行(t.目标地址)
      of ttCool:
        for _, cx in 冰.集群.池:
          if cx.态 == csLoaded and cx.过期时间 <= 冰.时钟:
            冰.卸载(cx.地址)
      of ttCollision: discard
    冰.推进()
    if tick <= 25:
      var loaded = ""
      for _, cx in 冰.集群.池:
        if cx.态 == csLoaded: loaded.add("#" & $cx.地址 & "(" & $cx.能量 & ") ")
      if loaded.len == 0 and tick < 4: continue
      echo "  时钟" & align($tick, 2) &
           " 温度=" & formatFloat(冰.温度, ffDecimal, 1) &
           " 活跃=[" & (if loaded.len > 0: loaded else: "无") & "]"

  echo ""
  echo "--- [历史温度] ---"
  var hist = "  "
  for i, e in 冰.历史:
    if i > 0 and i mod 10 == 0: hist.add("\n  ")
    hist.add($e & " ")
  echo hist
  echo ""

  echo "--- [最终容器状态] ---"
  for addr, cx in 冰.集群.池:
    echo "  #" & align($addr, 2) & " " & align(cx.名称, 12) &
         " 态=" & align($cx.态, 8) & " 值=" & align($cx.值, 4) &
         " 能量=" & align($cx.能量, 3) & " 激活@t" & $cx.激活时间 &
         " 过期@t" & $cx.过期时间 & (if cx.已执行: " ★已执行" else: "")

  echo ""
  echo "================================================"
  echo " Refrigerator Time Trigger OK"
  echo "================================================"
