import os
import strutils
import times
import sequtils
import types
import bootstrap

echo "═══════════════════════════════════════════════════════"
echo "  MCCP 自举编译器 · 演示"
echo "  满全法容器协议 · 从文件系统长出工具"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════
# Part 1: 创建规则文件
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 1】创建规则文件"
echo "═══════════════════════════════════════════════════════"
echo ""

let rules_dir = getCurrentDir() / "rules"
let output_dir = getCurrentDir() / "output"

discard existsOrCreateDir(rules_dir)
discard existsOrCreateDir(output_dir)

let rules_container = """
# MCCP rules: containers
# format: glyph : pattern -> action : target

mlll : container_root -> recursive_gen : nim
gtt : morphism -> gen_morphism : nim
mlnn : couple -> gen_couple : nim
mltt : cong_ti -> gen_cong_ti : nim
"""

let rules_cipher = """
# MCCP rules: cipher

lctqf : quan_fa -> gen_quan_fa : nim
lctfrqmy : wife -> gen_wife : nim
wgslz : husband -> gen_husband : nim
yinyang : polarity -> gen_polarity : json
"""

let rules_judge = """
# MCCP rules: judge

judge : verdict -> gen_verdict : nim
witness : record -> gen_record : json
conflict : dispute -> gen_dispute : nim
"""

writeFile(rules_dir / "containers.mccp", rules_container)
writeFile(rules_dir / "cipher.mccp", rules_cipher)
writeFile(rules_dir / "judge.mccp", rules_judge)

echo "  rules_dir: ", rules_dir
echo "  files:"
for f in walkFiles(rules_dir / "*.mccp"):
    echo "    - ", extractFileName(f)
echo ""

# ═══════════════════════════════════════════════════════
# Part 2: 初始化
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 2】MCCP init"
echo "═══════════════════════════════════════════════════════"
echo ""

var ctx = new_mccp(rules_dir, output_dir)

ctx.register_glyph("mlll", 1, 1)
ctx.register_glyph("gtt", 0, 2)
ctx.register_glyph("mlnn", 1, 1)
ctx.register_glyph("mltt", 0, 3)
ctx.register_glyph("lctqf", 1, 5)
ctx.register_glyph("lctfrqmy", -1, 4)
ctx.register_glyph("wgslz", 1, 4)
ctx.register_glyph("yinyang", 0, 1)
ctx.register_glyph("judge", 1, 6)
ctx.register_glyph("witness", 0, 6)
ctx.register_glyph("conflict", -1, 7)

echo "  glyphs: ", ctx.glyph_names.len
for pair in ctx.glyphs:
  echo "    ", pair.name.alignLeft(14), " yy=", pair.entry.yinyang.alignLeft(10),
       " pol=", pair.entry.polarity, " layer=", pair.entry.offset_layer
echo ""

# ═══════════════════════════════════════════════════════
# Part 3: 读取规则
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 3】读取规则"
echo "═══════════════════════════════════════════════════════"
echo ""

let rule_count = ctx.read_rules()
ctx.phase = bpParse

echo "  families: ", ctx.families.len
echo "  rules:    ", rule_count
echo ""

for fam in ctx.families:
  echo "  [", extractFileName(fam.source), " : ", fam.rules.len, " rules, cksum=", fam.checksum[0..7], "...]"
  for rule in fam.rules:
    echo "    ", rule.glyph.alignLeft(14), " : ", rule.pattern.alignLeft(20),
         " -> ", rule.action.alignLeft(25), " : ", rule.action_target
echo ""

# ═══════════════════════════════════════════════════════
# Part 4: 验证
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 4】规则验证"
echo "═══════════════════════════════════════════════════════"
echo ""

ctx.phase = bpValidate
let errors = ctx.validate_rules()

if errors.len == 0:
  echo "  all rules valid, no conflicts"
else:
  echo "  errors: ", errors.len
  for e in errors: echo "    ", e
echo ""

# ═══════════════════════════════════════════════════════
# Part 5: 代码生成
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 5】代码生成"
echo "═══════════════════════════════════════════════════════"
echo ""

ctx.phase = bpGen

let nim_code = ctx.generate_nim("mccp_self_bootstrap")
writeFile(output_dir / "mccp_self.nim", nim_code)
echo "  Nim:  output/mccp_self.nim (", nim_code.len, " bytes)"

let json_code = ctx.generate_json()
writeFile(output_dir / "mccp_self.json", json_code)
echo "  JSON: output/mccp_self.json (", json_code.len, " bytes)"
echo ""

echo "  Nim preview (first 15 lines):"
var line_idx = 0
for ln in nim_code.splitLines():
  if line_idx >= 15:
    echo "    ..."
    break
  echo "    ", ln
  line_idx.inc
echo ""

# ═══════════════════════════════════════════════════════
# Part 6: 自举验证
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 6】自举验证 MCCP'(MCCP) == MCCP'"
echo "═══════════════════════════════════════════════════════"
echo ""

ctx.phase = bpVerify
let verify = ctx.self_verify()

echo "  Nim hash:  ", verify.hash1
echo "  JSON hash: ", verify.hash2
echo "  result:    ", if verify.ok: "OK bootstrap valid" else: "FAIL"
echo ""

# ═══════════════════════════════════════════════════════
# Part 7: 九元偏移率
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 7】九元偏移率"
echo "═══════════════════════════════════════════════════════"
echo ""

var family_idx = 0
for fam in ctx.families:
  let layer = family_idx + 1
  let offset = float(family_idx) * 0.1
  echo "  Layer", layer, " d=", formatFloat(offset, ffDecimal, 2),
       " -> ", extractFileName(fam.source).alignLeft(20), " (", fam.rules.len, " rules)"
  family_idx.inc
echo ""

# ═══════════════════════════════════════════════════════
# 完成
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "  MCCP bootstrap demo complete"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  families:        ", ctx.families.len
echo "  glyphs:          ", ctx.glyph_names.len
echo "  rules:           ", rule_count
echo "  validation:      ", if errors.len == 0: "PASS" else: "FAIL"
echo "  Nim generated:   ", nim_code.len, " bytes"
echo "  JSON generated:  ", json_code.len, " bytes"
echo "  self-verify:     ", if verify.ok: "PASS" else: "FAIL"
echo ""
echo "  [MCCP ONLINE]"
echo ""
