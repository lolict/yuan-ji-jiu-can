# IPA 特殊韵母符号 — 大写
# 来源：z_clipboard.txt 行29-38（用户指定大小写配对）
# 行6大写: Ɐ(a) Ɛ(e) Ɨ(i) Ɔ(o) Ʊ(u) [行6无ü候选·留空]
# 行7大写: Ə(schwa) Ɜ(open-mid) Ɨ(barred-i) Ɵ(barred-o) Ʉ(barred-u) Ɏ(barred-y=ü)
#
# 行7第6位：Ɏ (U+024E LATIN CAPITAL LETTER Y WITH STROKE)
#   Unicode 标准大小写配对：ɏ ↔ Ɏ
#   产权：Latin Extended-B 区 → plOwner

import std/[strutils, unicode]
import ipa_lower

const
  IPAUpper*: seq[string] = @[
    "Ɐ",   # 0  U+2C6F LATIN CAPITAL LETTER TURNED A
    "Ɛ",   # 1  U+0190 LATIN CAPITAL LETTER OPEN E
    "Ɨ",   # 2  U+0197 LATIN CAPITAL LETTER I WITH STROKE
    "Ɔ",   # 3  U+0186 LATIN CAPITAL LETTER OPEN O
    "Ʊ",   # 4  U+01B1 LATIN CAPITAL LETTER UPSILON
    "",    # 5  [行6·ü位·留空待补]
    "Ə",   # 6  U+018F LATIN CAPITAL LETTER SCHWA
    "Ɜ",   # 7  U+A7AB LATIN CAPITAL LETTER REVERSED OPEN E
    "Ɨ",   # 8  U+0197 LATIN CAPITAL LETTER I WITH STROKE
    "Ɵ",   # 9  U+019F LATIN CAPITAL LETTER BARRED O
    "Ʉ",   # 10 U+0244 LATIN CAPITAL LETTER U BAR
    "Ɏ"    # 11 U+024E LATIN CAPITAL LETTER Y WITH STROKE (= ü行7解)
  ]

  IPAUpperSize* = 11   # 含1个占位符，有效符号10个


func ipaUpper*(idx: int): string =
  ## 按索引取 IPA 大写符号，越界或占位符返回 ""
  if idx >= 0 and idx < IPAUpperSize:
    result = IPAUpper[idx]
  else:
    result = ""


func ipaUpperValid*(idx: int): bool =
  ## 该索引是否有有效符号（排除占位符）
  if idx >= 0 and idx < IPAUpperSize:
    result = IPAUpper[idx].len > 0
  else:
    result = false


func ipaPair*(idx: int): tuple[lower: string; upper: string; valid: bool] =
  ## 同时返回小写/大写配对
  if idx >= 0 and idx < IPAUpperSize:
    result.lower = ipaLower(idx)
    result.upper = IPAUpper[idx]
    result.valid = IPAUpper[idx].len > 0
  else:
    result = (lower: "", upper: "", valid: false)


func ipaUpperAll*(): seq[string] =
  ## 返回全部符号（含占位符）
  result = IPAUpper


func ipaUpperValidCount*(): int =
  ## 有效符号数量（排除占位符）
  result = 0
  for s in IPAUpper:
    if s.len > 0: inc(result)
