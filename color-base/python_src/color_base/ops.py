"""
像素相位运算 · 仅允许加法(+)/减法(-)，禁止乘法(*)/除法(/)

加法 = 部件安装、拼装、移植
减法 = 部件拆解、剥离、挖除
乘除 = 对符号构造无意义，直接禁用
"""

from __future__ import annotations
from typing import Tuple
from .pixel import PhasePixel, PhaseCanvas


class PhaseOpsError(Exception):
    """相位运算错误"""
    pass


def phase_add(base: PhasePixel, component: PhasePixel, 
              blend_mode: str = "overlay") -> PhasePixel:
    """
    像素相位加法: base + component
    语义: 将组件部件安装/叠加到基础像素上
    
    blend_mode:
      - "overlay": 叠加（component 覆盖 base 的非零部分）
      - "merge": 融合（取平均/加权）
      - "stamp": 烙印（component 的相位完全替换 base）
    """
    if base.blacklisted or component.blacklisted:
        raise PhaseOpsError("无法对黑名单像素执行加法运算")
    
    result = PhasePixel(
        phase=base.phase,
        weight=base.weight,
        brightness=base.brightness,
        owner=base.owner,
        level=max(base.level, component.level),
        family=base.family,
        parent_source=base.parent_source + component.parent_source,
        blacklisted=False,
        prefix_ids=list(base.prefix_ids),
        suffix_ids=list(base.suffix_ids),
    )
    
    if blend_mode == "overlay":
        # 叠加：组件有值时取组件，否则保留 base
        if component.phase != 0:
            result.phase = component.phase
            result.weight = component.weight
            result.brightness = component.brightness
            result.family = component.family
            result.suffix_ids.extend(component.suffix_ids)
    elif blend_mode == "merge":
        # 融合：取加权平均
        total_w = base.weight + component.weight
        if total_w > 0:
            result.phase = int((base.phase * base.weight + component.phase * component.weight) / total_w)
            result.weight = min(total_w, 1.0)
            result.brightness = (base.brightness + component.brightness) / 2
        result.parent_source.append(f"merge(φ{base.phase}+φ{component.phase})")
    elif blend_mode == "stamp":
        result.phase = component.phase
        result.weight = component.weight
        result.brightness = component.brightness
        result.family = component.family
        result.parent_source.append(f"stamp(φ{component.phase})")
    else:
        raise PhaseOpsError(f"未知混合模式: {blend_mode}")
    
    return result


def phase_sub(base: PhasePixel, component: PhasePixel) -> PhasePixel:
    """
    像素相位减法: base - component
    语义: 从基础像素上剥离/挖除部件
    
    用途: 把装配成品拆回 L1 基元本体
    """
    if base.blacklisted:
        raise PhaseOpsError("无法对黑名单像素执行减法运算")
    
    result = PhasePixel(
        phase=base.phase,
        weight=base.weight,
        brightness=base.brightness,
        owner=base.owner,
        level=base.level,
        family=base.family,
        parent_source=list(base.parent_source),
        blacklisted=base.blacklisted,
        prefix_ids=list(base.prefix_ids),
        suffix_ids=list(base.suffix_ids),
    )
    
    # 减法: 剥离与 component 同相位的区域
    if base.phase == component.phase:
        result.phase = 0
        result.weight = 0.0
        result.brightness = 0.0
        result.parent_source.append(f"sub(φ{component.phase})")
    elif base.weight > component.weight:
        result.weight = base.weight - component.weight
        result.parent_source.append(f"sub_w({component.weight:.2f})")
    
    return result


def canvas_add(base: PhaseCanvas, component: PhaseCanvas, 
               offset_x: int = 0, offset_y: int = 0,
               blend_mode: str = "overlay") -> PhaseCanvas:
    """画布级加法: 将 component 安装到 base 上"""
    if base.blacklisted:
        raise PhaseOpsError("Cannot add to blacklisted canvas")
    
    # 创建新画布
    result = PhaseCanvas(base.width, base.height, f"{base.name}+{component.name}")
    # 先复制 base
    for y in range(base.height):
        for x in range(base.width):
            result.set(x, y, PhasePixel(
                phase=base.grid[y][x].phase,
                weight=base.grid[y][x].weight,
                brightness=base.grid[y][x].brightness,
                owner=base.grid[y][x].owner,
                level=base.grid[y][x].level,
                family=base.grid[y][x].family,
                parent_source=list(base.grid[y][x].parent_source),
                blacklisted=base.grid[y][x].blacklisted,
                prefix_ids=list(base.grid[y][x].prefix_ids),
                suffix_ids=list(base.grid[y][x].suffix_ids),
                r=base.grid[y][x].r, g=base.grid[y][x].g, b=base.grid[y][x].b,
            ))
    # 叠加 component
    for y in range(component.height):
        for x in range(component.width):
            tx, ty = x + offset_x, y + offset_y
            if 0 <= tx < base.width and 0 <= ty < base.height:
                try:
                    merged = phase_add(result.get(tx, ty), component.grid[y][x], blend_mode)
                    result.set(tx, ty, merged)
                except PhaseOpsError:
                    pass  # 跳过黑名单区域
    return result


def canvas_sub(base: PhaseCanvas, component: PhaseCanvas,
               offset_x: int = 0, offset_y: int = 0) -> PhaseCanvas:
    """画布级减法: 从 base 上剥离 component"""
    result = PhaseCanvas(base.width, base.height, f"{base.name}-{component.name}")
    for y in range(base.height):
        for x in range(base.width):
            result.set(x, y, PhasePixel(
                phase=base.grid[y][x].phase,
                weight=base.grid[y][x].weight,
                brightness=base.grid[y][x].brightness,
                owner=base.grid[y][x].owner,
                level=base.grid[y][x].level,
                family=base.grid[y][x].family,
                parent_source=list(base.grid[y][x].parent_source),
                blacklisted=base.grid[y][x].blacklisted,
                prefix_ids=list(base.grid[y][x].prefix_ids),
                suffix_ids=list(base.grid[y][x].suffix_ids),
            ))
    for y in range(component.height):
        for x in range(component.width):
            tx, ty = x + offset_x, y + offset_y
            if 0 <= tx < base.width and 0 <= ty < base.height:
                result.set(tx, ty, phase_sub(result.get(tx, ty), component.grid[y][x]))
    return result


# ============================================================
# 乘法/除法 — 明确禁止
# ============================================================

def _multiply_disabled(*args, **kwargs):
    raise PhaseOpsError(
        "乘法运算已禁用: 对符号构造无意义（会改变视觉权重，破坏等效镜像距离）"
    )

def _divide_disabled(*args, **kwargs):
    raise PhaseOpsError(
        "除法运算已禁用: 对符号构造无意义（不具有均分结构的合理业务）"
    )

# 使乘除法不可调用
phase_mul = _multiply_disabled
phase_div = _divide_disabled
