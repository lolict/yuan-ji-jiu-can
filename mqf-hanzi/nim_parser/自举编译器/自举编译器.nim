## 自举编译器
##
##  读取域规则.txt → 一次性生成索引文件 → 运行时寻址不搜索
##
##  运行：nim c --cc:clang -r 自举编译器.nim

import std/tables
import std/strutils
import std/os

# =================================================================
#  类型
# =================================================================
type
  声母条目* = object
    声母*: string
    态*: string
    极性*: string
    值*: int

  域索引* = object
    ## 一次性构建的编译期索引
    字根*: Table[string, int]
    声母*: seq[声母条目]
    极性锚点*: Table[string, string]
    qmf*: Table[string, string]
    物理公理*: Table[string, string]
    声调层*: Table[string, string]
    容器字段*: seq[string]
    流动作*: Table[string, string]

# =================================================================
#  解析域规则文本
# =================================================================
proc 解析文本*(文本: string): OrderedTable[string, OrderedTable[string, string]] =
  var 当前段 = ""
  for 原始行 in 文本.splitLines:
    let 行 = 原始行.strip
    if 行.len == 0 or 行[0] == '#': continue

    if 行.len > 3 and 行[0] == '[' and 行[1] == '@' and 行[^1] == ']':
      当前段 = 行[2 .. ^2]
      if not result.hasKey(当前段):
        result[当前段] = initOrderedTable[string, string]()
    elif 当前段.len > 0 and '=' in 行:
      let 分隔 = 行.find('=')
      let 键 = 行[0 ..< 分隔].strip
      let 值 = 行[分隔+1 .. ^1].strip
      result[当前段][键] = 值

# =================================================================
#  一次性构建索引
# =================================================================
proc 构建索引*(段: OrderedTable[string, OrderedTable[string, string]]): 域索引 =
  if 段.hasKey("字根表"):
    for k, v in 段["字根表"].pairs:
      result.字根[k] = v.parseInt

  if 段.hasKey("声母映射"):
    for k, v in 段["声母映射"].pairs:
      let 部分 = v.split('|')
      if 部分.len >= 3:
        result.声母.add(声母条目(
          声母: k, 态: 部分[0], 极性: 部分[1], 值: 部分[2].parseInt
        ))

  if 段.hasKey("极性锚点"):
    for k, v in 段["极性锚点"].pairs:
      result.极性锚点[k] = v

  if 段.hasKey("qmf体素"):
    for k, v in 段["qmf体素"].pairs:
      result.qmf[k] = v

  if 段.hasKey("物理公理"):
    for k, v in 段["物理公理"].pairs:
      result.物理公理[k] = v

  if 段.hasKey("声调层"):
    for k, v in 段["声调层"].pairs:
      result.声调层[k] = v

  if 段.hasKey("容器字段"):
    for k in 段["容器字段"].keys:
      result.容器字段.add(k)

  if 段.hasKey("流动作"):
    for k, v in 段["流动作"].pairs:
      result.流动作[k] = v

# =================================================================
#  生成域索引文件（运行时期产物）
# =================================================================
proc 生成域索引文件*(索引: 域索引, 输出路径: string): int =
  var 行列表: seq[string]
  行列表.add "# ==========================================="
  行列表.add "#  满全法 · 域索引（自举编译器生成）"
  行列表.add "#  一次性构建 · 跨会话复用"
  行列表.add "# ==========================================="
  行列表.add ""

  # 字根
  行列表.add "[字根]"
  for k, v in 索引.字根.pairs:
    行列表.add k & "=" & $v

  # 声母
  行列表.add ""
  行列表.add "[声母]"
  for e in 索引.声母:
    行列表.add e.声母 & "=" & e.态 & "|" & e.极性 & "|" & $e.值

  # 极性
  行列表.add ""
  行列表.add "[极性]"
  for k, v in 索引.极性锚点.pairs:
    行列表.add k & "=" & v

  # QMF
  行列表.add ""
  行列表.add "[QMF]"
  for k, v in 索引.qmf.pairs:
    行列表.add k & "=" & v

  # 公理
  行列表.add ""
  行列表.add "[公理]"
  for k, v in 索引.物理公理.pairs:
    行列表.add k & "=" & v

  # 声调
  行列表.add ""
  行列表.add "[声调]"
  for k, v in 索引.声调层.pairs:
    行列表.add k & "=" & v

  # 流动作
  行列表.add ""
  行列表.add "[流动作]"
  for k, v in 索引.流动作.pairs:
    行列表.add k & "=" & v

  # 字段
  行列表.add ""
  行列表.add "[字段]"
  for f in 索引.容器字段:
    行列表.add f

  let 文本 = 行列表.join("\n") & "\n"
  输出路径.writeFile(文本)
  result = 行列表.len

# =================================================================
#  生成 Nim 索引模块（编译进运行时）
# =================================================================
proc 生成Nim索引模块*(索引: 域索引, 输出路径: string): int =
  var 行列表: seq[string]
  行列表.add "## 自举编译器生成的索引模块（不要手动修改）"
  行列表.add "## 运行期直接寻址 · 零搜索"
  行列表.add ""
  行列表.add "const 索引指纹* = \"MQF.selfboot." & $索引.声母.len & "." & $索引.字根.len & "\""

  # 声母常量表
  行列表.add ""
  行列表.add "const"
  行列表.add "  声母表* = ["
  let 声数 = 索引.声母.len
  for i in 0..<声数:
    let e = 索引.声母[i]
    let 尾 = if i < 声数 - 1: "," else: ""
    行列表.add "    (\"" & e.声母 & "\", \"" & e.态 & "\", \"" & e.极性 & "\", " & $e.值 & ")" & 尾
  行列表.add "  ]"

  let 文本 = 行列表.join("\n") & "\n"
  输出路径.writeFile(文本)
  result = 行列表.len

# =================================================================
#  主流程
# =================================================================
when isMainModule:
  let 源文件 = currentSourcePath().parentDir / "域规则.txt"
  let 产物路径 = currentSourcePath().parentDir / "域索引.txt"
  let 模块路径 = currentSourcePath().parentDir / "索引.nim"

  echo "========================================"
  echo "  自举编译器启动"
  echo "  源: " & 源文件
  echo "========================================"

  if not 源文件.fileExists:
    echo "[失败] 域规则.txt 不存在。请先确认文件路径。"
    quit(1)

  let 源文本 = 源文件.readFile
  let 段 = 解析文本(源文本)
  let 索引 = 构建索引(段)

  echo ""
  echo "▸ 解析段数: " & $段.len
  var 名: seq[string]
  for k in 段.keys:
    名.add(k)
  echo "  段名: " & 名.join(", ")

  echo ""
  echo "▸ 索引状态:"
  echo "    字根条目: " & $索引.字根.len
  echo "    声母条目: " & $索引.声母.len
  echo "    极性锚点: " & $索引.极性锚点.len
  echo "    QMF参数:  " & $索引.qmf.len
  echo "    物理公理: " & $索引.物理公理.len
  echo "    声调层:   " & $索引.声调层.len
  echo "    流动作:   " & $索引.流动作.len

  let 行产物 = 生成域索引文件(索引, 产物路径)
  echo ""
  echo "▸ 域索引文件生成: " & 产物路径 & " (" & $行产物 & " 行)"

  let 行模块 = 生成Nim索引模块(索引, 模块路径)
  echo "▸ Nim索引模块生成: " & 模块路径 & " (" & $行模块 & " 行)"

  echo ""
  echo "========================================"
  echo "  自举编译完成"
  echo "========================================"
  echo "  索引指纹: MQF.selfboot." & $索引.声母.len & "." & $索引.字根.len
