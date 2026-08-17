# 满全法 33韵母表（标准韵母，不带声调）
# 按四呼分组：开口呼/齐齿呼/合口呼/撮口呼/卷舌呼
# 与 yunmu66.nim 配合：33韵母 × 2(大小写) = 66进制

import tables

const STANDARD_YUNMU* = [
  # 开口呼(11)
  "a", "o", "e", "ai", "ei", "ao", "ou",
  "an", "en", "ang", "eng",
  # 齐齿呼(10)
  "i", "ia", "ie", "iao", "iu", "in", "ian", "ing", "iang", "iong",
  # 合口呼(7)
  "u", "ua", "uo", "uai", "uan", "uang", "ong",
  # 撮口呼(4)
  "ü", "üe", "ün", "üan",
  # 卷舌呼(1)
  "er",
]

const STANDARD_COUNT* = STANDARD_YUNMU.len  # = 33

type
  YunmuNode* = object
    name*: string
    idx*: int
    kind*: string    # "标准"
    group*: string   # 四呼分组

  YunmuTable* = object
    name*: string
    nodes*: seq[YunmuNode]
    name_to_idx*: Table[string, int]

func ym_group*(ym: string): string =
  case ym:
  of "a", "o", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ueng":
    return "开口呼"
  of "i", "ia", "ie", "iao", "iu", "in", "ian", "ing", "iang", "iong":
    return "齐齿呼"
  of "u", "ua", "uo", "uai", "un", "uan", "uang", "ong":
    return "合口呼"
  of "ü", "üe", "ün", "üan":
    return "撮口呼"
  of "er":
    return "卷舌呼"
  else:
    return "其他"

func new_yunmu_table*(name: string): YunmuTable =
  result.name = name
  result.nodes = @[]
  result.name_to_idx = initTable[string, int]()
  for i, ym in STANDARD_YUNMU:
    var n: YunmuNode
    n.name = ym
    n.idx = i
    n.kind = "标准"
    n.group = ym_group(ym)
    result.nodes.add(n)
    result.name_to_idx[ym] = i

func group_by*(): seq[tuple[group: string; items: seq[string]]] =
  var groups = @["开口呼", "齐齿呼", "合口呼", "撮口呼", "卷舌呼"]
  var tmp = new_yunmu_table("tmp")
  for g in groups:
    var items: seq[string]
    for n in tmp.nodes:
      if n.group == g:
        items.add(n.name)
    if items.len > 0:
      result.add((g, items))

when is_main_module:
  echo "=== 满全法 33标准韵母 ==="
  var yt = new_yunmu_table("韵母表")
  echo "标准韵母: ", STANDARD_COUNT
  echo "四呼分组: 开口/齐齿/合口/撮口/卷舌"
  echo "---"

  echo "【按韵部分组】"
  for g in group_by():
    echo "  ", g.group, ":"
    var line = "    "
    for item in g.items:
      line &= item & " "
    echo line

  echo ""
  echo "【完整列表】"
  for n in yt.nodes:
    echo "  #", n.idx, " ", n.name, " [", n.group, "]"

  echo ""
  echo "总数:", STANDARD_COUNT, " | 配合 yunmu66 = ", STANDARD_COUNT * 2, "进制"
