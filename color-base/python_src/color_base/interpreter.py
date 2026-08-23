"""
颜色进制解释器 · 交互式 REPL

支持命令:
  > canvas 16x16 "my_canvas"     — 创建画布
  > phase 128 weight 0.8         — 填充相位
  > pixel 3 4 phase 64 weight 0.5 — 设置单个像素
  > rect 0 0 8 8 family ipa      — 矩形区域
  > add "file.cb" at 4 4        — 加法（安装部件）
  > sub "file.cb" at 4 4        — 减法（剥离部件）
  > judge p1 p2                  — 审判配对
  > show                         — 显示当前画布
  > rgb                          — 翻译为 RGB
  > status                       — 系统状态
  > help                         — 帮助
  > quit                         — 退出
"""

from __future__ import annotations
import os
import sys
from typing import Optional
from .pixel import PhasePixel, PhaseCanvas
from .ops import phase_add, phase_sub, canvas_add, canvas_sub, PhaseOpsError
from .balance import BalanceNetwork
from .compiler import ColorBaseCompiler


class ColorBaseInterpreter:
    """交互式颜色进制解释器"""
    
    def __init__(self):
        self.canvas: Optional[PhaseCanvas] = None
        self.balance = BalanceNetwork()
        self.balance.default_setup()
        self.compiler = ColorBaseCompiler()
        self.history: list[str] = []
        self.running = False
    
    def run(self):
        """启动 REPL"""
        self.running = True
        print("=== 颜色进制 · 相位编码解释器 v1.0 ===")
        print("输入 'help' 查看命令, 'quit' 退出\n")
        
        while self.running:
            try:
                line = input("φ> ").strip()
            except (EOFError, KeyboardInterrupt):
                print()
                break
            
            if not line:
                continue
            
            self.history.append(line)
            self._execute(line)
    
    def _execute(self, line: str):
        """执行单条命令"""
        parts = line.split()
        cmd = parts[0].lower()
        
        try:
            if cmd == 'quit' or cmd == 'exit':
                self.running = False
                print("再见")
            
            elif cmd == 'help':
                self._show_help()
            
            elif cmd == 'canvas':
                w, h = 16, 16
                name = "canvas"
                if len(parts) > 1:
                    dim = parts[1].split('x')
                    w, h = int(dim[0]), int(dim[1])
                if len(parts) > 2:
                    name = parts[2].strip('"')
                self.canvas = PhaseCanvas(w, h, name)
                print(f"  → 创建画布: {w}x{h} '{name}'")
            
            elif cmd == 'phase':
                if not self._check_canvas(): return
                kwargs = self._parse_kwargs(parts[1:])
                for y in range(self.canvas.height):
                    for x in range(self.canvas.width):
                        p = PhasePixel(
                            phase=int(kwargs.get('phase', 0)),
                            weight=float(kwargs.get('weight', 0.5)),
                            brightness=float(kwargs.get('brightness', 1.0)),
                            level=self._parse_level(kwargs.get('level', 'L1')),
                            family=kwargs.get('family', 'default'),
                        )
                        self.canvas.set(x, y, p)
                print(f"  → 全局填充: φ={kwargs.get('phase','0')}")
            
            elif cmd == 'pixel':
                if not self._check_canvas(): return
                x, y = int(parts[1]), int(parts[2])
                kwargs = self._parse_kwargs(parts[3:])
                p = self.canvas.get(x, y)
                p.phase = int(kwargs.get('phase', p.phase))
                p.weight = float(kwargs.get('weight', p.weight))
                p.family = kwargs.get('family', p.family)
                self.canvas.set(x, y, p)
                print(f"  → 像素({x},{y}): φ={p.phase} w={p.weight:.2f}")
            
            elif cmd == 'rect':
                if not self._check_canvas(): return
                x, y, w, h = int(parts[1]), int(parts[2]), int(parts[3]), int(parts[4])
                kwargs = self._parse_kwargs(parts[5:])
                for dy in range(h):
                    for dx in range(w):
                        self.canvas.set(x+dx, y+dy, PhasePixel(
                            phase=int(kwargs.get('phase', 0)),
                            weight=float(kwargs.get('weight', 0.5)),
                            family=kwargs.get('family', 'default'),
                            level=self._parse_level(kwargs.get('level', 'L1')),
                        ))
                print(f"  → 矩形区域: ({x},{y})+{w}x{h}")
            
            elif cmd == 'add':
                if not self._check_canvas(): return
                src = parts[1].strip('"')
                x, y = int(parts[3]), int(parts[4])
                try:
                    # 简化：创建一个默认组件
                    comp = PhaseCanvas(8, 8, src)
                    for cy in range(8):
                        for cx in range(8):
                            comp.set(cx, cy, PhasePixel(phase=64, weight=0.7, level=2))
                    self.canvas = PhaseCanvas(self.canvas.width, self.canvas.height, self.canvas.name)
                    self.canvas = canvas_add(self.canvas, comp, x, y)
                    print(f"  → 添加组件 '{src}' 在 ({x},{y})")
                except PhaseOpsError as e:
                    print(f"  ✗ {e}")
            
            elif cmd == 'sub':
                if not self._check_canvas(): return
                src = parts[1].strip('"')
                x, y = int(parts[3]), int(parts[4])
                try:
                    comp = PhaseCanvas(8, 8, src)
                    for cy in range(8):
                        for cx in range(8):
                            comp.set(cx, cy, PhasePixel(phase=64, weight=0.7, level=2))
                    self.canvas = canvas_sub(self.canvas, comp, x, y)
                    print(f"  → 剥离组件 '{src}' 在 ({x},{y})")
                except PhaseOpsError as e:
                    print(f"  ✗ {e}")
            
            elif cmd == 'judge':
                if not self._check_canvas(): return
                x1, y1, x2, y2 = int(parts[1]), int(parts[2]), int(parts[3]), int(parts[4])
                p1 = self.canvas.get(x1, y1)
                p2 = self.canvas.get(x2, y2)
                verdict = self.balance.judge(p1, p2)
                print(f"  → 审判结果: {verdict.summary}")
                print(f"     全局分: {verdict.global_score:.2f}")
                for dim, (p, conf, reason) in verdict.dimension_votes.items():
                    mark = "✓" if p else "✗"
                    print(f"     {mark} {dim}: {reason} ({conf:.1f})")
            
            elif cmd == 'show':
                if not self._check_canvas(): return
                for y in range(min(self.canvas.height, 16)):
                    row = ""
                    for x in range(min(self.canvas.width, 32)):
                        p = self.canvas.get(x, y)
                        if p.phase == 0:
                            row += "·"
                        elif p.phase < 50:
                            row += "░"
                        elif p.phase < 150:
                            row += "▒"
                        else:
                            row += "█"
                    print(f"  {row}")
            
            elif cmd == 'rgb':
                if not self._check_canvas(): return
                for y in range(min(self.canvas.height, 8)):
                    row = ""
                    for x in range(min(self.canvas.width, 16)):
                        r, g, b = self.canvas.get(x, y).to_rgb()
                        row += f"\033[48;2;{r};{g};{b}m  \033[0m"
                    print(f"  {row}")
            
            elif cmd == 'status':
                self._show_status()
            
            else:
                print(f"  ? 未知命令 '{cmd}', 输入 'help' 查看帮助")
        
        except Exception as e:
            print(f"  ✗ 错误: {e}")
    
    def _check_canvas(self) -> bool:
        if self.canvas is None:
            print("  ✗ 未创建画布, 先执行: canvas 16x16 \"name\"")
            return False
        return True
    
    def _show_help(self):
        print("""
命令列表:
  canvas WxH "name"          — 创建画布
  phase V weight W [level L] — 全局填充相位
  pixel X Y ...              — 设置单个像素
  rect X Y W H ...           — 填充矩形区域
  add "file" at X Y          — 加法: 安装部件
  sub "file" at X Y          — 减法: 剥离部件
  judge X1 Y1 X2 Y2          — 审判两个像素的配对
  show                       — 字符方式显示画布
  rgb                        — 真彩色显示（需要真彩色终端）
  status                     — 系统状态
  help                       — 此帮助
  quit                       — 退出
""")
    
    def _show_status(self):
        print(f"  画布: {self.canvas or '未创建'}")
        print(f"  制衡维度: {len(self.balance.dimensions)}")
        print(f"  历史命令: {len(self.history)}条")
    
    def _parse_kwargs(self, tokens: list) -> dict:
        kwargs = {}
        i = 0
        while i < len(tokens) - 1:
            kwargs[tokens[i]] = tokens[i+1]
            i += 2
        return kwargs
    
    def _parse_level(self, s: str) -> int:
        import re
        m = re.match(r'L?(\d)', s, re.I)
        return int(m.group(1)) if m else 1
