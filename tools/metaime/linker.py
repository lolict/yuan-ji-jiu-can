#!/usr/bin/env python3
"""
FQM 链接器 · 本体论量化版

核心改变：不用代数地址，用本体论路径。

旧方式（代数）:
  addr = base + offset → 0x00, 0x01, 0x02
  
新方式（本体论）:
  位置 = 域.身份 → 声母.首, 声母.次, 韵母.首

符号的"地址"不是数字，是它在 ontology 树中的身份路径。
链接不是地址映射，是本质与本质之间的连接。
"""

import os
import sys
from loader import FQMLoader


class OntologyPath:
    """本体论路径 - 替代数字地址"""
    
    def __init__(self, domain, identity):
        self.domain = domain        # 声母/韵母/声调
        self.identity = identity    # 首/次/末 或 具体符号
    
    def __str__(self):
        return f"{self.domain}.{self.identity}"
    
    def __repr__(self):
        return f"OntologyPath({self.domain}.{self.identity})"
    
    def __eq__(self, other):
        return self.domain == other.domain and self.identity == other.identity
    
    def __hash__(self):
        return hash((self.domain, self.identity))


class FQMLinker:
    """FQM 链接器 - 本体论量化版"""
    
    def __init__(self, loader):
        self.loader = loader
        self.ontology_index = {}    # OntologyPath -> symbol
        self.symbol_to_path = {}    # (domain, symbol) -> OntologyPath
        self.resolved = []          # 已解析的链接
        self.unresolved = []        # 未解析的链接
        self.conflicts = []         # 冲突
    
    def build_ontology_index(self):
        """构建本体论索引 - 按身份而非数字"""
        self.ontology_index = {}
        self.symbol_to_path = {}
        
        identity_names = ["首", "次", "三", "四", "五", "六", "七", "八", "九", "十",
                        "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
                        "二十一", "二十二", "二十三", "二十四"]
        
        for domain_name, container in self.loader.get_all_containers().items():
            for idx, entry in enumerate(container.symbols):
                symbol = entry["symbol"]
                # 身份 = 首/次/三... 而非 0/1/2...
                identity = identity_names[idx] if idx < len(identity_names) else f"第{idx+1}"
                
                path = OntologyPath(domain_name, identity)
                
                key = (domain_name, symbol)
                if key in self.symbol_to_path:
                    self.conflicts.append(f"重复定义: {domain_name}.{symbol}")
                
                self.ontology_index[path] = symbol
                self.symbol_to_path[key] = path
        
        return len(self.conflicts) == 0
    
    def resolve_links(self):
        """解析跨容器链接 - 本质对本质"""
        self.resolved = []
        self.unresolved = []
        
        for from_c, from_s, to_c, to_s in self.loader.get_links():
            from_key = (from_c, from_s)
            to_key = (to_c, to_s)
            
            from_path = self.symbol_to_path.get(from_key)
            to_path = self.symbol_to_path.get(to_key)
            
            if from_path is None:
                self.unresolved.append(f"未定义: {from_c}.{from_s}")
                continue
            
            if to_path is None:
                self.unresolved.append(f"未定义: {to_c}.{to_s}")
                continue
            
            self.resolved.append({
                "from": {"domain": from_c, "symbol": from_s, "path": from_path},
                "to": {"domain": to_c, "symbol": to_s, "path": to_path},
                "link": f"{from_path} → {to_path}"
            })
        
        return len(self.unresolved) == 0
    
    def lookup(self, domain, symbol):
        """按域和符号查找本体论路径"""
        return self.symbol_to_path.get((domain, symbol))
    
    def get_ontology_index(self):
        """获取完整本体论索引"""
        return self.ontology_index
    
    def display_ontology(self):
        """显示本体论树"""
        print(f"\n{'='*60}")
        print("本体论树")
        print(f"{'-'*60}")
        
        # 按域分组
        domains = {}
        for path, symbol in self.ontology_index.items():
            if path.domain not in domains:
                domains[path.domain] = []
            domains[path.domain].append((path.identity, symbol))
        
        for domain in sorted(domains.keys()):
            entries = domains[domain]
            print(f"\n  ┌─ {domain}")
            for identity, symbol in sorted(entries, key=lambda x: x[0]):
                print(f"  │  {identity}: {symbol}")
            print(f"  └─")
        
        print(f"\n{'='*60}\n")
    
    def display_resolution(self):
        """显示链接解析结果"""
        print(f"\n{'='*60}")
        print("本体论链接")
        print(f"{'-'*60}")
        print(f"已解析: {len(self.resolved)}")
        print(f"未解析: {len(self.unresolved)}")
        print(f"冲突: {len(self.conflicts)}")
        
        if self.resolved:
            print(f"\n本质连接:")
            for r in self.resolved:
                print(f"  {r['link']}")
        
        if self.unresolved:
            print(f"\n未解析:")
            for u in self.unresolved:
                print(f"  ! {u}")
        
        if self.conflicts:
            print(f"\n冲突:")
            for c in self.conflicts:
                print(f"  ✗ {c}")
        
        print(f"{'='*60}\n")


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("用法: python linker.py <fqm文件>")
        sys.exit(1)
    
    # 加载
    loader = FQMLoader()
    if not loader.load_file(sys.argv[1]):
        print("加载失败")
        sys.exit(1)
    
    loader.display_summary()
    
    # 链接
    linker = FQMLinker(loader)
    linker.build_ontology_index()
    linker.resolve_links()
    
    linker.display_ontology()
    linker.display_resolution()
