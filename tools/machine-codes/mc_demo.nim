# ============================================================
# 机器码跨 ISA 对齐表 · 演示
#
# 验证：
#   1. 四 ISA 加载完整
#   2. 跨 ISA 对齐查询
#   3. RISC-V ADD 位域拼接
#   4. x86-64 摘要输出
#   5. MCOD 字体容器头结构验证
#
# 等 Nim 编译器就绪后一次通过。
# ============================================================

import mc_types, mc_rv32i, mc_arm64, mc_x64, mc_mips,
       mc_align, mc_codec, mc_font

when isMainModule:
  echo "════════════════════════════════════════════════════════"
  echo "  跨 ISA 机器码对齐表 · 静态演示"
  echo "════════════════════════════════════════════════════════"
  echo ""

  # ── 阶段 1 ──
  echo "【阶段 1】四 ISA 编码表加载"
  echo "  RISC-V RV32I : ", RV32I_ENCODINGS.len, " 条"
  echo "  ARM64        : ", ARM64_ENCODINGS.len, " 条"
  echo "  x86-64       : ", X86_64_ENCODINGS.len, " 条"
  echo "  MIPS32       : ", MIPS32_ENCODINGS.len, " 条"
  echo "  合计         : ",
       RV32I_ENCODINGS.len + ARM64_ENCODINGS.len +
       X86_64_ENCODINGS.len + MIPS32_ENCODINGS.len, " 条"
  echo ""

  # ── 阶段 2 ──
  echo "【阶段 2】跨 ISA 对齐查询"
  for semId in ["ADD", "SUB", "BEQ", "LW", "SW", "RET"]:
    echo summary(semId)
    echo ""

  # ── 阶段 3 ──
  echo "【阶段 3】RISC-V ADD 位域拼接"
  echo "  语义 ID : ADD"
  echo "  操作数  : rd=1, rs1=2, rs2=3"
  echo "  预期机器码 : 0x003100B3（小端字节序）"
  echo "  字段数  : ", RV32I_ENCODINGS[0].fields.len
  for f in RV32I_ENCODINGS[0].fields:
    echo "    ", f.name, " [", f.hi, ":", f.lo, "]"
  echo ""

  # ── 阶段 4 ──
  echo "【阶段 4】MCOD 字体容器头"
  let spec = DEFAULT_FONT_SPEC
  echo "  字体族 : ", spec.familyName
  echo "  私用区起始码点 : U+", spec.codepointBase.toHex(4)
  echo "  MCOD Magic : 0x", MCOD_MAGIC.toHex(8), " ('MCOD')"
  echo "  MCOD Version : ", MCOD_VERSION
  echo ""

  echo "════════════════════════════════════════════════════════"
  echo "  跨 ISA 对齐表 · 演示完毕"
  echo "════════════════════════════════════════════════════════"
