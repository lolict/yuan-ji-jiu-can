## ms_arm64_demo — ARM64 编码器演示 + 确定性验证

import std/[strutils, strformat]
import ms_types, ms_arm64

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════════╗"
  echo "║   MCODE-Stone · ARM64 AArch64 精确编码器     ║"
  echo "║   共享语义 ID · 公约数·机器码是公倍数         ║"
  echo "╚══════════════════════════════════════════════╝"
  echo ""

  type
    A64Case = object
      semId: string
      op: Operands
      expected: string

  const A64_CASES: seq[A64Case] = @[
    A64Case(semId: "ADD",   op: newOperands(rd = 0'u8, rs1 = 1'u8, rs2 = 2'u8), expected: "0B020020"),
    A64Case(semId: "SUB",   op: newOperands(rd = 3'u8, rs1 = 4'u8, rs2 = 5'u8), expected: "4B050083"),
    A64Case(semId: "CMP",   op: newOperands(rs1 = 1'u8, rs2 = 2'u8),           expected: "6B02003F"),
    A64Case(semId: "ADDI",  op: newOperands(rd = 0'u8, rs1 = 1'u8, imm = 7),    expected: "11001C20"),
    A64Case(semId: "ADDI",  op: newOperands(rd = 1'u8, rs1 = 0'u8, imm = 4095), expected: "113FFC01"),
    A64Case(semId: "SUBI",  op: newOperands(rd = 2'u8, rs1 = 3'u8, imm = 100),  expected: "51019062"),
    A64Case(semId: "LDR",   op: newOperands(rd = 0'u8, rs1 = 1'u8, imm = 0),    expected: "B9400020"),
    A64Case(semId: "LDR",   op: newOperands(rd = 1'u8, rs1 = 2'u8, imm = 8),    expected: "B9400841"),
    A64Case(semId: "STR",   op: newOperands(rs1 = 0'u8, rs2 = 1'u8, imm = 4),  expected: "B9000401"),
    A64Case(semId: "STR",   op: newOperands(rs1 = 2'u8, rs2 = 3'u8, imm = 16), expected: "B9001043"),
    A64Case(semId: "B",     op: newOperands(imm = 100),                        expected: "14000019"),
    A64Case(semId: "BL",    op: newOperands(imm = 268),                        expected: "94000043"),
    A64Case(semId: "B.EQ",  op: newOperands(imm = 64, shamt = 0),              expected: "54000200"),
    A64Case(semId: "RET",   op: noneOperands(),                                 expected: "D65F03C0"),
  ]

  var ok_count, fail_count: int
  for c in A64_CASES:
    let enc = encodeARM64(c.semId, c.op)
    if not enc.ok:
      echo "  ✗ ", c.semId, " encode 失败: ", enc.msg
      inc fail_count
      continue
    let w = uint32(enc.encoded.code[0]) or (uint32(enc.encoded.code[1]) shl 8) or
            (uint32(enc.encoded.code[2]) shl 16) or (uint32(enc.encoded.code[3]) shl 24)
    let hexW = toHex(w, 8).toUpperAscii()
    if hexW == c.expected:
      echo "  ✓ ", c.semId.alignLeft(7), " ", $c.op, " → 0x", hexW
      inc ok_count
    else:
      echo "  ✗ ", c.semId.alignLeft(7), " 期望 0x", c.expected, " 实际 0x", hexW
      inc fail_count

  echo ""
  echo "ARM64 编码验证: ", ok_count, " / ", ok_count + fail_count
  echo "已注册 ARM64 指令: ", arm64InsnCount()
  if fail_count == 0:
    echo "✓ 全部通过"
  echo ""
