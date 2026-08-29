# MetaIME Nim FFI Bridge · 导出 Nim 实现为 C ABI
#
# 编译: nim c --app:lib --staticLib --noMain -o:libmetaime_nim_bridge.a metaime_nim_bridge.nim

import strutils
import options
import metaime/loader
import metaime/linker
import metaime/translator

# === 错误码 ===
const MI_OK* = 0
const MI_ERR_NOT_FOUND* = -1
const MI_ERR_FULL* = -2
const MI_ERR_INVALID* = -3

# === 加载器 ===
type MiLoader* = object
  inner*: FQMLoader

proc mi_loader_create*(): ptr MiLoader {.exportc, dynlib.} =
  result = create(MiLoader)
  result.inner = FQMLoader()

proc mi_loader_destroy*(l: ptr MiLoader) {.exportc, dynlib.} =
  if l != nil:
    dealloc(l)

proc mi_loader_load_default*(l: ptr MiLoader): cint {.exportc, dynlib.} =
  if l == nil: return MI_ERR_INVALID
  # 加载默认 FQM 数据
  let fqm_data = """袋.声母.容=b,p,m,f,d,t,n,l,g,k,h,j,q,x,zh,ch,sh,r,z,c,s,y,w
袋.韵母.容=a,o,e,i,u,v,ai,ei,ui,ao,ou,iu,ie,ve,er,an,en,in,un,vn,ang,eng,ing,ong
袋.声调.容=1声,2声,3声,4声,轻声"""
  discard l.inner.load_string(fqm_data)
  return MI_OK

proc mi_loader_container_count*(l: ptr MiLoader): cint {.exportc, dynlib.} =
  if l == nil: return 0
  return l.inner.container_count.cint

# === 翻译器 ===
type MiTranslator* = object
  inner*: FQMTranslator

proc mi_translator_create*(): ptr MiTranslator {.exportc, dynlib.} =
  result = create(MiTranslator)
  result.inner = FQMTranslator()

proc mi_translator_destroy*(t: ptr MiTranslator) {.exportc, dynlib.} =
  if t != nil:
    dealloc(t)

proc mi_translator_translate*(t: ptr MiTranslator, foreign: cstring, out_buf: pointer, out_len: csize_t): cint {.exportc, dynlib.} =
  if t == nil or foreign == nil or out_buf == nil: return MI_ERR_INVALID
  var tr = t.inner
  let trans_result = tr.translate_symbol($foreign)
  if isSome(trans_result):
    let s = get(trans_result)
    let len = min(s.len, out_len.int - 1)
    copyMem(out_buf, s[0].addr, len)
    cast[ptr UncheckedArray[char]](out_buf)[len] = '\0'
    return MI_OK
  return MI_ERR_NOT_FOUND

# === 公约数 ===
proc mi_gcd*(a, b: uint8): uint8 {.exportc, dynlib.} =
  var x = a
  var y = b
  while y != 0:
    let t = y
    y = x mod y
    x = t
  return x

proc mi_lcm*(a, b: uint8): uint16 {.exportc, dynlib.} =
  if a == 0 or b == 0: return 0
  return (a.uint16 * b.uint16) div mi_gcd(a, b).uint16
