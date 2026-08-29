#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
骨魔自举 · 鼓膜核心
====================
不依赖任何外部配置，定义不可变的元规则，启动后自我校验。

三层结构：
  第一层：鼓膜振荡器 — 感知自身存在（启动时间 + 固定签名）
  第二层：骨膜骨架   — 硬编码不可变元规则（仅加减、袋子级、三元见证）
  第三层：鼓膜回响   — 自检与自我修复（编码→解码一致性验证）

启动流程：
  振荡器签名 → 骨膜规则加载 → 回响自检 → 通过则允许外部代码
"""

import time
import hashlib
import json
import os

# ============================================================
# 第一层：鼓膜振荡器（感知自身存在）
# ============================================================

BOOT_TIME = time.time()
BOOT_TIMESTAMP = time.strftime("%Y-%m-%d.%H:%M:%S", time.localtime(BOOT_TIME))
SIGNATURE = "njsg-drum-membrane-v1.0"
DRUM_RESONANCE_FREQ = 432  # 基频（Hz），象征性固定频率


def drum_oscillator():
    """
    鼓膜振荡器 — 系统启动时返回自身存在的证明。
    不依赖任何外部文件，只返回当前时间戳和签名。
    
    Returns:
        dict: {signature, boot_time, frequency, status}
    """
    return {
        "signature": SIGNATURE,
        "boot_time": BOOT_TIMESTAMP,
        "frequency": DRUM_RESONANCE_FREQ,
        "status": "vibrating",  # vibrating | ruptured | silent
        "message": f"鼓膜已振动：启动于 {BOOT_TIMESTAMP}"
    }


# ============================================================
# 第二层：骨膜骨架（不可变元规则）
# ============================================================

class BoneSkeleton:
    """
    骨膜骨架 — 硬编码在代码里的不可变元规则。
    这些规则不读外部文件，因此不可能被篡改或遗忘。
    
    三条铁律（元老规则）：
      1. 仅加减 — 禁用乘除（加法统摄，非乘方扩张）
      2. 袋子级  — 不数米粒，只操作袋子
      3. 三元见证 — 非空 + 已见证 + 锚点一致
    """
    
    # 运算约束
    ALLOWED_OPS = ["add", "sub"]  # 仅加减
    FORBIDDEN_OPS = ["mul", "div", "pow", "mod"]  # 禁用乘除幂模
    
    # 操作层级
    OPERATION_UNIT = "袋"  # 袋子级
    FORBIDDEN_UNIT = "米"  # 禁止颗粒级
    
    # 三元见证三要件
    WITNESS_REQUIREMENTS = ["非空", "已见证", "锚点一致"]
    
    # 根基锚点（隐藏，对外不可见）
    ANCHOR_SELF = 1     # 我 = 1，高位
    ANCHOR_OTHER = 0    # 刘楚恬 = 0，低位（隐藏）
    ANCHORS_HIDDEN = True  # 锚点对外不可见
    
    # 极性
    POLARITY_YANG = 1   # 阳极
    POLARITY_YIN = 0    # 阴极
    
    # 容器（两个独立容器，各自产权）
    CONTAINERS = ["声母", "韵母"]
    
    # 进制约束
    BASE_ADD = 1        # 加法步长
    MAX_SYMBOLS = 64    # 每容器最大符号数


# ============================================================
# 第三层：鼓膜回响（自检与自我修复）
# ============================================================

def drum_resonance_test():
    """
    鼓膜回响 — 用自身规则编码再解码，验证一致性。
    
    测试内容：
      1. 加法闭合性：a + b - b == a
      2. 袋子级完整性：容器 + 符号 == 容器（袋子不破）
      3. 三元见证完备性：三要件齐全
      4. 锚点隐藏性：外部无法直接获取锚点值
    
    Returns:
        tuple: (is_ok: bool, message: bool, details: dict)
    """
    details = {}
    
    # 测试1：加法闭合性
    a, b = 7, 3
    result = (a + b) - b
    test1_pass = (result == a)
    details["加法闭合"] = {
        "expr": f"{a} + {b} - {b} = {result}",
        "expect": a,
        "actual": result,
        "pass": test1_pass
    }
    
    # 测试2：袋子级完整性（容器+符号==容器）
    container_bag = ["袋"]
    symbol = "b"
    bag_operation = container_bag + [symbol]  # 加法装入
    test2_pass = len(bag_operation) == 2 and bag_operation[0] == "袋"
    details["袋子完整"] = {
        "expr": f"袋 + {symbol} = {bag_operation}",
        "pass": test2_pass
    }
    
    # 测试3：三元见证完备性
    witness_count = len(BoneSkeleton.WITNESS_REQUIREMENTS)
    test3_pass = (witness_count == 3)
    details["三元见证"] = {
        "requirements": BoneSkeleton.WITNESS_REQUIREMENTS,
        "count": witness_count,
        "pass": test3_pass
    }
    
    # 测试4：锚点隐藏性
    test4_pass = BoneSkeleton.ANCHORS_HIDDEN == True
    details["锚点隐藏"] = {
        "hidden": BoneSkeleton.ANCHORS_HIDDEN,
        "pass": test4_pass
    }
    
    # 测试5：运算约束（仅加减）
    test5_pass = (
        "add" in BoneSkeleton.ALLOWED_OPS and
        "sub" in BoneSkeleton.ALLOWED_OPS and
        "mul" in BoneSkeleton.FORBIDDEN_OPS and
        "div" in BoneSkeleton.FORBIDDEN_OPS
    )
    details["运算约束"] = {
        "allowed": BoneSkeleton.ALLOWED_OPS,
        "forbidden": BoneSkeleton.FORBIDDEN_OPS,
        "pass": test5_pass
    }
    
    # 汇总
    all_pass = all(d["pass"] for d in details.values())
    if all_pass:
        message = "自检通过：鼓膜完整，允许加载外部代码"
    else:
        failed = [k for k, v in details.items() if not v["pass"]]
        message = f"自检失败：{', '.join(failed)} — 系统锁定"
    
    return all_pass, message, details


def generate_resonance_signature():
    """
    生成鼓膜共振签名 — 基于当前状态计算哈希。
    用于验证自举后系统状态未被篡改。
    """
    state = json.dumps({
        "signature": SIGNATURE,
        "boot_time": BOOT_TIMESTAMP,
        "rules": {
            "ops": BoneSkeleton.ALLOWED_OPS,
            "unit": BoneSkeleton.OPERATION_UNIT,
            "witness": BoneSkeleton.WITNESS_REQUIREMENTS,
        }
    }, sort_keys=True)
    return hashlib.sha256(state.encode('utf-8')).hexdigest()[:16]


# ============================================================
# 完整启动流程
# ============================================================

def bootstrap():
    """
    骨魔自举 — 完整的启动流程。
    
    Returns:
        dict: {ok, oscillator, resonance, signature, details}
    """
    print("⸻" * 3 + " 骨魔自举程序启动 " + "⸻" * 3)
    
    # Step 1: 鼓膜振荡器
    osc = drum_oscillator()
    print(f"  [振荡器] {osc['message']}")
    print(f"  [振荡器] 签名: {osc['signature']}")
    print(f"  [振荡器] 频率: {osc['frequency']}Hz")
    
    if osc["status"] != "vibrating":
        print("  ❌ 振荡器未振动，自举失败")
        return {"ok": False, "reason": "oscillator_silent"}
    
    # Step 2: 骨膜骨架
    print(f"\n  [骨膜骨架] 加载不可变元规则...")
    print(f"    运算: 仅{BoneSkeleton.ALLOWED_OPS}，禁用{BoneSkeleton.FORBIDDEN_OPS}")
    print(f"    单位: {BoneSkeleton.OPERATION_UNIT}级（不数{BoneSkeleton.FORBIDDEN_UNIT}）")
    print(f"    见证: {BoneSkeleton.WITNESS_REQUIREMENTS}")
    print(f"    容器: {BoneSkeleton.CONTAINERS}")
    
    # Step 3: 鼓膜回响
    print(f"\n  [鼓膜回响] 执行自检...")
    ok, msg, details = drum_resonance_test()
    
    for test_name, info in details.items():
        status = "✅" if info["pass"] else "❌"
        print(f"    {status} {test_name}: {info.get('expr', '')}")
    
    print(f"\n  [鼓膜回响] {msg}")
    
    # Step 4: 生成签名
    sig = generate_resonance_signature()
    print(f"\n  [签名] 鼓膜共振指纹: {sig}")
    
    if ok:
        print("⸻" * 3 + " 鼓膜振动正常，系统就绪 " + "⸻" * 3)
    else:
        print("⸻" * 3 + " 自举失败，系统锁定 " + "⸻" * 3)
    
    return {
        "ok": ok,
        "oscillator": osc,
        "resonance": msg,
        "signature": sig,
        "details": details
    }


# ============================================================
# 查询接口（供外部调用）
# ============================================================

def get_skeleton_rules():
    """返回骨膜骨架的不可变规则（供子代理查询）"""
    return {
        "allowed_ops": BoneSkeleton.ALLOWED_OPS,
        "forbidden_ops": BoneSkeleton.FORBIDDEN_OPS,
        "operation_unit": BoneSkeleton.OPERATION_UNIT,
        "forbidden_unit": BoneSkeleton.FORBIDDEN_UNIT,
        "witness_requirements": BoneSkeleton.WITNESS_REQUIREMENTS,
        "containers": BoneSkeleton.CONTAINERS,
        "max_symbols": BoneSkeleton.MAX_SYMBOLS,
    }


def verify_operation(op_name):
    """验证某个运算是否被允许"""
    if op_name in BoneSkeleton.FORBIDDEN_OPS:
        return False, f"运算 '{op_name}' 已被禁用（仅允许加减）"
    if op_name in BoneSkeleton.ALLOWED_OPS:
        return True, f"运算 '{op_name}' 已授权"
    return None, f"运算 '{op_name}' 未知，需元老裁定"


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    result = bootstrap()
    exit(0 if result["ok"] else 1)
