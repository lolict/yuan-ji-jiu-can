#!/usr/bin/env python3
"""
FQM 链接器 · 按坐标定位符号，解析跨容器引用

核心功能：
- 接收加载器输出的容器和链接
- 按坐标索引定位符号（不是搜索，是索引）
- 解析跨容器引用，生成可执行的地址对
- 检测冲突（重复定义、未定义引用）

这是"自己的工具"的第二步：
加载器负责读，链接器负责连。
"""

import os
import sys
from loader import FQMLoader


class FQMLinker:
    """FQM 链接器"""
    
    def __init__(self, loader):
        self.loader = loader
        self.index = {}           # (container, symbol) -> address
        self.resolved = []        # 已解析的引用
        self.unresolved = []      # 未解析的引用
        self.conflicts = []       # 冲突
    
    def build_index(self):
        """构建坐标索引"""
        self.index = {}
        offset = self.loader.offset
        
        for name, container in self.loader.get_all_containers().items():
            for entry in container.symbols:
                key = (name, entry["symbol"])
                addr = offset + entry["position"]
                if key in self.index:
                    self.conflicts.append(f"重复定义: {name}.{entry['symbol']}")
                self.index[key] = addr
        
        return len(self.conflicts) == 0
    
    def resolve_links(self):
        """解析所有跨容器引用"""
        self.resolved = []
        self.unresolved = []
        
        for from_c, from_s, to_c, to_s in self.loader.get_links():
            from_key = (from_c, from_s)
            to_key = (to_c, to_s)
            
            from_addr = self.index.get(from_key)
            to_addr = self.index.get(to_key)
            
            if from_addr is None:
                self.unresolved.append(f"未定义: {from_c}.{from_s}")
                continue
            
            if to_addr is None:
                self.unresolved.append(f"未定义: {to_c}.{to_s}")
                continue
            
            self.resolved.append({
                "from": {"container": from_c, "symbol": from_s, "address": from_addr},
                "to": {"container": to_c, "symbol": to_s, "address": to_addr},
                "link": f"{from_c}.{from_s}(0x{from_addr:02X}) → {to_c}.{to_s}(0x{to_addr:02X})"
            })
        
        return len(self.unresolved) == 0
    
    def lookup(self, container, symbol):
        """按坐标查找符号地址"""
        return self.index.get((container, symbol))
    
    def get_index(self):
        """获取完整索引"""
        return self.index
    
    def display_index(self):
        """显示索引表"""
        print(f"\n{'='*60}")
        print("坐标索引表")
        print(f"{'-'*60}")
        
        # 按容器分组
        containers = {}
        for (container, symbol), addr in self.index.items():
            if container not in containers:
                containers[container] = []
            containers[container].append((symbol, addr))
        
        for container in sorted(containers.keys()):
            symbols = containers[container]
            print(f"\n  [{container}]")
            for symbol, addr in sorted(symbols, key=lambda x: x[1]):
                print(f"    0x{addr:02X}  {symbol}")
        
        print(f"\n{'='*60}\n")
    
    def display_resolution(self):
        """显示解析结果"""
        print(f"\n{'='*60}")
        print("链接解析结果")
        print(f"{'-'*60}")
        print(f"已解析: {len(self.resolved)}")
        print(f"未解析: {len(self.unresolved)}")
        print(f"冲突: {len(self.conflicts)}")
        
        if self.resolved:
            print(f"\n已解析链接:")
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
    linker.build_index()
    linker.resolve_links()
    
    linker.display_index()
    linker.display_resolution()
