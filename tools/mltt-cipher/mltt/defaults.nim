# ============================================================
# mltt 命名密码表 · 默认数据
#
# 家庭命名密码表（来自方法论）：
#   mlll = 集装箱, gtt = 音韵态射, ml.gy = 世俗归一
#   mlct → mltt = 岔途→从体, mlnn = 囡囝, lctqf = 零岔途全法
#
# 动漫角色映射：
#   毛利兰 = 模/面/名（表面属性）
#   工藤新一 = 骨/头/体（结构属性）
#   灰原哀 = 行/页/业（横向属性）
# ============================================================

import ./types
import ./cipher

# 加载默认密码表
proc load_defaults*(): CipherTable =
  var table = new_cipher_table()
  
  # ── 容器家族 ──
  discard table.register(CipherEntry(
    code: "mlll", meaning: "集装箱", polarity: yin,
    category: "容器",
    description: "254+2根基编码，袋子级操作的基本单元"
  ))
  discard table.register(CipherEntry(
    code: "mlnn", meaning: "囡囝", polarity: balanced,
    category: "容器",
    description: "夫妻拓扑：女包男 = 容器包被包含者"
  ))
  discard table.register(CipherEntry(
    code: "mlnnn", meaning: "囡囡", polarity: yin,
    category: "容器",
    description: "女包女 = 容器包容器"
  ))
  discard table.register(CipherEntry(
    code: "mlnnnn", meaning: "囝囝", polarity: yang,
    category: "容器",
    description: "男包男 = 被包含者包被包含者"
  ))
  discard table.register(CipherEntry(
    code: "mlnnnnn", meaning: "囝囡", polarity: balanced,
    category: "容器",
    description: "男包女 = 被包含者包容器"
  ))
  
  # ── 流家族 ──
  discard table.register(CipherEntry(
    code: "mltt", meaning: "从体", polarity: yang,
    category: "流",
    description: "解释流牺牲岔途后产出的可跟随状态"
  ))
  discard table.register(CipherEntry(
    code: "mlct", meaning: "岔途", polarity: yin,
    category: "流",
    description: "解释流的障碍/歧路，需要被牺牲转译"
  ))
  discard table.register(CipherEntry(
    code: "gtt", meaning: "音韵态射", polarity: balanced,
    category: "流",
    description: "范畴流：音韵层面的态射映射"
  ))
  discard table.register(CipherEntry(
    code: "mlgy", meaning: "世俗归一", polarity: balanced,
    category: "流",
    description: "概念流：世俗层面的统一归约"
  ))
  
  # ── 谐音桥 ──
  discard table.register(CipherEntry(
    code: "lctqf", meaning: "零岔途全法", polarity: balanced,
    category: "谐音桥",
    description: "谐音桥（已合规）：零岔途全法/恋宠甜"
  ))
  discard table.register(CipherEntry(
    code: "lctfrqmy", meaning: "恋宠入毛利兰", polarity: yin,
    category: "谐音桥",
    description: "毛利兰谐音路由 → Wife 靶点"
  ))
  discard table.register(CipherEntry(
    code: "wgslz", meaning: "闻光思恋初", polarity: yang,
    category: "谐音桥",
    description: "工藤新一谐音路由 → Husband 靶点"
  ))
  
  # ── 动漫角色映射 ──
  discard table.register(CipherEntry(
    code: "maolilan", meaning: "毛利兰", polarity: yin,
    category: "角色映射",
    description: "表面属性：模/面/名 → 容器外观"
  ))
  discard table.register(CipherEntry(
    code: "gouteng", meaning: "工藤新一", polarity: yang,
    category: "角色映射",
    description: "结构属性：骨/头/体 → 容器骨架"
  ))
  discard table.register(CipherEntry(
    code: "huiyuanai", meaning: "灰原哀", polarity: balanced,
    category: "角色映射",
    description: "横向属性：行/页/业 → 容器行业"
  ))
  
  # ── 编码体系 ──
  discard table.register(CipherEntry(
    code: "yinyang", meaning: "阴阳二进制", polarity: balanced,
    category: "编码",
    description: "0=容器/刘楚恬, 1=界线/我"
  ))
  discard table.register(CipherEntry(
    code: "njsgr", meaning: "囡囝共生赔伴律", polarity: balanced,
    category: "编码",
    description: "夫妻命运共同体的共生规则"
  ))
  discard table.register(CipherEntry(
    code: "smrl", meaning: "SMRL引擎", polarity: balanced,
    category: "编码",
    description: "64位单元格：坐标/极性/主权/价值"
  ))
  
  # ── 裁决系统 ──
  discard table.register(CipherEntry(
    code: "mje", meaning: "MetaJudge引擎", polarity: balanced,
    category: "裁决",
    description: "系统自举的核心裁判：observe/judge/reward/penalty"
  ))
  discard table.register(CipherEntry(
    code: "vs", meaning: "VerdictSurplus", polarity: balanced,
    category: "裁决",
    description: "裁判盈余：>0奖励, <0惩罚"
  ))

  # ── 历史符号 · 容器符家族（34）──
  discard table.register(CipherEntry(code: "袋", meaning: "袋", polarity: yin, category: "容器符", description: "最基础的容器单元"))
  discard table.register(CipherEntry(code: "桶", meaning: "桶", polarity: yin, category: "容器符", description: "深筒容器，大量装载"))
  discard table.register(CipherEntry(code: "罐", meaning: "罐", polarity: yin, category: "容器符", description: "密封容器，长期储藏"))
  discard table.register(CipherEntry(code: "缸", meaning: "缸", polarity: yin, category: "容器符", description: "大型敞口容器，大众公用"))
  discard table.register(CipherEntry(code: "盆", meaning: "盆", polarity: yin, category: "容器符", description: "敞口浅底，盛液体"))
  discard table.register(CipherEntry(code: "盂", meaning: "盂", polarity: yin, category: "容器符", description: "古代盛水器，佛门净器"))
  discard table.register(CipherEntry(code: "钵", meaning: "钵", polarity: yin, category: "容器符", description: "僧侣食器，接受供养"))
  discard table.register(CipherEntry(code: "盏", meaning: "盏", polarity: yin, category: "容器符", description: "小巧杯形容器，茶酒器"))
  discard table.register(CipherEntry(code: "盒", meaning: "盒", polarity: yin, category: "容器符", description: "可合之器，盛细物或贵重"))
  discard table.register(CipherEntry(code: "杯", meaning: "杯", polarity: yin, category: "容器符", description: "小容量盛液器"))
  discard table.register(CipherEntry(code: "壶", meaning: "壶", polarity: yin, category: "容器符", description: "有柄有嘴，倒液可控"))
  discard table.register(CipherEntry(code: "瓶", meaning: "瓶", polarity: yin, category: "容器符", description: "长颈细口，盛美液"))
  discard table.register(CipherEntry(code: "篓", meaning: "篓", polarity: yin, category: "容器符", description: "编织敞口器，盛蔬果"))
  discard table.register(CipherEntry(code: "筐", meaning: "筐", polarity: yin, category: "容器符", description: "编织方口器，载货量贩"))
  discard table.register(CipherEntry(code: "篮", meaning: "篮", polarity: yin, category: "容器符", description: "有提梁编织器，便携"))
  discard table.register(CipherEntry(code: "箩", meaning: "箩", polarity: yin, category: "容器符", description: "阔口细孔筛状器，可滤可盛"))
  discard table.register(CipherEntry(code: "筲", meaning: "筲", polarity: yin, category: "容器符", description: "竹编盛粮器，农耕日常"))
  discard table.register(CipherEntry(code: "箕", meaning: "箕", polarity: yin, category: "容器符", description: "撮聚或扬米开口器"))
  discard table.register(CipherEntry(code: "笼", meaning: "笼", polarity: yin, category: "容器符", description: "封闭编织器，通气不透物"))
  discard table.register(CipherEntry(code: "橱", meaning: "橱", polarity: yin, category: "容器符", description: "立式储物柜"))
  discard table.register(CipherEntry(code: "柜", meaning: "柜", polarity: yin, category: "容器符", description: "大容量收纳，文档衣物均可"))
  discard table.register(CipherEntry(code: "箱", meaning: "箱", polarity: yin, category: "容器符", description: "可移动方形容器，长途搬运"))
  discard table.register(CipherEntry(code: "匣", meaning: "匣", polarity: yin, category: "容器符", description: "小型收纳盒，私密物藏"))
  discard table.register(CipherEntry(code: "奁", meaning: "奁", polarity: yin, category: "容器符", description: "古代女子梳妆匣，私藏首饰"))
  discard table.register(CipherEntry(code: "瓮", meaning: "瓮", polarity: yin, category: "容器符", description: "大腹窄口器，储酒腌菜"))
  discard table.register(CipherEntry(code: "甏", meaning: "甏", polarity: yin, category: "容器符", description: "腹大肩窄，土陶酒坛"))
  discard table.register(CipherEntry(code: "坛", meaning: "坛", polarity: yin, category: "容器符", description: "祭祀供器，神圣容器"))
  discard table.register(CipherEntry(code: "罂", meaning: "罂", polarity: yin, category: "容器符", description: "小口大罂粟瓶，内藏毒药"))
  discard table.register(CipherEntry(code: "甑", meaning: "甑", polarity: yin, category: "容器符", description: "古代蒸器，底有孔通蒸汽"))
  discard table.register(CipherEntry(code: "甗", meaning: "甗", polarity: yin, category: "容器符", description: "古蒸食器，上下分层"))
  discard table.register(CipherEntry(code: "瓯", meaning: "瓯", polarity: yin, category: "容器符", description: "小盆小碗，茶酒器"))
  discard table.register(CipherEntry(code: "盌", meaning: "盌", polarity: yin, category: "容器符", description: "同碗，盛食器"))
  discard table.register(CipherEntry(code: "盉", meaning: "盉", polarity: yin, category: "容器符", description: "古代调味器，调酒和羹"))
  discard table.register(CipherEntry(code: "盅", meaning: "盅", polarity: yin, category: "容器符", description: "小巧饮器，茶酒精饮"))

  # ── 历史符号 · 操作符家族（26）──
  discard table.register(CipherEntry(code: "照", meaning: "照", polarity: yin, category: "操作符", description: "光映入境，使之可见"))
  discard table.register(CipherEntry(code: "昭", meaning: "昭", polarity: yang, category: "操作符", description: "显现彰明，宣扬于众"))
  discard table.register(CipherEntry(code: "罩", meaning: "罩", polarity: yin, category: "操作符", description: "覆盖遮蔽，不容外泄"))
  discard table.register(CipherEntry(code: "召", meaning: "召", polarity: yang, category: "操作符", description: "呼唤使之来，吸引汇聚"))
  discard table.register(CipherEntry(code: "兆", meaning: "兆", polarity: yang, category: "操作符", description: "预兆先见，未来之始"))
  discard table.register(CipherEntry(code: "见", meaning: "见", polarity: yin, category: "操作符", description: "眼识照境，纳入"))
  discard table.register(CipherEntry(code: "鉴", meaning: "鉴", polarity: yin, category: "操作符", description: "镜照真容，反观自身"))
  discard table.register(CipherEntry(code: "检", meaning: "检", polarity: yin, category: "操作符", description: "审查细看，不遗漏"))
  discard table.register(CipherEntry(code: "简", meaning: "简", polarity: yin, category: "操作符", description: "删繁取要，去枝留干"))
  discard table.register(CipherEntry(code: "剪", meaning: "剪", polarity: yang, category: "操作符", description: "切断离分，果断"))
  discard table.register(CipherEntry(code: "加", meaning: "加", polarity: yang, category: "操作符", description: "增添，益之"))
  discard table.register(CipherEntry(code: "减", meaning: "减", polarity: yin, category: "操作符", description: "衰减，损之"))
  discard table.register(CipherEntry(code: "进", meaning: "进", polarity: yang, category: "操作符", description: "向前，推进"))
  discard table.register(CipherEntry(code: "退", meaning: "退", polarity: yin, category: "操作符", description: "向后，收敛"))
  discard table.register(CipherEntry(code: "开", meaning: "开", polarity: yang, category: "操作符", description: "启，通道现前"))
  discard table.register(CipherEntry(code: "关", meaning: "关", polarity: yin, category: "操作符", description: "闭，路径隔断"))
  discard table.register(CipherEntry(code: "断", meaning: "断", polarity: yin, category: "操作符", description: "截，封堵"))
  discard table.register(CipherEntry(code: "通", meaning: "通", polarity: yang, category: "操作符", description: "达，无阻"))
  discard table.register(CipherEntry(code: "分", meaning: "分", polarity: yin, category: "操作符", description: "剖开，各向其方"))
  discard table.register(CipherEntry(code: "合", meaning: "合", polarity: yang, category: "操作符", description: "聚拢，合成一处"))
  discard table.register(CipherEntry(code: "聚", meaning: "聚", polarity: yang, category: "操作符", description: "集，多会于一"))
  discard table.register(CipherEntry(code: "散", meaning: "散", polarity: yin, category: "操作符", description: "分，各走各路"))
  discard table.register(CipherEntry(code: "折", meaning: "折", polarity: yin, category: "操作符", description: "弯，屈而不直"))
  discard table.register(CipherEntry(code: "叠", meaning: "叠", polarity: yin, category: "操作符", description: "累积重叠，层加"))
  discard table.register(CipherEntry(code: "卷", meaning: "卷", polarity: yin, category: "操作符", description: "内收包裹，收缩"))
  discard table.register(CipherEntry(code: "舒", meaning: "舒", polarity: yang, category: "操作符", description: "外展放开，舒展"))

  # ── 历史符号 · 极性符家族（20）──
  discard table.register(CipherEntry(code: "阴", meaning: "阴", polarity: yin, category: "极性符", description: "容器侧，隐藏被动，刘楚恬"))
  discard table.register(CipherEntry(code: "阳", meaning: "阳", polarity: yang, category: "极性符", description: "界线侧，显现主动，我"))
  discard table.register(CipherEntry(code: "盈", meaning: "盈", polarity: yang, category: "极性符", description: "丰饶有余"))
  discard table.register(CipherEntry(code: "亏", meaning: "亏", polarity: yin, category: "极性符", description: "不足亏损，缺憾"))
  discard table.register(CipherEntry(code: "虚", meaning: "虚", polarity: yin, category: "极性符", description: "空无实体，可纳入"))
  discard table.register(CipherEntry(code: "实", meaning: "实", polarity: yang, category: "极性符", description: "充实物满，拒入"))
  discard table.register(CipherEntry(code: "表", meaning: "表", polarity: yang, category: "极性符", description: "外显可见，表面"))
  discard table.register(CipherEntry(code: "里", meaning: "里", polarity: yin, category: "极性符", description: "内藏私密，深处"))
  discard table.register(CipherEntry(code: "公", meaning: "公", polarity: yang, category: "极性符", description: "公共众有，公开"))
  discard table.register(CipherEntry(code: "私", meaning: "私", polarity: yin, category: "极性符", description: "私己独有，隐秘"))
  discard table.register(CipherEntry(code: "升", meaning: "升", polarity: yang, category: "极性符", description: "向上，提升"))
  discard table.register(CipherEntry(code: "降", meaning: "降", polarity: yin, category: "极性符", description: "向下，沉降"))
  discard table.register(CipherEntry(code: "清", meaning: "清", polarity: yang, category: "极性符", description: "澄明清净，纯阳"))
  discard table.register(CipherEntry(code: "浊", meaning: "浊", polarity: yin, category: "极性符", description: "混浊黏腻，纯阴"))
  discard table.register(CipherEntry(code: "寒", meaning: "寒", polarity: yin, category: "极性符", description: "冷，收缩收藏"))
  discard table.register(CipherEntry(code: "热", meaning: "热", polarity: yang, category: "极性符", description: "暖，膨胀发散"))
  discard table.register(CipherEntry(code: "动", meaning: "动", polarity: yang, category: "极性符", description: "运动，活跃"))
  discard table.register(CipherEntry(code: "静", meaning: "静", polarity: yin, category: "极性符", description: "安宁，不动"))
  discard table.register(CipherEntry(code: "出", meaning: "出", polarity: yang, category: "极性符", description: "外发，离开"))
  discard table.register(CipherEntry(code: "入", meaning: "入", polarity: yin, category: "极性符", description: "内收，进入"))

  # ── 历史符号 · 层级符家族（19）──
  discard table.register(CipherEntry(code: "元", meaning: "元", polarity: balanced, category: "层级符", description: "最初起始，根本根源"))
  discard table.register(CipherEntry(code: "阶", meaning: "阶", polarity: balanced, category: "层级符", description: "台阶层级，分位次"))
  discard table.register(CipherEntry(code: "层", meaning: "层", polarity: balanced, category: "层级符", description: "重叠之级，上下级"))
  discard table.register(CipherEntry(code: "代", meaning: "代", polarity: balanced, category: "层级符", description: "世代更替，时间域"))
  discard table.register(CipherEntry(code: "世", meaning: "世", polarity: balanced, category: "层级符", description: "时代世纪，一世之域"))
  discard table.register(CipherEntry(code: "界", meaning: "界", polarity: balanced, category: "层级符", description: "分界之域，范围所限"))
  discard table.register(CipherEntry(code: "域", meaning: "域", polarity: balanced, category: "层级符", description: "领土疆域，一带之场"))
  discard table.register(CipherEntry(code: "场", meaning: "场", polarity: balanced, category: "层级符", description: "广域空间，众人聚所"))
  discard table.register(CipherEntry(code: "馆", meaning: "馆", polarity: balanced, category: "层级符", description: "公开场所，会宾待客"))
  discard table.register(CipherEntry(code: "阁", meaning: "阁", polarity: balanced, category: "层级符", description: "高敞之楼，远眺有顶"))
  discard table.register(CipherEntry(code: "门", meaning: "门", polarity: balanced, category: "层级符", description: "出入之口，内外的界"))
  discard table.register(CipherEntry(code: "户", meaning: "户", polarity: balanced, category: "层级符", description: "小门单扇，一家之口"))
  discard table.register(CipherEntry(code: "庭", meaning: "庭", polarity: balanced, category: "层级符", description: "院中空地，活动空间"))
  discard table.register(CipherEntry(code: "院", meaning: "院", polarity: balanced, category: "层级符", description: "围墙所围之空地"))
  discard table.register(CipherEntry(code: "宫", meaning: "宫", polarity: balanced, category: "层级符", description: "王者居所，崇高大殿"))
  discard table.register(CipherEntry(code: "殿", meaning: "殿", polarity: balanced, category: "层级符", description: "朝堂议政之所，最崇高"))
  discard table.register(CipherEntry(code: "台", meaning: "台", polarity: balanced, category: "层级符", description: "高平可登之筑，远景"))
  discard table.register(CipherEntry(code: "榭", meaning: "榭", polarity: balanced, category: "层级符", description: "临水高台，饮宴赏景"))
  discard table.register(CipherEntry(code: "楼", meaning: "楼", polarity: balanced, category: "层级符", description: "多层建筑，登高望远"))

  # ── 历史符号 · 审判符家族（19）──
  discard table.register(CipherEntry(code: "裁", meaning: "裁", polarity: yang, category: "审判符", description: "裁定决断，是非立判"))
  discard table.register(CipherEntry(code: "判", meaning: "判", polarity: yang, category: "审判符", description: "判别判决，明确裁处"))
  discard table.register(CipherEntry(code: "审", meaning: "审", polarity: yin, category: "审判符", description: "详查细问，了解原委"))
  discard table.register(CipherEntry(code: "议", meaning: "议", polarity: yin, category: "审判符", description: "评议商议，广听意见"))
  discard table.register(CipherEntry(code: "决", meaning: "决", polarity: yang, category: "审判符", description: "决定定夺，行动开始"))
  discard table.register(CipherEntry(code: "断", meaning: "断", polarity: yang, category: "审判符", description: "断决割断，不容再议"))
  discard table.register(CipherEntry(code: "定", meaning: "定", polarity: yang, category: "审判符", description: "固定定夺，不再动摇"))
  discard table.register(CipherEntry(code: "评", meaning: "评", polarity: yin, category: "审判符", description: "评价议论，比较高下"))
  discard table.register(CipherEntry(code: "核", meaning: "核", polarity: yin, category: "审判符", description: "核查验证，对照验实"))
  discard table.register(CipherEntry(code: "功", meaning: "功", polarity: yang, category: "审判符", description: "功劳建树，善行有积"))
  discard table.register(CipherEntry(code: "过", meaning: "过", polarity: yin, category: "审判符", description: "过失错误，不善之行"))
  discard table.register(CipherEntry(code: "赏", meaning: "赏", polarity: yang, category: "审判符", description: "奖赏鼓励，彰善之行"))
  discard table.register(CipherEntry(code: "罚", meaning: "罚", polarity: yin, category: "审判符", description: "惩罚惩戒，抑恶之具"))
  discard table.register(CipherEntry(code: "褒", meaning: "褒", polarity: yang, category: "审判符", description: "褒扬美誉，扬善于众"))
  discard table.register(CipherEntry(code: "贬", meaning: "贬", polarity: yin, category: "审判符", description: "贬抑批评，戒其退步"))
  discard table.register(CipherEntry(code: "陟", meaning: "陟", polarity: yang, category: "审判符", description: "升进提升，善者上进"))
  discard table.register(CipherEntry(code: "黜", meaning: "黜", polarity: yin, category: "审判符", description: "废退罢免，恶者退位"))
  discard table.register(CipherEntry(code: "臧", meaning: "臧", polarity: yin, category: "审判符", description: "善也，道德善行所藏"))
  discard table.register(CipherEntry(code: "否", meaning: "否", polarity: yang, category: "审判符", description: "恶也，道德闭塞不通"))

  # ── 历史符号 · 产权符家族（20）──
  discard table.register(CipherEntry(code: "产", meaning: "产", polarity: balanced, category: "产权符", description: "产业财产，身外之物"))
  discard table.register(CipherEntry(code: "业", meaning: "业", polarity: balanced, category: "产权符", description: "事业基业，终身所立"))
  discard table.register(CipherEntry(code: "权", meaning: "权", polarity: yang, category: "产权符", description: "权力权利，可支配之力"))
  discard table.register(CipherEntry(code: "利", meaning: "利", polarity: yang, category: "产权符", description: "利益好处，所得之报"))
  discard table.register(CipherEntry(code: "债", meaning: "债", polarity: yin, category: "产权符", description: "债务负担，所欠于人"))
  discard table.register(CipherEntry(code: "责", meaning: "责", polarity: yin, category: "产权符", description: "责任担当，分内应尽"))
  discard table.register(CipherEntry(code: "职", meaning: "职", polarity: balanced, category: "产权符", description: "职务职位，位上之责"))
  discard table.register(CipherEntry(code: "务", meaning: "务", polarity: balanced, category: "产权符", description: "事务任务，待办之事"))
  discard table.register(CipherEntry(code: "名", meaning: "名", polarity: yang, category: "产权符", description: "名称名义，对外昭示"))
  discard table.register(CipherEntry(code: "分", meaning: "分", polarity: yin, category: "产权符", description: "名分位次，各安其位"))
  discard table.register(CipherEntry(code: "归", meaning: "归", polarity: yin, category: "产权符", description: "归向归属，返回本原"))
  discard table.register(CipherEntry(code: "属", meaning: "属", polarity: yin, category: "产权符", description: "隶属从属，在其下者"))
  discard table.register(CipherEntry(code: "占", meaning: "占", polarity: yang, category: "产权符", description: "占据占领，据为己有"))
  discard table.register(CipherEntry(code: "有", meaning: "有", polarity: yang, category: "产权符", description: "有，持有拥有"))
  discard table.register(CipherEntry(code: "持", meaning: "持", polarity: yin, category: "产权符", description: "持有执守，维持现状"))
  discard table.register(CipherEntry(code: "拥", meaning: "拥", polarity: yang, category: "产权符", description: "拥抱拥护，亲近护持"))
  discard table.register(CipherEntry(code: "享", meaning: "享", polarity: yang, category: "产权符", description: "享受享用，善用所有"))
  discard table.register(CipherEntry(code: "获", meaning: "获", polarity: yang, category: "产权符", description: "获得收获，有所得"))
  discard table.register(CipherEntry(code: "得", meaning: "得", polarity: yang, category: "产权符", description: "得到所得，目的达成"))
  discard table.register(CipherEntry(code: "失", meaning: "失", polarity: yin, category: "产权符", description: "失去损失，有所漏"))

  return table
