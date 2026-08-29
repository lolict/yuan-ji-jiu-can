# ============================================================
# FQM MetaIME · 统一入口
# 重新导出所有模块
# ============================================================

import ../nim/fqm_constants
import ../nim/loader
import ../nim/linker
import ../nim/translator
import ../nim/interpreter
import ../nim/radix3d
import ../nim/dimension
import ../nim/divisor
import ../nim/yuan

export fqm_constants
export loader
export linker
export translator
export interpreter
export radix3d
export dimension
export divisor
export yuan

# ============================================================
# 主程序入口
# ============================================================

when is_main_module:
  import strutils
  from options import isSome, get

  echo "============================================================"
  echo "FQM MetaIME · Nim 零运行时内存版"
  echo "============================================================"

  # 加载器测试
  var fqm_loader: FQMLoader
  discard fqm_loader.load_string("""袋.声母.容=b,p,m,f
袋.韵母.容=a,o,e,i
袋.声调.容=1,2,3,4
袋.声母.引.韵母=b→a,p→o
偏=0x00""")
  fqm_loader.display_summary()

  # 链接器测试
  var fqm_linker: FQMLinker
  fqm_linker.loader = addr fqm_loader
  discard fqm_linker.build_ontology_index()
  echo "本体论索引: ", fqm_linker.stp_count, " 条"

  # 翻译器测试
  echo ""
  echo "翻译示例:"
  var tr: FQMTranslator
  let t1 = tr.translate_symbol("class")
  echo "  class → ", if isSome(t1): get(t1) else: "未知"
  let t2 = tr.translate_symbol("def")
  echo "  def → ", if isSome(t2): get(t2) else: "未知"
  let t3 = tr.translate_symbol("unknown")
  echo "  unknown → ", if isSome(t3): get(t3) else: "未知"

  echo ""
  echo "============================================================"
  echo "所有模块加载完成"
  echo "============================================================"
