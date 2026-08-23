"""
相位画布 —— 核心数据结构

每个格子（PhaseCell）存储：
  - phase_value: 相位编码值（0~255 对应一个相位环）
  - weight: 视觉块面重量（胖瘦）
  - owner: 归属权（SELF=自有, EXTERNAL=外来）
  - level: 层级标签（L1=基元本体, L2=修饰零件, L3=装配成品）
  - family: 字形家族（latin/cyrillic/ipa/cjk/math）
  - goal: 目标锚点
  - metadata: 任意扩展元数据

相位值是"一等公民"，RGB只是输出翻译。
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
import math


@dataclass
class PhaseCell:
    """
    相位单元 = 画布上的一个最小格子
    
    不是 RGB 颜色，是一组相位编码 + 元数据。
    相位值决定这个格子在"相位环"上的位置。
    两个格子的相位差 = 它们的视觉/结构差异量。
    """
    phase_value: float = 0.0       # 0.0 ~ 360.0（相位环上的角度）
    weight: float = 1.0            # 视觉块面重量（0.0=极瘦, 10.0=极胖）
    density: float = 1.0           # 笔画疏密（0.0=极简, 10.0=极繁）
    roundness: float = 0.5         # 圆弧/棱角（0.0=全棱, 1.0=全圆）
    owner: str = "SELF"            # SELF=自有, EXTERNAL=外来, UNKNOWN=未分类
    level: str = "L1"              # L1=基元本体, L2=修饰零件, L3=装配成品
    family: str = "latin"          # 字形家族
    goal: str = "system"           # 目标锚点
    symmetry: float = 0.5          # 对称性（0.0=不对称, 1.0=完全对称）
    topology: float = 1.0          # 拓扑复杂度
    active: bool = True            # 是否激活（False=被挖除）
    metadata: Dict = field(default_factory=dict)  # 扩展元数据纸条
    
    def phase_distance(self, other: 'PhaseCell') -> float:
        """
        相位距离 = 两个相位的环形最短距离
        范围 0~180（360度环上最远是对面的180度）
        """
        raw = abs(self.phase_value - other.phase_value) % 360
        return min(raw, 360 - raw)
    
    def visual_distance(self, other: 'PhaseCell') -> float:
        """
        视觉距离 = 综合字重、密度、圆度、对称性、拓扑的差异
        用于审美配对判断
        """
        dw = (self.weight - other.weight) ** 2
        dd = (self.density - other.density) ** 2
        dr = (self.roundness - other.roundness) ** 2
        ds = (self.symmetry - other.symmetry) ** 2
        dt = (self.topology - other.topology) ** 2
        
        # 字重和密度的权重最高（它们直接决定"胖瘦"感知）
        return math.sqrt(dw * 2.0 + dd * 1.5 + dr + ds * 0.5 + dt * 0.5)
    
    def to_rgb(self) -> Tuple[int, int, int]:
        """
        编译：相位值 → RGB 颜色
        
        相位环上的角度映射到色相环：
        - 0° = 红色（基元本色）
        - 120° = 绿色（韵母衍生）
        - 240° = 蓝色（声调装配）
        
        这只是输出翻译。底层本体是相位值。
        """
        hue = self.phase_value % 360
        saturation = min(1.0, self.weight / 5.0)  # 重量越大越饱和
        value = 1.0 - (self.density / 20.0)       # 密度越大越暗
        
        return hsv_to_rgb(hue, max(0.3, saturation), max(0.4, value))
    
    def snapshot(self) -> Dict:
        """输出当前状态的快照（用于区块链索引）"""
        return {
            "phase_value": self.phase_value,
            "weight": self.weight,
            "density": self.density,
            "roundness": self.roundness,
            "owner": self.owner,
            "level": self.level,
            "family": self.family,
            "goal": self.goal,
            "active": self.active,
        }


@dataclass
class PhaseCanvas:
    """
    相位画布 = 二维相位单元矩阵
    
    不是 RGB 像素图。
    每个格子存的是相位编码。
    画图 = 在相位矩阵上做加减拼装。
    """
    name: str
    width: int
    height: int
    cells: List[List[PhaseCell]] = field(default_factory=list)
    
    def __post_init__(self):
        if not self.cells:
            self.cells = [
                [PhaseCell() for _ in range(self.width)]
                for _ in range(self.height)
            ]
    
    def get(self, x: int, y: int) -> Optional[PhaseCell]:
        """获取指定坐标的相位单元"""
        if 0 <= x < self.width and 0 <= y < self.height:
            return self.cells[y][x]
        return None
    
    def set(self, x: int, y: int, cell: PhaseCell) -> None:
        """设置指定坐标的相位单元"""
        if 0 <= x < self.width and 0 <= y < self.height:
            self.cells[y][x] = cell
    
    def region(self, x: int, y: int, w: int, h: int) -> List[List[PhaseCell]]:
        """获取一个矩形区域的相位单元"""
        region = []
        for dy in range(h):
            row = []
            for dx in range(w):
                cell = self.get(x + dx, y + dy)
                if cell:
                    row.append(cell)
            if row:
                region.append(row)
        return region
    
    def install_component(self, component: 'PhaseCanvas', 
                          offset_x: int = 0, offset_y: int = 0,
                          mode: str = "add") -> None:
        """
        加法运算：把一个部件画布安装到当前画布上
        
        mode="add"：相位值叠加（平均融合）
        mode="replace"：直接替换
        
        这就是你说的"把别的字的结构拆解下来安装在他的身上"
        """
        for y in range(component.height):
            for x in range(component.width):
                src = component.get(x, y)
                dst = self.get(offset_x + x, offset_y + y)
                
                if src is None or dst is None:
                    continue
                if not src.active:
                    continue
                
                if mode == "add":
                    # 相位叠加：取平均（制造"装配成品"的效果）
                    dst.phase_value = (dst.phase_value + src.phase_value) / 2
                    dst.weight = max(dst.weight, src.weight)
                    dst.density = min(10.0, dst.density + src.density * 0.3)
                    dst.level = "L3"  # 装配结果自动标记为 L3
                    dst.metadata["assembled_from"] = dst.metadata.get("assembled_from", []) + [component.name]
                elif mode == "replace":
                    dst.phase_value = src.phase_value
                    dst.weight = src.weight
                    dst.density = src.density
                    dst.level = src.level
                    dst.metadata = {**dst.metadata, **src.metadata}
    
    def subtract_region(self, x: int, y: int, w: int, h: int,
                        mode: str = "deactivate") -> None:
        """
        减法运算：挖除一个区域
        
        mode="deactivate"：标记为未激活（保留相位信息但标记挖除）
        mode="erase"：重置为零
        
        这就是你说的"把一个字拆解下来"
        """
        for dy in range(h):
            for dx in range(w):
                cell = self.get(x + dx, y + dy)
                if cell is None:
                    continue
                
                if mode == "deactivate":
                    cell.active = False
                elif mode == "erase":
                    cell.phase_value = 0.0
                    cell.weight = 0.0
                    cell.density = 0.0
                    cell.active = False
    
    def compile_to_rgb(self) -> List[List[Tuple[int, int, int]]]:
        """
        编译输出：把整个相位画布翻译为 RGB 像素矩阵
        
        相位值 → 色相环映射 → RGB
        这是"最后一公里"，只在输出时翻译。
        """
        rows = []
        for y in range(self.height):
            row = []
            for x in range(self.width):
                cell = self.cells[y][x]
                if cell.active:
                    row.append(cell.to_rgb())
                else:
                    # 未激活区域 = 透明度提示（浅灰色）
                    row.append((240, 240, 240))
            rows.append(row)
        return rows
    
    def to_image(self):
        """
        编译为 PIL Image 对象（需要安装 pillow）
        """
        try:
            from PIL import Image
        except ImportError:
            raise ImportError("需要安装 pillow: pip install pillow")
        
        rgb_matrix = self.compile_to_rgb()
        img = Image.new("RGB", (self.width, self.height))
        pixels = img.load()
        
        for y in range(self.height):
            for x in range(self.width):
                pixels[x, y] = rgb_matrix[y][x]
        
        return img
    
    def to_ascii_art(self, charset: str = " .:-=+*#%@") -> str:
        """
        编译为 ASCII 艺术（无需任何依赖）
        
        用相位值和字重决定字符密度。
        用于终端预览和快速调试。
        """
        lines = []
        for y in range(self.height):
            line = ""
            for x in range(self.width):
                cell = self.cells[y][x]
                if not cell.active:
                    line += " "
                else:
                    # 用密度+重量决定字符
                    intensity = (cell.weight + cell.density) / 20.0
                    idx = int(intensity * (len(charset) - 1))
                    idx = max(0, min(len(charset) - 1, idx))
                    line += charset[idx]
            lines.append(line)
        return "\n".join(lines)
    
    def statistics(self) -> Dict:
        """画布统计信息"""
        total = self.width * self.height
        active = sum(1 for row in self.cells for c in row if c.active)
        avg_phase = sum(c.phase_value for row in self.cells for c in row if c.active) / max(active, 1)
        avg_weight = sum(c.weight for row in self.cells for c in row if c.active) / max(active, 1)
        
        levels = {}
        for row in self.cells:
            for c in row:
                if c.active:
                    levels[c.level] = levels.get(c.level, 0) + 1
        
        return {
            "name": self.name,
            "width": self.width,
            "height": self.height,
            "total_cells": total,
            "active_cells": active,
            "inactive_cells": total - active,
            "avg_phase": round(avg_phase, 2),
            "avg_weight": round(avg_weight, 2),
            "level_distribution": levels,
        }


class PhaseRadix:
    """
    相位进制系统 = 管理多个相位画布的工厂
    
    每个画布是一个"符号的相位矩阵"。
    画布之间通过加减运算做部件拼装。
    """
    
    def __init__(self):
        self.canvases: Dict[str, PhaseCanvas] = {}
        self.component_library: Dict[str, PhaseCanvas] = {}  # 部件库
    
    def canvas(self, name: str, width: int = 16, height: int = 16) -> PhaseCanvas:
        """创建或获取命名画布"""
        if name not in self.canvases:
            self.canvases[name] = PhaseCanvas(name=name, width=width, height=height)
        return self.canvases[name]
    
    def register_component(self, name: str, canvas: PhaseCanvas) -> None:
        """注册一个部件到部件库"""
        self.component_library[name] = canvas
    
    def get_component(self, name: str) -> Optional[PhaseCanvas]:
        """从部件库获取部件"""
        return self.component_library.get(name)
    
    def assemble(self, target_name: str, components: List[Dict]) -> PhaseCanvas:
        """
        装配：把多个部件拼装到一个目标画布上
        
        每个 component 是 {"name": 部件名, "x": x偏移, "y": y偏移}
        """
        target = self.canvases.get(target_name)
        if not target:
            raise ValueError(f"目标画布 {target_name} 不存在")
        
        for comp_info in components:
            comp_name = comp_info["name"]
            offset_x = comp_info.get("x", 0)
            offset_y = comp_info.get("y", 0)
            
            component = self.component_library.get(comp_name)
            if component:
                target.install_component(component, offset_x, offset_y, mode="add")
        
        return target


def hsv_to_rgb(h: float, s: float, v: float) -> Tuple[int, int, int]:
    """
    HSV → RGB 转换
    h: 0~360, s: 0~1, v: 0~1
    """
    h = h % 360
    c = v * s
    x = c * (1 - abs((h / 60) % 2 - 1))
    m = v - c
    
    if h < 60:
        r, g, b = c, x, 0
    elif h < 120:
        r, g, b = x, c, 0
    elif h < 180:
        r, g, b = 0, c, x
    elif h < 240:
        r, g, b = 0, x, c
    elif h < 300:
        r, g, b = x, 0, c
    else:
        r, g, b = c, 0, x
    
    return (
        int((r + m) * 255),
        int((g + m) * 255),
        int((b + m) * 255),
    )
