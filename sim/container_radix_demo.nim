## ═══════════════════════════════════════════════════════════════
## container_radix_demo.nim — 多进制约定的零维映射验证
## ═══════════════════════════════════════════════════════════════
##
## 核心: 汉字的部首(base50) × 结构(base6) × 笔画序列(base5^n) = 唯一零维ID
## 验证: 复杂事物能否通过多个进制容器精确映射到唯一一个点
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm]

# ═══════════════════════════════════════════════════════════════
## 四个独立进制容器
# ═══════════════════════════════════════════════════════════════

const
  # 容器1: 笔画 (base 5)
  BASE_STROKE* = 5

  # 容器2: 部首 (base 50,简化)
  BASE_RADICAL* = 50

  # 容器3: 结构 (base 6)
  BASE_STRUCT* = 6

  # 容器4: 位置 (base 4)
  BASE_POS* = 4

# ═══════════════════════════════════════════════════════════════
## 查询表: 符号 → 编码
# ═══════════════════════════════════════════════════════════════

const RADICAL_MAP* = [
  ("日", 1), ("月", 2), ("人", 3), ("水", 4), ("木", 5),
  ("土", 6), ("火", 7), ("口", 8), ("手", 9), ("心", 10),
]

const STRUCT_MAP* = [
  ("左右", 1), ("上下", 2), ("左包围", 3),
  ("右包围", 4), ("全包围", 5), ("独体", 6),
]

const STROKE_MAP* = [
  ("横", 1), ("竖", 2), ("撇", 3), ("捺", 4), ("折", 5),
]

# ═══════════════════════════════════════════════════════════════
## 编码函数: 从多个容器查询 → 计算零维ID
# ═══════════════════════════════════════════════════════════════

proc lookupRadical(key: string): int =
  for (k, v) in RADICAL_MAP:
    if k == key: return v
  return 0

proc lookupStruct(key: string): int =
  for (k, v) in STRUCT_MAP:
    if k == key: return v
  return 0

proc lookupStroke(key: string): int =
  for (k, v) in STROKE_MAP:
    if k == key: return v
  return 0

proc encodeCharacter*(radical: string, structure: string, strokes: seq[string]): int =
  ## 零维ID = 部首 × 结构基数 + 笔画序列编码
  let radical_id = lookupRadical(radical)
  let struct_id = lookupStruct(structure)

  # 笔画序列 (base 5 → 十进制)
  var stroke_val = 0
  for i, s in strokes:
    let v = lookupStroke(s)
    stroke_val = stroke_val * BASE_STROKE + v

  # 组合: 部首×结构基数 + 笔画偏移
  result = (radical_id * STRUCT_MAP.len) + struct_id + stroke_val

# ═══════════════════════════════════════════════════════════════
## 解码函数: 从零维ID → 还原多个容器
# ═══════════════════════════════════════════════════════════════

proc decodeCharacter*(id: int): (string, string, seq[string]) =
  let struct_base = STRUCT_MAP.len
  let radical_part = id div struct_base
  let remainder = id mod struct_base

  let radical_id = radical_part mod BASE_RADICAL
  let stroke_code = radical_part div BASE_RADICAL
  let struct_id = remainder

  # 反查
  var radical = "unknown"
  for (k, v) in RADICAL_MAP:
    if v == radical_id:
      radical = k; break

  var structure = "unknown"
  for (k, v) in STRUCT_MAP:
    if v == struct_id:
      structure = k; break

  # 笔画解码 (base5数列)
  var strokes: seq[string] = @[]
  var n = stroke_code
  while n > 0:
    strokes.add($(n mod BASE_STROKE))
    n = n div BASE_STROKE
  strokes.reverse()

  result = (radical, structure, strokes)

# ═══════════════════════════════════════════════════════════════
## 验证: 不同汉字是否有唯一ID
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 容器进制验证: 多进制 → 零维映射 ═══"
  echo ""

  # 例1: 明 (日+月, 左右结构, 日4画+月4画)
  let id_ming = encodeCharacter("日", "左右", @["横","竖","横折","横","横","横","竖","横折","横"])
  echo "明 = " & $id_ming

  # 例2: 休 (人+木, 左右, 人2画+木4画)
  let id_xiu = encodeCharacter("人", "左右", @["撇","捺","横","竖","撇","捺"])
  echo "休 = " & $id_xiu

  # 例3: 林 (木+木, 左右, 木4画+木4画)
  let id_lin = encodeCharacter("木", "左右", @["横","竖","撇","捺","横","竖","撇","捺"])
  echo "林 = " & $id_lin

  echo ""

  # 解码验证
  let (r1, s1, k1) = decodeCharacter(id_ming)
  echo "解码" & $id_ming & ": 部首=" & r1 & " 结构=" & s1 & " 笔画=" & k1.join("")

  let (r2, s2, k2) = decodeCharacter(id_xiu)
  echo "解码" & $id_xiu & ": 部首=" & r2 & " 结构=" & s2 & " 笔画=" & k2.join("")

  echo ""
  echo "结论: " & (
    if id_ming != id_xiu and id_xiu != id_lin:
      "✓ 三个不同汉字有唯一零维ID → 进制容器成功"
    else:
      "✗ 存在冲突 → 需要调整基数"
  )
