## ═══════════════════════════════════════════════════════════════
## compile_demo.nim — 编译实战演示
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils]

type
  CompileStep* = object
    step*: int
    action*: string
    detail*: string

  CompileSession* = object
    id*: string
    title*: string
    source*: string
    target*: string
    steps*: seq[CompileStep]
    success*: bool
    summary_text*: string

proc makeStep(n: int, act: string, det: string): CompileStep =
  result.step = n
  result.action = act
  result.detail = det

proc directDemo*(): CompileSession =
  result.id = "D1"
  result.title = "直接编译: SM50P.v → SM48"
  result.source = "FQ-SM50P"
  result.target = "FQ-SM48"
  result.success = true
  result.steps = @[
    makeStep(1, "检测公约数", "gcd(50,48)=2 > 1 → 直接编译"),
    makeStep(2, "映射", "v(位49) → 49 mod 48=1 → b(位1) · v保留独立"),
    makeStep(3, "验证", "SM48无v → v是独立声母补充"),
  ]
  result.summary_text = "v无法映射到SM48 → v作为独立声母 · SM50=SM48+v/V"

proc bridgeDemo*(): CompileSession =
  result.id = "D2"
  result.title = "桥接编译: YM100.ạ → Color-42"
  result.source = "FQ-YM100"
  result.target = "Color-42"
  result.success = true
  result.steps = @[
    makeStep(1, "直接路径", "gcd(100,42)=2 → 存在直接路径"),
    makeStep(2, "同形匹配", "ạ=下点韵母 → Color-42[0]=ạ(同形) → 置信0.95"),
    makeStep(3, "mod路径", "ạ(位9) → 9 mod 42=9 → Color-42[9]=ị → 置信0.3"),
    makeStep(4, "修正", "同形 > mod → 选用同形映射"),
  ]
  result.summary_text = "ạ→ạ同形映射 · 下点=红色系 · 语义一致"

proc radiateDemo*(): CompileSession =
  result.id = "D3"
  result.title = "辐射编译: Mirror-10 → Music-3"
  result.source = "Mirror-10"
  result.target = "Music-3"
  result.success = true
  result.steps = @[
    makeStep(1, "直接检测", "gcd(10,3)=1 → 无直接路径"),
    makeStep(2, "搜索桥接", "镜像↔TONE60: gcd=10>1; TONE60↔Music-3: gcd=3>1"),
    makeStep(3, "桥接路径", "Mirror-10 → TONE60 → Music-3"),
    makeStep(4, "映射", "节拍≈音调分层 · 拍子映射"),
  ]
  result.summary_text = "Mirror-10→TONE60→Music-3 · 辐射桥接 · 置信0.6"

proc compoundDemo*(): CompileSession =
  result.id = "D4"
  result.title = "复合编译: 声母+韵母 → 时钟"
  result.source = "FQ-SM50P+FQ-YM100"
  result.target = "Clock-60"
  result.success = true
  result.steps = @[
    makeStep(1, "声母提取", "SM50P.b(位0) → 声母指数=0"),
    makeStep(2, "韵母提取", "YM100.a(位0) → 韵母指数=0"),
    makeStep(3, "时钟定位", "Clock-60(a=0) → 进度=0/60"),
    makeStep(4, "计算地址", "(0×0×5 + 0 + 0) mod 60 = 0 → 时钟#0"),
  ]
  result.summary_text = "b+a+ā=0 → 时钟容器#a · 复合编译O(1)"

proc allDemos*(): seq[CompileSession] =
  result = @[directDemo(), bridgeDemo(), radiateDemo(), compoundDemo()]

proc printStep*(s: CompileStep): string =
  result = "  步骤" & $s.step & ": " & s.action & " — " & s.detail

proc printSession*(s: CompileSession): string =
  result = "── " & s.id & " ──  " & s.title & "\n"
  for st in s.steps:
    result &= printStep(st) & "\n"
  result &= "  结果: " & (if s.success: "✓ " else: "✗ ") & s.summary_text & "\n"

when isMainModule:
  echo "═══ 跨容器编译实战 ═══"
  echo ""
  for d in allDemos():
    echo printSession(d)
    echo ""