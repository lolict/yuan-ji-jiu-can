## ═══════════════════════════════════════════════════════════════
## container_evolution_engine.nim — 容器进化引擎
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 容器可以从容器中诞生 · 系统可以自我生长
## 两个父容器 → 交配 → 子容器 · 不等人大·不必人定义
##
## 四种进化操作:
## 1. crossover: 符号交叉 · 子容器base=gcd(父A,父B) · 符号从两父各取一部分
## 2. mutation: 单容器突变 · 子容器base=父base±1 · 随机替换若干符号
## 3. fusion: 融合 · 子容器base=lcm(父A,父B) · 符号取并集
## 4. spawn: 分裂 · 子容器base=父base÷2 · 取代父容器一半符号
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, math]

type
  EvoContainer* = object
    id*: string
    name*: string
    base*: int
    kind*: string         # 音调/韵母/声母/分身/备用/复韵母/鼻韵母/辅音/进化
    gen*: int             # 进化代数
    parents*: seq[string]
    children*: seq[string]
    base_history*: seq[int]  # 历代base值
    op*: string            # 产生方式: 原始/crossover/mutation/fusion/spawn

  EvolutionEngine* = object
    containers*: seq[EvoContainer]

# ═══════════════════════════════════════════════════════════════
## 进化操作
## ═══════════════════════════════════════════════════════════════

proc gcd*(a, b: int): int =
  var x = a; var y = b
  while y != 0:
    let t = y; y = x mod y; x = t
  result = x

proc lcm*(a, b: int): int =
  result = a * b div gcd(a, b)

proc crossover*(c1, c2: EvoContainer, id: string): EvoContainer =
  result.id = id
  result.name = "交叉[" & c1.id & "×" & c2.id & "]"
  result.base = gcd(c1.base, c2.base)
  result.kind = "进化"
  result.gen = max(c1.gen, c2.gen) + 1
  result.parents = @[c1.id, c2.id]
  result.base_history = c1.base_history & c2.base_history & @[result.base]
  result.op = "crossover(交叉)"

proc mutation*(c1: EvoContainer, id: string, delta: int = 1): EvoContainer =
  result.id = id
  result.name = "突变[" & c1.id & "Δ" & $delta & "]"
  result.base = max(2, c1.base + delta)
  result.kind = "进化"
  result.gen = c1.gen + 1
  result.parents = @[c1.id]
  result.base_history = c1.base_history & @[result.base]
  result.op = "mutation(突变)"

proc fusion*(c1, c2: EvoContainer, id: string): EvoContainer =
  result.id = id
  result.name = "融合[" & c1.id & "+" & c2.id & "]"
  result.base = lcm(c1.base, c2.base)
  result.kind = "进化"
  result.gen = max(c1.gen, c2.gen) + 1
  result.parents = @[c1.id, c2.id]
  result.base_history = c1.base_history & c2.base_history & @[result.base]
  result.op = "fusion(融合)"

proc spawn*(c1: EvoContainer, id: string): EvoContainer =
  result.id = id
  result.name = "分裂[" & c1.id & "→" & $(c1.base div 2) & "]"
  result.base = max(2, c1.base div 2)
  result.kind = "进化"
  result.gen = c1.gen + 1
  result.parents = @[c1.id]
  result.base_history = c1.base_history & @[result.base]
  result.op = "spawn(分裂)"

# ═══════════════════════════════════════════════════════════════
## 初始化引擎: 从22容器中提取原始容器
## ═══════════════════════════════════════════════════════════════

proc initEngine*(): EvolutionEngine =
  # 第0代: 22个原始容器
  const raw = [
    ("FQ-TONE60", 60, "音调", @[60]),
    ("FQ-ATONE50", 100, "音调", @[100]),
    ("FQ-YM-a", 28, "韵母", @[28]),
    ("FQ-YM-o", 28, "韵母", @[28]),
    ("FQ-YM-e", 26, "韵母", @[26]),
    ("FQ-YM-i", 26, "韵母", @[26]),
    ("FQ-YM-u", 26, "韵母", @[26]),
    ("FQ-YM-ü", 7, "韵母", @[7]),
    ("FQ-YM-CMPD", 49, "复韵母", @[49]),
    ("FQ-SM48", 48, "声母", @[48]),
    ("FQ-SM50P", 50, "声母", @[50]),
    ("FQ-NASAL", 85, "鼻韵母", @[85]),
    ("FQ-CLONE", 11, "分身", @[11]),
    ("Clock-60", 60, "时钟", @[60]),
    ("Color-42", 42, "颜色", @[42]),
    ("Space-18", 18, "空间", @[18]),
    ("Mirror-10", 10, "镜像", @[10]),
    ("Music-3", 3, "音乐", @[3]),
    ("FQ-SPARE1", 70, "备用", @[70]),
    ("FQ-SPARE2", 29, "备用", @[29]),
    ("FQ-SPARE3", 31, "备用", @[31]),
    ("FQ-SPCON", 60, "辅音", @[60]),
  ]
  for (id, base, kind, bh) in raw:
    result.containers.add(EvoContainer(
      id: id, name: id, base: base, kind: kind,
      gen: 0, parents: @[], children: @[],
      base_history: bh, op: "原始",
    ))
  
  # ═══════════════════════════════════════════════════════════════
## 进化: 生成第1代子容器
## ═══════════════════════════════════════════════════════════════

proc evolve*(engine: var EvolutionEngine): seq[EvoContainer] =
  let originals = engine.containers.filter(proc(c: EvoContainer): bool = c.gen == 0)
  var new_containers: seq[EvoContainer] = @[]
  var child_id = 1

  for i in 0 ..< originals.len:
    let c1 = originals[i]

    # 突变: 每个原始容器产生一个突变版
    new_containers.add(c1.mutation("EVO-mut-" & $child_id))
    child_id += 1

    # 交叉: 每对相邻容器产生一个交叉版
    if i < originals.high:
      let c2 = originals[i+1]
      new_containers.add(c1.crossover(c2, "EVO-cro-" & $child_id))
      child_id += 1

    # 融合: 每个容器和自己融合 (扩展)
    if i mod 3 == 0:
      new_containers.add(c1.fusion(c1, "EVO-fus-" & $child_id))
      child_id += 1

    # 分裂: 大于10的容器分裂
    if c1.base > 10:
      new_containers.add(c1.spawn("EVO-spw-" & $child_id))
      child_id += 1

  # 更新父子关系
  for nc in new_containers:
    for i in 0 ..< engine.containers.len:
      for pid in nc.parents:
        if engine.containers[i].id == pid:
          engine.containers[i].children.add(nc.id)
    engine.containers.add(nc)
  return new_containers

# ═══════════════════════════════════════════════════════════════
## 报告
## ═══════════════════════════════════════════════════════════════

proc `$`*(c: EvoContainer): string =
  result = c.id & "(" & $c.base & "进制," & "G" & $c.gen & "," & c.op & ")"

proc printLineage*(engine: EvolutionEngine): string =
  var lines: seq[string] = @[]
  lines.add("═══ 容器进化树 ═══")
  lines.add("")

  var remaining = engine.containers.toSeq
  var current_gen = 0
  while remaining.len > 0:
    var gen_str = "第" & $current_gen & "代: "
    var gen_containers: seq[EvoContainer] = @[]
    for c in remaining:
      if c.gen == current_gen:
        gen_containers.add(c)

    if gen_containers.len > 0:
      lines.add("── " & gen_str & "──")
      for c in gen_containers:
        lines.add("  " & $c & " ← " & c.parents.join("+") &
          "  base历史: " & c.base_history.map(proc(x:int): string = $x).join("→"))
      lines.add("")

    remaining = remaining.filter(proc(c: EvoContainer): bool = c.gen != current_gen)
    current_gen += 1

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  var engine = initEngine()
  echo "原始容器数: " & $engine.containers.len
  echo ""

  let children1 = engine.evolve()
  echo "第1代: " & $children1.len & " 子容器诞生"
  echo "总容器数: " & $engine.containers.len
  echo ""

  let children2 = engine.evolve()
  echo "第2代: " & $children2.len & " 子容器诞生"
  echo "总容器数: " & $engine.containers.len
  echo ""

  echo printLineage(engine)