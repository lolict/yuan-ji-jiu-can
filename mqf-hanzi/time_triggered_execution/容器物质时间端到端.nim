import std/[tables, sets, math, strutils, sequtils]

#==============================================================
#  容器=物质=时间 端到端执行演示
#
#  机器码容器引擎 (容器=编码结构=字形载体)
#    ↓ 接口: pack_id/operand/time_window/witness/entry
#  时间触发执行引擎 (时间=激活时隙)
#    ↓ 触发执行
#  机器码运行 (实际LOAD/STORE/ADD/...指令)
#
#  接口对齐:
#    pack_id      → 容器校验和 (唯一标识)
#    operand      → 第一条指令操作数 (归一化16位)
#    time_window  → Z轴时隙 (触发相位)
#    witness      → 容器校验和 (裁决用)
#    entry        → 入口地址 (CRT入口)
#==============================================================

type
  整数* = int

  #=== 机器码操作码 (私有指令集) ===#
  操作码* = enum
    NOP, LOAD, STORE, ADD, SUB, MUL, CMP,
    JMP, JZ, JNZ, SEND, RECV, GLYPH, CALL, RET, HALT

  #=== 机器码指令 (一字节=一字形) ===#
  指令* = object
    opcode*: 操作码
    operand*: 整数
    glyph*: string
    address_3d*: tuple[横: float, 纵: float, 深: float]
    polarity*: 整数
    timing*: 整数

  #=== 机器码容器 = 字体文件+可执行代码 ===#
  机器码容器* = object
    魔数*: array[4, char]
    总字数*: 整数
    校验和*: 整数
    创建时间*: float
    入口地址*: 整数
    程序*: seq[指令]
    数据*: Table[string, 整数]
    状态*: seq[整数]

  #=== 执行状态 ===#
  执行状态* = object
    容器*: 机器码容器
    PC*: 整数
    容器栈*: seq[整数]
    ACC*: 整数
    FLAGS*: tuple[零: bool, 进位: bool, 极性: bool]
    步数*: 整数
    活跃*: bool

  #=== 时间触发容器 = 机器码容器 + 接口字段 ===#
  时间触发容器* = object
    容器*: 机器码容器
    容器标识*: 整数
    时间窗口*: 整数
    见证*: 整数
    入口*: 整数
    已激活*: bool

  端到端引擎* = object
    容器表*: seq[时间触发容器]
    当前相位*: 整数
    周期*: 整数
    裁决统计*: tuple[空拒: 整数, 有效纳: 整数, 全零拒: 整数]
    历史*: seq[string]

#==============================================================
#  机器码容器引擎 API (内联副本)
#==============================================================

proc 操作码名*(编码: 操作码): string =
  case 编码
    of NOP: "NOP"
    of LOAD: "LOAD"
    of STORE: "STORE"
    of ADD: "ADD"
    of SUB: "SUB"
    of MUL: "MUL"
    of CMP: "CMP"
    of JMP: "JMP"
    of JZ: "JZ"
    of JNZ: "JNZ"
    of SEND: "SEND"
    of RECV: "RECV"
    of GLYPH: "GLYPH"
    of CALL: "CALL"
    of RET: "RET"
    of HALT: "HALT"

proc 操作码到字形*(操作: 操作码; 极性: 整数): string =
  case 操作
    of NOP: "Ѧ"
    of LOAD: "ѧ"
    of STORE: "ə"
    of ADD: "ɐ"
    of SUB: "ɒ"
    of MUL: "ɔ"
    of CMP: "ɛ"
    of JMP: "ɜ"
    of JZ: "ɝ"
    of JNZ: "ɞ"
    of SEND: "ɑ"
    of RECV: "ɒ"
    of GLYPH: "ꜵ"
    of CALL: "Ꜵ"
    of RET: "ʔ"
    of HALT: "Ɑ"

proc 创建立方体编码*(编码: 整数): tuple[横: 整数, 纵: 整数, 深: 整数] =
  let x = (编码 div 16) mod 4
  let y = (编码 div 4) mod 4
  let z = 编码 mod 4
  (x, y, z)

proc 创建容器*(入口符号: string): 机器码容器 =
  var c: 机器码容器
  c.魔数 = ['M','Q','F','\0']
  c.总字数 = 0
  c.校验和 = 0
  c.创建时间 = 0.0
  c.入口地址 = 0
  c.程序 = @[]
  c.数据 = initTable[string, 整数]()
  c.状态 = newSeq[整数](256)
  result = c

proc 添加指令*(容器: var 机器码容器; 操作: 操作码; 操作数: 整数 = 0; 极性: 整数 = 1; 时序: 整数 = 0) =
  var instr: 指令
  instr.opcode = 操作
  instr.operand = 操作数
  instr.glyph = 操作码到字形(操作, 极性)
  let 编码 = 操作.ord * 16 + (操作数 mod 16)
  let xyz = 创建立方体编码(编码)
  instr.address_3d = (float(xyz.横), float(xyz.纵), float(xyz.深))
  instr.polarity = 极性
  instr.timing = 时序
  容器.程序.add(instr)
  容器.总字数 = 容器.程序.len

proc 编译*(容器: var 机器码容器) =
  var sum = 0
  for instr in 容器.程序:
    sum += instr.opcode.ord + instr.operand + instr.polarity + instr.timing
  容器.校验和 = sum mod 256

proc 创建执行状态*(容器: 机器码容器): 执行状态 =
  执行状态(容器: 容器, PC: 0, 容器栈: @[], ACC: 0,
            FLAGS: (false, false, false), 步数: 0, 活跃: true)

proc 读容器值*(状态: 执行状态; 地址: 整数): 整数 =
  let idx = 地址 mod 256
  状态.容器.状态[idx]

proc 写容器值*(状态: var 执行状态; 地址: 整数; 值: 整数) =
  let idx = 地址 mod 256
  状态.容器.状态[idx] = 值

proc 取指*(状态: 执行状态): 指令 =
  if 状态.PC < 状态.容器.程序.len:
    result = 状态.容器.程序[状态.PC]
  else:
    result = 指令(opcode: HALT, operand: 0, polarity: 1, timing: 0)

proc 单步执行*(状态: var 执行状态; 日志: var seq[string]): bool =
  if not 状态.活跃 or 状态.PC >= 状态.容器.程序.len:
    状态.活跃 = false
    return false
  let instr = 取指(状态)
  let oldPC = 状态.PC
  状态.PC += 1
  case instr.opcode
    of NOP:
      日志.add "[" & align($oldPC, 3) & "] NOP"
    of LOAD:
      状态.ACC = 读容器值(状态, instr.operand)
      日志.add "[" & align($oldPC, 3) & "] LOAD " & align($instr.operand, 3) & " → ACC=" & $状态.ACC
    of STORE:
      写容器值(状态, instr.operand, 状态.ACC)
      日志.add "[" & align($oldPC, 3) & "] STORE " & align($instr.operand, 3) & " ← ACC=" & $状态.ACC
    of ADD:
      状态.ACC = (状态.ACC + 读容器值(状态, instr.operand)) mod 256
      日志.add "[" & align($oldPC, 3) & "] ADD " & align($instr.operand, 3) & " → ACC=" & $状态.ACC
    of SUB:
      状态.ACC = (状态.ACC - 读容器值(状态, instr.operand) + 256) mod 256
      日志.add "[" & align($oldPC, 3) & "] SUB " & align($instr.operand, 3)
    of MUL:
      状态.ACC = (状态.ACC * 读容器值(状态, instr.operand)) mod 256
      日志.add "[" & align($oldPC, 3) & "] MUL " & align($instr.operand, 3) & " → ACC=" & $状态.ACC
    of CMP:
      let v = 读容器值(状态, instr.operand)
      状态.FLAGS = ((状态.ACC == v), false, 状态.ACC > v)
    of JMP:
      状态.PC = instr.operand
      日志.add "[" & align($oldPC, 3) & "] JMP " & $instr.operand
    of JZ:
      if 状态.ACC == 0:
        状态.PC = instr.operand
        日志.add "[" & align($oldPC, 3) & "] JZ → " & $instr.operand
    of JNZ:
      if 状态.ACC != 0:
        状态.PC = instr.operand
        日志.add "[" & align($oldPC, 3) & "] JNZ → " & $instr.operand
    of SEND:
      日志.add "[" & align($oldPC, 3) & "] SEND " & $instr.operand
    of RECV:
      日志.add "[" & align($oldPC, 3) & "] RECV"
    of GLYPH:
      let 编码 = instr.operand
      let 字节码 = (编码 * 3 + 17) mod 256
      状态.ACC = 字节码
      日志.add "[" & align($oldPC, 3) & "] GLYPH " & $编码 & " → 字节码=" & $字节码
    of CALL:
      日志.add "[" & align($oldPC, 3) & "] CALL " & $instr.operand
    of RET:
      日志.add "[" & align($oldPC, 3) & "] RET"
    of HALT:
      状态.活跃 = false
      日志.add "[" & align($oldPC, 3) & "] HALT"
  状态.步数 += 1
  result = 状态.活跃

proc 运行*(状态: var 执行状态; 日志: var seq[string]; 最大步数: 整数 = 1000) =
  while 状态.活跃 and 状态.步数 < 最大步数:
    discard 单步执行(状态, 日志)

#==============================================================
#  时间触发执行引擎
#==============================================================

proc 创建设施*(周期: 整数 = 12): 端到端引擎 =
  result.周期 = 周期
  result.当前相位 = 0
  result.容器表 = @[]
  result.裁决统计 = (0, 0, 0)
  result.历史 = @[]

proc 注册容器*(引擎: var 端到端引擎; 容器: 机器码容器; 时间窗口: 整数; 索引: 整数): 整数 =
  ## 注册一个机器码容器, 返回 pack_id (校验和)
  let 容器标识 = 容器.校验和 or (索引 * 256)
  var 触发容器: 时间触发容器
  触发容器.容器 = 容器
  触发容器.容器标识 = 容器标识
  触发容器.时间窗口 = 时间窗口
  触发容器.见证 = 容器.校验和
  触发容器.入口 = 容器.入口地址
  触发容器.已激活 = false
  引擎.容器表.add(触发容器)
  引擎.历史.add("注册: 标识=" & $容器标识 & " 总字数=" & $容器.总字数 &
                  " 校验和=" & $容器.校验和 & " 窗口=" & $时间窗口)
  result = 容器标识

proc 裁决*(引擎: 端到端引擎; 容器标识: 整数; 见证: 整数): string =
  for 触发容器 in 引擎.容器表:
    if 触发容器.容器标识 == 容器标识:
      if 触发容器.见证 == 0: return "全零拒"
      elif 触发容器.见证 == 见证: return "有效纳"
      else: return "空拒"
  return "空拒"

proc 裁决记录*(引擎: var 端到端引擎; 容器标识: 整数; 见证: 整数): string =
  let r = 裁决(引擎, 容器标识, 见证)
  case r
    of "空拒": 引擎.裁决统计.空拒 += 1
    of "有效纳": 引擎.裁决统计.有效纳 += 1
    of "全零拒": 引擎.裁决统计.全零拒 += 1
    else: discard
  result = r

proc 推进*(引擎: var 端到端引擎; 步长: 整数 = 1) =
  引擎.当前相位 = (引擎.当前相位 + 步长) mod 引擎.周期

proc 窗口匹配*(引擎: 端到端引擎): seq[整数] =
  result = @[]
  for 触发容器 in 引擎.容器表:
    if not 触发容器.已激活 and 引擎.当前相位 == 触发容器.时间窗口:
      result.add(触发容器.容器标识)

proc 触发执行*(引擎: var 端到端引擎; 容器标识: 整数): seq[string] =
  ## 时间触发的执行: 裁决 → 加载 → 运行 → 收集日志
  result = @[]

  # 查找容器 (索引方式)
  var index = -1
  for i, c in 引擎.容器表:
    if c.容器标识 == 容器标识:
      index = i
      break

  if index < 0:
    result.add "[空拒] 标识=" & $容器标识
    引擎.裁决统计.空拒 += 1
    return

  引擎.容器表[index].已激活 = true
  let witness = 引擎.容器表[index].见证
  let verdict = 裁决记录(引擎, 容器标识, witness)

  if verdict != "有效纳":
    result.add "[" & verdict & "] 标识=" & $容器标识
    return

  let 容器 = 引擎.容器表[index].容器
  result.add "[有效纳] 标识=" & $容器标识 & " 总字数=" & $容器.总字数
  result.add "  ── 开始执行机器码 ──"

  var 状态 = 创建执行状态(容器)
  var 执行日志: seq[string] = @[]
  运行(状态, 执行日志, 200)

  for entry in 执行日志:
    result.add "  " & entry

  result.add "  ── 执行结束 ──"
  result.add "  → ACC=" & $状态.ACC & " 步数=" & $状态.步数

#==============================================================
#  容器=物质=时间 端到端演示
#==============================================================

when isMainModule:
  echo "═══ 容器=物质=时间 端到端执行 ═══"
  echo ""
  echo "机器码容器→时间窗口→裁决→实际执行"
  echo ""

  var 引擎 = 创建设施(12)

  # 容器甲: 累加器
  var 甲 = 创建容器("adder")
  var 状态甲 = 创建执行状态(甲)
  状态甲.容器.状态[0] = 5
  for inst in 状态甲.容器.程序.mitems: discard
  # 通过添加指令构建
  var 甲容器 = 创建容器("adder")
  # 手动构建指令序列
  var inst1: 指令
  inst1.opcode = LOAD; inst1.operand = 0; inst1.polarity = 1
  let c1 = 1 * 16
  let xyz1 = 创建立方体编码(c1)
  inst1.address_3d = (float(xyz1.横), float(xyz1.纵), float(xyz1.深))
  inst1.glyph = "ѧ"
  甲容器.程序.add(inst1)
  var inst2: 指令
  inst2.opcode = ADD; inst2.operand = 0; inst2.polarity = 1
  let c2 = 3 * 16
  let xyz2 = 创建立方体编码(c2)
  inst2.address_3d = (float(xyz2.横), float(xyz2.纵), float(xyz2.深))
  inst2.glyph = "ɐ"
  甲容器.程序.add(inst2)
  var inst3: 指令
  inst3.opcode = STORE; inst3.operand = 1; inst3.polarity = 0
  let c3 = 2 * 16
  let xyz3 = 创建立方体编码(c3)
  inst3.address_3d = (float(xyz3.横), float(xyz3.纵), float(xyz3.深))
  inst3.glyph = "ə"
  甲容器.程序.add(inst3)
  var inst4: 指令
  inst4.opcode = HALT; inst4.operand = 0; inst4.polarity = 0
  let c4 = 15 * 16
  let xyz4 = 创建立方体编码(c4)
  inst4.address_3d = (float(xyz4.横), float(xyz4.纵), float(xyz4.深))
  inst4.glyph = "Ɑ"
  甲容器.程序.add(inst4)
  甲容器.总字数 = 4
  甲容器.状态[0] = 5
  编译(甲容器)
  let id甲 = 注册容器(引擎, 甲容器, 0, 0)

  # 容器乙: 字形投影
  var 乙容器 = 创建容器("glyph")
  var g1: 指令
  g1.opcode = GLYPH; g1.operand = 238; g1.polarity = 1
  let gc = 12 * 16 + 14
  let gxyz = 创建立方体编码(gc)
  g1.address_3d = (float(gxyz.横), float(gxyz.纵), float(gxyz.深))
  g1.glyph = "ꜵ"
  乙容器.程序.add(g1)
  var g2: 指令
  g2.opcode = STORE; g2.operand = 50; g2.polarity = 0
  let sc = 2 * 16 + 50
  let sxyz = 创建立方体编码(sc)
  g2.address_3d = (float(sxyz.横), float(sxyz.纵), float(sxyz.深))
  g2.glyph = "ə"
  乙容器.程序.add(g2)
  var g3: 指令
  g3.opcode = HALT; g3.operand = 0; g3.polarity = 0
  g3.glyph = "Ɑ"
  乙容器.程序.add(g3)
  乙容器.总字数 = 3
  编译(乙容器)
  let id乙 = 注册容器(引擎, 乙容器, 4, 1)

  # 容器丙: 全零见证 (测试全零拒)
  var 丙容器 = 创建容器("zero_witness")
  var z1: 指令
  z1.opcode = NOP; z1.operand = 0; z1.polarity = 0; z1.glyph = "Ѧ"
  丙容器.程序.add(z1)
  var z2: 指令
  z2.opcode = HALT; z2.operand = 0; z2.polarity = 0; z2.glyph = "Ɑ"
  丙容器.程序.add(z2)
  丙容器.总字数 = 2
  丙容器.校验和 = 0
  编译(丙容器)
  let id丙 = 注册容器(引擎, 丙容器, 8, 2)

  echo "─ 注册容器 ─"
  for i, c in 引擎.容器表:
    let op0 = if c.容器.总字数 > 0: 操作码名(c.容器.程序[0].opcode) else: "空"
    echo "  [" & $i & "] 标识=" & $c.容器标识 &
         " 校验和=" & $c.容器.校验和 &
         " 窗口=" & $c.时间窗口 &
         " 首指令=" & op0
  echo ""

  # 演示1: 相位0 → 触发容器甲
  echo "─ 演示1: 相位0 → 容器甲(累加器) ─"
  引擎.当前相位 = 0
  let m1 = 窗口匹配(引擎)
  echo "匹配: ", m1
  for id in m1:
    let lines = 触发执行(引擎, id)
    for l in lines:
      echo "  " & l
  echo ""

  # 演示2: 推进到相位4 → 触发容器乙
  echo "─ 演示2: 相位4 → 容器乙(字形投影) ─"
  while 引擎.当前相位 != 4: 推进(引擎)
  let m2 = 窗口匹配(引擎)
  echo "匹配: ", m2
  for id in m2:
    let lines = 触发执行(引擎, id)
    for l in lines:
      echo "  " & l
  echo ""

  # 演示3: 相位8 → 触发容器丙 (全零拒)
  echo "─ 演示3: 相位8 → 容器丙(全零见证) ─"
  引擎.当前相位 = 8
  let m3 = 窗口匹配(引擎)
  echo "匹配: ", m3
  for id in m3:
    let lines = 触发执行(引擎, id)
    for l in lines:
      echo "  " & l
  echo ""

  # 演示4: 裁决未注册
  echo "─ 演示4: 裁决未注册标识 ─"
  let m4 = 触发执行(引擎, 9999)
  echo "  " & m4[0]

  # 报告
  echo ""
  echo "═" & repeat("═", 50)
  echo "  容器=物质=时间 端到端 — 裁决报告"
  echo "═" & repeat("═", 50)
  echo "注册容器: ", 引擎.容器表.len
  echo "空拒: ", 引擎.裁决统计.空拒
  echo "有效纳: ", 引擎.裁决统计.有效纳
  echo "全零拒: ", 引擎.裁决统计.全零拒
  echo ""
  echo "容器=物质=时间: 容器(编码) → 物质(字形) → 时间(窗口触发) → 裁决 → 执行"
  echo "全链路: 纯Nim命令行·全中文符号·零外部依赖"
