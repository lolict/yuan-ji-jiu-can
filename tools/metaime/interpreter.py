#!/usr/bin/env python3
"""
FQM 解释器 · 本体论执行

核心功能：
- 执行容器操作：装入、取出、比较
- 所有操作使用本体论路径（袋.域.身份），非代数地址
- 支持跨容器链接查询

操作语法：
  袋.声母              → 查询声母容器所有符号
  袋.声母.装入(b)      → 把b装入声母容器
  袋.声母.取出(首)     → 取出声母容器首个符号
  袋.声母.比较(b, p)   → 比较两个符号是否相同
  袋.声母.引.韵母(首)  → 查询声母首个符号链接的韵母符号
"""

import os
import sys
import re


class FQMInterpreter:
    """FQM 解释器 - 本体论执行"""
    
    def __init__(self, linker):
        self.linker = linker
        self.memory = {}
        self.output = []
        self._init_memory()
    
    def _init_memory(self):
        """从链接器初始化内存"""
        for (domain, symbol), path in self.linker.symbol_to_path.items():
            self.memory[str(path)] = symbol
    
    def execute(self, command):
        """执行一条本体论命令"""
        command = command.strip()
        if not command or command.startswith('#'):
            return None
        
        # 链接查询: 袋.声母.引.韵母(首)
        if '引' in command and '(' in command:
            return self._op_link(command)
        
        # 带参数操作: 袋.声母.装入(b)
        if '(' in command:
            return self._handle_operation(command)
        
        # 无参数查询: 袋.声母 或 袋.声母.首
        if re.match(r'^袋\.[^.]+(?:\.[^.]+)?$', command):
            return self._handle_query(command)
        
        return f"错误: 无效命令 {command}"
    
    def _handle_query(self, command):
        """处理无参数查询: 袋.声母 或 袋.声母.首"""
        parts = command.split('.')
        
        if len(parts) == 2:
            # 袋.声母 - 列出容器内所有符号
            domain = parts[1]
            container = self.linker.loader.get_container(domain)
            if not container:
                return f"错误: 容器 {domain} 不存在"
            symbols = [e["symbol"] for e in container.symbols]
            result = f"袋.{domain} = {symbols}"
            self.output.append(result)
            return result
        
        elif len(parts) == 3:
            # 袋.声母.首 - 查询具体位置的符号
            domain = parts[1]
            identity = parts[2]
            
            for (d, s), p in self.linker.symbol_to_path.items():
                if p.domain == domain and p.identity == identity:
                    result = f"{command} = {s}"
                    self.output.append(result)
                    return result
            
            return f"错误: {command} 为空"
        
        return f"错误: 路径格式 {command}"
    
    def _handle_operation(self, command):
        """处理带参数的操作"""
        match = re.match(r'(袋\.[^.]+\.[^.]+)\((.+)\)', command)
        if not match:
            return f"错误: 无法解析 {command}"
        
        path_str = match.group(1)
        args_str = match.group(2)
        args = [a.strip() for a in args_str.split(',')]
        
        parts = path_str.split('.')
        domain = parts[1]
        op = parts[2]
        
        if op == '装入':
            return self._op_load(domain, args[0])
        elif op == '取出':
            return self._op_extract(domain, args[0])
        elif op == '比较':
            return self._op_compare(domain, args[0], args[1])
        else:
            return f"错误: 未知操作 {op}"
    
    def _op_load(self, domain, symbol):
        """操作: 袋.声母.装入(b)"""
        existing = self.linker.lookup(domain, symbol)
        if existing:
            return f"{symbol} 已在 {existing}"
        
        container = self.linker.loader.get_container(domain)
        if not container:
            return f"错误: 容器 {domain} 不存在"
        
        container.add(symbol)
        self.linker.build_ontology_index()
        path = self.linker.lookup(domain, symbol)
        
        result = f"装入成功: {symbol} → {path}"
        self.output.append(result)
        return result
    
    def _op_extract(self, domain, identity):
        """操作: 袋.声母.取出(首)"""
        for (d, s), p in self.linker.symbol_to_path.items():
            if p.domain == domain and p.identity == identity:
                result = f"取出: {domain}.{identity} = {s}"
                self.output.append(result)
                return result
        
        return f"错误: {domain}.{identity} 为空"
    
    def _op_compare(self, domain, sym1, sym2):
        """操作: 袋.声母.比较(b, p)"""
        path1 = self.linker.lookup(domain, sym1)
        path2 = self.linker.lookup(domain, sym2)
        
        if path1 is None:
            return f"错误: {sym1} 不在 {domain} 容器中"
        if path2 is None:
            return f"错误: {sym2} 不在 {domain} 容器中"
        
        if path1 == path2:
            result = f"比较: {sym1} == {sym2}（同一位置 {path1}）"
        else:
            result = f"比较: {sym1}({path1}) ≠ {sym2}({path2})"
        
        self.output.append(result)
        return result
    
    def _op_link(self, command):
        """操作: 袋.声母.引.韵母(首) - 查询链接"""
        # 解析: 袋.声母.引.韵母(首)
        match = re.match(r'袋\.([^.]+)\.引\.([^.]+)\((.+)\)', command)
        if not match:
            return f"错误: 链接格式 {command}"
        
        from_domain = match.group(1)
        to_domain = match.group(2)
        identity = match.group(3).strip()
        
        from_path = f"{from_domain}.{identity}"
        
        for link in self.linker.resolved:
            if str(link['from']['path']) == from_path:
                to_sym = link['to']['symbol']
                to_path = link['to']['path']
                result = f"链接: {from_path} → {to_path}（{to_sym}）"
                self.output.append(result)
                return result
        
        return f"错误: {from_path} 无链接"
    
    def execute_batch(self, commands):
        """执行多条命令"""
        results = []
        for cmd in commands:
            result = self.execute(cmd)
            if result:
                results.append(result)
        return results
    
    def display_memory(self):
        """显示当前内存状态"""
        print(f"\n{'='*60}")
        print("内存状态（本体论路径 → 符号）")
        print(f"{'-'*60}")
        
        domains = {}
        for path_str, symbol in self.memory.items():
            parts = path_str.split('.')
            domain = parts[1] if len(parts) > 1 else parts[0]
            if domain not in domains:
                domains[domain] = []
            domains[domain].append((path_str, symbol))
        
        for domain in sorted(domains.keys()):
            entries = domains[domain]
            print(f"  [{domain}]")
            for path, symbol in sorted(entries):
                print(f"    {path} → {symbol}")
        
        print(f"{'='*60}\n")
    
    def display_output(self):
        """显示执行输出"""
        print(f"\n{'='*60}")
        print("执行记录")
        print(f"{'-'*60}")
        for i, line in enumerate(self.output, 1):
            print(f"  {i}. {line}")
        print(f"{'='*60}\n")


# ============================================================
# 完整管道
# ============================================================

class FQMPipeline:
    """FQM 完整管道"""
    
    def __init__(self, fqm_file):
        self.fqm_file = fqm_file
        self.loader = None
        self.linker = None
        self.interpreter = None
    
    def run(self):
        """运行完整管道"""
        from loader import FQMLoader
        from linker import FQMLinker
        
        self.loader = FQMLoader()
        if not self.loader.load_file(self.fqm_file):
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.interpreter = FQMInterpreter(self.linker)
        
        return True
    
    def execute(self, command):
        """执行命令"""
        return self.interpreter.execute(command)
    
    def display_all(self):
        """显示全部"""
        self.loader.display_summary()
        self.linker.display_ontology()
        self.linker.display_resolution()
        self.interpreter.display_memory()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    print("=" * 60)
    print("FQM 完整工具链测试")
    print("=" * 60)
    
    fqm_file = "containers.fqm"
    if not os.path.exists(fqm_file):
        print(f"创建测试文件: {fqm_file}")
        with open(fqm_file, 'w') as f:
            f.write("""# 测试容器
袋.声母.容=b,p,m,f
袋.声母.进制=4
袋.声母.极性=1

袋.韵母.容=a,o,e,i
袋.韵母.进制=4
袋.韵母.极性=0

袋.声母.引.韵母=b→a,p→o
""")
    
    pipe = FQMPipeline(fqm_file)
    if not pipe.run():
        print("管道启动失败")
        sys.exit(1)
    
    pipe.display_all()
    
    print("\n执行测试:")
    commands = [
        "袋.声母",
        "袋.声母.首",
        "袋.声母.取出(首)",
        "袋.声母.取出(次)",
        "袋.韵母.取出(首)",
        "袋.声母.引.韵母(首)",
        "袋.声母.比较(b, p)",
    ]
    
    for cmd in commands:
        result = pipe.execute(cmd)
        if result:
            print(f"  {cmd}")
            print(f"    → {result}")
