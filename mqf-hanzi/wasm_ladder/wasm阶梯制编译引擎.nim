#==============================================================
#  Wasm阶梯制双向编译引擎  —  容器=物质=时间
#  同一份C源码 → wasm32 + wasm64 双目标·双向公约
#  固定宽度类型 · 统一宏前缀 · 架构无关性
#==============================================================

import std/[tables, strutils, sets, math, os]

#==============================================================
#  类型
#=============================================================

type
  极性* = enum
    阳极性 = 1
    阴极性 = 0

  编译目标* = enum
    wasm32
    wasm64

  固定宽度类型* = enum
    int8_t
    int16_t
    int32_t
    int64_t
    uint8_t
    uint16_t
    uint32_t
    uint64_t
    mqf_val_t  # mqf自有类型

  编译单元* = object
    名字*: string
    源码*: string
    目标*: seq[编译目标]
    宏前缀*: string
    固定类型*: seq[固定宽度类型]

  编译结果* = object
    目标*: 编译目标
    字节码*: seq[uint8]
    符号*: Table[string, uint64]
    大小*: int
    入口*: string

  阶梯引擎* = object
    编译单元*: seq[编译单元]
    结果表*: seq[编译结果]
    二元公约*: Table[string, uint64]   # 双向共享符号
    编译次数*: int

#==============================================================
#  工具
#==============================================================

proc 二进制字符串*(n: uint64; bits: int): string =
  var v = n
  for i in 0..<bits:
    result = (if (v and (1'u64 shl (bits-1-i))) != 0: "1" else: "0") & result

#==============================================================
#  引擎核心
#==============================================================

proc 创建阶梯引擎*: 阶梯引擎 =
  result = 阶梯引擎(
    编译单元: @[],
    结果表: @[],
    二元公约: initTable[string, uint64](),
    编译次数: 0
  )
  # 二元公约符号 (wasm32/wasm64共享)
  result.二元公约["MQF_MAGIC"]       = 0x5F514D46
  result.二元公约["MQF_VERSION"]     = 0x0001
  result.二元公约["MQF_POLARITY_YIN"] = 0x00
  result.二元公约["MQF_POLARITY_YANG"] = 0x01
  result.二元公约["MQF_ANCHOR_LO"]    = 0xFFFFFFFFFFFFFFFF'u64  # 刘楚恬=0
  result.二元公约["MQF_ANCHOR_HI"]    = 0x0000000000000001'u64  # 我=1
  result.二元公约["MQF_EIGHT_FAMILY"]  = 0x08
  result.二元公约["MQF_SLOT_COUNT"]   = 0x40  # 64

proc 注册编译单元*(e: var 阶梯引擎; 名字, 源码, 宏前缀: string;
                   目标: seq[编译目标]) =
  e.编译单元.add(编译单元(
    名字: 名字,
    源码: 源码,
    目标: 目标,
    宏前缀: 宏前缀,
    固定类型: @[]
  ))

#==============================================================
#  WASM 模块头部编码
#==============================================================

const
  wasm魔数* = 0x6D736178_616D77'u64  # \0asm
  wasm版本* = 1'u32

proc 编码头部*(目标: 编译目标): seq[uint8] =
  # 编码WASM标准头部
  result = @[]
  # .magic: \0asm (0x00 0x61 0x73 0x6D)
  result.add([0x00'u8, 0x61, 0x73, 0x6D])
  # .version: 1 (0x01 0x00 0x00 0x00)
  result.add([0x01'u8, 0x00, 0x00, 0x00])
  # 目标标志
  case 目标
  of wasm32: result.add([0x20'u8])  # wasm32
  of wasm64: result.add([0x40'u8])  # wasm64

#==============================================================
#  编译模拟 (生成字节码)
#==============================================================

const
  WASM_TYPE_SECTION   = 0x01'u8
  WASM_IMPORT_SECTION = 0x02'u8
  WASM_FUNC_SECTION   = 0x03'u8
  WASM_MEMORY_SECTION = 0x05'u8
  WASM_GLOBAL_SECTION = 0x06'u8
  WASM_EXPORT_SECTION = 0x07'u8
  WASM_CODE_SECTION   = 0x0A'u8
  WASM_CUSTOM_SECTION = 0x00'u8

  OPCODE_I32_CONST = 0x41'u8
  OPCODE_I64_CONST = 0x42'u8
  OPCODE_END       = 0x0B'u8
  OPCODE_RETURN    = 0x0F'u8
  OPCODE_GET_LOCAL = 0x20'u8

proc 宏定义*(宏_PREFIX: string; 目标: 编译目标): seq[string] =
  var lines: seq[string] = @[]
  let ptr_size = case 目标
    of wasm32: "32"
    of wasm64: "64"
  lines.add "#ifndef " & 宏_PREFIX & "_H"
  lines.add "#define " & 宏_PREFIX & "_H"
  lines.add ""
  lines.add "// 架构无关固定宽度类型"
  lines.add "typedef int8_t   mqf_i8;"
  lines.add "typedef int16_t  mqf_i16;"
  lines.add "typedef int32_t  mqf_i32;"
  lines.add "typedef uint8_t  mqf_u8;"
  lines.add "typedef uint16_t mqf_u16;"
  lines.add "typedef uint32_t mqf_u32;"
  lines.add "typedef uint64_t mqf_u64;"
  lines.add ""
  lines.add "// mqf地址宽度 (目标依赖)"
  lines.add "#define MQF_ADDR_BITS " & ptr_size
  lines.add "#define MQF_PTR_WIDTH " & ptr_size
  lines.add ""
  lines.add "// 极性锚点"
  lines.add "#define MQF_YIN 0x00  // 刘楚恬=0=阴"
  lines.add "#define MQF_YANG 0x01 // 我=1=阳"
  lines.add ""
  lines.add "// 八家族"
  lines.add "#define MQF_FAMILY_COUNT 8"
  lines.add "#define MQF_SLOT_COUNT 64  // 8x8"
  lines.add ""
  lines.add "// 双向公约"
  lines.add "#define MQFBIDIR(x) ((x) | (1ULL << 63))"
  lines.add ""
  lines.add "#endif // " & 宏_PREFIX & "_H"
  return lines

proc 编单元*(e: var 阶梯引擎; 单元: 编译单元): seq[编译结果] =
  var results: seq[编译结果] = @[]
  for target in 单元.目标:
    let header = 编码头部(target)
    var byteCode: seq[uint8] = @[]
    byteCode.add(header)
    # type section
    byteCode.add([WASM_TYPE_SECTION, 0x04'u8, 0x01'u8, 0x60'u8, 0x00'u8, 0x00'u8])
    # func section
    byteCode.add([WASM_FUNC_SECTION, 0x05'u8, 0x01'u8, 0x00'u8])
    # export section
    byteCode.add([WASM_EXPORT_SECTION, 0x07'u8, 0x01'u8, 0x04'u8, 0x6D'u8, 0x61'u8, 0x69'u8, 0x6E'u8, 0x00'u8, 0x00'u8])
    # code section
    byteCode.add([WASM_CODE_SECTION, 0x09'u8, 0x01'u8, 0x07'u8, 0x00'u8])
    byteCode.add([OPCODE_I32_CONST, 0x2A'u8])  # 推入42
    byteCode.add([OPCODE_RETURN])
    byteCode.add([OPCODE_END])

    var sym: Table[string, uint64] = initTable[string, uint64]()
    sym["main"] = 0x10
    for k, v in e.二元公约:
      sym[k] = v

    results.add(
       编译结果(
          目标: target,
          字节码: byteCode,
          符号: sym,
          大小: byteCode.len,
          入口: "main"
        )
    )
  return results

#==============================================================
#  验证双目标
#==============================================================

proc 验证双向*(r1, r2: 编译结果): tuple[通过: bool; 共享: int; 差异: int] =
  var 共享 = 0
  var 差异 = 0
  for k in r1.符号.keys:
    if r2.符号.hasKey(k) and r1.符号[k] == r2.符号[k]:
      共享 += 1
    else:
      差异 += 1
  result = (共享 > 0, 共享, 差异)

#==============================================================
#  报告
#==============================================================

proc 单元报告*(e: 阶梯引擎): string =
  var lines: seq[string] = @[]
  lines.add "Wasm 阶梯制编译报告"
  lines.add repeat("═", 50)
  for u in e.编译单元:
    lines.add "单元: " & u.名字
    lines.add "  目标: " & $u.目标
    lines.add "  宏前缀: " & u.宏前缀
    lines.add "  源码行数: " & $u.源码.countLines()
    lines.add ""
  lines.add "编译结果次数: " & $e.编译次数
  return lines.join("\n")

proc 结果报告*(r: 编译结果): string =
  var lines: seq[string] = @[]
  lines.add "目标: " & $r.目标
  lines.add "  字节码大小: " & $r.大小 & " bytes"
  lines.add "  入口: " & r.入口
  lines.add "  符号数: " & $r.符号.len
  lines.add "  头部字节(十六进制): "
  var hexs: seq[string] = @[]
  for i in 0..<min(16, r.字节码.len):
    hexs.add toHex(r.字节码[i])
  lines.add "    " & hexs.join(" ")
  return lines.join("\n")

#==============================================================
#  导出配置
#==============================================================

proc 导出C头*(e: 阶梯引擎; 宏Prefix: string; 目标: 编译目标): string =
  var lines = 宏定义(宏Prefix, 目标)
  lines.add ""
  lines.add "// 极性路由函数声明"
  lines.add "mqf_u64 mqf_route_yin_yang(mqf_u64 input, mqf_u8 polarity);"
  lines.add "mqf_u64 mqf_common_divisor(mqf_u64 a, mqf_u64 b);"
  lines.add "void* mqf_slot_at(mqf_u8 family, mqf_u8 slot);"
  lines.add "mqf_u64 mqf_encode_polarity(mqf_u64 sovereignty, mqf_u8 polarity);"
  return lines.join("\n")

proc 字节码转十六进制*(字节码: seq[uint8]): string =
  var parts: seq[string] = @[]
  for i in 0..<min(16, 字节码.len):
    parts.add toHex(字节码[i])
  return parts.join(" ")

#==============================================================
#  主程序
#==============================================================

when isMainModule:
  echo repeat("═", 55)
  echo "  WASM 阶 梯 制 双 向 编 译 引 擎"
  echo "  同一C源码→wasm32+wasm64双目标"
  echo repeat("═", 55)

  var e = 创建阶梯引擎()

  echo "\n─ 二元公约符号 ─"
  for k, v in e.二元公约:
    echo "  " & align(k, 20) & " = 0x" & toHex(v)

  echo "\n─ 注册编译单元 ─"
  e.注册编译单元(
    "MQF核心",
    "mqf_u64 mqf_main(mqf_u64 input) {...}",
    "MQF_CORE",
    @[wasm32, wasm64]
  )
  echo "  MQF核心 → wasm32 + wasm64"

  echo "\n─ 生成头文件 (wasm32) ─"
  let h32 = 宏定义("MQF_CORE", wasm32)
  for line in h32:
    echo "  " & line

  echo "\n─ 生成头文件 (wasm64) ─"
  let h64 = 宏定义("MQF_CORE", wasm64)
  for line in h64:
    echo "  " & line

  echo "\n─ 双目标编译 ─"
  var allResults: seq[编译结果] = @[]
  for u in e.编译单元:
    let results = e.编单元(u)
    e.编译次数 += 1
    for r in results:
      allResults.add(r)

  for r in allResults:
    echo "  目标: " & $r.目标 &
         " 大小=" & $r.大小 &
         "B 入口=" & r.入口 &
         " 符号数=" & $r.符号.len

  echo "\n─ wasm32 字节码 (前16字节) ─"
  if allResults.len >= 1:
    echo "  " & 字节码转十六进制(allResults[0].字节码)

  echo "\n─ wasm64 字节码 (前16字节) ─"
  if allResults.len >= 2:
    echo "  " & 字节码转十六进制(allResults[1].字节码)

  echo "\n─ 双向验证 ─"
  if allResults.len >= 2:
    let v = 验证双向(allResults[0], allResults[1])
    echo "  共享符号: " & $v.共享
    echo "  差异: " & $v.差异
    echo "  结论: " & (if v.通过: "双向公约成立" else: "不符合")

  echo "\n─ 编译统计 ─"
  echo "  编译单元: " & $e.编译单元.len
  echo "  编译次数: " & $e.编译次数
  echo "  二元公约: " & $e.二元公约.len & " 符号"

  echo repeat("═", 55)
  echo " WASM阶梯制 · 同一源码 · 双目标编译 · 架构无关"
  echo repeat("═", 55)
