#!/usr/bin/env python3
"""
跨 ISA 语义字体生成器
同一字体内的字形按当前 ISA 上下文渲染不同机器码

语义符号: U+F100..U+F10F (跨 ISA 统一)
字形设计: 每种语义一个独特的可视符号

用法:
  python3 gen_semantic_font.py [输出路径]
"""

import sys
import json
import os

try:
    from fontTools.ttLib import TTFont
    from fontTools.pens.ttGlyphPen import TTGlyphPen
    from fontTools.fontBuilder import FontBuilder
except ImportError:
    print("安装依赖: pip install fonttools")
    sys.exit(1)

# ── 字形绘制函数 ─────────────────────────────────────────

def draw_notdef(pen):
    """notdef: 空心矩形"""
    pen.moveTo((100, 0))
    pen.lineTo((400, 0))
    pen.lineTo((400, 700))
    pen.lineTo((100, 700))
    pen.closePath()
    pen.moveTo((140, 40))
    pen.lineTo((140, 660))
    pen.lineTo((360, 660))
    pen.lineTo((360, 40))
    pen.closePath()

def draw_plus(pen):
    """semAdd: 加号 +"""
    pen.moveTo((200, 480))
    pen.lineTo((800, 480))
    pen.lineTo((800, 520))
    pen.lineTo((200, 520))
    pen.closePath()
    pen.moveTo((480, 200))
    pen.lineTo((520, 200))
    pen.lineTo((520, 800))
    pen.lineTo((480, 800))
    pen.closePath()

def draw_minus(pen):
    """semSub: 减号 -"""
    pen.moveTo((200, 500))
    pen.lineTo((800, 500))
    pen.lineTo((800, 540))
    pen.lineTo((200, 540))
    pen.closePath()

def draw_and(pen):
    """semAnd: 与 ∧"""
    pen.moveTo((500, 200))
    pen.lineTo((750, 700))
    pen.lineTo((710, 720))
    pen.lineTo((500, 320))
    pen.lineTo((290, 720))
    pen.lineTo((250, 700))
    pen.closePath()

def draw_or(pen):
    """semOr: 或 ∨"""
    pen.moveTo((500, 800))
    pen.lineTo((750, 300))
    pen.lineTo((710, 280))
    pen.lineTo((500, 680))
    pen.lineTo((290, 280))
    pen.lineTo((250, 300))
    pen.closePath()

def draw_xor(pen):
    """semXor: 异或 ⊕"""
    pen.moveTo((200, 480))
    pen.lineTo((800, 480))
    pen.lineTo((800, 520))
    pen.lineTo((200, 520))
    pen.closePath()
    pen.moveTo((480, 200))
    pen.lineTo((520, 200))
    pen.lineTo((520, 800))
    pen.lineTo((480, 800))
    pen.closePath()
    pen.moveTo((300, 650))
    pen.qCurveTo((500, 750), (700, 650))
    pen.lineTo((690, 680))
    pen.qCurveTo((500, 770), (310, 680))
    pen.closePath()

def draw_sll(pen):
    """semSll: 逻辑左移 <<"""
    pen.moveTo((350, 500))
    pen.lineTo((550, 250))
    pen.lineTo((550, 300))
    pen.lineTo((400, 500))
    pen.lineTo((550, 700))
    pen.lineTo((550, 750))
    pen.closePath()
    pen.moveTo((500, 500))
    pen.lineTo((700, 250))
    pen.lineTo((700, 300))
    pen.lineTo((550, 500))
    pen.lineTo((700, 700))
    pen.lineTo((700, 750))
    pen.closePath()

def draw_srl(pen):
    """semSrl: 逻辑右移 >>"""
    pen.moveTo((450, 500))
    pen.lineTo((250, 250))
    pen.lineTo((250, 300))
    pen.lineTo((400, 500))
    pen.lineTo((250, 700))
    pen.lineTo((250, 750))
    pen.closePath()
    pen.moveTo((600, 500))
    pen.lineTo((400, 250))
    pen.lineTo((400, 300))
    pen.lineTo((550, 500))
    pen.lineTo((400, 700))
    pen.lineTo((400, 750))
    pen.closePath()

def draw_sra(pen):
    """semSra: 算术右移 >>> (带符号扩展)"""
    pen.moveTo((450, 500))
    pen.lineTo((250, 250))
    pen.lineTo((250, 300))
    pen.lineTo((400, 500))
    pen.lineTo((250, 700))
    pen.lineTo((250, 750))
    pen.closePath()
    pen.moveTo((600, 500))
    pen.lineTo((400, 250))
    pen.lineTo((400, 300))
    pen.lineTo((550, 500))
    pen.lineTo((400, 700))
    pen.lineTo((400, 750))
    pen.closePath()
    pen.moveTo((200, 300))
    pen.lineTo((220, 300))
    pen.lineTo((220, 700))
    pen.lineTo((200, 700))
    pen.closePath()

def draw_slt(pen):
    """semSlt: 小于 <"""
    pen.moveTo((700, 250))
    pen.lineTo((300, 500))
    pen.lineTo((700, 750))
    pen.lineTo((700, 700))
    pen.lineTo((380, 500))
    pen.lineTo((700, 300))
    pen.closePath()

def draw_load(pen):
    """semLoad: 加载 ↓ (内存到寄存器)"""
    pen.moveTo((500, 250))
    pen.lineTo((540, 250))
    pen.lineTo((540, 550))
    pen.lineTo((600, 550))
    pen.lineTo((520, 700))
    pen.lineTo((440, 550))
    pen.lineTo((500, 550))
    pen.closePath()

def draw_store(pen):
    """semStore: 存储 ↑ (寄存器到内存)"""
    pen.moveTo((500, 750))
    pen.lineTo((540, 750))
    pen.lineTo((540, 450))
    pen.lineTo((600, 450))
    pen.lineTo((520, 300))
    pen.lineTo((440, 450))
    pen.lineTo((500, 450))
    pen.closePath()

def draw_branch_eq(pen):
    """semBranchEq: 分支相等 =="""
    pen.moveTo((250, 420))
    pen.lineTo((550, 420))
    pen.lineTo((550, 460))
    pen.lineTo((250, 460))
    pen.closePath()
    pen.moveTo((250, 540))
    pen.lineTo((550, 540))
    pen.lineTo((550, 580))
    pen.lineTo((250, 580))
    pen.closePath()
    pen.moveTo((600, 300))
    pen.lineTo((750, 200))
    pen.lineTo((750, 250))
    pen.lineTo((700, 250))
    pen.lineTo((700, 300))
    pen.closePath()
    pen.moveTo((650, 250))
    pen.lineTo((750, 200))
    pen.lineTo((700, 200))
    pen.closePath()

def draw_branch_ne(pen):
    """semBranchNe: 不等 !="""
    pen.moveTo((250, 420))
    pen.lineTo((450, 420))
    pen.lineTo((450, 460))
    pen.lineTo((250, 460))
    pen.closePath()
    pen.moveTo((250, 540))
    pen.lineTo((450, 540))
    pen.lineTo((450, 580))
    pen.lineTo((250, 580))
    pen.closePath()
    pen.moveTo((500, 300))
    pen.lineTo((650, 200))
    pen.lineTo((650, 250))
    pen.lineTo((600, 250))
    pen.lineTo((600, 300))
    pen.closePath()
    pen.moveTo((550, 250))
    pen.lineTo((650, 200))
    pen.lineTo((600, 200))
    pen.closePath()
    pen.moveTo((300, 650))
    pen.lineTo((500, 350))
    pen.lineTo((520, 370))
    pen.lineTo((320, 670))
    pen.closePath()

def draw_jump(pen):
    """semJump: 跳转 ↪"""
    pen.moveTo((200, 600))
    pen.lineTo((600, 600))
    pen.lineTo((600, 400))
    pen.lineTo((650, 400))
    pen.lineTo((600, 300))
    pen.lineTo((550, 400))
    pen.lineTo((600, 400))
    pen.closePath()
    pen.moveTo((200, 600))
    pen.lineTo((200, 560))
    pen.lineTo((560, 560))
    pen.lineTo((560, 380))
    pen.closePath()

def draw_jump_reg(pen):
    """semJumpReg: 间接跳转"""
    pen.moveTo((200, 600))
    pen.lineTo((600, 600))
    pen.lineTo((600, 400))
    pen.lineTo((650, 400))
    pen.lineTo((600, 300))
    pen.lineTo((550, 400))
    pen.lineTo((600, 400))
    pen.closePath()
    pen.moveTo((200, 600))
    pen.lineTo((200, 560))
    pen.lineTo((560, 560))
    pen.lineTo((560, 380))
    pen.closePath()
    pen.moveTo((680, 650))
    pen.lineTo((750, 720))
    pen.lineTo((740, 730))
    pen.lineTo((670, 660))
    pen.closePath()
    pen.moveTo((680, 720))
    pen.lineTo((750, 650))
    pen.lineTo((740, 660))
    pen.lineTo((670, 730))
    pen.closePath()

def draw_lui(pen):
    """semLui: 高位立即数"""
    pen.moveTo((500, 200))
    pen.lineTo((540, 200))
    pen.lineTo((540, 500))
    pen.lineTo((600, 500))
    pen.lineTo((520, 650))
    pen.lineTo((440, 500))
    pen.lineTo((500, 500))
    pen.closePath()
    pen.moveTo((620, 300))
    pen.lineTo((640, 300))
    pen.lineTo((640, 450))
    pen.lineTo((700, 450))
    pen.lineTo((700, 300))
    pen.lineTo((720, 300))
    pen.lineTo((720, 650))
    pen.lineTo((700, 650))
    pen.lineTo((700, 500))
    pen.lineTo((640, 500))
    pen.lineTo((640, 650))
    pen.lineTo((620, 650))
    pen.closePath()

# ── 符号表 ───────────────────────────────────────────────
SEM_GLYPHS = [
    ("sem_add",       0xF100, draw_plus),
    ("sem_sub",       0xF101, draw_minus),
    ("sem_and",       0xF102, draw_and),
    ("sem_or",        0xF103, draw_or),
    ("sem_xor",       0xF104, draw_xor),
    ("sem_sll",       0xF105, draw_sll),
    ("sem_srl",       0xF106, draw_srl),
    ("sem_sra",       0xF107, draw_sra),
    ("sem_slt",       0xF108, draw_slt),
    ("sem_load",      0xF109, draw_load),
    ("sem_store",     0xF10A, draw_store),
    ("sem_branch_eq", 0xF10B, draw_branch_eq),
    ("sem_branch_ne", 0xF10C, draw_branch_ne),
    ("sem_jump",      0xF10D, draw_jump),
    ("sem_jump_reg",  0xF10E, draw_jump_reg),
    ("sem_lui",       0xF10F, draw_lui),
]

# ── 机器码查找表 (按 ISA 索引) ──────────────────────────
MACHINE_CODES = {
    "sem_add":       {"RiscV": "00208033", "Arm64": "8B020020", "X86_64": "01D8",  "Mips": "00221020"},
    "sem_sub":       {"RiscV": "40208033", "Arm64": "CB020020", "X86_64": "29D8",  "Mips": "00221022"},
    "sem_and":       {"RiscV": "00207033", "Arm64": "8A020020", "X86_64": "21D8",  "Mips": None},
    "sem_or":        {"RiscV": "00206033", "Arm64": "AA020020", "X86_64": "09D8",  "Mips": None},
    "sem_xor":       {"RiscV": "00204033", "Arm64": "CA020020", "X86_64": "31D8",  "Mips": None},
    "sem_sll":       {"RiscV": "00201033", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_srl":       {"RiscV": "00205033", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_sra":       {"RiscV": "40205033", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_slt":       {"RiscV": "0020A033", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_load":      {"RiscV": "0080A003", "Arm64": "F9400420", "X86_64": None,     "Mips": "8C220008"},
    "sem_store":     {"RiscV": "0020A423", "Arm64": "F9000420", "X86_64": None,     "Mips": "AC220008"},
    "sem_branch_eq": {"RiscV": "00200863", "Arm64": "04000054", "X86_64": None,     "Mips": None},
    "sem_branch_ne": {"RiscV": "00201863", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_jump":      {"RiscV": "0080006F", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_jump_reg":  {"RiscV": "00008067", "Arm64": None,       "X86_64": None,     "Mips": None},
    "sem_lui":       {"RiscV": "12345037", "Arm64": None,       "X86_64": None,     "Mips": None},
}

def build_font(output_path):
    glyph_order = [".notdef"]
    glyphs = {".notdef": TTGlyphPen(None).glyph()}
    cmap = {}

    # 先创建 notdef
    pen = TTGlyphPen(None)
    draw_notdef(pen)
    glyphs[".notdef"] = pen.glyph()

    for name, codepoint, draw_fn in SEM_GLYPHS:
        glyph_order.append(name)
        pen = TTGlyphPen(None)
        draw_fn(pen)
        glyphs[name] = pen.glyph()
        cmap[codepoint] = name

    fb = FontBuilder(1024, isTTF=True)
    fb.setupGlyphOrder(glyph_order)
    fb.setupGlyf(glyphs)
    fb.setupCharacterMap(cmap)
    fb.setupNameTable({
        "familyName": "NJSG Semantics",
        "styleName": "Regular",
        "psName": "NJSG-Semantics-Regular",
    })
    fb.setupHead(unitsPerEm=1024)
    fb.setupHorizontalHeader(ascent=800, descent=-200)
    fb.setupHorizontalMetrics({g: (500, 0) for g in glyph_order})
    fb.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWinAscent=1024, usWinDescent=256)
    fb.setupPost()

    fb.font.save(output_path)
    return fb.font

if __name__ == "__main__":
    output = sys.argv[1] if len(sys.argv) > 1 else "semantics.ttf"
    font = build_font(output)

    json_path = output.replace(".ttf", "_mc.json")
    with open(json_path, "w") as f:
        json.dump(MACHINE_CODES, f, indent=2)

    print(f"语义字体: {output} ({os.path.getsize(output)} bytes)")
    print(f"机器码映射: {json_path}")
    print(f"字形数: {len(SEM_GLYPHS)}")
    print(f"cmap 条目: {len(font.getBestCmap())}")
    print()
    print("符号 -> 码点 -> 机器码:")
    for name, cp, _ in SEM_GLYPHS:
        mc = MACHINE_CODES[name]
        rv = mc.get("RiscV") or "--"
        arm = mc.get("Arm64") or "--"
        print(f"  {name:14s} U+{cp:04X}  RV={rv}  ARM={arm}")
