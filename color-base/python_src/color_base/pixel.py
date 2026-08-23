"""
相位像素 · 颜色进制底层对象

每个像素不是 RGB，而是：
  - phase: 相位编码值（进制编码，不是颜色）
  - weight: 视觉权重（笔画密度/胖瘦）
  - brightness: 亮度
  - metadata: 元数据纸条（来源母体、层级、归属权、黑名单标记）
"""

from __future__ import annotations
import struct
from typing import Optional, Dict, Any, Tuple
from dataclasses import dataclass, field


@dataclass
class PhasePixel:
    """相位像素 — 颜色进制的最小单元"""
    phase: int = 0           # 相位编码值（64进制/256进制等）
    weight: float = 0.5      # 视觉权重 0-1（笔画密度/胖瘦）
    brightness: float = 1.0  # 亮度 0-1
    owner: str = "SELF"      # 归属权: SELF(己方) / EXTERNAL(外来)
    level: int = 1           # 层级: L1=基元本体 / L2=修饰部件 / L3=装配成品
    family: str = "default"  # 字形家族（用于审美配对）
    parent_source: list = field(default_factory=list)  # 母体部件来源（记录加减运算）
    blacklisted: bool = False  # 黑名单标记（犯罪笔录）
    
    # 区块链纠缠索引
    prefix_ids: list = field(default_factory=list)  # 可前置连接的节点 IDs
    suffix_ids: list = field(default_factory=list)  # 可后置连接的节点 IDs
    
    # 可选的 RGB 翻译层（输出时才用，不是底层本体）
    r: int = 0
    g: int = 0
    b: int = 0

    def to_bytes(self) -> bytes:
        """序列化为二进制（CBF 格式）"""
        # 核心 12 字节 + 变长元数据
        header = struct.pack(
            '>H B f f B B',
            self.phase & 0xFFFF,   # 2 bytes: phase (16-bit for compat)
            int(self.weight * 255), # 1 byte: weight
            self.brightness,        # 4 bytes: float
            self.level,             # 1 byte
            1 if self.owner == "SELF" else 0,  # 1 byte: owner flag
            1 if self.blacklisted else 0,      # 1 byte: blacklist
        )
        return header

    @classmethod
    def from_bytes(cls, data: bytes) -> 'PhasePixel':
        """从二进制反序列化"""
        phase, w, bright, lvl, owner_flag, bl = struct.unpack('>H B f f B B', data[:12])
        return cls(
            phase=phase,
            weight=w / 255.0,
            brightness=bright,
            level=lvl,
            owner="SELF" if owner_flag else "EXTERNAL",
            blacklisted=bool(bl),
        )

    def to_rgb(self) -> Tuple[int, int, int]:
        """相位 → RGB 翻译（输出的最后一步）"""
        if self.r == 0 and self.g == 0 and self.b == 0:
            # 自动映射：相位 → 色相，权重 → 饱和度，亮度 → 明度
            h = (self.phase % 360) / 360.0
            s = self.weight
            v = self.brightness
            return self._hsv_to_rgb(h, s, v)
        return (self.r, self.g, self.b)

    @staticmethod
    def _hsv_to_rgb(h: float, s: float, v: float) -> Tuple[int, int, int]:
        """HSV → RGB"""
        if s == 0:
            val = int(v * 255)
            return (val, val, val)
        i = int(h * 6)
        f = h * 6 - i
        p = v * (1 - s)
        q = v * (1 - f * s)
        t = v * (1 - (1 - f) * s)
        rgb_table = [
            (v, t, p), (q, v, p), (p, v, t),
            (p, q, v), (t, p, v), (v, p, q),
        ]
        r, g, b = rgb_table[i % 6]
        return (int(r * 255), int(g * 255), int(b * 255))

    def compatible_with(self, other: 'PhasePixel') -> Tuple[bool, str]:
        """等效镜像距离检查 — 两个节点能否配对"""
        # 层级等效
        if self.level != other.level:
            return False, f"层级不匹配: L{self.level} vs L{other.level}"
        # 字形家族相容性
        if self.family != other.family:
            return False, f"家族不同: {self.family} vs {other.family}"
        # 视觉权重等效（胖瘦差距不能过大）
        wdiff = abs(self.weight - other.weight)
        if wdiff > 0.35:
            return False, f"视觉重量差过大: {wdiff:.2f}"
        # 黑名单检查
        if self.blacklisted or other.blacklisted:
            return False, "黑名单标记"
        return True, "等效对齐"

    def __repr__(self):
        return (f"Pxl(φ={self.phase},w={self.weight:.2f},"
                f"L{self.level},{self.family},owner={self.owner})")


class PhaseCanvas:
    """相位画布 — 二维 PhasePixel 网格"""
    
    def __init__(self, width: int, height: int, name: str = "canvas"):
        self.width = width
        self.height = height
        self.name = name
        self.grid: list[list[PhasePixel]] = [
            [PhasePixel() for _ in range(width)] for _ in range(height)
        ]

    def get(self, x: int, y: int) -> PhasePixel:
        if 0 <= x < self.width and 0 <= y < self.height:
            return self.grid[y][x]
        return PhasePixel()  # 越界返回空白

    def set(self, x: int, y: int, pixel: PhasePixel):
        if 0 <= x < self.width and 0 <= y < self.height:
            self.grid[y][x] = pixel

    def to_rgb_matrix(self) -> list[list[Tuple[int, int, int]]]:
        """翻译为 RGB 矩阵（最后输出用）"""
        return [[self.grid[y][x].to_rgb() for x in range(self.width)]
                for y in range(self.height)]

    def __repr__(self):
        return f"Canvas({self.width}x{self.height},'{self.name}')"
