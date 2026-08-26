#!/usr/bin/env python3
"""
bootstrap_compiler.py — 自举编译器

核心哲学:
  用自己的规则从文件系统长出工具 · 避免被他人规则绑架

功能:
  1. 解析 C 头文件中的函数签名、常量、枚举、结构体
  2. 自动生成 Python ctypes 绑定 (替代手写的 _native.py 部分代码)
  3. 自动生成文档字符串
  4. 检测头文件与绑定之间的漂移

这是"自举"的第一层: 系统可以从自己的 C 定义重新生成自己的 Python 接口 ·
不需要人工维护绑定代码 · 消除接口漂移

用法:
  python3 bootstrap_compiler.py --parse <header.h>         — 解析并显示
  python3 bootstrap_compiler.py --generate <header.h>      — 生成绑定
  python3 bootstrap_compiler.py --check                     — 漂移检测
  python3 bootstrap_compiler.py --bootstrap                 — 全量重建
"""

import os
import re
import sys
import json
import hashlib
from pathlib import Path
from typing import List, Dict, Optional, Tuple, Any

# ── 路径 ──────────────────────────────────────────────────

TOOLS_DIR = Path(__file__).parent.resolve()
# tools/phase-radix/ → ../../encoding = hunyuan-os/encoding
ENCODING_DIR = TOOLS_DIR.parent.parent / "encoding"
NATIVE_PY = TOOLS_DIR / "phase_radix" / "_native.py"

# ── C 语法解析器 ───────────────────────────────────────────

class CSymbol:
    """解析出的 C 符号"""
    def __init__(self, name: str, kind: str, raw: str, value: str = ""):
        self.name = name
        self.kind = kind      # 'function', 'constant', 'enum', 'struct', 'typedef', 'macro'
        self.raw = raw        # 原始文本
        self.value = value    # 常数值/类型定义
        self.doc = ""         # 上方注释

class CHeaderParser:
    """轻量级 C 头文件解析器 · 专为 NJSG 编码系统设计"""
    
    def __init__(self, header_path: Path):
        self.path = header_path
        self.text = header_path.read_text(encoding='utf-8', errors='replace')
        self.symbols: List[CSymbol] = []
        self._parse()
    
    def _parse(self):
        """主解析流程"""
        text = self.text
        
        # 移除块注释 (保留文档)
        # 先提取文档-符号关联
        doc_chunks = self._extract_documented_chunks(text)
        
        for chunk, doc in doc_chunks:
            symbols = self._parse_chunk(chunk)
            for sym in symbols:
                sym.doc = doc
            self.symbols.extend(symbols)
    
    def _extract_documented_chunks(self, text: str) -> List[Tuple[str, str]]:
        """提取带文档注释的代码块"""
        chunks = []
        # 匹配 /** ... */ 后跟代码
        pattern = r'/\*\*(.*?)\*/\s*\n([^/\n][^\n]*(?:\n(?!/$)[^\n]*)*)'
        for m in re.finditer(pattern, text, re.DOTALL):
            doc = m.group(1).strip()
            code = m.group(2).strip()
            chunks.append((code, doc))
        
        # 也捕获无文档的顶层定义
        # 函数: 返回类型 名称(参数);
        # 常量: #define 名称 值
        # 枚举: typedef enum { ... } 名称;
        return chunks
    
    def _parse_chunk(self, chunk: str) -> List[CSymbol]:
        """解析单个代码块"""
        symbols = []
        
        # 函数声明: 返回类型 名称(参数列表);
        for m in re.finditer(
            r'(NJSG_EXPORT\s+)?([\w\s\*]+?)\s+(\w+)\s*\(([^)]*)\)\s*;',
            chunk
        ):
            ret_type = m.group(2).strip()
            func_name = m.group(3).strip()
            params = m.group(4).strip()
            if ret_type and func_name not in ('typedef', 'struct', 'enum', 'union'):
                symbols.append(CSymbol(func_name, 'function', m.group(0), ret_type))
        
        # #define 常量
        for m in re.finditer(r'#define\s+(\w+)\s+(\S+)', chunk):
            symbols.append(CSymbol(m.group(1), 'constant', m.group(0), m.group(2)))
        
        # typedef enum
        for m in re.finditer(r'typedef\s+enum\s*\{([^}]*)\}\s*(\w+)\s*;', chunk):
            enum_name = m.group(2)
            symbols.append(CSymbol(enum_name, 'enum', m.group(0), m.group(1)))
        
        # typedef struct (前向声明)
        for m in re.finditer(r'typedef\s+struct\s*\{([^}]*)\}\s*(\w+)\s*;', chunk):
            struct_name = m.group(2)
            symbols.append(CSymbol(struct_name, 'struct', m.group(0), m.group(1)))
        
        return symbols
    
    def functions(self) -> List[CSymbol]:
        return [s for s in self.symbols if s.kind == 'function']
    
    def constants(self) -> List[CSymbol]:
        return [s for s in self.symbols if s.kind == 'constant']
    
    def enums(self) -> List[CSymbol]:
        return [s for s in self.symbols if s.kind == 'enum']
    
    def structs(self) -> List[CSymbol]:
        return [s for s in self.symbols if s.kind == 'struct']


# ── 绑定生成器 ─────────────────────────────────────────────

class BindingGenerator:
    """从解析的 C 符号生成 Python ctypes 绑定"""
    
    # C 类型 → ctypes 类型映射
    C_TO_CTYPES = {
        'void': 'None',
        'int': 'ctypes.c_int',
        'unsigned int': 'ctypes.c_uint',
        'uint8_t': 'ctypes.c_uint8',
        'uint16_t': 'ctypes.c_uint16',
        'uint32_t': 'ctypes.c_uint32',
        'uint64_t': 'ctypes.c_uint64',
        'int8_t': 'ctypes.c_int8',
        'int16_t': 'ctypes.c_int16',
        'int32_t': 'ctypes.c_int32',
        'int64_t': 'ctypes.c_int64',
        'size_t': 'ctypes.c_size_t',
        'bool': 'ctypes.c_bool',
        'char': 'ctypes.c_char',
        'char*': 'ctypes.c_char_p',
        'const char*': 'ctypes.c_char_p',
        'float': 'ctypes.c_float',
        'double': 'ctypes.c_double',
    }
    
    def __init__(self, parser: CHeaderParser):
        self.parser = parser
        self.lines: List[str] = []
    
    def generate(self) -> str:
        """生成完整绑定代码"""
        self.lines = []
        self._header()
        self._constants()
        self._function_signatures()
        return '\n'.join(self.lines)
    
    def _header(self):
        self.lines.append(f'"""')
        self.lines.append(f'Auto-generated bindings for {self.parser.path.name}')
        self.lines.append(f'Generated by bootstrap_compiler.py — DO NOT EDIT BY HAND')
        self.lines.append(f'"""')
        self.lines.append('')
        self.lines.append('import ctypes')
        self.lines.append('')
    
    def _constants(self):
        consts = self.parser.constants()
        if not consts:
            return
        self.lines.append('# ── Constants ──')
        for sym in consts:
            val = self._translate_value(sym.value)
            self.lines.append(f'{sym.name} = {val}  # {sym.raw[:60]}')
        self.lines.append('')
    
    def _function_signatures(self):
        funcs = self.parser.functions()
        if not funcs:
            return
        self.lines.append('# ── Function Signatures ──')
        for sym in funcs:
            ctypes_ret = self._ctype_for(sym.value)
            self.lines.append(f'_lib.{sym.name}.argtypes = [...]  # TODO: parse params')
            self.lines.append(f'_lib.{sym.name}.restype = {ctypes_ret}')
            if sym.doc:
                self.lines.append(f'# {sym.doc[:80]}')
            self.lines.append('')
    
    def _translate_value(self, val: str) -> str:
        """转换 C 值为 Python 表达式"""
        val = val.strip()
        # 十六进制
        if val.startswith('0x') or val.startswith('0X'):
            return val
        # 字符串
        if val.startswith('"') and val.endswith('"'):
            return val
        # 数字
        if val.isdigit():
            return val
        # ULL 后缀
        if val.endswith('ULL') or val.endswith('ull'):
            return '0x' + val[:-3]  # 简化
        return val
    
    def _ctype_for(self, ctype: str) -> str:
        """C 类型 → ctypes 类型"""
        ctype = ctype.strip()
        # 指针
        if ctype.endswith('*'):
            base = ctype.rstrip('*').strip()
            if base in ('char', 'const char'):
                return 'ctypes.c_char_p'
            return 'ctypes.c_void_p'
        return self.C_TO_CTYPES.get(ctype, f'ctypes.c_void_p  # {ctype}')


# ── 漂移检测器 ─────────────────────────────────────────────

class DriftDetector:
    """检测 C 头文件与 Python 绑定之间的漂移"""
    
    def __init__(self, encoding_dir: Path = ENCODING_DIR, native_py: Path = NATIVE_PY):
        self.encoding_dir = encoding_dir
        self.native_py = native_py
    
    def check(self) -> Dict[str, Any]:
        """执行漂移检测"""
        result = {
            'headers': {},
            'missing_in_python': [],
            'extra_in_python': [],
            'drift_detected': False,
        }
        
        # 收集所有头文件中的导出函数
        c_functions = set()
        c_constants = set()
        for h in self.encoding_dir.glob('*.h'):
            parser = CHeaderParser(h)
            for f in parser.functions():
                c_functions.add(f.name)
            for c in parser.constants():
                c_constants.add(c.name)
        
        # 收集 Python 绑定中引用的函数
        if self.native_py.exists():
            py_text = self.native_py.read_text()
            py_functions = set(re.findall(r'_lib\.(\w+)\.argtypes', py_text))
            py_constants = set(re.findall(r'^(\w+)\s*=\s*(?:0x|\d+)', py_text, re.MULTILINE))
        else:
            py_functions = set()
            py_constants = set()
        
        result['missing_in_python'] = sorted(c_functions - py_functions)
        result['extra_in_python'] = sorted(py_functions - c_functions)
        result['drift_detected'] = bool(result['missing_in_python'] or result['extra_in_python'])
        result['c_functions'] = sorted(c_functions)
        result['py_functions'] = sorted(py_functions)
        
        return result


# ── 自举引擎 ───────────────────────────────────────────────

class BootstrapEngine:
    """自举编译器核心引擎"""
    
    def __init__(self, encoding_dir: Path = ENCODING_DIR):
        self.encoding_dir = encoding_dir
        self.parsers: Dict[str, CHeaderParser] = {}
    
    def load_all(self):
        """加载所有头文件"""
        for h in sorted(self.encoding_dir.glob('*.h')):
            try:
                self.parsers[h.name] = CHeaderParser(h)
            except Exception as e:
                print(f"  [WARN] 解析 {h.name} 失败: {e}")
    
    def stats(self) -> Dict[str, int]:
        """统计所有符号"""
        stats = {'functions': 0, 'constants': 0, 'enums': 0, 'structs': 0}
        for parser in self.parsers.values():
            stats['functions'] += len(parser.functions())
            stats['constants'] += len(parser.constants())
            stats['enums'] += len(parser.enums())
            stats['structs'] += len(parser.structs())
        return stats
    
    def generate_registry(self) -> str:
        """生成符号注册表 (JSON 格式 · 供其他工具消费)"""
        registry = {}
        for name, parser in self.parsers.items():
            registry[name] = {
                'functions': [{'name': f.name, 'ret': f.value} for f in parser.functions()],
                'constants': [{'name': c.name, 'value': c.value} for c in parser.constants()],
            }
        return json.dumps(registry, indent=2, ensure_ascii=False)


# ── CLI 入口 ───────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='自举编译器 — 从 C 规则生长 Python 工具'
    )
    parser.add_argument('--parse', metavar='HEADER', help='解析单个头文件')
    parser.add_argument('--generate', metavar='HEADER', help='生成绑定')
    parser.add_argument('--check', action='store_true', help='漂移检测')
    parser.add_argument('--bootstrap', action='store_true', help='全量重建')
    parser.add_argument('--stats', action='store_true', help='符号统计')
    parser.add_argument('--registry', action='store_true', help='生成符号注册表')
    
    args = parser.parse_args()
    
    if args.parse:
        p = CHeaderParser(Path(args.parse))
        print(f'═══ 解析: {Path(args.parse).name} ═══')
        print(f'函数: {len(p.functions())}')
        for f in p.functions():
            print(f'  {f.value} {f.name}(...)')
        print(f'常量: {len(p.constants())}')
        for c in p.constants():
            print(f'  {c.name} = {c.value}')
    
    elif args.generate:
        p = CHeaderParser(Path(args.parse))
        g = BindingGenerator(p)
        print(g.generate())
    
    elif args.check:
        d = DriftDetector()
        result = d.check()
        print('═══ 漂移检测 ═══')
        print(f'C 函数总数: {len(result["c_functions"])}')
        print(f'Python 绑定函数: {len(result["py_functions"])}')
        if result['missing_in_python']:
            print(f'  ✗ 缺失绑定 ({len(result["missing_in_python"])}):')
            for f in result['missing_in_python']:
                print(f'    - {f}')
        if result['extra_in_python']:
            print(f'  ✗ 多余绑定 ({len(result["extra_in_python"])}):')
            for f in result['extra_in_python']:
                print(f'    - {f}')
        if not result['drift_detected']:
            print('  ✓ 无漂移')
    
    elif args.bootstrap:
        engine = BootstrapEngine()
        engine.load_all()
        stats = engine.stats()
        print('═══ 全量自举 ═══')
        print(f'头文件: {len(engine.parsers)}')
        print(f'函数: {stats["functions"]}')
        print(f'常量: {stats["constants"]}')
        print(f'枚举: {stats["enums"]}')
        print(f'结构体: {stats["structs"]}')
    
    elif args.stats:
        engine = BootstrapEngine()
        engine.load_all()
        stats = engine.stats()
        print('═══ 符号统计 ═══')
        for k, v in stats.items():
            print(f'  {k}: {v}')
        print()
        for name, parser in sorted(engine.parsers.items()):
            nf = len(parser.functions())
            nc = len(parser.constants())
            if nf + nc > 0:
                print(f'  {name}: {nf} functions, {nc} constants')
    
    elif args.registry:
        engine = BootstrapEngine()
        engine.load_all()
        print(engine.generate_registry())
    
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
