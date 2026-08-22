## ═══════════════════════════════════════════════════════════════
## cross_compile.nim — 跨容器范畴编译桥
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, math]

type
  ContainerProfile* = object
    id*: string
    base*: int
    kind*: string

  CompileRoute* = object
    source_id*: string
    target_id*: string
    path*: seq[string]
    compile_method*: string
    ratio*: string
    confidence*: float

const CONTAINER_PROFILES* = [
  ("FQ-TONE60", 60, "音调"),
  ("FQ-ATONE50", 100, "音调"),
  ("FQ-YM-a", 28, "韵母"),
  ("FQ-YM-o", 28, "韵母"),
  ("FQ-YM-e", 26, "韵母"),
  ("FQ-YM-i", 26, "韵母"),
  ("FQ-YM-u", 26, "韵母"),
  ("FQ-YM-ü", 7, "韵母"),
  ("FQ-YM-CMPD", 49, "复韵母"),
  ("FQ-SM48", 48, "声母"),
  ("FQ-SM50P", 50, "声母"),
  ("FQ-NASAL", 85, "鼻韵母"),
  ("Clock-60", 60, "时钟"),
  ("Color-42", 42, "颜色"),
  ("Space-18", 18, "空间"),
  ("Mirror-10", 10, "镜像"),
  ("Music-3", 3, "音乐"),
  ("FQ-CLONE", 11, "分身"),
  ("FQ-SPARE1", 70, "备用"),
  ("FQ-SPARE2", 29, "备用"),
  ("FQ-SPARE3", 31, "备用"),
  ("FQ-SPCON", 60, "辅音"),
  ("FQ-YM100", 100, "韵母"),
  ("FQ-YM66", 66, "韵母"),
]

proc gcd(a, b: int): int =
  var x = a
  var y = b
  while y != 0:
    let t = y
    y = x mod y
    x = t
  result = x

proc findProfile(id: string): ContainerProfile =
  for p in CONTAINER_PROFILES:
    if p[0] == id:
      return ContainerProfile(id: p[0], base: p[1], kind: p[2])
  return ContainerProfile(id: "UNKNOWN", base: 0, kind: "UNKNOWN")

proc compile*(id_a, id_b: string): CompileRoute =
  let pa = findProfile(id_a)
  let pb = findProfile(id_b)
  result.source_id = id_a
  result.target_id = id_b
  result.path = @[id_a, id_b]

  let g = gcd(pa.base, pb.base)

  if pa.id == pb.id:
    result.compile_method = "自体"
    result.confidence = 1.0
    result.ratio = "同容器 · 无需编译"
  elif g == pa.base:
    result.compile_method = "直接"
    result.confidence = 0.9
    result.ratio = $pa.base & "整除" & $pb.base & " → " & id_a & "是" & id_b & "的从体"
  elif g == pb.base:
    result.compile_method = "直接"
    result.confidence = 0.9
    result.ratio = $pb.base & "整除" & $pa.base & " → " & id_b & "是" & id_a & "的从体"
  elif g > 1:
    result.compile_method = "直接"
    result.confidence = 0.7
    result.ratio = "gcd(" & $pa.base & "," & $pb.base & ")=" & $g & " → 可约化编译"
  else:
    # 寻找桥接节点
    var bridges: seq[(string, int)] = @[]
    for p in CONTAINER_PROFILES:
      if p[0] == id_a or p[0] == id_b: continue
      let g1 = gcd(pa.base, p[1])
      let g2 = gcd(p[1], pb.base)
      if g1 > 1 and g2 > 1:
        bridges.add((p[0], p[1]))
    if bridges.len > 0:
      result.compile_method = "桥接"
      result.path = @[id_a, bridges[0][0], id_b]
      result.confidence = 0.5
      result.ratio = "桥接容器" & bridges[0][0] & "(base=" & $bridges[0][1] & ")"
    else:
      result.compile_method = "辐射"
      result.confidence = 0.1
      result.ratio = "无公约数 · 无桥接节点 · 辐射兜底"

proc printRoute*(r: CompileRoute): string =
  result = r.source_id & " → " & r.target_id &
    "  方法=" & r.compile_method &
    "  路径=" & r.path.join("→") &
    "  置信=" & $r.confidence &
    "  比率=" & r.ratio

when isMainModule:
  echo "═══ 跨容器范畴编译桥 ═══"
  echo ""

  let test_pairs = @[
    ("FQ-YM-a", "FQ-YM66"),
    ("FQ-TONE60", "Clock-60"),
    ("FQ-SM48", "FQ-SM50P"),
    ("FQ-YM100", "Color-42"),
    ("Mirror-10", "Music-3"),
    ("FQ-YM-a", "FQ-YM-i"),
    ("FQ-TONE60", "FQ-ATONE50"),
    ("FQ-NASAL", "Clock-60"),
    ("FQ-YM100", "FQ-SPARE1"),
    ("Space-18", "Mirror-10"),
  ]
  for (a, b) in test_pairs:
    echo printRoute(compile(a, b))
    echo ""
