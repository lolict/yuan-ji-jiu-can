# ============================================================
# FQM 加载器 · 零运行时内存版
# 读取 .fqm 容器定义文件，输出结构化数据
# 所有容器/符号/链接使用固定宽度数组栈分配
# ============================================================

import strutils
import os
import ../nim/fqm_constants

# ============================================================
# 数据类型定义（栈分配，固定宽度）
# ============================================================

type
  # 符号条目
  SymbolEntry* = object
    symbol*: array[MAX_SYMBOL_LEN, char]
    symbol_len*: int
    position*: int

  # 容器定义
  Container* = object
    name*: array[MAX_NAME_LEN, char]
    name_len*: int
    container_type*: array[MAX_NAME_LEN, char]
    type_len*: int
    symbols*: array[MAX_SYMBOLS, SymbolEntry]
    symbol_count*: int
    capacity*: int

  # 链接关系
  Link* = object
    from_container*: array[MAX_NAME_LEN, char]
    from_container_len*: int
    from_symbol*: array[MAX_SYMBOL_LEN, char]
    from_symbol_len*: int
    to_container*: array[MAX_NAME_LEN, char]
    to_container_len*: int
    to_symbol*: array[MAX_SYMBOL_LEN, char]
    to_symbol_len*: int

  # 近义词
  SynonymEntry* = object
    standard*: array[MAX_SYMBOL_LEN, char]
    standard_len*: int
    variants*: array[8, array[MAX_SYMBOL_LEN, char]]
    variant_lens*: array[8, int]
    variant_count*: int

  # FQM 加载器
  FQMLoader* = object
    containers*: array[MAX_CONTAINERS, Container]
    container_count*: int
    links*: array[MAX_LINKS, Link]
    link_count*: int
    synonyms*: array[MAX_SYNONYMS, SynonymEntry]
    synonym_count*: int
    offset*: int
    errors*: array[32, array[64, char]]
    error_lens*: array[32, int]
    error_count*: int

# ============================================================
# 通用辅助函数（编译时多态，零运行时开销）
# ============================================================

proc set_arr*[N: static int](dst: var array[N, char], len: var int, src: string) =
  let slen = min(src.len, N - 1)
  for i in 0..<slen:
    dst[i] = src[i]
  dst[slen] = '\0'
  len = slen

proc get_arr*[N: static int](arr: array[N, char], len: int): string =
  result = newString(len)
  for i in 0..<len:
    result[i] = arr[i]

proc arr_eq*[N: static int](arr: array[N, char], len: int, s: string): bool =
  if len != s.len: return false
  for i in 0..<len:
    if arr[i] != s[i]: return false
  return true

# 兼容旧名的别名
template set_str(dst: var array[MAX_SYMBOL_LEN, char], len: var int, src: string) =
  set_arr(dst, len, src)

template set_str24(dst: var array[MAX_NAME_LEN, char], len: var int, src: string) =
  set_arr(dst, len, src)

template get_str(arr: array[MAX_SYMBOL_LEN, char], len: int): string =
  get_arr(arr, len)

template get_str24(arr: array[MAX_NAME_LEN, char], len: int): string =
  get_arr(arr, len)

template get_str64(arr: array[64, char], len: int): string =
  get_arr(arr, len)

template str_eq(arr: array[MAX_SYMBOL_LEN, char], len: int, s: string): bool =
  arr_eq(arr, len, s)

template str24_eq(arr: array[MAX_NAME_LEN, char], len: int, s: string): bool =
  arr_eq(arr, len, s)

# ============================================================
# 容器操作
# ============================================================

proc find_container*(loader: var FQMLoader, name: string): int =
  for i in 0..<loader.container_count:
    if str24_eq(loader.containers[i].name, loader.containers[i].name_len, name):
      return i
  return -1

proc get_container*(loader: var FQMLoader, name: string): ptr Container =
  let idx = find_container(loader, name)
  if idx >= 0:
    return addr loader.containers[idx]
  return nil

proc add_symbol*(container: var Container, sym: string) =
  for i in 0..<container.symbol_count:
    if str_eq(container.symbols[i].symbol, container.symbols[i].symbol_len, sym):
      return
  if container.symbol_count < MAX_SYMBOLS:
    let pos = container.symbol_count
    set_str(container.symbols[pos].symbol, container.symbols[pos].symbol_len, sym)
    container.symbols[pos].position = pos
    container.symbol_count += 1

# ============================================================
# 行解析
# ============================================================

proc parse_line(loader: var FQMLoader, line: string, line_num: int) =
  let stripped = strip(line)
  if stripped.len == 0 or stripped[0] == '#':
    return

  let eq_pos = stripped.find('=')
  if eq_pos < 0:
    if loader.error_count < 32:
      let msg = "缺少 '=' 分隔符: " & stripped
      set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
      loader.error_count += 1
    return

  let left = strip(stripped[0..<eq_pos])
  let right = strip(stripped[eq_pos+1..^1])

  # 全局偏移: 偏=0x00
  if left == ATTR_OFFSET:
    try:
      if right.len > 2 and (right[0] == '0') and (right[1] == 'x' or right[1] == 'X'):
        loader.offset = fromHex[int](right)
      else:
        loader.offset = parseInt(right)
    except:
      if loader.error_count < 32:
        let msg = "无效的偏移量: " & right
        set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
        loader.error_count += 1
    return

  # 近义词: 近.标准=变体1,变体2
  if left.len > 2 and left.startsWith("近."):
    let standard = left[2..^1]
    let variants = right.split(',')
    if loader.synonym_count < MAX_SYNONYMS:
      let idx = loader.synonym_count
      set_str(loader.synonyms[idx].standard, loader.synonyms[idx].standard_len, standard)
      loader.synonyms[idx].variant_count = 0
      for v in variants:
        let vs = strip(v)
        if vs.len > 0 and loader.synonyms[idx].variant_count < 8:
          let vi = loader.synonyms[idx].variant_count
          set_str(loader.synonyms[idx].variants[vi], loader.synonyms[idx].variant_lens[vi], vs)
          loader.synonyms[idx].variant_count += 1
      loader.synonym_count += 1
    return

  # 袋.容器名.属性=值
  if left.len > 3 and left.startsWith("袋."):
    let parts = left.split('.')
    if parts.len < 3:
      if loader.error_count < 32:
        let msg = "格式错误: " & left
        set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
        loader.error_count += 1
      return

    let container_name = parts[1]
    let attribute = parts[2]

    var idx = find_container(loader, container_name)
    if idx < 0:
      if loader.container_count >= MAX_CONTAINERS:
        return
      idx = loader.container_count
      set_str24(loader.containers[idx].name, loader.containers[idx].name_len, container_name)
      loader.containers[idx].symbol_count = 0
      loader.containers[idx].capacity = 0
      loader.container_count += 1

    if attribute == ATTR_CONTAINER:
      let symbols = right.split(',')
      for sym in symbols:
        let s = strip(sym)
        if s.len > 0:
          add_symbol(loader.containers[idx], s)

    elif attribute == ATTR_RADIX:
      try:
        loader.containers[idx].capacity = parseInt(right)
      except:
        if loader.error_count < 32:
          let msg = "无效进制: " & right
          set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
          loader.error_count += 1

    elif attribute == ATTR_POLARITY:
      set_str24(loader.containers[idx].container_type, loader.containers[idx].type_len,
                container_name & "(极性=" & right & ")")

    elif attribute == ATTR_LINK:
      if parts.len < 4:
        if loader.error_count < 32:
          let msg = "引用格式错误: " & left
          set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
          loader.error_count += 1
        return
      let target_container = parts[3]
      let pairs = right.split(',')
      for pair in pairs:
        let p = strip(pair)
        if p.len == 0: continue
        let arrow_pos = p.find("→")
        if arrow_pos < 0:
          if loader.error_count < 32:
            let msg = "引用缺少 →: " & p
            set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
            loader.error_count += 1
          continue
        if loader.link_count >= MAX_LINKS:
          return
        let li = loader.link_count
        let from_sym = strip(p[0..<arrow_pos])
        let to_sym = strip(p[arrow_pos+1..^1])
        set_str24(loader.links[li].from_container, loader.links[li].from_container_len, container_name)
        set_str(loader.links[li].from_symbol, loader.links[li].from_symbol_len, from_sym)
        set_str24(loader.links[li].to_container, loader.links[li].to_container_len, target_container)
        set_str(loader.links[li].to_symbol, loader.links[li].to_symbol_len, to_sym)
        loader.link_count += 1
    return

  if loader.error_count < 32:
    let msg = "未知声明类型: " & left
    set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
    loader.error_count += 1

# ============================================================
# 文件加载
# ============================================================

proc load_file*(loader: var FQMLoader, filepath: string): bool =
  loader.container_count = 0
  loader.link_count = 0
  loader.synonym_count = 0
  loader.offset = 0
  loader.error_count = 0

  if not fileExists(filepath):
    if loader.error_count < 32:
      let msg = "文件不存在: " & filepath
      set_arr(loader.errors[loader.error_count], loader.error_lens[loader.error_count], msg)
      loader.error_count += 1
    return false

  let content = readFile(filepath)
  let lines = content.split('\n')
  for i, line in lines:
    parse_line(loader, line, i + 1)

  return loader.error_count == 0

proc load_string*(loader: var FQMLoader, content: string): bool =
  loader.container_count = 0
  loader.link_count = 0
  loader.synonym_count = 0
  loader.offset = 0
  loader.error_count = 0

  let lines = content.split('\n')
  for i, line in lines:
    parse_line(loader, line, i + 1)

  return loader.error_count == 0

# ============================================================
# 查询接口
# ============================================================

proc get_all_containers*(loader: FQMLoader): ptr Container =
  if loader.container_count > 0:
    return unsafeAddr loader.containers[0]
  return nil

proc get_links*(loader: FQMLoader): ptr Link =
  if loader.link_count > 0:
    return unsafeAddr loader.links[0]
  return nil

proc get_synonyms*(loader: FQMLoader): ptr SynonymEntry =
  if loader.synonym_count > 0:
    return unsafeAddr loader.synonyms[0]
  return nil

# ============================================================
# 显示摘要
# ============================================================

proc display_summary*(loader: FQMLoader) =
  echo ""
  echo "============================================================"
  echo "FQM 加载结果"
  echo "------------------------------------------------------------"
  echo "容器数量: ", loader.container_count
  echo "链接数量: ", loader.link_count
  echo "近义词数量: ", loader.synonym_count
  echo "全局偏移: 0x", toHex(loader.offset, 2)

  if loader.container_count > 0:
    echo ""
    echo "容器列表:"
    for i in 0..<loader.container_count:
      let name = get_str24(loader.containers[i].name, loader.containers[i].name_len)
      echo "  ", name, ": ", loader.containers[i].symbol_count, " 个符号"

  if loader.link_count > 0:
    echo ""
    echo "链接列表:"
    let show_count = min(loader.link_count, 10)
    for i in 0..<show_count:
      let fc = get_str24(loader.links[i].from_container, loader.links[i].from_container_len)
      let fs = get_str(loader.links[i].from_symbol, loader.links[i].from_symbol_len)
      let tc = get_str24(loader.links[i].to_container, loader.links[i].to_container_len)
      let ts = get_str(loader.links[i].to_symbol, loader.links[i].to_symbol_len)
      echo "  ", fc, ".", fs, " → ", tc, ".", ts
    if loader.link_count > 10:
      echo "  ... (共 ", loader.link_count, " 条)"

  if loader.error_count > 0:
    echo ""
    echo "错误 (", loader.error_count, "):"
    let show_err = min(loader.error_count, 5)
    for i in 0..<show_err:
      echo "  ! ", get_arr(loader.errors[i], loader.error_lens[i])

  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when is_main_module:
  if paramCount() < 1:
    echo "用法: nim c -r loader.nim <fqm文件>"
    echo "示例: nim c -r loader.nim containers.fqm"
    quit(1)

  var loader: FQMLoader
  let success = loader.load_file(paramStr(1))
  loader.display_summary()

  if not success:
    echo "加载失败，请检查错误"
    quit(1)
