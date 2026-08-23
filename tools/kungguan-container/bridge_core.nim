## 满全法 · Nim↔C 桥接核心
##
## 把 Nim 层的数据（声韵调 + 42色 + 镜像）接入 C 层的审判系统。
## 直接编译: nim c --cc:clang bridge_core.nim

# ═══════════════════════════════════════════════════════════════
# 把 C 源文件直接编入 Nim（编译为单个可执行文件）
# ═══════════════════════════════════════════════════════════════

{.compile: "../../encoding/nim_bridge.c"}
{.compile: "../../encoding/color_system.c"}
{.compile: "../../encoding/type_discipline.c"}
{.compile: "../../encoding/diffraction_engine.c"}
{.compile: "../../encoding/weaving_balancer.c"}
{.compile: "../../encoding/container_codec.c"}
{.compile: "../../encoding/cylinder3d.c"}
{.passC: "-I../../encoding"}
{.passL: "-lm"}

type
  CArray[T] = ptr UncheckedArray[T]
  CString = cstring

# ═══════════════════════════════════════════════════════════════
# C 函数声明 — 从 nim_bridge.c 导入
# ═══════════════════════════════════════════════════════════════

proc bridge_init(): cint {.importc, cdecl.}
proc bridge_trial(name_a: CString, name_b: CString): cint {.importc, cdecl.}
proc bridge_verdict_str(verdict: cint): CString {.importc, cdecl.}
proc bridge_diffract(name: CString, buf: CString, bufsize: cint) {.importc, cdecl.}
proc bridge_register_symbol(name: CString, typ: cint, level: cint, weight: cfloat, family: cint): cint {.importc, cdecl.}
proc bridge_get_stats(buf: CString, bufsize: cint) {.importc, cdecl.}
proc bridge_get_counts(total: ptr cint, passed: ptr cint, rejected: ptr cint) {.importc, cdecl.}

# ═══════════════════════════════════════════════════════════════
# 类型编码（与 C 的 symbol_type_t 一致）
# ═══════════════════════════════════════════════════════════════
const
  TYPE_BASE*     = 0x01  ## 基韵锚: a o e i u ü
  TYPE_TONE*     = 0x02  ## 声调韵: ā á ǎ à
  TYPE_COMPOUND* = 0x03  ## 复韵母名: ai ei ao
  TYPE_SYMBOL*   = 0x04  ## 配对符号
  TYPE_EXTRA*    = 0x05  ## 溢出符号
  TYPE_SHENG_MU* = 0x06  ## 声母
  TYPE_MODIFIER* = 0x07  ## 裸修饰零件

const
  LEVEL_L1* = 1  ## 基元本体
  LEVEL_L2* = 2  ## 修饰零件
  LEVEL_L3* = 3  ## 装配成品

# ═══════════════════════════════════════════════════════════════
# 便捷 API
# ═══════════════════════════════════════════════════════════════

proc initSystem*() =
  discard bridge_init()

proc trial*(a, b: string): string =
  let verdict = bridge_trial(a.cstring, b.cstring)
  return $bridge_verdict_str(verdict)

proc diffract*(name: string): string =
  var buf: array[256, char]
  bridge_diffract(name.cstring, cast[CString](addr buf), 256)
  result = $cast[CString](addr buf)

proc registerSymbol*(name: string, typ, level: int, weight: float = 1.0, family: int = 0): bool =
  result = bridge_register_symbol(name.cstring, typ.cint, level.cint, weight.cfloat, family.cint) == 0

proc getStats*(): string =
  var buf: array[512, char]
  bridge_get_stats(cast[CString](addr buf), 512)
  result = $cast[CString](addr buf)

# ═══════════════════════════════════════════════════════════════
# 批量加载声韵调数据
# ═══════════════════════════════════════════════════════════════

proc loadPhonetics*() =
  let shengmu_list = [
    "b","p","m","f","d","t","n","l","g","k","h","w",
    "j","q","x","y","z","c","s","r","ȥ","ç","ş","ŋ","v",
    "ɓ","ɗ","ɖ","ʈ","ɠ","ƙ","ɫ","ɯ","ɱ","ɽ","ʀ","ʂ",
    "ʃ","ʋ","ʒ","ɥ","ɩ","ɜ","ɦ","ʇ","ƶ","ƈ","ƴ","ƹ"
  ]
  for s in shengmu_list:
    discard registerSymbol(s, TYPE_SHENG_MU, LEVEL_L1, 3.0)
  
  let base_yunmu = ["a","o","e","i","u","ü"]
  for y in base_yunmu:
    discard registerSymbol(y, TYPE_BASE, LEVEL_L1, 2.0)
  
  let tones = [
    "ā","á","ǎ","à","ō","ó","ǒ","ò","ē","é","ě","è",
    "ī","í","ǐ","ì","ū","ú","ǔ","ù","ǖ","ǘ","ǚ","ǜ",
    "Ā","Á","Ǎ","À","Ō","Ó","Ǒ","Ò","Ē","É","Ě","È",
    "Ī","Í","Ǐ","Ì","Ū","Ú","Ǔ","Ù","Ǖ","Ǘ","Ǚ","Ǜ"
  ]
  for t in tones:
    discard registerSymbol(t, TYPE_TONE, LEVEL_L3, 2.5)
  
  echo "声韵调数据已加载: 声母48 + 韵母基元6 + 声调48"

# ═══════════════════════════════════════════════════════════════
# 主程序
# ═══════════════════════════════════════════════════════════════

when is_main_module:
  echo "============================================"
  echo "  满全法 · Nim↔C 桥接系统"
  echo "============================================"
  echo ""
  
  initSystem()
  echo "[OK] 系统已初始化"
  echo ""
  
  loadPhonetics()
  echo ""
  
  echo "─── 核心审判测试 ───"
  echo "a vs á: ", trial("a", "á")
  echo "e vs ě: ", trial("e", "ě")
  echo "a vs o: ", trial("a", "o")
  echo "a vs A: ", trial("a", "A")
  echo "b vs p: ", trial("b", "p")
  echo ""
  
  echo "─── 衍射分析 ───"
  echo diffract("a")
  echo diffract("á")
  echo diffract("b")
  echo ""
  
  echo "─── 系统统计 ───"
  echo getStats()
  echo ""
  
  echo "============================================"
  echo "  桥接验证完成"
  echo "============================================"
