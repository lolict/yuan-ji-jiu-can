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
    """生成操作函数 — 实现真正的容器语义"""
    return r"""
/* 操作函数 — 容器执行语义 */

/* 全局容器栈（模拟内存） */
static float container_stack[64];
static int stack_top = 0;

void stack_push(float v) { if (stack_top < 64) container_stack[stack_top++] = v; }
float stack_pop(void) { return stack_top > 0 ? container_stack[--stack_top] : 0.0f; }

/* 装入：将力值压入容器 */
void glyph_install(GlyphSyllable* s) {
    stack_push(s->force);
    printf("  [装入] %s%s: 力%.2f → 容器[深度=%d]\n",
           s->shengmu, s->yunmu, s->force, stack_top);
}

/* 取出：从容器取出力值 */
void glyph_extract(GlyphSyllable* s) {
    float v = stack_pop();
    printf("  [取出] %s%s: 容器[深度=%d] → 力%.2f\n",
           s->shengmu, s->yunmu, stack_top, v);
}

/* 遍历：复制栈顶，模拟遍历所有容器 */
void glyph_traverse(GlyphSyllable* s) {
    int n = stack_top;
    printf("  [遍历] %s%s: 遍历%d个容器 [", s->shengmu, s->yunmu, n);
    for (int i = 0; i < n; i++) {
        if (i > 0) printf(", ");
        printf("%.2f", container_stack[i]);
    }
    printf("]\n");
}

/* 摩擦：相邻容器力值相减取绝对值 */
void glyph_friction(GlyphSyllable* s) {
    if (stack_top >= 2) {
        float a = container_stack[stack_top-1];
        float b = container_stack[stack_top-2];
        float friction = (a > b) ? (a - b) : (b - a);
        printf("  [摩擦] %s%s: |%.2f - %.2f| = %.2f\n",
               s->shengmu, s->yunmu, a, b, friction);
    } else {
        printf("  [摩擦] %s%s: 容器不足，无法摩擦\n", s->shengmu, s->yunmu);
    }
}

/* 定位：根据力值查找容器位置 */
void glyph_locate(GlyphSyllable* s) {
    int pos = -1;
    for (int i = stack_top - 1; i >= 0; i--) {
        if ((int)container_stack[i] == (int)s->force) { pos = i; break; }
    }
    if (pos >= 0)
        printf("  [定位] %s%s: 力%.2f 在位置[%d]\n", s->shengmu, s->yunmu, s->force, pos);
    else
        printf("  [定位] %s%s: 力%.2f 不在容器中\n", s->shengmu, s->yunmu, s->force);
}

/* 锚定：将栈顶与当前力值绑定（取最大值） */
void glyph_anchor(GlyphSyllable* s) {
    if (stack_top > 0) {
        float top = container_stack[stack_top-1];
        container_stack[stack_top-1] = (top > s->force) ? top : s->force;
        printf("  [锚定] %s%s: 栈顶锁定为%.2f\n", s->shengmu, s->yunmu, container_stack[stack_top-1]);
    } else {
        stack_push(s->force);
        printf("  [锚定] %s%s: 新锚点%.2f\n", s->shengmu, s->yunmu, s->force);
    }
}

/* 比较：比较栈顶与当前力值 */
void glyph_compare(GlyphSyllable* s) {
    if (stack_top > 0) {
        float top = container_stack[stack_top-1];
        const char* rel = (top > s->force) ? ">" : (top < s->force) ? "<" : "=";
        printf("  [比较] %s%s: %.2f %s %.2f\n", s->shengmu, s->yunmu, top, rel, s->force);
    } else {
        printf("  [比较] %s%s: 栈为空\n", s->shengmu, s->yunmu);
    }
}

/* 连接：将当前力值加到栈顶 */
void glyph_connect(GlyphSyllable* s) {
    if (stack_top > 0) {
        container_stack[stack_top-1] += s->force;
        printf("  [连接] %s%s: 栈顶 += %.2f → %.2f\n",
               s->shengmu, s->yunmu, s->force, container_stack[stack_top-1]);
    } else {
        stack_push(s->force);
        printf("  [连接] %s%s: 栈顶 = %.2f\n", s->shengmu, s->yunmu, s->force);
    }
}

/* 螺旋：取栈顶与栈底的平均值，存到中间 */
void glyph_spiral(GlyphSyllable* s) {
    if (stack_top >= 3) {
        float avg = (container_stack[0] + container_stack[stack_top-1]) / 2.0f;
        int mid = stack_top / 2;
        container_stack[mid] = avg;
        printf("  [螺旋] %s%s: (底%.2f + 顶%.2f)/2 = %.2f → 位置[%d]\n",
               s->shengmu, s->yunmu, container_stack[0], container_stack[stack_top-1], avg, mid);
    } else {
        printf("  [螺旋] %s%s: 容器不足3个，无法螺旋\n", s->shengmu, s->yunmu);
    }
}

/* 支撑：将栈顶力值分散到所有容器（加法分解） */
void glyph_support(GlyphSyllable* s) {
    if (stack_top > 1) {
        float top = stack_pop();
        float each = top / stack_top;
        for (int i = 0; i < stack_top; i++) {
            container_stack[i] += each;
        }
        printf("  [支撑] %s%s: %.2f 均分%.2f到%d个容器\n",
               s->shengmu, s->yunmu, top, each, stack_top);
    } else {
        printf("  [支撑] %s%s: 不足以分散\n", s->shengmu, s->yunmu);
    }
}

/* 承载：累加当前力值到所有容器 */
void glyph_bear(GlyphSyllable* s) {
    for (int i = stack_top - 1; i >= 0; i--) {
        container_stack[i] += s->force * 0.1f;
    }
    printf("  [承载] %s%s: 全部+%.2f×0.1, 共%d个容器\n",
           s->shengmu, s->yunmu, s->force, stack_top);
}

/* 捕获：找到最接近当前力值的容器并标记 */
void glyph_capture(GlyphSyllable* s) {
    if (stack_top == 0) { printf("  [捕获] %s%s: 空容器\n", s->shengmu, s->yunmu); return; }
    int best = 0;
    float best_diff = (container_stack[0] > s->force) ? 
                      (container_stack[0] - s->force) : (s->force - container_stack[0]);
    for (int i = 1; i < stack_top; i++) {
        float diff = (container_stack[i] > s->force) ? 
                     (container_stack[i] - s->force) : (s->force - container_stack[i]);
        if (diff < best_diff) { best_diff = diff; best = i; }
    }
    printf("  [捕获] %s%s: 最接近%.2f的是位置[%.2f], 差%.2f\n",
           s->shengmu, s->yunmu, s->force, container_stack[best], best_diff);
}

/* 追踪：从栈顶往下查找力值相同的容器 */
void glyph_trace(GlyphSyllable* s) {
    int found = -1;
    for (int i = stack_top - 1; i >= 0; i--) {
        if ((int)container_stack[i] == (int)s->force) { found = i; break; }
    }
    if (found >= 0)
        printf("  [追踪] %s%s: 从顶往下追踪，在位置[%d]找到力%.2f\n",
               s->shengmu, s->yunmu, found, s->force);
    else
        printf("  [追踪] %s%s: 未找到力%.2f\n", s->shengmu, s->yunmu, s->force);
}

/* 交叉：两个栈顶容器交换位置 */
void glyph_intersect(GlyphSyllable* s) {
    if (stack_top >= 2) {
        float tmp = container_stack[stack_top-1];
        container_stack[stack_top-1] = container_stack[stack_top-2];
        container_stack[stack_top-2] = tmp;
        printf("  [交叉] %s%s: 交换位置[%d]↔[%d]\n",
               s->shengmu, s->yunmu, stack_top-1, stack_top-2);
    } else {
        printf("  [交叉] %s%s: 不足2个容器\n", s->shengmu, s->yunmu);
    }
}

/* 折叠：将容器从中间对折相加 */
void glyph_fold(GlyphSyllable* s) {
    int n = stack_top;
    int half = n / 2;
    for (int i = 0; i < half; i++) {
        container_stack[i] += container_stack[n-1-i];
    }
    stack_top = half + (n % 2);
    printf("  [折叠] %s%s: 折叠后%d个容器\n", s->shengmu, s->yunmu, stack_top);
}

/* 包裹：将当前力值包裹容器（栈顶乘以力值系数） */
void glyph_wrap(GlyphSyllable* s) {
    if (stack_top > 0) {
        container_stack[stack_top-1] += s->force * 0.5f;
        printf("  [包裹] %s%s: 栈顶+%.2f×0.5 = %.2f\n",
               s->shengmu, s->yunmu, s->force, container_stack[stack_top-1]);
    }
}

/* 流动：将栈顶力值传递给下一个（右移） */
void glyph_flow(GlyphSyllable* s) {
    if (stack_top >= 2) {
        float top = stack_pop();
        container_stack[stack_top-1] += top;
        printf("  [流动] %s%s: %.2f → 邻居, 邻居=%.2f\n",
               s->shengmu, s->yunmu, top, container_stack[stack_top-1]);
    }
}

/* 分支：如果栈顶>力值，复制栈顶分二 */
void glyph_branch(GlyphSyllable* s) {
    if (stack_top == 0) { stack_push(s->force); return; }
    float top = container_stack[stack_top-1];
    if (top > s->force && stack_top < 64) {
        container_stack[stack_top] = top * 0.5f;
        stack_top++;
        printf("  [分支] %s%s: %.2f > %.2f, 分裂出%.2f\n",
               s->shengmu, s->yunmu, top, s->force, top * 0.5f);
    } else {
        printf("  [分支] %s%s: %.2f ≤ %.2f, 不分裂\n",
               s->shengmu, s->yunmu, top, s->force);
    }
}

/* 振荡：交替加减力值 */
void glyph_oscillate(GlyphSyllable* s) {
    if (stack_top >= 2) {
        container_stack[stack_top-1] -= s->force;
        container_stack[stack_top-2] += s->force;
        printf("  [振荡] %s%s: 顶层-%.2f, 次层+%.2f\n",
               s->shengmu, s->yunmu, s->force, s->force);
    }
}

/* 包围：将容器总和追加为新容器 */
void glyph_enclose(GlyphSyllable* s) {
    float sum = s->force;
    for (int i = 0; i < stack_top; i++) sum += container_stack[i];
    stack_push(sum);
    printf("  [包围] %s%s: 包围总和=%.2f, 新深度=%d\n",
           s->shengmu, s->yunmu, sum, stack_top);
}

/* 传递：复制栈顶 */
void glyph_transfer(GlyphSyllable* s) {
    if (stack_top > 0 && stack_top < 64) {
        container_stack[stack_top] = container_stack[stack_top-1] + s->force * 0.1f;
        stack_top++;
        printf("  [传递] %s%s: 复制栈顶+%.2f×0.1\n",
               s->shengmu, s->yunmu, s->force);
    }
}

/* 分裂：栈顶一分为二 */
void glyph_split(GlyphSyllable* s) {
    if (stack_top > 0 && stack_top < 64) {
        float top = stack_pop();
        stack_push(top / 2.0f);
        stack_push(top / 2.0f);
        printf("  [分裂] %s%s: %.2f → [%.2f, %.2f]\n",
               s->shengmu, s->yunmu, top, top/2.0f, top/2.0f);
    }
}

/* 合并：合并栈顶两个容器 */
void glyph_merge(GlyphSyllable* s) {
    if (stack_top >= 2) {
        float a = stack_pop();
        float b = stack_pop();
        stack_push(a + b);
        printf("  [合并] %s%s: %.2f + %.2f = %.2f\n",
               s->shengmu, s->yunmu, a, b, a+b);
    }
}

"""


def escape_c_string(s: str) -> str:
    """转义C字符串"""
    # 过滤注释行，只取第一行非注释内容作为显示
    lines = s.strip().split('\n')
    display_lines = [l for l in lines if not l.strip().startswith('#')]
    display = ' '.join(display_lines).strip()
    # 转义C特殊字符
    escaped = display.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')
    return escaped


def generate_c_main(program: str, tokens: list) -> str:
    """生成main函数"""
    lines = []
    prog_display = escape_c_string(program)
    
    lines.append('int main(int argc, char* argv[]) {')
    lines.append(f'    printf("=== 字形几何程序 ===\\n");')
    lines.append(f'    printf("程序: {prog_display}\\n\\n");')
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
    lines.append('    /* 力场演化 — 加法统摄，收敛到均衡 */')
    lines.append('    printf("\\n力场演化:\\n");')
    lines.append('    for (int iter = 0; iter < 10; iter++) {')
    lines.append('        float total_force = 0;')
    lines.append('        float new_forces[%d];' % len(tokens))
    lines.append('        for (int i = 0; i < %d; i++) {' % len(tokens))
    lines.append('            /* 与邻居的平均化 (加法统摄=无乘法膨胀) */')
    lines.append('            float sum = syllables[i].force;')
    lines.append('            int count = 1;')
    lines.append('            if (i > 0) { sum += syllables[i-1].force; count++; }')
    lines.append('            if (i < %d-1) { sum += syllables[i+1].force; count++; }' % len(tokens))
    lines.append('            new_forces[i] = sum / count;')
    lines.append('        }')
    lines.append('        for (int i = 0; i < %d; i++) {' % len(tokens))
    lines.append('            syllables[i].force = new_forces[i];')
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

def main():
    compiler = GlyphCCompiler()
    
    if len(sys.argv) < 2:
        print("用法:")
        print("  python3 glyph_cc.py <程序>           # 输出C代码")
        print("  python3 glyph_cc.py <程序> --run     # 编译并执行")
        print("  python3 glyph_cc.py <程序> --emit out.c  # 输出到文件")
        print("  python3 glyph_cc.py --file prog.glyph    # 从文件加载")
        print("")
        print("示例:")
        print('  python3 glyph_cc.py "ba ma pa" --run')
        print("  python3 glyph_cc.py --file examples/xi_gua.glyph --run")
        return
    
    # 从文件加载
    if sys.argv[1] == "--file":
        if len(sys.argv) < 3:
            print("错误: --file 需要文件路径")
            return
        file_path = sys.argv[2]
        if not os.path.exists(file_path):
            print(f"错误: 文件不存在: {file_path}")
            return
        with open(file_path, 'r', encoding='utf-8') as f:
            source = f.read()
        # 过滤注释行和空行
        lines = [l.strip() for l in source.split('\n') if l.strip() and not l.strip().startswith('#')]
        program = ' '.join(lines)
        print(f"从文件加载: {file_path}")
        print(f"程序内容: {program}")
        print("")
    else:
        program = sys.argv[1]
    
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


if __name__ == "__main__":
    main()
