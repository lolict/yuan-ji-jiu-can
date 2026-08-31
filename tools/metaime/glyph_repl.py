#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
字形几何编程语言 · 交互式解释器 (REPL)

输入: 音节程序 (如 "ba ma pa fa")
输出: 力场演化过程 + 几何关系可视化

命令:
  /parse <程序>     - 仅解析，显示AST
  /run <程序>       - 执行，显示演化过程
  /step <程序>      - 单步执行
  /show <字母>      - 显示字母的几何信息
  /compat <a> <b>   - 检查两个字母的兼容性
  /force <程序>     - 显示力场计算
  /viz <程序>       - ASCII可视化
  /help             - 帮助
  /quit             - 退出
"""

import os
import sys
import math
import time
from typing import List, Dict, Optional

# 导入引擎
from glyph_engine import (
    GlyphParser, GlyphExecutor, GLYPH_REGISTRY, 
    YUNMU_VECTORS, TONE_ENERGY, ForceElement
)


# ============================================================
# 可视化工具
# ============================================================

class ASCIIVisualizer:
    """ASCII 几何可视化"""
    
    # 字母的ASCII表示
    GLYPH_ART = {
        'b': ["│", "│", "╭─", "╰─"],
        'p': ["╭─", "╰─", "│", "│"],
        'm': ["╭─╮", "│ │", "╰─┴─╮", "    ╰─"],
        'f': ["───", " │ ", "───", " │ "],
        'd': ["╮", "│", "╯", "│"],
        't': ["───", " │ ", " │ ", " ╰─"],
        'n': ["╭─╮", "│ │", "╰─┴─", "    "],
        'l': ["│", "│", "│", "│"],
        'g': ["╭─╮", "│ │", "╰─╯", " ╰─"],
        'k': ["│╲", "│ ╲", "│ ╱", "│╱"],
        'h': ["│ ╭─", "│ │ ", "│ │ ", "╰─╯─"],
        'j': ["  .", "  │", "  │", "╰─╯"],
        'q': ["╭─╮", "│ │", "╰─╯", "  ╰→"],
        'x': ["╲ ╱", " ╳ ", "╱ ╲", "   "],
        'zh': ["───", "  │", " ╱ ", "╱  "],
        'ch': ["╭──", "│  ", "│  ", "╰──"],
        'sh': ["╭─╮", "│  ", "╰─╮", "  ╰─"],
        'r': ["│", "│", "├─", "│"],
        'z': ["───", "  ╱", " ╱ ", "───"],
        'c': ["╭─", "│ ", "│ ", "╰─"],
        's': ["╭─", "│ ", "╰─", " ╰"],
        'y': ["╲ ╱", " ╳ ", " │ ", " │ "],
        'w': ["╲   ╱", " ╲ ╱ ", "  ╳  ", " ╰ ╯ "],
        'a': ["╭─╮", "│ │", "├─┤", "╰─╯"],
        'o': ["╭─╮", "│ │", "│ │", "╰─╯"],
        'e': ["╭─╮", "│  ", "├─ ", "╰─╯"],
        'i': [" │ ", " │ ", " │ ", " │ "],
        'u': ["│ │", "│ │", "│ │", "╰─╯"],
    }
    
    @staticmethod
    def draw_syllable(syllable: str, force: float = 0, energy: float = 1.0) -> str:
        """绘制单个音节的ASCII图"""
        sm = syllable[0] if syllable else '?'
        # 找到声母
        for i in range(2, 0, -1):
            if syllable[:i] in GLYPH_REGISTRY:
                sm = syllable[:i]
                break
        
        art = ASCIIVisualizer.GLYPH_ART.get(sm, [" ??? "])
        
        # 根据力值调整显示
        intensity = min(int(force / 5), 5)
        indicators = [" ", "·", "░", "▒", "▓", "█"]
        indicator = indicators[intensity]
        
        lines = []
        lines.append(f"  ┌{'─' * 8}┐")
        for row in art[:4]:
            lines.append(f"  │{indicator}{row:^6}{indicator}│")
        lines.append(f"  └{'─' * 8}┘")
        lines.append(f"   {syllable:^6} F={force:.1f}")
        
        return "\n".join(lines)
    
    @staticmethod
    def draw_field_state(state: List[Dict], title: str = "力场状态") -> str:
        """绘制整个力场状态"""
        lines = []
        lines.append(f"  ═══ {title} ═══")
        lines.append("")
        
        # 绘制每个音节
        for i, s in enumerate(state):
            syllable = s.get('syllable', '?')
            force = s.get('force', 0)
            energy = s.get('energy', 1.0)
            vector = s.get('vector', 'unknown')
            
            # 简化的ASCII表示
            sm = ''
            for j in range(2, 0, -1):
                if syllable[:j] in GLYPH_REGISTRY:
                    sm = syllable[:j]
                    break
            
            art = ASCIIVisualizer.GLYPH_ART.get(sm, ["?"])
            
            # 力值条
            bar_len = min(int(force / 2), 20)
            bar = "█" * bar_len + "░" * (20 - bar_len)
            
            lines.append(f"  [{i}] {syllable:4} │{bar}│ F={force:6.2f} E={energy:.1f}")
            
            # 显示几何基元
            glyph = GLYPH_REGISTRY.get(sm)
            if glyph:
                prims = ", ".join([f"{p.geo_type[0]}({p.zone[0]})" for p in glyph.primitives[:3]])
                lines.append(f"       操作: {glyph.operation[:30]}")
                lines.append(f"       基元: {prims}")
            
            lines.append("")
        
        return "\n".join(lines)
    
    @staticmethod
    def draw_evolution(history: List[List[Dict]]) -> str:
        """绘制演化历史"""
        lines = []
        lines.append("  ═══ 力场演化过程 ═══")
        lines.append("")
        
        for step, state in enumerate(history):
            lines.append(f"  ── 第 {step} 步 ──")
            total_force = sum(s.get('force', 0) for s in state)
            lines.append(f"  总力: {total_force:.2f}")
            
            for s in state:
                syllable = s.get('syllable', '?')
                force = s.get('force', 0)
                bar_len = min(int(force / 2), 15)
                bar = "█" * bar_len + "░" * (15 - bar_len)
                lines.append(f"    {syllable:4} │{bar}│ {force:.2f}")
            
            lines.append("")
        
        return "\n".join(lines)
    
    @staticmethod
    def draw_compatibility(a: str, b: str) -> str:
        """绘制两个字母的兼容性"""
        lines = []
        lines.append(f"  ═══ 兼容性分析: {a} ↔ {b} ═══")
        lines.append("")
        
        ga = GLYPH_REGISTRY.get(a)
        gb = GLYPH_REGISTRY.get(b)
        
        if not ga or not gb:
            lines.append("  未知字母")
            return "\n".join(lines)
        
        # 显示两个字母
        lines.append(f"  {a}: {ga.operation}")
        lines.append(f"     力={ga.force_strength}, 利用区={ga.zone_used}, 类型={ga.geo_type}")
        lines.append(f"  {b}: {gb.operation}")
        lines.append(f"     力={gb.force_strength}, 利用区={gb.zone_used}, 类型={gb.geo_type}")
        lines.append("")
        
        # 兼容性判断
        compatible = ga.zone_used == gb.zone_used
        type_compat = ga.geo_type == gb.geo_type
        
        lines.append(f"  利用区兼容: {'✅' if compatible else '❌'} ({ga.zone_used} vs {gb.zone_used})")
        lines.append(f"  类型兼容:   {'✅' if type_compat else '❌'} ({ga.geo_type} vs {gb.geo_type})")
        
        # 相互作用力
        interaction = ga.force_strength + gb.force_strength
        lines.append(f"  相互作用力: {interaction} (加法统摄)")
        
        # 几何关系
        if compatible:
            lines.append(f"  关系: 可组合（{ga.zone_used}区相互作用）")
        else:
            lines.append(f"  关系: 需转换（{ga.zone_used} → {gb.zone_used}）")
        
        return "\n".join(lines)


# ============================================================
# REPL
# ============================================================

class GlyphREPL:
    """字形几何交互式解释器"""
    
    def __init__(self):
        self.parser = GlyphParser()
        self.executor = GlyphExecutor()
        self.viz = ASCIIVisualizer()
        self.history = []
        self.running = True
    
    def run(self):
        """启动REPL"""
        self.print_banner()
        
        while self.running:
            try:
                line = input("\n_glyph> ").strip()
            except (EOFError, KeyboardInterrupt):
                print("\n再见")
                break
            
            if not line:
                continue
            
            self.handle_command(line)
    
    def print_banner(self):
        """打印欢迎信息"""
        print("""
╔══════════════════════════════════════════════════════════╗
║           字形几何编程语言 · 交互式解释器                  ║
║                                                          ║
║  输入音节程序，观察力场演化                               ║
║  命令: /parse /run /step /show /compat /force /viz /help ║
╚══════════════════════════════════════════════════════════╝
""")
    
    def handle_command(self, line: str):
        """处理命令"""
        if line.startswith('/'):
            parts = line.split(None, 1)
            cmd = parts[0]
            arg = parts[1] if len(parts) > 1 else ''
            
            if cmd == '/quit' or cmd == '/q':
                self.running = False
                print("再见")
            elif cmd == '/help' or cmd == '/h':
                self.print_help()
            elif cmd == '/parse':
                self.cmd_parse(arg)
            elif cmd == '/run':
                self.cmd_run(arg)
            elif cmd == '/step':
                self.cmd_step(arg)
            elif cmd == '/show':
                self.cmd_show(arg)
            elif cmd == '/compat':
                self.cmd_compat(arg)
            elif cmd == '/force':
                self.cmd_force(arg)
            elif cmd == '/viz':
                self.cmd_viz(arg)
            else:
                print(f"未知命令: {cmd}")
        else:
            # 默认执行
            self.cmd_run(line)
    
    def print_help(self):
        """打印帮助"""
        print("""
命令:
  <程序>              直接执行程序 (如: ba ma pa)
  /parse <程序>       仅解析，显示AST
  /run <程序>         执行程序，显示演化过程
  /step <程序>        单步执行
  /show <字母>        显示字母的几何信息
  /compat <a> <b>     检查两个字母的兼容性
  /force <程序>       显示力场计算详情
  /viz <程序>         ASCII可视化
  /help               显示帮助
  /quit               退出
""")
    
    def cmd_parse(self, program: str):
        """解析程序"""
        tokens = self.parser.parse(program)
        if not tokens:
            print("  无有效音节")
            return
        
        print(f"  解析结果: {len(tokens)} 个音节")
        for i, t in enumerate(tokens):
            sm = t['shengmu']
            ym = t['yunmu']
            tone = t['tone']
            glyph = GLYPH_REGISTRY.get(sm)
            op = glyph.operation if glyph else "未知"
            print(f"    [{i}] {sm}{ym} (声调{tone}) → {op}")
    
    def cmd_run(self, program: str):
        """执行程序"""
        result = self.executor.execute(program)
        
        if not result['ok']:
            print(f"  错误: {result['error']}")
            return
        
        # 显示演化历史
        if hasattr(self.executor, '_evolution_history'):
            history = self.executor._evolution_history
            print(self.viz.draw_evolution(history))
        
        # 显示最终状态
        print(self.viz.draw_field_state(result['final_state'], "最终状态"))
        print(f"  {result['summary']}")
    
    def cmd_step(self, program: str):
        """单步执行"""
        tokens = self.parser.parse(program)
        if not tokens:
            print("  无有效音节")
            return
        
        ast = self.parser.build_ast()
        
        print(f"  初始状态 ({len(ast)} 个音节):")
        for node in ast:
            sm = node['shengmu']
            ym = node['yunmu']
            force = node.get('force', 0)
            print(f"    {sm}{ym}: 力={force:.2f}")
        
        # 逐步演化
        for step in range(10):
            changes = self.executor._evolve_step()
            self.executor.state.iterations = step + 1
            
            print(f"\n  ── 第 {step+1} 步 ──")
            for node in self.executor.parser.ast:
                sm = node['shengmu']
                ym = node['yunmu']
                force = node.get('force', 0)
                bar_len = min(int(force / 2), 15)
                bar = "█" * bar_len + "░" * (15 - bar_len)
                print(f"    {sm}{ym:3} │{bar}│ {force:.2f}")
            
            if not changes:
                print("  → 达到平衡态")
                break
            
            input("  按回车继续...")
    
    def cmd_show(self, letter: str):
        """显示字母信息"""
        glyph = GLYPH_REGISTRY.get(letter)
        if not glyph:
            print(f"  未知字母: {letter}")
            return
        
        print(f"  ═══ 字母: {letter} ═══")
        print(f"  操作: {glyph.operation}")
        print(f"  力场强度: {glyph.force_strength}")
        print(f"  利用区: {glyph.zone_used}")
        print(f"  几何类型: {glyph.geo_type}")
        print(f"  几何基元:")
        for p in glyph.primitives:
            print(f"    - {p.geo_type} ({p.zone}): {p.desc} [力={p.force_value}]")
        
        # ASCII可视化
        print(f"\n  ASCII表示:")
        art = ASCIIVisualizer.GLYPH_ART.get(letter, ["?"])
        for row in art:
            print(f"    {row}")
    
    def cmd_compat(self, arg: str):
        """检查兼容性"""
        parts = arg.split()
        if len(parts) < 2:
            print("  用法: /compat <字母1> <字母2>")
            return
        
        a, b = parts[0], parts[1]
        print(self.viz.draw_compatibility(a, b))
    
    def cmd_force(self, program: str):
        """显示力场计算"""
        tokens = self.parser.parse(program)
        if not tokens:
            print("  无有效音节")
            return
        
        print(f"  ═══ 力场计算: {program} ═══")
        print()
        
        total_force = 0
        for i, t in enumerate(tokens):
            sm = t['shengmu']
            ym = t['yunmu']
            tone = t['tone']
            
            glyph = GLYPH_REGISTRY.get(sm)
            if not glyph:
                continue
            
            base_force = glyph.force_strength
            energy = TONE_ENERGY.get(tone, 1.0)
            final_force = base_force * energy
            
            print(f"  [{i}] {sm}{ym}:")
            print(f"      声母力: {base_force} (", end="")
            prim_forces = [f"{p.geo_type[0]}={p.force_value}" for p in glyph.primitives]
            print(" + ".join(prim_forces), end="")
            print(f")")
            print(f"      声调能量: {energy}")
            print(f"      最终力: {base_force} × {energy} = {final_force:.2f}")
            
            total_force += final_force
            print()
        
        print(f"  总力（加法统摄）: {total_force:.2f}")
        
        # 显示韵母向量
        print(f"\n  韵母向量:")
        for t in tokens:
            ym = t['yunmu']
            vec = YUNMU_VECTORS.get(ym, YUNMU_VECTORS['a'])
            print(f"    {ym}: {vec['direction']} ({vec['force']})")
    
    def cmd_viz(self, program: str):
        """ASCII可视化"""
        tokens = self.parser.parse(program)
        if not tokens:
            print("  无有效音节")
            return
        
        ast = self.parser.build_ast()
        
        print(f"  ═══ 几何可视化: {program} ═══")
        print()
        
        # 绘制每个音节
        for i, node in enumerate(ast):
            syllable = f"{node['shengmu']}{node['yunmu']}"
            force = node.get('force', 0)
            print(self.viz.draw_syllable(syllable, force, node.get('energy', 1.0)))
            
            # 显示链接
            if 'next' in node:
                print(f"      │")
                print(f"      ↓ 力传递")
                print(f"      │")
        
        # 力场总图
        print(f"\n  力场总图:")
        total_force = sum(n.get('force', 0) for n in ast)
        for node in ast:
            syllable = f"{node['shengmu']}{node['yunmu']}"
            force = node.get('force', 0)
            pct = (force / total_force * 100) if total_force > 0 else 0
            bar_len = int(pct / 2)
            bar = "█" * bar_len + "░" * (50 - bar_len)
            print(f"  {syllable:4} │{bar}│ {pct:.1f}%")


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    repl = GlyphREPL()
    repl.run()
