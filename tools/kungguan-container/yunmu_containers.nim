## ═══════════════════════════════════════════════════════════════
## 集装箱 · 0 · 1 · 2 · 5
## ═══════════════════════════════════════════════════════════════
## 集装箱就是盒子。名字就是名字。
##   39韵母删掉aoeiuv → 剩33韵母配对66符号（装入主系统）
##   多出来的（aoeiuv及其变音）→ 装进这些盒子
##   装什么随便你，不满意可以把东西取出来换掉
## ═══════════════════════════════════════════════════════════════

import tables

# ═══ 集装箱0 ═══════════════════════════════════════════════════
# 名字: 0
# 里面装什么: 随便
# ═══════════════════════════════════════════════════════════════
const CONTAINER_0* = [
  "ạ", "ẹ", "ị", "ọ", "ụ",     # 下点
  "ä", "ë", "ï", "ö", "ü",     # 分音
]

# ═══ 集装箱1 ═══════════════════════════════════════════════════
# 名字: 1
# ═══════════════════════════════════════════════════════════════
const CONTAINER_1* = [
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ",     # 倒短音
  "a̋", "e̋", "i̋", "ő", "ű",     # 双重音
]

# ═══ 集装箱2 ═══════════════════════════════════════════════════
# 名字: 2
# ═══════════════════════════════════════════════════════════════
const CONTAINER_2* = [
  "å", "e̊", "i̊", "o̊", "ů",     # 上圈
  "ą", "ę", "į", "ǫ", "ų",     # 反尾
]

# ═══ 集装箱5 ═══════════════════════════════════════════════════
# 名字: 5
# ═══════════════════════════════════════════════════════════════
const CONTAINER_5* = [
  "ã", "ẽ", "ĩ", "õ", "ũ",     # 波浪
  "â", "ê", "î", "ô", "û",     # 帽子
  "ă", "ĕ", "ĭ", "ŏ", "ŭ",     # 短音
  "ȁ", "ȅ", "ȉ", "ȍ", "ȕ",     # 反钝音
]

# ═══ 集装箱容器 ═══════════════════════════════════════════════
type
  XiangZi* = object
    name*: string        # 集装箱名字: "0" "1" "2" "5"
    items*: seq[string]  # 里面装的东西

  XiangZiSystem* = object
    boxes*: seq[XiangZi]

func new_xiangzi_system*(): XiangZiSystem =
  result.boxes = @[]

  var b0: XiangZi
  b0.name = "0"
  b0.items = @CONTAINER_0
  result.boxes.add(b0)

  var b1: XiangZi
  b1.name = "1"
  b1.items = @CONTAINER_1
  result.boxes.add(b1)

  var b2: XiangZi
  b2.name = "2"
  b2.items = @CONTAINER_2
  result.boxes.add(b2)

  var b5: XiangZi
  b5.name = "5"
  b5.items = @CONTAINER_5
  result.boxes.add(b5)

func get_box*(sys: XiangZiSystem, name: string): XiangZi =
  for b in sys.boxes:
    if b.name == name: return b
  return XiangZi()

proc show*(sys: XiangZiSystem) =
  for b in sys.boxes:
    var line = "  集装箱 " & b.name & " ["
    for s in b.items:
      line &= s
    line &= "]"
    echo line

when is_main_module:
  echo "═══ 集装箱系统 ═══"
  let sys = new_xiangzi_system()
  sys.show()
  echo ""
  echo "总数:"
  for b in sys.boxes:
    echo "  集装箱", b.name, ": ", b.items.len, "个符号"
