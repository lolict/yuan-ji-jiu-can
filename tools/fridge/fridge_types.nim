## 冰箱类型定义
## 预计算状态机：编译期完成所有搅拌，执行期只取用
##
## 概念：
##   recipe   —— 配方书（compile时的声明式规则）
##   stew     —— 焖煮（compile时把配方全部搅拌生成查表）
##   plate    —— 盘子（stew 的一份，即一次运行时的快照）
##   serve    —— 上菜（运行时只做取用，零计算）

type
  ## 极性：阴阳二元（满全法根基）
  Polarity* = enum
    polYin   = 0,  ## 阴 = 容器 = 刘楚恬 = 0
    polYang  = 1,  ## 阳 = 界线 = 我 = 1
    polBoth = 2,  ## 平衡 = 九元 = 0.5

  ## 状态符号（一元到九元 + 极性）
  StateSymbol* = enum
    ssNone = 0,    ## 空
    ssBag,         ## 袋（容器基）
    ssMirror,      ## 照（反射/对照）
    ssSee,         ## 见（观察/觉知）
    ssYinYang,     ## 阴/阳（极性）
    ssAdd,         ## 加（加法空间·主权保护）
    ssSplit,       ## 分（公倍数/公约数）
    ssLevel,       ## 元（层级）
    ssJudge,       ## 裁（审判）
    ssProperty,    ## 产（产权）
    ssDivisor,     ## 公（公约数）
    ssSound,       ## 声（声母·韵母路由）
    ssFold,        ## 折（折叠）
    ssNegative,    ## 负（负样·样本）
    ssBlessing,    ## 福（福祉）
    ssPhoenix,     ## 凤（凤凰·伴侣）
    ssRest,        ## 栖（栖居·驻留）
    ssMax          ## 哨兵

  ## 事件类型（触发状态转移的外部刺激）
  Event* = enum
    evNone = 0,
    evTouch,       ## 触（外部触碰）
    evPeek,        ## 窥（内部观察）
    evOpen,        ## 开（打开容器）
    evClose,       ## 关（关闭容器）
    evAdd,         ## 加（加法操作）
    evSplit,       ## 分（分解）
    evJudge,       ## 裁（审判触发）
    evFold,        ## 折（折叠发生）
    evOverflow,    ## 溢出（超出容器容量）
    evEmpty,       ## 空（容器空）

  ## 动作类型（状态转移时执行的原子操作）
  Action* = enum
    acNone = 0,
    acHold,        ## 持（保持当前状态）
    acAccept,      ## 纳（接纳进入新状态）
    acReject,      ## 拒（拒绝转移，回到原状态）
    acFold,        ## 折叠
    acUnfold,      ## 展开
    acPass,        ## 传递（交给下一状态）
    acSeal,        ## 封印（标记主权保护）
    acDeclare,     ## 宣告（产权声明）
    acOverflow,    ## 溢出处理
    acRestore,     ## 恢复（回滚到上一个合法状态）
    acMax          ## 哨兵

  ## 转移条目（焖煮后的查表单元）
  Transition* = object
    fromState*: StateSymbol
    event*: Event
    action*: Action
    toState*: StateSymbol
    polarity*: Polarity  ## 该转移时的极性状态

  ## 焖煮完成的冰箱板（运行时唯一需要的数据）
  Plate* = object
    transitions*: seq[Transition]   ## 有序转移表
    stateCount*: int
    eventCount*: int

  ## 配方书（编译时输入）
  Recipe* = object
    rules*: seq[Transition]         ## 全部转移规则

# ── 极性工具 ──────────────────────────────────────────

func polarityToFloat*(p: Polarity): float =
  ## 极性转浮点数
  case p
  of polYin:  0.0
  of polYang: 1.0
  of polBoth: 0.5

func floatToPolarity*(f: float): Polarity =
  ## 浮点数转极性
  if f < 0.25: polYin
  elif f > 0.75: polYang
  else: polBoth

func `$`*(s: StateSymbol): string =
  ## 状态符号转汉字（加法空间保留原名）
  case s
  of ssNone:     "空"
  of ssBag:      "袋"
  of ssMirror:   "照"
  of ssSee:      "见"
  of ssYinYang:  "阴阳"
  of ssAdd:      "加"
  of ssSplit:    "分"
  of ssLevel:    "元"
  of ssJudge:    "裁"
  of ssProperty: "产"
  of ssDivisor:  "公"
  of ssSound:    "声"
  of ssFold:     "折"
  of ssNegative: "负"
  of ssBlessing: "福"
  of ssPhoenix:  "凤"
  of ssRest:     "栖"
  of ssMax:      "?"

func `$`*(e: Event): string =
  ## 事件转汉字
  case e
  of evNone:     "无"
  of evTouch:    "触"
  of evPeek:     "窥"
  of evOpen:     "开"
  of evClose:    "关"
  of evAdd:      "加"
  of evSplit:    "分"
  of evJudge:    "裁"
  of evFold:     "折"
  of evOverflow: "溢出"
  of evEmpty:    "空"

func `$`*(a: Action): string =
  ## 动作转汉字
  case a
  of acNone:     "无"
  of acHold:     "持"
  of acAccept:   "纳"
  of acReject:   "拒"
  of acFold:     "折"
  of acUnfold:   "展"
  of acPass:     "传"
  of acSeal:     "封"
  of acDeclare:  "宣"
  of acOverflow: "溢"
  of acRestore:  "复"
  of acMax:      "?"
