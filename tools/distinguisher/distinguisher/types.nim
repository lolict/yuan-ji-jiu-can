# ============================================================
# 分别器 (Distinguisher) · 类型定义
#
# 三层路由：
#   第一层：感知家族输入 → 视觉族
#   第二层：谐音靶点发音 → 靶点对象
#   第三层：主权路由靶点身份 → 主权级别
#
# 核心存在理由：系统没有分别器时，无法区分对象/目标，
# 路由到错误的靶点 = 对方不承认 = 流断裂。
# ============================================================

import tables
import options

# ── 第一层：感知家族 ──
type GlyphFamily* = enum
  containerGlyph   # 方形/圆形容器 → 刘楚恬（0=容器）
  phallicGlyph     # 竖长/界线 → 我（1=被包含者）
  coupleGlyph      # 夫妻图腾
  homophoneBridge  # 谐音桥（专用路由）
  unknownGlyph     # 未知家族

# ── 第二层：谐音靶点 ──
type SoundTarget* = enum
  wife       # 毛利兰→莫刘连理 / lctfrqmy
  husband    # 工藤新一→恋初 / wgslz
  sweet      # mlct/mltt/ml.tt → Wife
  soundMorph # gtt / gtt谐音扩展
  cbsyAddr   # cbsy 索引地址（声母+韵母+声调夫妻共同体操作）
  sleepRes   # 休眠资源（可被唤醒）
  external   # 外部参与者
  system     # 系统级

# ── 第三层：主权级别 ──
type Sovereignty* = enum
  active    # 完全权限（夫妻）
  passive   # 只读权限（外部参与者）
  sleeping  # 可被唤醒（休眠资源）
  system    # 无个人主权（系统级）

# ── 路由结果 ──
type RouteResult* = object
  input*: string              # 原始输入
  family*: GlyphFamily        # 第一层结果
  target*: SoundTarget        # 第二层结果
  sovereignty*: Sovereignty   # 第三层结果
  confidence*: float          # 置信度 0..1
  path*: seq[string]          # 经过的节点
  ackable*: bool              # 是否可被承认（闭环条件）

# ── 路由记录 ──
type RouteRecord* = object
  result*: RouteResult
  timestamp*: int
  success*: bool

# ── 分别器状态 ──
type Distinguisher* = object
  # 第一层：字符 → 家族 映射表
  family_map*: Table[string, GlyphFamily]
  # 第二层：谐音 → 靶点 映射表
  sound_map*: Table[string, SoundTarget]
  # 第三层：靶点 → 主权 映射表
  sovereignty_map*: Table[SoundTarget, Sovereignty]
  # 路由历史
  history*: seq[RouteRecord]
  # 统计
  total_routed*: int
  total_acked*: int
  total_broken*: int
  # 映射表大小（手动维护，因为Table.len不可用）
  family_count*: int
  sound_count*: int
  sovereignty_count*: int
