"""
相位加减运算

只允许：加法（部件安装）、减法（部件拆解）
禁用：乘法、除法

这不是限制——这是对汉字/符号构造本质的反映：
部件只能拼装或拆解，不能被"放大"或"均分"。
"""

from dataclasses import dataclass
from typing import Dict, Optional, Tuple
from .canvas import PhaseCanvas, PhaseCell


class AddOperation:
    """
    像素相位加法 = 部件移植、拼接、安装
    
    把 A 字形拆解出来的局部像素部件，拆卸，移植、安装到另一个字形本体上。
    不是文本字符串拼接，是像素矩阵层面的物理安装。
    """
    
    @staticmethod
    def execute(
        target: PhaseCanvas,
        source: PhaseCanvas,
        offset_x: int = 0,
        offset_y: int = 0,
        blend_mode: str = "average"  # average/max/weighted
    ) -> PhaseCanvas:
        """
        执行加法：把 source 安装到 target 上
        
        blend_mode:
          - average: 相位值平均融合（适合同权重的部件拼装）
          - max: 取最大相位值（适合强调结构差异）
          - weighted: 按字重加权融合（适合视觉重量不同的部件）
        """
        for y in range(source.height):
            for x in range(source.width):
                src_cell = source.get(x, y)
                dst_cell = target.get(offset_x + x, offset_y + y)
                
                if src_cell is None or dst_cell is None:
                    continue
                if not src_cell.active:
                    continue
                
                if blend_mode == "average":
                    dst_cell.phase_value = (dst_cell.phase_value + src_cell.phase_value) / 2
                    dst_cell.weight = (dst_cell.weight + src_cell.weight) / 2
                    dst_cell.density = min(10.0, dst_cell.density + src_cell.density * 0.3)
                    
                elif blend_mode == "max":
                    dst_cell.phase_value = max(dst_cell.phase_value, src_cell.phase_value)
                    dst_cell.weight = max(dst_cell.weight, src_cell.weight)
                    dst_cell.density = max(dst_cell.density, src_cell.density)
                    
                elif blend_mode == "weighted":
                    total_w = dst_cell.weight + src_cell.weight
                    if total_w > 0:
                        rw_dst = dst_cell.weight / total_w
                        rw_src = src_cell.weight / total_w
                    else:
                        rw_dst = rw_src = 0.5
                    
                    dst_cell.phase_value = (
                        dst_cell.phase_value * rw_dst + src_cell.phase_value * rw_src
                    )
                    dst_cell.weight = dst_cell.weight * rw_dst + src_cell.weight * rw_src
                    dst_cell.density = min(10.0, dst_cell.density * rw_dst + src_cell.density * rw_src)
                
                # 装配结果自动标记为 L3
                dst_cell.level = "L3"
                
                # 元数据溯源
                history = dst_cell.metadata.get("assembly_history", [])
                history.append({
                    "operation": "ADD",
                    "source": source.name,
                    "blend_mode": blend_mode,
                    "position": (offset_x + x, offset_y + y),
                })
                dst_cell.metadata["assembly_history"] = history
        
        return target


class SubtractOperation:
    """
    像素相位减法 = 拆解、剥离、挖除部件
    
    从一个完整字形上面，挖掉、剥离掉某一部分像素部件，得到剩下的骨架碎片。
    这是层级校验的底层物理证据。
    """
    
    @staticmethod
    def execute(
        target: PhaseCanvas,
        x: int, y: int, w: int, h: int,
        mode: str = "deactivate"  # deactivate/erase/extract
    ) -> Optional[PhaseCanvas]:
        """
        执行减法：从 target 中挖除一个区域
        
        mode:
          - deactivate: 标记未激活（保留相位信息）
          - erase: 完全清除
          - extract: 把挖除的部分单独输出为一个新画布
        """
        if mode == "extract":
            # 提取挖除的部分为新画布
            extracted = PhaseCanvas(
                name=f"{target.name}_extracted",
                width=w, height=h
            )
            for dy in range(h):
                for dx in range(w):
                    src = target.get(x + dx, y + dy)
                    dst = extracted.get(dx, dy)
                    if src and dst:
                        dst.phase_value = src.phase_value
                        dst.weight = src.weight
                        dst.density = src.density
                        dst.level = src.level
                        dst.active = True
            
            # 原位置标记未激活
            for dy in range(h):
                for dx in range(w):
                    cell = target.get(x + dx, y + dy)
                    if cell:
                        cell.active = False
            
            return extracted
        
        elif mode == "deactivate":
            for dy in range(h):
                for dx in range(w):
                    cell = target.get(x + dx, y + dy)
                    if cell:
                        cell.active = False
                        cell.metadata["deactivated"] = True
                        cell.metadata["deactivation_reason"] = "subtract_operation"
        
        elif mode == "erase":
            for dy in range(h):
                for dx in range(w):
                    cell = target.get(x + dx, y + dy)
                    if cell:
                        cell.phase_value = 0.0
                        cell.weight = 0.0
                        cell.density = 0.0
                        cell.active = False
        
        return None  # deactivate/erase 不返回新画布


class PhaseArithmetic:
    """
    相位运算器：统一管理加减运算
    
    乘法、除法不在此类中实现——它们被永久禁用。
    """
    
    def __init__(self):
        self.add = AddOperation()
        self.subtract = SubtractOperation()
        self.operation_log = []
    
    def assemble(
        self,
        target: PhaseCanvas,
        source: PhaseCanvas,
        offset: Tuple[int, int] = (0, 0),
        blend_mode: str = "average"
    ) -> PhaseCanvas:
        """加法：安装部件"""
        self.operation_log.append({
            "op": "ADD",
            "source": source.name,
            "target": target.name,
            "offset": offset,
            "blend_mode": blend_mode,
        })
        return self.add.execute(target, source, offset[0], offset[1], blend_mode)
    
    def dismantle(
        self,
        target: PhaseCanvas,
        region: Tuple[int, int, int, int],
        mode: str = "deactivate"
    ) -> Optional[PhaseCanvas]:
        """减法：拆解部件"""
        x, y, w, h = region
        self.operation_log.append({
            "op": "SUBTRACT",
            "target": target.name,
            "region": region,
            "mode": mode,
        })
        return self.subtract.execute(target, x, y, w, h, mode)
    
    def reverse_assembly(self, assembled: PhaseCanvas) -> Dict:
        """
        溯源：从装配结果反推它是由哪些部件拼装而来
        
        这就是"á − 部件 = a"的底层实现。
        用于层级校验：证明两个符号是"本体‑装配产物"关系，不是平级基元。
        """
        sources = {}
        for row in assembled.cells:
            for cell in row:
                if not cell.active:
                    continue
                history = cell.metadata.get("assembly_history", [])
                for h in history:
                    src_name = h.get("source", "unknown")
                    sources[src_name] = sources.get(src_name, 0) + 1
        
        return {
            "assembled_name": assembled.name,
            "source_components": sources,
            "is_assembled": len(sources) > 0,
            "inference": "装配成品" if sources else "原生基元（未检测到装配历史）",
        }
