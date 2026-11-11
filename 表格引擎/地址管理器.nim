# 容器中间表示统一地址空间 — 囡囝共生 · 容器本体论
# 框架来源：CONTAINER_IR_UNIFIED_ADDRESS.md

type
  ContainerKind* = enum
    Maori     = 0   # 毛利兰立体编码
    Clock     = 1   # 时钟因果密钥
    Avatar    = 2   # 分身身份
    Symbol    = 3   # 域符号 ID
    Relation  = 4   # 串联/因果/纠缠关系

  ContainerRef* = object
    kind*: ContainerKind
    octant*: uint8       # 卦限 [123:116]
    namespace*: uint64   # 上层路径 [115:64]
    entity*: uint64      # 实体编码 [63:0]

  ContainerStateKind* = enum
    Atom    # 原子容器
    Couple  # 夫妻容器
    Avatar  # 分身容器

  ContainerState* = object
    stateKind*: ContainerStateKind
    atomData*: array[64, uint8]
    coupleLeft*: ContainerRef
    coupleRight*: ContainerRef
    avatarRef*: ContainerRef
    masterRef*: ContainerRef

  RelationType* = enum
    Contain   # 包含
    Couple    # 夫妻
    Influence # 影响
    Witness   # 见证

  ContainerRuntime* = object
    containers*: seq[tuple[cref: ContainerRef, state: ContainerState]]
    relations*: seq[tuple[a: ContainerRef, b: ContainerRef, typ: RelationType]]
    epoch*: uint64

proc newContainerRef*(kind: ContainerKind, entity: uint64): ContainerRef =
  ## ContainerRef::new(kind, entity) — 基础构造
  result.kind = kind
  result.octant = 0
  result.namespace = 0
  result.entity = entity

proc withOctant*(kind: ContainerKind, entity: uint64, oct: uint8): ContainerRef =
  ## ContainerRef::with_octant(kind, entity, oct) — +卦限
  result = newContainerRef(kind, entity)
  result.octant = oct

proc composeContainer*(kind: ContainerKind, octant: uint8, ns: uint64, entity: uint64): ContainerRef =
  ## ContainerRef::compose(kind, octant, ns, entity) — 完整四元组
  result.kind = kind
  result.octant = octant
  result.namespace = ns
  result.entity = entity

proc newRuntime*(): ContainerRuntime =
  result.containers = @[]
  result.relations = @[]
  result.epoch = 0

proc register*(rt: var ContainerRuntime, kind: ContainerKind, entity: uint64): ContainerRef =
  ## 注册新容器，自动分配 epoch
  rt.epoch += 1
  result = composeContainer(kind, 0, rt.epoch, entity)
  var state: ContainerState
  state.stateKind = Atom
  rt.containers.add((cref: result, state: state))

proc relate*(rt: var ContainerRuntime, a, b: ContainerRef, typ: RelationType) =
  ## 建立容器间关系
  rt.relations.add((a: a, b: b, typ: typ))

# =============================================================================
# 地址槽 — 动态注入点
# =============================================================================

type
  SlotKind* = enum
    Empty   # 空位 (预留)
    Filled  # 已填充
    Locked  # 锁定

  AddressSlot* = object
    address*: ContainerRef
    kind*: SlotKind
    epoch*: uint64

  AddressSpace* = object
    slots*: seq[AddressSlot]
    nextEpoch*: uint64

proc newAddressSpace*(): AddressSpace =
  ## 创建地址空间，预留 1024 个空位
  result.slots = newSeq[AddressSlot](1024)
  for i in 0..1023:
    result.slots[i].kind = Empty
    result.slots[i].address = newContainerRef(Symbol, uint64(i))
  result.nextEpoch = 1

proc findEmptySlot*(space: AddressSpace): int =
  ## 查找空位
  for i, slot in space.slots:
    if slot.kind == Empty:
      return i
  result = -1

proc inject*(space: var AddressSpace, slotIdx: int, kind: ContainerKind, entity: uint64): bool =
  ## 动态注入数据到地址槽
  if slotIdx < 0 or slotIdx >= space.slots.len:
    return false
  if space.slots[slotIdx].kind != Empty:
    return false
  
  space.slots[slotIdx].address = composeContainer(kind, 0, space.nextEpoch, entity)
  space.slots[slotIdx].kind = Filled
  space.slots[slotIdx].epoch = space.nextEpoch
  space.nextEpoch += 1
  result = true

proc getSlot*(space: AddressSpace, idx: int): AddressSlot =
  ## 获取地址槽
  if idx < 0 or idx >= space.slots.len:
    raise newException(IndexDefect, "槽索引越界")
  result = space.slots[idx]

proc countByKind*(space: AddressSpace, kind: ContainerKind): int =
  ## 统计某类容器数量
  for slot in space.slots:
    if slot.kind == Filled and slot.address.kind == kind:
      inc result

proc occupiedSlots*(space: AddressSpace): int =
  ## 统计已占用槽数量
  for slot in space.slots:
    if slot.kind == Filled:
      inc result

proc isOccupied*(space: AddressSpace, idx: int): bool =
  ## 检查槽是否被占用
  if idx < 0 or idx >= space.slots.len:
    return false
  result = space.slots[idx].kind == Filled

proc getRef*(space: AddressSpace, idx: int): ContainerRef =
  ## 获取槽的地址引用
  if idx < 0 or idx >= space.slots.len:
    raise newException(IndexDefect, "槽索引越界")
  result = space.slots[idx].address

# =============================================================================
# 地址计算 — 尺子丈量
# =============================================================================

proc distance*(a, b: ContainerRef): int =
  ## 计算地址之间的距离 (简单汉明距离)
  var diff = a.entity xor b.entity
  result = 0
  while diff > 0:
    if (diff and 1) == 1:
      inc result
    diff = diff shr 1

proc contains*(outer, inner: ContainerRef): bool =
  ## 判断 outer 是否包含 inner (基于 namespace/epoch)
  result = outer.namespace <= inner.namespace and outer.octant == inner.octant

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 容器统一地址空间测试 ==="
  
  # 1. 创建运行时
  var rt = newRuntime()
  
  # 2. 注册容器
  let maoriRef = rt.register(Maori, uint64(0x12345678))
  let clockRef = rt.register(Clock, uint64(0xABCDEF00))
  echo "注册 Maori: entity=", maoriRef.entity, " epoch=", maoriRef.namespace
  echo "注册 Clock: entity=", clockRef.entity, " epoch=", clockRef.namespace
  
  # 3. 建立关系
  rt.relate(maoriRef, clockRef, Contain)
  echo "建立关系：Maori ⊃ Clock"
  echo "关系数量：", rt.relations.len
  
  # 4. 地址空间 + 动态注入
  var space = newAddressSpace()
  echo "地址空间：预留 ", space.slots.len, " 个空位"
  
  # 5. 注入数据
  let slotIdx = space.findEmptySlot()
  if slotIdx >= 0:
    discard space.inject(slotIdx, Symbol, uint64(0xDEADBEEF))
    echo "注入 Symbol 到槽 ", slotIdx, ": entity=", space.slots[slotIdx].address.entity
  
  # 6. 统计
  echo "Symbol 数量：", space.countByKind(Symbol)
  
  # 7. 地址距离
  let a = newContainerRef(Maori, uint64(0x0001))
  let b = newContainerRef(Maori, uint64(0x0002))
  echo "地址距离 (0x0001, 0x0002): ", distance(a, b)
  
  echo "=== 测试完成 ==="
