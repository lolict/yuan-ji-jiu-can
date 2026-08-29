#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
鼓膜自举 · 统一入口
====================
整合所有鼓膜相关功能：
  1. 三层鼓膜自举（振荡器 + 骨膜骨架 + 回响）
  2. 文件系统扫描（从文件系统重建操作系统）
  3. 呼唤编程接口（7种呼唤类型）
  4. FFI 桥接调用（Nim/Zig/Rust 编译后的库）

启动流程：
  鼓膜振荡 → 骨膜加载 → 回响自检 → 文件扫描 → 重建系统 → 呼唤就绪
"""

import os
import sys
import json
import time
import ctypes
import hashlib
import subprocess
from pathlib import Path

# 确保能导入同目录模块
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bootstrap as drum_bootstrap


# ============================================================
# 常量
# ============================================================

ROOT_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
TOOLS_DIR = os.path.join(ROOT_PATH, "tools")
FFI_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ffi")


# ============================================================
# 第一~三层：鼓膜自举（来自 bootstrap.py）
# ============================================================

def run_bootstrap():
    """执行三层鼓膜自举"""
    return drum_bootstrap.bootstrap()


# ============================================================
# 第四层：文件系统扫描（从文件系统重建操作系统）
# ============================================================

class FileSystemScanner:
    """扫描文件系统，重建操作系统"""
    
    CORE_FILES = {
        "SYMBOL_DICTIONARY.md": "符号字典",
        "CONTAINER_TOPOLOGY.md": "容器拓扑",
        "GLOBAL_DESTINATION.md": "全局目的地",
        "TOOL_REGISTRY.md": "工具注册表",
        "BOOTSTRAP.md": "启动协议",
    }
    
    def __init__(self, root_path=ROOT_PATH):
        self.root_path = root_path
        self.scanned_files = []
        self.file_count = 0
        self.restored_tools = []
        self.integrity_checks = []
    
    def scan(self):
        """扫描文件系统"""
        self.scanned_files = []
        
        # 扫描核心文件
        for fname, description in self.CORE_FILES.items():
            fpath = os.path.join(self.root_path, fname)
            if os.path.exists(fpath):
                info = os.stat(fpath)
                self.scanned_files.append({
                    "path": fname,
                    "kind": "core",
                    "description": description,
                    "size": info.st_size,
                    "exists": True
                })
        
        # 扫描 tools/ 目录
        tools_dir = os.path.join(self.root_path, "tools")
        if os.path.exists(tools_dir):
            for dirpath, dirnames, filenames in os.walk(tools_dir):
                for fname in filenames:
                    if fname.endswith(('.nim', '.py', '.zig', '.rs')):
                        rel_path = os.path.relpath(os.path.join(dirpath, fname), self.root_path)
                        self.scanned_files.append({
                            "path": rel_path,
                            "kind": "source",
                            "description": fname,
                            "size": os.path.getsize(os.path.join(dirpath, fname)),
                            "exists": True
                        })
        
        self.file_count = len(self.scanned_files)
        return self.scanned_files
    
    def rebuild_dictionary(self):
        """重建符号字典"""
        for f in self.scanned_files:
            if f["path"] == "SYMBOL_DICTIONARY.md":
                return True, f"符号字典: {f['size']} 字节"
        return False, "符号字典: 未找到"
    
    def rebuild_topology(self):
        """重建容器拓扑"""
        for f in self.scanned_files:
            if f["path"] == "CONTAINER_TOPOLOGY.md":
                return True, f"容器拓扑: {f['size']} 字节"
        return False, "容器拓扑: 未找到"
    
    def rebuild_tools(self):
        """重建工具清单"""
        tool_names = set()
        for f in self.scanned_files:
            parts = f["path"].split("/")
            if len(parts) >= 2 and parts[0] == "tools":
                tool_names.add(parts[1])
        
        self.restored_tools = sorted(tool_names)
        return len(self.restored_tools), f"工具清单: {len(self.restored_tools)} 个工具"
    
    def verify_integrity(self):
        """验证完整性"""
        self.integrity_checks = []
        all_passed = True
        
        # 检查核心文件
        for fname in self.CORE_FILES:
            found = any(f["path"] == fname for f in self.scanned_files)
            if found:
                self.integrity_checks.append(f"✅ {self.CORE_FILES[fname]}")
            else:
                self.integrity_checks.append(f"❌ {self.CORE_FILES[fname]}")
                all_passed = False
        
        # 检查工具数量
        if len(self.restored_tools) >= 5:
            self.integrity_checks.append(f"✅ 工具数量: {len(self.restored_tools)}")
        else:
            self.integrity_checks.append(f"⚠️ 工具数量偏少: {len(self.restored_tools)}")
        
        return all_passed, self.integrity_checks


# ============================================================
# 第五层：呼唤编程接口
# ============================================================

class InvocationEngine:
    """呼唤编程引擎 — 7种呼唤类型"""
    
    # 身份注册表（符号 → 实现路径）
    IDENTITY_MAP = {
        # 声母
        "b": {"container": "声母", "position": 0, "identity": "首"},
        "p": {"container": "声母", "position": 1, "identity": "次"},
        "m": {"container": "声母", "position": 2, "identity": "三"},
        "f": {"container": "声母", "position": 3, "identity": "四"},
        # 韵母
        "a": {"container": "韵母", "position": 0, "identity": "首"},
        "o": {"container": "韵母", "position": 1, "identity": "次"},
        "e": {"container": "韵母", "position": 2, "identity": "三"},
        "i": {"container": "韵母", "position": 3, "identity": "四"},
        # 汉字
        "袋": {"container": "汉字", "position": 0, "family": "容器族"},
        "照": {"container": "汉字", "position": 1, "family": "操作族"},
        "见": {"container": "汉字", "position": 2, "family": "操作族"},
        "阴": {"container": "汉字", "position": 3, "family": "极性族"},
        "加": {"container": "汉字", "position": 4, "family": "操作族"},
        # 自定义符号
        "knmll": {"type": "进制", "description": "三维进制感知空间", "tool": "kndoyle"},
        "kndl": {"type": "外壳", "description": "柯南道尔斯外壳·管辖", "tool": "kndoyle"},
        "kndoyle": {"type": "IP胶囊", "description": "柯南道尔斯IP胶囊", "tool": "kndoyle"},
        "maolilan": {"type": "框架", "description": "毛利兰立体进制框架", "tool": "maolilan"},
        "mlll": {"type": "编码", "description": "254+2集装箱编码", "tool": "mlll-254"},
        "mlil": {"type": "属性", "description": "毛利兰·表面属性(模/面/名)", "tool": "maolilan"},
        "gtxone": {"type": "分析", "description": "吸引子分析", "tool": "gtxone"},
        "gttx1": {"type": "分析", "description": "gtXone-1", "tool": "gtxone"},
        "cocoon": {"type": "寄生", "description": "蚕蛹反向寄生壳", "tool": "canjiang"},
        "fqm": {"type": "协议", "description": "满全法容器操作协议", "tool": "metaime"},
        "njsg": {"type": "共生", "description": "囡囝共生NJSG", "tool": "njsg"},
        "mltt": {"type": "密码", "description": "mltt命名密码表", "tool": "mltt-cipher"},
    }
    
    # 容器注册表
    CONTAINER_MAP = {
        "声母": {"symbols": 23, "capacity": 64, "empty": 41},
        "韵母": {"symbols": 24, "capacity": 64, "empty": 40},
        "声调": {"symbols": 4, "capacity": 64, "empty": 60},
        "汉字": {"symbols": 64, "capacity": 64, "empty": 0},
    }
    
    # 进制注册表
    RADIX_MAP = {
        "mlll": {"base": 254, "description": "254+2集装箱，我=1/刘楚恬=0"},
        "color": {"base": 64, "description": "64进制相位，PhaseCell"},
        "3d": {"base": 23 * 24 * 4, "description": "声母×韵母×声调空间"},
        "fqm": {"base": None, "description": "满全法容器路径语法"},
    }
    
    # 规则注册表
    RULE_MAP = {
        "加法统摄": {"description": "加法统摄，禁用乘除，A+B=AB", "allowed": ["add", "sub"]},
        "袋子级": {"description": "袋子级操作，不数米粒", "unit": "袋"},
        "三元见证": {"description": "非空+已见证+锚点一致", "requirements": 3},
        "满全法": {"description": "满全法核心裁判", "type": "裁判"},
        "蚕蛹": {"description": "蚕蛹反向寄生壳", "type": "寄生"},
        "njsg": {"description": "囡囝共生NJSG", "type": "共生"},
    }
    
    def invoke(self, invocation_type, target, params=None):
        """
        执行呼唤
        
        Args:
            invocation_type: 呼唤类型 (identity/container/radix/relation/queue/rule/geometry)
            target: 呼唤目标
            params: 额外参数
        
        Returns:
            dict: {ok, type, target, result}
        """
        handlers = {
            "identity": self._invoke_identity,
            "container": self._invoke_container,
            "radix": self._invoke_radix,
            "relation": self._invoke_relation,
            "queue": self._invoke_queue,
            "rule": self._invoke_rule,
            "geometry": self._invoke_geometry,
        }
        
        handler = handlers.get(invocation_type)
        if handler:
            return handler(target, params or {})
        else:
            return {"ok": False, "error": f"未知呼唤类型: {invocation_type}"}
    
    def _invoke_identity(self, symbol, params):
        """身份呼唤"""
        info = self.IDENTITY_MAP.get(symbol)
        if info:
            return {"ok": True, "type": "identity", "target": symbol, "result": info}
        return {"ok": False, "type": "identity", "target": symbol, "error": "未知身份"}
    
    def _invoke_container(self, name, params):
        """容器呼唤"""
        info = self.CONTAINER_MAP.get(name)
        if info:
            return {"ok": True, "type": "container", "target": name, "result": info}
        return {"ok": False, "type": "container", "target": name, "error": "未知容器"}
    
    def _invoke_radix(self, code, params):
        """进制呼唤"""
        info = self.RADIX_MAP.get(code)
        if info:
            return {"ok": True, "type": "radix", "target": code, "result": info}
        return {"ok": False, "type": "radix", "target": code, "error": "未知进制"}
    
    def _invoke_relation(self, target, params):
        """关系呼唤"""
        # 格式: 关系(b, a) → 声母.首 ↔ 韵母.首
        symbols = params.get("symbols", [])
        if len(symbols) >= 2:
            return {
                "ok": True,
                "type": "relation",
                "target": target,
                "result": f"{symbols[0]} ↔ {symbols[1]} (跨容器链接)"
            }
        return {"ok": False, "type": "relation", "error": "需要至少2个符号"}
    
    def _invoke_queue(self, target, params):
        """队列呼唤"""
        chain = params.get("chain", [target])
        return {
            "ok": True,
            "type": "queue",
            "target": target,
            "result": " → ".join(chain)
        }
    
    def _invoke_rule(self, name, params):
        """规则呼唤"""
        info = self.RULE_MAP.get(name)
        if info:
            return {"ok": True, "type": "rule", "target": name, "result": info}
        return {"ok": False, "type": "rule", "target": name, "error": "未知规则"}
    
    def _invoke_geometry(self, target, params):
        """几何呼唤"""
        # 格式: 坐标(b, a, 1)
        coords = params.get("coords", [])
        if len(coords) >= 3:
            return {
                "ok": True,
                "type": "geometry",
                "target": target,
                "result": {
                    "shengmu": coords[0],
                    "yunmu": coords[1],
                    "tone": coords[2],
                    "space": "声母×韵母×声调三维空间"
                }
            }
        return {"ok": False, "type": "geometry", "error": "需要3个坐标参数"}


# ============================================================
# 第六层：FFI 桥接调用
# ============================================================

class FFIBridge:
    """FFI 桥接 — 调用 Nim/Zig/Rust 编译后的库"""
    
    def __init__(self, ffi_dir=FFI_DIR):
        self.ffi_dir = ffi_dir
        self.libraries = {}
        self._load_libraries()
    
    def _load_libraries(self):
        """加载所有可用的 FFI 库"""
        lib_paths = {
            "nim": os.path.join(self.ffi_dir, "libmetaime_nim_bridge.so"),
            "zig": os.path.join(self.ffi_dir, "libffi_bridge.so"),
            "rust": os.path.join(self.ffi_dir, "rust_bridge", "target", "release", "libmetaime_rust_bridge.so"),
        }
        
        for lang, path in lib_paths.items():
            if os.path.exists(path):
                try:
                    lib = ctypes.CDLL(path)
                    self.libraries[lang] = lib
                except OSError:
                    pass
    
    def available_languages(self):
        """返回可用的语言列表"""
        return list(self.libraries.keys())
    
    def call_translate(self, lang, foreign_symbol):
        """调用翻译函数"""
        lib = self.libraries.get(lang)
        if not lib:
            return None, f"语言库不可用: {lang}"
        
        try:
            # 设置函数签名
            func = lib.mi_translator_translate
            func.restype = ctypes.c_int
            func.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
            
            out_buf = ctypes.create_string_buffer(128)
            result = func(None, foreign_symbol.encode('utf-8'), out_buf, 128)
            
            if result == 0:  # MI_OK
                return out_buf.value.decode('utf-8'), None
            else:
                return None, f"翻译错误码: {result}"
        except Exception as e:
            return None, str(e)
    
    def call_gcd(self, lang, a, b):
        """调用 GCD 函数"""
        lib = self.libraries.get(lang)
        if not lib:
            return None
        
        try:
            func = lib.mi_gcd
            func.restype = ctypes.c_uint8
            func.argtypes = [ctypes.c_uint8, ctypes.c_uint8]
            return func(a, b)
        except:
            return None
    
    def call_lcm(self, lang, a, b):
        """调用 LCM 函数"""
        lib = self.libraries.get(lang)
        if not lib:
            return None
        
        try:
            func = lib.mi_lcm
            func.restype = ctypes.c_uint16
            func.argtypes = [ctypes.c_uint8, ctypes.c_uint8]
            return func(a, b)
        except:
            return None


# ============================================================
# 统一入口
# ============================================================

class DrumUnifiedSystem:
    """鼓膜自举统一系统"""
    
    def __init__(self):
        self.bootstrap_result = None
        self.scanner = FileSystemScanner()
        self.invocation = InvocationEngine()
        self.ffi = FFIBridge()
    
    def start(self):
        """启动完整系统"""
        print("=" * 60)
        print("鼓膜自举 · 统一系统")
        print("=" * 60)
        
        # 步骤1: 鼓膜自举
        print("\n【步骤1】鼓膜自举")
        self.bootstrap_result = run_bootstrap()
        if not self.bootstrap_result["ok"]:
            print("❌ 自举失败，系统锁定")
            return False
        
        # 步骤2: 文件系统扫描
        print("\n【步骤2】文件系统扫描")
        files = self.scanner.scan()
        print(f"  扫描文件: {len(files)} 个")
        
        # 步骤3: 重建系统
        print("\n【步骤3】重建系统")
        ok1, msg1 = self.scanner.rebuild_dictionary()
        print(f"  {'✅' if ok1 else '❌'} {msg1}")
        ok2, msg2 = self.scanner.rebuild_topology()
        print(f"  {'✅' if ok2 else '❌'} {msg2}")
        count, msg3 = self.scanner.rebuild_tools()
        print(f"  ✅ {msg3}")
        
        # 步骤4: 完整性验证
        print("\n【步骤4】完整性验证")
        passed, checks = self.scanner.verify_integrity()
        for check in checks:
            print(f"  {check}")
        
        # 步骤5: FFI 桥接状态
        print("\n【步骤5】FFI 桥接")
        langs = self.ffi.available_languages()
        if langs:
            print(f"  可用语言库: {', '.join(langs)}")
        else:
            print("  ⚠️ 无编译后的语言库（需先编译 Nim/Zig/Rust）")
        
        # 步骤6: 呼唤编程就绪
        print("\n【步骤6】呼唤编程接口")
        print("  7种呼唤类型已就绪: 身份/容器/进制/关系/队列/规则/几何")
        
        print("\n" + "=" * 60)
        print("鼓膜自举完成 · 系统就绪")
        print("=" * 60)
        
        return True
    
    def invoke(self, invocation_type, target, **params):
        """执行呼唤"""
        return self.invocation.invoke(invocation_type, target, params)
    
    def translate(self, lang, symbol):
        """通过 FFI 翻译符号"""
        return self.ffi.call_translate(lang, symbol)
    
    def get_status(self):
        """获取系统状态"""
        return {
            "bootstrap": self.bootstrap_result is not None and self.bootstrap_result["ok"],
            "files_scanned": self.scanner.file_count,
            "tools_restored": len(self.scanner.restored_tools),
            "ffi_languages": self.ffi.available_languages(),
            "invocation_types": 7,
        }


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    system = DrumUnifiedSystem()
    success = system.start()
    
    if success:
        # 演示呼唤编程
        print("\n【呼唤演示】")
        
        # 身份呼唤
        result = system.invoke("identity", "b")
        print(f"  身份呼唤(b): {result.get('result', {})}")
        
        result = system.invoke("identity", "袋")
        print(f"  身份呼唤(袋): {result.get('result', {})}")
        
        # 容器呼唤
        result = system.invoke("container", "声母")
        print(f"  容器呼唤(声母): {result.get('result', {})}")
        
        # 进制呼唤
        result = system.invoke("radix", "mlll")
        print(f"  进制呼唤(mlll): {result.get('result', {})}")
        
        # 规则呼唤
        result = system.invoke("rule", "满全法")
        print(f"  规则呼唤(满全法): {result.get('result', {})}")
        
        # 几何呼唤
        result = system.invoke("geometry", "coord", coords=["b", "a", "1"])
        print(f"  几何呼唤(b,a,1): {result.get('result', {})}")
        
        # 关系呼唤
        result = system.invoke("relation", "link", symbols=["b", "a"])
        print(f"  关系呼唤(b↔a): {result.get('result', '')}")
    
    exit(0 if success else 1)
