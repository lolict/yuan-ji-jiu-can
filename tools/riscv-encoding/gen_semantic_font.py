#!/usr/bin/env python3
"""Cross-ISA Semantic Font Generator - U+F100..F10F"""
import sys,os
try:
    from fontTools.pens.ttGlyphPen import TTGlyphPen
    from fontTools.fontBuilder import FontBuilder
except ImportError:
    print("pip install fonttools");sys.exit(1)

GLYPHS=[("sem_add",0xF100),("sem_sub",0xF101),("sem_and",0xF102),("sem_or",0xF103),
        ("sem_xor",0xF104),("sem_sll",0xF105),("sem_srl",0xF106),("sem_sra",0xF107),
        ("sem_slt",0xF108),("sem_load",0xF109),("sem_store",0xF10A),("sem_beq",0xF10B),
        ("sem_bne",0xF10C),("sem_jmp",0xF10D),("sem_jmr",0xF10E),("sem_lui",0xF10F)]

def build_font(out):
    go=[".notdef"];gl={".notdef":TTGlyphPen(None).glyph()};cm={}
    for name,cp in GLYPHS:
        go.append(name);p=TTGlyphPen(None)
        y=(cp-0xF100)*30+200
        p.moveTo((150,y));p.lineTo((350,y));p.lineTo((350,y+200));p.lineTo((150,y+200));p.closePath()
        gl[name]=p.glyph();cm[cp]=name
    fb=FontBuilder(1024,isTTF=True);fb.setupGlyphOrder(go);fb.setupGlyf(gl);fb.setupCharacterMap(cm)
    fb.setupNameTable({"familyName":"NJSG Semantics","styleName":"Regular"})
    fb.setupHead(unitsPerEm=1024);fb.setupHorizontalHeader(ascent=800,descent=-200)
    fb.setupHorizontalMetrics({g:(500,0) for g in go})
    fb.setupOS2(sTypoAscender=800,sTypoDescender=-200,usWinAscent=1024,usWinDescent=256)
    fb.setupPost();fb.font.save(out)

def main():
    out=sys.argv[1] if len(sys.argv)>1 else "semantics.ttf"
    build_font(out);print(f"Font: {out} ({os.path.getsize(out)} bytes)")
    import json
    mc={"sem_add":{"RiscV":"00208033","ARM64":"8B020020","X86_64":"01D8","MIPS":"00221020"},
        "sem_sub":{"RiscV":"40208033","ARM64":"CB020020","X86_64":"29D8","MIPS":"00221022"},
        "sem_and":{"RiscV":"00207033","ARM64":"8A020020","X86_64":"21D8"},
        "sem_or":{"RiscV":"00206033","ARM64":"AA020020","X86_64":"09D8"},
        "sem_xor":{"RiscV":"00204033","ARM64":"CA020020","X86_64":"31D8"},
        "sem_sll":{"RiscV":"00201033"},"sem_srl":{"RiscV":"00205033"},"sem_sra":{"RiscV":"40205033"},
        "sem_slt":{"RiscV":"0020A033"},"sem_load":{"RiscV":"0080A003","ARM64":"F9400420","MIPS":"8C220008"},
        "sem_store":{"RiscV":"0020A423","ARM64":"F9000420","MIPS":"AC220008"},
        "sem_beq":{"RiscV":"00200863","ARM64":"04000054"},"sem_bne":{"RiscV":"00201863"},
        "sem_jmp":{"RiscV":"0080006F"},"sem_jmr":{"RiscV":"00008067","ARM64":"D65F03C0","X86_64":"FFE0"},
        "sem_lui":{"RiscV":"12345037"}}
    jp=out.replace(".ttf","_mc.json")
    with open(jp,"w") as jf:json.dump(mc,jf,indent=2)
    print(f"MC map: {jp} ({os.path.getsize(jp)} bytes)")

if __name__=="__main__":main()
