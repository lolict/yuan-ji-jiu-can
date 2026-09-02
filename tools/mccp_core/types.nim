# ============================================================
# MCCP 自举编译器 · 类型定义
#
# 自举条件：输出的编译器MCCP'满足 MCCP'(MCCP) = MCCP'
# ============================================================

type
  BootPhase* = enum
    bpRead
    bpParse
    bpValidate
    bpGen
    bpVerify

  RuleSource* = string

  FCCPRule* = object
    name*: string
    source*: RuleSource
    glyph*: string
    polarity*: int8
    pattern*: string
    action*: string
    action_target*: string

  RuleFamily* = object
    source*: RuleSource
    rules*: seq[FCCPRule]
    checksum*: string

  GlyphEntry* = object
    name*: string
    yinyang*: string
    polarity*: int8
    cluster_id*: int
    offset_layer*: int

  ParseState* = object
    text*: string
    pos*: int
    glyphs*: seq[GlyphEntry]
    errors*: seq[string]

  GenUnit* = object
    target*: string
    module*: string
    body*: string
    rules_applied*: seq[string]
    phase*: BootPhase

  MCCPContext* = object
    rules_dir*: string
    output_dir*: string
    families*: seq[RuleFamily]
    glyphs*: seq[tuple[name: string; entry: GlyphEntry]]
    glyph_names*: seq[string]
    units*: seq[GenUnit]
    phase*: BootPhase
    self_hash*: string

  TargetLang* = enum
    tlNim
    tlC
    tlJson
    tlZig
    tlWat
