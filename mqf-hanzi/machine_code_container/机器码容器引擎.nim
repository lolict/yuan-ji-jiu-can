import std/[tables, sets, math, strutils, sequtils, algorithm]

#==============================================================
#  机器码容器引擎 — 字节码↔字形双向可执行封装 + 容器加载 + 执行编排
#
#  机器码→C代码本身作为数字工具载体允许使用 (用户定义)
#  一字节码=一字形, 字形地址=3D坐标(x声母辐射, y韵母曲率, z声调高低)
#  容器=物质=时间: 容器(编码结构) 物质(字体文件) 时间(执行时序)
#  加载→链接→执行 全链路不依赖外部工具链, 端到端纯Nim命令行
#==============================================================

type
  整数* = int
  #=== 机器码操作码 (私有指令集, 基于满全法容器) ===#
  操作码* = enum
    NOP,        # 0x00 空操作 — 容器初始化
    LOAD,       # 0x01 加载地址 — 域加载
    STORE,      # 0x02 存储 — 注册
    ADD,        # 0x03 加法 — 加法统计
    SUB,        # 0x04 减法 — 负向反馈
    MUL,        # 0x05 乘法 — 频率叠加(仅统计)
    CMP,        # 0x06 比较 — 门控规则
    JMP,        # 0x07 跳转 — 路由
    JZ,         # 0x08 零跳转 — 通断
    JNZ,        # 0x09 非零跳转 — 护栏
    SEND,       # 0x0A 发送 — 跨壁通信
    RECV,       # 0x0B 接收 — 跨壁接收
    GLYPH,      # 0x0C 字形投影 → 一字节码映射
    CALL,       # 0x0D 调用 — 地址呼唤
    RET,        # 0x0E 返回 — 结束=开始
    HALT        # 0x0F 停机 — 失爱恐惧终止

  #=== 机器码指令 (一字节=一字形) ===#
  指令* = object
    opcode*: 操作码
    operand*: 整数           # 0-255: 地址/立即数
    glyph*: string          # 对应的字形符号
    address_3d*: tuple[横: float, 纵: float, 深: float]
    polarity*: 整数          # 0=阴 1=阳
    timing*: 整数            # 执行时序

  #=== 机器码容器 = 字体文件+可执行代码 ===#
  机器码容器* = object
    魔数*: array[4, char]          # "MQF\0"
    总字数*: 整数                    # 字形数量 = 指令数
    校验和*: 整数                    # 全部字节累加 mod 256
    创建时间*: float                # 时间戳
    入口地址*: 整数                  # 第一条指令地址 (CRT入口)
    程序*: seq[指令]        # 指令序列
    数据*: Table[string, 整数]      # 符号表
    状态*: seq[整数]                 # 256个槽位的运行时空容器

  #=== 执行状态 ===#
  执行状态* = object
    容器*: 机器码容器
    PC*: 整数                        # 程序计数器
    容器栈*: seq[整数]                # 容器运行时数据栈
    ACC*: 整数                       # 累加器
    FLAGS*: tuple[零: bool, 进位: bool, 极性: bool]
    步数*: 整数
    活跃*: bool

  #=== 操作码名称 ===#
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

#==============================================================
#  机器码→字形映射表 (满全法私有字形编码)
#==============================================================

proc 操作码到字形*(操作: 操作码; 极性: 整数): string =
  ## 机器码 → 字形 (声母韵母声调三元编码)
  ## 阴(刘楚恬)用上相位, 阳(我)用下相位
  let 基字形 = case 操作
    of NOP: "Ѧ"      # 空
    of LOAD: "ѧ"     # 加载
    of STORE: "ə"    # 存储
    of ADD: "ɐ"      # 加
    of SUB: "ɒ"      # 减
    of MUL: "ɔ"      # 乘
    of CMP: "ɛ"      # 比较
    of JMP: "ɜ"      # 跳转
    of JZ: "ɝ"       # 零跳
    of JNZ: "ɞ"      # 非零跳
    of SEND: "ɑ"     # 发送
    of RECV: "ɒ"     # 接收
    of GLYPH: "ꜵ"    # 字形
    of CALL: "Ꜵ"    # 调用
    of RET: "ʔ"      # 返回
    of HALT: "Ɑ"     # 停机
  if 极性 == 1:
    result = 基字形    # 阳用基本形
  else:
    result = 基字形 & "̥" # 阴加环下符

#==============================================================
#  构造机器码容器 (编译器前端)
#==============================================================

proc 创建容器*(入口符号: string): 机器码容器 =
  var 容器: 机器码容器
  容器.魔数 = ['M', 'Q', 'F', '\0']
  容器.总字数 = 0
  容器.校验和 = 0
  容器.创建时间 = 0.0
  容器.程序 = @[]
  容器.数据 = initTable[string, 整数]()
  容器.状态 = newSeq[整数](256)
  容器

proc 立方体编码*(编码: 整数): tuple[横: 整数, 纵: 整数, 深: 整数]
proc 添加指令*(容器: var 机器码容器; 操作: 操作码; 操作数: 整数 = 0; 极性: 整数 = 1; 时序: 整数 = 0) =
  ## 添加一条机器码指令
  var 指令字: 指令
  指令字.opcode = 操作
  指令字.operand = 操作数
  指令字.glyph = 操作码到字形(操作, 极性)
  指令字.polarity = 极性
  指令字.timing = 时序
  # 3D地址 = 操作码编码的几何位置
  let 编码 = 操作.ord * 16 + (操作数 mod 16)
  let xyz = 立方体编码(编码)
  指令字.address_3d = (float(xyz.横), float(xyz.纵), float(xyz.深))

  容器.程序.add(指令字)
  容器.总字数 = 容器.程序.len

proc 添加数据*(容器: var 机器码容器; 名字: string; 值: 整数) =
  容器.数据[名字] = 值
  容器.状态[容器.数据.len mod 256] = 值

proc 编译*(容器: var 机器码容器) =
  ## 编译: 计算校验和
  var sum = 0
  for 指令字 in 容器.程序:
    sum += 指令字.opcode.ord + 指令字.operand + 指令字.polarity + 指令字.timing
  容器.校验和 = sum mod 256

#==============================================================
#  3D字体文件 (容器=物质 的具体形式)
#==============================================================

proc 立方体编码*(编码: 整数): tuple[横: 整数, 纵: 整数, 深: 整数] =
  let x = (编码 div 16) mod 4
  let y = (编码 div 4) mod 4
  let z = 编码 mod 4
  (x, y, z)

proc 推导一字节码(编码: 整数): 整数 =
  (编码 * 3 + 17) mod 256

proc 生成字形文件头*(容器: 机器码容器): seq[string] =
  result.add "Q MQFont Container Format v1"
  result.add "G " & $容器.总字数 & " glyph instructions"
  result.add "E " & $容器.入口地址
  result.add "C " & $容器.校验和

proc 字形渲染一行*(指令字: 指令; 索引: 整数): string =
  let 极性标 = if 指令字.polarity == 1: "阳" else: "阴"
  let op名 = 操作码名(指令字.opcode)
  let 三维 = "(" & $整数(指令字.address_3d.横) & "," & $整数(指令字.address_3d.纵) & "," & $整数(指令字.address_3d.深) & ")"
  var 行 = "  " & align($索引, 4) & " | " & 指令字.glyph & " | " & align(op名, 6)
  行.add " " & align($指令字.operand, 3)
  行.add " | " & 极性标
  行.add " | " & 三维
  行

#==============================================================
#  执行引擎 — 不依赖外部工具链的端到端执行
#==============================================================

proc 创建执行状态*(容器: 机器码容器): 执行状态 =
  执行状态(容器: 容器, PC: 0, 容器栈: @[], ACC: 0,
            FLAGS: (false, false, false), 步数: 0, 活跃: true)

proc 入栈*(状态: var 执行状态; 值: 整数) =
  状态.容器栈.add(值)

proc 出栈*(状态: var 执行状态): 整数 =
  if 状态.容器栈.high >= 0:
    result = 状态.容器栈.pop()
  else:
    result = 0

proc 读容器值*(状态: 执行状态; 地址: 整数): 整数 =
  let idx = 地址 mod 256
  状态.容器.状态[idx]

proc 写容器值*(状态: var 执行状态; 地址: 整数; 值: 整数) =
  let idx = 地址 mod 256
  状态.容器.状态[idx] = 值

# === 指令周期: 取指 → 解码 → 执行 ===

proc 取指*(状态: 执行状态): 指令 =
  if 状态.PC >= 0 and 状态.PC < 状态.容器.程序.len:
    状态.容器.程序[状态.PC]
  else:
    指令(opcode: HALT, operand: 0, glyph: "Ɑ", polarity: 1, timing: 0)

proc 单步执行*(状态: var 执行状态; 日志: var seq[string]): bool =
  ## 执行单条指令, 返回是否继续
  if not 状态.活跃 or 状态.PC >= 状态.容器.程序.len:
    状态.活跃 = false
    return false

  let 指令字 = 取指(状态)
  let oldPC = 状态.PC
  状态.PC += 1

  case 指令字.opcode
    of NOP:
      日志.add "[" & align($oldPC, 3) & "] NOP     容器空操作"
    
    of LOAD:
      状态.ACC = 读容器值(状态, 指令字.operand)
      日志.add "[" & align($oldPC, 3) & "] LOAD    " & align($指令字.operand, 3) & " → ACC=" & $状态.ACC
    
    of STORE:
      写容器值(状态, 指令字.operand, 状态.ACC)
      日志.add "[" & align($oldPC, 3) & "] STORE   " & align($指令字.operand, 3) & " ← ACC=" & $状态.ACC
    
    of ADD:
      状态.ACC = (状态.ACC + 读容器值(状态, 指令字.operand)) mod 256
      状态.FLAGS = ((状态.ACC == 0), 状态.ACC < 0, 状态.ACC > 127)
      日志.add "[" & align($oldPC, 3) & "] ADD     " & align($指令字.operand, 3) & " → ACC=" & $状态.ACC
    
    of SUB:
      状态.ACC = (状态.ACC - 读容器值(状态, 指令字.operand) + 256) mod 256
      状态.FLAGS = ((状态.ACC == 0), false, 状态.ACC > 127)
      日志.add "[" & align($oldPC, 3) & "] SUB     " & align($指令字.operand, 3) & " → ACC=" & $状态.ACC
    
    of MUL:
      let 被乘 = 读容器值(状态, 指令字.operand)
      状态.ACC = (状态.ACC * 被乘) mod 256
      日志.add "[" & align($oldPC, 3) & "] MUL     " & align($指令字.operand, 3) & " → ACC=" & $状态.ACC
    
    of CMP:
      let diff = 状态.ACC - 读容器值(状态, 指令字.operand)
      状态.FLAGS = ((diff == 0), false, diff > 0)
      日志.add "[" & align($oldPC, 3) & "] CMP     " & align($指令字.operand, 3) & " → 零=" & $状态.FLAGS.零
    
    of JMP:
      状态.PC = 指令字.operand
      日志.add "[" & align($oldPC, 3) & "] JMP     → addr " & $指令字.operand
    
    of JZ:
      if 状态.FLAGS.零:
        状态.PC = 指令字.operand
        日志.add "[" & align($oldPC, 3) & "] JZ      → addr " & $指令字.operand & " (跳转)"
      else:
        日志.add "[" & align($oldPC, 3) & "] JZ      → addr " & $指令字.operand & " (不跳)"
    
    of JNZ:
      if not 状态.FLAGS.零:
        状态.PC = 指令字.operand
        日志.add "[" & align($oldPC, 3) & "] JNZ     → addr " & $指令字.operand & " (跳转)"
      else:
        日志.add "[" & align($oldPC, 3) & "] JNZ     → addr " & $指令字.operand & " (不跳)"
    
    of SEND:
      入栈(状态, 状态.ACC)
      日志.add "[" & align($oldPC, 3) & "] SEND    ACC→栈 (跨壁发送)"
    
    of RECV:
      let v = 出栈(状态)
      写容器值(状态, 指令字.operand, v)
      日志.add "[" & align($oldPC, 3) & "] RECV    栈→addr" & $指令字.operand & " (跨壁接收)"
    
    of GLYPH:
      let 编码 = 指令字.operand
      let byteVal = 推导一字节码(编码)
      状态.ACC = byteVal
      日志.add "[" & align($oldPC, 3) & "] GLYPH   " & align($编码, 3) & " → 字节码=" & $byteVal & " ('" & 指令字.glyph & "')"
    
    of CALL:
      入栈(状态, 状态.PC)
      状态.PC = 指令字.operand
      日志.add "[" & align($oldPC, 3) & "] CALL    addr " & $指令字.operand & " (入栈返回=" & $状态.PC & ")"
    
    of RET:
      let 返回地址 = 出栈(状态)
      状态.PC = 返回地址
      日志.add "[" & align($oldPC, 3) & "] RET     → 返回 addr " & $返回地址
    
    of HALT:
      状态.活跃 = false
      日志.add "[" & align($oldPC, 3) & "] HALT    容器停机"

  状态.步数 += 1
  状态.活跃

proc 运行*(状态: var 执行状态; 日志: var seq[string], 最大步数: 整数 = 1000) =
  while 状态.活跃 and 状态.步数 < 最大步数:
    var line: seq[string] = @[]
    discard 单步执行(状态, line)
    日志.add line[0]

#==============================================================
#  流式执行 — 时间触发的执行入口
#==============================================================

proc 时间切片*(时间: float; 容器: 机器码容器; 时间因子: float = 0.0): seq[string] =
  let phase = 整数(时间 * 8.0) mod max(1, 容器.程序.len)
  result.add "时间 " & formatFloat(时间, ffDecimal, 3) & " 相位 " & $phase & "/" & $容器.程序.len
  if phase < 容器.程序.len:
    let 指令字 = 容器.程序[phase]
    result.add "激活: " & 操作码名(指令字.opcode) & " '" & 指令字.glyph & "' @3D(" & $整数(指令字.address_3d.横) & "," & $整数(指令字.address_3d.纵) & "," & $整数(指令字.address_3d.深) & ")"

#==============================================================
#  示例和报告
#==============================================================

proc 示例计算1到N*(上限: 整数): 机器码容器 =
  ## 计算1到N的累加 (演示: LOAD,ADD,JNZ,HALT)
  var 容器 = 创建容器("sum_loop")
  # 地址0: LOAD counter = N
  var N值 = min(上限, 127)
  容器.程序.add(指令(opcode: LOAD, operand: 0, polarity: 1))
  容器.程序.add(指令(opcode: STORE, operand: 1, polarity: 0))
  容器.程序.add(指令(opcode: LOAD, operand: 2, polarity: 1))
  # 地址3: 循环开始
  # addr 3: ADD counter
  容器.程序.add(指令(opcode: ADD, operand: 0, polarity: 1))
  # addr 4: STORE result
  容器.程序.add(指令(opcode: STORE, operand: 3, polarity: 0))
  # addr 5: LOAD counter
  容器.程序.add(指令(opcode: LOAD, operand: 0, polarity: 1))
  # addr 6: SUB 1 (= address 4 常量)
  容器.程序.add(指令(opcode: SUB, operand: 4, polarity: 0))
  # addr 7: STORE counter
  容器.程序.add(指令(opcode: STORE, operand: 0, polarity: 0))
  # addr 8: JNZ 循环
  容器.程序.add(指令(opcode: JNZ, operand: 3, polarity: 1))
  # addr 9: LOAD result
  容器.程序.add(指令(opcode: LOAD, operand: 3, polarity: 1))
  # addr 10: HALT
  容器.程序.add(指令(opcode: HALT, operand: 0, polarity: 1))
  # 设置数据
  容器.状态[0] = N值
  容器.状态[1] = 0
  容器.状态[2] = N值
  容器.状态[4] = 1  # 常量1
  容器.入口地址 = 0
  容器.总字数 = 容器.程序.len
  编译(容器)
  容器

proc 示例字形投影*(声: 整数; 韵: 整数; 调: 整数): 机器码容器 =
  ## 将声母/韵母/声调组合转成GLYPH指令序列
  var 容器 = 创建容器("glyph_project")
  # 计算编码
  let 编码 = 声 * 30 * 60 + 韵 * 60 + 调
  let byteVal = 推导一字节码(编码)
  容器.程序.add(指令(opcode: GLYPH, operand: 编码 mod 256, polarity: 1))
  容器.程序.add(指令(opcode: STORE, operand: 100, polarity: 0))
  容器.程序.add(指令(opcode: HALT, operand: 0, polarity: 1))
  容器.入口地址 = 0
  容器.总字数 = 容器.程序.len
  编译(容器)
  容器

proc 引擎报告*(容器: 机器码容器; 执行日志: seq[string] = @[]) =
  echo "══════════════════════════════════════════════════════"
  echo "  机 器 码 容 器 引 擎 — 报 告"
  echo "══════════════════════════════════════════════════════"
  echo "总字(指令)数: ", 容器.总字数
  echo "校验和: ", 容器.校验和
  echo "入口地址: ", 容器.入口地址
  echo ""
  echo "─ 程序(机器码↔字形) ─"
  for i in 0..<容器.程序.len:
    echo " ", 字形渲染一行(容器.程序[i], i)
  echo ""
  echo "─ 状态空间(容器=物质) ─"
  var 非零数 = 0
  for i in 0..<min(16, 容器.状态.len):
    if 容器.状态[i] != 0:
      echo "  [" & align($i, 3) & "] = " & $容器.状态[i]
      非零数 += 1
  if 非零数 == 0:
    echo "  (全零)"
  echo ""

  if 执行日志.len > 0:
    echo "─ 执行日志 ─"
    for line in 执行日志:
      echo " ", line
    echo ""

#==============================================================
#  测试运行
#==============================================================

when isMainModule:
  echo "═══ 机器码容器引擎 ═══"
  echo ""

  # 1. 累加器示例: 1+2+...+5
  echo "─ 示例1: 累加器 1+..+5 ─"
  var 累加器 = 示例计算1到N(5)
  var 日志: seq[string] = @[]
  var 状态 = 创建执行状态(累加器)
  运行(状态, 日志, 50)
  引擎报告(累加器, 日志)
  echo "  → 结果(槽3): ", 累加器.状态[3]
  echo ""

  # 2. 字形投影示例
  echo "─ 示例2: 字形投影(声12韵18调3) ─"
  var 字形容器 = 示例字形投影(12, 18, 3)
  var 字形日志: seq[string] = @[]
  var 字形状态 = 创建执行状态(字形容器)
  运行(字形状态, 字形日志, 50)
  引擎报告(字形容器, 字形日志)
  echo "  → 字节码结果(槽100): ", 字形容器.状态[100]
  echo ""

  # 3. 复合容器: 先投影字形, 再乘以极性
  echo "─ 示例3: 复合容器(字形+极性乘法) ─"
  var 复合 = 创建容器("composite")
  # 声=7 韵=3 调=2
  let 编码3 = 7 * 30 * 60 + 3 * 60 + 2
  复合.程序.add(指令(opcode: GLYPH, operand: 编码3 mod 256, polarity: 1))
  复合.程序.add(指令(opcode: STORE, operand: 10, polarity: 1))
  复合.程序.add(指令(opcode: LOAD, operand: 10, polarity: 0))
  # MUL: 阳=1 (极性 × 2)
  复合.程序.add(指令(opcode: MUL, operand: 11, polarity: 1))
  复合.程序.add(指令(opcode: STORE, operand: 12, polarity: 0))
  复合.程序.add(指令(opcode: HALT, operand: 0, polarity: 1))
  复合.状态[10] = 0
  复合.状态[11] = 2
  复合.状态[12] = 0
  复合.入口地址 = 0
  复合.总字数 = 复合.程序.len
  编译(复合)

  var 复合日志: seq[string] = @[]
  var 复合状态 = 创建执行状态(复合)
  运行(复合状态, 复合日志, 50)
  引擎报告(复合, 复合日志)

  echo ""
  echo "─ 容器=物质=时间 验证 ─"
  echo "  容器: 指令槽(容器) + 数据槽(物质) + 时序触发(时间)"
  echo "  每个指令: 一字节码=一字形=一个3D几何坐标=一个时隙"
  echo "  端到端: LOAD→STORE→ADD→GLYPH→MUL→HALT 全链路不依赖外部工具"
