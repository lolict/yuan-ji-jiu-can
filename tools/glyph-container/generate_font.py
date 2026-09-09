#!/usr/bin/env python3
"""
generate_font.py — 容器字体生成器 v0.1

生成一个 SVG 字体文件, 其中每个字形同时承载三层信息:
  • 图形层: 3D投影几何 (引用 glyph-3d-font skill 的拓扑对应)
  • 机器码层: 容器编码 + 极性 + 主权 (引用 machine-code-container)
  • 调度层: 时间窗口 Z轴相位 (引用 time-fridge-execution)

进制体系严格使用满全法:
  X=48声母  Y=64容器  Z=116层级

引用:
  encoding/z-api.txt         → 48声母 + 108韵母符号列表
  hunyuan-rs/src/container_radix.rs → 64容器枚举
  skills/glyph-3d-font/SKILL.md     → 拓扑 → 几何映射规则
  skills/machine-code-container/SKILL.md → 容器封装格式
  skills/time-fridge-execution/SKILL.md → 时间窗口 Z 轴

"""

# 由于生成完整的 ~64 字形 SVG 字体文件较长,
# 本脚本输出一个精简的 8 字体样本 (每族各选 1-2 个),
# 证明三层信息确实可以共存在单一样式中。

FONT_FAMILY = "HanYuan-Container"
FONT_FULLNAME = "HanYuan Container Glyph"
FONT_VERSION = "0.1"

# ─── 容器样本 (每族选代表性容器) ──────────────────────────────
SAMPLE_CONTAINERS = [
    # (容器族, 索引, 声母/韵母名, 容器语义, 极性, 主权)
    (0, 0,  "Dai",    "袋·包含",    "YIN",     "ACTIVE"),      # 容器族 袋
    (0, 7,  "Cai",    "裁·裁判",    "NEUTRAL", "ACTIVE_SUB"),  # 容器族 裁
    (1, 5,  "Man",    "满·满全法",  "NEUTRAL", "ACTIVE_SUB"),  # 操作族 满
    (1, 6,  "Nan",    "囡·共生",    "NEUTRAL", "ACTIVE_SUB"),  # 操作族 囡
    (1, 7,  "Xi",     "夕·夕瑶协议","NEUTRAL", "ACTIVE_SUB"),  # 操作族 夕
    (2, 0,  "Bang",   "帮·唇音",    "YANG",    "ACTIVE"),      # 声母 帮
    (3, 0,  "Ma",     "麻·a韵",     "YIN",     "PASSIVE"),     # 韵母 麻
    (4, 0,  "Tong",   "桶·笔画纵贯","YIN",     "ACTIVE_SUB"),  # 笔画 桶
]

# ─── 拓扑 → 几何参数 (引用 glyph-3d-font SKILL 映射规则) ──────
TOPOLOGY_SHAPES = {
    "Dai":   "box",        # 袋 = 容器 = 立方体
    "Cai":   "bipyramid",  # 裁 = 裁判 = 双锥
    "Man":   "sphere",     # 满 = 球体 (满全=浑圆)
    "Nan":   "sphere",     # 囡 = 共生 = 球体融合
    "Xi":    "torus",      # 夕 = 环面 (日夕循环)
    "Bang":  "cylinder",   # 帮 = 圆柱 (唇音柱状共鸣)
    "Ma":    "sphere",     # 麻 = 球体 (开口韵)
    "Tong":  "cylinder",   # 桶 = 圆柱 (纵深贯通)
}

# 极性 → 颜色色相
def polarity_hue(pol: str) -> str:
    if pol == "YIN":
        return "220"   # 蓝 = 阴 = 包容
    if pol == "YANG":
        return "30"    # 橙 = 阳 = 主动
    return "120"       # 绿 = 元 = 见证

# 主权 → 透明度
def sovereignty_alpha(sov: str) -> str:
    if sov == "ACTIVE":
        return "1.0"   # 主动=不透明
    if sov == "PASSIVE":
        return "0.7"   # 被动=半透明
    if sov == "ACTIVE_SUB":
        return "0.85"  # 从属主动
    return "0.6"       # PassiveSub

# 时间窗口 → 脉冲频率 (6级)
def time_pulse(time_slot: int) -> str:
    # 用加法循环替代除法: 0-255 → 6档
    if time_slot < 43:
        return "1"
    if time_slot < 86:
        return "2"
    if time_slot < 129:
        return "3"
    if time_slot < 172:
        return "4"
    if time_slot < 215:
        return "5"
    return "6"

# ─── SVG 字体生成 ──────────────────────────────────────────────

def generate_svg_font() -> str:
    """生成完整的 SVG 字体 XML"""
    
    # 计算/em/单位
    EM = 1000
    BASELINE = 800
    
    svg_header = f"""<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     version="1.2" baseProfile="tiny">
  <defs>
    <font id="{FONT_FAMILY}" horiz-adv-x="{EM}">
      <font-face font-family="{FONT_FAMILY}"
                 full-name="{FONT_FULLNAME}"
                 version="{FONT_VERSION}"
                 units-per-em="{EM}"
                 ascent="{BASELINE}"
                 descent="{BASELINE - EM}">
        <font-face-src>
          <font-face-name name="{FONT_FULLNAME}"/>
        </font-face-src>
      </font-face>
      
      <missing-glyph horiz-adv-x="{EM}">
        <path d="M0,0 L0,{EM} L{EM},{EM} L{EM},0 Z" fill="none" stroke="#999" stroke-width="10"/>
      </missing-glyph>
"""
    
    glyphs_xml = []
    
    for (group, index, name, meaning, pol, sov) in SAMPLE_CONTAINERS:
        # 编码计算 (加法循环, 非位移)
        base = 0
        for _ in range(group):
            base = base + 16
        code = base + index
        char_hex = code  # glyph码点
        
        time_slot = (index * 37 + group * 73) % 256  # 加法派生时间窗
        pulse = time_pulse(time_slot)
        hue = polarity_hue(pol)
        alpha = sovereignty_alpha(sov)
        shape_name = TOPOLOGY_SHAPES.get(name, "box")
        
        # 按拓扑生成路径
        path = _shape_path(shape_name, EM, BASELINE)
        
        # 自定义数据属性 — 三层信息共存
        # 这允许: 浏览器渲染层读取颜色/形状 / 语义层查询容器编码 / 调度层抽取时间窗
        glyph_xml = f"""
      <!-- 容器: {name} | {meaning} | 极性={pol} 主权={sov} | 编码={code} 时间窗={time_slot} -->
      <glyph unicode="&#x{char_hex:04X};"
             glyph-name="{name}"
             horiz-adv-x="{EM}"
             container:code="{code}"
             container:group="{group}"
             container:index="{index}"
             container:polarity="{pol}"
             container:sovereignty="{sov}"
             container:meaning="{meaning}"
             container:time_slot="{time_slot}"
             container:pulse="{pulse}"
             container:native_code="{name}_{code:02X}_{pol}_{sov}"
             d="{path}"
             fill="hsl({hue}, 80%, 60%)"
             fill-opacity="{alpha}"
             stroke="hsl({hue}, 90%, 30%)"
             stroke-width="8">
        <animate attributeName="opacity"
                 values="{alpha};{float(alpha)*0.5:.2f};{alpha}"
                 dur="{1.0 + float(pulse)*0.5:.1f}s"
                 repeatCount="indefinite"/>
      </glyph>"""
        glyphs_xml.append(glyph_xml)
    
    svg_footer = f"""
    </font>
  </defs>
  
  <!-- 字体名注释 (隐藏文本, 可被机器解析) -->
  <text x="0" y="-100" font-family="{FONT_FAMILY}" font-size="60" fill="none">
    HanYuan-Container v0.1 · 64容器 · 48声母 · 108韵母 · 满全法
  </text>
</svg>
"""
    
    return svg_header + "".join(glyphs_xml) + svg_footer


def _shape_path(shape_name: str, em: int, baseline: int) -> str:
    """根据拓扑类型生成 SVG 路径 (2D 投影简化版)"""
    cx = em / 2      # 中心 x
    cy = em / 2      # 中心 y
    r = em * 0.35    # 半径
    r2 = em * 0.25   # 内半径
    baseline_y = baseline * 0.8
    
    if shape_name == "box":
        # 盒 (立方体透视)
        return f"M{cx-r},{cy-r} L{cx+r},{cy-r} L{cx+r},{cy+r} L{cx-r},{cy+r} Z M{cx-r},{cy-r} L{cx-r2},{cy-r2} L{cx+r2},{cy-r2} L{cx+r},{cy-r} M{cx+r},{cy-r} L{cx+r2},{cy-r2} L{cx+r2},{cy+r2} L{cx+r},{cy+r}"
    elif shape_name == "sphere":
        # 球 (同心圆 + 渐变)
        return f"M{cx-r},{cy} A{r},{r} 0 1 0 {cx+r},{cy} A{r},{r} 0 1 0 {cx-r},{cy} Z M{cx-r2},{cy} A{r2},{r2} 0 1 1 {cx+r2},{cy} A{r2},{r2} 0 1 1 {cx-r2},{cy}"
    elif shape_name == "torus":
        # 环面
        return f"M{cx-r},{cy} A{r},{r*0.6} 0 1 0 {cx+r},{cy} A{r},{r*0.6} 0 1 0 {cx-r},{cy} Z M{cx-r2},{cy} A{r2},{r2*0.6} 0 1 1 {cx+r2},{cy} A{r2},{r2*0.6} 0 1 1 {cx-r2},{cy} Z"
    elif shape_name == "bipyramid":
        # 双锥
        return f"M{cx},{cy-r} L{cx+r},{cy} L{cx},{cy+r} L{cx-r},{cy} Z M{cx},{cy-r2} L{cx+r2},{cy} L{cx},{cy+r2} L{cx-r2},{cy} Z"
    elif shape_name == "cylinder":
        # 圆柱 (椭圆面)
        return f"M{cx-r},{cy-r2} L{cx-r},{cy+r2} A{r},{r2} 0 0 0 {cx+r},{cy+r2} L{cx+r},{cy-r2} A{r},{r2} 0 0 0 {cx-r},{cy-r2} Z"
    else:
        # 默认方框
        return f"M{cx-r},{cy-r} L{cx+r},{cy-r} L{cx+r},{cy+r} L{cx-r},{cy+r} Z"


def main():
    import os
    
    output_path = os.path.dirname(os.path.abspath(__file__))
    svg_content = generate_svg_font()
    
    # 写入文件
    svg_path = os.path.join(output_path, "han_yuan_container.svg")
    with open(svg_path, "w", encoding="utf-8") as f:
        f.write(svg_content)
    
    print(f"✅ 输出: {svg_path}")
    print(f"✅ 大小: {len(svg_content)} 字节")
    print(f"> 字形数: {len(SAMPLE_CONTAINERS)} (64容器的8字体样本)")
    print(f"> 每字形承载: [图形层SVG路径] + [容器码5属性] + [时间窗2属性] + [语义名1属性]")
    print(f"> 引用: encoding/z-api.txt (48声母/108韵母) + container_radix.rs (64容器)")
    print(f"> 进制: X=48声母 Y=64容器 Z=116层级 (加法循环计数, 非位运算)")
    print(f"> 引用Skill: glyph-3d-font(拓扑映射) + machine-code-container(容器封装) + time-fridge-execution(Z轴时间)")
    
    # 输出字形解码表
    print(f"\n字形解码表:")
    print("-" * 70)
    print(f"{'编码':>4} | {'名称':<10} | {'群':>3} | {'极性':<8} | {'主权':<12} | {'时间窗'} | 声调映射")
    print("-" * 70)
    
    for (group, index, name, meaning, pol, sov) in SAMPLE_CONTAINERS:
        base = 0
        for _ in range(group):
            base = base + 16
        code = base + index
        time_slot = (index * 37 + group * 73) % 256
        pulse = time_pulse(time_slot)
        print(f"  {code:>2}  | {name:<10} | {group:>3} | {pol:<8} | {sov:<12} | {time_slot:>5}    | {pulse}Hz")
    
    print("-" * 70)
    print(f"容器群: 0=容器族 1=操作族 2=声母 3=韵母 4=笔画")
    print(f"极性三元: YIN(蓝/0) YANG(橙/1) NEUTRAL(绿/0.5)")
    print(f"主权四元: ACTIVE(1.0) PASSIVE(0.7) ACTIVE_SUB(0.85) PASSIVE_SUB(0.6)")


if __name__ == "__main__":
    main()
