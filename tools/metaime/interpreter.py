#!/usr/bin/env python3
"""
MetaIME 解释器 · 活码运行时 v3

完整功能：
- 交互式用户输入处理
- 实时错字纠正（基于发音地址）
- 完整声母×韵母训练流程
- 新符号入住审判
- 全局偏移量活码机制
"""

import json
import os
import sys
import readline
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
# 声母/韵母/声调 物理定义
# ============================================================

SHENGMU_LIST = [
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "j", "q", "x",
    "zh", "ch", "sh", "r", "z", "c", "s",
    "y", "w"
]

YUNMU_LIST = [
    "a", "o", "e", "i", "u", "v",
    "ai", "ei", "ui", "ao", "ou", "iu",
    "ie", "ve", "er",
    "an", "en", "in", "un", "vn",
    "ang", "eng", "ing", "ong"
]

TONE_LIST = ["1", "2", "3", "4", "0"]

# 错字纠正表：常见错字 → 正确字（基于发音邻近）
TYPO_CORRECTION = {
    "带": "袋", "代": "袋", "逮": "袋",
    "召": "照", "招": "照", "沼": "照",
    "建": "见", "健": "见", "键": "见",
    "因": "阴", "音": "阴", "姻": "阴",
    "家": "加", "嘉": "加", "佳": "加",
    "份": "分", "纷": "分", "芬": "分",
    "园": "元", "圆": "元", "原": "元",
    "载": "裁", "栽": "裁", "哉": "裁",
    "福": "富", "幅": "福", "辐": "福",
    "凤": "风", "枫": "凤", "封": "凤",
}

# ============================================================
# 第三层：容器
# ============================================================

class Container:
    """产权容器"""
    
    def __init__(self, name, container_type):
        self.name = name
        self.container_type = container_type
        self.symbols = []
        self.capacity = 64
    
    def add(self, symbol, synonyms=None):
        if len(self.symbols) >= self.capacity:
            return False, f"容器已满（{len(self.symbols)}/{self.capacity}）"
        entry = {
            "symbol": symbol,
            "synonyms": synonyms or [],
            "position": len(self.symbols),
            "occupied_at": datetime.now().isoformat()
        }
        self.symbols.append(entry)
        return True, entry
    
    def remove(self, symbol):
        for i, entry in enumerate(self.symbols):
            if entry["symbol"] == symbol:
                self.symbols.pop(i)
                for j in range(i, len(self.symbols)):
                    self.symbols[j]["position"] = j
                return True
        return False
    
    def get_position(self, symbol):
        for entry in self.symbols:
            if entry["symbol"] == symbol:
                return GLOBAL_OFFSET + entry["position"]
        return None
    
    def is_full(self):
        return len(self.symbols) >= self.capacity
    
    def is_empty(self):
        return len(self.symbols) == 0
    
    def display(self):
        print(f"\n{'='*60}")
        print(f"{self.name} ({self.container_type})")
        print(f"占用: {len(self.symbols)}/{self.capacity}")
        print(f"{'-'*60}")
        for entry in self.symbols:
            pos = GLOBAL_OFFSET + entry["position"]
            synonyms_str = "、".join(entry["synonyms"][:3])
            print(f"  0x{pos:02X}  {entry['symbol']:<6} [{synonyms_str}]")
        print(f"{'='*60}\n")


# ============================================================
# 第四层：跨容器训练
# ============================================================

class CrossContainerOrder:
    def __init__(self, shengmu, yunmu):
        self.shengmu = shengmu
        self.yunmu = yunmu
        self.current_index = 0
        self.training_log = []
    
    def train_next(self):
        if self.shengmu.is_empty():
            return "声母容器为空"
        if self.yunmu.is_empty():
            return "韵母容器为空"
        if self.current_index >= len(self.shengmu.symbols):
            return "BOUNDARY_REACHED"
        
        shengmu = self.shengmu.symbols[self.current_index]
        results = []
        for yunmu in self.yunmu.symbols:
            record = {
                "声母": shengmu["symbol"],
                "韵母": yunmu["symbol"],
                "训练": f"{shengmu['symbol']} → {yunmu['symbol']} (0维→1维)",
                "时间": datetime.now().isoformat()
            }
            results.append(record)
            self.training_log.append(record)
        
        self.current_index += 1
        return results
    
    def get_status(self):
        return {
            "声母容器": len(self.shengmu.symbols),
            "韵母容器": len(self.yunmu.symbols),
            "当前训练索引": self.current_index,
            "已训练次数": len(self.training_log),
            "下一个声母": self.shengmu.symbols[self.current_index]["symbol"] 
                         if self.current_index < len(self.shengmu.symbols) else "（完毕）"
        }
    
    def display_status(self):
        status = self.get_status()
        print(f"\n{'='*60}")
        print("跨容器训练状态")
        print(f"{'-'*60}")
        for k, v in status.items():
            print(f"  {k}: {v}")
        print(f"{'='*60}\n")


# ============================================================
# 第五层：五维审判
# ============================================================

class JudgmentEngine:
    def __init__(self, shengmu, yunmu):
        self.shengmu = shengmu
        self.yunmu = yunmu
    
    def judge_pronunciation(self, new_symbol, candidate, container_type):
        if container_type == "声母":
            if new_symbol in SHENGMU_LIST:
                return True, f"合法声母"
            common = set(new_symbol) & set(candidate)
            if common:
                return True, f"共享字符: {common}"
        elif container_type == "韵母":
            if new_symbol in YUNMU_LIST:
                return True, f"合法韵母"
            common = set(new_symbol) & set(candidate)
            if common:
                return True, f"共享字符: {common}"
        return False, "发音不匹配"
    
    def judge_glyph(self, new_symbol, candidate):
        common = set(new_symbol) & set(candidate)
        if common:
            return True, f"共享部件: {common}"
        return False, "无共享部件"
    
    def judge_ontology(self, new_symbol, candidate, container_type):
        container = self.shengmu if container_type == "声母" else self.yunmu
        for entry in container.symbols:
            if entry["symbol"] == candidate:
                if new_symbol in entry.get("synonyms", []):
                    return True, f"在{candidate}的近义词族中"
        return False, "不在任何近义词族中"
    
    def judge_polarity(self, new_symbol, candidate, container_type):
        if container_type == "声母":
            if new_symbol in SHENGMU_LIST and candidate in SHENGMU_LIST:
                return True, "同为声母"
        elif container_type == "韵母":
            if new_symbol in YUNMU_LIST and candidate in YUNMU_LIST:
                return True, "同为韵母"
        return False, "极性不同"
    
    def judge_hierarchy(self, new_symbol, candidate, container_type):
        container = self.shengmu if container_type == "声母" else self.yunmu
        for i, entry in enumerate(container.symbols):
            if entry["symbol"] == candidate:
                if i > 0 and container.symbols[i-1]["symbol"] == new_symbol:
                    return True, "在容器队列中相邻"
                if i < len(container.symbols) - 1 and container.symbols[i+1]["symbol"] == new_symbol:
                    return True, "在容器队列中相邻"
        return False, "不在同一层级"
    
    def judge(self, new_symbol, candidate, container_type):
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
        if passed >= 5: return "完全合法"
        elif passed >= 4: return "合法"
        elif passed >= 3: return "远方亲戚"
        elif passed >= 2: return "冲突"
        else: return "直接拒绝"


# ============================================================
# 第六层：错字纠正器
# ============================================================

class TypoCorrector:
    """基于发音地址的实时错字纠正"""
    
    def __init__(self):
        self.correction_map = TYPO_CORRECTION
    
    def correct(self, text):
        """纠正文本中的错字"""
        corrected = text
        corrections = []
        
        for wrong, right in self.correction_map.items():
            if wrong in corrected:
                corrected = corrected.replace(wrong, right)
                corrections.append(f"{wrong} → {right}")
        
        return corrected, corrections
    
    def add_correction(self, wrong, right):
        """添加新的纠正规则"""
        self.correction_map[wrong] = right


# ============================================================
# 第七层：主交互系统
# ============================================================

class MetaIMESystem:
    """MetaIME 主系统"""
    
    def __init__(self):
        self.shengmu = Container("声母容器", "声母")
        self.yunmu = Container("韵母容器", "韵母")
        self.order = None
        self.judge = None
        self.corrector = TypoCorrector()
        self.running = False
    
    def initialize(self):
        """初始化系统"""
        print("="*60)
        print("MetaIME 解释器 · 活码运行时 v3")
        print("="*60)
        print(f"\n零维根: 0=刘楚恬(隐藏), 1=我(可见)")
        print(f"初始全局偏移量: 0x{GLOBAL_OFFSET:02X}")
        
        # 加载声母
        print(f"\n加载声母（{len(SHENGMU_LIST)} 个）...")
        for sm in SHENGMU_LIST:
            self.shengmu.add(sm, [f"{sm}{vm}" for vm in YUNMU_LIST[:3]])
        
        # 加载韵母
        print(f"加载韵母（{len(YUNMU_LIST)} 个）...")
        for ym in YUNMU_LIST:
            self.yunmu.add(ym, [f"{sm}{ym}" for sm in SHENGMU_LIST[:3]])
        
        # 创建跨容器训练关系
        self.order = CrossContainerOrder(self.shengmu, self.yunmu)
        self.judge = JudgmentEngine(self.shengmu, self.yunmu)
        
        print(f"\n系统初始化完成。")
        print(f"声母容器: {len(self.shengmu.symbols)} 个符号")
        print(f"韵母容器: {len(self.yunmu.symbols)} 个符号")
        print(f"全局偏移量: 0x{GLOBAL_OFFSET:02X}")
    
    def process_input(self, user_input):
        """处理用户输入"""
        # 步骤1：错字纠正
        corrected, corrections = self.corrector.correct(user_input)
        
        if corrections:
            print(f"\n[错字纠正] {'; '.join(corrections)}")
        
        # 步骤2：解析命令
        if corrected.startswith("/"):
            return self.process_command(corrected)
        
        # 步骤3：普通输入处理
        print(f"\n[输入] {corrected}")
        print(f"[状态] 声母容器: {len(self.shengmu.symbols)} | 韵母容器: {len(self.yunmu.symbols)} | 偏移量: 0x{GLOBAL_OFFSET:02X}")
        
        return True
    
    def process_command(self, command):
        """处理命令"""
        parts = command[1:].strip().split()
        if not parts:
            return True
        
        cmd = parts[0].lower()
        args = parts[1:] if len(parts) > 1 else []
        
        if cmd == "quit" or cmd == "exit":
            print("\n[系统关闭]")
            return False
        
        elif cmd == "status":
            self.show_status()
        
        elif cmd == "train":
            self.train_next()
        
        elif cmd == "train_all":
            self.train_all()
        
        elif cmd == "insert":
            if len(args) >= 3:
                self.insert_symbol(args[0], args[1], args[2])
            else:
                print("用法: /insert <符号> <近义词> <容器类型>")
        
        elif cmd == "shift":
            if args:
                try:
                    delta = int(args[0])
                    self.shift_offset(delta)
                except ValueError:
                    print("偏移量必须是整数")
            else:
                print("用法: /shift <偏移量>")
        
        elif cmd == "show":
            self.shengmu.display()
            self.yunmu.display()
        
        elif cmd == "help":
            self.show_help()
        
        else:
            print(f"未知命令: {cmd}")
            print("输入 /help 查看帮助")
        
        return True
    
    def show_status(self):
        """显示系统状态"""
        print(f"\n{'='*60}")
        print("系统状态")
        print(f"{'-'*60}")
        print(f"声母容器: {len(self.shengmu.symbols)}/{self.shengmu.capacity}")
        print(f"韵母容器: {len(self.yunmu.symbols)}/{self.yunmu.capacity}")
        print(f"全局偏移量: 0x{GLOBAL_OFFSET:02X}")
        if self.order:
            status = self.order.get_status()
            for k, v in status.items():
                print(f"{k}: {v}")
        print(f"{'='*60}\n")
    
    def train_next(self):
        """执行下一步训练"""
        if not self.order:
            print("训练系统未初始化")
            return
        
        result = self.order.train_next()
        
        if result == "BOUNDARY_REACHED":
            print("\n[边界到达] 所有声母已训练完毕。")
            print("[正确做法] 不是重启，是从另一个容器拿新东西来训练当前容器。")
            print("[当前状态] 声母容器已满，需要从外部获取新声母。")
        elif isinstance(result, str):
            print(f"\n[错误] {result}")
        else:
            print(f"\n[训练] 当前声母训练记录（前5个）:")
            for r in result[:5]:
                print(f"  {r['训练']}")
            if len(result) > 5:
                print(f"  ... (共 {len(result)} 个)")
            self.order.display_status()
    
    def train_all(self):
        """执行完整训练流程"""
        if not self.order:
            print("训练系统未初始化")
            return
        
        print("\n[完整训练流程] 开始...")
        total = 0
        while True:
            result = self.order.train_next()
            if result == "BOUNDARY_REACHED":
                break
            if isinstance(result, str):
                print(f"[错误] {result}")
                break
            total += len(result)
        
        print(f"\n[训练完成] 共执行 {total} 次训练")
        self.order.display_status()
    
    def insert_symbol(self, symbol, nearest, container_type):
        """插入新符号"""
        if not self.judge:
            print("审判系统未初始化")
            return
        
        result = self.judge.judge(symbol, nearest, container_type)
        
        print(f"\n[审判] 符号 '{symbol}' 申请入住{container_type}容器")
        print(f"结果: {result['passed']}/5 → {result['verdict']}")
        
        for dim, (passed, reason) in result["dimensions"].items():
            status = "✓" if passed else "✗"
            print(f"  {status} {dim}: {reason}")
        
        if result["passed"] >= 3:
            container = self.shengmu if container_type == "声母" else self.yunmu
            success, entry = container.add(symbol, [nearest])
            if success:
                print(f"\n[入住成功] {symbol} @ 位置 0x{GLOBAL_OFFSET + entry['position']:02X}")
            else:
                print(f"\n[入住失败] {entry}")
    
    def shift_offset(self, delta):
        """全局偏移量变化"""
        global GLOBAL_OFFSET
        old_offset = GLOBAL_OFFSET
        GLOBAL_OFFSET += delta
        print(f"\n[全局偏移量] 0x{old_offset:02X} → 0x{GLOBAL_OFFSET:02X} (变化: {delta:+d})")
        print("[活码机制] 所有符号位置已自动更新")
    
    def show_help(self):
        """显示帮助"""
        print(f"\n{'='*60}")
        print("MetaIME 命令帮助")
        print(f"{'-'*60}")
        print("  /status          - 显示系统状态")
        print("  /train           - 执行下一步训练")
        print("  /train_all       - 执行完整训练流程")
        print("  /insert <符号> <近义词> <容器> - 插入新符号")
        print("  /shift <偏移量>  - 全局偏移量变化")
        print("  /show            - 显示容器内容")
        print("  /help            - 显示帮助")
        print("  /quit            - 退出系统")
        print(f"{'='*60}\n")
    
    def run(self):
        """运行交互式系统"""
        self.initialize()
        self.running = True
        
        print("\n" + "="*60)
        print("系统已就绪。输入 /help 查看帮助，/quit 退出。")
        print("="*60)
        
        while self.running:
            try:
                user_input = input("\nMetaIME> ").strip()
                if not user_input:
                    continue
                
                result = self.process_input(user_input)
                if result is False:
                    self.running = False
            
            except KeyboardInterrupt:
                print("\n\n[系统关闭]")
                self.running = False
            except EOFError:
                print("\n[系统关闭]")
                self.running = False


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    system = MetaIMESystem()
    system.run()
