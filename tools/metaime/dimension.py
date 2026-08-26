#!/usr/bin/env python3
"""
FQM 维度升降解释器 · 高维⇄低维映射

核心概念：
- 高维空间：(声母.x, 韵母.y, 声调.z) = 三维坐标
- 低维投影：(声母.x, 韵母.y) = 二维平面（去掉声调）
- 一维压缩：声母.x = 一轴（只保留声母）
- 升维：从低维恢复高维（需要默认值或推断）

操作语法：
  降维(b, a, 1, 声调)     → 去掉声调维度，得到 (声母.x, 韵母.y)
  升维(b, a, 声调)        → 从二维升到三维，声调取默认
  投影(b, a, 1, 声母)    → 只保留声母维度
  展开(b, 韵母, 声调)     → 从声母展开到完整三维
"""

import os
import sys
import re


class DimensionInterpreter:
    """维度升降解释器"""
    
    def __init__(self, linker):
        self.linker = linker
        self.coordinates = {}    # (shengmu_id, yunmu_id, tone_id) -> (sm_sym, ym_sym, t_sym)
        self._build_coordinates()
    
    def _build_coordinates(self):
        """构建所有三维坐标"""
        shengmu_container = self.linker.loader.get_container("声母")
        yunmu_container = self.linker.loader.get_container("韵母")
        tone_container = self.linker.loader.get_container("声调")
        
        if not shengmu_container or not yunmu_container or not tone_container:
            return
        
        for link in self.linker.resolved:
            from_sym = link['from']['symbol']
            to_sym = link['to']['symbol']
            from_path = link['from']['path']
            to_path = link['to']['path']
            
            for tone_entry in tone_container.symbols:
                tone_sym = tone_entry["symbol"]
                tone_identity = self._find_identity("声调", tone_sym)
                if tone_identity:
                    key = (from_path.identity, to_path.identity, tone_identity)
                    self.coordinates[key] = (from_sym, to_sym, tone_sym)
    
    def _find_identity(self, domain, symbol):
        """查找符号在容器中的身份"""
        for (d, s), p in self.linker.symbol_to_path.items():
            if d == domain and s == symbol:
                return p.identity
        return None
    
    def _find_symbol(self, domain, identity):
        """查找身份对应的符号"""
        for (d, s), p in self.linker.symbol_to_path.items():
            if d == domain and p.identity == identity:
                return s
        return None
    
    def _symbol_to_full_coord(self, shengmu_sym, yunmu_sym, tone_sym):
        """符号转完整三维坐标（身份）"""
        sm_id = self._find_identity("声母", shengmu_sym)
        ym_id = self._find_identity("韵母", yunmu_sym)
        t_id = self._find_identity("声调", tone_sym)
        if sm_id and ym_id and t_id:
            return (sm_id, ym_id, t_id)
        return None
    
    def execute(self, command):
        """执行维度命令"""
        command = command.strip()
        if not command or command.startswith('#'):
            return None
        
        # 降维: 降维(b, a, 1, 声调)
        if command.startswith('降维('):
            return self._cmd_downshift(command)
        
        # 升维: 升维(b, a, 声调)
        if command.startswith('升维('):
            return self._cmd_upshift(command)
        
        # 投影: 投影(b, a, 1, 声母)
        if command.startswith('投影('):
            return self._cmd_project(command)
        
        # 展开: 展开(b, 韵母, 声调)
        if command.startswith('展开('):
            return self._cmd_expand(command)
        
        # 维度信息
        if command == '维度':
            return self._cmd_dimensions()
        
        return f"错误: 未知命令 {command}"
    
    def _cmd_downshift(self, command):
        """降维: 降维(b, a, 1, 声调) → 去掉声调维度"""
        match = re.match(r'降维\((.+)\)', command)
        if not match:
            return "错误: 降维格式"
        
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 4:
            return "错误: 降维需要4个参数 (b, a, 1, 声调)"
        
        shengmu_sym, yunmu_sym, tone_sym, drop_dim = args
        
        # 查找完整坐标
        coord = self._symbol_to_full_coord(shengmu_sym, yunmu_sym, tone_sym)
        if not coord:
            return f"错误: 找不到坐标 ({shengmu_sym}, {yunmu_sym}, {tone_sym})"
        
        sm_id, ym_id, t_id = coord
        
        # 去掉指定维度
        drop_dim = drop_dim.strip()
        result_parts = []
        
        if drop_dim != '声母':
            result_parts.append(f"声母.{sm_id}({shengmu_sym})")
        if drop_dim != '韵母':
            result_parts.append(f"韵母.{ym_id}({yunmu_sym})")
        if drop_dim != '声调':
            result_parts.append(f"声调.{t_id}({tone_sym})")
        
        result = f"降维: ({shengmu_sym},{yunmu_sym},{tone_sym}) → "
        result += f"({', '.join(result_parts)})"
        
        return result
    
    def _cmd_upshift(self, command):
        """升维: 升维(b, a, 声调) → 从声母韵母升到三维，声调取默认"""
        match = re.match(r'升维\((.+)\)', command)
        if not match:
            return "错误: 升维格式"
        
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 3:
            return "错误: 升维需要3个参数 (b, a, 声调)"
        
        shengmu_sym, yunmu_sym, tone_default = args
        
        sm_id = self._find_identity("声母", shengmu_sym)
        ym_id = self._find_identity("韵母", yunmu_sym)
        
        if not sm_id or not ym_id:
            return f"错误: 找不到 ({shengmu_sym}, {yunmu_sym})"
        
        # 查找声调身份
        t_id = self._find_identity("声调", tone_default)
        if not t_id:
            return f"错误: 声调中无 {tone_default}"
        
        result = f"升维: ({shengmu_sym},{yunmu_sym}) + 声调{tone_default} → "
        result += f"(声母.{sm_id}({shengmu_sym}), 韵母.{ym_id}({yunmu_sym}), 声调.{t_id}({tone_default}))"
        
        return result
    
    def _cmd_project(self, command):
        """投影: 投影(b, a, 1, 声母) → 只保留声母维度"""
        match = re.match(r'投影\((.+)\)', command)
        if not match:
            return "错误: 投影格式"
        
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 4:
            return "错误: 投影需要4个参数 (b, a, 1, 声母)"
        
        shengmu_sym, yunmu_sym, tone_sym, keep_dim = args
        
        sm_id = self._find_identity("声母", shengmu_sym)
        if not sm_id:
            return f"错误: 声母中无 {shengmu_sym}"
        
        result = f"投影: ({shengmu_sym},{yunmu_sym},{tone_sym}) → "
        result += f"声母.{sm_id}({shengmu_sym})"
        
        return result
    
    def _cmd_expand(self, command):
        """展开: 展开(b, 韵母, 声调) → 从声母展开到完整三维"""
        match = re.match(r'展开\((.+)\)', command)
        if not match:
            return "错误: 展开格式"
        
        args = [a.strip() for a in match.group(1).split(',')]
        if len(args) != 3:
            return "错误: 展开需要3个参数 (b, 韵母, 声调)"
        
        shengmu_sym, yunmu_sym, tone_sym = args
        
        coord = self._symbol_to_full_coord(shengmu_sym, yunmu_sym, tone_sym)
        if not coord:
            return f"错误: 找不到完整坐标 ({shengmu_sym}, {yunmu_sym}, {tone_sym})"
        
        sm_id, ym_id, t_id = coord
        
        result = f"展开: {shengmu_sym} → "
        result += f"(声母.{sm_id}({shengmu_sym}), 韵母.{ym_id}({yunmu_sym}), 声调.{t_id}({tone_sym}))"
        
        return result
    
    def _cmd_dimensions(self):
        """显示维度信息"""
        shengmu_c = self.linker.loader.get_container("声母")
        yunmu_c = self.linker.loader.get_container("韵母")
        tone_c = self.linker.loader.get_container("声调")
        
        sm_count = len(shengmu_c) if shengmu_c else 0
        ym_count = len(yunmu_c) if yunmu_c else 0
        t_count = len(tone_c) if tone_c else 0
        
        result = "维度信息:\n"
        result += f"  声母轴: {sm_count} 个位置\n"
        result += f"  韵母轴: {ym_count} 个位置\n"
        result += f"  声调轴: {t_count} 个位置\n"
        result += f"  三维空间总容量: {sm_count}×{ym_count}×{t_count} = {sm_count*ym_count*t_count}\n"
        result += f"  已占用坐标: {len(self.coordinates)}"
        
        return result
    
    def display_coordinates(self):
        """显示所有坐标"""
        print(f"\n{'='*60}")
        print("高维坐标（完整三维）")
        print(f"{'-'*60}")
        
        for (sm_id, ym_id, t_id), syms in self.coordinates.items():
            print(f"  (声母.{sm_id}, 韵母.{ym_id}, 声调.{t_id}) = {syms}")
        
        print(f"\n{'='*60}\n")


# ============================================================
# 完整管道
# ============================================================

class FQMPipeline:
    """FQM 完整管道 v4（含维度解释器）"""
    
    def __init__(self, fqm_file):
        self.fqm_file = fqm_file
        self.loader = None
        self.linker = None
        self.interpreter = None
        self.radix3d = None
        self.dimension = None
    
    def run(self):
        """运行完整管道"""
        from loader import FQMLoader
        from linker import FQMLinker
        from interpreter import FQMInterpreter
        from radix3d import Radix3DInterpreter
        
        self.loader = FQMLoader()
        if not self.loader.load_file(self.fqm_file):
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.interpreter = FQMInterpreter(self.linker)
        self.radix3d = Radix3DInterpreter(self.linker)
        self.dimension = DimensionInterpreter(self.linker)
        
        return True
    
    def execute(self, command):
        """执行命令"""
        if command.strip().startswith('降维(') or command.strip().startswith('升维(') or \
           command.strip().startswith('投影(') or command.strip().startswith('展开(') or \
           command.strip() == '维度':
            return self.dimension.execute(command)
        if command.strip().startswith('三维.'):
            return self.radix3d.execute(command)
        return self.interpreter.execute(command)
    
    def display_all(self):
        """显示全部"""
        self.loader.display_summary()
        self.linker.display_ontology()
        self.radix3d.display_space()
        self.dimension.display_coordinates()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("=" * 60)
    print("FQM 维度升降解释器测试")
    print("=" * 60)
    
    fqm_file = "containers.fqm"
    
    pipe = FQMPipeline(fqm_file)
    if not pipe.run():
        print("管道启动失败")
        sys.exit(1)
    
    pipe.display_all()
    
    print("\n维度命令测试:")
    commands = [
        "维度",
        "降维(b, a, 1, 声调)",
        "升维(b, a, 1)",
        "投影(b, a, 1, 声母)",
        "展开(b, a, 1)",
    ]
    
    for cmd in commands:
        result = pipe.execute(cmd)
        if result:
            print(f"\n  {cmd}")
            for line in result.split('\n'):
                print(f"    {line}")
