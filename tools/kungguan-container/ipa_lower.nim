# IPA 特殊韵母符号 — 小写
# 来源：z_clipboard.txt 行6-7
# 行6（开元音/变形系列）: ɐ(a) ɛ(e) ɪ(i) ɔ(o) ʊ(u) [行6无ü候选·留空]
# 行7（闭央/杠系列）  : ə(schwa) ɜ(open-mid) ɨ(barred-i) ɵ(barred-o) ʉ(barred-u) ɏ(barred-y=ü)
#
# 行7第6位：ɏ (U+024F LATIN SMALL LETTER Y WITH STROKE)
#   模式匹配：ɨ→杠i · ɵ→杠o · ʉ→杠u · ɏ→杠y=ü
#   大写配对：Ɏ (U+024E) — Unicode 标准大小写配对
#   产权：Latin Extended-B 区 → plOwner（主人创造变音符号区）

import std/[strutils, unicode]

const
  IPALower*: seq[string] = @[
    "ɐ",   # 0  U+0250 LATIN SMALL LETTER TURNED A
    "ɛ",   # 1  U+025B LATIN SMALL LETTER OPEN E
    "ɪ",   # 2  U+026A LATIN LETTER SMALL CAPITAL I
    "ɔ",   # 3  U+0254 LATIN SMALL LETTER OPEN O
    "ʊ",   # 4  U+028A LATIN SMALL LETTER UPSILON
    "",    # 5  [行6·ü位·留空待补]
    "ə",   # 6  U+0259 LATIN SMALL LETTER SCHWA
    "ɜ",   # 7  U+025C LATIN SMALL LETTER REVERSED OPEN E
    "ɨ",   # 8  U+0268 LATIN SMALL LETTER I WITH STROKE
    "ɵ",   # 9  U+0275 LATIN SMALL LETTER BARRED O
    "ʉ",   # 10 U+0289 LATIN SMALL LETTER U BAR
    "ɏ"    # 11 U+024F LATIN SMALL LETTER Y WITH STROKE (= ü行7解)
  ]

  IPALowerSize* = 11   # 含1个占位符，有效符号10个


func ipaLower*(idx: int): string =
  ## 按索引取 IPA 小写符号，越界或占位符返回 ""
  if idx >= 0 and idx < IPALowerSize:
    result = IPALower[idx]
  else:
    result = ""


func ipaLowerValid*(idx: int): bool =
  ## 该索引是否有有效符号（排除占位符）
  if idx >= 0 and idx < IPALowerSize:
    result = IPALower[idx].len > 0
  else:
    result = false


func ipaLowerName*(idx: int): string =
  ## 返回符号的 Unicode 名称
  case idx
  of 0: result = "LATIN SMALL LETTER TURNED A"
  of 1: result = "LATIN SMALL LETTER OPEN E"
  of 2: result = "LATIN LETTER SMALL CAPITAL I"
  of 3: result = "LATIN SMALL LETTER OPEN O"
  of 4: result = "LATIN SMALL LETTER UPSILON"
  of 5: result = "[RESERVED·行6ü位·待补]"
  of 6: result = "LATIN SMALL LETTER SCHWA"
  of 7: result = "LATIN SMALL LETTER REVERSED OPEN E"
  of 8: result = "LATIN SMALL LETTER I WITH STROKE"
  of 9: result = "LATIN SMALL LETTER BARRED O"
  of 10: result = "LATIN SMALL LETTER U BAR"
  of 11: result = "LATIN SMALL LETTER Y WITH STROKE"
  else: result = "OUT OF RANGE"


func ipaLowerAll*(): seq[string] =
  ## 返回全部符号（含占位符）
  result = IPALower


func ipaLowerValidCount*(): int =
  ## 有效符号数量（排除占位符）
  result = 0
  for s in IPALower:
    if s.len > 0: inc(result)
