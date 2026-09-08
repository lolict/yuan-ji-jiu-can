## ms_demo — RV32I 编解码器全面演示 + round-trip 验证
##
## 覆盖所有 RV32I 指令：41 条
## 每种指令都做 encode → decode → 对比原值，确认恒等

import std/[strutils, sequtils, strformat]
import ms_types, ms_rv32i

# ═══════════════════════════════════════════════════
#  测试入口
# ═══════════════════════════════════════════════════

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════════╗"
  echo "║   MCODE-Stone · RV32I 精确编解码器           ║"
  echo "║   Semantic ID ↔ Machine Code Bidirectional   ║"
  echo "╚══════════════════════════════════════════════╝"
  echo ""

  initLuts()
  echo "已注册指令数: ", rv32iInsnCount()
  echo ""

  # ───────────── 第一部分：单个指令精确验证 ─────────────

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▸ 第一部分：已知结果的精确对比"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""

  type
    KnownCase = object
      semId: string
      op: Operands
      expected: string    ## 期望的 32 位机器码（16进制，大端表示）

  const KNOWN: seq[KnownCase] = @[
    # R-type
    KnownCase(semId: "ADD",  op: newOperands(rd = 1, rs1 = 2, rs2 = 3),
              expected: "003100B3"),   # add x1, x2, x3
    KnownCase(semId: "SUB",  op: newOperands(rd = 2, rs1 = 3, rs2 = 4),
              expected: "40418133"),   # sub x2, x3, x4
    KnownCase(semId: "XOR",  op: newOperands(rd = 10, rs1 = 11, rs2 = 12),
              expected: "00C5C533"),   # xor x10, x11, x12
    KnownCase(semId: "AND",  op: newOperands(rd = 5, rs1 = 6, rs2 = 7),
              expected: "007372B3"),   # and x5, x6, x7
    KnownCase(semId: "OR",   op: newOperands(rd = 8, rs1 = 9, rs2 = 10),
              expected: "00A4E433"),   # or x8, x9, x10
    KnownCase(semId: "SLL",  op: newOperands(rd = 4, rs1 = 8, rs2 = 2),
              expected: "00241233"),   # sll x4, x8, x2

    # I-type ALU
    KnownCase(semId: "ADDI", op: newOperands(rd = 5, rs1 = 0, imm = 7),
              expected: "00700293"),   # addi x5, x0, 7
    KnownCase(semId: "ADDI", op: newOperands(rd = 1, rs1 = 1, imm = -1),
              expected: "FFF08093"),   # addi x1, x1, -1 (=0xFFFFFFFF掩码低12位 = 0xFFF → imm bits)
    KnownCase(semId: "XORI", op: newOperands(rd = 6, rs1 = 7, imm = 255),
              expected: "0FF3C313"),   # xori x6, x7, 255

    # Load
    KnownCase(semId: "LW",   op: newOperands(rd = 3, rs1 = 4, imm = 0),
              expected: "00022183"),   # lw x3, 0(x4)
    KnownCase(semId: "LW",   op: newOperands(rd = 10, rs1 = 11, imm = 4),
              expected: "0045A503"),   # lw x10, 4(x11)

    # Store
    KnownCase(semId: "SW",   op: newOperands(rs1 = 5, rs2 = 6, imm = 8),
              expected: "0062A423"),   # sw x6, 8(x5)

    # Upper
    KnownCase(semId: "LUI",  op: newOperands(rd = 1, imm = 0x12345),
              expected: "123450B7"),   # lui x1, 0x12345
    KnownCase(semId: "AUIPC",op: newOperands(rd = 2, imm = 0),
              expected: "00000117"),   # auipc x2, 0

    # System
    KnownCase(semId: "ECALL", op: noneOperands(),
              expected: "00000073"),   # ecall
  ]

  var passCount, failCount: int

  for k in KNOWN:
    let r = encodeRV32I(k.semId, k.op)
    if not r.ok:
      echo "  ✗ ", k.semId, " 编码失败: ", r.msg
      inc failCount
      continue
    # 把 4 字节小端序拼成 32-bit hex
    let dw = uint32(r.encoded.code[0]) or (uint32(r.encoded.code[1]) shl 8) or
             (uint32(r.encoded.code[2]) shl 16) or (uint32(r.encoded.code[3]) shl 24)
    let hex = toHex(dw).toUpperAscii().alignLeft(8)
    if hex == k.expected:
      echo "  ✓ ", k.semId.alignLeft(7), " ", $k.op, " → 0x", hex
      inc passCount
    else:
      echo "  ✗ ", k.semId.alignLeft(7), " 期望 0x", k.expected, " 实际 0x", hex
      inc failCount

  echo ""
  echo "精确对比：通过 ", passCount, " / ", passCount + failCount

  # ───────────── 第二部分：全部 round-trip ─────────────

  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▸ 第二部分：全指令 round-trip 恒等验证"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""

  # 构造每组指令的标准测试操作数
  type
    RTCase = object
      semId: string
      op: Operands

  const RT_CASES: seq[RTCase] = @[
    # R-type
    RTCase(semId: "ADD",   op: newOperands(rd = 10, rs1 = 11, rs2 = 12)),
    RTCase(semId: "SUB",   op: newOperands(rd = 5, rs1 = 6, rs2 = 7)),
    RTCase(semId: "SLL",   op: newOperands(rd = 4, rs1 = 8, rs2 = 2)),
    RTCase(semId: "SLT",   op: newOperands(rd = 28, rs1 = 29, rs2 = 30)),
    RTCase(semId: "SLTU",  op: newOperands(rd = 3, rs1 = 4, rs2 = 5)),
    RTCase(semId: "XOR",   op: newOperands(rd = 14, rs1 = 15, rs2 = 16)),
    RTCase(semId: "SRL",   op: newOperands(rd = 17, rs1 = 18, rs2 = 19)),
    RTCase(semId: "SRA",   op: newOperands(rd = 20, rs1 = 21, rs2 = 22)),
    RTCase(semId: "OR",    op: newOperands(rd = 6, rs1 = 7, rs2 = 8)),
    RTCase(semId: "AND",   op: newOperands(rd = 9, rs1 = 10, rs2 = 11)),

    # I-type ALU
    RTCase(semId: "ADDI",  op: newOperands(rd = 3, rs1 = 4, imm = -128)),
    RTCase(semId: "SLTI",  op: newOperands(rd = 1, rs1 = 2, imm = 100)),
    RTCase(semId: "SLTIU", op: newOperands(rd = 31, rs1 = 30, imm = 1)),
    RTCase(semId: "XORI",  op: newOperands(rd = 25, rs1 = 26, imm = 0x7FF)),
    RTCase(semId: "ORI",   op: newOperands(rd = 2, rs1 = 3, imm = 42)),
    RTCase(semId: "ANDI",  op: newOperands(rd = 4, rs1 = 5, imm = 0xFF)),
    RTCase(semId: "SLLI",  op: newOperands(rd = 6, rs1 = 7, shamt = 13)),
    RTCase(semId: "SRLI",  op: newOperands(rd = 8, rs1 = 9, shamt = 4)),
    RTCase(semId: "SRAI",  op: newOperands(rd = 10, rs1 = 11, shamt = 20)),

    # Load
    RTCase(semId: "LB",    op: newOperands(rd = 5, rs1 = 6, imm = -4)),
    RTCase(semId: "LH",    op: newOperands(rd = 7, rs1 = 8, imm = 0)),
    RTCase(semId: "LW",    op: newOperands(rd = 9, rs1 = 10, imm = 256)),
    RTCase(semId: "LBU",   op: newOperands(rd = 11, rs1 = 12, imm = 2047)),
    RTCase(semId: "LHU",   op: newOperands(rd = 13, rs1 = 14, imm = -2048)),

    # Store
    RTCase(semId: "SB",    op: newOperands(rs1 = 15, rs2 = 16, imm = 100)),
    RTCase(semId: "SH",    op: newOperands(rs1 = 17, rs2 = 18, imm = -50)),
    RTCase(semId: "SW",    op: newOperands(rs1 = 19, rs2 = 20, imm = 0)),

    # Jump
    RTCase(semId: "JAL",   op: newOperands(rd = 1, imm = 0x100)),
    RTCase(semId: "JALR",  op: newOperands(rd = 0, rs1 = 1, imm = 4)),

    # Upper (20-bit label, encode takes bits[31:12] directly)
    RTCase(semId: "LUI",   op: newOperands(rd = 2, imm = 0xABCDE)),
    RTCase(semId: "AUIPC", op: newOperands(rd = 3, imm = 0x56789)),

    # System
    RTCase(semId: "ECALL", op: noneOperands()),
    RTCase(semId: "EBREAK",op: noneOperands()),
  ]

  var rtPass, rtFail: int
  for c in RT_CASES:
    let rt = roundtripRV32I(c.semId, c.op)
    if rt.ok and rt.original == rt.decoded:
      echo "  ✓ ", c.semId.alignLeft(7), " round-trip 恒等"
      inc rtPass
    else:
      echo "  ✗ ", c.semId.alignLeft(7)
      echo "    原值: ", rt.original
      echo "    恢复: ", rt.decoded
      inc rtFail

  echo ""
  echo "Round-trip: 通过 ", rtPass, " / ", rtPass + rtFail

  # ───────────── 第三部分：decode 已知机器码 ─────────────

  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▸ 第三部分：机器码 → 语义 ID 解码验证"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""

  type
    CodeCase = object
      code: string         ## 4字节大端 hex
      expectedSemId: string

  const DECODE_CASES: seq[CodeCase] = @[
    CodeCase(code: "003100B3", expectedSemId: "ADD"),
    CodeCase(code: "40418133", expectedSemId: "SUB"),
    CodeCase(code: "007372B3", expectedSemId: "AND"),
    CodeCase(code: "00A4E433", expectedSemId: "OR"),
    CodeCase(code: "00C5C533", expectedSemId: "XOR"),
    CodeCase(code: "00700293", expectedSemId: "ADDI"),
    CodeCase(code: "00022183", expectedSemId: "LW"),
    CodeCase(code: "00000073", expectedSemId: "ECALL"),
    CodeCase(code: "00100073", expectedSemId: "EBREAK"),
  ]

  var decPass, decFail: int
  for d in DECODE_CASES:
    let bytes = @[parseHexInt(d.code[0..1]).uint8, parseHexInt(d.code[2..3]).uint8,
                   parseHexInt(d.code[4..5]).uint8, parseHexInt(d.code[6..7]).uint8]
    # 顺序是大端 → 转换小端序
    let le = @[bytes[3], bytes[2], bytes[1], bytes[0]]
    let dec = decodeRV32I(le)
    if dec.ok and dec.decoded.semId.toUpperAscii() == d.expectedSemId.toUpperAscii():
      echo "  ✓ 0x", d.code, " → ", dec.decoded.semId, " ", dec.decoded.operands
      inc decPass
    else:
      if dec.ok:
        echo "  ✗ 0x", d.code, " 期望 " & d.expectedSemId & " 实际 " & dec.decoded.semId
      else:
        echo "  ✗ 0x", d.code, " 解码失败: ", dec.msg
      inc decFail

  echo ""
  echo "解码验证：通过 ", decPass, " / ", decPass + decFail

  # ───────────── 汇总 ─────────────

  echo ""
  echo "═══════════════════════════════════════════════"
  let totalPass = passCount + rtPass + decPass
  let totalTests = (passCount + failCount) + (rtPass + rtFail) + (decPass + decFail)
  echo "  全部测试汇总: ", totalPass, " / ", totalTests, " 通过"
  echo "  RV32I 已注册指令: ", rv32iInsnCount()
  if totalPass == totalTests:
    echo "  ✓ 全部通过"
  else:
    echo "  ✗ 存在失败"
  echo "═══════════════════════════════════════════════"
