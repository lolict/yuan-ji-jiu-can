#!/usr/bin/env python3
"""
FQM 公约数公倍数解释器 · 关系运算

核心概念：
- 公约数：不同容器共享的身份位置（如"首"是声母、韵母、声调共有的）
- 公倍数：不同容器组合产生的新单位（如 声母.首 × 韵母.首 = 一个完整音节）
- 关系运算：判断两个符号是否在同一公约数上，或能否组成公倍数

操作语法：
  公约(声母, 韵母)        → 查找声母和韵母的共有身份
  公倍(声母.首, 韵母.首)  → 计算声母首和韵母首的组合
  同公(b, p)             → 判断b和p是否在同一公约数上
  齐公(声母.首, 韵母.首)  → 判断两个位置是否对齐
"""

import os
import sys
import re


class DivisorInterpreter:
    """公约数公倍数解释器"""
    
    def __init__(self, linker):
        self.linker = linker
        self.domains = {}          # domain_name -> [identities]
        self._build_domain_info()
    
    def _build_domain_info(self):
        """构建域信息"""
        for (domain, symbol), path in self.linker.symbol_to_path.items():
            if domain not in self.domains:
                self.domains[domain] = []
            if path.identity not in self.domains[domain]:
                self.domains[domain].append(path.identity)
    
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
    
    def execute(self, command):
        """执行公约数/公倍数命令"""
        command = command.strip()
        if not command or command.startswith('#'):
            return None
        
        # 公约(声母, 韵母)
        if command.startswith('公约('):
            return self._cmd_common_divisor(command)
        
        # 公倍(声母.首, 韵母.首)
        if command.startswith('公倍('):
            return self._cmd_common_multiple(command)
        
        # 同公(b, p)
        if command.startswith('同公('):
            return self._cmd_same_divisor(command)
        
        # 齐公(声母.首, 韵母.首)
        if command.startswith('齐公('):
            return self._cmd_aligned(command)
        
        # 关系(声母.b, 韵母.a)
        if command.startswith('关系('):
            return self._cmd_relation(command)
        
        # 显示所有公约数
        if command == '公约数':
            return self._cmd_list_divisors()
        
        return f"错误: 未知命令 {command}"
    
    def _cmd_common_divisor(self, command):
        """公约数: 查找两个域共有的身份"""
        match = re.match(r'公约\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 公约格式"
        
        domain1 = match.group(1).strip()
        domain2 = match.group(2).strip()
        
        if domain1 not in self.domains or domain2 not in self.domains:
            return f"错误: 域不存在"
        
        # 找共有身份
        identities1 = set(self.domains[domain1])
        identities2 = set(self.domains[domain2])
        common = identities1 & identities2
        
        if common:
            result = f"公约({domain1}, {domain2}):\n"
            result += f"  共有身份: {', '.join(sorted(common))}\n"
            result += f"  数量: {len(common)}"
        else:
            result = f"公约({domain1}, {domain2}): 无共有身份"
        
        return result
    
    def _cmd_common_multiple(self, command):
        """公倍: 计算两个位置的组合"""
        match = re.match(r'公倍\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 公倍格式"
        
        pos1 = match.group(1).strip()
        pos2 = match.group(2).strip()
        
        # 解析 声母.首 格式
        parts1 = pos1.split('.')
        parts2 = pos2.split('.')
        
        if len(parts1) != 2 or len(parts2) != 2:
            return "错误: 位置格式应为 域.身份"
        
        domain1, id1 = parts1
        domain2, id2 = parts2
        
        sym1 = self._find_symbol(domain1, id1)
        sym2 = self._find_symbol(domain2, id2)
        
        result = f"公倍({pos1}, {pos2}):\n"
        result += f"  {pos1} = {sym1}\n"
        result += f"  {pos2} = {sym2}\n"
        result += f"  组合: ({domain1}.{sym1}, {domain2}.{sym2})"
        
        return result
    
    def _cmd_same_divisor(self, command):
        """同公: 判断两个符号是否在同一公约数上"""
        match = re.match(r'同公\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 同公格式"
        
        sym1 = match.group(1).strip()
        sym2 = match.group(2).strip()
        
        # 查找两个符号的身份
        path1 = None
        path2 = None
        
        for (d, s), p in self.linker.symbol_to_path.items():
            if s == sym1:
                path1 = (d, p.identity)
            if s == sym2:
                path2 = (d, p.identity)
        
        if not path1 or not path2:
            return f"错误: 找不到符号 {sym1} 或 {sym2}"
        
        if path1[1] == path2[1]:
            result = f"同公({sym1}, {sym2}): 是\n"
            result += f"  {sym1} 在 {path1[0]}.{path1[1]}\n"
            result += f"  {sym2} 在 {path2[0]}.{path2[1]}\n"
            result += f"  共享公约数: {path1[1]}"
        else:
            result = f"同公({sym1}, {sym2}): 否\n"
            result += f"  {sym1} 在 {path1[0]}.{path1[1]}\n"
            result += f"  {sym2} 在 {path2[0]}.{path2[1]}"
        
        return result
    
    def _cmd_aligned(self, command):
        """齐公: 判断两个位置是否对齐（同一身份）"""
        match = re.match(r'齐公\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 齐公格式"
        
        pos1 = match.group(1).strip()
        pos2 = match.group(2).strip()
        
        parts1 = pos1.split('.')
        parts2 = pos2.split('.')
        
        if len(parts1) != 2 or len(parts2) != 2:
            return "错误: 位置格式应为 域.身份"
        
        domain1, id1 = parts1
        domain2, id2 = parts2
        
        sym1 = self._find_symbol(domain1, id1)
        sym2 = self._find_symbol(domain2, id2)
        
        if id1 == id2:
            result = f"齐公({pos1}, {pos2}): 对齐\n"
            result += f"  身份: {id1}\n"
            result += f"  {domain1}的{id1} = {sym1}\n"
            result += f"  {domain2}的{id1} = {sym2}"
        else:
            result = f"齐公({pos1}, {pos2}): 不对齐\n"
            result += f"  {pos1} 身份为 {id1}\n"
            result += f"  {pos2} 身份为 {id2}"
        
        return result
    
    def _cmd_relation(self, command):
        """关系: 判断两个符号之间的关系"""
        match = re.match(r'关系\(([^,]+),\s*([^)]+)\)', command)
        if not match:
            return "错误: 关系格式"
        
        sym1 = match.group(1).strip()
        sym2 = match.group(2).strip()
        
        path1 = None
        path2 = None
        
        for (d, s), p in self.linker.symbol_to_path.items():
            if s == sym1:
                path1 = (d, p.identity)
            if s == sym2:
                path2 = (d, p.identity)
        
        if not path1 or not path2:
            return f"错误: 找不到符号"
        
        # 检查是否在同一域
        if path1[0] == path2[0]:
            result = f"关系({sym1}, {sym2}): 同域({path1[0]})\n"
            result += f"  {sym1} 在 {path1[0]}.{path1[1]}\n"
            result += f"  {sym2} 在 {path2[0]}.{path2[1]}"
        elif path1[1] == path2[1]:
            result = f"关系({sym1}, {sym2}): 同公({path1[1]})\n"
            result += f"  {sym1} 在 {path1[0]}.{path1[1]}\n"
            result += f"  {sym2} 在 {path2[0]}.{path2[1]}\n"
            result += f"  共享身份: {path1[1]}"
        else:
            result = f"关系({sym1}, {sym2}): 异域异公\n"
            result += f"  {sym1} 在 {path1[0]}.{path1[1]}\n"
            result += f"  {sym2} 在 {path2[0]}.{path2[1]}"
        
        return result
    
    def _cmd_list_divisors(self):
        """列出所有公约数"""
        # 找所有域共有的身份
        if not self.domains:
            return "无域信息"
        
        all_identities = []
        for domain, identities in self.domains.items():
            all_identities.append(set(identities))
        
        if not all_identities:
            return "无身份信息"
        
        # 所有域的交集
        common = all_identities[0]
        for s in all_identities[1:]:
            common = common & s
        
        result = "公约数（所有域共有）:\n"
        if common:
            result += f"  {', '.join(sorted(common))}\n"
            result += f"  数量: {len(common)}"
        else:
            result += "  无"
        
        # 每个域的独有身份
        result += "\n\n各域身份:\n"
        for domain in sorted(self.domains.keys()):
            result += f"  {domain}: {', '.join(self.domains[domain][:5])}"
            if len(self.domains[domain]) > 5:
                result += f"... (共{len(self.domains[domain])}个)"
            result += "\n"
        
        return result.strip()
    
    def display_divisors(self):
        """显示公约数信息"""
        print(f"\n{'='*60}")
        print("公约数信息")
        print(f"{'-'*60}")
        
        for domain in sorted(self.domains.keys()):
            identities = self.domains[domain]
            print(f"  {domain}: {len(identities)} 个身份")
        
        print(f"{'='*60}\n")


# ============================================================
# 完整管道
# ============================================================

class FQMPipeline:
    """FQM 完整管道 v5（含公约数解释器）"""
    
    def __init__(self, fqm_file):
        self.fqm_file = fqm_file
        self.loader = None
        self.linker = None
        self.interpreter = None
        self.radix3d = None
        self.dimension = None
        self.divisor = None
    
    def run(self):
        """运行完整管道"""
        from loader import FQMLoader
        from linker import FQMLinker
        from interpreter import FQMInterpreter
        from radix3d import Radix3DInterpreter
        from dimension import DimensionInterpreter
        
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
        
        return True
    
    def execute(self, command):
        """执行命令"""
        if command.strip().startswith('公约(') or command.strip().startswith('公倍(') or \
           command.strip().startswith('同公(') or command.strip().startswith('齐公(') or \
           command.strip().startswith('关系(') or command.strip() == '公约数':
            return self.divisor.execute(command)
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
        self.divisor.display_divisors()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("=" * 60)
    print("FQM 公约数公倍数解释器测试")
    print("=" * 60)
    
    fqm_file = "containers.fqm"
    
    pipe = FQMPipeline(fqm_file)
    if not pipe.run():
        print("管道启动失败")
        sys.exit(1)
    
    pipe.display_all()
    
    print("\n公约数命令测试:")
    commands = [
        "公约数",
        "公约(声母, 韵母)",
        "公倍(声母.首, 韵母.首)",
        "同公(b, p)",
        "齐公(声母.首, 韵母.首)",
        "关系(b, a)",
    ]
    
    for cmd in commands:
        result = pipe.execute(cmd)
        if result:
            print(f"\n  {cmd}")
            for line in result.split('\n'):
                print(f"    {line}")
