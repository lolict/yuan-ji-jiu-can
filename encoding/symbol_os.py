#!/usr/bin/env python3
"""
符号操作系统 — 通过 FFI 调用 C 共享库

符号只定义一次（symbol_table.c），Python 不重复定义。
查询通过 ctypes 调用 libsymbol_table.so，O(1) 哈希索引。
"""

import ctypes
import os

# 加载共享库
_lib_path = os.path.join(os.path.dirname(__file__), "libsymbol_table.so")
_lib = ctypes.CDLL(_lib_path)

# 函数签名
_lib.查地址.argtypes = [ctypes.c_char_p]
_lib.查地址.restype = ctypes.c_int

_lib.查符号.argtypes = [ctypes.c_int]
_lib.查符号.restype = ctypes.c_char_p

_lib.存在.argtypes = [ctypes.c_char_p]
_lib.存在.restype = ctypes.c_int

_lib.符号数量.argtypes = []
_lib.符号数量.restype = ctypes.c_int

_lib.族起始.argtypes = [ctypes.c_int]
_lib.族起始.restype = ctypes.c_int

_lib.族数量.argtypes = [ctypes.c_int]
_lib.族数量.restype = ctypes.c_int


def 查地址(符号):
    """符号 → 地址，返回 -1 表示不存在"""
    return _lib.查地址(符号.encode("utf-8"))


def 查符号(地址):
    """地址 → 符号，返回 None 表示地址无效"""
    result = _lib.查符号(地址)
    if result is None:
        return None
    return result.decode("utf-8")


def 存在(符号):
    """符号是否存在"""
    return _lib.存在(符号.encode("utf-8")) == 1


def 符号数量():
    """符号总数"""
    return _lib.符号数量()


def 族起始(族编号):
    """族起始地址：0=声母, 1=韵母, 2=声调, 3=容器, 4=操作, 5=极性, 6=层级, 7=审判, 8=产权"""
    return _lib.族起始(族编号)


def 族数量(族编号):
    """族内符号数量"""
    return _lib.族数量(族编号)


# ═══════════════════════════════════════════════════════════════
# 便捷接口
# ═══════════════════════════════════════════════════════════════

def 查询(符号):
    """查询符号的完整信息"""
    地址 = 查地址(符号)
    if 地址 < 0:
        return None
    return {"符号": 符号, "地址": 地址}


def 批量查询(符号列表):
    """批量查询多个符号"""
    return [查询(符号) for 符号 in 符号列表]


if __name__ == "__main__":
    print("=" * 50)
    print("符号操作系统 (FFI → C 共享库)")
    print("=" * 50)
    
    print(f"\n符号总数: {符号数量()}")
    
    print("\n【查询演示】")
    for 符号 in ["b", "袋", "照", "阴", "元", "裁", "产"]:
        结果 = 查询(符号)
        if 结果:
            print(f"  {结果['符号']} → 地址{结果['地址']}")
    
    print("\n【族信息】")
    族名 = ["声母", "韵母", "声调", "容器", "操作", "极性", "层级", "审判", "产权"]
    for i, 名 in enumerate(族名):
        print(f"  {名}: 起始={族起始(i)}, 数量={族数量(i)}")
    
    print("\n" + "=" * 50)
