## 满全法 - 三维字体文件结构
## Container -> Voxel -> Binary .mqf Font File
##
## 字体文件 = 容器集群的三维投影记录
## 每个字 = 一组容器的体素集合
## 笔画顺序 = 容器写入的先后顺序 = 地址
##
## 字体文件结构（二进制）：
##   +----------+
##   | 文件头   | 魔数+版本+容器总数+体素尺寸
##   +----------+
##   | 容器索引 | 每个容器的地址/相位/位置
##   +----------+
##   | 体素表   | 每个字的3D像素矩阵
##   +----------+
##   | 笔画序   | 每个字的笔画寻址序列
##   +----------+
##   | 声调层   | 每个字的声调容器编码
##   +----------+

import sequtils
import strutils

type
  体素* = object
    亮度* : int
    源容器* : int
    激活* : bool

  体素网格* = object
    X* : int
    Y* : int
    Z* : int
    体素* : seq[seq[seq[体素]]]

  字体头* = object
    魔数* : string
    版本* : int
    容器总数* : int
    字符总数* : int
    网格X* : int
    网格Y* : int
    网格Z* : int

  容器索引项* = object
    容器地址* : int
    相位* : int
    空间位置* : int
    值* : int
    层级* : int
    锚点极性* : int

  字符项* = object
    字* : string
    笔画序* : seq[int]
    声调编码* : int
    相位* : int
    亮度值* : int

  字体文件* = ref object
    头* : 字体头
    容器索查* : seq[容器索引项]
    字符集* : seq[字符项]
    网格* : 体素网格

# ============================================================================
# 创建与基础操作
# ============================================================================

proc 创建字体文件*(网格X, 网格Y, 网格Z: int): 字体文件 =
  new(result)
  result.头 = 字体头(
    魔数: "MQF",
    版本: 0x0100,
    容器总数: 0,
    字符总数: 0,
    网格X: 网格X,
    网格Y: 网格Y,
    网格Z: 网格Z
  )
  result.容器索查 = @[]
  result.字符集 = @[]
  result.网格 = 体素网格(X: 网格X, Y: 网格Y, Z: 网格Z)
  result.网格.体素 = newSeqWith(网格Z, newSeqWith(网格Y, newSeqWith(网格X, 体素(亮度: 0, 源容器: -1, 激活: false))))

proc 亮度映射*(值: int): int =
  if 值 <= 0: return 0
  elif 值 >= 255: return 255
  else: return 值

proc 写入容器索引*(字体: var 字体文件, 地址, 相位, 空间位置, 值, 层级, 锚点极性: int) =
  var x = 容器索引项(
    容器地址: 地址, 相位: 相位, 空间位置: 空间位置,
    值: 值, 层级: 层级, 锚点极性: 锚点极性
  )
  字体.容器索查.add(x)
  字体.头.容器总数 = 字体.容器索查.len

proc 添加字符*(字体: var 字体文件, 字: string, 笔画序: seq[int], 声调编码, 相位, 容器值: int) =
  var ch = 字符项(字: 字, 笔画序: 笔画序, 声调编码: 声调编码, 相位: 相位, 亮度值: 亮度映射(容器值))
  字体.字符集.add(ch)
  字体.头.字符总数 = 字体.字符集.len

proc 投影容器到体素*(字体: var 字体文件, 容器地址, 容器值, 相位: int) =
  let 亮度 = 亮度映射(容器值)
  let z层 = min(字体.网格.Z - 1, max(0, 相位 div 3))
  let baseX = 1 + (容器地址 * 3) mod max(1, (字体.网格.X - 3))
  let baseY = 1 + (容器地址 * 2) mod max(1, (字体.网格.Y - 3))
  for dy in 0..2:
    for dx in 0..2:
      let px = baseX + dx
      let py = baseY + dy
      if px >= 0 and px < 字体.网格.X and py >= 0 and py < 字体.网格.Y:
        字体.网格.体素[z层][py][px] = 体素(亮度: 亮度, 源容器: 容器地址, 激活: true)

proc 取切片*(字体: 字体文件, z层: int): seq[string] =
  result = @[]
  if z层 < 0 or z层 >= 字体.网格.Z: return
  for y in 0..<字体.网格.Y:
    var row = ""
    for x in 0..<字体.网格.X:
      let v = 字体.网格.体素[z层][y][x]
      if v.激活:
        row.add(if v.亮度 > 200: "█" elif v.亮度 > 150: "▓" elif v.亮度 > 100: "▒" elif v.亮度 > 50: "░" else: "·")
      else:
        row.add(" ")
    result.add(row)

# ============================================================================
# 笔画寻址
# ============================================================================

proc 笔画到地址*(笔画序列: seq[string]): seq[int] =
  for bihua in 笔画序列:
    case bihua:
      of "横":  result.add(1)
      of "竖":  result.add(2)
      of "撇":  result.add(3)
      of "捺":  result.add(4)
      of "点":  result.add(4)
      of "折":  result.add(5)
      else:     result.add(6)

proc 容器寻址*(容器地址: int, 相位: int, 笔画数: int): int =
  result = 容器地址 * 64 + 相位 + 笔画数

# ============================================================================
# 主程序
# ============================================================================
when isMainModule:
  echo "========================================="
  echo " MQF 3D Font File Structure"
  echo " Container -> Voxel -> Binary"
  echo "========================================="
  echo ""

  var font = 创建字体文件(48, 48, 9)

  let 基础容器 = [
    (0, "bit_2",   2,  3, 1),
    (1, "bit_4",   4,  4, 0),
    (2, "bit_8",   8,  5, 1),
    (3, "bit_16",  16,  6, 0),
    (4, "mlll_64", 64,  4, 1),
    (5, "tone_100",100, 7, 0),
    (6, "yun_32",  32,  8, 1),
    (7, "phase_9",  9,  4, 0)
  ]

  echo "--- Container Index ---"
  for c in 基础容器:
    font.写入容器索引(c[0], c[3], c[0] * 10, c[2], c[3], c[4])
    echo "  #" & $c[0] & " | " & c[1].alignLeft(10) & " | val=" & align($c[2], 3) &
        " | phase=" & $c[3] & " | anchor=" & $c[4]

  echo ""
  echo "--- Glyph Definitions (stroke sequence = address sequence) ---"

  let glyphs = [
    ("\xE6\x88\x91", @["横", "竖", "横折", "横", "竖钩", "撇", "点"], 5, 200),
    ("\xE5\x88\x98", @["点", "横", "撇", "竖钩"],                              5, 180),
    ("\xE6\xA5\x9A", @["横", "竖", "撇", "点", "横", "竖", "横向左", "捺"],         5, 160),
    ("\xE6\x81\xAC", @["点", "点", "提", "撇", "横折", "横", "横", "竖"],          5, 150)
  ]

  for g in glyphs:
    let zi = g[0]
    let strokes = g[1]
    let tone_id = g[2]
    let brightness = g[3]
    let addr_seq = 笔画到地址(strokes)
    let phase = 容器寻址(tone_id, 4, addr_seq.len)
    font.添加字符(zi, addr_seq, tone_id, phase mod 9, brightness)
    echo "  '" & zi & "' strokes=" & $strokes.len &
        " addr=[" & addr_seq.mapIt($it).join(",") &
        "] tone@" & $tone_id & " phase=" & $(phase mod 9)

  echo ""
  echo "--- Voxel Projection ---"
  for c in 基础容器:
    font.投影容器到体素(c[0], c[2], c[3])
    echo "  c#" & $c[0] & " -> Z" & $(c[3] div 3) & " brightness=" & $亮度映射(c[2])

  echo ""
  echo "--- Slice Z=1 (Phase 0-2) ---"
  let sl = font.取切片(1)
  for y in 0..<min(sl.len, 24):
    echo "  |" & sl[y] & "|"

  echo ""
  echo "--- File Header ---"
  echo "  magic: " & font.头.魔数
  echo "  version: 0x" & $toHex(font.头.版本, 4)
  echo "  containers: " & $font.头.容器总数
  echo "  glyphs: " & $font.头.字符总数
  echo "  grid: " & $font.头.网格X & "x" & $font.头.网格Y & "x" & $font.头.网格Z

  echo ""
  echo "========================================="
  echo " MQF 3D Font Structure OK"
  echo "========================================="

# 三维字体文件结构结束
