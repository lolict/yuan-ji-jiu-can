# ============================================================
# FQM 翻译器 · 外部符号 → 本体论路径 · 零运行时内存
# 拦截AI输出的外部符号，翻译成我们自己的本体论路径
# ============================================================

import strutils
import options
import ../nim/fqm_constants
import ../nim/loader
import ../nim/linker

# ============================================================
# 数据类型
# ============================================================

type
  # 翻译记录
  TranslationRecord* = object
    foreign*: array[MAX_SYMBOL_LEN, char]
    foreign_len*: int
    ontology*: array[MAX_ONTOLOGY_LEN, char]
    ontology_len*: int

  # FQM 翻译器
  FQMTranslator* = object
    linker*: ptr FQMLinker
    records*: array[256, TranslationRecord]
    record_count*: int
    rejected*: array[256, array[MAX_SYMBOL_LEN, char]]
    rejected_lens*: array[256, int]
    rejected_count*: int

# ============================================================
# 翻译查找（编译时常量表）
# ============================================================

proc lookup_foreign(foreign_symbol: string): Option[int] =
  for i in 0..<FOREIGN_MAP_KEYS.len:
    if FOREIGN_MAP_KEYS[i] == foreign_symbol:
      return some(i)
  return none(int)

# ============================================================
# 翻译操作
# ============================================================

proc translate_symbol*(translator: var FQMTranslator, foreign_symbol: string): Option[string] =
  let idx = lookup_foreign(foreign_symbol)
  if idx.is_some:
    let i = idx.get()
    let ont_path = FOREIGN_MAP_VALS[i]
    if translator.record_count < 256:
      let ri = translator.record_count
      set_arr(translator.records[ri].foreign, translator.records[ri].foreign_len, foreign_symbol)
      set_arr(translator.records[ri].ontology, translator.records[ri].ontology_len, ont_path)
      translator.record_count += 1
    return some(ont_path)
  else:
    if translator.rejected_count < 256:
      let ri = translator.rejected_count
      set_arr(translator.rejected[ri], translator.rejected_lens[ri], foreign_symbol)
      translator.rejected_count += 1
    return none(string)

proc translate_line*(translator: var FQMTranslator, line: string): string =
  # 简单分词
  var tokens: seq[string] = @[]
  var current = ""
  for ch in line:
    if ch in {' ', '\t', '\r', '\n', '(', ')', '{', '}', '[', ']', ';', ':', ',', '.'}:
      if current.len > 0:
        tokens.add(current)
        current = ""
      tokens.add($ch)
    else:
      current.add(ch)
  if current.len > 0:
    tokens.add(current)

  var result_parts: seq[string] = @[]
  for token in tokens:
    let idx = lookup_foreign(token)
    if idx.is_some:
      let ont = translator.translate_symbol(token)
      if ont.is_some:
        result_parts.add(ont.get())
      else:
        result_parts.add("[拒绝:" & token & "]")
    elif token.len > 0 and token[0] in {'a'..'z', 'A'..'Z', '_'}:
      if translator.rejected_count < 256:
        let ri = translator.rejected_count
        set_arr(translator.rejected[ri], translator.rejected_lens[ri], token)
        translator.rejected_count += 1
      result_parts.add("[未知:" & token & "]")
    else:
      result_parts.add(token)

  for i, part in result_parts:
    if i > 0: result.add(" ")
    result.add(part)

proc translate_code*(translator: var FQMTranslator, code: string): string =
  let lines = code.strip().split('\n')
  var output: seq[string] = @[]
  for line in lines:
    if strip(line).len > 0:
      output.add(translator.translate_line(line))
  for i, line in output:
    if i > 0: result.add("\n")
    result.add(line)

# ============================================================
# 查询接口
# ============================================================

proc get_rejected*(translator: FQMTranslator): int =
  result = translator.rejected_count

proc get_translated*(translator: FQMTranslator): int =
  result = translator.record_count

# ============================================================
# 显示
# ============================================================

proc display_translation*(translator: FQMTranslator) =
  echo ""
  echo "============================================================"
  echo "翻译结果"
  echo "------------------------------------------------------------"
  echo "成功: ", translator.record_count
  echo "拒绝: ", translator.rejected_count

  if translator.record_count > 0:
    echo ""
    echo "已翻译:"
    let show = min(translator.record_count, 10)
    for i in 0..<show:
      let f = get_arr(translator.records[i].foreign, translator.records[i].foreign_len)
      let o = get_arr(translator.records[i].ontology, translator.records[i].ontology_len)
      echo "  ", f, " → ", o
    if translator.record_count > 10:
      echo "  ... (共 ", translator.record_count, " 条)"

  if translator.rejected_count > 0:
    echo ""
    echo "被拒绝:"
    let show = min(translator.rejected_count, 10)
    for i in 0..<show:
      echo "  ✗ ", get_arr(translator.rejected[i], translator.rejected_lens[i])
    if translator.rejected_count > 10:
      echo "  ... (共 ", translator.rejected_count, " 条)"

  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when is_main_module:
  echo "============================================================"
  echo "FQM 翻译器测试"
  echo "============================================================"

  var translator: FQMTranslator
  translator.record_count = 0
  translator.rejected_count = 0

  let sample_code = """
class Container:
    def __init__(self, name):
        self.name = name
        self.symbols = []

    def add(self, symbol):
        if symbol not in self.symbols:
            self.symbols.append(symbol)
        return True
"""

  echo ""
  echo "原始代码:"
  echo sample_code

  let result = translator.translate_code(sample_code)

  echo ""
  echo "翻译后:"
  echo result

  translator.display_translation()
