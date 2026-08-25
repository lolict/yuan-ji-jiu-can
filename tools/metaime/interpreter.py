#!/usr/bin/env python3
"""
MetaIME 解释器 · 活码运行时 v2

核心改进：
- 正确分离声母容器和韵母容器
- 实现声母训练韵母的秩序流程
- 边界到达后从另一个容器拿新东西
"""

import json
import os
import sys
from datetime import datetime

# ============================================================
# 第一层：零维根（静态 · 不可修改）
# ============================================================

ZERO_DIM_ROOT = {
    "0": {"symbol": "刘楚恬", "side": "隐藏侧", "visible": False},
    "1": {"symbol": "我", "side": "可见侧", "visible": True}
}

# ============================================================
# 第二层：全局偏移量（活码机制）
# ============================================================

GLOBAL_OFFSET = 0x00

# ============================================================
# 声母/韵母物理定义（人的发声器官决定，不是人为定义）
# ============================================================

# 声母 = 辅音 = 气流受阻产生的音
SHENGMU_LIST = [
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "j", "q", "x",
    "zh", "ch", "sh", "r", "z", "c", "s",
    "y", "w"
]

# 韵母 = 元音 或 元音+辅音韵母 = 气流不受阻产生的音
YUNMU_LIST = [
    "a", "o", "e", "i", "u", "ü",
    "ai", "ei", "ui", "ao", "ou", "iu",
    "ie", "üe", "er",
    "an", "en", "in", "un", "ün",
    "ang", "eng", "ing", "ong"
]

# 声调
TONE_LIST = ["阴平", "阳平", "上声", "去声", "轻声"]

# ============================================================
# 第三层：容器（声母容器和韵母容器独立）
# ============================================================

class Container:
    """产权容器"""
    
    def __init__(self, name, container_type):
        self.name = name
        self.container_type = container_type  # "声母" 或 "韵母"
        self.symbols = []
        self.occupied = 0
        self.capacity = 64
    
    def add(self, symbol, synonyms=None):
        """添加符号到容器"""
        if self.occupied >= self.capacity:
            return False, f"容器已满（{self.occupied}/{self.capacity}）"
        
        entry = {
            "symbol": symbol,
            "synonyms": synonyms or [],
            "position": self.occupied,
            "occupied_at": datetime.now().isoformat()
        }
        self.symbols.append(entry)
        self.occupied += 1
        return True, entry
    
    def remove(self, symbol):
        """从容器移除符号"""
        for i, entry in enumerate(self.symbols):
            if entry["symbol"] == symbol:
                self.symbols.pop(i)
                self.occupied -= 1
                # 后续符号前移
                for j in range(i, self.occupied):
                    self.symbols[j]["position"] = j
                return True
        return False
    
    def get_position(self, symbol):
        """获取符号位置（含全局偏移）"""
        for entry in self.symbols:
            if entry["symbol"] == symbol:
                return GLOBAL_OFFSET + entry["position"]
        return None
    
    def is_full(self):
        return self.occupied >= self.capacity
    
    def is_empty(self):
        return self.occupied == 0
    
    def display(self):
        """显示容器状态"""
        print(f"\n{'='*60}")
        print(f"{self.name} ({self.container_type})")
        print(f"占用: {self.occupied}/{self.capacity}")
        print(f"{'-'*60}")
        for entry in self.symbols:
            pos = GLOBAL_OFFSET + entry["position"]
            synonyms_str = "、".join(entry["synonyms"][:3])
            print(f"  0x{pos:02X}  {entry['symbol']:<6} [{synonyms_str}]")
        print(f"{'='*60}\n")


# ============================================================
# 第四层：跨容器关系（秩序的来源）
# ============================================================

class CrossContainerOrder:
    """
    秩序 = 从另一个容器拿东西来训练当前容器
    
    声母容器 → 拿一个声母 → 韵母容器
    → 训练韵母的 0维和1维
    → 训练到终点（边界）
    → 减法倒计时 → 拿下一个声母
    """
    
    def __init__(self, shengmu_container, yunmu_container):
        self.shengmu = shengmu_container
        self.yunmu = yunmu_container
        self.current_shengmu_index = 0
        self.training_log = []
    
    def train_next(self):
        """拿下一个声母训练韵母"""
        if self.shengmu.is_empty():
            return "声母容器为空，无法训练"
        if self.yunmu.is_empty():
            return "韵母容器为空，无法训练"
        
        if self.current_shengmu_index >= self.shengmu.occupied:
            # 所有声母训练完毕，回到开头（边界到达后重启是错误做法）
            # 正确做法：从另一个容器拿新东西
            return "所有声母已训练完毕。边界到达后不是重启，需要从另一个容器拿新东西。"
        
        shengmu = self.shengmu.symbols[self.current_shengmu_index]
        results = []
        
        # 声母遍历韵母的 0维到1维
        for yunmu in self.yunmu.symbols:
            record = {
                "声母": shengmu["symbol"],
                "韵母": yunmu["symbol"],
                "训练": f"{shengmu['symbol']} → {yunmu['symbol']} (0维→1维)",
                "时间": datetime.now().isoformat()
            }
            results.append(record)
            self.training_log.append(record)
        
        # 减法倒计时：当前声母训练完毕，下一个
        self.current_shengmu_index += 1
        
        return results
    
    def get_status(self):
        """获取训练状态"""
        return {
            "声母容器占用": self.shengmu.occupied,
            "韵母容器占用": self.yunmu.occupied,
            "当前训练声母索引": self.current_shengmu_index,
            "已训练次数": len(self.training_log),
            "下一个声母": self.shengmu.symbols[self.current_shengmu_index]["symbol"] 
                         if self.current_shengmu_index < self.shengmu.occupied else "（完毕）"
        }
    
    def display_status(self):
        """显示训练状态"""
        status = self.get_status()
        print(f"\n{'='*60}")
        print("跨容器训练状态")
        print(f"{'-'*60}")
        for k, v in status.items():
            print(f"  {k}: {v}")
        print(f"{'='*60}\n")


# ============================================================
# 第五层：五维审判引擎
# ============================================================

class JudgmentEngine:
    """五维审判引擎"""
    
    def __init__(self, shengmu_container, yunmu_container):
        self.shengmu = shengmu_container
        self.yunmu = yunmu_container
    
    def judge_pronunciation(self, new_symbol, candidate, container_type):
        """发音审判"""
        if container_type == "声母":
            # 检查是否是合法声母
            if new_symbol in SHENGMU_LIST:
                return True, f"合法声母（{new_symbol}）"
            # 检查是否与候选邻近
            common = set(new_symbol) & set(candidate)
            if common:
                return True, f"共享字符: {common}"
        elif container_type == "韵母":
            if new_symbol in YUNMU_LIST:
                return True, f"合法韵母（{new_symbol}）"
            common = set(new_symbol) & set(candidate)
            if common:
                return True, f"共享字符: {common}"
        return False, "发音不匹配"
    
    def judge_glyph(self, new_symbol, candidate):
        """字形审判"""
        common = set(new_symbol) & set(candidate)
        if len(common) >= 1:
            return True, f"共享部件: {common}"
        return False, "无共享部件"
    
    def judge_ontology(self, new_symbol, candidate, container_type):
        """存在论审判"""
        for entry in self.shengmu.symbols:
            if entry["symbol"] == candidate:
                if new_symbol in entry.get("synonyms", []):
                    return True, f"在声调节器容器中，{candidate}的近义词族"
        for entry in self.yunmu.symbols:
            if entry["symbol"] == candidate:
                if new_symbol in entry.get("synonyms", []):
                    return True, f"在韵母容器中，{candidate}的近义词族"
        return False, "不在任何近义词族中"
    
    def judge_polarity(self, new_symbol, candidate, container_type):
        """极性审判：同极性才是近亲"""
        if container_type == "声母":
            if new_symbol in SHENGMU_LIST and candidate in SHENGMU_LIST:
                return True, "同为声母"
        elif container_type == "韵母":
            if new_symbol in YUNMU_LIST and candidate in YUNMU_LIST:
                return True, "同为韵母"
        return False, "极性不同（声母/韵母不相混）"
    
    def judge_hierarchy(self, new_symbol, candidate, container_type):
        """层级审判"""
        container = self.shengmu if container_type == "声母" else self.yunmu
        for i, entry in enumerate(container.symbols):
            if entry["symbol"] == candidate:
                # 检查相邻
                if i > 0 and container.symbols[i-1]["symbol"] == new_symbol:
                    return True, "在容器队列中相邻"
                if i < container.occupied - 1 and container.symbols[i+1]["symbol"] == new_symbol:
                    return True, "在容器队列中相邻"
        return False, "不在同一层级"
    
    def judge(self, new_symbol, candidate, container_type):
        """执行五维审判"""
        results = {
            "发音": self.judge_pronunciation(new_symbol, candidate, container_type),
            "字形": self.judge_glyph(new_symbol, candidate),
            "存在论": self.judge_ontology(new_symbol, candidate, container_type),
            "极性": self.judge_polarity(new_symbol, candidate, container_type),
            "层级": self.judge_hierarchy(new_symbol, candidate, container_type)
        }
        
        passed = sum(1 for v in results.values() if v[0])
        
        return {
            "dimensions": results,
            "passed": passed,
            "total": 5,
            "verdict": self._verdict(passed)
        }
    
    def _verdict(self, passed):
        if passed >= 5:
            return "完全合法"
        elif passed >= 4:
            return "合法"
        elif passed >= 3:
            return "远方亲戚"
        elif passed >= 2:
            return "冲突"
        else:
            return "直接拒绝"


# ============================================================
# 第六层：主程序
# ============================================================

def main():
    """主程序"""
    global GLOBAL_OFFSET
    
    print("="*60)
    print("MetaIME 解释器 · 活码运行时 v2")
    print("="*60)
    print(f"\n零维根: 0=刘楚恬(隐藏), 1=我(可见)")
    print(f"初始全局偏移量: 0x{GLOBAL_OFFSET:02X}")
    
    # 创建两个独立的产权容器
    shengmu_container = Container("声母容器", "声母")
    yunmu_container = Container("韵母容器", "韵母")
    
    # 加载声母
    print(f"\n加载声母（{len(SHENGMU_LIST)} 个）...")
    for i, sm in enumerate(SHENGMU_LIST):
        shengmu_container.add(sm, [f"{sm}{vm}" for vm in YUNMU_LIST[:3]])
    print(f"声母容器: {shengmu_container.occupied} 个符号")
    
    # 加载韵母
    print(f"加载韵母（{len(YUNMU_LIST)} 个）...")
    for i, ym in enumerate(YUNMU_LIST):
        yunmu_container.add(ym, [f"{sm}{ym}" for sm in SHENGMU_LIST[:3]])
    print(f"韵母容器: {yunmu_container.occupied} 个符号")
    
    # 显示容器
    shengmu_container.display()
    yunmu_container.display()
    
    # 创建跨容器训练关系
    order = CrossContainerOrder(shengmu_container, yunmu_container)
    order.display_status()
    
    # 演示：声母训练韵母
    print("\n" + "="*60)
    print("演示：声母训练韵母（秩序流程）")
    print("="*60)
    
    # 第一个声母训练所有韵母
    result = order.train_next()
    if isinstance(result, list):
        print(f"\n当前声母训练记录（前5个）:")
        for r in result[:5]:
            print(f"  {r['训练']}")
        if len(result) > 5:
            print(f"  ... (共 {len(result)} 个)")
    else:
        print(f"  {result}")
    
    # 显示训练后状态
    order.display_status()
    
    # 演示：五维审判
    print("\n" + "="*60)
    print("演示：五维审判（新符号入住）")
    print("="*60)
    
    judge = JudgmentEngine(shengmu_container, yunmu_container)
    
    # 合法申请：新声母 "m" 的近义词
    print("\n申请: 'm' 入住声母容器（近义词: 'b'）")
    result = judge.judge("m", "b", "声母")
    print(f"结果: {result['passed']}/5 → {result['verdict']}")
    for dim, (passed, reason) in result["dimensions"].items():
        status = "✓" if passed else "✗"
        print(f"  {status} {dim}: {reason}")
    
    # 非法申请：声母申请入住韵母容器
    print("\n申请: 'a' 入住声母容器（错误：韵母不能住声母容器）")
    result = judge.judge("a", "b", "声母")
    print(f"结果: {result['passed']}/5 → {result['verdict']}")
    for dim, (passed, reason) in result["dimensions"].items():
        status = "✓" if passed else "✗"
        print(f"  {status} {dim}: {reason}")
    
    # 演示：全局偏移量变化
    print("\n" + "="*60)
    print("演示：全局偏移量变化（活码机制）")
    print("="*60)
    
    print(f"当前全局偏移量: 0x{GLOBAL_OFFSET:02X}")
    print("执行: 全局偏移量 += 1")
    GLOBAL_OFFSET += 1
    
    print(f"新的全局偏移量: 0x{GLOBAL_OFFSET:02X}")
    print("\n声母容器位置变化:")
    for entry in shengmu_container.symbols[:5]:
        pos = GLOBAL_OFFSET + entry["position"]
        print(f"  {entry['symbol']}: 0x{entry['position']:02X} → 0x{pos:02X}")
    print("  ...")
    
    # 演示：边界到达后的处理
    print("\n" + "="*60)
    print("演示：边界到达后的处理")
    print("="*60)
    
    # 让所有声母都训练完毕
    for _ in range(shengmu_container.occupied):
        order.train_next()
    
    result = order.train_next()
    print(f"所有声母训练完毕后的结果:")
    print(f"  {result}")
    print("\n正确做法：不是重启，是从另一个容器（韵母容器）拿新东西来训练声母")
    print("或者：从声母容器拿新符号来训练韵母（但声母容器已满）")
    
    order.display_status()
    
    print("\n[活码运行结束]")


if __name__ == "__main__":
    main()
