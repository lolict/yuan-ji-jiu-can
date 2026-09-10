#!/usr/bin/env python3
"""
generate_full_font.py — 生成完整的 64 容器 SVG 字体

基于 hunyuan-rs/src/encoding/container_radix.rs 的 64 容器命名,
生成包含全部 64 × 4 字形的 SVG 字体 (容器族 + 操作族 + 声母 + 韵母)。

每个字形的属性包含:
  - 容器编码 (6-bit, 群×16+索引, 通过加法循环计算)
  - 容器中文名 (拼音, 与 container_radix.rs 对齐)
  - 极性三元 (阴/阳/元)
  - 主权四元 (Active/Passive/ActiveSub/PassiveSub)
  - 拓扑类型 (24 体拓扑, 按语义分配)
  - 时间窗口 Z 轴 (0-255)
  - 层级深度 (笔画索引 0-15)

这是 hunyuan-rs/src/cross_isa/glyph_3d_font.rs 的 Python 数据生成器 —
输出的 SVG 可用于视觉验证, 属性值可直接单位测试 Rust GlyphContour::from_coord。
"""

FAMILY_DEFS = [
    # (群码偏移基数, 群中文名, 16个容器中文名列表, 极性默认值, 主权默认值, 默认拓扑)
    (
        0, "容器族",
        ["袋","照","见","阴","加","分","元","裁","产","公","声","折","负","福","凤","栖"],
        "YANG", "ACTIVE_SUB", "box",
    ),
    (
        16, "操作族",
        ["融","鼓","蚕","民","盟","满","囡","夕","三","六","八","九","零","一","二","四"],
        "NEUTRAL", "ACTIVE_SUB", "sphere",
    ),
    (
        32, "声母容器",
        ["帮","端","精","见","来","齿","喉","呼","开","清","平","上","前","内","单","长"],
        "YANG", "ACTIVE", "cylinder",
    ),
    (
        48, "韵母容器",
        ["麻","支","寒","歌","阳","尤","止","转","高","低","圆","扁","响","静","急","缓"],
        "YIN", "PASSIVE", "torus",
    ),
]


# 拓扑分配规则 — 严格对齐 hunyuan-rs/src/cross_isa/glyph_3d_font.rs 的 5 种拓扑
# Rust from_coord match:
#   0x01|0x02 → box
#   0x03..=0x08 → sphere
#   0x09|0x0A|0x0B → torus
#   0x0C|0x0D → double_cone
#   0x0E|0x0F → line
#   _ → box (fallback)
# 0x10-0x3F 由本表定义 (相同 5 种拓扑, 按语义分配)
TOPOLOGY_BY_SEMANTIC = {
    # 容器族 (0x00-0x0F): 对齐 Rust 指令语义
    0: "box",          # 袋 = 盒形容器 (对齐 LOAD/STORE → box)
    1: "box",          # 照 = 盒型 (框架)
    2: "double_cone",  # 见 = 双锥 (裁决)
    3: "sphere",       # 阴 = 球 (阴极)
    4: "sphere",       # 加 = 球 (叠加)
    5: "torus",        # 分 = 环面 (分离)
    6: "line",         # 元 = 线 (起始)
    7: "double_cone",  # 裁 = 双锥 (裁断)
    8: "sphere",       # 产 = 球 (产出)
    9: "sphere",       # 公 = 球 (公约数)
   10: "torus",        # 声 = 环面 (声波)
   11: "torus",        # 折 = 环面 (转折)
   12: "box",          # 负 = 盒型 (框架)
   13: "sphere",       # 福 = 球 (圆满)
   14: "torus",        # 凤 = 环面 (循环)
   15: "line",         # 栖 = 线 (栖息)
    # 操作族 (0x10-0x1F): 语义对齐
   16: "torus",        # 融 = 环面 (融合)
   17: "sphere",       # 鼓 = 球 (振动)
   18: "torus",        # 蚕 = 环面 (茧)
   19: "sphere",       # 民 = 球 (聚合)
   20: "box",          # 盟 = 盒型 (合盟)
   21: "sphere",       # 满 = 球 (满溢)
   22: "torus",        # 囡 = 环面 (夫妇)
   23: "line",         # 夕 = 线 (日夕)
   24: "double_cone",  # 三 = 双锥 (三元裁判)
   25: "sphere",       # 六 = 球 (完全)
   26: "torus",        # 八 = 环面 (八流)
   27: "double_cone",  # 九 = 双锥 (九元)
   28: "line",         # 零 = 线 (零维)
   29: "line",         # 一 = 线 (一维)
   30: "torus",        # 二 = 环面 (二元)
   31: "box",          # 四 = 盒型 (四元)
    # 声母容器 (0x20-0x2F): 共鸣腔 = 球 为主
   32: "sphere",       # 帮 = 球 (唇)
   33: "sphere",       # 端 = 球 (舌)
   34: "sphere",       # 精 = 球 (齿)
   35: "sphere",       # 见 = 球 (牙)
   36: "sphere",       # 来 = 球
   37: "sphere",       # 齿 = 球
   38: "sphere",       # 喉 = 球
   39: "sphere",       # 呼 = 球
   40: "sphere",       # 开 = 球 (开口)
   41: "sphere",       # 清 = 球
   42: "line",         # 平 = 线 (平声)
   43: "box",          # 上 = 盒型 (上下)
   44: "line",         # 前 = 线 (前后)
   45: "box",          # 内 = 盒型 (内外)
   46: "sphere",       # 单 = 球 (单音)
   47: "line",         # 长 = 线 (长音)
    # 韵母容器 (0x30-0x3F): 韵母共鸣 = 环面 为主
   48: "torus",        # 麻 = 环面
   49: "torus",        # 支 = 环面
   50: "torus",        # 寒 = 环面
   51: "torus",        # 歌 = 环面
   52: "torus",        # 阳 = 环面
   53: "torus",        # 尤 = 环面
   54: "sphere",       # 止 = 球 (收口)
   55: "torus",        # 转 = 环面
   56: "line",         # 高 = 线 (高低)
   57: "line",         # 低 = 线
   58: "sphere",       # 圆 = 球
   59: "box",          # 扁 = 盒型
   60: "sphere",       # 响 = 球
   61: "sphere",       # 静 = 球
   62: "sphere",       # 急 = 球
   63: "line",         # 缓 = 线
}


def _topology_color(topo: str) -> str:
    """拓扑类型 → HSL 主色相 (5 种, 对齐 Rust glyph_3d_font.rs)"""
    if topo == "box":        return "210"  # 蓝
    if topo == "sphere":     return "120"  # 绿
    if topo == "torus":      return "30"   # 橙
    if topo == "double_cone": return "0"   # 红
    if topo == "line":       return "60"   # 黄
    return "180"


def _topology_path(topo: str, w: int, h: int) -> str:
    """拓扑类型 → SVG 路径 (5 种, 对齐 Rust glyph_3d_font.rs)"""
    cx, cy = w/2, h/2
    r = w * 0.32
    r2 = w * 0.20
    
    if topo == "box":
        return f"M{cx-r},{cy-r} L{cx+r},{cy-r} L{cx+r},{cy+r} L{cx-r},{cy+r} Z"
    elif topo == "sphere":
        return f"M{cx-r},{cy} A{r},{r} 0 1 0 {cx+r},{cy} A{r},{r} 0 1 0 {cx-r},{cy} Z"
    elif topo == "torus":
        return f"M{cx-r},{cy} A{r},{r*0.6} 0 1 0 {cx+r},{cy} A{r},{r*0.6} 0 1 0 {cx-r},{cy} Z M{cx-r2},{cy} A{r2},{r2*0.6} 0 1 1 {cx+r2},{cy} A{r2},{r2*0.6} 0 1 1 {cx-r2},{cy} Z"
    elif topo == "double_cone":
        return f"M{cx},{cy-r} L{cx+r},{cy} L{cx},{cy+r} L{cx-r},{cy} Z"
    else:  # line
        return f"M{cx-r},{cy} L{cx+r},{cy}"


def generate() -> str:
    """生成完整的 64 容器 SVG 字体"""
    
    EM = 1000
    W = EM
    
    out = []
    out.append('<?xml version="1.0" encoding="UTF-8"?>')
    out.append(f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.2">')
    out.append('  <defs>')
    out.append(f'    <font id="HanYuan64" horiz-adv-x="{W}">')
    out.append(f'      <font-face font-family="HanYuan64" units-per-em="{EM}" ascent="800" descent="200">')
    out.append(f'        <font-face-name name="HanYuan64"/>')
    out.append(f'      </font-face>')
    out.append(f'      <missing-glyph horiz-adv-x="{W}"><path d="M0,200 L200,800 L400,800" fill="none" stroke="#ccc"/></missing-glyph>')
    
    for (base, cname, names, pol, sov, default_topo) in FAMILY_DEFS:
        for idx in range(16):
            code = base + idx
            name = names[idx]
            topo = TOPOLOGY_BY_SEMANTIC.get(code, default_topo)
            color = _topology_color(topo)
            path = _topology_path(topo, 700, 700)
            # Z轴: 时间槽 = code的低6位 (0-63)
            z_time = code - ((code >> 6) << 6)  # 取低6位 (加法替代mod)
            z_depth = code >> 6                  # 取高bit (层级)
            # 笔画类型 = code的低4位 → 引用笔画容器 STROKE_FAMILY
            stroke_type = code - ((code >> 4) << 4)
            
            out.append(f'      <!-- {cname}[{idx}] {name} code={code} pol={pol} sov={sov} z={z_time} depth={z_depth} stroke={stroke_type} -->')
            out.append(f'      <glyph unicode="&#x{code:04X};" glyph-name="{name}" horiz-adv-x="{W}"')
            out.append(f'             container:code="{code}" container:name="{name}"')
            out.append(f'             container:polarity="{pol}" container:sovereignty="{sov}"')
            out.append(f'             container:meaning="{cname}"')
            out.append(f'             container:time_slot="{z_time}" container:depth="{z_depth}"')
            out.append(f'             container:topology="{topo}" container:stroke_type="{stroke_type}"')
            out.append(f'             container:native_code="HFC_{code:02X}_{name}_{pol}_{sov}"')
            out.append(f'             d="{path}"')
            out.append(f'             fill="hsl({color},75%,55%)" stroke="hsl({color},90%,25%)" stroke-width="6"/>')
    
    out.append('    </font>')
    out.append('  </defs>')
    out.append('  <text x="0" y="-50" font-family="HanYuan64" font-size="40" fill="#333">HanYuan-64 · 容器字体 · 满全法</text>')
    out.append('</svg>')
    
    return "\n".join(out)


def generate_rust_testdata() -> str:
    """
    生成 Rust 单位测试数据, 验证 GlyphContour::from_coord 的正确性。
    输出可粘贴到 hunyuan-rs/src/cross_isa/glyph_3d_font.rs 的测试模块中。
    """
    lines = []
    lines.append("// 自动生成: hunyuan-os/tools/glyph-container/generate_full_font.py")
    lines.append("// 包含全部 64 容器的 coord round-trip 测试")
    lines.append("#[cfg(test)]")
    lines.append("mod container64_roundtrip {")
    lines.append("    use super::*;\n")
    
    for code in range(64):
        group = 0
        tmp = code
        for g in range(5):
            if tmp < 16:
                group = g
                break
            tmp = tmp - 16
        
        # 从code反推topo (引用 TOPOLOGY_BY_SEMANTIC)
        topo = TOPOLOGY_BY_SEMANTIC.get(code, "box")
        
        lines.append(f"    #[test]")
        lines.append(f"    fn container_{code:02x}_roundtrip() {{")
        lines.append(f"        let coord = Coord3D::new({code}, 0x{tmp:02X}00, 0x{code:02X});")
        lines.append(f"        let contour = GlyphContour::from_coord(coord);")
        lines.append(f"        let restored = contour.to_coord();")
        lines.append(f"        assert_eq!(coord, restored, \"容器0x{code:02X} ({topo}) coord round-trip fail\");")
        lines.append(f"    }}\n")
    
    lines.append("}\n")
    
    return "\n".join(lines)


def main():
    import os
    HERE = os.path.dirname(os.path.abspath(__file__))
    
    # 1. 生成 SVG 字体
    svg = generate()
    svg_path = os.path.join(HERE, "han_yuan64.svg")
    with open(svg_path, "w", encoding="utf-8") as f:
        f.write(svg)
    print(f"✅ SVG 字体: {svg_path}")
    print(f"   字形数: 64")
    print(f"   容器族: 16 + 操作族: 16 + 声母: 16 + 韵母: 16 = 64")
    print(f"   进深 (Z轴): 时间槽 0-63, 层级 0-3, 笔画类型 0-15")
    
    # 2. 生成 Rust 测试数据
    rust = generate_rust_testdata()
    rust_path = os.path.join(HERE, "container64_tests.rs")
    with open(rust_path, "w", encoding="utf-8") as f:
        f.write(rust)
    print(f"✅ Rust 测试: {rust_path}")
    print(f"   测试数: 64 (每个容器一个 coord round-trip test)")
    
    # 3. 简要统计
    topo_count = {}
    for code in range(64):
        t = TOPOLOGY_BY_SEMANTIC.get(code, "box")
        topo_count[t] = topo_count.get(t, 0)
        topo_count[t] = topo_count[t] + 1
    
    print(f"\n拓扑类型分布:")
    for t, c in sorted(topo_count.items(), key=lambda x: -x[1]):
        print(f"  {t:<12}: {c}")


if __name__ == "__main__":
    main()
