# ============================================================
# 字体容器 MCOD 表协议定义
#
# 字体作为容器：私有码点 + 自定义表 + 位域模板
# 遵循 OpenType SFNT 容器格式（容器化打包）
# ============================================================

import mc_types

const
  MCOD_MAGIC* = 0x4D434F44u32   # 'MCOD'
  MCOD_VERSION* = 0x0001u16     # 协议版本 1

type
  MCODEntryV1* = object
    SemId*: array[16, char]     # 语义 ID（C 字符串）
    ISAId*: uint8               # ISA 枚举值
    ByteLen*: uint8             # 机器码字节长度
    FieldCount*: uint8          # 位域条目数
    Glyph*: uint32              # 对应字形码点

  MCODTableV1* = object
    Magic*: uint32              # 'MCOD'
    Version*: uint16
    EntryCount*: uint16
    Reserved*: uint32
    ## 变长：EntryCount * MCODEntryV1

  FontContainerSpec* = object
    ## 字体容器规范描述（不依赖运行时）
    familyName*: string
    unitsPerEm*: uint16
    codepointBase*: uint32      # 私用区起始码点（建议 U+E000）
    mcodOffset*: uint32         # MCOD 表在 .ttf 中的文件偏移
    mcodLength*: uint32

const DEFAULT_FONT_SPEC*: FontContainerSpec = FontContainerSpec(
  familyName: "ContainerMachineCode",
  unitsPerEm: 1024,
  codepointBase: 0xE000,
  mcodOffset: 0,
  mcodLength: 0,
)
