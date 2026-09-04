# ============================================================
# 机器码跨 ISA 对齐表 · 类型定义
#
# 规则：语义 ID 是公约数，机器码是公倍数。
#      同一语义在不同 ISA 下映射到不同位域模板，但共享唯一 ID。
#      位域以模板形式存入，bit 偏移固定。
#
# 对齐规则：语义 ID → 各 ISA 位域模板 → 汇编文本 → 机器码字节序
# ============================================================

type
  ISA* = enum
    isaRISCV32
    isaRISCV64
    isaARM64
    isaX86_64
    isaMIPS32

  FieldFormat* = enum
    ffRegDst    # 目的寄存器
    ffRegSrc1   # 源寄存器 1
    ffRegSrc2   # 源寄存器 2
    ffImmI      # I-型立即数
    ffImmS      # S-型立即数
    ffImmB      # B-型立即数
    ffImmU      # U-型立即数
    ffImmJ      # J-型立即数
    ffShamt     # 移位量
    ffFunct3    # 3 位功能码
    ffFunct7    # 7 位功能码
    ffOpcode    # 操作码
    ffSuffix    # x86 前缀/后缀

  Field* = object
    name*: string
    format*: FieldFormat
    hi*: int      # bit offset (high)
    lo*: int      # bit offset (low)

  InsnEncoding* = object
    semId*: string          # 语义 ID（跨 ISA 唯一）
    mnemonic*: string       # 汇编助记符模板
    isa*: ISA
    byteLen*: int           # 机器码字节长度
    fields*: seq[Field]     # 位域表

  FontMCODEntry* = object
    semId*: string
    encoding*: InsnEncoding
    glyph*: uint32          # 私用区码点占位（待分配）
    description*: cstring
