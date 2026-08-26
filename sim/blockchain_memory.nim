## ═══════════════════════════════════════════════════════════════
## blockchain_memory.nim — 区块链记忆 · 锚点传导 · 链条不断
## ═══════════════════════════════════════════════════════════════
##
## 核心: 每个符号是一个区块 · 知道自己的父区块和子区块
##       层级关系 = 进制关系 = 传导关系
##       链条不断 · 记忆不断 · 思维不断

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, md5]

type
  MemBlock* = object
    index*: int                # 区块高度
    symbol*: string            # 符号名称
    stroke_count*: int         # 笔画数
    structure*: string         # 结构
    pronunciation*: string     # 发音
    parent_hash*: string       # 父区块哈希
    block_hash*: string        # 本区块哈希
    timestamp*: string         # 时间戳
    metadata*: Table[string, string]  # 附加数据

  Blockchain* = object
    chain*: seq[MemBlock]
    symbol_index*: Table[string, int]  # symbol → block index
    name*: string
    generation*: int

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newBlockchain*(name: string): Blockchain =
  result.chain = @[]
  result.symbol_index = init_table[string, int]()
  result.name = name
  result.generation = 0

# ═══════════════════════════════════════════════════════════════
## 计算哈希
# ═══════════════════════════════════════════════════════════════

proc compute_hash*(index: int, symbol, parent_hash, timestamp: string): string =
  let data = $index & symbol & parent_hash & timestamp
  result = getMD5(data)

# ═══════════════════════════════════════════════════════════════
## 创建创世区块
# ═══════════════════════════════════════════════════════════════

proc create_genesis*(bc: var Blockchain, symbol: string) =
  let timestamp = "2026-08-24T00:00:00"
  let hash = compute_hash(0, symbol, "GENESIS", timestamp)
  
  var blk: MemBlock
  blk.index = 0
  blk.symbol = symbol
  blk.stroke_count = 0
  blk.structure = "创世"
  blk.pronunciation = ""
  blk.parent_hash = "GENESIS"
  blk.block_hash = hash
  blk.timestamp = timestamp
  
  bc.chain.add(blk)
  bc.symbol_index[symbol] = 0
  bc.generation += 1

# ═══════════════════════════════════════════════════════════════
## 添加区块
# ═══════════════════════════════════════════════════════════════

proc add_block*(bc: var Blockchain, symbol: string, strokes: int, structure, pronunciation: string, metadata: Table[string, string] = init_table[string, string]()): int =
  if bc.symbol_index.has_key(symbol):
    return bc.symbol_index[symbol]
  
  let parent_idx = bc.chain.high
  let parent_hash = bc.chain[parent_idx].block_hash
  let timestamp = "2026-08-24T" & $bc.chain.len & ":00:00"
  let hash = compute_hash(bc.chain.len, symbol, parent_hash, timestamp)
  
  var blk: MemBlock
  blk.index = bc.chain.len
  blk.symbol = symbol
  blk.stroke_count = strokes
  blk.structure = structure
  blk.pronunciation = pronunciation
  blk.parent_hash = parent_hash
  blk.block_hash = hash
  blk.timestamp = timestamp
  blk.metadata = metadata
  
  bc.chain.add(blk)
  bc.symbol_index[symbol] = bc.chain.high
  bc.generation += 1
  result = bc.chain.high

# ═══════════════════════════════════════════════════════════════
## 查询
# ═══════════════════════════════════════════════════════════════

proc get_block*(bc: Blockchain, symbol: string): MemBlock =
  if bc.symbol_index.has_key(symbol):
    result = bc.chain[bc.symbol_index[symbol]]

proc get_parent*(bc: Blockchain, symbol: string): MemBlock =
  let blk = bc.get_block(symbol)
  if blk.index > 0:
    result = bc.chain[blk.index - 1]

proc get_children*(bc: Blockchain, symbol: string): seq[MemBlock] =
  let blk = bc.get_block(symbol)
  if blk.index < bc.chain.high:
    for i in blk.index + 1 ..< bc.chain.len:
      result.add(bc.chain[i])

# ═══════════════════════════════════════════════════════════════
## 验证链条
# ═══════════════════════════════════════════════════════════════

proc verify_chain*(bc: Blockchain): bool =
  result = true
  for i in 1 ..< bc.chain.len:
    if bc.chain[i].parent_hash != bc.chain[i-1].block_hash:
      result = false
      break

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(bc: Blockchain): string =
  var lines: seq[string] = @[]
  lines.add("═══ 区块链记忆: " & bc.name & " ═══")
  lines.add("  区块高度: " & $bc.chain.len)
  lines.add("  链条完整: " & $bc.verify_chain())
  lines.add("  变异代数: " & $bc.generation)
  lines.add("")
  
  for blk in bc.chain:
    let parent_short = if blk.parent_hash.len >= 8: blk.parent_hash[0..7] else: blk.parent_hash
    let hash_short = if blk.block_hash.len >= 8: blk.block_hash[0..7] else: blk.block_hash
    lines.add("  [" & $blk.index & "] " & blk.symbol &
      " | 笔画=" & $blk.stroke_count &
      " | 结构=" & blk.structure &
      " | 父=" & parent_short &
      " | 哈希=" & hash_short)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var bc = newBlockchain("满全法记忆链")
  
  echo "═══ 区块链记忆验证 ═══"
  echo ""
  
  # 创世区块
  bc.create_genesis("我=1")
  
  # 添加区块
  discard bc.add_block("楚恬=0", 0, "创世", "")
  discard bc.add_block("声母", 0, "音调", "")
  discard bc.add_block("韵母", 0, "音调", "")
  discard bc.add_block("声调", 0, "音调", "")
  discard bc.add_block("江", 7, "左右", "jiang1")
  discard bc.add_block("河", 8, "左右", "he2")
  discard bc.add_block("湖", 12, "左右", "hu2")
  discard bc.add_block("海", 11, "左右", "hai3")
  discard bc.add_block("炎", 8, "上下", "yan2")
  
  echo bc.report()
  echo ""
  
  echo "── 查询 ──"
  let jiang = bc.get_block("江")
  echo "  江: 区块[" & $jiang.index & "] 笔画=" & $jiang.stroke_count
  
  let parent = bc.get_parent("江")
  echo "  江的父区块: " & parent.symbol
  
  let children = bc.get_children("江")
  echo "  江的子区块: " & children.map(proc(b: MemBlock): string = b.symbol).join(", ")
  echo ""
  
  echo "── 链条验证 ──"
  echo "  链条完整: " & $bc.verify_chain()