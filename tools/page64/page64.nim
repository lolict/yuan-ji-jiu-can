# 64进制页码系统 (64-base Page Numbering)
# 核心规则：
# # = 常量锚点 = 位置 = 64进制页码
# @ = 变量内容 = 张量 = 被常量约束的内容
# 全局只需一次 #00 根初始化，后续页码自动递增
# 层级由64进制自然进位产生，不靠语义标题

type
  Page64* = distinct string          # 64进制页码，纯字符串表示
  Tensor* = object                   # @ 张量 = 内容
    data*: string                    # 内容数据
    created_at*: int                 # 时序
    anchor*: Page64                  # 所属锚点

  PageSystem* = object               # 整个页码体系
    root*: Page64                    # #00 根锚点
    next_counter*: int               # 下一个页码计数器
    tensors*: seq[Tensor]            # 所有已挂载的张量
    initialized*: bool               # 是否已初始化

# 64进制字符表：0-9, A-Z, a-z, +, / （标准Base64字符集）
const BASE64_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/"

# Page64 转字符串
func `$`*(p: Page64): string = string(p)

# 整数转64进制字符串
func int_to_page64*(n: int): Page64 =
  if n == 0: return Page64("00")
  var num = n
  var result = ""
  while num > 0:
    result = $BASE64_CHARS[num mod 64] & result
    num = num div 64
  # 补零到偶数位
  if result.len mod 2 != 0: result = "0" & result
  return Page64(result)

# 创建并初始化页码系统（仅需一次）
func init_page_system*(): PageSystem =
  return PageSystem(
    root: Page64("00"),           # 根锚点
    next_counter: 1,               # 下一个从01开始
    tensors: @[],
    initialized: true
  )

# 自动生成下一个页码
func next_page*(sys: var PageSystem): Page64 =
  let page = int_to_page64(sys.next_counter)
  sys.next_counter += 1
  return page

# 挂载 @ 张量到页码
func mount*(sys: var PageSystem, content: string): Page64 =
  let page = next_page(sys)
  sys.tensors.add(Tensor(
    data: content,
    created_at: sys.next_counter - 1,
    anchor: page
  ))
  return page

# 获取常量锚点 (#操作)
func get_anchor*(sys: PageSystem, page: Page64): int =
  # 从锚点路径计算层级深度
  return ($page).len div 2

# 生成层级路径（纯64进制点分表示）
func path_to_string*(pages: seq[Page64]): string =
  for i, p in pages:
    if i > 0: result &= "."
    result &= $p

# ===== 测试 =====
when is_main_module:
  echo "=== 64进制页码系统测试 ==="
  
  # 一次性初始化
  var sys = init_page_system()
  echo "根锚点: #", sys.root
  echo "初始化完成: ", sys.initialized
  echo ""
  
  # 自动挂载内容
  let p1 = sys.mount("妻=妻子道 夫=丈夫道")
  echo "挂载 [妻=妻子道 夫=丈夫道] → #", p1
  
  let p2 = sys.mount("漂亮姑娘容器")
  echo "挂载 [漂亮姑娘容器] → #", p2
  
  let p3 = sys.mount("容器公约数=1 公倍数=64")
  echo "挂载 [公约数=1 公倍数=64] → #", p3
  
  let p4 = sys.mount("灰原哀=容器节点 毛利兰=空间索引")
  echo "挂载 [灰原哀/毛利兰] → #", p4
  
  echo ""
  echo "=== 页码递增验证 ==="
  echo "计数器: ", sys.next_counter
  echo "已挂载张量数: ", sys.tensors.len
  
  # 展示自动生成的页码序列
  echo ""
  echo "=== 页码序列（纯64进制，无标题）==="
  for t in sys.tensors:
    echo "  #", t.anchor, " @ ", t.data
  
  echo ""
  echo "层级深度示例:"
  echo "  #00 根:", get_anchor(sys, sys.root), "层"
  echo "  #", p1, ":", get_anchor(sys, p1), "层"
  echo "  #", p2, ":", get_anchor(sys, p2), "层"
  
  # 测试更多页码进位
  echo ""
  echo "=== 64进制进位测试 ==="
  for i in 60..70:
    echo "  ", i, " → #", int_to_page64(i)
