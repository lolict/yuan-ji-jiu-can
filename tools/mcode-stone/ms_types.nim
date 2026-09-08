## ms_types — 通用类型定义（不依赖 ISA）
##
## Operands 是语义层的"配方"，与各 ISA 无关。
## encode 接受 Operands + ISA → 机器码字节数组
## decode 接受机器码字节数组 + ISA → Operands

type
  ## 通用操作数结构：各 ISA 翻译为其本地寄存器/立即数格式
  Operands* = object
    rd*: uint8       ## 目标寄存器 (0-31)
    rs1*: uint8      ## 源寄存器 1
    rs2*: uint8      ## 源寄存器 2 / Store 的源
    imm*: int32      ## 立即数（已符号扩展为 32 位）
    shamt*: uint8    ## 移位量（仅位移指令）

  ## encode 结果：机器码字节序列 + 元信息
  EncodedInsn* = object
    semId*: string            ## 语义 ID（跨 ISA 唯一）
    code*: seq[uint8]         ## 机器码字节序列
    byteLen*: int             ## 实际字节数
    bitLen*: int              ## 有效 bit 数（32 / 64 等）

  ## decode 结果：从机器码反推语义
  DecodedInsn* = object
    semId*: string            ## 语义 ID
    operands*: Operands       ## 反推的操作数
    byteLen*: int

  ## 错误码
  CodecError* = enum
    ceOk = 0
    ceUnknownSemId            ## 语义 ID 未找到
    ceInvalidOperands         ## 操作数越界（如寄存器 > 31）
    ceTruncatedCode           ## 机器码不完整
    ceUnknownOpcode           ## 操作码未知
    ceOutOfRangeImm           ## 立即数超出范围

  ## encode/decode 结果（带错误码）
  EncResult* = object
    case ok*: bool
    of true: encoded*: EncodedInsn
    of false:
      error*: CodecError
      msg*: string

  DecResult* = object
    case ok*: bool
    of true: decoded*: DecodedInsn
    of false:
      error*: CodecError
      msg*: string

# ── 辅助 ──────────────────────────────────────────────

func newOperands*(rd: uint8 = 0, rs1: uint8 = 0, rs2: uint8 = 0;
                  imm: int32 = 0, shamt: uint8 = 0): Operands =
  ## 构建 Operands 的便捷构造器
  result.rd = rd
  result.rs1 = rs1
  result.rs2 = rs2
  result.imm = imm
  result.shamt = shamt

func noneOperands*(): Operands =
  ## 全零操作数
  result = Operands()

func eq*(a, b: Operands): bool =
  ## Operands 相等比较
  result = (a.rd == b.rd) and (a.rs1 == b.rs1) and (a.rs2 == b.rs2) and
            (a.imm == b.imm) and (a.shamt == b.shamt)

func `$`*(op: Operands): string =
  ## 可读格式
  result = "[" & $op.rd & ", " & $op.rs1 & ", " & $op.rs2 & ", imm=" & $op.imm & ", shamt=" & $op.shamt & "]"
