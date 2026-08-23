"""
满全法 · 相位进制画布 (Phase Radix Canvas)

底层不是 RGB 颜色，是相位编码。
每个最小单元存储相位进制值 + 元数据纸条。
运算仅允许加法（部件安装）、减法（部件拆解），禁用乘除。
文字符号只是贴在相位矩阵上的标签纸条。

使用方式：
    from phase_radix import PhaseCanvas, PhaseRadix
    
    radix = PhaseRadix()
    canvas = radix.canvas("my_symbol", width=32, height=32)
    canvas.install_component("a_glyph", x=0, y=0)
    canvas.subtract_region(x=10, y=10, w=5, h=5)
    image = canvas.compile_to_rgb()
    image.save("output.png")
"""

from .canvas import PhaseCanvas, PhaseCell, PhaseRadix
from .arithmetic import PhaseArithmetic, AddOperation, SubtractOperation
from .blockchain import BlockchainIndex, SignalNode
from .compiler import PhaseCompiler
from .governance import (
    GovernancePipeline,
    LevelStackValidator,
    TypeIdentityValidator,
    AestheticValidator,
    OwnershipValidator,
    DiffractionRuleValidator,
    BlacklistValidator,
    IntentionFilter,
    GlobalCoordinationScore,
)

__version__ = "0.1.0"
__author__ = "刘楚恬和我 · 满全法"

__all__ = [
    "PhaseCanvas",
    "PhaseCell", 
    "PhaseRadix",
    "PhaseArithmetic",
    "AddOperation",
    "SubtractOperation",
    "BlockchainIndex",
    "SignalNode",
    "PhaseCompiler",
    "GovernancePipeline",
    "LevelStackValidator",
    "TypeIdentityValidator",
    "AestheticValidator",
    "OwnershipValidator",
    "DiffractionRuleValidator",
    "BlacklistValidator",
    "IntentionFilter",
    "GlobalCoordinationScore",
]
