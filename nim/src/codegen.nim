# codegen.nim — 兵工厂代码生成器
# Nim 作为工具工厂，为 hunyuan-os 生成标准化模块和编码表

import std/[strutils, os, times, strformat, math]

const
  SYMBOL_COUNT* = 64
  COLOR_COUNT* = 42
  LAYER_COUNT* = 6
  COL_COUNT* = 7

# ── 色名表 ──
const COLOR_NAMES = [
  "妃", "缃", "绛", "彤", "紫", "玄", "黟",        # Layer 0 (R0-R6)
  "繠", "縹", "絳", "紺", "黝", "緇", "黻",        # Layer 1
  "霙", "霓", "霞", "彤", "靛", "蒼", "黝",        # Layer 2
  "冰", "霧", "霜", "雪", "霰", "雹", "霪",        # Layer 3
  "皛", "皚", "皙", "皓", "皤", "黔", "黯",        # Layer 4
  "縞", "練", "素", "緗", "縹", "紗", "繭"         # Layer 5
]

# ── 模块模板生成器 ──
proc genModuleTemplate(name: string): string =
  let content = &"""// {name}.zig — 自动生成模块
// 由 Nim 兵工厂 codegen.nim 生成于 {$now()}

const std = @import("std");
const ColorQuant = @import("color_quant.zig");

pub const MODULE: u32 = 0x{(name.len.uint32) and 0xFF:02X};

// === 核心结构 ===
pub const State = struct {{
    idx: u8 = 0,
    layer: u8 = 0,
    col: u8 = 0,
    value: f32 = 0.0,
}};

// === 方法 ===
pub fn init(idx: u8) State {{
    return .{{ .idx = idx, .layer = idx / 7, .col = idx % 7 }};
}}

// === 测试 ===
test "{name}_basic" {{
    const s = init(0);
    try std.testing.expectEqual(@as(u8, 0), s.idx);
}}
"""
  return content

# ── 编码表批量生成 ──
proc genBindingTable(): string =
  result = "// 64符→42色 循环绑定表 (预计算, comptime 零开销)\n"
  result &= "pub const bindings: [SYMBOL_COUNT]u8 = [\n"
  for i in 0..<SYMBOL_COUNT:
    let color_idx = i mod COLOR_COUNT
    if i > 0: result &= "    "
    result &= &"{$color_idx},"
    if (i + 1) mod 8 == 0:
      result &= &"  // #{(i-6)}-#{(i+1)}\n"
    else:
      result &= " "
  if SYMBOL_COUNT mod 8 != 0:
    result &= "\n"
  result &= "];\n\n"

  # 符→层 映射表
  result &= "// 符→层 映射\n"
  result &= "pub const sym2layer: [SYMBOL_COUNT]u8 = [\n"
  for i in 0..<SYMBOL_COUNT:
    let layer = i div COL_COUNT
    result &= &"{$layer},"
    if (i + 1) mod 16 == 0: result &= &"  // #{(i-15)}-#{(i+1)}\n"
    else: result &= " "
  result &= "];\n"

# ── 管辖关系矩阵生成 ──
proc genGovernanceMatrix(): string =
  result = "// 层级管辖矩阵: governance[宽层, 窄层] = 指数衰减因子\n"
  result &= "// 窄层(加法) 管辖 宽层(乘法), 距离越远衰减越大\n"
  result &= "pub const governance: [LAYER_COUNT, LAYER_COUNT]f32 = [\n"
  for wide in 0..<LAYER_COUNT:
    result &= "    ["
    for narrow in 0..<LAYER_COUNT:
      let dist = if wide >= narrow: wide - narrow else: narrow - wide
      let factor = if wide >= narrow: 1.0 / pow(2.0, float(dist)) else: 0.0
      result &= &"{factor:.4}f"
      if narrow < LAYER_COUNT - 1: result &= ", "
    result &= "]"
    if wide < LAYER_COUNT - 1: result &= ","
    result &= &"  // {COLOR_NAMES[wide*7]}层\n"
  result &= "];\n"

# ── 色名→索引 查找表 ──
proc genColorNameTable(): string =
  result = "// 色名→索引 正向查找\n"
  result &= "pub const color_names: [COLOR_COUNT][]const u8 = [\n"
  for i, name in COLOR_NAMES:
    result &= &"    \"{name}\""
    if i < COLOR_COUNT - 1: result &= ","
    if i mod 7 == 6: result &= &"  // Layer {i div 7}\n"
    else: result &= "\n"
  result &= "];\n"

# ── 命令行入口 ──
proc main() =
  let args = commandLineParams()

  if args.len == 0:
    echo "Nim 兵工厂 codegen — 用法:"
    echo "  nim gen <module_name>   — 生成 Zig 模块模板"
    echo "  nim table               — 生成编码绑定表"
    echo "  nim matrix              — 生成管辖矩阵"
    echo "  nim names               — 生成色名表"
    echo "  nim all                 — 生成全部到 zig/gen/"
    return

  let cmd = args[0]

  if cmd == "gen" and args.len >= 2:
    let name = args[1]
    let code = genModuleTemplate(name)
    echo code
    echo &"\n  // 模块 {name} 模板已生成 ({code.len} 字节)"

  elif cmd == "table":
    echo genBindingTable()
    echo &"\n  // 64→42 绑定表已生成"

  elif cmd == "matrix":
    echo genGovernanceMatrix()
    echo &"\n  // 管辖矩阵已生成"

  elif cmd == "names":
    echo genColorNameTable()
    echo &"\n  // 色名表已生成"

  elif cmd == "all":
    let gen_dir = "../zig/gen"
    if not dirExists(gen_dir):
      createDir(gen_dir)
    writeFile(&"{gen_dir}/bindings.zig", genBindingTable())
    writeFile(&"{gen_dir}/governance.zig", genGovernanceMatrix())
    writeFile(&"{gen_dir}/color_names.zig", genColorNameTable())
    echo &"  // 全部已生成到 {gen_dir}/"

  else:
    echo "  未知命令: ", cmd


when isMainModule:
  main()
