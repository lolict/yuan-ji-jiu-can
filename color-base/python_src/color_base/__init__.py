"""
颜色进制 · 相位编码系统
========================

核心概念：
- PhasePixel: 每个像素存储相位、权重、亮度、元数据
- PhaseCanvas: 相位像素网格
- ADD: 部件安装（像素相位加法）
- SUB: 部件拆解（像素相位减法）
- 禁止: 乘法、除法（对符号构造无意义）
- 区块链纠缠索引: 每个节点携带前后缀指针 + 元数据纸条
- 分布式制衡网络: 多维度互相拉扯校验

可编译: .cbf 二进制格式
可解释: REPL 交互式解释器
可打包: .cbpack 分发包
"""

from .pixel import PhasePixel, PhaseCanvas
from .ops import phase_add, phase_sub, phase_mul, phase_div, PhaseOpsError
from .blockchain import GlyphNode, BlockchainIndex
from .balance import BalanceDimension, BalanceNetwork
from .compiler import ColorBaseCompiler
from .interpreter import ColorBaseInterpreter
from .packager import ColorBasePackager

__version__ = "1.0.0"
__all__ = [
    "__version__",
    "PhasePixel", "PhaseCanvas",
    "phase_add", "phase_sub", "phase_mul", "phase_div", "PhaseOpsError",
    "GlyphNode", "BlockchainIndex",
    "BalanceDimension", "BalanceNetwork",
    "ColorBaseCompiler", "ColorBaseInterpreter", "ColorBasePackager",
]
