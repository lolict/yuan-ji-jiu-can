# ============================================================
# FQM 链接器 · 本体论量化版 · 零运行时内存
# 不用代数地址，用本体论路径（域.身份）
# ============================================================

import strutils
import options
import std/os
import ../nim/fqm_constants
import ../nim/loader

# ============================================================
# 数据类型
# ============================================================

type
  # 本体论路径
  OntologyPath* = object
    domain*: array[MAX_NAME_LEN, char]
    domain_len*: int
    identity*: array[MAX_NAME_LEN, char]
    identity_len*: int

  # 本体论索引项
  OntologyEntry* = object
    path*: OntologyPath
    symbol*: array[MAX_SYMBOL_LEN, char]
    symbol_len*: int

  # 符号→路径映射
  SymbolToPath* = object
    domain*: array[MAX_NAME_LEN, char]
    domain_len*: int
    symbol*: array[MAX_SYMBOL_LEN, char]
    symbol_len*: int
    path*: OntologyPath

  # 已解析链接
  ResolvedLink* = object
    from_domain*: array[MAX_NAME_LEN, char]
    from_domain_len*: int
    from_symbol*: array[MAX_SYMBOL_LEN, char]
    from_symbol_len*: int
    from_path*: OntologyPath
    to_domain*: array[MAX_NAME_LEN, char]
    to_domain_len*: int
    to_symbol*: array[MAX_SYMBOL_LEN, char]
    to_symbol_len*: int
    to_path*: OntologyPath

  # FQM 链接器
  FQMLinker* = object
    loader*: ptr FQMLoader
    ontology_index*: array[MAX_SYMBOLS, OntologyEntry]
    ontology_count*: int
    symbol_to_path*: array[MAX_SYMBOLS, SymbolToPath]
    stp_count*: int
    resolved*: array[MAX_LINKS, ResolvedLink]
    resolved_count*: int
    unresolved*: array[MAX_LINKS, array[64, char]]
    unresolved_lens*: array[MAX_LINKS, int]
    unresolved_count*: int
    conflicts*: array[32, array[64, char]]
    conflict_lens*: array[32, int]
    conflict_count*: int

# ============================================================
# 辅助函数
# ============================================================

proc set_path(path: var OntologyPath, domain, identity: string) =
  set_arr(path.domain, path.domain_len, domain)
  set_arr(path.identity, path.identity_len, identity)

proc path_eq(a, b: OntologyPath): bool =
  result = arr_eq(a.domain, a.domain_len, get_arr(b.domain, b.domain_len)) and
           arr_eq(a.identity, a.identity_len, get_arr(b.identity, b.identity_len))

proc path_str*(path: OntologyPath): string =
  result = get_arr(path.domain, path.domain_len) & "." & get_arr(path.identity, path.identity_len)

# ============================================================
# 构建本体论索引
# ============================================================

proc build_ontology_index*(linker: var FQMLinker): bool =
  linker.ontology_count = 0
  linker.stp_count = 0
  linker.conflict_count = 0

  let loader = linker.loader

  for i in 0..<loader.container_count:
    let container = addr loader.containers[i]
    let domain_name = get_arr(container.name, container.name_len)

    for j in 0..<container.symbol_count:
      let symbol = get_arr(container.symbols[j].symbol, container.symbols[j].symbol_len)
      let identity_idx = container.symbols[j].position

      let identity = if identity_idx < 24: IDENTITY_NAMES[identity_idx]
                    else: "第" & $(identity_idx + 1)

      var path: OntologyPath
      set_path(path, domain_name, identity)

      var duplicate = false
      for k in 0..<linker.stp_count:
        if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, domain_name) and
           arr_eq(linker.symbol_to_path[k].symbol, linker.symbol_to_path[k].symbol_len, symbol):
          duplicate = true
          if linker.conflict_count < 32:
            let msg = "重复定义: " & domain_name & "." & symbol
            set_arr(linker.conflicts[linker.conflict_count], linker.conflict_lens[linker.conflict_count], msg)
            linker.conflict_count += 1
          break

      if not duplicate:
        if linker.ontology_count < MAX_SYMBOLS:
          linker.ontology_index[linker.ontology_count].path = path
          set_arr(linker.ontology_index[linker.ontology_count].symbol,
                  linker.ontology_index[linker.ontology_count].symbol_len, symbol)
          linker.ontology_count += 1

        if linker.stp_count < MAX_SYMBOLS:
          set_arr(linker.symbol_to_path[linker.stp_count].domain,
                    linker.symbol_to_path[linker.stp_count].domain_len, domain_name)
          set_arr(linker.symbol_to_path[linker.stp_count].symbol,
                  linker.symbol_to_path[linker.stp_count].symbol_len, symbol)
          linker.symbol_to_path[linker.stp_count].path = path
          linker.stp_count += 1

  return linker.conflict_count == 0

# ============================================================
# 解析跨容器链接
# ============================================================

proc resolve_links*(linker: var FQMLinker): bool =
  linker.resolved_count = 0
  linker.unresolved_count = 0

  let loader = linker.loader

  for i in 0..<loader.link_count:
    let link = addr loader.links[i]
    let from_c = get_arr(link.from_container, link.from_container_len)
    let from_s = get_arr(link.from_symbol, link.from_symbol_len)
    let to_c = get_arr(link.to_container, link.to_container_len)
    let to_s = get_arr(link.to_symbol, link.to_symbol_len)

    var from_path: OntologyPath
    var from_found = false
    for k in 0..<linker.stp_count:
      if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, from_c) and
         arr_eq(linker.symbol_to_path[k].symbol, linker.symbol_to_path[k].symbol_len, from_s):
        from_path = linker.symbol_to_path[k].path
        from_found = true
        break

    if not from_found:
      if linker.unresolved_count < MAX_LINKS:
        let msg = "未定义: " & from_c & "." & from_s
        set_arr(linker.unresolved[linker.unresolved_count],
                linker.unresolved_lens[linker.unresolved_count], msg)
        linker.unresolved_count += 1
      continue

    var to_path: OntologyPath
    var to_found = false
    for k in 0..<linker.stp_count:
      if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, to_c) and
         arr_eq(linker.symbol_to_path[k].symbol, linker.symbol_to_path[k].symbol_len, to_s):
        to_path = linker.symbol_to_path[k].path
        to_found = true
        break

    if not to_found:
      if linker.unresolved_count < MAX_LINKS:
        let msg = "未定义: " & to_c & "." & to_s
        set_arr(linker.unresolved[linker.unresolved_count],
                linker.unresolved_lens[linker.unresolved_count], msg)
        linker.unresolved_count += 1
      continue

    if linker.resolved_count < MAX_LINKS:
      let ri = linker.resolved_count
      set_arr(linker.resolved[ri].from_domain, linker.resolved[ri].from_domain_len, from_c)
      set_arr(linker.resolved[ri].from_symbol, linker.resolved[ri].from_symbol_len, from_s)
      linker.resolved[ri].from_path = from_path
      set_arr(linker.resolved[ri].to_domain, linker.resolved[ri].to_domain_len, to_c)
      set_arr(linker.resolved[ri].to_symbol, linker.resolved[ri].to_symbol_len, to_s)
      linker.resolved[ri].to_path = to_path
      linker.resolved_count += 1

  return linker.unresolved_count == 0

# ============================================================
# 查找接口
# ============================================================

proc lookup*(linker: FQMLinker, domain: string, symbol: string): Option[OntologyPath] =
  for k in 0..<linker.stp_count:
    if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, domain) and
       arr_eq(linker.symbol_to_path[k].symbol, linker.symbol_to_path[k].symbol_len, symbol):
      return some(linker.symbol_to_path[k].path)
  return none(OntologyPath)

proc find_identity*(linker: FQMLinker, domain: string, symbol: string): Option[string] =
  for k in 0..<linker.stp_count:
    if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, domain) and
       arr_eq(linker.symbol_to_path[k].symbol, linker.symbol_to_path[k].symbol_len, symbol):
      return some(get_arr(linker.symbol_to_path[k].path.identity,
                            linker.symbol_to_path[k].path.identity_len))
  return none(string)

proc find_symbol*(linker: FQMLinker, domain: string, identity: string): Option[string] =
  for k in 0..<linker.stp_count:
    if arr_eq(linker.symbol_to_path[k].domain, linker.symbol_to_path[k].domain_len, domain) and
       arr_eq(linker.symbol_to_path[k].path.identity, linker.symbol_to_path[k].path.identity_len, identity):
      return some(get_arr(linker.symbol_to_path[k].symbol,
                          linker.symbol_to_path[k].symbol_len))
  return none(string)

# ============================================================
# 显示
# ============================================================

proc display_ontology*(linker: FQMLinker) =
  echo ""
  echo "============================================================"
  echo "本体论树"
  echo "------------------------------------------------------------"

  var shown_domains: array[MAX_CONTAINERS, array[MAX_NAME_LEN, char]]
  var shown_lens: array[MAX_CONTAINERS, int]
  var shown_count = 0

  for i in 0..<linker.ontology_count:
    let domain = get_arr(linker.ontology_index[i].path.domain,
                           linker.ontology_index[i].path.domain_len)
    var already_shown = false
    for s in 0..<shown_count:
      if arr_eq(shown_domains[s], shown_lens[s], domain):
        already_shown = true
        break
    if already_shown: continue

    if shown_count < MAX_CONTAINERS:
      set_arr(shown_domains[shown_count], shown_lens[shown_count], domain)
      shown_count += 1

    echo ""
    echo "  ┌─ ", domain
    for j in 0..<linker.ontology_count:
      let d = get_arr(linker.ontology_index[j].path.domain,
                        linker.ontology_index[j].path.domain_len)
      if d == domain:
        let identity = get_arr(linker.ontology_index[j].path.identity,
                                  linker.ontology_index[j].path.identity_len)
        let symbol = get_arr(linker.ontology_index[j].symbol,
                             linker.ontology_index[j].symbol_len)
        echo "  │  ", identity, ": ", symbol
    echo "  └─"

  echo ""
  echo "============================================================"
  echo ""

proc display_resolution*(linker: FQMLinker) =
  echo ""
  echo "============================================================"
  echo "本体论链接"
  echo "------------------------------------------------------------"
  echo "已解析: ", linker.resolved_count
  echo "未解析: ", linker.unresolved_count
  echo "冲突: ", linker.conflict_count

  if linker.resolved_count > 0:
    echo ""
    echo "本质连接:"
    for i in 0..<linker.resolved_count:
      echo "  ", path_str(linker.resolved[i].from_path), " → ",
           path_str(linker.resolved[i].to_path)

  if linker.unresolved_count > 0:
    echo ""
    echo "未解析:"
    for i in 0..<linker.unresolved_count:
      echo "  ! ", get_arr(linker.unresolved[i], linker.unresolved_lens[i])

  if linker.conflict_count > 0:
    echo ""
    echo "冲突:"
    for i in 0..<linker.conflict_count:
      echo "  ✗ ", get_arr(linker.conflicts[i], linker.conflict_lens[i])

  echo "============================================================"
  echo ""

# ============================================================
# 入口
# ============================================================

when false:
  let cmdArgs = commandLineParams()
  if cmdArgs.len < 1:
    echo "用法: nim c -r linker.nim <fqm文件>"
    quit(1)

  var myLoader: FQMLoader
  discard myLoader.load_file(cmdArgs[0])

  myLoader.display_summary()

  var myLinker: FQMLinker
  myLinker.loader = addr myLoader
  discard myLinker.build_ontology_index()
  discard myLinker.resolve_links()

  myLinker.display_ontology()
  myLinker.display_resolution()
