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
  
  return table
