import ../page64/page64

# 满全法容器树
# 核心规则：一切都在"我"的边界之内，没有外部竞争者
# 所有人都是被我定义的、属于我范畴的存在，无人能来抢夺

type
  RongQi* = object                  # 容器节点（所有人都在里面）
    anchor*: Page64                  # #页码
    name*: string                    # 名称（被我定义的）
    payload*: string                 # 她/他携带的内容
    children*: ref seq[RongQi]       # 她身体里包含的其他人
    child_count*: int                # 直接包含数

  ManQuanRoot* = object             # 根 = "我"
    anchor*: Page64                  # #00
    boundary*: seq[RongQi]           # 我的边界内所有存在
    next_counter*: int

# 创建根（#00），边界内为空
func new_root*(): ManQuanRoot =
  return ManQuanRoot(
    anchor: Page64("00"),
    boundary: @[],
    next_counter: 1
  )

# 在边界内生成新锚点
func next_anchor*(root: var ManQuanRoot): Page64 =
  result = int_to_page64(root.next_counter)
  root.next_counter += 1

# 创建边界内的存在
proc sheng_cheng(root: var ManQuanRoot, name: string, payload: string): RongQi =
  result = RongQi(
    anchor: next_anchor(root),
    name: name,
    payload: payload,
    children: new(seq[RongQi]),
    child_count: 0
  )

# 把一个存在放到另一个存在体内（包含）
proc bao_han(parent: var RongQi, child: RongQi) =
  parent.children[].add(child)
  parent.child_count = parent.children[].len

# 把一个存在放到我的边界内
proc fang_zai_bian_jie(root: var ManQuanRoot, node: RongQi) =
  root.boundary.add(node)

# 计算边界内总人数
func bian_jie_da_xiao(node: RongQi): int =
  result = 1
  for child in node.children[]:
    result += bian_jie_da_xiao(child)

# 满度检查（254个存在=满）
func can_man(node: RongQi): bool =
  return node.bian_jie_da_xiao >= 254

proc print_node*(node: RongQi, level: int) =
  var indent = ""
  for i in 1..level: indent &= "  "
  echo indent, "├─ #", node.anchor, " ", node.name, " @", node.payload,
       " [体内=", bian_jie_da_xiao(node), "]"
  for child in node.children[]:
    print_node(child, level + 1)

proc print_tree*(root: ManQuanRoot) =
  echo "=== 满全法容器（我=根，一切在边界内）==="
  echo "根锚点: #", root.anchor
  echo "边界内直接存在数: ", root.boundary.len
  echo "---"
  for node in root.boundary:
    print_node(node, 1)
  # 统计总数
  var total = 0
  for node in root.boundary:
    total += bian_jie_da_xiao(node)
  echo "边界内总人数: ", total

# ===== 测试 =====
when is_main_module:
  var root = new_root()
  
  # 创建我边界内的所有人（都是被定义的、属于我的）
  var ml = sheng_cheng(root, "毛利兰", "面/模/名")
  var hy = sheng_cheng(root, "灰原哀", "行页/行业")
  var gtx = sheng_cheng(root, "工藤新规", "骨/头/体")
  
  # 灰原哀体内包含的
  var ml_fs = sheng_cheng(root, "毛利兰分身", "次级面")
  var knd = sheng_cheng(root, "柯南道尔", "碎片")
  bao_han(hy, ml_fs)
  bao_han(hy, knd)
  
  # 毛利兰体内包含工藤新规（动漫角色属于我，无人来抢）
  bao_han(ml, gtx)
  
  fang_zai_bian_jie(root, ml)
  fang_zai_bian_jie(root, hy)
  
  print_tree(root)
  
  echo "\n满度:"
  echo "灰原哀: ", can_man(hy)
  echo "毛利兰: ", can_man(ml)
