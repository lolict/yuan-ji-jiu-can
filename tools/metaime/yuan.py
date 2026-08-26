#!/usr/bin/env python3
"""
FQM 元解释器 · 一元、二元、三元、四元关系层级

核心概念（来自满全法哲学）：
- 一元 = 时间因果 — 代表目标、红利、受益者（中心不变量）
- 二元 = 盈亏镜像 — 围绕中心展开的辅助变化（可统计、可镜像）
- 三元 = 见证镜像 — 裁判观测者，公约化二元（静态公约化、动态收敛）
- 四元 = 包含镜像 — 面包裹其内/归属权（容积内部再套容积）

操作语法：
  一元(目标)            → 声明一个一元目标
  二元(中心, 辅助)      → 声明中心与辅助的镜像关系
  三元(观测, A, B)      → 声明见证者对A和B的裁判
  四元(外壳, 内容)      → 声明包含关系（外壳包裹内容）
  层级                  → 查看完整层级结构
  裁判(A, B)            → 三元见证：裁定A与B的关系
"""

import os
import sys
import re


class YuanInterpreter:
    """元解释器 - 1-4元关系层级"""
    
    def __init__(self, linker):
        self.linker = linker
        self.ones = []          # 一元声明
        self.twos = []          # 二元声明
        self.threes = []        # 三元声明
        self.fours = []         # 四元声明
    
    def execute(self, command):
        """次元命令"""
        command = command.strip()
        if not command or command.startswith('#'):
            return None
        
        if command.startswith('一元('):
            return self._cmd_one(command)
        
        if command.startswith('二元('):
            return self._cmd_two(command)
        
        if command.startswith('三元('):
            return self._cmd_three(command)
        
        if command.startswith('四元('):
            return self._cmd_four(command)
        
        if command == '层级':
            return self._cmd_hierarchy(command)
        
        if command.startswith('裁判('):
            return self._cmd_judge(command)
        
        if command.startswith('升元('):
            return self._cmd_upgrade(command)
        
        if command.startswith('降元('):
            return self._cmd_downgrade(command)
        
        return f"错误: 未知命令 {command}"
    
    def _cmd_one(self, command):
        """一元: 声明中心目标"""
        match = re.match(r'一元\((.+)\)', command)
        if not match:
            return "错误: 一元的格式"
        
        target = match.group(1).strip()
        self.ones.append(target)
        
        result = f"一元声明:\n"
        result += f"  目标: {target}\n"
        result += f"  性质: 时间因果、中心不变量\n"
        result += f"  角色: 受益者/受益目标"
        
        return result
    
    def _cmd_two(self, command):
        """二元: 声明中心与辅助的镜像关系"""
        match = re.match(r'二元\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 二元的格式"
        
        center = match.group(1).strip()
        auxiliary = match.group(2).strip()
        self.twos.append((center, auxiliary))
        
        result = f"二元声明:\n"
        result += f"  中心: {center}\n"
        result += f"  辅助: {auxiliary}\n"
        result += f"  性质: 盈亏镜像、可统计、可镜像\n"
        result += f"  角色: 手段/被利用者（服务中心目标）"
        
        return result
    
    def _cmd_three(self, command):
        """三元: 声明见证者对A和B的裁判"""
        match = re.match(r'三元\(([^,]+),\s*([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 三元的格式"
        
        observer = match.group(1).strip()
        a = match.group(2).strip()
        b = match.group(3).strip()
        self.threes.append((observer, a, b))
        
        result = f"三元声明:\n"
        result += f"  观测者: {observer}\n"
        result += f"  甲方: {a}\n"
        result += f"  乙方: {b}\n"
        result += f"  性质: 见证镜像、公约化二元\n"
        result += f"  角色: 裁判/观测者（裁定A与B的关系）"
        
        return result
    
    def _cmd_four(self, command):
        """四元: 声明包含关系"""
        match = re.match(r'四元\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 四元的格式"
        
        shell = match.group(1).strip()
        content = match.group(2).strip()
        self.fours.append((shell, content))
        
        result = f"四元声明:\n"
        result += f"  外壳: {shell}\n"
        result += f"  内容: {content}\n"
        result += f"  性质: 包含镜像、归属权\n"
        result += f"  角色: 容积包裹其内（面包裹内容）"
        
        return result
    
    def _cmd_hierarchy(self, command):
        """显示层级结构"""
        result = "元层级结构:\n"
        result += f"{'='*50}\n"
        
        if self.ones:
            result += f"【一元】(时间因果)\n"
            for o in self.ones:
                result += f"  ◆ {o}\n"
            result += f"\n"
        
        if self.twos:
            result += f"【二元】(盈亏镜像)\n"
            for c, a in self.twos:
                result += f"  ◇ 中心:{c} ← 辅助:{a}\n"
            result += f"\n"
        
        if self.threes:
            result += f"【三元】(见证镜像)\n"
            for obs, a, b in self.threes:
                result += f"  ▷ 观测:{obs} 裁定 {a} ? {b}\n"
            result += f"\n"
        
        if self.fours:
            result += f"【四元】(包含镜像)\n"
            for shell, content in self.fours:
                result += f"  □ 外壳:{shell} ⟶ 内容:{content}\n"
            result += f"\n"
        
        if not self.ones and not self.twos and not self.threes and not self.fours:
            result += "  （暂无声明）"
        
        return result.strip()
    
    def _cmd_judge(self, command):
        """裁判: 裁定A与B的关系"""
        match = re.match(r'裁判\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 裁判格式"
        
        a = match.group(1).strip()
        b = match.group(2).strip()
        
        relation = self._find_relation(a, b)
        
        result = f"裁判({a}, {b}):\n"
        result += f"  甲方: {a}\n"
        result += f"  乙方: {b}\n"
        result += f"  关系: {relation}"
        
        return result
    
    def _find_relation(self, a, b):
        """查找两个符号之间的关系"""
        for link in self.linker.resolved:
            from_sym = link['from']['symbol']
            to_sym = link['to']['symbol']
            
            if (from_sym == a and to_sym == b):
                return f"直接连接 {a} → {b}"
            if (from_sym == b and to_sym == a):
                return f"反向连接 {b} → {a}"
        
        domain_a = None
        domain_b = None
        id_a = None
        id_b = None
        
        for (d, s), p in self.linker.symbol_to_path.items():
            if s == a:
                domain_a = d
                id_a = p.identity
            if s == b:
                domain_b = d
                id_b = p.identity
        
        if domain_a and domain_b:
            if domain_a == domain_b:
                if id_a == id_b:
                    return f"同域同位({domain_a}.{id_a})"
                else:
                    return f"同域异位({domain_a}内)"
            else:
                if id_a == id_b:
                    return f"异域同公({id_a})"
                else:
                    return f"异域异公"
        
        return "无直接关系"
    
    def _cmd_upgrade(self, command):
        """升元: 从低元升到高元"""
        match = re.match(r'升元\((.+)\)', command)
        if not match:
            return "错误: 升元格式"
        
        element = match.group(1).strip()
        
        result = f"升元({element}):\n"
        result += f"  元素: {element}\n"
        result += f"  升元路径: 一元 → 二元 → 三元 → 四元\n"
        result += f"  说明: 从中心目标开始，逐步被镜像、被裁判、被包含"
        
        return result
    
    def _cmd_downgrade(self, command):
        """降元: 从高元降到低元"""
        match = re.match(r'降元\((.+)\)', command)
        if not match:
            return "错误: 降元格式"
        
        element = match.group(1).strip()
        
        result = f"降元({element}):\n"
        result += f"  元素: {element}\n"
        result += f"  降元路径: 四元 → 三元 → 二元 → 一元\n"
        result += f"  说明: 从包含外壳逐步回归中心目标"
        
        return result
    
    def display_yuan(self):
        """次元信息"""
        print(f"\n{'='*60}")
        print("元层级")
        print(f"{'-'*60}")
        print(f"一元声明: {len(self.ones)}")
        print(f"二元声明: {len(self.twos)}")
        print(f"三元声明: {len(self.threes)}")
        print(f"四元声明: {len(self.fours)}")
        print(f"{'='*60}\n")


# ============================================================
# 完整管道（最终版）
# ============================================================

class FQMPipeline:
    """FQM 完整管道（含所有解释器）"""
    
    def __init__(self, fqm_file):
        self.fqm_file = fqm_file
        self.loader = None
        self.linker = None
        self.interpreter = None
        self.radix3d = None
        self.dimension = None
        self.divisor = None
        self.yuan = None
    
    def run(self):
        """运行完整管道"""
        from loader import FQMLoader
        from linker import FQMLinker
        from interpreter import FQMInterpreter
        from radix3d import Radix3DInterpreter
        from dimension import DimensionInterpreter
        from divisor import DivisorInterpreter
        
        self.loader = FQMLoader()
        if not self.loader.load_file(self.fqm_file):
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.interpreter = FQMInterpreter(self.linker)
        self.radix3d = Radix3DInterpreter(self.linker)
        self.dimension = DimensionInterpreter(self.linker)
        self.divisor = DivisorInterpreter(self.linker)
        self.yuan = YuanInterpreter(self.linker)
        
        return True
    
    def execute(self, command):
        """执行命令"""
        s = command.strip()
        if s.startswith('一元(') or s.startswith('二元(') or \
           s.startswith('三元(') or s.startswith('四元(') or \
           s == '层级' or s.startswith('裁判(') or \
           s.startswith('升元(') or s.startswith('降元('):
            return self.yuan.execute(command)
        if s.startswith('公约(') or s.startswith('公倍(') or \
           s.startswith('同公(') or s.startswith('齐公(') or \
           s.startswith('关系(') or s == '公约数':
            return self.divisor.execute(command)
        if s.startswith('降维(') or s.startswith('升维(') or \
           s.startswith('投影(') or s.startswith('展开(') or \
           s == '维度':
            return self.dimension.execute(command)
        if s.startswith('三维.'):
            return self.radix3d.execute(command)
        return self.interpreter.execute(command)
    
    def display_all(self):
        """显示全部"""
        self.loader.display_summary()
        self.linker.display_ontology()
        self.yuan.display_yuan()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("=" * 60)
    print("FQM 元解释器测试")
    print("=" * 60)
    
    fqm_file = "containers.fqm"
    
    pipe = FQMPipeline(fqm_file)
    if not pipe.run():
        print("管道启动失败")
        sys.exit(1)
    
    pipe.display_all()
    
    print("\n元命令测试:")
    commands = [
        "一元(囝囝共生)",
        "二元(目标, 声母b)",
        "三元(满全法, 声母.b, 韵母.a)",
        "四元(声母容器, b)",
        "裁判(b, a)",
        "层级",
    ]
    
    for cmd in commands:
        result = pipe.execute(cmd)
        if result:
            print(f"\n  {cmd}")
            for line in result.split('\n'):
                print(f"    {line}")
