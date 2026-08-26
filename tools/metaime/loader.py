#!/usr/bin/env python3
"""
FQM 加载器 · 读取我们自己的文件格式

功能：
- 读取 .fqm 文件（容器定义）
- 读取 .fql 文件（链接关系）
- 读取 .fqn 文件（近义词表）
- 输出结构化的容器数据供解释器使用

这是"自己的工具"的第一步：
不依赖 Python 的 import 或 JSON 的 parse，
而是用我们自己的语法来定义和加载。
"""

import os
import sys
from interpreter import Container, GLOBAL_OFFSET


class FQMLoader:
    """FQM 文件加载器"""
    
    def __init__(self):
        self.containers = {}      # name -> Container
        self.links = []           # (from_container, from_sym, to_container, to_sym)
        self.synonyms = {}        # standard -> [variants]
        self.offset = 0x00
        self.errors = []
    
    def load_file(self, filepath):
        """加载一个 FQM 文件"""
        if not os.path.exists(filepath):
            self.errors.append(f"文件不存在: {filepath}")
            return False
        
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        for line_num, line in enumerate(lines, 1):
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            self._parse_line(line, filepath, line_num)
        
        return len(self.errors) == 0
    
    def load_string(self, content):
        """从字符串加载 FQM 定义"""
        for line_num, line in enumerate(content.split('\n'), 1):
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            self._parse_line(line, "<string>", line_num)
        
        return len(self.errors) == 0
    
    def _parse_line(self, line, filepath, line_num):
        """解析一行声明"""
        if '=' not in line:
            self.errors.append(f"{filepath}:{line_num} - 缺少 '=' 分隔符: {line}")
            return
        
        left, right = line.split('=', 1)
        left = left.strip()
        right = right.strip()
        
        # 全局偏移
        if left == '偏':
            self._parse_offset(right, filepath, line_num)
            return
        
        # 近义词定义
        if left.startswith('近.'):
            self._parse_synonym(left, right, filepath, line_num)
            return
        
        # 袋.容器名.属性=值
        if left.startswith('袋.'):
            self._parse_container_line(left, right, filepath, line_num)
            return
        
        self.errors.append(f"{filepath}:{line_num} - 未知声明类型: {left}")
    
    def _parse_offset(self, value, filepath, line_num):
        """解析全局偏移量"""
        try:
            if value.startswith('0x') or value.startswith('0X'):
                self.offset = int(value, 16)
            else:
                self.offset = int(value)
        except ValueError:
            self.errors.append(f"{filepath}:{line_num} - 无效的偏移量: {value}")
    
    def _parse_synonym(self, left, right, filepath, line_num):
        """解析近义词: 近.标准=变体1,变体2"""
        standard = left[2:]  # 去掉 "近."
        variants = [v.strip() for v in right.split(',') if v.strip()]
        self.synonyms[standard] = variants
    
    def _parse_container_line(self, left, right, filepath, line_num):
        """解析容器定义行"""
        # 袋.声母.容=b,p,m,f
        # 袋.声母.进制=23
        # 袋.声母.引.韵母=b→a
        
        parts = left.split('.')
        if len(parts) < 3:
            self.errors.append(f"{filepath}:{line_num} - 格式错误: {left}")
            return
        
        container_name = parts[1]  # 声母
        attribute = parts[2]       # 容/进制/极性/引
        
        if attribute == '容':
            # 容器内容
            symbols = [s.strip() for s in right.split(',') if s.strip()]
            if container_name not in self.containers:
                self.containers[container_name] = Container(container_name, container_name)
            for sym in symbols:
                self.containers[container_name].add(sym)
        
        elif attribute == '进制':
            if container_name not in self.containers:
                self.containers[container_name] = Container(container_name, container_name)
            try:
                self.containers[container_name].capacity = int(right)
            except ValueError:
                self.errors.append(f"{filepath}:{line_num} - 无效进制: {right}")
        
        elif attribute == '极性':
            if container_name not in self.containers:
                self.containers[container_name] = Container(container_name, container_name)
            # 极性存储在容器类型字段中
            self.containers[container_name].container_type = f"{container_name}(极性={right})"
        
        elif attribute == '引':
            # 跨容器引用: 袋.声母.引.韵母=b→a,p→o,m→e
            if len(parts) < 4:
                self.errors.append(f"{filepath}:{line_num} - 引用格式错误: {left}")
                return
            target_container = parts[3]  # 韵母
            # 支持一行多个链接，用逗号分隔
            pairs = right.split(',')
            for pair in pairs:
                pair = pair.strip()
                if '→' in pair:
                    from_sym, to_sym = pair.split('→', 1)
                    self.links.append((container_name, from_sym.strip(), target_container, to_sym.strip()))
                elif pair:
                    self.errors.append(f"{filepath}:{line_num} - 引用缺少 →: {pair}")
    
    def get_container(self, name):
        """获取容器"""
        return self.containers.get(name)
    
    def get_all_containers(self):
        """获取所有容器"""
        return self.containers
    
    def get_links(self):
        """获取所有链接"""
        return self.links
    
    def get_synonyms(self):
        """获取所有近义词"""
        return self.synonyms
    
    def apply_offset(self, offset=None):
        """应用全局偏移量"""
        if offset is not None:
            self.offset = offset
    
    def display_summary(self):
        """显示加载结果摘要"""
        print(f"\n{'='*60}")
        print("FQM 加载结果")
        print(f"{'-'*60}")
        print(f"容器数量: {len(self.containers)}")
        print(f"链接数量: {len(self.links)}")
        print(f"近义词数量: {len(self.synonyms)}")
        print(f"全局偏移: 0x{self.offset:02X}")
        
        if self.containers:
            print(f"\n容器列表:")
            for name, container in self.containers.items():
                print(f"  {name}: {len(container.symbols)} 个符号")
        
        if self.links:
            print(f"\n链接列表:")
            for from_c, from_s, to_c, to_s in self.links[:10]:
                print(f"  {from_c}.{from_s} → {to_c}.{to_s}")
            if len(self.links) > 10:
                print(f"  ... (共 {len(self.links)} 条)")
        
        if self.errors:
            print(f"\n错误 ({len(self.errors)}):")
            for err in self.errors[:5]:
                print(f"  ! {err}")
        
        print(f"{'='*60}\n")


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("用法: python loader.py <fqm文件>")
        print("示例: python loader.py containers.fqm")
        sys.exit(1)
    
    loader = FQMLoader()
    success = loader.load_file(sys.argv[1])
    loader.display_summary()
    
    if not success:
        print("加载失败，请检查错误")
        sys.exit(1)
