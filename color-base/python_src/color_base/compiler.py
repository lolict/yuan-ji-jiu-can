"""
颜色进制编译器

输入: .cb 文本格式（颜色进制脚本）
输出: .cbf 二进制格式（可执行/可加载）

CB 语法示例:
  canvas 16x16 "阿大"
  phase 128 weight 0.8 level L1
  rect 0 0 8 8 family ipa_extend
  add "声调部件.cb" at 4 4
  export "阿大.cbf"
"""

from __future__ import annotations
import struct
import json
import re
from typing import Optional
from .pixel import PhasePixel, PhaseCanvas
from .ops import canvas_add, canvas_sub, PhaseOpsError
from .balance import BalanceNetwork


class ColorBaseCompiler:
    """CB → CBF 编译器"""
    
    def __init__(self):
        self.canvas: Optional[PhaseCanvas] = None
        self.balance = BalanceNetwork()
        self.balance.default_setup()
        self.errors: list[str] = []
        self.warnings: list[str] = []
    
    def compile_source(self, source: str) -> Optional[PhaseCanvas]:
        """编译 CB 源代码为 PhaseCanvas"""
        lines = source.strip().split('\n')
        
        for lineno, line in enumerate(lines, 1):
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            try:
                self._parse_line(line)
            except Exception as e:
                self.errors.append(f"L{lineno}: {e}")
        
        return self.canvas
    
    def _parse_line(self, line: str):
        """解析单行 CB 指令"""
        parts = line.split()
        cmd = parts[0].lower()
        
        if cmd == 'canvas':
            # canvas 16x16 "name"
            dim = parts[1].split('x')
            w, h = int(dim[0]), int(dim[1])
            name = parts[2].strip('"') if len(parts) > 2 else "canvas"
            self.canvas = PhaseCanvas(w, h, name)
        
        elif cmd == 'phase':
            # phase 128 weight 0.8 level L1 family ipa_extend
            kwargs = self._parse_kwargs(parts[1:])
            pixel = PhasePixel(
                phase=int(kwargs.get('phase', 0)),
                weight=float(kwargs.get('weight', 0.5)),
                brightness=float(kwargs.get('brightness', 1.0)),
                level=self._parse_level(kwargs.get('level', 'L1')),
                family=kwargs.get('family', 'default'),
                owner=kwargs.get('owner', 'SELF'),
            )
            if self.canvas:
                for y in range(self.canvas.height):
                    for x in range(self.canvas.width):
                        self.canvas.set(x, y, PhasePixel(
                            phase=pixel.phase, weight=pixel.weight,
                            brightness=pixel.brightness, level=pixel.level,
                            family=pixel.family, owner=pixel.owner,
                        ))
        
        elif cmd == 'rect':
            # rect x y w h family ipa_extend
            x, y, w, h = int(parts[1]), int(parts[2]), int(parts[3]), int(parts[4])
            kwargs = self._parse_kwargs(parts[5:])
            family = kwargs.get('family', 'default')
            phase = int(kwargs.get('phase', 0))
            weight = float(kwargs.get('weight', 0.5))
            level = self._parse_level(kwargs.get('level', 'L1'))
            if self.canvas:
                for dy in range(h):
                    for dx in range(w):
                        self.canvas.set(x+dx, y+dy, PhasePixel(
                            phase=phase, weight=weight, family=family, level=level,
                        ))
        
        elif cmd == 'add':
            # add "file.cb" at x y
            src = parts[1].strip('"')
            x, y = int(parts[3]), int(parts[4])
            if self.canvas:
                try:
                    other = self._load_canvas(src)
                    self.canvas = canvas_add(self.canvas, other, x, y)
                except Exception as e:
                    self.errors.append(f"add failed: {e}")
        
        elif cmd == 'sub':
            # sub "file.cb" at x y
            src = parts[1].strip('"')
            x, y = int(parts[3]), int(parts[4])
            if self.canvas:
                try:
                    other = self._load_canvas(src)
                    self.canvas = canvas_sub(self.canvas, other, x, y)
                except Exception as e:
                    self.errors.append(f"sub failed: {e}")
        
        elif cmd == 'pixel':
            # pixel x y phase 128 weight 0.8
            x, y = int(parts[1]), int(parts[2])
            kwargs = self._parse_kwargs(parts[3:])
            if self.canvas:
                p = self.canvas.get(x, y)
                p.phase = int(kwargs.get('phase', p.phase))
                p.weight = float(kwargs.get('weight', p.weight))
                p.brightness = float(kwargs.get('brightness', p.brightness))
                p.level = self._parse_level(kwargs.get('level', f'L{p.level}'))
                p.family = kwargs.get('family', p.family)
                p.owner = kwargs.get('owner', p.owner)
                self.canvas.set(x, y, p)
    
    def _parse_kwargs(self, tokens: list) -> dict:
        """解析 key value key value ..."""
        kwargs = {}
        i = 0
        while i < len(tokens) - 1:
            key = tokens[i]
            val = tokens[i+1]
            kwargs[key] = val
            i += 2
        return kwargs
    
    def _parse_level(self, s: str) -> int:
        """L1 → 1, L2 → 2, L3 → 3"""
        m = re.match(r'L?(\d)', s, re.I)
        return int(m.group(1)) if m else 1
    
    def _load_canvas(self, filename: str) -> PhaseCanvas:
        """加载已有 canvas（简化：创建一个占位）"""
        # 实际项目中从文件加载 CBF
        c = PhaseCanvas(8, 8, filename)
        for y in range(8):
            for x in range(8):
                c.set(x, y, PhasePixel(phase=64, weight=0.7, level=2, family="modifier"))
        return c
    
    def compile_to_bytes(self, source: str) -> bytes:
        """编译源代码为 CBF 二进制"""
        canvas = self.compile_source(source)
        if not canvas:
            return b''
        return self._canvas_to_cbf(canvas)
    
    def _canvas_to_cbf(self, canvas: PhaseCanvas) -> bytes:
        """PhaseCanvas → CBF 二进制"""
        header = struct.pack('>4s H H 32s',
            b'CBF\x00',            # magic
            canvas.width,
            canvas.height,
            canvas.name.encode('utf-8')[:32].ljust(32, b'\0'),
        )
        pixel_data = b''
        for y in range(canvas.height):
            for x in range(canvas.width):
                pixel_data += canvas.grid[y][x].to_bytes()
        return header + pixel_data
    
    def get_report(self) -> str:
        """获取编译报告"""
        lines = []
        if self.errors:
            lines.append(f"Errors ({len(self.errors)}):")
            for e in self.errors:
                lines.append(f"  {e}")
        if self.warnings:
            lines.append(f"Warnings ({len(self.warnings)}):")
            for w in self.warnings:
                lines.append(f"  {w}")
        if not self.errors and not self.warnings:
            lines.append("编译成功, 无错误")
        return '\n'.join(lines)
