"""
相位编译器

把相位编码翻译为可显示的输出：
  → RGB 图像 (PIL Image)
  -→ ASCII 艺术 (终端预览)
  -→ JSON 序列化
  -→ WAV 音频 (听觉侧)

这是"最后一公里"——只在输出时翻译。
底层本体永远是相位值。
"""

import json
import math
import struct
import wave
from typing import Dict, List, Optional, Tuple

from .canvas import PhaseCanvas, PhaseCell, hsv_to_rgb


class PhaseCompiler:
    """
    相位编译器：相位 → 可消费输出的翻译器
    
    不是改变底层数据，只是在输出时做一次翻译。
    就像中文书翻译成英文书——原文不变。
    """
    
    def __init__(self, charset: str = " .:-=+*#%@"):
        self.charset = charset
    
    def to_rgb_matrix(self, canvas: PhaseCanvas) -> List[List[Tuple[int, int, int]]]:
        """编译为 RGB 矩阵（二维数组）"""
        return canvas.compile_to_rgb()
    
    def to_image(self, canvas: PhaseCanvas, scale: int = 1):
        """
        编译为 PIL Image（需要 pillow）
        
        scale: 放大倍数（1=原始大小，10=10倍放大便于查看）
        """
        try:
            from PIL import Image
        except ImportError:
            raise ImportError("需要 pillow: pip install pillow")
        
        rgb_matrix = canvas.compile_to_rgb()
        h = len(rgb_matrix)
        w = len(rgb_matrix[0]) if h > 0 else 0
        
        img = Image.new("RGB", (w * scale, h * scale))
        pixels = img.load()
        
        for y in range(h):
            for x in range(w):
                color = rgb_matrix[y][x]
                for sy in range(scale):
                    for sx in range(scale):
                        pixels[x * scale + sx, y * scale + sy] = color
        
        return img
    
    def to_ascii(self, canvas: PhaseCanvas) -> str:
        """编译为 ASCII 艺术（无需依赖）统计信息"""
        header = f"┌── 相位画布: {canvas.name} ({canvas.width}x{canvas.height}) ──┐"
        footer = f"└{'─' * (canvas.width + 4)}┘"
        
        lines = [header]
        art = canvas.to_ascii_art(self.charset)
        for line in art.split("\n"):
            lines.append(f"│ {line} │")
        lines.append(footer)
        
        # 追加统计
        stats = canvas.statistics()
        lines.append(f"  激活单元: {stats['active_cells']}/{stats['total_cells']}")
        lines.append(f"  平均字重: {stats['avg_weight']}")
        lines.append(f"  层级分布: {stats['level_distribution']}")
        
        return "\n".join(lines)
    
    def to_json(self, canvas: PhaseCanvas, include_inactive: bool = False) -> str:
        """编译为 JSON（序列化相位矩阵）"""
        cells_data = []
        for y in range(canvas.height):
            for x in range(canvas.width):
                cell = canvas.get(x, y)
                if cell and (cell.active or include_inactive):
                    snap = cell.snapshot()
                    snap["x"] = x
                    snap["y"] = y
                    cells_data.append(snap)
        
        data = {
            "name": canvas.name,
            "width": canvas.width,
            "height": canvas.height,
            "cells": cells_data,
        }
        return json.dumps(data, ensure_ascii=False, indent=2)
    
    def to_wave(self, canvas: PhaseCanvas, 
                sample_rate: int = 44100, duration_ms: int = 200) -> bytes:
        """
        编译为 WAV 音频（听觉侧）
        
        把相位值映射为音高：
        - phase_value → 频率 (Hz)
        - weight → 振幅
        - density → 谐波复杂度
        
        这是"相位→声波"的物理翻译。
        相位连续变化 = 连续声波。
        """
        num_samples = int(sample_rate * duration_ms / 1000)
        samples = []
        
        # 收集所有激活单元的相位值
        active_cells = []
        for row in canvas.cells:
            for cell in row:
                if cell.active:
                    active_cells.append(cell)
        
        if not active_cells:
            return b""
        
        # 每个样本是所有激活相位的叠加
        for i in range(num_samples):
            t = i / sample_rate
            value = 0.0
            
            for cell in active_cells:
                # 相位值映射为频率（20Hz ~ 2000Hz）
                freq = 20 + (cell.phase_value / 360.0) * 1980
                # 字重映射为振幅（0.0 ~ 0.3，避免削波）
                amplitude = 0.3 * min(1.0, cell.weight / 10.0)
                
                value += amplitude * math.sin(2 * math.pi * freq * t)
            
            # 归一化
            value /= max(len(active_cells), 1)
            samples.append(value)
        
        # 转换为 16-bit PCM
        wav_data = b""
        for s in samples:
            s16 = int(max(-1, min(1, s)) * 32767)
            wav_data += struct.pack("<h", s16)
        
        # 包装为 WAV 格式
        wav_buffer = b""
        wav_buffer += b"RIFF"
        wav_buffer += struct.pack("<I", 36 + len(wav_data))
        wav_buffer += b"WAVE"
        wav_buffer += b"fmt "
        wav_buffer += struct.pack("<I", 16)  # PCM header size
        wav_buffer += struct.pack("<H", 1)   # PCM format
        wav_buffer += struct.pack("<H", 1)   # mono
        wav_buffer += struct.pack("<I", sample_rate)
        wav_buffer += struct.pack("<I", sample_rate * 2)  # byte rate
        wav_buffer += struct.pack("<H", 2)   # block align
        wav_buffer += struct.pack("<H", 16)  # bits per sample
        wav_buffer += b"data"
        wav_buffer += struct.pack("<I", len(wav_data))
        wav_buffer += wav_data
        
        return wav_buffer
    
    def save_image(self, canvas: PhaseCanvas, filepath: str, scale: int = 10) -> None:
        """保存为图像文件"""
        img = self.to_image(canvas, scale)
        img.save(filepath)
    
    def save_wave(self, canvas: PhaseCanvas, filepath: str) -> None:
        """保存为 WAV 音频文件"""
        wav_data = self.to_wave(canvas)
        with open(filepath, "wb") as f:
            f.write(wav_data)
    
    def save_json(self, canvas: PhaseCanvas, filepath: str) -> None:
        """保存为 JSON"""
        json_str = self.to_json(canvas)
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(json_str)


class BatchCompiler:
    """
    批量编译器
    
    一次性编译多个画布，统一输出。
    """
    
    def __init__(self):
        self.compiler = PhaseCompiler()
    
    def compile_all(self, canvases: List[PhaseCanvas], 
                    output_dir: str, formats: List[str] = None) -> Dict[str, List[str]]:
        """
        批量编译所有画布
        
        formats: 输出格式列表 ["png", "wav", "json", "ascii"]
        返回：每个画布生成的文件列表
        """
        if formats is None:
            formats = ["png", "json", "ascii"]
        
        import os
        os.makedirs(output_dir, exist_ok=True)
        
        results = {}
        for canvas in canvases:
            files = []
            
            if "png" in formats:
                try:
                    png_path = os.path.join(output_dir, f"{canvas.name}.png")
                    self.compiler.save_image(canvas, png_path)
                    files.append(png_path)
                except ImportError:
                    pass
            
            if "wav" in formats:
                wav_path = os.path.join(output_dir, f"{canvas.name}.wav")
                self.compiler.save_wave(canvas, wav_path)
                files.append(wav_path)
            
            if "json" in formats:
                json_path = os.path.join(output_dir, f"{canvas.name}.json")
                self.compiler.save_json(canvas, json_path)
                files.append(json_path)
            
            if "ascii" in formats:
                ascii_path = os.path.join(output_dir, f"{canvas.name}.txt")
                with open(ascii_path, "w", encoding="utf-8") as f:
                    f.write(self.compiler.to_ascii(canvas))
                files.append(ascii_path)
            
            results[canvas.name] = files
        
        return results
