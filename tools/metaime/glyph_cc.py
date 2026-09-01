#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
字形几何 → C 编译器

将音节程序编译为可执行的C代码。

编译策略:
- 每个音节 → 一个结构体实例 (GlyphSyllable)
- 每个声母操作 → 一个C函数调用
- 力场演化 → 主循环中的迭代计算
- 加法统摄 → C中的加法运算
"""

import os
import sys
import subprocess
from typing import List, Dict

from glyph_engine import GlyphParser, GLYPH_REGISTRY, YUNMU_VECTORS, TONE_ENERGY


# ============================================================
# 操作到C函数的映射
# ============================================================

OP_TO_C_FUNC = {
    'b': 'glyph_install',
    'p': 'glyph_extract',
    'm': 'glyph_traverse',
    'f': 'glyph_friction',
    'd': 'glyph_locate',
    't': 'glyph_anchor',
    'n': 'glyph_compare',
    'l': 'glyph_connect',
    'g': 'glyph_spiral',
    'k': 'glyph_support',
    'h': 'glyph_bear',
    'j': 'glyph_capture',
    'q': 'glyph_trace',
    'x': 'glyph_intersect',
    'zh': 'glyph_fold',
    'ch': 'glyph_wrap',
    'sh': 'glyph_flow',
    'r': 'glyph_branch',
    'z': 'glyph_oscillate',
    'c': 'glyph_enclose',
    's': 'glyph_transfer',
    'y': 'glyph_split',
    'w': 'glyph_merge',
}


# ============================================================
# C代码生成器
# ============================================================

def generate_c_header() -> str:
    """生成C头文件和类型定义"""
    return r"""/* 字形几何编译输出 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

/* 几何类型 */
typedef enum {
    GEO_STRAIGHT, GEO_CURVE, GEO_CROSS, GEO_BEND,
    GEO_CLOSED, GEO_OPEN, GEO_SPIRAL, GEO_POINT
} GeoType;

/* 利用区 */
typedef enum {
    ZONE_TOP, ZONE_BOTTOM, ZONE_MIDDLE, ZONE_INSIDE,
    ZONE_OUTSIDE, ZONE_JUNCTION, ZONE_ENDPOINT,
    ZONE_INFLECTION, ZONE_SPIRAL_BOUNDARY, ZONE_FINE_TRANSITION
} ZoneType;

/* 力场向量 */
typedef struct {
    float x;
    float y;
    float magnitude;
    const char* direction;
} ForceVector;

/* 音节结构体 */
typedef struct {
    char shengmu[4];
    char yunmu[4];
    int tone;
    float force;
    float energy;
    ForceVector vector;
    const char* operation;
    ZoneType zone_used;
    int geo_type;
} GlyphSyllable;

"""


def generate_c_operations() -> str:
    """生成操作函数"""
    return r"""
/* 操作函数 */

void glyph_install(GlyphSyllable* s) {
    printf("  [装入] %s%s: 直线支撑曲线，中间交界被利用 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_extract(GlyphSyllable* s) {
    printf("  [取出] %s%s: 直线悬挂曲线，中间交界被利用 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_traverse(GlyphSyllable* s) {
    printf("  [遍历] %s%s: 三个容器并排，峰谷交替 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_friction(GlyphSyllable* s) {
    printf("  [摩擦] %s%s: 三向直线，接触摩擦 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_locate(GlyphSyllable* s) {
    printf("  [定位] %s%s: 曲线直线共享起点终点 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_anchor(GlyphSyllable* s) {
    printf("  [锚定] %s%s: 交叉点固定位置，弯钩锁定 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_compare(GlyphSyllable* s) {
    printf("  [比较] %s%s: 框架结构，峰谷过渡 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_connect(GlyphSyllable* s) {
    printf("  [连接] %s%s: 纯直线，两端开口 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_spiral(GlyphSyllable* s) {
    printf("  [螺旋] %s%s: 同时产生向内和向外螺旋 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_support(GlyphSyllable* s) {
    printf("  [支撑] %s%s: 从一点向外分叉 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_bear(GlyphSyllable* s) {
    printf("  [承载] %s%s: 椅子形状，弯折处承载上方 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_capture(GlyphSyllable* s) {
    printf("  [捕获] %s%s: 直线略多于曲线，勾住标记 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_trace(GlyphSyllable* s) {
    printf("  [追踪] %s%s: 曲线自我包围，沿尾寻找 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_intersect(GlyphSyllable* s) {
    printf("  [交叉] %s%s: 两线交叉，找交集 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_fold(GlyphSyllable* s) {
    printf("  [折叠] %s%s: 折线改变方向 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_wrap(GlyphSyllable* s) {
    printf("  [包裹] %s%s: 环抱+支撑，包裹内容 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_flow(GlyphSyllable* s) {
    printf("  [流动] %s%s: 纯曲线弯转，传递力 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_branch(GlyphSyllable* s) {
    printf("  [分支] %s%s: 直线+短曲线，弯折处分二 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_oscillate(GlyphSyllable* s) {
    printf("  [振荡] %s%s: 锯齿形折转，来回切换 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_enclose(GlyphSyllable* s) {
    printf("  [包围] %s%s: 半包围，开口向右 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_transfer(GlyphSyllable* s) {
    printf("  [传递] %s%s: 纯曲线滑动，传递力 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_split(GlyphSyllable* s) {
    printf("  [分裂] %s%s: 一分为二，分叉处产生 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

void glyph_merge(GlyphSyllable* s) {
    printf("  [合并] %s%s: 二合为一，峡谷汇聚 (力=%.2f)\n",
           s->shengmu, s->yunmu, s->force);
}

"""


def generate_c_main(program: str, tokens: list) -> str:
    """生成main函数"""
    lines = []
    
    lines.append('int main(int argc, char* argv[]) {')
    lines.append(f'    printf("=== 字形几何程序 ===\\n");')
    lines.append(f'    printf("程序: {program}\\n\\n");')
    lines.append('')
    lines.append(f'    /* 初始化音节 */')
    lines.append(f'    GlyphSyllable syllables[{len(tokens)}];')
    
    # 初始化代码
    for i, token in enumerate(tokens):
        sm = token['shengmu']
        ym = token['yunmu']
        tone = token['tone']
        
        glyph = GLYPH_REGISTRY.get(sm)
        force = glyph.force_strength * TONE_ENERGY.get(tone, 1.0) if glyph else 1.0
        energy = TONE_ENERGY.get(tone, 1.0)
        vec = YUNMU_VECTORS.get(ym, YUNMU_VECTORS['a'])
        vec_x, vec_y = vec['vector']
        
        lines.append(f'    /* 音节 {i}: {sm}{ym} (声调{tone}) */')
        lines.append(f'    strcpy(syllables[{i}].shengmu, "{sm}");')
        lines.append(f'    strcpy(syllables[{i}].yunmu, "{ym}");')
        lines.append(f'    syllables[{i}].tone = {tone};')
        lines.append(f'    syllables[{i}].force = {force:.2f};')
        lines.append(f'    syllables[{i}].energy = {energy:.2f};')
        lines.append(f'    syllables[{i}].vector.x = {vec_x};')
        lines.append(f'    syllables[{i}].vector.y = {vec_y};')
        lines.append(f'    syllables[{i}].vector.magnitude = {force:.2f};')
        lines.append(f'    syllables[{i}].vector.direction = "{vec["direction"]}";')
        lines.append(f'    syllables[{i}].operation = "{glyph.operation if glyph else "未知"}";')
        lines.append('')
    
    lines.append('    printf("初始状态:\\n");')
    lines.append('    for (int i = 0; i < %d; i++) {' % len(tokens))
    lines.append('        printf("  [%d] %s%s: 力=%.2f, 能量=%.2f\\n",')
    lines.append('               i, syllables[i].shengmu, syllables[i].yunmu,')
    lines.append('               syllables[i].force, syllables[i].energy);')
    lines.append('    }')
    lines.append('    printf("\\n");')
    lines.append('')
    lines.append('    /* 执行操作序列 */')
    lines.append('    printf("执行操作:\\n");')
    
    # 执行代码
    for i, token in enumerate(tokens):
        sm = token['shengmu']
        func = OP_TO_C_FUNC.get(sm, 'glyph_connect')
        lines.append(f'    {func}(&syllables[{i}]);')
    
    lines.append('')
    lines.append('    /* 力场演化 */')
    lines.append('    printf("\\n力场演化:\\n");')
    lines.append('    for (int iter = 0; iter < 10; iter++) {')
    lines.append('        float total_force = 0;')
    lines.append('        for (int i = 0; i < %d; i++) {' % len(tokens))
    lines.append('            /* 邻居相互作用 (加法统摄) */')
    lines.append('            float neighbor_force = 0;')
    lines.append('            if (i > 0) neighbor_force += syllables[i-1].force * 0.3;')
    lines.append('            if (i < %d-1) neighbor_force += syllables[i+1].force * 0.3;' % len(tokens))
    lines.append('            syllables[i].force = syllables[i].force * 0.5 + neighbor_force + syllables[i].energy * 0.2;')
    lines.append('            total_force += syllables[i].force;')
    lines.append('        }')
    lines.append('        printf("  迭代 %d: 总力=%.2f\\n", iter+1, total_force);')
    lines.append('    }')
    lines.append('')
    lines.append('    printf("\\n=== 执行完成 ===\\n");')
    lines.append('    return 0;')
    lines.append('}')
    
    return '\n'.join(lines)


# ============================================================
# 编译器类
# ============================================================

class GlyphCCompiler:
    """字形几何 → C 编译器"""
    
    def __init__(self):
        self.parser = GlyphParser()
    
    def compile(self, program: str, output_file: str = None) -> str:
        """
        编译程序为C代码
        
        Args:
            program: 音节程序（空格分隔的音节序列）
            output_file: 输出文件路径（可选）
        
        Returns:
            生成的C代码字符串
        """
        # 解析
        tokens = self.parser.parse(program)
        if not tokens:
            return "/* 错误: 无有效音节 */"
        
        # 生成C代码
        c_code = generate_c_header()
        c_code += generate_c_operations()
        c_code += generate_c_main(program, tokens)
        
        # 写入文件
        if output_file:
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(c_code)
        
        return c_code
    
    def compile_and_run(self, program: str) -> tuple:
        """
        编译并运行程序
        
        Returns:
            (成功否, 输出)
        """
        c_file = "/tmp/glyph_out.c"
        out_file = "/tmp/glyph_out"
        
        # 编译为C
        c_code = self.compile(program, c_file)
        
        # 检测C编译器
        cc = "gcc" if subprocess.run(["which", "gcc"], capture_output=True).returncode == 0 else "cc"
        
        try:
            # 编译为可执行文件
            result = subprocess.run(
                [cc, "-o", out_file, c_file, "-lm"],
                capture_output=True, text=True, timeout=10
            )
            if result.returncode != 0:
                return False, f"编译错误:\n{result.stderr}\n\n生成的C代码前200字符:\n{c_code[:200]}"
            
            # 运行
            run_result = subprocess.run(
                [out_file],
                capture_output=True, text=True, timeout=5
            )
            
            # 清理临时文件
            for f in [c_file, out_file]:
                if os.path.exists(f):
                    os.remove(f)
            
            return True, run_result.stdout
            
        except FileNotFoundError:
            return False, "C编译器未安装"
        except subprocess.TimeoutExpired:
            return False, "编译/运行超时"


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    compiler = GlyphCCompiler()
    
    if len(sys.argv) > 1:
        program = sys.argv[1]
    else:
        program = "ba ma pa fa"
    
    if "--run" in sys.argv:
        ok, output = compiler.compile_and_run(program)
        if ok:
            print(output)
        else:
            print(f"错误: {output}")
    elif "--emit" in sys.argv:
        idx = sys.argv.index("--emit")
        out = sys.argv[idx + 1] if idx + 1 < len(sys.argv) else "output.c"
        c_code = compiler.compile(program, out)
        print(f"C代码已写入: {out} ({len(c_code)} 字符)")
    else:
        c_code = compiler.compile(program)
        print(c_code)
