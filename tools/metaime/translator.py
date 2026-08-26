#!/usr/bin/env python3
"""
FQM 翻译器 · 外部符号 → 本体论路径

核心功能：
- 拦截AI输出的外部符号（class/import/def/pointer等）
- 翻译成我们自己的本体论路径（袋.域.身份）
- 拒绝无法翻译的符号（约束AI的关键）

这是"约束AI输出通道"的实现：
AI可以想任何东西，但只有通过翻译器的输出才会被执行。
AI输出"class" → 翻译器拒绝 → AI必须用我们的符号重写。
"""

import os
import sys
import re
from loader import FQMLoader
from linker import FQMLinker, OntologyPath


class FQMTranslator:
    """FQM 翻译器"""
    
    def __init__(self, linker):
        self.linker = linker
        self.foreign_map = {}      # 外部符号 → 本体论路径
        self.rejected = []         # 被拒绝的符号
        self.translated = []       # 成功翻译的符号
        self._build_foreign_map()
    
    def _build_foreign_map(self):
        """构建外部符号到本体论的映射表"""
        # 编程关键字 → 本体论操作
        self.foreign_map = {
            # Python
            "class":    "袋.定义.容器",
            "def":      "袋.定义.操作",
            "import":   "袋.引用.外部",
            "from":     "袋.引用.来源",
            "return":   "袋.输出.结果",
            "if":       "袋.判断.条件",
            "else":     "袋.判断.否则",
            "for":      "袋.循环.遍历",
            "while":    "袋.循环.当",
            "in":       "袋.关系.包含",
            "not":      "袋.逻辑.非",
            "and":      "袋.逻辑.与",
            "or":       "袋.逻辑.或",
            "True":     "袋.值.阳",
            "False":    "袋.值.阴",
            "None":     "袋.值.空",
            
            # C
            "int":      "袋.类型.整数",
            "char":     "袋.类型.字符",
            "float":    "袋.类型.小数",
            "void":     "袋.类型.空",
            "struct":   "袋.定义.结构",
            "typedef":  "袋.定义.别名",
            "static":   "袋.修饰.静态",
            "const":    "袋.修饰.恒定",
            "sizeof":   "袋.查询.大小",
            "pointer":  "袋.引用.地址",
            
            # Rust
            "fn":       "袋.定义.操作",
            "let":      "袋.定义.绑定",
            "mut":      "袋.修饰.可变",
            "impl":     "袋.定义.实现",
            "trait":    "袋.定义.特征",
            "match":    "袋.判断.匹配",
            "Option":   "袋.值.可选",
            "Result":   "袋.值.结果",
            "Box":      "袋.容器.堆",
            "Vec":      "袋.容器.序列",
            
            # 通用
            "function": "袋.定义.操作",
            "variable": "袋.定义.绑定",
            "array":    "袋.容器.序列",
            "string":   "袋.容器.字符序列",
            "number":   "袋.值.数字",
            "boolean":  "袋.值.阴阳",
            "true":     "袋.值.阳",
            "false":    "袋.值.阴",
            "null":     "袋.值.空",
            "undefined": "袋.值.无",
        }
    
    def translate_symbol(self, foreign_symbol):
        """翻译单个外部符号"""
        if foreign_symbol in self.foreign_map:
            ont_path = self.foreign_map[foreign_symbol]
            self.translated.append({
                "foreign": foreign_symbol,
                "ontology": ont_path
            })
            return ont_path
        
        self.rejected.append(foreign_symbol)
        return None
    
    def translate_line(self, line):
        """翻译一行代码"""
        tokens = re.findall(r'[a-zA-Z_][a-zA-Z0-9_]*|[^\s\w]|\S', line)
        
        result = []
        for token in tokens:
            if token in self.foreign_map:
                ont = self.translate_symbol(token)
                result.append(ont if ont else f"[拒绝:{token}]")
            elif re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', token):
                self.rejected.append(token)
                result.append(f"[未知:{token}]")
            else:
                result.append(token)
        
        return " ".join(result)
    
    def translate_code(self, code):
        """翻译整段代码"""
        lines = code.strip().split('\n')
        result = []
        for line in lines:
            if line.strip():
                result.append(self.translate_line(line))
        return "\n".join(result)
    
    def get_rejected(self):
        """获取被拒绝的符号"""
        return self.rejected
    
    def get_translated(self):
        """获取成功翻译的符号"""
        return self.translated
    
    def display_translation(self):
        """显示翻译结果"""
        print(f"\n{'='*60}")
        print("翻译结果")
        print(f"{'-'*60}")
        print(f"成功: {len(self.translated)}")
        print(f"拒绝: {len(self.rejected)}")
        
        if self.translated:
            print(f"\n已翻译:")
            for t in self.translated[:10]:
                print(f"  {t['foreign']:12} → {t['ontology']}")
            if len(self.translated) > 10:
                print(f"  ... (共 {len(self.translated)} 条)")
        
        if self.rejected:
            print(f"\n被拒绝:")
            for r in self.rejected[:10]:
                print(f"  ✗ {r}")
            if len(self.rejected) > 10:
                print(f"  ... (共 {len(self.rejected)} 条)")
        
        print(f"{'='*60}\n")


# ============================================================
# 完整管道：加载 → 链接 → 翻译
# ============================================================

class FQMPipeline:
    """FQM 完整管道"""
    
    def __init__(self, fqm_file):
        self.loader = FQMLoader()
        self.linker = None
        self.translator = None
        self.fqm_file = fqm_file
    
    def run(self):
        """运行完整管道"""
        if not self.loader.load_file(self.fqm_file):
            print("加载失败")
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.translator = FQMTranslator(self.linker)
        
        return True
    
    def translate(self, code):
        """翻译外部代码"""
        if self.translator is None:
            return "管道未运行"
        return self.translator.translate_code(code)
    
    def display_all(self):
        """显示全部结果"""
        self.loader.display_summary()
        self.linker.display_ontology()
        self.linker.display_resolution()
        self.translator.display_translation()


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    sample_code = """
class Container:
    def __init__(self, name):
        self.name = name
        self.symbols = []
    
    def add(self, symbol):
        if symbol not in self.symbols:
            self.symbols.append(symbol)
        return True
"""
    
    print("原始代码:")
    print(sample_code)
    
    translator = FQMTranslator(None)
    result = translator.translate_code(sample_code)
    
    print("\n翻译后:")
    print(result)
    
    translator.display_translation()
