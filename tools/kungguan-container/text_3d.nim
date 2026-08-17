# 满全法·三维空间文本定位与显示容器
# 在三维空间中定位、输入、显示文字
# 支持任意方向显示（上下左右前后 + 斜角）
# 不占用内存：只存储位置和方向，内容引用外部大数据

import tables

type
  # ============================================================
  # 显示方向（6基本 + 4斜角 + 8天地角 = 18方向）
  # ============================================================
  DisplayDir* = enum
    ddRight       # 0 右（东）→ 水平向右
    ddLeft        # 1 左（西）→ 水平向左
    ddDown        # 2 下（地）→ 垂直向下
    ddUp          # 3 上（天）→ 垂直向上
    ddFront       # 4 前（南）→ 向内/向前
    ddBack        # 5 后（北）→ 向外/向后
    ddRightDown   # 6 右下（东南）→ 斜向右下
    ddLeftDown    # 7 左下（西南）→ 斜向左下
    ddRightUp     # 8 右上（东北）→ 斜向右上
    ddLeftUp      # 9 左上（西北）→ 斜向左上
    ddHeavenRight # 10 天右（天东）→ 斜向天右
    ddHeavenLeft  # 11 天左（天西）→ 斜向天左
    ddHeavenFront # 12 天前（天南）→ 斜向天前
    ddHeavenBack  # 13 天后（天北）→ 斜向天后
    ddEarthRight  # 14 地右（地东）→ 斜向右下地
    ddEarthLeft   # 15 地左（地西）→ 斜向左下地
    ddEarthFront  # 16 地前（地南）→ 斜向前下地
    ddEarthBack   # 17 地后（地北）→ 斜向后下地

  # ============================================================
  # 显示方法
  # ============================================================
  DisplayMethod* = enum
    dmNormal      # 0 正常显示
    dmBold        # 1 粗体
    dmItalic      # 2 斜体
    dmBlink       # 3 闪烁
    dmReverse     # 4 反色
    dmUnderline   # 5 下划线
    dmStrikethru  # 6 删除线
    dmGlow        # 7 发光
    dmShadow      # 8 阴影
    dmMirror      # 9 镜像

  # ============================================================
  # 三维空间位置
  # ============================================================
  Vec3* = object
    x*: int     # 东西
    y*: int     # 南北
    z*: int     # 天地

  # ============================================================
  # 文本锚点（在三维空间中的一个文字位置）
  # ============================================================
  TextAnchor* = object
    pos*: Vec3              # 三维位置
    dir*: DisplayDir        # 显示方向
    style*: DisplayMethod   # 显示方法
    content_ref*: int        # 内容引用（指向外部大数据表）
    color*: int             # 颜色索引（引用42色容器）
    size*: int              # 大小（1-7 级）

  # ============================================================
  # 输入缓冲区（临时存储输入内容）
  # ============================================================
  InputBuffer* = object
    cursor*: Vec3           # 当前光标位置
    anchors*: seq[TextAnchor]  # 已放置的文本锚点
    buffer*: string         # 当前输入缓冲

# ============================================================
# 方向 → 单位向量（用于计算下一个位置）
# ============================================================

func dir_to_vec*(d: DisplayDir): Vec3 =
    case d
    of ddRight:       result = Vec3(x: 1, y: 0, z: 0)
    of ddLeft:        result = Vec3(x: -1, y: 0, z: 0)
    of ddDown:        result = Vec3(x: 0, y: 0, z: -1)
    of ddUp:          result = Vec3(x: 0, y: 0, z: 1)
    of ddFront:       result = Vec3(x: 0, y: 1, z: 0)
    of ddBack:        result = Vec3(x: 0, y: -1, z: 0)
    of ddRightDown:   result = Vec3(x: 1, y: 0, z: -1)
    of ddLeftDown:    result = Vec3(x: -1, y: 0, z: -1)
    of ddRightUp:     result = Vec3(x: 1, y: 0, z: 1)
    of ddLeftUp:      result = Vec3(x: -1, y: 0, z: 1)
    of ddHeavenRight: result = Vec3(x: 1, y: 0, z: 2)
    of ddHeavenLeft:  result = Vec3(x: -1, y: 0, z: 2)
    of ddHeavenFront: result = Vec3(x: 0, y: 1, z: 2)
    of ddHeavenBack:  result = Vec3(x: 0, y: -1, z: 2)
    of ddEarthRight:  result = Vec3(x: 1, y: 0, z: -2)
    of ddEarthLeft:   result = Vec3(x: -1, y: 0, z: -2)
    of ddEarthFront:  result = Vec3(x: 0, y: 1, z: -2)
    of ddEarthBack:   result = Vec3(x: 0, y: -1, z: -2)

# ============================================================
# 位置加法（沿方向移动）
# ============================================================

func `+`*(a, b: Vec3): Vec3 =
  result = Vec3(x: a.x + b.x, y: a.y + b.y, z: a.z + b.z)

func `$`*(v: Vec3): string = "(" & $v.x & "," & $v.y & "," & $v.z & ")"
func dir_name*(d: DisplayDir): string = $ord(d)
func method_name*(d: DisplayMethod): string = $ord(d)

# ============================================================
# 输入缓冲区操作
# ============================================================

proc new_input_buffer*(): InputBuffer =
  result.cursor = Vec3(x: 0, y: 0, z: 0)
  result.anchors = @[]
  result.buffer = ""

# 在当前位置插入文本
proc insert_text*(buf: var InputBuffer, text: string, dir: DisplayDir, style: DisplayMethod, color: int) =
  let anchor = TextAnchor(
    pos: buf.cursor,
    dir: dir,
    style: style,
    content_ref: buf.anchors.len,
    color: color,
    size: 4
  )
  buf.anchors.add(anchor)
  buf.buffer &= text

# 沿方向移动光标
proc move_cursor*(buf: var InputBuffer, dir: DisplayDir, steps: int = 1) =
  let v = dir_to_vec(dir)
  buf.cursor = Vec3(
    x: buf.cursor.x + v.x * steps,
    y: buf.cursor.y + v.y * steps,
    z: buf.cursor.z + v.z * steps
  )

# 直接定位光标到指定位置
proc goto*(buf: var InputBuffer, x, y, z: int) =
  buf.cursor = Vec3(x: x, y: y, z: z)

# 获取所有锚点
func anchors*(buf: InputBuffer): seq[TextAnchor] = buf.anchors

# 锚点占用比特统计
func anchor_bits*(): int =
  # 位置: 3×8 = 24 bits (每个坐标8 bits)
  # 方向: 5 bits (18种方向)
  # 方法: 4 bits (10种方法)
  # 颜色: 6 bits (42色)
  # 大小: 3 bits (7级)
  # 引用: 16 bits (内容引用)
  result = 24 + 5 + 4 + 6 + 3 + 16

when is_main_module:
  echo "=== 三维空间文本定位与显示容器 ==="
  echo ""

  var buf = new_input_buffer()

  # 在中心位置输入"满全法"
  insert_text(buf, "满", ddRight, dmNormal, 1)
  move_cursor(buf, ddRight, 1)
  insert_text(buf, "全", ddRight, dmBold, 13)
  move_cursor(buf, ddRight, 1)
  insert_text(buf, "法", ddRight, dmGlow, 36)

  # 向上显示
  goto(buf, 0, 0, 0)
  move_cursor(buf, ddUp, 3)
  insert_text(buf, "↑向上", ddUp, dmItalic, 18)

  # 斜向右下
  goto(buf, 0, 0, 0)
  insert_text(buf, "↘斜下", ddRightDown, dmShadow, 30)

  # 向天右
  goto(buf, 0, 0, 0)
  move_cursor(buf, ddHeavenRight, 2)
  insert_text(buf, "天右斜", ddHeavenRight, dmMirror, 40)

  echo "【已放置的文本锚点】"
  for i, a in buf.anchors:
    echo "  #", i, " ", a.pos, " ", dir_name(a.dir), " 颜色#", a.color, " 样式:", method_name(a.style)

  echo ""
  echo "【光标最终位置】"
  echo "  ", buf.cursor

  echo ""
  echo "【锚点占用】"
  echo "  每个锚点: ", anchor_bits(), " bits = ", anchor_bits() / 8, " bytes"
  echo "  总锚点数: ", buf.anchors.len
  echo "  总占用: ", buf.anchors.len * anchor_bits(), " bits"

  echo ""
  echo "【方向向量示例】"
  echo "  右: ", dir_to_vec(ddRight)
  echo "  上: ", dir_to_vec(ddUp)
  echo "  斜右下: ", dir_to_vec(ddRightDown)
  echo "  天右: ", dir_to_vec(ddHeavenRight)
