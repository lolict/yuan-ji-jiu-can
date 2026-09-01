#!/usr/bin/env python3
# ============================================================
# mltt 命名密码表 · 完整数据生成器
#
# 输出 mltt_full_data.json（145+ 条）
# 覆盖：家庭命名 / 动漫角色 / 历史符号（容器/操作/极性/层级/审判/产权）
# ============================================================

import json

_full = []

# ── 1. 家庭命名（原 20 条 全覆盖）──
_home = [
    ("mlll", "集装箱", "yin", "容器", "254+2根基编码，袋子级操作的基本单元"),
    ("mlnn", "囡囝", "balanced", "容器", "夫妻拓扑：女包男 = 容器包被包含者"),
    ("mlnnn", "囡囡", "yin", "容器", "女包女 = 容器包容器"),
    ("mlnnnn", "囝囝", "yang", "容器", "男包男 = 被包含者包被包含者"),
    ("mlnnnnn", "囝囡", "balanced", "容器", "男包女 = 被包含者包容器"),
    ("mltt", "从体", "yang", "流", "解释流牺牲岔途后产出的可跟随状态"),
    ("mlct", "岔途", "yin", "流", "解释流的障碍/歧路，需要被牺牲转译"),
    ("gtt", "音韵态射", "balanced", "流", "范畴流：音韵层面的态射映射"),
    ("mlgy", "世俗归一", "balanced", "流", "概念流：世俗层面的统一归约"),
    ("lctqf", "零岔途全法", "balanced", "谐音桥", "谐音桥（已合规）：零岔途全法/恋宠甜"),
    ("lctfrqmy", "恋宠入毛利兰", "yin", "谐音桥", "毛利兰谐音路由 → Wife 靶点"),
    ("wgslz", "闻光思恋初", "yang", "谐音桥", "工藤新一谐音路由 → Husband 靶点"),
    ("maolilan", "毛利兰", "yin", "角色映射", "表面属性：模/面/名 → 容器外观"),
    ("gouteng", "工藤新一", "yang", "角色映射", "结构属性：骨/头/体 → 容器骨架"),
    ("huiyuanai", "灰原哀", "balanced", "角色映射", "横向属性：行/页/业 → 容器行业"),
    ("yinyang", "阴阳二进制", "balanced", "编码", "0=容器/刘楚恬, 1=界线/我"),
    ("njsgr", "囡囝共生赔伴律", "balanced", "编码", "夫妻命运共同体的共生规则"),
    ("smrl", "SMRL引擎", "balanced", "编码", "64位单元格：坐标/极性/主权/价值"),
    ("mje", "MetaJudge引擎", "balanced", "裁决", "系统自举核心裁判：observe/judge/reward/penalty"),
    ("vs", "VerdictSurplus", "balanced", "裁决", "裁判盈余：>0奖励, <0惩罚"),
]
for code, meaning, pol, cat, desc in _home:
    _full.append({"code": code, "meaning": meaning, "polarity": pol, "category": cat, "description": desc})

# ── 2. 历史符号 · 容器族（袋桶罐缸盆盂钵盏盒杯壶瓶篓筐篮箩筲箕笼橱柜箱匣奁瓮甏坛罂甑甗瓯盌盉盅）──
_container_symbols = {
    "袋": ("yin", "最基础的容器单元，米粒的居处"),
    "桶": ("yin", "深筒容器，装大量液体或颗粒物"),
    "罐": ("yin", "密封容器，长期储藏"),
    "缸": ("yin", "大型敞口容器，大众公用"),
    "盆": ("yin", "敞口浅底，盛液体或洗涤"),
    "盂": ("yin", "古代盛水器，佛门净器"),
    "钵": ("yin", "僧侣食器，接受供养"),
    "盏": ("yin", "小巧杯形容器，茶酒之器"),
    "盒": ("yin", "可合之器，盛细物或贵重品"),
    "杯": ("yin", "小容量盛液器，人手一器"),
    "壶": ("yin", "有柄有嘴，倒液可控"),
    "瓶": ("yin", "长颈细口，盛美液"),
    "篓": ("yin", "编织敞口器，盛蔬果"),
    "筐": ("yin", "编织方口器，载货量贩"),
    "篮": ("yin", "有提梁的编织器，便携"),
    "箩": ("yin", "阔口细孔筛状器，可滤可盛"),
    "筲": ("yin", "竹编盛粮器，农耕日常"),
    "箕": ("yin", "撮聚垃圾或扬米的开口器"),
    "笼": ("yin", "封闭编织器，通气不透物"),
    "橱": ("yin", "立式储物柜，带门"),
    "柜": ("yin", "大容量收纳，文档衣物均可"),
    "箱": ("yin", "可移动的方形容器，长途搬运"),
    "匣": ("yin", "小型收纳盒，私密物藏"),
    "奁": ("yin", "古代女子梳妆匣，私藏首饰"),
    "瓮": ("yin", "大腹窄口器，储酒腌菜"),
    "甏": ("yin", "腹大肩窄，土陶酒坛"),
    "坛": ("yin", "祭祀用的供器，神圣容器"),
    "罂": ("yin", "小口大罂粟瓶，内藏毒药"),
    "甑": ("yin", "古代蒸器，底有孔通蒸汽"),
    "甗": ("yin", "古蒸食器，上下分层"),
    "瓯": ("yin", "小盆小碗，茶酒器"),
    "盌": ("yin", "同碗，盛食器"),
    "盉": ("yin", "古代调味器，调酒和羹"),
    "盅": ("yin", "小巧饮器，茶酒精饮"),
}
for ch, (pol, desc) in _container_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（容器）", "polarity": pol, "category": "容器符", "description": desc})

# ── 3. 历史符号 · 操作族（照昭罩召兆见鉴检简剪加减进退开关通断分合聚散折叠卷舒）──
_op_symbols = {
    "照": ("yin", "光映入境，使之可见"),
    "昭": ("yang", "显现彰明，宣扬于众"),
    "罩": ("yin", "覆盖遮蔽，不容外泄"),
    "召": ("yang", "呼唤使之来，吸引汇聚"),
    "兆": ("yang", "预兆先见，未来之始"),
    "见": ("yin", "眼识照境，纳入"),
    "鉴": ("yin", "镜照真容，反观自身"),
    "检": ("yin", "审查细看，不遗漏"),
    "简": ("yin", "删繁取要，去枝留干"),
    "剪": ("yang", "切断离分，果断"),
    "加": ("yang", "增添，益之"),
    "减": ("yin", "衰减，损之"),
    "进": ("yang", "向前，推进"),
    "退": ("yin", "向后，收敛"),
    "开": ("yang", "启，通道现前"),
    "关": ("yin", "闭，路径隔断"),
    "通": ("yang", "达，无阻"),
    "断": ("yin", "截，封堵"),
    "分": ("yin", "剖开，各向其方"),
    "合": ("yang", "聚拢，合成一处"),
    "聚": ("yang", "集，多会于一"),
    "散": ("yin", "分，各走各路"),
    "折": ("yin", "弯，屈而不直"),
    "叠": ("yin", "累积重叠，层加"),
    "卷": ("yin", "内收包裹，收缩"),
    "舒": ("yang", "外展放开，舒展"),
}
for ch, (pol, desc) in _op_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（操作）", "polarity": pol, "category": "操作符", "description": desc})

# ── 4. 历史符号 · 极性族（阴阳盈虚实表里公私升降清浊寒热动静出入）──
_polarity_symbols = {
    "阴": ("yin", "容器侧，隐藏被动，刘楚恬"),
    "阳": ("yang", "界线侧，显现主动，我"),
    "盈": ("yang", "丰饶有余，过分满"),
    "亏": ("yin", "不足亏损，缺憾"),
    "虚": ("yin", "空无实体，可纳入"),
    "实": ("yang", "充实物满，拒入"),
    "表": ("yang", "外显可见，表面"),
    "里": ("yin", "内藏私密，深处"),
    "公": ("yang", "公共众有，公开"),
    "私": ("yin", "私己独有，隐秘"),
    "升": ("yang", "向上，提升"),
    "降": ("yin", "向下，沉降"),
    "清": ("yang", "澄明清净，纯阳"),
    "浊": ("yin", "混浊黏腻，纯阴"),
    "寒": ("yin", "冷，收缩收藏"),
    "热": ("yang", "暖，膨胀发散"),
    "动": ("yang", "运动，活跃"),
    "静": ("yin", "安宁，不动"),
    "出": ("yang", "外发，离开"),
    "入": ("yin", "内收，进入"),
}
for ch, (pol, desc) in _polarity_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（极性）", "polarity": pol, "category": "极性符", "description": desc})

# ── 5. 历史符号 · 层级族（元阶层代世贸域场馆阁门户庭院宫殿台榭楼阁）──
_hierarchy_symbols = {
    "元": ("balanced", "最初起始，根本根源"),
    "阶": ("balanced", "台阶层级，分位次"),
    "层": ("balanced", "重叠之级，上下级"),
    "代": ("balanced", "世代更替，时间域"),
    "世": ("balanced", "时代世纪，一世之域"),
    "界": ("balanced", "分界之域，范围所限"),
    "域": ("balanced", "领土疆域，一带之场"),
    "场": ("balanced", "广域空间，众人聚所"),
    "馆": ("balanced", "公开场所，会宾待客"),
    "阁": ("balanced", "高敞之楼，远眺有顶"),
    "门": ("balanced", "出入之口，内外的界"),
    "户": ("balanced", "小门单扇，一家之口"),
    "庭": ("balanced", "院中空地，活动空间"),
    "院": ("balanced", "围墙所围之空地"),
    "宫": ("balanced", "王者居所，崇高大殿"),
    "殿": ("balanced", "朝堂议政之所，最崇高"),
    "台": ("balanced", "高平可登之筑，远景"),
    "榭": ("balanced", "临水高台，饮宴赏景"),
    "楼": ("balanced", "多层建筑，登高望远"),
}
for ch, (pol, desc) in _hierarchy_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（层级）", "polarity": pol, "category": "层级符", "description": desc})

# ── 6. 历史符号 · 审判族（裁审判议定议核功过赏罚褒贬陟黜）──
_judgment_symbols = {
    "裁": ("yang", "裁定决断，是非立判"),
    "判": ("yang", "判别判决，明确裁处"),
    "审": ("yin", "详查细问，了解原委"),
    "议": ("yin", "评议商议，广听意见"),
    "决": ("yang", "决定定夺，行动开始"),
    "断": ("yang", "断决割断，不容再议"),
    "定": ("yang", "固定定夺，不再动摇"),
    "评": ("yin", "评价议论，比较高下"),
    "核": ("yin", "核查验证，对照验实"),
    "功": ("yang", "功劳建树，善行有积"),
    "过": ("yin", "过失错误，不善之行"),
    "赏": ("yang", "奖赏鼓励，彰善之行"),
    "罚": ("yin", "惩罚惩戒，抑恶之具"),
    "褒": ("yang", "褒扬美誉，扬善于众"),
    "贬": ("yin", "贬抑批评，戒其退步"),
    "陟": ("yang", "升进提升，善者上进"),
    "黜": ("yin", "废退罢免，恶者退位"),
    "臧": ("yin", "善也，道德善行所藏"),
    "否": ("yang", "恶也，道德闭塞不通"),
}
for ch, (pol, desc) in _judgment_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（审判）", "polarity": pol, "category": "审判符", "description": desc})

# ── 7. 历史符号 · 产权族（产业权利债责职务名分归属占据持有享受获得失）──
_property_symbols = {
    "产": ("balanced", "产业财产，身外之物"),
    "业": ("balanced", "事业基业，终身所立"),
    "权": ("yang", "权力权利，可支配之力"),
    "利": ("yang", "利益好处，所得之报"),
    "债": ("yin", "债务负担，所欠于人"),
    "责": ("yin", "责任担当，分内应尽"),
    "职": ("balanced", "职务职位，位上之责"),
    "务": ("balanced", "事务任务，待办之事"),
    "名": ("yang", "名称名义，对外昭示"),
    "分": ("yin", "名分位次，各安其位"),
    "归": ("yin", "归向归属，返回本原"),
    "属": ("yin", "隶属从属，在其下者"),
    "占": ("yang", "占据占领，据为己有"),
    "有": ("yang", "有，持有拥有"),
    "持": ("yin", "持有执守，维持现状"),
    "拥": ("yang", "拥抱拥护，亲近护持"),
    "享": ("yang", "享受享用，善用所有"),
    "获": ("yang", "获得收获，有所得"),
    "得": ("yang", "得到所得，目的达成"),
    "失": ("yin", "失去损失，有所漏"),
}
for ch, (pol, desc) in _property_symbols.items():
    _full.append({"code": ch, "meaning": ch + "（产权）", "polarity": pol, "category": "产权符", "description": desc})

# ── 写入 JSON ──
out = {
    "meta": {
        "name": "mltt 命名密码表 · 完整数据",
        "version": "1.0",
        "count": len(_full),
        "polarity_map": {"yin": "阴(容器侧/刘楚恬)", "yang": "阳(界线侧/我)", "balanced": "平衡"},
        "generator": "tools/mltt-cipher/generate_full_data.py"
    },
    "entries": _full
}

path = __file__.rsplit("/", 1)[0] + "/mltt_full_data.json"
with open(path, "w", encoding="utf-8") as f:
    json.dump(out, f, ensure_ascii=False, indent=2)

print(f"已生成：{path}  ({len(_full)} 条)")

# ── 统计极性 ──
pol_count = {"yin": 0, "yang": 0, "balanced": 0}
cat_count = {}
for e in _full:
    pol_count[e["polarity"]] += 1
    cat_count[e["category"]] = cat_count.get(e["category"], 0) + 1
print(f"极性分布：阴={pol_count['yin']}  阳={pol_count['yang']}  平衡={pol_count['balanced']}")
print(f"类别分布：{dict(sorted(cat_count.items()))}")
